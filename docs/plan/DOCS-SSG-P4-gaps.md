# DOCS-SSG-P4-gaps · Rustify UI · 官方指南一句话描述与 P4 结论文档同步（DOM · 内容提交 + CLI 补丁配对发布）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-20 · `4894d4ccc7c462a344489748553615a352031192` · dirty。未提交项只在 `.xgent-ai/` 和 `docs/plan/` 下（SDLC、评审报告、UI 采集），产品源码、各指南、根配置与 HEAD 一致；以下事实基于当前工作树。线上为 C1 `1146b7d8febdf53ca1bc1a4c01946604e7625a95` / R0 `0fd34edaffda1926b38ea64c023b2fe1fc5115c5`，CLI 0.1.31，build `9e8ea80d57750509`。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P4（第 4 期）**。本计划补齐[原计划 DOCS-SSG-P4](DOCS-SSG-P4.md)（7/7 已完成）；增量来源是[差异报告](DOCS-SSG-P4.prd-dev-gaps.md)的 G-01、G-02 和用户决定 D-01。
>
> 本期交付：**12 份官方英文指南各有一句说明主题的显式描述，en 与 zh 两份 llms.txt 和页面 meta 描述随之更新；P4 的浏览器、手机与 VoiceOver 结论在作者参考和设计文档中与 PRD D17/D18 一致；以内容提交 + CLI 0.1.32 补丁配对 C2 经既有 Pages 工作流上线，随后把 C2 的安装与 C1 回滚说明推送到公共 `main`。**
> **开发范围：`docs/*.md` 中 12 份指南的 frontmatter、`docs/docs-ssg.md`、`DESIGN.docs-ssg.md`、`README.md` 的发布说明，CLI 的版本配对文件，以及新增的两处测试断言；依据见 §0。**
> 本期独特职责：只改官方内容与说明，并经原 P4 的 R/C 和发布流程把一次纯内容变更安全地发布出去。
> **顶层排除：不改框架的描述选取语义（`ai.rs` 的 `description()`），不改 PRD 措辞，不改 CLI 帮助里的回滚提示，不补做手机实机，不重做原 P4 已完成的检查。**

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

- 最近更新：2026-10-20T16:30+08:00
- 当前进度：3/3 个里程碑完成
- 当前状态：全部完成；C2 `88566ba1`（CLI 0.1.32 / R0）经 run 39388531674 上线，说明提交 `c6d49c97` 已在公共 `main`
- 最近完成：M3 · 授权发布、V-6–V-10 通过，last-good 为 C1 0.1.31
- 下一步：代码评审——由用户以本计划路径为参数运行 review-code
- 当前阻塞：无
- 代码基线：公共 `main` 为 `c6d49c97`；本地其上只有本次计划回填提交（未推送）；SDLC 台账与他人评审文件仍未提交

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-20T15:16+08:00 | 12 份指南加一句话描述，docs-ssg 与 DESIGN 的 P4 结论按 D17/D18 同步；V-1–V-3 通过，V-5 对线上 C1 红 | [M1 记录](DOCS-SSG-P4-gaps.records/M1.md) | 53087eb7 |
| M2 | 已完成 | 2026-10-20T15:40+08:00 | C2 只改配对文件（0.1.32，R0 不变）；实装、工具链组、官方 0 error、97 文件预检、与本机 C1 比对只有允许的 27 个变化，official-subpath 56/56 | [M2 记录](DOCS-SSG-P4-gaps.records/M2.md) | 88566ba1 |
| M3 | 已完成 | 2026-10-20T16:30+08:00 | 授权后推送并以 C2 发布（run 39388531674，97 文件同 run 核验、线上 4/4、公开安装 0.1.32）；说明提交推送后公共读回通过 | [M3 记录](DOCS-SSG-P4-gaps.records/M3.md) | c6d49c97 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

