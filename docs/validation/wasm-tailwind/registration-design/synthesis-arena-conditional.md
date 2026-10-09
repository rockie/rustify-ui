# 初始注册：条件性选定结构与接入合同

> 历史条件性arena草案。后续结构探索与实际类型实验改为优先验证existing-value overlay，当前选择见[synthesis.md](synthesis.md)。本稿不代表生产采用；其旧类型/迁移草图保留作为对照。

计划仍是3/7，M4进行中。本文件选择下一步实现的内部边界，**没有交付生产修复**。[实际WASM RED](../utf16-families-registration-expanded-before-red.log)为45场景、90次比较、86失败；8项首错与82项前缀图差异有重叠，epoch差异为0。

## 调用方先看

`Compiler::new_with_ast`仍负责一次构造，不要求native调用方依次协调register、normalize、lower。内部注册owner接收导入完成后的当前输入，返回完整注册数据或语义失败及已经完成的效果；资源拒绝仍是外层Err。public adapter仅翻译效果，既有setup machine拥有ParseCss/Compatibility反应时刻和host反馈。

```rust
// 内部接口草图，未实现；不新增对外App/API。
fn prepare_initial(
    input: InitialInput,
    limits: &Limits,
) -> Result<InitialOutcome, ResourceRefusal> { todo!("not implemented") }

enum InitialOutcome {
    Ready(RegisteredInitial),
    Failed { error: CompileError, record: InitialRecord },
}
struct RegisteredInitial {
    current: InitialTree,
    registrations: Registrations,
    initial: InitialRecord,
}
struct InitialRecord {
    effects: Vec<InitialEffect>,
    values: EffectValues, // emitted values retain their then-current storage
    provenance: Provenance,
}
enum InitialEffect {
    Set { target: NodeId, field: Field, value: ValueVersion },
    Replace { target: ListId, index: usize, value: ReplacementVersion },
}
enum Field { Name, Params, Children }
// Native IDs are current identities; versions are immutable effect-value storage.
struct NodeId(usize);
struct ListId(usize);
struct ValueVersion(usize);
struct ReplacementVersion(usize);
struct InitialTree { nodes: Vec<NodeCell>, lists: Vec<ListCell>, root: ListId }
struct ListCell { current: Vec<NodeId>, binding: ListBinding }
struct NodeCell { current: InitialNode, binding: NodeBinding }
enum Body { Absent, Present(ListId) }
enum InitialValue { Text(Text), Undefined }
enum Slot<T> { Absent, Present(T) }
// ContextPatch must preserve admitted field presence; false/empty overwrite.
// Unrecognized raw fields/provenance remain opaque to directive semantics.
struct ContextPatch { fields: ContextFields }
// Deferred bodies keep a current ListId until postcompat registration.
struct PendingUtility { node: NodeId, body: ListId, name: UtilityName }
struct Cursor { list: ListId, index: usize, path: Vec<NodeId>, context: Context }
enum WalkAction { Continue, Skip, Stop }
struct VisitResult { action: WalkAction, replacement_count: Option<usize> }
```

这些类型是合同示意；未填充源码、没有声称已有可编译的完整注册接口。`InitialRecord`在失败时保留target和历史value storage，不能仅保存指向最后状态的ID。`EffectValues`必须区分原caller live handle与私有生成/COW版本，实际adapter写入仍用原对象/数组与原receiver splice；不能按最后图diff生成写入。

## 选择、修订与证据

采用[native候选](candidate-native.md)的领域效果边界，补入[graph候选](candidate-graph.md)明确的赋值时刻快照合同；[独立审查](review.md)双展示顺序都条件偏好这一边界。两案实际都收敛到canonical node/list及延后lowering，区别在转换/效果所有权；不声称穷尽两种不同完整架构、不声称跨模型胜出。全部上下文使用同一可用模型与共享文件系统，只有指令分隔；judge意外看到manifest映射已披露，不能称盲测。

[WHY约束](why.md)区分旧表示的条件性取舍与必须保留的语义。canonical图是当前设计推断，尚无性能/完整峰值优势证据。共享native领域层不直接依赖public `InputMutation`；已有后期函数/apply日志仍在其既有provenance/public边界，不能扩散到初始grammar。native构造与public setup共用同一handler；adapter不识别指令、不再次验证。

[原型](cursor-prototype.rs)仅验证cursor/当前拓扑，用固定上游`ast.ts`实际walk比较18场景；[恢复后结果](../utf16-families-registration-cursor-restored-differential.log)全部一致。覆盖共享数组删除、Context parent与false/empty override、0/1/2替换的Continue/Skip/Stop、嵌套Stop、visitor重赋children、延迟body读取、AtRoot parent和失败后不继续。三个错误版本分别在Stop优先级、丢false/empty、旧children读取处被抓住，见[cursor反证](cursor-mutations.json)。这是隔离native实验，**没有证明生产注册、Source/metadata、host/Proxy、资源峰值或任意导入器正确**。

