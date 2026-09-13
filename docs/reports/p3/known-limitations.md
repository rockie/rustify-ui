# P3 known limitations

Everything not built, not verified, or built in a way that should not be read
as general. A limitation is here whether it was a decision, a discovery or a
shortage of time, and which of those it was is said plainly. P1's and P2's
limitation lists still hold.

## Decided against, and not built

| Not provided | Why |
| --- | --- |
| A GPU-drawn table | ADR-9. Two of R20's three acceptance criteria are about identity and accessibility, and a GPU grid would need a DOM equivalent for every visible cell anyway, kept aligned to the CSS pixel. The fork's `DataGrid` stays a fallback, not a plan. |
| Region-level trap isolation | The boundary is the application instance (ADR-8). Two regions in one instance die together, as they always did. |
| Unbounded restarts | A dead instance's linear memory is never released (see [compatibility](compatibility.md)), so "restart as often as you like" means "grow without bound". Three per slot. |
| Threads, `SharedArrayBuffer`, WebGPU | A plain deployment is not cross-origin isolated (C-4). Sliced work on the main thread is the answer for this environment, not a stopgap for a better one. |
| Server-side paging, a formula engine, pivoting | Not this product. |
| A virtualised tree | Two fixed levels of ten by ten is what B2's navigation needs. A general tree is a different component. |
| A GPU text editor | Editing stays "the region gives up a rectangle, a native control takes it" (ADR-3). |
| Hot module replacement, incremental build timings | R24 AC3 / R37 AC2, deferred. The documents continue to describe a whole-page reload. |
| Migration samples, a five-developer study, a deprecation window | R28 AC3, R37 AC1, R38 AC3 — P4. |

## Built, with a boundary worth knowing

- **"B0", "B2", "B3", "B4" are names with definitions.** They are in
  `tests/browser/loads.ts` under the baseline identifier `p3-loads-1`, and a
  figure is only comparable to another measured against the same identifier.
  B2 and B3 are measured **separately**; this release does not claim they hold
  simultaneously.
- **The GPU figure is a ledger, not a driver reading.** It counts what the
  renderer asked for and nothing the driver adds around it. The table page
  holds under two kilobytes because the strip draws no text; the scene page,
  which draws ten thousand labels, holds 286,072.
- **The scene's objects are not in the accessibility tree.** What is provided
  is a complete alternative route — query, list, form — not an annotated
  canvas. See [accessibility](accessibility.md).
- **The sort is stable and byte-ordered.** `A–Z0–9` is the alphabet, but the
  order is the bytes': digits sort before letters. A test that expected
  otherwise was the test being wrong.
- **One machine, one browser.** Every figure in [performance](performance.md)
  is from a single macOS machine and a single Chrome. R29 asks for each
  performance machine and fully supported browser to pass on its own; one has.
- **Cold start is measured on the loopback.** It measures what the machine
  costs, not what a link costs. Throttled figures stay observations in
  `m8-network.spec.ts`, and are served uncompressed there, so they are not a
  reading of R29 AC2.

## Measured, and unresolved

- **B1's input latency has no explanation for its drift.** It rose across four
  milestones — 41.7, 44.9, 45.3, 46.3 ms — against a 50 ms gate, on a build
  whose action path did not change, and then fell to 45.5 ms on a tree that
  changed nothing at all. The likeliest reading is the machine rather than the
  build, and that reading is now supported by the fall, but it is a reading and
  not a finding. Margin is 4.5 ms.
- **A growth check reads the machine as well as the build.** The
  hundred-mount-round check failed once in this phase — in a sweep started
  immediately after the two-hour endurance run — and passed on a rerun of the
  identical tree. Endurance runs go last in a session.
- **Whether the working set is settled is a property of the build.** The
  unmount-residue check needs 300 warm-up rounds on this build and needed 150
  before; the endurance fixture needs 5,000. Both numbers are measured and
  commented where they are used, and both will need re-measuring when the
  caches or the allocator change.

## Not verified

- **A-4: VoiceOver over the two large loads.** **Waived by the user on
  2026-09-13** and not performed. Not a pass and not a failure — untested, and
  reported as waived by `cargo xtask verify --suite p3` on every run. This
  release claims a correct accessibility tree over both large loads and does
  **not** claim screen-reader support for them.
- **`document.hidden`.** Could not be made true in this headless browser by
  either method tried. The element-level hidden path is measured.
- **R30 AC2 over B3 in CI.** Deliberately: there is no GPU there to gate
  against. It is run by hand on headed Chrome and the figure is in
  [performance](performance.md).

## What a pass here is not

P3 closing does not mean the PRD is met. The full matrix, a WCAG 2.2 AA
review, the migration samples, the developer study, the deprecation window and
any public release are outside this phase. This release is a set of loads that
can fail a build, an instance boundary with a test behind it, and documents
that say which is which.
