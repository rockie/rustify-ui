> Historical design-only candidate. Setup order corrected after judging; all unresolved interface, error-prefix, equality, compatibility and fee blockers are governed by synthesis.md. This is not an implementation contract.

# Candidate B: Compiler-owned public mirrors and explicit clone transactions

Status: design sketch only; no repository changes, implementation, builds, tests or runtime claims. Read `native-ground.md`, `public-ground.md`, `history-ground.md`, architect runner/rationale/red-flags references and agi-mode Arena constraints. Intervention supplied by the parent: explore out-of-line clone lineage/public mirror ownership, rather than attaching a complete `PublicValue` to each node. This intervention is recorded; candidate differences must not be attributed solely to model choice.

## Problem

Custom cloning replaces declaration identity while complete caller fields remain behind old identities in `Origins`; other kinds lack stable occurrence identities and projection reconstructs fixed fields. CSS shape cannot select the right clone when equal definitions differ by metadata. We must preserve arbitrary enumerable own string fields, body updates, full Source/src/dst alias graphs and key order, while keeping the single raw native candidate cache, render-only optimization, precommit quota rejection and existing crossing failure semantics. The proposed owner is Compiler, with a private mirror graph and transaction-local edits. Nodes contain a small stable identity only, never a full public payload. The mirror is the public observable graph/projection representation; it does not compile, sort, calculate GPU declarations or become another native candidate cache. This is a larger change than a node payload; its feasibility depends on routing identity-changing and observable mutation operations through one native edit boundary.

## Usage (caller's view)

The external Rust and WASM APIs remain unchanged. A consumer still constructs `AstCompiler` and calls `build`, receiving the caller input for Features.None and a stable optimized array on a no-op build.

```javascript
const compiler = new facade.AstCompiler(input, JSON.stringify({ limits }))
const first = compiler.build(['card'])
const second = compiler.build([]) // same handle as first
```

Internal call site 1: public ingestion and staged crossing. The Compiler owns the decoded mirror after construction; the facade owns real JS handles only.

```rust
let decoded = graph::read(&input_js, &limits, &mut budget)?;
let compiler = Compiler::from_public_ast(decoded.value, options, assets, limits)?;
// Constructor result includes fully reserved effects; no separate mirror owner.
let setup = compiler.setup_publication();
input_graph.write(&[], setup.effects(), &Graph::default())?;

let prepared = compiler.prepare_public_build(&candidates)?;
input_graph.check_crossing_budget(prepared.fees(), prepared.publication())?;
let publication = compiler.commit_public_build(prepared)?;
// Admission precedes host replay, preserving existing admitted-candidate semantics.
match input_graph.write(publication.nodes(), publication.effects(), &previous_graph) {
    Ok(result) => publish(result),
    Err(error) => { compiler.restore_output_after_crossing_error(previous); return Err(error); }
}
```

Internal call site 2: static/functional custom handler cloning. Static clones the body array root; functional clones the entire definition object root and then selects its cloned body. This matches upstream `utilities.ts:6097` versus `:5929,6053`; the definition wrapper itself does not become the class rule.

```rust
let body = edits.clone_definition(&definition, candidate.kind())?;
let mut nodes = body.nodes;
resolve_functional_values(&mut nodes, candidate, theme, &mut edits)?;
let class = edits.generated_rule(selector, nodes); // fresh ID, standard fields only
```

Internal call site 3: normalization and apply body mutation. Each operation changes native facts and journals the observable edit together. The caller supplies the semantic value, not the transport `PublicValue`.

```rust
// Executed even when normalized == old text; setters remain observable.
edits.assign_declaration_value(child, normalized)?;
// Apply's owner is the parent array handle and removed node ID, not a Source tuple.
edits.replace_body_occurrence(parent_body, active_index, apply_node.id(), expanded)?;
```

## Shape

### Data structures

All types below are private implementation types except the existing exported AST enum. Sketch bodies intentionally remain `not implemented`.

