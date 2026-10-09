# Registration proposal

## Usage first

Native setup consumes parsed nodes into private shared values before registration. Public setup supplies the original shared values before `Origins::convert` can discard presence or positions. Both prepare through one registration owner:

```rust
// Native: adaptation belongs to prepare, before registration.
let prepared = registration::prepare(Input::Parsed(parsed), bundle, &limits)?;
let compiler = prepared.into_native_result()?;

// Public: existing crossing owns admission, handles and reaction delivery.
let prepared = registration::prepare(Input::Public(input), bundle, &limits)?;
crossing.admit(prepared, input_handles)?.start(entry);
```

A semantic rejection retains its exact completed effects; resource rejection authorizes no compiler AST writes. Neither caller selects extraction passes. These signatures are sketches, not implementation or acceptance evidence.

## Problem and rationale

The expanded manifest records 45 registration cases, 90 comparisons and 86 overlapping failure records; full conformance remains pending. Ground describes competing typed-tree passes, object views and array views. This alternative makes existing `PublicValue` graphs plus extended `Edits` the sole registration input/current store. It introduces no canonical node/list arena and no universal identity topology. Existing provenance connects identity to storage; source spans and `OriginKey` shapes never become identity substitutes.

This follows the immutable snapshot/host-effect separation in [why.md](why.md), while removing the current dependence on refreshing a second typed tree. It is conditionally viable: present `Edits::assign` and `sync` are insufficient without structural operations.

## Shape and ownership

`Edits` owns sparse original/current views, not a second tree. Objects retain their existing `PublicObject.identity`. An array's original `Arc<Vec<Value>>` is its identity anchor. Splice or index assignment creates a new storage version under that anchor; every occurrence resolves the anchor's latest contents. A reverse storage-to-anchor association covers every retained version, pins its owners and distinguishes an edit from assigning a genuinely different array. Identity is inherited only through an explicit mutation, never by equality. Private fresh clones receive fresh anchors while preserving aliases inside their clone group.

Only the owner interprets these associations. Cursor frames retain original array anchors, current indices and semantic parent/context; they re-resolve contents after every mutation. Shared aliases therefore see deletions and field changes before their next visit. Detached definition bodies remain reachable through their owners. Assignment replaces an edge, while splice changes the referenced array everywhere; neither operation rebuilds unrelated parents.

Effects retain both their original target and assignment-time storage versions. A newly allocated value is projected from that captured version. An existing caller alias reuses its live handle, without rehydrating its members from an older or later snapshot. Same-value assignments remain events. Strong retained values prevent address reuse; publication can release versions only after effects, pending import work and escaped handles no longer require them.

```rust
// All declarations below are private compiler implementation detail.
struct Registration {
    root: PublicAst,
    current: Edits,
    state: RegistrationState,
    effects: Vec<Effect>,
    ledger: SetupLedger,
}
struct Captured { anchor: PublicValue, version: PublicValue }
enum Input { Parsed(Vec<AstNode>), Public(PublicAst) }
enum Effect {
    Assign { target: NodeOrigin, field: Field, value: Captured },
    Replace { target: PublicAst, at: usize, deleted: usize, items: Captured },
}
enum DomainNode<'a> {
    AtRule { name: &'a Text, params: &'a Text, body: Option<&'a PublicAst> },
    Declaration { property: &'a Text, value: DeclarationValue<'a>, important: bool },
    Other, // Rule/comment/context/root variants omitted from this bounded sketch.
}
enum DeclarationValue<'a> { Missing, Undefined, Null, Text(&'a Text) }
fn prepare(input: Input, bundle: Bundle, limits: &Limits)
    -> Result<PreparedRegistration, CompileError> { todo!() }
impl Registration {
    // Domain decoding and mutation-aware traversal stay inside this owner.
    fn walk(&mut self, root: PublicAst, context: ContextState) -> Result<(), CompileError> { todo!() }
}
```

The borrowed domain view decodes only the current node's discriminant and relevant typed fields. It is not a getter facade module and never produces a recursively lowered native subtree. Presence is retained for body, Source and Context fields; explicit empty base, false reference and sourceBase override inheritance. Context is transparent to semantic parent/nesting. Unknown context/metadata fields remain in storage. Malformed-field policy belongs to this owner, not the host adapter.

