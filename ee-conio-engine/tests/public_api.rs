//! The example in this crate's README, kept honest.
//!
//! The previous README showed `use ee_conio_engine::cprintln;`, which never
//! existed here -- `cprintln!` is a proc macro in ee-conio-macro.  Keeping the
//! example as a test means the next wrong one fails the build.

#[test]
fn readme_overview() {
    use ee_conio_engine::{fg_256, fg_color_256};

    // The macros cook into a &'static str while compiling.
    const YELLOW: &str = fg_256!(227);
    assert_eq!("\x1b[38;5;227m", YELLOW);

    // The functions build the same sequence at run time.
    assert_eq!("\x1b[38;5;227m", fg_color_256(227));
}

#[test]
fn compile_time_and_run_time_agree() {
    use ee_conio_engine::{
        bg_256, bg_color_256, bg_color_rgb, bg_rgb, csi, csi_sequence, esc, fg_256, fg_color_256,
        fg_color_rgb, fg_rgb, sgr, sgr_code,
    };

    // Every macro/function pair must produce identical output.  This is the
    // property the crate split was designed to preserve.
    assert_eq!("\x1b", esc!());
    assert_eq!(csi!("K"), csi_sequence("K"));
    assert_eq!(sgr!(0), sgr_code(0));
    assert_eq!(fg_256!(227), fg_color_256(227));
    assert_eq!(bg_256!(196), bg_color_256(196));
    assert_eq!(fg_rgb!(57, 255, 20), fg_color_rgb(57, 255, 20));
    assert_eq!(bg_rgb!(0, 0, 0), bg_color_rgb(0, 0, 0));
}
