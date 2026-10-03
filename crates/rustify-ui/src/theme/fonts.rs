//! Offline font choices shared by DOM and GPU previews.

use super::{FontFace, ResolveContext};

/// The semantic font slot a catalogue face is designed for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontSlot {
    Sans,
    Serif,
    Mono,
}

/// A face backed by a resource included in the static product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ThemeFont {
    IbmPlexSans,
    NotoSans,
    RustifyWenKai,
    NotoSerif,
    LiberationMono,
    JetBrainsMono,
}

pub const THEME_FONTS: [ThemeFont; 6] = [
    ThemeFont::IbmPlexSans,
    ThemeFont::NotoSans,
    ThemeFont::RustifyWenKai,
    ThemeFont::NotoSerif,
    ThemeFont::LiberationMono,
    ThemeFont::JetBrainsMono,
];

pub const FONT_FACES: [FontFace; 6] = [
    FontFace {
        family: "IBM Plex Sans",
        resource_path: "makepad_widgets/resources/IBMPlexSans-Text.ttf",
    },
    FontFace {
        family: "Noto Sans",
        resource_path: "makepad_widgets/resources/NotoSans-Regular.ttf",
    },
    FontFace {
        family: "Rustify WenKai",
        resource_path: "makepad_widgets/resources/LXGWWenKaiRegular.ttf",
    },
    FontFace {
        family: "Noto Serif",
        resource_path: "makepad_widgets/resources/NotoSerif-Regular.ttf",
    },
    FontFace {
        family: "Liberation Mono",
        resource_path: "makepad_widgets/resources/LiberationMono-Regular.ttf",
    },
    FontFace {
        family: "JetBrains Mono",
        resource_path: "makepad_widgets/resources/jetbrains_mono_variable.ttf",
    },
];

/// Sans, serif and mono fallbacks, matching the `ResolvedTheme.fonts` order.
pub const FONT_FALLBACKS: [&str; 3] = ["IBM Plex Sans", "Noto Serif", "Liberation Mono"];
pub const FONT_GLYPH_FALLBACKS: [FontFace; 2] = [
    FontFace {
        family: "Rustify WenKai",
        resource_path: "makepad_widgets/resources/LXGWWenKaiRegular.ttf",
    },
    FontFace {
        family: "Noto Color Emoji",
        resource_path: "makepad_widgets/resources/NotoColorEmoji.ttf",
    },
];

impl ThemeFont {
    pub const fn face(self) -> FontFace {
        FONT_FACES[self as usize]
    }

    pub const fn slot(self) -> FontSlot {
        match self {
            Self::IbmPlexSans | Self::NotoSans | Self::RustifyWenKai => FontSlot::Sans,
            Self::NotoSerif => FontSlot::Serif,
            Self::LiberationMono | Self::JetBrainsMono => FontSlot::Mono,
        }
    }
}

/// Resolve preview font slots against the same offline resources both renderers use.
pub fn font_context(rem_px: f64, reduce_motion: bool) -> ResolveContext<'static> {
    ResolveContext {
        rem_px,
        reduce_motion,
        fonts: &FONT_FACES,
        fallback_fonts: FONT_FALLBACKS,
    }
}

/// Use the same primary, CJK and emoji order as the GPU family.
pub fn dom_font_stack(font: &super::ResolvedFont) -> String {
    if font.resource_path.is_none()
        && ["sans-serif", "serif", "monospace"].contains(&font.family.as_str())
    {
        return font.family.clone();
    }
    let primary = font.family.replace('\\', "\\\\").replace('"', "\\\"");
    if font.family == "Rustify WenKai" {
        format!("\"{primary}\", \"Noto Color Emoji\"")
    } else {
        format!("\"{primary}\", \"Rustify WenKai\", \"Noto Color Emoji\"")
    }
}

/// Build the GPU family from the resolved resource, with CJK and emoji fallbacks.
#[cfg(target_arch = "wasm32")]
pub fn gpu_font_family(
    cx: &mut crate::makepad_widgets::Cx,
    font: &super::ResolvedFont,
) -> Option<crate::makepad_widgets::makepad_draw::shader::draw_text::FontFamily> {
    let primary = font.resource_path.as_deref()?;
    let mut paths = vec![primary];
    for fallback in FONT_GLYPH_FALLBACKS.map(|font| font.resource_path) {
        if fallback != primary {
            paths.push(fallback);
        }
    }
    crate::makepad_widgets::makepad_draw::shader::draw_text::FontFamily::from_resource_paths(
        cx, &paths,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{resolve, Theme, ThemeDocument, ThemeMode};

    #[test]
    fn catalogue_resources_exist_and_cover_all_slots() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../makepad/widgets/resources");
        for font in THEME_FONTS {
            let file = font.face().resource_path.rsplit('/').next().unwrap();
            assert!(root.join(file).is_file(), "missing font resource: {file}");
        }
        for slot in [FontSlot::Sans, FontSlot::Serif, FontSlot::Mono] {
            assert!(THEME_FONTS.iter().any(|font| font.slot() == slot));
        }
    }

    #[test]
    fn all_catalogue_faces_resolve_to_their_packaged_resource() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for font in THEME_FONTS {
            document
                .styles
                .light
                .0
                .insert("font-sans".into(), font.face().family.into());
            let theme = resolve(&document, ThemeMode::Light, &font_context(16., false)).unwrap();
            assert_eq!(theme.fonts[0].family, font.face().family);
            assert_eq!(
                theme.fonts[0].resource_path.as_deref(),
                Some(font.face().resource_path)
            );
            assert!(!theme.fonts[0].fallback);
        }
    }

    #[test]
    fn unknown_author_stack_survives_and_each_slot_has_visible_packaged_fallback() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for token in ["font-sans", "font-serif", "font-mono"] {
            document
                .styles
                .light
                .0
                .insert(token.into(), "Unknown Author Face, serif".into());
        }
        let theme = resolve(&document, ThemeMode::Light, &font_context(20., false)).unwrap();
        for (index, token) in ["font-sans", "font-serif", "font-mono"].iter().enumerate() {
            assert_eq!(
                document.styles.light.get(token).unwrap(),
                "Unknown Author Face, serif"
            );
            assert_eq!(theme.fonts[index].requested[0], "Unknown Author Face");
            assert_eq!(theme.fonts[index].family, FONT_FALLBACKS[index]);
            assert!(theme.fonts[index].resource_path.is_some());
            assert!(theme
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.field.ends_with(token)));
        }
    }
}
