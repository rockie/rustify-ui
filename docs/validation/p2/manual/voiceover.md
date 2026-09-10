# Manual record: VoiceOver over the catalogue and B1

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

VoiceOver on macOS with Chrome at the version in the table above.

1. **The catalogue's eighteen categories.** Open each category page and move
   through its example with VO. For each, note what is announced for the
   control: its name, its role, its value where it has one, and its state.
2. **The state matrix on each page.** The same component in default, disabled,
   read-only and invalid. Note whether the state is announced, and whether a
   disabled item is skipped rather than announced as available.
3. **The four floating categories** (dialog, menu, select, tooltip). Open each
   from the keyboard. Note what is announced on open, whether the rest of the
   page is silent while it is open, and where focus is announced on close.
4. **B1's five journeys** in property-workbench: select an object, edit a
   property, run a command, read a failed lookup's result, and undo the
   selection. Note what is announced at each step, and in particular whether
   the failed lookup's status is announced without moving focus.

## What to look for

- A control announced with no name, or with a name that is not what it looks
  like on screen.
- **Anything announced twice.** The tree is checked for duplicate names, but a
  duplicate *announcement* can also come from nesting, and only listening finds
  that.
- Focus announced somewhere the keyboard did not go.
- A live region that says nothing, or says something twice.

## Results

| # | What was done | What VoiceOver said | Pass / fail |
| --- | --- | --- | --- |
| 1 | | | |
| 2 | | | |

## Findings

<!-- One line per problem. If there were none, say so explicitly - "no
findings" is a result; an empty section is an unfinished record. -->
