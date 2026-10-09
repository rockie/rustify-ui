# Anonymous architecture judgment

## Decision

Neither package is implementation-ready as written. Package two is the preferred experimental base, conditional on the blocking corrections below. It is the smaller coherent foundation because it reuses `PublicValue`, keeps the existing declaration identity, attaches only an opaque origin handle to a native node, and puts COW canonicalization, clone policy, projection, and accounting in one bounded module. Package one is conceptually capable, but its parallel `MirrorValue`/object/array runtime duplicates the complete public graph beside both the typed AST and materialized `PublicValue` output. That creates a substantially larger synchronization, migration, retained-memory, and public-model surface before it demonstrates any behavior package two cannot provide.

This preference does not approve package two unchanged. If preserving Rust source compatibility for the public `AstNode` enum is a hard release requirement, neither package currently qualifies: both add mandatory fields to all public enum variants. That decision must be made explicitly or the carrier must move behind a source-compatible internal representation.

## Ground-truth correction: observable setup order

Both packages give an inaccurate single-line phase order. In the pinned upstream source, the relevant observable order is:

1. The initial AST walk processes `@theme`, replacing the first theme occurrence and removing later occurrences (`ref/tailwindcss/packages/tailwindcss/src/index.ts:528-580`). This array mutation is before custom utility registration.
2. Custom variants are registered at `index.ts:608-610`; custom utilities are registered at `index.ts:612-614`. Functional registration performs its declaration-value preprocessing assignment at `utilities.ts:5796-5925`, including an assignment when the rendered text is unchanged.
3. Final theme materialization happens after registration: keyframe contexts are pushed and `firstThemeRule.nodes` is assigned at `index.ts:618-636`. This is distinct from the earlier theme-array replacement.
4. Authored `@variant` nodes are changed with `Object.assign` at `index.ts:639-659`.
5. Functions run at `index.ts:663`, then apply at `index.ts:664`.
6. The utilities node receives `kind` and `context` assignments at `index.ts:666-672`.
7. `@utility` definitions are removed at `index.ts:674-685`.

Package one (`custom normalization -> theme effects -> ...`, line 226) and package two (`registration/normalization, theme replacement, ...`, line 216) both collapse the two theme moments and place all theme effects after normalization. This is a correctness defect, not editorial wording: setters, array methods, their successful prefixes, and thrown exceptions are observable in chronological order.

Both packages also under-specify the variant step. Their proposed edit interfaces have declaration-value, at-rule-params, and child-splice operations, but no operation that reproduces the ordered `Object.assign(variantNode, styleRule)` writes to `kind`, `selector`, and `nodes` while retaining old unrelated fields. A chronological flat journal (with phase tags only as metadata) or distinct `InitialThemeWalk` and `FinalizeTheme` phases is required. A phase-bucket implementation may not reorder effects.

There is a second common blocker at the constructor error boundary. Core `compile_ast` currently returns `Result<AstCompiler, CompileError>` (`crates/rustify-style/src/compiler/public_api/compile_ast.rs:864-1028`), and the WASM constructor calls `input_graph.write` only after that result succeeds (`crates/rustify-style-wasm/src/public_api.rs:486-519`). If normalization, final theme work, variants, functions, apply, or removal fails after earlier upstream mutations, a journal owned only by the successful compiler is lost and cannot reproduce the caller's successful mutation prefix. Both packages claim or imply preservation of constructor failure prefixes, but neither sketches a return channel that can do it.

The selected design needs a private setup outcome such as `Complete { compiler, effects, fees }` versus `Failed { error, prefix_effects, fees }`, or an equivalent staged boundary. The facade must preflight the complete returned prefix before any caller write; on a semantic setup failure it then replays that admitted prefix and returns the semantic error if replay succeeds, while an earlier host setter/splice failure returns the host failure after its own successful prefix. A resource rejection still performs no writes. A callback that mutates JS phase-by-phase would violate the existing preflight rule and is not an acceptable shortcut.

## Package one: independent assessment

### What is viable

- Stable node/object/array handles can identify equal-looking definitions and repeated occurrences without CSS-shape recovery.
- A single `CloneSession` can preserve aliases across node fields, source tuples, arbitrary metadata, and body arrays while giving independent custom/apply clones fresh identities.
- Explicit deep-clone versus shallow-spread operations can express the upstream raw/optimized identity rules.
- Transaction-local mirror edits and prefix-version journals can preserve pre-admission rejection and post-admission crossing failures.

### Why it is not the preferred base

