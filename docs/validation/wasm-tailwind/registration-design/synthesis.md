# 初始注册：条件性选定结构与接入合同

计划仍是3/7，M4进行中。生产统一初始visitor已接入，48场景/96比较及媒体4/4通过；本文件的完整ownership合同尚未交付，ownership已扩至14场景/28比较并全部通过，包含variant嵌套/重复alias、live模板、资源与共享Source，已加入默认compiler-api门，当前证据见[M4](../M4.md)。以下保留原设计调查的基线：[实际WASM RED](../utf16-families-registration-expanded-before-red.log)为45场景、90次比较、86失败；8项首错与82项前缀图差异有重叠，epoch差异为0。

## 调用方先看

`Compiler::new_with_ast`仍负责一次构造，不要求native调用方依次协调register、normalize、lower。内部注册owner接收导入完成后的当前输入，返回完整注册数据或语义失败及已经完成的效果；资源拒绝仍是外层Err。public adapter仅翻译效果，既有setup machine拥有ParseCss/Compatibility反应时刻和host反馈。

```rust
// 私有合同草图，未实现；PublicValue只作已有存储，不新增wire/API。
enum RegistrationInput { Parsed(Vec<AstNode>), Public(PublicAst) }
fn prepare_registration(input: RegistrationInput, bundle: Bundle, limits: &Limits)
    -> Result<RegistrationOutcome, ResourceRefusal> { todo!("not implemented") }
struct Registration {
    root: PublicAst,
    current: CurrentOverlay,
    state: Registrations,
    effects: Vec<InitialEffect>,
}
struct CurrentOverlay {
    // Existing strong original/current views + explicit array-version lineage.
    // Original object identity and original array anchors are distinct from storage.
    views: CurrentViews,
    versions: ArrayVersions,
}
struct Captured { anchor: PublicValue, version: PublicValue }
enum InitialEffect {
    Assign { target: NodeOrigin, field: Field, value: Captured },
    Replace { target: PublicAst, index: usize, deleted: usize, items: Captured },
}
enum Field { Name, Params, Children }
enum DeclarationValue<'a> { Missing, Undefined, Null, Text(&'a Text) }
struct InitialRecord { effects: Vec<InitialEffect>, retained: CurrentOverlay }
// Ready and Failed own the same current store until projection/quiescence.
enum RegistrationOutcome {
    Ready(RegisteredInitial),
    Failed { error: CompileError, record: InitialRecord },
}
struct Cursor { array: PublicAst, index: usize, path: Vec<NodeOrigin>, context: Context }
// Resolve borrowed domain fields from an external shallow immutable lease;
// release the local borrow before mutation, re-lease after writes, re-read children.
enum WalkAction { Continue, Skip, Stop }
struct VisitResult { action: WalkAction, replacement_count: Option<usize> }
```

类型仍是合同示意，没有完整生产接口。改为优先验证现有`PublicValue`/NodeOrigin/current COW overlay，不新增NodeId/ListId arena。array版本须显式关联原anchor并保留强owner，不能按内容相等或裸地址复用猜身份。赋新`nodes`边只改变该边；splice改变同一数组的全部alias。独立fresh clone与普通clone的身份规则还需验证。保留Missing/Undefined/Null仅保证不丢表示，不擅自承诺扩大支持范围。

`Captured`区分赋值时不可变storage与原caller live alias。生成值投影用当时版本，已有caller对象/数组直接复用原host handle，不重新灌回旧或最后字段。原赋值、同值赋值、index Set和原receiver splice是事件。已有`Edits`仍含InputMutation和只刷新value/params；复用其provenance/current概念不等于直接复用现存类型就能满足结构合同，更不等于可以先删掉后期journal。

## 选择、修订与证据

最终条件性选择[existing-value候选](candidate-existing-value.md)，保留前两案的native领域effect/不可变版本边界。[补充独立审查](review-existing-value.md)在两个展示顺序都优先建议验证该方案；生产采用证据仍不足。前两案收敛同一canonical arena，因此追加了真正不同的existing graph+sparse overlay方案。它减少一次presence/identity转换，但仍有array lineage/identity基础设施与收费成本，不宣称更快或更省内存。此前[arena条件稿](synthesis-arena-conditional.md)为历史对照，当前接入依据为本文件。

[初始审查](review.md)以及补充审查均同一可用模型/共享文件系统、指令分隔；judge误读旧manifest映射已披露，不称盲测。第三候选未读另两案/评审/综合，父任务补充了Undefined/lease接口条件；结构干预与模型差异不混淆。独立纸面判断不等于生产实测。

[WHY约束](why.md)区分旧表示的条件性取舍与必须保留的语义。canonical图是当前设计推断，尚无性能/完整峰值优势证据。共享native领域层不直接依赖public `InputMutation`；已有后期函数/apply日志仍在其既有provenance/public边界，不能扩散到初始grammar。native构造与public setup共用同一handler；adapter不识别指令、不再次验证。

