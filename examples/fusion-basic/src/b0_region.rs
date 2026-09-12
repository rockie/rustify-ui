//! The GPU half of the smallest complete application: twenty controls in one
//! region, drawn from the same state the ten DOM controls beside them edit.
//!
//! Twenty is the load's number, not a display of the catalogue: two of each
//! control the SDK draws, so that the smallest application still exercises
//! every kind of projection - a label, a request, a value on a range, a
//! choice out of a list - rather than twenty copies of the easiest one.

use rustify_ui::makepad_widgets::*;
use rustify_ui::{
    LocalRect, Pace, RegionApp, RegionGlyph, RustifyButton, RustifyCheckBox, RustifyDropDown,
    RustifyIcon, RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar,
    RustifyToggle, Theme,
};

/// Everything the region draws, and nothing it owns.
#[derive(Clone, Debug, PartialEq)]
pub struct B0Props {
    pub count: i64,
    pub visible: bool,
    pub locked: bool,
    pub compact: bool,
    pub busy: bool,
    pub choice: usize,
    pub size: f64,
    pub weight: f64,
    pub palette: String,
    pub density: String,
    pub tab: usize,
}

#[derive(Debug)]
pub enum B0Action {
    Bump,
    SetVisible(bool),
    SetLocked(bool),
    SetCompact(bool),
    SetBusy(bool),
    Choose(usize),
    SetSize(f64),
    SetWeight(f64),
    SetTab(usize),
    NextPalette,
    NextDensity,
    /// Where each control ended up, in the region's own local pixels. Sent
    /// when it changes, which after start-up is never: it is how a pointer
    /// finds a control the browser cannot see, and how the load's count of
    /// twenty is a measurement rather than a claim.
    Layout(Vec<(&'static str, LocalRect)>),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(B0Region::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    frame := View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 8
                        padding: 10

                        count_label := Label{
                            text: "count 0"
                            draw_text.text_style.font_size: 14
                        }
                        buttons := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            bump_first := RustifyButton{}
                            bump_second := RustifyButton{}
                        }
                        switches := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            align: Center
                            check_visible := RustifyCheckBox{}
                            check_locked := RustifyCheckBox{}
                            toggle_compact := RustifyToggle{}
                            toggle_busy := RustifyToggle{}
                            radio_first := RustifyRadio{}
                            radio_second := RustifyRadio{}
                        }
                        values := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            align: Center
                            slider_size := RustifySlider{}
                            slider_weight := RustifySlider{}
                        }
                        bars := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 6
                            progress_size := RustifyProgress{}
                            progress_weight := RustifyProgress{}
                        }
                        marks := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            align: Center
                            spinner_first := RustifySpinner{}
                            spinner_second := RustifySpinner{}
                            icon_check := RustifyIcon{}
                            icon_chevron := RustifyIcon{}
                        }
                        lists := View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            chooser_palette := RustifyDropDown{}
                            chooser_density := RustifyDropDown{}
                        }
                        tabs_primary := RustifyTabBar{}
                        tabs_secondary := RustifyTabBar{}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct B0Region {
    #[live]
    ui: WidgetRef,
    /// The rectangles last reported, so only a change is sent.
    #[rust]
    layout: Vec<(&'static str, LocalRect)>,
    /// Asked for after a projection, so the rectangles below are read once the
    /// draw that produced them has happened.
    #[rust]
    next_frame: Option<NextFrame>,
}

fn colour(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}

/// The two entries each chooser offers. The region shows one and asks for the
/// next; which one it then shows is the application's answer.
pub const PALETTES: [&str; 2] = ["slate", "amber"];
pub const DENSITIES: [&str; 2] = ["roomy", "tight"];
pub const TABS: [&str; 3] = ["one", "two", "three"];

impl B0Region {
    fn rect(&mut self, cx: &mut Cx, name: &'static str) -> Option<(&'static str, LocalRect)> {
        let widget = self.ui.widget(cx, &[LiveId::from_str(name)]);
        let rect = match name {
            "bump_first" | "bump_second" => widget.borrow_mut::<RustifyButton>()?.drawn(cx)?,
            "check_visible" | "check_locked" => {
                widget.borrow_mut::<RustifyCheckBox>()?.drawn(cx)?
            }
            "toggle_compact" | "toggle_busy" => widget.borrow_mut::<RustifyToggle>()?.drawn(cx)?,
            "radio_first" | "radio_second" => widget.borrow_mut::<RustifyRadio>()?.drawn(cx)?,
            "slider_size" | "slider_weight" => widget.borrow_mut::<RustifySlider>()?.drawn(cx)?,
            "progress_size" | "progress_weight" => {
                widget.borrow_mut::<RustifyProgress>()?.drawn(cx)?
            }
            "spinner_first" | "spinner_second" => {
                widget.borrow_mut::<RustifySpinner>()?.drawn(cx)?
            }
            "icon_check" | "icon_chevron" => widget.borrow_mut::<RustifyIcon>()?.drawn(cx)?,
            "chooser_palette" | "chooser_density" => {
                widget.borrow_mut::<RustifyDropDown>()?.drawn(cx)?
            }
            "tabs_primary" | "tabs_secondary" => widget.borrow_mut::<RustifyTabBar>()?.drawn(cx)?,
            _ => return None,
        };
        (rect.size.x > 0.0 && rect.size.y > 0.0).then_some((
            name,
            LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y),
        ))
    }
}

