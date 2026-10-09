> Historical design-only candidate. Setup order corrected after judging; all unresolved interface, error-prefix, equality, compatibility and fee blockers are governed by synthesis.md. This is not an implementation contract.

# this package: attached node origin with immutable effective graph snapshots



## Problem

Custom utility bodies are native-fresh-cloned, while complete caller fields remain in public Origins indexed by old declaration IDs. The clone loses arbitrary fields at projection. Source-only repair cannot cover Context/AtRoot and creates src where the caller had no src. Correctness is graph-shaped: custom clone must isolate the whole body graph, preserve aliases among metadata/src/dst/nodes, reflect normalization/function writes through aliases, keep raw versus optimized wrapper identities distinct, and share surviving declaration/comment objects. Immutable raw native candidates remain the single candidate fact store; metadata snapshots carry public object information, not another optimized native AST. Existing quota admission and host failure prefixes remain requirements.

## Usage (caller's view)

Public usage stays the existing API:

```javascript
const child = {kind: 'declaration', property: 'color', value: '--value(--color)',
  important: false, extra: {owner: 'customer'}}
const source = {file: 'entry.css', code: '…', extra: 'source data'}
child.src = [source, 1.5, 9]
child.dst = child.src
child.extra.source = source
const definition = {kind: 'at-rule', name: '@utility', params: 'tone-*',
  nodes: [child], inspectChild: child}
const api = new facade.AstCompiler([definition, utilities], config)
const result = api.build(['tone-card'])
// Old child value is normalized during construction. Generated child retains
// all enumerable string fields; cloned src/dst/extra.source share one Source.
// Child and Source identities differ from original caller objects.
```

Internal call sites (pseudocode, API choices derived from these consumers):

```rust
// Public ingestion: capture full ordered object, independently of src presence.
let mut core = origins.convert(&input, None, 0, limits.ast_depth)?;
let origin_graph = OriginGraph::capture(&input)?;
origin_graph.attach(&mut core)?;
let compiler = Compiler::new_with_ast(core, assets, limits, options, None)?;

// Functional registration: actual assignment site records exact setter event.
let mut phase = OriginPhase::begin(SetupPhase::RegisterUtilities, roots, limits)?;
normalize(&mut body, depth, warnings, &mut phase)?;
phase.finish_and_bind(all_retained_setup_roots, &mut setup_journal)?;

// Native custom invocation: one effective graph clone, then private edits.
let mut body = ast::clone_fresh(&definition.nodes); // full effective origin graph
let mut phase = OriginPhase::begin_generated(&body, limits)?;
substitute(&mut body, value, modifier, fraction, theme, limits, state, &mut phase)?;
phase.finish_and_bind([body.as_mut_slice()], None)?;

// Projection: raw retains node identity; render spreads wrappers only.
let raw = origins.output_raw(&candidate.node, &mut projection)?;
let optimized = origins.output_optimized(&prepared.ast, &mut projection)?;
```

The real JS caller learns no origin IDs, clone modes, journal phases, or fee knobs. Core consumers use phase-local edits only where they actually assign node fields or splice arrays. Plain native Clone and unrelated parser construction stay ordinary operations with `origin: None`.

## Shape

### Data structures first

Helpers, views and journals below are crate-private; bodies intentionally omitted. Because AstNode is public, NodeOrigin must be an opaque public type with private fields (or an existing public metadata envelope must wrap it), rather than an inaccessible field type that triggers public-interface lint. No public constructors or mutation methods are added for it. Names are provisional, responsibilities are not.

