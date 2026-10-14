# DOCS-SSG-P2 · Rustify UI · 组件演示与英文文档站（DOM 交互岛 · 构建期静态生成 · 原生 Rust 工具链）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-13 +08:00 · `785f51ad9fdcb69e624a0c8f85d91e3541d58e90` · dirty；事实基于当前工作树。未提交项为 SDLC 台账、本计划及评审报告，产品源码未改。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · P2（第 2 期），范围以 §7.1 出口、§7.2 分期矩阵为准。计划版本：v2。
>
> 本期交付：**组件库模板、可校验的组件嵌入与同源演示、定制页/布局插槽、ThemeDocument 接入和 Rust 开发预览；12 份指南与 24 类 DOM 演示组成的英文文档站上线。**
> 本期独特职责：在 P1 完整静态 HTML 上按需接管局部交互，保留无脚本阅读与失败安全；默认内容站继续不编译站点 Rust。
> **顶层排除：Makepad/GPU 演示、整页水合与 SPA、P3 多语言、P4 搜索和 llms.txt、Linux/Windows CLI 支持。**
>
> 用户 2026-10-13 确认：交付是 **SSG**。HTML 在 CLI 的构建期生成，线上只托管静态文件；本文中的 Leptos/SDK `ssr` 是执行这一生成步骤的编译 feature，helper 是构建工具。组件使用确定初态，异步工作在浏览器开始交互后运行；不引入按读者请求渲染页面的服务。

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

### 恢复快照

- 最近更新：2026-10-14T06:00:56+08:00
- 当前进度：5/7 个里程碑完成
- 当前状态：M1–M5已完成；M6原生Chrome发现自动favicon404，最小修复库R2已提交0494f304；CLI0.1.12配对准备中，旧C1正式测量保留为历史，不冒新候选通过
- 最近完成：M5 · 同源英文官方站、真实dev/三浏览器入口、设计确认及B语料增量
- 下一步：接收R2/CLI0.1.12五文件配对与locked/offline源门，Root提交C2并固定安装后重跑受影响UAT/正式性能/集中矩阵；当前资源hydration_harness独占
- 当前阻塞：无技术开工阻塞；用户本轮无法安排首次用户与实体手机，M6 保留验收缺口；候选分发与 Pages 条件按 §11 准备
- 代码基线：0494f304（M6 native favicon修复库R2）；M1/M2仍5632d13，M3–M5仍676a5577，历史C1为8e86626f；用户既有工作树改动保留

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M6 | 进行中 | 2026-10-14T06:00:56+08:00 | native Chrome真实favicon404先红后绿，Docs191/clippy/fmt及开发CLI浏览器通过，库R2已提交；CLI0.1.12配对准备，旧C1正式A/B/内容测量保留，受影响门待新候选复验 | [M6 记录](../validation/docs-ssg-p2/M6.md) | 0494f304（库R2）；C2待提交 |
| M1 | 已完成 | 2026-10-13T12:53:00+08:00 | V-2 水合 10/10；默认 DOM 12/12；旧示例 58/73/20/72/20 通过，geometry reset 缺陷先红后绿 | [M1 记录](../validation/docs-ssg-p2/M1.md) | 5632d13 |
| M2 | 已完成 | 2026-10-13T19:01:02+08:00 | release代理371.9ms/位移0、开发档增量中位1944.6ms；配对/错配、subpath20+4/meta24、features/检查与设计收口通过 | [M2 记录](../validation/docs-ssg-p2/M2.md) | 5632d13 |
| M3 | 已完成 | 2026-10-13T23:31:15+08:00 | Docs229/CLI131、subpath12/meta36、离线Git字体/检查/设计通过；release255.9ms/位移0；开发档失败2369.4ms保留，修正最终1731ms，真实同步/失败保护与受影响检查通过 | [M3 记录](../validation/docs-ssg-p2/M3.md) | 676a5577 |
| M4 | 已完成 | 2026-10-14T01:59:33+08:00 | 已安装模板/README与离线check/build、Rust dev3/3及严格clippy、九图ship/设计同步通过；完整dev五次中位1425.1ms/位移0，content首屏264ms/0wasm，原红与归因保留 | [M4 记录](../validation/docs-ssg-p2/M4.md) | 676a5577 |
| M5 | 已完成 | 2026-10-14T04:02:13+08:00 | CLI0.1.10：官方37页53文件/浏览器46+47+47、Docs262/CLI72、真实dev11与焦点1、设计ship/规范同步；B原500字节未变，512页check/build及最大候选预检通过；原红保留 | [M5 记录](../validation/docs-ssg-p2/M5.md) | 676a5577 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

