//! Image loading is presentation state; callers retain their source and content.
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
mod browser;
mod model;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImageLoadingStatus {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error,
}
impl ImageLoadingStatus {
    pub const ALL: &'static [Self] = &[Self::Idle, Self::Loading, Self::Loaded, Self::Error];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Loading => "loading",
            Self::Loaded => "loaded",
            Self::Error => "error",
        }
    }
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    status: RwSignal<ImageLoadingStatus>,
    generation: RwSignal<u64>,
}

#[component]
pub fn Root(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let context = Context { status: RwSignal::new(ImageLoadingStatus::Idle), generation: RwSignal::new(0) };
        provide_context(context);
        view! {<span class=crate::macros::reactive_merge("",&class) data-rustify-part="avatar-root" data-testid=test_id
            data-state=move ||context.status.get().as_str()>{children()}</span>}
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[component]
pub fn Image(
    #[prop(into)] src: Signal<String>,
    #[prop(into)] alt: Signal<String>,
    #[prop(optional_no_strip, into)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let context = use_context::<Context>();
    let request = RwSignal::new(model::Request::new(src.get_untracked()));
    #[cfg(target_arch = "wasm32")]
    {
        browser::source(src, request, context, on_loading_status_change);
        on_cleanup(move || {
            if let Some(context) = context {
                let _ = context.status.try_set(ImageLoadingStatus::Idle);
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = on_loading_status_change;
    let class = StoredValue::new(class);
    let test_id = StoredValue::new(test_id);
    view! {<leptos::attribute_interceptor::AttributeInterceptor let:attributes>
        {let attributes=StoredValue::new_local(attributes);
        view! {<For each=move ||request.with(|request|vec![(request.generation,request.source.clone())]) key=|(generation,_)|*generation let:entry>
            {let node=NodeRef::<leptos::html::Img>::new();
            #[cfg(target_arch = "wasm32")]
            browser::image(node,request,context,on_loading_status_change,entry.0);
            view! {<img node_ref=node src=(!entry.1.is_empty()).then_some(entry.1) alt=move ||alt.get()
                class=crate::macros::reactive_merge("",&class.get_value()) data-testid=test_id.get_value() data-rustify-part="avatar-image"
                data-state=move ||request.with(|request|request.status.as_str())
                aria-hidden=move ||request.with(|request|(context.is_some() && request.status!=ImageLoadingStatus::Loaded).then_some("true"))
                inert=move ||request.with(|request|(context.is_some() && request.status!=ImageLoadingStatus::Loaded).then_some(""))
                data-rustify-issue=context.is_none().then_some("missing-root") {..attributes.get_value()} />}}
        </For>}}
    </leptos::attribute_interceptor::AttributeInterceptor>}
}

#[component]
pub fn Fallback(
    #[prop(optional)] delay_ms: u32,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<Context>();
    let ready = RwSignal::new(true);
    #[cfg(target_arch = "wasm32")]
    if let Some(context) = context {
        browser::fallback(context, ready, delay_ms);
    }
    let visible = Signal::derive(move || {
        ready.get()
            && context.is_none_or(|context| context.status.get() != ImageLoadingStatus::Loaded)
    });
    view! {<span class=crate::macros::reactive_merge("",&class) data-rustify-part="avatar-fallback" data-testid=test_id
        data-visible=move ||visible.get().to_string() aria-hidden=move ||(!visible.get()).then_some("true")
        inert=move ||(!visible.get()).then_some("")
        data-rustify-issue={if context.is_none(){Some("missing-root")}else if delay_ms>i32::MAX as u32 {Some("delay_ms")}else{None}}>
        {children()}
    </span>}
}
