# DOCS-SSG-P1 · Rustify UI · 纯 DOM SDK 与静态内容文档站（GPU 按需启用 · 原生 Rust CLI · 静态 HTML）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-10 +08:00 · `main` / `b0a6bc5b9010e57d1f0d758a37af573ea0712faa` · dirty。事实以当前工作树为准；PRD、计划、评审及相关 SDLC 更新尚未提交，代码无改动。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P1（第 1 期）**，以 §7.1 出口和 §7.2 AC 分期为范围；PRD SHA-256 `c588c505348107acf7e90f1b9bf0a941d28e158abd85a3aeec62a57a130bfbd5`。
>
> 本期交付：**先交付默认不带 Makepad、行为完整的纯 DOM SDK；再交付 Rustify Docs 内容模板与 `rustify-docs new/dev/build/preview/check`，生成含导航、主题、代码高亮、元数据的静态文档站。**
> 本期独特职责：现有工具链服务 GPU 示例；本期首次让独立消费者不经过 Makepad，同时让内容作者不编译站点 Rust 就能建站和迭代。
> **顶层排除：交互岛与水合、组件嵌入与演示、定制页与布局插槽、Theme Studio 文档导入、自身文档站迁移（P2）；多语言（P3）；搜索与 llms.txt（P4）；SPA、浏览器快照生成、部署上传命令。**

## 实施者定位

执行本计划的 agent 是**资深软件工程师**，依据本计划、仓库规则和当前代码实施。

- **表达与源码**：报告用路径、需求/检查/里程碑 ID 定位，不复述计划。源码不放需求编号、agent 标记或规划元数据，注释解释必要原因；追溯放记录或提交说明。
- **编号与任务**：需求/约束/假设 ID 只在需求账本登记一次，上游 ID 保留原编号及来源；检查、里程碑、任务 ID 分别由验证表、里程碑表、按需内部任务表定义，正文、NFR 展开、映射和进度记录只引用。修订保留已有 ID，删除/拆分时同步引用；任务表若存在，按交付物、前置成果与检查 ID 推进，任务进展及证据写入所属里程碑记录，不另设任务状态表。
- **完成的定义**：本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口，不冒充通过。集中到阶段收口的终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收。
- **工作顺序**：需要行为测试的改动先红、再绿、再重构。按实际前置成果及检查推进，不等无关终验，数据、安全、发布及用户分期门不得绕过；稳定契约下可用 fixture 开发，不能替代真实集成。按 §9 执行局部检查与集中验收，同环境走查可合并；相关代码、配置、依赖、输入与环境未变可引用已通过证据，否则重跑受影响检查，收口仍跑必要集成回归。
- **并行交付**：独立任务在收益超过委派/集成成本且工具可用时使用 subagents，模型默认继承。任务说明只给目标/ID、稳定契约、可写与禁改路径、运行资源和检查；共享文件单人负责，数据库、端口、浏览器等隔离或串行。子 agent 返回结果、路径、验证证据与缺口，默认不另写任务文档；主 agent 审查并集成验收。边界变化先回报，失败只阻塞依赖任务；结束对话前收集或停止仍在运行的任务。

## 实施进度（实施期持续更新）

本节是本计划跨对话恢复的**唯一汇总入口**，协议文字不随实施改写，实施期只更新「恢复快照」和「完成记录」。续做时先读仓库规则、本节、当前任务相关正文与依赖，再核对 `git status` 和相关 diff；记录与工作树不符时先查明原因，不凭记录覆盖用户改动，也不凭代码存在推断验收已通过。只有继续未完任务、核验前置验收、排错或审查时，才按链接读取对应记录的相关部分；不要默认加载全部记录或原始日志。

**回写时机**：里程碑完成后立即回写，再报告完成或提交；下游只需部分成果时，所需前置检查通过并在记录保存证据即可推进。实质进展后受阻、暂停、交接、发现偏差或结束对话时也保存成果与缺口；普通子任务切换不单独回写。

**一次回写**：主 agent 更新对应记录，再同步快照变化字段和完成记录一行，不重写无变化内容或追加流水。快照写当前任务、进展与下一步，命令及证据只进记录。首次回写删占位行，每个已记进展的里程碑恰好一行，状态仅「进行中 / 阻塞 / 已完成」；完成时移至已完成行末，最近完成取该行。更新时间带时区，摘要 1–2 句，记录填相对计划的链接（如 `[M1 记录](FEATURE.records/M1.md)`）。范围、决策、接口、数据、风险或退出条件有偏差时，就地修订对应正文，不追加勘误历史。

**独立记录**：按需建 `<计划名>.records/M<n>.md`，头部写计划链接、更新时区、状态及验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）。正文只写交付行为/路径、必要偏差、未完项与继续动作；验收按 `检查 ID/覆盖范围 | 命令或步骤 | 结果/必要证据` 记录，共同环境和基线在头部写一次，有差异再单列。同一测试组可覆盖多条退出条件，须逐条可追溯；引用原证据与复用依据，不复制断言全文、成功日志或调试流水。未运行、失败、环境缺失明确区分；只有复杂交接、排错或证据需独立保存时才另建任务记录/日志/截图并链接。

**项目跟进索引**：仓库已登记状态台账，或存在仓根 `.xgent-ai/sdlc/protocol.md` 时（兼容已有 `docs/project-status/protocol.md`；两套并存未切换时先核对，不双写），每次上述回写完成后，读取台账的回写规则（默认目录读上述协议）并追加本计划的实施变化记录，链接本节和对应验收记录，再刷新受影响对象状态页和必要索引；未启用则跳过。复用已有计划 ID，只更新本次触及的范围，保留其他计划和各轮评审；台账仅作项目索引，不替代本节的恢复快照。跨机器使用仓根相对路径与明确基线，未提交成果标明同步限制；登记失败说明缺口，不将原成果回退。

**提交**：在里程碑表后的「提交点」提交：里程碑较多时在标出的阶段收口后提交，全部里程碑完成后必须提交一次；仓库规则或用户对分支、提交信息、签名或是否由 agent 提交另有要求时从其要求。提交前先回写本次进度并通过该提交点要求的检查；只暂存本计划相关改动，不纳入、不回退他人改动，不跳过提交钩子。提交后把短 SHA 写入完成记录中本次覆盖的各里程碑行的「提交」列（代码评审修复的提交写作「修复」加短 SHA，追加在原 SHA 之后），再单独提交这次只改计划文件的更新，该提交不回填自己的 SHA。已完成但未到提交点的行写「待提交」；无法提交时写明原因并记入当前阻塞，不伪造 SHA。

**全部完成**：最后一个里程碑完成后，按上条完成最终提交并回写 SHA，确认每个已完成行的「提交」都有 SHA；恢复快照的「下一步」写代码评审。随后向用户报告完成，并提示下一步用 review-code 以本计划路径为参数评审本计划：它从完成记录找到要审的提交，并纳入未提交改动。评审由用户发起，实施中不自行启动评审或修复评审意见。

**回写后自查**：每次回写后直接核对以下条目：「当前进度 n/N」的 N 等于里程碑总数，n 只计状态为「已完成」的行；「最近完成」对应最后一条已完成记录，不按编号推算；链接指向已落盘文件；已完成行的「提交」是短 SHA 或「待提交」，全部完成时每行都有 SHA 且没有「待提交」。上述结构、计数与链接检查不证明验收真实通过。

本仓 `CLAUDE.md` 的 Plans 约定优先：上述独立记录实际落在 `docs/validation/docs-ssg-p1/M<n>.md`，完成记录使用相对链接；不预建空记录。计划对象 ID：`526c88c7-a4b3-4ea9-9193-371420b66d1d`。

### 恢复快照

