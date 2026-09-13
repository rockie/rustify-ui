# P3 requirement matrix

Every PRD requirement P3 touched, what it did about it, and where the evidence
is. P2's row for the same requirement is in
[`docs/reports/p2/requirements.md`](../p2/requirements.md); requirements P3 did
not touch keep their P2 status.

Status words mean exactly this:

- **met** — everything P3 scoped for it has a passing result.
- **partial** — some of it has a passing result and the rest is named as deferred, in the plan, before this report.
- **deferred** — nothing was built for it in P3, by decision.
- **not verified** — built, but the check that would settle it has not been run.

| PRD | P3 status | What P3 did | Evidence |
| --- | --- | --- | --- |
| R05 late callbacks, other instances | met (raised to instance level) | AC2 is now about wasm instances, not mounts: a trapped instance's page-level listeners are aborted by the loader and its URL ownership cleared, and the instance beside it accepted 100 of 100 actions | [M5](../../validation/p3/m5.md) |
| R08 custom GPU controls | met (another sample) | The scene is a second custom control: culling, picking, marquee, hover streams, an absolute camera | [M4](../../validation/p3/m4.md) |
| R15 query and navigation | met (AC3) | The three table rows and the three scene objects are each reached by a query entry and changed, with the same result as the pointer path | [M2](../../validation/p3/m2.md), [M4](../../validation/p3/m4.md) |
| R20 large tabular data | met | A hundred thousand rows reachable at first, middle and last without silent truncation; selection by ID surviving sort, filter, insert and delete; a row reached and edited from the keyboard alone; a two-level group tree | [M2](../../validation/p3/m2.md), [M3](../../validation/p3/m3.md) |
| R24 diagnostics | partial | Two informational kinds added (`GpuInitRetry`, `JobCancelled`); the inspector and hot reload are not P3's | M3, M5 |
| R25 twenty named targets | met | Twenty roles and names frozen in `loads.ts`, located 30 times each across both views | M2, M4 |
| R26 instance isolation | met (AC3) | Four trap doors — exported call, host task, DOM handler, simulated — all ending at one notice with a restart entry; restarts bounded at three; one URL owner per page | [M5](../../validation/p3/m5.md), [ADR-8](../../plan/P3-WASM-UI.md) |
| R28 example applications and docs | partial | The fourth example (data-workbench) is the large-data sample, and `docs/data.md` is delivered. Migration samples are P4's | [M2](../../validation/p3/m2.md)–[M8](../../validation/p3/m8.md) |
| R29 start-up and size | met (B0 and B1, one combination) | **Gated**: B0's cold, hot and first-load figures are asserted by `budget-minimal`, B1's by `budget` | [performance](performance.md) |
| R30 interaction latency | met (one combination; B3 headed only) | **Gated**: AC1 by `budget`, AC2 over B2 and AC3 over the jobs by `budget-data`, AC2 over B3 by `budget-scene` on headed Chrome. Each frame gate runs a negative probe that must fail first | [performance](performance.md) |
| R31 idle and hidden | partial | 0 frames over 60 s idle; stops inside 1 s hidden and does not accumulate over 60 s; restores in 10.5 ms; 100 rounds with no repeat. The `document.hidden` half could not be produced in this browser and is recorded untested | [M6](../../validation/p3/m6.md) |
| R32 resources | met (GPU as an estimate) | B0 and B2 peaks under budget, a GPU byte ledger that returns to zero when a region is destroyed, B4 growth inside both bounds, zero residue after unmount | [M6](../../validation/p3/m6.md) |
| R33 reliability | met | Two hours, 72,000 actions sent and accepted, none lost, duplicated or errored; four fault classes twenty times each against a written expectation; GPU failure answered under two seconds with at most three retries | [M7](../../validation/p3/m7.md) |
| R34 browser matrix | partial | macOS Chrome is the gate and is unchanged. The rest of the matrix is P4's | [compatibility](compatibility.md) |
| R35 accessibility | partial | The tree over both large loads is correct and checked — row indices, one tab stop, query entries, keyboard reach. The VoiceOver walkthrough (A-4) is **not performed** | [accessibility](accessibility.md) |
| R36 security | met (P3 scope) | No new egress — the sample is generated in memory; the policy unchanged, including on the two-instance page; cell text that looks like markup stays a value | [compatibility](compatibility.md) |
| R38 traceability | met | The fork's `web_gl.js` change registered in prose with the ledger's terms; `sources verify` reports it as drift by design | [compatibility](compatibility.md) |
| R39 diagnostics overhead | met (AC2) | A thousand cross-region actions with the record on and off: p95 45.10 ms against 45.30 ms, a difference smaller than the noise, against a 5% allowance | [M7](../../validation/p3/m7.md) |
| R40 acceptance | partial (P3 closed; full PRD deferred) | `cargo xtask verify --suite p3`, two clean builds of each of four examples, these six reports and this matrix. A-4 is open | [M8](../../validation/p3/m8.md) |

## Plan-level ledger

| ID | Where it landed | Result |
| --- | --- | --- |
| R-1 the large table | ADR-9, §5.1–§5.3 | Delivered: windowing, selection by ID, keyboard reach, tree, jobs |
| R-2 the large scene | §5.4, §6.2 | Delivered: culling, real input paths, absolute camera, query entry |
| R-3 B0, B4 and the fourth example | §6.3, D1/D9 | Delivered; B0 is the default mount of fusion-basic and B4 is two long-lived instances |
| R-4 instance isolation | ADR-8, D14/D15, §5.5 | Delivered: four trap doors, bounded restarts, one URL owner, JS-side listener lifetimes |
| R-5 idle and hidden | §5.6 | Delivered except `document.hidden`, which could not be produced here |
| R-6 documents and reports | §9.5 | Delivered: `docs/data.md`, the architecture and compatibility updates, these seven files |
| NFR-1 performance | §7 | A gate: B0's three figures, B2's frame interval and jobs, B3's frame interval on headed Chrome |
| NFR-2 resources | §5.7 | Delivered; the GPU half is an estimate and says so |
| NFR-3 reliability | §5.5, §8 | Delivered: two hours clean, four fault classes, bounded retry |
| NFR-4 compatibility and usability | §6.2, §9.4 | Partial: the matrix is unchanged and A-4 is open |
| NFR-5 security | §4.2 | Delivered: no new egress, policy unrelaxed, cell text is data |
| NFR-6 maintainability | §4.1, §7 | Delivered: fork change registered, diagnostics overhead gated, clean builds |
| A-1 scope and key decisions | §0.1 | **Closed** by the user, 2026-09-11 |
| A-2 DOM windowing holds the frame budget | ADR-9 | **Closed, holds**: a bare windowed grid measured p95 16.80 ms |
| A-3 measurement environment | D5, §7 | **Closed, headed only**: 50.10 ms under SwiftShader against 17.60 ms headed, so B3's gate is headed Chrome and CI does not run it |
| A-4 acceptance resources | §9.4 | **Open.** The VoiceOver form is prepared and not performed |
| A-5 two instances of one module | ADR-8 | **Closed, holds in full**: separate linear memories, three real traps attributed correctly, the survivor at 100/100 |
| A-6 memory and CPU readings | §5.6, §5.7 | **Closed**: CDP `Runtime.getHeapUsage` is the reading; `performance.memory` is quantised and is corroboration only |
| A-7 a sliced sort inside the budget | D4, §5.2 | **Closed, holds** with about eight times the margin at the probe |
