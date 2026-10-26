//! Controlled theme configuration. The application owns values and commands.

use leptos::prelude::*;
use rustify_ui::{
    AccentColor, Appearance, ConfigChange, EffectiveThemeConfig, GrayColor, Locale,
    PanelBackground, ThemeConfigGroup, ThemeConfigPatch, ThemeMode, ThemeOverrides, ThemeRadius,
    ThemeScaling,
};

use crate::{Button, ButtonSize, ButtonVariant, Select, SelectOption};

/// Explicit overrides belonging to each configuration group's responsibility.
///
/// Array entries follow [`ThemeConfigGroup::ALL`]. Independent tokens are not
/// counted, and clearing a group must leave those tokens in place.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeOverrideCounts {
    pub light: [usize; 4],
    pub dark: [usize; 4],
}

impl ThemeOverrideCounts {
    /// The group's override count in one mode.
    pub fn count(&self, group: ThemeConfigGroup, mode: ThemeMode) -> usize {
        let index = match group {
            ThemeConfigGroup::Accent => 0,
            ThemeConfigGroup::Gray => 1,
            ThemeConfigGroup::Panel => 2,
            ThemeConfigGroup::Radius => 3,
        };
        match mode {
            ThemeMode::Light => self.light[index],
            ThemeMode::Dark => self.dark[index],
        }
    }

    /// The group's combined count across light and dark mode.
    pub fn total(&self, group: ThemeConfigGroup) -> usize {
        self.count(group, ThemeMode::Light) + self.count(group, ThemeMode::Dark)
    }
}

impl From<&ThemeOverrides> for ThemeOverrideCounts {
    fn from(overrides: &ThemeOverrides) -> Self {
        let mut counts = Self::default();
        for (mode, target) in [
            (ThemeMode::Light, &mut counts.light),
            (ThemeMode::Dark, &mut counts.dark),
        ] {
            for (count, group) in target.iter_mut().zip(ThemeConfigGroup::ALL) {
                *count = group
                    .tokens()
                    .iter()
                    .filter(|token| overrides.values(mode).0.contains_key(**token))
                    .count();
            }
        }
        counts
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConfigField {
    Appearance,
    Accent,
    Gray,
    Panel,
    Radius,
    Scaling,
}

impl ConfigField {
    const ALL: [Self; 6] = [
        Self::Appearance,
        Self::Accent,
        Self::Gray,
        Self::Panel,
        Self::Radius,
        Self::Scaling,
    ];

    fn key(self) -> &'static str {
        match self {
            Self::Appearance => "appearance",
            Self::Accent => "accent_color",
            Self::Gray => "gray_color",
            Self::Panel => "panel_background",
            Self::Radius => "radius",
            Self::Scaling => "scaling",
        }
    }

    fn label(self, locale: Locale) -> &'static str {
        match self {
            Self::Appearance => tr(locale, "Appearance", "外观"),
            Self::Accent => tr(locale, "Brand color", "品牌色族"),
            Self::Gray => tr(locale, "Neutral color", "中性色族"),
            Self::Panel => tr(locale, "Panel background", "面板背景"),
            Self::Radius => tr(locale, "Radius", "圆角"),
            Self::Scaling => tr(locale, "Scaling", "缩放"),
        }
    }

    fn group(self) -> Option<ThemeConfigGroup> {
        match self {
            Self::Appearance | Self::Scaling => None,
            Self::Accent => Some(ThemeConfigGroup::Accent),
            Self::Gray => Some(ThemeConfigGroup::Gray),
            Self::Panel => Some(ThemeConfigGroup::Panel),
            Self::Radius => Some(ThemeConfigGroup::Radius),
        }
    }

    fn value(self, config: EffectiveThemeConfig) -> String {
        let config = config.selected;
        match self {
            Self::Appearance => config.appearance.as_str(),
            Self::Accent => config
                .accent_color
                .map(AccentColor::as_str)
                .unwrap_or("custom"),
            Self::Gray => config.gray_color.map(GrayColor::as_str).unwrap_or("custom"),
            Self::Panel => config
                .panel_background
                .map(PanelBackground::as_str)
                .unwrap_or("custom"),
            Self::Radius => config.radius.map(ThemeRadius::as_str).unwrap_or("custom"),
            Self::Scaling => return config.scaling.percent().to_string(),
        }
        .into()
    }

