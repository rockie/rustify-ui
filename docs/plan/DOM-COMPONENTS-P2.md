# DOM-COMPONENTS-P2 · Rustify UI · 可组合的 DOM 表单（Primitives 与 Themes 共用行为）

```text
::OBJECTIVE{implementation}
  target: 直接依据本计划、仓库规则和当前代码实施或续做
  ACCEPT: 全部里程碑退出检查通过，按「实施进度」完成最终提交、SHA 回填与自查，并提示用户发起代码评审
  实施须知: 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
```

> **计划状态：Ready**
>
> 版本：v0.2。
>
> 调查基线：2026-10-30 · `721651e6e71906e046753dd258df5e7b527128e7` · main / clean。代码事实来自该提交；计划及状态登记的新增文档不属于产品实现。
>
> 需求来源：[DOM-COMPONENTS PRD v0.2](../PRD-DOM-COMPONENTS.md) · **P2（第 2 期）**，承接 §7.2 的 10 个条目、19 个 AC；前置为 P1 的主题、样式与交换契约。
>
> 本期交付：**可独立使用的无样式 DOM 组合入口，以及共用交互实现的选择控件、选择卡片、单值/多值 Slider、字段/Form、Select；现有 Studio 增加真实表单样本。**
> **开发范围：本仓 rustify-components、rustify-dom 的表单/浮层必要增量、直接消费者与验证设施；呈现路径为 DOM。**
> 本期独特职责：把已有受控行为与品牌外观拆开，补齐组合、原生表单参与和输入完成/取消语义；继续消费 P1 的主题快照。
> **顶层排除：P3 的新面板/对话框/内容/反馈，P4 导航，P5 工作区与大数据重构及全量迁移；GPU 新组件、主题 schema/存储升级、发布和门户接入。** 既有 GPU 预览仅保留兼容回归。

## 实施者定位

```text
::CONSTRAINT{implementer}
  角色: 执行本计划的 agent 是资深软件工程师，依据本计划、仓库规则和当前代码实施。

::CONSTRAINT{development_scope}
  规则: 开发边界
  范围: 仅实施本计划明确纳入的系统与改动；范围外依赖按 §4 的契约、提供方与解除条件跟进，不代建、改造或部署。源码可访问、自有服务或同仓目录不扩大开发范围。
  [IF] 外部缺口阻塞 => [DO] 回写影响与继续条件，推进不依赖它的已授权工作
  [IF] 依赖到位 => [DO] 从当前阻塞移除，剩余项按本方未实现或验收未运行重新归因
  变更: 就地修订偏差和委派不扩大范围，新增开发对象须有用户明确依据。
  fixture: 只覆盖约定契约的必要行为，不能成为替代外部系统的产品，也不能冒充真实集成验收。

::CONSTRAINT{communication_and_source}
  规则: 表达与源码
  报告: 用路径、需求/检查/里程碑 ID 定位，不复述计划。
  源码: 不放需求编号、agent 标记或规划元数据，注释解释必要原因；追溯放记录或提交说明。

::CONSTRAINT{identifiers_and_tasks}
  规则: 编号与任务
  定义: 需求/约束/假设 ID 只在需求账本登记一次，上游 ID 保留原编号及来源；检查、里程碑、任务 ID 分别由验证表、里程碑表、按需内部任务表定义，正文、NFR 展开、映射和进度记录只引用。
  修订: 保留已有 ID，删除/拆分时同步引用。
  [IF] 任务表存在 => [DO] 按交付物、前置成果与检查 ID 推进，任务进展及证据写入所属里程碑记录，不另设任务状态表

::CONSTRAINT{milestone_completion}
  规则: 完成的定义
  完成: 本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口，不冒充通过。
  终验: 集中到阶段收口的终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收。

::CONSTRAINT{role_acceptance}
  规则: 角色验收
  模拟: 上手与业务场景由 Agent 按 §9 指定的知识、资料及权限约束模拟，不能借内部源码或实现者补步骤；记录实际结果与卡点，不把模拟结果写成真人数据。
  真实检查: 有工具和权限的真实环境检查由 Agent 执行；真人公测指标按其指定阶段另验，不冒充本阶段已通过。

::CONSTRAINT{implementation_route}
  规则: 按计划路线实现
  路线: 计划已定的技术路线、接口、复用锚点和检查命令直接照做，不重新调研，也不为已定决策另起方案设计、比较或评判；计划没写的实现细节按最近的同构代码就地决定。
  调查: 只读本里程碑的改动落点及其直接调用方和依赖。
  [IF] 代码事实推翻计划前提或路线走不通 => [DO] 停下，就地修订对应正文并在记录写明偏差；仅此条件触发路线重审

::FLOW{implementation_sequence|trigger:on_start}
  规则: 工作顺序
  [READ] 按 @resume_protocol 恢复，核对仓库规则、当前任务相关正文与依赖、工作树
  [DO] 按 §10 的连续实施批次推进，默认一个实现里程碑为一批，不按文件、函数、一次编辑或一轮 goal 对话切批
  [IF] 需要行为测试 => [DO] 用当前行为的最小用例先红、再绿、再重构，红绿循环不附带整组回归
  [DO] 完成本批相关实现和测试代码，再做局部退出检查
  [VERIFY] 下游所需前置、数据、安全、不可逆操作、发布及用户分期门及时验证，不等无关终验；稳定契约下可用 fixture 开发，不能替代真实集成
  [DO] 按 @verification_rhythm 与 @reverification 验证；批次通过后继续下一可执行批次，里程碑完成按 @progress_writeback 回写，汇报或回答状态询问后继续实施
  [IF] 用户明确暂停、取消或切换任务 => [DO] 按 @writeback_timing 保存后停止

::CONSTRAINT{verification_rhythm}
  规则: 验证节奏
  批内: 只为具体失败、技术疑问或必要前置运行能回答它的最小测试/编译检查；没有这些触发就继续实现。
  局部退出: §9 中整组局部检查（含受影响模块的回归和静态检查）等本批交付物齐备后统一运行，不能因命令叫“局部/快速”就每改一点重跑。
  收口: 全量测试、端到端、真实环境与界面走查、故障注入、评测、干净构建等只在指定阶段收口运行，同环境走查合并。
  成本: 优先复用增量构建、缓存和已就绪环境，不无故清缓存、重启栈或重复启动尚在运行的检查；调优用缩小版本迭代，达标后再跑完整检查。
  检查门: 仓库或用户明确指定的检查门仍须遵守。

::CONSTRAINT{reverification}
  规则: 复验时机
  失效: 证据仅在实际覆盖的代码、契约、配置、依赖或数据改变时失效；将受影响检查合并到下一适用批次退出或阶段收口，不因失效立即停工。
  合并: 同批连续改动只复验一次，未补证不能宣称新版通过。
  [IF] 下游要依赖该证据或即将跨风险门 => [VERIFY] 先补必要检查
  [IF] 检查失败 => [DO] 先定位原因，完成针对同一原因的修复 => [VERIFY] 失败项与修复波及的检查；未影响的已通过项直接引用
  [IF] 共享契约、公共入口、构建或依赖变化 => [DO] 核对波及面，必要时扩大复验；不能把所有局部修复都升级成整组或全量重跑
  复用: 续做、汇报、回写和提交本身不使证据失效。

::CONSTRAINT{parallel_delivery}
  规则: 并行交付
  条件: 独立任务在收益超过委派/集成成本且工具可用时使用 subagents，模型默认继承。
  任务说明: 只给目标/ID、稳定契约、可写与禁改路径、运行资源和检查；共享文件单人负责，数据库、端口、浏览器等隔离或串行。
  交付: 子 agent 返回结果、路径、验证证据与缺口，默认不另写任务文档；主 agent 审查并集成验收。
  异常: 边界变化先回报，失败只阻塞依赖任务。
  结束: 结束对话前收集或停止仍在运行的任务。
```

## 实施进度（实施期持续更新）

```text
::CONSTRAINT{progress_entry}
  入口: 本节是本计划跨对话恢复的唯一汇总入口。
  固定协议: 协议文字不随实施改写，实施期只更新「恢复快照」和「完成记录」。

::CONSTRAINT{resume_protocol}
  顺序: 续做时先读仓库规则、本节、当前任务相关正文与依赖，再核对 git status 和相关 diff。
  [IF] 记录与工作树不符 => [DO] 先查明原因，不凭记录覆盖用户改动，也不凭代码存在推断验收已通过
  按需读取: 只有继续未完任务、核验前置验收、排错或审查时，才按链接读取对应记录的相关部分；不要默认加载全部记录或原始日志。
  恢复: 从未完交付继续，不先例行重跑回归。
  检查前: 核对当前批次、触发原因与记录中的证据；未到验证点且无具体失败/疑问或必要前置就继续编码，已通过且覆盖范围未变的检查直接引用。

::CONSTRAINT{writeback_timing}
  规则: 回写时机
  [IF] 里程碑完成 => [DO] 立即按 @progress_writeback 回写，再报告完成或提交
  [IF] 下游只需部分成果 => [DO] 所需前置检查通过并在记录保存证据即可推进
  [IF] 实质进展后受阻、暂停、交接、发现偏差或结束对话 => [DO] 保存成果与缺口
  排除: 普通子任务切换不单独回写。

::CONSTRAINT{progress_writeback}
  规则: 一次回写
  顺序: 主 agent 更新对应记录，再同步快照变化字段和完成记录一行，不重写无变化内容或追加流水。
  快照: 写当前任务、进展与下一步，命令及证据只进记录。
  完成记录: 首次回写删占位行，每个已记进展的里程碑恰好一行，状态仅「进行中 / 阻塞 / 已完成」；完成时移至已完成行末，最近完成取该行。
  格式: 更新时间带时区，摘要 1–2 句，记录填相对计划的链接（如 [M1 记录](FEATURE.records/M1.md)）。
  定位: 回写在本节的恢复快照、完成记录及对应记录文件内定位，不按里程碑 ID、「状态」等同名字样对全文件查找替换；里程碑表、检查表、ADR 等定义行不随进度改动。
  [IF] 范围、决策、接口、数据、风险或退出条件有偏差 => [PATCH] 就地修订对应正文，不追加勘误历史
  [IF] 来源 CR 的承接请求或明确子范围已完成 => [DO] 按 @source_cr_writeback 核对并回写实现结果
  [DO] 回写后按 @project_index 同步已启用的台账
  [VERIFY] 每次回写后直接执行 @progress_consistency

::CONSTRAINT{source_cr_writeback}
  规则: 来源 CR 完成回写
  入口: 仅处理「需求来源」明确由本计划承接的 CR 请求；保留 CR/原 R/Q/RESP 语境，以既有需求/验证映射核对承接范围与收口检查，不因文档提及 CR 就改写。
  时点: 对应请求或明确子范围实现且约定检查通过后，在本次进度回写时追加结果，不必等无关里程碑；最后一个里程碑完成时再次核对有无漏回告。
  核对: 通读来源 CR，核对最新有效请求与往返答复、计划范围及适用验证证据；本计划仅承接部分或仍需其他计划交付时只回告已完成部分，列出剩余项，不关闭整条请求或整份 CR。
  写入: 复核实际回写文件的最新内容、保留并发增量，在文末追加实现结果 RESP，沿用编号并引用原请求和被更新的旧 RESP，说明哪些旧「未实现」判断被本轮取代；附实现行为、计划/里程碑、代码版本、验证结果、使用条件与剩余配合，不改正文或历史。
  边界: 本方实现、发布生效与对方验收分别陈述；无证据不标已发布或已获对方验收，不自动发送、部署或改平台状态，不外写敏感内部证据。
  去重: 同一请求范围、实现版本和验证结论已有实质等价回告时跳过，不重复追加或更新时间；有变化只追加受影响范围，不因无关 HEAD 变化记新轮。
  异常: 来源缺失、目标不唯一、只读或无法保真写回时，在对应记录保存可追加答复、原因和实际目标，恢复快照的下一步保留恢复动作；仅更新工作副本须说明原件未改，不宣称回写完成。
  留证: 写入后回读，确认范围、编号及历史完整，在对应里程碑记录保存实际 CR/RESP 链接；无需重新加载规划或答复技能。

::CONSTRAINT{milestone_records}
  规则: 独立记录
  路径: 按需建 <计划名>.records/M<n>.md。
  头部: 写计划链接、更新时区、状态及验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）。
  正文: 只写交付行为/路径、必要偏差、未完项与继续动作；未完项含当前批次剩余交付、受影响而需复验的检查及下次执行点，按回写时机保存，不每改一处记一笔。
  验收: 按「检查 ID/覆盖范围 | 命令或步骤 | 结果/必要证据」记录，共同环境和基线在头部写一次，有差异再单列。
  追溯: 同一测试组可覆盖多条退出条件，须逐条可追溯；引用原证据与复用依据，不复制断言全文、成功日志或调试流水。
  证据: 命令、结果和必要的少量输出，不为留证另写一次性脚本、哈希清单或大体量证据目录。
  缺口: 未运行、失败、环境缺失明确区分；只有复杂交接、排错或证据需独立保存时才另建任务记录/日志/截图并链接。

::CONSTRAINT{project_index}
  规则: 项目跟进索引
  启用: 仓库已登记状态台账，或存在仓根 .xgent-ai/sdlc/protocol.md 时启用；兼容已有 docs/project-status/protocol.md，两套并存未切换时先核对，不双写；未启用则跳过。
  同步: 上述回写完成后读取台账协议，只同步实际改变的当前状态和来源链接，必要时更新索引；默认直接更新本计划状态页。
  来源复用: 依据直接链接本计划的恢复快照/完成记录、相关里程碑记录中的验收或未完项章节、已有评审/处置记录及其版本；不重写成 SDLC event、过程总结或证据副本，不先造 event 再替代原记录作为来源。
  去重: 里程碑推进、验证结果更新、来源版本变化、最终提交和整份计划完成都不单独构成新建事件的理由。
  留存门槛: 新建前先核对原记录及其可取回版本；仅仓内明确要求不可变审计，或仍有无法由原记录与当前页保留的并发/纠错事实时独立留存，说明依据与必须保留的差异，其余直接引用。
  旧协议: 仓内旧协议强制事件时遵守其要求，只保留最小必要内容与原记录链接，不复述实施和验收过程；不自行升级协议。
  范围: 复用已有计划 ID，只更新本次触及的范围，保留其他计划和各轮评审，不复制本节及里程碑记录；台账仅作项目索引，不替代本节的恢复快照。
  跨机器: 使用仓根相对路径与明确基线，未提交成果标明同步限制；登记失败说明缺口，不将原成果回退。

::CONSTRAINT{commit_protocol}
  规则: 提交
  提交点: 在里程碑表后的「提交点」提交；里程碑较多时在标出的阶段收口后提交，全部里程碑完成后必须提交一次。
  优先约定: 仓库规则或用户对分支、提交信息、签名或是否由 agent 提交另有要求时从其要求。
  提交前: 先回写本次进度；提交点要求的检查就是本次覆盖的里程碑或阶段收口的检查，已通过且未失效的证据直接引用，不为提交重跑。
  暂存与钩子: 只暂存本计划相关改动，不纳入、不回退他人改动，不跳过提交钩子。
  提交后: 把短 SHA 写入完成记录中本次覆盖的各里程碑行的「提交」列，再单独提交这次只改计划文件的更新，该提交不回填自己的 SHA。
  修复提交: 代码评审修复的提交写作「修复」加短 SHA，UI 验收修复的提交写作「UI修复」加短 SHA，均追加在原 SHA 之后。
  [IF] 已完成但未到提交点 => [DO] 对应行写「待提交」
  [IF] 无法提交 => [DO] 写明原因并记入当前阻塞，不伪造 SHA

::CONSTRAINT{completion_handoff}
  规则: 全部完成
  [IF] 最后一个里程碑完成 => [DO] 按 @source_cr_writeback 核对来源 CR 回告 => [DO] 按 @commit_protocol 完成最终提交并回写 SHA，确认每个已完成行的「提交」都有 SHA => [DO] 恢复快照的「下一步」写代码评审；仍有 CR 回写缺口时一并保留
  报告: 随后向用户报告完成，并提示下一步用 review-code 以本计划路径为参数评审本计划；它从完成记录找到要审的提交，并纳入未提交改动。
  发起: 评审由用户发起，实施中不自行启动评审或修复评审意见。

::CHECK{progress_consistency}
  规则: 回写后自查
  OBSERVE: 每次回写后核对恢复快照、完成记录、记录链接及本计划文件的 git diff；非 Git 仓库对照回写前副本。
  PASS: 当前进度 n/N 的 N 等于里程碑总数，n 只计状态为「已完成」的行；最近完成对应最后一条已完成记录，不按编号推算；链接指向已落盘文件；已完成行的「提交」是短 SHA 或「待提交」，全部完成时每行都有 SHA 且没有「待提交」；本节以外只有有意修订的正文，定义行没有被进度文字改写。
  边界: 上述结构、计数与链接检查不证明验收真实通过。
```