- 最近更新：2026-10-12T11:13:25.159213+08:00
- 当前进度：8/9 个里程碑完成
- 当前状态：原 M1–M8 保留完成；M9 等待独立 gaps M2 的同候选真实矩阵收口
- 最近完成：M8（P1 跨浏览器、部署与性能终验）
- 下一步：先按 [DOCS-SSG-P1-gaps](DOCS-SSG-P1-gaps.md#实施进度实施期持续更新) 补 V-6，并由用户发起本轮候选代码评审；收口后 M9 对 CLI 0.1.3 / writer 9 候选重新定安装 SHA，执行原 file:// 与 GitHub 同 SHA 安装、帮助/模板/仓外DOM/安装后二进制/预览及 V-15，远端仍先取得推送授权与方式
- 当前阻塞：gaps M2/V-6 的最新稳定 Chrome、真实 Safari 及两手机同产物证据待补；原 M9 的安装与远端授权门保留，本次未执行
- 代码基线：原实现与 R1–R5 修复 SHA 保留在完成记录；本轮已提交 gaps 候选 `42757c6` 为 CLI `0.1.3`、writer `9`，提交与指纹见 [gaps M1](../validation/docs-ssg-p1-gaps/M1.md)。旧性能/真机事实保留其历史范围；原用户报告与 SDLC dirty 资料未纳入本轮候选



### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-10T17:20:00+08:00 | 默认 DOM SDK、中性 runtime/DOM loader、GPU opt-in 与示例迁移；V-1/V-2 仓内外、V-15 矩阵通过，V-3 全部适用原回归 604 例通过（theme-studio 整项目重跑 65/65） | [M1 记录](../validation/docs-ssg-p1/M1.md) | 179c5ec |
| M2 | 已完成 | 2026-10-10T18:10:00+08:00 | rustify-docs 库与 CLI 骨架、SSR 页面、原生样式/压缩/前缀高亮、片段缓存与并行、B 语料与采时 harness；风险门 195 ms/33 ms/22.8 KB 通过，git 安装排练、V-15 矩阵通过，示例产物逐字节不变 | [M2 记录](../validation/docs-ssg-p1/M2.md) | 9442f5d |
| M3 | 已完成 | 2026-10-11T23:15:15+08:00 | 原内容图交付与 R1–R4 修复保留；R5 修复 F-21：按完整文档 body/article 上下文收集真实锚点，根/子路径同页、跨页 check/build/dev、模板幽灵拒绝与旧产物保全通过，待独立复审 | [M3 记录](../validation/docs-ssg-p1/M3.md) | 9442f5d；修复 12113ab；修复 a297dbd；修复 016110f；修复 f7a3e94；修复 998ee18 |
| M4 | 已完成 | 2026-10-11T14:59:07+08:00 | `new`/`build`/`preview` 与内容模板、元数据/404/站点文件/manifest、资源指纹、发布事务与锁；V-5/V-6/V-10 基本门、V-7 三项目各 5/5、B 完整构建中位 347 ms、V-15 矩阵与安装排练通过，示例产物逐字节不变；R1 修复 F-05、F-07、F-09–F-11；R2 修复 F-15：原 URL 先拆后缀再解码文件名 | [M4 记录](../validation/docs-ssg-p1/M4.md) | 9442f5d；修复 12113ab；修复 a297dbd |
| M6 | 已完成 | 2026-10-11T16:34:55+08:00 | `rustify-docs dev`：每轮全量输入+内容键片段缓存、受影响页按两代字节比较刷新并保滚动、失败/panic 保留上一代、诊断面板（finish review ship）；V-9 浏览器 9/9 与 CLI 9 例、abort 构建拒绝 dev、B 保存可见中位 177/172/169 ms 与短轮 20 次 ≤178 ms；R1 修复 F-13；R2 续修 F-13：真实目标/别名及失效依赖；原子替换、删除恢复与无关页稳定已验证；R3 续修 F-13：失效 content/static 附件 symlink 保留恢复依赖，仅恢复目标即清错/刷新，无关页稳定 | [M6 记录](../validation/docs-ssg-p1/M6.md) | 1d239ff；修复 12113ab；修复 a297dbd；修复 016110f |
| M7 | 已完成 | 2026-10-11T16:34:55+08:00 | `check --external-links`（rustls+内置根证书、去重/并发/超时、失败即报错）、发布三步骤真实 SIGKILL/磁盘满/权限/损坏 manifest/并发注入与输出归属加固、新窗站外链接需 noopener、零机器路径泄漏；V-15 矩阵、features/artifacts/reliability/toolchain 四组（禁网作者演练只调用 2 次 `rustc --version`）、docs-root/meta/dev 全过；R1 修复 F-01–F-03、F-06；R2 续修 F-06、新增 F-16/F-17：CSS token、实体与文档片段、实际发布集合检查；原生工具链/安全门通过；R3 续修 F-16、新增 F-19：数字实体范围与 compiler 任意值解码一致，生成路径防二次 underscore 解码 | [M7 记录](../validation/docs-ssg-p1/M7.md) | 1d239ff；修复 12113ab；修复 a297dbd；修复 016110f |
| M5 | 已完成 | 2026-10-11T12:40:00+08:00 | 默认阅读主题、finish review（ship）、axe 零违规；Chromium 三项目各 21/21、WebKit 20+1 跳过、B 最大页 LCP ≤740 ms/CLS 0；用户确认真实 macOS Safari 27.0.1 与手机走查通过；Firefox 移出本期（用户决定）；R1 修复 F-04、F-12（页头链接真机复走待用户决定） | [M5 记录](../validation/docs-ssg-p1/M5.md) | 1d239ff、72418b3；修复 12113ab |
| M8 | 已完成 | 2026-10-11T23:15:15+08:00 | 原终验与 R1–R4 证据保留；R5 复验 207 Rust、93 Chromium、24 份 production 文档 ID/导航/拒绝保全与 500 页/短轮保存通过；未重验的真机、GPU、安装与长轮证据按原范围保留，待独立复审 | [M8 记录](../validation/docs-ssg-p1/M8.md) | 72418b3；修复 a297dbd；修复 016110f；修复 f7a3e94；修复 998ee18 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

WORK-* 均沿用 [PRD §3、§7.2](../PRD-DOCS-SSG.md)；下表列出本期 AC 子集，不重定义其原文。跨期条目只能报告 P1 子集通过，不报告整条需求终验。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F1 | 功能 | PRD F1/AC-1、3、4；终验 P2 | 内容模板开箱预览；缺工具与非空目录拒绝且无残留 | §5.1；V-5、V-9、V-11 | P1 纳入 |
| WORK-F2 | 功能 | PRD F2/全部 AC；终验 P1 | Markdown、GFM、锚点、链接、图片、草稿 | §3、§5.2；V-4、V-6、V-9 | 全部纳入 |
| WORK-F3 | 功能 | PRD F3/全部 AC；终验 P1 | 11 种语言高亮、源码区间、复制与分组 | §5.2、§6；V-4、V-8、V-13 | 全部纳入 |
| WORK-F6 | 功能 | PRD F6/AC-1；终验 P2 | frontmatter 首页；站点基础配置 | §3、§6；V-8 | P1 纳入 |
| WORK-F7 | 功能 | PRD F7/全部 AC；终验 P1 | 普通链接导航、目录、上一页下一页、真实 404 | §5.3、§6；V-7、V-8 | 全部纳入 |
| WORK-F8 | 功能 | PRD F8/AC-1–4；终验 P2 | 模式首帧、三档布局、主色与 logo、可读行宽 | §5.4、§6；V-8、V-13 | P1 纳入 |
| WORK-F11 | 功能 | PRD F11/AC-1–3；终验 P3 | 标题、摘要、canonical、分享卡片、站点文件与更新时间 | §5.3；V-6、V-7 | P1 纳入 |
| WORK-F13 | 功能 | PRD F13/AC-1–3；终验 P2 | 内容增量刷新、保滚动、诊断恢复、端口冲突回退 | §5.5；V-9、V-14 | P1 纳入 |
| WORK-F14 | 功能 | PRD F14/全部 AC；终验 P1 | 基路径产物、指纹、压缩、生产条件 preview、结束摘要 | §5.3–5.4；V-6、V-7、V-10 | 全部纳入 |
| WORK-F15 | 功能 | PRD F15/AC-1、2、5；终验 P3 | 全量内容诊断、纯 JSON、默认离线 | §5.1–5.2；V-4、V-10、V-11 | P1 纳入 |
| WORK-F17 | 功能 | PRD F17/全部 AC；终验 P1；最先交付 | GPU 默认关闭、DOM 生命周期不退化、7 示例回归 | §4.1；V-1、V-2、V-3 | 全部纳入 |
| WORK-N1 | 性能 | PRD N1/AC-1–3；终验 P4 | 无 wasm 基础交互、读者加载与体积预算 | §7；V-8、V-14 | P1 纳入 |
| WORK-N2 | 性能 | PRD N2/AC-1–3；终验 P2 | 保存可见、新建启动与 500 页完整构建预算 | §5.5、§7；V-14 | P1 纳入 |
| WORK-N3 | 兼容 | PRD N3/AC-1–4；终验 P4 | 静态托管、header/meta CSP、同源请求、浏览器矩阵 | §5.3、§7；V-7、V-12 | P1 纳入；Firefox 稳定版移出本期（用户 2026-10-11） |
| WORK-N4 | 可维护性 | PRD N4/全部 AC；终验 P1 | 依赖及产物无 Makepad、最小工具链、离线 build/check | §4、§7；V-1、V-11 | 全部纳入 |
| WORK-N5 | 可用性 | PRD N5/AC-1、3、5、6；终验 P4 | 对比度、语义、缩放/窄屏、自动化 AA | §6–7；V-8、V-13 | P1 纳入 |
| WORK-N6 | 可靠性 | PRD N6/全部 AC；终验 P1 | 字节可复现、旧产物保全、定位诊断、千次保存 | §5.3、§5.5、§8；V-10、V-14 | 全部纳入 |
| WORK-N7 | 安全 | PRD N7/全部 AC；终验 P1 | 发布白名单、无私有路径/数据、拒绝违例 HTML、无跟踪 | §5.2、§7；V-10、V-11、V-7 | 全部纳入；AI 输出 P4 复核 |
| C-1 | 工具链 | `mise.toml`、根 `Cargo.toml`、PRD §4 | 仓内走 mbx；Leptos =0.8.22 不升级；消费者只需 Rust 稳定版及 wasm32 目标和 CLI | §4.2；V-1、V-11 | 已确认 |
| C-2 | 运行契约 | `docs/architecture.md` Runtime contracts；`web/loader.js` | 实例 trap 隔离、abort 清理、受控值、URL 单一所有者不改 | §4.1；V-2、V-3 | 已确认 |
| C-3 | 范围/提交 | PRD §7.1；`AGENTS.md`、`CLAUDE.md` Plans/Conventions | 9 个里程碑；当前分支；只提交相关文件；记录进 validation；不装未知服务/容器 | §10、§11 | 已确认 |
| C-4 | 验收 | `CLAUDE.md` Browser tests；PRD N1/N2/N3 | Playwright 显式 project、不同运行串行；性能在页内；真实 Safari/手机不可用引擎仿真冒充 | §9；V-12、V-14 | 已确认 |
| C-5 | 产品默认值 | 用户 2026-10-10 确认沿用 PRD Q1/Q3/Q5 默认 | Rustify Docs / rustify-docs；完整浏览器矩阵及本期现有阈值作为合同，测量后不自动放宽 | §7、§11；V-12、V-14 | 已确认 |
| C-6 | 平台 | 用户 2026-10-10 确认 | P1 CLI 只承诺 macOS（实际记录架构）；Linux/Windows 未验证、不承诺；N4/AC-3 的“无额外系统库或工具”按 macOS 判定 | §4.2、§8；V-11 | 已确认 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | SDK 开关 | `gpu` 默认关闭；SDK/components 默认 `csr`，另提供 `ssr` | DOM 消费者入口易用；宿主 SSR 与 GPU/CSR 分次编译，不依赖 feature-unified 工作区结果 | WORK-F17、C-1；§4.1–4.2 |
| D2 | 产品边界 | 一个库 `rustify-docs` 加一个二进制包 `rustify-docs-cli`，bin 名 `rustify-docs` | 库集中分析、渲染与产物事务；CLI 管参数、服务和 watch。不抽插件/后端接口 | WORK-F1、WORK-N4；ADR-2 |
| D3 | 默认主题 | Leptos 宿主渲染完整 HTML；小型同源原生 JS 负责基础交互 | 内容页绝不加载 wasm。只使默认主题实际使用的 Button/Link/Icon 宿主可用，其余组件 SSR 留 P2 | WORK-N1、DEP-2；§6 |
| D4 | 内容/地址 | `rustify-docs.toml` 加 YAML frontmatter；目录/index.html；站内跳转原生链接 | 一个内容图拥有地址、锚点、导航与校验；不经过 SDK router | WORK-F2、WORK-F7、PRD D2/D9 |
| D5 | 样式 | rustify-style 原生编译，加原生 CSS 压缩；从渲染结果提取类 | 只编译页面用到的类与框架固定交互状态类；不调用 Tailwind CLI、不加载 Worker | WORK-N4、DEP-4；ADR-3 |
| D6 | 安装渠道 | P1 按 PRD Q2 默认提供 git 安装，锁定可达提交 | 不修改 workspace `publish = false`，不做 crates.io 发布；最终安装必须来自提交而非 dirty 工作树 | WORK-N4；§8、V-11 |
| D7 | 更新 | 单写入构建器、依赖失效图、SSE、受影响页面整页刷新并恢复滚动 | 不做 DOM 热替换。全站布局变化可影响全部页；错误时保留上次成功版本 | WORK-F13、WORK-N6；§5.5 |
| D8 | 404/CSP | preview 为真实目录托管，不开 SPA；完整 header 加受限 meta | 404 按托管合同返回，不能用首页 200 掩盖；meta-only 不承诺防嵌入 | WORK-F7、WORK-N3、PRD D11 |
| D9 | DOM 资源分发 | runtime 随 SDK module snippet 分发，loader/基础 CSS 作为 SDK 常量由模板导出，components 导出 CSS；启动前校验 hooks 协议号 | JS 与 Rust 同 rev，错配明确拒启；不采用作者手动复制，见 ADR-1、§4.1 | WORK-F17、C-2；用户 2026-10-10 确认 |
| D10 | CLI 构建与平台 | 根 workspace `docs-cli` profile（unwind/thin LTO），安装与原生候选均指定；CSR/GPU 与 docs 两包分组运行；P1 只承诺 macOS | 每代 catch_unwind 隔离；不改根 release abort；依赖不得引入 macOS 上 Rust 与平台 cc/链接器之外的系统库或工具，见 ADR-2、§4.2、§5.5、§8 | WORK-N4、WORK-N6、C-6；用户 2026-10-10 确认 |
| D11 | 品牌配置 v1 | `theme.primary` 可缺或 `{light,dark}`；logo 可缺、单资源或浅深成对；按钮前景每态自动选黑/白 | 单色无法两态同时达 4.5:1；不提供单色派生，后续可兼容追加单值简写；见 §3、§6 | WORK-F8、WORK-N5；用户 2026-10-10 确认 |

### 0.3 ADR-lite

#### ADR-1：中性 DOM host 契约，不搬迁 GPU 引擎

- 状态：Accepted。
- 背景：`listeners`、toast、job、resize 借用了 Makepad host，只有 feature gate 不会得到完整 DOM SDK；WORK-F17 要求已开启 GPU 的运行契约不变。
- 候选 A：把共享 JS 生命周期从 `embedded.js` 提取到同一中性 runtime，GPU 和 DOM 共用实现；清理规则最集中，但影响成熟 GPU 的 fatal/pump 顺序和资源账本。
- 候选 B：SDK 内集中中性 Rust host binding；GPU boot 把现有 hooks 同时注入它，纯 DOM boot 提供独立无 GL 的 hooks。选 B：保持 GPU host 算法原样，仅适配初始化；新增 DOM 实现以同一行为契约测试防漂移。
- 后果：无需新公共 DOM crate、没有 SDK↔Makepad 循环依赖；代价是两种 JS host 存在部分策略重复，必须做双端 trap/abort/timer/resize 合同测试。`Pace` 按 feature 定义：未开 GPU 时 SDK 定义同形 DOM 枚举；开 GPU 时继续 reexport 原 Makepad Pace，保持 RegionApp 的返回类型和现有调用完全一致。两种定义保留 Discrete/Continuous 流名称语义，以共享 scheduler 合同测试保证一致；不增加转换或反向依赖。
- 资源分发（D9）：runtime 放 `crates/rustify-ui/web/` 并用 wasm-bindgen module snippet 编入（同 `crates/rustify-ui/src/utility/browser.rs` 的 utility-host 先例），每次启动调用工厂创建独立 host；loader/基础 CSS 由 SDK 的静态资源常量和模板的标准库导出程序写出，配合 §4.1 的协议握手。未选手动复制配方：它少公共资源入口，但作者须自行保持 JS/Rust 版本一致，还要访问仓库或 Cargo 缓存。M1 仓外 fixture 是实现后的可行性门，不把当前原型视为已有 API。
- 重评触发：第二次相同生命周期修复需同时修改两个 host，且合同测试不足以避免漂移时，再评估共享 JS 提取；不提前改 pump。

#### ADR-2：CLI 内置通用内容构建器，不为每站编译 Rust

- 状态：Accepted。
- 候选 A：CLI 编入原生 HTML 模板渲染器，内容图填模板；同样不编译站点 Rust，宿主依赖更轻，但布局需要另行维护模板语义，P2 复用 Leptos 组件时会形成双渲染路径。
- 候选 B：CLI 编入通用 Leptos SSR 默认主题和内容流水线，纯内容站只有 Markdown、配置、静态目录。选 B；复用现有组件与语义 token，P2 再为确有站点 Rust 的工程增加编译路径。
- 后果：CLI 安装时编译一次，new/dev/build/check 不启动站点 cargo；SSR 宿主需要按包选择构建，不使用 GPU 示例的 `xtask build-web`。Markdown 解析选择 `pulldown-cmark` 的 offset 事件，GFM 扩展和 HTML 校验由内容模块集中承接；不用浏览器快照或独立模板语言形成第二套主题。
- 构建（D10）：根 workspace 新增 `[profile.docs-cli] inherits="release", panic="unwind", lto="thin"`，保持既有 GPU/wasm release abort 不变；全部 CLI 安装/原生候选构建指定该 profile，每代 catch_unwind 隔离。未选保持 abort：那需要把渲染搬入可重启子进程才能满足 dev 连续服务，增加进程边界与 last-good 交接；根 release 改 unwind 会破坏 GPU 实例依赖的 abort 语义，不考虑。不采用“单测能捕获”来宣称安装后二进制安全。Cargo profile 仅在 workspace 根生效，按包覆盖不能改 panic/lto，测试不能证明安装 profile，见 [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)。2026-10-10 在临时 file:// git workspace 的最小 spike 已证实：`cargo install --git … --profile docs-cli` 采用根自定义 profile 且 catch_unwind 可恢复，不带 profile 时继承 release 并 abort，`[profile.release.package.*]` 写 panic 被 Cargo 拒绝。本仓真实依赖图下的 git 安装排练与安装二进制故障注入仍在 M2/M6/M9 补证。
- 重评触发：P2 组件注册/定制页需要站点代码，不改变纯内容快路。

#### ADR-3：补齐原生高亮与 CSS 压缩，不手写替代编译器

- 状态：Accepted。
- 候选 A：扩展 rustify-style 自己实现通用压缩与语法高亮；工具链纯 Rust，但增加与需求无关的语言维护面。
- 候选 B：复用现有 Tailwind 编译器，库内使用 `lightningcss` 压缩、`syntect` 构建期 class-based 高亮。选 B；它们是进程内依赖，不是用户工具。
- 证据：现有 `CssCompiler::build` 返回带格式 CSS；[Lightning CSS StyleSheet 文档](https://docs.rs/lightningcss/latest/lightningcss/stylesheet/struct.StyleSheet.html) 提供 parse/minify/to_css。新增依赖具体兼容版本在 M2 解析后锁进 Cargo.lock，不升级现有 Major。高亮使用 `ClassStyle::SpacedPrefixed { prefix: "hl-" }`，其 class 不进入工具类候选，配色 CSS 单独生成。覆盖 11 种语言，以测试枚举核验，缺 TypeScript 等 grammar 时补最小授权 grammar 并登记来源与许可，不把“默认语法包”视为已覆盖。
- 后果：避免破坏已有编译器契约；增加 CLI 编译时间与依赖体积。禁止 syntect 输出内联颜色或 lightningcss 引入外部请求；无脚本亮暗配色由外部 CSS 管理。
- 重评触发：锁定依赖不能在最小工具链构建，或无法满足预算；优先替换局部依赖，不转为 Node/Tailwind CLI。

### 0.4 职责与事实所有权

|  | 应用 SDK | 文档库/CLI | 站点作者/托管 |
| --- | --- | --- | --- |
| 拥有 | 实例生命周期、DOM 原语、GPU opt-in、语义 token | 内容图、诊断、主题 SSR、产物事务；CLI 服务/watch | Markdown/配置/静态资源；托管目录地址、错误页与 MIME |
| 不拥有 | 内容路由与站点 watch | GPU 引擎、账号/数据库、作者 Rust 沙箱、上传 | 用服务器 rewrite 修补生成错误；篡改生成的 CSP |

### 0.5 明确不在本期

- WORK-F4/F5、F16/AC-1–3 及 F1/AC-2、5、F6/AC-2–3、F8/AC-5–6、F13/AC-4、F15/AC-3、N1/AC-4、N2/AC-4 → P2；不迁移指南，不铺 24 类演示，不实现 hydrate。
- WORK-F10、F11/AC-4、F15/AC-4 → P3；单页 `lang` 属 P1 语义，不等于多语言路由。
- WORK-F9/F12、F16/AC-4、N1/AC-5、N3/AC-5–6、N5/AC-2、4 → P4。P1 不渲染失效的搜索/演示/语言按钮；基础键盘路径仍作为本期交互质量验证。
- SDK GPU 运行时算法、Makepad 分叉、既有样式范围和预算不改；没有 Portal App 接入、服务端身份、共享数据库或容器。

## 1. 当前事实与改动面

### 1.1 现状与缺口

| 结论 | 行为级证据 | 本期差异 |
| --- | --- | --- |
| 已核实·缺口 | `crates/rustify-ui/Cargo.toml` 无条件依赖 rustify-makepad；`lib.rs` 导出 GPU API；`scheduler.rs` reexport Makepad Pace | 可选依赖、明确 gate、Pace 条件定义；默认图才可能不含 Makepad |
| 已核实·缺口 | `listeners.rs` 从 Makepad 取 signal；`toast.rs`/`job.rs` 调其 defer；`overlay.rs` 用 observe_resize；`crates/rustify-makepad/src/wasm/host.rs` 未 boot 时 hooks 为 None | DOM host 注入不能省略或退化为无保护的 listener/timer |
| 已核实·缺口 | `lib.rs` 把 defer/resize helpers 与 GPU 类型同条导出；`diagnostics.rs` 的 asset failure 使用 Makepad hook；`data_table/view.rs`、Vellum 的 storage/app/presentation、Theme Studio 的 gpu_preview 使用 SDK helpers | wasm32 下 `defer`、`defer_after`、`observe_resize`、`ResizeObservation` 继续由 SDK 中性实现导出，路径/签名不随 gpu 开关变化；单独 gate GPU 类型与资源失败 hook |
| 已核实·缺口 | `web/loader.js` 顶部静态 import bridge/embedded，start 强制核对 message hash 并 Makepad boot；`xtask/src/build.rs` 调 cargo-makepad、先清输出 | 新 DOM boot 与普通 Cargo/bindgen 打包；文档输出不能复用会删除旧产物的 build-web |
| 已核实·缺口 | 根 workspace Leptos 强制 csr；`mount.rs` 建新内容/overlay 根并 `mount_to`；theme provider、部分组件仅 wasm32 | 拆 csr/ssr；默认主题 host render；mount 不用于 SSG，既有 HTML 水合明确 P2 |
| 已核实·足够 | `crates/rustify-style/Cargo.toml` compiler 为原生可选能力；`compiler/public_api.rs` compile/build 接受显式 CSS、assets、limits、候选 | 复用完整 CSS 编译，不复用 wasm facade；新增压缩与本地资源解析 |
| 已核实·足够/有限 | `xtask/src/serve.rs` 有完整严格 CSP、base 规范化、MIME；`tests/theme-export` 已隔离 workspace | CSP 字符串、服务行为测试及隔离范式可复用；现 server 不是通用内容 404/watch 服务，不整文件搬过去 |
| 已核实·缺口 | `tests/theme-export` 的 runtime/generator 默认路径依赖 SDK；`build.sh` 用 cargo-makepad 与 `--locked --offline`，generator 复制根 loader 并读取 THEME_FONTS | 两个包显式开启 gpu，更新其独立锁文件；保留宿主字体目录与打包合同，M1 覆盖根/子路径及两个浏览器 project |
| 已核实·缺口 | `theme/fonts.rs` GPU 字体函数和测试引用 Makepad 资源；`rustify-components/src/lib.rs` 多组件 gate wasm32 | gate GPU helper/资源测试；P1 默认系统阅读字体与内嵌小图标，不复制 GPU 字体目录 |
| 已核实·足够/有限 | `playwright.config.ts` 按 project 起服务；`tests/tier.ts` 分回归/证据；现 config 默认 Chromium | 复用测试分层，新增独立 docs config；现测试不证明 Firefox/Safari/真实手机通过 |

### 1.2 文件拓扑

★ 表示拟新增，不是现有能力。精确文件可在保持职责边界时简化；公共类型和入口由主实施者单写。

| 路径 | 改动 |
| --- | --- |
| `Cargo.toml`、`Cargo.lock` | 移除强制 csr；登记新包；按 D10 定 CLI profile；锁依赖，不动既有版本主线与 Makepad patch |
| `crates/rustify-ui/{Cargo.toml,src/lib.rs}`、`src/{scheduler,listeners,toast,job,region,diagnostics,theme}.rs`、`src/theme/fonts.rs` | csr/ssr/gpu 转发；切断默认 Makepad 依赖；GPU gate、Pace 条件定义、DOM host 调用 |
| `crates/rustify-ui/src/runtime.rs` ★；`crates/rustify-ui/web/{dom-loader.js,dom-runtime.js,dom-runtime.css}` ★ | D9 布局：中性 binding、协议/资源常量与纯 DOM 实例 boot/fatal/abort；`web/loader.js` 仅适配相同中性 hooks 初始化 |
| `crates/rustify-components/{Cargo.toml,src/lib.rs,src/button.rs,src/link.rs,src/icon.rs}` | 禁 SDK 默认 features、精确转发；实际布局组件 SSR 适配；导出已提交组件 CSS 静态常量，不扩展其余控件 |
| `examples/{fusion-basic,component-catalog,property-workbench,data-workbench,vellum,theme-studio,utility-lab}/Cargo.toml` | 明确 csr/gpu；utility-lab 保留 utility；核对直接 Makepad 使用不漏配 |
| `tests/theme-export/{runtime,generator}/Cargo.toml`、`tests/theme-export/Cargo.lock` | 两包显式 gpu，保留现 loader/cargo-makepad/字体读取；独立锁文件同步，build.sh 与 CI 接线复核 |
| `crates/rustify-docs/` ★ | 内容图/Markdown、诊断、SSR theme、原生样式与高亮、资源、输出事务；框架 JS/CSS 随库编入 |
| `crates/rustify-docs-cli/` ★ | 命令、环境 doctor、模板、服务、watch/SSE；依赖库不依赖 xtask |
| `tests/docs-ssg/` ★、`tests/dom-only/` ★、`playwright.docs.config.ts` ★ | 内容/性能 fixture、B 语料生成器与测量 harness（M2 负责）；隔离 DOM、仓外 git 消费者配方与负向编译工程；新 docs 浏览器项目 |
| `xtask/src/main.rs`、`xtask/src/docs.rs` ★、`.github/workflows/{verify,evidence}.yml` | 仓内 DOM fixture/build-bindgen 与 docs 验证入口，复用现 CI 分层；消费者不需要 xtask/mbx |
| `README.md`、`CLAUDE.md`、`docs/quickstart.md`、`docs/architecture.md`、`docs/docs-ssg.md` ★ | 默认行为、GPU 开关、分模式检查入口、安装与 CLI、托管合同及限制 |
| `.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md` ★（默认阅读主题）、`.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md` ★（dev 诊断；文件名由 impeccable 按主目标路径生成）、`DESIGN.docs-ssg.md` ★ | M5 的两份 surface brief、阅读主题规则与 finish review；先设计后实现，不在计划期预填验收 |

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| SDK runtime | listener/toast/job/overlay；两种 boot | 启动先于 mount；每实例一个 signal；fatal 后绝不回调死 wasm | 内部 JS hooks，非公共插件 trait | abort、timer、resize 所有权 | 本地浏览器；GPU 旧 hooks/DOM hooks 两实现 | 相同 DOM 行为与两实例故障隔离 |
| Docs 分析 | check/build/dev | 配置与输入快照 → 内容图+稳定排序诊断；不写发布目录、不联网 | 无 | 路由/锚点/链接、span、草稿过滤、依赖失效 | 进程内；真实临时目录文件 | CLI 输出、诊断、公开分析结果 |
| Docs render | build/dev | 图与发布模式 → 页面/资源；同输入字节稳定 | 无 | SSR、元数据、类集合、原生 CSS/高亮 | 进程内库；不做 browser renderer adapter | HTML、CSP、CSS、资源 hash |
| Docs publish | build | 全部验证成功才提交；一个输出一个写入者，dev 不写生产 out | 文件系统内部接缝 | staging/备份/恢复/并发锁 | 本地真实目录与故障注入 | 旧树字节保全、重跑恢复 |
| CLI serve/watch | 作者/读者 | preview 只读；dev last-good + SSE | HTTP 本地接口 | 路径/MIME/404/CSP、变更批处理、诊断刷新 | 本地文件系统/HTTP；外链检查是显式外部依赖 | HTTP 响应、浏览器状态、端口回退 |

不引入远程数据库、队列、统一插件总线或通用文件系统 trait。删除内容图会把地址、校验和失效规则散回三个命令，故值得集中；CLI 不成为所有业务的转发类。

## 3. 配置、内容与产物模型

以下是新增 v1 契约，定义在库 serde 类型与 CLI 帮助中；未知键报错，未来不依靠静默忽略配置升级。

品牌字段按 D11 的浅深成对形状冻结；不发布单色形状。

| 对象 | 关键字段与约束 | 生命周期/权威来源 | 查询与索引 |
| --- | --- | --- | --- |
| 站点配置 | `schema_version=1`；site.name/description/lang；site.url 可缺；site.base 规范为 `/` 或 `/segments/`；site.image 可缺（分享图，本地 PNG/JPEG/GIF/WebP，仅有 site.url 时输出）；content.dir=`content`、assets.dir=`static`、build.out=`dist`；repo.url/ref/path/edit_url；theme.primary 可缺或 `{light,dark}`（两项必填的 #RRGGBB）；theme.logo 可缺、同源本地路径或 `{light,dark}`；nav.groups/top/social；dev.port=4188（拟新增默认值，避开现浏览器配置4173–4187，冲突自动回退） | 作者 TOML。路径相对工程根，logo 两项均进资源图；out 不得为根/输入目录或其祖先，不追随出界 symlink | 一次读取并验证；base 的 URL 生成全图共用 |
| frontmatter | title、description、lang（缺省取 site.lang）、sidebar.label/order/group、draft、index（默认 true，搜索引擎索引声明，独立于 search/ai）、search、ai、toc.levels；首页 `layout=home`、tagline、actions、features | YAML；lang 为有效语言标签；title 可取首个 H1，无两者则报错；search/ai 仅保留作者声明，本期不产索引。首页 action 为普通链接，支持两个主操作 | 路由由文件派生：index.md → 同目录 `/`；其他 .md → `/<stem>/`；规范化后的重复、404 保留路由冲突报错 |
| Page/Heading | 工程相对源路径、源码 span、route/title/lang、描述、草稿、标题层级/id、渲染片段、代码原文、依赖 | 分析阶段派生；一页只呈现一个 H1（复用正文标题或由 frontmatter 生成，不重复） | route→Page、Page→Heading id、文件→依赖者、Page→类集合；内存 map，一次整批建图 |
| 资源 | 来源相对路径、字节 SHA-256、MIME、稳定输出路径 | 页面或主题引用的文件（含 static 中被引用者与 logo）一律按内容指纹进 `assets/files/`；static 目录另按原路径发布供外部已知地址；框架生成文件指纹化；生成器保留 `assets/{site,mode,share}.*` 与 `assets/files/` 前缀及站点根文件名，static 占用即报错；拒绝秘密路径和出界符号链接；删除引用后不再复制内容附件 | 内容地址的资源图，引用者随内容 hash 更新；不按 mtime 判相等 |
| 产物 manifest | schema_version、mode（必须 production）、base、cli_version、页面/文件相对路径及 digest、build_id | 从输出字节派生；不含本机路径、环境变量、时间戳、随机值；cli_version 的独立版本规则见 §8 | preview 必须读取，非 production/未知模式与 base mismatch 拒启；部署文件清单与回滚识别 |
| dev 状态 | 输入快照、依赖图、last-good 完整代、递增 revision、诊断、当前请求页、片段缓存 | 全部在进程内；不写 build.out、不持有生产输出锁，不是持久业务数据 | 增量失效和 SSE；开发消息与草稿不进生产 manifest，允许 dev 运行期间另行 build |

内容不存在时不造成功的空白站：new 保证首页与至少两页；已有工程无可发布内容报可修复诊断。导航配置引用未知页、跨出内容根的“内容 URL”均需定位错误。

### 3.1 兼容与迁移

- SDK 是显式行为变更：7 示例与 theme-export 两包同一阶段全部 opt-in，quickstart 与 GPU API 文档同改。保留 GPU 开启后的 API/行为；DOM public helper 继续由 SDK 导出。
- root workspace 不强制所有调用者选择同一模式；DOM/SSR 独立消费者各自选择。不得删根 patch 来“证明无 Makepad”，检测的是解析依赖图，不是含其他包的全仓 Cargo.lock 文本。按 PRD N4/AC-1 的包名规则拒绝 `makepad*`、`rustify-makepad`、`cargo-makepad`；`makepad/libs/bitflags` 与 `makepad/libs/smallvec` 是已登记通用库副本，可出现于根 workspace 的解析路径，不能用路径含 makepad 的 grep 判失败。V-1/V-11 附包名/来源清单，并逐文件对照 sources.lock.json 核对这两处无新增漂移；`sources verify` 的成功退出本身不证明零漂移。仓外消费者不复制根 patch，也必须通过。
- 新工程配置 v1，无历史数据迁移。首次发布即带版本与托管合同；旧版 CLI 不支持新 schema 时清楚拒绝。
- 产物只整套替换/回滚，不混发 manifest 和资源；回滚 SDK default 变更须一起回滚示例依赖与文档。没有数据库 schema 或不可逆回填。

## 4. 集成契约

### 4.1 GPU 默认关闭与 DOM boot

中性 host 的方向沿用 ADR-1；以下资源/配方是 D9 的已定合同，T-1/T-2 按此实施。

- `gpu = [dep:rustify-makepad, csr]`，GPU 模块/资源入口同时 gate `gpu` 与必要 target 条件。components 提供 `gpu → rustify-ui/gpu` 便利转发，默认不启用。`utility` 的 GPU resolver 同样 gate，不让仅 DOM utility 间接引用 GPU API。
- SDK 中性 hooks 包含 signal、defer、defer_after、observe_resize 和实例标识；对应 SDK 启动函数在 mount 前校验，未初始化返回明确 startup 错误，不让 `unwrap_or_default()` 静默丢失 abort 保护。GPU loader 将现 hooks 注入中性 SDK 和 Makepad；GPU region 仍走旧 host。
- ★ SDK runtime 的 DOM 工厂通过 `#[wasm_bindgen(module="/web/dom-runtime.js")]` 引用 crate 内 JS，bindgen 自动带出 snippets；模块只定义工厂、不保留实例可变状态、不依赖其他模块 import，遵循 [JS snippets](https://wasm-bindgen.github.io/wasm-bindgen/reference/js-snippets.html) 的 crate-relative/target-web 合同。SDK 以 `#[wasm_bindgen]` 直接导出 `rustify_runtime_protocol`、`rustify_dom_create_host`（仅未开 gpu 时编译）、`rustify_runtime_boot` 三个原语，依赖 SDK 的应用自动带出，无需薄封装。loader 读取 SDK 协议、创建带相同协议号/instance_id 的 hooks，经 Rust boot 再校验后才调用应用启动入口；缺导出/协议不一致明确拒启并释放本次 host，不能回退裸 mount。GPU boot 也校验同一中性协议，不更改其 bridge hash 合同。
- ★ `rustify_ui::runtime::{DOM_LOADER_JS, DOM_RUNTIME_CSS}` 在宿主目标也可读取，components 提供 `CSS` 常量，编入现有 `crates/rustify-components/css/rustify.css`。模板内 `export-dom-assets` 仅用 Rust 标准库把 loader/基础 CSS/组件 CSS 写到 public/，资源来自与 wasm 相同 SDK/components rev，不要求作者访问 Cargo 缓存、手抄 snippets 路径或调用仓根 xtask。小型应用外部 app.js 与 HTML 由 fixture 提供，所有脚本同源，遵循现 Strict CSP，不用内联脚本/style。
- DOM loader 只导入普通 wasm-bindgen glue 与中性 JS，不 import message_bridge、embedded、Makepad env，不校验 Makepad schema、不碰 `window.makepad_resource_base`。编译共享模块、独立 wasm 实例和带实例 query 的 glue 维持现有隔离方式。
- DOM fatal 顺序：标死 → abort listeners → 释放本实例 URL 标记与登记的页面资源 → 取消 timer/deferred/observer → 通知页面。export boundary、DOM 事件 error/rejection 的 glue URL 归属与既有 loader 一致；非 fat-LTO 构建的 panic 在 V8 默认 10 帧内到不了 glue 帧，DOM loader 仅上调（≥200、不下调）`Error.stackTraceLimit` 保住归属；普通 Rust Result 错误不升级为 trap。失败提示由活 JS 显示，重启沿用现有最多 3 次限制。
- mount/dispose 的 Owner cleanup→DOM unmount、浮层焦点回归与 URL 所有权不改；resize 回调必须 abort 后断开，toast timer/job 不能进入已死实例。GPU 开启时 `Pace` 保持原 reexport，`accept/submit` 与 RegionApp 的 Makepad 类型签名不改；默认 DOM 枚举仅在未开 GPU 时编译，避免 trait 返回类型不匹配，也不改变流合并规则。
- ★ `tests/dom-only/` 独立 workspace 通过路径依赖构建默认配置；新增仓内 `mbx xtask build-dom --manifest-path … --output …` 使用普通 `mbx build --target wasm32-unknown-unknown` 加现有同版本 bindgen 库，不调用 cargo-makepad。另将完整 consumer 复制到仓外临时目录，SDK/components 只用 file:// git 候选 rev，根 patch 不复制。M1 可将本次相关源码快照放进临时 git 仓并提交成验证候选，记录 SHA/文件指纹；这不替代 M1 检查通过后的主树提交。M1 的 V-1/V-2 执行下面同一配方，M9 用已授权推送的 GitHub rev 重验。

普通消费者配方（拟交付 fixture 包名 dom-consumer；应用库为 cdylib，另含 export-dom-assets bin）：

```sh
cargo install wasm-bindgen-cli --version 0.2.128 --locked
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--cfg=web_sys_unstable_apis" cargo build --locked --release --target wasm32-unknown-unknown --lib
wasm-bindgen target/wasm32-unknown-unknown/release/dom_consumer.wasm --target web --out-name app --out-dir public/pkg
cargo run --locked --bin export-dom-assets
```

`public/pkg/` 整目录保留，包括 snippets 和 app_bg.wasm。loader 参数显式给 glue_url/wasm_url（本例 `./pkg/app.js`、`./pkg/app_bg.wasm`），不写死 bindgen.js；编译模块可缓存，glue 用实例 query 隔离状态。app.js 在 init 后依协议→工厂→boot→应用 start 顺序执行；应用不能在 wasm-bindgen start 中提前 mount。配方的 cfg 保证 clipboard::available() 为真，实际成功/拒绝仍按权限合同。消费者不需要 Node/mbx/xtask/Makepad；V-2 浏览器由验收 harness 提供，不把浏览器服务工具写成作者编译条件。

### 4.2 CSR/SSR 特性与外部依赖

分模式与平台约束按 D10 与 C-6 执行。

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| Leptos =0.8.22 | 已核实·缺口 | 根 Cargo.toml 强制 csr；本机 crate Cargo.toml 分别定义 csr/ssr/hydrate/islands | workspace 不带渲染模式；SDK/components default=csr，ssr 精确转发；components 对 SDK 必须 default-features=false，docs 对两者禁默认并选 ssr。独立 invocation 验证模式，P1 不开放 hydrate | 只在 docs 单包图/SSR HTML 测试断言无 csr/effects/gpu；不在 SDK 写 csr+ssr 互斥 compile_error |
| 默认组件 | 已核实·缺口 | `button.rs` prop:disabled；`lib.rs`、theme provider 有 wasm gate | Button 的可序列化状态写 HTML attribute，保留浏览器受控语义；Link/Icon 只补实际布局缺口。不用 Slider/Progress/ThemeScope 承担 SSR 壳 | V-4 HTML 断言失败不得进入主题交付；其余组件按 P2 处理 |
| rustify-style | 已核实·足够/缺口 | public_api compile/build 接收显式 Stylesheet 与 Limits；build 累积候选 | 配置 compiler；内置 Tailwind 4.1.13 CSS 输入与 SDK CSS 作为虚拟 assets，使用显式来源。CSS 输入/全局主题改变或候选删除时重建 compiler | Syntax/MissingAsset/Limit 映射相对源诊断；不得忽略缺类或绕过限制 |
| 新依赖 | 新增，不声称已集成 | pulldown-cmark 本机 0.13.4 源码提供 GFM/heading/footnote/table/task 等选项；syntect 本机 5.3.0 提供 fancy-regex 可选后端；Lightning CSS 文档见 ADR-3 | M2 锁版本/许可；syntect 用预编译 grammar dump，缺语言补最小许可语法。正则后端与外链 HTTPS 客户端优先纯 Rust（fancy-regex、rustls/webpki 根），选 onig 或系统 TLS 时在 M2 记录中写明理由；Markdown/YAML/tokenizer/watch/HTTP 选最小维护中依赖，tokenizer 需 span | macOS 上不引入 Rust 稳定版与平台 cc/链接器之外的系统库或工具（如 OpenSSL、pkg-config、CMake、NASM）；完整 normal/build 图和 macOS 安装不满足则局部替换，不用安装工具兜底 |
| git 元数据 | 已核实·足够/有限 | PRD DEP-8：非 git 仓库省略更新时间 | 一次批量读文件历史、缓存文件时间映射，UTC 固定 YYYY-MM-DD 呈现，不用主机时区/构建时间/mtime。消费者缺 git 或历史不可用只省略并警告，不作为额外必装工具 | 内容仍可构建；本机绝对路径不可写入页面 |

CLI/框架库的最小工具链 gate 必须逐包独立解析完整含 build-dependency 图；新增包不用 xtask 做依赖。P1 CLI 只支持 macOS（C-6），V-11 在实际 macOS 原生环境记录 OS/架构/Rust 版本并跑 git 安装和五命令；Linux、Windows 未验证、不承诺，文档与 CLI 帮助如实写明。受限 PATH 保留平台 cc/链接器等正常 Rust 编译条件，不把这些基础条件误写为 CLI 额外依赖；消费者命令运行不调用它们。

M2 将新 docs 包加入 workspace 时，同步 CI/README/CLAUDE 的矩阵入口。CSR/GPU 组用 `mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins` 及同样 exclude 的 clippy；文档库和 CLI 各自单独 invocation。SDK/components 分别测 default、gpu、no-default+ssr 三种组合。只配置 default-members 不能修复 --workspace 的合并；不以全 workspace csr+ssr union 作为支持合同，显式分组排除，而 SDK 仍能被普通 Cargo 合并解析。具体命令见 §9.2/V-15。

## 5. 构建、诊断与开发机制

### 5.1 命令合同与前置闸

通用参数：`--config <path>`；诊断命令支持 `--format text|json`、`--deny-warnings`。JSON 模式 stdout 只含一个完整 JSON 对象 `{schema_version, success, diagnostics, summary}`，进度去 stderr；诊断字段为 severity/rule/file/line/column/message/suggestion，适用位置从 1 开始，非源码故障可为 null 并说明。成功退出 0；任一错误/提升后的警告非零。

| 命令 | v1 语义 |
| --- | --- |
| `new <dir> --template content` | 校验工具链 → 校验空目录 → sibling 临时目录生成配置、首页、至少两页、static、README、.gitignore → 成功提交；不存在或空目录可用，非空拒绝。P2 模板选项未交付时明确拒绝 |
| `dev` | 校验工具链/配置 → 首次分析构建 → loopback 服务/watch；有内容错误仍提供可关闭诊断页，修复可继续；不自动开浏览器、不编译站点 Rust |
| `build [--base <path>] [--out <dir>]` | 覆盖值先验证；分析→渲染→检查生成输出→staging 发布。结果最后输出页数、总字节、最大 5 页、警告数；失败最后输出错误数与首错 |
| `preview [--dir <dir>] [--base <path>] [--port <n>] [--host <address>]` | 产物只读，核对 production mode/base；默认 loopback:4189，端口冲突回退并打印实际地址，--port 0 由 OS 分配。显式 --host 供真机只读访问，打印监听地址；不 rebuild/watch，不混入 dev 脚本，同 out 的 build 并发规则见 §5.4 |
| `check [--external-links]` | 与 build 共用全量检查、无发布写入。默认离线；显式外链检查才发送 HTTP(S)，有超时/有限并发/去重，网络失败定位为外链诊断，不伪造成功 |

`new/dev` 按工程实际能力做 doctor，不自动补装：P1 内容模板只检查 `rustc --version` 能取得稳定工具链版本，不因缺 wasm32 目标而拒绝；缺 rustc 时给出 rustup 安装入口及 `rustup toolchain install stable`。这是启动前置检查，不执行 Rust 编译；缺组件 fixture 使用“已装 CLI、rustc 被移出 PATH”的环境，两命令均在写文件/watch/监听端口前失败。SDK 的 DOM wasm 示例和后续组件库模板才检查目标并给出与当前 toolchain 一致的 `rustup target add wasm32-unknown-unknown`。已安装 CLI 的纯内容命令不运行 cargo 编译；build/check 不需要网络，preview 不需要浏览器。仓内可走 mbx，不把 mbx 变成消费者必装工具。

模板 .gitignore 忽略默认 `dist/` 与任意层级 `.rustify-docs/`；自定义 out 的忽略写法由生成 README 说明，不擅改既有工程的忽略规则。生产私有构建区固定为 out 父目录下 `.rustify-docs/<输出标识>/`（标识按规范目标路径派生，仅用于本地）：锁、staging、backup、事务日志均在此处，与 out 同文件系统，不进入发布白名单和 watch 输入。V-5 验证 new→build 后 git status 不含默认产物与私有文件。

### 5.2 单一内容图、可定位语法与安全检查

1. 收集配置、内容、引用附件与 static 的规范相对路径快照；排除输出/私有构建区/watch 自身，排序一致。发布图剔除草稿；dev 图保留并展示草稿标记。发布页链接到不发布的草稿在 build/check 中是错误，在 dev 中是注明“生产不可发布”的警告。
2. YAML frontmatter 先分离并保存 offset；Markdown 用 offset iterator，显式开启 tables/tasks/footnotes/strikethrough/GFM 与 `ENABLE_HEADING_ATTRIBUTES`；标题属性只接受 `#id`，`.class` 与任意其他属性定位报错。统一事件 writer 输出标题 id、五种 alert、外链 rel 与表格对齐 class（text-left/center/right），不直接采用会输出 style 属性的 push_html。自动链接补足 GFM 裸 URL/email 测试，不能假定 parser 的 GFM 开关包含全部自动链接。
3. 标题 id：Unicode NFKC、Latin 小写、保留中文与文字数字、空白连字符、去标点；空结果用 section；按文档顺序避碰追加 `-1/-2`。自定义 `{#id}` 优先；显式 id 重复报错，自动 id 绕开所有显式占用。标题/链接断言跨构建稳定。
4. 相对 `.md#fragment` 从源文件目录解析，再查目标 Page/Heading 转 URL；图片随资源图复制。内容目录外的仓库文件只在存在且配置 repo.url/ref/path 后映射网页；不能从当前机器路径猜 URL，缺配置/文件报错。顶栏、首页、HTML 中资源与链接也经过图验证。
5. 代码围栏保留原文另存用于复制，高亮 HTML 不反推代码；info 支持语言、`title="…"`、`linenos`、`highlight="1,3-5"`，非法范围定位围栏。分组用 GitHub 可忽略的 `<!-- rustify:code-group name="…" -->` 与 `<!-- rustify:end-code-group -->` 包围普通围栏；同名同标签集合才同步，否则诊断配置冲突。
6. 片段用 `<!-- rustify:include path="src/example.rs" region="example" lang="rust" -->`；工程内源文件以成对 `rustify-region:example` / `rustify-endregion:example` 注释标记，输出排除标记行但保留中间原始字节/换行。只读当前文件、不编译；缺区间、重复/不闭合、出界路径在 Markdown 引用位置报错。该语法文档提供 GitHub 可读占位说明，不提供 MDX。
7. HTML tokenizer 同时校验原始 HTML 与最终产物：拒绝内联可执行 script、on* 属性、style 属性/元素、javascript URL；大小写、实体、SVG/嵌套及畸形 HTML 均测试。资源 URL 按严格 CSP 的上下文校验：Markdown 图片与 HTML img/srcset/source/video/audio/poster、SVG 引用、CSS url/@import 等不得请求跨源资源，定位报错并建议放入 static/；img 的合法 data: 图片沿 CSP 允许，media/font/script/style 仅允许可解析发布资源。iframe/object/embed 被策略禁止，同源也报错。普通站外 a 链接不受资源规则影响，新窗统一 noopener；所有插入文本/属性正确转义。
8. 全图分析完收集独立错误，排序并去重，不因首错丢掉其他已知诊断；失效锚点/缺图/内联 script fixture 必须恰为三条错误。描述缺失、导航遗漏、alt 缺失、标题跳级为警告；禁联网为默认，无“修复内容”副作用。

### 5.3 产物、元数据与生产预览

- 首页及内容路径用稳定目录 URL；404 固定输出根 `404.html`。页面 head 含唯一 title、description（缺时正文首段摘要）、Open Graph/Twitter 信息、lang；site.url 已配置才产 canonical/sitemap；缺 URL 警告并省略两者。分享图使用作者配置的已发布本地资源或框架内置同源默认图，不伪造远端 URL。title 模板冲突须稳定消歧，空正文没有首段时要求显式描述。
- sitemap 恰为已发布且 `index=true` 的普通页面，排除草稿、`index=false` 与 404；`search=false`/`ai=false` 不能替代该声明。`index=false` 页面输出 noindex 标记；site.url 存在时 robots 的 Sitemap 行为带 base 的绝对 sitemap 地址，无 URL 时不输出不存在的指向。robots 不泄漏源文件路径；git 文件更新时间按 §4.2，非 git 不报致命错误。
- 统一 URL builder：base 与 route 按 URL 规则编码，不以文件系统 join 代替；不输出 `<base>`。404 的资源/首页链接必须根相对并含 base，深度不存在地址不改变解析结果。
- 页面 route 名稳定；内容指纹用于 CSS/JS/图片等资源，不改变文档 URL。资源图自底向上改写引用再 hash；资源改变只使自身及引用链上的资源重命名，引用页面更新内容而保持路由。无关资源名字不得改变，未变重建名字相同；manifest 不包含自己 digest 的递归。
- build 只向 out 提交 mode=production 的 manifest；dev 的完整代只在内存服务。草稿、源 Markdown、未引用源码、诊断通信端点和 dev-client 均不进 production。文件清单就是发布白名单，不递归复制工程；preview 拒绝缺少或不是 production 的模式。
- preview 使用库产物元数据和实际目录服务；基路径下找不到资源/页面返回 404，不用 SPA fallback；页面错误返回站点 404 HTML，资源丢失保持相应错误而不伪装 JS 成功。GET/HEAD、URL 解码、目录入口、MIME 与 path traversal 负向测试齐备。
- header CSP 与 `xtask/src/serve.rs::CspMode::Strict` 逐字一致；所有页面的 meta 是相同策略仅去除 frame-ancestors，位于资源加载之前。fixture 分别用完整 header 与仅 meta 的托管走查；无需 COOP/COEP。

### 5.4 样式、高亮与发布事务

- 从框架模板/Markdown writer 和作者原始 HTML 收集实际 class，合并框架 JS 可能切换的固定状态类；高亮节点及代码块文本不参加候选提取。原生 compiler 一次构建共享 CSS，不扫全仓、不 ship 整个 SDK 预编译 CSS。全局候选增加可缓存，删除必须重建；Limits 按实际输入/候选规模设足够且有界的工程值，写入测试与记录，不沿用 Worker 的交互小配额来拒绝 500 页。
- 高亮语法集构建期载入一次，使用 `ClassStyle::SpacedPrefixed { prefix: "hl-" }`；11 种语言、全部 grammar context 的正则预编译与未知语言纯文本回退都测。浅/深 token 配色单独生成外部 CSS，不写 style；V-4 断言候选无高亮原子，M5 验含 meta.block 的 span 仍为 inline。复制按钮读取独立原始文本节点，输出规范化不改变复制内容。
- 同一规范 out 共用私有区锁文件：build 持独占 OS 锁，preview 会话持共享读锁；无法取得锁时立即拒绝并提示先停止该 out 的 preview/build，不等待或读取交换中的目录。dev 不取得这个锁，运行期间允许 build。staging 位于 §5.1 的同文件系统私有区；全部渲染/资源验证成功才进入交换，绝不先清旧目录。
- 交换用旧 out→私有区固定 backup、staging→out；第二步失败恢复 backup。锁/交换日志/备份不进入产物；build 启动在锁内恢复未完成事务，再处理新构建，preview 遇到未恢复事务拒启并提示运行 build。首次构建失败无新 out。kill、磁盘满、权限失败、损坏 manifest 与并发均有注入；不删除无法确认归属的目录。
- 不声称 portable 两次 rename 是零窗口原子替换：它保证失败可恢复和旧字节保全，不承诺外部服务器在交换间零停顿。dev 请求持有内存完整代快照，成功后切换指针；preview 只维护产物外的锁文件，不修改产物，持锁期间同目标 build 明确拒绝。V-9 验证 dev/build 可并行，V-10 验证 build/preview 的互斥与锁释放。

### 5.5 dev 失效、错误恢复与滚动

- watch 所有内容、配置、static 和已引用片段源文件；一次更新事务内使用同一输入快照。结构变化重算路由/导航，配置变化按影响失效全站，单页正文改变只更新本页和实际反向依赖；图片改变刷新引用页，删除文件重新诊断依赖者。P1 的“源文件修改”只重读片段，不做 Rust 编译。实现上每轮重读全部输入快照、按内容键复用片段，受影响页面由新旧两代输出逐字节比较（含删除）确定，反向依赖因此不另建显式图，也不按 mtime 判断；失败的一轮（即便诊断相同）推进 revision；成功且字节与诊断均未变的一轮不推进、不广播。
- 正文/高亮片段按内容与引用片段字节 hash、grammar 版本、解析/高亮选项及语言缓存。配置只改布局/导航/品牌时复用片段，仍重渲全部受影响页面与样式后整代提交；改变解析选项时按键失效。共享不可变语法集，页面渲染使用独立 Owner，按 available_parallelism 限制工作数，不跨线程共享可变 Owner/compiler。M2 用 T-8 测 500 页配置热渲染余量，M6 用真实 watch/SSE/浏览器复测 ≤1 s；不能只提交当前页来绕过完整代合同。
- 单写入队列归并同时保存，构建期间的新 revision 排到下一轮，不丢最终快照。native watch overflow 时重扫相对路径/字节指纹，不要求重启；SSE 重连读当前 revision 与错误状态。
- D10 的 docs-cli profile 必须编为 panic=unwind。每代分析/渲染外层 catch_unwind，把 panic 转为一条“本代构建失败”的诊断，不换 last-good、不退出服务；丢弃该代 Owner/可变 compiler/cache 更新，下一代从已提交片段/语法数据重建，不能复用已损坏的可变对象。CLI 在 panic=abort 的误用构建下明确拒绝 dev 并给出正确安装命令；单测通过不算证据，M6 用该 profile 的真实二进制注入 panic，M9 用安装后二进制重验。普通语法错误走 Result，OOM/OS 强杀不承诺进程内恢复。
- 成功 SSE 带 revision 和受影响 route；只有当前页受影响才刷新。刷新前用 sessionStorage 保存此 route 的 scrollX/Y 与分组选择；next load 等内容布局稳定恢复，字体默认系统避免迟加载。配置改变 page 结构时恢复可达位置，不伪称内容高度不足仍能保留原 y。
- 分析/渲染失败：不提交新代，终端与浏览器展示同一诊断；last-good 可继续阅读。诊断恢复消息即使当前页不刷新也清掉浮层；首构建失败显示诊断壳而非假成功首页。关闭浮层后保留键盘可达的“N 个错误 · 显示上一个可用版本”标记和展开入口；再次保存失败时更新数量/revision，即使诊断相同也保留状态，修复后两者均清除，不抢阅读焦点。
- loopback bind 默认读 `dev.port`，`--port` 可覆盖，测试用 `--port 0` 让 OS 分配；默认端口冲突回退空闲端口并打印实际地址，不 kill 他人程序。docs Playwright 使用独立配置与实际端口传递，不占现有 4173–4187 项目端口。

## 6. 默认主题与基础交互

**方向：阅读型文档，不是 Theme Studio 工作台。** 沿用 DESIGN.md 的语义 token、紧凑导航和控件语言；正文保持安静，以层级、行宽、代码与 alert 区分信息，不铺装饰卡片。系统 sans 正文、系统 mono 代码，不增加在线字体依赖；内置图标按文件/静态 markup 随产物。

| 页面/区块 | 信息架构与交互 | 关键状态 |
| --- | --- | --- |
| 首页 | 站点身份、真实定位、两个普通链接主操作、特性网格；1280 px 主操作首屏可见 | 配置缺项诊断；长标题/无 logo 有文字身份；不出现演示/搜索伪入口 |
| 内容页 | 跳到正文、页头/顶栏、侧栏树、标题与正文、默认 H2/H3 目录、上一页/下一页、编辑与更新时间 | 当前页/分区高亮；侧栏加载后当前项可见；无标题的目录不占空栏；dev 草稿页头与侧栏项均有文字“草稿 / Draft”，生产不显示 |
| 404 | 相同壳、一个 H1、返回首页；任意深度地址资源可用 | 真 404；不调用搜索，不依赖 wasm |
| 代码 | 标题/语言、行号/指定行、可选择原文、复制；分组按 label 切换 | 成功/拒绝均 aria-live 状态；拒绝后手工选择仍可用；同名分组选择跨页保持 |
| dev 浮层 | 错误数量、相对路径与行列、说明/建议、关闭操作及关闭后的错误标记/展开入口 | 构建中保持 last-good；重复错误保存仍有标记；修复自动清除；异常长路径/多条错误可滚动；无 last-good 时用诊断壳说明 |

- ≥1280 px 三栏，768–1279 px 两栏、目录用原生 details 折叠，<768 px 单栏、侧栏以原生 details 为无脚本兜底并可增强。375/320 px 和 200% 放大无页面横滚，代码和表格只在自身容器内滚动。
- 正文 near 16px，行高与间距继承现有阅读风格；用中/英文实际样本核对英文 60–80 字符、中文 30–45 字的行宽，不凭 `ch` 单位宣称达标。heading scroll-margin 避开固定页头；目录 scroll spy 选视口顶部章节，首个/末个与短章节有稳定规则。
- theme-mode 同源外部小脚本在 head 同步执行、CSS 首次绘制前读以 base 隔离的 localStorage 键；取值仅 system/light/dark；存储拒绝不影响切换，系统模式用 matchMedia/CSS media。无脚本按系统 CSS 呈现，模式控件清楚不可交互而非假按钮。
- 静态 html 带 `data-rustify-scope data-theme="system"`，无需 mount 才有 token。文档虚拟 CSS 输入移除 SDK 的 `@source "../src"`，显式候选由 §5.4 提供；文档 dark 变体覆盖显式 data-theme=dark 和 prefers-color-scheme:dark 下的 system 根，显式 light 不受系统深色影响。同步为两种深色路径写 token，不改 SDK 的作用域语义和默认 CSS。
- 阅读主题默认浅色 primary/ring=`#175cd3`、background=`#f9fafb`、按钮前景=`#ffffff`；深色 primary/ring=`#53b1fd`、background=`#0c111d`、按钮前景按 D11 规则取黑白中对比较高者（`#000000`）。沿用其余语义 token 后逐项验正文、次级文案与高亮配色；这些是拟实现值，不是浏览器实测。Button/Link 通过调用方 class 与 tw_merge 覆盖半透明 focus，使用 3px 不透明指示与 2px 间隔，验指示与每种相邻表面 ≥3:1；链接/普通文字 ≥4.5:1。hover/active 同验，避免 alpha 状态降低对比度；不修改全 SDK 默认值。
- 基础 JS 不引入 Leptos/wasm/runtime compiler：主题、复制、代码分组、移动菜单增强、目录高亮。无脚本时所有代码分组全部可读、导航和目录都是锚点；脚本失败不藏正文，不留不可达代码。
- 页面 html 的 lang 取 frontmatter.lang 或 site.lang；界面目录按语言主标签选择 en/zh（包括跳到正文、复制/成功/失败、目录、上一页/下一页、主题、草稿与诊断）。其他有效语言标签保留正文 lang，界面回退英文并在界面容器标 lang=en；P1 不增加语言路由或语言切换。V-13 同时检查中文页、英文页与回退页的文案和 lang。
- 本期布局只用 SSR 安全的语义 HTML 与 Button/Link/Icon；不挂 ThemeScope/OverlayStack 来渲染静态页面。主色/logo 从配置生成外部 CSS/本地资源，亮暗两态焦点、链接、按钮同步；不在 HTML 塞 style 或内联启动脚本。
- D11：primary 浅深成对，缺省用本节阅读主题值；logo 单路径在两态复用，成对时 CSS 控制对应资源的可见性，无脚本也跟随系统。按钮前景按每态 primary 在黑/白中选择较高对比者；链接、focus 和按钮各态/hover 的计算值不满足 §7 时 check 定位配置警告，deny-warnings 非零。颜色格式非法、缺成对字段、缺 logo 是错误；低对比色不静默改值。V-8/V-13 含有效品牌色与故意低对比 fixture，logo 是否清楚仍用实际浏览器核对。
- 默认主题浅深对比度、可见 focus、地标、lang/一个 H1、reduced-motion 均验。主题/复制/分组/菜单的键盘质量本期验证；完整搜索/语言/演示键盘与 VoiceOver 终验仍按 N5 分期，不能提前标整条通过。

M5 在 T-6/T-7 前先用 impeccable 完成默认阅读主题与 dev 诊断的两份 surface brief，记录标题层级、正文/侧栏/目录宽度、页头、代码/alert、间距对齐、模式、动效、键盘及空/错误状态；阅读主题规则在实现与 finish review 后由 impeccable documenter 依实际产物写入 DESIGN.docs-ssg.md（不先写规则再实现）。用户 2026-10-10 确认：首页为安静的文档入口（身份、一句定位、两个主操作、特性网格，无插画/装饰卡片）；代码块随模式用浅/深面板；dev 诊断为右下浮动面板、可收成键盘可达标记。实现后在同一产物基线上完成 finish review，M6 按该 brief 集成实际诊断状态并验证。不以既有 Theme Studio 的设计或本节文本代替这些记录。

## 7. NFR、安全与运行保障

预算来源仅 PRD 与 C-5；下列值是验收合同，不是现状实测。无前后性能数据的改动不能声称优化成功。

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD N1 测量合同，当前无 Docs 基线 | wasm 0；LCP≤2.5s、CLS≤0.1；HTML+CSS+JS gzip -9 总≤100KB | 优化实际瓶颈；禁止改口径/阈值掩盖 | 静态 HTML、系统字体、原生小 JS、原生压缩/只用到的类 | V-14；V-8 无 wasm基础交互 |
| WORK-N2 | PRD N2，当前无 Docs 基线 | B 站内容/配置/资源保存可见≤1s；new→可打开首页≤60s；500页空输出完整构建≤20s | 保存完整测量；不能自动放宽，修改合同须用户裁定 | 内容 hash 片段缓存、共享语法集、有界并行/独立 Owner、整代提交；不启动站点 cargo | V-14 |
| WORK-N3 | PRD N3、现严格 CSP | 根/子路径、真404；两种CSP零违规；默认仅同源；指定最新浏览器 | 有缺陷或未测环境不得结案 | URL builder、外部启动脚本、生产 preview 与独立静态托管 fixture | V-7、V-12 |
| WORK-N4 | PRD N4；SDK当前强依赖 | 默认依赖/产物无Makepad；Rust+CLI环境全部命令可用；离线构建 | 依赖泄露立即阻止发布，不以dead-code删产物代替 | 可选GPU、分包/隔离图、原生库、内编资源 | V-1、V-11 |
| WORK-N5 | PRD N5与DESIGN.md | 对比度4.5:1/3:1；一个H1/正确地标lang；320px/200%；AA重大严重0 | 缺陷修复；作者警告由deny-warnings决定失败 | class-based高亮配色、语义HTML、可见焦点、reduced-motion | V-13 |
| WORK-N6 | PRD N6 | 双构建字节同；失败旧产物不变；完整诊断；1000保存不重启 | 不提交不完整代，重启恢复事务 | 内容hash、无构建时钟、输出锁/staging/backup、last-good | V-10、V-14 |
| WORK-N7 | PRD N7 | 白名单发布；无秘密/机器路径；违例HTML阻断；无跟踪/外部字体 | 错误定位、拒绝发布，不静默清洗或放宽CSP | tokenizer、相对路径诊断、资源图、敏感路径拒绝 | V-10、V-11、V-7 |

- 产物拒绝 `.env`/其变体、`.git`、源 Markdown、未引用源码；即使 static 内误放也不得复制。部署白名单和禁止路径先于 copy。HTML/resource URL 默认不主动联网；包含作者显式外链不等于框架默认请求外部资源。
- 原始 HTML 并非不可信代码沙箱，但框架不能发布 CSP 违例；实体/大小写/畸形/嵌套绕过需负向测试。服务 URL decode 后再 canonical containment 检查，防 `..`、编码分隔符和 symlink 出界；dev 只绑定 loopback，不当公开生产服务。
- CLI 终端可展示本机错误上下文，发布文件绝不携带机器绝对路径、用户名、环境变量值；JSON 面向站点相对位置。源片段只公开作者明确引用的部分，不能整个源文件打包。
- 可观测性最小增量：命令结束摘要、分组定位诊断、build耗时/受影响页数、dev revision 与错误恢复消息；开发计时不进发布manifest。不引入遥测/统计服务器；输出生产尺寸/失败摘要是交付契约。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| DOM wasm trap/boot未初始化 | 单实例 | 实例内状态失去；其余实例保留 | JS失败提示，静态宿主仍可用；不继续点击死控件 | export/error归属、startup检查 | abort清理、受限重启；不得调用死wasm清理 | V-2、V-3 |
| 内容错误/缺片段/违例HTML | 本次构建 | 旧产物不变 | 全量诊断；dev保留last-good | 共用分析规则/JSON | 作者修正后重建、浮层消失 | V-4、V-9、V-10 |
| 写入失败/kill/并发构建 | 输出事务 | staging未完成，旧字节可恢复 | 非零摘要；第二写入拒绝 | 文件锁、交换状态 | 重启恢复backup，重新构建；不能删除未知用户目录 | V-10 |
| watch丢事件/SSE断开 | dev进程 | 可能延迟，不丢最终版本 | 最终更新/当前错误重新同步 | overflow/revision与hash | 重扫依赖、SSE重连 | V-9、V-14 |
| base错/资源缺失/manifest损坏 | 单部署 | 无源码变化 | preview拒启或真实404 | manifest校验/HTTP失败 | 按实际base重建整套部署，不开SPA掩盖 | V-7 |
| localStorage/clipboard拒绝、脚本禁用 | 当前浏览器 | 偏好无法持久/复制失败 | 模式可临时切换、复制失败提示/手选；正文导航可读 | 捕获异常/无脚本fixture | 不删正文不阻塞阅读 | V-8、V-12 |
| 网络不可达 | 显式外链check | 无发布副作用 | 外链诊断；默认build/check无变化 | egress记录/请求结果 | 用户选择关闭外链或修复；不得自动联网 | V-11 |

发布顺序：M1 SDK 与示例兼容收口 → M2–M6 内容链路 → M7 工具链/事务/安全收口 → M8 终验 → M9 文档与可安装提交。`rustify-docs-cli` 单独维护 Cargo package version，不继承固定 workspace version；CLI 源码、内嵌资源或解析依赖改变的验证候选递增版本，`--version` 与 production manifest.cli_version 取同一值。仅文档/证据变化可保留二进制版本，仍分别记录源码 SHA；不为拿 git rev 在消费者 build 时强制执行 git。

D10 的安装合同使用 docs-cli profile，M2 先在临时 file:// git workspace 排练根 profile/patch/lock 实际生效情况，再用于 M6 二进制和 M9 交付；不要拿测试 profile 推断安装结果。M9 的 README、CLI 帮助及生成 README 写入已验证的完整 SHA/CLI 版本，给出安装、升级和回滚命令，先本地候选、再经用户授权推送后验证相同 SHA 的 GitHub URL：

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev <已验证完整SHA> --locked --profile docs-cli rustify-docs-cli
cargo install --list
cargo install --git https://github.com/rockie/rustify-ui --rev <升级候选完整SHA> --locked --profile docs-cli --force rustify-docs-cli
cargo install --git https://github.com/rockie/rustify-ui --rev <上个已验证完整SHA> --locked --profile docs-cli --force rustify-docs-cli
```

尖括号是计划期参数说明，交付文档必须换成实际已验证值。升级/回滚显式 --force，并核对 --version、cargo install --list 的来源/rev 和重新 build 的 manifest；各版本锁文件与 profile 随其对应提交。命令选项见 [cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html)。站点配置不依赖仓库绝对路径。

只承诺通用静态产物与部署说明，不执行上传。说明至少覆盖有 header 托管的完整 CSP 和 GitHub Pages 一类 meta-only/自定义 404 托管；后者不交付 frame-ancestors，真实官方站线上验收属于 P2。产物含空 `.nojekyll`，从分支发布时跳过 Jekyll 构建，见 [GitHub Pages 静态生成器说明](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-github-pages-site#static-site-generators)。回滚整套产物和匹配 manifest，CLI 按上个锁定提交重装；清本地构建缓存后重建，不混用两版本哈希资源。SDK 默认行为回滚需同时回退相关依赖声明和指南。

## 9. 验证

### 9.1 检查定义

★ 命令/入口为拟新增，必须先实现再执行；其余沿用现有入口。新增验证脚本不成为消费者工具链依赖。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | F17/AC-1、5；N4/AC-1–2：独立 DOM/CLI graph 含 build deps，按 §3.1 包名规则无 Makepad；附通用库来源核对；GPU API 负向编译、opt-in 正向构建 | ★ `mbx xtask docs-verify --group features`；隔离 `tests/dom-only/` 与仓外无 patch 的 git 消费者，逐包 metadata/tree 及产物检查 | M1 DOM/GPU 切片；M2 补 CLI 图；M7 重跑；M8 受影响重跑 |
| V-2 | F17/AC-2–3：两 DOM 实例 toast、resize、listener/observer/timer abort；boot 遗漏/协议不匹配明确失败；trap 隔离与重启上限；clipboard::available() 为真 | ★ docs config `--project=dom-only`；仓内与仓外普通 Cargo/bindgen 的真实 release wasm；仓外仅 git SDK/components 依赖，照 §4.1 配方，不用 xtask/根 patch；与 GPU hooks 同合同对照 | M1 T-1/T-2 汇合门；M8 回归；M9 远端 SHA 配方重验 |
| V-3 | F17/AC-4 全部原回归通过；C-2：7 GPU 示例及 theme-export 兼容基线，不用“失败未增加”替代全部通过 | 既有 build-web/root+子路径；theme-export 的 build.sh/root+子路径及 `mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins`；逐一 regression project，Vellum 用现 config；具体基线/触发见 §9.2 | 91e81b8 改前基线（§9.2）、M1 全范围退出；M2 与后续里程碑有横切变化时，退出或下一提交点前重跑受影响范围；M7/M8 复核触发范围 |
| V-4 | F2/AC-1–3、F3/AC-1、3、F15/AC-1–2，N5/AC-3 标题跳级诊断：SSR/span/GFM/anchor；五种 alert 语义/样式及同份文件 GitHub 可读；自定义 id、表格 class 无 style；高亮 class 不进候选，含 block scope 的 span 在浏览器仍 inline；跨源/被 CSP 禁止资源拒绝；恰 3 错 | ★ `mbx test --locked -p rustify-docs` 与 CLI 集成测试；11 语言/grammar context/畸形输入 fixture；M3 的 GitHub 源格式核对先用本地原文件，M9 对已授权可达文件做 GitHub 预览核对；computed display 在 M5 用 V-8 同页补证 | M2 SSR/样式/grammar 切片；M3 内容全组；M5 样式浏览器切片；M7/M8 受影响重跑；M9 模板与 GitHub 核对 |
| V-5 | F1/AC-1、3、4 的 new/build 侧：成功页数、缺 rustc/非空/失败无残留，模板 ignore 生效；纯内容无站点编译。dev 侧由 V-9 验收 | ★ CLI 集成组；受限 PATH、空/非空临时目录；仅允许 doctor 的 rustc --version，监测 cargo/编译工具执行；new→build 后核对 git status | M4 new/build 切片；M7 复测；M8 受影响重跑；M9 生成 README 变更后复测 |
| V-6 | F2/AC-4 发布侧、F11/AC-1–3、F14/AC-1–2：草稿过滤、元数据缺 URL/non-git、可索引页面 sitemap/robots、指纹反向依赖失效、CSS url 与 logo 引用链、源码不发布；草稿 dev 侧由 V-9 验收 | ★ `mbx xtask docs-verify --group artifacts`；根/子路径 fixture，逐字节/URL/资源图断言；manifest mode=production | M4 build 切片；M7 安全收口（含 M5 新资源）；M8 受影响重跑；M9 模板变更后复测 |
| V-7 | F7/AC-2、4–5；F14/AC-3；N3/AC-1–3；N7/AC-4：遍历全站链接、任意深 404、header/meta CSP、同源请求与 noopener；meta-only 下执行全部基础交互 | ★ docs config 分别串行 `--project=docs-root`、`--project=docs-subpath`、`--project=docs-meta`；preview 与独立无 rewrite 静态托管 fixture | M4 HTTP/preview/base 切片；M5 集成；M7 N7/AC-4 收口；M8 全期部署收口 |
| V-8 | F3/AC-2、4；F6/AC-1；F7/AC-1–4；F8/AC-1–4；N1/AC-1；N3/AC-2：主题首帧/系统、导航、scroll spy、复制原文/拒绝、跨页分组、无脚本/无 wasm；配置浅深主色/logo 在两态可见 | ★ docs config 分别串行 docs-root/docs-subpath/docs-meta；冷缓存、storage/clipboard 故障、JS 禁用与系统深色；1280/768/375/320px；带品牌配置 fixture | M5 主路径与 V-4 computed style 补证；M8 跨浏览器收口 |
| V-9 | F13/AC-1–3；F1/AC-1、3 的 dev 侧；F2/AC-4 的 dev 侧：new 后首页+两页可浏览、缺 rustc 的 dev 前置失败；草稿双处文字标识、草稿链接警告；依赖刷新/保滚动/恢复/SSE/端口；关闭浮层后重复错误仍有标记；dev 不写 out/不锁 out，dev/build 可并行；安装 profile 的 panic 恢复 | ★ docs config `--project=docs-dev`；真实 CLI watch、受控写文件与页内见证；新建/缺组件场景复用 V-5 fixture | M6；M8 正式长跑/主路径；M9 安装后二进制重验 panic 子集 |
| V-10 | F14/AC-4、N6/AC-1–3、N7/AC-1–3：双构建同字节、旧产物保全、摘要/JSON、kill/磁盘/并发/出界/秘密/HTML与跨源资源负向；preview 拒绝非 production，build/preview 同 out 互斥且退出释放锁 | ★ `mbx xtask docs-verify --group reliability`；真实临时文件系统+子进程注入，不只 mock rename | M4 事务/模式/并发基本门；M7 完整安全/恢复门；M8 收口 |
| V-11 | N4/全部 AC、F15/AC-5：macOS（C-6）上 Rust+CLI 五命令、断网 build/check、无额外工具/Makepad；按 §3.1 包名与副本来源断言；已提交 git 候选安装及 panic 策略 | ★ `mbx xtask docs-verify --group toolchain`；非容器临时安装根/受限 PATH、原生执行与 egress 见证；macOS 环境须保留平台 cc/链接器等基础 Rust 构建条件；浏览器由验收 harness 提供，不参与作者工具链 | M7；M8 UAT 串联所需复测；M9 本地 file:// 与授权后 GitHub SHA 两步重验 |
| V-12 | N3/AC-4：桌面 Chrome/Firefox/Safari 最新稳定、真实 iOS Safari/Android Chrome 的 P1 阅读/导航/主题路径；phone HTTP 剪贴板拒绝可解释并可手选 | ★ docs config 的 Firefox/WebKit 冒烟是补充；真实 Firefox 稳定/Safari 与手机按 V-7/V-8 合并走查，记录 OS/版本/步骤；真机入口见 §9.3 | M5 Firefox/WebKit 自动化及真实 Safari 早期冒烟、设备/入口确认门；M8 完整真实矩阵，不能用引擎仿真替代 |
| V-13 | N5/AC-1、3、5、6 与 F3 配色：亮暗文字及焦点/相邻表面对比度；中文/英文/回退界面 lang、H1/地标、标题跳级 check 警告与 deny-warnings；320px/200%、reduced-motion、AA 重大严重 0 | ★ docs a11y 组；axe 加 computed style/相对亮度计算与键盘补充；首页/内容/404、11 语言代码及品牌色 fixture；无脚本系统深色截图 | M5 局部；M8 全主题收口 |
| V-14 | N1/AC-2–3、N2/AC-1–3、N6/AC-4：精确性能合同及 1000 次逐次可见保存；PoC 与正式验收分开，不降回合 | ★ `mbx xtask docs-verify --group performance`；M2 交付 B 生成器与采时 harness；按 §9.3 分阶段测量，与长跑服务同实例 | M2 有判据的原生风险门；M4 完整构建/字节、M5 LCP/CLS、M6 保存/短轮连续数据；M8 正式全组 |
| V-15 | C-1/C-3/C-4：格式、SDK/components default/gpu/ssr 与 CSR/GPU、docs 两包隔离单测/clippy；docs 单包图无 csr/effects/gpu，SSR 输出真 HTML；来源、CLI profile/版本、生成文档及范围台账一致 | `mbx fmt --all -- --check`、`mbx xtask sources verify`；§9.2 显式矩阵、`mbx build --locked -p rustify-docs-cli --profile docs-cli`；M2 同步 CI/README/CLAUDE，来源白名单另断言无新增漂移 | M1 适用项；M2 完整组合矩阵；后续每个提交点/表列出口；M9 全收口 |

### 9.2 运行编排与实际入口

- `playwright.docs.config.ts` 与现 GPU config 分开，项目名为 V-2/V-7–V-9 指定的名字；统一入口为 `npx playwright test -c playwright.docs.config.ts --project=<单个项目名>`；每个命令恰好一个 project，服务仅起该 project，端口由 OS 分配/单次配置传入，所有 Playwright invocation 串行。docs config 可加入 Firefox/WebKit项目，不把它们声称Safari。
- F17 的 7 示例是 fusion-basic、component-catalog、property-workbench、data-workbench、vellum、theme-studio、utility-lab；另覆盖 theme-export 消费者。基线在独立 worktree 固定开工 HEAD `91e81b8`（相对计划调查时的 `b0a6bc5` 只多了 `8e0ece1` 的 Rust 1.99.0 升级及其 makepad 弃用修复与文档提交，取它使基线与候选同一工具链），不把主树改动混入改前数据；用现配置逐一登记适用 regression project 的 spec/通过/失败/跳过清单及环境，随后在候选上对照同一清单。根 config 覆盖 fusion-basic、property-workbench、component-catalog、data-workbench、deployment、workbench-deep、theme-studio/deep、theme-export/deep、utility-lab/scanner/deep/scanner-deep；Vellum 按其独立 config。对应根/子路径产物分别构建，theme-export 还执行其独立宿主测试与两次 build.sh，保留 THEME_FONTS 等资源。utility-baseline/theme-text 不依赖此 SDK，不新增到本次 GPU 回归范围；不以 smoke 代替原有完整用例。
- 基线已有失败/不稳定项单列并定位，不能靠重试或“失败集合不扩大”判 F17/AC-4 通过；M1 退出仍要求全部适用原回归通过，既有失败未消除时保存证据并阻塞出口，若需范围豁免另请用户决定，不能自动改变 PRD。SDK/components、根 Cargo.toml/Cargo.lock 或启动 JS 改动时，由本里程碑判断受影响 project，最迟在其退出/下一提交点前重跑；无法证明不受影响时跑全范围，M7/M8 核对复用证据的代码/依赖/输入/环境。
- `mbx xtask docs-verify` ★ 分组只编排库/CLI 测试、fixture 构建、浏览器/测量；阶段全组覆盖本期需求，不给消费者增加 mbx/Node 条件。其依赖不链接 rustify-docs，不为 CSP 等值测试引入 SSR；读源字面值或对照 HTTP 字符串。host/SSR 的调用保持独立，矩阵如下；每格 test 与 clippy 分次运行，不能合并成一条 workspace union。
- CLI/framework依赖的完整图与默认DOM消费者图分别检查，含build dependencies。`cargo install --git`可在本地file git候选上排练；最终锁SHA的安装必须针对可达真实提交验证，不把安装工作树算发布。

| 组 | test | clippy / 其他门 |
| --- | --- | --- |
| CSR/GPU 工作区 | `mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins` | `mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings` |
| SDK 与 components 各包默认 | 各自 `mbx test --locked -p <包名> --lib` | 各自 `mbx clippy --locked -p <包名> --all-targets -- -D warnings` |
| SDK 与 components 各包 GPU | 各自 `mbx test --locked -p <包名> --features gpu --lib` | 各自 `mbx clippy --locked -p <包名> --features gpu --all-targets -- -D warnings` |
| SDK 与 components 各包 SSR | 各自 `mbx test --locked -p <包名> --no-default-features --features ssr --lib` | 各自 `mbx clippy --locked -p <包名> --no-default-features --features ssr --all-targets -- -D warnings` |
| Docs 两包 | 各自 `mbx test --locked -p <包名>`，包名为 rustify-docs / rustify-docs-cli | 各自 `mbx clippy --locked -p <包名> --all-targets -- -D warnings`；单包 tree/metadata 和 SSR HTML 断言 |

表内包名占位仅用于表达两包分别执行；实际 CI 不保留占位。M1 尚未新增文档包时继续当前工作区命令，并补已落地 SDK/features；M2 加入 docs 包时切换 exclude 与 SSR 分组。独立 DOM 的 wasm 及仓外默认/opt-in 编译仍由 V-1/V-2 负责；theme-export 保留独立 workspace 命令，不被 exclude 掩盖。

### 9.3 性能、千次保存与真实环境合同

- 参考站点B严格500页、中英各半，正文/图片数量按PRD列明12份指南分布生成，冻结生成种子与输入指纹；中英语料不引入多语言路由。默认首页、内容、404每类选首次加载字节最大的一页。
- 读者指标：Apple Silicon macOS Chrome稳定；M2冻结Chrome版本和Slow 4G实际参数；375px、CPU4倍，每页冷缓存5次取中位；HTML/CSS/脚本体积求各文件gzip -9字节之和，按PRD排除图片/字体，不把首帧后基础交互JS漏算。LCP/CLS在页内observer采集，404服务也要参加。
- 作者指标：依赖与编译缓存就绪；保存完成的见证时间与浏览器渲染见证时间落在同一可校准测量时轴，不能直接相减进程和浏览器不同time origin。每项5次取中位；完整构建每次先清空测试out，不清用户产物；new计时不含CLI安装、不触发站点编译。报告实际输入/硬件/版本与前后数据。
- 千次保存逐次写独特内容并等待浏览器见到这一revision再写下一次，期间不重启，不能把1000个事件合并后只看最后一次冒充通过；watch overflow/SSE断线负向另测。
- 性能 Chrome 固定版本与兼容矩阵最新稳定版分别登记，不混用。固定版本与 Slow 4G 参数已由用户 2026-10-10 确认：性能测量固定使用 Playwright 1.63.0 锁定的 Chromium 153.0.8010.12（本机 `ms-playwright/chromium-1243`，不随系统 Chrome 更新；记录中注明不是 Google 品牌稳定版）；Slow 4G 取 Lighthouse 模拟口径，经 CDP `Network.emulateNetworkConditions` 施加 latency 150 ms、下行 1.6×1024 kbit/s（209,715 B/s）、上行 750 kbit/s（96,000 B/s），不乘 DevTools 的 3.75 延迟与 0.9 吞吐系数，CPU 用 `Emulation.setCPUThrottlingRate` 4 倍；M5 退出前确认 Firefox/WebKit 可执行文件或下载授权、真实 Firefox/Safari、设备与走查人（用户 2026-10-10 授权安装 Playwright Firefox/WebKit 做引擎冒烟；真实 Safari 与手机由用户按清单走查，本机无 Firefox 应用）。Playwright 的 Firefox/WebKit 只提供早期引擎回归，不替代真实 Firefox 最新稳定版或 Safari/手机出口。用户 2026-10-10 决定：Playwright Firefox 155（firefox-1543）在本机 macOS 27.0.1 无法启动，记为 M5 引擎冒烟的环境缺口，不升级 Playwright（保持性能用 Chromium 153），M8 前由用户安装 Firefox 稳定版按同一清单手动走查；V-13 新增 `@axe-core/playwright` 开发依赖。用户 2026-10-11 进一步决定：本期不做 Firefox（真实稳定版走查与 Playwright 引擎冒烟一并留后续期），本期 N3/AC-4 浏览器矩阵为 Chrome 稳定版、macOS Safari 与手机。
- 真机默认通过显式 `preview --host <本机局域网地址>` 访问只读 production 产物，dev 保持 loopback；M5 登记设备可达 URL 与网络条件。局域网 HTTP 不是安全上下文，复制不可用时显示失败状态并保留手选，不冒称复制成功；完整复制成功路径在 loopback/用户提供的 HTTPS 入口验收。用户可提供 HTTPS 静态托管或隧道作为入口，未经明确指示不安装隧道服务、不上传。缺真实 Safari/手机时请求设备/真人走查，不用引擎仿真替代。保存实际入口、主路径、关键边界、console/失败请求与截图，并确认最新产物。

**V-14 分阶段判据与归属**：M2 的 T-8 固定 12 份指南的正文/图片分布、生成种子与源指纹，交付 500 页/中英各半的 B 语料和页内采时 harness。M2 尚无正式 build/dev，使用 B 片段数组与 SSR 最小壳演练：11 语言覆盖及全部 grammar 正则检查必须通过，最大最小壳的 HTML+CSS+全部基础 JS gzip -9 总量≤100 KB；500 页冷全量渲染与样式生成≤20 s；缓存命中后的配置变化全站重渲染和内存代提交必须<1 s，并报告留给 watch/SSE/浏览器的余量。这些仅为必要的风险门，不能当正式 AC 通过；任一不满足即 no-go，先修复瓶颈/替换局部依赖，重测后才退出 M2。

M4 对完整 B 首次跑空输出完整 build（含样式/站点地图/全部页）和产物字节；M5 对实际首页/内容/404 最大页跑 LCP/CLS/体积；M6 对正文、配置、资源保存各跑正式 5 次中位数，并先做短轮逐次可见保存。每步留下数据；发现超限就由该里程碑负责优化和复测，不带着已知超限推进后续提交点。M8 在最终同一服务与冻结输入上完成全部正式指标及 1000 次保存，不能复用 PoC 外推来关闭 AC。

### 9.4 P1 UAT 串联出口

下表在 M8 承接 PRD §6.3 的本期用户路径，复用各 V 组证据但补齐串联观察；跨期 UAT 只登记 P1 子集。后续 test-plan 可细化执行脚本与造数，不能作为省略本表或绕过 M8 的理由。

| UAT / 本期范围 | 检查 ID | 串联观察 |
| --- | --- | --- |
| UAT-1 | V-5、V-9、V-11、V-14 | 同一个受限作者环境依生成 README 执行 new→dev，60 s 内由验收 harness 打开首页与至少两页；只给作者进程 Rust/CLI 工具，不触发站点编译；缺 rustc 的 new/dev 无半成品，非空目录不变 |
| UAT-2 | V-4、V-8、V-9、V-14 | 同一 dev 会话改段落/图片/片段，≤1 s 可见且保滚动；写错链接、关闭诊断、再次保存、改正，状态明确并自动恢复；复制与 GitHub 内容格式检查有对应证据 |
| UAT-4 · 无脚本 P1 子集 | V-4、V-7、V-8 | 无自定义组件页面禁脚本后正文/11 语言高亮/全部代码分组/导航目录可读，无 wasm；演示初始画面/加载中留 P2，整条不记终验 |
| UAT-7 | V-6、V-7、V-8 | 同一 `/rustify-ui/` 产物在 preview 与无 rewrite 静态托管分别走全部页、深锚点/404、元数据和全部基础交互；完整 header 与 meta-only 均零 CSP 违规，失效引用 0 |
| UAT-10 · P1 子集 | V-4、V-10、V-11 | 离线运行含失效锚点/缺图/内联 script 的 check 和 JSON 模式，恰 3 错、位置/建议齐、非零退出；未注册组件/演示错误留 P2 |
| UAT-12 · P1 子集 | V-8、V-12、V-13 | 真实手机阅读/导航/切主题，桌面 200% 与 320px 无页面横滚；搜索留 P4，整条不记终验 |
| UAT-13 | V-10 | 先有成功产物，再制造内容错误并 build；旧树逐字节不变，最后输出错误数与首错 |
| UAT-14 | V-1、V-2、V-3 | 仓外默认 components 应用按普通配方使用 toast/对话框/resize；另一个应用 opt-in GPU、未 opt-in 负向编译；7 示例全部原回归及开关文档证据齐全 |

## 10. 里程碑与验收安排

F17 是P1最先交付能力，M1退出前不开始站点实现。依赖落在必要合同与检查；已满足的独立局部成果可以并行，但用户分期门、安全/发布风险门不绕过。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 默认纯DOM SDK可用，GPU兼容收口 | 固定 91e81b8 的 7 示例/theme-export 基线（§9.2）与启动/清理合同 | 主实施者负责 Cargo/SDK 公共出口/loader；DOM host、隔离及仓外 git fixture 在契约稳定后推进；7 示例与 theme-export 两包显式 gpu/独立锁更新，所有浏览器串行 | V-1 的 DOM/GPU 切片、V-2 仓内/仓外汇合门、V-3 全范围（含 theme-export 宿主/build/两 project）、V-15 适用矩阵；F17 全部 AC 成立，文档明确 gpu；回写「实施进度」 |
| M2 | 宿主SSG与原生工具链最小纵切片 | M1退出，V-1无依赖污染；§3/§4模式及 CLI 构建契约固定 | SSR最小页→显式候选→native CSS/minify/前缀高亮；实现片段缓存/有界并行，T-8 交付 B 生成器与 harness；锁依赖/许可、校验安装 profile/来源，同步 CI 分模式入口，冻结测量环境；不扩其余控件SSR | V-1 补 CLI 图、V-4 SSR/CSS/11语言切片、V-14 §9.3 的 go/no-go 门、V-15 组合矩阵与受影响 V-3；失败修复/重测后才退出，不放宽合同；回写「实施进度」 |
| M3 | 可校验的Markdown内容图 | M2解析/渲染公开合同及V-4切片 | parser/links/anchors与代码片段可分任务，共享Page/Diagnostic单人；统一 writer/lang/标题属性与 raw HTML/CSP 资源规则集中集成 | V-4 内容全组（computed display 留 M5，GitHub 可达预览留 M9）；恰 3 错、span 与片段同源；回写「实施进度」 |
| M4 | 内容模板与可部署静态产物 | M3内容图V-4；M2渲染与风险门 | CLI new/build/check/preview、production manifest、私有输出事务/ignore；模板/meta 可并行，manifest/URL/锁主实施者单写；交付显式 preview --host | V-5/V-6 的 new/build 侧、V-7 HTTP/preview/base 切片、V-10 事务/模式/并发基本门、V-14 完整 B 构建/字节早期数据、V-15；不含尚未实现的 dev 侧；回写「实施进度」 |
| M5 | 默认阅读主题与零wasm基础交互 | M2主题接口/V-4；M3内容图；完整集成需M4产物 | 先完成两份 impeccable brief，再做静态壳/增强；主实施者集成设计 sidecar、finish review 与 a11y。确认浏览器/真机入口，Firefox/WebKit 和真实 Safari 先冒烟 | V-4 computed style 切片、V-7、V-8、V-12 早期冒烟/环境确认、V-13 局部、V-14 LCP/CLS/体积早期数据；设计记录/finish review、失败/无脚本状态齐；回写「实施进度」 |
| M6 | 作者随改随看，错误可恢复 | M4 V-10 基本门，M5 基本 UI/诊断 brief，M2 缓存与热配置风险门 | watch 失效、片段缓存/并行渲染、内存 last-good/SSE/保滚动；dev 状态单写，fixture 编辑目录隔离；不得写生产 out | V-9（含 F1/F2 的 dev 侧及安装 profile panic），V-14 正文/config/static 保存和短轮逐次可见数据；端口/SSE/错误关闭恢复通过；回写「实施进度」 |
| M7 | 最小工具链、安全与恢复收口 | M4–M6完整命令链路及局部检查 | macOS 消费者演练、离线、完整故障注入/泄漏审查；可并行撰写 M9 文档草稿，锁/公共文件单写 | V-1、V-5、V-6 重跑、V-7 的 N7/AC-4、V-10、V-11、V-15，受影响 V-3/V-4；全部 N4/N7 与失败安全门；回写「实施进度」 |
| M8 | P1跨浏览器/部署/性能终验 | M7风险门通过；M5–M6真实交互就绪；已确认设备与入口可用 | 同一根/子路径/header/meta产物走查；500页性能/1000保存；执行 §9.4 UAT 串联。昂贵未变证据复用须逐组给依据 | V-2 回归、V-7–V-14 正式出口及受影响 V-1/V-3–V-6；全部 P1 AC 与 §9.4 本期 UAT 有证据，不提前终验跨期条目；回写「实施进度」 |
| M9 | 可安装交付与维护文档 | M8全P1出口；远端阶段必须先取得候选推送授权/方式 | 文档/帮助/生成 README 填已验证 SHA 与独立 CLI 版本；先对已提交候选做 file:// 安装和 DOM 配方，再在授权推送后对同 SHA 的 GitHub URL 重验。拒绝/缺推送授权只保存本地结果，M9 仍阻塞，不能称远端交付完成 | V-4/V-5/V-6 受模板影响切片、V-11 本地与远端两步、V-2 仓外远端配方、V-9 安装二进制 panic 子集、V-15；实际缺口为零才完成；回写「实施进度」 |

**提交点**：M1（SDK 阶段收口）、M4（首次可部署产物及 V-10 基本门）、M7（内容链路/安全/工具链）、M8（全部 P1 AC/UAT/性能证据）、M9（安装与文档最终交付），每次均通过该阶段退出条件及 V-15。M9 先形成候选代码提交并验证 file:// 安装，再确认用户授权与推送方式；推送后验证 GitHub 同 SHA。候选失败则修复、递增 CLI 版本并重新提交，追加推送仍须在已授权范围内；通过才记 M9 完成。仅暂存本计划文件，进度提交遵循顶部协议；不把 M8 证据滞留到推送门之后。

### 10.1 必要的内部任务

| 任务 ID | 所属里程碑 | 交付物/影响路径 | 前置成果 | 检查 ID |
| --- | --- | --- | --- | --- |
| T-1 | M1 | 中性Rust binding/feature/API gate/Pace条件定义；SDK与Cargo公共文件 | 已核对旧GPU合同 | V-1、V-2 |
| T-2 | M1 | DOM JS host/loader、随 SDK 分发的资源与普通消费者配方；仓内/仓外隔离 fixture | ADR-1/§4.1 中性 host/启动契约固定；与 T-1 集成后验收，GPU loader 主实施者单写 | V-1、V-2 |
| T-3 | M1 | 7 示例与 theme-export 两包 gpu 开关、独立锁与原回归兼容 | T-1 编译合同 V-1 通过；宿主字体目录保留；回归资源与 T-2 串行 | V-3 |
| T-4 | M3 | 文件/地址/标题/链接/diagnostic图 | M2公开数据契约；主实施者单写共享类型 | V-4 |
| T-5 | M3 | 围栏/分组/引用片段语法与依赖 | T-4稳定Page/span接口；可与其内部链接规则实现并行 | V-4 |
| T-6 | M5 | 默认静态壳、响应式/导航与诊断 markup；两份设计 brief、DESIGN sidecar、finish review | 先完成 impeccable brief；M2 SSR 合同/M3 Page 图，M4 未就绪时用固定 fixture；诊断真实链路 M6 集成 | V-8、V-13 |
| T-7 | M5 | 原生theme/copy/code-group/scrollspy增强 | T-6 markup合同固定；只写框架JS与对应spec | V-8 |
| T-8 | M2 | tests/docs-ssg 的 B 生成器、冻结输入与页内采时 harness；SSR 500 页与配置热渲染 PoC | PRD B 分布、M2 最小 SSR/样式/高亮、固定 Chrome 来源；不依赖尚未实现的正式 build/dev | V-4、V-14 |

其余按里程碑直接推进，不为每个文件造任务。Cargo.lock、workspace、SDK lib、共享模型、manifest/发布事务、计划与实施记录始终单一写入者。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| SSR/原生依赖与 git 安装 profile 尚无实测 | 未来实现风险，默认主题必须真 HTML且无内联样式 | M2 实施者排练 file:// 安装根 profile/patch/lock、无额外系统依赖/serde 锁兼容、11 语言全部 grammar、SSR/CSS/体积；临时验证提交不冒称交付 | M2 出口/下游依赖前 | 当前否；实测失败阻塞出口 |
| 性能当前无 Docs 基线 | 不能宣称满足预算或虚报 PoC | 用户已于 2026-10-10 确认合同；T-8/M2 风险门→M4 构建→M5 加载→M6 保存→M8 正式收口，超限在当阶段优化/复测 | M2/M4/M5/M6 各出口及 M8 | 当前否；未达标阻塞相应出口 |
| 固定 Chrome 来源/冻结副本 | 系统稳定版自动更新会破坏性能口径 | 已解除：用户 2026-10-10 确认 Playwright Chromium 153.0.8010.12 与 Lighthouse 模拟 Slow 4G 参数（§9.3）；不自行安装浏览器 | M2 冻结测量前 | 否 |
| Firefox/WebKit、真实 Firefox/Safari、手机和走查人（2026-10-10 用户已授权安装 Playwright Firefox/WebKit，真实 Safari/手机由用户走查；本机无 Firefox 应用） | 引擎回归与真实矩阵需要各自证据，手机不能访问 loopback | M5 负责人确认现有可执行文件/下载授权、真实设备/走查人，登记 preview --host 的可达 URL；HTTP 复制拒绝/手选及安全入口成功路径分别验。Playwright Firefox 本机无法启动；用户 2026-10-11 决定 Firefox 整体留后续期（§9.3） | M5 出口确认/早期冒烟，M8 完整矩阵 | 不阻塞 M1；缺环境阻塞 M5/M8 相应出口 |
| GUI 与全 GPU 回归成本/既有失败 | 不完整基线会掩盖 SDK 横切回归 | M1 在固定 worktree 建全范围清单，全部适用原回归须通过；既有失败定位并阻塞出口，未经用户裁定不豁免。后续按 §9.2 触发；不增加 headed GPU 性能承诺 | M1 出口及横切改动提交前 | 验证门，未过不得开始站点实现 |
| 远端候选推送授权/方式未提供 | 本地安装不能证明 GitHub SHA 可安装 | M9 先对具体已提交候选完成 file:// 安装/配方，给出可审阅候选与结果后取得用户推送指示，再对同 SHA GitHub URL 验证；拒绝/缺授权保持 M9 阻塞 | M9 远端阶段前 | 不阻塞前序实施；未授权阻塞 M9 完成 |
| P1 CLI 仅承诺 macOS（C-6） | Linux CI（如 GitHub Pages 常见流水线）或 Windows 上建站不在本期承诺内 | 文档与 CLI 帮助如实写明；P2 官方站改用 Linux CI 或用户需要其他平台时，重评平台范围并补对应原生环境的 V-11 | P2 官方站部署方案确定前 | 否 |
| Q2正式发布渠道/Q4官方站地址 | 后续安装分发与真实站部署 | P1按git可安装提交交付通用产物；crates.io决定P4前、官方GitHub Pages线上P2实测 | 对应后续出口 | 否，本期不承诺线上上传 |
| 既有未提交PRD/台账、其他活动计划共享SDK | 证据未随Git共享；共享改动不能混入本期提交 | 主实施者开工核对dirty文件与他人任务；只提交本期改动，记录起始SHA和实际diff | 每次写入/提交前 | 无已证实冲突；冲突时停相应写入 |

- 最终状态：**Ready**。D9–D11 已由用户于 2026-10-10 确认并写入决策表、ADR 与相关契约；C-5 的名称、完整矩阵与阈值及 C-6 的平台范围已确认，不自动放宽。剩余各项都是带负责人与最晚确认点的环境、授权或验证门，只阻塞对应里程碑出口，不影响开工。
- 文档修订不计实施。原独立评审结论保留，处置另记；实施进度以「实施进度」为准。环境/实测与 M9 推送是已登记的后续门，既不把未来实现当通过，也不据此额外阻塞 M1 基线采集。

## 12. 已知坑与历史教训

- `crates/rustify-makepad/src/wasm/host.rs` 的 defer/resize不是浏览器默认能力封装，未boot会没有hooks；不能只optional依赖就宣称DOM完整。
- `web/loader.js` 顶部Makepad静态import无法靠“页面不创建canvas”消除；DOM loader必须独立。
- `Cargo.toml` workspace feature统一会让CLI测试看起来能跑，却污染SSR/默认DOM依赖图；隔离fixture与分包invocation才是证据。
- `mount.rs` 新建root并CSR mount，不是hydrate；P1不得借它擦掉静态正文。
- `xtask/src/build.rs` 先清旧输出是GPU示例既有行为，不是文档失败安全范式；新产物事务必须独立。
- `CssCompiler::build` 累积候选；dev类删除不重建会残留无用样式、破坏复现与预算。
- `theme/fonts.rs` 的字体目录/测试属于GPU资源历史，P1默认系统字体与本地小图标，不把整个资源包复制到内容站。
- `docs/architecture.md` 测试分层与 `docs/plan/DX-REFINE.md` 的软件WebGL启动成本：F17必须回归但不新造全工作区浏览器运行；显式project、共享页范式与证据复用降低成本，不降预算/回合。
- `AGENTS.md` UI完成标准与 `CLAUDE.md` Browser tests：build/lint、WebKit或截图缩宽均不能代替真实Safari/手机走查；未验证项明确保留。
