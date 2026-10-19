# DOCS-SSG-P4 · Rustify UI · 离线搜索、AI 索引与官方站发布（DOM · 构建期静态生成）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-16 +08:00 · `77ca4f47ad099d1b162cf5b6190cc0b49d775da0` · dirty；未提交项为 SDLC 登记、P3 评审、P4 计划与评审报告，产品源码和原 PRD 与 HEAD 一致；事实基于当前工作树核对。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P4（第 4 期）**，以 §7.1 出口、§7.2 矩阵为准。计划版本：v2。
>
> 本期交付：**当前语言离线全文搜索、每语言 llms.txt、官方站中英完整能力，以及固定 Git 安装和 GitHub Pages 发布/回滚**。
> **开发范围：本仓文档流水线、同源搜索脚本、官方 site/配置/中文内容、验证与既有 Pages 工作流；DOM 路径，依据 PRD P4 与 D16**。
> 本期独特职责：把 P3 语言图同时用于读者搜索和外部 AI 发现，完成官方站真实发布门。
> **顶层排除：不做 AI 问答、语义检索、MCP、Markdown 副本、统计、Service Worker 离线缓存、多版本、GPU 或其他平台的部署适配**。

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

- 最近更新：2026-10-19T12:18:07+08:00
- 当前进度：6/7 个里程碑完成
- 当前状态：V-11按D17/D18范围已通过；C0恢复后完整97文件逐项匹配，准备首发与回滚C0/C1
- 最近完成：M6 · C₅本地出口、完整回归与正式性能
- 下一步：冻结仅公开说明/配对身份变化的C1，完成受影响本地身份/消费验证，呈现具体C/R/产物/Pages缺口及回滚范围后取得最终公开授权
- 当前阻塞：Pages未启用、C0/R公共commit仍422；公开推送/部署授权与V-12未完成，原生资源门已解除
- 代码基线：R0fd34eda/C3dbb079a（0.1.29）真实本地Git安装；Docs363、CLI182（另1原有ignored）、搜索74、官方168、正式搜索240样本及最大官方岛5冷样本通过；原生与旧证据复用范围见M7记录

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-16T00:35:39+08:00 | 当前语言索引、最终 DOM 提取、注册布尔策略与保留路径通过 V-1 | [M1 记录](DOCS-SSG-P4.records/M1.md) | fecc230c |
| M4 | 已完成 | 2026-10-16T01:04:20+08:00 | llms.txt、来源地址与描述兜底、排除/确定性/旧代保护通过 V-5；Dev 默认草稿补证通过 | [M4 记录](DOCS-SSG-P4.records/M4.md) | fecc230c |
| M5 | 已完成 | 2026-10-16T01:14:22+08:00 | 官方两真实中文页、双语配置与 V-6 冷源图/直接 SSR 通过 | [M5 记录](DOCS-SSG-P4.records/M5.md) | fecc230c |
| M2 | 已完成 | 2026-10-16T02:59:24+08:00 | 固定 C 96a739c9 的真实100页 matching5/5通过，撤销开发态退出限制 | [M2 记录](DOCS-SSG-P4.records/M2.md) | fecc230c、C 96a739c9 |
| M3 | 已完成 | 2026-10-16T03:24:58+08:00 | C₃ 根/meta各21/21、子路径恢复及键盘15/15，快速关闭重开新反例关闭 | [M3 记录](DOCS-SSG-P4.records/M3.md) | fecc230c、修复 a2ad3940、C 18b229d9 |
| M6 | 已完成 | 2026-10-16T11:34:47+08:00 | C₅真实安装/三托管/图与作者检查、受影响引擎及正式搜索/B500/A-B读者性能均通过；M7另验；UI R1四项已修改待评审 | [M6 记录](DOCS-SSG-P4.records/M6.md) | R₂ 41b19b63、C₂ d0d84d7b；R₃ a2ad3940、C₃ 18b229d9；测试/撤回 a13a2793、14269ed7、58b7a0d9；C₅ 936149b9；验收 06be2ee3；UI修复 740610db（C 1bbc54b0，证据 600efef4） |
| M7 | 阻塞 | 2026-10-19T12:18:07+08:00 | V-11已通过；Safari新岛干净前台及Chrome禁脚本/恢复已补；C0环境恢复后97文件逐项匹配，继续C1和公开交付 | [M7 记录](DOCS-SSG-P4.records/M7.md) | 修复R 78f54b62；C a31509aa；证据 3c38da04；续查 a5fc4a29；修复R 0fd34eda；C 3dbb079a；补验 455527ad；验收决定 a46ad4da；Chrome补验 918be79b；V-11收口/恢复 c0611fde |


## 0. 需求、范围与决策

### 0.1 需求与约束账本

AC 原文沿用 PRD §3。本期终验 WORK-F9/F12，收口 WORK-F16、WORK-N1、WORK-N3、WORK-N5 的跨期需求；前期未覆盖的真实环境证据仍需补齐，不能用新增 AC 的通过关闭整条需求。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F9 | 功能 | PRD §3/§7.2，AC-1–7 | 当前语言全文搜索、标题优先、定制/缺译页、键盘、404 与失败重试 | §2、§5.1–§6；V-1–V-4、V-8–V-11 | 已确认；本期终验 |
| WORK-F12 | 功能 | PRD §3/§7.2，AC-1–4、D12–D14 | 每语言 llms.txt，导航分组、HTML 地址/描述、缺译地址和排除规则 | §3、§5.3；V-5、V-8、V-12 | 已确认；本期终验 |
| WORK-F16 | 功能 | PRD §3/§7.2，AC-3/4；AC-1/2 继承 P2；D16 | 官方中英首页/quickstart、搜索与 AI；固定 Git/真实 Pages 部署及零 CSP 违规 | §4、§5.4、§8；V-6–V-8、V-12 | 已确认；P4 终验 |
| WORK-F4 | 前期验收补证 | PRD AC-4、D16；P2 原公开交付检查 | 只补组件围栏在真实 GitHub 文件预览中可识别；不重开原全部功能验收 | §9.3；V-12 | P2 原欠证在本期发布节点补齐 |
| WORK-N1 | 性能 | PRD AC-5；AC-1–4 继承 P1/P2 | 1000 页首次查询 ≤1s、热查询 ≤100ms；输入立即聚焦，继承内容/岛预算 | §2.2、§7、§9.2；V-2、V-10 | 已确认；P4 终验 |
| WORK-N3 | 兼容 | PRD AC-5/6、AC-2/4 复核、D11/D17/Q3 | 禁脚本可读、桌面稳定矩阵与严格 CSP；Firefox 延期本期补验；手机实机按 D17 不再执行 | §4、§6、§9；V-8、V-9、V-11、V-12 | 已确认；P4 终验 |
| WORK-N5 | 可访问性 | PRD AC-2/4；其余 AC 覆盖全部页面类型 | 键盘完整操作、焦点、VoiceOver 结果数量/选中朗读、主题/reflow/AA | §6；V-8、V-9、V-11 | 已确认；P4 终验 |
| WORK-N4 | 工具链 | PRD 全部 AC 每期复核 | 搜索不带 Makepad，不增加作者 Node 或站点 Rust 编译要求 | §2、§4；V-7、V-9 | 本期复核 |
| WORK-N6 | 可靠性 | PRD AC-1–3 继承，新增索引/产物回归 | 确定字节、错误不覆盖旧代；索引与页面同一事务 | §3、§8；V-5、V-9 | 本期回归 |
| WORK-N7 | 安全/隐私 | PRD 全 AC，AI 输出属于作用域 | 无草稿/源文件/机器信息泄露，无外部查询，无 HTML 注入 | §3、§5、§7；V-1、V-5、V-8、V-9 | 本期回归 |
| C-1 | 实际前置 | PRD §7.1；[P3 计划](DOCS-SSG-P3.md) §2/§9、[本地交付](DOCS-SSG-P3.records/M5.md) | P3 的 PageLocale/LocaleView、按语言渲染及本地出口已交付，固定 R₂/C₂ 与记录可用 | §4、§11；V-1 | 已满足本地开发前置；不等待 P3 设备补验 |
| C-2 | 发布边界 | PRD D16/Q2/Q4；`docs-pages.yml` | 公开 Git 安装、Pages 与回滚由本期负责；发布目标固定现有仓库和 /rustify-ui/ | §4、§8；V-7、V-12 | 合同确定；环境/发布授权到节点确认 |
| C-3 | 工具/身份 | `mise.toml`、CLI build/new、P2 固定配对流程 | mbx，Docs/CLI 分包，真实库 R/CLI C 版本与完整锁；单项目浏览器串行 | §3.2、§9；V-7、V-9 | 已核实 |
| C-4 | 呈现/视觉 | `AGENTS.md`、`DESIGN.docs-ssg.md`、既有 shell brief | 仅 DOM，搜索和语言界面扩展阅读世界，真实浏览器验收 | §6；V-8、V-11 | 已确认 |
| C-5 | 验收资源与结论 | 用户本轮决定；[PRD D17/D18](../PRD-DOCS-SSG.md) | 手机实机本次取消，VoiceOver由用户本人确认通过；桌面原生其余任务继续，历史手机证据保留原范围 | §4、§9.3、§11；V-11 | 手机与VoiceOver不再阻塞；其余必需检查未齐仍阻塞发布门 |
| C-6 | 历史闭环 | P2 原计划 M6/M7、PRD D15/D16 | 保留本人通过事实；原生环境/正式性能/真实 GitHub 预览按原出处补证，不宣称 P2 已全验 | §4、§9、§11；V-10–V-12 | 已确认 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 搜索运行形态 | 同源原生 DOM 对话框 + 按需 JS module/Worker | 内容站不编译站点 Rust、不下载搜索 wasm；普通链接导航 | ADR-1；WORK-N1/N4 |
| D2 | 索引 | 每语言一份结构化页面/章节 JSON；Worker 缓存文本，查询筛选后校验词边界 | 不在首次查询前给全部文本建倒排表；仅本地内存，重载后重取 | §2.2；规划探针；PRD 1000 页规模 |
| D3 | 中文与英文匹配 | 同一个 Worker 使用 Intl.Segmenter 做词边界，英文 ASCII 大小写无关及词首前缀 | 不引入字典下载、外部搜索或 Node 构建；明确测试查询和边界 | §2.2、§7；ECMA-402 |
| D4 | 缺译 | 搜索 href 为当前 ui_lang route；AI href 为真实 source_route 并标 source_lang | 两种消费者使用同一语言事实、不同公开地址语义 | PRD D13；P3 PageLocale |
| D5 | 官方中文范围 | P4 增加首页和 quickstart 真译文，其他指南/分类回退 | 12 份英文指南仍直接取原源文件；不顺手翻译全站 | WORK-F16/AC-4；§5.4 |
| D6 | 发布 | 既有固定 C 工作流构建并发布完整目录，记录真实 build_id；上一成功 C 可回滚 | 不新增部署平台；Pages 配置和授权先核实，实际环境绿才完成 | ADR-2；既有 docs-pages 工作流 |
| D7 | 无描述首页的 AI 条目 | 显式描述 → 正文摘要 → 首页 tagline → 来源语言的“{站点名}首页” | 空/纯空白视为缺失；合法空首页继续发布且索引描述非空，不沿用无描述注册页的排除规则 | 用户 2026-10-16 确认兼容方案；WORK-F12；§5.3 |