### 恢复快照

- 最近更新：2026-10-31T00:30:39+08:00
- 当前进度：5/5 个里程碑完成
- 当前状态：M1–M5 已完成；R1 五项问题均已修复并验证，修复已提交，待独立复审
- 最近完成：M5 · 真实样本、资料与阶段收口
- 下一步：由用户发起 `review-code docs/plan/DOM-COMPONENTS-P2.md`；实施中不自行发起评审
- 当前阻塞：无；OS IME/VoiceOver/真机触控/浏览器 UI 缩放按记录留作 P5 missing
- 代码基线：`89ea6be2`（main；本轮修复，原实现 `9fd21ca8`；验证与复用边界见 M5 记录）

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-30T15:29:43+08:00 | 改前基线冻结；组合基础、受控选择控件/组/卡片、共用 Themes 和独立消费者已交付。V-1—V-3 通过，最新局部浏览器 6 项通过；整期验收留 M5。 | [M1 记录](DOM-COMPONENTS-P2.records/M1.md) | `9fd21ca8` |
| M2 | 已完成 | 2026-10-31T00:30:39+08:00 | 具名 Slider 交付保留；R1 F-03 以整数 step 序号修复，U-01 按用户决定明确，纯逻辑/raw 回归通过。 | [M2 记录](DOM-COMPONENTS-P2.records/M2.md) | `9fd21ca8`；修复 `89ea6be2` |
| M3 | 已完成 | 2026-10-31T00:30:39+08:00 | 字段/Form 原交付保留；R1 reset、实际 ID/legacy 聚焦和 Input/editor 生命周期已修复，55 SSR 与 raw/恢复增量通过。 | [M3 记录](DOM-COMPONENTS-P2.records/M3.md) | `9fd21ca8`；修复 `89ea6be2` |
| M4 | 已完成 | 2026-10-31T00:30:39+08:00 | Select 原交付保留；R1 缺上下文降级已修复，属性转发/取消与条件挂载、SSR/raw 回归通过。 | [M4 记录](DOM-COMPONENTS-P2.records/M4.md) | `9fd21ca8`；修复 `89ea6be2` |
| M5 | 已完成 | 2026-10-31T00:30:39+08:00 | R1 五项确认问题全部修复并验证，最终三模式、真实 DOM/Docs 消费者与性能取证闭合；人工 missing 仍留 P5，待独立复审。 | [M5 记录](DOM-COMPONENTS-P2.records/M5.md) | `9fd21ca8`；修复 `89ea6be2` |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

类型为既有模块重构与纯前端增量。以下需求 ID 沿用 PRD；AC 的唯一产品定义在来源正文。本期只结算列出的 AC；“本期覆盖”表示设计去向明确，完成事实以顶部记录为准。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F1 | 功能 | PRD §3.1、§7.2 P2 / AC-2 | 无样式消费者独立完成表单交互；整条终验 P4 | §2、§5.1；V-2、V-8 | 本期覆盖 AC-2 |
| WORK-F5 | 功能 | PRD §3.1、§7.2 P2 / AC-1—AC-4 | 单元素组合、方向、辅助部件、scope Portal 与稳定状态 | §5.1、§5.5；V-2、V-6、V-8、V-9 | 本期完整覆盖 |
| WORK-F6 | 功能 | PRD §3.1、§7.2 P2 / AC-1—AC-3 | 三态/单选/多选、Toggle 与选择卡片共用值和表单规则 | §5.2；V-3、V-8、V-9 | 本期完整覆盖 |
| WORK-F7 | 功能 | PRD §3.1、§7.2 P2 / AC-1—AC-2 | 多滑块、方向/间隔、连续请求与完成/取消 | §5.3；V-4、V-8、V-9、V-11 | 本期完整覆盖 |
| WORK-F8 | 功能 | PRD §3.1、§7.2 P2 / AC-1—AC-3 | 原生字段/表单、Draft、校验与失败重试 | §5.4；V-5、V-8、V-9 | 本期完整覆盖 |
| WORK-F9 | 功能 | PRD §3.1、§7.2 P2 / AC-1—AC-2 | 可组合 Select、分组、字符定位、动态/空/长列表 | §5.5；V-6、V-8、V-9 | 本期完整覆盖 |
| WORK-F15 | 功能 | PRD §3.1、§7.2 P2 / AC-2 | 表单选择/输入的逐组件视觉参数及 utility 覆盖；整条终验 P4 | §5.6；V-3—V-6、V-9、V-10 | 本期覆盖 AC-2 |
| WORK-F19 | 功能 | PRD §3.1、§7.2 P2 / AC-1 | 已交付基础组件有真实状态样本；整条终验 P5 | §6；V-7、V-9 | 本期覆盖 AC-1 |
| WORK-N1 | 可访问性 | PRD §3.2、§7.2 P2 / AC-1 | 基础控件名称/状态/键盘/可见焦点与默认对比度；整条终验 P5 | §5、§6、§7；V-8、V-9 | 本期覆盖 AC-1 |
| WORK-N4 | 兼容性 | PRD §3.2、§7.2 P2 / AC-1 | CSR、SSR、hydrate 分别消费；无 Makepad、确定接管；整条终验 P5 | §3、§4、§5.7；V-8、V-10 | 本期覆盖 AC-1 |
| C-1 | 前置 | PRD §7.1；P1 恢复快照及 §4 所列源码 | 使用已交付的 v2 作者文档/快照、作用域样式；不以公开 0.2.1 代替源码配对 | §1.1、§4；V-1、V-10 | 已核实足够 |
| C-2 | 运行契约 | AGENTS.md「运行时契约」；docs/architecture.md | 显示值由应用持有；Draft 例外有界；监听随实例 abort，资源随 scope/部件释放 | §5.1—§5.5、§8；V-8、V-9 | 已确认 |
| C-3 | 性能 | AGENTS.md「性能」及 PRD §4/Q4 | 改动热路径须有同条件前后 p50/p95 与压缩体积；新能力增量分列，不增造毫秒 SLO | §7；V-1、V-11 | 已确认；数值待测 |
| C-4 | 来源/构建 | AGENTS.md「编码与样式约定」；PRD §1.5、§4 | ref 仅调查；采纳的输入/许可正式入库；生成 CSS 与目录同步，单份应用 CSS | §1.2、§5.6；V-2、V-10 | 已确认 |
| C-5 | 工具/资源 | mise.toml、package.json、AGENTS.md | mbx 编译测试；模式分开；单项目串行 Playwright；无新服务/工具 major/自动安装 | §9；V-8—V-11 | 已确认 |
| C-6 | 迁移/边界 | PRD §1.3、§5.1 D1—D3、§7.2；AGENTS.md | 允许必要 API 变化，提供可用替代；只迁移本期触及入口，不双轨实现或扩建 GPU | §0.5、§3；V-7、V-10 | 已确认 |
| C-7 | UI/隔离 | PRODUCT.md、DESIGN.md；PRD §2.5、§7.1 | 编辑壳保持中性，作者主题留在预览；适用质量门当期执行，不能等跨期终验 | §6、§7；V-8—V-10 | 已确认 |

没有依赖未证实产品假设的 A 项。浏览器产物需要按 §9 构建，不把历史截图或前期验收当作 P2 已通过。

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 分发边界 | 在现有组件 crate 内增加 `primitives` 公共模块，根入口继续提供 Themes | 不新建 crate；无需主题 provider/皮肤即可使用 Primitives，但仍依赖 DOM runtime | WORK-F1；现有 feature 与 CSS 显式导出模式 |
| D2 | 组合表达 | 原生类型约束的 Slot + 部件组件；行为在所属 Root 的冒泡边界组合 | 不克隆 AnyView、不猜 children 数量、不叠加交互元素；调用方事件先执行 | WORK-F5；`layout::Reset`、受控事件范式 |
| D3 | 表单策略 | 原生成功控件或接受值镜像；FormRoot 只桥接既有 FormState | 应用持值，浏览器参与 FormData；reset 是应用请求，不保留 defaultValue 通道 | WORK-F6、WORK-F8、C-2 |
| D4 | Slider | 共用一套状态计算与指针/键盘实现；多值携带稳定 thumb id | 返回值按数值排序，焦点/Tab 顺序按身份保持；标量 Slider 是单项投影 | WORK-F7；§0.3 ADR-2 |
| D5 | Select 数据 | Root 持有受控值、开放状态与显式选项元数据；部件独立组合 | 闭合 SSR 也能显示标签和原生表单值，不靠未挂载 Portal 注册选项 | WORK-F9、WORK-N4；§5.5 |
| D6 | 视觉 | 每个组件自己的 size/variant 类型；颜色/半径/响应式复用 P1 模型 | 无全局 variant 大枚举；Themes 只投影样式，交互仍在 Primitives | WORK-F15；本地参考 props |
| D7 | 展示 | 扩展 Cards 现有场景，增加表单样本区 | 不新增持久化 scene 枚举，不改变主题文档/存储格式 | WORK-F19；现有 `scenes::CardsScene`、preferences 校验 |

### 0.3 ADR-lite

#### ADR-1：无样式入口放在哪里

