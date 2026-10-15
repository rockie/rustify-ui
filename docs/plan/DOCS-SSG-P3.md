# DOCS-SSG-P3 · Rustify UI · 多语言路由、缺译回退与检查（DOM · 构建期静态生成）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-15 +08:00 · `3803148929ebb2f69d7549c9f054ff81eb9a6d20` · dirty；未提交项为 P3/P4 计划、P3 文档评审与相关 SDLC 登记，产品源码和原 PRD 与 HEAD 一致；事实基于当前工作树核对。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P3（第 3 期）**，以 §7.1 出口、§7.2 矩阵为准。计划版本：v2。
>
> 本期交付：**通用文档站按语言生成静态页面、切换同页译文、缺译回退、语言化 404、互指与缺失报告**。
> **开发范围：本仓 rustify-docs / CLI / 注册语法及宏、文档站 fixture、作者文档与检查入口；仅 DOM 静态生成及既有 DOM 岛的语言上下文，依据 PRD P3**。
> 本期独特职责：维护者配置语言，读者在目标语言界面阅读真实译文或有标记的默认语言内容。
> **顶层排除：搜索、AI 索引、官方站中文内容与公开发布由 P4 负责；不建设翻译服务，不翻译全部指南，不接入 Portal，不增加 GPU**。

## 实施者定位

执行本计划的 agent 是**资深软件工程师**，依据本计划、仓库规则和当前代码实施。

- **开发边界**：仅实施本计划明确纳入的系统与改动；范围外依赖按 §4 的契约、提供方与解除条件跟进，不代建、改造或部署。源码可访问、自有服务或同仓目录不扩大开发范围。外部缺口阻塞时回写影响与继续条件，推进不依赖它的已授权工作；就地修订偏差和委派不扩大范围，新增开发对象须有用户明确依据。fixture 只覆盖约定契约的必要行为，不能成为替代外部系统的产品，也不能冒充真实集成验收。
- **表达与源码**：报告用路径、需求/检查/里程碑 ID 定位，不复述计划。源码不放需求编号、agent 标记或规划元数据，注释解释必要原因；追溯放记录或提交说明。
- **编号与任务**：需求/约束/假设 ID 只在需求账本登记一次，上游 ID 保留原编号及来源；检查、里程碑、任务 ID 分别由验证表、里程碑表、按需内部任务表定义，正文、NFR 展开、映射和进度记录只引用。修订保留已有 ID，删除/拆分时同步引用；任务表若存在，按交付物、前置成果与检查 ID 推进，任务进展及证据写入所属里程碑记录，不另设任务状态表。
- **完成的定义**：本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口，不冒充通过。集中到阶段收口的终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收。
- **角色验收**：上手与业务场景由 Agent 按 §9 指定的知识、资料及权限约束模拟，不能借内部源码或实现者补步骤；记录实际结果与卡点，不把模拟结果写成真人数据。有工具和权限的真实环境检查由 Agent 执行；真人公测指标按其指定阶段另验，不冒充本阶段已通过。
- **按计划路线实现**：计划已定的技术路线、接口、复用锚点和检查命令直接照做，不重新调研，也不为已定决策另起方案设计、比较或评判；计划没写的实现细节按最近的同构代码就地决定。调查只读本里程碑的改动落点及其直接调用方和依赖。只有代码事实推翻计划前提或路线走不通时才停下，就地修订对应正文并在记录写明偏差。
- **工作顺序**：需要行为测试的改动先红、再绿、再重构。按实际前置成果及检查推进，不等无关终验，数据、安全、发布及用户分期门不得绕过；稳定契约下可用 fixture 开发，不能替代真实集成。
- **验证节奏**：开发中和里程碑退出只跑 §9 中排在本里程碑的局部检查，即新增或改动行为的测试、直接受影响模块的已有测试和快速静态检查。全量测试、端到端、真实环境与界面走查、故障注入、评测等收口检查只在 §9 指定的阶段收口跑一次，同环境走查合并，不在中间里程碑或小改之后提前跑。已通过的证据只在它实际覆盖的代码、契约、配置、依赖或数据被改动时失效，失效就重跑那几项，不扩成全量。收口失败修复后重跑失败项和修复波及的局部检查；修复改动共享契约、公共入口、构建或依赖时，再重跑受影响的收口检查。调优、容量这类要反复尝试的工作，先用几分钟内能复测的缩小版本迭代，达标后再跑完整检查。
- **并行交付**：独立任务在收益超过委派/集成成本且工具可用时使用 subagents，模型默认继承。任务说明只给目标/ID、稳定契约、可写与禁改路径、运行资源和检查；共享文件单人负责，数据库、端口、浏览器等隔离或串行。子 agent 返回结果、路径、验证证据与缺口，默认不另写任务文档；主 agent 审查并集成验收。边界变化先回报，失败只阻塞依赖任务；结束对话前收集或停止仍在运行的任务。

## 实施进度（实施期持续更新）

本节是本计划跨对话恢复的**唯一汇总入口**，协议文字不随实施改写，实施期只更新「恢复快照」和「完成记录」。续做时先读仓库规则、本节、当前任务相关正文与依赖，再核对 `git status` 和相关 diff；记录与工作树不符时先查明原因，不凭记录覆盖用户改动，也不凭代码存在推断验收已通过。只有继续未完任务、核验前置验收、排错或审查时，才按链接读取对应记录的相关部分；不要默认加载全部记录或原始日志。

**回写时机**：里程碑完成后立即回写，再报告完成或提交；下游只需部分成果时，所需前置检查通过并在记录保存证据即可推进。实质进展后受阻、暂停、交接、发现偏差或结束对话时也保存成果与缺口；普通子任务切换不单独回写。

**一次回写**：主 agent 更新对应记录，再同步快照变化字段和完成记录一行，不重写无变化内容或追加流水。快照写当前任务、进展与下一步，命令及证据只进记录。首次回写删占位行，每个已记进展的里程碑恰好一行，状态仅「进行中 / 阻塞 / 已完成」；完成时移至已完成行末，最近完成取该行。更新时间带时区，摘要 1–2 句，记录填相对计划的链接（如 `[M1 记录](FEATURE.records/M1.md)`）。范围、决策、接口、数据、风险或退出条件有偏差时，就地修订对应正文，不追加勘误历史。

**独立记录**：按需建 `<计划名>.records/M<n>.md`，头部写计划链接、更新时区、状态及验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）。正文只写交付行为/路径、必要偏差、未完项与继续动作；验收按 `检查 ID/覆盖范围 | 命令或步骤 | 结果/必要证据` 记录，共同环境和基线在头部写一次，有差异再单列。同一测试组可覆盖多条退出条件，须逐条可追溯；引用原证据与复用依据，不复制断言全文、成功日志或调试流水。证据是命令、结果和必要的少量输出，不为留证另写一次性脚本、哈希清单或大体量证据目录。未运行、失败、环境缺失明确区分；只有复杂交接、排错或证据需独立保存时才另建任务记录/日志/截图并链接。

**项目跟进索引**：仓库已登记状态台账，或存在仓根 `.xgent-ai/sdlc/protocol.md` 时（兼容已有 `docs/project-status/protocol.md`；两套并存未切换时先核对，不双写），上述回写完成后读取台账协议，只同步实际改变的当前状态和来源链接，必要时更新索引；默认直接更新本计划状态页，不因普通进展或来源版本变化另建事件，独立留存按仓内协议；未启用则跳过。复用已有计划 ID，只更新本次触及的范围，保留其他计划和各轮评审，不复制本节及里程碑记录；台账仅作项目索引，不替代本节的恢复快照。跨机器使用仓根相对路径与明确基线，未提交成果标明同步限制；登记失败说明缺口，不将原成果回退。

**提交**：在里程碑表后的「提交点」提交：里程碑较多时在标出的阶段收口后提交，全部里程碑完成后必须提交一次；仓库规则或用户对分支、提交信息、签名或是否由 agent 提交另有要求时从其要求。提交前先回写本次进度；提交点要求的检查就是本次覆盖的里程碑或阶段收口的检查，已通过且未失效的证据直接引用，不为提交重跑；只暂存本计划相关改动，不纳入、不回退他人改动，不跳过提交钩子。提交后把短 SHA 写入完成记录中本次覆盖的各里程碑行的「提交」列（代码评审修复的提交写作「修复」加短 SHA，UI 验收修复的提交写作「UI修复」加短 SHA，均追加在原 SHA 之后），再单独提交这次只改计划文件的更新，该提交不回填自己的 SHA。已完成但未到提交点的行写「待提交」；无法提交时写明原因并记入当前阻塞，不伪造 SHA。

