# DOM-COMPONENTS-RELEASE · Rustify UI · DOM 组件成果发行与官方站更新（十包 0.3.0 · 完整 DOM 成果展示 · 公开 Theme Studio · Pages）

```formal
INTENT:
  GOAL: "直接依据本计划、仓库规则和当前代码实施或续做"
  SUCCESS: "全部里程碑退出检查通过，按‘实施进度’完成最终提交、SHA 回填与自查，并提示用户发起代码评审"

RULE @implementation_entry:
  MUST: "实施须知：先读下方‘实施者定位’‘实施进度’，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；goal 执行也遵循这些约定"
```

> **计划状态：Ready**
>
> 调查基线：2026-11-11 · `921802b3c9d1022033bd500681b870a430652616`（main）· dirty（M4 补证、代码评审报告与 SDLC 台账；产品代码事实取自 HEAD，补证事实取自当前工作树）。
>
> 需求来源：用户需求（2026-11-10）：「更新 rustify-docs 做的 github pages，展示最新的 docs/PRD-DOM-COMPONENTS.md 成果」；用户同日选定「先发 crates.io 新版再部署」「三篇指南都加入 Guides」。发行对象是 [DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) 在 P1–P5 计划中已交付的源码成果。用户 2026-11-11 再次明确「要按照原目标完成」「把我们做的东西充分展示」。本计划承接该 PRD P1–P5 已交付能力的官网展示和 §5.2 Q3 的发行配对，不重切 PRD 期次、不重做已交付组件。
>
> 本期交付：**十包 0.3.0 的已完成发行，加上官网可操作的完整成果展示：Colors、Primitives、Themes、全部已交付 DOM 组件与 Rustify 扩展；公开现有 Theme Studio 的组件陈列、主题编辑、Inspector、保存和导出。读者从官网首页即可找到、操作并带走成果。**
> **开发范围：保留 M1–M4 发行交付；续做 `site/` 页面、demo 与站点导航，必要的站点专属静态样式、公开组件/主题/迁移说明，现有 Theme Studio 的公开入口与部署适配，以及 `scripts/docs-pages-*.py`、`.github/workflows/docs-pages.yml` 的整站产物组合和验证。依据见 §0、§5.5–§6。**
> 呈现路径：官网与新增示例为 **DOM/SSG**；公开 Studio 复用已有 DOM/GPU/对比功能，组件陈列以 DOM 为主，不新增 GPU 组件、布局或绘制能力。两者为独立应用实例，文档页不加载 Studio 或 Makepad。
> 本期独特职责：把已完成的组件与主题研发变成可发现、可试用、可复制的公开成果；**版本号、发包成功、三篇指南可访问、旧 24 类简单样例均不能单独证明本期完成。**
> **顶层排除：重新发布未改字节的 0.3.0 包；新增未交付组件、GPU 能力、账号/云同步/AI、文档生成器重构、另建主题编辑器、全文中文翻译。现有组件 demo 升级、新组件 demo 和现有 Studio 公开部署均属必做。**

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

