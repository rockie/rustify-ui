//! New form composition uses the same view in all three rendering modes.
use leptos::prelude::*;
use rustify_components::{primitives as p, radio_cards as cards, slider::ThumbValue, Form};

#[derive(Clone, Copy)]
pub(crate) struct Parts {
    checked: RwSignal<p::CheckedState>,
    text: RwSignal<String>,
    number: RwSignal<f64>,
    color: RwSignal<String>,
    selected: RwSignal<Option<String>>,
    open: RwSignal<bool>,
    range: RwSignal<Vec<ThumbValue>>,
    multiple: RwSignal<Vec<String>>,
    pinned: RwSignal<bool>,
    radio: [RwSignal<Option<String>>; 12],
    saves: RwSignal<u32>,
    requests: RwSignal<u32>,
    refuse: RwSignal<bool>,
}
fn range() -> Vec<ThumbValue> {
    vec![
        ThumbValue {
            id: "low".into(),
            value: 10.0,
        },
        ThumbValue {
            id: "high".into(),
            value: 40.0,
        },
    ]
}
impl Parts {
    pub fn new() -> Self {
        Self {
            checked: RwSignal::new(p::CheckedState::Indeterminate),
            text: RwSignal::new("Parts title".into()),
            number: RwSignal::new(12.5),
            color: RwSignal::new("#336699".into()),
            selected: RwSignal::new(Some("b".into())),
            open: RwSignal::new(false),
            range: RwSignal::new(range()),
            multiple: RwSignal::new(vec!["a".into(), "b".into()]),
            pinned: RwSignal::new(true),
            radio: std::array::from_fn(|at| {
                RwSignal::new(Some(if at % 2 == 0 { "a".into() } else { "b".into() }))
            }),
            saves: RwSignal::new(0),
            requests: RwSignal::new(0),
            refuse: RwSignal::new(false),
        }
    }
    fn take(self) -> bool {
        self.requests.update(|count| *count += 1);
        !self.refuse.get_untracked()
    }
    #[cfg(target_arch = "wasm32")]
    pub fn snapshot(self) -> serde_json::Value {
        serde_json::json!({
            "checked": format!("{:?}",self.checked.get_untracked()), "text":self.text.get_untracked(),
            "number":self.number.get_untracked(), "color":self.color.get_untracked(), "selected":self.selected.get_untracked(),
            "range":self.range.get_untracked().iter().map(|value| (&value.id,value.value)).collect::<Vec<_>>(),
            "multiple":self.multiple.get_untracked(), "pinned":self.pinned.get_untracked(),
            "radio":self.radio.iter().map(|value| value.get_untracked()).collect::<Vec<_>>(),
            "saves":self.saves.get_untracked(), "requests":self.requests.get_untracked(), "refuse":self.refuse.get_untracked(),
        })
    }
}
#[component]
fn RadioCase(
    value: RwSignal<Option<String>>,
    key: String,
    #[prop(optional)] cards: bool,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    let a = format!("{key}-a");
    let b = format!("{key}-b");
    if cards {
        view! { <cards::Root value=value on_change=move |next| value.set(next) name=name aria_label=key.clone() test_id=key>
            <cards::Item value="a" test_id=a>"Alpha"</cards::Item><cards::Item value="b" test_id=b>"Beta"</cards::Item>
        </cards::Root> }.into_any()
    } else {
        view! { <p::radio_group::Root value=value on_change=move |next| value.set(next) name=name aria_label=key.clone() test_id=key>
            <p::radio_group::Item value="a" test_id=a>"Alpha"</p::radio_group::Item><p::radio_group::Item value="b" test_id=b>"Beta"</p::radio_group::Item>
        </p::radio_group::Root> }.into_any()
    }
}

