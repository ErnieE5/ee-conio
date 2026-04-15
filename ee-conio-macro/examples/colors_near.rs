mod common;
use common::*;

use ee_conio::{bg_color_rgb, named_color_iter};

use ee_conio_macro::cprintln;

#[rustfmt::skip]
fn main() {
    let codes = std::env::args().skip(1).collect::<Vec<String>>();
    let in_str:&str = if codes.len() > 0 { &codes[0] } else { "#000000" };

    header!("Named RGB colors near {}", in_str);

    let start = match Color::from_str(in_str) { Ok(s) => s,
            Err(e) => { cprintln!("~[c196]{in_str} ~[c255]{e:#?}"); return; } };

    let mut near_matches: BTreeMap<Color, (String, String)> = BTreeMap::new();

    near_matches.insert(start.clone(), (String::from(""), start.to_string()));

    for (name, code) in named_color_iter() {
        let cc = Color::from_str(code).expect("invalid hex string");

        if cc.diff(&start) < 7.0 {
            near_matches.insert(cc, (name.to_string(), code.to_string()));
        }
    }

    for (cc, (name, code)) in near_matches.iter() {
        let (r, g, b)   = cc.into_components();
        let luma        = cc.luma();
        let fg          = cc.fg_best_contrast();
        let bg          = bg_color_rgb(r, g, b);
        let (m1, m2)    = if *cc == start { ("🡪 ", " 🡨") } else { ("  ", "  ") };

        cprintln!("{fg}{bg}{m1} {name:<25} {luma:7.3} {code:>10}{m2} ~[x0 XK]");
    }

    cprintln!();
}
