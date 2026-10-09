# compileAst 调用方 AST 变更设计

关联：[WASM-TAILWIND](../../plan/WASM-TAILWIND.md)、[M4 验收](M4.md)。完整输入变更契约仍待完成；本记录说明 utilities、theme、直接 @apply 与custom utility origin/成功事件链路及后续约束。

## 问题与调用方式

固定上游在构造时改写调用方 AST，在 build 时把原始候选数组赋给 utilities 节点，再优化返回值。行为事实源为 `ref/tailwindcss/packages/tailwindcss/src/index.ts:670` 的构造赋值、`:799` 的 build 赋值和 `ref/tailwindcss/packages/tailwindcss/src/ast.ts:281` 的 optimizeAst。原实现只发布优化后的输出，候选在缓存前也已被裁剪，无法从输出恢复原始节点与数组身份。

调用方式保持不变：

```js
const oldChildren = utilities.nodes
const compiler = new AstCompiler(input, config)
// utilities 仍是原对象，kind 为 context，原字段保留。
const first = compiler.build(['border-2'])
// utilities.nodes 是新数组，保留 at-root；oldChildren 保持原内容。
const second = compiler.build(['flex'])
// 缓存候选对象与声明共享；优化后的 rule 包装是新对象。
console.assert(compiler.build([]) === second)
```

Native 的 `PublicAst = Arc<Vec<PublicValue>>` 继续表示不可变快照。WASM 通过明确的变更记录操作真实 JS 对象，无需把 Rust 的公共值迁移为共享可变对象。

## 结构与所有权

`crates/rustify-style/src/compiler/utilities.rs:600` 的 finalize 保留唯一的原始候选树，render 负责最终裁剪和去重。`crates/rustify-style/src/compiler/public_api/compile_ast.rs:105` 保存原始候选公共投影；指针索引同时持有对应 Arc，防止地址复用；`:1950` 的 build 投影复用缓存。原始投影和优化输出共用声明身份表。

```rust
pub enum InputMutation {
    Set { target: PublicValue, field: Text, value: PublicValue },
    Splice { target: PublicValue, start: usize, delete_count: usize, values: PublicValue },
}
```

utilities 链路使用字段赋值：构造阶段按顺序设置 `kind`、`context`；候选数量或已使用变量变化时赋值 `nodes`。原始输入对象的稳定身份用于找到原 JS handle，不按 selector 或输出位置定位。

theme 链路由 `crates/rustify-style/src/compiler/public_api/compile_ast.rs:633` 记录原数组操作：第一条 theme 用单索引赋值替换，后续 theme 用原数组的 splice 删除，随后给新规则赋最终 context/声明。新对象先以空 children 发布，保留 setter 与后续 splice 抛错前已经发生的写入。声明使用 native 最终 ThemeToken 的顺序、选项、值与来源，和优化输出共享身份；原 theme 对象及旧 children 数组保留。导入/media搬移的下标与原父数组已覆盖；更早删除的其他组合及完整解析失败前缀仍待完成。

直接 @apply 的替换节点在 `crates/rustify-style/src/compiler/apply.rs:486` 的实际 arena 替换处记录，继续使用 native 的依赖顺序和声明身份。`crates/rustify-style/src/compiler/public_api/compile_ast.rs:1181` 将每条原 directive 的独立 canonical Source wrapper 关联到原数组及原位置；wrapper 的 Arc owner 保留到投影结束，不把共享 tuple 的内容当成节点身份。单节点用索引赋值，其他长度用原 splice；原数组前面的 theme 删除和 apply 展开通过前缀长度调整位置，避免逐条扫描前缀。原始新声明和优化输出共用声明缓存，临时替换记录及原位置表投影后释放，保留的公共声明表、输入操作与 JS handles 计费。utilities 构造赋值在 apply 操作之后执行，保留 apply setter/splice 失败前缀。native size 原树补回实际 --tw-sort 声明；apply 的 at-root 包装从其共享 candidate 来源恢复 src。直接成功构造、原异常与额度边界已覆盖；custom字段/归一化/函数的14场景及5组metadata别名已验证；空展开、更广共享AST、多阶段失败及所有raw family仍待完成。

`build_text_with_input_check` 在候选提交前检查原始公共树、变更记录与优化输出；已有 `build_text_with_output_check` 保留其接口。额度拒绝不发布缓存、原始候选身份或 JS 写入。`Graph::write` 使用同一张 next handle 表，先执行定向输入操作，再生成优化输出，避免两条路径产生两份声明对象。输入赋值用 `Reflect.set`；splice 读取原数组方法并以原数组作 receiver 调用，保留方法抛出的原异常。构造创建的 handles 并入 input 图并收费，后续 build 不丢失其身份与保留费用。新输出字段沿用现有 writer。

host crossing 失败仍只恢复上次成功发布的输出，保留已接纳候选与计数；不能把它当作额度拒绝或声称回滚已发生的 JS 写入。导入begin/complete失败后的materialized handles由Graph::apply保留，捕获context的后续完成已验证；其他首次/部分写入失败及完整accessor/proxy时序尚待验证。