```rust
// compiler/ast.rs: add exactly one independent optional slot to all six variants.
pub enum AstNode {
    AtRoot { #[serde(skip)] origin: Option<NodeOrigin>, nodes: Vec<AstNode> },
    Context { #[serde(skip)] origin: Option<NodeOrigin>, base: Text,
        source_base: Option<Text>, reference: bool, nodes: Vec<AstNode> },
    Rule { #[serde(skip)] origin: Option<NodeOrigin>, source: Option<SourceLocation>,
        destination: Option<SourceLocation>, selector: Text, nodes: Vec<AstNode> },
    AtRule { #[serde(skip)] origin: Option<NodeOrigin>, source: Option<SourceLocation>,
        destination: Option<SourceLocation>, name: Text, params: Text,
        nodes: Option<Vec<AstNode>> },
    Declaration { #[serde(skip)] origin: Option<NodeOrigin>,
        source: Option<SourceLocation>, destination: Option<SourceLocation>,
        property: Text, value: Text, important: bool, identity: Arc<()> },
    Comment { #[serde(skip)] origin: Option<NodeOrigin>,
        source: Option<SourceLocation>, destination: Option<SourceLocation>, value: Text },
}

// compiler/origin.rs: opaque identity wrappers retain their Arc owners.
#[derive(Clone)]
struct ObjectId(Arc<()>);              // PublicObject semantic identity
#[derive(Clone)]
struct ArrayId(Arc<Vec<PublicValue>>);  // array identity is storage identity

#[derive(Clone)]
#[doc(hidden)]
pub struct NodeOrigin {
    graph: Arc<OriginGraph>,
    object: ObjectId,                  // exact occurrence, never CSS shape
}

// One immutable public metadata snapshot per clone scope/phase.
// Fields include kind/nodes/value and unknown fields in original key order.
// Maps canonicalize changed COW views and array splices; they are not a second
// native AST. Root retainers keep arbitrary references to removed nodes alive.
struct OriginGraph {
    roots: Vec<PublicValue>,
    object_views: BTreeMap<ObjectId, PublicValue>,
    array_views: BTreeMap<ArrayId, PublicValue>,
    caller_objects: BTreeMap<ObjectId, PublicValue>,
    caller_arrays: BTreeMap<ArrayId, PublicValue>,
}

// Phase-local writable overlay. Never shared by Compiler/Prepared snapshots.
struct OriginPhase {
    base: Vec<Arc<OriginGraph>>,
    object_edits: BTreeMap<ObjectId, PublicValue>,
    array_edits: BTreeMap<ArrayId, PublicValue>,
    writes: Vec<SetupWrite>,
    limits: Limits,
}

enum SetupPhase { InitialThemeWalk, RegisterUtilities, FinalizeTheme, AuthoredVariants,
    Functions, Apply, UtilitiesContext, RemoveUtilities }

// Ordered events; intermediate values are retained, not recomputed from final AST.
struct SetupJournal { phases: Vec<(SetupPhase, Vec<SetupWrite>)> }
enum SetupWrite {
    Set { target: PublicValue, field: Text, value: PublicValue },
    Splice { target: PublicValue, start: usize, delete_count: usize,
        values: PublicValue },
}

enum ProjectionKind { Raw, Optimized }
struct OriginProjection<'a> {
    // Reuse the existing staged/generated declaration and raw candidate owners.
    // Comments are also object-reused; wrappers are shallow-spread on optimization.
    staged: &'a mut BTreeMap<usize, (Arc<()>, PublicValue)>,
    shared_values: BTreeMap<ObjectId, PublicValue>,
}
```

`OriginGraph` snapshots avoid interior mutability: setup phase edits cannot mutate a cache or a Prepared snapshot through an Arc. Snapshot rebinding is explicit at each semantic edit phase and covers all retained native roots, including extracted definitions/theme keyframes, not merely current input. Capture seeds node origin by exact object identity while Origins converts; the two-call usage can become one conversion return internally so capture is not a second traversal requirement. The field `origin` never encodes src presence. Parser/generated nodes have None until a full graph clone/projection materializes a public identity.

`ObjectId` equality/order compares retained Arc identity pointers; `ArrayId` compares retained original array storage pointers. Snapshot storage is counted separately from semantic identity. Private types prevent a Source content hash or selector string becoming a node identity (per domain-models-and-types).

### Signatures and module ownership

