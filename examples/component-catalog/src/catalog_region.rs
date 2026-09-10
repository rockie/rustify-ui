use rustify_components::Category;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{
    LocalRect, RegionApp, RegionGlyph, RustifyCheckBox, RustifyDropDown, RustifyIcon,
    RustifyProgress, RustifyRadio, RustifySlider, RustifySpinner, RustifyTabBar, RustifyToggle,
    Theme,
};

/// What the region is told: the scope's theme, which category the catalogue is
/// showing, and the values of the controls on that page.
///
/// One props type for eighteen pages rather than eighteen: the region draws
/// whichever control the category names and hides the rest, so the page a
/// reader is on is a value and not a different region.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogProps {
    pub theme: Theme,
    pub category: Category,
    /// The values the DOM half is bound to. Both halves read these, which is
    /// the point: a reader changes one and watches the other follow.
    pub checked: bool,
    pub chosen: usize,
    pub on: bool,
    pub size: f64,
    pub fraction: f64,
    pub spinning: bool,
    pub tab: usize,
    /// What the chooser shows while closed. The list is the DOM's.
    pub chooser: String,
    /// The text a field holds, drawn as text because a region is not a text
    /// editor (P1 M5: editing happens in a native control over the rectangle).
    pub text: String,
    /// What to say for a category no region draws.
    pub note: String,
    pub disabled: bool,
    pub read_only: bool,
}

#[derive(Debug)]
pub enum CatalogAction {
    /// The user clicked the region's own theme button. The catalogue answers
    /// by switching the theme, which is how a reader sees that the request
    /// came from the GPU half and the answer came back to both.
    Toggle,
    /// Where the region drew that button, in its own local CSS pixels.
    Button(LocalRect),
    /// Where it drew the control for the current category, so a test can put
    /// a real pointer on it without a second copy of the layout.
    Control(LocalRect),
    SetChecked(bool),
    Choose(usize),
    SetOn(bool),
    SetSize(f64),
    SetTab(usize),
    /// The region's chooser was asked to open. The list is a DOM layer
    /// anchored to the rectangle, because a popup cannot leave the canvas.
    OpenChooser(LocalRect),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(CatalogRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    frame := RoundedView{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 8
                        padding: 10

                        // Two rows rather than one: the button's own width
                        // plus four swatches plus the label does not fit
                        // across the region, and a control drawn past the
                        // edge is a control a pointer cannot reach.
                        header := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 6

                            swatches := View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 6
                                align: Center

                                name_label := Label{
                                    text: "theme"
                                    draw_text.text_style.font_size: 12
                                }
                                primary := RoundedView{ width: 26 height: 18 }
                                accent := RoundedView{ width: 26 height: 18 }
                                destructive := RoundedView{ width: 26 height: 18 }
                                border_swatch := RoundedView{ width: 26 height: 18 }
                            }
                            toggle_button := Button{ text: "switch from the region" }
                        }

                        stage := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 6