**全部完成**：最后一个里程碑完成后，按上条完成最终提交并回写 SHA，确认每个已完成行的「提交」都有 SHA；恢复快照的「下一步」写代码评审。随后向用户报告完成，并提示下一步用 review-code 以本计划路径为参数评审本计划：它从完成记录找到要审的提交，并纳入未提交改动。评审由用户发起，实施中不自行启动评审或修复评审意见。

**回写后自查**：每次回写后直接核对以下条目：「当前进度 n/N」的 N 等于里程碑总数，n 只计状态为「已完成」的行；「最近完成」对应最后一条已完成记录，不按编号推算；链接指向已落盘文件；已完成行的「提交」是短 SHA 或「待提交」，全部完成时每行都有 SHA 且没有「待提交」。上述结构、计数与链接检查不证明验收真实通过。

### 恢复快照

- 最近更新：2026-10-15T23:41:07+08:00
- 当前进度：4/5 个里程碑完成
- 当前状态：M1–M4 已完成；M5 本地交付及 V-4、V-7–V-9 通过，V-10 出口资源未齐
- 最近完成：M4 · SEO、404 与缺失检查闭环
- 下一步：补 V-10 指定稳定浏览器、真机与 VoiceOver；本地交付可由用户以本计划发起 review-code
- 当前阻塞：C-5 / V-10 完整矩阵缺证；没有稳定 Firefox App、两种真机与 VoiceOver资源，Safari 完整原生200%视觉补证也未齐
- 代码基线：库 R₂ `820cea0f0ea886606427bfd031e17c93d401aca8`；CLI C₂ `bae5b882d9304a47254d81662b5163981bb0adaf`（0.1.21）；证据 `c64f133a`（本地提交，未推送）

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M5 | 阻塞 | 2026-10-15T23:34:52+08:00 | C₂实装与固定配对、本地343/179测试、49/49浏览器及features/artifacts/toolchain通过；V-10资源未齐，不记整期完成。 | [M5 记录](DOCS-SSG-P3.records/M5.md) | R `08eb0d32`、C `53c0be03`、UI修复 `820cea0f`、C `bae5b882`、证据 `c64f133a` |
| M1 | 已完成 | 2026-10-15T23:34:52+08:00 | 配置/目录/注册语言元数据与真实/回退图已实现；V-1/V-2和内容图回归通过。 | [M1 记录](DOCS-SSG-P3.records/M1.md) | R `08eb0d32`、证据 `c64f133a` |
| M2 | 已完成 | 2026-10-15T23:34:52+08:00 | 语言内链接/导航、目录输入/收据、缓存与Markdown文案已接入；V-3和直接回归通过。 | [M2 记录](DOCS-SSG-P3.records/M2.md) | R `08eb0d32`、证据 `c64f133a` |
| M3 | 已完成 | 2026-10-15T23:34:52+08:00 | 新R/C的真实三语言helper/slot/demo及metadata通过；SDK与阅读壳契约、本地浏览器均补验。 | [M3 记录](DOCS-SSG-P3.records/M3.md) | R `08eb0d32`、C `53c0be03`、UI修复 `820cea0f`、C `bae5b882`、证据 `c64f133a` |
| M4 | 已完成 | 2026-10-15T23:34:52+08:00 | V-5/V-6通过；SEO/404、独立dist、首次语言壳与dev HTTP/SSE恢复闭环。 | [M4 记录](DOCS-SSG-P3.records/M4.md) | R `08eb0d32`、C `53c0be03`、UI修复 `820cea0f`、C `bae5b882`、证据 `c64f133a` |


## 0. 需求、范围与决策

### 0.1 需求与约束账本

AC 原文见来源 PRD §3；本表只登记 P3 增量和必要回归。WORK-F10 本期终验；WORK-F11、WORK-F15 要连同前期 AC 核对后才能终验。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F10 | 功能 | PRD §3、§7.2，AC-1–4 | 同页语言切换、缺译回退、自定义文案及两类托管的语言化 404 | §2–§6；V-1–V-8、V-10 | 已确认；P3 终验 |
| WORK-F11 | 功能 | PRD §7.2，AC-4；AC-1–3 继承 P1 | 真实译文双向互指，回退页不入互指或站点地图 | §3、§5.3；V-5、V-7、V-8 | 已确认；P3 终验 |
| WORK-F15 | 功能 | PRD §7.2，AC-4；AC-1/2/5 继承 P1、AC-3 继承 P2 | 缺译及缺文案逐项诊断；警告提升为错误、离线与既有错误诊断回归 | §5.4；V-6、V-8、V-9 | 已确认；P3 终验 |
| WORK-N3 | 兼容 | PRD AC-2/4 每期复核，D16/Q3 | 严格 CSP、无重写的子路径/404，按实际浏览器与设备分别记证 | §4、§6、§9.2；V-7、V-10 | 本期复核；整条终验 P4 |
| WORK-N4 | 工具链 | PRD 全部 AC 每期复核 | 内容站不编译 Rust，无 Makepad，不增加作者工具链 | §2、§4；V-8、V-9 | 已确认；本期复核 |
| WORK-N5 | 可访问性 | PRD AC-1/3/5/6 新交互交付期复核 | 文案逐项标语言；键盘切换、双主题、窄屏、缩放与减少动效 | §6；V-7、V-10 | 本期复核；整条终验 P4 |
| WORK-N6 | 可靠性 | PRD AC-1–3，既有流水线回归 | 同输入同产物；语言文件变化纳入收据，错误不污染 last-good/dist | §3、§5、§8；V-3、V-8、V-9 | 本期回归 |
| WORK-N7 | 安全 | PRD AC-1–4，新增文案文件与派生页 | 文案/配置为输入，不能作为静态资源泄露；保留最终 HTML 检查 | §3、§5；V-3、V-8、V-9 | 本期回归 |
| C-1 | 前置/范围 | PRD §7.1、D15/D16；P2 恢复快照与本人验收记录 | P2 本地能力及本人验收可用；历史整期 5/7、设备/性能/公开发布缺证仍归原计划 | §4、§11；V-9 | 本地开发前置足够，不宣称整 P2 完成 |
| C-2 | 工具/交付 | `mise.toml`、`CLAUDE.md` Toolchain；CLI `new.rs::framework_revision` | Rust 1.99.0、mbx；Docs/CLI 分包检查；保持固定库 R → CLI C 与锁文件配对 | §3.2、§9；V-8、V-9 | 已核实 |
| C-3 | UI/路径 | `AGENTS.md` 项目定位与前端验收；`DESIGN.docs-ssg.md` | 仅 DOM；沿用阅读主题；真实浏览器检查新交互 | §6；V-7、V-10 | 已确认 |
| C-4 | 产品边界 | PRD D13、DEP-7、非目标 RTL | 回退正文有源语言；搜索和 AI 归属交给 P4；第三语言 SDK 文案回退英文 | §2.3、§4；V-2、V-4 | 已确认 |
| C-5 | 验收资源 | 用户 2026-10-15 确认当前没有真机和 VoiceOver 资源；PRD D16 | 保留发布前补证，不把设备或辅助技术缺证当成本地开发前置 | §4、§9.2、§11；V-10 | 已确认 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 呈现 | 构建期生成每语言完整 HTML，普通链接切换 | 无全站 CSR 或新运行服务；内容站零 wasm | PRD D2；`generation.rs::generate_extended` |
| D2 | 翻译关联 | 默认语言是逻辑页面全集；内容按配置前缀配对，注册页以相同逻辑 route 和 lang 配对 | 不增加 translation_key；不支持不同语言各自改 slug；源文件路径仍用于引用解析 | ADR-1；§2.1 |
| D3 | 默认地址 | `site.lang` 为默认语言，继续占部署基路径根部；其他语言各占一个前缀 | 既有地址不迁移；省略 i18n 配置时旧单语言行为不变 | PRD WORK-F10、Q6 |
| D4 | 文案 | 内置 en/zh 加项目级 TOML 覆盖，逐项记录文字的真实语言 | 英文回退不能把整个第三语言界面标成英文；不增加热更新服务 | §2.2；已有 `labels.rs` |
| D5 | 404 | 每语言物理错误页；前缀到错误文件的映射随 manifest 冻结，preview/dev 消费同代数据；根错误页含各语言静态壳 | preview 可脱离源码/配置；单错误页托管由外置脚本选择，禁脚本仍有默认语言及各语言首页；不重定向，不伪造 200 | §2.4、§5.2；`publish.rs`、`preview.rs` 与 `static-host.mjs` |
| D6 | 开发与终验 | 本地开发沿 D16 继续；真实设备、稳定浏览器缺证保留在出口检查 | 不用 P1 或 P2 例外豁免 P3；不因暂缺设备阻止独立本地工作 | PRD WORK-N3、D16；§11 |

