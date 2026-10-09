# Registration proposal

## Problem

This candidate tests the imposed **native AST authority** intervention; it is not a synthesis recommendation. One Rust registry owns directive grammar, traversal state, structural changes, and ordered effects. Public setup projects those effects without recognizing directives again. Imports retain their existing Program/Machine schedule. The milestone remains **3/7** and full registration conformance remains pending ([manifest](manifest.json), [plan](../../../plan/WASM-TAILWIND.md)).

The current separate source/custom/theme passes and full-tree media preprocessing cannot preserve authored first errors or mutation prefixes (`compiler/mod.rs:231–258`, `imports.rs:376–379`). Fixed reference `index.ts:169–580` and `ast.ts:136–220` determine the replacement, validation, and descent semantics below.

## Usage (caller's view)

Native construction keeps one internal construction result; ordinary native callers discard semantic prefixes only at their outer convenience boundary.

```rust
// compiler/mod.rs, replacement for Compiler::new_with_ast's registration block.
let expanded = match imports::expand_only_with_base(
    &mut input_ast, base, assets, &limits, options.from.is_some(),
    public_setup.as_mut().map(|s| &mut **s as &mut dyn imports::PublicSetup),
) { // same Bundle/Program/Machine; no global media transform
    Ok(expanded) => expanded,
    Err(error @ CompileError::Limit { .. }) => return Err(error),
    Err(error) => return Ok(NativeSetup::Failed(NativeSetupFailure::imports(error))),
};
let initial = registry::InitialTree::capture(input_ast, input_index)?;
let ready = match registry::register(initial, base, &limits)? {
    registry::InitialOutcome::Complete(ready) => ready,
    registry::InitialOutcome::Failed(failed) => {
        return Ok(NativeSetup::Failed(NativeSetupFailure::initial(failed)));
    }
};
// Compatibility acknowledgement remains here, before register_deferred.
// finish_registered captures later semantic failures with ready.prefix;
// only admission/resource errors escape through outer Err.
Compiler::finish_registered(ready, expanded, limits, public_setup)
```

Public AST setup uses the same constructor. Its adapter translates domain targets and values into existing InputMutation operations, including failures.

```rust
// public_api/compile_ast.rs, inside compile_ast_setup.
let converted = origins.convert_registration_input(&input, limits.ast_depth)?;
let setup = Compiler::new_with_ast(
    converted.nodes, converted.index, &assets, limits.clone(), options,
    None, Some(&mut import_graph),
)?;
// Root outcome owns initial prefix even when no Compiler exists.
// Outer Err is a boundary/resource/admission refusal, authorizing no writes.
project_native_setup(setup, import_graph, origins, input, &limits)
// -> Result<AstSetupOutcome, CompileError>; existing prepare/start then replay.
```

## Shape

`InitialTree` is a short-lived typed native graph, not a grammar over PublicValue. Canonical node cells own typed fields; canonical list cells own **current ordered NodeIds**. Public conversion interns by object identity and array identity; native-only objects receive fresh IDs. Repeated object occurrences read one cell. Repeated child arrays read one list. Every cursor iteration, subtree walk, and deferred body access resolves current cells, never an earlier Vec copy. Removing a source from a shared list therefore affects its next occurrence and a previously captured utility body.

