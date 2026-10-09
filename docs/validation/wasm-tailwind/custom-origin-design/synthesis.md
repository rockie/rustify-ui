# Custom utility origin 实验结论

本记录面向继续实现 M4 的工程师。基线为 main、dirty@`ef766ce`（2026-10-05，`chore: remove hook`）；已核对远端为 `https://github.com/rockie/rustify-ui.git`。这是继续实现的结构合同；native载体、成功custom事件和局部完整字段已接入并验证，成功import图21+顺序/107阶段已接入；完整import/精确失败前缀及统一峰值未通过。

## 问题与调用方式

`AstCompiler(input, config).build(candidates)` 的调用方式继续保持。调用方保存的旧 definition、body 数组、原 child 和 metadata 引用必须仍能观察到注册、函数与 apply 的写入；生成节点需保留全部已表示的字段和克隆内别名。声明普通 Clone 与优化输出共享身份，custom/apply 深克隆产生独立身份，优化规则包装浅拷贝为新对象。

原基线的声明字段按native identity索引，fresh clone刷新身份导致投影丢字段；现由独立origin解决该断点，当前证据见[M4](../M4.md)。`SourceLocation` 无法代替所有节点的 origin：Context/AtRoot 没有 location，也必须保留真正缺少 src 的状态。依据为 `crates/rustify-style/src/compiler/ast.rs:21`、`:254` 和 `crates/rustify-style/src/compiler/public_api/compile_ast.rs:1352`；实际缺字段失败保留在父目录的 `utf16-families-custom-removal-final-custom-fields-red.log`。

## 结构与综合选择

采用候选 A 作为条件性实验基础：节点持有独立、不可变的 origin 快照句柄，克隆前先按语义 identity 解析当前 COW view，再按解析后的 storage 建立深克隆 memo。继续使用 PublicValue、既有声明 identity 与唯一 raw candidate cache。候选 B 的外置 MirrorValue/Object/Array 运行时需要另一份完整公共图和更广的 universal-ID 迁移，暂不采用。

两个候选来自独立的继承模型上下文，分别被指定探索 attached origin 与 out-of-line mirror；差异不能归因于模型。默认 Claude/Grok 不可调用，独立匿名评审使用可用的 GPT-5.6 Sol max。中性标签与冻结/审后注释 hash 见 `manifest.json`；Ground、候选和 `judgment.md` 均已落盘。两个原草图错误合并了 theme 的两个时刻，已在四份归档中更正；下面的合同优先。

先以调用方用途约束内部类型，草图并非生产实现：

```rust
// public-facing constructor/build interfaces stay as currently defined.
struct NodeOrigin { graph: Arc<OriginGraph>, object: Arc<()> }
struct OriginGraph { /* owned current object/array views, scoped reachable roots */ }
enum DefinitionCloneRoot {
    StaticBody(PublicValue),
    FunctionalDefinition(NodeOrigin),
}
fn clone_definition_checked(
    root: &DefinitionCloneRoot, limits: &Limits,
) -> Result<ClonedBody, CompileError> { /* not implemented */ }

// The facade needs effects even when semantic setup fails.
// Its cross-crate carrier must be explicitly exported behind compiler-api;
// it cannot be a private type that the separate WASM crate cannot access.
pub enum AstSetupOutcome {
    Complete(Box<AstCompiler>),
    Failed(AstSetupFailure),
}
pub struct AstSetupFailure {
    pub error: CompileError,
    pub input_mutations: Vec<InputMutation>,
    pub cache_bytes: usize,
}
fn compile_ast_setup(/* existing input/options/assets/limits */)
    -> Result<AstSetupOutcome, CompileError>; // carrier implemented; full phase coverage pending
```

`OriginGraph` 负责 view 解析、作用域、字段顺序、临时/保留费用；native 变换在真正赋值处记录精确事件。raw 投影复用对象，optimized Rule/AtRule 创建新浅包装并共用 metadata；声明与 comment 复用。static 克隆 body array；functional 克隆完整 definition，再选择 body，不能把 wrapper-only metadata 带到生成 class rule。固定依据为 `ref/tailwindcss/packages/tailwindcss/src/utilities.ts:5929`、`:6097` 和 `ref/tailwindcss/packages/tailwindcss/src/ast.ts:349`、`:391`、`:430`、`:443`。

## 必须修正的边界

准确事件顺序是初始 theme 数组替换/删除 → custom 注册归一化 → 最终 theme.nodes 和 keyframe context → variant Object.assign → guarded functions → apply → utilities kind/context → utility 删除。初始 theme 在 `ref/tailwindcss/packages/tailwindcss/src/index.ts:528`，注册在 `:612`，最终 nodes 在 `:636`，其余在 `:639` 到 `:685`。归一化即使文本相同也要赋值；variant 的 kind/selector/nodes 逐字段写入不能漏掉。

