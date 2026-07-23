use crate::*;

use std::fmt::Display;

pub fn fg_color_256<T>(c: T) -> String
where
    T: Display,
{
    format!(fg_256!("{}"), c)
}

pub fn fg_color_rgb<T>(r: T, g: T, b: T) -> String
where
    T: Display,
{
    format!(fg_rgb!("{}", "{}", "{}"), r, g, b)
}

pub fn bg_color_256<T>(c: T) -> String
where
    T: Display,
{
    format!(bg_256!("{}"), c)
}

pub fn bg_color_rgb<T>(r: T, g: T, b: T) -> String
where
    T: Display,
{
    format!(bg_rgb!("{}", "{}", "{}"), r, g, b)
}

pub fn sgr_code<T>(c: T) -> String
where
    T: Display,
{
    format!(sgr!("{}"), c)
}

pub fn csi_sequence(s: &str) -> String {
    format!(csi!("{}"), s)
}
