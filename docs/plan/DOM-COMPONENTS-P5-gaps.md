# DOM-COMPONENTS-P5-gaps · Rustify UI · 迁移事实源与扩展验收补缺（DOM 公共契约 · Studio 真实样本 · 终验证据闭环）

```formal
INTENT:
  GOAL: "直接依据本计划、仓库规则和当前代码实施或续做"
  SUCCESS: "全部里程碑退出检查通过，按‘实施进度’完成最终提交、SHA 回填与自查，并提示用户发起代码评审"

RULE @implementation_entry:
  MUST: "实施须知：先读下方‘实施者定位’‘实施进度’，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；goal 执行也遵循这些约定"
```

> **计划状态：Ready**
>
> 调查基线：2026-11-10 · `f12d5220b014bc7c7baa9679f38afb2388ac749b` · main / dirty；产品代码与测试无未提交改动，既存 SDLC/R1/R2 材料及新增差异报告不属于本计划已交付成果。
>
> 需求来源：[DOM-COMPONENTS PRD v0.2](../PRD-DOM-COMPONENTS.md) · P5（第 5 期）；补齐 [DOM-COMPONENTS-P5](DOM-COMPONENTS-P5.md)，差异与决定以 [P5 PRD 差异报告](DOM-COMPONENTS-P5.prd-dev-gaps.md) 的 G-01—G-06、U-01/U-02、D-01—D-03 为准。
>
> 本期交付：**补齐主题旧参数、旧方法身份与 chart 颜色语义的迁移去向，修正 DropZone 错误语义、Studio 局部 PanelTabs 真实关闭和 Table 目录事实，并完成保留范围内的 Studio/GPU 行为补证与 P1—P4 性能证据索引**。
> **开发范围：本仓 `rustify-components` 的目录事实与 DropZone、theme-studio 的 Extensions 样本、既有 SSG/Playwright 场景和当前公开文档；依据为上述 PRD 与已确认差异。**
> 本期独特职责：只修补已经确认的 P5 交付缺口，不重做原里程碑，不以计划/代码复审完成替代 PRD 行为覆盖。
> **顶层排除：不恢复 PanelTabs 独立聚焦关闭按钮，不调整窗口化/拖放/浮层算法，不新增 GPU 组件或 SSR 能力，不重跑 C-7 豁免性能项，不修改原 PRD、原计划或历史完成记录。**

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
  MUST: ["需求/约束/假设 ID 只在账本登记一次，上游 ID 保留原编号及来源；检查、里程碑、任务 ID 分别在验证表、里程碑表、按需内部任务表定义，其他位置仅引用或展开；单处定义不限制复用，同一检查可被多个里程碑引用、批末只运行一次",
         "修订保留已有 ID，删除/拆分/合并同步引用",
         =>[task_table_present, "按交付物、前置成果与检查 ID 推进，进展及证据写入所属里程碑记录，不另设任务状态表"]]

RULE @milestone_completion:
  MUST: ["本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口",
         "同批里程碑共用 §9 检查：批末运行一次，通过后一并记完成；批末前已交付的里程碑保持进行中，不单独补验；里程碑和 commit 仍各自独立",
         "最后的收口须通过 §9 真实使用验收；测试全过或代码评审通过不等于能用，汇报进度先说真实场景结果，不以测试通过数衡量",
         "阶段终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收"]

RULE @role_acceptance:
  MUST: ["Agent 按 §9 指定的知识、资料及权限约束模拟上手与业务场景，记录实际结果与卡点",
         "真实使用验收从干净的用户状态出发，只凭公开入口与文档，用真实或有代表性的输入连续完成完整任务，亲自查看产物，不只看退出码、回执或断言",
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
         "优先扩展 §9 的场景 E2E，用真实入口和真实运行栈证明行为；局部测试只写 §9 已列的，或 E2E 暴露的缺陷确需窄测试防复发时补最少用例",
         =>[existing_local_test_fails_after_feature_change, "先判断是否真实回归；行为有意改变或断言绑定内部结构时删除或改到当前行为，在记录写一句"],
         =>[user_visible_problem_found, "修复，或列给用户确认延期并记入当前阻塞；不记为观察后继续完成"]]
  MUST_NOT: ["为假想故障、内部契约或框架已保证的行为加防御、回退或测试",
             "为正文未列为本期的 NFR 做基线、压测、专项验证、调优迭代或引入专门基础设施",
             "用 fallback 或静默恢复掩盖编程错误",
             "为覆盖率、逐个模块、逐个函数或穷举非法输入批量写测试",
             "为本方代码新造 fixture、mock 或内存替身；为保住旧测试保留兼容层或绕开设计",
             "把已知产品缺陷写成测试预期或例外",
             "另建验收框架、认证流水线或证据体系；E2E 用项目已有工具或短脚本照用户操作跑"]

