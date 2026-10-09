# Import failure scheduling synthesis

## Problem

The pre-integration importer used synchronous DFS. The fixed upstream starts every valid import during the current synchronous walk, then awaits parallel tasks; its failure can leave already started tasks that complete after rejection. A failure journal containing every successful sibling is incorrect. The complete M4 contract and admission-before-write rule remain unchanged; [Ground](ground.md) records the archived code trace and [expanded actual-WASM RED](../utf16-families-import-failures-expanded-before-red.log) records the gap before integration.

## Usage (caller's view)

Existing `compile_ast(...) -> Result<AstCompiler, CompileError>` and `new AstCompiler(input, config)` remain. Native callers use the same importer with a recording host. The synchronous constructor models the caller observation boundary and must retain detached tasks automatically; it cannot provide real await timing. The additive Rust-controlled `compileAst(input, configJSON)` now owns the actual Promise boundary for finite virtual bundles, with JavaScript restricted to posting callbacks and publishing settlement. No half initialized compiler is returned on failure.

## Shape

Use the owned per-occurrence native fragments and bounded FIFO reaction machine from [candidate A](candidate-A.md). All propagation comes from settled/pending promise registration, loader/nested awaits, `Promise.all` member and scope reactions, and actual parseCss/compileAst awaits. No depth or epoch delay constants enter production. Native parsing happens once per occurrence; the actual parsed nodes, public parent handles, fresh Source, and stable wrapper context have one owner through the session. Ordinary callers do not coordinate importer phases.

Preparation records the finite virtual bundle's complete ownership through quiescence, including work after root rejection. Execution reuses prepared fragments and repeats the same state transitions with real host acknowledgements. A begin setter failure stops that walk after the task was launched; a completion setter failure rejects that task, while independent siblings continue. Before any host call, release mutable session borrows. Retain newly materialized handles before calling a setter that can capture them and throw. Reserve native/working graph overlap, journal, crossing graph, ready callbacks and detached owners before any write.

Every ready reaction is posted when it is created. Do not schedule one pump that runs A and only then posts already-ready B: a caller marker can then slip between A and B. The async exported Promise settles at the actual compileAst operation; only the synchronous adapter attaches a synthetic caller observer.

## Synthesis decision

Base: candidate A, frozen SHA256 `028818d4c8c5012ccaaadec6ae80c4915a4d89f18597d6e0f3d211e9e665a893`. Candidate B SHA256 `e2eb883f4889c0e8059bc1240ebfe5fdb766936c1e437e8bf1a1c58f25695506`. The anonymous [judge](judge.md) read proposal-2 before proposal-1 and selected proposal-2, which [frozen mapping](frozen.json) identifies as A. Two candidates were independent, with intentionally different whole-shape constraints; unavailable default architect models were replaced by the inherited model and the supported GPT-5.6 runner. This comparison does not isolate model effects. The independent judge used supported GPT-6 Astra.

Zero structural grafts. B's event DAG, registration order, ticket frontier and settlement cells add overlapping scheduling representations without demonstrated benefit. Its written one-turn pump and synthetic-observer publication also fail the external-marker thought experiment. A already contains persistent handle retention and host-error feedback; those common requirements do not require importing the DAG. Both were screened for shallow interfaces, policy leakage, temporal decomposition and pass-through boundaries. The judge's reverse-order check was a mental review, not a second blind run. Selection is design evidence, not production conformance.

## Executable check

[reaction-probe.rs](reaction-probe.rs) is a disposable standalone Rust implementation of the selected reaction rules. It uses dependency fixtures, not the compiler parser or AST. [verify-reaction-probe.mjs](verify-reaction-probe.mjs) verifies pinned upstream source hashes and compares 25 cases against the actual installed 4.1.13 oracle: all load/index/nodes traces, marker epochs, selected normalized error, and pre-/post-observation split match. Success includes the real compatibility await. Two intentionally bad variants—drain all successful siblings before error and cancel a deep sibling after a host failure—are rejected. [Result](../utf16-families-import-reaction-probe-verify.log).

