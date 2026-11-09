# DOM-COMPONENTS-P5 · Rustify UI · 原应用迁移与整体交付（扩展组件同源主题/状态 · 迁移事实源 · Studio 与官方站收窄终验）

```formal
INTENT:
  GOAL: "直接依据本计划、仓库规则和当前代码实施或续做"
  SUCCESS: "全部里程碑退出检查通过，按‘实施进度’完成最终提交、SHA 回填与自查，并提示用户发起代码评审"

RULE @implementation_entry:
  MUST: "实施须知：先读下方‘实施者定位’‘实施进度’，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；goal 执行也遵循这些约定"
```

> **计划状态：Ready**
>
> 调查基线：2026-11-04 · `3735889c89319be11b0d25857f2a2730d96c6e97` · main / dirty；产品源码与测试无改动，未提交内容为 SDLC 台账、`skills-lock.json` 及 P3/P4 评审/覆盖报告，事实来自当前工作树。
>
> 需求来源：[DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) v0.2 · **P5（第 5 期）**，§7.2 的 WORK-F19/AC-3、WORK-F22/AC-1—AC-2、WORK-F23/AC-2—AC-3、WORK-N1/AC-3、WORK-N2/AC-2、WORK-N4/AC-2—AC-3、WORK-N5/AC-2、WORK-N7/AC-3，共 11 条 AC；这 8 条需求的整条终验期都在 P5（§7.3）。验收范围与一条 AC 子句按用户 2026-11-04 决定调整（C-6、C-7、C-8），未验证部分写入 §9.1 的豁免表，不记通过。
>
> 本期交付：**Rustify 扩展组件迁入统一主题角色、状态标识、焦点、空/错误/禁用语义与本地化默认文案；Theme Studio 陈列并可检查全部扩展组件；旧公共入口逐项去向进入目录事实源并重整迁移指南；以真实 Chrome 200%、减少动效、Theme Studio 与官方站 components 页面完成收口。**
> **开发范围：DOM SDK（rustify-dom 的 i18n 消息、rustify-components 的扩展组件、目录与样式产物）、Theme Studio 陈列与测试、官方站 spec、迁移与接入文档。依据见 §0。**
> 本期独特职责：P2—P4 交付新组件，本期负责让旧扩展能力、旧入口和整体交付说明与新体系一致，并把未验证范围如实列出。
> **顶层排除：旧主题路径（ThemedScope 等）的迁移或删除；property-workbench、data-workbench、fusion-basic、vellum、component-catalog、DOM-only 与独立主题消费者的浏览器回归；扩展组件前后性能对比；RTL 扩展；registry/官网发布与版本变更。**

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

- 最近更新：2026-11-09T14:20:00+08:00
- 当前进度：4/4 个里程碑完成
- 当前状态：全部里程碑完成，待代码评审
- 最近完成：M4 · 全部 AC 与整条终验收口，V-5—V-13 全过（V-12 三项 missing，按 AC 允许列出；[M4 记录](DOM-COMPONENTS-P5.records/M4.md)）
- 下一步：用户以 `review-code docs/plan/DOM-COMPONENTS-P5.md` 发起代码评审；豁免与 missing 见 [M4 记录](DOM-COMPONENTS-P5.records/M4.md)「AC → 证据 → 豁免」
- 当前阻塞：无
- 代码基线：M1—M3 在 `500022a5`；M4 见完成记录

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-05T01:17:05+08:00 | 扩展组件统一契约：主题角色/焦点、part 与状态标识、空/错误语义、语义修正、默认文案、Tree 图标、Glyph，V-1 七条全过 | [M1 记录](DOM-COMPONENTS-P5.records/M1.md) | 500022a5 |
| M2 | 已完成 | 2026-11-05T03:55:53+08:00 | Studio Extensions 节（49 组/9 节）、六类样本、局部主题、本地化与 SDK locale 桥、axe=0、矩阵/资源/拾取扩展；V-2 三条全过（spec 15/15）；PanelTabs close 按 axe 实测就地表内修订 | [M2 记录](DOM-COMPONENTS-P5.records/M2.md) | 500022a5 |
| M3 | 已完成 | 2026-11-05T04:30:18+08:00 | 迁移事实源：LEGACY_ENTRIES 152 行+冻结基线 675 键+inventory 完整性测试、目录修正（Primitives.Label/DirectionProvider/Icon/字段）、components.md 再生成含 Legacy API destinations 节、指南重整（How to migrate/生成节替换/Rustify extensions/Default palette/D13/=0.2.1）；V-3、V-4 全过 | [M3 记录](DOM-COMPONENTS-P5.records/M3.md) | 500022a5 |
| M4 | 已完成 | 2026-11-09T14:20:00+08:00 | 基线产物、真实 Chrome 200%/减少动效用例、V6 断言修正；收口修复 PanelTabs 闲置参数、candidates.json、Splitter 样本 CSP 与 4 组过期 Studio 用例；V-5—V-13 全过，AC→证据→豁免表与 V-12 missing 已列 | [M4 记录](DOM-COMPONENTS-P5.records/M4.md) | 待提交 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

功能 AC 来源均为 [PRD §3、§7.2](../PRD-DOM-COMPONENTS.md)，不复制验收原文。“整条终验”表示本期负责结算该需求全部 AC；前期 AC 只在 Theme Studio 与官方站范围重跑，其余引用当期证据并按 §9.1 写明范围。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F22 | 功能 | PRD / AC-1—AC-2 / P5 | DataTable、Tree、FilePicker、DropZone、Splitter、PanelTabs、CommandPalette、Icon/Glyph 保留入口与任务能力；统一主题角色、局部覆盖有效；空、错误、禁用可辨认；大数据机制不丢失 | §5.1—§5.7、§6；V-1、V-2、V-7 | 已确认；前后性能比较按 C-7 豁免 |
| WORK-F19 | 跨期功能 | PRD / AC-3 / P5；AC-1 P2、AC-2 P4 | 扩展组件与保留场景在新主题下可检查；GPU 视图只报告实际支持；GPU 局部故障与实例 fatal 按既定边界 | §6；V-2、V-7、V-9 | 本期验 AC-3，整条终验 |
| WORK-F23 | 跨期功能 | PRD / AC-2—AC-3 / P5；AC-1 P1 | 旧入口/参数/事件/Button 变体/颜色语义/主题文件/CSS 用法逐项去向；消费者按说明完成原有任务；类型、状态表与示例一致 | §3、§5.8；V-3、V-4、V-6、V-13 | 已确认；AC-2 子句按 C-6 调整，AC-3 按 C-7 收窄 |
| WORK-N1 | 可访问性 | PRD / AC-3 / P5；AC-1 P2、AC-2 P4 | 放大、减少动效、辅助技术下关键任务可完成；人工缺失明确列未验证 | §5.2、§6、§7；V-2、V-10、V-12 | 本期验 AC-3，整条终验 |
| WORK-N2 | 性能 | PRD / AC-2 / P5；AC-1 P1；PRD Q4 | 受影响热路径同环境前后 p50/p95、产物压缩尺寸与资源差值；旧功能与新能力分列；无持续积压或资源残留 | §7；V-2、V-11 | 本期验 AC-2，整条终验；扩展前后对比按 C-7 豁免 |
| WORK-N4 | 兼容 | PRD / AC-2—AC-3 / P5；AC-1 P2 | 根/子路径静态消费、文档演示、主题 JSON/CSS、既有 GPU 共享主题继续成立；最低版本写明；资产外置、不增服务/Node/工具链 major | §3 第 6 条、§7；V-6、V-7、V-8、V-13 | 本期验两条，整条终验；消费者浏览器部分按 C-7 豁免 |
| WORK-N5 | 可维护性 | PRD / AC-2 / P5；AC-1 P1 | 稳定参考与旧导出逐项去向；声明的尺寸、变体、交互、渲染模式可对应真实结果，差异可查 | §3、§5.8；V-3、V-9、V-13 | 本期验 AC-2，整条终验 |
| WORK-N7 | 视觉/响应式 | PRD / AC-3 / P5；AC-1 P1、AC-2 P4 | 390/768/桌面与 200% 下主题编辑、导航、浮层、导出主路径可达；数据区域局部滚动；语言切换不改作者数据 | §6；V-2、V-7、V-8、V-10 | 本期验 AC-3，整条终验 |
| WORK-F4 | 前置契约 | PRD / P1；theme/provider.rs | 局部主题边界与浮层继承创建处有效主题 | §5.1、§6；V-2 | 已有契约，本期只验证扩展组件 |
| WORK-F5 | 前置契约 | PRD / P2 §2.6；§2.3「Rustify 扩展」行 | 稳定状态标识；扩展迁入统一状态契约 | §5.2；V-1 | 本期补扩展组件 |
| C-1 | 前置 | DOM-COMPONENTS-P4 恢复快照 7/7、代码复审 R3 通过 | 当前 source SDK/CSS 配对；不以 registry 0.2.1 代表源码 API | §1、§4 | 已核实足够 |
| C-2 | 运行时 | AGENTS.md「运行时契约」；docs/architecture.md | 应用持有值；页面监听与 abort；CommandPalette 继续走 Layer | §5 | 已确认 |
| C-3 | 工具/环境 | AGENTS.md「工具链」「浏览器测试」；mise.toml | mbx 编译；固定 Rust/Tailwind；Playwright 每命令一个 project、串行；不装服务、不升 major、不开分支 | §9、§10 | 已确认 |
| C-4 | 样式 | AGENTS.md「编码与样式约定」；sdk.css、controls.css | 无前缀 utilities + tw_merge；改 class 后 `mbx xtask css` 并提交 rustify.css；无全局 reset；具名元素沿 `--motion-duration` | §5.1、§5.2 | 已确认 |
| C-5 | 交付边界 | PRD §1.3、§7；AGENTS.md「项目定位」「仅 CSR/SSG」 | 不新增 GPU 组件、不发行、不部署；不追加请求时 SSR | §0.5 | 已确认 |
| C-6 | 用户决定 | 用户 2026-11-04 确认：“旧主题不改了” | ThemedScope、ThemeOverride、ThemePatch、use_theme、use_theme_values 不迁移、不删除，三个示例继续使用；不设删除期限 | D9、§3 第 5 条；V-3 | 已确认；WORK-F23/AC-2“不保留无边界期限的平行实现”对该路径不成立，结论须写明 |
| C-7 | 用户决定 | 用户 2026-11-04 确认：“只保证 theme-studio 和 github pages 里的 components 正常”，适用于整个 P5 的浏览器验收 | 浏览器验收只跑 Theme Studio 与官方站；其余示例与消费者只保证编译与 host 检查 | D10、§9.1；V-6、V-7、V-8 | 已确认 |
| C-8 | 用户决定 | 用户 2026-11-04 确认：本地按 Pages 方式构建验证 | 官方站以 official-root/subpath/meta 验证，不部署、不发行，线上 Pages 保持原版本 | D11；V-8 | 已确认 |

