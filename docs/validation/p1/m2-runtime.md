# M2 report: embeddable and disposable runtime

Date: 2026-09-08. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Build: `cargo xtask build-web --example fusion-basic --release`. Tests: `npx playwright test m2-runtime` (`tests/browser/m2-runtime.spec.ts`), plus the M1 probes as regression.

## Status

M2 is **closed**. Every exit condition on the M2 row of the plan holds.

## Exit conditions

| Condition | Evidence |
| --- | --- |
| V1 ordinary behaviour: valid, missing and occupied containers | `a missing container fails and leaves the running scopes alone` (the throw names the container, the four live regions and the DOM counter are untouched); `mounting over a live scope fails and leaves that scope alone` (the scope keeps its count and keeps counting) |
| V1: twenty close-and-mount rounds; two scopes with two regions each | M1 probe 7, now with the failure log asserted empty every round |
| Two mounts × two regions, closing one leaves the rest operable | `closing one scope leaves the other fully operable, DOM and GPU` — after disposing scope a, a click inside scope b's first region raises its DOM count and repaints **both** of its regions |
| Old handles and late messages produce no callback | `a disposed handle is spent` (second dispose reports nothing to do); `events aimed at a disposed scope produce no callback and no error` (pointer, keyboard and a viewport change aimed at the removed region: no page error, the surviving scope's counter stays at 0); M1 probe 6 covers a dispose racing an in-flight message at 0/5/50 ms |
| 100 rounds of resource and host checks, no functional leak | `repeated mount and dispose returns every browser resource`: 250 rounds; regions, timers and animation frames return to the baseline, two canvases remain, the error log is empty, no page error, and wasm linear memory sampled at round 50 is unchanged 200 rounds later. The window is that wide on purpose — linear memory only grows in 16 MiB steps, so a shorter one could not resolve a small per-round leak. The scope then re-mounts and drives both DOM and GPU again |
| Host page keeps working | `the host page keeps its link, scrolling and text selection`; `an embedded region takes no page-level state` (focus stays on the body, no hidden textarea, title, hash and `body.style.overflow` untouched) |

## What M2 changed

- **Region state.** `GpuRegion` takes an optional `RwSignal<RegionState>` and publishes `Ready`, `Failed(UiError)` or `Disposed`. `fusion-basic` renders the application's own failure notice from it and reports every live region's state to the page.
- **A visible GPU failure.** A canvas that cannot provide a WebGL2 context leaves the region `Failed(GpuUnavailable)`, records one bounded host error and leaves the DOM half of that scope working. The test forces it by making `getContext("webgl2")` return null, so the path is exercised rather than assumed.
- **Signals are pushed, not polled.** `SignalToUI` gained a hook (`makepad/platform/network/src/ui_signal.rs`); raising a UI or action signal wakes the host, which reads the flags once per microtask and hands them to every live region. The 16 ms interval per runtime is gone.
- **Runtime counters.** The host reports regions, timers, animation frames, recorded errors, pumps and wasm linear memory, so teardown can be compared against the baseline it started from.

## Design decision: no public RegionHandle

The plan's §2 sketch named a `RegionHandle` alongside `AppHandle`. M2 does not ship one: a region's lifetime is exactly the lifetime of its `GpuRegion` component in the view, teardown runs from Leptos `on_cleanup`, and the application observes the result through `RegionState`. A second, independent way to close a region would give the region two owners for no case P1 has — rebuilding a region is expressed by changing the view. `RegionId` stays internal and is allocated monotonically, never reused, so it serves as the generation: messages and deferred work carrying a retired id are refused by the registry. Recorded in the plan under §5.1; to be revisited if M7's GPU-failure rebuild needs more than view structure.

## Measurements (baselines, no budget claimed)

| Measure | Value |
| --- | --- |
| Idle, four regions | 0 pumps in 3 s (was 1 pump/s under the 16 ms poll), 0 animation frames |
| wasm linear memory, mount/dispose rounds of a two-region scope | 125,173,760 B at round 10; one allocator step to about 142 MB in the first tens of rounds; then identical at every sample through round 250 |
| Browser resources after 250 rounds | 2 regions, 0 timers, 0 animation frames, 2 canvases, 0 recorded errors |
| Region denied a WebGL2 context | 2 recorded errors for the scope, both its regions `failed`, the other scope untouched |

The early memory step is a one-time allocator high-water mark, not growth per round: it does not repeat over the next 200. Where exactly it falls moves with unrelated changes, which is why the leak sample starts after it rather than at a fixed round. Linear memory never shrinks, so this is the honest shape to report.

## Not done in M2

- `RegionState::Suspended` — zero-sized and hidden regions belong to the geometry work in M4.
- Diagnostics are one bounded error list plus counters; the full diagnostic ring with the R39 limits is M7.
- Trap isolation between mounts remains out of scope (ADR-1/D9): a wasm trap ends the whole runtime and the host asks for a reload.
