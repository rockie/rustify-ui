//! A finite positive ratio expressed through external geometry rules.
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Ratio(f64);
impl Default for Ratio {
    fn default() -> Self {
        Self(1.)
    }
}
impl Ratio {
    fn apply(&mut self, next: f64) -> bool {
        if next.is_finite() && next > 0. {
            self.0 = next;
            true
        } else {
            false
        }
    }
}
#[component]
pub fn Root(
    #[prop(default=Signal::stored(1.),into)] ratio: Signal<f64>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let last = StoredValue::new(Ratio::default());
    let geometry = crate::geometry::style_class(move || {
        last.update_value(|last| {
            last.apply(ratio.get());
        });
        let ratio = last.get_value().0;
        let mut styles = crate::geometry::StyleSet::default();
        styles.insert(0, "aspect-ratio", ratio.to_string())?;
        Ok(styles)
    });
    view! {<div class=crate::macros::merge(geometry,&class) data-rustify-part="aspect-ratio-root" data-testid=test_id
        data-rustify-issue=move ||{let ratio=ratio.get();(!ratio.is_finite() || ratio<=0.).then_some("ratio")}>
        <div data-rustify-part="aspect-ratio-content">{children()}</div>
    </div>}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_ratios_keep_the_last_usable_geometry() {
        let mut ratio = Ratio::default();
        for next in [0., -1., f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(!ratio.apply(next));
            assert_eq!(ratio.0, 1.);
        }
        assert!(ratio.apply(16. / 9.));
        assert!(!ratio.apply(0.));
        assert_eq!(ratio.0, 16. / 9.);
        assert!(ratio.apply(3. / 4.));
        assert_eq!(ratio.0, 3. / 4.);
    }
}
