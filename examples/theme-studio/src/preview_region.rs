//! One application-owned projection rendered by real GPU widgets and samples.

use std::sync::Arc;

use rustify_ui::gpu::{
    apply_text_theme, css_px_to_points, rgba_color, DrawRustifyShadow, FontSlot,
};
use rustify_ui::makepad_widgets::makepad_platform::ScrollBoundary;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::theme::{ResolvedTheme, Rgba, ShadowLayer, COLOR_TOKENS};
use rustify_ui::{
    LocalRect, Pace, RegionApp, RegionGlyph, RustifyButton, RustifyCheckBox, RustifyDropDown,
    RustifyIcon, RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar,
    RustifyToggle,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PreviewControls {
    pub checked: bool,
    pub toggled: bool,
    pub value: f64,
    pub index: usize,
    pub clicks: u64,
    pub disabled: bool,
    pub read_only: bool,
}

impl Default for PreviewControls {
    fn default() -> Self {
        Self {
            checked: false,
            toggled: false,
            value: 50.0,
            index: 0,
            clicks: 0,
            disabled: false,
            read_only: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreviewProps {
    pub theme: Arc<ResolvedTheme>,
    pub revision: u64,
    pub scene: String,
    pub controls: PreviewControls,
    pub scroll_y: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ShadowSample {
    pub rgba: [f64; 4],
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur: f64,
    pub spread: f64,
}

impl From<ShadowLayer> for ShadowSample {
    fn from(layer: ShadowLayer) -> Self {
        Self {
            rgba: channels(layer.color),
            offset_x: layer.offset_x,
            offset_y: layer.offset_y,
            blur: layer.blur,
            spread: layer.spread,
        }
    }
}

/// Draw areas and text layout bounds are reported after the draw event completes.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeSample {
    pub id: String,
    pub token_ids: Vec<String>,
    pub rect: LocalRect,
    pub visible_rect: LocalRect,
    pub hit_rect: Option<LocalRect>,
    pub visual_rect: Option<LocalRect>,
    pub rgba: Option<[f64; 4]>,
    pub radius_px: Option<f64>,
    pub font: Option<String>,
    pub font_size_px: Option<f64>,
    pub letter_spacing_em: Option<f64>,
    pub layout_size_px: Option<[f64; 2]>,
    pub shadow_layers: Vec<ShadowSample>,
}

impl ThemeSample {
    fn new(id: impl Into<String>, tokens: &[&str]) -> Self {
        Self {
            id: id.into(),
            token_ids: tokens.iter().map(|token| (*token).into()).collect(),
            rect: LocalRect::default(),
            visible_rect: LocalRect::default(),
            hit_rect: None,
            visual_rect: None,
            rgba: None,
            radius_px: None,
            font: None,
            font_size_px: None,
            letter_spacing_em: None,
            layout_size_px: None,
            shadow_layers: Vec::new(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let rect = |rect: LocalRect| [rect.x, rect.y, rect.width, rect.height];
        serde_json::json!({
            "id": self.id, "token_ids": self.token_ids, "rect": rect(self.rect),
            "visible_rect": rect(self.visible_rect), "hit_rect": self.hit_rect.map(rect),
            "visual_rect": self.visual_rect.map(rect), "rgba": self.rgba, "radius_px": self.radius_px,
            "font": self.font, "font_size_px": self.font_size_px, "letter_spacing_em": self.letter_spacing_em,
            "layout_size_px": self.layout_size_px,
            "shadow_layers": self.shadow_layers,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreviewValue {
    Clicked,
    Boolean(bool),
    Number(f64),
    Index(usize),
    Open(LocalRect),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PreviewAction {
    Drawn {
        revision: u64,
        samples: Vec<ThemeSample>,
    },
    Interacted {
        control: String,
        value: PreviewValue,
    },
}

const SHADOWS: [&str; 8] = [
    "shadow-2xs",
    "shadow-xs",
    "shadow-sm",
    "shadow",
    "shadow-md",
    "shadow-lg",
    "shadow-xl",
    "shadow-2xl",
];
const TABS: [&str; 3] = ["Overview", "Usage", "Settings"];
const LATIN: &str = "Rustify · ffi fi · e\u{301}";
const MULTILINGUAL: &str = "中文你好 · 👩‍💻";
const PAD: f64 = 16.0;

fn columns(width: f64) -> usize {
    ((width / 128.0) as usize).clamp(2, 7)
}

/// Logical sample height; the DOM viewport bounds the GPU surface separately.
pub fn preview_height(theme: &ResolvedTheme, width: f64) -> f64 {
    let width = (width - PAD * 2.0).max(1.0);
    let color_rows = COLOR_TOKENS.len().div_ceil(columns(width));
    let shadow_columns = if width >= 620.0 { 4 } else { 2 };
    let radius_rows = if width >= 440.0 { 1.0 } else { 2.0 };
    let font_height = theme.font_size_px * 3.0 + 30.0;
    let control_height = (theme.spacing_px * 9.0).max(32.0) + 30.0;
    PAD * 2.0
        + 28.0
        + 176.0
        + 28.0
        + color_rows as f64 * 64.0
        + 28.0
        + radius_rows * 88.0
        + 28.0
        + font_height * 3.0
        + 28.0
        + SHADOWS.len().div_ceil(shadow_columns) as f64 * 128.0
        + 28.0
        + control_height * 6.0
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ThemePreviewBoardBase = #(PreviewBoard::register_widget(vm))
    mod.widgets.ThemePreviewBoard = set_type_default() do mod.widgets.ThemePreviewBoardBase{
        width: Fill height: Fill
        draw_shape: mod.widgets.RustifyBox{}
        draw_border: mod.widgets.RustifyBox{}
        draw_shadow: mod.widgets.DrawRustifyShadow{}
        draw_checker +: {
            pixel: fn() {
                let point = self.pos * self.rect_size
                let check = fract((floor(point.x / 8.0) + floor(point.y / 8.0)) * 0.5) * 2.0
                return mix(vec4(0.98, 0.98, 0.98, 1.0), vec4(0.82, 0.82, 0.82, 1.0), check)
            }
        }
        draw_label +: { text_style: theme.font_regular_i18n{ font_size: 8.0 } }
        draw_sans +: { text_style: theme.font_regular_i18n{ font_size: 11.25 } }
        draw_serif +: { text_style: theme.font_regular_i18n{ font_size: 11.25 } }
        draw_mono +: { text_style: theme.font_regular_i18n{ font_size: 11.25 } }
        button: mod.widgets.RustifyButton{}
        checkbox: mod.widgets.RustifyCheckBox{}
        radio: mod.widgets.RustifyRadio{}
        toggle: mod.widgets.RustifyToggle{}
        slider: mod.widgets.RustifySlider{}
        progress: mod.widgets.RustifyProgress{}
        spinner: mod.widgets.RustifySpinner{}
        icon: mod.widgets.RustifyIcon{}
        dropdown: mod.widgets.RustifyDropDown{}
        tabs: mod.widgets.RustifyTabBar{}
    }

    startup() do #(PreviewRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass +: { clear_color: #0000 }
                body +: {
                    board := mod.widgets.ThemePreviewBoard{}
                }
            }
        }
    }
}

struct SampleDraw {
    sample: ThemeSample,
    area: Area,
    shadows: Vec<Area>,
    hit: Option<Area>,
}

#[derive(Script, ScriptHook, Widget)]
pub struct PreviewBoard {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_checker: DrawColor,
    #[live]
    draw_shape: DrawColor,
    #[live]
    draw_border: DrawColor,
    #[live]
    draw_shadow: DrawRustifyShadow,
    #[live]
    draw_label: DrawText,
    #[live]
    draw_sans: DrawText,
    #[live]
    draw_serif: DrawText,
    #[live]
    draw_mono: DrawText,
    #[live]
    button: RustifyButton,
    #[live]
    checkbox: RustifyCheckBox,
    #[live]
    radio: RustifyRadio,
    #[live]
    toggle: RustifyToggle,
    #[live]
    slider: RustifySlider,
    #[live]
    progress: RustifyProgress,
    #[live]
    spinner: RustifySpinner,
    #[live]
    icon: RustifyIcon,
    #[live]
    dropdown: RustifyDropDown,
    #[live]
    tabs: RustifyTabBar,
    #[rust]
    props: Option<PreviewProps>,
    #[rust]
    drawings: Vec<SampleDraw>,
    #[rust]
    shadow_hits: Vec<(String, Area)>,
    #[rust]
    actions: Vec<PreviewAction>,
    #[rust]
    drawn_revision: Option<u64>,
    #[rust]
    last_report: Option<(u64, Vec<ThemeSample>)>,
}

fn channels(color: Rgba) -> [f64; 4] {
    [color.r, color.g, color.b, color.a]
}
fn local(rect: Rect) -> LocalRect {
    LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y)
}
fn rect(x: f64, y: f64, width: f64, height: f64) -> Rect {
    Rect {
        pos: dvec2(x, y),
        size: dvec2(width.max(0.0), height.max(0.0)),
    }
}

fn place(
    widget: &mut impl Widget,
    cx: &mut Cx2d,
    scope: &mut Scope,
    position: DVec2,
    width: Option<f64>,
) {
    let mut walk = widget.walk(cx);
    walk.abs_pos = Some(position);
    if let Some(width) = width {
        walk.width = Size::Fixed(width);
    }
    let _ = widget.draw_walk(cx, scope, walk);
}

impl PreviewBoard {
    fn apply_props(&mut self, cx: &mut Cx, props: &PreviewProps) {
        if self.props.as_ref() == Some(props) {
            return;
        }
        let theme = &props.theme;
        let controls = &props.controls;
        self.draw_bg.color = rgba_color(theme.colors["background"]);
        self.draw_label.color = rgba_color(theme.colors["foreground"]);
        apply_text_theme(&mut self.draw_label, cx, theme, FontSlot::Sans);
        self.draw_label.text_style.font_size = css_px_to_points(14.0);
        self.draw_label.text_style.letter_spacing = 0.0;
        for (draw, slot) in [
            (&mut self.draw_sans, FontSlot::Sans),
            (&mut self.draw_serif, FontSlot::Serif),
            (&mut self.draw_mono, FontSlot::Mono),
        ] {
            apply_text_theme(draw, cx, theme, slot);
            draw.color = rgba_color(theme.colors["foreground"]);
        }
        self.button.apply_theme(cx, theme);
        self.button
            .set_state(cx, &format!("Run · {}", controls.clicks), controls.disabled);
        self.checkbox.apply_theme(cx, theme);
        self.checkbox
            .set_state(cx, controls.checked, controls.disabled, controls.read_only);
        self.radio.apply_theme(cx, theme);
        self.radio.set_state(
            cx,
            controls.index == 1,
            controls.disabled,
            controls.read_only,
        );
        self.toggle.apply_theme(cx, theme);
        self.toggle
            .set_state(cx, controls.toggled, controls.disabled, controls.read_only);
        self.slider.apply_theme(cx, theme);
        self.slider.set_range(cx, 0.0, 100.0, 1.0);
        self.slider
            .set_state(cx, controls.value, controls.disabled, controls.read_only);
        self.progress.apply_theme(cx, theme);
        self.progress.set_fraction(cx, controls.value / 100.0);
        self.spinner.apply_theme(cx, theme);
        self.spinner
            .set_state(cx, controls.toggled, theme.reduce_motion);
        self.icon.apply_theme(cx, theme);
        self.icon.set_glyph(cx, RegionGlyph::Check);
        self.dropdown.apply_theme(cx, theme);
        self.dropdown.set_state(
            cx,
            TABS.get(controls.index).copied().unwrap_or(TABS[0]),
            controls.disabled,
            controls.read_only,
        );
        self.tabs.apply_theme(cx, theme);
        let tabs: Vec<String> = TABS.iter().map(|label| (*label).into()).collect();
        self.tabs
            .set_state(cx, &tabs, controls.index, controls.disabled);
        self.props = Some(props.clone());
        self.last_report = None;
        self.redraw(cx);
    }

    fn shape(
        &mut self,
        cx: &mut Cx2d,
        id: impl Into<String>,
        token: &str,
        theme: &ResolvedTheme,
        bounds: Rect,
        radius: f64,
    ) -> Area {
        self.draw_shape.color = rgba_color(theme.colors[token]);
        self.draw_shape
            .set_uniform(cx, id!(corner), &[(radius * 0.5) as f32]);
        self.draw_shape.set_uniform(cx, id!(stroke_width), &[0.0]);
        self.draw_shape.draw_abs(cx, bounds);
        let area = self.draw_shape.area();
        let mut sample = ThemeSample::new(id, &[token]);
        sample.rgba = Some(channels(theme.colors[token]));
        sample.radius_px = Some(radius.min(bounds.size.x.min(bounds.size.y) * 0.5));
        self.drawings.push(SampleDraw {
            sample,
            area,
            shadows: Vec::new(),
            hit: None,
        });
        area
    }

    fn label(&mut self, cx: &mut Cx2d, x: f64, y: f64, text: &str) {
        self.draw_label.draw_abs(cx, dvec2(x, y), text);
    }

    fn heading(&mut self, cx: &mut Cx2d, x: f64, y: &mut f64, text: &str) {
        self.draw_label.text_style.font_size = css_px_to_points(16.0);
        self.label(cx, x, *y, text);
        self.draw_label.text_style.font_size = css_px_to_points(14.0);
        *y += 28.0;
    }

    fn border(
        &mut self,
        cx: &mut Cx2d,
        theme: &ResolvedTheme,
        token: &str,
        bounds: Rect,
        radius: f64,
    ) {
        self.draw_border.color = rgba_color(theme.colors[token]);
        self.draw_border
            .set_uniform(cx, id!(corner), &[(radius * 0.5) as f32]);
        self.draw_border.set_uniform(cx, id!(stroke_width), &[1.0]);
        self.draw_border.draw_abs(cx, bounds);
    }

    fn draw_scene(&mut self, cx: &mut Cx2d, theme: &ResolvedTheme, x: f64, y: f64, width: f64) {
        let sidebar_width = (width * 0.3).clamp(104.0, 130.0);
        let sidebar = rect(x, y, sidebar_width, 156.0);
        self.shape(
            cx,
            "scene.sidebar",
            "sidebar",
            theme,
            sidebar,
            theme.radii_px[2],
        );
        self.border(cx, theme, "sidebar-border", sidebar, theme.radii_px[2]);
        self.draw_label.color = rgba_color(theme.colors["sidebar-foreground"]);
        self.label(cx, x + 8.0, y + 12.0, "Workspace");
        for (index, (surface, foreground, label)) in [
            ("sidebar-primary", "sidebar-primary-foreground", "Overview"),
            ("sidebar-accent", "sidebar-accent-foreground", "Reports"),
        ]
        .into_iter()
        .enumerate()
        {
            let item = rect(
                x + 8.0,
                y + 40.0 + index as f64 * 44.0,
                sidebar_width - 16.0,
                32.0,
            );
            if index == 1 {
                self.draw_shadow.draw_layers(
                    cx,
                    item,
                    theme.radii_px[0],
                    &[ShadowLayer {
                        color: Rgba {
                            a: theme.colors["sidebar-ring"].a * 0.5,
                            ..theme.colors["sidebar-ring"]
                        },
                        offset_x: 0.0,
                        offset_y: 0.0,
                        blur: 0.0,
                        spread: 2.0,
                    }],
                );
            }
            self.shape(
                cx,
                format!("scene.{surface}"),
                surface,
                theme,
                item,
                theme.radii_px[0],
            );
            self.draw_label.color = rgba_color(theme.colors[foreground]);
            self.label(cx, item.pos.x + 5.0, item.pos.y + 6.0, label);
        }
        let chart = rect(
            x + sidebar_width + 12.0,
            y,
            width - sidebar_width - 12.0,
            156.0,
        );
        self.shape(cx, "scene.card", "card", theme, chart, theme.radii_px[2]);
        self.border(cx, theme, "border", chart, theme.radii_px[2]);
        self.draw_label.color = rgba_color(theme.colors["card-foreground"]);
        self.label(cx, chart.pos.x + 12.0, y + 12.0, "Activity");
        let bar_width = ((chart.size.x - 32.0) / 5.0).max(1.0);
        for index in 0..5 {
            let height = [46.0, 70.0, 52.0, 94.0, 78.0][index];
            let token = format!("chart-{}", index + 1);
            self.shape(
                cx,
                format!("scene.{token}"),
                &token,
                theme,
                rect(
                    chart.pos.x + 16.0 + index as f64 * bar_width,
                    y + 140.0 - height,
                    (bar_width - 5.0).max(1.0),
                    height,
                ),
                theme.radii_px[0],
            );
        }
        self.draw_label.color = rgba_color(theme.colors["foreground"]);
    }

    fn take_drawn(&mut self, cx: &Cx) -> Option<PreviewAction> {
        let revision = self.drawn_revision?;
        let props = self.props.as_ref()?;
        let mut samples = Vec::with_capacity(self.drawings.len() + 10);
        for drawing in &self.drawings {
            if !drawing.area.is_valid(cx) {
                continue;
            }
            let mut sample = drawing.sample.clone();
            if sample.layout_size_px.is_some() {
                sample.visible_rect = local(drawing.area.clipped_rect_union(cx));
            } else {
                sample.rect = local(drawing.area.rect(cx));
                sample.visible_rect = local(drawing.area.clipped_rect(cx));
            }
            sample.hit_rect = drawing
                .hit
                .filter(|area| area.is_valid(cx))
                .map(|area| local(area.rect(cx)));
            let mut visual: Option<Rect> = None;
            for area in drawing.shadows.iter().filter(|area| area.is_valid(cx)) {
                let next = area.rect(cx);
                visual = Some(match visual {
                    Some(old) => {
                        let left = old.pos.x.min(next.pos.x);
                        let top = old.pos.y.min(next.pos.y);
                        let right = (old.pos.x + old.size.x).max(next.pos.x + next.size.x);
                        let bottom = (old.pos.y + old.size.y).max(next.pos.y + next.size.y);
                        rect(left, top, right - left, bottom - top)
                    }
                    None => next,
                });
            }
            sample.visual_rect = visual.map(local);
            samples.push(sample);
        }
        for (id, bounds, tokens) in [
            (
                "button",
                self.button.drawn(cx),
                &["primary", "primary-foreground"][..],
            ),
            (
                "checkbox",
                self.checkbox.drawn(cx),
                &["input", "primary", "primary-foreground"][..],
            ),
            (
                "radio",
                self.radio.drawn(cx),
                &["input", "primary", "border"][..],
            ),
            (
                "switch",
                self.toggle.drawn(cx),
                &["border", "primary", "background"][..],
            ),
            (
                "slider",
                self.slider.drawn(cx),
                &["secondary", "primary", "background"][..],
            ),
            (
                "progress",
                self.progress.drawn(cx),
                &["secondary", "primary"][..],
            ),
            ("spinner", self.spinner.drawn(cx), &["primary"][..]),
            ("icon", self.icon.drawn(cx), &["foreground"][..]),
            (
                "dropdown",
                self.dropdown.drawn(cx),
                &["input", "foreground", "border"][..],
            ),
            (
                "tabs",
                self.tabs.drawn(cx),
                &["muted", "background", "foreground"][..],
            ),
        ] {
            if let Some(bounds) = bounds {
                let mut sample = ThemeSample::new(format!("control.{id}"), tokens);
                sample.rect = local(bounds);
                let clip = self.draw_bg.area().clipped_rect(cx);
                sample.visible_rect = local(bounds.clip((clip.pos, clip.pos + clip.size)));
                if !matches!(id, "progress" | "spinner" | "icon") {
                    sample.hit_rect = Some(sample.rect);
                }
                if matches!(id, "button" | "dropdown" | "tabs") {
                    sample.font = Some(props.theme.fonts[0].family.clone());
                    sample.font_size_px = Some(props.theme.font_size_px);
                    sample.letter_spacing_em = Some(props.theme.letter_spacing_em);
                }
                sample.radius_px = match id {
                    "button" | "dropdown" => Some(props.theme.radii_px[1]),
                    "tabs" => Some(props.theme.radii_px[2]),
                    "checkbox" => Some(4.0),
                    "radio" | "switch" | "progress" => Some(bounds.size.x.min(bounds.size.y) * 0.5),
                    _ => None,
                }
                .map(|radius| radius.min(bounds.size.x.min(bounds.size.y) * 0.5));
                samples.push(sample);
            }
        }
        let report = (revision, samples);
        if self.last_report.as_ref() == Some(&report) {
            return None;
        }
        self.last_report = Some(report.clone());
        Some(PreviewAction::Drawn {
            revision,
            samples: report.1,
        })
    }

    fn request(&mut self, control: &str, value: PreviewValue) {
        self.actions.push(PreviewAction::Interacted {
            control: control.into(),
            value,
        });
    }
}

impl Widget for PreviewBoard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
        self.checkbox.handle_event(cx, event, scope);
        self.radio.handle_event(cx, event, scope);
        self.toggle.handle_event(cx, event, scope);
        self.slider.handle_event(cx, event, scope);
        self.progress.handle_event(cx, event, scope);
        self.spinner.handle_event(cx, event, scope);
        self.icon.handle_event(cx, event, scope);
        self.dropdown.handle_event(cx, event, scope);
        self.tabs.handle_event(cx, event, scope);
        if self.button.take_click().is_some() {
            self.request("button", PreviewValue::Clicked);
        }
        if let Some(value) = self.checkbox.take_change() {
            self.request("checkbox", PreviewValue::Boolean(value));
        }
        if self.radio.take_change().is_some() {
            self.request("radio", PreviewValue::Index(1));
        }
        if let Some(value) = self.toggle.take_change() {
            self.request("switch", PreviewValue::Boolean(value));
        }
        if let Some(value) = self.slider.take_change() {
            self.request("slider", PreviewValue::Number(value));
        }
        if let Some(value) = self.tabs.take_change() {
            self.request("tabs", PreviewValue::Index(value));
        }
        if self.dropdown.take_open().is_some() {
            if let Some(bounds) = self.dropdown.drawn(cx) {
                self.request("dropdown", PreviewValue::Open(local(bounds)));
            }
        }
        let mut clicked_shadows = Vec::new();
        for (id, area) in &self.shadow_hits {
            if let Hit::FingerUp(fe) = event.hits(cx, *area) {
                if fe.is_primary_hit() && fe.is_over {
                    clicked_shadows.push(id.clone());
                }
            }
        }
        for id in clicked_shadows {
            self.request(&id, PreviewValue::Clicked);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        let Some(props) = self.props.clone() else {
            return DrawStep::done();
        };
        let theme = &props.theme;
        self.drawings.clear();
        self.shadow_hits.clear();
        let x = pane.pos.x + PAD;
        let width = (pane.size.x - PAD * 2.0).max(1.0);
        let mut y = pane.pos.y + PAD - props.scroll_y;
        self.heading(cx, x, &mut y, &format!("GPU · {}", props.scene));
        self.draw_scene(cx, theme, x, y, width);
        y += 176.0;
        self.heading(cx, x, &mut y, "Color tokens");
        let columns = columns(width);
        let cell_width = width / columns as f64;
        for (index, token) in COLOR_TOKENS.iter().enumerate() {
            let left = x + (index % columns) as f64 * cell_width;
            let top = y + (index / columns) as f64 * 64.0;
            let bounds = rect(left, top, cell_width - 8.0, 36.0);
            self.draw_checker.draw_abs(cx, bounds);
            self.shape(cx, format!("color.{token}"), token, theme, bounds, 3.0);
            let caption = token.replace("foreground", "fg").replace("sidebar", "side");
            self.label(cx, left, top + 42.0, &caption);
        }
        y += COLOR_TOKENS.len().div_ceil(columns) as f64 * 64.0;
        self.heading(cx, x, &mut y, "Corner radius");
        let radius_columns = if width >= 440.0 { 4 } else { 2 };
        let cell_width = width / radius_columns as f64;
        for (index, name) in ["sm", "md", "lg", "xl"].into_iter().enumerate() {
            let bounds = rect(
                x + (index % radius_columns) as f64 * cell_width,
                y + (index / radius_columns) as f64 * 88.0,
                cell_width - 8.0,
                52.0,
            );
            self.shape(
                cx,
                format!("radius.{name}"),
                "primary",
                theme,
                bounds,
                theme.radii_px[index],
            );
            self.label(
                cx,
                bounds.pos.x,
                bounds.pos.y + 60.0,
                &format!("{name} · {}px", theme.radii_px[index]),
            );
        }
        y += 4_usize.div_ceil(radius_columns) as f64 * 88.0;
        self.heading(cx, x, &mut y, "Typography");
        let font_height = theme.font_size_px * 3.0 + 30.0;
        for (index, name) in ["sans", "serif", "mono"].into_iter().enumerate() {
            self.label(cx, x, y, &format!("{name} · {}", theme.fonts[index].family));
            let draw = match index {
                0 => &mut self.draw_sans,
                1 => &mut self.draw_serif,
                _ => &mut self.draw_mono,
            };
            for (row, (kind, text)) in [("latin", LATIN), ("multilingual", MULTILINGUAL)]
                .into_iter()
                .enumerate()
            {
                let position = dvec2(x, y + 24.0 + row as f64 * theme.font_size_px * 1.5);
                let layout = draw.layout(cx, 0.0, 0.0, None, false, Align::default(), text);
                let size = [
                    layout.size_in_lpxs.width as f64,
                    layout.size_in_lpxs.height as f64,
                ];
                draw.draw_abs(cx, position, text);
                let mut sample = ThemeSample::new(format!("font.{name}.{kind}"), &["foreground"]);
                sample.rect = local(rect(position.x, position.y, size[0], size[1]));
                sample.layout_size_px = Some(size);
                sample.font = Some(theme.fonts[index].family.clone());
                sample.font_size_px = Some(theme.font_size_px);
                sample.letter_spacing_em = Some(theme.letter_spacing_em);
                sample.rgba = Some(channels(theme.colors["foreground"]));
                self.drawings.push(SampleDraw {
                    sample,
                    area: draw.area(),
                    shadows: Vec::new(),
                    hit: None,
                });
            }
            y += font_height;
        }
        self.heading(cx, x, &mut y, "Shadow layers · click each card");
        let shadow_columns = if width >= 620.0 { 4 } else { 2 };
        let cell_width = width / shadow_columns as f64;
        for (index, name) in SHADOWS.into_iter().enumerate() {
            let left = x + (index % shadow_columns) as f64 * cell_width;
            let top = y + (index / shadow_columns) as f64 * 128.0;
            let bounds = rect(left + 16.0, top + 16.0, cell_width - 40.0, 58.0);
            let layers = &theme.shadows[name];
            let mut shadow_areas = Vec::with_capacity(layers.len());
            for layer in layers.iter().rev() {
                self.draw_shadow.draw_layers(
                    cx,
                    bounds,
                    theme.radius_px,
                    std::slice::from_ref(layer),
                );
                if layer.color.a > 0.0
                    && bounds.size.x + layer.spread * 2.0 > 0.0
                    && bounds.size.y + layer.spread * 2.0 > 0.0
                {
                    shadow_areas.push(self.draw_shadow.area());
                }
            }
            let id = format!("shadow.{name}");
            let area = self.shape(cx, &id, "card", theme, bounds, theme.radius_px);
            self.border(cx, theme, "border", bounds, theme.radius_px);
            if let Some(drawing) = self.drawings.last_mut() {
                drawing.shadows = shadow_areas;
                drawing.hit = Some(area);
                drawing.sample.token_ids.push("shadow-color".into());
                drawing.sample.shadow_layers =
                    layers.iter().copied().map(ShadowSample::from).collect();
            }
            self.shadow_hits.push((id, area));
            self.label(cx, left + 16.0, top + 92.0, name);
        }
        y += SHADOWS.len().div_ceil(shadow_columns) as f64 * 128.0;
        self.heading(cx, x, &mut y, "Interactive controls");
        let row_height = (theme.spacing_px * 9.0).max(32.0) + 30.0;
        let half = width * 0.5;
        for (index, label) in [
            "Button", "Checkbox", "Radio", "Switch", "Slider", "Progress", "Spinner", "Icon",
        ]
        .into_iter()
        .enumerate()
        {
            self.label(
                cx,
                x + (index % 2) as f64 * half,
                y + (index / 2) as f64 * row_height,
                label,
            );
        }
        let pos = |index: usize| {
            dvec2(
                x + (index % 2) as f64 * half,
                y + (index / 2) as f64 * row_height + 24.0,
            )
        };
        let full = Some((half - 12.0).max(1.0));
        place(&mut self.button, cx, scope, pos(0), full);
        place(&mut self.checkbox, cx, scope, pos(1), None);
        place(&mut self.radio, cx, scope, pos(2), None);
        place(&mut self.toggle, cx, scope, pos(3), None);
        place(&mut self.slider, cx, scope, pos(4), full);
        place(&mut self.progress, cx, scope, pos(5), full);
        place(&mut self.spinner, cx, scope, pos(6), None);
        place(&mut self.icon, cx, scope, pos(7), None);
        for (row, label) in [(4.0, "Select"), (5.0, "Tabs")] {
            self.label(cx, x, y + row * row_height, label);
        }
        place(
            &mut self.dropdown,
            cx,
            scope,
            dvec2(x, y + 4.0 * row_height + 24.0),
            Some(width),
        );
        place(
            &mut self.tabs,
            cx,
            scope,
            dvec2(x, y + 5.0 * row_height + 24.0),
            Some(width),
        );
        self.drawn_revision = Some(props.revision);
        // The DOM viewport owns scrolling; draw coordinates include its offset.
        cx.report_scroll_boundary(ScrollBoundary::vertical(0.0, 0.0, true));
        DrawStep::done()
    }
}

#[derive(Script, ScriptHook)]
pub struct PreviewRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for PreviewRegion {
    type Props = PreviewProps;
    type Action = PreviewAction;

    fn pace(action: &PreviewAction) -> Pace {
        match action {
            PreviewAction::Drawn { .. } => Pace::Continuous("theme-drawn"),
            PreviewAction::Interacted {
                control,
                value: PreviewValue::Number(_),
            } if control == "slider" => Pace::Continuous("preview-slider"),
            _ => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &PreviewProps) {
        if let Some(mut board) = self.ui.widget(cx, ids!(board)).borrow_mut::<PreviewBoard>() {
            board.apply_props(cx, props);
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<PreviewAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if let Some(mut board) = self.ui.widget(cx, ids!(board)).borrow_mut::<PreviewBoard>() {
            outbox.append(&mut board.actions);
            if matches!(event, Event::Draw(_)) {
                if let Some(action) = board.take_drawn(cx) {
                    outbox.push(action);
                }
            }
        }
    }
}
