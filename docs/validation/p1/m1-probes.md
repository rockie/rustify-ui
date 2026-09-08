# M1 report: fixed engineering and the three technical probes

Date: 2026-09-08. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Google Chrome 152.0.7977.77, Playwright 1.63.0 (bundled Chromium). Build: `cargo xtask build-web --example fusion-basic --release`, build id in `target/makepad-wasm-app/release/fusion-basic/build-manifest.json`.

## Status

M1 is **not closed**. Two exit conditions still fail, everything else on the M1 list holds. Details in the last section.

## Fixed engineering

| Item | Evidence |
| --- | --- |
| Makepad closure hard-forked into `makepad/`, import record with per-file SHA-256, exclusions and manifest edits | `sources.lock.json` (1455 files); commits `60828a7` (as received) and `0e5c1af` (trim); `cargo xtask sources verify` lists the files changed since import |
| Leptos locked to crates.io 0.8.20, `cargo vendor` diff against `ref/leptos-main` recorded | `sources.lock.json` → `leptos.reference_tree_diff`; the files behind facts F2/F3/F4 are identical, differences are in islands/forms/async-derived/stores/macro internals; no Leptos code is modified |
| Toolchain, wasm-bindgen crate + in-process CLI support, Playwright and Chrome versions pinned or reported | `rust-toolchain.toml`, `Cargo.lock`, `makepad/Cargo.lock`, `package-lock.json`; `cargo xtask doctor` passes all eight checks |
| Resource location under path members | `cargo tree` path dependencies resolve `makepad_widgets/resources` into the app dir (fonts served at `/makepad_widgets/resources/...`) |
| New checkout builds without `ref/` | The build reads only `makepad/`, crates.io and the repository; `ref/` is git-ignored and not referenced by any manifest |
| Licenses | `makepad/LICENSE` (MIT), `makepad/widgets/resources/FONT-LICENSES.md` |
| Entry points | `cargo xtask doctor|build-web|serve|sources verify`, `npm run test:browser`, `cargo test --workspace --lib`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check` (all pass), `.github/workflows/verify.yml` |

## Probe ①: Leptos CSR + Makepad in one wasm, no isolation

Passed. Browser probes 1–3 (`tests/browser/m1-probes.spec.ts`): the page boots with `crossOriginIsolated === false` and no `SharedArrayBuffer`; the region draws (lit pixels in the canvas capture); clicking the DOM button changes the GPU label (pixel diff of the canvas after the DOM count reads 1); clicking the GPU button changes the DOM count. The same build was opened in Chrome 152 and rendered "Count: 0" / "GPU +1" in every region.

## Probe ②: several `Cx` in one wasm, dispatch and symmetric release

Mostly passed; one exit condition open.

- Probes 4–6 pass: two mount scopes with two regions each keep separate state (incrementing scope b leaves scope a's pixels and DOM count unchanged); disposing scope a leaves scope b usable and `live_region_count` drops from 4 to 2 with no page errors; disposing 0/5/50 ms after mounting leaves no region and produces no late callback.
- Probe 7 (20 mount/dispose rounds) **fails intermittently**: around the fourth round one region's pump traps with `panicked at makepad/libs/wasm_bridge/src/to_wasm.rs:181: index out of bounds: the len is 10 but the index is 10` (a ToWasm batch shorter than a block claims), after which that region's `Cx` is never returned to the registry and the count stays one too high. With extra JS instrumentation (changed timing) 12 rounds passed, so the corruption is timing dependent. Not yet root-caused; candidates are noted in the plan's progress section.

F15 global-state checklist, as implemented and verified in the browser:

| Global | Decision | Evidence |
| --- | --- | --- |
| `wasm._bridge` (JS) | removed; `env` imports decode from memory directly, network imports route to the bridge currently pumping (`WasmBridge.current`) | `makepad/libs/wasm_bridge/src/wasm_bridge.js` |
| typed-array views of memory (JS) | re-validated on every access; the previous cache-after-own-call scheme silently lost writes once another region or Leptos grew memory (first three of four regions never received `ToWasmInit`) | same file; probes 4–6 |
| `wasm_check_signal` / UI+action flags (Rust) | stay process-wide; one 16 ms poll per runtime while regions exist, broadcast to all regions | `crates/rustify-makepad/web/embedded.js` |
| `ACTION_SENDER_GLOBAL` | `Cx::post_action` now delivers to the `Cx` whose pump is running; the global only serves calls outside any pump | `makepad/platform/src/action.rs`, `cx.rs`, `web.rs` |
| studio websocket thread sender / `init_websockets` | never called for regions | `crates/rustify-makepad/src/wasm/host.rs` |
| network shim backend | process-wide by design; responses are routed per request to the runtime that issued them, and dropped by destroyed hosts | `makepad/platform/src/os/web/web_network.rs`, `web.js` |
| `live_id` interner, widget uid counter | shared, idempotent | unchanged |
| `gpu_texture` / `shader_error` / `devtools` slots | not reached on the web backend in P1 | unchanged; to re-check when M7 touches GPU failure paths |

Other multi-`Cx` fixes made in the fork: the web backend indexed window 0 before the widget tree created it (trapped at `ToWasmInit` for every region); `clear_memory_refs` nulled the shared instance's memory; `ResizeObserver` replaces window resize listeners; all listeners are registered with an `AbortController`; timers, animation frames, in-flight fetches and GL objects are released on destroy.

## Probe ③: static message bridge under a strict CSP

Passed. Probe 8: the page runs under `default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; …` with no CSP report in the console; `rustify_makepad/message_bridge.js` is generated at build time by running the wasm's `wasm_js_message_bridge_source` export in `wasmi` with every import stubbed, its `SCHEMA_HASH` equals `wasm_js_message_bridge_hash()` and the manifest; the shipped `wasm_bridge.js` contains neither `new Function` nor `eval(`. A-4 is therefore resolved for the current message set.

## Measurements recorded (baselines, no budget claimed)

| Measure | Value |
| --- | --- |
| Release output | wasm 7,682,693 B; js 158,766 B; css 714 B; fonts 51,187,844 B (all shipped, fetched lazily); images 20,701 B; data 8,219 B |
| Page ready (headless Chromium, 4 regions) | 3.5 s from navigation to `status=ready` |
| wasm linear memory after start-up, 4 regions | 123,994,112 B; stable across 6 mount/dispose rounds |
| Idle | 1 pump/s, 0 animation frames/s with 4 idle regions |
| Region canvas screenshot (headless) | ≈70 ms |

## Assumptions registered

- A-2 (build combination): resolved by probe ①.
- A-3 (multiple `Cx`): resolved for dispatch and release except the intermittent trap above.
- A-4 (static bridge/CSP): resolved.
- A-5: Chrome 152 present; availability of macOS 拼音 and VoiceOver test time is not yet confirmed by the user; Chinese fonts distributable (OFL).
- A-6: baseline metrics above are the M1 measurement contract seed; the M8 method (release build, headless and Chrome, 30 start-ups, ≥1000 interaction samples) still needs the user's confirmation.
