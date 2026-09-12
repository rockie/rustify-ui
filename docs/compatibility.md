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
| noUiSlider | `15.8.1`, MIT, vendored into `examples/property-workbench/vendor/nouislider/` (the minified module and stylesheet only) | `sources.lock.json` (`vendor`), digests checked by `cargo xtask sources verify`, presence by `cargo xtask doctor` |
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

All fonts received with Makepad stay in `makepad/widgets/resources`; licenses and notices are in `makepad/widgets/resources/FONT-LICENSES.md` (OFL 1.1 for all text fonts, GUST Font License for New Computer Modern Math). Both LXGW WenKai weights and Noto Color Emoji pass the readability and licensing gates, so no `--small-fonts` style substitution is used; that option was removed from the fork's build tool. Fonts are fetched by a region when it draws with the family that names them. On the web the default families cover Latin only, because the Chinese and emoji faces are 30 MB together; the theme also publishes `font_regular_i18n` and its bold/italic variants, which name all three. A region that has to draw a value beyond ASCII moves that label to the i18n family, and that is when those files are fetched: measured, a first load transfers 9,009,274 bytes of which 182,092 is one font, and the first value that needs more costs a fetch of `LXGWWenKaiRegular.ttf` and `NotoColorEmoji.ttf` together, 29,718,416 bytes. Until they arrive the region draws `.notdef` for those characters; the frame after they land draws the glyphs. The size report counts every shipped font file (51,187,844 bytes).

Which font draws what: the Latin UI and the region's own fixed labels use IBM Plex Sans Text; a value a region draws that needs more than ASCII - the name or notes of an object typed in Chinese, for instance - comes from LXGW WenKai Regular through the i18n family described above. Text being *edited* is drawn by the browser's own control, not by the region, so it uses the page's font stack rather than these. Each region fetches the fonts it needs independently, so two regions showing Chinese fetch the 19 MB file twice into linear memory; that is a recorded cost, not a leak, and is on the list to audit.

## Embedded region capabilities

| Area | State at the end of P1 |
| --- | --- |
| Drawing | WebGL2 into a host-owned `<canvas>`; DOM layout decides the rectangle; backing store follows `devicePixelRatio`; container size changes arrive through `ResizeObserver`. |
| Pointer input | Pointer events on the canvas with pointer capture; coordinates are canvas-local CSS pixels; wheel is delivered as scroll. No window-level listeners. |
| Keyboard, text input, IME | A region hands over the rectangle it drew text into, and a real `input`/`textarea` takes it for the life of the session, so the caret, the selection, the system's undo and the input method are the browser's. The fork's hidden textarea path stays disabled in embedded mode. |
| Cursor | Set on the canvas element only. |
| Document title, URL/history, fullscreen, open URL | Refused, and since M7 recorded once per capability per region as `UnsupportedCapability` with a next step. Note that every region asks for the document title as it starts (Makepad names its window), so that refusal is the first entry in every runtime's record. |
| XR, audio output, MIDI, geolocation, permissions, video playback | Present in the fork's JS but not part of the P1 contract; nothing in the examples triggers them, and nothing is claimed about them. |
| Network initiated by GPU code | Only the fork's resource loader uses it (fonts). Responses are routed to the region that made the request and dropped after that region is destroyed. |
| Lifecycle | Document visibility events reach every region; pagehide sends shutdown. |
| Teardown | Destroying a region removes listeners, observers, timers, animation frames, in-flight requests, GL objects (context released via `WEBGL_lose_context`) and then drops the Rust `Cx`. Verified for 20 mount/dispose rounds and for dispose-immediately-after-mount. |

## Third-party DOM components

P1 verifies exactly one third-party DOM component and claims nothing about
any other. noUiSlider 15.8.1 was chosen because it has real initialization and
teardown, so a rebuild can be inspected rather than assumed clean.

