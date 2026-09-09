# Compatibility and capability boundaries (P1 research preview)

Everything below describes what this repository has actually built and
verified. Items marked *not verified* or *unsupported* are not claims of
future behaviour.

## Fixed versions

| Component | Version | Where it is pinned |
| --- | --- | --- |
| Rust toolchain | `nightly-2026-05-20` (rustc 1.97.0-nightly), `rust-src`, target `wasm32-unknown-unknown`; the wasm build uses `-Z build-std=panic_abort,std` with a generated single-threaded target spec | `rust-toolchain.toml`; read by `makepad/tools/cargo_makepad` |
| Leptos | `0.8.20` from crates.io, feature `csr` only; no Leptos source is modified | `Cargo.toml` (`=0.8.20`), `Cargo.lock`, `sources.lock.json` |
| wasm-bindgen | crate `0.2.128` and `wasm-bindgen-cli-support` `0.2.128` (run in-process by cargo-makepad; a version mismatch fails the build) | `Cargo.lock`, `makepad/Cargo.lock`; checked by `cargo xtask doctor` |
| Makepad | hard fork in `makepad/`, declared version 2.0.0, upstream commit unknown; never synced with upstream again | `sources.lock.json` |
| Playwright | `@playwright/test 1.63.0` with its bundled Chromium (continuous regression only) | `package.json`, `package-lock.json` |
| Node | 26.1.0 (browser tests only; the build does not need Node) | reported by `cargo xtask doctor` |
| Pass-gate browser | Google Chrome 152.0.7977.77 on macOS (Darwin 25.6) | reported by `cargo xtask doctor` |

## Browser matrix

| Environment | Status |
| --- | --- |
| macOS Chrome 152 | P1 pass gate. M1 probes run automatically in Playwright Chromium; the same build was opened in Chrome 152 (regions render, counters update). Manual Chrome records for later milestones live under `docs/validation/p1/`. |
| macOS Safari | Observation only (recorded at M8, never blocking). Not exercised yet. |
| Windows, Linux, mobile | Not tested; no support claimed. |

## Deployment contract

- Plain static hosting. The page is not cross-origin isolated (`crossOriginIsolated === false`, no `SharedArrayBuffer`); verified by the M1 probes.
- Scripts are external ES modules. The release Content Security Policy served by `cargo xtask serve --csp strict` is  
  `default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'`.  
  `'wasm-unsafe-eval'` is required for WebAssembly; JavaScript `'unsafe-eval'` is not, because the message bridge is generated at build time (`rustify_makepad/message_bridge.js`) and no shipped JS uses `new Function` or `eval`.
- The loader refuses to drive a wasm whose exported bridge fingerprint differs from the shipped bridge module (`BuildContractMismatch`); `build-manifest.json` records the same fingerprint and the build id.
- Nothing is sent anywhere by default: the fork's browser issue reporter only logs to the console unless the host page installs `window.makepad_report_browser_issue`.
- Assets resolve relative to the loader module URL, so root, sub-path and embedded pages work with the same build (sub-path served via `--base`). Not verified yet: deep links and history behaviour (out of scope until R22).

## Fonts

All fonts received with Makepad stay in `makepad/widgets/resources`; licenses and notices are in `makepad/widgets/resources/FONT-LICENSES.md` (OFL 1.1 for all text fonts, GUST Font License for New Computer Modern Math). Both LXGW WenKai weights and Noto Color Emoji pass the readability and licensing gates, so no `--small-fonts` style substitution is used; that option was removed from the fork's build tool. Fonts are fetched lazily by each region when a glyph needs them: a fresh region fetches only `IBMPlexSans-Text.ttf` (181,792 bytes) for the Latin UI; the Chinese font (19,073,964 bytes regular) is fetched on first CJK use. The size report counts every shipped font file (51,187,844 bytes); transfer measurements per journey are an M8 deliverable.

Which font draws what, as of M5: the Latin UI and the region's own labels use IBM Plex Sans Text; any CJK glyph the region draws - the name and notes of an object typed in Chinese, for instance - comes from LXGW WenKai Regular, fetched on first use. Text being *edited* is drawn by the browser's own control, not by the region, so it uses the page's font stack rather than these. Each region fetches the fonts it needs independently, so two regions showing Chinese fetch the 19 MB file twice into linear memory; that is a recorded cost, not a leak, and is on the list to audit.

## Embedded region capabilities

| Area | State in M1 |
| --- | --- |
| Drawing | WebGL2 into a host-owned `<canvas>`; DOM layout decides the rectangle; backing store follows `devicePixelRatio`; container size changes arrive through `ResizeObserver`. |
| Pointer input | Pointer events on the canvas with pointer capture; coordinates are canvas-local CSS pixels; wheel is delivered as scroll. No window-level listeners. |
| Keyboard, text input, IME | Not wired for regions yet (M4/M5). The fork's hidden textarea path is disabled in embedded mode. |
| Cursor | Set on the canvas element only. |
| Document title, URL/history, fullscreen, open URL | Refused; logged once as an unsupported capability. |
| XR, audio output, MIDI, geolocation, permissions, video playback | Present in the fork's JS but not part of the P1 contract; nothing in the examples triggers them. Explicit refusal is scheduled for M7. |
| Network initiated by GPU code | Only the fork's resource loader uses it (fonts). Responses are routed to the region that made the request and dropped after that region is destroyed. |
| Lifecycle | Document visibility events reach every region; pagehide sends shutdown. |
| Teardown | Destroying a region removes listeners, observers, timers, animation frames, in-flight requests, GL objects (context released via `WEBGL_lose_context`) and then drops the Rust `Cx`. Verified for 20 mount/dispose rounds and for dispose-immediately-after-mount. |

## Known limitations recorded at M1

- Each region owns a full Makepad `Cx` (script VM, theme, font atlas). Four regions occupied 123,994,112 bytes of wasm linear memory after start-up on the fusion-basic page; wasm memory never shrinks. Sharing font data across regions is open work.
- Makepad's UI/action signals are process-wide flags; one 16 ms poll per runtime runs while any region exists and broadcasts the signal to all regions. Converting this to push delivery is scheduled with M2.
- A trap in the shared wasm affects every mount scope on the page (ADR-1, D9). The static failure notice exists in the loader; the runtime fatal path is M7 work.
- Release wasm is 7,682,693 bytes uncompressed with `opt-level=z`; no size budget is claimed.
- The Leptos reference tree under `ref/` carries post-release changes in files unrelated to the SDK (see `sources.lock.json`); the crates.io release is what is built and tested.
