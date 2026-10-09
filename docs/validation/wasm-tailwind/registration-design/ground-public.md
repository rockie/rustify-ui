# Initial registration：公开 AST 与宿主回放的代码事实

本文只记录当前代码，不提出实现方案，不改变生产代码或验收状态。关注问题：初始 directive 变更如何穿过 native compiler、公开 AST、同步构造 / Promise `compileAst` 和失败 setup，最终作用于调用方的 live JS AST。所有行号对应本次读取的工作树；未运行测试、服务或浏览器。

## Components Found

| 组件 | 位置 | 职责 |
| --- | --- | --- |
| `PublicSetup` | `crates/rustify-style/src/compiler/imports.rs:303` | native 导入 / setup 对公开 AST 的回调边界，包含 root、parsed、begin、scheduled、media、compatibility、failed_functions、failed_apply、complete。 |
| `PublicAst` / `InputMutation` | `crates/rustify-style/src/compiler/public_api/compile_ast.rs:15`、`:24` | `Arc<Vec<PublicValue>>` 保存数组身份；有序 `Set` / `Splice` 保存对调用方对象或数组的操作，与优化输出分离。 |
| `Origins` | 同文件 `:94` | native declaration、authored node、theme、utilities、apply occurrence 与公开字段 / Source 的身份关联。 |
| `ImportViews` / `ImportGraph` | 同文件 `:111`、`:129` | 在宿主尚未写入时保存导入和 media 变更后的数组 / 对象视图，收集导入、media 与失败前缀日志。 |
| `NodeOrigin` / `Edits` / `Snapshot` / `Projection` | `crates/rustify-style/src/compiler/origin.rs:50`、`:115`、`:304`、`:431` | 保存原对象身份、当前字段与阶段共享视图；显式字段赋值产生日志；投影使用当前字段并维护生成图的别名。 |
| `Bundle` / `Program` / `Machine` | `compiler/imports/bundle.rs:340`；`compiler/imports/schedule.rs:55`、`:171` | 先用同一 reaction machine 准备 native 导入碎片和效果，再由 replay host 确认相同状态迁移；ready jobs 在 root 失败后仍有所有权。 |
| `AstSetupOutcome` / `AstSetupFailure` | `compiler/public_api/compile_ast.rs:73`、`:82` | 区分成功 compiler 和带语义错误、已记录效果及保守缓存 reserve 的失败会话。 |
| `PreparedAstSetup` / `Replay` | `compiler/public_api/compile_ast/setup.rs:16`、`:28` | start 前准备完整有限资源包；按 reaction 回放效果，发布一次结果，保留原宿主异常 token。 |
| WASM `Crossing` / `Completion` | `crates/rustify-style-wasm/src/public_api/setup.rs:10`、`:16` | 持有 setup、live JS 图和原始异常；同步入口内部驱动队列，Promise 入口逐个投递真实 `queueMicrotask`。 |
| WASM `Graph` | `crates/rustify-style-wasm/src/public_api/graph.rs:12` | 从 live JS 图读取共享身份，再把 mutation target 找回同一个宿主对象 / 数组，保留新建对象的 crossing handles。 |

## Flow

### 1. 先准备 native 和公开图，尚不执行调用方 AST 写入

WASM 同步 `AstCompiler::new` 调用 `setup::compile`（`rustify-style-wasm/src/public_api.rs:489`）；导出的 Promise `compileAst` 调用同模块 `prepare(..., Entry::Promise)`（`public_api/setup.rs:282`）。共同准备流程是：

1. 读配置与限额，验证 root 为数组；`graph::read` 按 own keys / 数组索引递归读取 JS 字段，JS `Map` 识别已经读完的共享对象，native key 同时记录 live `JsValue`（`graph.rs:29–96`）。读取 getter 可能抛异常，该路径直接通过 `Result<_, JsValue>` 返回。
2. native `compile_ast_setup` 校验 limits 和输入预算；`Origins::convert` 创建 typed AST，并给节点绑定 `NodeOrigin::caller(node.clone())`（`compile_ast.rs:1673–1707`、`:1216–1366`）。它不修改调用方 JS 对象。
3. `Compiler::new_with_ast` 执行 native setup：imports/media → source discovery → utilities selection → custom variants → custom utilities → theme → compatibility marker → theme functions / authored variants → ordinary/custom functions → apply → setup render / metrics（`compiler/mod.rs:208–368`）。这是当前实际顺序，尚非统一 initial registration walk。