| Question | Answer |
| --- | --- |
| Where does it come from | The checkout. It is vendored with its licence and recorded with a digest per file; the build copies it and nothing fetches it at build or run time. |
| How is it reached from Rust | The page's own module imports it and exposes a small shim; the application creates and destroys it from a Leptos component (`examples/property-workbench/src/third_party.rs`). There is no SDK wrapper: an SDK that wrapped one library would be claiming a contract for all of them. |
| Who owns its lifetime | The application. The component is created when its element exists and destroyed on the scope's cleanup, together with the closure it subscribed with. |
| What is verified | Twenty rebuilds leave one instance in the document and one subscription; one change produces one callback; focus is not taken across a rebuild. See `docs/validation/p1/m6-components-async-theme.md`. |
| What is not supported | Anything requiring `eval`, an inline `<script>`, or a stylesheet the page cannot serve from its own origin: the release policy is `script-src 'self' 'wasm-unsafe-eval'; style-src 'self'`. A component that writes to `document.title`, the URL or history reaches the host page, not the scope - the SDK does not sandbox it. A component that installs window-level listeners is not isolated to a scope either. |
| What is not claimed | That other versions of this component, or any other library, work. Each one is the application's own integration until it is listed here. |

## Deployment

| Question | Answer |
| --- | --- |
| Root and sub-path | Both. The build uses relative URLs throughout, and `cargo xtask serve --base /tools/demo/` runs the same directory under a sub-path; the browser tests cover it as a third project. |
| Embedded in an existing page | Yes. A scope mounts into an element the page owns; the page's own headings, links, scrolling and text selection are untouched, and no theme or attribute is written to the document. |
| Outbound network | Only the build's own files, same-origin and under its base. Business network calls are the application's to make. |
| Content-Security-Policy | `default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; …`. Nothing in the shipped JS needs `unsafe-eval` or `unsafe-inline`; the theme is written through the CSSOM for that reason. |
| Assets from two builds | Refused at start. The bridge module carries the schema hash the wasm also exports; a mismatch stops the boot with `BuildContractMismatch` and a message saying to redeploy matching assets. |
| A missing, corrupt or truncated font | The runtime still starts and both halves keep working; what is lost is the glyphs the region cannot draw. The failure is **not** reported: the fork's own resource loader absorbs it, so `AssetLoadFailed` is registered as a class but has no producer in P1. |
| A lost WebGL context | Recoverable. The region reports `Lost`, is torn down, and is built again with the application's current state when the browser restores the context. Verified over twenty rounds with an edit between each. |
| A trapped wasm module | Every mount in that runtime is dead. The page's static notice says so and says that unsaved in-memory state is lost; only a reload brings it back. |

## Known limitations recorded at M1

- Each region owns a full Makepad `Cx` (script VM, theme, font atlas). Four regions occupied 123,994,112 bytes of wasm linear memory after start-up on the fusion-basic page; wasm memory never shrinks. Sharing font data across regions is open work.
- Makepad's UI/action signals are process-wide flags; one 16 ms poll per runtime runs while any region exists and broadcasts the signal to all regions. Converting this to push delivery is scheduled with M2.
- A trap affects every mount scope in the instance it happened in. Since P3 M5 that is the instance rather than the page: see "Instances and what they cost" below.
- Release wasm is 7,682,693 bytes uncompressed with `opt-level=z`; no size budget is claimed.
- The Leptos reference tree under `ref/` carries post-release changes in files unrelated to the SDK (see `sources.lock.json`); the crates.io release is what is built and tested.

## Instances and what they cost

One page can run several application instances, and each is a wasm instance
with its own linear memory. They share the compiled `WebAssembly.Module`, so a
second instance is one extra JS request for a copy of the generated glue under
a query string and no extra wasm request - but its memory is its own. A
fusion-basic instance starts at about 39.6 MB of linear memory
(`docs/reports/p2/performance.md`), so two of them cost about twice that.
Fonts are still taken per region, not per instance.

**A dead instance's memory does not come back.** Every restart evaluates a
fresh copy of the glue under a new URL, and an ES module record lives as long
as the document: the module that trapped, its binding to its wasm instance, and
that instance's linear memory are all reachable until the page is unloaded.
Collecting garbage does not reclaim them, and there is no release entry point
in the generated glue to call.