[原型](cursor-prototype.rs)仅验证cursor/当前拓扑，用固定上游[ast.ts](../../../../ref/tailwindcss/packages/tailwindcss/src/ast.ts)实际walk比较18场景；[恢复后结果](../utf16-families-registration-cursor-restored-differential.log)全部一致。覆盖共享数组删除、Context parent与false/empty override、0/1/2替换的Continue/Skip/Stop、嵌套Stop、visitor重赋children、延迟body读取、AtRoot parent和失败后不继续。三个错误版本分别在Stop优先级、丢false/empty、旧children读取处被抓住，见[cursor反证](cursor-mutations.json)。这是隔离native实验，**没有证明生产注册、Source/metadata、host/Proxy、资源峰值或任意导入器正确**。

[existing-value存储原型](existing-value-prototype.rs)实际链接本仓compiler的PublicValue类型，在隔离temporary Cargo workspace通过5项检查：共享数组删除影响另一alias、早期storage版本仍解析最新anchor、重赋edge与splice区分、捕获storage不随后续更新漂移、对象COW保留identity。[恢复后结果](../utf16-families-registration-existing-value-restored-probe.log)和[反证](existing-value-mutations.json)确认“绕过current数组”和“丢version anchor”都在各自见证处panic（预期101）。这些仅是存储原语，不是完整cursor、domain handlers、fresh clone、Source或host投影验收。

媒体规则还有[实际WASM补充RED](../utf16-families-registration-media-source-before-red.log)：两个媒体条件×两入口4项全失败，上游给两个utilities原对象赋params，本仓只给第一个，首个语义异常一致。原native候选和proposal-2的first-match措辞已按原文和实验改正，提交前SHA保留在manifest；旧文本不是当前规则。主45/90/86 RED统计与此补充诊断分开，不合并重复样本。

`ast.ts:195–207`先处理replacement，之后才处理Stop。有replacement时Continue重访该index；Skip或Stop推进插入数量并继续兄弟，Stop不向外传播。无replacement的Stop才停止整个walk。原型捕获此差别；不要把普通Stop规则套在media targeted callback上。Context不调用visitor、不成为parent；普通AtRoot会成为parent。每次迭代与visitor之后的children读取都解析当前cell。

## 输入与导入边界

原始public输入直接进入已有不可变值图，先于`Origins::convert`的丢undefined/合并Context presence/复制body，保留原数组槽位。lossy native Vec不能事后恢复这些事实。native parse输入一次适配到同一存储，源/身份codec不运行directive grammar；Absent body与Present空数组区分。public结构准入以实际契约为准，新支持形态另做差分，不凭保留原字段就宣称undefined/null全部支持。

导入仍由已有Bundle/Program/Machine完成。binding sidecar按实际occurrence更新原列表槽位、loaded current数组与生成context；不能使用lowered native数组下标反推caller位置。注册仅在import root成功settlement后开始，失败时不执行media/初始handler；已经启动的import jobs仍继续。移除`expand_with_base`末尾全树media处理后，media移到注册visitor实际遇到的位置，不改变begin/complete、队列、orphan与首次settlement。

ImportViews现在复用Current；实际begin/complete已使用有局部cache_bytes上限的结构操作，新增检查覆盖undefined原槽位、Context空/false字段、共享原数组、旧版本读取、生成context身份与begin不可变版本；导入事件复用live operand anchor，完整captured host投影仍待接入。native/public现在共用Current上的utilities/source/media/theme/custom variant/legacy variant及utility capture入口；旧媒体/source collector、生产typed theme/custom variant collector及事后初始theme模拟移除。媒体4/4与主注册48/96通过。native adapter恢复原Source/跨度并释放caller provenance；public为导入仍先convert。functional utility归一化与原值theme publication已在兼容后的同一Current执行，随后降低root/deferred bodies；已发布声明、附着utility body及最后的keyframes现在在同一raw Current实际函数walk中逐次赋值，完成的赋值在失败时保留；utility补全先于函数替换捕获，旧setup函数Edits及独立失败journal移除。authored variant也已按捕获receiver顺序在该Current执行，复用variant kernel并保留opaque原body数组；实际clone时只读所选live模板，没有全局刷新或逐节点Current复制。共享声明、theme/函数前缀及variant/slot资源的14场景/28项实际WASM比较全绿并进入默认门。导入前typed转换仍先运行，单次ownership完整移交未完成；sidecar向单一原始注册图移交仍是**尚未通过的接口条件**。不得新增第二个directive parser或用CSS/span猜身份。

## Handler与后期交接

