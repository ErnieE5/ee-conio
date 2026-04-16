#[test]
fn formatting_fg1() {
    use ee_conio_macro::cformat;

    let r = "\u{1b}[38;2;247;13;26m";
    assert!(r == cformat!("~[#'Vivid Red']"));
    assert!(r == cformat!("~[#F70D1A]"));
}

#[test]
fn formatting_bg1() {
    use ee_conio_macro::cformat;

    let r = "\u{1b}[48;2;70;27;126m";
    assert!(r == cformat!("~[$'Purple Monster']"));
    assert!(r == cformat!("~[$461B7E]"));
}

#[test]
fn formatting_fg2() {
    use ee_conio_macro::cformat;

    let x0 = "\x1b[38;2;57;255;20m";
    let x1 = cformat!("~[X38;2;57;255;20m]");
    let x2 = cformat!("~[#39FF14]");
    let x3 = cformat!("~[#'Neon Green']");

    assert!(x0 == x1);
    assert!(x0 == x2);
    assert!(x0 == x3);
}

#[test]
fn formatting_bg2() {
    use ee_conio_macro::cformat;

    let x0 = "\x1b[48;2;248;116;49m";
    let x1 = cformat!("~[X48;2;248;116;49m]");
    let x2 = cformat!("~[$F87431]");
    let x3 = cformat!("~[$'Construction Cone Orange']");

    assert!(x0 == x1);
    assert!(x0 == x2);
    assert!(x0 == x3);
}

#[test]
fn transform1() {
    use ee_conio_macro::ctransform;

    let a = ctransform!("~[]");
    let b = ctransform!("~[                            ]");
    let c = "oh my~[   ]";
    ctransform!(
    let d = "oh my~[   ]";
    );

    assert_eq!("", a);
    assert_eq!("", b);
    assert_eq!("oh my~[   ]", c);
    assert_eq!("oh my", d);
    assert_ne!(c, d);
}

#[test]
fn transform2() {
    use ee_conio::{bg_256, bg_rgb, csi, fg_256, fg_rgb, sgr};
    use ee_conio_macro::{ctransform};

    ctransform!(
        let a = "~[c227]";
        let b = "~[C197]";
        let c = "~[x0]";
        let d = "~[XK]";
        let e = "~[$000000]";
        let f = "~[#000000]";
        let g = "~[$'Black']";
        let h = "~[#'Black']";
    );

    assert_eq!("\u{1b}[38;5;227m", a);
    assert_eq!(a, fg_256!("227"));

    assert_eq!("\u{1b}[48;5;197m", b);
    assert_eq!(b, bg_256!("197"));

    assert_eq!("\u{1b}[0m", c);
    assert_eq!(c, sgr!(0));

    assert_eq!("\u{1b}[K", d);
    assert_eq!(d, csi!("K"));

    assert_eq!("\u{1b}[48;2;0;0;0m", e);
    assert_eq!(e, bg_rgb!(0, 0, 0));

    assert_eq!("\u{1b}[38;2;0;0;0m", f);
    assert_eq!(f, fg_rgb!(0, 0, 0));

    assert_eq!("\u{1b}[48;2;0;0;0m", g);
    assert_eq!(g, bg_rgb!(0, 0, 0));

    assert_eq!("\u{1b}[38;2;0;0;0m", h);
    assert_eq!(h, fg_rgb!(0, 0, 0));
}

#[test]
fn transform3() {
    use ee_conio_macro::ctransform;

    ctransform!(
        let t = "~[dhtop]";
        let b = "~[dhbot]";
    );

    assert_eq!("\x1b#3", t);
    assert_eq!("\x1b#4", b);
}