That is why restarting is bounded. A slot may be restarted **three times**;
after that the failure notice offers only a page reload. The worst case a page
has to be sized for is therefore about four times one instance's starting
memory - the three that died and the one running - and an application that
expects to fail more often than that should be reloading rather than
restarting.

## Routing

The SDK has its own minimal router (`docs/navigation.md`): static segments,
`:param`, one URL owner per page, and a guard that can undo a refused move
through history. It is not `leptos_router`, and it provides none of its
contexts - a third-party component that calls `leptos_router::hooks::*` will
panic for want of one. Nested routing, route-level authorisation, server
redirects and form actions are not provided and are not planned for this
release.

## The clipboard

`navigator.clipboard` is behind `--cfg=web_sys_unstable_apis` in web-sys
0.3.105, so whether the SDK can reach it at all is decided when the wasm is
compiled rather than by the browser it runs in. `cargo xtask build-web` sets
the flag (`xtask/src/build.rs`), and cargo-makepad composes inherited
`RUSTFLAGS` with its own, so every build made that way is clipboard-capable.

A build made another way is not, and this is not a crash: the two entry points
answer `ClipboardError::Unavailable`, and `rustify_ui::clipboard::available()`
says so before anything is attempted. An application should ask that once and
offer its manual path from the start rather than at the moment somebody presses
copy.

Two things the browser decides at run time, and neither is worked around:

- Reading the clipboard needs permission, and a page that has not been granted
  it gets a rejected promise. Every rejection arrives as `ClipboardError::Denied`,
  whatever the browser's reason - permission, an unfocused document, or a
  gesture the call was not part of - because the answer to all of them is the
  same: say so, and offer the path that needs no permission.
- A success is never reported that did not happen. The manual path is the text,
  selected, in a control the user can press the copy key in.

Programmatic reading is only for a deliberate "paste" control on something that
is not a text field. A text field's own paste is the browser's, needs no
permission, and is the path a person expects.

## Downloads under the strict policy

`default-src 'none'` governs what a page may load, not what it may hand the
user to save, and an export started from a `blob:` object URL and an anchor's
`download` attribute is not affected by it. No directive was added: the
committed strict policy is unchanged, and `p2-files.spec.ts` compares the bytes
the browser actually saved against the bytes the application meant to write, so
a browser that ever did block this would fail a test rather than lose a
feature quietly.

The object URL is revoked on the next turn rather than in the same one. A click
on an anchor starts the download asynchronously, and revoking immediately
cancels it in some browsers.

## Dragging

Dragging inside a page is the SDK's own pointer session (`rustify_ui::drag`),
not HTML5 drag and drop. Makepad's web backend has no part in the browser's DnD
protocol, so a GPU target could never be a drop target under it; the pointer
session works across both halves because the region answers a question about a
*point* and names the target itself.

HTML5 drag and drop is used for exactly one thing: files dragged in from the
operating system, which arrive as a `drop` event and nothing else can receive
them. Dragging *out* to the operating system is not provided.

## Languages

Two ship: English and `zh-CN` (`docs/i18n.md`). The SDK translates the words it
puts on screen on its own account and nothing else; an application's vocabulary
is the application's. `Locale::from_tag` matches on the language subtag, so
`zh`, `zh-CN` and `zh-Hans-CN` are one language.

Number and date formatting is `Intl`, with the options object supplied by the
application. Plural rules, message interpolation and locale-aware collation are
not provided.

Direction is a value the layout reads (`dir` on the scope root, logical
properties in the component classes), but no right-to-left language ships. The
B5 sample page draws right-to-left text so that a reviewer can see what the two
halves do with it. The theme's wide family covers Latin, Chinese and emoji and
does **not** include Arabic or Hebrew, so the region draws those samples with
whatever that font has for absent glyphs. That is a gap in the shipped fonts,
not in the layout, and it is what the reviewer's comparison against the browser
column is there to record.

The missing-glyph mark (`Message::MissingGlyph`) is for the other case: a font
that fails to arrive. The region then falls back to its built-in Latin face and
the page says so, because a row of boxes on its own tells a reader nothing about
whose fault it is.
