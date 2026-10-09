# Anonymous registration design review

## Calibration recorded before candidate access

The decision rule is behavioral admissibility before interface preference. Reject a design that finishes whole-tree media/source/theme passes and reconstructs an early failure prefix afterward: a later media write or validation would already precede the earlier semantic error. This is the existing failure mechanism described in [ground.md:19–25](ground.md), [ground-native.md:74–79](ground-native.md), and [ground-public.md:151–152](ground-public.md).

Only a conditional sketch can be approved at this stage. It must explicitly provide:

- A mutation-aware current-array cursor: replacement Continue revisits the same position; Skip suppresses replacement descent; deletion updates later indices; semantic failure stops later siblings.
- Context field override, including explicit `reference:false`, while preserving the semantic parent and transparent descent.
- Custom-variant nesting validation followed by removal **before** name/body/selector validation, with legacy name assignment retained before that removal.
- Attached live utility bodies throughout initial traversal; normalization and registry-owned extraction happen only after compatibility.
- The initial empty theme rule replacement before compatibility and final theme nodes assignment after normalization, as separate caller-visible writes.

These constraints come from [ground.md:25–41](ground.md), [ground.md:54–58](ground.md), [ground-native.md:83–89](ground-native.md), and [ground-public.md:54–65](ground-public.md). Imports remain before this walk; existing Program/Machine reaction order, original host exceptions, captured context identity, and already-started work after rejection remain owned by the existing setup boundary ([ground-public.md:89–109](ground-public.md)).

Resource and alias claims are separate proof obligations. Admission must cover the complete potential journal and retained crossing handles before start and must reject Limit without replaying AST writes; a final retained-byte total is not a temporary allocation peak proof. Object identity, current object fields, original array identity, current array contents, and occurrence position must not be collapsed into one identifier ([ground.md:68](ground.md), [ground-public.md:137–154](ground-public.md)). A shallow facade exposing coordination steps, duplicated directive policy between native/public modules, or pass-through layers is a reason to revise under [design-red-flags.md](../../../../.agents/skills/architect/references/design-red-flags.md).

## Evaluation conditions and limitations

This reviewer did not author the current registration candidates. The review inherits the same available model and historical parent context; it is not a different-model or context-isolated evaluation. The workspace is an open shared filesystem. Candidate separation is enforced by instructions, not by a blind environment or access isolation. Author/model/preference identity is not a ranking input.

Read before candidate access: `manifest.json`, `ground.md`, `ground-native.md`, `ground-public.md`, architect `SKILL.md`, and `design-red-flags.md`. The manifest records bounded actual-WASM RED as 36 scenarios / 72 comparisons / 68 failure records, with 8 wrong-error and 64 prefix mismatch comparisons; those categories overlap, and full registration conformance remains pending. This review has not run tests, builds, browsers, or services and does not independently reproduce those results.

The calibration above was written before receiving or reading either proposal. Subsequently, the two authorized files `proposal-1.md` (130 lines) and `proposal-2.md` (198 lines) were read completely. Neither `candidate-native.md` nor `candidate-graph.md` was opened. While reading the subsequently added manifest comparison rubric, the selected 110–190 line range also displayed its source-to-proposal mapping at 148–160. That exceeded the requested reading boundary and was reported immediately to the parent. Consequently this review does **not** claim that mapping remained unseen. No author/model/preference identity was supplied or used; conclusions below cite proposal content rather than source labels.

## Decision and both display orders

Both sketches pass the recorded semantic calibration as **conditional candidates**. Neither establishes production correctness or is ready for adoption without the interface and resource gates below. Prefer proposal-2's domain-effect boundary, with proposal-1's explicit immutable effect-value requirement carried into it. This is a conditional design preference, not acceptance of proposal-2's current signatures as complete.

| Display order assessed | First preference | Second preference | Reason independent of display position |
| --- | --- | --- | --- |
| proposal-1, then proposal-2 | proposal-2, conditional | proposal-1, conditional/revise boundary | Initial grammar owns typed domain effects rather than public InputMutation; historical effect values still need an explicit contract. |
| proposal-2, then proposal-1 | proposal-2, conditional | proposal-1, conditional/revise boundary | The same domain-boundary reasoning applies; proposal-1 still provides the clearer snapshot-value statement. |