    fn options(self, locale: Locale) -> Vec<SelectOption> {
        let mut choices: Vec<SelectOption> = match self {
            Self::Appearance => {
                return [
                    (Appearance::Inherit, tr(locale, "Inherit", "继承")),
                    (Appearance::Light, tr(locale, "Light", "浅色")),
                    (Appearance::Dark, tr(locale, "Dark", "深色")),
                ]
                .into_iter()
                .map(|(value, label)| SelectOption::new(value.as_str(), label))
                .collect()
            }
            Self::Accent => AccentColor::ALL
                .iter()
                .map(|value| SelectOption::new(value.as_str(), value.as_str()))
                .collect(),
            Self::Gray => GrayColor::ALL
                .iter()
                .map(|value| {
                    SelectOption::new(
                        value.as_str(),
                        if *value == GrayColor::Auto {
                            tr(locale, "Auto", "自动")
                        } else {
                            value.as_str()
                        },
                    )
                })
                .collect(),
            Self::Panel => [
                (PanelBackground::Solid, tr(locale, "Solid", "不透明")),
                (
                    PanelBackground::Translucent,
                    tr(locale, "Translucent", "半透明"),
                ),
            ]
            .into_iter()
            .map(|(value, label)| SelectOption::new(value.as_str(), label))
            .collect(),
            Self::Radius => ThemeRadius::ALL
                .iter()
                .map(|value| {
                    SelectOption::new(
                        value.as_str(),
                        match value {
                            ThemeRadius::None => tr(locale, "None", "无圆角"),
                            ThemeRadius::Small => tr(locale, "Small", "小"),
                            ThemeRadius::Medium => tr(locale, "Medium", "中"),
                            ThemeRadius::Large => tr(locale, "Large", "大"),
                            ThemeRadius::Full => tr(locale, "Full", "全圆角"),
                        },
                    )
                })
                .collect(),
            Self::Scaling => {
                return ThemeScaling::ALL
                    .iter()
                    .map(|value| {
                        SelectOption::new(
                            value.percent().to_string(),
                            format!("{}%", value.percent()),
                        )
                    })
                    .collect()
            }
        };
        choices.insert(
            0,
            SelectOption::new("custom", tr(locale, "Custom", "自定义")),
        );
        choices
    }

    fn request(self, value: &str) -> Option<ThemeConfigPatch> {
        let mut patch = ThemeConfigPatch::default();
        match self {
            Self::Appearance => {
                patch.appearance = ConfigChange::Set(
                    *[Appearance::Inherit, Appearance::Light, Appearance::Dark]
                        .iter()
                        .find(|choice| choice.as_str() == value)?,
                )
            }
            Self::Accent => {
                patch.accent_color = ConfigChange::Set(if value == "custom" {
                    None
                } else {
                    Some(
                        *AccentColor::ALL
                            .iter()
                            .find(|choice| choice.as_str() == value)?,
                    )
                })
            }
            Self::Gray => {
                patch.gray_color = ConfigChange::Set(if value == "custom" {
                    None
                } else {
                    Some(
                        *GrayColor::ALL
                            .iter()
                            .find(|choice| choice.as_str() == value)?,
                    )
                })
            }
            Self::Panel => {
                patch.panel_background = ConfigChange::Set(if value == "custom" {
                    None
                } else {
                    Some(
                        *[PanelBackground::Solid, PanelBackground::Translucent]
                            .iter()
                            .find(|choice| choice.as_str() == value)?,
                    )
                })
            }
            Self::Radius => {
                patch.radius = ConfigChange::Set(if value == "custom" {
                    None
                } else {
                    Some(
                        *ThemeRadius::ALL
                            .iter()
                            .find(|choice| choice.as_str() == value)?,
                    )
                })
            }
            Self::Scaling => {
                patch.scaling = ConfigChange::Set(
                    *ThemeScaling::ALL
                        .iter()
                        .find(|choice| choice.percent().to_string() == value)?,
                )
            }
        }
        Some(patch)
    }

