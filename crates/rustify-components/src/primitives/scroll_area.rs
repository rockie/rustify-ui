//! One native scrollport with custom bars derived from its actual dimensions and scroll position.

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod model;

use crate::geometry::{PropError, StyleSet};
use crate::Orientation;
use leptos::prelude::*;
use model::{Activity, AxisMetrics};
use rustify_ui::Direction;

/// Whether a scrollport passes a boundary scroll to its ancestors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Boundary {
    #[default]
    Propagate,
    Stop,
}
impl Boundary {
    pub const ALL: &'static [Self] = &[Self::Propagate, Self::Stop];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Propagate => "propagate",
            Self::Stop => "stop",
        }
    }
}

/// Axes enabled on the native scrollport and its custom bars.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scrollbars {
    Vertical,
    Horizontal,
    #[default]
    Both,
}
impl Scrollbars {
    pub const ALL: &'static [Self] = &[Self::Vertical, Self::Horizontal, Self::Both];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        }
    }
}

/// When custom tracks are visible; the native scroll position is unchanged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Visibility {
    Auto,
    Always,
    #[default]
    Hover,
    Scroll,
}
impl Visibility {
    pub const ALL: &'static [Self] = &[Self::Auto, Self::Always, Self::Hover, Self::Scroll];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Hover => "hover",
            Self::Scroll => "scroll",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Delay {
    ms: i32,
    issue: Option<PropError>,
}
fn accept_delay(value: u32, previous: Option<&Delay>) -> Delay {
    match i32::try_from(value) {
        Ok(ms) => Delay { ms, issue: None },
        Err(_) => Delay {
            ms: previous.map_or(600, |previous| previous.ms),
            issue: Some(PropError {
                field: "hide_delay_ms",
                message: "duration exceeds the browser timer range".into(),
            }),
        },
    }
}

fn index(orientation: Orientation) -> usize {
    if orientation == Orientation::Horizontal {
        0
    } else {
        1
    }
}
#[derive(Clone, Copy)]
struct Context {
    viewport_id: StoredValue<String>,
    root: NodeRef<leptos::html::Div>,
    viewport: NodeRef<leptos::html::Div>,
    content: NodeRef<leptos::html::Div>,
    tracks: [NodeRef<leptos::html::Div>; 2],
    viewports: RwSignal<u32>,
    track_counts: RwSignal<[u32; 2]>,
    metrics: RwSignal<[AxisMetrics; 2]>,
    activity: RwSignal<Activity>,
    scrollbars: Signal<Scrollbars>,
    visibility: Signal<Visibility>,
    delay: Memo<Delay>,
    boundary: Signal<Boundary>,
    direction: Signal<Direction>,
    label: Signal<String>,
    labelled_by: Signal<String>,
    #[cfg(target_arch = "wasm32")]
    browser: StoredValue<browser::State, leptos::reactive::owner::LocalStorage>,
}
impl Context {
    fn visible(self, orientation: Orientation) -> bool {
        self.track_counts.get()[index(orientation)] == 1
            && model::enabled(self.scrollbars.get(), orientation)
            && self.activity.get().visible(
                self.visibility.get(),
                self.metrics.get()[index(orientation)].overflow,
            )
    }
    fn issue(self) -> Option<&'static str> {
        if self.viewports.get() > 1 || self.track_counts.get().iter().any(|count| *count > 1) {
            Some("duplicate-part")
        } else if self.delay.get().issue.is_some() {
            Some("hide_delay_ms")
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
struct TrackContext {
    area: Context,
    orientation: Orientation,
}

/// Provides one native scrollport and its decorative, pointer-operable scrollbar parts.
#[component]
pub fn Root(
    #[prop(optional, into)] scrollbars: Signal<Scrollbars>,
    #[prop(optional, into)] visibility: Signal<Visibility>,
    #[prop(default = Signal::stored(600), into)] hide_delay_ms: Signal<u32>,
    #[prop(optional, into)] boundary: Signal<Boundary>,
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional, into)] labelled_by: Signal<String>,
    #[prop(optional_no_strip, into)] direction: Option<Signal<Direction>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let context = Context {
            viewport_id: StoredValue::new(crate::id::next("scroll-viewport")),
            root: NodeRef::new(), viewport: NodeRef::new(), content: NodeRef::new(), tracks: [NodeRef::new(), NodeRef::new()],
            viewports: RwSignal::new(0), track_counts: RwSignal::new([0, 0]), metrics: RwSignal::new([AxisMetrics::default(); 2]),
            activity: RwSignal::new(Activity::default()), scrollbars, visibility,
            delay: Memo::new(move |previous| accept_delay(hide_delay_ms.get(), previous)),
            boundary, direction: direction.unwrap_or_else(rustify_ui::use_direction), label: aria_label, labelled_by,
            #[cfg(target_arch = "wasm32")]
            browser: StoredValue::new_local(browser::State::default()),
        };
        provide_context(context);
        let geometry = crate::geometry::style_class(move || {
            if let Some(issue) = context.delay.get().issue { return Err(issue); }
            Ok(StyleSet::default())
        });
        #[cfg(target_arch = "wasm32")]
        browser::install(context);
        view! { <div node_ref=context.root
            class=crate::macros::reactive_merge(geometry, &class)
            data-rustify-part="scroll-area-root" data-testid=test_id
            dir=move || context.direction.get().attribute()
            data-scrollbars=move || scrollbars.get().as_str() data-visibility=move || visibility.get().as_str()
            data-rustify-issue=move || context.issue()
            data-x-visible=move || context.visible(Orientation::Horizontal).then_some("")
            data-y-visible=move || context.visible(Orientation::Vertical).then_some("")
            data-x-overflow=move || (model::enabled(scrollbars.get(), Orientation::Horizontal) && context.metrics.get()[0].overflow).then_some("")
            data-y-overflow=move || (model::enabled(scrollbars.get(), Orientation::Vertical) && context.metrics.get()[1].overflow).then_some("")>
            {children()}
        </div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

/// The only scrolling element and keyboard entry. Label properties override Root's name.
#[component]
pub fn Viewport(
    #[prop(optional, into)] aria_label: Signal<String>,
    #[prop(optional, into)] labelled_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view! { <div data-rustify-part="scroll-area-viewport" data-rustify-issue="missing-root" class=class>{children()}</div> }.into_any();
    };
    context.viewports.update(|count| *count += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        context.viewports.try_update(|count| *count -= 1);
        context.viewport.try_update(|node| *node = None);
        context.content.try_update(|node| *node = None);
        context.end_drag();
    });
    view! { <div node_ref=context.viewport id=context.viewport_id.get_value()
        data-rustify-part="scroll-area-viewport" data-testid=test_id class=crate::macros::reactive_merge("", &class)
        data-boundary=move || context.boundary.get().as_str()
        data-scrollbars=move || context.scrollbars.get().as_str()
        data-rustify-issue=move || (context.viewports.get()!=1).then_some("duplicate-part")
        role="group" tabindex="0"
        aria-label=move || { let label=aria_label.get(); let label=if label.is_empty(){context.label.get()}else{label}; (!label.is_empty()).then_some(label) }
        aria-labelledby=move || { let label=labelled_by.get(); let label=if label.is_empty(){context.labelled_by.get()}else{label}; (!label.is_empty()).then_some(label) }>
        <div node_ref=context.content data-rustify-part="scroll-area-content">{children()}</div>
    </div> }.into_any()
}