The ordering is stable under reversed presentation. This same-reviewer reversal is a procedural check, not an independent blinded replication; it does not remove the disclosed context or accidental mapping exposure.

The proposals converge on the same structural solution: canonical node/list cells, a cursor reading current identities, queued live utility bodies, COW provenance, then native lowering. Compare proposal-1:41–60,84–98 with proposal-2:53–107,149–161. “Graph authority” and “native authority” therefore name a conversion/effect ownership difference, not two demonstrably distinct traversal architectures. Do not claim that this exhausts the design space or that two separate executable designs were compared.

## Calibration applied to the actual sketches

| Non-negotiable behavior | Proposal-1 evidence | Proposal-2 evidence | Paper finding |
| --- | --- | --- | --- |
| Current-array deletion/revisit; aliases remain current | 86: read each step, no cloned lists or global visited set, shifted successor and Continue/Skip behavior | 53,151: canonical shared lists, per-iteration current cells, immediate replacement and revisit | Explicit in both; conversion and shared-array differential still required. |
| Context override and preserved semantic parent | 86: explicit `reference:false`, Context does not replace parent | 80–84,149,151: presence-aware patch intent, replacement merge, preserved parent | Explicit intended behavior in both; proposal-2's presence representation needs refinement below. |
| Legacy Rename; custom-variant removal before validation | 92 | 151–153 | Both satisfy the ordering calibration; no validate-all-before-remove policy proposed. |
| Utility body remains attached and live through initial walk | 59,92,96 | 99,153,157 | Both queue current body identities and defer normalization; no detached snapshot is presented as authoritative. |
| Media only when reached; targeted subwalks remain | 94 | 155 | Neither proposes completing global media/source/theme passes before reconstructing failure. |
| Initial theme replacement separate from postcompat publication | 94–96,102 | 157–159 | Both retain the empty-rule versus final-nodes split and stop on early semantic failure. |

Evidence in this table is what each sketch specifies. It does not certify that current production code obeys it or that an unspecified adapter can implement it without changing these promises.

## Proposal-specific findings

### Proposal-2: preferred boundary, incomplete historical-value and shape contracts

**P1 — historical effect values must be represented before adoption.** `InitialEffect::Children { body: ListId }` and `Replace { replacement: Vec<NodeId> }` name current cells (102–107), and `project_native_setup` translates after construction (139–141). The same cells intentionally continue changing (53,151). Its COW statement (161) does not say which storage version an already-recorded effect resolves. A concrete witness is a media reference children assignment followed by a nested theme replacement: a setter observing the first assignment must see that assignment's then-current generated value, rather than a wrapper reconstructed using final children. Later assignments to the original shared children must still address the same live caller array. Proposal-1:88 explicitly requires immutable effect-value storage snapshots; carry that requirement into proposal-2 using a captured value/storage version distinct from target identity, or project and freeze values at effect emission. No extra directive grammar belongs in the projector. This is an unmet interface condition, not proof that every possible implementation of the sketch fails.

**P1 — conversion must preserve semantic field presence and occurrence topology explicitly.** `InitialNode::AtRule` always has `body: ListId` (72), although public/native at-rules can have absent bodies; `InitialValue::Undefined` retains the previously skipped declaration case, but does not establish the rest of node-field presence (76–82,149). `ContextPatch` distinguishes absent from a scalar false/empty value, but `Option<Text>`/`Option<bool>` alone does not describe present undefined/null values where the public graph admits them. Opaque provenance may preserve raw fields, but semantic code needs a declared way to read the distinctions it uses without consulting public wire grammar. Specify optional/presence-aware body/context contracts and the supported admission boundary. The import sidecar named at 149 must preserve original array slots—including undefined declarations—and update by occurrence at completion; it cannot infer public positions from lowered native indices (ground-public:34–38). Reject a prototype that erases those distinctions or adds a second directive classifier in conversion.

**P2 — targeted early-stop and later wire ownership remain underspecified.** Media subwalks require Stop/Skip (155), but `Advance` has no Stop variant (85); state the separate targeted-walk control result or add it. `NativeSetupFailure.later: Vec<InputMutation>` (120–125) also contradicts an unrestricted reading of the “no public wire types” screen (186). Initial grammar is better separated than proposal-1, but later public mutation ownership should stay in the public adapter or have a deliberately bounded existing provenance boundary. Do not spread InputMutation policy back into domain handlers merely to make failure projection easier.

