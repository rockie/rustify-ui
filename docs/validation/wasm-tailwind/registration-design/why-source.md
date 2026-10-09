# Bounded WHY investigation — compiler and public ownership boundary

## Source and question

Assigned source: local Git, comments, tests and existing in-repository design/audit records. Question: what explicit rationale supports the owned typed `Vec<AstNode>` compiler plus immutable/COW public graph, edit journal and projector boundary, and which contracts should survive an ownership redesign?

This is an evidence packet for independent synthesis, not a design selection. No production edits, builds, tests, outbound writes, external searches or delegation were performed. The only new file is this report. Paths below are repository-relative.

## Code anchor and history

| Target | Symbols / inspected range | History result |
| --- | --- | --- |
| `crates/rustify-style/src/compiler/mod.rs` | `Compiler::new_with_ast`, 208–324 | untracked; combined path log and ls-files empty; blame 208–218 reports no path in HEAD |
| `crates/rustify-style/src/compiler/origin.rs` | `Edits`, `refresh`, 115–180; assignment, clone/projector and tests below | untracked; combined path log and ls-files empty; blame 160–180 reports no path in HEAD |
| `crates/rustify-style/src/compiler/public_api/compile_ast.rs` | `Origins::convert`, 1216–1366; `theme_input_mutations`, 633–685 | untracked; combined path log and ls-files empty; blame 1216–1225 reports no path in HEAD |

Commands executed: `git status --short -- <three paths>`, `git log --oneline -10 -- <three paths>`, `git ls-files -- <three paths>`, and the three bounded blame commands above. Status showed `??` for all three. There are no initial target commits, PR numbers or linked issue IDs to inspect. Original introduction-time motivation, authors and review dates are **Unknown**; absence of Git evidence is not evidence that deliberation did not happen.

## What I searched

- Read the WHY investigator prompt, epistemics and code-archaeology references in `.agents/skills/why/references/`.
- Symbol search: `pub.*new_with_ast|struct Edits|impl Edits|fn convert|fn theme_input_mutations` in the three anchor files.
- Broad bounded rationale search: `ownership|COW|journal|projector|canonical|注册|registration|Vec.*AstNode|immutable|不可变|变更记录|共享可变` in `docs/validation/wasm-tailwind/*design.md`, `docs/plan/WASM-TAILWIND.md`, and the three anchor files.
- Read `ast-input-mutation-design.md:1–69`, historical `custom-origin-design/synthesis.md:1–74`, then reread its precise relevant ranges 7–15 and 47–57. These are existing local design contracts; candidate/proposal files in the new registration design were not read.
- Searched `registration|注册|canonical|view|journal|failure|prefix|ownership` in M4 and migration audit; long search output was truncated, so only passages reread or returned fully are used below. Read `M4.md:9–13,29–33` and plan restoration snapshot `WASM-TAILWIND.md:36–42`.
- Audit bounded search: `rg -n -m 8 'ownership|wholeBook|whole-book|registration|COW|peak' docs/validation/wasm-tailwind/utf16-migration-audit.json`. The resulting lines 4765, 5256–5257, 5266 and 5800 are cited only as explicit stored scope/contract strings, not fresh test results.
- Read current source: `origin.rs:1–182,182–280,326–405,425–488,533–668`; `compile_ast.rs:1–124,625–720,1216–1366,2624–2759`; `mod.rs:208–324`.
- Related test discovery search: `fn .*shared|fn .*alias|fn .*normal|fn .*failure|fn .*prefix|fn .*function|fn .*theme` across origin/public/custom utility/function files; relevant assertions were then read at the ranges above. Test names alone are not used to infer original human intent.
- Location discovery used `rg --files docs .codex` filtered for registration/custom-origin/audit paths. An initial broad `/private/tmp` path discovery encountered a restricted daemon directory; it supplied no rationale evidence and was replaced with repository-local discovery.

## Direct explicit-text evidence

### 1. Immutable public snapshots and explicit host effects were chosen without migrating Rust public values to shared mutability

**[Direct local rationale]** `docs/validation/wasm-tailwind/ast-input-mutation-design.md:22` states: “Native 的 `PublicAst = Arc<Vec<PublicValue>>` 继续表示不可变快照。WASM 通过明确的变更记录操作真实 JS 对象，无需把 Rust 的公共值迁移为共享可变对象。”

