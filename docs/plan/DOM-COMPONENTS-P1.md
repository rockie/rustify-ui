# DOM-COMPONENTS-P1 · Rustify UI · Colors 与统一主题基础（DOM · 单一作者文档）

```text
::OBJECTIVE{implementation}
  target: 直接依据本计划、仓库规则和当前代码实施或续做
  ACCEPT: 全部里程碑退出检查通过，按「实施进度」完成最终提交、SHA 回填与自查，并提示用户发起代码评审
  实施须知: 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
```

> **计划状态：Ready**
>
> 调查基线：2026-10-26 · `11b6912919a2a34c372aac4d6b7d51052820600f` · main / dirty。事实取自当前工作树；输入 PRD v0.2、其评审处置及相关 SDLC 文件尚未提交，不得用 HEAD 代替这些输入。
>
> 需求来源：[DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) · **P1**，以 §7.2 的 AC 分配为边界；[PRD 评审处置](../PRD-DOM-COMPONENTS.review.md)中的故障分层约束同时生效。
>
> 本期交付：**可独立静态消费的 Colors、统一主题 v2、基础 Themes 操作/布局/排版、现有 Theme Studio 的配置—精调—保存—导出闭环。**
> **开发范围：本仓 Colors 数据包、DOM 主题内核、基础 DOM 组件、Studio、受影响的 Docs/独立消费者和验证入口。**
> 本期独特职责：在已有完整双模式语义 token 编辑上增加可追溯配置与显式覆盖，完成旧主题/本地库迁移，而非新建另一套主题站。
> **顶层排除：P2–P5 的组件补齐与通用 Primitives 引擎、完整新组件 Inspector 目录、GPU 对等组件/P3 渲染、重写大数据组件、云端服务、公开发布及门户接入。** 现有 GPU 仅复用共享主题投影并做回归。

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
  [DO] 回写后按 @project_index 同步已启用的台账
  [VERIFY] 每次回写后直接执行 @progress_consistency

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
  [IF] 最后一个里程碑完成 => [DO] 按 @commit_protocol 完成最终提交并回写 SHA，确认每个已完成行的「提交」都有 SHA => [DO] 恢复快照的「下一步」写代码评审
  报告: 随后向用户报告完成，并提示下一步用 review-code 以本计划路径为参数评审本计划；它从完成记录找到要审的提交，并纳入未提交改动。
  发起: 评审由用户发起，实施中不自行启动评审或修复评审意见。

::CHECK{progress_consistency}
  规则: 回写后自查
  OBSERVE: 每次回写后核对恢复快照、完成记录、记录链接及本计划文件的 git diff；非 Git 仓库对照回写前副本。
  PASS: 当前进度 n/N 的 N 等于里程碑总数，n 只计状态为「已完成」的行；最近完成对应最后一条已完成记录，不按编号推算；链接指向已落盘文件；已完成行的「提交」是短 SHA 或「待提交」，全部完成时每行都有 SHA 且没有「待提交」；本节以外只有有意修订的正文，定义行没有被进度文字改写。
  边界: 上述结构、计数与链接检查不证明验收真实通过。
