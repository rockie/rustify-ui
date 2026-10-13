use leptos::prelude::*;
use rustify_components::{Button, Dialog, Menu, MenuItem};
use rustify_docs::site::RenderContext;
use rustify_ui::{use_resolved_theme, Anchor};

fn context_marker(slot: &'static str, context: &RenderContext) -> impl IntoView {
    let route = context.route.clone();
    let lang = context.lang.clone();
    let base = context.base.clone();
    view! {
        <span data-slot=slot data-context-route=route data-context-lang=lang data-context-base=base
            class="text-sm text-muted-foreground">
            {match slot {
                "header" => "Reading laboratory",
                "footer" => "Keep reading",
                "sidebar_top" => "Examples in this guide",
                "sidebar_bottom" => "Each example keeps its source nearby",
                "before_content" => "Try the example, then read its implementation",
                "after_content" => "Continue with the next page",
                _ => "This page is unavailable",
            }}
        </span>
    }
}

pub fn custom_page(context: RenderContext) -> impl IntoView {
    let first = context.demo("CounterDemo");
    let second = context.demo("CounterDemo");
    let theme = context.demo("ThemeProbeDemo");
    let counter = format!("{}counter/#three-independent-instances", context.base);
    let plain = format!("{}plain/#plain-details", context.base);
    view! {
        <div data-custom-page="" class="space-y-6">
            <p>"A custom page shares the same registered views and exact source as Markdown."</p>
            <p data-font-sans="" data-theme-surface="" class="font-sans rounded-md bg-card p-4 text-card-foreground">"Reader sample — résumé, 中文正文, 🙂"</p>
            <p data-font-serif="" class="font-serif">"A serif reading sample — 中文标题, 🙂"</p>
            <p data-font-mono="" class="font-mono">"let greeting = \"Hello, 中文, 🙂\";"</p>
            <p><a href="#custom-details">"Jump to details"</a>" · "<a href=counter>"Registered counter anchor"</a>" · "<a href=plain>"Static custom page"</a></p>
            <figure class="space-y-2">
                <img data-custom-image="" src="static/custom.svg" width="120" height="72"
                    alt="Three filled blue and green bars" />
                <figcaption class="text-sm text-muted-foreground">"Image example"</figcaption>
            </figure>
            <figure class="space-y-2">
                <div data-custom-background="" role="img" aria-label="Three filled blue and green bars as a background"
                    class="h-20 w-36 bg-[url(static/custom.svg)] bg-no-repeat" />
                <figcaption class="text-sm text-muted-foreground">"Background example"</figcaption>
            </figure>
            <h2 id="custom-details">"Two matching registered demos"</h2>
            {first}
            {second}
            <h2 id="custom-theme">"A shared theme"</h2>
            {theme}
        </div>
    }
}

pub fn plain_page(context: RenderContext) -> impl IntoView {
    let custom = format!("{}custom/#custom-details", context.base);
    view! {
        <div data-plain-page="">
            <p>"This custom page contains ordinary reading content."</p>
            <h2 id="plain-details">"Static details"</h2>
            <p><a href="#plain-details">"This section"</a>" · "<a href=custom>"Custom page anchor"</a></p>
            <p class="font-serif">"Readable without scripts — 中文, 🙂"</p>
        </div>
    }
}

pub fn header(context: RenderContext) -> impl IntoView {
    context_marker("header", &context)
}

pub fn footer(context: RenderContext) -> impl IntoView {
    let home = context.base.clone();
    view! { <p>{context_marker("footer", &context)}" · "<a href=home>"Home"</a></p> }
}

pub fn sidebar_top(context: RenderContext) -> impl IntoView {
    view! { <p>{context_marker("sidebar_top", &context)}</p> }
}

pub fn sidebar_bottom(context: RenderContext) -> impl IntoView {
    view! { <p>{context_marker("sidebar_bottom", &context)}</p> }
}

pub fn before_content(context: RenderContext) -> impl IntoView {
    let demo = (context.route == "/custom/").then(|| context.demo("CounterDemo"));
    let route = context.route;
    let lang = context.lang;
    let base = context.base;
    view! {
        <div data-slot="before_content" data-context-route=route data-context-lang=lang data-context-base=base>
            <p class="text-sm text-muted-foreground">"Try the example, then read its implementation"</p>
            {demo}
        </div>
    }
}

pub fn after_content(context: RenderContext) -> impl IntoView {
    view! { <p>{context_marker("after_content", &context)}</p> }
}

pub fn not_found(context: RenderContext) -> impl IntoView {
    let home = context.base.clone();
    view! {
        <div>{context_marker("not_found", &context)}
            <p>"Choose an example from the guide or "<a href=home>"return home"</a>"."</p>
        </div>
    }
}

pub fn theme_probe_demo() -> impl IntoView {
    let menu = RwSignal::new(false);
    let dialog = RwSignal::new(false);
    let future = RwSignal::new(false);
    let action = RwSignal::new(String::new());
    let menu_anchor = NodeRef::<leptos::html::Button>::new();
    #[cfg(target_arch = "wasm32")]
    let anchor = Signal::derive(move || {
        menu_anchor
            .get()
            .map(|node| Anchor::element(node.as_ref()))
            .unwrap_or(Anchor::Centred)
    });
    #[cfg(not(target_arch = "wasm32"))]
    let anchor = Signal::derive(|| Anchor::Centred);
    let theme = use_resolved_theme();
    view! {
        <div data-theme-probe="" class="space-y-3">
            <p data-testid="theme-probe-context"
                data-theme-document=move || theme.map(|theme| theme.get().document_id.clone()).unwrap_or_default()
                data-theme-mode=move || theme.map(|theme| theme.get().mode.as_str()).unwrap_or_default()
                data-theme-radius=move || theme.map(|theme| theme.get().radius_px.to_string()).unwrap_or_default()>
                "Theme sample — résumé, 中文, 🙂"
            </p>
            <div class="flex flex-wrap items-center gap-3">
                <button type="button" node_ref=menu_anchor data-testid="theme-probe-menu-trigger"
                    class="rounded-md border border-border bg-background px-3 py-2 text-foreground"
                    on:click=move |_| menu.set(true)>"Open themed menu"</button>
                <Button test_id="theme-probe-dialog-trigger" on_click=move || dialog.set(true)>"Open themed dialog"</Button>
                <Button test_id="theme-probe-reveal" on_click=move || future.set(true)>"Show a style sample"</Button>
            </div>
            <Menu test_id="theme-probe-menu" open=menu on_open_change=move |open| menu.set(open)
                anchor=anchor aria_label="Themed example commands"
                items=vec![MenuItem::new("choose", "Choose résumé 中文 🙂"), MenuItem::new("leave", "Leave unchanged")]
                on_activate=move |chosen| action.set(chosen) />
            <output data-testid="theme-probe-menu-value">{move || action.get()}</output>
            <Dialog test_id="theme-probe-dialog" open=dialog on_open_change=move |open| dialog.set(open)
                title="Theme probe dialog" description="The same reading theme reaches this floating panel.">
                <p>"A floating sample — résumé, 中文, 🙂"</p>
            </Dialog>
            <Show when=move || future.get() fallback=|| ()>
                <div data-testid="theme-probe-future" role="img" aria-label="Activated style sample"
                    class="w-[73px] h-[31px] bg-emerald-600 bg-[url(static/future.svg)] rounded-xl" />
            </Show>
        </div>
    }
}
