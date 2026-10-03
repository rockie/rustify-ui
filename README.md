# Rustify UI

Rustify UI is an experimental Rust SDK for combining Leptos CSR components and Makepad WebGL2 rendering in a browser application. Leptos owns the DOM, layout and reactive application state; each `GpuRegion` renders into a canvas positioned by the DOM. Applications send props into regions and receive typed actions back.

This repository contains the SDK, DOM components, a Makepad fork and six runnable examples. The workspace is version `0.1.0`, with publishing disabled. Start here to build an example; use the [quick start](docs/quickstart.md) to write a region or integrate the components into an application.

## Quick start

The validated development environment is macOS. Use a browser with WebAssembly and WebGL2; macOS Chrome is the current pass gate. See [compatibility](docs/compatibility.md) for the tested browser matrix.

Install and activate mise before running the commands below. The repository's [mise.toml](mise.toml) pins:

| Tool | Version | Purpose |
| --- | --- | --- |
| Rust | `1.98.1` stable | Includes `rustfmt`, `clippy` and the `wasm32-unknown-unknown` target |
| mbx (mr-boxington) | `1.15.0` | Compiler cache used by the workspace and nested wasm builds |
| Tailwind standalone CLI | `4.1.13` | Generates component CSS and compiles example Tailwind inputs |

Clone the repository and install the pinned tools:

```sh
git clone https://github.com/rockie/rustify-ui.git
cd rustify-ui
mise install
```

Build and serve the smallest example:

```sh
mbx xtask build-web --example fusion-basic --release
mbx xtask serve --example fusion-basic --release --port 4173
```

Open <http://127.0.0.1:4173/> and wait for the page status to read `ready`. The default view has ten DOM controls and one GPU region with twenty controls, backed by the same application state. Click `DOM +1` or change the `visible` checkbox to see the GPU presentation follow the DOM controls. The other sections are test fixtures mounted on request.

`serve` serves an existing build; it does not rebuild or watch source files. After a change, run `build-web` again and reload the page. Without `--port`, the server chooses a free port and prints its URL.

The build produces a static directory at `target/makepad-wasm-app/release/fusion-basic/`, including the wasm, JavaScript, styles, resources and a build manifest. It does not require Node, a database or a backend service. Use HTTP hosting rather than opening the page as a local file.

To diagnose the local setup:

```sh
mbx xtask doctor
```

`doctor` reports toolchain, source records, licenses and browser-test prerequisites without changing them. Its Node, Playwright and installed-Chrome checks also run when you only intend to build an example. The mise Rust postinstall script links `libLLVM` for `rust-lld`; if the diagnostic reports a missing link, use the existing repair script:

```sh
sh scripts/link-libllvm.sh
```

## Examples

Choose an example name for `mbx xtask build-web --example NAME --release` and `mbx xtask serve --example NAME --release`:

| Example | What to try |
| --- | --- |
| [fusion-basic](examples/fusion-basic/) | Update shared DOM/GPU state, mount multiple scopes and exercise independent application instances |
| [component-catalog](examples/component-catalog/) | Browse 24 component categories, switch themes and languages, and inspect DOM/GPU capability differences |
| [property-workbench](examples/property-workbench/) | Select, rename, recolour and delete objects through a DOM property panel and GPU view; try the vendored noUiSlider integration |
| [data-workbench](examples/data-workbench/) | Sort and filter a 100,000-row DOM table, inspect its GPU selection strip, and explore a separate 10,000-object GPU scene |
| [vellum](examples/vellum/README.md) | Edit a design document with a GPU scene, DOM inspector, native text editing, local persistence, import and export |
| [theme-studio](examples/theme-studio/README.md) | Edit full light/dark themes, compare DOM/GPU samples, save locally and export JSON or Tailwind CSS |

For example, to run the design editor:

```sh
mbx xtask build-web --example vellum --release
mbx xtask serve --example vellum --release --port 4179
```

Its [integration guide](docs/vellum.md) explains which capabilities belong to the SDK and which belong to the editor.

## Development

Use `mbx` for compilation, tests and Clippy, including builds in the Makepad fork. Formatting uses Cargo directly. The host checks in [CI](.github/workflows/verify.yml) are:

```sh
cargo fmt --all -- --check
mbx test --workspace --lib --bins
mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins
sh tests/theme-text/run.sh --lib
mbx clippy --workspace --all-targets -- -D warnings
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
(cd makepad && mbx test)
(cd makepad && mbx test --manifest-path platform/script/Cargo.toml)
```

`makepad/` is a separate Cargo workspace. The root workspace excludes it; run its tests separately as above. To run a subset of SDK unit tests:

```sh
mbx test -p rustify-ui --lib router::
```

After changing component class strings, regenerate the committed stylesheet. After changing the capability catalogue, regenerate its reference document:

```sh
mbx xtask css
mbx xtask catalog --write docs/components.md
```

