# P3 compatibility report

Which combination is the pass gate, which is only observed, what the instance
model costs, and what is pinned. P1's and P2's statements still hold; this
report adds what P3 changed.

## The matrix

| Combination | Standing |
| --- | --- |
| macOS + Google Chrome (fixed version) | **The pass gate.** Everything claimed here was measured on it. |
| macOS + Safari | Observed. Not a gate; failures are recorded, not blocking. |
| Headless Chromium (SwiftShader) | What CI runs. It is the gate's environment for everything **except** R30 AC2 over B3 — see below. |
| Windows, Linux, mobile, NVDA, Windows IME | **Not tested and not claimed.** P4. |

## One gate does not run where the others do

R30 AC2 over B3 — the frame interval while ten thousand objects are panned —
runs on **headed Chrome only**, in its own project (`budget-scene`), and CI
does not run it.

This is a measurement, not a preference. M1's second probe drove the same scene
the same way under both:

| Environment | p95 effective frame interval | Share over 50 ms |
| --- | --- | --- |
| Headless Chromium, SwiftShader | 50.10 ms | 13.9% |
| Headed Chrome 152.0.7977.84 | 17.60 ms | 0% |

SwiftShader is a software rasteriser; R30's figures are about a machine with a
GPU. A failure under it would say nothing about the budget and a pass would say
nothing about the machine. The spec skips itself with that explanation if it is
ever started headless, rather than reporting a rasteriser as a red build.

B2's frame gate and B0's start-up gates run headless, because B2 is DOM work —
the rows are elements and the cells are text nodes — and start-up is dominated
by fetching and compiling a module.

## What an application instance costs

Each application instance is its own wasm instance: its own linear memory, its
own host hooks, its own diagnostics ring, its own router slot. Two instances on
a page cost roughly twice one.

**A restarted instance's predecessor is never released.** Every restart
evaluates a fresh copy of the generated glue at a new URL, and an ES module
record lives as long as the document — so the dead instance's linear memory
stays reachable until the page is unloaded. This is why restarts are capped at
**three** per slot: the cap is a bound on what a page can be made to hold
(at most 3 × one instance's linear memory, which P2 measured at about 39.6 MB
for B1 at start). The fourth attempt offers only "reload the page".

The alternative — appending a release entry point to the generated glue and
re-`init`ing at the same URL — was rejected: the glue's `CLOSURE_DTORS` is a
module-level `FinalizationRegistry`, so a dead instance's closures would run
their destructors against the new instance sharing that binding.

Fonts are still taken per region (P1's known cost), and the second evaluation
of the glue is one extra same-origin fetch that the HTTP cache answers.
`modulepreload` covers only the copy without a query string.

## The resource ledgers, and what they are not

| Ledger | What it counts | What it is not |
| --- | --- | --- |
| CPU | committed wasm linear memory + the JavaScript heap read through CDP `Runtime.getHeapUsage` | not `performance.memory`, which this browser quantises — it reported a round 10,000,000 where CDP reported 7,340,180. It is kept as corroboration only. |
| GPU | bytes this renderer asked the context to hold: buffer (re)allocations and the four texture entry points, by format, plus render targets | **an estimate, not a driver reading.** It does not include what the driver allocates around each object, nor anything the browser's compositor holds. The PRD permits an estimate; this is one. |
| Frames | the region's own presentation count, incremented where the host dispatches `FromWasmBeginRenderCanvas` | not animation-frame callbacks, which keep arriving at 60 Hz in front of a frozen picture |

A region's GPU figure returns to zero when it is destroyed, and that is
asserted rather than assumed.

## Pinned, and drifting on purpose

- **Leptos 0.8.20**, unmodified. The windowed table uses only public API:
  signals, `<For>`, `NodeRef`.
- **The Makepad fork is first-class source.** Its changes are ordinary commits.
  `cargo xtask sources verify` compares every frozen file against the import
  and reports differences as drift — which is the mechanism, not a failure:
  the lock records what was *imported*, so editing a digest to match a later
  edit would erase the only evidence that the edit happened. P3's fork change
  is `platform/src/os/web/web_gl.js`, which keeps the GPU byte ledger, and it
  is registered here and in `docs/compatibility.md` in prose.
- **`makepad-stitch` must stay at `opt-level = 1`.** Higher optimisation makes
  the script VM's wasm loops blow the stack after a few thousand iterations,
  and the abort names only whichever test reached it first.
- **`--cfg=web_sys_unstable_apis`** is set by `cargo xtask build-web`. A build
  made another way loses the clipboard and says so rather than failing.

## Deployment

Unchanged from P2 and re-verified: a build carries its base path and refuses to
be served from anywhere else, `serve --spa` answers route-shaped paths with the
document while leaving anything with a file extension a 404, and a region's
resources come from the deployment rather than from the current route. The
strict Content Security Policy is not relaxed — including on the two-instance
page, where a second copy of the glue is fetched with a query string and
`script-src 'self'` permits it.
