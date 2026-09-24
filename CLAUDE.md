# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Rustify UI is an experimental SDK that runs Leptos (CSR, DOM) and Makepad (GPU, WebGL2) in one browser page and one wasm module. Leptos owns the DOM and all application state; each `GpuRegion` owns a Makepad `Cx` that draws into a canvas the DOM laid out. The two never share widgets or state: the application projects state into a region as props and receives typed actions back. `docs/architecture.md` (runtime contracts) and `docs/quickstart.md` are the entry points.

## Toolchain

- `mise install` provides everything `mise.toml` pins: stable Rust 1.98.1 with the `wasm32-unknown-unknown` target, rustfmt and clippy, and mbx 1.15.0. There is no nightly, no `rust-toolchain.toml` and no `build-std`.
- Every build goes through **mbx** (the compiler cache): `mbx build`, `mbx test`, `mbx clippy`, `mbx xtask ...`. Plain `cargo` is wrapped by mise too, but anything that launches a *nested* build must call `mbx` explicitly, because `mbx run` gives its child a plain `$CARGO`.
- mise's postinstall runs `scripts/link-libllvm.sh`. mbx clears `DYLD_*`, so without that link rust-lld cannot load libLLVM. `mbx xtask doctor` reports toolchain state and never changes it.
- Browser tests need Node 26: `npm ci && npx playwright install chromium`.

## Commands

```sh
# Host checks (CI's `host` job)
cargo fmt --all -- --check                     # makepad/ opts out via its own rustfmt.toml
mbx test --workspace --lib
mbx test -p rustify-ui --lib router::          # one crate, filtered
mbx clippy --workspace --all-targets -- -D warnings
(cd makepad && mbx test)                       # the fork is its own workspace
(cd makepad && mbx test --manifest-path platform/script/Cargo.toml)
mbx xtask sources verify                       # makepad/ drift against sources.lock.json
mbx xtask css --check                          # committed stylesheet matches the class strings
mbx xtask catalog --write docs/components.md --check

# Build and serve an example; output is target/makepad-wasm-app/<profile>/<example>/
mbx xtask build-web --example fusion-basic --release
mbx xtask serve --example fusion-basic --release   # strict CSP; also --base, --spa, --port, --csp, --fault
mbx xtask report-size --example fusion-basic --release --compressed
```

### Browser tests

Playwright runs against **release builds that already exist**. `serve` does not rebuild, so after a Rust change, run `build-web --release` again for the affected example. Playwright starts every `webServer` in `playwright.config.ts` whatever `--project` you pass. All four root builds (fusion-basic, property-workbench, component-catalog, data-workbench) must therefore exist; the two sub-path servers build their own `<example>@tools-demo` copies.

```sh
npx playwright test --project=property-workbench p3-faults.spec.ts   # one spec: iterate this way
npx playwright test --project=fusion-basic                            # one whole project
npx playwright test -c tests/vellum/playwright.config.ts m2-scene.spec.ts
VELLUM_ARTIFACT_SCOPE=m4-pointer npx playwright test -c tests/vellum/playwright.config.ts m4-pointer.spec.ts
```

- `playwright.config.ts` maps specs to projects. There is one project per example, plus budget and deployment projects, and a spec runs only under the projects that list it.
- Never run without `--project`, because the whole suite gets killed for memory. Run one project per command, and never start two Playwright runs at once: each clears `test-results/` at startup, and the other run then fails with phantom ENOENT errors.
- Whole projects take tens of minutes (fusion-basic ~47 min, property-workbench ~38 min), so iterate on single spec files. A full sweep is evidence for a milestone's exit, not a debugging tool.
- `budget-scene` needs headed Chrome on a real GPU. CI leaves it out on purpose, and it skips itself when headless.
- Performance figures are measured inside the page, never from Playwright timings (the harness adds seconds per navigation).
- `mbx xtask verify --suite p1|p2|p3` runs a release's full check set. It lists the manual records (VoiceOver, real IME input, zoom walkthrough) as *missing*, never as passed.

## Architecture

The workspace excludes `makepad/` (a fork with its own workspace) and `ref/`.

- `crates/rustify-ui` is the public SDK.
- `crates/rustify-components` holds the DOM component catalogue of twenty categories:
  - The `clx!` / `variants!` class macros, forked from Rust/UI.
  - `CATALOG`, from which `docs/components.md` is generated.
  - The Tailwind input and the **committed** output under `css/`.