构造语义失败必须返回已计划的精确前缀。跨crate AstSetupOutcome与compile_ast_setup现已导出（`crates/rustify-style/src/compiler/public_api/compile_ast.rs:73`、`:1673`）；WASM由`crates/rustify-style-wasm/src/public_api/setup.rs:227`预检后host反馈重放。ordinary/custom函数在Err时也finish Edits，复用成功theme投影保留此前写入；12失败图/4原异常与额度边界通过。实际导入器共享FIFO、owned片段与Facade保留任务通过15语义失败/5原异常/1首次settlement/4拒绝不写边界，见[M4](../M4.md)；finite-bundle实际Promise边界、compatibility阶段拆分和orphan报告现通过20时序图/6 orphan/4额度；apply失败现在携带已完成setup/functions及原生逐directive替换，复用成功投影；16场景/两入口32比较、8原异常及额度边界通过。ordinary/custom函数按AST原位置共用一遍Edits/current view，19场景/38比较含8原异常通过；首错与已完成前缀按原顺序保留，caller at-root provenance不能选private slot。external loader、动态shared/Proxy walk、其他早期阶段与更广跨阶段顺序仍待完成。额度拒绝不写调用方；语义失败通过完整前缀费用检查后重放，若 host 先抛错返回 host 错误，否则返回语义错误。不发布半个 compiler。任意 getter/Proxy 的读取时序仍需要另行差分。

添加 origin 会影响公开 AstNode 的 enum literal 和 exact pattern；Serialize 必须跳过 origin，结构 Eq 必须忽略它，普通 Clone 保留而 fresh clone 刷新。该新 compiler 模块尚未跟踪；是否存在外部源码消费者未知。native类型迁移已在仓内构造、匹配、测试、WASM与utility-lab完成并检查；enum literal须补origin、exact pattern须接收origin或`..`。外部消费者仍未知；仅孤立原型阶段没有改AstNode。

origin-aware 的 compiler 克隆必须是 fallible，并在比例分配前预留实际峰值。快照仅保留选定根可达的数据，包含 metadata 指向的 detached 节点和 live journal 版本；不得每个候选保留整个输入。所有提取 definition、theme/keyframe、重复引用都须同步解析 current view。同一 public 对象的多个 native occurrence 在 guarded 变换前也须读取 current view，并同步 typed 字段；仅重绑 origin 可能让后续 guard 看到旧值，此项需独立差分。无法做到这些就废弃结构，不加 CSS/位置启发式。

## 实际实验与限制

`../custom-origin-prototype/src/main.rs:112` 使用真实 Rust PublicValue 展示：原 child 与 COW child 有相同语义 identity、不同 storage；普通 storage memo 克隆会拆散引用，current-view 解析后两条路径会合到同一新对象。`../custom-origin-prototype/oracle.mjs` 用固定 Tailwind 4.1.13 先执行真实注册/函数替换，再以 structuredClone 分别克隆完整 definition 与 body array。

两根的字段顺序、6 种节点形状、src/dst/extra/children 别名、相同内容独立 Source、分数位置和孤立 UTF-16 unit 图均与 oracle 一致；独立克隆彼此不同。保守实验 reserve 为完整根 119,728、body 根 114,032 bytes，低于阈值时 deep-clone allocation 计数为 0。首次原型误把 Text 当作 Display，编译错误已保存并修正；它不是语义 RED。

这是 COW 与克隆根的孤立验证。body 根使用功能注册后同一份数据测试根范围；没有验证另一条真实 static 注册链、native AstNode 携带/变换、apply 二次克隆、raw/optimized 稳定身份、setter 中间值、失败前缀和生产 allocator 峰值。实验 reserve 是保守演示值，不能当生产计费证明。该孤立实验时完整custom默认模式仍RED；后续native集成已通过默认14场景84阶段与5组alias图30阶段，未过滤字段。实验reserve不因此变成生产peak证明。

可重放：从仓库或现有 runner 运行 Node oracle；从仓库运行锁定 Rust 原型。使用本仓允许的临时构建根和工具链环境，不安装依赖。

```sh
node docs/validation/wasm-tailwind/custom-origin-prototype/oracle.mjs
mbx run --locked --offline --manifest-path docs/validation/wasm-tailwind/custom-origin-prototype/Cargo.toml -- docs/validation/wasm-tailwind/custom-origin-prototype/oracle.json
```

最小origin carrier、checked clone两根、COW/source/body别名与成功注册/函数事件已接入；现有source_locations表复用raw/优化canonical tuple。保留收费与保守预留不证明全部临时峰值，whole-book作用域裁剪仍须落实。成功imported caller图、media选项及搬移apply已通过；下一步落实更广loader/Source事件、variant/theme/keyframe精确事件及Failed setup完整阶段覆盖。完整现有图、输入、Source、额度和 frozen/sentinel 失败检查继续作为门；M4 退出条件不变。
