# Manual record: a real pinyin session

<!-- Fill in every field. `verify --suite p2` reports this record as
outstanding while the STATUS line below still says NOT PERFORMED. -->

STATUS: NOT PERFORMED

| | |
| --- | --- |
| Date | |
| Device | |
| OS | |
| Browser and version | |
| Assistive technology and version | |
| Build id | (from `build-manifest.json`, or the page's own diagnostics) |
| Performed by | |

## What to do

A real pinyin input method - the macOS one, not synthetic events - in
property-workbench.

1. **The property form.** Focus the name field and compose 属性工作台 with
   pinyin, choosing candidates from the input method's own list. Then compose
   something and **abandon it** with Escape.
2. **The notes field.** Compose several lines, including one where a candidate
   list is open when you press Enter.
3. **The command palette.** Open it, compose a search term, and press Enter
   while the candidate list is open. Then finish the composition and run a
   command.

## What to look for

- The application taking the raw pinyin as a value at any point. (This was a
  real defect, found and fixed in M8; `p2-ime` now holds the line. What a real
  input method does that synthetic events do not is the point of this session.)
- Enter choosing a candidate *and* also submitting or running a command.
- The field's value being replaced underneath the input method mid-composition.
- The candidate window appearing somewhere unrelated to the caret.

## Results

| # | Where | What was composed | What the application ended up with | Pass / fail |
| --- | --- | --- | --- | --- |
| 1 | | | | |
| 2 | | | | |

## Findings

<!-- One line per problem, or "no findings" explicitly. -->