- `crates/rustify-makepad` is the private Makepad integration:
  - The `RegionApp` trait and a region registry with never-reused `RegionId`s.
  - The pump entry points exported to JS, deferred props and the action outbox.
  - `web/embedded.js`, the JS host for a region's canvas and GL.
- `makepad/` is a hard fork of the Makepad wasm closure, trimmed to the browser backend. It is first-class source: edit it directly, and `sources verify` reports the edits as attributable drift. The embedding changes are in `platform/src/os/web/*`, `platform/src/{action,cx}.rs`, `libs/wasm_bridge` and `tools/cargo_makepad`, which is reduced to the single-threaded browser build.
- `web/loader.js` and `web/runtime.css` are the page boot. `boot()` instantiates one wasm instance, checks the bridge fingerprint, installs host hooks and owns the static failure notice.
- `examples/*` each have `src/` (the Rust app), `index.html`, and an `app.js` page script. The script calls `boot()` and exposes a `window.__<example>` handle that the Playwright specs drive.
- `xtask build-web` runs the fork's `cargo-makepad` through `mbx`, pulls the generated message-bridge JS out of the wasm with `wasmi` and ships it as a static ES module, then writes `build-manifest.json`. There is no runtime codegen, which is what keeps the strict CSP.

Runtime contracts to know before touching the runtime (the full list is in `docs/architecture.md`):

- **Instance > scope > region.**
  - Each `boot()` is its own wasm instance, with its own linear memory, host hooks and diagnostics; all instances share one compiled `WebAssembly.Module`.
  - A trap kills only its own instance. The loader then aborts that instance's page-level listeners, clears its `data-rustify-url-owner` mark, drops its tasks, destroys its regions, and only then shows the notice.
  - A slot restarts at most three times.
  - Within one instance, `mount()` scopes live and die together.
- **Page-level listeners** go through `rustify_makepad::listener_options` so they carry the instance's abort signal: `panic = "abort"` runs no destructors.
- **Pump.** JS calls `rustify_region_process(region, msg)`, and the region's `Cx` is taken out of the registry for the whole pump. Deferred `apply` closures run first, so a re-entrant `apply` only queues. Actions reach the application after the pump returns.
- **Controlled values.** A control never holds the value it shows. Each input is a request, and the control is put back in step with whatever the application decided.
- **Action streams.** `Pace::Continuous("<name>")` states supersede only their own stream. Discrete events are never merged or reordered.
- **Decisions the host must make synchronously**, such as a wheel's `preventDefault`, are answered from the region's report of its last draw. The answer is one event old by design.
- **Target split.**
  - Many modules and re-exports are `#[cfg(target_arch = "wasm32")]`, so host `--lib` tests cover only target-independent logic, and the Playwright specs cover browser behaviour.
  - The clipboard needs `--cfg=web_sys_unstable_apis`, which `build-web` sets. Without it, `clipboard::available()` is false.

## Conventions

- **Component classes** are Tailwind utilities with a `rui:` prefix. After changing a class string, run `mbx xtask css` and commit `crates/rustify-components/css/rustify.css`; otherwise the class has no rule and CI's `--check` fails. Likewise, regenerate `docs/components.md` after changing the catalogue.
- **Scripted edits.** `cargo fmt` reflows lines, so a scripted find/replace on Rust source must assert that the old text was found.
- **Plans.**
  - Development plans live in `docs/plan/*.md` and are written in Chinese. Milestone evidence goes in `docs/validation/`, and release reports go in `docs/reports/`.
  - A plan's 「实施进度 → 恢复快照」 section is the only resume point between conversations. Rewrite it as soon as a milestone's exit checks pass, before reporting completion or committing.
  - Source code carries no requirement, plan or milestone IDs. Traceability lives in the plan and in commit messages.
- **`ref/`** is gitignored and local-only: upstream Leptos, Makepad, Rust/UI and Vellum trees. Use it for reading, and as the twin reference server (port 4180) for the Vellum visual and interaction comparisons.
- **缺服务、缺容器就停下来问，不要擅自装。** 如果遇到本地开发环境缺少依赖的服务或容器，停下来要求用户协助或确认是否安装环境依赖，不要擅自下载镜像启动新的容器。
- **不新开分支**：改动直接提交在当前分支（通常是 `main`），除非用户明确要求开分支。
