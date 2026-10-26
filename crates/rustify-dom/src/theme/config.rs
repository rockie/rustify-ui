//! Author configuration is separate from generated values and explicit overrides.

use super::{parse_color, ColorFormat, Rgba, ThemeError, ThemeMode, ThemeValuePatch};
use rustify_colors::{ColorFamily, ColorGamut, ColorMode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock};

macro_rules! colors {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[doc = "The supported brand palette choices, separate from semantic accent values."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "lowercase")]
        pub enum $name { $($variant),+ }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            pub const fn family(self) -> ColorFamily {
                match self { $(Self::$variant => ColorFamily::$variant),+ }
            }

            pub const fn as_str(self) -> &'static str { self.family().name() }
        }
    };
}

// The other neutral Colors families are not brands.
colors! { AccentColor {
    Gray, Gold, Bronze, Brown, Yellow, Amber, Orange, Tomato, Red, Ruby, Crimson,
    Pink, Plum, Purple, Violet, Iris, Indigo, Blue, Cyan, Teal, Jade, Green, Grass,
    Lime, Mint, Sky,
}}

impl AccentColor {
    /// The pinned Themes matching-gray rule, without guessing from a color value.
    pub const fn matching_gray(self) -> GrayColor {
        match self {
            Self::Tomato
            | Self::Red
            | Self::Ruby
            | Self::Crimson
            | Self::Pink
            | Self::Plum
            | Self::Purple
            | Self::Violet => GrayColor::Mauve,
            Self::Iris | Self::Indigo | Self::Blue | Self::Sky | Self::Cyan => GrayColor::Slate,
            Self::Teal | Self::Jade | Self::Mint | Self::Green => GrayColor::Sage,
            Self::Grass | Self::Lime => GrayColor::Olive,
            Self::Yellow | Self::Amber | Self::Orange | Self::Brown | Self::Gold | Self::Bronze => {
                GrayColor::Sand
            }
            Self::Gray => GrayColor::Gray,
        }
    }
}

/// Explicit neutral palette or the brand's matching palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GrayColor {
    Auto,
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
}

impl GrayColor {
    pub const ALL: &'static [Self] = &[
        Self::Auto,
        Self::Gray,
        Self::Mauve,
        Self::Slate,
        Self::Sage,
        Self::Olive,
        Self::Sand,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Gray => "gray",
            Self::Mauve => "mauve",
            Self::Slate => "slate",
            Self::Sage => "sage",
            Self::Olive => "olive",
            Self::Sand => "sand",
        }
    }

    pub const fn family(self, accent: AccentColor) -> ColorFamily {
        match self {
            Self::Auto => accent.matching_gray().family(accent),
            Self::Gray => ColorFamily::Gray,
            Self::Mauve => ColorFamily::Mauve,
            Self::Slate => ColorFamily::Slate,
            Self::Sage => ColorFamily::Sage,
            Self::Olive => ColorFamily::Olive,
            Self::Sand => ColorFamily::Sand,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    #[default]
    Inherit,
    Light,
    Dark,
}

impl Appearance {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
    pub const fn mode(self, inherited: ThemeMode) -> ThemeMode {
        match self {
            Self::Inherit => inherited,
            Self::Light => ThemeMode::Light,
            Self::Dark => ThemeMode::Dark,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelBackground {
    Solid,
    Translucent,
}

impl PanelBackground {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Translucent => "translucent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeRadius {
    None,
    Small,
    Medium,
    Large,
    Full,
}

impl ThemeRadius {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Full => "full",
        }
    }
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::Small,
        Self::Medium,
        Self::Large,
        Self::Full,
    ];

    pub const fn factor(self) -> f64 {
        match self {
            Self::None => 0.,
            Self::Small => 0.75,
            Self::Medium => 1.,
            Self::Large | Self::Full => 1.5,
        }
    }
}

/// A supported percentage, serialized as a JSON number rather than a string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u16", into = "u16")]
pub struct ThemeScaling(u16);

impl ThemeScaling {
    pub const P90: Self = Self(90);
    pub const P95: Self = Self(95);
    pub const P100: Self = Self(100);
    pub const P105: Self = Self(105);
    pub const P110: Self = Self(110);
    pub const ALL: &'static [Self] = &[Self::P90, Self::P95, Self::P100, Self::P105, Self::P110];

    pub const fn percent(self) -> u16 {
        self.0
    }
    pub fn factor(self) -> f64 {
        f64::from(self.0) / 100.
    }
}

impl Default for ThemeScaling {
    fn default() -> Self {
        Self::P100
    }
}

impl From<ThemeScaling> for u16 {
    fn from(scaling: ThemeScaling) -> Self {
        scaling.0
    }
}

impl TryFrom<u16> for ThemeScaling {
    type Error = ThemeError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if matches!(value, 90 | 95 | 100 | 105 | 110) {
            Ok(Self(value))
        } else {
            Err(ThemeError::new(
                "config.scaling",
                "use 90, 95, 100, 105 or 110",
            ))
        }
    }
}

