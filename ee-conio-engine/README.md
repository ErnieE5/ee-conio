
[![Crates.io](https://img.shields.io/crates/v/ee-conio-engine.svg)](https://crates.io/crates/ee-conio-engine/) [![Docs.rs](https://docs.rs/ee-conio-engine/badge.svg)](https://docs.rs/ee-conio-engine)

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape]
in [Rust][rust] source code.

This library contains shared code for [ee-conio](../ee-conio).

Most of the fuctionality in this lirarby is either exposed via [ee-conio](../ee-conio) or [ee-conio-macro](../ee-conio-macro).  It _may_ be used directly, but that is not the intent. There are a few functions that are helpful for examples and other "a-typical" use. Some of these helpers can _increase_ the linked binary size. The _intended_ use of this library is mostly for compile time macros ... unless you use the supporting functions. 

# Overview
Simple use to show color output.
```Rust
use ee_conio_engine::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  

[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/