                            note_slot := View{
                                width: Fill
                                height: Fit
                                visible: false
                                note_label := Label{
                                    width: Fill
                                    text: ""
                                    draw_text.text_style.font_size: 12
                                }
                            }
                            text_slot := View{
                                width: Fill
                                height: Fit
                                visible: false
                                text_label := Label{
                                    width: Fill
                                    text: ""
                                    draw_text.text_style.font_size: 12
                                }
                            }
                            button_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_button := Button{ text: "a button drawn here" }
                            }
                            checkbox_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_checkbox := RustifyCheckBox{}
                            }
                            radio_slot := View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 8
                                visible: false
                                radio_first := RustifyRadio{}
                                radio_second := RustifyRadio{}
                                radio_third := RustifyRadio{}
                            }
                            switch_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_switch := RustifyToggle{}
                            }
                            slider_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_slider := RustifySlider{}
                            }
                            progress_slot := View{
                                width: Fill
                                height: Fit
                                visible: false
                                gpu_progress := RustifyProgress{}
                            }
                            spinner_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_spinner := RustifySpinner{}
                            }
                            icon_slot := View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 8
                                visible: false
                                icon_check := RustifyIcon{}
                                icon_chevron := RustifyIcon{}
                                icon_close := RustifyIcon{}
                            }
                            chooser_slot := View{
                                width: Fit
                                height: Fit
                                visible: false
                                gpu_chooser := RustifyDropDown{}
                            }
                            tabs_slot := View{
                                width: Fill
                                height: Fit
                                visible: false
                                gpu_tabs := RustifyTabBar{}
                            }
                            scroll_slot := View{
                                width: Fill
                                height: 64
                                flow: Down
                                visible: false
                                scroll_bars: ScrollBars{}

                                scroll_1 := Label{ text: "one" }
                                scroll_2 := Label{ text: "two" }
                                scroll_3 := Label{ text: "three" }
                                scroll_4 := Label{ text: "four" }
                                scroll_5 := Label{ text: "five" }
                                scroll_6 := Label{ text: "six" }
                                scroll_7 := Label{ text: "seven" }
                                scroll_8 := Label{ text: "eight" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct CatalogRegion {
    #[live]
    ui: WidgetRef,
    /// The last rectangles reported, so only a change is sent.
    #[rust]
    button: Option<LocalRect>,
    #[rust]
    control: Option<LocalRect>,
    #[rust]
    category: Option<Category>,
}

fn colour(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}

/// Which slot a category is drawn in. Every category has one: the four no
/// region draws get the note slot, which says where they are drawn instead
/// rather than leaving an empty rectangle.
fn slot(category: Category) -> &'static str {
    match category {
        Category::Button => "button_slot",
        Category::Checkbox => "checkbox_slot",
        Category::Radio => "radio_slot",
        Category::Switch => "switch_slot",
        Category::Slider => "slider_slot",
        Category::Progress => "progress_slot",
        Category::Loading => "spinner_slot",
        Category::Icon => "icon_slot",
        Category::Select => "chooser_slot",
        Category::Tabs => "tabs_slot",
        Category::ScrollArea => "scroll_slot",
        Category::TextField | Category::TextArea => "text_slot",
        Category::Label
        | Category::Link
        | Category::Tooltip
        | Category::Menu
        | Category::Dialog => "note_slot",
    }
}

impl CatalogRegion {
    /// The rectangle the current category's control ended up in, in the
    /// region's own local pixels.
    fn control_rect(&mut self, cx: &mut Cx, category: Category) -> Option<LocalRect> {
        let area = match category {
            Category::Button => self.ui.widget(cx, ids!(gpu_button)).area(),
            Category::Checkbox => self.ui.widget(cx, ids!(gpu_checkbox)).area(),
            Category::Radio => self.ui.widget(cx, ids!(radio_first)).area(),
            Category::Switch => self.ui.widget(cx, ids!(gpu_switch)).area(),
            Category::Slider => self.ui.widget(cx, ids!(gpu_slider)).area(),
            Category::Progress => self.ui.widget(cx, ids!(gpu_progress)).area(),
            Category::Loading => self.ui.widget(cx, ids!(gpu_spinner)).area(),
            Category::Icon => self.ui.widget(cx, ids!(icon_check)).area(),
            Category::Select => self.ui.widget(cx, ids!(gpu_chooser)).area(),
            Category::Tabs => self.ui.widget(cx, ids!(gpu_tabs)).area(),
            Category::ScrollArea => self.ui.widget(cx, ids!(scroll_slot)).area(),
            _ => return None,
        };
        let rect = area.rect(cx);
        (rect.size.x > 0.0)
            .then(|| LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y))
    }
}

