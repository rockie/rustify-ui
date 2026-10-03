use rustify_ui::theme::{
    contrast_ratio, parse_font_stack, parse_length, LengthRule, ResolvedTheme,
};

const LENGTH_FIELDS: [(&str, LengthRule); 8] = [
    ("font-size", LengthRule::Positive),
    ("radius", LengthRule::NonNegative),
    ("spacing", LengthRule::Positive),
    ("layout-gap", LengthRule::NonNegative),
    ("shadow-blur", LengthRule::NonNegative),
    ("shadow-spread", LengthRule::Signed),
    ("shadow-offset-x", LengthRule::Signed),
    ("shadow-offset-y", LengthRule::Signed),
];

fn valid_author_value(token: &str, value: &str, theme: &ResolvedTheme) -> bool {
    if ["font-sans", "font-serif", "font-mono"].contains(&token) {
        return parse_font_stack(value).is_ok();
    }
    if value.trim().ends_with('.') {
        return false;
    }
    if let Some((_, rule)) = LENGTH_FIELDS.iter().find(|(name, _)| *name == token) {
        return parse_length(value, theme.rem_px, *rule).is_ok();
    }
    match token {
        "letter-spacing" => {
            let value = value.trim();
            if value == "normal" || value == "0" {
                return true;
            }
            let em = if let Some(raw) = value.strip_suffix("em") {
                raw.parse::<f64>().ok()
            } else {
                parse_length(value, theme.rem_px, LengthRule::Signed)
                    .ok()
                    .map(|px| px / theme.font_size_px)
            };
            em.is_some_and(|em| em.is_finite() && (-0.5..=0.5).contains(&em))
        }
        "shadow-opacity" => value
            .parse::<f64>()
            .ok()
            .is_some_and(|number| number.is_finite() && (0.0..=1.0).contains(&number)),
        "reduce-motion" => matches!(value, "true" | "false"),
        _ => false,
    }
}

const CONTRAST_PAIRS: [(&str, &str); 15] = [
    ("foreground", "background"),
    ("card-foreground", "card"),
    ("popover-foreground", "popover"),
    ("primary-foreground", "primary"),
    ("secondary-foreground", "secondary"),
    ("muted-foreground", "muted"),
    ("accent-foreground", "accent"),
    ("destructive-foreground", "destructive"),
    ("sidebar-foreground", "sidebar"),
    ("sidebar-primary-foreground", "sidebar-primary"),
    ("sidebar-accent-foreground", "sidebar-accent"),
    ("foreground", "input"),
    ("muted-foreground", "input"),
    ("foreground", "success"),
    ("foreground", "warning"),
];

fn pair_contrast(theme: &ResolvedTheme, foreground: &str, background: &str) -> Option<f64> {
    // A token snapshot cannot identify the actual underlay of every button or
    // floating panel. Opaque surfaces need no such assumption; text alpha is
    // still composited by the shared WCAG helper.
    contrast_ratio(
        *theme.colors.get(foreground)?,
        *theme.colors.get(background)?,
        None,
    )
}

#[cfg(target_arch = "wasm32")]
pub use ui::{ThemeControls, ThemeDiagnostics};

#[cfg(target_arch = "wasm32")]
mod ui {
    use super::{pair_contrast, valid_author_value, CONTRAST_PAIRS};
    use crate::state::{HslShift, TokenScope};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use rustify_components::{
        Button, ButtonSize, ButtonVariant, Select, SelectOption, Slider, Switch,
    };
    use rustify_ui::{
        theme::{
            parse_color, parse_font_stack, ResolvedTheme, ThemeValues, COLOR_TOKENS, FONT_FACES,
        },
        Locale,
    };
    use std::sync::Arc;

    const FIELD: &str = "w-full min-w-0 rounded-md border border-border bg-input text-foreground px-3 py-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring aria-invalid:border-destructive";