- 最近更新：2026-11-14 +08:00 · 用户发现公开 Studio 底部空白，M7 重新打开
- 当前进度：6/7 个里程碑完成
- 当前状态：进行中；已在公开 S3 复现外层页面多滚出空白，新增 1440/768/390px 回归修前全部失败；根因是 Select 隐藏原生表单镜像缺少所属根的定位上下文。SDK 结构 CSS 修复及生成物已写入，本地 release、8/8 定向浏览器、CSS 漂移检查与底部实拍已过；新候选整包门及上线待完成
- 最近完成：M6 的 R5 修复及原有展示范围；原 M7 的资源校验与已覆盖任务保留历史证据，未覆盖本次根滚动路径，不能继续支撑 M7 完成
- 下一步：冻结含滚动修复的新候选，沿既有授权完成 source CLI/Docs/Studio 整包门、发布及线上受影响终验。
- 当前阻塞：无。registry 0.3.0 归档不变，源码修复的新 registry 发行另行处理。
- 代码基线：新候选 `e0ba61cd`（SDK primitives.css/rustify.css 与 theme-scroll.spec.ts）；当前公开 S3=`29a69cd6`，build `e7bc8812ab87e7d9`。

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-11-10 | 三篇指南进官方导航（A=`53ff0666`）；`rustify-colors` 入清单、十包 0.3.0、模板/锁/常量/预取与措辞齐备（R=`3f2fd1e1`）；V-1、V-2 全过 | [M1.md](DOM-COMPONENTS-RELEASE.records/M1.md) | A=`53ff0666`、R=`3f2fd1e1` |
| M2 | 已完成 | 2026-11-10 | §5.2 冻结与 V-3–V-7 全过（C=`2445aea6`、CLI 候选 `dde6fe85`、runner 修复 `6993fed9`；receipt 十包 0.3.0 冻结 1817 KB；远端预检 run 42833566509 双平台成功） | [M2.md](DOM-COMPONENTS-RELEASE.records/M2.md) | C=`2445aea6`、`dde6fe85`、`6993fed9` |
| M3 | 已完成 | 2026-11-11 | V-8–V-10 全过；十包 0.3.0 公开可取（checksum 全一致）；D=`8d3544bb` 推送公共 main；标签 rustify-libs-v0.3.0→R、rustify-docs-cli-v0.3.0→C 读回；token 撤销暂缓（用户确认） | [M3.md](DOM-COMPONENTS-RELEASE.records/M3.md) | D=`8d3544bb` |
| M4 | 已完成 | 2026-11-11 | 差异门与 V-11 预演（D 与 D3 两轮）通过；部署 run `42953922752`（build 28m39s＋deploy 9s）；V-12 全过（线上 manifest cli_version=0.3.0、build_id `3fe8d486f30d8aa5` 与 run 一致）；V-13 四任务全过（V-6 卡点消除）；E=`458333d5`；worktree 清理 | [M4.md](DOM-COMPONENTS-RELEASE.records/M4.md) | D3=`596175e3`、E=`458333d5` |
| M5 | 已完成 | 2026-11-13 | 展示目录/九分类页/Colors/Primitives/Studio 页、72 demo（旧 24 保留升级）、theme.json（R3 修复后为 schema v2 标准 + Indigo 11 覆盖）、静态 Colors CSS、docs/README 链接；偏差见覆盖记录（catalogue 承载总索引、Select 部件分组（R3 更正）、ThemeBoundary 静态对照、phoenix-ui 替代） | [M5.md](DOM-COMPONENTS-RELEASE.records/M5.md) | `624ea1e3`；修复 `c3605c2e`；UI修复 `a6493064`、`042ccca5`、`af73bfcd` |
| M6 | 已完成 | 2026-11-14 | compose.py/workflow/verify.py check_studio/serve.sh RUSTIFY_STUDIO_DIR/official.spec V-14+V-15 场景/ComposeTests/theme-studio 接线；原候选 S=`45d5d514` 冻结；R3 修复后批门 V-16 全过、V-17 三项目各 66/66；R5 F-14 修复及三个定向浏览器场景通过 | [M6.md](DOM-COMPONENTS-RELEASE.records/M6.md) | `45d5d514`；修复 `c3605c2e`；UI修复 `a6493064`、`042ccca5`、`af73bfcd`；修复 `824ae589` |
| M7 | 进行中 | 2026-11-14 | S3 `29a69cd6` 本地整包及 run `43497922320` 成功；线上 build `e7bc8812ab87e7d9` 全 700 文件比对通过；V-19 三段和最新导出/许可/registry 消费通过，复用与网络重读如实记录；原收口遗漏根页面滚动；用户发现空白后重新打开，修复验证与新候选上线待完成 | [M7.md](DOM-COMPONENTS-RELEASE.records/M7.md) | `8de588f5`、`6b6bde91`、`cd062cad`；修复 `1e021db3`、`29a69cd6`；最终记录 `bd06a4bd`；滚动修复 `e0ba61cd` |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能 | 用户 2026-11-10 原目标及 2026-11-11 再次确认 | 首页、组件索引、具体 demo 与公开 Studio 完整展示 P1–P5 已交付成果；读者可以找到并实际操作，不能只看到更新后的版本说明 | §5.5–§6、ADR-2；V-14–V-19 | 已确认 |
| R-2 | 功能 | 用户 2026-11-10 选定「先发 crates.io 新版再部署」 | 十个包以 0.3.0 公开发布，各包 checksum 与冻结归档一致，CLI 的 registry 配对指向 0.3.0 三包；Pages 在公开门通过后才部署 | §5.1–§5.3、D1–D3；V-4、V-7–V-10 | 已确认 |
| R-3 | 功能 | 用户 2026-11-10 选定「三篇都加入 Guides」 | `docs/menus.md`、`docs/panels.md`、`docs/dom-components-migration.md` 进入 `rustify-docs.toml` 的 include、英文与中文 Guides 导航；官方站 spec 的固定计数同步 | §1.2（A）、D5、§5.4；V-1、V-12、V-13 | 已确认 |
| R-4 | 功能 | 用户「把我们做的东西充分展示」；[PRD §2.3、WORK-F1/AC-1–AC-3、WORK-F5–WORK-F17、WORK-F22–WORK-F23](../PRD-DOM-COMPONENTS.md#23-组件覆盖表) | 按 §6.2 全量覆盖，保留旧路由并升级样例；每个已交付组件/部件有可定位的真实样本、适用的变体/尺寸/状态、公共 API 与运行示例源代码；Primitives 和 Themes 使用同一受控行为 | §5.5、§6.1–§6.3；V-14、V-15、V-17、V-19 | 已确认 |
| R-5 | 功能 | 用户完整展示目标；[PRD WORK-F2–WORK-F4、WORK-F18–WORK-F21、WORK-F19/AC-1–AC-3](../PRD-DOM-COMPONENTS.md#3-需求与验收) | Colors 色阶可查看与取用；现有 Studio 在同一 Pages 公开，保留预设、主题配置、精调、完整 DOM 陈列、Inspector、本地库、导入导出与撤销重做 | §5.6、§6.2–§6.4；V-15–V-19 | 已确认 |
| R-6 | 验收 | 用户 2026-11-11 原目标；[PRD §6.3](../PRD-DOM-COMPONENTS.md#63-uat-预期状态) | 仅凭公开官网完成选组件、比较样式与状态、键盘操作、编辑并导出主题、在独立 registry 应用消费；逐项记录可见结果与卡点，任何必展示项缺失都不能记整期完成 | §9.1、§10 M7；V-18、V-19 | 已确认 |
| C-1 | 约束 | [PRD §5.2 Q3](../PRD-DOM-COMPONENTS.md#52-待落实项) | 接入说明须写明新导出格式（主题 schema v2）的最低消费者版本 | §5.3 的 D 提交（themes.md、迁移指南、README）；V-10、V-13 | 已确认 |
| C-2 | 约束 | AGENTS.md「完成标准与发布」；[docs/crates-io-release.md](../crates-io-release.md) | 发行走 runbook 与 `scripts/crates-io.py`；每包归档绑定 source_rev；框架三包 docs/dom/components 版本同步移动；Python 3.12；构建一律 mbx | §5.1–§5.3；V-4、V-9 | 已确认 |
| C-3 | 约束 | docs/crates-io-release.md「Remote preflight」「Account and authorization gate」 | 推送 main + dispatch、上传（包名与版本）、公开说明与标签推送分别取得用户授权；临时 token 只记账号/权限/过期，不记值；结束后撤销 | §5.2、§5.3、§5.4、§8、§11；V-7、V-8、V-10、V-11（M4 派发门） | 已确认 |
| C-4 | 约束 | AGENTS.md「浏览器测试」 | Playwright 每条命令只带一个 `--project`，串行执行，不同时运行两个 | §9.4；V-1、V-5 | 已确认 |
| C-5 | 约束 | AGENTS.md「工具链与本地开发」 | 不新开分支、不升依赖 major；提交直接落 main | §10 提交点 | 已确认 |
| C-6 | 约束 | `.github/workflows/docs-pages.yml` | 只部署显式给出的完整候选 SHA；候选必须在 origin 上可检出；Pages 已用 Actions 源（[docs-ssg.md 官方站交付](../docs-ssg.md#official-site-delivery)） | ADR-1；V-11、V-12 | 已确认 |
| C-7 | 约束 | [PRD §1.3](../PRD-DOM-COMPONENTS.md#13-非目标) | 沿用 Rustify Docs 的页面/demo/SSG、搜索和现有 Pages 流水线；增加本期展示页面与 Studio 整包组合；用户 2026-11-13 允许 UI R1 必要 SDK/Docs 本地缺陷修复（见 §1.2），新版本发行另行处理 | §0.5、§1.2、§5.6；V-16–V-18 | 已确认 |
| A-1 | 假设 | [M3 公开发行记录](DOM-COMPONENTS-RELEASE.records/M3.md) | 原上传名称与凭证前提已在十包 0.3.0 发行中核实；M5–M7 不再依赖 registry 上传凭证 | V-8、V-9 原证据复用 | 已解除 |
| A-2 | 假设 | [M4 成功部署记录](DOM-COMPONENTS-RELEASE.records/M4.md) | source 分发 CLI 已在 run 42953922752 完成 prepare/build/deploy；新增 Studio 的组合交付另由 V-16、V-18 验证 | §5.6–§5.7；V-11、V-12 原证据 | 已解除 |
| A-3 | 假设 | [M2 浏览器发行矩阵](DOM-COMPONENTS-RELEASE.records/M2.md) | 原 0.3.0 发行矩阵已有记录；新增页面与公开 Studio 不能沿用旧矩阵判通过 | §9.2；V-14–V-19 | 已解除（原范围） |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 版本号 | 十包统一 **0.3.0**：workspace `version = "0.3.0"`，CLI 自有 `version = "0.3.0"`，所有内部 `=0.2.1` 钉改为 `=0.3.0` | 0.x 下破坏性 API 按 Cargo 语义升 minor；五个源码未变的包（classification-macros、style、docs-syntax、docs-macros、style-wasm）也升 0.3.0，没有 API 变化，只是版本号变 | PRD 允许破坏性调整；`scripts/crates-io.py::library_inputs_changed` 把仓根 `Cargo.toml` 算作每个库的输入，而它自 R `b32844a4` 起已改（新增 `crates/rustify-colors` 成员），脚本会要求所有库升版；CRATES-IO-P1 §5.4 的统一版本先例 |
| D2 | `rustify-colors` | 作为第十个包进入 `release/crates-io.toml` 的 docs 组，排在 `rustify-dom` 之前，`dependencies = []` | 首次上传新名称，需要 `publish-new`；`rustify-dom` 的依赖可从 registry 解算 | `inspect` 当前报 `private workspace package permits publishing: rustify-colors`；`crates/rustify-dom/Cargo.toml` 依赖它 |
| D3 | 归档复用 | `package-libraries` 不带 `--reuse`，按首发方式重建全部九个库；不运行 `plan-libraries` | D1 下所有库都会移动版本，没有可复用的归档；`plan-libraries` 必须有 `--reuse` | 上一发行目录 `~/Documents/rustify-releases/crates-io-0.2.1-20261024/release-R1` 已不存在（2026-11-10 核对）；`crates-io.yml` 本就不带 reuse |
| D4 | Pages 候选 | M1–M4 的候选 D3 已上线；M5–M7 用含全部展示源码的新候选 S，同一 workflow 构建文档与 Studio | 库 R/CLI C 包字节不变；来源 S、framework R 与组合 build_id 分别记录；不重新发布 0.3.0 | ADR-1、ADR-2；§5.7 |
| D5 | 指南导航位置 | 三篇插在 `components.md` 之后、`vellum.md` 之前，顺序为 menus → panels → dom-components-migration；include、英文 Guides、中文「指南」三处一致 | 组件参考之后紧接组合指南与迁移；中文导航沿用现有英文内容回退 | `rustify-docs.toml` 现有顺序；本地 `check` 0 error（§1.1） |
| D6 | 验证范围 | 原发行矩阵保留；M5–M6 同批做官网场景与 Studio 子路径验收，M7 只对同候选部署完成线上读者/开发者完整任务 | 按受影响入口验，不因补展示重发包或重跑无关发行矩阵；原人工 missing 不冒充通过 | §9 V-14–V-19 |
| D7 | 首页与公开说明的版本文案 | 仓根 README（官方站首页引言）、`site/src/pages.rs` 的中文首页段落和各指南的 0.2.1/「source-only」说法，都只在上传成功后的 D 提交中改；包内 README 和模板在 R/C 前改 | 上传前 main 不宣称尚未发生的发布；包内文案随归档冻结 | CRATES-IO-P1 §5.1 的同一做法；runbook「Public completion」 |
| D8 | 完整主题工作台 | 公开现有 Theme Studio，保留已交付功能并把其当前消费说明更新为 registry 0.3.0 / schema v2 | 不另写一个删减编辑器；已有 GPU 模式仅复用，官网组件文档仍为 DOM | ADR-2；§5.6、§6.4 |
| D9 | 发布失败处理 | M5–M7 先本地整包验证，再授权部署；构建/上传失败保持当前 0.3.0 站点，部署后缺陷修复后重发完整候选 | 静态展示无远端用户数据写入，不新增版本恢复状态机；原 M1–M4 回滚证据保留在 M4 记录 | §8、V-18、V-19 |

### 0.3 ADR-lite

#### ADR-1：Docs 使用候选 Git CLI，库与 CLI 的发行字节保持配对

- 状态：Accepted。
- 驱动：R-2、C-2、C-6。线上文档源码可以继续变化，已经公开的 0.3.0 包不能改字节。
- 决策：沿现有 `docs-pages.yml`，安装完整候选 SHA 的 source 分发 CLI，prepare 只投影三条框架依赖与对应 lock；保留 framework R 的身份。M5–M7 使用候选 S 的 source CLI 和同 checkout 的框架路径依赖；原发行 R/C 与 registry 0.3.0 字节保持不变。用户已授权的必要 SDK/Docs 本地修复可以进入官网 source 构建；包括 M7 冷构建暴露的 CLI 一次重新分析修复，不能宣称已在 registry 发布。
- 备选：改成 registry CLI 与自定义投影会改变已验证发行链路；本期无须为展示重做它。原源码配对、上传与部署证据见 M2–M4 记录。
- 后果：候选 S、framework R、CLI 0.3.0 与组合包 build_id 分别记录；不能只看 CLI 版本推断展示源码新旧。
- 重新评估触发：本期真实展示暴露必须修改库/CLI 的缺陷；先按 §8 明确受影响包和新版本，再修改本决策，不悄悄使用未发布接口。

#### ADR-2：官网组件文档与完整 Studio 共同展示成果

- 状态：Accepted（用户 2026-11-11 明确要求充分展示；具体接线为本计划技术决策）。
- 驱动：R-1、R-4–R-6；官网不能停在发包说明与旧演示，主题创作也不能只剩截图或源码链接。
- 备选：A＝只扩充官网组件/demo，并在其中另做完整主题编辑、Inspector、保存与导出；B＝官网组件/demo 展示可用 API，现有 Studio 原样公开并互相导流。两者均可行；A 的单页体验更统一，但会复制 Studio 的状态、持久化与导出规则，或迫使本期抽取跨 workspace 公共模块；B 增加一个静态应用入口和组合步骤，保持已有工作流与独立实例边界。
- 决策：选 B。本站继续用 Rustify Docs；Studio 发布在 `/rustify-ui/theme-studio/`，以既有组件陈列与 Inspector 承担完整创作体验。官网自有 demo 从 Studio 中复用已验证的受控组合写法，不直接依赖 `StudioState` 或 Makepad，不抽取新 SDK 包。
- 后果：读者可直接试用全部成果；文档和 Studio 有各自的主题/语言/本地状态，不能暗示跨应用自动同步。文档普通页面不加载 Studio wasm；进入 Studio 才加载其既有 GPU 集成。
- 重新评估触发：已存在的静态构建不能在同一 Pages 子路径运行时，先修路径/资源接线；不得因此缩成代码展示或删掉 Studio。SDK 缺陷按 §8 的真实问题边界处理。

### 0.4 职责与事实所有权

|  | crates.io / docs.rs / GitHub | 本计划 | 发布人（用户） |
| --- | --- | --- | --- |
| 拥有 | 包名归属、版本不可变、索引、限流、docs.rs 构建；Actions 运行器与 Pages 托管 | 原发行 R/C/D 与证据；本站组件/demo、完整展示映射、Studio 公开接线、组合产物、用户主路径验收 | 账号、凭证管理、生产推送与 Pages 派发授权 |
| 不拥有 | — | **不保存或打印 token；不 yank；不改 Pages 仓库设置；不向 registry 发布 GPU SDK/Makepad/示例** | — |

一句话：本计划负责本站与现有 Studio 的展示、组合及交付；组件契约仍由 SDK 拥有，registry 字节不可改，生产推送与派发仍由发布人授权。

### 0.5 明确不在本期

- **新增组件、GPU 绘制能力及 GPU SDK/Makepad 的 registry 发布**——只展示已交付能力；Studio 使用现有 GPU 可选视图，不补 DOM/GPU 对等组件。
- **替换现有 Studio、账号/云同步/AI/社区功能**——公开现有本地主题工作流，不另建编辑器或服务。
- **重做 Rustify Docs 生成器、搜索或通用托管系统**——复用既有 SSG、注册与搜索；Pages 组合脚本只组装已构建的本站与 Studio。
- **重发未变化的十包、补齐 PRD 其他计划尚未实现的能力**——示例调用公开 0.3.0 能力；发现真实 SDK 缺陷按 §8 处理，不能悄悄用源码新 API 伪装为已发行。
- **英文指南全文翻译、性能/包体积专项优化、Windows/完整 Linux 运行认证**——保持现有平台边界；新增导航、说明与操作控件有中英文，英文内容可显式回退。

组件目录扩展、旧 demo 升级、Colors/Primitives 展示、公开 Studio 和线上完整任务均在本期，不能以原 24 类导航、原排除或发行已完成为由省略。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** 十包 0.3.0 已公开，原库 R=`3f2fd1e18e48b0cf76c634512411dfb0493d35d0`、CLI C=`dde6fe85149c7f362f458a8a3139257fd0984c67`；上传、配对与平台记录见 [M2](DOM-COMPONENTS-RELEASE.records/M2.md)、[M3](DOM-COMPONENTS-RELEASE.records/M3.md)。没有必要重新发包。
- **[已核实·足够]** 现网 manifest 为 CLI 0.3.0、build_id `3fe8d486f30d8aa5`；Menus、Panels、迁移三篇已上线。候选 D3=`596175e3550794fd83683e433702bea0534b509c`，run `42953922752`（[M4](DOM-COMPONENTS-RELEASE.records/M4.md)）。
- **[已核实·缺口]** `site/src/lib.rs::site` 只注册 24 个 demo 和对应页面，`site/src/pages.rs::category` 依赖 SDK 旧 `CATALOG` 分类；该分类不是完整公共能力清单 → 增加本站展示目录和新组件入口，保留旧地址 → 否则 Accordion、Menubar 等只有指南代码，读者无法操作。
- **[已核实·缺口]** `site/src/demos/basics.rs::button_demo` 仅演示 Increment/Reset/Unavailable；`feedback.rs` 沿用简单 Menu/Dialog。该目录在本期发行区间未改动 → 用当前公开 API 升级样例，覆盖适用变体/尺寸/状态 → 否则新库重建后仍像旧组件。
- **[已核实·足够]** `examples/theme-studio/src/gallery.rs`、`gallery_catalog.rs`、`gallery_extensions.rs` 已有受控样例、部件说明和九组陈列；`tests/browser/theme-gallery.spec.ts` 已有真实操作范式。此处是复用写法与覆盖的证据，不等于官网已展示，也不以既有 49 样本数代替 §6.2 的逐项覆盖。
- **[已核实·足够]** Studio 的 `app.js` 使用相对 `theme-studio.wasm`，`xtask/src/build.rs::build` 支持 `--base`，复制 loader、静态 message bridge、snippets、public 字体与图片；无需新服务。需在本地验证公开子路径与 meta-only CSP，不能仅由源码推断通过。
- **[已核实·缺口]** 现有 Pages 只打包 Docs output；`scripts/docs-pages-verify.py::check_artifact` 拒绝 manifest 外文件，`verify` 要求每个 HTML 带 strict meta CSP → 新增本站专用组合步骤，将完整 Studio 与 Docs 一起进入既有 manifest/identity/打包校验 → 不能部署后手工塞一个未绑定候选的目录。
- **[已核实·足够]** `scripts/docs-pages-prepare.py::prepare` 对候选全树保护，只投影 `site/Cargo.toml` 和 lock。新增 `site/` 源码自动进入保护，不需改 prepare 的身份协议。
- **[已核实·足够]** `rustify_colors::ColorFamily::ALL/scale/css` 提供完整命名色阶与原始值，`crates/rustify-colors/README.md` 明确 31 色族、12 阶、浅深/透明/P3 及黑白透明；P3 是独立静态资源能力，共享主题与 GPU 仍按 sRGB 解析。
- **[已核实·足够]** `ThemePanel`、`ThemeBoundary`、主题 codec 与 Studio 的配置/精调/导出已存在；不为展示再造主题事实源或持久化格式。Studio 继续使用其独立 `rustify-ui.theme-studio.v2` 本地 key。
- **[已核实·缺口]** `tests/docs-ssg/official.spec.ts` 从旧 CATALOG 推导页面并假设一个 demo island；需扩展到本站展示目录与一页多个具名样例的实际公开行为，保留旧路由、语言、静态页面/按需 wasm、CSP、搜索和来源代码的保障，不能只改通过计数。


### 1.2 拓扑与文件清单

M1–M4 已完成发行的文件与契约保留如下；续做改动见本节表后及 §5.5–§6，不重复运行发包步骤。

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

续做改动（★ 为新增文件）：

| 路径 | 行为与复用点 | 执行范围 |
| --- | --- | --- |
| `site/src/lib.rs`、`site/src/pages.rs`、`site/src/showcase.rs` ★ | 本站展示索引/分类/条目，继续注册具名 demo 与自定义静态页面；不修改 SDK 的 24 类 CATALOG | M5 |
| `site/src/demos/{basics,choices,fields,feedback,collections}.rs`；`site/src/demos/{composition,menus,content,layout,typography,colors}.rs` ★ | 升级已有 demo，补全 §6.2 的公开能力与部件；按现有 `pub fn ...() -> impl IntoView` 注册，参考 Studio 受控样本 | M5 |
| `site/Cargo.toml`、`site/Cargo.lock`、`rustify-docs.toml`、`site/theme.json` ★ | 仅给本站增加 `rustify-colors = { version = "=0.3.0", features = ["p3"] }` 的直接消费、导航与 schema v2 主题；样式使用现有无前缀 Tailwind utilities，由 Docs 编译；主 CLI 配对不变 | M5 |
| `site/static/colors/` ★ | 从已发行 `crates/rustify-colors/css/` 原样复制色阶 CSS 与完整 `vendor/colors/LICENSE`（发布名 `LICENSE.txt`），供静态 CSS 下载；不从忽略的 ref 构建、不把数据转换成另一份色表 | M5 |
| `docs/{components,themes,menus,panels,forms,quickstart,dom-components-migration}.md`、`docs/zh/quickstart.md`、`README.md` | 从说明与首页链接到真实样例、Colors 与公开 Studio；当前可用版本一致 | M5–M6 |
| `examples/theme-studio/index.html`、`app.css`、`src/data_tools.rs` | 仅增加公开文档往返入口、更新 0.3.0 消费提示及必要的公开子路径适配；保留现有编辑、保存、导出和 GPU 功能 | M6 |
| `scripts/docs-pages-compose.py` ★、`.github/workflows/docs-pages.yml`、`scripts/docs-pages-verify.py` | 组合两份已构建静态目录，复用既有产物身份与校验；整包部署和对照，不重构 prepare 或生成器 | M6 |
| `tests/docs-ssg/official.spec.ts`、`tests/docs-ssg/serve.sh`、`tests/browser/theme-deployment.spec.ts`、`tests/crates-io/test_pairing.py` | 官网真实用户旅程、组合包部署、既有协议回归；不把演示覆盖降为数字断言 | M5–M7 |
| `docs/validation/dom-components-release/showcase-coverage.md` ★、本计划 M5–M7 记录 | 实施时写真实展示落点与验收结果；不预填通过 | M5–M7 |

不改 Makepad/GPU 绘制、SDK 组件 API 或已发布 registry 归档。 M7 实际接入发现 `makepad/widgets/resources/FONT-LICENSES.md` 两个许可/源码下载链接失效，允许只修正为已核验的不可变公开归档并刷新同候选 Studio 资源；不改字体或绘制代码。用户 2026-11-13 授权 UI R1 必要 SDK/Docs 本地修复：scope 内模态完整可读、默认色族标签达到 AA、源码 tabpanel 使用有效容器；不扩大为生成器重构。新版本发行另行处理，修复后的本地源码行为不代表 0.3.0 registry 归档已更新，处置及用户原话见 [M6 UI 修复](DOM-COMPONENTS-RELEASE.records/M6.md#ui-验收修复)。

## 2. 本站展示目录与页面契约

`site/src/showcase.rs` 仅供本站使用，定义 `ShowcaseEntry { group, route, anchor, title, demo, public_paths, guide }` 的静态条目；字段均为静态字符串/切片，不生成运行期路由或查 registry API。`route + anchor` 是用户可分享位置，`demo` 对应 `site/src/lib.rs` 已注册函数，`public_paths` 显示实际 `rustify_components` / `rustify_dom` 公共路径。该目录服务首页、分类、样例页面三个真实调用者，不加入 SDK 或通用组件包。

- 既有 24 个 `/components/<name>/` 地址保持，在其中升级真实样例；同类新部件可在其页内有独立锚点，不另造一个 Props 类型页面。
- 新 `/components/` 是总索引；新增分类页 `/components/interaction/`、`/components/overlays/`、`/components/menus/`、`/components/navigation/`、`/components/content/`、`/components/feedback/`、`/components/layout/`、`/components/typography/`、`/components/extensions/`。每个 §6.2 项有条目锚点；旧组件条目链接回旧详情页，新家族在分类页用 `context.demo(entry.demo)` 渲染。分类内允许多个具名 island，不能用“存在一个 island”作为整页完成条件。
- `/colors/` 为色阶实物与消费入口；`/primitives/` 给出无样式组合与 themed 对照和各家族入口；`/studio/` 为可搜索的工作台介绍页，`/theme-studio/` 是公开完整应用；Docs 内另有主题介绍链接，不能把该应用伪装成静态指南。
- 分类页和源码在 SSG 时生成，可索引/搜索/深链；交互按现有 Docs island hydrate 接管。同一示例的状态由其 `RwSignal` 拥有，部件不持有另一份业务值；重新挂载恢复样例初态。
- `site/src/lib.rs` 沿既有宏声明方式注册新静态页与 demo；`pages.rs` 复用 `RenderContext::demo`。源码展示继续由实际注册函数产生，不能贴一份与运行样例分叉的代码。
- 控制台和展示区域分开；只暴露该 API 已支持的参数，不给不存在的功能增加空控件。函数复用受控示例组合，不把 Studio 应用状态搬进 docs，不为共享代码改 SDK 包。

## 3. 数据与版本边界

组件 demo 的数据只驻留当前 island，不写用户本地库或示例业务文件。导入使用用户主动选取的文件；演示下载使用明确命名的演示文件。Studio 沿既有本地 key、schema v2、备份和存储冲突处理；官网文档不读写 Studio key。JSON/CSS/Rust 导出沿现有 codec/data_tools，不新增交换格式。

0.3.0 已发布字节不变。本站增加 Colors 的公开依赖并由锁正常解析；示例公共 API 取自 R 中已发行能力；UI R1 的必要 SDK/Docs 本地修复按 §1.2 执行，不能将其行为等同于现有 registry 归档。Studio 从相同候选 S 的现有应用代码构建，保持其 Git-only GPU 来源说明，不能写成可从 crates.io 安装 GPU SDK。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| crates.io registry | 已核实足够（已完成发行） | M3 记录十包 0.3.0 公开与 checksum 核验 | M5–M7 仅消费已发行版本，不重新上传 | 发布字节不可变；真实 SDK 缺陷依 §8 重新界定版本 |
| docs.rs | 已核实足够（原发行） | M3 记录九库 0.3.0；CLI 为 binary-only | 官网公共 API 链接匹配 0.3.0，CLI 继续链接公开 README/help | 链接错误修本站；不能据版本号声称新展示完成 |
| 发布人账号与临时 token | 已核实足够（原发行） | M3 上传完成；用户已决定保留 token | M5–M7 不访问该凭证、不重新上传 | 新上传不在默认执行范围 |
| `crates-io.yml` 远端预检 | 已核实足够（原发行） | M2 记录预取修复及双平台成功 | 原源码/包字节未变则复用，不作为 M5–M7 新增上传门 | 不以原矩阵通过替代本站展示验收 |
| `docs-pages.yml` + prepare/verify 脚本 | 已核实足够（原 Docs）；已核实缺口（组合 Studio） | §1.1、M4 | 保持 prepare 配对，按 §5.6 增加两份静态输出组合与整包校验 | 组合/校验失败则不上传、不部署，修复后安全重做 |
| GitHub Pages 环境 `github-pages` | 已核实足够 | 线上可读，Actions 源已配置（docs-ssg.md） | 不改设置 | — |
| 现有 Studio | 已核实足够（本地功能）；公开子路径需验 | `examples/theme-studio/app.js`、`xtask/src/build.rs`、现有 gallery/deployment 测试 | 同候选构建完整应用，公开在固定子路径；保持存储命名空间与既有 DOM/GPU 边界 | 原 startup/fatal/storage 错误如实可见，不用静态截图替代失败应用 |
| GitHub Pages 生产派发权限 | 已有原发行授权记录，新的候选尚未派发 | M4、C-3 | M7 在本地完整可审产物通过后展示 S、组合 identity 和准确命令，再申请本次授权 | 不获授权保持 M7 未完成，不把本地通过写作上线 |

## 5. 核心机制：发行配对、完整展示与整站部署

§5.1–§5.4 是 M1–M4 已完成发行交付的契约；当前续做从 §5.5 开始，不重新发包或重复原矩阵。

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

### 5.5 官网 demo 与完整展示映射

1. 在 `site/src/showcase.rs` 登记 §6.2 的每个已交付家族、旧详情页/新分类页锚点、具名 demo 和公共 API 路径；以 `docs/components.md` 的 Public source entries、PRD §2.3、Studio `gallery_catalog.rs` 三者交叉核对。数量不相等不等于缺陷：helper/Props 在所属家族说明中展示，不能凑空 demo；任何有独立可观察行为的组件不能遗漏。
2. 首页同时给出 Components、Colors、Primitives、Theme Studio 入口；组件总索引按九组分区，列出具体名称，不让用户只能搜索一个旧分类词。分类/详情与 Guides 双向链接。
3. 在既有 24 页原地升级 demo；新家族在分类页有独立锚点、标题、真实样本和源码。Button 至少可切换全部 `ButtonVariant::ALL`、`ButtonSize::ALL`、颜色/高对比、disabled/loading；其它控件只提供自己公开支持的维度，使用相应枚举 `ALL`，不维护另一份手写选项表。
4. 扩展 `official.spec.ts` 的页面获取：既有 CATALOG 场景仍保护旧路由，新增来源是本站展示目录/注册页；每个具名 island 单独等待 ready，不再全页强制只有一个。操作断言检查用户状态和实际控件结果，不以 `data-variant` 等内部标记代替可见效果。
5. 实施时填写 `docs/validation/dom-components-release/showcase-coverage.md`：`公开能力/交付来源 | 页面+锚点 | 可操作行为/视觉维度 | 示例源函数/公共路径 | 真实结果/证据`。可合并同家族部件，但每个公开能力必须有落点；未实现、只写文档、无法操作分别标缺口，不标完成。
6. 本站使用 `site/theme.json` 的 schema v2 标准主题（依据 `ThemeDocument::standard` 的完整两模式数据），在 `rustify-docs.toml` 用 `theme.document` 声明，不以旧 legacy 默认色板代表新主题系统。Docs 壳以 `--primary` 作链接色，标准 indigo 第 9 阶在深色背景与浅色强调底上不足 4.5:1，因此另以 v2 overrides 将两模式 `primary`/`primary-foreground` 设为 Indigo 第 11 阶（light `#3a5bc7`/`#ffffff`，dark `#9eb1ff`/`#11131f`；用户 2026-11-13 决定）。页面主题切换与 SSR 初始外观一致；不为了本站展示修改 Docs 的 CSS 编译器。

### 5.6 现有 Studio 的公开接线与静态产物组合

- 应用入口固定为 `/rustify-ui/theme-studio/`；根路径验收时为 `/theme-studio/`。从首页、主题指南和组件页可到达，Studio 提供返回官网的普通链接。`index.html` 使用 `href="../"` 的普通返回链接，随根/子路径自然回到官网；不把 docs 组件嵌入 Studio、不跨实例共享内存。
- 从同一候选 S 的未投影 checkout 执行 `mbx xtask build-web --example theme-studio --release --base /rustify-ui/theme-studio/`，使用 `xtask` 返回的产物目录，复制到仓外稳定目录后才接后续检查；根路径产物另用 `--base /theme-studio/`。不猜 `@...` 目录名。保持 `app.js`、loader/bridge、snippets、CSS、字体、图片与 WASM 完整配套。
- 新增 `scripts/docs-pages-compose.py`，接口固定为 `--docs-dir <已构建Docs> --studio-dir <完整Studio> --out <不存在或空的仓外目录> --base /rustify-ui/`。用 Python 3.12 标准库；输入分别读既有 Docs manifest 和 Studio build-manifest，要求 base 为 `<base>` 与 `<base>theme-studio/`，不在组合阶段编译应用。
- 组合器先验证输入路径/清单/字节，再复制 Docs 全部文件到 out、Studio 完整文件到 `out/theme-studio/`；输出不得覆盖任一输入，路径冲突/缺资源/错误 base 直接报错。失败的临时 out 可删除后重试，不写事务日志或恢复状态机。
- Studio HTML 在构建产物中加入与 `scripts/docs-pages-verify.py::META_CSP` 相同的 meta CSP；保留模块脚本和外置资源，不加入 unsafe-inline/eval。组合不改候选源码或输入产物；Docs HTML 除下面明确的 Studio 链接本地化外字节不变。
- 按 `crates/rustify-docs/src/publish.rs::Manifest::of` 的**既有**算法对最终文件集重算 `files` 和 `build_id`（排除根 manifest 自身），保留 schema/base/cli_version/locale_not_found/page_policies。只使用既有 sha256 和 build_id，不增加第二套指纹或协议。Studio HTML 在已有返回链接之外加入 CSP 后，按 `xtask/src/build.rs::finish` 的既有文件清单结构更新 Studio build-manifest 的 `files: BTreeMap<String,u64>` 与相应 `size_report` 大小信息，WASM build_id/schema_hash/base 保持实际构建值；再纳入根 manifest。不能让根 manifest 正确却保留一份与 HTML 不符的嵌套清单。
- Docs 构建期还没有 Studio 文件，`crates/rustify-docs/src/generation.rs` 的最终链接检查会拒绝尚未发布的内部 `/theme-studio/`。因此站点源码/导航/指南先以绝对官网 URL `https://rockie.github.io/rustify-ui/theme-studio/` 链接完整应用，另注册可搜索的 `/studio/` 静态介绍页；不伪造应用页面或放空 iframe。组合器只把这个明确 URL 在生成 HTML 的 href 中本地化为 `<base>theme-studio/`，使 root/subpath/meta 与线上均访问同包应用；不得泛化重写其它外链。组件/指南的索引搜索仍由 Docs 生成，Studio 的动态 UI 不冒充静态搜索内容。
- workflow 仍保留 Docs 的 `build.json`，同时把组合 manifest 写入原 evidence 工件；`identity.json` 的 build_id 指向最终组合包，原 candidate/framework/projection 字段不变。必须在生成 identity、verify、archive **之前**完成组合；最终 tar 从组合 out 打包，不能继续打包原 Docs 目录。把 workflow 的 docs-site 固定路径改为该明确输出，不更改 GitHub Pages 环境设置。
- `docs-pages-verify.py` 保持完整文件集合、资源字节、地址、MIME、meta CSP 与语言/搜索/AI 检查；增加 Studio 必需入口及其 build-manifest base/依赖完整性核对，不把 Studio 当 Docs 内容页要求侧栏/搜索。`prepare.py` 继续保护全部候选源码，不需新增投影。
- `tests/docs-ssg/serve.sh` 的 official 分支复用组合器。通过 `RUSTIFY_STUDIO_DIR` 指向按相同 base 已构建的 Studio，由调用方预构建，缺失直接失败；依现有 root/subpath/meta 三种宿主服务同一个组合包。`official.spec.ts` 中覆盖公开 Studio 的场景通过同一配置运行，不在验收时偷换成本地独立端口。

### 5.7 新候选 S 的部署顺序

1. M5–M6 同批实现并完成 V-14–V-17；本批 site、Studio 接线与组合脚本可分别提交，但共用文件由一人集成。提交后冻结完整 S，回写 M5/M6 记录，不能继续使用 D3 的源码身份代表新展示。
2. 检查 S 相对 R 的库及根输入差异仅为已授权并经 R4–R6 验证的 SDK/Docs 修复，以及用户本次反馈的 Select 镜像滚动定位修复（M7 红绿回归与定向浏览器验证），CLI 包相对 C 仅增加本次真实冷构建失败所需的一次重新分析修复及回归测试；核对 source CLI 的候选身份及 prepare 投影的三条路径依赖都指向同一 S。原 registry 归档不变，其 framework R 身份与官网 source S 分别记录；独立 registry 消费只验 0.3.0 已有契约。按现有 Git CLI 安装与 prepare 命令做完整候选预演，再从 S 未投影源码构建 Studio、组合/verify。由于 prepare 会投影 site manifest，不能从投影目录启动根 xtask 构建 Studio。
3. V-18 的本地候选门通过后，出示 S、最终 build_id、主要可操作入口与生产命令；获本次推送/派发授权后推送并执行 `gh workflow run docs-pages.yml --ref main -f candidate_sha="$S"`，再 `gh run watch`。没有授权不派发，但其它已授权本地工作照常完成。
4. 用同 run 工件与线上 HTTP 核对完整组合包，再在真实官网执行 V-19。任何缺失演示、不可用状态、资源错误或导出消费失败均是未完成；修复后只重验失败任务与波及项，并发布新 S。
5. V-19 全部通过后，M7 才能记完成、整期进度才可成为 7/7；记录实际 S/run/build_id、展示覆盖和已确认的限制。只有新版文档上线、Studio 启动成功或截图好看都不够。

## 6. 前端与展示验收契约

### 6.1 信息架构与页面行为

首页首屏清楚呈现三层能力（Colors / Primitives / Themes）与“浏览组件 / 打开 Theme Studio”主入口，辅以真实组件构成的短预览；不把原 Git/GPU 定位或发行说明堆成唯一主体。组件总索引按 §2 九组列出完整集合，当前分类/面包屑与页面标题一致；移动端导航可用。旧 24 页可直接访问且已有链接不失效。

详情结构统一为：组件用途与使用条件 → 可操作样例和适用参数 → 关键状态/组合 → 实际源码 → 公共路径/部件结构/相关指南 → Studio 对应场景入口。部件与类型辅助说明跟随所属家族，不用导出符号总数冒充用户可操作组件数。没有业务行为的布局/排版以真实语义、响应式和视觉变化证明，不能放假操作按钮。

### 6.2 必须完整展示的能力矩阵

下面是交付集合，不是抽样清单；来源为 PRD §2.3、当前公共组件清单与已有 Studio 样本。每行都要在覆盖记录中有页面/锚点和实际结果，逐个家族核对；相同根部件与便捷组件可组合展示，不能删除。

| 组/展示落点 | 已交付能力 | 读者必须能实际看到或完成的事 |
| --- | --- | --- |
| Colors `/colors/` | 全部 `ColorFamily::ALL` 的 12 阶、浅深、实色/透明、黑白透明；sRGB 与 P3 数据/静态 CSS | 选择色族/模式，看到色块、阶号和真实颜色字符串，复制值/下载匹配 CSS；P3 标明浏览器/显示器条件与 sRGB 回退，不暗示 GPU P3 |
| Primitives `/primitives/` 及各家族条目 | 当前公开 raw 部件与 themed 对应；typed native decorators、Direction、Portal、AccessibleIcon、VisuallyHidden | 同类 raw/themed 受控样例都可操作，看到结构、状态及公共路径；示范 RTL、原生按钮/链接语义与组合，不把 Props/helper 单独凑 demo |
| 操作与图标：旧 Button/Link/Icon + Interaction | Button、IconButton、Link、Icon/Glyph | 全部声明的变体/尺寸，颜色、高对比、disabled/loading；点击有结果，禁用/加载不触发，链接保留原生打开方式 |
| 基础选择：旧 Checkbox/Radio/Switch/ToggleGroup + Interaction | Checkbox、CheckboxGroup、Radio、RadioGroup、Switch、Toggle、ToggleGroup、SegmentedControl | 三态、单选/多选、受控回写、方向与禁用；SegmentedControl 不伪装成 Tabs |
| 卡片选择 Interaction | CheckboxCards、RadioCards 及对应 Root/Item/Control/Indicator/Text | 整卡交互、选中/禁用、键盘和表单值；可以对比尺寸/变体 |
| 范围输入：旧 Slider + Interaction | Slider、RangeSlider、公开 slider Root/Track/Range/Thumb | 多滑块、垂直/反向、值/提交、键盘、最小间距及应用拒绝请求时显示值保持 |
| 字段与表单：旧字段页 + Interaction | TextField、TextArea、NumberField、ColorField、Label、Field、FormRoot、SubmitButton、FormStatus 及字段部件 | 前后插槽、只读/禁用/invalid、数字/颜色草稿，真实提交/错误恢复/reset，展示应用最后接受值与原生表单结果 |
| 选择器：旧 Select | Select 的便捷/部件 API、分组、长列表、空值 | 打开/选择/清空、滚动与键盘、disabled 项不可选，正确焦点返回 |
| 模态/浮层：旧 Dialog/Tooltip + Overlays | Dialog、AlertDialog、Popover、HoverCard、Tooltip 的公开部件 | 确认/取消有不同结果，Escape/外点/嵌套与焦点，Popover 内实际操作；Tooltip 用可聚焦原生目标 |
| 菜单：旧 Menu + Menus | Menu、DropdownMenu、ContextMenu、Menubar | 点/右键/键盘入口、复选/单选/子菜单/禁用、菜单间移动、执行命令和焦点返回；沿用指南真实组合 |
| 导航/切换：旧 Tabs + Navigation | NavigationMenu、Toolbar、Tabs/TabPanel、TabNav、Accordion、Collapsible | 网站链接与应用命令区分，工具栏 roving focus，Tabs/展开受控结果、单项/多项选择及键盘操作 |
| 反馈：旧 Progress/Loading/Toast + Feedback | Progress、Spinner、Skeleton、Toast/Toaster | 确定/不确定进度、真实等待/空/完成反馈，Toast 更新/关闭；不只放永远转动的装饰 |
| 内容 Content | Avatar/Image/Fallback、Badge、Callout/Icon/Text、Card、DataList 部件、Table 全部表格部件 | 图片失败回退、可见状态/变体、真实文本和数据；普通 Table 与 DataTable 分清用途 |
| 尺寸内容 Content + 旧 ScrollArea | AspectRatio、Separator、ScrollArea 的 Viewport/Scrollbar/Thumb/Corner | 比例随容器变化、分隔方向、横纵滚动与长内容，不用静态外框代替行为 |
| 布局 Layout | Box、Flex、Grid、Container、Section、Inset、Reset | 响应式间距/对齐/列数/容器尺寸，Inset 与最近容器关系，Reset 作用域不影响页面其它内容 |
| 排版 Typography | Text、Heading、Blockquote、Code、Kbd、Em、Quote、Strong | 原生语义、字级/字重/对齐/裁切/换行等实际支持字段；同一文本随参数变化可见，不以 class 文本代替视觉 |
| Rustify 扩展：旧 DataTable/Tree/字段等 + Extensions | DataTable、Tree、FilePicker、DropZone、Splitter、PanelTabs、CommandPalette；保留 NumberField/ColorField/Icon 对应页面 | 排序/筛选/选择，树展开与键盘，用户主动文件选择/拖放，拖动/键盘调分隔、切换面板、搜索执行命令；演示数据和操作结果可见 |
| 主题与公开 Studio | Theme/ThemeScope、ThemeBoundary、ThemePanel、主题配置、codec 与来源 API | 官网从公开入口编辑主题；比较根/局部/浮层继承，变体与尺寸受主题驱动；完整工作流见 §6.4 |

Colors 色块可用 SVG `rect` 的颜色属性在 SSG 中展示真实 sRGB 值，保留可读数值，不加内联 style 或运行时拼 CSS；P3 数据读 `ColorFamily::css/scale` 的公开 p3 feature，下载链接指向 `site/static/colors/` 中原样打包的 CSS（包括相对导入的 sRGB 文件）与许可证，不使用示意字符串。可交互选择的状态随用户操作更新，内容页不为色块而强制加载 Studio。复制/下载沿浏览器原生能力，失败必须对用户可见且仍能选中原始文本。

### 6.3 状态、主题与可访问性

复用 PRODUCT.md / DESIGN.md 的中性文档壳和作用域主题；预览承载新组件外观。实施前按仓库要求使用 impeccable 做页面设计；当前技能目录未发现该技能，实施者须明确提示安装并如实记录，不能声称已经使用。设计需覆盖视觉层级、信息架构、间距、颜色、动效、可访问性和 loading/empty/error；缺技能不授权改技术栈或缩减展示范围。

每个可操作示例有明确结果和复位；禁用/只读/加载/错误只用于对应语义，不能用它们掩盖未实现交互。页面和 Studio 均测试浅深、中文导航/英文回退、窄屏及放大、键盘焦点。错误资源、CSP/pageerror、空示例与永远 loading 都是失败；保留实际截图与操作结果。文档壳与某个局部主题互不污染，浮层继承创建样例的有效主题。

### 6.4 公开 Studio 的完整使用路径

从官网点击 Studio → 选现有预设 → 修改品牌/中性色族、面板、圆角和缩放 → Colors/Typography/Other 精调 → Components 场景检查对应组件的变体/状态/部件 → Inspector 选真实组件及浮层，看到有效值和根/局部/继承来源 → 撤销/重做 → 保存并刷新恢复 → 导出 JSON/CSS/Rust → 按公开说明在独立 registry 0.3.0 DOM 应用消费。

现有导入、导出、本地库、草稿、错误恢复与独立 scope 对比全部保留；不可替换成一个删减面板。默认读者主路径选 DOM；GPU/对比仍是原有可选模式，显示真实支持与错误，不为“完整展示”增加 GPU 组件。中文入口与语言控件沿现有应用，文档与 Studio 不承诺自动同步编辑状态。

## 7. 质量边界

沿用现有 strict CSP、静态托管、资源外置、无脚本可读与按需 island。官网普通文档不依赖 Makepad；Studio 可选独立应用保留已存在的 Makepad 集成。新增组合 sha256/build_id 只延续现有 manifest 合同。无本期新增性能数值或真人样本指标，不以性能专项替代功能交付；已有人工/平台 missing 继续如实列出，不推导可访问性认证。

## 8. 失败模式、发布与恢复

M1–M4 已完成的上传/冻结/派发故障及处置保存在原 M2–M4 记录，不作为 M5–M7 重新发行的触发。续做只处理当前真实入口的失败：

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 官网仍遗漏已交付家族、样例不能操作、显示旧视觉 | 对应读者任务 | 无 | 看不到或无法评估研发成果 | §6.2 逐项映射与实际操作 | 在 M5/M6 修复，记录缺口；不能按“指南已写”记完成 | V-14、V-15、V-19 |
| Studio 公开子路径资源/CSP 失败，或组合漏打包 | Studio/本次部署 | 不写已有用户数据 | 白屏、启动失败或字体/菜单/下载不工作 | 组合 CLI、既有 verify、同包真实浏览器与失败请求 | 不上传失败组合包；修正接线后重建，失败临时 out 可安全清理重做 | V-16、V-17、V-18 |
| 组合 build_id 或线上候选与同 run 工件不一致 | 本次展示身份 | 无 | 线上可能仍是旧成果或资源混版 | manifest/identity 与公开 HTTP 字节对照 | 保留当前完整站直到候选通过；部署后核验不通过则记 M7 未完成，修复候选重发，不手工补线上文件 | V-18 |
| 导入/保存/导出失败 | 当前 Studio 实例 | 沿既有存储/codec 合同 | 显示真实错误，已有内存编辑与可用导出按原合同保留 | 真实主题任务，刷新/导出文件检查 | 不新增持久化/恢复状态机；按既有错误入口处理，真实产品缺陷修复后重验 | V-19 |
| 独立 registry 示例调用不到源码中使用的接口，或 SDK 真实缺陷 | 公开教程与发行配对 | 0.3.0 字节不可变 | 读者按官网无法复现 | 公共源码示例实际构建与消费 | 先区分站点组装问题与 SDK 缺陷；前者在本计划修复，后者记录影响并按 runbook 修订明确的新版本交付范围；禁止悄悄改包或跳过任务 | V-19 |

M5/M6 实现先本地验，M7 再冻结 S、授权推送/派发、核对整包、真实线上任务、提交最终记录。生产审批、包不可变和全文件校验沿既有合同，低成本静态构建失败可重做；不新增兼容模式、自动回滚或恢复基础设施。原 last-good 身份保留供发布人查证；不得把旧站可用当新目标完成。

## 9. 验证

### 9.1 检查表

V-1–V-13 为 M1–M4 已完成发行范围，证据见原记录，未受影响则复用。当前必需出口为 V-14–V-19；前者不能代替后者。

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
| V-14 | 场景 E2E | R-1、R-4：从首页按分类找到并比较全部交付家族；旧页已升级，新组件有真实入口/源码；按 §6.2 逐项记录结果 | 扩展现有 `tests/docs-ssg/official.spec.ts`：首次读者从 Components 索引进入 Button 调变体/尺寸/颜色并点击，再使用表单/卡片/Slider、浮层/菜单/导航、反馈、内容、布局、排版和扩展各组，观察用户任务结果；页面覆盖遍历核对每个声明条目和部件样本，不用仅计数断言 | M5–M6 全部展示和组合产物齐备后，与 V-15/V-17 同一轮 official-root/subpath/meta 命令执行、各项目串行 |
| V-15 | 场景 E2E | R-5：从官网到 Colors/Primitives/完整 Studio，编辑并检查真实组件；原研发能力公开可用 | 同一 `official.spec.ts` 的连续任务：选色族并查看阶值/复制或下载；操作 raw/themed 对照与 RTL；从公开 Studio 入口选预设、改配置/模式/覆盖、进入 Components 操作新组件、Inspector 检查局部和浮层来源，撤销/重做；检查不是删减编辑器，保持原场景/本地库/导入导出入口 | 同 V-14；使用 official 服务的组合包，不能换成独立 Studio 端口 |
| V-16 | 局部/风险门 | C-2、C-6、C-7：错误 base/缺文件/输出覆盖输入被拒绝，完整组合包与已发行框架配对 | 先写非法输入复现再做组合器；用真实 Docs 与 Studio 产物在临时目录调用组合 CLI，覆盖合法组合、缺资源、错误 base、覆盖输入三类可达错误，失败不修改输入；把检查纳入 `tests/crates-io/test_pairing.py` 既有协议测试，以 `/opt/homebrew/bin/python3.12 -m unittest discover -s tests/crates-io -p test_pairing.py` 执行。新脚本标准库、`python3.12 -m py_compile scripts/docs-pages-compose.py`；`cargo fmt --all -- --check`，本站分别 `mbx test --locked --manifest-path site/Cargo.toml --features ssr --lib` 与 `mbx clippy --locked --manifest-path site/Cargo.toml --features ssr --lib -- -D warnings` | M5–M6 同批退出；组合错误路径属于真实发布边界校验，复用实际构建结果，不造本方 HTML/SDK mock；其它包未改不跑全仓 |
| V-17 | 阶段收口 | R-1、R-4、R-5：整站导航/搜索/源码、SSG→hydrate、CSP/资源与响应式完整；旧路由可用、纯 Docs 不加载 Makepad | 预构建匹配 base 的 Studio；每次设置 `RUSTIFY_STUDIO_DIR` 后逐条串行执行 `npx playwright test -c playwright.docs.config.ts --project=official-root`、`--project=official-subpath`、`--project=official-meta`。这同一轮包含 V-14/V-15 和原 official 回归，不再重复运行。真实浏览器核对浅深/中英、窄屏/200% 放大、键盘、无脚本可读/按需 island、源码与示例一致、console/pageerror/失败请求；Studio 的完整已有功能引用未变历史证据，新增公开接线由同包路径实验补证 | M5–M6 全批唯一完整本地收口；失败只重验失败场景与实际波及项，耗时实测记记录 |
| V-18 | 发布风险门 | R-1、C-6：新候选、Docs/Studio 两份构建、最终组合包与线上同 run 一致 | §5.7 的 S→安装 Git CLI→prepare Docs→同 S 构建 Studio→compose→`docs-pages-verify.py --dir`；通过并获新授权后推送/dispatch/watch，再 `python3.12 scripts/docs-pages-verify.py --url https://rockie.github.io/rustify-ui/ --candidate-sha "$S" --run-id "$RUN"`。核对新组件页、Studio HTML/wasm/字体与构建身份全部在整包 manifest 内且线上字节一致，不只比 CLI 版本 | M7；本地完整候选先过门再请求生产授权，线上成功后进入 V-19 |
| V-19 | 真实使用验收（阶段终验） | R-6：只凭线上入口连续完成评估、创作和独立消费；证明充分展示 | Agent 用干净 Chromium context 模拟首次读者和 DOM 开发者，只读官网：①首页找到 Colors/Primitives/新组件，比较 Button 与表单样式状态，键盘执行 Menubar 命令并使用 DataTable/Tree；②进入公开 Studio 选预设、精调、切换模式、检查实际部件和浮层来源，保存并刷新恢复，导出 JSON/CSS/Rust，故意导入不合法文件后可恢复；③在全新仓外 registry `=0.3.0` DOM 项目按页面源码构建一个含菜单/表单与主题边界的真实界面，消费刚导出的主题，浏览器确认主题/状态/菜单键盘结果与官网一致。逐个检查 §6.2 的覆盖记录并亲自看页面与实际导出文件，所有卡点修复或获用户明确延期；不能用截图/计数/退出码代替结果 | M7，V-18 线上身份通过后；完整源码/主题文件/必要截图/卡点记录于 M7；未通过不能记 7/7 |

### 9.2 证据失效与复用

- M1–M4 的发行记录与 U-01/R1/R2 保持原身份和历史结论；M5–M7 新增展示范围尚未验收，不能用 R2 通过覆盖新范围。
- 新改 site 页面、demo、主题/导航，只使对应展示、搜索/SSR/hydrate 与本期场景证据失效，合并在 M5–M6 批末 V-14–V-17 复验，不触发重发包或无关全仓矩阵。
- Studio 公开路径、资源、启动或 CSS 改动须重建匹配 base 的完整产物并重验该应用与整包部署；源码/配置未变的历史编辑功能直接引用原证据。
- 组合器、workflow、identity 改动使 V-16/V-18 的相应产物门失效；线上内容变化需对同候选补 V-19 的受影响任务。部署候选必须在本地门所验代码提交后冻结。
- 任何库/CLI 包字节变化按 runbook 重新确认版本与失效范围；上传后已发布 0.3.0 不可修改。未声明的 SDK 缺陷不靠跳过样例处理。
- 批内最小反馈：围绕当前失败使用 `official.spec.ts --grep` 对应用户任务或复用已有 Studio spec 的具体失败用例；不因一次样式微调跑三个项目或重建未变化的应用。退出门统一执行一次。

### 9.4 浏览器执行纪律

每条 Playwright 命令只带一个项目，串行执行（C-4）；`target/` 可能被 mbx 回收，产物和证据放仓外，必要时用 `serve --dir` 服务复制出的产物。spec 写入 `docs/validation/` 下的历史截图时，收口后用 `git checkout --` 还原（P5 §9.4 的做法），并用 `git diff` 核实。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 官方站导航与库候选 R | 无 | 一批，两个提交：A＝`rustify-docs.toml` 三处、`official.spec.ts` 五处（§1.1）；R＝§1.2 中标为 R 的全部改动，照 `b32844a4` 的改动集做，并覆盖其后新增的 fixture；锁用 `mbx update --offline -p <包>` 重解析。单人负责所有 manifest 与锁 | V-1、V-2；R 已提交，且 `git status` 中没有遗留的库改动；回写「实施进度」 |
| M2 | 上传前发行收口 | M1 的 R；预取完成 | 严格串行：detached R 上的 V-3 库部分 → package-libraries → freeze-cli → 提交 C → C 上的 V-3 CLI 三行 → package-cli → verify（V-4）→ V-5 → V-6 → 授权后 V-7。`DIR` 单人负责；不上传 | V-3–V-7；receipt 冻结且没有必需缺口；回写「实施进度」 |
| M3 | crates.io 发布与公开说明 | M2 的 `DIR` 与 V-7；V-8 的授权与凭证 | 按 §5.3：先上传 docs 组，过公开门，再上传 remaining；提交并推送 D；V-10 通过后推送标签；撤销临时 token。凭证与生产 registry 单人负责 | V-8–V-10；十包与公开说明实际可获取；未完成不算发布成功；回写「实施进度」 |
| M4 | Pages 部署与真实使用验收 | M3 的 D 已在公共 main，V-10 通过；M4 派发授权 | §5.4：差异门 → V-11 → 授权派发 → V-12 → V-13 → 提交并推送 E；清理 `$PREVIEW` 下的 detached worktree（`git worktree remove --force`）以及 `git worktree prune` | V-11–V-13；线上 manifest 的 `cli_version` 为 0.3.0，`build_id` 等于本次 run 的记录；回写「实施进度」 |
| M5 | 官网完整组件、Colors 与 Primitives 展示 | M1–M4 发行成果；现有 R/C 与公开 API | 与 M6 同批；按 §2、§5.5、§6.2 改 `site` 目录/旧 demo/新增分类与 demo、导航、标准主题和公开说明；先贯通 Button 与 Menubar 的可见新能力，再完成全部矩阵。共享 `site/src/lib.rs`/导航/覆盖表单人集成，独立样例可按组分工 | 与 M6 共用 V-14–V-17，全量展示矩阵没有未处置缺口后完成；回写「实施进度」 |
| M6 | 完整 Studio 公开入口与整站组合交付 | 现有 Studio；与 M5 稳定的导航/路由合同；不要求先等 M5 全部完成 | 与 M5 同批；按 §5.6 接线 `examples/theme-studio`、组合 CLI、workflow、verify 和官方测试服务；同候选 Docs+Studio 在真实子路径上运行，保持全部创作能力；脚本/workflow 单人负责，浏览器串行 | 与 M5 共用 V-14–V-17，整包产物、strict CSP 和真实公开入口可用，保留可审产物和覆盖记录；回写「实施进度」 |
| M7 | 新版 Pages 上线与完整成果真实终验 | M5–M6 本地门通过；冻结 S；本次生产推送/派发授权 | 按 §5.7 完整候选预演→授权→部署→线上整包核验→角色任务与独立消费；更新展示覆盖记录、M7 实际证据和当前交付说明，不重发未改 0.3.0 | V-18、V-19 全部通过，官网逐项可发现/操作/带走成果，缺口已修复或用户逐项确认延期；完成最终提交及回填后才报 7/7；回写「实施进度」 |

提交点：M1–M4 原提交与发行记录保留。M5/M6 同批交付，可各自提交源码但批末共用门只运行一次；本地阶段收口通过后必须提交并冻结 S，再做 M7 生产门。M7 全部退出后提交公开交付记录与最终进度，按顶部协议回填 SHA 并提交回填；推送/派发只在本次授权范围执行，不新开分支。完成记录只有实际开始/完成才写对应行，不预建 M5–M7 空记录。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 新候选推送与 Pages 派发授权 | 生产更新 | 实施者先交付 V-14–V-18 本地可审结果与 S/build_id，发布人按具体命令授权；原 M4 授权不默认为新候选授权 | M7 生产门前 | 仅阻塞生产动作，不阻塞 M5/M6 |
| 展示范围再次缩成旧分类/指南 | 原目标落空 | 实施者逐项完成 §6.2 与覆盖记录，验收者从公开官网重走 V-19；不能以数量或旧评审通过关闭 | M5–M7 | 缺项直接阻塞本期完成 |
| 公开 Studio 的子路径/CSP/资源接线 | 真实可用性 | 实施者沿既有 build-web 与 §5.6 组合合同修接线，在 meta-only 和同包官网入口验证；不能换成根路径独立 demo 自证 | M5–M6 批末 | 未通过则不进 M7 |
| 用户主题与演示主题/本地库混淆 | 保存、导出结果 | 演示 island 不写持久化，Studio 沿独立 key 与既有 schema，不增新存储模型；真实任务检查刷新及导出消费 | V-15、V-19 | 有真实错误则阻塞相应任务 |
| 真正 SDK 缺陷或未发行接口 | 公开消费契约 | 先排除示例组装错误；记录具体能力/影响，按 §8 修订必要版本范围，不隐藏或把它写成演示限制 | 首次具体失败后，M7 前 | 若影响必做项则阻塞终验 |
| impeccable 技能缺失 | 实施设计工具约定 | 当前可见技能中未发现；实施前明确提示安装，依据 PRODUCT/DESIGN 完成设计并如实记录所用工具，不声称已调用缺失技能 | 页面设计开始前 | 不阻塞计划修订；是否影响实际执行按仓库规则说明 |

- 最终状态：**Ready**。
- 定级理由：用户已明确完整展示范围；采用现有 Docs + 完整 Studio，路由、组合边界、版本、可操作矩阵和验收已给定。本地实现没有未决产品/接口选择；生产权限在具体候选门办理。Ready 是可继续实施，当前仅完成 4/7，不能解释为整期交付通过。

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
