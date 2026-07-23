# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Note: `D:\Applications\CLAUDE.md` (the parent container) also applies; this file is the authoritative one for `ee-conio`.

## What this is

A cargo workspace (published on crates.io) that turns human-readable mnemonics like `~[c227 C0]` or `~[#'Vivid Red']` into [ANSI escape sequences][ansi], **at compile time** inside string literals. `cprintln!("~[c51]Hello")` expands to `println!("\u{1b}[38;5;51mHello\u{1b}[0m")` — no runtime cost, no escape soup in the source.

Four crates, all at the same version (`0.1.0-alpha.5`), all `edition = "2024"`, all `unsafe_code = "forbid"`:

```
ee-conio             facade — re-exports what users need; owns all examples and the user-facing docs
├── ee-conio-engine  escape primitives: macro_rules! + format! wrappers.  ZERO dependencies.
└── ee-conio-macro   proc-macro: cprintln!/cprint!/cformat!/cwrite!/cwriteln!/ctransform!
    ├── ee-conio-engine
    └── ee-conio-parse   ~[...] parser, keyword + color tables.  Owns the `regex` dependency.
        └── ee-conio-engine
```

**The shape of this graph is deliberate.** `ee-conio-parse` hangs only off the proc-macro crate, which makes it a *host* dependency — so `regex` and the ~1300-entry color table never enter a downstream user's target graph or binary. `ee-conio-engine` is what the facade actually links, and it must stay dependency-free. Verify with `cargo tree -p ee-conio -e normal`: `regex` must appear only underneath the `(proc-macro)` node.

Adding a runtime feature that needs parsing or the tables would undo this — put it behind an opt-in dependency on `ee-conio-parse` rather than promoting it into the engine.

The deps are `{ path = "...", version = "..." }` pairs, so **version bumps must happen in all four manifests together** or `cargo publish` breaks. Publish order: `ee-conio-engine` → `ee-conio-parse` → `ee-conio-macro` → `ee-conio`.

## Commands

Run from the workspace root:

```bash
cargo build --all --all-targets --examples
cargo test --workspace              # integration tests + doctests (docs carry real assertions)
cargo test transform2               # single test by name filter
cargo clippy --all-targets --all-features
cargo run --example colors256       # examples live only in ee-conio/
cargo run --example colors256 -- --bg --pad
cargo run --example names_match -- neon
```

Available examples: `all_named_colors`, `colors256`, `colors_near`, `compile_vs_runtime`, `document_screenshots`, `names_match`, `sgr_table`, `shenanigans`, `smorgasbord`, `suggest_contrasting_color`, `wcag21`.

To see what a macro actually emits (the primary debugging tool for this codebase):

```bash
cargo install cargo-expand
cd ee-conio && cargo expand --example document_screenshots
```

`document_screenshots` is the example that produces the images embedded in `ee-conio/src/lib.rs` docs — regenerate `screenshots/` from it when doc output changes.

## Architecture

### The escape builders are shared between compile time and runtime — that is the whole design

`ee-conio-engine` is a *normal* library, not a proc-macro crate. `ee-conio-parse` calls it during compilation; user code calls it at runtime. Both paths bottom out in the same `macro_rules!` in `ee-conio-engine/src/macros.rs`:

```
esc!() → csi!() → sgr!() → fg_256!/bg_256!/fg_rgb!/bg_rgb!
```

These are pure `concat!` chains, so they cook to `&'static str`. The runtime functions in `ansi_escape.rs` (`fg_color_256`, `bg_color_rgb`, …) are `format!` wrappers around those same macros and return `String`. Compile-time and runtime results are therefore guaranteed identical — preserve that property when adding a new escape kind (add the `macro_rules!` first, then the `format!` wrapper on top of it).

### Compile-time pipeline

`ee-conio-macro/src/lib.rs` — every public macro is one line delegating to `remap_token_stream(wrap, input, at_end)`:

| macro | `wrap` | `at_end` |
|---|---|---|
| `cprintln!` / `cwriteln!` | `println` / `writeln` | `EndWith::Reset` |
| `cprint!` / `cformat!` / `cwrite!` | `print` / `format` / `write` | `EndWith::Nothing` |
| `ctransform!` | `None` (rewrites literals in place, emits the body unchanged) | `Nothing` |

`eeimpl.rs::process_stream` walks the `TokenStream` recursively and touches **only** `Literal` tokens; idents, puncts, and group structure pass through untouched. Per literal:

