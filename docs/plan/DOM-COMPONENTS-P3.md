# DOM-COMPONENTS-P3 · Rustify UI · 可操作的 DOM 应用面板（同源 Primitives / Themes）

```formal
INTENT:
  GOAL: "直接依据本计划、仓库规则和当前代码实施或续做"
  SUCCESS: "全部里程碑退出检查通过，按‘实施进度’完成最终提交、SHA 回填与自查，并提示用户发起代码评审"

RULE @implementation_entry:
  MUST: "实施须知：先读下方‘实施者定位’‘实施进度’，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；goal 执行也遵循这些约定"
```

> **计划状态：Ready**
>
> 版本：v0.2。
>
> 调查基线：2026-10-31 · `614173ecc11354a7f5083bd0250849c90073141e` · main / dirty；源码无改动，当前未提交内容为本计划、评审报告及关联台账。
>
> 需求来源：[DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) v0.2 · **P3（第 3 期）**，§7.2 的 WORK-F10、WORK-F11、WORK-F12、WORK-F14、WORK-F15/AC-3、WORK-F17，共 12 条 AC。
>
> 本期交付：**内容切换与展开、普通/警告对话、辅助浮层、反馈及结构化内容组成的真实应用面板**。
> **开发范围：DOM SDK 的组件与必要浮层/通知核心、纯 DOM 消费者及静态接管 fixture、受影响示例和文档。依据见 §0。**
> 本期独特职责：复用 P2 的组合、方向、受控与表单能力，补齐面板交互；Primitives 和 Themes 使用同一行为实现。
> **顶层排除：新增 GPU 组件、菜单/网站导航体系、完整 Studio 组件陈列、全库迁移、请求时 SSR、公开发行。**

## 实施者定位

```formal
ROLE @implementer:
  AS: senior_software_engineer
  MISSION: "依据本计划、仓库规则和当前代码实施"

RULE @development_scope:
  MUST: ["仅实施本计划明确纳入的系统与改动；范围外依赖按 §4 的契约、提供方与解除条件跟进",
         =>[external_gap_blocks, "回写影响与继续条件，推进不依赖它的已授权工作"],
         =>[dependency_ready, "从当前阻塞移除，剩余项按本方未实现或验收未运行重新归因"],
         "新增开发对象须有用户明确依据；fixture 只覆盖约定契约的必要行为"]
  MUST_NOT: ["因源码可访问、自有服务、同仓目录、就地修订或委派扩大开发范围",
             "代建、改造或部署范围外依赖；将 fixture 做成替代外部系统的产品或冒充真实集成验收"]

RULE @communication_and_source:
  MUST: ["报告用路径、需求/检查/里程碑 ID 定位，不复述计划",
         "源码注释解释必要原因；追溯放记录或提交说明"]
  MUST_NOT: "在源码放需求编号、agent 标记或规划元数据"

RULE @identifiers_and_tasks:
  MUST: ["需求/约束/假设 ID 只在账本登记一次，上游 ID 保留原编号及来源；检查、里程碑、任务 ID 分别在验证表、里程碑表、按需内部任务表定义，其他位置仅引用或展开",
         "修订保留已有 ID，删除/拆分同步引用",
         =>[task_table_present, "按交付物、前置成果与检查 ID 推进，进展及证据写入所属里程碑记录，不另设任务状态表"]]

RULE @milestone_completion:
  MUST: ["本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口",
         "阶段终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收"]

RULE @role_acceptance:
  MUST: ["Agent 按 §9 指定的知识、资料及权限约束模拟上手与业务场景，记录实际结果与卡点",
         "有工具和权限的真实环境检查由 Agent 执行；真人公测指标按指定阶段另验"]
  MUST_NOT: ["角色模拟借内部源码或实现者补步骤", "将模拟结果写成真人数据或冒充本阶段真人指标已通过"]

RULE @implementation_route:
  MUST: ["直接遵循已定技术路线、接口、复用锚点和检查命令；未写明的实现细节按最近的同构代码就地决定",
         "调查只读本里程碑改动落点及其直接调用方和依赖",
         =>[plan_premise_refuted_or_route_infeasible, "停下，就地修订对应正文并在记录写明偏差；仅此条件触发路线重审"]]
  MUST_NOT: "无上述偏差而重新调研已定决策，或另起方案设计、比较、评判"

WORKFLOW @implementation_sequence:
  TRIGGER: on_implementation_start_or_resume
  DO: ["按 @resume_protocol 恢复，核对仓库规则、当前任务相关正文与依赖、工作树",
       "按 §10 连续实施批次推进，默认一个实现里程碑为一批，不按文件、函数、一次编辑或一轮 goal 对话切批",
       =>[behavior_tests_needed, "用当前行为的最小用例先红、再绿、再重构，红绿循环不附带整组回归"],
       "完成本批相关实现和测试代码，再按 @verification_rhythm 与 @reverification 做局部退出检查",
       "批次通过后继续下一可执行批次，里程碑完成按 @progress_writeback 回写，汇报或回答状态询问后继续实施"]

RULE @implementation_stop:
  WHEN: user_explicitly_pauses_cancels_or_switches_task
  MUST: "按 @writeback_timing 保存后停止"

RULE @verification_rhythm:
  MUST: ["批内只为具体失败、技术疑问或必要前置运行能回答它的最小测试/编译检查，无触发则继续实现",
         "§9 整组局部检查（含受影响模块回归与静态检查）等本批交付物齐备后统一运行，不因命令叫‘局部/快速’就逐次编辑重跑",
         "全量测试、端到端、真实环境与界面走查、故障注入、评测、干净构建仅在指定阶段收口运行，同环境走查合并",
         "下游前置、数据、安全、不可逆操作、发布及用户分期门及时验证，不等无关终验；稳定契约下可用 fixture 开发，不能替代真实集成",
         "优先复用增量构建、缓存和已就绪环境；调优用缩小版本迭代，达标后再跑完整检查",
         "遵守仓库或用户明确指定的检查门"]
  MUST_NOT: "无故清缓存、重启栈或重复启动尚在运行的检查"

RULE @reverification:
  MUST: ["证据仅在实际覆盖的代码、契约、配置、依赖或数据改变时失效；受影响检查合并到下一适用批次退出或阶段收口，不因失效立即停工",
         "同批连续改动只复验一次，未补证不能宣称新版通过",
         =>[downstream_needs_evidence_or_risk_gate_imminent, "先补必要检查"],
         =>[check_failed, "先定位原因并完成同因修复，再验失败项与修复波及项；未影响的已通过项直接引用"],
         =>[shared_contract_public_entry_build_or_dependency_changed, "核对波及面，必要时扩大复验，不把所有局部修复升级成整组或全量重跑"]]
  MUST_NOT: "仅因续做、汇报、回写或提交使证据失效"

RULE @parallel_delivery:
  MUST: [=>[&&[tasks_independent, "收益超过委派及集成成本", subagents_available], "使用 subagents，模型默认继承"],
         "任务说明只给目标/ID、稳定契约、可写与禁改路径、运行资源和检查",
         "共享文件单人负责，数据库、端口、浏览器等隔离或串行",
         "子 agent 返回结果、路径、验证证据与缺口，默认不另写任务文档；主 agent 审查并集成验收",
         "边界变化先回报，失败只阻塞依赖任务；结束对话前收集或停止仍在运行的任务"]
```

## 实施进度（实施期持续更新）

```formal
RULE @progress_entry:
  MUST: "本节是跨对话恢复的唯一汇总入口；协议文字固定，实施期只更新‘恢复快照’和‘完成记录’"

RULE @resume_protocol:
  MUST: ["依次读仓库规则、本节、当前任务相关正文与依赖，再核对 git status 和相关 diff",
         =>[records_conflict_with_worktree, "先查明原因，不凭记录覆盖用户改动，不凭代码存在推断验收已通过"],
         "仅在继续未完任务、核验前置验收、排错或审查时，按链接读对应记录的相关部分，不默认加载全部记录或原始日志",
         "从未完交付继续，不先例行重跑回归；检查前核对当前批次、触发原因与已有证据，未到验证点且无具体失败/疑问或必要前置则继续编码，已通过且覆盖未变的检查直接引用"]

RULE @writeback_timing:
  MUST: [=>[milestone_completed, "立即按 @progress_writeback 回写，再报告完成或提交"],
         =>[downstream_needs_partial_delivery, "所需前置检查通过并在记录保存证据即可推进"],
         =>["实质进展后受阻、暂停、交接、发现偏差或结束对话", "保存成果与缺口"]]
  MUST_NOT: "仅因普通子任务切换单独回写"

WORKFLOW @progress_writeback:
  DO: ["主 agent 更新对应记录，再同步快照变化字段和完成记录一行，不重写无变化内容或追加流水",
       =>["范围、决策、接口、数据、风险或退出条件有偏差", "就地修订对应正文，不追加勘误历史"],
       =>[source_cr_request_or_explicit_subscope_completed, "按 @source_cr_writeback 核对并回写实现结果"],
       "按 @project_index 同步已启用台账", "执行 @progress_consistency 自查"]

RULE @progress_format:
  MUST: ["快照写当前任务、进展与下一步，命令及证据只进记录",
         "首次回写删占位行，每个已记进展的里程碑恰好一行，状态仅‘进行中 / 阻塞 / 已完成’；完成时移至已完成行末，最近完成取该行",
         "更新时间带时区，摘要 1–2 句，记录填相对计划的链接，如 [M1 记录](FEATURE.records/M1.md)",
         "仅在本节快照、完成记录及对应记录文件内定位回写；里程碑表、检查表、ADR 等定义行不随进度改动"]
  MUST_NOT: "按里程碑 ID、‘状态’等同名字样对全文件查找替换"

RULE @source_cr_writeback:
  WHEN: requirement_source_explicitly_assigns_cr_requests_to_this_plan
  MUST: ["保留 CR/原 R/Q/RESP 语境，以既有需求/验证映射核对承接范围与收口检查，不因文档提及 CR 就改写",
         "对应请求或明确子范围实现且约定检查通过后，在本次进度回写追加结果，不等无关里程碑；最后一个里程碑完成时再核对漏回告",
         "通读来源 CR，核对最新有效请求、往返答复、计划范围与适用证据；部分承接或仍需其他计划时只回告已完成部分并列余项，不关闭整条请求或整份 CR",
         "复核实际回写文件最新内容，保留并发增量；文末追加实现结果 RESP，沿用编号并引用原请求、被更新的旧 RESP，说明被取代的旧‘未实现’判断；附实现行为、计划/里程碑、代码版本、验证结果、使用条件与剩余配合，不改正文或历史",
         "本方实现、发布生效、对方验收分别陈述，无证据不标已发布或已获对方验收",
         "同请求范围、实现版本、验证结论已有实质等价回告则跳过，不重复追加或更新时间；有变化只追加受影响范围，不因无关 HEAD 变化记新轮",
         "来源缺失、目标不唯一、只读或无法保真写回时，在对应记录保存可追加答复、原因与实际目标，快照下一步保留恢复动作；仅改工作副本须说明原件未改，不宣称回写完成",
         "写入后回读范围、编号及历史完整性，在里程碑记录保存实际 CR/RESP 链接，无需重新加载规划或答复技能"]
  MUST_NOT: [automatic_sending_deployment_or_platform_state_changes, disclose_sensitive_internal_evidence_externally]

RULE @milestone_records:
  MUST: ["按需建 <计划名>.records/M<n>.md；头部写计划链接、更新时区、状态与验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）",
         "正文只写交付行为/路径、必要偏差、未完项与继续动作；未完项含当前批次剩余交付、需复验检查及下次执行点，按回写时机保存，不每改一处记一笔",
         "按‘检查 ID/覆盖范围 | 命令或步骤 | 结果/必要证据’记录验收，共同环境与基线头部只写一次，有差异再单列",
         "同一测试组可覆盖多条退出条件，须逐条可追溯；引用原证据及复用依据，不复制断言全文、成功日志或调试流水",
         "证据只留命令、结果与必要少量输出，不为留证另写一次性脚本、哈希清单或大体量证据目录",
         "明确区分未运行、失败、环境缺失；仅复杂交接、排错或证据需独立保存时另建任务记录/日志/截图并链接"]

RULE @project_index:
  WHEN: "仓库已登记状态台账，或存在仓根 .xgent-ai/sdlc/protocol.md；兼容已有 docs/project-status/protocol.md，未启用则跳过"
  MUST: ["两套协议并存未切换时先核对，不双写；上述回写后读协议，只同步实际改变的当前状态与来源链接，必要时维护索引，默认直接更新本计划状态页",
         "来源直接链接计划快照/完成记录、里程碑记录的验收或未完项章节、已有评审/处置记录及其版本，不重写为 SDLC event、过程总结或证据副本，不先造 event 替代原记录",
         "里程碑推进、验证结果更新、来源版本变化、最终提交、整份计划完成均不单独构成新建事件理由",
         "新建前核对原记录及可取回版本；仅仓内明确要求不可变审计，或原记录与当前页无法保留并发/纠错事实时独立留存，说明依据与必留差异，其余直接引用",
         "旧协议强制事件时只留最小必要内容与原记录链接，不复述实施验收过程，不自行升级协议",
         "复用已有计划 ID，只更新本次范围，保留其他计划与各轮评审，不复制本节或里程碑记录；台账仅作索引，不替代恢复快照",
         "使用仓根相对路径与明确基线，未提交成果标明同步限制；登记失败说明缺口，不回退原成果"]

RULE @commit_protocol:
  MUST: ["在 §10 里程碑表后的‘提交点’提交，里程碑较多时在标出的阶段收口后提交，全部里程碑完成后必须提交一次",
         "仓库或用户对分支、提交信息、签名、是否由 agent 提交另有要求时从其要求",
         "提交前先回写进度；仅要求本次覆盖里程碑或阶段收口的检查，已通过且未失效的证据直接引用，不为提交重跑",
         "只暂存本计划相关改动，不纳入、不回退他人改动，不跳过提交钩子",
         "提交后把短 SHA 回填本次覆盖的各里程碑行‘提交’列，再单独提交只改计划文件的更新，该提交不回填自己的 SHA",
         "代码评审修复写‘修复’加短 SHA，UI 验收修复写‘UI修复’加短 SHA，均追加在原 SHA 后",
         =>[completed_before_commit_point, "对应行写‘待提交’"],
         =>[commit_unavailable, "写明原因并记入当前阻塞，不伪造 SHA"]]

RULE @completion_handoff:
  WHEN: last_milestone_completed
  MUST: ["依次按 @source_cr_writeback 核对 CR 回告、按 @commit_protocol 完成最终提交并回写 SHA，确认每个已完成行都有 SHA，再将快照‘下一步’写为代码评审；仍有 CR 回写缺口时一并保留",
         "随后报告完成，提示用户用 review-code 以本计划路径为参数评审；它从完成记录定位提交，并纳入未提交改动；评审由用户发起"]
  MUST_NOT: "实施中自行启动评审或修复评审意见"

REVIEW @progress_consistency:
  CHECK: ["每次回写后核对快照、完成记录、记录链接及本计划文件 git diff；非 Git 仓库对照回写前副本",
          "进度 n/N 的 N 等于里程碑总数，n 只计‘已完成’行；最近完成取最后一条已完成记录，不按编号推算",
          "链接指向已落盘文件；已完成行‘提交’为短 SHA 或‘待提交’，全部完成时每行有 SHA 且无‘待提交’",
          "本节外只有有意修订的正文，定义行未被进度文字改写"]
  EXPOSE: on_issue
  # 结构、计数与链接检查不证明验收真实通过。
```

### 恢复快照

