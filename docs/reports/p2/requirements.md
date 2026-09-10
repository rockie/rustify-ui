# P2 requirement matrix

Every PRD requirement, what P2 actually did about it, and where the evidence is.
P1's row for the same requirement is in
[`docs/reports/p1/requirements.md`](../p1/requirements.md); this table says what
*changed* and what the requirement now stands at.

Status words mean exactly this:

- **met** — everything P2 scoped for it has a passing result.
- **partial** — some of it has a passing result and the rest is named as deferred, in the plan, before this report.
- **deferred** — nothing was built for it in P2, by decision.
- **not verified** — built, but the check that would settle it has not been run.

| PRD | P2 status | What P2 did | Evidence |
| --- | --- | --- | --- |
| R11 disabled, read-only, wheel, cross-region drag | met | One pointer session across both halves; a hundred drags delivering a hundred drops; the four required sequences; a wheel at an edge handed to the page by a synchronous host decision | [M6](../../validation/p2/m6.md) |
| R14 clipboard and files | met | Copy and paste with a manual path on refusal; import with three answers and a fourth for a dismissed picker; text and binary export compared byte for byte | M6 |
| R15 / R25 semantics | partial | Eighteen categories with name, role, value and state; the four floating ones rebuilt onto the overlay stack with focus trapping and return; the screen-reader pass is still a missing manual record | [M2](../../validation/p2/m2.md), [accessibility](accessibility.md) |
| R16 theme | met | One token table drives both halves; contrast over both themes is a host test, and it found two real failures when written | M2, [M7](../../validation/p2/m7.md) |
| R17 languages and Unicode | partial | The SDK's own words in two languages, `Intl` formatting with the application's options, twenty B5 samples drawn by both halves. The reviewer's comparison against a reference rendering is a missing manual record (A-4) | M7 |
| R18 component catalogue | met | Eighteen categories, each with a DOM example, a GPU column and a state matrix; the capability table has no blanks and is generated | M2 |
| R19 forms | met | A state machine that holds no values, generations that void a late answer, errors that carry their source, and one save per request | [M3](../../validation/p2/m3.md) |
| R21 workspace | met | Three panels with application-declared minimums, ten views, a keyboard-operable splitter, a command palette that is itself a modal layer, and a menu opened inside the region | [M5](../../validation/p2/m5.md) |
| R22 navigation | met (scoped) | Static segments and `:param`, one URL owner, anchor interception, a guard that undoes a refused move through history by sequence number, and deep links at a root and a sub-path deployment | [M4](../../validation/p2/m4.md) |
| R27 deployment | met (scoped) | `build-web --base`, a build that carries its base and refuses to be served elsewhere, `serve --spa`, and a region whose resources come from the deployment rather than the route | M4 |
| R28 examples and docs | partial | The third example (component-catalog) is delivered, and the document set is complete. The migration samples and the developer study are not P2's | M1–M7, [quickstart](../../quickstart.md) |
| R29 start-up and size | measured | B1's baselines beside R29's figures. **No budget claimed** (A-3) | [performance](performance.md) |
| R30 interaction latency | measured | Counted inside the page, as accepted actions rather than dispatched events | performance |
| R35 accessibility numbers | partial | The contrast ratios are now a host test over both themes; the 200%/400% reflow walkthrough is still a missing manual record | [accessibility](accessibility.md) |
| R36 security | met (P2 scope) | The strict policy unchanged and nothing added for the export; text that looks like markup stays a value; no user text in diagnostics by construction | [compatibility](compatibility.md) |
| R38 traceability | met | The Rust/UI import recorded file by file with a digest and a verbatim/rewritten mark; the fork's M6 change is one message and the decision that reads it | M1, compatibility |
| R40 acceptance | partial | `cargo xtask verify --suite p2`, two clean builds of each of three examples, these six reports and this matrix; four manual records are missing | [M8](../../validation/p2/m8.md) |

Requirements P2 did not touch keep their P1 status.

## Plan-level ledger

| ID | Where it landed | Result |
| --- | --- | --- |
| R-1 component layer | §2, §6.1 | Delivered: eighteen categories, controlled, prefixed, catalogued |
| R-2 forms | §5.3 | Delivered |
| R-3 navigation | §5.2 | Delivered within ADR-6's scope |
| R-4 workspace | §6.2 | Delivered as B1's shape |
| R-5 drag, wheel, clipboard, files | §5.4, §5.5 | Delivered; dragging out to the OS and pen pressure excluded by the requirement itself |
| R-6 languages and samples | §5.6 | Delivered except the reviewer's comparison (A-4) |
| R-7 acceptance | §10 | The automated half is delivered; the manual half is not in |
| NFR-1 performance | §7 | Baselines only, by A-3. No budget claimed |
| NFR-2 reliability | §5, §8 | Exactly one drop per drag, one save per request, no action lost or duplicated |
| NFR-3 compatibility | §9.4 | Chrome is the gate; the manual records that would complete it are missing |
| NFR-4 security | §4 | Met for P2's scope; the strict policy was not relaxed |
| NFR-5 maintainability | §4, §7 | Sources locked, drift attributable, diagnostics bounded, reports delivered |
| A-3 measurement contract | §7 | Registered: baselines only |
| A-4 acceptance resources | §0.2 | **Open.** VoiceOver, real pinyin and a reference rendering were promised; the sessions have not been run |
| A-6 `web_sys_unstable_apis` | §0.2 | **Resolved** in M6: the flag reaches the wasm through cargo-makepad, and a build without it says so rather than failing |
