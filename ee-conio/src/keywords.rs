use std::collections::HashMap;
use std::sync::LazyLock;

use crate::transform_one;

#[rustfmt::skip]
pub static NAMED_ESCAPES: &[(&str, &str)] = &[
    ("black", "c0"),
    ("BLACK", "C0"),
    ("red", "c1"),
    ("RED", "C1"),
    ("green", "c10"),
    ("GREEN", "C10"),
    ("yellow", "c11"),
    ("YELLOW", "C11"),
    ("blue", "c4"),
    ("BLUE", "C4"),
    ("magenta", "c5"),
    ("MAGENTA", "C5"),
    ("cyan", "c6"),
    ("CYAN", "C6"),
    ("white", "c15"),
    ("WHITE", "C15"),
    ("CUU", "XA"),
    ("CUD", "XB"),
    ("CUF", "XC"),
    ("CUB", "XD"),
    ("CNL", "XE"),
    ("CPL", "XF"),
    ("CPL", "XF"),
    ("ED", "X2J"),
    ("clear", "X2J X3J X1;1H"),
    ("reset", "x0"),
    ("italic", "x3"),
    ("italic_on", "x3"),
    ("italic_off", "x23"),
    ("underline", "x4"),
    ("under_on", "x4"),
    ("under_off", "x24"),
    ("blink", "x5"),
    ("blink_on", "x5"),
    ("blink_off", "x25"),
    ("reverse", "x7"),
    ("reverse_on", "x7"),
    ("reverse_off", "x27"),
    ("hide", "x8"),
    ("reveal", "x28"),
    ("hide_on", "x8"),
    ("hide_off", "x28"),
    ("strike", "x9"),
    ("strike_on", "x9"),
    ("strike_off", "x29"),
    ("overline", "x53"),
    ("over_on", "x53"),
    ("over_off", "x55"),
];

// This list could get huge and I am not certain
// it is worth it
//
pub static NAMED_ETC: &[(&str, &str)] = &[
    ("not", "🚫"),
    ("face", "😀"),
    ("peach", "🍑"),
    ("poo", "💩"),
    ("dhtop", "\x1b#3"),
    ("dhbot", "\x1b#4"),
    ("swsh", "\x1b#5"),
    ("dwsh", "\x1b#6"),
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
    match KEYWORDS.get(name) {
        Some(s) => Some(s),
        None => None,
    }
}
