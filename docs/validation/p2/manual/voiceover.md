# Manual record: VoiceOver over the catalogue and B1

STATUS: WAIVED

Partial result, 2026-09-11: the user reported **PASS** and clarified
“只走查了 dialog 弹窗” (only the dialog walkthrough). The user then instructed
“不查了，跳过” (stop checking and skip). The remaining VoiceOver checks are
waived for P2; they were not performed and are not recorded as passes. This
decision does not waive pinyin, text samples, or contrast and zoom acceptance.

| | |
| --- | --- |
| Date | 2026-09-11 |
| Device | Local Mac, model identifier Mac17,9 |
| OS | macOS 26.6.2 (25G83) |
| Browser and version | Google Chrome 152.0.7977.84 installed locally |
| Assistive technology and version | VoiceOver 10 (993) installed locally |
| Build id | component-catalog: `bee689d75a2bd76f`, served at `http://127.0.0.1:4176/` |
| Performed by | User, reporting the result in this conversation |

The OS and installed application versions were read from the local machine;
the build ID was read from the running server's `build-manifest.json`.
The result and its scope come from the user. No verbatim speech transcript
or individual step observations were supplied.

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
| 1 | Dialog walkthrough described in the conversation: locate `open the dialog`, open `a modal dialog`, inspect navigation, close with Escape and check focus return | No verbatim transcript supplied | PASS — user-reported; dialog only |
| 2 | Remaining catalogue categories and component state matrices | Not recorded | SKIPPED — user decision, 2026-09-11 |
| 3 | Menu, select and tooltip walkthroughs | Not recorded | SKIPPED — user decision, 2026-09-11 |
| 4 | B1's five journeys in property-workbench | Not recorded | SKIPPED — user decision, 2026-09-11 |

## Findings

No dialog issue was reported. Only that walkthrough has a user-reported pass.
The remaining rows are untested and waived as P2 completion gates by the user;
this record does not establish full screen-reader support.
