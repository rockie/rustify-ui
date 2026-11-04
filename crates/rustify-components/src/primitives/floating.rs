//! Controlled lifecycle shared by dialog and anchored floating parts.
use leptos::prelude::*;
use rustify_ui::{Cancelable, LayerDismiss, ViewportAlign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Dialog,
    AlertDialog,
    Popover,
    HoverCard,
    Tooltip,
    Menu,
}
impl Kind {
    pub(super) fn hover(self) -> bool {
        matches!(self, Self::HoverCard | Self::Tooltip)
    }
    fn anchored(self) -> bool {
        matches!(self, Self::Popover | Self::Menu) || self.hover()
    }
    fn name(self) -> &'static str {
        match self {
            Self::Dialog => "dialog",
            Self::AlertDialog => "alert-dialog",
            Self::Popover => "popover",
            Self::HoverCard => "hover-card",
            Self::Tooltip => "tooltip",
            Self::Menu => "menu",
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(crate) struct Hooks {
    pub escape: Option<Callback<Cancelable<LayerDismiss>>>,
    pub pointer: Option<Callback<Cancelable<LayerDismiss>>>,
    pub focus: Option<Callback<Cancelable<LayerDismiss>>>,
    pub interact: Option<Callback<Cancelable<LayerDismiss>>>,
    pub open: Option<Callback<Cancelable<()>>>,
    pub close: Option<Callback<Cancelable<()>>>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(super) struct Context {
    key: StoredValue<String>,
    pub(super) kind: Kind,
    pub(super) interaction_issue: RwSignal<Option<&'static str>>,
    pub(super) open: Signal<bool>,
    pub(super) change: Callback<bool>,
    pub(super) modal: bool,
    align: RwSignal<ViewportAlign>,
    position: RwSignal<rustify_ui::FloatingPosition>,
    anchor: NodeRef<leptos::html::Div>,
    #[cfg(target_arch = "wasm32")]
    pub(super) virtual_anchor: RwSignal<Option<rustify_ui::Anchor>>,
    anchors: RwSignal<u32>,
    arrow: NodeRef<leptos::html::Div>,
    arrows: RwSignal<u32>,
    pub(super) panel: NodeRef<leptos::html::Div>,
    pub(super) panel_id: StoredValue<String>,
    pub(super) trigger_id: RwSignal<String>,
    pub(super) triggers: RwSignal<Vec<String>>,
    pub(super) portals: RwSignal<u32>,
    contents: RwSignal<u32>,
    overlays: RwSignal<u32>,
    titles: RwSignal<Vec<String>>,
    descriptions: RwSignal<Vec<String>>,
    cancels: RwSignal<Vec<String>>,
    named: RwSignal<bool>,
    described: RwSignal<bool>,
    hooks: RwSignal<Hooks>,
}
impl Context {
    pub(super) fn part(self, suffix: &str) -> String {
        format!("{}-{suffix}", self.kind.name())
    }
    fn duplicate(self) -> bool {
        self.arrows.get() > 1
            || self.anchors.get() > 1
            || self.triggers.get().len() > 1
            || self.portals.get() > 1
            || self.contents.get() > 1
            || self.overlays.get() > 1
            || self.titles.get().len() > 1
            || self.descriptions.get().len() > 1
            || self.cancels.get().len() > 1
    }
    fn has_anchor(self) -> bool {
        #[cfg(target_arch = "wasm32")]
        if self.virtual_anchor.get().is_some() {
            return true;
        }
        self.anchors.get() > 0 || !self.triggers.get().is_empty()
    }
    fn issue(self) -> Option<&'static str> {
        if self.duplicate() {
            Some("duplicate-part")
        } else if self.kind.anchored() && !self.has_anchor() {
            Some("missing-anchor")
        } else if self.portals.get() == 0 {
            Some("missing-portal")
        } else if self.open.get() && self.contents.get() == 0 {
            Some("missing-content")
        } else {
            None
        }
    }
    fn content_issue(self) -> Option<&'static str> {
        self.issue()
            .or_else(|| self.interaction_issue.get())
            .or_else(|| {
                if self.kind.hover() {
                    return None;
                }
                if self.kind.anchored() {
                    return (!self.named.get()).then_some("missing-name");
                }
                if self.titles.get().is_empty() {
                    Some("missing-title")
                } else if !self.named.get() {
                    Some("empty-title")
                } else if self.kind == Kind::AlertDialog
                    && (self.descriptions.get().is_empty() || !self.described.get())
                {
                    Some("missing-description")
                } else if self.kind == Kind::AlertDialog && self.cancels.get().is_empty() {
                    Some("missing-cancel")
                } else {
                    None
                }
            })
    }
}
#[derive(Clone, Copy)]
struct PortalContext(StoredValue<String>);
#[derive(Clone, Copy)]
struct ContentContext(StoredValue<String>);

pub(crate) fn root(
    open: Signal<bool>,
    change: Callback<bool>,
    modal: bool,
    kind: Kind,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
        let context=Context {
            #[cfg(target_arch="wasm32")] virtual_anchor:RwSignal::new(None),
            interaction_issue:RwSignal::new(None),key:StoredValue::new(crate::id::next(kind.name())),kind,open,change,modal,align:RwSignal::new(ViewportAlign::Center),position:RwSignal::new(rustify_ui::FloatingPosition::default()),anchor:NodeRef::new(),anchors:RwSignal::new(0),arrow:NodeRef::new(),arrows:RwSignal::new(0),
            panel:NodeRef::new(),panel_id:StoredValue::new(crate::id::next("floating-content")),
            trigger_id:RwSignal::new(String::new()),triggers:RwSignal::new(Vec::new()),portals:RwSignal::new(0),contents:RwSignal::new(0),overlays:RwSignal::new(0),
            titles:RwSignal::new(Vec::new()),descriptions:RwSignal::new(Vec::new()),cancels:RwSignal::new(Vec::new()),
            named:RwSignal::new(true),described:RwSignal::new(true),hooks:RwSignal::new(Hooks::default())};
        provide_context(context);
        if kind.hover() {super::hover::install(context);}
        Effect::new(move || {
            let ids=context.triggers.get();
            if ids.len()==1 && context.trigger_id.get_untracked()!=ids[0] {context.trigger_id.set(ids[0].clone());}
        });
        let children=children();
        view! {<div data-rustify-part=context.part("root") data-state=move ||if open.get(){"open"}else{"closed"}
            data-rustify-issue=move ||context.issue()>{children}</div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
fn count(value: RwSignal<u32>) {
    value.update(|n| *n += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        value.try_update(|n| *n -= 1);
    });
}
pub(super) fn register(ids: RwSignal<Vec<String>>, prefix: &str) -> String {
    let id = crate::id::next(prefix);
    ids.update(|ids| ids.push(id.clone()));
    #[cfg(target_arch = "wasm32")]
    {
        let id = id.clone();
        on_cleanup(move || {
            ids.try_update(|ids| ids.retain(|value| value != &id));
        });
    }
    id
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ButtonAction {
    Trigger,
    Close,
    Cancel,
    Action,
}
fn button_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    action: ButtonAction,
    on_action: Option<Callback<Cancelable<()>>>,
) -> AnyView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    let suffix = match action {
        ButtonAction::Trigger => "trigger",
        ButtonAction::Close => "close",
        ButtonAction::Cancel => "cancel",
        ButtonAction::Action => "action",
    };
    let Some(context) = use_context::<Context>() else {
        return super::Slot(
            target,
            (
                attr::disabled(true),
                data("data-rustify-part", format!("dialog-{suffix}")),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    let id = if action == ButtonAction::Trigger {
        let id = register(context.triggers, "floating-trigger");
        context.trigger_id.set(id.clone());
        Some(id)
    } else if action == ButtonAction::Cancel {
        Some(register(context.cancels, "floating-cancel"))
    } else {
        None
    };
    let invalid = move || {
        action == ButtonAction::Trigger
            && (context.triggers.get().len() != 1 || context.portals.get() != 1)
    };
    let target = target.on(leptos::ev::click, move |event| {
        if event.default_prevented() || invalid() {
            return;
        }
        if action == ButtonAction::Action {
            let action = Cancelable::new(());
            if let Some(callback) = on_action {
                callback.run(action.clone());
            }
            if action.default_prevented() {
                return;
            }
        }
        context
            .change
            .run(action == ButtonAction::Trigger && !context.open.get_untracked());
    });
    super::Slot(
        target,
        (
            attr::r#type("button"),
            super::tabs::optional_attribute(id.map(attr::id)),
            data("data-rustify-part", context.part(suffix)),
            data("data-rustify-issue", move || {
                invalid().then_some("duplicate-or-missing-part")
            }),
            data("aria-disabled", move || invalid().then_some("true")),
            data(
                "aria-haspopup",
                (action == ButtonAction::Trigger).then_some("dialog"),
            ),
            data("aria-expanded", move || {
                (action == ButtonAction::Trigger).then(|| context.open.get().to_string())
            }),
            data(
                "aria-controls",
                (action == ButtonAction::Trigger).then(|| context.panel_id.get_value()),
            ),
            data("data-state", move || {
                if context.open.get() {
                    "open"
                } else {
                    "closed"
                }
            }),
        ),
    )
    .into_any()
}
pub fn trigger_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    button_as(target, ButtonAction::Trigger, None)
}
pub fn close_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    button_as(target, ButtonAction::Close, None)
}
pub fn cancel_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    button_as(target, ButtonAction::Cancel, None)
}
pub fn action_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    on_action: Callback<Cancelable<()>>,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    button_as(target, ButtonAction::Action, Some(on_action))
}

