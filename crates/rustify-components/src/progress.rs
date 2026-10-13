//! How far along something is, when anybody knows.

use leptos::prelude::*;

const TRACK: &str = "relative h-2 w-full overflow-hidden rounded-full bg-secondary";
const FILL: &str = "h-full bg-primary transition-[width]";

/// The initial width is an external geometry rule. Hydration and later value
/// changes update the same rule through the CSS object model, without a
/// `style` attribute.
///
/// `indeterminate` is a separate prop rather than an absent value because the
/// two mean different things to a reader: a bar with no `aria-valuenow` says
/// "in progress, no idea how far", and one at zero says "not started".
#[component]
pub fn Progress(
    /// Optional, because a bar whose quantity is not known has no value to
    /// give: `indeterminate` with no `value` is the honest pair.
    #[prop(optional, into)]
    value: Signal<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let max = max.unwrap_or(100.0).max(f64::MIN_POSITIVE);
    let aria_label = (!aria_label.is_empty()).then_some(aria_label);
    let percent = move || (value.get() / max * 100.0).clamp(0.0, 100.0);
    let geometry = crate::geometry::geometry_class(move || {
        let width = if indeterminate.get() {
            100.0
        } else {
            percent()
        };
        format!("width:{width}%;")
    });
    view! {
        <div
            class=crate::macros::reactive_merge(TRACK, &class)
            data-name="Progress"
            data-testid=test_id
            data-state=move || if indeterminate.get() { "indeterminate" } else { "determinate" }
            role="progressbar"
            aria-label=aria_label
            aria-valuemin="0"
            aria-valuemax=max.to_string()
            aria-valuenow=move || (!indeterminate.get()).then(|| percent().to_string())
        >
            <div
                class=format!("{FILL} {geometry}")
                aria-hidden="true"
                class=("opacity-40", move || indeterminate.get())
            />
        </div>
    }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn host_progress_keeps_geometry_out_of_style_attributes() {
        let owner = Owner::new();
        let styles = crate::geometry::GeometryStyles::default();
        let html = owner.with(|| {
            rustify_ui::id::provide("progress-test");
            crate::geometry::provide_geometry_styles(styles.clone());
            view! { <Progress value=37.5 max=75.0/> }.to_html()
        });
        assert!(!html.contains(" style="), "{html}");
        assert!(html.contains("aria-valuenow=\"50\""), "{html}");
        assert!(styles.css().contains("width:50%;"), "{}", styles.css());
    }

    #[test]
    fn host_unknown_progress_has_a_full_fill_without_an_announced_value() {
        let owner = Owner::new();
        let styles = crate::geometry::GeometryStyles::default();
        let html = owner.with(|| {
            crate::geometry::provide_geometry_styles(styles.clone());
            view! { <Progress indeterminate=true/> }.to_html()
        });
        assert!(!html.contains("aria-valuenow="), "{html}");
        assert!(styles.css().contains("width:100%;"), "{}", styles.css());
    }
}