/// `None` is Custom, not an instruction to inherit from another boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {
    pub appearance: Appearance,
    pub accent_color: Option<AccentColor>,
    pub gray_color: Option<GrayColor>,
    pub panel_background: Option<PanelBackground>,
    pub radius: Option<ThemeRadius>,
    pub scaling: ThemeScaling,
}

impl ThemeConfig {
    pub fn fields(self) -> [(&'static str, String); 6] {
        [
            ("appearance", self.appearance.as_str().to_owned()),
            (
                "accent_color",
                self.accent_color
                    .map(AccentColor::as_str)
                    .unwrap_or("null")
                    .to_owned(),
            ),
            (
                "gray_color",
                self.gray_color
                    .map(GrayColor::as_str)
                    .unwrap_or("null")
                    .to_owned(),
            ),
            (
                "panel_background",
                self.panel_background
                    .map(PanelBackground::as_str)
                    .unwrap_or("null")
                    .to_owned(),
            ),
            (
                "radius",
                self.radius
                    .map(ThemeRadius::as_str)
                    .unwrap_or("null")
                    .to_owned(),
            ),
            ("scaling", self.scaling.percent().to_string()),
        ]
    }
    /// Configuration for legacy/imported authored tables: generate no semantic values.
    pub const fn custom() -> Self {
        Self {
            appearance: Appearance::Inherit,
            accent_color: None,
            gray_color: None,
            panel_background: None,
            radius: None,
            scaling: ThemeScaling::P100,
        }
    }

    /// An explicitly new brand theme, rather than the legacy adapter.
    pub const fn standard() -> Self {
        Self {
            appearance: Appearance::Inherit,
            accent_color: Some(AccentColor::Indigo),
            gray_color: Some(GrayColor::Auto),
            panel_background: Some(PanelBackground::Translucent),
            radius: Some(ThemeRadius::Medium),
            scaling: ThemeScaling::P100,
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::custom()
    }
}

/// A runtime boundary input: Keep inherits; Set(None) explicitly selects Custom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConfigChange<T> {
    #[default]
    Keep,
    Set(T),
}

impl<T: Copy> ConfigChange<T> {
    fn apply(self, target: &mut T) {
        if let Self::Set(value) = self {
            *target = value;
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeConfigPatch {
    pub appearance: ConfigChange<Appearance>,
    pub accent_color: ConfigChange<Option<AccentColor>>,
    pub gray_color: ConfigChange<Option<GrayColor>>,
    pub panel_background: ConfigChange<Option<PanelBackground>>,
    pub radius: ConfigChange<Option<ThemeRadius>>,
    pub scaling: ConfigChange<ThemeScaling>,
}

impl ThemeConfigPatch {
    pub fn apply(self, target: &mut ThemeConfig) {
        self.appearance.apply(&mut target.appearance);
        self.accent_color.apply(&mut target.accent_color);
        self.gray_color.apply(&mut target.gray_color);
        self.panel_background.apply(&mut target.panel_background);
        self.radius.apply(&mut target.radius);
        self.scaling.apply(&mut target.scaling);
    }

    pub fn is_empty(self) -> bool {
        self == Self::default()
    }
}

/// A two-mode boundary keeps authored source data; it never patches a scaled snapshot.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThemeBoundaryPatch {
    pub config: ThemeConfigPatch,
    pub light: ThemeValuePatch,
    pub dark: ThemeValuePatch,
}

impl From<ThemeValuePatch> for ThemeBoundaryPatch {
    /// Apply a mode-independent legacy patch without replacing either author table.
    fn from(patch: ThemeValuePatch) -> Self {
        Self {
            light: patch.clone(),
            dark: patch,
            ..Default::default()
        }
    }
}

impl ThemeBoundaryPatch {
    pub fn is_empty(&self) -> bool {
        self.config.is_empty() && self.light.0.is_empty() && self.dark.0.is_empty()
    }

    pub fn values(&self, mode: ThemeMode) -> &ThemeValuePatch {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
        }
    }
}

/// The exhaustive semantic keys each shortcut can generate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeConfigGroup {
    Accent,
    Gray,
    Panel,
    Radius,
}

impl ThemeConfigGroup {
    pub const ALL: &'static [Self] = &[Self::Accent, Self::Gray, Self::Panel, Self::Radius];

