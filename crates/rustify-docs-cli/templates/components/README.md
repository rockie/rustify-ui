# {{name}}

A documentation site with Markdown pages and Rust components, built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

## Create and run the site

The command that created this project is:

```sh
rustify-docs new {{directory}} --template components
cd {{directory}}
rustify-docs dev
```

Use stable Rust with the `wasm32-unknown-unknown` target installed. If either
is missing, `new` stops before writing the project:

```sh
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
```

The first `dev` compiles a native renderer and a browser wasm module, and
may download locked dependencies. It prints the local address first; open
that address while the initial build runs. The examples are usable when
the loading message disappears and Increment responds. Later Rust saves reuse the two
compilation targets. Markdown-only saves reuse the compiled renderer and
wasm module.

| Command | What it does |
| --- | --- |
| `rustify-docs new <directory> --template components` | Creates this project in a new or empty directory |
| `rustify-docs dev` | Serves the site and updates it as you save |
| `rustify-docs check` | Checks content, registrations and both Rust targets; may write private compilation caches, but leaves source and published output unchanged |
| `rustify-docs build` | Builds and validates a complete site, then publishes it to `dist/` |
| `rustify-docs preview` | Serves the complete `dist/` with static-host behavior |

The package name in `Cargo.toml` is `{{package}}`; the site's display name
keeps the directory name. The library name is always `site_docs`, including
when the directory name is a Rust keyword. Commit `Cargo.toml` and
`Cargo.lock`. Normal commands use the supplied lock without changing it.
The CLI includes wasm bindings support; there is no separate wasm-pack,
wasm-bindgen CLI or GPU toolchain to install.

## Add your first demo

The home page already contains a `Counter` whose Markdown props set its
initial value, and `CounterDemo` whose Rust function is shown beside it.
To add another demo, put this function in `src/demos.rs`:

```rust
pub fn double_counter_demo() -> impl IntoView {
    counter(14, "Double counter".to_string(), ().into_any())
}
```

The file already imports `leptos::prelude::*`. Add the function to the
existing `demos` block in `site()` in `src/lib.rs`:

```rust
demos {
    CounterDemo => crate::demos::counter_demo,
    DoubleCounterDemo => crate::demos::double_counter_demo,
}
```

Add this block to `content/index.md`:

````markdown
```rustify-demo
name: DoubleCounterDemo
```
````

With `dev` running, save both files and the page. The new example starts at
14; Increment changes it to 15. Its source panel shows the registered
function and its full file. No second copy of that source belongs in the
Markdown page.

Rust errors retain their original file, line and column. A failed rebuild
keeps the last successful page and shows the diagnostics. Fix the source
and save again to replace it with a complete new version; an unfinished
build never mixes its code with the previous page's assets.

## Props, children and the first render

`site!` declares each component's props once. `Counter` accepts `initial:
i32` and `label: String`, with the defaults in `src/lib.rs`. Unknown fields
and values of the wrong type are errors at the Markdown block:

````markdown
```rustify-component
name: Counter
props:
  initial: 10
  label: A counter with a child
children: |
  This **Markdown** appears inside the counter.
```
````

The adapter receives typed props and an `AnyView` containing validated,
rendered child Markdown. Keep those children in the returned view, as
`counter()` does. Their links, images and headings use the same content
pipeline as the rest of the page.

The host and browser render the same view with the same initial props and
IDs. Initial markup must be deterministic: start browser measurements,
async work and browser APIs after attachment, rather than using time,
randomness or a browser-only lookup to choose the first tree.

For a component that cannot render on the host, declare a visible
placeholder explicitly in the `components` block:

```rust
#[client_only(class = "min-h-48", message = "This counter starts in the browser.")]
BrowserCounter { initial: i32 = 2, label: String = "Browser counter".to_string() } =>
    |props, children| demos::counter(props.initial, props.label, children),
