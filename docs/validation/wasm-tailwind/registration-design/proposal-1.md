# Registration proposal

Phase checklist: Ground complete; Sketch complete; Agree/Implement/Scrap belong to synthesis. Independent structural candidate; no production change or executable prototype.

## Problem

Make the Rust-owned current graph authoritative for the initial directive walk. Derive native compilation inputs afterward, instead of extracting registrations from a native tree and reconstructing caller effects later. Current setup globally transforms media, then separately extracts sources, variants, utilities and themes (`compiler/mod.rs:220–258`; `compiler/imports.rs:376–379`). The pinned reference makes those decisions in one mutation-aware walk (`ref/tailwindcss/packages/tailwindcss/src/index.ts:164–582`).

The [manifest](manifest.json) records 36 RED cases, 72 actual-WASM comparisons, 68 failures: 8 wrong-error and 64 prefix mismatches, zero epoch mismatches. Artifact: 2,394,595 bytes, SHA-256 `e65e098b1e96b711a65c5e05f2712efdb94cf93001487bed194473ea4951e513`. This design addresses that registration seam; goal remains 3/7 and full M4 remains pending.

## Usage (caller's view)

Native compilation uses one internal session operation; it receives registered native inputs or an error, without coordinating collectors:

```rust
// Compiler::new_with_ast, after raw import expansion, before candidate setup:
let registered = RegistrationGraph::from_native(expanded, bindings, limits)?
    .register(options.base.as_deref().unwrap_or(""))?;
let prepared = registered.after_compatibility()?;
Self::from_registered(prepared, options, limits)

// compile_ast_setup, using the public topology and raw imported current view:
let draft = RegistrationGraph::from_public(input, imported, origins, limits)?
    .prepare(options)?;
draft.into_ast_setup_outcome() // success or semantic failure with exact prefix

// Existing WASM preparation, before start/start_async:
let setup = prepare_ast_setup(input, options, assets, limits)?;
// Existing native + crossing admission includes this draft's graph/effects.
// Existing ParseCss and Compatibility jobs replay their respective ranges.
```

`register` is the native convenience projection of `prepare`; the public caller gets the retained outcome. Both run the same Rust handlers. Production runs no reference JS/TS compiler. Lowering helpers are private, so callers never reconcile graph arrays, traversal indices or journals.

## Shape

```rust
// Private compiler/registration.rs. Bodies intentionally not implemented.
struct NodeId(usize);
struct ArrayId(usize);
struct RegistrationGraph {
    nodes: Vec<NodeCell>, arrays: Vec<ArrayCell>, root: ArrayId,
    object_ids: IdentityIndex<NodeId>, array_ids: IdentityIndex<ArrayId>,
    registrations: Registrations, effects: Vec<InputMutation>, budget: Reserve,
}
struct NodeCell {
    original: OriginBinding, current: NodeFields, children: Option<ArrayId>,
    // OriginBinding retains caller identity, extras and canonical src/dst tuples.
}
struct ArrayCell {
    original: ArrayBinding, current: Vec<NodeId>,
    // One cell per original array identity; generated arrays get fresh bindings.
}
struct Registrations {
    theme: Theme, theme_rule: Option<NodeId>, utilities: Option<NodeId>,
    sources: SourceSelection, variants: Vec<VariantDraft>,
    utilities_pending: Vec<UtilityDraft>, applications: Vec<NodeId>, important: bool,
}
struct UtilityDraft { root: NodeId, body: ArrayId, name: ValidUtilityName }
struct Frame { array: ArrayId, index: usize, parent: Option<NodeId>, context: Context }
enum Step { Descend, Skip, Stop, Revisit, Advance(usize) }
struct RegistrationDraft {
    outcome: Result<RegisteredGraph, CompileError>, effects: Vec<InputMutation>,
    compatibility: Option<usize>, reserve: Reserve,
}
impl RegistrationGraph {
    fn from_native(ast: Vec<AstNode>, bindings: ImportBindings, limits: Limits)
        -> Result<Self, CompileError> { unimplemented!() }
    fn from_public(input: PublicAst, imported: ImportBindings, origins: Origins,
        limits: Limits) -> Result<Self, CompileError> { unimplemented!() }
    fn prepare(self, options: CompileOptions) -> Result<RegistrationDraft, CompileError>
        { unimplemented!() }
    fn register(self, base: &str) -> Result<RegisteredGraph, CompileError>
        { unimplemented!() }
    fn visit(&mut self, frame: &Frame) -> Result<Step, CompileError>
        { unimplemented!() }
}
impl RegisteredGraph {
    fn after_compatibility(self) -> Result<RegisteredNative, CompileError>
        { unimplemented!() }
}
```

There is one topology and one current field store, per single-source-of-truth. The public adapter interns original arrays/objects by identity, retaining omitted/undefined fields and source tuples; the native adapter creates owned cells from parsed nodes. No registration decision runs in either adapter. Already supported structural validation stays at the input boundary; directive semantics run when reached.

The cursor reads `arrays[frame.array].current[frame.index]` anew each step. It never walks a cloned child list or globally marks arrays visited. Removal mutates the cell and leaves the cursor at the shifted successor. Continue after replacement revisits the inserted nodes at the same index; Skip advances by inserted count. Ordinary nodes become the logical parent; Context only merges context by override, including `reference:false`, and preserves parent. Shared arrays remain shared: removal through the first Context changes the second Context's subsequent walk. Cycle/depth admission remains separate from occurrence traversal.