    fn tr(locale: Locale, en: &'static str, zh: &'static str) -> &'static str {
        if locale == Locale::Chinese {
            zh
        } else {
            en
        }
    }

    fn label(token: &str, locale: Locale) -> &'static str {
        let (en, zh) = match token {
            "font-sans" => ("Sans font stack", "无衬线字体栈"),
            "font-serif" => ("Serif font stack", "衬线字体栈"),
            "font-mono" => ("Mono font stack", "等宽字体栈"),
            "font-size" => ("Base font size", "基础字号"),
            "letter-spacing" => ("Letter spacing", "字距"),
            "radius" => ("Corner radius", "圆角"),
            "spacing" => ("Spacing unit", "间距单位"),
            "layout-gap" => ("Layout gap", "布局间距"),
            "shadow-opacity" => ("Shadow opacity", "阴影不透明度"),
            "shadow-blur" => ("Shadow blur", "阴影模糊"),
            "shadow-spread" => ("Shadow spread", "阴影扩散"),
            "shadow-offset-x" => ("Horizontal shadow offset", "阴影水平偏移"),
            "shadow-offset-y" => ("Vertical shadow offset", "阴影垂直偏移"),
            "reduce-motion" => ("Reduce motion", "减少动效"),
            _ => ("Theme value", "主题值"),
        };
        tr(locale, en, zh)
    }

    fn hint(token: &str, locale: Locale) -> &'static str {
        let (en, zh) = match token {
            "font-sans" | "font-serif" | "font-mono" => ("Comma-separated families. Unknown names are preserved with a visible preview fallback.", "用逗号分隔字体名。未知字体名会保留，预览回退会明确提示。"),
            "letter-spacing" => ("em, px, rem or normal; resolved range −0.5…0.5em.", "支持 em、px、rem 或 normal；解析后范围为 −0.5…0.5em。"),
            "shadow-opacity" => ("A number from 0 to 1.", "0 到 1 之间的数值。"),
            "shadow-spread" | "shadow-offset-x" | "shadow-offset-y" => ("Signed px or rem; unitless zero is allowed.", "支持正负 px 或 rem，也可填写无单位的 0。"),
            "spacing" | "font-size" => ("Positive px or rem.", "大于 0 的 px 或 rem。"),
            _ => ("Nonnegative px or rem; unitless zero is allowed.", "非负 px 或 rem，也可填写无单位的 0。"),
        };
        tr(locale, en, zh)
    }

    struct Callbacks {
        edit: Arc<dyn Fn(String, String) + Send + Sync>,
        begin: Arc<dyn Fn() + Send + Sync>,
        hsl: Arc<dyn Fn(HslShift, TokenScope) + Send + Sync>,
        commit: Arc<dyn Fn() + Send + Sync>,
        cancel: Arc<dyn Fn() + Send + Sync>,
    }

    #[component]
    pub fn ThemeControls(
        #[prop(into)] values: Signal<ThemeValues>,
        #[prop(into)] baseline: Signal<ThemeValues>,
        #[prop(into)] locale: Signal<Locale>,
        #[prop(into)] resolved: Signal<Arc<ResolvedTheme>>,
        on_edit: impl Fn(String, String) + Clone + Send + Sync + 'static,
        on_begin: impl Fn() + Clone + Send + Sync + 'static,
        on_hsl: impl Fn(HslShift, TokenScope) + Clone + Send + Sync + 'static,
        on_commit: impl Fn() + Clone + Send + Sync + 'static,
        on_cancel: impl Fn() + Clone + Send + Sync + 'static,
        #[prop(optional, into)] common_differences: Signal<Vec<String>>,
        #[prop(default = Signal::derive(|| true), into)] active: Signal<bool>,
    ) -> impl IntoView {
        let callbacks = StoredValue::new(Callbacks {
            edit: Arc::new(on_edit),
            begin: Arc::new(on_begin),
            hsl: Arc::new(on_hsl),
            commit: Arc::new(on_commit),
            cancel: Arc::new(on_cancel),
        });
        view! {
            <div class="theme-controls space-y-6 text-sm" data-testid="theme-controls">
                <p class="text-muted-foreground" data-testid="common-fields-note">{move || tr(locale.get(), "All 14 shared fields apply to light and dark together. Existing differences remain until you edit that field.", "14 个公共字段的改动会同步浅深色。原有差异会保留到该字段被编辑。")}</p>
                <Show when=move || !common_differences.get().is_empty()>
                    <p class="text-muted-foreground break-words" data-testid="common-differences">{move || format!("{} {}",tr(locale.get(), "Different authored values:", "浅深色原始值不同："),common_differences.get().join(", "))}</p>
                </Show>
                <section class="space-y-4" data-theme-group="typography" data-testid="typography-controls">
                    <h2 class="font-semibold">{move || tr(locale.get(), "Typography", "排版")}</h2>
                    {["font-sans","font-serif","font-mono"].into_iter().map(move |token| view! {
                        <div class="space-y-2"><FontChoice token=token values=values locale=locale callbacks=callbacks />
                            <AuthorField token=token values=values baseline=baseline resolved=resolved locale=locale callbacks=callbacks />
                        </div>
                    }).collect_view()}
                    {["font-size","letter-spacing"].into_iter().map(move |token| view! {<AuthorField token=token values=values baseline=baseline resolved=resolved locale=locale callbacks=callbacks />}).collect_view()}
                </section>
                <section class="space-y-4" data-theme-group="other" data-testid="geometry-controls">
                    <h2 class="font-semibold">{move || tr(locale.get(), "Geometry", "尺寸")}</h2>
                    {["radius","spacing","layout-gap"].into_iter().map(move |token| view! {<AuthorField token=token values=values baseline=baseline resolved=resolved locale=locale callbacks=callbacks />}).collect_view()}
                </section>
                <section class="space-y-4" data-theme-group="other" data-testid="shadow-controls">
                    <h2 class="font-semibold">{move || tr(locale.get(), "Shadow", "阴影")}</h2>
                    <p class="text-muted-foreground">{move || tr(locale.get(), "Edit shadow-color in Colors. These values control the shared shadow layers.", "请在颜色面板编辑 shadow-color。以下数值控制公共阴影层。")}</p>
                    {["shadow-opacity","shadow-blur","shadow-spread","shadow-offset-x","shadow-offset-y"].into_iter().map(move |token| view! {<AuthorField token=token values=values baseline=baseline resolved=resolved locale=locale callbacks=callbacks />}).collect_view()}
                </section>
                <section class="space-y-2" data-theme-group="other" data-testid="motion-controls">
                    <div class="flex items-center justify-between gap-3"><label class="flex items-center gap-2">
                        <Switch checked=Signal::derive(move || values.get().get("reduce-motion")==Ok("true")) on_change=move |value| callbacks.with_value(|callbacks|(callbacks.edit)("reduce-motion".into(),value.to_string())) test_id="edit-reduce-motion" />
                        <span>{move || label("reduce-motion",locale.get())}</span></label>
                        <ResetField token="reduce-motion" values=values baseline=baseline locale=locale callbacks=callbacks />
                    </div>
                    <p class="text-muted-foreground">{move || tr(locale.get(), "Host reduced-motion preferences can also disable transitions.", "宿主的减少动效偏好也可能关闭过渡。")}</p>
                </section>
                <BatchHsl locale=locale callbacks=callbacks active=active />
            </div>
        }
    }

    #[component]
    fn ResetField(
        token: &'static str,
        values: Signal<ThemeValues>,
        baseline: Signal<ThemeValues>,
        locale: Signal<Locale>,
        callbacks: StoredValue<Callbacks>,
    ) -> impl IntoView {
        view! {
            <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost
                disabled=Signal::derive(move || values.get().get(token).ok()==baseline.get().get(token).ok())
                on_click=move || {if let Ok(value)=baseline.get_untracked().get(token) {callbacks.with_value(|callbacks|(callbacks.edit)(token.into(),value.to_owned()));}}
                test_id=format!("reset-{token}")>{move || tr(locale.get(), "Reset", "重置")}<span class="sr-only">{format!(" {token}")}</span></Button>
        }
    }

    #[component]
    fn AuthorField(
        token: &'static str,
        values: Signal<ThemeValues>,
        baseline: Signal<ThemeValues>,
        resolved: Signal<Arc<ResolvedTheme>>,
        locale: Signal<Locale>,
        callbacks: StoredValue<Callbacks>,
    ) -> impl IntoView {
        let draft = RwSignal::new(
            values
                .get_untracked()
                .get(token)
                .unwrap_or_default()
                .to_owned(),
        );
        let focused = StoredValue::new(false);
        let composing = StoredValue::new(false);
        let dirty = RwSignal::new(false);
        let rejected = RwSignal::new(false);
        let id = rustify_components::id::next("theme-value");
        let hint_id = format!("{id}-hint");
        let error_id = format!("{id}-error");
        let invalid = Memo::new(move |_| {
            rejected.get()
                || (dirty.get()
                    && !composing.get_value()
                    && !valid_author_value(token, &draft.get(), &resolved.get()))
        });
        Effect::new(move || {
            let value = values.get().get(token).unwrap_or_default().to_owned();
            if !composing.get_value() && (!focused.get_value() || !dirty.get_untracked()) {
                draft.set(value);
            }
        });
        let restore = move || {
            draft.set(
                values
                    .get_untracked()
                    .get(token)
                    .unwrap_or_default()
                    .to_owned(),
            );
            dirty.set(false);
            rejected.set(false);
        };
        let submit = move || {
            if composing.get_value() || !dirty.get_untracked() {
                return;
            }
            let value = draft.get_untracked();
            if valid_author_value(token, &value, &resolved.get_untracked()) {
                callbacks.with_value(|callbacks| (callbacks.edit)(token.into(), value));
                dirty.set(false);
                rejected.set(false);
                draft.set(
                    values
                        .get_untracked()
                        .get(token)
                        .unwrap_or_default()
                        .to_owned(),
                );
            } else {
                rejected.set(true);
                dirty.set(false);
                draft.set(
                    values
                        .get_untracked()
                        .get(token)
                        .unwrap_or_default()
                        .to_owned(),
                );
            }
        };
        view! {
            <div class="space-y-2" data-testid=format!("field-{token}")>
                <div class="flex items-center justify-between gap-2"><label for=id.clone()>{move || label(token,locale.get())}</label><ResetField token=token values=values baseline=baseline locale=locale callbacks=callbacks /></div>
                <input type="text" id=id class=FIELD spellcheck="false" autocomplete="off" data-testid=format!("edit-{token}")
                    aria-describedby=format!("{hint_id} {error_id}") aria-invalid=move || if invalid.get() {"true"} else {"false"} prop:value=move || draft.get()
                    on:focus=move |_| focused.set_value(true)
                    on:input:target=move |event| {if !composing.get_value() {draft.set(event.target().value());dirty.set(true);rejected.set(false);}}
                    on:compositionstart=move |_| composing.set_value(true)
                    on:compositionend:target=move |event| {composing.set_value(false);draft.set(event.target().value());dirty.set(true);rejected.set(false);}
                    on:blur=move |_| {submit();focused.set_value(false);}
                    on:keydown=move |event| {if event.is_composing() || composing.get_value() {return;} match event.key().as_str() {"Enter"=>{event.prevent_default();submit();},"Escape"=>{event.prevent_default();event.stop_propagation();restore();},_=>()}} />
                <p id=hint_id class="text-xs text-muted-foreground">{move || hint(token,locale.get())}</p>
                <p id=error_id class="text-xs text-destructive" role="status" data-testid=format!("error-{token}")>{move || if invalid.get() {tr(locale.get(), "Complete a valid value. The effective theme is unchanged.", "请填写完整的有效值。当前主题未改变。")} else {""}}</p>
            </div>
        }
    }

    #[component]
    fn FontChoice(
        token: &'static str,
        values: Signal<ThemeValues>,
        locale: Signal<Locale>,
        callbacks: StoredValue<Callbacks>,
    ) -> impl IntoView {
        let open = RwSignal::new(false);
        let selected = Signal::derive(move || {
            let author = values.get();
            let first = author
                .get(token)
                .ok()
                .and_then(|value| parse_font_stack(value).ok())
                .and_then(|families| families.into_iter().next())
                .unwrap_or_default();
            FONT_FACES
                .iter()
                .find(|face| face.family.eq_ignore_ascii_case(&first))
                .map(|face| face.family.to_owned())
                .unwrap_or_else(|| "custom".into())
        });
        view! {
            <label class="block space-y-2"><span class="text-xs text-muted-foreground">{move || tr(locale.get(), "Choose a bundled font", "选择已打包字体")}</span>
                <Select value=selected open=open on_open_change=move |value| open.set(value)
                    options=Signal::derive(move || {let mut choices=FONT_FACES.iter().map(|face|SelectOption::new(face.family,face.family)).collect::<Vec<_>>();if selected.get()=="custom" {choices.insert(0,SelectOption::new("custom",tr(locale.get(), "Custom author stack", "自定义作者字体栈")).disabled());}choices})
                    on_change=move |family| {if FONT_FACES.iter().any(|face|face.family==family) {let generic=match token {"font-serif"=>"serif","font-mono"=>"monospace",_=>"sans-serif"};callbacks.with_value(|callbacks|(callbacks.edit)(token.into(),format!("\"{family}\", {generic}")));}}
                    test_id=format!("choose-{token}") />
            </label>
        }
    }

    #[derive(Clone, Copy)]
    enum Axis {
        Hue,
        Saturation,
        Lightness,
    }

    impl Axis {
        fn name(self) -> &'static str {
            match self {
                Self::Hue => "hue",
                Self::Saturation => "saturation",
                Self::Lightness => "lightness",
            }
        }
        fn bounds(self) -> (f64, f64, f64) {
            match self {
                Self::Hue => (-180., 180., 1.),
                Self::Saturation => (0., 2., 0.01),
                Self::Lightness => (0.2, 2., 0.01),
            }
        }
        fn get(self, shift: HslShift) -> f64 {
            match self {
                Self::Hue => shift.hue_degrees,
                Self::Saturation => shift.saturation_scale,
                Self::Lightness => shift.lightness_scale,
            }
        }
        fn set(self, shift: &mut HslShift, value: f64) {
            match self {
                Self::Hue => shift.hue_degrees = value,
                Self::Saturation => shift.saturation_scale = value,
                Self::Lightness => shift.lightness_scale = value,
            }
        }
        fn label(self, locale: Locale) -> &'static str {
            match self {
                Self::Hue => tr(locale, "Hue shift (°)", "色相偏移（°）"),
                Self::Saturation => tr(locale, "Saturation (×)", "饱和度（×）"),
                Self::Lightness => tr(locale, "Lightness (×)", "亮度（×）"),
            }
        }
    }

    #[derive(Clone, Copy)]
    struct Batch {
        shift: RwSignal<HslShift>,
        scope: RwSignal<TokenScope>,
        started: RwSignal<bool>,
        generation: RwSignal<u64>,
        callbacks: StoredValue<Callbacks>,
    }

    impl Batch {
        fn update(self, shift: HslShift) {
            if self.shift.get_untracked() == shift {
                return;
            }
            if !self.started.get_untracked() {
                self.callbacks.with_value(|callbacks| (callbacks.begin)());
                self.started.set(true);
            }
            self.shift.set(shift);
            self.callbacks
                .with_value(|callbacks| (callbacks.hsl)(shift, self.scope.get_untracked()));
        }
        fn finish(self, commit: bool) {
            if self.started.get_untracked() {
                self.callbacks.with_value(|callbacks| {
                    if commit {
                        (callbacks.commit)()
                    } else {
                        (callbacks.cancel)()
                    }
                });
                self.started.set(false);
            }
            self.shift.set(HslShift::default());
            self.generation.update(|value| *value += 1);
        }
        fn axis(self, axis: Axis, value: f64) {
            let mut shift = self.shift.get_untracked();
            axis.set(&mut shift, value);
            self.update(shift);
        }
    }

    const PRESETS: [(&str, &str, HslShift); 15] = [
        (
            "Hue −120°",
            "色相 −120°",
            HslShift {
                hue_degrees: -120.,
                saturation_scale: 1.,
                lightness_scale: 1.,
            },
        ),
        (
            "Hue −60°",
            "色相 −60°",
            HslShift {
                hue_degrees: -60.,
                saturation_scale: 1.,
                lightness_scale: 1.,
            },
        ),
        (
            "Hue +60°",
            "色相 +60°",
            HslShift {
                hue_degrees: 60.,
                saturation_scale: 1.,
                lightness_scale: 1.,
            },
        ),
        (
            "Hue +120°",
            "色相 +120°",
            HslShift {
                hue_degrees: 120.,
                saturation_scale: 1.,
                lightness_scale: 1.,
            },
        ),
        (
            "Hue invert",
            "色相反转",
            HslShift {
                hue_degrees: 180.,
                saturation_scale: 1.,
                lightness_scale: 1.,
            },
        ),
        (
            "Grayscale",
            "灰度",
            HslShift {
                hue_degrees: 0.,
                saturation_scale: 0.,
                lightness_scale: 1.,
            },
        ),
        (
            "Muted",
            "柔和",
            HslShift {
                hue_degrees: 0.,
                saturation_scale: 0.6,
                lightness_scale: 1.,
            },
        ),
        (
            "Vibrant",
            "鲜艳",
            HslShift {
                hue_degrees: 0.,
                saturation_scale: 1.4,
                lightness_scale: 1.,
            },
        ),
        (
            "Dimmer",
            "更暗",
            HslShift {
                hue_degrees: 0.,
                saturation_scale: 1.,
                lightness_scale: 0.8,
            },
        ),
        (
            "Brighter",
            "更亮",
            HslShift {
                hue_degrees: 0.,
                saturation_scale: 1.,
                lightness_scale: 1.2,
            },
        ),
        (
            "H +30° · S ×0.5 · L ×0.95",
            "H +30° · S ×0.5 · L ×0.95",
            HslShift {
                hue_degrees: 30.,
                saturation_scale: 0.5,
                lightness_scale: 0.95,
            },
        ),
        (
            "H −20° · S ×1.2 · L ×1.05",
            "H −20° · S ×1.2 · L ×1.05",
            HslShift {
                hue_degrees: -20.,
                saturation_scale: 1.2,
                lightness_scale: 1.05,
            },
        ),
        (
            "H +20° · S ×0.7 · L ×0.95",
            "H +20° · S ×0.7 · L ×0.95",
            HslShift {
                hue_degrees: 20.,
                saturation_scale: 0.7,
                lightness_scale: 0.95,
            },
        ),
        (
            "H −10° · S ×0.75 · L ×1.1",
            "H −10° · S ×0.75 · L ×1.1",
            HslShift {
                hue_degrees: -10.,
                saturation_scale: 0.75,
                lightness_scale: 1.1,
            },
        ),
        (
            "H +60° · S ×1.5 · L ×1.1",
            "H +60° · S ×1.5 · L ×1.1",
            HslShift {
                hue_degrees: 60.,
                saturation_scale: 1.5,
                lightness_scale: 1.1,
            },
        ),
    ];

    #[component]
    fn BatchHsl(
        locale: Signal<Locale>,
        callbacks: StoredValue<Callbacks>,
        active: Signal<bool>,
    ) -> impl IntoView {
        let batch = Batch {
            shift: RwSignal::new(HslShift::default()),
            scope: RwSignal::new(TokenScope::Both),
            started: RwSignal::new(false),
            generation: RwSignal::new(0),
            callbacks,
        };
        let scope_open = RwSignal::new(false);
        let node = NodeRef::<leptos::html::Section>::new();
        let cleanup_cancel = callbacks.with_value(|callbacks| Arc::clone(&callbacks.cancel));
        on_cleanup(move || {
            if batch.started.try_get_untracked() == Some(true) {
                cleanup_cancel();
            }
        });
        Effect::new(move || {
            if !active.get() {
                batch.finish(true);
            }
        });
        view! {
            <section node_ref=node class="space-y-4 border-t border-border pt-4" data-testid="batch-hsl" data-theme-group="other"
                on:keydown=move |event| {if event.key()=="Escape" && !event.is_composing() {event.prevent_default();event.stop_propagation();batch.finish(false);}}
                on:focusout=move |event| {if scope_open.get_untracked() {return;}let inside=event.related_target().and_then(|target|target.dyn_into::<leptos::web_sys::Node>().ok()).is_some_and(|target|node.get_untracked().is_some_and(|node|node.contains(Some(&target))));if !inside {batch.finish(true);}}>
                <h2 class="font-semibold">{move || tr(locale.get(), "Batch HSL adjustment", "批量 HSL 调整")}</h2>
                <p class="text-xs text-muted-foreground">{move || tr(locale.get(), "Preview updates are live. Apply, or leave this panel, to record one undo step. Escape or Cancel restores the starting theme.", "调整会实时预览。应用或离开此面板后记录一个撤销步骤。Esc 或取消可恢复起点主题。")}</p>
                <label class="block space-y-2"><span>{move || tr(locale.get(), "Apply to", "作用范围")}</span>
                    <Select value=Signal::derive(move || if batch.scope.get()==TokenScope::Both {"both".to_owned()} else {"current".to_owned()})
                        options=Signal::derive(move ||vec![SelectOption::new("both",tr(locale.get(),"Both modes","浅深色模式")),SelectOption::new("current",tr(locale.get(),"Current mode","当前模式"))])
                        open=scope_open on_open_change=move |value|scope_open.set(value) on_change=move |value| {batch.finish(true);batch.scope.set(if value=="current" {TokenScope::Current} else {TokenScope::Both});} test_id="hsl-scope" />
                </label>
                {[Axis::Hue,Axis::Saturation,Axis::Lightness].into_iter().map(move |axis| view! {<HslAxis axis=axis batch=batch locale=locale />}).collect_view()}
                <details><summary class="cursor-pointer text-sm">{move || tr(locale.get(), "Adjustment presets", "快捷调整")}</summary><div class="flex flex-wrap gap-1 mt-2">
                    {PRESETS.into_iter().enumerate().map(move |(index,(en,zh,shift))|view! {<Button size=ButtonSize::Sm variant=ButtonVariant::Outline on_click=move ||batch.update(shift) test_id=format!("hsl-preset-{index}")>{move ||tr(locale.get(),en,zh)}</Button>}).collect_view()}
                </div></details>
                <div class="flex flex-wrap gap-2">
                    <Button on_click=move ||batch.finish(true) disabled=Signal::derive(move ||!batch.started.get()) test_id="hsl-apply">{move ||tr(locale.get(),"Apply","应用")}</Button>
                    <Button on_click=move ||batch.finish(false) variant=ButtonVariant::Outline disabled=Signal::derive(move ||!batch.started.get()) test_id="hsl-cancel">{move ||tr(locale.get(),"Cancel","取消")}</Button>
                    <Button on_click=move || {batch.update(HslShift::default());batch.generation.update(|value|*value+=1);} variant=ButtonVariant::Ghost test_id="hsl-reset">{move ||tr(locale.get(),"Reset adjustments","重置调整数值")}</Button>
                </div>
                <p role="status" class="text-xs text-muted-foreground" data-testid="hsl-status">{move ||if batch.started.get() {tr(locale.get(),"Live preview · awaiting Apply","实时预览 · 等待应用")} else {tr(locale.get(),"Ready for a new adjustment","可开始新的调整")}}</p>
            </section>
        }
    }

    #[component]
    fn HslAxis(axis: Axis, batch: Batch, locale: Signal<Locale>) -> impl IntoView {
        let (min, max, step) = axis.bounds();
        let value = Signal::derive(move || axis.get(batch.shift.get()));
        let draft = RwSignal::new(value.get_untracked().to_string());
        let focused = StoredValue::new(false);
        let dirty = StoredValue::new(false);
        let composing = StoredValue::new(false);
        let generation = StoredValue::new(batch.generation.get_untracked());
        let id = rustify_components::id::next("hsl");
        let slider_id = format!("{id}-slider");
        let error_id = format!("{id}-error");
        let parse = move |raw: &str| {
            raw.trim()
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite() && (min..=max).contains(value))
        };
        let invalid = Memo::new(move |_| parse(&draft.get()).is_none());
        Effect::new(move || {
            let current = value.get();
            let next_generation = batch.generation.get();
            if next_generation != generation.get_value() {
                generation.set_value(next_generation);
                dirty.set_value(false);
                draft.set(current.to_string());
            } else if !composing.get_value() && (!focused.get_value() || !dirty.get_value()) {
                draft.set(current.to_string());
            }
        });
        let preview = move |raw: String| {
            draft.set(raw.clone());
            dirty.set_value(true);
            if let Some(value) = parse(&raw) {
                batch.axis(axis, value);
            }
        };
        view! {
            <div class="space-y-2">
                <div class="flex items-center justify-between gap-3"><label for=id.clone()>{move ||axis.label(locale.get())}</label>
                    <input type="text" inputmode="decimal" autocomplete="off" spellcheck="false" role="spinbutton" id=id class=format!("{FIELD} max-w-24 tabular-nums")
                        data-testid=format!("hsl-{}",axis.name()) aria-valuemin=min.to_string() aria-valuemax=max.to_string() aria-valuenow=move ||value.get().to_string() aria-invalid=move ||if invalid.get() {"true"} else {"false"} aria-describedby=error_id.clone() prop:value=move ||draft.get()
                        on:focus=move |_|focused.set_value(true)
                        on:compositionstart=move |_|composing.set_value(true)
                        on:compositionend:target=move |event| {composing.set_value(false);preview(event.target().value());}
                        on:input:target=move |event| {if !composing.get_value() {preview(event.target().value());}}
                        on:blur=move |_| {focused.set_value(false);dirty.set_value(false);draft.set(value.get_untracked().to_string());}
                        on:keydown=move |event| {if event.is_composing() ||composing.get_value() {return;}let multiplier=match event.key().as_str() {"ArrowUp"=>1.,"ArrowDown"=>-1.,"PageUp"=>10.,"PageDown"=>-10.,"Enter"=>{event.prevent_default();if let Some(value)=parse(&draft.get_untracked()) {batch.axis(axis,value);}return;},_=>return};event.prevent_default();let from=parse(&draft.get_untracked()).unwrap_or_else(||value.get_untracked());let next=rustify_components::number_field::stepped(from,step*multiplier).clamp(min,max);preview(next.to_string());} />
                </div>
                <label class="block" for=slider_id.clone()><span class="sr-only">{move ||axis.label(locale.get())}</span><Slider id=slider_id.clone() value=value min=min max=max step=step on_change=move |value|batch.axis(axis,value) test_id=format!("hsl-{}-slider",axis.name()) /></label>
                <p class="text-xs text-destructive" id=error_id role="status">{move ||if invalid.get() {format!("{} {min}…{max}",tr(locale.get(),"Use a finite number in","请输入有限数值，范围："))} else {String::new()}}</p>
            </div>
        }
    }

    #[component]
    pub fn ThemeDiagnostics(
        #[prop(into)] resolved: Signal<Arc<ResolvedTheme>>,
        #[prop(into)] locale: Signal<Locale>,
    ) -> impl IntoView {
        let gamut = Memo::new(move |_| {
            let theme = resolved.get();
            COLOR_TOKENS
                .iter()
                .copied()
                .filter(|token| {
                    theme
                        .values
                        .get(token)
                        .ok()
                        .and_then(|value| parse_color(value).ok())
                        .is_some_and(|color| color.out_of_gamut)
                })
                .collect::<Vec<_>>()
        });
        view! {
            <section class="theme-diagnostics space-y-4 text-sm" data-testid="theme-diagnostics">
                <h2 class="font-semibold">{move ||tr(locale.get(),"Preview diagnostics","预览诊断")}</h2>
                <div role="status" data-testid="gamut-diagnostics"><Show when=move ||!gamut.get().is_empty() fallback=move ||view! {<p class="text-muted-foreground">{move ||tr(locale.get(),"Authored colors fit the sRGB preview.","作者颜色未超出 sRGB 预览色域。")}</p>}>
                    <p class="text-destructive">{move ||format!("{} {}",tr(locale.get(),"Preview clipped to sRGB; author values are preserved:","预览已裁剪至 sRGB，作者值保留："),gamut.get().join(", "))}</p>
                </Show></div>
                <div class="space-y-2" data-testid="font-diagnostics">
                    {move ||resolved.get().fonts.clone().into_iter().enumerate().map(|(index,font)| {
                        let slot=["font-sans","font-serif","font-mono"][index];
                        let requested=font.requested.join(", ");
                        let fallback=font.fallback;
                        let packaged=font.resource_path.is_some();
                        let family=font.family;
                        view! {<p class="break-words" data-testid=format!("diagnostic-{slot}")><code>{slot}</code>{format!(" · {requested} → {family}")}
                            <span class="text-muted-foreground">{move ||if !packaged {tr(locale.get()," · no packaged resource; preview cannot guarantee matching fonts"," · 没有已打包资源，预览无法保证字体一致")} else if fallback {tr(locale.get()," · bundled fallback"," · 已打包字体回退")} else {tr(locale.get()," · requested bundled font"," · 使用所选已打包字体")}}</span>
                        </p>}
                    }).collect_view()}
                </div>
                <details open=true><summary class="cursor-pointer font-medium">{move ||tr(locale.get(),"Text contrast pairs","文字对比度")}</summary>
                    <p class="text-xs text-muted-foreground mt-2">{move ||tr(locale.get(),"Text alpha is composited over each surface. A translucent surface without its actual underlay is unknown. Body text needs 4.5:1; large text needs 3:1. Verdicts use the unrounded ratio.","文字透明度会与表面合成。半透明表面缺少实际底色时无法判定。普通文字要求 4.5:1，大文字要求 3:1；结论使用未四舍五入的比值。")}</p>
                    <ul class="space-y-2 mt-3">
                        {CONTRAST_PAIRS.into_iter().map(move |(foreground,background)|view! {
                            <li class="border-b border-border pb-2" data-testid=format!("contrast-{foreground}-on-{background}")>
                                <code class="text-xs break-words">{format!("{foreground} / {background}")}</code>
                                <p>{move ||match pair_contrast(&resolved.get(),foreground,background) {
                                    None=>tr(locale.get(),"Unknown · actual backing required","无法判定 · 需要实际底色").to_owned(),
                                    Some(ratio)=>format!("{ratio:.3}:1 · {}",if ratio>=4.5 {tr(locale.get(),"Meets body-text threshold","达到普通文字阈值")} else if ratio>=3. {tr(locale.get(),"Large text only; body text below threshold","仅达到大文字阈值；普通文字不足")} else {tr(locale.get(),"Below text thresholds","低于文字阈值")}),
                                }}</p>
                            </li>
                        }).collect_view()}
                    </ul>
                </details>
            </section>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustify_ui::{
        theme::{font_context, resolve},
        Theme, ThemeDocument, ThemeMode,
    };

    fn theme() -> ResolvedTheme {
        resolve(
            &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
            ThemeMode::Light,
            &font_context(20., false),
        )
        .unwrap()
    }

    #[test]
    fn author_parameters_preserve_supported_units_and_reject_incomplete_text() {
        let theme = theme();
        for (token, value) in [
            ("font-sans", "Unknown Face, serif"),
            ("radius", "0"),
            ("spacing", ".25rem"),
            ("shadow-offset-y", "-2px"),
            ("shadow-opacity", ".4"),
            ("letter-spacing", "-.5em"),
            ("letter-spacing", "normal"),
        ] {
            assert!(valid_author_value(token, value, &theme), "{token}: {value}");
        }
        for (token, value) in [
            ("font-sans", "\"Open quote"),
            ("radius", "-"),
            ("spacing", "0"),
            ("font-size", "12"),
            ("shadow-blur", "-1px"),
            ("shadow-opacity", "1e"),
            ("shadow-opacity", "NaN"),
            ("letter-spacing", ".51em"),
            ("letter-spacing", "0."),
        ] {
            assert!(
                !valid_author_value(token, value, &theme),
                "{token}: {value}"
            );
        }
        assert!(valid_author_value("letter-spacing", "5px", &theme));
        assert!(!valid_author_value("letter-spacing", "8px", &theme));
    }

    #[test]
    fn translucent_text_is_composited_and_unknown_surface_backing_is_not_a_pass() {
        let mut theme = theme();
        let color = |value| rustify_ui::theme::parse_color(value).unwrap().rgba;
        theme
            .colors
            .insert("foreground", color("rgb(255 255 255 / .5)"));
        theme.colors.insert("background", color("#000"));
        let ratio = pair_contrast(&theme, "foreground", "background").unwrap();
        assert!((ratio - 5.280822809644651).abs() < 1e-8);
        theme.colors.insert("background", color("rgb(0 0 0 / .5)"));
        assert_eq!(pair_contrast(&theme, "foreground", "background"), None);
        theme.colors.insert("popover", color("transparent"));
        assert_eq!(pair_contrast(&theme, "popover-foreground", "popover"), None);
    }
}
