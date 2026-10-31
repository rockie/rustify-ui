# DOM-COMPONENTS-P3 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P3 · 可操作的 DOM 应用面板](DOM-COMPONENTS-P3.md) | v0.1；SHA-256 `836707836161a872b522e8d84e18415371446b7f18420b65b5f4ef3bddc821ca`；当前未提交工作树 | 全文；P3 的 12 条 AC；M1–M7、V-1–V-14 |

评审日期：2026-10-31（Asia/Shanghai）。代码基线：main / `614173ecc11354a7f5083bd0250849c90073141e`。开始评审时工作树 dirty，仅含本计划与其已有 SDLC 登记，无产品源码改动；本轮采用该工作树文档及 HEAD 源码。交付前再次核对 HEAD、计划 hash 与相关源码，未发生并发变化。本轮为首次完整评审，不是实施或修复效果复审。

## 1. 确认问题

1. **[P2] F-01 · 让局部模态的 ARIA 声明与实际交互边界一致**
   - 定位：[计划 §5.1](DOM-COMPONENTS-P3.md#51-组合层级定位与焦点):405、410–411，与 [§5.3](DOM-COMPONENTS-P3.md#53-dialog-与-alertdialog):435–436；关联 WORK-F11/AC-1、AC-2、WORK-N1，M3/M4。
   - 问题：计划明确要求对话打开期间，相邻 scope 和宿主仍可点击、聚焦，却要求 AlertDialog 输出 `aria-modal=true`，普通 modal Dialog 也未另行澄清。该属性向辅助技术表达对话外内容不可操作；部分辅助技术可能因此限制对话外内容的访问。按此方案，仍有效的宿主和其他 scope 对不同使用者具有不一致的可达范围，不能据此宣称与 APG 模态模式一致。
   - 证据：[PRD WORK-F11/AC-2](../PRD-DOM-COMPONENTS.md#work-f11-完成普通对话和警告确认):322 要求限制本 scope；[overlay.rs](../../crates/rustify-dom/src/overlay.rs):671–676 当前也只 inert 本 scope，:1243–1244 已有该属性投影，说明本期重组 Content 时需要明确处理。W3C 的 [Dialog 模式](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/#wai-aria-roles-states-and-properties)及 [AlertDialog 模式](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog/#wai-aria-roles-states-and-properties)均限定：对话外内容须对所有用户不可交互且被视觉遮蔽，才应声明该模态属性。官方页面于 2026-10-31 查阅；此处确认的是方案语义冲突，未声称已观察到某款读屏器的具体故障。
   - 修订：保留既定 scope 隔离，对局部 Dialog、AlertDialog 及 modal Popover 明确省略不成立的整页 `aria-modal=true`，保留正确角色、名称/描述、安全初始焦点和 scope 内约束；说明其与 APG 整页模态模式的适配差异。同步便捷入口/旧 Layer 在本期受影响路径的属性投影与 V-3/V-6 断言，不能通过 inert 整个宿主页修复。
   - 复核：改稿后，§5.1/§5.3/§5.4 的属性契约与 scope 外可交互要求一致；实现后 V-6 同时检查 Content 的 ARIA、scope 内底层约束、另一 scope 和宿主的指针及键盘可达性。实际读屏表现仍按已有人工证据边界记录。
   - 置信度：高；计划两处要求及官方属性适用条件均已核实。阻塞：M3 的公开模态语义定稿及其后续 V-6；不阻塞独立基线、切换和静态内容工作。

2. **[P2] F-02 · 补齐通知更换宿主时浮层父关系与存活规则**
   - 定位：[计划 §5.1](DOM-COMPONENTS-P3.md#51-组合层级定位与焦点):407–408、[§5.6](DOM-COMPONENTS-P3.md#56-反馈与通知):518，以及 [§9.3](DOM-COMPONENTS-P3.md#93-场景视觉矩阵与角色模拟):654；关联 WORK-F11/AC-3、WORK-F14/AC-2，T-2/T-3/T-7、M6/M7。
   - 问题：计划让 Layer 从创建 Owner 的 context 取得父层，又承诺 Toast Viewport 随 top modal 迁移 DOM、保留原 Root 节点及计时而不 remount。若通知先于 modal 创建，搬动 DOM 不会让通知内浮层的 Owner 自动继承新 modal。文本没有确定通知内后来打开的浮层在哪个时点绑定哪个有效父层，也没有确定已打开后代在通知换宿主时关闭、保留还是重绑。实施者仍须决定影响挂载、Tab 域、outside/Escape 顺序与回焦的公共行为；仅照“从创建 context 取 parent”实现不能保证已承诺的通知内嵌套任务。
   - 证据：[Cargo.lock](../../Cargo.lock):2265–2268 锁定 `reactive_graph 0.2.15`。该版本的 [Owner::new](https://docs.rs/reactive_graph/0.2.15/reactive_graph/owner/struct.Owner.html#method.new)在创建时记录 reactive parent；[context 源码](https://docs.rs/reactive_graph/0.2.15/src/reactive_graph/owner/context.rs.html#46-73)沿 Owner 图查找上下文，不查 DOM 祖先。本轮亲读同版本本地源码并核对官方文档。[现有 Select](../../crates/rustify-components/src/primitives/select.rs):254–282、471–473、517 也在构造时提供/读取 context。因而“DOM 迁移会自动改变层级”不能成为方案前提；这是机制推导，不是对尚未实现功能的失败实测。
   - 修订：在 §5.1/§5.6 明确通知有效父层由谁更新、后代注册时的绑定时点，以及换宿主时既存后代的处置和回焦规则；保留通知节点、业务 open 与剩余时间的既定契约。通知操作自身打开一个新 modal、Viewport 随之迁入该 modal 时，规则也须避免把该 modal 回绑为自身父层。可以用现有 Owner context 携带动态值实现，无须指定内部 Signal 类型或新增公共 API。把这份行为契约列为 T-2/T-3 向 T-7 提供的前置，并补进 V-11 的既有通知组合场景。
   - 复核：改稿后，新后代和既存后代的父关系、关闭与恢复规则均唯一可判定；实现后验证“先创建 Toast → 打开 modal → 从原 Toast 打开 Popover/Select → Tab/outside/Escape → 关闭 modal”，以及 Toast 内打开新 modal 的路径。核对正确后代域、无父子自环/无主层、回焦有效，通知节点及剩余时间按契约保留。
   - 置信度：中高；框架行为与两个计划契约确定，缺口在可观察迁移规则，具体未来实现尚不可实测。阻塞：T-7 的层级集成契约及 M6/M7 对应验收；应在共享 core 契约定稿时补齐。

共 2 项 P2；无 P0、P1、P3。相同根因的多视角发现已合并，未将新组件尚未实现计为问题。

## 2. 待核实与未来证据

没有需要用户额外决定产品范围、外部服务或发行方式的未决项。以下均是计划已安排的实施验证，不因本次缺少未来页面而新增开工阻塞；F-01/F-02 的契约修订另按确认问题处理。

| 项目 | 当前缺少的证据及影响 | 提供角色 / 最晚时点 |
| --- | --- | --- |
| U-01 实际组合行为 | 新父子挂载、局部模态几何、通知迁移、滚动拖动及清理的真实结果；源码推演不能证明通过 | 实施者；分别在 M4/V-6、M7/V-8–V-12 收口 |
| U-02 性能与资源 | 本期 before/after 页内 p50/p95、压缩体积和释放结果未知；不能将 P2 数据或功能增量当同功能对照 | 实施者；改旧行为前 M1/V-1，最终 M7/V-13 |
| U-03 视觉与辅助技术 | 本期无截图、实测对比度、触控、200% 缩放及实际朗读结果；不能给 UI 通过结论 | 实施者完成 M4/M7 的可执行矩阵；VoiceOver、OS IME、真机触控及未执行的真实 UI 缩放按计划保留 P5 missing，实际配合人待需要时指定 |
| U-04 公开接入可用性 | 新指南及 API 尚未交付，开发者只凭公开资料能否完成面板尚未实测 | 实施者；M7/V-14 的受约束角色模拟 |

正式参考的逐文件归档和许可是 M1 交付，不冒称已完成；当前已确认本地输入可读，不要求为评审下载或更新上游。

## 3. 覆盖摘要

实际使用 3 位只读 subagent：facts、architecture、schedule 同时调查；schedule 返回后，再单独执行 delivery 视角。主 agent 负责需求账本、原始证据回读、跨视角裁定、报告与台账单写。没有派生实施任务，也没有修改计划、源码或真实业务数据。

| 视角 / 负责人 | 覆盖与核实结论 | 限制 |
| --- | --- | --- |
| facts reviewer | 核对 PRD 12 AC、P2 恢复快照/R3 范围、Layer/Tabs/Tooltip/Toast/Progress/ScrollArea 现状、参考版本及构建入口；未发现独立事实问题 | 新设计不当作现状；参考视觉参数已核对主要 props，未运行未来代码 |
| architecture reviewer | 核对 OverlayStack/floating、单一通知调度、原生 scrollport、三模式消费及迁移；F-01/F-02 | 未实施 spike 或产品验收 |
| schedule reviewer | 需求到设计/任务/验收闭合，依赖无环，M1 风险门和 M4/M7 收口合理；无独立编排问题 | 命令/配置核实不等于未来测试通过 |
| delivery reviewer | 核对取得/配置 SDK、面板主任务、取消恢复、内容反馈、静态接管、响应式与键盘；独立发现 F-01 | UI 证据为文档推演 |
| 主 agent | 回读确认问题的计划/源码/锁定依赖证据，核对 W3C 官方页面及四视角结论，去重并限定修订范围 | 未在浏览器中验证本期 UI；不宣称具体 AT 故障已复现 |

主要事实锚点：

| 计划主张 | 已核实依据与判断 |
| --- | --- |
| P2 具体契约可复用 | [P2 恢复快照](DOM-COMPONENTS-P2.md#恢复快照):192–196 为 5/5；[R3 结论](DOM-COMPONENTS-P2.code-review.r3.md#总体结论与下一步):106–110 限于 CSR/SSG 修复范围。计划没有将其扩大为 P2 独立 UI 通过 |
| 浮层和反馈确有演进需求 | [overlay.rs](../../crates/rustify-dom/src/overlay.rs):45–84、671–676、1214–1219；[toast.rs](../../crates/rustify-dom/src/toast.rs):70–72、216、266–267；[progress.rs](../../crates/rustify-components/src/progress.rs):47–48。现状支持计划所述缺口，扩展既有 core 有实际调用方 |
| 消费与验收构建入口真实 | [xtask/src/docs.rs](../../xtask/src/docs.rs):255–257 复制 public，:957–990 构建两个 DOM 消费者、hydrate 和 component-site；:1001 起独立三模式。新增 spec 接线是计划任务，未误当已有测试 |
| 性能测量口径有来源 | [forms-performance.spec.ts](../../tests/dom-only/forms-performance.spec.ts):7–9、77–90 使用 5 次预热、40 样本、1440×900 与页内测时；计划未伪造本期性能值或新 SLO |

架构选择的适配与代价：扩展现有 OverlayStack 可集中真实嵌套复杂度，代价是旧 Layer/Region anchor 回归；floating 生命周期供多类部件共用，Themes 只投影外观；core 单调度器避免便捷通知和受控 Root 双重计时；原生 scrollport 加自绘条保留 wheel/touch/键盘，代价是尺寸、RTL、capture 和清理验证。这些路线可保留，无须新增 crate、服务、GPU 能力或更换依赖 Major。CSR/SSG/hydrate 分开构建及初态关闭 Portal 也符合现有静态契约。

UI/UX 采用 `impeccable` 的现有产品/设计上下文：本轮成功执行一次 `impeccable context --target docs/plan/DOM-COMPONENTS-P3.md`，并读取 PRODUCT/DESIGN 及相关 Operate 约束。没有改界面或重设计品牌。P3 尚无已实现的新面板，本轮未启动旧 UI 冒充新功能验收；证据等级为**计划文档推演，待实现后浏览器验收**。布局、对比度、真实焦点、手势与朗读均未在浏览器中验证。

结构检查实际结果：

| 检查 | 结果与人工裁定 |
| --- | --- |
| `.agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P3.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 检查 23 个路径 token，10 个 MISSING；均在 §1.2/§9/§10 明确为新增交付或可选新增拆分，包括 panels.css、3 类测试/fixture/指南及 baseline.json，没有确认错误复用 |
| `.agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P3.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 0/7、里程碑 7、完成记录 0、ERROR 0、WARN 1；告警来自已有未提交计划/台账，不是漏记已完成实施 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P3.md` | ERROR 0、WARN 20；均为上游 WORK-* 自定义 ID，已按 PRD 和账本人工核对 |

未运行产品单测、Clippy、build、性能或 Playwright；本轮无产品代码变更，以上脚本也不能证明未来实现或验收完成。未新建临时 worktree、分支、容器或常驻服务，无本轮运行资源待清理。

## 4. 映射与实施编排

本期需求来自 [PRD §7.2](../PRD-DOM-COMPONENTS.md#72-需求与-ac-分配):684–691，与计划 §0.1 的唯一完整映射一致；没有功能 AC 断链。

| 本期 AC | 设计 / 实现 | 收口检查 |
| --- | --- | --- |
| WORK-F10/AC-1、AC-2 | §5.2 / M2、T-1 | V-2 → V-6、V-11 |
| WORK-F11/AC-1–AC-3 | §5.1/§5.3 / M3、T-2/T-3 | V-3 → V-6、V-11；修订 F-01/F-02 的相关契约 |
| WORK-F12/AC-1、AC-2 | §5.1/§5.4 / M4、T-4 | V-4 → V-6、V-11 |
| WORK-F14/AC-1、AC-2 | §5.6 / M6、T-7 | V-7 → V-10、V-11；补 F-02 迁移场景 |
| WORK-F15/AC-3 | §5.7/§6 / M2–M6 | V-6、V-8、V-10、V-11 |
| WORK-F17/AC-1、AC-2 | §5.5 / M5、T-5/T-6 | V-5 → V-8、V-11 |

WORK-N1–N8 与 C-1–C-6 均有本期设计/验证落点；P4 导航/完整 Studio、P5 全库迁移及整体终验、公开发行仍明确排除，没有用本期局部完成认领跨期交付。F-01/F-02 是既有范围内的契约修订，不要求新增里程碑或重复映射表。

| 波次 / 任务 | 前置及解锁证据 | 可同时做的工作 | 写入/资源负责人 | 汇合验收 |
| --- | --- | --- | --- | --- |
| M1 基线 | 旧实现及 P2 具体契约；V-1 | 不并行改待比较旧行为 | 主集成人管基线/来源/接线 | before 与源码可取回后解锁组件重构 |
| T-1；T-2→T-3；T-5/T-6 | V-1；T-3 另需 T-2 core 签名/纯模型检查；先修 F-01 和共享 F-02 契约 | 切换、core、静态内容、滚动可在稳定输入下分文件推进 | lib/mod、CSS 导入、catalog、配置由主集成人独占；高内存检查串行 | V-2、V-3、V-5 |
| T-4；T-7 | T-2/T-3 稳定接口及 V-3；T-7 需包含 F-02 有效父层规则 | 辅助浮层与通知可分文件实施，T-7 不改 hover 资源 | core 契约由主集成人统一；端口/Playwright 串行 | M4 V-4/V-6；M6 V-7 |
| M7/T-8 | M2–M6、V-2–V-7、阶段一出口；V-12 先生成 V-9 产物 | 文档/fixture 可准备，最终验收统一串行 | 主集成人负责整体验收、证据复用、回写与提交 | V-8–V-14，收口全部 12 AC |

现有依赖无环；T-2→T-3 不是等待包含 T-3 的整个 V-3，而是其明确的 core 子成果检查。M5 可在阶段一独立开发。M1 是重构前的必要基线门，M4/M7 是两阶段收口；现有批内反馈、整批退出、证据复用和受影响范围补证安排成立。V-8/V-10 与 §9.4 已合并的 spec 不应再重复跑，V-9/V-14 重叠静态检查也按原计划合并。没有工期数据，本轮只判断拓扑和资源边界，不估算工期收益。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked。** 原计划自报 Ready、实施 0/7 保留。本轮建议与其不同，原因是 F-01 的局部模态公共语义及 F-02 的通知/浮层迁移规则仍需修订；这些会改变可访问性和关闭/回焦契约，不能留给实施者边做边决定。Blocked 限于受影响 core、模态和通知契约，不否定总体技术路线，也不阻止不依赖它们的 M1、M2 和 M5 工作。

修订顺序：先按 F-01 对齐局部模态属性与隔离边界；再按 F-02 定义通知有效父层、注册时点、迁移期间后代处置及回焦；随后同步 T-2/T-3→T-7 前置与 V-3/V-6/V-11 的针对性断言，核对相关正文并重新判定 Ready。无需为此新增产品能力、里程碑、服务或扩大 GPU 范围。

解除条件：两项可观察行为均在计划中唯一确定，相关公开 props/默认策略、失败语义、依赖和验收断言一致。文档修订复核可以解除计划就绪阻塞；实现与浏览器通过仍须按各里程碑另外取证。

下一步仅为建议，未自行启动：

```text
$apply-doc-review docs/plan/DOM-COMPONENTS-P3.review.md
```

本报告已保存并回读核验；原计划与产品代码未修改。本轮报告及 SDLC 登记未提交、未推送；所审计划版本当前仅原工作树可取回，hash 用于身份核对，不代表另有已共享快照。


## 修订处置与复核

- 来源：本报告原始 SHA-256 `a766de61c1d7f660662a17175ec8a4a521737c4558da60820500f71e4bc27018`；评审对象 ID `82810742-c017-4d8a-a8d0-8eb10b49b487`，原 F-01/F-02、所审基线及 §5 的原结论保留。
- 目标：[DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md)，修前 v0.1 / SHA-256 `836707836161a872b522e8d84e18415371446b7f18420b65b5f4ef3bddc821ca`；修后 v0.2 / SHA-256 `f11d67ea0529014d8384cd17f3d633fc552aa93f22d38cd5b5ba228b7ce6fbb8`。
- 复核日期与代码基线：2026-10-31（Asia/Shanghai）；main / dirty@`614173ecc11354a7f5083bd0250849c90073141e`。开始时已有未提交计划、报告及关联台账；本次仅修订这些文档/状态记录，产品源码未改。修前 hash 与报告所审 hash 一致，无此前处置，不覆盖其他工作树改动。
- 本次范围：全部 2 项确认问题；核对 U-01–U-04 的归属与时点。属于同轮改稿及文档层面的效果复核，没有重跑完整独立评审，也没有实施计划。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 本轮复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。PRD WORK-F11/AC-2 仅要求本 scope 模态；亲读 overlay.rs::refresh_inert 与 Layer 的 ARIA 投影，确认旧实现只 inert 本 scope 却输出 aria-modal=true。重新查阅 [APG Dialog](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/#wai-aria-roles-states-and-properties) 与 [AlertDialog](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog/#wai-aria-roles-states-and-properties)，该属性的条件包含禁止所有用户操作对话外内容，与本 SDK 保留宿主交互的契约不符 | 已修订并复核 | 计划 §3/§5.1/§5.3/§5.4 明确 Dialog、AlertDialog、modal Popover 及旧直接 Layer/便捷入口省略 aria-modal，保留角色/命名/安全焦点及 scope 约束；V-3/V-6、§9.3、M3/T-2/T-3 与 §11 同步 | 已解决（文档）。属性与边界一致，未扩大整页 inert；V-6 明列本 scope 底层约束、另一 scope/宿主真实点击和键盘可达。实际 ARIA/焦点与 AT 结果待实施取证 |
| F-02 | 成立。读完计划 §5.1/§5.6/§9.3，原文没有规定通知换宿主后的已有后代处置。Cargo.lock 锁定 reactive_graph 0.2.15；亲读本机同版本 src/owner.rs::Owner::new、src/owner/context.rs::with_context，以及 Select 的 Owner/context/Layer 入口，确认 Owner 父关系不是 DOM 祖先，不能靠搬动节点更新层级 | 已修订并复核 | 计划 §5.1/§5.6 定义 stack 更新动态父引用、实际注册时绑定、已打开通知子树重绑并保留、通知打开 modal 的祖先分支例外、宿主离开时的迁移/清理和有效回焦；§3/§8、V-3/V-7/V-11、§9.3、M3/M6、T-2/T-3→T-7 与 §11 同步 | 已解决（文档）。新/既存后代、迁移关闭顺序及防自环行为唯一确定，见下方静态推演。保留通知节点/open/计时；实际浏览器与资源结果待 M7/V-11 |
| §2：U-01–U-04 与映射/编排 | 均为原计划已安排的未来实施证据，不是当前文档修订的缺证。保留全部 12 AC、7 个里程碑、V-1–V-14；受影响前置和断言已接续到原检查 | 归属核对完成；未认领实现验收 | 计划 §0.1、§9、§10；U-01 的迁移组合增强到既有 V-11，U-02 性能、U-03 人工 missing、U-04 公开资料消费保持原时点 | 待未来实施验证；不计入本次 2 项文档关闭数量，实施仍 0/7 |

### 决策与未完项

沿用原 PRD 的 scope 隔离和通知连续性，没有新增范围或需要用户另拍板的产品选择。局部模态不通过限制整个宿主页来满足 ARIA；通知迁移选择“保留并重绑”，而非迁移时发关闭请求，以免中断通知内正在进行的任务。动态值仍通过既有 Owner context 传递，不规定具体 Signal 类型、不增加公共配置/API。

本次按修订后的契约逐项作静态推演，以下均不是实际运行结果：

| 场景 | 文档现在确定的行为与验收落点 |
| --- | --- |
| Toast 先于 modal 创建，之后从原 Toast 打开 Popover/Select | stack 更新同一个引用到 modal；Popover 在实际注册时绑定 modal，Select 绑定最近 Popover。Tab/outside/Escape 使用同一后代域；V-3 的模型及 V-11 的真实组合验证 |
| Toast 内 Popover/Select 已打开，再打开/关闭另一个 modal | 存活通知子树根随宿主重绑，内部 Popover→Select 关系不变；关闭宿主前先迁走该子树。保留节点、open、剩余时长及有效焦点，迁移不增加 auto_focus/关闭/宣告；自身失效仍按 §5.1 清理 |
| Toast 操作直接或经已有通知浮层打开 modal M | M 先绑定注册前的父层；包含 M 的通知子树祖先分支不回绑到 M，Viewport 可迁入 M，独立后代随后绑定 M，避免 Layer/DOM 自环。M 关闭后更新到下一宿主，再执行其清理/回焦 |
| scope dispose / instance abort | 结束所属通知与浮层资源，不迁往宿主页，不影响其他独立实例；仍由原 V-11/V-12 取证 |

无未采纳项、待决项或当前改稿遗留。U-01–U-04、正式参考归档及 VoiceOver/OS IME/真机触控/UI 缩放 missing 保留原实施归属；没有用本次文档处置替它们记通过。关联 PRD、架构说明和消费指南本次只读，无须改变上游产品承诺；实际行为/迁移说明的文档同步已纳入本计划 §3、M7/V-14。

### 修订后结论与验证

- 确认问题 2 项：已修订并复核 2，未采纳 0，待决/待核实/待修订 0。本轮文档效果均为“已解决”；整轮处置 `review-applied`，计划 **Ready / dev-plan-ready**。原评审“需要修改、建议 Blocked”的历史结论保留，实施 **0/7**，不是新一轮独立评审通过。
- 实际运行 `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P3.md /Users/rockie/Documents/gh-rockie/rustify-ui`：检查 23 个路径，10 个 MISSING；逐条核对均为 §1.2/§9/§10 的 ★新增交付或可选 placement.rs 拆分，没有错误复用路径。
- 实际运行 `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P3.md /Users/rockie/Documents/gh-rockie/rustify-ui`：0/7、7 个里程碑、完成记录 0、ERROR 0、WARN 1。脚本不能把未跟踪计划识别为已改 tracked 文件；告警对应原有未提交计划/台账，无产品实施或漏记完成。
- 实际运行 `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P3.md`：ERROR 0、WARN 20；为账本沿用上游 WORK-* 自定义 ID，受影响 WORK-F11/F12/F14 及 P3 的 12 AC 已对照 PRD §7.2/计划 §0.1 人工核对。
- 实际 `git diff --check` 通过；另核对未跟踪目标文档的修前/修后差异、链接、处置表及进度，原报告正文保持不变。末次微调澄清迁移失效边界、T-3 交付物/前置列，以及父子交互优先级与模态登记顺序的区别：已有后代重绑后仍先于父层处理，祖先 host 不得整体 inert 而禁用 top modal。不改变路径、ID 或里程碑结构，已回读相关契约，复用上述有效结构检查。
- 未运行产品单测、Clippy、构建、性能或 Playwright；**未在浏览器中验证**。这次只改文档，没有本期新实现可供验收；未来行为由各里程碑取证。未创建分支、worktree、容器或常驻服务。

下一步仅建议，未自行启动：

```text
/goal 按 docs/plan/DOM-COMPONENTS-P3.md 开发
```

计划、报告和关联台账当前未提交/未推送；修前 v0.1 未提交且未另存完整共享快照，旧 hash 仅作身份记录。原评审正文完整保留；修后文档 hash 用于版本核对，不代表已有共享归档。
