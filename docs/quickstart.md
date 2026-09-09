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
cargo xtask serve --example fusion-basic --release          # prints http://127.0.0.1:<port>/
cargo xtask serve --example fusion-basic --release --base /tools/demo/ --port 4173
cargo xtask serve --example fusion-basic --release --csp no-wasm   # negative test: wasm must fail visibly
```

`build-web` runs the fork's `cargo-makepad` (nightly, `build-std`, single-threaded wasm, in-process wasm-bindgen), extracts the static message bridge from the built wasm with a host interpreter, copies the runtime JS and the example page, and writes `build-manifest.json` with a size report. Output lives in `target/makepad-wasm-app/release/<example>/` and is a plain static directory.

## Verification commands

```sh
cargo test --workspace --lib
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check           # the fork under makepad/ is excluded by its rustfmt.toml
cargo xtask sources verify           # drift of makepad/ against the import record
npm run test:browser                 # Playwright probes against a release build
cd makepad && cargo test -p cargo-makepad
```

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
same numbers. `rustify_ui::CATALOG` (printed in `docs/components.md`) says which
categories exist and what each of them supports.

`rustify_ui::mount(container, config, view)` returns an `AppHandle`; disposing it runs the scope's cleanups (destroying its regions) and unmounts the DOM. The page loads the wasm through `web/loader.js` (`boot(...)`), which validates the bridge fingerprint and hands the host hooks to the runtime before the application's own exported entry points are called. `examples/fusion-basic/app.js` is the reference page script.

## Limits of the preview

Error recovery, capability refusal and bounded diagnostics are later milestones, and nine of R18's eighteen component categories are not in this release at all. See `docs/components.md` for what each category supports, `docs/compatibility.md` for the capability state and the one third-party component that is verified, and `docs/plan/P1-WASM-UI.md` for progress.
