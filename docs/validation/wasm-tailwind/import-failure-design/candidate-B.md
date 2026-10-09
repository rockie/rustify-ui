# Candidate B · 导入事件 DAG 与因果调度投影

## Problem

目标是在不缩减 [WASM-TAILWIND](../../../plan/WASM-TAILWIND.md) M4 范围的前提下，让 Rust/WASM 的 `compileAst` 导入失败行为与固定 Tailwind v4.1.13 一致。当前 `Bundle::read` 把资源查找、`begin`、解析、递归和 `complete` 串在一次 DFS 中；缺资源发生在 `begin` 之前，并立即终止后续兄弟遍历（`crates/rustify-style/src/compiler/imports.rs:350-522`）。上游则在同步 walk 中依次解析参数、启动 loader task、以空 context 替换原节点，walk 结束后才 `await Promise.all`；已经启动的 task 不随首个拒绝取消（`ref/tailwindcss/packages/tailwindcss/src/at-import.ts:25-79`）。`parseCss` 与 `compileAst` 又各有一层 await（`index.ts:149,709`）。因此问题不是把 `begin` 向前移动，而是表达 task 启动、await continuation、`Promise.all` member reaction、错误传播和拒绝后仍存活的工作。

固定 oracle 给出了结构性约束，而非可编码的 epoch 常量：leaf completion 经过 `ResumeLoaded -> ResumeChild` 在 epoch 2；单层 chain 每多一层就多出 member/all/parent continuation，outer fill 在 epoch 6；missing 经 loader、member、all、`parseCss`、`compileAst`、caller 传播，在 epoch 6 被观察；同步参数错误直接从 scope rejection 传播，在 epoch 3 被观察。chain2 outer fill 在 catch 后的 epoch 10，真实 parser failure 直接/嵌套分别在 epoch 6/10（`utf16-families-import-failures-horizon-parser-oracle.log`）。任何固定深度分支或“先完成所有成功任务”的规则都会错。

设计还必须保留：每次公共导入都只解析一次并产生 fresh native 节点与 fresh `Source`；同一 occurrence 的 `src`/`dst` tuple 独立但共享该 `Source`；所有 mutation 指向原 root 或解析所得的真实父 `PublicAst`；Facade 不二次解析；额度在首个 host write 前完成；`Reflect.set`/真实 `splice` 的原异常和成功前缀可观察；既有同步 `AstCompiler` 和 native `Result` 调用方不破坏。

## Usage (caller's view)

既有 native 调用不变。native 没有 JS host effect，内部以“所有写入成功”投影同一事件图；成功返回 compiler，首个被根调用方观察的语义错误仍是 `Err`。

```rust
let mut compiler = compile_ast(input, options, assets, limits)?;
let output = compiler.build(&["flex".into()])?;
```

既有同步 WASM 构造器也不改签名。它在返回或抛出前折叠运行事件图到 public observation 边界；若失败时还有已启动 task，构造器把余下 run 交给模块内 detached owner，后续微任务继续写这些 task 已持有的 context。额度失败发生在任何 setter、splice 或 detached job 注册之前。

```js
try {
  new AstCompiler(input, JSON.stringify({ limits, options, assets }))
} catch (error) {
  // 此处图等于上游 `await compileAst(...)` rejection 时的 prefix。
  // 更深的已启动导入仍可能在后续 microtask 填充先前发布的 context。
}
```

完整异步时序由新增的忠实入口承担，且不替换同步构造器。该入口的 Rust run 一次只前进一个 upstream reaction turn；JS 只负责 `queueMicrotask`、Promise resolve/reject 和把异步 loader settlement 回送 Rust，不承载导入语义。

```js
const compiler = await compileAst(input, { limits, options, assets })
// build 仍是同步方法；await 返回时全部成功导入已经完成。
const output = compiler.build(['flex'])
```

同步构造器能保证 catch 边界图、错误身份、effect 顺序和 catch 后 tail 的相对关系，但无法让一个同步函数拥有与 Promise 完全相同的逐 epoch 可观察时序。需要逐 microtask 对照的 API/Proxy 测试必须走异步入口；不能用同步入口通过的四个浅层 failure case 宣称完整 async API 已完成。

## Shape

### Data structures

事件图只建模 `substituteAtImports` 已知的 await 点，不实现通用 Promise runtime。循环 import 按 occurrence 展开到既有递归/`ast_depth` 限制，所以图本身保持 DAG；递归深度只参与语义限额，不参与调度优先级。

