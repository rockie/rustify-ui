# Candidate A: owned import fragments and a bounded reaction queue

## Problem

The full [WASM Tailwind plan](../../../plan/WASM-TAILWIND.md) remains the contract. Replace the depth-first import expansion with a small Rust model of the actual upstream async operations, preserving native parsing, caller handles, admission before writes, and the synchronous native and WASM consumers. The [Ground](ground.md) and [horizon oracle](../utf16-families-import-failures-horizon-oracle.log) show why recording every successful sibling before reporting an error is wrong: chain1 fills its outer context before the caller catches a root missing asset; chain2 fills afterward. Import errors need a retained owner after rejection, rather than a longer synchronous failure journal.

## Usage (caller's view)

The following three real consumers constrain the design before its types are chosen.

1. The current actual-WASM failure fixture calls `new facade.AstCompiler(input, JSON.stringify({limits, options, assets}))`, catches synchronously, and compares the live input graph with the upstream graph at `await compileAst` rejection: [fixture](../../../../tests/utility/compiler/run-public-ast-import-failure-wasm.mjs), lines 69–74. This constructor remains usable. Its synchronous catch is an emulation of that boundary, not an actual async continuation.

   ```js
   // Existing supported virtual-bundle call.
   try {
     const compiler = new facade.AstCompiler(input, JSON.stringify(config))
     const result = compiler.build(['text-red-500'])
   } catch (error) {
     // Exactly the effects admitted through the modeled caller boundary.
     // Already started import tasks retain input handles and can finish later.
   }
   // Post-rejection jobs use real microtasks; there is no user drain() call.
   ```

2. Existing native callers use `compile_ast(...) -> Result<AstCompiler, CompileError>` and immediately build, including the repeated-import source-identity test: [native call](../../../../crates/rustify-style/src/compiler/public_api/compile_ast.rs), line 2064. Their signature and successful build behavior stay unchanged; a native semantic failure returns the selected error and drops private detached work because these callers expose no live host graph.

   ```rust
   let mut compiler = core::compile_ast(input, options, assets, limits)?;
   let output = compiler.build(&candidates)?;
   // No executor, promise IDs, or import phases enter the consumer's API.
   ```

3. Upstream exposes an actual Promise from `compileAst`: [upstream call boundary](../../../../ref/tailwindcss/packages/tailwindcss/src/index.ts), line 709. Complete public async behavior therefore also requires a Rust-backed async WASM entry point, alongside the constructor. Merely adding delayed mutations to the constructor does not finish this requirement.

   ```js
   // Proposed async companion; the exported Promise is compileAst's boundary.
   try {
     const compiler = await facade.compileAst(input, JSON.stringify(config))
     const result = compiler.build(['text-red-500'])
   } catch (error) {
     // Native Promise rejection; started siblings continue after this catch.
   }
   ```

`compileAst` takes the current finite virtual bundle. Arbitrary externally delayed JavaScript loaders are not represented by that bundle API. The plan requires virtual resources and forbids compiler fetch/disk access; this proposal does not claim an arbitrary callback loader is already implemented. An eventual loader callback adapter must feed settlement into the same Rust scheduler, with an explicit admission contract for unknown future resources.

## Shape

### Module ownership

| Module | Knowledge it owns | Change |
| --- | --- | --- |
| `compiler/imports.rs` | Parameters, path/base normalization, recursion, wrappers, owned parse fragments, import task/scope reaction graph | Replace the recursive fail-fast expansion with the bounded queue and fragment plan; keep existing helpers |
| `compiler/public_api/compile_ast.rs` | `Origins`, original `PublicAst` handles, fresh parse public views, caller mutation journal, compiler setup and failure outcomes | Make the graph own `Origins` instead of borrowing it, add preparation/execution ownership, retain existing Result adapters |
| `compiler/mod.rs` | Compiler construction from a fully expanded native AST | Split import expansion from the already-existing remainder internally so prepared native fragments are consumed directly without another import pass |
| `style-wasm/src/public_api.rs` and its `graph.rs` | Graph crossing, quotas including host handles, exact host exceptions, JavaScript Promise publication, callback lifetime | Apply one acknowledged effect at a time, retain failed-session graphs, post Rust reactions to real microtasks |

These are knowledge boundaries, not separate load/validate/transform modules. The core session concentrates scheduling and ownership; callers do not coordinate stages (`interface-depth`, `boundary-discipline`). The three-file semantic path is imports → AST setup → WASM crossing; `mod.rs` only consumes the resulting native tree.

