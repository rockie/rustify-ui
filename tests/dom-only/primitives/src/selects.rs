//! Select metadata and rich parts are deliberately independent here.
use leptos::prelude::*;
use p::select::SelectEntry;
use rustify_components::{primitives as p, Form};

fn entries(mode: &str) -> Vec<SelectEntry> {
    match mode {
        "empty" => Vec::new(),
        "long" => (0..150)
            .map(|at| SelectEntry::new(format!("item-{at}"), format!("Item {at:03}")))
            .collect(),
        "duplicate-entries" => vec![
            SelectEntry::new("a", "Apricot"),
            SelectEntry::new("b", "Banana"),
            SelectEntry::new("b", "Blueberry"),
        ],
        "remove" => vec![
            SelectEntry::new("a", "Apricot"),
            SelectEntry::new("c", "Cherry"),
        ],
        _ => vec![
            SelectEntry::new("a", "Apricot"),
            SelectEntry::new("b", "Banana"),
            SelectEntry::new("bb", "Blackberry"),
            SelectEntry::new("off", "Blueberry").disabled(),
            SelectEntry::new("c", "Cherry"),
        ],
    }
}
#[component]
pub fn Selects() -> impl IntoView {
    let value = RwSignal::new(Some("b".to_string()));
    let open = RwSignal::new(false);
    let policy = RwSignal::new("accept".to_string());
    let mode = RwSignal::new("normal".to_string());
    let options = Memo::new(move |_| entries(&mode.get()));
    let rows = Memo::new(move |_| {
        let mut rows: Vec<_> = options
            .get()
            .into_iter()
            .enumerate()
            .filter(|(_, entry)| mode.get() != "missing-item" || entry.value != "a")
            .map(|(at, entry)| {
                (
                    format!("{at}-{}", entry.value),
                    entry.value,
                    entry.text_value,
                )
            })
            .collect();
        if mode.get() == "duplicate-items" {
            rows.push(("duplicate".into(), "b".into(), "Duplicate Banana".into()));
        }
        if mode.get() == "unknown-item" {
            rows.push(("unknown".into(), "unknown".into(), "Unknown".into()));
        }
        rows
    });
    let pending = RwSignal::new(None::<Option<String>>);
    let requests = RwSignal::new(Vec::<String>::new());
    let open_requests = RwSignal::new(Vec::<String>::new());
    let hooks = RwSignal::new(Vec::<String>::new());
    let selection_result = RwSignal::new("No selection attempted".to_string());
    let cancel = RwSignal::new(false);
    let cancel_escape = RwSignal::new(false);
    let cancel_outside = RwSignal::new(false);
    let refuse_close = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let trigger_visible = RwSignal::new(true);
    let cancel_target = RwSignal::new(false);
    let cancel_content = RwSignal::new(false);
    let direction = RwSignal::new(p::Direction::LeftToRight);
    let saves = RwSignal::new(0);
    let form = Form::new(&["fruit"], move || saves.update(|n| *n += 1));
    view! {
        <section data-testid="raw-select-section">
            <h2>"Select parts"</h2>
            <label>"Policy"<select data-testid="select-policy" on:change=move |event| policy.set(event_target_value(&event))>
                <option value="accept">"Accept"</option><option value="refuse">"Refuse"</option><option value="delay">"Delay"</option>
            </select></label>
            <label>"Entries"<select data-testid="select-mode" on:change=move |event| mode.set(event_target_value(&event))>
                { ["normal", "empty", "remove", "long", "missing-item", "duplicate-entries", "duplicate-items", "unknown-item"].into_iter().map(|mode| view! { <option value=mode>{mode}</option> }).collect_view() }
            </select></label>
            <button type="button" data-testid="select-apply" on:click=move |_| { if let Some(next) = pending.get_untracked() { value.set(next); pending.set(None); form.changed("fruit"); } }>"Accept delayed value"</button>
            <button type="button" data-testid="select-cancel" on:click=move |_| cancel.update(|value| *value = !*value)>"Cancel selection"</button>
            <button type="button" data-testid="select-cancel-escape" on:click=move |_| cancel_escape.update(|value| *value = !*value)>"Cancel Escape"</button>
            <button type="button" data-testid="select-cancel-outside" on:click=move |_| cancel_outside.update(|value| *value = !*value)>"Cancel outside"</button>
            <button type="button" data-testid="select-refuse-close" on:click=move |_| refuse_close.update(|value| *value = !*value)>"Refuse close"</button>
            <button type="button" data-testid="select-readonly" on:click=move |_| read_only.update(|value| *value = !*value)>"Toggle read only"</button>
            <button type="button" data-testid="select-disabled" on:click=move |_| disabled.update(|value| *value = !*value)>"Toggle disabled"</button>
            <button type="button" data-testid="select-cancel-target" on:click=move |_| cancel_target.update(|value| *value = !*value)>"Cancel native trigger"</button>
            <button type="button" data-testid="select-cancel-content" on:click=move |_| cancel_content.update(|value| *value = !*value)>"Cancel native content"</button>
            <button type="button" data-testid="select-direction" on:click=move |_| direction.update(|value| *value = if *value == p::Direction::LeftToRight { p::Direction::RightToLeft } else { p::Direction::LeftToRight })>"Change direction"</button>
            <button type="button" data-testid="select-trigger-visible" on:click=move |_| trigger_visible.update(|value| *value = !*value)>"Toggle trigger"</button>
            <button type="button" data-testid="select-program-close" on:click=move |_| open.set(false)>"Program close"</button>
            <button type="button" data-testid="select-clear" on:click=move |_| { value.set(None); form.changed("fruit"); }>"Clear value"</button>
            <dl class="sample-feedback">
                <div><dt>"Accepted value:"</dt><dd><output data-testid="select-accepted">{move || value.get().unwrap_or_default()}</output></dd></div>
                <div><dt>"Value request count:"</dt><dd><output data-testid="select-request-count">{move || requests.with(Vec::len)}</output></dd></div>
                <div><dt>"Last value request:"</dt><dd><output data-testid="select-last-request">{move || requests.with(|all| all.last().cloned().unwrap_or_else(|| "—".into()))}</output></dd></div>
                <div><dt>"Value request history:"</dt><dd><output data-testid="select-requests">{move || requests.get().join("|")}</output></dd></div>
                <div><dt>"List open:"</dt><dd><output data-testid="select-open">{move || open.get()}</output></dd></div>
                <div><dt>"Open / close requests:"</dt><dd><output data-testid="select-open-requests">{move || open_requests.get().join("|")}</output></dd></div>
                <div><dt>"Selection result:"</dt><dd><output data-testid="select-result">{move || selection_result.get()}</output></dd></div>
                <div><dt>"Hook events:"</dt><dd><output data-testid="select-hooks">{move || hooks.get().join("|")}</output></dd></div>
                <div><dt>"Save calls:"</dt><dd><output data-testid="select-saves">{move || saves.get()}</output></dd></div>
            </dl>
            <p::DirectionProvider direction=direction>
                <p::form::Root form=form rules=Vec::new id="raw-select-form" test_id="raw-select-form">
                    <p::form::Field form=form field="fruit"><p::form::Label>"Fruit"</p::form::Label>
                        <p::select::Root value=value open=open entries=options name="fruit" required=true id="raw-select-trigger" placeholder="Choose fruit"
                            disabled=disabled read_only=read_only
                            on_change={move |next: Option<String>| {
                                requests.update(|all| all.push(next.clone().unwrap_or_default()));
                                match policy.get_untracked().as_str() { "refuse" => {}, "delay" => pending.set(Some(next)), _ => { value.set(next); form.changed("fruit"); } }
                            }}
                            on_open_change=move |next: bool| { open_requests.update(|all| all.push(next.to_string())); if next || !refuse_close.get_untracked() { open.set(next); } }
                            on_select=Callback::new(move |event: rustify_ui::Cancelable<String>| {
                                hooks.update(|all| all.push(format!("select:{}", event.value())));
                                let cancelled = cancel.get_untracked();
                                selection_result.set(format!("{}: {}", if cancelled { "Cancelled" } else { "Requested" }, event.value()));
                                if cancelled { event.prevent_default(); }
                            })
                            on_escape_key_down=Callback::new(move |event: rustify_ui::Cancelable<rustify_ui::LayerDismiss>| { hooks.update(|all| all.push("escape".into())); if cancel_escape.get_untracked() { event.prevent_default(); } })
                            on_pointer_down_outside=Callback::new(move |event: rustify_ui::Cancelable<rustify_ui::LayerDismiss>| { hooks.update(|all| all.push("outside".into())); if cancel_outside.get_untracked() { event.prevent_default(); } })
                            on_open_auto_focus=Callback::new(move |_| hooks.update(|all| all.push("open-focus".into())))
                            on_close_auto_focus=Callback::new(move |_| hooks.update(|all| all.push("close-focus".into())))>
                            <Show when=move || trigger_visible.get()>
                                {p::select::trigger_as(leptos::html::button().attr("data-testid", "raw-select-trigger").class("host-select-trigger")
                                    .on(leptos::ev::click, move |event| { if cancel_target.get_untracked() { event.prevent_default(); } })
                                    .child(view! { <p::select::Value />" "<p::select::Icon>"Open"</p::select::Icon> }))}
                            </Show>
                            <p::select::Content test_id="raw-select-list" attr:data-caller-content="kept" on:click=move |event| { if cancel_content.get_untracked() { event.prevent_default(); } }>
                                <p::select::ScrollUpButton>"Up"</p::select::ScrollUpButton>
                                <p::select::Viewport>
                                    <p::select::Group><p::select::Label>"Fruit choices"</p::select::Label>
                                        <For each=move || rows.get() key=|(key, _, _)| key.clone() let:row>
                                            <p::select::Item value=row.1 test_id=format!("raw-option-{}", row.0)>
                                                <p::select::ItemText>{row.2}</p::select::ItemText><span>" Custom layout"</span><p::select::ItemIndicator>"Chosen"</p::select::ItemIndicator>
                                            </p::select::Item>
                                        </For>
                                    </p::select::Group>
                                </p::select::Viewport>
                                <p::select::Separator /><p::select::ScrollDownButton>"Down"</p::select::ScrollDownButton>
                            </p::select::Content>
                        </p::select::Root>
                        <p::form::Message test_id="select-error" />
                    </p::form::Field>
                    <button type="submit" data-testid="select-submit">"Submit selection"</button>
                </p::form::Root>
            </p::DirectionProvider>
            <button type="button" data-testid="select-after">"After select"</button>
        </section>
    }
}

