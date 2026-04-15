
[![Crates.io](https://img.shields.io/crates/v/ee-conio-macro.svg)](https://crates.io/crates/ee-conio-macro/) [![Docs.rs](https://docs.rs/ee-conio-macro/badge.svg)](https://docs.rs/ee-conio-macro)

Simple tools to allow more human readable encodings of [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
in [Rust](https://rust-lang.org/) source code.

This part of the "ee-conio ecosystem" is the "main focus."

# Overview
Simple use to show color output.
```Rust
use ee_conio_macro::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  