### 0.3 ADR-lite：语言图作为地址权威

- 状态：Accepted（技术方案）。
- 驱动：WORK-F10/F11/F15 要共用翻译存在性、回退、规范地址与缺失报告；P4 也必须复用同一归属。
- 候选 A：每语言分别构建并在配置中手填跨语言链接。改动局部、容易复用单语言引擎；代价是资源重复、无法统一判断缺译/草稿，必须另合并 sitemap 和未来索引。
- 候选 B：一次输入扫描生成统一页面图，再按语言投影导航和输出。需要修改内容分析与 helper 分批，但检查、资源和输出事务仍只有一份。
- 决策：B。翻译关联与派生页集中在 `content.rs`，`generation.rs` 消费结果，不让 shell、CLI、P4 各自猜测语言。
- 后果：页面数量随语言数增加，输出和本机渲染成本增加；无新基础设施。相同源片段可共享输入字节，不能共享语言不同的呈现缓存。
- 重评条件：要求翻译各自拥有不同地址或独立发布生命周期时，另行确认产品地址合同；不预建映射插件。

### 0.4 职责与事实所有权

|  | 维护者/内容仓库 | 本期框架与 CLI | 静态托管 |
| --- | --- | --- | --- |
| 拥有 | 默认语言、语言列表、真实译文、文案覆盖与导航配置 | 关联、回退判定、诊断、整代事务发布与语言 UI | 实际 HTTP 状态、目录索引、错误页选择和 MIME |
| 不拥有 | 自动生成的回退页 | **不机器翻译、不建设托管、不改变 SDK 支持的语言集合** | 翻译关联、文案或搜索范围 |

构建图拥有回退事实，托管只选择已生成的文件；任何错误都先拒绝候选构建并保留上一成功版本。

### 0.5 明确不在本期

- 搜索、llms.txt、官方站中文首页/quickstart 与公开 Git/Pages/回滚：由 [P4 计划](DOCS-SSG-P4.md)交付；P3 只冻结其语言图输入。
- P2 未完的原期验收/发布与设备、正式性能、GitHub 预览缺证：保留原计划和证据；本期不改它们的完成计数。
- RTL、机器翻译、翻译管理系统、全部指南翻译、多版本、Portal/onebox 与 GPU：不开发；无需服务或容器。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- [已核实·足够] `content.rs::snapshot/analyze_with_surfaces` 统一读输入、解析引用和诊断；`generation.rs` 有最终文档检查，`publish.rs` 有既有输出事务，可扩展而无需重写。
- [已核实·缺口] `Config.site.lang` 是一个字符串；`route_for` 只按文件路径生成地址，`Analysis.nav` 是单份 → 增加语言声明、逻辑页面关联和按语言导航 → 否则无法同页切换或区分回退。
- [已核实·缺口] `labels.rs::for_lang` 只选内置静态 en/zh，第三语言整体回英文 → 加项目文案输入和逐项语言来源 → 否则 AC-3 与“精确缺两条”无法实现。
- [已核实·缺口] `rendering.rs::build_cached` 的所有 Rust 请求共用 `config.site.lang`；`registry.rs::prepare_environment` 拒绝第三语言 → 按目标界面语言分批，未知 SDK locale 回英文 → 否则同一站里的中文/第三语言岛状态不正确。
- [已核实·缺口] `generation.rs::sitemap` 仅按 draft/index 过滤；只生成根 `404.html`，无 hreflang → 消费统一语言图并补每语言错误页。
- [已核实·缺口] `publish.rs::Manifest` 未声明语言映射，文件列表也不标产物类型；`main.rs::preview --dir` 不读工程配置 → 增加随产物冻结的语言错误文件映射，不能从普通资源目录猜语言。
- [已核实·足够] P2 的“本人验收通过”记录有用户直接确认；P2 主计划保持 5/7，最新增量计划 2/2 及独立复审仅证明其修复范围，不替代整期验收。

### 1.2 文件清单

以下均属于本仓；★ 为新增。同一文件不并行写。

| 落点 | 改动与同构实现 |
| --- | --- |
| `crates/rustify-docs/src/config.rs`、`content.rs`、`labels.rs` | 沿 Raw/Config 严格解析、InputReceipts、Page/Analysis、navigation 和 EN/ZH 增量实现 §2/§5 |
| `crates/rustify-docs/src/markdown.rs`、`builtins.rs`、`demo.rs`、`shell.rs` | 把界面文案消费者接到目录；区分界面与正文语言；复用现有 HTML writer、原生 details、复制/演示状态 |
| `crates/rustify-docs/src/generation.rs`、`publish.rs`、`html_check.rs`、`classes.rs`、`crates/rustify-docs/assets/mode.js`、`crates/rustify-docs/assets/site.js`、`crates/rustify-docs/assets/dev.js`、`crates/rustify-docs/assets/docs.css` | 每语言 shell/SEO/404 及 manifest 映射；预留文件保护与最终链接检查；错误页选择先于基础交互和水合；样式沿既有类/token |
| `crates/rustify-docs-syntax/src/lib.rs`、`manifest.rs`、`crates/rustify-docs-macros/src/expand.rs` | 注册页增加可选 lang；沿现有 title/description 的三端一致解析/导出 |
| `crates/rustify-docs/src/site/registry.rs`、`crates/rustify-docs-cli/src/rendering.rs`、`main.rs`、`dev.rs`、`preview.rs` | 每语言 helper/岛元数据、文案变化观察；build 写入、preview 读取产物映射，dev 保留同代错误页；复用现有批量 render/last-good |
| `crates/rustify-docs-cli/src/new.rs`、两种模板的 README、CLI Cargo/锁与配对资产 | 描述新配置，旧模板保持单语言可用；按 §3.2 固定 R/C |
| `crates/rustify-docs/tests/i18n.rs` ★、`crates/rustify-docs-cli/tests/i18n.rs` ★ | 公开生成/CLI 行为；沿 `generate.rs`、CLI `check.rs`/`dev.rs` fixture 和断言方式 |
| `tests/docs-ssg/fixtures/i18n/` ★、`tests/docs-ssg/i18n.spec.ts` ★、`tests/docs-ssg/i18n-islands.spec.ts` ★、`dev-i18n.spec.ts` ★、`serve.sh`、`playwright.docs.config.ts` | 内容站 fixture 与 components 临时副本的语言叠加件；内容站、真实 Rust 消费者各有根/子路径/meta 项目，dev 复用同一 Rust 叠加件，见 §9.2 |
| `docs/docs-ssg.md`、`DESIGN.docs-ssg.md`、现有文档站 surface brief | 配置/迁移/缺失说明与验收后的新交互规范 |
| `xtask/src/docs.rs`、`crates/rustify-docs-cli/tests/artifacts.rs`、`crates/rustify-docs-cli/tests/new_build.rs` | 现有 artifacts/toolchain 组纳入新语言 fixture 的公开生成/离线入口，命令见 V-9 |

SDK `Locale` 的两语言集合、Makepad、七个 GPU 示例均无需修改。注册语言选择/回退只在文档层适配。

## 2. 模块、接口与不变量

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| config / labels | 内容分析、shell、CLI | 见 §2.1/§2.2；配置错误定位到原文；目录在一次构建内不可变 | 无新 adapter | 前缀校验、文案逐项回退与缺失清单 | 进程内/真实本地文件 | parse、公开 check、HTML |
| content | CLI、generator、P4 | `Analysis` 拥有唯一语言图；有效 route 全局唯一；链接解析保留源文件位置 | 现有 Snapshot | 默认页全集、回退、导航与引用关系 | 进程内 | 生成/检查接口 |
| rendering / registry | CLI/站点 helper | 请求按 ui_lang 分批；每条岛 metadata 的环境与 SSR 一致 | 现有 helper 协议 | 语言作用域、源页/目标壳、seed 与批次结果关联 | 本地子进程，真实 helper | 独立 SSR/hydrate、CLI |
| shell / generation | build、dev、preview | 已解析的文案、导航和页面身份；不再从 URL 猜真实译文 | 无 | 语义语言、SEO、404、文件保护 | 进程内/真实静态托管 | HTML、HTTP、浏览器 |
| publish / preview / dev | build、产物预览、开发请求 | §2.4 的冻结映射；文件与映射同代，preview 不读工程语言配置 | 既有 manifest / Good | 校验、旧产物兼容、错误文件选择与重载 | 进程内/真实产物目录 | manifest 往返、脱离工程的 HTTP、last-good |

保留原模块边界。若删除语言图会让四个消费者各自实现回退，因此它属于现有 content 深模块；不另造通用 i18n 服务或 Provider 层。

### 2.1 配置、内容目录与注册页

