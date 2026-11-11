# DOM-COMPONENTS-RELEASE · Rustify UI · DOM 组件成果发行与官方站更新（十包 0.3.0 · registry 配对 · 源码 Git CLI 部署 Pages）

```formal
INTENT:
  GOAL: "直接依据本计划、仓库规则和当前代码实施或续做"
  SUCCESS: "全部里程碑退出检查通过，按‘实施进度’完成最终提交、SHA 回填与自查，并提示用户发起代码评审"

RULE @implementation_entry:
  MUST: "实施须知：先读下方‘实施者定位’‘实施进度’，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；goal 执行也遵循这些约定"
```

> **计划状态：Ready**
>
> 调查基线：2026-11-10 · `e243405029997be74ebc8e6683e05397fc8d8a36`（main，与 origin/main 相同）· dirty（本计划未跟踪＋SDLC 台账登记；代码事实取自 HEAD）。
>
> 需求来源：用户需求（2026-11-10）：「更新 rustify-docs 做的 github pages，展示最新的 docs/PRD-DOM-COMPONENTS.md 成果」；用户同日选定「先发 crates.io 新版再部署」「三篇指南都加入 Guides」。发行对象是 [DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) 在 P1–P5 计划中已交付的源码成果。本计划承接该 PRD §5.2 的 Q3（新 SDK/CLI 配对与破坏性发行版本），以及 §1.3 写明的「发行沿原流程」，不属于 PRD 期次。
>
> 本期交付：**crates.io 上的 Rustify 十个 Docs/DOM/样式包 0.3.0（新增 `rustify-colors`），含 P1–P5 的组件、主题 schema v2 与 Colors；切换到 0.3.0 的公开安装、迁移与最低版本说明；官方站 <https://rockie.github.io/rustify-ui/> 从发布后的提交重建，新增 Menus、Panels、迁移三篇指南。**
> **开发范围：发行清单与 `rustify-colors` 发布元数据、各包版本与内部精确钉、包内 README/模板、CLI 配对记录与模板锁、仓内测试 fixture 的版本与锁、`crates-io.yml` 与 runbook 的预取清单及包数措辞、AGENTS.md 的发行包数、官方站配置（`rustify-docs.toml`）及其 spec、`site/` 的版本钉与中文首页文案、仓根公开说明与 CHANGELOG；依据见 §0。**
> 本期独特职责：首次发布含破坏性 DOM API 和 schema v2 的 registry 版本；官方站首次以 0.2.x 之后的 source 分发 Git CLI 部署（此前线上一直用 legacy 0.1.x 的 Git CLI）。
> **顶层排除：GPU SDK（`rustify-ui`）/`rustify-makepad`/示例的 registry 发布；Theme Studio 部署或文案；rustify-docs 生成器、发行脚本或 Pages 工作流的行为改动（预取清单除外）；新增组件 demo 或中文翻译。**

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

