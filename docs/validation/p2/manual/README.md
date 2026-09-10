# The four records only a person can write

Everything in P2's acceptance that a machine can settle has been settled and is
green. What is here is the residue: four sessions that need ears, an input
method, a reference rendering, and a look at a screen.

Each file beside this one is a **form, not a result.** Every one of them carries
the line

```
STATUS: NOT PERFORMED
```

and `cargo xtask verify --suite p2` treats a record still carrying that line as
outstanding, exactly as it treats a missing file. A blank form must not be able
to read as a pass - that is the whole reason the suite prints these at all.

To hand one in: perform the session, fill in the header and the results, and
change that line to `STATUS: PERFORMED`. Leave the failures in. A record with no
findings is a record nobody will believe.

## What each one still needs, and what is already checked

| Record | Needs | Already checked by a test |
| --- | --- | --- |
| [voiceover.md](voiceover.md) | A screen reader and someone to listen to it | Every operable control has a name; twenty distinct nav names; Tab comes back round; a modal the keyboard cannot get behind (`p2-semantics`, `p2-a11y`) |
| [pinyin.md](pinyin.md) | A real input method and someone to type in it | A composition in flight is not a value; keys during one belong to the composition; one commit at the end (`p2-ime`) |
| [samples.md](samples.md) | A reference rendering of the twenty B5 texts | No sample blank; a Latin pangram proportional to its text; combining marks correctly narrow; the required scripts loaded (`p2-i18n`) |
| [contrast-and-zoom.md](contrast-and-zoom.md) | Eyes on the pages at 200% and 400% | Contrast ratios over both themes; B1's five journeys at 200%; the catalogue reflowing at 320 CSS px (`theme` host test, `p2-zoom`, `p2-reflow`) |

The tests in the right-hand column are why the sessions are short: what is left
to judge is the part that was never measurable.
