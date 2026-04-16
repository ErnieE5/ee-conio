| [<img alt="github" src="https://img.shields.io/badge/github-ErnieE5/ee--conio-2B60DE?style=for-the-badge&labelColor=555555&logo=github" height="18">](https://github.com/ErnieE5/ee-conio)| | |
|:---|:---|:---|
|ee-conio|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio.svg?style=for-the-badge&color=fc8d62&logo=rust" height="18">](https://crates.io/crates/ee-conio) | [<img alt="docs.rs" src="https://docs.rs/ee-conio/badge.svg" height="18">](https://docs.rs/ee-conio)|
|ee-conio-engine|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-engine.svg?style=for-the-badge&color=fc8d62&logo=rust" height="18">](https://crates.io/crates/ee-conio-engine) | [<img alt="docs.rs" src="https://docs.rs/ee-conio-engine/badge.svg" height="18">](https://docs.rs/ee-conio-engine)|
|ee-conio-macro|[<img alt="crates.io" src="https://img.shields.io/crates/v/ee-conio-macro.svg?style=for-the-badge&color=fc8d62&logo=rust" height="18">](https://crates.io/crates/ee-conio-macro)| [<img alt="docs.rs" src="https://docs.rs/ee-conio-macro/badge.svg" height="18">](https://docs.rs/ee-conio-macro)|


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