1. `unescape_literal` — `proc_macro2` can't hand back a `Literal`'s cooked value on stable, so it round-trips through `syn::LitStr` + `rustc_literal_escaper::unescape_str`.
2. `find_replacement_patterns` (parse) — regex-scans for `~[...]`, returning `Vec<(whole_match, Vec<escape>)>`.
3. String-replace each match with its joined escapes.
4. `EndWith::Reset` appends `sgr!(0)` — but only to the **first** literal at the top level (`if tokens.is_empty()`), and only if replacements were found. Nested groups recurse with `noop`, never appending a reset.

A literal containing no `~[...]` is cloned through verbatim, so wrapping unrelated code in `ctransform!` is a no-op.

### Mnemonic resolution (`parse/transform.rs`)

`transform_all` splits the inside of `~[...]` on whitespace, with the exception `[#$@]'quoted name'` which is kept as one token. For each token:

1. `get_keyword` (`keywords.rs`) — exact match against `NAMED_ESCAPES` (`underline`, `cls`, `BLUE`, …) or `NAMED_ETC` (emoji, DEC double-height `dhtop`/`dhbot`).
2. Otherwise `transform_one` — walks the ordered `RE_TRANS` list of `(regex, handler)` pairs and takes the first non-empty result. **Order matters** (`c`/`C` 256-color first, then `x` SGR, `X` CSI, then named `#'…'`/`$'…'`, then `#RRGGBB`/`$RRGGBB`). Adding a pattern that could shadow an earlier one means inserting it in the right place, not appending.
3. No match → `ParseError`, which becomes a compile error.

`~[]` and `~[   ]` are legal and expand to nothing — that's how the docs' alignment padding works.

Grammar, for reference: `x{0..108}` SGR · `c{0..255}` fg 256 · `C{0..255}` bg 256 · `#RRGGBB` fg rgb · `$RRGGBB` bg rgb · `#'Name'` / `$'Name'` named color · `X{seq}` raw CSI (`X2J`, `XK`, `X5;5H`).

### Lookup tables and the initializer hazard

`keywords.rs` and `named_colors.rs` build `LazyLock` maps whose *values* are produced by calling `transform_one` on short mnemonic strings — i.e. the tables are defined in terms of the transformer. `KEYWORDS` therefore **must not** call `transform_all` (which consults `get_keyword`) from inside its initializer; it splits on spaces and calls `transform_one` per piece instead. There is a comment marking this; re-introducing `transform_all` there deadlocks. `named_colors.rs` builds fg and bg variants of each color from one table by swapping the `#` prefix for `$`.

`NAME_RGB_COLORS` is ~1300 entries and is deliberately ordered for a pleasing gradient when iterated — don't sort it. `named_color_iter` / `match_name_iter` exist for the examples, which take `ee-conio-parse` as a dev-dependency. `match_name_iter` compiles a user-supplied pattern at runtime and is the one place regex is genuinely irreplaceable.

### Error spans

`ParseError` (`helpers.rs`) carries byte `start`/`end` offsets, and `wrap()` re-bases them as the error propagates up (`transform_one` → `transform_all` → `find_replacement_patterns`), so offsets stay relative to the whole literal. `eeimpl.rs` feeds them to `Literal::subspan` to underline the exact offending characters. Break the offset arithmetic and errors silently point at the wrong column. `subspan` only yields a real span on nightly; on stable it returns `None` and the code falls back to the whole-literal span.

## Conventions and gotchas

- Docs in `ee-conio/src/lib.rs` are the user-facing manual *and* part of the test suite (8 doctests). Recent history is a run of docs.rs build failures — verify with `cargo test --doc -p ee-conio` and, for docs.rs-specific breakage, `cargo doc -p ee-conio`. Doc images are plain `raw.githubusercontent.com` URLs; the `embed-doc-image` dependency and `doc-images` feature in `ee-conio/Cargo.toml` are leftovers from the previous approach and are no longer referenced.
- Examples share `ee-conio/examples/common/` (`mod common;` + `use common::*;`) providing the `header!` macro and a `Color` type built on `palette`. `palette`, `enterpolation`, and `ordered-float` are dev-dependencies for examples only — never pull them into `src/`.
- Style is exploratory and personal: commented-out alternatives left in place, `todo!()` on unreachable branches, `.expect("this is bad 3")` on table-initializer paths, column-aligned literals under `#[rustfmt::skip]`. Match it rather than tidying it.
- Version is alpha; the public surface is whatever `ee-conio/src/lib.rs` re-exports. Engine and macro crates are published but "may be used directly, but that is not the intent."

[ansi]: https://en.wikipedia.org/wiki/ANSI_escape_code
