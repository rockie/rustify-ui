use crate::preview_region::{preview_height, PreviewAction, PreviewProps, PreviewRegion};
use leptos::prelude::*;
use rustify_ui::{GpuRegion, RegionState};
use std::marker::PhantomData;

#[component]
pub fn GpuPreview(
    props: Signal<PreviewProps>,
    locale: Signal<rustify_ui::Locale>,
    on_action: impl Fn(PreviewAction) + Clone + Send + Sync + 'static,
    state: RwSignal<RegionState>,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Canvas>>,
) -> impl IntoView {
    let tr = move |en: &'static str, zh: &'static str| {
        if locale.get() == rustify_ui::Locale::Chinese {
            zh
        } else {
            en
        }
    };
    let canvas = node_ref.unwrap_or_else(NodeRef::new);
    let root = NodeRef::<leptos::html::Div>::new();
    let width = RwSignal::new(760.0f64);
    let content = NodeRef::<leptos::html::Div>::new();
    let surface = NodeRef::<leptos::html::Div>::new();
    let scroll_y = RwSignal::new(0.0f64);
    let projected = Signal::derive(move || PreviewProps {
        scroll_y: scroll_y.get(),
        ..props.get()
    });
    let observation = StoredValue::new_local(None::<rustify_ui::ResizeObservation>);
    Effect::new(move || {
        let Some(node) = root.get() else { return };
        if observation.with_value(|held| held.is_some()) {
            return;
        }
        let element: leptos::web_sys::Element = node.into();
        let measured = element.clone();
        let resize = rustify_ui::observe_resize(&element, move || {
            let next = measured.client_width() as f64;
            if next > 0.0 && width.get_untracked() != next {
                width.set(next)
            }
        });
        observation.set_value(resize);
    });
    Effect::new(move || {
        let theme = props.get().theme;
        let height = preview_height(&theme, width.get());
        for (node, value) in [
            (root, height.min(4096.0) + 2.0),
            (content, height),
            (surface, height.min(4096.0)),
        ] {
            if let Some(node) = node.get() {
                let element: leptos::web_sys::HtmlElement = node.into();
                let _ = element
                    .style()
                    .set_property("height", &format!("{value}px"));
            }
        }
        if let Some(node) = root.get() {
            node.set_scroll_top(scroll_y.get_untracked().min((height - 4096.0).max(0.0)));
            scroll_y.set(node.scroll_top());
        }
    });
    on_cleanup(move || {
        observation.try_update_value(Option::take);
    });
    view! {
        <section class="gpu-preview-section">
            <h2>{move || tr("GPU theme samples", "GPU 主题样本")}</h2>
            <p class="gpu-state" data-testid="gpu-state">{move || match state.get(){RegionState::Starting=>tr("Starting GPU…","正在启动 GPU…"),RegionState::Ready=>tr("Ready","已就绪"),RegionState::Suspended=>tr("Preview hidden","预览已隐藏"),RegionState::Lost=>tr("Restoring GPU…","正在恢复 GPU…"),RegionState::Failed(_)=>tr("GPU unavailable. DOM preview remains available.","GPU 不可用，仍可使用 DOM 预览。"),RegionState::Disposed=>tr("GPU disposed","GPU 已释放")}}</p>
            <Show when=move || {rustify_ui::asset_failures() > 0} fallback=|| ()>
                <p class="gpu-state" role="status" data-testid="gpu-asset-warning">{move || tr("A preview resource failed to load. Font samples may use fallback glyphs. Reload to retry; your saved draft stays on this device.","预览资源加载失败，字体样本可能使用回退字形。刷新页面可重试；已保存草稿仍保留在本机。")}</p>
            </Show>
            <div class="gpu-preview-board" node_ref=root data-testid="gpu-board" tabindex="0" role="region" aria-label=move || tr("Scrollable GPU samples", "可滚动 GPU 样本")
                on:scroll=move |_| { if let Some(node) = root.get() { scroll_y.set(node.scroll_top()); } }>
                <div node_ref=content><div class="gpu-preview-surface" node_ref=surface>
                    <GpuRegion app={PhantomData::<PreviewRegion>} props=projected on_action=on_action state=state node_ref=canvas class="gpu-region" test_id="theme-gpu" />
                </div></div>
            </div>
        </section>
    }
}
