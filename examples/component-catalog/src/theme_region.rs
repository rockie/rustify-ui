use rustify_ui::makepad_widgets::*;
use rustify_ui::{LocalRect, RegionApp, Theme};

/// What the region is told: the scope's theme, and nothing else.
///
/// The catalogue's GPU half exists to answer one question - does a theme
/// change reach the pixels in the same breath as it reaches the DOM - so the
/// region draws the tokens themselves rather than a component that happens to
/// use them.
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeProps {
    pub theme: Theme,
}

#[derive(Debug)]
pub enum ThemeAction {
    /// The user clicked the region's own button. The catalogue answers by
    /// switching the theme, which is how a reader sees that the request came
    /// from the GPU half and the answer came back to both.
    Toggle,
    /// Where the region drew that button, in its own local CSS pixels.
    /// Reported when it changes, so a caller can aim a pointer at it without
    /// keeping a second copy of the region's layout.
    Button(LocalRect),
}

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(ThemeRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    frame := RoundedView{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 8
                        padding: 12

                        name_label := Label{
                            text: "theme"
                            draw_text.text_style.font_size: 13
                        }
                        swatches := View{
                            width: Fill
                            height: 28
                            flow: Right
                            spacing: 6

                            primary := RoundedView{ width: 48 height: Fill }
                            accent := RoundedView{ width: 48 height: Fill }
                            destructive := RoundedView{ width: 48 height: Fill }
                            border_swatch := RoundedView{ width: 48 height: Fill }
                        }
                        toggle_button := Button{ text: "switch from the region" }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct ThemeRegion {
    #[live]
    ui: WidgetRef,
    /// The last rectangle reported, so only a change is sent.
    #[rust]
    button: Option<LocalRect>,
}

fn colour(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}

impl RegionApp for ThemeRegion {
    type Props = ThemeProps;
    type Action = ThemeAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &ThemeProps) {
        let theme = props.theme;
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
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<ThemeAction>) {
        if let Event::Actions(actions) = event {
            if self.ui.button(cx, ids!(toggle_button)).clicked(actions) {
                outbox.push(ThemeAction::Toggle);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        // Where it ended up, not where the layout walked: a parent that aligns
        // its children moves them after the walk, and what a pointer has to be
        // aimed at is the former.
        let area = self.ui.widget(cx, ids!(toggle_button)).area();
        let rect = area.rect(cx);
        let drawn = LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y);
        if drawn.width > 0.0 && self.button != Some(drawn) {
            self.button = Some(drawn);
            outbox.push(ThemeAction::Button(drawn));
        }
    }
}
