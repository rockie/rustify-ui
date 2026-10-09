//! Application-owned region for the SDK utility preview and truthful draw reports.
#[path = "button_baseline.rs"]
mod button_baseline;
use rustify_ui::gpu::utility::{
    RustifyUtilityPreview, UtilityNodeReport, UtilityPreviewProps, UtilityPreviewReport,
};
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::utility::gpu::*;
use rustify_ui::{Pace, RegionApp};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum UtilityLabAction {
    Report(Value),
    ButtonClick(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UtilityLabProps {
    pub preview: UtilityPreviewProps,
    /// Explicit command from the DOM accessibility mirror; None leaves the
    /// Makepad pointer/keyboard state under the widget's own event handling.
    pub button_state: Option<GpuControlState>,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    startup() do #(UtilityLabRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass +: {clear_color: #ffffff}
                body +: {
                    padding: 0
                    preview := mod.widgets.RustifyUtilityPreview{width: Fill height: Fill}
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct UtilityLabRegion {
    #[live]
    ui: WidgetRef,
    #[rust]
    props: Option<UtilityLabProps>,
    #[rust]
    next_frame: Option<NextFrame>,
    #[rust]
    last_report: Option<Value>,
}

impl RegionApp for UtilityLabRegion {
    type Props = UtilityLabProps;
    type Action = UtilityLabAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        rustify_ui::gpu::utility::script_mod(vm);
        self::script_mod(vm)
    }

    fn pace(action: &UtilityLabAction) -> Pace {
        match action {
            UtilityLabAction::Report(_) => Pace::Continuous("utility-gpu-report"),
            UtilityLabAction::ButtonClick(_) => Pace::Discrete,
        }
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &UtilityLabProps) {
        if self.props.as_ref() == Some(props) {
            return;
        }
        let revisions = |props: &UtilityPreviewProps| {
            [
                props.container.revision,
                props.button.revision,
                props.text.revision,
                props.swatch.revision,
            ]
        };
        let new_revision = self
            .props
            .as_ref()
            .is_none_or(|previous| revisions(&previous.preview) != revisions(&props.preview));
        if let Some(mut preview) = self
            .ui
            .widget(cx, ids!(preview))
            .borrow_mut::<RustifyUtilityPreview>()
        {
            preview.apply_props(cx, &props.preview);
            if let Some(state) = props.button_state {
                preview.set_button_state(cx, state);
            }
        }
        self.props = Some(props.clone());
        if new_revision {
            // Equal numeric styles still need one physical draw to acknowledge
            // a new revision. The widget publishes its own drawn revision.
            self.ui.redraw(cx);
        }
        self.queue_changed_report(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<UtilityLabAction>) {
        if matches!(event, Event::Draw(_)) {
            // CSS media queries use the browser viewport, independently of
            // the smaller region/canvas used for drawing and parent geometry.
            let viewport = leptos::web_sys::window().and_then(|window| {
                Some([
                    window.inner_width().ok()?.as_f64()?,
                    window.inner_height().ok()?.as_f64()?,
                ])
            });
            if let Some(viewport) = viewport.filter(|size| {
                size.iter()
                    .all(|dimension| dimension.is_finite() && *dimension > 0.)
            }) {
                if let Some(mut preview) = self
                    .ui
                    .widget(cx, ids!(preview))
                    .borrow_mut::<RustifyUtilityPreview>()
                {
                    preview.update_viewport(cx, viewport);
                }
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if let Some(mut preview) = self
            .ui
            .widget(cx, ids!(preview))
            .borrow_mut::<RustifyUtilityPreview>()
        {
            if preview.take_click() {
                outbox.push(UtilityLabAction::ButtonClick(report_json(
                    &preview.report(),
                )));
            }
        }
        if matches!(event, Event::Draw(_)) {
            self.next_frame = Some(cx.new_next_frame());
        } else if self
            .next_frame
            .is_some_and(|next| next.is_event(event).is_some())
        {
            self.next_frame = None;
            if let Some(preview) = self
                .ui
                .widget(cx, ids!(preview))
                .borrow::<RustifyUtilityPreview>()
            {
                let report = report_json(&preview.report());
                if self.last_report.as_ref() != Some(&report) {
                    self.last_report = Some(report.clone());
                    outbox.push(UtilityLabAction::Report(report));
                }
            }
        } else {
            // State/diagnostics can change while numeric styles stay equal.
            // Publish the truthful previous drawn revision without demanding
            // a redundant physical redraw.
            self.queue_changed_report(cx);
        }
    }
}

impl UtilityLabRegion {
    fn queue_changed_report(&mut self, cx: &mut Cx) {
        if self.next_frame.is_some() {
            return;
        }
        let changed = self
            .ui
            .widget(cx, ids!(preview))
            .borrow::<RustifyUtilityPreview>()
            .is_some_and(|preview| {
                let report = preview.report();
                report.container.geometry.is_some()
                    && self.last_report.as_ref() != Some(&report_json(&report))
            });
        if changed {
            self.next_frame = Some(cx.new_next_frame());
        }
    }
}

fn length_json(length: GpuLength) -> Value {
    match length {
        GpuLength::Pixels(px) => json!({"kind": "px", "value": px}),
        GpuLength::Auto => json!({"kind": "auto"}),
        GpuLength::Unbounded => json!({"kind": "unbounded"}),
    }
}

fn node_json(report: &UtilityNodeReport) -> Value {
    let style = &report.computed;
    let rgba = |color: rustify_ui::theme::Rgba| [color.r, color.g, color.b, color.a];
    let status = match report.status {
        GpuResolutionStatus::Applied => "applied",
        GpuResolutionStatus::Partial => "partial",
        GpuResolutionStatus::Rejected => "rejected",
    };
    let diagnostics: Vec<_> = report
        .diagnostics
        .iter()
        .map(|diagnostic| {
            let kind = match diagnostic.kind {
                GpuDiagnosticKind::UnsupportedUtility => "unsupportedUtility",
                GpuDiagnosticKind::UnsupportedVariant => "unsupportedVariant",
                GpuDiagnosticKind::UnsupportedValue => "unsupportedValue",
                GpuDiagnosticKind::InvalidContext => "invalidContext",
            };
            json!({"candidate": diagnostic.candidate, "property": diagnostic.property,
                "kind": kind, "reason": diagnostic.reason})
        })
        .collect();
    let shadows: Vec<_> = style
        .shadows
        .iter()
        .map(|shadow| {
            json!({"rgba": rgba(shadow.color), "offset_x": shadow.offset_x,
            "offset_y": shadow.offset_y, "blur": shadow.blur, "spread": shadow.spread})
        })
        .collect();
    let line_height = match style.line_height {
        GpuLineHeight::Normal => json!({"kind": "normal"}),
        GpuLineHeight::Pixels(px) => json!({"kind": "px", "value": px}),
    };
    let flow = match style.flow {
        GpuFlow::Row => "row",
        GpuFlow::Column => "column",
    };
    let items = match style.items {
        GpuAlignment::Start => "start",
        GpuAlignment::Center => "center",
        GpuAlignment::End => "end",
        GpuAlignment::Stretch => "stretch",
    };
    let justify = match style.justify {
        GpuJustification::Start => "start",
        GpuJustification::Center => "center",
        GpuJustification::End => "end",
        GpuJustification::SpaceBetween => "space-between",
        GpuJustification::SpaceAround => "space-around",
        GpuJustification::SpaceEvenly => "space-evenly",
    };
    let modality = match report.state.modality {
        InputModality::Keyboard => "keyboard",
        InputModality::Pointer => "pointer",
        InputModality::Programmatic => "programmatic",
    };
    json!({
        "attempted_revision": report.attempted_revision,
        "applied_revision": report.applied_revision,
        "drawn_revision": report.drawn_revision,
        "geometry": report.geometry,
        "text_rows": report.text_rows.iter().map(|row| json!({"text": row.text, "x": row.x,
            "width": row.width, "top": row.top, "height": row.height})).collect::<Vec<_>>(),
        "status": status, "diagnostics": diagnostics,
        "state": {"hover": report.state.hover, "active": report.state.active,
            "focused": report.state.focused, "focus_visible": report.state.focus_visible,
            "disabled": report.state.disabled, "modality": modality},
        "computed": {"background": rgba(style.background), "color": rgba(style.color),
            "border_colors": style.border_colors.map(rgba), "padding": style.padding,
            "gap": style.gap, "width": length_json(style.width), "height": length_json(style.height),
            "min_width": length_json(style.min_width), "min_height": length_json(style.min_height),
            "max_width": length_json(style.max_width), "max_height": length_json(style.max_height),
            "flow": flow, "items": items, "justify": justify, "radii": style.radii,
            "border_widths": style.border_widths, "opacity": style.opacity,
            "font": {"requested": style.font.requested, "family": style.font.family,
                "resource_path": style.font.resource_path, "fallback": style.font.fallback},
            "font_size_px": style.font_size_px, "font_size_points": style.font_size_points,
            "font_weight": style.font_weight, "line_height": line_height,
            "letter_spacing_em": style.letter_spacing_em, "shadows": shadows}
    })
}

pub fn report_json(report: &UtilityPreviewReport) -> Value {
    json!({"container": node_json(&report.container), "button": node_json(&report.button),
        "text": node_json(&report.text), "swatch": node_json(&report.swatch)})
}