```rust
// Private registration-only shape; metadata is opaque provenance, not grammar.
struct NodeId(usize);
struct ListId(usize);
struct InitialTree {
    root: ListId,
    nodes: Vec<NodeCell>,
    lists: Vec<ListCell>,
    provenance: Provenance,
}
impl InitialTree {
    fn capture(nodes: Vec<AstNode>, index: NativeInputIndex)
        -> Result<Self, ResourceRefusal> { todo!("not implemented") }
}
struct NodeCell { data: InitialNode, metadata: NodeMetadata }
struct ListCell { current: Vec<NodeId>, origin: Option<ArrayOrigin> }
enum InitialNode {
    AtRule { name: Text, params: Text, body: ListId },
    Rule { selector: Text, body: ListId },
    Context { patch: ContextPatch, body: ListId },
    AtRoot { body: ListId },
    Declaration { property: Text, value: InitialValue, important: bool },
    Comment(Text),
}
enum InitialValue { Text(Text), Undefined }
struct ContextPatch {
    base: Option<Text>, source_base: Option<Text>, reference: Option<bool>,
}
struct Context { base: Text, source_base: Option<Text>, reference: bool }
struct Cursor { list: ListId, index: usize, parent: Option<NodeId>, context: Context }
enum Advance { Descend, Skip, RevisitReplacement, SkipReplacement(usize) }

struct Registry {
    tree: InitialTree,
    theme: Theme,
    sources: sources::Selection,
    utilities: Option<NodeId>,
    important: Option<bool>,
    first_theme: Option<NodeId>,
    custom_variants: Vec<PendingVariant>,
    custom_utilities: Vec<PendingUtility>,
    authored_variants: Vec<NodeId>,
    effects: Vec<InitialEffect>,
}
struct PendingUtility { node: NodeId, name: UtilityName, body: ListId }
struct PendingVariant { name: VariantName, definition: VariantInput }
enum VariantInput { Selectors(ParsedSelectors), Body(ListId) }
enum InitialEffect {
    Rename { node: NodeId, name: Text },
    Params { node: NodeId, params: Text },
    Children { node: NodeId, body: ListId },
    Replace { list: ListId, index: usize, replacement: Vec<NodeId> },
}
struct InitialPrefix { effects: Vec<InitialEffect> }
struct RegisteredInitial { registry: Registry, prefix: InitialPrefix }
// Outcome construction moves Registry.effects into prefix; never copies/synchronizes it.
struct InitialFailure {
    error: CompileError,
    prefix: InitialPrefix,
    retained: InitialTree, // generated values and original targets remain resolvable
}
enum InitialOutcome { Complete(RegisteredInitial), Failed(InitialFailure) }
struct ResourceRefusal { resource: Text, limit: usize }
enum NativeSetup { Complete(Box<Compiler>), Failed(NativeSetupFailure) }
struct InitialRecord { prefix: InitialPrefix, retained: InitialTree }
struct NativeSetupFailure {
    error: CompileError,
    initial: Option<InitialRecord>, // absent only when imports failed before registration
    later: Vec<InputMutation>,
    compatibility_reached: bool,
    // ImportGraph retains the unchanged import program/prefix independently.
}

fn register(tree: InitialTree, base: &str, limits: &Limits)
    -> Result<InitialOutcome, ResourceRefusal> { todo!("not implemented") }
impl Registry {
    fn walk(&mut self, cursor: Cursor, limits: &Limits)
        -> Result<(), RegistrationFault> { todo!("not implemented") }
    fn visit(&mut self, cursor: &Cursor, limits: &Limits)
        -> Result<Advance, RegistrationFault> { todo!("not implemented") }
    fn register_deferred(&mut self, limits: &Limits)
        -> Result<(), RegistrationFault> { todo!("not implemented") }
}
enum RegistrationFault { Semantic(CompileError), Resource(ResourceRefusal) }
fn project_native_setup(setup: NativeSetup, imports: ImportGraph<'_>,
    origins: Origins, input: PublicAst, limits: &Limits)
    -> Result<AstSetupOutcome, CompileError> { todo!("not implemented") }
impl NativeSetupFailure {
    fn imports(error: CompileError) -> Self { todo!("not implemented") }
    fn initial(failed: InitialFailure) -> Self { todo!("not implemented") }
}
// ResourceRefusal converts only to CompileError::Limit.
```

Conversion must stop discarding undefined declarations before registration (`compile_ast.rs:1295–1299`): retain typed slots and exact body lengths in `InitialTree`, then omit them when lowering to the optimized native AST. Decode context property **presence**, including empty base and explicit `reference:false`; merge patches by replacement, not native's current OR. Existing Vec imports need a typed input-index sidecar for slots/provenance, updated at occurrence completion. This is a required boundary implementation, not recovered magically from IDs.

`visit` dispatches utilities, utility, source, variant, custom variant, media, theme in fixed reference order. Semantic helpers operate on native domain data. Each explicit assignment updates the canonical cell and appends an effect, even when equal. Replacement updates its current list immediately: zero/multiple replacements project as Splice, one as numeric Set. Continue revisits index; Skip advances by replacement length. Context changes context without changing parent; ordinary containers, including AtRoot, establish parent.

Custom variant checks nesting, removes its occurrence, then validates; legacy root conversion first emits Rename and falls through. Utility validates and queues a ListId without detaching; initial descent continues through its body. Source validates body before nesting and removes only after successful collection. Selected utilities keep authored params; duplicate/reference removal happens before option parsing. Source-root discovery stays distinct from source-map metadata.

Media performs its targeted child walks when reached, in parameter order: source wraps each reached exact utilities match with sourceBase; replacement plus Stop skips inserted nodes and continues later siblings; theme/prefix append params; reference assigns a wrapper; importance changes registry state; unknown conditions assign params or consumed wrappers flatten and revisit. These child walks use the same cursor machinery with their own fixed callbacks, including Stop/Skip behavior. They do not impose native definition-body skips absent from that callback.

