mod common;
use common::*;

use ee_conio::{bg_color_rgb, fg_color_rgb};

use ee_conio_macro::cprintln;

fn usage() {
    cprintln!("\n\n~[c255]arg is optional ~[c7]: ~[c227]fg~[c7]|~[c227]bg\n\nTry:\n");
    cprintln!("cargo run --example all_named_colors -- ~[c227]--fg\n");
}

fn bg_mode(args: &Vec<String>) -> Option<bool> {
    match args.len() {
        0 => Some(true),
        1 => match args[0].as_str() {
            "--bg" => Some(true),
            "--fg" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

#[rustfmt::skip]
fn main() {
    let codes = std::env::args().skip(1).collect::<Vec<String>>();
    let Some(bg_mode) = bg_mode(&codes) else { usage(); return };

    header!("Full list of named RGB colors supported in this library.");

    for (k, (_code, name)) in get_named_color_map() {
        let (r, g, b)   = k.into_components();
        let hex         = k.to_string();
        let luma        = k.luma();
        let (bg, fg)    = if bg_mode {
            (bg_color_rgb(r, g, b), k.fg_best_contrast())
        } else {
            (k.bg_best_contrast(), fg_color_rgb(r, g, b))
        };

        cprintln!("{bg}{fg} {name:<25} {luma:5.3} {hex:>10} ~[x0 XK]");
    }

    cprintln!();
}
