# P1 functional report

What the research preview does, and what it refuses to claim. Every row points
at the milestone report that carries the evidence; this document adds no result
of its own.

Scope: `crates/rustify-ui`, `crates/rustify-makepad`, the `makepad/` fork, and
the two examples. Build under test: `cargo xtask build-web --release` for both
examples, verified by `cargo xtask verify --suite p1`.

## The two examples

| Example | What it demonstrates | Evidence |
| --- | --- | --- |
| fusion-basic | Two mount scopes, each with a DOM counter and two GPU regions on one signal; a geometry fixture of twenty anchors in two nested scrollers; a menu anchored to a GPU anchor and a modal over the region | [M1](../../validation/p1/m1-probes.md), [M2](../../validation/p1/m2-runtime.md), [M4](../../validation/p1/m4-geometry.md), [M6](../../validation/p1/m6-components-async-theme.md) |
| property-workbench | A thousand objects with stable ids, a DOM property panel, a GPU view, native text sessions over the region's own rectangles, the component subset, the theme, asynchronous state, one vendored third-party component, and the capability catalogue on the page | [M3](../../validation/p1/m3-state.md), [M5](../../validation/p1/m5-text-and-semantics.md), [M6](../../validation/p1/m6-components-async-theme.md), [M7](../../validation/p1/m7-deployment-and-recovery.md) |

## What is delivered, by requirement

| PRD | Delivered in P1 | Evidence |
| --- | --- | --- |
| R01 mount, remount, bad container | `mount` returns a handle; a missing or occupied container fails and leaves running scopes alone; twenty close-and-mount rounds | M2 |
| R02 one authoritative state | 1,000 objects with stable ids, 10,000 sequenced GPU actions arriving once each in order, 100 objects changed in one update | M3 |
| R03 stable ids | Reordering keeps the selection on the object; adding and removing keeps every other id; a list naming one object twice is refused rather than guessed at | M3 |
| R04 mixed input | 240 pointer moves at 120 Hz interleaved with 20 clicks and 20 saves: 20 discrete actions accepted, none lost or duplicated | M3 |
| R05 two scopes, late messages | Two scopes × two regions; closing one leaves the other operable; disposed handles and late messages produce no callback | M2 |
| R06 four states | `Load` four states, a hundred pairs answered backwards ending on the newer answer, a hundred tickets after a close delivering nothing | M6 |
| R07 host coexistence | The host page keeps its link, scrolling and text selection; a scope writes no page-level state; one fixed-version third-party DOM component rebuilt twenty times | M2, M6 |
| R08 custom GPU controls | A selectable object grid, a GPU checkbox and slider, both strictly controlled; the six capability classes in the catalogue | M3, M6 |
| R09 geometry | Three viewports × three zooms × twenty anchors within 1 CSS px for both display and hit; a hundred resize-and-scroll rounds with nothing outside the clip; back from 0×0 on the first frame; the backing store follows the device pixel ratio | M4 |
| R10 overlays | A DOM menu anchored to a GPU anchor within 1 CSS px and not occluded; a modal taking a hundred clicks and passing none down; two layers where Escape closes the top only and focus returns | M4 |
| R11 disabled and read-only | A disabled control changes nothing and takes no keyboard; a read-only control is reachable and unchanged by typing; a business rule makes a field read-only in both halves | M4, M6 |
| R12 focus | Twenty Tab / Shift+Tab stops with the disabled item absent; the application's commands do not take Escape from an open layer; no focus is taken from the host page | M4 |
| R13 text editing | The region hands over the rectangle it drew and a real control takes it; Enter commits once, Escape abandons, keys during composition belong to the composition, one composition produces one value, an external change ends the session rather than being overwritten | M5 |
| R15 / R25 semantics | Twenty controls found by role and name, none twice; the canvas is `aria-hidden`; the DOM path and the pointer path reach the same object; a lookup answers within five seconds; five keyboard-only journeys | M5 |
| R16 theme | One token table per scope written through the CSSOM; twenty switches leave the host page untouched; a local override writes only what it names and can be turned off | M6 |
| R18 catalogue | Eighteen categories, three presentation columns and six capability classes each, no blank cell, nine categories shipped and nine saying `no` with a reason | M6 |
| R19 submit | A refused value is replaced by the value in force with a reason; a failure offers the application's own retry | M6 |
| R23 / R26 / R27 deployment | Root, sub-path and embedded pages from one build; assets from two builds refused at start; a missing, corrupt or truncated font costs glyphs rather than the application; a lost GPU context is recovered with the application's current state | M7 |
| R24 / R39 diagnostics | Ten registered failure classes each with a next step, in a record bounded by 1,000 entries and 4 MiB, leading with what it dropped; entry details are `&'static str`, so the user's text has no path into a log | M7 |
| R36 security | The release policy needs no `unsafe-inline` and no JavaScript `unsafe-eval`; the message bridge is generated at build time; text that looks like markup is a value | M1, M7 |
| R40 traceability | Import records with a digest per file, two clean builds producing identical manifests, and this set of reports | M1, M8 |

## What P1 does not do

These are scope decisions recorded in the plan, not defects:

- Nine of R18's eighteen component categories are absent (link, icon, radio, switch, select, progress, tooltip, tabs, scroll area). The catalogue says so in every column.
- No routing, deep links or history (R22), no workspace panels or tabs (R21), no large-data list, tree or table (R20), no general cross-region drag, clipboard or file import (R11 in full, R14).
- No GPU text editor: a region displays text and hands editing to a native control over the rectangle it drew.
- No trap isolation between mount scopes. A trap in the shared wasm module ends every scope on the page; the page says so and asks for a reload.
- No performance budget is claimed. [The performance report](performance.md) carries baselines only.

## Defects found and fixed during P1

Kept here because each one changed a contract rather than a line:

- A multi-block message from JS moved the Rust read cursor to the wrong offset, so any batch of two or more blocks trapped. Single-block messages hid it upstream; embedding produces batches. Fixed in the fork and pinned by unit tests (M1).
- Typed-array views of wasm memory were cached after a call, so a region's writes were silently lost once another region or Leptos grew memory. Views are re-validated on every access (M1).
- Page-level seams were single slots: a second scope took the slot and its teardown left the page with nothing while the first scope was still mounted. Each scope now holds its own entry and withdraws only that one (M5, M6).
- A widget reported the rectangle the layout walked rather than the one it was aligned into, so a pointer aimed at the reported position missed by the width of the centring (M6).
- A region drew a notes value it never updated: the helper that would have updated it was dead code (M6).
- Recovering a lost GPU context by replacing the canvas element left the rebuilt region compiling shaders against the dead context, so it came back blank — worse than staying visibly lost. The platform's own restore event is what the region now waits for (M7).
- An embedded region never collected its script heap, so every props application that set a value on a shader leaked three objects. Two hours at ten actions a second cost 145 MB. A region now sweeps between pumps once it has made enough garbage to be worth it (M8).
