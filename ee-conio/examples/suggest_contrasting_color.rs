mod common;
use common::*;

use ee_conio::{bg_color_rgb, cprintln, fg_color_rgb};
use ee_conio_engine::named_color_iter;

use ordered_float::OrderedFloat;

#[rustfmt::skip]
fn usage() {
    cprintln!("\n\n~[c255]args required~[c7]: ~[c227]fg~[c7]|~[c227]bg, ~[c51]hex value~[x0]\n\nTry:\n");
    cprintln!("cargo run --example suggest_contrasting_color -- ~[c227]bg ~[c51]ee88ee\n");
    cprintln!("cargo run --example suggest_contrasting_color -- ~[c227]fg ~[c51]4020FF\n");
}

#[rustfmt::skip]
fn main() {
    let codes = std::env::args().skip(1).collect::<Vec<_>>();
    if codes.len() < 2 { usage(); return; }

    let mode = match codes[0].as_str() {
        "bg" => "background",
        "fg" => "foreground",
        _    => { usage(); return; }
    };

    let txt_in = codes[1].as_str();
    let primary: Color = Color::from_str(txt_in).expect("invalid hex string");

    let hex = primary.to_string();
    header!("Suggested named {mode} colors that contrast well with {hex}.");

    type ContrastMap = BTreeMap<OrderedFloat<f32>, (String, Color)>;
    let mut good_matches = ContrastMap::new();

    for (name, code) in named_color_iter() {
        let cc = Color::from_str(code).expect("invalid hex string");

        match cc.meets_text_min_contrast(&primary) {
            Some(c) => { good_matches.insert(OrderedFloat(c), (name.to_string(), cc)); }
            None => {}
        };
    }

    let (pr, pg, pb) = primary.into_components();

    for (rc, (name, cc)) in good_matches {
        let (r, g, b)   = cc.into_components();
        let hex         = cc.to_string();

        let (fg, bg)    = if mode == "foreground" {
            (fg_color_rgb(r, g, b), bg_color_rgb(pr, pg, pb))
        } else {
            (bg_color_rgb(r, g, b), fg_color_rgb(pr, pg, pb))
        };

        let icons       = primary.wcag21_test(&cc).test_icons();

        cprintln!("{fg}{bg} {name:<25} ~[x0]{rc:7.3} {icons} {hex} ~[x0 XK]");
    }

    cprintln!();
}
