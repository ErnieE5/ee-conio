use ee_conio::{cprintln, fg_256, fg_color_256, sgr, sgr_code};

/// Adding a little spice using ANSI 8 bit color mode escapes
fn main() {
    // By hand
    //
    println!("manual   \x1b[38;5;11mHello\x1b[0m, \x1b[38;5;51mWorld\x1b[38;5;255m!\x1b[0m");

    // at compile time
    //
    cprintln!("compiled ~[c11]Hello~[x0], ~[c51]World~[c255]!");

    let h = fg_256!(11);
    let w = fg_256!(51);
    let e = fg_256!(255);
    let r = sgr!(0);
    println!("compiled {h}Hello{r}, {w}World{e}!{r}");

    // at run time
    //
    let h = fg_color_256(11);
    let w = fg_color_256(51);
    let e = fg_color_256(255);
    let r = sgr_code(0);
    println!("runtime  {h}Hello{r}, {w}World{e}!{r}");
}
