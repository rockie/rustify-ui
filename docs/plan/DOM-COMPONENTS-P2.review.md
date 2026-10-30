# DOM-COMPONENTS-P2 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P2 · 可组合的 DOM 表单](DOM-COMPONENTS-P2.md) | v0.1；SHA-256 `485c430bc5494fd7cf92098cff2fb3066423366cfa90582577decddd1ff08725` | 全文；M1–M5、T-1–T-8、V-1–V-11；P2 的 10 项需求、19 个 AC |

评审日期：2026-10-30（Asia/Shanghai）。独立评审 ID：`d2eed6a8-ae96-481f-b597-ddd75656fe17`。本轮为完整初评，不是 P1 或其他历史评审的修复复审。关联 PRD 是背景及需求依据，不是本轮自动修订对象。

## 1. 确认问题

### [P2] F-01 · 分开 RadioGroup 的内部组身份与可选业务 name

- **定位**：[计划 §5.2](DOM-COMPONENTS-P2.md)，第 388、394、396 行；关联 WORK-F6/AC-1、AC-3，T-3/M1。
- **问题**：第 394 行约定未提供 name 不参与 FormData，第 396 行却要求未命名 radio 组使用 scope ID，第 388 行又指定原生同名 radio。若把同一 scope ID 写入原生 name，同一 form 内的两个匿名组会合并为一个原生组，选择一组会取消另一组；两组都没有 form owner 时也存在该问题。即便将生成名改成每组唯一，只要它仍是非空原生 name，已选项就会进入 FormData，仍不满足“未提供 name 不提交”的合同。
- **代码与规则证据**：[现有 RadioGroup](../../crates/rustify-components/src/radio.rs) 第 45–59、102–106 行以 `id::next("radio")` 生成每组名称并写入原生 input；[组件 ID 分配器](../../crates/rustify-components/src/id.rs) 第 15–26 行和 [scope seeded 分配器](../../crates/rustify-dom/src/id.rs) 第 13–27 行已有每次调用唯一的身份机制，不能与共享 scope 名等同。HTML 规范将同树、相同 form owner、相同非空 name 的 radio 归为一组；具名且已选的 radio 会生成表单条目，没有内部生成名豁免。见 [Radio Button state](https://html.spec.whatwg.org/multipage/input.html#radio-button-state-(type=radio)) 和 [Constructing the entry list](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#constructing-the-entry-list)（2026-10-30 查阅，页面显示 2026-10-26 更新）。
- **实测**：主 agent 在现有 Playwright Chromium `153.0.8010.12` 的临时页面中放入同一 form、两个 fieldset，各含一个 `name="rui-scope"` 的 radio。初始 A 被选中；选择 B 后 A 被浏览器取消，FormData 中出现内部名称。实验没有调用 P2 代码，也没有改动仓库或用户数据；复现步骤与输出见 §3.4。
- **最小修订**：明确每个 Root 的内部唯一 ID、可选业务 name、实际原生 name 的用途及匿名组不提交的具体机制。若匿名原生控件省略 name，则由同一 Root 实现受控单选与键盘组行为；若保留内部原生分组名，必须说明普通原生 form、FormRoot 外及直接 `new FormData(form)` 时如何避免提交该内部键。只把“scope ID”改成“每组 ID”不能关闭第二个后果。
- **复核**：改稿后，§5.2 的分组、命名和成功控件规则应能同时成立，不留给实施者重新选择方案。实现后的 V-3/V-8 增加同 scope/同 form 两匿名组、无 form 两匿名组、两个不同 form 的显式同名组，以及 SSR/hydrate 身份保持；断言匿名组各自保留接受值且 FormData 无内部键，命名组仅输出约定接受值。
- **置信度**：高；计划内部冲突、现有实现、规范及隔离浏览器实验相互支持。**阻塞范围**：T-3 的 Radio/RadioGroup/RadioCards 命名与表单合同，必须在对应实施前修订；不阻塞独立的基础调查与 Slot/方向工作。

### [P2] F-02 · 把 TextField.Slot 自身的视觉参数纳入公共矩阵

- **定位**：[计划 §5.4、§5.6](DOM-COMPONENTS-P2.md)，第 425、468–479 行；关联 WORK-F15/AC-2，T-5/M3。
- **问题**：计划只为新 `text_field::Slot` 定义 Start/End，§5.6 所称“本期公共矩阵”中 TextField 行也只有 Root 的 size/variant/color/radius。参考中 Slot 自己公开的颜色、gap、水平及单侧 padding 没有对应接口和响应式规则。直接照该矩阵实施，会遗漏本期要求覆盖的稳定公共视觉能力；第 364 行“未列常规 props 保留”无法补足新部件原本不存在的接口。
- **需求与来源证据**：[PRD WORK-F15](../PRD-DOM-COMPONENTS.md) 第 363–368 行要求按各组件自身定义覆盖本地参考的稳定公共视觉参数。[本地 TextField props](../../ref/themes-main/packages/radix-ui-themes/src/components/text-field.props.tsx) 第 23–35 行的 `textFieldSlotPropDefs` 定义 side、color、gap、px、pl、pr；[TextField 实现](../../ref/themes-main/packages/radix-ui-themes/src/components/text-field.tsx) 第 92–113 行公开 Slot/SlotProps 并实际投影这些参数，不能归为未公开内部能力。参考 package.json 的本地版本为 `@radix-ui/themes 3.3.0`；这里引用的是用户提供的本地快照，不声称对应最新上游发行。
- **最小修订**：在 §5.4/§5.6 增加 Slot 的 color、gap、padding 与既有 P1 维度/Responsive 类型的映射、默认值及覆盖规则。允许采用 Rustify 的逻辑 start/end 命名；若替代物理 pl/pr，须写清 LTR/RTL 对应和组合优先级。不需要照搬 React 签名或新建通用样式系统。
- **复核**：改稿后矩阵、T-5 与 V-5/V-9 应覆盖该部件；实现后检查 Slot 独立颜色、gap/padding、响应式、RTL 和调用方 utility 覆盖，不能仅验证 Root 的外框尺寸。
- **置信度**：高；PRD、参考公开 props 和计划矩阵均已回读。**阻塞范围**：M3 的 TextField 部件接口与声明矩阵；其他组件无需因此改路线。

## 2. 待核实／待明确的合同

| ID | 已核实事实与缺少内容 | 影响／所需证据 | 责任与最晚确认点 |
| --- | --- | --- | --- |
| U-01 | 计划第 433 行只给 `on_asked: Callback<Submit>`；第 435 行允许普通原生 input 参与约束检查，第 442 行又不把 name 自动映射为业务字段。[Submit](../../crates/rustify-dom/src/form.rs) 第 57–69 行的 Blocked 只携带 `first_error: &'static str`；同文件第 83–105、238–272 行只认识预声明字段。[Form::carry_out](../../crates/rustify-components/src/form/provider.rs) 第 117–121 行按该字段的 control_id 回焦。尚不明确未使用 FieldBinding 的 `<input required>` 被阻塞时，如何报告结果、标识控件及决定焦点；外部 form 关联控件也需落在明确边界内。 | 请明确普通原生约束失败是否触发 on_asked；如触发，定义可表示该结果的身份/载荷；如不触发，明确它是 FormState 前置阻塞及其公开可观察方式。Root 可以先检查原生 validity 并阻止 save，所以本项**不足以证明非法输入必然被保存**，不计入确认问题。需要的是合同说明，不是现在实现新表单引擎。 | 计划作者／SDK 维护者，T-5 公共接口实施前明确；修订后定向复核 V-5/V-8 的未绑定及外部关联控件。 |

其余尚未取得的真实 P2 UI、SSR/hydrate、资源与性能结果属于计划已安排的未来实施验收，不当作新功能必须预先存在的开工条件。OS IME、VoiceOver、真机触控等限制已在计划第 592、594、619、659 行登记；本轮不把它们改成新增真人招募门。

## 3. 覆盖、基线与实际验证

### 3.1 调查基线

- 代码 HEAD：`721651e6e71906e046753dd258df5e7b527128e7`，分支 main；事实源为当前工作树。初始 dirty 仅含待评审计划和其 SDLC 登记，没有产品实现改动；计划头的“产品 clean”与此不冲突。
- 计划 v0.1 的 SHA-256 见评审对象；关联 [PRD v0.2](../PRD-DOM-COMPONENTS.md) 的 SHA-256 为 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`。所审计划尚未提交，hash 只用于识别本次工作树版本，不表示另一台机器已可取回该文档。
- 已读取适用 AGENTS.md、README、Cargo/mise/Node 配置、PRODUCT.md、DESIGN.md、相关 P1 恢复入口与源码；按当前 dev-plan 合同核对实施/进度、依赖和验证安排。当前没有对产品源码或原计划作修改。
- F-02 的本地参考 props 文件 SHA-256：`43836e10c22b7489a1481c10e1078f72d0e7405dd99775c07b4ca185c859d138`；对应实现文件：`9dbab6e4a6f1a0a5eab48feb0f90bd324ed32a7cb4c984ff1670b5f00c0563e5`。ref 仅为本轮可见调查材料；正式采用仍按计划 T-1 纳入来源与许可，不能成为构建依赖。

### 3.2 四视角分工及裁定

实际使用三个只读 reviewer：先并行 facts、architecture、schedule；schedule 完成后复用该 agent 独立检查 delivery，再对未绑定原生字段作定向交叉核对。主 agent 逐条回读候选问题的原计划和原始证据，单独写本报告。未把多个 reviewer 同意当成事实依据。

| 视角 | 负责人 | 覆盖与结果 | 限制 |
| --- | --- | --- | --- |
| facts | `/root/facts_review` | 全文关键事实、内部矛盾、参考公开参数；交回 F-01/F-02，经主 agent 复核保留 | 新接口尚未实现，不作运行通过声明 |
| architecture | `/root/architecture_review` | 模块边界、Slot/方向/Portal、choice/Slider/Form/Select、SSR/hydrate、CSP/清理；F-01 合并去重，U-01 保留待明确 | 没有编译新组件或运行产品 spike |
| schedule | `/root/schedule_review` 的首轮 | 10 项/19 AC 双向映射、依赖边、连续批次、局部/收口时点、现有命令入口；未发现额外编排问题 | 映射存在不代表 F-01/F-02 的具体设计已充分 |
| delivery | `/root/schedule_review` 的后续独立视角 | 公开资料接入、无样式/主题表单、提交/重置/失败恢复、Studio 入口、响应式/语言/可访问性；无额外确认问题，交叉核对 U-01 | UI 主要为文档推演和源码事实核查 |

主要现状主张的复核结果：

| 计划主张／类型 | 原始证据 | 核实结论 |
| --- | --- | --- |
| P1 已有契约可作为前置／事实 | [P1 恢复快照](DOM-COMPONENTS-P1.md#恢复快照)、[补齐计划恢复快照](DOM-COMPONENTS-P1-gaps.md#恢复快照)，均第 178 行 | 分别为 7/7、2/2；只取本期依赖的源码合同，不重新宣称历史人工/性能全通过 |
| 组件三模式分开，CSS 显式选择／事实 | [components manifest](../../crates/rustify-components/Cargo.toml) 第 15–30 行、[styles.rs](../../crates/rustify-components/src/styles.rs) | 依赖别名指向 DOM，样式为静态常量；支持同 crate 的 primitives 路线 |
| 无样式消费可不创建主题 provider／可行性判断 | [mount](../../crates/rustify-dom/src/mount.rs) 第 193–207 行、[DOM runtime CSS](../../crates/rustify-dom/web/dom-runtime.css) | mount 本身不注入 ThemeScope；最小 runtime 样式负责 scope/layer/fatal，未发现必须附带 Themes 皮肤的前提 |
| FormState 无字段值且集中 save／事实 | [FormState](../../crates/rustify-dom/src/form.rs) 第 9–15、238–272 行、[Form provider](../../crates/rustify-components/src/form/provider.rs) 第 75–86、117–123 行 | 主体复用成立，原生未绑定约束分支另见 U-01 |
| hydration 三模式独立构建／事实 | [build_hydration](../../xtask/src/docs.rs) 第 982–1065 行 | 分别检查 ssr/hydrate/csr 图并生成产品；P2 新样本仍须交付 |

主要技术选择可以保持：同 crate 模块避免新增分发闭包；显式 thumb ID 解决跨越后的身份稳定，符合 [APG multi-thumb 的固定 Tab 顺序](https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb/)；Select 显式 entries 支持闭合 SSR 的标签和表单值；有限 Layer/ScopePortal 增量服务 P2，不要求提前实施 P3 全部浮层能力。基线与当前使用相同有皮肤旧功能负载的性能比较也合理，没有把移除皮肤的体积差当成性能收益。

### 3.3 结构脚本

以下检查实际执行，未运行未来产品的验收命令：

| 实际命令 | 结果及人工解释 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P2.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 检查 23 个路径样式 token，9 个 MISSING；逐项对照 §1.2，均为明确 ★ 新增的 primitives 目录/CSS/vendor/fixture/spec/SSR 测试及证据目录，没有把未创建的交付物误报成虚构复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P2.md /Users/rockie/Documents/gh-rockie/rustify-ui` | ERROR 0、WARN 1；0/5、五行里程碑、零完成记录一致。告警来自工作树有未提交文档但计划未被 tracked diff 识别；已核对不存在未回写的产品实施 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P2.md` | ERROR 0、WARN 10，均为 WORK 自定义 ID 需人工核对；已对照 PRD §7.2 的 P2 分配，10 项/19 AC 相符 |

脚本不证明行为真实、接口完整或验收已经通过。未跑 Rust 全量测试、Clippy、产品构建、性能测量或 P2 Playwright suite；本轮不是实施验收。

### 3.4 浏览器与 UI 证据等级

F-01 的隔离实验使用已有 `@playwright/test` 和 Chromium，无服务、无新安装、无持久化页面。以 `page.setContent` 创建下列原生 HTML，再执行 `page.locator('#b').check()`；前后均读取两个 input 的 checked 及 `new FormData(form)`：

```html
<form id="demo">
  <fieldset id="group-a"><input id="a" type="radio" name="rui-scope" value="a" checked></fieldset>
  <fieldset id="group-b"><input id="b" type="radio" name="rui-scope" value="b"></fieldset>
</form>
```

```json
{"browser":"153.0.8010.12","before":{"checked":[["a",true],["b",false]],"data":[["rui-scope","a"]]},"after":{"checked":[["a",false],["b",true]],"data":[["rui-scope","b"]]}}
```

该结果只核实原生 radio 与 FormData 的行为，**不是 P2 UI 已在浏览器验收通过**。实验浏览器已关闭，没有保留预览进程。

UI/UX 按 impeccable 读取产品、设计与 Studio surface brief，执行了一次 context 加载，并从 Operate 任务检查层级、表单反馈、空/错误/禁用状态、方向、键盘、对比度、响应式和语言安排；没有启动独立的产品重设计或 UI 打分流程。主 agent 实际查看了计划引用的 [P1 历史截图](DOM-COMPONENTS-P1.ui-captures/r8/U5-dom-caller-1440-dark.png)，仅用于确认中性壳与作者预览分离的既有方向；已与 DESIGN.md 的 Scope Rule 和当前 Studio 源码核对，没有把旧图作为新组件或新交互证据。

**未在浏览器中验证 P2 产品 UI**：本期尚未实现，当前 target/dom、target/makepad-wasm-app 中没有可直接验收的产品页面；本轮没有为只读计划评审构建整套未来产物。计划已在 V-8/V-9 安排实际 UI、console/失败请求/CSP、焦点、390/768/桌面和资源清理验证，因此“尚无 P2 页面”本身不构成额外计划缺陷。

## 4. 需求映射与实施编排

### 4.1 本期映射

对照 [PRD §7.2](../PRD-DOM-COMPONENTS.md) 第 675–701 行与计划 §0.1，本期条目和 AC 数相符；没有缺失整条需求、将 P3–P5 移入 P2，或用局部完成提前关闭跨期条目。发现的是两处设计内容缺口，不是缺少更多映射表。

| 需求／AC | 设计 → 交付 | 现有验收 | 本轮裁定 |
| --- | --- | --- | --- |
| WORK-F1/AC-2 | §5.1、§5.6 → T-2、T-3–T-6 | V-2、V-8 | 无皮肤消费路线成立；整条终验仍为 P4 |
| WORK-F5/AC-1–AC-4 | §5.1、§5.5 → T-2、T-6 | V-2、V-6、V-8、V-9 | 组合、方向、Portal 和公开状态有去向 |
| WORK-F6/AC-1–AC-3 | §5.2 → T-3 | V-3、V-8、V-9 | F-01 需补正确的匿名分组/提交机制 |
| WORK-F7/AC-1–AC-2 | §5.3 → T-4 | V-4、V-8、V-9、V-11 | 稳定身份、排序与 commit/cancel 方案有定义 |
| WORK-F8/AC-1–AC-3 | §5.4 → T-5 | V-5、V-8、V-9 | 主路径有去向；未绑定原生约束结果见 U-01 |
| WORK-F9/AC-1–AC-2 | §5.5 → T-6 | V-6、V-8、V-9 | 组合、空/动态/长列表及关闭恢复有去向 |
| WORK-F15/AC-2 | §5.6 → T-3–T-6 | V-3–V-6、V-9、V-10 | F-02 需补 Slot 子部件矩阵；整条终验仍为 P4 |
| WORK-F19/AC-1 | §6 → T-7 | V-7、V-9 | Cards 中真实本地表单及状态样本有去向；整条终验 P5 |
| WORK-N1/AC-1 | §5–§7 → 对应控件与样本 | V-8、V-9 | 名称/状态/键盘/焦点/默认对比度有检查；不把扫描当辅助技术实测 |
| WORK-N4/AC-1 | §5.7、§9.3 → T-2、T-8 | V-8、V-10 | 三模式、纯 DOM 图/产物和接管有去向；整条终验 P5 |

SDK 主路径为“匹配源码与样式 → 无样式或 Themes 初始化 → 组合命名控件 → 接受/拒绝/延迟 → 提交/重置 → 错误定位/重试”；Studio 主路径为“Cards 表单区 → 真实输入 → 查看 FormData/本地保存状态 → 切模式/宽度/语言及状态样本”。计划 §3、§6、§9.4 已覆盖公开接入资料、可见结果与恢复。F-01 影响表单结果，F-02 影响部件定制范围；其余发现性、双语、窄屏、错误恢复和角色模拟没有额外确认缺口。发布、门户授权和业务后端不在范围内，不新增相应工作。

### 4.2 依赖与安全并行范围

依赖边可沿用：`C-1 → T-1`；`T-1 固定参考及旧产品基线 → T-2/V-2`；`T-2/V-2 → T-3、T-4、T-5`；`T-3/V-3 → T-5 选择镜像集成`；`T-5/V-5 → T-6 FieldBinding 接入`；`T-3–T-6 接口稳定 → T-7`；`T-7、V-2–V-7、同版产品 → T-8`。没有发现循环依赖，无需另造里程碑或重复映射。

| 波次 | 前置与解锁证据 | 可同时推进的内容 | 共享写入／资源负责人 | 汇合门 |
| --- | --- | --- | --- | --- |
| 修订前提 | 先处理 F-01/F-02，并明确 U-01 的合同边界 | 不依赖这些合同的参考整理与基础调查可独立推进 | 主计划维护者统一修订；本轮不启动实施 | 定向复核设计与 V-3/V-5/V-8/V-9 的断言 |
| T-1 → T-2 | C-1；改行为前 V-1 基线保存；随后 T-2/V-2 | 正式来源与基线准备可在单实施者内连续完成 | 来源锁、项目接线、lib.rs/CSS 导入由主实施者统一 | M1 基础与选择整批完成后 V-2/V-3；必要的前置证据可提前解锁 |
| T-3；T-4 与 T-5 | T-2/V-2；T-3 先消费修订后的 name 合同；T-5 的组接入需 T-3/V-3 | 有并行收益时 Slider 与 Field/Form 文件可分开，未满足组镜像前置只开发独立字段部分 | FormState/绑定、公共导出、CSS 生成由主实施者单写 | V-3、V-4、V-5 各在相应整批交付后运行 |
| T-6 | T-2/V-2、T-5/V-5 | 接口稳定后可准备 T-7 资料和样本，不提前记验收通过 | overlay 栈与 hooks 单人维护，避免 Dialog/Menu 兼容修改冲突 | V-6 |
| T-7 → T-8 | 全部公开接口及局部检查、同版 CSS/fixture | 主实施者集成样本、资料和目录 | 浏览器、构建身份、4190 端口与结果目录由主集成人串行调度 | V-7 后，M5 唯一阶段收口 V-8–V-11 |

计划的 M1–M4 局部退出、M5 集中终验、baseline-before-change 和失败后仅复验受影响范围安排合理。DOM-only、dom-primitives、baseline/current 复用端口必须串行；每条 Playwright 命令只选一个 project，不能为并行实现同时启动两个浏览器测试运行。共享文件没有稳定输入和单写负责人时不建议并行。

M5 仍由主实施者负责最终集成：同版源码/CSS/模式产物、原始与 Themes 消费者、Studio/既有嵌套、Docs islands、19 个 AC 逐行证据、资源与性能比较。保留现有阶段收口，不要求各里程碑重复全量浏览器或全仓检查。

## 5. 总体结论与修订顺序

**是否需要修改：需要。建议状态：Blocked。**

确认 2 个 P2，无 P0/P1/P3。计划自述 Ready，但 F-01 的 radio 分组身份与表单数据合同尚不能同时成立，F-02 的稳定公共视觉范围也未完整落到新部件接口；因此当前不能把所审范围整体判 Ready。Blocked 只表示计划需要先修订对应合同，不表示所有基础工作都不能做，也不是因新功能尚未实现或没有 P2 截图而阻塞。

修订顺序：

1. 先修 §5.2 的组身份／业务 name／实际原生控件策略，并把 F-01 的具体跨组及 FormData 断言补进 V-3/V-8。
2. 补 §5.4/§5.6 的 TextField.Slot 参数、默认及逻辑边缘规则，同步 V-5/V-9 和公开矩阵；同时明确 U-01 的原生约束失败回调边界，避免 T-5 实施时再定公共结果语义。
3. 定向复核受影响的 choice/Form/Slot 设计及验收映射，再判断 Ready；其余主要架构、五里程碑和集中终验安排可保持，不需要整份重写。

解除条件是上述设计和公开合同可一致执行、对应验证有确定断言，且 U-01 的分支边界已明确；不要求在文档修订阶段提前交付全部实现或性能数字。本轮没有修改原计划、产品实现、实施进度或历史证据，也没有启动后续 apply/goal、提交或发布。

下一步：

```text
$apply-doc-review docs/plan/DOM-COMPONENTS-P2.review.md
```

SDLC 按仓内 v3 协议登记本轮独立评审、待处置与计划文档评审事实，保留原文 Ready 和实施 0/5；评审建议 Blocked 与台账固定状态分别保存，不以文档评审替代功能验收。

## 修订处置与复核

- 来源：本报告完整初评 `d2eed6a8-ae96-481f-b597-ddd75656fe17`；原报告 SHA-256 `6044ccba81f744407b31a27a65366896f396ddd1f8351e96526064ec00900203`。原问题 ID、证据、所审基线与结论保持不变。
- 目标：[DOM-COMPONENTS-P2 · 可组合的 DOM 表单](DOM-COMPONENTS-P2.md)；修前 v0.1 / SHA-256 `485c430bc5494fd7cf92098cff2fb3066423366cfa90582577decddd1ff08725`，修后 v0.2 / SHA-256 `491fe21920067758af74d50ba6ac0b1be8cdcd56cba38b232b678f70ef789ada`。
- 复核日期与代码基线：2026-10-30；`721651e6e71906e046753dd258df5e7b527128e7`，main / dirty。dirty 范围为本次计划、报告及既有 SDLC 登记；没有产品代码改动。
- 本次范围：F-01、F-02 与影响公共合同的 U-01；关联 PRD 只读，未处理其他文档或实施任务。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。修前 §5.2 同时要求匿名不提交、原生同名及匿名使用共享 scope ID；现有 `radio.rs` 也证明内部 name 会进入原生分组/提交，且 allocator 本可提供每组独立身份。 | **已修订并复核** | 修后 §5.2 将独立 `group_id` 限定为焦点/ARIA/注册身份；显式业务 name 才写原生 name，匿名 Item 省略 name，由各自 Root 按接受值同步整组。V-3/V-8 与 §9.4 补同 form/无 form 匿名组、跨 form 同名组、直接 FormData 及 SSR/hydrate 断言。 | **已解决（文档层）**：分组、匿名不提交、受控拒绝/延迟与约束代理不再互相冲突；实现后仍须执行 V-3/V-8。 |
| F-02 | 成立。PRD WORK-F15/AC-2 要求覆盖各组件稳定公共视觉参数；本地公开 `textFieldSlotPropDefs` 的 color、gap、px、pl/pr 未进入修前矩阵。 | **已修订并复核** | 修后 §5.4/§5.6 为 Themes Slot 增加 `Responsive<Spacing>` 参数、独立 color、默认度量、物理 pl/pr 与逻辑 side 规则及覆盖优先级；同步 M3/T-5、V-5/V-9 和 §9.4。 | **已解决（文档层）**：接口、默认、LTR/RTL、响应式和 utility 覆盖均有确定合同；实际视觉效果留未来 V-9。 |
| U-01 | 合同缺口成立，不作为非法输入必然保存的缺陷。现有 `Submit::Blocked` 只接受静态字段键，不能表示未绑定原生控件；`Form::carry_out` 也按该键推导焦点 ID。 | **已修订并复核** | 修后 §5.4 明确按 `form.elements` 检查绑定、未绑定及外部关联控件。未绑定失败是 FormState 前置阻塞，不运行 rules/save、不调用 on_asked，聚焦实际控件；控件自身的原生 invalid 事件及 event.target/validationMessage 提供公开结果。V-5/V-8 同步分支与零调用断言。 | **已解决（文档层）**：结果、控件身份、焦点和 on_asked 边界明确，无需扩大 Submit 或新建表单引擎；真实控件行为留 V-8。 |

### 决策与未完项

无需新的用户拍板。修订采用三项最小技术选择：匿名 radio 不使用内部提交名；Slot 保留物理 px/pl/pr、仅以 Start/End 表达逻辑位置；未绑定原生约束不触发 `on_asked`，沿用控件自身的原生 invalid 观察通道。这些选择不改变 PRD 的产品范围或既有 FormState 字段身份。

关联 [PRD v0.2](../PRD-DOM-COMPONENTS.md) 未修改，SHA-256 仍为 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`。没有未采纳项、待决项或文档范围内的未完项；P2 实现、V-1—V-11 验收及跨期终验仍是未来工作，不能由本次改稿记为完成。

### 修订后结论与验证

F-01、F-02、U-01 共 3 项均已修订并复核，修后计划判 **Ready**；这是本次定向处置复核，不是重跑完整独立评审。计划顶部版本已更新为 v0.2，实施进度保持 0/5、尚未开始，五里程碑及完成记录未改。

实际检查：

- `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P2.md /Users/rockie/Documents/gh-rockie/rustify-ui`：检查 23 个路径样式 token，9 个 MISSING；逐项对照 §1.2，均为明确 ★ 新增交付物，没有虚构复用路径。
- `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P2.md /Users/rockie/Documents/gh-rockie/rustify-ui`：ERROR 0、WARN 1；0/5、五行里程碑、零完成记录一致。告警来自工作树有未提交文档而计划尚未被 Git 跟踪，不存在未回写实施。
- `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P2.md`：ERROR 0、WARN 10，均为 WORK 自定义 ID 需人工核对；已对照 PRD §7.2 的 P2 分配，10 项/19 AC 相符。
- 使用现有 Playwright Chromium `153.0.8010.12` 的隔离 `page.setContent` 页面验证：带 `novalidate` 的 form 含一个内部及一个外部关联 required input；`form.elements` 返回二者，`form.checkValidity()` 返回 false，两个实际控件分别派发 `invalid`，可读 `event.target` 对应 ID 与原生 validationMessage。实验未调用 P2 代码、未写入仓库，浏览器已关闭。

未运行 Rust 测试、Clippy、产品构建或 P2 Playwright suite，未在浏览器中验证 P2 产品 UI；这些属于计划实施验收，不是本次文档修订的替代证据。下一步可按修后计划进入实施，但不由本处置自动启动。
