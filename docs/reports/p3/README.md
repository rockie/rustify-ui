# P3 reports

The six reports the plan's M8 asks for, plus the requirement matrix. Each one
points at the milestone reports under [`docs/validation/p3/`](../../validation/p3/)
for evidence and adds no result of its own.

| Report | What it answers |
| --- | --- |
| [Functional](functional.md) | What the release does, per requirement, and what it deliberately does not |
| [Performance](performance.md) | R29's B0 column and R30's B2/B3 columns **as gates**, with what each gate does not cover |
| [Compatibility](compatibility.md) | Which browser is the pass gate, what is only observed, what the instance model costs, and what is pinned |
| [Accessibility](accessibility.md) | Reaching a row nothing has drawn: keyboard, semantics and the query entries, and what still needs a person |
| [Fault and recovery](fault-recovery.md) | Every failure exercised — including two hours of not failing — and which ones the product cannot see |
| [Known limitations](known-limitations.md) | Everything not built, not verified, or done in a way that should not be read as general |
| [Requirement matrix](requirements.md) | The PRD's requirements, status and evidence |

P3 is the loads-and-boundaries phase. P1 delivered the runtime and P2 the
application skeleton; P3 turns the PRD's remaining fixed loads into fixtures
that can fail a build — B0 the smallest complete application, B2 a hundred
thousand rows, B3 ten thousand GPU objects, B4 two instances left running for
two hours — and turns "one instance traps, the others carry on" from a
paragraph in a document into a mechanism with a test.

Since 2026-09-11 the performance report *is* a claim: the user approved R29 and
R30 as gates, and `budget`, `budget-minimal` and `budget-data` each fail a
build that misses one. What these gates do **not** cover is stated in
`tests/browser/budgets.ts` and repeated in [performance](performance.md); read
that before quoting a pass. Nothing here is a conformance statement of any
other kind.

**This is not the full PRD.** P3 closes its own scope. The Windows, Linux and
mobile matrix, a WCAG 2.2 AA review, the migration samples, the developer
study and the deprecation window are P4's, and are listed in
[known limitations](known-limitations.md).
