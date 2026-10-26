//! Exact Radix Colors strings and scoped, script-free CSS assets.
//!
//! sRGB is always available. Enable `p3` to query Display P3 data in Rust;
//! static P3 CSS files remain independently usable and import their sRGB fallback.
//!
//! ```
//! use rustify_colors::{ColorFamily, ColorGamut, ColorMode};
//! let scale = ColorFamily::Indigo.scale(ColorMode::Light, ColorGamut::Srgb, false);
//! assert_eq!(scale.step(9), Some("#3e63dd"));
//! assert_eq!(scale.step(0), None);
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod families;
#[cfg(feature = "p3")]
mod p3;
mod srgb;

pub use families::ColorFamily;

/// The explicitly selected palette appearance; never inferred from the page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    /// Light palette.
    Light,
    /// Dark palette.
    Dark,
}

/// The color space of the original, unmodified strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorGamut {
    /// Standard RGB, supported without an optional feature.
    Srgb,
    /// Wide-gamut Display P3; opt in with the `p3` crate feature.
    #[cfg(feature = "p3")]
    P3,
}

/// Twelve original strings in ascending scale order. No normalization is applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorScale(&'static [&'static str; 12]);

impl ColorScale {
    /// Looks up a one-based step. Zero and values above twelve return `None`.
    pub fn step(self, step: u8) -> Option<&'static str> {
        step.checked_sub(1)
            .and_then(|i| self.0.get(usize::from(i)))
            .copied()
    }

    /// All twelve strings in scale order.
    pub fn steps(self) -> &'static [&'static str; 12] {
        self.0
    }
}

/// A packaged stylesheet. Copy both the P3 file and its sibling sRGB fallback
/// when deploying an optional P3 asset; the import is relative to `path`.
#[derive(Clone, Copy, Debug)]
pub struct CssAsset {
    /// Path relative to the crate's package root.
    pub path: &'static str,
    /// Contents, available through the package without accessing its source tree.
    pub content: &'static str,
}

impl ColorFamily {
    /// Gets a light/dark, solid/alpha scale in the selected color space.
    pub fn scale(self, mode: ColorMode, gamut: ColorGamut, alpha: bool) -> ColorScale {
        match gamut {
            ColorGamut::Srgb => srgb::scale(self, mode, alpha),
            #[cfg(feature = "p3")]
            ColorGamut::P3 => p3::scale(self, mode, alpha),
        }
    }

    /// Gets the independently consumable CSS asset for this scale.
    pub fn css(self, mode: ColorMode, gamut: ColorGamut, alpha: bool) -> CssAsset {
        match gamut {
            ColorGamut::Srgb => srgb::css(self, mode, alpha),
            #[cfg(feature = "p3")]
            ColorGamut::P3 => p3::css(self, mode, alpha),
        }
    }
}

/// Mode-independent black alpha scale.
pub fn black_alpha(gamut: ColorGamut) -> ColorScale {
    match gamut {
        ColorGamut::Srgb => srgb::BLACK_ALPHA,
        #[cfg(feature = "p3")]
        ColorGamut::P3 => p3::BLACK_ALPHA,
    }
}

/// Mode-independent white alpha scale.
pub fn white_alpha(gamut: ColorGamut) -> ColorScale {
    match gamut {
        ColorGamut::Srgb => srgb::WHITE_ALPHA,
        #[cfg(feature = "p3")]
        ColorGamut::P3 => p3::WHITE_ALPHA,
    }
}

/// Mode-independent black alpha CSS asset.
pub fn black_alpha_css(gamut: ColorGamut) -> CssAsset {
    match gamut {
        ColorGamut::Srgb => srgb::BLACK_ALPHA_CSS,
        #[cfg(feature = "p3")]
        ColorGamut::P3 => p3::BLACK_ALPHA_CSS,
    }
}

/// Mode-independent white alpha CSS asset.
pub fn white_alpha_css(gamut: ColorGamut) -> CssAsset {
    match gamut {
        ColorGamut::Srgb => srgb::WHITE_ALPHA_CSS,
        #[cfg(feature = "p3")]
        ColorGamut::P3 => p3::WHITE_ALPHA_CSS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_scale_has_twelve_original_ordered_strings_and_scoped_css() {
        assert_eq!(ColorFamily::ALL.len(), 31);
        for family in ColorFamily::ALL {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                for alpha in [false, true] {
                    let scale = family.scale(mode, ColorGamut::Srgb, alpha);
                    let asset = family.css(mode, ColorGamut::Srgb, alpha);
                    for step in 1..=12 {
                        let value = scale.step(step).unwrap();
                        let a = if alpha { "a" } else { "" };
                        assert!(asset
                            .content
                            .contains(&format!("--{}-{a}{step}: {value};", family.name())));
                        assert!(!value.contains("display-p3"));
                    }
                    assert_eq!(scale.step(0), None);
                    assert_eq!(scale.step(13), None);
                    assert!(!asset.content.contains(":root"));
                    assert!(!asset.content.contains("@import"));
                    assert!(asset.content.contains("[data-rustify-colors~="));
                }
            }
        }
    }

    #[cfg(feature = "p3")]
    #[test]
    fn every_p3_asset_imports_its_srgb_fallback_before_gated_enhancement() {
        for family in ColorFamily::ALL {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                for alpha in [false, true] {
                    let srgb = family.css(mode, ColorGamut::Srgb, alpha);
                    let p3 = family.css(mode, ColorGamut::P3, alpha);
                    assert!(p3.content.starts_with("/* Generated"));
                    assert!(p3.content.contains(&format!(
                        "@import \"./{}\";",
                        srgb.path.strip_prefix("css/").unwrap()
                    )));
                    assert!(p3
                        .content
                        .contains("@supports (color: color(display-p3 1 1 1))"));
                    assert!(p3.content.contains("@media (color-gamut: p3)"));
                    assert!(family
                        .scale(mode, ColorGamut::P3, alpha)
                        .steps()
                        .iter()
                        .all(|v| v.starts_with("color(display-p3 ")));
                }
            }
        }
    }

    #[test]
    fn black_and_white_are_mode_independent_alpha_scales() {
        assert_eq!(
            black_alpha(ColorGamut::Srgb).step(1),
            Some("rgba(0, 0, 0, 0.05)")
        );
        assert_eq!(
            white_alpha(ColorGamut::Srgb).step(12),
            Some("rgba(255, 255, 255, 0.95)")
        );
        assert!(black_alpha_css(ColorGamut::Srgb)
            .content
            .contains("--black-a12:"));
        assert!(white_alpha_css(ColorGamut::Srgb)
            .content
            .contains("--white-a12:"));
    }
}