新增配置均是 schema 1 的可选字段，旧 CLI 对未知字段报错，因此新配置只与新配对 CLI/库一起使用。

```toml
[site]
name = "Example"
lang = "en"
base = "/docs/"

[[i18n.locales]]
lang = "en"
label = "English"
prefix = ""

[[i18n.locales]]
lang = "zh"
label = "中文"
prefix = "zh"
messages = "locales/zh.toml"

[i18n.locales.nav]
# 可选：完整覆盖该语言的 groups/top/social；字段与现有 nav 相同。
# items 仍引用默认语言的内容相对路径；routes 仍使用无语言前缀的逻辑地址。
groups = [{ label = "指南", items = ["guide/start.md"] }]
```

- `site.lang` 恰好对应一项，且只有该项的 prefix 为空；无 i18n 时按旧行为处理。lang 按有效 tag 校验、大小写无关去重，保留声明拼写写入 HTML；prefix 为唯一单段安全路径，不许空白、点段、保留目录/站点文件或多段路径。所有语言沿既有 LTR 布局；文档明确不支持 RTL，不增加语言方向推断表或方向配置。
- 默认源 `guide/start.md`，译文源 `zh/guide/start.md`。`content.include` 显式模式仍只选列出的真实文件，维护者需要加入译文路径；不自动越过 allowlist。前缀只在启用该语言配置后才有配对语义。按前缀剥离后由现有 `route_for` 得到逻辑地址 `/guide/start/`。
- 单语言模式保留既有 frontmatter.lang 行为。多语言模式下 frontmatter.lang 若提供，必须与所在语言声明一致，避免路径与语言冲突。配置的其他语言前缀在默认语言空间保留。
- `site!` 页元数据增加可选 `lang: "zh"`，缺省为默认语言；`route` 是不带部署 base/语言前缀的逻辑地址。名称仍全局唯一，同一逻辑地址可按不同语言注册不同函数；Markdown 与注册页不能占用同一 `(lang, logical_route)`。
- 对应类型增量为 `PageDeclaration.lang: Option<LitStr>`、`PageManifest.lang: Option<String>` 和 `RegisteredPage.lang: Option<String>`；PageManifest 反序列化缺省 None。真实声明列表保留为真实版本，不把渲染出的回退副本注册成译文。
- 只有默认语言页面可建立新的逻辑页；没有默认来源的译文/注册页报 `i18n-orphan`，不猜关联。相同逻辑页允许 Markdown 与注册页互为译文。
- 地图中默认页为 draft 时，生产模式整个逻辑页不发布；仅译文 draft 时，生产模式发布默认正文的目标语言回退页，不能泄露草稿正文。dev 可以展示草稿，但生产搜索/AI 永远不消费 dev 数据。

核心数据结构增量：

```rust
pub struct LocaleConfig {
    pub lang: String,
    pub label: String,
    pub prefix: String,
    pub messages: Option<(String, std::ops::Range<usize>)>,
    pub nav: Option<Nav>,
}
// Config 增加 locales: Vec<LocaleConfig>；单语言模式为空。
pub struct PageLocale {
    pub logical_route: String,
    pub ui_lang: String,
    pub source_lang: String,
    pub source_route: String, // 真实内容版本的最终 route，无部署 base
    pub is_fallback: bool,
}
// Page 增加 locale: PageLocale；原 lang 始终等于 ui_lang。
// Analysis 增加 locales: BTreeMap<String, LocaleView>。
pub struct LocaleView {
    pub root_route: String,
    pub pages: BTreeMap<String, usize>, // logical_route -> Analysis.pages 下标
    pub nav: Vec<NavGroup>,
    pub top: Vec<Target>,
    pub social: Vec<Target>,
}
```

`Page.route` 继续是无部署 base 的最终 route，例如 `/zh/guide/start/`；`Page.file/path` 始终指真实内容来源，回退页不能虚构一个不存在的译文文件。保留 `Analysis.routes` 最终地址索引；旧 `Analysis.nav/top/social` 对应默认语言。source_route 在真实译文指向自身，在缺译投影指向默认语言版本。P4 只消费这些已解析字段，不再推断来源。

`Surfaces` 增加 `rendered_pages: BTreeMap<String, String>`（最终 route → 对应语种的已渲染正文），原 pages 仍只保存真实注册声明。先以真实声明建立含回退的图，render_surfaces 遍历图中 Registered 来源页面按目标环境渲染，再把 HTML 放入此映射并重新分析引用。重新分析不能把渲染覆盖映射解释为新增真实译文；即使同名函数在两语种执行，is_fallback 仍由原声明图决定。

### 2.2 文案目录

`labels.rs` 保留 EN/ZH 文案及 `for_lang` 的既有兼容入口，增加项目级 `Catalog` 和 `LabelText { text: String, lang: String }`。`Catalog::get(key: &str) -> &LabelText` 的键集合固定来自内置英文目录，包含现有根键、`demo.*`、`dev.*`，增加 `language`、`untranslated`、`available_languages`；不允许把未知键静默显示在 UI。`resolve(lang: &str, overrides: &BTreeMap<String, String>) -> Catalog` 接收已通过键/类型/占位符校验的数据；`Analysis.catalogs` 保存 `BTreeMap<String, Arc<Catalog>>`，供这一代全部消费者借用。无 i18n 配置时保留原第三语言整套英文行为，不新增缺键警告。

- messages 使用同名 TOML 键和嵌套表，全部值为普通字符串；绝不作为 HTML。zh/en 对应内置目录，可只覆盖部分键。其他语言的已提供键标其声明语言，缺键回英文并标 `lang="en"`；每个缺键仅产生一个警告。SDK 自身的缺文案不报此警告。
- `resolve(lang, overrides)` 每轮每语言生成一份不可变目录，缓存到构建上下文；不在每页重复读文件。目录列举/序列化沿内置 Labels 的字段生成，不维护另一份英文字符串副本。未知键、非字符串、控制字符和缺少/增加格式占位符报配置错误；仅显示字符串的正常换行合法。
- Snapshot 增加独立的 `messages: BTreeMap<String, Arc<str>>` 输入集合，沿 `InputReceipts` 记录实际读取及符号链接目标；不放入可发布 `files/resources`。目录路径必须在工程内且非私密路径，消息变化由 dev watch 触发。即使文案文件位于 content/assets 的扫描范围或被显式作为资源引用，仍按现有主题输入的拒绝方式报错，不发布源文件。
- 所有文案生产者——Markdown 提示框/代码控制、shell、demo wrapper、dev shell/SSE——都使用该目录，避免只翻译顶栏。单条英文回退用该条 `LabelText.lang`，不把整个 article 或 document 改为英文。
- `FragmentKey::of(body, offset, source_lang, ui_lang, labels_digest, includes)` 取代原四参数入口，三个新增参数均为 `&str`；正文源语言、界面语言与目录字节变化都失效对应缓存。保留 include 内容指纹，不能因缓存命中跳过当前收据验证。

shell::Site 增加目标语言首页 `home_route: String` 与 `catalog: Arc<Catalog>`；原 Site.lang/nav/top/social 均指当前语言，Site::url 仍只负责部署 base。shell::Page 增加 `source_lang: &str` 与 `language_links: &[LanguageLink]`，Meta 增加真实版本互指列表。渲染闭包捕获所需文字的 owned 值，不泄漏目录引用为 `'static`；去掉内部消费者对 `&'static Labels` 的要求，不用 Box::leak。

dev.js 现有 data-labels 来自页面本身，继续独立于 SSE 的诊断数据；新增逐键语言属性，按当前目录为 status/按钮分别写 lang，不能用整面板单个英文 lang 覆盖第三语言已提供键。成功重建加载新 data-labels，失败继续使用 last-good 目录；首次无成功代时按请求语言生成 dev shell。

### 2.3 helper 与 SDK locale

`SiteEnvironment` 的 JSON 形状与 `PROTOCOL_VERSION` 保持不变，`lang` 现在取每批目标 ui_lang。`rendering.rs` 把页面/插槽/嵌入请求按此字段分组，每批仍调用一次 `Prepared::render`；不能每个岛启动一个编译或渲染进程。结果按既有 key 归并。

默认注册页缺译时复用其函数、源码和描述，但为目标界面重新 SSR。`RenderContext.route` 继续表示声明中的逻辑 route，`lang` 是 ui_lang，`base` 仍是部署 base；RenderTarget 的 route 也保持逻辑值。这样现有 `site/src/pages.rs::category` 按 `/components/.../` 选择类别的函数在缺译页面不会变空。CLI 请求 key/seed 和结果归属使用最终 route；既有 RenderRequest.key 已能把逻辑 route 相同的不同语言结果分别归并，无需扩 helper JSON。插槽上下文同样给逻辑 route，Surfaces.slots 的 map key 则为最终 route；错误页分别存 `/404/` 与 `/<prefix>/404/`，上下文均给 `/404/`。