Each field assignment updates current fields and records an effect against the original binding. Each replacement updates the current array and records the exact index Set or Splice. Immutable effect values retain storage snapshots while cells remain mutable; identity is independent of snapshot storage, preserving existing Graph/Edits COW semantics. This concentrates cursor, alias and prefix policy behind `prepare`, per interface-depth; domain helpers receive validated directive data rather than host replay bookkeeping.

### Shared grammar and precise effects

Dispatch uses the reference branch order. Utilities selection removes duplicates/reference nodes before option parsing; the first eligible node sets source root using `sourceBase ?? base`. Source validates body, nesting, quoting and expansion before removal. Utility validates nesting/body/name but leaves its body attached and descends. Root legacy variants assign the original `name` before falling through. Custom variants remove their position before name/body/selector validation, retaining the detached body; ordinary variant applications retain ordered NodeIds. Extract existing Rust grammar into non-traversing helpers rather than copying it into a public visitor (`sources.rs`, `custom_variants.rs:234`, `custom_utilities.rs:202–255`, `theme.rs:119–235`).

Media options perform their specified targeted subwalks through the same cursor engine, in option order: source wrapping, theme/prefix parameter assignments, importance, reference children replacement, then unknown-parameter assignment or flatten/revisit. They run only when reached, rather than globally before registration. Theme body collection uses transparent Context traversal, keeps original keyframe handles while skipping their bodies, and validates before replacing the first theme with a fresh **empty** rule or splicing later themes. Canonical Source tuples, extra fields and owners remain retained; discovery metadata is separate from source-map Source.

A semantic error immediately stops this walk and captures precisely its already completed effects. No later sibling or final theme assignment is manufactured. Compatibility offset exists only if the initial walk completed. After compatibility, activate variant drafts, then normalize functional utility bodies and build completion metadata from their **current** ArrayIds, then publish final theme nodes. Thus nested theme/media transformations encountered through utility bodies precede normalization; detached/stale native body copies cannot conceal them (`index.ts:608–613`; `custom_utilities.rs:256–260` currently normalizes eagerly).

Lower native input only after this sequence. Build existing Theme/CustomVariants/CustomUtilities and authored application data from registrations; generate trusted native utility-position markers at lowering, not caller-visible graph nodes. Preserve original declaration/keyframe provenance through NodeOrigin and SourceLocation.public. Subsequent function/apply edits receive the graph's frozen current view, and their journals follow the compatibility offset. Retain later-stage failure handling without rescanning theme positions to invent earlier effects.

### Imports and accounting

Keep existing Program/Machine/Host reaction scheduling and failed-root continuation. Split `expand_with_base` into raw import expansion plus registration; remove its eager `public.media()`/`transform_media`. `ImportBindings` is the adapter over original topology plus loaded arrays/current import replacements, not a second registration authority. Import begin/complete effects keep their handles and reserve; registration begins only on successful import-root settlement. Failed imports retain scheduler behavior and never run registration. The prepared draft's early range runs at ParseCss; its deferred range runs at Compatibility (`public_api/compile_ast/setup.rs:166–184`).

Prepare privately before any host writes. Charge arena capacities, identity indexes, cursor/context storage, retained original/COW fields, Source owners, drafts, lowered native data while coexisting, effects and all pending import continuation handles. Deduplicate shared identities/storage with the existing seen-book discipline; count distinct COW snapshots separately. Check expansion/depth/cache/queue limits and existing crossing reserve before start. Any admission refusal discards the entire draft and authorizes no compiler AST writes; admitted semantic errors replay their prefix, with original earlier host exceptions winning. This is a logical reserve design, not a measured allocator-peak claim.

## Synthesis decision

Pending independent comparison. This candidate deliberately chooses owned graph authority and later native derivation.

## Tradeoffs accepted

- Accept a temporary graph plus lowered native tree in exchange for exact shared-array and current-body semantics; account their coexistence explicitly.
- Accept changing internal collector constructors in exchange for one registration grammar and no retrospective public prefix reconstruction.
- Accept targeted media/theme subwalks in exchange for the reference behavior; one initial dispatcher does not imply one visit per node.

## Alternatives considered

Native-tree authority with an observation sink hides grammar but exposes alias reconciliation and mutable body synchronization across two representations. An immutable registration event tape hides replay ordering but needs a current topology anyway to decide revisits/shared-array deletions. Both leave callers coordinating more invariants than this owned graph.

## Open questions and risks

Can lowering preserve all existing origin/projection and first-theme anchors without restoring a second theme traversal? Can existing import bindings adopt every shared public array without native-index inference? Do ledger measurements expose uncharged COW or graph/native coexistence? These require isolated experiments; this sketch makes no blanket live-Proxy, broader loader or full-M4 claim.

Scrap if lowering requires repeating registration validation, if captured utility bodies need periodic synchronization, or if two independent alias failures demand graph exceptions. Revise rather than add parallel views.

## Next implementation step

Implement an isolated graph cursor plus source/custom-variant/theme handlers against the RED and shared-array fixtures before replacing production collectors; then integrate remaining handlers and both public entries.

Removal list: eager media passes; setup calls to `sources::extract/select_utilities`, `CustomVariants::extract`, eager `CustomUtilities::extract_marked`, `Theme::extract`, and whole-tree `authored_headers`; public media/theme reconstruction and corresponding ArrayPositions compensation for effects now recorded directly. Keep grammar helpers, imports scheduler, later apply positioning where still needed, canonical sources and host Graph. Remove only replaced responsibilities after equivalent gates pass.
