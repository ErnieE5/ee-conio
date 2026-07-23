|[<img alt="github" src="https://img.shields.io/badge/github-ErnieE5/ee--conio-2B60DE?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ErnieE5/ee-conio)|||
|:---|:---|:---|
|ee-conio       |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio)|[<img alt="docs.rs" src="https://docs.rs/ee-conio/badge.svg" height="20">](https://docs.rs/ee-conio)|
|ee-conio-engine|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-engine)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-engine/badge.svg" height="20">](https://docs.rs/ee-conio-engine)|
|ee-conio-macro |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-macro.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-macro)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-macro/badge.svg" height="20">](https://docs.rs/ee-conio-macro)|
|ee-conio-parse |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-parse.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-parse)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-parse/badge.svg" height="20">](https://docs.rs/ee-conio-parse)|

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape]
in [Rust][rust] source code.

This library implements the `~[...]` mnemonic parser and the keyword/named color
tables for [ee-conio](../ee-conio).

[ee-conio-macro](../ee-conio-macro) consumes it __at compile time__, which is why
the `regex` dependency and the ~1300 entry color table never reach your binary.

It _may_ be used directly if you want the transforms or the color tables at run
time, but that is not the intent.

# Overview
```Rust
use ee_conio_parse::{named_color_iter, transform_all};

assert_eq!(transform_all("c227 C0").unwrap().concat(), "\x1b[38;5;227m\x1b[48;5;0m");

for (name, rgb) in named_color_iter() {
    println!("{name:32} {rgb}");
}
```

[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/
