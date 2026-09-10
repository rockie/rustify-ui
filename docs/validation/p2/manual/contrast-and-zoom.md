# Manual record: reading the pages at 200% and 400%

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
| 1 | | | | | |
| 2 | | | | | |

## Findings

<!-- One line per problem, or "no findings" explicitly. -->