```rust
// Strong owners eliminate pointer-reuse ambiguity; equality/order use token address.
// Do not expose pointer-valued IDs or clone reason to facade consumers.
#[derive(Clone)]
pub struct NodeId(Arc<NodeToken>); // opaque type only; field remains private
struct NodeToken;
#[derive(Clone)]
struct ObjectId(Arc<ObjectToken>);
struct ObjectToken;
#[derive(Clone)]
struct ArrayId(Arc<ArrayToken>);
struct ArrayToken;
#[derive(Clone)]
struct CloneId(Arc<CloneToken>);
struct CloneToken;

// Mandatory ID for every kind. Existing declaration identity becomes this same ID,
// avoiding two independent declaration identity invariants.
enum AstNode {
    Rule { id: NodeId, source: Option<SourceLocation>, destination: Option<SourceLocation>,
           selector: Text, nodes: Vec<AstNode> },
    AtRule { id: NodeId, source: Option<SourceLocation>, destination: Option<SourceLocation>,
             name: Text, params: Text, nodes: Option<Vec<AstNode>> },
    Declaration { id: NodeId, source: Option<SourceLocation>, destination: Option<SourceLocation>,
                  property: Text, value: Text, important: bool },
    Comment { id: NodeId, source: Option<SourceLocation>, destination: Option<SourceLocation>, value: Text },
    Context { id: NodeId, base: Text, source_base: Option<Text>, reference: bool, nodes: Vec<AstNode> },
    AtRoot { id: NodeId, nodes: Vec<AstNode> },
}

enum MirrorValue {
    Null, Bool(bool), Number(serde_json::Number), String(Text),
    Object(ObjectId), Array(ArrayId), Undefined,
}
// Full ordered own-property graph. References are handles, so aliases through an
// extra field to an AST object or body array are preserved, including detached nodes.
struct MirrorObject { fields: IndexMap<Text, MirrorValue> }
struct MirrorArray { values: Vec<MirrorValue> }
struct MirrorEntry { object: ObjectId, body: Option<ArrayId>, lineage: Lineage }
enum Lineage {
    Caller,
    Generated,
    DeepClone { source: NodeId, group: CloneId },
    ShallowSpread { source: NodeId },
}

struct PublicMirrors {
    // One lookup, independent of CSS text, selector, sibling position or src.
    nodes: BTreeMap<NodeId, MirrorEntry>,
    objects: BTreeMap<ObjectId, Arc<MirrorObject>>,
    arrays: BTreeMap<ArrayId, Arc<MirrorArray>>,
    // PublicObject semantic identity / Array Arc ownership to mirror graph identity.
    // Ingest memo entries hold the original Arc owners, never bare pointer keys.
    ingest: IngestAliases,
    // Emitted public graph maps mirror handle to strong PublicValue owner.
    projected: ProjectedHandles,
}

// Immutable committed base plus actor-local edits. No Arc<Mutex<_>>/global registry.
struct MirrorTxn<'a> {
    base: &'a PublicMirrors,
    nodes: BTreeMap<NodeId, MirrorEntry>,
    objects: BTreeMap<ObjectId, Arc<MirrorObject>>,
    arrays: BTreeMap<ArrayId, Arc<MirrorArray>>,
    effects: Vec<CallerEffect>,
    projected: ProjectedHandles,
    reservations: Reservation,
}
struct NativeEdits<'a> { public: Option<MirrorTxn<'a>>, limits: &'a Limits }
// Both typed Source identity and canonical public graph use this SAME clone session.
struct CloneSession {
    group: CloneId,
    nodes: BTreeMap<NodeId, NodeId>,
    objects: BTreeMap<ObjectId, ObjectId>,
    arrays: BTreeMap<ArrayId, ArrayId>,
    sources: BTreeMap<SourceOwner, Arc<Source>>,
}
enum CallerEffect {
    Set { target: ObjectId, field: Text, value: MirrorValue },
    SetIndex { target: ArrayId, index: usize, value: MirrorValue },
    Splice { target: ArrayId, start: usize, delete: usize, values: Vec<MirrorValue> },
}
enum Projection { Raw, Optimized }
struct Definition {
    // Owned native body remains the only candidate compilation fact.
    node: NodeId,
    body: ArrayId,
    name: String,
    nodes: Vec<AstNode>,
    completion: CompletionMetadata,
}
struct PublicPublication {
    nodes: PublicAst,
    effects: Vec<InputMutation>, // Lowered at the native/public boundary only.
    fees: PublicationFees,
}
```

