//! Selection cards: one native group item with explicit label and description slots.
use crate::{Gap, GridTracks, Responsive};
use leptos::prelude::*;
use rustify_ui::AccentColor;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioCardsSize {
    One,
    #[default]
    Two,
    Three,
}
impl RadioCardsSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
    pub const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}
impl From<RadioCardsSize> for Signal<Responsive<RadioCardsSize>> {
    fn from(value: RadioCardsSize) -> Self {
        Self::stored(value.into())
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioCardsVariant {
    #[default]
    Surface,
    Classic,
}
impl RadioCardsVariant {
    pub const ALL: &'static [Self] = &[Self::Surface, Self::Classic];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Classic => "classic",
        }
    }
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional, into)] size: Signal<Responsive<RadioCardsSize>>,
    #[prop(optional, into)] variant: Signal<RadioCardsVariant>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] columns: Signal<Responsive<GridTracks>>,
    #[prop(optional, into)] gap: Signal<Gap>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::choice_style::cards_geometry(
        Signal::derive(move || size.get().map(|size| size.number())),
        columns,
        gap,
    );
    let classes = crate::macros::reactive_merge_with(move || {
        vec!["rustify-choice-cards".into(), geometry.clone(), class.get()]
    });
    let size = Signal::derive(move || {
        size.get().map(|size| match size {
            RadioCardsSize::One => crate::radio_group::RadioGroupSize::One,
            RadioCardsSize::Two => crate::radio_group::RadioGroupSize::Two,
            RadioCardsSize::Three => crate::radio_group::RadioGroupSize::Three,
        })
    });
    let variant = Signal::derive(move || match variant.get() {
        RadioCardsVariant::Surface => crate::radio_group::RadioGroupVariant::Surface,
        RadioCardsVariant::Classic => crate::radio_group::RadioGroupVariant::Classic,
    });
    view! { <crate::radio_group::Root value=value on_change=on_change name=name form=form aria_label=aria_label
        described_by=described_by disabled=disabled read_only=read_only invalid=invalid required=required
        size=size variant=variant color=color high_contrast=high_contrast class=Signal::derive(classes) test_id=test_id>
        {children()}
    </crate::radio_group::Root> }
}
#[component]
pub fn Item(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] label: Option<Children>,
    #[prop(optional)] description: Option<Children>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let label_id = crate::id::next("card-label");
    let labelled_by = if label.is_some() || children.is_some() {
        label_id.clone()
    } else {
        String::new()
    };
    let description_id = description
        .as_ref()
        .map(|_| crate::id::next("card-description"));
    let description_ref = StoredValue::new(description_id.clone().unwrap_or_default());
    let described_by = Signal::derive(move || {
        crate::primitives::choice::merge_ids(&[&described_by.get(), &description_ref.get_value()])
    });
    let invalid_content = RwSignal::new(false);
    let node = NodeRef::<leptos::html::Label>::new();
    let revision = crate::primitives::choice::group_content_revision();
    #[cfg(not(target_arch = "wasm32"))]
    let _ = revision;
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        revision.track();
        if let Some(label) = node.get() {
            invalid_content.set(label.query_selector("button,a[href],select,textarea,[contenteditable=true],input:not([data-rustify-choice-owner])").ok().flatten().is_some());
        }
    });
    let disabled = Signal::derive(move || disabled.get() || invalid_content.get());
    view! { <crate::radio_group::Item value=value disabled=disabled id=id aria_label=aria_label labelled_by=labelled_by
        described_by=described_by class=format!("rustify-choice-card {class}") test_id=test_id>
        <span class="rustify-choice-card-content">
            <span id=label_id>{label.or(children).map(|label| label())}</span>
            {description.map(|description| view! { <span id=description_id class="rustify-choice-card-description">{description()}</span> })}
        </span>
    </crate::radio_group::Item> }.add_any_attr((leptos::tachys::html::node_ref::node_ref(node),
        leptos::attr::custom::custom_attribute("data-rustify-issue", move || invalid_content.get().then_some("interactive-card-content"))))
}
