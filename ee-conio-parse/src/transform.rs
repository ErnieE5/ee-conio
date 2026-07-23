use regex::{Captures, Regex};
use regex_macro::regex;
use std::sync::LazyLock;

// The escape *builders* live in ee-conio-engine so that the runtime crate
// carries no dependencies.  Everything below is parsing, and only ever runs
// inside the proc macro (or for anyone who opts into this crate directly).
use ee_conio_engine::{
    bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb, sgr_code,
};

use crate::helpers::{ParseError, r_g_b_from_string};
use crate::{get_keyword, get_named_background_escape, get_named_foreground_escape};

type ParseStringResult = Result<String, ParseError>;
type CapHandle = fn(Captures) -> ParseStringResult;

#[allow(non_snake_case)]
fn crack_ansi_fg_rgb(c: Captures) -> ParseStringResult {
    match r_g_b_from_string(&c["rgb"], "#", "crack_ansi_fg_rgb") {
        Ok((r, g, b)) => Ok(fg_color_rgb(r, g, b)),
        Err(e) => Err(e),
    }
}

#[allow(non_snake_case)]
fn crack_ansi_bg_rgb(c: Captures) -> ParseStringResult {
    match r_g_b_from_string(&c["rgb"], "$", "crack_ansi_bg_rgb") {
        Ok((r, g, b)) => Ok(bg_color_rgb(r, g, b)),
        Err(e) => Err(e),
    }
}

// `at` is where `dig` starts inside the token, so the error underlines the
// digits themselves rather than the whole mnemonic.
fn chardig(o: &str, dig: &str, at: usize) -> Result<u8, ParseError> {
    match dig.parse() {
        Ok(v) => Ok(v),
        Err(_) => {
            let msg = format!("'{dig}' is not a base 10 value in the range 0..=255");
            Err(ParseError::new(o, msg, at, at + dig.len()))
        }
    }
}

#[allow(non_snake_case)]
fn ansi_c(c: Captures) -> ParseStringResult {
    let d = c.name("dig").expect("'dig' capture group");
    Ok(fg_color_256(chardig("ansi_c", d.as_str(), d.start())?))
}

#[allow(non_snake_case)]
fn ansi_C(c: Captures) -> ParseStringResult {
    let d = c.name("dig").expect("'dig' capture group");
    Ok(bg_color_256(chardig("ansi_C", d.as_str(), d.start())?))
}

#[allow(non_snake_case)]
fn ansi_x(c: Captures) -> ParseStringResult {
    let d = c.name("dig").expect("'dig' capture group");
    Ok(sgr_code(chardig("ansi_x", d.as_str(), d.start())?))
}

#[allow(non_snake_case)]
fn ansi_X(c: Captures) -> ParseStringResult {
    Ok(csi_sequence(&c["dig"]))
}

#[allow(non_snake_case)]
fn ansi_s(c: Captures) -> ParseStringResult {
    let n = c.name("name").expect("'name' capture group");
    match get_named_foreground_escape(n.as_str().trim()) {
        Some(s) => Ok(s.to_string()),
        None => {
            let msg = format!("'{}' not a known named color", n.as_str());
            Err(ParseError::new("ansi_s", msg, n.start(), n.end()))
        }
    }
}

#[allow(non_snake_case)]
fn ansi_S(c: Captures) -> ParseStringResult {
    let n = c.name("name").expect("'name' capture group");
    match get_named_background_escape(n.as_str().trim()) {
        Some(s) => Ok(s.to_string()),
        None => {
            let msg = format!("'{}' not a known named color", n.as_str());
            Err(ParseError::new("ansi_S", msg, n.start(), n.end()))
        }
    }
}

type Vsvs = Vec<(String, Vec<String>)>;
pub fn find_replacement_patterns(source: &str) -> Result<Vsvs, ParseError> {
    // For every ~[...] collect the sequences to replace
    let mut replace: Vsvs = Vec::new();

    // Scan for any ~[...] patterns
    for a in regex!("~\\[(?<b>.*?)\\]").captures_iter(source) {
        // Group 0 is the whole match and 'b' is the only other group in the
        // pattern above, so both always participate in a match that captures_
        // iter yielded.  Neither expect is reachable.
        let t = a.get(0).expect("group 0 is the whole ~[...] match");
        let m = a.get(1).expect("group 'b' is the ~[...] body");

        match transform_all(m.as_str()) {
            Ok(x) => replace.push((t.as_str().to_string(), x)),
            Err(e) => {
                let length = e.end - e.start;

                return Err(e.wrap(
                    "find_replacement_patterns",
                    e.msg.to_string(),
                    m.start() + e.start,
                    m.start() + e.start + length,
                ));
            }
        }
    }

    Ok(replace)
}