`Context`/`AtRoot` receive independent identity without fabricated SourceLocation. Their public src/dst/extra fields remain in the mirror graph even though they do not have typed spans. Known optional fields retain presence in the ordered map: omitted and explicitly undefined are different. A NodeId belongs to an observable object occurrence, not to its text. During public ingestion, repeated references to the same caller AST object reuse the same NodeId and body handle, while equal distinct objects receive different IDs. Native parser/generated constructors always allocate fresh IDs. Derived Clone preserves IDs and mirrors. This encodes occurrence provenance directly, per the domain-models-and-types principle.

The mirror graph is necessary rather than independent scalar extra maps: an extra may refer to another node, the original body array, or the same Source reached through src/dst. Mirror objects survive node removal when reachable through caller roots, pending effects or arbitrary graph edges. Detached public nodes retain their last observed fields. Mirrored CSS fields are a boundary snapshot of native facts; native typed fields remain authoritative for compilation. Mirror updates occur in `NativeEdits` at actual assignments and replacements, and projection overlays typed facts before publication. No sorting/emission algorithm reads a cached mirror scalar to determine CSS. This explicitly accepts synchronization work rather than hiding it as a later map.

### Signatures and ownership

```rust
impl AstNode {
    fn id(&self) -> &NodeId { todo!("not implemented") }
    fn set_id(&mut self, id: NodeId) { todo!("not implemented") }
    // Existing constructors allocate IDs, preserving existing external convenience.
}
impl Compiler {
    fn from_public_ast(input: PublicValue, options: CompileOptions,
        assets: Vec<Stylesheet>, limits: Limits) -> Result<Self, CompileError> {
        todo!("not implemented")
    }
    fn prepare_public_build(&self, candidates: &[Text]) -> Result<PreparedPublicBuild, CompileError> {
        todo!("not implemented")
    }
    fn commit_public_build(&mut self, prepared: PreparedPublicBuild)
        -> Result<PublicPublication, CompileError> { todo!("not implemented") }
}
impl NativeEdits<'_> {
    fn clone_definition(&mut self, definition: &Definition, kind: UtilityKind)
        -> Result<ClonedBody, CompileError> { todo!("not implemented") }
    fn clone_subtree(&mut self, nodes: &[AstNode])
        -> Result<Vec<AstNode>, CompileError> { todo!("not implemented") }
    // Caller effects derive from mirror lineage, never a duplicated caller flag.
    fn assign_declaration_value(&mut self, node: &mut AstNode, value: Text)
        -> Result<(), CompileError> { todo!("not implemented") }
    fn assign_at_rule_params(&mut self, node: &mut AstNode, value: Text)
        -> Result<(), CompileError> { todo!("not implemented") }
    fn replace_body_occurrence(&mut self, body: &ArrayId, index: usize, removed: &NodeId,
        nodes: Vec<AstNode>) -> Result<Vec<AstNode>, CompileError> { todo!("not implemented") }
    fn generated_rule(&mut self, selector: Text, nodes: Vec<AstNode>) -> AstNode {
        todo!("not implemented")
    }
    fn spread_wrapper(&mut self, node: &AstNode, nodes: Vec<AstNode>)
        -> Result<AstNode, CompileError> { todo!("not implemented") }
    fn project(&mut self, nodes: &[AstNode], mode: Projection)
        -> Result<PublicAst, CompileError> { todo!("not implemented") }
    fn stamp_sources(&mut self, nodes: &mut [AstNode], source: Option<&SourceLocation>)
        -> Result<(), CompileError> { todo!("not implemented") }
}
struct PreparedPublicBuild {
    native: Prepared,
    mirror: MirrorDelta,
    publication: PublicPublication,
}
```