- 状态：Accepted（本计划技术决策）。
- 驱动：WORK-F1、WORK-N4；需要独立行为消费，也需要让既有应用和 CSS 接入保持局部迁移。
- 候选 A：新建 `rustify-primitives` crate，Themes 依赖它。可获得 Cargo 级边界，但需迁移 ID、字段类型、表单 wiring、geometry 与 feature/发行闭包，增加本期分发和兼容成本。
- 候选 B：现有 crate 的 `primitives` 模块；独立导出最小行为样式，Themes 包装同一部件。Cargo 图不隔离整个主题内核，但消费者不加载皮肤/品牌 token，不创建主题状态；链接器按实际可达代码处理。
- 决策：B。PRD 要求消费与行为边界，没有要求独立 Cargo 包；当前皮肤是调用方显式读取/链接的静态资源，没有导入组件即注入 CSS 的机制。
- 正面后果：沿用已验证的 csr/ssr/hydrate 分发，消除两份行为，局部迁移即可验证独立采用。
- 代价：模块边界需要依赖检查与无皮肤消费者持续约束；不承诺仅依赖 Primitives 后 Cargo 图也不包含主题 Rust 模块。
- 重评触发：用户要求独立版本/许可边界，或 V-11 证明不可达 Themes 仍显著进入无样式 WASM 且链接层无法消除时，再拆包；不预建 adapter。

#### ADR-2：多滑块的数值顺序与焦点身份

- 状态：Accepted（本计划技术决策）。
- 驱动：WORK-F7 要求默认跨越和排序，WORK-N1 要求操作中焦点可继续。APG 要求多滑块跨越后 Tab 顺序保持稳定，见[多滑块模式](https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb/)。
- 候选 A：仅暴露 `Vec<f64>`，组件内部维护值排名到 DOM 的映射。签名接近参考，但应用拒绝/延迟/改写请求、相同值和动态增删时需要推测身份。
- 候选 B：多值使用 `ThumbValue { id: String, value: f64 }`，部件按 id 绑定；请求按值排序，元素按首次声明顺序挂载。调用方多维护一个稳定键，身份与排序不再含混。
- 决策：B；现有标量入口保留 `f64`，内部固定单个 id。`RangeSlider`/部件入口使用具名多值，不移植 React 签名。
- 正面后果：拒绝值不会交换焦点身份；重复值、跨越、动态移除都能确定回焦与提交对象。
- 代价：迁移多值状态需要显式键；文档和示例必须同时展示排序结果与固定部件顺序。
- 重评触发：真实调用方无法稳定提供键且已有无歧义的替代身份协议时再评估，不能退化成按数组下标重建 thumb。

### 0.4 职责与事实所有权

| 事实/职责 | 应用与 Studio | Primitives / DOM 内核 | Themes |
| --- | --- | --- | --- |
| 拥有 | value/checked/open、业务校验/保存、主题作者文档与接受/拒绝策略 | 临时焦点/手势、原生事件桥接、表单 bookkeeping、scope 资源清理 | 已解析主题到部件 class/视觉参数的投影 |
| 不拥有 | 不直接改组件私有焦点状态 | 不缓存一份显示业务值，不做业务后端，不接管宿主全局主题 | 不重做选择/关闭/校验状态机，不保存第二份主题 |
| 失败恢复 | 决定请求是否接受、错误与重试 | 终止取消手势、释放失效节点/监听/定位；只影响所属 scope/实例 | 保留名称、焦点与可操作布局，显示实际错误 |

### 0.5 明确不在本期

- P3 的 Accordion/Collapsible、新 Dialog/AlertDialog、Popover/HoverCard/Tooltip、Toast/内容组件完整重构；只保留现有调用方和 Select 需要的底层层级行为。
- P4 的菜单/导航/工具栏及完整跨组件等价终验；已有 `ToggleGroup.toolbar` 只保留原能力，不新增 Toolbar 产品。
- P5 的 DataTable/Tree/FilePicker/DropZone/Splitter/PanelTabs/CommandPalette 全量主题迁移和全库旧 API 清退；本期不借适配表单重写这些机制。
- WORK-F17 的完整内容集合仍在 P3；CheckboxCards/RadioCards 在本期仅作为 WORK-F6 的选择表达交付，不能据此关闭 WORK-F17。
- 新 GPU 组件、P3 色域渲染、React 运行时、任意 HTML/CSS 兼容、账号/云同步/远程服务、官网与 crates.io 发布均不开发。不存在待其他系统建设的接口依赖。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** P1 原计划恢复快照为 7/7，补齐计划为 2/2；现有代码提供 `ThemeDocument` v2、`ThemeScope`/`ThemeBoundary` 和 component-local palette。补齐计划的[代码复审](DOM-COMPONENTS-P1-gaps.code-review.r2.md)覆盖静态接入修复。P2 依赖这些具体契约，不要求重结算 P1 全部人工/性能证据。
- **[已核实·足够]** `crates/rustify-components/Cargo.toml` 用 `rustify-ui` 依赖别名指向 rustify-dom，csr/ssr/hydrate 单独启用；`lib.rs`/`styles.rs` 仅导出 CSS 字符串，组件导入不会自动加载皮肤。
- **[已核实·足够]** `input.rs`/`textarea.rs` 在 IME 期间不回正输入，普通输入后按应用值 settle；NumberField/ColorField 复用 `rustify-dom/src/draft.rs`。后者的输入适配器本身不检查 composition，需要字段层补齐 guard。
- **[已核实·缺口]** `checkbox.rs` 只收 bool、无 name/required；`switch.rs` 是 button role=switch；`radio.rs` 和 `toggle_group.rs` 把数组和外观包在一起 → 提取同一受控行为、补三态/组/卡片和表单参与，否则只有改皮肤，没有独立组合能力。
- **[已核实·缺口]** `slider.rs` 只有单个原生 range 和 on_change → 增加多值、身份、方向/间隔及完成/取消，移除旧 native range 专属皮肤，避免新旧两条行为路线。
- **[已核实·缺口]** `form::Form` 已处理校验代次与单次 save，Field 只关联 label/error；SubmitButton 通过点击调用 submit，没有原生 form/reset 桥 → 增加 FormRoot、描述/错误来源、原生提交/重置协议。
- **[已核实·缺口]** `select.rs` 有受控值、活动项滚动和 Layer，但缺分组/部件/字符定位；`overlay::Layer` 的关闭回调无原因，Escape 在捕获阶段路由，不能由自定义子事件撤销 → 在层级拥有者处增加可取消原因钩子，不能只在 Select 子节点补监听。
- **[已核实·缺口]** `i18n::Direction` 已有 LTR/RTL 类型，两种 Locale 当前都返回 LTR；`roving::step/edge` 可复用，但不足以表达局部显式方向 → 加 direction context，语言和方向分别投影。
- **[已核实·足够]** `layout::Reset` 已以 `HtmlElement<E, At, Ch>` 限制单原生元素；`geometry.rs` 已支持 SSR 外部规则和浏览器 CSSOM。Slot/响应式/动态轨道沿这两种范式，不用内联 style 或 AnyView 克隆。
- **[已核实·足够]** `xtask/src/docs.rs::build_hydration` 单独构建 ssr、hydrate、csr，并检查图/产物；`tests/dom-only/hydration.spec.ts` 已检查节点保留、ID 和引用。P2 在此加样本。
- **[已核实·缺口]** `scenes.rs` 有 7 个场景，Cards 用既有表单控件；持久化会拒绝未知 scene → 在 Cards 内增表单区，不引入需要迁移的 scene 值。现有截图用于视觉方向，不能证明 P2 行为通过。

### 1.2 拓扑与文件清单

★ 表示新增；列出的现有目录只允许修改本表所述行为。根 workspace 不增加生产 crate。

| 落点 | 交付或精确改动 | 同构实现 |
| --- | --- | --- |
| ★ `crates/rustify-components/src/primitives/` | `mod.rs`、`slot.rs`、`direction.rs`、`portal.rs`、`choice.rs`、`slider.rs`、`field.rs`、`form.rs`、`select.rs`；公共部件与内部纯计算 | Reset、现有受控控件、roving、Form/Draft |
| 组件 `src/lib.rs`、`styles.rs`、`Cargo.toml` | 公开 Primitives/新组件/样式常量；补必要 web-sys feature 与 package include | 现有模式和显式静态资产 |
| `src/checkbox.rs`、`radio.rs`、`switch.rs`、`toggle_group.rs`；★ `src/toggle.rs`、`checkbox_group.rs`、`checkbox_cards.rs`、`radio_cards.rs`（均在组件 crate） | 改为同一 Primitives 的 Themes 包装；补独立 Radio/Toggle、组与卡片 | 原受控 callback 与 `button.rs` 的视觉投影 |
| 组件 `src/slider.rs`、`input.rs`、`textarea.rs`、`number_field.rs`、`color_field.rs`、`label.rs`、`form/`、`select.rs` | 既有便捷入口保留其值语义，公开部件；删除迁出后的旧事件实现 | 现有 settle、Draft、FormState、Select |
| ★ `crates/rustify-components/css/primitives.css`、★ `form-controls.css`；现有 `slider.css`、`sdk.css`、`rustify.css`、`candidates.json` | 最小行为样式与 Themes 皮肤分开；CSS 生成物同批更新 | `button.css` 的低权重 component layer |
| ★ `crates/rustify-components/vendor/radix/`、`sources.lock.json` | 正式采用的参考 props/CSS/交互源码及 MIT 许可、来源清单 | 既有 vendor/source_files/import_root 校验 |
| `crates/rustify-dom/src/form.rs`、`draft.rs`、`overlay.rs`、`i18n.rs` | 错误来源/重置代次；必要的草稿组合输入桥；scope Portal/方向上下文、Cancelable 中立类型与可取消层钩子/定位结果 | FormState::submit/validated、overlay::route/Layer |
| `crates/rustify-dom/src/lib.rs`、`crates/rustify-ui/src/lib.rs` | 仅重导出新增中立类型；不修改 GPU 实现 | 当前重导出方式 |
| `examples/theme-studio/src/scenes.rs`、`main.rs`；★ `src/form_samples.rs`（该示例）；按需 `app.css` | Cards 中实际表单/视觉状态区，主模块声明；不改 persistence/schema/预设 | CardsScene、现有 provider fixture |
| 受影响示例、DOM consumer、hydration view、Docs islands 的直接调用点 | 按 §3 调整本期返回值/部件/HTML 形态；DOM consumer 加隔离的 forms 测试页/benchmark fixture，不改变原生命周期页的 scope/统计 | 原组件调用点 |
| ★ `tests/dom-only/primitives/`；`tests/dom-only/Cargo.toml`、`Cargo.lock` | 独立无样式 CSR fixture；同一 workspace 新成员，无版本升级 | consumer 的 lib、public、export-dom-assets |
| ★ `tests/dom-only/primitives.spec.ts`、★ `forms.spec.ts`；`hydration.spec.ts` 与 hydration fixture | 原始样式隔离、组合表单、SSR/hydrate 行为；仅重复访问可观测接口 | 现有 DOM-only 与 hydration fixtures |
| ★ `crates/rustify-components/tests/forms_ssr.rs`；`ssr_controls.rs`、`theme_layout.rs` | 初始状态、组合标签/表单值、外部样式；对旧 HTML 断言作必要迁移 | 现有 SSR control tests |
| ★ `tests/browser/theme-forms.spec.ts`、★ `tests/dom-only/forms-performance.spec.ts`；`playwright.docs.config.ts`、`playwright.config.ts`、`xtask/src/docs.rs` | 项目/测试映射、纯 DOM 构建检查及同负载测量 | 既有按 project 启服、build_hydration、theme-performance |
| `crates/rustify-docs/src/style.rs`、`xtask/src/docs.rs` 的样式资产清单 | 注册/导出新 primitives.css、form-controls.css，保持文档单 CSS 构建可解析新 import，不改内容/搜索/发布 | 现有 SDK_LAYER/SDK_SLIDER 与 asset 列表 |
| `docs/components.md`、`docs/dom-components-migration.md`、`docs/quickstart.md`、组件 README；`src/catalog.rs` | 公开入口/状态/样式与本期迁移示例；CATALOG 的 24 个旧导航分类保持 | 已有 COMPONENTS/SUB_CAPABILITIES 和生成器 |

独立记录在计划旁按需生成，必要的可共享终验证据放 ★ `docs/validation/dom-components-p2/`。新增文件只为本期公开能力或可复测检查，不预建空记录。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| Primitives 组合/方向/Portal | 自有设计语言、Themes | 单原生目标；稳定状态；最近方向/scope；不消费品牌变量 | typed Slot 与 children | 事件顺序、ARIA/ID、资源归属 | 进程内；真实 DOM，无 adapter | 编译失败例、SSR、浏览器取消/嵌套/释放 |
| choice / slider / field / select | 所有对应便捷与部件入口 | Signal 是唯一显示事实；请求不代表接受 | 类型化回调/只读 Signal | 原生回正、手势、焦点、列表规则 | 进程内；纯函数直接测，浏览器验证 DOM | 接受/拒绝/延迟、动态项、FormData |
| FormState + FormRoot | 表单调用方、SubmitButton | 不持字段值；陈旧校验不生效；同一次 submit 最多 save 一次 | 应用 rules/save/on_reset | 代次、错误优先级、原生提交桥 | 进程内；继续实际 FormState | 校验逆序、reset 后旧结果、Busy/失败重试 |
| DOM overlay | Select、现有 Dialog/Menu | 编辑会话优先，最上层消费关闭；取消不穿透 | 可取消原因/自动聚焦回调 | 焦点、inert、定位与 abort | 进程内浏览器宿主；无远程依赖 | 嵌套 Select、触发器移除、CSP/清理 |
| Themes | 应用、Studio | 按组件 props 投影同一 Primitive；class 仍可覆盖 | 公开 part/class 与 P1 snapshot | 尺寸、状态、颜色边界 | 进程内静态 CSS/现有 geometry | 声明矩阵、两模式/局部覆盖 |

