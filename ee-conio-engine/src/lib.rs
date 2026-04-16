//!
//!
//!# Quick Start
//![![Docs.rs](https://docs.rs/ee-conio/badge.svg)](https://docs.rs/ee-conio)
//!
//!See  [ee_conio](https://docs.rs/ee-conio)
//!

pub mod ansi_escape;
pub mod helpers;
pub mod keywords;
pub mod macros;
pub mod named_colors;

pub use crate::{
    ansi_escape::{
        bg_color_256, bg_color_rgb, csi_sequence, fg_color_256, fg_color_rgb,
        find_replacement_patterns, sgr_code, transform_all, transform_one,
    },
    helpers::r_g_b_from_string,
    keywords::get_keyword,
    named_colors::{
        get_named_background_escape, get_named_foreground_escape, match_name_iter, named_color_iter,
    },
};