```

### 恢复快照

- 最近更新：2026-10-26T17:44:26+08:00
- 当前进度：4/7 个里程碑完成
- 当前状态：进行中，M1–M4 已完成，继续 M5
- 最近完成：M4 · 基础 DOM 组件与响应式外部样式
- 下一步：M5 · Studio配置/历史、导入review、本地库升级与同页/跨页提交安全；退出条件见 §10
- 当前阻塞：无；新主题/组件/存储与整期验收尚未完成
- 代码基线：219b8481（M3–M4；工作树仍含原有资料）

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-26T14:44:13+08:00 | Colors 数据/资产、来源和字段矩阵冻结，静态浏览器与真实 GPU 改前基线通过。 | [M1 记录](DOM-COMPONENTS-P1.records/M1.md) | d60a8e11 |
| M2 | 已完成 | 2026-10-26T15:56:53+08:00 | v2/严格v1升级、配置责任与双模式来源、单次尺度解析通过；受影响调用点已迁移。 | [M2 记录](DOM-COMPONENTS-P1.records/M2.md) | d60a8e11 |
| M3 | 已完成 | 2026-10-26T16:25:04+08:00 | 根预览/双模式边界、CSS模式与清理、局部浮层及既有GPU回归通过。 | [M3 记录](DOM-COMPONENTS-P1.records/M3.md) | 219b8481 |
| M4 | 已完成 | 2026-10-26T17:44:26+08:00 | 基础操作/布局/排版、ThemePanel、响应式与外部SSR样式及真实浏览器通过，旧调用已迁移。 | [M4 记录](DOM-COMPONENTS-P1.records/M4.md) | 219b8481 |


## 0. 需求、范围与决策

### 0.1 需求与约束账本

上游 ID 均定位到 PRD §3、§7.2；本表只登记 P1 的 AC，不将跨期需求整体记为完成。UAT 编号引用 PRD §6.3；其 P1 子路径由 §9 检查承接。

| ID | 本期能力 / 边界 | 设计落点 | 验证落点 |
| --- | --- | --- | --- |
| WORK-F1/AC-1 | Colors 不需要 wasm、Makepad 或 React | §1.2、§5.1 | V1、V8；UAT-1 |
| WORK-F2/AC-1—AC-2 | 全部色阶及显式 sRGB/P3 按需消费 | §5.1 | V1；UAT-2 |
| WORK-F3/AC-1—AC-3 | 六类配置、嵌套外观、形状与单次缩放 | §3.1、§5.2–5.4 | V2、V3、V4；UAT-2 |
| WORK-F4/AC-1—AC-3 | 覆盖保留/解除、隔离、浮层继承与度量语义 | §3.2、§5.2–5.4 | V2、V3、V6；UAT-3、6 |
| WORK-F15/AC-1 | Button/IconButton/Link 的公开视觉行为 | §5.5 | V4；UAT-2、9 |
| WORK-F16/AC-1—AC-2 | 布局/排版语义、响应式、局部 Reset | §5.6 | V4；UAT-2、9 |
| WORK-F18/AC-1—AC-2 | 44 预设、统一编辑与既有双模式规则 | §5.7、§6 | V5；UAT-2、3 |
| WORK-F20/AC-1—AC-4 | v1 升级、新格式、候选导入、独立消费者及有损说明 | §3、§5.8 | V2、V6、V8；UAT-3、11 |
| WORK-F21/AC-1—AC-3 | 历史、本地库迁移、写失败与跨标签显式决策 | §3.3、§5.7 | V5、V6；UAT-3、10 |
| WORK-F23/AC-1 | 能力目录区分可用/部分/未交付及呈现路径 | §5.9 | V7；UAT-1、13、14 |
| WORK-N2/AC-1 | 一帧一次发布、最终值可达、无重建和空闲更新 | §5.4、§7 | V9；UAT-10、12 |
| WORK-N3/AC-1—AC-2 | CSP、样式隔离、导入只作数据与失败不应用 | §5.1、§5.4、§5.8 | V3、V6、V8；UAT-6、10、14 |
| WORK-N5/AC-1 | 目录/文档/示例对实际交付状态一致 | §5.9 | V7 |
| WORK-N6/AC-1 | 失败保留有效主题与可取回原始数据 | §3.3、§8 | V5、V6 |
| WORK-N7/AC-1 | 中性编辑壳；配置、精调、来源与保存状态清楚 | §6 | V5、V10；UAT-2、9、14 |
| WORK-N8/AC-1—AC-2 | 来源许可、指纹与脱离 ref 可重现构建 | §5.1、§4 | V1、V7 |
| C-1 | [AGENTS.md](../../AGENTS.md)：按职责选 DOM；不新增 GPU 对等路径 | §1.2、§5.4 | V7、V8 |
| C-2 | AGENTS.md、[工具链](../../mise.toml)：现有 mbx/Tailwind/Leptos；不升级依赖 Major、不另装服务 | §4、§9.2 | V7、V8 |
| C-3 | AGENTS.md、[设计规范](../../DESIGN.md)：设计与真实浏览器验收；旧 UI/数据保留 | §6、§9.3 | V5、V10 |
| C-4 | AGENTS.md：热路径改动给同环境前后数值，且避免重复解析/序列化 | §7、§9 | V9；这是 P1 工程门槛，不提前结算 WORK-N2/AC-2 |
| C-5 | PRD §2.7：正常清理、GPU 局部失败、实例 fatal 不混同 | §8 | V3、V8；只回归已影响路径，不提前结算 F19/N3/N6 的后期 AC |

### 0.2 决策表

| # | 决策 | 含义与依据 |
| --- | --- | --- |
| D1 | 新增无 UI 依赖的 `rustify-colors`；Themes 继续在 `rustify-components`，配置归 `rustify-dom` | Colors 独立分发；不因分层先拆全套组件 crate。F1/F2 与既有依赖方向均可满足。 |
| D2 | JSON 主格式升为 v2；保留完整双模式 base、稀疏 overrides 和 config | 不把元数据塞进 v1 token，不从最终颜色反推色族；旧 accent 保持原义。 |
| D3 | 主题上下文保留双模式作者来源；快照只是派生物 | 当前 `ResolvedTheme::patched` 复制当前模式为两模式，不能用于新嵌套 appearance。 |
| D4 | Colors 原始字符串/色域数据精确保留；统一主题仍解析 sRGB | P3 是静态 Colors 可选资产，不扩大 `parse_color` 或 GPU 色域承诺。 |
| D5 | 属性使用各组件自己的枚举；响应式采用参考断点与受控 CSSOM | 不以一个全局 variant 枚举覆盖所有组件，也不借 Tailwind 同名断点偷换语义。 |
| D6 | Studio 仍只有一份作者文档和一条历史/发布链 | ThemePanel 发请求、不另存主题；所有配置、覆盖、重置和预设操作复用 EditorState。 |
| D7 | 新持久化键 + v1 只读保留；本地库升级须显式确认；同页单写者、跨页 Web Locks 与锁内比较 | 待锁不冻结编辑；旧请求只确认所存快照，取消升级持续禁止自动写入；无锁能力时保留编辑/导出但禁持久化写。 |
| D8 | P1 按源码配套消费交付，不变更已发布版本记录 | 完整 v2 需本期收口 SHA 的 SDK/源码 CLI；已发布 0.2.1 不支持。公开发布另行授权，不把未发布版本写进 registry 配对记录。 |

### 0.3 ADR-lite

**ADR-1 · 作者数据模型（Proposed）**

- 共同约束：旧主题值无损、配置不清覆盖、JSON 保留意图、DOM/GPU/导出同源。
- 候选 A：单份最终 token 表加“被改过”标志；结构小，但切色族/解除覆盖须重建被覆盖前的值，无法稳定区分旧作者值和生成值。
- 候选 B：base + config + overrides；多一份稀疏数据，但生成可重算，来源和 reset 均局部、确定。
- 选择 B。旧数据迁移时仅复制配置可能重写的已知键到 overrides（完整名单见 §5.2），其余仍在 base；未知键保留但不投影。代价是有限重复，不再额外持久化 resolved 结果。
- 重审触发：将来增加动态引用 token 或跨文档继承；本期不预建引用图。

**ADR-2 · Colors 分发（Proposed）**

- 候选 A：随组件 CSS 分发色板；改动小，但仅 Colors 的消费者被绑定完整皮肤入口，且不利于 sRGB/P3 按需选择。
- 候选 B：独立纯数据/资产 crate，DOM 仅依赖 sRGB 数据；静态页只取对应 CSS。
- 选择 B；不引入 JS 运行时或网络数据服务。新增包许可、Cargo package 内容检查和来源生成检查属于本期；公开 registry 发布不属于本期。

**ADR-3 · 本地数据升级与并发（Proposed）**

- 原键原地升级成本较低，但旧标签会把 v2 视为坏缓存并可能改写同一键；新键保留原文能隔离格式与回滚。
- 选择独立 v2 键。完整转换后先等待显式升级确认，再把迁移原文保存到内容寻址的独立备份键并回读确认，最后写v2；单次v2 `setItem` 是迁移提交点，不把多键操作描述成事务。取消不改变有效主题/历史，也不授权后续autosave。v2写锁+锁内重读解决新标签之间竞争；旧标签不遵守新锁，所以v1永不由新站覆盖，独立备份也不随旧标签变化。
- 同页采用单写者队列，连续未发出的autosave可合并，命名库动作保序。相比等待锁时冻结整个编辑器，代价是区分快照确认与当前草稿；§3.3明确成功确认、续写及失败暂停规则，不直接复用同步路径的整份EditorState回写。
- 代价：保留两份数据、迁移写可能触发 quota；不自动删旧数据以腾空间。无锁/写权限时降为内存编辑和下载。

## 1. 现状与架构

### 1.1 已核实事实与缺口

| 事实源 | 当前行为 | 本期增量 |
| --- | --- | --- |
| `crates/rustify-dom/src/theme/document.rs`：`ThemeDocument`、`COMMON_TOKENS` | schema v1，`styles.light/dark`，35 色值 + 14 公共值；无 config/overrides | v2、显式来源与升级器；保留公共值双模式编辑规则 |
| `theme/codec.rs`：`import_json/import_css/export_css` | 256 KiB 输入上限、重复 token 拒绝、CSS 候选解析、未知字符串 token 不投影 | 版本分派、配置/覆盖 diff、有损兼容清单 |
| `theme/resolve.rs`：`ResolvedTheme::patched`、`properties` | 当前模式 patch；`--spacing` 是 layout-gap、`--rustify-unit` 是单位；有 shadow 派生 | 双模式来源保留、配置生成、来源结果和单次 scaling |
| `theme/provider.rs`；`crates/rustify-dom/src/overlay.rs` | scope root 所有权、boundary 清理、浮层订阅有效快照 | 配置/appearance 边界与完整属性集替换，继续复用浮层订阅 |
| `examples/theme-studio/src/state.rs`、`controller.rs` | 文档整体历史、候选两模式校验、帧合并、手势取消、公共 token 双模式更新 | 配置和覆盖进入同一命令/候选/历史；更新缓存相等判断 |
| `examples/theme-studio/src/persistence.rs` | v1 envelope；100 个命名主题上限；坏缓存保留 raw；无版本迁移/写锁 | v2 envelope、原始数据取回、并发写协议 |
| `button.rs`、`link.rs`、`geometry.rs` | 旧视觉枚举；原生 disabled/href/aria-current；外部几何样式收集与 CSSOM 生命周期 | 新视觉参数、IconButton、布局排版及响应式规则收集 |
| `catalog.rs`、`xtask/src/catalog.rs` | 组件类别与支持表生成文档；类别不等于完整公共导出清单 | 基础能力/参考差异与未交付项的统一事实源 |
| `tests/theme-export/README.md` | 独立 workspace 编译真实 Studio Rust snippet、两 CSS profile，根/子路径严格 CSP | 新格式、迁移、源码版本配对与纯 DOM 消费者 |
| `crates/rustify-docs-cli/src/theme.rs`、`assets/framework-release.json` | CLI 通过共享 import/resolve；发布配对记录为 0.2.1 | 源码 CLI/SSR 适配 v2；保留已发布记录，不声称旧二进制能读 v2 |

### 1.2 目标依赖与职责

```text
rustify-colors（纯 Rust 数据 + 外部按需 CSS；无 Leptos/浏览器依赖）
  ├─ 静态 Colors 页面
  └─ rustify-dom::theme（作者文档 → 配置/覆盖解析 → ResolvedTheme）
       ├─ DOM ThemeScope/ThemeBoundary → rustify-components 基础 Themes
       ├─ 现有 GPU 投影（仅既有共享语义、sRGB）
       ├─ Studio EditorState / ThemePanel / 导入导出
       └─ Docs 源码 CLI / SSR 与独立消费 fixture