#[component]
pub fn SelectLayers() -> impl IntoView {
    let outer = RwSignal::new(false);
    let inner = RwSignal::new(false);
    let value = RwSignal::new(Some("a".to_string()));
    let number = RwSignal::new(12.0);
    let cancel_escape = RwSignal::new(true);
    let cancel_open_focus = RwSignal::new(false);
    let cancel_close_focus = RwSignal::new(false);
    let calls = RwSignal::new(Vec::<String>::new());
    let opener = NodeRef::<leptos::html::Button>::new();
    view! {
        <section aria-label="Nested select and layer">
            <button type="button" node_ref=opener data-testid="layer-open" on:click=move |_| outer.set(true)>"Open parent layer"</button>
            <button type="button" data-testid="layer-cancel-open-focus" on:click=move |_| cancel_open_focus.update(|flag| *flag = !*flag)>"Cancel open focus"</button>
            <output data-testid="layer-calls">{move || calls.get().join("|")}</output>
            <Show when=move || outer.get()>
                <rustify_ui::Layer modal=true fit=true anchor=Signal::derive(move || opener.get().map(|node| rustify_ui::Anchor::element(node.as_ref())).unwrap_or(rustify_ui::Anchor::Centred))
                    on_close=move || { calls.update(|all| all.push("outer-close".into())); outer.set(false); }
                    on_escape_key_down=Callback::new(move |_| calls.update(|all| all.push("outer-escape".into())))
                    on_open_auto_focus=Callback::new(move |event: rustify_ui::Cancelable<()>| { calls.update(|all| all.push("outer-open-focus".into())); if cancel_open_focus.get_untracked() { event.prevent_default(); } })
                    on_close_auto_focus=Callback::new(move |event: rustify_ui::Cancelable<()>| { calls.update(|all| all.push("outer-close-focus".into())); if cancel_close_focus.get_untracked() { event.prevent_default(); } })>
                    <div data-testid="layer-panel">
                        <button type="button" data-testid="layer-first">"Parent action"</button>
                        <button type="button" data-testid="layer-cancel-escape" on:click=move |_| cancel_escape.update(|flag| *flag = !*flag)>"Toggle inner Escape cancellation"</button>
                        <button type="button" data-testid="layer-cancel-close-focus" on:click=move |_| cancel_close_focus.update(|flag| *flag = !*flag)>"Cancel close focus"</button>
                        <button type="button" data-testid="layer-program-close" on:click=move |_| outer.set(false)>"Close parent"</button>
                        <p::select::Root value=value on_change={move |next: Option<String>| value.set(next)} open=inner on_open_change=move |next| inner.set(next)
                            entries=vec![SelectEntry::new("a", "Apricot"), SelectEntry::new("b", "Banana")]
                            aria_label="Nested fruit"
                            on_escape_key_down=Callback::new(move |event: rustify_ui::Cancelable<rustify_ui::LayerDismiss>| { calls.update(|all| all.push("inner-escape".into())); if cancel_escape.get_untracked() { event.prevent_default(); } })>
                            <p::select::Trigger test_id="layer-select"><p::select::Value /></p::select::Trigger>
                            <p::select::Content test_id="layer-list"><p::select::Viewport><p::select::Item value="a" /><p::select::Item value="b" /></p::select::Viewport></p::select::Content>
                        </p::select::Root>
                        <p::number_field::Root value=number on_change=move |next| number.set(next) aria_label="Nested number">
                            <p::number_field::Input test_id="layer-number" />
                        </p::number_field::Root>
                        <p::slider::Root values=Signal::derive(move || vec![p::slider::ThumbValue { id: "nested".into(), value: 20.0 }])
                            on_change=|_| {} on_commit=Callback::new(move |_| calls.update(|all| all.push("slider-commit".into())))
                            on_cancel=Callback::new(move |_| calls.update(|all| all.push("slider-cancel".into())))>
                            <p::slider::Track><p::slider::Thumb id="nested" aria_label="Nested slider" test_id="layer-slider" /></p::slider::Track>
                        </p::slider::Root>
                        <p><button type="button" data-testid="layer-after">"After nested choices"</button></p>
                    </div>
                </rustify_ui::Layer>
            </Show>
        </section>
    }
}
