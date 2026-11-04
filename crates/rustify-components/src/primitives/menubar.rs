//! A controlled horizontal command bar over the shared menu tree.
pub use super::menu::{
    Arrow, CheckboxItem, Group, Item, ItemIndicator, Label, Portal, RadioGroup, RadioItem,
    Separator, Sub, SubContent, SubTrigger,
};
use super::{floating, menu};
use leptos::prelude::*;
use rustify_ui::{Cancelable, Direction, FloatingPosition, LayerDismiss};

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone)]
struct Entry {
    key: String,
    id: String,
    disabled: Signal<bool>,
    menu: menu::Context,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Bar {
    id: StoredValue<String>,
    value: Signal<Option<String>>,
    change: Callback<Option<String>>,
    current: RwSignal<Option<String>>,
    entries: RwSignal<Vec<Entry>>,
    direction: Signal<Direction>,
    #[cfg(not(target_arch = "wasm32"))]
    host_owner: StoredValue<leptos::reactive::owner::WeakOwner>,
    #[cfg(not(target_arch = "wasm32"))]
    host_children: StoredValue<Vec<Owner>>,
    looping: bool,
    #[cfg(target_arch = "wasm32")]
    pointer_on_trigger: StoredValue<bool>,
}
impl Bar {
    #[cfg(not(target_arch = "wasm32"))]
    fn retain_item(self) {
        // SSR consumes each child before serializing the remaining siblings.
        let root = self.host_owner.with_value(|owner| owner.upgrade());
        let mut owner = Owner::current();
        self.host_children.update_value(|children| {
            while let Some(current) = owner.take() {
                if root.as_ref() == Some(&current) {
                    break;
                }
                owner = current.parent();
                children.push(current);
            }
        });
    }