macro_rules! button {
    ($name:ident,$action:ident) => {
        #[component]
        pub fn $name(
            #[prop(optional, into)] class: String,
            #[prop(optional, into)] test_id: String,
            children: Children,
        ) -> impl IntoView {
            button_as(
                leptos::html::button()
                    .class(class)
                    .attr("data-testid", test_id)
                    .child(children()),
                ButtonAction::$action,
                None,
            )
        }
    };
}
button!(Trigger, Trigger);
button!(Close, Close);
button!(Cancel, Cancel);
#[component]
pub fn Action(
    #[prop(into)] on_action: Callback<Cancelable<()>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    button_as(
        leptos::html::button()
            .class(class)
            .attr("data-testid", test_id)
            .child(children()),
        ButtonAction::Action,
        Some(on_action),
    )
}

#[component]
pub fn Portal(
    #[prop(optional)] force_mount: bool,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view!{<div data-rustify-part="dialog-portal" data-rustify-issue="missing-root">{children()}</div>}.into_any();
    };
    let owner = Owner::new();
    let view=owner.with(move || {
        count(context.portals);provide_context(PortalContext(context.key));
        let children=StoredValue::new(children);
        let active=Signal::derive(move || context.open.get() && !context.duplicate() && context.portals.get()==1
            && (!context.kind.anchored() || context.has_anchor()));
        let anchor=Signal::derive(move || {
            if context.kind.anchored() {
                #[cfg(target_arch="wasm32")] {
                    if let Some(anchor)=context.virtual_anchor.get() {return anchor;}
                    if let Some(anchor)=context.anchor.get() {return rustify_ui::Anchor::element(&anchor.into());}
                    if let Some(trigger)=document().get_element_by_id(&context.trigger_id.get()) {return rustify_ui::Anchor::element(&trigger);}
                }
                rustify_ui::Anchor::Centred
            } else {rustify_ui::Anchor::Viewport {align:context.align.get()}}
        });
        let position=(context.kind.anchored()).then(||Signal::derive(move ||context.position.get()));
        let hover_dismiss=super::hover::dismiss_handler();
        let escape=Callback::new(move |event:Cancelable<LayerDismiss>| {if let Some(hook)=context.hooks.get_untracked().escape {hook.run(event.clone());}
            if context.kind.hover() && !event.default_prevented(){hover_dismiss.run(());}});
        let pointer=Callback::new(move |event:Cancelable<LayerDismiss>| {
            if let Some(hook)=context.hooks.get_untracked().pointer {hook.run(event.clone());}
            if context.kind.hover() {hover_dismiss.run(());}
            if context.kind==Kind::AlertDialog {event.prevent_default();}
        });
        let focus=Callback::new(move |event:Cancelable<LayerDismiss>| {
            if let Some(hook)=context.hooks.get_untracked().focus {hook.run(event.clone());}
            if context.kind.hover() {hover_dismiss.run(());}
            if context.modal {event.prevent_default();}
        });
        let interact=Callback::new(move |event:Cancelable<LayerDismiss>| {if let Some(hook)=context.hooks.get_untracked().interact {hook.run(event);}});
        let open_focus=Callback::new(move |event:Cancelable<()>| {
            if let Some(hook)=context.hooks.get_untracked().open {hook.run(event.clone());}
            if context.kind.hover() {event.prevent_default();}
            #[cfg(target_arch="wasm32")] {
                if event.default_prevented() || !context.open.get_untracked() {return;}
                event.prevent_default();
                if context.kind==Kind::AlertDialog {
                    use leptos::wasm_bindgen::JsCast;
                    let cancel=context.cancels.get_untracked().into_iter().filter_map(|id|document().get_element_by_id(&id))
                        .find(|element|!element.matches(":disabled, [hidden], [inert]").unwrap_or(false) && element.get_client_rects().length()>0);
                    if let Some(cancel)=cancel.and_then(|element|element.dyn_into::<web_sys::HtmlElement>().ok()) {let _=cancel.focus();}
                    else if let Some(panel)=context.panel.get_untracked() {let element:web_sys::HtmlElement=panel.into();let _=element.focus();}
                } else if let Some(panel)=context.panel.get_untracked() {
                    rustify_ui::overlay::focus_first(&panel.into());
                }
            }
        });
        let close_focus=Callback::new(move |event:Cancelable<()>| {if let Some(hook)=context.hooks.get_untracked().close {hook.run(event.clone());}
            if context.kind.hover(){event.prevent_default();}});
        view! {<Show when=move ||force_mount ||context.open.get()>
            <rustify_ui::Layer active=active modal=context.modal anchor=anchor panel=Some(context.panel) arrow=Some(context.arrow) position=position
                class=class.clone() test_id=test_id.clone()
                trigger_id=Signal::derive(move ||context.trigger_id.get()) on_close=move ||context.change.run(false)
                on_escape_key_down=escape on_pointer_down_outside=pointer on_focus_outside=focus on_interact_outside=interact
                on_open_auto_focus=open_focus on_close_auto_focus=close_focus>
                {move ||children.with_value(|children|children())}
            </rustify_ui::Layer>
        </Show>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}