`registry.rs::prepare_environment` 使用 `Locale::from_tag(...).unwrap_or(Locale::English)`；无效 tag 仍在配置入口拒绝。SSR 和 hydrate 同时采用该映射，第三语言岛不因 SDK 只有 en/zh 而失败。不扩 SDK locale。单语言模板和旧 helper 元数据仍可用；冷读取和编译清单必须一起认识注册页 lang，错配按既有配对检查拒绝。

### 2.4 产物中的语言错误文件映射

复用 `rustify-docs-manifest.json`，schema_version 仍为 1，增加可选 `locale_not_found: BTreeMap<String, String>`，键是配置的 prefix、值是相对 dist 的错误文件路径。反序列化缺省空表，空表不序列化；旧单语言产物和未启用 i18n 的新产物保持原合同。多语言示例：

```json
{"locale_not_found":{"":"404.html","zh":"zh/404.html","fr":"fr/404.html"}}
```

- **唯一来源/写入方**：`generation::generate_extended` 从已校验语言图生成 `Generation.locale_not_found`，与每语言错误 HTML、根 404 template 的 prefix 属性一起产生；不扫描 `Generation.files` 猜语言。`main.rs::build` 将该表显式交给 `publish::add_manifest` 写入 Manifest，再沿原 publish 事务发布。文件字节仍纳入既有 files/build_id，不改变其摘要算法；不增加另一个站点文件。
- **校验**：非空表必须含 `"" → "404.html"`；其他键沿 §2.1 的单段 prefix 规则，值只能是该键对应的 `<prefix>/404.html`。每个引用必须在 manifest.files 中，且能安全读取为输出根内的普通文件；沿现有路径安全规则拒绝绝对路径、点段、编码绕过与符号链接越界。新字段类型/映射非法、已声明文件缺失或不可读时 preview 启动失败并提示重新构建，不把损坏的多语言产物降为单语言。
- **preview 读取方**：`main.rs::preview` 在持有输出共享锁后读取/校验 manifest 及全部声明的错误文件，再构造 `preview::Site` 的冻结映射和文件字节；每次未命中请求只查内存。按既有请求解码/安全闸剥离 manifest.base，完整匹配首段 prefix，未匹配或在 base 外则用根错误页。两种 preview 入口都只从产物读取语言数据；无 `--dir` 时配置仅用于定位 out_dir。空表只走既有根 404 降级，即使目录里有普通 `foo/404.html` 也不创建语言声明。旧 CLI 不保证新增选择行为，回滚按 §3.2 恢复旧配对及旧产物。
- **dev 读取方**：`Outcome::Built` 将同一 Generation 的文件、base、映射一起交给 `Good`，成功时一次替换；HTTP 404 与 `needs_reload` 都据此定位语言错误文件。新增/移除/改名 prefix 后，成功代同时更新选择和重载判断；失败构建继续使用 last-good 的旧映射与文件，不能借新配置改变旧代响应。首次无成功代的诊断 shell 仍按 §2.2 处理。

## 3. 派生数据与兼容

### 3.1 生命周期

| 对象 | 关键约束 | 来源与生命周期 | 查询 |
| --- | --- | --- | --- |
| 默认逻辑页/真实语言版本 | `(lang, logical_route)` 唯一，最终 route 唯一，默认来源存在 | Markdown allowlist/注册清单；每次分析重新确定 | 内存 map，一次扫描 |
| 缺译投影 | 只复制已发布默认来源；is_fallback=true；源路径真实 | 分析后、引用解析前生成；补译后同地址自动替换 | LocaleView.pages |
| 文案目录 | 已知键，逐条文字语言和缺失状态 | 工程文件为输入；变化更新缓存键和 dev 版本 | 每轮一次读取/每语言一次解析 |
| 语言文件及错误映射 | 根及每前缀 `404.html`、真实译文 sitemap/hreflang；§2.4 的 prefix → 错误文件 | 同一 Generation 生成；manifest 与文件同一事务发布，dev 同代替换；移除语言则下一成功构建删除旧文件和映射 | manifest.locale_not_found / Good 同代表 |

没有数据库或数据回填。移除语言、改 prefix 是公开地址变更；本期不自动生成重定向或维持旧地址，作者文档明确其影响。

### 3.2 固定交付与回退

每期收口沿既有 R/C 流程：先提交库/CLI逻辑/测试为 R，再用独立 C 更新 `crates/rustify-docs-cli/Cargo.toml` 的 patch 版本、根 `Cargo.lock` 的 CLI version、`crates/rustify-docs-cli/assets/framework-revision.txt`、`crates/rustify-docs-cli/assets/components-Cargo.lock` 与 `docs/docs-ssg.md` 的版本/已存在 R 身份。C 除这五个配对文件不夹产品变化；版本读取实施时实际当前值递增，不把本基线的 0.1.19 永久写成下一版本。生成工程沿现有 SOURCE_REVISION 输出实际 C 安装命令；作者参考在 C 之后的文档提交才登记已存在的 C，不伪造自引用哈希。

模板锁由真实 R 的 Git 依赖生成，不手改 source；用 `new --template components`、独立 SSR/hydrate 与 `scripts/docs-pages-prepare.py --project` 检查三份框架依赖都落到 R。开发与库 R 的模板契约测试通过后才生成 C。失败回退为恢复原配对二进制及原静态产物；旧 CLI 不读新 i18n 配置，故回滚时配置/内容/注册代码也取同一旧提交。

从调查基线实施时，HEAD 已含非空的上一期 R，配对脚本会按 CLI 的 Git source 投影旧 R，并非保留 dirty 工作树 path。新增语言 Rust 消费者不能用旧 R 验证；不清空 dirty 资产、不绕过配对。M3 库内局部通过后可先推进不依赖该检查结果的 M4 工作，M5 形成新 R/C 后重跑 V-4 的真实消费者，再核定 M3/M4 退出；该项未通过前不记 M3 完成。内容站与其余局部检查仍按原安排执行。

M5 记录 `DOCS_LOCAL_GIT`（本仓的绝对 file:// Git URL）、`DOCS_C`（已存在的 C 完整 SHA）与 `DOCS_INSTALL`（独立绝对安装目录），再执行本地安装并把浏览器入口绑定到安装二进制：

```sh
mbx install --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C" --locked --profile docs-cli --root "$DOCS_INSTALL" rustify-docs-cli
export RUSTIFY_DOCS_BIN="$DOCS_INSTALL/bin/rustify-docs"
"$RUSTIFY_DOCS_BIN" --build-info
cargo install --list --root "$DOCS_INSTALL"
```

V-9 的 toolchain 组不读取 RUSTIFY_DOCS_BIN，须另用 `mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"`，由组内 plain Cargo 实装同一 C，并以其内部二进制执行受限 PATH、离线及新增语言 walkthrough。不得使用默认合成工作树提交代替交付 C；`CLI source`、`--build-info`、`cargo install --list` 须共同对应 C，模板框架为 R。features/artifacts 和包级测试仍是开发态检查，其来源单独记录。

## 4. 集成与前置

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P2 本地能力与本人验收 | 已核实足够 | 原计划快照、`docs/validation/docs-ssg-p2/M6.md` 本人验收与 D16 节；最新增量记录及复审 | 直接沿当前代码；不等旧期设备/发布全部完成 | 发现真实回归按受影响模块修复，不改历史证据 |
| SDK locale | 已核实缺口 | `Locale::from_tag` 对 en/zh 外返回 None；registry 当前把 None 当错 | 文档层按 §2.3 回英文 | 无效 tag 配置错误；有效第三语言正常生成 |
| 单错误页静态托管 | 已核实足够（合同） | PRD DEP-9/D11；本仓 `static-host.mjs` 根错误页路径 | 通过根静态错误壳和外部脚本适配；真实 Pages 不属于本期发布 | HTTP 404 保持；禁脚本默认语种 |
| preview/dev | 已核实缺口 | preview::Site 仅有一份 not_found；dev 持有 last-good 代 | 按 §2.4 由产物 manifest / Good 读取冻结的 prefix → 错误文件，不读漂移配置或扫资源目录 | 完整首段匹配，否则根错误页；损坏的产物映射拒绝启动，失败开发代保留旧映射 |
| 设备/稳定浏览器 | 已核实缺口（验收资源） | P2 原期验收与 PRD Q3 保留未验证；本机有 Chrome/Safari，无 Firefox 应用 | C-5资源由维护者发布前提供；Agent 执行 V-10；无资源保留缺证 | 不阻塞本地开发，不把模拟计作终验；本期矩阵仍未通过 |
| P4 消费者 | 已定义、尚未实现 | §2.1 的 PageLocale/LocaleView | 只交付稳定语言归属；本期不生成搜索/AI 文件 | P4 开工前核对本期出口记录 |