Theme chooses the last prefix, gathers Context-transparent tokens and identity-deduplicated keyframes, and skips keyframe bodies. Invalid child errors stop **before** theme replacement. First success replaces with an empty rule carrying the original src; later themes splice away. Token unescaping is registry data, not an authored property assignment. Dependency/function preparation waits until after initial success. Custom variant registries then materialize in queue order after compatibility, followed by utility registration/functional normalization. Publication hoists original keyframes through generated theme/AtRoot wrappers onto the synthetic outer root, then assigns final theme.nodes; it does not append them to the caller's original array or clear their bodies (`index.ts:627–639`). Capture therefore retains distinct synthetic-root and caller-list IDs. Utilities retain live bodies until final removal; trusted private position markers are introduced only when lowering for existing function passes.

No semantic-error catch resumes the walk. Root failure retains exactly the effects already emitted; it never creates final theme.nodes or deferred normalization. Success records the compatibility offset after all initial effects; final theme publication remains after deferred registration. Later failures extend that existing prefix, rather than rescan theme input. Earlier replayed host faults still supersede the stored semantic error through the existing crossing.

Provenance shares Source/src/dst tuples, unknown object fields, caller targets, and stable identity. COW clones only changed field/list storage; original handles stay targets. Effect projection does no directive traversal. Lowering may materialize native Vec bodies once, preserving NodeOrigin/current views for later passes; originals and aliases remain in the public graph. Unchanged metadata is never reconstructed from typed Source's file/code subset.

## Module map and replacement

- `compiler/registry.rs`: typed graph, one visitor, state, initial domain effects, deferred queues; owns initial policy.
- Existing source/theme/custom modules: retain domain algorithms/registries; replace extractors with non-traversing validation/registration operations. Delete initial whole-tree extraction and `authored_headers` calls.
- `imports.rs`: remove media work from expansion; retain bundle/schedule. Delete global transform_media and PublicSetup::media once callers migrate.
- `origin.rs` and `compile_ast.rs`: identity/provenance binding and effect projection. Replace theme-input reconstruction and caller_utilities rescans with captured targets; final theme publication uses the captured generated rule. Existing later apply/position machinery adapts to current lists, then retires redundant position reconstruction where effects already supply it.

These are knowledge boundaries, not load/validate/save stages. Two operations—register and consume its outcome—hide ordering, alias resolution, and failure policy (boundary-discipline, single-source-of-truth). No per-directive public coordination API is introduced.

## Tradeoffs accepted

- We accept a temporary canonical graph and conversion sidecar for shared-array semantics that mutable detached Vecs cannot represent.
- We accept targeted subtree walks and one lowering for exact fixed-reference behavior and existing compiler interoperability.
- We accept retained failed-session graphs for original targets and generated handles captured by throwing setters.

Reserve graph capacities, canonical indexes, pending bodies, journals, provenance, lowered overlap, and all import-program potential effects before start. Deduplicate owners by identity/storage as appropriate; include failure continuations and host handle/completion reserves. Limit refusal is outer Err and replays nothing. This conservative retained estimate is not proof of allocator peak.

## Alternatives considered

A direct mutable Vec visitor plus Edits is smaller internally but leaks alias reconciliation and deferred-body copying into callers; Edits currently refreshes only value/params (`origin.rs:160–180`). It loses shared-list current state. A PublicValue-first registry hides alias machinery naturally but violates this candidate's explicit native-authority intervention. Duplicated public grammar and extractor-per-node wrappers are rejected.

## Red-flag screen

No shallow directive facade, public wire types, parallel semantic projection, temporal stage modules, or pass-through handler chain. Registry complexity is retained because alias/current-container and stop-position semantics are data requirements; projection only translates effects.

## Open questions and risks

Can the conversion/import sidecar preserve absent versus undefined fields and shared arrays without a second semantic read? Can lowering preserve later utility/variant alias effects without reintroducing duplicate current graphs? Can peak accounting cover simultaneous arena/Vec/journal storage under existing limits? These need isolated fixtures, not asserted support.

## Synthesis decision

Independent candidate; parent fills this after comparison.

## Next implementation step

Build an isolated typed graph/cursor prototype covering shared child-array removal, legacy removal-before-error, media replacement revisit, and deferred utility body aliasing; migrate only after those agree with the pinned reference. Scrap this shape if projection needs directive grammar, repeated bridge exceptions appear, or later passes require independently synchronized semantic graphs. Then redesign the representation before adding adapters.
