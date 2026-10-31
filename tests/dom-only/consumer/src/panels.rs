use leptos::prelude::*;
use rustify_components::primitives::{
    tabs::{ActivationMode, PanelEntry},
    DirectionProvider,
};
use rustify_components::{accordion, collapsible, tabs};
use rustify_ui::Direction;

#[component]
pub fn Panels() -> impl IntoView {
    let value = RwSignal::new(Some("a".to_string()));
    let accept = RwSignal::new(true);
    let requests = RwSignal::new(0);
    let entries = RwSignal::new(vec![
        PanelEntry::new("a"),
        PanelEntry::new("b"),
        PanelEntry::new("c"),
    ]);
    let direction = RwSignal::new(Direction::LeftToRight);
    let accordion = RwSignal::new(accordion::AccordionValue::Single(Some("first".into())));
    let open = RwSignal::new(false);
    let auto = RwSignal::new(Some("x".to_string()));
    let auto_requests = RwSignal::new(0);
    let cancel = RwSignal::new(false);
    let target_calls = RwSignal::new(0);
    let sections = RwSignal::new(vec!["one", "two", "three"]);
    let disabled_section = RwSignal::new(String::new());
    let multiple = RwSignal::new(accordion::AccordionValue::Multiple(vec![]));
    let view = RwSignal::new(Some("list".to_string()));
    view! {<section id="panel-parts" aria-label="Panel parts" data-testid="panel-parts">
        <h2>"Entry details"</h2>
        <button type="button" data-testid="panel-refuse" on:click=move |_|accept.update(|v|*v=!*v)>"Accept or refuse changes"</button>
        <button type="button" data-testid="panel-direction" on:click=move |_|direction.update(|v|*v=if *v==Direction::LeftToRight{Direction::RightToLeft}else{Direction::LeftToRight})>"Change direction"</button>
        <button type="button" data-testid="panel-cancel" on:click=move |_|cancel.update(|value|*value=!*value)>"Cancel target default"</button>
        <tabs::Root value=auto entries=vec![PanelEntry::new("x"),PanelEntry::new("y")] test_id="automatic-tabs" on_change=move |next|{auto_requests.update(|n|*n+=1);if accept.get_untracked(){auto.set(next);}}>
            <tabs::List aria_label="Automatic sections">
                <For each=||vec!["x","y"] key=|key|*key let:key>{tabs::trigger_as(leptos::html::button().attr("data-testid",format!("auto-{key}"))
                    .on(leptos::ev::click,move |event|{target_calls.update(|count|*count+=1);if cancel.get_untracked(){event.prevent_default();}})
                    .on(leptos::ev::keydown,move |event|{if cancel.get_untracked(){event.prevent_default();}}).child(key),key)}</For>
            </tabs::List>
            <tabs::Content value="x">"Automatic X"</tabs::Content><tabs::Content value="y">"Automatic Y"</tabs::Content>
        </tabs::Root>
        <output data-testid="auto-request-count">{move ||auto_requests.get()}</output><output data-testid="target-calls">{move ||target_calls.get()}</output>
        <DirectionProvider direction=direction>
        <tabs::Root value=value entries=entries activation_mode=ActivationMode::Manual test_id="manual-tabs"
            on_change=move |next|{requests.update(|n|*n+=1);if accept.get_untracked(){value.set(next);}}>
            <tabs::List aria_label="Entry sections"><For each=move ||entries.get() key=|entry|entry.value.clone() let:entry>{
                let key=entry.value.clone();
                tabs::trigger_as(leptos::html::button().attr("data-testid",format!("manual-{}",entry.value)).on(leptos::ev::keydown,move |event|{
                    if event.key()=="Delete" {entries.update(|entries|entries.retain(|entry|entry.value!=key));event.prevent_default();}
                    if event.key()=="d" {entries.update(|entries|{if let Some(entry)=entries.iter_mut().find(|entry|entry.value==key){entry.disabled=true;}});event.prevent_default();}
                }).child(format!("Section {}",entry.value)),entry.value)
            }</For></tabs::List>
            <tabs::Content value="a"><label>"Notes"<input data-testid="panel-notes" value="Retained notes" /></label></tabs::Content>
            <tabs::Content value="b">"Section b content"</tabs::Content><tabs::Content value="c">"Section c content"</tabs::Content>
        </tabs::Root>
        </DirectionProvider>
        <accordion::Root value=accordion on_change=move |next|{requests.update(|n|*n+=1);if accept.get_untracked(){accordion.set(next);}}>
            <accordion::Item value="first"><accordion::Header><accordion::Trigger test_id="accordion-first">"Overview"</accordion::Trigger></accordion::Header><accordion::Content force_mount=true test_id="accordion-first-content">"Overview detail"</accordion::Content></accordion::Item>
            <accordion::Item value="second"><accordion::Header><accordion::Trigger test_id="accordion-second">"History"</accordion::Trigger></accordion::Header><accordion::Content force_mount=true test_id="accordion-second-content">"History detail"</accordion::Content></accordion::Item>
        </accordion::Root>
        <accordion::Root value=multiple on_change=move |next|multiple.set(next) test_id="dynamic-accordion">
            <For each=move ||sections.get() key=|key|*key let:key>
                <accordion::Item value=key disabled=Signal::derive(move ||disabled_section.get()==key)>
                    <accordion::Header>{accordion::trigger_as(leptos::html::button().attr("data-testid",format!("dynamic-{key}"))
                        .on(leptos::ev::keydown,move |event|{match event.key().as_str(){
                            "r"=>{sections.update(|keys|keys.reverse());event.prevent_default();}
                            "Delete"=>{sections.update(|keys|keys.retain(|value|*value!=key));event.prevent_default();}
                            "d"=>{disabled_section.set(key.into());event.prevent_default();}
                            _=>{}
                        }}).child(key))}</accordion::Header>
                    <accordion::Content force_mount=true>"Dynamic section"</accordion::Content>
                </accordion::Item>
            </For>
        </accordion::Root>
        <button type="button" data-testid="remove-outside" on:click=move |_|sections.set(vec![])>"Clear dynamic sections"</button>
        <collapsible::Root open=open on_open_change=move |next|{requests.update(|n|*n+=1);if accept.get_untracked(){open.set(next);}}>
            <collapsible::Trigger test_id="collapsible-trigger">"Supplementary information"</collapsible::Trigger>
            <collapsible::Content force_mount=true test_id="collapsible-content"><button>"Retained action"</button></collapsible::Content>
        </collapsible::Root>
        <form id="panel-choice-form">
        <rustify_components::segmented_control::Root value=view on_change=move |value|view.set(value) name="panel-view" aria_label="Presentation">
            <rustify_components::segmented_control::Item value="list">"List"</rustify_components::segmented_control::Item>
            <rustify_components::segmented_control::Item value="grid">"Grid"</rustify_components::segmented_control::Item>
        </rustify_components::segmented_control::Root>
        </form>
        <rustify_components::tab_nav::Root aria_label="Local pages"><rustify_components::tab_nav::Link href="#panel-parts" active=true>"Details"</rustify_components::tab_nav::Link><rustify_components::tab_nav::Link href="/panels.html" target="_blank" rel="noopener">"Open panel"</rustify_components::tab_nav::Link></rustify_components::tab_nav::Root>
        <CorePosition /><CoreHierarchy /><CoreOutside /><CoreLifecycle /><CoreReturnFocus /><CoreRadioForms /><CoreScopeViewports /><DialogParts /><ProgramDialog /><DynamicDialog /><FloatingPanelSurface />
        <output data-testid="panel-value">{move ||value.get().unwrap_or_default()}</output><output data-testid="panel-requests">{move ||requests.get()}</output>
    </section>}
}