```rust
pub(crate) struct ImportProgram {
    scopes: Arena<ImportScope>,
    tasks: Arena<ImportTask>,
    events: Arena<EventNode>,
    root: ScopeId,
    reserve: ImportReserve,
}

struct ImportScope {
    parent: PublicAst,              // 原 root 或本 occurrence 的 fresh parsed array
    base: Text,
    visits: Vec<VisitId>,           // 上游 walk 的同步顺序
    members: Vec<TaskId>,           // Promise.all 注册顺序
    settlement: SettlementCell,
}

struct ImportTask {
    request: StylesheetLoad,
    original: AstNode,
    parent: PublicAst,
    index: usize,
    context: PublicValue,           // begin 与 later nodes= 共用同一身份
    load: PlannedLoad,
    parsed: Option<ParsedOccurrence>,
    settlement: SettlementCell,
}

struct ParsedOccurrence {
    native: Vec<AstNode>,
    public: PublicAst,
    source_owner: Option<Arc<Source>>,
}

enum EventKind {
    WalkVisit(VisitId),             // 同一 JS job 内继续；可启动 task 后再写 begin
    ResumeLoaded(TaskId),           // await loadStylesheet reaction
    ResumeChild(TaskId),            // await substituteAtImports reaction；成功时 complete
    ObserveMember { scope: ScopeId, task: TaskId }, // Promise.all member reaction
    ResumeAll(ScopeId),             // await Promise.all reaction
    ResumeParseCss,
    ResumeCompileAst,
    ObserveCaller,
}

struct EventNode {
    kind: EventKind,
    edges: SmallVec<[PropagationEdge; 4]>,
    registration: RegistrationOrder,
}

enum PropagationEdge {
    InlineOnSuccess(EventId),
    ReactionOnSettlement(EventId),
    FirstRejectedMember { aggregate: ScopeId },
    AllMembersFulfilled { aggregate: ScopeId },
}

struct CausalProjection {
    program: Arc<ImportProgram>,
    state: EventStates,
    ready: ReadyFrontier,            // 由已满足的 reaction edge 推导，不是事实源
    next_ticket: u64,                // activation FIFO；不含 depth/epoch 启发式
}
```

`SettlementCell` 只能从 pending 转为 fulfilled/rejected 一次。`ObserveMember` 中首个实际执行的 rejection 锁定 aggregate 错误，后到的失败只结束自己的 task；这编码了正确的首个语义错误。同步参数解析或 begin setter 失败会直接 reject 当前 scope 并停止其剩余 walk visits；已经启动的 task 没有反向 cancellation edge，仍可继续。begin 的顺序明确为“启动 loader task、注册其 `ResumeLoaded` reaction、执行 host mutation”，所以即使 begin setter 抛错，当前 task 仍存活。complete setter 失败则 reject 该 task，并沿其 member edge 传播。

`ReadyFrontier` 只是事件边的投影缓存。事件 settlement 按 `registration` 激活 reaction，并分配单调 ticket；frontier 始终取最小 ticket。这能自然得到 chain 每层增加四个 reaction、leaf sibling 先于深 chain 完成，以及 chain2 在根 rejection 后继续完成；生产代码不读取 oracle epoch。

### Signatures and pseudocode

核心入口保持深接口：调用方只要求“编译”，不会看到 walk/load/join 等阶段选项。

```rust
pub fn compile_ast(
    input: PublicAst,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<AstCompiler, CompileError> {
    let setup = prepare_ast_setup(input, options, assets, limits)?;
    setup.finish_native() // host effects 视为成功；not implemented
}

pub fn compile_ast_setup(/* existing signature */) -> Result<AstSetupOutcome, CompileError> {
    let setup = prepare_ast_setup(/* ... */)?;
    setup.finish_native_setup() // 保留现有 Complete/Failed 形状；not implemented
}

#[doc(hidden)]
pub fn prepare_ast_setup(/* same domain inputs */) -> Result<PreparedAstSetup, CompileError> {
    // TODO: 1. 转换 caller AST 并建立实际 parent/origin 对应。
    // TODO: 2. 假定 host writes 成功，展开所有可能激活的 import occurrences。
    //          每个 occurrence 在 ResumeLoaded 所属位置 parse 一次并保留结果；不缓存 public parse。
    // TODO: 3. 建 EventNode 与显式 propagation edges；模拟 native-success projection，
    //          只在 root import scope 成功时准备后续 Compiler::new_with_ast 阶段。
    // TODO: 4. 计入 DAG/state、fresh AST/Source、全部可能 mutation graph、WASM handle reserve
    //          与失败后的 detached owner；check_limit 后才返回可执行 plan。
    unimplemented!()
}
```

投影器以一个 reaction turn 为单位驱动，host adapter 在内部逐条确认写入，公共调用者不需要协调这些方法。

