# P2 functional report

What this release does, and what it refuses to claim. Every row points at the
milestone report that carries the evidence; this document adds no result of its
own.

Scope: `crates/rustify-components`, the P2 additions to `crates/rustify-ui`, the
M4 and M6 changes to the `makepad/` fork, and the three examples. Build under
test: `cargo xtask build-web --release` for each example, verified by
`cargo xtask verify --suite p2`.

## The three examples

| Example | What it demonstrates | Evidence |
| --- | --- | --- |
| component-catalog | Eighteen component categories, each with a DOM example, a GPU column and a state matrix; the capability table; theme and language switches; the B5 text samples | [M1](../../validation/p2/m1.md), [M2](../../validation/p2/m2.md), [M7](../../validation/p2/m7.md) |
| property-workbench | B1: three panels with declared minimums, ten views over a thousand objects, twenty visible properties, a context menu opened inside the region, a command palette, deep links, cross-region dragging, import and export | [M3](../../validation/p2/m3.md), [M4](../../validation/p2/m4.md), [M5](../../validation/p2/m5.md), [M6](../../validation/p2/m6.md) |
| fusion-basic | Carried from P1, plus the navigation fixtures: two mount scopes, URL ownership, and a guard that can undo a refused move through history | [M4](../../validation/p2/m4.md) |

## What is delivered

| Area | Delivered | Evidence |
| --- | --- | --- |
| Component layer | Eighteen categories imported from Rust/UI and rewritten: every one controlled, `rui:`-prefixed, with no inline script or style; the four floating ones rebuilt on the SDK's overlay stack with role, keyboard, focus trapping and focus return | M2 |
| Capability catalogue | Eighteen rows × three columns × six capability classes, no blanks, generated into `docs/components.md` and checked for drift | M2 |
| GPU control subset | Button, radio, toggle, progress, spinner, icon stable; drop-down and tab bar experimental; scroll bars declared | M2 |
| Forms | A state machine holding errors, generations, pending checks and a submit request - and deliberately not the values; errors carry their source so re-running the rules cannot wipe an asynchronous failure | M3 |
| Routing | Static segments and `:param`, one URL owner per page, anchor interception inside a scope, and a guard that undoes a refused move through history by sequence number | M4 |
| Deployment | `build-web --base`, a build that carries the base it was made for, `serve --spa`, and a region that loads its resources from the deployment rather than from the route | M4 |
| Workspace | Three panels with application-declared minimums, ten views, a command palette that is itself a modal layer, and a context menu anchored inside the region | M5 |
| Dragging | One pointer session across both halves; a GPU target answers a question about a *point* and names itself; exactly one drop per drag | M6 |
| The wheel at an edge | A region reports where its scrolling ends after each draw, and the host decides synchronously whether the event is the page's | M6 |
| Clipboard and files | Copy and paste with a manual path on refusal; import with three answers and a fourth for "nobody chose anything"; text and binary export compared byte for byte | M6 |
| Two languages | The SDK's own words in English and `zh-CN`, `Intl` formatting with the application's options, and twenty B5 samples drawn by both halves | M7 |

## What is deliberately not delivered

| Not provided | Why |
| --- | --- |
| Nested routing, route-level authorisation, server redirects, form actions | Out of scope for one URL owner and a static matcher (ADR-6) |
| An arbitrary dock tree, layout in the URL, layout persistence | B1 asks for three panels with minimums |
| Dragging out to the operating system, pen pressure | Excluded by R-5 |
| Plural rules, interpolation, collation, a right-to-left interface language | Two languages of the SDK's own words is the claim |
| A performance budget | A-3 stands: baselines beside R29/R30, not a gate |

The full list, including the boundaries around things that *were* built, is in
[known-limitations.md](known-limitations.md).
