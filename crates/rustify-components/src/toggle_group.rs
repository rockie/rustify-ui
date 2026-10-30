//! Themed toggle groups using the same native button and shared group behavior.
use crate::{primitives, Orientation};
use leptos::prelude::*;
pub use primitives::SelectionMode;
/// One button of a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ToggleItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// The requested set in declaration order, preserving the legacy helper contract.
pub fn pressed_after(
    pressed: &[String],
    items: &[ToggleItem],
    value: &str,
    multiple: bool,
) -> Vec<String> {
    primitives::choice::selection_request(
        &pressed.iter().cloned().collect(),
        &items
            .iter()
            .map(|item| item.value.clone())
            .collect::<Vec<_>>(),
        value,
        if multiple {
            SelectionMode::Multiple
        } else {
            SelectionMode::Single
        },
        true,
    )
    .unwrap_or_else(|| pressed.to_vec())
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Vec<String>>,
    #[prop(into)] on_change: Callback<Vec<String>>,
    #[prop(optional)] mode: SelectionMode,
    #[prop(default = true)] allow_empty: bool,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] r#loop: bool,
    #[prop(optional)] toolbar: bool,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let classes = crate::macros::reactive_merge_with(move || {
        vec![
            if toolbar {
                "inline-flex w-fit items-center gap-1 rounded-md border border-border bg-card p-1"
                    .into()
            } else {
                "inline-flex w-fit items-center gap-1".into()
            },
            if orientation == Orientation::Vertical {
                "flex-col items-stretch".into()
            } else {
                String::new()
            },
            class.get(),
        ]
    });
    view! { <primitives::toggle_group::Root value=value on_change=on_change mode=mode allow_empty=allow_empty name=name form=form disabled=disabled
        read_only=read_only invalid=invalid required=required described_by=described_by aria_label=aria_label
        orientation=orientation r#loop=r#loop toolbar=toolbar class=Signal::derive(classes) test_id=test_id>
        {children()}
    </primitives::toggle_group::Root> }
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <primitives::toggle_group::Item value=value id=id aria_label=aria_label labelled_by=labelled_by described_by=described_by disabled=disabled class=crate::toggle::item_class(class) test_id=test_id>
        {children()}
    </primitives::toggle_group::Item> }
}
#[component]
pub fn ToggleGroup(
    #[prop(into)] value: Signal<Vec<String>>,
    on_change: impl Fn(Vec<String>) + Send + Sync + 'static,
    #[prop(into)] items: Signal<Vec<ToggleItem>>,
    #[prop(optional)] multiple: bool,
    #[prop(optional)] toolbar: bool,
    #[prop(optional)] orientation: Orientation,
    #[prop(default = true)] r#loop: bool,
    #[prop(default = true)] allow_empty: bool,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let index = Memo::new(move |_| {
        items
            .get()
            .into_iter()
            .map(|item| (item.value.clone(), item))
            .collect::<std::collections::HashMap<_, _>>()
    });
    let item_test_id = StoredValue::new(test_id.clone());
    view! { <Root value=value on_change=Callback::new(on_change) mode=if multiple { SelectionMode::Multiple } else { SelectionMode::Single }
        toolbar=toolbar orientation=orientation r#loop=r#loop allow_empty=allow_empty name=name form=form
        disabled=disabled read_only=read_only invalid=invalid required=required described_by=described_by
        aria_label=aria_label class=class test_id=test_id>
        <For each=move || items.get() key=|item| item.value.clone() let:item>
            {
                let key = StoredValue::new(item.value);
                let off = Signal::derive(move || index.with(|index| index.get(&key.get_value()).is_none_or(|item| item.disabled)));
                view! { <Item value=key.get_value() disabled=off test_id=format!("{}-{}", item_test_id.get_value(), key.get_value())>
                    {move || index.with(|index| index.get(&key.get_value()).map(|item| item.label.clone()).unwrap_or_default())}
                </Item> }
            }
        </For>
    </Root> }
}
#[cfg(test)]
mod tests {
    use super::{pressed_after, ToggleItem};

    fn items() -> Vec<ToggleItem> {
        vec![
            ToggleItem::new("bold", "B"),
            ToggleItem::new("italic", "I"),
            ToggleItem::new("strike", "S").disabled(),
            ToggleItem::new("underline", "U"),
        ]
    }

    fn set(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn one_at_a_time_a_press_asks_for_that_one_or_for_none() {
        let items = items();
        assert_eq!(
            pressed_after(&set(&["bold"]), &items, "italic", false),
            set(&["italic"])
        );
        assert_eq!(
            pressed_after(&set(&["bold"]), &items, "bold", false),
            set(&[])
        );
        assert_eq!(
            pressed_after(&set(&[]), &items, "bold", false),
            set(&["bold"])
        );
    }

    #[test]
    fn several_at_once_a_press_toggles_one_and_keeps_the_buttons_order() {
        let items = items();
        assert_eq!(
            pressed_after(&set(&["underline"]), &items, "bold", true),
            set(&["bold", "underline"])
        );
        assert_eq!(
            pressed_after(&set(&["bold", "underline"]), &items, "bold", true),
            set(&["underline"])
        );
    }
}
