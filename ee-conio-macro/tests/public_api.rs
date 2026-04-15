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
