use leptos::prelude::*;
use rustify_components::CATALOG;
use rustify_docs::site::RenderContext;

pub fn home(context: RenderContext) -> impl IntoView {
    let quickstart = format!("{}quickstart/", context.base);
    let components = format!("{}components/button/", context.base);
    let introduction = context.markdown_intro("README.md");
    view! {
        <div data-official-home="">
            <div class="rd-hero">
                <h1 class="rd-home-title">"Rustify UI"</h1>
                <p class="rd-tagline">"An experimental Rust SDK for browser applications, with DOM components and optional GPU regions."</p>
                <nav class="rd-actions" aria-label="Start reading">
                    <a class="rd-action rd-action-primary" href=quickstart>"Quick start"</a>
                    <a class="rd-action rd-action-secondary" href=components>"Components"</a>
                </nav>
            </div>
            <ul class="rd-features" data-columns="3">
                <li><h2>"DOM interfaces"</h2><p>"Leptos owns the DOM, layout and reactive application state."</p></li>
                <li><h2>"GPU when needed"</h2><p>"Opt in to GPU regions for content that needs custom drawing."</p></li>
                <li><h2>"Static deployment"</h2><p>"Build browser assets and serve them over HTTP without a backend."</p></li>
            </ul>
            <article class="rd-prose rd-home-prose">{introduction}</article>
        </div>
    }
}

pub fn category(context: RenderContext) -> impl IntoView {
    CATALOG
        .iter()
        .filter(|entry| {
            context.route == format!("/components/{}/", entry.category.name().replace(' ', "-"))
        })
        .map(|entry| {
            let demo_name = entry
                .category
                .name()
                .split(' ')
                .map(|word| {
                    let mut letters = word.chars();
                    match letters.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + letters.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<String>()
                + "Demo";
            let demo = context.demo(&demo_name);
            let capabilities = entry.capabilities().into_iter().map(|(name, capability)| {
                view! {
                    <tr>
                        <th scope="row">{name}</th>
                        <td>{capability.support.name()}</td>
                        <td>{capability.note}</td>
                    </tr>
                }
            }).collect_view();
            view! {
                <div data-component-category=entry.category.name()>
                    <p>{entry.properties.note}</p>
                    <h2 id="example">"Example"</h2>
                    {demo}
                    <h2 id="capabilities">"Capabilities"</h2>
                    <div class="rd-table" role="region" aria-label="Component capabilities" tabindex="0"><table>
                        <thead><tr><th scope="col">"Capability"</th><th scope="col">"Support"</th><th scope="col">"Details"</th></tr></thead>
                        <tbody>{capabilities}</tbody>
                    </table></div>
                    <h2 id="gpu-integration">"GPU integration"</h2>
                    <p>"GPU drawing support: "{entry.presentation.gpu.name()}". This documentation example uses the DOM component. See the "
                        <a href="https://github.com/rockie/rustify-ui/tree/main/examples/component-catalog">"component catalogue example"</a>
                        " for the available DOM and GPU presentations."
                    </p>
                </div>
            }
        })
        .collect_view()
}
