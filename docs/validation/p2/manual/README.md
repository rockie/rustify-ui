# The four records only a person can write

The automated checks have passing results in the P2 reports. Manual acceptance
was closed on 2026-09-11: the user confirmed pinyin, text samples, and contrast
and zoom passed; VoiceOver records a dialog pass and an explicit waiver for the
remaining checks. The records identify these as user-reported conclusions and
retain missing detailed observations, reference images, and known limitations.

Unperformed records carry the line

```
STATUS: NOT PERFORMED
```

and `cargo xtask verify --suite p2` treats a record still carrying that line as
outstanding, exactly as it treats a missing file. A blank form must not be able
to read as a pass - that is the whole reason the suite prints these at all.

To hand one in: perform the session, fill in the header and the results, and
change that line to `STATUS: PERFORMED`. Leave the failures in. A record with no
findings should explicitly say that none were found.

An explicit user waiver is recorded as `STATUS: WAIVED`, with its scope and
date, and leaves untested checks marked skipped. The verifier prints `held`
for that record, meaning a disposition is on file; it is not a test pass.

## Recorded disposition and automated checks

| Record | Manual disposition | Already checked by a test |
| --- | --- | --- |
| [voiceover.md](voiceover.md) | Dialog passed; remaining checks waived by the user on 2026-09-11 | Every operable control has a name; twenty distinct nav names; Tab comes back round; a modal the keyboard cannot get behind (`p2-semantics`, `p2-a11y`) |
| [pinyin.md](pinyin.md) | PASS — user-reported, 2026-09-11; individual input strings not supplied | A composition in flight is not a value; keys during one belong to the composition; one commit at the end (`p2-ime`) |
| [samples.md](samples.md) | PASS — user-reported, 2026-09-11; reference images and individual comparisons not supplied; known font limitations retained | No sample blank; a Latin pangram proportional to its text; combining marks correctly narrow; the required scripts loaded (`p2-i18n`) |
| [contrast-and-zoom.md](contrast-and-zoom.md) | PASS — user-reported, 2026-09-11; per-page/theme/zoom observations not supplied | Contrast ratios over both themes; B1's five journeys at 200%; the catalogue reflowing at 320 CSS px (`theme` host test, `p2-zoom`, `p2-reflow`) |

Automated checks and user acceptance are separate evidence sources. A supplied
record or waiver does not establish full PRD conformance or remove a known
implementation limitation.
