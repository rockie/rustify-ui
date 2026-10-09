# Native custom utility grounding

Scope: read-only architectural exploration of native custom registration, normalization, function/apply expansion, ownership, clone semantics, and cache accounting. No builds, tests, external services, or repository edits were performed. Paths below are relative to `/Users/rockie/Documents/gh-rockie/rustify-ui`. Findings describe source as read; carrier feasibility observations do not select a candidate design.

## Components Found

- `AstNode` (`crates/rustify-style/src/compiler/ast.rs:19–67`): ordered native syntax enum containing typed CSS fields, optional typed locations on rule/at-rule/declaration/comment, and an `Arc<()>` identity only on declarations. It has no arbitrary caller node fields or explicit full-node provenance. Context/at-root do not carry typed locations (`152–190`).
- `SourceLocation` (`compiler/public_api/source_map.rs:13–22`): typed `Arc<Source>` and UTF-16 spans plus optional `Arc<PublicValue>` holding canonical caller metadata, including full Source objects and JavaScript number offsets. Serializer prefers that public tuple (`66–72`). `Source` itself has only file/code (`7–11`).
- `PublicValue` / `PublicObject` (`compiler/public_api/design_system.rs:25–107`): objects retain insertion order, Text keys and a semantic identity separate from object storage; arrays and objects use Arc. `clone_fresh` recursively copies graphs with memoization by array/object storage pointers and creates fresh object identities; shared references within one clone remain shared.
- `Definition` / `CustomUtilities` (`compiler/custom_utilities.rs:21–26,129–142`): definitions own native node bodies; root indexes point into their ordered vector; completion metadata and warnings contribute owned bytes. It exposes independent outputs per matching definition.
- `clone_fresh` (`compiler/ast.rs:308–363`): native deep clone that memoizes original declaration identities, typed Source pointers and public graph pointers. New declarations and sources differ from their originals, but aliases inside one cloned group are preserved. Normal derived Clone shares declaration and Source handles.
- `CompiledCandidate` (`compiler/utilities.rs:13–22`): raw node tree, GPU program, globals, variable dependencies, and sort metadata. Raw tree is intentionally retained before render pruning (`631–633`).
- `Origins` (`compiler/public_api/compile_ast.rs:66–79`): side tables holding complete caller declarations by native declaration identity, authored wrappers by structural keys, generated declarations by native declaration identity, raw candidate values by CompiledCandidate pointer, and source/apply ownership metadata.
- Apply arena and `Replacement` (`compiler/apply.rs:18–40`): stable arena indexes preserve native parent identity during replacement; `Replacement` retains the owner public source tuple and raw expanded native nodes for public input mutation replay.
- `Compiler` setup/staging (`compiler/mod.rs:209–349,455–735`): owns the transformed input, definitions/theme, prepared candidate cache and aggregate resource accounting. Candidates are compiled into Arc<CompiledCandidate> and published only at commit.

## Flow

1. Public `compile_ast` checks input budget, converts the caller graph, and calls `Compiler::new_with_ast` (`compiler/public_api/compile_ast.rs:857–879`). `Origins::convert` saves complete declaration values under newly allocated declaration IDs (`608–619`) and saves full rule/at-rule/comment values in authored tables (`651–656`). Native rules/comments have no typed src; declarations preserve supplied src via `caller_source`, and only theme/apply/tailwind at-rules get a typed caller location (`541–626`). `caller_source` deliberately has empty typed file/code and zero spans; the full caller tuple resides in `.public` (`128–137`).

2. Native setup expands imports and extracts source metadata (`compiler/mod.rs:215–225`), extracts custom variants and marked utilities, then extracts theme (`243–249`). Custom extraction moves utility children into `Definition.nodes`, removes the actual @utility from native input, and leaves an internal at-root/comment marker (`compiler/custom_utilities.rs:189–281`). Duplicate roots keep multiple ordered definition indexes (`259–270`). Context is transparent to the top-level restriction; real nested rules are not (`284–292`). The definition wrapper's arbitrary caller fields are not transferred into generated class rules.

3. Functional definitions alone run `normalize` before completion extraction (`custom_utilities.rs:251–258`). Normalization mutates only declaration value Text and recursively visits children (`696–706`); value/modifier arguments normalize escaped stars and whitespace, append namespace wildcard suffixes, and generate unsupported datatype warnings (`607–691`). Static definition bodies bypass this pass.