The external wrapper is not made to call clone/normalize phases. `NativeEdits` is a private capability borrowed by existing domain transforms; it hides mirror handles, alias maps and fee reservations, per boundary discipline. Constructor/setup uses a private fresh mirror owner, so setup failure discards it. Builds borrow committed state immutably and own delta writes, then one admission merges candidate and mirror deltas. The facade's graph remains a JS crossing adapter and does not own native mirrors. These two actors therefore never mutate one shared mirror, per state ownership.

### Dominant access patterns

1. **Conversion:** index every caller object/array identity once while reading the existing public graph. Register a NodeId for each AST object during native conversion; do not inspect arbitrary metadata objects as AST based on a `kind` string. If later encountered through a real `nodes` edge, bind its existing object handle to that NodeId. All six kinds use the same registration rule.
2. **Functional registration:** `assign_declaration_value` writes native Text, COW-updates the mirror object's ordered `value` field and appends a caller Set unconditionally for each upstream assignment. Static bodies skip normalization. The definition's `body` remains the original ArrayId, even after the definition disappears from input.
3. **Fresh clone:** freeze the mirror view at the clone boundary, traverse the full selected public root once with one CloneSession, remap native NodeIds through the same node table, and attach cloned public tuples to typed SourceLocations via the same object/array map. Typed Source Arc memo remains independent because equal typed and public representations are different domains, but their public tuple graph is not cloned twice. Clones have fresh NodeId/ObjectId/ArrayId; within-clone sharing survives. No textual matching or original-ID heuristic is used. A functional clone includes the definition container root before selecting children; its extra fields stay on that cloned container only.
4. **Functional/generic substitutions:** all scalar AST mutations relevant to public observation use NativeEdits. Mutations of generated objects do not append caller Set, but update their clone mirror. Dropping a declaration removes its body reference; it does not erase an object still reached through an extra field. Thus detached aliases preserve the actual in-place substitutions they observed before removal.
5. **Apply body update:** use the explicit ArrayId, active index and removed NodeId to validate the exact occurrence in that known array. Repeated same-object occurrences are distinguished by the active index supplied by the native body walk, not a search for first equal NodeId. Single replacement journals SetIndex; zero/multiple journal Splice; update the mirror's array view sequentially. Apply returns the same native replacement nodes written into the mirror, avoiding a second expansion. Original old arrays remain observable. Definition extraction/removal occurs after these recorded operations in the public journal even though native extraction happened earlier for compilation.
6. **Apply second clone/source stamp:** clone_subtree runs another independent CloneSession. `stamp_sources` overwrites `src` on every cloned descendant in the mirrored ordered map, including context/at-root where applicable, and replaces typed sources on the four supported kinds. It preserves cloned dst and extra graph references. Shared apply candidate tuple remains one tuple; existing caller Source identity is used for the override as upstream requires.
7. **Raw/optimized:** raw candidate mirror persists with the single `Arc<CompiledCandidate>` owner. `spread_wrapper` allocates a fresh NodeId and ObjectId, shallow-copies ordered fields, preserves extra/src/dst references, and supplies a new transformed nodes array. Optimized declarations retain raw NodeId/ObjectId. `&`, context and at-root elimination transfer only surviving children; metadata on the eliminated wrapper is not copied to child/generated class nodes. Raw `--tw-sort`, duplicates and empty wrappers remain in raw mirror and raw AST.
8. **Projection:** one graph materializer resolves MirrorValue handles using semantic ObjectId/ArrayId and ordered fields. Native nodes overlay authoritative known scalar fields in place without reordering keys. New synthesized fields append in upstream order. The existing PublicObject identity is seeded from the ObjectId's strong token so COW views synchronize the intended JS object; optimized spreads always get a different token. ProjectedHandles retains owners for every pointer-derived index. Projecting raw and optimized uses the same delta so declarations and metadata graph objects alias correctly.

### Phase journal and failure semantics

The journal belongs to semantic Compiler setup/build, not separate load/normalize/save modules. Internal scope tagging may organize production, but external calls never expose phases. Constructor order is initial theme array replacements → custom normalization → final theme nodes assignment → authored variant/function edits → apply body edits in dependency order → utilities kind/context → utility array removal. This phase description was corrected after the anonymous judgment; the original sketch incorrectly combined the two theme moments. The old body array references stay anchored until no caller/effect/projected graph owns them. Journal values are immutable snapshots at the time of each assignment; targets are stable handles, not regenerated graph positions.