Relevance: this explicitly establishes the existing immutable-public-value/journal boundary and avoids a shared mutable public-value migration. It does **not** explain the first historical choice of native Vec, prove that Vec is optimal, or forbid a later coherent redesign.

### 2. Avoiding duplicated facts and representation conversion shaped the prior local choice

**[Direct local tradeoff]** `ast-input-mutation-design.md:59` chooses the single raw candidate cache with “优化归 render，避免两份候选事实；新 CallerGraph/ID 系统增加表示转换”. Line 60 says two raw/optimized trees and capture hints add synchronization/accounting responsibility; line 61 excludes rebuilding caller input from Prepared.ast because it loses internal nodes, array operations and original object identity.

**[Direct historical conditional choice]** `custom-origin-design/synthesis.md:13` adopts immutable node-origin snapshot handles with current-view resolution before deep cloning, reuses PublicValue/native declaration identity/single raw cache, and temporarily declines an external MirrorValue/Object/Array runtime because it requires another complete public graph and broader universal-ID migration. The wording is “条件性实验基础” and “暂不采用”.

Relevance: these are explicit local reasons to limit duplicated representations and migration cost. They are conditional prior design choices, not permanent requirements to retain the current COW tables or typed tree ownership model.

### 3. Independent origin data preserves node fields without manufacturing source spans

**[Direct local constraint]** `custom-origin-design/synthesis.md:9` says SourceLocation cannot substitute for every node origin because Context/AtRoot have no location and true absence of src must remain. Line 55 requires origin to be skipped in Serialize and structural Eq, ordinary Clone to retain it, and fresh clone to refresh it.

Current module wording agrees: `origin.rs:1` says “Exact public node fields, independent of source spans and native declaration IDs”; lines 48–50 describe opaque provenance ignored by serialization and native structural equality.

Relevance: separate CSS structural behavior, provenance identity and source-presence semantics are explicit. The exact NodeOrigin representation is not the externally observable requirement.

### 4. Refresh exists to respect edits in the present phase without overwriting other typed transforms

**[Direct source rationale]** `origin.rs:160` says: “Refresh only after an explicit edit in this phase; preserve other typed transforms.”

**[Direct local requirement]** `custom-origin-design/synthesis.md:57` requires all extracted definitions/theme/keyframes/repeated references to resolve current views; aliased native occurrences must read current values before guards and synchronize typed fields. It explicitly warns that rebinding origin alone can leave guards seeing stale values, requires separate differential verification, and says to abandon the structure if the contract cannot be met rather than add CSS/position heuristics.

Relevance: an ownership redesign must retain current-value read-after-write behavior and avoid silently undoing independent native edits. This is stronger than preserving a particular refresh helper.

### 5. Private clone aliases and live caller references need distinct projection handling

**[Direct source rationale]** `origin.rs:425–426` states: “Canonical current views keep private cloned array aliases together. Caller metadata retains its host references, whose ordered effects are replayed separately.”

**[Direct local explanation]** `ast-input-mutation-design.md:51` states that unchanged caller storage returns a live JS handle “避免后续Source metadata把先前apply的caller数组恢复成旧内容”.

Relevance: output projection must not resynchronize caller metadata references to old immutable snapshots after earlier effects have already mutated those host arrays. Private generated graphs still require internal alias reconciliation. The evidence explains the existing distinction, not the only permissible implementation.

### 6. Journaling preserves observable write order, targets and failure prefixes

**[Direct local contract]** `ast-input-mutation-design.md:37` requires first theme replacement by original-array index, later themes removed by original splice, and publishing the new rule with empty children before later writes so setter/splice failure leaves the correct successful prefix. Line 41 preflights raw trees, mutations and optimized output before candidate commit and uses Reflect.set/real splice with the original receiver. Line 43 distinguishes a host crossing failure from resource rejection and says not to claim rollback of existing JS writes.

**[Direct historical ordering contract]** `custom-origin-design/synthesis.md:51` separates initial theme replacement/removal, registration normalization, final theme/keyframe assignment, variant writes, functions, apply, utilities writes and utility removal. It requires normalization assignments even if the resulting text is equal. Line 53 requires exact semantic failure prefixes, resource admission before replay, host-error precedence and no half-initialized compiler publication.