4. Native setup prepares theme functions, authored variants and custom functions before @apply (`compiler/mod.rs:281–296`). `CustomUtilities::prepare_functions` mutates each definition and recounts its owned bytes (`custom_utilities.rs:370–390`). Generic `functions::substitute` edits declaration values and selected at-rule params in place and recurses (`compiler/functions.rs:351–375`); it does not refresh declaration identity. The local unit test asserts preserved declaration Arc identity after substitution (`383–407`).

5. Apply reconstructs definition markers in an arena using plain native clones of definition bodies (`compiler/apply.rs:43–84`), builds dependency graph and topologically orders parents (`132–295`), expands parent apply nodes, writes expanded definitions back and recounts custom bytes (`521–565`). Restoration uses plain Clone (`87–98`). Replacements are recursively revisited, including @apply generated by custom variant slots (`459–518`).

6. `apply::expand` compiles and sorts the applied candidates, removes the generated outer class rule, and fresh-clones children (`392–449`). It then replaces every descendant's source with a candidate span based on the @apply location (`420–451`): source start adds `7 + UTF-16 candidate offset`, end counts candidate units, and public numeric offsets are updated. These applied clones must retain arbitrary node metadata while their src obeys this explicit override. `replace_sources` affects src only, not dst (`compiler/ast.rs:299–305`).

7. On build, each uncached candidate passes `candidate::parse_with_utilities` and `utilities::compile_all` (`compiler/mod.rs:494–529`). Native custom matching calls `CustomUtilities::compile`, which fresh-clones the definition once per matching definition (`custom_utilities.rs:408–470`). Functional value/modifier substitution mutates retained node values and drops unresolved declarations (`476–495,809–851`); ratio pruning uses traversal positions (`855–868`). Static candidates use the same fresh clone, with no functional substitution.

8. `compile_custom` inspects sort declarations, constructs GPU declarations, marks important if requested, and wraps nodes in a fresh generated class Rule (`compiler/utilities.rs:637–753`). `finalize` applies variants and generic function substitution and retains raw children (`595–634`). Both built-in and custom matching outputs may coexist (`756–799`).

9. Cache outputs become `Arc<CompiledCandidate>`. Rendering plain-clones each cached native node and globals (`compiler/mod.rs:644–647`), substitutes @tailwind markers, and recurses through plain native clones (`907–959`), then hoists at-roots and optimizes (`1110–1125`). Optimization removes --tw-sort, flattens &/nonreference contexts, drops empty branches, and removes earlier duplicate declarations by declaration identity (`compiler/ast.rs:375–452`). The surviving declaration retains its raw tree identity. Fresh-cloning during rendering would break that observable relationship.

10. Public build materializes raw candidate nodes first and stages generated declarations; optimized output is materialized afterward with the same declaration IDs (`compiler/public_api/compile_ast.rs:1101–1164`). Raw CompiledCandidate pointers stabilize repeated raw values (`1117–1135`). Output resolves caller declarations only by old native ID (`713–717`); generated IDs fall back to generated/staged values, then to standard declaration fields (`784–795`). Source and destination resolution overrides serialized metadata afterward (`819–842`). Generated declaration public values are staged by their new ID (`843–848`) and committed after resource and host checks (`1213–1261`). No-op build returns the previous optimized array when accepted output count and variable count do not change (`1150–1157`); Features.None returns the original input array (`1069–1071`).

## Boundaries

### Exact failure boundary

The current persisted RED log reports `static: build ["card"]` and missing `extra: "old child"` (`docs/validation/wasm-tailwind/utf16-families-custom-input-first-red.log:5–26`). The fixture attaches extra fields to definition bodies and snapshots all object fields and reference aliases (`tests/utility/compiler/run-public-ast-custom-input-wasm.mjs:27–57,75–87`). This was not rerun here.

Mechanism: full caller body declaration lives in `Origins.declarations[old_id]`; custom native cloning replaces `old_id` with `new_id`, but no full-node provenance follows that clone. `Origins.output_inner` cannot resolve `new_id` in caller declarations and constructs a plain declaration. Source survives only when it was represented separately by `SourceLocation.public`. Therefore teaching output to copy a single `extra` field would not solve the general ownership problem.

