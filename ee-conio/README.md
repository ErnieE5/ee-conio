
[![Crates.io](https://img.shields.io/crates/v/ee-conio.svg)](https://crates.io/crates/ee-conio/) [![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape] in [Rust][rust] source code.

# Overview
```bash
$ cargo add ee-conio
```
Add Dependency
```toml
[dependencies]
ee-conio = "0.1.0-alpha.1"

```
use the code
```Rust
use ee_conio::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](../screenshots/hello_world.png?raw=true "Screenshot")  


[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/