Lowering mirrors to InputMutation must not synchronize the final version of a target before its earlier journal operations. Project/marshal mutation values first and preserve chronological snapshots; use the retained original target handle for Reflect.set or its actual splice method. Earlier values that alias an object later edited require their staged current view to advance in replay order, not eager last-version replacement. This is an explicit obligation of the lowering routine. A useful invariant is that each operation's reachable value graph is materialized from the journal prefix's mirror version, with alias tokens held constant across later prefixes.

Preflight reserves complete native delta, mirrors, publication and JS handles before native admission and before any JS effects. Quota rejection discards delta and preserves committed candidate/output identities. After admission, host setter/splice failure retains candidates/counters and already executed JS effect prefix; only the published optimized array is restored. The failed journal is not automatically retried on a no-op build. Constructor failure exposes no Compiler but may leave the same successful JS prefix as upstream. There is no blanket idempotent replay claim: JS accessor effects are deliberately not idempotent.

### Fee accounting

- Universal NodeId token and per-node handle allocation are charged in AST storage, replacing existing declaration identity fees instead of double counting them. Parser/native-only callers incur ID cost even without mirrors; this is an accepted cost, not a free optional feature.
- MirrorBook charges map capacity/entries, strong tokens, ordered field storage, Text/number payloads, array capacity, lineage handles and projected handle owners. One actual object/array storage is charged once in a shared seen set; COW field versions are separately charged while live. SourceLocation public tuples and mirrored src/dst/extras share the same value graph fee domain, so the same stored subgraph is not billed twice.
- CloneSession reserves per-entry memo storage and copied public/native allocations before publishing fresh handles. Deep clones are actual new storage and are fully charged even when lineage points back to older nodes. Shallow spreads charge object/map/array storage plus references, not another deep metadata tree.
- Setup journal snapshots, build delta/raw graphs, detached referenced objects, current output and crossing estimates are all retained owners. Failed/obsolete transaction roots are dropped; map garbage is collected from input/custom/cache/current publication/effect roots at transaction release. Old JS output handles retained by consumers do not keep native mirrors alive once no native owner exists; facade caches follow current graph policy.
- `json_bytes_seen` remains authoritative for materialized PublicValue allocation; MirrorGraph's own storage is independently charged because it is a distinct allocation. Expanded wire `input_bytes`, cached candidates, queued inputs and CSS limits retain their current semantics. `Graph::output_bytes` covers JS publication; it does not substitute for MirrorGraph fees.
- Admission sums committed native/mirror fee + transaction delta + simultaneously live publication/COW versions + crossing reserve. A mirror entry reachable only via extra metadata is still charged. No fixed one-constant-per-lineage estimate may substitute for traversed graph ownership.

### Module map and migration boundary

- `compiler/ast.rs`: universal identity, constructors, source helpers and native clone/optimizer edit calls. Existing direct enum consumers must receive `id` fields or use constructors; exact `{ nodes }` patterns must be updated. This cannot be honestly sold as a small local patch.
- New `compiler/public_api/mirrors.rs`: private MirrorGraph, clone transaction, ordered journal, graph projection and allocation ownership; all graph identity rules live here.
- `compiler/mod.rs`: sole owner of optional mirror book, transaction creation/admission and publication lifecycle; existing single raw candidate cache retained.
- `custom_utilities.rs`, `apply.rs`, `functions.rs`, `variants` clone sites: borrow NativeEdits for actual assignments, body replacements and fresh clone boundaries. They do not manipulate mirror tables.
- `public_api/compile_ast.rs`: replace custom-path OriginKey recovery and generated-declaration-only map with exact NodeId projection. Existing theme/import construction can migrate incrementally only where converted nodes already carry IDs; do not leave a heuristic fallback for a node registered in the mirror book.
- WASM `public_api/graph.rs`: reuse real handles, Set/Splice exceptions and staged fee crossing; lower prefix snapshots without eagerly applying final target mirror state.

## Synthesis decision

