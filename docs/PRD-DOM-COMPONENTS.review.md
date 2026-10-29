# DOM-COMPONENTS · PRD 完整评审报告

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| PRD | [DOM-COMPONENTS · DOM 组件体系重构](PRD-DOM-COMPONENTS.md) | v0.1，2026-10-26；SHA-256：fd1bf351eba689d15e30018765eda57228cf257e8f8ef89495722512d3099bc4 | 全文：G1–G5、S1–S8、WORK-F1–F23、WORK-N1–N8、82 条 AC、UAT-1–14、P1–P5 |

调查日期：2026-10-26（Asia/Shanghai）。本轮为完整初审，不是代码验收或修复效果复审。原 PRD 与产品代码保持只读。

## 1. 确认问题

### [P2] F-01 · 区分 scope 清理、GPU 局部故障与实例终止的恢复边界

- **定位**：[WORK-N3/AC-3](PRD-DOM-COMPONENTS.md#L490) 将 scope 释放、浮层关闭、所属实例中止并列后，统一要求其他 scope/实例继续可用；[WORK-F19/AC-3](PRD-DOM-COMPONENTS.md#L405)、[WORK-N6/AC-2](PRD-DOM-COMPONENTS.md#L524) 与 [UAT-12](PRD-DOM-COMPONENTS.md#L639) 又未限定 GPU 失败/不可用的类别。同时，[§4 运行时约束](PRD-DOM-COMPONENTS.md#L556) 要求保持既有 instance/scope 生命周期。
- **场景与后果**：同一 wasm 实例挂载编辑壳、DOM 预览和 GPU 预览，GPU pump 发生 trap。现有契约要求该实例全部 scope 终止；只有其他独立实例继续运行。当前 AC 可被理解为同实例的其他 scope 仍须可用，且当前 Studio 仍能编辑、保存、导出。这会让符合既有约束的实现被判失败，或使下游误增实例内故障隔离范围。问题在产品验收口径，不在于现有代码尚未实现新能力。
- **已核实证据**：[架构说明第 22–24 行](architecture.md#L22) 明确 trap 终止该实例全部 scope、region 和任务；[原 Studio 计划第 354–355 行](plan/THEME-STUDIO.md#L354) 区分 GPU 字体/上下文故障与 wasm trap，后者只能重启恢复最近持久化草稿，未保存内存可能丢失。实际链路为 [GPU pump 捕获异常](../crates/rustify-makepad/web/embedded.js#L195) → [runtime fatal 中止监听、清空任务、销毁 regions](../crates/rustify-makepad/web/embedded.js#L367) → [Studio fatal 释放三个容器并提供重启入口](../examples/theme-studio/app.js#L13)；[loader](../web/loader.js#L87) 拒绝失败实例的后续调用。证据等级为静态行为链核实，本轮未注入故障运行。
- **最小修订**：分别定义三种结果：正常 scope/浮层清理不影响其他有效 scope；region/资源级 GPU 故障保留存活实例中的 DOM 编辑与导出；实例 fatal 终止该实例全部 scope，只保证其他独立实例继续可用，当前 Studio 进入 fatal/restart，重启恢复最近持久化草稿，并明确未保存输入可能丢失。同步 N3、F19、N6 和 UAT-12；无需新增 GPU trap 隔离方案。
- **复核条件**：修后原文对“同实例两个 scope，其中 GPU 发生 trap”和“仅 WebGL 上下文丢失”给出不同且可判断的状态、可继续操作及数据后果；不再无条件承诺 fatal 后继续编辑或导出。未来运行验收分别证明这些结果，本次改稿复核不替代运行证据。
- **置信度**：高，PRD、既有产品约束与实际调用链均已核对。**阻塞范围**：生命周期、GPU 降级和恢复语义的下游交接；不否定 DOM 分层、组件集合或五期产品方向。

本轮确认 1 项 P2；无 P0/P1 问题。未把尚待开发的三态选择、多滑块、无样式组合、新主题配置等目标记作缺陷。

## 2. 待核实与待落实项

除 F-01 外，没有发现必须先补外部事实或重新决定产品范围才能完成本稿的事项。以下保持为对应交付阶段的证据/实现前置，不能记为本轮已通过，也不据此额外阻塞 PRD。

| 项目 | 当前判断、所需材料 | 责任角色与节点 |
| --- | --- | --- |
| 新入口及新主题格式 | Q1/Q2 已明确由开发计划确定 Rust 命名、包边界和 schema；需以独立消费、往返不丢作者值与显式兼容损失证明结果 | SDK/主题维护者；对应开发计划定稿及 P1 主题交换交付 |
| SDK/CLI 配对 | Q3 与 F20/AC-3 已要求标明最低消费版本；当前没有新格式可供实际兼容核验 | 发行维护者；新格式对外提供前 |
| 完整稳定能力去向 | 固定参考集合已被 §2.3、N5/AC-2 和 §5.2 末段承接；交付时仍需逐项核对，包括 Theme 背景绘制控制、Primitive/Theme 的相应入口与声明状态 | SDK 维护者；各期能力说明及 P5 总验 |
| 来源的可重复使用 | 本地三份 ref 可读取且指纹复算一致，但不随 Git 分发；正式采用的数据/许可仍须按 N8 纳入交付。复算采用 Python pathlib.Path 分段路径排序，后续来源工具应固定该顺序，避免与整条路径字符串排序混用 | 实施者；P1 来源与干净 checkout 门 |
| 新界面与运行质量 | 尚无本次重构后的真实页面、辅助技术或性能结果；需按已有 AC 提供组件状态、响应式、模式、导出消费和热路径前后证据 | 实施/验收者；当期交付及相应终验期 |

没有可用的关联 SPMS 只读连接，本轮未核验平台项目或重复立项；PRD 已明确仅使用本地工作号，此限制不构成需求缺陷。

## 3. 代码现状核实矩阵

“已有满足”仅表示所述现状有源码依据，不表示本次重新运行验收。目标能力的实际交付仍须后续验证。

| PRD 主张/定位 | 类型 | 实际行为与证据等级 | 与目标的关系 | 产品影响 |
| --- | --- | --- | --- | --- |
| §1.1：CSR、SSR/hydrate 与纯 DOM 依赖 | 现状/约束 | [组件 manifest](../crates/rustify-components/Cargo.toml#L17) 分别映射三种模式，依赖名 rustify-ui 实际映射到 rustify-dom 并关闭默认 feature；源码核实 | 已有满足 | 模式保留是回归约束，不能把 GPU 变为新组件前置 |
| §1.1：24 类目录不等于全部公共入口 | 现状 | [CATALOG](../crates/rustify-components/src/catalog.rs#L181) 为 24 类；[公共导出](../crates/rustify-components/src/lib.rs#L54) 另含 Form、文件和工作区等入口；源码核实 | 已有满足 | F22/F23 保留扩展的范围合理 |
| §1.1：Dialog/Select 组合入口有限 | 现状/目标 | [Dialog](../crates/rustify-components/src/dialog.rs#L102) 固定绘制标题栏和关闭按钮；[Select](../crates/rustify-components/src/select.rs#L41) 接收选项集合并由[应用值决定显示及交付请求](../crates/rustify-components/src/select.rs#L86)；静态行为核实 | 部分满足；公开部件为真实增量 | 不能仅按同名组件宣布新组合能力完成 |
| §1.1：Checkbox 二态、Slider 单值 | 现状/目标 | [Checkbox](../crates/rustify-components/src/checkbox.rs#L19) 接收 bool，变化后恢复应用决定；[Slider](../crates/rustify-components/src/slider.rs#L11) 为单个 f64 与原生 range；静态行为核实 | 部分满足；三态/多滑块为真实增量 | F6/F7 的增量有事实基础 |
| §1.1：49 token、作者字符串与 sRGB 边界 | 现状/约束 | [token 集合与文档](../crates/rustify-dom/src/theme/document.rs#L5) 包含 35 个颜色及 14 个值 token，并保存双模式字符串；[颜色解析](../crates/rustify-dom/src/theme/color.rs#L37) 只接受所列函数，不接受 Display P3 输入，OKLCH 解析结果裁到 sRGB；源码核实 | 已有满足；新配置/格式为真实增量 | 独立 Colors P3 与共享主题默认 sRGB 的分工成立 |
| §1.1、F18：44 个预设 | 现状/延续目标 | [预设装载](../examples/theme-studio/src/presets.rs#L11) 将 42 个 vendor 项与两个默认项组合；本轮读取数据确认 42；第 42 行起已有 44 数量及双模式解析测试断言，未运行 | 已有满足 | 新快捷配置不能静默重生成旧作者数据 |
| F20/F21：导入候选、取消及本地恢复 | 现状/延续目标 | [导入预审和 Apply](../examples/theme-studio/src/data_tools.rs#L91) 先产生差异，当前主题发生变化会拒绝旧候选；[坏缓存/冲突入口](../examples/theme-studio/src/data_tools.rs#L555) 提供取回原数据、Load/Keep/Save copy；静态行为核实 | 已有机制；新格式迁移为真实增量 | 沿用既有闭环合理，新的快捷项/覆盖需纳入同一历史与保存结果 |
| F20/AC-2：256 KiB 输入限制 | 现状/约束 | [codec](../crates/rustify-dom/src/theme/codec.rs#L14) 定义字节限制，第 67 行检查大小，第 95 行起返回经两模式验证的候选；源码核实 | 已有满足 | 该数字不是未经依据的新门槛 |
| §1.1、F4：局部浮层与主题继承 | 现状/延续目标 | [overlay](../crates/rustify-dom/src/overlay.rs#L388) 只使本 scope 内容及低层 inert；[Layer](../crates/rustify-dom/src/overlay.rs#L725) 捕获创建处的有效主题并更新；[ThemeBoundary](../crates/rustify-dom/src/theme/provider.rs#L128) 解析局部 patch；静态行为核实 | 部分满足；新快捷选项/局部外观为增量 | 保持 scope 所有权正确；不能由此推导 scope 可独立承受实例 fatal |
| N2：每帧合并、无重复快照发布 | 延续目标 | [controller](../examples/theme-studio/src/controller.rs#L184) 合并待执行 frame，相同快照提前返回；[原计划 §7](plan/THEME-STUDIO.md#L337) 给出测量口径；源码与历史约束核实 | 部分满足；新负载表现未核实 | 没有本轮 p50/p95、产物尺寸或空闲绘制结果 |
| §1.5：三份参考版本和指纹 | 参考事实 | 本轮读取 package.json 并复算：Colors 3.0.0 / 20 文件、Primitives 1.8.1 / 703 文件、Themes 3.3.0 / 668 文件；摘要均等于 [PRD §1.5](PRD-DOM-COMPONENTS.md#L63) | 已有满足 | 固定的是本地快照，不是上游当前发行承诺 |
| F19/N3/N6：故障后仍可用 | 约束/目标 | GPU pump → runtime fatal → Studio 容器释放的调用链见 F-01；静态行为核实 | 行为边界冲突 | 需限定存活范围及可恢复的数据，不能扩大成新增 GPU 隔离项目 |

## 4. 覆盖、基线与产品质量

### 4.1 调查与执行边界

- 仓库：/Users/rockie/Documents/gh-rockie/rustify-ui；分支 main；HEAD：11b6912919a2a34c372aac4d6b7d51052820600f。
- 使用当前工作树。初始 dirty 为未跟踪 PRD、其 PRD 状态页/事件目录及三个已有台账索引改动；产品代码无未提交改动。评审结束前再次核对，HEAD、PRD hash 及所审产品代码未观察到并发变化。
- 约束来源：用户提供的 AGENTS.md、[README](../README.md)、[PRODUCT](../PRODUCT.md)、[DESIGN](../DESIGN.md)、当前 Cargo/mise/package 配置、[原 Studio 计划](plan/THEME-STUDIO.md) 与 PRD 引用。D1/D2 以本稿记载的已确认约束为本次边界，不重新扩张范围或推定额外授权。
- 实际采用 review-prd 工作流；3 个只读 subagent 先分别负责 facts、product、experience，experience 完成后复用该 agent 执行 acceptance。第四视角是另一次审查任务，不冒称第四位独立 reviewer。主 agent 逐条回读候选证据并单写综合报告。
- 已执行文档结构、AC 分期与 UAT 引用检查、50 处本地引用定位、三份参考目录指纹复算。未执行产品构建、单测、Playwright、辅助技术或性能测试，未修改原 PRD/代码、未调用外部业务写入。
- impeccable context 已读取现有 Studio 产品/设计规范；本次是 PRD 体验要求评估，未运行完整界面 critique、未审阅本次新增界面截图，未在浏览器中验证，也不给实际视觉评分。

| 审查维度 | 执行者与覆盖 | 实质结论 |
| --- | --- | --- |
| facts | /root/facts；模式、组件入口、主题、预设、参考快照、浮层及恢复链路 | 关键现状主张基本准确；故障范围需结合实例契约修订 |
| product | /root/product；分层采用、同源主题、兼容策略、范围与 AI 取舍 | 用户收益明确；没有必须追加的 AI 或外部服务需求 |
| experience | /root/experience 第一任务；四类角色、正常/失败旅程、视觉与响应式要求 | 文档层面的主路径和恢复路径充分；实际视觉与交互未验 |
| acceptance | /root/experience 第二任务；SMAR、82 条 AC、UAT、分期与终验 | 追踪结构闭合；交叉复核确认 F-01 是局部产品验收歧义 |

### 4.2 六个产品设计维度

| 维度 | 结论与依据 |
| --- | --- |
| 创新性 | 分层采用、可组合交互、同源主题创作与作者值保留直接降低采用及迁移成本；§1.4 与 F1/F3/F4/F20 有产品落点，不只是翻译组件名称 |
| 先进性 | 静态 Colors、无样式交互和 CSR/SSR/hydrate 并存，且保持无 React/默认无 GPU 的边界；参考能力和 Rustify 现状没有混为已交付 |
| 可用性 | 表单受控/拒绝、嵌套浮层、失败导入、坏缓存和跨标签处理有具体结果；F-01 修订后恢复边界可一致 |
| 便捷体验 | 快捷配置先于 token 精调，Inspector 能解释来源，局部解除覆盖与撤销有承接；不需要另加一个主题编辑站 |
| 美观 UI | §2.5、N7 与中性编辑壳、独立滚动、窄屏切换和真实组件状态一致；可评价要求合理，不能声称新界面已美观或对比度已达标 |
| AI 赋能 | 本地确定性的配色、编辑、迁移和导出没有必须依赖模型的环节；非目标与 §1.4 已给理由，排除运行时 AI 合理 |

已核对 [Radix 色阶用途说明](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale)、[WAI-ARIA APG 模式目录](https://www.w3.org/WAI/ARIA/apg/patterns/) 与 [WCAG 2.2](https://www.w3.org/TR/WCAG22/) 的官方原文。色阶用途不是任意配色组合合格的证明；组件需要适用的语义、键盘与质量判据。PRD 已保留这种区分，本轮不声称产品获得认证。

### 4.3 未采纳为缺陷的候选

- **旧预设覆盖遮住快捷色族**：§2.4、F4/AC-1/AC-3 已要求保留显式覆盖、分别解除并恢复继承，§2.5 要求标明有效值/来源/重置范围。可以按此设计可完成路径；缺少具体按钮形式不是 PRD 缺陷。
- **Theme 的 hasBackground 未进入快捷枚举**：[参考源码](../ref/themes-main/packages/radix-ui-themes/src/components/theme.props.tsx#L13) 确有背景绘制控制，但 PRD 已覆盖 Theme，N5/AC-2 与第 594 行禁止漏掉稳定公共能力。可由下游细化透明/显式背景行为，不因未逐字列举参数强加新缺陷。
- **质量终验在后期**：第 657 行明确所有期次执行当期适用质量要求；后期为完整范围终验，不等于 P1–P3 可跳过可访问性、隔离或部署质量。

## 5. 需求追踪与分期终验

### 5.1 SMAR 与双向追踪

| 检查 | 结果 |
| --- | --- |
| Specific | 角色、三层消费、DOM/GPU 职责、业务状态归属、稳定/unstable 范围明确；F-01 的故障作用域需修订 |
| Measurable | 大多数 AC 以可观察状态、相等/不变、枚举或明确边界判断；性能沿用原口径并要求前后测量，没有虚构已测时延或以目录存在代表验收 |
| Actionable | 开发者接入、维护者调主题、Studio 保存交换、最终使用者操作都有可完成任务及结果；不要求 PRD 提供技术模块表或测试脚本 |
| Reasonable | 无新增后台/账号/模型依赖，固定工具链和参考快照，兼容破坏与数据保留边界合理；实例生命周期须按 F-01 保持一致 |
| 目标 → 场景 → 需求 | G1–G5、S1–S8 均有 §6.2 承接；本轮未发现孤立目标或为凑目录扩大 GPU 范围 |
| AC → UAT | 23 条 FR + 8 条 NFR，共 82 条 AC；82/82 均有 §6.3 引用，未发现失效 AC 引用 |
| 正文 → 期次矩阵 → 终验 | 82 条 AC 的正文期次与 §7.2 一致；各条终验期及 §7.3 跨期登记一致 |
| 人工与自动证据 | N1/AC-3 和 §6.3 明确保留真实浏览器/辅助技术及缺失项；没有擅自增加真人招募或满意度门槛 |

本轮唯一确认断点是 S7 → F19/N3/N6 → UAT-12 中的故障存活及恢复语义。编号完整不能抵消这个语义问题；它也不要求重写其余已闭合的追踪关系。

### 5.2 各期独立价值与规模

| 期次 | 可独立交付的用户价值 | 前置与出口判断 |
| --- | --- | --- |
| P1 | 在原 Studio 创作并导出统一主题，独立消费颜色、按钮和排版 | 主题交换、旧数据迁移、来源与静态消费都有出口；实现包边界和格式留给计划合理 |
| P2 | 用无样式或主题入口完成可组合表单 | 依赖 P1 的主题契约；选择、字段、Select 及初始消费模式有验收 |
| P3 | 完成对话确认、浮层、内容切换、反馈与结构化内容任务 | 依赖组合/焦点基础；任务切分不以技术层代替用户价值 |
| P4 | 完整菜单/导航/工具栏及参考组件检查 | F1/F15 的累计终验在此；生命周期与恢复交接需先解决 F-01 |
| P5 | 原应用、工作区与外部消费者完成整体迁移 | F19/F23、扩展及全量质量收口；不能由 P1 的局部完成提前宣称整条交付 |

§7 已要求每期计划建议 5–8 个 Milestone、连续性例外 9–10 且最多 10。PRD 没有把全部工作压成一个技术计划，也没有通过分期删除终局组件集合；实际拆解能否满足预算由对应 dev-plan 核实。本轮没有依据仅按需求条数认定某期必然超额。

## 6. 非阻塞的产品增强机会

完整集合进入 Studio 后，可让能力目录直达对应组件及状态样本，并保留当前主题。价值是减少维护者查找指定变体的步骤；代价是持续维护目录与样本映射。若采纳，仅需在 F19/F23 增补可观察的定位行为。这是可选便利性，不是本轮通过的前提，也不要求新增平行预览站。

旧主题转为色族驱动的代表性示例也有帮助：保留原覆盖 → 显示覆盖来源 → 明确解除指定角色覆盖 → 色族生效 → 撤销恢复。该示例用于说明已经要求的能力，不新增功能承诺。

## 7. 总体结论与修订顺序

**是否需要修改：需要。建议结论：需修订。**

下一步命令：

> $apply-doc-review docs/PRD-DOM-COMPONENTS.review.md

仅 F-01 阻塞相关产品语义的交接。DOM 三层体系、固定稳定参考集合、保留扩展、单一主题闭环、破坏性 API 的迁移策略与五期用户价值均有充分设计依据；无需因本次评审追加 GPU 组件、运行时 AI 或外部服务。

修订顺序：先按既有实例契约区分 scope 清理、region/资源故障和实例 fatal；再同步 F19/N3/N6、S7/UAT-12 的状态、恢复动作与数据后果；最后复核 AC 与分期引用仍一致。范围与既有约束未变时，不需要重新决定包拆分或技术隔离方案。

解除条件是原文满足 F-01 的文档复核判据。修订后的 PRD 可再评审，随后作为开发/测试计划输入；评审通过也不表示功能已实现、性能已实测、UI 已通过浏览器验收或可以上线。本轮未自行启动改稿或开发计划。

---

## 修订处置与复核

- 来源：本报告原始版本，SHA-256 `2f8236b0a0b0bfb1c040f10b642f20df4729ac434c96dab22ae90f702f37d6b0`；原问题 ID 保持 F-01，原问题、证据、排除项与总体结论不改写。
- 目标：[PRD-DOM-COMPONENTS.md](PRD-DOM-COMPONENTS.md)；修前 v0.1 SHA-256 `fd1bf351eba689d15e30018765eda57228cf257e8f8ef89495722512d3099bc4`，修后 v0.2 SHA-256 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`。
- 复核日期与代码基线：2026-10-26（Asia/Shanghai）；main / `11b6912919a2a34c372aac4d6b7d51052820600f` / dirty（改动仅 PRD 与台账，产品代码未改）。
- 本次范围：全部；即 F-01。报告 §2 的待核实项默认为对应交付阶段的证据前置，未按缺陷处理；§4.3 与 §6 的未采纳候选、提案未纳入。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 区分 scope 清理、GPU 局部故障与实例终止的恢复边界 | 成立。本轮亲自复读：`docs/architecture.md:22`「A trap stops at the instance boundary. Everything in the instance it happened in is gone - every scope, every region, every task」；`docs/plan/THEME-STUDIO.md:354` 把「GPU 字体请求失败或上下文丢失」判为预览暂不可用、编辑文档仍在 DOM 应用状态，`:355` 把「wasm trap」判为同实例编辑壳与 GPU 都终止、重启后恢复最近持久化草稿；调用链为 `crates/rustify-makepad/web/embedded.js:43` 上下文丢失走 region 级回调、`:188–200` pump 异常走 `enter_fatal`（定义在 `:367`），`web/loader.js:89` 对 fatal 实例抛 `InstanceDead`，`examples/theme-studio/app.js:15–20` fatal 时释放三个容器并给出重启入口（提示恢复最近已保存草稿）。原 N3/AC-3、F19/AC-3、N6/AC-2、S7 与 UAT-12 把三类失败合并成「继续可用」。 | 已修订并复核 | 新增 `§2.7 故障与恢复边界`（正常清理 / GPU 局部故障 / 实例致命故障三行）；同步 `§1.4` 失败恢复、`S7`、`WORK-F19/AC-3`、`WORK-N3/AC-3`、`WORK-N6/AC-2`、`UAT-12`、`§4` 运行时契约行、`§6.2 G5` 行；版本 v0.1→v0.2。AC 数量、期次与终验期未变。 | 文档判据满足：同实例 GPU trap（终止本实例全部 scope、Studio 显示 fatal/restart、重启恢复最近持久化草稿、未保存输入可能丢失）与仅上下文丢失（DOM 查看/编辑/保存/导出继续，GPU 视图显示真实失败并可重新应用最新主题）给出不同状态与数据后果；原文不再无条件承诺 fatal 后继续编辑或导出。未注入故障运行，运行证据留在 UAT-12 与相应期次。 |

### 决策与未完项

无新增产品决策，也无待决项：修订沿用既有实例/scope 契约（本稿 D3、D7 与 §4 运行时契约），未新增 GPU 故障隔离范围，未调整 P1–P5、AC 期次与终验期，也未改动 Q1–Q4。报告 §2 的待核实项与 §4.3 未采纳候选保持原状，仍为对应交付阶段的证据/实现前置。

### 修订后结论与验证

- 计数：确认问题 1 项（F-01，P2），已修订并复核 1 项；不采纳 0，待决 0，未处理 0。原报告「需要修改 / 需修订」的历史结论保留；本处置只说明 F-01 的文档复核判据已满足，不冒称新一轮独立评审，也不代表功能已实现或已通过运行验收。
- 实际运行的检查：`git rev-parse HEAD`、`git status --porcelain`（基线核对）；PRD 修前/修后 SHA-256（`fd1bf351…` → `b857222e…`）；结构脚本核对 31 条需求 / 82 条 AC、每条 AC 均有 §6.3 UAT 引用、无悬空 `WORK-*/AC-*` 引用、§7.2 各需求 AC 计数与正文一致（0 处不符）、§7.3 11 条跨期登记与 §7.2 跨期集合一致；PRD 内 51 处本地链接全部可解析、4 处外链；回读 §2.7、三处 AC、S7、UAT-12、§4 与 §6.2 的改动位置。
- 未运行：产品构建、单测、Playwright/浏览器、辅助技术与性能验收；本轮不改代码，也不为文档修订启动实施或开发计划。
- 关联文档：`docs/architecture.md` 与 `docs/plan/THEME-STUDIO.md` 作为既有事实来源保持只读，其故障模式与本次修订一致；下游开发/测试计划需承接 §2.7 的三类边界。