Application Tailwind classes need an application input; the committed SDK stylesheet contains only component classes. The [Tailwind guide](docs/quickstart.md#using-tailwind-v4-in-your-app) shows how to import the SDK layer and keep `source(none)` on the utilities import. The build compiles an example's own `tailwind.css` automatically. `RUSTIFY_TAILWIND` can select another CLI binary, but it must still be version `4.1.13`.

### Browser tests

Browser tests additionally need Node 26 and npm. Install the pinned Playwright package and Chromium:

```sh
npm ci
npx playwright install chromium
```

Build the example first, then run one project at a time:

```sh
mbx xtask build-web --example fusion-basic --release
npx playwright test --project=fusion-basic
```

Playwright starts the required preview servers. Keep port 4173 free for this project, stopping any manual preview that uses it. If Chromium is already installed elsewhere, set `RUSTIFY_CHROMIUM` to its executable path.

Always name a project. Running the entire root suite at once can exhaust memory; simultaneous Playwright runs also interfere with the shared results directory. For faster iteration, name a spec as well:

```sh
mbx xtask build-web --example property-workbench --release
npx playwright test --project=property-workbench m3-workbench.spec.ts
```

Vellum has its own configuration:

```sh
mbx xtask build-web --example vellum --release
npx playwright test -c tests/vellum/playwright.config.ts m2-scene.spec.ts
```

Theme Studio and the independent export consumers each have root and child-path
projects. Build their products, then run each project in a separate invocation:

```sh
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example theme-studio --release --base /tools/demo/
npx playwright test --project=theme-studio --workers=1 --output=test-results/theme-studio
npx playwright test --project=theme-studio-deep --workers=1 --output=test-results/theme-studio-deep
mbx fetch --locked --manifest-path tests/theme-export/Cargo.toml
sh tests/theme-export/build.sh
sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
npx playwright test --project=theme-export --workers=1 --output=test-results/theme-export
npx playwright test --project=theme-export-deep --workers=1 --output=test-results/theme-export-deep
```

Keep ports 4181–4184 free. The [editor guide](examples/theme-studio/README.md)
describes fresh storage contexts; the [consumer fixture guide](tests/theme-export/README.md)
describes its strict CSP server and resource paths. CI runs each new project
serially and saves its JSON report before starting the next one. The headed
Theme Studio measurement fixture is skipped on headless CI, so CI does not
establish its real-GPU timings.

The default `regression` tier checks behaviour. `RUSTIFY_TIER=evidence` runs tagged measurements at full strength; `RUSTIFY_TIER=all` runs both tiers. Frame budgets require headed Chrome on a real GPU. See the [verification guide](docs/quickstart.md#verification-commands) for evidence runs, sub-path builds and release checks; [evidence CI](.github/workflows/evidence.yml) runs measurements weekly and on request.

## Runtime and deployment boundaries

One application instance contains both Leptos and Makepad in one wasm instance. A page can start multiple independent instances with separate memory and host hooks. A trap ends every scope and region in the affected instance. Restarts are capped at three per slot because dead-instance memory remains until the page unloads.

Build output supports plain static hosting without cross-origin isolation. The preview server applies a strict Content Security Policy that allows WebAssembly through `'wasm-unsafe-eval'`, without JavaScript `'unsafe-eval'`. For sub-path hosting, build and serve with the same `--base`; use `--spa` when the application needs deep-link fallback. See the [deployment contract](docs/compatibility.md#deployment-contract).

This preview uses single-threaded wasm and WebGL2. It provides no WebGPU backend, `SharedArrayBuffer`, hot module replacement or GPU-drawn table. Text editing uses native DOM controls; large-table rendering uses DOM windowing, and long jobs yield in slices on the main thread. Windows, Linux and mobile support is unverified, and the large data examples have no verified screen-reader support. The [known limitations](docs/reports/p3/known-limitations.md) distinguish implemented behaviour from deferred or untested work.

## Repository and documentation

| Path | Contents |
| --- | --- |
| [crates/rustify-ui](crates/rustify-ui/) | Public SDK: mounting, GPU regions, themes, routing, overlays, text, files, tasks and selection |
| [crates/rustify-components](crates/rustify-components/) | DOM components, forms, workspace controls and Tailwind styles |
| [crates/rustify-makepad](crates/rustify-makepad/) | Private Makepad integration and browser host |
| [makepad](makepad/) | Checked-in Makepad fork and wasm build tooling |
| [web](web/) | Instance loader and runtime styles |
| [xtask](xtask/) | Build, preview, diagnostics, stylesheet generation and verification commands |
| [tests/browser](tests/browser/) and [tests/vellum](tests/vellum/) | Browser regression and evidence checks |
| [docs/plan](docs/plan/), [docs/validation](docs/validation/) and [docs/reports](docs/reports/) | Development plans, validation records and release reports |

Read the [architecture](docs/architecture.md) for runtime ownership and lifecycle contracts, and the [component reference](docs/components.md) for supported controls. Application guides cover [navigation](docs/navigation.md), [workspace layout](docs/workspace.md), [forms](docs/forms.md), [localisation](docs/i18n.md), [large data](docs/data.md) and [full theme integration](docs/themes.md).

Third-party source provenance and file digests are recorded in [sources.lock.json](sources.lock.json). Included license notices cover [Makepad](makepad/LICENSE), its [fonts](makepad/widgets/resources/FONT-LICENSES.md), [Rust/UI component sources](crates/rustify-components/LICENSE-RUST-UI), [noUiSlider](examples/property-workbench/vendor/nouislider/LICENSE.md), [Vellum](examples/vellum/LICENSE-VELLUM) and [tweakcn theme data](examples/theme-studio/vendor/tweakcn/LICENSE).
