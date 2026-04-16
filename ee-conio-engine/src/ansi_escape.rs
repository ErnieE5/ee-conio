use regex::{Captures, Regex};
use regex_macro::regex;
use std::sync::LazyLock;

#[allow(unused_imports)]
use crate::*;
//{
//     get_named_background_escape,
//     get_named_foreground_escape,
//     get_keyword,
//     fg_256,
//     bg_256
// };

use crate::helpers::{ParseError, r_g_b_from_string};

use std::fmt::Display;

pub fn fg_color_256<T>(c: T) -> String
where
    T: Display,
{
    format!(fg_256!("{}"), c)
}

pub fn fg_color_rgb<T>(r: T, g: T, b: T) -> String
where
    T: Display,
{
    format!(fg_rgb!("{}", "{}", "{}"), r, g, b)
}

pub fn bg_color_256<T>(c: T) -> String
where
    T: Display,
{
    format!(bg_256!("{}"), c)
}

pub fn bg_color_rgb<T>(r: T, g: T, b: T) -> String
where
    T: Display,
{
    format!(bg_rgb!("{}", "{}", "{}"), r, g, b)
}

pub fn sgr_code<T>(c: T) -> String
where
    T: Display,
{
    format!(sgr!("{}"), c)
}

pub fn csi_sequence(s: &str) -> String {
    format!(csi!("{}"), s)
}

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

fn chardig(o: &str, dig: &str) -> Result<u8, ParseError> {
    Ok(match dig.parse() {
        Ok(v) => v,
        Err(_) => {
            let msg = format!("'{dig}' is is not a base 10 value in the range 0..=255");
            return Err(ParseError::new(o, msg, 0, 0));
        }
    })
}

#[allow(non_snake_case)]
fn ansi_c(c: Captures) -> ParseStringResult {
    match chardig("ansi_c", &c["dig"]) {
        Ok(v) => Ok(fg_color_256(v)),
        Err(e) => Err(e),
    }
}

#[allow(non_snake_case)]
fn ansi_C(c: Captures) -> ParseStringResult {
    match chardig("ansi_C", &c["dig"]) {
        Ok(v) => Ok(bg_color_256(v)),
        Err(e) => Err(e),
    }
}

#[allow(non_snake_case)]
fn ansi_x(c: Captures) -> ParseStringResult {
    match chardig("ansi_x", &c["dig"]) {
        Ok(v) => Ok(sgr_code(v)),
        Err(e) => Err(e),
    }
}

#[allow(non_snake_case)]
fn ansi_X(c: Captures) -> ParseStringResult {
    Ok(csi_sequence(&c["dig"]))
}

#[allow(non_snake_case)]
fn ansi_s(c: Captures) -> ParseStringResult {
    let v = &c["name"];
    match get_named_foreground_escape(v.trim()) {
        Some(s) => Ok(s.to_string()),
        None => {
            let msg = format!("'{v}' not a known named color");
            Err(ParseError::new("ansi_s", msg, 0, 0))
        }
    }
}

#[allow(non_snake_case)]
fn ansi_S(c: Captures) -> ParseStringResult {
    let v = &c["name"];
    match get_named_background_escape(v.trim()) {
        Some(s) => Ok(s.to_string()),
        None => {
            let msg = format!("'{v}' not a known named color");
            Err(ParseError::new("ansi_S", msg, 0, 0))
        }
    }
}

type Vsvs = Vec<(String, Vec<String>)>;
pub fn find_replacement_patterns(source: &str) -> Result<Vsvs, ParseError> {
    // For every ~[...] collect the sequences to replace
    let mut replace: Vsvs = Vec::new();

    // Scan for any ~[...] patterns
    for a in regex!("~\\[(?<b>.*?)\\]").captures_iter(source) {
        let Some(t) = a.get(0) else {
            todo!();
        };
        let Some(m) = a.get(1) else {
            todo!();
        };

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

    // foreground and background RGB colors #RRGGBB $RRGGBB [[:xdigit:]]
    m.push((regex!("^(?<rgb>[#].{6,6})$"), crack_ansi_fg_rgb));
    m.push((regex!("^(?<rgb>[$].{6,6})$"), crack_ansi_bg_rgb));

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

    Err(ParseError::new(
        "transform_one",
        format!(
            "'{}' does not match known keywords, names, or mnemonics",
            value
        ),
        0,
        0,
    ))
}

pub fn transform_all(value: &str) -> Result<Vec<String>, ParseError> {
    let mut items: Vec<String> = Vec::new();

    for a in regex!("([#$@]'[^']+'|\\S+)").captures_iter(value) {
        let Some(m) = a.get(1) else {
            panic!("regex expresion must contain at least one capture group");
        };

        let token: &str = m.as_str();

        if let Some(esc) = get_keyword(token) {
            items.push(esc.to_string());
        } else {
            match transform_one(token) {
                Ok(x) => items.push(x),
                Err(e) => {
                    return Err(e.wrap("transform_to_escapes", e.msg.clone(), m.start(), m.end()));
                }
            };
        }
    }

    Ok(items)
}
