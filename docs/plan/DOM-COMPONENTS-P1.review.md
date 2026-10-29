# DOM-COMPONENTS-P1 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | SHA-256 `93d7cb156a67e4133fe3e0d5250583e9f2e924d8017431b97d233836f109126f` | 全文；P1 的 16 条需求、30 条 AC、M1–M7、V1–V10 |

调查日期：2026-10-26（Asia/Shanghai）。本轮为首次完整计划评审，不是 PRD 评审问题的修复复审。评审 ID：`606191c6-42f8-4ac4-a654-42b29bfa9e71`。

## 1. 确认问题

### [P2] F-01 · 明确异步保存的同页请求顺序及成功确认范围

- **定位**：[§3.2 持久化接缝](DOM-COMPONENTS-P1.md#L350)、[§3.3 锁内写入](DOM-COMPONENTS-P1.md#L358)、[V6](DOM-COMPONENTS-P1.md#L542)、[M5](DOM-COMPONENTS-P1.md#L606)；关联 WORK-F21/AC-1、AC-3、WORK-N6/AC-1。
- **问题**：计划定义了跨标签的存储版本比较，没有定义同一 Studio 内多次在途保存的排序、合并，以及保存完成时可以确认哪份 editor。例：另一页只持有同名锁而不修改存储，本页保存 A 等锁；等待期间用户编辑到 B 或撤销；释放锁后，存储比较仍可通过，却不能据此判断保存前的 editor 副本能否回写。照搬当前同步成功路径会回退新草稿、历史或 checkpoint；只去掉回写又不足以决定新草稿是否仍为未保存、何时继续持久化。实现者必须另做数据提交决策。
- **证据**：[controller.rs:534](../../examples/theme-studio/src/controller.rs#L534) 的 save_named 在保存前克隆 EditorState、应用身份并建立 checkpoint，再交给 commit_named；[同文件:613](../../examples/theme-studio/src/controller.rs#L613) 同步写成功后直接替换 envelope 和整个 editor。[EditorState:79](../../examples/theme-studio/src/state.rs#L79) 包含 revision、undo、redo、checkpoint、gesture 与 cache；[编辑及撤销入口:285](../../examples/theme-studio/src/controller.rs#L285) 会独立更新状态并再次保存。[Web Locks 规范 §3.2.1](https://w3c.github.io/web-locks/#api-lock-manager-request)规定锁请求异步调度；锁不管理应用的编辑状态（查询于 2026-10-26）。计划 §5.4 的发布 revision、§5.8 的导入 reviewed revision，以及 §12 的 Autosave 校验，均未定义这个跨等待的成功确认合同。
- **修订**：在 §3.3 选定同页单写者策略，区分可合并的 autosave 与不可丢弃的命名库动作；规定请求对应的操作身份、editor revision 和存储基线。若保留等待期间编辑，旧请求成功只能确认其实际保存的快照，不得替换更新后的 editor/history 或把新草稿标为已保存，并明确最新草稿如何继续保存、自身前次成功如何区别于外部冲突。也可明确冻结相关操作，但须说明等待与恢复行为。无需新建存储系统或里程碑。
- **复核**：改稿后能逐步推演“外页持锁 → 本页保存 → 本页编辑/撤销/再保存 → 释放锁”的确定结果。M5/V6 加延迟授锁、同页连续 autosave 与命名保存/删除交错的用例，验证草稿、undo/redo、checkpoint 不回退，库动作不丢失，保存反馈准确，最终草稿可持久化并重开恢复。
- **置信度**：高；确认的是同步复用前提变化及计划提交合同缺口，未声称未来实现已经发生数据丢失。**阻塞：M5 的持久化/controller 设计及依赖它的交付，不阻塞 M1–M4 独立工作。**

### [P2] F-02 · 为本地旧库升级定义确认、取消和后续写保护

- **定位**：[§3.3 读取与提交](DOM-COMPONENTS-P1.md#L357)、[§6 迁移待确认](DOM-COMPONENTS-P1.md#L498)、[V6](DOM-COMPONENTS-P1.md#L542)、[M5](DOM-COMPONENTS-P1.md#L606)；关联 WORK-F21/AC-2、WORK-N6/AC-1、UAT-3。
- **问题**：首次遇到有效 v1 库而不存在 v2 时，§3.3 从转换候选直接描述请求锁、备份和写 v2；§6 虽列出“迁移待确认”，没有规定确认/取消如何驱动提交、取消后保留哪份有效主题与历史，以及编辑、模式或偏好变化是否仍能触发升级。实现者仍须选择自动升级还是显式提交；缺少写保护还可能使用户取消后被 autosave 完成升级。
- **证据**：[PRD UAT-3:640](../PRD-DOM-COMPONENTS.md#L640)明确要求取消升级/导入不改主题、原数据可取回。现有 [controller.rs:87](../../examples/theme-studio/src/controller.rs#L87) 读到有效库即恢复草稿；[persist:423](../../examples/theme-studio/src/controller.rs#L423) 只有坏缓存、外部冲突和手势保护，没有可直接复用的待升级门。V6 列了导入 Apply、迁移成功及失败，没有明确本地库升级取消的断言；§9.3 角色路径也未补足这个状态转换。
- **修订**：仅对启动时的本地库升级补齐“完整验证候选 → 待确认 → 显式升级或取消”的合同；确定有效主题、候选、历史和持久化许可的关系。取消不提交 v2、不替换当前有效主题或历史，后续 autosave 不得绕过确认；保留内存编辑、导出、原文取回及重新发起入口。将确认门放在备份及 v2 写入前是简单实现，但不得提前备份不是上游硬要求。JSON 解码器仍可纯粹转换 v1，不要求每次解码弹确认。
- **复核**：改稿后明确每个状态的允许动作和迁移提交点。M5/V6 覆盖首次启动取消升级，随后编辑、切模式或修改偏好并重开，确认没有暗中提交；再显式升级，核对成功恢复、失败重试及原文仍可取回。
- **置信度**：高；上游取消结果明确，计划仅命名状态而未定义提交门。**阻塞：M5 迁移交互及验收合同，不阻塞 Colors 和主题内核等独立工作。**

### [P3] F-03 · 将性能原始证据放入仓库约定的验证目录

- **定位**：[§7 性能输出](DOM-COMPONENTS-P1.md#L513)、[§9.2 性能命令](DOM-COMPONENTS-P1.md#L581)。
- **问题与证据**：两处把前后测 JSON 写到 docs/plan/DOM-COMPONENTS-P1.records/；[AGENTS.md:233](../../AGENTS.md#L233) 要求里程碑证据放在 docs/validation/。性能文件是后续比较所依赖的原始证据，宜与验证资料归档位置一致；这不影响测量算法或本期就绪。
- **修订与复核**：仅把两份性能 JSON 的输出统一到 docs/validation/dom-components-p1/ 下的明确路径，里程碑 Markdown 记录继续按原恢复协议链接它们；检查命令、正文和记录引用一致，不覆盖旧 Theme Studio 证据。
- **置信度**：高；明确仓库约定与路径不一致。**阻塞：否，可选采纳。**

## 2. 待核实与实施期证据

F-01、F-02 需要补进计划；其余未运行事项已有相应实施/验收落点，不因尚未实现而另判设计阻塞。

| 项目 | 已知依据与仍需证据 | 责任角色 / 最晚确认点 |
| --- | --- | --- |
| 清除缓存与备份的并发覆盖 | 现有 [clear_cache:457](../../examples/theme-studio/src/controller.rs#L457) 经 [remove_storage:380](../../examples/theme-studio/src/persistence.rs#L380) 直接删键。计划“同一写入口”可合理覆盖删除，但实现时须核对显式坏库重置、remove_item 和备份清除是否也遵守锁/比较及成功反馈；以双页写入与清除竞争核验。不能只凭枚举未点名就断言未来仍无锁删除。 | Studio 持久化实现者；M5/V6 |
| 待锁期间 dispose/fatal | 浏览器 Promise 不被 runtime 自动停止；已有 [instance_failed 与 release_on_abort](../../crates/rustify-dom/src/listeners.rs#L44)，以及 [utility/browser.rs:199](../../crates/rustify-dom/src/utility/browser.rs#L199) 的回调 guard 可复用。检查请求取消、过期续体和失败后不再写库/更新已销毁状态；不是缺少新基础设施。[Web Locks §2.6、§3.2.1](https://w3c.github.io/web-locks/#termination-of-locks) 的页面终止清理与仅取消待授锁请求，不能等同 wasm fatal。 | 持久化实现者与主集成人；M5/V6、M7/V8 |
| 本机工具与真实测量 | 配置版本已核对；本轮未运行 doctor、产品构建、headed Chrome/GPU 或性能测量。按计划取新前后数据，不能引用旧通过数字代替。 | 实施者；M1 前测及 M7 后测 |
| 新 UI、响应式与独立消费 | 新参数、来源说明、导出包、DOM-only/Docs 及 SSR/hydrate 一致性均待实现后按 V3–V10 取证；人工读屏、OS IME 未运行须保留缺失。 | 主集成人；对应局部退出与 M7 |

## 3. 覆盖、基线与实际检查

### 3.1 调查基线及分工

- 代码：`11b6912919a2a34c372aac4d6b7d51052820600f`，main / dirty。计划、PRD、PRD 处置和相关台账是未提交输入；产品源码未发现相对于该 HEAD 的改动。仅 HEAD 无法复现未跟踪文档，当前结论须连同这些文件使用。
- 需求输入：[PRD v0.2](../PRD-DOM-COMPONENTS.md)，SHA-256 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`；[PRD 评审处置](../PRD-DOM-COMPONENTS.review.md)，SHA-256 `4456ac8b887487750a3dbf6f114ae733e89fb3c62b5a3e29bf0896aa865b4551`。
- 范围采用 DOM：Colors 静态资产、DOM 主题/基础组件、Studio 与 Docs 消费。现有 GPU 只承接共享主题及回归；没有扩展 GPU 对等组件、通用 Primitives、云端、门户或公开发布。
- 实际启动四个只读 reviewer：facts、architecture、schedule 先独立调查；槽位释放后以无历史上下文的新 delivery reviewer 审第四视角。facts 随后针对 F-01 做交叉复核。主 agent 回读所有确认问题的原文及行为级证据，独立裁定并单写本报告。

| 维度 / 负责人 | 已完成覆盖 | 结论 |
| --- | --- | --- |
| facts / facts | 主题 schema/codec/resolve/provider、Studio 存储与历史、参考属性/尺度、CLI 配对 | 未确认现状事实错误；跨视角复核 F-01 的同步复用前提 |
| architecture / architecture | Colors 分层、双模式 source、v2 数据、CSSOM/SSR、写锁与源码消费 | 主要路线可行；确认 F-01 异步提交接缝缺口 |
| schedule / schedule | P1 需求与 AC、检查/里程碑、恢复快照、风险门与并行资源 | 16 条需求/30 条 AC 映射完整；0/7 一致；F-03 为非阻塞目录建议 |
| delivery / delivery | 三角色主路径、升级/恢复、导出接入及 UI 状态 | 确认 F-02；其余路径在文档层闭合，实际 UI 未验 |
| 综合 / 主 agent | 全局矛盾、候选去重、脚本人工复核、结论与台账 | 两项 P2 分别属于异步提交和迁移批准，修订动作不同，保留独立编号 |

### 3.2 关键事实与技术选择

| 主张 / 选择 | 代码或规则依据 | 裁定 |
| --- | --- | --- |
| v1 为双模式作者表，未知字符串保留；新格式需版本化 | [document.rs:108](../../crates/rustify-dom/src/theme/document.rs#L108)、[codec.rs:47](../../crates/rustify-dom/src/theme/codec.rs#L47)、[codec.rs:93](../../crates/rustify-dom/src/theme/codec.rs#L93) | 当前事实准确；base/config/overrides 是明确新增设计，不是已实现宣称 |
| 双模式 source 取代仅 patch 当前快照 | [resolve.rs:249](../../crates/rustify-dom/src/theme/resolve.rs#L249) 旧 patched 把当前值同时写入 light/dark | 有真实必要性；计划同时纳入 provider 与 SSR 来源，路线保留 |
| 独立 Colors 与 sRGB/P3 分界 | PRD §2.4、计划 §5.1；本地四份 Colors TS 数据及 Themes 参考定义 | 静态 P3 与共享 sRGB 没有互相矛盾；vendor、许可、无 ref 构建和包内容检查已入任务 |
| CSR CSSOM、SSR 外部样式及响应式扩展 | [geometry.rs:55](../../crates/rustify-components/src/geometry.rs#L55)、[geometry_class:95](../../crates/rustify-components/src/geometry.rs#L95) | 已有收集/生命周期复用点；媒体规则及值校验是本期增量，不能据现有文件宣称全部支持 |
| 单文档、单历史与单帧发布 | [EditorState](../../examples/theme-studio/src/state.rs#L79)、[controller connect](../../examples/theme-studio/src/controller.rs#L184) | 继续扩展既有链路合理，无需另建主题编辑器；异步持久化仍须 F-01 |
| 保留旧发布记录、源码 CLI 验收 | [release.rs:109](../../crates/rustify-docs-cli/src/release.rs#L109) 同时检查版本、直接依赖及同 checkout canonical path | 计划使用源码模式可行，不应改成任意 Git 依赖或放宽 registry 配对；旧 0.2.1 不支持 v2 的说明正确 |

### 3.3 脚本与 UI 证据等级

| 实际执行（仓根为 cwd，根目录参数归一为 .） | 结果与人工判定 |
| --- | --- |
| bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P1.md . | 检查 25 个路径样式 token，9 个 MISSING。3 个为明确新增（Colors crate、colors.rs、迁移文档）；5 个为参考/CLI 的目录内短路径，已核到真实文件；另 1 个是 §12 明确禁止误用的旧主题目录。没有据此确认虚构复用路径。 |
| bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P1.md . | ERROR 0、WARN 1；0/7、7 行里程碑、0 行完成记录。警告由 dirty 工作树与未跟踪计划触发；人工确认都是既有文档/台账输入，没有遗漏实施完成记录。 |
| python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P1.md | **未通过：ERROR 72、WARN 0。** 脚本按固定表头识别主表，本计划紧凑表头不匹配，继而产生未定义 ID 连锁报告（识别逻辑见脚本 79–89 行）。已人工核对账本、V1–V10、M1–M7 及引用，未把解析格式不兼容当成 72 个实质缺陷，也未改稿迁就脚本。 |
| impeccable context 与设计原则核对 | 已运行 context，读取 PRODUCT、DESIGN、Studio surface brief 及 Operate 原则；只用于评审界面方案，没有运行完整 UI critique、视觉评分或重设计。 |
| 运行入口检查 | target 是失效符号链接，theme-studio release 目录不存在，4181 无监听；未为评审新建构建/服务。**未在浏览器中验证**，UI 证据仅为文档推演与静态源码核对，未审阅新实现截图。 |

未执行产品单测、Clippy、build、Playwright、故障注入、辅助技术或性能测试；这些是未来实现的验证，不是本轮计划评审的通过证据。计划与产品代码保持只读，报告及受影响台账是本轮写入范围。

## 4. 需求映射与实施编排

### 4.1 映射结论及用户主路径

[PRD §7.2](../PRD-DOM-COMPONENTS.md#L669) 的 P1 集合与 [计划 §0.1](DOM-COMPONENTS-P1.md#L193) 一致：WORK-F1/F2/F3/F4/F15/F16/F18/F20/F21/F23、WORK-N2/N3/N5/N6/N7/N8，共 16 条需求、30 条 AC。未发现漏登记、无来源任务，或把跨期条目整体提前记为完成。F-01、F-02 是已经登记需求内的行为/验收细化缺口；仅有 ID 映射不能替代这两项合同。

| 角色 | 已有闭环 | 本轮必要补充 |
| --- | --- | --- |
| Colors 消费者 | 公开说明 → 按需色族/模式/P3 资产 → 无 JS 静态消费与 sRGB 回退 → 多根隔离、来源/许可 | 无新增断链；保留 V1/V7/V8/V10 的真实接入检查 |
| 主题作者 | 预设 → 基础配置/覆盖解释 → 解除指定组/精调 → 真实预览 → 历史 → 保存、重开、导出 | F-01 的待锁编辑及保存反馈；F-02 的旧库升级确认/取消 |
| 独立 DOM 开发者 | JSON/CSS/Rust 导出 → 匹配源码/字体 → 独立 workspace → 模式/覆盖/根子路径与 sentinel 比较；旧 CLI 明确拒绝 v2 | 无新增断链；确切源码 SHA 及实际 build-info 在收口时填写，不预造发布版本 |

### 4.2 依赖、并行和集成门

现有里程碑无需重排。依赖主链为 M1 → M2 → M3/M4 → M5 → M6 → M7；M4 的静态部分可在所需 M2 契约稳定后与 M3 错峰，接入与浏览器验收仍依赖对应 provider 成果。按顶部协议，部分成果通过必要检查即可解锁下游，不要求增设里程碑或等待无关走查。

| 波次 | 前置与解锁证据 | 并行 / 共享责任与资源 | 汇合验收 |
| --- | --- | --- | --- |
| M1 | 工具/输入核对；Colors 数据、来源、字段矩阵及前测 | 数据生成与只读能力盘点可分工；Cargo/来源入口由集成人单写 | V1 数据/静态局部、V7 来源、V9 前测 |
| M2 | M1 数据/矩阵及必要检查 → v2/迁移/resolver | document/codec/resolve 统一模型后集成，不能分别定义格式 | V2 及旧 v1 回归 |
| M3 / M4 | M2 稳定接口 → source/provider 与基础组件；运行集成用对应 M3 成果 | lib.rs、catalog.rs、CSS 入口单一集成人；浏览器/GPU/同例构建串行 | V3、V4、V7 局部及必要 V8 GPU 共享值回归 |
| M5 | M2–M4 成果；**先落实 F-01、F-02 的数据/交互合同** | controller/state/persistence 由同一语义负责人集成；不让不同实现者各定迁移策略 | V5、V6 持久化/导入；新增同页异步与升级取消检查保留为本阶段风险门 |
| M6 | M3–M5 → 导出、独立 DOM/Docs、迁移/目录资料 | 独立 workspace 各自构建；Playwright 每次一个 project；源码配对不绕过检查 | V6 导出、V7、V8 消费者 |
| M7 | M1–M6 交付齐备 | 主集成人负责真实角色路径、console/请求、性能与故障结果，并单写计划回写 | V1–V10 阶段收口；未失效证据可引用，有变化才复验相应范围 |

M5 的迁移和真实双页竞争没有被推迟到最终才发现；M7 负责跨模块主路径及整期收口，不把局部通过当成整期验收。现有提交点及证据复用规则可保留。没有工期数据，本报告只给拓扑与资源边界，不估算耗时或并行收益比例。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked，范围为 M5 数据提交/迁移合同及依赖它们的整期就绪判断。** 计划自报 Proposed，表示只待文档评审；本轮确认两项会改变数据保存及取消语义的实施前置，不能把整份计划直接判 Ready。M1–M4 不依赖这些未定合同的工作具有独立可执行范围；本轮不自行启动实施。

先在 §3.3 定义 F-01 同页异步提交规则，再补 F-02 本地库升级确认/取消和写保护，并同步 §3.2、§6、V6、M5；F-03 可顺带修正证据路径。无需改动产品范围、重新划期、增加服务或另建编辑器。

解除条件是：两项场景从输入到候选、存储提交、当前草稿/历史、保存反馈均能按正文得到唯一结果；V6 有对应断言；状态/风险与正文一致。改稿复核可确认设计就绪，产品运行结果仍须在实现后按计划取证。

下一步由用户发起计划修订，本轮不自动改稿：

```text
$apply-doc-review docs/plan/DOM-COMPONENTS-P1.review.md
```

## 修订处置与复核

- 来源：本报告原始版本 SHA-256 `1eeb81c49c4ec4fb7a1c6d545c6865196e6f70a3558ad5577bb231d608196251`；评审 ID 仍为 `606191c6-42f8-4ac4-a654-42b29bfa9e71`。原问题、原结论、原审基线及引用均保留，上述行号属于修前版本，不作为当前修改坐标。
- 目标：[DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md)；修前 SHA-256 `93d7cb156a67e4133fe3e0d5250583e9f2e924d8017431b97d233836f109126f`（与所审版本一致），修后 SHA-256 `a5289ffe7a18f7dcdde9c3ab39369d635830b5fa9fa5feec19b27080d4d50b48`。
- 复核日期与代码基线：2026-10-26（Asia/Shanghai）；`11b6912919a2a34c372aac4d6b7d51052820600f` / main / dirty。输入计划、PRD、两份评审及相关台账原已未提交；本次只修计划、追加本节并同步受影响台账，产品源码与关联PRD保持只读。未提交文件及修前计划没有Git可取回版本，只能连同本地工作树/修前副本核对，不声明已跨机器共享。
- 本次范围：F-01–F-03及§2影响修订的清除/生命周期接缝；其余实施期证据保留原门槛。本次是同轮文档处置复核，不是新一轮独立评审。

| 来源 / 问题 | 上次处置 | 核实依据与最小修订 | 当前处置 | 修改位置 / 复核结果与剩余动作 |
| --- | --- | --- | --- | --- |
| 本报告 / F-01 | 首次处置；原为待修订 | 成立。亲读[save_named](../../examples/theme-studio/src/controller.rs#L534)、[commit_named](../../examples/theme-studio/src/controller.rs#L613)：现有同步路径预先克隆editor并在写成功后整体替换；[EditorState](../../examples/theme-studio/src/state.rs#L79)还拥有历史/checkpoint/gesture，文档revision不足以保护全部状态。选择同页单写者，连续未发出的autosave可合并，命名动作保序；以op_id、生命周期/编辑代次、请求快照和出队基线区分提交事实与当前编辑器。旧成功不替换editor，新草稿按队列继续保存，前次本页成功推进自身基线。 | 已修订并复核 | §0.2 D7、ADR-3、§3.2/3.3、§6/8、V6、§9.3、M5、§11/12同步。**已解决（文档层）**；延迟授锁及库动作交错的逐步结果见下表，真实运行留M5/V6，未记为测试通过。 |
| 本报告 / F-02 | 首次处置；原为待修订 | 成立。[PRD UAT-3](../PRD-DOM-COMPONENTS.md#L640)已要求取消升级不改主题、原数据可取回；[启动恢复](../../examples/theme-studio/src/controller.rs#L87)可沿用内存恢复，而[persist](../../examples/theme-studio/src/controller.rs#L423)没有升级许可门。补齐完整转换后待确认、取消持续写保护、确认绑定原文/当前草稿、失败重试、成功开放写入的状态表；确认门位于备份/提交前。取消不清历史，也不让后续编辑、模式或偏好变化偷升。 | 已修订并复核 | D7、ADR-3、§3.3、§6/8、V6、§9.3、M5、§11/12同步。**已解决（文档层）**；首次取消→继续内存操作→重开→重新确认的结果已可唯一推演；真实升级/失败/重开验收留M5/V6。 |
| 本报告 / F-03 | 首次处置；原为可选 | 成立并采纳。[AGENTS.md](../../AGENTS.md)「计划与证据」要求原始证据进docs/validation；亲读[性能spec](../../tests/browser/theme-performance.spec.ts)确认仍固定写旧目录。仅改计划中的新输出目的地，不改测试代码或旧证据。 | 已修订并复核 | §7及§9.2统一为 `docs/validation/dom-components-p1/performance-before.json` / `performance-after.json`，M1/M7记录继续链接JSON。**已解决（文档层）**；环境变量支持和两次真实测量仍属M1/M7增量。 |
| §2 · 清除缓存/备份及待锁dispose/fatal | 实施期待核实 | [clear_cache](../../examples/theme-studio/src/controller.rs#L457)经[remove_storage](../../examples/theme-studio/src/persistence.rs#L380)直接删键；亲读[listeners](../../crates/rustify-dom/src/listeners.rs#L44)及[回调guard](../../crates/rustify-dom/src/utility/browser.rs#L199)确认可复用存活检查/abort释放。将清除明确纳入同一写者/锁与确认比较；补取消待授锁、失效代次、授锁及续体guard。Web Locks的signal在授锁后不再负责取消，不能代替guard，见[规范§3.2.1](https://w3c.github.io/web-locks/#api-lock-manager-request)。 | 文档接缝已补；运行待核实 | §3.3、§8、V6、M5及§11；正常dispose/清除竞争在M5，fatal恢复结合V8在M7收口。没有把未运行事项关闭为产品通过，也未新增运行时设施。 |
| §2 · 工具/性能、新UI及独立消费 | 已有实施落点 | 仍需实施期证据；未以旧报告或静态代码代替实测。 | 待实施核实 | 原M1环境/前测、M3–M6局部与M7/V1–V10保持；不新增开工决策，不改变范围/AC/里程碑数量。 |

### 关键场景复核（静态推演）

| 输入与顺序 | 按修后§3.3得到的结果 |
| --- | --- |
| 外页仅持锁；本页保存A，等锁时编辑B并撤销/重做，再释放锁 | A仍按请求快照提交，当前编辑代次已变，成功不回写旧editor、历史或checkpoint；反馈先前版本成功/当前待保存。队尾保存最新已提交草稿，队列成功结束后重开恢复最终值。手势未结束时不持久化中间值，结束后补存。 |
| A在途，连续autosave B/C，中间夹命名保存或删除 | 只合并同一连续段的未发出autosave；命名动作是顺序边界，候选从本页最近确认库应用意图。每次自身成功推进基线，后续不会误判自身为外部冲突，也不会用旧整库快照复活已删项。外部真变化/写失败则保留全队列并暂停，逐项显式决定。 |
| 有效v1且无v2，恢复后取消，再编辑、切模式、改偏好、重开 | 取消瞬间不改主题、历史或checkpoint；取消后变化仅在内存，不写v2/备份或取消标记，也不积压可偷升的autosave。重开恢复原v1/偏好并再次待确认；原文下载与重新检查入口可达。 |
| 取消后重新Review并确认；待锁期间又编辑，或备份/提交失败 | 批准的是Review时完整旧库与当前草稿/偏好；确认时编辑已变则重新Review。待锁期间再变不被旧成功覆盖，成功后续写最新草稿；备份/提交失败仍写保护，不冒称成功，原文及已写备份保留，修复后显式重试。 |
| 清除与写入竞争；等待授锁时Owner清理或实例fatal | 清除范围在锁内又变则重新确认，清除成功后旧pending不得自动重建库。失效生命周期取消待锁且guard拒绝续写/访问旧signals；fatal只按既有重启恢复最近持久化草稿，不承诺未保存输入存活。 |

### 决策与未完项

- 无新增需用户裁决的产品范围/数据归属/外部契约。PRD的取消与失败保留语义、既有启动内存恢复和单一编辑历史继续有效；队列和升级许可门是既定Web Locks方案的本地实现细化。相比冻结整个编辑器，保留待锁期间编辑并以快照确认避免旧副本回写；没有引入新存储系统、服务或里程碑。
- 未采纳0、待决0；F-03虽非阻塞，也已采纳。PRD及其处置无需同步修改；产品代码、现有实施记录、旧性能证据保持只读。
- 清除竞争、dispose/fatal、工具与真实性能、新UI/响应式、独立DOM/Docs及SSR/hydrate仍需计划指定阶段实施取证；人工读屏/OS IME未运行须继续列缺失。本轮**未在浏览器中验证**，不为文档修订启动产品构建或服务。

### 修订后结论与验证

- F-01–F-03共3项均已修订并完成文档复核；0项拒绝、0项待决、0项未处理。原报告「需要修改 / 建议Blocked」是修前历史结论，保持原样；本次未进行新的独立评审。
- **当前计划Ready / dev-plan-ready**：两项M5数据/迁移合同已明确，相关V6断言、失败模式、里程碑、恢复快照和风险口径一致，无已证实的关键开工阻塞。实施仍为**尚未开始0/7**，完成记录占位和全部实施协议未改；Ready不表示产品实现、运行验收或发布完成。
- 实际运行：路径检查27个token、11个MISSING，人工逐项分类为5个计划新增/生成物（Colors crate、colors.rs、迁移文档、前后测JSON）、5个目录内短路径（已核对到真实参考/CLI文件）、1个明确禁止误用的旧主题路径，无虚构复用路径；进度检查ERROR 0、WARN 1，0/7、7行里程碑、0行完成记录，警告由未跟踪计划及原有dirty文档/台账触发，不是遗漏实施回写。
- 实际静态检查：P1账本原文未改，与PRD§7.2的16条需求/30条AC逐项一致；M1–M7和V1–V10定义无缺号/重复；实施进度节只更新当前阻塞文案；性能JSON目标路径一致；关联PRD及其处置与修前字节相同；`git diff --check`通过，计划/报告未跟踪部分另与修前副本比较。
- 最终回读：计划8处、追加处置后的报告49处本地文档链接均可达，无行尾空白；原报告全文逐字节保留为当前报告的前缀，仅追加一个独立处置节。计划diff只涉及本次合同、关联验证/风险、证据路径及就绪口径。
- `check_plan.py`本轮未运行：已读其79–89行，固定表头识别与本计划紧凑表头不兼容；用上述针对现有表格的结构/映射核对补足，未改模板迁就脚本，也不冒称该脚本通过。未运行产品单测、Clippy、build、Playwright、性能、故障注入或辅助技术验收。

下一步（仅建议，不自动执行）：

```text
/goal 按 docs/plan/DOM-COMPONENTS-P1.md 开发
```