native 简便入口 `public_api::compile_ast` 只把成功 outcome 提取为 compiler、把失败 outcome 提取为语义 error（`compile_ast.rs:1659–1668`）；真正需要失败效果和宿主确认的 crossing 使用 `prepare_ast_setup`，不能从该简便入口拿到失败 journal。

### 2. 导入的准备和回放共同使用 schedule，数组身份不能由 native 下标推断

`Bundle::visit` 为每个可加载 occurrence 记录 task 和 site。其 `public_index` 按 `NodeOrigin::object_id()` 对齐公开 parent 数组，而 `path` 使用 native index（`bundle.rs:76–90`）。两者可能不同，例如公开 undefined declaration 已在 `Origins::convert` 中跳过（`compile_ast.rs:1295–1299`）。

导入 begin 产生一个空 context，并记录原 parent 数组数字字段的 `Set`（`ImportGraph::begin`，`compile_ast.rs:157–165`）。loaded 解析资源，调用 `PublicSetup::parsed` 得到公开节点数组；complete 构造 base、layer、media、supports wrappers，记录对这个 context 的 `nodes` 赋值，并在 `ImportViews.arrays` 中将原 parent 的当前位置替换为完成的 context（`bundle.rs:199–265`、`:268–335`；`compile_ast.rs:401–458`）。wrapper 使用 import 原节点的 `src`，loaded 数组仍是同一个公开数组 handle。

`Bundle::expand` 用 `Boundary::Observed` 运行准备 machine，即使 root 已观察到失败，也继续 pop 剩余 ready jobs；保存观察时的 `prefix_length`，把当时已完成的 effects 前缀和完整 `Program` 交给 `scheduled`（`bundle.rs:384–405`）。`ImportGraph` 仅在有 task 或 root scope error 时保留 program（`compile_ast.rs:167–174`）。Program 内 begin / complete effects 包含失败观察点之后的潜在效果；其枚举是 reserve 集合，不应当作最终回放次序（`schedule.rs:78–82`）。实际时序由 Machine 决定。

`imports::expand_with_base` 先 bundle expand，再调用公开 `media()`，然后调用 native `transform_media`（`imports.rs:348–380`）。bundle 返回错误时，后两步不会执行。

### 3. media 使用两种 current view：对象当前字段和数组当前位置

`ImportViews.node` 以对象稳定 identity 查当前对象字段；`nodes` 以原数组 `Arc` 地址查当前内容（`compile_ast.rs:115–126`）。journal target 仍指向原对象 / 原数组，current view 描述尚未写给宿主的未来状态。

`ImportGraph::media` 的 `assign` 同时记录原对象的字段 `Set` 和更新 `views.objects`（`:216–229`）。它对 `theme(...)` / `prefix(...)` 向 descendant theme params 追加选项；对 `source(...)` 找到首个 params 恰为 `utilities` 的 tailwind，追加 params，并把当前 parent index 换为 `sourceBase` context（`:230–283`）。`reference` 给 media 原节点赋新 children context；未知条件保留为新 params；条件全部被消耗时，单子节点用 index `Set`，其他数量用原数组 `Splice`，同时调整 `current_index`（`:284–379`）。

后续遍历从修改后的 node 读 children 和 context base；遇 `@utility`、`@custom-variant`、`@theme` 则不进入 body（`:381–399`）。native `transform_media` 的跳过条件另包括特定 `@variant`、未列 `@utility`（`imports.rs:253–258`）；这是当前两侧代码边界，不能仅从一个遍历推断另一侧。

### 4. theme journal 的可见中间状态及其与 compatibility 的分界

`theme_setup_mutations` 重新从 import/media current view 找 theme rule 和 declaration Source，建立 theme output declarations。native token source 优先通过 `Origins::location` 解析；找不到再按 prefixed/escaped property 回退，reference token 不进入最终声明列表（`compile_ast.rs:566–630`）。