不另造 renderer、输入适配器总线或第二套 form model。分层的价值是把真实选择/关闭策略收进同一实现；纯转发的内部 helper 不公开。

## 3. 数据与公共 API 迁移

不改变主题 JSON v2、存储键、已有库/草稿或 scene 偏好；无数据库与回填。

1. **Expand**：增加 `primitives`、部件与新视觉类型；旧便捷函数保留当前标量/布尔回调。`Checkbox` 增加 `indeterminate`，多滑块另有 `RangeSlider`；复杂组合使用对应部件。
2. **Migrate**：逐个将本期 Themes 包装指向同一 Primitive，立即移除原事件状态机；同一批修改直接调用点和必要测试。内部 DOM 层级、原生 range 专属选择器不作为兼容 API。
3. **Form 兼容**：保留现有 `Form::new/changed/validating/validated/submit/submitted/set_error`；增加来源明确的错误方法与 `reset()`，`set_error` 继续表示同步规则。既有手工 submit 路径仍由同一 FormState 裁决；FormRoot 内的 SubmitButton 使用原生 submit，不再同时调用两次 submit。
4. **Contract**：旧 bool/标量/数组便捷入口可以继续存在，因为它们只是同一实现的投影；不保留旧组件实现。无需本期清除与 P2 无关的公共入口。目录逐项更新可用能力，不能把 P3–P5 计划项改为支持。
5. **说明与回滚**：迁移指南包含 checked/indeterminate、具名多值、name/required/reset、Select 元数据/部件、TextField 插槽/attr 落点、CSS 与事件取消的可复制示例。失败时恢复受影响源码与匹配 CSS/锁文件，重建消费者；没有主题数据逆迁移。代码与 CSS 必须同版交付，不把 source API 写成 registry 0.2.1 已支持。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P1 作者主题/作用域/配色 | 已核实足够 | theme/provider.rs 由创建上下文投影；component_color::set_color 只写本地色板；两模式由源文档解析 | 消费原接口，无 schema/存储修改 | 自定义低对比保留；不把默认配色结果当认证 |
| SDK ID 与静态 scope | 已核实足够 | components::id::next 优先 SDK allocator；ssr_scope/hydrate 使用相同 seed 与初始 view | 所有 part ID 用现有 allocator；不从用户 value 拼 DOM id | 初始开启 Portal 沿用 StaticRenderIssues；SSR 输出闭合触发器 |
| Layer/OverlayStack | 已核实缺口 | overlay::route 在 capture 阶段处理 Escape；Layer 只接 on_close，旧 fit 只反馈坐标 | §5.5 增量；旧调用省略新 hooks 时行为保持 | 取消由顶层消费；anchor 失效转安全清理，不递归弹请求 |
| Leptos 单元素与事件 | 已核实足够 / 组合缺口 | Reset 的原生泛型已可编译；现有控件未提供 Slot/事件组合层 | Root 冒泡处理在自定义子事件之后；保留子事件，不覆盖同名目标 listener | 非原生/多根由类型错误定位；缺 Root 保留内容并给 part 诊断 |
| SSR/hydrate 构建 | 已核实足够 | xtask::docs::build_hydration 按三个 mode 分别编译、检查；host Layer 不生成浮层 DOM | 增加本期样本/图检查，Primitives 独立 CSR fixture | 非确定性输入不宣称可接管；非法组合保留可读输出与诊断 |
| Studio 预览 | 已核实缺口 | Cards 只有旧字段/二态；ScenePreview 与主题状态分开 | 增 `FormSamples`，沿用现有 root/宽度/模式控制 | 失败与拒绝可见；局部样本不影响主题历史/保存 |
| 发行/外部系统 | 已核实足够 | PRD 明确无外部服务依赖；当前 source/registry API 有区别 | 只交源码、资料、同版测试资产；不部署/发包 | 工具或浏览器缺失时记录所缺资源，请用户提供；不自动安装基础设施 |

前置 C-1 由仓内源码提供，已具备开工条件。P3 对话框重构、P5 全量消费者迁移均不是 P2 开发前置；现有对话框会作为真实 Select 嵌套集成环境。

## 5. 核心机制与接口

以下为新增/改变接口的规范。未列的现有常规 props 保留。所有信号接受 `#[prop(into)]`，事件回调使用 Leptos `Callback<T>`；不增加 `defaultValue/defaultChecked/defaultOpen`。

### 5.1 组合、状态与 scope

- `primitives` 下公开 `checkbox`、`radio`、`radio_group`、`checkbox_group`、`switch`、`toggle`、`toggle_group`、`slider`、`text_field`、`text_area`、`number_field`、`color_field`、`form`、`select` 命名空间；choice 与 field 的多个 namespace 可由各自实现模块重导出，不要求一 namespace 一文件。
- `Slot<E, At, Ch, A>(target: HtmlElement<E, At, Ch>, attrs: A) -> impl IntoView`，约束沿 Reset：E 为 ElementType + Send、At/A 为 Attribute + Send、Ch 为 RenderHtml + Send。typed 目标就是 Slottable 的 Rust 表达，不另加运行时 children 猜测。tuple/fragment/AnyView 编译失败；不丢内容、不包第二个 button/link。各适用 part 提供 `*_as(target)` 函数，从所属 Root 取得属性后调用 Slot。
- Slot 只追加部件属性，不往目标叠加同名事件。触发/选择行为在所属 Root 的冒泡边界处理，Content 在自己的 DOM 边界处理 Portal 内事件。每个边界用所属 Root ID 验证目标，嵌套 Root 不被外层重复处理。调用方目标事件先执行；`default_prevented` 为 true 则不执行库默认请求。原生链接的 href、修饰键与新标签页行为保留，库不以 preventDefault 抹掉导航。
- `Cancelable<T>` 在 rustify-dom 的 overlay.rs 目标中立部分定义，由 SDK 和 Primitives 重导出；提供 `value() -> &T`、`prevent_default()`、`default_prevented() -> bool`，clone 共享本次取消标记，仅活到当前回调链。选择、外部交互、Escape、自动聚焦等语义钩子均同步调用；业务值 callback 只是请求，不能把拒绝值等同于取消默认关闭。
- 普通属性/调用方事件保留；`role`、部件 ID、ARIA 关联和下列状态由部件拥有。调用方通过公开 props 选择状态，不手写相互矛盾的保留属性。class 走现有 merge；不接收原始 HTML 字符串。
- 状态协议：适用节点有 `data-rustify-part`、`data-state`（checked/unchecked/indeterminate、on/off、open/closed）、`data-disabled`/`data-readonly`/`data-invalid`（true 时空属性）、`data-orientation`、`dir`；浮层增加实际 `data-side`/`data-align`。状态在交互节点、装饰 Indicator/Thumb 同步；ARIA 使用规范值，不能直接复制三态字符串。状态值由应用已接受值派生。
- `DirectionProvider(direction: Signal<rustify_ui::Direction>, children)` 在自己根写 dir；底层 `provide_direction(Signal<Direction>)` / `use_direction() -> Signal<Direction>` 放入 core i18n.rs，Portal/Layer 和组件共同读取。use_direction 优先最近显式 context，否则按 locale 方向。局部 RTL 不更改 Locale/应用值；horizontal 箭头、start/end、Switch knob、Select 对齐读同一方向。
- `VisuallyHidden(children)` 仅用最小外置 clipping 样式隐藏，保留辅助语义；`AccessibleIcon(label, children)` 在有 label 时提供一次名称，否则装饰 `aria-hidden=true`，独立图标操作的名称仍在按钮上。现有 Icon/Glyph 不复制成另一图标库。
- `Portal(client_only=false, children)` 由 core overlay.rs 的新增 `ScopePortal` 实现，Primitives 仅重导出；只挂在 `use_overlay().overlay_root()`，不挂 body/任意外部容器。它在 core 内读取创建处的 theme/locale/direction，使用私有 write_snapshot 的既有投影，避免跨 crate 调用私有函数；无主题 provider 时不生成品牌 token。不自动注册 modal、关闭或定位。on_cleanup 删除包装节点，instance abort 通过绑定到该节点的原生 JS Element.remove 注册 release_on_abort，不能使用进入已失败 wasm 的闭包。浏览器无 scope 时保留 inline 内容并标 `data-rustify-issue="missing-scope"`；host 的 client_only 模式沿 §5.7，否则记录 StaticRenderIssues，不假装服务端已挂载 Portal。
- 非法 min/step/重复值键/缺 Root 用现有 `geometry::PropError { field, message }` 描述，并在可定位 Root 上写固定 `data-rustify-issue`；数值/键校验 helper 返回 Result。只报告 prop/part 名，不记录用户输入。非法单元素优先使用编译诊断；无效配置不发业务请求，原 children/最后有效显示保留。

### 5.2 选择、组与选择卡片

核心公开类型：`CheckedState { Unchecked, Checked, Indeterminate }`、`SelectionMode { Single, Multiple }`；复用现有 Orientation。

| 入口/部件 | 受控接口与默认 | 原生/请求规则 |
| --- | --- | --- |
| `checkbox::{Root, Control, Indicator}` | Root：checked: Signal<CheckedState>，on_change: Callback<CheckedState>；Control 是一个原生 checkbox | Indeterminate 的 property 为 true、ARIA 为 mixed；激活请求 Checked；其余两态互换。每次 change 后同时 settle checked/indeterminate，包括应用拒绝和 read-only |
| Themes `Checkbox` | 原 checked: Signal<bool>/on_change(bool) 保留；加 indeterminate: Signal<bool> 默认 false | indeterminate 优先显示；激活请求 true，应用负责接受并清除 indeterminate。部件 Root 可直接消费 CheckedState，无平行实现 |
| `radio::{Root, Control, Indicator}` / Themes `Radio` | checked: Signal<bool>、on_change: Callback<bool>、name/value；单项无隐式取消 | 原生 radio；同组一次只成功一个；只读阻止键盘/点击变更并回正整个组，不能只恢复刚点节点 |
| `radio_group::{Root, Item, Indicator}` | value: Signal<Option<String>>、on_change: Callback<Option<String>>；可选 name、required、orientation、loop=true | 显式 name 时使用原生同名 radio，匿名时省略原生 name 并由本 Root 实现单选；一个组 tab stop，箭头跳禁用项并请求选择；Home/End 可达边缘；只读仍可移动焦点但不请求值 |
| `checkbox_group::{Root, Item, Indicator}` | value: Signal<Vec<String>>、on_change: Callback<Vec<String>>；name、required | 各原生 checkbox 独立 Tab；组多值去重，输出按声明顺序，禁用项不被用户改变；required 表示至少一项，不误变成所有项必选 |
| `switch::{Root, Control, Thumb}` | checked: Signal<bool>、on_change(bool)、name/value/required | 原生 checkbox role=switch；同一成功控件，不再用 button 加第二份表单状态 |
| `toggle::{Root}`、`toggle_group::{Root, Item}` | pressed: Signal<bool> 或 value: Signal<Vec<String>>；mode、allow_empty=true、orientation、loop=true | 原生 type=button/aria-pressed；单选不超过一项、多选保留声明顺序；命名且按下时才输出 hidden accepted value，原生 button 不直接提交 |
| `CheckboxCards::{Root, Item}`、`RadioCards::{Root, Item}` | 转发对应 group 的值/禁用/表单 props；Item 另有 label/description 插槽 | 一个可点击 label 容器和同一 Control；说明 ID 进 describedby，不允许嵌套按钮/链接成为选择卡片点击区。使用者可把额外操作放卡片外 |

每个 RadioGroup 与 RadioCards Root 用现有 allocator 分配独立 `group_id`，只用于 roving focus、ARIA/label 关联与私有注册，不得作为原生 name 或业务提交键；SSR/hydrate 按 §5.7 保持调用顺序和身份。

所有成功控件支持 name/value/form/required/disabled/read_only/invalid/described_by；name 缺省不参与 FormData，checkbox/switch 的 value 默认 on。RadioGroup/RadioCards 仅在显式提供 name 时把同一业务 name 写到 Item 的原生 radio，由浏览器按 form owner 分组；匿名 Item 必须省略 name 属性，由所属 Root 按受控 value 同步整组 checked。点击/箭头只发请求，请求后按当前接受值回正整组，拒绝或延迟不留下候选选中；同 scope/同 form、无 form owner 的两个匿名组互不取消。FormData 只含显式命名组的接受值，不含内部 ID。indeterminate 不作为成功选中值。禁用项不提交，read-only 保留已接受值且不请求。Toggle 默认未命名，不凭空生成业务字段。

