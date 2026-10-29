# DOM-COMPONENTS-P1-gaps · 导出接入资料与当前 API 说明补齐（DOM · 既有交换格式）

```text
::OBJECTIVE{implementation}
  target: 直接依据本计划、仓库规则和当前代码实施或续做
  ACCEPT: 全部里程碑退出检查通过，按「实施进度」完成最终提交、SHA 回填与自查，并提示用户发起代码评审
  实施须知: 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
```

> **计划状态：Ready**
>
> 调查基线：2026-10-29 · `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e` · main / dirty。产品源码与 HEAD 一致；PRD、部分评审和台账尚未提交，输入以工作树及差异报告中的指纹为准。
>
> 需求来源：[DOM-COMPONENTS PRD v0.2](../PRD-DOM-COMPONENTS.md) · **P1**；补齐 [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md)。增量依据：[覆盖差异报告](DOM-COMPONENTS-P1.prd-dev-gaps.md) G-01、G-02、V-01；无待决 C、无新增用户产品决定。
>
> 本期交付：**默认导出即可理解并带走的接入资料、与当前 API 一致的说明，以及真实反馈区域的存储回归断言。**
> **开发范围：本仓 Theme Studio 导出界面/资料生成、主题指南和迁移说明、直接相关的导出/存储验证。**
> 本期独特职责：补齐原 P1 已承诺的版本/资源/使用说明，不重做主题配置、解析或存储实现。
> **顶层排除：P2–P5 组件能力、GPU/性能调优、主题 schema 或存储键变更、registry/官网发布、历史 PRD/计划/完成记录改写。**

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

