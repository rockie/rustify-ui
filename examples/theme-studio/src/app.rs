use crate::{
    color_editor::ColorEditor,
    controller::{now, StudioState},
    data_tools::{DataTools, ToolCommand},
    gpu_preview::GpuPreview,
    inspector::Inspector,
    presets,
    preview_region::{
        PreviewAction, PreviewControls, PreviewProps as GpuPreviewProps, PreviewValue,
    },
    provider_fixture::ProviderFixture,
    scenes::{ScenePreview, SCENE_NAMES},
    theme_controls::{ThemeControls, ThemeDiagnostics},
};
use leptos::{
    ev::PointerEvent,
    prelude::*,
    wasm_bindgen::{prelude::*, JsCast},
};
use rustify_components::{Button, ButtonSize, ButtonVariant, Menu, MenuItem, Select, SelectOption};
use rustify_ui::{
    mount,
    theme::{font_context, COMMON_TOKENS},
    Anchor, AppHandle, LocalRect, MountConfig, Theme, ThemeBoundary, ThemeDocument, ThemeMode,
    ThemeScope, ThemeValuePatch, ThemedScope,
};
use std::{cell::RefCell, collections::BTreeMap};

fn neutral_patch() -> ThemeValuePatch {
    ThemeValuePatch(
        ThemeDocument::from_legacy(Theme::light(), Theme::dark())
            .styles
            .light
            .0,
    )
}

fn tool_command(state: StudioState, command: ToolCommand) {
    match command {
        ToolCommand::LoadLocal(id) => state.load_local(id),
        ToolCommand::Import(document) => {
            state.apply(document, false);
        }
        ToolCommand::Save { name, overwrite } => state.save_named(name, overwrite),
        ToolCommand::Copy { id, name } => state.copy_named(id, name),
        ToolCommand::Rename { id, name } => state.rename_named(id, name),
        ToolCommand::Delete(id) => state.delete_named(id),
        ToolCommand::ClearCache => state.clear_cache(),
        ToolCommand::LoadExternal => state.load_external(),
        ToolCommand::KeepCurrent => state.keep_current(),
        ToolCommand::SaveConflictCopy { name } => state.conflict_copy(name),
    }
}