This validates the reaction primitive, not owned compiler fragments, unified peak accounting, JS callback interleaving, real original-error identity or production AST behavior. At this investigation's capture, the 15-case production matrix had 12 failing prefixes and 12 failing settled graphs; production sources and artifacts had not changed. That capture is preserved as historical evidence.

## Production integration

`crates/rustify-style/src/compiler/imports/bundle.rs` now owns actual per-occurrence native fragments and Sources. `crates/rustify-style/src/compiler/imports/schedule.rs` supplies the same FIFO transitions for preparation through quiescence and host-acknowledged replay. `crates/rustify-style/src/compiler/public_api/compile_ast/setup.rs:78` prepares all potential mutations and retained owners; `crates/rustify-style-wasm/src/public_api/setup.rs:227` checks native, input-handle and crossing reserves before start. `crates/rustify-style-wasm/src/public_api/graph.rs:172` retains newly materialized handles when a setter captures a context and throws. The synchronous adapter posts every remaining job individually after its modeled observation; its last callback releases the session.

[Actual WASM](../utf16-families-async-setup-final-import-failures.log) passes fifteen semantic rejection/settled graphs, five original host exceptions, one first-settlement case, four no-write quotas and one untracked Source admission control. Three native tests cover selection, failed-parser Source fees and retained context/Source lifetime. The first integrated module exposed a [Source-reserve RED](../utf16-families-import-session-source-owner-before-red.log), now fixed. The previous success/function/quota gates remain green; exact artifacts, measurements and full checks are recorded in [M4](../M4.md).

The finite-bundle Promise entry is implemented at `crates/rustify-style-wasm/src/public_api/setup.rs:282`. Ready loader callbacks post before begin setters and their external markers; setup journal boundaries split initial theme/media from normalization/final-theme/functions around the compatibility await. Direct Promise publication omits the synthetic observer. [Actual WASM](../utf16-families-async-setup-final-async.log) matches twenty marker/epoch/full-graph cases, six orphan cases and four no-write quotas, including original orphan host-error identity and a joined-task control. No-import calls retain their awaits. External loaders and arbitrary successful setter/Proxy changes to subsequent walks remain pending. Conservative admission is not a unified working-AST/parser/config/callback/JS allocator peak proof.

## Tradeoffs accepted

- Keep finite preparation plus host-controlled execution to admit all possible retained work before mutation; their overlapping ownership must be charged.
- Retain failed sessions until their launched tasks settle, so deep contexts remain live after root rejection.
- Keep the existing synchronous adapter and expose actual asynchronous timing through a separate Promise boundary.

## Alternatives considered

Moving begin earlier in DFS cannot recover unvisited siblings, first settlement or detached work. Depth-based scheduling and all-success-before-error replay are contradicted by oracle traces. The event DAG adds representation and its current callback driver changes external interleaving; a single owned reaction queue hides that policy within the importer.

## Open questions and risks

The oracle emits an unhandled rejection when a missing task was started before the synchronous walk throws on later invalid parameters. The first expanded oracle experiment exited with Node's default fatal policy; the retained harness captures and records that event explicitly. The prototype does not report it; production now tracks unhandled rejected cells and reports each via native Promise.reject, with the covered six-case scope recorded above. The [successful oracle capture](../utf16-families-import-reaction-probe-oracle.log) confirms one `missing missing.css` event. The covered host exceptions and retained-context teardown now pass; arbitrary successful accessor/Proxy mutations, external loaders and wider alias walks still require tests. Unified allocator peaks, complete public API/helpers, ledger execution, prior fresh-import cost, Promise-entry performance and browser validation remain unfinished.

## Next implementation step

The apply failure journal now retains completed setup/functions and native replacements, with 16 cases through two entries, 8 original host exception checks and quota boundaries in [M4](../M4.md). Ordinary/custom function order and first-error prefixes now pass 19 cases/38 entry comparisons with 8 original host exceptions. Extend early-stage semantic failure prefixes, wider normalization/variant/theme phase interactions and successful shared-array/Proxy changes during the walk, then complete helper/API and external-loader coverage with the required admission contract. Preserve the passing production rejection/settled graphs, success/function/quota gates and original exception identities while extending the scope. Full M4 conformance remains pending.