```rust
trait HostEffects {
    type Error;
    fn apply(&mut self, effect: &InputMutation) -> Result<(), Self::Error>;
}

impl CausalProjection {
    fn drive_turn<H: HostEffects>(&mut self, host: &mut H) -> TurnOutcome<H::Error> {
        // TODO: 执行一个 ready reaction 及其所有 inline successors。
        // TODO: 写失败时从该 EventId 走 rejection edge；不要清空 ready frontier。
        // TODO: ObserveCaller 只发布一次 outcome，但允许 projection 继续到 Quiescent。
        unimplemented!()
    }
}

enum TurnOutcome<E> {
    Yield,
    Published(Result<PreparedCompiler, SetupFailure<E>>),
    Quiescent,
}
```

`finish_native*` 使用不失败的 `NativeEffects`，将 observation 前的 event effects 扁平化为现有 `input_mutations`，从而保留 native `Result` 和 `AstSetupOutcome` 消费者。WASM 使用持久 `GraphCursor`；它在调用 `Reflect.set`/splice 前把 target 和 value materialize 到自己的 created-handle 表，写入抛错也不丢表，因此被 setter 捕获后再抛出的 context 以及 post-rejection tail 仍有稳定身份。

```rust
struct GraphCursor {
    input: Graph,
    created: Graph,
    previous: Graph,
}

impl HostEffects for GraphCursor {
    type Error = HostErrorToken; // facade 侧表保存原始 JsValue

    fn apply(&mut self, effect: &InputMutation) -> Result<(), HostErrorToken> {
        // TODO: 复用 graph.rs 的真实 Reflect.set/splice 规则；不 rollback 成功前缀。
        // TODO: 失败时保留 created handles，并把原 JsValue 存入 run 的 error table。
        unimplemented!()
    }
}
```

同步 `AstCompiler::new` 在完整 reserve 通过后循环 `drive_turn`，折叠 `Yield`，直到 `Published`。失败时把仍未 quiescent 的 projection、`GraphCursor` 和 host error table 移入 `DetachedImportRuns`，先注册下一次 microtask，再返回原错误；detached owner 在 quiescent 后自删。异步 `compileAst` 则每次 `Yield` 真正 `queueMicrotask` 后再驱动一 turn，在 `Published(Err)` 时 reject Promise，但继续持有并调度同一个 run 到 quiescent。JS glue 不解析 CSS、不选择错误、不拼 mutation。

### Native assembly and public identity

并行 task 不直接共同改写一个 native `Vec<AstNode>`。每个 `ImportTask` 独占 `ParsedOccurrence` 和完成后的 replacement；scope 仅在全部 members fulfilled 时按原 slot 顺序合并这些结果。公共可观察更新仍按事件逐条写 context。这样 task 之间没有共享 writer，读取边界才 merge，且最终 `Compiler::new_with_ast` 只收到一棵完整成功树。

public path 不使用当前 `parsed` path cache。每个 resolved occurrence 调用一次 `parser::parse_with_source`，其 typed AST 随 `ParsedOccurrence` 进入最终 assembly，同一次解析投影出 fresh `PublicAst`；Facade 不再解析内容。`parent: PublicAst`、`context: PublicValue` 和 `source_owner` 都是强 owner，mutation target 不用 path、文本或数组下标重新查找。wrapper 继续从原 import origin 取 `src`，现有 `ImportGraph::complete` 的 wrapper/source 规则归入 task completion projection，而不是移到 JS。

### Module map

| Ownership | Location | Responsibility |
| --- | --- | --- |
| 导入语义、事件与 native assembly | `crates/rustify-style/src/compiler/imports.rs`（可用私有子模块整理） | 参数、路径、fresh parse、wrapper、task/scope DAG、causal projection；同一知识不拆成 load/validate/finish 三层 |
| caller AST 投影与 compiler continuation | `crates/rustify-style/src/compiler/public_api/compile_ast.rs` | origin/parent identity、scheduled `InputMutation`、全 reserve、成功后接现有 theme/function/apply setup、legacy outcome adapter |
| host handles 与入口驱动 | `crates/rustify-style-wasm/src/public_api.rs`、`public_api/graph.rs` | `GraphCursor`、原始 `JsValue` error table、同步折叠、detached tail；异步 glue 只排 microtask/settlement |

外部调用链仍是 public API → `compile_ast.rs` → `imports.rs`，没有要求读者跨更多层追踪。事件调度与导入语义同属一个模块，避免按时间阶段做 temporal decomposition。

### Admission boundary

