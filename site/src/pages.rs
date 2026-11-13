use leptos::prelude::*;
use rustify_components::CATALOG;
use rustify_docs::site::RenderContext;

use crate::showcase::{ShowcaseEntry, GROUPS, SHOWCASE};
use rustify_colors::ColorFamily;

/// The public Theme Studio application. The absolute URL keeps the Docs link
/// checker satisfied before the application is deployed beside the site; the
/// Pages composer localizes exactly this URL to the packaged copy.
const PUBLIC_STUDIO_URL: &str = "https://rockie.github.io/rustify-ui/theme-studio/";

pub fn home(context: RenderContext) -> impl IntoView {
    let quickstart = format!("{}quickstart/", context.base);
    let components = format!("{}components/", context.base);
    let colors = format!("{}colors/", context.base);
    let primitives = format!("{}primitives/", context.base);
    let studio = format!("{}studio/", context.base);
    let introduction = context.markdown_intro("README.md");
    view! {
        <div data-official-home="">
            <div class="rd-hero">
                <h1 class="rd-home-title">"Rustify UI"</h1>
                <p class="rd-tagline max-md:text-base">"An experimental Rust SDK for browser applications, with DOM components and optional GPU regions."</p>
                <nav class="rd-actions max-md:mt-4 max-md:gap-2" aria-label="Start reading">
                    <a class="rd-action rd-action-primary" href=components>"Components"</a>
                    <a class="rd-action rd-action-secondary" href=quickstart>"Quick start"</a>
                    <a class="rd-action rd-action-secondary" href=PUBLIC_STUDIO_URL>"Open Theme Studio"</a>
                </nav>
            </div>
            <ul class="rd-features mt-8 max-md:mt-6 max-md:gap-2 [&_li]:max-md:pt-2 [&_p]:max-md:mt-1 [&_p]:max-md:text-[0.8125rem] [&_p]:max-md:leading-snug" data-columns="3">
                <li><h2><a href=colors>"Colors"</a></h2><p>"Light and dark scales with downloadable CSS."</p></li>
                <li><h2><a href=primitives>"Primitives"</a></h2><p>"Raw parts alongside their themed components."</p></li>
                <li><h2><a href=studio>"Themes"</a></h2><p>"Edit, inspect and export your theme."</p></li>
            </ul>
            <section class="rd-preview mt-8 max-md:mt-4 [&_.rd-demo-source]:hidden [&_.demo-controls]:hidden [&_.demo-help]:hidden">
                <h2>"Live components"</h2>
                <p class="text-sm text-muted-foreground">"Real DOM components from the released packages, not screenshots."</p>
                <div class="rd-preview-grid grid gap-4 md:grid-cols-2">
                    {context.demo("ButtonDemo")}
                    {context.demo("SegmentedControlDemo")}
                </div>
            </section>
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
            let family = SHOWCASE.iter().find(|family| family.route == context.route);
            let purpose = match entry.category {
                rustify_components::Category::Button => "Run an action such as saving or confirming. Use Link when the destination is another page.",
                rustify_components::Category::Label => "Name a form control and connect its text to the input. Use ordinary text for instructions that do not label a control.",
                rustify_components::Category::Link => "Navigate to a page or location with a native link. Use Button for an application command.",
                rustify_components::Category::Icon => "Reinforce an action or status with a glyph. Give standalone meaningful icons an accessible name; keep decorative icons hidden.",
                rustify_components::Category::TextField => "Edit one line of text. Use TextArea for paragraphs or NumberField for numeric drafts.",
                rustify_components::Category::TextArea => "Edit multiline text such as notes. Use TextField for a short single-line value.",
                rustify_components::Category::Checkbox => "Choose independent options or show a mixed parent selection. Use Radio when exactly one option is allowed.",
                rustify_components::Category::Radio => "Choose one option from a short visible list. Use Select when the list is long.",
                rustify_components::Category::Switch => "Turn a setting on or off immediately. Use Checkbox for an option submitted with a form.",
                rustify_components::Category::Select => "Choose from a compact list, including grouped and disabled options. Use Radio when all options should stay visible.",
                rustify_components::Category::Slider => "Adjust a bounded numeric value by pointer or keyboard. Use NumberField when exact typed values matter.",
                rustify_components::Category::Progress => "Show completion against a known total. Use Spinner when progress cannot be measured.",
                rustify_components::Category::Loading => "Announce work in progress. Replace the indicator with the result when the operation finishes.",
                rustify_components::Category::Tooltip => "Add a brief description to a focusable control. Keep essential instructions visible outside the tooltip.",
                rustify_components::Category::Menu => "Offer application commands at an anchor. Use NavigationMenu for ordinary destination links.",
                rustify_components::Category::Dialog => "Ask for a focused decision while retaining the surrounding task. Use AlertDialog for a destructive confirmation.",
                rustify_components::Category::Tabs => "Switch between related panels without navigating to another page. Use TabNav for destination links.",
                rustify_components::Category::ScrollArea => "Keep long content inside a bounded viewport. Avoid nesting scroll areas when the page can scroll naturally.",
                rustify_components::Category::DataTable => "Explore a large dataset with sorting, selection and keyboard navigation. Use Table for a simple static table.",
                rustify_components::Category::Tree => "Explore a hierarchy with expandable branches and keyboard navigation. Use a plain list for flat choices.",
                rustify_components::Category::Toast => "Confirm a completed action without interrupting the current task. Keep essential errors beside the affected control.",
                rustify_components::Category::NumberField => "Edit a number while retaining incomplete drafts during typing. Use Slider for approximate bounded adjustment.",
                rustify_components::Category::ColorField => "Edit a color value while retaining incomplete drafts. Use the Colors explorer to compare complete scales.",
                rustify_components::Category::ToggleGroup => "Choose one or more related command states. Use Tabs when the choice switches a content panel.",
            };
            let source_note = family.map(|family| entry_source_note(family, &context.base));
            let guide = family.map(|family| format!("{}components/{}/",context.base,family.group));

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
                    <p>{purpose}</p>
                    <h2 id="example">"Example"</h2>
                    {demo}
                    <h2 id="public-api">"Public API and composition"</h2>
                    {source_note}
                    <p class="mt-[1.125rem]">"The source above shows this component and the parts the application passes as its children. State stays in the application: the example reports the values it accepts after each action."</p>
                    <p class="mt-[1.125rem]">{guide.map(|href| view! { <a href=href>"Related components"</a> })}{" · "}<a href=PUBLIC_STUDIO_URL>"Explore the Components scene in Theme Studio"</a></p>
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

/// Group label for a `/components/<group>/` route, or `None` off the matrix.
fn group_of(route: &str) -> Option<&'static str> {
    GROUPS
        .iter()
        .map(|(slug, _)| *slug)
        .find(|slug| route == format!("/components/{slug}/"))
}

fn entry_heading(entry: &ShowcaseEntry) -> impl IntoView {
    let anchor = format!("{}#{}", entry.route, entry.anchor);
    view! {
        <h2 id=entry.anchor class="group">{entry.title}<a class="rd-anchor-link ml-1.5 inline-block opacity-0 group-hover:opacity-100 focus-visible:opacity-100 before:content-['§']" href=anchor aria-label=format!("Link to {}", entry.title)></a></h2>
    }
}

fn entry_source_note(entry: &ShowcaseEntry, base: &str) -> impl IntoView {
    let paths = entry.public_paths.join(", ");
    let guide = if entry.guide.is_empty() {
        None
    } else {
        Some(format!("{base}{}", entry.guide.trim_start_matches('/')))
    };
    view! {
        <p class="text-sm text-muted-foreground">
            "Public paths: "<code class="break-words">{paths}</code>
            {guide.map(|href| view! { " · " <a href=href>"Guide"</a> })}
        </p>
    }
}

pub fn showcase_category(context: RenderContext) -> impl IntoView {
    let Some(group) = group_of(&context.route) else {
        return view! {
            <div data-showcase-error="">
                <p>"This showcase page is not part of the release matrix."</p>
            </div>
        }
        .into_any();
    };
    let label = GROUPS
        .iter()
        .find(|(slug, _)| *slug == group)
        .map(|(_, label)| *label)
        .unwrap_or(group);
    let index = format!("{}components/", context.base);
    let entries: Vec<&ShowcaseEntry> = SHOWCASE
        .iter()
        .filter(|entry| entry.group == group)
        .collect();
    let linked: Vec<&ShowcaseEntry> = entries
        .iter()
        .copied()
        .filter(|entry| entry.route != context.route)
        .collect();
    let linked_view = linked
        .iter()
        .map(|entry| {
            let href = format!("{}{}#{}", context.base, entry.route.trim_matches('/'), entry.anchor);
            view! {
                <li><a href=href>{entry.title}</a><span class="text-sm text-muted-foreground">{" · "}{entry.public_paths.first().copied().unwrap_or_default()}</span></li>
            }
        })
        .collect_view();
    let linked_section = (!linked.is_empty()).then(move || {
        view! {
            <h2 id="component-pages">"Component pages"</h2>
            <ul class="rd-showcase-links">{linked_view}</ul>
        }
        .into_any()
    });
    let sections = entries
        .iter()
        .filter(|entry| entry.route == context.route)
        .map(|entry| {
            let demo = context.demo(entry.demo);
            let note = entry_source_note(entry, &context.base);
            let heading = entry_heading(entry);
            view! {
                <section class="rd-showcase-entry" data-showcase-entry=entry.anchor>
                    {heading}
                    {demo}
                    {note}
                </section>
            }
        })
        .collect_view();
    view! {
        <div data-showcase-category=group>
            <nav class="text-sm" aria-label="Breadcrumb"><a href=index>"Components"</a>{" › "}{label}</nav>
            <h1>{label}</h1>
            {linked_section}
            {sections}
        </div>
    }
    .into_any()
}

pub fn colors_page(context: RenderContext) -> impl IntoView {
    let families = ColorFamily::ALL
        .iter()
        .map(|family| format!("{family:?}").to_lowercase())
        .collect::<Vec<_>>();
    const SCALES: &[(&str, &str)] = &[
        ("-light", "Light"),
        ("-dark", "Dark"),
        ("-light-alpha", "Light alpha"),
        ("-dark-alpha", "Dark alpha"),
        ("-light-p3", "Light P3"),
        ("-dark-p3", "Dark P3"),
        ("-light-alpha-p3", "Light alpha P3"),
        ("-dark-alpha-p3", "Dark alpha P3"),
    ];
    let downloads = families
        .iter()
        .map(|name| {
            let cells = SCALES
                .iter()
                .map(|(suffix, label)| {
                    let href = format!("{}colors/{name}{suffix}.css", context.base);
                    let id = format!("css-{name}{suffix}");
                    view! { <td><a class="whitespace-nowrap target:bg-primary target:text-primary-foreground target:outline-2 target:outline-primary target:outline-offset-2" id=id href=href download="">{*label}</a></td> }
                })
                .collect_view();
            view! { <tr><th scope="row">{name.clone()}</th>{cells}</tr> }
        })
        .collect_view();
    let alpha_downloads = ["black", "white"].into_iter().map(|name| {
        let links = [("-alpha", "sRGB"), ("-alpha-p3", "P3")].into_iter().map(|(suffix,label)| {
            view! { <td><a class="whitespace-nowrap target:bg-primary target:text-primary-foreground target:outline-2 target:outline-primary target:outline-offset-2" id=format!("css-{name}{suffix}") href=format!("{}colors/{name}{suffix}.css",context.base) download="">{label}</a></td> }
        }).collect_view();
        view! { <tr><th scope="row">{format!("{name}-alpha")}</th>{links}</tr> }
    }).collect_view();
    let license = format!("{}colors/LICENSE.txt", context.base);
    view! {
        <div data-colors-page="">
            <h1>"Colors"</h1>
            <p>"Every color family released in "<code>rustify-colors</code>", with its twelve steps in light and dark, solid and alpha, sRGB and Display P3. Browse the scales below, copy the raw strings, or download the packaged CSS."</p>
            <section id="colors">{context.demo("ColorsDemo")}</section>
            <h2 id="downloads">"Static CSS downloads"</h2>
            <p class="text-sm text-muted-foreground">"Each file sets twelve custom properties on an explicitly marked element and never touches root selectors. Alpha scales add the -a1 through -a12 suffix; Display P3 files use the -p3.css suffix and import their sRGB sibling. Black and white alpha scales are packaged as black-alpha and white-alpha."</p>
            <div class="rd-table rd-color-downloads mt-[1.125rem]" role="region" aria-label="Color CSS downloads" tabindex="0"><table>
                <thead><tr><th scope="col">"Family"</th>{SCALES.iter().map(|(_,label)| view! { <th scope="col">{*label}</th> }).collect_view()}</tr></thead>
                <tbody>{downloads}</tbody>
            </table></div>
            <div class="rd-table rd-color-downloads mt-[1.125rem]" role="region" aria-label="Black and white alpha downloads" tabindex="0"><table>
                <thead><tr><th scope="col">"Scale"</th><th scope="col">"Alpha"</th><th scope="col">"Alpha P3"</th></tr></thead>
                <tbody>{alpha_downloads}</tbody>
            </table></div>
            <p class="mt-[1.125rem] text-sm text-muted-foreground">"The packaged files are redistributed under their original license: "<a href=license>"colors LICENSE"</a>". P3 strings need a P3-capable browser and display; sRGB files never import P3 assets."</p>
        </div>
    }
}

pub fn primitives_page(context: RenderContext) -> impl IntoView {
    let groups: Vec<(String, String)> = GROUPS
        .iter()
        .map(|(slug, label)| {
            (
                format!("{}components/{}/", context.base, slug),
                label.to_string(),
            )
        })
        .collect();
    let links = groups
        .iter()
        .map(|(href, label)| {
            view! { <li><a href=href.clone()>{label.clone()}</a></li> }
        })
        .collect_view();
    view! {
        <div data-primitives-page="">
            <h1>"Primitives"</h1>
            <p>"The unstyled parts underneath the themed components: raw roots, typed native decorators for buttons and inputs, direction handling, portals, accessible icons and visually hidden text. Operate the same control raw and themed and compare the structure and state."</p>
            <section id="primitives">{context.demo("PrimitivesDemo")}</section>
            <section id="direction">{context.demo("DirectionDemo")}</section>
            <h2 id="families">"Themed families"</h2>
            <p class="text-sm text-muted-foreground">"Each themed family lists its raw parts where it has them; the category pages show the working themed controls."</p>
            <ul class="rd-showcase-links">{links}</ul>
        </div>
    }
}

pub fn studio_page(context: RenderContext) -> impl IntoView {
    let themes = format!("{}themes/", context.base);
    view! {
        <div data-studio-page="">
            <h1>"Theme Studio"</h1>
            <p>"The complete public Theme Studio workbench: a real application, not a reduced panel. Open it beside this documentation and edit the same theme system the site uses."</p>
            <nav class="rd-actions max-md:mt-4 max-md:gap-2" aria-label="Open the workbench">
                <a class="rd-action rd-action-primary" href=PUBLIC_STUDIO_URL>"Open Theme Studio"</a>
                <a class="rd-action rd-action-secondary" href=themes>"Theme guide"</a>
            </nav>
            <h2 id="workflow">"The complete workflow"</h2>
            <ol class="rd-studio-steps">
                <li>"Pick an existing preset, then adjust the brand and neutral families, panels, radius and scale."</li>
                <li>"Fine-tune Colors, Typography and Other groups in the editor."</li>
                <li>"Check Components scenes for the variant, state and part you care about."</li>
                <li>"Select a real component or overlay in the Inspector and read its effective values with their root, local and inherited sources."</li>
                <li>"Undo and redo edits, save, and refresh: the draft and saved state return from the same local key."</li>
                <li>"Export JSON, CSS or Rust, and consume the exported theme in an independent registry 0.3.0 DOM application."</li>
            </ol>
            <h2>"Theme boundaries"</h2>
            <p class="text-sm text-muted-foreground">"The page theme stays the page's; overlays inherit the theme that created them. This sample shows a local boundary changing tokens for one surface only."</p>
            <section id="theme-boundary">{context.demo("ThemeBoundaryDemo")}</section>
            <h2 id="what-stays">"What is preserved"</h2>
            <p>"Import and export, the local library, drafts, error recovery and the independent scope comparison all remain. The GPU preview stays an optional mode with its own real support and errors; nothing requires an account, and the studio never shares memory with this documentation site."</p>
        </div>
    }
}

pub fn home_zh(context: RenderContext) -> impl IntoView {
    let quickstart = format!("{}zh/quickstart/", context.base);
    let components = format!("{}zh/components/", context.base);
    let colors = format!("{}zh/colors/", context.base);
    let primitives = format!("{}zh/primitives/", context.base);
    let studio = format!("{}zh/studio/", context.base);
    view! {
        <div data-official-home="">
            <div class="rd-hero">
                <h1 class="rd-home-title">"Rustify UI"</h1>
                <p class="rd-tagline max-md:text-base">"面向浏览器应用的实验性 Rust SDK，提供 DOM 组件和按需启用的 GPU region。"</p>
                <nav class="rd-actions max-md:mt-4 max-md:gap-2" aria-label="开始阅读">
                    <a class="rd-action rd-action-primary" href=components>"浏览组件"</a>
                    <a class="rd-action rd-action-secondary" href=quickstart>"快速开始"</a>
                    <a class="rd-action rd-action-secondary" href=PUBLIC_STUDIO_URL>"打开 Theme Studio"</a>
                </nav>
            </div>
            <ul class="rd-features mt-8 max-md:mt-6 max-md:gap-2 [&_li]:max-md:pt-2 [&_p]:max-md:mt-1 [&_p]:max-md:text-[0.8125rem] [&_p]:max-md:leading-snug" data-columns="3">
                <li><h2><a href=colors>"Colors"</a></h2><p>"全部已发布色族的浅深色阶与可下载的打包 CSS。"</p></li>
                <li><h2><a href=primitives>"Primitives"</a></h2><p>"raw 部件与类型化原生装饰，以及对应的 themed 控件。"</p></li>
                <li><h2><a href=studio>"Themes"</a></h2><p>"完整公开的 Theme Studio：预设、精调、检查与导出。"</p></li>
            </ul>
            <section class="rd-preview mt-8 max-md:mt-4 [&_.rd-demo-source]:hidden [&_.demo-controls]:hidden [&_.demo-help]:hidden">
                <h2>"真实组件"</h2>
                <p class="text-sm text-muted-foreground">"以下为可操作的 DOM 组件，示例界面为英文。"</p>
                <div class="rd-preview-grid grid gap-4 md:grid-cols-2">
                    {context.demo("ButtonDemo")}
                    {context.demo("SegmentedControlDemo")}
                </div>
            </section>
            <article class="rd-prose rd-home-prose">
                <p>"Rustify UI 以 Leptos CSR 管理浏览器界面，默认构建仅使用 DOM。启用 SDK 的 gpu feature 后，GpuRegion 在由 DOM 定位的 canvas 中运行 Makepad WebGL2 绘制；应用向 region 传入 props，并接收类型化动作。"</p>
                <p>"每项任务按实际需要选择呈现路径。共享状态和主题 token 不要求 DOM 与 GPU 拥有相同的组件目录或 CSS 支持；只有明确需要两端或跨区协作时，才同时实现两端。当前尚未选定具体业务场景。"</p>
                <p>"本仓包含 SDK、DOM 组件、Makepad fork 和可运行示例。Docs、DOM 与样式等十个包以 0.3.0 发布在 crates.io；GPU SDK、Makepad 桥接和示例仍从 Git 使用。从快速开始进入构建，或在组件文档中操作实际 DOM 示例并查看 Rust 源码。"</p>
            </article>
        </div>
    }
}
