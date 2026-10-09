# Phase A — source-control and rationale ground

Investigation target: `compiler/public_api::Origins`, `ast::clone_fresh`, `SourceLocation.public`, and the raw candidate cache/projection shape. Scope is local Git plus source comments, tests and relevant local design records. No code edits, builds, test execution, remote lookup or external writes were performed.

## Historical anchor and source coverage

All three target files are untracked in the inspected checkout:

| Target | `git status --short` | `git blame -L 1,85` | `git log --follow --oneline -20` |
| --- | --- | --- | --- |
| `crates/rustify-style/src/compiler/ast.rs` | `??` | fatal: no such path in HEAD | empty |
| `crates/rustify-style/src/compiler/public_api/compile_ast.rs` | `??` | fatal: no such path in HEAD | empty |
| `crates/rustify-style/src/compiler/public_api/source_map.rs` | `??` | fatal: no such path in HEAD | empty |

`git ls-files` for the three paths and `git log --all --oneline -- <three paths>` were also empty. This establishes **Unknown** original commit authorship, original review discussion and introduction-time motivation. It does not establish that these decisions lacked deliberation. No target commit, PR number or issue identifier was found, so no remote PR/issue lookup was justified for this investigation.

Coverage map supplied by the parent investigation, distinguished from sources actually queried here:

- Source control: Git and GitHub capabilities are available; local Git queried as above; no relevant target commit/PR exists in the local history.
- Issue/ticket tracker: GitHub/SPMS capabilities are available, but no associated issue identifier has been identified. Relevance gap; not a searched-and-empty ticket result.
- Long-form documents: local repository documents read below. No target Page/Space was identified or queried; no account relevance was checked.
- Team chat: no matching available tool; conversational decision record unknown.
- Infrastructure observability: no matching available tool; no runtime incident evidence collected.
- Error tracking: no matching available tool; no exception service history collected.
- Product analytics: no matching available tool; no analytics evidence collected.

## What I searched and read

- `.agents/skills/why/SKILL.md`, `references/epistemics.md`, `references/investigator-prompt.md`, and `references/sources/code-archaeology.md`.
- Repository context: `README.md` initial development sections, `Cargo.toml`, `mise.toml`, `PRODUCT.md`, and `DESIGN.md` opening product/style context. These establish repository context, not compiler-origin rationale.
- Local target Git status, tracked-file lookup, blame, follow history and all-ref path history, listed above.
- Exact text search: `Origins|clone_fresh|SourceLocation|candidate.*cache|cache.*candidate|origin|provenance` in target files, M4 and product/design context; follow-up `Origins|provenance|origin` in local design records.
- `docs/validation/wasm-tailwind/ast-input-mutation-design.md` lines 1–65; current and explicitly historical M4 passages at lines 5–26, 49–51, 96–100, 257–269; `docs/plan/WASM-TAILWIND.md` lines 242–249.
- `ast.rs` lines 1–66, 288–363 and 593–678; `source_map.rs` lines 1–72 and 698–734.
- `compile_ast.rs` lines 1–79, 128–137, 402–524, 525–855, 1031–1034, 1070–1140 and 1426–1474; `utilities.rs` lines 595–634.
- `tests/utility/compiler/run-public-ast-custom-input-wasm.mjs` lines 1–92, including the full constructor/build graph fixtures; clone call sites found in `custom_utilities.rs:470`, `custom_variants.rs:163,525,531`, and `apply.rs:447,449` with source replacement at `apply.rs:451`.

## Direct evidence: explicit current rationale or constraints

1. **Unique raw candidate cache; optimization belongs to rendering.** `ast-input-mutation-design.md:55` states: “优化归 render，避免两份候选事实；新 CallerGraph/ID 系统增加表示转换” and adopts raw caching without a second graph type. Its line 56 rejects a second native candidate cache and capture configuration because two trees increase synchronization/accounting responsibility. Line 57 explicitly rejects reconstructing caller input from `Prepared.ast`: “代码较少，但丢失内部节点、数组操作和原对象身份”. These are local design statements, with original human authorship/date unavailable in Git.

2. **Observable unoptimized children motivated the raw cache correction.** `M4.md:49` records that sort markers, duplicate declarations and an empty wrapper were prematurely reduced from four nodes to one, then describes keeping the single raw cache and optimizing during render. The current source comment in `utilities.rs:631–632` states: “compileAst exposes these cached children before optimization. Rendering owns pruning; it shares declaration identities with this raw tree.” This is direct current rationale, independent of unknown original history. The historical RED log was not executed or independently revalidated here.