```

Use `name: BrowserCounter` in its component fence. The placeholder is
available without JavaScript; after it attaches, the real component mounts
inside it. Choose a placeholder size that matches the initial client view.
Ordinary counters use SSR and do not need this declaration.

## Custom pages and layout slots

A registered page can reuse the same demo and source block. Add this to
`src/demos.rs`:

```rust
pub fn examples_page(context: rustify_docs::site::RenderContext) -> impl IntoView {
    let first = context.demo("CounterDemo");
    let second = context.demo("CounterDemo");
    view! {
        <section>
            <h2 id="paired-counters">"Two independent counters"</h2>
            {first}
            {second}
        </section>
    }
}

pub fn footer(context: rustify_docs::site::RenderContext) -> impl IntoView {
    view! { <p><a href=context.base>"Home"</a></p> }
}
```

Add these blocks alongside `components` and `demos` in `site!`:

```rust
pages {
    ExamplesPage { route: "/examples/", title: "Examples" } => crate::demos::examples_page,
}
slots {
    footer => crate::demos::footer,
}
```

`RenderContext` supplies `route`, `lang`, `base` and `demo(name)`. Each call
to `demo` has independent state and IDs, with the same source controls as
a Markdown demo. Use `base` when making links to other registered routes.
Page titles and routes must be unique within each configured language.

The seven slots are `header`, `footer`, `sidebar_top`, `sidebar_bottom`,
`before_content`, `after_content` and `not_found`. Each accepts a function
with the same context. `header` replaces the brand area while the framework
keeps navigation and mode controls. `not_found` replaces the 404 body;
`footer` also appears on that page. A demo in an unconditional global slot
creates an interactive block on every page, so keep shared reading content
static or make the demo conditional on `context.route`.

Add the custom page to navigation in `rustify-docs.toml`:

```toml
[[nav.groups]]
label = "Examples"
routes = ["/examples/"]
```

The existing Guide group uses `dir = "guide"`; a group chooses one of
`dir`, Markdown `items`, or registered `routes`.

## Publish several languages

The generated project uses one language until you configure
`[[i18n.locales]]`. Keep the locale matching `[site].lang` at `prefix = ""`
and give each other language a distinct prefix, such as `zh` or `fr`.
Markdown translations mirror default paths below that prefix:
`content/zh/guide/getting-started.md` translates
`content/guide/getting-started.md`.

A registered translation keeps the logical route and declares its language:

```rust
pages {
    ExamplesPage { route: "/examples/", title: "Examples" } => crate::demos::examples_page,
    ExamplesZh { route: "/examples/", title: "示例", lang: "zh" } => crate::demos::examples_page,
}
```

Use this in the existing `pages` block. Registration names remain globally
unique. `RenderContext.route` is the logical route and `lang` is the
requested UI language, including when an untranslated page reuses the
default function. The same context reaches slots and demos. SDK controls
use English for languages beyond its built-in English and Chinese.

Each locale may set `messages = "locales/fr.toml"` for project UI text and
its own navigation. Missing pages keep their source language and show a
fallback notice; missing third-language UI keys use English. See the
[author reference](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#publish-several-languages)
for the complete configuration, validation and static-host behavior.

## Theme, fonts and included code

The default reading theme uses system fonts and makes no font requests. To
customize a complete theme without building Theme Studio:

1. Copy `theme.example.json` to `theme.json`. The example is inactive until you
   set `document` below.
2. In both `styles.light` and `styles.dark`, set the needed font token, for
   example `"font-sans": "Brand Sans, sans-serif"`. Keep all other tokens.
3. Put a font you may redistribute at `fonts/brand-sans.woff2`, then add:

```toml
[theme]
document = "theme.json"