WORKFLOW @implementation_sequence:
  TRIGGER: on_implementation_start_or_resume
  DO: ["按 @resume_protocol 恢复，核对仓库规则、当前任务相关正文与依赖、工作树",
       "按 §10 连续实施批次推进，批次可含多个里程碑，不按文件、函数、一次编辑或一轮 goal 对话切批",
       =>[local_test_admitted, "用当前行为的最小用例先红、再绿、再重构，红绿循环不附带整组回归"],
       "完成本批相关实现，再按 @verification_rhythm 与 @reverification 跑批末检查；场景 E2E 失败就是红灯，定位修复后只复跑失败场景及波及项",
       "批次通过后继续下一可执行批次，里程碑完成按 @progress_writeback 回写，汇报或回答状态询问后继续实施"]

RULE @implementation_stop:
  WHEN: user_explicitly_pauses_cancels_or_switches_task
  MUST: "按 @writeback_timing 保存后停止"

RULE @verification_rhythm:
  MUST: ["批内只为具体失败、技术疑问或必要前置运行能回答它的最小测试/编译检查，无触发则继续实现",
         "批末运行本批场景 E2E、§9 准入的局部测试和改动范围的静态检查；已通过且未受影响的场景不复跑，改动共享入口时加跑主路径",
         "不跑受影响模块或整包的全部已有测试，不因命令叫‘局部/快速’就逐次编辑重跑",
         "全量回归及跨平台矩阵、发布包与干净构建、故障注入、评测、成批真实会话仅在指定阶段收口运行，同环境走查合并",
         "下游前置、数据、安全、不可逆操作、发布及用户分期门及时验证，不等无关终验；范围外依赖未就绪时可在外部边界用 fixture 开发，不能替代真实集成",
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
  MUST: ["按需建 <计划名>.records/M<n>.md，同批共用检查的证据只写批末里程碑记录，其他里程碑链接引用；头部写计划链接、更新时区、状态与验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）",
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
         "同批里程碑可在各自交付后分别提交并回填各自行，不等批末检查；提交只保存成果，不代表验收通过，批末失败用后续提交修复",
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

- 最近更新：2026-11-10T01:44:26+08:00
- 当前进度：2/2 个里程碑完成
- 当前状态：M1、M2 均已完成（V-1—V-5 全部通过）；待收口提交与回填 SHA
- 最近完成：M2 · Studio/GPU 行为补证与终验索引——扩展两组既有浏览器场景覆盖 §6 四段任务，V-2 两命令全过、V-4 消费者走查 11/11、M2 记录补 P1—P4 性能证据索引与 C-7 豁免边界，详见 [M2 记录](DOM-COMPONENTS-P5-gaps.records/M2.md)
- 下一步：收口 · 最终提交（只暂存本计划列明路径）、回填 M1/M2 行短 SHA、单独提交计划文件更新、同步 SDLC 台账，随后报告完成并提示用户 review-code
- 当前阻塞：无
- 代码基线：`f12d5220b014bc7c7baa9679f38afb2388ac749b`

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-10T00:43:51+08:00 | 公共事实与真实样本修正：基线 675→693 键与 8 条 fate 行、DropZone aria-invalid、局部 PanelTabs controlled close、DataTable 激活输出、Table/chart 文档与 catalog 再生成；V-1、V-3 通过 | [M1 记录](DOM-COMPONENTS-P5-gaps.records/M1.md) | 待提交 |
| M2 | 已完成 | 2026-11-10T01:44:26+08:00 | Studio/GPU 行为补证与终验索引：扩展 gallery/acceptance 既有场景覆盖 §6 四段（键盘/真实文件/拖拽/GPU loss 保存导出）、V-2 两命令通过、V-4 消费者走查 11/11、P1—P4 性能证据索引与 C-7 豁免边界 | [M2 记录](DOM-COMPONENTS-P5-gaps.records/M2.md) | 待提交 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| G-01 | 功能/迁移 | PRD WORK-F23/AC-2、WORK-N5/AC-2；差异报告 | 四个主题组件的 10 个旧参数均有正确 fate 和指南去向 | §3；V-1 | 已确认 |
| G-02 | 功能/可访问性 | PRD WORK-F22/AC-1、WORK-N1/AC-3、WORK-N5/AC-2 | DropZone invalid 同时有视觉、数据和 ARIA 状态，指南与实现一致 | §5、§6；V-1/V-2 | 已确认 |
| G-03 | 功能 | PRD WORK-F22/AC-1、WORK-F19/AC-3、§2.1 | 局部主题样本的 PanelTabs 关闭是真实 controlled 操作 | §6；V-2/V-4 | 已确认 |
| G-04 | 文档事实 | PRD WORK-N5/AC-2、UAT-13 | 公开目录不再把已交付 Table 写成未实现 | §6；V-3/V-4 | 已确认 |
| G-05 | NFR 证据 | PRD WORK-N2/AC-2；原计划性能终验节 | P1—P4 既有性能证据进入终验索引，不重测豁免项 | §7；V-5 | 已确认 |
| G-06 | 已确认变更 | 差异报告；PRD WORK-F22/AC-1、WORK-N1 | 保留可及树外鼠标 close 与 Delete 键，不恢复独立聚焦按钮 | §0.5；无需新增验证 | 保留 |
| G-07 | 功能/迁移 | PRD WORK-F23/AC-2、WORK-N5/AC-2；差异报告 | 旧方法按接收者独立登记，ThemeDocument 替代和 ThemeFont const 方法有真实去向 | §3；V-1 | 已确认 |
| G-08 | 功能/迁移 | PRD WORK-F23/AC-2、WORK-N5/AC-2、UAT-13；差异报告 | chart-1..5 及既有回退关系逐项标 Kept 并可查 | §3；V-1/V-3 | 已确认 |
| U-01 | 行为补证 | PRD WORK-F22/AC-1；原计划 §6 | Studio 最终入口的键盘、真实文件、指针与禁用命令行为可观察 | §6；V-2 | 已确定 |
| U-02 | 行为补证 | PRD WORK-F19/AC-3 | GPU 局部故障未恢复时保存、导出和 DOM 编辑仍可用 | §6；V-2 | 已确定 |
| C-1 | 约束 | 仓库 `AGENTS.md`；`mbx`/`cargo fmt`/Playwright 约定 | Rust 检查用 mbx，格式化用 cargo fmt；浏览器每次一个 project、使用最新 release | §9 | 已确认 |
| C-6 | 历史决定 | 差异报告 D-01 | 旧主题路径无删除期限地保留，但仍须登记旧参数去向 | §0.5、§3 | 已确认 |
| C-7 | 历史决定 | 差异报告 D-02 | 原示例、扩展性能前后、P2—P4 最终重测及列明消费者浏览器范围豁免 | §0.5、§7、§9 | 已确认 |
| C-8 | 历史决定 | 差异报告 D-03 | 完整 SSG 接入、演示构建及对应消费验收不进入本期 | §0.5 | 已确认 |
| C-3 | 约束 | 本次核查边界 | 原 PRD、原计划、原完成记录只读；新证据写本计划记录 | §1.2、§10 | 已确认 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 交付切片 | M1 修公共事实与真实样本，M2 在同一最终产物上补行为验收和证据索引 | M2 依赖 M1 的 DropZone/PanelTabs 终态；不并行改共享文件 | G-01—G-05、G-07、G-08、U-01/U-02 |
| D2 | PanelTabs 关闭复用 | 局部样本沿用主 `WorkspaceSample` 的 controlled 更新规则，不新增抽象 | 两个调用点保留就地逻辑；永久项与邻接选择一致 | `gallery_extensions.rs` 既有实现 |
| D3 | 历史记录 | 不改写原第三/第四里程碑的完成事实；在 M2 新记录补正确文档结果和性能索引 | 保留历史，同时让当前终验可追溯 | C-3、G-04/G-05 |
| D4 | 故障验证 | 只扩展既有 `WEBGL_lose_context` 场景，不增加故障恢复机制 | 验证局部故障与 fatal 的边界，不改 GPU 运行时代码 | U-02；现有 `theme-acceptance.spec.ts` |

### 0.4 职责与事实所有权

|  | SDK / 目录 | Studio 样本与验收 | 历史性能证据 |
| --- | --- | --- | --- |
| 拥有 | 旧参数/方法身份与 fate、颜色语义、DropZone ARIA、生成目录与当前公开事实 | controlled tabs 状态、真实 UI 操作、故障期间用户结果 | 各自原基线、负载、测量记录 |
| 不拥有 | **不改变组件算法、旧主题保留决定或发布版本** | **不新增产品能力、不改 GPU runtime 或浮层实现** | **不改历史数值、不把豁免项写成通过** |

一句话：当前实现与文档由本计划修正，历史证据只引用和校准索引，不重写来源。

### 0.5 明确不在本期

- **PanelTabs 旧结构恢复**——G-06 已判合理变更；保留可及树外鼠标 close 与 Delete 键，不恢复独立聚焦按钮。
- **组件算法与 GPU/SSR 新能力**——窗口化、Tree 深度、拖放算法、浮层栈、Makepad、请求时 SSR 均零改动。
- **C-7/C-8 豁免范围**——不补扩展组件前后性能、P2—P4 最终代码重测、原示例/独立消费者浏览器矩阵、完整 SSG 接入和演示构建。
- **VoiceOver、OS IME、物理触控**——继续按 PRD AC-3 列 missing；不借本计划宣称通过。
- **新性能调优或预算**——只整理既有证据；实测暴露的独立性能缺陷须另行确认范围，不在本计划顺带优化。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **已核实·缺口**：675 键基线漏列四个主题组件的 10 个旧参数；通用解析测试不能发现未列入的键 → 增加明确覆盖断言并补齐 §3 事实源（`catalog/legacy*`、`catalog.rs::inventory_tests`）。
- **已核实·缺口**：旧方法键折叠接收者，ThemeDocument 两个旧方法误标 Kept，ThemeFont 两个 const 方法漏列且产生 `fn` 伪键；chart-1..5 被无依据排除 → 按 §3.1/§3.2 修正身份、fate 和颜色映射。
- **已核实·缺口**：DropZone 无 `aria-invalid`，而 FilePicker 和迁移指南已有对应契约 → 复用 FilePicker 的 reactive 属性写法（`drop_zone.rs`、`file_picker.rs`）。
- **已核实·缺口**：`ExtensionLocalSample` 的 `on_close` 为空；主样本已有正确 controlled close → 按 D2 接通，不改 `PanelTabs` 组件。
- **已核实·缺口**：`docs/components.md` 手写 Source boundaries 与 `table.rs`、CATALOG、Studio Table 样本冲突 → 只修当前公开文档，不为静态文本新增自动化测试。
- **已核实·缺口**：P5 性能终验记录只链接 P1；P2/P3/P4 的 baseline/current 与记录仍存在 → M2 建立完整索引，不重跑。
- **已核实·足够**：现有 `theme-gallery.spec.ts` 与 `theme-acceptance.spec.ts` 提供真实 release、GPU loss、保存和导出操作原语，可直接扩展，无需新 fixture 或测试框架。

### 1.2 拓扑与文件清单

- `crates/rustify-components/src/catalog/legacy_baseline.txt`、`catalog/legacy.rs`：冻结键与 fate；`catalog.rs` 同步修订生成目录的基线来源说明；本次新增目录断言全部并入既有 `inventory_tests::legacy_baseline_keys_resolve_exactly_once` 函数体。
- `crates/rustify-components/src/drop_zone.rs`、`crates/rustify-components/tests/extensions_ssg.rs`：ARIA 属性及公开 SSG 渲染断言。
- `examples/theme-studio/src/gallery_extensions.rs`：局部 PanelTabs controlled 行为；主样本是同构实现。`DataTableSample` 按同文件 `CommandSample` 的 `output` 模式显示被激活的行，只补真实操作结果，不新增编辑器或业务动作。
- `tests/browser/theme-gallery.spec.ts`、`tests/browser/theme-acceptance.spec.ts`：扩展现有场景，不新增 spec、fixture 或端口。
- `docs/components.md`：由 catalog 生成 Legacy 节；Source boundaries 为手写修正。`docs/dom-components-migration.md` 补 chart 回退及重提时发现的真实方法去向。不预期新增 Tailwind class；候选与生成 CSS 是否漂移由 V-3 的 `css --check` 确认。
- ★ `docs/plan/DOM-COMPONENTS-P5-gaps.records/M1.md`、`M2.md`：实际实施与验收时按需创建；初稿不预建。
- 零改动：`crates/rustify-dom`、Makepad、组件算法、原 P5 计划及其 `records/`、性能 JSON、依赖和工具链。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| catalog legacy | 迁移指南、生成 `docs/components.md` | 冻结键排序唯一、每键恰一解析；Changed 不被导出通配项吞掉 | 既有 `LegacyEntry`/`catalogue_markdown`，无新接缝 | 基线完整性与 fate | 仓内数据 | V-1 |
| DropZone | 应用文件导入、Studio | `invalid` 同时驱动视觉、`data-invalid`、`aria-invalid`；described-by 保持 | 复用现有 props，无新 API | 状态优先级与可访问语义 | 仓内组件 | V-1/V-2 |
| ExtensionLocalSample | Studio Components 预览 | tabs/active 由应用持有；close 请求更新两者，永久项由组件阻止关闭 | 复用 `PanelTabs` 回调 | 关闭活动/非活动项的选择结果 | 仓内应用 | V-2/V-4 |
| 既有 Playwright 场景 | P5 终验 | 同一 release、真实 UI、局部 GPU 故障与 fatal 分开 | 既有 helper/测试入口 | 等待条件与真实状态观察 | 本地浏览器 | V-2/V-4 |

## 3. 迁移事实源契约

在 `legacy_baseline.txt` 按既有字母序加入并映射：

| 组件 | 旧参数键 | fate | 去向与理由 |
| --- | --- | --- | --- |
| ThemedScope | `prop:ThemedScope.theme` | Kept | 同名 `Signal<Theme>`，旧路径按 C-6 保留 |
| ThemeOverride | `prop:ThemeOverride.patch`、`.class`、`.test_id`、`.children` | Kept | 同名同义，链接主题接入/迁移指南 |
| ThemeScope | `prop:ThemeScope.resolved` | Kept | 同名 `Signal<Arc<ResolvedTheme>>`；后续新增 preview_mode 不冒充旧键 |
| ThemeBoundary | `prop:ThemeBoundary.class`、`.test_id`、`.children` | Kept | 同名同义 |
| ThemeBoundary | `prop:ThemeBoundary.patch` | Changed | `Signal<ThemeValuePatch>` → `Signal<ThemeBoundaryPatch>`；双模式/config 来源及 `ThemeValuePatch.into()` 适配链接到现有迁移指南对应段落 |

`LEGACY_ENTRIES` 可用一条 `names` 列多个精确键；不得再用 `export:*` 的 Kept 结论代替这些 prop 的 fate。本节及 §3.1/§3.2 的新增断言全部并入既有 `catalog::inventory_tests::legacy_baseline_keys_resolve_exactly_once` 函数体，使上述 10 键缺席或 `ThemeBoundary.patch` 不是 Changed 时失败；同时要求每个非通配 `LEGACY_ENTRIES.names` 精确名存在于基线键集合，避免孤儿行进入公开目录。这是本次事实源补齐的窄回归，不建立通用源码解析器。生成目录的来源说明同步写明：基线以 `11b69129` 机械提取，并按旧签名核对补全主题参数、接收者限定方法及 chart 颜色语义。

### 3.1 旧方法身份与 fate

基线中来自 `impl Type` 的公共方法统一使用 `export:<file>::<Type>::<method>`；自由函数、类型、常量保持原格式。以接收者限定键替换旧折叠键，不保留会重复计算旧身份的伪别名；`pub const fn` 与 `pub fn` 同样纳入。现有文件级 `::*` 仍可覆盖未单独列出的 Kept 方法，精确 Changed/Replaced/Removed 行继续优先。

- 至少把旧 `export:catalog.rs::name` 拆为 `Category::name`、`Support::name`，把旧 `export:theme.rs::properties` 拆为 `Theme::properties`、`ThemePatch::properties`；重提后按接收者核对全部方法，而非只搜当前同名裸方法。
- `export:document.rs::ThemeDocument::values` 与 `values_mut` 均为 **Replaced**：分别指向当前 `base_values`/`base_values_mut`，需要组合值时指向 `effective_values`，指南使用现有“JSON, CSS and spacing changes”段。不得因当前 `ThemeOverrides::values` 同名而标 Kept。
- 新增 `export:fonts.rs::ThemeFont::face`、`slot`，均为 **Kept**；删除不存在的 `export:fonts.rs::fn`。
- 若重提发现其他旧方法没有同接收者同义去向，按真实行为标 Changed/Replaced/Removed，并在迁移指南补对应当前入口；不修改产品实现来迁就旧表。若只有新的产品取舍才能决定去向，则把对应项记入当前阻塞并请求决定，不默选。

### 3.2 chart 颜色语义

新增 `semantic:chart-1`、`chart-2`、`chart-3`、`chart-4`、`chart-5`，均为 **Kept**。destination 为当前同名主题 token；reason 分别记录 `--primary`、`--success`、`--warning`、`--destructive`、`--muted-foreground` 的既有回退。迁移指南“Default palette and color semantics”补这组 token 与回退关系，不改 `sdk.css` 或颜色实现。

扩展 §3 的 inventory 断言覆盖本节已知键、伪键移除、两个 Replaced 和五个 Kept；这些是已暴露数据完整性缺陷的窄回归，不为 Markdown 文本另建测试。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| 基线主题组件 | 已核实缺口 | `11b69129` 与当前 `rustify-dom/src/theme*.rs` 的公开签名 | 只读核对并登记，不改组件 | 事实不符则测试失败，实施者修正映射 |
| catalog/xtask | 已核实足够 | 既有生成器输出 `Legacy API destinations` | 复用并再生成 | 生成漂移即不通过 |
| theme-studio release | 已核实足够 | `build-web --example theme-studio --release` 与 `--project=theme-studio` | 所有浏览器证据使用同一最新产物 | 构建/页面失败则场景红灯 |
| GPU fault / fatal | 已核实足够 | 既有 `WEBGL_lose_context`、fatal 注入和诊断 | 只扩局部故障保存/导出；fatal 用例不动 | 不增加重试或恢复逻辑 |
| P1—P4 性能材料 | 已核实足够 | 各期 baseline/current JSON 与对应收口记录 | 只读引用，保留原负载和口径 | 链接或范围不清则 V-5 不通过 |

无外部服务、账号、权限、配置、种子或新增 scope 前置。

## 5. 核心机制

1. 先让 §3—§3.2 的参数、方法、颜色回归断言和 DropZone 的 `aria-invalid` 公开渲染断言在当前代码上失败，证明测试能发现本次缺口；再实施最小修复。
2. DropZone 仅在根元素增加 `aria-invalid=move || invalid.get().then_some("true")`；不改变 drag/drop、disabled 视觉优先或理由关联。
3. 局部 PanelTabs 收到 close 请求时，按主样本规则：非活动项只移除；活动项优先选右邻、否则左邻；随后移除请求项。`permanent()` 不出 close 且 Delete 不删除，沿用组件现有边界。
4. Table 文档只删除过期的“未实现”声明，继续说明普通语义 Table 与虚拟化交互 DataTable 是不同能力。
5. 不新增运行期错误分支、恢复状态或数据迁移；已有错误语义保持不变。

## 6. 前端与交互

- **信息架构与角色视图**：仍从 Studio → Components → Rustify extensions 进入；普通开发者从公开 `docs/components.md`/迁移指南查 Table 和主题参数，不需读内部实现。
- **状态**：DropZone invalid 同时可看见并可被辅助技术读取；disabled 继续优先于错误视觉，described-by 理由保持可见。PanelTabs 空/活动/永久规则不变。
- **可访问性、响应式、主题、i18n**：沿用现有颜色、布局与 locale，文案仅补 DataTable 激活结果；复用现有 axe 与 locale 证据，只有本批触及的扩展行为/axe 场景在 V-2 复验。
- **真实浏览器走查**：在 V-2 同一 release 上完成下述连续任务：
  1. DataTable 键盘移动/选择、Enter/F2 激活与局部滚轮；激活后样本 `output` 显示当前行，作为打开请求已交付应用的可观察结果。Tree ArrowRight/ArrowLeft/Enter；CommandPalette ArrowDown/Enter 选择可运行项、不可运行项 Enter 不产生动作。
  2. FilePicker 选择真实临时 `.json`/`.txt` 文件，DropZone 从 DOM 真实 dispatch 带 File 的 drop；观察应用 output 文件名，invalid 时观察 DropZone ARIA 和可见理由。
  3. Splitter 指针拖动改变 `sizes`/CSSOM 列宽；主与局部 PanelTabs 分别验证非活动关闭、活动关闭、Delete 和永久项边界。
  4. GPU `lost` 尚未恢复时保存一个命名主题、打开导出并全选当前草稿，再改 DOM 颜色；保存列表、导出选择范围和预览值均可观察。fatal 恢复仍引用独立既有用例，不混入此断言。
- 前端无权限模型；真实文件只进入既有浏览器测试页，不执行应用外业务操作。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| G-05 / WORK-N2-AC2 | P1 `performance-after.json`；P2/P3/P4 各自 baseline/current 与收口记录 | 终验能追溯受影响热路径的既有 p50/p95、尺寸与资源口径；不设新预算 | 证据缺失则如实列缺口，不重跑 C-7 豁免项 | M2 记录直接链接来源；Studio 的“240 输入→发布”标为完成发布次数 | V-5 |

本计划无新权限、数据、外部服务或安全边界；无新增遥测、告警或运行基础设施。

## 8. 失败模式、发布与回滚

不引入新的运行期失败模式：本批只补属性、受控样本、事实源和验证，既有文件导入、GPU 故障和存储失败语义不变。无需事务、重试、降级或恢复机制。

- 发布顺序：无发布动作；最终提交不代表 registry 或部署生效。
- 回滚：若本批修正导致回归，按普通 Git 提交回退本批增量；无持久数据、schema、外部契约或不可逆点。
- 完成门：M1/M2 各自退出检查通过，且 M2 真实使用验收完成；不能用历史全绿替代当前增量结果。

## 9. 验证

### 9.1 行为与接口验证

| 检查 ID | 层级 | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- | --- |
| V-1 | 局部契约 | G-01/G-02/G-07/G-08；主题参数、方法接收者/fate、const 方法、chart 五色、精确名无孤儿与 DropZone 公开 ARIA；所有新增目录断言均在既有单测试函数体内。准入：本次已暴露的漏列/误映射/语义缺陷需窄回归，且现有通用测试无法发现 | 先运行新增最小断言确认当前失败，再修复并运行：`RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib catalog::inventory_tests::legacy_baseline_keys_resolve_exactly_once`；`mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg`；`mbx xtask catalog --write docs/components.md --check` | M1 的 §3/§5 交付齐备后 |
| V-2 | 场景 E2E | G-02/G-03、U-01/U-02；§6 的四段连续真实用户任务，从 Studio 公开 UI 观察状态、文件名、输出、保存和导出 | `mbx xtask build-web --example theme-studio --release`；随后串行运行 `npx --no-install playwright test --project=theme-studio theme-gallery.spec.ts -g 'extension samples'` 与 `npx --no-install playwright test --project=theme-studio theme-acceptance.spec.ts -g 'DOM edits remain usable'`；每命令一个 project，不与其他 Playwright 并行 | M2 的样本和测试改动齐备、M1 已通过 V-1 后 |
| V-3 | 静态/文档 | G-01—G-04、G-07、G-08；Rust 格式/ lint、Studio 样本 host 检查、Table/chart/旧方法当前文档与生成目录一致 | `cargo fmt --all -- --check`；`mbx clippy --locked -p rustify-components -p theme-studio --all-targets -- -D warnings`；`mbx test --locked -p theme-studio --bins`；人工回读 Table Source boundaries、基线机械提取后补全的来源说明、chart 回退与 ThemeDocument 迁移指南对应句；末尾运行一次 `mbx xtask css --check` | M1 全部交付齐备后；M2 只改 TS/记录时不因续做重跑 |
| V-4 | 真实使用验收 | G-03/G-04、U-01/U-02；Agent 模拟只掌握公开文档的组件消费者，从干净 Studio 会话定位 Table/扩展样本并完成 §6 关键任务，亲自查看实际页面与导出内容 | V-2 结束并释放服务后，用 `mbx xtask serve --example theme-studio --release --port 4181` 启动同一最新 release，Agent 经可用浏览器通道在同一 §6 场景上串行走查，不与 V-2 或其他 Playwright 运行并行；角色只读公开文档与 UI，不读内部源码补步骤。U-02 的 GPU loss 注入与故障期间结果引用 V-2 记录，不由消费者角色自行触发，不冒充 V-4 独立实测；记录实际卡点，发现用户可见问题须修复 | 最后收口；V-2 完成后执行，结束时停止本次 serve |
| V-5 | 证据收口 | G-05 与全部 G/U；P1—P4 链接真实、负载/基线/豁免清楚，进度和差异去向一致 | 回读各期记录与 JSON；`git diff --check`；按本计划「实施进度」协议核对链接、状态和提交 | M2 最后收口，真实使用验收后 |

批内最小反馈只用于当前失败：目录问题用 V-1 的单测试过滤器，ARIA 用 `extensions_ssg` 中对应用例名，浏览器问题用 V-2 的一个 `-g` 场景。不得把这些窄入口升级为每次编辑的整包/全项目回归。

### 9.2 集成与回归

只扩展既有 `theme-gallery.spec.ts` 和 `theme-acceptance.spec.ts`。DataTable/Tree 等公共组件算法未变，不重跑原示例或全量浏览器项目；R2 已通过的未改 host/SSG 路径按证据复用规则处理。V-2 若暴露本批造成的相邻回归，只补验受影响场景。

### 9.3 NFR 与故障注入

不重测性能，不新增毫秒目标。唯一故障注入是 V-2 既有 GPU loss 场景；fatal 注入属于原 P5 既有证据，本计划不重复运行，除非本批改动实际触及该路径。

### 9.4 真实环境与 UI

浏览器从公开 Studio 根入口开始，使用最新 release；检查 console/pageerror/requestfailed 和现有 CSP 监听。主题、语言、响应式套件未受本批布局/样式改动影响，不新增矩阵。环境受阻时在 M2 记录「未在浏览器中验证」、原因与所需协助，不以静态检查代替 V-2/V-4。

### 9.5 静态校验

静态命令集中在 V-1/V-3；依赖与工具链不变，不预期新增 Tailwind class；`legacy.rs` 位于 CSS 候选扫描树内，因此 V-3 末尾运行一次 `mbx xtask css --check` 确认候选与生成 CSS 无漂移。若目录叙述误入候选，优先改写叙述，不扩大 utility 范围。文档语义由 V-3 人工回读，不为静态 Markdown 文本新增自动化测试。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 公共事实与真实样本修正 | 差异报告 G-01—G-04、G-07、G-08；当前基线可读 | 完成 §3 的参数/方法/颜色键与 fate、窄回归、DropZone ARIA、局部 PanelTabs controlled close、DataTable 激活结果输出、Table/chart 文档与基线来源说明和 catalog 再生成；共享文件由同一实现者串行修改，不派并行 agent | V-1、V-3 全通过；回写「实施进度」 |
| M2 | Studio/GPU 行为补证与终验索引 | M1 已完成，V-1/V-3 证据有效 | 扩展 §6 既有浏览器场景；在同一 release 完成真实使用验收；在新 M2 记录补 P1—P4 性能索引、豁免边界和 G/U 去向，不改历史记录 | V-2、V-4、V-5 全通过；回写「实施进度」 |

提交点：M2 全部完成后最终提交一次，再按实施进度协议回填 SHA。只暂存本计划列明的代码、测试、文档、计划与记录，不纳入既存 SDLC/R1/R2 或其他用户改动。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 工作树存在既存评审/台账改动 | 提交可能混入非本批材料 | 实施者按 §1.2 和提交点逐路径暂存，不回退他人改动 | M1 开工及最终提交前 | 否 |
| 真实文件/GPU 浏览器环境失败 | U-01/U-02 不能记通过 | 实施者先按现有 Playwright 配置排查；仍受阻则记录缺证和协助事项 | M2 退出前 | 否；失败时该里程碑不得完成 |

- 最终状态：**Ready**。
- 定级理由：所有差异均有明确 PRD/用户决定来源和当前代码落点；无未决产品取舍、外部依赖、数据/安全/发布前置，验证均可由 Agent 使用本仓既有工具完成。

## 12. 已知坑与历史教训

- 通用“已列键恰一解析”不能证明冻结基线完整；本次用实际漏键、接收者折叠、误标 Kept、const 方法和 chart 排除建立最小回归，不造通用源码文本测试（差异报告 G-01/G-07/G-08）。
- `xtask catalog --check` 只保证生成区与 CATALOG 一致，手写 Source boundaries 必须人工回读（差异报告 G-04）。
- axe 通过不等于显式 invalid 状态存在；DropZone 需直接断言 ARIA，不能仅依赖自动扫描（差异报告 G-02）。
- controlled 组件显示值由应用决定；空 `on_close` 就是真实空操作，不能因有回调签名而记完成（`gallery_extensions.rs` 主/局部样本对照）。
- GPU 局部故障与实例 fatal 是不同验收边界；fatal 重启后的保存/导出不能替代 loss 期间的保存/导出（PRD WORK-F19/AC-3）。
- C-7 豁免重测不等于删除证据索引；历史性能数据保持原路径、原口径，不为了收口重新测量（原计划性能终验节、差异报告 G-05）。
