

use ee_conio::{ cprintln, cprint };

fn main() {
    cprintln!();
    cprintln!("~[c51 C0]Hello~[c7], ~[c227]World~[c197]!");
    cprintln!();

    cprintln!();
    cprintln!("~[C0 #'Vivid Red']This is Vivid Red.        ");
    cprintln!("~[C0 #F70D1A     ]This is also Vivid Red.   ");
    cprintln!("~[C0 c196        ]8bit red color.           ");
    cprintln!("~[C0 x31         ]4bit red color.           ");
    cprintln!("");

    println!("");
    println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is Vivid Red.        \u{1b}[0m");
    println!("\u{1b}[48;5;0m\u{1b}[38;2;247;13;26mThis is also Vivid Red.   \u{1b}[0m");
    println!("\u{1b}[48;5;0m\u{1b}[38;5;196m8bit red color.           \u{1b}[0m");
    println!("\u{1b}[48;5;0m\u{1b}[31m4bit red color.           \u{1b}[0m");
    println!("");

    cprintln!("~[c227 C0]Bright Yellow text on a black background!");
    cprint!(  "~[c227 C0]Bright Yellow text on a black background!~[x0]\n");
    println!("");

}