Current API comments express the same separation: `compile_ast.rs:21–22` says hosts apply ordered caller operations separately from optimized output; `:80–88` says a failed constructor has no compiler to publish, hosts preflight its journal/graph reserve and crossing handles, and an earlier host write error supersedes the semantic error.

Relevance: preserve semantic stages and error boundaries; do not preserve an incomplete phase grouping merely because it is present in current code.

### 7. Budget and clone scope are still explicit unfinished obligations

**[Direct local contract / limitation]** `custom-origin-design/synthesis.md:57` requires fallible origin-aware clones, reserving actual peak before proportional allocation, retaining only chosen-root reachable data including detached metadata/live versions, and forbids every candidate retaining the whole input. `origin.rs:340–341` explicitly reserves provenance before COW field views while saying the session-wide temporary/staging meter remains separate.

The audit repeats the unresolved scope at `utf16-migration-audit.json:5266`: conservative checked clone/retained fees do not certify all live compiler/COW/memo/staged peak accounting, snapshot pruning or allocator instrumentation. Its line 5800 lists whole-book pruning and unified core/facade/COW/memo/journal/crossing peak accounting with conservative overlapping charges as work remaining. These stored limitations are not new resource measurements.

Relevance: ownership changes need explicit retained and temporary accounting; existing logical fees are not a certification of whole-session allocator bounds.

## Indirect evidence and current mechanics — not original motivation

- **[Supported contract] CSS structure and provenance are separated by tests.** `origin.rs:534–571` attaches arbitrary origin fields to all native node forms, asserts unchanged native equality/serialization and ordinary clone sharing. `:575–626` tests current child value across body/metadata/src/dst aliases and independent fresh clone identity. The assertions support retaining these behaviors; they do not establish why Vec was originally chosen.
- **[Supported contract] Aliased guards must observe earlier writes.** `origin.rs:630–648` applies functions to two native occurrences of the same public declaration and asserts one mutation, matching current values and removed function syntax. M4's explicit failure explanation at `M4.md:33` records a repeated setter when custom current views were not propagated to an ordinary guard; line 11 records replacement with one native-position traversal/current view/journal. These converge on a current-value contract. This investigation did not execute the test or historical RED.
- **[Supported contract] Semantic failure effects target original objects/arrays while public snapshots remain unchanged.** `compile_ast.rs:2624–2666` asserts one completed assignment on the original first declaration and an unchanged immutable input; `:2670–2703` retains an ordinary assignment before a custom failure; `:2707–2736` retains only a completed apply replacement on its original array. `:2740–2757` rejects below the failure reserve without authorizing a prefix. These are observed assertions, not a fresh pass verdict.
- **[Observed mechanics] Native setup owns and mutates a typed Vec.** `mod.rs:208–256` expands imports, extracts sources/custom variants/custom utilities/theme by mutable native references. `:299–316` prepares theme, validates/applies variants, then captures normalization/functions in failure or success. This shows stage arrangement; no nearby explicit text explains why owned Vec rather than a canonical shared graph was the initial representation choice.
- **[Observed mechanics] Public conversion snapshots typed fields and stores graph identity separately.** `compile_ast.rs:1216–1366` recursively creates native occurrences from PublicAst, adds NodeOrigin plus source/destination, and retains declarations/authored tables. `theme_input_mutations:633–685` later walks public/import views and records theme operations, skipping utility/custom-variant bodies. Its former comment incorrectly attributed that skip to upstream parseCss: upstream `index.ts:219–250` queues a utility without Skip, and `ast.ts:136–220` descends into it. The parent corrected that comment after the investigation; behavior remains unfixed. This is the current representation boundary; source mechanics alone cannot justify its historical intent.

## Contradictions, tradeoffs and open questions

No textual contradiction was found in the explicit semantic contracts. There is a material distinction between the **historical conditional representation choice** and the **current redesign requirements**:

- Historical records prefer immutable snapshots and attached origins to avoid a second graph/migration. The same historical contract says to abandon that structure if current-view/alias/peak obligations cannot be met (`synthesis.md:13,57`). Preserve those obligations, not an unconditional ban on redesign.
- `Edits.refresh` explicitly avoids overwriting typed transforms except where this phase edited them (`origin.rs:160`), while the design demands shared occurrences see newest public values before guards (`synthesis.md:57`). Which ownership model can satisfy both across registration/theme/variant/source stages remains a design question; no evidence establishes that the current phase-local view scope already does so.
- The plan still names early registration/theme/keyframe/variant/source failure prefixes, normalization/variant/theme shared views and dynamic shared/Proxy walk as next work (`WASM-TAILWIND.md:40`). Current success/selected failure tests cannot prove those missing stages.
- Original Vec/public COW split motivation, original author deliberation and external consumers are **Unknown**. Target Git history is absent, and the historical synthesis itself marks external consumers unknown (`synthesis.md:55`). No concrete PR/ticket/Page anchors were identified.

## Evidence coverage and gaps

- **Source-control history:** local Git queried for the three exact targets; empty tracked-file/log results, failed HEAD blame, zero target commits/PRs reviewed. Current comments/tests/local records provide evidence.
- **Issue tracker / GitHub:** available capability per parent map; not queried. The assigned target is untracked, not-yet-published local accumulated work with no PR/ticket ID. Remote issues have no anchor to this target; unrelated repository/account searches are irrelevant to this bounded rationale investigation. No claim that all informal historical motivation is absent.
- **Long-form / ChatGPT Spaces:** local design/plan/audit records were read. No target Page/Space identified or queried; no account relevance was checked. Available unrelated remote spaces cannot supply target evidence without a link/identifier.
- **Team chat, infrastructure observability, external error tracking and analytics:** no matching available MCP identified by the parent's coverage map; not searched. Runtime metrics/production incidents cannot establish the original rationale for these unpublished local module changes within this bounded scope.
- **Execution gap:** no build/test/benchmark ran. Assertions and stored historical statements are cited as source evidence only. Full current registration-stage failure prefixes and peak scope remain unverified here.

## Preserve / Change / Avoid / Risk evidence constraints

- **Preserve:** represented full fields, true src absence, Source and metadata aliases, fresh clone boundaries and shallow optimized wrapper/declaration sharing; original caller targets and exact ordered writes including same-value assignment; latest alias values before guards; resource rejection before writes and semantic/host error precedence; one raw candidate fact store with owner-retaining pointer indexes. Sources: local design `:26,37–43,49–51,59–61`, historical synthesis `:7,47,51–57`, current origin tests above.
- **Change permitted:** native ownership, snapshot/view/journal/projector internals and shared graph representation are implementation choices if the semantic contracts and resource obligations are met. Historical temporary non-adoption of a mirror is not an eternal prohibition. Source: synthesis `:13,57`; original Vec rationale unknown.
- **Avoid:** recovering exact origin solely from CSS/position heuristics; using source spans as all-node provenance; rendering-only diffs to represent constructor effects; reapplying immutable caller snapshots after live effects; half-compiler publication or replay before reserve checks; using narrow logical fees as proof of whole allocator peak.
- **Risk:** shared occurrences spanning normalization/theme/keyframes/variants/source phases may diverge between typed fields and public current views; replay order may omit early semantic prefixes; COW graph views may retain unrelated input or incur overlapping fees; representation migrations may touch public enum literal consumers whose external existence is unknown. These are bounded design risks, not newly reproduced defects.

## Additional leads for the parent synthesizer

The registration redesign should independently trace the exact early setup walk/registration event order against pinned upstream and evaluate whether one owner can cover typed/public fields without weakening clone/COW identity boundaries. Existing local documents provide conditions and prior costs, not proof of a new candidate. No additional remote source is identified.

## Confidence summary

**Direct:** explicit local reasons for immutable public snapshots/journaling, avoiding duplicated graphs/facts, independent origins, selective typed refresh and live caller/private projection separation. **Supported:** current tests plus documented historical counterexamples establish alias, identity, prefix and admission contracts. **Inferred:** extending or replacing phase-local ownership may be required to close early registration/shared-view behavior. **Unknown:** first introduction intent for typed Vec/COW split and any unpublished private discussion. Parent synthesis should keep these confidence classes separate.