### Types, before algorithms

All sketches are signatures and pseudocode, not compilable production changes.

```rust
// imports.rs: private domain IDs; never serialized or exposed to JS consumers.
struct FragmentId(usize);
struct ScopeId(usize);
struct TaskId(usize);
struct PromiseId(usize);
struct JobId(usize);
struct EffectId(usize);

struct ParsedFragment {
    // The ONE native parse for this import occurrence, not a resource cache.
    seed: Vec<AstNode>,
    public: PublicAst,
    // NodeOrigin and Source Arc owners are retained through seed/public/Origins.
    // Nested import slots are addressed by native node paths within this fragment.
    occurrences: Vec<ImportOccurrence>,
}

struct ImportOccurrence {
    site: ImportSite,
    params: Import,
    resource: ResolvedResource,
    loaded: Result<FragmentId, CompileError>,
    // This occurrence has its own context object and wrappers.
    // Repeated references to one asset never share parsed Source/node identities.
    effects: ImportEffects,
}

struct ImportSite {
    parent: PublicAst,       // Original array owner; never a reconstructed handle.
    index: usize,
    original: AstNode,       // Its original src supplies every import wrapper src.
    native_path: NodePath,   // Internal stable location, not a public array copy.
}

enum PromiseState {
    Pending { reactions: Vec<Reaction> },
    Settled(Settlement),
}
enum Settlement { Fulfilled, Rejected(FaultId) }
enum Reaction {
    ResumeLoad(TaskId),
    ResumeNested(TaskId),
    AllElement { group: ScopeId, ordinal: usize },
    ResumeScope(ScopeId),
    ResumeParseCss,
    ResumeCompileAst,
    ObserveCaller, // Only the synchronous adapter's observer.
    ResumeCompatibility, // Success pipeline's actual await, not a delay constant.
}

struct ImportScope {
    fragment: FragmentId,
    returned: PromiseId,
    group: Option<AllGroup>, // Absent if the synchronous walk throws.
    walk: WalkState,
}
struct AllGroup {
    returned: PromiseId,
    pending: usize,
    seen: Vec<bool>,        // One reaction per element; duplicate delivery is inert.
}
enum TaskState {
    AwaitingLoad,
    AwaitingNested { fragment: FragmentId, scope: ScopeId },
    AwaitingHost { effect: EffectId },
    Settled,
}
struct ImportTask { occurrence: usize, returned: PromiseId, state: TaskState }

struct ReactionMachine {
    promises: Vec<PromiseState>,
    scopes: Vec<ImportScope>,
    tasks: Vec<ImportTask>,
    ready: VecDeque<(JobId, Reaction)>,
    // Queue capacity and lifetime-owner cost are included in admission.
    compile_promise: PromiseId,
    faults: FaultStore,
}

// compile_ast.rs: the public carrier remains opaque to ordinary consumers.
pub struct PreparedAstSetup {
    fragments: FragmentArena,
    graph: OwnedImportGraph, // Owns Origins, views, journals, and original root.
    machine_seed: MachineSeed,
    completion: PreparedCompletion,
    reserve: SetupReserve,
}
enum PreparedCompletion {
    Ready(Box<AstCompiler>),
    SemanticFailure(CompileError),
}
struct SetupReserve {
    native_peak_bytes: usize,
    crossing_values: RetainedValueSet,
    queued_peak_bytes: usize,
    // One calculation includes seed/active structures, journal and detached tail.
}
```

`FaultStore` stores native errors by ownership and host errors as opaque tokens held by the WASM session. It does not stringify thrown objects. The first transition settling a promise wins; an aggregate settled by one rejection still receives later element reactions without changing its error. A global first-error field would be wrong because an unobserved task can reject before a later synchronous parameter error rejects its enclosing scope.

`FragmentArena` is indexed by **import occurrence**, not canonical resource path. Canonical paths index only immutable bundle lookup. Preparation records actual parse fragments, their fresh public views and stable owners; execution activates those fragments rather than calling `CSS.parse` or re-running a second native parser. Fragment snapshots can be cloned as native nodes for a working tree, retaining the same per-occurrence Source and Origin identities; simultaneous seed/working ownership is explicitly charged.

### Signatures and host contract

