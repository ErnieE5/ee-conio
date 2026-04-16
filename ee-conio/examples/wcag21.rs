mod common;
use common::*;

use ee_conio::{bg_color_rgb, cprintln, fg_color_rgb};

#[rustfmt::skip]
pub fn main() {
    let codes = std::env::args().skip(1).collect::<Vec<_>>();

    if codes.len() < 2 {
        cprintln!("\n\n~[c255] args required~[c7]: two hex values \n\nTry:\n\ncargo run --example wcag21 -- ~[c227]EE55EE ~[c51]00ff00\n");
        return;
    }

    let bg_in = codes[0].as_str();
    let fg_in = codes[1].as_str();

    header!("WCAG 2.1 checks for {bg_in} with {fg_in}.");

    let bg_c: Color = Color::from_str(bg_in).expect("invalid hex string");
    let fg_c: Color = Color::from_str(fg_in).expect("invalid hex string");

    let bg = {
        let (r, g, b) = bg_c.into_components();
        bg_color_rgb(r, g, b)
    };

    let fg = {
        let (r, g, b) = fg_c.into_components();
        fg_color_rgb(r, g, b)
    };

    cprintln!("{bg}{fg} This is some test text. ~[x0]\n");

    cprintln!("{bg}{fg}~[dhtop] This is some test text. ~[x0]");
    cprintln!("{bg}{fg}~[dhbot] This is some test text. ~[x0]");
    cprintln!("\n");

    let r = bg_c.wcag21_test(&fg_c);
    cprintln!("~[c208]{:8.5}~[c7] Relative Luminance of Background {bg_in}", r.relative_luminance);
    cprintln!("~[c227]{:8.5}~[c7] Relative Luminance of Forground  {fg_in}", r.relative_luminance_other);
    cprintln!("~[c51 ]{:8.5}~[c7] Ratio", r.contrast);

    cprintln!("{} Minimum Contrast for text.",          r.mark(r.min_contrast_text));
    cprintln!("{} Minimum Contrast for large text.",    r.mark(r.min_contrast_large_text));
    cprintln!("{} Enhanced Contrast for text.",         r.mark(r.enhanced_contrast_text));
    cprintln!("{} Enhanced Contrast for large text.",   r.mark(r.enhanced_contrast_large_text));
    cprintln!("{} Minimum Contrast for graphics.",      r.mark(r.min_contrast_graphics));

    cprintln!();
}
