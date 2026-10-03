//! Author-preserving color drafts and application-owned editing transactions.

fn plane_channels(x: f64, y: f64, width: f64, height: f64) -> Option<(f64, f64)> {
    if ![x, y, width, height].iter().all(|value| value.is_finite()) || width <= 0.0 || height <= 0.0
    {
        return None;
    }
    Some((
        (x / width).clamp(0.0, 1.0),
        (1.0 - y / height).clamp(0.0, 1.0),
    ))
}

fn plane_key(s: f64, l: f64, key: &str, shift: bool) -> Option<(f64, f64)> {
    let step = if shift { 0.1 } else { 0.01 };
    let (s, l) = match key {
        "ArrowLeft" => (s - step, l),
        "ArrowRight" => (s + step, l),
        "ArrowUp" => (s, l + step),
        "ArrowDown" => (s, l - step),
        "PageUp" => (s, l + 0.1),
        "PageDown" => (s, l - 0.1),
        "Home" => (0.0, l),
        "End" => (1.0, l),
        _ => return None,
    };
    Some((s.clamp(0.0, 1.0), l.clamp(0.0, 1.0)))
}

#[cfg(target_arch = "wasm32")]
pub use ui::ColorEditor;

#[cfg(target_arch = "wasm32")]
mod ui {
    use super::{plane_channels, plane_key};
    use leptos::{
        ev::{KeyboardEvent, PointerEvent},
        prelude::*,
        wasm_bindgen::JsCast,
    };
    use rustify_components::{Button, ButtonSize, ButtonVariant, Select, SelectOption};
    use rustify_ui::theme::{
        format_color, hsla_to_rgba, parse_color, rgba_to_hsla, ColorFormat, Hsla, Rgba,
        COLOR_TOKENS,
    };
    use rustify_ui::{Locale, ThemeValues};
    use std::sync::Arc;

    #[derive(Clone, Copy)]
    struct ColorGroup {
        en: &'static str,
        zh: &'static str,
        rows: &'static [(&'static str, Option<&'static str>)],
    }

    const GROUPS: [ColorGroup; 7] = [
        ColorGroup {
            en: "Surfaces",
            zh: "表面",
            rows: &[
                ("background", Some("foreground")),
                ("card", Some("card-foreground")),
                ("popover", Some("popover-foreground")),
            ],
        },
        ColorGroup {
            en: "Semantic colors",
            zh: "语义色",
            rows: &[
                ("primary", Some("primary-foreground")),
                ("secondary", Some("secondary-foreground")),
                ("muted", Some("muted-foreground")),
                ("accent", Some("accent-foreground")),
                ("destructive", Some("destructive-foreground")),
            ],
        },
        ColorGroup {
            en: "Borders & focus",
            zh: "边框与焦点",
            rows: &[("border", None), ("input", None), ("ring", None)],
        },
        ColorGroup {
            en: "Charts",
            zh: "图表",
            rows: &[
                ("chart-1", None),
                ("chart-2", None),
                ("chart-3", None),
                ("chart-4", None),
                ("chart-5", None),
            ],
        },
        ColorGroup {
            en: "Sidebar",
            zh: "侧栏",
            rows: &[
                ("sidebar", Some("sidebar-foreground")),
                ("sidebar-primary", Some("sidebar-primary-foreground")),
                ("sidebar-accent", Some("sidebar-accent-foreground")),
                ("sidebar-border", None),
                ("sidebar-ring", None),
            ],
        },
        ColorGroup {
            en: "Feedback",
            zh: "反馈",
            rows: &[("success", None), ("warning", None)],
        },
        ColorGroup {
            en: "Shadow",
            zh: "阴影",
            rows: &[("shadow-color", None)],
        },
    ];

