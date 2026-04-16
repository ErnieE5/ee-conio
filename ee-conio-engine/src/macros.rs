#[macro_export]
macro_rules! esc {
    () => {
        "\x1b"
    };
}

#[macro_export]
macro_rules! csi {
    ($a:expr) => {
        concat!($crate::esc!(), "[", $a)
    };
}

#[macro_export]
macro_rules! sgr {
    ($a:expr) => {
        concat!($crate::csi!($a), "m")
    };
}

#[macro_export]
macro_rules! fg_256 {
    ($a:literal) => {
        $crate::sgr!(concat!("38;5;", $a))
    };
}

#[macro_export]
macro_rules! bg_256 {
    ($a:literal) => {
        $crate::sgr!(concat!("48;5;", $a))
    };
}

#[macro_export]
macro_rules! fg_rgb {
    ($r:literal,$g:literal,$b:literal) => {
        $crate::sgr!(concat!("38;2;", $r, ";", $g, ";", $b))
    };
}

#[macro_export]
macro_rules! bg_rgb {
    ($r:literal,$g:literal,$b:literal) => {
        $crate::sgr!(concat!("48;2;", $r, ";", $g, ";", $b))
    };
}