## 5. 核心流程

### 5.1 页面图与链接

1. 严格解析配置，读取文案，保留原位置/收据；再扫描选中的真实 Markdown 与注册清单。
2. 建 `(lang, logical_route)` 图、验证冲突/孤儿/保留路径；以默认语言的生产可见集生成其余语言的真实版本或缺译投影。
3. 为每个投影按 ui_lang 生成文案控制、按 source_lang 解释正文；资源、include、相对 `.md` 引用始终相对真实源文件解析。源文件到逻辑页的 map 与最终地址 map 分开，不把相同源路径的回退副本互相覆盖。
4. 普通内容相对页链接默认投影到当前 ui_lang；显式带已配置语言前缀的站内地址尊重作者指定语言。资源与仓库外链不加语言前缀。跨页锚点必须存在于实际目标版本，缺失仍报原有链接错误，不能静默删除 fragment。
5. 每语言导航采用 locale.nav 或默认 nav 的结构；items/routes/dir 以默认逻辑页/逻辑路径解析，再投影为目标版本，scope 同样加语言根。sidebar.group 按真实来源语言的导航验证，缺译投影不再拿源组名与目标语言的翻译标签比对；目标覆盖导航的显式列表/目录决定归组，未覆盖时沿默认归组。译文的 sidebar.label/title 优先，回退标题保持源语言并标注来源。侧栏/顶栏/页脚/上一下一页不跨出当前语言。
6. 批量 SSR → 完整文章/插槽引用检查 → 统一生成和最终 HTML 检查 → 既有 publish 事务；任何错误不替换成功产物。文案变化不重编 Rust，只重渲染所影响语言。

### 5.2 语言切换与 404

- 每个页面向 shell 提供 `LanguageLink { lang, label, href, is_fallback }` 列表，href 为同一 logical_route 的目标版本；没有脚本也可点击。单语言不输出入口。
- 保留相同 hash 只在目标确实存在该 ID 时；跨语言标题 ID 不同则去目标页面首部。不猜对应翻译标题。
- 输出根 `404.html` 和每个非默认前缀下的 `404.html`，都使用各自语言导航、文案和首页链接、noindex，无 canonical/hreflang。无 sitemap 项。
- 根错误页的活动主体是默认错误页并列各语首页链接；其他语言的完整 body 内容（含其导航、共享 slot、NotFound 覆盖和岛 metadata）存入各自 inert template，脚本元素不克隆。所有语言需要的外部资源统一加入这一文档的资源集合/样式收集。模板中的 id 不进入活动文档 ID 唯一性判断，但模板本身仍做 CSP/URL 校验。
- `mode.js::prepare` 在现有 interactive 解析边界先处理 404：剥离 `data-rd-base`、按完整首段匹配 prefix、激活相应 template、移除未选模板、更新 html.lang/title、dev脚本的目录属性，然后才扫描活动岛并 resolve 原有 coordinator.parsed。同步 head 输入闸与首帧主题逻辑不延后。site.js/dev.js 在 404 上等待此 parsed 再取 DOM/绑定，islands.js 原已等待此 promise，无需另造就绪事件或改变其 boot 协议。每个 slot 只在活动树出现一次，选中语言的 metadata/seed 保留。任意深度失败地址的资源均带部署 base。
- 根 404 template 的 prefix 属性来自 §2.4 的同一生成表，浏览器不另拉 manifest。preview/dev 按该节的产物/Good 映射选文件，无脚本同样为目标语种；路径安全规则、404 状态与输出共享锁沿用。

### 5.3 SEO 与站点文件

真实版本 canonical 指自身，缺译版本 canonical 指 source_route 的绝对地址；回退页加 noindex。同逻辑页的真实、已发布且允许索引的版本互指，包含自身；不出现缺译投影。只有一个真实版本可只声明自身；无 site.url 时沿用原缺地址警告并省略 canonical/sitemap/hreflang，不生成伪绝对地址。

sitemap 是全站一份，仅列全部真实且可索引的版本；robots 仍指这一份。`SITE_FILES`/保留路径判断需按语言根同时保护 `404.html`，不能只保护默认根。语言选择链接和 hreflang 进入最终链接检查。站点标题冲突检查按最终语言文档题名执行，语言化 404 不与相同文字的异语种误冲突；同语言重名仍按现有策略诊断。

### 5.4 缺失诊断

- `i18n-missing-page`：每个非默认语言、每个缺少生产译文的默认逻辑页恰好一条 warning。定位到该 locale.lang 配置声明（文件/行/列），消息给 logical_route、期望文件路径或注册页 lang，建议创建对应版本。共享默认来源不按每个引用重复报。
- `i18n-missing-label`：仅内置 en/zh 以外的语言，英文键集合减去已提供键，每条一项；定位到 messages 配置声明，或没有文件时 locale.lang 声明，建议包含完整键。文件本身无法解析是 error，不再把全部键另报 missing。
- 配置错误规则：`i18n-config`、`i18n-messages`、`i18n-content-lang`、`i18n-orphan`；现有冲突规则继续使用。所有动态代码片段走现有 `code_span`/`literal`，不截断第三方错误原文。
- 独立计数 fixture 除这两类外零警告：默认四页，第三语言有一页真译文且目录只缺两键，准确缺 3 页、2 文案 = 5 条；不同时声明另一套不完整语言。普通 check 返回成功并报告 warning；`--deny-warnings` 非零；JSON shape/schema 不变；build 使用相同检查并拒绝错误候选，dev 修正后清除浮层。

## 6. 前端与交互

沿用 `DESIGN.docs-ssg.md` 的 Read 表现、系统字体、正文行宽与现有断点；不重做首页、侧栏或演示。

- 顶栏右侧在主题控制旁放原生 `<details>` 语言入口，summary 为当前语言自称，列表为真实 `<a hreflang>`。当前项 aria-current，回退项附目标语言的缺译说明；同页切换用普通跳转。移动端可换行，保持所有入口可触达，点击后收起是增强行为。
- 缺译提示在标题后、正文前使用既有 note 的轻边界与文字，无整屏警报。外层 html.lang 为 ui_lang，正文容器及位于 article 外的来源标题标 source_lang；UI 文案逐项标其实际语言。英文回退标题/导航文字也标源语言。
- 无 loading：语言切换是页面导航；缺译是稳定 partial 状态，不是错误。配置/文案语法错误只出现在 CLI/dev 诊断，不向读者暴露路径。可用译文替换回退后同一地址恢复，无客户端存储迁移。
- 键盘可以展开、Tab 到目标、Enter 切换，焦点可见；Esc 收起时回到 summary。单语言无空菜单。移动宽度 320/375、桌面 200% 原生缩放、双主题与 reduced-motion 都验。
- V-7 在同一次部署验证所有语言/404/主题/CSP/console/失败请求，保留桌面与移动关键截图；按既有 brief 做一次批量检查和至多一次修正后复核。通过后同步实际新规则至文档站设计文档。规划不代表界面已验收。

## 7. NFR 与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N3 | PRD CSP 与当期矩阵 | 两种 CSP 零违规，语言 404 保持真实状态；设备资源待提供 | 缺证不记通过，继续独立开发 | 外置脚本/静态模板；真实设备和模拟分记 | V-7、V-10 |
| WORK-N4 | 现有分特性和内容站入口 | 无新作者依赖，无 Makepad；第三语言不强迫站点 Rust | 框架错配失败，不绕过锁 | 无新 crate/服务，原 feature 分图 | V-8、V-9 |
| WORK-N5 | 现有 Read 主题与 PRD AA | 新控件对比度、焦点、逐项语言、320px/200%/reduce | 缺失译文有明确标记 | 原生链接/details、语义 lang 和既有 token | V-7、V-10 |
| WORK-N6 | InputReceipts/FragmentCache/publish | 文案改动、删除/新增译文可见；失败保留旧产物 | 返回现有错误/last-good | 原始读取收据、目录指纹、整代事务 | V-3、V-8、V-9 |
| WORK-N7 | is_private/final_documents | 文案仅作为文本；不输出其源文件或机器信息 | 输出检查失败 | 输入/可发布资源分开，转义，无 innerHTML 用户文本 | V-8、V-9 |

本期不引入新性能数值承诺；输出随语言线性增加，以实际生成页数和构建耗时记录影响。沿现有 JSON/终端诊断观察，不新增遥测或日志服务。