    fn help(self, config: EffectiveThemeConfig, locale: Locale) -> String {
        match self {
            Self::Appearance => tr(locale,
                "Inherit follows the containing theme. This authored choice is separate from preview mode.",
                "继承会跟随外层主题。此处修改作者配置，与预览模式分开。",
            ).into(),
            Self::Scaling => tr(locale,
                "Scales theme sizes once. Explicit CSS lengths and breakpoints stay unchanged.",
                "主题尺寸只缩放一次；显式 CSS 长度与响应式断点不变。",
            ).into(),
            Self::Gray => {
                let mut text = match locale {
                    Locale::English => format!("Resolved neutral: {}.", config.gray.name()),
                    Locale::Chinese => format!("已解析中性色：{}。", config.gray.name()),
                };
                if let Some(accent) = config.gray_fallback_accent {
                    text.push_str(&match locale {
                        Locale::English => format!(" Auto source: {} (originating Custom brand fallback).", accent.as_str()),
                        Locale::Chinese => format!(" 自动匹配来源：{}（来源品牌为自定义时的默认值）。", accent.as_str()),
                    });
                }
                if self.value(config) == "custom" {
                    text.push_str(tr(locale, " Custom keeps authored base values.", " 自定义保留作者基础值。"));
                }
                text
            }
            _ if self.value(config) == "custom" => tr(locale,
                "Custom keeps authored base values. Explicit overrides still take precedence.",
                "自定义保留作者基础值；显式覆盖仍优先生效。",
            ).into(),
            _ => tr(locale,
                "Generates this group's theme values. Explicit overrides still take precedence.",
                "生成本组主题值；显式覆盖仍优先生效。",
            ).into(),
        }
    }
}

fn tr(locale: Locale, english: &'static str, chinese: &'static str) -> &'static str {
    match locale {
        Locale::English => english,
        Locale::Chinese => chinese,
    }
}

fn override_summary(
    counts: ThemeOverrideCounts,
    group: ThemeConfigGroup,
    locale: Locale,
) -> String {
    format!(
        "{} · {} {} · {} {}",
        if counts.total(group) == 0 {
            tr(locale, "No overrides", "无覆盖")
        } else {
            tr(locale, "Overridden", "已覆盖")
        },
        tr(locale, "Light", "浅色"),
        counts.count(group, ThemeMode::Light),
        tr(locale, "Dark", "深色"),
        counts.count(group, ThemeMode::Dark),
    )
}

/// Edits the application's effective configuration through controlled requests.
///
/// `on_reset(group)` requests removal of that group's overrides in **both**
/// modes. The application performs the command, validation and history update.
/// A rejected request leaves each Select showing the actual `config` value.
#[component]
pub fn ThemePanel(
    #[prop(into)] config: Signal<EffectiveThemeConfig>,
    #[prop(into)] overrides: Signal<ThemeOverrideCounts>,
    on_change: impl Fn(ThemeConfigPatch) + Send + Sync + 'static,
    on_reset: impl Fn(ThemeConfigGroup) + Send + Sync + 'static,
    #[prop(optional, into)] locale: Signal<Locale>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let on_change = Callback::new(on_change);
    let on_reset = Callback::new(on_reset);
    let panel_id = crate::id::next("theme-panel");
    let title_id = format!("{panel_id}-title");
    let labelled_by = title_id.clone();
    let reset_help_id = format!("{panel_id}-reset-help");
    let fields_reset_help = reset_help_id.clone();
    let fields_test_id = test_id.clone();
    let computed =
        crate::macros::reactive_merge_with(move || vec!["space-y-5 text-sm".into(), class.get()]);
    view! {
        <section class=computed data-name="ThemePanel" data-testid=test_id lang=move || locale.get().tag() aria-labelledby=labelled_by>
            <div class="space-y-2">
                <h2 id=title_id class="font-semibold text-foreground">{move || tr(locale.get(), "Theme configuration", "主题基础配置")}</h2>
                <p class="text-xs leading-relaxed text-muted-foreground">{move || tr(locale.get(),
                    "Base choices generate theme values. Overrides remain in place when configuration changes.",
                    "基础配置生成主题值。修改配置会保留已有覆盖。",
                )}</p>
                <p id=reset_help_id class="text-xs leading-relaxed text-muted-foreground">{move || tr(locale.get(),
                    "A group reset clears only that group's overrides in both modes. Other groups and independent adjustments stay unchanged.",
                    "使用基础或配置值会清除该组两模式覆盖；其他组和独立精调保持不变。",
                )}</p>
            </div>
            {ConfigField::ALL.into_iter().map(move |field| view! {
                <ThemeConfigControl field=field config=config overrides=overrides on_change=on_change on_reset=on_reset locale=locale reset_help_id=fields_reset_help.clone() test_prefix=fields_test_id.clone()/>
            }).collect_view()}
        </section>
    }
}