fn scene_label(state: StudioState, scene: &'static str) -> &'static str {
    match scene {
        "Cards" => state.tr("Cards", "卡片"),
        "Dashboard" => state.tr("Dashboard", "仪表盘"),
        "Application" => state.tr("Application", "应用"),
        "Marketing" => state.tr("Marketing", "营销"),
        "Mail" => state.tr("Mail", "邮件"),
        "Typography" => state.tr("Typography", "排版"),
        _ => state.tr("Color palette", "调色板"),
    }
}

#[component]
fn PresetChooser(state: StudioState) -> impl IntoView {
    let built_in = StoredValue::new(presets::built_in().expect("validated presets"));
    let query = RwSignal::new(String::new());
    let source = RwSignal::new("all".to_owned());
    let favorites = RwSignal::new(false);
    let open = RwSignal::new(false);
    let source_open = RwSignal::new(false);
    let all = Memo::new(move |_| {
        let mut themes =
            built_in.with_value(|v| v.iter().cloned().map(|v| (v, false)).collect::<Vec<_>>());
        themes.extend(state.envelope.with(|e| {
            e.saved_themes
                .iter()
                .cloned()
                .map(|v| (v, true))
                .collect::<Vec<_>>()
        }));
        themes
    });
    let options = Signal::derive(move || {
        let query = query.get().to_lowercase();
        let source = source.get();
        let favorite_ids = state.envelope.with(|e| e.preferences.favorites.clone());
        all.get()
            .into_iter()
            .filter(|(theme, local)| {
                (source == "all" || (source == "local") == *local)
                    && (!favorites.get() || favorite_ids.contains(&theme.id))
                    && (theme.name.to_lowercase().contains(&query)
                        || theme.id.to_lowercase().contains(&query))
            })
            .map(|(theme, local)| {
                SelectOption::new(
                    theme.id,
                    format!(
                        "{} · {}",
                        if local {
                            state.tr("Local", "本地")
                        } else {
                            state.tr("Built-in", "内置")
                        },
                        theme.name
                    ),
                )
            })
            .collect::<Vec<_>>()
    });
    let sources = Signal::derive(move || {
        vec![
            SelectOption::new("all", state.tr("All sources", "全部来源")),
            SelectOption::new("built-in", state.tr("Built-in", "内置")),
            SelectOption::new("local", state.tr("Local", "本地")),
        ]
    });
    let selected = Signal::derive(move || state.editor.with(|e| e.document.id.clone()));
    let favorite = Signal::derive(move || {
        state
            .envelope
            .with(|e| e.preferences.favorites.contains(&selected.get()))
    });
    Effect::new(move || {
        state.reset_epoch.get();
        query.set(String::new());
        source.set("all".into());
        favorites.set(false);
        open.set(false);
        source_open.set(false);
    });
    view! {
        <section class="studio-presets" aria-label=move || state.tr("Presets", "预设")>
            <div class="studio-section-title"><h2>{move || state.tr("Presets", "预设")}</h2><span data-testid="preset-count">{move || all.get().len()}</span></div>
            <input type="search" class="studio-search" aria-label=move || state.tr("Search presets", "搜索预设") placeholder=move || state.tr("Search presets…", "搜索预设…") data-testid="preset-search" prop:value=move || query.get() on:input=move |ev| query.set(event_target_value(&ev)) />
            <div class="studio-filter-row">
                <Select value=source options=sources open=source_open on_open_change=move |v| source_open.set(v) on_change=move |v| source.set(v) aria_label="Preset source" test_id="preset-source" />
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || favorites.get().to_string() on_click=move || favorites.update(|v| *v = !*v) test_id="preset-favorites">{move || state.tr("Favorites", "收藏")}</Button>
            </div>
            <Select value=selected options=options disabled=Signal::derive(move || options.get().is_empty()) open=open on_open_change=move |v| open.set(v) on_change=move |id| {
                if let Some((theme, _)) = all.get_untracked().into_iter().find(|(theme, _)| theme.id == id) { state.apply(theme, true); }
            } aria_label="Preset" test_id="preset-select" />
            <div class="studio-filter-row"><span class="studio-current-name">{move || state.editor.with(|e| e.document.name.clone())}</span>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost attr:aria-pressed=move || favorite.get().to_string() on_click=move || state.favorite(selected.get_untracked()) test_id="preset-favorite">{move || if favorite.get() { state.tr("Unfavorite", "取消收藏") } else { state.tr("Favorite", "收藏") }}</Button>
            </div>
            <Show when=move || options.get().is_empty()><p data-testid="preset-search-empty">{move || state.tr("No matching presets.", "没有匹配的预设。")}</p></Show>
            <Show when=move || !query.get().is_empty() || source.get() != "all" || favorites.get()>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost test_id="preset-search-clear" on_click=move || { query.set(String::new()); source.set("all".into()); favorites.set(false); }>{move || state.tr("Clear filters", "清除筛选")}</Button>
            </Show>
        </section>
    }
}

#[component]
fn Editor(state: StudioState) -> impl IntoView {
    let width = RwSignal::new(320.0f64);
    let drag = StoredValue::new(None::<(i32, f64, f64)>);
    let colors_tab = NodeRef::<leptos::html::Button>::new();
    let controls_tab = NodeRef::<leptos::html::Button>::new();
    let tab_key = move |event: leptos::ev::KeyboardEvent| {
        if event.is_composing() {
            return;
        }
        let next = match event.key().as_str() {
            "Home" => "colors",
            "End" => "controls",
            "ArrowLeft" | "ArrowRight" => {
                if state.editor_tab.get_untracked() == "colors" {
                    "controls"
                } else {
                    "colors"
                }
            }
            _ => return,
        };
        event.prevent_default();
        state.editor_tab.set(next.into());
        let node = if next == "colors" {
            colors_tab
        } else {
            controls_tab
        };
        if let Some(node) = node.get_untracked() {
            let _ = node.focus();
        }
    };
    let common_differences = Signal::derive(move || {
        state.editor.with(|e| {
            COMMON_TOKENS
                .iter()
                .filter(|token| {
                    e.document.styles.light.0.get(**token) != e.document.styles.dark.0.get(**token)
                })
                .map(|token| (*token).to_owned())
                .collect::<Vec<_>>()
        })
    });
    let status = Signal::derive(move || {
        let error = state.error.get();
        if error.is_empty() {
            state.storage_status.get()
        } else {
            error
        }
    });
    Effect::new(move || {
        state.reset_epoch.get();
        width.set(320.);
        drag.set_value(None);
    });
    Effect::new(move || {
        if let Ok(Some(layout)) = document().query_selector(".studio-layout") {
            if let Some(html) = layout.dyn_ref::<leptos::web_sys::HtmlElement>() {
                let _ = html
                    .style()
                    .set_property("--studio-editor-width", &format!("{}px", width.get()));
            }
            let _ = layout.set_attribute("data-mobile-tab", &state.mobile_tab.get());
        }
        if let Some(html) = document().document_element() {
            let _ = html.set_attribute(
                "lang",
                if state.locale().get() == rustify_ui::Locale::Chinese {
                    "zh"
                } else {
                    "en"
                },
            );
        }
        if let Some(summary) = document().get_element_by_id("compare-summary") {
            summary.set_text_content(Some(
                state.tr("Compare an independent scope", "对比独立主题作用域"),
            ));
        }
    });
    view! {
        <ThemedScope theme=Signal::derive(Theme::light) />
        <nav class="studio-mobile-tabs" aria-label=move || state.tr("Workbench panels", "工作台面板")>
            <Button variant=ButtonVariant::Ghost attr:aria-pressed=move || (state.mobile_tab.get() == "edit").to_string() on_click=move || state.mobile_tab.set("edit".into()) test_id="mobile-edit">{move || state.tr("Edit", "编辑")}</Button>
            <Button variant=ButtonVariant::Ghost attr:aria-pressed=move || (state.mobile_tab.get() == "preview").to_string() on_click=move || state.mobile_tab.set("preview".into()) test_id="mobile-preview">{move || state.tr("Preview", "预览")}</Button>
        </nav>
        <div class="studio-editor studio-editor-content" data-testid="editor-shell">
            <header><div class="studio-section-title"><h1>"Rustify Theme Studio"</h1><div class="studio-locale">
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost attr:aria-pressed=move || (state.locale().get() == rustify_ui::Locale::English).to_string() on_click=move || state.preference(|p| p.locale = "en".into()) test_id="locale-en">"EN"</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost attr:aria-pressed=move || (state.locale().get() == rustify_ui::Locale::Chinese).to_string() on_click=move || state.preference(|p| p.locale = "zh".into()) test_id="locale-zh">"中文"</Button>
            </div></div><p>{move || state.tr("Shape a theme. See both renderers.", "编辑主题，同步查看两种渲染器。")}</p></header>
            <div class="studio-history">
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline disabled=Signal::derive(move || state.editor.with(|e| e.undo_len() == 0 || e.gesture_active())) on_click=move || state.history(false) test_id="theme-undo">{move || state.tr("Undo", "撤销")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline disabled=Signal::derive(move || state.editor.with(|e| e.redo_len() == 0 || e.gesture_active())) on_click=move || state.history(true) test_id="theme-redo">{move || state.tr("Redo", "重做")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost disabled=Signal::derive(move || state.editor.with(|e| e.gesture_active() || (!e.is_modified() && state.patch.get().0.is_empty() && state.controls.get() == PreviewControls::default()))) on_click=move || state.reset() test_id="theme-reset">{move || state.tr("Reset", "重置")}</Button>
            </div>
            <p class="studio-modified" data-testid="theme-modified">{move || if state.editor.with(|e| e.is_modified()) { state.tr("Modified", "已修改") } else { state.tr("Matches preset", "与预设一致") }}</p>
            <DataTools document=state.document() envelope=state.envelope locale=state.locale() context=Signal::derive(move || font_context(state.rem.get(), state.reduce_motion.get())) corrupt=state.corrupt conflict=Signal::derive(move || state.external.get().is_some()) status=status on_command=move |command| tool_command(state, command) />
            <PresetChooser state=state />
            <div class="editor-mode">
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || (state.mode().get() == ThemeMode::Light).to_string() on_click=move || state.set_mode(ThemeMode::Light) test_id="mode-light">{move || state.tr("Light", "浅色")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || (state.mode().get() == ThemeMode::Dark).to_string() on_click=move || state.set_mode(ThemeMode::Dark) test_id="mode-dark">{move || state.tr("Dark", "深色")}</Button>
            </div>
            <div class="studio-editor-tabs" role="tablist" aria-orientation="horizontal" aria-label=move || state.tr("Theme fields", "主题字段") on:keydown=tab_key>
                <button type="button" role="tab" node_ref=colors_tab id="studio-editor-tab-colors" aria-controls="studio-editor-panel-colors" tabindex=move || if state.editor_tab.get() == "colors" {"0"} else {"-1"} aria-selected=move || (state.editor_tab.get() == "colors").to_string() on:click=move |_| state.editor_tab.set("colors".into()) data-testid="editor-colors">{move || state.tr("Colors", "颜色")}</button>
                <button type="button" role="tab" node_ref=controls_tab id="studio-editor-tab-controls" aria-controls="studio-editor-panel-controls" tabindex=move || if state.editor_tab.get() == "controls" {"0"} else {"-1"} aria-selected=move || (state.editor_tab.get() == "controls").to_string() on:click=move |_| state.editor_tab.set("controls".into()) data-testid="editor-controls">{move || state.tr("Type & other", "字体与其他")}</button>
            </div>
            <div hidden=move || state.editor_tab.get() != "colors" role="tabpanel" id="studio-editor-panel-colors" aria-labelledby="studio-editor-tab-colors">
                <For each=move || vec![state.reset_epoch.get()] key=|epoch| *epoch children=move |_| view! {
                <ColorEditor values=state.values() baseline=state.baseline() locale=state.locale() selected=state.selected_token on_edit=move |token, value| { state.edit(&token, value); } on_begin=move || state.begin() on_update=move |token, value| state.update(token, value) on_commit=move || state.commit() on_cancel=move || state.cancel() />
                } />
            </div>
            <div hidden=move || state.editor_tab.get() != "controls" role="tabpanel" id="studio-editor-panel-controls" aria-labelledby="studio-editor-tab-controls">
                <ThemeControls values=state.values() baseline=state.baseline() locale=state.locale() resolved=state.resolved() common_differences=common_differences active=Signal::derive(move || state.editor_tab.get() == "controls") on_edit=move |token, value| { state.edit(&token, value); } on_begin=move || state.begin() on_hsl=move |shift, scope| state.update_hsl(shift, scope) on_commit=move || state.commit() on_cancel=move || state.cancel() />
            </div>
            <p class="editor-error" role="status" data-testid="field-error">{move || state.error.get()}</p>
            <details class="studio-diagnostics"><summary>{move || state.tr("Diagnostics", "诊断")}</summary><ThemeDiagnostics resolved=state.resolved() locale=state.locale() /></details>
            <details class="studio-isolation"><summary>{move || state.tr("Scope isolation checks", "作用域隔离验证")}</summary>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline on_click=move || state.patch.set(ThemeValuePatch(BTreeMap::from([("radius".into(), "0px".into()), ("shadow-opacity".into(), "0.4".into()), ("popover".into(), "#fff4dd".into())]))) test_id="apply-patch">{move || state.tr("Apply local patch", "应用局部覆盖")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost disabled=Signal::derive(move || state.patch.get().0.is_empty()) on_click=move || state.patch.set(ThemeValuePatch::default()) test_id="clear-patch">{move || state.tr("Clear patch", "清除覆盖")}</Button>
            </details>
            <p class="editor-note">{move || state.tr("Your theme stays inside the preview.", "主题仅作用于预览区域。")}</p>
        </div>
        <div class="studio-separator" role="separator" tabindex="0" aria-orientation="vertical" aria-label=move || state.tr("Resize editor", "调整编辑栏宽度") aria-valuemin="260" aria-valuemax="520" aria-valuenow=move || width.get() data-testid="editor-separator"
            on:pointerdown=move |ev: PointerEvent| { if ev.button() != 0 { return; } ev.prevent_default(); drag.set_value(Some((ev.pointer_id(), ev.client_x() as f64, width.get_untracked()))); if let Some(target) = ev.current_target().and_then(|v| v.dyn_into::<leptos::web_sys::Element>().ok()) { let _ = target.set_pointer_capture(ev.pointer_id()); } }
            on:pointermove=move |ev: PointerEvent| { if let Some((pointer, start, previous)) = drag.get_value() { if pointer == ev.pointer_id() { width.set((previous + ev.client_x() as f64 - start).clamp(260., 520.)); } } }
            on:pointerup=move |_| drag.set_value(None) on:pointercancel=move |_| drag.set_value(None)
            on:keydown=move |ev| { let next = match ev.key().as_str() { "ArrowLeft" => width.get_untracked() - 10., "ArrowRight" => width.get_untracked() + 10., "Home" => 260., "End" => 520., _ => return }; ev.prevent_default(); width.set(next.clamp(260., 520.)); }
        ></div>
    }
}

#[component]
fn PreviewToolbar(state: StudioState, fullscreen: RwSignal<bool>) -> impl IntoView {
    let scene_open = RwSignal::new(false);
    let width_open = RwSignal::new(false);
    let scene = Signal::derive(move || state.envelope.with(|e| e.preferences.scene.clone()));
    let options = Signal::derive(move || {
        SCENE_NAMES
            .iter()
            .map(|name| SelectOption::new(*name, scene_label(state, name)))
            .collect::<Vec<_>>()
    });
    let width =
        Signal::derive(move || state.envelope.with(|e| e.preferences.preview_width.clone()));
    let widths = Signal::derive(move || {
        vec![
            SelectOption::new("responsive", state.tr("Responsive", "自适应")),
            SelectOption::new("desktop", "1024 px"),
            SelectOption::new("tablet", "768 px"),
            SelectOption::new("mobile", "390 px"),
        ]
    });
    Effect::new(move || {
        state.reset_epoch.get();
        fullscreen.set(false);
        scene_open.set(false);
        width_open.set(false);
    });
    view! {
        <header class="studio-preview-toolbar" data-inspector-ui="true" data-testid="preview-toolbar">
            <div class="studio-section-title"><h2>{move || state.tr("Preview", "预览")}</h2><span>{move || state.tr("Live theme", "实时主题")}</span></div>
            <div class="studio-preview-options">
                <Select value=scene options=options open=scene_open on_open_change=move |v| scene_open.set(v) on_change=move |v| state.preference(|p| p.scene = v) aria_label="Preview scene" test_id="preview-scene" />
                <Select value=width options=widths open=width_open on_open_change=move |v| width_open.set(v) on_change=move |v| state.preference(|p| p.preview_width = v) aria_label="Preview width" test_id="preview-width" />
                <div class="studio-renderer-buttons">{[("dom", "DOM"), ("gpu", "GPU"), ("compare", "DOM + GPU")].into_iter().map(move |(id, label)| view! {
                    <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || (state.envelope.with(|e| e.preferences.renderer == id)).to_string() on_click=move || state.preference(|p| p.renderer = id.into()) test_id=format!("renderer-{id}")>{label}</Button>
                }).collect_view()}</div>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || state.inspector.get().to_string() on_click=move || state.inspector.update(|v| *v = !*v) test_id="inspector-toggle">{move || state.tr("Inspect", "拾取")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost attr:aria-pressed=move || fullscreen.get().to_string() on_click=move || fullscreen.update(|v| *v = !*v) test_id="preview-focus">{move || if fullscreen.get() { state.tr("Exit focus", "退出专注") } else { state.tr("Focus preview", "专注预览") }}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Ghost class="studio-back-edit" on_click=move || { fullscreen.set(false); state.mobile_tab.set("edit".into()); } test_id="preview-back-edit">{move || state.tr("Back to edit", "返回编辑")}</Button>
            </div>
        </header>
    }
}

#[component]
fn Preview(state: StudioState, compare: bool) -> impl IntoView {
    let resolved = Signal::derive(move || {
        state.frame.with(|frame| {
            if compare {
                frame.compare.clone()
            } else {
                frame.main.clone()
            }
        })
    });
    let scene = Signal::derive(move || state.envelope.with(|e| e.preferences.scene.clone()));
    if compare {
        return view! {
            <ThemeScope resolved=resolved />
            <ThemeBoundary patch=Signal::derive(neutral_patch)><div class="studio-compare-toolbar" data-inspector-ui="true">
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || (state.compare_mode.get() == ThemeMode::Light).to_string() on_click=move || state.set_compare_mode(ThemeMode::Light) test_id="compare-light">{move || state.tr("Light", "浅色")}</Button>
                <Button size=ButtonSize::Sm variant=ButtonVariant::Outline attr:aria-pressed=move || (state.compare_mode.get() == ThemeMode::Dark).to_string() on_click=move || state.set_compare_mode(ThemeMode::Dark) test_id="compare-dark">{move || state.tr("Dark", "深色")}</Button>
            </div></ThemeBoundary>
            <ScenePreview scene=scene locale=state.locale() />
            <details class="studio-provider-fixture"><summary>{move || state.tr("Provider & overlay checks", "主题作用域与浮层验证")}</summary><ProviderFixture patch=Signal::derive(ThemeValuePatch::default) /></details>
        }.into_any();
    }
    let root = NodeRef::<leptos::html::Div>::new();
    let canvas = NodeRef::<leptos::html::Canvas>::new();
    let fullscreen = RwSignal::new(false);
    let previous_focus = StoredValue::new_local(None::<leptos::web_sys::Element>);
    let dropdown = RwSignal::new(false);
    let dropdown_rect = RwSignal::new(LocalRect {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    });
    let anchor = Signal::derive(move || {
        canvas
            .get()
            .map(|node| {
                let element: leptos::web_sys::Element = node.into();
                Anchor::region(&element, dropdown_rect.get())
            })
            .unwrap_or(Anchor::Centred)
    });
    let menu_items = Signal::derive(move || {
        vec![
            MenuItem::new("0", state.tr("Overview", "概览")),
            MenuItem::new("1", state.tr("Usage", "用量")),
            MenuItem::new("2", state.tr("Settings", "设置")),
        ]
    });
    let props = Signal::derive(move || {
        state.frame.with(|frame| GpuPreviewProps {
            theme: frame.main.clone(),
            revision: frame.revision,
            scene: scene.get(),
            controls: state.controls.get(),
            scroll_y: 0.0,
        })
    });
    let action = move |action| match action {
        PreviewAction::Drawn { revision, samples } => {
            state.drawn.set(revision);
            state
                .samples
                .set(samples.iter().map(|sample| sample.to_json()).collect());
            state.timings.update(|values| {
                if let Some(value) = values.iter_mut().find(|value| value.0 == revision) {
                    if value.3 == 0. {
                        value.3 = now();
                    }
                }
            });
        }
        PreviewAction::Interacted { control, value } => {
            if let ("dropdown", PreviewValue::Open(rect)) = (control.as_str(), &value) {
                dropdown_rect.set(*rect);
                dropdown.set(true);
                return;
            }
            state
                .controls
                .update(|controls| match (control.as_str(), value) {
                    ("button", PreviewValue::Clicked) => controls.clicks += 1,
                    ("checkbox", PreviewValue::Boolean(value)) => controls.checked = value,
                    ("switch", PreviewValue::Boolean(value)) => controls.toggled = value,
                    ("slider", PreviewValue::Number(value)) => controls.value = value,
                    ("radio" | "tabs", PreviewValue::Index(value)) => controls.index = value,
                    _ => {}
                });
        }
    };
    Effect::new(move || {
        let width = state.envelope.with(|e| e.preferences.preview_width.clone());
        if let Some(node) = root.get() {
            let html: leptos::web_sys::HtmlElement = node.into();
            if let Some(pixels) = match width.as_str() {
                "desktop" => Some("1024px"),
                "tablet" => Some("768px"),
                "mobile" => Some("390px"),
                _ => None,
            } {
                let _ = html.style().set_property("--preview-width", pixels);
            } else {
                let _ = html.style().remove_property("--preview-width");
            }
        }
    });
    Effect::new(move || {
        let active = fullscreen.get();
        if let Ok(Some(previews)) = document().query_selector(".studio-previews") {
            let _ =
                previews.set_attribute("data-fullscreen", if active { "true" } else { "false" });
        }
        if active {
            previous_focus.set_value(document().active_element());
            if let Some(node) = root.get() {
                let html: leptos::web_sys::HtmlElement = node.into();
                let _ = html.focus();
            }
        } else if let Some(element) = previous_focus.get_value() {
            if element.is_connected() {
                if let Some(html) = element.dyn_ref::<leptos::web_sys::HtmlElement>() {
                    let _ = html.focus();
                }
            }
            previous_focus.set_value(None);
        }
    });
    Effect::new(move || {
        state.reset_epoch.get();
        dropdown.set(false);
    });
    view! {
        <ThemeScope resolved=resolved />
        <div class="studio-preview-root" node_ref=root tabindex="-1" data-testid="preview-workbench" data-width=move || state.envelope.with(|e| e.preferences.preview_width.clone()) data-inspecting=move || state.inspector.get().to_string()>
            <ThemeBoundary patch=Signal::derive(neutral_patch)>
                <PreviewToolbar state=state fullscreen=fullscreen />
                <Inspector state=state root=root canvas=canvas fullscreen=fullscreen />
            </ThemeBoundary>
            <div class="studio-renderers" data-renderer=move || state.envelope.with(|e| e.preferences.renderer.clone())>
                <section class="studio-renderer-pane studio-dom-pane" hidden=move || state.envelope.with(|e| e.preferences.renderer == "gpu") data-testid="dom-preview">
                    <p class="studio-renderer-label">"DOM"</p><ScenePreview scene=scene locale=state.locale() />
                    <details class="studio-provider-fixture" open><summary>{move || state.tr("Provider & overlay checks", "主题作用域与浮层验证")}</summary><ProviderFixture patch=state.patch /></details>
                </section>
                <section class="studio-renderer-pane studio-gpu-pane" hidden=move || state.envelope.with(|e| e.preferences.renderer == "dom") data-testid="gpu-preview">
                    <p class="studio-renderer-label">"GPU"</p><GpuPreview props=props locale=state.locale() on_action=action state=state.gpu_state node_ref=canvas />
                    <Menu open=dropdown on_open_change=move |v| dropdown.set(v) anchor=anchor items=menu_items on_activate=move |id| { if let Ok(index) = id.parse::<usize>() { state.controls.update(|controls| controls.index = index); } } aria_label="GPU dropdown" test_id="gpu-dropdown-menu" />
                </section>
            </div>
        </div>
    }.into_any()
}

struct Studio {
    handles: Vec<AppHandle>,
    owner: Owner,
    state: StudioState,
}
impl Drop for Studio {
    fn drop(&mut self) {
        self.handles.clear();
        self.owner.cleanup();
    }
}
thread_local! {
    static STUDIOS: RefCell<BTreeMap<u32, Studio>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT: RefCell<u32> = const { RefCell::new(1) };
}

#[wasm_bindgen]
pub fn theme_studio_mount(
    editor: &str,
    preview: &str,
    compare: &str,
    rem: f64,
    reduce_motion: bool,
) -> Result<u32, JsValue> {
    let owner = Owner::new();
    let state = owner.with(|| StudioState::new(rem, reduce_motion));
    let mut handles = Vec::new();
    for (id, kind) in [(editor, 0), (preview, 1), (compare, 2)] {
        let root = document()
            .get_element_by_id(id)
            .ok_or_else(|| JsValue::from_str("missing studio container"))?
            .dyn_into::<leptos::web_sys::HtmlElement>()?;
        let handle = mount(
            root,
            MountConfig {
                scope: format!("theme-studio-{kind}"),
                ..Default::default()
            },
            move || match kind {
                0 => view! { <Editor state=state /> }.into_any(),
                _ => view! { <Preview state=state compare=kind==2 /> }.into_any(),
            },
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        handles.push(handle);
    }
    owner.with(|| state.connect());
    let id = NEXT.with(|next| {
        let mut next = next.borrow_mut();
        let id = *next;
        *next += 1;
        id
    });
    STUDIOS.with(|studios| {
        studios.borrow_mut().insert(
            id,
            Studio {
                handles,
                owner,
                state,
            },
        )
    });
    Ok(id)
}
#[wasm_bindgen]
pub fn theme_studio_dispose(id: u32) -> bool {
    STUDIOS
        .with(|studios| studios.borrow_mut().remove(&id))
        .is_some()
}
#[wasm_bindgen]
pub fn theme_studio_reset(id: u32) -> bool {
    STUDIOS.with(|studios| {
        studios.borrow().get(&id).is_some_and(|studio| {
            studio.state.reset_fixture();
            true
        })
    })
}
#[wasm_bindgen]
pub fn theme_studio_edit(id: u32, token: &str, value: String) -> bool {
    STUDIOS.with(|studios| {
        studios
            .borrow()
            .get(&id)
            .is_some_and(|studio| studio.state.edit(token, value))
    })
}
#[wasm_bindgen]
pub fn theme_studio_environment(id: u32, rem: f64, reduce_motion: bool) {
    STUDIOS.with(|studios| {
        if let Some(studio) = studios.borrow().get(&id) {
            let state = studio.state;
            let changed = (rem.is_finite() && rem > 0. && state.rem.get_untracked() != rem)
                || state.reduce_motion.get_untracked() != reduce_motion;
            if rem.is_finite() && rem > 0. {
                state.rem.set(rem);
            }
            state.reduce_motion.set(reduce_motion);
            if changed {
                state.last_input_ms.set(now());
                state.request.update(|revision| *revision += 1);
            }
        }
    });
}
#[wasm_bindgen]
pub fn theme_studio_snapshot(id: u32) -> String {
    STUDIOS.with(|studios| studios.borrow().get(&id).map(|studio| {
        let state = studio.state; let editor = state.editor.get_untracked(); let controls = state.controls.get_untracked();
        serde_json::json!({ "document": editor.document, "mode": editor.mode, "document_revision": editor.revision,
            "history": {"undo": editor.undo_len(), "redo": editor.redo_len(), "gesture": editor.gesture_active()}, "modified": editor.is_modified(), "resolve_count": editor.resolutions(),
            "sdk_resolve_count": rustify_ui::theme::resolution_count(), "publish_count": state.publish_count.get_untracked(), "error": state.error.get_untracked(), "storage_status": state.storage_status.get_untracked(), "preferences": state.envelope.get_untracked().preferences,
            "region_count": rustify_makepad::live_region_count(), "revision": state.frame.get_untracked().revision, "drawn_revision": state.drawn.get_untracked(), "samples": state.samples.get_untracked(), "timings": state.timings.get_untracked(), "gpu_state": format!("{:?}", state.gpu_state.get_untracked()),
            "controls": {"checked": controls.checked, "toggled": controls.toggled, "value": controls.value, "index": controls.index, "clicks": controls.clicks, "disabled": controls.disabled, "read_only": controls.read_only}
        }).to_string()
    }).unwrap_or_else(|| "null".into()))
}
#[wasm_bindgen]
pub fn theme_studio_identify(runtime: u32, build: &str) {
    rustify_ui::identify_runtime(runtime, build);
}
#[wasm_bindgen]
pub fn theme_studio_diagnostics() -> String {
    rustify_ui::report_json()
}