```rust
pub fn prepare_ast_setup(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<PreparedAstSetup, CompileError> {
    // TODO: validate limits/input/assets before allocations; run the native
    // reaction graph to quiescence with a recording host; parse each occurrence
    // exactly once; keep actual fragments and all effects including detached work.
    // TODO: run remaining compiler setup only if root imports succeed; preserve
    // the existing semantic function-failure journal and success operations.
    // TODO: reserve the combined preparation, execution and crossing peak.
    unimplemented!("NotImplemented")
}

// Bridge capability, not the consumer API. No JS wire representations in core.
pub trait AstSetupHost: 'static {
    type Fault: 'static;
    fn apply(&self, effect: &InputMutation) -> Result<(), Self::Fault>;
    fn post(&self, reaction: Box<dyn FnOnce()>);
    fn publish(&self, result: Result<AstCompiler, SetupFault<Self::Fault>>);
}

impl PreparedAstSetup {
    pub fn start<H: AstSetupHost>(self, host: H) {
        // TODO: consume admission token, synchronous initial walk, then post each
        // queued reaction individually; running callbacks retain the session.
        // TODO: publish success/error only when compileAst's promise settles.
        unimplemented!("NotImplemented")
    }

    pub fn run_to_caller(self) -> Result<AstSetupOutcome, CompileError> {
        // TODO: same machine with FIFO recording host and an explicit observer
        // reaction attached to compile_promise; stop when observer runs.
        // Return prefix plus opaque owned continuation, not a fully drained tail.
        unimplemented!("NotImplemented")
    }

    pub fn run_to_caller_with_host<H: AstSetupHost>(self, host: H)
        -> Result<AstCompiler, SetupFault<H::Fault>> {
        // TODO: synchronous constructor adapter; apply acknowledged real effects
        // until its synthetic observer runs, then post every remaining job.
        // Host faults participate in Promise propagation and remain exact tokens.
        unimplemented!("NotImplemented")
    }
}

pub struct AstSetupFailure {
    pub error: CompileError,
    pub input_mutations: Vec<InputMutation>, // Prefix at ObserveCaller only.
    pub cache_bytes: usize,                  // Includes retained detached work.
    continuation: Option<DetachedSetup>,     // Owned graph, fragments, ready jobs.
}

pub fn compile_ast_setup(/* current arguments */)
    -> Result<AstSetupOutcome, CompileError> {
    // TODO: prepare once and use run_to_caller; preserve Result admission failures.
    unimplemented!("NotImplemented")
}

pub fn compile_ast(/* current arguments */) -> Result<AstCompiler, CompileError> {
    // TODO: unchanged Complete/Failed adapter; no public host graph to retain.
    unimplemented!("NotImplemented")
}

// WASM shell, proposed new entry. Uses js_sys::Promise as the public carrier.
pub fn compile_ast_async(input: JsValue, config: &str) -> js_sys::Promise {
    // TODO: read graph once, prepare native task once, preflight entire reserve
    // including host graph and ALL retained suffix values, then start a WASM host.
    // TODO: admission failure rejects without input writes; no half compiler.
    unimplemented!("NotImplemented")
}
```

`post` schedules each specific Rust continuation at its creation time. It must not post one pump that drains an arbitrary batch or creates the next callback only after the previous callback: that changes interleaving with external microtasks. JS glue, if needed, is only `queueMicrotask(callback)`. Rust chooses, owns and enqueues every reaction. The exported Promise is settled by the Rust `ResumeCompileAst` operation; the real JavaScript caller await supplies the last reaction. Do **not** use `future_to_promise` and also simulate its wrapper hops without checking the resulting extra scheduling boundary.

`start` is the deep bridge operation: it owns setup, effect acknowledgement, async lifetime and publication. The ordinary native API and WASM constructor expose no scheduling stages. `run_to_caller` is a native recording adapter, not a second import implementation.

### Exact reaction rules