- 最近更新：2026-10-30T00:08:42+08:00
- 当前进度：2/2 个里程碑完成
- 当前状态：代码初审两项修复已验证、提交并回填 SHA；待独立复审
- 最近完成：M2 · 独立消费与浏览器阶段收口
- 下一步：由用户发起 `$review-code docs/plan/DOM-COMPONENTS-P1-gaps.md` 独立复审
- 当前阻塞：无；历史 U-01 不在本增量退出范围
- 代码基线：原实现 0abf0f39；本轮修复 17544a64，浏览器构建于 dirty@dc9df3547c4c164af7cf3433bef883332659b6f6（build ID 7dabd60834cb1030）；源码身份与局部补验见 M1/M2 记录

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-30T00:04:57+08:00 | 原交付保留；代码初审 F-01/F-02 已补齐静态根标记并正确编码 ID，9 项模块测试及局部检查通过 | [M1 记录](DOM-COMPONENTS-P1-gaps.records/M1.md) | 0abf0f39；修复 17544a64 |
| M2 | 已完成 | 2026-10-30T00:04:57+08:00 | 修复版重建后导出 spec 5/5 与展开说明走查通过；其余未改行为证据限定复用，待独立复审 | [M2 记录](DOM-COMPONENTS-P1-gaps.records/M2.md) | 0abf0f39；修复 17544a64 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F20/AC-3 | 功能 | PRD §3.1、§7.2 P1；差异 G-01/G-02 | 完整导出的源码版本、资源和独立接入信息可获取且正确 | §5–§6；V-1/V-2/V-3 | 已确定增量 |
| WORK-N5/AC-1 | 可维护性 | PRD §3.2、§7.2 P1；差异 G-02 | 指南、屏内调用与当前公开 API 一致 | §5.2；V-2/V-3 | 已确定增量 |
| WORK-F21/AC-2—AC-3 | 验证维护 | PRD §3.1；差异 V-01 | 保留 quota、命名失败/重试、拒绝存储的数据断言，改为观察现有实际反馈区域 | §5.3；V-3 | 行为已实测成立，测试待同步 |
| WORK-N6/AC-1 | 可靠性 | PRD §3.2；差异 V-01 | 已保存草稿、未保存草稿和命名动作失败分辨清楚，不显示假成功 | §5.3、§8；V-3 | 不改存储协议 |
| V-01 | 验证来源 | [覆盖差异报告](DOM-COMPONENTS-P1.prd-dev-gaps.md#52-v-01--存储行为成立但三个回归用例仍断言旧文案) | 原报告三处文案断言未同步，不是新产品缺陷 | §5.3；V-3 | 待同步测试 |
| C-1 | 约束 | [AGENTS.md](../../AGENTS.md)、原计划范围 | DOM 呈现，保持中性壳/中英文/窄屏与 strict CSP；资源外置 | §6；V-3 | 已确认 |
| C-2 | 约束 | 原计划 §5.8；当前 schema 与配对代码 | 完整 JSON 字节/格式不加入隐藏 metadata；源码 SDK/CSS/CLI 与已发布 0.2.1 区分 | §5.1；V-1/V-3 | 已确认 |
| C-3 | 约束 | AGENTS.md、[工具链](../../mise.toml)与原记录 | mbx 与现有测试入口；真实浏览器串行；不覆写旧截图，不新建基础设施或升级依赖 | §9、§12 | 已确认 |

跨期 WORK-N5 的 AC-2 留 P5。本表只承接上述确定增量，不重新结算原计划其余 AC，也不把现有数据保护功能重新列为未实现。

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 接入信息载体 | 导出框公共说明 + 可下载纯文本接入说明 | 各格式用户都能看到前提，离开页面后仍可取用；增加一个辅助下载入口，JSON schema 不变 | G-01；现有 ExportArtifact/download 路径 |
| D2 | 版本口径 | 沿用实际构建的 RUSTIFY_STUDIO_SOURCE_REVISION | 显示确切源码修订及公开 0.2.1 的限制；源码 CLI 只对使用 Docs 的路径要求，静态 CSS 不被误说成需要 wasm/CLI | 原 build.rs 与 export Rust 分支 |
| D3 | 错误反馈检查 | 观察已存在的标题/草稿/命名结果区域 | 不恢复冗余旧文案；数据、重试和可导出契约仍保留 | storage_controls.rs、专项浏览器证据 |

这些是可逆的局部资料与表达决策，无新增共享模块、外部服务、数据生命周期变化或难回退架构，省略 ADR。

### 0.4 职责与事实所有权

- 主题内核的 `export_json` / `resolve` 仍拥有格式校验、实际解析值与字体选择事实。
- Studio `data_tools.rs` 拥有本次接入说明、输出格式相关提示和真实下载反馈；从现有事实生成说明，不反向修改主题。
- `docs/themes.md` / 迁移指南拥有公开接入步骤；原 PRD、原计划和历史完成记录保留原位。
- 测试观察公开 UI/导出和存储结果，不重新定义存储事务、队列或权限。

### 0.5 明确不在本期

- 原报告 U-01 的多轮性能归因、P5 完整性能和人工辅助技术/真机终验，保留原去向。
- 原计划其余已交付逻辑、P2–P5 的新增组件与完整迁移；不增 GPU 路径。
- 改 schema、增加新 JSON 字段、嵌入字体、打 ZIP 或自动下载全部字体；接入说明只是现有导出的配套文本。
- 自动发布、更新 `framework-release.json`、升级版本或部署官网；源模式成立不代表 registry 已支持。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **已核实·足够**：`data_tools.rs::ExportArtifact` 已有 text/name/mime/losses；`download` 在实际发起后显示真实反馈。Rust 分支包含构建源码修订，CSS 输出包含外部字体路径与许可说明。
- **已核实·缺口**：默认 JSON 导出没有上述接入前提；版本提示局限于 Rust 分支。两格式以外的信息不能要求用户自行发现。
- **已核实·缺口**：`docs/themes.md` 前后两段 provider 契约冲突；屏内 preview 调用缺 container/config。实际 `rust_snippet()` 已提供完整四参函数。
- **已核实·足够**：独立 DOM fixture 有编译现有原始 Rust 片段与 JSON 的消费路径，普通 provider 与 preview 分开；当前源码重建还须满足下一项的修订输入。存储专项验证保留数据和可恢复路径，但三条旧文案断言失败，详见原差异 V-01。
- **已核实·缺口**：生成器 `include!` 整份 `data_tools.rs`，其中的 `env!("RUSTIFY_STUDIO_SOURCE_REVISION")` 不会继承 Studio 包的 build.rs 注入；未设置该变量时 host 编译失败。采用 §4/§9.2 的命令级显式注入，生成器最小编译已核实可通过；完整 `--dom-only` 重建仍由 M2 验证，不以旧产物代替。
- **未核实阻塞**：无。生成器修订输入路线已明确；工具、浏览器与本地 strict CSP 入口沿用调查基线，最终同版产物按 §9 验证。

### 1.2 文件清单与复用锚点

| 路径 | 本增量职责 |
| --- | --- |
| `examples/theme-studio/src/data_tools.rs` | 复用 ExportArtifact、output/download、tr 和现有 tools-summary/tools-status；增加公共版本/资源提示与纯文本接入说明；修正屏内调用；追加资料契约测试 |
| `tests/browser/theme-export-ui.spec.ts` | 验证默认 JSON/其它格式均有资料，真实接入说明下载、所选模式与字体资源、原 JSON 字节不变 |
| `tests/browser/theme-storage.spec.ts` | 只同步三处错误反馈观察区域，继续执行并保留原 bytes/草稿/库/重试/导出断言 |
| `docs/themes.md`、`docs/dom-components-migration.md` | 同步当前 provider、模式、校验入口与可操作接入步骤；必要时修正指向完整原片段的说明 |
| `tests/theme-export/README.md` | 同步本计划已确定的显式修订注入命令，供独立角色仅靠公开资料重建；不改构建脚本 |
| `tests/theme-export/generator/src/main.rs` | 仅当验证当前屏内调用需要时扩展现有生成片段的消费断言；保持编译真实 SDK/原片段的路径，不引入替代实现；编译期修订由 §9.2 的命令传入，不新增 build.rs 或改 build.sh |
| `docs/plan/DOM-COMPONENTS-P1-gaps.records/M1.md`、`M2.md` ★ | 按里程碑实际发生后创建记录；不预建占位 |

不修改库解析/provider API、存储队列或 runtime。若代码事实表明必须改变这些契约，先记录具体阻塞和原因，不能顺带扩大本计划。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| 主题内核 | 已核实足够 | `theme/codec.rs::export_json/export_css`、`resolve.rs::resolve` 保留双模式与字体 | 读现有公开结果，不新增 metadata schema | 解析/导出失败沿用 ThemeError，不返回假可用说明 |
| 构建身份 | Studio 已核实足够；生成器修订输入已核实缺口 | `examples/theme-studio/build.rs` 通过 `git rev-parse HEAD` 向本包注入修订；generator include 的同一源码也需要编译期变量，但没有对应 build.rs | 独立检查/构建命令从仓根读取同一 checkout 的 HEAD，显式赋给 RUSTIFY_STUDIO_SOURCE_REVISION，经 mise → mbx → rustc 传入；不新增构建脚本 | Studio 无法取得时沿用明确 unavailable 说明；独立消费要求可取得修订并与 Studio 一致，否则保留构建/身份验证失败，不编造修订 |
| 独立消费 | 消费路径足够；裸 build.sh 重建缺少上述输入 | `tests/theme-export/README.md`、generator 的 include 与 `build.sh --dom-only` 的 mbx 调用；显式注入后最小编译通过 | 按 §9.2 从未预设该变量的 shell 执行显式注入入口；M2 完整重建后核对 Studio、配套资料与 fixture 同源 | 不混 registry/source、不修改发布配对记录；旧产物可运行不等于当前源码可重建 |
| 本地存储反馈 | 已核实足够 | storage_controls 的标题/草稿/命名区域；报告 browser-probe 的三失败场景 | 只校正测试的观察对象 | 不能用放宽断言掩盖数据或重试失败 |
| 浏览器/服务 | 已核实足够 | Playwright 配置 theme-studio 4181，strict CSP；现有 release 构建入口 | M2 重建受影响 Studio，再串行验收 | 环境缺失明确未验证，不另装服务 |

## 5. 核心机制与接入说明

### 5.1 各格式共用的说明

在现有导出格式选择之后、代码框之前显示一块中性说明：源码修订、该输出的适用方式、v2 与公开 0.2.1 的边界，以及外部字体/许可。格式切换不重置主题、历史或预览模式。

- JSON v2 和 Rust 接入：匹配该修订的源码 SDK/CSS；使用 Docs 时从同 checkout 构建 source CLI，公开 0.2.1 不能读取完整 v2。不得把 workspace 同名版本号当可消费证明。
- CSS：说明它是两模式解析结果，静态 CSS 消费不要求 wasm；Tailwind profile 沿其既有构建入口。使用新组件或 source Docs 时才要求相应同源资产/工具，不向纯 CSS 用户强加运行时。
- 外部资源：按实际解析的两模式 sans/serif/mono 给出请求字体、有效字体及 `resource_path`；去重资源路径，有 packaged face 时包含 `FONT_GLYPH_FALLBACKS`。借鉴 `theme/codec.rs::export_css` 已有的逐字体与 fallback 逻辑，不复制字体数据库；无资源路径时写明系统/非打包字体，不能伪造文件。

在 `data_tools.rs` 内增加一个局部函数：

```rust,ignore
fn integration_notes(
    document: &ThemeDocument,
    context: &ResolveContext<'_>,
    preview_mode: ThemeMode,
) -> Result<ExportArtifact, ThemeError>;
```

返回 `theme-setup.txt`、`text/plain`，内容包括同一源码修订与版本边界、两模式/preview/context、完整函数调用、所需 CSS、字体 URL/许可来源和子路径设置。沿用 `ExportArtifact` 和现有 `download`；导出页提供“下载接入说明 / Download setup notes”辅助动作，不新增模态层、ZIP 打包或独立资料模型。屏内展示和下载均从现有 document/context/preview_mode signal 读取当前值，显式传入三参；不能从作者 appearance 或 ResolveContext 推断预览模式。说明按用户打开/下载时生成，最多显式解析两模式；不放进连续颜色输入、发布帧或 autosave 热路径。

JSON 输出仍直接来自原 `export_json`。纯文本不作为 JSON token、schema 字段或 CSS 输入；不把说明下载宣称成实际字体下载。失败保留有效主题/原输出，沿用可见错误和复制/下载出口。

### 5.2 当前 API 的单一说明口径

- `docs/themes.md` 后段改为 `Signal<ThemeBoundaryPatch>`、独立两模式 patch/局部 appearance 与 `ResolvedTheme::with_boundary`；模式无关旧 `ThemeValuePatch` 仅经 `.into()` 明确适配。与前段保持一致，不删除有价值的旧入口迁移解释。
- 指南若保留“与 Studio 导出确切相同”的声明，片段必须包含当前 with_context/preview 等实际导出；否则明确该段为简化例子，并链接完整导出/独立消费者。采用后者，避免在文档重复维护整份片段。
- 屏内与配套文本均展示真实四参顺序，例如 `mount_exported_preview(container, config, ThemeMode::Light, &font_context(16., false))`；mode/rem/reduce-motion 从当前 signal 取值，不固定为例中的 Light/16/false。说明 `config: MountConfig` 的 base 使用部署路径，container 是应用自己的挂载节点。普通 `mount_exported_theme` 遵守作者 appearance，显式 preview 不回写文档。
- 全仓搜索相同旧说法，只同步当前消费文档中的同源错误；历史 PRD、计划、评审及完成记录保留其原版本，不修改历史事实。

### 5.3 旧存储测试的最小同步

`theme-storage.spec.ts` 中 quota 用例观察 `.tools-storage-state > strong` 的失败/存储已满和内存草稿状态；命名失败观察 `named-storage-feedback[data-result=failed]` 与该名称/Retry；拒绝存储观察常驻 unavailable 标题及编辑/导出出口。保留各用例后面的实际存储字节、当前草稿/modified、空库与显式重试成功断言，不能只删掉失败文本检查后结束用例。

原三个失败场景必须在最终同一版 release 上跑完；调查期额外 probe 不是现有回归转绿的证明。

## 6. 界面与体验

沿现有 `tools-summary`、`tools-status`、`tools-actions` 和中性编辑壳设计，不改导出主动作层级。常驻说明在默认 JSON 状态即可读，使用明确标签而非悬停帮助；长源码修订可换行，字体路径使用可选文本/局部滚动。辅助下载用现有次级按钮，保留焦点和真实失败反馈。

新增提示和动作同时提供中英文，不翻译主题名/token/作者值。390px、768px、桌面以及原有 200% 主路径可达；资料变长后对话框仍可滚动，复制/下载按钮可达、焦点限制与 Escape 回焦保留。设计与真实浏览器检查按 AGENTS 的界面规则执行，当前方案只增加 DOM 文本/动作，不新增 GPU 行为。

## 7. NFR 与边界

读现有解析/导出结果，字体仍通过外部静态资源分发；不增加网络请求或后台服务。说明生成属于低频导出动作，不改变主题发布、GPU region/实例身份和持久化热路径。若只按此改动，不为资料文案重复性能测量；发现实际修改输入/发布链才扩大受影响检查。

## 8. 失败模式与回滚

| 触发 | 爆炸半径/数据后果 | 用户表现与检测 | 恢复 | 验证 |
| --- | --- | --- | --- | --- |
| 说明解析/生成失败 | 仅本次辅助说明，无作者文档写入 | 可见真实 ThemeError；不宣称已下载 | 保留 JSON/复制出口，修正后重试 | V-1/V-3 |
| 浏览器拒绝下载/复制 | 原有效主题和作者数据不变 | 沿现有失败/Download started 语义，不假报磁盘写入成功 | 选择文本、复制或重新下载 | V-3 的现有复制拒绝与实际下载 |
| 资料变长遮住动作 | 导出任务可达性风险 | 三尺寸/200% 实测发现 | 修复局部布局，不改主题 | V-3 |
| 测试只匹配新文案而漏掉数据失败 | 回归保障不足 | 数据/库/重试断言仍须运行 | 保留原数据断言，并按实际反馈区域定位 | V-3 |

不改变 schema/存储/公共 API，回滚仅撤回资料、文档与测试增量，不需要数据迁移。源码交付与资源同版；公开发布另走仓内流程，不在本计划执行。

## 9. 验证

### 9.1 检查定义

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F20/AC-3、C-2：各格式说明与源码修订/两模式资源正确；同一 document/context 分别传 Light/Dark 时，说明中的完整四参调用准确反映该 mode；原 JSON 不变，生成失败不修改文档 | `mise exec -- mbx test --locked -p theme-studio --bin theme-studio data_tools::`；在现有模块测试追加必要资料契约用例 | M1：资料生成、界面提示、文档和测试批次齐备后局部退出 |
| V-2 | WORK-N5/AC-1、WORK-F20/AC-3：指南/调用/模式/校验入口一致，未遗留当前文档的同源错误 | 对照 `provider.rs::ThemeBoundary`、`data_tools.rs::rust_snippet` 与公开指南；定向 `rg -n -e 'Signal<ThemeValuePatch>' -e 'ResolvedTheme::patched' -e 'mount_exported_preview' -e 'exact snippet' docs/themes.md docs/dom-components-migration.md examples/theme-studio/src/data_tools.rs` 人工逐命中判读；fmt 与 scoped Clippy | M1 同批局部退出；不以关键词零命中代替语义检查 |
| V-3 | G-01/G-02/V-01 的真实导出闭环、存储全数据断言、字体路径、版本/模式/四参调用与 UI 可达 | 下方现有 build/Playwright/独立 DOM 入口；Agent 模拟外部开发者只给新导出的主题/接入说明与公开文档，按所选 mode/base/context 消费原片段，记录卡点 | M2 阶段收口：M1 成果稳定后统一构建和集成；只在此运行完整相关四 spec/真实走查 |

### 9.2 可执行命令与范围

M1 局部：V-1/V-2；`mise exec -- cargo fmt --all -- --check`、`mise exec -- mbx clippy --locked -p theme-studio --all-targets -- -D warnings`。批内仅为具体失败用 V-1 的单个测试过滤器；不每改一段文案就重跑包/浏览器。

同批局部退出时，还须用以下最小编译核对生成器的修订传递，以解锁 M2；所有命令从仓根执行，显式覆盖同名环境变量，不依赖调用者预设值。Studio 的修订仍由原 build.rs 从同一 checkout 读取。

```sh
RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mise exec -- mbx check --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bin theme-export-generator
```

M2 收口：

```sh
mise exec -- mbx xtask build-web --example theme-studio --release
RUSTIFY_XTASK=target/debug/xtask RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_UI_EVIDENCE_OUT=docs/validation/dom-components-p1-gaps mise exec -- npx --no-install playwright test --project=theme-studio theme-config.spec.ts theme-export-ui.spec.ts theme-exchange.spec.ts theme-storage.spec.ts --workers=1
RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mise exec -- sh tests/theme-export/build.sh --dom-only
```

独立构建后运行全部八种 CSS 消费与一个默认 DOM 用例，共同覆盖本增量的静态/SDK 接入：

```sh
RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_THEME_EXPORT_OUT=docs/validation/dom-components-p1-gaps/export mise exec -- npx --no-install playwright test --project=theme-export theme-export-runtime.spec.ts --grep 'default DOM SDK consumes original Rust and JSON|independent .* CSS renders' --workers=1
```

该过滤匹配当前实际测试名，排除 `compiled original Rust snippet mounts JSON in independent light and dark scopes` 的 GPU fixture；不把未重建 GPU 作为本轮产物。独立角色可读配套导出/接入说明和公开消费文档，不能依靠内部源码补缺失步骤；收口记录真实命令、结果、mode/context/base 与字体资源，并核对 Studio 修订、下载资料修订及 fixture 编译输入修订一致。记录构建时的 HEAD/dirty 基线，不把 HEAD header 当作未提交内容的唯一身份。

真实页面再核默认 JSON/兼容 v1/CSS/Rust 的说明、实际 `theme-setup.txt` 下载字节和字体/许可路径；保持作者文档与 context 不变，仅切换 Light/Dark 预览后重新打开导出并下载，核对屏内/资料的四参调用随模式变化，完整 JSON 字节和作者历史不变；英文/中文、390/768/桌面/200%，console/失败请求/CSP。核对新 JSON/截图均落入增量目录，历史证据无本次覆盖。修改 class 字符串时按 AGENTS 同步 CSS 并 `mbx xtask css --check`，没有 class 变动则复用已有 SDK 生成物。

不跑全工作区、全部 GPU/Docs 或完整性能：本增量不改其契约。若独立消费暴露原范围内真实缺陷，保留失败并说明，按影响复核，不能为了退出而降低原数据断言。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 接入资料与当前 API 说明补齐 | 既有 export_json/resolve/download 契约、§4 的显式修订注入路线及 G-01/G-02/V-01 已核实 | 单人连续完成 §5 的资料/通用提示/四参调用/指南和三旧断言同步，按 §1.2 既有路径修改，integration_notes 显式接收当前 preview_mode；复用原导出组件，不新建通用资料模块 | V-1、V-2 与 §9.2 生成器最小编译；回写「实施进度」 |
| M2 | 独立消费与浏览器阶段收口 | M1 成果、V-1/V-2 及显式注入修订的生成器最小编译通过 | 最新 Studio release；按 §9.2 显式注入修订重建独立 DOM，V-3 的四 spec、配套说明与独立消费，同一浏览器串行走查，证据输出到增量目录 | V-3，G-01/G-02 均有关闭证据、V-01 原失败测试运行通过；回写「实施进度」 |

提交点：M2 全部完成后最终提交一次，再按实施进度协议回填 SHA。共享 `data_tools.rs`/说明/测试由同一实现者维护；工作量小，不拆并行 agent 或重复环境。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 生成器缺少编译期修订输入 | 裸 build.sh 无法重建独立消费者 | 实施者采用 §9.2 显式注入，M1 验最小编译，M2 从未预设变量的 shell 完整重建并核同源身份 | M1 局部退出 / M2 消费前 | 否，路线已核实；实施失败时阻塞相应退出 |
| 把纯 CSS 消费说成必须启动 SDK/CLI | 扭曲已有静态能力 | 实施者按 §5.1 分格式写适用条件，V-1/V-3 核对 | M1 | 否，方案已定 |
| 只更新文案、不运行原数据断言 | 验证缺口持续 | 实施者完成 V-3 四 spec，保留失败/重试/库断言 | M2 | 否，验收已定义 |
| 历史 U-01、VoiceOver/OS IME/真机未验 | 超出本增量的结论边界 | 保留原报告与 P5 去向，不虚报关闭 | 不属于本计划退出 | 否 |

最终状态：**Ready**。无未决产品 C、外部接口或格式/数据迁移前置；当前模式显式输入、生成器修订注入与证据隔离入口已明确。功能及完整消费者重建仍由 M1/M2 验收，普通计划评审由用户发起，不作为额外开工阻塞。

## 12. 已知坑与历史约束

- 原报告 V-01 的三条文案断言失败不能改记通过；必须在最终 release 真正跑到数据断言与重试结果。
- 原 M7/配对说明包含历史源码 SHA，不能把旧 source CLI 当新源码重新构建；资料的修订取实际 build.rs 值。
- 默认完整 JSON 是作者数据，不承载接入说明元数据；字体 URL/许可可携带但字体文件仍外置，见原 PRD/WORK-F20 和 AGENTS。
- 浏览器 spec 单 project 串行；Studio 导出用 `RUSTIFY_UI_EVIDENCE_OUT`，独立消费用 `RUSTIFY_THEME_EXPORT_OUT`，按 §9.2 分别指定增量目录，前者不能替代后者。边界测试另有 `RUSTIFY_THEME_BOUNDARY_OUT` 参数，使用它时避免覆盖原截图。
- 原 PRD、计划、完成记录和旧评审不修改；新里程碑证据留本计划 records 与独立验证目录。
