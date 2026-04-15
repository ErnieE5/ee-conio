
[![Crates.io](https://img.shields.io/crates/v/ee-conio.svg)](https://crates.io/crates/ee-conio/) [![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)

Simple tools to allow more human readable encodings of [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
in [Rust](https://rust-lang.org/) source code.

# Overview
The **primary** _intended_ use is with ee_conio_macro. 
```Rust
use ee_conio_macro::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  

This crate is used by ee-conio-macro and exposes "more primitive" helpers that aren't proc_macros.  The examples folder contains uses of BOTH crates.


