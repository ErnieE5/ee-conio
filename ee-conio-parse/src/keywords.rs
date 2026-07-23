use std::collections::HashMap;
use std::sync::LazyLock;

use crate::transform_one;

#[rustfmt::skip]
pub static NAMED_ESCAPES: &[(&str, &str)] = &[
    // The standard eight, ANSI 0..=7.  Lowercase is foreground, UPPERCASE is
    // background.  These follow the usual convention (as colored, owo-colors
    // and ansi_term do) so that ~[green] and ~[c2] agree.  Reach for the
    // bright_ set below on a dark background; 2, 3 and 7 are muddy on many
    // terminals.
    ("black",           "c0"),
    ("BLACK",           "C0"),
    ("red",             "c1"),
    ("RED",             "C1"),
    ("green",           "c2"),
    ("GREEN",           "C2"),
    ("yellow",          "c3"),
    ("YELLOW",          "C3"),
    ("blue",            "c4"),
    ("BLUE",            "C4"),
    ("magenta",         "c5"),
    ("MAGENTA",         "C5"),
    ("cyan",            "c6"),
    ("CYAN",            "C6"),
    ("white",           "c7"),
    ("WHITE",           "C7"),

    // The bright eight, ANSI 8..=15.
    ("bright_black",    "c8"),
    ("BRIGHT_BLACK",    "C8"),
    ("bright_red",      "c9"),
    ("BRIGHT_RED",      "C9"),
    ("bright_green",    "c10"),
    ("BRIGHT_GREEN",    "C10"),
    ("bright_yellow",   "c11"),
    ("BRIGHT_YELLOW",   "C11"),
    ("bright_blue",     "c12"),
    ("BRIGHT_BLUE",     "C12"),
    ("bright_magenta",  "c13"),
    ("BRIGHT_MAGENTA",  "C13"),
    ("bright_cyan",     "c14"),
    ("BRIGHT_CYAN",     "C14"),
    ("bright_white",    "c15"),
    ("BRIGHT_WHITE",    "C15"),

    // Return a channel to the terminal's default without an SGR 0, which
    // would also drop bold, underline and friends.
    ("default",         "x39"),
    ("DEFAULT",         "x49"),
    ("CUU",             "XA"),
    ("CUD",             "XB"),
    ("CUF",             "XC"),
    ("CUB",             "XD"),
    ("CNL",             "XE"),
    ("CPL",             "XF"),
    // DEC private modes.  Not named hide_cursor: "hide" is already SGR 8,
    // which conceals text rather than the cursor.
    ("curs_off",        "X?25l"),
    ("curs_on",         "X?25h"),
    ("ED",              "X2J"),
    ("cls",             "X2J X3J X1;1H"),
    ("clreol",          "XK"),
    ("clrbol",          "X1K"),
    ("clrln",           "X2K"),
    ("reset",           "x0"),
    // SGR 22 is "normal intensity" and clears bold and dim together, so both
    // _off names map to it.
    ("bold",            "x1"),
    ("bold_on",         "x1"),
    ("bold_off",        "x22"),
    ("dim",             "x2"),
    ("dim_on",          "x2"),
    ("dim_off",         "x22"),
    ("italic",          "x3"),
    ("italic_on",       "x3"),
    ("italic_off",      "x23"),
    ("underline",       "x4"),
    ("under_on",        "x4"),
    ("under_off",       "x24"),
    ("blink",           "x5"),
    ("blink_on",        "x5"),
    ("blink_off",       "x25"),
    ("reverse",         "x7"),
    ("reverse_on",      "x7"),
    ("reverse_off",     "x27"),
    ("hide",            "x8"),
    ("reveal",          "x28"),
    ("hide_on",         "x8"),
    ("hide_off",        "x28"),
    ("strike",          "x9"),
    ("strike_on",       "x9"),
    ("strike_off",      "x29"),
    ("overline",        "x53"),
    ("over_on",         "x53"),
    ("over_off",        "x55"),
];

// This list could get huge and I am not certain
// it is worth it
//
#[rustfmt::skip]
pub static NAMED_ETC: &[(&str, &str)] = &[
    ("not",     "🚫"),
    (":)",      "😀"),
    ("heart",   "❤"),
    ("peach",   "🍑"),
    ("poo",     "💩"),
    ("dhtop",   "\x1b#3"),
    ("dhbot",   "\x1b#4"),
    ("swsh",    "\x1b#5"),
    ("dwsh",    "\x1b#6"),
];

type Kwmap = HashMap<String, String>;

static KEYWORDS: LazyLock<Kwmap> = LazyLock::new(|| {
    let mut m: Kwmap = HashMap::new();

    for (k, v) in NAMED_ESCAPES {
        let mut vv = String::new();

        // !!!!!
        // Deadlock when we use this because this list
        // is referenced BY transform_all
        //
        // let vv = match transform_all(v) {
        //     Ok(v)   => v,
        //     Err(e)  => {
        //         panic!("This is bad");
        //     }
        // };

        // Pretend that we are better than that and do the simple split
        //
        for v1 in v.split(" ") {
            vv.push_str(transform_one(v1).expect("this is bad 3").as_str())
        }
        m.insert(k.to_string(), vv);
    }
    for (k, v) in NAMED_ETC {
        m.insert(k.to_string(), v.to_string());
    }

    m
});

pub fn get_keyword(name: &str) -> Option<&str> {
    KEYWORDS.get(name).map(|kw| kw.as_str())
    // match KEYWORDS.get(name) {
    //     Some(s) => Some(s),
    //     None => None,
    // }
}