Reserved for the parent Arena synthesis. Candidate B has not read other candidates and makes no winner claim.

## Tradeoffs accepted

- We accept mandatory identity fields and constructor/pattern migration across six kinds in exchange for exact provenance without CSS/path heuristics.
- We accept explicit out-of-line mirror synchronization at semantic edit points in exchange for keeping PublicValue ownership off native nodes and centralizing host graph policy.
- We accept a retained public observable graph in addition to typed raw AST in exchange for preserving arbitrary references to detached nodes and chronological caller effects; there is still only one native candidate compilation tree.
- We accept transaction-local copy-on-write mirror maps and fee overhead in exchange for quota rejection without corrupting committed mirror identities.
- We accept that eager graph ingestion cannot establish complete Proxy/getter timing equivalence in exchange for preserving the existing adapter boundary; this candidate must not advertise full JS language compatibility.

## Alternatives considered

- Complete PublicValue payload on every node: shorter edit call chains and naturally clone-carried metadata, but native nodes expose ownership of canonical public graph; Source/extras clone memo and actual assignments still need careful coordination. Candidate B hides representation in one owner at the price of more edit plumbing.
- Map only old declaration ID → new declaration ID: shallow interface and few edits, but cannot represent all six kinds, duplicate wrapper occurrences, metadata alias graphs or body effects. It loses the required provenance rather than hiding complexity.
- Pointer address of native AstNode as side-table key: impossible with movable Vec/arena nodes and plain clones. Without explicit identity, out-of-line ownership is untenable. This is why Candidate B uses mandatory small identity tokens.
- Path/index/selector/CSS-shape lineage: hides a map behind projection but leaks positional/structural assumptions through every transform; detached arrays and independently equal definitions invalidate it.

## Open questions and risks

- Can all relevant in-place scalar mutation/clone/body replacement sites be enumerated and migrated in one scoped change, so a direct write cannot silently skip mirror or caller effect updates? This is the central viability risk, rather than a future optional audit.
- Will lower-prefix materialization preserve alias identity when an effect value references an object changed later, without final-state projection invoking setters early? A sentinel trace plus stored-old-node alias fixture must decide this before accepting the transaction shape.
- What do the pinned upstream fixtures require for metadata references back to removed AST objects/body arrays? The mirror representation can retain them, but the exact assignment/removal visitation semantics need direct differential evidence.
- Is the all-kind ID cost acceptable for native-only use when full compiler code requires a wide constructor migration? No unmeasured performance improvement is claimed.
- Which existing public AstNode consumers outside this workspace construct enum literals? Internal migration is visible, but source compatibility of adding mandatory fields is an explicit API risk.

## Red-flag screen

| Flag | Screen result |
| --- | --- |
| Shallow module | Revised: public consumer still has constructor/build; private NativeEdits performs clone + alias ownership + journal + reservation atomically. Domain transforms choose semantic edits but never coordinate several mirror-table methods. Risk remains if implementation splits it into clone/register/recount calls. |
| Information leakage | MirrorGraph, clone reasons and handles stay private; only small NodeId belongs in AST. Known CSS snapshot synchronization is centralized. Optional public mode is carried by one edit capability, not duplicated boolean policy across transforms. |
| Temporal decomposition | One mirror owner contains setup/build journal and clone/projection knowledge. No separate NormalizeMirrors/ApplyMirrors/OutputMirrors modules. Existing domain phases remain because upstream effects are observable in that order. |
| Pass-through methods | No per-phase forwarding facades. `assign_declaration_value` adds coupled native/public update and journal policy; `clone_definition` adds whole-root clone semantics and source alias handling; `spread_wrapper` encodes different identity semantics. |

Paper screen outcome: viable but materially larger synchronization/migration surface than a payload carrier. Reject B if the implementation cannot route every required mutation through NativeEdits or cannot reserve prefix snapshots without hidden final-state writes. No executed validation supports viability yet.

## Next implementation step

Build one identity/mirror transaction slice for the static custom fixture, including extra→src/dst/body aliases and raw-versus-optimized shallow wrapper identity, before migrating generic function/apply mutations.
