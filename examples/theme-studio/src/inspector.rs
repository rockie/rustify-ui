//! Picking is limited to declared samples in this workbench's preview.

fn viewport_rect(local: [f64; 4], canvas: [f64; 4], layout: [f64; 2]) -> Option<[f64; 4]> {
    if !local
        .iter()
        .chain(canvas.iter())
        .chain(layout.iter())
        .all(|v| v.is_finite())
        || layout[0] <= 0.0
        || layout[1] <= 0.0
        || canvas[2] <= 0.0
        || canvas[3] <= 0.0
        || local[2] <= 0.0
        || local[3] <= 0.0
    {
        return None;
    }
    let sx = canvas[2] / layout[0];
    let sy = canvas[3] / layout[1];
    Some([
        canvas[0] + local[0] * sx,
        canvas[1] + local[1] * sy,
        local[2] * sx,
        local[3] * sy,
    ])
}

fn intersect(a: [f64; 4], b: [f64; 4]) -> Option<[f64; 4]> {
    let left = a[0].max(b[0]);
    let top = a[1].max(b[1]);
    let right = (a[0] + a[2]).min(b[0] + b[2]);
    let bottom = (a[1] + a[3]).min(b[1] + b[3]);
    (right > left && bottom > top).then_some([left, top, right - left, bottom - top])
}

#[cfg(target_arch = "wasm32")]
pub use ui::Inspector;

#[cfg(target_arch = "wasm32")]
mod ui {
    use super::{intersect, viewport_rect};
    use crate::controller::StudioState;
    use leptos::{prelude::*, wasm_bindgen::JsCast};
    use rustify_components::{Button, ButtonSize, ButtonVariant};
    use rustify_ui::{
        theme::{COLOR_TOKENS, VALUE_TOKENS},
        ListenOptions,
    };
    use std::collections::BTreeSet;

    #[derive(Clone)]
    enum Target {
        Dom(leptos::web_sys::Element),
        Gpu(String),
    }

    fn tokens(raw: &str) -> Vec<String> {
        raw.split(|c: char| c == ',' || c.is_whitespace())
            .filter(|token| COLOR_TOKENS.contains(token) || VALUE_TOKENS.contains(token))
            .map(str::to_owned)
            .collect()
    }

    fn rectangle(value: &serde_json::Value, name: &str) -> Option<[f64; 4]> {
        let data = value.get(name)?.as_array()?;
        if data.len() != 4 {
            return None;
        }
        Some([
            data[0].as_f64()?,
            data[1].as_f64()?,
            data[2].as_f64()?,
            data[3].as_f64()?,
        ])
    }

    fn element_rect(element: &leptos::web_sys::Element) -> [f64; 4] {
        let rect = element.get_bounding_client_rect();
        [rect.x(), rect.y(), rect.width(), rect.height()]
    }

    fn clipped(mut bounds: [f64; 4], element: &leptos::web_sys::Element) -> Option<[f64; 4]> {
        let win = window();
        bounds = intersect(
            bounds,
            [
                0.,
                0.,
                win.inner_width().ok()?.as_f64()?,
                win.inner_height().ok()?.as_f64()?,
            ],
        )?;
        let mut ancestor = element.parent_element();
        while let Some(parent) = ancestor {
            if let Ok(Some(style)) = win.get_computed_style(&parent) {
                let x = style.get_property_value("overflow-x").unwrap_or_default();
                let y = style.get_property_value("overflow-y").unwrap_or_default();
                if [x.as_str(), y.as_str()]
                    .iter()
                    .any(|v| matches!(*v, "auto" | "scroll" | "hidden" | "clip"))
                {
                    bounds = intersect(bounds, element_rect(&parent))?;
                }
            }
            ancestor = parent.parent_element();
        }
        Some(bounds)
    }

