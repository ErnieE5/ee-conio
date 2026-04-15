mod common;
use common::*;

use ee_conio::{bg_color_256, fg_color_256, sgr};

/// layout based on  colortest-python -n -f -p
/// (original 2013)  http://zhar.net/projects/
/// (current  2026)  https://github.com/eikenb/terminal-colors
pub fn main() {
    header!("Table of 8bit Terminal Colors");

    let mut pad = false;
    let mut show_bg = false;

    for a in std::env::args() {
        match a.as_str() {
            "--pad" => pad = true,
            "--bg" => show_bg = true,
            _ => {}
        }
    }

    let pnl = if pad {
        concat!(sgr!(0), "\n\n")
    } else {
        concat!(sgr!(0), "\n")
    };
    let psp = if pad { concat!(sgr!(0), " ") } else { sgr!(0) };

    // Background is color
    let show = |code: u8, high: u8| {
        let mut bg = String::new();
        let fg;

        if show_bg {
            bg = bg_color_256(code);
            fg = fg_color_256(high);
        } else {
            fg = fg_color_256(code);
        }

        print!("{fg}{bg}{code:^5 }{psp}");
    };

    // Standard (Same as SGR 30..38)
    for code in 0..8 {
        show(code, if code > 5 { 0 } else { 255 });
    }
    print!("{pnl}");

    // High intensity (Same as SGR 40..48)
    for code in 8..16 {
        show(code, 0);
    }
    print!("{pnl}\n");

    // 216 Color "cubes"
    for r in 16..22 {
        for code in (r..r + 103).step_by(6) {
            let f = match code {
                16..34 | 52..70 | 88..106 => 255,
                34..52 | 70..88 | 106..124 => 0,
                _ => 197,
            };
            show(code, f);
        }
        print!("{pnl}");
    }
    print!("{pnl}");

    for r in 124..130 {
        for code in (r..r + 103).step_by(6) {
            let f = match code {
                124..130 | 160..166 | 196..202 => 255,
                130..160 | 166..196 | 202..232 => 0,
                _ => 197,
            };
            show(code, f);
        }
        print!("{pnl}");
    }
    print!("{pnl}");

    // Gray Scale
    for code in 232..244 {
        show(code, 255);
    }
    print!("{pnl}");
    for code in 244..=255 {
        show(code, 0);
    }
    print!("{pnl}");
}