#[component]
fn CorePosition() -> impl IntoView {
    use rustify_ui::{Anchor, FloatingPosition, Layer, Side};
    let trigger = NodeRef::<leptos::html::Button>::new();
    let open = RwSignal::new(false);
    let position = RwSignal::new(FloatingPosition {
        side_offset: 8.,
        avoid_collisions: false,
        ..Default::default()
    });
    let anchor = Signal::derive(move || {
        trigger
            .get()
            .map(|node| Anchor::element(&node.into()))
            .unwrap_or(Anchor::Centred)
    });
    view! {<section aria-label="Core floating geometry">
        <button type="button" node_ref=trigger data-testid="core-anchor" on:click=move |_|open.set(true)>"Open geometry layer"</button>
        <For each=||vec![("top",Side::Top),("right",Side::Right),("bottom",Side::Bottom),("left",Side::Left)] key=|(key,_)|*key let:entry>
            <button type="button" data-testid=format!("core-{}",entry.0) on:click=move |_|position.update(|value|value.side=entry.1)>{entry.0}</button>
        </For>
        <button type="button" data-testid="core-invalid" on:click=move |_|position.update(|value|value.side_offset=f64::NAN)>"Reject non-finite offset"</button>
        <button type="button" data-testid="core-boundary" on:click=move |_|position.update(|value|{value.side_offset=8.;value.collision_boundary_ids=vec!["missing-boundary".into()];})>"Reject missing boundary"</button>
        <Show when=move ||open.get()><Layer anchor=anchor position=Some(position.into()) test_id="core-layer" on_close=move ||open.set(false)>
            <div>"Positioned content"</div>
        </Layer></Show>
    </section>}
}

#[component]
fn CoreHierarchy() -> impl IntoView {
    use rustify_ui::{Anchor, Layer};
    let outer = RwSignal::new(false);
    let outer_active = RwSignal::new(true);
    let child = RwSignal::new(false);
    let modal = RwSignal::new(false);
    let closed = RwSignal::new(0);
    let opened = RwSignal::new(0);
    let trigger = NodeRef::<leptos::html::Button>::new();
    let anchor = Signal::derive(move || trigger.get().map(|node| Anchor::element(&node.into())).unwrap_or(Anchor::Centred));
    view! { <section aria-label="Nested layer ownership">
        <CoreNotifications deactivate=Callback::new(move |_|outer_active.set(false)) />
        <button type="button" data-testid="tree-open" id="tree-trigger" on:click=move |_|outer.set(true)>"Edit nested details"</button>
        <button type="button" data-testid="tree-activate" on:click=move |_|outer_active.set(true)>"Restore editor"</button>
        <output data-testid="tree-close-count">{move ||closed.get()}</output>
        <output data-testid="tree-open-count">{move ||opened.get()}</output>
        <Show when=move ||outer.get()><Layer anchor=Anchor::Centred active=outer_active modal=true trigger_id="tree-trigger" test_id="tree-outer"
            on_open_auto_focus=Callback::new(move |_|opened.update(|count|*count+=1))
            on_close_auto_focus=Callback::new(move |_|closed.update(|count|*count+=1))
            on_close=move ||outer.set(false)>
            <button type="button" data-testid="tree-first" node_ref=trigger on:click=move |_|child.set(true)>"Open child"</button>
            <button type="button" data-testid="tree-disable" on:click=move |_|outer_active.set(false)>"Hide editor"</button>
            <Show when=move ||child.get()><Layer anchor=anchor test_id="tree-child" on_close=move ||child.set(false)>
                <button type="button" data-testid="tree-child-first" on:click=move |_|modal.set(true)>"Open confirmation"</button>
                <button type="button" data-testid="tree-child-last">"Child last"</button>
                <Show when=move ||modal.get()><Layer anchor=Anchor::Centred modal=true test_id="tree-modal" on_close=move ||modal.set(false)>
                    <button type="button" data-testid="tree-modal-first">"Cancel confirmation"</button>
                    <button type="button" data-testid="tree-modal-last">"Confirm"</button>
                </Layer></Show>
            </Layer></Show>
        </Layer></Show>
    </section> }
}