无支撑设计的未证实重大假设。VoiceOver 的自动化可行性未知，但 AC 允许列缺失项，不影响设计（§11）。

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 形态 | 留在 rustify-dom / rustify-components；不增 crate、feature 或 Primitives 命名空间 | 扩展组件仍是带样式组件；所有接口变化都是增量；`Message`、`Glyph` 增枚举项是源码级破坏，写入迁移指南 | WORK-F22“只调整必要的主题/公共接口适配”；§1.1 |
| D2 | 主题角色 | 根补 `text-foreground`；选中态用 `bg-accent` + `text-accent-foreground` 成对 | 修复深色下选中行/项继承前景导致的低对比；局部 ThemeBoundary 自动生效 | §1.1 缺口；WORK-F22/AC-2 |
| D3 | 焦点 | 统一为 2px `outline`：`outline-ring`，选中态用 `outline-current` | 与 P4 皮肤同形；选中背景上 ring 对比不足时取前景色 | §1.1；menus.css 焦点规则 |
| D4 | 部件与状态 | 增 `data-rustify-part` 与 `data-state`/`data-selected`/`data-highlighted`/`data-disabled`/`data-invalid`；保留 `data-name` 与 test id | Inspector 无需改码即可定位；既有选择器不失效 | §5.2；inspector.rs:371-372 |
| D5 | 空、错误、禁用 | DataTable/Tree 增 `empty_label`；FilePicker/DropZone 增 `invalid`；不为 Tree/Splitter/PanelTabs 新增 disabled | 只补有真实路径的状态；未加的写入目录限制 | WORK-F22/AC-1；§5.3 |
| D6 | 默认文案 | 新增 `Message::{Commands, TypeCommand, NoMatches, ResizePanel}`，Close/Empty 复用已有 | 英文默认词不变，中文 scope 不再显示硬编码英文 | i18n.rs；§1.1 |
| D7 | 语义修正 | PanelTabs 把关闭按钮移出 `role=tab`；CommandPalette 空提示移出 listbox；DropZone 用 `role=group`；Splitter 补 valuemin/valuemax | 解决嵌套交互与 listbox 非法子元素；test id 不变 | §1.1；WORK-N1 |
| D8 | 迁移事实源 | 目录新增 `LEGACY_ENTRIES` 与冻结基线，生成进 components.md | 见 ADR-1 | WORK-F23/AC-2、WORK-N5/AC-2 |
| D9 | 旧主题路径 | 保留、不改码、不设删除期限（C-6） | 旧路径继续与 ThemeScope 并存；AC-2 该子句写明不成立；新应用指向 ThemeScope/ThemeBoundary | C-6 |
| D10 | 浏览器验收范围 | 只跑 Theme Studio 与官方站（C-7） | 扩展组件在 Studio 陈列中验证；其余项目只做 wasm 编译与 host 测试；§9.1 列豁免 | C-7 |
| D11 | Pages 口径 | 本地 official-root/subpath/meta，不部署（C-8） | 线上站点不变；本地产物与部署路径同构（根、子路径、无 headers 静态宿主） | C-8；docs-pages.yml |
| D12 | 真实 200% | Playwright persistent context 启动真实 Chrome，在 profile 预置 `partition.default_zoom_level` | 实测 Chrome 156.0.8078.80：window 1280 下 innerWidth 640、devicePixelRatio 2，未预置时 1280/1；可自动化，关闭 P2—P4 记录的“真实浏览器 UI 缩放 missing”；既有 settingsPrivate 回归用例保留，不是本结论的证据 | §1.1 实验与既有用例 |
| D13 | 最低消费者版本 | 不升版、不发行；文档继续写“schema v2 需同一 source checkout，registry 0.2.1 不兼容”，setup notes 记录 revision | 首个含 v2 的 registry 版本号（PRD Q3）留给发行计划 | PRD §5.2 Q3、§1.3；themes.md:8 |
| D14 | 默认调色板语义 | 写入迁移指南：`--primary`、`--accent-foreground` 默认值变化；`--ring` 不变 | 关闭 P4 遗留的待决项；AC 明确要求颜色语义有去向 | WORK-F23/AC-2；P4 收口记录「评审修复」 |
| D15 | 目录修正 | Glyph 增 `DividerHorizontal`；Direction 映射 DirectionProvider；Primitives Label 映射 `primitives::Label`；Slottable 保持 no 并写明按 PRD §1.3 不做 | 关闭过期的 `no`；Slottable 是 React cloneElement 辅助 | §1.1；PRD §1.3 |
| D16 | 扩展组件 RTL | 不做，目录限制写明方向键为物理方向 | PRD F22 未要求；Tree/PanelTabs/Splitter 现状就是物理方向 | tree.rs、panel_tabs.rs、splitter.rs |

### 0.3 ADR-lite

#### ADR-1：旧入口去向进入目录事实源

- 状态：Accepted。
- 背景与驱动：WORK-F23/AC-2 要求每个旧入口、参数、事件、Button 变体、颜色语义、主题文件和 CSS 用法都有去向；WORK-N5/AC-2 要求逐项可查。现有迁移指南第 353—383 行是手写分组表，无法证明完整；P4 代码评审 R1 已发现目录与交付不同步的真实缺陷。
- 备选 A：继续在迁移指南手写表，一次性人工比对基线。改动最小，但完整性无机器证据，后续改稿可能漏行。
- 备选 B：`catalog/legacy.rs` 定义 `LEGACY_ENTRIES`，★`legacy_baseline.txt` 冻结 `11b69129` 的公开面，host 测试断言每个基线键恰好落在一行，`catalogue_markdown()` 生成到 components.md；迁移指南删除手写表并链接生成节。
- 决策：选 B。基线是不再变化的数据，测试只比较两份数据，不读源码文本。
- 正面后果：完整性可机器验证，复用现有 `xtask catalog --check` 防漂移；reference 与 legacy 两张表在同一文档。
- 负面/中性后果：多约 70 行目录数据与一个基线文件；基线由一次性提取生成，提取方法写入 M3 记录。
- 重新评估触发：旧入口全部退场，或发行计划决定把迁移表拆成独立文档。

### 0.4 职责与事实所有权

|  | 应用/消费者 | SDK（rustify-dom、rustify-components） | Theme Studio 与官方站 |
| --- | --- | --- | --- |
| 拥有 | 行、节点、文件的业务值，选择与展开、面板尺寸、命令执行、错误判定（`invalid`） | 主题角色映射、部件/状态标识、窗口化、焦点、默认文案与可访问语义 | 陈列样本、局部主题样本、Inspector 展示与诊断；官方站 demo |
| 不拥有 | 虚拟化、监听释放 | **校验文件内容、决定什么算错误、替应用排序或筛选** | 第二套扩展实现、另一份主题事实源、改写作者数据 |

一句话：组件只表达应用给出的状态，错误由应用判定；Studio 只读取真实组件，不为陈列另造行为。

### 0.5 明确不在本期

