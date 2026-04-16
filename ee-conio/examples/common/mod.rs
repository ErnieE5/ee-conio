mod color;
mod header;

pub use color::Color;
pub use header::header_impl;
pub use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn get_named_color_map() -> BTreeMap<Color, (String, String)> {
    //! Generates a map ordered by a pleasing linear color
    //! gradient when enumerated in order.
    let mut ncm = BTreeMap::<Color, (String, String)>::new();

    for (name, code) in ee_conio_engine::named_color_iter() {
        ncm.insert(
            Color::from_str(code).expect("invalid hex string"),
            (code.to_string(), name.to_string()),
        );
    }

    ncm
}