1. A scope runs its initial walk synchronously. For a valid local import, create the task and empty context, start the virtual loader IIFE, register its await, then apply the original-parent index replacement. Missing assets reject the loader Promise; they do not throw out of the walk. Parameter errors throw from the walk before launching that occurrence. The recursion check is inside the task launch, just as [upstream](../../../../ref/tailwindcss/packages/tailwindcss/src/at-import.ts), line 44, so its rejected task still has its replacement operation.
2. After a successful walk, create `Promise.all` by registering element reactions in input order, and register the scope await on its group Promise. If the walk throws, reject the scope immediately and **do not create its aggregate**. Already launched tasks remain alive.
3. Registering an await/element reaction on a settled Promise enqueues a new FIFO job. Settling a pending Promise enqueues its existing reactions in registration order. Both mechanisms charge queue storage and produce one callback per job; no depth-based timestamps exist.
4. `ResumeLoad` either rejects the task with its loader error or activates its owned fresh parse fragment and starts the nested scope synchronously. Parser failure rejects this task; independent siblings continue. Register `ResumeNested` on that nested scope's Promise, including the no-import case where that Promise has already fulfilled.
5. `ResumeNested` builds the wrappers from the owned native fragment, preserving original import `src`, and applies the context `nodes` setter. On acknowledged success, update the semantic view and settle the task. On host failure, reject this task with that same host object; do not claim it completed.
6. An `AllElement` fulfillment decrements its own group's pending count once; zero fulfills it. Rejection settles that aggregate once, without cancelling or removing other tasks. `ResumeScope` forwards the group outcome to the scope's returned Promise.
7. `ResumeParseCss` and `ResumeCompileAst` represent the actual nested awaits in [index.ts](../../../../ref/tailwindcss/packages/tailwindcss/src/index.ts), lines 149 and 709. Failed imports skip subsequent compiler setup. The success path must additionally represent its compatibility-hook await at line 600; it is not justified to match success timing using only the failure path.
8. Native `run_to_caller` attaches `ObserveCaller` to the compile Promise and stops when that reaction executes. A Promise can have rejected earlier while other jobs already queued ahead of the observer still legitimately run. WASM async entry attaches no synthetic observer: real caller reactions determine the catch boundary.

These rules reproduce the horizon without storing oracle epoch values in production:

| Scenario | Derivation | Boundary effect |
| --- | --- | --- |
| root leaf + missing | Leaf load resumes, awaits an already fulfilled empty nested scope, then fills. Missing propagates through all-element → root scope → parseCss → compileAst → observer | Leaf fill before catch |
| root chain1 + missing | Its nested leaf adds child load resume, child empty-scope await, child all-element, child scope await before the parent's fill continuation | Outer fill before observer when both land in the same marker epoch, in FIFO order |
| root chain2 + missing | One additional real nested group/await cycle; missing still takes the root propagation path | Outer fill after catch; continuation retains the original context |
| earlier leaf + later invalid params | Enclosing scope rejects synchronously without waiting on any group; earlier leaf's two await resumes still run | Leaf can fill before the shorter rejection observer path |

The oracle's epochs 6/10/14 are validation observations, not delay constants or formulas.

The [corrected parser oracle](../utf16-families-import-failures-horizon-parser-oracle.log), lines 20–21, also rejects with `Missing closing } at .broken`: the direct parser failure catches at epoch 6, the nested parser failure at epoch 10, while their independent root leaf fills at epoch 2. These are genuine parser failures; the earlier log's parser fixtures completed successfully and cannot establish parser-error ordering.

### Admission and host failures

Preparation drives all already-started tasks to quiescence under successful host writes, including tasks that run after root rejection, and retains the actual parsed fragments, planned mutation values and reaction structure. This discovers and charges the entire finite suffix. It does **not** publish mutations or expose a compiler. A limit violation anywhere in this preparation refuses the whole setup before the first host write, even if that violation belongs to a later detached task. This is the existing admission contract, not upstream's unlimited-input behavior.

The dry run is for ownership/reservation, not a replay authority. Actual execution repeats **state transitions**, reusing the prepared per-occurrence fragments and effects; it does not parse again. A setter failure can stop a scope before aggregate creation or reject an async task earlier than the dry-run semantic outcome. Therefore execution must rerun the reaction machine with host acknowledgements rather than replay an unconditional precomputed suffix. Host exceptions only prune later native work from the read snapshot; they do not cause new virtual resources to appear. This assumption follows the current boundary's snapshot behavior and must remain explicit.

Reserve includes input/origin owners, parsed seeds, working native clones, final prepared compiler if successful, wrappers/public views, complete journals, strong native-address cache owners, all host graph handles and pending microtask callback captures. Use actual capacities and identity-aware graph accounting; charge overlap, not just final retained bytes. `queued_bytes` bounds reactions as well as work descriptions. Checking a per-operation journal immediately before replay is insufficient once earlier operations have already mutated the host.

The existing [graph writer](../../../../crates/rustify-style-wasm/src/public_api/graph.rs), line 272, uses exact `Reflect.set`/`splice` targets. Adapt it to an acknowledged one-effect crossing with persistent created-handle ownership. Created handles must be retained even if a subsequent effect throws. No writer reparses CSS or reconstructs the original parent array. A failed setup's session retains these handles until every launched job settles, then releases them without publishing a compiler. References that the caller kept from published contexts naturally remain live.