- 最近更新：2026-11-11 · M4 完成（全部里程碑）
- 当前进度：4/4 个里程碑完成
- 当前状态：发行完成——十包 0.3.0 公开在 crates.io、Pages 上线（run `42953922752`、线上 build_id `3fe8d486f30d8aa5`）、E=`458333d5` 在公共 main；临时 token 撤销仍为发布人待办（见 M3 记录）
- 最近完成：M4 Pages 部署与真实使用验收——部署候选 D3=`596175e3`（差异门通过、V-11 预演全过）、四次派发授权后 run `42953922752` build＋deploy 成功、V-12 线上核验与 8 用例全过、V-13 四任务真实使用全过（V-6 卡点消除）、E=`458333d5` 记录官方站交付；详见 [M4 记录](DOM-COMPONENTS-RELEASE.records/M4.md)
- 下一步：代码评审——请以本计划路径发起 review-code（评审由用户执行）
- 当前阻塞：无（遗留待办仅发布人 token 撤销，见 M3 记录与 M4 观察项）
- 代码基线：`e243405029997be74ebc8e6683e05397fc8d8a36`

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-10 | 三篇指南进官方导航（A=`53ff0666`）；`rustify-colors` 入清单、十包 0.3.0、模板/锁/常量/预取与措辞齐备（R=`3f2fd1e1`）；V-1、V-2 全过 | [M1.md](DOM-COMPONENTS-RELEASE.records/M1.md) | A=`53ff0666`、R=`3f2fd1e1` |
| M2 | 已完成 | 2026-11-10 | §5.2 冻结与 V-3–V-7 全过（C=`2445aea6`、CLI 候选 `dde6fe85`、runner 修复 `6993fed9`；receipt 十包 0.3.0 冻结 1817 KB；远端预检 run 42833566509 双平台成功） | [M2.md](DOM-COMPONENTS-RELEASE.records/M2.md) | C=`2445aea6`、`dde6fe85`、`6993fed9` |
| M3 | 已完成 | 2026-11-11 | V-8–V-10 全过；十包 0.3.0 公开可取（checksum 全一致）；D=`8d3544bb` 推送公共 main；标签 rustify-libs-v0.3.0→R、rustify-docs-cli-v0.3.0→C 读回；token 撤销暂缓（用户确认） | [M3.md](DOM-COMPONENTS-RELEASE.records/M3.md) | D=`8d3544bb` |
| M4 | 已完成 | 2026-11-11 | 差异门与 V-11 预演（D 与 D3 两轮）通过；部署 run `42953922752`（build 28m39s＋deploy 9s）；V-12 全过（线上 manifest cli_version=0.3.0、build_id `3fe8d486f30d8aa5` 与 run 一致）；V-13 四任务全过（V-6 卡点消除）；E=`458333d5`；worktree 清理 | [M4.md](DOM-COMPONENTS-RELEASE.records/M4.md) | D3=`596175e3`、E=`458333d5` |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能 | 用户 2026-11-10：「更新 rustify-docs 做的 github pages，展示最新的 docs/PRD-DOM-COMPONENTS.md 成果」 | 线上官方站由发布后的提交重建，12 篇原有指南和 24 个组件页反映当前 SDK、组件、主题与迁移状态；读者只看站点就能知道这些能力已在 0.3.0 中可用 | §5.4、ADR-1；V-11、V-12、V-13 | 已确认 |
| R-2 | 功能 | 用户 2026-11-10 选定「先发 crates.io 新版再部署」 | 十个包以 0.3.0 公开发布，各包 checksum 与冻结归档一致，CLI 的 registry 配对指向 0.3.0 三包；Pages 在公开门通过后才部署 | §5.1–§5.3、D1–D3；V-4、V-7–V-10 | 已确认 |
| R-3 | 功能 | 用户 2026-11-10 选定「三篇都加入 Guides」 | `docs/menus.md`、`docs/panels.md`、`docs/dom-components-migration.md` 进入 `rustify-docs.toml` 的 include、英文与中文 Guides 导航；官方站 spec 的固定计数同步 | §1.2（A）、D5、§5.4；V-1、V-12、V-13 | 已确认 |
| C-1 | 约束 | [PRD §5.2 Q3](../PRD-DOM-COMPONENTS.md#52-待落实项) | 接入说明须写明新导出格式（主题 schema v2）的最低消费者版本 | §5.3 的 D 提交（themes.md、迁移指南、README）；V-10、V-13 | 已确认 |
| C-2 | 约束 | AGENTS.md「完成标准与发布」；[docs/crates-io-release.md](../crates-io-release.md) | 发行走 runbook 与 `scripts/crates-io.py`；每包归档绑定 source_rev；框架三包 docs/dom/components 版本同步移动；Python 3.12；构建一律 mbx | §5.1–§5.3；V-4、V-9 | 已确认 |
| C-3 | 约束 | docs/crates-io-release.md「Remote preflight」「Account and authorization gate」 | 推送 main + dispatch、上传（包名与版本）、公开说明与标签推送分别取得用户授权；临时 token 只记账号/权限/过期，不记值；结束后撤销 | §5.2、§5.3、§5.4、§8、§11；V-7、V-8、V-10、V-11（M4 派发门） | 已确认 |
| C-4 | 约束 | AGENTS.md「浏览器测试」 | Playwright 每条命令只带一个 `--project`，串行执行，不同时运行两个 | §9.4；V-1、V-5 | 已确认 |
| C-5 | 约束 | AGENTS.md「工具链与本地开发」 | 不新开分支、不升依赖 major；提交直接落 main | §10 提交点 | 已确认 |
| C-6 | 约束 | `.github/workflows/docs-pages.yml` | 只部署显式给出的完整候选 SHA；候选必须在 origin 上可检出；Pages 已用 Actions 源（[docs-ssg.md 官方站交付](../docs-ssg.md#official-site-delivery)） | ADR-1；V-11、V-12 | 已确认 |
| C-7 | 约束 | [PRD §1.3](../PRD-DOM-COMPONENTS.md#13-非目标) | 发行与文档沿原流程，不重做 Rustify Docs 的内容、搜索或发布系统 | §0.5；零改动面见 §1.2 | 已确认 |
| A-1 | 假设 | crates.io API 2026-11-10 对 `rustify-colors` 返回「does not exist」；其余九个名称由本仓库账号持有 0.2.1 | 上传时 `rustify-colors` 名称仍可用；发布人能提供含 `publish-new`（rustify-colors）与 `publish-update`（其余九包）、名称模式覆盖十包的凭证 | V-8 重查名称/owner 与凭证范围；责任人：发布人（用户）；最晚 M3 上传前 | 开放，只阻塞 M3 |
| A-2 | 假设 | Git 安装的 CLI 已核实报告 `distribution: source`（cargo checkout 含 `.git`，见 §1.1）；CI 中 `cli_identity` 在 `CARGO_HOME/git/checkouts` 下查找该 checkout | Pages 工作流在 CI 中能以 source 模式完成 prepare | V-11 本地预演；CI 中由 V-12 首次证实，失败路线见 §8；责任人：实施者；最晚 M4 | 开放，不阻塞 M1–M3 |
| A-3 | 假设 | DOM-COMPONENTS-P5 按其 C-7 未运行 `dom-*`、`components-*`、`docs-*` 浏览器项目（[P5 §0.5](DOM-COMPONENTS-P5.md)） | 这些项目在 0.3.0 归档上通过，或失败可定位为本方缺陷或过期 spec 并在范围内修复 | V-5；责任人：实施者；最晚 M2 上传前 | 开放，失败按 §8 处理 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 版本号 | 十包统一 **0.3.0**：workspace `version = "0.3.0"`，CLI 自有 `version = "0.3.0"`，所有内部 `=0.2.1` 钉改为 `=0.3.0` | 0.x 下破坏性 API 按 Cargo 语义升 minor；五个源码未变的包（classification-macros、style、docs-syntax、docs-macros、style-wasm）也升 0.3.0，没有 API 变化，只是版本号变 | PRD 允许破坏性调整；`scripts/crates-io.py::library_inputs_changed` 把仓根 `Cargo.toml` 算作每个库的输入，而它自 R `b32844a4` 起已改（新增 `crates/rustify-colors` 成员），脚本会要求所有库升版；CRATES-IO-P1 §5.4 的统一版本先例 |
| D2 | `rustify-colors` | 作为第十个包进入 `release/crates-io.toml` 的 docs 组，排在 `rustify-dom` 之前，`dependencies = []` | 首次上传新名称，需要 `publish-new`；`rustify-dom` 的依赖可从 registry 解算 | `inspect` 当前报 `private workspace package permits publishing: rustify-colors`；`crates/rustify-dom/Cargo.toml` 依赖它 |
| D3 | 归档复用 | `package-libraries` 不带 `--reuse`，按首发方式重建全部九个库；不运行 `plan-libraries` | D1 下所有库都会移动版本，没有可复用的归档；`plan-libraries` 必须有 `--reuse` | 上一发行目录 `~/Documents/rustify-releases/crates-io-0.2.1-20261024/release-R1` 已不存在（2026-11-10 核对）；`crates-io.yml` 本就不带 reuse |
| D4 | Pages 候选 | 用发布后的公开说明提交 **D** 作为 `candidate_sha`，沿用现有 `docs-pages.yml` 的 Git 安装（source 分发），工作流零改动 | 线上内容与公开安装说明一致；D 中九个库目录及仓根库输入必须与 R 逐字节相同，`crates/rustify-docs-cli` 必须与 C 逐字节相同 | ADR-1 |
| D5 | 指南导航位置 | 三篇插在 `components.md` 之后、`vellum.md` 之前，顺序为 menus → panels → dom-components-migration；include、英文 Guides、中文「指南」三处一致 | 组件参考之后紧接组合指南与迁移；中文导航沿用现有英文内容回退 | `rustify-docs.toml` 现有顺序；本地 `check` 0 error（§1.1） |
| D6 | 浏览器矩阵 | 归档 CLI 跑 docs-*、components-*、docs-theme-v2、docs-dev；源码构建跑 dom-only、dom-primitives、dom-hydrate、components-static、colors-static 与 `docs-verify --group features`；GPU 示例与 Theme Studio 只做编译和 host 回归 | 覆盖 registry 消费者实际能拿到的 CLI 与 DOM SDK 路径；GPU SDK 不发布，Theme Studio 已在 P5 验收且本期不改 | CRATES-IO-P1 §9.5 矩阵去掉 GPU 两项；P5 C-7 |
| D7 | 首页与公开说明的版本文案 | 仓根 README（官方站首页引言）、`site/src/pages.rs` 的中文首页段落和各指南的 0.2.1/「source-only」说法，都只在上传成功后的 D 提交中改；包内 README 和模板在 R/C 前改 | 上传前 main 不宣称尚未发生的发布；包内文案随归档冻结 | CRATES-IO-P1 §5.1 的同一做法；runbook「Public completion」 |
| D8 | Theme Studio 导出说明 | 不改 | 「Published 0.2.1 SDK/CLI reject complete v2」在 0.3.0 发布后依然成立；Studio 不在 Pages 上；最低版本由 themes.md 和迁移指南说明（C-1） | `examples/theme-studio/src/data_tools.rs:346`；`tests/browser/theme-export-ui.spec.ts:99` |
| D9 | Pages 回滚目标 | last-good 仍是线上现有的 C2 `88566ba1ed08223a5834f794840fff5e48f4ba8a`（CLI 0.1.32，build `f74f4a4ece90f08a`） | 回滚用同一工作流重派 C2；legacy 模式已在 run 39388531674 部署成功 | 2026-11-10 读取线上 `rustify-docs-manifest.json` |

### 0.3 ADR-lite

#### ADR-1：Pages 用发布后的说明提交 D、以 source 分发的 Git CLI 构建

- 状态：Accepted
- 背景与驱动：R-1 要求线上反映已发布能力，C-6 要求候选 SHA 在 origin 可检出，D7 要求公开说明在上传后才切到 0.3.0。`docs-pages.yml` 会检出候选并执行 `mbx install --git … --rev <候选>`；`docs-pages-prepare.py::prepare` 要求 CLI 的 `--build-info source` 等于候选；`docs-pages-verify.py::check_identity` 要求候选中 `framework-release.json` 的 `library_rev` 等于 prepare 记录的 R。
- 备选：
  1. 用冻结的 CLI 提交 C 部署：C 中的公开说明仍是上传前的「0.2.1 / 源码可用」措辞，线上会说新 API 未发布，违背 R-1。淘汰。
  2. 改工作流为 `cargo install rustify-docs-cli --version 0.3.0`，以 registry 模式投影成 crates.io 依赖：线上产物与 registry 三包完全一致，但要改工作流和预检规则，且站点仍从候选检出读取 docs 内容，候选问题仍存在。改动面更大，收益只是把「源码 = 归档」由差异检查改成 registry 解算。淘汰。
  3. **选择**：上传成功后提交 D（只改公开说明、CHANGELOG、`site/src/pages.rs`），推送后以 D 派发。Git 安装的 CLI 报告 `source: D`、`distribution: source`，prepare 把 `site/Cargo.toml` 的三条框架依赖投影到该 checkout 的源码；D 继承 C 的 `framework-release.json`，`library_rev = R`。
- 决策：备选 3；另加一道门：`git diff --quiet R D -- <九个库目录> Cargo.toml mise.toml release/crates-io.toml` 与 `git diff --quiet C D -- crates/rustify-docs-cli` 必须都成立（C 在 R 之后改了 CLI 资产，CLI 目录只能对 C 比较），证明线上站编译的库与 CLI 源码就是已发布的源码。
- 正面后果：工作流和脚本零改动；线上内容与公开说明是同一次提交。
- 负面/中性后果：线上站点的 CLI 身份是 source 分发的 0.3.0，不是 registry 归档本身；0.2.x 之后 source 模式还没有在 CI 中部署过（A-2）。
- 重新评估触发：V-12 在 prepare 阶段因找不到 checkout 失败，按 §8 改走备选 2。

### 0.4 职责与事实所有权

|  | crates.io / docs.rs / GitHub | 本计划 | 发布人（用户） |
| --- | --- | --- | --- |
| 拥有 | 包名归属、版本不可变、索引、限流、docs.rs 构建；Actions 运行器与 Pages 托管 | 冻结 R/C/D 与 receipt、逐包上传顺序与续接、公开说明、站点配置、所有验收证据 | 账号与邮箱、临时 token 的创建/范围/过期/撤销，各授权门的同意 |
| 不拥有 | — | **不保存或打印 token；不 yank；不改 Pages 仓库设置；不发布 GPU SDK/Makepad/示例** | — |

一句话：本计划只把已冻结的字节交给 registry 和 Pages，凭证与授权始终在发布人手里，已上传的版本不可撤回。

### 0.5 明确不在本期

- **GPU SDK `rustify-ui`、`rustify-makepad` 与示例的 registry 发布**——保持 `publish = false`，留后续发行计划。
- **Theme Studio 的公开部署与导出文案更新**——D8；如需在 Studio 中写「需 0.3.0+」，另提需求。
- **为新组件（Accordion、Popover、DropdownMenu、Menubar 等）新增官方站 demo 页或扩展 24 类 CATALOG**——P5 固定 24 类；这些组件通过三篇新指南和组件目录说明展示。
- **三篇新指南与其他英文指南的中文翻译**——沿用现有英文内容回退。
- **`.agents/skills`、`.claude/skills` 下技能参考中的 0.2.1 文字**——由技能分发管理，不在本仓发行范围。
- **rustify-docs 生成器、`scripts/crates-io.py`、`scripts/docs-pages-*.py`、`docs-pages.yml` 的行为改动**——C-7；唯一例外是 `crates-io.yml` 与 runbook 的预取清单（§1.1 已核实缺口）。
- **性能基线、包体积对比、Windows/完整 Linux 运行支持**——没有本期要求；平台边界照实写进公开说明。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** 线上官方站：`cli_version 0.1.32`、build `f74f4a4ece90f08a`、96 个文件，来自 run 39388531674（2026-10-20），早于 DOM-COMPONENTS P1。线上内容是 C2 时的源码，完全不含 P1–P5 成果。
- **[已核实·足够]** crates.io 上九包为 0.2.1：库 R `b32844a489943649a49d9fbe46ee837e32017885`，CLI C `8fd7fd63679196e20f8e3572e4234bb51d8faf17`，标签 `rustify-libs-v0.2.1` / `rustify-docs-cli-v0.2.1`（[CRATES-IO-P1 发行记录](CRATES-IO-P1.records/M6.md#021-补丁发布2026-10-24-1926-0800)）。R 是 HEAD 的祖先。
- **[已核实·足够]** 自 R 以来，`crates/rustify-dom`（67 个文件）、`crates/rustify-components`（318 个）、`crates/rustify-docs`（5 个）、`crates/rustify-docs-cli`（5 个）有改动；classification-macros、style、docs-syntax、docs-macros、style-wasm 的目录没有改动；仓根 `Cargo.toml` 新增了 `crates/rustify-colors` 成员（`git diff --stat b32844a4 HEAD`）。
- **[已核实·缺口]** `python3.12 scripts/crates-io.py inspect` 失败：`private workspace package permits publishing: rustify-colors`。`crates/rustify-colors/Cargo.toml` 没有 `publish`、`readme`、`documentation`、`rust-version`、docs.rs 元数据，也没有 `LICENSE`；其 README 第 59 行写「currently consumed from this source checkout」。→ 按 `crates/rustify-docs-syntax/Cargo.toml` 补齐元数据与 LICENSE，并进入清单（D2）→ 漏做则无法打包 `rustify-dom`。
- **[已核实·缺口]** `tests/crates-io/test_boundaries.py:52` 断言清单恰为 9 项 → 改为 10。
- **[已核实·缺口]** `scripts/crates-io.py::lock_inputs` 会读取 `tests/dom-only` 下的所有锁，其中包括 `tests/dom-only/public-panel-consumer/Cargo.lock`；该锁有 16 个第三方 registry 包（如 `icu_* 2.4.0`、`serde_json 1.0.154`、`uuid 1.29.0`）不在已预取的八个锁中。`stage_registry` 只读 Cargo 本地缓存，缺失时报 `missing … run mbx fetch --locked`，不会自行下载。→ 在 `.github/workflows/crates-io.yml` 的预取步骤和 runbook 第 2 步各加一行 `mbx fetch --locked --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml` → 漏做则 CI 预检在 package-libraries 失败。
- **[已核实·缺口]** 包内 README 的安装钉仍是 `=0.2.0`（dom、docs、docs-syntax、docs-macros、style、style-wasm、classification-macros 及 `crates/rustify-style/upstream/README.md`），或是 `=0.2.1`（components、CLI README、components 模板 README）→ R/C 前统一改为 0.3.0 → 漏做则 crates.io 页面给出错误的安装命令且无法改字节。
- **[已核实·足够]** 0.2.1 的版本改动集是 `b32844a4`（32 个文件）加上 `8fd7fd63`（freeze）与 `0efbea59`（说明）。本次照它做，另需覆盖其后新增的 fixture：`tests/dom-only/primitives`、`tests/dom-only/public-panel-consumer`、`tests/theme-export/{dom,runtime}`、`tests/theme-text`、`tests/docs-ssg/fixtures/theme-v2`（`git grep -l "0\.2\.1"`）。
- **[已核实·足够]** 三篇新指南加入配置后，`target/docs-cli/rustify-docs check`（source CLI）的结果为 80 页、0 error、40 warning；基线为 74 页、0 error、37 warning，新增的 3 条都是 `i18n-missing-page`，与其他英文指南同类。指向 `../crates/…`、`plan/…`、`validation/…` 的链接不报错。
- **[已核实·缺口]** `tests/docs-ssg/official.spec.ts` 固定了与指南数相关的数字：`guides` 长度 12（213 行）、测试标题「exact twelve guides」（311 行）、该用例的侧栏链接数 36（315 行，12 篇指南＋24 个组件页）、中文搜索索引 37 页/35 页回退（255–256 行）、`llms.txt` 37 条（265 行），共五处。加三篇后应为 15、「fifteen」、39、40/38、40；A 落地时以 `toHaveLength\|toHaveCount` grep 复查 spec 内全部计数断言。`guides` 由 include 解析，逐篇的「English, reachable」用例（320 行）会自动覆盖新指南。
- **[已核实·缺口]** `site/src/pages.rs::home_zh` 写着「工作区版本为 0.1.0，尚未开放包发布」，与已发布的事实不符；英文首页引言来自 README.md（`markdown_intro`），README 第 7 行仍写 0.2.1、九包 → 在 D 中更正（D7）。
- **[已核实·足够]** Git 安装 CLI 的身份：cargo checkout（如 `~/.cargo/git/checkouts/rustify-ui-2d63cd2817e50ebf/d0d84d7`）含 `.git` 目录，`refs/heads/master` 指向该 rev；`crates/rustify-docs-cli/src/release/source.rs::identify` 因此报告 `distribution: source` 和真实 SHA。`docs-pages-prepare.py::cli_identity` 在 `CARGO_HOME/git/checkouts/*/*` 中按 `development_checkout` 指纹找回该 checkout。
- **[已核实·足够]** `docs-pages-prepare.py::prepare_project` 在 source 模式下只投影 `site/Cargo.toml` 的三条依赖到 checkout 的 `crates/<包>`，并用 `mbx metadata --offline` 校验，不改 `site/Cargo.lock`；`read_identity` 要求 checkout 中各包版本等于 `framework-release.json`，所以 D 中三包版本须为 0.3.0（由 R/C 继承）。
- **[已核实·足够]** `docs-pages-verify.py` 不固定文件数或指南数；`--dir` 做本地产物与图检查，`--url --run-id` 读取同一 run 的产物，并逐个读取公开文件做比对。

### 1.2 拓扑与文件清单

发布顺序沿 [release/crates-io.toml](../../release/crates-io.toml)：classification-macros → style → docs-syntax → docs-macros → **colors ★** → dom → components → docs → docs-cli（docs 组），公开门通过后发 style-wasm（remaining 组）。

| 路径 | 改动 | 提交 |
| --- | --- | --- |
| `rustify-docs.toml` | include、`[[nav.groups]]` Guides、`[[i18n.locales.nav.groups]]` 指南三处插入三篇（D5） | A |
| `tests/docs-ssg/official.spec.ts` | §1.1 所列的五处计数与标题 | A |
| `release/crates-io.toml` | 新增 `rustify-colors` 条目（D2）；`rustify-dom` 条目的 `dependencies` 加 `rustify-colors`（`crates/rustify-dom/Cargo.toml` 已有该 path 依赖，`inspect` 要求清单依赖与 manifest 一致） | R |
| `crates/rustify-colors/Cargo.toml`、`crates/rustify-colors/LICENSE` ★、`crates/rustify-colors/README.md` | 补 `publish = ["crates-io"]`、`documentation = "https://docs.rs/rustify-colors/0.3.0"`、`readme`、`rust-version = "1.99"`、include 加 `LICENSE`、`[package.metadata.docs.rs]`（同 docs-syntax）；LICENSE 复制 `crates/rustify-docs-syntax/LICENSE`；README 第 59 行改为 registry 安装 `rustify-colors = "=0.3.0"` | R |
| `Cargo.toml`、各包 `Cargo.toml`、`Cargo.lock` | workspace 版本 0.3.0；所有内部 `version = "=0.2.1"` → `=0.3.0`；`documentation` 链接改为 0.3.0；CLI `version = "0.3.0"` | R |
| 各包 `README.md`、`crates/rustify-style/upstream/README.md`、`crates/rustify-docs-cli/templates/components/{Cargo.toml.in,README.md}`、CLI `README.md` | 安装钉与「Expect version」改为 0.3.0（CLI README 没有 C/R 字段；期望的 C/R 值随 D 写入仓根 README 与 docs-ssg.md） | R |
| `crates/rustify-docs-cli/src/new.rs:431`、`crates/rustify-docs-cli/tests/new_components.rs:177,193`、`tests/crates-io/test_boundaries.py`、`tests/crates-io/test_pairing.py`、`tests/crates-io/test_release.py` | 断言或常量中的当前发行版本 0.2.1 → 0.3.0；清单计数 9 → 10。历史记录（如 0.1.32/0.2.0 迁移分支）不动。`scripts/crates-io.py`、`scripts/docs-pages-prepare.py`、`xtask/src/docs.rs` 已核对只含 wasm-bindgen `0.2.128`，无发行版本常量 | R |
| `site/Cargo.toml`、`site/Cargo.lock`、`tests/docs-ssg/fixtures/{components,theme-v2}/`、`tests/dom-only/**`、`tests/theme-export/**`、`tests/theme-text/`、`tests/crates-io/compat/` 的 manifest 与锁 | 版本钉与 path 包版本跟随，锁以 `mbx update --offline -p <包>` 重新解析，不手改 checksum | R |
| `.github/workflows/crates-io.yml`、`docs/crates-io-release.md` | 预取清单加 `tests/dom-only/public-panel-consumer/Cargo.toml`；runbook 包数措辞随清单改：nine packages → ten（第 3、117、191、197 行），eight library archives/Docs-group packages/libraries/documentation jobs → nine（第 88、92、177、198–199 行） | R |
| `AGENTS.md`「完成标准与发布」 | 「crates.io 九包」→「十包」 | R |
| `crates/rustify-docs-cli/assets/framework-release.json` | 按 runbook 第 1 步与 `b32844a4` 先例：R 中 `library_rev` 置 `null`（不写 R 自身）、三包版本改为 0.3.0 | R |
| `crates/rustify-docs-cli/assets/{framework-release.json,components-Cargo.lock}` | 由 `freeze-cli` 生成，不手写 | C |
| `README.md`、`CHANGELOG.md`、`docs/{quickstart,docs-ssg,themes,dom-components-migration,components,forms,menus,panels}.md`、`docs/zh/quickstart.md`、`site/src/pages.rs` | 上传后切换公开说明（§5.3 逐行清单与关键词兜底） | D |
| `docs/docs-ssg.md`「Official site delivery」 | 记录 D 的 run、build、last-good C2 | E |
| `docs/plan/DOM-COMPONENTS-RELEASE.records/` ★ | 里程碑记录 | 随里程碑 |

零改动面：`makepad/`、GPU SDK 与 Makepad 桥、`examples/**`（Theme Studio 见 D8）、`crates/rustify-docs*/src` 的行为、`scripts/*.py` 的逻辑、`docs-pages.yml`、`verify.yml`。依据：本期只换版本、配置与说明，不改组件或生成规则。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| crates.io registry | 已核实足够（外部） | 0.2.1 九包由本仓库账号发布；`rustify-colors` 尚不存在；新包有限流（CRATES-IO-P1 §5.4） | 只上传冻结归档；`publish` 子命令逐包 dry-run、核对 digest、查询索引后再继续 | 超时→`waiting-index` 先查状态；429→记录 `next_retry_at` 后续接；同名同版本异 checksum→停止（runbook「Upload and resume」） |
| docs.rs | 已核实足够（外部） | 0.2.1 八库构建成功；CLI 无库 target，用 README、`--help` 和真实安装作文档门（CRATES-IO-P1 发行记录） | 上传后等待九个库（含 colors）的 0.3.0 构建 | 排队或失败记为未通过项，不宣称完成；修复需 0.3.1（§8） |
| 发布人账号与临时 token | 未核实（外部，只阻塞 M3） | A-1 | V-8 核对名称模式覆盖十包、`publish-new` + `publish-update`、过期时间覆盖上传等待；Agent 不读取凭证文件 | 凭证缺失或过期→停在当前包，由发布人更换后用同一 receipt 续接 |
| `crates-io.yml` 远端预检 | 已核实缺口（自有） | 预取缺 `public-panel-consumer`（§1.1） | R 中补一行；授权推送后派发 | 任一平台失败→不进 M3，按失败点修复后重冻 |
| `docs-pages.yml` + prepare/verify 脚本 | 已核实足够；source 模式在 CI 未实跑（A-2） | §1.1 | 零改动，派发 D | build 失败→线上保持 C2；部署后核验失败→重派 C2（§8） |
| GitHub Pages 环境 `github-pages` | 已核实足够 | 线上可读，Actions 源已配置（docs-ssg.md） | 不改设置 | — |

## 5. 核心机制：冻结、发行与部署顺序

### 5.1 R：库候选

1. 提交 A（站点导航与 spec）可先于 R，二者互不影响：库输入只有包目录、仓根 `Cargo.toml`、`mise.toml` 与清单（`library_inputs_changed`）。
2. R 包含 §1.2 中标为 R 的全部改动；`framework-release.json` 在 R 中 `library_rev` 置 `null`、三包版本为 0.3.0，不写 R 自身（runbook 第 1 步，同 `b32844a4`），freeze-cli 生成 C 时覆盖为 R。C 只提交两份资产（同 `8fd7fd63`）；公开说明中期望的 C/R 值随 D 写入仓根 README 与 docs-ssg.md（同 `0efbea59`）。
3. R 提交前跑 V-2；R 之后任何改动若落在库输入上，必须重新提交 R′ 并重跑 §9.2 所列失效检查，不得在 C 中混入库改动（`package-cli` 会拒绝）。

### 5.2 C：配对冻结与上传前收口

在仓外持久目录 `DIR=~/Documents/rustify-releases/crates-io-0.3.0-<YYYYMMDD>/release` 中执行（不在仓内、不在 mbx 管理的 `target/`）：

```sh
mbx fetch --locked   # 及 runbook 第 2 步全部预取，含 public-panel-consumer
python3.12 scripts/crates-io.py inspect
python3.12 scripts/crates-io.py package-libraries --rev "$R" --out "$DIR"
python3.12 scripts/crates-io.py freeze-cli --libraries "$DIR"
git add crates/rustify-docs-cli/assets && git commit   # 即 C
python3.12 scripts/crates-io.py package-cli --rev "$C" --libraries "$DIR" --out "$DIR"
python3.12 scripts/crates-io.py verify --release "$DIR"
```

随后跑 V-3 中的 CLI 三行、V-5、V-6；全部通过后，在 V-7 授权门向用户出示 R、C、receipt 中十包版本与压缩大小，以及要执行的推送和派发命令，获准后推送 main 并派发 `crates-io.yml`。

### 5.3 上传与公开说明 D

1. V-8 通过后，`publish --release "$DIR" --group docs`（九包，含 colors），然后跑 `verify --public-docs`，再 `--group remaining`（style-wasm）。
2. 十包都可从公开索引获取后，提交 D，只改下列说明，不碰库输入与 `crates/rustify-docs-cli`：
   - `README.md`：第 7 行改为 0.3.0、十个包；第 9、12、23、29 行的「source / 不在 0.2.1」说法改为「0.3.0 起可用」；第 247–250 行的安装命令改为 0.3.0；第 258–263 行写新的 C/R，回滚目标改为 registry 0.2.1。
   - `docs/quickstart.md`：第 232 行的 fixture 永久链接改为新 R 的 `tests/dom-only/consumer/`；第 238–306 行的 `=0.2.1` 改为 `=0.3.0`；第 405–406 行整句改为说明这些 API 由 0.3.0 提供。
   - `docs/zh/quickstart.md` 第 153–157 行：去掉 Git rev `1b81de25…` 来源，改为与英文 quickstart 相同的 registry `=0.3.0` 依赖，示例链接指向新 R 的 consumer，并删除「已验证的 Git 来源」说明。
   - `docs/docs-ssg.md`「Install and maintain the CLI」：0.3.0、新的 C/R、期望 `rustify-docs 0.3.0`；回滚目标改为 registry 0.2.1（`cargo install rustify-docs-cli --version 0.2.1 --locked --force`，同时恢复项目 manifest/lock 与 schema v1 主题文件），0.1.32 历史段保留；第 333、740 行的多语言示例与第 792–793 行的 components 模板说明改为 0.3.0。
   - `docs/themes.md` 第 8、236、372–374 行，`docs/dom-components-migration.md` 第 6–8、12–14、26、155、241、466、512 行：写明 schema v2 的最低消费者版本是 SDK/CLI **0.3.0**，0.2.1 只读 v1（C-1）；`rustify-colors` 改为已发布；迁移步骤改为「从 0.2.1 升级到 0.3.0」。
   - `docs/components.md` 第 6–9 行（`<!-- catalogue -->` 标记之上的手写引言）、`docs/forms.md:12–15`、`docs/menus.md:8`、`docs/panels.md:8–9`：改为「随 0.3.0 发布」。
   - 关键词兜底：按上述清单改完后，`git grep -nE '0\.2\.1([^0-9]|$)|source checkout|source-only|[Nn]ine published|b32844a4|1b81de25' -- README.md docs site/src ':!docs/plan' ':!docs/validation' ':!docs/reports' ':!docs/reviews' ':!docs/*.review*.md'` 逐处核对，除回滚说明外全部改为 0.3.0 后的事实（CHANGELOG 的历史条目不在检索范围内，保持不动）。
   - `site/src/pages.rs::home_zh` 第三段改为当前事实（Docs/DOM/样式等十个包以 0.3.0 发布在 crates.io，GPU SDK、Makepad 桥接和示例仍从 Git 使用）。
   - `CHANGELOG.md`：新增 0.3.0 条目，列出新增 `rustify-colors`、破坏性 API 与 schema v2，链接迁移指南，并写明验证平台边界。
3. 推送 D（在 M3 授权范围内）后跑 V-10；通过后打并推送标签 `rustify-libs-v0.3.0 → R`、`rustify-docs-cli-v0.3.0 → C`。最后撤销临时 token，并由发布人执行 `cargo logout --registry crates-io`（若凭证保存在本地）。

### 5.4 Pages：从 D 部署

1. 门：`git diff --quiet "$R" "$D" -- crates/rustify-style/upstream/classification-macros crates/rustify-style crates/rustify-docs-syntax crates/rustify-docs-macros crates/rustify-colors crates/rustify-dom crates/rustify-components crates/rustify-docs crates/rustify-style-wasm Cargo.toml mise.toml release/crates-io.toml` 与 `git diff --quiet "$C" "$D" -- crates/rustify-docs-cli` 都成立（C 改了 CLI 资产，CLI 目录不能对 R 比较）；`git show "$D":crates/rustify-docs-cli/assets/framework-release.json` 的 `library_rev` 等于 R。
2. V-11 本地预演通过后，在 M4 授权门出示 D、预演结果和派发命令；获准后执行 `gh workflow run docs-pages.yml --ref main -f candidate_sha="$D"`，build 与 deploy 都成功才进入 V-12。
3. V-12、V-13 通过后提交 E（docs-ssg.md 官方站交付节：候选 D、run ID、build_id、last-good C2），在同一授权内推送。E 不重新部署：线上站点不显示自身的部署记录，与 DOCS-SSG-P4-gaps 的 `c6d49c97` 做法相同。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| docs 组部分上传后中断（网络、429、凭证过期） | 已上传的低层包公开，上层包缺失 | 无损；版本不可变 | 新版本暂时无法安装 CLI 或上层包 | `publish` 的 receipt 状态与退出码 75 | 同一 receipt 续接，不重打包、不换版本（runbook） | V-9 |
| 公开门（`verify --public-docs`/`--public`）或 docs.rs 在上传后失败 | 0.3.0 已公开但有缺陷 | 无法撤回字节 | 作者按说明安装失败或文档缺失 | V-9、V-10 | 不推送 D、不部署 Pages；在 R′ 修复并发布 0.3.1 重走 M2–M3；yank 须发布人另行明确授权（runbook） | 重跑 V-4–V-10 |
| Pages build 在 prepare 阶段找不到 source checkout（A-2） | 本次部署 | 无 | 线上保持 C2 | run 日志 | 改走 ADR-1 备选 2：另开计划修订工作流，不在本计划内临时改动 | V-12 |
| Pages 部署后核验失败（文件、身份、浏览器） | 官方站 | 无 | 读者看到异常内容 | V-12、V-13 | `gh workflow run docs-pages.yml --ref main -f candidate_sha=88566ba1ed08223a5834f794840fff5e48f4ba8a` 回滚到 C2，并对该 run 跑 V-12 的 `--url` 核验；回滚派发沿用 M4 已取得的派发授权，执行后回写 M4 记录 | V-12 |

- 发布顺序：A → R → C → 推送与预检 → 上传 docs 组 → 公开门 → 上传 remaining → D → V-10 → 标签 → Pages(D) → E。
- 回滚门：上传不可回滚；Pages 只按上表回滚到 C2；作者端回滚到 0.2.1 的方法写在 D 的 docs-ssg.md 中。

## 9. 验证

### 9.1 检查表

共同环境：macOS，Rust 1.99.0（mise），mbx，`/opt/homebrew/bin/python3.12`；浏览器类命令前把指向 python3.12 的 `python3` 放到 PATH 最前（`PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH"`），Node 26，Playwright 自带 Chromium。

| 检查 ID | 层级 | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- | --- |
| V-1 | 场景 E2E | R-3：读者在官方站根、子路径和无 headers 宿主上，从侧栏依次打开 Menus、Panels、迁移三篇，页面为英文、无 wasm 请求；中文壳中这三篇以英文回退出现；侧栏顺序等于配置 | `for p in official-root official-subpath official-meta; do PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH" npx --no-install playwright test -c playwright.docs.config.ts --project="$p" official.spec.ts \|\| break; done`（source CLI，当前工作树；单项目约 6.5 分钟，依据 P5 §9.4） | M1：A 的改动齐备后；批内反馈用 `--project=official-subpath -g "guide /menus/\|ordinary sidebar"` |
| V-2 | 局部 | R-2、C-2：清单恰为十包，`rustify-colors` 可打包；内部钉、模板、锁和测试常量一致；静态门通过 | `python3.12 scripts/crates-io.py inspect`；`python3.12 -m unittest discover -s tests/crates-io`（准入：仓库已有发行协议测试，版本常量改动直接影响它）；`mbx test --locked -p rustify-colors --lib` 及 `--features p3`；`mbx test --locked -p rustify-docs-cli --bin rustify-docs new::`；`mbx test --locked -p rustify-docs-cli --test new_components`；对 §1.2 中每个改过的 manifest 执行 `mbx metadata --locked --format-version 1 --manifest-path <路径> > /dev/null`；`cargo fmt --all -- --check`；`mbx xtask css --check`；`mbx xtask catalog --write docs/components.md --check`；`mbx xtask sources verify`；`mbx clippy --locked -p rustify-colors --all-targets -- -D warnings` | M1：R 的改动齐备后、提交 R 前 |
| V-3 | 阶段收口 | C-2：源码全量回归不减少，含 colors | [CRATES-IO-P1 §9.3](CRATES-IO-P1.md#93-源码回归命令组) 的命令组，另加 `mbx test --locked -p rustify-colors --lib --features p3`；在 detached R 跑库与非 CLI 部分，在 C 跑 docs-cli 的 test/clippy/corpus 三行；docs 两包不与 CSR workspace 联合运行 | M2：库部分在 package-libraries 前，CLI 三行在 package-cli 前，各一次 |
| V-4 | 阶段收口 | R-2：十个真实归档（九库 + CLI）在 staging registry 中可安装；CLI 报告 `distribution: registry`、`source: C`、`panic: unwind`、三包 0.3.0 与 `library_rev = R`；两种模板在仓外 check/build 通过；归档与锁 digest 可复现 | §5.2 的 package-libraries → freeze-cli → package-cli → `verify --release "$DIR"`；记录每包 compressed bytes 与 SHA-256 | M2：V-3 库部分之后 |
| V-5 | 阶段收口 | R-2、A-3：注册表消费者的页面、组件岛、主题 v2、dev 与 DOM SDK 无回退 | 按 [CRATES-IO-P1 §9.5](CRATES-IO-P1.md#95-v-10-的浏览器用例与环境) 的环境（staging CARGO_HOME、`RUSTIFY_DOCS_BIN` 指向归档安装的 CLI、禁止回退到源码 CLI）逐条串行：`--project=docs-root`、`docs-subpath`、`docs-meta`；`components-root/subpath/meta islands.spec.ts custom.spec.ts`；`docs-theme-v2`；`docs-dev dev.spec.ts dev-components.spec.ts dev-i18n.spec.ts dev-uat.spec.ts dev-fonts.spec.ts`；`mbx xtask docs-verify --group features`；源码构建的 `dom-only`、`dom-primitives`、`dom-hydrate`、`components-static`、`colors-static`（均 `-c playwright.docs.config.ts`，每条一个项目）。总耗时未知，实测记入 M2 记录；每条的报告与截图另存到仓外目录，防止下一条覆盖 | M2：V-4 之后、V-7 之前 |
| V-6 | 场景 E2E（角色模拟） | R-2、C-1：已有 0.2.1 components 站点的作者升级到 0.3.0，并换用 v2 主题 | 角色：会 Rust 和 Cargo、维护一个由公开 0.2.1 CLI 生成的 components 站点；只能读 C 树中的 CLI README、components 模板 README 与 `docs/dom-components-migration.md`，不读源码。准备：在仓外用全新 `--root` 从 crates.io 安装 0.2.1 CLI，`new --template components` 生成站点并 build 一次。任务：按文档换用 staging 0.3.0 CLI，升级 manifest/lock，把 `tests/docs-ssg/fixtures/theme-v2` 中的 v2 主题放进站点，`check --deny-warnings`、`build`、`preview`，在浏览器中确认组件岛可交互、主题生效；再按文档回滚到 0.2.1 并恢复 v1 主题。记录每个卡点；文档卡点在包内的按 §5.1 重冻，在仓根的留到 D | M2：V-4 之后，早于任何上传 |
| V-7 | 风险门 | C-3：远端双平台预检 | 授权后 `git push origin main`；`gh workflow run crates-io.yml --ref main -f library_rev="$R" -f cli_rev="$C"`；`gh run watch <id> --exit-status`；确认 run 使用的是 R/C，macos-14 与 ubuntu-24.04 都成功，记录 run ID。推送会触发 verify 工作流，可能让派发排队约 13 分钟 | M2 末：V-3–V-6 通过后 |
| V-8 | 风险门 | A-1、C-3：上传授权与账号 | 向用户出示十包名称/版本、R/C、receipt、上传命令以及后续 D/标签推送范围，取得授权；发布人确认账号、邮箱与 token 范围（名称模式、`publish-new` + `publish-update`、过期时间），不打印 token；Agent 重查 crates.io 上十个名称的 owner 与现有最高版本（`rustify-colors` 不存在，其余为 0.2.1） | M3 开始、任何上传之前 |
| V-9 | 风险门 | R-2：真实上传与组间公开门 | `publish --release "$DIR" --group docs`；`verify --release "$DIR" --public-docs`（全新 Cargo home 与 root，无源替换）；通过后 `publish --release "$DIR" --group remaining`；公开 API 的十个 checksum 与 receipt 逐一一致 | M3：V-8 之后 |
| V-10 | 阶段收口 | R-2、C-1：公开终验 | 提交 D 前：§5.3 的关键词检索除回滚说明外无残留，`docs/zh/quickstart.md` 的依赖来源为 registry `=0.3.0`。D 推送后：`verify --release "$DIR" --public`；用 `git ls-remote` 确认 D 在公共 main，从 `raw.githubusercontent.com/rockie/rustify-ui/<该 SHA>/` 读回 README、docs-ssg.md、themes.md，确认安装命令为 0.3.0、v2 最低版本为 0.3.0；九个库的 docs.rs 0.3.0 页面返回 200 且构建成功（CLI 按既有边界核对公开 README、`--help` 与真实安装）；通过后推送两个标签，并用 `git ls-remote --tags` 读回 | M3 末 |
| V-11 | 风险门 | R-1、A-2、C-6：Pages 工作流本地预演 | §5.4 第 1 步的差异门；`mbx install --git "file://$PWD" --rev "$D" --locked --profile docs-cli --root "$PREVIEW/cli" rustify-docs-cli`；`/opt/homebrew/bin/python3.12 scripts/docs-pages-prepare.py --candidate-sha "$D" --cli "$PREVIEW/cli/bin/rustify-docs" --out "$PREVIEW/site"`（`$PREVIEW` 在 `$TMPDIR`，位于仓库和任何 `.rustify-docs/` 之外）；在该目录跑 `check`/`build --format json`，0 error；照工作流的内联脚本写 `identity.json`，再跑 `docs-pages-verify.py --dir … --identity … --candidate-sha "$D" --report …`；确认产物包含 `menus/`、`panels/`、`dom-components-migration/` 及其 zh 回退页，manifest 的 `cli_version` 为 0.3.0 | M4 开始、派发之前 |
| V-12 | 风险门 | R-1、R-3：线上身份与读者任务 | 派发后 `gh run watch`，build 与 deploy 都成功；`python3.12 scripts/docs-pages-verify.py --url https://rockie.github.io/rustify-ui/ --candidate-sha "$D" --run-id "$RUN"` 通过；在 `.rustify-docs/release-0.3.0/` 写临时配置：沿用 `official-meta` 项目，`baseURL` 改为线上地址，`webServer: []`；运行 `npx playwright test -c <临时配置> --project=official-meta official.spec.ts --grep "official Chinese home\|ordinary sidebar\|guide /menus/\|guide /panels/\|guide /dom-components-migration/\|CATALOG button uses\|deep not-found\|the dark first frame"`，全部通过 | M4：派发成功后 |
| V-13 | 真实使用验收 | R-1、R-2、R-3、C-1：读者和开发者只凭线上站点完成任务 | 干净浏览器上下文 + 全新 `CARGO_HOME`/`--root`，只用线上站点与 crates.io。任务一（读者）：从首页进入 Guides → Menus，读完后到组件页操作 Data table 和 Tree demo，切换深色与中文，再用站内搜索找到 Menubar 并定位到正确段落；确认页面说明「0.3.0 可用」，没有「仅源码」或「0.2.1」误导。任务二（DOM 开发者）：按线上 quickstart 的 DOM 应用一节新建项目，依赖 registry `=0.3.0`，把 menus.md 的 Menubar 示例放进去，用 `wasm-bindgen-cli 0.2.128` 构建，在浏览器中用键盘打开菜单、切换项目。任务三（作者）：按线上 docs-ssg 安装 CLI 0.3.0，`new --template components`，导入一份 v2 主题后 build、preview。任务四（既有作者升级）：用 crates.io 0.2.1 CLI 生成 components 站点并 build 一次，只按线上迁移指南与 docs-ssg 把 CLI、manifest/lock 与 v1 主题升级到 0.3.0 和 v2，再 build、preview；同时确认 V-6 记录的仓根文档卡点已消除。亲自查看页面和产物，记录卡点；发现的问题修复后重跑受影响的部分，或列给用户确认延期 | M4 收口：V-12 通过后 |

### 9.2 证据失效

- 改动落在库输入上（包目录、仓根 `Cargo.toml`、`mise.toml`、清单）→ 重新提交 R′，V-2 的相关项、V-3 库部分以及 V-4–V-7 全部失效；V-1 只有在站点内容或 CLI 行为变化时才失效。
- 只改 CLI 包 → 重冻 C′，V-3 的 CLI 三行、V-4 的 CLI 部分、V-5、V-6、V-7 失效。
- 上传之后，只允许 D/E 这类仓根说明改动；任何包字节变化都需要新版本（§8）。

### 9.4 浏览器执行纪律

每条 Playwright 命令只带一个项目，串行执行（C-4）；`target/` 可能被 mbx 回收，产物和证据放仓外，必要时用 `serve --dir` 服务复制出的产物。spec 写入 `docs/validation/` 下的历史截图时，收口后用 `git checkout --` 还原（P5 §9.4 的做法），并用 `git diff` 核实。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 官方站导航与库候选 R | 无 | 一批，两个提交：A＝`rustify-docs.toml` 三处、`official.spec.ts` 五处（§1.1）；R＝§1.2 中标为 R 的全部改动，照 `b32844a4` 的改动集做，并覆盖其后新增的 fixture；锁用 `mbx update --offline -p <包>` 重解析。单人负责所有 manifest 与锁 | V-1、V-2；R 已提交，且 `git status` 中没有遗留的库改动；回写「实施进度」 |
| M2 | 上传前发行收口 | M1 的 R；预取完成 | 严格串行：detached R 上的 V-3 库部分 → package-libraries → freeze-cli → 提交 C → C 上的 V-3 CLI 三行 → package-cli → verify（V-4）→ V-5 → V-6 → 授权后 V-7。`DIR` 单人负责；不上传 | V-3–V-7；receipt 冻结且没有必需缺口；回写「实施进度」 |
| M3 | crates.io 发布与公开说明 | M2 的 `DIR` 与 V-7；V-8 的授权与凭证 | 按 §5.3：先上传 docs 组，过公开门，再上传 remaining；提交并推送 D；V-10 通过后推送标签；撤销临时 token。凭证与生产 registry 单人负责 | V-8–V-10；十包与公开说明实际可获取；未完成不算发布成功；回写「实施进度」 |
| M4 | Pages 部署与真实使用验收 | M3 的 D 已在公共 main，V-10 通过；M4 派发授权 | §5.4：差异门 → V-11 → 授权派发 → V-12 → V-13 → 提交并推送 E；清理 `$PREVIEW` 下的 detached worktree（`git worktree remove --force`）以及 `git worktree prune` | V-11–V-13；线上 manifest 的 `cli_version` 为 0.3.0，`build_id` 等于本次 run 的记录；回写「实施进度」 |

提交点：M1 的 A、R 各自提交；M2 中 freeze 后提交 C，阶段完成后提交记录与进度回填；M3 提交 D（只含公开说明）并推送，随后提交记录；M4 提交 E（docs-ssg 交付节）与最终进度，推送。每次推送都要在对应授权范围内；当前分支 main，不新开分支。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| A-1：账号、token 与 `rustify-colors` 名称 | 上传 | 发布人准备凭证；Agent 在 V-8 重查，名称被占用就停止并询问，不自行改名 | M3 上传前 | 只阻塞 M3 |
| 推送、上传、Pages 派发三道授权 | 远端预检、发布、部署 | 每道门出示具体 SHA、包名/版本与命令后询问用户；本计划获批不等于这些动作已获授权；§8 回滚到 C2 的重派沿用 M4 派发授权 | V-7 / V-8 / M4 派发前 | 未获授权则阻塞对应门 |
| A-3：多期未运行的浏览器项目 | M2 工期与修复量 | V-5 失败先判断是产品回归还是过期 spec；产品缺陷修在 R′（按 §9.2 重冻）；修复量明显超出版本发行时报告用户，决定继续修复还是收窄矩阵 | M2 | 否（工期未知） |
| A-2：CI 中 source 模式 Pages | 部署 | V-11 本地预演；CI 失败按 §8 保持 C2，另起工作流修订 | M4 | 否 |
| 上传后发现缺陷 | 0.3.0 不可修改 | 先做 V-4–V-6 再上传；之后的缺陷走 0.3.1（§8） | M3 | 否 |

- 最终状态：**Ready**。
- 定级理由：版本、包集合、复用方式、Pages 候选和浏览器矩阵都有确定路线，事实已核到脚本行为；M1 可直接开工。外部凭证与三道授权是 M2 末、M3、M4 的阶段门，不影响开发。

## 12. 已知坑与历史教训

- 系统 `python3` 是 3.9，没有 `tomllib`；脚本用 `/opt/homebrew/bin/python3.12`，`serve.sh` 子进程要靠 PATH 上的 3.12（项目记忆 docs-pages-release-env-traps）。
- `rustify-docs build` 拒绝把输出放在仓库内或任何 `.rustify-docs/` 下，prepare 的 `--out` 用 `$TMPDIR`；每次 prepare 会留一个约 4 GB 的 detached worktree，验证后需移除。
- 全新 `CARGO_HOME` 下 mbx 会报找不到真实 Cargo，要先把 `~/.rustup/toolchains/1.99.0-aarch64-apple-darwin/bin` 放到 PATH 前面；mbx 的 socket 路径过长时用短 `TMPDIR`（如 `/tmp/…`）。
- zsh 中 `"$SHA:path"` 会被当成修饰符解析，一律写成 `"${SHA}:path"`。
- 推送 main 会触发 verify 工作流占满 macOS 运行器，紧随其后的派发约排队 13 分钟，不是故障。
- 首次 `publish` 曾遇到 LibreSSL `SSL_ERROR_SYSCALL`：先查 crates.io 确认没有半成品，再用同一 receipt 重试，不重新打包（[CRATES-IO-P1 发行记录](CRATES-IO-P1.records/M6.md#021-补丁发布2026-10-24-1926-0800)）。
- 上一发行目录已丢失，因此复用不了旧归档（D3）。本次的 `DIR` 在 M3 完成后保留，并在记录中写明路径，供下次 `--reuse` 使用。
- `target/` 是 mbx 管理的符号链接，可能在会话中途被回收，证据和产物不要只放在 `target/` 中（项目记忆 mbx-prunes-managed-target）。
- Playwright 每条命令一个项目、串行执行；全量并发会耗尽内存，并发运行还会互相清掉 `test-results/`（AGENTS.md「浏览器测试」）。
- cargo fmt 会重排换行；用脚本批量改版本号时要断言旧文本确实命中（AGENTS.md「编码与样式约定」）。
