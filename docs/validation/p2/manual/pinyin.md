# Manual record: a real pinyin session

STATUS: PERFORMED

Result: **PASS — user-reported, 2026-09-11.** The user confirmed
“拼音 通过” (pinyin passed). This records that overall manual acceptance result;
individual input strings, per-step observations, and the input method's exact
name and version were not supplied.

| | |
| --- | --- |
| Date | 2026-09-11 |
| Device | Local Mac, model identifier Mac17,9 |
| OS | macOS 26.6.2 (25G83) |
| Browser and version | Google Chrome 152.0.7977.84 installed locally |
| Input method and version | Real pinyin input; exact product and version not supplied |
| Build id | property-workbench: `13dd1aba0b6c5afc`, served at `http://127.0.0.1:4174/` |
| Performed by | User, reporting the result in this conversation |

The local machine and installed Chrome version were read from the host; the
build ID was read from the running server's `build-manifest.json`. The manual
result comes from the user, not an automated reproduction of the session.

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
| 1 | P2 real pinyin manual acceptance in property-workbench; overall user report | Exact strings not supplied | User confirmed acceptance passed; individual final values not supplied | PASS — user-reported |

## Findings

No issue was reported with the user's pass confirmation. The steps above remain
the acceptance procedure; this record does not invent a sentence count or
individual observations, and does not backfill P1's unperformed pinyin session.