Each visit takes a shallow immutable lease of the current object outside the mutable overlay, then borrows domain fields from that lease. Borrowed fields are released before an overlay mutation; only charged Text operands and retained handles cross that boundary. Multi-step visitors re-lease after writes. After the visitor, resolve children again from the current owner and advance against its current array. Never clone a traversal list. Undefined declarations remain slots until the proper downstream operation discards them.

## Registration behavior

One preorder dispatcher owns authored order and stops on the first semantic failure. It checks utilities, utility, source, legacy variant, custom variant, media and theme in fixed order. Legacy rename is an actual assignment followed by custom-variant handling in the same visit. Custom variants first validate nesting, then remove their occurrence before name/body/selector validation; their removed bodies are not descended into. Utilities validate nesting/body/name, retain their live attached body and defer registry work. Nested initial directives can change that body before deferred registration rereads it.

Replacement handling precedes Stop handling, exactly as fixed `src/ast.ts:198–208`. In media's source subwalk, `replaceWith` followed by Stop skips the replacement but continues subsequent siblings; it is **not** first-match search (`src/index.ts:454–457`). Theme/prefix propagation skips encountered theme bodies. Media tokens execute in order; flattening revisits replacements. Existing `append_source`'s first-return implementation cannot be reused.

Theme performs its dedicated Context-transparent body walk, retaining keyframes without descending into them. Its first replacement publishes an empty rule; later themes are removed only after validation. Compatibility remains between this initial walk and deferred custom registration/functional normalization. Normalization updates current shared declaration fields even for equal text, then final theme nodes are assigned. Deferred definitions, applications, functions and apply keep their ordering and failure prefixes; they are not reconstructed from final CSS.

## Module map and migration

- `compiler/registration.rs`: one deep owner of traversal, directive state, deferred registration and typed local views. Existing family modules supply semantic algorithms, not whole-tree extractors.
- `compiler/origin.rs`: reuse its provenance/current-overlay concept, extending structural mutations, captured versions and clone rules. Registration grammar must not depend on `InputMutation`. Existing post-phase public journals still require adapter migration; physically separating that dependency is proposed, not proven, and does not authorize deleting those journals.
- Existing imports `Bundle/Program/Machine`: retain occurrence scheduling, synthetic root, compatibility boundary and failure continuation. Parsed import fragments adapt once into the same store; no separate public media grammar.
- Public/native adapters: preserve codecs, Source graphs and output projection; only public crossing translates effects and executes host operations. Existing registration scans and post-hoc theme/media prefix builders are removed.

Later typed registry/render objects are derived products, not competing current setup trees. Lower only at an explicit completed boundary. If normalization/functions/apply require repeated value-to-native-to-value round trips, port their field operations to this owner or abandon this shape; do not add per-directive synchronization hooks.

## Accounting and obligations

One setup ledger counts actual capacities of values, Text, overlay/reverse-map entries, cursor stacks, definitions, Source owners, captured versions, effects and temporary parsing/projection. Deduplicate allocations by storage, never logical identity; count distinct COW versions. Transfer parsing includes simultaneous parsed and adapted graphs. Admission reserves the complete finite bundle, all potential effects, host handles and detached continuation before start. Existing conservative reserve is not allocator-peak evidence.

Bundle rejection does not cancel started tasks. Captured contexts survive setter failure; host exception identity and earlier-host-error precedence remain intact. Source tuples, extra fields, absent src, original offsets and aliases survive directly. Full M4 helper/API, loader/Proxy feedback, UTF-16 paths, unified peak/performance and browser obligations remain pending. Dynamic host feedback must update this same owner; a second host grammar is disallowed.

## Tradeoff, alternative and falsifiable scrap

We accept wider handler migration and overlay lookups to retain the existing lossless graph without a new topology. A typed canonical arena could hide field decoding, but would require another identity/storage conversion boundary; this exploration does not select between them. Synthesis is intentionally unfilled.

First prototype structural splice/reassignment on shared arrays, before moving any directive. Scrap if later aliases see stale contents, captured assignment versions drift, fresh clones share anchors, or private/public topology needs parallel bookkeeping. Then challenge legacy removal, utility-live-body/theme, media replacement-before-Stop and detached import failure. Scrap on repeated lowering, repeated shallow view modules, or inability to charge/release all reachable versions. These are rejection criteria, not claimed passes.