#[component]
pub(crate) fn FormParts(scope: String, state: Parts) -> impl IntoView {
    let scope = StoredValue::new(scope);
    let id = move |name: &str| format!("{}-{name}", scope.get_value());
    let completion = StoredValue::new(None::<Form>);
    let form = Form::new(
        &[
            "title", "agree", "topics", "range", "number", "color", "select",
        ],
        move || {
            state.saves.update(|count| *count += 1);
            if let Some(form) = completion.get_value() {
                form.submitted(Ok(()));
            }
        },
    );
    completion.set_value(Some(form));
    view! {
        <section class="harness-card harness-form-parts" data-testid=id("form-parts")>
            <h3>"Composed form parts"</h3>
            <DeferredFields scope=scope.get_value() />
            <button type="button" data-testid=id("raw-refuse") aria-pressed=move || state.refuse.get().to_string() on:click=move |_| state.refuse.update(|flag| *flag=!*flag)>"Refuse value requests"</button>
            <output data-testid=id("raw-requests")>{move || state.requests.get()}</output>
            <p::form::Root form=form id=id("parts-form") test_id=id("parts-form") rules=Vec::new
                on_reset=Callback::new(move |_| {
                    state.text.set("Parts title".into()); state.checked.set(p::CheckedState::Indeterminate);
                    state.multiple.set(vec!["a".into(),"b".into()]); state.range.set(range()); state.number.set(12.5); state.color.set("#336699".into());
                    state.selected.set(Some("b".into())); state.open.set(false); state.pinned.set(true);
                    for (at,value) in state.radio.iter().take(4).enumerate() { value.set(Some(if at%2==0 {"a".into()} else {"b".into()})); }
                    form.reset();
                })>
                <p::form::Field form=form field="title"><p::form::Label>"Parts title"</p::form::Label><p::form::Description>"Native name and mounted description"</p::form::Description>
                    <p::text_field::Root value=state.text on_change=move |next| { if state.take() { state.text.set(next); form.changed("title"); } } name="title" required=true>
                        <p::text_field::Slot>"Prefix"</p::text_field::Slot><p::text_field::Input test_id=id("raw-title") />
                    </p::text_field::Root><p::form::Message test_id=id("raw-title-error") />
                </p::form::Field>
                <p::form::Field form=form field="agree"><p::form::Label>"Agreement"</p::form::Label>
                    <p::checkbox::Root checked=state.checked on_change=move |next| { if state.take() { state.checked.set(next); form.changed("agree"); } } name="agree" required=true>
                        <p::checkbox::Control test_id=id("raw-mixed") /><p::checkbox::Indicator>"State marker"</p::checkbox::Indicator>
                    </p::checkbox::Root><p::form::Message test_id=id("raw-agree-error") />
                </p::form::Field>
                <p::form::Field form=form field="topics"><p::form::Label>"Topics"</p::form::Label>
                    <p::checkbox_group::Root value=state.multiple on_change=move |next| { if state.take() { state.multiple.set(next); form.changed("topics"); } } name="topics" required=true aria_label="Parts topics">
                        <p::checkbox_group::Item value="a" test_id=id("raw-topic-a")>"Alpha"</p::checkbox_group::Item><p::checkbox_group::Item value="b" test_id=id("raw-topic-b")>"Beta"</p::checkbox_group::Item>
                    </p::checkbox_group::Root><p::form::Message />
                </p::form::Field>
                <p::form::Field form=form field="range"><p::form::Label>"Range"</p::form::Label>
                    <p::slider::Root values=state.range on_change={move |next: Vec<ThumbValue>| { if state.take() { state.range.set(next); form.changed("range"); } }} name="range" class="harness-raw-slider">
                        <p::slider::Track><p::slider::Range /></p::slider::Track><p::slider::Thumb id="low" aria_label="Low" test_id=id("raw-low") /><p::slider::Thumb id="high" aria_label="High" test_id=id("raw-high") />
                    </p::slider::Root><p::form::Message />
                </p::form::Field>
                <p::form::Field form=form field="number"><p::form::Label>"Number"</p::form::Label>
                    <p::number_field::Root value=state.number on_change=move |next| { if state.take() { state.number.set(next); form.changed("number"); } } name="number" required=true>
                        <p::number_field::Input test_id=id("raw-number") />
                    </p::number_field::Root><p::form::Message />
                </p::form::Field>
                <p::form::Field form=form field="color"><p::form::Label>"Colour"</p::form::Label>
                    <p::color_field::Root value=state.color on_change=move |next| { if state.take() { state.color.set(next); form.changed("color"); } } name="color" required=true>
                        <p::color_field::Input test_id=id("raw-color") /><p::color_field::Swatch />
                    </p::color_field::Root><p::form::Message />
                </p::form::Field>
                <p::form::Field form=form field="select"><p::form::Label>"Choice"</p::form::Label>
                    <p::select::Root value=state.selected on_change={move |next: Option<String>| { if state.take() { state.selected.set(next); form.changed("select"); } }}
                        open=state.open on_open_change=move |next| state.open.set(next) entries=vec![p::select::SelectEntry::new("a","Alpha"),p::select::SelectEntry::new("b","Bravo")] name="choice" required=true>
                        <p::select::Trigger test_id=id("raw-select")><p::select::Value /><p::select::Icon>"Open"</p::select::Icon></p::select::Trigger>
                        <p::select::Content test_id=id("raw-select-list")><p::select::Viewport><p::select::Group><p::select::Label>"Grouped choices"</p::select::Label>
                            <p::select::Item value="a"><p::select::ItemText />" · Rich content"<p::select::ItemIndicator>"Chosen"</p::select::ItemIndicator></p::select::Item><p::select::Item value="b" />
                        </p::select::Group></p::select::Viewport></p::select::Content>
                    </p::select::Root><p::form::Message />
                </p::form::Field>
                <p::toggle::Root pressed=state.pinned on_change=move |next| { if state.take() { state.pinned.set(next); } } name="pin" test_id=id("raw-toggle")>"Pinned"</p::toggle::Root>
                <RadioCase value=state.radio[0] key=id("form-radio-left") /><RadioCase value=state.radio[1] key=id("form-radio-right") />
                <RadioCase value=state.radio[2] key=id("form-cards-left") cards=true /><RadioCase value=state.radio[3] key=id("form-cards-right") cards=true />
                <button type="submit" data-testid=id("parts-submit")>"Submit parts"</button><button type="reset" data-testid=id("parts-reset")>"Reset parts"</button>
                <output data-testid=id("parts-saves")>{move || state.saves.get()}</output>
            </p::form::Root>
            <RadioCase value=state.radio[4] key=id("outside-radio-left") /><RadioCase value=state.radio[5] key=id("outside-radio-right") />
            <RadioCase value=state.radio[6] key=id("outside-cards-left") cards=true /><RadioCase value=state.radio[7] key=id("outside-cards-right") cards=true />
            <form id=id("named-form-left") data-testid=id("named-form-left")>
                <RadioCase value=state.radio[8] key=id("named-radio-left") name="shared-radio" /><RadioCase value=state.radio[10] key=id("named-cards-left") name="shared-cards" cards=true />
            </form>
            <form id=id("named-form-right") data-testid=id("named-form-right")>
                <RadioCase value=state.radio[9] key=id("named-radio-right") name="shared-radio" /><RadioCase value=state.radio[11] key=id("named-cards-right") name="shared-cards" cards=true />
            </form>
        </section>
    }
}