`theme_input_mutations` 深度优先遍历 current arrays / fields：第一条 theme 在原数组当前位置被新空 `:root, :host` rule 替换，之后 theme 通过原数组 `Splice` 删除；同数组后续 index 减去已移除数。`first` 跨所有递归共享，因此不是每个 parent 都各有第一条。utility/custom-variant bodies 不进入。最后单独给新 rule 的 `nodes` 赋 `{theme:true}` context 和声明（`:633–685`）。原 theme 对象和旧 children 数组不会因此被字段覆写。

成功组装时，`compile_ast_setup` 从 theme journal 暂时 pop 最后的 `nodes` assignment，然后按以下顺序保存（`:1787–1810`）：

1. import prefix + media mutations；
2. theme 数组替换 / 删除；此后就是 `setup_compatibility` offset；
3. custom registration normalization assignments；
4. theme 最终 `nodes` assignment；
5. function assignments；
6. apply replacements；
7. utilities 原节点的 `kind = context`、`context = {}`；
8. utility definition 数组删除。

apply 先根据 current import/media view 重绑 occurrence 的 parent/index，再用 `ArrayPositions` 的 Fenwick prefix 表换算前面删除及替换后的下标（`:521–564`、`:688–745`、`:1181–1213`）。utility 删除使用同一本 positions，仅穿过 context 递归（`:748–775`）。这些是数组位置 seam，不等同于字段 view。

### 5. Edits 让后续 native 消费者和 output projection 看到当前字段

`Edits` 以数组地址 / 对象稳定 identity 保存 `View {original,current}`；从 origins 的已有 views 导入共享 book，`NodeOrigin::fields()` 返回 current（`origin.rs:18–46`、`:77–82`、`:115–157`）。`assign` 更新 current，caller origin 追加 target 为原对象的 `Set`，同时更新 typed declaration value / at-rule params（`:182–199`）。`refresh` 只在本阶段有显式 edit 时同步这两个 typed 字段，避免覆盖其他 typed transforms（`:160–180`）。

functional utility normalization 使用同一个 `Edits` 遍历，遇 `--value` / `--modifier` 时赋 declaration value；成功 collect 后 `Snapshot` 绑定普通 nodes、definition body 和 definition root origin，normalization journal 从 snapshot 取出（`custom_utilities.rs:179–318`、`:700–805`）。`Snapshot::bind_origin` 保留原值和 caller 标记，只添加共享 views（`origin.rs:295–325`）。

theme keyframes 是另一个保留 seam：`Theme::extract` 把 `@keyframes` child clone 到 `theme.keyframes`，首条成功 theme 留 native anchor；不把原 caller keyframe body 清空（`compiler/theme.rs:143–147`、`:237–248`）。公开 `theme_setup_mutations` 只从 ordered tokens 建立 declarations，final rule.nodes 是 theme context 包裹这些 declarations（`compile_ast.rs:618–630`、`:682–683`）。因此 theme.keyframes 的 native 保留、原 theme/keyframe对象的 caller aliases、公开 rule 的最终 nodes 是不同观察面；早期 Theme extraction `?` 失败时尚未有该公开 theme 替换 journal，不能借后面的 output projection 补出已经丢失的 registration 前缀。

`Edits::sync` / `sync_root` 为 native 结构准备当前图，但并不把所有结构同步都变成宿主写日志；显式 `assign` 才追加这些字段效果（`:201–301`）。`Projection::new` 建立该当前图；caller fields 取 Edits current，generated graph 递归投影并缓存 storage 别名（`:431–514`）。这层与 WASM Graph 的 live handles 解决不同问题，不能互相替代。

### 6. 语义失败保留哪些效果，由实际回调位置决定

ordinary/custom function 处理失败调用 `failed_functions(theme, normalization, function_mutations)`；apply 失败调用 `failed_apply`，额外传入已完成 apply replacements（`compiler/mod.rs:303–334`）。公开 `failed_functions` 重建 theme 效果，保留 theme 数组前缀长度，然后接 normalization、theme 最终 nodes、function effects；`failed_apply` 再添加已完成 apply replacements（`compile_ast.rs:179–214`）。

`compile_ast_setup` 把 import/media journal 和上述失败 journal 合并；`reached_compatibility` 决定是否有阶段 offset。非 Limit error 变成 `AstSetupFailure`，原语义 error、已记录效果和 reserve 一起保存。任何 `CompileError::Limit` 直接返回 `Err`；失败 reserve 再超额也直接 `Err`（`:1725–1757`）。

