//! Panels side by side, with a divider between each pair.
//!
//! The arithmetic is separate from the drawing because it is the part that
//! goes wrong: a divider that can push a panel to nothing, or that keeps
//! moving after its neighbour has stopped, is a workspace a person cannot get
//! back. What a drag does to a row of sizes is a function, and it is tested as
//! one.

use leptos::ev::{KeyboardEvent, PointerEvent};
use leptos::prelude::*;
use std::sync::Arc;

/// One panel's content, rendered where the splitter puts it.
pub type PanelView = Arc<dyn Fn() -> AnyView + Send + Sync>;

/// Moves `delta` pixels across the divider to the right of panel `index`.
///
/// The pair either side of the divider is the only pair that changes: a drag
/// takes from one and gives to the other, so the total width is what it was
/// and nothing further along the row moves. Neither of them goes below its
/// minimum, which means a divider stops rather than pushing.
pub fn drag(sizes: &[f64], mins: &[f64], index: usize, delta: f64) -> Vec<f64> {
    let mut sizes = sizes.to_vec();
    if index + 1 >= sizes.len() {
        return sizes;
    }
    let min_left = mins.get(index).copied().unwrap_or(0.0);
    let min_right = mins.get(index + 1).copied().unwrap_or(0.0);
    // How far it can actually go, in each direction, before one of the two
    // would be smaller than it is allowed to be.
    let room_right = (sizes[index + 1] - min_right).max(0.0);
    let room_left = (sizes[index] - min_left).max(0.0);
    let moved = delta.clamp(-room_left, room_right);
    sizes[index] += moved;
    sizes[index + 1] -= moved;
    sizes
}

/// The sizes a row starts at, given how much room there is.
///
/// Every panel gets at least its minimum, and what is left over is shared
/// equally. A row that does not fit keeps the minimums and overflows, because
/// a panel squeezed below the size its content needs is not a smaller panel,
/// it is a broken one.
pub fn initial(mins: &[f64], available: f64) -> Vec<f64> {
    let needed: f64 = mins.iter().sum();
    if mins.is_empty() {
        return Vec::new();
    }
    let spare = (available - needed).max(0.0) / mins.len() as f64;
    mins.iter().map(|min| min + spare).collect()
}

