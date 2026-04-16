#[test]
fn named() {
    use ee_conio_engine::{get_named_background_escape, get_named_foreground_escape};

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