### 0.3 ADR-lite

#### ADR-1：原生搜索增强与按查询取词边界

- 状态：Accepted（技术方案）。
- 驱动：WORK-F9 中文/英文语义与 WORK-N1 首次查询预算，WORK-N4 的内容站不编译 Rust/最小工具链。
- 候选 A：框架携带独立预构建的 Rust/Wasm 搜索器，构建期词典分词与倒排索引。跨浏览器算法一致；代价是新增二进制供应、字典版本/体积和独立 wasm 产物制作流程。不能复用任一作者站点的 Rust 编译，否则破坏内容模板。
- 候选 B：预渲染 DOM 搜索对话框，延迟加载 Worker 和页面数据，浏览器词边界只用于查询及候选匹配。已有静态 JS/CSP/Worker 能力足够；代价是词边界受浏览器实现影响，要验证全部目标浏览器。
- 决策：B。搜索不依赖任一演示实例，演示 trap 后仍可搜索。不存在第三种运行服务或测试专用搜索实现。
- 已核实的可行性边界：临时 Chromium 153.0.8010.12 探针以 12 份当前指南轮换成 1000 页（约 16.6 MB JSON 字符数）测得数据准备 20.8ms，四个查询 3.4–6.4ms。它没有真实 fetch、Worker 传输、正式中文语料或结果 DOM，不能记为 AC 通过。全量构词试验仅在 Node 中已有约 829ms 成本，因此不把它放在首次查询路径。
- 正面后果：不新增作者工具/依赖，输入主线程独立，零搜索 wasm，现有构建即可嵌入脚本。
- 负面后果：1000 页文本仍须首次下载；冷态 IO/解析/绘制必须合并量测。更多页面可能让线性匹配失效，不能仅靠引擎微基准保证预算。
- 重评条件：实现经过 V-2 缩小基准后仍无法满足 V-10，先定位提取冗余、JSON/网络与渲染瓶颈；若必须引入预分词/新二进制才能达标，修订本 ADR 和工具链交付合同后继续，不静默换算法或放宽预算。

