use leptos::prelude::*;
use rustify_docs::site::RenderContext;

fn locale_page(context: RenderContext, marker: &'static str) -> impl IntoView {
    let counter = context.demo("CounterDemo");
    let theme = context.demo("ThemeProbeDemo");
    view! {
        <div data-locale-page="" data-context-route=context.route data-context-lang=context.lang data-context-base=context.base>
            <p>{marker}</p>
            <h2 id="shared-section">"Shared section"</h2>
            {counter}
            {theme}
        </div>
    }
}

pub fn english(context: RenderContext) -> impl IntoView {
    locale_page(context, "English locale page")
}

pub fn chinese(context: RenderContext) -> impl IntoView {
    locale_page(context, "中文语言页面")
}

pub fn before_content(context: RenderContext) -> impl IntoView {
    let demo = (context.route == "/404/").then(|| context.demo("CounterDemo"));
    view! {
        <div data-slot="before_content" data-context-route=context.route data-context-lang=context.lang data-context-base=context.base>
            {demo}
        </div>
    }
}

pub fn not_found(context: RenderContext) -> impl IntoView {
    let demo = context.demo("ThemeProbeDemo");
    view! {
        <div data-slot="not_found" data-context-route=context.route data-context-lang=context.lang data-context-base=context.base>
            <p>"This page is unavailable."</p>
            {demo}
        </div>
    }
}