[[theme.fonts]]
family = "Brand Sans"
src = "fonts/brand-sans.woff2"
weight = "400" # or an ascending range such as "100 900"
style = "normal" # or "italic"
```

Registration alone does not select a font; the document's font stack selects
it. Paths are project-local, preferably outside `static/`; woff2, woff, ttf
and otf are supported. URLs, escaping symlinks, private files, output files,
empty files and duplicate family/weight/style faces fail the build. Keep
font notices and any redistribution/source obligations with your project.
The build publishes declared bytes at same-origin `assets/fonts/<sha256>.<ext>`
URLs with `font-display: swap`. It performs no font download.

Run `rustify-docs check --deny-warnings` and `rustify-docs build`, then preview
the complete output. Both modes and islands use the same CSS font policy.
The CLI embeds no font files. An imported but undeclared named family gives
`theme-font-system`: use a local declaration or change the relevant tokens in
both modes to `sans-serif`, `serif` or `monospace`. Remaining warnings still
fail `--deny-warnings`. CJK and emoji now follow the author stack and system
fallback rather than the old bundled glyph fallbacks.

For a code example that does not need to be interactive, include the real
file in Markdown:

```markdown
<!-- rustify:include path="src/demos.rs" lang="rust" title="Counter implementation" -->
```

`path` is relative to the project root. An optional `region` selects lines
between `// rustify-region:<name>` and `// rustify-endregion:<name>` markers.
An include displays source; the `rustify-demo` fence registers a live demo
and automatically displays its function and full-file source.

## Keep the CLI and framework paired

This project was generated with CLI {{version}} and the exact crates.io
versions in `Cargo.toml` and `Cargo.lock`. Reinstall that CLI with:

```sh
{{install}} --force
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

`rustify-docs`, `rustify-dom` and `rustify-components` pin `=0.2.0`;
wasm-bindgen pins `=0.2.128`. The lock includes real registry checksums.
A mismatched version, source or duplicate DOM core is rejected before
rendering. Cargo dependency aliases are supported. Keep the profiles in
`Cargo.toml`: the native renderer uses unwind, browser wasm uses abort,
and the CLI builds them separately with `--locked`.

Before upgrading, back up `Cargo.toml`, `Cargo.lock`, the CLI installation
command and the complete successful `dist/`. Install a released CLI, update
all three dependencies to its `--build-info` versions, explicitly generate
the new lock, then run `check` and `build`. Stop and restart `dev` after
changing the pair. Ordinary `check`, `build` and `dev` use the existing lock.

To roll back, reinstall the backed-up CLI version and restore its matching
manifest, lock and complete output. Restore an entire build so HTML, wasm,
scripts and fonts share one generation. The output's
`rustify-docs-manifest.json` records the CLI version and file digests.

For the historical 0.1.32 Git CLI, restore its original Git manifest and
lock together and reinstall it with:

```sh
{{rollback}}
```

When upgrading a 0.1.32 project, replace its `rustify-ui` dependency with
`rustify-dom` and update `rustify_ui` imports to `rustify_dom`. To keep
imports, use `rustify-ui = { package = "rustify-dom", version = "=0.2.0",
default-features = false }` and retain that key in feature lists. Remove
`rustify-components/gpu` from custom feature lists. Back up an imported
theme and declare its local fonts as described above; system fallbacks,
including CJK and emoji, can differ from the old bundled fonts.

## Publish static output

Set `[site].url` to the public address before publishing. Its path must
match `base`; for example, use `url = "https://example.com/my-docs/"` with
`base = "/my-docs/"`. Then run `build` and `preview` and upload all of
`dist/`, including `404.html` and the manifest. No application server or
rewrite of every request to the home page is needed.

Each page includes a content security policy in a `<meta>` element. A host
with response-header support should also send the full policy shown by
`preview` as `Content-Security-Policy`. Serve wasm with
`Content-Type: application/wasm`, and keep scripts, wasm snippets and fonts
at their generated paths. The meta policy supports static hosts without
custom headers; its `frame-ancestors` restriction requires the header.

`dist/`, `target/` and `.rustify-docs/` are ignored. If you use `--out`, also
ignore that output and the private `.rustify-docs/` beside it. Keep source,
configuration and the dependency lock in version control.
