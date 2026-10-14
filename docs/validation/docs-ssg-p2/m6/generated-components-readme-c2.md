# docs-site

A documentation site with Markdown pages and Rust components, built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

## Create and run the site

The command that created this project is:

```sh
rustify-docs new 'docs-site' --template components
cd 'docs-site'
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

The package name in `Cargo.toml` is `docs-site`; the site's display name
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
Page titles and routes must be unique across registered and Markdown pages.

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

## Theme, fonts and included code

Export a complete Theme Studio JSON document into the project, then set:

```toml
[theme]
document = "theme.json"
```

The build validates both light and dark modes and projects the same theme
onto reading content and interactive blocks. System mode follows the
reader's preference. Referenced fonts come from the CLI's bundled font
bytes and licenses and are published under `assets/fonts/`; there are no
remote font requests. The theme document itself is an input, not a public
asset. Without an imported document, the site uses its default appearance.

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

This project was generated with CLI 0.1.12 at source revision
`c1061ced3deaa4e6669bc45eb3382306fa85c958`, paired with framework revision `0494f304f0f9f8b620affb7309df23db3cb2377c`.
Reinstall that CLI with:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev c1061ced3deaa4e6669bc45eb3382306fa85c958 --locked --profile docs-cli rustify-docs-cli --force
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

The three `rustify-*` dependencies pin that framework revision;
wasm-bindgen pins `=0.2.128`. A mismatched CLI, framework source or bindings
version is rejected before rendering. Keep the profiles in `Cargo.toml`:
the native renderer uses unwind, while browser wasm uses abort, and the
CLI builds them separately with `--locked`.

Before upgrading, back up `Cargo.toml`, `Cargo.lock`, the CLI installation
command and the complete successful `dist/`. Use a published CLI/framework
pair together, update all three framework revisions, explicitly generate
the new dependency lock, and run `check` and `build`. Stop and restart
`dev` after changing the pair. Ordinary `check`, `build` and `dev` are not
dependency-update commands.

To roll back, reinstall the backed-up CLI revision and restore its matching
manifest, lock and complete output. Restore an entire build rather than
mixing old HTML with new wasm, scripts or fonts. The output's
`rustify-docs-manifest.json` records the CLI version and file digests.

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
