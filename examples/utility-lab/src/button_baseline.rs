//! On-demand ordinary Button versus utility-clear rendering fixture.
use leptos::{prelude::*, wasm_bindgen::prelude::*};
use rustify_style::program::{GpuDeclaration, GpuProgram, GpuValue};
use rustify_ui::gpu::{utility::ButtonUtilityProps, RustifyButton};
use rustify_ui::makepad_widgets::{widget::*, *};
use rustify_ui::theme::{
    font_context, resolve, ResolvedTheme, ThemeDocument, ThemeMode, ThemeValuePatch,
};
use rustify_ui::utility::gpu::{GpuComputedStyle, GpuResolveContext};
use rustify_ui::{mount, AppHandle, GpuRegion, MountConfig, Pace, RegionApp, Theme};
use serde_json::{json, Value};
use std::{cell::RefCell, collections::BTreeMap, marker::PhantomData, sync::Arc};

#[derive(Clone, Debug, PartialEq)]
pub struct Props {
    generation: u32,
    phase: u32,
    themed: bool,
    palette: bool,
    disabled: bool,
    theme: Arc<ResolvedTheme>,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    startup() do #(ButtonBaselineRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass +: {clear_color: #ffffff}
                body +: {
                    flow: Down
                    padding: 12
                    spacing: 12
                    ordinary := mod.widgets.RustifyButton{width: 220 height: 36}
                    cleared := mod.widgets.RustifyButton{width: 220 height: 36}
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct ButtonBaselineRegion {
    #[live]
    ui: WidgetRef,
    #[rust]
    props: Option<Props>,
    #[rust]
    next_frame: Option<NextFrame>,
    #[rust]
    drawn_report: Option<Value>,
    #[rust]
    clicks: [u32; 2],
}

impl RegionApp for ButtonBaselineRegion {
    type Props = Props;
    type Action = Value;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        rustify_ui::gpu::utility::script_mod(vm);
        self::script_mod(vm)
    }

    fn pace(_: &Value) -> Pace {
        Pace::Continuous("button-baseline-report")
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &Props) {
        for (index, path) in [ids!(ordinary), ids!(cleared)].into_iter().enumerate() {
            if let Some(mut button) = self.ui.widget(cx, path).borrow_mut::<RustifyButton>() {
                // Phase 4 must exercise clear without refreshing its saved baseline.
                if props.phase != 4 {
                    if props.themed {
                        button.apply_theme(cx, &props.theme);
                    }
                    button.set_state(cx, "Clear utility baseline", props.disabled);
                    if props.palette {
                        button.set_palette(cx, 0x123456, 0xabcdef, 0x987654, 0xffffff);
                    }
                }
                if index == 1 {
                    if props.phase == 1 {
                        let declarations = [
                            ("background-color", "#fedcba"),
                            ("color", "#123456"),
                            ("font-family", "'Liberation Mono'"),
                            ("font-size", "24px"),
                            ("letter-spacing", "0.08em"),
                            ("line-height", "32px"),
                            ("border-radius", "12px"),
                            ("height", "64px"),
                        ]
                        .into_iter()
                        .map(|(property, value)| GpuDeclaration {
                            property: property.into(),
                            value: GpuValue::CssExpression {
                                value: value.into(),
                            },
                            important: false,
                        })
                        .collect();
                        button.apply_utility_programs(
                            cx,
                            &ButtonUtilityProps {
                                revision: 1,
                                theme: props.theme.clone(),
                                programs: Arc::from([GpuProgram {
                                    candidate: "baseline-fixture".into(),
                                    conditions: vec![],
                                    cascade_order: 0,
                                    declarations,
                                }]),
                                context: GpuResolveContext::new(GpuComputedStyle::default()),
                            },
                        );
                    } else if props.phase == 2 || props.phase == 4 {
                        button.clear_utility(cx);
                    }
                }
            }
        }
        self.props = Some(props.clone());
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<Value>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        for (index, path) in [ids!(ordinary), ids!(cleared)].into_iter().enumerate() {
            if let Some(mut button) = self.ui.widget(cx, path).borrow_mut::<RustifyButton>() {
                if button.take_click().is_some() {
                    self.clicks[index] += 1;
                }
            }
        }
        if matches!(event, Event::Draw(_)) {
            // Freeze geometry and generation together; props may change before NextFrame.
            let buttons: Vec<_> = [ids!(ordinary), ids!(cleared)].into_iter().map(|path| {
                self.ui.widget(cx, path).borrow::<RustifyButton>().map(|button| {
                    let geometry = button.drawn(cx).map(|rect| [rect.pos.x, rect.pos.y, rect.size.x, rect.size.y]);
                    let utility = button.utility_report().map(|report| json!({"status": format!("{:?}", report.status), "drawn_revision": report.drawn_revision}));
                    json!({"geometry": geometry, "utility": utility})
                })
            }).collect();
            let baseline = self.props.as_ref().map(|props| json!({
                "themed": props.themed, "palette": props.palette, "disabled": props.disabled,
                "mode": props.theme.mode, "rem": props.theme.rem_px,
                "font_size": props.theme.font_size_px, "tracking": props.theme.letter_spacing_em,
            }));
            self.drawn_report = Some(
                json!({"generation": self.props.as_ref().map(|props| props.generation), "phase": self.props.as_ref().map(|props| props.phase), "baseline": baseline, "buttons": buttons, "clicks": self.clicks}),
            );
            self.next_frame = Some(cx.new_next_frame());
        } else if self
            .next_frame
            .is_some_and(|next| next.is_event(event).is_some())
        {
            self.next_frame = None;
            if let Some(report) = self.drawn_report.take() {
                outbox.push(report);
            }
        }
    }
}

struct Live {
    mount: AppHandle,
    props: RwSignal<Props>,
    report: RwSignal<Option<Value>>,
}
thread_local! { static LIVE: RefCell<Option<Live>> = const { RefCell::new(None) }; }

fn theme(dark: bool, rem: f64) -> Result<Arc<ResolvedTheme>, JsValue> {
    resolve(
        &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
        if dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        },
        &font_context(rem, false),
    )
    .map(Arc::new)
    .map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn utility_lab_button_baseline_mount(
    id: &str,
    base: &str,
    themed: Option<bool>,
) -> Result<(), JsValue> {
    utility_lab_button_baseline_dispose();
    let container = document()
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("missing baseline container"))?
        .unchecked_into();
    let initial = Props {
        generation: 0,
        phase: 0,
        themed: themed.unwrap_or(true),
        palette: false,
        disabled: false,
        theme: theme(false, 16.)?,
    };
    let props = RwSignal::new(initial);
    let report = RwSignal::new(None);
    let handle = mount(container, MountConfig {scope: id.into(), base: base.into(), url_owner: false}, move || view! {
        <GpuRegion app={PhantomData::<ButtonBaselineRegion>} props={Signal::derive(move || props.get())}
            test_id="button-baseline" on_action=move |value| report.set(Some(value)) />
    }).map_err(|error| JsValue::from_str(&error.to_string()))?;
    LIVE.with(|live| {
        *live.borrow_mut() = Some(Live {
            mount: handle,
            props,
            report,
        })
    });
    Ok(())
}

