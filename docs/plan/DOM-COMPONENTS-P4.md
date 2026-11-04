# DOM-COMPONENTS-P4 · Rustify UI · 完整导航与参考交互（同源菜单/导航/工具栏 · Studio 完整陈列 · 跨期终验）

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
> 调查基线：2026-11-03 · `0d96270fc5eae9cc9a71774649f3254499bf3f20` · main / dirty；产品源码与测试无改动，未提交内容为 SDLC 台账、`skills-lock.json` 及 P3 评审/覆盖报告，事实来自当前工作树。
>
> 需求来源：[DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) v0.2 · **P4（第 4 期）**，§7.2 的 WORK-F1/AC-3、WORK-F13/AC-1—AC-3、WORK-F15/AC-4、WORK-F19/AC-2、WORK-N1/AC-2、WORK-N3/AC-3、WORK-N6/AC-2、WORK-N7/AC-2，共 10 条 AC；WORK-F1、WORK-F15、WORK-N3、WORK-N6 在本期整条终验（§7.3）。
>
> 本期交付：**同源 Primitives/Themes 的 DropdownMenu、ContextMenu、Menubar、NavigationMenu、Toolbar 与旧 Menu 迁移；Theme Studio 完整参考陈列与部件级 Inspector；全部交互组件的两表达等价、浮层/导航/反馈可访问性、清理与故障、默认视觉的跨组件终验。**
> **开发范围：DOM SDK（rustify-dom 主题来源标记、rustify-components 菜单/导航/工具栏及其样式与目录）、纯 DOM 消费者与静态接管 fixture、Theme Studio 预览/Inspector/诊断、受影响示例/Docs 调用方与文档。依据见 §0。**
> 本期独特职责：在 P3 的受控浮层栈上补齐命令菜单、网站导航和工具栏三种不同语义，并对 P2—P4 已交付的全部交互组件完成跨期终验。
> **顶层排除：GPU 新组件/菜单、Rustify 扩展组件（DataTable 等）的主题迁移与 Studio 检查、全库公共入口迁移指南终稿、人工辅助技术终验、请求时 SSR、发行。**

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

RULE @delivery_first:
  MUST: ["优先交付里程碑的用户可用行为；事务、锁、重试、幂等、降级、补偿、自愈、沙箱/隔离、额外校验与告警只实现正文已定义或仓库规则要求的",
         "实现以性能等 NFR 为目标：同一问题开销相近时选更优写法，避免明显低效",
         =>[unplanned_failure_path_found, "仅当本期真实路径会触发、后果不可接受且现有框架/约束不足时用最小机制处理，就地补正文触发与后果并在记录写明偏差；否则失败时返回清晰错误"],
         "测试围绕 §9 检查对应的行为、关键边界与实施中发现的真实缺陷，每个行为用能捕获回归的最少用例，优先扩展既有测试"]
  MUST_NOT: ["为假想故障、内部契约或框架已保证的行为加防御、回退或测试",
             "为正文未列为本期的 NFR 做基线、压测、专项验证、调优迭代或引入专门基础设施",
             "用 fallback 或静默恢复掩盖编程错误",
             "为覆盖率、逐个函数或穷举非法输入批量写测试"]

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