/// A custom track for one enabled axis. It mirrors native scrolling and adds no keyboard stop.
#[component]
pub fn Scrollbar(
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(area) = use_context::<Context>() else {
        return view! { <div data-rustify-part="scroll-area-scrollbar" data-rustify-issue="missing-root" hidden>{children()}</div> }.into_any();
    };
    let owner = Owner::new();
    let view = owner.with(move || {
        let axis = index(orientation);
        area.track_counts.update(|counts| counts[axis] += 1);
        provide_context(TrackContext { area, orientation });
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            area.track_counts.try_update(|counts| counts[axis] -= 1);
            area.tracks[axis].try_update(|node| *node = None);
            area.end_drag();
        });
        view! { <div node_ref=area.tracks[axis] data-rustify-part="scroll-area-scrollbar"
            data-testid=test_id class=crate::macros::reactive_merge("", &class)
            data-orientation=orientation.attribute()
            data-rustify-issue=move || (area.track_counts.get()[axis]!=1).then_some("duplicate-part")
            data-state=move || if area.visible(orientation){"visible"}else{"hidden"}
            data-disabled=move || (!area.metrics.get()[axis].overflow).then_some("")
            hidden=move || !model::enabled(area.scrollbars.get(),orientation) aria-hidden="true">{children()}</div> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}

/// A pointer thumb whose size and offset come from its enclosing native viewport.
#[component]
pub fn Thumb(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let Some(track) = use_context::<TrackContext>() else {
        return view! { <div data-rustify-part="scroll-area-thumb" data-rustify-issue="missing-scrollbar" hidden /> }.into_any();
    };
    let axis = index(track.orientation);
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        let metrics = track.area.metrics.get()[axis];
        styles.insert(
            0,
            if axis == 0 { "width" } else { "height" },
            format!("{}px", metrics.thumb),
        )?;
        styles.insert(
            0,
            "transform",
            format!(
                "translate{}({}px)",
                if axis == 0 { "X" } else { "Y" },
                metrics.offset
            ),
        )?;
        Ok(styles)
    });
    view! { <div data-rustify-part="scroll-area-thumb" data-testid=test_id
        class=crate::macros::reactive_merge(geometry,&class) data-orientation=track.orientation.attribute()
        hidden=move || !track.area.metrics.get()[axis].overflow /> }.into_any()
}

/// Fills the corner only when both visible tracks meet.
#[component]
pub fn Corner(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    view! { <div data-rustify-part="scroll-area-corner" data-testid=test_id class=crate::macros::reactive_merge("",&class)
    data-rustify-issue=if context.is_none(){Some("missing-root")}else{None}
    hidden=move || context.is_none_or(|area| !area.visible(Orientation::Horizontal) || !area.visible(Orientation::Vertical)) aria-hidden="true" /> }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_hide_delay_preserves_the_last_schedulable_delay() {
        let initial = accept_delay(u32::MAX, None);
        assert_eq!(initial.ms, 600);
        assert_eq!(initial.issue.unwrap().field, "hide_delay_ms");
        let valid = accept_delay(120, None);
        let rejected = accept_delay(u32::MAX, Some(&valid));
        assert_eq!(rejected.ms, 120);
        assert!(rejected.issue.is_some());
        assert_eq!(accept_delay(0, Some(&rejected)).issue, None);
    }
}