    fn tr(locale: Locale, en: &'static str, zh: &'static str) -> &'static str {
        if locale == Locale::Chinese {
            zh
        } else {
            en
        }
    }

    fn matches(group: ColorGroup, row: (&str, Option<&str>), query: &str, locale: Locale) -> bool {
        let query = query.trim().to_lowercase();
        query.is_empty()
            || row.0.contains(&query)
            || row.1.is_some_and(|token| token.contains(&query))
            || tr(locale, group.en, group.zh)
                .to_lowercase()
                .contains(&query)
    }

    fn author(values: &ThemeValues, token: &str) -> String {
        values.get(token).unwrap_or_default().to_owned()
    }

    fn notation(value: &str) -> ColorFormat {
        let value = value.trim().to_ascii_lowercase();
        if value.starts_with("oklch(") {
            ColorFormat::Oklch
        } else if value.starts_with("hsl") {
            ColorFormat::Hsl
        } else if value.starts_with("rgb") || value == "transparent" {
            ColorFormat::Rgb
        } else {
            ColorFormat::Hex
        }
    }

    fn notation_name(format: ColorFormat) -> &'static str {
        match format {
            ColorFormat::Hex => "HEX",
            ColorFormat::Rgb => "RGB",
            ColorFormat::Hsl => "HSL",
            ColorFormat::Oklch => "OKLCH",
        }
    }

    fn display(raw: &str, format: Option<ColorFormat>) -> String {
        match (format, parse_color(raw)) {
            (Some(format), Ok(color)) => format_color(color.rgba, format),
            _ => raw.to_owned(),
        }
    }

    #[derive(Clone)]
    struct Callbacks {
        edit: Arc<dyn Fn(String, String) + Send + Sync>,
        begin: Arc<dyn Fn() + Send + Sync>,
        update: Arc<dyn Fn(String, String) + Send + Sync>,
        commit: Arc<dyn Fn() + Send + Sync>,
        cancel: Arc<dyn Fn() + Send + Sync>,
    }

    #[derive(Clone, Debug)]
    struct Draft {
        token: String,
        value: String,
        dirty: bool,
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Source {
        Pointer(i32),
        Keyboard,
        Other,
    }

    #[derive(Clone)]
    struct Gesture {
        token: String,
        control: &'static str,
        start: Hsla,
        source: Source,
    }

    #[derive(Clone, Copy)]
    struct Picker {
        hsl: RwSignal<Hsla>,
        token: Memo<String>,
        gesture: StoredValue<Option<Gesture>>,
        callbacks: StoredValue<Callbacks>,
    }

    impl Picker {
        fn begin(self, control: &'static str, source: Source) {
            if self.gesture.with_value(|gesture| {
                gesture
                    .as_ref()
                    .is_some_and(|gesture| gesture.control == control)
            }) {
                return;
            }
            if self.gesture.with_value(Option::is_some) {
                self.finish(None, false)
            }
            self.gesture.set_value(Some(Gesture {
                token: self.token.get_untracked(),
                control,
                start: self.hsl.get_untracked(),
                source,
            }));
            self.callbacks.with_value(|callbacks| (callbacks.begin)());
        }

        fn update(self, hsl: Hsla) {
            let Some(token) = self
                .gesture
                .with_value(|gesture| gesture.as_ref().map(|gesture| gesture.token.clone()))
            else {
                return;
            };
            let before = hsla_to_rgba(self.hsl.get_untracked());
            let color = hsla_to_rgba(hsl);
            self.hsl.set(hsl);
            if color == before {
                return;
            }
            // RGB preserves continuous alpha/channel values without HEX byte rounding.
            let value = format_color(color, ColorFormat::Rgb);
            self.callbacks
                .with_value(|callbacks| (callbacks.update)(token, value));
        }

        fn finish(self, control: Option<&str>, cancel: bool) {
            let Some(gesture) = self
                .gesture
                .get_value()
                .filter(|gesture| control.is_none_or(|control| gesture.control == control))
            else {
                return;
            };
            self.gesture.set_value(None);
            if cancel {
                self.hsl.set(gesture.start);
                self.callbacks.with_value(|callbacks| (callbacks.cancel)());
            } else {
                self.callbacks.with_value(|callbacks| (callbacks.commit)())
            }
        }

        fn has_pointer(self, control: &str, pointer: i32) -> bool {
            self.gesture.with_value(|gesture| {
                gesture.as_ref().is_some_and(|gesture| {
                    gesture.control == control && gesture.source == Source::Pointer(pointer)
                })
            })
        }

        fn channel(self, channel: &str) -> f64 {
            let hsl = self.hsl.get();
            let color = hsla_to_rgba(hsl);
            match channel {
                "r" => color.r * 255.0,
                "g" => color.g * 255.0,
                "b" => color.b * 255.0,
                "h" => hsl.h,
                "s" => hsl.s * 100.0,
                "l" => hsl.l * 100.0,
                _ => hsl.a,
            }
        }

        fn set_channel(self, channel: &str, value: f64) {
            if !value.is_finite() {
                return;
            }
            let mut hsl = self.hsl.get_untracked();
            match channel {
                "h" => hsl.h = value.clamp(0.0, 360.0),
                "s" => hsl.s = value.clamp(0.0, 100.0) / 100.0,
                "l" => hsl.l = value.clamp(0.0, 100.0) / 100.0,
                "a" => hsl.a = value.clamp(0.0, 1.0),
                _ => {
                    let mut color = hsla_to_rgba(hsl);
                    let value = value.clamp(0.0, 255.0) / 255.0;
                    match channel {
                        "r" => color.r = value,
                        "g" => color.g = value,
                        "b" => color.b = value,
                        _ => return,
                    }
                    let mut converted = rgba_to_hsla(color);
                    if converted.s == 0.0 {
                        converted.h = hsl.h
                    }
                    hsl = converted;
                }
            }
            self.update(hsl);
        }
    }

    fn capture(event: &PointerEvent) {
        if let Some(element) = event
            .current_target()
            .and_then(|target| target.dyn_into::<leptos::web_sys::Element>().ok())
        {
            let _ = element.set_pointer_capture(event.pointer_id());
        }
    }

    fn range_key(key: &str) -> bool {
        matches!(
            key,
            "ArrowLeft"
                | "ArrowRight"
                | "ArrowUp"
                | "ArrowDown"
                | "PageUp"
                | "PageDown"
                | "Home"
                | "End"
        )
    }

    #[component]
    fn ColorSwatch(values: Signal<ThemeValues>, token: &'static str) -> impl IntoView {
        let node = NodeRef::<leptos::html::Span>::new();
        Effect::new(move || {
            let raw = values.with(|values| author(values, token));
            if let (Some(node), Ok(color)) = (node.get(), parse_color(&raw)) {
                let element: leptos::web_sys::HtmlElement = node.into();
                let _ = element
                    .style()
                    .set_property("--ce-color", &format_color(color.rgba, ColorFormat::Rgb));
            }
        });
        view! { <span node_ref=node class="ce-swatch" aria-hidden="true" /> }
    }

    #[component]
    fn TokenChoice(
        values: Signal<ThemeValues>,
        baseline: Signal<ThemeValues>,
        locale: Signal<Locale>,
        selected: RwSignal<String>,
        token: &'static str,
        callbacks: StoredValue<Callbacks>,
    ) -> impl IntoView {
        view! {
            <div class="ce-token-choice">
                <button type="button" class="ce-token" data-testid=format!("color-token-{token}")
                    aria-pressed=move || (selected.get()==token).to_string() title=move || values.with(|values|author(values,token))
                    on:click=move |_| selected.set(token.into())>
                    <ColorSwatch values=values token=token />
                    <span class="ce-token-name">{token}</span>
                </button>
                <button type="button" class="ce-token-reset" data-testid=format!("color-reset-{token}")
                    aria-label=move || format!("{} {token}",tr(locale.get(),"Reset","重置"))
                    title=move || tr(locale.get(),"Reset to baseline","恢复基准值")
                    disabled=move || baseline.with(|baseline|values.with(|values|baseline.get(token)==values.get(token)))
                    on:click=move |_| {
                        if let Ok(value)=baseline.with_untracked(|values|values.get(token).map(str::to_owned)) {
                            callbacks.with_value(|callbacks|(callbacks.edit)(token.into(),value));
                        }
                    }>"↺"</button>
            </div>
        }
    }

    #[component]
    fn Channel(
        picker: Picker,
        locale: Signal<Locale>,
        channel: &'static str,
        label_en: &'static str,
        label_zh: &'static str,
        max: f64,
        step: f64,
    ) -> impl IntoView {
        view! {
            <label class="ce-channel">
                <span>{move || tr(locale.get(),label_en,label_zh)}</span>
                <input type="range" class="ce-range" data-channel=channel data-testid=format!("color-channel-{channel}")
                    min="0" max=max step=step prop:value=move || picker.channel(channel)
                    aria-label=move || tr(locale.get(),label_en,label_zh)
                    on:pointerdown=move |event:PointerEvent| { if event.is_primary() && event.button()==0 {picker.begin(channel,Source::Pointer(event.pointer_id()));capture(&event)} }
                    on:pointerup=move |event:PointerEvent| {if picker.has_pointer(channel,event.pointer_id()) {picker.finish(Some(channel),false)}}
                    on:pointercancel=move |event:PointerEvent| {if picker.has_pointer(channel,event.pointer_id()) {picker.finish(Some(channel),true)}}
                    on:keydown=move |event:KeyboardEvent| {
                        if event.is_composing() {return}
                        if event.key()=="Escape" {event.prevent_default();picker.finish(Some(channel),true)}
                        else if range_key(&event.key()) {picker.begin(channel,Source::Keyboard)}
                    }
                    on:keyup=move |event:KeyboardEvent| {if !event.is_composing() && range_key(&event.key()) {picker.finish(Some(channel),false)}}
                    on:input=move |event| {
                        if let Ok(value)=event_target_value(&event).parse::<f64>() {picker.begin(channel,Source::Other);picker.set_channel(channel,value)}
                    }
                    on:change=move |_| {if picker.gesture.with_value(|gesture|gesture.as_ref().is_some_and(|gesture|gesture.control==channel && gesture.source==Source::Other)) {picker.finish(Some(channel),false)}}
                    on:blur=move |_| picker.finish(Some(channel),false) />
                <output>{move || {let value=picker.channel(channel);if channel=="a" {format!("{value:.2}")} else {format!("{value:.0}")}}}</output>
            </label>
        }
    }

    #[component]
    pub fn ColorEditor(
        values: Signal<ThemeValues>,
        baseline: Signal<ThemeValues>,
        locale: Signal<Locale>,
        selected: RwSignal<String>,
        on_edit: impl Fn(String, String) + Clone + Send + Sync + 'static,
        on_begin: impl Fn() + Clone + Send + Sync + 'static,
        on_update: impl Fn(String, String) + Clone + Send + Sync + 'static,
        on_commit: impl Fn() + Clone + Send + Sync + 'static,
        on_cancel: impl Fn() + Clone + Send + Sync + 'static,
    ) -> impl IntoView {
        let callbacks = StoredValue::new(Callbacks {
            edit: Arc::new(on_edit),
            begin: Arc::new(on_begin),
            update: Arc::new(on_update),
            commit: Arc::new(on_commit),
            cancel: Arc::new(on_cancel),
        });
        let query = RwSignal::new(String::new());
        let chosen = Memo::new(move |_| {
            let token = selected.get();
            if COLOR_TOKENS.contains(&token.as_str()) {
                token
            } else {
                "primary".into()
            }
        });
        let format = RwSignal::new(None::<ColorFormat>);
        let format_open = RwSignal::new(false);
        let focused = StoredValue::new(false);
        let composing = StoredValue::new(false);
        let initial = chosen.get_untracked();
        let draft = RwSignal::new(Draft {
            value: author(&values.get_untracked(), &initial),
            token: initial,
            dirty: false,
        });
        let hsl = RwSignal::new(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        });
        let picker = Picker {
            hsl,
            token: chosen,
            gesture: StoredValue::new(None),
            callbacks,
        };
        let field = NodeRef::<leptos::html::Input>::new();
        let plane = NodeRef::<leptos::html::Div>::new();
        let picker_node = NodeRef::<leptos::html::Div>::new();
        let cleanup_cancel = callbacks.with_value(|callbacks| Arc::clone(&callbacks.cancel));
        on_cleanup(move || {
            if picker.gesture.try_get_value().flatten().is_some() {
                cleanup_cancel();
            }
        });

        Effect::new(move || {
            let token = chosen.get();
            let raw = author(&values.get(), &token);
            let format = format.get();
            let pending = draft.get_untracked();
            if !focused.get_value()
                && !composing.get_value()
                && (!pending.dirty || pending.token != token)
            {
                draft.set(Draft {
                    token,
                    value: display(&raw, format),
                    dirty: false,
                });
            }
        });
        Effect::new(move || {
            let token = chosen.get();
            let values = values.get();
            let draft = draft.get();
            let raw = if draft.dirty && draft.token == token {
                draft.value
            } else {
                author(&values, &token)
            };
            if !picker.gesture.with_value(Option::is_some) {
                if let Ok(color) = parse_color(&raw) {
                    let mut next = rgba_to_hsla(color.rgba);
                    if next.s == 0.0 {
                        next.h = hsl.get_untracked().h
                    }
                    hsl.set(next);
                }
            }
        });
        Effect::new(move || {
            let hsl = hsl.get();
            if let Some(node) = picker_node.get() {
                let element: leptos::web_sys::HtmlElement = node.into();
                let color = hsla_to_rgba(hsl);
                let hue = hsla_to_rgba(Hsla {
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                    ..hsl
                });
                for (name, value) in [
                    ("--ce-color", format_color(color, ColorFormat::Rgb)),
                    ("--ce-hue", format_color(hue, ColorFormat::Rgb)),
                    ("--ce-s", format!("{}%", hsl.s * 100.0)),
                    ("--ce-y", format!("{}%", (1.0 - hsl.l) * 100.0)),
                    (
                        "--ce-opaque",
                        format_color(Rgba { a: 1.0, ..color }, ColorFormat::Rgb),
                    ),
                ] {
                    let _ = element.style().set_property(name, &value);
                }
            }
        });

        let submit = move || {
            if composing.get_value() {
                return;
            }
            let held = draft.get_untracked();
            if held.dirty && parse_color(&held.value).is_ok() {
                callbacks.with_value(|callbacks| (callbacks.edit)(held.token, held.value));
                draft.update(|draft| draft.dirty = false);
            }
        };
        let plane_update = move |event: &PointerEvent| {
            if let Some(node) = plane.get_untracked() {
                let bounds = node.get_bounding_client_rect();
                if let Some((s, l)) = plane_channels(
                    event.client_x() as f64 - bounds.left(),
                    event.client_y() as f64 - bounds.top(),
                    bounds.width(),
                    bounds.height(),
                ) {
                    picker.update(Hsla {
                        s,
                        l,
                        ..hsl.get_untracked()
                    });
                }
            }
        };
        let visible_count = Memo::new(move |_| {
            let query = query.get();
            let locale = locale.get();
            GROUPS
                .iter()
                .map(|group| {
                    group
                        .rows
                        .iter()
                        .filter(|row| matches(*group, **row, &query, locale))
                        .count()
                })
                .sum::<usize>()
        });
        let options = Signal::derive(|| {
            ["HEX", "RGB", "HSL", "OKLCH"]
                .into_iter()
                .map(|value| SelectOption::new(value, value))
                .collect::<Vec<_>>()
        });
        let displayed_format = Signal::derive(move || {
            notation_name(
                format
                    .get()
                    .unwrap_or_else(|| notation(&author(&values.get(), &chosen.get()))),
            )
            .to_owned()
        });
        let invalid = move || {
            let draft = draft.get();
            parse_color(&draft.value).is_err()
        };
        let format_change = move |choice: String| {
            format.set(Some(match choice.as_str() {
                "RGB" => ColorFormat::Rgb,
                "HSL" => ColorFormat::Hsl,
                "OKLCH" => ColorFormat::Oklch,
                _ => ColorFormat::Hex,
            }));
        };

        view! {
            <section class="ce-editor" data-testid="color-editor">
                <div class="ce-search">
                    <input type="search" data-testid="color-search" class="ce-field"
                        aria-label=move || tr(locale.get(),"Search color tokens","搜索颜色 token")
                        placeholder=move || tr(locale.get(),"Search colors…","搜索颜色…")
                        prop:value=move || query.get() on:input=move |event|query.set(event_target_value(&event)) />
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm test_id="color-search-clear"
                        disabled=Signal::derive(move || query.get().is_empty()) on_click=move ||query.set(String::new())>
                        {move || tr(locale.get(),"Clear","清除")}
                    </Button>
                </div>
                <div class="ce-picker" node_ref=picker_node>
                    <h3 class="ce-picked-token">{move || chosen.get()}</h3>
                    <div class="ce-plane" node_ref=plane role="slider" tabindex="0" data-testid="color-plane"
                        aria-label=move || tr(locale.get(),"Saturation and lightness","饱和度与亮度")
                        aria-valuemin="0" aria-valuemax="100" aria-valuenow=move || (hsl.get().s*100.0).round()
                        aria-valuetext=move || {let hsl=hsl.get();format!("{} {:.0}%, {} {:.0}%",tr(locale.get(),"Saturation","饱和度"),hsl.s*100.0,tr(locale.get(),"Lightness","亮度"),hsl.l*100.0)}
                        aria-describedby="color-plane-help"
                        on:pointerdown=move |event:PointerEvent| {
                            if event.is_primary() && event.button()==0 {event.prevent_default();if let Some(node)=plane.get_untracked(){let _=node.focus();}picker.begin("plane",Source::Pointer(event.pointer_id()));capture(&event);plane_update(&event)}
                        }
                        on:pointermove=move |event:PointerEvent| {if picker.has_pointer("plane",event.pointer_id()){plane_update(&event)}}
                        on:pointerup=move |event:PointerEvent| {if picker.has_pointer("plane",event.pointer_id()){plane_update(&event);picker.finish(Some("plane"),false)}}
                        on:pointercancel=move |event:PointerEvent| {if picker.has_pointer("plane",event.pointer_id()){picker.finish(Some("plane"),true)}}
                        on:keydown=move |event:KeyboardEvent| {
                            if event.is_composing(){return}
                            if event.key()=="Escape"{event.prevent_default();picker.finish(Some("plane"),true);return}
                            let held=hsl.get_untracked();
                            if let Some((s,l))=plane_key(held.s,held.l,&event.key(),event.shift_key()) {event.prevent_default();picker.begin("plane",Source::Keyboard);picker.update(Hsla {s,l,..held});}
                        }
                        on:keyup=move |event:KeyboardEvent| {if !event.is_composing() && range_key(&event.key()){picker.finish(Some("plane"),false)}}
                        on:blur=move |_|picker.finish(Some("plane"),false)>
                        <span class="ce-plane-marker" aria-hidden="true" />
                    </div>
                    <p id="color-plane-help" class="ce-help">{move || tr(locale.get(),"← → saturation · ↑ ↓ lightness · Shift: 10% · Esc: cancel","← → 饱和度 · ↑ ↓ 亮度 · Shift：10% · Esc：取消")}</p>
                    <div class="ce-text-row">
                        <Select value=displayed_format options=options open=format_open on_open_change=move |open|format_open.set(open)
                            on_change=format_change aria_label="HEX / RGB / HSL / OKLCH" test_id="color-format" />
                        <input node_ref=field type="text" class="ce-field ce-color-text" data-testid="color-value"
                            aria-label=move || format!("{} {}",tr(locale.get(),"Color value","颜色值"),draft.get().token)
                            aria-invalid=move || if invalid() {"true"} else {"false"} aria-describedby="color-value-help" spellcheck="false"
                            prop:value=move || draft.get().value
                            on:focus=move |_|focused.set_value(true)
                            on:input=move |event| {if !composing.get_value(){draft.update(|draft|{draft.value=event_target_value(&event);draft.dirty=true})}}
                            on:compositionstart=move |_|composing.set_value(true)
                            on:compositionend=move |event|{composing.set_value(false);draft.update(|draft|{draft.value=event_target_value(&event);draft.dirty=true});}
                            on:blur=move |_|{
                                let held=draft.get_untracked();
                                submit();focused.set_value(false);
                                if !held.dirty && parse_color(&held.value).is_ok() {
                                    let token=chosen.get_untracked();
                                    draft.set(Draft {value:display(&author(&values.get_untracked(),&token),format.get_untracked()),token,dirty:false});
                                }
                            }
                            on:keydown=move |event:KeyboardEvent| {
                                if composing.get_value() || event.is_composing(){return}
                                if event.key()=="Enter"{event.prevent_default();submit()}
                                else if event.key()=="Escape"{event.prevent_default();let token=draft.get_untracked().token;draft.set(Draft {value:display(&author(&values.get_untracked(),&token),format.get_untracked()),token,dirty:false});}
                            } />
                    </div>
                    <p id="color-value-help" class="ce-help" role="status">{move || if invalid(){tr(locale.get(),"Invalid color. Use HEX, RGB, HSL or OKLCH; preview keeps the last valid theme.","颜色无效。支持 HEX、RGB、HSL、OKLCH；预览保留上次有效主题。")}else{tr(locale.get(),"Format changes the display only. Enter or leave the field to apply.","格式只改变显示。按 Enter 或离开输入框后应用。")}}</p>
                    <div class="ce-channels">
                        <Channel picker=picker locale=locale channel="r" label_en="Red" label_zh="红" max=255.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="g" label_en="Green" label_zh="绿" max=255.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="b" label_en="Blue" label_zh="蓝" max=255.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="h" label_en="Hue" label_zh="色相" max=360.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="s" label_en="Saturation" label_zh="饱和度" max=100.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="l" label_en="Lightness" label_zh="亮度" max=100.0 step=1.0 />
                        <Channel picker=picker locale=locale channel="a" label_en="Alpha" label_zh="透明度" max=1.0 step=0.01 />
                    </div>
                </div>
                <Show when=move || visible_count.get()==0>
                    <p class="ce-empty" data-testid="color-search-empty" role="status">{move || tr(locale.get(),"No matching colors. Clear search to show every token.","没有匹配颜色。清除搜索以显示全部 token。")}</p>
                </Show>
                {GROUPS.into_iter().map(move |group|view!{
                    <Show when=move ||group.rows.iter().any(|row|matches(group,*row,&query.get(),locale.get()))>
                        <div class="ce-group">
                            <h3>{move ||tr(locale.get(),group.en,group.zh)}</h3>
                            <For each={move ||group.rows.iter().copied().filter(|row|matches(group,*row,&query.get(),locale.get())).collect::<Vec<_>>()} key=|row|row.0 children=move |(background,foreground)|view!{
                                <div class="ce-pair">
                                    <TokenChoice values=values baseline=baseline locale=locale selected=selected token=background callbacks=callbacks />
                                    {foreground.map(|token|view!{<TokenChoice values=values baseline=baseline locale=locale selected=selected token=token callbacks=callbacks />})}
                                </div>
                            } />
                        </div>
                    </Show>
                }).collect_view()}
            </section>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{plane_channels, plane_key};

    #[test]
    fn plane_corners_and_center_match_saturation_lightness() {
        assert_eq!(plane_channels(0.0, 0.0, 200.0, 100.0), Some((0.0, 1.0)));
        assert_eq!(plane_channels(200.0, 100.0, 200.0, 100.0), Some((1.0, 0.0)));
        assert_eq!(plane_channels(100.0, 50.0, 200.0, 100.0), Some((0.5, 0.5)));
    }

    #[test]
    fn captured_pointer_coordinates_clamp_at_edges() {
        assert_eq!(plane_channels(-9.0, 130.0, 200.0, 100.0), Some((0.0, 0.0)));
        assert_eq!(plane_channels(300.0, -10.0, 200.0, 100.0), Some((1.0, 1.0)));
    }

    #[test]
    fn zero_size_and_nonfinite_geometry_are_not_color_updates() {
        assert_eq!(plane_channels(1.0, 1.0, 0.0, 20.0), None);
        assert_eq!(plane_channels(f64::NAN, 1.0, 20.0, 20.0), None);
        assert_eq!(plane_channels(1.0, 1.0, 20.0, f64::INFINITY), None);
    }

    #[test]
    fn keyboard_preserves_the_other_axis_and_shift_step() {
        assert_eq!(plane_key(0.5, 0.5, "ArrowRight", false), Some((0.51, 0.5)));
        assert_eq!(plane_key(0.5, 0.5, "ArrowUp", true), Some((0.5, 0.6)));
        assert_eq!(plane_key(0.0, 1.0, "ArrowLeft", true), Some((0.0, 1.0)));
        assert_eq!(plane_key(1.0, 1.0, "PageUp", false), Some((1.0, 1.0)));
    }

    #[test]
    fn home_end_do_not_change_lightness_and_unknown_keys_pass_through() {
        assert_eq!(plane_key(0.5, 0.4, "Home", false), Some((0.0, 0.4)));
        assert_eq!(plane_key(0.5, 0.4, "End", false), Some((1.0, 0.4)));
        assert_eq!(plane_key(0.5, 0.4, "Tab", false), None);
    }
}
