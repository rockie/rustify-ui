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

pub fn home_zh(context: RenderContext) -> impl IntoView {
    let quickstart = format!("{}zh/quickstart/", context.base);
    let components = format!("{}zh/components/button/", context.base);
    view! {
        <div data-official-home="">
            <div class="rd-hero">
                <h1 class="rd-home-title">"Rustify UI"</h1>
                <p class="rd-tagline">"面向浏览器应用的实验性 Rust SDK，提供 DOM 组件和按需启用的 GPU region。"</p>
                <nav class="rd-actions" aria-label="开始阅读">
                    <a class="rd-action rd-action-primary" href=quickstart>"快速开始"</a>
                    <a class="rd-action rd-action-secondary" href=components>"组件"</a>
                </nav>
            </div>
            <ul class="rd-features" data-columns="3">
                <li><h2>"DOM 界面"</h2><p>"Leptos 管理 DOM、布局和响应式应用状态。"</p></li>
                <li><h2>"按需启用 GPU"</h2><p>"需要自定义绘制的交互内容可使用 GPU region。"</p></li>
                <li><h2>"静态部署"</h2><p>"构建浏览器资源后通过 HTTP 托管，无需后端服务。"</p></li>
            </ul>
            <article class="rd-prose rd-home-prose">
                <p>"Rustify UI 以 Leptos CSR 管理浏览器界面，默认构建仅使用 DOM。启用 SDK 的 gpu feature 后，GpuRegion 在由 DOM 定位的 canvas 中运行 Makepad WebGL2 绘制；应用向 region 传入 props，并接收类型化动作。"</p>
                <p>"每项任务按实际需要选择呈现路径。共享状态和主题 token 不要求 DOM 与 GPU 拥有相同的组件目录或 CSS 支持；只有明确需要两端或跨区协作时，才同时实现两端。当前尚未选定具体业务场景。"</p>
                <p>"本仓包含 SDK、DOM 组件、Makepad fork 和可运行示例。工作区版本为 0.1.0，尚未开放包发布；从快速开始进入本地构建，或在组件文档中操作实际 DOM 示例并查看 Rust 源码。"</p>
            </article>
        </div>
    }
}