type RemapItem<'a> = Vec<(&'a LazyLock<Regex>, CapHandle)>;
static RE_TRANS: LazyLock<RemapItem> = LazyLock::new(|| {
    let mut m: RemapItem = Vec::new();

    // These are in rough expected usage order.  The ideas originated as
    // a simple way to enable 256 color options in xterm.

    // All of the RE patterns expect the patters to be isolated.

    // forground/background 256 color escapes
    m.push((regex!("^(?<opr>c[:]?|fore[:])(?<dig>.+)$"), ansi_c));
    m.push((regex!("^(?<opr>C[:]?|back[:])(?<dig>.+)$"), ansi_C));

    // SGR sequences
    m.push((regex!("^(?<opr>x[:]?|(?i:SGR)[:])(?<dig>.+)$"), ansi_x));

    // CSI control  \x1b[38;2;255;255;255m
    m.push((
        regex!("^(?<opr>X[:]?|(?i:CSI)[:])(?<dig>[0-9;]{0,20}[ABCDEFGJHKSTfhilmnrsu])$"),
        ansi_X,
    ));

    // Named RGB colors #'foreground' $'background'
    m.push((regex!("^(?<opr>[#]')(?<name>.*)'$"), ansi_s));
    m.push((regex!("^(?<opr>[$]')(?<name>.*)'$"), ansi_S));

    // foreground and background RGB colors #RRGGBB $RRGGBB
    //
    // [[:xdigit:]] rather than `.` so a quoted name can never *also* parse as
    // hex.  With `.{6,6}` the token #'abcd' matched both this and the named
    // color pattern above, and correctness depended on the push order here.
    m.push((regex!("^(?<rgb>[#][[:xdigit:]]{6})$"), crack_ansi_fg_rgb));
    m.push((regex!("^(?<rgb>[$][[:xdigit:]]{6})$"), crack_ansi_bg_rgb));

    m
});

// Transform a single item
pub fn transform_one(value: &str) -> ParseStringResult {
    // Find the matching codes from the inner text
    // 'outer: for (regex,transform) in &re_transforms {
    for (regex, transform) in RE_TRANS.iter() {
        for a in regex.captures_iter(value) {
            let s = transform(a)?;

            if !s.is_empty() {
                return Ok(s);
            }
        }
    }

    // Name the likely intent when the token at least *looks* like a color
    // literal.  Tightening the rgb patterns to [[:xdigit:]] means malformed
    // hex no longer reaches r_g_b_from_string, so say so here instead.
    let msg = match value.chars().next() {
        Some('#') | Some('$') => {
            format!("'{value}' is neither six hex digits nor a quoted color name")
        }
        _ => format!("'{value}' does not match known keywords, names, or mnemonics"),
    };

    Err(ParseError::new("transform_one", msg, 0, value.len()))
}

pub fn transform_all(value: &str) -> Result<Vec<String>, ParseError> {
    let mut items: Vec<String> = Vec::new();

    for a in regex!("([#$@]'[^']+'|\\S+)").captures_iter(value) {
        // The pattern is a single group covering the whole token, so it always
        // participates.  Not reachable.
        let m = a.get(1).expect("token pattern has exactly one capture group");

        let token: &str = m.as_str();

        if let Some(esc) = get_keyword(token) {
            items.push(esc.to_string());
        } else {
            match transform_one(token) {
                Ok(x) => items.push(x),
                Err(e) => {
                    // Shift the inner span instead of replacing it with the
                    // whole token, so precision computed by the leaf survives
                    // (chardig points at the digits).  Same shape as the
                    // re-basing in find_replacement_patterns.
                    let length = e.end - e.start;

                    return Err(e.wrap(
                        "transform_to_escapes",
                        e.msg.clone(),
                        m.start() + e.start,
                        m.start() + e.start + length,
                    ));
                }
            };
        }
    }

    Ok(items)
}
