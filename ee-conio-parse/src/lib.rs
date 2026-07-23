/*!
Mnemonic parser and lookup tables behind [ee-conio].

This crate turns the text inside a `~[...]` marker into ANSI escape sequences.
It is consumed by [ee-conio-macro] at compile time, so for the common case
(`cprintln!` and friends) none of this — including the `regex` dependency and
the ~1300 entry color table — reaches the final binary.

Depend on it directly only if you need the transforms or the color tables at
__runtime__.

[ee-conio]: https://docs.rs/ee-conio
[ee-conio-macro]: https://docs.rs/ee-conio-macro
*/

pub mod helpers;
pub mod keywords;
pub mod named_colors;
pub mod transform;

pub use crate::{
    helpers::{ParseError, r_g_b_from_string},
    keywords::get_keyword,
    named_colors::{
        get_named_background_escape, get_named_foreground_escape, match_name_iter, named_color_iter,
    },
    transform::{find_replacement_patterns, transform_all, transform_one},
};
