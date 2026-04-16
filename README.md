| ee-conio | [![Crates.io](https://img.shields.io/crates/v/ee-conio.svg)](https://crates.io/crates/ee-conio/) [![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)|
|:---|:---|
| ee-conio-engine| ![Crates.io](https://img.shields.io/crates/v/ee-conio-engine.svg) [![Docs.rs](https://docs.rs/ee-conio-engine/badge.svg)](https://docs.rs/ee-conio-engine) |
| ee-conio-macro | [![Crates.io](https://img.shields.io/crates/v/ee-conio-macro.svg)](https://crates.io/crates/ee-conio-macro/) [![Docs.rs](https://docs.rs/ee-conio-macro/badge.svg)](https://docs.rs/ee-conio-macro) |


Simple tools to allow more human readable encodings of [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
in [Rust](https://rust-lang.org/) source code.

# Workspace
This is a cargo workspace that contains the three parts of the "ee-conio" system published on [crates.io].<br/>

```text
ee-conio 
├── ee-conio-engine
└── ee-conio-macro
    └── ee-conio-engine
```

[ee-conio] General use API<br/>
[ee-conio-engine] Shared implementation for compile/run time use<br/>
[ee-conio-macro] is the compile time proc_macro routines<br/>

__This README.md is for building/using the crates locally.__

# Overview 
ee-conio exposes the _general use_ API. 

```bash
cargo add ee-conio
```
```rust
use ee_conio::cprintln;
cprintln!("~[c227 C0]Hello, ~[c51]World~[c196]!");
```
![screenshot](screenshots/hello_world.png?raw=true "Screenshot")

# Building/Using Locally

##### WIP

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


[ee-conio]: /ee-conio/
[ee-conio-engine]: /ee-conio-engine/
[ee-conio-macro]: /ee-conio-engine/
[crates.io]:https:://crates.io