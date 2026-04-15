mod common;
use common::*;
use ee_conio::{sgr, sgr_code};

fn main() {
    header!("Test of terminal SGR support.");

    for i in 1..111 {
        let txt = format!("SGR {}", i);
        let sgr = sgr_code(i);
        let brk = if i % 5 == 0 {
            concat!(sgr!(0), "\n")
        } else {
            sgr!(0)
        };

        print!("{}|{:^10}|{}", sgr, txt, brk);
    }

    println!("\n\n");
}