#[component]
pub fn Overlay(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    if context.is_some_and(|context| !context.modal) {
        return ().into_any();
    }
    if let Some(context) = context {
        count(context.overlays);
    }
    view! {<div class=class data-testid=test_id data-rustify-part=context.map_or("dialog-overlay".into(),|context|context.part("overlay"))
    data-rustify-issue=if context.is_none(){Some("missing-root")}else if !valid_portal(context){Some("missing-portal")}else{None}
    data-state=move ||if context.is_some_and(|context|context.open.get()){"open"}else{"closed"}
    aria-hidden="true" />}.into_any()
}
fn valid_portal(context: Option<Context>) -> bool {
    context.is_some_and(|context| {
        use_context::<PortalContext>().is_some_and(|portal| portal.0 == context.key)
    })
}
#[component]
pub fn Content(
    #[prop(optional, into)] align: Signal<ViewportAlign>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pointer_down_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_focus_outside: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_interact_outside: Option<
        Callback<Cancelable<LayerDismiss>>,
    >,
    #[prop(optional_no_strip, into)] on_open_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional_no_strip, into)] on_close_auto_focus: Option<Callback<Cancelable<()>>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    content(
        ContentConfig {
            align,
            position: None,
            aria_label: None,
        },
        Hooks {
            escape: on_escape_key_down,
            pointer: on_pointer_down_outside,
            focus: on_focus_outside,
            interact: on_interact_outside,
            open: on_open_auto_focus,
            close: on_close_auto_focus,
        },
        class,
        test_id,
        children,
    )
}
#[derive(Clone, Copy)]
pub(crate) struct ContentConfig {
    pub align: Signal<ViewportAlign>,
    pub position: Option<Signal<rustify_ui::FloatingPosition>>,
    pub aria_label: Option<Signal<String>>,
}
pub(crate) fn content(
    config: ContentConfig,
    hooks: Hooks,
    class: String,
    test_id: String,
    children: ChildrenFn,
) -> AnyView {
    let Some(context) = use_context::<Context>() else {
        return view!{<div data-rustify-part="dialog-content" data-rustify-issue="missing-root" class=class>{children()}</div>}.into_any();
    };
    let portal = valid_portal(Some(context));
    let owner = Owner::new();
    let view=owner.with(move || {
        count(context.contents);provide_context(ContentContext(context.key));
        if context.kind.anchored() {context.named.set((context.kind==Kind::Menu && !context.trigger_id.get_untracked().is_empty()) || config.aria_label.is_some_and(|label|!label.get_untracked().trim().is_empty()));}
        let panel=NodeRef::<leptos::html::Div>::new();
        Effect::new(move || {
            if context.contents.get()!=1 {return;}
            context.align.set(config.align.get());
            if let Some(position)=config.position {context.position.set(position.get());}
            context.hooks.set(hooks);
            #[cfg(target_arch="wasm32")]
            if let Some(panel)=panel.get() {
                use leptos::tachys::html::node_ref::NodeRefContainer;
                let element:web_sys::Element=panel.into();
                context.panel.load(&element);
            }
        });
        #[cfg(target_arch="wasm32")]
        if portal {rustify_ui::overlay::on_layer_content_cleanup();}

        #[cfg(target_arch="wasm32")]
        browser::observe_names(context);
        let labelled=leptos::attr::aria_labelledby(move || {
            if context.kind==Kind::Menu {return (!config.aria_label.is_some_and(|label|!label.get().trim().is_empty()) && !context.trigger_id.get().is_empty()).then(||context.trigger_id.get());}
            (!context.kind.anchored() && context.titles.get().len()==1 && context.named.get()).then(||context.titles.get()[0].clone())
        });
        view!{<div node_ref=panel id=context.panel_id.get_value() class=class data-testid=test_id
            data-rustify-part=context.part("content") role=match context.kind {Kind::Menu=>Some("menu"),Kind::AlertDialog=>Some("alertdialog"),Kind::Tooltip=>Some("tooltip"),Kind::HoverCard=>None,_=>Some("dialog")} tabindex="-1"
            aria-orientation=(context.kind==Kind::Menu).then_some("vertical")
            data-state=move ||if context.open.get(){"open"}else{"closed"}
            data-rustify-issue=move ||if !portal{Some("missing-portal")}else{context.content_issue()}
            aria-label=move ||config.aria_label.map(|label|label.get()).filter(|label|!label.trim().is_empty())
            aria-describedby=move ||(context.descriptions.get().len()==1 && context.described.get()).then(||context.descriptions.get()[0].clone())
            hidden=move ||portal && !context.open.get() inert=move ||portal && !context.open.get()>{children()}</div>}.add_any_attr(labelled)
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}

fn text_part(
    title: bool,
    level: crate::HeadingLevel,
    class: String,
    test_id: String,
    children: Children,
) -> AnyView {
    let context = use_context::<Context>();
    let valid = context.is_some_and(|context| {
        use_context::<ContentContext>().is_some_and(|content| content.0 == context.key)
    });
    let id = context.map(|context| {
        register(
            if title {
                context.titles
            } else {
                context.descriptions
            },
            if title {
                "floating-title"
            } else {
                "floating-description"
            },
        )
    });
    let tag = if !title {
        "p"
    } else {
        match level {
            crate::HeadingLevel::H1 => "h1",
            crate::HeadingLevel::H2 => "h2",
            crate::HeadingLevel::H3 => "h3",
            crate::HeadingLevel::H4 => "h4",
            crate::HeadingLevel::H5 => "h5",
            crate::HeadingLevel::H6 => "h6",
        }
    };
    leptos::html::custom(tag)
        .id(id)
        .class(class)
        .attr("data-testid", test_id)
        .attr(
            "data-rustify-part",
            context.map_or_else(
                || {
                    if title {
                        "dialog-title".into()
                    } else {
                        "dialog-description".into()
                    }
                },
                |context| context.part(if title { "title" } else { "description" }),
            ),
        )
        .attr(
            "data-rustify-issue",
            if context.is_none() {
                Some("missing-root")
            } else if !valid {
                Some("missing-content")
            } else {
                None
            },
        )
        .child(children())
        .into_any()
}
#[component]
pub fn Title(
    #[prop(default=crate::HeadingLevel::H2)] level: crate::HeadingLevel,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    text_part(true, level, class, test_id, children)
}
#[component]
pub fn Description(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    text_part(false, crate::HeadingLevel::H2, class, test_id, children)
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use leptos::wasm_bindgen::{closure::Closure, JsCast};
    struct Names {
        observer: web_sys::MutationObserver,
        _callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
        _release: rustify_ui::AbortRelease,
    }
    impl Drop for Names {
        fn drop(&mut self) {
            self.observer.disconnect();
        }
    }
    pub(super) fn observe_names(context: Context) {
        let held = StoredValue::new_local(None::<Names>);
        Effect::new(move || {
            let titles = context.titles.get();
            let descriptions = context.descriptions.get();
            let trigger = (context.kind == Kind::Menu).then(|| context.trigger_id.get());
            let Some(panel) = context.panel.get() else {
                return;
            };
            let title = (titles.len() == 1)
                .then(|| document().get_element_by_id(&titles[0]))
                .flatten();
            let description = (descriptions.len() == 1)
                .then(|| document().get_element_by_id(&descriptions[0]))
                .flatten();
            let named_panel = panel.clone();
            let update = move || {
                if context.kind.anchored() {
                    let named = named_panel
                        .get_attribute("aria-label")
                        .is_some_and(|name| !name.trim().is_empty())
                        || named_panel
                            .get_attribute("aria-labelledby")
                            .is_some_and(|ids| {
                                ids.split_whitespace().any(|id| {
                                    document().get_element_by_id(id).is_some_and(|node| {
                                        node.text_content()
                                            .is_some_and(|text| !text.trim().is_empty())
                                    })
                                })
                            });
                    if context.named.try_get_untracked() != Some(named) {
                        let _ = context.named.try_set(named);
                    }
                    return;
                }
                for (node, signal) in [(&title, context.named), (&description, context.described)] {
                    let named = node.as_ref().is_some_and(|node| {
                        node.is_connected()
                            && node
                                .text_content()
                                .is_some_and(|text| !text.trim().is_empty())
                    });
                    if signal.try_get_untracked() != Some(named) {
                        let _ = signal.try_set(named);
                    }
                }
            };
            update();
            let callback =
                Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| {
                    update()
                });
            let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref())
            else {
                return;
            };
            let options = web_sys::MutationObserverInit::new();
            options.set_child_list(true);
            options.set_character_data(true);
            options.set_subtree(true);
            options.set_attributes(true);
            options.set_attribute_filter(&js_sys::Array::of2(
                &"aria-label".into(),
                &"aria-labelledby".into(),
            ));
            if observer.observe_with_options(&panel, &options).is_err() {
                return;
            }
            if let Some(trigger) = trigger.and_then(|id| document().get_element_by_id(&id)) {
                let _ = observer.observe_with_options(&trigger, &options);
            }
            let disconnect = js_sys::Reflect::get(&observer, &"disconnect".into())
                .ok()
                .and_then(|value| value.dyn_into::<js_sys::Function>().ok());
            let Some(disconnect) = disconnect else {
                return;
            };
            let release = rustify_ui::release_on_abort(disconnect.bind0(&observer));
            held.set_value(Some(Names {
                observer,
                _callback: callback,
                _release: release,
            }));
        });
        on_cleanup(move || {
            held.try_update_value(|held| {
                held.take();
            });
        });
    }
}

