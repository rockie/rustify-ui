#[cfg(target_arch = "wasm32")]
mod preview_region;

#[cfg(target_arch = "wasm32")]
mod component_baseline;

#[cfg(target_arch = "wasm32")]
mod app {
    use crate::preview_region::{UtilityLabAction, UtilityLabProps, UtilityLabRegion};
    use leptos::portal::Portal;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::{prelude::*, JsCast};
    use rustify_style::protocol::{CompileReport, CompilerInput, Limits};
    use rustify_style::text::Text;
    use rustify_ui::gpu::utility::{UtilityNodeKind, UtilityNodeProps, UtilityPreviewProps};
    use rustify_ui::theme::{
        font_context, resolve, use_resolved_theme, ResolvedTheme, Rgba, ThemeBoundary,
        ThemeDocument, ThemeMode, ThemeScope, ThemeValuePatch,
    };
    use rustify_ui::utility::gpu::{
        GpuApplyPolicy, GpuComputedStyle, GpuControlState, GpuLength, GpuResolveContext,
        InputModality,
    };
    use rustify_ui::utility::{SourceContent, SourceSnapshot, UtilityConfig, UtilityScope};
    use rustify_ui::{
        mount, AppHandle, GpuRegion, Locale, MountConfig, RegionState, ScopeRoots, Theme,
    };
    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use std::sync::Arc;

    #[derive(Clone, Copy)]
    struct GpuLive {
        props: RwSignal<UtilityLabProps>,
        report: RwSignal<Option<serde_json::Value>>,
        clicks: RwSignal<u64>,
        theme: RwSignal<Arc<ResolvedTheme>>,
        patch: RwSignal<ThemeValuePatch>,
        locale: RwSignal<Locale>,
    }

    struct Live {
        mount: AppHandle,
        utility: UtilityScope,
        gpu: Option<GpuLive>,
    }
    thread_local! { static SCOPES: RefCell<BTreeMap<String, Live>> = const { RefCell::new(BTreeMap::new()) }; }

    fn error(error: impl std::fmt::Display) -> JsValue {
        JsValue::from_str(&error.to_string())
    }
    fn scope(id: &str) -> Result<UtilityScope, JsValue> {
        SCOPES.with(|scopes| {
            scopes
                .borrow()
                .get(id)
                .map(|live| live.utility.clone())
                .ok_or_else(|| error("utility scope is not mounted"))
        })
    }

    fn gpu(id: &str) -> Result<GpuLive, JsValue> {
        SCOPES.with(|scopes| {
            scopes
                .borrow()
                .get(id)
                .and_then(|live| live.gpu)
                .ok_or_else(|| error("GPU preview is not mounted"))
        })
    }