动态数据：键唯一；移除的值不再有成功控件，不擅自修改应用状态。活动项失效时，焦点若仍在本组则移向同方向下一个可用项、再前一个、最后组根（tabindex=-1）；焦点在组外不偷取。

只对需要代理的组约束设置原生 validity；至少一项必选时，以首个可用 Control 为 constraint/focus 代表，根据**整个组的接受值**计算 required/custom validity，不要求固定第一项被选中。匿名组不依赖原生同名分组实现 required。没有可用项时组不可操作，依 disabled 语义不阻止提交。

### 5.3 Slider：值、身份、手势

```rust
pub struct ThumbValue { pub id: String, pub value: f64 }
pub struct ThumbLabel { pub id: String, pub label: String }
// primitives::slider::Root / Themes slider::Root
// values: Signal<Vec<ThumbValue>>, on_change: Callback<Vec<ThumbValue>>
// on_commit: Option<Callback<Vec<ThumbValue>>>, on_cancel: Option<Callback<()>>
// min=0, max=100, step=1, min_steps_between=0, preserve_order=false
// orientation=Horizontal, inverted=false; name/disabled/read_only 等同 §5.2
// 部件 Track、Range、Thumb；Thumb(id, aria_label / labelled_by, value_text)
```

- `RangeSlider(values, thumbs: Vec<ThumbLabel>, callbacks, ...)` 是上述部件的便捷组装；现有 `Slider(value: Signal<f64>, on_change(f64), ...)` 使用唯一 id=value 投影，并新增同类型 on_commit/on_cancel。没有另一套 native range 事件/皮肤实现。
- 校验所有数有限、min < max、step > 0、id 唯一且每个 thumb 具名；保留最后有效配置，初始非法则渲染不可操作且可读的配置错误。合法外部值按 min/max/step 规范化用于绘制/ARIA/表单，但不回写应用；目录说明这层规范化，不伪装成应用已存储另一个值。
- 沿用 `rustify_ui::snap` 的边界/步长；按 min 为基点计算整数 step 序号，避免浮点累积。候选更新带 id，数值输出按 value 排序，相等时按稳定声明顺序。相邻最小距离为 min_steps_between × step；空间不足的配置失败，不悄悄压缩间隔。
- preserve_order=false 时允许跨越，更新的是拖动的同一个 id；开启后按手势开始时的数值邻接关系约束上下限。最小间隔对候选排序后的邻居始终成立；不能成立的候选不发请求。Thumb DOM 顺序取首次声明顺序，增项追加、删项移除，值排序不移动 DOM/焦点。
- 横向从左增加由 LTR/RTL 与 inverted 共同确定；竖向默认向上增加，inverted 翻转。键盘 Arrow/Home/End/PageUp/PageDown 走同一计算；Page 为 10 step，此为本期交互决策。aria-valuemin/max 在 preserve_order 下反映实际邻接限制；每个 Thumb 有 role=slider、值/方向/名称，键盘都可达。
- 嵌套 Layer 内的活动 Slider 收到 Escape 时沿父 Layer 的既有优先级关闭浮层，Slider 在卸载清理中取消手势且不 commit；不单独保留父浮层。键盘与指针手势遵循同一规则。
- pointerdown 在轨道选最近可用 thumb，等距优先已聚焦、再声明顺序；Thumb 捕获 pointer。缓存轨道 rect 到手势结束/实际 resize/scroll 失效；pointermove 不每次查询所有 DOM 或重建样式规则。纵向保留水平滚动、横向保留纵向滚动，触控 drag 用 Pointer Events；多指不抢占已有手势。
- 临时 session 只保存 pointer id、活动 thumb、开始接受值与**最后请求候选**，不作为显示来源。每次 move 发 on_change 后显示仍取应用 Signal。pointerup 在有有效变化时 on_commit **一次**，载荷是最后请求候选，即使应用拒绝；键盘一次按键序列在 keyup/失焦结束时提交一次。
- pointercancel、lostpointercapture、Escape、活动 thumb 移除/禁用或 scope 清理都取消，不发 commit；on_cancel 供应用撤回已经接受的 preview，组件不擅自恢复业务值。没有手势不发取消。所有结束路径先解除捕获/临时监听；提交后 blur 不再提交。
- 几何只从接受值通过 geometry CSSOM/SSR 外部规则投影；不使用 style 属性，不把帧内请求排成历史队列。命名多值按数值排序输出同名 hidden accepted inputs；表单值不读取 session 的最后请求。

### 5.4 字段与 Form

**字段入口。** `text_field::{Root, Input, Slot}`、`text_area::Root`、`number_field::{Root, Input}`、`color_field::{Root, Input, Swatch}` 以及无样式 Label。主题入口由现有 TextField/TextArea/NumberField/ColorField/Label 包装；TextField 的前后插槽只在显式给出时出现。无样式 Slot 只承载 side/children；Themes `text_field::Slot` 另公开 `color: Signal<Option<AccentColor>>` 与 `gap/px/pl/pr: Signal<Responsive<Spacing>>`，默认均不声明覆盖，默认值与优先级见 §5.6。Slot 的 side 使用 Start/End，随 direction 投影；px/pl/pr 保留参考的物理左右含义，LTR/RTL 只改变 Start/End 的位置，不互换 pl/pr。兼容文档说明参考 left/right 与逻辑 side 的区别。

- 所有字段把 id、aria_label/labelled_by、described_by、name、form、required、disabled、read_only、placeholder 和适用原生属性传到真正编辑元素；wrapper 的 class 与 input_class 分开。组合 Input 可通过原生 attr 接入 autocomplete/inputmode/pattern/length 等，便捷入口公开这些常用字段，不把 name 放在 div 上。
- TextKind 保留现有类别，并补参考稳定原生 date/datetime-local/month/time/week/hidden；文件与颜色选择仍使用专门能力，不移植 unstable 密码切换器。TextArea 的换行/CR/HTML 转义和 SSR 前导 LF 规则原样移入行为实现。
- 复用现有“值不同时才 set_value”以保留光标/选区/原生撤销。compositionstart 到 compositionend 不 publish/settle；end 对最终文本只请求一次，过滤紧随的同值 input 重复交付，下一次不同内容仍正常交付。禁用/只读中不请求。
- NumberField/ColorField 继续使用 SDK Draft；增加字段级 composition guard，使未完成 IME 不进入 parse/preview，Enter/Escape 在 composing 时不提交/关闭。Enter 或 blur 的一次 commit、Escape 取消、失焦恢复接受值、未触碰时接受外部更新均保持既有规则；NumberField 的边界继续只宣告，不替应用夹取业务值。
- Draft 的可见编辑 input 不作为命名成功控件；命名时另有隐藏接受值镜像，因此 FormData 不包含 `-`、`#1f` 或被拒绝草稿。required/validity 关联可聚焦编辑控件；提交桥按下述注册顺序结束当前 Draft，再按应用此时的接受值取表单值；拒绝或延迟接受不把请求值写入镜像。

**新增 FormRoot。** 公开 `FormRoot(form: Form, rules, on_reset: Option<Callback<()>>, on_asked: Option<Callback<Submit>>, children)`；无样式为 `primitives::form::Root`，Themes 仅增加布局。实际输出一个 form，内部不允许第二个 form；action/method 等原生属性可透传，但 Rustify 提交桥默认 preventDefault，不自动发网络请求。

1. 原生 submit（Enter、button、requestSubmit）进入同一个边界；调用方取消/IME 未结束则不触发。FormRoot 内的 Control 在 primitives/form.rs 的私有注册表登记可见 focus id、接受值 flush、原生约束读取与可选 Draft commit；清理时注销。提交先沿字段顺序结束当前注册 Draft（仍按既有 last-preview 规则），同步其接受值镜像，再读取 validity/validationMessage。这个注册表只协调已有控件，不持业务值或公开通用 adapter。用 novalidate 收口浏览器弹层，首错定位到绑定可见控件。
2. 先检查提交忙状态；非 Busy 时按 `form.elements` 顺序读取绑定及未绑定原生控件（含外部 form 关联）的 validity/validationMessage。绑定 Control 的失败写入 Constraint 并继续 FormState；未绑定控件的失败是 **FormState 前置阻塞**：不运行 rules/save，不调用 on_asked，聚焦首个此类实际控件。checkValidity 在该控件自身派发的原生 invalid 事件（不冒泡）及其 event.target/validationMessage 是公开结果，调用方直接在控件上观察；不得伪造静态 field 键或改写成 `Submit::Blocked`。
3. 随后依次检查当前成功控件/required 约束、应用同步 rules、尚未返回的异步代次；继续 `FormState::submit`/`validated` 处理 Busy/Blocked/Waiting/Save，on_asked 只报告这条 FormState 路径的结果。保存函数只由 Form handle 运行一次，不能先 button 保存再 form 保存。
4. 错误来源公开为 `FieldErrorSource { Constraint, Rule, Check, Application }`。每个 Field 把当前单个 error tuple 改为按来源存放的错误集合，展示优先序为 Constraint → Rule → Check → Application；error/error_source 返回同一条。既有 set_error 保持 Rule；新增 set_application_error、error_source 与约束错误入口。重跑 rules/constraints 只替换各自那一格，不能覆盖或抹掉未修改值的应用错误；高优先来源清除后仍可显示保留的低优先来源。changed 清除此字段旧错误和陈旧等待，与现有语义一致。错误文本有对应 data-source。
5. `FieldBinding` 增 description_id、error_id 与当前 source；described_by 以去重 ID 列表合并 hint 与真实存在的错误，label/ARIA 都指向实际 Control。新 `Field` 保留现有 control(binding) 接缝，增加 description；parts 公开 Label/Description/Message，不能强迫开发者依赖固定 DOM 层级。
6. reset 事件总是先 preventDefault，再交 on_reset；应用接受后设置业务值并调用新增 `Form::reset()`，后者清校验/等待/dirty/failure 并递增各字段 generation，旧异步回包失效；成功时同步使已注册字段的旧 Draft 会话失效并显示当前接受值，不交付旧 preview/commit/cancel；没有回调或应用拒绝时显示和 FormData 不变。submitting 时拒绝 reset，报告 Busy，不让旧保存结果写入新会话。
7. FormStatus/SubmitButton 继续可单独使用。FormRoot 内 SubmitButton 是 native submit，只呈现状态；外部 legacy 调用仍直接 Form::submit，同一 save 防重。失败保留接受值与错误说明，submitted(Err) 后可修改/重试。

原生表单键与 Label 不能从用户 value 拼接为全局 ID。库不自动把 name 变为业务字段身份；Field 的 field 继续用已声明的静态键，外部 form attr 通过 form 的 id 关联。离开 FormRoot 的命名控件也保持 FormData/原生约束语义，其 invalid 事件仍指向实际 Control；FormRoot 提交桥按上述前置分支聚焦。

### 5.5 Select、层级与取消

**部件。** 无样式与 Themes 均公开 Root/Trigger/Value/Icon/Content/Viewport/Group/Label/Item/ItemText/ItemIndicator/Separator/ScrollUpButton/ScrollDownButton；Portal 使用 §5.1。组合部件不要求接受一个固定 options 数组视图。

- Root 的数据接口是 `value: Signal<Option<String>>`、`on_change: Callback<Option<String>>`、`open: Signal<bool>`、`on_open_change: Callback<bool>`、`entries: Signal<Vec<SelectEntry>>`；`SelectEntry { value: String, text_value: String, disabled: bool }` 只定义稳定身份/字符定位/原生表单元数据。Item(value, children) 自由组合图标、说明和分组；ItemText 可用自定义 children，否则使用 text_value。Group 的 Label 只关联本组，不成为可选项。
- 显式 entries 让闭合 SSR 在不执行 Portal 时已有标签和表单结果。Root 按 entries 取有效选择和字符定位顺序；Content 只呈现其 Item。未知/重复 Item 或重复 entry 值诊断并禁用该非法项，不产生错误选择；有可用 entry 却没有对应可操作 Item 也标诊断。分组/自定义内容不能偷偷替换 text_value 的定位语义。
- 既有 Select(value: Signal<String>, options: Signal<Vec<SelectOption>>, callbacks...) 保留作为默认部件组装，空字符串映射 None，非空 value 必须唯一。新的 Root 保留 None 表示空值，禁止空字符串 entry（明确诊断），避免 placeholder 与真实项相撞。
- 采用 select-only combobox：Trigger 保持 DOM 焦点，role=combobox、aria-expanded/controls/activedescendant 指向已挂载的 listbox/活动项；Content 不抢焦点。Item 是 option，不是 button，mousedown 保留 trigger 焦点。触控选择同样走一次选择请求。
- 开启时活动项取当前有效项（禁用项除外），否则首个可用项；ArrowUp 打开时无已选则从末项进入。ArrowUp/Down、Home/End 跳禁用项；Enter/Space 选择，Escape 取消；Tab 请求关闭并继续自然焦点路径，不阻塞到已卸载 Content。
- 无 modifier 且非 IME 的可打印字符进入 prefix 定位；连续同字符轮选，搜索缓冲在最后输入后 1000ms 清空（与参考 useTypeaheadSearch 一致），计时器用 runtime 可释放资源。关闭时清空；不会新增搜索框/过滤/Combobox 产品。
- on_select(Cancelable<String>) 在 on_change 前运行；取消不改变值/不关闭。未取消则发一次值请求和一次关闭请求；应用拒绝值仍显示旧值，若拒绝关闭则保持 open；不能把异步接受前的候选临时显示为已选。
- entries 为空、当前值被移除或未知时，Value 显示 placeholder、无 aria-selected 项，原生镜像使用空值；业务 Signal 不擅自清除。活动项移除/禁用时按可用顺序回退，无项则清 aria-activedescendant。长列表用实际滚动 viewport，活动项只滚自己的 scrollport，滚动按钮 disabled 状态反映边界。
- name/form/required 用单个 visually-hidden 原生 select 镜像 entries 和**接受值**；不是第二份业务状态。disabled 不提交，read-only 可检查列表但不能请求新值，required 的原生失败定位 Trigger。

