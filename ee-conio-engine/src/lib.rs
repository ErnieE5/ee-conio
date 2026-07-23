//!# Quick Start
//![![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)
//!
//!See  [ee_conio](https://docs.rs/ee-conio)
//!
//! Escape sequence primitives, with no dependencies.  The `~[...]` mnemonic
//! parser and the color/keyword tables live in [ee-conio-parse], which is
//! pulled in by the proc macros at compile time only.
//!
//! [ee-conio-parse]: https://docs.rs/ee-conio-parse

pub mod ansi_escape;
pub mod macros;

pub use crate::ansi_escape::{
    bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb, sgr_code,
};
