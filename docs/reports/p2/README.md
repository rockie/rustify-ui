# P2 reports

The six reports the plan's M8 asks for, plus the requirement matrix. Each one
points at the milestone reports under [`docs/validation/p2/`](../../validation/p2/)
for evidence and adds no result of its own.

| Report | What it answers |
| --- | --- |
| [Functional](functional.md) | What the release does, per requirement, and what it deliberately does not |
| [Performance](performance.md) | R29 and R30's B1 figures, **as a gate** since A-3 closed on 2026-09-11, with what the gate does not cover |
| [Compatibility](compatibility.md) | Which browser is the pass gate, what is only observed, and what is pinned |
| [Accessibility](accessibility.md) | Keyboard, semantics, contrast and focus results, and the checks that need a person |
| [Fault and recovery](fault-recovery.md) | Every failure exercised, what the product does about it, and which ones it cannot see |
| [Known limitations](known-limitations.md) | Everything not built, not verified, or done in a way that should not be read as general |
| [Requirement matrix](requirements.md) | The PRD's requirements, status and evidence |

P2 builds a component layer, forms, routing, a workspace, cross-region dragging,
files and two languages on top of P1's fusion runtime. Since 2026-09-11 the
performance report *is* a claim against R29 and R30's B1 figures - the user
approved them as a gate (A-3), and `--project=budget` fails a build that misses
one. Nothing here is a conformance statement of any other kind, and the gate's
own boundaries are stated in that report.