1. **It introduces a second full public graph runtime.** `MirrorValue`, `MirrorObject`, and `MirrorArray` duplicate every represented field and edge. Known CSS fields then exist in the typed AST and mirror, and output exists again as `PublicValue`. Calling one copy a boundary snapshot does not remove the synchronization invariant. Every scalar write, array edit, clone, optimizer wrapper, removal, and generated node must pass through `NativeEdits`, or the graph silently diverges.
2. **The current sketch already lacks required edit operations.** It does not model the split theme schedule or exact variant `Object.assign`. Its own viability criterion says missing one routed mutation invalidates the shape; that criterion is currently met on paper.
3. **Its public-model migration is larger.** It adds a mandatory universal `NodeId` to all six public variants and replaces the existing declaration identity. Native/parser-only users pay the identity cost, external enum literals and exhaustive field patterns break, and equality/debug semantics need redesign.
4. **It multiplies fee surfaces.** It correctly acknowledges independent mirror storage, projected `PublicValue`, clone memo, lineage, journals, and JS handles. That accounting can be made sound, but it is strictly more machinery and simultaneous live storage than reusing the current public graph representation.

Package one remains a fallback only if the attached-snapshot experiment proves that effective COW views cannot be canonicalized or scoped without global synchronization. It should not be selected merely because its explicit handles make the diagrams easier to state.

### Package-one rejection criteria

Reject the package if any transformation can directly mutate a represented field or child array without a `NativeEdits` operation; if setup errors discard the mirror transaction and its caller-effect prefix; if mirror GC requires reconstructing reachability from CSS/native shape; if one public graph is charged as though it were merely a reference while mirror and materialized storage are both live; or if the universal identity changes structural `AstNode` equality.

## Package two: independent assessment

### Why it is the preferred base

- `NodeOrigin` separates provenance from `SourceLocation`, so true src absence remains observable and Context/AtRoot are representable.
- Effective views are keyed by public semantic object identity while allocation accounting remains keyed by actual COW storage. That is the necessary distinction for an alias to a normalized child: an `extra.child` edge still pointing at old storage and the body edge pointing at a COW view must resolve to one current public object before fresh-clone memoization.
- One clone memo spanning node origins, src, dst, arbitrary fields, and arrays can preserve the complete clone-local graph. Optimized projection can then shallow-spread wrappers without deep-cloning metadata again.
- The existing declaration identity, raw candidate cache, public graph type, and WASM crossing adapter remain in place. The intervention is therefore narrower and more reversible than the mirror runtime.

### Blocking corrections before implementation

1. **Encode the upstream clone root in the API.** The sketch's `clone_fresh(&definition.nodes) -> Vec<_>` cannot express “static clones the body array, functional clones the whole definition object and then selects its body.” Store an explicit origin root on `Definition`, for example a private `CloneRoot::{StaticBody(ArrayId), FunctionalDefinition(NodeOrigin)}`, and make the checked clone operation consume that root. Do not infer it from CSS kind or a child slice.
2. **Make origin-aware cloning fallible and budgeted.** The proposed `clone_fresh` is infallible, while the same package requires preflight of the effective graph, copied storage, and clone memo. Add a private checked clone path returning `Result` and reserve peak storage before allocation. The existing public native-only helper may remain only if it cannot encounter an attached origin; it cannot become an unchecked quota bypass.
3. **Fix the setup journal order and missing operations.** Split initial theme walk from final theme assignment, add exact variant-object assignments, retain same-value normalization writes, and record only the generic function assignments whose upstream guards execute. Apply and removal must follow the upstream sequence above.
4. **Return admitted prefixes on setup error.** `OriginPhase`/`SetupJournal` must survive a semantic constructor error long enough to cross the facade boundary. A plain `Result<Compiler, CompileError>` plus replay-on-success cannot satisfy this. Define the failed-setup outcome and error precedence described above without publishing a partial compiler.
5. **Preserve native model semantics.** `AstNode` currently derives structural `PartialEq/Eq/Debug/Serialize`. Adding `origin: Option<NodeOrigin>` must not make otherwise equal ASTs unequal, leak provenance into serialization, or make ordinary `Clone` accidentally freshen origins. Use explicit structural equality that ignores provenance, initialize `origin: None` in deserialization/generated constructors, and separately test fresh clone versus ordinary clone.
6. **Resolve the Rust source-compatibility break.** `AstNode` is publicly reachable as `rustify_style::compiler::ast::AstNode`; adding a field breaks existing enum literals and field-exact patterns even if the type is doc-hidden. Either accept and document a versioned breaking change, or prototype an internal wrapper/carrier. Package two is less disruptive than package one because callers can at least spell `origin: None` and declaration identity is unchanged, but it is still a break.
7. **Bound snapshot retention.** A candidate clone must retain only its effective clone scope, not the entire original input graph. Pruning must preserve edges reachable from the chosen root, detached objects referenced by metadata, body arrays, and live journal values. A missing retained root fails correctness; retaining all prior phase graphs indefinitely fails memory behavior.

### Package-two rejection criteria