- 最近更新：2026-11-03T13:52:21+08:00
- 当前进度：7/7 个里程碑完成
- 当前状态：已完成
- 最近完成：M7 · 完整任务与最终交付收口
- 下一步：由用户用 `review-code docs/plan/DOM-COMPONENTS-P3.md` 发起代码评审；人工 missing 按 P5 继续
- 当前阻塞：无
- 代码基线：`ec6bd506`；M5—M7 实现为 `ef8b0156`，CSS 整理为 `ec6bd506`；M2—M4 为 `4152c4c2`，M1 before 为 `3f23c064`；评审修复为 `3d078e29`、`f2bb1bb1`

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-01T00:07:45+08:00 | 六组旧面板负载及正式参考已冻结，V-1 通过 | [M1 记录](DOM-COMPONENTS-P3.records/M1.md) | `3f23c064` |
| M2 | 已完成 | 2026-11-01T00:46:19+08:00 | 同源切换/展开/导航交付，V-2 及两消费者定向浏览器通过 | [M2 记录](DOM-COMPONENTS-P3.records/M2.md) | `4152c4c2` |
| M3 | 已完成 | 2026-11-01T03:13:11+08:00 | V-3 通过；同源 Dialog/AlertDialog、生命周期/名称/取消/恢复与旧调用迁移齐备，真实两消费者及 Studio 定向验证通过 | [M3 记录](DOM-COMPONENTS-P3.records/M3.md) | `4152c4c2` |
| M4 | 已完成 | 2026-11-01T06:08:43+08:00 | V-4/V-6 通过；辅助浮层、Arrow/hover/typed 迁移和完整任务面板齐备，两消费者及旧入口验证通过 | [M4 记录](DOM-COMPONENTS-P3.records/M4.md) | `4152c4c2` |
| M5 | 已完成 | 2026-11-01T06:45:10+08:00 | V-5 通过；内容/媒体/原生滚动与样本、静态语义和参数齐备，两消费者构建通过，真实浏览器留 M7 | [M5 记录](DOM-COMPONENTS-P3.records/M5.md) | `ef8b0156`、整理 `ec6bd506` |
| M6 | 已完成 | 2026-11-01T07:23:10+08:00 | V-7 通过；唯一通知调度、加载反馈、直接调用及两消费者构建齐备，真实行为留 M7 | [M6 记录](DOM-COMPONENTS-P3.records/M6.md) | `ef8b0156` |
| M7 | 已完成 | 2026-11-01T18:38:29+08:00 | V-8—V-14 通过；真实内容/反馈/完整任务、接管、调用方与性能闭合，原生wheel一帧回退已修正，人工missing归P5 | [M7 记录](DOM-COMPONENTS-P3.records/M7.md) | `ef8b0156`、整理 `ec6bd506`；修复 `3d078e29`、`f2bb1bb1` |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

本计划是既有 SDK 的 DOM 增量与必要重构。功能终验仅承接下表的 12 条 P3 AC；质量要求作用于本期增量，跨期需求仍在 PRD 指定期次终验。来源均为 [PRD §3、§7.2](../PRD-DOM-COMPONENTS.md)，不复制验收原文。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F10 | 功能 | PRD / AC-1、AC-2 / P3 | Tabs、Accordion、Collapsible、SegmentedControl、TabNav 各按自身语义工作，动态项有焦点去向 | §5.2；V-2、V-6 | 已确认，本期终验 |
| WORK-F11 | 功能 | PRD / AC-1—AC-3 / P3 | 可组合普通/警告对话，取消、嵌套、scope 模态与焦点恢复 | §5.3；V-3、V-6 | 已确认，本期终验 |
| WORK-F12 | 功能 | PRD / AC-1、AC-2 / P3 | Popover、HoverCard、Tooltip 的操作、悬停、定位与内容边界 | §5.1、§5.4；V-4、V-6 | 已确认，本期终验 |
| WORK-F14 | 功能 | PRD / AC-1、AC-2 / P3 | Progress、Spinner、Skeleton 与可配置 Toast 通知 | §5.6；V-7、V-10、V-11 | 已确认，本期终验 |
| WORK-F15 | 跨期功能 | PRD / 仅 AC-3 / P3 | 本期内容、反馈、切换和浮层的全部声明尺寸/变体有效 | §5.7、§6；V-6、V-8、V-10、V-11 | 本期验 AC-3；整条 P4 终验 |
| WORK-F17 | 功能 | PRD / AC-1、AC-2 / P3 | 结构化内容、比例/分隔/滚动与既有选择卡片 | §5.5；V-5、V-8、V-11 | 已确认，本期终验 |
| WORK-F1 | 前置契约 | PRD / AC-2；P2 §5.1、§5.6 | 无样式消费路径继续用于新增交互部件 | §2、§5.7；V-6、V-8、V-10、V-12 | 前置复用，整条 P4 终验 |
| WORK-F4 | 前置契约 | PRD / AC-2；主题 provider 与 overlay | 局部主题/方向及浮层继承创建位置的有效快照 | §4、§5.1；V-6、V-11 | 已有契约，回归本期影响 |
| WORK-F5 | 前置契约 | PRD / AC-1—AC-4；P2 | 单元素组合、取消、方向、稳定部件状态 | §5.1；V-2—V-6、V-10 | 前置复用，不重结算 P2 |
| WORK-F6 | 前置契约 | PRD / AC-1—AC-3；P2 | SegmentedControl 及卡片使用已有单选/选择语义 | §5.2、§5.5；V-2、V-8 | 前置复用，不另造选择状态 |
| WORK-F19 | 阶段边界 | PRD / AC-2 在 P4、AC-3 在 P5 | 本期提供真实消费者面板；完整 Studio 参考陈列和 Inspector 全覆盖留原期次 | §0.5、§6 | 非 P3 终验项 |
| WORK-F23 | 阶段边界 | PRD / AC-1 已交付，AC-2、AC-3 在 P5 | 同步本期公开入口、能力表和必要旧→新说明；全库迁移不提前认领 | §3、§6、§9.4 | 本期跟随文档，整条 P5 终验 |
| WORK-N1 | 可访问性 | PRD §3.2；AC-2 在 P4、AC-3 在 P5 | 本期名称/角色、键盘、焦点、宣告及默认对比度；真人辅助技术缺证单列 | §5、§6、§7；V-6、V-8、V-10、V-11 | 本期增量必须满足，非整条终验 |
| WORK-N2 | 性能 | PRD §3.2；AC-2 在 P5；既有 forms-performance spec | 改热路径留同负载前后数据、压缩体积、空闲与释放结果 | §7、§9.2；V-1、V-13 | 本期增量测量，非整条终验 |
| WORK-N3 | 隔离/安全 | PRD §2.7、§3.2；运行时契约 | strict CSP、scope/实例边界、正常清理与 fatal 清理 | §5.1、§8；V-6、V-11、V-12 | 本期影响回归；AC-3 在 P4 终验 |
| WORK-N4 | 消费模式 | PRD §3.2；AGENTS.md「仅 CSR/SSG」 | DOM 默认无 Makepad；构建期静态 HTML 与确定性 hydrate 分别构建，资源外置 | §3、§5.8；V-9、V-12、V-14 | 本期增量检查，整条 P5 终验 |
| WORK-N5 | 可维护性 | PRD §3.2；catalog.rs | 能力、默认值、差异、引用入口和实际交付一致 | §3、§6；V-14 | 本期同步，整条 P5 终验 |
| WORK-N6 | 可靠性 | PRD §2.7、§3.2 | 触发器消失、取消与失败不遗留遮罩/焦点限制，不改业务事实 | §5.1、§8；V-6、V-11 | 本期影响回归，整条 P4 终验 |
| WORK-N7 | 视觉/响应式 | PRD §2.5、§3.2；DESIGN.md | 现有中性编辑壳；两模式、长内容、窄屏、放大与减少动效下可操作 | §5.7、§6；V-6、V-8、V-10、V-11 | 本期影响回归，整条 P5 终验 |
| WORK-N8 | 来源 | PRD §1.5、§3.2；sources.lock.json | 采纳参考及许可正式入仓，正常构建不依赖 ref/网络抓取 | §1.2、§4；V-1、V-14 | 既有来源规则覆盖本期新增输入 |
| C-1 | 前置 | P2 恢复快照、代码复审 R3；§4 的源码 | 使用当前 source SDK/CSS/CLI 配对；公开 0.2.1 不能代表当前 v2 源码 API | §3、§4；V-1、V-12 | 已核实足够 |
| C-2 | 运行时 | AGENTS.md「运行时契约」；architecture.md | 应用持有业务值/open；保留 Draft、scope、实例 abort 与现有 GPU 接入 | §5、§8；V-6、V-11、V-12 | 已确认 |
| C-3 | 工具/环境 | mise.toml、package.json、AGENTS.md | mbx 编译；固定 Rust/Tailwind；单项目串行 Playwright，不安装服务/升级 major/另开分支 | §9、§10 | 已确认 |
| C-4 | 样式/组合 | AGENTS.md「编码与样式约定」；sdk.css、geometry.rs | 单份 Tailwind、无全局 reset、class 可覆盖、动态 CSSOM、资产不嵌 wasm | §5.7；V-8、V-12、V-14 | 已确认 |
| C-5 | 交付边界 | PRD §1.3、§7；AGENTS.md | 不新增 GPU、请求时 SSR、服务、账号或发行；保留扩展，不改写 DataTable | §0.5、§1.2、§10 | 已确认 |
| C-6 | 验收边界 | PRD §6.3、§7.1；AGENTS.md「浏览器测试」 | Agent 走真实主路径；OS IME/VoiceOver/真机触控缺失保留 missing，不冒称真人通过 | §9.3；V-11 | 已确认 |

无支撑本设计的未证实重大假设；运行耗时及性能差值在实施测量前为未知，不用猜测的规模或 SLO 决定架构。

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 包与入口 | 留在 rustify-dom / rustify-components；新增 primitives 子模块与同名 themed 部件 | 不增加 crate、feature 或依赖服务；继续现有单行为/两种外观模式 | WORK-F1、C-1；primitives/select.rs、select.rs |
| D2 | 浮层职责 | 扩展现有 OverlayStack；组件只提供受控状态、部件与自身默认策略 | 模态、定位、最上层仲裁集中；公共行为改动需覆盖旧 Layer 消费者 | WORK-F11、WORK-F12；ADR-1 |
| D3 | 通知状态 | core 提供唯一调度器及无样式部件；ToastHandle 是应用使用的便捷状态持有者 | 声明式 Root 仍受控；ToastRegion/Toaster 不另设队列或计时器 | WORK-F14、C-2；ADR-2 |
| D4 | 滚动实现 | 原生 scrollport + 同步的可组合滚动条部件 | 保留原生滚动/键盘，补齐可拖动 Thumb、方向与显示策略；替换旧皮肤实现 | WORK-F17；ADR-3 |
| D5 | 便捷入口 | Tabs/Dialog/Tooltip/Progress/ScrollArea/Toaster 改为新部件组装 | 仍有实际用途的便利 API 保留；被替代的内部行为删除，不保留双实现 | WORK-F23；§3 |
| D6 | 样本归属 | styled 与 raw DOM 消费者承载本期完整面板；Studio 只适配直接影响 | 本期能独立验收；完整 Studio 参考矩阵仍在 P4 | WORK-F19；PRD §7.2 |
| D7 | 静态消费 | 内容/切换/反馈的确定初态生成 HTML；浮层关闭、运行通知为空 | 不支持任意初始已打开 Portal；hydrate 保留原节点与 ID | WORK-N4；architecture.md「Static scopes and hydration」 |

### 0.3 ADR-lite

#### ADR-1：由现有浮层栈拥有跨组件的层级与焦点

- 状态：Accepted。
- 背景与驱动：Dialog 内已有 Select；P3 再加入 Popover、HoverCard、Tooltip。WORK-F11/WORK-F12 要求同一层级的取消、主题与清理语义。
- 备选 A：各组件各自维护定位、outside 与焦点监听。可直接实现、单模块初始改动小，但嵌套时必须另设协调协议，事件与恢复规则散落。
- 备选 B：扩展现有 OverlayStack，统一注册父子关系、交互根、定位结果和焦点域。改动面包含现有 Layer 消费者，但已有 runtime 释放/观察/取消机制可复用。
- 决策：选 B；§5.1 定义增量，Select 与旧 Region anchor 不换行为实现。新共享浮层部件只集中四类真实调用方的 Root/Trigger/Content 生命周期，不做通用组件框架。
- 正面后果：同一事件仅由合适的顶层处理，父层知道后代 Portal，正常清理与 abort 沿现有链路。
- 负面/中性后果：需要覆盖旧 Layer 的 modal、非 modal、GPU rectangle anchor 和 Draft 优先级；不能仅测新组件。
- 重新评估触发：本期 scope 内无法表达新的实际层级关系，或实测定位/焦点开销无法满足已有负载；不是因假设中的跨页面浮层重写。

#### ADR-2：通知数据与呈现调度分离，保留一个计时事实源

- 状态：Accepted。
- 备选 A：只扩展 ToastHandle，把开放状态与倒计时都留在服务中。现有调用改动少，但直接组合的受控 Root 还需第二条计时/队列路径。
- 备选 B：core 的 Provider/Root 注册表只持临时调度状态；开放事实来自 Root 的 Signal。ToastHandle 持有便捷消息列表，ToastRegion 把列表投影到同一个 Provider/Root。
- 决策：选 B。ToastHandle::show/dismiss/current 保留用途，移除其旧 hide_after 定时实现；核心通知部件由 primitives 重导出，Themes 只加皮肤。
- 正面后果：受控拒绝关闭、排队、暂停、滑动和键盘进入只有一套行为；core 不反向依赖组件 crate。
- 负面/中性后果：便捷通知从实际进入可见槽位才开始计时；单独的静态 Toast 改用明确的内联反馈或公开 Root，迁移见 §3。调度可见性不等于业务 open 状态。
- 重新评估触发：出现需要跨 scope/持久化的通知产品需求；本期不建设消息中心或跨页任务系统。

#### ADR-3：滚动条部件驱动真实的浏览器 scrollport

- 状态：Accepted。
- 备选 A：继续只用原生滚动条，公开 orientation/size。改动与维护成本最低，但无法提供稳定参考中的 Scrollbar/Thumb/Corner 组合及 hover/scroll 显示行为。
- 备选 B：Viewport 仍由浏览器滚动，独立 Scrollbar/Thumb 读取其尺寸/偏移并写回 scrollLeft/scrollTop。成本是一次测量、pointer capture 和清理；不接管 wheel 惯性或构造虚拟内容。
- 决策：选 B；这是 WORK-F17 和完整稳定参考的本期交付，不把原滚动条只换色声称为全部能力。参照本地 scroll-area.tsx 的比例、RTL 和 18px 最小拇指算法，保留 Boundary 语义。
- 正面后果：原生滚动、键盘和内容可访问性仍成立，部件可以独立定制。
- 负面/中性后果：需处理溢出、resize、拖动取消和 RTL；不承诺浏览器系统滚动条的像素一致。
- 重新评估触发：真实兼容环境证明自绘滚动条破坏滚动或无障碍；先修复实际路径，再依据产品范围决定是否调整公开能力。

### 0.4 职责与事实所有权

|  | 应用/消费者 | DOM core 与 Primitives | Themes 与示例 |
| --- | --- | --- | --- |
| 拥有 | value/open、业务确认和提交、通知文本/重要程度/持久操作、是否接受请求 | 临时焦点/悬停/手势、ID/部件关联、层级/定位/释放、通知调度 | 声明的视觉参数、真实组装、演示状态、文案与资料 |
| 不拥有 | 浏览器资源释放和组件内部层级算法 | **替应用确认危险操作、持久化业务数据、擅自修正业务值、重写宿主主题** | 第二套选择/浮层/队列行为、另一份主题事实源 |

命令与业务结果由应用决定；组件发请求并投影接受值；core 负责其实际创建的浏览器资源正常释放及 abort 释放。

### 0.5 明确不在本期