/// Panels in a row, with a draggable divider between each pair.
#[component]
pub fn Splitter(
    #[prop(into)] sizes: Signal<Vec<f64>>,
    /// The smallest each panel is allowed to be, declared by the application:
    /// only it knows what its own content needs.
    mins: Vec<f64>,
    on_resize: impl Fn(Vec<f64>) + Send + Sync + 'static,
    panels: Vec<PanelView>,
    /// How far one arrow key press moves a divider.
    #[prop(optional)]
    step: Option<f64>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let step = step.unwrap_or(16.0);
    let mins = StoredValue::new(mins);
    let on_resize = StoredValue::new(on_resize);
    let panels = StoredValue::new(panels);
    let count = panels.with_value(Vec::len);
    // Which divider is being dragged, and where the pointer was when the drag
    // that is in flight last moved.
    let dragging = RwSignal::new(None::<(usize, f64)>);

    let apply = move |index: usize, delta: f64| {
        let next = mins.with_value(|mins| drag(&sizes.get_untracked(), mins, index, delta));
        on_resize.with_value(|resize| resize(next));
    };

    view! {
        <div
            class=crate::macros::merge("rui:flex rui:items-stretch rui:w-full", &class)
            data-name="Splitter"
            data-testid=test_id
            on:pointermove=move |event: PointerEvent| {
                let Some((index, from)) = dragging.get_untracked() else {
                    return;
                };
                let at = event.client_x() as f64;
                dragging.set(Some((index, at)));
                apply(index, at - from);
            }
            on:pointerup=move |_| dragging.set(None)
            on:pointercancel=move |_| dragging.set(None)
        >
            {(0..count)
                .map(|index| {
                    let panel = panels.with_value(|panels| panels[index].clone());
                    let divider = index + 1 < count;
                    view! {
                        <div
                            class="rui:overflow-hidden rui:shrink-0"
                            data-name="Panel"
                            data-testid=format!("panel-{index}")
                            style:width=move || {
                                format!("{}px", sizes.get().get(index).copied().unwrap_or(0.0))
                            }
                        >
                            {panel()}
                        </div>
                        <Show when=move || divider fallback=|| ()>
                            <div
                                class="rui:w-1.5 rui:shrink-0 rui:cursor-col-resize rui:bg-border rui:outline-none rui:focus-visible:ring-ring/50 rui:focus-visible:ring-[3px]"
                                data-name="SplitterDivider"
                                data-testid=format!("divider-{index}")
                                role="separator"
                                aria-orientation="vertical"
                                aria-label=format!("resize panel {}", index + 1)
                                tabindex="0"
                                aria-valuenow=move || {
                                    sizes.get().get(index).copied().unwrap_or(0.0).round()
                                        as i64
                                }
                                on:pointerdown=move |event: PointerEvent| {
                                    // The divider keeps the pointer, so a fast
                                    // drag that leaves it does not stop.
                                    if let Some(element) = event
                                        .target()
                                        .and_then(|target| {
                                            leptos::wasm_bindgen::JsCast::dyn_into::<
                                                leptos::web_sys::Element,
                                            >(target)
                                            .ok()
                                        })
                                    {
                                        let _ = element.set_pointer_capture(event.pointer_id());
                                    }
                                    dragging.set(Some((index, event.client_x() as f64)));
                                }
                                on:keydown=move |event: KeyboardEvent| {
                                    let delta = match event.key().as_str() {
                                        "ArrowLeft" => -step,
                                        "ArrowRight" => step,
                                        _ => return,
                                    };
                                    event.prevent_default();
                                    apply(index, delta);
                                }
                            />
                        </Show>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{drag, initial};

    #[test]
    fn a_drag_takes_from_one_panel_and_gives_to_its_neighbour() {
        let sizes = [300.0, 400.0, 300.0];
        let mins = [100.0, 100.0, 100.0];
        assert_eq!(drag(&sizes, &mins, 0, 50.0), [350.0, 350.0, 300.0]);
        assert_eq!(drag(&sizes, &mins, 0, -50.0), [250.0, 450.0, 300.0]);
        // The total is what it was: nothing further along the row moved.
        let after = drag(&sizes, &mins, 1, 75.0);
        assert_eq!(after.iter().sum::<f64>(), sizes.iter().sum::<f64>());
    }

    #[test]
    fn a_divider_stops_at_a_minimum_rather_than_pushing_through_it() {
        let sizes = [300.0, 400.0, 300.0];
        let mins = [100.0, 350.0, 100.0];
        // The neighbour has fifty to give and no more.
        assert_eq!(drag(&sizes, &mins, 0, 500.0), [350.0, 350.0, 300.0]);
        // And the panel being dragged cannot go below its own.
        assert_eq!(drag(&sizes, &mins, 0, -500.0), [100.0, 600.0, 300.0]);
    }

    #[test]
    fn a_divider_at_a_minimum_does_nothing_at_all() {
        let sizes = [100.0, 400.0];
        let mins = [100.0, 100.0];
        assert_eq!(drag(&sizes, &mins, 0, -20.0), sizes);
    }

    #[test]
    fn the_last_panel_has_no_divider_to_its_right() {
        let sizes = [300.0, 300.0];
        assert_eq!(drag(&sizes, &[0.0, 0.0], 1, 50.0), sizes);
        assert_eq!(drag(&sizes, &[0.0, 0.0], 9, 50.0), sizes);
    }

    #[test]
    fn a_hundred_drags_never_take_a_panel_below_its_minimum() {
        let mins = [120.0, 240.0, 160.0];
        let mut sizes = initial(&mins, 900.0);
        // Alternating, and far larger than the room available, which is what
        // a person dragging quickly actually produces.
        for round in 0..100 {
            let index = round % 2;
            let delta = if round % 3 == 0 { 400.0 } else { -350.0 };
            sizes = drag(&sizes, &mins, index, delta);
            for (panel, min) in sizes.iter().zip(mins.iter()) {
                assert!(panel >= min, "round {round}: {sizes:?} against {mins:?}");
            }
            assert!(
                (sizes.iter().sum::<f64>() - 900.0).abs() < 0.001,
                "{sizes:?}"
            );
        }
    }

    #[test]
    fn a_row_shares_out_what_is_left_after_every_minimum() {
        assert_eq!(
            initial(&[100.0, 200.0, 100.0], 700.0),
            [200.0, 300.0, 200.0]
        );
        // A row that does not fit keeps the minimums: a panel squeezed below
        // what its content needs is broken, not small.
        assert_eq!(
            initial(&[100.0, 200.0, 100.0], 200.0),
            [100.0, 200.0, 100.0]
        );
        assert!(initial(&[], 500.0).is_empty());
    }
}