For synchronous constructor failure, the original semantic outcome alone is not enough: invoke the admitted machine with the real host through its synthetic observer so a thrown index setter stops that walk and a thrown context setter rejects that task. Then throw the selected exact host/semantic error and schedule the retained continuation automatically. Returning the dry-run error after replaying its whole prefix would be incorrect.

Before calling host setters, release any `RefCell` borrow of session state; hold an owned in-flight effect token, then reborrow to acknowledge it. A nested constructor triggered by a setter has its own session. A job/effect token transitions only once, so duplicate callbacks cannot apply a setter twice (`encode-lessons-in-structure`, `make-operations-idempotent`). Reentrancy must not silently turn into a borrow panic.

### Design-red-flags screen

- **Shallow module:** passes. Ordinary consumers have one constructor or one Promise call; the opaque session hides ownership, quota, error races and detached completion. Three host capabilities are necessary boundary operations, not consumer phase controls.
- **Information leakage:** passes with an explicit condition: Promise IDs/reactions/fragment paths stay private. `InputMutation` remains the existing domain crossing representation. No JS Promise or `JsValue` enters core types.
- **Temporal decomposition:** passes. Import rules and reaction scheduling live with the actual import fragments; `Origins` remains with public AST identity rather than a separate staging service.
- **Pass-through methods:** the Result adapter is justified compatibility policy; it selects/discards the failed carrier for consumers without a host. The async bridge adds real host lifetime/publication behavior rather than simply renaming core setup.

## Synthesis decision

Reserved for the orchestrator's independent-candidate comparison. This candidate has not inspected any other candidate.

## Tradeoffs accepted

- We accept a small explicit Promise subset in Rust in exchange for matching reaction order and detached work across native and WASM without JS scheduling logic.
- We accept an admitted finite dry run and active execution metadata in exchange for limits checked before writes and host-failure-aware control flow; there is only one native parse per occurrence and all overlapping storage is charged.
- We accept a synchronous adapter's modeled observer boundary in exchange for preserving current consumers. Actual Promise timing remains the async entry point's requirement; the synchronous constructor cannot make its catch execute as an await reaction.
- We accept retaining a failed session through its remaining microtasks in exchange for faithful post-rejection mutations and stable native/host identities.

## Alternatives considered

- **Move `begin` before asset lookup in the current DFS.** Tiny interface, but hides no real async lifetime and gives callers incorrect sibling/error order. It cannot produce both the chain1 prefix and chain2 suffix.
- **Eager success-journal replay with depth-based completion epochs.** It hides scheduling superficially but leaks depth assumptions into error handling and host behavior. Nested parameter errors, independent siblings and setter failure invalidate the presumed fixed schedule.
- **Delegate Promise orchestration to a JS loader wrapper.** It could use native JS timing with a small facade, but exposes parse/task ownership across languages and duplicates scheduler policy outside Rust. It violates this task's Rust scheduling constraint; queueMicrotask is enough as a boundary.

## Open questions and risks

- Can the fixed oracle demonstrate exact host-exception behavior at original index replacement, inner unpublished contexts and outer `nodes` assignment, including a task launched just before replacement throws?
- Does the complete public conformance ledger include browser `unhandledrejection` for tasks launched before a walk throws and never attached to `Promise.all`? The virtual machine cannot claim those native browser events without a Rust-controlled host rejection-reporting carrier and an oracle test.
- Can the full unified peak accounting include the preparation/active-clone/host overlap without unnecessarily rejecting current admitted fixtures? This needs concrete limit boundaries, not only a conservative prose reserve.
- Can the async wrapper settle the exported Promise with exactly the observed await hops, including success compatibility hooks, without extra wasm-bindgen Future-adapter jobs?
- Are custom loader callbacks in a separate explicit public compatibility requirement beyond the plan's finite virtual-resource map? If so, what bounded resource-admission API satisfies unknown future contents without writes preceding final admission? Keep this as unfinished full-goal work rather than claiming virtual assets implement arbitrary delayed loaders.

## Next implementation step

Implement the native bounded Promise/reaction machine with owned occurrence fragments and compare full pre-catch/post-catch traces for leaf, chain1, chain2, nested errors and synchronous parameter errors before changing the WASM crossing.