3. **Pointer maps must retain their owners.** `ast-input-mutation-design.md:26` explicitly states “指针索引同时持有对应 Arc，防止地址复用” for raw candidate public projections, and says raw projection and optimized output share a declaration identity table. Line 39 similarly requires an independent canonical Source wrapper owner for each original apply directive and says shared tuple content must not become node identity.

4. **Canonical caller provenance cannot be reduced to typed offsets.** `source_map.rs:19–20` documents `SourceLocation.public` as “Canonical metadata supplied through the public AST, including shared Source objects and JavaScript number offsets.” `compile_ast.rs:589–590` states: “The typed span tracks relative candidate offsets; the public tuple retains the caller's full Source graph.” `source_map.rs:13` explicitly fixes offsets to UTF-16 units.

5. **Bind authored occurrences before destructive rendering.** `compile_ast.rs:663–665` says initialization may replace apply children and binds authored occurrences to declaration identities surviving those transformations “before render prunes empty/reference rules or flattens wrapper contexts.” This explains the binding boundary; it does not justify every key/shape matching heuristic.

6. **Identity cannot be replaced by equal Source content.** `M4.md:257` explicitly states “相同内容的不同 Source 不按字符串合并”. The current map module documents authored/generated source identity at `source_map.rs:1`. Equal textual content is not an allowed general deduplication key.

7. **Keep caller mutation effects distinct from optimized output and preserve error boundaries.** `ast-input-mutation-design.md:22` retains immutable native `PublicAst` snapshots and explicit mutations for real JS objects. Its line 41 requires preflight of raw public trees, mutations and optimized output before candidate commit; quota rejection must not publish caches, raw identities or JS writes. Line 43 says host crossing failure keeps admitted candidates/counters and only restores the last successful output; it must not be presented as quota rejection or rollback of JS writes. `compile_ast.rs:1031–1034` restates that crossing rule in source.

8. **Existing coverage is incomplete.** `M4.md:269` explicitly states “compileAst现有Origins重建不等于完整provenance”. Current `M4.md:15,26` names custom definition/body/input-array behavior and broader setup/raw/helper/API work as remaining; `ast-input-mutation-design.md:65` requires continued complete input graph differential checks and says successful-event replay alone does not prove getter/setter/proxy timing. Earlier completion statements in M4 are labeled historical and narrower in scope.

## Supported evidence: current behavior, not original motivation

- **Fresh clone boundary is intentional in the current tested contract.** `ast.rs:593–632` tests public Source graph metadata charging, alias sharing within one clone, and distinct Source identities across original and successive clones. `ast.rs:658–666` tests declarations equal by value while preserving aliases within a clone and obtaining a new identity graph relative to the original. `ast.rs:308–363` implements separate per-call maps for declaration, Source and PublicValue identities. The code and tests strongly support this as a present behavioral requirement; no original discussion explains why this same behavior should apply to caller provenance in every transformation.
- **Public provenance carries information not present in typed `Source`.** `source_map.rs:8–21` keeps typed `Source { file: Option<String>, code: Text }` separately from optional canonical public data. `compile_ast.rs:128–137` uses a zero/default typed carrier while retaining the supplied PublicValue; serialization prefers public data at `source_map.rs:66–72`; `Origins::location` returns public data before typed lookup at `compile_ast.rs:430–443`. This supports preserving arbitrary public graph fields and JS number offsets through the public path, without claiming typed fields alone represent them.
- **Raw and rendered declarations intentionally share stable public objects.** `compile_ast.rs:1426–1474` tests four cached raw children (including `--tw-sort`, duplicate declaration, empty `&` rule), one optimized output declaration and identity equality with the generated declaration projection. `Origins.raw_candidates` at line 77 holds `(Arc<CompiledCandidate>, Value)`; build reuses those projections by candidate identity at lines 1117–1135. These are observable ownership/shape facts; no tests were run in this investigation.
- **Origins presently reconstructs nondeclaration authored matches heuristically.** `compile_ast.rs:651–656` stores authored fields keyed by `OriginKey`; lines 718–742 match output nodes using unused occurrence, intersecting declaration IDs or CSS shape, with ranking by shape and overlap. This is a current mechanics observation, not evidence of full provenance or of its original intended correctness envelope.
- **Custom input verification compares alias graphs, not only JSON/CSS.** `run-public-ast-custom-input-wasm.mjs:27–38` records object references, ordered own fields and undefined; lines 41–57 retain caller definitions, old arrays/children, extra fields, shared Source metadata and fractional spans; lines 59–88 compare constructor and every build graph against fixed Tailwind. This strongly supports complete graph fidelity as the present acceptance shape. Its `pending` field at line 92 is a runner annotation, not a newly observed test result.

## Inferred implications for the next design