### Proposal-1: stronger effect snapshot statement, public representation in the core

**P1 — public mutation representation leaks into shared registration ownership.** `RegistrationGraph.effects` and `RegistrationDraft.effects` are `Vec<InputMutation>` (44,63), while domain cells are supposed to serve both native and public input (67–74,84). This makes the shared grammar's recorded changes depend on public target/value representation even when no caller graph exists. The proposal says domain helpers do not receive replay bookkeeping (88), which limits the leak but does not remove it from the module owning grammar and failure policy. A wire mutation change would require coordinated changes in registration and projection, the information-leakage concern in design-red-flags:17–21. Prefer an internal domain effect with opaque provenance and immutable captured values; translate to InputMutation at the existing public boundary. Keeping immutable snapshots is essential—the domain-effect revision must not discard proposal-1's useful 88 contract.

**P2 — native usage must hide deferred-stage coordination and resolve prepare/register semantics.** The claimed single operation still asks the native caller to perform `register`, `after_compatibility`, and `from_registered` (17–20), whereas 33 calls `register` the convenience projection of `prepare`. Clarify whether prepare has already executed deferred registration and how later semantic failures retain their prefix; never normalize twice. Compatibility is a real protocol boundary and is not itself a red flag, but callers should not coordinate duplicated registry invariants or learn graph lowering stages. Put the complete native convenience operation behind one policy-bearing boundary, while the existing setup machine retains the explicit public reaction split.

**P2 — adapter claims are intentionally abstract, not demonstrated.** Retaining omitted/undefined fields and shared public arrays is stated at 84; loaded current import replacements are delegated to `ImportBindings` at 102. Before migration, specify how those identities/slots survive native inputs already converted with omissions, how imported aliases bind by occurrence, and how lowering preserves current field snapshots. The proposal's own 122 questions correctly leave these open. Its explicit effect snapshots make this aspect clearer than proposal-2, but names such as OriginBinding/ImportBindings do not constitute a proven bridge.

## Shared resource, alias, and migration gates

Both sketches honestly reserve graph/native coexistence and distinguish logical reserve from measured allocator peak (proposal-1:104,112; proposal-2:174–178,190). Neither provides allocation bounds for arena/index growth, cursor/context cloning, targeted-walk temporaries, value snapshots, journal growth, normalization temporaries, or conversion/lowering coexistence. Do not infer peak safety from a final owned-bytes sum. Admission must include failure-retained graph storage, generated values captured by throwing setters, complete potential import continuations and crossing handles **before** start; resource refusal authorizes no AST write (ground-public:89–109,142–154).

The following are adoption conditions, not optional confidence scores:

1. Cursor differential: current shared-array deletion through two Context aliases; explicit false/empty overrides; replaced node revisit versus Skip; original parent through Context; custom-variant Rename/removal before invalid validation; early error preventing later media setters. Utility nesting must read the current body after earlier aliased changes.
2. Effect-value differential: observe values in setters at their assignment time, then observe later mutations of the same captured arrays/objects; verify Source/src/dst canonical aliases and extra fields, plus detached original theme/keyframe/utility body identities. Use the exact original numeric Set versus caller-owned splice behavior; do not replace it with final-array projection.
3. Import and phase differential: original slot/index mapping with skipped/undefined declarations, imported occurrence identity, empty context begin and same-context completion, ParseCss initial prefix versus Compatibility deferred range, and continued already-started jobs after rejection. No final theme.nodes or normalization may be replayed after an initial semantic failure.
4. Admission and fault differential: native and crossing shared reserves at limit boundaries, refusal with no AST effects, earlier original host exception overriding pending semantic error, and retained captured values after a throwing setter. A logical byte model and an allocator experiment answer different questions.
5. Migration: remove replaced eager passes only after equivalent gates pass; later functions/apply/variants must consume one frozen current view rather than separately synchronized semantic graphs. Both candidates' scrap rules are sound (proposal-1:124; proposal-2:198): duplicate grammar, repeated body synchronization, or accumulating alias exceptions require redesign.

The isolated cursor experiment may confirm or falsify these conditions. This paper review cannot establish production conformance, full M4, unified ledger/peak correctness, live Proxy/loader behavior, performance, or browser acceptance. The manifest's bounded RED and pending status remain unchanged by this review. Only this review file was written; no implementation or build was performed.
