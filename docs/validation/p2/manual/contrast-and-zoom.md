# Manual record: reading the pages at 200% and 400%

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
| Build id | component-catalog: `bee689d75a2bd76f`; property-workbench: `13dd1aba0b6c5afc` (ports 4176 and 4174) |
| Performed by | User, reporting the result in this conversation |

The local environment and served build IDs were read by the agent. The pass
is the user's report, not an independent automated replay of this session.

## What is already checked, and what is not

| Checked by a test | Left to judge |
| --- | --- |
| Contrast ratios, both themes, thirteen pairs each (`cargo test -p rustify-ui theme`) | Whether the result is comfortable to read, not merely above the ratio |
| B1's five journeys at 200% (`p2-zoom`) | Whether they are *pleasant* to do at that size |
| The catalogue reflowing at 320 CSS px with nothing lost (`p2-reflow`) | Whether the reflowed page is still legible and its order still makes sense |

This session is the second column.

## What to do

Browser zoom, not a resized window: 200% and then 400% on a 1280-wide window.

1. **component-catalog at 200% and 400%.** Read a category page, the status
   table and the samples page. The capability table is allowed to scroll
   sideways inside itself; the page is not.
2. **property-workbench at 200%.** Do the five journeys. The three panels are
   allowed to need horizontal scrolling at that size; note whether they do and
   whether it gets in the way.
3. **Both themes** at both zoom levels.

## What to look for

- Text clipped, overlapped, or cut off by a fixed-height box.
- A control whose label has wrapped away from it.
- Something that reflowed into an order that no longer reads sensibly.
- A focus ring that has moved off its control, or become invisible.
- Anything that passes its contrast ratio and is still hard to read - a thin
  weight on a busy background, for instance.

## Results

| # | Page | Zoom | Theme | What was found | Pass / fail |
| --- | --- | --- | --- | --- | --- |
| 1 | P2 contrast and zoom manual acceptance | Per-level observations not supplied; procedure specifies 200% and 400% | Per-theme observations not supplied | User confirmed the overall acceptance passed | PASS — user-reported |

## Findings

No issue was reported with the user's pass confirmation. Per-page, per-theme
and per-zoom observations were not supplied and have not been reconstructed.
