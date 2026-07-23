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

#[test]
fn error_messages_reach_display() {
    use ee_conio_parse::transform_one;

    // Display used to print a fixed joke string, discarding the msg field.
    let e = transform_one("{}").expect_err("unknown mnemonic");
    assert_eq!(e.msg(), format!("{e}"));
    assert!(format!("{e}").contains("{}"), "got: {e}");
}

#[test]
fn parse_error_is_a_std_error() {
    use ee_conio_parse::transform_one;
    use std::error::Error;

    // Usable with ? into Box<dyn Error> and the anyhow/thiserror ecosystem.
    fn boxed() -> Result<String, Box<dyn Error>> {
        Ok(transform_one("{}")?)
    }

    let e = boxed().expect_err("unknown mnemonic");
    assert!(e.to_string().contains("does not match"), "got: {e}");

    // No underlying cause to report.
    let pe = transform_one("{}").expect_err("unknown mnemonic");
    assert!(pe.source().is_none());
}

#[test]
fn error_spans_point_at_the_offending_text() {
    use ee_conio_parse::{transform_all, transform_one};

    // Whole token for an unknown mnemonic.
    let e = transform_one("qqq").expect_err("unknown");
    assert_eq!(e.span(), 0..3);

    // Just the digits for an out of range color -- not the leading 'c'.
    let e = transform_one("c999").expect_err("out of range");
    assert_eq!(e.span(), 1..4);
    assert_eq!("999", &"c999"[e.span()]);
    assert_eq!((e.start(), e.end()), (1, 4));

    // Just the name for an unknown named color, inside the quotes.
    let e = transform_one("#'Nope'").expect_err("unknown color");
    assert_eq!("Nope", &"#'Nope'"[e.span()]);

    // transform_all must SHIFT that span by the token offset, not replace it
    // with the whole token.  "999" sits at 6..9 of "c227 c999".
    let src = "c227 c999";
    let e = transform_all(src).expect_err("out of range");
    assert_eq!(e.span(), 6..9);
    assert_eq!("999", &src[e.span()]);
}

#[test]
fn quoted_names_are_never_ambiguous_with_hex() {
    use ee_conio_parse::transform_one;

    // "#'abcd'" is '#' plus six characters, so the old `.{6,6}` rgb pattern
    // matched it too -- only push order kept it resolving as a named color.
    let e = transform_one("#'abcd'").expect_err("not a color name");
    assert!(format!("{e}").contains("not a known named color"), "got: {e}");

    // Malformed hex is reported as such rather than as an unknown mnemonic.
    let e = transform_one("#ZZZZZZ").expect_err("bad hex");
    assert!(format!("{e}").contains("six hex digits"), "got: {e}");

    // Valid hex still resolves, upper and lower case.
    assert_eq!("\x1b[38;2;57;255;20m", transform_one("#39FF14").expect("hex"));
    assert_eq!("\x1b[38;2;57;255;20m", transform_one("#39ff14").expect("hex"));
}
