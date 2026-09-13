# P3 functional report

What this release does, per requirement, and what it deliberately does not.
P1's runtime and P2's application layer are unchanged and are not restated;
everything here is what P3 added. Evidence is the milestone reports under
[`docs/validation/p3/`](../../validation/p3/).

## Data that is larger than the screen

A hundred thousand rows by twenty columns, generated in memory from a fixed
seed — no network, no data file, and a second implementation of the same
generator in TypeScript that every expectation is taken from (C-5).

| Delivered | What it means |
| --- | --- |
| A windowed table | About sixty rows and twelve columns exist as elements at any moment. Rows and columns are pools keyed by **slot**, not by identity; the identity lives on the slot as `aria-rowindex` and `data-row-id`. |
| Reaching any row | First, middle and last are reachable by the go-to-row entry, by `Ctrl+Home` / `Ctrl+End`, and by a query — and nothing is silently truncated. |
| Selection that survives | Selection is by business ID. Sorting and filtering keep it, deleting a row clears that row from it, and the count of selected-but-hidden is a question asked of the current view. |
| Editing a row you cannot see | Reach it from the keyboard, open its details, change a field, submit, scroll back: the cell holds what was typed. |
| A group tree | Two levels, arrow-key navigation, `aria-expanded` that changes when the group does, and choosing a group starts a filter. |
| A GPU overview strip | Beside the table, drawn by a region: it shows where the selection falls in a hundred thousand rows, and clicking it jumps the table there. |

## Work that takes longer than a frame

There is no worker and no thread — a plain deployment is not cross-origin
isolated, so there is no `SharedArrayBuffer` and nothing to move 32 MB into
(C-4). Sorting, filtering and finding are therefore **sliced jobs on the main
thread**.

- Each slice runs for an 8 ms time budget; a row count is only an upper bound.
- Yielding is a `MessageChannel` message, not a timer: chained `setTimeout(0)`
  is clamped to 4 ms from the fifth nesting level, which measured 716–743 ms
  for 150 hops against 0–4 ms for the message port.
- Every slice re-checks the data version before it reads. An insert, a delete
  or an edit mid-job invalidates it, and it re-runs once.
- Cancelling is a ticket becoming invalid. The old view does not move, and the
  answer is on screen well inside a tenth of a second.
- Only the newest job is the subject of the status line. Four jobs started in
  one turn used to leave the line saying "cancelled" while one was still
  running.

## A scene of ten thousand objects

Ten thousand rectangles with eight-character labels, at most two layers deep,
drawn by a region and culled to the viewport.

| Delivered | What it means |
| --- | --- |
| Real input paths | Press-and-drag on blank space pans; on a rectangle it marquees; a press-to-release move of 3 px or less is a click; the wheel pans; hover is a continuous stream. Whether a modifier means "add to" or "replace" is decided at the moment of the press. |
| Absolute camera | Continuous streams carry positions, never deltas: `Pace::Continuous` overwrites within a batch, so a delta stream loses movement. Ten wheel events in one task move the camera by their sum. |
| Picking and marquee | Both are computed against the layout formula rather than by walking ten thousand objects, and both are compared against the twin, which *does* walk all ten thousand — an optimisation written twice makes the same mistake twice. |
| A query entry | Find an object by label, select it, rename it — the same main action the pointer performs. |
| One selection, two views | The table and the scene share one selection by ID, and so does the strip. |

**A camera change is presented in the frame it happens.** Until M4 the scene
presented on every second frame: the props pump asked for the next frame from a
microtask, and a repeat request made before the previous callback had run was
dropped, so two asks landed on one frame and the third went by empty. The host
now fulfils that frame in place — 53 presentations for 54 drives, against 35
for 71 before.

## The smallest complete application, and the largest one left running

**B0** is ten DOM controls — all of them phase-one components — twenty GPU
controls in one region, two of each kind the SDK draws, and one shared state
behind both halves. It is what the fusion-basic page shows by default, and it
is the load the start-up and first-load budgets are about.

**B4** is two wasm instances with two regions each, booted once and left alive:
rounds mount and unmount scopes *inside* them rather than restarting them,
because a dead instance's memory never comes back and a run that restarted
would be measuring restarts.

## Idle, hidden and restored

| Requirement | Result |
| --- | --- |
| 60 s idle: at most one presentation | **0 frames**, 0.023% of the main thread against 0.011% for a blank page |
| Hidden: stops within 1 s, no backlog over 60 s | stopped inside 1 s, unchanged over 60 s |
| Restored within 500 ms, showing current state | **10.5 ms**, and no replay |
| 100 hide/restore rounds with no repeated action | **100 / 100** |

## What P3 deliberately does not do

- **No GPU table.** ADR-9 keeps B2 in the DOM: semantics, keyboard, IME, text
  selection and contrast then come from the browser rather than from a mirror
  that has to be kept aligned to the pixel. A GPU `DataGrid` exists in the fork
  and stays a fallback.
- **No region-level trap isolation.** The boundary is the instance (ADR-8).
- **No threads, no `SharedArrayBuffer`, no WebGPU.**
- **No hot module replacement, no server-side paging, no formula engine, no
  arbitrary dock tree, no virtualised tree, no GPU text editor.**
- **No new network egress.** The sample is generated in memory; the fourth
  example asks for nothing but its own build.