- P4 的 DropdownMenu/ContextMenu/Menubar/NavigationMenu/Toolbar 及完整 Studio 参考陈列；TabNav 只在本期实现真实链接导航条，不提前建设完整导航系统。
- P5 的全库公共入口迁移、工作区/大数据重构、整套人工辅助技术及全部部署/性能终验；仅做本期改动所需适配与回归。
- 不新增 GPU 控件、GPU P3 色域或 DOM/GPU 布局/视觉对等。保留现有 GPU 共享主题与 region anchor；不修改 Makepad 渲染器。
- 不改主题文档 schema、本地库/导入迁移、账号云端、后台服务、业务权限、React 运行时、unstable 参考组件或永久兼容层。
- 不实现请求时 SSR、流式 SSR 或 SSR 服务部署；host 的 ssr feature 只验证 SSG 构建期同步 HTML。不开发布提案、不部署官网、不发 crates.io。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** P2 [恢复快照](DOM-COMPONENTS-P2.md#恢复快照)为 5/5；[代码复审 R3](DOM-COMPONENTS-P2.code-review.r3.md#总体结论与下一步)通过 CSR/SSG 修复范围。当前 `primitives::Slot` 通过类型限定一个 native target，Select trigger_as 保留目标监听并在上层处理默认行为；DirectionProvider 使用独立 Owner，Portal 保留 scope 与主题。它们足够支撑本期组合基础，不把 P2 UI 自检当作独立 UI 复审通过。
- **[已核实·足够]** `OverlayStack::route/arm/refresh_inert` 已集中处理编辑优先级、顶层 Escape、按需页面监听与本 scope inert。Layer 使用 resize/tree/scroll 观察，失效 anchor 会释放注册并只请求一次关闭；主题取创建位置的 resolved signal。`Layer` host/browser 签名已有 Cancelable hooks、direction 与实际 Top/Bottom 反馈。
- **[已核实·缺口]** Layer 仅提供上下放置；非 modal 包装为 presentation，modal 的 Tab 监听限自身 DOM；outside 只有 pointer。→ 增加 §5.1 的交互面板、父子关系、焦点 outside 和四边定位/箭头。→ 否则非模态对话语义、嵌套 Portal 焦点与 WORK-F12 的配置不能成立（`crates/rustify-dom/src/overlay.rs`）。
- **[已核实·缺口]** Dialog 固定标题/关闭栏；Tooltip 在 span 上立即响应进入/离开，没有可组合 Root/Content 或过渡容忍。→ 同源部件与组件专属策略。→ 否则无法自定义对话操作、警告语义或从触发器移至卡片（dialog.rs、tooltip.rs）。
- **[已核实·缺口]** Tabs 的箭头直接以 active 值计算并请求激活，仅自动模式；TabPanel 保留 DOM 并 hidden。→ 分开焦点与选中状态、补手动/RTL/动态恢复，保留隐藏面板的状态。→ 否则应用拒绝/动态移除时 roving 与业务值混淆（tabs.rs、roving.rs）。
- **[已核实·缺口]** core Toasts 只有一个 showing、替换和绝对到期时间；ToastRegion 只有 polite live region。Progress 的 aria-valuenow 使用百分比但 aria-valuemax 使用原 max。→ 唯一队列调度器、重要程度/暂停/动作/手势，以及进度原单位 ARIA。→ 否则通知能力不足且非 100 max 的辅助语义错误（toast.rs、progress.rs）。
- **[已核实·缺口]** ScrollArea 是原生 overflow-auto 单 div；catalog 中 Avatar/Badge/Callout/Card/DataList/Table/AspectRatio/Separator/Skeleton 等仍明确无独立入口。→ 实现 §5.5/§5.6，不把 Studio 的手写 article/span 当 SDK 交付（scroll_area.rs、catalog.rs、examples/theme-studio/src/scenes.rs）。
- **[已核实·足够]** geometry 的 StyleSet/GeometryStyles、组件颜色 helper、P2 选择卡片以及独立 CSR/SSG/hydrate fixture 已存在；固定构建/浏览器入口见 §9。调查未重新运行产品测试或浏览器，不据此宣称本期行为通过。

### 1.2 拓扑与文件清单

★ 为新增；其余均为本次已定位的现有落点。新测试/示例文件只承担本期已定义的能力，不做新的产品。

| 系统/模块 | 改动路径与符号 | 复用锚点与边界 |
| --- | --- | --- |
| DOM 浮层核心 | `crates/rustify-dom/src/overlay.rs` 的 Layer/OverlayStack/Side/LayerDismiss/place；必要时内部拆 ★ `crates/rustify-dom/src/overlay/placement.rs`，不改变 module 公共路径；`crates/rustify-dom/src/lib.rs` 重导出 | 保留 Anchor::Region、现有 hooks、runtime listeners/resize/abort；只加 §5.1 所需行为 |
| 原生链接接缝 | `crates/rustify-dom/src/router.rs` 的 anchor_target | TabNav 标记仅使其原生 a 跳过 owner/guest router；普通 scope 链接契约保留，不建设导航框架 |
| 通知核心 | `crates/rustify-dom/src/toast.rs`、core i18n.rs、lib.rs；Cargo.toml 仅补实际用到的 web-sys feature | ToastHandle/ToastRegion 上演进；SDK facade 已 glob 重导出，不复制类型 |
| 交互原语 | `crates/rustify-components/src/primitives/mod.rs`；★ 同目录 `tabs.rs`、`accordion.rs`、`collapsible.rs`、`floating.rs`、`dialog.rs`、`alert_dialog.rs`、`popover.rs`、`hover_card.rs`、`tooltip.rs`、`avatar.rs`、`progress.rs`、`scroll_area.rs`、`aspect_ratio.rs`、`separator.rs`、`toast.rs` | select.rs/browser.rs 的 Owner、typed target、缺上下文诊断/注销；choice.rs 与 roving.rs。toast.rs 只重导出 core 无样式部件 |
| 主题组件 | 现有 tabs.rs/dialog.rs/tooltip.rs/progress.rs/spinner.rs/scroll_area.rs/toast.rs；★ 同目录 `accordion.rs`、`collapsible.rs`、`segmented_control.rs`、`tab_nav.rs`、`alert_dialog.rs`、`popover.rs`、`hover_card.rs`、`avatar.rs`、`badge.rs`、`callout.rs`、`card.rs`、`data_list.rs`、`table.rs`、`aspect_ratio.rs`、`separator.rs`、`skeleton.rs`；lib.rs | select.rs、checkbox_cards.rs 的行为投影与 theme/color/geometry。表格不碰 data_table/；卡片复用既有选择组件 |
| 样式/目录 | primitives.css、sdk.css、styles.rs、catalog.rs；★ `crates/rustify-components/css/panels.css`、★ `content.css`、★ `feedback.css`；重新生成 rustify.css/candidates.json 与 `docs/components.md` | 三个皮肤文件放在现有 css 目录；结构 CSS 不含主题；Catalogue 保持既有 24 分类并扩展独立能力项 |
| 真实 DOM 消费者 | `tests/dom-only/consumer/src/lib.rs`、`tests/dom-only/primitives/src/lib.rs` 及各自 export-dom-assets.rs；★ 两侧各自 `src/panels.rs`、`src/content.rs`、`src/feedback.rs`；★ `tests/dom-only/consumer/src/panel_benchmark.rs`、★ `tests/dom-only/consumer/public/panels.html`、★ `tests/dom-only/consumer/public/panels.js` | 在现有两个产物挂载真实能力；raw 只加载 PRIMITIVES_CSS；styled 测试页加载现有 CSS，不增 crate；xtask 已复制包内 public/ |
| 静态接管/测试 | `tests/dom-only/hydration/src/view.rs`，★ 同目录 `panel_parts.rs`；★ `crates/rustify-components/tests/panels_ssg.rs`、★ `content_ssg.rs`、★ `feedback_ssg.rs`；既有 ssr_controls、missing_context_ssr | 同步 SSG 原节点/ID、延迟 children 关联；不引入流式服务测试 |
| 浏览器接线 | `playwright.docs.config.ts`、`playwright.config.ts`；★ `tests/dom-only/panels.spec.ts`、★ `content.spec.ts`、★ `feedback.spec.ts`、★ `panels-performance.spec.ts`；hydration.spec.ts；★ `tests/browser/theme-panels.spec.ts` | 扩展已有 dom-only/dom-primitives/dom-hydrate 项目；不另占端口、不并发 Playwright |
| 受影响调用方/资料 | component-catalog 的 main.rs/theme_fixture.rs；theme-studio 的 scenes.rs/provider_fixture.rs/data_tools.rs/storage_controls.rs；必要的现有测试调用；README、组件 README、forms.md、navigation.md、themes.md、architecture.md、dom-components-migration.md；★ `docs/panels.md` | 只改本期实际接口/行为影响和示例；保留 Studio 工作台/字体及已有 GPU 预览 |
| 来源/CI | `sources.lock.json`；在现有 `crates/rustify-components/vendor/radix/` 增补 ★ P3 采用的 props/CSS/行为源；`.github/workflows/verify.yml` | 参考范围与版本见 §4；CI 把新增测试接到现有模式/浏览器门，不引入另一套流水线 |

不改 makepad/、GPU 控件实现、主题文档 schema 或发布脚本。若兼容构建暴露调用方受公共签名影响，仅作对应参数适配并记录路径，不顺带重写业务。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| OverlayStack / Layer | 旧 Layer、Select；本期浮层部件 | 应用控制存在；所有页面资源归 scope/instance；语义回调在锁外 | 现有 Anchor/Callback，无新 adapter | 父子层级、输入仲裁、焦点与定位/清理 | 进程内 + 真实浏览器 | 取消/嵌套/回焦/失效/旧 region anchor |
| primitives::floating | dialog/alert_dialog/popover/hover_card/tooltip | 独立 Owner 的 Root；同一受控 open；Trigger/Content 不另存副本 | typed trigger_as、公开 Cancelable hooks | 部件注册、可访问关联、默认策略投影 | 进程内；真实 Layer | 原始部件和 Themes 的同输入结果 |
| 切换与展开 | Tabs/Accordion/Collapsible、应用 | 值来自外部；临时焦点可独立；按语义公开部件 | typed target + children | 项注册、roving/隐藏与动态恢复 | 进程内；复用 roving | 静态 HTML、手动/自动/RTL/拒绝 |
| 内容/滚动 | 原始部件、Themes、静态消费者 | 原生语义；滚动事实来自 Viewport；图片状态非业务值 | children/原生属性；没有数据 adapter | 图片结果代次、滚动比例/指针、结构关联 | 进程内 + 浏览器本地图片 | 错图、长内容、table/dl、滚动 |
| core toast Provider/Root | ToastRegion、primitives::toast | open 外部所有；队列只调度呈现；一个到期器 | 公开 on_open_change/取消/操作 | 剩余时长、可见槽位、暂停/滑动、焦点入口 | 进程内 + runtime 资源 | 受控拒绝、FIFO/替换、暂停恢复、清理 |
| Themes | 消费者/示例 | 单份 primitives 行为；声明矩阵与实际 CSS 一致 | class/typed native target | 尺寸、色板、几何、部件视觉 | 现有 CSSOM/静态 CSS | 两模式、参数矩阵与 utility 覆盖 |

删除 floating 或通知调度模块会把同一层级/计时策略分散回多个真实调用方，因此保留；纯转发的单部件不再抽 factory、服务层或可插拔后端。测试使用真实浏览器与本地资产，不用替身浮层/滚动条冒充运行验收。

## 3. 公共 API 与迁移

没有持久化 schema 或数据回填。现有 `ThemeDocument` v2、作者值、草稿与导出格式不变。

1. 新组合入口按 `rustify_components::primitives::<component>::{Root, ...}` 和 `rustify_components::<component>::{Root, ...}` 组织；静态内容直接提供具名组件。所有新增公共 props、枚举及事件载荷在 §5 定义，不复制 React Context/ref/defaultValue API。
2. Tabs(active/tabs/on_activate)、TabPanel、Dialog(open/title/description/...)、Tooltip(open/label/...)、Progress(value/max/indeterminate)、ScrollArea(boundary/...)、ToastHandle::show/dismiss/current、Toaster 保留具名用途，内部改成新部件组装。Tabs 新增 activation_mode/direction；Dialog 新增 hooks/parts；Tooltip 的便捷入口使用单 native target 的 typed 组合，现有仓内单按钮调用迁到 trigger_as，片段不静默加语义错误的 wrapper。允许该签名破坏并给编译器可定位错误。
3. ToastOptions 保留 `duration_ms` 与 tone，增加 importance、可选 title/action；`duration_ms=0` 明确为不自动关闭。新增 ToastConfig/ToastQueuePolicy，默认 Replace、visible_limit=1，保留现有单条通知默认；FIFO 为显式配置。现有不带 `..Default` 的字面量同步补字段。Toast 快照保存上述选项和 seq/message；含 callback 的选项/快照仅要求 Clone，不再要求 Copy/Eq。旧静态字面量在 fixture 改用 Callout 或公开的受控 toast::Root，不能让失去上下文的关闭按钮伪装可用。
4. Progress 保留便捷参数，映射到 `primitives::progress::Root(value: Option<f64>, max)`；修正原单位 ARIA 后同步旧测试，不能保留错误的百分比断言。
5. 目录与迁移指南逐项写旧调用→新部件组装、事件/焦点/关闭差异、结构 CSS 与主题 CSS、默认参数；包含 §5.1 局部模态省略 aria-modal 的属性变化及 §5.6 通知换宿主的层级规则。保留便捷入口不等于保留旧内部实现。受影响调用方同批切换，源码/CSS/调用方作为一个 source 版本使用，不称已包含于公开 0.2.1。
6. 回退用对应提交的源码与 CSS/示例一起回退。无业务持久数据逆迁移；不发明新消费者版本号，正式发行由既有流程另行决定。本期指南使用实际 source revision，并明确公开 registry 包未包含本次 API。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P2 组合/选择/表单 | 已核实足够 | primitives/slot.rs；select.rs 的 trigger_as、Content、缺 Root 分支；choice.rs；P2 恢复快照与 R3 | 使用这些具体 API；SegmentedControl 映射 radio_group；选择卡片不重写 | 非法组合可读且诊断，不 panic；拒绝值不临时显示 |
| Layer/主题/runtime | 已核实缺口 | overlay.rs 的 route/arm/LayerResources/Layer；theme/provider.rs 的有效快照契约；runtime::defer_after 不提供取消句柄 | 范围内补 §5.1；可撤销短计时器参照 Select SearchTimer，用 clearTimeout + release_on_abort；不改 host protocol | scope 清理销毁；实例 fatal 不能再回调 wasm；anchor 失效请求一次并撤下失效层 |
| 正式参考输入 | 已核实可取 | PRD §1.5 的本地 Themes 3.3.0 / Primitives 聚合包 1.8.1；本次读过 P3 props、核心交互源和既有 vendor/sources 格式 | M1 正式纳入采用文件及依赖的 base-tab-list/base-card 等局部样式。新增来源项 `radix-themes-panels` / `radix-primitives-panels`，逐文件记录本地相对路径/摘要、包版本、许可、改写说明；上游 commit 未知如实为 null | ref 只供导入/调查，后续正常构建不得读它；来源缺失不标能力已交付 |
| 结构 CSS/主题 CSS | 已核实足够 | styles.rs 的 PRIMITIVES、SDK constants；geometry::StyleSet/GeometryStyles；component_color::set_color/use_mode | 加三份皮肤与必要结构规则，扩大已有 include/目录。使用同一主题 snapshot，不新增 theme schema | 非法 CSS 值沿既有 PropError/diagnostics；保留可用默认，宿主不被重置 |
| DOM/SSG/hydrate | 已核实足够 | xtask/src/docs.rs 的 features/build_hydration；hydration-harness 的独立 features 与 render_scope | 扩展 fixture 与检查，复用现有 4190/4194；不要求新服务 | 浮层初态打开时报 StaticRenderIssues；身份不匹配沿现有拒绝，初态一致由 fixture 验证，不宣称通用自动检测 |
| Studio/Catalog/Docs | 已核实可接 | 已有 Dialog、Tabs、Tooltip/Toast 直接调用；catalog 独立能力表；Docs 读取 styles 常量 | 必要参数适配、关联说明与定向真实走查；P4 才扩完整 Studio 陈列 | 保留原工作流；缺 GPU 对应如实声明；不修改发布/站点系统 |

没有需要其他仓库、账号、服务接口、数据库或外部部署才能解除的开发依赖。P2 UI 复审未完成和 P5 人工 missing 不阻止复用已可核实的组合/表单契约，也不据此宣称上游全部验收结束。

## 5. 核心机制与公共接口

### 5.1 组合、层级、定位与焦点

**共同约定。** 沿 P2 的独立 Owner、typed Slot/trigger_as、attributes 透传、清理注销和 `data-rustify-part`。原生目标监听先运行；`default_prevented` 后不再执行组件默认动作。离散请求一次；没有 default_open/default_value 通道。状态标识取实际接受值：`data-state`、`data-disabled`、`data-orientation`、`dir`，浮层另有实际 `data-side/data-align`。缺 Root、重复部件、重复业务 key/缺必要名称分别给可定位的 `data-rustify-issue`；保留可读 children，失去契约的操作禁用，不 panic/静默丢弃。typed 单元素入口沿 Slot 的编译期约束，不能把 erased fragment 当单元素接受。

浮层部件的固定结构为 `Root(Trigger, Portal(Overlay?, Content(..., Arrow?)))`。Portal 创建唯一 Layer 并接受 `force_mount=false`；Content 在其独立 Owner 构造时登记 panel ref、position/hooks/名称，Layer 的浏览器 effect 等有效 panel 就绪才注册和度量。Content 不再创建第二个 Portal/Layer。Overlay 与 Content 是同一 Layer 内的兄弟；不能把遮罩塞进 role=dialog 的交互根。遗漏专属 Portal 的 Content 给 missing-portal 诊断并保留可读内容，便捷组件总是组装完整结构。下文浮层的 force_mount 均在其 Portal 上配置。

**core 类型与 Layer 增量。** host/browser 的 Layer props 同步；core lib 重导出中立类型。以下是新增签名，不删除旧 Anchor/LayerPlacement：

```rust
// Side 扩展为 Top / Right / Bottom / Left；Align 仍为 Start / Center / End。
pub enum ViewportAlign { Start, Center }
pub struct CollisionPadding { pub top: f64, pub right: f64, pub bottom: f64, pub left: f64 }
pub enum Sticky { Partial, Always }
pub enum PositionUpdate { Optimized, Always }
pub struct FloatingPosition {
    pub side: Side, pub align: Align,
    pub side_offset: f64, pub align_offset: f64, pub arrow_padding: f64,
    pub avoid_collisions: bool, pub collision_padding: CollisionPadding,
    pub collision_boundary_ids: Vec<String>, pub sticky: Sticky,
    pub hide_when_detached: bool, pub update: PositionUpdate,
}
// Anchor 增加 Viewport { align: ViewportAlign }，Centred 与 Region/Element 仍有用途。
// Layer 新增 active: Signal<bool> (=true)，panel / arrow: Option<NodeRef<html::Div>>，
// trigger_id: Signal<String>（静态 String 可 into），以及 position: Option<Signal<FloatingPosition>>。
// LayerDismiss 增加 FocusOutside；以下 hooks 为 Option<Callback<Cancelable<LayerDismiss>>>：
// on_focus_outside / on_interact_outside；已有 Escape/pointer/auto_focus hooks 保留。
```

- `position=None` 保留旧 fit 与旧 Region anchor 的坐标语义；本期 anchored parts 显式给 position。默认 FloatingPosition 为 Bottom/Center、offset/padding=0、avoid_collisions=true、Sticky::Partial、hide_when_detached=false、Optimized。Select 继续显式 Bottom/Start，沿旧 min-width/viewport 行为，不能因新增默认值漂移。配置数值有限、padding 非负；非法字段沿公开 prop 诊断保留最后有效配置，初始非法用已说明默认，不向样式插入 NaN/任意片段。boundary ID 只解析本 scope 内实际节点，非法/失联边界诊断，不把 document/body 当新的主题或资源所有者。
- 纯定位函数输入 anchor rect、panel/arrow 尺寸、可见 viewport 与边界交集、direction 和 FloatingPosition，输出坐标、实际 LayerPlacement、可用尺寸、箭头偏移/隐藏。先算请求边侧与逻辑 align，再比较请求/对侧溢出，避免碰撞时选较少溢出的一侧，同量保留请求侧；沿交叉轴 shift。Top/Bottom 的 Start/End 随 RTL，Left/Right 的 Start/End 为上/下，不翻转物理 side。Partial 限制 shift 使面板仍与 anchor 相连，Always 允许继续 shift；箭头沿面板边夹在 arrow_padding 内，无法指向 anchor 则隐藏。碰撞结果、CSSOM 和 data-side 来自同一次输出；不维护独立“预测边侧”。
- 可见边界用 visible viewport 减去 scrollbar，并与显式 collision boundaries 相交。oversize panel 暴露可用宽高变量，由结构 CSS 限高/局部滚动，关键操作可达。居中对话用 Viewport Start/Center，外侧保留现有 viewport-minus-2rem 约束；不使用带 backdrop 的外框尺寸作为对话内容尺寸。
- 新对话的有效居中/Start 区域及遮罩限定为 scope container 的可见矩形与 viewport 的交集；Overlay 的定位/尺寸由同一 scope 几何投影，不继续使用覆盖整个页面的 fixed inset-0。对话内容也限制在该区域，保证相邻 scope 与宿主仍可点击/聚焦。没有可见 scope 区域时不创建拦截全页的遮罩，按锚点不可用处理；旧的直接 Layer/Region 坐标契约保留。V-6 同时验证指针命中和键盘边界，不能仅凭本 scope inert 属性推断隔离。
- Layer 的外框负责定位/层序/清理，`panel` 指定实际测量、outside、名称/角色与焦点节点；不传 panel 的旧调用继续以外框为交互根。新 Dialog/Popover 的 role/ARIA 在 Content 本身，外框与遮罩为 presentation，避免重复 dialog 或把遮罩当内容。新增结构规则若落 runtime planes，同时更新 `crates/rustify-dom/web/dom-runtime.css` 与 `web/runtime.css`，不引入 GPU 皮肤。
- `modal` 只表示本 scope 的交互/焦点约束，不表示整页模态。Dialog、AlertDialog 和 modal Popover 的 Content 均省略 `aria-modal`；旧直接 Layer 的 modal 分支及 Dialog 便捷入口也移除自动输出的 `aria-modal=true`，保留对应角色与名称/说明。不通过 inert 宿主页扩大边界。命名、安全初始焦点与 scope 内约束参照 APG；因宿主和其他 scope 仍可交互，不宣称符合 APG 整页模态模式，实际辅助技术表现另留人工证据。
- 每个 Layer 记录内部 `parent: Option<LayerId>`。创建 Owner 的 layer context 携带有效父层引用；构造部件时取得引用，在 active 且 panel 就绪、实际注册时读取并绑定有效父层，不按 DOM 祖先、视觉邻近或当前焦点猜测。普通层引用指向所属 Layer，通知引用由 §5.6 动态更新；已注册通知子树的迁移也按该节处理。后代 Portal 的挂载节点在所属父 Content 的专用 children host 中，仍归原 mount scope；同一模态的后代内容属于它的 DOM/焦点子树。children host 不参与父面板度量、不裁切固定浮层，不使用会建立 fixed containing block 的 transform。top modal 按存活 modal 的登记次序取最后一层，迁移不改变该次序；后代的显示层级与输入优先级高于祖先，同父层后登记的兄弟在上。inert 只作用于 top modal 域外的交互分支，不能整体 inert 包含它的祖先 host 而连带禁用后代；层序只在所属 scope 内产生作用。
- 父层失效或被移除时，先按 §5.6 迁走仍存活的通知子树，再撤下其余后代的注册/捕获/观察，最后处理父层恢复；仍被应用保留 open=true 的失效后代等待有效父宿主，不成为无主全页层。Layer 同步投影创建处 locale 的 lang、direction 与 theme snapshot，局部语言/主题改变时更新这些属性，清理不写宿主根。
- 全局监听仍由 stack 按需持有；输入顺序保持 composition/native Draft → 合适的最上层 → 应用。Escape hook 取消也消费这一层，不能继续关父层。outside 只对 top 层判断 panel/后代/关联 trigger 之外的目标：先专属 pointer/focus hook，再共享同一取消状态的 on_interact_outside，最后才请求关闭。pointerdown 后同一目标引起的 focusin 合并，避免一次外部交互请求两次关闭。程序 `open=false`/close_top 不伪造用户取消事件。
- modal Dialog/Popover 的 focus outside 默认阻止关闭并在本 scope 内恢复其焦点域；公开 hook 可以取消关闭请求，不能借此解除 modal 的 inert/Tab 约束。非 modal 才按 focus outside 策略关闭。scope 外宿主/其他 scope 的交互可按所属层策略请求关闭，但不能取消目标的原生操作、回抢其焦点或将其 inert；模态焦点约束本身只在本 scope 内执行。
- 模态约束在 scope 捕获阶段统一执行 Tab/focusin，并使用 top modal 的 Content 与后代域；两个嵌套模态只约束最上面的一个。普通非模态不阻止 Tab 离开，可按其组件策略请求关闭。保留实际 Tab stops 的原生 disabled/hidden/inert/radio/form owner 规则；不用查询整个页面作为焦点域。
- 接受打开后才运行可取消 open_auto_focus；关闭被接受、active 转 false 或实际卸载时运行 close_auto_focus，恰好一次。默认回焦顺序：仍 connected、可见、非 disabled/inert 的显式 Trigger → 有效的打开前元素 → 最近存活父层的可用焦点 → scope container。外部 pointer/focus 已移到有效目标时不抢回；Tooltip/HoverCard 永不默认移焦。所有焦点判断以实际 live DOM 为准。
- `active=false` 或 force_mount 的关闭内容不注册 stack、不占模态/焦点、不运行定位；仅保留 hidden/inert DOM。anchor/必需 Trigger 断开是生命周期失效：立即隐去失效内容、释放注册/观察/计时/捕获，必要时恢复焦点并请求 close 一次。应用拒绝关闭也不能留下死遮罩；有效 Trigger 重新出现且 open 仍为 true 后才恢复。scope cleanup 与 instance abort 各沿既有链路，不承诺同实例 fatal 后仍能操作。
- Optimized 复用 stack moved、ResizeObservation、MutationObserver，实际滚动/resize/内容变化才脏；同一帧合并读几何和写坐标。Always 是显式开放层逐帧更新，关闭即停；不把该选择的预期开销说成空闲泄漏。新计时/RAF/observer 使用现有 runtime 资源或 Select SearchTimer 的 Drop + release_on_abort 形式；不新增常驻轮询、宿主协议或无法撤销的延迟链。

### 5.2 切换、展开、状态选择与链接

| 入口 | 关键参数/部件 | 确定的行为 |
| --- | --- | --- |
| `primitives::tabs` / Themes `tabs` | `Root(value: Signal<Option<String>>, on_change: Callback<Option<String>>, entries: Signal<Vec<PanelEntry>>, orientation=Horizontal, activation_mode=Automatic, loop_focus=true)`；List/Trigger/Content；`PanelEntry { value: String, disabled: bool }` | Trigger role=tab，List=tablist，Content=tabpanel；独立临时 focused key，箭头/Home/End 移焦，自动模式请求激活；手动模式只在 Enter/Space/点击请求。水平键尊重 direction，垂直不吞左右键 |
| `primitives::accordion` / Themes `accordion` | `Root(value: Signal<AccordionValue>, on_change: Callback<AccordionValue>, orientation=Vertical, collapsible=false, disabled)`；`AccordionValue::Single(Option<String>) / Multiple(Vec<String>)`；Item(value,disabled)/Header(level)/Trigger/Content | Single 最多一个，collapsible=false 时已打开项不请求清空；Multiple 可分别展开/折叠。Header 是原生 h1–h6，Trigger 为 button/aria-expanded/controls；每个可用 Trigger 都可 Tab，方向键/Home/End 为附加移动 |
| `primitives::collapsible` / Themes `collapsible` | `Root(open: Signal<bool>, on_open_change: Callback<bool>, disabled)`；Trigger/Content | 单个展开面板，原生按钮与区域关联，拒绝请求时状态/hidden 保持应用值 |
| Themes `segmented_control` | Root 的 value/on_change/name/form/required/disabled/read_only/invalid、Item(value)；视觉见 §5.7 | 将 Option 单选值投影到已有 primitives::radio_group；单选不可通过再次点击清空。使用 radiogroup/radio，不制造 tabpanel，表单成功值跟随应用 |
| Themes `tab_nav` | Root(aria_label,视觉参数)；Link(href,active: Signal<bool>,target,rel,download,children)；typed link_as | nav 内真实 a，active 对应 aria-current=page。保留 modifier/new-tab/native navigation；不设 tablist/tabpanel，不拦截上下文菜单或普通链接行为 |

新部件的 Trigger 提供与 Select trigger_as 同构的 native button 入口；Accordion/Collapsible Content 提供 `force_mount=false`，Tabs 默认保留面板 DOM/内部状态。关闭/未选内容必须 hidden/inert，不在普通 Tab 或可访问内容中留下活动控件；force_mount 也不等于可交互。主题值和应用状态均不因查看面板重置。

所有 key 唯一、非空；重复项诊断并禁用相应非法项。ID 由 scope seed 和内部单调分配序列生成，业务 value 不拼成 DOM selector。Root 为静态 entries 分配关联，Trigger/Content 使用同一映射，延迟 children、条件渲染/顺序变化不再各造一份 ID。Accordion 的 Item 局部 Owner 持 Trigger/Content 关联；登记/注销以当前存活部件为准。

当前 focused 项删除/禁用时，仅在焦点属于该组或因该节点删除失去焦点的情况下，按原索引选下一个可用项、再前一个、最后可程序聚焦的组根；组外焦点不被抢回。当前业务项失效不擅自写 value：Tabs 没有匹配 panel 时为空，roving 仍可从首个可用 Trigger 进入；Accordion 只显示仍存在且被接受的项。应用明确更新 value 后正常投影。这一路径在 raw/Themes 上使用同一规则。

### 5.3 Dialog 与 AlertDialog

- 两层公开 `dialog::{Root, Trigger, Portal, Overlay, Content, Title, Description, Close}`；Root 接收 `open: Signal<bool>`、`on_open_change: Callback<bool>`、`modal: bool=true`。Content 接收 outside/Escape/auto_focus hooks；Portal 配置 force_mount；Trigger/Close 有 native button 的 typed `_as` 入口，保留调用方事件与属性。组件不附带第二个 button。
- `alert_dialog` 复用同一 floating 生命周期，公开 Root/Trigger/Portal/Overlay/Content/Title/Description/Cancel/Action，始终为 scope 局部 modal。Content 的 role=alertdialog，Title 与必要 Description 显式关联；普通 Dialog 不论 modal 值均为 role=dialog。两者均按 §5.1 省略 aria-modal，可访问名称不能为空。Title 默认 h2，语义级别可选，不用视觉 size 决定标题级别。
- 普通 Dialog 接受打开时聚焦第一个可用项，无项则 Content；可取消 open_auto_focus 并由调用方选择长内容标题。AlertDialog 默认聚焦 Cancel，缺 Cancel 诊断且聚焦 Content，绝不默认聚焦危险 Action。焦点与命名参照 [APG modal dialog](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/) / [alertdialog](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog/)；与其整页模态模式的差异是本 SDK 只约束本 scope，宿主和其他 scope 保持可达，因此不声明整页模态属性。
- AlertDialog 的普通遮罩/外部 pointer/focus 不关闭，更不触发确认；Escape 仅请求取消关闭，允许公开 hook 阻止。Action 增 `on_action: Callback<Cancelable<()>>`：原生目标 handler → on_action → 未取消时一次关闭请求。只有 Action 发确认回调；Cancel、Close、Escape、外部或程序关闭都不伪造确认。应用拒绝关闭时内容保留，可继续修改/取消。
- 非 modal Dialog 不挂载 Overlay；modal 的 Overlay 只是一份拥有该层 ID 的遮罩；pointer outside 由 stack 集中处理，不能再用 backdrop click 多发一次。普通 Dialog 的 close_on_backdrop=false 映射为取消外点默认；AlertDialog 的安全默认不能被“自动确认”替换。
- 独立 Content 清理在其回调 Owner 释放前调用 core 的 `overlay::on_layer_content_cleanup()` 撤销所属层；唯一存活 Content 重新投影 panel、align 和 hooks，重复部件修复后不保留失效引用。打开 auto_focus 延至 DOM 可见后调用，默认普通对话复用 core `overlay::focus_first`；重开后跳过旧 session 的默认回焦。
- Title/Description 的关联随实际部件挂载/移除变化；说明缺省时不引用不存在节点。Root 无 Trigger 的程序打开是合法路径，返回打开前的有效元素；多 Trigger 使用诊断而非猜选返回位置。动态卸载和嵌套关闭按 §5.1。
- 原 Dialog 便捷视图使用同一部件组装既有标题/关闭栏；新 Content 不强制标题栏布局。不能把新库面板强行应用到 Studio 的中性编辑壳，preview dialog 仍取创建处主题。

### 5.4 Popover、HoverCard 与 Tooltip

| 入口/参数 | 行为与默认策略 |
| --- | --- |
| `popover::{Root, Trigger, Anchor, Portal, Content, Close, Arrow}`；Root(open,on_open_change,modal=false)，Content(position,hooks) | 指针点击或 native button 的 Enter/Space 请求打开；Content role=dialog，modal=true 也只约束本 scope 并省略 aria-modal；进入其可用焦点，Tab 在非 modal 时可离开并请求关闭；Escape/外部交互可取消；显式 Anchor 与 Trigger 各有职责，位置使用 Anchor，回焦使用 Trigger |
| `hover_card::{Root, Trigger, Portal, Content, Arrow}`；Root(open,on_open_change,open_delay_ms=700,close_delay_ms=300)，Content(position,hooks) | Trigger 支持原生 a，保留 href/native link；hover/focus 请求打开，pointer 在 Trigger 与 Content 之间移动时取消关闭计时；保持文字选区/内容 pointerdown 时不关闭；不默认移焦、不把补充资料变为唯一关键动作 |
| `tooltip::{Provider, Root, Trigger, Portal, Content, Arrow}`；Provider(delay_ms=700,skip_delay_ms=300,disable_hoverable_content=false)，Root(open,on_open_change,可选 delay override) | hover 延迟、focus 即时请求打开，Provider 在同域连续触发时跳过延迟；离开两端、blur、激活或 Escape 请求关闭。Escape 后保持“本次触发已关闭”直到重新进入/聚焦，避免同一 focus 立刻重开；hoverable 默认可停留阅读 |

延迟默认值来自本地 hover-card.tsx / tooltip.tsx；它们是临时交互计时，不能先把 open 候选写成本地显示值。所有 pending timer 在相反输入、close、卸载、scope cleanup、instance abort 时取消。移动到内容的过渡区域采用触发器出口点与目标矩形的凸包判定，复用本地 Tooltip 的 pointer grace 算法；只在实际 transit 时监听 pointermove，进入两端即解除，不为整个页面常驻轨迹监听。

Tooltip Content role=tooltip，Trigger 的 aria-describedby 与已有说明去重合并，关闭移除自身 ID；内容禁止可聚焦操作，非法交互子项给诊断。HoverCard 的辅助内容保留选择和链接阅读路径，不使用 tooltip 角色冒充交互面板。Tooltip/轻量悬停内容不抢焦点，Escape 关闭不把焦点跳到页面容器，参照 [APG Tooltip](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/)；该 APG 页面仍标为工作草案，不作为整体认证依据。

便捷 Tooltip 的新签名接收 `trigger: HtmlElement<E, At, Ch>`，以及既有 open/on_open_change/label/class/test_id；泛型约束同 Slot。其内部调用 Root/trigger_as/Portal/Content。Dialog/Popover 的 trigger_as 限 native button；HoverCard 使用 native anchor；Tooltip 可装饰调用方提供的单个 native element，调用方确保该元素具备适用的键盘入口/名称。仓内旧 children 写法按迁移指南转为 typed trigger 或显式 parts，不再用 span 掩盖多目标。

Arrow 是真实定位部件，使用同一几何结果转向，不能画固定在底边的假箭头。触发器滚动、resize、字体/内容尺寸改变后走 §5.1 定位；方向、theme snapshot、locale 和层级继承创建 context。触控不依赖 hover 获取关键内容：Popover 有点按入口；HoverCard Trigger 保留直达详情的链接；Tooltip 的关键信息还应在可见名称/说明中提供，样本据此设计。

### 5.5 结构化内容与滚动

| 入口/结构 | 数据、失败及语义 |
| --- | --- |
| `primitives::avatar::{Root, Image, Fallback}`；Themes Avatar 及对应 parts | Image 接收 src/alt、原生图片属性，Fallback 接收 children、delay_ms=0；Image 状态 `Idle/Loading/Loaded/Error` 与 on_loading_status_change 属呈现临时状态。src 改变递增代次，旧 load/error 不覆盖新图；缓存图片在 attachment 检查 complete/naturalWidth。SSG 固定保留可读 fallback，hydrate 后才切呈现；非装饰头像有清楚名称，装饰图 alt 为空不重复读 |
| Badge、Card | 默认 span / div；提供 typed native target 装饰入口以保留需要的链接/语义属性。Card 不自动加点击、tabindex 或 button；真正交互由调用方原生元素承载，不鼓励嵌套交互元素 |
| `callout::{Root, Icon, Text}` | Root 是带静态说明的容器，显式 `role` 可选 status/alert，默认不主动宣告；状态含图标或文字，不能只靠颜色。Icon 装饰时 aria-hidden，Text 保留原文，不把所有 callout 都变成紧急 alert |
| `data_list::{Root, Item, Label, Value}` | dl / div / dt / dd；多项标签和值保持原生定义列表关联，允许长值/多个 dd；不重排成无语义 grid。Item align、Label width/color 见 §5.7 |
| `table::{Root, Header, Body, Row, ColumnHeaderCell, RowHeaderCell, Cell, Caption}` | 原生 table/thead/tbody/tr/th/td/caption；列/行表头分别 scope=col/row，可透传 colspan/rowspan/headers。外部局部 scrollport 承担横向溢出；不增加 role=grid、排序/虚拟化或 DataTable 状态 |
| `primitives::aspect_ratio::Root` / Themes AspectRatio | ratio:f64=1，有限且 >0；使用 geometry 的外部规则/CSSOM 设置 aspect-ratio。非法输入诊断、保留有效值，初始非法回到 1；图片由 URL 引用，不嵌资源 |
| `primitives::separator::Root` / Themes Separator | orientation 与 decorative；decorative=true 时 presentation/aria-hidden，false 时 separator/aria-orientation；不制造键盘 stop |
| 选择卡片 | 直接使用既有 CheckboxCards/RadioCards Root/Item/Label/Description/Indicator；对 name/value、禁用、描述关联和长内容做组合回归，不复制一套新卡片选择状态 |

**ScrollArea。** 无样式与 Themes 都公开 Root/Viewport/Scrollbar/Thumb/Corner；便捷 ScrollArea 以相同部件组装。Root 接收 `scrollbars: Vertical/Horizontal/Both=Both`、`visibility: Auto/Always/Hover/Scroll=Hover`、`hide_delay_ms=600`、现有 Boundary、aria_label、方向；Themes 另有 size/radius。Viewport 是唯一真实 overflow 元素与键盘入口，名称可由 label/labelled_by 给出，Root 不额外增加重复 tab stop。默认方向不允许的轴不出现伪滚动条。

- 用 Viewport 和 content/track 的 resize/scroll 事件更新尺寸与偏移；一个变更帧统一度量。Thumb 长度为 track 可用长度 × viewport/content，最少 18px 且不超过 track；偏移按最大 scroll 与可用 thumb 行程线性映射，无溢出时隐藏或禁用该轴，不除零。
- 横向 RTL 统一转换为逻辑距离，再转换回浏览器的 scrollLeft；读取/写入共用同一映射。拖动 Thumb 保留 pointer 在 thumb 内的抓取偏移；点击 Track 按当前位置移动；pointerup/cancel/lostcapture/卸载均释放 capture 与短监听，不提交任何业务 value。
- Auto 仅在溢出时展示，Always 常显，Hover 在 root hover/focus 时展示并按延迟隐藏，Scroll 在实际滚动期间展示、hover/focus 保持；减少动效取消位移/淡出动画，不能隐藏当前拖动目标。
- 保留原生 wheel/touch/inertia 与 Viewport 的箭头/Page/Home/End 滚动；Boundary 只决定边界 overscroll。隐藏 native scrollbar 的规则仅作用到这个 Viewport；自绘条不是第二份滚动事实，不捕获整个页面的 wheel。键盘无需操作 Thumb 才能到内容。

### 5.6 反馈与通知

**Progress。** 原语 Root(value: Signal<Option<f64>>, max: Signal<f64>=100, value_label: Option<Callback<ProgressValue,String>>)/Indicator；`ProgressValue { value: Option<f64>, max: f64 }`。有限 max>0，Some(value) 需有限且在 0..=max；无效值给 prop 诊断并呈现 indeterminate，无效 max 用 100，不改应用值。aria-valuenow 为原单位 value，aria-valuemax 为 max，视觉宽度才用 value/max；None 无 aria-valuenow，data-state 为 indeterminate/loading/complete，aria-valuetext 使用显式 value_label。名称通过 aria_label/labelled_by。便捷 Progress 的 indeterminate=true 映射 None。

**Spinner / Skeleton。** Spinner(label、loading: Signal<bool>=true、size、可选 children) 在 loading 时提供唯一可读 status 和装饰图形，保留 children 的占位尺寸、隐藏其朗读与交互；loading=false 恢复 children/无内容时不留 status。label 缺省从 core locale 获取 Loading；不要继续固定英文。Skeleton(loading=true、layout: Signal<Layout>、children) 加载时 hidden-to-AT/inert，不重复朗读或留下可聚焦子项；结束后原 children/ID/业务状态继续，不重建输入；恢复其键盘可达性，不增加异步自动回焦。减少动效时图形静止/无 shimmer，加载事实仍可读。Progress 的不确定动画 duration 采用结构化 `duration_ms`，沿 CSSOM 写样式；不是虚假的完成百分比。

针对已有单 native 元素，公开 `spinner_as` / `skeleton_as(target: HtmlElement<E,At,Ch>, ...)` 并沿 Slot 装饰/隐藏该元素，保留原标签和 listeners；不将 block/table 结构塞进固定 span。无目标的 Skeleton 是独立占位 span，调用方可显式选择 block 展示；带普通 children 的便捷组装使用合法 div 包装。这些入口共用同一 loading 投影，不在状态切换时重建目标。

**通知部件与数据。** 唯一行为入口在 core `toast`：Provider/Viewport/Root/Title/Description/Action/Close；`primitives::toast` 重导出，Themes 同名部件与 Toast/Toaster 加外观。关键公开类型：

```rust
pub enum ToastImportance { Background, Foreground } // polite / assertive
pub enum ToastQueuePolicy { Replace, Fifo }
pub enum SwipeDirection { Up, Down, Left, Right }
pub struct ToastSwipe { pub direction: SwipeDirection, pub delta_x: f64, pub delta_y: f64 }
// ToastOptions: duration_ms:u32 (=3200，0 为 persistent), tone:ToastTone,
//               importance:ToastImportance (=Background),
//               title:Option<String>, action:Option<ToastAction> （均默认 None）
pub struct ToastConfig {
    pub policy: ToastQueuePolicy, pub visible_limit: usize,
    pub swipe_direction: SwipeDirection, pub swipe_threshold: f64,
    pub hotkey: Keystroke, pub viewport_label: String,
}
// Default: Replace / 1 / Right / 50 / F8；空 viewport_label 使用 locale 的通知区文案。
pub struct ToastAction {
    pub label: String, pub alt_text: String,
    pub on_action: Callback<Cancelable<()>>,
}
// provide_toasts_with(config: ToastConfig) -> ToastHandle；provide_toasts 使用默认值。
// ToastHandle 新增 items() -> Vec<Toast>；current() 取当前首项；show/dismiss(seq) 仍适用。
// Root(open:Signal<bool>, on_open_change:Callback<bool>, duration_ms, importance,
//      on_escape_key_down, on_pause, on_resume,
//      on_swipe_start/move/cancel/end: Option<Callback<Cancelable<ToastSwipe>>>, children)
// Action(alt_text:String, on_action:Callback<Cancelable<()>>, children)；Close 为 native button。
```

- Provider 只拥有注册顺序、可见槽位、剩余时间、暂停原因与手势，不拥有 open。Root 注册使用内部 ID；ToastHandle 持有应用通过 show 接受的消息列表，ToastRegion 将其映射到同一 Root，render 回调在该 Root 的 Owner 下构造 body，避免缺失部件 context、嵌套两个 Root或重复计时。便捷 ToastAction 数据若由 show 使用，采用 `label/alt_text/on_action: Callback<Cancelable<()>>`，不把 Fn/DOM 节点序列化到业务对象。
- FIFO 按被接受 open 的登记顺序展示，排队不提前耗时；Replace 在便捷 handle 中原子替换列表。声明式 Replace 对旧 Root 仅发一次关闭请求，未被接受则新项等待，不强行抹掉外部 open。visible_limit=0、非有限/负 swipe_threshold、超出可调度范围的时间等配置在边界诊断，拒绝非法字段并保留有效配置；不在内部反复校验已成立不变量。
- Provider 使用一个可取消的最早到期 timer，基于单调 performance 时间保存剩余时长。Viewport（含通知浮层子树）的 hover/focus 暂停 Provider 内全部可见 session，window blur/document hidden 同样全局暂停；活动 swipe 暂停所属 Root。只有全部暂停原因解除才继续，恢复不重置为完整 duration。关闭请求被拒绝后不每帧重发，到下一次明确重新打开/时长改变才重设 session。timer、监听、pointer capture 在 Root 注销/Provider 清理/abort 释放。
- Viewport 有本地化名称与可配置 hotkey，F8 仅在当前 scope 有通知时把焦点送到首个可操作通知；不劫持普通输入字符。关闭当前通知后到下一可用通知、有效原焦点或 scope fallback；Tab 自然退出通知区，Escape 只请求关闭当前聚焦通知。
- 与 modal 的组合在 core 完成：stack 拥有本 scope 的通知宿主及有效父层引用；Viewport 的 Owner 向通知内容提供同一个动态引用，DOM 搬动不改变 Owner 继承。当前有效 top modal 为 M 时，Viewport 的同一个 DOM 根挂入 M 的 Content notifications host，引用指向 M；无 modal 时放本 scope overlay，引用为 None。通知内新浮层在实际注册时读取该引用；若其外侧已有实际 Layer，则优先使用最近 Layer 的 context。有效主题/方向仍来自创建 provider。stack 在 native 编辑之后、父 modal 之前处理聚焦通知的 Escape；通知上方已打开的后代浮层先处理。Toast 不是新的 outside-dismiss/modal 层，不在 scope 外加第二套键盘监听。
- **已打开后代采用重绑并保留，不因换宿主请求关闭。** 直接从通知动态引用注册的浮层是迁移子树根；换宿主时把这些存活根的 parent 和 Portal 挂载点一起改到新父层的 children host，无 modal 时改到本 scope overlay，子树内部父子关系保持。保留 LayerId、Owner、Root DOM、业务 open、通知 session/剩余时间与原登记次序，后代优先级按 §5.1 的父子关系确定，不重注册、不重跑 auto_focus、不重复宣告；完成父关系与 DOM 更新后再统一刷新 inert、Tab 域和 outside/Escape 仲裁。相比迁移时关闭，这一选择保留正在进行的通知内任务，无需增加公共 props。
- **通知打开 modal 的例外。** 先按注册前的有效父层登记新 modal，再以更新后的活动栈选 top modal M；不根据搬动后的 DOM 反推父层。若 M 位于某个通知迁移子树内，该子树从根到 M 的分支保持原父关系和挂载点，不能把 M 或其祖先回绑到 M；其余独立通知子树才迁入 M。M 的 Portal 不属于被搬动的 Viewport DOM，移动 Viewport 进入 M 不形成 DOM 自包含。新通知浮层随后以 M 为有效父层，显式嵌在已有 Layer 下的部件仍遵守最近 Layer context；不因迁移改写栈登记顺序或重新打开 M。
- **离开宿主与焦点。** modal 关闭/失效时，先排除本次将撤下的层，选下一有效 top modal 或本 scope overlay；在普通后代清理前迁走仍存活的独立通知子树并更新引用。实际关闭、卸载或自身 anchor/通知子树内部固定父分支失效的层仍按 §5.1 清理，不借迁移保活；scope 释放/instance abort 直接释放，不转移到宿主页。迁移时焦点原在保留子树内则保留该有效节点，浏览器搬动丢焦时只恢复同一节点；节点已失效才回到当前有效父层可用焦点或 scope。外部有效焦点不抢回，宿主关闭的默认回焦不得覆盖已迁出的有效焦点；之后真正关闭后代再按 §5.1 回到有效 Trigger/父层，不引用已撤下 modal。迁移完成重新判定 hover/focus 暂停原因，继续使用原剩余时长。
- 分开常驻 polite/assertive announcement 节点，进入可见槽位时仅宣告一次该条 title/description；不把整个可交互 Viewport 设为不断重复朗读的 live region。Action 的 alt_text 提供等价操作说明，按钮有真实名称；tone 只影响视觉，importance 才决定宣告。关键任务仍有面板里的持久入口，示例不把唯一撤销/恢复能力放在消失提示中。
- swipe 只在配置方向超过 threshold 后请求关闭；反向/取消还原视觉，不请求关闭，取消 hook 阻止默认。坐标通过 CSSOM/geometry 写偏移；按真实 pointer 手势，不拦截 Action/Close 的普通点击。关闭先完成资源/焦点处理，不等待装饰动画；减少动效保留同一操作结果。

### 5.7 Themes 参数、度量与样式

下表从本次读过的本地 Themes `*.props.tsx` 与 base-tab-list props 确定；它是本期必交矩阵，不能以少声明字段缩小需求。各组件定义具名 Size/Variant 枚举与 ALL；不造通用全组件 variant。size、标为响应式的布局字段使用现有 Responsive；未显式给 color 时继续消费该组件有效语义角色，局部 color 只改其声明角色。

| 组件/部件 | size / 默认 | variant / 默认 | 其他本期公开视觉参数 |
| --- | --- | --- | --- |
| Tabs List、TabNav Root | 1/2；2 | 无 | color、high_contrast；Responsive wrap(nowrap/wrap/wrap-reverse)、justify(start/center/end) |
| SegmentedControl Root | 1/2/3；2 | surface/classic；surface | radius；不凭空补 color/high_contrast |
| Dialog/AlertDialog Content | 1/2/3/4；3 | 无 | align Start/Center（默认 Center）；Layout 的 width/min/max/height，max_width 默认 600px |
| Popover Content | 1/2/3/4；2 | 无 | Layout 宽高/min/max；max_width 默认 480px；Arrow 取面板外观 |
| HoverCard Content | 1/2/3；2 | 无 | Layout 宽高/min/max；max_width 默认 480px |
| Tooltip Content | 无 | 无 | Responsive width/min_width/max_width；max_width 默认 360px |
| Progress | 1/2/3；2 | classic/surface/soft；surface | color、high_contrast、radius、duration_ms |
| Spinner | 1/2/3；2 | 无 | loading；名称与 children 见 §5.6 |
| Skeleton | 无 | 无 | loading；Layout 中宽高/min/max |
| Avatar | 1—9；3 | solid/soft；soft | color、high_contrast、radius、fallback |
| Badge | 1/2/3；1 | solid/soft/surface/outline；soft | color、high_contrast、radius |
| Callout Root | 1/2/3；2 | soft/surface/outline；soft | color、high_contrast；无参考 radius 参数 |
| Card | 1—5；1 | surface/classic/ghost；surface | native target、Layout/children；不附加无依据色板参数 |
| DataList | 1/2/3；2 | 无 | Responsive orientation Horizontal/Vertical（Horizontal）、trim；Item align；Label 宽度/min/max、color/high_contrast |
| Table Root/Row/Cell | 1/2/3；2 | surface/ghost；ghost | Root Responsive layout Auto/Fixed；Row align；Cell justify、宽度/min/max、Padding；原生表格属性 |
| Separator | 1/2/3/4；1 | 无 | Responsive orientation、color 默认 Gray、decorative=true |
| ScrollArea | 1/2/3；1 | 无 | radius、scrollbars；visibility/hide_delay 属行为配置 |
| Accordion/Collapsible、Toast | 不虚构 Themes 同名参考 | 不虚构参考变体 | Rustify 的既有语义面板/通知外观，class 可覆盖；ToastTone 保留 |
| CheckboxCards/RadioCards | 沿 P2 | 沿 P2 | 原有 color/high_contrast/columns/gap，复用并回归 |

`Layout`/Padding/Spacing、TextTrim、Align 等已有类型优先复用；各组件仅接受其表内需要的部分，不把整套布局变为每个 primitive 的必需参数。Themes 的参考通用 margin 能力统一用既有 `margin: Signal<Margin>` 表达 m/mx/my/mt/mr/mb/ml，保留负 step 与 CSS 值/响应式规则，不将 margin 伪装成不存在的 Layout 字段。原生属性和现有 utility class 继续可用。

度量、字号、边界、阴影与状态按正式入仓的对应 CSS 转写到低权重 `@layer components`；使用已解析 `--rustify-space-N`、字体/半径、面板 surface、shadow，不再乘 scaling。扩展 sdk.css/styles 常量导入三份皮肤，raw 仅导入结构 CSS。高对比只改变该组件声明的文字/指示或配对背景；不能染掉相邻字段、默认错误或创建处 theme。主题两模式、部件边界、caller utility 与取消覆盖须有可观察检查。

### 5.8 构建期静态 HTML 与接管

- 按独立 feature 构建 CSR、host ssr、hydrate。host 不读 DOM/图片状态/计时器，使用相同 scope seed、业务初值及 entries；无请求时渲染/异步流式验收。
- Tabs/Accordion/Collapsible 的已接受内联内容、表格/列表、图片回退、Progress 值和必要几何生成静态 HTML；关闭面板保留语义关联且不能键盘进入。Title/Description 的 ID 与实际部件一致，包含延迟 children/Show/For，不依赖浏览器 effect 修复初始 HTML。
- Dialog/Popover/HoverCard/Tooltip 初始关闭，浮层 Portal 输出确定占位，attachment 后才挂浏览器内容；host 请求初始打开时记录 StaticRenderIssues，force_mount 不绕过此边界。运行 ToastRegion 初始为空；固定状态展示用内联 Callout，非把有定时器的通知强塞进 SSG。
- hydration 复用已有节点、唯一 ID 和关联；Avatar 先接管 fallback 再处理图片结果，Skeleton/Spinner 切换保留 children；几何由 GeometryStyles 写外部 stylesheet，动态通过 CSSOM。多 scope/独立实例不能共享 allocator 或清理资源。

## 6. 前端与交互

本期页面遵循已有 DESIGN.md 与 Studio surface brief 的 Operate 工作台方向：紧凑、任务优先、清楚焦点、语义面板；不重做品牌或编辑器布局。

- **主样本**：在 styled DOM 消费者中提供一个完整本地面板：Tabs 在摘要/明细间切换；Accordion/Collapsible 查看补充说明；Popover 编辑辅助选项并包含 P2 Select；Dialog 编辑本地条目；AlertDialog 确认删除该演示条目；进度/加载和通知反馈结果。表格与 DataList 呈现对应明细，Avatar 同时展示真实本地图与失败回退；关键取消/恢复动作留在面板。全部动作改变本地演示状态，不声称有后台业务。
- **层级**：一个面板标题、可辨认的主操作、较轻的补充操作、就近的状态/错误。默认首屏不铺开所有变体；完整 parts/参数/拒绝请求/动态移除实验收进独立 details 状态工具，供 SDK 开发者使用。raw 消费者只演示新增 primitives 的行为/结构，静态 Themes 专属内容不另造无样式副本。
- **布局**：390px 为单列；768px 与桌面按可用空间组织内容，不强制沿用 Studio 两列。长 label/URL/单元格可换行，二维表格局部滚动。Dialog/Popover 限制到视口可用宽高并内部滚动；确认/取消和关闭入口不会被内容挤到不可达区域。RTL 用逻辑边缘，原文和业务值不变。
- **状态**：空列表/无活动 panel 是可读结果；loading 保留名称/尺寸；图片失败显示 fallback；配置错误/非法组合给开发者定位信息；应用拒绝 open/value 时仍能看见接受状态并继续操作。使用者文案不暴露资源计数、内部层 ID 或测试指令。
- **视觉**：浅/深默认主题按同一字级、间距、边界/阴影与焦点层级；状态文字/图标与颜色一起表达结果。样式取有效 snapshot，组件局部 color/highContrast/radius 不扩散到邻居。默认文字/非文本边界按既有可访问门槛实测；作者低对比主题不被静默改写。
- **动效与可访问性**：只为开闭、进度/加载和 swipe 提供必要状态反馈，尊重 prefers-reduced-motion。名称/说明/错误关联到实际 native target；Toast 不重复宣告，Tooltip 不包交互菜单；全程键盘可达。对话确认遵循安全默认焦点，模态只约束所属 scope。
- **调用方**：Studio 的预设、精调、导入/导出、本地库与 GPU/DOM 预览保持已有任务；本期在其已有导入对话/预览边界做定向回归。Catalog 的现有分类与独立能力表明确区分；新增组件文档链接到真实消费示例，不以计划项标“已支持”。

真实浏览器检查见 V-6、V-8、V-10、V-11；在最终产物中检查 console、pageerror、CSP、失败请求与必要截图。图片失败场景只允许预先声明的本地失败 URL，不统一忽略资源失败。没有本期真实截图/操作记录，不能用 lint/build 替代 UI 通过。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD §3.2、APG、现有默认焦点 | 本期键盘/角色/名称与关联正确；默认普通文本 ≥4.5:1、必要非文本边界 ≥3:1；真人 AT 结论仍待实际验证 | 不把无名称操作/焦点丢失记为通过；保留人工 missing | 原生语义、scope 焦点域、安全取消、独立 live 宣告 | V-6、V-8、V-10、V-11 |
| WORK-N2 | P2 forms-performance 的页内测时/压缩尺寸口径；PRD Q4 | 同功能前后 p50/p95、WASM/CSS gzip 与资源差值；新能力单列。具体耗时/差值未知，无新增毫秒 SLO | 持续资源增长/请求重复先修；差值不能由功能增量解释时不得退出 | 几何帧内合并、只观察开放层、一个通知到期器、按存活项缓存索引/引用 | V-1、V-13 |
| WORK-N3 | architecture.md；两 host 的 abort/strict CSP | 无新增 CSP 违规、宿主 reset/主题改写；清理后监听/observer/capture/焦点限制消失 | 失效对象立即撤下；fatal 按实例终止，不回调死 wasm | listen/ResizeObservation、SearchTimer 式 clearTimeout + abort、scope CSSOM | V-6、V-11、V-12 |
| WORK-N4 | features/build_hydration 与 source 配对 | DOM 默认无 Makepad；SSG 输出可读、hydrate 原节点/ID；资产 URL 外置 | 不支持的初始浮层打开明确拒绝；不放宽 CSP | 独立 feature/target、确定初态、外部 CSS 与静态资源 | V-9、V-12、V-14 |
| WORK-N5 | catalog.rs 与迁移指南 | 本期全部字段/变体/入口有对应真实结果，限制可查 | 缺实现不能标 Yes | 唯一能力表、生成文档、source revision 配对 | V-8、V-10、V-14 |
| WORK-N6 | PRD §2.7；LayerResources | 取消/移除/清理不遗留不可操作页面，不擅自确认/丢业务状态 | 应用拒绝关闭时维持可用层；anchor 消失则清理失效层 | 请求和接受分离、一次失效请求、安全 fallback | V-6、V-11 |
| WORK-N7 | DESIGN.md；PRD §2.5 | 两模式、声明响应式/缩放/长内容可操作，编辑壳稳定；不增加壳内永久诊断噪音 | 保留可达操作与局部滚动，不裁掉关键内容 | 有限高浮层、语义层级、稳定占位、减少动效 | V-6、V-8、V-10、V-11 |
| WORK-N8 | sources.lock.json/vendor | 正式采用代码/样式可取回且带许可；正常构建无 ref 依赖 | 缺来源/许可不进入交付 | M1 正式输入、归档 include、sources verify | V-1、V-14 |

这是静态浏览器 SDK，没有新增 SQL、网络供应商、服务端鉴权、租户表或审计服务，不套用后端 N+1/缓存/限流方案。热路径不在 pointermove/scroll 中重新扫描整页或重复测量每个 sibling；每帧复用已注册节点/索引，内容/项变化时才更新。对外字符串按 Leptos 文本/原生属性表达，不提供 innerHTML 或执行导入 CSS 的入口。

可观测证据沿现有实例 stats/diagnostics、页面事件/DOM/ARIA、GeometryStyles diagnostics、console/request/CSP 与 benchmark JSON；不新增遥测后台或把用户正文写入 diagnostics。

## 8. 失败模式、切换与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 应用拒绝 value/open/关闭请求，或取消 hook | 对应控件/层 | 无业务值被擅自写入；危险确认不重复 | 继续显示接受值，内容仍可操作；生命周期失效例外按下行撤下 | 请求计数、实际值/ARIA/焦点 | 应用明确接受新值或继续原任务 | V-2、V-3、V-6、V-10 |
| Trigger/anchor/当前项删除或禁用 | 对应层/组及其后代 | 不修改应用存储值；失去呈现对象 | 无死遮罩/捕获；焦点落有效 Trigger/父层/组根/scope | DOM、stack depth、inert、listeners/observers | 清理失效注册；新有效节点出现后按接受状态恢复 | V-6、V-8、V-11 |
| 图片失败、旧 src 回包晚到 | 单个 Avatar | 无作者/业务数据改变 | 显示有名称的 fallback；旧成功不覆盖新 URL 状态 | load/error、代次、可读名称 | 更换 URL，重试当前请求；只重置此头像临时状态 | V-5、V-8、V-9 |
| 浮层过大/碰撞边缘，内容/视口改变 | 对应开放层 | 无 | flip/shift 与局部滚动，箭头真实反映；操作仍可达 | rect/data-side、命中、键盘到达 | 下一实际几何事件合并重算；非法配置保留有效值 | V-4、V-6 |
| Toast 暂停、过期/关闭被拒绝、滑动取消 | 对应 Provider/Root | 消息列表由应用决定，无强行丢弃 | 剩余时间正确；无重复关闭/宣告；等待项不提前过期 | 单调时间、on_open_change/announce/pause 计数 | 继续或接受关闭；操作还有面板持久入口 | V-7、V-10、V-11 |
| Toast 所在 modal 更换、关闭或失效 | 本 scope 通知及其浮层子树 | open/剩余时间不因迁移被重置；实际失效层正常释放 | 存活通知/后代按 §5.6 换宿主，无自环、无主层或失效回焦 | Layer 父关系、DOM 身份、焦点/仲裁与计时 | 先迁移存活子树，再清理失效层；scope/instance 结束不迁出 | V-3、V-7、V-11 |
| scope 正常释放，或实例 wasm trap | scope / 所属实例全部 scope | 临时 UI 状态结束；既有持久数据不变，未持久输入可能丢失 | 正常释放不影响其他 scope；fatal 只保证其他独立实例继续 | 现有 abort/stats、host fatal、页面交互 | 正常释放全部资源；fatal 沿既有 restart，恢复最近持久草稿 | V-11、V-12 |
| 新源码与旧 CSS/消费者混用 | 使用该产物的页面 | 无 schema 写入 | 参数视觉无效或编译失败，不能称兼容成功 | mode/build/目录与 source revision | 源码、CSS、示例/fixture 一起切换或一起回退 | V-9、V-12、V-14 |

提交点先建立可取回基线，再完成阶段一面板，最后完整 P3。源码/生成 CSS/调用方同批更新；检查未通过不得宣称阶段出口。没有部署/放量动作或不可逆数据迁移。若回归阻断现有入口，回退相应阶段提交及其 CSS/示例；不单独留下新 CSS 或新事件语义，也不恢复已经被替代的平行实现。

## 9. 验证

### 9.1 检查定义与时点

所有命令在仓根执行。`--test panels_ssg/content_ssg/feedback_ssg` 及新增 specs 是本期交付文件；先创建对应测试再运行。批内只对具体失败或必要前置运行一个测试过滤器，例如 `mbx test --locked -p rustify-components --lib primitives::tabs::tests::manual_activation`；没有具体疑问就连续实现。测试名称按实际行为命名，源码中不带需求/里程碑 ID。下面整组命令仅在交付齐备后执行，不逐次编辑重复跑。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | C-1、WORK-N2/WORK-N8；P2 具体前置可用、旧面板行为可重现，before 数据与正式参考可取 | §9.2 的 baseline 命令；`mbx test --locked -p rustify-dom --lib overlay::tests`；`mbx test --locked -p rustify-dom --lib toast::tests`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test ssr_controls`；`mbx xtask sources verify` | M1：同功能测量 fixture、来源/许可与 before 实验齐备后；baseline 必须在改旧行为前，作为重构风险门 |
| V-2 | WORK-F10/AC-1、AC-2；本期 WORK-F15/AC-3；单/多展开、手动/自动/方向/拒绝、动态焦点/隐藏状态 | `mbx test --locked -p rustify-components --lib primitives::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg`；`mbx clippy --locked -p rustify-components --all-targets -- -D warnings`；`cargo fmt -p rustify-components -- --check` | M2：切换/展开原语、Themes、同语义样本与模型/SSG 测试整批齐备后；真实键盘留 V-6 |
| V-3 | WORK-F11/AC-1—AC-3；Dialog/AlertDialog 取消、命名、安全 Action、局部模态 ARIA 投影（含旧 Layer/便捷入口）；动态通知父引用、迁移/关闭和防自环的纯层级模型 | `mbx test --locked -p rustify-dom --lib overlay::`；`mbx test --locked -p rustify-components --lib primitives::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg --test missing_context_ssr`；`mbx clippy --locked -p rustify-dom -p rustify-components --all-targets -- -D warnings` | M3：core 父子/焦点域/通知宿主重绑契约与 Dialog/AlertDialog、便捷调用迁移、对应 tests 齐备后；解除 M4/M6 的 core 前置，不代替 V-6 |
| V-4 | WORK-F12/AC-1、AC-2；四边/RTL/collision/arrow、可撤计时、grace、Tooltip 非交互/不移焦 | `mbx test --locked -p rustify-dom --lib overlay::`；`mbx test --locked -p rustify-components --lib primitives::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg`；定向两包 Clippy 同 V-3，另以 `--no-default-features --features ssr` 检查 | M4：辅助浮层的行为、皮肤、typed 旧调用迁移与定位/计时测试齐备后，随后进入本阶段 V-6 |
| V-5 | WORK-F17/AC-1、AC-2；内容原生语义/图片代次、比例/分隔与滚动模型，WORK-F15/AC-3 参数静态输出 | `mbx test --locked -p rustify-components --lib primitives::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test content_ssg`；组件 CSR/ssr 各自 Clippy；`cargo fmt -p rustify-components -- --check` | M5：全部内容、滚动部件/便捷入口、raw 适用样本与 styled 真实内容整批齐备后；不逐个组件跑包级门 |
| V-6 | UAT-5/6 的 P3 范围：WORK-F10 全部、WORK-F11 全部、WORK-F12 全部及本阶段视觉；含 P2 Select 嵌套与旧 Layer；实查 Content 角色/名称、无 aria-modal、本 scope inert/Tab 约束，另一 scope/宿主的真实点击与键盘可达 | §9.4 两个 build-dom；串行 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives panels.spec.ts` 与 `--project=dom-only panels.spec.ts`；实际截图与同一次 UI 走查；旧 GPU anchor `--project=fusion-basic m4-overlay.spec.ts`（先 build-web） | M4：V-2—V-4、完整面板和最新两消费者产物齐备后的**阶段一收口**；不等内容/通知全完成，验证模态/输入风险；后续仅改相关路径才补受影响项 |
| V-7 | WORK-F14 全部；本期 WORK-F15/AC-3：进度原单位/未知状态、Spinner/Skeleton、通知 FIFO/替换/暂停/拒绝/取消/计时及换宿主后剩余时长保持 | `mbx test --locked -p rustify-dom --lib toast::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test feedback_ssg`；两包 CSR/ssr Clippy；`cargo fmt -p rustify-dom -p rustify-components -- --check` | M6：core 唯一通知调度、原语/Themes、反馈样本与确定性模型/静态测试齐备后 |
| V-8 | WORK-F17/AC-1、AC-2 与本期 WORK-F15/AC-3；真实图片/滚动/长内容/table/dl/卡片、全部视觉声明 | 最新 build-dom 后，串行 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only content.spec.ts`、`--project=dom-primitives content.spec.ts`；§9.3 参数矩阵与截图 | M7：V-5 与内容/皮肤/组合样本全部齐备后的**阶段二收口**；raw 只验其实际 primitive 集合，不能把跳过的 themed rows 算通过 |
| V-9 | 本期 WORK-N4：同步 SSG 内容、唯一 ID/关联、原节点接管/关闭 Portal，覆盖本期交付组件 | `mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg --test content_ssg --test feedback_ssg --test ssr_controls --test missing_context_ssr`；V-12 构建 fixture 后 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` | M7：扩展 deterministic view、独立 features 产物与所有静态测试齐备后的阶段二收口；不测试流式 SSR 服务 |
| V-10 | WORK-F14/AC-1、AC-2、WORK-F15/AC-3；真实计时/hover/focus/window pause、swipe/F8/action、宣告结构及动效 | 串行 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only feedback.spec.ts`、`--project=dom-primitives feedback.spec.ts`；§9.3 的反馈视觉矩阵 | M7：V-7 与新通知/反馈真实产物齐备后的阶段二收口 |
| V-11 | 全部 12 条 AC 的完整任务组合；本期隔离/清理/响应式质量；上游直接调用无回归 | §9.3 的角色模拟、通知先建/后开浮层及通知打开 modal 的迁移组合、scope dispose/独立实例 fatal；§9.4 的 Studio/Catalog/Docs 定向命令。新增端到端放 panels.spec/feedback.spec 的 integration describe，用 `-g integration` 只验新增组合；检查实际 console/请求/CSP | M7：M2—M6 全部交付后最终集成；复用 V-6 未变内容，只补新组合及受影响旧调用，不例行重跑阶段一全部走查 |
| V-12 | C-1/C-4、本期 WORK-N4：无 Makepad/无皮肤产物、feature 分离、Docs 直接消费者可编译 | `mbx xtask docs-verify --group features`；根/子路径与受影响真实入口按 §9.4；源码图/产物由现有命令检查 | M7：最终 API/CSS/fixture 齐备，先运行以生成 V-9 所需 hydrate/CSR 产物；这是最终环境/构建门，仅安排一次 |
| V-13 | WORK-N2 本期增量：before/after 同功能与 new-only 的页内 p50/p95、压缩体积/资源计数；清理后恢复、无空闲积压 | §9.2 的 current 命令与 baseline.json，40 个样本/5 次预热沿用现有测量口径，结果写相同验证目录 | M7：最终 release 产物与行为回归通过后一次完整 evidence；批内只用缩小 workload 定位具体问题，不反复全量取证 |
| V-14 | 本期文档/能力/来源与交付相符；workspace/Docs/SDK 模式无公共 API 回归 | §9.5 的范围静态/host 回归命令、CSS/catalog/sources；Agent 仅凭公开指南消费实际源码 API 的角色模拟 | M7：文档、目录、迁移、生成物、CI 接线与全部能力齐备后的阶段二收口 |

上述检查默认由实施 Agent 执行。M2/M3/M5 的局部模型与 SSG 检查不能证明浏览器行为；M4、M7 收口才运行完整真实场景。已通过的 model/SSG/Clippy 在最终相关代码未变时直接引用，V-9/V-14 中重叠包级命令合并为一次；不能因表中被多项引用而重复执行。

### 9.2 行为基线、性能与成本

M1 在旧组件上加入 panel_benchmark.rs、外部 panels.html/js 及 panels-performance.spec.ts。测量复用现有 `tests/dom-only/forms-performance.spec.ts` 的浏览器事件 capture → 接受状态与 DOM 到达下一帧口径，使用 1440×900、5 次预热、40 次样本及 p50/p95；测量数字来自该既有口径，不是新 SLO。匹配 workload 为 Dialog 开/关、Tabs 切换、Tooltip focus/Escape、单条 Toast 替换/关闭、Progress 更新、ScrollArea 滚动；benchmark 暴露策略/快照/资源读数，不直接调用组件 handler。

```sh
# M1：旧行为基线；先建好该测试及页面，再执行。
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml
PANEL_BENCH_PHASE=baseline RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only panels-performance.spec.ts

# M7：最终同功能对照与新增能力分列。
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml
PANEL_BENCH_PHASE=current RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only panels-performance.spec.ts
```

新增环境变量 `PANEL_BENCH_PHASE` 只接受 baseline/current，输出 ★ `docs/validation/dom-components-p3/baseline.json`、★ `current.json`；代码版本、browser 版本、OS/viewport/DPR/可见性、工具链、样本与输入负载写进 JSON。M1 提交保证测量代码与旧实现可取回；最终环境不同时重新从该提交构建对照并标明原因，不拿 P2 数值冒充 P3 before。计时在页内，Playwright 仅驱动原生事件。

旧新同功能负载分列延迟和请求计数；新多层定位/通知队列/自绘滚动条增量单列，不比较不存在的旧功能。记录 wasm、全部 CSS/runtime 与新增图片资源的原始/gzip 大小；旧图与新图不得混算同功能大小。循环打开/关闭、增删项、滚动拖动后，比较现有 stats、监听、observer、pointer capture、geometry rules/DOM 数；等待稳定后无持续增长/多余调度，Optimized 空闲时无定位帧，Always 的显式运行与关闭后停用分别核验。

本期无可引用的新 p95 上限；退出需要全部行为通过、无未解释的持续积压/资源残留，并对每个可测回退说明功能与算法成本。测量和新增 fixture 的耗时目前未知。`docs-verify --group features` 包含多个构建与外部消费者，属于收口成本；GPU fixture 新 context 编译和完整项目可能很慢，按 AGENTS.md 的现有记录选择定向 spec，不跑不相关的发行总套件。

### 9.3 场景、视觉矩阵与角色模拟

- **阶段一面板**：键盘/指针完成自动/手动 Tabs、单/多 Accordion、Collapsible、SegmentedControl 和真实 TabNav 链接；分别接受/拒绝/延迟请求。Dialog → Popover → Select 的三层组合只关闭顶层；AlertDialog 初始聚焦安全 Cancel，取消零危险回调，Action 仅一次；outside/Escape/auto_focus 各可取消；移除 Trigger/当前项后焦点确定，hidden/force_mount 内容不误入。
- **浮层边界**：四边、三种 align、LTR/RTL、offset/箭头/碰撞边界、很长内容、嵌套滚动/resize；hover 从触发器移到卡片/Tooltip 可停留阅读，选区不误关，focus/blur/Escape 不劫持；触控路径用真实浏览器 Pointer Events 验证可操作行为，OS 真机结论单列。
- **内容和反馈**：普通/空/长 table/dl、header/label 关联、Avatar 正常/失败/换 src 竞态、ratio/Separator、两轴滚动与拖动取消、卡片 FormData/描述；Progress 非 100 max 的原单位与未知状态；Spinner/Skeleton 在 loading 结束后保留原节点/草稿并恢复键盘可达，不抢走外部已有焦点；加载时隐藏内容不可操作。
- **通知组合**：FIFO/Replace、限量/持久条目、等待不耗时、hover+focus 多原因暂停、blur/hidden、拒绝过期关闭、滑动成功/取消、Action/Close/热键进入与返回；分别验证“先创建 Toast → 打开 modal → 从原 Toast 打开 Popover → Select → Tab/outside/Escape → 关闭 modal”和“Toast 内已有 Popover/Select → 打开另一个 modal → 关闭宿主/返回”；再从 Toast 操作打开新 modal，并覆盖新 modal 嵌在已有通知浮层内的路径。按 §5.6 核对新/既存后代的父关系与实际挂载、无自环/无主层、Tab 域和关闭顺序、有效回焦与 native Draft/IME composing 优先；迁移前后 Root/Content DOM 身份、业务 open、auto_focus/关闭/宣告计数及剩余时长保持。重复操作计数与两个 live region 的结构证明不重复派发；不以 DOM 断言冒充 VoiceOver 实际朗读。
- **视觉采集**：每个 §5.7 枚举的 ALL 都有至少一个真实可消费状态，并在正常/焦点/选中或打开/禁用/加载及适用错误状态检查，不做没有意义的全参数笛卡尔积。浅/深两模式、默认/局部色板、高对比和 caller utility 覆盖均有结果；390/768/桌面、200% 放大与 reduced motion 按受影响任务采集。浏览器 UI 缩放若只能用 CSS/视口模拟须分别标注，真实浏览器 UI 缩放缺失仍记 missing。
- **隔离/清理**：两个 scope、嵌套反向主题与普通宿主元素共存；局部 Dialog/AlertDialog/modal Popover 与旧 modal Layer 分别核对角色/名称、无 aria-modal、本 scope 底层不可操作，以及另一 scope/宿主可点击且可键盘聚焦。正常 dispose/Trigger 失效只释放所属资源；新实例 fatal 结束其全部 scope，独立实例继续。沿已有故障 fixture 触发，不新增 GPU 故障隔离机制；Studio 已有持久化/GPU 故障证据未变部分引用原记录。
- **角色模拟**：Agent 模拟应用开发者，只持有 README、docs/panels.md、迁移指南和公开示例/API，使用当前 source SDK/CSS；不借私有模块或实现者口头步骤，完成一个含“安全确认→辅助选择→内容反馈”的纯 DOM 面板并在真实浏览器操作。记录代码位置、命令与缺失步骤；失败先补公开文档或实际接口，再以相同角色继续。此结果不代表真人上手率/满意度。

每次正式收口使用最新 release，记录操作与必要截图，检查 console/pageerror/requestfailed/CSP；预期本地坏图单独声明。自动扫描、accessible name/ARIA 与截图自检不代替人工辅助技术结论。OS IME、VoiceOver、真机触控、未能执行的浏览器 UI 缩放继续列 P5 missing；无需招募真人才能退出 P3。

### 9.4 构建与真实入口

DOM 基础项目不启动 GPU；既有 config 为 dom-only/dom-primitives 使用 4190、dom-hydrate 使用 4194，按需启动所选 server，均须空闲。构建命令：

```sh
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml
mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml
```

在 playwright.docs.config.ts 将 panels/content/feedback specs 同时登记到 dom-only 与 dom-primitives；dom-only ignore 列表排除既有 raw-only 文件，dom-primitives 的 match 增加这三份；panels-performance 仅 dom-only。最终把内容/反馈与直接受影响的既有生命周期、表单及 raw parts 合并执行，不再另跑 V-8/V-10 的相同单 spec：

```sh
npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only content.spec.ts feedback.spec.ts dom-only.spec.ts forms.spec.ts slider.spec.ts
npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives content.spec.ts feedback.spec.ts primitives.spec.ts
npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only panels.spec.ts -g integration
npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives panels.spec.ts -g integration
```

V-6 的阶段一已覆盖不变项直接引用；M7 的 integration describe 是新增模态/通知/内容完整任务，不重复旧路径。若 core/契约的后续改动实际使阶段一证据失效，在下一收口补齐其受影响测试，记录失效范围，不借此删减终验覆盖。

受影响的 GPU host/示例只做兼容验证，先构建当前源码，不复用旧 release：

```sh
# 阶段一：现有 Layer 的 rectangle anchor 与嵌套输入风险。
mbx xtask build-web --example fusion-basic --release
npx --no-install playwright test --project=fusion-basic m4-overlay.spec.ts

# 最终调用方：每个 Playwright 命令必须串行。
mbx xtask build-web --example component-catalog --release
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example theme-studio --release --base /tools/demo/
npx --no-install playwright test --project=component-catalog p2-semantics.spec.ts theme-compat.spec.ts
npx --no-install playwright test --project=theme-studio theme-panels.spec.ts theme-forms.spec.ts theme-exchange.spec.ts theme-ui-review.spec.ts
npx --no-install playwright test --project=theme-studio-deep theme-deployment.spec.ts
```

新增 theme-panels spec 只检查本期导致的 Studio 既有导入/取消/导出、局部主题与新组合，不提前新增完整 Studio gallery。Docs 真实演示沿 V-12 的 component-site 构建，再串行 `npx --no-install playwright test -c playwright.docs.config.ts --project=components-subpath islands.spec.ts`，检查实际样本/样式/打开关闭；CLI 内容/搜索和无关应用不扩入本期。若确有公共改动影响其他已有消费者，由 V-14 host/build 发现并定向补相关入口。

`docs-verify --group features` 会生成 hydration 的独立三模式产物，不再人为安装 wasm-bindgen CLI/建临时装配包。阶段一尚不需三模式完整构建，host SSG 定向测试承担局部边界。V-12 在 V-9 browser 前运行，serve 不重建。若工具/浏览器缺失，按既有环境先排查并求助，不擅自安装基础设施；结果明确记“未在浏览器中验证”。

### 9.5 最终静态与 host 回归

与 `.github/workflows/verify.yml` 的相关入口一致，在 M7 一次完成；Docs 分别运行，不和 CSR workspace 联合启用。局部已通过且本阶段未改的相同检查引用原证据，避免重复执行。

```sh
cargo fmt --all -- --check
cargo fmt --manifest-path tests/dom-only/Cargo.toml --all -- --check
cargo fmt --manifest-path tests/dom-only/hydration/Cargo.toml --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-dom --no-default-features --features ssr --lib
mbx test --locked -p rustify-components --lib
mbx test --locked -p rustify-components --no-default-features --features ssr --lib
mbx test --locked -p rustify-ui --lib
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib
mbx clippy --locked -p rustify-dom -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings
mbx test --locked -p rustify-docs
mbx test --locked -p rustify-docs-cli
mbx clippy --locked -p rustify-docs --all-targets -- -D warnings
mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
git diff --check
```

UI class/皮肤或目录变更后，先 `mbx xtask css` 与 `mbx xtask catalog --write docs/components.md` 更新产物再执行对应 check。新增 SSG integration tests 由 V-9 执行，不能因 workspace 命令含 `--lib --bins` 就漏掉它们。源码 corpus 的旧静态文本测试不作为本期质量证据，不新增此类实现镜像测试。CI 在现有相关 job 中加入新 SSG tests/spec 接线与必要产物，保留单 project 串行策略。

## 10. 里程碑与验收安排

阶段一为 M1—M4 的可操作面板，M4 统一收口切换/对话/辅助浮层。阶段二为 M5—M7 的结构化内容/反馈与完整组合，M7 统一收口本期全部 12 AC。M1 是改动旧行为前的基线风险门。默认每个实现里程碑一批，批内相关实现、测试和必要调用方连续完成，退出后继续下一可执行批次；不把单文件或一次编辑升成批次。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 可取回的行为基线与正式参考输入 | C-1 的 P2 具体组合/表单/主题成果；当前源码未改的基线 | 先在旧组件建立 ★ panel_benchmark.rs、consumer/public/panels.html/js 与 ★ panels-performance.spec.ts，冻结同功能 before；正式纳入本地 P3 props/CSS/行为源与许可、sources.lock；确认 §5.7 字段映射。沿现有 forms-performance/fixture/export 入口，不修改待比较组件行为。主 agent 负责共享接线与基线提交 | V-1 通过，before JSON 和构建/源码版本可取回；回写「实施进度」 |
| M2 | 内容切换、展开与单选/链接导航 | M1 的 V-1；§5.2 已定接口 | ★ primitives tabs/accordion/collapsible 与对应 Themes，★ segmented_control/tab_nav，替换旧 Tabs 内部实现；PanelEntry/AccordionValue、关联 ID/临时焦点按 §5.2；照 choice/roving/Select typed target 组织。补 ★ panels_ssg 与 consumer/raw panels 样本。共享 lib/样式导入由主 agent 集成 | V-2 通过，包含非法/重复 key 与先红后绿的动态/拒绝回归；回写「实施进度」 |
| M3 | 同源层级与安全的普通/警告对话 | V-1；§5.1 的稳定 core/parts 契约；不依赖 M2 完成 | overlay.rs 的纯定位/父子/交互根/清理、通知动态父引用/宿主迁移与防自环、局部模态 ARIA 投影、两 host 结构样式、hooks；★ primitives/floating/dialog/alert_dialog 与 Themes；旧 Dialog 组装同源 parts，直接调用适配。沿 LayerResources/Select Content 和现有 runtime listener/abort，不造新的 overlay manager。T-2 的 core 成果可先解除 T-3/T-4 前置 | V-3 通过；core 签名/默认值与旧 Layer 语义有测试，静态关闭与名称关联正确；回写「实施进度」 |
| M4 | 可操作辅助浮层与阶段一面板收口 | M3 的 T-2 与 V-3；终验还需 M2/V-2 | ★ Popover/HoverCard、重组 Tooltip、Arrow/延迟/grace，按 §5.4 复用 floating；raw/styled 同一完整面板包含 Select 嵌套、接受/拒绝与移除实验；Portal/几何/主题/窄屏由 V-6 真实验证。更新受到 Tooltip typed API 影响的仓内调用；主 agent 串行浏览器与 shared 接线 | V-4、V-6 通过；只宣称阶段一面板出口，未宣称 P3 完成；回写「实施进度」 |
| M5 | 结构化内容、图片与真实滚动 | V-1；已有 geometry/主题/选择卡片契约；可在阶段一独立开发，终验留 M7 | ★ Avatar/Badge/Callout/Card/DataList/Table/AspectRatio/Separator 与 ★ primitives 适用部件；重组 ScrollArea Root/Viewport/Scrollbar/Thumb/Corner。按 §5.5/§5.7，复用 native tag、geometry、component_color/选择卡片；★ content_ssg、两消费者 content 样本。T-5 与 T-6 可分文件实施；主 agent 统一样式/能力表 | V-5 通过；全部内容交付后一次局部退出，真实滚动/视觉留 V-8；回写「实施进度」 |
| M6 | 可读取的加载反馈与完整通知 | V-1；M3/T-2/T-3 的动态父引用、Content/notifications host 重绑/移除顺序与防自环契约及 V-3，不等待无关 Studio 陈列 | core toast 唯一 Provider/Root 调度，ToastHandle/ToastRegion 迁入同一行为；★ primitives/toast/progress、★ Skeleton，重组 Progress/Spinner/Toast/Toaster；core 本地化必要文案。按 §5.6/§5.7 处理 queue/pause/swipe/F8 与原单位进度；★ feedback_ssg/样本，现有字面量同批适配 | V-7 通过；确定性时间/取消/拒绝与 SSG 语义覆盖，真实通知行为留 V-10；回写「实施进度」 |
| M7 | 本期完整面板、真实消费与最终交付收口 | V-2—V-7 全部通过；M2—M6 全部交付；M4 的 V-6 阶段出口 | 集成内容/反馈/层级完整任务与 deterministic hydrate view；同步 Catalog/源码入口/★ docs/panels.md/迁移/README、styles 常量和 CI。先生成最终产物，再按 §9 合并真实浏览器/静态/性能/角色验证。主 agent 统一收口并修复范围内相关验收缺陷，不自行发起独立评审或发行 | V-8—V-14 通过，V-6 的复用/补证边界明确；12 AC 逐条可追溯，manual missing 单列且非 P3 必交缺口；回写「实施进度」 |

提交点：**M1（旧行为/测量基线可取回）、M4（阶段一收口）、M7（全部里程碑完成后的最终提交）**。每次仅提交本期相关改动，按顶部协议回填所覆盖行的 SHA；不把无关工作树变动纳入。M2/M3 在 M4 前完成时写待提交，M5/M6 在 M7 前完成时写待提交。

### 10.1 内部任务与调度

这些任务用于解除真实前置与按文件分工，不另建状态表/子任务文档。实施者默认可顺序完成；只有独立任务的收益超过委派/集成成本时才按顶部协议委派。**主 agent 独占 lib.rs、primitives/mod.rs、sdk.css/styles.rs、catalog.rs、配置/CI、计划与台账；子任务返回需要的导出/导入，由主 agent 集成。** 页面/端口/Playwright 和共享编译缓存的高内存检查串行，不能两个浏览器运行同时清 test-results。

| 任务 ID | 所属里程碑 | 交付物/影响路径 | 前置成果 | 检查 ID |
| --- | --- | --- | --- | --- |
| T-1 | M2 | §5.2 的切换/展开/状态选择与原生链接，primitives 与 Themes 文件、模型/SSG 增量 | V-1 的基线与正式输入 | V-2 |
| T-2 | M3 | §5.1 core Layer/placement/parent/focus/abort、局部模态 ARIA 与通知动态父引用/子树重绑及两 host 结构 CSS | V-1；既有 Layer 契约 | V-3 |
| T-3 | M3 | §5.3 floating 公共生命周期与 Dialog/AlertDialog、便捷调用/测试；Content 的 children/notifications host 与迁移生命周期接线 | T-2 的 core 签名/纯模型检查先通过，包含局部模态投影、动态父层、换宿主及防自环；主 agent 集成 | V-3 |
| T-4 | M4 | §5.4 Popover/HoverCard/Tooltip/Arrow、timers/grace 与完整面板 | T-2、T-3 的稳定接口及 V-3；阶段终验需 T-1 | V-4、V-6 |
| T-5 | M5 | §5.5 静态内容/Avatar/比例/分隔及 §5.7 皮肤、content_ssg 对应部分 | V-1；已有 Layout/geometry/主题 | V-5、V-8 |
| T-6 | M5 | §5.5 ScrollArea 原语/皮肤、真实 scrollport 与模型测试 | V-1；方向/监听/geometry 稳定契约 | V-5、V-8 |
| T-7 | M6 | §5.6 core 通知及 Progress/Spinner/Skeleton、feedback_ssg/真实样本 | T-2/T-3 的有效父层引用、子树重绑/回焦及 Content 通知宿主接线，含 V-3 对应模型证据；不修改 T-4 的 hover 资源 | V-7、V-10 |
| T-8 | M7 | 公开指南/能力表、hydration 扩展、消费适配、CI 与统一验收 | 全部组件交付和相关局部检查；V-6 阶段一出口 | V-8—V-14 |

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| core 层级/回焦影响旧 Layer 与通知键盘顺序 | 现有 Select/Dialog、region anchor 与 native Draft | 局部模态属性、通知注册/迁移/防自环及回焦规则已由 §5.1/§5.6 定义；实施者通过 V-3、V-6、V-11，出现反例就地修订相关正文 | M4 / M7 对应收口 | 否，技术路线与回归入口已定 |
| 来源是本地快照，上游 commit 未知 | 可取回性与许可归因 | M1 正式纳入采用文件、逐包版本/文件摘要与 MIT；不伪造上游 SHA，不把 ref 当构建依赖 | M1/V-1 | 否，本地输入当前可读 |
| 旧同功能 p50/p95 与产物差值尚未测得 | 无法预先给出回退数字/排期 | 实施者在旧行为改动前留 V-1 基线，最终 V-13 同条件比较；新能力独立计成本 | M1 基线门 / M7 | 否，未知是待测结果，不改变已定方案 |
| P2 UI R1 作者修复尚待独立复审 | 不能宣称 P2 视觉整体通过 | 复用当前实际 API 与具体代码证据；本期相关 UI 自己按 V-6/V-11 真实验证，不替 P2 关闭评审 | M4 / M7 | 否，不依赖其独立评分 |
| macOS Chrome 以外、VoiceOver/OS IME/真机触控/UI 缩放可能缺现场证据 | 不能对未测环境作支持/真人结论 | 实施者列实际 missing、所需设备/人工动作，保留 P5 对应终验；能用现有工具执行的真实检查由 Agent 执行 | M7，完整终验在 P5 | 否，非 P3 新增真人门槛 |
| 公共 Tooltip typed 入口与 Toast 字面量有编译适配 | 仓内直接调用与公开说明 | 主 agent 同批更新已定位调用、文档及 V-12/V-14；外部使用匹配 source，未发布 API 不冒称 registry 可用 | M4 / M7 | 否，迁移决策已明确 |

- 待用户拍板：无。没有影响范围、安全、数据或关键契约的未决产品选择；实现细节与可行备选已收敛。
- 最终状态：**Ready**。
- 定级理由：P3 的 12 AC、前置、公共接口/默认值、所有权与失败语义、迁移和分层验收可执行；局部模态属性与 scope 边界一致，通知新/既存后代的绑定、迁移、防自环与回焦规则已确定；所有开发对象都在本仓已授权范围，没有需要外部系统新增能力的开工阻塞。Ready 不表示实现完成、独立评审通过或跨期需求终验。

## 12. 已知坑与历史教训

- 内部 ssr 命名只供构建期 SSG；不要重走流式/SSR 服务调查和测试（AGENTS.md「仅 CSR/SSG」；P2 代码复审 R3「产品路线与本轮范围校准」）。
- host lib tests 不覆盖 wasm 监听、焦点与 Portal；必须用真实产物的 V-6/V-9/V-11 补边界，不能用 host 通过推断浏览器通过（AGENTS.md「目标拆分」）。
- 缺 Root/重复存活部件不得 `.expect()` 导致 abort；保留 children 和可定位诊断，注册数随卸载恢复（P2 代码复审 R1/R3；primitives/select.rs 的缺 Root/注销分支）。
- Label/Title/Description 不能只靠构造时是否已登记来决定静态关联；延迟 children 也要产生正确引用，不能靠 hydrate 再修（P2 R3 的字段关联结果；form/ssr_field.rs；本期 SSG 测试覆盖同类情况）。
- 页面监听和短计时器都必须能在 abort 时由 JS 释放；只靠 Rust Drop 或 runtime::defer_after 的无句柄延迟链不足以实现可取消 hover/暂停计时（architecture.md；Select SearchTimer）。
- Layer 的 modal inert 不证明外部宿主没有被遮罩截住；本期对 scope 外做真实 pointer/focus 命中检查，并保留其他独立实例故障隔离（overlay.rs、dialog.rs、PRD §2.7）。
- 新组件 class 改动后同步生成 CSS/candidates；示例自有 utility 不放入 SDK 产物。Styles constants/包 include 必须覆盖新增皮肤，Docs 不跨 crate include 私有文件（AGENTS.md；styles.rs）。
- 24 是导航分类，计划覆盖不等于已实现；新组件入口与 STABLE_REFERENCES/SUB_CAPABILITIES 同步，未交付项不提前转 Yes（catalog.rs；WORK-N5）。
- 同一浏览器验证只跑一个 project；serve 不重建，源码修改后先 build 对应产物，不用旧截图/旧结果填当前通过（AGENTS.md「浏览器测试」；README）。
- 里程碑记录只保存行为、有效证据与未完项；按仓内 SDLC v3 的事件要求同步最小状态事件与当前页，不升级协议，不复制验证报告（.xgent-ai/sdlc/protocol.md）。