Wrapper matching is also weaker than full origin transport: `OriginKey` uses selector, name/params, or comment value (`compile_ast.rs:270–284`), and authored matching relies on surviving declaration IDs/CSS shape plus one-use bookkeeping (`713–742`). Custom cloning refreshes descendants' IDs, and functional/generic function transformations can change CSS shape. Rules and at-rules are rebuilt from standard fields even when an authored origin is found (`748–783`); only comments clone the whole matched value (`798–800`). Arbitrary wrapper metadata cannot reliably be recovered through those tables alone.

### Carrier feasibility observations

- The enum has six variants, but typed source/destination slots exist on four. Context/AtRoot have no slots; source()/destination() always return None and their setters ignore writes (`ast.rs:21–67,152–190`). Consequently SourceLocation by itself has no place to store full provenance for Context/AtRoot.
- True source absence is observable. `fill_sources` assigns inherited source only when source() is None (`ast.rs:288–296`), and public output emits any resolvable location.public as src (`compile_ast.rs:831–835`). Introducing a dummy Some(SourceLocation) merely to carry origin would change the current source-absence signal. `SourceLocation::undefined()` is already an explicit value serialized as the undefined sentinel (`source_map.rs:24–35,66–72`), which is distinct from no location slot.
- An optional node-origin handle independent of SourceLocation can be represented uniformly across the six native variants without requiring a src value. It would add enum fields: constructing native literals and exhaustive field patterns would require adjustments. A source search under `crates/rustify-style/src/compiler` reports 320 AstNode variant-reference lines, including constructors, patterns and tests; this is not a constructor count. Ten exact AtRoot `{ nodes }` patterns occur in rustify-style source: ast.rs:194,206,466,513,528; sources.rs:411; custom_variants.rs:633; custom_utilities.rs:512; apply.rs:47; compile_ast.rs:801 (including two Self patterns). These patterns omit `..` and would no longer compile when AtRoot gains a field. Most Context patterns already use `..`.
- Ordinary derived Clone shares declaration/Source handles; fresh custom/apply boundaries allocate new handles with alias memoization. Native declaration identity currently has two roles: identifying the generated declaration public object and preserving raw/optimized relationships. It cannot recover the caller origin after fresh clone, because the old origin table is keyed by old ID (`compile_ast.rs:608–619,713–717`).
- Existing Source fresh clone uses one shared PublicValue memo across all node src/dst locations (`ast.rs:308–363`). Independently cloned origin payloads could duplicate a Source reached through both node metadata and src unless they participate in the same graph clone relationship; this relationship requires upstream evidence.
- Context is converted with known base/reference properties only (`compile_ast.rs:627–644`); OriginKey excludes Context/AtRoot (`270–284`). Output rejects unoptimized Context, and constructs a standard at-root object with computed children/src (`801–817`). These paths do not preserve arbitrary wrapper fields today.

### Cache accounting

- `ast::source_owned_bytes_many` deduplicates typed Sources by pointer, outer public tuple owners by Arc pointer, and nested public graph storage/identity through one seen set (`compiler/ast.rs:217–267`). It measures source/destination only; arbitrary node fields currently cannot contribute there.
- Custom definition measurement counts Vec capacity, typed Text allocations, declaration identity fees, root map allocations, completion metadata and warnings (`custom_utilities.rs:503–595`). It does not independently measure Source/public tuple payloads. The initial compiler instead records all source locations before utilities are extracted (`compiler/mod.rs:224–242`) and later adds growth still visible in input (`297–299`). Any origin payload added to native definitions needs an explicit accounting owner, deduplication policy and recount after transformations.
- Accepted candidate fees currently use twice CSS serialization lengths, CompiledCandidate size, declaration identity costs and actual source graph fees (`compiler/mod.rs:575–613`); full arbitrary fields would be invisible to the CSS estimate. A new provenance/metadata graph must contribute here, or its allocation must be accounted in a distinct retained owner with evidence that no deep-cloned copies escape accounting.
- Public constructor retained bytes combine input graph, origin tables, mutations and banner with a shared seen set, then add those bytes to compiler retained cache accounting (`compile_ast.rs:994–1010`). Public build fees combine candidate metadata, new raw public nodes, current compiled output/mutations and generated declaration growth; failure discards prepared state before committing (`1183–1256`). Native payloads and public materialized copies may share semantic identity but occupy distinct copy-on-write storage; `json_bytes_seen` deduplicates object storage separately from identity (`design_system.rs:488–530`). Do not deduplicate only by semantic ID.
- Source-map allocation estimates measure actual typed source text and line structures (`compiler/public_api/source_map.rs:388–476`), not arbitrary node metadata. Source map indexes preserve separate equal-text source handles (`171–205`); no text-based interning should be introduced as an origin shortcut.