    pub const fn field(self) -> &'static str {
        match self {
            Self::Accent => "accent_color",
            Self::Gray => "gray_color",
            Self::Panel => "panel_background",
            Self::Radius => "radius",
        }
    }

    pub const fn tokens(self) -> &'static [&'static str] {
        match self {
            Self::Accent => &[
                "primary",
                "primary-foreground",
                "ring",
                "sidebar-primary",
                "sidebar-primary-foreground",
                "sidebar-ring",
            ],
            Self::Gray => &[
                "foreground",
                "card-foreground",
                "popover-foreground",
                "sidebar-foreground",
                "secondary",
                "secondary-foreground",
                "muted",
                "muted-foreground",
                "border",
                "sidebar-border",
                "input",
                "sidebar-accent",
                "sidebar-accent-foreground",
                "background",
                "sidebar",
            ],
            Self::Panel => &["card", "popover"],
            Self::Radius => &["radius"],
        }
    }
}

/// Selected configuration and the actual palette inherited at this boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectiveThemeConfig {
    pub selected: ThemeConfig,
    pub accent: AccentColor,
    pub gray: ColorFamily,
    /// Auto used this explicit default because its originating brand was Custom.
    pub gray_fallback_accent: Option<AccentColor>,
}

impl EffectiveThemeConfig {
    pub(super) fn root(selected: ThemeConfig) -> Self {
        let accent = selected.accent_color.unwrap_or(AccentColor::Indigo);
        Self {
            selected,
            accent,
            gray: selected
                .gray_color
                .unwrap_or(GrayColor::Slate)
                .family(accent),
            gray_fallback_accent: (selected.gray_color == Some(GrayColor::Auto)
                && selected.accent_color.is_none())
            .then_some(AccentColor::Indigo),
        }
    }

    pub(super) fn patched(mut self, patch: ThemeConfigPatch) -> Self {
        patch.apply(&mut self.selected);
        self.accent = self.selected.accent_color.unwrap_or(AccentColor::Indigo);
        if let ConfigChange::Set(gray) = patch.gray_color {
            self.gray = gray.unwrap_or(GrayColor::Slate).family(self.accent);
            self.gray_fallback_accent = (gray == Some(GrayColor::Auto)
                && self.selected.accent_color.is_none())
            .then_some(AccentColor::Indigo);
        }
        self
    }
}

/// Parsed sRGB scale and supplementary roles shared by CSS and existing GPU props.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedPalette {
    pub family: ColorFamily,
    pub solid: [Rgba; 12],
    pub alpha: [Rgba; 12],
    pub contrast: Rgba,
    pub surface: Rgba,
    pub indicator: Rgba,
    pub track: Rgba,
}

impl ResolvedPalette {
    /// A cached, pinned palette for an explicit component color in one mode.
    pub fn for_family(family: ColorFamily, mode: ThemeMode) -> Result<Arc<Self>, ThemeError> {
        palette(family, mode)
    }
}

