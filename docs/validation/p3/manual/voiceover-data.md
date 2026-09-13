# Manual record: VoiceOver over a hundred thousand rows and ten thousand objects

STATUS: NOT PERFORMED

This is the form, not the record. It is filled in after a person does the
walkthrough, and until then P3's A-4 stays open: a blank form is reported by
`cargo xtask verify --suite p3` as a record that has not been handed in, never
as a pass. The user may waive it as they waived P2's remaining VoiceOver rows
(`docs/validation/p2/manual/voiceover.md`), in which case the waiver is
recorded here and in `docs/reports/p3/accessibility.md` as **untested, waived**
and not as a pass.

| | |
| --- | --- |
| Date | |
| Device | |
| OS | |
| Browser and version | |
| Assistive technology and version | |
| Build id | |
| Performed by | |

Read the build ID from the running server's `build-manifest.json` rather than
from the source tree, so the record names the artefact that was actually
walked.

## What to do

VoiceOver on macOS with Chrome. Serve the fourth example:

    cargo xtask serve --example data-workbench --release --port 4178 --spa

### B2, the table at `/table`

R20 AC3 and R15 AC3 are about reaching a row that has never been drawn. The
three targets are frozen in `tests/browser/loads.ts`: **row 1, row 50,000 and
row 100,000**.

1. **Reach each target from the keyboard alone**, through the go-to-row entry
   and through `Ctrl+Home` / `Ctrl+End`. Note what VoiceOver announces for the
   row: its position, and whether the position it says is the row's own number
   out of a hundred thousand rather than its position in the sixty rows the
   browser is currently holding.
2. **Open the details of the row you reached, change a field, and submit.**
   Note what is announced for the field, for a value the application refuses,
   and for the save.
3. **Scroll away and come back to the same row.** Note whether what is
   announced is the same value you typed.
4. **Sort by a column, then filter.** Note what is announced for the column
   header's sort state, for the progress of a job over the whole sample, and
   for a cancel.
5. **Move through the group tree** with the arrow keys. Note whether expanding
   and collapsing are announced, and whether choosing a group announces what
   the table then shows.

### B3, the scene at `/scene`

6. **Find each of the three objects** - `OBJ00000`, `OBJ05000`, `OBJ09999` -
   through the query entry, and perform on each the same main action the
   pointer performs. Note what is announced when an object is found, selected,
   and renamed.
7. **Read the selection count** after selecting several. Note whether the
   count announced is the application's, including objects scrolled out of
   sight.

## What to look for

- A row announced with its slot number instead of its row number. The pool
  reuses sixty elements for a hundred thousand rows, and this is the failure
  that pool makes possible.
- A grid that takes more than one tab stop, or that traps the keyboard.
- **A target that can be seen but not reached**: the automated checks prove the
  entries work, not that a person can find them.
- A job's progress announced as a stream of interruptions rather than once.
- Anything announced twice, which nesting can cause and only listening finds.

## Results

| # | What was done | What VoiceOver said | Pass / fail |
| --- | --- | --- | --- |
| 1 | | | |
| 2 | | | |
| 3 | | | |
| 4 | | | |
| 5 | | | |
| 6 | | | |
| 7 | | | |

## Findings