#[component]
fn DeferredFields(scope: String) -> impl IntoView {
    let scope = StoredValue::new(scope);
    let form = Form::new(&["direct", "show", "for", "closure"], || {});
    view! {
        <For each=|| ["direct", "show", "for", "closure"] key=|mode| *mode children=move |mode| view! {
            <p::form::Field form=form field=mode>
                <p::form::Label test_id=format!("{}-deferred-{mode}-label", scope.get_value())>{format!("Deferred {mode}")}</p::form::Label>
                <DeferredEditor mode=mode id=format!("{}-deferred-{mode}", scope.get_value()) />
            </p::form::Field>
        } />
    }
}
#[component]
fn DeferredEditor(mode: &'static str, id: String) -> impl IntoView {
    let id = StoredValue::new(id);
    let control = move || {
        view! {
            <p::text_field::Root id=id.get_value() value="Accepted" on_change=|_| {}>
                <p::text_field::Input test_id=id.get_value() />
            </p::text_field::Root>
        }
    };
    match mode {
        "direct" => control().into_any(),
        "show" => view! { <Show when=|| true>{control}</Show> }.into_any(),
        "for" => view! { <For each=|| [1] key=|n| *n children=move |_| control() /> }.into_any(),
        _ => control.into_any(),
    }
}