**Layer 的必要增量。** 在 overlay.rs 的 host/browser 两个签名同步增加可选 hooks：`on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>`、同类型 `on_pointer_down_outside`，以及 `on_open_auto_focus`/`on_close_auto_focus: Option<Callback<Cancelable<()>>>`；`LayerDismiss { Escape, PointerOutside }` 和 Cancelable 均在 core 定义。另加 `direction: Option<Signal<Direction>>` 与 `on_placement: Option<Callback<LayerPlacement>>`，其中 `LayerPlacement { side: Side, align: Align }`、Side 本期为 Top/Bottom、Align 为 Start/Center/End。全部中立类型由 SDK 重导出。现有省略 hooks 的调用语义不变。

- `route` 保留 native composing/Draft 优先；随后在最上层调用 Escape hook，取消也算该层已消费，不交下层关闭。回调在锁外执行。程序 `close_top()` 保留直接请求关闭，不伪造 Escape。
- 外部 pointer 用 stack 级监听，只处理最上层与其关联触发器之外的目标；取消则无关闭，触发器切换不被当作外部再关闭。真正外点后不强制回抢焦点；关闭时仅在原焦点仍属于该层/触发器且 hook 未取消时回焦，触发器失效则落现有 scope fallback。
- Select 取消 Layer 默认打开自动聚焦，改为保留 Trigger；关闭自动聚焦可公开取消。程序应用设置 open=false 也触发 close_auto_focus，value 不受其影响；不产生“用户取消选择”事件。
- 现有 fit 支持保留；增加实际定位结果 Side/Align 的反馈与 direction 参数，Select 默认 Bottom/Start、适配视口。反馈与 CSSOM 坐标同一次计算，碰撞后 data-side 反映实际 Top/Bottom；不把 P3 的任意边侧/箭头配置整套提前纳入。Resize/scroll 复用已有 observation 与 moved 信号。
- anchor 断开是生命周期清理：请求 close 一次并移除该层的注册/观察/捕获，不能因应用拒绝 open=false 留下无法定位的层；应用此时仍持 open=true 则不显示失效内容，等待有效 Trigger 再次存在。正常清理和 instance abort 保持其他 scope 可用，不承诺同实例 trap 后继续编辑。

### 5.6 Themes 视觉参数与样式

下表为本期公共矩阵，来自本地 Themes props；枚举各自定义且提供 ALL 常量，便于穷举声明状态。默认 size=2、variant=surface；不向没有该字段的组件添加 highContrast 或 radius。Toggle 不在 Themes 参考公共导出中，延续现有 Rustify 外观并沿用组状态，不伪造上游 variant 集合。

| 组件 | size（均支持 Responsive） | variant | 其他视觉能力 |
| --- | --- | --- | --- |
| Checkbox、CheckboxGroup、Radio、RadioGroup | 1/2/3 | classic/surface/soft | color、high_contrast；必要方框/圆形不被 full 改形 |
| Switch、Slider | 1/2/3 | classic/surface/soft | color、high_contrast、radius |
| TextField、TextArea | 1/2/3 | classic/surface/soft | color、radius；TextArea resize=None/Vertical/Horizontal/Both 响应式 |
| TextField Slot（Themes） | 不适用；默认度量随 Root | 不适用 | 独立 color；gap、px、pl/pr 均为 `Responsive<Spacing>`；side=Start/End |
| Select Root | 1/2/3 | 由下两部件定义 | group/viewport/font-size 随 size |
| Select Trigger | 继承 Root | classic/surface/soft/ghost-offset/ghost | color、radius、placeholder |
| Select Content | 继承 Root | solid/soft（默认 solid） | color、high_contrast |
| CheckboxCards、RadioCards | 1/2/3 | surface/classic | color、high_contrast；columns、gap 复用 GridTracks/Gap 响应式 |
| NumberField、ColorField | 1/2/3 | 与 TextField 同族 | Rustify 扩展采用同一输入皮肤，保留 Draft；Color Swatch 为显式装饰 |

- 样式来源按 §1.2 正式纳入 props、相关 CSS 和必要交互参考，记录本地版本、指纹、许可与实际采用文件清单。以参考为度量/状态依据，将 rt 选择器改为 scope 内的 Rustify component layer；不导入其全局 reset 或 React 源码运行时。构建从正式输入读取，不访问 ref。
- 颜色调用 `component_color::set_color/use_mode`，局部未指定值继承最近 snapshot 的语义对，不把 accent 重命名为 primary。high_contrast 仅改变选中前景/背景等声明角色；必要边界、错误、焦点分别可见。
- TextField/Select 的三个高度采用参考 space-5/6/7，经现有 `--rustify-space-N` 一次解析；各控件/卡片尺寸从正式参考 CSS 转写为命名度量。不可硬套 Button 的四尺寸。圆角/字体/间距使用已解析值，不二次乘 scaling。
- TextField Slot 未指定 color 时使用中性 11 号前景；显式 color 只将该 Slot 改为 accent 11 号前景，不随 Root color 自动染色。Root size 1/2/3 对应默认 gap `Scale(2)/Scale(2)/Scale(3)`、左右 padding `Scale(1)/Scale(2)/Scale(3)`。显式 gap/px/pl/pr 按 Responsive 的就近较小断点继承；同一物理边缘的 pl/pr 覆盖 px，未声明的边缘保留默认。调用方同类 utility 按既有层级覆盖默认与组件声明。
- `PRIMITIVES_CSS` / `styles::PRIMITIVES` 只含必要布局、隐藏、触控和轨道结构，不能定义品牌色、主题字体、阴影或全局 reset；export-dom-assets 的无样式 fixture 只输出该样式和 runtime CSS。Themes sdk.css 导入同一结构层及 form-controls.css，不用复制两份。
- Themes class 用现有 reactive_merge；低权重规则放 @layer components，调用方 utility 位于其后。局部颜色写在实际组件边界并复位子部件角色，嵌套按钮/字段不继承错误的本地色板。
- 改 class/skin 后统一 `mbx xtask css` 更新 rustify.css/candidates.json；发布给调用方的 CSS constants 与包归档 include 覆盖全部新样式/vendor 许可。应用仍只能选择预编译 CSS 或自有 Tailwind 单份构建。

### 5.7 SSR 与 hydrate

- 纯计算、类型/props、初始受控属性可在 host 使用；NodeRef 访问、pointer capture、selection、timers、Portal、监听只在 wasm 分支/effect 中调用。
- ID 来自 ssr_scope 的 seeded allocator，不能在 client effect 前后改变调用顺序；SSR/hydrate 使用相同 accepted values、声明项序列与 scope seed。三态 SSR 写 aria-checked=mixed，接管后设置 indeterminate property；原节点和 ID 保持。
- 业务静态内容、Label/Description、selected text、named accepted values 和必要几何写进 SSR；geometry 通过 collector 导出外部 CSS。TextArea 保留转义/前导换行测试。
- Select/可浮动 Portal 初始关闭。host 发现开启请求则 StaticRenderIssues 明确拒绝该静态用法；浏览器完成接管后可打开。声明为 client-only 的裸 Portal 在 host 留确定占位，hydrate 初次同占位，attachment 后挂入 scope overlay。不能用 SSR body portal 假装支持。
- hydration fixture 追加本期原生/主题与原始部件样本；多个 scope/不同实例的相同业务值不复用 ID。验证无重复节点、引用失配或 CSR feature 偷带入 hydrate。

## 6. 前端与交互

沿既有 Studio 的 Operate 方向：中性编辑壳与蓝色焦点，作者主题只影响预览。已查看的 P1 [实拍](DOM-COMPONENTS-P1.ui-captures/r8/U5-dom-caller-1440-dark.png)仅说明当前壳/预览边界；新增样本使用实际新组件。

- Cards 场景增加“表单组件 / Form components”区，使用现有 Heading/Flex/Grid/Container 组织；第一层是完整可提交本地表单，第二层可展开“状态与部件 / States and parts”。不在编辑列塞入组件配置，也不为每个字段套装饰卡片。
- 完整表单含名称/说明/错误、三态与组、选择卡片、单值/范围、Number/Color Draft、分组 Select；展示实际 FormData 与保存状态。提交在本地内存完成，成功文案明确为本地演示；不创建业务后端。
- 状态区按组件选择 variant/size/color/状态，并支持 Primitives/Themes 展示方式；策略控制“接受/拒绝/延迟接受”是可操作的示例工具。每个实际请求计数/最后请求与应用值分别可见，禁用态使用真正 disabled/readonly，不能用静态图伪装交互。
- P1 已交付 Button/布局/排版继续有可操作样本；表单区用实际布局/排版承载任务，保留原 provider fixture。预览宽度、Light/Dark 不修改 theme document，不清空编辑历史；新增样本不成为独立持久化主题事实。
- 既有 compare/GPU 视图保留原 GPU 内容，在新 DOM 表单样本旁明确“这些新增表单仅 DOM”；不显示空白 GPU 控件或承诺交互对等。
- 名称、帮助、错误和状态区控制使用中英文；作者输入、value/name/主题名称不因语言切换被翻译。空 Select 提示无可用选项，未选显示 placeholder；应用错误与客户端错误靠文本/位置和来源标识区分，状态不只靠颜色。
- 390px 单列与 768px/桌面使用现有响应式容器；组/卡片可换行，长错误/值折行，二维数据才局部滚动。Select 高度受可用视口限制；键盘和触控能到达全部选项。
- 焦点保持：加载/提交不删除标签和触发器，错误把焦点定位到绑定 Control；调用方取消 hook 不移动焦点。减少动效沿 motion-duration，不以动画表达唯一状态。
- 浏览器验收覆盖默认浅/深、配置主题、局部色/语义覆盖、RTL、窄屏、长内容、键盘与真正取消/拒绝/延迟。默认普通文本至少 4.5:1、必要非文本边界至少 3:1，按 PRD WORK-N1/AC-1；自行编辑低对比作者值不被强改。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD AC-1；APG checkbox/combobox/multi-thumb | 本期所有适用交互有名字/状态/键盘/焦点；默认对比度遵 PRD | 缺项计验收失败；不把扫描当人工辅助技术结果 | 原生 Control、语义状态、固定焦点身份、文本错误 | V-8、V-9 |
| WORK-N4 | PRD AC-1；独立 feature 构建 | 三模式分别生成/接管，默认 DOM 无 Makepad | 模式串用或初始 DOM 不一致则不通过 | 复用 docs-verify/hydration，增加原始入口 | V-8、V-10 |
| C-2 | runtime/listen、geometry、overlay | strict CSP 无新增违规、scope/实例无残留；不处理业务凭证 | 失效资源释放，保留其他实例；不放宽内联策略 | runtime abort、scope cleanup、CSSOM、无 innerHTML 注入 | V-8、V-9 |
| C-3 | 仓库热路径要求 | 当前吞吐/延迟/压缩增量未知，须同条件实测并解释差值 | 未解释回退或资源随交互增长不得记阶段完成 | 稳定键；数据变更时建索引；手势缓存几何；无每事件全量解析/规则增长 | V-1、V-11 |
| C-4 | 来源清单与单 CSS | 无 ref/在线源码依赖；class、目录、公开模式与实际一致 | source/css/catalog 漂移直接失败 | 正式 vendor、ALL/状态表、生成物、包 include | V-2、V-7、V-10 |

Select 的 entries 更新时构建一次 value→index 索引和可用项/归一化 text_value；导航复用该版本，不在每次 keydown 重新生成全部 DOM/CSS。choice 组把已接受集合建为 memoized set，Item 检索与 toggle 不在循环中反复线性 contains；回调向量按声明顺序单次生成。Slider 把外部接受值的排序/身份索引缓存到 Signal 版本；候选只移出活动项、二分插入并检查相邻间隔，不为每次 move 全量排序。回调构造的 O(n) 拷贝是输出向量本身的成本。每次动作只更新相关 thumb 与必要范围规则；数量变化才重建身份表，旧几何规则随 cleanup 移除。没有 SQL、Redis 或外部调用，不增加 TTL 配置缓存。

本期不承诺新的浏览器支持矩阵或绝对性能预算。V-11 沿现有 page timestamp/percentile 口径，比较旧功能同负载；新多值/组合负载单列。性能证据不提前关闭 P5 的整体 WORK-N2/AC-2。

