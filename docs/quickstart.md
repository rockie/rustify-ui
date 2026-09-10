# Quick start

## Prerequisites

- `rustup`. The toolchain in `rust-toolchain.toml` (a pinned nightly with `rust-src` and the wasm32 target) is installed by rustup on first use; nothing else is installed silently.
- Node 26 and npm for the browser tests: `npm ci` then `npx playwright install chromium`.
- Google Chrome on macOS for the manual pass gate.

`cargo xtask doctor` reports the state of all of the above and never changes it.

## Build and preview an example

```sh
cargo xtask build-web --example fusion-basic --release
cargo xtask build-web --example property-workbench --release
cargo xtask build-web --example component-catalog --release
cargo xtask serve --example fusion-basic --release          # prints http://127.0.0.1:<port>/
cargo xtask serve --example fusion-basic --release --base /tools/demo/ --port 4173
cargo xtask serve --example fusion-basic --release --csp no-wasm   # negative test: wasm must fail visibly
cargo xtask serve --example fusion-basic --release --fault missing:makepad_widgets/resources/IBMPlexSans-Text.ttf
cargo xtask report-size --example fusion-basic --release           # six category totals, read off the directory
cargo xtask report-size --example fusion-basic --release --compressed  # and what a gzip host would send
```

`serve --fault` makes the server break the deployment in one specific way -
`missing:<path>`, `corrupt:<path>`, `truncated:<path>` or `stale-bridge` - so a
failure can be exercised against the running product instead of a doctored
copy of it. A running server also takes `GET <base>__fault/<spec>` (and
`.../none` to stop), which is how the browser tests switch between them
without a restart.

`build-web` runs the fork's `cargo-makepad` (nightly, `build-std`, single-threaded wasm, in-process wasm-bindgen), extracts the static message bridge from the built wasm with a host interpreter, copies the runtime JS and the example page, and writes `build-manifest.json` with a size report. Output lives in `target/makepad-wasm-app/release/<example>/` and is a plain static directory.

## Verification commands

```sh
cargo test --workspace --lib
cargo xtask css --check              # the component stylesheet is in step with the classes
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check           # the fork under makepad/ is excluded by its rustfmt.toml
cargo xtask sources verify           # drift of makepad/ against the import record
npm run test:browser                 # Playwright probes against a release build
cd makepad && cargo test -p cargo-makepad
cargo xtask verify --suite p2        # everything above in one report, plus what needs a person
```

`verify` runs the checks, double-builds each example, runs every browser
project, and then lists the records only a person can write - the VoiceOver
pass, real pinyin input, the sample comparison, the zoom walkthrough. It reports
those as *missing*, never as passed, which is the whole reason it exists.

## Changing a component's classes

The component crate's classes are Tailwind utilities behind a `rui:` prefix,
and the stylesheet they need is **committed**, so building the wasm never needs
Node:

```sh
cargo xtask css            # regenerate crates/rustify-components/css/rustify.css
cargo xtask css --check    # fail if the committed product is not what the input produces
```

A class string added without regenerating would simply have no rule; `--check`
is what turns that into a failure. `build-web` copies the product into any
example whose manifest names the component crate.

The prefix is what keeps a host page's own Tailwind build and ours apart, and
`dark` is bound to the SDK's scope attribute rather than to any `.dark`
ancestor, so a host page using that convention does not darken a scope.

## Two languages

```rust
let locale = rustify_ui::provide_locale(Locale::English);   // once, in the scope root
locale.text(Message::Retry)                                  // tracked; changes with the language
```

The SDK ships the words it puts on screen for itself; the application's
vocabulary is the application's. `format_number` and `format_date` go through
`Intl` with an options object the application builds, because what a value
*means* decides how it should be written. See `docs/i18n.md`.

## Writing a GPU region

A region is a Makepad app that implements `RegionApp`: it declares its widget tree with `script_mod!`, projects host state in through `apply_props`, and pushes typed actions into the outbox while handling events. The host renders it with `GpuRegion`, passing a `Signal` of props and an action callback; every change to the signal is applied at the region's next safe point, and actions arrive after the pump that produced them.

```rust
impl RegionApp for CounterRegion {
    type Props = CounterProps;
    type Action = CounterAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &CounterProps) {
        self.ui.label(cx, ids!(counter_label)).set_text(cx, &format!("Count: {}", props.count));
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<CounterAction>) {
        if let Event::Actions(actions) = event {
            if self.ui.button(cx, ids!(increment_button)).clicked(actions) {
                outbox.push(CounterAction::Increment);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

```rust
view! {
    <GpuRegion app=PhantomData::<CounterRegion> props=props on_action=on_action class="gpu-region" />
}
```

## Using the DOM components

The DOM half of the component subset lives in `rustify_ui::components`. Every
control is controlled by the application: the user's action is a request, and
what the control shows afterwards is the answer. A refused value is replaced by
the value in force rather than left on screen as if it had been taken.

```rust
view! {
    <TextField
        label="name"
        value=Signal::derive(move || current.get().map(|o| o.name).unwrap_or_default())
        // Return without changing the signal and the field goes back to the
        // value the application holds.
        on_input=move |value| { rename(value); }
        read_only=Signal::derive(move || current.get().is_some_and(|o| o.locked))
    />
    <Slider label="size" min=0.0 max=100.0 step=5.0 value=size on_change=move |v| set_size(v) />
    <LoadView label="details" value=details on_retry=move || reload() />
}
```

`ThemedScope` writes the scope's tokens onto its own root, so two scopes on one
page hold different themes and the host page keeps its own; `ThemeOverride`
changes part of the theme for one area and leaves the rest inherited. A region
inside an override is handed the patched table, so both halves draw with the
same numbers. `rustify_components::CATALOG` (printed in `docs/components.md`)
says which categories exist and what each of them supports.

`rustify_ui::mount(container, config, view)` returns an `AppHandle`; disposing it runs the scope's cleanups (destroying its regions) and unmounts the DOM. The page loads the wasm through `web/loader.js` (`boot(...)`), which validates the bridge fingerprint and hands the host hooks to the runtime before the application's own exported entry points are called. `examples/fusion-basic/app.js` is the reference page script.

## Limits of the preview

All eighteen of R18's component categories have a DOM component
(`rustify_components`), and fourteen of them are drawn by a GPU region as well;
the four that are not say where they are drawn instead. There is no router, no
workspace, no large-data view and no cross-region drag - those are P2 M4 to M6.
See `docs/components.md` for what each category supports today, `docs/compatibility.md` for the capability state
and the one third-party component that is verified, `docs/reports/p1/` for what
P1 measured and what it could not, and `docs/plan/` for progress.
