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

`rustify_ui::mount(container, config, view)` returns an `AppHandle`; disposing it runs the scope's cleanups (destroying its regions) and unmounts the DOM. The page loads the wasm through `web/loader.js` (`boot(...)`), which validates the bridge fingerprint and hands the host hooks to the runtime before the application's own exported entry points are called. `examples/fusion-basic/app.js` is the reference page script.

## Limits of the preview

Keyboard, text editing, overlays, theming and error recovery are later milestones; see `docs/compatibility.md` for the exact capability state and `docs/plan/P1-WASM-UI.md` for progress.