Reject the package if a fresh clone needs CSS/path matching; if a node can observe two effective views for one semantic object after normalization; if snapshot rebinding misses extracted definitions, theme/keyframe roots, duplicate references, or removed nodes; if setup failure cannot return its exact admitted journal prefix; if an attached graph causes ordinary render clone to deep-clone metadata; if an origin-aware clone allocates before its peak check; or if no bounded rule can release superseded snapshots on warm no-op builds.

## Concrete first feasibility experiment

Prototype only package two's origin slot, explicit clone root, effective-view lookup, and checked clone transaction. Do not migrate all setup phases yet, and do not weaken or replace the pending custom-input RED.

Use one differential fixture with two definitions and the pinned upstream implementation as oracle:

- A functional definition whose wrapper has wrapper-only metadata, whose body array is also referenced by metadata, and whose declaration is referenced both from `nodes` and an extra field. Its declaration is normalized during registration and later function-substituted. Its src and dst share a tuple/Source also reached through extra metadata; include a second equal-but-distinct Source.
- A static definition with analogous wrapper-only and body metadata.
- Nested rule, at-rule, comment, Context, and AtRoot objects, including true missing src/dst. Include an apply expansion so a second independent fresh clone and descendant src overwrite are exercised.
- Tight limits around wrapper-only metadata, effective COW storage, clone memo, and projected output.

The experiment must distinguish the root scopes: functional cloning must account for the whole definition object before returning the cloned body; static cloning must clone only the body array. Wrapper fields must never leak onto the generated class rule. Within each returned clone, the alias to the normalized/substituted declaration and the body edge must converge to the same fresh object; independent candidates and apply expansions must not share it.

Promote package two only if this slice passes all of these gates:

1. **Graph fidelity:** all enumerable own string fields and key order match upstream; missing differs from explicit undefined; equal-distinct objects stay distinct; src/dst/extra/body aliases and fractional/lone-UTF-16 values match; candidate/apply clones are fresh and independent.
2. **COW discriminator:** metadata pointing to the old storage of a declaration observes the normalized/current child after effective-view resolution. A plain attached `Arc<PublicValue>` with no effective view must fail this test.
3. **Source-absence discriminator:** Context/AtRoot and nodes with no src remain truly absent until an upstream operation explicitly assigns src. A SourceLocation-only carrier must fail this test.
4. **Raw/optimized identity:** surviving declarations and comments are the same public objects in raw and optimized projections; surviving rule/at-rule wrappers are fresh shallow spreads whose metadata references still alias raw metadata; eliminated wrapper metadata does not migrate.
5. **Exact journal trace:** initial theme array replacement precedes normalization; normalization precedes final theme nodes assignment; variant writes, guarded function writes, apply replacements, utilities writes, and removal follow in upstream order. Same-value setters and intermediate values are present. A deliberately late semantic constructor error returns the already-planned prefix through the failed-setup boundary. Frozen/nonwritable/setter/splice sentinels have the same successful prefix and thrown result, including precedence between the semantic error and a host replay error.
6. **Retained and peak fees:** charge input/origin roots, extracted and detached definition data, every live COW version, journal targets and intermediate values, candidate snapshots, raw projection, optimized output, source graphs, map capacities/owners, and WASM handles. Separately reserve the simultaneously live phase overlay, prior/new snapshot, clone memo, cloned graph, staged projection, and crossing materialization. Semantic identity may deduplicate identity ownership; it must not deduplicate distinct live COW field storage.
7. **Transaction behavior:** quota rejection occurs before native publication and before caller writes. A host crossing failure after admission retains admitted candidates/counters and the executed caller-effect prefix while restoring only the last published output. Repeated no-op builds do not grow retained fees or change handles.
8. **Compatibility:** serialization is unchanged, structural AST equality is unchanged, ordinary `Clone` preserves identity, fresh clone freshens it, and the chosen policy for public enum source compatibility is explicit and compile-checked.
9. **Regression floor:** run the existing custom-input, public-graph, input-mutation, raw/apply, source, budget, and pending-family suites unchanged. The saved pending RED must turn green by preserving the full graph; no expected-field filtering, ignored assertion, fixture narrowing, or status relabeling is acceptable.

After this feasibility slice, full acceptance still requires all six node kinds, both clone scopes, repeated definitions and occurrences, complete setup journaling, raw/cache/render ownership, quota boundaries, and crossing failures. Passing the slice is evidence for the carrier, not completion.

## Presentation-order check and limits

I first assessed package one then package two, then reapplied the same hard gates in the reverse order (package two then package one): graph fidelity, exact effect order, clone scope, raw/optimized identity, retained/peak fees, and compatibility. The preference remained package two; the deciding factor was the smaller graph/runtime and migration surface, not presentation position.

This is one anonymous judge's design-only assessment. I read the two anonymous packages and grounding, and bounded source around the cited upstream order, clone/projection behavior, current AST model, and current publication/accounting paths. I did not read candidate identity files, implement code, run tests/builds, or measure memory. Feasibility and fee constants therefore remain unproven until the experiment above.