#[component]
pub fn Anchor(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(context) = use_context::<Context>() else {
        return view!{<div class=class data-testid=test_id data-rustify-part="popover-anchor" data-rustify-issue="missing-root">{children()}</div>}.into_any();
    };
    let owner = Owner::new();
    let view=owner.with(move || {
        count(context.anchors);let node=NodeRef::<leptos::html::Div>::new();
        #[cfg(target_arch="wasm32")]
        Effect::new(move || {
            use leptos::tachys::html::node_ref::NodeRefContainer;
            if context.anchors.get()==1 {if let Some(node)=node.get() {let element:web_sys::Element=node.into();context.anchor.load(&element);}}
        });
        view!{<div node_ref=node class=class data-testid=test_id data-rustify-part=context.part("anchor") data-rustify-issue=move ||(context.anchors.get()>1).then_some("duplicate-part")>{children()}</div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner).into_any()
}

#[component]
pub fn Arrow(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    let node = NodeRef::<leptos::html::Div>::new();
    if let Some(context) = context {
        count(context.arrows);
        #[cfg(target_arch = "wasm32")]
        Effect::new(move || {
            use leptos::tachys::html::node_ref::NodeRefContainer;
            if context.arrows.get() == 1 {
                if let Some(node) = node.get() {
                    let element: web_sys::Element = node.into();
                    context.arrow.load(&element);
                }
            }
        });
    }
    let valid = context.is_some_and(|context| {
        use_context::<ContentContext>().is_some_and(|content| content.0 == context.key)
    });
    view! {<div node_ref=node class=class data-testid=test_id data-rustify-floating-arrow="" aria-hidden="true"
    data-rustify-part=context.map_or("popover-arrow".into(),|context|context.part("arrow"))
    data-rustify-issue=if context.is_none(){Some("missing-root")}else if !valid{Some("missing-content")}else{None} />}
}