    fn preview_props() -> Result<UtilityLabProps, JsValue> {
        let theme = Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                ThemeMode::Light,
                &font_context(16., false),
            )
            .map_err(error)?,
        );
        let node = |kind, label: &str| {
            let mut base = GpuComputedStyle::from_theme(&theme);
            base.background = Rgba {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            };
            base.radii = [0.; 4];
            base.font_size_px = 14.;
            base.font_size_points = rustify_ui::theme::css_px_to_points(14.);
            if kind == UtilityNodeKind::Container {
                base.padding = [24.; 4];
                base.gap = [12.; 2];
            }
            if kind == UtilityNodeKind::ColorBlock {
                base.width = GpuLength::Pixels(40.);
                base.height = GpuLength::Pixels(40.);
                base.background = Rgba {
                    r: 21. / 255.,
                    g: 112. / 255.,
                    b: 239. / 255.,
                    a: 1.,
                };
            }
            UtilityNodeProps {
                revision: 0,
                programs: Arc::from([]),
                theme: theme.clone(),
                context: GpuResolveContext::new(base),
                label: label.into(),
                kind,
            }
        };
        Ok(UtilityLabProps {
            preview: UtilityPreviewProps {
                container: node(UtilityNodeKind::Container, ""),
                button: node(UtilityNodeKind::Button, "Live utility sample"),
                text: node(UtilityNodeKind::Text, "Utility text"),
                swatch: node(UtilityNodeKind::ColorBlock, ""),
            },
            button_state: None,
        })
    }

    fn node_label(key: &str, zh: bool) -> &'static str {
        match (key, zh) {
            ("container", false) => "Container",
            ("container", true) => "容器",
            ("button", false) => "Button",
            ("button", true) => "按钮",
            ("text", false) => "Text",
            ("text", true) => "文字",
            (_, false) => "Color block",
            (_, true) => "色块",
        }
    }
    fn status_word(status: Option<&str>, zh: bool) -> &'static str {
        match (status, zh) {
            (Some("applied"), false) => "applied",
            (Some("applied"), true) => "已应用",
            (Some("partial"), false) => "partial",
            (Some("partial"), true) => "部分应用",
            (Some("rejected"), false) => "rejected",
            (Some("rejected"), true) => "已拒绝",
            (_, false) => "pending",
            (_, true) => "等待中",
        }
    }

    #[component]
    fn GpuPreview(live: GpuLive, target: leptos::web_sys::Element) -> impl IntoView {
        let effective = use_resolved_theme().expect("GPU preview requires its ThemeBoundary");
        let props = Signal::derive(move || {
            let theme = effective.get();
            let mut props = live.props.get();
            for node in [
                &mut props.preview.container,
                &mut props.preview.button,
                &mut props.preview.text,
                &mut props.preview.swatch,
            ] {
                node.theme = theme.clone();
                node.context.rem_px = theme.rem_px;
                if let Some(viewport) = leptos::web_sys::window().and_then(|window| {
                    Some([
                        window.inner_width().ok()?.as_f64()?,
                        window.inner_height().ok()?.as_f64()?,
                    ])
                }) {
                    node.context.viewport = viewport;
                }
            }
            props
        });
        let state = RwSignal::new(RegionState::Starting);
        view! { <Portal mount=target>
            <GpuRegion app={PhantomData::<UtilityLabRegion>} props=props
                state=state class="utility-gpu" test_id="utility-gpu"
                on_action=move |action| { match action {
                    UtilityLabAction::Report(report) => live.report.set(Some(report)),
                    UtilityLabAction::ButtonClick(report) => { live.report.set(Some(report)); live.clicks.update(|count| *count += 1); }
                } } />
            <p id="gpu-status" role="status" aria-live="polite">{move || {
                let zh = live.locale.get() == Locale::Chinese;
                match state.get() {
                    RegionState::Starting => if zh { "正在启动 GPU…" } else { "Starting GPU…" }.into(),
                    RegionState::Ready => if zh { "GPU 就绪" } else { "GPU ready" }.into(),
                    RegionState::Suspended => if zh { "GPU 预览已隐藏" } else { "GPU preview hidden" }.into(),
                    RegionState::Lost => if zh { "正在恢复 GPU…" } else { "Restoring GPU…" }.into(),
                    RegionState::Failed(reason) => if zh {
                        format!("GPU 不可用：{reason}。DOM 编辑仍可使用。请先复制类名输入，再重新加载页面重试 GPU。")
                    } else {
                        format!("GPU unavailable: {reason}. DOM editing remains available. Copy your class inputs, then reload the page to retry GPU.")
                    },
                    RegionState::Disposed => if zh { "GPU 已释放" } else { "GPU disposed" }.into(),
                }
            }}</p>
            <p id="gpu-revision">{move || {
                let zh = live.locale.get() == Locale::Chinese;
                live.report.get().map_or_else(|| if zh { "正在等待第一帧…" } else { "Waiting for the first frame…" }.into(), |report| {
                    let button = &report["button"];
                    let status = status_word(button["status"].as_str(), zh);
                    if zh {
                        format!("按钮：{status} · 已应用 {} · 已绘制 {} · 点击 {} 次", button["applied_revision"], button["drawn_revision"], live.clicks.get())
                    } else {
                        format!("Button: {status} · applied {} · drawn {} · {} clicks", button["applied_revision"], button["drawn_revision"], live.clicks.get())
                    }
                })
            }}</p>
            <p id="gpu-results">{move || {
                let zh = live.locale.get() == Locale::Chinese;
                live.report.get().map_or_else(String::new, |report| {
                    ["container", "text", "swatch"].iter()
                        .map(|key| format!("{}{} {}", node_label(key, zh), if zh { "：" } else { ":" }, status_word(report[key]["status"].as_str(), zh)))
                        .collect::<Vec<_>>().join(" · ")
                })
            }}</p>
            <p id="gpu-warning" role="status" aria-live="polite">{move || {
                let zh = live.locale.get() == Locale::Chinese;
                live.report.get().map_or_else(String::new, |report| {
                    ["container", "button", "text", "swatch"].iter()
                        .filter_map(|key| report[key]["diagnostics"].as_array().and_then(|items| items.first()).map(|item| {
                            let (label, status) = (node_label(key, zh), status_word(report[key]["status"].as_str(), zh));
                            // The reason is the resolver's diagnostic text, which the SDK reports in English.
                            let (candidate, property, reason) = (item["candidate"].as_str().unwrap_or(if zh { "宿主上下文" } else { "Host context" }), item["property"].as_str().unwrap_or("utility"), item["reason"].as_str().unwrap_or(if zh { "不支持的属性" } else { "Unsupported property" }));
                            if zh {
                                format!("{label}{status}：{candidate}（{property}）— {reason}。请移除该类名或选择仅应用支持的属性。详情见下方。")
                            } else {
                                format!("{label} {status}: {candidate} ({property}) — {reason}. Remove the class or choose supported-only application. More details below.")
                            }
                        })).collect::<Vec<_>>().join("\n")
                })
            }}</p>
            <details><summary>{move || if live.locale.get() == Locale::Chinese { "GPU 数值与诊断" } else { "GPU values and diagnostics" }}</summary><pre id="gpu-report">{move || live.report.get().map_or_else(String::new, |report| serde_json::to_string_pretty(&report).unwrap_or_default())}</pre></details>
        </Portal> }
    }

    #[wasm_bindgen]
    pub fn utility_lab_mount(
        id: &str,
        base: &str,
        timeout: u32,
        css: Option<String>,
        limits: Option<String>,
    ) -> Result<(), JsValue> {
        let container = document()
            .get_element_by_id(id)
            .ok_or_else(|| error("missing utility container"))?
            .dyn_into()
            .map_err(|_| error("utility container is not HTML"))?;
        let config = UtilityConfig {
            input: CompilerInput {
                css: css.unwrap_or_else(|| "@tailwind utilities;".into()).into(),
                entry: "memory:input.css".into(),
            },
            assets: vec![],
            request_timeout_ms: timeout,
            limits: match limits {
                Some(limits) => serde_json::from_str(&limits).map_err(error)?,
                None => Limits {
                    input_bytes: 65536,
                    asset_bytes: 65536,
                    candidate_count: 1024,
                    ast_depth: 32,
                    css_bytes: 262144,
                    queued_bytes: 65536,
                    cache_bytes: 1048576,
                },
            },
        };
        let constructed = Rc::new(RefCell::new(None));
        let runtime = constructed.clone();
        let sample_id = format!("{id}-sample");
        let frame_id = format!("{id}-frame");
        let text_id = format!("{id}-text");
        let swatch_id = format!("{id}-swatch");
        let nested_id = format!("{id}-nested");
        let gpu_mount = (id == "scope-a")
            .then(|| document().get_element_by_id("gpu-board"))
            .flatten();
        let initial = gpu_mount.as_ref().map(|_| preview_props()).transpose()?;
        let mut handle = mount(container, MountConfig { scope: id.into(), url_owner: false, base: base.into() }, move || {
            let roots = expect_context::<ScopeRoots>();
            let gpu = initial.map(|props| GpuLive { theme: RwSignal::new(props.preview.button.theme.clone()),
                locale: RwSignal::new(Locale::English),
                patch: RwSignal::new(ThemeValuePatch::default()), props: RwSignal::new(props),
                report: RwSignal::new(None), clicks: RwSignal::new(0) });
            *runtime.borrow_mut() = Some((UtilityScope::new(&roots, config), gpu));
            let samples = move || view! { <div id=frame_id.clone() class="preview-content"><div id=sample_id.clone() class="sample" tabindex="0" role="button">"Live utility sample"</div><div id=text_id.clone() class="sample-text">"Utility text"</div><div id=swatch_id.clone() class="sample-swatch"></div><div id=nested_id.clone() class="nested-slot"></div></div> };
            if let Some((live, target)) = gpu.zip(gpu_mount) {
                let theme_scope = view! { <ThemeScope resolved=live.theme /> };
                view! { {theme_scope}<ThemeBoundary patch=live.patch test_id="utility-theme-boundary">
                    {samples}<GpuPreview live=live target=target />
                </ThemeBoundary> }.into_any()
            } else {
                samples().into_any()
            }
        }).map_err(error)?;
        let utility = constructed
            .borrow_mut()
            .take()
            .ok_or_else(|| error("utility scope did not mount"))?;
        let (utility, gpu) = utility;
        match utility {
            Ok(utility) => {
                SCOPES.with(|scopes| {
                    scopes.borrow_mut().insert(
                        id.into(),
                        Live {
                            mount: handle,
                            utility,
                            gpu,
                        },
                    )
                });
                Ok(())
            }
            Err(failure) => {
                handle.dispose();
                Err(error(failure))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn utility_lab_gpu_apply(
        id: String,
        classes: String,
        partial: bool,
        disabled: bool,
    ) -> Result<String, JsValue> {
        let classes: [Vec<String>; 4] = serde_json::from_str(&classes).map_err(error)?;
        let active: Vec<String> = classes.iter().flatten().cloned().collect();
        let snapshot = scope(&id)?.ensure_classes(&active).await.map_err(error)?;
        let report: &CompileReport = &snapshot.report;
        let active: BTreeSet<Text> = active.iter().map(Text::from).collect();
        if report
            .rejected
            .iter()
            .chain(&report.unsupported)
            .any(|item| active.contains(&item.candidate))
        {
            return serde_json::to_string(report).map_err(error);
        }
        let live = gpu(&id)?;
        let mut props = live.props.get_untracked();
        for (node, classes) in [
            &mut props.preview.container,
            &mut props.preview.button,
            &mut props.preview.text,
            &mut props.preview.swatch,
        ]
        .into_iter()
        .zip(classes)
        {
            let classes: BTreeSet<Text> = classes.iter().map(Text::from).collect();
            node.revision = report.revision;
            node.programs = report
                .gpu_programs
                .iter()
                .filter(|program| classes.contains(&program.candidate))
                .cloned()
                .collect();
            node.context.variables.clone_from(&report.gpu_variables);
            node.context.policy = if partial {
                GpuApplyPolicy::Partial
            } else {
                GpuApplyPolicy::Strict
            };
            node.context.state.disabled = disabled && node.kind == UtilityNodeKind::Button;
        }
        props.button_state = None;
        live.props.set(props);
        serde_json::to_string(report).map_err(error)
    }

    /// Publishes a validated host environment to the actual scope and boundary.
    #[wasm_bindgen]
    pub fn utility_lab_theme(id: &str, dark: bool, rem: f64, patch: &str) -> Result<(), JsValue> {
        if !rem.is_finite() || rem <= 0. {
            return Err(error("rem must be positive and finite"));
        }
        let patch = ThemeValuePatch(serde_json::from_str(patch).map_err(error)?);
        let theme = Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                if dark {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                },
                &font_context(rem, false),
            )
            .map_err(error)?,
        );
        theme.patched(&patch).map_err(error)?;
        let live = gpu(id)?;
        live.theme.set(theme);
        live.patch.set(patch);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn utility_lab_gpu_report(id: &str) -> Result<String, JsValue> {
        serde_json::to_string(&gpu(id)?.report.get_untracked()).map_err(error)
    }

    #[wasm_bindgen]
    pub fn utility_lab_gpu_viewport(id: &str, width: f64, height: f64) -> Result<(), JsValue> {
        let Some(live) = SCOPES.with(|scopes| scopes.borrow().get(id).and_then(|live| live.gpu))
        else {
            return Ok(());
        };
        live.props.update(|props| {
            for node in [
                &mut props.preview.container,
                &mut props.preview.button,
                &mut props.preview.text,
                &mut props.preview.swatch,
            ] {
                node.context.viewport = [width, height];
            }
        });
        Ok(())
    }

    #[wasm_bindgen]
    pub fn utility_lab_gpu_keyboard(
        id: &str,
        focused: bool,
        active: bool,
        clicked: bool,
    ) -> Result<(), JsValue> {
        let live = gpu(id)?;
        live.props.update(|props| {
            props.button_state = Some(GpuControlState {
                focused,
                focus_visible: focused,
                active,
                disabled: props.preview.button.context.state.disabled,
                modality: InputModality::Keyboard,
                ..GpuControlState::default()
            })
        });
        if clicked
            && !live
                .props
                .get_untracked()
                .preview
                .button
                .context
                .state
                .disabled
        {
            live.clicks.update(|count| *count += 1);
        }
        Ok(())
    }

    /// The page's language, for the strings the GPU preview renders.
    #[wasm_bindgen]
    pub fn utility_lab_locale(tag: &str) {
        let locale = Locale::from_tag(tag).unwrap_or_default();
        SCOPES.with(|scopes| {
            for live in scopes.borrow().values() {
                if let Some(gpu) = live.gpu {
                    gpu.locale.set(locale);
                }
            }
        });
    }
    /// Merges base, override and conditional fragments with the scope's
    /// committed cn, before any of them reaches the compiler.
    #[wasm_bindgen]
    pub fn utility_lab_merge(id: &str, fragments: &str) -> Result<String, JsValue> {
        let fragments: Vec<String> = serde_json::from_str(fragments).map_err(error)?;
        let fragments: Vec<&str> = fragments.iter().map(String::as_str).collect();
        scope(id)?.cn_context().merge(&fragments).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_ensure(id: String, classes: String) -> Result<String, JsValue> {
        let classes: Vec<String> = serde_json::from_str(&classes).map_err(error)?;
        let snapshot = scope(&id)?
            .ensure_classes(&classes)
            .await
            .map_err(|failure| {
                error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
            })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_scan_content(id: String, content: String) -> Result<String, JsValue> {
        let content: Vec<SourceContent> = serde_json::from_str(&content).map_err(error)?;
        let snapshot = scope(&id)?
            .scan_content(&content)
            .await
            .map_err(|failure| {
                error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
            })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_scan_sources(id: String, sources: String) -> Result<String, JsValue> {
        let sources: SourceSnapshot = serde_json::from_str(&sources).map_err(error)?;
        let snapshot = scope(&id)?
            .scan_sources(&sources)
            .await
            .map_err(|failure| {
                error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
            })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_replace(id: String, css: String) -> Result<String, JsValue> {
        let scope = scope(&id)?;
        let mut config = scope.configuration();
        config.input.css = css.into();
        let snapshot = scope
            .replace_config(config, rustify_style::cn::Cn::default())
            .await
            .map_err(|failure| {
                error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
            })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_rebuild(id: String, active: String) -> Result<String, JsValue> {
        let active: Vec<String> = serde_json::from_str(&active).map_err(error)?;
        let snapshot = scope(&id)?.rebuild(&active).await.map_err(|failure| {
            error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
        })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_retry(id: String) -> Result<String, JsValue> {
        let snapshot = scope(&id)?.retry().await.map_err(|failure| {
            error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
        })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub async fn utility_lab_flush(id: String) -> Result<String, JsValue> {
        let snapshot = scope(&id)?.flush().await.map_err(|failure| {
            error(serde_json::to_string(&failure).unwrap_or_else(|_| failure.to_string()))
        })?;
        serde_json::to_string(&*snapshot.report).map_err(error)
    }
    #[wasm_bindgen]
    pub fn utility_lab_report(id: &str) -> Result<String, JsValue> {
        serde_json::to_string(
            &scope(id)?
                .snapshot()
                .map(|snapshot| snapshot.report.clone()),
        )
        .map_err(error)
    }
    /// How many compiler inputs the installed snapshot retains, the count
    /// `candidate_count` limits.
    #[wasm_bindgen]
    pub fn utility_lab_retained(id: &str) -> Result<usize, JsValue> {
        Ok(scope(id)?
            .snapshot()
            .map_or(0, |snapshot| snapshot.classes.len()))
    }
    #[wasm_bindgen]
    pub fn utility_lab_dispose(id: &str) -> bool {
        SCOPES.with(|scopes| {
            if let Some(mut live) = scopes.borrow_mut().remove(id) {
                live.utility.dispose();
                live.mount.dispose();
                true
            } else {
                false
            }
        })
    }
    #[wasm_bindgen]
    pub fn utility_lab_identify(runtime: u32, build: &str) {
        rustify_ui::identify_runtime(runtime, build);
    }
    #[wasm_bindgen]
    pub fn utility_lab_trap() {
        panic!("utility cleanup fixture");
    }
}
fn main() {}
