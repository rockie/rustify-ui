# P3 accessibility report

P3's accessibility question is narrower than P2's and harder: not "is this
control announced correctly" but **can a person reach something the browser has
not drawn**. Sixty row elements stand in for a hundred thousand rows, and ten
thousand objects are pixels in a canvas. Both are cases where the obvious
implementation produces an interface that looks right and cannot be used.

P2's results for the component catalogue still hold and are not repeated here.

## The table: sixty elements, a hundred thousand rows

| What is required | How it is provided | Checked by |
| --- | --- | --- |
| The grid is a grid | `role="grid"`, `aria-rowcount` of the whole sample, `aria-colcount` of every column | `p3-table.spec.ts` |
| A row says which row it is | `aria-rowindex` is the row's own number out of a hundred thousand, written on the slot currently showing it, alongside `data-row-id` | `p3-table.spec.ts` |
| One tab stop | The grid is a single tab stop with a roving focus inside it; the keyboard does not have to walk sixty rows to leave | `p3-table.spec.ts` |
| Any row is reachable | A go-to-row entry, `Ctrl+Home` / `Ctrl+End`, and a query. Rows 1, 50,000 and 100,000 are each reached, opened, changed and verified after scrolling back | `p3-table.spec.ts` |
| Sorting is announced | Each sortable header is a real `<button>` carrying `aria-sort` | `p3-table.spec.ts` |
| The tree opens and closes | `role="tree"`, arrow keys, and `aria-expanded` **read** from the group rather than captured when the row was built | `p3-table.spec.ts` |
| Twenty named targets | Twenty roles and names frozen in `tests/browser/loads.ts`, located 30 times each | `p3-table`, `p3-scene` |

**The bug this arrangement makes possible, and the check that catches it.**
Because rows are pooled by slot, `aria-expanded` and `aria-rowindex` are
properties of *what a slot is currently showing*, not of the element. Written
as a value captured when the element was made, they are correct on first render
and wrong forever after — the element is never rebuilt, so nothing updates
them. The tree shipped with exactly that defect until a check read the
attribute after collapsing a group.

The same shape appears in focus: which row a slot shows is a reactive property,
and it is applied *after* the effect that moves the keyboard. Focus therefore
has to be placed by finding the element for a slot, and must not be read back
from where it was just put.

## The scene: pixels with a query entry

A canvas has no accessibility tree to speak of — Makepad's web
`AccessibilityUpdate` branch is empty (P2 F22) — so the scene's accessible
surface is the DOM around it: a query entry that finds an object by label, a
selection list, and a details form that performs the same main action the
pointer does. R15 AC3's three targets (`OBJ00000`, `OBJ05000`, `OBJ09999`) are
each reached this way and renamed, and the automated check asserts that the
result is identical to the pointer path's.

This is a real limitation stated plainly: **the objects themselves are not in
the accessibility tree.** What is provided is a complete alternative route to
every action, not an annotated canvas.

## Text is data

A cell whose value looks like `<script>` is drawn as those characters. There is
no path from sample data into markup, and the strict Content Security Policy is
not relaxed anywhere in this release — including on the page that runs two wasm
instances, which reports zero violations.

## Contrast, zoom and reflow

Unchanged from P2 and re-run: contrast ratios over both themes are a host test,
the five journeys pass at 200%, and the catalogue reflows at 320 CSS pixels.
P3 adds no new colour tokens.

## What still needs a person

**A-4 is open.** The form is prepared at
[`docs/validation/p3/manual/voiceover-data.md`](../../validation/p3/manual/voiceover-data.md)
and has **not** been performed. `cargo xtask verify --suite p3` reports it as a
record that has not been handed in, and a blank form is never counted as a
pass.

What the automated checks establish is that the tree is correct: the row
indices are the rows' own, there is one tab stop, the query entries work, and
each target can be reached and changed from the keyboard. What they cannot
establish is whether **the speech makes a hundred thousand rows navigable
rather than merely announced** — whether a screen-reader user can tell where
they are in the sample, whether a job's progress arrives as one statement or as
a stream of interruptions, and whether the scene's query entry is discoverable
at all by someone who cannot see the canvas.

Until that walkthrough is done or explicitly waived, this release claims a
correct accessibility tree over both large loads and **does not** claim
screen-reader support for them.
