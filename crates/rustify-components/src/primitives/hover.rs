//! Temporary hover intent; accepted visibility remains owned by the application.
use super::floating::{Context, Kind};
use leptos::prelude::*;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(super) struct ProviderContext {
    pub delay: u32,
    pub skip: u32,
    pub disable_hoverable: bool,
    active: RwSignal<u32>,
    until: StoredValue<f64>,
}
impl ProviderContext {
    pub fn new(delay: u32, skip: u32, disable_hoverable: bool) -> Self {
        Self {
            delay,
            skip,
            disable_hoverable,
            active: RwSignal::new(0),
            until: StoredValue::new(0.),
        }
    }
}
#[derive(Clone, Copy)]
pub(super) struct Options {
    pub open_delay: u32,
    pub close_delay: u32,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct State {
    context: Context,
    options: Options,
    provider: Option<ProviderContext>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<browser::Timer>, leptos::reactive::owner::LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    pending: RwSignal<bool>,
    #[cfg(target_arch = "wasm32")]
    transit: StoredValue<Option<rustify_ui::Listener>, leptos::reactive::owner::LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    blocked: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    pressed: StoredValue<bool>,
    #[cfg(target_arch = "wasm32")]
    inside: StoredValue<[bool; 2]>,
}
pub(super) fn install(context: Context) {
    let provider = (context.kind == Kind::Tooltip)
        .then(use_context::<ProviderContext>)
        .flatten();
    let options = use_context::<Options>().unwrap_or(Options {
        open_delay: 700,
        close_delay: if context.kind == Kind::Tooltip {
            0
        } else {
            300
        },
    });
    let state = State {
        context,
        options,
        provider,
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        pending: RwSignal::new(false),
        #[cfg(target_arch = "wasm32")]
        transit: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        blocked: StoredValue::new(false),
        #[cfg(target_arch = "wasm32")]
        pressed: StoredValue::new(false),
        #[cfg(target_arch = "wasm32")]
        inside: StoredValue::new([false; 2]),
    };
    provide_context(state);
    if options.open_delay > i32::MAX as u32
        || options.close_delay > i32::MAX as u32
        || provider.is_some_and(|provider| {
            provider.delay > i32::MAX as u32 || provider.skip > i32::MAX as u32
        })
    {
        context.interaction_issue.set(Some("invalid-delay"));
    }
    #[cfg(target_arch = "wasm32")]
    browser::install(state);
}
pub(super) fn root(
    open: Signal<bool>,
    change: Callback<bool>,
    kind: Kind,
    options: Options,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        provide_context(options);
        super::floating::root(open, change, false, kind, children)
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
pub(super) fn trigger_as<E, At, Ch>(target: leptos::html::HtmlElement<E, At, Ch>) -> AnyView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let Some(context) = use_context::<Context>().filter(|context| context.kind.hover()) else {
        return super::Slot(
            target,
            (
                data("data-rustify-issue", "missing-root"),
                data("data-rustify-part", "hover-trigger"),
            ),
        )
        .into_any();
    };
    let id = super::floating::register(context.triggers, "hover-trigger");
    context.trigger_id.set(id.clone());
    let attrs = (
        leptos::attr::id(id),
        data("data-rustify-part", context.part("trigger")),
        data("data-state", move || {
            if context.open.get() {
                "open"
            } else {
                "closed"
            }
        }),
        data("data-rustify-issue", move || {
            (context.triggers.get() != [context.trigger_id.get()] || context.portals.get() != 1)
                .then_some("duplicate-or-missing-part")
        }),
    );
    #[cfg(target_arch = "wasm32")]
    {
        browser::trigger(
            target,
            attrs,
            use_context::<State>().expect("hover Root provides intent state"),
        )
        .into_any()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        super::Slot(target, attrs).into_any()
    }
}
pub(super) fn dismiss_handler() -> Callback<()> {
    let state = use_context::<State>();
    Callback::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        if let Some(state) = state {
            state.blocked.set_value(true);
            state.cancel();
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = state;
    })
}

#[cfg(target_arch = "wasm32")]
mod browser;
