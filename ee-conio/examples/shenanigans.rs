mod common;
use common::*;

use ee_conio::{bg_color_rgb, cprintln, sgr};

pub fn main() {
    cprintln!("~[cls]");
    header!("shenanigans");

    let o = format!("{:^66}", "Text with a gradient background");
    cprintln!("~[#'Traffic White']{}", purple_grad(o));
    cprintln!();

    let t = format!("{:^33}", "Big Text on black background");
    cprintln!("~[$000000 #'Golden Poppy' dhtop]{t}");
    cprintln!("~[$000000 #'Golden Poppy' dhbot]{t}");
    cprintln!();

    let t = format!("{:^33}", "Gradient background");
    let o = purple_grad(t);
    cprintln!("~[#'Unmellow Yellow' dhtop]{}", o);
    cprintln!("~[#'Unmellow Yellow' dhbot]{}", o);
    cprintln!();

    let t1 = format!("{:<33}", "  Large text in terminal output ");
    let t2 = format!("{:<33}", "  is FUN when done right.");
    let o1 = other_grad(t1);
    let o2 = other_grad(t2);
    cprintln!("~[#'Vampire Black' dhtop]{}", o1);
    cprintln!("~[#'Vampire Black' dhbot]{}", o1);
    cprintln!("~[#'Vampire Black' dhtop]{}", o2);
    cprintln!("~[#'Vampire Black' dhbot]{}", o2);
    cprintln!("~[X16A X80G]\u{1D6D1}~[X16B]");
    cprintln!();
    cprintln!();
}

use enterpolation::{
    Curve,
    linear::{ConstEquidistantLinear, Linear},
};

use palette::{IntoColor, LinSrgb, Srgb};

use std::str::FromStr;

fn purple_grad(out: String) -> String {
    let a1: LinSrgb = Srgb::from_str("#DF00FF").unwrap().into_linear();
    let a2: LinSrgb = Srgb::from_str("#6A287E").unwrap().into_linear();
    let a3: LinSrgb = Srgb::from_str("#4C0255").unwrap().into_linear();

    let grad = Linear::builder()
        .elements([a1, a2, a3, a2, a1])
        .knots([0.0, 0.25, 0.5, 0.75, 1.0])
        .build()
        .expect("bad gradient");

    let mut s = String::new();
    for (lc, (_, ch)) in grad.take(out.len()).zip(out.chars().enumerate()) {
        let c = Srgb::<u8>::from_linear(lc.into_color());
        let (r, g, b) = c.into_components();
        let a = bg_color_rgb(r, g, b);
        s.push_str(format!("{a}{ch}").as_str());
    }
    s.push_str(sgr!(0));
    s
}

fn other_grad(out: String) -> String {
    let grad = ConstEquidistantLinear::<f64, _, 3>::equidistant_unchecked([
        Srgb::from_str("#55DD22").unwrap().into_linear(),
        Srgb::from_str("#FF5555").unwrap().into_linear(),
        Srgb::from_str("#22AAFF").unwrap().into_linear(),
    ]);

    let mut s = String::new();
    for (lc, (_, ch)) in grad.take(out.len()).zip(out.chars().enumerate()) {
        let c = Srgb::<u8>::from_linear(lc.into_color());
        let (r, g, b) = c.into_components();
        let a = bg_color_rgb(r, g, b);
        s.push_str(format!("{a}{ch}").as_str());
    }
    s.push_str(sgr!(0));
    s
}
