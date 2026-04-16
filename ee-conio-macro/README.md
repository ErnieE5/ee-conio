
[![Crates.io](https://img.shields.io/crates/v/ee-conio-macro.svg)](https://crates.io/crates/ee-conio-macro/) [![Docs.rs](https://docs.rs/ee-conio-macro/badge.svg)](https://docs.rs/ee-conio-macro)

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape]
in [Rust][rust] source code.

This library implements proc_macro's for ee-conio.

It _may_ be used directly, but that is not the intent. 

# Overview
Simple use to show color output.
```Rust
use ee_conio_macro::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  

[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/