# P1 requirement matrix

Every PRD requirement R01–R40, what P1 actually did about it, and where the
evidence is. `R01`–`R40` are the PRD's local working numbers, not SPMS keys.

Status words mean exactly this:

- **met** — everything P1 scoped for it has a passing result.
- **partial** — some of it has a passing result and the rest is named as deferred, in the plan, before this report.
- **deferred** — nothing was built for it in P1, by decision.
- **not verified** — built, but the check that would settle it has not been run.

| PRD | P1 status | What was done | Evidence |
| --- | --- | --- | --- |
| R01 mount, remount, bad container | met | `mount` / `AppHandle`; missing and occupied containers fail without disturbing running scopes; twenty close-and-mount rounds | [M2](../../validation/p1/m2-runtime.md) |
| R02 authoritative state | met (P1 load) | 1,000 objects, 10,000 sequenced actions, 100 in one batch. Not the PRD's B1 | [M3](../../validation/p1/m3-state.md) |
| R03 stable ids | met | Reordering, adding, removing, and a duplicate id refused rather than guessed | M3 |
| R04 mixed input | partial | 240 moves at 120 Hz with 20 clicks and 20 saves, nothing lost; R30's latency targets are measured, not met | M3, [M8](../../validation/p1/m8-baselines.md) |
| R05 several scopes | partial | Two scopes × two regions, late messages, disposal. One shared wasm module: not an independent failure domain | M2 |
| R06 four states | met | Four states, a hundred pairs answered backwards, a hundred tickets after a close | [M6](../../validation/p1/m6-components-async-theme.md) |
| R07 host coexistence | partial | Host links, scrolling and selection survive; one third-party component verified and no claim made about others | M2, M6 |
| R08 custom GPU controls | met (subset) | Selectable object grid, GPU checkbox and slider, both strictly controlled; six capability classes in the catalogue | M3, M6 |
| R09 geometry | met | Three viewports × three zooms × twenty anchors ≤1 CSS px; a hundred resize-and-scroll rounds; 0×0 recovery; DPR | [M4](../../validation/p1/m4-geometry.md) |
| R10 overlays | met | Anchored menu ≤1 px and not occluded; a hundred modal clicks pass none down; Escape and focus return over two layers | M4 |
| R11 disabled, read-only, wheel | partial | Disabled and read-only verified in both halves; general cross-region drag and file import are P2 | M4, M6 |
| R12 focus and commands | met | Twenty Tab stops, command ownership, no host focus stolen, no hidden text box | M4 |
| R13 text and IME | **partial, blocked** | The composition rules are verified with synthetic events; a real macOS pinyin session is the missing manual record | [M5](../../validation/p1/m5-text-and-semantics.md) |
| R14 clipboard and files | deferred | Native copy and paste inside a control is the browser's; programmatic clipboard and file import are P2 | plan §0.6 |
| R15 / R25 semantics | partial | Twenty controls by role and name, five keyboard journeys, a five-second answer bound; the screen-reader pass is the missing manual record | M5 |
| R16 theme | met (subset) | One token table per scope, twenty switches, a local override that can be turned off | M6 |
| R17 languages and Unicode | partial | Text that needs more than Latin moves its label to the family that covers it, and that fetch is measured; full translation, RTL and GPU typography are P2 | M5, M8 |
| R18 component catalogue | partial | Eighteen categories answered; nine shipped, nine saying `no` with a reason | M6 |
| R19 forms | partial | A refused value returns to the value in force with a reason; a general validation framework is P2 | M6 |
| R20 large data | deferred | The thousand-object fixture is not a substitute and is not used as one | plan §0.6 |
| R21 workspace | deferred | P2 | plan §0.6 |
| R22 navigation | deferred | P2. P1 verifies only that a region cannot change the URL, history or title | [M7](../../validation/p1/m7-deployment-and-recovery.md) |
| R23 resources | partial | Fonts, images and data in six categories, with the three asset-failure classes exercised; only the missing one is reported | M7 |
| R24 developer diagnostics | partial | Ten registered classes each with a next step, bounded and truncation-transparent; region and component inspection are not built | M7 |
| R26 failure boundaries | partial | Capability refusals recorded; a lost context recovered; per-instance trap isolation is not P1's | M7 |
| R27 deployment | partial | Root, sub-path and embedded from one build under the strict policy; the enhanced mode and a third example are later | M7 |
| R28 examples and docs | partial | Two examples, a quick start, an architecture note, a capability document and this report set; the third example and migration samples are later | M8 |
| R29 start-up and size | measured | Start-up, first-load transfer and six-category size baselines. **No budget claimed**: the PRD's numbers are unapproved (A-6) | [performance](performance.md) |
| R30 interaction latency | measured | 1,000 controlled round trips, p95 and worst case, taken inside the page. No budget claimed | performance |
| R31 idle and restore | partial | Idle pumps, hide-and-restore rounds, first frame after a 0×0 recovery; CPU figures per machine class are not P1's | M2, M4, M8 |
| R32 memory | partial | Symmetric release over 250 rounds, a hundred measured mount rounds, the endurance curve; the full B0/B2 matrix is later | M2, M8 |
| R33 reliability over time | **partial, with a failure** | Endurance at ten actions a second: nothing lost, duplicated or late, at two minutes and at two hours. But the two-hour run's memory tail is not flat and the test fails on it - an open defect, see [the performance report](performance.md) | M8 |
| R34 browser matrix | partial | macOS Chrome is the pass gate; Safari is an observation and has not been run | [compatibility](compatibility.md) |
| R35 accessibility numbers | **not verified** | The contrast and 200%/400% reflow walkthrough is a missing manual record | [accessibility](accessibility.md) |
| R36 security | met (P1 scope) | No default outbound traffic, no JavaScript `unsafe-eval`, no user text in logs by construction, text that looks like markup stays a value | M1, M7 |
| R37 developer experience | partial | Build and verification entry points with their timings; the five-person study is later | [quickstart](../../quickstart.md), M8 |
| R38 traceability | met | A hard-fork import record with a digest per file, pinned toolchain and dependencies, drift reported by `cargo xtask sources verify` | M1, compatibility |
| R39 diagnostics cost | partial | The record is bounded and its cost is now measurable both ways: [performance](performance.md) reports it with the record on and off. The B1 ≤5% budget is not P1's | M7, M8 |
| R40 acceptance | partial | `cargo xtask verify --suite p1`, two clean builds of each example, these six reports and this matrix; four manual records are missing | M8 |

## Plan-level ledger

| ID | Where it landed | Result |
| --- | --- | --- |
| R-1 … R-7 | §2, §5, §6 of the plan | Delivered as the milestone reports record; every partial is named above |
| NFR-1 performance | §7, §9 | Baselines only, by A-6. No budget claimed |
| NFR-2 reliability | §5, §8 | No loss, duplication or late delivery on P1's load; region rebuild keeps the application's state |
| NFR-3 compatibility | §6, §9.4 | Chrome is the gate; three of the four manual records that would complete it are missing |
| NFR-4 security | §4, §7 | Met for P1's scope |
| NFR-5 maintainability | §4, §7, §9 | Sources locked, builds reproducible, diagnostics bounded, reports delivered |
| A-1 … A-4, C-5 | — | Resolved in M1 |
| A-5 real input and assistive technology | §9.4 | **Open.** The environment was promised; the sessions have not been run |
| A-6 measurement contract | §7 | Registered: baselines only, method frozen at M1 |
