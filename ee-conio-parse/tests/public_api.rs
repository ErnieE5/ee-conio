#[test]
fn named() {
    use ee_conio_parse::{get_named_background_escape, get_named_foreground_escape};

    let x0 = "\x1b[38;2;132;222;2m";
    assert_eq!(
        x0,
        get_named_foreground_escape("Alien Armpit").expect("found")
    );

    let x1 = "\x1b[48;2;142;58;89m";
    assert_eq!(
        x1,
        get_named_background_escape("Quinacridone Magenta").expect("found")
    );
}

#[test]
fn transforms() {
    use ee_conio_parse::{transform_all, transform_one};

    // Every mnemonic form, resolved without going through the proc macro.
    assert_eq!("\x1b[38;5;227m", transform_one("c227").expect("fg 256"));
    assert_eq!("\x1b[48;5;0m", transform_one("C0").expect("bg 256"));
    assert_eq!("\x1b[0m", transform_one("x0").expect("sgr"));
    assert_eq!("\x1b[K", transform_one("XK").expect("csi"));
    assert_eq!("\x1b[38;2;57;255;20m", transform_one("#39FF14").expect("fg rgb"));
    assert_eq!("\x1b[48;2;0;0;0m", transform_one("$000000").expect("bg rgb"));

    // Space separated, and quoted names survive tokenization intact.
    assert_eq!(
        "\x1b[38;5;227m\x1b[48;5;0m",
        transform_all("c227 C0").expect("pair").concat()
    );
    assert_eq!(
        "\x1b[38;2;247;13;26m",
        transform_all("#'Vivid Red'").expect("named").concat()
    );

    // Keywords resolve ahead of the mnemonic patterns.
    assert_eq!("\x1b[4m", transform_all("underline").expect("keyword").concat());

    assert!(transform_one("{}").is_err());
}