pub(super) struct PaletteRoles {
    pub contrast: &'static str,
    pub surface: [&'static str; 2],
    pub indicator: [u8; 2],
    pub dark_track_mix: u8,
}

fn color_mode(mode: ThemeMode) -> ColorMode {
    match mode {
        ThemeMode::Light => ColorMode::Light,
        ThemeMode::Dark => ColorMode::Dark,
    }
}

fn parse_palette(family: ColorFamily, mode: ThemeMode) -> Result<ResolvedPalette, ThemeError> {
    let parse = |value: &str| {
        parse_color(value)
            .map(|c| c.rgba)
            .map_err(|e| ThemeError::new(format!("palette.{}", family.name()), e))
    };
    let mut solid = [Rgba {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    }; 12];
    let mut alpha = solid;
    for (target, is_alpha) in [(&mut solid, false), (&mut alpha, true)] {
        for (target, value) in target.iter_mut().zip(
            family
                .scale(color_mode(mode), ColorGamut::Srgb, is_alpha)
                .steps(),
        ) {
            *target = parse(value)?;
        }
    }
    let roles = super::palette_roles::roles(family);
    let index = usize::from(mode == ThemeMode::Dark);
    let indicator = solid[usize::from(roles.indicator[index] - 1)];
    let track = if mode == ThemeMode::Dark && roles.dark_track_mix != 0 {
        super::color::mix_oklab(solid[7], solid[8], f64::from(roles.dark_track_mix) / 100.)
    } else {
        indicator
    };
    Ok(ResolvedPalette {
        family,
        solid,
        alpha,
        contrast: if roles.contrast == "white" {
            Rgba {
                r: 1.,
                g: 1.,
                b: 1.,
                a: 1.,
            }
        } else {
            parse(roles.contrast)?
        },
        surface: parse(roles.surface[index])?,
        indicator,
        track,
    })
}

/// Palette facts never change; parse them once instead of on every author edit.
pub(super) fn palette(
    family: ColorFamily,
    mode: ThemeMode,
) -> Result<Arc<ResolvedPalette>, ThemeError> {
    type Palettes =
        BTreeMap<(&'static str, &'static str), Result<Arc<ResolvedPalette>, ThemeError>>;
    static PALETTES: LazyLock<Palettes> = LazyLock::new(|| {
        ColorFamily::ALL
            .iter()
            .flat_map(|&family| {
                [ThemeMode::Light, ThemeMode::Dark]
                    .into_iter()
                    .map(move |mode| {
                        (
                            (family.name(), mode.as_str()),
                            parse_palette(family, mode).map(Arc::new),
                        )
                    })
            })
            .collect()
    });
    PALETTES[&(family.name(), mode.as_str())].clone()
}

pub(super) fn generated_values(
    config: EffectiveThemeConfig,
    mode: ThemeMode,
) -> Result<ThemeValuePatch, ThemeError> {
    let mut values = BTreeMap::new();
    if config.selected.accent_color.is_some() {
        let raw = config
            .accent
            .family()
            .scale(color_mode(mode), ColorGamut::Srgb, false)
            .steps();
        let contrast = super::format_color(
            palette(config.accent.family(), mode)?.contrast,
            ColorFormat::Hex,
        );
        for (token, value) in [
            ("primary", raw[8]),
            ("primary-foreground", contrast.as_str()),
            ("ring", raw[7]),
            ("sidebar-primary", raw[8]),
            ("sidebar-primary-foreground", contrast.as_str()),
            ("sidebar-ring", raw[7]),
        ] {
            values.insert(token.into(), value.to_owned());
        }
    }
    if config.selected.gray_color.is_some() {
        let raw = config
            .gray
            .scale(color_mode(mode), ColorGamut::Srgb, false)
            .steps();
        let background = if mode == ThemeMode::Light {
            "#ffffff"
        } else {
            raw[0]
        };
        for (token, value) in [
            ("foreground", raw[11]),
            ("card-foreground", raw[11]),
            ("popover-foreground", raw[11]),
            ("sidebar-foreground", raw[11]),
            ("secondary", raw[2]),
            ("secondary-foreground", raw[11]),
            ("muted", raw[2]),
            ("muted-foreground", raw[10]),
            ("border", raw[6]),
            ("sidebar-border", raw[6]),
            ("input", raw[1]),
            ("sidebar-accent", raw[2]),
            ("sidebar-accent-foreground", raw[11]),
            ("background", background),
            ("sidebar", background),
        ] {
            values.insert(token.into(), value.to_owned());
        }
    }
    if let Some(panel) = config.selected.panel_background {
        let mode = color_mode(mode);
        let panel = match (mode, panel) {
            (ColorMode::Light, PanelBackground::Solid) => "#ffffff",
            (ColorMode::Light, PanelBackground::Translucent) => "rgba(255,255,255,.7)",
            (ColorMode::Dark, PanelBackground::Solid) => {
                config.gray.scale(mode, ColorGamut::Srgb, false).steps()[1]
            }
            (ColorMode::Dark, PanelBackground::Translucent) => {
                config.gray.scale(mode, ColorGamut::Srgb, true).steps()[1]
            }
        };
        for token in ThemeConfigGroup::Panel.tokens() {
            values.insert((*token).into(), panel.to_owned());
        }
    }
    if let Some(radius) = config.selected.radius {
        values.insert("radius".into(), format!("{}px", 6. * radius.factor()));
    }
    Ok(ThemeValuePatch(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_options_and_numeric_scaling_are_strict() {
        assert_eq!(AccentColor::ALL.len(), 26);
        assert_eq!(GrayColor::ALL.len(), 7);
        assert_eq!(ThemeRadius::ALL.len(), 5);
        for scaling in ThemeScaling::ALL {
            let json = serde_json::to_string(scaling).unwrap();
            assert_eq!(json, scaling.percent().to_string());
            assert_eq!(
                serde_json::from_str::<ThemeScaling>(&json).unwrap(),
                *scaling
            );
        }
        for invalid in ["0", "99", "111", "\"100\"", "null", "100.5"] {
            assert!(serde_json::from_str::<ThemeScaling>(invalid).is_err());
        }
        assert!(serde_json::from_str::<AccentColor>("\"mauve\"").is_err());
    }

    #[test]
    fn every_auto_gray_pair_matches_the_vendored_rule() {
        let source = include_str!("vendor/themes/src/helpers/get-matching-gray-color.ts");
        let mut pending = Vec::new();
        let mut checked = 0;
        for line in source.lines().map(str::trim) {
            if let Some(name) = line
                .strip_prefix("case '")
                .and_then(|v| v.strip_suffix("':"))
            {
                pending.push(name);
            }
            if let Some(gray) = line
                .strip_prefix("return '")
                .and_then(|v| v.strip_suffix("';"))
            {
                for accent in pending.drain(..) {
                    let actual = AccentColor::ALL
                        .iter()
                        .find(|a| a.as_str() == accent)
                        .unwrap();
                    assert_eq!(actual.matching_gray().as_str(), gray, "{accent}");
                    checked += 1;
                }
            }
        }
        assert!(pending.is_empty());
        assert_eq!(checked, 26);
    }

    #[test]
    fn keep_and_explicit_custom_are_different_boundary_requests() {
        let mut config = ThemeConfig::standard();
        ThemeConfigPatch::default().apply(&mut config);
        assert_eq!(config, ThemeConfig::standard());
        ThemeConfigPatch {
            accent_color: ConfigChange::Set(None),
            ..Default::default()
        }
        .apply(&mut config);
        assert_eq!(config.accent_color, None);
        assert_eq!(config.gray_color, Some(GrayColor::Auto));
    }

    #[test]
    fn generation_responsibility_does_not_capture_independent_roles() {
        let tokens: std::collections::BTreeSet<_> = ThemeConfigGroup::ALL
            .iter()
            .flat_map(|g| g.tokens().iter().copied())
            .collect();
        assert_eq!(tokens.len(), 24);
        for independent in [
            "accent",
            "accent-foreground",
            "destructive",
            "success",
            "warning",
            "chart-1",
            "font-sans",
            "spacing",
            "shadow-color",
        ] {
            assert!(!tokens.contains(independent));
        }
    }
}