## 8. 失败模式与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 缺少译文/第三语言键 | 对应页面/控件 | 无源数据丢失 | 默认正文/英文键及明确语言 | warning 明细 | 补文件/键后同地址重建 | V-3、V-6 |
| 前缀冲突、孤儿、错误消息文件 | 本候选构建 | dist 不变 | dev 保留 last-good；CLI 非零 | 定位诊断 | 修正后重新构建 | V-1、V-2、V-8 |
| 语言改动命中旧缓存或 helper 使用默认环境 | 对应语言 | 发布错误语言/状态 | 文案或岛语言不符 | 元数据/浏览器断言 | 目录指纹、按语种批次；修正后整代替换 | V-3、V-4、V-7 |
| 单错误页托管/禁脚本 | 请求不存在地址的读者 | 无 | 默认错误页及所有语言首页仍可用 | HTTP + no-script 检查 | 启脚本选语言；托管不变 | V-5、V-7 |
| 产物错误映射损坏/声明文件丢失 | 本次 preview | dist 不变 | 启动失败并提示重新构建；旧单语言空表仍可用 | §2.4 启动校验 | 用原成功产物恢复或重新构建 | V-5 |
| 新 CLI 与旧配置/注册库错配 | 作者本地构建 | 不写半成品 | 明确配对/配置失败 | 现有身份/manifest 检查 | 同时回退 CLI、配置与库 R | V-9 |

本期仅交付本地固定配对，不公开推送或部署。配置、内容与 CLI/库按同一提交集回滚；输出用原成功代恢复，不删除作者内容。公开发布由 P4 的独立发布门负责。

## 9. 验证

### 9.1 检查定义

下列新增测试/项目是计划交付物；只有对应实现完成后才能执行。每项只安排一个时点。项目内使用 mbx，Playwright 每次单 project 串行；浏览器入口的 RUSTIFY_DOCS_BIN 按 §3.2 绑定安装 C，toolchain 另以 --git/--rev 绑定 C，不能靠该变量指定来源。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F10/AC-1/3；配置兼容、前缀/tag/消息字段/占位符错误定位 | `mbx test --locked -p rustify-docs --lib config::`；`mbx test --locked -p rustify-docs --lib labels::` | M1 局部 |
| V-2 | WORK-F10/AC-1/2；lang 注册三端一致、默认/真实/回退/草稿/孤儿/地址冲突 | `mbx test --locked -p rustify-docs-syntax`；`mbx test --locked -p rustify-docs-macros`；`mbx test --locked -p rustify-docs --test i18n graph` | M1 局部；解除 M2 前置 |
| V-3 | WORK-N6/N7；相对链接/资源、目录导航、源 lang、目录 cache/watch、allowlist 不扩张 | `mbx test --locked -p rustify-docs --test i18n projection`；`mbx test --locked -p rustify-docs-cli --test i18n inputs` | M2 局部 |
| V-4 | WORK-F10/AC-1/3、C-4；同页三语言 helper/slot/demo，未知 SDK 语言回英文，hydrate metadata 一致 | `mbx test --locked -p rustify-docs-cli --test i18n render`；`mbx test --locked -p rustify-docs --no-default-features --features site-ssr --lib site::registry::`；`mbx test --locked -p rustify-docs --lib shell::` | M3 库内局部；真实消费者在 M5 形成新 R/C 后补验 |
| V-5 | WORK-F10/AC-2/4、WORK-F11/AC-1–4；SEO/多语 404/无 URL；映射往返/安全、脱离源码的 dist、配置漂移/普通 foo/404.html/旧产物兼容；dev 失败后保持同代选择并可恢复 | `mbx test --locked -p rustify-docs --test i18n metadata`；`mbx test --locked -p rustify-docs --lib publish::`；`mbx test --locked -p rustify-docs-cli --test i18n serving`；`mbx test --locked -p rustify-docs-cli --bin rustify-docs preview::` | M4 局部 |
| V-6 | WORK-F15/AC-4；恰好 3+2 warning、位置/建议、JSON/提升错误、dev 恢复契约 | `mbx test --locked -p rustify-docs-cli --test i18n diagnostics`；`mbx test --locked -p rustify-docs-cli --test check` | M4 局部 |
| V-7 | WORK-F10 全 AC；WORK-N3/AC-2、WORK-N5 新 UI；内容站与真实 Rust 消费者的根/子路径/meta 切换、无脚本、三语言 SSR→ready→点击；404 活动树/共享 slot/SDK 回退、焦点/双主题/移动/reduce、dev last-good | §9.2 的两组三项目和 docs-dev，命令逐一串行；Rust 载体复用 components 的真实双图及语言叠加件，不能以 CLI HTML/metadata 检查替代水合 | M5 阶段收口 |
| V-8 | WORK-F11/F15 前期 AC 与本期终验；WORK-N6/N7、严格 lint、完整 docs 回归与幂等生成 | 分别 `mbx test --locked -p rustify-docs`、`mbx test --locked -p rustify-docs-cli`；分别 `mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` 与 CLI 同命令；`cargo fmt --all -- --check`；两次构建新语言 fixture 比较完整产物，坏配置构建核对旧产物不变 | M5 阶段收口 |
| V-9 | WORK-N4 全 AC；固定 R/C 实装、两模板/第三语言离线、独立 SSR/hydrate/配对错配 | 按 §3.2 安装并记录 C；`mbx xtask docs-verify --group features`、`mbx xtask docs-verify --group artifacts` 各一次；`mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"` 一次。新增语言 walkthrough 使用 toolchain 内部实装的 C；artifacts 为开发态产物检查，独立记录来源 | M5 阶段收口 |
| V-10 | WORK-N3/AC-4 与 WORK-N5；真实稳定浏览器、手机、原生缩放与语言朗读 | §9.2 同一已构建 fixture/入口；Agent 操作可用真实渠道，记录版本/资源和缺口，不用引擎或模拟替代设备 | M5 阶段收口，资源未齐则保留未验证 |

### 9.2 真实环境和证据边界

内容站与 Rust 消费者分别保留，避免零 wasm 站点通过却未执行水合；均复用 `serve.sh` 的 build/preview 和既有 meta host，不另起基础设施。新增托管项目如下，base 分别为 `/`、`/rustify-ui/`、`/rustify-ui/`；原 components 项目继续使用原单语言 fixture。

| 载体/serve source | fixture 与构建 | project / 端口 / spec |
| --- | --- | --- |
| 内容站 / i18n | `fixtures/i18n/content/` ★，无 `[rust]`；沿 content 分支构建 | docs-i18n-root / 4201、docs-i18n-subpath / 4202、docs-i18n-meta / 4203；只匹配 i18n.spec.ts |
| 真实 Rust 消费者 / i18n-components | 复制现有 components，再叠加 `fixtures/i18n/components/` ★ 中的测试配置/注册页/slot 增量；保留原 Cargo 包和演示，无另一套组件实现 | components-i18n-root / 4207、components-i18n-subpath / 4208、components-i18n-meta / 4209；只匹配 i18n-islands.spec.ts |
| 开发态 / docs-dev | dev-i18n.spec.ts 每例组装同一 components 语言叠加件，启动已安装 C 的 dev，复用 dev-components.spec.ts 的临时副本与真实编译方式 | 动态 `--port 0`；只归 docs-dev，不交给内容站三项目 |

Rust 副本先复制到独立目录，将三份框架相对 path 解析为本仓绝对路径（沿 dev-components.spec.ts），再运行 `scripts/docs-pages-prepare.py --project <副本 Cargo.toml> --cli "$RUSTIFY_DOCS_BIN"`；开发态保留 path，固定 C 时投影为真实 R 依赖和锁。不在原 fixture 上叠加，不绕过配对。serve 的 Rust 分支设置 RUSTIFY_DOCS_CARGO=mbx，保留独立 SSR/hydrate 图。原 docs 项目（含引擎项目）排除 i18n.spec.ts 和 i18n-islands.spec.ts；新项目分别只认自己的 spec，dev-i18n 只归 docs-dev，输出目录和浏览器会话独占。

Rust 叠加件的固定页面合同如下（路径均相对部署 base）；en 为默认语言，zh/fr 分别用 prefix zh/fr，fr 提供部分项目文案。CLI 的 render 测试可复用相同测试数据，但不替代下表的浏览器执行。