`ast.ts:195–207`先处理replacement，之后才处理Stop。有replacement时Continue重访该index；Skip或Stop推进插入数量并继续兄弟，Stop不向外传播。无replacement的Stop才停止整个walk。原型捕获此差别；不要把普通Stop规则套在media targeted callback上。Context不调用visitor、不成为parent；普通AtRoot会成为parent。每次迭代与visitor之后的children读取都解析当前cell。

## 输入与导入边界

原始public输入必须在`Origins::convert`丢弃undefined声明、合并Context presence、复制body之前建立canonical node/list绑定，保留原数组槽位。现有lossy native Vec不能事后恢复这些事实；不能把`capture(Vec<AstNode>)`当成无损public入口。native parse输入建立同样domain cells，Absent body与Present空数组区分；public的结构准入范围沿用实际已支持契约，新增形态支持需另有差分，不能猜undefined/null都可无损。

导入仍由已有Bundle/Program/Machine完成。binding sidecar按实际occurrence更新原列表槽位、loaded current数组与生成context；不能使用lowered native数组下标反推caller位置。注册仅在import root成功settlement后开始，失败时不执行media/初始handler；已经启动的import jobs仍继续。移除`expand_with_base`末尾全树media处理后，media移到注册visitor实际遇到的位置，不改变begin/complete、队列、orphan与首次settlement。

sidecar与原始图如何接入现有`ImportViews`/public setup是**尚未通过的接口条件**。先证明undefined槽位、共享原数组、import replacement occurrence及原context身份，再迁移生产；不得新增第二个directive parser或用CSS/span猜身份。

## Handler与后期交接

初始owner按上游分支顺序处理utilities选择、utility capture、source、legacy variant转换、custom variant、media和theme。从现有source/theme/custom模块抽出不遍历的领域操作；禁止对单节点重复调用完整extractor。首次utilities的reference/重复删除在选项解析之前；source成功后才删除；custom variant先核对嵌套，再删除，之后验证名称/body/selector；legacy name赋值先于删除。

utility保持attached并继续初始下行，pending body保留当前ListId。media只在遇到时做它规定的targeted子walk，flatten后重访替换项。theme的子walk对Context透明，跳过keyframe body；整个theme验证成功后才产生第一个空rule替换或后续splice。收集token不等于给原property字段赋值。

初始成功才记录compatibility边界。compatibility之后按捕获顺序启用custom variants、执行functional utility归一化、发布最终theme nodes；随后既有variants/functions/apply/helpers消费同一冻结current view。成功lowering引入private utility marker，caller AtRoot仍不能选它。首个theme replacement、最终theme.nodes、keyframe在synthetic root的hoist是不同事件，不写入caller root的额外尾项、不清空原keyframe body。

任意语义失败返回已完成效果，不继续后续兄弟、不虚构最终theme写入或归一化。先完整准入，再由machine按reaction范围回放；较早host原异常优先于尚未回放的语义异常。不发布半个compiler。resource refusal丢弃整个draft，不授权AST写入；不将此保证扩大成getter无用户副作用。

## 迁移和下一步

先实现canonical输入/occurrence binding以及不遍历的source/custom-variant/theme操作，再补utilities/utility capture/media并替换一轮dispatcher；完整functional归一化延后，与失败outcome、effect snapshot投影和Compatibility ranges一起接入。删除被替代的eager media、分轮extract/select/header扫描和theme/media事后投影；后期apply定位暂留其仍有职责的部分。不保留两份互相同步的注册grammar。

最小复核顺序：18项cursor差分保持；45场景/90实际WASM比较转绿并加入默认compiler-api；补赋值时观察/后续别名变化、原host异常、额度拒绝不写、导入槽位与成功5次build/no-op。再跑已有native/facade/xtask、全部API/UTF-16/216,133 CSS及同源完整CLI。原1398 ledger运行/断言/排除仍pending/0；检查范围不降级，M4不会仅凭本runner转绿就完成。

准入须在start前覆盖graph/index/cursor/context、provenance/COW历史版本、pending bodies、journal、生成值及失败保留、所有潜在import continuation/crossing handles、lowered native共存和normalization暂存。identity与storage去重标准不同，不能把逻辑owned bytes当allocator峰值；峰值与跨阶段动态Proxy仍有独立义务。

若projection需要重复directive判断、live utility body需要周期同步，或两个独立别名问题都需要额外例外，废弃此边界、重做Ground/Sketch。当前cursor原型没有触发scrap；public conversion/effect版本/预算条件尚未通过，生产实施仍未开始。