- **旧主题路径迁移或删除**（ThemedScope、ThemeOverride、ThemePatch、use_theme、use_theme_values）——用户决定不改（C-6）；property-workbench、vellum、component-catalog 继续使用，不做 Vellum twin 比对。
- **Theme Studio 与官方站之外的浏览器回归**——用户决定收窄（C-7）：property-workbench、data-workbench、fusion-basic、vellum、component-catalog、utility-lab、`dom-only`、`dom-primitives`、`dom-hydrate`、`components-*`、`docs-*`、`theme-export*`、`budget*` 不运行浏览器项目，只做 §9 的编译与 host 检查。
- **扩展组件前后性能对比与 P2—P4 热路径在最终代码上重测**——用户决定收窄（C-7）；引用各期已有证据，并写明基线。
- **公开发布、版本变更与线上 Pages 部署**——PRD 非目标（C-5、C-8）；首个含 schema v2 的 registry 版本（PRD Q3）留给发行计划。
- **扩展组件 RTL、Tree 任意深度或虚拟化、DataTable 内建筛选排序**——保持现状，目录写限制（D16）。
- **新增 GPU 组件或菜单**——PRD §1.3；GPU 视图只报告实际支持。
- **真人辅助技术招募、物理触控设备采购**——Agent 只用本机已有能力尝试，缺失如实列出（V-12）。
- **性能调优**——发现回退如实记录，不在本期做专项调优；同一问题开销相近时选更优写法。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** P4 交付 7/7、代码复审 R3 通过（[P4 恢复快照](DOM-COMPONENTS-P4.md#恢复快照)、[P4 收口记录](DOM-COMPONENTS-P4.records/M7.md)）；Studio 陈列 43 组、Inspector 从 CSSOM 推导 token（gallery_catalog.rs、inspector.rs）。
- **[已核实·足够]** 公开面差异：基线 `11b69129` 的 34 个 `#[component]`、266 个参数在当前代码全部保留，只有 `Tooltip.children` 变为 `trigger`；公开名字仅 `button::ButtonClass`（宏生成）消失；其余为增量。本结论来自按文件的公开项与组件参数比对，迁移行为差异已在指南各节（dom-components-migration.md）。
- **[已核实·足够]** 扩展组件全部用 token utilities（`bg-muted`、`border-border`、`bg-popover`），实现自基线只有增删各 3 行（`git diff --stat 11b69129..HEAD`）。token 在 ThemeBoundary 内自动替换，浮层 Layer 继承创建处主题。
- **[已核实·足够]** 具名元素的 `transition-*` 已沿 `--motion-duration`（controls.css:7-9），host 减少动效时 ThemeScope 写 0；PanelTab、DropZone 有 `data-name`，无需新增动效规则。
- **[已核实·足够]** 官方站以 path 依赖当前 SDK（site/Cargo.toml）；source CLI 下 `serve.sh official` 使用当前 checkout，`official.spec.ts` 覆盖 12 篇指南与 24 个 CATALOG 页面的真实 demo。
- **[已核实·足够]** `mbx check --locked -p fusion-basic --target wasm32-unknown-unknown`、`--manifest-path tests/dom-only/Cargo.toml --workspace` 与 `tests/dom-only/public-panel-consumer` 的 wasm 检查均实跑通过（26 s、16 s、18 s，含缓存命中）。host `--lib --bins` 不覆盖 `cfg(wasm32)` 代码，因此 V-6 必须跑 wasm 目标。
- **[已核实·足够] 实验**：把 `{"partition":{"default_zoom_level":{"x":3.8017840169239308}}}` 写入新 profile 的 `Default/Preferences`，Playwright `launchPersistentContext` 启动 `/Applications/Google Chrome.app`（156.0.8078.80，headless，`viewport:null`，`--window-size=1280,800`），页面读到 innerWidth 640、devicePixelRatio 2、`max-width:768px` 命中；未预置的对照为 1280/1。
- **[已核实·足够]** 既有缩放用例：`theme-acceptance.spec.ts:354` 属 `theme-studio` regression 层，用 `chrome.settingsPrivate.setDefaultZoom(2)` 验证编辑、GPU 对比与导出；它需要临时 profile 与 `chrome://settings` 标签页，未设 `RUSTIFY_CHROMIUM` 时跑 Playwright 自带 Chromium。P4 §0.5 把真实缩放划给 P5，P4 未实跑。本期保留，随 V-7 运行，只作回归（仅去掉当前 Chromium 已不支持的 CDP `fromSurface: false`，断言不变）；`tests/native-zoom.ts` 是 V-10 的证据入口（真实 Chrome、启动前预置、不依赖设置页、官方站同样可用）。两条路径的结论不混用。
- **[已核实·缺口]** 选中态对比：`data_table/view.rs:11`、`tree.rs:17` 根无前景色，`view.rs:16-17`、`tree.rs:18` 只有 `aria-selected:bg-accent` → 深色默认主题选中文字继承 `#f9fafb`，叠在 `#32d583` 上只有 1.83:1（改用 accent-foreground 后浅/深为 7.19、9.86）→ §5.1。
- **[已核实·缺口]** 焦点：DataTable、Tree、PanelTabs、Splitter、FilePicker 用 `focus-visible:ring-ring/50`（半透明 box-shadow），默认主题对背景只有 2.02（浅）、2.89（深），低于 3:1，且与 P4 的 2px outline 不一致；不透明 ring 对背景为 4.37、8.15，但对选中背景只有 1.74、1.21 → §5.1。
- **[已核实·缺口]** Inspector 只认 `data-rustify-part, button, a, input, textarea, select, [role='button']`（inspector.rs:371-372），扩展组件只有 `data-name` → Tree 项、表格行、Tab 无法定位部件 → §5.2。
- **[已核实·缺口]** `panel_tabs.rs:127-140` 在 `role=tab` 内嵌 `<button>`（嵌套交互）；`command_palette.rs:257-262` 把无 role 的 `li` 放进 `role=listbox`；`drop_zone.rs` 的 `aria-disabled`/`aria-describedby` 在无角色 `div` 上；`splitter.rs:154` 只有 `aria-valuenow`，范围默认 0—100 而值是像素 → §5.4。
- **[已核实·缺口]** 硬编码英文：`command_palette.rs:100,105,261`、`splitter.rs:93`、`panel_tabs.rs:82`；调用方无法覆盖“nothing matches”；DropZone 已是 `Message` 同构（drop_zone.rs:43-48）→ §5.5。
- **[已核实·缺口]** DataTable `rows=0` 只剩表头，Tree 空 `nodes` 输出无 treeitem 的 `role=tree`；FilePicker/DropZone 无错误态 → §5.3。
- **[已核实·缺口]** Studio 陈列无扩展组件；`theme-gallery.spec.ts:82-83` 固定 43 组、8 节；陈列控件标签（Size/Variant/Palette/Disabled 等）只有英文（gallery.rs:18-35、71-72）→ §6。
- **[已核实·缺口]** 目录过期：`catalog.rs:1174` Direction 记 no 而 `primitives::DirectionProvider` 已存在（catalog.rs:1008）；`catalog.rs:1178` Label 记“无 unstyled primitive”而 `primitives::Label` 已导出（primitives/mod.rs:58）；`catalog.rs:1129` ThickDividerHorizontalIcon 记 no；`docs/components.md:480` 仍写“themed ordinary Table remains unimplemented”；`catalog.rs:972` SubmitButton 缺 `rules`、`on_asked`，`catalog.rs:973` FormStatus 缺 `saving`（form/submit.rs:17-30、62-69）→ §5.8。
- **[已核实·缺口]** 迁移指南是分期累加的：手写导出表（:353-383）无完整性证据；未写默认调色板变化；`:414` 写旧主题继续可用但无取舍说明；`:658` “仍缺能力”需核对 → §3、§5.8。
- **[已核实·缺口]** `theme-acceptance.spec.ts:244,253` 的 V6 用例断言已不存在的标题“GPU 主题样本”与文案“Below text thresholds”（theme_controls.rs:739 现为 “Fails text thresholds”）；P4 只跑 `-g "isolated failures"` 绕过 → 本期跑 Studio 全项目必须修断言 → §9.2。
- **[已核实·足够]** 旧主题路径：`ThemedScope` 走独立的 `claim_root(…, "legacy")` 投影（theme.rs:337），`component_color.rs:26` 读 `use_theme_values`；按 C-6 零改动。

### 1.2 拓扑与文件清单

★ 为新增；其余为已定位的现有落点。

| 系统/模块 | 改动路径与符号 | 边界 |
| --- | --- | --- |
| i18n | `crates/rustify-dom/src/i18n.rs`：`Message` 四项及 english/chinese/all/key | 不改 Locale 与 `use_locale` |
| 扩展组件 | `crates/rustify-components/src/data_table/view.rs`、`tree.rs`、`workspace/panel_tabs.rs`、`workspace/splitter.rs`、`workspace/command_palette.rs`、`file_picker.rs`、`drop_zone.rs`、`icon.rs` | 不改 `data_table/keys.rs`、`window.rs`、`splitter::{drag,initial,columns}`、`command_palette::{matching,neighbour}`、`tree::{rows,command}`；窗口化与键盘算法零改动 |
| 样式产物 | 重新生成 `crates/rustify-components/css/rustify.css`、`crates/rustify-components/css/candidates.json` | 无新增 CSS 文件 |
| 目录 | `crates/rustify-components/src/catalog.rs`；★`crates/rustify-components/src/catalog/legacy.rs`；★`crates/rustify-components/src/catalog/legacy_baseline.txt`；`docs/components.md` 生成区与首尾说明 | CATALOG 24 类不变 |
| SSG 测试 | ★`crates/rustify-components/tests/extensions_ssg.rs`；`.github/workflows/verify.yml` 的 `--test` 列表 | 同 `menus_ssg.rs` 结构 |
| Theme Studio | `examples/theme-studio/src/gallery_catalog.rs`、`gallery.rs`、★`gallery_extensions.rs`、`main.rs`（模块）、`app.css`、`README.md` | Inspector、controller、主题 schema 零改动；GPU 预览零改动 |
| 浏览器测试 | `tests/browser/theme-gallery.spec.ts`（含 Extensions 节 `@axe-core/playwright` 扫描，依赖已在根 package.json）、`gallery-matrix.ts`、`theme-acceptance.spec.ts`（V6 断言）；★`tests/native-zoom.ts`；★`tests/browser/theme-zoom.spec.ts`；`tests/docs-ssg/official.spec.ts` | 证据写 `docs/validation/dom-components-p5/` |
| 文档 | `docs/dom-components-migration.md`、`docs/workspace.md`、`docs/data.md`、`docs/themes.md`、`README.md`、`crates/rustify-components/README.md` | 只写当前事实；不改 `docs/zh/quickstart.md`，因 quickstart 不动 |
| 证据 | ★`docs/validation/dom-components-p5/` | 只放本期 JSON 与必要截图 |

**零改动邻接面**：makepad/、GPU 控件与 region、router、主题 schema 与存储、发布脚本、`site/src`（官方站 demo 随 SDK 回归）、`examples/{property-workbench,data-workbench,fusion-basic,vellum,component-catalog,utility-lab}`、`tests/dom-only/*`、`tests/theme-export/*`。这些只跑 V-6；若接口增量使其编译失败，属于本方缺陷，就地修。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| 扩展组件（增量） | 应用、Studio、官方站 demo | 旧参数全部保留；新增参数可选；窗口化池大小不变；test id 不变 | 无 | 选中/焦点/空态组合 | 进程内 | `extensions_ssg`、host 单测、Studio spec |
| `catalog/legacy.rs` | `catalogue_markdown`、`xtask catalog` | 每个基线键恰好落一行；每行有去向、理由、指南入口 | 无 | 精确键优先于通配 | 进程内 | `inventory_tests` |
| `gallery_extensions.rs` | `gallery.rs` | 样本状态局部，不改主题文档 | 无 | 无 | 浏览器 | theme-gallery.spec |
| `tests/native-zoom.ts` | Studio 与官方站 spec | 可执行文件只取 `RUSTIFY_CHROMIUM`，未设置或路径不存在时用例 skip；启动前写 profile，启动后断言 DPR 2、innerWidth 640，并记录实际路径与浏览器版本 | 无 | profile 与清理 | 本机 Chrome | 两个 evidence 用例 |

删除测试：去掉 `legacy.rs` 后完整性证明回到人工；保留。不为 `outline` 字面量或 `Message` 回退抽 helper：调用点是各组件的 class 常量与单条默认值，共享只会让 class 扫描更绕。

## 3. 公共 API 与迁移

无持久化 schema、导入导出或本地库变化。全部为增量，唯 `Message`、`Glyph` 增枚举项对穷尽 `match` 是源码级破坏，`Message::all()` 的返回类型也从 `[Self; 19]` 变为 `[Self; 23]`。

1. `DataTable`、`Tree` 增 `#[prop(optional, into)] empty_label: String`；空串取 `use_locale().text(Message::Empty)`。
2. `FilePicker`、`DropZone` 增 `#[prop(optional, into)] invalid: Signal<bool>`。
3. `Glyph::DividerHorizontal`（`name()` = `divider-horizontal`，`path()` = `M5 12h14`）。
4. `Message::{Commands, TypeCommand, NoMatches, ResizePanel}`，英文词与现有硬编码相同（`commands`、`type a command`、`nothing matches`、`resize panel`），中文为“命令”“输入命令”“没有匹配项”“调整面板大小”，key 为 `commands`、`type-command`、`no-matches`、`resize-panel`。
5. 旧主题路径（C-6）：目录行 fate=Kept，理由写“用户决定保留，当前无删除计划；新应用用 ThemeScope/ThemeBoundary”。指南不得写删除期限，也不得写已满足“无平行实现”。
6. 最低消费者版本（D13）：迁移指南「Published and source version pairing」与 themes.md 保持“schema v2 需同一 source checkout”，补一句“首个含 v2 的 registry 版本尚未确定，确定前旧消费者不兼容”。
7. 迁移指南结构：开头增「How to migrate」清单（版本配对 → 样式入口 → 主题文档 → 组件替换 → 验证），「Existing exports that remain available」手写表替换为指向生成节的说明和只对例外的详述；新增「Rustify extensions」（新参数、新增 `Message`/`Glyph` 变体与穷尽 `match` 需补的分支、`Message::all()` 长度 19→23、PanelTabs 结构变化、选中配色、部件标识）与「Default palette and color semantics」（D14）两节；修正 Glyph 数量与「Capabilities that are still absent」。

### 3.1 LEGACY_ENTRIES

```rust
pub enum LegacyFate { Kept, Changed, Replaced, Removed }
pub struct LegacyEntry {
    /// 基线键；以 `.*` 或 `::*` 结尾的是通配，精确键优先。
    pub names: &'static [&'static str],
    pub fate: LegacyFate,
    pub destination: &'static str,
    pub reason: &'static str,
    /// 迁移指南锚点或真实示例路径；Kept 可为空。
    pub guide: &'static str,
}
pub const LEGACY_ENTRIES: &[LegacyEntry];
```

fate 语义：Kept 同名同义；Changed 同名但行为或输出有差异（如 Menu 外部关闭、Progress ARIA、ScrollArea class 落到 Root）；Replaced 旧用法失效，改用新入口（Button 变体与尺寸、`Tooltip.children`）；Removed 无同名去向（`button::ButtonClass`）。

`legacy_baseline.txt` 每行一个键，按字母序：

| 前缀 | 来源 | 示例 |
| --- | --- | --- |
| `export:` | 基线 `crates/rustify-components/src/**/*.rs` 与 `crates/rustify-dom/src/theme.rs`、`theme/*.rs` 的 `pub fn/struct/enum/type/const/static/mod/trait` 名，以及 `lib.rs` 的 `pub use` 名；另手工加 `variants!` 生成的 `button.rs::ButtonClass` | `export:tree.rs::rows` |
| `prop:` | 基线 34 个 `#[component]` 的参数 | `prop:Tooltip.children` |
| `value:` | 基线 `ButtonVariant`、`ButtonSize` 的取值 | `value:ButtonVariant::Secondary` |
| `semantic:` | 主题语义：基线 `Theme::properties()` 的全部 23 个 token（键去掉 `--`），加基线 `sdk.css` `@theme` 中不在其内的 `--color-*` 别名（`card-foreground`、`popover-foreground`、8 个 `sidebar*`），均从基线机械提取；另手工加默认值变化键（D14） | `semantic:accent`、`semantic:spacing`、`semantic:sidebar-ring`、`semantic:primary-default`、`semantic:accent-foreground-default`、`semantic:ring-default` |
| `file:` | 主题文件，手工枚举 | `file:theme-json-v1`、`file:theme-css-export`、`file:theme-rust-export`、`file:studio-storage-v1` |
| `css:` | CSS 用法，手工枚举 | `css:rustify.css`、`css:sdk.css`、`css:theme-v4.css`、`css:styles-module`、`css:global-reset` |

host 测试（`inventory_tests`）：每个基线键恰好解析到一行（精确优先于通配，通配之间不得重叠）；每行 `destination`、`reason` 非空；每个通配至少命中一键；`catalogue_markdown()` 包含 `## Legacy API destinations`。

rustify-dom 其他公开项与 rustify-ui 门面的公开名字相对基线无消失，不进基线，在该节开头用一句话说明并给出比对范围。

### 3.2 回滚

以提交回退 SDK、CSS、Studio 与文档；无数据迁移。回退不影响 v1/v2 存储。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| Studio Inspector | 已核实缺口（扩展不可定位） | inspector.rs:371-372 只认 part 与原生控件；CSSOM 推导已覆盖嵌套变体 | SDK 增 part，Inspector 零改动 | 无 token 显示“未标注”，不编造 |
| 官方站 CATALOG demo | 已核实足够 | site/Cargo.toml path 依赖；official.spec.ts:324-331 真实 demo | 零改动；随 SDK 回归 | spec 失败按本方缺陷修 |
| 其余示例与消费者 | 已核实足够（接口增量） | §1.1 参数比对；wasm check 实跑通过 | 只跑 V-6 | 编译失败就地修；运行时回归不在本期检测范围（C-7） |
| 旧主题路径 | 已核实足够 | theme.rs:337、component_color.rs:26 | 零改动（C-6） | 无 |
| 基线产物 | 已核实可取 | `11b69129` 工具链相同（mise.toml 一致）；前期已用实际旧源码重建 Studio | M4 按 §9.3 worktree 构建，用后清理 | 构建失败则 V-11 记未运行及原因 |
| 真实 Chrome | 已核实可用 | §1.1 实验 | `tests/native-zoom.ts`，路径由 V-10 命令的 `RUSTIFY_CHROMIUM` 传入；既有 settingsPrivate 用例保留（§1.1） | 变量未设置或路径不存在时 evidence 用例 skip，V-10 记未运行 |
| VoiceOver、OS IME、物理触控 | 未核实 | 本机有 VoiceOver，AppleScript 控制权限未知；此前各期未自动化 | V-12 尝试，不改系统设置 | 缺权限或设备列 missing，不阻塞（AC 允许） |

没有需要其他仓库、账号、服务或外部部署才能解除的依赖。

## 5. 核心机制与公共接口

### 5.1 主题角色与焦点（class 常量）

替换字面量，不引入共享常量（§2 删除测试）：

- `data_table/view.rs`：`GRID` 增 `text-foreground`；`ROW` 增 `group/row aria-selected:text-accent-foreground`；三处 ring 形态不一，逐处换为同一 outline（`focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-ring focus-visible:-outline-offset-2`）：`GRID` 去掉 `focus-visible:ring-ring/50 focus-visible:ring-[3px]`；`HEADER_BUTTON` 去掉 `focus-visible:ring-ring/50 focus-visible:ring-2`；`CELL` 去掉 `focus-visible:ring-ring/50 focus-visible:ring-2 focus-visible:ring-inset`，再加 `group-aria-selected/row:focus-visible:outline-current`。
- `tree.rs`：`TREE` 增 `text-foreground`；`ITEM` 增 `aria-selected:text-accent-foreground aria-selected:focus-visible:outline-current`，焦点同上替换。
- `panel_tabs.rs`、`splitter.rs`、`file_picker.rs`：焦点同上替换（FilePicker 去掉 `focus-visible:outline-none`，改 `outline-none` 为基底）。
- `command_palette.rs`：样式保持，`bg-muted` 高亮不变。

`outline-none` 保留为基底：Tailwind 4 的 `outline-none` 写 `--tw-outline-style: none`，`focus-visible:outline-solid` 才能恢复。改后跑 `mbx xtask css`。

### 5.2 部件与状态标识

新增属性均为静态或复用已有 Memo，不新增信号；新增 part/state 不改变既有 `data-name`、`data-testid`。ARIA 变化仅限 §5.3 的空与错误语义和 §5.4 的语义修正。

| 组件 | 元素 | `data-rustify-part` | 状态 |
| --- | --- | --- | --- |
| DataTable | 根 | `data-table` | `data-state=ready\|empty` |
| | 表头单元格 | `data-table-header` | — |
| | 行 | `data-table-row` | 选中 `data-selected`（复用 `chosen`） |
| Tree | 根 | `tree` | `data-state=ready\|empty` |
| | 项 | `tree-item` | 可展开项 `data-state=open\|closed`；选中 `data-selected`（复用 `chosen`） |
| PanelTabs | 根 | `panel-tabs` | — |
| | 框（§5.4） | `panel-tab` | `data-state=active\|inactive` |
| | 关闭按钮 | `panel-tab-close` | — |
| Splitter | 根 | `splitter` | — |
| | 分隔条 | `splitter-divider` | `data-orientation=vertical`、`data-state=dragging\|idle` |
| CommandPalette | 面板 | `command-palette` | — |
| | 搜索框 | `command-search` | — |
| | 选项 | `command-item` | `data-highlighted`、`data-disabled` |
| FilePicker | input | `file-picker` | `data-disabled`、`data-invalid` |
| DropZone | 根 | `drop-zone` | 既有 `data-state`，加 `data-disabled`、`data-invalid` |

`data-selected` 选中时为空属性，与 P2 的 `data-disabled`/`data-invalid` 同形；树项的 `data-state` 已被展开状态占用，所以行与项统一用它，调用方只写一个选择器。

表格单元格与 Icon 不加 part：Inspector 取“元素 + 最近部件”，单元格落到行才能同时读到 `accent`；Icon 加 part 会让 Button 内图标抢走 Button 的部件定位。

### 5.3 空与错误

- **DataTable**：`rows=0` 时在行区渲染一行 `<div role="row" aria-rowindex="2" data-testid="{name}-empty"><div role="gridcell" aria-colindex="1" aria-colspan={列数}>{label}</div></div>`，`aria-rowcount` 记 2，空行不参与选中、键盘与 `data-row-id`。grid 的子元素必须是 row/rowgroup，所以不用普通段落。
- **Tree**：`shown` 为空时根不输出 `role=tree` 与 `aria-label`，渲染 `<p data-rustify-part="tree-empty" data-testid="{name}-empty" class="px-2 py-1 text-muted-foreground">`。有节点时不变。
- **FilePicker**：`aria-invalid=invalid.then_some("true")`，`FIELD` 增 `aria-invalid:text-destructive aria-invalid:file:border-destructive`。
- **DropZone**：`invalid` 且未拖入时加 `INVALID = "border-destructive text-destructive"`；禁用优先于错误。错误说明沿用既有 `described_by`，由应用提供。
- 组件不判断文件内容；错误来自应用。DataTable/Tree 没有错误属性，应用用 Callout 或 `empty_label` 表达加载失败；写入 docs/data.md 与目录限制。

### 5.4 语义修正

- **PanelTabs**（panel_tabs.rs:122-170）：结构改为 `tablist > div[role=presentation] > div[role=tab]（内含 label 与可及树外的鼠标 close span）`；close 点击 `stop_propagation` 防误激活、Delete 键在活动 tab 上关闭（M2 实测任何 control/tab-stop 形态的 close——sibling button、tab 内 button、tab 内 span[role=button]——均触发 axe critical/serious，最终采用可及树外鼠标目标+键盘 Delete，就地修订）。

  ```text
  FRAME   = "group inline-flex items-stretch rounded-t-md border border-transparent text-sm text-muted-foreground whitespace-nowrap transition-colors hover:bg-muted data-[state=active]:bg-background data-[state=active]:border-border data-[state=active]:text-foreground"
  TRIGGER = "px-3 py-1.5 cursor-pointer rounded-t-md outline-none focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-ring focus-visible:-outline-offset-2"
  ```

  框带 `data-name="PanelTabFrame"`（保留 `--motion-duration`）；`data-name="PanelTab"`、`panel-tab-{id}` test id、`aria-selected`、`tabindex`、click 都在 `role=tab` 上。关闭按钮 `mr-1.5 self-center`，`close-{id}` 不变；框上不挂 click，关闭按钮不再嵌套在 tab 内，删除 stop_propagation。
- **CommandPalette**（command_palette.rs:213-263）：`ul` 增 `hidden=move || shown.get().is_empty()`；空提示移到 `ul` 之后，`<p role="status" data-testid="command-empty" class="px-4 py-3 text-sm text-muted-foreground">`。搜索框 `aria-expanded=move || (!shown.get().is_empty()).to_string()`；`aria-activedescendant` 只在 `active` 位于 `shown` 中时输出（`Option` 属性，不新增信号），避免 IDREF 指向已被过滤掉的选项。
- **DropZone**：`role="group"`、`aria-label` 与可见文字取同一 reactive 标签（§5.5）。
- **Splitter**：分隔条增 `aria-valuemin`、`aria-valuemax`（取整），与 `drag` 同一夹取边界和同一越界容忍：min 为 `mins.get(index).copied().unwrap_or(0.0)`，max 为 `sizes` 第 `index`、`index + 1` 项之和减 `mins.get(index + 1).copied().unwrap_or(0.0)`，`sizes` 同样经 `.get(…).copied().unwrap_or(0.0)` 读取。拖动同一分隔条时两侧之和恒定，max 是稳定上界。

### 5.5 默认文案

默认值只在调用方传空时使用，并放进 reactive closure，语言切换后更新（`LocaleHandle::text` 是 tracked，i18n.rs:343）：

- CommandPalette：`placeholder`、`aria_label` 的两处 `StoredValue` 存调用方原值，读取时空串取 `TypeCommand`、`Commands`；空提示取 `NoMatches`（无覆盖参数，保持接口不变）。
- Splitter：`divider_label` 空串取 `ResizePanel`。PanelTabs：`close_label` 空串取 `Message::Close`（英文仍是 `close`）。
- DropZone：`label` 改存调用方原值，空串在 reactive closure 中取 `DropFileHere`，可见文字与 `aria-label` 共用，写法同 Splitter。

### 5.6 Tree 图标

`tree.rs:257-265` 的 `·▾▸` 文本换成 `<Icon glyph=… class="size-3" />`（Dot / ChevronDown / ChevronRight），外层 `aria-hidden` 由 Icon 默认提供，不再依赖字体含这些字符。

### 5.7 Glyph

`icon.rs` 增 `DividerHorizontal`，同步模块注释（`icon.rs:6`）与 `catalog.rs:215`、`catalog.rs:220`、`catalog.rs:935` 的 Glyph 数量。`catalog.rs:143`、`:1549`、`:1698` 的“six”指类别、配置控件和能力维度，与 Glyph 无关，不改。M3 重新生成 `docs/components.md` 后核对 Icon 行与 `icon` 类别不再写六种。

### 5.8 目录修正

- DataTable、Tree、FilePicker、DropZone 的 `fields` 增新参数；Tree/DataTable/PanelTabs/Splitter 的 `limits` 写明方向键为物理方向、无 disabled 或无错误属性的处理方式；`verification` 增 `tests/browser/theme-gallery.spec.ts`、`crates/rustify-components/tests/extensions_ssg.rs`。
- `catalog.rs:1129` → `component: Some("Icon")`，Partial，沿用同组三个 Icon 的 limitation。
- `catalog.rs:1174` → `component: Some("DirectionProvider")`，Partial，limitation 写明 Rust provider 与 React Direction.Provider 的差异。
- `catalog.rs:1178` → 同构 `Primitives.Select` 新增 `component!("Primitives.Label", Helper, "rustify_components::primitives::Label", …)`，映射到它，Partial。
- `catalog.rs:1149` Slottable 保持 no，limitation 改为“React cloneElement 辅助，PRD §1.3 不逐字兼容；typed Slot 覆盖单元素组合”。
- `SubmitButton` 的 fields 补 `rules`、`on_asked`，`FormStatus` 补 `saving`。
- `docs/components.md` 非生成区：删 :480 的过期句，首段按当前能力改写“unstyled Primitives”措辞，增 `Legacy API destinations` 的说明。

## 6. 前端与交互

- **入口与信息架构**：沿 P4 的 Preview → Scene「Components」。`gallery_catalog::Section` 增 `Extensions`（`["Rustify extensions","Rustify 扩展"]`），放在 Typography 之后；`ENTRIES` 增 6 行（`primitives_path` 全为 None）：`["DataTable"]`、`["Tree"]`、`["FilePicker","DropZone"]`、`["Splitter","PanelTabs"]`、`["CommandPalette"]`、`["Icon","Glyph"]`，共 49 组、9 节。`gallery::sample` 分派到 ★`gallery_extensions.rs`。`EXCLUDED` 与“每个稳定参考恰好一个样本”的测试不变（扩展不是参考名）。样本组件照 `examples/component-catalog/src/main.rs:733-800` 与 `tests/dom-only/hydration/src/view.rs:279` 的调用写法。

| 样本 | 内容 | 控件 | test id / 可观察结果 |
| --- | --- | --- | --- |
| DataTable | 100 000 行 × 6 列，`row_id`=行号+1，h-48，排序按钮 | Empty、Selected | `gallery-table`；`aria-rowcount=100001`；空态同时显示 `Restore rows` 按钮；Space 选中、滚走再回仍选中；Ctrl+End 到末行；DOM 行数不随总行数增长；窄屏表格内部横向滚动 |
| Tree | 3 组 × 3 子项 | Empty | `gallery-tree`；方向键/Enter；选中配色；空提示与 `Restore tree` 按钮 |
| FilePicker · DropZone | 共用 `<output>` 显示文件名；DropZone 不传 `label`，显示 SDK 默认词 | Disabled、Invalid | `gallery-file`、`gallery-drop`；`setInputFiles`、`DataTransfer` drop；禁用不出请求；Invalid 边框与 `aria-invalid`，样本同时显示一句拒绝理由并经 `described_by` 关联，随 Invalid 出现和消失 |
| Splitter · PanelTabs | 三面板，`workspace::columns`；四个可关闭 Tab，一个固定 | Reset | `gallery-split`、`gallery-tabs`；键盘调整到最小值停住；关闭当前移到相邻；Tab 与关闭按钮可分别聚焦 |
| CommandPalette | 打开按钮 `aria-haspopup="dialog"`；可运行、不可运行（带原因）各两条 | — | `gallery-command`；搜索、空提示、运行一次、不可运行不触发、Escape 关闭回焦 |
| Icon · Glyph | 七个 glyph 与具名/装饰两种 | — | `gallery-icon`；具名 `role=img` |

- **局部主题**：`LocalThemeSample` 之外新增 `ExtensionLocalSample`，放在 Extensions 节末，`ThemeBoundary` 内放小型 Tree、PanelTabs 与打开 CommandPalette 的按钮；开关切换 `accent`、`accent-foreground`、`border`、`popover`、`popover-foreground` 的局部 patch，并 `RegisterPreviewSource` 登记标签“Extension local theme / 扩展局部主题”。
- **状态**：Empty/Invalid/Disabled 由样本控件真实触发，不放假按钮；空提示文字来自 SDK。Empty 样本沿 Select/Table 配 `Restore …` 动作（`theme-gallery.spec.ts` 的状态循环要求它出现）。
- **对比与 axe 用例**：`theme-gallery.spec.ts` 的状态循环增 `Invalid`；矩阵增 DataTable、Tree 选中后聚焦该元素的步骤，以及 FilePicker Invalid 的步骤，断言文字 ≥4.5:1、焦点与 Invalid 边框 ≥3:1，结果写入证据 JSON。代表名单增 `DataTable`、`Tree`、`FilePicker`，使 V-2 的 regression 也覆盖，V-9 覆盖全部 49 组。`gallery-matrix.ts` 的焦点边在 `outline-offset < 0` 时按元素自身背景计算（内缩 outline 画在自身背景上，选中项的背景不在父元素）；增 `invalid boundary` 边：DropZone 读根边框，FilePicker 读 `::file-selector-button` 边框。新增 axe 用例在浅深两模式下扫 Extensions 节的默认态与选中、空、错误态，再打开 CommandPalette 输入无匹配词并扫其 Layer；标签与 `tests/docs-ssg/a11y.ts` 的 `axeTags` 相同，只断言 serious/critical。axe 不判定 outline 与边框对比，二者由矩阵负责。
- **本地化**：`Gallery` 提供 `GalleryLocale(Signal<Locale>)` context；`choice()` 与 `StateToggle` 的 label 经 `shell_label(en)` 映射中文（Size 尺寸、Variant 变体、Palette 配色、High contrast 高对比、Disabled 禁用、Loading 加载中、Indeterminate 半选、Show parts 显示部件、Empty 空、Low contrast 低对比、Invalid 错误、Selected 选中、Reset 重置）；variant 取值与样本内容保持 API 原文。`aria-label` 同步。既有 spec 在默认英文 locale 下的 `getByLabel` 不受影响。
- **响应式**：`.gallery-demo` 内 DataTable 用 `class="h-48 w-full"`，表格 `min-w-0`，不撑宽 sample；390/768/桌面沿 `gallery layout keeps samples…` 用例。
- **视觉**：Extensions 节版式与 Inspector 信息结构沿 P4 surface brief；扩展组件配色和间距变化按 AGENTS.md 用 impeccable，依据 DESIGN.md 与 `.impeccable/surfaces/examples-theme-studio-src-app-rs.md`（`@implementation_route` 的明确例外）；不改 DESIGN.md 的 token 表。
- **GPU**：陈列已显示“组件陈列仅 DOM”（app.rs:575），不增 GPU 样本。
- **前端隐藏只改善体验**：本期无权限模型。
- **真实浏览器走查**：V-2、V-7、V-9、V-10；console、pageerror、requestfailed、CSP 违规必须为空，预期内 404 单独声明。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD AC-3；WCAG 2.2 AA | Studio 与官方站主路径在真实 200%、减少动效下可完成；Studio Extensions 节与无匹配的 CommandPalette 的 axe serious/critical 为 0（§6）；AT 实测或列 missing | 缺陷按范围修；AT 无法执行列未验证 | §5.1—§5.4；`tests/native-zoom.ts` | V-2、V-10、V-12 |
| WORK-N2 | THEME-STUDIO §7、PRD Q4；无毫秒 SLO | Studio 主题编辑同机新旧 p50/p95；Studio、data-workbench、fusion-basic 压缩尺寸差值；扩展样本操作后资源不增长 | 回退如实记录，不调优 | 窗口化算法零改动；新增属性复用既有 Memo | V-2、V-11 |
| WORK-N4 | architecture.md；mise.toml | 根/子路径/无 headers 官方站、Studio 根与子路径、Studio 导出、GPU 预览成立；外置资产；工具链 major 不变 | 编译失败就地修；消费者浏览器按 C-7 不验 | 无新依赖 | V-6、V-7、V-8、V-13 |
| WORK-N5 | catalog.rs | 目录与交付一致；差异可查 | `xtask catalog --check` 与 host 测试失败 | ADR-1 | V-3、V-9、V-13 |
| WORK-N7 | DESIGN.md、surface brief | 390/768/桌面与 200% 主路径可达；数据区域局部滚动；中英文切换不改作者数据 | 缺陷按范围修 | §6 | V-2、V-7、V-8、V-10 |

安全：无新入口、无新导入路径；FilePicker、DropZone 仍把文件交给应用；导入校验、CSP 零改动，Studio 全项目回归覆盖。

可观测性：不新增；沿现有 `hooks.runtime.stats()`、`__theme_studio.stats()`。

## 8. 失败模式、发布与回滚

不引入新的运行期失败模式：本期是 class、属性、可选参数和文案增量，算法、监听、持久化与生命周期不变，因此不设失败模式表。

- 发布顺序：无发布；SDK、CSS、Studio、文档同一提交。
- 回滚门：§3.2。
- 已知风险：C-7 使 Studio 与官方站之外的运行时回归无法被本期发现，见 §11。

## 9. 验证

### 9.1 行为与接口验证

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F22/AC-1—AC-2、WORK-F5（host 可见部分）：§5.2 部件与状态属性；DataTable 空行 ARIA；Tree 空根无 `role=tree`；PanelTabs 中 `role=tab` 不含 `button`；FilePicker `aria-invalid`、DropZone `role=group`；中文 locale 下默认文案；Glyph；既有窗口化/键盘单测不变 | `mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg`；`mbx test --locked -p rustify-components --no-default-features --features ssr --lib`；`mbx test --locked -p rustify-components --lib`；`mbx test --locked -p rustify-dom --lib i18n`；`mbx xtask css --check`；`mbx clippy --locked -p rustify-components --all-targets -- -D warnings`；`cargo fmt --all -- --check` | M1：§5.1—§5.7 全部交付后 |
| V-2 | WORK-F19/AC-3（扩展部分）、WORK-F22/AC-1—AC-2、WORK-F4、WORK-N1（axe）、WORK-N2（资源）、WORK-N7（局部滚动、中英文）：§6 表每行；49 组/9 节无 `data-rustify-issue`；局部 patch 只改边界内且浮层继承；Inspector 拾取 Tree 项、表格行、Tab、Command 项显示 `accent`、`accent-foreground`、`border`、`popover` 与来源；Icon 显示未标注；§6 的 axe 用例 serious/critical 为 0（浅深，含选中、空、错误态与无匹配的 CommandPalette）；DataTable 选中行、Tree 选中项、FilePicker/DropZone Invalid 达 §6 对比门槛；CommandPalette 无匹配时 `aria-expanded=false`、无 `aria-activedescendant`、`ul` 隐藏、提示可读；Invalid 态 `aria-describedby` 指向可见理由；20 轮操作后 `__theme_studio.stats()` 与节点数不增长；切 zh 后控件标签与 SDK 默认词（含 DropZone 默认标签）变化，`snapshot().document`、`revision` 不变 | `mbx test --locked -p theme-studio --bins`；`mbx xtask build-web --example theme-studio --release`；`npx --no-install playwright test --project=theme-studio theme-gallery.spec.ts` | M2：§6 全部交付后 |
| V-3 | WORK-F23/AC-2、WORK-N5/AC-2：§3.1 host 测试；参考表无过期 `no`；`Legacy API destinations` 已生成；旧主题行写明保留 | `RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib catalog`；`mbx xtask catalog --write docs/components.md --check` | M3：目录与文档交付后 |
| V-4 | WORK-F23/AC-3（编译层角色模拟）：Agent 模拟按公开指南迁移的原应用开发者，把基线 hydration fixture 改到能在当前 SDK 编译，记录卡点并修指南 | 见 §9.3；`mbx check --manifest-path <副本>/Cargo.toml --no-default-features --features ssr`（不带 `--locked`，原因见 §9.3）；同清单 `--features hydrate --target wasm32-unknown-unknown` | M3：V-3 通过后；由独立上下文 subagent 执行 |
| V-5 | 全部 AC 的静态与 host 门：§9.5 | §9.5 | M4：检查序列首项（§9.3 的基线产物构建与 §10 的 M4 交付物在此之前） |
| V-6 | WORK-F23/AC-3、WORK-N4（C-7 下的编译层）：其余示例与消费者编译通过 | §9.2 | M4：V-5 之后 |
| V-7 | WORK-F19/AC-1—AC-3、WORK-N7、WORK-N4/AC-2（Studio 部分）：Studio 全项目；根与子路径；GPU 预览、GPU 局部故障、fatal 重启；导出与导入；旧站回滚 | §9.4 | M4：V-6 之后 |
| V-8 | WORK-N4/AC-2、WORK-N7、WORK-F23/AC-3（官方站）：12 篇指南与 24 个 CATALOG 页，根、子路径、无 headers 静态宿主；Tree、DataTable 等 demo 可操作；无外源、无 Makepad | §9.4 | M4：V-7 之后，串行 |
| V-9 | WORK-N5/AC-2、WORK-N1、WORK-F19：完整视觉矩阵（49 组，浅深），未获参考例外的文字 ≥4.5:1，焦点 ≥3:1，含选中行/项的焦点与 Invalid 边框（§6），证据 JSON 含对应 `edges`；D11 参考组合沿 P4 规则单列 | `RUSTIFY_TIER=evidence npx --no-install playwright test --project=theme-studio theme-gallery.spec.ts -g 'matrix complete'` | M4：与 V-7 同一 Studio 产物 |
| V-10 | WORK-N1/AC-3、WORK-N7/AC-3：真实 Chrome 200%（DPR 2、innerWidth 640）下，Studio 走预设→精调→陈列→Dialog/Select/CommandPalette→导出，官方站走首页→导航→`/components/tree/`、`/components/data-table/` demo→模式切换；文档无横向溢出，浮层在视口内，表格局部滚动；减少动效下 Studio 同路径完成且 PanelTab、DropZone 过渡为 0 | `RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_TIER=evidence npx --no-install playwright test --project=theme-studio theme-zoom.spec.ts`；`PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH" RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_TIER=evidence npx --no-install playwright test -c playwright.docs.config.ts --project=official-subpath official.spec.ts -g 'native 200'`；记录实际可执行路径与版本 | M4：V-8 之后 |
| V-11 | WORK-N2/AC-2：§9.3 的新旧 Studio 同机 p50/p95；三个示例压缩尺寸差值；`rustify.css` 差值；旧功能（data-workbench、fusion-basic）与新能力（theme-studio）分列；P2—P4 的 JSON 引用并写明基线 | §9.3 | M4：基线产物最先构建；测量在 V-10 之后，无其他构建或测试并行 |
| V-12 | WORK-N1/AC-3：VoiceOver + Chrome 走 Studio 主路径与官方站 `/components/tree/`；OS IME 在 Studio 字段；物理触控。每项记录已实测、用户确认或 missing | 本机已有的 computer-use、osascript；不改系统设置，不下载；步骤清单写入记录供负责人后续执行 | M4：与 V-10 同一产物；缺失不阻塞 |
| V-13 | 全部 AC：逐条“AC → 证据 → 豁免”可追溯；文档与交付一致；工具链与依赖未变 | `git diff 11b6912919a2a34c372aac4d6b7d51052820600f HEAD -- mise.toml package.json`；`RUSTIFY_DOCS_CARGO=mbx target/docs-cli/rustify-docs check --config rustify-docs.toml --format json`；§9.4 清理；`git diff --check` | M4：收口末项 |

**批内最小反馈**：组件改动用 `mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg <用例名>`；目录改动用 `RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib catalog::`；Studio 样本先 `mbx check --locked -p theme-studio --target wasm32-unknown-unknown`，浏览器用 `-g` 选单用例。Studio release 构建约 3 分钟（P4 记录），不作为每次编辑的反馈。

**收窄后的结论口径**（C-6、C-7）。M4 记录与最终报告必须按此表分开陈述，豁免项不写“通过”：

| AC | 本期可下结论的范围 | 用户豁免，须写明未验证 |
| --- | --- | --- |
| WORK-F19/AC-3 | 全部（Studio） | 无 |
| WORK-F22/AC-1 | Studio 样本的选择、导航、拖放、文件、工作区、可访问语义、空/错误/禁用 | 原示例内的同类任务（p2-workspace、p3-table、m6-components） |
| WORK-F22/AC-2 | 主题角色、局部覆盖、100 000 行窗口化 | 原有同类负载的规模与性能前后比较 |
| WORK-F23/AC-2 | 全部去向表与示例 | “不保留无边界期限的平行实现”对旧主题路径不成立（C-6） |
| WORK-F23/AC-3 | Studio、官方站浏览器；其余编译与 host；编译层角色模拟 | 其余示例、DOM-only、独立主题消费者的浏览器任务完成 |
| WORK-N1/AC-3 | Studio、官方站的 200%、减少动效；V-12 实测项 | 其余消费者；V-12 的 missing 项 |
| WORK-N2/AC-2 | Studio 主题热路径同机对比、三个示例尺寸、扩展样本资源 | 扩展组件前后对比；P2—P4 热路径在最终代码上重测 |
| WORK-N4/AC-2 | 官方站三种部署、Studio 根与子路径、Studio 导出、GPU 预览、`tests/theme-export` host 测试 | 独立主题消费者与 DOM-only 的浏览器、`deployment`、`components-*`、`docs-*` |
| WORK-N4/AC-3 | 全部 | 无 |
| WORK-N5/AC-2 | 目录、Studio 矩阵、host SSG | `dom-hydrate` 浏览器接管重测 |
| WORK-N7/AC-3 | 全部 | 无 |

因此 P5 完成不等于 PRD UAT-13、UAT-14 全部通过，也不等于整份 PRD 无保留验收。

### 9.2 编译门（V-6）与 V6 断言修正

```sh
for p in fusion-basic property-workbench data-workbench component-catalog vellum utility-lab; do
  mbx check --locked -p "$p" --target wasm32-unknown-unknown || break
done
mbx check --locked --manifest-path tests/dom-only/Cargo.toml --workspace --target wasm32-unknown-unknown
mbx check --locked --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml --target wasm32-unknown-unknown
RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins
PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH" mbx xtask docs-verify --group features
```

逐包循环是为了不让 `utility-lab` 的 feature 统一掩盖其他包。`docs-verify --group features` 覆盖默认依赖图、GPU 切换、Docs 隔离、hydration 三模式与组件站构建，不跑浏览器。

`theme-acceptance.spec.ts` 的 V6 断言在 M4 按当前真实文案修正（标题、`Fails text thresholds`），先确认实际行为符合产品意图再改，不改产品文案迁就旧断言。

### 9.3 基线产物、性能与尺寸（V-11）与角色模拟（V-4）

```sh
# M4 最先执行：旧产物同时供 V-7 的 RUSTIFY_THEME_OLD_SITE
git worktree add ../rustify-ui-dom-baseline 11b6912919a2a34c372aac4d6b7d51052820600f
cd ../rustify-ui-dom-baseline && ln -s ../rustify-ui/node_modules node_modules
for e in theme-studio data-workbench fusion-basic; do
  mbx xtask build-web --example "$e" --release
  mbx xtask report-size --example "$e" --release --compressed
done
cp -R target/makepad-wasm-app/release/theme-studio "$TMPDIR/rustify-p5-old-studio"

# V-11：V-10 之后执行，无其他构建或测试并行
RUSTIFY_TIER=evidence RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
  npx --no-install playwright test --project=theme-studio theme-performance.spec.ts --headed --workers=1
mkdir -p ../rustify-ui/docs/validation/dom-components-p5
cp docs/validation/theme-studio/M6/performance.json \
  ../rustify-ui/docs/validation/dom-components-p5/performance-paired-old.json
cd ../rustify-ui
for e in theme-studio data-workbench fusion-basic; do
  mbx xtask build-web --example "$e" --release
  mbx xtask report-size --example "$e" --release --compressed
done
RUSTIFY_THEME_PERF_OUT=docs/validation/dom-components-p5/performance-current.json \
RUSTIFY_THEME_CONFIG_PERF_OUT=docs/validation/dom-components-p5/performance-config-current.json \
RUSTIFY_TIER=evidence RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
  npx --no-install playwright test --project=theme-studio theme-performance.spec.ts --headed --workers=1
git worktree remove --force ../rustify-ui-dom-baseline && git worktree prune
```

旧站先测，当前站后测，两次同机同窗口。基线用它自带的 spec，负载（40 次颜色编辑）前后一致。`target/` 由 mbx 管理且可能被清理，产物与证据先复制走。`rustify.css` 差值用 `git show 11b6912919a2a34c372aac4d6b7d51052820600f:crates/rustify-components/css/rustify.css | gzip -9 | wc -c` 对比当前文件。外部字体由 `report-size` 单列，确认不在 WASM 内（WORK-N4/AC-3）。

V-4 角色模拟：

```sh
mkdir "$TMPDIR/rustify-p5-migrate" && git archive 11b6912919a2a34c372aac4d6b7d51052820600f tests/dom-only/hydration | tar -x -C "$TMPDIR/rustify-p5-migrate"
```

- 角色：熟悉基线 API 的原应用开发者。可读：根 README、`docs/dom-components-migration.md`、`docs/components.md`、`docs/quickstart.md`、公开 crate 文档、编译器输出。不可读：当前 `tests/dom-only/hydration/`、SDK 私有实现、本计划。
- 权限：只改副本内文件，把路径依赖指向当前 checkout。切换后依赖闭包变化（如 `rustify-colors`、`cssparser`），基线 `Cargo.lock` 没有这些条目，因此不带 `--locked`，让 Cargo 更新副本的 lockfile；这是迁移的正常一步，不记为指南卡点。
- 任务与判据：副本在 `ssr` 与 `hydrate` 两种模式编译通过。基线 `view.rs` 含 50 处组件使用，覆盖 24 个类别。记录每个编译错误是否能从指南定位；指南答不了的就是卡点，修指南后结束。
- 结果标为角色模拟，不是真人数据，也不证明运行时任务完成。

### 9.4 Studio 与官方站收口（V-7、V-8）

```sh
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example theme-studio --release --base /tools/demo/
RUSTIFY_THEME_OLD_SITE="$TMPDIR/rustify-p5-old-studio" npx --no-install playwright test --project=theme-studio
npx --no-install playwright test --project=theme-studio-deep
for p in official-root official-subpath official-meta; do
  PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH" npx --no-install playwright test -c playwright.docs.config.ts --project="$p" official.spec.ts || break
done
```

成本依据：P4 记录 Studio 合并批 130 项约 14 分钟，官方站单项目 56 项约 6.5 分钟（含构建另约 2 分钟）。Studio 全项目与 deep 的总耗时未知，实测记录。每命令一个 project，串行。`theme-studio` 项目的 `theme-performance` 在 regression 层跳过，由 V-11 负责。既有 `actual Chrome 200%` 用例随 V-7 运行，是回归，不作真实 Chrome 结论；真实 Chrome 200% 由 V-10 负责。

收口后执行 `git checkout -- docs/validation/dom-components-p1 docs/validation/dom-components-p2 docs/validation/dom-components-p3 docs/validation/dom-components-p4` 还原 spec 写入的历史截图；`theme-gallery.spec.ts` 的输出路径在 M2 改到 `docs/validation/dom-components-p5/`。以 git diff 核实无残留。

UI 走查还需查看 Extensions 节浅深两模式、390/768/桌面截图；只看断言不算。环境失败先按现有配置排查；仍受阻写“未在浏览器中验证”、原因与所需协助。

### 9.5 静态与 host 回归（V-5）

与 `.github/workflows/verify.yml` 相关入口一致，已通过且本阶段未改的局部检查直接引用。

```sh
cargo fmt --all -- --check
cargo fmt --manifest-path site/Cargo.toml --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
for package in rustify-dom rustify-components; do
  mbx test --locked -p "$package" --no-default-features --features ssr --lib
  mbx clippy --locked -p "$package" --no-default-features --features ssr --all-targets -- -D warnings
done
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib
mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg --test menus_ssg --test panels_ssg --test content_ssg --test feedback_ssg --test ssr_controls --test missing_context_ssr
mbx test --locked -p rustify-docs
mbx clippy --locked -p rustify-docs --all-targets -- -D warnings
mbx clippy --locked --manifest-path site/Cargo.toml --no-default-features --features ssr --all-targets -- -D warnings
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
```

xtask 的 `docs_corpus` 用例需 Python 3.12 与 `RUSTIFY_DOCS_BIN=$PWD/target/docs-cli/rustify-docs`。CI：`verify.yml` 的 SSG `--test` 列表加 `extensions_ssg`；Studio spec 由 `theme-*.spec.ts` glob 覆盖；evidence 用例不进 regression 层。不新增流水线。

## 10. 里程碑与验收安排

用户收窄后共 4 个里程碑（少于 PRD 建议的 5—8 个）：扩展契约、Studio 陈列、迁移事实、统一收口。M1—M3 各一批，按 M1 → M2 → M3 → M4 串行，M4 是唯一阶段收口。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 扩展组件统一契约 | C-1 | §5.1—§5.7：`i18n.rs`、`data_table/view.rs`、`tree.rs`、`workspace/{panel_tabs,splitter,command_palette}.rs`、`file_picker.rs`、`drop_zone.rs`、`icon.rs`、★`crates/rustify-components/tests/extensions_ssg.rs`；`mbx xtask css` 重新生成；同构参考 `drop_zone.rs` 的 Message 用法、`menus_ssg.rs` 的 SSG 断言、`menus.css` 的焦点形态；`verify.yml` 增测试名 | V-1 通过；回写「实施进度」 |
| M2 | Studio 扩展陈列与可检查 | M1 的 part、状态与新参数（V-1） | §6：`gallery_catalog.rs`、`gallery.rs`、★`gallery_extensions.rs`、`main.rs`、`app.css`、`README.md`；`theme-gallery.spec.ts`（计数、路径、新用例、Extensions 节 axe 扫描、选中与 Invalid 测量）、`gallery-matrix.ts`（CommandPalette 入口、内缩 outline 的背景、`invalid boundary`）；同构参考 `gallery.rs` 样本、`component-catalog` 与 hydration 调用。扩展节版式按 AGENTS.md 用 impeccable（`@implementation_route` 的明确例外）。主 agent 串行构建与浏览器 | V-2 通过；回写「实施进度」 |
| M3 | 迁移事实源与文档 | M1 的最终接口（V-1）；M2 退出（V-2）。不与 M2 并行：`catalog` 模块无条件编译，Studio 测试直读 `STABLE_REFERENCES`，两者又共用 `target/`，并行会让 V-2 的证据来自未定型的 SDK | §3、§5.8：`catalog.rs`、★`crates/rustify-components/src/catalog/legacy.rs`、★`crates/rustify-components/src/catalog/legacy_baseline.txt`、`docs/components.md`、迁移指南重整、`docs/{workspace,data,themes}.md`、`README.md`、`crates/rustify-components/README.md`（`=0.2.0` 改 `=0.2.1`）；同构参考 `STABLE_REFERENCES` 与 `inventory_tests`；基线提取方法写入记录，含 `semantic:` 的提取范围与手工键 | V-3、V-4 通过；回写「实施进度」 |
| M4 | 全部 AC 与整条终验收口 | V-1—V-4；最终产物齐备 | 先按 §9.3 建基线产物；交付 ★`tests/native-zoom.ts`、★`tests/browser/theme-zoom.spec.ts`（真实 200% 与减少动效各一个用例，均带 `@evidence`）、`official.spec.ts` 的 `native 200%` 用例（`@evidence`）并修正 V6 过期断言；再按 §9 依次运行 V-5—V-13，串行；修复范围内验收缺陷（同因修复后只复验受影响项）；整理“AC → 证据 → 豁免”表与 missing；清理基线 worktree 与临时目录 | V-5—V-13 通过；11 条 AC 与 8 条整条终验按 §9.1 口径可追溯；回写「实施进度」 |

提交点：**M4（全部里程碑完成后的最终提交）**。M1—M3 在 M4 前完成时写待提交；只暂存本计划相关改动，不纳入已有评审报告、台账与 `skills-lock.json` 的既存未提交内容。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| C-7 收窄后，Studio 与官方站之外的运行时回归无法被发现（PanelTabs 结构、Tree 图标、DropZone 角色会影响 property-workbench、component-catalog 等） | 示例可能在运行时出错但编译通过 | 用户已接受；V-6 只保证编译。后续如需，可单独运行 `--project=property-workbench p2-workspace.spec.ts`、`--project=component-catalog catalog-inputs.spec.ts`、`--project=data-workbench p3-table.spec.ts` | 无 | 否 |
| C-6：旧主题路径长期并存 | WORK-F23/AC-2 该子句不成立，PRD 原文未改 | 负责人决定是否改 PRD；覆盖核查会把它列为变更 | M4 报告 | 否 |
| PRD Q3：首个含 v2 的 registry 版本未定 | 接入文档只能写 source checkout | 发行计划负责 | 对外发行前 | 否 |
| VoiceOver、OS IME、物理触控可能无法自动化 | WORK-N1/AC-3 只能列未验证 | V-12 尝试；负责人可按记录中的步骤清单确认（DOCS-SSG-P4 有用户本人确认 VoiceOver 的先例） | M4 | 否，AC 允许 |
| 新旧 Studio 主题编辑 p95 可能仍高于旧站（P1 已记录 26.4 对 15.4 ms） | WORK-N2 只能如实报告 | 记录成本，不调优（无毫秒 SLO） | M4 | 否 |
| 官方站在 path 依赖下随 SDK 变化（Tree 图标、DataTable 属性） | official.spec 可能失败 | V-8 发现后按本方缺陷修 | M4 | 否 |
| 基线 worktree 旧 spec 与当前 Playwright、Chrome 不兼容 | V-11 新旧对比缺一侧 | 记未运行及原因，保留尺寸数据与 P1 配对记录 | M4 | 否 |

- 待用户拍板：无。三项用户决定已写入 C-6、C-7、C-8。
- 最终状态：**Ready**。
- 定级理由：11 条 AC 均有设计落点、检查与收窄口径；接口、默认值、所有权、结构变化已定；全部开发对象在本仓授权范围，无外部依赖；范围相关问题已由用户决定。Ready 不表示实现完成或评审通过。

## 12. 已知坑与历史教训

- `outline-none` 与 `outline-2` 同元素时样式仍为 none；必须用 `focus-visible:outline-solid` 恢复（Tailwind 4 的 `--tw-outline-style`）。
- 选中背景上的 ring 对比不足，统一 ring 会在选中行失败；因此选中态用 `outline-current`（§5.1）。
- grid 与 listbox 的子元素有角色约束；空状态放普通段落会触发 axe `aria-required-children`（§5.3、§5.4）。
- ARIA 只放在允许的角色上：P4 代码评审 R1 的两条 ARIA 问题即来自无角色元素（`docs/plan/DOM-COMPONENTS-P4.records/M7.md`「评审修复」）。
- 目录不同步是真实缺陷：`catalog --check` 只证明源与生成物一致，不证明与交付一致（P4 评审 G-01）。限制文字若含 `transition` 等词会被当成 utility 候选写入 `rustify.css`。
- host `--lib --bins` 不覆盖 `cfg(wasm32)` 代码；示例的 wasm 分支必须用 wasm 目标检查（AGENTS.md「目标拆分」）。
- 改 class 后先 `mbx xtask css` 再 check 并提交 `rustify.css`、`candidates.json`（AGENTS.md）。
- `target/` 由 mbx 管理、可能被清理；证据放 `docs/validation/dom-components-p5/`，浏览器长跑前确认产物仍在；基线产物复制到 `$TMPDIR`。
- `serve.sh` 与 `docs-verify --group features` 调用 `python3` 且需 3.12；系统版本 3.9 无 `tomllib`，PATH 前置 `/opt/homebrew/opt/python@3.12/libexec/bin`。
- zsh 中 `"$SHA:path"` 会被当成修饰符，写 `"${SHA}:path"`。
- serve 不重建，源码改动后先重建对应产物；Playwright `reuseExistingServer: false`，端口占用要先查。
- Studio spec 会写入已入库的 P1—P4 验证目录，收口后按 §9.4 还原（P4 评审 F-23 同类）。
- `theme-gallery.spec.ts` 对带 `Empty` 标签的样本要求出现 `Restore` 按钮；新样本配真实恢复动作，不要为过断言删掉 Empty 控件（§6）。
- 矩阵的焦点边默认取父元素背景，代表名单之外的样本在 regression 层不进矩阵；内缩 outline 与选中态要按 §6 测，否则对比结果不代表实际画面。
- 新组件进 `gallery_catalog::ENTRIES` 后，`theme-gallery.spec.ts` 的计数、代表名单与 `floatingParts` 要同步；`sample()` 未分派的名字会 `unreachable!`。
- 选择 `-g` 时核对标题真实存在，避免空跑当通过（P4 评审 F-10）。
- CATALOG 与官方站、hydration 断言固定 24 类；扩展进 Studio 的 Section，不新增 Category。
- 内部 ssr 命名只服务 SSG 构建期；`extensions_ssg` 不追加请求时 SSR 验收（AGENTS.md「仅 CSR/SSG」）。
- 里程碑记录只存行为、有效证据与未完项；按仓内 SDLC v3 同步最小事件与当前页，不复制报告（`.xgent-ai/sdlc/protocol.md`）。