来源由 native 的 canonical public 元数据携带，utilities 的来源参与生成节点与 property fallback；无来源的 fallback 保留显式 undefined。排序元数据也需保留 `--tw-sort` 的实际作用，而不能仅依赖最终 CSS 看不到该声明。

`crates/rustify-style/src/compiler/mod.rs:1002` 在规则优化后裁剪 theme：未使用变量删空后，移除其 at-rule 祖先，遇 style-rule 父节点停止。所需变量已保留时，其他未用声明不影响规则的非空判断；全部未用时用一条真实可输出声明保留该判断，优化后按身份删除，避免每次重建全部 theme 声明。完整 raw theme 声明仍保留在调用方图中。局部前后测量及输出摘要见 M4；完整分配与性能终验仍 pending。

最终custom删除在utilities赋值后记录原数组splice，透明context遍历共用位置表；共享位置已经删除时跳过。独立NodeOrigin现已接入六种native节点、转换和fresh clone；当前COW view在深克隆及私有投影中收口，Src不充当Origin。注册归一化与guarded函数在实际赋值处记录事件，alias occurrence先刷新typed值；成功theme最终赋值与注册时刻分开。raw fields保留，优化Rule/AtRule为浅包装，生成src/dst的canonical tuple由既有source_locations表复用并计费。

writer对未变caller storage返回live JS handle，避免后续Source metadata把先前apply的caller数组恢复成旧内容。原默认14场景84阶段和5组metadata/wrapper/Source/函数别名30阶段通过，见[M4当前证据](M4.md)。[结构合同](custom-origin-design/synthesis.md)仍要求精确语义失败前缀、其余事件、快照裁剪及统一峰值，当前保守checked clone与保留计费不证明全部临时分配已经受控。函数阶段失败已通过12图/4原异常与额度边界：AstSetupOutcome携带prefix和费用，WASM预检后重放，host异常优先；实际导入器/Facade已接入共享FIFO与保留任务，15导入语义失败、5原异常、1首次settlement和4拒绝不写边界通过；finite-bundle实际Promise边界、compatibility阶段拆分和orphan报告已通过20时序图/6 orphan/4额度；@apply失败现保留已完成setup/functions与逐directive替换，16场景的同步/Promise两入口32比较、8原异常和额度边界通过；ordinary/custom函数现按AST原位置使用同一Edits遍历，19场景/38双入口比较含8原异常覆盖顺序、首错、失败前缀、shared声明与caller at-root marker；同一对象guard读取最新值。external loader、早期theme/注册/variant失败及更广跨阶段/shared/Proxy行为仍待补齐。

## 比较与选择

architect 的 Ground 由两个代码探索角色与一个 source-control 调查角色提供，另一个角色合成并抽查引用。两个独立上下文产生结构不同的方案，独立判断角色比较方案；设计比较未运行实现测试。部分默认模型不可用，采用当前可用模型，其中一个候选使用该技能列出的 GPT-5.6 Sol max。

| 方案 | 取舍 | 处置 |
| --- | --- | --- |
| 原始候选为唯一缓存，独立 caller 图 | 优化归 render，避免两份候选事实；新 CallerGraph/ID 系统增加表示转换 | 采用原始缓存，暂不引入第二套图类型 |
| 变更 journal，按需保留 raw 与 optimized 两份候选 | 操作与部分写入清楚；两份树和 capture hints 增加同步、收费责任 | 采用明确赋值记录，未采用第二份 native 候选缓存与 capture 配置 |
| 从 Prepared.ast 重建输入 | 代码较少，但丢失内部节点、数组操作和原对象身份 | 排除 |

两个候选分别被要求探索上述结构，差异不能仅归因于模型。选择依据是原始/优化图、身份、额度与错误边界，以及现有接口；未声称性能胜出。Ground 查不到目标文件的 Git 历史：实现与记录尚未跟踪，原始私人讨论未知。当前身份、getter 恢复与 DAG 预算理由来自 M4 的既有记录；其他外部证据不可用或没有关联引用。

## 验证与后续

先保留实际失败证据，再比较构造完成和每次 build 后的完整 alias 图。覆盖原 utilities 对象、额外字段、旧数组、原始 at-root/内部排序节点、优化包装、声明/Source 共享、无变化与非法候选；额度失败检查未写入与可恢复性。完整 native、WASM API、CSS 与来源回归必须覆盖优化位置变化。

导入实际parser节点已绑定origin，初始替换/完成、media选项及移动apply父数组通过21+顺序/107阶段；Source地址缓存保留strong owner并计费。完整输入图27场景已通过，成功路径不证明全部失败/访问时序。后续完成更广source/variant/function 的构造变更和更广泛 apply 原数组行为、更广泛 theme 优化/build 变更与原数组操作、所有 family 的原始结构，以及解析失败前缀、首次/连续/部分写入失败、mutable/helper/async API 与完整分配预算。提前读取 JS 图再重放成功事件不能证明全部 getter/setter/proxy 时序。M4 退出条件不降低，完整输入图差分已作为三个flags的默认gate，继续保留更广范围与失败时序检查。