## Non-Obvious Things / Gotchas

1. `clone_fresh` preserves aliases within one invocation and splits them across invocations. Existing source graph unit tests assert same tuple inside one clone, different Source object identity from original and different clone, and metadata fee deduplication (`compiler/ast.rs:593–632`). Those are requirements, not incidental allocations.
2. Apply uses custom compilation's fresh clone and then another fresh clone before source stamping. Provenance must remain recoverable across both; declaration old-to-new mapping limited to one custom compile call would be insufficient.
3. Public caller Source values need not have native-compatible file strings/spans: empty native Source plus canonical public tuple is intentional. Do not normalize caller offsets to integer native spans when serializing full metadata (`compile_ast.rs:128–137`; source_map.rs:39–63).
4. Native optimizer prunes by declaration IDs after collecting duplicate triples. Distinct declaration clones with equal property/value still need distinct identities until pruning; equal CSS text is not sufficient provenance.
5. Functional normalization and function substitution mutate values in place. A transported immutable origin alone does not communicate constructor-time edits back to old caller body arrays; those effects belong to public mutation replay, not native render output.
6. Current custom marker rehydration creates a synthetic @utility wrapper with no src/dst (`apply.rs:63–71`), so wrapper provenance must not accidentally propagate from a deleted @utility into generated class rules.

## Files Read

- `AGENTS.md`, `README.md`, `PRODUCT.md`, `DESIGN.md`, `Cargo.toml`, `package.json`, `mise.toml`, `crates/rustify-style/Cargo.toml` (project context; PRODUCT/DESIGN command output was partial and no truncated facts are cited).
- `.agents/skills/how/SKILL.md`, `.agents/skills/how/references/explorer-prompt.md`.
- `docs/plan/WASM-TAILWIND.md` (located current custom input scope/status with rg; not exhaustively read).
- `crates/rustify-style/src/compiler/ast.rs` (relevant implementation and clone tests).
- `crates/rustify-style/src/compiler/sources.rs` (`405–415`, exact AtRoot pattern only).
- `crates/rustify-style/src/compiler/custom_variants.rs` (`628–639`, exact AtRoot pattern only).
- `crates/rustify-style/src/compiler/custom_utilities.rs` (registration, normalization, substitution, accounting).
- `crates/rustify-style/src/compiler/utilities.rs` (selected built-in context plus custom/finalize flow).
- `crates/rustify-style/src/compiler/apply.rs` (arena, graph, expansion, replacement, selected tests).
- `crates/rustify-style/src/compiler/functions.rs` (substitution and identity test).
- `crates/rustify-style/src/compiler/mod.rs` (setup, staging, candidate accounting, rendering).
- `crates/rustify-style/src/compiler/public_api/source_map.rs` (location, map identity, allocation budget).
- `crates/rustify-style/src/compiler/public_api/compile_ast.rs` (conversion, origins, setup/build publication, budget, selected tests).
- `crates/rustify-style/src/compiler/public_api/design_system.rs` (public graph, clone, accounting).
- `tests/utility/compiler/run-public-ast-custom-input-wasm.mjs`.
- `docs/validation/wasm-tailwind/utf16-families-custom-input-first-red.log` (failure excerpt); `utf16-families-custom-input-remaining-red.log` (search result only).

## Open Questions

- This explorer did not inspect the upstream `cloneAst` implementation, facade graph crossing, or caller mutation planning exhaustively; exact clone breadth for arbitrary nested metadata, dst and frozen objects must be resolved against the pinned upstream by the other assigned angles.
- The exact representation of node provenance remains undecided. Existing native declaration IDs alone and structural CSS keys do not retain full caller provenance after cloning; both independent node handles and location-based transport would need their identity, absence and accounting semantics specified before implementation.
- No new resource measurements were run. Additional native payload fees, duplicate Source ownership across payload/location, and saturated cache failure atomicity must be verified during implementation.