需要准确区分早期错误：sources/custom variants/custom utilities/theme extraction，以及 compatibility 后的 `theme.prepare_functions`、authored variant 处理，都在 `failed_functions` 回调前使用 `?` 返回（`compiler/mod.rs:231–258`、`:299–304`）。functional utility `collect` 错误也发生在 `Edits::finish` 和 normalization journal 赋值之前（`custom_utilities.rs:308–318`）。当前公开 failure 对这些路径只拥有此前实际回调已经保存的效果；不能声称每一个早期 registration mutation 都已经进入失败前缀。这是现状 seam，本文没有修改它。

### 7. full finite-bundle reserve 先 admission，之后才 replay

`prepare_ast_setup` 取 native 完整 outcome / program / compatibility offset，收集所有 program effects 加 outcome 的 post-import effects，给 reserve Vec 容量计费，再检查 cache limit 和 `Machine::prepared` 的 queue limit（`compile_ast/setup.rs:78–126`）。`reserve_mutations()` 明确包括 root rejection 后的潜在 host values（`:154–164`）。没有 program 的 Promise preparation 创建空 root scope，仍计入 machine/program 并检查额度（`:129–151`）。

WASM `prepare` 再把 native cache reserve、input Graph retained handles、`Graph::output_bytes([], reserve_mutations)` 和 Promise Completion reserve 相加；超过 caller cache limit 时直接返回错误（`rustify-style-wasm/public_api/setup.rs:198–246`）。这些检查完成后才调用 `start` / `start_async`。这是代码中的“超额无 caller AST 写入”边界；读取 getter 的用户副作用不在“无写入”保证内。这个保守计量也不等于实测 allocator peak。

### 8. 同步构造和 Promise 保留相同 reaction 次序，但宿主驱动方式不同

Machine start 同步 walk root：每个 task 先创建 Loaded reaction，再调用 host begin；begin 失败停止该 scope walk，但已经创建的任务仍在 ready/outstanding 集合里（`schedule.rs:290–350`）。Replay begin 在调用 user setter 前先把当前 ready jobs 逐个 post，以保留 loader await 与 setter 自己 queueMicrotask 的顺序（`compile_ast/setup.rs:33–47`）。nested scope 完成后才运行 task complete；Promise.all member / forward 再触发 ParseCss（`schedule.rs:352–398`）。

Prepared drive 在 successful `ParseCss` 回放 `prefix_length..compatibility`，successful `Compatibility` 回放 `compatibility..length`，后者完成时返回保留的语义 failure；effect 的宿主异常会提前截断该范围并成为 fault（`compile_ast/setup.rs:166–184`、`:260–306`）。没有 compatibility offset 时 ParseCss 就回放全部并结束语义阶段。

同步 constructor 调用 `start(Boundary::Observed)`，在本次 JS 调用中用 VecDeque 驱动 jobs 到 Observe 出现结果；成功发布 compiler，失败返回原 fault，并把仍在 ready 的 jobs 投递到真实 microtask（`rustify-style-wasm/public_api/setup.rs:248–277`）。无导入 program 的同步路径在 start 直接回放并发布（`compile_ast/setup.rs:224–245`）。

Promise `compileAst` 在 JS Promise executor 中 prepare 和 start，逐个用 `queueMicrotask` 执行反应；没有 imports 也有 synthetic root scope，所以 theme 数组效果属于 ParseCss，normalization / theme final nodes / function / apply 属于后面的 Compatibility。Machine Promise boundary 在 CompileAst reaction 发布，不添加同步入口的 Observe（`schedule.rs:299–307`、`:392–407`）。WASM callback 从 `Rc<RefCell<Option<Crossing>>>` take 出 Crossing，执行 user setters 时不持有 RefCell borrow；结果发布后只要仍有 pending jobs 就把 Crossing 放回（`rustify-style-wasm/public_api/setup.rs:167–191`、`:282–310`）。

### 9. live JS 写入、原始异常和失败后的 detached work

`Graph::apply` 对单个 effect 投影 target / value，再执行 `Reflect::set` 或调用 target 自己的 `splice` 方法。这里每条 Set 都实际执行，不做值相等短路；`Reflect::set` 返回 false 则产生 TypeError，抛出的 JS 值通过 `?` 原样返回（`graph.rs:172–184`、`:297–337`）。未修改的 caller native storage 直接复用 live JS handle，避免按照旧数组快照重建已被前面 effects 改过的内容（`:220–228`）。尚未发布的 nested import arrays 也先建立稳定 handle（`:306–312`）。