#[wasm_bindgen]
pub fn utility_lab_button_baseline_update(
    phase: u32,
    dark: bool,
    rem: f64,
    palette: bool,
    disabled: bool,
) -> Result<u32, JsValue> {
    LIVE.with(|live| {
        let live = live.borrow();
        let live = live
            .as_ref()
            .ok_or_else(|| JsValue::from_str("baseline is not mounted"))?;
        let mut next = live.props.get_untracked();
        next.generation = next
            .generation
            .checked_add(1)
            .ok_or_else(|| JsValue::from_str("baseline generation exhausted"))?;
        next.phase = phase;
        if phase != 4 {
            let theme = theme(dark, rem)?;
            next.theme = if phase == 3 {
                Arc::new(
                    theme
                        .patched(&ThemeValuePatch(BTreeMap::from([
                            ("font-size".into(), "21px".into()),
                            ("letter-spacing".into(), "0.04em".into()),
                        ])))
                        .map_err(|error| JsValue::from_str(&error.to_string()))?,
                )
            } else {
                theme
            };
            next.palette = palette;
            next.disabled = disabled;
        }
        let generation = next.generation;
        live.props.set(next);
        Ok(generation)
    })
}

#[wasm_bindgen]
pub fn utility_lab_button_baseline_report() -> String {
    LIVE.with(|live| {
        serde_json::to_string(
            &live
                .borrow()
                .as_ref()
                .and_then(|live| live.report.get_untracked()),
        )
        .unwrap_or_default()
    })
}

#[wasm_bindgen]
pub fn utility_lab_button_baseline_dispose() {
    LIVE.with(|live| {
        if let Some(mut live) = live.borrow_mut().take() {
            live.mount.dispose();
            live.props.dispose();
            live.report.dispose();
        }
    });
}