| 真实入口 | 页面与交互断言 |
| --- | --- |
| locale/ ↔ zh/locale/ | 新增同一逻辑 `/locale/` 的 en/zh 注册页，分别有可见语言标记，复用 CounterDemo/ThemeProbeDemo；按语言菜单同页切换，SSR 与 hydrate metadata 的目标语言一致；到 ready 后计数并打开 Dialog，SDK 默认关闭名称分别为 close / 关闭 |
| fr/locale/、zh/plain/ | fr/locale/ 复用英文注册页形成缺译投影，html.lang=fr、正文源语言 en、缺译提示与逐项文案语言正确；岛环境保持 fr，SDK Dialog 的关闭名称回英文 close，实际点击可关闭；zh/plain/ 验证默认静态注册正文的中文壳回退 |
| zh/missing/a/b/、fr/missing/a/b/ | 叠加件在逻辑 `/404/` 的 before_content 共享 slot 放 CounterDemo、not_found 覆盖放 ThemeProbeDemo。preview 返回目标语言文件；meta host 返回根错误页，激活目标 template 后才扫描/水合。活动树中每个 slot 一次、island key/seed 唯一、环境为所选 ui_lang、挂载根 seed 对应 metadata，全部 ready 后计数/打开并关闭 Dialog；未选模板不挂载，不重复 boot |

V-7 沿 islands.spec.ts 的真实资源暂停方式先检查 SSR 和禁用控件，释放 wasm 后检查原 SSR 树附着、ready 和可点击结果；meta 404 须在 boot 前断言已选择目标语言且仅活动树进入 parsed。禁脚本时内容、源码和首页链接可读：preview 就近语言，meta 默认语言并列各语言首页，岛控件保持禁用。检查每个入口的 console、CSP、失败请求与真实 HTTP 404，不把预期的主文档 404 记为意外失败。dev-i18n 从 fr/locale/ 及 fr 深层 404 进入同一真实消费者，覆盖文案/译文成功变化、错误构建期间保留旧页/旧语言映射和修复后恢复 ready；不重新把已在 V-5 证明的全部映射边界做一遍浏览器穷举。

先按 §3.2 导出安装 C 的 RUSTIFY_DOCS_BIN，再逐条运行；每次仅一个 project，端口、输出目录和会话释放后再开始下一条：

```sh
npx playwright test -c playwright.docs.config.ts --project=docs-i18n-root i18n.spec.ts
npx playwright test -c playwright.docs.config.ts --project=docs-i18n-subpath i18n.spec.ts
npx playwright test -c playwright.docs.config.ts --project=docs-i18n-meta i18n.spec.ts
npx playwright test -c playwright.docs.config.ts --project=components-i18n-root i18n-islands.spec.ts
npx playwright test -c playwright.docs.config.ts --project=components-i18n-subpath i18n-islands.spec.ts
npx playwright test -c playwright.docs.config.ts --project=components-i18n-meta i18n-islands.spec.ts
npx playwright test -c playwright.docs.config.ts --project=docs-dev dev-i18n.spec.ts
```

浏览器任务中的“读者”只读取页面，以菜单切换/阅读/复制/岛交互完成任务，不靠源码补步骤；测试探针只用于记录水合、语言和失败边界。

桌面 Chrome/Firefox/Safari 稳定版的同页切换、语言 404、导航/主题/演示按当期范围执行；iPhone/Safari、Android/Chrome 以设备真实可用 CSS viewport 接近 375px 的入口记录实际值，不能把桌面 375px 当真机。若无法满足 PRD 指定宽度或最新版本，记差异并保留该验收项未通过。VoiceOver/Safari 核对新控件名称与 source_lang，不把自动化 aria 快照当朗读证据。

Playwright Chromium/Firefox/WebKit 仅为引擎回归；可用的原生浏览器通道与真机/远程会话由维护者提供，Agent 操作。有资源时执行，无资源时精确记录缺口，按 D16 不阻塞继续本地开发；本期阶段出口不能写全矩阵通过。正式 P2 性能与真机欠证仍只在原计划补证，不借本期局部回归关闭。

## 10. 里程碑与验收安排

一阶段五个里程碑。M1–M4 都是局部行为门，M5 集中执行整期回归与 UI/真实环境。共享 `content.rs`/`generation.rs`/`rendering.rs` 由主实施者顺序集成；独立测试和作者说明在接口稳定后可委派，不能并写共享文件或并跑浏览器/同一输出目录。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 配置和真实语言页面身份 | C-1 本地能力；当前工作树核对 | §2.1/§2.2 的 config/labels/注册元数据与 content 初始图；沿 RawConfig、PageDeclaration/Manifest 和原 route_for；★i18n 测试先红后绿 | V-1、V-2；回写「实施进度」 |
| M2 | 完整回退图与语言内链接 | PageLocale/LocaleView 与 V-2 | content::analyze_with_surfaces/navigation、FragmentCache、Snapshot/InputReceipts、dev watch；按 §5.1 扩展现有图解析，不重写引用器；主实施者写共享图 | V-3；回写「实施进度」 |
| M3 | 多语言阅读与岛语言一致 | M2 投影图与 V-3 | rendering::build_cached/render_surfaces、registry::prepare_environment、shell、markdown/builtins/demo 目录消费及语言 UI；按 §2.3/§6 复用批量 helper 与现有 details | V-4；回写「实施进度」 |
| M4 | SEO、404 与缺失检查闭环 | M3 的 shell/批次；目录收据可用 | generation::sitemap/generate_extended、publish::Manifest/add_manifest、main::build/preview、preview::Site、dev::Good/needs_reload、mode.js/site.js、html_check；按 §2.4、§5.2–§5.4 和原 warning/report 实现同代映射/404；补作者说明与 fixture | V-5、V-6；含脱离工程预览、旧产物兼容与失败代保持检查；回写「实施进度」 |
| M5 | P3 阶段收口与固定本地交付 | V-1–V-3、V-4 库内局部、V-5/V-6 通过；派生页/CLI/fixture 可用。V-4 真实消费者随本里程碑新 R/C 补验，通过前 M3 保持进行中 | §3.2 R/C 配对、真实安装；按 §9.2 组装内容站与真实 Rust 三种托管/dev 载体，集中验收；toolchain 显式 --git/--rev 实装 C；同步设计和作者文档，主实施者负责最终集成与缺证登记 | V-4 待补项及 V-7–V-10；含活动 404 岛及工具链 C/R 身份，仅全部必需项通过才记本期终验；回写「实施进度」 |

提交点：M5 全部里程碑完成后的最终交付提交必有；该点内先形成库 R，再形成仅五个配对文件的 C，随后提交证据/进度并按顶部协议回填 SHA。中间局部通过不要求制造阶段发布或重跑全量。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 真实稳定浏览器/两手机/VoiceOver 资源 | WORK-N3 当期出口和实际可访问性缺证 | 维护者提供现有设备或远程入口；Agent 按 V-10 执行；无资源保留未验证 | M5 终验；不作为本地开发前置 | 不阻塞 M1–M4；阻塞完整矩阵终验 |
| P2 未完 5/7 | 不应把继承能力视为整期通过 | 原计划负责历史欠证；本期只核对实际依赖，真实缺陷按影响处置 | 使用相应能力时 | 否；D16 已明确 |
| 语言目录改变公开地址 | 维护者可能破坏既有链接 | 作者文档说明 prefix 变更；不自动迁移 URL | 配置变更提交前 | 否；技术行为已定 |
| 旧 CLI 不能读新配置 | 独立消费者构建失败 | 同步安装 C/库 R 与配置；V-9 验证明确拒绝与回退 | M5 | 否；已定义兼容策略 |

- 最终状态：**Ready**。
- 定级理由：P2 本地和本人验收前置有证据，设计、地址、目录、错误与迁移均已明确；产物到 preview/dev 的映射合同、真实多语 Rust 浏览器载体和固定 C 工具链来源已定义，可以开始本地实施。外部验收资源是已知出口依赖，不是未知产品合同；未取得时 M5 不记完成，继续独立工作不改变要求。M5 的本地 V-7–V-9 通过并形成记录/固定配对后即可解除 P4 的相应开发前置；不要求等 V-10 设备补验或先把 M5 整行标完成。
- 待用户拍板的产品问题：无。设备可用性按实际回复更新资源登记，不默认为已具备。

## 12. 已知坑与历史教训

- `PRD-DOCS-SSG.md` 的早期现状/工具链叙述早于 P1/P2；当前运行配置以 `mise.toml`/Cargo 及本计划核对的行为为准，不照搬旧 Makepad/CSR 耦合描述。
- P2 真实本人验收只覆盖其记录版本；[原验收记录](../validation/docs-ssg-p2/M6.md)的历史“待确认”已被后面的本人通过节替代，不能重新要求招募或限时。
- [最新诊断修复复审](DOCS-SSG-P2-gaps-2.code-review.r2.md)保留第三方原文边界；新增 i18n 诊断用现有逐字编码，不按 ` at line ` 等内容分割作者值。
- 省略 i18n 配置时旧单语言页面的 frontmatter.lang 不自动变为翻译关系；必须保留该兼容边界，避免把旧中英独立文章互相回退。
