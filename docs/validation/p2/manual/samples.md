# Manual record: the twenty B5 samples against a reference

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

Open `/samples` in component-catalog. The twenty texts are drawn twice: by the
browser on the left, by the region on the right.

A **reference rendering** of the same twenty strings is needed for this and is
the reviewer's to supply - see A-4. Record where it came from:

| | |
| --- | --- |
| Reference rendering | (what produced it, and where it is kept) |

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

| Sample | Browser | Region | Matches reference | Notes |
| --- | --- | --- | --- | --- |
| en-plain | | | | |
| en-punctuation | | | | |
| zh-common | | | | |
| zh-punctuation | | | | |
| zh-latin | | | | |
| zh-traditional | | | | |
| zh-vertical-forms | | | | |
| ar-plain | | | | |
| ar-digits | | | | |
| ar-latin | | | | |
| he-plain | | | | |
| combining-acute | | | | |
| combining-stack | | | | |
| combining-devanagari | | | | |
| combining-thai | | | | |
| emoji-family | | | | |
| emoji-skin-tone | | | | |
| emoji-flags | | | | |
| emoji-in-text | | | | |
| mixed-everything | | | | |

## Findings

<!-- One line per problem, or "no findings" explicitly. -->
