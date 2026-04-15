
#[test]
fn compiled_macro_rules() {
    use ee_conio::{bg_256, bg_rgb, csi, fg_256, fg_rgb, sgr};

    assert_eq!("\x1b[", csi!(""));
    assert_eq!("\x1b[0m", csi!("0m"));
    assert_eq!("\x1b[K", csi!("K"));
    assert_eq!("\x1b[0m", sgr!(0));
    assert_eq!("\x1b[38;5;227m", fg_256!(227));
    assert_eq!("\x1b[48;5;51m", bg_256!(51));
    assert_eq!("\x1b[38;2;32;229;32m", fg_rgb!(0x20, 0xe5, 0x20));
    assert_eq!("\x1b[48;2;32;229;32m", bg_rgb!(0x20, 0xe5, 0x20));
}

#[test]
fn runtime_rules() {
    use ee_conio::{
        bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb, sgr_code,
    };

    assert_eq!("\x1b[", csi_sequence(""));
    assert_eq!("\x1b[0m", csi_sequence("0m"));
    assert_eq!("\x1b[K", csi_sequence("K"));
    assert_eq!("\x1b[0m", sgr_code(0));
    assert_eq!("\x1b[38;5;227m", fg_color_256(227));
    assert_eq!("\x1b[48;5;196m", bg_color_256(196));
    assert_eq!("\x1b[38;2;32;229;32m", fg_color_rgb(32, 0xe5, 32));
    assert_eq!("\x1b[48;2;32;229;32m", bg_color_rgb(32, 0xe5, 0x20));
}


#[test]
fn named() {
    use ee_conio::{get_named_background_escape, get_named_foreground_escape};

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