[ECMA-402 Segmenter 规范](https://tc39.es/ecma402/2026/#segmenter-objects)定义 word granularity、索引和 isWordLike；词边界依实现而定，因此各引擎需用同一查询断言验证。这里采用的查询匹配及排序是本计划的技术决策，不是规范保证。

#### ADR-2：固定来源发布与完整目录回滚

- 状态：Accepted（技术方案）；发布环境见 §4。
- 候选 A：本机构建后上传 dist。步骤少，但来源、依赖投影与线上字节难统一追溯。
- 候选 B：复用 `.github/workflows/docs-pages.yml`，指定完整 C SHA，真实 Git 安装、prepare、check/build、同 run 上传与部署。保留候选/框架/build_id，运维步骤已有实现。
- 决策：B。允许的源码投影仅 `site/Cargo.toml` 三条依赖与 `site/Cargo.lock`；新中文源、配置、脚本必须已在 C 中，不在部署时临时改。
- 代价：依赖 GitHub Pages 配置/环境权限和真实公共 Git 可达；这些必须核实，不用本地 Git remap 当公开安装证明。
- 重评：托管变更、工作流无法满足现有 artifact identity 或需要独立域名时另行修订；当前不新增云部署适配。

### 0.4 职责与事实所有权

|  | 文档流水线/官方源 | 浏览器增强 | GitHub/维护者 |
| --- | --- | --- | --- |
| 拥有 | 页面语言事实、公开范围、序列化索引、llms、固定候选 | 查询、排序、高亮、焦点、失败重试和本地内存 | Pages 启用/权限、实际部署和原生设备资源 |
| 不拥有 | **搜索服务、模型服务、跟踪或用户数据存储** | 翻译存在性、发布授权、可信源范围 | 文档索引或运行时搜索算法 |

框架决定公开内容集合，Worker 只能搜索交付给它的那份数据；发布平台只接收已校验的完整产物。

### 0.5 明确不在本期

不建设云搜索/统计、AI 问答、MCP、语义检索、Markdown 副本、llms-full.txt、多版本、CMS、RTL、GPU 或 Portal。离线搜索指已取得索引后不请求外部服务；不承诺首次断网可打开未缓存站点，不新增 Service Worker。

P2 原有记录、未完成计数不重写为“已完成”；补验证据链接回原计划，P4 仅记录本期整合出口。用户本人通过事实保留，不另加招募、资格或限时门。

## 1. 当前事实与改动面

### 1.1 已核实行为

- [已核实·足够] frontmatter 已有独立 `search`/`ai`/`index` 字段；目前只是意愿元数据。不能用 index=false 自动替代 search=false 或 ai=false。
- [已核实·缺口] `generation.rs::SITE_FILES` 没有 llms.txt，脚本只有基础交互/岛启动，无搜索 → 补语言索引、保留文件与按需增强，纳入既有输出事务。
- [已核实·足够] P2 注册页有 description，真实 HTML 经 `analyze_with_surfaces` 和最终文档检查；已有 dom_query/html5ever，能从已生成 DOM 提取文字，不必新增 HTML 解析库。
- [已核实·足够] `shell.rs::CSP` 已允许同源 worker/connect；没有 blob worker 或内联脚本的必要。
- [已核实·缺口] 官方 `rustify-docs.toml` 只有英文配置，site/lib 的页面只有英文，12 指南由 include 白名单选取 → P4 加双语声明、中文首页/quickstart，其他页面由 P3 回退。
- [已核实·足够] 既有 Pages 工作流只接受完整候选 C，已实现固定 Git 安装、prepare 输入保护、check/build、manifest 身份、上传与部署，不会自动启用 Pages。
- [未核实·阻塞发布] 2026-10-15 的只读 API 调查确认仓库 public/main 且当时凭证具 admin/push，但 `gh api repos/rockie/rustify-ui/pages` 返回 HTTP 404；这不能证明 Pages 已配置。发布前仍须核实 source/environment，不在规划时启用或部署。
- [已核实·足够] `content.rs` 已有 PageLocale/LocaleView 与 Analysis.locales/catalogs；P3 当前 4/5，M1–M4 已完成，M5 的 V-4、V-7–V-9 本地交付及固定配对见[记录](DOCS-SSG-P3.records/M5.md)。其 V-10 设备欠证不阻塞 P4 本地开发；没有将这些历史结果计作 P4 验收。

### 1.2 文件清单

| 落点 | 改动与同构实现 |
| --- | --- |
| `crates/rustify-docs/src/search.rs` ★、`ai.rs` ★、`lib.rs`、`generation.rs` | 两个本地深模块各拥有提取/过滤/序列化；generator 连接同一语言图与输出事务；沿 sitemap、insert_output/fingerprint |
| `crates/rustify-docs/assets/search.js` ★、`crates/rustify-docs/assets/search-worker.js` ★、`crates/rustify-docs/assets/site.js` | bootstrap 拥有 module 加载前状态/重试，module 拥有 Worker/结果交互；复用 site.js 的键盘/复制反馈约定，不依赖岛运行时 |
| `crates/rustify-docs/src/shell.rs`、`labels.rs`、`crates/rustify-docs/assets/docs.css`、`classes.rs` | 静态搜索入口/对话框、文案、主题样式及脚本状态类；基于 P3 Catalog |
| `crates/rustify-docs/src/content.rs`、`frontmatter.rs`、`html_check.rs` | 保留路径、已解析 search/ai 策略，最终索引 href/anchor 校验；HTML 提取复用已锁定解析器 |
| `crates/rustify-docs-syntax/src/lib.rs`、`manifest.rs`、`crates/rustify-docs-macros/src/expand.rs`、CLI `rendering.rs` | 注册页新增可选 draft/index/search/ai 布尔元数据；冷清单、宏及实际 Page.front 一致 |
| `rustify-docs.toml`、`site/src/lib.rs`、`site/src/pages.rs`、`docs/zh/quickstart.md` ★ | 官方语言配置/中文真实源与注册首页；只维护两份真实中文内容 |
| `crates/rustify-docs/tests/search.rs` ★、`crates/rustify-docs/tests/ai.rs` ★、`crates/rustify-docs-cli/tests/search.rs` ★、`crates/rustify-docs-cli/tests/artifacts.rs` | 公开生成/CLI行为与源泄露/拒绝/确定性测试，沿现有测试 fixture |
| `xtask/src/docs_corpus.rs`、`xtask/src/docs.rs`、`xtask/src/docs_search.rs` ★、`xtask/src/main.rs` | 新搜索语料和性能/验收入口；保留原 B 与 --p2 输出字节合同 |
| `tests/docs-ssg/search-worker.spec.ts` ★、`search.spec.ts` ★、`search-perf.spec.ts` ★、`official.spec.ts`、`serve.sh`、`playwright.docs.config.ts` | Worker 局部协议与完整交互；真实产品/三种托管、按需资源与计时 |
| `scripts/docs-pages-prepare.py`、`.github/workflows/docs-pages.yml`、`scripts/docs-pages-verify.py` ★ | 保留已有 prepare 的受保护输入，必要时扩临时 fixture 新目录复制；发布前/后遍历语言页面、索引与身份，不新建部署平台 |
| CLI 版本/两配对资产/根锁、`README.md`、`docs/docs-ssg.md`、`DESIGN.docs-ssg.md`、既有 surface brief | 按 §3.2 配对和已通过的公开安装/托管说明；不能预写已上线 |

所有代码改动限本仓。没有新 crate/npm 依赖，没有基础设施或容器；不触碰 Makepad/SDK 的 GPU 模式。

## 2. 模块、接口与匹配合同

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| search.rs | generator | §2.1 的 build 签名；只用本次生产文档和语言图 | 无 | 公开过滤、章节/锚点、文字提取与确定性数据 | 进程内/现有 DOM parser | 生成数据和可达 href |
| ai.rs | generator | §2.1 的 build 签名；每语言 llms.txt，描述必需、Home 兜底及缺译链接见 §5.3 | 无 | 导航分组、描述优先级、去重、转义、绝对/根相对地址 | 进程内 | 文本/HTTP |
| site.js / search.js | static shell | bootstrap 同步打开/聚焦并负责导入失败重试；module 仅显式初始化后绑定 Worker/结果；只应用当前请求/实例 | module加载、Worker消息 | UI状态/键盘/恢复与安全渲染 | 本地 module/Worker，无远程 adapter | 浏览器公开交互 |
| search-worker.js | search.js | §2.3；输入固定 schema；同一查询返回稳定排序与原文偏移 | 原生 Worker | 词边界、排名、分页/摘要 | 进程内浏览器能力 | Worker协议测试 |
| 发布验证 | existing Pages workflow | 固定 C/R/build_id 与完整文件清单 | GitHub既有 API/HTTP | 同 run 身份与可达性核对 | 真正外部平台；不模拟真实发布 | CLI/公共 HTTP/浏览器 |

search/ai 各自有完整且不同的过滤和输出策略，删除任一会把实际复杂度散回 generator；不为它们新增一层通用“索引适配器”。

### 2.1 公开数据与提取

入口使用现有 Project/Analysis/Diagnostic 和 Arc，不引入共享索引框架：

```rust
// search.rs；HTML 已由本代 shell 渲染，随后仍须通过现有最终文档校验。
pub struct SearchBundle {
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub indexes: BTreeMap<String, String>, // ui_lang -> 带部署 base 的指纹 URL
    pub module_url: String,
    pub worker_url: String,
}
pub fn build(
    project: &Project,
    analysis: &Analysis,
    documents: &BTreeMap<String, Arc<[u8]>>, // output path -> 完整候选 HTML
) -> Result<SearchBundle, Vec<Diagnostic>>;
// ai.rs
pub fn build(
    project: &Project,
    analysis: &Analysis,
) -> Result<BTreeMap<String, Arc<[u8]>>, Vec<Diagnostic>>;
```

SearchIndex schema 1，确定 JSON key/数组顺序，文本为原文，不携带源文件路径、机器信息或 HTML 字符串。

```text
{
  schema_version: 1, ui_lang: "zh",
  pages: [{
    route: "/zh/guide/start/", title: "Start", source_lang: "en",
    is_fallback: true, nav_order: 0,
    sections: [{ anchor: "setup", path: ["Start", "Setup"], text: "..." }]
  }]
}
```

- 每份索引的 pages 是当前 ui_lang 的全部已发布页面，按该语言导航去重排序，再追加不在导航中的页面（按 route）；不依赖导航才可搜索。过滤 draft 和 search=false，404 永不入数据；index 与 ai 开关互不影响。
- 从最终文档唯一 `<main>` 抽取标题/章节和正文，包含普通代码与真实初始组件文字；排除 nav/footer、`.rd-layout-slot`、脚本/style/template/noscript、复制/演示状态与源文件路径、备用完整文件面板和 UI 控件说明。`<main>` 的首页 tagline/features 及定制页正文必须保留。按 DOM 树遍历，不能正则剥标签或连词粘成一个 token。
- H2/H3 维护 heading path，锚点只能引用最终存在的 id；无 id 的定制页小标题文字并入最近可达的章节，至少落到页面标题对应的根章节，不返回无法到达的独立标题。根章节 anchor=null，链接到页面顶部。重名标题以已有 ID 为准，不重新 slug 或修改岛内 SSR 节点。主文档标题已来自 Page，不从导航标题替代。
- 注册页元数据增加可选 draft/index/search/ai 布尔字段，缺省沿普通页面的默认；lang 保留 P3 定义。三端声明解析一致，最终传入 Page.front；default source 的排除策略约束该逻辑页所有缺译投影，真实译文可自行更严格，但不能让默认草稿通过译文泄露。
- 同一 source 在不同 ui_lang 是不同搜索条目；不得合并为默认语言地址。开关/翻译变化后整个索引由本次图重新构造，不读取旧 dist。

### 2.2 Worker 查询算法

1. 初始化只解析目标语言 JSON，检查 schema/字段/URL，缓存各字段 ASCII 小写副本；只转换 A–Z，保留 UTF-16 原文偏移。不给全部正文预先运行分词器。
2. query 去两端空白，空输入显示提示并清空结果。用 Intl.Segmenter 的 word 词段取非空查询词，isWordLike 为 true 或词段含汉字时视为单词；中文含汉字部分按 zh 词边界，英文按 en。英文只折叠 ASCII A–Z，在词段开头支持前缀，不在驼峰、下划线或数字内部另拆词首。
3. 先用字符串包含筛选候选 title、heading path 和正文，再仅为命中字段按上述语种创建 Segments，以原文 UTF-16 命中位置 `i` 调用 `containing(i)`；要求词段 isWordLike 或含汉字，且 index 等于 `i`。汉字段不依赖引擎的单词资格标志，仍采用原生词段边界；中文查询须等于完整词段，英文查询须为该词段 ASCII 折叠后的前缀。汉字紧邻英文时按 en 的实际词段起点判断，不能用“前字符属于任意 Unicode 字母”拒绝。两种 token 可混查，所有查询词必须在同一章节的 title/path/text 联合字段中命中；不对全量正文预分词。
4. 每词在每字段只记一次命中，不让正文重复次数压过标题。章节排序键依次为标题命中词数、小标题命中词数、正文命中词数、完整查询短语命中、nav_order、章节原顺序；前三项降序，末两项升序。按页面分组，以其最佳章节排序；同分用 route 稳定排序。
5. 返回每页最佳章节和至多 3 个命中标题路径。摘要取实际命中字段的原文窗口；即使只命中标题，摘要也保留该匹配文本。高亮是 UTF-16 `[start,end)` 偏移列表，合并交叠区间；UI 使用文本节点和 `<mark>`，不把 query/正文送入 innerHTML。
6. 每批返回 20 个页面组，更多结果通过本地“显示更多”递增 20；total 是全部匹配页面数，绝不拿展示条数充总数。已取索引后所有查询/翻页零网络。

边界断言固定为：`使用ThemeScope` / `ThemeScope主题` 均可被 `ThemeScope`、`theme` 命中；`heme` 不匹配。`Theme_Scope2` 可被 `theme_s` 前缀命中，`Scope` 不匹配；`foo_ThemeScope`、`_ThemeScope`、`2ThemeScope` 中的 `Theme` 及 `Theme2Scope` 中的 `Scope` 都不算词首。`😀ThemeScope` 仍命中且高亮从 UTF-16 偏移 2 开始。V-2 与 V-11 的目标浏览器使用相同断言，不把某引擎结果当规范保证。

1000 页线性候选扫描属于本计划确定路线。局部迭代用固定 100 页同构缩小语料，正式容量只在阶段收口测；不依赖提前下载、读者停顿或 HTTP 热缓存满足首次查询。

### 2.3 消息协议与失败

```text
UI -> Worker: {type:"init", instance, index_url}
UI -> Worker: {type:"query", instance, id, query, offset}
Worker -> UI: {type:"ready", instance}
Worker -> UI: {type:"results", instance, id, query, total, offset, has_more, groups}
Worker -> UI: {type:"error", instance, id?, code:"load"|"schema"|"query"|"unsupported"}
```

instance 在重试及关闭未就绪会话时递增，id 每次输入递增；UI 同时核对二者，晚到结果不覆盖新 query。bootstrap 导入完成也须校验当前 instance 和 dialog 打开状态，过期导入不得初始化 module。加载期间只保留最新 query，ready 后立即提交，不积压旧输入。组合输入期间不查询，compositionend 执行最新值。

index_url、worker/module URL 由构建输出给定，带部署 base 与指纹；Worker 只允许与页面同源且在该部署 base 下的索引地址，schema 不符或 HTTP 非成功都失败，不把 404 HTML 当空索引。初次网络/初始化及单次查询的终态 watchdog 为 10s（明确运行决策，不是性能预算）；超时/Worker error/messageerror/导入拒绝均转失败，清理计时器并终止已存在的 Worker。module 尚未加载时由 bootstrap 按 §5.1 恢复；module 已加载后的 Worker/JSON 故障复用该 module，新建 instance/Worker 并重取索引，保留输入，再显示当前查询结果。

## 3. 派生文件与兼容

| 对象 | 约束 | 来源/生命周期 | 可见地址 |
| --- | --- | --- | --- |
| 搜索 JSON | 每语言一份、仅公开允许搜索页；schema 1 | 本次最终 HTML/语言图；指纹随字节变化，manifest 同代 | `assets/search.<lang>.<hash>.json` |
| 搜索 module/worker | 同源、外部文件、无 blob/内联脚本；URL 按构建指纹；module 恢复只加 §5.1 的受控参数，路径/字节不变 | CLI 内嵌，生成时写入同一 files map | `assets/search.<hash>.js`、`assets/search-worker.<hash>.js` |
| AI 索引 | 导航/页面唯一，允许 AI 的真实公开内容；缺译语义见 §5.3 | 每次完整重建，无旧代追加 | 根 `llms.txt` 与 `<prefix>/llms.txt` |
| 中文真实内容 | 原文件不复制为英文“同步副本” | `docs/zh/quickstart.md` 与注册中文首页 | `/zh/quickstart/`、`/zh/`（再加部署 base） |

### 3.1 文件事务与路由保护

生成顺序为：公开页面渲染并保留搜索 URL 专用占位 → search::build 从本次候选 HTML 提取、ai::build 从图生成 → 校验所有条目 href/anchor/语言/排除 → 把 SearchBundle 的语言索引/module/worker URL 写回各页 data 属性并加入 files → 原有最终 HTML/资源校验一次 → 生成现有 manifest → 同一 publish 事务。索引占位携带 ui_lang，使根404内各语言模板也取得自己的索引；module/worker占位可共用。占位沿现有 stylesheet 占位的确定替换方式，但只出现在框架拥有的属性，不进入被索引正文；索引字节不依赖自己的 URL，因此没有指纹循环。提取失败/输出路径冲突是构建错误；不发布仅有索引或仅有新页面的混合代。

`SITE_FILES` 与 `GENERATED_PREFIXES` 增加 llms.txt/`assets/search.`/`assets/search-worker.`，且按每语言根验证，作者静态资源和注册路由不得覆盖。保留路径校验应识别文件及其目录形式，例如 `/zh/llms.txt/`；普通输出最终碰撞检查仍保留。回退页由 source front 的 search/ai 策略继承。删除页面、切语言、改开关后旧条目和旧指纹文件在下一成功完整产物中消失。

### 3.2 固定 CLI/库配对

沿 P3 §3.2：先形成产品 R，再形成仅五配对文件的 C，两个真实提交。patch 版本从实施时实际 CLI 版本递增，模板锁来自真实 Git R；不得复制源码或用 file/path/link 冒充公共依赖。库的 SSR/hydrate 两图、冷清单与宏含相同注册元数据。

V-7 记录 DOCS_LOCAL_GIT、DOCS_C、DOCS_INSTALL，并按 P3 §3.2 将浏览器 RUSTIFY_DOCS_BIN 绑定该安装目录。V-9 的 toolchain 组须另显式传入同一 DOCS_LOCAL_GIT/DOCS_C，不读取 RUSTIFY_DOCS_BIN，也不接受默认 candidate() 的合成工作树提交。组内 CLI source、--build-info、cargo install --list 必须共同对应 C，模板框架对应 R；features 与完整包级测试保留开发态来源记录。

本地可用进程级 Git remap 准备/测试固定配对，但 V-12 公共安装环境必须去除 remap，真实读取公共 HTTPS 的完整 C/R。源 C、framework R、CLI version、panic=unwind 与 manifest build_id 必须能相互核对。没有公共可达性证据不改 README 为已发布。

## 4. 集成与前置

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P3 PageLocale/LocaleView、按语种 SSR/404 | 已交付；本地开发前置满足 | [P3 快照](DOCS-SSG-P3.md#恢复快照)为 4/5；[M5 记录](DOCS-SSG-P3.records/M5.md)含 R₂ `820cea0f0ea886606427bfd031e17c93d401aca8`、C₂ `bae5b882d9304a47254d81662b5163981bb0adaf`（0.1.21）与 V-4、V-7–V-9 本地结果；当前 content.rs 实有语言图字段 | 本期消费已实现接口；V-1 核对实际字段/输出并测新增提取，不等 P3 V-10 或 M5 整行完成 | P4 新集成失败按本期修复，不把 P3 历史通过视作 P4 通过 |
| DOM parser/CSP/输出事务 | 已核实足够 | 已锁 dom_query/html5ever、shell::CSP、generation/publish | 本地处理，无新外部依赖 | HTML/输出错误拒绝整代 |
| 官方页面/指南/演示 | 已核实足够（英文基线） | 根 config 12 指南 allowlist、site 的 24 分类/同源演示，P2 记录 | §5.4 加两真实中文页和目标语言导航 | 缺失来源/演示编译失败沿既有诊断 |
| 公共 Git 来源 | 已核实足够（仓库可读）；新 C/R 尚未存在 | repo API public/main；当前尚无本期候选 | 维护者确认推送候选/公开交付时机后公开 C/R；Agent 做真实安装 | 远端缺对象/安装失败阻塞发布，不能回退为本地 remap 证据 |
| GitHub Pages source/environment | 未核实阻塞发布 | 2026-10-15 调查的 Pages API HTTP404；工作流 `enablement:false`，故不会补启用 | 提供方为仓库维护者；在现有仓库设置确认 Actions source 和 github-pages 环境权限；就绪证据为 API 正常响应/设置可见与工作流读取配置成功 | 未就绪不 dispatch，不建设另一托管替代 |
| 原生稳定浏览器 | 当前V-11范围已通过 | Chrome155.0.8059.85、Firefox158、Safari27.0.1原生读者、同组边界/恢复、禁脚本、200%及窄屏已补齐，见M7 | 不重复已通过且未失效的资源检查，继续公开交付 | 本人VoiceOver结论与技术实测分开，手机按D17取消 |
| VoiceOver | 用户本人验收通过（C-5、D18） | 用户确认旁白可听、可见，并明确VoiceOver通过；临时开启后已确认恢复off | 保留本人结论与实际采证范围，不再重复；手机实机按D17取消 | 本项不再阻塞M7，不代填其他桌面任务 |

没有登录、租户、授权 API 或数据库需求，Portal/onebox 与其发布技能不适用于这个 SDK 的静态官方文档站。已有 GitHub 工作流是本仓实际发布入口。

## 5. 核心机制

### 5.1 搜索启动、状态与隔离

shell 为首页/内容/定制/404 都写搜索按钮、禁脚本说明、原生 dialog 和输入/结果/状态骨架。`site.js` 的轻量 bootstrap 一次绑定打开/关闭、输入保存和重试入口：按钮点击或 Cmd/Ctrl+K 时同步 showModal/focus，之后动态 import 指纹 search.js；关闭后回到实际触发元素，快捷键打开则回到打开前的活动元素。module 不可用时的 loading/error 文案、重试和 watchdog 都由 bootstrap 拥有，不能等待失败的 module 绘制恢复入口。

query/input 永远不等待 module/JSON/Worker；加载期间显示独立状态并保留输入。search.js 为自足单模块，无静态模块依赖，导出显式初始化/清理入口，顶层执行不绑定监听、不创建 Worker。bootstrap 仅在当前 instance 且 dialog 仍打开时初始化；清理上一会话后新建同源 module Worker，由 Worker fetch 对应语种 JSON。首次启动、初始化、查询错误都走统一 UI 失败和重试；没有错误文本外泄或第三方网络。演示 trap 不触及 dialog/Worker；切换语言普通导航销毁本页 Worker。

首次导入使用构建给定的原指纹 URL。导入拒绝/超时后 bootstrap 清除其缓存 promise；用户点重试才递增页面内 attempt，用 URL API 给原同源 URL 设置保留参数 `rd_search_attempt=<attempt>`，只变 query，不改部署 base、文件路径、指纹或响应字节。关闭未完成导入后再次打开也使用新的 attempt，避免复用失败/未终结的会话；同次 loading 不并发重复启动。异步回调同时核对捕获的 attempt/instance，旧导入即使晚成功也不得初始化、清空新 promise 或重复绑定。成功 module 缓存在本页，后续 Worker/JSON 重试直接复用；无自动重试循环。此参数兼容目标 Chromium 的同 URL 导入失败行为，不将其宣称为所有引擎或最新版规范的保证。

搜索 UI 的 data 属性分别携带 catalog 文案、当前 ui_lang 和 asset URL，JSON 通过现有转义路径写属性，不加 inline script。P3 根错误页选择 locale 后，搜索使用该语种的数据/文字；不能仍搜默认语言。

### 5.2 交互与高亮

输入是 `role=combobox`，aria-controls 指唯一结果 listbox；aria-expanded/activedescendant 跟随状态。输入保持焦点，上下键选择结果，Enter 普通导航到已校验 href#anchor；Tab 可到关闭/重试/更多按钮。Esc 关闭，原生 dialog 限制焦点在内部。按页面成组的 option 含标题、heading path 和摘要，允许 aria-label 汇总，选中状态可被朗读。

搜索结果通过 document.createTextNode/mark 组装，href 来自同源索引且经过校验；禁用任意协议/跨源值。显示更多保留既有选择，查询变化回到第一页；新的总数写单独 aria-live=polite，错误用 role=alert。无结果与加载失败分别有不同文字和恢复动作。

### 5.3 AI 索引合同

- `llms.txt` 是 UTF-8 文本，H1 站点名、简介，再按当前语言导航分组，项为标题、HTML URL 与单句描述；不在导航的公开页追加一个本语言“其他页面”组，不遗漏允许 AI 的内容页。每逻辑页在该语言索引恰好一项，组内沿导航顺序。
- 描述逐候选 trim，空值/纯空白视为缺失，输出时将内部换行/连续空白收成空格。普通 Markdown 依次取 frontmatter description、已有正文首段摘要；不能让空显式值遮蔽有效摘要。仅 `layout: home` 在两者均缺失时继续取首页 tagline，最后使用来源语言 Catalog 的新增 `ai.home_description` 文案（中文“{site}首页”、英文“Homepage of {site}.”，site 来自配置的 site.name）。第三语言沿 P3 逐键解析/英文回退及缺键诊断；不从 features/动作/任意 HTML 拼接描述，不调用模型。此兜底只用于 AI 条目，不改 Page.description、SEO 或现有合法首页的 check/build 成功语义。用户 2026-10-16 已确认该兼容规则；未选择要求作者补描述的构建错误方案，以保留现有合法输入。
- 允许 AI 的非 Home Markdown 若两项均为空，由 ai::build 返回定位到文件的 `missing-description` error，建议补 description 或正文首段，候选失败并保留旧代；这是对空白显式值绕过现有必填校验的补齐，不为普通页使用首页兜底。ai=false/草稿先按公开策略排除。
- 注册页仍须有非空、去空白后的显式 description 才列入，不能从 custom HTML 自动摘取，也不使用首页兜底。默认/真实译文各自用该版本描述和来源语言 Catalog；缺译沿默认来源并附 source_lang，不用目标界面语言冒充源描述。每个仍可发布且允许 AI 的 Markdown 页（包括无正文/只有首页元数据的 Home）恰好一项且描述非空。
- 普通译文链接其最终 route；缺译链接默认真实 source_route 并在标题/描述处标“未翻译，内容语言 en”等相应本语言文字。不能把回退页面标成真实译文；搜索则仍链接目标界面，二者不可共用 href。
- 有 site.url 则复用 generator 的 origin/base 绝对 URL 算法；无 URL 则输出带 base 的根相对路径，复用既有 site-url 警告，不增加一份重复警告。所有链接最终可解析，不输出 file URI 或仓库文件地址替代页面。
- draft、ai=false、404、未提供描述的注册页不出现。文本格式转义标题的 `[]`、描述换行与链接字符，避免作者内容制造额外项或错误链接；不发布任何 Markdown 源、逐页 md 或 llms-full.txt。

### 5.4 官方双语站点

根配置增加 P3 的 en/zh 声明、中文导航组/顶栏文字；include 白名单只新增 `zh/quickstart.md`。`site!` 增加 `HomeZh { route:"/", lang:"zh", title/description... }`，中文首页函数在现有 pages.rs 内，保持与英文首页相同信息结构；真实中文文案从已存在产品事实翻译，不复制英文 Markdown 作为中文。

`docs/zh/quickstart.md` 是所需中文快速开始的唯一真实源，命令/参数遵循当前 SDK；相对图片、指南、仓库链接按它的真实路径书写，交给 P3 投影。已有英文 12 指南继续直接引用仓库文件，24 分类名称/正文缺译时有明确回退；中文界面导航、上一下一页和搜索仍停在 zh。根配置不扩 allowlist 到计划/报告/验证/PRD。

P2 的全部 DOM 演示、源码与 GPU 外链沿用，站点不加载 Makepad。官方产物数按实际图统计，不把旧 37 页写成新站固定页数；构建/check 有缺译 warning 合法，发布门要求零 error，不使用 deny-warnings 把正常缺译挡住。

## 6. 前端与视觉合同

采用既有 Read 世界：文档列保持主位，系统字体、原主题 token、轻边界和一处蓝色。新增搜索不是重设计站点。

- 顶栏按钮在语言/主题旁显示“搜索”及桌面快捷键提示，移动端隐藏冗长提示但保留可读名称；控制区域可换行。无需任何 wasm 就能打开。
- dialog 在桌面宽不超过阅读列，顶栏为显式 label、输入、关闭；其下是状态与结果列表。移动端留安全边距，max-height 跟随可视 viewport，列表独立滚动，键盘弹出时输入和关闭仍可达。内容不做卡片网格。
- 状态固定为 closed、idle、loading、results、empty、error。idle 提示输入词；loading 告知正在准备搜索且输入可编辑；empty 给换词建议；error 显示暂不可用及重试。没有后台索引时不显示 empty。
- 缺译结果在标题旁显示目标语言 badge，结果正文标 source_lang，控件/数量状态沿 P3 Catalog。新增 `search.*` 键包括入口、输入、关闭、提示、加载、空结果/建议、失败、重试、结果数 `{n}`、更多、缺译说明和禁脚本说明。第三语言缺键按 P3 同一机制报告，不新增第二套 fallback。
- 高亮 mark 使用现有 primary 的低比例底色和正常文字色，选中 option 用边界/背景与 aria-selected 双重表达；两主题达到 PRD AA，焦点不依赖仅颜色。
- 默认无装饰动效，reduced-motion 不平滑滚动。禁脚本搜索按钮保留 disabled 与 noscript 解释；正文、导航、主题 CSS 初帧及演示静态源码仍可读。dialog 不增加第二个 H1。
- V-8 同部署批量覆盖 desktop/mobile、语言/缺译/404/no-script/loading/failure/双主题/320px/200%；记录 console、CSP、失败请求与截图。一次批量缺陷修正后至多再确认一次；实际规则同步至现有设计文档和 brief。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD 完整测量合同 | 1000条当前语种数据；冷查询 ≤1s，热查询 ≤100ms；输入立即可用 | 超预算不记通过；按瓶颈修正后重测相应项 | 按需 Worker、候选取词边界、20组结果、输入不等待 | V-2、V-10 |
| WORK-N3 | PRD CSP/矩阵、D17 | 完整 header 与 meta 零违规，无外源；桌面全稳定浏览器，手机实机本次不验 | 其余必需环境不足保留发布门，资源错误可重试 | 同源 module Worker/fetch、无 blob、无 SPA rewrite | V-8、V-11、V-12 |
| WORK-N5 | PRD AA 与 VoiceOver | 结果总数/当前项朗读，全部键盘操作；严重/重大自动化违规0 | 不以 aria 属性推导真实朗读成功 | 原生 dialog、live region、明确焦点/状态 | V-8、V-11 |
| WORK-N4 | 既有内容站/配对工具链 | 无 Makepad，无作者 Node/浏览器依赖；离线 check/build | 缺依赖给现有明确错误，禁绕过锁 | 嵌入静态搜索脚本，无新依赖 | V-7、V-9 |
| WORK-N6 | generation/publish | 搜索/AI 与 HTML 同代，确定字节，失败保留旧产物 | 候选失败，不部分覆盖 | map排序、fingerprint、同一事务 | V-5、V-9 |
| WORK-N7 | PRD 产物/AI 范围 | 无私密源/草稿/机器路径/查询上传/HTML注入 | 校验或 UI error；不放宽 CSP | 生产图过滤、文本节点、URL限制、无日志/存储查询 | V-1、V-5、V-8、V-9 |

继承的 WORK-N1 内容/岛预算不变：HTML/CSS/基础脚本 gzip -9 合计 ≤100KB（搜索/演示按需资源排除）；移动 LCP≤2.5s、CLS≤0.1；FCP→全部演示可输入≤1s，水合位移0。搜索入口 bootstrap 计入基础脚本，不能把默认下载的数据/代码改名为“按需”规避。

Worker 的故障只在页面用状态/重试呈现；CLI/index 生成错误进入原诊断 JSON 和终端总结。发布观察沿 workflow artifact、identity/manifest 和公共 HTTP；不新增运行服务、告警系统或读者跟踪。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| module/Worker/JSON 请求失败、超时或 schema 错配 | 当前搜索会话 | 无用户数据持久化 | 输入保留，暂不可用与重试，正文不受影响 | error/watchdog/HTTP | module 未加载由 bootstrap 清 promise、换受控 attempt URL；已加载则复用 module、新建 instance/Worker 重取同代 JSON；见 §5.1 | V-4、V-8 |
| 快速输入/重试/关闭产生晚响应 | 当前搜索结果 | 错结果风险 | 仅当前 id/instance 可改变结果 | 协议/输入逆序测试 | 丢弃过期消息；关闭终止未就绪会话 | V-2、V-4 |
| 提取或过滤错误 | 本次公开索引 | 草稿/源数据泄露风险 | 不应出现公开产物 | 硬性 allowlist、sentinel与完整文件检查 | 拒绝候选；线上出现则回退完整上一 C | V-1、V-5、V-9、V-12 |
| 指纹文件/页面代不一致 | 受影响站点访问 | 搜索不可用，无源丢失 | 清晰重试，可继续阅读 | manifest/全页资源核对 | 完整目录部署/回滚，禁止零散补文件 | V-7、V-12 |
| Pages source/权限/候选不可达 | 此次发布 | 当前线上不变 | 发布失败或未启动 | API/工作流结论 | 维护者修正现有配置后重跑同一候选 | V-12 |
| 线上新版本 CSP/深404/索引异常 | 官方站 | 新内容暂时不可用 | 读者报错/找不到页 | 公共全页与交互走查 | 同 workflow 指定 last-good C，恢复完整 artifact | V-12 |

发布顺序：

1. M6 本地能力收口，形成固定 R/C，已有检查结果按实际适用范围引用；必需环境检查 V-11 全部补齐。尚缺任何必需项时不能进入发布。
2. 保留具体 last-good C 和其 artifact/manifest。首次官方上线没有 last-good 时，先指定一个已通过本期出口的基线 C0 作为首次发布，再以仅公开说明/配对身份变化的 C1 验证后续完整发布与回滚 C0；两个候选分别有可达源码与实际 artifact，不把未发布的 P1 版本当已存在的官方回滚目标。
3. 只在已有授权覆盖公开推送/部署时执行；否则此时向用户呈现已经可审查的 C/R、产物、检查、Pages 配置缺口、发布范围和回滚 C 后请求最终授权。当前写计划不构成未来发布授权。
4. 使用 `gh workflow run docs-pages.yml --ref main -f candidate_sha="$DOCS_C"`（DOCS_C 从已核实完整候选 SHA 赋值）；跟踪该 run，要求 build/deploy 成功，artifact/build_id 与公共线上 manifest 一致。
5. V-12 浏览全部语言页、资源、搜索/llms/深404与 meta CSP。失败执行同一工作流的 last-good C；成功也执行一次约定回滚演练，再按用户批准的目标 C 恢复并验证身份。

若没有任何官方 last-good 且不能授权两次候选发布，保留首次发布回退方案待用户决定，不伪造回滚通过。不得擅自开启 Pages、变更默认分支或扩大权限。

## 9. 验证

### 9.1 检查定义

新增文件/命令是本期交付物，非当前已有入口。局部检查与阶段收口分别列出；每项一个执行时点。同环境 API/HTTP/UI 合并，浏览器项目逐个串行。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F9/AC-4/5、WORK-N7；真实语言图提取、定制页/草稿/开关/词间空白/锚点/无路径泄露 | `mbx test --locked -p rustify-docs --test search extraction`；新增注册布尔元数据测试用 `mbx test --locked -p rustify-docs-syntax`、`mbx test --locked -p rustify-docs-macros`；对照 P3 本地记录和实际字段，非 fixture 自证前置 | M1 局部，解除 M2/M4 |
| V-2 | WORK-F9/AC-1/2/4、WORK-N1 缩小基准；中文整词/英文词首/权重/分组/更多/逆序响应；含 §2.2 的中英紧邻、大小写/前缀、下划线/数字/词中反例及 emoji 偏移 | `npx playwright test -c playwright.docs.config.ts --project=docs-search-subpath search-worker.spec.ts --grep matching`；Worker 协议定向测试，固定 100 页局部数据；混排标题须保有标题优先级，非完整 UI 走查 | M2 局部 |
| V-3 | WORK-F9/AC-3/6、WORK-N4；每页入口静态语义、404 当前语言、禁脚本解释、按需 URL | `mbx test --locked -p rustify-docs --lib shell::`；`mbx test --locked -p rustify-docs --test search shell` | M3 局部 |
| V-4 | WORK-F9/AC-7；load/schema/timeout/worker异常、保留query/重试/关闭/晚消息；单列首次 module HTTP/网络失败→恢复→同页点击重试，确有新 URL 加载及当前结果，原输入保留、无重复监听/CSP违规；超时/关闭后的旧导入晚成功不得启动旧会话 | 同 M2 project，定向 `search-worker.spec.ts --grep recovery`；shell 对话框状态逻辑定向用 `search.spec.ts --grep recovery`，核对 bootstrap 在 module 不可用时仍有失败/重试入口；只测新增状态，不做全页回归 | M3 局部 |
| V-5 | WORK-F12 全 AC、WORK-N6/N7；导航去重/非导航补齐、缺译/定制描述/排除/无URL/源不发布；无描述/无首段空 Home、仅首页元数据、空/纯空白值、显式描述优先及各级兜底，中英/缺译的来源语言，所有允许 AI 的合法内容页条目唯一且描述非空 | `mbx test --locked -p rustify-docs --test ai`；`mbx test --locked -p rustify-docs-cli --test search ai`；包含坏输入旧dist保护、两构建字节一致；空首页 check/build 仍成功，无描述注册页仍排除，允许 AI 的非 Home 空白描述且无首段得到明确诊断/旧代保护 | M4 局部 |
| V-6 | WORK-F16/AC-1/2/4；中文两真实页、其他回退、原12指南/24演示与allowlist | `mbx test --locked -p rustify-docs-cli --test search official`；`mbx test --locked --manifest-path site/Cargo.toml --features ssr --lib`；官方声明/内容局部检查 | M5 局部 |
| V-7 | WORK-N4、C-3；R/C 五文件边界、真实固定本地安装、未改模板与官方 Git 消费者，产物身份 | §3.2；本地 `mbx install --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C" --locked --profile docs-cli --root "$DOCS_INSTALL" rustify-docs-cli`，变量均取本次记录的绝对路径/完整SHA；用安装二进制执行 new/check/build/preview；已有 prepare 工具改两消费文件的保护门 | M6 本地能力阶段收口 |
| V-8 | WORK-F9 全 AC、F12、F16/AC-4、N3/AC-2/5、N5 全页面；完整根/子路径/meta、语言/404/no-script、键盘/a11y/失败/双主题/reflow | 新 `docs-search-root/subpath/meta` 分别端口4204/4205/4206，每次 `npx playwright test -c playwright.docs.config.ts --project=docs-search-root search.spec.ts` 再换 project；官方原有 `official-root/subpath/meta` 分别串行跑 `official.spec.ts` | M6 本地能力阶段收口 |
| V-9 | WORK-N4/N6/N7、跨期回归；完整 tests/lint/原 docs 浏览器、离线作者环境及源泄露/可复现/失败安全 | Docs/CLI 分别 `mbx test --locked -p rustify-docs`、`mbx test --locked -p rustify-docs-cli`；分别 clippy all-targets -D warnings；fmt；`mbx xtask docs-verify --group features`；`mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"`（沿 V-7 已记录变量）；原 docs/components project 每个一次，复用未变独立图证据，变化即重跑对应图 | M6 本地能力阶段收口 |
| V-10 | WORK-N1/AC-1–5 终验；P2正式性能欠证与原 B 不变 | §9.2；新 `mbx xtask docs-verify --group search --report target/docs-ssg/p4-search.json`；继承 `--group performance` 及 P2 的 A/B 最大页/五冷样本合同，本期候选完整执行一次 | M6 本地能力阶段收口 |
| V-11 | WORK-N3/AC-4/6、WORK-N5/AC-2/4；桌面全部稳定浏览器、VoiceOver、原生200%；D17取消手机实机 | §9.3 原生资源；读者从真实导航进入含搜索/语言/演示的已构建站点，完整任务；必需资源缺失记未验证，不由引擎报告代填 | M7 发布前独立风险门；手机不再作为本次阻塞 |
| V-12 | WORK-F16/AC-3、WORK-F12/AC-1–4、WORK-F4/AC-4、C-2/C-6；真实公共Git、GitHub文件预览、Pages全页/资源/搜索/llms/深404/身份及回滚 | §8；先 `gh api repos/rockie/rustify-ui/pages`；公共 HTTPS 安装完整 C/R；新 `python3 scripts/docs-pages-verify.py --url https://rockie.github.io/rustify-ui/ --candidate-sha "$DOCS_C"`；同产物真实浏览器线上任务，跟踪完整 workflow run | M7 公开交付阶段收口；不重复 M6 全量 |

所有新增测试需先确认能因对应缺失行为失败，再实现。V-9 的原浏览器套件不重跑 V-8 新 spec；Worker 局部用例在收口只作为受影响回归引用，不把同一耗时走查排两次。

V-9 的完整包测试覆盖 artifacts 组的 Docs generate、CLI artifacts/new_build，以及 reliability 组的 Docs lib、CLI bin/reliability/faults/external；在记录逐 targets 映射结果，不再执行这两个重复组。包测试若中途退出，只补跑未执行 targets 和受修复影响项，失败或 ignored 不能计作通过。features 与 toolchain 含独立的图/构建和安装/隔离行为，仍分别执行；开发态测试与固定 C 实装证据分开登记。

### 9.2 数据与测量合同

`docs_corpus.rs` 新增 `--p4-search` 模式：`mbx xtask docs-corpus --p4-search --out target/docs-ssg/p4-search`，使用已有 b-profile 的正文/图片分布与固定种子。原500页 B 和 --p2 模式的字节不改。

仅此模式支持 `--pages 100` 的局部版本，默认1000；每语言索引规模等于 pages，中文真实/回退各半。构造一个真实注册定制页包含在总数内，其余为 Markdown；沿 --p2 已有模板 Cargo/锁/SSR helper 的拷贝方式生成可编译消费站，定制页没有演示岛。准备/编译不纳入读者端查询计时。

`serve.sh` 新 search 分支默认生成100页局部版本，三项目 testMatch 只包含 search-worker/search.spec；旧 docs 项目显式排除新增 i18n/search 专项，避免在旧 fixture 上误运行。正式 group 使用独立 `docs-search-perf` 项目（4210端口、testMatch仅search-perf.spec.ts、固定1000页），设置 `RUSTIFY_TIER=evidence` 并验证报告确实包含每项5样本，零测试或缺样本必须失败。数据/构建分别在 target 下隔离，沿现有 preview/static-host 服务，不另起基础设施。

正式语料按“单份当前语言索引 1000 个可搜索页面”计，中文索引含 500 真实译文、500 默认英文回退，英文索引1000默认页；内容/定制页均包含在1000内。另加草稿、search=false、ai=false、缺描述 custom 的哨兵，不靠排除后不足1000的语料测容量。至少一页标题为“主题切换”，另一页标题含 ThemeScope；正文重复干扰页与不存在查询由固定数据生成。这个规模拆分是本计划的造数决策，避免用两份各500的索引充当1000页搜索。

性能由页面内 input 事件 `performance.now()` 起，到对应 id 的结果 DOM 已更新后的两个 requestAnimationFrame 回调间确认绘制后计时；测试同时断言该 id 的可见结果，不能只计 Worker 回包或首次 rAF。索引冷态从打开后立即输入计，含 module/Worker 创建、fetch剩余、JSON准备、查询/消息与实际绘制。输入获焦也以页面内事件/activeElement 和输入可达断言记录，不能用 Playwright 操作耗时。

参考机、无CPU/网络节流；每个目标语种、最大数据页/404入口分别5个新浏览器上下文冷 HTTP/Service Worker 缓存样本取中位数；同会话热索引下，对同一固定查询序列的每种输入分别5次中位数，不能把慢输入平均掉。加载时立即输入中英词、前缀、无结果、缺译与定制页词；正式结果必须达到首查1s/热查100ms。故障重试的10s watchdog另测，不替代1s预算。

继承性能走原 A/B 各页面类型最大首次体积候选，冷5次中位数；移动使用 PRD 固定 Chromium153、375px/CPU4倍/latency150ms/下行209715B/s/上行96000B/s；本机记录实际版本，不能换品牌浏览器测量合同。P2 演示仍从 FCP 计，不改为资源加载完成。正式指标失败或未测不记跨期 NFR 通过。

### 9.3 真实设备、辅助技术与线上验收

用户本轮明确“手机的测试不做，其他的继续”，按PRD D17取消本次iPhone/Safari与Android/Chrome手机实机测试及其发布前门槛，不把旧缺证改记通过；已有桌面窄屏、原生200%与实验室性能证据保留。M7继续使用现有桌面原生会话和VoiceOver；不能自行购买测试服务，浏览器安装/重启按已有明确授权执行。Agent记录浏览器/OS、实际CSS视口、版本日期与阅读/导航/搜索、中文/缺译、主题、源码复制/演示；桌面最新稳定Chrome/Firefox/Safari仍覆盖全部读者能力。Playwright Firefox/WebKit只计引擎回归，必须补原生稳定版证据。

V-11 的各目标浏览器还执行 §2.2 的同组词首断言和 §5.1 的首次 module 失败后同页恢复，防止只在固定性能引擎成立；记录实际结果，不以 Node/单引擎探针代填。

VoiceOver+Safari的结果数量、当前选择、copy成功/失败朗读目标保留；本轮用户按D18明确确认VoiceOver通过，本项记录为本人验收通过，不再重复。Agent未取得的完整逐项音频/转录不伪写为已采集，不拿DOM aria属性代替实际朗读。原生200%缩放与320/375宽走查仍分开记录。其余必需检查未齐时M7不能记完成或公开发布通过；用户改变验收要求须记录明确决定并同步，不能默默豁免。

线上公共 HTTP 全部页面与资源遍历由脚本执行，搜索/切语种/深404/CSP/同源请求由真实浏览器操作；原 GitHub组件围栏可识别占位另看真实源文件预览。现有 manifest 只有 cli_version/build_id/文件列表，没有 C/R 字段，本期不为此改 schema：线上 manifest/build_id、索引指纹与每文件字节必须等于该 run artifact，再由该 run 的 identity.json/prepare receipt 关联 C/R。不能只凭 workflow 绿色或首页200判断上线通过。

## 10. 里程碑与验收安排

七个里程碑，M1–M6 构成本地完整能力阶段，M7 为独立公开交付阶段；环境风险门 V-11 只在 M7 发布前执行，不阻塞本地开发。主实施者单写 content/generation/shell/labels 与配置；纯语料和 AI 模块可在 M1 契约稳定后委派，但共享生成器由主实施者集成，浏览器/输出/端口串行。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 可检查的当前语种公开语料 | C-1 的 P3 本地图/渲染成果及记录已交付（§4）；不等设备验收 | ★search.rs/测试，注册布尔元数据三端、content保留路径、最终main提取；复用dom_query/现有 Page.front 与 P3 PageLocale，按 §2.1 固定 JSON | V-1；回写「实施进度」 |
| M2 | 本地搜索匹配与排序 | V-1 的公开数据/URL合同 | ★search-worker.js、★Worker协议测试、docs_corpus 的缩小数据与serve/project；按 §2.2/§2.3 实现，沿现有指纹静态资产生成 | V-2；回写「实施进度」 |
| M3 | 所有页面可用的搜索与恢复 | V-2 Worker消息稳定；P3 Catalog/shell/404可用 | ★search.js，site.js入口、shell/labels/docs.css/classes；§5/§6 的语义、焦点、状态；复用当前copy状态和details主题，不依赖岛 | V-3、V-4；回写「实施进度」 |
| M4 | 每语言 AI 发现与同代输出 | V-1 页面图/公开策略与generator输出；与 M2/M3 无行为依赖 | ★ai.rs、generation/files保留、★AI测试；按 §5.3；复用 sitemap 地址/输出事务。可独立实现，generator单人整合 | V-5；回写「实施进度」 |
| M5 | 官方双语全能力候选 | P3语言渲染、M3搜索与M4 AI已本地可用 | 根config、site/lib/pages、★docs/zh/quickstart；§5.4，原12指南/24演示同源；按已有中文产品词汇翻译两页，无全站复制 | V-6；回写「实施进度」 |
| M6 | 本地能力阶段收口与固定候选 | V-1–V-6通过；1000页正式语料和官方候选齐备 | §3.2固定R/C；新增search性能group与正式语料；统一部署合并新旧浏览器/离线/性能/产物检查；同步作者与设计文档，主实施者集成 | V-7–V-10；本地出口完整，未把公开交付/设备计作通过；回写「实施进度」 |
| M7 | 原生风险门、公开交付与回滚 | M6本地出口；C-5桌面/朗读资源、Pages环境、公共候选与有效发布授权 | §8/§9.3；复用docs-pages/prepare，补verify脚本；真实公共安装/源预览、桌面稳定浏览器/VoiceOver、完整Pages发布/回滚；手机实机按D17取消；主实施者独占发布 | V-11先通过，再V-12；其余必需资源/授权不足保留阻塞，不改写前期完成；回写「实施进度」 |

提交点：M6 阶段收口提交（产品 R、五文件 C、验收及进度分别按配对/顶部合同办理）；M7 全部里程碑完成后的最终提交必有，回填各里程碑 SHA 后再提交计划回填。发布前补证不重复 M6 已通过且未失效的全量检查；若为修复改变公共入口/依赖/构建则只重跑受影响项并重新固定候选。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| C-1：P3 必要本地成果已交付 | 真实语言图、回退/目录/404 可复用；P3 整期设备终验仍未完成 | §4 固定 R₂/C₂ 与 M5 本地记录；P4 按 V-1 验证新增提取/集成，不重复计入历史验收 | M1 使用时 | 否；本地开发前置已满足 |
| C-5：桌面验收与本人结论 | D17取消手机实机，D18确认VoiceOver通过；桌面矩阵仍须补齐 | Chrome更新重启已授权，Agent继续其余V-11；本人结论与技术实测分开记录 | M7 发布前 | 其余必需项未齐仍阻塞M7；手机与VoiceOver不再阻塞 |
| Pages API 404/source 未核实 | 实际发布可能没有可用目标 | 维护者确认现有仓库 Pages Actions source/environment；API与configure-pages成功证明 | M7 dispatch前 | 不阻塞本地；阻塞发布 |
| 公共候选/推送与部署授权 | Git HTTPS 安装和真实线上/回滚 | 已有授权足够即推进；否则先完成可审查候选、产物/验证再询问最后发布动作 | M7 外部写入前 | 发布依赖；不视为已获授权 |
| 浏览器分词与冷态全成本 | 查询语义/性能不能仅凭探针推导 | 实施者先 V-2 局部验证、V-10同候选正式门；若路线不成立修订对应设计 | M2/M6 | 否；已定路线、可验证技术风险 |
| P2 欠证与首次回滚基线 | 不能冒称旧官方站或全NFR已验 | V-10按原合同补正式性能；V-11原生；V-12真实源预览/实际发布基线 | M6/M7对应门 | 阻塞相应终验，不阻塞其他开发 |

- 最终状态：**Ready**。
- 定级理由：P3 的必要本地语言图、渲染与固定配对已有代码/记录，M1 前置满足；搜索词首边界、module 加载前恢复、Home 描述兼容规则及固定 C 验证来源均已明确，可以开始本地实施。桌面稳定浏览器/VoiceOver、Pages和公共候选授权仍是M7出口依赖，手机实机按D17取消；必需项未齐不得发布或记M7完成，不回流阻塞M1–M6。Ready不表示P4已实现或已通过验收。
- 待用户拍板的产品问题：无。首次发布具体候选/回滚目标与授权在 M7 以实际可审查产物提出；不在规划时要求批准空白发布。

## 12. 已知坑与历史教训

- [P2原 M6](../validation/docs-ssg-p2/M6.md)与 PRD D16 已明确不以公开来源/缺设备阻塞后续本地开发；本计划保留 M6/M7 分阶段门，不能把它们合成“先发布再开发”。
- `docs-pages.yml` 明确不会启用 Pages；仓库admin权限与工作流存在都不等于线上托管可用。本期 API404保留为缺证，不编写已发布结论。
- `scripts/docs-pages-prepare.py` 只允许两消费文件投影；新译文、schema/索引脚本不得在部署时偷偷替换。实际线上与 artifact身份都必须核对。
- P1性能固定引擎/节流合同与稳定浏览器兼容是两种证据；原Firefox延期在本期恢复。本次手机实机按D17不再执行，已有模拟/性能证据仍不改称手机实机通过。
- `site.js` 与 DOM 岛共享页面但不共享 wasm故障域；搜索不可放进演示同一实例，否则一次 trap 会破坏 WORK-F9 的恢复路径。
