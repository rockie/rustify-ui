//! Offline font choices shared by DOM and GPU previews.

use super::{FontFace, FontResolution, ResolveContext};

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

/// Identity of an offline face, without embedding the font in a DOM build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontResourceDigest {
    pub family: &'static str,
    pub resource_path: &'static str,
    pub sha256: &'static str,
}

/// The six catalogue faces and glyph fallbacks, deduplicated by resource.
pub const FONT_RESOURCE_DIGESTS: [FontResourceDigest; 7] = [
    FontResourceDigest {
        family: "IBM Plex Sans",
        resource_path: "makepad_widgets/resources/IBMPlexSans-Text.ttf",
        sha256: "8c358c56c56f76a28cecb8ca10734c1c9cafa01a752dc8364b9ef7de412437dc",
    },
    FontResourceDigest {
        family: "Noto Sans",
        resource_path: "makepad_widgets/resources/NotoSans-Regular.ttf",
        sha256: "371924e7480d3d01807fdc1bd30442d0a6651e6b5ea546482f5883d613cf0419",
    },
    FontResourceDigest {
        family: "Rustify WenKai",
        resource_path: "makepad_widgets/resources/LXGWWenKaiRegular.ttf",
        sha256: "1d4b2f008ac7e02780c19801d4b04dd4538e40215943cc8ad374dba8ec2278c0",
    },
    FontResourceDigest {
        family: "Noto Serif",
        resource_path: "makepad_widgets/resources/NotoSerif-Regular.ttf",
        sha256: "c8f669ceb2c9c60ccf55198b305e08a997ffca79a38cc7eeb551e643cbe66505",
    },
    FontResourceDigest {
        family: "Liberation Mono",
        resource_path: "makepad_widgets/resources/LiberationMono-Regular.ttf",
        sha256: "3e396ddd3d1c7d3dca1fb16dd64802b32f295f856ac10c0c1f2ab1c738196045",
    },
    FontResourceDigest {
        family: "JetBrains Mono",
        resource_path: "makepad_widgets/resources/jetbrains_mono_variable.ttf",
        sha256: "662a196d58f1183bf2d77428b6d5283fe3f45161ab021bea4036bc98e5cac016",
    },
    FontResourceDigest {
        family: "Noto Color Emoji",
        resource_path: "makepad_widgets/resources/NotoColorEmoji.ttf",
        sha256: "39ee3c587e10e89669b9ff32703261d10d5f9c4dd5ad147b6b5a1c5200591817",
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
        font_resolution: FontResolution::Packaged,
    }
}

/// Use the same primary, CJK and emoji order as the GPU family.
pub fn dom_font_stack(font: &super::ResolvedFont) -> String {
    if font.font_resolution == FontResolution::Css {
        return font.css_stack.clone().unwrap_or_default();
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{resolve, Theme, ThemeDocument, ThemeMode};

    #[test]
    fn resource_digests_cover_the_catalogue_and_glyph_fallbacks_once() {
        let resources: std::collections::BTreeSet<_> = FONT_FACES
            .into_iter()
            .chain(FONT_GLYPH_FALLBACKS)
            .map(|face| (face.family, face.resource_path))
            .collect();
        let digests: std::collections::BTreeSet<_> = FONT_RESOURCE_DIGESTS
            .into_iter()
            .map(|face| {
                assert_eq!(face.sha256.len(), 64);
                assert!(face.sha256.bytes().all(|byte| byte.is_ascii_hexdigit()));
                (face.family, face.resource_path)
            })
            .collect();
        assert_eq!(resources, digests);
        assert_eq!(digests.len(), FONT_RESOURCE_DIGESTS.len());
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

    #[test]
    fn css_resolution_keeps_named_families_without_packaged_resources() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        document
            .styles
            .light
            .0
            .insert("font-sans".into(), "\"Brand, Sans\", system-ui".into());
        let context = ResolveContext {
            font_resolution: super::super::FontResolution::Css,
            ..font_context(16., false)
        };
        let theme = resolve(&document, ThemeMode::Light, &context).unwrap();
        let font = &theme.fonts[0];
        assert_eq!(font.requested, ["Brand, Sans", "system-ui"]);
        assert_eq!(dom_font_stack(font), "\"Brand, Sans\", system-ui");
        assert!(theme.fonts.iter().all(|font| font.resource_path.is_none()));
        assert!(!dom_font_stack(font).contains("WenKai"));
        assert!(!dom_font_stack(font).contains("Emoji"));
    }

    #[test]
    fn css_resolution_distinguishes_quoted_generics_and_preserves_the_policy_in_patches() {
        let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
        for token in ["font-sans", "font-serif", "font-mono"] {
            document
                .styles
                .light
                .0
                .insert(token.into(), "\"sans-serif\"".into());
        }
        let context = ResolveContext {
            font_resolution: super::super::FontResolution::Css,
            ..font_context(16., false)
        };
        let theme = resolve(&document, ThemeMode::Light, &context).unwrap();
        for (font, generic) in theme.fonts.iter().zip(["sans-serif", "serif", "monospace"]) {
            assert_eq!(dom_font_stack(font), format!("\"sans-serif\", {generic}"));
        }
        let patch = crate::theme::ThemeValuePatch::default();
        let patched = theme.patched(&patch).unwrap();
        assert_eq!(patched.fonts, theme.fonts);
    }

    #[test]
    fn css_font_stack_quotes_names_and_refuses_executable_css() {
        let parse = super::super::parse_css_font_stack;
        assert_eq!(
            parse("\"Brand\\\"Family\", sans-serif").unwrap(),
            vec![("Brand\"Family".into(), false), ("sans-serif".into(), true)]
        );
        for value in [
            "x; color:red",
            "url(x)",
            "\"unclosed",
            "A,",
            "inherit",
            "x\nfont",
        ] {
            assert!(parse(value).is_err(), "{value}");
        }
    }
}