/// Every control in the region, in the order they are drawn. The load says
/// twenty; this is the list the count comes from.
pub const CONTROLS: [&str; 20] = [
    "bump_first",
    "bump_second",
    "check_visible",
    "check_locked",
    "toggle_compact",
    "toggle_busy",
    "radio_first",
    "radio_second",
    "slider_size",
    "slider_weight",
    "progress_size",
    "progress_weight",
    "spinner_first",
    "spinner_second",
    "icon_check",
    "icon_chevron",
    "chooser_palette",
    "chooser_density",
    "tabs_primary",
    "tabs_secondary",
];

impl RegionApp for B0Region {
    type Props = B0Props;
    type Action = B0Action;

    fn pace(action: &B0Action) -> Pace {
        match action {
            B0Action::Layout(_) => Pace::Continuous("layout"),
            _ => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &B0Props) {
        let theme = Theme::light();
        // Where a control ended up is only known after it is drawn, and a
        // projection brings no event of its own to read it on.
        self.next_frame = Some(cx.new_next_frame());

        self.ui
            .label(cx, ids!(count_label))
            .set_text(cx, &format!("count {}", props.count));
        self.ui
            .label(cx, ids!(count_label))
            .set_text_color(cx, colour(theme.foreground));

        let mut frame = self.ui.widget(cx, ids!(frame));
        let background = colour(theme.background);
        script_apply_eval!(cx, frame, {
            draw_bg +: {
                color: #(background)
            }
        });

        for (id, label) in [
            (ids!(bump_first), "GPU +1"),
            (ids!(bump_second), "GPU +1 again"),
        ] {
            if let Some(mut button) = self.ui.widget(cx, id).borrow_mut::<RustifyButton>() {
                button.set_state(cx, label, props.locked);
                button.set_palette(
                    cx,
                    theme.secondary,
                    theme.muted,
                    theme.border,
                    theme.secondary_foreground,
                );
            }
        }

        for (id, checked) in [
            (ids!(check_visible), props.visible),
            (ids!(check_locked), props.locked),
        ] {
            if let Some(mut checkbox) = self.ui.widget(cx, id).borrow_mut::<RustifyCheckBox>() {
                checkbox.set_state(cx, checked, false, false);
                checkbox.set_palette(cx, theme.input, theme.primary, theme.border);
            }
        }

        for (id, on) in [
            (ids!(toggle_compact), props.compact),
            (ids!(toggle_busy), props.busy),
        ] {
            if let Some(mut toggle) = self.ui.widget(cx, id).borrow_mut::<RustifyToggle>() {
                toggle.set_state(cx, on, false, false);
                toggle.set_palette(cx, theme.border, theme.primary, theme.background);
            }
        }

        for (index, id) in [ids!(radio_first), ids!(radio_second)]
            .into_iter()
            .enumerate()
        {
            if let Some(mut radio) = self.ui.widget(cx, id).borrow_mut::<RustifyRadio>() {
                radio.set_state(cx, props.choice == index, false, false);
                radio.set_palette(cx, theme.input, theme.primary, theme.border);
            }
        }

        for (id, value) in [
            (ids!(slider_size), props.size),
            (ids!(slider_weight), props.weight),
        ] {
            if let Some(mut slider) = self.ui.widget(cx, id).borrow_mut::<RustifySlider>() {
                slider.set_range(cx, 0.0, 100.0, 5.0);
                slider.set_state(cx, value, false, props.locked);
                slider.set_palette(cx, theme.border, theme.primary, theme.background);
            }
        }

        for (id, value) in [
            (ids!(progress_size), props.size),
            (ids!(progress_weight), props.weight),
        ] {
            if let Some(mut progress) = self.ui.widget(cx, id).borrow_mut::<RustifyProgress>() {
                progress.set_fraction(cx, value / 100.0);
                progress.set_palette(cx, theme.secondary, theme.primary);
            }
        }

        for id in [ids!(spinner_first), ids!(spinner_second)] {
            if let Some(mut spinner) = self.ui.widget(cx, id).borrow_mut::<RustifySpinner>() {
                spinner.set_state(cx, props.busy, theme.reduce_motion);
                spinner.set_ink(cx, theme.primary);
            }
        }

        for (id, glyph) in [
            (ids!(icon_check), RegionGlyph::Check),
            (ids!(icon_chevron), RegionGlyph::ChevronDown),
        ] {
            if let Some(mut icon) = self.ui.widget(cx, id).borrow_mut::<RustifyIcon>() {
                icon.set_glyph(cx, glyph);
                icon.set_ink(cx, theme.foreground);
            }
        }

        for (id, shown) in [
            (ids!(chooser_palette), props.palette.as_str()),
            (ids!(chooser_density), props.density.as_str()),
        ] {
            if let Some(mut chooser) = self.ui.widget(cx, id).borrow_mut::<RustifyDropDown>() {
                chooser.set_state(cx, shown, false, false);
                chooser.set_palette(
                    cx,
                    theme.input,
                    theme.border,
                    theme.foreground,
                    theme.muted_foreground,
                );
            }
        }

        let tabs: Vec<String> = TABS.iter().map(|name| name.to_string()).collect();
        for id in [ids!(tabs_primary), ids!(tabs_secondary)] {
            if let Some(mut bar) = self.ui.widget(cx, id).borrow_mut::<RustifyTabBar>() {
                bar.set_state(cx, &tabs, props.tab, false);
                bar.set_palette(cx, theme.muted, theme.background, theme.foreground);
            }
        }

        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<B0Action>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());

        for id in [ids!(bump_first), ids!(bump_second)] {
            let clicked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyButton>()
                .and_then(|mut button| button.take_click());
            if clicked.is_some() {
                outbox.push(B0Action::Bump);
            }
        }
        for (id, make) in [
            (
                ids!(check_visible),
                B0Action::SetVisible as fn(bool) -> B0Action,
            ),
            (ids!(check_locked), B0Action::SetLocked),
        ] {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyCheckBox>()
                .and_then(|mut checkbox| checkbox.take_change());
            if let Some(asked) = asked {
                outbox.push(make(asked));
            }
        }
        for (id, make) in [
            (
                ids!(toggle_compact),
                B0Action::SetCompact as fn(bool) -> B0Action,
            ),
            (ids!(toggle_busy), B0Action::SetBusy),
        ] {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyToggle>()
                .and_then(|mut toggle| toggle.take_change());
            if let Some(asked) = asked {
                outbox.push(make(asked));
            }
        }
        for (index, id) in [ids!(radio_first), ids!(radio_second)]
            .into_iter()
            .enumerate()
        {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyRadio>()
                .and_then(|mut radio| radio.take_change());
            if asked.is_some() {
                outbox.push(B0Action::Choose(index));
            }
        }
        for (id, make) in [
            (ids!(slider_size), B0Action::SetSize as fn(f64) -> B0Action),
            (ids!(slider_weight), B0Action::SetWeight),
        ] {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifySlider>()
                .and_then(|mut slider| slider.take_change());
            if let Some(asked) = asked {
                outbox.push(make(asked));
            }
        }
        for id in [ids!(tabs_primary), ids!(tabs_secondary)] {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyTabBar>()
                .and_then(|mut bar| bar.take_change());
            if let Some(asked) = asked {
                outbox.push(B0Action::SetTab(asked));
            }
        }
        for (id, action) in [
            (ids!(chooser_palette), B0Action::NextPalette),
            (ids!(chooser_density), B0Action::NextDensity),
        ] {
            let opened = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyDropDown>()
                .and_then(|mut chooser| chooser.take_open());
            if opened.is_some() {
                outbox.push(action);
            }
        }

        let drawn: Vec<(&'static str, LocalRect)> = CONTROLS
            .iter()
            .filter_map(|name| self.rect(cx, name))
            .collect();
        if !drawn.is_empty() && drawn != self.layout {
            self.layout = drawn.clone();
            outbox.push(B0Action::Layout(drawn));
        }
    }
}