    fn stop(self) -> Option<String> {
        let entries = self.entries.get();
        let desired = self.current.get().or_else(|| self.value.get());
        entries
            .iter()
            .find(|entry| !entry.disabled.get() && desired.as_ref() == Some(&entry.key))
            .or_else(|| entries.iter().find(|entry| !entry.disabled.get()))
            .map(|entry| entry.key.clone())
    }
    fn request(self, key: &str, open: bool) {
        if open {
            if self.value.get_untracked().as_deref() != Some(key) {
                self.change.run(Some(key.into()));
            }
        } else if self.value.get_untracked().as_deref() == Some(key) {
            self.change.run(None);
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn move_to(self, key: &str, step: i32, edge: Option<bool>, open: bool) {
        let root = document().get_element_by_id(&self.id.get_value());
        let entries = self.entries.get_untracked();
        let mut entries: Vec<_> = entries
            .into_iter()
            .filter(|entry| !entry.disabled.get_untracked())
            .filter_map(|entry| {
                document()
                    .get_element_by_id(&entry.id)
                    .filter(|element| {
                        root.as_ref()
                            .is_some_and(|root| root.contains(Some(element)))
                    })
                    .map(|element| (entry, element))
            })
            .collect();
        entries.sort_by(|a, b| {
            let position = a.1.compare_document_position(&b.1);
            if position & web_sys::Node::DOCUMENT_POSITION_FOLLOWING != 0 {
                std::cmp::Ordering::Less
            } else if position & web_sys::Node::DOCUMENT_POSITION_PRECEDING != 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let here = entries.iter().position(|(entry, _)| entry.key == key);
        let next = match edge {
            Some(false) => (!entries.is_empty()).then_some(0),
            Some(true) => entries.len().checked_sub(1),
            None => super::choice::navigation(entries.len(), here, step, self.looping),
        };
        if let Some((entry, element)) = next.and_then(|index| entries.get(index)) {
            self.current.set(Some(entry.key.clone()));
            menu::browser::focus_element(element);
            if open {
                entry.menu.focus.set_value(menu::OpenFocus::First);
                self.request(&entry.key, true);
            }
        }
    }
}
#[derive(Clone, Copy)]
struct Member {
    bar: Bar,
    key: StoredValue<String>,
}
#[component]
pub fn Root(
    #[prop(into)] value: Signal<Option<String>>,
    #[prop(into)] on_value_change: Callback<Option<String>>,
    #[prop(default = true)] loop_focus: bool,
    #[prop(into)] aria_label: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(|| {
        let bar=Bar{id:StoredValue::new(crate::id::next("menubar")),value,change:on_value_change,current:RwSignal::new(None),entries:RwSignal::new(Vec::new()),direction:rustify_ui::use_direction(),looping:loop_focus,
            #[cfg(not(target_arch="wasm32"))] host_owner:StoredValue::new(owner.downgrade()),
            #[cfg(not(target_arch="wasm32"))] host_children:StoredValue::new(Vec::new()),
            #[cfg(target_arch="wasm32")] pointer_on_trigger:StoredValue::new(false)};
        #[cfg(target_arch="wasm32")] install_pointer_target(bar);
        provide_context(bar);
        let unnamed=aria_label.trim().is_empty();
        view!{<div id=bar.id.get_value() role="menubar" aria-label=aria_label aria-orientation="horizontal" dir=move ||if bar.direction.get()==Direction::RightToLeft{"rtl"}else{"ltr"}
            data-rustify-part="menubar-root" data-rustify-issue=unnamed.then_some("missing-name") class=class data-testid=test_id>{children()}</div>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
pub fn Menu(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let Some(bar) = use_context::<Bar>() else {
        return view!{<div data-rustify-part="menubar-menu" data-rustify-issue="missing-root">{children()}</div>}.into_any();
    };
    let key = StoredValue::new(value);
    menu::root(
        Signal::derive(move || bar.value.get().as_ref() == Some(&key.get_value())),
        Callback::new(move |open| bar.request(&key.get_value(), open)),
        false,
        Box::new(move || {
            provide_context(Member { bar, key });
            let context = expect_context::<menu::Context>();
            context.tree.boundary.set(Some(bar.id.get_value()));
            children()
        }),
    )
    .into_any()
}
#[component]
pub fn Trigger(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let Some(member) = use_context::<Member>() else {
        return view!{<button type="button" disabled class=class data-testid=test_id data-rustify-part="menubar-trigger" data-rustify-issue="missing-menu">{children()}</button>}.into_any();
    };
    let context = expect_context::<menu::Context>();
    let Member { bar, key } = member;
    let id = floating::register(context.floating.triggers, "menubar-trigger");
    context.floating.trigger_id.set(id.clone());
    #[cfg(not(target_arch = "wasm32"))]
    bar.retain_item();
    bar.entries.update(|entries| {
        entries.push(Entry {
            key: key.get_value(),
            id: id.clone(),
            disabled,
            menu: context,
        })
    });
    let cleanup = id.clone();
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        bar.entries
            .try_update(|entries| entries.retain(|entry| entry.id != cleanup));
    });
    #[cfg(not(target_arch = "wasm32"))]
    let _ = cleanup;
    let unavailable = move || {
        disabled.get()
            || context.floating.triggers.get().len() != 1
            || context.floating.portals.get() != 1
    };
    let open = move |focus| {
        context.focus.set_value(focus);
        if context.floating.open.get_untracked() {
            #[cfg(target_arch = "wasm32")]
            menu::browser::focus(context);
        } else {
            bar.request(&key.get_value(), true);
        }
    };
    view!{<button type="button" id=id class=class data-testid=test_id role="menuitem" disabled=move ||disabled.get()
        data-rustify-part="menubar-trigger" data-rustify-issue=move ||(context.floating.triggers.get().len()!=1||context.floating.portals.get()!=1).then_some("duplicate-or-missing-part")
        tabindex=move ||if !unavailable()&&bar.stop().as_ref()==Some(&key.get_value()){0}else{-1}
        aria-haspopup="menu" aria-expanded=move ||context.floating.open.get().to_string()
        aria-controls=move ||context.floating.open.get().then(||context.floating.panel_id.get_value())
        data-state=move ||if context.floating.open.get(){"open"}else{"closed"}
        on:focus=move |_|bar.current.set(Some(key.get_value()))
        on:pointerdown=move |event|{
            if event.default_prevented()||event.button()!=0||event.ctrl_key()||unavailable(){return;}
            if context.floating.open.get_untracked(){bar.request(&key.get_value(),false);}else{open(menu::OpenFocus::Content);event.prevent_default();}
        }
        on:pointerenter=move |event|{
            if event.pointer_type()=="mouse"&&!event.default_prevented()&&!unavailable()&&bar.value.get_untracked().is_some()&&!context.floating.open.get_untracked(){
                #[cfg(target_arch="wasm32")]
                if let Some(element)=document().get_element_by_id(&context.floating.trigger_id.get_untracked()){menu::browser::focus_element(&element);}
                open(menu::OpenFocus::Content);
            }
        }
        on:keydown=move |event|{
            if event.default_prevented()||event.is_composing()||event.ctrl_key()||event.alt_key()||event.meta_key()||unavailable(){return;}
            match event.key().as_str(){
                "ArrowDown"|"Enter"|" "=>{event.prevent_default();open(menu::OpenFocus::First);}
                "ArrowUp"=>{event.prevent_default();open(menu::OpenFocus::Last);}
                "ArrowLeft"|"ArrowRight"|"Home"|"End"=>{
                    event.prevent_default();
                    #[cfg(target_arch="wasm32")]{
                        let name=event.key();let forward=(name=="ArrowRight")!=(bar.direction.get_untracked()==Direction::RightToLeft);
                        bar.move_to(&key.get_value(),if forward{1}else{-1},match name.as_str(){"Home"=>Some(false),"End"=>Some(true),_=>None},false);
                    }
                }
                _=>{}
            }
        }>{children()}</button>}.into_any()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn content(
    loop_focus: bool,
    position: Signal<FloatingPosition>,
    aria_label: Signal<String>,
    mut hooks: floating::Hooks,
    class: String,
    test_id: String,
    children: ChildrenFn,
) -> AnyView {
    let member = use_context::<Member>();
    if let Some(member) = member {
        let user_pointer = hooks.pointer;
        hooks.pointer = Some(Callback::new(move |event: Cancelable<LayerDismiss>| {
            if let Some(hook) = user_pointer {
                hook.run(event.clone());
            }
            #[cfg(target_arch = "wasm32")]
            if member.bar.pointer_on_trigger.get_value() {
                event.prevent_default();
            }
        }));
        let user_focus = hooks.focus;
        hooks.focus = Some(Callback::new(move |event: Cancelable<LayerDismiss>| {
            if let Some(hook) = user_focus {
                hook.run(event.clone());
            }
            #[cfg(target_arch = "wasm32")]
            if document().active_element().is_some_and(|active| {
                member
                    .bar
                    .entries
                    .with_untracked(|entries| entries.iter().any(|entry| entry.id == active.id()))
            }) {
                event.prevent_default();
            }
        }));
        let user_close = hooks.close;
        hooks.close = Some(Callback::new(move |event: Cancelable<()>| {
            if let Some(hook) = user_close {
                hook.run(event.clone());
            }
            if member.bar.value.get_untracked().is_some() {
                event.prevent_default();
            }
        }));
    }
    let children = StoredValue::new(children);
    menu::content(
        loop_focus,
        position,
        aria_label,
        hooks,
        class,
        test_id,
        std::sync::Arc::new(move || {
            #[cfg(target_arch = "wasm32")]
            if let Some(member) = member {
                install_content(member, expect_context::<menu::Context>());
            }
            children.with_value(|children| children())
        }),
    )
}
#[component]
pub fn Content(
    #[prop(optional)] loop_focus: bool,
    #[prop(optional, into)] position: Signal<FloatingPosition>,
    #[prop(optional, into)] aria_label: Signal<String>,
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
        loop_focus,
        position,
        aria_label,
        floating::Hooks {
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
#[cfg(target_arch = "wasm32")]
fn install_content(member: Member, context: menu::Context) {
    use leptos::wasm_bindgen::JsCast;
    let held = StoredValue::new(None::<rustify_ui::Listener>);
    Effect::new(move || {
        let Some(node) = context.floating.panel.get() else {
            return;
        };
        let target: web_sys::EventTarget = node.into();
        held.set_value(Some(rustify_ui::listen(
            &target,
            "keydown",
            Default::default(),
            move |event| {
                let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() else {
                    return;
                };
                if event.default_prevented()
                    || event.is_composing()
                    || event.ctrl_key()
                    || event.alt_key()
                    || event.meta_key()
                {
                    return;
                }
                let name = event.key();
                if name != "ArrowLeft" && name != "ArrowRight" {
                    return;
                }
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                else {
                    return;
                };
                let forward = (name == "ArrowRight")
                    != (member.bar.direction.get_untracked() == Direction::RightToLeft);
                if forward
                    && target
                        .closest("[data-menu-sub-trigger]")
                        .ok()
                        .flatten()
                        .is_some()
                {
                    return;
                }
                let child = target
                    .closest("[role=menu]")
                    .ok()
                    .flatten()
                    .is_some_and(|panel| panel.id() != context.floating.panel_id.get_value());
                if child && !forward {
                    return;
                }
                event.prevent_default();
                member.bar.move_to(
                    &member.key.get_value(),
                    if forward { 1 } else { -1 },
                    None,
                    true,
                );
            },
        )));
    });
    on_cleanup(move || {
        held.try_update_value(|listener| {
            listener.take();
        });
    });
}

#[cfg(target_arch = "wasm32")]
fn install_pointer_target(bar: Bar) {
    use leptos::wasm_bindgen::JsCast;
    let held = StoredValue::new(None::<rustify_ui::Listener>);
    Effect::new(move || {
        let active = bar.value.get().is_some();
        held.update_value(|listener| {
            listener.take();
        });
        bar.pointer_on_trigger.set_value(false);
        if active {
            // Window capture precedes the layer's document capture dismissal.
            held.set_value(Some(rustify_ui::listen(
                &window().into(),
                "pointerdown",
                rustify_ui::ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    let target = event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok());
                    let inside = target.is_some_and(|target| {
                        bar.entries.with_untracked(|entries| {
                            entries.iter().any(|entry| {
                                document()
                                    .get_element_by_id(&entry.id)
                                    .is_some_and(|trigger| trigger.contains(Some(&target)))
                            })
                        })
                    });
                    bar.pointer_on_trigger.set_value(inside);
                },
            )));
        }
    });
    on_cleanup(move || {
        held.try_update_value(|listener| {
            listener.take();
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requests_switch_directly_and_a_stale_panel_cannot_close_the_new_menu() {
        Owner::new().with(|| {
            let value = RwSignal::new(Some("file".to_string()));
            let requests = RwSignal::new(Vec::<Option<String>>::new());
            let bar = Bar {
                id: StoredValue::new("bar".into()),
                value: value.into(),
                change: Callback::new(move |next: Option<String>| {
                    requests.update(|log| log.push(next.clone()));
                    value.set(next);
                }),
                current: RwSignal::new(None),
                entries: RwSignal::new(Vec::new()),
                #[cfg(not(target_arch = "wasm32"))]
                host_owner: StoredValue::new(Owner::current().unwrap().downgrade()),
                #[cfg(not(target_arch = "wasm32"))]
                host_children: StoredValue::new(Vec::new()),
                direction: Signal::stored(Direction::LeftToRight),
                looping: true,
                #[cfg(target_arch = "wasm32")]
                pointer_on_trigger: StoredValue::new(false),
            };
            bar.request("edit", true);
            bar.request("file", false);
            assert_eq!(requests.get_untracked(), vec![Some("edit".into())]);
            assert_eq!(value.get_untracked(), Some("edit".into()));
            bar.request("edit", true);
            bar.request("edit", false);
            assert_eq!(requests.get_untracked(), vec![Some("edit".into()), None]);
        });
    }
    #[test]
    fn refused_switch_leaves_the_accepted_menu_open() {
        Owner::new().with(|| {
            let requests = RwSignal::new(Vec::<Option<String>>::new());
            let bar = Bar {
                id: StoredValue::new("bar".into()),
                value: Signal::stored(Some("file".into())),
                change: Callback::new(move |next| requests.update(|log| log.push(next))),
                current: RwSignal::new(None),
                entries: RwSignal::new(Vec::new()),
                #[cfg(not(target_arch = "wasm32"))]
                host_owner: StoredValue::new(Owner::current().unwrap().downgrade()),
                #[cfg(not(target_arch = "wasm32"))]
                host_children: StoredValue::new(Vec::new()),
                direction: Signal::stored(Direction::LeftToRight),
                looping: true,
                #[cfg(target_arch = "wasm32")]
                pointer_on_trigger: StoredValue::new(false),
            };
            bar.request("edit", true);
            assert_eq!(requests.get_untracked(), vec![Some("edit".into())]);
            assert_eq!(bar.value.get_untracked(), Some("file".into()));
        });
    }
}
