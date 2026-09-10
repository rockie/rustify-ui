//! The GPU half of the controls that have one on both sides of the boundary.
//!
//! None of these widgets holds the value it shows. A click is a request the
//! application answers by handing back a new projection, exactly as the DOM
//! half works, so one value cannot mean two things on one screen. A disabled
//! or read-only control makes no request at all.
//!
//! The shapes are drawn in the script beside each widget rather than in Rust:
//! a circle, a pill and a check mark are three lines of a signed-distance
//! shader and a dozen quads otherwise. What Rust decides is which parts to
//! draw and where, which is where the state lives.

mod button;
mod lists;
mod marks;
mod toggles;
mod values;

pub use button::RustifyButton;
pub use lists::{RustifyDropDown, RustifyTabBar};
pub use marks::{Glyph, RustifyIcon, RustifySpinner};
pub use toggles::{RustifyCheckBox, RustifyRadio, RustifyToggle};
pub use values::{RustifyProgress, RustifySlider};

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*

    /* A filled rectangle with rounded ends, sized to whatever it is drawn
       into. Both toggles and both bars use it. */
    mod.widgets.RustifyPill = mod.draw.DrawColor{
        pixel: fn(){
            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
            let radius = min(self.rect_size.x, self.rect_size.y) * 0.5
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, radius)
            return sdf.fill(self.color)
        }
    }

    /* A filled circle, centred and as large as it fits. */
    mod.widgets.RustifyDisc = mod.draw.DrawColor{
        pixel: fn(){
            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
            let radius = min(self.rect_size.x, self.rect_size.y) * 0.5
            sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, radius)
            return sdf.fill(self.color)
        }
    }

    /* A box with the scope's own corner. The radius is a uniform so a region
       can hand it the theme's, rather than every control agreeing by luck. */
    mod.widgets.RustifyBox = mod.draw.DrawColor{
        corner: uniform(6.0)
        pixel: fn(){
            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.corner)
            return sdf.fill(self.color)
        }
    }

    mod.widgets.RustifyButtonBase = #(RustifyButton::register_widget(vm))

    mod.widgets.RustifyButton = set_type_default() do mod.widgets.RustifyButtonBase{
        width: 170
        height: 30
        draw_bg: mod.widgets.RustifyBox{ color: #xeaecf0 }
        draw_border: mod.widgets.RustifyBox{ color: #xd0d5dd }
        draw_text +: {
            color: #x1d2939
            text_style: theme.font_regular{ font_size: 12.0 }
        }
    }

    mod.widgets.RustifyCheckBoxBase = #(RustifyCheckBox::register_widget(vm))

    mod.widgets.RustifyCheckBox = set_type_default() do mod.widgets.RustifyCheckBoxBase{
        width: 24
        height: 24
        draw_bg +: {
            color: #x101828
        }
        draw_mark +: {
            color: #x2e90fa
        }
        draw_border +: {
            color: #xd0d5dd
        }
    }

    mod.widgets.RustifySliderBase = #(RustifySlider::register_widget(vm))

    mod.widgets.RustifySlider = set_type_default() do mod.widgets.RustifySliderBase{
        width: 140
        height: 24
        draw_bg +: {
            color: #x101828
        }
        draw_fill +: {
            color: #x2e90fa
        }
        draw_knob +: {
            color: #xffffff
        }
    }

    mod.widgets.RustifyRadioBase = #(RustifyRadio::register_widget(vm))

    mod.widgets.RustifyRadio = set_type_default() do mod.widgets.RustifyRadioBase{
        width: 20
        height: 20
        draw_border: mod.widgets.RustifyDisc{ color: #xd0d5dd }
        draw_bg: mod.widgets.RustifyDisc{ color: #x101828 }
        draw_mark: mod.widgets.RustifyDisc{ color: #x2e90fa }
    }

    mod.widgets.RustifyToggleBase = #(RustifyToggle::register_widget(vm))

    mod.widgets.RustifyToggle = set_type_default() do mod.widgets.RustifyToggleBase{
        width: 44
        height: 24
        draw_track: mod.widgets.RustifyPill{ color: #xd0d5dd }
        draw_knob: mod.widgets.RustifyDisc{ color: #xffffff }
    }

    mod.widgets.RustifyProgressBase = #(RustifyProgress::register_widget(vm))

    mod.widgets.RustifyProgress = set_type_default() do mod.widgets.RustifyProgressBase{
        width: Fill
        height: 8
        draw_bg: mod.widgets.RustifyPill{ color: #xeaecf0 }
        draw_fill: mod.widgets.RustifyPill{ color: #x2e90fa }
    }

    mod.widgets.RustifySpinnerBase = #(RustifySpinner::register_widget(vm))

    /* The arc is the shader's; what turns it is the pass clock, and what keeps
       the pass coming is the widget asking for another frame while it is
       spinning. A scope that asks for less motion gets the same arc, still. */
    mod.widgets.RustifySpinner = set_type_default() do mod.widgets.RustifySpinnerBase{
        width: 20
        height: 20
        draw_bg +: {
            color: #x2e90fa
            turning: uniform(1.0)
            stroke_width: uniform(2.5)

            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let radius = min(self.rect_size.x, self.rect_size.y) * 0.5 - self.stroke_width
                let start = self.draw_pass.time * 2.0 * PI * self.turning
                sdf.arc_round_caps(
                    self.rect_size.x * 0.5
                    self.rect_size.y * 0.5
                    radius
                    start
                    start + 4.4
                    self.stroke_width
                )
                return sdf.fill(self.color)
            }
        }
    }

    mod.widgets.RustifyIconBase = #(RustifyIcon::register_widget(vm))

    /* One widget, six drawings, and the one Rust asks for is the one drawn.
       The grid is the same twenty-four units the DOM glyphs are drawn on, so
       the two halves are the same shapes at different sizes. */
    mod.widgets.RustifyIcon = set_type_default() do mod.widgets.RustifyIconBase{
        width: 20
        height: 20
        draw_check +: {
            color: #x1d2939
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(4.0 * s, 12.0 * s)
                sdf.line_to(9.0 * s, 17.0 * s)
                sdf.line_to(20.0 * s, 6.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_chevron_down +: {
            color: #x1d2939
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(6.0 * s, 9.0 * s)
                sdf.line_to(12.0 * s, 15.0 * s)
                sdf.line_to(18.0 * s, 9.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_chevron_up +: {
            color: #x1d2939
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(6.0 * s, 15.0 * s)
                sdf.line_to(12.0 * s, 9.0 * s)
                sdf.line_to(18.0 * s, 15.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_chevron_right +: {
            color: #x1d2939
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(9.0 * s, 6.0 * s)
                sdf.line_to(15.0 * s, 12.0 * s)
                sdf.line_to(9.0 * s, 18.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_close +: {
            color: #x1d2939
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(6.0 * s, 6.0 * s)
                sdf.line_to(18.0 * s, 18.0 * s)
                sdf.stroke(self.color, 2.0 * s)
                sdf.move_to(18.0 * s, 6.0 * s)
                sdf.line_to(6.0 * s, 18.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_dot: mod.widgets.RustifyDisc{ color: #x1d2939 }
    }

    mod.widgets.RustifyDropDownBase = #(RustifyDropDown::register_widget(vm))

    mod.widgets.RustifyDropDown = set_type_default() do mod.widgets.RustifyDropDownBase{
        width: 180
        height: 32
        draw_bg: mod.widgets.RustifyBox{ color: #xffffff }
        draw_border: mod.widgets.RustifyBox{ color: #xd0d5dd }
        draw_chevron +: {
            color: #x667085
            pixel: fn(){
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                let s = min(self.rect_size.x, self.rect_size.y) / 24.0
                sdf.move_to(6.0 * s, 9.0 * s)
                sdf.line_to(12.0 * s, 15.0 * s)
                sdf.line_to(18.0 * s, 9.0 * s)
                return sdf.stroke(self.color, 2.0 * s)
            }
        }
        draw_text +: {
            color: #x1d2939
            text_style: theme.font_regular{ font_size: 12.0 }
        }
    }

    mod.widgets.RustifyTabBarBase = #(RustifyTabBar::register_widget(vm))

    mod.widgets.RustifyTabBar = set_type_default() do mod.widgets.RustifyTabBarBase{
        width: Fill
        height: 32
        draw_bg: mod.widgets.RustifyBox{ color: #xf2f4f7 }
        draw_tab: mod.widgets.RustifyBox{ color: #xffffff }
        draw_text +: {
            color: #x1d2939
            text_style: theme.font_regular{ font_size: 12.0 }
        }
    }
}

/// A token colour, opaque.
pub(crate) fn colour(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}