## 8. 失败模式、切换与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 应用拒绝/延迟请求 | 当前控件 | 请求未成为接受值；不写坏 FormData | 显示仍为应用值，合法 focused Draft 除外 | 样本请求/接受值与 DOM/ARIA | 等应用投影；取消手势由应用撤回 preview | V-3—V-6、V-8 |
| 重复 submit、逆序校验、reset 后旧回包 | 当前表单 | 不能重复 save 或复活过期错误 | Busy/Waiting/Blocked/失败可见 | FormState 与保存计数、error source | 同一状态机防重，generation 失效；修改后重试 | V-5、V-8 |
| Select 触发器/活动项移除 | 当前浮层/表单 | 接受业务值不自动改变 | placeholder/活动项回退；失效层退出 | data issue、层深度、active descendant | 单次 close 请求并释放，回有效 trigger 或 scope fallback | V-6、V-8、V-9 |
| 选择/外点/Escape/自动聚焦被取消 | 当前最上层 | 无默认状态变化 | 原层/焦点保留，不穿透下层 | callback 次数、深度和焦点 | 等后续操作或程序关闭；清理不能被取消 | V-6、V-8 |
| 指针取消/捕获丢失/作用域释放 | 当前手势/scope | 无虚假 commit；接受过的 preview 由应用处理 | 当前接受值仍可见，其余区域继续 | commit/cancel 计数、pointer capture/监听统计 | 清 session/观察/样式；on_cancel 一次 | V-4、V-8 |
| 无效组合或数值/重复键 | 当前组件 | 不请求非法值 | 保留可读 children/最后有效状态并诊断 | PropError/data-rustify-issue；compile_fail | 应用修正 props；不得吞内容或 trap 整实例 | V-2—V-6 |
| wasm 实例致命故障 | 该实例全部 scopes | 该实例未持久化输入可丢失；Studio 已存主题不变 | 既有 fatal/重启；其他实例继续 | loader diagnostics/abort | 沿现有重启，不承诺同实例继续编辑 | V-8、V-9（复用未变边界并测新增资源） |
| 代码/CSS 不配对或 hydrate 初始状态不一致 | 当前消费页 | 无主题数据迁移 | 样式/接管验收失败 | CSS/feature/节点断言 | 回退同版源码+CSS并重建，不回滚用户主题 | V-8、V-10 |

切换顺序为“共用行为 → Themes/直接消费者 → 样式与资料 → 同版构建与阶段收口”。只有本期检查均通过才交付源码使用说明；无部署、灰度、数据库变更或不可逆操作。M5 收口失败时只修失败及波及项，未改变的已通过证据直接复用。

## 9. 验证

### 9.1 检查定义与时点

命令均从仓根执行；已启用 mise。编译/测试/Clippy 用 mbx。★ 测试入口是本计划交付的一部分，完成对应批次后才运行；不是现在已存在或已通过的命令。每个实现里程碑连续交付整批后做一次局部退出；续做不例行重跑。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | C-1、C-3；建立本期旧控件行为与相同负载性能基线 | `mbx test --locked -p rustify-components --no-default-features --features ssr --test ssr_controls`；§9.2 的旧消费者/测量基线 | M1 改受控实现前一次；仅旧行为刻画与改前测量，不跑全仓终验 |
| V-2 | WORK-F5/AC-1—AC-4 的基础、WORK-F1/AC-2；单元素诊断/方向/Portal/最小样式边界；C-4 输入可重现 | `mbx test --locked -p rustify-components --lib primitives::slot`；`mbx test --locked -p rustify-components --lib primitives::direction`；`mbx test --locked -p rustify-components --no-default-features --features ssr --doc`；`mbx xtask sources verify` | M1 Slot/方向/Portal、正式输入与独立 fixture 源码齐备后；运行时部分留 V-8 |
| V-3 | WORK-F6 全部 AC、WORK-F15/AC-2 选择部分；checked/mixed/组/卡片、受控拒绝和原生属性；同 form/无 form 两匿名组及跨 form 同名组的 SSR 初始选择、name 省略/保留与 group_id 稳定 | `mbx test --locked -p rustify-components --lib primitives::choice`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr choice`；`mbx clippy --locked -p rustify-components --all-targets -- -D warnings` | M1 选择 Primitive、Themes、表单镜像/局部样式及对应测试整批齐备后；和 V-2 合并退出 |
| V-4 | WORK-F7 全部 AC、WORK-F15/AC-2 范围部分；排序/间隔/身份/最后请求 commit/取消/非法数值 | `mbx test --locked -p rustify-components --lib primitives::slider`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr slider`；组件包 Clippy 同 V-3 | M2 Slider 行为、便捷包装、皮肤、测试和直接依赖迁移齐备后 |
| V-5 | WORK-F8 全部 AC、WORK-F15/AC-2 字段部分；提交/重置代次、来源错误、Draft/IME 边界；TextField Slot 独立 color/gap/px/pl/pr 的默认与 Responsive 声明；未绑定原生约束前置阻塞不进入 rules/save/on_asked | `mbx test --locked -p rustify-dom --lib form::`；`mbx test --locked -p rustify-dom --lib draft::`；`mbx test --locked -p rustify-components --lib primitives::field`；`mbx test --locked -p rustify-components --lib primitives::form`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr field`；`mbx clippy --locked -p rustify-dom -p rustify-components --all-targets -- -D warnings` | M3 字段/表单桥/校验与相关调用方整批齐备后；浏览器编辑保留 V-8/V-9 |
| V-6 | WORK-F9 全部 AC、WORK-F5/AC-1、AC-3—AC-4 的 Select/层钩子、WORK-F15/AC-2 Select 部分 | `mbx test --locked -p rustify-components --lib primitives::select`；`mbx test --locked -p rustify-dom --lib overlay::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr select`；DOM/组件 Clippy 同 V-5 | M4 元数据/部件/字符定位/镜像、Layer 双目标签名及取消/焦点/定位测试齐备后 |
| V-7 | WORK-F19/AC-1、C-4、C-6；真实样本/公开 props、迁移资料/目录与源码相符 | `mbx test --locked -p theme-studio --bins`；`mbx xtask catalog --write docs/components.md --check`；Agent 按 §9.4 独立公开资料角色检查 | M5 样本/资料/目录与直接消费者全部齐备后，先作局部静态退出；真实角色步骤并入 V-9 |
| V-8 | WORK-F1/AC-2、WORK-F5—WORK-F9、WORK-N1/AC-1、WORK-N4/AC-1 与 C-2；纯 DOM/无皮肤、FormData、拒绝/延迟/取消、RTL、多 scope、SSR/hydrate 及新增资源释放。同 scope/同 form 与无 form 的两匿名 Radio/RadioCards 组各自保留接受值，直接 `new FormData(form)` 无内部键；两个不同 form 的显式同名组各自提交接受值；SSR/hydrate 的 group_id/name/初始 checked 一致。未绑定及外部 form 关联的 required 控件前置阻塞时，原生 invalid target/message 与首错焦点可观察，rules/save/on_asked 均不调用 | 按 §9.3 构建；顺序运行 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives primitives.spec.ts`、`npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only forms.spec.ts dom-only.spec.ts`、`npx --no-install playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` | M5 唯一阶段收口；M1–M4 局部检查已通过，全部测试 fixture 与同版产品就绪后 |
| V-9 | WORK-F19/AC-1、WORK-F15/AC-2、WORK-N1/AC-1；全部声明状态/尺寸、独立操作者任务、现有嵌套和 GPU 兼容；TextField Slot 的独立 color、gap/padding、响应式、RTL 与调用方 utility 覆盖有实际样式断言 | §9.3 release 后：`npx --no-install playwright test --project=theme-studio theme-forms.spec.ts theme-scenes.spec.ts theme-projection.spec.ts`；`npx --no-install playwright test --project=component-catalog catalog-inputs.spec.ts`；`npx --no-install playwright test --project=property-workbench p2-form.spec.ts`；按 §9.4 合并走查 | M5 收口；单个 project 串行，实际 UI、错误/空状态、console/请求/CSP 一次合并检查 |
| V-10 | WORK-N4/AC-1、C-4—C-7；受影响构建/静态检查、SSR 回归、源码/CSS/资产一致 | §9.3 的模式与构建门；`cargo fmt --all -- --check`、`git diff --check`、`mbx xtask css --check`、`mbx xtask catalog --write docs/components.md --check`；§9.5 全量范围 | M5 收口，局部交付齐备后一次；不在中间里程碑反复全仓验证 |
| V-11 | C-3；同功能改前/改后 p50/p95、WASM/CSS gzip 与资源增量；新能力成本分列、无累积残留 | §9.2 两条带 PRIMITIVES_DIR/FORM_BENCH_PHASE 的固定命令（★spec 位于 tests/dom-only）；已有 GPU 主题性能只在投影热路径被实际修改时按原 headed spec 补测 | M1 只保存 baseline；M5 收口比较当前版本一次；新基线失效才补对应范围 |

批内最小反馈只回答具体失败/技术疑问，例如 `mbx test --locked -p rustify-components --lib primitives::slider::tests::commit_keeps_last_request` 或 DOM 的某个 form 代次测试；这些是★应编写的行为用例名。没有窄失败时继续完成实现，不把整包 Clippy/SSR/build 当逐编辑反馈。各批 class 修改完成后生成 CSS，下一批不重新跑未失效证据。

### 9.2 行为刻画、性能与证据

1. M1 在行为修改前扩展现有 DOM consumer 的一个小型 `form-benchmark` 入口，只组合已有 Checkbox/TextField/Slider/Select 与可观测的请求/接受值输出；不得先让 baseline 使用新行为。使用相同 loader/export-dom-assets 和既有 build-dom，保存构建到 `target/dom/dom-forms-baseline`。这只是测试 fixture 的公共入口，不新增生产 API。
2. ★ `tests/dom-only/forms-performance.spec.ts` 复用 `theme-performance.spec.ts` 的页面 performance.now、下一帧观察与 percentile 算法；通过真实浏览器输入操作驱动，计时在页面内开始/结束，不用 Playwright 操作耗时。共用负载固定 5 次预热、40 次采样、1440×900、同一默认主题与浏览器；这些是采样方法决策，不是性能 SLO。报告记录参数，baseline/current 不许改变。共同负载为原标量控件的请求→接受→DOM；新增三态/多滑块/部件负载单列 current-only，不拿不同功能互比。
3. baseline/current 页面暴露同一测试命名空间 `window.__form_bench`（reset、snapshot、测量记录），只是应用公共回调和 DOM 结果，不调用组件私有算法。两种产品由同一 driver 选择 URL；测量消费者的构建与页面不得混用。将基线应用源/配置和最终产品身份写进 M1，后续不覆盖 baseline 目录。
4. M5 使用相同环境与 driver 比较；页内至少报告 p50/p95、请求数/接受数/提交数与 idle 资源计数；WASM 和实际 CSS 以字节及 gzip 分开。CSS 原始皮肤与行为样式、新增外部输入分别列账，未解释回退/残留必须修复或由用户明确调整范围后才能通过。
5. 不新建通用压测平台。新 spec 受 docs config 的 `dom-primitives` 项目映射，平常 regression 不跑 evidence；server 从 `PRIMITIVES_DIR` 读取目录，默认仅供无皮肤测试。测量 baseline/current 均使用同一**有皮肤旧功能消费者**，避免把皮肤移除造成的体积差当性能改进；无样式产品单列 current 成本。固定命令如下，逐条串行执行：

```sh
# M1：增加旧 API 测量 fixture 后、改组件行为前
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml --output target/dom/dom-forms-baseline
PRIMITIVES_DIR=target/dom/dom-forms-baseline FORM_BENCH_PHASE=baseline RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives forms-performance.spec.ts
# M5：features 已构建最终 dom-consumer 后
PRIMITIVES_DIR=target/dom/dom-consumer FORM_BENCH_PHASE=current RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives forms-performance.spec.ts
```

spec 按 FORM_BENCH_PHASE 写入验证目录的 baseline.json/current.json，current 读取 baseline.json 做同条件比较；页面入口均为 consumer 的新增 forms.html。baseline/current server 串行复用 DOM_ONLY_PORT，不能覆盖 baseline 产物；原始 Primitives 验收不设置 PRIMITIVES_DIR。测量具体耗时未知，故障定位可缩小采样，完整证据须恢复上述参数。

### 9.3 构建与项目接线

