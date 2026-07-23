|[<img alt="github" src="https://img.shields.io/badge/github-ErnieE5/ee--conio-2B60DE?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ErnieE5/ee-conio)|||
|:---|:---|:---|
|ee-conio       |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio)|[<img alt="docs.rs" src="https://docs.rs/ee-conio/badge.svg" height="20">](https://docs.rs/ee-conio)|
|ee-conio-engine|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-engine)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-engine/badge.svg" height="20">](https://docs.rs/ee-conio-engine)|
|ee-conio-macro |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-macro.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-macro)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-macro/badge.svg" height="20">](https://docs.rs/ee-conio-macro)|
|ee-conio-parse |[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-parse.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ee-conio-parse)|[<img alt="docs.rs" src="https://docs.rs/ee-conio-parse/badge.svg" height="20">](https://docs.rs/ee-conio-parse)|

Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape]
in [Rust][rust] source code.

This library holds the escape sequence primitives for [ee-conio](../ee-conio):
the `macro_rules!` that build a sequence during compilation, and the matching
functions that build the same sequence at run time.

__It has no dependencies__, and it carries none of the lookup tables. The
`~[...]` parser, the keywords and the roughly 1300 named colors live in
[ee-conio-parse](../ee-conio-parse), which is reached only through the proc
macros and so never lands in your binary.

Everything here is re-exported by [ee-conio](../ee-conio). It _may_ be used
directly, but that is not the intent.

Note that the `~[...]` mnemonics and the `cprintln!` family are __not__ part of
this crate — those come from [ee-conio-macro](../ee-conio-macro), via
[ee-conio](../ee-conio).

# Overview
```Rust
use ee_conio_engine::{fg_256, fg_color_256};

// The macros cook into a &'static str while compiling.
const YELLOW: &str = fg_256!(227);
assert_eq!("\x1b[38;5;227m", YELLOW);

// The functions build the same sequence at run time.
assert_eq!("\x1b[38;5;227m", fg_color_256(227));
```

[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/
