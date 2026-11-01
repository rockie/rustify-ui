use super::{Scrollbars, Visibility};
use crate::Orientation;
use rustify_ui::Direction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AxisMetrics {
    pub overflow: bool,
    pub max_scroll: f64,
    pub thumb: f64,
    pub offset: f64,
    pub travel: f64,
}
impl Default for AxisMetrics {
    fn default() -> Self {
        Self {
            overflow: false,
            max_scroll: 0.,
            thumb: 0.,
            offset: 0.,
            travel: 0.,
        }
    }
}
impl AxisMetrics {
    pub fn measure(viewport: f64, content: f64, track: f64, scroll: f64, reverse: bool) -> Self {
        if viewport <= 0. || content <= viewport {
            return Self::default();
        }
        let max_scroll = content - viewport;
        let thumb = (track * viewport / content).max(18.).min(track);
        let travel = (track - thumb).max(0.);
        let fraction = (scroll / max_scroll).clamp(0., 1.);
        Self {
            overflow: true,
            max_scroll,
            thumb,
            offset: travel * if reverse { 1. - fraction } else { fraction },
            travel,
        }
    }
    pub fn scroll_from_pointer(self, pointer: f64, grab: f64, reverse: bool) -> f64 {
        if self.travel <= 0. {
            return 0.;
        }
        let fraction = ((pointer - grab) / self.travel).clamp(0., 1.);
        self.max_scroll * if reverse { 1. - fraction } else { fraction }
    }
}

pub(super) fn logical_left(native: f64, maximum: f64, direction: Direction) -> f64 {
    let value = if direction == Direction::RightToLeft {
        -native
    } else {
        native
    };
    value.clamp(0., maximum)
}
pub(super) fn native_left(logical: f64, direction: Direction) -> f64 {
    if direction == Direction::RightToLeft {
        -logical
    } else {
        logical
    }
}
pub(super) fn enabled(scrollbars: Scrollbars, orientation: Orientation) -> bool {
    scrollbars == Scrollbars::Both
        || matches!(
            (scrollbars, orientation),
            (Scrollbars::Horizontal, Orientation::Horizontal)
                | (Scrollbars::Vertical, Orientation::Vertical)
        )
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Activity {
    pub hovered: bool,
    pub focused: bool,
    pub dragging: bool,
    pub scrolling: bool,
    pub lingering: bool,
}
impl Activity {
    pub fn held(self) -> bool {
        self.hovered || self.focused || self.dragging
    }
    pub fn visible(self, visibility: Visibility, overflow: bool) -> bool {
        match visibility {
            Visibility::Always => true,
            Visibility::Auto => overflow,
            Visibility::Hover => overflow && (self.held() || self.lingering),
            Visibility::Scroll => overflow && (self.dragging || self.scrolling || self.lingering),
        }
    }
    pub fn idle(&mut self) {
        self.scrolling = false;
        self.lingering = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn thumb_tracks_native_scroll_with_minimum_and_track_saturation() {
        let start = AxisMetrics::measure(100., 1000., 200., 0., false);
        assert_eq!((start.thumb, start.offset, start.travel), (20., 0., 180.));
        let middle = AxisMetrics::measure(100., 1000., 200., 450., false);
        assert_eq!(middle.offset, 90.);
        assert_eq!(middle.scroll_from_pointer(90., 0., false), 450.);
        assert_eq!(
            AxisMetrics::measure(10., 10000., 50., 9990., false).thumb,
            18.
        );
        let tiny = AxisMetrics::measure(10., 10000., 12., 9990., false);
        assert_eq!((tiny.thumb, tiny.travel, tiny.offset), (12., 0., 0.));
        assert_eq!(tiny.scroll_from_pointer(30., 0., false), 0.);
        assert_eq!(
            AxisMetrics::measure(100., 100., 200., 0., false),
            AxisMetrics::default()
        );
        assert_eq!(
            AxisMetrics::measure(0., 100., 0., 0., false),
            AxisMetrics::default()
        );
    }
    #[test]
    fn rtl_native_reads_writes_and_dragging_share_one_logical_mapping() {
        for direction in [Direction::LeftToRight, Direction::RightToLeft] {
            let reverse = direction == Direction::RightToLeft;
            for logical in [0., 150., 300.] {
                let native = native_left(logical, direction);
                assert_eq!(logical_left(native, 300., direction), logical);
                let metrics = AxisMetrics::measure(100., 400., 100., logical, reverse);
                assert_eq!(
                    metrics.scroll_from_pointer(metrics.offset + 5., 5., reverse),
                    logical
                );
            }
        }
        assert_eq!(logical_left(-600., 300., Direction::RightToLeft), 300.);
        assert_eq!(logical_left(20., 300., Direction::RightToLeft), 0.);
        let rtl = AxisMetrics::measure(100., 400., 100., 0., true);
        assert_eq!(rtl.offset, rtl.travel);
        assert_eq!(rtl.scroll_from_pointer(0., 0., true), 300.);
        assert!((rtl.scroll_from_pointer(25., 5., true) - 220.).abs() < 1e-9);
    }
    #[test]
    fn visibility_preserves_hover_focus_drag_and_the_delayed_exit() {
        let mut activity = Activity::default();
        assert!(!activity.visible(Visibility::Hover, true));
        assert!(activity.visible(Visibility::Auto, true));
        assert!(activity.visible(Visibility::Always, false));
        activity.hovered = true;
        assert!(activity.visible(Visibility::Hover, true));
        activity.hovered = false;
        activity.lingering = true;
        assert!(activity.visible(Visibility::Hover, true));
        activity.idle();
        assert!(!activity.visible(Visibility::Hover, true));
        activity.scrolling = true;
        assert!(activity.visible(Visibility::Scroll, true));
        assert!(!activity.visible(Visibility::Hover, true));
        activity.dragging = true;
        activity.idle();
        assert!(activity.visible(Visibility::Scroll, true));
        assert!(!activity.visible(Visibility::Scroll, false));
        activity.dragging = false;
        activity.focused = true;
        assert!(activity.visible(Visibility::Hover, true));
        assert!(!activity.visible(Visibility::Scroll, true));
        activity.scrolling = true;
        assert!(activity.visible(Visibility::Scroll, true));
    }
    #[test]
    fn disabled_axes_never_publish_a_custom_bar() {
        assert!(enabled(Scrollbars::Both, Orientation::Horizontal));
        assert!(enabled(Scrollbars::Both, Orientation::Vertical));
        assert!(!enabled(Scrollbars::Vertical, Orientation::Horizontal));
        assert!(!enabled(Scrollbars::Horizontal, Orientation::Vertical));
    }
}