- 新 `dom-primitives` 项目复用 docs config 的 domOnlyPort/DOM_ONLY_PORT（默认 4190），独立产品目录 `target/dom/dom-primitives`，可用 `PRIMITIVES_DIR` 指向测量产品；serversOf/wanted/project/webServer 同步配置。`dom-only` 排除两个新 primitives/performance spec，避免把有皮肤产品误作纯入口测试。新测试无需新端口。
- 构建：`mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml`；`mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml`；`mbx xtask docs-verify --group features`（既有入口构建三模式 hydration；本期在 features 中追加一次 primitives 的构建及图/产物检查）。阶段执行一次 features，已包含前两个 build-dom，不再单独重复执行；不新增依赖时间戳猜测产物新旧的缓存机制。
- 纯消费者必须仅 import primitives 并输出 runtime CSS + PRIMITIVES_CSS；检查网络和 CSS 内容无 sdk.css/rustify.css/theme-v4.css/品牌变量/全局 reset，WASM/图无 Makepad/React。另让外部小量 CSS 消费公开 data-state，证明不依赖内部层级。
- GPU 示例使用既有 release 入口：`mbx xtask build-web --example theme-studio --release`、`--example component-catalog --release`、`--example property-workbench --release`。只构建后续选定项目，serve 不重建。不并行 Playwright；已有 server 占用时先定位归属，不随便停止用户进程。
- SSR 部件：`mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr`；同一 SSR 命令将测试名换为 `ssr_controls`；hydrate fixture 先静态禁 JS可读，再接管逐节点比较并操作。SSR 原有 `theme_layout` 与 docs 专项各用自己的 ssr 构建。
- 现有 Node/Playwright、工具链和可指定本机 Chrome 的通道已存在；本期源码调查时没有可直接使用的默认 Studio release 目录。实施者重建后记录 build manifest/实际可见版本，不能用旧截图代验。需要的服务全部为仓内预览，不装后台服务。

### 9.4 真正浏览器与角色验证

V-8/V-9 的真实浏览器由 Agent 执行。独立开发者角色只可读公开 README、quickstart、迁移说明、目录与导出资料，在临时目录按锁定源码依赖编译一个“原始部件表单”和一个“Themes 表单”；禁止查内部实现补隐藏步骤。应得到可提交/重置、具名/错误关联、相同受控拒绝结果和独立 CSS；只记录角色模拟产物与卡点，不外推真人成功率。

最终使用者角色用 UI 操作：键盘完成组选择/跨越后固定 Tab 顺序/Select 字符定位、提交阻塞与重试；触控用同浏览器 touch pointer；拒绝/延迟与取消钩子分别观察请求数、接受值、焦点、FormData。原生剪贴板/选区/撤销通过真实键盘和系统剪贴板可用通道验证；自动 dispatch composition 只证明事件契约，OS IME、VoiceOver 与真机触控不冒充已通过。

视觉至少含默认浅/深、size/variant 穷举、局部 color/半径/scaling、状态及调用方 utility 覆盖；重用枚举矩阵，不做所有维度的无意义笛卡尔积。重点补 mixed/readonly/invalid/focus、TextField Slot 的默认/显式间距与颜色、响应式及 RTL、Select 长列表/空值/外点、同 form/无 form 匿名组和跨 form 同名组。390/768/桌面取同一部署，检查布局/可达性；200% 浏览器缩放可操作时实际执行，否则明确仅 reflow 模拟，完整人工辅助技术矩阵仍按 PRD P5 归档。

查看 console、失败请求、CSP、截图和 DOM/ARIA 实际结果；截图只留必要代表与故障证据。scope dispose/触发器移除/手势取消后观察监听、计时器、观察器、几何规则/层数不随轮次增加；实例 trap 只验证新增资源进入既有 abort 边界，不扩大 GPU 故障隔离承诺。

### 9.5 阶段静态/回归范围

阶段收口一次执行：

```sh
cargo fmt --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-components --no-default-features --features ssr
mbx test --locked -p rustify-ui --lib
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib
mbx test --locked -p rustify-docs
mbx test --locked -p rustify-docs-cli
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
```

workspace CSR 已含默认 SDK 测试，实际执行时去掉重复的默认 `rustify-ui --lib`，GPU/SSR 仍各自运行。Docs 与 CSR 分开；只改 P2 组件时不扩成 makepad 独立 workspace 全量测试、所有 docs-verify 组或完整 GPU 性能套件。新 sdk.css 必然影响 Docs 样式资产；在 style.rs 的静态 asset 表中注册两份新 CSS，并同步打包/复制检查。收口保留同版 `components-root` 的真实 islands 定向检查；不得因不改 Docs 实现而省略真实消费者验证。该检查并入 V-10，同环境一次完成。

已知成本：AGENTS.md 记录完整 property-workbench 约 38 分钟、fusion-basic 约 47 分钟，因此不把全项目用例放入各批退出；只运行 V-9 指定 spec。新增用例与构建耗时未知；记录实测，不承诺几分钟完成全部终验。缺 OS IME/VoiceOver/真机的部分在记录标 missing 并保留 P5 去向，不把它们新增成没有资源来源的本期真人招募门。

## 10. 里程碑与验收安排

仅一个阶段：可组合的表单，从无皮肤消费到真实 Studio/独立消费者。M1–M4 是实现批次；M5 合并样本交付与整个阶段收口。每批含行为、Themes、直接调用方、相应测试与生成样式；不是仅完成一个模块壳。局部通过不等于整期完成。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 无样式基础与可组合选择表单 | C-1 已核实；改前先 V-1 | T-1—T-3：正式参考/许可、Slot/Direction/辅助/Portal、独立消费者；三态/Radio/Switch/Toggle/组/卡片与 Themes。落点 §1.2 新 primitives、choice 和选择包装/CSS，照 Reset/现有 Checkbox、RadioGroup、pressed_after 改；契约 §5.1—§5.2、§5.6。共享 lib.rs、CSS 导入与来源锁由主实施者维护 | V-1、V-2、V-3；仅已具备行为与编译证据，运行时验收留 M5；回写「实施进度」 |
| M2 | 单值与具名多滑块 | T-2 的 Slot/方向/ID/表单属性契约及 V-2；不依赖 M1 卡片视觉收口 | T-4：★ primitives/slider.rs、现有 slider.rs/CSS、几何与直接测试；照 snap、现有 Slider 与 geometry 改，按 §5.3 固定身份/提交/取消。连续完成 scalar/多值及直接调用迁移；不改 GPU slider | V-4；受影响 CSS 已生成、标量直接消费者可编译；回写「实施进度」 |
| M3 | 字段、Draft 与原生 Form | T-2 的 ID/Control/原生属性契约及 V-2；选择镜像接入需 T-3/V-3 | T-5：★ primitives/field.rs、form.rs 与现有字段/form、DOM form/draft。照 input/textarea settle、Draft、FormState 代次改；§5.4/§5.6 定义字段与 TextField Slot 皮肤、错误/重置/提交。主实施者单写 FormState/共享绑定，测试与功能同批齐备 | V-5；原手工表单调用仍可运行、错误来源/accepted FormData 契约具备；回写「实施进度」 |
| M4 | Select 部件与可取消层行为 | T-2/V-2；T-5 的 FieldBinding/FormRoot 接入及 V-5 | T-6：★ primitives/select.rs、select.rs、overlay.rs 双目标/SDK 重导出。照原 Select active scroll、roving、Layer/route 改，契约 §5.5；不提前实现 P3 对话框/任意浮层。overlay 栈和钩子由主实施者单写 | V-6；元数据/表单镜像/层级取消/焦点契约与对应 Themes 同批可用；回写「实施进度」 |
| M5 | 真实样本、迁移与 P2 阶段收口 | T-3—T-6 与 V-2—V-6 全部具备；同版 CSS/模式 fixture 就绪 | T-7—T-8：★ FormSamples、Cards、目录/迁移资料；照 CardsScene/provider fixture 组合，按 §6。完善 SSR/hydrate/原始 consumer 与 project 映射；按 §9 合并真实 UI/静态/资源/性能终验，主实施者集成并核实产物身份 | V-7—V-11 全部适用检查通过；19 个 AC 逐行有证据、跨期终验不提前关闭；回写「实施进度」 |

提交点：**M5（全部里程碑与唯一阶段收口完成后，最终实现提交；随后按顶部协议单独提交 SHA 回填）**。中间没有独立发布或数据迁移门，不为汇报/切换任务增加提交前回归。

### 10.1 内部任务

任务用于解除精确前置与必要的独立调度；没有第二份任务进度表。M1 先完成改前 V-1；T-2 的编译/契约证据可提前解除下游，不必等待无关卡片截图。当前默认单实施者；若已获并行授权且工具可用，T-4 与 T-5 在稳定基础上可分离实现，但共享导出/CSS/浏览器必须由主实施者串行集成。

| 任务 ID | 所属里程碑 | 交付物/影响路径 | 前置成果 | 检查 ID |
| --- | --- | --- | --- | --- |
| T-1 | M1 | 改前行为/测量 fixture 与 spec、dom-primitives 项目/PRIMITIVES_DIR 接线、原产品保存；正式参考输入/许可与来源清单 | C-1 | V-1、V-2 |
| T-2 | M1 | §5.1 Slot/方向/Portal/辅助与最小 CSS、原始 consumer/编译负例 | T-1 的固定参考与 baseline 已保存 | V-2 |
| T-3 | M1 | §5.2 选择/组/卡片，共用行为+Themes+native form 镜像 | T-2 单元素/方向/ID 契约 | V-3 |
| T-4 | M2 | §5.3 scalar/多值/取消、Slider Themes 与直接消费者 | T-2 / V-2 | V-4 |
| T-5 | M3 | §5.4 字段/FormRoot、错误来源与 reset、§5.6 TextField Slot 视觉参数、旧表单适配 | T-2 / V-2；组镜像集成需 T-3 / V-3 | V-5 |
| T-6 | M4 | §5.5 Select 组合、原生镜像、Layer 原因/聚焦/实际定位 | T-2 / V-2；FieldBinding 接入需 T-5 / V-5 | V-6 |
| T-7 | M5 | §6 真实 FormSamples、公开入口/状态/迁移资料与 catalog | T-3—T-6 的公开接口稳定 | V-7 |
| T-8 | M5 | 三模式/无样式/既有消费、UI/性能/清理终验证据 | T-7 及 V-2—V-7；同版构建 | V-8、V-9、V-10、V-11 |

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| typed Slot 与动态 AnyView 的边界 | 一些旧自定义组件需把单原生节点交给 part | SDK 实施者按 §5.1 的类型接口与迁移示例交付，compile_fail 证明诊断；不默默接受多根 | T-2 局部退出 | 否，路线已定 |
| 多滑块稳定身份增加调用方键 | API 与参考数组形式不同 | SDK 实施者交 RangeSlider/标量示例；按 ADR-2，拒绝/延迟与跨越共同验收 | M2 退出 | 否，契约已定 |
| Layer 捕获路由变化波及原 Dialog/Menu/Draft | 取消与焦点回归风险 | 单一负责人修改，锁外回调、默认 hooks 兼容；V-6 与现有嵌套/手工表单回归 | M4 退出 / M5 真实收口 | 否，属于实现与验证工作 |
| 数值/体积基线未知 | C-3 尚无 P2 测量值，不能先说无回退 | 实施者 M1 留同条件 baseline，M5 比较；不改负载来掩盖回退 | 改行为前 / M5 | 否，测量路线明确且无需外部资源 |
| 当前默认 Studio release 尚未生成 | 无法拿现存页面直接证明新增 UI | 实施者按仓库 build-web/build-dom 重建；缺工具按 C-5 记录并请用户协助，不安装服务 | V-8/V-9 前 | 否，源码与构建入口已具备 |
| OS IME/VoiceOver/真机人工证据 | 不可从合成事件/axe 外推真实辅助技术 | 本期记录实际可用通道和 missing；完整辅助技术终验保留 PRD P5。若发现影响本期主要任务的真实缺陷仍修复 | M5 记录 | 否，不新增本期真人招募门 |

- 最终状态：**Ready**。
- 定级理由：P2 的产品/范围/数据所有权、公共接口、失败语义与消费方式均已明确；前置由当前仓内源码提供，无待拍板产品问题或外部契约阻塞。尚未实施的里程碑/验收不构成设计阻塞。

## 12. 已知坑与历史教训

- `AGENTS.md` 的“serve 不重建”：每次 Rust/CSS 改动后先构建对应 release，再验证页面身份，不能继续使用旧产物。
- `docs/architecture.md` 的 instance/scope 边界：trap 后 Rust cleanup 不运行；新增 Portal/手势资源必须可由实例 abort 释放，不能声称单 region trap 隔离。
- `input.rs`/`textarea.rs` 的 settle：只在值不同时写 property；无条件写入会影响光标/原生撤销，不能为了受控迁移丢失这个条件。
- `draft.rs` 的 Enter/blur、Escape 优先级与 `overlay::route`：关闭钩子要在层级拥有者处组合，不能让子节点取消失效或冒出第二次 commit。
- `textarea.rs` 的 SSR 前导 LF/CR 转义与现有 hydration 检查：文本初始 HTML 不是普通 value attr，迁移只换外观也会破坏接管。
- `form.rs::submit` 只清 Rule 错误：扩展来源时不能让重试提交清掉未修改值的应用/异步错误；reset 必须使旧代次失效。
- `sources.lock.json` 与 `xtask/src/sources.rs`：参考文件存在不等于正式输入；采用文件的 import_root/source_files/许可随包提供，构建不读 ref。
- [P1 补齐代码复审](DOM-COMPONENTS-P1-gaps.code-review.r2.md)的静态消费修复：迁移示例必须带真实 scope 标记并正确编码属性，不能只展示看似合理但 CSS 不匹配的根节点。
- `AGENTS.md` 的单项目 Playwright 规则：并行运行会互相清结果并耗内存；本期所有浏览器/基线产品串行复用端口，证据只对应实际运行版本。