apply 无论成功或失败都 retain 新建图，因为 setter 能先捕获 context 再抛异常，已启动 import task 的 complete 仍要写同一个 context（`:181–184`）。Crossing 将原始 `JsValue` 存入 faults，native 只持有 index token；Host fault 恢复为原 JsValue，Semantic fault 才构造 compiler error（`rustify-style-wasm/public_api/setup.rs:24–62`）。因此 replay 的较早 host exception 能取代保留的 native semantic error。

Prepared publication 仅一次；take_result 出错会取走 native outcome，但 program/machine 仍可执行已启动任务（`compile_ast/setup.rs:199–204`、`:308–335`）。Crossing root reject 后仍持有 graph 和 faults，直到 jobs quiescent；孤立未处理 import rejection 通过原始 fault 创建 `Promise::reject` 交给宿主报告，不添加吞错 catch（`rustify-style-wasm/public_api/setup.rs:150–191`）。“root rejected”不代表所有 caller 写入已经停止，也不是 journal rollback。

## Files Read

项目上下文：`README.md`、`Cargo.toml`、`mise.toml`、`PRODUCT.md`、`DESIGN.md`；相关计划 / 记录：`docs/plan/WASM-TAILWIND.md`、`docs/validation/wasm-tailwind/M4.md`（本次只读取相关段落 / 搜索结果，未据此声明新的验收结果）。探索约定：`.agents/skills/how/SKILL.md`、`.agents/skills/how/references/explorer-prompt.md`。

代码读取为上述定位的相关行段：

- `crates/rustify-style/src/compiler/mod.rs`
- `crates/rustify-style/src/compiler/imports.rs`
- `crates/rustify-style/src/compiler/imports/bundle.rs`
- `crates/rustify-style/src/compiler/imports/schedule.rs`
- `crates/rustify-style/src/compiler/public_api/compile_ast.rs`
- `crates/rustify-style/src/compiler/public_api/compile_ast/setup.rs`
- `crates/rustify-style/src/compiler/public_api/source_map.rs`
- `crates/rustify-style/src/compiler/origin.rs`
- `crates/rustify-style/src/compiler/custom_utilities.rs`
- `crates/rustify-style/src/compiler/theme.rs`
- `crates/rustify-style/src/compiler/sources.rs`
- `crates/rustify-style/src/protocol.rs`
- `crates/rustify-style-wasm/src/public_api.rs`
- `crates/rustify-style-wasm/src/public_api/setup.rs`
- `crates/rustify-style-wasm/src/public_api/graph.rs`

## Boundaries

这些现有接口构成统一 initial registration 必须保持的行为 seam；以下是代码观察，不是接口变更建议。

- **native fragments ↔ import schedule**：occurrence 身份、begin 空 context、loaded children 数组、complete 同一个 context、scope 返回 / aggregate 时序、观察后的 owned jobs，均通过 Program/Machine/Host 连接，不能当作只有最终 expanded AST 的一次替换。
- **typed AST ↔ public current fields**：`NodeOrigin` 保存原对象身份，Edits current 描述字段变更；ImportViews 另描述 import/media 的数组位置和字段。功能值处理、后续 projection 与 journals 的目标必须仍能落回原 caller 对象。
- **registration ↔ compatibility replay**：theme 的空规则 index replacement / 后续 theme splices 在 compatibility offset 之前；normalization 和 final theme.nodes 在 offset 之后；成功与被捕获的函数/apply失败共享该分界。
- **Source metadata ↔ graph**：`SourceLocation.public` 是 canonical 公共图，保留共享 Source、JS number offsets 和额外对象字段；`Source` typed 形状只有 file/code，offset 计 UTF-16（`source_map.rs:7–21`）。`Origins::convert` 为 directive 创建的辅助 source wrapper 不是新增 src 字段；`Edits::native_value` 也避免这样发布（`origin.rs:255–263`）。parsed imports 在 public 模式不使用 parsed AST cache，按 occurrence 创建 Source，`bind_locations` 用 Source 指针和 destination 标志缓存位置 tuple，使 src/dst tuple 分开、Source 共用；`source_owners` / Program.sources 保留并计费 Source owners（`bundle.rs:224–235`；`compile_ast.rs:984–1028`、`:1070–1085`）。
- **source discovery ↔ registration**：`SourceMetadata` 是另一种数据，仅包含 sources / root，核心不遍历其路径（`protocol.rs:69–80`）。native sources extract 在 import/media 之后读取 base/source_base/reference/nested，首个非 reference utilities 决定 root，inline sources 产生候选/blocked，路径 sources 产生 patterns（`sources.rs:210–395`）。公开 media 的 sourceBase wrapper 和 native source_base 必须各自维持其现有语义，不能把 source discovery metadata 与 source-map Source 混为一物。
- **semantic result ↔ admission / host replay**：语义失败 outcome 可带前缀；Limit Err 不授权 AST 写入；crossing 的 reserve 覆盖全部潜在效果和失败后的 continuation，start 才进入 live writes。
- **host acknowledgements ↔ error identity**：Set / custom splice 的成功或原抛出值进入 settlement；较早 host exception 优先于尚未回放完的 native semantic failure。新 context 被 setter 捕获后即使 setter 抛出也要保留 handle，直到已启动任务结束。