#[component]
fn CoreNotifications(deactivate: Callback<()>) -> impl IntoView {
    use rustify_ui::{provide_toasts, ToastOptions, ToastRegion};
    let owner = Owner::new();
    let view = owner.with(move || {
        let toasts = provide_toasts();
        let render = Callback::new(move |_: rustify_ui::Toast| view! { <CoreNotificationBody deactivate=deactivate /> }.into_any());
        view! {
            <button type="button" data-testid="tree-toast-show" on:click=move |_|{toasts.show("Saved",ToastOptions {duration_ms:600_000,..Default::default()});}>"Show notification task"</button>
            <ToastRegion render=render test_id="tree-toast-region" />
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
fn CoreNotificationBody(deactivate: Callback<()>) -> impl IntoView {
    use rustify_ui::{Anchor, Layer};
    let trigger = NodeRef::<leptos::html::Button>::new();
    let open = RwSignal::new(false);
    let modal = RwSignal::new(false);
    let anchor = Signal::derive(move ||trigger.get().map(|node|Anchor::element(&node.into())).unwrap_or(Anchor::Centred));
    view! { <div data-testid="tree-toast">
        <button type="button" node_ref=trigger data-testid="tree-toast-action" on:click=move |_|open.set(true)>"Edit saved task"</button>
        <Show when=move ||open.get()><Layer anchor=anchor fit=true test_id="tree-notice-popup" on_close=move ||open.set(false)>
            <input data-testid="tree-notice-input" aria-label="Notification draft" on:keydown=move |event|{if event.key()=="F2"{deactivate.run(());event.prevent_default();}} />
            <button type="button" data-testid="tree-notice-confirm" on:click=move |_|modal.set(true)>"Review confirmation"</button>
            <Show when=move ||modal.get()><Layer anchor=Anchor::Centred modal=true test_id="tree-notice-modal" on_close=move ||modal.set(false)>
                <button type="button" data-testid="tree-notice-cancel" on:keydown=move |event|{if event.key()=="F2"{deactivate.run(());event.prevent_default();}}>"Cancel saved task confirmation"</button>
            </Layer></Show>
        </Layer></Show>
    </div> }
}

#[component]
fn CoreOutside() -> impl IntoView {
    use rustify_ui::{Anchor, Cancelable, Layer, LayerDismiss};
    let open = RwSignal::new(false);
    let cancel = RwSignal::new(false);
    let requests = RwSignal::new(0);
    let trace = RwSignal::new(String::new());
    let panel = NodeRef::<leptos::html::Div>::new();
    let trigger = NodeRef::<leptos::html::Button>::new();
    let anchor = Signal::derive(move ||trigger.get().map(|node|Anchor::element(&node.into())).unwrap_or(Anchor::Centred));
    let specific = Callback::new(move |event: Cancelable<LayerDismiss>| {
        trace.update(|trace|trace.push(if *event.value()==LayerDismiss::PointerOutside {'p'}else{'f'}));
        if cancel.get_untracked() {event.prevent_default();}
    });
    let interact = Callback::new(move |event: Cancelable<LayerDismiss>| trace.update(|trace|trace.push(if event.default_prevented(){'c'}else{'i'})));
    view! { <section aria-label="Outside interaction requests">
        <button type="button" data-testid="outside-open" node_ref=trigger on:click=move |_|open.set(true)>"Open nonmodal editor"</button>
        <output data-testid="outside-requests">{move ||requests.get()}</output><output data-testid="outside-trace">{move ||trace.get()}</output>
        <Show when=move ||open.get()><Layer anchor=anchor panel=Some(panel) test_id="outside-shell" on_close=move ||requests.update(|count|*count+=1)
            on_pointer_down_outside=specific on_focus_outside=specific on_interact_outside=interact>
            <div node_ref=panel role="dialog" aria-label="Actual interaction panel" data-testid="outside-panel" tabindex="-1">
                <input data-testid="outside-input" aria-label="Outside test draft" />
                <button type="button" data-testid="outside-cancel" on:click=move |_|{cancel.update(|value|*value=!*value);trace.set(String::new());}>"Change cancellation"</button>
            </div>
            <button type="button" data-testid="outside-cover">"Surrounding surface"</button>
        </Layer></Show>
    </section> }
}

#[component]
fn CoreLifecycle() -> impl IntoView {
    use rustify_ui::{Anchor, Layer, use_overlay};
    let open = RwSignal::new(false);
    let trigger_present = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let panel_present = RwSignal::new(true);
    let requests = RwSignal::new(0);
    let opened = RwSignal::new(0);
    let closed = RwSignal::new(0);
    let panel = NodeRef::<leptos::html::Div>::new();
    let stack = use_overlay();
    view! { <section aria-label="Layer lifetime">
        <Show when=move ||trigger_present.get()><button type="button" id="life-trigger" data-testid="life-open" disabled=move ||disabled.get() on:click=move |_|open.set(true)>"Open lifetime editor"</button></Show>
        <button type="button" data-testid="life-restore" on:click=move |_|{trigger_present.set(true);disabled.set(false);panel_present.set(true);}>"Restore editor parts"</button>
        <output data-testid="life-requests">{move ||requests.get()}</output>
        <output data-testid="life-opened">{move ||opened.get()}</output><output data-testid="life-closed">{move ||closed.get()}</output>
        <output data-testid="life-depth">{move ||{requests.track();opened.track();closed.track();stack.as_ref().map_or(0,|stack|stack.depth())}}</output>
        <Show when=move ||open.get()><Layer anchor=Anchor::Centred trigger_id="life-trigger" panel=Some(panel) modal=true test_id="life-shell"
            on_close=move ||requests.update(|count|*count+=1)
            on_open_auto_focus=Callback::new(move |_|opened.update(|count|*count+=1)) on_close_auto_focus=Callback::new(move |_|closed.update(|count|*count+=1))>
            <Show when=move ||panel_present.get()><div node_ref=panel role="dialog" aria-label="Lifetime editor" tabindex="-1" data-testid="life-panel">
                <button type="button" data-testid="life-remove-panel" on:click=move |_|panel_present.set(false)>"Remove panel"</button>
                <button type="button" data-testid="life-remove-trigger" on:click=move |_|trigger_present.set(false)>"Remove trigger"</button>
                <button type="button" data-testid="life-disable-trigger" on:click=move |_|disabled.set(true)>"Disable trigger"</button>
            </div></Show>
        </Layer></Show>
    </section> }
}
#[component]
fn CoreReturnFocus() -> impl IntoView {
    use rustify_ui::{Anchor, Layer};
    let open = RwSignal::new(false);
    let panel = NodeRef::<leptos::html::Div>::new();
    view! { <section aria-label="Return focus decisions">
        <button type="button" id="return-trigger" data-testid="return-open" on:click=move |_|open.set(true)>"Open focus editor"</button>
        <button type="button" data-testid="return-external">"Continue elsewhere"</button>
        <Show when=move ||open.get()><Layer anchor=Anchor::Centred trigger_id="return-trigger" panel=Some(panel) test_id="return-shell"
            on_pointer_down_outside=Callback::new(|_|{}) on_close=move ||open.set(false)>
            <div data-testid="return-backdrop">"Outside the panel"</div>
            <div node_ref=panel role="dialog" aria-label="Focus editor" tabindex="-1"><input data-testid="return-input" aria-label="Focus draft" /></div>
        </Layer></Show>
    </section> }
}
#[component]
fn CoreRadioForms() -> impl IntoView {
    use rustify_ui::{Anchor, Layer};
    let open = RwSignal::new(false);
    view! { <section aria-label="Radio form focus domains">
        <button type="button" data-testid="radio-forms-open" on:click=move |_|open.set(true)>"Open radio forms"</button>
        <Show when=move ||open.get()><Layer anchor=Anchor::Centred modal=true test_id="radio-forms-layer" on_close=move ||open.set(false)>
            <form id="radio-form-a"><label>"First form"<input type="radio" name="shared-name" checked=true data-testid="radio-form-a" /></label></form>
            <form id="radio-form-b"></form>
            <label>"Second form"<input type="radio" name="shared-name" form="radio-form-b" data-testid="radio-form-b" /></label>
        </Layer></Show>
    </section> }
}

#[component]
fn CoreScopeViewports() -> impl IntoView {
    use rustify_ui::{mount, AppHandle, MountConfig};
    let a = NodeRef::<leptos::html::Div>::new();
    let b = NodeRef::<leptos::html::Div>::new();
    let handles = StoredValue::new_local(Vec::<AppHandle>::new());
    let error = RwSignal::new(String::new());
    let hits = RwSignal::new(0);
    let hidden = RwSignal::new(false);
    Effect::new(move || {
        let (Some(a),Some(b))=(a.get(),b.get()) else {return;};
        if !handles.with_value(|handles|handles.is_empty()) {return;}
        for (key,node) in [("viewport-a",a),("viewport-b",b)] {
            match mount(node.into(),MountConfig {scope:key.into(),..Default::default()},move ||view!{<CoreScopedViewport scope_key=key />}) {
                Ok(handle)=>handles.update_value(|handles|handles.push(handle)),Err(issue)=>error.set(issue.to_string()),
            }
        }
    });
    on_cleanup(move ||{handles.try_update_value(|handles|handles.clear());});
    view! {<section aria-label="Independent scoped viewports">
        <button type="button" data-testid="viewport-host" on:click=move |_|hits.update(|n|*n+=1)>"Use host control"</button>
        <button type="button" data-testid="viewport-hide" on:click=move |_|hidden.update(|value|*value=!*value)>"Hide or restore first scope"</button>
        <output data-testid="viewport-host-hits">{move ||hits.get()}</output><output data-testid="viewport-error">{move ||error.get()}</output>
        <div data-testid="viewport-row"><div data-testid="viewport-a" node_ref=a hidden=move ||hidden.get()></div><div data-testid="viewport-b" node_ref=b></div></div>
    </section>}
}
#[component]
fn CoreScopedViewport(#[prop(into)] scope_key:String) -> impl IntoView {
    use rustify_ui::{Anchor, Layer, ViewportAlign};
    let key=StoredValue::new(scope_key);
    let open=RwSignal::new(false);
    let align=RwSignal::new(ViewportAlign::Center);
    let requests=RwSignal::new(0);
    let panel=NodeRef::<leptos::html::Div>::new();
    let anchor=Signal::derive(move ||Anchor::Viewport {align:align.get()});
    view! {
        <ScopedPopover scope_key=key.get_value() />
        <button type="button" id=format!("{}-trigger",key.get_value()) data-testid=format!("{}-open",key.get_value()) on:click=move |_|open.set(true)>"Open scoped editor"</button>
        <output data-testid=format!("{}-requests",key.get_value())>{move ||requests.get()}</output>
        <Show when=move ||open.get()><Layer modal=true anchor=anchor panel=Some(panel) trigger_id=format!("{}-trigger",key.get_value()) test_id=format!("{}-shell",key.get_value()) on_close=move ||requests.update(|n|*n+=1)>
            <div data-core-scope-shade="" data-testid=format!("{}-shade",key.get_value())></div>
            <div node_ref=panel data-core-scope-panel="" data-testid=format!("{}-panel",key.get_value()) role="dialog" aria-label="Scoped editor" tabindex="-1">
                <button type="button" data-testid=format!("{}-align",key.get_value()) on:click=move |_|align.set(ViewportAlign::Start)>"Align to start"</button>
                <button type="button" data-testid=format!("{}-close",key.get_value()) on:click=move |_|open.set(false)>"Close editor"</button>
            </div>
        </Layer></Show>
    }
}

#[component]
fn DialogParts() -> impl IntoView {
    use rustify_components::dialog;
    let open=RwSignal::new(false);
    let layout=RwSignal::new(rustify_components::Layout::default());
    let size=RwSignal::new(dialog::DialogSize::default());
    let accept=RwSignal::new(true);
    let cancel=RwSignal::new(false);
    let native_cancel=RwSignal::new(false);
    let custom_focus=RwSignal::new(false);
    let reopen=RwSignal::new(false);
    let description=RwSignal::new(true);
    let title=RwSignal::new(true);
    let empty_title=RwSignal::new(false);
    let requests=RwSignal::new(0);
    let opened=RwSignal::new(0);
    let closed=RwSignal::new(0);
    let targets=RwSignal::new(0);
    let escape=Callback::new(move |event:rustify_ui::Cancelable<rustify_ui::LayerDismiss>|{if cancel.get_untracked(){event.prevent_default();}});
    view! { <section aria-label="Dialog parts">
        <h2>"Task editor"</h2>
        <button type="button" data-testid="parts-dialog-refuse" on:click=move |_|accept.update(|value|*value=!*value)>"Accept or refuse dialog changes"</button>
        <button type="button" data-testid="parts-dialog-cancel" on:click=move |_|cancel.update(|value|*value=!*value)>"Cancel dismiss events"</button>
        <button type="button" data-testid="parts-dialog-native" on:click=move |_|native_cancel.update(|value|*value=!*value)>"Cancel native target action"</button>
        <button type="button" data-testid="parts-dialog-focus" on:click=move |_|custom_focus.update(|value|*value=!*value)>"Use custom focus"</button>
        <button type="button" data-testid="parts-dialog-reopen" on:click=move |_|reopen.update(|value|*value=!*value)>"Reopen from close callback"</button>
        <output data-testid="parts-dialog-requests">{move ||requests.get()}</output><output data-testid="parts-dialog-opened">{move ||opened.get()}</output><output data-testid="parts-dialog-closed">{move ||closed.get()}</output><output data-testid="parts-dialog-targets">{move ||targets.get()}</output>
        <dialog::Root open=open on_open_change=move |next|{requests.update(|count|*count+=1);if accept.get_untracked(){open.set(next);}}>
            {dialog::trigger_as(leptos::html::button().attr("data-testid","parts-dialog-trigger").attr("data-custom","retained").on(leptos::ev::click,move |event|{
                targets.update(|count|*count+=1);if native_cancel.get_untracked(){event.prevent_default();}
            }).child("Edit task details"))}
            <dialog::Portal force_mount=true>
                <dialog::Overlay test_id="parts-dialog-overlay" />
                <dialog::Content size=Signal::derive(move ||rustify_components::Responsive::from(size.get())) layout=layout test_id="parts-dialog-content" on_escape_key_down=escape on_pointer_down_outside=escape
                    on_open_auto_focus=Callback::new(move |event:rustify_ui::Cancelable<()>|{opened.update(|count|*count+=1);if custom_focus.get_untracked() || reopen.get_untracked(){event.prevent_default();focus_test_element("parts-dialog-title");}})
                    on_close_auto_focus=Callback::new(move |event:rustify_ui::Cancelable<()>|{closed.update(|count|*count+=1);if reopen.get_untracked(){open.set(true);}else if custom_focus.get_untracked(){event.prevent_default();focus_test_element("parts-dialog-native");}})>
                    <Show when=move ||title.get()><dialog::Title test_id="parts-dialog-title" attr:tabindex="-1">{move ||if empty_title.get(){"  "}else{"Edit task details"}}</dialog::Title></Show>
                    <Show when=move ||description.get()><dialog::Description>"Changes stay in this task."</dialog::Description></Show>

                    <label>"Task name"<input data-testid="parts-dialog-input" /></label>
                    <button type="button" data-testid="parts-dialog-size" on:click=move |_|size.update(|value|*value=dialog::DialogSize::ALL[value.number() as usize%4])>"Next panel size"</button>
                    <button type="button" data-testid="parts-dialog-layout" on:click=move |_|layout.set(rustify_components::Layout {width:rustify_components::Responsive::from(String::from("800px")),max_width:rustify_components::Responsive::from(String::from("300px")),height:rustify_components::Responsive::from(String::from("300px")),max_height:rustify_components::Responsive::from(String::from("180px")),..Default::default()})>"Use compact dimensions"</button>
                    <button type="button" data-testid="parts-dialog-description" on:click=move |_|description.update(|value|*value=!*value)>"Toggle explanation"</button>
                    <button type="button" data-testid="parts-dialog-title-empty" on:click=move |_|empty_title.update(|value|*value=!*value)>"Toggle title text"</button>
                    <button type="button" data-testid="parts-dialog-title-toggle" on:click=move |_|title.update(|value|*value=!*value)>"Toggle title"</button>
                    <button type="button" data-testid="parts-dialog-refuse-inside" on:click=move |_|accept.update(|value|*value=!*value)>"Accept or refuse closing"</button>
                    <button type="button" data-testid="parts-dialog-native-inside" on:click=move |_|native_cancel.update(|value|*value=!*value)>"Cancel native actions"</button>
                    <button type="button" data-testid="parts-dialog-reopen-inside" on:click=move |_|reopen.set(false)>"Stop reopening"</button>
                    <AlertParts accept=accept cancel=cancel native_cancel=native_cancel />
                    <dialog::Close test_id="parts-dialog-close">"Close task editor"</dialog::Close>
                </dialog::Content>
            </dialog::Portal>
        </dialog::Root>
    </section> }
}
#[component]
fn AlertParts(accept:RwSignal<bool>,cancel:RwSignal<bool>,native_cancel:RwSignal<bool>) -> impl IntoView {
    use rustify_components::alert_dialog;
    let open=RwSignal::new(false);
    let actions=RwSignal::new(0);
    let requests=RwSignal::new(0);
    let cancel_action=RwSignal::new(false);
    let missing_cancel=RwSignal::new(false);
    let empty_description=RwSignal::new(false);
    let escape=Callback::new(move |event:rustify_ui::Cancelable<rustify_ui::LayerDismiss>|{if cancel.get_untracked(){event.prevent_default();}});
    view! { <section aria-label="Removal confirmation">
        <button type="button" data-testid="parts-alert-missing" on:click=move |_|missing_cancel.update(|value|*value=!*value)>"Show confirmation without Cancel"</button>
        <output data-testid="parts-alert-actions">{move ||actions.get()}</output><output data-testid="parts-alert-requests">{move ||requests.get()}</output>
        <alert_dialog::Root open=open on_open_change=move |next|{requests.update(|count|*count+=1);if accept.get_untracked(){open.set(next);}}>
            <alert_dialog::Trigger test_id="parts-alert-trigger">"Remove task"</alert_dialog::Trigger>
            <alert_dialog::Portal>
                <alert_dialog::Overlay test_id="parts-alert-overlay" />
                <alert_dialog::Content test_id="parts-alert-content" on_escape_key_down=escape>
                    <alert_dialog::Title>"Remove this task?"</alert_dialog::Title>
                    <alert_dialog::Description>{move ||if empty_description.get(){"  "}else{"The task and its unsaved changes will be removed."}}</alert_dialog::Description>
                    <button type="button" data-testid="parts-alert-description-empty" on:click=move |_|empty_description.update(|value|*value=!*value)>"Toggle explanation text"</button>
                    <button type="button" data-testid="parts-alert-refuse" on:click=move |_|accept.update(|value|*value=!*value)>"Accept or refuse closing"</button>
                    {alert_dialog::action_as(leptos::html::button().attr("data-testid","parts-alert-action").on(leptos::ev::click,move |event|{if native_cancel.get_untracked(){event.prevent_default();}}).child("Remove task permanently"),
                        Callback::new(move |event:rustify_ui::Cancelable<()>|{actions.update(|count|*count+=1);if cancel_action.get_untracked(){event.prevent_default();}}))}
                    <Show when=move ||!missing_cancel.get()><alert_dialog::Cancel test_id="parts-alert-cancel">"Keep task"</alert_dialog::Cancel></Show>
                    <button type="button" data-testid="parts-alert-cancel-action" on:click=move |_|cancel_action.update(|value|*value=!*value)>"Cancel confirmation action"</button>
                </alert_dialog::Content>
            </alert_dialog::Portal>
        </alert_dialog::Root>
    </section> }
}

fn focus_test_element(id:&str) {
    use leptos::wasm_bindgen::JsCast;
    if let Ok(Some(element))=document().query_selector(&format!("[data-testid='{id}']")) {
        if let Some(element)=element.dyn_ref::<web_sys::HtmlElement>() {let _=element.focus();}
    }
}
#[component]
fn ProgramDialog() -> impl IntoView {
    use rustify_components::dialog;
    let open=RwSignal::new(false);let closed=RwSignal::new(0);
    view!{<section aria-label="Programmatic modeless editor">
        <button type="button" data-testid="program-dialog-open" on:click=move |_|open.set(true)>"Open modeless editor"</button>
        <output data-testid="program-dialog-closed">{move ||closed.get()}</output>
        <dialog::Root open=open modal=false on_open_change=move |next|open.set(next)>
            <dialog::Portal><dialog::Overlay test_id="program-dialog-overlay" /><dialog::Content test_id="program-dialog-content" on_close_auto_focus=Callback::new(move |_|closed.update(|count|*count+=1))>
                <dialog::Title>"Modeless task notes"</dialog::Title>
                <input data-testid="program-dialog-input" aria-label="Task notes" />
                <dialog::Close test_id="program-dialog-close">"Close notes"</dialog::Close>
            </dialog::Content></dialog::Portal>
        </dialog::Root>
    </section>}
}

#[component]
fn DynamicDialog() -> impl IntoView {
    use rustify_components::dialog;
    let open=RwSignal::new(false);
    let trigger=RwSignal::new(false);
    let duplicate=RwSignal::new(false);
    let duplicate_trigger=RwSignal::new(false);
    let closed=RwSignal::new(0);
    view!{<section aria-label="Dynamic dialog composition">
        <button type="button" data-testid="dynamic-dialog-trigger-toggle" on:click=move |_|trigger.update(|value|*value=!*value)>"Toggle trigger"</button>
        <button type="button" data-testid="dynamic-dialog-trigger-duplicate" on:click=move |_|duplicate_trigger.update(|value|*value=!*value)>"Toggle second trigger"</button>
        <button type="button" data-testid="dynamic-dialog-repair" on:click=move |_|duplicate.set(false)>"Remove duplicate content"</button>
        <output data-testid="dynamic-dialog-closed">{move ||closed.get()}</output>
        <dialog::Root open=open on_open_change=move |next|open.set(next)>
            <Show when=move ||trigger.get()><dialog::Trigger test_id="dynamic-dialog-trigger">"Open dynamic editor"</dialog::Trigger></Show>
            <Show when=move ||duplicate_trigger.get()><dialog::Trigger test_id="dynamic-dialog-trigger-two">"Second trigger"</dialog::Trigger></Show>
            <dialog::Portal force_mount=true>
                <dialog::Overlay />
                <dialog::Content test_id="dynamic-dialog-content" on_close_auto_focus=Callback::new(move |_|closed.update(|count|*count+=1))>
                    <dialog::Title>"Dynamic editor"</dialog::Title>
                    <input data-testid="dynamic-dialog-input" aria-label="Dynamic note" />
                    <button type="button" data-testid="dynamic-dialog-duplicate" on:click=move |_|duplicate.set(true)>"Add duplicate content"</button>
                    <dialog::Close test_id="dynamic-dialog-close">"Close editor"</dialog::Close>
                </dialog::Content>
                <Show when=move ||duplicate.get()><dialog::Content test_id="dynamic-dialog-content-two"><dialog::Title>"Duplicate editor"</dialog::Title><input aria-label="Duplicate note" /></dialog::Content></Show>
            </dialog::Portal>
        </dialog::Root>
    </section>}
}

#[component]
fn PopoverParts() -> impl IntoView {
    use rustify_components::popover;
    let size=RwSignal::new(popover::PopoverSize::default());
    let open=RwSignal::new(false);let label=RwSignal::new(String::from("Task schedule"));let accept=RwSignal::new(true);let cancel=RwSignal::new(false);let native=RwSignal::new(false);
    let delayed=RwSignal::new(false);let pending=RwSignal::new(None::<bool>);
    let anchor=RwSignal::new(true);let arrow=RwSignal::new(false);let requests=RwSignal::new(0);let clicks=RwSignal::new(0);let targets=RwSignal::new(0);
    let dismiss=Callback::new(move |event:rustify_ui::Cancelable<rustify_ui::LayerDismiss>|if cancel.get_untracked(){event.prevent_default();});
    let position=RwSignal::new(rustify_ui::FloatingPosition {align:rustify_ui::Align::Start,side_offset:8.,avoid_collisions:false,..Default::default()});
    view!{<section aria-label="Anchored task editor">
        <button type="button" data-testid="popover-delay" on:click=move |_|delayed.update(|value|*value=!*value)>"Delay application decisions"</button>
        <button type="button" data-testid="popover-apply" on:click=move |_|{if let Some(next)=pending.get_untracked(){pending.set(None);open.set(next);}}>"Apply delayed change"</button>
        <button type="button" data-testid="popover-native" on:click=move |_|native.update(|value|*value=!*value)>"Cancel target default"</button>
        <button type="button" data-testid="popover-cancel" on:click=move |_|cancel.update(|value|*value=!*value)>"Cancel outside and Escape"</button>
        <button type="button" data-testid="popover-restore" on:click=move |_|anchor.set(true)>"Restore position"</button>
        <button type="button" data-testid="popover-outside" on:click=move |_|clicks.update(|count|*count+=1)>"Continue outside editor"</button>
        <output data-testid="popover-requests">{move ||requests.get()}</output><output data-testid="popover-clicks">{move ||clicks.get()}</output><output data-testid="popover-targets">{move ||targets.get()}</output>
        <popover::Root open=open on_open_change=move |next|{requests.update(|count|*count+=1);if delayed.get_untracked(){pending.set(Some(next));}else if accept.get_untracked(){open.set(next);}}>
            <Show when=move ||anchor.get()><popover::Anchor test_id="popover-anchor" attr:data-arrow=move ||arrow.get().then_some("")>"Task position"</popover::Anchor></Show>
            {popover::trigger_as(leptos::html::button().attr("data-testid","popover-trigger").attr("data-custom","retained")
                .on(leptos::ev::click,move |event|{targets.update(|count|*count+=1);if native.get_untracked(){event.prevent_default();}}).child("Edit task schedule"))}
            <popover::Portal force_mount=true><popover::Content size=Signal::derive(move ||rustify_components::Responsive::from(size.get())) layout=rustify_components::Layout {width:rustify_components::Responsive::from(String::from("250px")),max_width:rustify_components::Responsive::from(String::from("200px")),..Default::default()} position=position aria_label=label test_id="popover-content"
                on_escape_key_down=dismiss on_pointer_down_outside=dismiss on_focus_outside=dismiss>
                <label>"Schedule note"<input data-testid="popover-input" /></label>
                <button type="button" data-testid="popover-name" on:click=move |_|label.update(|label|*label=if label.is_empty(){"Task schedule".into()}else{String::new()})>"Toggle name text"</button>
                <button type="button" data-testid="popover-refuse" on:click=move |_|accept.update(|value|*value=!*value)>"Accept or refuse closing"</button>
                <button type="button" data-testid="popover-remove" on:click=move |_|anchor.set(false)>"Remove position"</button>
                <button type="button" data-testid="popover-animation" on:click=move |_|position.update(|position|position.update=if position.update==rustify_ui::PositionUpdate::Always {rustify_ui::PositionUpdate::Optimized}else{rustify_ui::PositionUpdate::Always})>"Track position animation"</button>
                <button type="button" data-testid="popover-size" on:click=move |_|size.update(|value|*value=popover::PopoverSize::ALL[value.number() as usize%4])>"Next panel size"</button>
                <button type="button" data-testid="popover-detached" on:click=move |_|position.update(|position|position.hide_when_detached=!position.hide_when_detached)>"Hide detached position"</button>
                <button type="button" data-testid="popover-arrow-toggle" on:click=move |_|arrow.update(|value|*value=!*value)>"Show or hide pointer"</button>
                <For each=||vec![("top",rustify_ui::Side::Top),("right",rustify_ui::Side::Right),("bottom",rustify_ui::Side::Bottom),("left",rustify_ui::Side::Left)] key=|(name,_)|*name let:side>
                    <button type="button" data-testid=format!("popover-place-{}",side.0) on:click=move |_|position.update(|position|position.side=side.1)>{format!("Place {}",side.0)}</button>
                </For>
                <For each=||vec![("start",rustify_ui::Align::Start),("center",rustify_ui::Align::Center),("end",rustify_ui::Align::End)] key=|(name,_)|*name let:align>
                    <button type="button" data-testid=format!("popover-align-{}",align.0) on:click=move |_|position.update(|position|position.align=align.1)>{format!("Align {}",align.0)}</button>
                </For>
                <button type="button" data-testid="popover-boundary" on:click=move |_|position.update(|position|{position.avoid_collisions=true;position.collision_boundary_ids=vec!["panel-boundary".into()];})>"Keep inside panel boundary"</button>
                <Show when=move ||arrow.get()><popover::Arrow test_id="popover-arrow" /></Show>
                <popover::Close test_id="popover-close">"Close schedule"</popover::Close>
            </popover::Content></popover::Portal>
        </popover::Root>
    </section>}
}

#[component]
fn HoverParts() -> impl IntoView {
    use rustify_components::{hover_card,tooltip};
    let size=RwSignal::new(hover_card::HoverCardSize::default());
    let one=RwSignal::new(false);let two=RwSignal::new(false);let card=RwSignal::new(false);
    let mounted=RwSignal::new(true);let accept=RwSignal::new(true);let interactive=RwSignal::new(false);let native=RwSignal::new(false);
    let requests=RwSignal::new(0);let targets=RwSignal::new(0);
    let cancel_outside=Callback::new(move |event:rustify_ui::Cancelable<rustify_ui::LayerDismiss>|if native.get_untracked(){event.prevent_default();});
    let position=Signal::stored(rustify_ui::FloatingPosition {side_offset:16.,..Default::default()});
    view!{<section aria-label="Supplemental reading">
        <p id="hover-help">"Additional help remains available."</p>
        <button type="button" data-testid="hover-outside">"Continue reading"</button>
        <button type="button" data-testid="hover-unmount" on:click=move |_|mounted.update(|v|*v=!*v)>"Remove or restore descriptions"</button>
        <button type="button" data-testid="hover-accept" on:click=move |_|accept.update(|v|*v=!*v)>"Accept or refuse hover requests"</button>
        <button type="button" data-testid="hover-interactive" on:click=move |_|interactive.update(|v|*v=!*v)>"Add invalid tooltip action"</button>
        <button type="button" data-testid="hover-size" on:click=move |_|size.update(|value|*value=hover_card::HoverCardSize::ALL[value.number() as usize%3])>"Next reading card size"</button>
        <button type="button" data-testid="hover-native" on:click=move |_|native.update(|v|*v=!*v)>"Cancel native hover defaults"</button>
        <output data-testid="hover-requests">{move ||requests.get()}</output><output data-testid="hover-targets">{move ||targets.get()}</output>
        <Show when=move ||mounted.get()>
        <tooltip::Provider delay_ms=120 skip_delay_ms=400>
            <tooltip::Root open=one on_open_change=move |next|{requests.update(|n|*n+=1);if accept.get_untracked(){one.set(next);}}>
                {tooltip::trigger_as(leptos::html::button().r#type("button").attr("data-testid","tooltip-one").attr("aria-describedby","hover-help hover-help")
                    .on(leptos::ev::pointerenter,move |event|if native.get_untracked(){event.prevent_default();})
                    .on(leptos::ev::focusin,move |event|if native.get_untracked(){event.prevent_default();})
                    .on(leptos::ev::click,move |event|{targets.update(|n|*n+=1);if native.get_untracked(){event.prevent_default();}}).child("About schedule dates"))}
                <tooltip::Portal force_mount=true><tooltip::Content  position=position on_focus_outside=cancel_outside on_pointer_down_outside=cancel_outside test_id="tooltip-one-content">
                    "Dates use your local time zone. "<Show when=move ||interactive.get()><button type="button">"Invalid tooltip action"</button></Show><tooltip::Arrow />
                </tooltip::Content></tooltip::Portal>
            </tooltip::Root>
            <tooltip::Root open=two on_open_change=move |next|two.set(next)>
                <tooltip::Trigger test_id="tooltip-two">"About saved notes"</tooltip::Trigger>
                <tooltip::Portal><tooltip::Content  test_id="tooltip-two-content">"Notes stay with this task."</tooltip::Content></tooltip::Portal>
            </tooltip::Root>
        </tooltip::Provider>
        <hover_card::Root open=card open_delay_ms=120 close_delay_ms=90 on_open_change=move |next|card.set(next)>
            <hover_card::Trigger href="#panel-parts" attr:data-rustify-native-navigation="" test_id="hover-card-trigger">"Read task owner profile"</hover_card::Trigger>
            <hover_card::Portal><hover_card::Content size=Signal::derive(move ||rustify_components::Responsive::from(size.get())) position=rustify_ui::FloatingPosition {side_offset:16.,..Default::default()} test_id="hover-card-content">
                <p data-testid="hover-selectable">"Morgan owns this task and reviews schedule changes."</p><a href="#panel-parts" data-rustify-native-navigation="" data-testid="hover-card-link">"View profile details"</a><hover_card::Arrow />
            </hover_card::Content></hover_card::Portal>
        </hover_card::Root>
        </Show>
    </section>}
}

#[component]
fn FloatingPanelSurface()->impl IntoView {
    let owner=Owner::new();
    let view=owner.with(move || {
    let theme=rustify_ui::resolve(&rustify_ui::ThemeDocument::standard("panel-fixture","Panel fixture"),rustify_ui::ThemeMode::Light,&rustify_ui::ResolveContext::default()).expect("standard fixture theme is valid");
    rustify_ui::provide_static_theme(std::sync::Arc::new(theme));
    let dark=RwSignal::new(false);let rtl=RwSignal::new(false);
    let patch=Signal::derive(move || {let mut patch=rustify_ui::ThemeBoundaryPatch::default();patch.config.appearance=rustify_ui::ConfigChange::Set(if dark.get(){rustify_ui::Appearance::Dark}else{rustify_ui::Appearance::Light});patch});
    view!{<section aria-label="Floating panel presentation">
        <button type="button" data-testid="panel-theme" on:click=move |_|dark.update(|v|*v=!*v)>"Change panel appearance"</button>
        <button type="button" data-testid="panel-reading-direction" on:click=move |_|rtl.update(|v|*v=!*v)>"Change panel reading direction"</button>
        <rustify_ui::ThemeBoundary patch=patch test_id="panel-theme-boundary" attr:id="panel-boundary">
            <rustify_components::primitives::DirectionProvider direction=Signal::derive(move ||if rtl.get(){rustify_ui::Direction::RightToLeft}else{rustify_ui::Direction::LeftToRight})>
                <PopoverParts /><HoverParts /><crate::stage_panel::StagePanel />
            </rustify_components::primitives::DirectionProvider>
        </rustify_ui::ThemeBoundary>
    </section>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view,owner)
}

#[component]
fn ScopedPopover(scope_key:String)->impl IntoView {
    use rustify_components::popover;
    let key=StoredValue::new(scope_key);let open=RwSignal::new(false);let accept=RwSignal::new(true);let requests=RwSignal::new(0);
    view!{<popover::Root modal=true open=open on_open_change=move |next:bool|{if !next {requests.update(|n|*n+=1);}if next||accept.get_untracked(){open.set(next);}}>
        <popover::Trigger test_id=format!("{}-popover-open",key.get_value())>"Open scoped task options"</popover::Trigger>
        <output data-testid=format!("{}-popover-requests",key.get_value())>{move ||requests.get()}</output>
        <popover::Portal><popover::Content aria_label="Scoped task options" class="stage-options-content" position=rustify_ui::FloatingPosition{align:rustify_ui::Align::Start,..Default::default()} test_id=format!("{}-popover-content",key.get_value())>
            <label>"Scoped note"<input data-testid=format!("{}-popover-input",key.get_value()) /></label>
            <button type="button" data-testid=format!("{}-popover-refuse",key.get_value()) on:click=move |_|accept.set(false)>"Keep this panel open"</button>
            <popover::Close>"Close options"</popover::Close>
        </popover::Content></popover::Portal>
    </popover::Root>}
}