impl RegionApp for CatalogRegion {
    type Props = CatalogProps;
    type Action = CatalogAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &CatalogProps) {
        let theme = props.theme;
        self.category = Some(props.category);

        self.ui
            .label(cx, ids!(name_label))
            .set_text(cx, &format!("theme: {}", theme.name));
        self.ui
            .label(cx, ids!(name_label))
            .set_text_color(cx, colour(theme.foreground));

        let mut frame = self.ui.widget(cx, ids!(frame));
        let background = colour(theme.background);
        let border = colour(theme.border);
        let radius = theme.radius as f32;
        script_apply_eval!(cx, frame, {
            draw_bg +: {
                color: #(background)
                border_color: #(border)
                border_size: 1.0
                border_radius: #(radius)
            }
        });

        for (id, value) in [
            (ids!(primary), theme.primary),
            (ids!(accent), theme.accent),
            (ids!(destructive), theme.destructive),
            (ids!(border_swatch), theme.border),
        ] {
            let mut swatch = self.ui.widget(cx, id);
            let value = colour(value);
            script_apply_eval!(cx, swatch, {
                draw_bg +: {
                    color: #(value)
                    border_radius: 4.0
                }
            });
        }

        // One slot showing, the rest away. A hidden slot still exists, so the
        // control in it keeps its state while a reader is on another page.
        let showing = slot(props.category);
        for (name, id) in [
            ("note_slot", ids!(note_slot)),
            ("text_slot", ids!(text_slot)),
            ("button_slot", ids!(button_slot)),
            ("checkbox_slot", ids!(checkbox_slot)),
            ("radio_slot", ids!(radio_slot)),
            ("switch_slot", ids!(switch_slot)),
            ("slider_slot", ids!(slider_slot)),
            ("progress_slot", ids!(progress_slot)),
            ("spinner_slot", ids!(spinner_slot)),
            ("icon_slot", ids!(icon_slot)),
            ("chooser_slot", ids!(chooser_slot)),
            ("tabs_slot", ids!(tabs_slot)),
            ("scroll_slot", ids!(scroll_slot)),
        ] {
            self.ui.widget(cx, id).set_visible(cx, showing == name);
        }

        self.ui
            .label(cx, ids!(note_label))
            .set_text(cx, &props.note);
        self.ui
            .label(cx, ids!(note_label))
            .set_text_color(cx, colour(theme.muted_foreground));
        self.ui
            .label(cx, ids!(text_label))
            .set_text(cx, &props.text);
        self.ui
            .label(cx, ids!(text_label))
            .set_text_color(cx, colour(theme.foreground));

        if let Some(mut checkbox) = self
            .ui
            .widget(cx, ids!(gpu_checkbox))
            .borrow_mut::<RustifyCheckBox>()
        {
            checkbox.set_state(cx, props.checked, props.disabled, props.read_only);
            checkbox.set_palette(cx, theme.input, theme.primary, theme.border);
        }

        for (index, id) in [ids!(radio_first), ids!(radio_second), ids!(radio_third)]
            .into_iter()
            .enumerate()
        {
            if let Some(mut radio) = self.ui.widget(cx, id).borrow_mut::<RustifyRadio>() {
                radio.set_state(cx, props.chosen == index, props.disabled, props.read_only);
                radio.set_palette(cx, theme.input, theme.primary, theme.border);
            }
        }

        if let Some(mut switch) = self
            .ui
            .widget(cx, ids!(gpu_switch))
            .borrow_mut::<RustifyToggle>()
        {
            switch.set_state(cx, props.on, props.disabled, props.read_only);
            switch.set_palette(cx, theme.border, theme.primary, theme.background);
        }

        if let Some(mut slider) = self
            .ui
            .widget(cx, ids!(gpu_slider))
            .borrow_mut::<RustifySlider>()
        {
            slider.set_range(cx, 0.0, 100.0, 5.0);
            slider.set_state(cx, props.size, props.disabled, props.read_only);
            slider.set_palette(cx, theme.border, theme.primary, theme.background);
        }

        if let Some(mut progress) = self
            .ui
            .widget(cx, ids!(gpu_progress))
            .borrow_mut::<RustifyProgress>()
        {
            progress.set_fraction(cx, props.fraction);
            progress.set_palette(cx, theme.secondary, theme.primary);
        }

        if let Some(mut spinner) = self
            .ui
            .widget(cx, ids!(gpu_spinner))
            .borrow_mut::<RustifySpinner>()
        {
            spinner.set_state(cx, props.spinning, theme.reduce_motion);
            spinner.set_ink(cx, theme.primary);
        }

        for (id, glyph) in [
            (ids!(icon_check), RegionGlyph::Check),
            (ids!(icon_chevron), RegionGlyph::ChevronDown),
            (ids!(icon_close), RegionGlyph::Close),
        ] {
            if let Some(mut icon) = self.ui.widget(cx, id).borrow_mut::<RustifyIcon>() {
                icon.set_glyph(cx, glyph);
                icon.set_ink(cx, theme.foreground);
            }
        }

        if let Some(mut chooser) = self
            .ui
            .widget(cx, ids!(gpu_chooser))
            .borrow_mut::<RustifyDropDown>()
        {
            chooser.set_state(cx, &props.chooser, props.disabled, props.read_only);
            chooser.set_palette(
                cx,
                theme.input,
                theme.border,
                theme.foreground,
                theme.muted_foreground,
            );
        }

        if let Some(mut tab_bar) = self
            .ui
            .widget(cx, ids!(gpu_tabs))
            .borrow_mut::<RustifyTabBar>()
        {
            let tabs: Vec<String> = ["first", "second", "third"]
                .iter()
                .map(|name| name.to_string())
                .collect();
            tab_bar.set_state(cx, &tabs, props.tab, props.disabled);
            tab_bar.set_palette(cx, theme.muted, theme.background, theme.foreground);
        }

        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<CatalogAction>) {
        if let Event::Actions(actions) = event {
            if self.ui.button(cx, ids!(toggle_button)).clicked(actions) {
                outbox.push(CatalogAction::Toggle);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());

        let asked = self
            .ui
            .widget(cx, ids!(gpu_checkbox))
            .borrow_mut::<RustifyCheckBox>()
            .and_then(|mut checkbox| checkbox.take_change());
        if let Some(asked) = asked {
            outbox.push(CatalogAction::SetChecked(asked));
        }
        for (index, id) in [ids!(radio_first), ids!(radio_second), ids!(radio_third)]
            .into_iter()
            .enumerate()
        {
            let asked = self
                .ui
                .widget(cx, id)
                .borrow_mut::<RustifyRadio>()
                .and_then(|mut radio| radio.take_change());
            if asked.is_some() {
                outbox.push(CatalogAction::Choose(index));
            }
        }
        let asked = self
            .ui
            .widget(cx, ids!(gpu_switch))
            .borrow_mut::<RustifyToggle>()
            .and_then(|mut switch| switch.take_change());
        if let Some(asked) = asked {
            outbox.push(CatalogAction::SetOn(asked));
        }
        let asked = self
            .ui
            .widget(cx, ids!(gpu_slider))
            .borrow_mut::<RustifySlider>()
            .and_then(|mut slider| slider.take_change());
        if let Some(asked) = asked {
            outbox.push(CatalogAction::SetSize(asked));
        }
        let asked = self
            .ui
            .widget(cx, ids!(gpu_tabs))
            .borrow_mut::<RustifyTabBar>()
            .and_then(|mut tabs| tabs.take_change());
        if let Some(asked) = asked {
            outbox.push(CatalogAction::SetTab(asked));
        }
        let opened = self
            .ui
            .widget(cx, ids!(gpu_chooser))
            .borrow_mut::<RustifyDropDown>()
            .and_then(|mut chooser| chooser.take_open());
        if opened.is_some() {
            if let Some(rect) = self.control_rect(cx, Category::Select) {
                outbox.push(CatalogAction::OpenChooser(rect));
            }
        }

        // Where things ended up, not where the layout walked: a parent that
        // aligns its children moves them after the walk, and what a pointer
        // has to be aimed at is the former.
        let area = self.ui.widget(cx, ids!(toggle_button)).area();
        let rect = area.rect(cx);
        let drawn = LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y);
        if drawn.width > 0.0 && self.button != Some(drawn) {
            self.button = Some(drawn);
            outbox.push(CatalogAction::Button(drawn));
        }
        if let Some(category) = self.category {
            let control = self.control_rect(cx, category);
            if control.is_some() && self.control != control {
                self.control = control;
                outbox.push(CatalogAction::Control(control.expect("just checked")));
            }
        }
    }
}
