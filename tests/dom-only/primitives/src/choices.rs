//! Consumer-owned state and acceptance policy for the public choice APIs.

use leptos::prelude::*;
use rustify_components::primitives as p;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Accept,
    Refuse,
    Delay,
}
struct Controlled<T: Clone + Send + Sync + 'static> {
    value: RwSignal<T>,
    pending: RwSignal<Option<T>>,
    requests: RwSignal<u32>,
    accepted: RwSignal<u32>,
    last: RwSignal<String>,
    policy: RwSignal<Policy>,
}
impl<T: Clone + Send + Sync + 'static> Copy for Controlled<T> {}
impl<T: Clone + Send + Sync + 'static> Clone for Controlled<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Clone + Debug + Send + Sync + 'static> Controlled<T> {
    fn new(value: T, policy: RwSignal<Policy>) -> Self {
        Self {
            value: RwSignal::new(value),
            pending: RwSignal::new(None),
            requests: RwSignal::new(0),
            accepted: RwSignal::new(0),
            last: RwSignal::new(String::new()),
            policy,
        }
    }
    fn request(self, value: T) {
        self.requests.update(|n| *n += 1);
        self.last.set(format!("{value:?}"));
        match self.policy.get_untracked() {
            Policy::Accept => {
                self.value.set(value);
                self.accepted.update(|n| *n += 1);
            }
            Policy::Refuse => {}
            Policy::Delay => self.pending.set(Some(value)),
        }
    }
    fn apply(self) {
        if let Some(value) = self.pending.get_untracked() {
            self.value.set(value);
            self.pending.set(None);
            self.accepted.update(|n| *n += 1);
        }
    }
    fn output(self, id: &'static str) -> impl IntoView {
        view! { <div data-testid=id>
            <output data-value>{move || format!("{:?}", self.value.get())}</output>
            <output data-requests>{move || self.requests.get()}</output>
            <output data-accepts>{move || self.accepted.get()}</output>
            <output data-last>{move || self.last.get()}</output>
        </div> }
    }
}
#[component]
pub fn Choices() -> impl IntoView {
    let policy = RwSignal::new(Policy::Accept);
    let readonly = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let mixed = Controlled::new(p::CheckedState::Indeterminate, policy);
    let switch = Controlled::new(false, policy);
    let one = Controlled::new(Some("a".to_string()), policy);
    let peers = Controlled::new(Some("a".to_string()), policy);
    let two = Controlled::new(Some("a".to_string()), policy);
    let checks = Controlled::new(vec!["b".to_string(), "c".to_string()], policy);
    let toggles = Controlled::new(vec!["bold".to_string()], policy);
    let radio_items = RwSignal::new(vec!["a", "b", "c"]);
    let check_items = RwSignal::new(vec!["a", "b", "c"]);
    let disable_b = RwSignal::new(false);
    view! {
        <section aria-label="Unstyled choices">
            <h2>"Controlled choices"</h2>
            <label>"Acceptance policy"
                <select data-testid="choice-policy" on:change:target=move |event| policy.set(match event.target().value().as_str() {
                    "refuse" => Policy::Refuse, "delay" => Policy::Delay, _ => Policy::Accept,
                })><option value="accept">"Accept"</option><option value="refuse">"Refuse"</option><option value="delay">"Delay"</option></select>
            </label>
            <button type="button" data-testid="choice-readonly" on:click=move |_| readonly.update(|value| *value = !*value)>"Toggle read-only"</button>
            <button type="button" data-testid="choice-disabled" on:click=move |_| disabled.update(|value| *value = !*value)>"Toggle disabled"</button>
            <button type="button" data-testid="choice-apply" on:click=move |_| { mixed.apply(); switch.apply(); one.apply(); two.apply(); checks.apply(); toggles.apply(); peers.apply(); }>"Accept pending requests"</button>
            <button type="button" data-testid="choice-remove-b" on:click=move |_| radio_items.update(|items| items.retain(|value| *value != "b"))>"Remove B"</button>
            <button type="button" data-testid="choice-disable-b" on:click=move |_| disable_b.update(|value| *value = !*value)>"Disable B"</button>
            <button type="button" data-testid="choice-reverse" on:click=move |_| { radio_items.update(|items| items.reverse()); check_items.update(|items| items.reverse()); }>"Reverse item declaration"</button>
            <form id="choices-form" on:submit=move |event| event.prevent_default()>
                <label for="mixed-native">"Mixed checkbox"</label>
                <p::checkbox::Root checked=mixed.value on_change=move |value| mixed.request(value)
                    id="mixed-native" name="consent" value="yes" read_only=readonly disabled=disabled>
                    <p::checkbox::Control test_id="choice-mixed" /><p::checkbox::Indicator>"mixed marker"</p::checkbox::Indicator>
                </p::checkbox::Root>
                <label for="switch-native">"Notifications"</label>
                <p::switch::Root checked=switch.value on_change=move |value| switch.request(value)
                    id="switch-native" name="notifications" read_only=readonly disabled=disabled>
                    <p::switch::Control test_id="choice-switch" /><p::switch::Thumb />
                </p::switch::Root>
                <p::DirectionProvider direction=rustify_ui::Direction::LeftToRight>
                    <label for="peer-a">"Peer A"</label>
                    <p::radio::Root checked=Signal::derive(move || peers.value.get().as_deref() == Some("a"))
                        on_change=move |_| peers.request(Some("a".into())) name="peer" value="a" id="peer-a" disabled=disabled read_only=readonly>
                        <p::radio::Control test_id="choice-peer-a" />
                    </p::radio::Root>
                </p::DirectionProvider>
                <p::DirectionProvider direction=rustify_ui::Direction::RightToLeft>
                    <label for="peer-b">"Peer B"</label>
                    <p::radio::Root checked=Signal::derive(move || peers.value.get().as_deref() == Some("b"))
                        on_change=move |_| peers.request(Some("b".into())) name="peer" value="b" id="peer-b" disabled=disabled read_only=readonly>
                        <p::radio::Control test_id="choice-peer-b" />
                    </p::radio::Root>
                </p::DirectionProvider>
                <p::radio_group::Root value=one.value on_change=move |value| one.request(value) required=true
                    aria_label="First anonymous group" read_only=readonly disabled=disabled test_id="choice-radio-one">
                    <For each=move || radio_items.get() key=|value| *value let:value>
                        <p::radio_group::Item value=value disabled=Signal::derive(move || value == "b" && disable_b.get()) test_id=format!("choice-one-{value}")>{value}</p::radio_group::Item>
                    </For>
                </p::radio_group::Root>
                <p::radio_group::Root value=two.value on_change=move |value| two.request(value)
                    aria_label="Second anonymous group" read_only=readonly disabled=disabled test_id="choice-radio-two">
                    <p::radio_group::Item value="a" test_id="choice-two-a">"A"</p::radio_group::Item>
                    <p::radio_group::Item value="b" test_id="choice-two-b">"B"</p::radio_group::Item>
                </p::radio_group::Root>
                <p::checkbox_group::Root value=checks.value on_change=move |value| checks.request(value) required=true
                    name="features" aria_label="Features" read_only=readonly disabled=disabled>
                    <For each=move || check_items.get() key=|value| *value let:value>
                        <p::checkbox_group::Item value=value disabled=Signal::stored(value == "c") test_id=format!("choice-check-{value}")>{value}</p::checkbox_group::Item>
                    </For>
                </p::checkbox_group::Root>
                <p::toggle_group::Root value=toggles.value on_change=move |value| toggles.request(value) mode=p::SelectionMode::Multiple
                    name="format" aria_label="Formatting" read_only=readonly disabled=disabled>
                    <p::toggle_group::Item value="bold" test_id="choice-bold">"Bold"</p::toggle_group::Item>
                    <p::toggle_group::Item value="italic" test_id="choice-italic">"Italic"</p::toggle_group::Item>
                </p::toggle_group::Root>
                <p::checkbox::Root checked=p::CheckedState::Unchecked on_change=|_| panic!("cancelled control must not request")>
                    {p::checkbox::control_as(leptos::html::input().attr("data-testid", "choice-cancel")
                        .on(leptos::ev::click, |event| event.prevent_default()))}
                </p::checkbox::Root>
            </form>
            {mixed.output("choice-mixed-state")}{switch.output("choice-switch-state")}
            {one.output("choice-one-state")}{two.output("choice-two-state")}{peers.output("choice-peers-state")}
            {checks.output("choice-checks-state")}{toggles.output("choice-toggles-state")}
        </section>
    }
}
