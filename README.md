| ee-conio | [![Crates.io](https://img.shields.io/crates/v/ee-conio.svg)](https://crates.io/crates/ee-conio/) [![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)|
|:---|:---|
| ee-conio-engine| ![Crates.io](https://img.shields.io/crates/v/ee-conio-engine.svg) [![Docs.rs](https://docs.rs/ee-conio-engine/badge.svg)](https://docs.rs/ee-conio-engine) |
| ee-conio-macro | [![Crates.io](https://img.shields.io/crates/v/ee-conio-macro.svg)](https://crates.io/crates/ee-conio-macro/) [![Docs.rs](https://docs.rs/ee-conio-macro/badge.svg)](https://docs.rs/ee-conio-macro) |


Simple tools to allow more human readable encodings of [ANSI escape sequences][wiki_escape] in [Rust][rust] source code.

# Workspace
This is a cargo workspace that contains the three parts of the "ee-conio" system published via [crates.io].<br/>


```text
ee-conio 
├── ee-conio-engine
└── ee-conio-macro
    └── ee-conio-engine
```

[ee-conio] General use API<br/>
[ee-conio-engine] Shared implementation for compile/run time use<br/>
[ee-conio-macro] is the compile time proc_macro routines<br/>

# Building/Using Locally
```bash
$ cargo build --all --all-targets --examples
```

Cargo.toml (Library is a sibling to your project):
```toml
[dependencies]
ee-conio = { path="../ee-conio/ee-conio" }
```


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
$ cargo run --example colors256
```
![screenshot](screenshots/example_colors256a.png?raw=true "Screenshot")

```Bash
$ cargo run --example colors256 -- --bg --pad
```
![screenshot](screenshots/example_colors256b.png?raw=true "Screenshot")


```Bash
$ cargo run --example shenanigans
```
![screenshot](screenshots/example_shenanigans.png?raw=true "Screenshot")

```Bash
$ cargo run --example names_match -- neon
```
![screenshot](screenshots/example_names_match_neon.png?raw=true "Screenshot")


[ee-conio]: /ee-conio/
[ee-conio-engine]: /ee-conio-engine/
[ee-conio-macro]: /ee-conio-engine/
[crates.io]:https:://crates.io
[wiki_escape]: https://en.wikipedia.org/wiki/ANSI_escape_code
[rust]: https://rust-lang.org/