```

- 旧 `Theme`/`ThemePatch` 接入保留为既有输入适配，不维持第二份新组件实现；`ThemeDocument::from_legacy` 输出规范 v2。
- 新组件全部 DOM。原有 GPU 控件不新增品牌色族 prop、布局/排版 API 或专属组件；共享 geometry/token 的变化通过既有投影进入。
- 原有大数据、表单、浮层等功能仅做被公共接口变化强制要求的编译/主题适配；各自功能补齐留后续期次。

## 2. 改动面

下列“新增”为计划路径，其余为已存在入口；不提前创建空目录或占位实现。

| 路径 | 变更职责 |
| --- | --- |
| `crates/rustify-colors/`（新增） | 无默认 UI 依赖；vendor 四份 TS 数据、MIT 许可、生成的 Rust 表和按需 CSS；提供包内数据/资产 API |
| `xtask/src/colors.rs`（新增）、`main.rs`、`sources.rs`、`sources.lock.json` | 确定性 colors 生成/检查命令；双指纹算法分开记录；来源与许可校验 |
| `Cargo.toml`、相关 crate manifests/locks | Colors workspace 及依赖、Web Locks 所需 web-sys feature；不升级工具链/依赖 Major |
| `crates/rustify-dom/src/theme/{document,codec,resolve,provider}.rs`、`theme.rs`、`lib.rs` | v2、配置/覆盖、上下文、导出入口；必要时新增 `theme/config.rs`，不复制解析器 |
| `crates/rustify-components/src/{button,link,lib,catalog,geometry,styles}.rs` | 修改原实现/公共表；扩展样式收集以支持媒体规则 |
| `crates/rustify-components/src/{icon_button,layout,typography,theme_panel,responsive}.rs`（新增） | 基础组件与类型；只提取真实共享的尺寸/排版/响应式策略 |
| `crates/rustify-components/css/` | scoped 基础 Themes 样式、可选资源入口；`sdk.css`/`theme-v4.css` 与提交产物保持同一构建 |
| `examples/theme-studio/src/{state,controller,persistence,data_tools,app}.rs` 及该例 CSS/文案/预设适配 | 同一编辑闭环、来源 UI、迁移/冲突/失败提示；保留原预设数据 |
| `examples/component-catalog`、必要的仓内 Button/Link 调用点、Docs 示例 | 新契约样本与旧入口迁移；不扩展示例业务 |
| `tests/browser/`、`tests/theme-export/`、`playwright.config.ts` | 新主题/组件/Colors 静态与 DOM-only 消费测试；保留旧断言的真实目的 |
| `crates/rustify-docs*/` 的实际受影响主题/样式消费点 | 源码主题、外部 CSS 收集与 SSR 适配；不重写 docs 渲染系统或发布服务 |
| `docs/themes.md`、`docs/components.md`、`README.md`、`DESIGN.md` | 更新真实交付；新增 `docs/dom-components-migration.md`，按变更更新而非重写无关内容 |

## 3. 数据、接口与兼容

### 3.1 v2 作者文档

以下为待实现的规范内存结构与一般JSON形式；serde JSON字段使用snake_case，所有对象拒绝未知结构字段，token map允许未知**字符串**键且禁止重复键。另有下述等价Custom紧凑形式，防止v1升级的元数据增量突破输入上限。示意结构不是可直接导入的完整主题：

```text
ThemeDocument {
  schema_version: 2,
  id, name,
  base: { light: ThemeValues, dark: ThemeValues },
  config: ThemeConfig {
    appearance: inherit | light | dark,
    accent_color: null | AccentColor,
    gray_color: null | auto | GrayColor,
    panel_background: null | solid | translucent,
    radius: null | none | small | medium | large | full,
    scaling: 90 | 95 | 100 | 105 | 110
  },
  overrides: { light: ThemeValuePatch, dark: ThemeValuePatch }
}
```

- `base` 为完整、未缩放的两模式作者表；已知键沿用旧49 token名单和校验器。未知字符串保留用于JSON往返，但不变成CSS属性或GPU props。
- v2允许一个**等价Custom紧凑wire形式**：`{schema_version:2,id,name,styles:{light,dark}}`，不能与base/config/overrides混用。它明确表示Custom、inherit、100%和§5.2规定的迁移覆盖；解码后展开到统一内存结构，不保留第二套运行模型。仅当完整作者意图与该形式等价时序列化为它；任何非默认配置/解除覆盖都使用一般形式。这样合法v1可只换版本号成为不增大的紧凑v2，原未知字符串全部保留；不在v1中隐藏元数据。
- 规范JSON使用紧凑UTF-8序列化；所有候选提交前检查规范完整v2不超过256 KiB，导出与下次导入使用同一判据。临界文档后续配置编辑导致一般形式超限时拒绝该次候选、保留原文档/历史并解释体积原因，不静默删未知键或回退有损格式。V2覆盖恰限、超限、Custom升级不增大、配置增加与解除覆盖的临界用例。
- config 中 `null` 表示 **Custom / 不由该快捷项生成**，不是嵌套继承；迁移 v1 时四项均 null、appearance=inherit、scaling=100。新建标准主题默认 indigo/auto/translucent/medium/100、appearance=inherit。两种入口明确命名，不将旧 `from_legacy` 改成色族主题。
- `AccentColor` 的26项、`GrayColor` 的六项加auto严格采用PRD §2.4名单；Colors完整31色族与可选品牌色26项不是同一枚举。`accent_color` 是品牌色族，不是 `base.*.accent`。accent 的旧语义及 44 预设原始值不变。
- overrides 仅接收已知可编辑 token；移除键才是解除覆盖。显示来源属于派生数据，不写进 JSON。
- appearance=inherit：根取应用明确传入的 mode（Studio 预览偏好、SSR/导出所选模式），无调用方时默认 Light；嵌套取最近父有效模式。不订阅全局 root/body、不暗中依据宿主 `.dark`。
- 内核保留显式按模式解析的 `resolve(document, mode, context)`；它用于双模式校验和导出，不受文档 appearance 强制覆盖。普通 provider 先按 appearance 求有效 mode 再调用它。Studio 模式选择是显式 preview override，优先于根文档 appearance，但不覆盖嵌套边界显式模式；只改预览输入，两个模式的作者值/config均不写回，显示“预览模式”避免混淆。
- `ThemeConfigPatch`（新增）每项区分 Keep / Set；可配置项 Set(null) 表示 Custom，撤销局部选择用 Keep。`ThemeBoundaryPatch` 含 config patch 与两模式稀疏 token patch。该结构是应用运行态输入，不加入根文档持久化。

### 3.2 解析结果与接缝

| 接口 / 调用者 | 契约与隐藏的复杂度 | 失败和测试面 |
| --- | --- | --- |
| `import_json` / `import_css` → Studio、CLI | 输入大小、重复键、版本分派、v1 升级、两模式校验统一；返回候选与诊断，不应用 | `ThemeError { field, message }` 定位 config/base/overrides；V2/V6 |
| `resolve` → provider、exporter、GPU bridge | base → 根/边界配置合并并生成 → 文档 overrides → 由外向内边界 overrides → 未缩放度量派生 → 单次 scaling → immutable snapshot；返回有效配置、颜色阶/角色、来源 | 非法候选不替换上一有效文档；V2/V3 |
| `ThemeScope` / `ThemeBoundary` → 应用 | 保留双模式 `ThemeSource` 与 resolved；边界读取父 source，不在已缩放快照上再次 patch | 空 patch 直接复用父结果；清理恢复继承；非法局部 patch 回退外层并诊断；V3 |
| `ThemePanel` → 应用、Studio | controlled：有效配置 + 覆盖统计 + `on_change(ThemeConfigPatch)`/`on_reset(group)`；不拥有第二份主题/历史/存储 | 应用拒绝后仍显示实际受控值；V4/V5 |
| 编辑器命令 → `EditorState` | 候选整个文档验证、no-op 相等判断、手势合并、历史回滚、版本变化全在同一入口 | 不以逐 token 循环制造多个提交；V5 |
| 持久化适配 → controller | 纯候选转换和序列化；升级许可门、同页单写者及锁内比较/写入见§3.3；异步结果携带操作身份和实际提交快照 | 成功只确认对应快照，不替换更新后的editor/history；Invalid保留raw，写失败保留内存；V6 |

`ThemeSource` 是不可变双模式文档 + 已应用边界配置/覆盖信息；只服务 mode 切换、来源说明和重新解析，不建设通用主题树/远程 port。公开 `use_resolved_theme` 继续供现有浮层/消费者订阅；内部 source 与快照须同 revision 发布。每个 snapshot 必须能追溯完整作者来源，SSR 的 `provide_static_theme` 不能只留下当前模式而使嵌套深色失真。

### 3.3 本地 envelope v2 与迁移提交点

- 新键：`rustify-ui.theme-studio.v2`；旧键保持 `rustify-ui.theme-studio.v1`。v2 envelope含 `envelope_version:2`、draft、saved_themes、preferences、单调revision，以及可空 `migrated_from_v1`（旧原文摘要和备份键）。主题数量、身份校验沿用既有上限；256 KiB用于每份主题交换JSON，不误套为整个本地库的上限。

**读取、升级确认与写许可**

- 读取顺序：存在v2就完整验证；有效时允许后续持久化，坏v2进入坏缓存保护，不静默回退/覆盖。仅v2缺失时读取v1；完整转换草稿、每个saved theme及偏好后才能进入待确认，一项失败即不写新键，提供原始数据下载。两键都缺失的新库正常创建不需要旧库升级确认；JSON解码器仍只转换数据，不承担弹窗或写许可。
- 有效v1沿用现有启动恢复：以转换后的旧草稿/偏好初始化**内存**编辑器，以对应保存主题/预设或草稿建立checkpoint，启动历史为空；不改作者值，不表示升级已提交。完整库候选与原文另存待确认区，后续编辑进入同一EditorState。确认/取消本身不重置当前主题、undo/redo、checkpoint或预览模式。

| 升级状态 | 允许动作与转移 | 持久化许可 / 反馈 |
| --- | --- | --- |
| 待确认 | 编辑、撤销/重做、切模式/偏好、导出、下载原文；可“升级并保存当前草稿”或取消 | 不写v2/备份，不积压待放行的autosave；命名保存/覆盖/删除等写操作提示先确认升级 |
| 已取消 | 保留取消瞬间的有效主题、历史和偏好；上述内存操作及“重新检查升级”继续可达 | 持续禁止自动/普通保存写入，明确改动仅在内存、重开不能恢复未保存编辑；取消不写任何标记，重开仍从原v1恢复并待确认 |
| 已确认待提交 | Review展示原v1库的完整迁移结果及将保存的当前草稿/偏好，绑定原文摘要和当前编辑代次；确认时已变化则重新Review，否则进入下述单写者 | 确认门在备份和v2写入前；待锁可继续编辑，后续变更按快照确认规则处理，尚不显示升级/保存成功 |
| 提交失败 / 原文变化 | 原文、当前内存和已写备份保留；错误可修复后重新Review、显式重试，或取消 | 写保护不解除；不自动重试，不把后续编辑/切模式/偏好变化当成批准 |
| 提交成功 | 更新已确认库和存储基线，打开正常写入口；若当前草稿已变化则续写最新已提交草稿 | 只确认实际写入快照；不恢复旧editor副本。原文取回入口持续保留 |

- 升级请求锁内重读v2，若另页已创建则转冲突；核对v1原文仍与本次Review相同，否则先重新Review。把**实际参与转换的完整v1原文**写入 `rustify-ui.theme-studio.v1.backup.<sha256>` 并回读逐字节确认；同名已存在时必须内容相等，不覆盖。只有备份可取回才写v2并标成功。备份失败不写v2；v2写失败保留已写备份且报告未迁移成功。可变v1键不是备份，不能用摘要代替原文。取消后再次升级的候选包含重新Review时的当前草稿/偏好与完整旧库，不回放取消期间的旧autosave。

**同页顺序、锁内比较与成功确认**

- controller持有一个写者，最多一个已请求/持有Web Lock的操作。请求使用单调`op_id`，携带生命周期代次、`editor_epoch`、文档revision、当前draft/偏好快照和库操作意图（目标ID、名称及保存内容）；`editor_epoch`在文档、模式/偏好、历史、checkpoint、手势开始/结束或编辑对象变化时递增，不以文档revision代替。出队时绑定已确认存储的原始文本/版本，候选在最近**本页已确认**库上应用该意图，锁外完成验证/序列化；autosave复用已确认库及编辑器的校验缓存，不每次重验未变命名库，也不保留排队时的整库副本。
- 所有autosave、命名保存/复制/改名/覆盖/删除、Keep/Save copy及升级提交走该入口，按请求顺序执行；仅相邻且尚未请求锁的autosave可替换为最新快照，不跨命名库动作合并。请求锁后快照不变；等待期间的编辑、撤销/重做、模式及偏好变化留在内存，产生后续autosave，手势过程仍不存储中间值。命名库动作不可被后续autosave覆盖或吞掉，每个操作有独立待处理/成功/失败结果。
- 锁内重读原始文本/版本并比较出队时绑定的基线；不相等则暂停队列，保留所有pending意图和当前草稿、显示冲突，禁止自动重试/跳过。相等则写已验证的完整候选，存储revision+1；异常不更新已保存事实。前次**本页**成功后，先用实际写入原文/版本推进基线，再出队下一项；只允许承接该成功更新，不把锁内发现的外部变更自动吸收为已批准基线。
- 成功回调只推进所对应`op_id`的已确认库、基线及快照保存记录，不能用保存前克隆的EditorState替换当前editor、undo/redo、gesture或缓存，也不能覆盖当前偏好。命名保存的身份/checkpoint更新仅在`editor_epoch`仍匹配且没有更新的命名保存请求时，经当前编辑器的既有命令完成；否则只确认库中已存下点击时的命名快照，当前草稿身份/checkpoint不变。autosave不建立checkpoint。
- “当前草稿已保存”须对应当前编辑代次及完整draft/偏好快照，且没有写保护、外部冲突或更晚待处理写操作；旧请求成功但当前已变更时显示“先前版本已保存，当前改动待保存”。最新已提交草稿若未由后续请求覆盖，则在队尾补autosave；活动手势在提交/取消后补。正常完成后最终草稿可重开恢复；存储失败/冲突则暂停队列、保留所有意图并给显式重试/取消入口，不声称已保存。冲突Load放弃待处理动作须明确其范围，Keep/Save copy只授权所展示的候选，未处理的命名动作不得暗中丢弃或重放。
- Load 读取最新库并候选校验；Keep/Save copy 是对用户看见的冲突版本作决策，锁内又变化就再次询问，不沿用过期批准。Save copy 在最新库上校验名称与 ID；写成功再更新 UI。
- 显式坏库重置、清除缓存及备份清除也经同一写者/同名锁：锁内比较用户确认清除的原文/键集合，变化则重新确认，成功后才显示清除结果。保留当前内存草稿；清除动作前的pending请求不得在清除后自动重建库，须在明确展示后取消或由用户重新保存。普通清除不碰v1/备份；备份清除须单独确认。
- Owner清理或实例fatal使该生命周期代次失效，清空未发出请求并取消待授锁请求；用`release_on_abort`登记原生AbortController清理。授锁回调及Promise续体在读取signals/写库前核对存活代次与`instance_failed()`；锁内不等待用户或其它异步操作，写前再核对，过期请求不写入、不回写UI。AbortSignal只取消待授锁请求，不能代替已授锁回调guard；用现有生命周期能力，不新增运行时隔离模型。
- storage 事件同时监听两键。升级后的旧键变化显示“旧版窗口修改了旧库”，保留原文供显式导入；不把旧库自动合并或盖过新库。关闭/重开时再次比对 migrated_from_v1；旧写不遵守新锁的限制需写进迁移文档。
- 无 Web Locks、非安全上下文、storage 不可用或 quota 时明确“仅内存编辑/可下载”，本地写操作禁用或报真实失败；不得降为无锁写。锁异常路径释放由浏览器 promise 生命周期保证，不持锁等待用户交互。
- 重复启动/迁移幂等；备份按原文摘要去重，失败不删任何源/备份键。Studio提供“下载升级前原始数据”，从备份键读取并核对摘要；即使旧窗口改过v1、所有窗口都已关闭后仍可取回。备份只允许用户明确清除，普通保存/删除主题/重置草稿不动备份。回滚旧站只读旧键；v2后续编辑不能自动回灌，须显式有损导出或保留v2文件待重新升级。

## 4. 依赖与外部契约

| 依赖 / 提供方 | 分类与核实结论 | 本期处理与解除条件 |
| --- | --- | --- |
| `ref/colors-main`；本地参考快照 | 已核实足够：四份 TS 数据、生成器、许可证可读 | 采纳输入与 PRD §1.5 对齐，vendor 后构建不读取 ref；不需要上游服务 |
| `ref/themes-main`；本地参考快照 | 已核实足够：组件 prop、theme 运行规则、样式 tokens、MIT | 仅采纳 P1 相关稳定定义，逐字段来源见 §5；不运行 React |
| 既有主题/Studio/浮层 | 已核实缺口：v2、双模式边界、迁移和写锁不存在 | 本方 M2–M5 实现，不标为外部阻塞 |
| 浏览器 Web Locks / CSSOM / GPU | 现有浏览器集成；新锁语义尚未运行验证 | M5 真双页验证；不支持写锁降为只读持久化；GPU 只跑现有恢复链路 |
| 本地 Rust 1.99.0、Leptos =0.8.22、Tailwind 4.1.13、Playwright 1.63.0 | 配置已核实；实际环境可用性在 M1 核验 | 缺工具/浏览器/依赖先报告，不擅自安装或升级；无需新服务/容器 |
| 已发布 SDK/CLI 0.2.1 | 已核实不满足 v2：schema 校验只接受 1，发布配对记录亦指向旧包 | 本期源码开发模式配套验证；未来公开版本由发布流程决定，不能修改发布记录冒充已发布 |

M1 将逐字段能力清单与来源检查转成可重复测试，是已定范围的盘点，不是重新决定交付范围。若实际输入偏离 PRD 指纹或某项能力无法按本方案实现，记录具体差异并停止受影响工作，不静默替换基线。

## 5. 实现设计

### 5.1 Colors 数据与静态分发

1. `rustify-colors` vendor `src/light.ts`、`dark.ts`、`blackA.ts`、`whiteA.ts` 及完整 MIT notice。生成器仅接受这些常量对象的有限语法，遇到无法识别内容失败，不 eval TS/JS。
2. 保留原始字符串，不先浮点归一化再输出。生成纯 Rust typed `ColorFamily`/`ColorMode`/`ColorGamut`/`ColorScale` 查询，步骤限定 1–12，完整覆盖 31 色族×浅深×实色/alpha 及 blackA/whiteA。sRGB 数据默认编译；P3 Rust 表按非默认 feature 启用，P3 CSS 不被默认 SDK/Studio 导入。
3. CSS 按 family/mode/alpha 拆分；普通入口只有 sRGB，`*-p3.css` 显式导入对应 sRGB 回退，再在 `@supports(color:color(display-p3 1 1 1))` 与 `@media(color-gamut:p3)` 下覆盖。以显式 `[data-rustify-colors]` + mode 标记限定静态选择器；不同 mode 根可同页并存，无裸 `:root/.dark/body` 污染。允许无 Rust 应用直接复制/链接资产，不需要 Node/wasm。
4. Themes 不导入静态 Colors 页的根选择器；resolver 从数据生成有效 scope 属性。品牌阶使用 `--rustify-brand-1..12`/`-a1..a12`，中性阶用独立 `--rustify-gray-*`；绝不覆写旧语义 `--accent` 为色族名。
5. 新增命令 `mbx xtask colors [--check]`，从 vendor 重建，检查每组成员、阶序、值和生成漂移；`sources verify` 校验 vendor 与许可，正常 check 不依赖 ref。开发时再对 ref 输入逐项校准。
6. `sources.lock.json` 分别保存原路径/包版本/逐文件 SHA-256/修改说明/生成器输入输出；PRD 目录指纹使用其“路径 + NUL + 二进制摘要”算法，现有 xtask 的“路径 + 空格 + hex”指纹另命名，不能混记成同一摘要。上游 commit 未核实就不填写。

### 5.2 配置生成与覆盖优先级

**生成责任表（穷尽会被快捷配置重写的旧语义键）**：

| 配置 | 生成键 / 规则 | 不触及 |
| --- | --- | --- |
| accent_color | primary=品牌9；primary-foreground=参考 contrast；ring=品牌8；sidebar-primary 同 primary，sidebar-primary-foreground 同 contrast，sidebar-ring 同 ring | 旧 accent/accent-foreground、destructive、success、warning、chart-* |
| gray_color | foreground/card-foreground/popover-foreground/sidebar-foreground=灰12；secondary/secondary-foreground=灰3/灰12；muted/muted-foreground=灰3/灰11；border/sidebar-border=灰7；input=灰2；sidebar-accent/sidebar-accent-foreground=灰3/灰12；background/sidebar：浅白、深灰1 | 字体、阴影及独立状态色 |
| panel_background | card/popover：浅 solid 白、translucent rgba(255,255,255,.7)；深 solid 灰2、translucent 灰a2；生成独立 panel/surface 角色 | 编辑壳背景，不改宿主 |
| radius | 语义 radius 对应参考 radius-3；组件的 1–6、full/thumb 分别求值 | Checkbox 方形/Radio 圆形/Switch 轨道等必要形状 |
| scaling | 见 §5.3，仅改变派生几何，不改作者字符串 | 颜色、alpha/opacity、无量纲字距、断点 |

- contrast/surface/indicator/track 等品牌补充角色采用本地 Themes 相应色族定义并 vendor 来源，不用“第 9 阶都是白字”的猜测替代；auto-gray 配对严格采用 `helpers/get-matching-gray-color.ts`。
- 生成责任集合为上表列出的 token 加 radius。v1 升级把这些已有作者键同时记入 overrides，从而后续选色族不悄悄改写旧外观；其他已知值仅留 base。新建标准主题 overrides 为空。
- 通用顺序：先由根到局部合并有效config并生成当前模式基础值，再叠文档显式override、由外向内的边界显式override，最后组件局部视觉选择。局部快捷配置本身不能越过显式语义override；解除override后才显现最近config生成值。组件 color 只改变该组件负责的色阶/角色；若调用方显式选色则该组件不再使用根 primary，来源说明需反映此选择。未选 color 的 Button solid、Link 等必须消费有效 primary/foreground，不能绕开作者覆盖。
- 改配置只改 config；被覆盖的角色继续生效，Panel 显示“配置已选择，N 项被覆盖”。重置分为“解除该 token”“解除分组覆盖”“恢复预设/文档”，须显示范围；不自动清空其它模式/分组。
- 边界未指定配置时继承父有效值；中性色默认继承父**已解析中性色**。仅改子品牌不会重新 auto 配灰；显式子 gray=auto 才随子品牌配对。根 gray=auto 而品牌 Custom 时使用有效默认 indigo 配对并显示该来源，不从 custom primary 猜色族。
- 边界的颜色/字体/尺寸/阴影独立可清除；空 patch 复用父 source/snapshot；appearance 切换重新取该模式 base/overrides，不复制当前模式充当另一模式。

### 5.3 几何与缩放

- `spacing` 是单位，`layout-gap` 是布局间距；迁移只搬字符串，不执行 px↔rem 二次换算。继续保留 `--rustify-unit` 与 `--spacing`/`--layout-gap` 的既有区别。
- 长度先按 ResolveContext 的 rem 基准解析为未缩放 CSS px；radii/shadow 完整层列表先派生，之后统一乘 scaling。GPU 只做现有 CSS px→points 换算，不再次乘 scaling；不用 `zoom` 或 transform 缩放整个 scope。
- 受 scaling 控制：字体字号/行高、主题 spacing/layout-gap、1–6 radius、控件尺寸与 padding、主题阴影所有层的偏移/blur/spread。letter-spacing 的 em、颜色、opacity 不乘；边框/焦点可见线宽、full 哨兵 9999px、断点及显式任意 CSS 长度不乘。字体资源本身不变。
- 参考 radius 基数 3/4/6/8/12/16px，factor=0/.75/1/1.5/1.5；full 只由允许胶囊形的组件消费。显式 radius token 覆盖时按其相对 radius-3 的比例生成有限 radius；full/thumb 独立规则仍受控。旧 Custom 路径维持原四档 radius 求值，新增组件用有效 radius 接入，不把旧导入强制改成 medium。
- 参考 spacing/字号/行高/控件尺寸表作为有来源的固定基数；spacing 作者值相对既有 .25rem 单位调整空间尺度，font-size 作者值相对既有默认字号调整字级，之后统一 scaling 一次。CSS 与 GPU 使用同一数值快照，表值不在多个组件重复写公式。
- V2 必须覆盖 shadow 派生中固定第二层的缩放、父110%→子继承/95%→解除，以及两模式不同公共作者值；不只测单个 radius。

### 5.4 作用域、CSS 与发布链

- 公共 Themes 入口提供 `rustify_components::theme::{Theme, ThemePanel, ThemeBoundary}`；`Theme` 是增强后的 SDK `ThemeScope` 的重导出，保留 resolved/source 接入方式而非第二个主题所有者。文档展示根 Theme 与嵌套 ThemeBoundary 的 Rust 用法；旧 `rustify_ui::Theme` 值类型不改名为组件。ThemeScope 仍独占一个 mount root；嵌套边界拥有自己的 Owner/context，CSS 写在自身节点，不改 root/body；新增写入键要纳入 cleanup，移除 patch 时清除上一次完整属性集合，避免残留角色。
- 浮层使用创建点的有效 resolved signal；模式、字体、度量和新角色随 snapshot 更新，不在 Portal 自己解析第二份主题。
- 保留一帧最多一次 PublishedFrame。revision/相等判断包含 config、base、overrides；相同文档/context/mode 复用 Arc。存储与历史在手势提交点执行，不能在 input loop 反复验证整个保存库。
- 无全局 preflight。SDK 皮肤统一经现有 Tailwind 构建/提交产物，不新增竞争的同名 utility 样式表。固定视觉枚举可使用静态 scoped 类，默认规则和响应式几何规则放在低于 utilities 的组件层并限制选择器权重；调用方 class 仍经既有 merge 路径并在同类 utility 上覆盖默认值。
- 响应式任意值复用 `geometry.rs` 的外部样式收集/constructable stylesheet 生命周期，扩展到媒体规则；CSR 不设置字符串 style 属性，SSR 将收集结果写外部 CSS，hydrate 使用同 seed/规则。规则限定本 scope，Owner cleanup/fatal release 均清理。
- 只接受单个对应 CSS 属性值；拒绝多声明、规则/URL/外部导入注入，枚举/数值使用类型校验。无效值给 prop 定位诊断，不静默吞掉；SSR 与 CSR 共享校验及序列化，不以浏览器专有 parser 决定服务端结果。

### 5.5 Button、IconButton、Link

| 组件 | P1 公共契约与默认 |
| --- | --- |
| Button | size=1–4，可响应式，默认2；variant=classic/solid/soft/surface/outline/ghost-offset/ghost，默认solid；color 未设继承，radius 未设继承，high_contrast=false，loading=false；原生 type 默认button，可显式 submit/reset；保留 on_click、disabled、unavailable、aria_label、controls、expanded、class |
| IconButton | 与 Button 同视觉选项；独立名称必填、图标装饰；复用同一原生按钮行为/样式规则，不复制点击状态机 |
| Link | 保留 href、Within/Exact、aria-current、disabled 移除 href；size1–9/weight/trim/wrap 响应式，默认不覆盖继承字号；underline=auto/always/hover/none，默认auto；color/high_contrast、truncate、class；保持原生链接及 target/rel 等标准属性透传 |

来源：Themes `_internal/base-button.props.ts`、`link.props.tsx`；Rustify 现有 button/link 行为。

- loading 强制阻止点击请求和原生重复提交，不允许 disabled=false 穿透；保留原名称，aria-busy 可见，内部 spinner 不重复朗读，宽度不因文字被移除而塌缩。disabled 永远优先；unavailable 仍表示“保留可达性并交付请求，由应用拒绝”，不得误改成 disabled。
- P1 不提供通用 asChild/Slot；这些组合能力在能力表标 P2。原生 button/link 语义先完整，不用 a 包 button 模拟替换。
- 旧 Button 映射：Default→Solid、Secondary→Soft+gray、Outline→Outline、Ghost→Ghost、Accent→组件局部角色补丁消费旧 accent、Destructive→局部角色补丁消费 destructive、Link→真实导航用 Link、纯操作用 Ghost 并显式下划线。旧 Default/Sm/Lg→2/1/3，Icon→IconButton size2。Accent/Destructive 局部补丁使用同一组件色角色路径，不能另留 LegacyButton 实现。
- `ButtonClass`/`ButtonVariant`/`ButtonSize` 等旧公共参数逐项列到迁移文档；更新仓内调用点再移除。颜色语义不能用“Accent→品牌色族”替换。

### 5.6 布局、排版与响应式

`Responsive<T>` 是单值或 initial/xs/sm/md/lg/xl 稀疏值；缺失沿最近较小断点，不修改应用数据。断点固定为无条件/520/768/1024/1280/1640px（Themes `styles/breakpoints.css`），不用 Tailwind 同名断点。共享 prop 以 Rust 类型/结构组合表达，不要求照搬 React prop spelling。

| 组件组 | 必须交付的参数/语义 |
| --- | --- |
| Box | as div/span，默认div；display none/inline/inline-block/block/contents；共享 Layout/Margin |
| Flex | 默认div、可span；display、direction、align、justify、wrap、gap/gap_x/gap_y；between映射space-between |
| Grid | 默认div、可span；display、areas、columns/rows（1–9或合法CSS）、flow、align/justify、align_content/justify_items、gap |
| Container | size1–4默认4；align left/center/right；display；内容 max-width 448/688/880/1136px 的参考尺度与响应式行为 |
| Section | section语义；size1–4默认3，纵向24/40/64/80px参考尺度；display |
| Inset | side默认all；clip默认border-box；p/px/py/pt/pr/pb/pl只接受current或0；消除最近容器 inset，不成为任意负 margin 逃逸 |
| Reset | 作用于调用方显式提供的单个原生元素，保留其语义/事件；提供类型约束的 Leptos element 装饰入口，不为此引入完整 Slot；不允许多元素悄悄丢失，也不施加全页 reset |
| Text | as span/div/label/p，默认span；size1–9默认继承；weight/align/trim/wrap响应式，truncate、color、high_contrast |
| Heading | h1–h6默认h1；size1–9默认6，视觉大小不决定层级；其余同Text |
| Blockquote | blockquote；size、weight、wrap、truncate、color、high_contrast；不凭空增加参考没有的align/trim |
| Code | code；variant solid/soft/outline/ghost，默认soft；size、weight、wrap、truncate、color、high_contrast |
| Kbd | kbd；size1–9；variant classic/soft，默认classic |
| Em / Quote / Strong | em/q/strong；wrap响应式、truncate；不新造独立size/color API |

共享参数按参考 `props/{layout,margin,padding,gap,width,height}.props.ts` 完整登记：padding/gap 0–9或合法CSS；margin/inset/edge可负刻度；width/min/max、height/min/max；position、overflow/x/y、flex basis/grow/shrink、grid area/column/row/start/end、align-self/justify-self。只在参考适用组件上提供，不将整组强塞进每个排版组件。枚举、CSS值和响应式都进入 V4 的参数矩阵。

字体 weight=light/regular/medium/bold，align=left/center/right，trim=normal/start/end/both，wrap=wrap/nowrap/pretty/balance；truncate 不是响应式布尔。默认值未设置与 CSS 初始计算值分列，M1 的能力表不将二者混写。主题缩放按 §5.3 作用于参考尺度，不作用于任意绝对CSS尺寸。

### 5.7 编辑器命令与历史

- 增加配置修改、token override 设置/解除、分组解除命令；全部从当前 v2 文档生成候选，验证两模式后一次替换。候选失败不改变文档、revision、undo/redo 或缓存。
- 原 14 COMMON_TOKENS 的显式编辑仍写两模式；shadow-color 仍是颜色项、不是公共字段。导入不同模式作者值保留到用户显式编辑；预览模式变化不修改任一作者值。
- 44 预设导入为 Custom v2，名字/色值/模式差异不变；选色族后保留原覆盖并显示解释。显式“使用配置值”只清理用户选择的责任组，不默认清空49项。
- 手势开始保存整个 v2 文档与redo，过程只预览，提交一次历史，取消完全恢复；输入框原生undo/IME继续优先。HSL shift只改变原本负责的颜色作者值，不另存一套色族状态。
- Inspector 已有 token 编辑入口继续工作，可增加 P1 主题来源说明；完整组件/状态 Inspector 不提前做 F19 的后期验收。

### 5.8 导入、导出与版本接入

- JSON：按schema_version分派；v1先沿用旧严格语法/校验，再转换§3.1；v2严格区分一般形式与等价Custom紧凑形式，混合字段/未知版本拒绝。未知字符串token在统一内存base保留。校验整个输入UTF-8字节及规范输出体积，均沿用256 KiB，不分段绕过；每种合法导出必须可重新导入。
- CSS：沿用数据解析器，保留浅值补深/缺项从候选base补齐的诊断；转换到 Custom、以导入值形成明确作者覆盖，不执行 CSS。不扩展 display-p3 主题输入；Colors P3 资产可用不代表主题解析器可用。
- Review 显示版本、名称、配置、两模式base与override新增/删除及诊断；持有 reviewed base/revision。Apply时编辑器已变化就拒绝过期候选并要求重新review；取消不写历史。
- 默认完整 JSON 导出 v2。显式“兼容v1”先列具体损失：色族/auto选择、appearance意图、radius full/组件尺度角色、scaling意图、覆盖来源、非旧49键派生角色；导出两模式最终旧token及可保留未知作者字符串，不伪称新组件外观/完整意图都能恢复。
- CSS 输出继续提供标准 Tailwind v4 与 Rustify scoped profiles、既有四种颜色记法；新增有效阶/角色/度量均在声明的profile内生成。标准 profile 的 :root/.dark 是用户主动导出选项，不混进 SDK 作用域CSS；scoped profile 不写宿主。CSS只是已解析结果，不能反推出config/overrides。
- Rust snippet和JSON独立消费者使用共享import/resolve，不借Studio内存。完整JSON保留appearance；“按当前预览接入”的Rust说明明确传递所选preview override，CSS标出其light/dark选择方式。普通provider默认遵守文档appearance，显式preview override时与Studio相同；对比时固定同一有效模式，不能用文档light与预览dark相互冒充。字体继续外部资源与许可；保留根/子路径资源基址。
- 接入说明必须写：旧 registry 0.2.1/旧CLI不能消费v2；本期完整模式使用收口记录中**确切源码SHA**的SDK和源码CLI开发模式，并记录实际 `--build-info`。收口前不填伪SHA或未发布semver。已有CLI严格registry配对路径不放宽；源模式按既有开发入口验证，未来公开发布再更新 `framework-release.json` 及发布清单。
- 复制Promise成功、下载实际发起后才报成功；拒绝/异常保留可选文本和下载路径。

### 5.9 能力目录与迁移资料

- `catalog.rs` 是能力事实源；扩展为能区分组件/子能力、参考来源、DOM/GPU路径、入口、支持状态、限制及验证入口的结构。不要用当前24类别数量冒充全部公共导出。
- 记录完整稳定参考集合与旧公共导出去向，但只把P1已实现且有对应检查者标可用；P2–P5、asChild、完整Primitives均明确未交付/部分。unstable单列候选，不纳入实现数量。
- `xtask catalog` 继续生成能力文档；编译/测试检查公开入口与目录不存在悬空、已支持属性都有实例。Stage证据在里程碑记录，不把一次浏览器成功硬编码为永久认证。
- 迁移文档至少覆盖v1→v2、旧accent与品牌色区别、Button映射、CSS入口/spacing语义、localStorage备份与回滚、源码版本配对。P1影响到的旧接口必须当期给替代，不以F23终验在P5为由延期修复破坏。

## 6. UI 与体验边界

沿用 [PRODUCT.md](../../PRODUCT.md)、[DESIGN.md](../../DESIGN.md) 与现有 Studio 中性双栏工具布局；不另做品牌化编辑站。

- 顶部保留主题名、模式、撤销/保存/导出。左侧顺序为预设搜索→基础配置→精调；右侧仍为真实预览与既有场景/renderer切换。ThemePanel在基础区与可嵌入样例复用，选择器有明确标签/有效auto结果。
- 基础配置用色族/中性色/面板/圆角/缩放，外观与预览模式语义标清；Custom与overridden以文字+状态标识显示，不只靠颜色或悬停。来源说明能回答“值来自哪里”“哪个覆盖在生效”“解除将影响哪些项”。
- 新增配置/错误/迁移文案同步中英文；不翻译主题名/token/作者数据。中性壳不消费正在编辑的主题，低对比主题不使工具不可用。
- 迁移待确认、缓存损坏、外部冲突、只读持久化、导入失败、复制失败均有常驻反馈与继续动作。保存失败不显示已保存；下载原始数据与当前主题出口始终可达（实例fatal除外）。
- 升级状态和允许动作按§3.3展示：先恢复旧草稿到内存，再Review、显式“升级并保存当前草稿”或取消；取消后常驻“仅内存编辑”和重新检查入口，模式/偏好变化不能绕过确认。待锁不禁用编辑/历史/导出；保存状态区分排队、先前快照成功而当前仍待保存，以及当前草稿确已保存，命名库动作单独反馈。
- 保留390px编辑/预览分区、768px/桌面与200% zoom的主路径可达作为本期改动回归；新增控件键盘可用、焦点可见、无装饰动画依赖。完整跨产品辅助技术/响应式终验仍按PRD后期分配，不冒称P1结算N1/N7整条。

## 7. NFR 与测量

| 关联约束 | 机制 / 失败行为 | 检查 |
| --- | --- | --- |
| N2/AC-1、C-4 | 同revision不重复解析；一帧一次发布；手势结束最终revision可达；无重建/空闲持续更新 | V9 |
| N3/AC-1 | scope限定选择器、CSSOM/外部样式、无preflight；失败有诊断，不放宽CSP | V3/V4/V8 |
| N3/AC-2、N6/AC-1 | 不执行导入；全候选验证；未知版本/颜色/字体失败不应用 | V2/V6 |
| N8 | 按需sRGB/P3、source锁与许可；脱离ref重建/包内容一致 | V1/V7 |
| N7/AC-1、C-3 | 中性工具壳、可见状态与操作；真实浏览器而非仅HTML快照 | V5/V10 |

性能没有新增毫秒SLO。M1先以当前工具链采集可比基线，M7同机同Chrome/GPU、viewport/DPR/rem基准重测旧40次颜色编辑负载，再单列新增配置切换负载；记录p50/p95、解析/帧提交计数、最终revision、long tasks、region/canvas身份、5秒idle、WASM/CSS gzip尺寸和新Colors资产。异常增长需定位解释，不用“没有SLO”放过持续积压/泄漏。

现有 `theme-performance.spec.ts` 固定写旧 `docs/validation/theme-studio/M6/performance.json`；M1先增加 `RUSTIFY_THEME_PERF_OUT` 证据输出环境变量（新增测试参数），保留旧默认行为；前测指定 `docs/validation/dom-components-p1/performance-before.json`，后测指定 `docs/validation/dom-components-p1/performance-after.json`，M1/M7 Markdown记录链接对应JSON，不覆盖历史。headless/软件GPU数据不伪称实机绘制数据。

## 8. 失败模式、恢复与回滚

| 失败/触发 | 用户表现与数据后果 | 检测 / 恢复 | 验证 |
| --- | --- | --- | --- |
| JSON/CSS/配置校验失败或超限 | 候选错误，当前文档/历史不变 | 字段诊断；修改或取消 | V2/V6 |
| 导入review后主题变化 | 不应用旧候选 | stale提示，重新review | V6 |
| 取消本地旧库升级 | 当前主题/历史不变，v1保留，不创建v2/备份 | 持续写保护；可内存编辑/导出/取回原文及重新Review；重开仍待确认 | V6 |
| 旧库单项坏数据/未知版本 | 不写v2，不自动清理v1 | 原始下载、明确重置/重新导入；不篡改其它项目键 | V6 |
| 备份写入/回读失败，或备份成功但v2写失败 | 不显示迁移成功；原文、有效草稿和已成功写的备份保留 | 错误说明与导出；修复存储后幂等重试 | V6 |
| quota/denied/无写锁 | 内存草稿可编辑可导出，保存事实不变 | 常驻只读/失败说明；不自动删旧库 | V6 |
| 待锁期间编辑/撤销或连续库动作 | 旧成功仅确认所存快照，不回退新草稿/历史/checkpoint | 单写者保序、最新草稿续写；失败暂停，库动作显式重试/取消 | V6 |
| 两新标签同时写/决定后第三次写 | 锁内版本不符转冲突，不静默合并 | Load/Keep/Save copy重新选择 | V6 |
| 写入与清除竞争 / 待锁期间dispose或fatal | 清除后不被旧pending重建；失效请求不再写库或更新已销毁状态 | 同写入口比较清除范围；取消待锁及生命周期guard；已提交数据不倒撤 | V6/V8 |
| 旧标签改v1 | v1保留，v2不被旧数据自动覆盖 | storage事件及启动摘要比对；显式导入 | V6 |
| boundary移除/scope释放 | 父继承恢复，其它scope继续 | 清理CSS属性、规则、订阅；宿主sentinel比较 | V3 |
| GPU局部context/字体资源失败 | DOM编辑、保存、导出仍可用；GPU明确失败 | 既有恢复链路重放最新有效snapshot | V8 |
| wasm实例fatal | 本实例所有scope/编辑终止，不承诺未持久化输入可取回 | 既有fatal UI与重启；从最近持久化draft恢复，独立实例继续 | V8；不建设新的隔离模型 |
| 撤回P1源码/静态产物 | 旧站与v1库仍可用，v2新编辑不会自动回灌 | 原子使用同版wasm/CSS/字体/manifest；保留v2 JSON或显式有损v1文件 | V8 |

## 9. 验证

### 9.1 检查定义

| ID | 检查与可观察断言 | 位置 / 时机 |
| --- | --- | --- |
| V1 | Colors全量字符串/阶序对照、sRGB入口无P3、P3有sRGB回退；无脚本静态页两模式/多根隔离；vendor可重现与许可 | 新Colors单测/xtask + 静态browser fixture；M1局部、M7集成 |
| V2 | v1/v2往返、未知token/版本/重复键、两模式保留；完整auto配对/责任表/override解除；尺度/阴影仅乘一次 | DOM内核单测；M2 |
| V3 | 两mount scope+嵌套强制相反模式+局部浮层+host sentinel；父配置变化/解除后继承；CSP与规则清理 | catalog/主题浏览器fixture；M3 |
| V4 | P1组件/参数矩阵可编译消费；Button全部变体/size/loading拒绝；Link原生导航；布局语义、每断点上下边界、长文长链接、Reset边界、class覆盖；外部CSS SSR确定性 | 组件单测/SSR单独构建 + browser；M4 |
| V5 | 44预设、配置与精调往返、覆盖解释、双模式、撤销/重做/no-op/取消/草稿native undo；中性壳与关键错误状态 | Studio state/controller单测+browser；M5 |
| V6 | 全候选导入与stale Apply；§3.3升级确认/取消持续写保护、失败重试与原文取回；同页队列/快照确认及最终草稿恢复；真实同源双页并发写与Load/Keep/Save copy、第三写、清除竞争、dispose/fatal、无锁降级；真实复制结果；具体断言见下 | codec/persistence/controller单测+fresh-context browser；M5/M6，fatal关联V8/M7 |
| V7 | sources/css/catalog漂移；公开入口/能力状态/迁移表完整；ref不是构建依赖；纯DOM依赖图无Makepad | xtask/包内容/文档检查；M6 |
| V8 | 独立Colors静态/DOM-only JSON/Rust/标准与scoped CSS消费者，根/子路径、字体、strict CSP；源码Docs主题构建；既有GPU投影/局部故障/fatal恢复回归 | 独立workspace/Docs及browser；M6局部、M7全量 |
| V9 | 同环境旧负载前后+新配置负载；一帧一次、最终值、idle/对象身份/资源释放及压缩尺寸 | headed Chrome evidence、report-size；M1前测/M7后测 |
| V10 | 三个角色闭环及UI走查：入口、步骤、关键边界、截图、console/失败请求、实际可用性；中英文/浅深/390/768/桌面/200% | 真实浏览器；M7 |

V6持久化/导入风险门在M5退出前完成，M6补导出/复制；下列fatal场景结合V8在M7收口，其余在M5验证。这些不是本次文档修订已运行的测试：

- 外页只持有同名锁、不写存储 → 本页autosave或命名保存A → 本页编辑到B、撤销/重做、继续autosave并交错命名保存/删除 → 释放锁。逐次核对所存快照、op结果、草稿/undo/redo/checkpoint及保存反馈；连续autosave只合并相邻未发出项，库动作保序且不丢，自身前次成功不误报外部冲突，队列完成后重开恢复最终草稿。另测旧成功到来时有活动手势、模式/偏好变化以及写失败。
- 有效v1、无v2首次启动 → 核对恢复值/偏好和空历史 → 取消升级 → 编辑/切模式/改偏好并重开。断言无v2/备份写入、原v1逐字节不变，取消不改当前主题/历史，后续autosave不能偷升；重开仅恢复原v1。再显式Review并升级，核对当前草稿与完整库成功恢复；Review过期、v1变化、备份/提交失败保持保护和原文，显式重试幂等。保留坏v2不覆盖、未知版本/坏单项不部分迁移、重入及旧页改v1后全关再开仍能取回原备份。
- 同源双页真实写入与清除缓存/坏库重置/单独清备份竞争：比较范围变化即重新确认，失败不报清除成功，普通清除不删v1/备份，清除后旧pending不重建库。Load/Keep/Save copy后的第三次写仍重新决策，未处理命名动作保持可见。
- 外页延迟授锁时dispose/fatal，再释放锁：旧请求不能写库或访问已销毁signals；实例重启只恢复最近已提交草稿。另验证授锁后guard、无锁/denied/quota降级；fatal结果不冒称未保存输入可取回。

### 9.2 命令与运行顺序

下列已存在命令用法来自 README、AGENTS、Playwright配置和独立消费者README；`colors`及带“新增”的fixture须先实现，不能提前声称可运行。CSR与SSR/Hydrate检查**分开**，不要用workspace feature合并污染渲染模式。

```sh
# 环境与轻量本地反馈
mbx xtask doctor
mbx test --locked -p rustify-dom --lib
mbx test --locked -p theme-studio --bins
mbx test --locked -p rustify-components --no-default-features --features ssr --lib
cargo fmt --all -- --check
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check

