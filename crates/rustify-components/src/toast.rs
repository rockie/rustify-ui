//! Notification skins over the core's controlled parts and single scheduler.

use crate::icon::{Glyph, Icon};
use crate::primitives::toast as raw;
use leptos::prelude::*;
pub use raw::{
    Provider, SwipeDirection, ToastAction, ToastConfig, ToastImportance, ToastIssue, ToastOptions,
    ToastQueuePolicy, ToastSwipe, ToastTone,
};

#[component]
pub fn Viewport(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <raw::Viewport class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast-viewport".into(), class.get()])) test_id=test_id>
            {children()}
        </raw::Viewport>
    }
}

#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(default = Signal::stored(ToastOptions::default().duration_ms), into)]
    duration_ms: Signal<u32>,
    #[prop(optional, into)] importance: Signal<ToastImportance>,
    #[prop(optional, into)] tone: Signal<ToastTone>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<
        Callback<rustify_ui::Cancelable<rustify_ui::LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_pause: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] on_resume: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] on_swipe_start: Option<
        Callback<rustify_ui::Cancelable<ToastSwipe>>,
    >,
    #[prop(optional_no_strip, into)] on_swipe_move: Option<
        Callback<rustify_ui::Cancelable<ToastSwipe>>,
    >,
    #[prop(optional_no_strip, into)] on_swipe_cancel: Option<
        Callback<rustify_ui::Cancelable<ToastSwipe>>,
    >,
    #[prop(optional_no_strip, into)] on_swipe_end: Option<
        Callback<rustify_ui::Cancelable<ToastSwipe>>,
    >,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <raw::Root open=open on_open_change=on_open_change duration_ms=duration_ms importance=importance
            on_escape_key_down=on_escape_key_down on_pause=on_pause on_resume=on_resume
            on_swipe_start=on_swipe_start on_swipe_move=on_swipe_move on_swipe_cancel=on_swipe_cancel on_swipe_end=on_swipe_end
            class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast".into(), class.get()])) test_id=test_id
            attr:data-name="ToastRoot" attr:data-tone=move || tone.get().key()>
            {children()}
        </raw::Root>
    }
}

#[component]
pub fn Title(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <raw::Title class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast-title".into(), class.get()])) attr:data-name="ToastTitle">
            {children()}
        </raw::Title>
    }
}

#[component]
pub fn Description(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <raw::Description class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast-description".into(), class.get()])) attr:data-name="ToastMessage">
            {children()}
        </raw::Description>
    }
}

#[component]
pub fn Action(
    #[prop(into)] alt_text: String,
    #[prop(into)] on_action: Callback<rustify_ui::Cancelable<()>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <raw::Action alt_text=alt_text on_action=on_action
            class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast-action".into(), class.get()])) attr:data-name="ToastAction">
            {children()}
        </raw::Action>
    }
}

#[component]
pub fn Close(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <raw::Close class=Signal::derive(crate::macros::reactive_merge_with(move || vec!["rustify-toast-close".into(), class.get()])) test_id=test_id attr:data-name="ToastDismiss">
            {children.map(|children| children()).unwrap_or_else(|| view! { <Icon glyph=Glyph::Close/> }.into_any())}
        </raw::Close>
    }
}

/// One accepted handle message, using the same Root as declarative parts.
/// Map `use_toasts().items()` by `seq`; removing a message unmounts its Root.
#[component]
pub fn Toast(
    toast: rustify_ui::Toast,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let handle = raw::use_toasts();
    let seq = toast.seq;
    let duration_ms = toast.duration_ms;
    let importance = toast.importance;
    let tone = toast.tone;
    let dismiss_id = format!("{test_id}-dismiss");
    view! {
        <Root open=true on_open_change=move |open: bool| { if !open { handle.dismiss(seq); } }
            duration_ms=duration_ms importance=importance tone=tone class=class test_id=test_id>
            {body(toast, String::new(), dismiss_id)}
        </Root>
    }
}

fn body(toast: rustify_ui::Toast, test_id: String, dismiss_id: String) -> impl IntoView {
    view! {
        <div class="rustify-toast-body" data-name="Toast" data-tone=toast.tone.key() data-testid=(!test_id.is_empty()).then_some(test_id)>
            <div class="rustify-toast-content">
                {toast.title.map(|title| view! { <Title>{title}</Title> })}
                <Description>{toast.message}</Description>
                {toast.action.map(|action| view! { <Action alt_text=action.alt_text on_action=action.on_action>{action.label}</Action> })}
            </div>
            <Close test_id=dismiss_id/>
        </div>
    }
}

/// The scope's themed handle messages. Call `provide_toasts` in the scope root.
/// Each visible body keeps the previous `test_id-toast` and dismiss selectors.
#[component]
pub fn Toaster(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let card = format!("{test_id}-toast");
    let render = Callback::new(move |toast: rustify_ui::Toast| {
        body(toast, card.clone(), format!("{card}-dismiss")).into_any()
    });
    view! {
        <raw::ToastRegion render=render class=Signal::derive(crate::macros::reactive_merge("rustify-toaster", &class)) test_id=test_id/>
    }
}