- 最近更新：2026-11-04T15:30:13+08:00
- 当前进度：7/7 个里程碑完成
- 当前状态：已完成
- 最近完成：M7 · 全部 AC 与整条终验收口
- 下一步：代码评审 R1 的 G-01、F-01、F-02 与 Indicator 偏差已修复并验证（[M7 评审修复](DOM-COMPONENTS-P4.records/M7.md#评审修复)）；由用户发起 `review-code docs/plan/DOM-COMPONENTS-P4.md` 复审
- 当前阻塞：无；待负责人决定是否为默认调色板变化补迁移说明（R1 待决策项）；D11 参考配色按 Radix UI 保留并记录实测值，人工辅助技术项目按计划留 P5
- 代码基线：`16ed128b`；M1、M2—M4、M5—M7 均已提交，V-1—V-15 通过；既有评审台账/报告/skills-lock 改动保留，未推送

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-04T09:30:33+08:00 | 旧 Menu 同功能 before 与 19 个正式参考文件入仓；V-1 通过 | [M1 记录](DOM-COMPONENTS-P4.records/M1.md) | `0b6f0d6c` |
| M2 | 已完成 | 2026-11-04T09:58:00+08:00 | 共享菜单、两族原语/皮肤、便捷入口与 DOM 调用方迁移；V-2 通过，浏览器留 M4 | [M2 记录](DOM-COMPONENTS-P4.records/M2.md) | `60c0a367` |
| M3 | 已完成 | 2026-11-04T10:16:42+08:00 | Menubar/Toolbar 原语与皮肤、工具栏 roving 接管、两消费者样本；V-3 通过 | [M3 记录](DOM-COMPONENTS-P4.records/M3.md) | `60c0a367` |
| M4 | 已完成 | 2026-11-04T11:21:11+08:00 | 导航、两消费者及调用方阶段出口完成；V-4/V-5 通过，含验收发现的焦点与命名修正 | [M4 记录](DOM-COMPONENTS-P4.records/M4.md) | `60c0a367` |
| M6 | 已完成 | 2026-11-04T11:43:28+08:00 | 两侧受控等价夹具与共享 36 用例已集成；V-7 通过，浏览器见 M7 | [M6 记录](DOM-COMPONENTS-P4.records/M6.md) | `16ed128b` |
| M5 | 已完成 | 2026-11-04T12:08:19+08:00 | 43 组陈列、来源 Inspector 与诊断；V-6 与定向浏览器通过，修复多 Provider Toast ID | [M5 记录](DOM-COMPONENTS-P4.records/M5.md) | `16ed128b` |
| M7 | 已完成 | 2026-11-04T15:30:13+08:00 | V-8—V-15 通过；10 条 AC、四条整条终验可追溯；R1 评审的目录同步与两处 ARIA 已修复复验 | [M7 记录](DOM-COMPONENTS-P4.records/M7.md) | `16ed128b` |


## 0. 需求、范围与决策

### 0.1 需求与约束账本

本计划是既有 SDK 的 DOM 增量与跨期终验。功能 AC 来源均为 [PRD §3、§7.2](../PRD-DOM-COMPONENTS.md)，不复制验收原文。“整条终验”表示本期须连同前期 AC 的当前回归证据一起闭合。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F13 | 功能 | PRD / AC-1—AC-3 / P4 | 命令菜单（分组/分隔/禁用/勾选/单选/子菜单、指针与键盘进入/退回/字符定位/关闭顺序）、ContextMenu 右键与键盘、Menubar 顶层切换、NavigationMenu 原生链接、Toolbar 组内焦点且不吞字段按键；现有 Menu 调用方有去向 | §3、§5.1—§5.5；V-2—V-5、V-8、V-10、V-11 | 已确认，本期终验 |
| WORK-F15 | 跨期功能 | PRD / AC-4 / P4；AC-1—AC-3 已在 P1—P3 | 菜单、导航和工具栏遵守同一视觉参数契约，无“接受但无效”的参数 | §5.6；V-2—V-5、V-10；整条回归 V-13 | 本期验 AC-4，整条终验 |
| WORK-F1 | 跨期功能 | PRD / AC-3 / P4；AC-1 P1、AC-2 P2 | 全部已覆盖交互组件在 Primitives/Themes 两表达下，接受/拒绝同一请求的值、焦点、关闭结果一致，离散动作只交付一次 | §5.9；V-8；整条回归 V-12、V-13 | 本期验 AC-3，整条终验 |
| WORK-F19 | 跨期功能 | PRD / AC-2 / P4；AC-1 P2、AC-3 P5 | Studio 中完整参考集合的交互/内容/导航有真实 DOM 样本与结构说明；从实际部件定位色族、语义角色或覆盖来源，浮层继承来源可辨认 | §5.8、§6；V-6、V-9 | 本期验 AC-2，整条 P5 终验 |
| WORK-N1 | 可访问性 | PRD / AC-2 / P4；AC-1 P2、AC-3 P5 | 浮层、导航、反馈的焦点顺序、模态、关闭和状态宣告；低对比主题有带上下文的诊断，不凭色阶名断言合格 | §5.1—§5.5、§5.8、§7；V-9、V-10 | 本期验 AC-2，整条 P5 终验 |
| WORK-N3 | 隔离/清理 | PRD §2.7 / AC-3 / P4；AC-1—AC-2 P1 | scope 释放或浮层关闭后监听/定位/焦点限制/资源不再影响页面；实例致命故障只终止本实例 | §5.1、§5.5、§8；V-11；整条回归 V-13 | 本期验 AC-3，整条终验 |
| WORK-N6 | 可靠性 | PRD §2.7 / AC-2 / P4；AC-1 P1 | 触发器移除、内容变化、浮层取消、GPU 局部故障后剩余 DOM 任务可继续；fatal 按实例重启 | §5.1—§5.5、§8；V-11；整条回归 V-13 | 本期验 AC-2，整条终验 |
| WORK-N7 | 视觉/响应式 | PRD / AC-2 / P4；AC-1 P1、AC-3 P5 | 默认浅/深主题一致；每个声明变体在其交互状态下可辨认；加载/空/错误布局不丢关键操作 | §5.6、§5.8、§6；V-10 | 本期验 AC-2，整条 P5 终验 |
| WORK-F4 | 前置契约 | PRD / P1；theme/provider.rs | 局部主题边界与浮层继承创建处有效主题 | §5.8 的主题来源标记；V-6、V-9 | 已有契约，本期只加可辨认标记 |
| WORK-F5 | 前置契约 | PRD / P2；primitives/slot.rs | typed 单元素组合、稳定部件状态、缺上下文诊断 | §5.1；V-2—V-5 | 前置复用 |
| WORK-F6 | 前置契约 | PRD / P2；primitives/choice.rs | CheckedState 三态与 toggle_group 选择语义复用到 CheckboxItem/Toolbar | §5.1、§5.4；V-2、V-3 | 前置复用 |
| WORK-F11、WORK-F12 | 前置契约 | PRD / P3；overlay.rs、primitives/floating.rs | scope 局部模态、最上层处理 Escape/外部交互、回焦与清理 | §4、§5.1；V-5、V-10、V-11 | P3 已终验，回归本期影响 |
| WORK-F22 | 阶段边界 | PRD / AC-1、AC-2 在 P5 | Rustify 扩展组件的主题/接口迁移与大数据能力 | §0.5 | 非本期；仅在调用新入口处适配 |
| WORK-F23 | 阶段边界 | PRD / AC-2、AC-3 在 P5 | 同步本期公开入口、能力表与旧 Menu→新部件说明；全库迁移终稿不提前认领 | §3、§9.5 | 本期跟随文档 |
| WORK-N2 | 性能 | PRD / AC-2 在 P5；PRD Q4；AGENTS.md「性能」 | 被替换的菜单热路径留同负载前后 p50/p95 与压缩体积；新能力单列 | §7、§9.2；V-1、V-14 | 本期增量测量，非整条终验 |
| WORK-N4 | 消费模式 | PRD / AC-1 P2、AC-2—AC-3 P5；AGENTS.md「仅 CSR/SSG」 | 默认无 Makepad；SSG 同步 HTML 与确定性 hydrate；资源外置 | §5.7；V-12 | 本期增量回归 |
| WORK-N5 | 可维护性 | PRD / AC-2 在 P5；catalog.rs | 目录、文档与实际交付一致 | §3、§9.5；V-15 | 本期同步 |
| WORK-N8 | 来源 | PRD / P1；sources.lock.json | 新采用的参考源与许可正式入仓，构建不依赖 ref/ | §1.2、§4；V-1、V-15 | 既有规则覆盖本期新增输入 |
| C-1 | 前置 | DOM-COMPONENTS-P3 恢复快照 7/7、代码复审 R3 通过、[P3 覆盖核查](DOM-COMPONENTS-P3.prd-dev-gaps.md) 12/12 | 当前 source SDK/CSS 配对；不以公开 0.2.x 代表源码 API | §3、§4 | 已核实足够 |
| C-2 | 运行时 | AGENTS.md「运行时契约」；docs/architecture.md | 应用持有 open/value；Draft 例外；scope/实例 abort；页面监听经 `rustify_ui::listen` | §5、§8 | 已确认 |
| C-3 | 工具/环境 | AGENTS.md「工具链」「浏览器测试」；mise.toml | mbx 编译；固定 Rust/Tailwind；Playwright 每命令一个 project、串行；不装服务、不升 major、不开分支 | §9、§10 | 已确认 |
| C-4 | 样式 | AGENTS.md「编码与样式约定」；sdk.css、styles.rs | 无前缀 utilities + tw_merge；改 class 后 `mbx xtask css` 并提交 rustify.css；无全局 reset；CSSOM 动态样式 | §5.6；V-2—V-4、V-15 | 已确认 |
| C-5 | 交付边界 | PRD §1.3、§7；AGENTS.md「项目定位」 | 不新增 GPU 组件；Rustify 扩展与 GPU 预览只做兼容回归；不发行 | §0.5、§1.2 | 已确认 |
| C-6 | 验收边界 | PRD §6.3；AGENTS.md「前端 UI 开发与验收」 | Agent 走真实浏览器主路径；VoiceOver/OS IME/真机触控/真实 UI 缩放缺证列 missing，归 P5 | §9.3；V-10、V-15 | 已确认 |

无支撑设计的未证实重大假设。菜单同功能耗时、新负载耗时与产物差值在测量前未知，不用于架构取舍。

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 包与入口 | 留在 rustify-dom / rustify-components；新增 `primitives::{dropdown_menu, context_menu, menubar, navigation_menu, toolbar}` 与同名 Themes 模块；三类命令菜单共享 crate 内 `primitives::menu` | 不增 crate/feature；沿 P2/P3 “同名模块 + 原语重导出 + 皮肤部件”模式 | WORK-F1、WORK-F13；popover.rs、dialog.rs |
| D2 | 菜单生命周期 | 复用 floating 受控生命周期，新增 `Kind::Menu` | 共享 Root/Portal/Content 注册、诊断、清理与回焦；floating 增分支 | ADR-1 |
| D3 | 子菜单关闭语义 | Escape/ArrowLeft 只关当前子菜单并回到 SubTrigger；选择项与菜单树外指针整树关闭 | 与 P3「最上层处理关闭」和 APG 一致；与 Radix（Escape 整树关）差异写入文档 | ADR-2；WORK-F11/AC-3 |
| D4 | 菜单内 Tab | 在菜单内容内消费 Tab（不移出），退出用 Escape/选择/外部交互 | 与参考 Primitives 一致；不实现 APG“Tab 关菜单并移到下一停靠点” | ref menu.tsx Tab 处理；§5.1 |
| D5 | 旧 Menu | 保留数据驱动 `Menu`/`MenuItem`/`anchor_at` 便捷入口，内部改为 Themes 菜单部件组装；**非模态**并新增外部指针/焦点关闭（锚点元素或 canvas 之外；canvas 内由应用经 GPU action 决定） | GPU region/点锚定调用方不变签名；外部交互从“不关闭”变为关闭，迁移说明写明范围 | WORK-F13「现有 Menu 调用方有迁移去向」；§3 |
| D6 | DOM 触发器调用方 | 组件目录的下拉/右键演示与 Studio 场景菜单迁到 `dropdown_menu`/`context_menu` 部件；GPU 锚定菜单（目录 region-chooser、property-workbench、Studio GPU dropdown）保留便捷 Menu | 演示真实新组件；GPU 调用方无 DOM Trigger，不强改 | §3、§1.2 |
| D7 | NavigationMenu 结构 | Content 内联渲染在所属 Item 内（自然 Tab 顺序），Viewport 是测量与外框部件；链接带 `data-rustify-native-navigation` | 无需 Radix 的 focus proxy；与 TabNav 原生导航一致 | ADR-3；architecture.md「Native page links」 |
| D8 | Menubar/NavigationMenu/Toolbar 视觉 | 参考 Themes 无同名组件：栏/导航/工具栏用 Rustify 语义外观（class 可覆盖，不造 size/variant）；Menubar 的菜单内容复用 base-menu 皮肤及同一 Content 视觉参数 | 不虚构参考参数；同一菜单内容一套视觉契约 | WORK-F15/AC-4；ref base-menu.props.ts |
| D9 | Studio 陈列与 Inspector | 新增 “Components” 预览场景；Inspector 从实际部件的 CSSOM 规则推导所用 token，读取 SDK 主题来源标记确定根/边界/浮层来源；拾取模式放行会打开浮层的用户激活 | 新组件无需手写 token 表；浮层可检查；需 core 小增量 | ADR-4；WORK-F19/AC-2 |
| D10 | 两表达等价证据 | 双项目同断言的既有 specs（panels/content/feedback）+ 本期 menus.spec 计为等价证据；为缺共享场景的 P2 表单/选择族新增 parity.spec 与两侧同 test id fixture | 不重复已有场景；Themes 独有组件（SegmentedControl、TabNav、卡片、Button/Link）不在等价对比集合 | WORK-F1/AC-3；§5.9 |
| D11 | 参考配色与统一对比度门槛 | 与 Radix UI 保持一致，保留其色阶及各组件标准/高对比配色规则 | 对有参考源码依据的低对比组合逐项记录实测值和诊断，不改参考色板、不虚称达到 4.5:1；其余默认 Rustify 语义配色仍按既定门槛验收 | 用户本轮答复“和 radix-ui 保持一致”；[M7 决策证据](DOM-COMPONENTS-P4.records/M7.md#配色决策) |

### 0.3 ADR-lite

#### ADR-1：菜单复用 floating 受控生命周期

- 状态：Accepted。
- 背景与驱动：WORK-F13 需要 DropdownMenu、ContextMenu、Menubar 及子菜单的开闭、外部交互、回焦和清理；P3 的 floating 已承载 Dialog/AlertDialog/Popover/HoverCard/Tooltip，并修过“唯一 Content 卸载后的死遮罩”“aria-controls 仅开启时输出”等生命周期缺陷。
- 备选 A：像 Select 那样直接在 `Layer` 上写菜单生命周期。菜单语义不碰共享代码，但要重做部件注册、重复/缺失诊断、Content 清理顺序与回焦，复制 P3 已修复的缺陷面。
- 备选 B：floating 增加 `Kind::Menu`，菜单 Trigger/Content/Item 的键盘、typeahead、子菜单逻辑放在 `primitives::menu`。共享代码增加少量分支（haspopup、虚拟锚点、角色）。
- 决策：选 B。floating 增加 `Kind::Menu` 与 §5.1 列出的锚点/角色/注册增量；菜单专属的键盘、typeahead、子菜单与整树关闭留在 `primitives::menu`。
- 正面后果：一套开闭/清理/诊断；子菜单作为嵌套 Layer 自动成为父层子节点，随父层撤下。
- 负面/中性后果：floating 改动需回归 P3 浮层（V-5 的 panels.spec 两个 dom 项目、V-8 复跑）。
- 重新评估触发：菜单需要 floating 不具备的生命周期（例如跨 scope 宿主）；不是因代码行数。

#### ADR-2：子菜单逐层关闭，菜单树外指针整树关闭

- 状态：Accepted。
- 背景与驱动：现有栈只让最上层处理 Escape 与外部指针（overlay.rs 的 `route` 与 document capture pointerdown 只读 `layers.last()`），父层撤下时子层随之 retire 但不通知应用。
- 备选 A：改 core，使外部指针按每层独立判定（Radix DismissableLayer）。会改变 P3 已验收的 Dialog→Popover→Select“只关顶层”语义。
- 备选 B：core 不变；菜单在组件层补两条规则：①Sub 观察根菜单 open 由 true 变 false 时请求自身关闭一次（程序关闭，不冒充用户取消）；②根菜单在任一 Sub 打开期间按需持有一个 document capture `pointerdown` 监听，目标在整个菜单树（根 panel，含其内嵌的子层宿主）和根 Trigger 之外时，以 `LayerDismiss::PointerOutside` 构造 `Cancelable`，依次运行根 Content 的 `on_pointer_down_outside` 与 `on_interact_outside`，未取消才请求根关闭一次。
- 决策：选 B；Escape 按 core 逐层（APG），与 Radix 的整树关闭差异写入 docs/menus.md。
- 正面后果：不改 P3 契约；一次外部点击关闭整棵菜单树。
- 负面/中性后果：多一个按需监听，须 abort 绑定并在 Sub 全部关闭/根关闭/清理时释放。
- 重新评估触发：出现第二类需要整树外部关闭的浮层族，再考虑上移到 core。

#### ADR-3：NavigationMenu 内容内联，Viewport 只做测量与外框

- 状态：Accepted。
- 备选 A：照 Radix 把 Content 搬进 Viewport，并用不可见 focus proxy 维持 Trigger→Content→下一项的 Tab 顺序。参考一致，但焦点代理与搬动是额外的焦点/清理面。
- 备选 B：Content 留在 Item 内（DOM 顺序即 Tab 顺序），绝对定位到 Root 下方；Viewport 测量当前打开 Content 尺寸并写 CSSOM 变量作为外框，Indicator 读取活动 Trigger 位置。
- 决策：选 B；保持全部公开部件与状态，结构差异写入文档。Root 是 List/Item/Content 的唯一相对定位祖先，List/Item 保持 static；Indicator 位于 List 外的轨道元素（ul 只含 li）；Content 绝对定位到 Root 且 z-index 高于 Viewport。
- 正面后果：无焦点代理；SSG 可直接输出 `force_mount` 的内容链接。
- 负面/中性后果：多个 Item 共用 Viewport 动画的视觉需由 CSS 变量过渡实现，不复制 Radix 的内容迁移动画。
- 重新评估触发：真实布局证明内联内容无法与 Viewport 对齐或被祖先 overflow 裁切且无法用局部样式解决。

#### ADR-4：Inspector 从实际部件推导 token 与主题来源

- 状态：Accepted。
- 背景与驱动：现有 Inspector 只认应用手写的 `data-theme-tokens`，SDK 组件只输出 `data-name`/`data-rustify-part`；浮层 portal 到 Inspector 监听根之外；来源只读根主题（inspector.rs:255-300、controller.rs:252-254）。
- 备选 A：在 Studio 维护 part→token 手写表。每个新部件都要同步，容易与 CSS 漂移。
- 备选 B：对选中元素及其最近部件，遍历 `document.styleSheets` 与 adopted sheets（递归 `@layer`/`@media`/`@supports`/`@container`）；对每个顶层 style rule 取去掉伪元素、用户动作伪类（`:hover`、`:active`、`:focus*`、`:checked` 等）与运行态属性条件（`[data-state]`、`[data-highlighted]`、`[data-disabled]`、状态位 `[aria-*]`）后的主体选择器与两者做 `matches`，命中后从该规则完整 `cssText`（含嵌套变体）收集 `var(--<token>)`（compiled utilities 与皮肤直接引用 token 变量，如 rustify.css 的 `.bg-popover { background-color: var(--popover) }`）；组件局部色板由最近 `[data-color]` 与 `--rustify-component-color-N` 得出色族与阶；来源帧由 SDK 新增的 `data-rustify-theme-source` 确定。
- 决策：选 B；语义与元素当前状态无关，表示“该部件在任何状态下引用的 token”，状态与容器条件不参与命中判定；仍保留 `data-theme-tokens` 作为显式标注的优先来源，无匹配显示“未标注”。
- 正面后果：部件、皮肤和浮层自动可检查；来源随实际边界。
- 负面/中性后果：一次规则扫描约与样式规则数线性，按“部件名+class+变体属性”缓存，拾取与悬停预览共用，按键首次计算；结果不是当前状态生效值，有效值另由解析值/计算样式显示；不在热路径。
- 重新评估触发：规则扫描在 Studio 实测造成可感知延迟，或样式改为无法通过 CSSOM 读取的形式。

### 0.4 职责与事实所有权

|  | 应用/消费者 | DOM core 与 Primitives | Themes、Studio 与示例 |
| --- | --- | --- | --- |
| 拥有 | 每个菜单/子菜单/Menubar 值/导航值的 open 与 value、勾选/单选值、命令执行与导航目标 | 临时焦点、typeahead 查询、悬停意图/宽限、部件关联、层级/定位/释放、主题来源标记 | 声明的视觉参数与 CSS、陈列样本、Inspector 推导与诊断展示 |
| 不拥有 | 浏览器监听/计时器释放 | **替应用执行命令、修正业务值、劫持字段编辑按键、改写宿主主题或拦截原生导航** | 第二套菜单行为、另一份主题事实源、编造未标注 token |

命令与业务结果由应用决定；组件只发请求并投影接受值；core 与原语释放其实际创建的浏览器资源。

### 0.5 明确不在本期

- **Rustify 扩展组件**（DataTable、Tree、FilePicker、DropZone、Splitter、PanelTabs、CommandPalette、Icon/Glyph）的主题/接口迁移与 Studio 检查——P5（WORK-F22、WORK-F19/AC-3）；本期只在它们实际调用新入口时做适配。
- **全库旧→新迁移终稿与全部示例迁移**——P5（WORK-F23/AC-2—AC-3）；本期只写 Menu 与新增入口的迁移段。
- **人工辅助技术、OS IME、真机触控、真实浏览器 UI 缩放、200%/放大全路径**——P5（WORK-N1/AC-3、WORK-N7/AC-3）；本期 Agent 自动/模拟证据不冒充人工结论。
- **WORK-N2/AC-2 全量性能终验**——P5；本期只测被替换的菜单热路径与新增负载。
- **Studio 每个陈列样本的 Primitives 重渲染副本**——不做；结构演示以真实样本上的部件轮廓与原语路径说明表达（§5.8），P2 的 “States and parts” 工具保留。
- **菜单项 `item_as`/链接型菜单项、NavigationMenu 内容迁移动画、`data-motion`**——不做；目录限制列写明。
- 不新增 GPU 控件/菜单、GPU P3 色域；不改主题文档 schema、存储、导入导出；不实现请求时 SSR；不发行 crates.io、不部署站点。
- 既有 `theme-acceptance.spec.ts` V6 两处断言疑似与当前文案不一致（§11）——不在本期修复，只报告；本期按 `-g` 选择所需用例。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** P3 交付 7/7、代码复审 R3 通过、PRD 覆盖 12/12（[P3 恢复快照](DOM-COMPONENTS-P3.md#恢复快照)、[覆盖核查](DOM-COMPONENTS-P3.prd-dev-gaps.md)）。`OverlayStack` 在 scope 容器 capture keydown 中只把 Escape 交给最上层并在 Taken 时 stopPropagation；document capture pointerdown/focusin 只判定最上层；`Hierarchy::remove` 让普通后代随父层 retire（overlay.rs:1102-1150、1152-1301；hierarchy.rs:64-88）。子 Layer 经 `ParentContext` 自动成为父层子节点，并被搬入父 panel 的 children host，DOM 上是父 panel 的后代（overlay.rs:1817、1852、997-1008）。
- **[已核实·足够]** 关闭回焦依次为 trigger_id 元素/`Anchor::Element`、打开前焦点、父层首个可聚焦、scope 容器（overlay.rs:2034-2067）；顶层模态的 Tab 只在存在停靠点时包裹，否则交给浏览器（overlay.rs:1625-1660、`wrap_tab`）。
- **[已核实·缺口]** `floating::Kind` 只有 Dialog/AlertDialog/Popover/HoverCard/Tooltip；Trigger 固定 `aria-haspopup="dialog"`；锚点只来自 Anchor 部件或 Trigger 元素，`issue()` 与 Portal `active` 只按 anchors/triggers 判定；`button_as` 无条件挂 click 切换、无条件输出 `aria-controls`（floating.rs:5-29、82-85、105-110、244-252、363-371）。→ §5.1 增 `Kind::Menu`、菜单自带 Trigger、crate 内虚拟锚点与 Sub 的 `modal=false`；`button_as` 仍只服务既有 kind。→ 否则菜单角色/点锚定无法表达，直接复用会先开后关并让 aria 失效。
- **[已核实·缺口]** 现有 `Menu` 是数据驱动列表：`Layer anchor fit`，无外部关闭钩子（外部指针/焦点永不关闭），无部件、勾选/单选、子菜单、typeahead、ContextMenu 键盘（menu.rs:174-370）。仓内无 `menuitemcheckbox`/`menuitemradio`/`menubar`/独立 Toolbar 产品；公开 `ToggleGroup` 根已有 `toolbar` 参数并输出 `role=toolbar`（目录记 toggle-toolbar behavior），但目录把 Primitives Menubar/NavigationMenu/Toolbar 记为 No，DropdownMenu/ContextMenu 记为经 Menu 的 Partial（catalog.rs:960、1012、1101、1104、1157、1160、1164、1165、1180、1547、1549）。
- **[已核实·足够]** 可复用：`hover_grace::{polygon, contains}`（primitives 子模块可见，wasm/test）；`choice::navigation(count, here, step, looping)`（choice.rs:933，私有）；`CheckedState`/`selection_request`；`Shortcut` 的 `Display` 即 aria-keyshortcuts 形式、`rustify_ui::is_text_entry`；`rustify_ui::release_on_abort`；TabNav 的 `data-rustify-native-navigation` 被两种 router 放行（router.rs:780-810；tab_nav.rs:75、102）。各处可撤计时器均为私有副本（hover/browser.rs:6-37、select/browser.rs:5-39 等）。
- **[已核实·缺口]** Studio 无组件陈列；场景是字符串常量，持久化偏好对 scene 白名单校验，未知值拒绝整个 envelope（scenes.rs:12-20；persistence.rs:133-163）。Inspector 只认 `data-theme-tokens`，监听只挂 `.studio-preview-root`，浮层不可检查，来源只读根主题；`ThemeBoundary` 只输出空的 `data-rustify-theme-boundary`，Layer 投影主题变量但不标来源（inspector.rs:66-71、255-300；provider.rs:150-180、240）。→ §5.8。
- **[已核实·缺口]** 对比诊断只覆盖根主题 15 组 token 对，无部件或边界上下文（theme_controls.rs:50-87；app.rs:387-400）。→ §5.8。
- **[已核实·缺口]** 两消费者均未挂菜单；无跨项目对比；P2 表单/选择族在 raw（primitives.spec）与 styled（forms/slider specs）用不同场景与 test id（tests/dom-only/primitives/src/choices.rs、consumer/src/form_benchmark.rs）。→ §5.9。
- **[已核实·足够]** GPU 局部故障与实例 fatal 已有 Studio 用例：`theme-acceptance.spec.ts`（GPU Lost 时 DOM 继续编辑、缺字体 404、模拟 fatal 重启恢复草稿）与 `theme-gpu.spec.ts` 的 context restore；两实例 dispose/trap 在 consumer `index.html` 与 dom-only.spec.ts。本期复用并加入菜单/导航场景。
- **[已核实·足够]** 参考源本地可读：Themes 3.3.0 的 dropdown-menu/context-menu 与 `_internal/base-menu.{props.ts,css}`（Content size 1/2 默认 2 响应式、variant solid/soft 默认 solid、color、highContrast；Item color/shortcut；CheckboxItem color/shortcut；RadioItem color；Content 默认 align start、sideOffset 4、collisionPadding 10；Sub 默认 sideOffset 1、alignOffset −size×4；ContextMenu Content side right、sideOffset 2、align start、alignOffset −size×4）；Primitives 1.8.1 的 menu/dropdown-menu/context-menu/menubar/navigation-menu/toolbar/roving-focus（typeahead 1000 ms、子菜单指针 100 ms 打开、宽限 300 ms、ContextMenu 长按 700 ms、NavigationMenu delay 200/skip 300/close 150 ms）。均未入 vendor。

### 1.2 拓扑与文件清单

★ 为新增；其余为已定位的现有落点。

| 系统/模块 | 改动路径与符号 | 复用锚点与边界 |
| --- | --- | --- |
| 主题来源标记（core） | `crates/rustify-dom/src/theme/provider.rs` 的 ThemeScope/ThemeBoundary、`use_resolved_theme` 旁新增 `use_theme_source`；`crates/rustify-dom/src/overlay.rs` Layer 投影主题处写来源属性；`crates/rustify-dom/src/lib.rs` 重导出 | 只加标记与访问器，不改 ThemeDocument、解析或投影值 |
| 浮层生命周期 | `crates/rustify-components/src/primitives/floating.rs`（Kind、Context、虚拟锚点、Portal 锚点、content 角色/命名；`button_as` 不变） | §5.1 所列增量；菜单 Trigger 复用现有 Trigger 注册与 ARIA 输出，不经 `button_as`；不动 Dialog/Popover 等既有 kind 行为 |
| 菜单原语 | ★ `crates/rustify-components/src/primitives/menu.rs`（pub(crate)）、★ `primitives/menu/model.rs`、★ `primitives/menu/browser.rs`；★ `primitives/dropdown_menu.rs`、★ `context_menu.rs`、★ `menubar.rs`；★ `primitives/timer.rs`（pub(crate) 可撤计时器） | hover_grace、choice::navigation（改 pub(crate)）、CheckedState、Select 的 typeahead 算法同构 |
| 导航与工具栏原语 | ★ `primitives/navigation_menu.rs`（+ ★ `navigation_menu/browser.rs`）、★ `primitives/toolbar.rs`；`primitives/choice.rs` 的 toggle group 增 crate 内“由工具栏接管 roving”参数；`primitives/mod.rs` | TabNav 原生链接标记；`rustify_ui::listen`；`rustify_ui::is_text_entry` |
| Themes 组件 | ★ `crates/rustify-components/src/{dropdown_menu,context_menu,menubar,navigation_menu,toolbar}.rs`；`menu.rs`（便捷 Menu 重组，MenuItem 不变）；`lib.rs` | badge.rs/popover.rs 的皮肤与 geometry/component_color 模式；菜单滚动容器不自带 Tab 停靠点 |
| 样式/目录 | ★ `crates/rustify-components/css/menus.css`；`crates/rustify-components/css/primitives.css`（结构规则）；`crates/rustify-components/css/sdk.css` 导入；`crates/rustify-components/src/styles.rs` 的 ★ `MENUS`；`crates/rustify-docs/src/style.rs` 注册；`crates/rustify-components/src/catalog.rs`；重新生成 `crates/rustify-components/css/rustify.css`、`crates/rustify-components/css/candidates.json`、`docs/components.md` | 不新增 Category（保持 24） |
| 来源 | `sources.lock.json` ★ `radix-themes-menus`、★ `radix-primitives-menus`；★ `crates/rustify-components/vendor/radix/themes-menus/`、★ `vendor/radix/primitives-menus/` | 沿 `radix-themes-panels`/`radix-primitives-panels` 格式；上游 commit 未知写 null |
| 纯 DOM 消费者 | `tests/dom-only/consumer/`：★ `src/menus.rs`、★ `src/menu_benchmark.rs`、★ `src/parity.rs`、★ `public/menus.html`、★ `public/menus.js`、★ `public/parity.html`、★ `public/parity.js`，`src/lib.rs`（导出与两实例 Consumer 内菜单/导航样本）；`tests/dom-only/primitives/`：★ `src/menus.rs`、★ `src/parity.rs`、`src/lib.rs` | 两侧同 test id；raw 只加载 PRIMITIVES_CSS |
| 静态/接管测试 | ★ `crates/rustify-components/tests/menus_ssg.rs`；`tests/dom-only/hydration/src/view.rs` DeliveryParts；`tests/dom-only/hydration.spec.ts` | 不新增 Category section |
| 公开消费者（角色模拟） | `tests/dom-only/public-panel-consumer/`：★ `src/menus.rs`、★ `public/menus.html`、★ `public/menus.js`；沿其独立 Playwright 配置 | 只用公开入口与文档；不复用 `tests/dom-only/consumer`、`primitives` 的实现 |
| 浏览器测试 | ★ `tests/dom-only/menus.spec.ts`、★ `parity.spec.ts`、★ `menus-performance.spec.ts`；`tests/dom-only/dom-only.spec.ts`；★ `tests/browser/theme-gallery.spec.ts`；`playwright.docs.config.ts`（dom-primitives testMatch；dom 项目只在此配置）；`.github/workflows/verify.yml` | 端口沿 4190/4194/4181 |
| Theme Studio | ★ `examples/theme-studio/src/gallery.rs`（wasm 视图）、★ `src/gallery_catalog.rs`（host 可编译的陈列分类表）；`scenes.rs`、`app.rs`（scene_label、PreviewDiagnostics）、`persistence.rs`（scene 白名单）、`inspector.rs`、`controller.rs`、`storage_controls.rs`（TokenSource 根主题语义）、`theme_controls.rs`、`provider_fixture.rs`、`main.rs`、`app.css`、`README.md` | 中性编辑壳与 DESIGN.md 不变 |
| 受影响调用方 | `examples/component-catalog/src/main.rs`（下拉/右键演示改部件，region-chooser 保留）；`examples/theme-studio/src/scenes.rs`（cards-menu 改 dropdown_menu）；`examples/property-workbench/src/main.rs`、`examples/theme-studio/src/app.rs` GPU dropdown、`tests/docs-ssg/fixtures/components/src/{catalog.rs,custom.rs}`、`site/src/demos/feedback.rs`、hydration view（保持便捷 Menu，仅随行为回归） | 不改 GPU 渲染 |
| 文档 | ★ `docs/menus.md`；`docs/dom-components-migration.md`（Menu 段与“仍缺能力”段）、`docs/architecture.md`（菜单树关闭、主题来源标记）、`docs/themes.md`、README、`crates/rustify-components/README.md` | 只写本期事实 |

不改 makepad/、GPU 控件、router、主题 schema、发布脚本。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| floating（扩展） | dialog/alert_dialog/popover/hover_card/tooltip、★ 菜单三族 | 应用持有 open；唯一 Portal/Content；Trigger 关联随存活部件 | 无新接缝 | 注册/诊断/清理/回焦 | 进程内 + 真实 Layer | 既有 panels specs 回归 + 菜单 SSG/浏览器 |
| primitives::menu | dropdown_menu、context_menu、menubar、便捷 Menu | 项按 DOM 顺序；只有本 panel 的项参与导航；一次激活最多一个 on_select + 一个值请求 + 一个根关闭请求 | 无 | typeahead、子菜单意图/宽限、整树外部关闭 | 进程内；model 纯函数 | model 单测、SSG、两消费者浏览器 |
| navigation_menu | 应用 | value 外部所有；Content 内联；链接原生导航 | 无 | 悬停/跳过延迟、Viewport/Indicator 几何、外部关闭监听 | 进程内 + 浏览器 | SSG、浏览器 |
| toolbar | 应用；可嵌菜单 Trigger 与 toggle group | 单一 Tab 停靠点；字段与编辑按键不被吞 | 无 | 异构项 roving | 进程内 | SSG、浏览器 |
| 主题来源标记 | Layer、Studio Inspector | 边界在浏览器端自取唯一来源 ID；浮层投影创建处来源 | 无 | 无 | 进程内 | Studio 浏览器（theme-gallery.spec）；`theme::` host 仅回归 |
| Studio gallery/inspector | Studio 使用者 | 陈列覆盖分类表；无映射显示未标注 | 无 | CSSOM 规则匹配与缓存 | 浏览器 | gallery_catalog host 单测 + theme-gallery.spec |

删除测试：去掉 `primitives::menu` 会把同一键盘/子菜单/关闭语义复制到三族与便捷入口，保留；`primitives::timer` 有菜单、子菜单、导航三类新调用（≥3 同构用例），新增；既有私有计时器（hover、select、avatar、scroll_area、toast）不在本期迁移。不为 Menubar/NavigationMenu/Toolbar 造通用“组件框架”。

## 3. 公共 API 与迁移

无持久化 schema、导入导出或本地库变化。

1. 新入口：`rustify_components::primitives::{dropdown_menu, context_menu, menubar, navigation_menu, toolbar}::*` 与同名 Themes 模块 `rustify_components::{dropdown_menu, context_menu, menubar, navigation_menu, toolbar}::*`。部件与签名见 §5；不提供 React `asChild`/ref/defaultOpen/defaultValue。
2. `Menu`、`MenuItem`、`MenuItemKind`、`anchor_at` 签名不变。`Menu` 改为 `context`-free 的 Themes 菜单组装：Heading→Group+Label，Separator→Separator，Command→Item（禁用原因经 `aria-describedby`，快捷键显示并输出 `aria-keyshortcuts`），test id `menu-item-{id}`/`menu-shortcut-{id}` 保留；打开时聚焦首个可用项；**锚点元素或 canvas 之外的指针/焦点交互现在请求关闭**（canvas 内点击由应用经 GPU action 决定）；非模态；视觉改为 base-menu 皮肤默认值（size 2、solid）。
3. 组件目录下拉与右键演示、Studio `cards-menu`/`preview-menu` 改为部件写法，保留原 test id；GPU 锚定调用方继续便捷 Menu。
4. floating Trigger 的 `aria-haspopup`：Dialog/AlertDialog/Popover 仍为 `dialog`，菜单为 `menu`。菜单 Trigger 不复用 `button_as`，自行输出展开状态与开启时的 `aria-controls`；click 不重复请求。
5. `ThemeBoundary` 与 Layer 新增 `data-rustify-theme-source`；`rustify_ui::use_theme_source() -> Option<String>`。
6. 迁移指南写：旧 Menu 行为差异（外部关闭的范围、视觉、DOM 结构从 ul/li/button 到 div role=menuitem）、何时用部件/便捷入口、Escape/Tab 与参考差异、NavigationMenu 原生导航；“仍缺能力”段删除已交付项。回退以对应提交的源码、CSS、示例一起回退；本期 API 仅属 source 版本，不称已含于公开 registry 版本。
7. `ToggleGroup` 的 `toolbar` 参数保留为单组便捷用法；Toolbar 内的切换组固定 `toolbar=false`（`role=group`）。目录为五个模块的两种表达新增公开入口（yes）；参考对应中 Primitives Menubar/NavigationMenu/Toolbar 从 No、两库 DropdownMenu/ContextMenu 从经 Menu 映射，改为指向新入口的 Partial（与 P3 浮层同一口径：不提供通用 React props/ref/asChild）；ToggleGroup 记为单组便捷用法。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P3 浮层栈 / floating | 已核实缺口（菜单所需） | §1.1 所列 overlay.rs、floating.rs 行 | 范围内按 §5.1 增量；core overlay 不改 | 失效 Trigger/锚点沿 P3：请求一次关闭、撤下层 |
| P2 选择语义 | 已核实足够 | choice.rs `CheckedState`、`selection_request`、`navigation`、toggle_group Root | CheckboxItem 用 CheckedState；Toolbar ToggleGroup 复用 toggle_group 选择与请求，roving 交给工具栏 | 禁用/只读不请求 |
| 主题 provider | 已核实缺口（来源不可辨认） | provider.rs:150-180、240 | §5.8 增边界自取来源 ID 与 Layer 投影 | 无来源时 Inspector 显示根帧并标“来源未标注”；未登记边界显示“未登记的局部边界”，TokenSource 仍按根主题 |
| router | 已核实足够 | router.rs:780-810 放行带标记/修饰键/target 的链接 | NavigationMenu Link 带标记；Toolbar Link 不带标记，沿普通 scope 链接契约 | 原生导航由浏览器负责 |
| 正式参考输入 | 已核实可取 | ref 下 Themes 3.3.0、Primitives 1.8.1 本地快照 | M1 纳入 vendor 与 sources.lock，许可 MIT，上游 commit null | ref 只作导入源，构建不读 |
| Studio 持久化 | 已核实缺口 | persistence.rs:133-163 白名单 | 新 scene 名加入白名单 | 旧 envelope 仍可读；未知 scene 继续拒绝 |
| Docs/站点 | 已核实可接 | docs-ssg 组件 fixture、site demos 用便捷 Menu；rustify-docs style.rs 注册皮肤 | 注册 menus.css；行为随便捷 Menu 回归 | 不改站点系统 |
| Toast 调用方回归 | Catalog 的最终通知关闭丢失焦点 | catalog-inputs.spec.ts 关闭断言；toast.rs Root 清理在 keyed DOM 卸载之后 | Root 保留焦点归属；无目标 focusout 经实例托管任务延后清理，卸载沿现有回焦链 | 显式 blur 后不抢焦点，任务在实例 abort 时释放；不改通知排队语义 |
| Toast 陈列双 scope | 已复现 CSR 无 ID seed 时不同 Provider 的 Root ID 重复且文字未关联 | theme-gallery.spec.ts 重复 `rustify-toast-1`；feedback_ssg unseeded 用例先红；mount.rs 普通 CSR 不提供接管 seed | Toast Root/Title/Description 共用单调备用 DOM ID 分配；保留有 seed 时原编号 | 不改变 Scheduler ID、排队或接管协议；双预览 ID 唯一且标题/描述指向本 Root |

没有需要其他仓库、账号、服务或外部部署才能解除的依赖。

## 5. 核心机制与公共接口

### 5.1 floating 增量与共享菜单原语

**floating 增量：**

1. `Kind::Menu`：`anchored()=true`、`hover()=false`、`name()="menu"`；Content 角色 `menu`、`aria-orientation="vertical"`，有 Trigger 时 `aria-labelledby=<trigger id>`，否则用 `aria_label`，二者皆无给 `missing-name`。
2. 菜单 Trigger（§5.2）不复用 `button_as`，自行注册 `context.triggers`/`trigger_id` 并输出 `aria-haspopup=menu`、`aria-expanded`、只在开启时 `aria-controls`；`button_as` 保持 `aria-haspopup="dialog"` 与 click 切换，只服务既有 kind。
3. Context 新增 crate 内 `virtual_anchor: RwSignal<Option<rustify_ui::Anchor>>`（wasm），Portal 锚点顺序为 virtual → Anchor 部件 → Trigger 元素 → Centred；只由 context_menu 与便捷 Menu 设置；`issue()` 与 Portal `active` 把 `virtual_anchor` 视为锚点（三者皆无仍判 `missing-anchor`）。
4. Sub 一律以 `Kind::Menu, modal=false` 建嵌套 root，子层不得继承根 `modal=true`；菜单的打开焦点通过 Content 传入的 open hook 完成并 `prevent_default`，floating 默认 `focus_first` 不再执行；不新增 Context 字段。

**共享部件（`primitives::menu`，由三族模块重导出）：**

```rust
pub enum OpenFocus { Content, First, Last } // crate 内：Trigger/键盘写入，Content 打开 hook 读取
// Portal(force_mount=false, class, test_id, children)
// Content(loop_focus: bool = false, position: Signal<FloatingPosition> (族默认见下), aria_label: Signal<String>,
//         on_escape_key_down / on_pointer_down_outside / on_focus_outside / on_interact_outside:
//             Option<Callback<Cancelable<LayerDismiss>>>,
//         on_close_auto_focus: Option<Callback<Cancelable<()>>>, class, test_id, children)
// Group(children)   // role=group；含 Label 时 aria-labelledby
// Label(children)   // 不可聚焦
// Separator()       // role=separator，aria-orientation=horizontal
// Item(on_select: Callback<Cancelable<()>>, disabled: Signal<bool> = false, text_value: Option<String>, described_by: Signal<String>, class, test_id, children)
// CheckboxItem(checked: Signal<CheckedState>, on_checked_change: Callback<bool>,
//              on_select: Option<Callback<Cancelable<()>>>, disabled, text_value, described_by, class, test_id, children)
// RadioGroup(value: Signal<Option<String>>, on_value_change: Callback<String>, children)
// RadioItem(value: String, on_select: Option<Callback<Cancelable<()>>>, disabled, text_value, described_by, class, test_id, children)
// ItemIndicator(force_mount=false, children)   // 勾选/单选/不确定时渲染
// Sub(open: Signal<bool>, on_open_change: Callback<bool>, children)
// SubTrigger(disabled, text_value, class, test_id, children)
// SubContent(loop_focus=false, position (默认 Right/Start，RTL 为 Left), 同 Content 钩子, class, test_id, children)
// Arrow(class, test_id)
```

- 角色与状态：Item `role=menuitem`、CheckboxItem `menuitemcheckbox`（`aria-checked` 取 `CheckedState::aria()`，含 mixed）、RadioItem `menuitemradio`；项为 `div tabindex=-1`，`data-highlighted` 跟随焦点，`data-disabled`/`aria-disabled`，勾选类 `data-state`；SubTrigger `aria-haspopup=menu`、`aria-expanded`、`data-state=open|closed`，`aria-controls` 只在打开时输出。缺 Root/Content/Sub 上下文、重复 RadioItem value 给可定位 `data-rustify-issue`，保留可读内容并禁用失去契约的操作。禁用原因由调用方渲染，组件不合成文案；Item/CheckboxItem/RadioItem 的 `described_by` 引用该元素；`docs/menus.md` 给出部件写法下调用方渲染可见原因并以 `described_by` 引用的示例。
- 导航：keydown 时在当前 panel 内查询 `closest('[role=menu]') == 本 panel` 的可用项（子菜单 panel 在父 panel 的 children host 内，必须排除），用 `choice::navigation`（改 pub(crate)）计算：ArrowDown/Up、Home/PageUp→首项、End/PageDown→末项，`loop_focus` 默认 false。父 panel 的处理函数忽略 target 属于子 panel 的事件。
- typeahead：无 Ctrl/Alt/Meta 的可打印键追加查询并重启 1000 ms `timer::Timeout`；查询非空时空格视为字符；`model::next_match(labels, enabled, query, current)` 与 Select `Index::search` 同构（同字符重复时循环、大小写不敏感前缀、从当前项之后找）；标签取 `text_value`，缺省取元素 `textContent` 去空白。
- Tab：Content 内消费 Tab/Shift+Tab（`prevent_default`），焦点不移出。
- 激活（Enter/Space keydown 先 `prevent_default`，或 click，或在别处 pointerdown 后于该项 pointerup）：禁用项无请求。顺序：原生/调用方监听 → `on_select(c)` → CheckboxItem 请求 `on_checked_change(checked != CheckedState::Checked)`（Indeterminate 视为请求 true）一次；RadioItem 仅在 value 不同于当前时请求 `on_value_change(value)` 一次 → 若 `!c.default_prevented()`，请求根 `on_open_change(false)` 一次。
- 打开焦点：键盘打开（Enter/Space/ArrowDown）→ 首个可用项；ArrowUp → 末项；指针打开 → Content 本身（typeahead 可用、无高亮）。Content 的 open hook 先运行用户 hook，未取消才按 `OpenFocus` 聚焦。
- 指针：鼠标在可用项上 pointermove 即聚焦该项；离开项回到 Content（宽限期内除外）。
- 子菜单：SubTrigger 鼠标 pointermove 启动 100 ms Timeout 后请求 `Sub.on_open_change(true)`；click 或 ArrowRight（LTR）/ArrowLeft（RTL）/Enter/Space 立即请求并以 First 打开。SubContent 内 ArrowLeft（LTR）/ArrowRight（RTL）请求关闭，回焦经 trigger_id 回到 SubTrigger；Escape 由 core 交给最上层（即子菜单）关闭。离开 SubTrigger 指向已开 SubContent 时用 `hover_grace::polygon(exit, trigger_rect, content_rect)` 建立宽限区，300 ms 内父项的 pointermove 不夺焦、不关子菜单，期间临时 pointermove 监听进入子菜单或超时即释放；宽限外移到父项时该项获得焦点，core 的 focus-outside 使非模态子层关闭。
- 整树关闭（ADR-2）：Sub 观察根 open 由 true 变 false（或自身卸载）且自身 open 为 true 时请求 `on_open_change(false)` 一次。根在 “打开的 Sub 计数 > 0” 期间持有 document capture `pointerdown` 监听（`rustify_ui::listen`，实例 abort 绑定）：目标不在根 panel（含内嵌子层）与根 Trigger（Menubar 为 Menubar Root）内时，以 `LayerDismiss::PointerOutside` 构造 `Cancelable`，依次运行根 Content 的 `on_pointer_down_outside` 与 `on_interact_outside`，未取消才请求根关闭一次；子层在同一击中的关闭请求属预期，不计重复；计数归零、根关闭、清理时释放。
- 模态：`Root.modal` 默认 true（DropdownMenu/ContextMenu），沿 P3：本 scope inert、焦点外不关闭、外部指针请求关闭；Menubar 与便捷 Menu 非模态。

**族默认位置（Primitives）**：DropdownMenu Content = `FloatingPosition::default()`；ContextMenu Content = Right/Start、`side_offset=2`；SubContent = Right（RTL Left）/Start。Themes 覆盖值见 §5.6。

### 5.2 DropdownMenu 与 ContextMenu

| 入口 | 参数/部件 | 行为 |
| --- | --- | --- |
| `primitives::dropdown_menu` / Themes `dropdown_menu` | `Root(open, on_open_change, modal=true)`；`Trigger`/`trigger_as(HtmlElement<Button,..>)`（disabled）；其余为 §5.1 共享部件 | 菜单自带 Trigger（不复用 `button_as`）：主键 pointerdown（无 Ctrl）请求切换并以 Content 焦点打开，click 不重复请求，打开时阻止 Trigger 抢焦；Enter/Space 请求切换（First）；ArrowDown 请求打开（First）、ArrowUp 请求打开（Last）。`aria-haspopup=menu`、`aria-expanded`、开启时 `aria-controls` |
| `primitives::context_menu` / Themes `context_menu` | `Root(open, on_open_change, modal=true)`；`Trigger`/`trigger_as(HtmlElement<E,..>)`（disabled）；共享部件 | `contextmenu` 事件：`prevent_default`，以 `anchor_at(trigger, clientX, clientY)` 写 virtual anchor，未开则请求打开（Content 焦点）。打开期间按需持有 document capture `contextmenu` 监听（`rustify_ui::listen`，实例 abort 绑定）：坐标落在 Trigger rect 内时 `prevent_default` 并重写 virtual anchor——模态下 Trigger 被 inert，浏览器不会把事件送到它，故由该监听接管并阻止原生菜单；非模态由 Trigger 自身处理。非模态时 Trigger 区域内的主键 pointerdown 请求关闭一次（core 的 Region 锚点把 Trigger 视为内部）；Trigger 自身在已开或默认已阻止时只同步锚点，不重复请求。键盘：Trigger 上 `ContextMenu` 键或 Shift+F10 keydown → `prevent_default`、锚到 Trigger 元素、请求打开（First），并置 “键盘待消费” 标记，直到该键 keyup 前到达的 `contextmenu` 只消费不再请求。触控：非鼠标 pointerdown 启动 700 ms Timeout，move/up/cancel 取消，到期按触点打开。`disabled` 时不拦截浏览器原生菜单 |
| 便捷 `Menu` | 签名不变（§3） | crate 内以 Themes 部件组装，`virtual_anchor` 取 `anchor` prop，非模态；外部指针/焦点只在锚点元素或 canvas 之外请求关闭（canvas 内由应用经 GPU action 决定）；打开聚焦首个可用项；`on_activate(id)` 后请求关闭 |

Trigger 可聚焦性由调用方的原生元素保证；ContextMenu Trigger 需可聚焦才能键盘打开，缺 `tabindex`/非原生可聚焦元素时给 `data-rustify-issue="unfocusable-trigger"`。

### 5.3 Menubar

- `primitives::menubar` / Themes `menubar`：`Root(value: Signal<Option<String>>, on_value_change: Callback<Option<String>>, loop_focus=true, aria_label: String, children)` 渲染 `role=menubar`、`aria-orientation=horizontal`；`Menu(value: String, children)` 以 `open = value == Some(key)`、`change(open)` 映射到 `on_value_change`（关闭只在当前值为本 key 时请求 `None`），`modal=false`；Trigger `role=menuitem`、`aria-haspopup=menu`，其余部件同 §5.1。
- Trigger 间 roving：单一 `tabindex=0`（当前或首个可用），ArrowLeft/Right（RTL 反向）、Home/End，`loop_focus` 默认 true。ArrowDown/Enter/Space 请求打开（First），ArrowUp（Last）；鼠标 pointerdown 切换；有菜单打开时 pointerenter 另一 Trigger 请求切换到该 key（一次）并聚焦它。
- Root 在打开期间以 window capture `pointerdown` 记录目标是否在本栏已注册 Trigger 内；Content 的 `on_pointer_down_outside` 在命中时取消 core 的外部关闭，保证切换只请求新值一次（window capture 先于 core document capture），关闭/卸载释放监听。Content 的 `on_focus_outside` 在目标为本 Menubar 的 Trigger 时取消（切换请求由该 Trigger 发出，避免 core 另发关闭）；`on_close_auto_focus` 在 Menubar `value` 仍为 `Some` 时取消，只在 Escape 或 value 变为 `None` 后回到本 Trigger。
- Content 内 ArrowRight/Left（RTL 反向）请求相邻菜单 key（一次）并以 First 打开；子 panel 内只忽略后退键（先由子层关闭），前进键仍切到相邻顶层菜单；SubTrigger 上忽略前进键（自身打开子菜单）。Escape 由 core 关闭（请求 `None`），回焦到该 Trigger；Tab 消费。Menubar 自身不是 Layer。

### 5.4 Toolbar

- `primitives::toolbar` / Themes `toolbar`：`Root(orientation=Horizontal, loop_focus=true, aria_label: String, children)` 渲染 `role=toolbar`、`aria-orientation`、方向随 DirectionProvider；缺名称给 `missing-name`。
- Root 内嵌 `toggle_group` 时固定 `toolbar=false`（输出 `role=group`），避免 toolbar 套 toolbar；`ToggleGroup` 既有 `toolbar=true` 保留为独立便捷用法。
- 项：`Button(disabled, class, test_id, children)` / `button_as(HtmlElement<Button,..>)`；`Link(href, target, rel, download, children)` / `link_as(HtmlElement<A,..>)`（普通 scope 链接，Space 触发点击）；`Separator()`（方向与工具栏垂直）；`ToggleGroup(value: Signal<Vec<String>>, on_change: Callback<Vec<String>>, mode=Single, allow_empty=true, disabled)` + `ToggleItem(value, disabled)`，选择与请求复用 `toggle_group`，其 roving 由工具栏接管（choice.rs 的 group root 增 crate 内参数关闭自身 keydown 与 tabindex 管理）。
- `toolbar::item(disabled: Signal<bool>) -> impl Attribute`：给任意原生可聚焦元素加入工具栏 roving（注册顺序即渲染顺序，SSG 可确定 tabindex），用于 `dropdown_menu::trigger_as(button().add_any_attr(toolbar::item(..)))` 这类嵌入。
- Root 委托 keydown：`default_prevented`、composing、带 Ctrl/Alt/Meta、`rustify_ui::is_text_entry(target)` 时不处理；按方向键/Home/End 在 DOM 顺序的可用项间移动（loop），focusin 更新当前停靠点。字段与其编辑按键（方向/Home/End/选择）保持原生；菜单内容位于 overlay，不经过工具栏。

### 5.5 NavigationMenu

| 部件 | 参数 | 行为 |
| --- | --- | --- |
| `Root` | `value: Signal<Option<String>>`, `on_value_change: Callback<Option<String>>`, `delay_ms=200`, `skip_delay_ms=300`, `orientation=Horizontal`, `aria_label`, children | `nav` 加名称，`position:relative`，是 List/Item/Content 的唯一相对定位祖先（List/Item 保持 static）；打开期间持有 document capture `pointerdown`/`focusin` 监听（abort 绑定）：目标在 Root 外且不在本 scope 已开 Layer 内时请求 `None` 一次（pointerdown 后同目标 focusin 合并）；关闭/卸载/abort 释放 |
| `List` / `Item(value)` | — | `ul`/`li`；List 内 Trigger 与顶层 Link 用方向键（随 orientation 与 RTL）、Home/End 移动，所有项保持原生 Tab |
| `Trigger` | disabled | 原生 button，`aria-expanded`、开启时 `aria-controls`、`data-state`；click 切换（请求 `Some(key)`/`None`）并取消计时；鼠标 pointerenter：已有项打开或处于跳过窗口时立即请求，否则 `delay_ms` 后请求；离开 Trigger/Content 后 150 ms 请求 `None`，重新进入取消；入口键（横向 ArrowDown，纵向 ArrowRight/RTL ArrowLeft）在打开时聚焦内容首个可聚焦元素 |
| `Content` | `force_mount=false`, children | 内联于 Item，绝对定位到 Root，`z-index` 高于 Viewport；关闭时 hidden+inert；`aria-labelledby` Trigger；Escape 请求 `None` 并回焦 Trigger；内容内方向键在可聚焦元素间移动 |
| `Link` / `link_as(HtmlElement<A,..>)` | `href`, `active: Signal<bool>`, `on_select: Option<Callback<Cancelable<()>>>`, `target`, `rel`, `download` | 原生 `a` + `data-rustify-native-navigation`；`active` → `aria-current=page`、`data-active`；无修饰键主键点击运行 `on_select`，未取消则请求 `None` 一次；修饰键/中键不关闭；导航始终交浏览器 |
| `Indicator` | — | 位于 List 外的轨道元素（`ul` 只含 `li`），offset 相对 Root；活动 Trigger 存在时 `data-state=visible` 并以 CSSOM 写 `--rustify-navigation-menu-indicator-{offset,size}` |
| `Viewport` | — | 位于 List 后且不建立 Content 的包含块；ResizeObservation 读取打开 Content 尺寸写 `--rustify-navigation-menu-viewport-{width,height}`，Root 标 `data-viewport`，Content 绝对定位到 Viewport 位置 |
| `Sub` | `value`, `on_value_change`, `orientation`, children | 嵌在 Content 内的同构子导航；外部关闭由外层 Root 负责，Escape 只关子层 |

计时器使用 `primitives::timer::Timeout`（Drop 时 clearTimeout + `release_on_abort`），在相反输入、关闭、卸载、scope 清理、实例 abort 时取消；不常驻 pointermove 监听。Root 定位、List/Item static、Indicator 轨道与 Content z-index 一并写入 `primitives.css`。

### 5.6 Themes 视觉参数与样式

| 组件/部件 | size / 默认 | variant / 默认 | 其他公开视觉参数 |
| --- | --- | --- | --- |
| DropdownMenu/ContextMenu/Menubar Content 与 SubContent | `MenuSize::{One, Two}`，Two，Responsive | `MenuVariant::{Solid, Soft}`，Solid | `color: Signal<Option<AccentColor>>`、`high_contrast`；SubContent 继承所属 Content 选择 |
| Item / CheckboxItem | 继承 Content | 继承 Content | `color`（只改该项高亮/文字角色）、`shortcut: Option<Shortcut>`（显示 kbd 并输出 aria-keyshortcuts） |
| RadioItem | 继承 | 继承 | `color` |
| Menubar Root、NavigationMenu、Toolbar | 不虚构 | 不虚构 | Rustify 语义外观（border/muted/accent/ring/popover token），class 可覆盖 |

- Themes Content 默认位置：DropdownMenu/Menubar 为 Bottom/Start、`side_offset=4`、`collision_padding=10`；ContextMenu 为 Right/Start、`side_offset=2`、`align_offset=−4×size`、`collision_padding=10`；SubContent `side_offset=1`、`align_offset=−4×size`。Content 内用无 Tab 停靠点的 `overflow:auto` 容器承载项（不设 `role`/`tabindex`，避免模态菜单的 Tab 包裹只落到滚动框），`max-height` 取 `--rustify-floating-available-height`。
- 度量、高亮、分隔、标签、指示器从 vendor 的 `base-menu.css` 转写到 `menus.css` 的低权重 `@layer components`，引用已解析 `--rustify-space-N`/字体/半径/shadow 与 token，不再乘 scaling；`color` 经 `component_color::set_color`，未设时消费 primary 角色；high_contrast 只改项文字/高亮配对。结构规则（hidden/inert、Root 定位/List 与 Item static、Indicator 轨道、Content z-index 与导航内容定位、Indicator/Viewport 变量使用）进 `primitives.css`，raw 消费者只加载它。

### 5.7 构建期静态 HTML 与接管

- 菜单族初始关闭：SSG 输出 Trigger/Menubar/Toolbar 与关闭语义，Portal 不输出浏览器内容；host 请求初始打开记录 `StaticRenderIssues`。Toolbar 首个可用项 `tabindex=0`、其余 −1，SSG 与 hydrate 一致。 便捷 Menu 的应用自有 anchor 在 host 与浏览器均登记存在，关闭态静态根不误报 `missing-anchor`。
- NavigationMenu 的 List/Trigger/顶层 Link 静态输出；`force_mount` 的 Content 以 hidden+inert 输出其链接，关闭内容不可键盘进入。
- hydration 复用节点与 ID；主题来源属性只在浏览器端输出，SSG 不输出，不参与 hydrate 一致性，也不占用组件 ID 序列。

### 5.8 主题来源标记与 Studio 完整陈列

**core。** `ThemeScope` 提供来源 `"root"`；每个 `ThemeBoundary` 在浏览器端用边界自有计数器自取唯一 ID（`boundary-<n>`），不依赖 ThemeScope 的渲染顺序或计数器，在其元素上写 `data-rustify-theme-source`，并经 context 向下提供；Layer 在投影主题快照时同时写创建处的来源；`use_theme_source()` 在边界内返回该 ID，根帧返回 `"root"`，无主题 scope 时返回 `None`。现有 `data-rustify-theme-boundary=""` 保留。

**陈列（Components 场景）。**

- `gallery_catalog.rs`：`pub(crate) struct GalleryEntry { references: &'static [&'static str], section: Section, parts: &'static [&'static str], primitives_path: Option<&'static str> }`，`Section::{Interaction, Overlay, Menu, Navigation, Content, Feedback, Layout, Typography}`；`EXCLUDED: &[(&str, &str /*原因*/)]` 只列非视觉或 Studio 自身项（Slot、Slottable、Direction、Portal、Theme、ThemePanel、ThemeContext 类、各 icon 导出）。host 单测断言 `catalog::STABLE_REFERENCES` 每个名称恰好落入一个 entry 或 EXCLUDED。
- `gallery.rs`：按 Section 分组，每个 entry 一张样本：真实 Themes 组件的可操作实例（应用状态在本地 signal，接受请求）、该组件声明的 variant/size 选择（由各枚举 `ALL` 驱动，默认取组件默认值）、适用时的 disabled 开关；结构说明为部件树文本与原语路径；“显示部件”开关为样本内 `[data-rustify-part]` 加名称轮廓。浮层/菜单从样本触发器真实打开；拾取模式下也放行会打开浮层的激活（§5.8 Inspector 第 1 条），无需退出拾取。已有 FormSamples 的表单族直接复用其组件写法，不复制第二套状态模型。
- 场景名 `Components` 加入 `SCENE_NAMES`、`scene_label`（中英）与 persistence 白名单；GPU 区域在该场景显示本地化说明“组件陈列仅 DOM；GPU 显示通用主题样本”，不把场景键直接拼进标题。

**Inspector。**

1. 监听根增加主预览 scope 的 overlay root；`choose()` 只接受来源帧为根主题或已登记预览边界的目标（Studio 自身 Layer 在编辑壳边界内，因此不被拾取）。拾取开启时放行会打开浮层的用户激活：`[aria-haspopup]` 元素上的主键 pointerdown/click，以及 ContextMenu Trigger 上的次键 pointerdown/`contextmenu`；这些事件不被 `prevent_default`/`stopImmediatePropagation` 消费，浮层打开后继续拾取。Esc 先退出拾取（`stopImmediatePropagation`），不关闭浮层；再次 Esc 才由 core 关闭最上层。提示文案写明“除打开浮层的控件外，预览动作暂停；Esc 退出拾取”。
2. 解析顺序：最近 `data-theme-tokens`（显式标注）→ 最近 `[data-rustify-part]` 部件：遍历 `document.styleSheets` 与 adopted sheets（递归 `@layer`/`@media`/`@supports`/`@container`），对每个顶层 style rule 取去掉伪元素、用户动作伪类（`:hover`、`:active`、`:focus*`、`:checked` 等）与运行态属性条件（`[data-state]`、`[data-highlighted]`、`[data-disabled]`、状态位 `[aria-*]`）后的主体选择器与选中元素及部件元素做 `matches`，命中后从该规则完整 `cssText`（含嵌套变体）收集 `var(--<token>)`，token 限 `COLOR_TOKENS`∪`VALUE_TOKENS`；`matches` 抛错的选择器跳过；结果按“部件名+className+data-variant/data-color”缓存，样式表数量变化时清空 → 都无则“未标注”。该语义与元素当前状态无关，表示“该部件在任何状态下引用的 token”；不包括状态条件本身。
3. 组件局部色板：最近 `[data-color]` 给出色族，规则中的 `--rustify-component-color-N` 给出阶。
4. 来源帧：最近 `[data-rustify-theme-source]`（无属性时为根主题）。Studio 在每个预览内 `ThemeBoundary`（ProviderFixture、陈列的局部主题样本）内用小组件读取 `use_theme_source()` 与 `use_resolved_theme()`，注册到 `StudioState` 的来源表（清理时移除），登记必须带中英标签；未登记边界显示“未登记的局部边界”，不显示原始 ID。Inspector 用该帧解析值与 `ThemeValueOrigin`（含 Boundary configuration/override）显示来源与值；选中部件属于边界时只读显示“来自局部边界〈标签〉· 由该样本控制；编辑器修改根主题”，并并列根主题的值。`TokenSource` 保持根主题语义：路径、解除、跳转只作用于根主题 overrides。元素位于浮层时额外显示“浮层继承自”及该来源显示名（根主题或边界标签）。

**对比诊断上下文。** PreviewDiagnostics 对根与已注册边界分别计算现有 15 组 token 对，按来源分组列出；Inspector 选中部件若同时使用某对 fg/bg token，就显示该帧实际解析值的比值与结论及“定位来源”。仍以解析值计算，不按色阶名推断；作者颜色不改写。

### 5.9 两表达等价（WORK-F1/AC-3）

- 等价集合：同时有 Primitives 与 Themes 表达的交互组件——P2 的 checkbox（含不确定）、checkbox_group、radio_group、switch、toggle、toggle_group、slider（单值/范围）、select、form/field + text_field、textarea、number_field、color_field；P3 的 tabs、accordion、collapsible、dialog、alert_dialog、popover、hover_card、tooltip、toast、scroll_area；P4 的 dropdown_menu、context_menu、menubar、navigation_menu、toolbar。
- P3/P4 族由在 dom-only 与 dom-primitives 两项目以同一断言运行的 panels/content/feedback/menus specs 证明；P2 族新增 `parity.rs`（两侧同 test id、同场景，styled 用 Themes、raw 用 primitives）与 `parity.spec.ts`。每个场景在 `window.__parity.policy(component, "accept"|"reject")` 下执行一次离散动作，断言：应用请求日志条数为 1 且载荷相同、显示值/ARIA 状态等于应用值、焦点落点 test id、开闭结果。M7 记录写“组件 → spec/用例”覆盖表；某 P3 族缺拒绝路径时只在 parity.spec 补该行。

## 6. 前端与交互

遵循 DESIGN.md 与 Studio surface brief：中性编辑壳、左编辑右预览、各区独立滚动、窄屏编辑/预览切换；作者主题只作用于预览及其浮层。

- **消费者样本（menus.html / raw index）**：一个应用外壳——顶部 Menubar（文件/编辑/视图，含勾选、单选、子菜单与快捷键显示），其下 NavigationMenu（含 Viewport、Indicator、active 链接与一个子导航），内容区 Toolbar（按钮、切换组、内嵌 DropdownMenu Trigger、搜索字段），列表行 ContextMenu；结果面板显示最近命令与请求计数（开发者工具折叠在 details，不在首屏铺开）。接受/拒绝/动态移除触发器的控制放在该 details。
- **Studio 陈列**：Components 场景首屏是分组目录与样本，样本卡只包含真实组件、选择器与结构说明，不加装饰插图；状态区分“演示控制”与组件真实状态；浮层可在预览内按真实触发器打开并检查（拾取期间同样放行，见 §5.8）；边界来源显示中英标签，未登记边界不显示原始 ID。
- **状态**：空菜单空态、导航内容加载中、工具栏字段错误属于样本与文档指引，组件不合成文案：空菜单显示可读空态项、禁用项保留可见原因、拒绝请求时显示保持应用值、加载中不移除链接、错误贴近字段。使用者文案不暴露内部层 ID 或请求计数（计数仅在开发者 details）。
- **响应式**：390px 下 Menubar/Toolbar/NavigationMenu 横向局部滚动而非撑破页面，NavigationMenu 不切换 orientation（避免 aria-orientation 与视觉相反）；菜单内容受可用高度限制并内部滚动；Studio 陈列在窄屏单列。
- **可访问性与动效**：菜单/导航/工具栏按 §5 的角色与键盘；焦点可见；开闭动效尊重 prefers-reduced-motion；不以颜色单独表达勾选/当前页。
- 真实浏览器验收见 V-5、V-9—V-11；检查 console、pageerror、CSP、失败请求并留必要截图；不能以 lint/build 替代。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD AC-2；APG menu/menubar/toolbar/disclosure navigation；P3 G-01 scope-local 口径 | 本期全部浮层/导航/反馈的角色、名称、焦点顺序、关闭与宣告正确；默认主题普通文本 ≥4.5:1、必要非文本边界 ≥3:1（逐表面测量，含 P3 延后的表面）；低对比主题诊断有部件/来源上下文 | 不把未测表面或人工 AT 结论记为通过 | §5.1—§5.5、§5.8 | V-9、V-10 |
| WORK-N2 | AGENTS.md 性能规则；P3 panels-performance 口径 | 旧 Menu 同功能 p50/p95 前后对比与 WASM/CSS gzip 差值；新负载单列；具体值未知，无新 SLO | 持续资源增长或重复请求先修 | 每帧合并、按需监听、单一计时器 | V-1、V-14 |
| WORK-N3 | architecture.md 实例/页面监听契约 | 菜单/导航/工具栏关闭或 scope dispose 后监听、计时器、capture、inert 归零；fatal 只终止本实例 | 未释放视为缺陷 | `rustify_ui::listen`、Timeout + release_on_abort | V-11 |
| WORK-N6 | PRD §2.7 | Trigger/SubTrigger/导航项/工具栏当前项移除、内容变化、取消、GPU 局部故障后剩余 DOM 任务可继续，无死遮罩/焦点锁 | 失效对象撤下并按 P3 回焦 | floating 失效路径、Sub 整树关闭、工具栏停靠点重选 | V-11 |
| WORK-N7 | DESIGN.md；PRD AC-2 | 浅/深一致、每个声明变体在其状态下可辨认、加载/空/错误不丢关键操作 | 记缺陷并修，不以截图自检冒充人工评审 | §5.6、§5.8 陈列 | V-10 |
| WORK-N4、N5、N8 | 既有 features/catalog/sources 门 | 无 Makepad、raw 无皮肤、目录与交付一致、来源可取回 | 门失败即不退出 | §5.7、§3、M1 | V-12、V-15 |

静态浏览器 SDK，无 SQL、网络供应商、服务端鉴权或租户；热路径不在 pointermove/scroll 中扫描整页或重复测量兄弟节点，菜单项查询只在 keydown 时对本 panel 执行；Inspector 的样式表扫描按选中目标缓存，拾取与悬停预览共用，每次目标变化只首次计算，不在 mousemove 中重复遍历。无 innerHTML 入口。

## 8. 失败模式、切换与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 应用拒绝 open/value/勾选请求，或取消 `on_select`/钩子 | 对应菜单/导航 | 无擅自写值 | 显示保持应用值；取消选择时菜单保持打开 | 请求日志、ARIA、焦点 | 应用接受后正常投影 | V-5、V-8 |
| Trigger/SubTrigger/导航项/工具栏当前项在打开时被移除 | 该菜单树/导航/工具栏 | 不改应用值 | 子层随父撤下，无遮罩/捕获；焦点按 P3 链或下一可用项 | DOM、stack depth、inert、监听计数 | 有效节点重现且 open 仍 true 后恢复 | V-11 |
| 子菜单打开时点击菜单树外 | 该菜单树 | 无 | 整树关闭一次（根钩子取消时保持打开） | 请求日志 | — | V-5 |
| scope dispose / 实例 trap（菜单、导航打开时） | scope / 实例全部 scope | 临时 UI 结束；持久数据不变 | 其他 scope/实例继续 | stats、host fatal | 正常释放；fatal 沿既有重启 | V-11 |
| Studio GPU 预览上下文丢失或字体 404 | 单个 GPU 视图 | 作者值在应用状态 | DOM 编辑、陈列、保存、导出继续 | gpu-state、DOM 交互 | 既有恢复链路重投影 | V-11 |
| 新源码与旧 CSS/消费者混用 | 使用该产物的页面 | 无 | 参数视觉无效或编译失败 | css/catalog check、build | 源码、CSS、示例一起切换/回退 | V-15 |

提交点先保存可取回的旧菜单基线（M1），再提交阶段一（M4），最后整期（M7）。没有部署、放量或不可逆数据迁移；回归阻断现有入口时回退对应阶段提交及其 CSS/示例。

## 9. 验证

### 9.1 检查定义与时点

所有命令在仓根执行。批内只对具体失败运行一个过滤器（例如 `mbx test --locked -p rustify-components --lib primitives::menu::model::tests::typeahead_cycles_same_letter`）或单个 spec 的 `-g`；整组命令只在本批交付齐备后运行一次。新测试文件与 specs 是本期交付物，先创建后运行；测试名按行为命名，不带需求/里程碑 ID。浏览器命令每次一个 project、串行。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-N2、WORK-N8：旧 Menu 同功能 before 数据与构建版本可取回；正式参考与许可入仓 | `mbx test --locked -p rustify-components --lib menu::tests`；§9.2 baseline 命令；`mbx xtask sources verify` | M1：基准 fixture、spec 与 vendor/lock 齐备后、改动旧 Menu 之前；重构风险门 |
| V-2 | WORK-F13/AC-1、AC-2 原语与 Themes、WORK-F15/AC-4 参数静态输出、便捷 Menu 重组：角色/ARIA/诊断、typeahead/导航/激活请求模型、整树关闭模型（含根钩子取消时保持打开）、ContextMenu 键盘去重模型 | `mbx test --locked -p rustify-components --lib primitives::menu`；`mbx test --locked -p rustify-components --lib menu::`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test menus_ssg --test panels_ssg --test missing_context_ssr`；`mbx clippy --locked -p rustify-components --all-targets -- -D warnings`；同 ssr feature 的 clippy；`cargo fmt -p rustify-components -- --check`；`mbx xtask css` 后 `mbx xtask css --check` | M2：floating 增量、菜单原语、两族 Themes、便捷 Menu、menus_ssg 与两消费者样本齐备后 |
| V-3 | WORK-F13/AC-2（Menubar）、AC-3（Toolbar）、WORK-F15/AC-4 | `mbx test --locked -p rustify-components --lib -- primitives::menubar primitives::toolbar primitives::choice`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test menus_ssg --test ssr_controls`；两种 clippy；fmt；css check | M3：Menubar、Toolbar、toggle group 接管参数、样本与 SSG 用例齐备后 |
| V-4 | WORK-F13/AC-2（NavigationMenu）、WORK-F15/AC-4 | `mbx test --locked -p rustify-components --lib primitives::navigation_menu`；`--test menus_ssg`（ssr）；两种 clippy；fmt；css check | M4：NavigationMenu 原语/皮肤/样本/SSG 齐备后，先于 V-5 |
| V-5 | **阶段一收口**：WORK-F13 全部、WORK-F15/AC-4 在两消费者的真实指针/键盘/触控模拟/RTL；接受/拒绝/取消/动态移除；子菜单宽限与整树关闭；ContextMenu 键盘；Toolbar 不吞字段按键；NavigationMenu 原生链接（修饰键/新标签）；旧 Menu 及 DOM 调用方迁移；floating 改动后的 P3 浮层回归 | §9.4 两个 build-dom；`npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only menus.spec.ts panels.spec.ts`；同配置 `--project=dom-primitives menus.spec.ts panels.spec.ts`；§9.4 的 catalog/workbench/studio 调用方命令；同一次 UI 走查与截图 | M4：V-2—V-4 通过且两消费者与三个示例最新产物齐备后 |
| V-6 | WORK-F19/AC-2、WORK-N1/AC-2（诊断上下文）局部：`theme::` 回归、陈列分类完整、persistence 白名单；来源分配/投影由 V-9 断言 | `mbx test --locked -p rustify-dom --lib theme::`；`mbx test --locked -p rustify-dom --no-default-features --features ssr --lib theme::`；`mbx test --locked -p theme-studio --bins`；`mbx clippy --locked -p rustify-dom -p theme-studio --all-targets -- -D warnings`；`cargo fmt -p rustify-dom -p theme-studio -- --check`；`mbx xtask build-web --example theme-studio --release`（Studio 大部分模块仅 wasm 可编译） | M5：core 标记、gallery/gallery_catalog、Inspector、诊断与 theme-gallery.spec 齐备后 |
| V-7 | WORK-F1/AC-3 fixture 可构建 | §9.4 两个 build-dom；`cargo fmt --manifest-path tests/dom-only/Cargo.toml --all -- --check` | M6：两侧 parity 模块、页面与 parity.spec 齐备后；浏览器执行在 V-8 |
| V-8 | WORK-F1/AC-3 终验；WORK-F15/AC-3 当前回归 | `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only parity.spec.ts panels.spec.ts content.spec.ts feedback.spec.ts menus.spec.ts dom-only.spec.ts`；同配置 `--project=dom-primitives parity.spec.ts panels.spec.ts content.spec.ts feedback.spec.ts menus.spec.ts primitives.spec.ts`；两条命令顺序运行，使用同一构建产物与代码基线（AGENTS.md 每命令一项目）；覆盖表写入 M7 记录 | M7：最终两消费者产物后 |
| V-9 | WORK-F19/AC-2（来源分配/投影、边界标签、Components GPU 说明）、WORK-N1/AC-2（低对比诊断） | `mbx xtask build-web --example theme-studio --release` 后 `npx --no-install playwright test --project=theme-studio theme-gallery.spec.ts theme-editor.spec.ts theme-ui-review.spec.ts theme-dom.spec.ts theme-scenes.spec.ts` | M7：与 V-11/V-13 的 Studio 用例合并为 §9.4 一条命令 |
| V-10 | WORK-N1/AC-2、WORK-N7/AC-2、WORK-F15/AC-4：浮层/导航/反馈焦点顺序、局部模态（无 aria-modal、scope 内 inert/Tab）、关闭与宣告；逐表面默认浅/深对比度；陈列声明变体×适用状态按 §9.3 组合规则；加载/空/错误保留关键操作；桌面为验收面，390/768 只验 §6 布局 | `theme-gallery.spec.ts -g matrix`（随 V-9 命令执行）；menus.spec a11y 断言随 V-8 的完整 menus.spec（两项目）；panels/feedback 的局部模态与宣告用例随 V-8；按 §9.3 组合规则截图 | M7 |
| V-11 | WORK-N3/AC-3、WORK-N6/AC-2：菜单/导航/工具栏生命周期失效、dispose/trap 后资源归零与其他实例继续；Studio GPU Lost、字体 404、fatal 重启 | dom-only：V-8 的合并命令已含 menus.spec.ts 与 dom-only.spec.ts（生命周期与两实例）；Studio：`theme-acceptance.spec.ts -g "isolated failures"`（三个用例）与 `theme-gpu.spec.ts`（并入 §9.4 Studio 命令） | M7 |
| V-12 | WORK-N4 增量与 WORK-F1/AC-2 回归：同步 SSG、唯一 ID、hydrate 原节点、raw 无皮肤、无 Makepad；Docs 组件与站点 demo 的便捷 Menu | `mbx test --locked -p rustify-components --no-default-features --features ssr --test menus_ssg --test panels_ssg --test content_ssg --test feedback_ssg --test ssr_controls --test missing_context_ssr`；`mbx xtask docs-verify --group features`；`-c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts`；`--project=components-subpath islands.spec.ts custom.spec.ts`；`--project=official-subpath official.spec.ts` | M7：最终 API/CSS/fixture 后；features 先于 hydrate 浏览器 |
| V-13 | 整条终验回归：WORK-F1/AC-1、WORK-F15/AC-1—AC-2、WORK-N3/AC-1—AC-2、WORK-N6/AC-1 | `-c playwright.docs.config.ts --project=colors-static colors.spec.ts`；`mbx xtask build-web --example component-catalog --release` 后 `npx --no-install playwright test --project=component-catalog dom-buttons.spec.ts dom-layout.spec.ts theme-compat.spec.ts p2-semantics.spec.ts catalog-inputs.spec.ts`；Studio `theme-forms.spec.ts theme-exchange.spec.ts theme-storage.spec.ts theme-v2-storage.spec.ts`（并入 §9.4 Studio 命令） | M7；与 V-5 已通过且未失效部分引用 |
| V-14 | WORK-N2 增量：旧 Menu 同负载 after 与 new-only 负载、压缩体积、资源归零 | §9.2 current 命令 | M7：最终产物与行为回归通过后一次 |
| V-15 | 静态/host 全量、目录/来源/CSS、文档；Agent 角色模拟 | §9.5 命令；§9.3 角色模拟 | M7 |

### 9.2 菜单热路径基线与成本

M1 在旧组件上新建 ★ `tests/dom-only/consumer/src/menu_benchmark.rs`、★ `tests/dom-only/consumer/public/menus.html` 与 `menus.js`、★ `tests/dom-only/menus-performance.spec.ts`，沿用 `panels-performance.spec.ts` 口径：1440×900、5 次预热、40 样本、页内原生事件 capture → 接受状态 DOM 到达下一帧，p50/p95。同功能负载：便捷 Menu 键盘打开到首项聚焦、ArrowDown 移动、Enter 激活到关闭并交付；拒绝策略预检请求计数。`MENU_BENCH_PHASE` 只接受 baseline/current，输出 ★ `docs/validation/dom-components-p4/{baseline,current}.json`，记录代码版本、浏览器、OS/viewport/DPR、工具链、样本与负载参数；current 要求参数与 baseline 相等。current 另列 new-only：子菜单打开、Menubar 切换、NavigationMenu 打开、Toolbar roving；记录 WASM/全部 CSS 原始与 gzip 大小，循环后 tasks/timers/observers/listeners/nodes 无增长。

```sh
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml
MENU_BENCH_PHASE=baseline RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only menus-performance.spec.ts
# M7
MENU_BENCH_PHASE=current RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only menus-performance.spec.ts
```

无新毫秒上限；退出要求行为通过、请求/接受计数正确、无持续积压，并对每个可测回退说明来源。测量耗时未知；完整 Studio/目录项目历史上可达数十分钟，只跑列出的 spec。

### 9.3 场景、视觉矩阵与角色模拟

- **菜单**：嵌套两级子菜单的键盘进入/退回、Escape 逐层、整树外部关闭、宽限区斜向移动不误关、选择子菜单项只交付一次命令与一次根关闭；CheckboxItem 三态、RadioItem 同值不请求；typeahead 同字母循环与空格；应用拒绝打开/关闭/勾选；取消 `on_select` 保持打开；ContextMenu 右键、Shift+F10、ContextMenu 键（键盘去重）、长按（Pointer Events 模拟）。
- **Menubar/导航/工具栏**：顶层左右切换与悬停切换、Escape 回焦；NavigationMenu 悬停延迟/跳过、Viewport/Indicator 变量、内容 Tab 顺序、修饰键点击不关闭、active 的 aria-current；Toolbar 中搜索字段内方向/Home/End 保持编辑、切换组请求、内嵌菜单 Trigger 打开后再回到工具栏。
- **生命周期/隔离**：打开中移除 Trigger/SubTrigger/导航项/当前工具栏项；两 scope 与嵌套反向主题共存；scope dispose 与 trap 时菜单与导航打开；断言监听、计时器、capture、inert 恢复，另一实例可点击可键盘操作。
- **视觉矩阵**：全部陈列组件在桌面、默认色板、浅/深两模式下，各声明 variant/size `ALL` 与适用 rest/hover/focus/open 或 checked/disabled、loading/invalid/empty 至少各一个状态，不要求全组合。局部色板、high_contrast、调用方 utility 覆盖只对菜单、导航、工具栏这样参数已落地的组件及目录中声明 high_contrast 的组件各取一例。390/768 只验 §6 应用外壳、菜单族与陈列布局，不计入 WORK-N7/AC-2（AC-3 归 P5）。断言：同状态不同变体的计算视觉签名（背景、边框、文字色、阴影、outline 的 style/width/color 与 opacity，以及参考变体定义的尺寸/间距）不同；交互状态相对 rest 有变化；文字与有效背景（合成 alpha 后）≥4.5:1、必要非文本边界 ≥3:1，适用例外（禁用、装饰）在记录列明；D11 确认的 Radix 参考配色组合按原规则保留，逐项记录来源、实测对比度及上下文诊断，不计作达标。回归层跑代表性子集，完整强度只在 evidence 层；截图供人工复核，不以自检冒充 UI 评审。
- **Studio**：陈列覆盖与结构说明；Inspector 选中菜单项/导航链接/Toolbar 按钮/浮层内容时显示 token、色族/阶、来源帧与“浮层继承自”，浮层经 §5.8 的拾取路径打开；边界显示中英标签、未登记边界不显示原始 ID；局部低对比边界的诊断按来源分组并可定位；Components 场景 GPU 区域显示本地化“组件陈列仅 DOM”说明；GPU Lost 与字体 404 时陈列、编辑、保存、导出继续；fatal 后重启恢复最近持久草稿。
- **角色模拟**：Agent 模拟应用开发者，只读 README、docs/menus.md、迁移指南与公开示例/API，使用当前 source SDK/CSS，不读私有模块或实现者补充；在独立公开消费者 `tests/dom-only/public-panel-consumer/` 中完成“顶部菜单栏 + 站点导航 + 带菜单的工具栏 + 右键菜单”的应用外壳并在真实浏览器操作；构建 `mbx xtask build-dom --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml`，用其 `playwright.config.ts` 的 `public-panel` project 进入/操作；`tests/dom-only/consumer` 与 `tests/dom-only/primitives` 属测试 fixture，不得作为答案来源。记录代码位置、命令与缺失步骤；失败先补公开文档或接口再以同一角色继续。结果不代表真人上手率。
- VoiceOver 实际朗读、OS IME、物理触控、真实浏览器 UI 缩放列 P5 missing；本期无需真人。

### 9.4 构建与真实入口

```sh
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml
mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml
mbx xtask build-dom --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml
```

`playwright.docs.config.ts` 的 dom-primitives testMatch 增 `menus.spec.ts`、`parity.spec.ts`；dom-only 由 glob 覆盖，`menus-performance.spec.ts` 加入 dom-only 的 tier 限制同 panels-performance（evidence）。`dom-only`/`dom-primitives`/`dom-hydrate`/`colors-static`/`components-*`/`official-*` 只在 `playwright.docs.config.ts` 定义，这些命令一律带 `-c playwright.docs.config.ts`；根配置无同名 project。阶段一调用方（V-5）：

```sh
mbx xtask build-web --example component-catalog --release
mbx xtask build-web --example property-workbench --release
mbx xtask build-web --example theme-studio --release
npx --no-install playwright test --project=component-catalog p2-semantics.spec.ts catalog-inputs.spec.ts
npx --no-install playwright test --project=property-workbench p2-workspace.spec.ts
npx --no-install playwright test --project=theme-studio theme-scenes.spec.ts theme-dom.spec.ts theme-review.spec.ts
```

M7 的 Studio 合并命令（V-9、V-10、V-11、V-13 共用一次服务）：

```sh
mbx xtask build-web --example theme-studio --release
npx --no-install playwright test --project=theme-studio theme-gallery.spec.ts theme-editor.spec.ts theme-ui-review.spec.ts theme-dom.spec.ts theme-scenes.spec.ts theme-gpu.spec.ts theme-review.spec.ts theme-forms.spec.ts theme-exchange.spec.ts theme-storage.spec.ts theme-v2-storage.spec.ts
npx --no-install playwright test --project=theme-studio theme-acceptance.spec.ts -g "isolated failures"
npx --no-install playwright test --project=property-workbench p2-workspace.spec.ts
```

V-5 已通过且相关代码未变的调用方 spec 在 M7 直接引用。M5 改动 `theme-studio` app.rs、overlay.rs、provider.rs 与 storage_controls 后，V-5 的 theme-review 及 property-workbench p2-workspace 证据失效，在 M7 重跑；其余按证据适用性引用。收口回归后执行 `git checkout -- docs/validation/dom-components-p1 docs/validation/dom-components-p2 docs/validation/dom-components-p3` 还原历史截图，P4 证据只写 `docs/validation/dom-components-p4/`，以 git diff 核实无残留。serve 不重建，源码改动后先重建对应产物。工具/浏览器缺失按既有环境排查并求助，记录“未在浏览器中验证”，不擅自安装。

### 9.5 最终静态与 host 回归

与 `.github/workflows/verify.yml` 相关入口一致，在 M7 一次完成；已通过且本阶段未改的局部检查直接引用。

```sh
cargo fmt --all -- --check
cargo fmt --manifest-path tests/dom-only/Cargo.toml --all -- --check
cargo fmt --manifest-path tests/dom-only/hydration/Cargo.toml --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-dom --no-default-features --features ssr --lib
mbx test --locked -p rustify-components --no-default-features --features ssr --lib
mbx test --locked -p rustify-ui --lib
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib
mbx clippy --locked -p rustify-dom -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings
mbx test --locked -p rustify-docs
mbx clippy --locked -p rustify-docs --all-targets -- -D warnings
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
git diff --check
```

CI：`verify.yml` 的 SSG `--test` 列表加 `menus_ssg`，dom-primitives 文件列表加 `menus.spec.ts parity.spec.ts`；Studio 由 `theme-*.spec.ts` glob 覆盖 theme-gallery。不新增流水线。

## 10. 里程碑与验收安排

阶段一（M1—M4）交付菜单/导航/工具栏并在 M4 收口 WORK-F13 与本期视觉参数；阶段二（M5—M7）交付 Studio 陈列与等价 harness，M7 收口全部 10 条 AC 与四条整条终验。默认每个实现里程碑一批。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 旧菜单同功能基线与正式参考输入 | C-1；旧 Menu 未改 | ★ menu_benchmark.rs、menus.html/js、menus-performance.spec.ts，冻结 before；vendor ★ themes-menus/primitives-menus 与 sources.lock；按 §5.6 确认字段映射。主 agent 负责共享接线与基线提交 | V-1 通过，before JSON 与构建/源码版本可取回；回写「实施进度」 |
| M2 | 共享菜单原语、DropdownMenu/ContextMenu 与旧 Menu 重组 | M1/V-1；§5.1—§5.2 接口 | §5.1 增量；★ primitives/menu(+model/browser)、★ timer.rs、★ dropdown_menu/context_menu（原语与 Themes）、★ menus.css 与结构规则、styles/sdk/docs 注册；便捷 Menu 重组；`choice::navigation` 改 pub(crate)；目录下拉/右键演示与 Studio 场景菜单迁移；★ menus_ssg 与两消费者 menus 样本（同 test id）。对照 popover.rs/badge.rs/select.rs | V-2 通过（含先红后绿的激活/整树关闭/键盘去重模型用例）；回写「实施进度」 |
| M3 | Menubar 与 Toolbar | M2 的共享菜单部件与 V-2 | ★ menubar（原语/Themes）、★ toolbar（原语/Themes、`toolbar::item`）、choice.rs toggle group 接管参数；两消费者样本与 SSG 用例。Menubar/Toolbar 外观按 AGENTS.md 用 impeccable，依据 DESIGN.md 与 surface brief 定版式（@implementation_route 的明确例外） | V-3 通过；回写「实施进度」 |
| M4 | NavigationMenu 与阶段一收口 | T-1 的 timer.rs 及 V-1；收口需 V-2、V-3 | ★ navigation_menu（原语/browser/Themes）、Viewport/Indicator 变量与结构 CSS；两消费者样本；★ menus.spec.ts 全部描述块（含 lifecycle/a11y 标签）；hydration DeliveryParts；playwright 配置；统一构建与 V-5 走查。NavigationMenu 外观同 M3 用 impeccable。主 agent 串行浏览器 | V-4、V-5 通过；只宣称阶段一（WORK-F13、WORK-F15/AC-4）出口；回写「实施进度」 |
| M5 | Studio 完整陈列、部件级 Inspector 与诊断上下文 | M2—M4 的组件（陈列样本）；core 来源标记在本批内先完成 | provider.rs/overlay.rs 来源标记与 `use_theme_source`；★ gallery.rs、★ gallery_catalog.rs、scenes/app/persistence/inspector/controller/storage_controls/theme_controls/provider_fixture；★ theme-gallery.spec.ts；README。Components 场景信息架构与 Inspector 信息结构按 AGENTS.md 用 impeccable（依据 DESIGN.md 与 surface brief；同 M3 的 @implementation_route 明确例外），信息结构变化后同步 DESIGN.md 对应小节；对照 form_samples.rs、form_states.rs、inspector.rs 现有拾取 | V-6 通过；回写「实施进度」 |
| M6 | 两表达等价 harness | P2/P3 现有 fixture 与 C-1（M2—M4 组件不构成前置；等价只覆盖 P2 表单/选择族） | ★ consumer/primitives `parity.rs`、★ parity.html/js、★ parity.spec.ts；配置与 CI 列表 | V-7 通过；回写「实施进度」 |
| M7 | 全部 AC 与整条终验收口 | V-1—V-7 通过；M4 的 V-5 阶段出口 | 文档（★ docs/menus.md、迁移、架构、主题、README）、catalog 与生成物、CI；按 §9 依次运行 V-8—V-15，重跑 M5 失效的 theme-review 与 property-workbench p2-workspace，修复范围内验收缺陷；整理“组件→等价证据”“AC→证据”表与人工 missing | V-8—V-15 通过；10 条 AC 与 WORK-F1/F15/N3/N6 整条逐条可追溯；回写「实施进度」 |

提交点：**M1（旧菜单基线可取回）、M4（阶段一收口）、M7（全部里程碑完成后的最终提交）**。M2/M3 在 M4 前完成写待提交，M5/M6 在 M7 前完成写待提交；只暂存本计划相关改动。

### 10.1 内部任务

主 agent 独占 lib.rs、primitives/mod.rs、sdk.css/styles.rs、catalog.rs、rustify-docs style.rs、playwright 配置/CI、计划与台账；浏览器、端口与共享编译缓存串行。

| 任务 ID | 所属里程碑 | 交付物/影响路径 | 前置成果 | 检查 ID |
| --- | --- | --- | --- | --- |
| T-1 | M2 | floating 增量 + `primitives::menu`（model/browser）+ `timer.rs` | V-1 | V-2 |
| T-2 | M2 | dropdown_menu/context_menu 原语与 Themes、menus.css、便捷 Menu 与调用方迁移、样本 | T-1 的部件签名与 model 测试通过 | V-2 |
| T-3 | M3 | Menubar | T-1、T-2 的 V-2 | V-3 |
| T-4 | M3 | Toolbar 与 toggle group 接管参数 | V-1；嵌入菜单场景需 T-2 | V-3 |
| T-5 | M4 | NavigationMenu | V-1、T-1 的 timer | V-4 |
| T-6 | M5 | core 来源标记 | 无 | V-6 |
| T-7 | M5 | gallery/gallery_catalog、Inspector、诊断 | T-6；陈列菜单样本需 M2—M4 | V-6 |
| T-8 | M6 | parity fixture 与 spec | P2/P3 现有 fixture 与 C-1；不依赖 M2—M4 | V-7 |

T-4、T-5、T-6 顺序完成；并行的只有 M5 ∥ M6，且按 §4 的写入边界与独立 worktree/串行编译约束。T-6 改上游 rustify-dom，会触发全部重编，不与其他任务并行。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 便捷 Menu 新增外部关闭与 DOM 结构变化，GPU 锚定调用方用例可能依赖旧行为 | property-workbench/Studio GPU dropdown/catalog region-chooser、docs/site demo 用例；canvas 内点击仍由应用经 GPU action 决定 | V-5/V-12 复跑；若用例断言旧“不关闭”，按新契约改调用方或用例并记录，不在 SDK 加兼容开关 | M4 / M7 | 否 |
| floating 改动波及 P3 浮层 | Dialog/Popover 等 haspopup、锚点 | 既有 kind 行为不变；V-2 的 panels_ssg 与 V-5 的 panels.spec（两个 dom 项目）复跑；V-8 引用 | M4 / M7 | 否 |
| Escape 逐层与 Tab 消费偏离不同参考 | 使用者预期 | D3/D4 已定，docs/menus.md 写明差异；评审可改但不阻塞实施 | M7 文档 | 否 |
| `theme-acceptance.spec.ts` V6 两处断言疑似过期（期望“1.000:1 · Below text thresholds”与标题“GPU 主题样本”，当前源码为 “Fails text thresholds”、无该标题；未运行确认） | 若整文件运行会失败 | 本期以 `-g "isolated failures"` 只运行所需用例；在 M7 记录中报告该既有问题，由用户决定另行修复 | M7 | 否 |
| 菜单/导航同功能前后耗时与体积未知 | 回退解释 | M1 基线、M7 同条件 current | M1 / M7 | 否 |
| 人工 AT、OS IME、真机触控、真实 UI 缩放缺现场证据 | 不能下人工结论 | 列 P5 missing；Agent 可执行的真实检查照做 | M7 | 否 |

- 待用户拍板：无。D3/D4/D5/D7 与“陈列以部件轮廓代替 Primitives 重渲染”是可逆的设计选择，已写入正文与迁移说明。
- 最终状态：**Ready**。
- 定级理由：10 条 P4 AC 与四条整条终验均有设计落点与检查；公共接口、默认值、所有权、失败语义、迁移与分层验收已定；全部开发对象在本仓授权范围，无外部依赖或未决的范围/安全/数据/契约问题。Ready 不表示实现完成或评审通过。

## 12. 已知坑与历史教训

- 子 Layer 的 DOM 在父 panel 的 children host 内：父菜单 keydown/typeahead/项查询必须按 `closest('[role=menu]')` 排除子 panel，否则子菜单按键被父层重复处理（overlay.rs:997-1008、2083-2087）。
- 菜单 Trigger 不复用 `button_as`：它无条件挂 click 切换、固定 `aria-haspopup="dialog"`、无条件输出 `aria-controls`，直接复用会先开后关并让 aria 失效（floating.rs:244-252）。
- 菜单 Content 不用 P3 `scroll_area` viewport：其 `role=group tabindex=0` 会成为模态菜单唯一 Tab 停靠点，core 的 `keep_tab_inside` 在 scope capture 先于 Content 运行（scroll_area.rs:216；overlay.rs:1114-1135、1626-1663）。
- core 只让最上层处理 Escape 与外部交互，且父层撤下时子层不通知应用；Sub 必须自行请求关闭，否则下次打开根菜单时子菜单立刻重开（overlay.rs:1177-1186；hierarchy.rs:64-88）。
- 缺 Root/重复部件不得 `.expect()`；保留 children 与 `data-rustify-issue`，注册随卸载恢复（P2/P3 代码复审；floating.rs 的 missing-root 分支）。
- `aria-controls` 只在开启时输出（P3 修复 `3d078e29` 与 hydration 断言修复 `f2bb1bb1`）；菜单 SubTrigger/导航 Trigger 同规则，SSG 与 hydrate 断言同步。
- 页面监听与计时器必须能在 abort 时由 JS 释放；只靠 Drop 不够（architecture.md「Page-level listeners」；select/browser.rs SearchTimer）。
- 新 scene 名必须加入 persistence 白名单，否则旧/新偏好 envelope 被整体拒绝（persistence.rs:133-163）。
- CATALOG 与 hydration/islands 断言固定 24 个 Category；新组件只进 COMPONENTS/STABLE_REFERENCES/SUB_CAPABILITIES（catalog.rs:165、576；hydration.spec.ts:87、501；islands.spec.ts:755）。
- 新 dom-primitives spec 需同时登记 `playwright.docs.config.ts` testMatch 与 verify.yml 文件列表；新 SSG 测试需加入 verify.yml `--test` 列表（`forms_ssr.rs`、`theme_layout.rs` 的漏接是先例）。
- class 改动后先 `mbx xtask css` 再 check 并提交 rustify.css/candidates.json；新皮肤要在 styles.rs、sdk.css 与 rustify-docs style.rs 三处注册（AGENTS.md；styles.rs）。
- 浏览器每命令一个 project 且串行；serve 不重建；性能只在页内测（AGENTS.md「浏览器测试」）。
- `ThemeScope`/`ThemeBoundary` 只在 wasm 目标存在，SSG 走 `provide_static_theme`；来源 ID 不能依赖 ThemeScope 渲染计数器，也不参与 hydrate 一致性（provider.rs:100、145-200；theme.rs:34-50）。
- `ToggleGroup` 的 `toolbar` 既已输出 `role=toolbar`，嵌入 Toolbar 时必须 `toolbar=false`，否则 toolbar 套 toolbar（choice.rs:849、1642）。
- mbx 管理的 `target/` 可能被 gc 清理，需长期保留的证据放 `docs/validation/dom-components-p4/`，浏览器长跑前确认产物仍在。
- 现有 `menus.spec.ts`/`panels.spec.ts` 等会把截图写入已入库的 P1—P3 验证目录；P4 收口后按 §9.4 还原，P4 证据只写 `docs/validation/dom-components-p4/`（panels.spec.ts:49、243、304、400-404、454、592-598、642）。
- `rustify.css` 是 Tailwind 嵌套产物（`:hover`、`@media`、`@container`，含 39 处 `&`），Inspector 不能只对顶层选择器 `matches`；语义与元素当前状态无关，嵌套变体只用于收集 token（rustify.css:590-615）。
- 内部 ssr 命名只服务 SSG 构建期；不追加流式/请求时 SSR 验收（AGENTS.md「仅 CSR/SSG」）。
- 里程碑记录只存行为、有效证据与未完项；按仓内 SDLC v3 要求同步最小事件与当前页，不复制验证报告（.xgent-ai/sdlc/protocol.md）。