```rust
// compiler/origin.rs owns full field ordering, canonicalization and accounting.
impl OriginGraph {
    fn capture(input: &PublicAst) -> Result<Arc<Self>, CompileError> { /* not implemented */ }
    fn attach(&self, nodes: &mut [AstNode]) -> Result<(), CompileError> { /* not implemented */ }
    fn fields(&self, node: &NodeOrigin) -> &PublicValue { /* not implemented */ }
    fn owned_bytes(&self, seen: &mut OriginUsage) -> usize { /* not implemented */ }
}
impl OriginPhase {
    fn begin(stage: SetupPhase, roots: &[&[AstNode]], limits: &Limits)
        -> Result<Self, CompileError> { /* not implemented */ }
    fn begin_generated(nodes: &[AstNode], limits: &Limits)
        -> Result<Self, CompileError> { /* not implemented */ }
    fn assign_value(&mut self, node: &mut AstNode, value: Text)
        -> Result<(), CompileError> { /* not implemented */ }
    fn assign_params(&mut self, node: &mut AstNode, params: Text)
        -> Result<(), CompileError> { /* not implemented */ }
    fn splice_children(&mut self, parent: &mut AstNode, start: usize,
        delete_count: usize, replacement: Vec<AstNode>)
        -> Result<(), CompileError> { /* not implemented */ }
    fn finish_and_bind(self, roots: impl IntoIterator<Item = &mut [AstNode]>,
        journal: Option<&mut SetupJournal>) -> Result<(), CompileError> { /* not implemented */ }
}

// ast.rs owns native clone semantics, using one origin graph clone context.
pub fn clone_fresh(nodes: &[AstNode]) -> Vec<AstNode> { /* not implemented */ }
fn clone_fresh_with(nodes: &[AstNode], graph: &mut EffectiveClone)
    -> Vec<AstNode> { /* not implemented */ }
fn node_origin(&self) -> Option<&NodeOrigin> { /* not implemented */ }
fn set_node_origin(&mut self, origin: Option<NodeOrigin>) { /* not implemented */ }
struct EffectiveClone { /* declaration/source maps plus canonical public graph memo */ }

// Changed actual mutation functions: local phase parameter; no new public API.
fn normalize(nodes: &mut [AstNode], depth: usize, warnings: &mut Vec<Text>,
    phase: &mut OriginPhase) -> Result<(), CompileError> { /* not implemented */ }
fn functions::substitute(nodes: &mut [AstNode], theme: &Theme, depth: usize,
    max_bytes: usize, phase: &mut OriginPhase) -> Result<(), CompileError> { /* not implemented */ }
fn custom_utilities::substitute(nodes: &mut Vec<AstNode>, /* existing inputs */,
    phase: &mut OriginPhase) -> Result<(), CompileError> { /* not implemented */ }
fn apply::substitute(nodes: &mut Vec<AstNode>, theme: &Theme,
    custom: &mut CustomUtilities, limits: &Limits, phase: &mut OriginPhase)
    -> Result<Vec<Replacement>, CompileError> { /* not implemented */ }

// compile_ast.rs projects existing native trees and translates journal events.
fn Origins::output(nodes: &[AstNode], kind: ProjectionKind,
    inherited: Option<&PublicValue>, from: bool, staged: &mut OriginProjection)
    -> Result<Vec<PublicValue>, CompileError> { /* not implemented */ }
fn SetupJournal::input_mutations(self) -> Vec<InputMutation> { /* not implemented */ }
```

Sketch pseudocode is intentionally not valid Rust syntax for qualified free-function names/anonymous lifetime iterator items. Implementation must use the existing modules' actual functions and named lifetimes. `assign_value` owns two policies callers should not repeat: update the phase canonical effective view, and append an exact caller write iff the object belongs to caller_objects. Generated phases have no caller targets. Every origin map retains owner handles; no pointer-only indexes.

Module map is bounded: `ast.rs` holds node slot/access/fresh clone; one small `origin.rs` holds immutable metadata views and phase writes; existing transformations report their actual edits; `compile_ast.rs` uses those exact fields and existing mutation/fee publication; WASM graph remains the current adapter. No generic mutable graph runtime, locks, externally configurable phases, or second candidate AST cache is introduced. Full public metadata is already a public-api dependency of AstNode through SourceLocation; the new origin types remain private (per boundary-discipline).

### Effective full graph and COW alias algorithm

At phase entry, gather every reachable OriginGraph once. The phase editor holds private COW changed views. For an assignment, locate node's exact public semantic ObjectId, resolve its last effective fields, replace only the assigned key preserving IndexMap position, and retain that view in object_edits. Event target is the original caller handle; event value is the value at that assignment, even when equal or later changed. Typed Text changes at the same operation. A native child-array splice resolves the original ArrayId, edits that array's effective view, and retains removed objects' current views. Replacement array assignment uses a fresh array identity; splice preserves the target array identity. Native body changes and journal values therefore agree without reducing the journal to final diffs.