## Gotchas

- input journal 与 optimized build result 是两条公开行为。WASM build 会复用原 input / previous result 数组身份，另外对 output projection 与 input mutations 做 crossing（`public_api.rs:492–536`）；只比最终 AST 值不能证明 constructor effects 和异常时序。
- object identity 和 storage 是两个 key：ImportViews/Edits 使用稳定对象 identity；WASM Graph 对同 identity 的同 storage 直接复用 live host value，changed storage 则走投影。只按值相等或 native declaration identity合并，会漏掉 caller aliases 和 setter observations。
- 第一条 theme 替换时新规则为空；final nodes 是后续 assignment。把它们合并为完整 rule 会丢掉中间 setter 可见状态和 compatibility microtask 分界。
- setup reserve 枚举的 effects 顺序不是 import reaction 的真实时间顺序；真实时间顺序由已启动 task、nested completion、aggregate、ParseCss、Compatibility、CompileAst / Observe 决定。
- 早期 registration error 当前没有普遍失败 journal hook；本文确认为现状缺口，不声称已完成兼容。function/apply 的已有失败 hooks 不能代表所有 `?` 返回点。
- 公开 media 在 registration extraction 之前遍历完整可进入图；其已经记录的后面 sibling params assignment 会在失败组装时保留（`imports.rs:376–379`；`compiler/mod.rs:220–256`；`compile_ast.rs:1725–1754`）。因此早期 registration 错误前缀既可能少了前面 registration effects，也可能带上文档位置更后面的 media effects；这是当前阶段拆分的直接代码后果，本文未通过运行给出影响场景数。
- admission 全部发生在 start 之前，但 getter 可以在 graph read 期间运行。所谓“无写入拒绝”指 compiler 未 replay caller AST effects，并不承诺任意用户 getter无副作用。
- 同步失败返回后，已启动导入的 microtask 仍可能更新 caller 捕获的 context；Promise root reject 同样不取消任务或回滚已经完成写入。

## Open Questions

- 本角度没有读取固定上游 parseCss 原文，因此不把当前 native 分阶段顺序当作完整上游等价证明，也不指定统一 walk 的最终设计。
- 没有运行本轮 Node / native / WASM differential 或真实浏览器；上述 microtask、getter/setter、原异常和配额结论来自当前源码路径，不是新的运行验收证据。
- 没有审计完整 temporary allocator peak；本次核实的是 finite bundle native/journal/host handles reserve 和 start 前 admission。统一峰值、所有早期失败、全部 M4 范围仍需独立完成；本记录不改变 goal 的 3/7 状态。

## 补充：media source的replacement/Stop反证

补充反证：第46行的「首个」描述本仓现状，不是固定上游语义。上游`index.ts:454–457`的source callback替换后返回Stop，`ast.ts:195–207`却先处理replacement并继续兄弟，因此还会处理后续精确utilities节点。[实际WASM补充探针](../utf16-families-registration-media-source-before-red.log)在2场景/两入口4比较中全部前缀失败：上游两次params赋值，本仓一次，语义错误相同。