# 新增入口（M1后）
mbx test --locked -p rustify-colors
mbx xtask colors --check

# Studio浏览器前必须重建；逐例、逐project串行
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example theme-studio --release --base /tools/demo/
npx playwright test --project=theme-studio --workers=1
npx playwright test --project=theme-studio-deep --workers=1

# 独立消费者；脚本使用自己的workspace/lock并离线构建
sh tests/theme-export/build.sh
sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
npx playwright test --project=theme-export --workers=1
npx playwright test --project=theme-export-deep --workers=1

# 当前性能脚本先按§7提供P1独立证据出口，之后才运行，不覆盖旧M6文件
RUSTIFY_THEME_PERF_OUT=docs/validation/dom-components-p1/performance-after.json RUSTIFY_TIER=evidence npx playwright test --project=theme-studio theme-performance.spec.ts --headed --workers=1
mbx xtask report-size --example theme-studio --release --compressed
```

- `mbx xtask verify --suite p1` 是既有 WASM UI 期次，不是本计划验收总入口，不能混名代替V1–V10。
- M6分别运行 `mbx test --locked -p rustify-docs`、`mbx test --locked -p rustify-docs-cli`；源码CLI以 `mbx build --locked -p rustify-docs-cli --profile docs-cli` 构建，作者fixture的三个直接框架依赖使用该checkout的canonical path（`release.rs::validate_pair` 既有source规则），不替换成Git依赖绕过配对。M7运行AGENTS中受影响root/fork检查与示例构建组合。具体命令及结果记M6/M7，不复用旧计划通过数字。
- 新静态Colors与DOM-only主题fixture扩展现有独立workspace/Playwright项目；纯DOM消费者必须单独依赖无gpu features的SDK，不能以原GPU导出fixture代替。
- browser各project单独执行，同一示例build与browser不可并行。端口/服务由现有配置管理，碰到用户服务占用先核对，不能杀无关进程。

### 9.3 角色与真实入口验收

1. **Colors消费者**：只给公开Colors README、资产路径，不给内部源码讲解；建立无JS静态页，显式选两色族/两模式/P3增强，检查请求无wasm/JS和样式不越界。记录卡点。
2. **主题作者**：从现有Studio正常入口，选预设→改品牌/auto灰→识别旧覆盖→解除一组→精调→嵌套/局部浮层观察→撤销/重做→保存→reload→导出；再走旧库升级取消/内存继续/重新确认、待锁继续编辑与保存反馈、坏缓存/写失败/跨标签冲突。不得借测试后台函数替代所有用户操作。
3. **独立DOM应用开发者**：只给导出包与迁移文档，在独立workspace导入JSON/编译原Rust snippet、加载CSS及外部字体；比较light/dark、config/override、根/子路径与scope外sentinel。另以旧CLI证明v2拒绝并给升级路径，不能误报兼容。

以上由Agent模拟，不是用户研究统计。真实浏览器须显示最新build；保存必要截图及console/CSP/失败请求。人工读屏、OS IME等未实际进行则明确未验证，不用axe或HTML快照代替。V10未通过不记本期完成。

## 10. 里程碑

| ID | 交付物与边界 | 前置 | 退出检查 |
| --- | --- | --- | --- |
| M1 | 冻结P1字段/旧公共入口能力矩阵；Colors包/vendor/生成器/来源许可；性能输出隔离并取改前基线 | 现有工作树/工具链核对 | V1数据与静态局部检查、V7来源检查、V9前测；范围不重新待定 |
| M2 | v2、v1升级、config责任表/来源、双模式resolve、尺度与shadow单次缩放 | M1数据与矩阵 | V2全部；旧v1解析回归 |
| M3 | 带来源的ThemeScope/Boundary、CSS投影/清理、已有浮层继承与共享GPU适配 | M2接口稳定 | V3；V8受影响GPU共享值局部回归 |
| M4 | Button/IconButton/Link、ThemePanel、全部布局排版、响应式/SSR外部CSS；迁移受影响调用点 | M2/M3；静态组件工作可与M3错峰 | V4；V7公开入口局部检查 |
| M5 | Studio基础配置/覆盖解释、历史命令、导入review；§3.3升级许可门、同页单写者/快照确认、锁内冲突/清除及生命周期保护、错误恢复 | M2–M4 | V5；V6持久化/导入与数据风险门；旧44预设回归 |
| M6 | 完整JSON/CSS/Rust导出、显式v1损失、DOM-only/Docs源码消费者；能力/迁移/设计文档 | M3–M5 | V6导出、V7、V8独立消费者局部检查 |
| M7 | 当前源码配套收口，全部受影响回归、三角色浏览器验收、性能前后与恢复证据 | M1–M6 | V1–V10阶段终验，所有缺口有真实通过或已获授权范围变更 |

提交点：M2、M4、M6阶段收口分别提交其累计相关改动；M7完成最终提交。各提交只纳入本计划变更，已有未提交PRD/评审资料的归属须先核对，不擅自打包进代码提交。阶段通过证据未失效可引用，避免每个小改动重跑昂贵GPU/全部浏览器。

并行边界：M1数据/来源与能力盘点可独立；M2文档/codec/resolve必须统一接口；M4样式/组件可在接口冻结后分工，但`lib.rs`/`catalog.rs`/CSS入口由一个集成人管理。M5同一controller/state/persistence链不能让多个实现者分别定义迁移语义。浏览器/GPU/同例构建保持串行。

## 11. 风险与未决项

| 风险 | 应对 / 最晚确认点 |
| --- | --- |
| 只在已解析快照加config导致另一模式丢失或重复缩放 | M2先测试双模式source与完整shadow派生；M3不得继续用旧patched临时双模式复制作为新边界 |
| 品牌色族把旧accent/预设覆盖洗掉 | v1 Custom +责任集合override；F4优先级测试是M2硬门 |
| 直接复制参考CSS污染宿主/依赖P3 | Colors显式根、Themes运行态属性；P3独立入口；V1/V3/CSP |
| 属性支持数量被“组件存在”掩盖 | M1逐字段矩阵，M4逐prop/断点测试；组合asChild只标后续能力，不伪称完整 |
| localStorage事件滞后、同页旧请求回写或迁移quota | §3.3单写者/快照确认、锁内比较、原键保留、只读降级；M5/V6延迟授锁、连续保存/库动作交错及失败注入 |
| 取消升级后被autosave绕过 / 清除或dispose后旧请求续写 | §3.3持续写保护、同写入口及生命周期guard；M5/V6重开与清除竞争，M7/V8收口fatal恢复 |
| v2源码可用但公开0.2.1仍不兼容 | 不改已发布配对事实；M6/M7记录实际源码SHA与CLI模式，明确未公开发布 |
| 完整UI/性能环境不可用 | 保留可执行本地单测成果；M7停在未验收，不替换为静态截图或旧数据 |

没有需要重新确认的产品范围决策或未解的数据提交前置；本计划为Ready。§3.3已明确同页顺序、快照确认和本地升级取消语义，V6/M5承接相应实现风险门；环境核验、真实并发/故障/UI与性能结果仍须按里程碑取证，不是文档就绪的已通过产品证据。公开发布版本号/发布日期不影响本期源码交付，属于范围外发布决策。任何无法满足现有PRD约束的新发现，按实施协议记为具体阻塞，不自行降低AC。

## 12. 实施注意

- [AGENTS.md](../../AGENTS.md)：读到文件原文才写引用；不要沿旧计划路径误改 `crates/rustify-ui/src/theme/`，真实主题内核在 `rustify-dom`。
- `persistence.rs` 才是Studio存储入口；旧坏缓存写保护需保留，新增升级许可门；命名库仅在写成功后确认，当前编辑器只能按§3.3的代次条件更新，不从同步成功分支照搬整份editor回写。
- v2用 `base_values(mode)` 显式访问base，编辑通过override命令；`ResolvedTheme.values` 定义为该模式配置/覆盖合成后的未缩放有效作者表。原 `ThemeDocument::values`/`values_mut` 不再保留歧义语义，迁移调用点并列入API迁移表。Autosave改用完整文档revision/context/mode的一致性判据；所有直接`.styles`调用整体迁移，不能只改serde结构。
- 参考默认prop与CSS computed默认分开；嵌套gray继承父resolved值，不能无意每层重算auto。
- 新包/CSS资源须经包内API消费，检查Cargo包内容；不跨crate include未打包文件，不依赖本机ref。
- 不用重新美化Studio壳、重做Docs首页或重写既有GPU控件来掩盖主题接入问题。
- 本计划的源码配套验收不等于发布；不推tag、不publish、不更新生产部署，也不修改外部平台记录。