On finish, freeze edits into a new snapshot, preflight graph storage, and rebind all affected retained native nodes to it. Subsequent phases read the immutable canonical views. NodeOrigin graph snapshots include metadata-only references to removed bodies, so an alias to a declaration edited before removal retains the edited value. The previous snapshot remains available to intermediate journal events and existing published caches; it cannot be altered by later edits.

Fresh clone first resolves any graph edge through the snapshot's current view: object semantic ObjectId selects its current COW storage; array original identity selects its current spliced view. Only then does the clone memoize resolved object/array storage and allocate fresh object semantic identity. Thus an `extra.child` edge pointing to old object storage and a `nodes[0]` edge pointing to updated COW storage converge to one effective object before memoization. Merely changing PublicValue.clone_fresh to memoize semantic identity globally would collapse unrelated storage and break fee accounting; canonicalization is scoped to a known OriginGraph snapshot.

One EffectiveClone spans the entire clone root, all node origins, src/dst tuples and arbitrary fields. Native declarations/source identities retain their current old-to-fresh maps. Native locations with public metadata bind to the corresponding cloned full-graph values through that same memo. Distinct equal objects stay distinct. Independent clones get independent snapshots and public identities. Apply's second clone resolves the finalized first clone snapshot rather than its original caller fields. Functional handler clones the full @utility graph because wrapper metadata can refer into its body; it returns body native nodes and does not spread definition fields onto the generated class wrapper (`ref/.../utilities.ts:5928–5929,6053`). Static handler clone root is the body array (`6095–6098`).

An immutable snapshot does retain a metadata map over public fields; this is accepted storage, not a mutable side registry silently bypassing fees. Scope ends when its last native origin or staged public value dies. Phase builders disappear immediately after snapshot publication. The implementation must prune unreachable map entries after preserving edges reachable from roots, body metadata, and journal values; absence of pruning is a memory regression and candidate failure criterion.

### Raw versus optimized projection

Raw: start from the effective attached origin object, preserving semantic identity for authored objects or freshly cloned bodies. Replace typed known fields/child arrays in their existing key positions. For generated class wrappers, create only standard kind/selector/nodes; definition fields do not leak. For origin-less generated descendants retain current generated/staged declaration policy.

Optimized: declarations and comments reuse raw public objects; surviving rule/at-rule wrappers shallow-spread raw effective fields into a fresh PublicObject identity, replace nodes with optimized children, and keep metadata references shared. Do not invoke full graph fresh clone at this stage. Context and AtRoot origins are available for raw projection but their wrappers disappear during optimization; their extra fields do not migrate to children. This matches upstream declaration reuse and wrapper spread (`ref/.../ast.ts:349,391,430,443,465–490`). The existing raw CompiledCandidate cache continues to own stable raw projections (`compile_ast.rs:1117–1135`); optimized no-op returns the old array (`1150–1157`).

True missing src/dst survives: fields are copied only if originally present or explicitly written. SourceLocation remains the actual typed span override, not the origin transport. Apply overwrites src on all cloned descendants; it does not discard dst/extra aliases from the cloned body. Existing authored structural heuristic stays as fallback for origin-less legacy nodes initially; the custom/input origin path does not consult selector/CSS overlap. Remove redundant declaration origins only when all existing public behavior tests remain green, not as incidental cleanup.

### Original body writes and phase journal

The upstream sequence is initial theme array replacements, registration/normalization, final theme nodes assignment, authored variants, functions, apply, utilities context writes, and final utility removal (`ref/tailwindcss/packages/tailwindcss/src/index.ts:528–685`). This phase description was corrected after the anonymous judgment; the original sketch incorrectly combined the two theme moments. Construct journals at actual edit sites, in that phase order; do not append all normalization and functions after theme/apply because final values happen to match. Registration normalization always records the matching declaration assignment (`utilities.ts:5796–5801,5925`), even unchanged. Generic functions journal only assignments the upstream guard executes; current native substitute calling in_text_value on every declaration is insufficient to decide event presence (`functions.rs:351–375`; upstream css-functions.ts:163,177).

Custom definitions remain rooted for setter targets and old array effects after their removal from input. Registration records `--value(--color)` becoming `--value(--color-*)`. Functions can later assign a distinct value to that same child: journal retains both values and their order. Apply splices old definition body arrays before removal. Replay remains InputMutation Set/Splice; Set uses Reflect.set and caller splice is called as a function with the target receiver (`wasm/.../graph.rs:258–294`). No rollback of an executed JS prefix is promised. Quota checks happen before replay; host crossing failures retain admitted candidates and restore only last output, per existing contract.

