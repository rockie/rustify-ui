//! Program/style/revision state without Makepad or a drawing loop.
use crate::{theme::ResolvedTheme, utility::gpu::*};
use rustify_style::{program::GpuProgram, text::Text};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UtilityNodeKind {
    Container,
    #[default]
    Button,
    Text,
    ColorBlock,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityNodeProps {
    pub revision: u64,
    pub programs: Arc<[GpuProgram]>,
    pub theme: Arc<ResolvedTheme>,
    pub context: GpuResolveContext,
    pub label: String,
    pub kind: UtilityNodeKind,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityPreviewProps {
    pub container: UtilityNodeProps,
    pub button: UtilityNodeProps,
    pub text: UtilityNodeProps,
    pub swatch: UtilityNodeProps,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityNodeReport {
    pub attempted_revision: Option<u64>,
    pub applied_revision: Option<u64>,
    pub drawn_revision: Option<u64>,
    pub geometry: Option<[f64; 4]>,
    /// Line boxes of the last painted label, in the same coordinates as `geometry`.
    pub text_rows: Vec<UtilityTextRow>,
    pub status: GpuResolutionStatus,
    pub diagnostics: Vec<GpuDiagnostic>,
    pub computed: GpuComputedStyle,
    pub state: GpuControlState,
}
/// One painted line: its text without the whitespace CSS collapses at a wrap,
/// the inked advance from the first to the last visible glyph, and the line box.
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityTextRow {
    pub text: String,
    pub x: f64,
    pub width: f64,
    pub top: f64,
    pub height: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityPreviewReport {
    pub container: UtilityNodeReport,
    pub button: UtilityNodeReport,
    pub text: UtilityNodeReport,
    pub swatch: UtilityNodeReport,
}
#[derive(Default)]
pub(crate) struct NodeModel {
    active: Option<UtilityNodeProps>,
    rejected: Option<UtilityNodeProps>,
    attempt_rejected: bool,
    attempted_revision: Option<u64>,
    applied_revision: Option<u64>,
    drawn_revision: Option<u64>,
    geometry: Option<[f64; 4]>,
    text_rows: Vec<UtilityTextRow>,
    resolution: Option<GpuResolution>,
    state: GpuControlState,
    style: Option<GpuComputedStyle>,
}
impl NodeModel {
    pub fn apply(&mut self, mut props: UtilityNodeProps) -> bool {
        if self.active.is_none() {
            self.state = props.context.state;
        }
        self.state.disabled = props.context.state.disabled;
        if self.state.disabled {
            self.state.active = false;
            self.state.focused = false;
            self.state.focus_visible = false;
        }
        props.context.state = self.state;
        props.context.capabilities.nonuniform_borders = false;
        props.context.capabilities.nonuniform_corners = false;
        match props.kind {
            UtilityNodeKind::Container => {
                props.context.capabilities.independent_leaf_opacity = false;
                props.context.capabilities.text = false;
            }
            UtilityNodeKind::ColorBlock => {
                props.context.capabilities.text = false;
                props.context.capabilities.flex_layout = false;
            }
            UtilityNodeKind::Button => {
                props.context.capabilities.independent_leaf_opacity = false;
            }
            UtilityNodeKind::Text => {}
        }
        if self.style.is_none() {
            let mut baseline = props.clone();
            baseline.programs = Arc::from([]);
            // A malformed requested environment must not erase a previously
            // drawable ordinary widget baseline on its first utility attempt.
            baseline.context = GpuResolveContext::new(props.context.base_style.clone());
            baseline.context.state = self.state;
            baseline.context.capabilities = props.context.capabilities;
            let result = resolve_gpu(&baseline.programs, &baseline.theme, &baseline.context);
            if result.status == GpuResolutionStatus::Applied {
                self.style = Some(result.style);
                self.active = Some(baseline);
            }
        }
        props.context.previous_style = self.style.clone();
        self.attempted_revision = Some(props.revision);
        let resolution = resolve_node(&props);
        let changed = resolution.changed
            || self
                .active
                .as_ref()
                .is_none_or(|old| old.label != props.label || old.kind != props.kind);
        if resolution.status != GpuResolutionStatus::Rejected {
            self.style = Some(resolution.style.clone());
            self.applied_revision = Some(props.revision);
            self.active = Some(props);
            self.rejected = None;
        } else {
            self.rejected = Some(props);
        }
        let successful = resolution.status != GpuResolutionStatus::Rejected;
        self.attempt_rejected = !successful;
        self.resolution = Some(resolution);
        changed && successful
    }
    pub fn update_state(&mut self, mut state: GpuControlState) -> bool {
        if state.disabled {
            state.active = false;
            state.focused = false;
            state.focus_visible = false;
        }
        if self.state == state {
            return false;
        }
        self.state = state;
        let Some(mut props) = self.active.clone() else {
            return false;
        };
        props.context.state = state;
        props.context.previous_style = self.style.clone();
        let resolution = resolve_node(&props);
        let changed = resolution.changed;
        if resolution.status != GpuResolutionStatus::Rejected {
            self.style = Some(resolution.style.clone());
            self.active = Some(props);
        }
        // A failed attempted revision stays observable while old programs react.
        if !self.attempt_rejected {
            self.resolution = Some(resolution);
        }
        changed
    }
    pub fn update_viewport(&mut self, viewport: [f64; 2]) -> bool {
        if let Some(props) = self.rejected.as_mut() {
            props.context.viewport = viewport;
        }
        let Some(mut props) = self.active.clone() else {
            return false;
        };
        if props.context.viewport == viewport {
            return false;
        }
        props.context.viewport = viewport;
        props.context.state = self.state;
        props.context.previous_style = self.style.clone();
        let resolution = resolve_node(&props);
        let changed = resolution.changed;
        if resolution.status != GpuResolutionStatus::Rejected {
            self.style = Some(resolution.style.clone());
            self.active = Some(props);
        }
        if !self.attempt_rejected {
            self.resolution = Some(resolution);
        }
        changed
    }
    pub fn props(&self) -> Option<&UtilityNodeProps> {
        self.active.as_ref()
    }
    pub fn layout_context(&self) -> Option<&GpuResolveContext> {
        self.rejected
            .as_ref()
            .or(self.active.as_ref())
            .map(|props| &props.context)
    }
    pub fn update_layout(
        &mut self,
        parent_size: [Option<f64>; 2],
        control_size: [Option<f64>; 2],
    ) -> bool {
        let Some(props) = self.rejected.as_ref().or(self.active.as_ref()) else {
            return false;
        };
        if props.context.parent_size == parent_size && props.context.control_size == control_size {
            return false;
        }
        let mut props = props.clone();
        props.context.parent_size = parent_size;
        props.context.control_size = control_size;
        let changed = self.apply(props);
        if !self.attempt_rejected {
            return changed;
        }
        // Invalid new programs do not stop the last committed programs reacting
        // to layout. Their attempted diagnostics and revision remain visible.
        let Some(mut active) = self.active.clone() else {
            return changed;
        };
        active.context.parent_size = parent_size;
        active.context.control_size = control_size;
        active.context.previous_style = self.style.clone();
        let resolution = resolve_node(&active);
        if resolution.status != GpuResolutionStatus::Rejected {
            self.style = Some(resolution.style);
            self.active = Some(active);
            return resolution.changed;
        }
        changed
    }
    pub fn update_baseline(
        &mut self,
        theme: Arc<ResolvedTheme>,
        baseline: GpuComputedStyle,
    ) -> bool {
        if let Some(props) = self.rejected.as_mut() {
            props.context.rem_px = theme.rem_px;
            props.theme = theme.clone();
            props.context.base_style = baseline.clone();
        }
        let Some(mut props) = self.active.clone() else {
            return false;
        };
        // A scoped snapshot is resolved using the host layout rem. Responsive
        // media rem remains the separate browser initial-font measurement.
        props.context.rem_px = theme.rem_px;
        props.theme = theme;
        props.context.base_style = baseline;
        props.context.state = self.state;
        props.context.previous_style = self.style.clone();
        let resolution = resolve_node(&props);
        let changed = resolution.changed;
        if resolution.status != GpuResolutionStatus::Rejected {
            self.style = Some(resolution.style.clone());
        }
        // The scope's environment changed even when its values cannot render.
        // Later input must evaluate the committed programs in that current
        // environment, while retaining the last drawable style on rejection.
        self.active = Some(props);
        if !self.attempt_rejected {
            self.resolution = Some(resolution);
        }
        changed
    }
    pub fn state(&self) -> GpuControlState {
        self.state
    }
    pub fn style(&self) -> Option<&GpuComputedStyle> {
        self.style.as_ref()
    }
    pub fn drawn(&mut self, geometry: [f64; 4], ready: bool) {
        self.geometry = Some(geometry);
        if ready {
            self.drawn_revision = self.applied_revision;
        }
    }
    pub fn drawn_text(&mut self, rows: Vec<UtilityTextRow>) {
        self.text_rows = rows;
    }
    pub fn report(&self) -> UtilityNodeReport {
        UtilityNodeReport {
            attempted_revision: self.attempted_revision,
            applied_revision: self.applied_revision,
            drawn_revision: self.drawn_revision,
            geometry: self.geometry,
            text_rows: self.text_rows.clone(),
            status: self
                .resolution
                .as_ref()
                .map_or(GpuResolutionStatus::Rejected, |result| result.status),
            diagnostics: self
                .resolution
                .as_ref()
                .map_or_else(Vec::new, |result| result.diagnostics.clone()),
            computed: self.style.clone().unwrap_or_default(),
            state: self.state,
        }
    }
}

fn resolve_node(props: &UtilityNodeProps) -> GpuResolution {
    let mut resolution = resolve_gpu(&props.programs, &props.theme, &props.context);
    if resolution.status != GpuResolutionStatus::Rejected
        && resolution.style.opacity != 1.
        && (props.kind == UtilityNodeKind::ColorBlock && !resolution.style.shadows.is_empty()
            || props.kind == UtilityNodeKind::Text
                && (resolution.style.background.a != 0.
                    || resolution
                        .style
                        .border_widths
                        .iter()
                        .any(|width| *width > 0.)
                    || !resolution.style.shadows.is_empty()))
    {
        resolution.diagnostics.push(GpuDiagnostic {
            candidate: props
                .programs
                .iter()
                .find(|program| {
                    program
                        .declarations
                        .iter()
                        .any(|declaration| declaration.property == "opacity")
                })
                .map_or_else(Text::default, |program| program.candidate.clone()),
            property: Some("opacity".into()),
            kind: GpuDiagnosticKind::UnsupportedUtility,
            reason: "this combination requires group opacity composition".into(),
        });
        if props.context.policy == GpuApplyPolicy::Strict {
            resolution.status = GpuResolutionStatus::Rejected;
            resolution.style = props
                .context
                .previous_style
                .clone()
                .unwrap_or_else(|| props.context.base_style.clone());
            resolution.changed = false;
        } else {
            resolution.status = GpuResolutionStatus::Partial;
            resolution.style.opacity = props.context.base_style.opacity;
            resolution.changed = props
                .context
                .previous_style
                .as_ref()
                .is_none_or(|previous| !previous.rendered_eq(&resolution.style));
        }
    }
    resolution
}

/// Percentage bases are the parent's content box, with unknown axes retained.
pub(super) fn content_size(
    style: &GpuComputedStyle,
    border_box: [Option<f64>; 2],
) -> [Option<f64>; 2] {
    [
        border_box[0].map(|width| {
            (width
                - style.padding[1]
                - style.padding[3]
                - style.border_widths[1]
                - style.border_widths[3])
                .max(0.)
        }),
        border_box[1].map(|height| {
            (height
                - style.padding[0]
                - style.padding[2]
                - style.border_widths[0]
                - style.border_widths[2])
                .max(0.)
        }),
    ]
}

/// Finite layout of the three preview children; rectangles use CSS pixels.
pub(super) fn arrange(
    container: [f64; 4],
    style: &GpuComputedStyle,
    children: [[f64; 2]; 3],
    stretchable: [[bool; 2]; 3],
) -> [[f64; 4]; 3] {
    let [x, y, width, height] = container;
    let [top, right, bottom, left] = style.padding;
    let content = [
        (width - left - right - style.border_widths[1] - style.border_widths[3]).max(0.),
        (height - top - bottom - style.border_widths[0] - style.border_widths[2]).max(0.),
    ];
    let origin = [
        x + left + style.border_widths[3],
        y + top + style.border_widths[0],
    ];
    let main = if style.flow == GpuFlow::Row { 0 } else { 1 };
    let cross = 1 - main;
    let gap = style.gap[1 - main];
    let used = children.iter().map(|size| size[main]).sum::<f64>() + gap * 2.;
    let free = content[main] - used;
    let (offset, extra) = match style.justify {
        GpuJustification::Start => (0., 0.),
        GpuJustification::End => (free, 0.),
        GpuJustification::Center => (free / 2., 0.),
        GpuJustification::SpaceBetween => (0., free.max(0.) / 2.),
        GpuJustification::SpaceAround => (free.max(0.) / 6., free.max(0.) / 3.),
        GpuJustification::SpaceEvenly => (free.max(0.) / 4., free.max(0.) / 4.),
    };
    let mut position = offset;
    std::array::from_fn(|index| {
        let mut size = children[index];
        if style.items == GpuAlignment::Stretch && stretchable[index][cross] {
            size[cross] = content[cross];
        }
        let cross_offset = match style.items {
            GpuAlignment::Start | GpuAlignment::Stretch => 0.,
            GpuAlignment::Center => (content[cross] - size[cross]) / 2.,
            GpuAlignment::End => content[cross] - size[cross],
        };
        let mut point = origin;
        point[main] += position;
        point[cross] += cross_offset;
        position += size[main] + gap + extra;
        [point[0], point[1], size[0], size[1]]
    })
}
pub(crate) fn constrained(value: GpuLength, min: GpuLength, max: GpuLength, intrinsic: f64) -> f64 {
    let mut value = match value {
        GpuLength::Pixels(px) => px,
        _ => intrinsic,
    };
    let minimum = if let GpuLength::Pixels(min) = min {
        min
    } else {
        0.
    };
    if let GpuLength::Pixels(max) = max {
        value = value.min(max.max(minimum));
    }
    value.max(minimum)
}
pub(crate) fn box_minimum(style: &GpuComputedStyle) -> [f64; 2] {
    let physical = [
        style.padding[1] + style.padding[3] + style.border_widths[1] + style.border_widths[3],
        style.padding[0] + style.padding[2] + style.border_widths[0] + style.border_widths[2],
    ];
    let requested = [style.min_width, style.min_height].map(|length| match length {
        GpuLength::Pixels(px) => px,
        _ => 0.,
    });
    [requested[0].max(physical[0]), requested[1].max(physical[1])]
}

/// The content width a box offers its text: its own definite width, or the
/// width its parent offers an auto box, less padding and borders.
pub(crate) fn text_available_width(
    style: &GpuComputedStyle,
    border_box_width: Option<f64>,
) -> Option<f64> {
    let border_box = match style.width {
        GpuLength::Pixels(px) => Some(px),
        _ => border_box_width,
    }
    .or(match style.max_width {
        GpuLength::Pixels(px) => Some(px),
        _ => None,
    })?;
    let border_box = constrained(
        GpuLength::Pixels(border_box),
        GpuLength::Pixels(box_minimum(style)[0]),
        style.max_width,
        border_box,
    );
    Some(
        (border_box
            - style.padding[1]
            - style.padding[3]
            - style.border_widths[1]
            - style.border_widths[3])
            .max(0.),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_carries_the_lines_of_the_latest_paint() {
        let mut model = NodeModel::default();
        assert!(model.report().text_rows.is_empty());
        let row = |text: &str, top| UtilityTextRow {
            text: text.into(),
            x: 8.,
            width: 70.,
            top,
            height: 21.,
        };
        model.drawn_text(vec![row("Live utility", 4.), row("sample", 25.)]);
        assert_eq!(model.report().text_rows[1], row("sample", 25.));
        model.drawn_text(vec![row("Live utility sample", 14.)]);
        assert_eq!(
            model.report().text_rows,
            vec![row("Live utility sample", 14.)]
        );
    }
    #[test]
    fn text_wraps_at_the_content_box_of_a_definite_or_offered_width() {
        let style = GpuComputedStyle {
            width: GpuLength::Pixels(137.),
            padding: [7.; 4],
            border_widths: [1.; 4],
            ..GpuComputedStyle::default()
        };
        assert_eq!(text_available_width(&style, Some(500.)), Some(121.));
        let auto = GpuComputedStyle {
            width: GpuLength::Auto,
            max_width: GpuLength::Pixels(90.),
            ..style.clone()
        };
        assert_eq!(text_available_width(&auto, Some(500.)), Some(74.));
        assert_eq!(text_available_width(&auto, None), Some(74.));
        let open = GpuComputedStyle {
            max_width: GpuLength::Unbounded,
            ..auto
        };
        assert_eq!(text_available_width(&open, Some(300.)), Some(284.));
        assert_eq!(text_available_width(&open, None), None);
        let crushed = GpuComputedStyle {
            width: GpuLength::Pixels(4.),
            ..style
        };
        assert_eq!(text_available_width(&crushed, None), Some(0.));
    }
    #[test]
    fn border_box_cannot_shrink_below_its_padding_and_borders() {
        let style = GpuComputedStyle {
            padding: [0., 16., 0., 16.],
            border_widths: [1.; 4],
            ..GpuComputedStyle::default()
        };
        assert_eq!(box_minimum(&style), [34., 2.]);
        let thick = GpuComputedStyle {
            padding: [0.; 4],
            border_widths: [100.; 4],
            ..style
        };
        assert_eq!(box_minimum(&thick), [200., 200.]);
    }
    use crate::theme::{font_context, resolve, Theme, ThemeDocument, ThemeMode};
    use rustify_style::program::{GpuCondition, GpuDeclaration, GpuValue};
    fn props(
        kind: UtilityNodeKind,
        revision: u64,
        property: &str,
        value: &str,
    ) -> UtilityNodeProps {
        let theme = Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                ThemeMode::Light,
                &font_context(16., false),
            )
            .unwrap(),
        );
        UtilityNodeProps {
            revision,
            kind,
            programs: Arc::from([GpuProgram {
                candidate: "test".into(),
                conditions: vec![],
                cascade_order: 0,
                declarations: vec![GpuDeclaration {
                    property: property.into(),
                    value: GpuValue::CssExpression {
                        value: value.into(),
                    },
                    important: false,
                }],
            }]),
            context: GpuResolveContext::new(GpuComputedStyle::from_theme(&theme)),
            theme,
            label: "Apply utility".into(),
        }
    }
    #[test]
    fn attempted_revision_does_not_advance_actual_drawn_revision() {
        let mut model = NodeModel::default();
        assert!(model.apply(props(UtilityNodeKind::Button, 1, "padding", "4px")));
        assert_eq!(model.report().drawn_revision, None);
        model.drawn([0., 0., 100., 40.], true);
        assert_eq!(model.report().drawn_revision, Some(1));
        assert!(!model.apply(props(UtilityNodeKind::Button, 2, "display", "grid")));
        let report = model.report();
        assert_eq!(report.attempted_revision, Some(2));
        assert_eq!(report.applied_revision, Some(1));
        assert_eq!(report.drawn_revision, Some(1));
        assert_eq!(report.computed.padding, [4.; 4]);
        model.drawn([10., 20., 100., 40.], true);
        assert_eq!(model.report().drawn_revision, Some(1));
    }
    #[test]
    fn unchanged_values_need_no_widget_redraw_but_do_not_invent_a_draw() {
        let mut model = NodeModel::default();
        model.apply(props(UtilityNodeKind::Text, 1, "padding", "4px"));
        model.drawn([0., 0., 100., 40.], true);
        assert!(!model.apply(props(UtilityNodeKind::Text, 2, "padding", "4px")));
        assert_eq!(model.report().drawn_revision, Some(1));
        model.drawn([0., 0., 100., 40.], false);
        assert_eq!(model.report().drawn_revision, Some(1));
        model.drawn([0., 0., 100., 40.], true);
        assert_eq!(model.report().drawn_revision, Some(2));
    }
    #[test]
    fn container_and_button_reject_group_opacity_but_solid_swatch_accepts() {
        for kind in [UtilityNodeKind::Container, UtilityNodeKind::Button] {
            let mut model = NodeModel::default();
            model.apply(props(kind, 1, "opacity", "0.5"));
            assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        }
        let mut model = NodeModel::default();
        assert!(model.apply(props(UtilityNodeKind::ColorBlock, 1, "opacity", "0.5")));
        assert_eq!(model.report().status, GpuResolutionStatus::Applied);
    }
    #[test]
    fn text_opacity_requires_pure_text_and_rejects_actual_layer_combinations() {
        let mut text = props(UtilityNodeKind::Text, 1, "opacity", "0.5");
        text.context.base_style.background.a = 0.;
        let mut pure = NodeModel::default();
        pure.apply(text.clone());
        assert_eq!(pure.report().status, GpuResolutionStatus::Applied);
        assert_eq!(pure.report().computed.opacity, 0.5);
        for (property, value) in [
            ("background-color", "#fff"),
            ("border", "2px solid #000"),
            ("box-shadow", "1px 2px 3px #0008"),
        ] {
            let layer = props(UtilityNodeKind::Text, 2, property, value);
            let mut combined = text.clone();
            combined.revision = 2;
            combined.programs = Arc::from([text.programs[0].clone(), layer.programs[0].clone()]);
            let previous = pure.report().computed;
            assert!(!pure.apply(combined.clone()));
            let rejected = pure.report();
            assert_eq!(rejected.status, GpuResolutionStatus::Rejected, "{property}");
            assert_eq!(rejected.computed, previous);
            assert!(rejected.diagnostics.iter().any(|diagnostic| {
                diagnostic.property.as_deref() == Some("opacity")
                    && diagnostic.reason.contains("group opacity")
            }));
            combined.context.policy = GpuApplyPolicy::Partial;
            let mut partial = NodeModel::default();
            partial.apply(combined);
            assert_eq!(partial.report().status, GpuResolutionStatus::Partial);
            assert_eq!(partial.report().computed.opacity, 1.);
        }
    }
    #[test]
    fn state_triggered_group_opacity_is_rejected_and_recovers_on_state_exit() {
        let mut value = props(
            UtilityNodeKind::ColorBlock,
            1,
            "box-shadow",
            "1px 2px 3px #0008",
        );
        let mut alpha =
            (*props(UtilityNodeKind::ColorBlock, 1, "opacity", "0.5").programs).to_vec();
        alpha[0].conditions.push(GpuCondition::Hover);
        value.programs = Arc::from([value.programs[0].clone(), alpha[0].clone()]);
        let mut model = NodeModel::default();
        model.apply(value);
        let before = model.style().unwrap().clone();
        let hover = GpuControlState {
            hover: true,
            ..GpuControlState::default()
        };
        assert!(!model.update_state(hover));
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.style(), Some(&before));
        model.update_state(GpuControlState::default());
        assert_eq!(model.report().status, GpuResolutionStatus::Applied);
    }
    #[test]
    fn old_successful_programs_keep_reacting_after_failed_attempt() {
        let mut value = props(UtilityNodeKind::Button, 1, "padding", "4px");
        Arc::make_mut(&mut value.programs)[0]
            .conditions
            .push(GpuCondition::Hover);
        let mut model = NodeModel::default();
        model.apply(value);
        model.apply(props(UtilityNodeKind::Button, 2, "display", "grid"));
        assert!(model.update_state(GpuControlState {
            hover: true,
            ..GpuControlState::default()
        }));
        assert_eq!(model.report().computed.padding, [4.; 4]);
        assert_eq!(model.report().attempted_revision, Some(2));
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert!(model.props().is_some());
        assert!(model.state().hover);
    }
    #[test]
    fn responsive_update_and_preview_reports_keep_node_revisions_distinct() {
        let mut value = props(UtilityNodeKind::Button, 4, "padding", "8px");
        Arc::make_mut(&mut value.programs)[0]
            .conditions
            .push(GpuCondition::Media {
                query: "(width >= 640px)".into(),
            });
        let mut model = NodeModel::default();
        model.apply(value.clone());
        assert!(model.update_viewport([800., 600.]));
        assert_eq!(model.style().unwrap().padding, [8.; 4]);
        assert!(!model.update_viewport([800., 600.]));
        let preview = UtilityPreviewProps {
            container: value.clone(),
            button: value.clone(),
            text: value.clone(),
            swatch: value,
        };
        let reports = UtilityPreviewReport {
            container: model.report(),
            button: model.report(),
            text: model.report(),
            swatch: model.report(),
        };
        assert_eq!(
            preview.container.revision,
            reports.container.attempted_revision.unwrap()
        );
        assert_eq!(reports.button.drawn_revision, None);
    }
    #[test]
    fn disabled_state_clears_activation_and_focus_for_host_state_updates() {
        let mut model = NodeModel::default();
        model.apply(props(UtilityNodeKind::Button, 1, "padding", "4px"));
        model.update_state(GpuControlState {
            disabled: true,
            active: true,
            focused: true,
            focus_visible: true,
            modality: InputModality::Keyboard,
            ..GpuControlState::default()
        });
        let state = model.report().state;
        assert!(state.disabled);
        assert!(!state.active);
        assert!(!state.focused);
        assert!(!state.focus_visible);
        model.update_viewport([900., 700.]);
        assert!(model.report().state.disabled);
    }
    #[test]
    fn rejected_dark_environment_stays_current_during_state_changes() {
        let mut value = props(UtilityNodeKind::Button, 3, "display", "grid");
        Arc::make_mut(&mut value.programs)[0]
            .conditions
            .push(GpuCondition::Dark);
        let light = value.theme.clone();
        let mut model = NodeModel::default();
        model.apply(value);
        model.drawn([0., 0., 170., 36.], true);
        let old = model.report().computed;
        let dark = Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                ThemeMode::Dark,
                &font_context(20., false),
            )
            .unwrap(),
        );
        assert!(!model.update_baseline(dark.clone(), GpuComputedStyle::from_theme(&dark)));
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        model.update_state(GpuControlState {
            hover: true,
            ..GpuControlState::default()
        });
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.report().computed, old);
        assert_eq!(model.props().unwrap().theme.mode, ThemeMode::Dark);
        assert_eq!(model.props().unwrap().context.rem_px, 20.);
        model.update_baseline(light.clone(), GpuComputedStyle::from_theme(&light));
        assert_eq!(model.report().status, GpuResolutionStatus::Applied);
        assert_eq!(model.report().computed, old);
        assert_eq!(model.report().attempted_revision, Some(3));
        assert_eq!(model.report().drawn_revision, Some(3));
    }
    #[test]
    fn viewport_condition_rejection_recovers_without_a_new_attempt() {
        let mut value = props(
            UtilityNodeKind::ColorBlock,
            1,
            "box-shadow",
            "1px 2px 3px #0008",
        );
        let mut alpha = props(UtilityNodeKind::ColorBlock, 1, "opacity", "0.5");
        Arc::make_mut(&mut alpha.programs)[0]
            .conditions
            .push(GpuCondition::Media {
                query: "(width >= 640px)".into(),
            });
        value.programs = Arc::from([value.programs[0].clone(), alpha.programs[0].clone()]);
        let mut model = NodeModel::default();
        model.apply(value);
        let before = model.style().unwrap().clone();
        assert!(!model.update_viewport([800., 600.]));
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.style(), Some(&before));
        model.update_viewport([500., 600.]);
        let report = model.report();
        assert_eq!(report.status, GpuResolutionStatus::Applied);
        assert_eq!(report.attempted_revision, Some(1));
        assert_eq!(report.applied_revision, Some(1));
    }
    #[test]
    fn finite_row_column_gap_alignment_and_constraint_geometry() {
        let mut style = GpuComputedStyle {
            flow: GpuFlow::Row,
            padding: [4.; 4],
            gap: [6., 10.],
            justify: GpuJustification::SpaceBetween,
            items: GpuAlignment::Center,
            ..GpuComputedStyle::default()
        };
        let boxes = arrange(
            [10., 20., 200., 80.],
            &style,
            [[20., 20.]; 3],
            [[false; 2]; 3],
        );
        assert_eq!(
            boxes,
            [
                [14., 50., 20., 20.],
                [100., 50., 20., 20.],
                [186., 50., 20., 20.]
            ]
        );
        style.flow = GpuFlow::Column;
        style.justify = GpuJustification::Start;
        style.items = GpuAlignment::Stretch;
        let boxes = arrange(
            [10., 20., 200., 80.],
            &style,
            [[20., 10.]; 3],
            [[true, false]; 3],
        );
        assert_eq!(
            boxes,
            [
                [14., 24., 192., 10.],
                [14., 40., 192., 10.],
                [14., 56., 192., 10.]
            ]
        );
        assert_eq!(
            constrained(
                GpuLength::Auto,
                GpuLength::Pixels(40.),
                GpuLength::Pixels(30.),
                10.
            ),
            40.
        );
    }
    #[test]
    fn measured_parent_layout_recovers_a_percentage_request_and_tracks_resize() {
        let mut model = NodeModel::default();
        let mut requested = props(UtilityNodeKind::ColorBlock, 7, "width", "50%");
        requested.programs = Arc::from([
            requested.programs[0].clone(),
            props(UtilityNodeKind::ColorBlock, 7, "padding", "10%").programs[0].clone(),
        ]);
        model.apply(requested);
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert!(model.update_layout([Some(200.), Some(100.)], [None; 2]));
        assert_eq!(model.report().status, GpuResolutionStatus::Applied);
        assert_eq!(model.report().applied_revision, Some(7));
        assert_eq!(model.report().computed.width, GpuLength::Pixels(100.));
        assert_eq!(model.report().computed.padding, [20.; 4]);
        assert!(model.update_layout([Some(300.), Some(50.)], [None; 2]));
        assert_eq!(
            model.layout_context().unwrap().parent_size,
            [Some(300.), Some(50.)]
        );
        assert_eq!(model.report().computed.width, GpuLength::Pixels(150.));
        assert_eq!(model.report().computed.padding, [30.; 4]);
        assert!(!model.update_layout([Some(300.), Some(50.)], [None; 2]));
        assert_eq!(model.report().drawn_revision, None);
    }
    #[test]
    fn content_box_keeps_unknown_height_and_excludes_padding_and_border() {
        let style = GpuComputedStyle {
            padding: [10., 20., 30., 40.],
            border_widths: [1., 2., 3., 4.],
            ..GpuComputedStyle::default()
        };
        assert_eq!(content_size(&style, [Some(300.), None]), [Some(234.), None]);
        assert_eq!(
            content_size(&style, [Some(30.), Some(50.)]),
            [Some(0.), Some(6.)]
        );
        let mut child = NodeModel::default();
        child.apply(props(UtilityNodeKind::ColorBlock, 1, "height", "50%"));
        child.update_layout(content_size(&style, [Some(300.), None]), [None; 2]);
        assert_eq!(child.report().status, GpuResolutionStatus::Rejected);
        assert!(!child.report().diagnostics.is_empty());
    }
    #[test]
    fn measured_control_recovers_percentage_radius_without_faking_draw_ack() {
        let mut model = NodeModel::default();
        model.apply(props(
            UtilityNodeKind::ColorBlock,
            2,
            "border-radius",
            "25%",
        ));
        model.update_layout([Some(300.), Some(200.)], [None; 2]);
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        model.update_layout([Some(300.), Some(200.)], [Some(40.), Some(40.)]);
        assert_eq!(model.report().computed.radii, [10.; 4]);
        assert_eq!(model.report().applied_revision, Some(2));
        assert_eq!(model.report().drawn_revision, None);
        model.update_layout([Some(300.), Some(200.)], [Some(80.), Some(40.)]);
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.report().computed.radii, [10.; 4]);
    }
    #[test]
    fn invalid_new_revision_keeps_old_percentage_programs_responsive() {
        let mut model = NodeModel::default();
        let mut valid = props(UtilityNodeKind::ColorBlock, 1, "width", "50%");
        valid.context.parent_size = [Some(200.), Some(100.)];
        model.apply(valid);
        model.apply(props(UtilityNodeKind::ColorBlock, 2, "display", "grid"));
        model.update_layout([Some(300.), Some(100.)], [None; 2]);
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.report().attempted_revision, Some(2));
        assert_eq!(model.report().applied_revision, Some(1));
        assert_eq!(model.report().computed.width, GpuLength::Pixels(150.));
    }
}
