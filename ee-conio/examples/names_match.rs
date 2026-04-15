mod common;
use common::*;

use ee_conio::{bg_color_rgb, fg_color_rgb, match_name_iter};

use ee_conio_macro::cprintln;

#[rustfmt::skip]
fn main() {
    let name_re = std::env::args().skip(1).filter(|n| !n.starts_with("--")).collect::<Vec<_>>().join(" ");
    let opts    = std::env::args().skip(1).filter(|n| n.starts_with("--") ).collect::<Vec<_>>().join(" ");

    let mode = if opts == "--fg" { "fg" } else { "bg" };

    header!("Named RGB colors that match '{}' pattern.", name_re);

    for (index, (name, nrgb)) in (1..).zip(match_name_iter(name_re.as_str())) {
        let cc          = Color::from_str(nrgb).expect("invalid hex string");
        let (r, g, b)   = cc.into_components();
        let luma        = cc.luma();
        let hex         = cc.to_string();
        let (bg, fg)    = if mode == "bg" {
            (bg_color_rgb(r, g, b), cc.fg_best_contrast())
        } else {
            (cc.bg_best_contrast(), fg_color_rgb(r, g, b))
        };

        cprintln!("{index:>3} {fg}{bg} {name:<25} {luma:7.3} {hex:>10} ~[x0]");
    }

    cprintln!();
}