Phase journals capture native planned assignment order, not arbitrary host getter/Proxy re-entry. Full host observation timing beyond existing regression still requires explicit future evidence; this candidate does not relabel that gap as conformance. Immutable ingestion remains the existing boundary.

### Fee accounting and transaction ownership

Introduce `OriginUsage` composed of seen graph snapshot Arc pointers, seen object/array storage, and seen semantic identity, with checked/saturating accumulation using existing PublicValue `json_bytes_seen`. Count each snapshot/header/map node, owner retention handle, roots/vector capacity and COW view map entries, plus recursive field/key/Text graph storage. Semantic identity dedup does not dedup COW field storage (`design_system.rs:488–530`). Canonical view lookup can use semantic identity; allocation accounting still uses storage.

Initial capture is charged with input/origins/journal under the same shared seen accounting context; it replaces existing origin-map duplicated field ownership rather than charging each attached handle as a deep graph. Each attached handle incurs its struct slot/Arc allocation only as represented by AstNode size and snapshot owner counts. Definition recount includes its origin graphs together with typed storage. Candidate admission includes fresh origin snapshots and Source graphs in one seen traversal, preventing src/dst/extra duplication or omission. The initial sources_cache_bytes baseline must become retained public graph baseline, using all setup roots before extraction; eliminate source-only growth deltas for origins rather than subtracting incompatible meters (`mod.rs:224–242,297–299`).

Phase temporary overlays, clone memo and journal intermediate snapshots have a peak reserve checked before allocating proportional maps/copies; cloning must preflight effective reachable public graph bytes and node count. When a phase fails, only its private values and uncommitted native setup/prepared state exist. Build adds raw projection/generation/output/journal views under existing staged fee boundary before candidate commit (`compile_ast.rs:1183–1240`). Successful commit publishes candidate origin snapshots and generated/raw/public projections together. Rejected quota leaves the original Compiler/Origins snapshots and facade handles intact. The WASM retained handle fees/output graph preflight remain additive.

Exact constant estimates need implementation measurement; passing the old CSS-only accepted-candidate fee is insufficient because arbitrary fields can dominate storage. No candidate may bypass cache_bytes with a large metadata DAG, and no warm no-op may monotonically accumulate snapshot fees.

### Evidence and validation contract

Keep all existing custom-input, raw/apply/input-mutation/source/graph/budget regressions. The current static RED is fixed by carrying whole fields, not filtering `extra` from expected snapshots. Required discriminators before acceptance:

- Full existing custom-input runner, with constructor and every incremental graph snapshot identical to pinned upstream.
- One cloned body graph sharing Source through src/dst/extra and two equal-but-distinct Source objects; compare all reference relationships, field key order and fractional/lone UTF-16 values.
- Alias metadata pointing to a functional child whose value is normalized then function-substituted; confirm cloned metadata sees current value and aliases cloned child. Include an alias to the body nodes array changed by @apply.
- Repeated matching declarations whose normalization assignment is unchanged; setter trace must include assignment. One declaration with normalization and later function write must report both intermediate values in upstream order.
- Nested rule/at-rule/comment/context/at-root arbitrary fields: raw fields preserved, wrappers spread/removed correctly, generated class wrapper avoids definition fields, surviving declarations/comments reused across raw/optimized output.
- Exact frozen/nonwritable/setter/splice thrown sentinel and successful-prefix comparisons; cache/output restoration remains unchanged.
- Low cache limits covering capture, phase overlay, clone memo and staged graphs; no native/facade publication or caller writes before quota admission; repeated no-op usage stable.

No tests were run for this sketch. These are required implementation gates, not claimed results.

## Synthesis decision

Pending orchestrator synthesis. this package is intentionally the attached-origin intervention, not a claim that competing candidates were reviewed. Original rationale records favor single raw candidate facts and explicit effects, not a mandate for this particular carrier.

## Tradeoffs accepted