- Given the full Source graph contract and current fresh clone isolation tests, a custom-definition provenance correction likely needs an explicit boundary between fresh generated identities and alias relationships inside each cloned Source/tuple graph. This is an inference; there is no historical statement authorizing globally changing `clone_fresh` semantics. The later Phase A public exploration resolves original-versus-cloned identity against pinned upstream structuredClone semantics; see grounding.md.
- Given the recorded inability to reconstruct internal nodes from optimized output and the explicit single-cache selection, additional origin information likely belongs alongside the raw native traversal/projection rather than in a second optimized candidate fact store. This is supported direction, not a mandated new type or API.
- Given occurrence/shape matching and M4's explicit incomplete-provenance warning, relying on CSS equality to recover custom template origins appears risky when equal-value objects or repeated definitions differ by metadata/aliasing. A targeted graph counterexample is needed before attributing any specific failure to that heuristic.

## Contradictions and unresolved questions

No direct contradiction between current explicit design statements was found. In this history-only slice, canonical public Source graph preservation did not establish whether the graph should retain original identity or get clone identity. The subsequent public exploration supplies pinned upstream structuredClone evidence: clone-local aliases must survive while original and independently cloned graphs remain distinct. This reconciles public graph preservation with freshening; the carrier representation remains unresolved.

The origin of `Origins`' key/shape heuristics, the original private discussion, and why caller Source identities were freshened in particular custom paths are **Unknown**. There is no relevant Git lineage to recover them. Broader getter/setter/proxy failure prefixes, custom definition/input-body behavior and full allocator/temp/crossing budgets remain explicitly incomplete in local records. No associated SPMS/issue/PR was identified; no external source can be claimed as searched.

## Preserve / Change / Avoid / Risk constraints

**Preserve**

- Unique raw native candidate facts, raw internal nodes/sort/empty wrappers, and rendering-only final pruning (`utilities.rs:631–634`; design lines 26,55–57).
- Stable cached raw projections, shared raw/rendered declaration identities and Arc owners for pointer indexes (design line 26; `compile_ast.rs:77,1117–1135,1467–1473`).
- Canonical PublicValue provenance, arbitrary caller Source metadata, shared versus equal-but-distinct object distinctions, JS number offsets and UTF-16 span units (`source_map.rs:13–21`; `compile_ast.rs:589–590`; map test `source_map.rs:698–734`).
- Existing per-clone alias preservation/fresh generated identity behavior unless a narrower boundary is explicitly justified (`ast.rs:593–632,658–666`).
- Precommit quota checks, old output on rejection, and crossing-error behavior distinct from quota rejection (design lines 41–43).

**Change candidates, not predetermined fixes**

- Replace proven ambiguous custom origin reconstruction with explicit provenance at the transformation boundary, if graph evidence demonstrates a collision; retain necessary existing authored output behavior while changing that boundary.
- Specify fresh generated AST/declaration identity and clone-local provenance aliases together where fixed-upstream graph evidence requires them. State and test the specific clone boundary rather than silently changing all callers.
- Extend full constructor/build graph comparisons for definition removal, original body rewrites and cached raw nodes; use existing custom-input fixtures and compare Source/tuple aliases and extra fields, not only CSS.

**Avoid**

- A second native optimized candidate cache, reconstruction solely from `Prepared.ast`, Source content deduplication, or selector/output-position identity substitutes (design lines 35,55–57; M4 line 257).
- Inferring original human motivation from code mechanics or presenting local untracked records as a reviewed historical decision.
- Treating no-op/output restoration as rollback of already executed JS writes, weakening quota checks, or claiming complete provenance/full M4 acceptance from narrow graph checks.

**Risks to verify**

- Caller Source/tuple alias identity can be lost even if all values, CSS and UTF-16 units remain correct; conversely global no-freshening can break the existing independent clone identity contract.
- Pointer-only origin keys without retained owners can alias after address reuse; new retained mappings must also be charged and published at the existing transaction boundary.
- Equal selectors/CSS shapes and declaration overlap can select an incorrect authored occurrence; require concrete graph counterexamples before redesigning broadly.
- Raw/input/output publication and accessor failure ordering are coupled; expanded replay events can alter successful-write prefixes or retain uncharged handles. Full proxy/accessor timing is still unverified.

## Confidence summary

Direct: current explicit local rationale for unique raw cache, identity owners, canonical public provenance, pre-render binding, error boundaries and incomplete scope. Supported: current clone/alias, raw/render sharing and graph-verification contracts from code plus tests. Inferred: a narrower provenance-versus-generated-identity clone boundary may be needed. Unknown: original authored motivation, commit/review lineage and external deliberation. No implementation or validation verdict is claimed.