本表登记上游需求及本期约束，AC 原文以 PRD §3 为准。本期终验 WORK-F1、WORK-F4、WORK-F5、WORK-F6、WORK-F8、WORK-F13、WORK-N2；WORK-F15 终验仍在 P3，WORK-F16 和 WORK-N1 仍在 P4。跨期终验须核对 P1 已交付 AC 及受本期改动影响的回归，不能仅凭新增 AC 通过关闭整条需求。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F1 | 功能 | PRD §3、§7.2，P2 AC-2、AC-5 | 组件库模板可运行；首次用户独立新增演示 | §2、§5.4；V-8、V-9、V-13、V-15，P1 条件由 V-1/V-14 复核 | 已确认；本期终验 |
| WORK-F4 | 功能 | PRD §3，P2 AC-1–4 | 带属性/子内容的内置或注册组件；SSR、局部交互、定位诊断、GitHub 占位 | §2、§5.1、§5.2；V-3、V-5、V-15 | 已确认；本期终验 |
| WORK-F5 | 功能 | PRD §3，P2 AC-1–6 | 编译与源码同源的演示；无脚本、加载中、故障与恢复 | §2、§5.2、§6、§8；V-4、V-5、V-9、V-11 | 已确认；本期终验 |
| WORK-F6 | 功能 | PRD §3、§7.2，P2 AC-2、AC-3 | 定制页与全部布局插槽；404 共用覆盖结果 | §3、§5.3；V-6、V-14；搜索由 P4 承接 | 已确认；本期终验 |
| WORK-F8 | 功能 | PRD §3、§7.2，P2 AC-5、AC-6 | 同一主题文档作用于站点与演示；Rust 中的类自动生成 | §5.5；V-7、V-11、V-12、V-14 | 已确认；本期终验 |
| WORK-F13 | 功能 | PRD §3、§7.2，P2 AC-4 | Rust 修改的构建中、成功刷新、失败保留上一版 | §5.4；V-9、V-12、V-14 | 已确认；本期终验 |
| WORK-F15 | 功能 | PRD §3、§7.2，P2 AC-3 | 一次 check 报出未注册组件与不能编译的演示 | §2、§5.1、§5.4；V-3、V-4 | 本期交付段；P3 终验 |
| WORK-F16 | 功能 | PRD §3，P2 AC-1–3 | 12 份同源指南、README 介绍、24 类 DOM 演示；英文子路径站上线 | §5.6、§8；V-10、V-11、V-15 | 本期交付段；P4 终验 |
| WORK-N1 | 性能 | PRD §3，P2 AC-4；P1 AC-1–3 回归 | FCP 至所有演示响应输入 ≤1 s、演示水合布局偏移为 0；静态页仍零 wasm | §5.2、§7、§9.2；V-5、V-12 | 本期交付段；P4 终验 |
| WORK-N2 | 性能 | PRD §3，P2 AC-4；P1 AC-1–3 回归 | Rust 增量编译完成至新画面 ≤2 s；内容迭代不重复 Rust 编译 | §5.4、§9.2；V-8、V-9、V-12 | 已确认；本期终验 |
| WORK-N3 | 兼容 | PRD §3、§7.2；各期 AC-2、AC-4 复核 | 完整响应头/受限 meta CSP；P2 浏览器矩阵和子路径正确 | §4、§6、§8；V-11、V-15 | 本期复核；P4 终验 |
| WORK-N4 | 工具链 | PRD §3、§7.2；全部 AC 每期复核 | 所有依赖/产物无 Makepad；作者仅需 Rust、wasm32 目标和 CLI | §2、§4；V-8、V-10、V-14、V-15 | 本期复核 |
| WORK-N5 | 可用性 | PRD §3；P1 AC-1、AC-3、AC-5、AC-6 回归 | 新演示/定制页保留 AA、键盘、阅读、缩放与地标 | §6；V-5、V-11；搜索相关 AC 留 P4 | 本期回归；P4 终验 |
| WORK-N6 | 可靠性 | PRD §3；P1 全部 AC 回归 | 同输入产物可复现；失败不替换成功产物/预览 | §3、§5.4、§8；V-4、V-9、V-14 | 本期回归 |
| WORK-N7 | 安全 | PRD §3；P1 全部 AC 回归 | 新生成 HTML/样式/props/源码不绕过 CSP 与发布集合校验 | §3、§4、§7；V-3、V-4、V-6、V-7、V-11、V-14 | 本期回归 |
| C-1 | 呈现约束 | PRD D2/D5/D7；AGENTS.md「项目定位与实现范围」 | 只实现 DOM；普通链接导航；仅有交互岛的页面加载 wasm，纯内容不编译站点 Rust | §2、§5.2；V-8、V-10、V-12 | 已确认 |
| C-2 | 基线与平台 | P1 恢复快照、[安装交付记录](../validation/docs-ssg-p1/M9.md)、[P1 决策 B-03](DOCS-SSG-P1.decisions.md#b-03--p1-cli-支持平台与原生依赖约束d10-第二部分)；mise.toml、Cargo.toml | 继承已交付 P1 和 macOS CLI；Rust 1.99.0、Leptos =0.8.22、bindgen =0.2.128；模式分包构建 | §1、§2、§4；V-1、V-2、V-8、V-14 | 已核实 |
| C-3 | 生命周期 | PRD D8、§4 DEP-3；SDK runtime、DOM loader、mount | runtime 先于水合；一页一个 wasm 实例，共享失败域；实例中止清理监听/任务/浮层 | §2、§5.2、§8；V-2、V-5 | 已确认 |
| C-4 | 交付与验证 | AGENTS.md「前端 UI 开发与验收」「完成标准」；CLAUDE.md「Conventions」「Browser tests」 | 中文计划；当前分支；真实浏览器与本仓检查；Playwright 单 project 串行；无缺失服务自动安装 | §6、§9、§10；V-11、V-14、V-15 | 已确认 |
| C-5 | 内容与部署 | PRD §5.2 Q2/Q4、D11/D12、WORK-F16；当前 GitHub 仓库与 Pages API | git 安装；默认 GitHub Pages /rustify-ui/；HTML页面及站点资源、不发布Markdown，meta不含frame-ancestors；Pages未启用由M7配置 | §3、§5.6、§8；V-10、V-15 | 已核实；上线条件单列 |
| C-6 | 本期验收口径 | PRD §3、§5.2 Q3/Q5；用户 2026-10-12 确认三个数值，2026-10-13 明确Firefox延期到P4；[P1后移范围](DOCS-SSG-P1.prd-dev-gaps.md#g-05--firefox-从-p1-验收矩阵后移--b) | P2 沿用 ≤1 s、≤2 s 与4/5人15分钟门槛；D-04不延伸P2；Firefox稳定版/引擎P2保留未验证，不阻塞P2出口，P4恢复完整矩阵 | §7、§9.2、§11；V-11、V-12、V-13 | 已确认；Firefox延期有新决定 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 局部水合 | SDK 增加 hydrate 模式与接管入口；每个嵌入区域显式 SSR/水合，使用 Leptos hydrate_from | 不接管 shell；共享同一页实例，作用域/状态独立；SDK 的 runtime、router、overlay 上下文随根建立 | WORK-F4/F5、C-1/C-3；现有 mount 与锁定 Leptos 接口 |
| D2 | 库与站点编译边界 | rustify-docs 默认 pipeline；helper 只开 site-ssr，浏览器只开 hydrate；站点同一 Rust 库分别编为宿主 helper 和 wasm | pipeline 包含 site-ssr，二者与 hydrate 互斥；内容/高亮/样式编译依赖只进 CLI 图，不进作者 helper | WORK-N4、C-2；§2.1 分目标图检查 |
| D3 | 注册与冷态诊断 | site! 用独立过程宏；宏与 CLI 共用一份宿主语法解析库；describe 与冷态清单逐项核对 | 名称、字段、默认值声明、函数/页面/插槽只定义一次；不维护手写 JSON 或两份语法 | WORK-F4/AC-3、WORK-F15/AC-3；§2.3、V-3 |
| D4 | 演示源码 | 注册直接引用函数符号；CLI 从同一函数所在的原始文件按语法范围切片 | 无 pretty-print、手抄副本或正则找大括号；全文件只作为 HTML 内源码文本，不发布原始 .rs | WORK-F5/AC-1、WORK-N7；已有 include 仅提取 region、从不编译 |
| D5 | 故障与恢复 | 外部基础脚本控制状态；同页共享实例失效时全部相关交互块终止，恢复 SSR 初始画面，提供整页重新加载 | 静态 shell/源码不依赖 wasm；不增加逐岛实例隔离或透明重启承诺 | PRD D8、WORK-F5/AC-4/6；DOM loader on_fatal |
| D6 | 主题 | 一个 ThemeDocument 同时解析 light/dark；构建期输出外部 CSS，水合只提供同一主题上下文 | 保留 system/light/dark 首帧逻辑；不在首帧重新写 inline style，不用默认 ThemeScope 覆盖静态投影 | WORK-F8、C-1；codec/resolve/font_context、既有 mode.js |
| D7 | 本期规模与上线 | 7 个里程碑；局部风险即时验，昂贵矩阵/性能/首次用户集中 M6，安装/实际上线 M7 | 遵循 PRD 6–8 个建议；验收和发布均有独立退出门 | PRD §7.1、WORK-F1/AC-5、WORK-F16/AC-3 |

### 0.3 ADR-lite

#### ADR-1：显式水合根与既有 SDK 作用域协同

- 状态：Accepted（技术方案；产品形态沿用 PRD D2/D8）。
- 背景与驱动：WORK-F4/F5 需要保留 SSR 节点并建立 ScopeRoots、ActionSink、路由与浮层；现有 mount 只能新建根。组件内部的 ID、受控属性和运行时资源同样必须一致。
- 备选 A：Leptos #[island] 与 hydrate_islands，框架自动生成 props 标记/导出。减少自有启动分发，但需额外把 SDK 作用域上下文接入宏生成的 Owner，源码注册与冷态诊断也需另一层桥接。
- 备选 B：注册表对指定容器调用 SDK hydrate，底层使用锁定版本的 hydrate_from。注册表同时承担类型、状态与源码契约；启动/清理边界显式，代价是自有少量 props/根 metadata。
- 决策：B。仓库已有多作用域 runtime 和 AppHandle，无需整页水合或引入 Leptos 的内联 HydrationScripts。先用 V-2/V-5 证明根身份、Owner 生命周期、三岛独立和 trap 清理，再解除后续集成前置。
- 正面后果：CSR/GPU 原消费者继续沿用 mount；文档岛可控制启动闸与静态恢复，诊断归属集中。
- 负面/中性后果：需维护一种私有岛 metadata 协议，并补 SDK hydrate feature 与 SSR ID 上下文；不能把已有 DOM loader 的 CSR 挂载当作水合。
- 运行前提：ScopeRoots、ActionSink、router、OverlayStack、ID allocator、locale/静态主题均在传给 hydrate_from 的闭包内建立，归其新根 Owner；岛和 slot 内岛的水合标记必须保留。按用户 2026-10-13 确认的 SSG 合同，只生成确定初态，异步工作放到浏览器；无法生成初态者显式 client-only。
- 重新评估触发：显式根无法保持复杂组件 SSR 对应关系，或上游 islands 提供无需额外上下文桥接的可验证方案。变更须保留 V-2/V-5 和本期 CSP/性能门，不改静态优先形态。

#### ADR-2：同库双目标与静态注册清单

- 状态：Accepted（技术方案）。
- 背景与驱动：组件代码必须实际执行 SSR/wasm，两目标必须使用同一注册和源码；check 在冷态编译失败时仍须报告内容错误，纯内容站须维持原成本。
- 备选 A：另建轻量 rustify-docs-site 包，把共享注册协议与水合全部移入，宿主库调用它。边界易读，但新增公开包与版本/导出维护，对本期没有额外独立生命周期。
- 备选 B：在 rustify-docs 内将站点注册与浏览器模块和宿主模块按 feature 分隔；保留现有默认宿主 API。新增功能集中到 site 模块，不把内容编译器编入 wasm。
- 决策：B；site-ssr 与 pipeline 分隔 helper 和 CLI 的宿主依赖，二者与 hydrate 互斥。site! 的过程宏位于独立 crate，冷态解析与宏展开共享语法库，运行期 DTO 仍在 rustify-docs。只对配置声明的 Rust 站点运行 Cargo/helper，不让 CLI 自身依赖作者代码或通过动态库加载。
- 正面后果：两种模板共用现有 CLI/内容管线；属性和名字来自一处声明；无插件加载、Node 或新后台服务。
- 负面/中性后果：Cargo 清单和模块 cfg 增量较多，需要独立两目标图验证；新增过程宏与语法库服务宏展开/冷态预检两个真实调用方，不承担运行期协议。注册名必须是字面量，Rust 函数须可从站点源码寻址；组件 SSR 能力与 client-only 边界按 §2.3/§11。
- 重新评估触发：第三种消费形态需要独立版本或 feature 隔离导致大量条件分支。按实际使用再拆包，不先建通用 adapter。

### 0.4 职责与事实所有权

|  | 站点作者与同源 Rust 库 | CLI / rustify-docs | SDK / 浏览器基础脚本 |
| --- | --- | --- | --- |
| 拥有 | 可信组件代码、props 定义、初始状态、源码、定制页与插槽内容 | Markdown 位置、静态注册预检、批量编译/渲染、内容/路由/资源图、主题 CSS、发布事务和 last-good | 接管 SSR 根、Owner/监听/浮层生命周期、岛状态/故障提示、基础交互和整页重载 |
| 不拥有 | shell 发布事务、内联脚本授权或框架安全豁免 | Rust 表达式求值、GPU 演示、插件沙箱、远端托管控制器 | 文档路由/地址栏所有权、正文/源码的权威内容、站点构建或翻译 |

Rust helper 是构建期的本地子进程，退出或协议错误由 CLI 转为诊断；wasm 失效由基础脚本保全静态页面，不能依赖 Rust 析构执行。

### 0.5 明确不在本期

- P3 的语言路由、译文与回退；P4 的搜索、AI 索引及 WORK-F16/AC-4。定制页保存标题/描述/可索引文本元数据即可，本期不生成搜索或 llms.txt。
- GPU region/Makepad、整页水合/SPA、逐岛故障隔离、动态库插件、运行时 AI、在线服务、Markdown/原始 Rust 下载端点。
- Linux/Windows CLI 承诺和浏览器快照生成。Pages 构建在已支持的 macOS runner，部署 job 只上传已有静态产物。
- P1 D-04 未指定补验期的历史缺口不直接认作 P2 豁免；P2 当期浏览器验收按 C-6。Firefox 的 P2 稳定版/引擎验收依用户 2026-10-13 新决定延期到 P4，保留未验证记录；依据不是 P1 D-01 的自动延伸。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·足够]** P1 顶部为 9/9，P1-gaps 为 2/2，[安装交付记录](../validation/docs-ssg-p1/M9.md)保留 CLI 0.1.4 同 SHA git 安装、仓外 DOM、禁网作者与浏览器证据。安装候选到 HEAD 的 SDK/components/style/docs/CLI 和锁文件无产品差异；基线检查 `mbx test --locked -p rustify-docs --lib` 为 127/127 通过。历史矩阵/性能/设备结果仍是对应记录的结果，并非本期复验。
- **[已核实·足够]** 默认 DOM、SSR 分组和原生构建链已存在；`xtask/src/docs.rs::assemble` 在进程内调用 `wasm_bindgen_cli_support::Bindgen`，版本由 xtask/Cargo.toml 锁定。P2 CLI 可复用同一打包范式，无需要求作者安装 bindgen CLI。
- **[已核实·缺口]** `crates/rustify-ui/src/mount.rs::mount` 创建 content/overlay 并 mount_to，且有 data-rustify-scope 即拒绝；清单只有 csr/ssr/gpu → 增加 hydrate 接管与 SSR 根契约 → 否则静态内容被重复创建或接管失败。
- **[已核实·缺口]** `crates/rustify-components/src/lib.rs` 仍将 Dialog/Select/Tooltip/Menu、DataTable、Tree、NumberField、ColorField、Toast 等入口限制在 wasm；`id.rs::next` 使用 thread_local 递增计数 → 补本期涉及的宿主 view 与每岛 ID 分配上下文 → 否则并行 SSR、连续构建与水合 ARIA/ID 不一致。
- **[已核实·缺口]** `crates/rustify-docs/src/markdown.rs` 只处理 include/code-group 指令；`Fragment` 没有嵌入记录。`new.rs::run` 仅接受 content，dev 的 `Builder::build` 仅 snapshot/analyze/generate → 增加静态声明、嵌入记录与 Rust 编译状态 → 否则冷态诊断、模板演示和 Rust 更新都无法交付。
- **[已核实·缺口]** `style.rs::site_css` 从生成 HTML 候选生成 CSS，config 只接主色/logo；SDK ThemeScope 在浏览器写根属性/CSS → 增加主题文档、隐藏交互状态候选与只提供上下文的静态主题绑定 → 否则初始主题/打开浮层/动态类与演示不一致。
- **[已核实·足够]** `generation.rs::generate` 已统一拼装 shell、收集类、检查最终 HTML/CSS 资源；`publish.rs` 已实现同盘 staging/backup、锁与恢复；`dev.rs` 已有事件流和成功代按字节比较。P2 在这些入口接线，不另造发布事务或预览服务器。
- **[已核实·缺口]** config 的 content.dir 当前递归读 Markdown，不以导航项决定发布范围 → 增加内容白名单 → 直接指向 docs 而不筛选会发布 PRD/计划/验证。官方站及 Pages workflow 尚不存在；只读 GitHub 仓库 API 显示 has_pages=false，Pages API 返回 404 → M7 配置与真正发布后验收，不能把本地 preview 当上线。
- **[已核实·缺口]** 锁定 Leptos `hydrate_from` 新建无父根 Owner 并返回 UnmountHandle；其 hydrate feature 独立于 csr，但外层 context 不会继承。HydrateSharedContext 使用资源/错误/未完成块全局量；tachys 的字符串 `style:` 水合不重写 CSS。shell 清理自身 `<!>` 后再拼入 article，正文已受保护；新增 slot 岛同样须事后拼接。补闭包内上下文、外部启动前提与几何类/CSSOM，确定初态与浏览器异步边界见 §2.3；实际 M1 证据见完成记录，不以选型代替验收。

### 1.2 文件与边界

★ 为拟新增路径；实现可按职责拆文件，但修改正文中的交付路径并保留接口契约。没有相关规则文件的子目录继承根规则。

| 路径 | 本期改动 |
| --- | --- |
| `crates/rustify-ui/Cargo.toml`、`crates/rustify-ui/src/mount.rs`、`crates/rustify-ui/src/lib.rs`、`crates/rustify-ui/src/diagnostics.rs` | hydrate feature/API、SSR 作用域与接管拒绝语义；仅共享 mount 必要上下文，CSR/GPU 默认不变 |
| `crates/rustify-ui/src/theme.rs`、`crates/rustify-ui/src/theme/provider.rs`、`crates/rustify-ui/src/i18n.rs`、`crates/rustify-ui/src/overlay.rs` | 宿主安全上下文、静态主题绑定和浮层初态；浏览器 effect/定位继续由现有 runtime 承担 |
| `crates/rustify-components/Cargo.toml`、`crates/rustify-components/src/lib.rs`、`crates/rustify-components/src/id.rs` 及本期受限的组件模块 | hydrate/SSR view、属性初值、稳定 ID；DataTable/Tree/Progress/ColorField/Splitter 的 style: 改为 SSR 几何类与接管后 CSSOM，不能仅抑制 SSR style |
| `crates/rustify-components/css/sdk.css`、`crates/rustify-components/css/rustify.css`、`crates/rustify-components/css/theme-v4.css` | 必要 SSR 样式与 v4 投影；★ `crates/rustify-components/css/candidates.json` 随包分发完整静态类候选 |
| `crates/rustify-docs/Cargo.toml`、`crates/rustify-docs/src/lib.rs` | pipeline/site-ssr/hydrate 依赖分组；★ `crates/rustify-docs/src/site.rs`：DTO、宿主/浏览器分发；★ `crates/rustify-docs-macros/`：site! 过程宏；★ `crates/rustify-docs-syntax/`：宏与 CLI 共用的语法/冷态清单解析，根 Cargo.toml 登记两 crate |
| `crates/rustify-docs/src/config.rs`、`content.rs`、`markdown.rs`、`generation.rs`、`shell.rs`、`style.rs` | 新配置/嵌入 span、白名单、定制页、插槽、主题与产物校验接线（这些短文件名均位于同一 src 目录） |
| `crates/rustify-docs/assets/mode.js`、`crates/rustify-docs/assets/site.js`、`crates/rustify-docs/assets/docs.css`、`crates/rustify-docs/assets/dev.js` | 早期输入闸与失败兜底、演示/源码呈现、building/多行 Rust 诊断；★ `crates/rustify-docs/assets/islands.js` 为外部模块启动桥 |
| `crates/rustify-docs-cli/Cargo.toml`、`src/main.rs`、`src/new.rs`、`src/dev.rs`、`src/report.rs` | 组件模板、工具链检查和命令接线；★ `crates/rustify-docs-cli/src/rust_site.rs` 集中 Cargo/helper/bindgen/缓存与错误映射 |
| ★ `crates/rustify-docs-cli/templates/components/`；★ `tests/docs-ssg/fixtures/components/` | 可运行组件站与公开 API fixture，独立 Cargo workspace、锁文件、host binary、wasm lib |
| ★ `tests/dom-only/hydration/`、★ `tests/dom-only/hydration.spec.ts` | M1 独立宿主 SSR + hydrate + CSR 对照 harness；同源确定 view，直接用 SDK，无 site!/CLI 依赖；产物 target/dom/hydration |
| ★ `rustify-docs.toml`、★ `site/Cargo.toml`、★ `site/Cargo.lock`、★ `site/src/`、★ `site/static/` | 官方站；root config 保证原指南/README 位于项目边界内；site 自成 workspace，根 Cargo.toml exclude 增加 site |
| `xtask/Cargo.toml`、`xtask/src/docs.rs`、`xtask/src/docs_corpus.rs`、`xtask/src/css.rs`、`playwright.docs.config.ts`、`tests/docs-ssg/serve.sh`、`tests/docs-ssg/static-host.mjs` | 特性/图、SDK 候选生成新增 scanner、M1 harness 构建、B 语料演示集合、§9.1 project/站点/MIME 映射；★ `tests/docs-ssg/islands.spec.ts`、★ `tests/docs-ssg/custom.spec.ts`、★ `tests/docs-ssg/dev-rust.spec.ts`、★ `tests/docs-ssg/official.spec.ts`、★ `tests/docs-ssg/harness/measure-islands.mjs` |
| `.github/workflows/verify.yml`、★ `.github/workflows/docs-pages.yml`、`README.md`、`docs/docs-ssg.md`、`docs/themes.md`、`docs/components.md` | 分目标 CI、Mac 构建与 Pages 发布、模板/注册/降级/安装/部署说明 |
| `.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md`、`.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md`、`DESIGN.docs-ssg.md` | M2/M4/M5 实施前设计扩展、实现后的 finish review 与规范同步；只覆盖 P2 新界面，见 §6 |
| `docs/utility.md`、`docs/vellum.md` | M5 修正失效仓库链接；将 vellum 唯一源译成英文，保留 12 页白名单 |
| ★ `crates/rustify-docs-cli/assets/fonts/`、`sources.lock.json` | M3 内置同版七个字体文件及许可/摘要，来源为现有资源；中性目录、无Makepad crate/运行时依赖；详 §5.5 |

Makepad fork、GPU 构建/loader 和现有示例业务无需新增能力；共享 SDK 或组件被改时，对实际受影响的旧示例回归，避免扩大为示例重构。截图/报告/原始证据按需存到 ★ `docs/validation/docs-ssg-p2/`，里程碑事实记录仍按顶部协议保存。

## 2. 模块、接口与依赖

以下为本期新增契约，按本文固定调用/失败语义；不是现有 API 的自述。实现中的局部命名调整需同步本文与模板，不能把契约选择留到验收时。

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类/适配器 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| SDK SSR scope / hydrate | site 宿主渲染与 wasm 分发 | SSR 输出一个 scope/content/overlay；hydrate 保留原 content 节点；runtime、容器、未占用、结构/标识匹配依次校验；文档岛 url_owner=false | SSR view 与浏览器根；复用现有 runtime binding | Owner/context/清理、已有 HTML 身份、拒绝后原 DOM 保全 | 进程内；真实 DOM/runtime，不加 adapter | V-2：初值/ID/ARIA、重复接管/坏根/失败清理 |
| docs site 声明与注册 | 两模板、官方站、CLI 预检/本地 helper/wasm | site! 名称字面量唯一；生成可序列化、deny_unknown_fields 的 props 类型和字段清单；注册函数接收 props/已渲染子内容并返回 view；demo 引用自由函数 | 宿主/浏览器各自的 cfg 分发，同一声明 | 名字、类型验证、源符号、定制页/插槽元数据与页面 scope | 进程内；无插件/动态库 | V-3/V-4/V-6：冷态清单、错误属性、三次同类嵌入、原文源码 |
| rust_site 构建协调 | check/build/dev | 原工程路径编译，编译前/提交前核对输入摘要；源码切片来自本轮读取的原文件字节；不一致丢弃整代重建 | 本地子进程 stdin/stdout 私有 JSON 协议 | Cargo 目标、工具链、源码切片、缓存失效、stderr/Rust 位置、bindgen 文件集合 | 本地可替代；真实临时站点/Cargo/helper；协议故障可用小进程 fixture | V-4/V-8/V-9：编译坏输入、异常退出、代错配、旧代保全 |
| content/generation/shell | 三条作者命令 | 内容白名单先过滤；嵌入/定制页加入统一路由/锚点/资源图，最终 HTML 与 CSS 共用安全闸；失败不返回半代 | 现有分析/生成入口的精确增量 | 内容位置与输出位置关联、全站插槽与元数据、发布资源集合 | 进程内 + 本地文件系统；无新 IO 适配器 | V-3/V-6/V-10/V-14 |
| style / 静态 theme context | 构建与每岛 view | light/dark 来自同一 resolve；外部 CSS 与 fonts；运行态读取相同 snapshot，不重新写主题值；类候选覆盖未来打开状态 | 复用 scanner::scan、style compiler、theme codec/resolve | v4 token、字体路径/诊断、系统模式、隐藏状态和候选缓存 | 进程内；真实编译器/主题/字体 | V-7/V-12 |
| 基础脚本 / islands | 读者页面 | 仅岛页启动；状态在 wasm 外；一次 boot 后逐 scope 接管；任一不可恢复错误使受影响实例所属块终态；恢复为重新加载页面 | DOM loader boot/on_fatal；不复用会替换容器的 show_fatal | 启动闸、资源失败与超时、状态锁、静态快照恢复 | 进程内浏览器/同源静态资源；故障由路由拦截注入 | V-5/V-11：加载/失败/无脚本/重载 |

删除测试：删除 site 注册模块会让类型、名字与相同渲染函数散到 CLI/helper/wasm，删除 rust_site 会让缓存和子进程失败散到三条命令；二者集中真实规则。共用数据仍放在现库，不另造 protocol 包；用真实本地子进程和文件系统验收，不为测试增加公开 adapter。

### 2.1 编译矩阵与作者工程

- SDK/components 增加 `hydrate`，传递 Leptos/hydrate；ssr、csr、hydrate 的包构建分开执行。csr 与 hydrate 同时启用明确编译拒绝，不依靠 workspace feature 并集证明隔离。gpu 继续依赖 csr；P2 不支持 gpu+hydrate。
- rustify-docs 默认 `pipeline`，保留原宿主公开 API；pipeline 启用 `site-ssr` 及 Markdown/高亮/配置/文件系统/CSS compiler/native scanner。site-ssr 只含 serde/注册、serve_site 与 SDK/components/Leptos SSR；hydrate 只含 serde/注册、水合与 DOM SDK。模板 default-features=false，站点自身 ssr feature 转发 site-ssr；hydrate 转发 hydrate。宏/语法库仅在编译宿主运行，syntect/two-face/lightningcss/html5ever/dom_query/rustify-style compiler 不进入 helper 或浏览器依赖图。
- 组件工程有同一 `src/lib.rs`/注册模块、宿主 `docs-render` binary 和 cdylib；ssr 构建宿主，hydrate 构建 wasm32。组件模板/fixture/官方站独立 workspace，root workspace union 不包含它们；纯 content 模板没有 Cargo 工程。作者依赖与 CLI 按下面 R/C 合同配对。
- CLI 引入现有锁版本 `wasm-bindgen-cli-support =0.2.128`，进程内完成 web glue 与 snippets。静态注册/源码按已有 `syn 2.0.119` 与 `proc-macro2 1.0.107` 增加宿主直接依赖和 span-locations；它们在 Cargo.lock 已存在。不引入 Node 工具或新增 parser Major。
- 仓内 CLI 子构建通过 `RUSTIFY_DOCS_CARGO=mbx` 明确选 mbx；仓外默认 cargo。启动 fixture/xtask 按此传入，不能依赖子进程的 CARGO 是包装器。该环境变量只改变编译命令，不能改变源/站点语义；记录实际 compiler、flags、target 和输入指纹。

**作者工程与版本预检**：用户 2026-10-13 选择“库 R → CLI C，两次提交固定配对”。先提交/冻结包含 SDK/components/docs/宏/语法/资源功能的库提交 R，再制作 C：仅修改 CLI 发布身份/版本元数据、模板固定 rev 与相应锁文件及发布文档，不改 R 的库/运行资源。CLI 内置 expected_framework_rev=R；三个直接框架依赖都写 `git="https://github.com/rockie/rustify-ui", rev="R", default-features=false`，features 按宿主/wasm 转发。安装 CLI 用 C，生成工程用 R；两者不要求 SHA 相等，要求属于同一已核验配对。

维护者在 R 可获取后用完整模板生成并验证 Cargo.lock，将该锁嵌入 C；new 仅替换根包名（无依赖根包记录），不动态解析依赖，正常 check/build/dev 全程 `--locked`、锁字节不变。发布版 CLI 的自身 SHA 由 git 安装构建身份读取，库 R 为显式内置常量；组件模板不能用 install_revision 的 PREVIOUS_REVISION 回落。无 CLI 来源身份时拒绝生成组件模板并提示安装可追溯的 C，content 模板维持既有行为。

只读 cargo metadata（依赖取齐后加 offline）核对三个框架 crate 的来源/rev 与 wasm-bindgen `=0.2.128`；不匹配时在 helper/bindgen 前拒绝整代，指出 Cargo.toml/Cargo.lock、期待的 R/C 和成对升级/回滚操作，不转发“另装 bindgen CLI”的建议。升级须备份 Cargo.toml/Cargo.lock 与完整 last-good，显式同时升级 CLI、框架 rev 并生成新 lock 后验 check/build；回滚恢复成对 CLI/manifest/lock/产物。单升 CLI、单升库和错误 bindgen 各有拒绝用例。

仓内 fixture 开发可用本地 path；M2/M4先在独立临时git仓形成 Rtest/Ctest，离线排练沿用 xtask::external 的 `file://` git+rev 改写，由测试harness在编译临时CLI时注入该测试配对和已生成lock。M6再冻结真实R/C及公开来源；模板/字体/文案冻结后测试源码不得不同于待发布版本。依赖取齐后禁网check/build，不能以file://排练替代M7的GitHub安装。对外模板不写工作树path、用户机器路径或测试覆盖开关。

dom-loader.js、dom-runtime.css、islands.js 与 SDK candidates 都由 helper 从工程锁定的库导出；CLI内置shell/dev资产和配对发行版字体表。describe 报告框架 release 标识、runtime::PROTOCOL、岛/helper 协议、注册清单和逐资源摘要；框架 git rev 以 metadata 为权威。CLI 校验配对身份及资源摘要再绑定本轮 wasm，避免从 CLI 取另一版 loader/candidates。

**固定编译档位**：以下 profile 写在组件模板/fixture/官方站各自 workspace 根，不从本仓 fat LTO 档隐式继承。

| 用途 | profile 及参数 | CLI 调用与 bindgen |
| --- | --- | --- |
| dev/check 的 host helper | docs-host：inherits=dev，panic=unwind，incremental=true，debug=0，opt-level=1，lto=false | `build --locked --profile docs-host --no-default-features --features ssr --bin docs-render` |
| dev/check 的 wasm | docs-wasm：inherits=dev，panic=abort，incremental=true，debug=0，opt-level=1，lto=false | `build --locked --profile docs-wasm --no-default-features --features hydrate --lib --target wasm32-unknown-unknown` |
| build 的 host helper | docs-host-release：inherits=release，panic=unwind，debug=0，lto=thin | host 同上，profile 替换为 docs-host-release |
| build 的 wasm | release：panic=abort，debug=0，lto=thin，codegen-units=1 | wasm 同上，profile 替换为 release |

bindgen 两档均使用进程内固定版本的 web target，debug=false，保留完整 snippets/相对 imports，不依赖外部 wasm-opt/wasm-pack。host 与 wasm 按此顺序串行构建、各复用稳定 target；不依赖并行绕过 Cargo 锁。dev 的 ≤2 s 按 docs-host/docs-wasm 测量，读者的 ≤1 s 按 release 测量。M2在同一代理站记录五次两目标编译与编译后渲染/加载时延，M4接入完整dev后首测真实保存链路；编译耗时是观测项，不擅自新增上限。

### 2.2 helper 与岛协议

私有 `protocol_version=1`，描述/批量渲染两类请求；stdout 只允许一个完整 JSON 响应，编译日志和作者诊断走 stderr。请求带 generation 指纹、site 配置、每次嵌入的 key/name/props/children、scope seed，以及定制页/插槽上下文；响应回传相同版本/指纹，每项带 HTML 或诊断。describe 同时回报 §2.1 的版本/资源 metadata，并与冷态静态清单比较 names、字段类型/默认值声明、函数身份、页面与 slots；任何差异均失败，不以已编译清单覆盖静态预检。版本错配、非零退出、无效 JSON、漏项/重复项均失败整代，不猜测修复。

一次分析读取声明/源码，一次宿主构建、一次 wasm 构建、一次批量 helper 调用；不逐页面/逐演示启动 Cargo 或子进程。浏览器 metadata 只含该页需要的 key/name/props/children、主题/语言与绝对基路径资源 URL；用 HTML 转义的 data 属性或 template 承载，禁止 inline script（含 application/json）、Rust 表达式、执行字符串或机器绝对路径。

### 2.3 最小注册与调用合同

`site!` 是 rustify-docs-macros 中的过程宏，由 rustify-docs 重导出，生成 `SiteRegistry`。宏和 CLI 都调用 rustify-docs-syntax 的同一 parser/规范化清单；它仅承担声明语法和定位，不依赖 Leptos、SDK 或内容管线。选择过程宏而非 macro_rules，避免 CLI 再写一份文法；采用共用语法库而非复写 parser，以冷态清单/describe 对照保证展开一致。

声明区固定为 components/demos/pages/slots，空区可省略。组件名是 Rust identifier，其字面文本即 Markdown 的 name；一个属性声明包括字段名、可 serde 的 Rust 类型及可选默认值，生成 props 带 serde 默认值与 deny_unknown_fields。默认值只在可信 Rust 中执行，CLI 只读声明；adapter 接收 typed props 与已校验子内容，再调用站点 Leptos 组件。demo 必须直接引用函数 path，不能以匿名 closure 代替其源码身份。

SSG 支持合同（用户 2026-10-13 确认）：构建只执行同步 view 生成确定初态，不等待异步数据或打包资源序列化/错误/未完成chunk结果。Resource/Suspense/Transition/ErrorBoundary 依赖上述跨端共享数据的用法不能无条件接管；作者把异步请求延后到接管完成后的浏览器工作，使用同一确定初态，或明确 client-only 固定占位。helper 对初始生成触发的跨端资源/错误/未完成块需求给组件/围栏定位诊断与该建议；实现时用真实依赖上下文反例核实检测，不用字符串扫描承诺识别所有作者代码。

islands.js 在启动wasm前校验/初始化空的 __RESOLVED_RESOURCES、__INCOMPLETE_CHUNKS、__SERIALIZED_ERRORS；不加载内联HydrationScripts、不承诺非空异步跨岛命名空间。SDK独立hydrate消费者须在外部脚本满足相同前提。V-2/V-5覆盖三同类岛、跨端异步反例的生成诊断、client-only恢复，以及接管完成后异步更新；空数组是锁定框架的运行前提，不能据此把异步静态生成写成已支持。

```rust
pub fn site() -> rustify_docs::SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 0 } =>
                |props, children| views::counter(props.initial, children),
        }
        demos {
            CounterDemo => crate::demos::counter_demo,
        }
    }
}
```

示例是拟交付的作者契约。pages 每项声明 route/title/可选 description/渲染函数；slots 以 §5.3 的 Slot enum 引用渲染函数，不接任意字符串名字。定制页/slot 的渲染函数接收 RenderContext；其公开 `demo(name)` 方法按已注册 name 输出与 Markdown 演示相同的画面、状态、函数/全文件源码和岛记录，不提供第二种渲染管线。无效 name 在 helper 渲染时报定位错误。

模板 host binary 调用 site-ssr 提供的 `serve_site(site())`，wasm 入口调用 hydrate 提供的 `hydrate_site(site())`；作者只维护一份 site 函数。仅静态 HTML 的页面不进入 wasm 分发请求。

SDK 的新增 `hydrate(container, MountConfig, view)` 返回 AppHandle，按现有 mount 的 Result/UiError 范式；SSR 根带显式 server-rendered 标记，运行时另有独占标记，不能把预渲染的 data-rustify-scope 当成“已占用”。根/seed/标记校验失败分别登记 scope 诊断；闸门失败不写 DOM。通过后调用 hydrate_from，并在传入的 view 闭包内部建立 ScopeRoots、ActionSink、router、OverlayStack、ID allocator、locale/静态主题，再执行作者 view；外层 provide_context 不会传入新根。AppHandle 持有 UnmountHandle，清理由其根 Owner 与 runtime 注册资源承担。新增错误种类在 diagnostics.rs/文档/V-2 同步；原 mount 的空/占用容器语义不放宽。

岛 HTML（含 slot 内岛）的 `<!>` 标记保留到最终产物；shell 只清理不参与接管的部分，岛通过独立占位拼接，禁止整份 shell 的全局 replace。DataTable/Tree/Progress/ColorField/Splitter 的初态几何由确定 props 生成外部样式类；接管后显式以 CSSOM 建立/更新动态规则，不依赖 style: 的 hydrate 空操作，也不产生 style 属性。

## 3. 派生数据、缓存与兼容

| 对象 | 关键字段与约束 | 生命周期/权威来源 | 查询与索引 |
| --- | --- | --- | --- |
| config v1 增量 | ★ `rust.manifest`、`rust.registry`、`theme.document` 为项目相对；★ `content.include` 为 content.dir 相对的显式文件列表；缺省保持 P1递归；未知键仍错误 | rustify-docs.toml；theme.document 与 primary 同配时报定位冲突，避免两套主色权威；logo 独立 | Config 一次读取；检查越界、私有路径、输入/输出重叠；include 在读取/监视前筛选，详§5.6 |
| 导航增量 | ★ nav.groups 每组在 dir/items 之外可选 routes；三者互斥，routes 为不含 base 的定制页 route 列表，次序即导航次序 | 定制页标题为默认 label，未注册/重复 route 诊断；P1 dir/items仍按内容路径 | 合并页图后解析，与内容页同样有当前项、prev/next；/首页不加入指南/分类集合 |
| 静态 site 声明 | 组件名/字段/默认值/adapter；演示名/函数；定制页 metadata；有限 Slot enum | 作者 Rust 声明一次定义；字段类型由生成 props 的 serde 解码验证；构建前提取 names 用于冷态预检 | 名字 map、route map；重复名称/路由拒绝 |
| Embed ★ | Markdown 来源 span、name、序列化 props、子内容、page 内 key、scope seed、静态/interactive/client-only 分类 | Fragment 解析派生；key 由页面与出现顺序确定，不用时钟或进程计数；子内容同一 Markdown 安全管线 | 页面中有序 vector；每岛 ID context 使用 key 前缀及从固定起点开始的计数 |
| Demo source ★ | 编译符号对应的项目相对路径、原始 UTF-8 文件、精确函数范围、内容摘要 | 从同一 generation 的原文件切片；模块解析验证符号路径，不能把同名函数或另一个文件误当源码 | 同文件读取/语法解析一次；按符号 map 取片段，不正则数括号 |
| Rust build cache ★ | CLI/协议版本、lock、Cargo metadata、功能/目标/flags、注册及站点/本地 path 依赖/build 输入摘要 | CLI 私有 `.rustify-docs` 工作区与 Cargo target；构建成功才更新可用索引，失败保留成功代；不发布缓存 | 一个 site generation 一次缓存命中判定；Rust 不变仅重新渲染，不重新编译 |
| 岛资源组 ★ | 有序 glue/wasm/snippets/runtime 文件集合摘要及相对文件名 | bindgen 完整输出生成不可变 `assets/islands/<digest>/` 组；内部相对 imports 保留，避免逐文件改名破坏 glue | 外层组目录按完整字节摘要；同一代 HTML 引用同一组 |
| 定制页/插槽结果 ★ | route/title/description/index 元数据、静态 HTML、依赖与岛记录 | host helper 派生；与 Markdown 页面共用路由/锚点/资源/样式检查；描述留给 P4 索引 | 合并单一页面图；重复路由（含 /、404 和保留输出）报错 |

迁移只涉及源配置和静态产物，不引入数据库。P1 config/schema、manifest 及内容模板继续接受；新键由新 CLI 理解，旧 CLI 明确拒绝未知键。私有岛协议与指纹成对发布，不要求旧 HTML 接管新 wasm。每次 CLI 代码、内置模板/资产或依赖变化都递增其独立 patch 版本，沿用当前 0.1.4 的版本纪律；版本、依赖/lock、模板与作者安装文案在 M6 第一阶段冻结并记录候选，不到 M7 才改产品候选。

先增加新 feature/键/API，再生成两模板和官方站并验证，最后切换托管产物。原内容命令不产生 Cargo/helper 调用；官方站白名单不修改旧工程的默认递归语义。发布沿用 `publish.rs` 的锁、staging/backup 与恢复，不在既有 dist 中原地混写。回滚使用旧 CLI+对应源配置/lock、旧完整产物；不能只替换 wasm 或 CSS。不要删除旧安装候选和发布 artifact，保留直到新站线上验收通过。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| P1 内容图/发布事务/dev | 已核实足够 | content::Snapshot/FragmentKey；generation::generate 的终态校验；publish 的独占/共享锁与恢复；dev 保留 good 并按字节比较 | 在同一流程接入 Rust 候选与新页面；变更前用 V-1 保留行为基线 | 新环节失败不提交，原产物/成功代保留 |
| Leptos =0.8.22 | 已核实缺口 | hydrate_from 的新根不继承外层 context；HydrateSharedContext 的全局数据、tachys style: 空操作与元素类型检查 | 不升级；按§2.3建根/保标记/改几何，生成确定初态、外部脚本预置空全局量；异步工作在浏览器接管后运行 | 根闸与初始化失败由 JS 保全静态内容；结构一致性由 V-2 测试期对照保证 |
| DOM SDK / components | 已核实缺口 | mount_to、wasm-only 导出、thread_local 递增 ID；DOM loader boot/on_fatal 与 runtime::PROTOCOL | §2/§5.2 的 SSR/hydrate/ID 增量，保留原默认和 GPU API | 闸门顺序固定，接管拒绝保全 DOM；trap 中止所有所属 scope |
| style scanner/compiler | 已核实足够 | scanner.rs::scan(content, extension)、style.rs::compile_utilities/minify；现 docs 未启用 scanner | 宿主增加 scanner feature；SDK 候选随包导出；一次合并候选交给 compiler | 非法候选/资源或 compiler 失败定位诊断，不切代 |
| Theme Studio / SDK theme | 已核实足够 | codec::import_json/export_css、resolve、fonts::font_context；export_css 输出 scope selector 和字体资源引用 | 使用同一验证/解析；为 site 根/system 模式做投影，资源按基路径打包 | 无效主题报错；既有字体 fallback warning 保留，不静默重设主题 |
| 原生 Rust / bindgen | 已核实缺口 | new::doctor 只 rustc --version，CLI 无站点编译；xtask::assemble 已内置 bindgen support | Rust 站点检查 wasm32 目标；复用原生 bindgen，host/wasm 目标分开 | 缺 rustup 目标给 `rustup target add wasm32-unknown-unknown`；检查完成前 new 不写半成品 |
| GitHub Pages | 已核实缺口 | origin 指向 rockie/rustify-ui；仓库 GET 显示 has_pages=false，Pages GET 为 404；现 workflow 仅 verify/evidence | M7 配置 Actions 来源并发布，经实际 URL 验收；构建用 Mac，deploy 只消费 artifact | 未启用/无发布授权/部署失败保留本地候选及旧部署，M7 不记完成 |

托管默认位置、meta 策略范围来自 PRD Q4/D11；不能据本地服务器推断 GitHub 设置已启用。GitHub 的 [custom 404 说明](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site)要求发布源提供 404.html；[自定义 workflow 部署说明](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)作为 M7 操作依据，执行前回读当时规则。Pages token/权限只来自 CI/用户现有登录，不写密钥入配置或 artifact。

## 5. 核心机制与作者链路

### 5.1 嵌入语法与校验顺序

采用能在 GitHub 显示的围栏代码块，语言标记为 `rustify-component`、`rustify-demo`；框架识别完整独立块，普通代码块不执行。载荷使用 YAML，源码可读，不在 Markdown 中求值 Rust。

```yaml
name: Counter
props:
  initial: 2
children: |
  A short **explanation**.
```

演示块只引用声明中的 demo name；其函数无外部运行期参数，内部自行建立初始状态。不额外用 include 区域充当演示源。内置提示框继续支持 GitHub alert；内置 Tabs、CardGrid、Steps 使用相同注册与 SSR 管线，子内容模型在 M2 用正反 fixture 固定。内置静态组件不要求作者添加 Rust 工程或加载 wasm。

M2 固定的内置载荷：Tabs 的 `props.tabs` 为 `{label, content}` 数组；CardGrid 的 `props.cards` 为 `{title, link, description}` 数组，`columns` 默认 2、范围 1–3；Steps 的 `props.items` 为 `{title, description}` 数组。字符串作为转义纯文本，额外 `children` 走同一 Markdown 管线；无脚本显示 Tabs 全部面板，基础脚本提供页签切换，均不生成 wasm 岛。

校验次序：配置/路径及白名单 → 解析 Markdown span、静态注册清单、字段名/声明分类与普通内容问题 → 并行于问题收集完成两目标编译检查 → 宿主 typed serde 解码和 SSR 批量响应 → 合并路由/锚点/资源 → 最终 HTML/CSS 策略检查 → 发布。存在预检错误也要收集可独立获得的编译诊断；不在遇到第一个未注册名时提前短路掉 Rust 错误。

注册清单的名字、可用字段与默认值来自 site!，不依赖上轮编译缓存。属性标量/序列/对象按生成类型反序列化，未知字段、重复字段、缺少必填字段与类型错分别定位；重复 YAML 键必须拒绝，不能用解析后的最后一个值掩盖错误。诊断关联围栏/属性的原始 Markdown 文件、行列和建议；Rust 编译错误另保留 Cargo JSON 里的 Rust 路径/行列与对应引用处。JSON 输出沿用现有 Diagnostic/report 结构，一次列出全部独立错误；一般日志不污染 stdout。

含原始 HTML、恶意 URL、脚本/事件/内联 style 的 props/children 不能绕过现有检查；可信组件也不豁免生成结果闸。块内子 Markdown 不允许嵌套交互岛，避免根相互拥有；提示需明确改为相邻块。这限制嵌入层级，不限制组件内部正常 Rust 组合。

### 5.2 静态生成、源码与浏览器接管

1. 从静态注册声明解析真实函数符号，读取其 Rust 模块/原文件；按 AST 的 source span 取得函数原文。包含函数的属性/签名/函数体，保留原缩进、空行、注释、Unicode 和换行，不从 token stream 重新生成源码。声明指向重导出/生成函数而无法唯一定位时，报有建议的源码诊断，要求直接引用站点内的源函数。
2. 编译并批量渲染同一 generation 的站点代码。SSR scope 提供与水合一致的 locale、主题、ID allocator 和共享状态上下文；view 由真实函数执行。所有依赖浏览器测量的初态使用确定的输入值；动态测量在水合后执行。浮层初始关闭的示例输出真实触发控件，DataTable 使用确定的初始可见窗口；不能为 24 类页面用另写的静态假控件代替真实函数。
3. 输出组件 scope、独立源码区域和状态区域。SSR 字段的 value/checked/disabled/readOnly/selected/ARIA 与首个浏览器 view 相同；从外层禁用交互，不覆盖组件自己的受控值。host 的并行任务和浏览器的接管顺序不能改变 ID；每岛使用带 key 的同一 seed，CSR 未提供 allocator 时保留原 next 行为。
4. 函数/全文件展示与复制使用原始切片，而非高亮器归一化后的文本。保留原有 raw template 复制范式；CR 需实体编码保留到 DOM 原文，测试 LF/CRLF、末尾换行与非 ASCII。全文件作为源码页签的可展开文本和复制目标，检查隐私后内嵌 HTML；没有 .rs 下载文件或原始 Markdown 副本。
5. 无脚本时：首帧、源码、普通导航可读，原生输入放在 scope 外层的 disabled fieldset，提示位于 noscript；不设置整块 inert 隐藏无脚本读者的可访问内容。脚本启用时同步 head 脚本先安装事件捕获闸，对未 ready 的岛拦截点击/键盘/输入；启动时设置 inert/aria-busy，再调用外部 islands 模块。状态区域与源码在锁定 scope 外，首帧两种条件都是同一 SSR 初态。
6. 一个页面一次 DOM loader boot，随后按已固定 key 逐 scope 调用新增 hydrate；文档岛不拥有 URL，不劫持 shell 普通链接。只有真实完成接管并确认控件可响应，才把其状态置 ready、解除闸并保存 AppHandle。仅 `boot()` resolve 不能计作水合完成。
7. 脚本资源加载、编译、初始化、水合或后续运行失败时：基础脚本终止相应实例所有岛，锁住输入、撤销浮层/主题订阅/监听，并用 JS 持有的初始 DOM 快照恢复 SSR 画面；保留源码及静态 shell。不得调用替换整块源码/内容的 SDK show_fatal；由 on_fatal 接到站点状态渲染。
8. 基础脚本监控启动模块 import/script error；启动超过 30 s 转 failed（本计划的保护截止值，独立于 ≤1 s 性能预算）。捕获晚到的完成不能复活已 failed 的代；异常类型/岛 key 记录到 console。失败入口使用整页重新加载以重新获取一致的 HTML/资源，不在死实例上再次派发调用。

无法 SSR 的外部组件须显式声明 client-only，注册同时给固定尺寸 class 与说明。占位参与最终类/资源检查，只有 hydrate 后才 CSR 创建其内容；不把它伪记为 WORK-F4/AC-1 或 WORK-F5/AC-3 的 SSR 证据。官方站的 24 个演示必须用可 SSR 的真实函数；client-only 不用于满足其分类覆盖门。

### 5.3 定制页与插槽

定制页声明规范 route、title、可选 description 和静态渲染函数；route 不含部署 base，输出仍为目录 index.html。统一记录来源及可索引正文，调用同一 shell；自定义页的动态局部仍用显式岛，不能接管 shell。无描述的自定义页仍可浏览，P4 再决定 AI 索引输出，当前不生成任何 AI 产物。

Slot 为有限集合：header、footer、sidebar_top、sidebar_bottom、before_content、after_content、not_found。未覆盖者保持默认；not_found 只替换错误正文，其余 slot 含 footer 同样作用于 404。覆盖 header 时仍保留框架负责的 CSP/meta/基础启动与可达的导航/主题入口；每种 slot 的输入包含 route/lang/base，不给作者构造部署 URL 的第二套规则。slot 输出可含显式岛，最终仍进入同一安全/样式图。

所有页面及插槽完成 SSR 后，再按最终浏览器文档上下文收集 ID/链接/图片/CSS 资源，验证同页和跨页引用；不能只通过 Markdown Fragment 的旧锚点集合。自定义页与 Markdown 重复路由、重复 document title、生成/保留目录冲突给定位诊断；不按注册顺序覆盖页面。

### 5.4 组件模板、命令与 Rust 热更新

- `new --template components` 先检查 Rust/wasm32 目标，再检查目录，使用现有临时同级目录/rename 的完整写入流程。模板含至少一个能调属性的组件、一个源函数演示、两页指南、README、锁文件和 host/wasm 入口；生成后不修改即可 dev/build/check。content 模板仅有原内容，不变为隐含 Rust 工程。
- 站点显示名保留目录名；Cargo 包名将 ASCII 大写转小写、连续空白/非 ASCII 字符/分隔符归并为 `-`、去掉首尾 `-`；空结果用 docs-site，非字母开头加 docs-。如 My Docs → my-docs；显示名仍为 My Docs。该值一次生成并用于锁文件根包条目/入口，V-8 覆盖空结果与非法开头，不到首次 dev 才报 manifest 错误。
- 组件 README 必含：new 的首条命令、五命令/工具链提示、冷启动/计时说明、写函数→site!注册→rustify-demo围栏→dev查看的完整示例、props/children/SSR与client-only边界、定制页/RenderContext::demo/七类slot、主题/字体/include/导航、Rust诊断与last-good、成对版本升级/回滚、静态托管与meta策略。M4 从空目录按该 README 试走并检查链接/示例与真实模板一致；此预检不代替 V-13 真人。
- `check` 分析两种站点；Rust 站点做冷态静态注册预检、host helper 构建及 wasm32 编译检查，汇总错误，不生成或替换发布目录。可写 Cargo/私有编译缓存，但不修改作者源码/config/已有产物；更新帮助中原有“不写文件”表述的准确范围。纯内容 check 维持原有零站点编译行为。
- `build` 先准备 host/wasm/bindgen 完整候选，再渲染并全量校验，最后进入现有发布事务。无错误才生成 manifest 和可预览目录；失败代码、协议或输出校验都保全之前成功目录。
- `dev` 继续复用现有 notify/Filter/Builder/Outcome/事件流，增加 building 阶段和 Rust 文件/manifest/lock/本地 path dependency/build 输入依赖。源码、声明、主题和解析缓存有各自内容摘要；Rust 不变的 Markdown/配置/资源保存只运行预编译 helper，不再次调用 Rust 编译器。先绑定监听并打印地址，再在后台做首次编译；无 last-good 时显示“首次编译中”的静态诊断 shell，终端单行显示 host/wasm/render 当前阶段；增量期间显示上一代与 building 状态。两目标编译完成事件包含 epoch 毫秒、generation 和各目标结果，供 V-12 对照。
- Rust 编译期间 old good 与其 glue/wasm/源码继续成套服务；缓存记录不能先替换旧资源。成功后 host/wasm/source/HTML/CSS 作为一代提交，按现有 changed_at 逻辑刷新受影响页并保持滚动；失败显示 Rust 文件行列，good 和其源码保持不变。
- watcher 排除 target、输出、CLI 私有工作区；一轮只运行一组 Cargo，保存合并后追到最新输入。构建期间再次保存时，候选需在提交前与输入指纹核对，过时代丢弃并重建；不得把新源码配到旧 wasm。跨目录 path dependency 同样记录，并在输入不可读时保留恢复监视。
- 选择原地构建而非每代复制输入快照，保留 Cargo 的包路径/mtime和作者诊断路径；代价是变化中的源码可能让本轮编译失败或失效，必须合并保存后重试。私有临时目录仅存 helper响应、bindgen和生成候选输出。注册/源码切片取自编译前读取的原文件字节，编译前、渲染后及发布前重新核对全部构建输入摘要；有变化丢弃整代并重建，不重新读取新源码拼到旧 wasm。连续保存、原路径行列与 Cargo JSON fresh 计数纳入 V-9。

M4 更新 docs/docs-ssg.md 的围栏/site!/client-only/定制页与slot/theme.document/include/导航/编译档位/版本配对，并将“check不写文件”改为“不修改源与发布目录，可写私有编译缓存”。docs/themes.md 归 M3，补静态主题和字体合同；README 与模板的固定版本、安装/部署段落归 M6 冻结、M7 复核，不在验收后静默改引导。

### 5.5 主题、类候选与字体

`theme.document` 指向 Theme Studio JSON。通过现有 import_json、resolve 和 font_context 验证 light/dark，继承 256 KiB 导入限制和未知 token/fallback 诊断。同一解析结果供 site 的 :root 和每岛 scope，使用 v4 token/半径/字号/字距；默认未导入仍使用 P1 外观。system 模式继续用 CSS 媒体查询加 mode.js 存储首帧，浏览器将 root 模式映射为每岛主题上下文，岛不各自保存偏好。

静态主题 binding 只提供 EffectiveTheme/locale，CSS 在构建期已经投影；避免默认 ThemeScope 再写/覆盖导出的根。沿用 export_css 的受校验属性投影与资源规则，对站点 root、scope 和 system selector 做明确适配，不用未经核验的字符串替换整个导出样式。用户 2026-10-13 选择 CLI 内置同版字体/许可，产物只打包引用字体；SDK 的 makepad_widgets/resources 仅作为 resolver 逻辑标识，通过固定资源表映射到 CLI 内置字节。所有字体输出统一为 `assets/fonts/<digest>.<ext>`，CSS 资源映射及注释不保留 Makepad 路径；缺字节报错、未知 family 保留 resolver fallback 诊断，不远程下载。

FONT_FACES六字面与FONT_GLYPH_FALLBACKS去重为七文件：IBMPlexSans-Text.ttf、NotoSans-Regular.ttf、LXGWWenKaiRegular.ttf、NotoSerif-Regular.ttf、LiberationMono-Regular.ttf、jetbrains_mono_variable.ttf、NotoColorEmoji.ttf。M3复制现有同版字节到CLI assets/fonts，保留字体修改/版权及各自许可并在sources.lock.json登记摘要；C内置表须匹配R的FONT_FACES/回退与资源摘要。现有Liberation Mono Version 1.04实际使用GPL v2及原发行版字体例外，内置对应许可和1.04.93.devel首选源码/构建脚本归档，仅引用该字面时随资产输出；其余六字面为OFL1.1。helper不用编译这些字节，CLI完整依赖图仍无Makepad。CSS只导出本主题引用的字面及既有glyph回退，随生成资产带相应许可；默认无theme.document仍用P1系统字体且不包字体。

七文件源字节合计/CLI体积增量与发布/实际请求的字体体积在M3记录，沿用既定V-12预算，不另设假定上限。V-7用英文/中文/emoji样本比较同主题Theme Studio字节/加载与中性路径；纯内容站在git安装CLI、禁网环境也能导入同文档构建，不要求作者持有Makepad目录。

候选合集为：最终 HTML、基础脚本状态类、所有已声明站点源码中的完整 Tailwind class 字面量、SDK 隐藏交互状态的随包候选清单。宿主复用 `rustify_style::scanner::scan` 并缓存输入摘要；扫描站点 Cargo 源集和 metadata 给出的本地 path 依赖，不扫描整个仓库、target、ref 或计划文本。M2为xtask css新增scanner/候选输出与--check漂移检查，生成SDK `candidates.json`并由组件常量导出，使describe与首次打开态测试已有真实资源；M3接主题文档后复核候选/样式。现xtask仅调用Tailwind CLI，不已有此能力。helper导出工程锁定版本的candidates，已安装CLI无需再读仓库路径或registry源码。完整运行态分支类即使静态初态未出现也生成；拼接未知类名须作者写完整候选，不声称支持任意运行态CSS。

最终 CSS 仍由内置 compiler/minify 生成，检查 URL 属于本代发布集合；新类生效无需额外作者命令。SDK 类变更同时再生既有 rustify.css 和 candidates，验证两者对齐。动态控件几何保留必要 CSSOM/外部规则机制，不让 host view 输出 inline style。

### 5.6 官方站内容来源

根配置声明 content.dir=docs，content.include 以 docs 为基准，只列 PRD 的 12 个文件：quickstart.md、architecture.md、navigation.md、workspace.md、forms.md、i18n.md、data.md、themes.md、utility.md、compatibility.md、components.md、vellum.md。不支持 glob，越界/私有路径拒绝；未配置 include 保持 P1 递归行为。配置 include 时先确定文件集合再读取/监视，未列出 Markdown 与无引用的非 Markdown 不枚举字节或触发 dev 重建；仅按已选页面引用显式读取图片等依赖，独立 static 目录仍按既有安全规则处理。docs/validation 写入验收证据不得触发重建；不以侧栏或草稿模拟白名单。

站点 root 位于仓库根，Rust 工程位于 site。首页为静态定制页：H1/一句定位、Quick start 与 Components 两个主操作、正文 README 介绍、按 DOM/按需 GPU/静态部署三个已存在能力组织的特性网格；1280 px 主操作在首屏内。README 中标题下至 Quick start 前的介绍仍由 Markdown writer 从同一原文渲染，不复制第二份简介；M5 将 README 的“Start here”改成显式指向 README Quick start 的链接以保留脱离文件后的语义。

README 是显式构建依赖，片段内链接以仓库根为源基准：docs/quickstart.md 等已发布指南转站内路由，README其他锚点/未发布文件转 repo URL。指南的相对链接仍以各源文件所在目录解析。新增“content 内但 include 排除的已存在非私有文件 → repo URL”规则；只验证其存在/边界，不读取或发布正文，缺 repo 配置报定位错误，不能用扩大白名单解决。M5 修正 utility.md 中错误的 protocol.rs:96 链接为真实文件和有效锚点，并对 12 指南的原始HTML/标题/链接运行零错误 check。标题/描述默认沿用 writer，vellum 的语言处理按下段决定。

24 个分类以 `rustify_components::CATALOG` 为完整性来源，每类注册为 `/components/<category>/` 定制页、至少一个真实 DOM demo，由 RenderContext::demo 引用 site 内同一函数。映射只在官方站声明一次，nav.groups 的 routes 按 CATALOG 顺序加入 Components 组；Guides 的 items 为 12 源文件。侧栏集合等于“12 指南 + 24 分类”，首页两主操作均可达，不只计数。GPU 段仅保留文字和对应示例链接，绝不引用 GpuRegion/Makepad。

用户 2026-10-13 选择在 M5 将 docs/vellum.md 唯一源译成英文，保留原事实、数值和链接，不生成另一份站点译文、不删出12页白名单。交付后首页/指南/分类均为英文，组件locale明确英文；所有页面lang须与正文相符，当前中文源不能在翻译完成前算英文内容出口。P3语言路由/翻译框架和P4中文首页/快速开始仍按原分期。

## 6. 前端与交互

沿用已批准的 [阅读表面](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 和 DESIGN.md 的语义 token：正文为中心、细分隔、蓝色链接/焦点，1280 三栏、768–1279 两栏、窄屏一栏；Theme Studio 工作台结构不移入文档。演示是正文的一段可运行证据，视图上方为画面与持久状态，下面为函数/完整文件源码页签和复制，不添外部品牌或工具链细节。

设计前置：M2 在演示/岛状态 UI 实施前使用 impeccable 扩展阅读 brief，定画面/状态/Reload位置、函数与全文件页签、长文件高度、client-only占位、空/失败/复制反馈与AA/减少动效。M4 实施 building/Rust多行诊断前扩展已有 dev brief（它已列 building，但现资产未实现），定首次编译与旧版提示/终端反馈。M5 页面实施前扩展首页/分类/导航设计。每次实现后 finish review 保存 verdict/截图并更新 DESIGN.docs-ssg.md，V-5/V-9/V-11 按相应新基准验收；旧 P1 brief 不充当新界面已经获验的证据。

| 状态 | 读者看到与可执行的动作 | 转换与可访问性 |
| --- | --- | --- |
| no-script | SSR 画面、完整源码；“Enable JavaScript to interact” | noscript 说明；内容仍在可访问树，普通导航/代码可选取 |
| loading | 相同初态与短加载提示 | 仅交互 scope 锁住，status 外置 role=status/aria-live；不出现启用脚本提示 |
| ready | 实际控件响应，状态提示收起 | 解除闸不改变尺寸，保留作者 disabled 状态；源码页签/复制由基础脚本提供 |
| failed | 恢复初态、错误说明、Reload page 入口 | role=alert，一次通知；所有失效块明确标识，正文/导航/源码可用 |
| client-only | 固定区域与浏览器要求说明 | 禁脚本保留占位，不能用于声称 SSR 验收通过 |
| dev building/error | 上一代页面继续显示、构建中/文件行列错误浮层 | 沿用现 dev 面板，可关闭、复制路径、修复后清除；成功才刷新 |

源码空值属于注册/构建错误，不向读者发布空演示；空 children 正常。长文件内部滚动、复制状态与失败反馈沿用 site.js，失败不清空原文。失效状态高度与加载区占位保持稳定；加载动画只用既有短反馈，并尊重 reduced motion。

新增控件遵循 SDK 现有键盘/焦点规则，三同类岛 ID 唯一，浮层不占用 shell 的焦点/地址栏；同一岛的 Tab/ARIA 关联不跨到另一岛。手机 375 px 和 200% 缩放无页面横向滚动；弹层边界、关闭、焦点返回要实际操作。Default 与导入主题均检验浅/深/系统模式，错误/加载状态也须 AA；不借助截图掩盖真实 class 缺失。

浏览器主路径使用已有 Playwright 通道与当前可用真实 Chrome/Safari；引擎/模拟/实体设备分别登记。P2 是独立静态站，无 Portal 宿主链路。最终从实际 Pages URL 进入；本地产物 preview/root/subpath/meta 是必要补充，不能替代 WORK-F16/AC-3 的线上出口。每次保留 manifest/build_id、URL、console/CSP、失败请求与必要桌面/手机截图；没有运行环境就如实记未验证。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N1 | PRD AC-4，用户 2026-10-12 确认；P1 测量合同继承 | 每个最大演示页冷缓存五次中位：FCP→全部岛真实可交互 ≤1 s；水合演示布局偏移 0 | 性能不达标不关闭阶段门，不改为下载后计时 | 仅岛页请求；一页一次 boot；宿主依赖剔出 wasm；岛页预加载资源，全部岛立即水合 | V-12；V-5 的延迟资源与零位移 |
| WORK-N2 | PRD AC-4，用户确认；P1 AC-1–3 回归 | 增量编译完成→显示新代 ≤2 s，五次中位；Markdown 保存不调用 Cargo | 不达标保留未验收，不把 Cargo 完成/服务端切代当浏览器可见 | 缓存编译 helper、一次批量渲染、共用字节差分/SSE | V-9、V-12 |
| WORK-N3 | shell::CSP / PRD D11；P2 无延伸 D-04 决定 | header 完整策略/meta 去掉 frame-ancestors；CSP 0；当期兼容矩阵 | 禁止增加 nonce、unsafe-inline/eval 或第三方资源解围 | 同源外部脚本/样式与绝对 base URL；实际托管404/MIME核对 | V-11、V-15 |
| WORK-N4 | P1 features/toolchain 与本期新 graphs | host/wasm/CLI/两模板/官方站无 Makepad，作者无第二工具链 | graph/产物或工具链门失败阻止交付 | 分目标 feature、bindgen 内置、Cargo 在需要时运行 | V-8、V-10、V-14、V-15 |
| WORK-N5 | PRD P1 AC；批准阅读布局 | 新块 AA、键盘、200% 缩放/375px；动态状态朗读 | axe 不替代键盘/读屏和真实布局 | 独立状态/源码区域，稳定尺寸，semantic fieldset/status | V-5、V-11 |
| WORK-N6 | publish 与 dev 的已有事务 | 同输入字节一致；Rust/渲染失败保全上一代；过时代不发布 | 编译/协议/输入变化失败即放弃候选，下一轮重建 | 源/lock/目标/flags 摘要、不可变资源组、锁、staging、事件流 | V-4、V-9、V-14 |
| WORK-N7 | html_check、最终文档/CSS URL 闸 | 每个新来源均在最终发布集合验证；无路径/密钥泄漏 | 拒绝 unsafe HTML、private/越界资源或同源外未授权请求 | props/源码转义、白名单、typed serde、子内容同管线，信任作者 Rust 但不豁免产物 | V-3、V-4、V-6、V-7、V-10、V-11 |

没有新增服务端身份、数据库或远程调用。运行期遥测不采集用户输入或搜索词；观测入口仍为 CLI 的 text/JSON diagnostics、dev 事件与浏览器 console/status，新增只需规则名/阶段/岛 key，不加入第三方监控。缓存可保存编译输入摘要；导出的 manifest、HTML 和公开诊断不得含机器绝对路径、env 值或私有 Cargo 文件。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 冷态未注册名 + Rust 编译错误 | 本次 check/build；dev 的新代 | 无源修改，无成功产物丢失 | 两处定位诊断；dev 保留旧页 | 静态清单预检 + Cargo JSON | 修复声明/代码后重新生成 | V-3/V-4/V-9 |
| helper crash/错误协议/SSR panic | 本次 generation | 新候选丢弃，已有代保留 | CLI 或浮层显示错误，不能显示空成功页 | exit、protocol/指纹/项集合检查 | 重建/修代码；helper unwind 或异常退出都转诊断 | V-4/V-9 |
| glue/wasm/snippet 404、网络拒绝、初始化/水合失败或启动超时 | 同页所属实例的全部交互岛 | 静态源不变，无用户持久数据 | failed 与重载，初态/源码/正文可用 | script/import catch、30s 保护截止、on_fatal | 整页重载；若发布不完整则恢复整组 artifact | V-5/V-11/V-15 |
| 交互运行 trap，浮层打开时失败 | 同页实例各 scope | 内存交互状态失去，恢复原始初态 | 全部失效块明确失败；不遗留阻挡 shell 的浮层/监听 | DOM loader/runtime fatal callback | JS cleanup/SSR 快照恢复/整页重载 | V-2/V-5 |
| SSR 与 hydrate 初值/ID/DOM 结构不同 | 单块，升级为同实例失败 | 运行期根闸保全坏根；release 水合不能保证发现同为元素的结构错位 | 可检测的拒绝显示 failed；不可检测错位须由测试期阻断发布 | 根标记/seed与元素类型检查；V-2逐类SSR/CSR结构、节点身份与交互对照，含故意错标签反例 | 修改同源 view 与 seed，再构建成对资源；不宣称运行期可全面避免错绑 | V-2/V-5/V-14 |
| Rust 构建中再保存、缓存失效漏项 | 本次候选/受影响页 | 禁止新源码配旧 wasm；保留 last-good | building 持续到最新成功代；不会闪过过时代 | 输入摘要提交前复核、测试观测代/源码 | 丢弃过时结果并构建最新输入 | V-4/V-9 |
| 主题错误、字体缺失、闭态没覆盖打开样式 | 所有使用该主题/类的页面 | 新代不发布；旧代完整 | 作者见定位错误/已声明 fallback；读者不接收坏主题 | resolver、CSS 发布集合、实际打开控件 | 修文档/资源/完整候选；不要临时放宽 CSP | V-7/V-11 |
| 构建/发布被杀、磁盘满、无权限/锁冲突 | 单输出目录 | 用 P1 事务恢复；非自有目录不覆盖 | 明确拒绝/失败，原输出可恢复 | publish 的 journal/manifest/lock | 现有 recover；恢复同一完整版本 | V-14 |
| Pages 未配置、权限不足或部署后资源/404不正确 | 官方站公开 URL | 构建/上传失败保留旧部署；线上验收失败回退完整 last-good，无旧站则取消发布 | 上线未完成，不能宣称英文站已上线 | CI 结论 + 实际全部 URL/manifest/MIME | §8手动从last-good SHA重建并部署；首发失败按官方取消发布入口撤下 | V-15 |

发布顺序：SDK/注册风险门 → 自定义/模板/站点集成 → M6第一阶段固定版本、依赖/lock、作者README与模板、候选SHA和分发来源 → 同候选性能/设备/首次用户出口 → 同SHA安装 → 配置Pages Actions来源 → 构建/上传完整artifact → deploy → 真实全页验收。M6 前由维护者确认候选推送及向受试者提供的方式/授权；需要发布 git 来源才能参加 V-13，不能等 M7 才可安装。缺分发授权不阻塞本地开发，但阻塞相应 M6 用户出口。

workflow 草案及授权准备可在 M5 期间完成，实际 deploy 仍等 M6/V-15前置。只以 workflow_dispatch 手动触发，输入明确 candidate_sha（回滚时为last-good SHA），不配置 push main 自动部署；checkout/lock/安装身份全部指向该 SHA。build 用受支持 macOS；deploy needs build，只消费同一run的完整artifact，environment为github-pages、权限按官方文档配置；单部署串行，不在deploy runner编译CLI。发布动作须在当时有效授权内执行，不将计划授权等同于推送或启用Pages的授权。

Pages 默认公开 URL 由部署返回值与仓库设置确认，不把推导域名当事实。upload-pages-artifact 显式 retention-days=1，短期artifact不作为永久回滚库；回滚定义为在last-good源码SHA上重跑Mac构建/检查/上传/部署，同run新artifact的build_id/文件摘要须与记录相符，不能只重跑deploy或假设能读取其他run。M7保留candidate SHA、CLI/框架版本配对、build_id、完整文件摘要、run/deployment identity与URL，做一次完整重部署/回滚演练；无旧版本时从首个验收成功SHA演练重建同代。若失败修正涉及模板/README/依赖或产品源码，产生新候选，回到M6重验受影响V-*，产品引导改变必须重验V-13。

首次部署成功但线上验收失败、无last-good时，由获授权维护者[取消发布站点](https://docs.github.com/en/pages/getting-started-with-github-pages/unpublishing-a-github-pages-site)，保留失败产物/记录再修新候选；上线失败不能只留下坏站称“停止发布”。完整流程依据[自定义workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)与[deploy-pages说明](https://github.com/actions/deploy-pages/blob/main/README.md)，执行前复核当时规则。不创建gh-pages源分支、不移动旧计划或清理其他项目输出。

## 9. 验证

### 9.1 检查定义

新增检查以真实 CLI/公开 API/已生成 HTML 和浏览器输入为观察面。下表中的拟新增文件已在 §1.2 标明；具体测试名称由实施时按行为命名，不能用检查 ID 写源码。昂贵全量回归、同代浏览器矩阵/性能/UAT 在 M6 收口；M1–M5 先跑各自新增行为与风险门。

**浏览器供给合同**：保留 P1 project 的 content 来源，新增 project 只收对应站点 spec；以下新端口为本计划分配，启动前检查占用，拒绝复用未知服务。所有 Playwright 命令单 project、单进程串行，包含根配置的 Theme Studio。M1 登记 dom-hydrate；M2 完成其余配置/serve/static-host接线，M5再交付official.spec及站点。

| 来源 | project / 端口 / base | 服务与 spec 分配 |
| --- | --- | --- |
| 既有 DOM CSR consumer | dom-only / 4190 / / | 原入口；明确排除 hydration.spec.ts |
| M1 水合 harness | dom-hydrate / 4194 / / | features组用SDK直接构建host SSR、hydrate与独立CSR对照产物到target/dom/hydration；xtask serve服务；仅dom-only/hydration.spec.ts |
| P1 content fixture | docs-root 4191 / /；docs-subpath 4192、docs-meta 4193 / /rustify-ui/ | serve.sh原二参数写法兼容；仅既有静态docs-ssg spec，排除islands/custom/official与所有dev*.spec.ts |
| M2 components fixture | components-root 4195 / /；components-subpath 4196、components-meta 4197 / /rustify-ui/ | serve.sh新增三参数components root或subpath或meta port；仅islands.spec.ts/custom.spec.ts |
| M5 官方站 | official-root 4198 / /；official-subpath 4199、official-meta 4200 / /rustify-ui/ | serve.sh新增三参数official root或subpath或meta port；临时配置副本改base、原指南/README/Rust保持同源；仅official.spec.ts |
| 引擎辅助 | docs-webkit沿用4192；components-webkit复用4196；official-webkit复用4199 | 各收对应来源相同spec；一次只启动该来源的subpath服务。既有docs-firefox保留但P2不运行，不新增P2 Firefox project；Firefox完整验证留P4，引擎结果不冒作真实Safari |
| dev 内容/组件模板 | docs-dev / 每次port 0 | 只收dev*.spec.ts（新增dev-rust.spec.ts）；spec自启独立临时工程、稳定target，RUSTIFY_DOCS_BIN可指向安装CLI，无静态webServer |

serve.sh 的工作目录按 source/variant 分开；header项目用preview，meta用static-host，不混用产物。static-host的MIME补齐.wasm=application/wasm及woff/woff2/ttf/otf，与preview映射一致。每个project执行 --list 核对预期spec集合；不能不带--project跑全配置。官方站V-11命令使用official-*，fixture故障注入使用components-*。

冷构建由features/artifacts的预备阶段完成，独立于webServer的600 s与普通测例的60 s；dev-rust在稳定临时工程中预取依赖/构建后复用target，首次冷启动用独立用例观察。M2记录host+release wasm的冷耗时，按实测为冷用例设置并记录timeout；冷构建或用例超时属于未通过，不能靠缓存命中伪造冷数据。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | C-2 与跨期基线：P1 完成记录/来源匹配，内容模板命令、SSR/内容图行为成立；不是重验 P1 历史设备 | `git status`/相关 diff；P1/M9 和 gaps/M2 的相关部分；`mbx test --locked -p rustify-docs`、`mbx test --locked -p rustify-docs-cli` 分包；保存本期前基线 | M1 开始，修改前；基线不符先处理影响，不按旧计数冒称通过 |
| V-2 | C-3/WORK-F4/F5：CATALOG全部24类各一个真实代表，确定props初态host SSR；水合同步返回时完整原始树与SSR相同、保留原节点/ID/ARIA；允许的浏览器延后工作完成后，规范化完整outerHTML与同props/seed独立CSR对照一致，同一输入后公开状态一致；三同类岛独立、复杂几何/style属性、拒绝保全与清理；故意错标签反例由测试期对比发现 | SDK/components各自default/gpu/ssr单包test与hydrate wasm32 check；features组构建M1 harness；`npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts`。规范化只去runtime独占/instance属性、锁定tachys的空/!/<() />占位注释及属性顺序；只在DOM克隆上处理，原始树/节点身份单独全量比较，不删元素结构/文本/value/ID/ARIA/几何；CSR对照为独立csr feature产物 | M1宿主/编译及真实浏览器全部24类风险门（含九个wasm-only入口、DataTable/闭Select/Dialog/Tabs/NumberField/Tree）；不依赖M2 fixture；M6受影响回归 |
| V-3 | WORK-F4/AC-1/3/4、WORK-F15/AC-3：正常props/children；字段/类型/缺值/重复键定位；冷态同时报未注册组件与坏Rust；无脚本/事件/style注入；静态清单与describe完全一致，故意不一致拒绝整代；版本/bindgen预检定向诊断 | docs/CLI新增测试；全新组件fixture缓存执行 `check --format json`；对照注册/describe，注入错配；围栏与渲染占位比较，真实GitHub预览V-15覆盖 | M2宏/parser/两目标可用后；不等待browser全量 |
| V-4 | WORK-F5/AC-1/2、WORK-N6/N7：函数/全文件与编译源码精确一致；LF/CRLF/Unicode；源码与 wasm 同代；异常 helper/协议、编译失败时旧目录字节不变；不公开原始文件/路径 | `mbx test --locked -p rustify-docs-cli` 中真实冷/热 build/check fixture；函数原文件→最终 DOM/raw template/复制比较；异常子进程及连续构建/保存注入 | M2 即时风险门；M4 接入 dev 后重跑相关代一致性，M6 回归 |
| V-5 | WORK-F4/AC-2、WORK-F5/AC-3–6、WORK-N1/AC-4：三同类独立、V-2的SSR/CSR判据在真实site!管线成立；同图首帧/loading不改状态；资源/init/水合/超时/trap失败与恢复、源码/导航、浮层清理；slot内岛标记与复杂几何不丢失；异步能力反例按定案合同验 | `npx playwright test -c playwright.docs.config.ts --project=components-subpath islands.spec.ts`；delay/404/bad wasm/init/hydration/throw及禁JS，meta项目检查wasm MIME；按§6设计基准保存状态/布局证据 | M2执行已交付的嵌入/源码/状态/失败部分，不验M3才交付的slots/theme；M3补slot与静态主题后复跑，M6在components-root/subpath/meta全范围串行复跑，官方站集成另见V-11 |
| V-6 | WORK-F6/AC-2/3：定制页/七类slot、footer含404；route/title/描述/锚点/资源纳图；RenderContext::demo与围栏块相同；nav.routes可达/有序，非法输出/未知导航/重复路由拒绝 | docs/CLI公开测试；`--project=components-subpath custom.spec.ts`及components-meta；深层404、slot内岛与最终IDs/refs安全检查 | M3页图/slot/导航接入后；M6复跑并与官方站V-11集成 |
| V-7 | WORK-F8/AC-5/6：site/demo/Theme Studio同文档light/dark颜色/圆角/字体一致；system首帧；隐藏状态与新class生效；font fallback/missing/冲突、字节同版、无Makepad资源路径；纯内容站导入主题禁网build | 宿主resolve/字体摘要与生成测试；components-subpath的custom.spec.ts；先 `mbx xtask build-web --example theme-studio --release`，再根配置 `npx playwright test --project=theme-studio theme-dom.spec.ts` 用同文档预览（实施时补对应断言），端口4181；随后串行跑docs，不同时占browser/results | M3主题/scanner/字体就绪；V-5复跑证明主题上下文未破坏水合；M6性能和首次帧V-11 |
| V-8 | WORK-F1/AC-2、WORK-N4、WORK-N2：new components后锁文件就绪，`build --locked`前后lock字节不变；rev/bindgen错配定向拒绝；目录名规范化、缺工具链/非空目录无半成品；helper不含内容编译管线，host/wasm/CLI无Makepad；纯content零编译 | 扩展features/toolchain组；file://候选及受限PATH，取齐依赖后禁网check/build；完整目标图含build deps、产物/import；README试走与示例/链接检查。未改模板的交互由V-9负责 | M4完整模板；M7以GitHub固定SHA安装CLI再跑，同§2.1合同不只测开发二进制 |
| V-9 | WORK-F1/AC-2、WORK-F13/AC-4、WORK-N2/AC-4、WORK-F5/AC-1/2：候选CLI new components后不改源码，dev可调演示/源码页签逐字符同源；首次building地址/状态与Rust成功/失败/恢复、last-good/滚动；Markdown不调Cargo、注册/lock/path失效、连续保存无过时代；Rust错误为作者原路径/行列，第二次build的path依赖fresh | `npx playwright test -c playwright.docs.config.ts --project=docs-dev dev-rust.spec.ts`；port0稳定临时工程，Cargo JSON fresh计数/epoch事件/可见generation、§6dev设计截图；M7用RUSTIFY_DOCS_BIN指向安装二进制 | M4含未改模板浏览器门与五次编译观测；M6连贯UAT-3及计时，M7安装后必跑；不作为官方站M5的无关前置 |
| V-10 | WORK-F16/AC-1/2、WORK-N4/N7、WORK-N5/AC-3：12指南/README同源及修改反映；24类集合、侧栏36项可达/有序；排除文档链接转repo URL、validation写入不触发dev、无原始文件/Makepad；lang与正文一致；整站check零错误 | `rustify-docs check --config rustify-docs.toml --format json`/build；official.spec.ts与manifest/CATALOG比较；临时同源副本修改指南/README/排除文件；检查repo映射/请求/目标图、字体中性路径 | M5完整站；M6同冻结候选必复跑；M7实际URL复核 |
| V-11 | WORK-N3/AC-2/4、WORK-N5回归、WORK-F16/AC-3本地：两CSP host的全部页面/深链接/404/本期交互，禁JS/主题/320与375px/200%/键盘/读屏，console/CSP/失败请求；首页1280首屏主操作及§6finish基准、正文lang | `npx playwright test -c playwright.docs.config.ts --project=official-root`，official-subpath/official-meta分别串行；真实稳定Chrome/Safari及iPhone Safari/Android Chrome，Firefox依用户2026-10-13决定留P4，P2未验证；axe+人工键盘/VoiceOver，同候选preview/meta | M5局部页面设计验收；M6当期矩阵，缺设备阻塞出口、不套D-04；V-15补真实Pages |
| V-12 | WORK-N1/AC-4、WORK-N2/AC-2/4及P1性能：五次冷缓存FCP→全部岛响应≤1s/零位移，编译完成→新画面≤2s；content new→首页≤60s、0wasm和原预算；不以boot完成替代输入响应 | performance组/measure-load/measure-save与新增measure-islands，参考A/B按§9.2，记录profile/资源gzip体积/CPU下载解析/余量与epoch映射；CLI调用从本轮固定候选 | M2必做链接全部24类的release wasm代理门及编译后时延探测，超预算先解决资源策略才进M3/M4；M4复测完整dev/new；M6正式最大页/参考站测量，代理不代替正式出口 |
| V-13 | WORK-F1/AC-5、UAT-15：5位真实首次用户，至少4位15分钟独立完成新建/改页/演示，同源可交互 | 冻结CLI的help给第一条new命令，随后只按生成README；已安装候选可获取，负责人按§9.3招募计时（增量编译计入）；自动化不代替真人 | M6第一阶段冻结/分发后；M7模板/README/配对或产品改变则新候选重跑，非产品记录差异须写复用依据 |
| V-14 | C-2/C-4、WORK-N4/N6/N7与P1回归：fmt/分目标test/clippy/build、sources/css/catalog、事务/安全、旧DOM/GPU受影响行为；helper/wasm无私有管线 | §9.4命令/features/artifacts/reliability；CI verify.yml新增site-ssr/hydrate及宏/parser分目标门。SDK mount/overlay/runtime改动至少fusion-basic/component-catalog；theme/i18n另加theme-studio/property-workbench，DataTable/Tree改动另加data-workbench；按具体改动补相关spec，每项目先build-web release再单project串行 | M1共享改动收口必跑对应旧示例；M2–M5就近；M6集中复跑受影响项目；M7只复核受文档/版本影响门 |
| V-15 | WORK-F1/AC-2、WORK-F4/AC-4、WORK-F16/AC-1–3、WORK-N3/4：同冻结SHA真实git安装/配对、未改模板交互与锁不变/禁网；GitHub围栏预览；Pages全页/资源/404/CSP/字体/交互与deployment identity；部署说明写明meta范围、不提供frame-ancestors；回滚演练 | 固定SHA `cargo install --git/--rev --locked --profile docs-cli`，复跑V-8/V-9与来源/manifest；管理员/CI按§8上线，从返回URL全页走查和MIME；本地/线上build_id一致、last-good重建或首版重部署证据，文档链接/安装命令一致 | M7在M6出口/分发/Pages授权齐备后；未部署、未线上走查或无回滚演练证据不能完成 |

**P2 UAT 映射与连续路径**：

| PRD 场景 | 本期检查与出口 |
| --- | --- |
| UAT-3 作者连续维护 | M6冻结候选生成同一components工程：未改模板交互→新增demo及围栏→注册定制页→覆盖footer（含404）→改坏Rust；V-9串起V-3/V-4/V-6，证明dev保留同一last-good、check/build非零且旧输出不变，修复后恢复；保存完整步骤/源与产物摘要 |
| UAT-4 禁脚本/加载中 | V-5及官方站V-11，含零wasm内容页/普通导航/源码/初态/加载前输入闸；搜索部分留P4 |
| UAT-9 演示失败 | V-5/V-11的下载/init/水合/trap与重载；搜索失败留P4 |
| UAT-10 同次校验 | V-3/V-4/V-8，冷态同时报注册与Rust错误、位置/JSON/禁网；翻译警告留P3 |
| UAT-11 官方读者 | V-10/V-11/V-15，12指南、24分类/DOM源码、GPU外链/无Makepad、零错误/实际子路径上线；搜索/中英切换/llms留P4 |
| UAT-15 首次用户 | V-13；预检README和agent连续路径不代替五位真人 |

### 9.2 性能合同

- 参考 A 为官方站；参考 B 使用现有 `tests/docs-ssg/b-profile.json`/docs-corpus 的 500 页内容分布。B 本身无 Rust，以组件 fixture 的确定演示页加入相同内容分布形成其演示测量集合，并明确记录这项 P2 增量，不改 P1 无 Rust 内容站基线。每个站点每类页面挑首次加载最大的候选，清单记录 HTML/CSS/脚本及岛资源大小依据，不能只挑简单 Counter。
- WORK-N1/AC-4 使用 Apple Silicon macOS 参考机，无 CPU/网络节流、每页冷 HTTP/Service Worker 缓存五次取中位。用 performance paint 的 first-contentful-paint 为起点；终点为所有岛完成水合且各类演示的确定输入探针实际改变目标状态。单记 boot/资源完成不算响应，控制类以 value/selected/open 等公开结果证明；纯呈现类也必须有演示容器的真实切换操作供探针验证。
- 用 MutationObserver、DOM 节点身份/box 与 LayoutShift entries 测演示区域水合前后变化。无同类失败样本偷剔除；探针输入自身的合理布局变化与水合变化分开窗口。M2 的 0 位移与 M6 的正式五次测量都要保留原始页面时间数据。
- M2必做代理风险门：release wasm链接CATALOG全部24类，页面含最复杂初态及三同类岛；记录gzip体积、下载/编译/初始化/接管/输入响应分项与≤1s预算余量。超预算先在M2按数据优化/确定逐页资源组划分，并同步§3资源合同/V-4同代校验，才解锁M3/M4。另以docs-host/docs-wasm测两目标冷/增量编译与编译后渲染/加载代理时延；这不是尚未接入的dev端到端结果，完整epoch→新画面合同由M4/V-9首次验证，M6正式关闭。
- WORK-N2/AC-4 起点为两目标所需增量编译成功完成时刻，dev事件携带该时刻的epoch毫秒、generation及目标结果；参考机的服务与浏览器都在同一macOS主机，先核对SystemTime与浏览器timeOrigin的同一epoch映射且测量期间无时钟跳变。终点由页面performance.now记录新generation显示并证明新增结果，在页面内换算两端时间；不能以浏览器收到SSE为起点，否则漏算传输等待。不同主机或时钟不一致的样本拒绝，不直接相减未校准单调钟。五次取中位，保留原Cargo完成信息/事件与页面标记对应关系。
- 内容模板new到首页≤60s（WORK-N2/AC-2）、内容页0wasm/≤100KB gzip 和原 LCP≤2.5s/CLS≤0.1、B 内容保存≤1s/完整构建≤20s仍是回归；M4改new/doctor即测≤60s，M6复核。按P1已批准引擎/CDP条件运行既有harness，CSP/兼容最新稳定版和实体设备另登记。新增wasm/主题/font候选影响预算时，报告改动前后字节/耗时，不用“量不大”解释退步。
- 若所有岛 eager hydration 不能达成 ≤1s，先测 CPU/下载/解析与依赖体积分项；按数据减依赖或按实际页面拆分资源。不能为了延后计算而把岛移出“全部可交互”集合，也不把 FCP 起点改成资源已就绪。

### 9.3 首次用户与真实环境

测试负责人提供5位符合G1的Rust熟悉者，未用本框架；起始只有稳定Rust/CLI，wasm32目标按工具链提示准备。候选CLI help给new首条命令，之后仅按生成README新建、改页、注册函数演示并预览；安装/框架R/README摘要都属于M6冻结候选。首次依赖下载/冷编译单记并剔除，操作/阅读/修正与之后的增量编译等待均计入15分钟。记录匿名编号、环境/CLI C/库R、三个可见成果、有效时长、人工帮助与结果；受帮助者不能冒作独立完成，须替补或失败。自动化/README试走不代替招募。

V-11覆盖当期桌面稳定Chrome/Safari及两类实体手机，记录具体版本/设备。P1的D-04只限原期；用户2026-10-13明确“延期到P4；P2保留未验证记录”，Firefox稳定版与引擎均按此范围后移，P4完整矩阵保留。WebKit仅为引擎辅助证据。设备/最新浏览器缺失按§11准备，不以模拟填真人/真机结论。

### 9.4 静态与回归入口

保留 README/CLAUDE/.github verify 的分包纪律：

```sh
cargo fmt --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-docs
mbx test --locked -p rustify-docs-cli
mbx clippy --locked -p rustify-docs --all-targets -- -D warnings
mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings
mbx build --locked -p rustify-docs-cli --profile docs-cli
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask docs-verify --group features
mbx xtask docs-verify --group artifacts
mbx xtask docs-verify --group reliability
```

SDK/components各自执行default/gpu/ssr单包test/clippy与hydrate wasm32 check；docs单独检查pipeline/site-ssr/hydrate，宏/parser各自test/clippy。新增site/fixture/harness按独立manifest分目标构建/lint/fmt；verify.yml在M2登记这些CI门，M5接入官方站两目标。不能用workspace union代替隔离；旧DOM/GPU浏览器项目按V-14规则选取，先build-web release再单project串行，M1收口与M6各执行一次受影响回归。完整CLI图/输出、字体资产来源摘要和candidate清单均检查。

## 10. 里程碑与验收安排

7 个里程碑，M2 为技术风险阶段收口，M6 为 P2 本地/用户出口，M7 为可安装交付/实际上线。每项只对本检查覆盖范围记进度；M7 前不宣称 WORK-F16/AC-3 或整期上线。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 可接管的 DOM 静态生成与 SDK 水合 | P1基线；V-1修改前通过；§2.3确定初态合同 | 根/context/ID、全部24类初态与style几何改造；独立host/hydrate/CSR harness和dom-hydrate project；先定根/seed，组件按文件分工，共享mount/lib/清单/lock单写；browser串行 | V-1/V-2及V-14对应默认/GPU旧示例回归通过；复杂类/三岛/节点/ARIA/反例有证据，不依赖M2 fixture；回写「实施进度」 |
| M2 | 真实组件嵌入与同源演示（阶段一收口） | M1根/ID/Owner与V-2；Rtest/Ctest配对合同；解析可先用固定fixture | 过程宏/共享parser/冷态清单、site-ssr分组、host/wasm桥/describe资源、围栏/span/切片/全状态；scanner与SDK候选新增，先做§6岛块设计，接§9.1project/serve/MIME、verify.yml与代理性能harness；共享site/rust_site/Cargo/CSS生成单写 | V-3/V-4/V-5、V-12代理风险门与V-14通过；冷构建/profile/feature拆分/Rtest+后续CLI错配探测有记录，超预算先解再进M3/M4；finish review/规范同步；回写「实施进度」 |
| M3 | 可定制的静态页、插槽与主题 | M2注册/describe协议、check/build和V-3/V-4/V-12代理门 | 统一页图/七slot/RenderContext::demo/nav.routes；主题/静态context、CLI同版字体/许可/来源摘要及主题下候选/样式复核；docs/themes.md；generation/shell/content单写，theme/style分工避共同文件 | V-6/V-7、主题/slot影响的V-5复跑及V-14通过；字体同源/中性路径/隐态样式可观察，themes文档示例/链接检查；回写「实施进度」 |
| M4 | 可复制的组件工程与 Rust 随改随看 | M2真实编译/资源指纹+V-4、岛启动可用与V-12代理门；不等V-5故障全门或M3全出口 | 模板/doctor/包名/locked配对、README与docs/docs-ssg.md作者合同；先做§6dev设计，再building/last-good/原路径指纹/epoch事件；new/dev可分工，rust_site/main/report/lock单写 | V-8/V-9、V-12的content≤60s与完整dev首测、受影响V-4/V-14通过；未改模板浏览器/README试走/冷启动与增量观测有证据，finish review/规范同步；回写「实施进度」 |
| M5 | 英文官方站与24类真实演示 | M2 check/build及V-4、M3页图/主题与V-6/V-7；演示函数只需M1对应类V-2及site!合同；不依赖M4 V-8/V-9 | root白名单/README首页/CATALOG定制页/nav36项；先做§6页面设计；vellum唯一源译英/utility坏链接修正、B语料演示页与docs_corpus、verify.yml官方目标；函数按文件分工，配置/nav/注册单写；可同期准备workflow/分发/Pages授权 | V-10/局部V-11/受影响V-14通过；12+24可达/同源/lang/零错误/排除读取与监视，finish review/规范同步；回写「实施进度」 |
| M6 | P2完整行为、性能与首次用户出口 | M1–M5交付与V-2–V-10局部门；分发/推送授权、浏览器/设备与5位用户就绪 | 第一阶段冻结真实库R/CLI C、lock/模板/README/安装命令、提交与分发（非仅工作树）；之后同候选components/official root/subpath/meta、UAT-3连续作者路径、性能/设备/真人/回归；主实施者收口、资源串行 | V-5/V-9连续UAT/V-10/V-11/V-12/V-13/V-14通过；P2 AC及跨期回归有证据，Firefox按C-6延期单列未验证；冻结候选与后续发布无产品差异；回写「实施进度」 |
| M7 | git安装交付与英文站实际上线（最终收口） | 实际deploy须M6全部出口及V-11–V-14；冻结R/C可公开安装，Pages配置/发布与回滚授权明确；草案/申请可提前 | 同C安装/模板、Mac构建/同run artifact+手动deploy/真实全页/回滚演练；复核README/模板/docs-docs-ssg部署meta范围与固定命令，不静默改候选；文档/台账单写 | V-15及安装后二进制V-8/V-9通过；受变化影响V-10/V-11/V-14复核；若产品/模板/引导变化，新候选重跑含V-13的受影响出口；部署身份/build_id一致、回滚有证据；回写「实施进度」 |

提交点：M2技术风险收口；M6第一阶段在V-2–V-10局部门通过后为候选冻结提交R/C（仅记M6进行中，不能预填用户/完整矩阵通过），随后M6完整本地/用户收口；M7全部完成最终提交。各次先回写实际证据再提交/回填SHA；R/C分工及允许差异按§2.1，原完成记录归属不重绑。M3–M5未到冻结点可保留待提交行。

按正文和检查已能调度，不另造内部任务状态表。已有稳定契约且文件/资源独立时才并行，默认从单人推进；测试运行、Cargo/锁文件、CSS生成、同一浏览器与Pages部署不并发。后续修改接口或候选基线时，先集成并重跑受影响检查，再解除依赖。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 首次用户当前无法安排 | WORK-F1/AC-5不能由自动化替代 | 用户2026-10-13本轮答复“目前无法安排，届时保留为验收缺口”；保留§9.3的5位真人门，不以自动化代替或预填通过 | M6/V-13前 | 不阻塞技术实施；M6保留未验证缺口 |
| 候选分发/推送授权与可获取R/C尚未准备 | 受试者不能只用未提交工作树；V-13必须验冻结引导 | 维护者先交付具体R/C及文案/lock，确认可获取的分发方式及有效推送授权；M5可提前准备，M6第一阶段完成 | M6冻结/V-13前 | 不阻塞本地开发；未就绪阻塞M6用户出口 |
| Pages 尚未启用，发布/取消发布/回滚授权待准备 | 无真实URL不能关闭WORK-F16/AC-3 | 2026-10-13只读API确认仓库public、has_pages=false、Pages GET404；维护者按§8完成可审workflow/产物再在有效授权内配置/手动发布，V-15含全页/回滚 | M7/V-15前 | 不阻塞M1–M6；未就绪阻塞M7 |
| P2稳定Chrome/Safari待核验，两类实体手机当前无法安排 | P1 D-04不能当P2豁免 | 用户本轮确认设备渠道无法安排，实体手机保留缺口；本机继续核验可用桌面渠道并记录版本。Firefox按下方新决定延期，其他缺环境明确未验证 | M6/V-11前 | 不阻塞技术实施；真机及其他缺证保留未验证 |
| M1全部24类静态初态与node/ID接管已实测，site!管线待验 | 站点接线仍可能错绑/布局跳动/受控值不一致 | M1 V-2独立harness及对应回归已通过，证据见M1记录；M2 V-5继续证明site!管线/slot/异步反例 | M2退出 | M1前置已解除；M2管线门仍必做 |
| 全类release代理已通过，正式最大页待测 | 集成后资源/时延仍需复核 | M2五样本中位371.9ms/位移0，gzip证据见M2记录；M6最大页正式五次 | M6出口 | M2资源风险门已解除；不代替正式页 |
| 开发档代理修正后余量269ms | 后续集成仍须守≤2s；用户增量等待计时 | M3失败基线2369.4ms保留；helper准备与wasm编译重叠、两Cargo串行，最终五轮1731ms；M4验证完整dev | M4/M6出口 | M3代理与失败保护通过；完整dev与正式最大页仍待验 |
| 临时R/C带锁配对已测，完整模板/公开来源待验 | 生成工程身份与固定锁仍须证明 | M2临时Git安装、离线check/build及CLI/库/bindgen错配均通过；M4 V-8补模板与无身份，M7真实GitHub来源再次验证 | M4模板、M7交付前 | M2配对探测通过；不代替模板或公开安装 |
| 字体本地门已过，集成体积仍须复核 | 同主题不一致或突破既定预算 | M3七字体/许可/首选源码摘要、实际五字体响应、CLI增量及禁网Git安装均有记录；M6 V-12复核集成体积/性能 | M6性能出口 | M3已交付；不代替正式最大页与实体设备测量 |
| 白名单新增链接规则及指南现存错误未全量实测 | 官方站check不为零或读取/监视内部证据 | M5修utility坏链接/README指代、vellum译英；V-10对12指南所有错误及排除依赖/写入不重建验收 | M5/V-10 | 可按已定合同实施；未通过不收口 |
| 私有协议/源码指纹遗漏 | 冷态诊断漏报或新源码配旧结果 | V-3/V-4/V-9验证describe一致、原地前后摘要、连续保存/原路径诊断/依赖fresh；host/wasm串行，不依赖Cargo锁并行 | M2/M4 | 设计明确；缺验证不能解锁相关下游 |

用户 2026-10-13 明确决定：

| 决策 | 最终合同、代价与受影响出口 |
| --- | --- |
| 作者版本 | 库R→CLIC两次提交固定配对（§2.1）；代价是维护发布lock与配对元数据，换取生成工程可复制/错配可定位；V-3/V-8/V-13/V-15 |
| SSG数据 | 构建确定初态、异步在浏览器运行；无法生成初态则固定占位client-only。线上静态文件，构建工具的ssr feature不改变产品形态；V-2/V-5 |
| 字体 | CLI内置同版字体/许可，产物只打包引用字面及既有glyph回退（§5.5）；代价是CLI体积增加，保留导入即用与禁网；V-7/V-12 |
| vellum内容 | M5翻译唯一源为英文（§5.6），保留原事实/数值/链接、12页集合；V-10/V-11 |
| Firefox | 明确延期到P4，P2稳定版/引擎保留未验证；不记为通过，不豁免其他设备。PRD WORK-N3/AC-4与Q3需按本决定同步，原PRD未在本计划修改范围；V-11/M6按此新例外执行 |

- 最终状态：**Ready**。
- 定级理由：作者配对、SSG确定初态/浏览器异步、site!过程宏与共享parser、原地指纹/档位、字体、vellum与Firefox合同均有唯一答案。M1 harness解除M1/M2循环，所有风险探测及本地/用户/发布门有真实载体和前置。Ready表示可从V-1/M1开工；未测的技术余量、尚未招募/核设备/分发/部署均是对应出口的必做任务，不能记满足或完成。实际实施进度见顶部恢复快照。
- 用户 2026-10-12 已确认沿用 P2 的三个数值门槛，PRD Q5 的本期数值不再是待拍板项。Q4 沿用 GitHub Pages 的当前默认；若更换托管，仅修订其实际配置/策略与 V-15，不默默改变内容或 CSP 目标。

## 12. 已知坑与历史教训

- P1 PRD E1/E2/E3 是旧调查快照，不能照抄成 SDK 现状；本期以 §1 当前代码和 P1安装交付基线为准，避免重新做 GPU opt-in（PRD §1.1、[P1安装交付](../validation/docs-ssg-p1/M9.md)）。
- `id.rs::next` 是 thread_local 递增，不能跨构建线程/接管顺序保证一致；每岛scoped allocator与跨构建/三岛测试一起交付，CSR fallback维持旧行为。
- 修改class只看闭态HTML会漏掉浮层/运行态类；随包SDK候选和站点scanner进入同一生成门，不让作者额外跑工具（style.rs、CLAUDE.md「Conventions」）。
- P1安装验收曾因长期Cargo target将旧库视为fresh，版本正确而HTML缺页脚；最终安装使用真实临时target/来源身份/产物摘要，不能只检查 --version（[P1安装验收缓存偏差](../validation/docs-ssg-p1/M9.md#安装验收缓存偏差)）。
- 最终浏览器文档树才是真实ID语义，template/未渲染节点不贡献可到达锚点；新的组件/定制页/slot结果进入最终图后再验，不拿source token存在当通过（content.rs、generation.rs、[P1 R6](DOCS-SSG-P1.code-review.r6.md)）。
- `check`冷态须既报内容错误又报编译错误；预检不能依赖旧helper registry，静态声明和编译步骤都留位置/建议（WORK-F15/AC-3）。
- SDK show_fatal 会改容器内容；演示源码必须在scope外并由基础脚本管理恢复，不能在trap后调用死wasm清理（DOM loader、PRD D8）。
- root config把源码/指南置于同一边界，但导航不是发布白名单；include在读取/监视前生效，内部排除文件转repo URL是本期新增规则，不能冒作既有能力或偷偷发布（content.rs::snapshot/real_source、WORK-F16）。
- P1 D-04仅批准原期范围；P2其他真机/稳定版仍验，Firefox延期到P4依据用户2026-10-13新决定。性能固定引擎、WebKit与真机分别登记，不以一次模拟覆盖多种OS（C-6、PRD WORK-N3）。
- Host CSR/SSR并集构建无法证明隔离；新hydrate和官方站同样分目标验证，Mac构建后上传artifact不等于Linux CLI受支持（Cargo.toml、.github/workflows/verify.yml、P1决策B-03）。