- We accept changing all six enum variants and updating direct literal/pattern call sites in exchange for source absence remaining independent of provenance. Ground search found ten exact AtRoot `{ nodes }` patterns and 320 constructor/pattern/test reference lines, not 320 constructors.
- We accept one bounded immutable effective public graph snapshot per semantic clone scope/phase in exchange for alias fidelity across COW views and references to removed nodes. We do not accept a second optimized native candidate cache.
- We accept a private phase editor parameter at actual mutable transforms in exchange for original body setter traces and intermediate values coming from execution rather than final diffs.
- We accept extra metadata storage and explicit temporary budget fees in exchange for preserving arbitrary enumerable fields under the current adapter's representation. Full Proxy/descriptors/symbol semantics remain outside the established adapter contract.

## Alternatives considered

- `SourceLocation.public` as the only full origin carrier: small patch surface, but leaks provenance into source presence, cannot represent Context/AtRoot, and spreads cloning policies through span handling. It hides little of the actual graph problem and loses on required behavior.
- Keep full fields in public Origins and recover custom children by CSS shape/old-to-new declaration ID tables: fewer enum changes but forces clone/apply/public callers to coordinate mappings and wrapper guesses, with metadata-only aliases still stale. Larger distributed interface exposes clone internals to every transform.
- Attached `Option<Arc<PublicValue>>` with direct fresh-clone and no effective views: smallest code sketch but wrong when aliases reach old storage after native normalization or apply edits. Memoizing COW storage splits one semantic object; memoizing semantic identity globally collapses distinct field-storage views. It fails the alias discriminator and is rejected.

## Open questions and risks

- Can graph capture/conversion be fused cleanly without a second full traversal or public object normalization changing getter timing? The candidate assumes current immutable ingestion semantics, not deeper host equivalence.
- Can snapshot rebinding cover every retained root that shares a caller body object—including duplicate definitions and theme resources—without retaining old snapshots unnecessarily? An implementation missing one root must be scrapped or redesigned, not patched with shape fallback.
- Does the full functional definition graph carry metadata aliases to its definition wrapper that survive through body references? The sketch includes the wrapper graph clone root; a body-only simplification would need a pinned upstream counterexample check.
- Can phase peak accounting use actual map capacities and graph reachability before clone allocation with reasonable overhead? This is unmeasured; failure to bound the peak is a hard design risk.
- Does generic function/apply traversal order agree with upstream across extracted definitions and nondefinition input? Native setup currently processes custom functions separately before input functions. Journal collection must use original traversal order or explicitly reorder based on captured input occurrence order; final-value equality is not adequate evidence.

## Red-flag screen

| Flag | Screen result | Evidence / required correction |
| --- | --- | --- |
| Shallow module | Conditional pass | OriginPhase performs canonical edits, journaling, private snapshot ownership and fee admission behind assign/splice/finish; callers do not manually stitch old/new identities. If callers must maintain overlay maps themselves, reject. |
| Information leakage | Pass in sketch | No JS transport identity/projection mode enters public API; full fields remain in private origin module. SourceLocation has only source responsibility. |
| Temporal decomposition | Conditional pass | Stage enum only captures observable journal order; metadata knowledge resides in one origin module. Do not create separate load/normalize/write origin modules or expose phases as public configuration. |
| Pass-through methods | Pass in sketch | assign_value/params/splice add identity, journal and fee policy; output mode changes wrapper identity behavior. Helpers forwarding unchanged parameters without policy should not be added. |
| Hidden duplicate facts | Conditional pass | Public snapshots retain observable field views, raw native cache retains CSS candidate facts. No duplicate optimized native cache. If origin views become another native compiler tree, reject. |
| Shared writable state | Pass in sketch | Phase-local overlays are owned values; snapshots are immutable Arcs; Prepared cannot mutate committed graph through carrier. |

Interface depth is improved for projection and clones because complete provenance is exact and local; it is worse for mutation transforms because phase edits become explicit. This is the principal comparative cost for synthesis, not something to disguise as a tiny field addition.

## Next implementation step

Build the origin slot plus one bounded effective fresh-clone slice for the static body and COW-normalized metadata alias discriminator, including storage fees, before migrating all constructor mutation phases.

Mandatory completion scope is the full existing graph/mutation/source/budget regression contract, including all six node kinds, effective clone aliases, original-body writes and exact journal phases. The first isolated slice may validate only origin capture/effective clone/COW alias propagation. It is a staged feasibility experiment, not a reduced acceptance gate, and must not be reported as completion or used to remove later phase coverage.
