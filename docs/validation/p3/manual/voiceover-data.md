# Manual record: VoiceOver over a hundred thousand rows and ten thousand objects

STATUS: WAIVED

Waived by the user on 2026-09-13. Asked to choose between performing the
walkthrough and waiving it, the user answered "豁免" (waive). The seven steps
below were **not performed**. Nothing here is recorded as a pass, and this
release does not claim screen-reader support for B2 or B3.

This follows the precedent set for P2's remaining VoiceOver rows
(`docs/validation/p2/manual/voiceover.md`, waived 2026-09-11). The full WCAG
2.2 AA review and the wider assistive-technology matrix were already outside
P3's scope (plan §0.6) and belong to P4; this waiver does not enlarge that
exclusion, and it does not waive anything the automated checks cover.

| | |
| --- | --- |
| Date | 2026-09-13 |
| Device | — (not performed) |
| OS | — |
| Browser and version | — |
| Assistive technology and version | — |
| Build id | — |
| Performed by | Nobody; waived by the user |

## What the automated checks do establish

These are not a substitute for the walkthrough. They are the reason the waiver
is a bounded one rather than a blank space, and they are asserted on every run
of `p3-table.spec.ts` and `p3-scene.spec.ts`:

- The grid is a grid: `role="grid"`, `aria-rowcount` over the whole sample,
  `aria-colcount` over every column.
- **A row announces its own number out of a hundred thousand.** Sixty pooled
  elements stand in for the sample, and `aria-rowindex` is a property of what a
  slot is currently showing rather than a value captured when the element was
  made. The tree shipped with exactly that defect until a check read the
  attribute after a group was collapsed.
- One tab stop per grid, with a roving focus inside it, and no focus trap.
- Rows 1, 50,000 and 100,000 are each reached from the keyboard alone, opened,
  changed, and verified after scrolling back.
- Each sortable column header is a real `<button>` carrying `aria-sort`.
- The group tree reads `aria-expanded` from the group rather than capturing it.
- `OBJ00000`, `OBJ05000` and `OBJ09999` are each found through the scene's
  query entry and renamed, with the same result as the pointer path.
- Twenty named targets, frozen in `tests/browser/loads.ts`, located 30 times
  each across both views.

## What is therefore untested

The judgement the walkthrough exists to make:

- Whether the speech makes a hundred thousand rows **navigable** rather than
  merely announced — whether a screen-reader user can tell where they are in
  the sample.
- Whether a job's progress over the whole sample arrives as one statement or as
  a stream of interruptions.
- Whether the scene's query entry is discoverable at all by someone who cannot
  see the canvas. The scene's objects are **not** in the accessibility tree
  (Makepad's web `AccessibilityUpdate` branch is empty); what is provided is a
  complete alternative route, and whether that route is findable is exactly
  what nobody has checked.

## If this is ever performed

The steps are unchanged and are kept here so the waiver can be lifted without
rewriting the form. Serve the example:

    cargo xtask serve --example data-workbench --release --port 4178 --spa

1. Reach rows 1, 50,000 and 100,000 from the keyboard, through the go-to-row
   entry and through `Ctrl+Home` / `Ctrl+End`. Note the announced position.
2. Open the details of the row reached, change a field, submit. Note what is
   announced for the field, for a refused value, and for the save.
3. Scroll away and back. Note whether the announced value is the one typed.
4. Sort by a column, then filter. Note the header's sort state, the job's
   progress, and a cancel.
5. Move through the group tree with the arrow keys. Note expand, collapse, and
   what choosing a group announces.
6. Find each of `OBJ00000`, `OBJ05000`, `OBJ09999` through the query entry and
   perform the pointer's main action on each.
7. Read the selection count after selecting several, including objects
   scrolled out of sight.

Watch for: a row announced with its slot number instead of its row number; a
grid taking more than one tab stop; a target that can be seen but not reached;
progress announced as a stream of interruptions; anything announced twice.

## Results

| # | What was done | What VoiceOver said | Pass / fail |
| --- | --- | --- | --- |
| 1–7 | Not performed | — | WAIVED — user decision, 2026-09-13 |

## Findings

None. Nothing was observed, because nothing was done. This record establishes
that the decision was made and by whom; it establishes nothing about the
product.
