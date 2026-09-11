# Manual record: the twenty B5 samples against a reference

STATUS: PERFORMED

Result: **PASS — user-reported, 2026-09-11.** The user confirmed text samples
and contrast and zoom passed. This records the overall human acceptance
result; detailed per-step observations were not supplied.

| | |
| --- | --- |
| Date | 2026-09-11 |
| Device | Local Mac, model identifier Mac17,9 |
| OS | macOS 26.6.2 (25G83) |
| Browser and version | Google Chrome 152.0.7977.84 installed locally |
| Assistive technology and version | Not reported for this visual walkthrough |
| Build id | component-catalog: `bee689d75a2bd76f`, served at `http://127.0.0.1:4176/` |
| Performed by | User, reporting the result in this conversation |

The local environment and served build IDs were read by the agent. The pass
is the user's report, not an independent automated replay of this session.

## What to do

Open `/samples` in component-catalog. The twenty texts are drawn twice: by the
browser on the left, by the region on the right.

A **reference rendering** of the same twenty strings is needed for this and is
the reviewer's to supply - see A-4. Record where it came from:

| | |
| --- | --- |
| Reference rendering | Not supplied in the conversation; source and image unavailable in this record |

Then, for each of the twenty, compare all three - browser, region, reference.

## What to look for

- **Direction.** The four right-to-left samples read right to left, and the
  digits and Latin words inside them sit where the reference puts them.
- **Order.** Combining marks are on the letter they belong to; the Devanagari
  and Thai clusters are in the order the reference shows.
- **Glyphs.** Anything drawn as a box, a question mark, or a blank. The
  *blank* case is already measured by `p2-i18n`; boxes are not, because a box
  is a glyph and has a width.

The shipped fonts have no Arabic or Hebrew face, so the region is **expected**
to fail those samples. Record what it actually draws rather than skipping them:
that is the evidence for the limitation in `docs/compatibility.md`.

## Results

| Scope | Result | Evidence detail |
| --- | --- | --- |
| P2 text-sample manual acceptance | PASS — user-reported | No reference image, per-sample comparison, or individual DOM/GPU observations supplied |

## Findings

No new issue was reported with the user's acceptance. The existing Arabic and
Hebrew GPU font coverage gap remains documented above and in
`docs/compatibility.md`; this confirmation is not evidence that those fonts
were added or that every glyph was independently verified. The record retains
the user's overall acceptance without inventing reference images or individual
comparison results.
