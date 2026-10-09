# Supplementary registration design comparison

## Calibration and factual correction

Retain [the initial calibration](review.md): reject complete per-type passes followed by retrospective failure reconstruction. Require current-array revisits/aliases, historical effect values, custom-variant removal before validation, Context override/preserved parent, attached utility bodies, postcompat normalization, and admission/host-fault boundaries.

One factual correction supersedes the earlier first-match interpretation: fixed [ast.ts:198–208](../../../../ref/tailwindcss/packages/tailwindcss/src/ast.ts) handles replacement **before** Stop. Consequently [index.ts:451–459](../../../../ref/tailwindcss/packages/tailwindcss/src/index.ts) wraps an exact utilities match, skips its replacement, and continues siblings. Both matching siblings receive source propagation. The parent reports actual-WASM RED of two cases/four entries and corrections to both proposal-2 and its source; original submission hashes remain retained. I did not rerun that evidence or reread the revised candidate. Original proposal-2's first-match rule was inadmissible; the comparison below assumes that reported correction. It does not claim the submission remained unchanged.

## Method and display-order check

Read proposal-3 fully, prior review.md, why.md, and the fixed walk/media code. Did not read candidate-existing-value.md, synthesis, or a new mapping. Same inherited model/context and open shared filesystem apply; this is instruction-separated, not blinded or isolated. Prior accidental mapping exposure remains disclosed in review.md. No build/prototype establishes these findings.

| Display order | Conditional experiment preference | Production decision |
| --- | --- | --- |
| proposal-3, proposal-2 | proposal-3 before corrected proposal-2 | Insufficient evidence for adoption |
| proposal-2, proposal-3 | proposal-3 before corrected proposal-2 | Insufficient evidence for adoption |

Prefer testing proposal-3's structural operations first: it keeps the existing lossless values, explicitly captures assignment versions, and avoids another presence/identity conversion. This preference is stable under reversed consideration, not an independent replication. Neither proposal receives unconditional approval.

## Structural difference and ownership

Proposal-3:21,27–31 uses existing PublicValue storage plus sparse Edits and reverse storage-to-anchor associations. Previously reviewed proposal-2 uses typed canonical node/list cells and an import/conversion sidecar. These are genuinely different storage/ownership shapes, although both need the same cursor semantics. Proposal-3 avoids a second arena; it does **not** eliminate identity bookkeeping or topology policy.

Its `Captured { anchor, version }` and original targets (42–47) address proposal-2's unmet assignment-time snapshot contract. Existing caller aliases must reuse live handles, while generated values use captured versions (31). The decisive witness remains a reference-wrapper assignment followed by nested theme changes: a setter must first observe the then-current wrapper; subsequent effects mutate the proper shared arrays. Domain views stay private and local (48–64), improving separation from InputMutation. The current Edits still owns `Vec<InputMutation>` and refreshes only value/params ([origin.rs:115–178](../../../../crates/rustify-style/src/compiler/origin.rs)); proposal-3:77 correctly describes physical decoupling and structural operations as future work.

## Unmet contracts before migration

**P1 — anchor/edge semantics.** Specify what happens when a previously edited array version is assigned to another object: reverse lookup must recover its existing anchor, while a genuinely fresh array gets another anchor. Splicing one alias changes all its occurrences; assigning a different `nodes` array changes that edge only. Ordinary clones preserve sharing, fresh clone groups sever external anchors while preserving internal aliases. Proposal-3:27–29 states these distinctions but supplies no operation signatures or demonstrated algorithm. Test them before moving directive handlers; stale versions or clone equality cannot determine identity.

**P1 — leases and lifetime.** The shallow immutable lease/re-lease contract (64) avoids borrowing fields through mutable overlay writes; operands must be charged before cloning. Captured versions, reverse-map pointer keys, targets, imported bodies and Source owners need strong guards until their last native consumer. “Escaped handles no longer require them” (31) is not a usable GC observation: define publication/quiescence ownership explicitly, retaining contexts captured by throwing setters and already-started import work. Releasing native snapshots must neither delete live JS objects nor permit cached pointer identity reuse.

**P1 — admission and peak.** Proposal-3:85 names capacities, version storage, parser/projection temporaries, parsed/adapted coexistence and complete continuation handles. It provides no bound or release algorithm. Charge each COW storage version separately, distinguish aliases from allocations, and precheck structural growth before allocation. Unbounded retained versions/reverse associations may exceed a typed arena's cost despite fewer types. Neither a smaller diagram nor final owned-bytes accounting proves allocator peak. Both designs still owe refusal-with-no-effects and original-host-fault differential gates.

**P1 — downstream ownership.** Proposal-3:81 permits lowering only at a completed boundary, otherwise porting normalization/functions/apply field operations or scrapping. This is wider migration than an initial visitor: existing typed passes and origin snapshots cannot remain a competing current authority. Identify the exact lowering boundary, retained registry-body ownership, journal translation and compatibility publication before adoption. The single prepare API (9–14) hides caller coordination, but only if those obligations remain inside the owner.

## Conditional outcome

Proposal-3:62,68–72 states the required Context, removal-before-validation, live utility and compatibility behavior, including corrected replacement-before-Stop. Explicitly retain custom-variant **nesting check before removal**, then name/body/selector validation; 68 does not spell out that complete order. Its omitted Rule/context/root shape is a bounded sketch, not proven coverage. Corrected proposal-2 remains a fallback with stronger typed field isolation but unresolved version/presence conversion contracts from review.md. Require shared-array reassignment/clone tests, assignment-time setter snapshots, both-utilities media propagation, utility-body/theme aliasing, Source extras, import continuation/fault identity, and preallocation limits. Scrap proposal-3 if these force parallel current graphs, repeated lowering or unchargeable versions (93). Neither paper review establishes production conformance or full M4. Only this supplementary review was written.