初始owner按上游分支顺序处理utilities选择、utility capture、source、legacy variant转换、custom variant、media和theme。从现有source/theme/custom模块抽出不遍历的领域操作；禁止对单节点重复调用完整extractor。首次utilities的reference/重复删除在选项解析之前；source成功后才删除；custom variant先核对嵌套，再删除，之后验证名称/body/selector；legacy name赋值先于删除。

utility保持attached并继续初始下行，pending body保留原array anchor并读取其current view。media只在遇到时做它规定的targeted子walk，flatten后重访替换项。theme的子walk对Context透明，跳过keyframe body；整个theme验证成功后才产生第一个空rule替换或后续splice。收集token不等于给原property字段赋值。

初始成功才记录compatibility边界。compatibility之后按捕获顺序启用custom variants、执行functional utility归一化、发布最终theme nodes；随后既有variants/functions/apply/helpers消费同一冻结current view。lowering必须有明确完成边界，并由一个owner移交当前状态；不能让初始图与后期typed tree成为并行current authority。后期variants/normalization/functions/apply若还需改原对象字段，就须在同一owner操作或证明一次性状态移交，不能循环value→typed→value同步。该边界目前尚未通过；若反复lowering不可避免则scrap。private utility marker只在适当native派生产物引入，caller AtRoot仍不能选它。首个theme replacement、最终theme.nodes、keyframe在synthetic root的hoist是不同事件，不写入caller root的额外尾项、不清空原keyframe body。

任意语义失败返回已完成效果，不继续后续兄弟、不虚构最终theme写入或归一化。先完整准入，再由machine按reaction范围回放；较早host原异常优先于尚未回放的语义异常。不发布半个compiler。resource refusal丢弃整个draft，不授权AST写入；不将此保证扩大成getter无用户副作用。

## 迁移和下一步

局部fallible结构操作、捕获版本与统一初始dispatcher已接入。Registered raw pending handles已保留到compatibility之后，functional归一化在同一Current落地；原值theme nodes已在该owner发布，事后final_theme_mutations移除。authored variant已按捕获顺序逐项赋值并保留完成前缀，旧全量header预校验和typed递归变换移除。函数已移入原始Current，caller root与keyframe尾部顺序保持；theme只准备token输出值/依赖，不再重复替换keyframes。下一步收拢apply、完整synthetic root表示与统一lower root、definitions、variants及keyframes的交接；删除被取代的转换与后期重复authority。失败outcome、captured host投影、无损导入绑定和完整准入须一起保留。后期apply定位暂留其仍有职责的部分；不新增周期typed body刷新或第二份directive grammar。

最小复核顺序：18项cursor差分保持；已绿48场景/96实际WASM比较与媒体4项保持在默认compiler-api；保持ownership14场景/28比较在默认门通过，补赋值时观察/后续别名变化、原host异常、额度拒绝不写、导入槽位与成功5次build/no-op。再跑已有native/facade/xtask、全部API/UTF-16/216,133 CSS及同源完整CLI。原1398 ledger运行/断言/排除仍pending/0；检查范围不降级，M4不会仅凭本runner转绿就完成。

准入须在start前覆盖graph/index/cursor/context、provenance/COW历史版本、pending bodies、journal、生成值及失败保留、所有潜在import continuation/crossing handles、lowered native共存和normalization暂存。identity与storage去重标准不同，不能把逻辑owned bytes当allocator峰值；峰值与跨阶段动态Proxy仍有独立义务。

生产存储基础已开始实施：`crates/rustify-style/src/compiler/origin/current.rs`将current values与公开journal分离，数组版本关联原anchor并保留强owner；Edits与Snapshot使用这一存储，既有CloneGraph/Projection读取同一视图。新增7项原生检查覆盖旧版本共享读取、snapshot交接、版本/旧snapshot有限释放、edge重赋、body/metadata/Source fresh clone别名及历史版本费用。前三项在修复前实际失败。后续已加入有额度的assign/splice与赋值时descendants/Source深版本，9项新增原生检查通过，生产Edits assign传播检查结果；后续ImportViews已在生产消费assign/splice并设置局部cache_bytes上限，捕获改为仅复制变化容器且释放临时memo费用；后期Edits默认不限，初始注册已消费有界splice，旧insert/include仍接收已构造storage。局部操作预留不等于完整输入准入；captured host投影、完整注册与统一峰值仍待实施。验收入口见[M4当前存储记录](../M4.md)。

若projection需要重复directive判断、live utility body需要周期同步，或两个独立别名问题都需要额外例外，废弃此边界、重做Ground/Sketch。当前原型和生产存储检查没有触发scrap；原host写入时快照、无损输入/导入绑定、后期ownership交接与完整预算条件仍未通过，生产初始dispatcher虽已接入，后期ownership交接尚未完成。
