use palette::{
    FromColor, Hsv, IntoColor, Lab, Srgb,
    color_difference::{ImprovedCiede2000, Wcag21RelativeContrast},
};

use std::str::FromStr;

use std::{cmp::Ordering, fmt};

use ee_conio::{bg_color_rgb, fg_color_rgb};

/// The Color structure is not overly memory helpful and
/// is used mostly for helping show interesting things that
/// can be done with the library.
#[derive(Clone, Debug, Copy)]
#[allow(dead_code)]
pub struct Color {
    /// rgb as 3 u8 values
    pub rgb: Srgb<u8>,
    /// rgb as 3 f32 values normalized  0.0 .. 1.0
    pub rgb32: Srgb<f32>,
    /// as HSV
    pub hsv: Hsv,
}

#[derive(Clone, Debug, Copy)]
#[allow(dead_code)]
pub struct Wcag21 {
    pub relative_luminance: f32,
    pub relative_luminance_other: f32,
    pub contrast: f32,
    pub min_contrast_text: bool,
    pub min_contrast_large_text: bool,
    pub enhanced_contrast_text: bool,
    pub enhanced_contrast_large_text: bool,
    pub min_contrast_graphics: bool,
}

#[allow(dead_code)]
impl Wcag21 {
    pub fn mark(&self, t: bool) -> String {
        if t {
            String::from("✅")
        } else {
            String::from("❌")
        }
    }

    pub fn test_icons(&self) -> String {
        let mut icons = String::new();
        icons.push_str(&self.mark(self.min_contrast_text));
        icons.push_str(&self.mark(self.min_contrast_large_text));
        icons.push_str(&self.mark(self.enhanced_contrast_text));
        icons.push_str(&self.mark(self.enhanced_contrast_large_text));
        icons.push_str(&self.mark(self.min_contrast_graphics));
        icons
    }
}

#[allow(dead_code)]
impl Color {
    /// Create a Color from an RGB color spec'd as
    /// a hex string #FFFFFF
    ///
    pub fn from_str(code: &str) -> Result<Color, String> {
        let rgb = match Srgb::from_str(code) {
            Ok(x) => x,
            Err(e) => return Err(format!("{}", e)),
        };

        let rgb32: Srgb<f32> = rgb.into_linear().into_color();
        let hsv = Hsv::from_color(rgb32);

        Ok(Color { rgb, rgb32, hsv })
    }

    /// used by the cmp implementation
    fn sort_key(&self) -> (f32, f32, f32) {
        // The ideas more or less come from these:
        // https://www.alanzucconi.com/2015/09/30/colour-sorting/
        // https://devforum.roblox.com/t/a-journey-of-color-sorting/29876
        //
        let h = (self.hsv.hue.into_positive_degrees() / 45.0).floor();
        let l = self.rgb32.relative_luminance().luma;
        let v = self.hsv.value;

        (h, l, v)
    }

    ///
    pub fn to_string(&self) -> String {
        format!("#{:X}", self.rgb)
    }

    ///
    pub fn into_components(&self) -> (u8, u8, u8) {
        self.rgb.into_components()
    }

    // pub fn hue(&self) -> f32 {
    //     self.hsv.hue.into_positive_degrees()
    // }

    ///
    pub fn luma(&self) -> f32 {
        self.rgb32.relative_luminance().luma
    }

    // pub fn sat(&self) ->f32 {
    //     self.hsv.saturation
    // }

    ///
    pub fn diff(&self, other: &Self) -> f32 {
        let t1 = Lab::from_color(self.rgb32);
        let t2 = Lab::from_color(other.rgb32);

        t1.improved_difference(t2)
    }

    /// Quick way to ensure that the background will be readable with this
    /// Color as a foreground color.
    pub fn bg_best_contrast(&self) -> String {
        if self.luma() < 0.08 {
            bg_color_rgb(255, 255, 255)
        } else {
            bg_color_rgb(0, 0, 0)
        }
    }

    /// Quick way to ensure that the foreground will be readable with this
    /// Color as a background color.
    pub fn fg_best_contrast(&self) -> String {
        if self.luma() < 0.3 {
            fg_color_rgb(255, 255, 255)
        } else {
            fg_color_rgb(0, 0, 0)
        }
    }

    pub fn relative_contrast(&self, other: &Self) -> f32 {
        self.rgb32.relative_contrast(other.rgb32)
    }

    pub fn meets_text_min_contrast(&self, other: &Self) -> Option<f32> {
        if self.rgb32.has_min_contrast_text(other.rgb32) {
            Some(self.relative_contrast(&other))
        } else {
            None
        }
    }

    /// Runs all the tests for WCAG 2.1
    pub fn wcag21_test(&self, other: &Self) -> Wcag21 {
        let relative_luminance = self.rgb32.relative_luminance().luma;
        let relative_luminance_other = other.rgb32.relative_luminance().luma;
        let contrast = self.relative_contrast(other);
        let min_contrast_text = self.rgb32.has_min_contrast_text(other.rgb32);
        let min_contrast_large_text = self.rgb32.has_min_contrast_large_text(other.rgb32);
        let enhanced_contrast_text = self.rgb32.has_enhanced_contrast_text(other.rgb32);
        let enhanced_contrast_large_text = self.rgb32.has_enhanced_contrast_large_text(other.rgb32);
        let min_contrast_graphics = self.rgb32.has_min_contrast_graphics(other.rgb32);

        Wcag21 {
            relative_luminance,
            relative_luminance_other,
            contrast,
            min_contrast_text,
            min_contrast_large_text,
            enhanced_contrast_text,
            enhanced_contrast_large_text,
            min_contrast_graphics,
        }
    }
}

/// Display shows the "sort keys"
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (a, b, c) = self.sort_key();
        f.write_fmt(format_args!(" a:{a:7.3} b:{b:7.3} c:{c:7.3} "))
    }
}

/// orders Colors based on the sort_key method
impl Ord for Color {
    fn cmp(&self, other: &Self) -> Ordering {
        let (sa, sb, sc) = self.sort_key();
        let (oa, ob, oc) = other.sort_key();

        sa.total_cmp(&oa)
            .then(sb.total_cmp(&ob))
            .then(sc.total_cmp(&oc))
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}

impl Eq for Color {}