#[component]
fn ThemeConfigControl(
    field: ConfigField,
    config: Signal<EffectiveThemeConfig>,
    overrides: Signal<ThemeOverrideCounts>,
    on_change: Callback<ThemeConfigPatch>,
    on_reset: Callback<ThemeConfigGroup>,
    locale: Signal<Locale>,
    reset_help_id: String,
    test_prefix: String,
) -> impl IntoView {
    let id = crate::id::next("theme-config");
    let help_id = format!("{id}-help");
    let status_id = format!("{id}-overrides");
    let described_by = if field.group().is_some() {
        format!("{help_id} {status_id}")
    } else {
        help_id.clone()
    };
    let test_id = format!(
        "{}-{}",
        if test_prefix.is_empty() {
            "theme-panel"
        } else {
            &test_prefix
        },
        field.key()
    );
    let reset_test_id = format!("{test_id}-reset");
    let open = RwSignal::new(false);
    #[cfg(target_arch = "wasm32")]
    Effect::watch(move || locale.get(), move |_, _, _| open.set(false), false);
    let options = Memo::new(move |_| field.options(locale.get()));
    view! {
        <div class="space-y-2" data-theme-config=field.key()>
            <label for=id.clone() class="block font-medium text-foreground">{move || field.label(locale.get())}</label>
            <Select id=id value=Signal::derive(move || field.value(config.get())) options=Signal::derive(move || options.get())
                open=open on_open_change=move |value| open.set(value)
                on_change=move |value| {if let Some(patch) = field.request(&value) {on_change.run(patch);}}
                described_by=described_by test_id=test_id/>
            <p id=help_id class="text-xs leading-relaxed text-muted-foreground" data-name="ThemeConfigHelp">{move || field.help(config.get(), locale.get())}</p>
            {field.group().map(move |group| view! {
                <div class="space-y-2">
                    <p id=status_id class="text-xs text-muted-foreground" role="status" data-name="ThemeOverrideCount"
                        data-overridden=move || (overrides.get().total(group) > 0).to_string()>
                        {move || override_summary(overrides.get(), group, locale.get())}
                    </p>
                    <Button size=ButtonSize::One variant=ButtonVariant::Outline
                        disabled=Signal::derive(move || overrides.get().total(group) == 0)
                        on_click=move || on_reset.run(group)
                        class="h-auto min-h-8 whitespace-normal py-1.5 text-left text-xs"
                        test_id=reset_test_id attr:aria-describedby=reset_help_id>
                        {move || if field.value(config.get()) == "custom" {
                            tr(locale.get(), "Use base values · both modes", "使用基础值 · 两模式")
                        } else {
                            tr(locale.get(), "Use configuration values · both modes", "使用配置值 · 两模式")
                        }}
                    </Button>
                </div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_follow_group_responsibility_without_capturing_independent_tokens() {
        let mut overrides = ThemeOverrides::default();
        for (mode, tokens) in [
            (
                ThemeMode::Light,
                vec![
                    "primary",
                    "ring",
                    "foreground",
                    "card",
                    "radius",
                    "accent",
                    "font-size",
                ],
            ),
            (
                ThemeMode::Dark,
                vec!["primary", "card", "popover", "shadow-color"],
            ),
        ] {
            for token in tokens {
                overrides
                    .values_mut(mode)
                    .0
                    .insert(token.into(), "test".into());
            }
        }
        let counts = ThemeOverrideCounts::from(&overrides);
        assert_eq!(counts.light, [2, 1, 1, 1]);
        assert_eq!(counts.dark, [1, 0, 2, 0]);
        for (index, group) in ThemeConfigGroup::ALL.iter().copied().enumerate() {
            assert_eq!(counts.count(group, ThemeMode::Light), counts.light[index]);
            assert_eq!(counts.count(group, ThemeMode::Dark), counts.dark[index]);
            assert_eq!(
                counts.total(group),
                counts.light[index] + counts.dark[index]
            );
        }
    }

    #[test]
    fn each_select_option_requests_exactly_one_set_and_all_other_fields_keep() {
        for (field, count) in ConfigField::ALL.into_iter().zip([3, 27, 8, 3, 6, 5]) {
            let options = field.options(Locale::English);
            assert_eq!(options.len(), count, "{field:?}");
            for option in options {
                let patch = field.request(&option.value).unwrap();
                let changed = [
                    !matches!(patch.appearance, ConfigChange::Keep),
                    !matches!(patch.accent_color, ConfigChange::Keep),
                    !matches!(patch.gray_color, ConfigChange::Keep),
                    !matches!(patch.panel_background, ConfigChange::Keep),
                    !matches!(patch.radius, ConfigChange::Keep),
                    !matches!(patch.scaling, ConfigChange::Keep),
                ];
                assert_eq!(
                    changed.into_iter().filter(|changed| *changed).count(),
                    1,
                    "{field:?}: {patch:?}"
                );
                let mut config = effective();
                patch.apply(&mut config.selected);
                assert_eq!(field.value(config), option.value, "{field:?}");
            }
            assert!(field.request("invalid-choice").is_none());
        }
    }

    #[test]
    fn custom_requests_are_explicit_nulls_instead_of_keep() {
        assert_eq!(
            ConfigField::Accent.request("custom").unwrap().accent_color,
            ConfigChange::Set(None)
        );
        assert_eq!(
            ConfigField::Gray.request("custom").unwrap().gray_color,
            ConfigChange::Set(None)
        );
        assert_eq!(
            ConfigField::Panel
                .request("custom")
                .unwrap()
                .panel_background,
            ConfigChange::Set(None)
        );
        assert_eq!(
            ConfigField::Radius.request("custom").unwrap().radius,
            ConfigChange::Set(None)
        );
    }

    fn effective() -> EffectiveThemeConfig {
        EffectiveThemeConfig {
            selected: rustify_ui::ThemeConfig::standard(),
            accent: AccentColor::Indigo,
            gray: rustify_ui::ColorFamily::Slate,
            gray_fallback_accent: None,
        }
    }

    #[test]
    fn gray_help_uses_the_effective_inherited_palette_and_its_original_auto_source() {
        let config = EffectiveThemeConfig {
            selected: rustify_ui::ThemeConfig {
                accent_color: Some(AccentColor::Tomato),
                gray_color: Some(GrayColor::Auto),
                ..rustify_ui::ThemeConfig::standard()
            },
            accent: AccentColor::Tomato,
            gray_fallback_accent: Some(AccentColor::Indigo),
            ..effective()
        };
        let help = ConfigField::Gray.help(config, Locale::English);
        assert!(help.contains("Resolved neutral: slate."), "{help}");
        assert!(help.contains("Auto source: indigo"), "{help}");
        assert!(!help.contains("mauve"), "{help}");
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn ssr_exposes_six_labeled_controls_and_both_mode_override_actions() {
        let owner = Owner::new();
        let html = owner.with(|| {
            view! {
                <ThemePanel config=effective()
                    overrides=ThemeOverrideCounts { light: [2, 1, 0, 0], dark: [1, 0, 1, 0] }
                    on_change=|_| {} on_reset=|_| {} test_id="panel"/>
            }
            .to_html()
        });
        drop(owner);
        assert_eq!(html.matches("role=\"combobox\"").count(), 6, "{html}");
        assert_eq!(
            html.matches("data-name=\"ThemeOverrideCount\"").count(),
            4,
            "{html}"
        );
        assert_eq!(
            html.matches("Use configuration values · both modes")
                .count(),
            4,
            "{html}"
        );
        assert!(html.contains("Overridden · Light 2 · Dark 1"), "{html}");
        assert!(html.contains("No overrides · Light 0 · Dark 0"), "{html}");
        assert!(html.contains("Resolved neutral: slate."), "{html}");
        assert!(
            html.contains("data-testid=\"panel-accent_color-reset\""),
            "{html}"
        );
        assert!(!html.contains(" style="), "{html}");
        assert!(!html.contains("role=\"listbox\""), "{html}");
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn ssr_translates_custom_values_and_auto_gray_origin_without_recomputing_it() {
        let owner = Owner::new();
        let html = owner.with(|| {
            let config = EffectiveThemeConfig {
                selected: rustify_ui::ThemeConfig {
                    gray_color: Some(GrayColor::Auto),
                    ..rustify_ui::ThemeConfig::custom()
                },
                gray_fallback_accent: Some(AccentColor::Indigo),
                ..effective()
            };
            view! {
                <ThemePanel config=config overrides=ThemeOverrideCounts::default()
                    on_change=|_| {} on_reset=|_| {} locale=Locale::Chinese/>
            }
            .to_html()
        });
        drop(owner);
        assert!(html.contains("lang=\"zh-CN\""), "{html}");
        assert!(html.contains("主题基础配置"), "{html}");
        assert!(html.contains("已解析中性色：slate。"), "{html}");
        assert!(
            html.contains("自动匹配来源：indigo（来源品牌为自定义时的默认值）。"),
            "{html}"
        );
        assert!(html.contains("使用基础值 · 两模式"), "{html}");
        assert!(html.contains("使用配置值 · 两模式"), "{html}");
        assert!(html.contains("自定义"), "{html}");
    }
}