    #[component]
    pub fn Inspector(
        state: StudioState,
        root: NodeRef<leptos::html::Div>,
        canvas: NodeRef<leptos::html::Canvas>,
        fullscreen: RwSignal<bool>,
    ) -> impl IntoView {
        let target = StoredValue::new_local(None::<Target>);
        let picked = RwSignal::new(Vec::<String>::new());
        let visible = RwSignal::new(false);
        let outline = NodeRef::<leptos::html::Div>::new();
        let update = move || {
            if !state.inspector.get_untracked() {
                visible.set(false);
                return;
            }
            let bounds = target.with_value(|target| match target {
                Some(Target::Dom(element)) if element.is_connected() => {
                    clipped(element_rect(element), element)
                }
                Some(Target::Gpu(id)) => {
                    let node: leptos::web_sys::Element = canvas.get_untracked()?.into();
                    let sample = state.samples.with_untracked(|samples| {
                        samples
                            .iter()
                            .find(|sample| sample["id"].as_str() == Some(id))
                            .cloned()
                    })?;
                    let local = rectangle(&sample, "visible_rect")?;
                    let bounds = viewport_rect(
                        local,
                        element_rect(&node),
                        [node.client_width() as f64, node.client_height() as f64],
                    )?;
                    clipped(bounds, &node)
                }
                _ => None,
            });
            visible.set(bounds.is_some());
            if let (Some(bounds), Some(node)) = (bounds, outline.get_untracked()) {
                let element: leptos::web_sys::HtmlElement = node.into();
                for (name, value) in [
                    ("--inspector-x", bounds[0]),
                    ("--inspector-y", bounds[1]),
                    ("--inspector-w", bounds[2]),
                    ("--inspector-h", bounds[3]),
                ] {
                    let _ = element.style().set_property(name, &format!("{value}px"));
                }
            }
        };
        let choose = move |event: &leptos::web_sys::MouseEvent| -> bool {
            let Some(root_node) = root.get_untracked() else {
                return false;
            };
            let root_element: leptos::web_sys::Element = root_node.into();
            let Some(element) = event
                .target()
                .and_then(|node| node.dyn_into::<leptos::web_sys::Element>().ok())
            else {
                return false;
            };
            if !root_element.contains(Some(&element)) {
                return false;
            }
            if element
                .closest("[data-inspector-ui]")
                .ok()
                .flatten()
                .is_some()
            {
                return false;
            }
            if let Some(canvas_node) = canvas.get_untracked() {
                let canvas_element: leptos::web_sys::Element = canvas_node.into();
                if element == canvas_element {
                    let bounds = element_rect(&canvas_element);
                    if bounds[2] <= 0. || bounds[3] <= 0. {
                        return false;
                    }
                    let x = (event.client_x() as f64 - bounds[0])
                        * canvas_element.client_width() as f64
                        / bounds[2];
                    let y = (event.client_y() as f64 - bounds[1])
                        * canvas_element.client_height() as f64
                        / bounds[3];
                    let sample = state.samples.with_untracked(|samples| {
                        samples
                            .iter()
                            .rev()
                            .find(|sample| {
                                rectangle(sample, "visible_rect").is_some_and(|rect| {
                                    x >= rect[0]
                                        && y >= rect[1]
                                        && x < rect[0] + rect[2]
                                        && y < rect[1] + rect[3]
                                })
                            })
                            .cloned()
                    });
                    if let Some(sample) = sample {
                        let ids = sample["token_ids"]
                            .as_array()
                            .map(|ids| {
                                ids.iter()
                                    .filter_map(|id| id.as_str().map(str::to_owned))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        if ids.is_empty() {
                            return false;
                        }
                        let Some(id) = sample["id"].as_str() else {
                            return false;
                        };
                        target.set_value(Some(Target::Gpu(id.into())));
                        picked.set(ids);
                        update();
                        return true;
                    }
                    return false;
                }
            }
            let Some(sample) = element
                .closest("[data-theme-tokens]")
                .ok()
                .flatten()
                .filter(|sample| root_element.contains(Some(sample)))
            else {
                return false;
            };
            let ids = tokens(
                &sample
                    .get_attribute("data-theme-tokens")
                    .unwrap_or_default(),
            );
            if ids.is_empty() {
                return false;
            }
            target.set_value(Some(Target::Dom(sample)));
            picked.set(ids);
            update();
            true
        };

        let listeners = StoredValue::new_local(None::<Vec<rustify_ui::Listener>>);
        Effect::new(move || {
            let Some(node) = root.get() else { return };
            if listeners.with_value(Option::is_some) {
                return;
            }
            let element: leptos::web_sys::Element = node.into();
            let event_target: leptos::web_sys::EventTarget = element.into();
            let move_listener = rustify_ui::listen(
                &event_target,
                "mousemove",
                ListenOptions {
                    capture: true,
                    passive: true,
                },
                move |event| {
                    if state.inspector.get_untracked() {
                        if let Some(event) = event.dyn_ref::<leptos::web_sys::MouseEvent>() {
                            choose(event);
                        }
                    }
                },
            );
            let click_listener = rustify_ui::listen(
                &event_target,
                "click",
                ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    if state.inspector.get_untracked() {
                        if let Some(mouse) = event.dyn_ref::<leptos::web_sys::MouseEvent>() {
                            if choose(mouse) {
                                event.prevent_default();
                                event.stop_immediate_propagation();
                                if let Some(token) = picked.get_untracked().first() {
                                    state.inspect(token.clone());
                                }
                            }
                        }
                    }
                },
            );
            let doc: leptos::web_sys::EventTarget = document().into();
            let scroll = rustify_ui::listen(
                &doc,
                "scroll",
                ListenOptions {
                    capture: true,
                    passive: true,
                },
                move |_| update(),
            );
            let win: leptos::web_sys::EventTarget = window().into();
            let resize = rustify_ui::listen(&win, "resize", Default::default(), move |_| update());
            let escape = rustify_ui::listen(
                &win,
                "keydown",
                ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    if let Some(key) = event.dyn_ref::<leptos::web_sys::KeyboardEvent>() {
                        if key.key() == "Escape"
                            && !key.is_composing()
                            && document()
                                .query_selector("[role='dialog'][aria-modal='true']")
                                .ok()
                                .flatten()
                                .is_none()
                        {
                            state.inspector.set(false);
                            fullscreen.set(false);
                            visible.set(false);
                        }
                    }
                },
            );
            listeners.set_value(Some(vec![
                move_listener,
                click_listener,
                scroll,
                resize,
                escape,
            ]));
        });
        let pending_layout = StoredValue::new_local(None::<rustify_ui::FrameHandle>);
        Effect::new(move || {
            state.frame.get();
            state.samples.get();
            state.envelope.with(|envelope| envelope.preferences.clone());
            state.inspector.get();
            picked.get();
            outline.get();
            if let Some(frame) = pending_layout.get_value() {
                frame.cancel();
            }
            // The selected-token list changes the preview's layout. Measure
            // after its DOM has committed, rather than the preceding event.
            pending_layout.set_value(Some(rustify_ui::next_frame(move || {
                pending_layout.update_value(|pending| {
                    pending.take();
                });
                update();
            })));
        });
        on_cleanup(move || {
            if let Some(frame) = pending_layout.get_value() {
                frame.cancel();
            }
            listeners.try_update_value(Option::take);
        });

        let available = Memo::new(move |_| {
            state.frame.get();
            state.samples.get();
            let mut ids = BTreeSet::new();
            if let Some(node) = root.get() {
                if let Ok(nodes) = node.query_selector_all("[data-theme-tokens]") {
                    for index in 0..nodes.length() {
                        if let Some(node) = nodes
                            .item(index)
                            .and_then(|node| node.dyn_into::<leptos::web_sys::Element>().ok())
                        {
                            ids.extend(tokens(
                                &node.get_attribute("data-theme-tokens").unwrap_or_default(),
                            ));
                        }
                    }
                }
            }
            state.samples.with(|samples| {
                for sample in samples {
                    if let Some(tokens) = sample["token_ids"].as_array() {
                        ids.extend(
                            tokens
                                .iter()
                                .filter_map(|token| token.as_str().map(str::to_owned)),
                        );
                    }
                }
            });
            ids.into_iter().collect::<Vec<_>>()
        });
        view! {
            <div class="inspector-outline" node_ref=outline hidden=move || !state.inspector.get() || !visible.get() data-testid="inspector-outline" aria-hidden="true">
                <span>{move || picked.get().join(", ")}</span>
            </div>
            <aside class="inspector-panel" data-inspector-ui="true" hidden=move || !state.inspector.get() data-testid="inspector-panel">
                <h3>{move ||state.tr("Inspect preview tokens","检查预览 token")}</h3>
                <p>{move ||state.tr("Point at a declared DOM or GPU sample, then choose a token. Esc exits.","指向已声明的 DOM 或 GPU 样本，再选择 token。按 Esc 退出。")}</p>
                <div class="inspector-tokens" role="group" aria-label="Sample tokens">
                    {move || picked.get().into_iter().map(|token|{let label=token.clone();view!{<Button variant=ButtonVariant::Outline size=ButtonSize::Sm on_click=move ||state.inspect(token.clone())>{label}</Button>}}).collect_view()}
                </div>
                <details><summary>{move ||state.tr("All drawn tokens · keyboard selection","全部绘制 token · 键盘选择")}</summary>
                    <div class="inspector-tokens">
                        <For each=move ||available.get() key=|token|token.clone() children=move |token|{let label=token.clone();let id=format!("inspect-token-{token}");view!{<button type="button" data-testid=id on:click=move |_|state.inspect(token.clone())>{label}</button>}} />
                    </div>
                </details>
            </aside>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{intersect, viewport_rect};

    #[test]
    fn css_scaling_uses_layout_units_without_double_applying_dpr() {
        assert_eq!(
            viewport_rect([20., 30., 40., 50.], [100., 200., 600., 300.], [300., 150.]),
            Some([140., 260., 80., 100.])
        );
    }
    #[test]
    fn clipped_samples_do_not_highlight_outside_the_scroll_viewport() {
        assert_eq!(
            intersect([10., 20., 100., 80.], [0., 0., 60., 50.]),
            Some([10., 20., 50., 30.])
        );
        assert_eq!(intersect([10., 20., 10., 10.], [20., 30., 5., 5.]), None);
    }
    #[test]
    fn hidden_canvas_geometry_is_not_an_inspection_target() {
        assert_eq!(
            viewport_rect([1., 2., 3., 4.], [0., 0., 0., 100.], [100., 100.]),
            None
        );
        assert_eq!(
            viewport_rect([1., 2., 3., 4.], [0., 0., 100., 100.], [0., 100.]),
            None
        );
    }
}