本计划只规划增量。原 P4 的需求在[原计划 §0.1](DOCS-SSG-P4.md#01-需求与约束账本) 已终验，这里只登记本次触及的部分。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F12 | 功能（行为补齐） | PRD WORK-F12 行为 L459、§1.4 L90、§2.1 L110；报告 G-02 | 每页一句话描述，让 AI 工具知道各页讲什么；官方 12 份指南目前用整段首段，其中 2 条与主题无关 | §5.1；V-1、V-5、V-8 | 已确认（D-01） |
| WORK-F16/AC-4 | 功能（官方站） | PRD L576；G5 | 官方站中英各一份 llms.txt，内容质量随上一行改善；首页与快速开始不变 | §5.1、§8；V-4、V-7、V-8 | 已确认 |
| WORK-N3/AC-4、AC-6 | 兼容（文档同步） | PRD L676、L678、D17；报告 G-01 | 作者参考仍写“P4 retains the full browser matrix”，要改成 D17 后的实际范围 | §5.2；V-3 | 已确认 |
| WORK-N5/AC-4 | 可访问性（文档同步） | PRD L716、D18；报告 G-01 | 设计文档仍把手机、VoiceOver 等列为原 P4 发布阶段尚待通过的门槛，要改成本人验收与原 P4 发布记录的实际结论 | §5.2；V-3 | 已确认 |
| D-01 | 用户决定 | [报告 §4](DOCS-SSG-P4.prd-dev-gaps.md#4-triage-与决策账本)，用户 2026-10-20 确认 | 在指南 frontmatter 补写显式描述；不改框架语义，不改 PRD 措辞 | §0.2 D2；§5.1 | 已确认 |
| C-1 | 约束 | [原计划 §3.2](DOCS-SSG-P4.md#32-固定-cli库配对)、[P3 §3.2](DOCS-SSG-P3.md#32-固定交付与回退) | 发布用固定 C：C 只改五个配对文件；版本从实际 CLI 版本递增；C 之后的文档提交才写入已存在的 C 身份，不伪造自引用 | §5.3；V-4 | 已确认 |
| C-2 | 约束 | [原计划 §8](DOCS-SSG-P4.md#8-失败模式发布与回滚) 第 3 步 | 公开推送和 Pages 发布须有覆盖本次范围的用户授权，范围包括发布后说明提交的推送（§8 第 2 步）；写计划不构成发布授权 | §8；M3、V-10 | 已确认 |
| C-3 | 约束 | `CLAUDE.md` 工具链、浏览器测试、提交约定 | 构建经 mbx；Playwright 一次只跑一个 project，并且须带 `--project`；直接提交到 `main`，不开分支 | §9、§10 | 已确认 |
| C-4 | 约束 | PRD WORK-F16 “单一来源 … 在 GitHub 上仍然可读”（L561）、D9 | 指南仍是唯一来源，加了 frontmatter 后在 GitHub 预览中正文仍可读 | §5.1；V-6 | 已确认 |
| A-1 | 假设 | GitHub 对 Markdown 开头的 YAML frontmatter 渲染为表格、正文照常显示 | 加 frontmatter 不影响 GitHub 可读性 | M3 用 V-6 实看三份指南；不成立时停在 M3 之前回到 D2 重新决定 | 开放（不阻塞） |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 发布路线 | 先做内容提交（只改内容、文档和测试），再做补丁配对 C2：CLI 0.1.31 → 0.1.32，`framework-revision.txt` 保持 R0，模板锁不变 | 不产生新的框架 R，岛 Wasm 与搜索资产不应变化；C2 的变更文件是五个配对文件的子集 | ADR-1；C1 先例 [candidate-boundary.json](../validation/docs-ssg-p4/m7/release-20261019-c31/candidate-boundary.json)（“Retain the existing verified product R0”） |
| D2 | 描述放在哪里 | 各指南开头的 frontmatter `description`，title 仍取 H1 | 框架既有契约：有显式描述就用，没有才取首段并警告（[docs-ssg.md L461](../docs-ssg.md)）；meta、og 描述同源更新 | D-01；`ai.rs:27` |
| D3 | `docs/components.md` | 直接在文件开头加 frontmatter，生成器不改 | `xtask catalog` 只重写两个 `<!-- catalogue -->` 标记之间的表，保留其上的内容（`xtask/src/catalog.rs` 的 `run`/`fences`）；`rustify-components` 的同步测试只检查文档是否包含该表 | 已读代码 |
| D4 | 回滚演练 | 不做演练；只在线上核验失败时用同一工作流回退到 last-good C1 | 发布与回滚机制已在原 P4 发布阶段验证过四轮；本次只变内容和 CLI 版本 | 原计划 §8；[原 P4 发布记录](DOCS-SSG-P4.records/M7.md)的四轮结果 |
| D5 | 已有证据的复用范围 | 框架库、CLI 逻辑、搜索和岛资产都不改，沿用原 P4 的搜索性能、原生浏览器、岛和 SDK 证据；用 V-4 的本机产物比对（本机 C2 对同环境的本机 C1 归档）证明这些字节不变，比对超出预期才重跑对应检查 | 只重跑官方内容、配对和发布检查 | 实施者定位「验证节奏」；§9 |

### 0.3 ADR-lite

#### ADR-1：纯内容变更以补丁配对 C2 发布

- 状态：Accepted。
- 背景与驱动：Pages 工作流只接受“完整 C”。它安装该提交的 CLI，要求 `--build-info` 的 source 等于候选，并从该提交读取官方内容（`.github/workflows/docs-pages.yml`、`scripts/docs-pages-prepare.py` 的 `prepare`）。原计划要求 C、R、CLI 版本、build_id 可以互相核对（C-1）。
- 备选：
  - A：内容提交 + C2，CLI 补丁版本 0.1.32，框架仍为 R0。
  - B：把内容提交本身当候选，CLI 版本仍为 0.1.31。工作流技术上能接受，但两个不同提交都是 0.1.31，版本与 C 无法一一对应，README 的“0.1.31 → C1”会变成歧义。淘汰。
  - C：形成新的框架 R1 再配对。没有框架代码变化，换框架修订会让模板锁的 6 个框架来源和岛 Wasm 无谓地变化，扩大验证面。淘汰。
- 决策：A。
- 正面后果：读者产物只有 23 个指南页（12 个 en、11 个 zh 回退）的描述元数据与“最后更新”日期、`sitemap.xml` 中 12 条指南的 `lastmod`、两份 llms.txt 和 manifest 会变；身份链完整；回滚目标就是现在的 C1。
- 负面/中性后果：多一个 CLI 版本（0.1.32），公开安装说明要更新；CLI 二进制因版本字符串不同而变化，要在 M2 实装核对。
- 重新评估触发：发布时同时要带上框架或 CLI 代码修改（例如代码复审 F-01），或工作流的身份契约改变。

### 0.4 职责与事实所有权

|  | 官方内容与说明（本计划） | 文档框架与 CLI（R0，不改） | GitHub Pages / 公共仓库 |
| --- | --- | --- | --- |
| 拥有 | 12 份指南的描述文字、作者参考与设计文档的结论陈述、C2 的版本身份 | 描述选取顺序、llms.txt 与 meta 的生成、产物事务 | 托管、部署结果、公共 Git 可达性 |
| 不拥有 | **框架的描述语义、PRD 措辞、CLI 帮助文案** | 内容措辞 | 内容与候选的选择 |

一句话：只改内容和身份，生成规则不动；发布成败以工作流和线上核验为准。

### 0.5 明确不在本期

- **框架改为截首句**（报告 Q-01 选项 2）、**修订 PRD 的“一句话描述”措辞**（选项 3）——用户已选 D-01，不做。
- **代码复审 R2 F-01**（CLI `--help` 和内容模板 README 的回滚目标仍为 0.1.3）——由代码评审承接。它要改 CLI 源码（`crates/rustify-docs-cli/src/main.rs:32-33`、`templates/content/README.md:22`），可以随 C2 一起发布，但须用户另行确认后才纳入；纳入时这些改动放在 M1 的内容提交里，C2 仍只改配对文件。
- **报告 U-01**（三款原生浏览器在线上 CI 的 Wasm 上抽查）和 **U-02**（在新产物上重测 B_P2 的岛输入）——可选补证，不纳入。
- **手机实机**——按 PRD D17 不做。**中文指南翻译**——缺译回退页沿用源语言描述，这符合 D13。
- **原 P4 各里程碑的检查**——不重做，只按 D5 复用。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **已核实·缺口**：12 份英文指南都没有 frontmatter。`ai.rs:27` 的 `description()` 在没有显式描述时用 `page.fragment.summary`，即整段首段、不截断。线上 llms.txt 中 10 条描述有 2–6 句；Quick start 取到 Prerequisites 列表之后的那句“mbx xtask doctor reports …”，Architecture 取到“Theme integration below was checked …”。漏做的后果：外部 AI 工具从索引看不出这些指南讲什么（报告 G-02）。
- **已核实·足够**：frontmatter 的 `description` 键可选；有它时 title 仍取第一个 `# heading`（[docs-ssg.md L456–461](../docs-ssg.md)）。`docs/zh/quickstart.md` 已经用 frontmatter 描述，llms.txt 里是合格的单句。
- **已核实·足够**：catalog 生成器保留第一个 `<!-- catalogue -->` 之前的内容（`xtask/src/catalog.rs`），`crates/rustify-components/src/catalog.rs` 的 `the_capability_document_holds_the_table_the_code_holds` 只断言文档包含该表。B 语料的 profile 已冻结在 `tests/docs-ssg/b-profile.json`，只有 `docs-corpus --profile` 才会重算，指南改动不影响它（`xtask/src/docs_corpus.rs` 顶部说明）。
- **已核实·足够**：`tests/docs-ssg/official.spec.ts` 用源文件的 `# ` 行取指南标题，frontmatter 不影响；该文件 L252–262 已断言两份 llms.txt 各 37 条，是加描述断言的位置。`crates/rustify-docs-cli/tests/search.rs` 的 `official_sources_have_only_two_real_chinese_pages_and_keep_the_existing_catalog` 加载真实仓库并分析官方站，是写源级断言的同构位置。
- **已核实·缺口**：[docs-ssg.md L1475–1479](../docs-ssg.md) 写着“P4 retains the full browser matrix”；[DESIGN.docs-ssg.md L400](../../DESIGN.docs-ssg.md) 中以“Native stable browsers, physical phones, spoken VoiceOver”开头的那句，仍把这些列为发布阶段尚待通过的门槛。两句都与 D17/D18 和[原 P4 发布记录](DOCS-SSG-P4.records/M7.md)的结论不符（报告 G-01）。
- **已核实·足够**：指南页底部的“最后更新”和 `sitemap.xml` 的 `lastmod` 取最近一次触及该源文件的提交日期（UTC，`crates/rustify-docs/src/history.rs`）；zh 回退页沿用英文源的日期；sitemap 不列 zh 回退页，只为 12 份 en 指南和 zh 快速开始写 `lastmod`。内容提交会把 12 份指南的这些日期改为提交当天；指南描述只出现在本页及其 zh 回退页的 meta 和 llms.txt 中。
- **已核实·足够**：原 P4 发布阶段留有两类 C1 归档，字节不同：[本机 C1 收据](../validation/docs-ssg-p4/m7/release-20261019-c31/c1-archive.json)（build `7e1e5c9e7e7392df`，`.rustify-docs/p4-release/c31-cvyouvfm/artifacts/c1-1146b7d8-site.tar.gz`），以及 [CI 收据](../validation/docs-ssg-p4/m7/publication-20261019/c1-qualified-archive.json)（run 39219151112，build `9e8ea80d57750509`，同 run 的 `artifact.tar`）。两者各 97 个文件，岛资源目录不同，48 个组件页和 manifest 也不同。
- **已核实·足够**：Pages 工作流与 prepare 只核对候选 SHA、CLI source 与 R 的格式，并保护除两个投影文件外的全部输入，不要求候选的父提交就是 R（C1 的父提交是文档提交 `0d9eedc7`）。当前 `framework-revision.txt` 为 R0，`components-Cargo.lock` 是 R0 的锁。
- **已核实·足够**：版本相关文字的位置：
  - `README.md` L205–208 与 `docs/docs-ssg.md` L20–41、L59–66、L79–81、L290、L704、L1385–1416 写的是 0.1.31/C1 和 0.1.29 回滚；
  - CLI 版本在 `crates/rustify-docs-cli/Cargo.toml`，根 `Cargo.lock` 有对应的 version 行。

### 1.2 文件清单

| 落点 | 改动 |
| --- | --- |
| `docs/{quickstart,architecture,navigation,workspace,forms,i18n,data,themes,utility,compatibility,components,vellum}.md` | 文件开头加 `---` / `description: …` / `---`，文字见 §5.1；正文、H1 和 catalogue 标记不动 |
| `docs/docs-ssg.md` | M1：L1475–1479 的 P4 浏览器范围（§5.2）。M2（C2）：L290、L704 的配对 CLI 版本。M3（发布后）：安装、回滚、线上交付各节改为 C2/0.1.32，last-good 改为 C1/0.1.31 |
| `DESIGN.docs-ssg.md` | M1：L400 的发布门槛句（§5.2） |
| `README.md` | M3（发布后）：L205–208 改为 0.1.32 与 C2 |
| `crates/rustify-docs-cli/tests/search.rs` | 新增 `official_guides_declare_one_sentence_descriptions`（§5.1 的规则） |
| `tests/docs-ssg/official.spec.ts` | 在 L252–262 的 llms.txt 循环里加断言：指南条目的描述等于源 frontmatter，zh 回退条目沿用同一句 |
| `crates/rustify-docs-cli/Cargo.toml`、`Cargo.lock` | M2（C2）：CLI 0.1.31 → 0.1.32，以及根锁中这一个包的 version 行 |
| `docs/plan/DOCS-SSG-P4-gaps.records/` ★、`docs/validation/docs-ssg-p4-gaps/` ★ | 里程碑记录、身份/比对/线上核验摘要 |

不改的邻接面：
- `crates/rustify-docs*` 的产品源码、`xtask/src/catalog.rs`、`framework-revision.txt`、`components-Cargo.lock`；
- `site/`、`rustify-docs.toml`；
- `.github/workflows/docs-pages.yml` 与 `scripts/docs-pages-*.py`。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| 文档框架的描述与 llms 生成（R0） | 已核实足够 | `ai.rs:27`：先用显式描述，再用首段；zh 回退沿用源描述（D13） | 零改动 | — |
| `docs-pages.yml` + `docs-pages-prepare.py` + `docs-pages-verify.py` | 已核实足够 | 候选、CLI source、R 校验；97 文件同 run 归档与线上核验在原 P4 发布阶段跑过四轮 | 零改动，指定 C2 运行 | 构建或部署失败时线上保持 C1；核验失败按 §8 回退 |
| GitHub Pages（`rockie/rustify-ui`，workflow 来源，`github-pages` 环境仅允许 main） | 已核实足够 | 原 P4 发布阶段的四次 run，最后一次是 39355775158 | 只用既有配置，不改设置 | 后台 `deployment_failed`（原 P4 发布阶段出现过）时不计通过，按 §8 处理 |
| 公共 Git（HTTPS） | 已核实足够 | 原 P4 发布阶段用隔离缓存公开安装过 C0/C1 | 推送 main 后公开安装 C2；说明提交后再快进推送一次，从公共 main 读回（V-10） | 推送不完整时不更新公开说明；说明推送失败时 M3 未完成，Pages 保持已核验的 C2 |
| GitHub 文件预览 | 依赖 A-1 | 未实看过带 frontmatter 的本仓指南 | V-6 实看 | 不可读时停在发布前，回到 D2 |

## 5. 核心机制

### 5.1 指南描述（G-02 / D-01）

规则：
- 每份指南一句英文，说明页面主题；以 `.` 结尾，中间没有句末标点加空格（`. `、`? `、`! `）；不超过 200 个字符。
- 写成 YAML 普通标量，不含 `: ` 和 ` #`。
- 不重复标题，也不写版本号、提交号或评审编号。

下表为定稿文字。实施时对照正文核对事实，必要时只调整措辞，并保持上述规则。

| 指南 | `description` |
| --- | --- |
| quickstart | Install the pinned toolchain, build and preview the examples, and start a DOM application, GPU region or Tailwind setup with Rustify UI. |
| architecture | How a page's wasm instances, scopes and GPU regions fit together, and the runtime, hydration, theme and build contracts they follow. |
| navigation | How a scope owns or shares the address bar, and how links, guards, sub-path deployment and routed GPU regions behave. |
| workspace | Side-by-side panels with dividers, tabs over panels and one command list, plus who owns drag and wheel input between panels. |
| forms | Forms as a state machine for field errors, validation, saving and unsaved changes, while the application keeps the values. |
| i18n | What the SDK translates in English and zh-CN, how the language is chosen, and how formatting, text direction and font fallback work. |
| data | Tables larger than the screen, with windowed rows, selection by identity, going to unseen rows, work sliced across frames, sorting, filtering and editing. |
| themes | Create, resolve, import and export versioned theme documents and apply one resolved snapshot to DOM scopes and GPU regions. |
| utility | Generate CSS at runtime for class names missing from the static build with UtilityScope and its Rust WASM worker. |
| compatibility | The pinned versions, browser matrix, deployment contract and capability limits this repository has actually verified. |
| components | The twenty-four component categories, each with a DOM component, and what a GPU region can draw for each of them. |
| vellum | How the Vellum editor example maps onto Rustify UI's DOM and GPU layers, with reproducible figures and twin comparison results. |

检查的断言（V-1、V-5 共用同一规则）：

- **源级（V-1）**：在 `crates/rustify-docs-cli/tests/search.rs` 中，沿用 `official_sources_have_only_two_real_chinese_pages_and_keep_the_existing_catalog` 的 `Project::load` → `content::snapshot` → `analyze_with_surfaces` 流程，对 12 个英文指南页断言：
  - `page.front.description` 存在，并满足上面的规则；
  - `analysis.diagnostics` 中没有这 12 个文件的 `missing-description`；
  - 对应的 11 个 zh 回退页的源描述与英文页相同。
- **产物级（V-5、V-8）**：`official.spec.ts` 的 llms.txt 循环中，en 指南条目为 `[标题](URL): <源 description>`；zh 回退条目的描述与之相同，标题带回退标注；中文快速开始仍用中文 frontmatter 描述。

### 5.2 P4 结论文档同步（G-01）

- **docs/docs-ssg.md L1475–1479**：把“P4 retains the full browser matrix.”改为 P4 的实际范围：
  - 桌面最新稳定版 Chrome、Firefox、Safari 已验证；
  - 按维护者决定没有做手机实机，因此不宣称任何手机浏览器已验证；
  - VoiceOver 与 Safari 由维护者本人验收。

  P1 那句与引擎模拟的说明不动。
- **DESIGN.docs-ssg.md L400**：把以“Native stable browsers, physical phones, spoken VoiceOver”开头、以“gates;”结尾的那句改为原 P4 发布记录的结论，并链接 `docs/plan/DOCS-SSG-P4.records/M7.md`：
  - 三款原生稳定版（含原生 200%）和公开发布已经验证；
  - 手机实机按维护者决定没有做；
  - VoiceOver 为维护者本人验收。

  保留“`search-text-200.png` is text/layout emulation”。
- 两处都不出现“phones passed”，也不再说这些检查仍待发布阶段。

### 5.3 C2 配对（M2）

1. M1 的内容提交在 `main` 上，作为 C2 的父提交。
2. C2 只做这些改动：
   - `crates/rustify-docs-cli/Cargo.toml` 的 version 从当时的实际值递增一个 patch（基线为 0.1.31 → 0.1.32）；
   - 运行 `mbx check --offline -p rustify-docs-cli`（不加 `--locked`），让根 `Cargo.lock` 只改写该包的 version 行；
   - `docs/docs-ssg.md` L290、L704 的“paired/published CLI 0.1.31”改为候选 0.1.32，用“candidate”措辞，安装与回滚各节在 M3 才改。

   `framework-revision.txt` 和 `components-Cargo.lock` 不动，仍为 R0。
3. 边界核对：`git diff --name-only C2^ C2` 是五个配对文件的子集；`git show C2:crates/rustify-docs-cli/assets/framework-revision.txt` 为 R0。
4. 本地实装沿原计划 §3.2，环境变量与原 P4 发布阶段相同（只用进程级 remap，不改全局 Git 配置）：

```sh
export DOCS_LOCAL_GIT="file://$PWD" DOCS_C=<C2 完整 SHA> DOCS_INSTALL="$PWD/.rustify-docs/p4-gaps/install-c2"
export CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
  GIT_CONFIG_KEY_0="url.file://$PWD.insteadOf" GIT_CONFIG_VALUE_0=https://github.com/rockie/rustify-ui
mbx install --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C" --locked --profile docs-cli --root "$DOCS_INSTALL" rustify-docs-cli
export RUSTIFY_DOCS_BIN="$DOCS_INSTALL/bin/rustify-docs"
"$RUSTIFY_DOCS_BIN" --build-info      # rustify-docs 0.1.32, source = C2, panic: unwind
```

### 5.4 发布与公开说明（M3）

见 §8 的发布顺序。发布与核验都通过后，单独做一个说明提交：
- `README.md` L205–208 和 `docs/docs-ssg.md` 的安装节（L18–41、L59）改为 0.1.32 与 C2；
- 回滚节（L62–66、L79–81）把已验证的回滚目标改为 C1 0.1.31；
- 线上交付节（L1385–1416）记录 C2 的 run 与 last-good C1。

措辞沿原 P4 发布后同节的写法，不写尚未发生的结论。

说明提交后按 §8 第 7 步快进推送 `main`，再用 V-10 从公共 `main` 读回；只有读回通过，公开说明才算交付。工作流只由 dispatch 触发，这次推送不会重新部署，Pages 仍是已核验的 C2，不为说明提交重新 dispatch。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1/AC-3 | 原 P4 发布记录：首载最大 gzip 40,584B，上限 100KB | 只有 23 个指南页的三处描述元数据和“最后更新”日期变化，每页 gzip 变化应小于 1KB | 超过 1KB 时，用 P2 的 [inventory-first-load.py](../validation/docs-ssg-p2/m6/inventory-first-load.py) 沿原用法复核 | 只改元数据 | V-4 比对时记录变化页的 gzip -9 增量 |
| WORK-N3/AC-2、AC-3 | 原 P4 线上 76 页 CSP 违规 0，外源 0 | 保持 | 不放宽 CSP；失败则不发布或回退 | 框架和资产不变 | V-4 预检（meta CSP 与引用）、V-7、V-8 |
| WORK-N7 | 原 P4 | 产物不出现机器路径或源 Markdown | 预检失败不上传 | 生成规则不变 | V-4、V-7 |

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 描述写成多句或含 `: ` 导致 YAML 解析失败 | 本地构建 | 无 | 不发布 | V-1 失败，或 check 报 frontmatter error | 改文字后重跑 V-1/V-4 | V-1、V-4 |
| C2 夹带配对以外的文件，或 R 被改动 | 发布身份 | 无 | 不发布 | §5.3 第 3 步的边界核对 | 重做 C2（新 patch 版本，不改写已推送的提交） | V-4 |
| 产物比对出现预期外的变化（搜索、CSS/JS、Wasm） | 已复用证据失效 | 无 | 不发布 | V-4 比对 | 查明原因；确需发布时重跑原 P4 的对应检查（搜索性能、岛输入等） | V-4 |
| GitHub 预览不可读（A-1 不成立） | 仓库读者 | 无 | 指南在 GitHub 上难读 | V-6 | 不发布 C2；回到 D2 请用户重新决定 | V-6 |
| 工作流 build 或 deploy 失败（含后台 `deployment_failed`） | 本次发布 | 无 | 线上仍为 C1 | run 结论 | 只重试失败的 deploy job，或用同一 C2 重新 dispatch；不计通过 | V-7 |
| 线上核验失败（文件、身份、llms、浏览器） | 官方站 | 新内容暂不可用 | 读者可能看到异常 | V-7、V-8 | `gh workflow run docs-pages.yml --ref main -f candidate_sha=1146b7d8febdf53ca1bc1a4c01946604e7625a95` 回退 C1，并对该 run 执行 V-7 的核验命令 | V-7 |
| 公开安装 C2 失败 | 作者安装 | 无 | 说明仍写 C1 | V-9 | 不做 §5.4 的说明提交，查明推送或锁的问题 | V-9 |
| 说明提交推送失败，或公共读回不符 | 公开说明 | 无 | Pages 已是 C2，公共 `main` 的安装与回滚说明仍指可安装的 C1 0.1.31 | V-10 | 不回退 Pages；不强推，不改写已推送的提交；查明原因后重推或修正说明再推。V-10 通过前 M3 记为阻塞 | V-10 |

发布顺序：

1. 在 M2 中 V-4、V-5 通过。
2. 向用户呈现：C2、产物差异摘要、last-good C1，以及要授权的全部动作：
   - 普通快进推送 `main` 到 C2；
   - 以 C2 dispatch，核验失败时以 C1 回退 dispatch；
   - 发布通过后普通快进推送 §5.4 的说明提交。

   取得覆盖这些动作的明确授权（C-2）。没有授权就停在这里，并记为阻塞。
3. 普通快进推送 `main`，不纳入未提交的工作树，不强推。随后执行 V-6。
4. `gh workflow run docs-pages.yml --ref main -f candidate_sha="$DOCS_C"`，跟踪该 run，要求 build 和 deploy 都成功。
5. 依次执行 V-7、V-8、V-9。任一失败就按上表回退 C1，记录后停止。
6. 都通过后做 §5.4 的说明提交。
7. 按第 3 步的规则普通快进推送该说明提交，授权已在第 2 步取得；不重新 dispatch。随后执行 V-10，失败按上表处理。
8. V-10 通过后进入最终提交。

## 9. 验证

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F12、D-01：12 份指南都有显式描述，并满足 §5.1 的规则；官方分析中这些文件没有 `missing-description`；zh 回退沿用源描述。先在改指南之前看到新测试失败 | `mbx test --locked -p rustify-docs-cli --test search official`（含新增测试与原官方图测试） | M1（局部） |
| V-2 | D3：加了 frontmatter 的 `docs/components.md` 仍与 catalogue 一致 | `mbx xtask catalog --write docs/components.md --check`；`mbx test --locked -p rustify-components --lib catalog::` | M1（局部） |
| V-3 | G-01：两处陈述与 D17/D18、原 P4 发布记录一致 | `rg -n "P4 retains the full browser matrix\|remain M[0-9] gates" docs/docs-ssg.md DESIGN.docs-ssg.md` 无输出；人工对读 §5.2 的三点要素，并核对链接存在 | M1（局部） |
| V-4 | C-1、D1、D5、NFR 行：C2 的边界与身份；官方产物 0 error；预检通过；产物只按预期变化 | 先按 §5.3 实装 C2，再运行：<br>• `mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"`；<br>• `python3 scripts/docs-pages-prepare.py --candidate-sha "$DOCS_C" --cli "$RUSTIFY_DOCS_BIN" --out "$DOCS_SITE"`（Python ≥ 3.11；`$DOCS_SITE` 在仓库和任何 `.rustify-docs/` 之外，例如 `$TMPDIR/rustify-p4-gaps-c2-*/official`，否则 build 拒绝输出位置），再在 `$DOCS_SITE` 上执行 `"$RUSTIFY_DOCS_BIN" check/build --config …/rustify-docs.toml --format json`（0 error，指南不再报 missing-description）；<br>• `python3 scripts/docs-pages-verify.py --dir "$DOCS_SITE/site/dist" --identity <按工作流格式写的 identity.json> --candidate-sha "$DOCS_C" --report …/preflight.json`；<br>• 与同环境的本机 C1 归档逐文件比对。基线是 §1.1 的本机 C1 收据所记归档，先核对其 SHA-256 与收据一致；归档缺失、哈希不符或本机工具链与当时不同，就按 §5.3 的同一方式实装 C1 `1146b7d8febdf53ca1bc1a4c01946604e7625a95`，走同一 prepare/build 重建 C1 再比较。CI 收据的归档只用于线上证据，不作本项基线。<br>• 只允许这些变化：23 个指南页 HTML，其中只有 meta/og/twitter 三处描述（等于源 frontmatter）和“最后更新”日期；`sitemap.xml` 中 12 条指南的 `lastmod`；两份 `llms.txt`；manifest。记录变化页的 gzip -9 增量 | M2（局部，发布前风险门） |
| V-5 | WORK-F12、F16/AC-4 的产物级断言，以及官方站回归 | 先在 M1 用临时配置（写法同 V-8）对线上 C1 跑 `--grep "official Chinese home"`，确认新断言失败（红）。M2 再用 C2 实装：`RUSTIFY_DOCS_BIN="$DOCS_INSTALL/bin/rustify-docs" npx playwright test -c playwright.docs.config.ts --project=official-subpath official.spec.ts`，原 56 项加新断言全部通过 | 红在 M1；绿在 M2（局部回归，单个 project） |
| V-6 | C-4、A-1：GitHub 预览可读 | 推送后，Agent 用真实浏览器打开 `https://github.com/rockie/rustify-ui/blob/<C2>/docs/quickstart.md`、`docs/components.md`、`docs/architecture.md`：frontmatter 显示为表格，H1 与正文、代码块、catalogue 表正常；保存截图 | M3（收口） |
| V-7 | WORK-F16/AC-3–4、D16：同 run 完整产物与线上身份 | `gh run watch <run>`，build 与 deploy 成功；`python3 scripts/docs-pages-verify.py --url https://rockie.github.io/rustify-ui/ --candidate-sha "$DOCS_C" --run-id "$DOCS_RUN_ID"` 通过（97 文件、76 个 HTML、2 个搜索索引、2 个 AI 索引，cli_version 0.1.32） | M3（收口） |
| V-8 | WORK-F12、F16/AC-4 线上：新描述生效，原有四项读者任务回归 | 在 `.rustify-docs/p4-gaps/` 写临时配置：沿用 `playwright.docs.config.ts` 的 `official-meta` 项目，`baseURL` 改为 `https://rockie.github.io/rustify-ui/`，`webServer: []`，代理取环境变量；然后 `npx playwright test -c <临时配置> --project=official-meta --grep "official Chinese home\|CATALOG button uses\|deep not-found\|the dark first frame"`，要求 4/4 | M3（收口） |
| V-9 | D16：公开 Git 安装 C2 | 使用新的空 `CARGO_HOME`（只共享 registry 缓存），不用 remap，并关闭全局/系统 Git 配置：`cargo install --git https://github.com/rockie/rustify-ui --rev "$DOCS_C" --locked --profile docs-cli --root <临时目录> rustify-docs-cli`；`--build-info` 为 0.1.32、source C2、unwind | M3（收口） |
| V-10 | C-2、D16：发布后的公开说明已到达读者 | 说明提交推送后：<br>• `git ls-remote https://github.com/rockie/rustify-ui refs/heads/main` 得到公共 main 的 SHA，`git merge-base --is-ancestor <说明提交> <该 SHA>` 成立；<br>• 用 `curl -fsSL https://raw.githubusercontent.com/rockie/rustify-ui/<该 SHA>/README.md`（`docs/docs-ssg.md` 同法）读回：安装与升级命令为 0.1.32、`--rev` C2；回滚目标为 C1 `1146b7d8febdf53ca1bc1a4c01946604e7625a95` 0.1.31；线上交付节记录 C2 的发布 run 与 last-good C1；不再把 0.1.31 写成当前版本，也不再把 0.1.29 写成本次回滚目标；<br>• `curl -fsSL https://rockie.github.io/rustify-ui/rustify-docs-manifest.json` 的 `cli_version` 仍为 0.1.32，`build_id` 等于 V-7 的 run 所记 | M3（收口） |

V-5 的新断言先对线上 C1 变红，是因为 C1 的指南描述不是源 frontmatter（源里还没有 frontmatter）。这次红灯只读访问公共站点，不改变任何状态。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 指南描述、文档同步与测试，形成内容提交 | 无 | <ul><li>V-1 新测试与 V-5 新断言先红；</li><li>按 §5.1 给 12 个指南加 frontmatter；</li><li>按 §5.2 改两处文档；</li><li>同构参照：`docs/zh/quickstart.md` 的 frontmatter 和 `official_sources_have_only_two_real_chinese_pages_and_keep_the_existing_catalog`；</li><li>主实施者一人修改，不委派；</li><li>完成后在 `main` 上提交内容（作为 C2 的父提交）。</li></ul> | V-1、V-2、V-3 通过，V-5 的红灯已记录；回写「实施进度」 |
| M2 | C2 配对与本地候选核验 | M1 的内容提交，以及 V-1–V-3 | <ul><li>按 §5.3 生成 C2 并实装；</li><li>浏览器与构建串行，只跑 `official-subpath` 一个 project；</li><li>证据摘要放在 `docs/validation/docs-ssg-p4-gaps/m2/`，大文件留在 `.rustify-docs/p4-gaps/`。</li></ul> | V-4、V-5（绿）通过；回写「实施进度」 |
| M3 | 发布授权、公开发布与线上核验（阶段收口，含发布风险门） | V-4、V-5 通过；用户授权（C-2） | <ul><li>按 §8 的发布顺序执行；</li><li>失败时回退 C1；</li><li>通过后按 §5.4 更新公开说明，快进推送并从公共 `main` 读回；</li><li>主实施者独占发布与推送动作。</li></ul> | V-6、V-7、V-8、V-9、V-10 通过，§5.4 的说明提交已在公共 `main` 上；没有授权、说明未推送或读回不符时记为阻塞，不算完成；回写「实施进度」 |

提交点：
- M1：内容提交。R/C 流程要求它作为 C2 的父提交。
- M2：C2 配对提交，只含配对文件。
- M3：发布后的公开说明提交，按 §8 第 7 步推送到公共 `main`。
- 全部完成后：最终提交，回填各行 SHA，再单独提交计划回填。

按原计划同样的约定，C2 和公开说明提交的 SHA 写入完成记录，C2 不自引用。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 发布授权（C-2） | 不能推送和发布；授权未覆盖说明推送时公开说明交付不了 | 用户；M3 开始时按 §8 第 2 步一次请求全部动作 | M3 | 否（是发布门，不阻塞 M1/M2） |
| A-1：GitHub 预览 | 指南在 GitHub 上的可读性 | Agent 用 V-6 实看；不成立时请用户在 D2 的替代方案间选择 | M3 推送后、dispatch 前 | 否 |
| 代码复审 F-01 是否随 C2 发布 | 发布范围 | 用户确认后才纳入，放在 M1 的内容提交 | M2 生成 C2 之前 | 否（默认不纳入） |
| Pages 后台偶发 `deployment_failed` | 发布延迟 | 按 §8 重试；线上保持 C1 | M3 | 否 |
| mbx 清理 `target/` 导致证据丢失 | 证据缺失 | 证据放 `.rustify-docs/` 和 `docs/validation/`；跑浏览器前先核对产物仍在 | 各里程碑 | 否 |

- 最终状态：**Ready**。
- 定级理由：范围、文字规则、发布路线和检查都已定；没有影响范围、安全、数据或外部契约的未决项。发布授权是 M3 的发布门，A-1 在推送后、发布前验证，两者都有处置路径。

## 12. 已知坑与历史教训

- **C 不得夹带产品改动，也不能引用自己的 SHA**（[原计划 §3.2](DOCS-SSG-P4.md#32-固定-cli库配对)、[P3 §3.2](DOCS-SSG-P3.md#32-固定交付与回退)）。所以要先做内容提交，C2 只改配对文件，C2 的 SHA 留到之后的说明提交再写。
- **构建前预检通过，不等于上传完整**：原 P4 首轮发布因为上传动作排除隐藏文件，少了 `.nojekyll`。现在由工作流的归档校验负责，V-7 仍以同 run 归档和线上 97 个文件为准（[原 P4 发布记录](DOCS-SSG-P4.records/M7.md)「首轮打包边界修复」）。
- **本机构建的 Wasm 与 CI 构建的 Wasm 不同**（代码复审 R2 待核实 1；两份 C1 归档实测岛资源和组件页不同，见 §1.1）。V-4 比对的是本机 C2 与同环境的[本机 C1 归档](../validation/docs-ssg-p4/m7/release-20261019-c31/c1-archive.json)，不是 [CI 收据](../validation/docs-ssg-p4/m7/publication-20261019/c1-qualified-archive.json)所记的归档；线上以 V-7 为准，不能把本机字节当成线上字节。
- **Node 请求可能不走代理**，HTTP/2 的 404 也没有 reason phrase（原 P4 发布记录「公开浏览器与后续检查」）。V-8 的临时配置要显式传入代理；`official.spec.ts` 已兼容空 reason phrase。
- **mbx 会清理受管的 `target/`**（原 P4 发布记录中原生检查结论与环境恢复一节）。实装目录和站点放在 `.rustify-docs/p4-gaps/`，跑浏览器前先确认产物还在。
- **一次只跑一个 Playwright project，并且必须带 `--project`**（`CLAUDE.md`「Browser tests」）。
