# P2 accessibility report

What was checked, how, and what still needs a person. The four manual records
are listed by `cargo xtask verify --suite p2`, which reports them as missing
rather than passed.

## The controlled contract, and why it is an accessibility matter

Every control's displayed value comes only from its `value` prop; a user action
is a *request* the application answers. A control that moved itself and then
told the application would, at some point, show one thing to a sighted user and
report another to a screen reader. One value, one answer, both halves.

`disabled` items are out of the tab order and produce no action. `readonly`
items can be reached, focused and read. The distinction is deliberate: a value
somebody may not change is still a value they may need.

## Names, roles, values and states

The eighteen catalogue categories each carry a name, a role, a value where they
have one, and their state. Where the browser's own control does the job it is
the browser's - the checkbox is a real `<input type="checkbox">` under a drawn
box, so its semantics, keyboard and form participation are the platform's, and
only the appearance is ours.

The four floating categories (dialog, menu, select, tooltip) were rewritten onto
the SDK's overlay stack in M2 to get `role`, `aria-modal`, focus trapping, an
Escape order and the return of focus, none of which the imported source had.

## Keyboard

| Journey | Result | Evidence |
| --- | --- | --- |
| Tab order through a category page, including the state matrix | Every reachable control is reachable, disabled ones are not | `p2-semantics.spec.ts` |
| Arrow keys within a radio group, tab strip and menu | One stop per group; arrows move within it | `p2-semantics.spec.ts`, `roving.rs` |
| The command palette | Opens, filters, runs, and returns focus where it came from | `p2-workspace.spec.ts` |
| The splitter | Arrow keys move a divider by the component's declared step; a panel stops at its minimum | `p2-workspace.spec.ts` |
| A refused command | Stays visible, says why, and does nothing on Enter or click | `p2-workspace.spec.ts` |

## A submit button that cannot submit

`aria-disabled`, not `disabled`. A natively disabled button swallows the click,
so a person pressing it learns nothing about why; `aria-disabled` keeps it
focusable and lets the press produce the explanation. The same reasoning put
`aria-invalid` and `aria-describedby` on a field with an error, so the reason
arrives with the field rather than somewhere else on the page.

## Contrast

The token table's contrast ratios are a **host test**
(`cargo test -p rustify-ui theme`) rather than a walkthrough, over both themes
and thirteen pairs each: normal text at 4.5:1, large text and non-text at 3:1.

It found two real failures when it was first written:

- White on the light theme's primary blue was 3.24:1 - under AA for anything
  that is not large text, and a primary button's label is not large text.
- The border token was 1.41:1 against the light background and 1.80:1 against
  the dark one. That token is a control's boundary as well as a panel's edge,
  and a boundary nobody can see is a control nobody can find.

Both are fixed. A token table is exactly the kind of thing that goes wrong one
value at a time, and each time it does, the person who cannot read the result
finds out first - which is why this is a test and not a review.

## Zoom and reflow

Also checked rather than walked through, and the way WCAG defines the test: 200%
zoom of a 1280-pixel window is a 640-pixel CSS viewport, 400% is 320.

| Check | Result |
| --- | --- |
| B1's five journeys at 200%, keyboard only, with the root font size doubled as well | Complete - select, edit, run a command, read a failed result, and undo the selection | 
| The region at 200% | Still drawn, still reports where it drew, and what it reports is inside the canvas it was given |
| The catalogue at 320 CSS pixels | Reflows: the document is no wider than the viewport |
| Content at 320 CSS pixels | Nothing lost - all eighteen categories plus the status and samples pages still reachable, and the page still switches |

**It failed when it was written, and by a lot.** The catalogue was 557 pixels
wide in a 320-pixel viewport: a fixed 220-pixel nav column beside a main column
with a minimum of its own could never have fitted, so the page had never
reflowed at any width. Below 640 pixels it is one column now, the header's row
of switches wraps, the region takes the width it is given, and the capability
table scrolls inside its own container - which is what WCAG allows for content
that genuinely needs two dimensions, as long as the *page* does not.

## Motion

`reduce_motion` is honoured by both halves: the DOM as a zero transition
duration, the region by not easing a change it would otherwise animate. It is a
theme value, so one setting covers both.

## What needs a person

| Record | What it covers |
| --- | --- |
| `docs/validation/p2/manual/voiceover.md` | VoiceOver + Chrome over the eighteen categories and B1's five journeys |
| `docs/validation/p2/manual/pinyin.md` | Real pinyin input in the property form and the command palette |
| `docs/validation/p2/manual/samples.md` | The twenty B5 samples against a reference rendering (A-4) |
| `docs/validation/p2/manual/contrast-and-zoom.md` | Whether the reflowed pages are *readable* at 200% and 400%. The ratios, the journeys and the reflow itself are checked above; this is the judgement a measurement cannot make |

None of these is in. This release cannot be called complete until they are, and
the verification suite says so rather than reporting a pass.