`ImportReserve` 在首个 host write 前覆盖“所有 host writes 成功时可能激活的分支”，包括 observation 后的 tail、fresh parsed/public graphs、Source owners、event/edge/state、mutation target/value、WASM handles 和 detached owner。额度失败不执行 setter/splice、不注册 microtask。host write 真失败时可能令一部分已预留分支永不激活，run 在 quiescent 后释放它们。

这是保住现有“Limit 不产生 caller mutation”的唯一保守边界，但会有一个需要明确裁定的后果：若一个确定会抛错的早期 setter 本可阻止很大的后续分支，按全成功分支 reserve 可能先返回 Limit。不能在设计中假装既不调用 setter又知道它会失败。

### Interface depth and red-flag screen

公共 surface 仍是 `compile_ast`、`compile_ast_setup`、`AstCompiler::new/build`，另加一个完整 async `compileAst`；DAG、turn、ticket、aggregate 和 detached owner 全部隐藏。这个小接口隐藏了同步 walk、异步传播、身份、预算和错误优先级，属于 deep module。`HostEffects` 是 core/WASM 的真实策略边界，会反馈写入结果并改变传播，不是 pass-through。wire `JsValue` 只存在 facade，core 只见 domain AST、mutation 和 opaque host error token。每个 invariant 只有一个事实源：event edges 决定传播，activation ticket 决定顺序，`SettlementCell` 决定 first error，`ParsedOccurrence` 决定 parser/Source identity，reserve 决定是否允许任何 host effect。

## Synthesis decision

待 Arena 合成；候选 B 不自行选择基线。本候选的独立结构主张是：以声明式 dependency/event DAG 保存因果，以 ready frontier 作为可丢弃的调度投影，而不是以一个 imperative job queue 兼任语义事实源。

## Tradeoffs accepted

- 我们接受为每个可能 occurrence 保留 event/edge/state 和 fresh parse 结果，以换取不靠深度常量表达 rejection cut 与 post-rejection completion。
- 我们接受同步构造器折叠 observation 前的 reaction turns，以换取既有同步调用不破坏；逐 microtask 忠实性由 async 入口提供。
- 我们接受失败后短暂存在 detached owner，以换取上游已启动 task 不被根 rejection 取消。
- 我们接受按 host-write-success 分支做保守 reserve，以换取任何 Limit 都发生在 caller write 之前。

## Alternatives considered

- **Imperative work queue**：`VecDeque<Job>` 容易实现，但“谁因谁被激活、哪个错误只取消哪个后继、为何 task 在根拒绝后仍运行”只能散落在 enqueue/clear 分支里。它隐藏的实现较少，caller adapter 反而要理解阶段与 cancellation，未选。
- **现有 DFS 加 begin 前移与成功 journal 重排**：接口最小，但 DFS 的错误返回已经删除未访问兄弟和后续 completion，无法从最终结果重建 first settlement 或 catch 后 mutation，未选。
- **通用 Promise/microtask 模拟器**：可以表达问题，却把 thenable assimilation、arbitrary callbacks 等无关状态带入 core，扩大接口和验证面。固定的七类 import continuation 已足够，未选。

## Open questions and risks

- 完整 async public API 是否确定命名为 `compileAst`，并与生成的 wasm-bindgen class 一起由哪一份受控 glue 导出？没有这个入口，不能声称 queueMicrotask/Proxy 时序完整兼容。
- 真正任意延迟、可观察副作用的 JS `loadStylesheet` 是否必须与“任何 Limit 前零 host writes”同时满足？当前纯 `assets` map 可先建图再重放；外部 loader 若观察 caller AST，这两个要求不能同时精确复刻，需要纯 loader/预声明资产上限协议或明确选择优先级。
- 全成功分支 reserve 在早期 setter 必然抛错时先报 Limit 是否可接受？若不可接受，就必须放宽全局 preflight 或引入显式 caller 授权的 reserve。
- 同步 walk 在到达 `Promise.all` 前因参数/setter 失败时，先前 task 的后续 rejection 在上游可能成为未观察 rejection；async facade 应复刻 unhandled rejection、吞掉它，还是只保证 AST effect？需要 oracle 决定，不能默认取消 task。
- detached run 中再次发生 setter/splice 错误时，除保留已发生 effect 外是否需要 host-visible reporting？根 Promise 已拒绝，需先固定上游对晚到错误的可观察方式。

## Next implementation step

先在 `imports.rs` 实现不接 WASM writer 的 `ImportProgram + CausalProjection` 骨架和四组纯 Rust schedule fixture（leaf/chain、missing sibling、同步参数错误、直接/嵌套 parser failure），用显式 event edges复现 oracle 的相对顺序、first error 与 rejection 后 tail；这些通过后再替换 `Bundle::read`，最后接 `GraphCursor` 与同步/异步 facade。
