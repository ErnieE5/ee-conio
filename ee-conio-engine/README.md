|[<img alt="github" src="https://img.shields.io/badge/github-ErnieE5/ee--conio-2B60DE?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ErnieE5/ee-conio)|||
|:---|:---|:---|
|ee-conio       |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio)|[<img alt="docs.rs" src="https://docs.rs/ee-conio/badge.svg" height="20">](https://docs.rs/ee-conio)|
|ee-conio-engine|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-engine)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-engine/badge.svg" height="20">](https://docs.rs/ee-conio-engine)|
|ee-conio-macro |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-macro.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-macro)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-macro/badge.svg" height="20">](https://docs.rs/ee-conio-macro)|

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape]
in [Rust][rust] source code.

This library contains shared code for [ee-conio](../ee-conio).

Most of the functionality in this library is either exposed via [ee-conio](../ee-conio) or [ee-conio-macro](../ee-conio-macro).  It _may_ be used directly, but that is not the intent. A few functions are helpful for examples and other "a-typical" use. Some of these helpers can _increase_ the linked binary size significantly. The _intended_ use of this library is mostly for compile time macros ... unless you use the supporting functions. 

# Overview
Simple use to show color output.
```Rust
use ee_conio_engine::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  

[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/
