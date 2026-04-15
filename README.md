| ee-conio-macro | [![Crates.io](https://img.shields.io/crates/v/ee-conio-macro.svg)](https://crates.io/crates/ee-conio-macro/) [![Docs.rs](https://docs.rs/ee-conio-macro/badge.svg)](https://docs.rs/ee-conio-macro) |
|-------------|-------------|
| ee-conio | [![Crates.io](https://img.shields.io/crates/v/ee-conio.svg)](https://crates.io/crates/ee-conio/) [![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)|

Simple tools to allow more human readable encodings of [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
in [Rust](https://rust-lang.org/) source code.

# ee-conio Workspace
This is a cargo workspace that contains the two parts of the "ee-conio system" published on [crates.io](https:://crates.io).<br/>

[ee-conio-macro](ee-conio-macro) is the compile time proc_macro routines.<br/>
[ee-conio](ee-conio) is the "not proc_macro" part. <br/>

This README.md is(WIP: will be) about building/using the crates locally.

# Overview
The **primary** _intended_ use is with ee_conio_macro.
```Rust
use ee_conio_macro::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](screenshots/hello_world.png?raw=true "Screenshot")

# Documentation
[ee-conio-macro](https://docs.rs/ee-conio-macro)<br/>
[ee-conio](https://docs.rs/ee-conio)

# Examples

```text
$ cargo run --examples

Available examples:
    all_named_colors
    colors256
    colors_near
    compile_vs_runtime
    names_match
    sgr_table
    shenanigans
    smorgasbord
    suggest_contrasting_color
    wcag21

```

```Bash
cargo run --example colors256
```
![screenshot](screenshots/example_colors256a.png?raw=true "Screenshot")

```Bash
cargo run --example colors256 --bg --pad
```
![screenshot](screenshots/example_colors256b.png?raw=true "Screenshot")


```Bash
cargo run --example shenanigans
```
![screenshot](screenshots/example_shenanigans.png?raw=true "Screenshot")

```Bash
cargo run --example names_match -- neon
```
![screenshot](screenshots/example_names_match_neon.png?raw=true "Screenshot")
