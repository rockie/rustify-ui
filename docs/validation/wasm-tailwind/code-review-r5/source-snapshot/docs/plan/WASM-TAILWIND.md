# WASM-TAILWIND · Rustify UI · Rust/WASM 类名合并、扫描与动态 utility 编译

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证并回写进度；使用 goal 执行时也遵循这些约定。
>
> **计划状态：已完成（7/7）**
>
> 调查基线：2026-10-04 · `fb288564caed23f79e34bd8ac28dae2f6f44bfeb` · dirty，仅本计划与评审报告未跟踪；代码事实依据当前工作树。
>
> 本期交付：**全 Rust 的 cn、内存内容扫描器及 Tailwind v4.1.13 CSS 编译核心，编译为 WASM；cn 接管组件拼接与 variant 类生成，完整移除 tw_merge/tw_merge_variants；运行时新增 utility 同时驱动浏览器 DOM 与 Makepad 的明确支持范围。**
> 本期独特职责：让构建时不存在的完整类名在运行时获得样式，并能验证合并、CSS 与 GPU 呈现三者各自的结果。
> **顶层排除：Tailwind v3、JavaScript 插件/配置执行、GPU 全量浏览器 CSS 语义、替换现有离线 Tailwind CLI、npm/crates.io 发布与网站部署。**

## 实施者定位

执行本计划的 agent 是**资深软件工程师**，依据本计划、仓库规则和当前代码实施。

- **表达与源码**：报告用路径、需求/检查/里程碑 ID 定位，不复述计划。源码不放需求编号、agent 标记或规划元数据，注释解释必要原因；追溯放记录或提交说明。
- **完成的定义**：本里程碑退出检查全部通过才能记完成；未运行、失败、环境缺失分别记缺口，不冒充通过。集中到阶段收口的终验由收口里程碑负责，通过前不宣称整期或跨期需求已验收。
- **工作顺序**：需要行为测试的改动先红、再绿、再重构。按实际前置成果及检查推进，不等无关终验，数据、安全、发布及用户分期门不得绕过；稳定契约下可用 fixture 开发，不能替代真实集成。按 §9 执行局部检查与集中验收，同环境走查可合并；相关代码、配置、依赖、输入与环境未变可引用已通过证据，否则重跑受影响检查，收口仍跑必要集成回归。
- **并行交付**：独立任务在收益超过委派/集成成本且工具可用时使用 subagents，模型默认继承。任务说明只给目标/ID、稳定契约、可写与禁改路径、运行资源和检查；共享文件单人负责，数据库、端口、浏览器等隔离或串行。子 agent 返回结果、路径、验证证据与缺口，默认不另写任务文档；主 agent 审查并集成验收。边界变化先回报，失败只阻塞依赖任务；结束对话前收集或停止仍在运行的任务。

## 实施进度（实施期持续更新）

本节是跨对话恢复的**唯一汇总入口**，协议文字不随实施改写，实施期只更新「恢复快照」和「完成记录」。续做时先读仓库规则、本节、当前任务相关正文与依赖，再核对 `git status` 和相关 diff；记录与工作树不符时先查明原因，不凭记录覆盖用户改动，也不凭代码存在推断验收已通过。只有继续未完任务、核验前置验收、排错或审查时，才按链接读取对应记录的相关部分；不要默认加载全部记录或原始日志。

**回写时机**：里程碑完成后立即回写，再报告完成或提交；下游只需部分成果时，所需前置检查通过并在记录保存证据即可推进。实质进展后受阻、暂停、交接、发现偏差或结束对话时也保存成果与缺口；普通子任务切换不单独回写。

**一次回写**：主 agent 更新对应记录，再同步快照变化字段和完成记录一行，不重写无变化内容或追加流水。快照写当前任务、进展与下一步，命令及证据只进记录。首次回写删占位行，每个已记进展的里程碑恰好一行，状态仅「进行中 / 阻塞 / 已完成」；完成时移至已完成行末，最近完成取该行。更新时间带时区，摘要 1–2 句，记录填相对计划的链接（如 [M1 记录](../validation/wasm-tailwind/M1.md)）。范围、决策、接口、数据、风险或退出条件有偏差时，就地修订对应正文，不追加勘误历史。

**独立记录**：按需建 `<计划名>.records/M<n>.md`，头部写计划链接、更新时区、状态及验证基线（commit SHA；未提交写 dirty@起始SHA 与关键改动路径）。正文只写交付行为/路径、必要偏差、未完项与继续动作；验收按 `检查 ID/覆盖范围 | 命令或步骤 | 结果/必要证据` 记录，共同环境和基线在头部写一次，有差异再单列。同一测试组可覆盖多条退出条件，须逐条可追溯；引用原证据与复用依据，不复制断言全文、成功日志或调试流水。未运行、失败、环境缺失明确区分；只有复杂交接、排错或证据需独立保存时才另建任务记录/日志/截图并链接。

**回写后自查**：每次回写后直接核对以下条目：「当前进度 n/N」的 N 等于里程碑总数，n 只计状态为「已完成」的行；「最近完成」对应最后一条已完成记录，不按编号推算；链接指向已落盘文件。上述结构、计数与链接检查不证明验收真实通过。

### 恢复快照

- 最近更新：2026-10-09T14:35:00+08:00
- 当前进度：7/7 个里程碑完成
- 当前状态：全部完成。M7在同一release上通过V-1至V-11：utility-lab root53/deep47、scanner 5+5，上游执行门1,183/0失败，workspace与特性测试、clippy、来源及旧包检查；V-11原生/WASM/浏览器/包体已与JS参考及M1基线对照记录（无数值SLO）。代码评审R1的3个P2已修复：scope关键帧随动画变量改写、新增`rebuild(active)`、CI安装oracle；CI仅本地等效验证；R2复审确认三项已解决并发现回归F-04；R3判F-04部分解决并新增F-05–F-07（变量代换、数学时长、数值iteration），已改为按var()代换后的层状态判定关键帧名，R4判F-05–F-07已解决、F-04部分解决并新增F-08（副本不跟随公开token覆盖）；经用户裁定收窄契约：公开token的关键帧名仍改为本owner名，用途冲突时报UnsupportedFeature，副本只留给`--tw-*`，修后root58/deep52通过（scanner浏览器未重跑），见[M6](../validation/wasm-tailwind/M6.md#评审修复代码评审-r4)/[M7](../validation/wasm-tailwind/M7.md#评审修复代码评审-r1)
- 最近完成：M7 · 全范围终验与开发者交付
- 下一步：`/review-code docs/plan/WASM-TAILWIND.md` 复审R4的F-04（契约收窄）/F-08修复。可选后续：远端运行新增CI任务；cn缓存命中路径的性能（需保持回调调用次数语义）；浏览器Worker内存（需跨源隔离）
- 当前阻塞：无
- 代码基线：`53652fc4`（dirty，未提交）；此前`ef766ce`、`fb28856`的历史证据保留

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 |
| --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-04T17:05:22+11:00 | 上游与迁移语料、三组 Worker/CSP/extractor 风险门和可重放基线通过 | [M1 记录](../validation/wasm-tailwind/M1.md) |
| M3 | 已完成 | 2026-10-04T19:02:18+11:00 | SDK ensure/Observer与prepare-install-ack闭环、utility-lab root9/subpath4浏览器通过；M4全兼容与GPU不抵扣 | [M3 记录](../validation/wasm-tailwind/M3.md) |
| M2 | 已完成 | 2026-10-04T21:57:12+11:00 | cn生产host56组、947+56 runtime及getter轨迹26组通过；静态迁移、Scanner及旧包删除局部验收通过，动态epoch与浏览器crossing归M6 | [M2 记录](../validation/wasm-tailwind/M2.md) |
| M4 | 已完成 | 2026-10-07T00:18:17+11:00 | 上游1,398测试经Rust执行逐条入账：1,183通过/0失败，其余证据化排除或独立核对；compiler-full门与批次性质测试通过 | [M4 记录](../validation/wasm-tailwind/M4.md) |
| M5 | 已完成 | 2026-10-07T11:05:00+11:00 | GPU文字换行/行盒、shadow/ring层序与列几何对DOM实测一致；§5.5矩阵逐行有证据，root49/deep44与旧示例None路径通过 | [M5 记录](../validation/wasm-tailwind/M5.md) |
| M6 | 已完成 | 2026-10-09T14:35:00+08:00 | 浏览器cn crossing 947通过、demo cn输入与中/英语言、V-9边界/循环/深度/远程import；评审R1–R4修复（F-04按用户裁定收窄契约、F-08冲突公开token改为拒绝）后root58/deep52通过 | [M6 记录](../validation/wasm-tailwind/M6.md) |
| M7 | 已完成 | 2026-10-08T20:30:00+08:00 | 同一release终验V-1至V-11通过，R1–R3修复后浏览器门已在新产物重跑；CI补oracle安装（远端未跑）；性能按无SLO记录并列明缺口 | [M7 记录](../validation/wasm-tailwind/M7.md) |



本仓 `CLAUDE.md` 要求里程碑证据归入 `docs/validation/`；记录目录采用 **docs/validation/wasm-tailwind/**，覆盖上文默认的计划旁目录约定。首次产生实施事实时再建记录，例如 `[M1 记录](../validation/wasm-tailwind/M1.md)`，不预建空记录。

## 0. 需求、范围与决策

### 0.1 需求与约束账本

计划类型：SDK 横向增量、编译器移植及双渲染器集成。用户于 2026-10-04 确认：cn、扫描器、编译核心全部使用 Rust/WASM；DOM 与 Makepad 两者都支持；固定 Tailwind v4.1.13、GPU 分矩阵覆盖、JS 插件后续处理。同日补充原则：先与上游保持一致，再做加法创新，不先缩减上游能力；不保留 tw_merge 旧包，将组件需要的能力迁入新 cn，避免双引擎维护债务。动态 CSS 主路径直接接收运行时完整类名，补齐构建时缺失的 utility 规则；源码扫描是独立输入能力，不是该主路径的前置。

| ID | 类型/来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- |
| R-1 | 用户指定 cn 上游 | 对齐固定 cn 的运行时/配置、表编译与条件拼接/冲突合并；Rust 调用及 WASM 导出 | §2、§5.1；V-2 | 已确认 |
| R-2 | 用户要求扫描器 | 对齐固定 Oxide 的抽取/预处理、源选择及增量行为；WASM 使用内存与虚拟源目录，native 可用实际文件源；独立于完整类名动态入口 | §2、§5.2；V-3 | 已确认 |
| R-3 | 用户确认全 Rust/WASM | 移植 v4.1.13 内置 utility、variant 与 CSS 配置，不在生产中运行 TS 编译器 | §4、§5.3；V-4、V-5、V-10 | 已确认 |
| R-4 | 用户要求动态增加 | 显式注册运行时完整类名，并观测 scope 内 DOM class 变化；生成构建时缺失的 utility 规则，无需重新构建或调用 extractor | §2、§3、§5.3–5.4；V-4、V-6 | 已确认 |
| R-5 | 用户确认两者支持 | 相同类列表投射到 DOM 和受支持的 GPU 控件；GPU 不支持项可见 | §5.5 支持矩阵；V-7 | 已确认，矩阵为本计划设计 |
| R-6 | 已有主题上下文 | 延续 ThemeScope/ThemeBoundary、暗色及旧主题行为 | §4、§5.4–5.5；V-8 | 由现有契约继承 |
| R-7 | 用户补充原则 | 先建立完整上游兼容基线，再增加动态能力；里程碑切片不得成为最终缩减范围 | §0.6、§4、§9.2；V-2/V-3/V-5/V-10 | 已确认 |
| R-8 | 用户明确移除旧包 | cn 成为静态与动态组件唯一合并引擎；迁入必要拼接、variant 类生成与 trait，移除 tw_merge/tw_merge_variants 依赖、旧导出和回退路径 | §2、§3、§5.1、§8；V-2/V-8/V-10 | 已确认 |
| C-1 | Cargo.toml、mise.toml | 沿用 Rust、Leptos、wasm-bindgen 版本；编译测试走 mbx；单线程 WASM、静态托管 | §1.2、§4；V-1、V-10 | 已核实 |
| C-2 | xtask/src/serve.rs、CLAUDE.md | 严格 CSP，不增加 JS unsafe-eval、style unsafe-inline、共享内存或服务依赖 | §5.4、§7；V-1、V-6、V-9 | 已核实 |
| C-3 | AGENTS.md、CLAUDE.md | 浏览器终验、中文计划、来源与许可证可追溯；不顺手改 Makepad 或依赖 major | §1.2、§9、§12；V-1、V-10 | 已确认 |
| NFR-1 | 动态输入新增风险 | 输入、候选、AST、缓存及队列有界；无重复编译与无限累积 | §3、§7；V-6、V-9 | 机制确定，配额由调用方显式提供 |
| NFR-2 | 无用户数值 SLO | 记录冷/热 cn、扫描吞吐、编译 p50/p95、主线程应用、GPU 帧与包体前后值 | §7、§9；V-11 | 数值目标未知，不承诺上游速度倍数 |
| A-1 | 抽取上游纯逻辑 | Oxide extractor 的最小依赖闭包可独立编译为非线程 WASM；仅源码扫描需要它 | §4；M1/V-1.extractor 实编译与 fixture，由实施负责人验证 | 已通过最小非线程 WASM 与 24 共享语料；完整源选择验收归 M2 |
| A-2 | 浏览器集成新增 | 验证生成后的 CSS 在当前 Chrome 严格 CSP 下的安装、@scope、嵌套规则与 @property；动态编译路径已有上游参考 | §5.4；M1/V-1.css 实浏览器验证，由实施负责人验证 | 已通过 strict CSP、staging/adopted、scope/property/nesting 与 computed style；完整 runtime 归 M3/M6 |

### 0.2 决策表

| 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- |
| 生产实现 | Rust 计算核心 + wasm-bindgen；JS 只做初始化、Worker 消息与 Web API 胶水 | cn、抽取、解析、编译、排序、GPU 求值都在 Rust；Node 仅作开发期参考执行 | R-1–R-3 |
| 兼容基线 | Tailwind v4.1.13；cn 固定 commit，见 §1.3 | 固定预期；升级另跑差分，不跟随 main 自动变更 | 用户确认、mise.toml |
| crate 划分 | 一个无 DOM 依赖的核心 crate、一个薄 WASM facade，运行时接入现有 SDK | 不为 cn/扫描/编译各造独立发布包；以 feature 控制重型编译代码 | §2 的真实调用边界 |
| 旧合并包替换 | rustify-style 的 cn 模块接管必要拼接、variant 类生成与 trait；组件统一使用 cn | 删除 tw_merge/tw_merge_variants 及旧包导出，不保留双引擎或旧包 fallback；组件生成接口保留，直接引用旧包的调用方迁到 cn 入口 | R-8、macros/clx.rs、macros/variants.rs、lib.rs |
| 并发 | cn 留在应用 WASM；扫描/CSS 编译在独立 module Worker 的单线程 WASM 中 | 不在 render 中跨 Worker 调 cn；编译不阻塞 GPU 主线程；增加 Worker 产物和消息复制成本 | C-1、NFR-2 |
| 增量 | 缓存每个候选 IR，新增候选后对当前集合重新排序/序列化，整表原子替换 | 避免到达顺序改变 cascade；“增量”不等于每次仅输出新增 CSS | 上游 compileCandidates/compileAst，R-4 |
| 动态入口 | ensure_classes 接收完整类名 + scope 内 MutationObserver 读取 classList 补充 | 直接进入 compiler，无需 extractor；scan_content 独立启用，Observer 不扫描整个 document | R-2、R-4 |
| GPU | 使用 CSS 编译候选 IR 的受支持投影，复用主题解析及受控 props | 不从 CSS 字符串反解析；GPU 不支持项返回诊断 | R-5、PRODUCT.md |
| 既有构建 | 保留 CLI 与静态 stylesheet，默认组件改用 cn；动态 CSS 能力 opt-in | 关闭动态功能仍使用 cn，且不增加 scanner/compiler Worker 体积；不恢复旧包 | R-8、README.md、xtask/src/css.rs |

### 0.3 ADR-lite

#### ADR-1：移植 CSS 编译核心与抽取器，而非嵌入脚本引擎

- 状态：Accepted（用户明确选择全 Rust/WASM）。
- 驱动：R-3；稳定兼容语义比表面 WASM 包装更重要。
- 备选 A：Rust cn/抽取器 + 官方 TS 编译核心，兼容成本最低，但不满足全 Rust。
- 备选 B：把 TS 产物放入 WASM 中的 JS 引擎，保留算法但引入解释器、脚本运行与更大的故障面，也不满足计算核心为 Rust。
- 选择 C：移植 Oxide 的纯抽取闭包；按行为移植 TS 编译器。声明式静态表可预生成，动态解析与规则产生必须是 Rust。
- 正面：native/WASM 共用行为测试，生产不需要 Node、JS 编译器或文件系统。
- 代价：这是完整编译器工程；需持续维护语法、utility、variant、主题、排序与错误语义。不能把它估作“把现有 Rust crate 改个 target”。
- 重评触发：固定版本内发现无法维持的语义差异，或以后要求执行第三方 JS 插件；回报成本与方案，不静默换成 TS。

#### ADR-2：隔离编译 Worker，明确 GPU 支持契约

- 状态：Accepted；M1 的 V-1.worker/css 已通过可行性验证，证据见 M1 记录；不依赖 extractor 风险门。
- 候选 A：全部嵌入应用 WASM，以主线程任务切片推进；资源和通信简单，但单次深层解析/排序仍可能阻塞绘制，trap 影响应用全部 scope。
- 候选 B（选择）：重型核心在 Worker；主线程 cn、CSSOM 提交与 GPU 映射留在应用；编译 trap 可单独恢复，代价是额外 WASM、协议和复制。
- GPU 候选 A：由隐藏 DOM 的 getComputedStyle 驱动 Makepad；可以借浏览器求值，但不适配原生目标、引入测量节点与布局读取，难处理状态及层级。
- GPU 候选 B（选择）：编译候选产生 CSS AST，同时保留受支持的声明/条件 IR；Rust 按 §5.5 求值。少数能力先明确交付，不承诺浏览器等价引擎。
- 重评触发：Worker 启动/复制成本实际高于收益，或 GPU 产品范围扩大到 DOM selector、grid、任意 CSS 函数；新增决策，不能只扩大枚举假装已经支持。

### 0.4 职责与排除

应用拥有输入 CSS、虚拟资源、类列表、主题、控件状态和配额。核心拥有 cn 规则、条件拼接、纯 Rust variant 类生成与 trait、抽取、候选解析、CSS 编译与排序；本计划的新 cn 包指 rustify-style 的 cn 模块。组件 crate 保留 Leptos 视图宏与 scope 上下文接入，不把 DOM 依赖迁入 cn。SDK runtime 拥有消息顺序、scope 注册、stylesheet 生命周期及失败恢复。GPU 投影拥有受支持声明求值与诊断，RegionApp 仍拥有控件树与业务状态。

不实现浏览器 CSS layout engine，不自动给任意 Makepad Widget 增加 class 语义。首个 GPU 接入覆盖 RustifyButton 与 demo 的容器/文本/色块，其他控件继续使用现有主题并明确记录未接入。不替换 GPU 业务事件、DOM 原生输入、已有主题编辑站整体布局。不新增数据库、服务、云端存储。Tailwind JS config/plugin 仍按用户已确认的分期另立阶段。cn 的自定义配置、validator、配置函数及表编译不因跨 WASM 就删掉：Rust 入口用等价 callback，WASM 入口支持调用方函数句柄，合并算法仍为 Rust。默认表、定制表与项目裁剪表对齐固定上游行为；动态模式默认使用完整表，主动选择项目裁剪的调用方可显式注册新增类并扩充/重建表。

### 0.6 上游基线与加法的顺序

建立两本清单：**上游兼容账本**完整登记固定版本的公开 API、配置、规则、源选择、边界与错误；**新增能力账本**登记 WASM 导出、动态注册、Worker、scope 生命周期与 GPU 投影。新增入口不修改上游接口输出语义。

- 上游能力不因“现有组件用不到”或“WASM 移植难”移到范围外；未完成项保持待实现，并阻止完整基线验收。M1 必须读取并锁定尚未逐项调查的 API/测试，不以本文示例代替上游完整清单。
- M3 是实现过程中的端到端切片，不是削减版产品或对外兼容声明。M4 以全部确认范围为门，M7 才交付整期，不降低兼容比例换取完成。
- JS 插件/配置和 GPU 全量浏览器语义仅保留用户已同意的分期边界。GPU 新后端报告能力差异，不能据此删除 DOM 的 group/peer、grid 或任意值。
- 输入配额是运行保障，不得用它隐藏合法语法尚未实现。

## 1. 当前事实与改动面

### 1.1 已核实的现状

- [已核实·足够] `README.md`、`Cargo.toml`：Leptos DOM 与 Makepad 在应用 WASM 中共存；六个示例，静态托管，不依赖后端。
- [已核实·足够] `xtask/src/tailwind.rs::compile`：只执行固定版本 CLI；`xtask/src/css.rs::run` 校验提交的 SDK stylesheet；`xtask/src/build.rs` 根据示例输入编译应用 CSS。
- [已核实·缺口] `crates/rustify-components/src/macros/mod.rs::merge` 调用 tw_merge；`clx!` 用其 join，`variants!` 用 TwClass/TwVariant derive，`lib.rs` 公开导出旧包 → 将必要拼接、纯 variant 类生成及 trait 迁入新 cn，迁移全部调用后删除依赖与旧导出；不能只替换 merge 函数。
- [已核实·足够] `crates/rustify-components/css/sdk.css`：scope 暗色 variant、@theme inline、reference spacing；`theme-v4.css` 为可选 v4 主题投影。现有 `--spacing` 是 layout gap，不应被 utility 当作单位。
- [已核实·足够] `crates/rustify-ui/src/theme/provider.rs`：scope/boundary 写运行时属性；`resolve.rs` 提供颜色、尺寸、字体和阴影快照；GPU `theme.rs`/`button.rs` 已消费数值快照。
- [已核实·缺口] `GpuRegion` 的 class 只落到 canvas；`RegionApp::apply_props` 接受应用定义的 props → utility 需要单独投影到控件，不会由 canvas CSS 自动影响内部绘制。
- [已核实·缺口] 当前构建和主题 provider 没有动态 CSS compiler、候选注册或 Worker → 新增 runtime 链路，不能以现有主题变量更新代替。
- [已核实·足够] `xtask/src/serve.rs::CspMode::header_value`：style-src self、worker-src self；`web/loader.js` 与 `listeners.rs` 提供实例 abort 清理模式。
- [已核实·缺口] Makepad 的 build 使用锁定版本的 in-process wasm-bindgen；其 glue patch 要求 env imports → 纯 Worker WASM 不能无条件套用该 Makepad patch，需要 xtask 独立生成 web-target glue。

### 1.2 文件改动清单

★ 表示拟新增，当前不存在；表中现有路径均已读到相关行为。

| 路径 | 处理与边界 |
| --- | --- |
| ★ `crates/rustify-style/` | 纯 Rust cn、必要 join/variant 类生成与 trait、scanner、candidate/CSS AST、theme、utilities、variants、排序与有界 session；cn::class_variants 与 CSS variants 分开；无 Leptos/web-sys/Makepad |
| ★ `crates/rustify-style-wasm/` | cdylib + rlib facade；WASM cn 导出、Worker compiler 请求响应；features 隔离重型核心 |
| ★ `crates/rustify-ui/src/utility/` | feature-gated runtime、scope 观测、CSSOM 安装及 GPU 声明求值；rustify-ui 导出 |
| `crates/rustify-ui/src/gpu/button.rs`、`crates/rustify-ui/src/gpu/theme.rs` | 明确 utility patch 接口、状态与默认值恢复；不把类字符串塞进现有 ResolvedTheme |
| `crates/rustify-components/src/macros/mod.rs`、`crates/rustify-components/src/macros/clx.rs`、`crates/rustify-components/src/macros/variants.rs`、`lib.rs`、`Cargo.toml` | 全部拼接/合并改用 cn；纯 variant 类生成迁入 cn，Leptos 宏保留；删除旧 derive 与公开 tw_merge，导出 cn 入口；删除 tw_merge 依赖 |
| `crates/rustify-components/src/` 中使用 merge/variant class 的组件 | 默认用 cn，UtilityScope 下用上下文 cn；从原始片段一次合并，已挂载组件跟踪已提交配置 epoch；只改必要接入点，不改变控件业务 |
| ★ `web/utility-worker.js` | 同源 module Worker；初始化/传输/错误边界，零 JS utility 编译逻辑 |
| `Cargo.toml`、`Cargo.lock`、`xtask/Cargo.toml`、`xtask/src/build.rs`、`xtask/src/main.rs` | 新 workspace member、features、锁定 wasm-bindgen-cli-support；新增 worker build/copy/hash 校验入口与静态候选 seed/bundle manifest |
| `sources.lock.json`、`xtask/src/sources.rs` | 增加 cn/Tailwind 来源和生成表 provenance，同步原组件宏的迁移/改写归属；如复用旧包片段，保留实际许可证与来源摘要；不将旧包整份 vendoring 或改名继续依赖 |
| ★ `crates/rustify-style/upstream/` | extractor 依赖闭包、必要参考数据与 MIT 通知；不搬整个 Tailwind monorepo |
| ★ `tests/utility/` | 固定输入/预期、参考执行工具、上游兼容与新增能力分类账；测试用 TS/Node 与生产隔离 |
| ★ `examples/utility-lab/` | cn 输入、源码扫描、DOM/GPU 动态类对照示例；独立于 Theme Studio 编辑业务 |
| ★ `tests/browser/utility-*.spec.ts`、`playwright.config.ts` | 新 root/subpath project，按已有 server-on-demand 模式接入；端口集中在配置中选空闲值 |
| ★ `docs/utility.md`、`docs/validation/wasm-tailwind/` | API/支持矩阵/接入文档与按需创建的里程碑证据 |
| `README.md`、`docs/architecture.md`、`.github/workflows/verify.yml` | 最小增量说明与检查任务；不移除现有 CLI/静态 CSS 流程 |

GPU 适配优先改 SDK 自有控件；除非实验证明现有 Makepad 数据结构不足，不修改 `makepad/`。不是 Portal App，无安装、授权或 onebox 链路。

### 1.3 上游事实与固定证据

| 上游 | 固定基线 | 已读行为/依赖 |
| --- | --- | --- |
| cn | `b7ec0fce43d824a60517fab508584eb860fdf717`；该树 packages/cn/package.json 为 0.4.0 | [index.ts](https://github.com/shadcn-ui/cn/blob/b7ec0fce43d824a60517fab508584eb860fdf717/packages/cn/src/index.ts)、[engine.ts](https://github.com/shadcn-ui/cn/blob/b7ec0fce43d824a60517fab508584eb860fdf717/packages/cn/src/engine.ts)：拼接层、表驱动分类、variant context 与有向冲突；[config.ts](https://github.com/shadcn-ui/cn/blob/b7ec0fce43d824a60517fab508584eb860fdf717/packages/cn/src/config.ts) 编译声明式扩展 |
| Tailwind | tag `v4.1.13` 对应 `1334c99db8fd26c8ea065375dd9259800863f072` | [Oxide extractor](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/crates/oxide/src/extractor/mod.rs) 抽取候选及变量；[scanner](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/crates/oxide/src/scanner/mod.rs) 有 filesystem、ignore、Rayon 等依赖，不整体带入 WASM |
| Tailwind 编译 | 同上 | [index.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/index.ts)：CSS 编译 session、候选累积；[compile.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/compile.ts)：variant、property-order、声明数量与类名共同排序 |
| Tailwind 规则 | 同上 | [utilities.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/utilities.ts)、[variants.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/variants.ts)、[theme.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/theme.ts)：注册式规则与 inline/reference/default/static 主题语义 |
| 官方 browser 包 | 同上 | [browser/src/index.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/@tailwindcss-browser/src/index.ts)：读取 classList、累积 DOM classes，直接调用 compiler.build 并更新 style，不经过 Oxide extractor；不支持 plugins/config files。是行为参考，不是可直接使用的 Rust compiler，也不是本仓严格 CSP 安装已通过的证据 |

两库根 LICENSE 均为 MIT；移植文件保留各自版权与许可。以上是源码行为调查，未证明本项目已具备这些能力。上游测试数量或性能宣传不作为本项目验收数字。

## 2. 模块、接口与依赖

| 模块/调用者 | 最小接口与不变量 | 集中的复杂度与依赖 | 测试面 |
| --- | --- | --- | --- |
| core::cn / Rust 组件、WASM facade | `Cn::new(config)`、`merge(inputs)`、`join(inputs)`；公开运行时/config/engine/build 核心语义经 Rust 入口或 WASM facade 对齐；merge 无编译副作用 | 完整默认表、拼接、有向冲突、配置 callback、表编译与有界缓存；进程内 | 固定 cn 运行时、配置与表编译逐例差分 |
| core::cn::class_variants / 组件宏与 Rust 调用方 | 提供纯 Rust variant 枚举/类结构的生成宏及必要 trait；保留组件生成类型名称、首项默认、base→variant→size→caller 顺序和常用 to_class/with_class；原始片段入口供组件使用上下文 cn | 用 macro_rules 生成本仓所需实现，不引入旧 TwClass/TwVariant derive 包；普通调用只执行一次默认 cn，动态组件从 raw 片段只执行一次上下文 cn | variant/size、默认项、caller override、宿主编译和自定义 cn 配置 fixture |
| core::scanner / Worker、host 工具 | `scan(content, syntax) -> ScanResult`、`scan_sources(entries, source_rules)`；native 文件源与 WASM 虚拟源目录使用同一选择规则 | Oxide extractor/preprocessing、glob/ignore/源选择与增量；源读取与纯抽取分开 | 上游 extractor/preprocessor/Scanner fixture + Rust/HTML/JS/JSON |
| core::compiler / Worker | `Compiler::new(input, assets, limits)`、`prepare_add(batch)`、`commit(prepared)`、`rebuild(input, assets, active)`；batch 接收完整类名，直接解析候选，不依赖 scanner；失败不改变 committed 状态 | CSS parser、theme、规则注册、排序、AST、变量/keyframes依赖、session/cache；进程内 | 官方 compile/compileAst 结果与行为 oracle；关闭 scanner 的动态候选用例 |
| utility runtime / 应用、scope | `UtilityScope`、`ensure_classes`、`scan_content`、`flush`、`replace_config`、`dispose`；异步入口返回完成凭证和结构化错误；每 scope 一个配置与有界候选集合 | 观测、Worker FIFO、两阶段安装、生命周期；浏览器本地 Web APIs | strict CSP、乱序、清理、动态 CSS 对照 |
| GPU resolver / RegionApp | `resolve_gpu(program, theme, context) -> GpuResolution`；context 显式含状态、尺寸与 rem；不访问隐藏 DOM | 有限 cascade/表达式求值、具体控件 capability；纯 Rust | 数值差分、unsupported 诊断及真实绘制 |

核心 feature 分为 `cn`、`scanner`、`compiler`，native 文件源另用 target/feature 隔离：静态与动态组件均依赖 cn feature，所需纯 variant 类生成及 trait 位于 cn 中，不另建 derive 发布包。动态 Worker 开 compiler 即可；源码扫描另外启用 scanner，compiler 不隐式开启或调用 scanner。未启用 scanner 的 runtime 调用 scan_content 返回明确的 UnsupportedFeature，其余完整类名入口正常工作。完整 demo/终验启用两者，R-2 不缩减。GPU 程序为可序列化声明和条件，不要求主线程带全套编译器。默认表与 session cache 不跨配置共享；同一页面的独立 WASM 不以全局可变单例共享状态。

接口类型草案（★，实现时保留不变量，允许调整命名）：

```rust
pub enum ClassValue<'a> {
    Text(&'a str), Number(f64), Bool(bool), Null,
    List(Vec<ClassValue<'a>>), Dict(Vec<(&'a str, ClassValue<'a>)>),
}
pub struct ScanResult { pub candidates: Vec<String>, pub variables: Vec<String> }
pub struct CompilerInput { pub css: String, pub entry: String }
pub struct SourceAsset { pub id: String, pub content: String }
pub struct Limits {
    pub input_bytes: usize, pub asset_bytes: usize, pub candidate_count: usize,
    pub ast_depth: usize, pub css_bytes: usize, pub queued_bytes: usize,
    pub cache_bytes: usize,
}
pub struct CompileReport {
    pub revision: u64, pub changed: bool, pub css: String,
    pub rejected: Vec<CandidateDiagnostic>, pub gpu_programs: Vec<GpuProgram>,
    pub gpu_variables: BTreeMap<String, String>, // 原始theme及注册property初值，live宿主值优先
}
```

Limits 是调用方必传的正数，不引入未测量的 SDK 默认上限；demo/测试的具体值在 M1 根据 fixture 设置并记录。Rust Dict 按输入顺序；JS 导出在 Rust facade 内复现 cn 的参数遍历/数值格式/对象枚举语义，不能先用 JSON 将 undefined、NaN 等有损转换。对 cn 公开类型范围做差分；循环对象等非法输入有深度/大小限制并返回结构化错误。配置支持内置 validator 及用户 callback；Rust callback/WASM 函数句柄逐例验证，不能将 function 输入一律变成 UnsupportedFeature；callback 异常沿用上游调用行为及 facade 错误边界，不静默当不匹配。

UtilityScope 另外接收调用方必传的正数 `request_timeout_ms`，用于初始化和单请求 deadline；超时错误与 Limits 错误分开，测试与 demo 的具体值在 M1 固定。Worker 通信返回 `Result<CompileReport, UtilityError>`；错误至少包含协议/资源不符、语法/缺资源、配额、超时/Worker失败、CSS安装失败、已释放，候选诊断与整个调用错误分开。

删除测试：去掉 core 将导致同一冲突/候选/排序规则散入 SDK 和 Worker；去掉 runtime 将让每个 scope 反复实现 CSS 事务、清理和乱序处理。因此这些模块有实际职责。不开泛化 backend trait、远程编译服务或多层 adapter。

## 3. 内存状态与兼容生命周期

不新增数据库或持久化候选。每个 scope 的状态包含唯一 runtime ID、config epoch、request ID、accepted classes、rejected cache、当前 stylesheet revision 与最后成功 GPU programs。runtime ID 由宿主跨 WASM 分配或浏览器唯一 ID 产生，不复用人类 scope 名作为唯一键。**主线程 runtime 的最后成功安装快照是恢复权威；Worker 的 committed 集合是可重建的编译缓存。**

- Worker 编译器维护 committed 候选集合与 pending 报告；配置 epoch 变化销毁全部候选解析/负缓存/规则缓存，再用 active classes 重建。主题 token 值变化只更新主题/GPU 求值，不重新解析同一份 CSS 配置。
- 候选集合在一个 epoch 内只增加。DOM 删除节点不立即回收 CSS；显式 rebuild/配置替换可重建当前 active 集合。超 quota 拒绝本批新增，不暗中驱逐仍在使用的规则，也不无限保留“所有历史类”。
- 队列保持一个执行中请求；按候选集合合并待编译请求，并保留各调用方完成凭证。配置切换只 supersede 旧 epoch，不能丢掉同 epoch 的新增类。
- scope dispose：停止 Observer、取消消息接收、删除拥有的 sheet、释放 programs/session；实例 abort 另用宿主清理函数 terminate Worker、解除 adopted sheet。不能只依赖 Rust Drop，因为 panic=abort 不跑析构。
- 保留组件 props、variants!/clx!/void! 调用语法及生成的 Class/Variant/Size 类型和常用方法；移除 rustify_components::tw_merge 公开路径，所需 trait/类工具通过 rustify_components::cn 暴露，直接依赖旧导出的调用方按文档迁移。默认静态与动态组件均用新 cn；SDK 不开启 compiler 时 build-web 不构建/复制 Worker。
- 所有 worker/glue/WASM 带构建摘要与协议版本，build manifest 登记；旧缓存与新协议不匹配时拒绝握手。移除 feature 并回到静态 stylesheet 即可回滚；不涉及数据迁移。

## 4. 集成与兼容契约

| 依赖/契约 | 现状判定 | 本期处理与失败语义 |
| --- | --- | --- |
| cn 默认表与配置 | Rust 默认表/config/table/build 核心差分通过；WASM facade 的 JS callback crossing 在 Node 与 strict CSP 浏览器页面均通过 | 保留 group 匹配顺序/有向邻接/内置 validator；生成表版本与摘要纳入锁；表与 engine 不匹配拒绝创建 |
| 组件类名及 variant 接入 | 静态迁移与旧包删除已通过；scope cn/raw接线与已提交epoch配置事务在root/subpath浏览器通过 | 所需拼接/variant 能力迁入 cn；组件从原始片段只执行一次 cn，删除旧包链；既有 override fixture 验证用户可观察行为，差异以固定 cn 契约判定并在迁移文档说明，不引入旧引擎兼容模式 |
| Oxide filesystem Scanner | Rust 真实/虚拟源选择核心及固定语料 native/WASM 已通过；可选 scanner Worker 与 demo 源码面板已集成并通过 root/subpath | 移植纯抽取闭包；源发现/ignore/glob 对齐上游，native 文件源与 WASM 虚拟目录/内容/变更共享选择语义；M1 确认目标闭包，不模拟浏览器没有的 OS 文件访问；失败只阻塞扫描集成，不阻塞完整类名的动态编译 |
| Tailwind TS compiler | 固定版本上游34个测试文件经 Rust facade 执行并逐条入账：1,183 通过、0 失败，JS 插件/配置及其辅助、JS walk API 按分期排除并留证据；compiler-api/slice/upstream 三组门通过 | 移植语义，以官方固定版本作为测试 oracle；无法识别的有效语法在兼容账本记缺口，不静默当作非法输入 |
| CSS import/resource | 已核实·足够：上游 loadStylesheet 是显式入口 | Rust 使用调用方提供的虚拟资源映射，带 base 与相对路径规范化；循环、缺资源报诊断；不在 compiler 内 fetch/访问磁盘 |
| SDK theme | 已核实·足够：主题提供者与 GPU 数值投影已存在 | 把 sdk.css/theme-v4.css 的编译配置加入虚拟 bundle，复用主题快照；边界值随原 provider 传播 |
| Worker bindgen | 独立 web glue 已实现/验证，不经过 Makepad patch；build-web 已产出带 manifest 的 Worker 产物 | xtask 通过与 workspace 同版本的 wasm-bindgen-cli-support 生成独立 web glue，不调用 Makepad patch；同源相对资源路径，支持 --base |
| strict CSP 动态 sheet | 本仓严格 CSP 下的 staging/adopted、scope/property/nesting 和 computed style probe 已通过，见 M1/M3 记录 | SDK runtime 继续沿用已验证的 CSSStyleSheet 路径；完整 cascade、scope/config/recovery 验收归 M6/M7，不放宽 CSP |

DOM 最终兼容范围：内置 static/functional/negative/fraction/arbitrary value/arbitrary property、prefix/important/modifier、全部内置 variant（含 group/peer、aria/data、responsive/container、supports、任意 selector）、CSS nesting 与 @layer、主题配置与关键帧、@utility 静态/函数式、@custom-variant、@variant、@apply 与其循环错误、@reference/@import、@theme inline/reference/default/static、@source inline/not inline 与 source(none)、CSS 中 Tailwind 函数和主题依赖优化。按 v4.1.13 的实际注册规则与测试建立清单，不能仅列函数名当完成。

浏览器模式的路径型 @source 在调用方提供的虚拟目录、ignore 信息与内容映射上执行源选择；缺少所需源信息明确诊断，不把路径型 @source 静默降成无效。native 文件源使用同一选择规则；虚拟源/真实目录 fixture 对照官方 Scanner 的结果。浏览器文件变化由调用方显式提交，不假装拥有 OS watch。JS @plugin/@config 按已确认分期明确 `UnsupportedFeature`，不会执行。自定义源文件可带普通 CSS，但 GPU 不解析全量 CSS selector；GPU 支持以 §5.5 为唯一契约。导出未加 runtime scope 变换的标准 CSS，以便与官方 compiler 差分；运行时输出加隔离规则，两者分开验收。

## 5. 核心机制

### 5.1 cn：合并语义与迁移

1. Rust 实现条件拼接：空/falsy 输入、数字、嵌套列表、字典；cn/twMerge/twJoin/clsx 分别验证，不混用拼接和冲突合并规则。
2. 按固定 cn 的表分类类名；保留不认识的用户自定义 class；顶层冒号/斜杠的解析考虑括号与任意值。
3. 从右向左应用有向冲突：`p-2 px-4` 保留两者，`px-4 p-2` 后者替换前者；variant context、重要性、postfix line-height、arbitrary property 单独匹配。顺序敏感 modifier 不能全排序；普通 modifier 的 canonicalization 与上游一致。
4. 使用全文与 token 有界缓存，按 config epoch 清空。哈希命中必须核对原文，不能用裸指针/哈希当相等；JS 可变数组/字典不按身份缓存。正确性先于复刻上游 JS 专用缓存优化。
5. 对齐固定上游 createCn/createTwMerge/extendTailwindMerge/fromTheme/validators/defaultConfig/mergeConfigs、完整 config、extend/override/prefix、配置函数与 validator callback。createEngine/table 输入校验格式和摘要；Rust 表编译与上游 compiler 差分。build 的选项与裁剪表行为在 M1 API 账本核实，不能仅复制一次 generated 表充当支持 build；Vite/Next/CLI 等入口只承担工具接入，核心表编译由 Rust 实现，逐项分类接口形态与语义。
6. Rust SDK 调用直接借用字符串；WASM JS cn 导出允许一般参数形态，初始化后同步使用。性能报告把 Rust 内部、JS/WASM crossing 与 cn JS reference 分开。
7. cn 是组件唯一合并引擎：未配置 UtilityScope 时使用固定 cn 默认配置，配置后使用该 scope 已提交的 cn。原始 base、variant、size、caller 片段按既定顺序一次性交给 cn，不先经旧 merger 或默认 cn 再合并。初始化时计算 class 的组件也须绑定已提交配置 epoch；replace_config 成功后从保留的 raw 片段重算，失败保持旧 class/config。
8. 将旧包中本仓实际使用的 join、纯 variant 类生成与必要 trait 迁入 cn::class_variants；用 macro_rules 生成枚举、首项 Default、类片段和常用方法，替代 TwClass/TwVariant derive。组件 crate 保留视图包装及 scope 配置读取。保留现有组件类型名称和调用语法，trait 改从 cn 导入；不复刻旧包无使用者的全部 API，不整体复制旧包。
9. 迁移全部组件、宏、测试导入后删除 tw_merge/tw_merge_variants 的依赖、锁文件项和公开旧包导出。最终源码无旧包调用，依赖图在 normal/build/dev 各类边均无旧包；不使用改名依赖、可选 feature、test-only 依赖、vendored 旧引擎或 fallback 保留它们。冻结的迁移 fixture 可记录原有行为，但合并 oracle 始终是固定 cn。

项目裁剪是调用方的主动选择，默认动态模式使用完整表；上游裁剪入口遇到未知类保持其原行为。动态扩表使用独立新增入口，不偷偷改变上游 build 契约。cn facade 的 API 名称/签名可保持上游风格，Rust 类型采用惯用表达，但每个转换必须在 API 账本注明等价关系。

cn 不编译 CSS、不自动注册类。调用者采用 `merge → ensure_classes → commit class/props`；Observer 是补充，存在一帧延迟，不能作为无闪烁承诺。

### 5.2 扫描：抽取完整文本，不推演程序

extractor 负责从整段源码文本发现候选，不生成 CSS。ensure_classes 和 DOM classList 已提供完整类名，绕过 extractor 直接进入 compiler；只有 scan_content/源扫描走本节。扫描能力仍按 R-2 完整交付，拥有独立风险门与验收。

- 使用上游状态机抽取：Rust/Leptos class 字符串、HTML、JS/TS/JSX、JSON；完整预处理清单从固定上游导入并逐项验收。可先实现默认文本/JSON/Svelte，但不得将先实现的语言当最终范围。
- 扫描保留 variants、任意值、嵌套方括号和 CSS 变量；零命中返回空结果。内存文本可带 syntax hint，用同一 fixture 验证 host/WASM 一致。
- `format!("bg-{}-500", color)`、`"p-" + size` 不可静态推导；调用方在值确定后传 `bg-red-500` 或 `p-7`。候选“形似有效”与 compiler 真正有效分开。
- Rust raw string、Unicode/转义、引号内复杂任意值、非法括号、长输入列入差分；不拿几条正则代替 extractor。
- 多次扫描结果只送新候选；CSS 变量依赖另传 compiler。glob、显式/自动源、ignore、变更文件与排序按固定 Scanner 语义实现，以真实/虚拟两种内容源验证等价；native 文件层不把 filesystem/Rayon 引入 WASM 计算闭包。

### 5.3 CSS compiler：移植依赖顺序

动态编译为运行时完整类名生成构建时缺失的 CSS 规则。任意值、主题 token、variant 条件及依赖需要按规则解析生成，不能只预存一张 class→声明字典；DOM 使用生成的 selector/at-rule/declaration，由浏览器求值，GPU 另按 §5.5 投影。相同 epoch 下复用已有候选缓存，只解析/生成新增候选；最终仍按完整集合排序/序列化，不能按到达顺序追加而改变 cascade。

1. CSS/value parser、token/segment/escape、AST walk/serializer；保留 CSS nesting、at-rule、空白/转义语义。排序和 GPU 投影依赖结构化 AST，不能从打印结果拆字符串。
2. theme 与虚拟 import/reference；复制固定 default theme 数据并可覆盖。实现 inline/reference/default/static、prefix、变量依赖、关键帧与注册属性 hoist；不能只把 @theme 拆成颜色字典。
3. Candidate/Variant parse；一个原始类可有多个解释，正确注册 utility 才决定有效性，解析失败与未注册分开诊断。
4. Utility registry 分组移植：布局/间距/尺寸/字体/颜色边框；shadow/ring/transform/filter；动画/交互/其余内置规则。静态映射可预生成，带负值、比例、alpha、CSS 函数的规则用 Rust 实现。
5. Variant registry 按上游顺序移植，支持 compound、任意 selector 与 at-rule。排序位集合不压成固定 u64，以免自定义 variants 超过位数后改变顺序。
6. @utility/@custom-variant/@variant/@apply、Tailwind CSS functions 与 @source inline brace expansion。@apply 拓扑排序，循环报错不提交半成品。
7. 每个新类缓存 parsed candidate/compiled AST；accepted 集合按官方 variant rank、property-order、declaration count、类名排序。serialize 当前完整输出，不按输入序、字典序或 DOM mutation 到达顺序决定 cascade。
8. Report 给出 admitted/rejected/unsupported 区别。普通未知候选不发规则，与官方宽容模式一致；显式 ensure 调用拿到逐候选诊断。有效但尚未移植的 family 必须计为兼容失败，不得伪装成 unknown。

M3 先完成第一组完整动态闭环，M4 完成全部上述范围；M3 不宣称完整 Tailwind compiler 交付。

### 5.4 DOM runtime、Worker 协议与 CSS 提交

协议载荷：`protocol_version, runtime_id, epoch, request_id, operation, payload`。请求包含 init、add_candidates、scan_content、replace_config、ack/nack、dispose；响应包含 ready、prepared（CSS/IR/诊断/计量）、committed、error。单 session 同时只有一个 pending prepared，确认 ack 后才进入下一请求；主线程所有等待受 deadline 约束。

主路径：

输入为 ensure_classes 的完整类名或 Observer 读取的 classList：去重/检查已有集合 → compiler 生成缺失规则及依赖 → 排序/安装 CSS → 提交 class/props。此路径不调用 extractor；独立的 scan_content 先抽取源码候选，再汇入同一编译队列。

1. 应用启用 feature，UtilityScope 从现有 ScopeRoots 获取容器、实例 abort signal 与 base；注册独占 runtime ID，异步初始化同源 Worker、WASM 和虚拟资源 bundle。scope 未配置则仅使用静态 CSS。上下文 cn 配置与 compiler 配置在同一 replace_config 事务生效。
2. 从构建 manifest 载入该配置的静态候选 seed，并枚举 scope root 与 descendants 的 classList；动态 sheet 由完整输入配置、静态 seed 与新增候选的合并集合生成，避免仅含新增规则而改变静态/动态同层优先级。原 CSS 排序和 scoping 后的实际 cascade 分别对照，不以 scoping proximity 掩盖候选顺序差异。Observer 只收集 attributes(class) 与新增子树，排除自己安装的载体；不扫描整个 document，不把 display:none 当无效。
3. 合并重复候选并校验 limits；若无新增、无变量依赖改变则返回现 revision，不重复排队/编译。显式 ensure_classes 完成凭证只在成功 sheet 安装与 runtime 快照提交后兑现；Worker ack 是编译缓存确认，不把分布在两处的状态假装成一个不可中断事务。
4. Worker prepare，保持最后成功的 committed 集合；输出完整有序 stylesheet、GPU programs 和诊断。runtime 检查实例/epoch/request；旧 epoch 回复丢弃，迟到回复不能覆盖新配置。
5. 为动态 stylesheet 做 AST 级 scope 输出：用唯一 data-rustify-style-owner 作为 @scope 根，嵌套 scope 作为下界；theme 的 :root/:host 输出重定位到该根。先保持原 AST 规则顺序，scoping 不增加 utility 相互间的 specificity。
6. @property 和 keyframes 属于 document 全局事实：编译器生成的内部变量/关键帧按 runtime ID 命名空间化，并同步改引用；应用公开 theme tokens 保留名称，其值中流入关键帧名位置的名称改为本 owner 的关键帧名，使在本 scope 内经 var() 读取该 token 的动画消费者（含内联 style）仍指向本 owner；外部把同一 token 当普通值读取时看到的是改名后的值，这是已确认的契约。引用按 var() 代换后的 animation 子属性归属判定；内部变量的用途需要不同改写时，用 owner 私有副本承载其余用途；公开 token 的用途需要不同改写时（同时作关键帧名与 timing/普通值），副本无法跟随应用对该 token 的覆盖，报告 UnsupportedFeature，配置事务保留旧 revision。归属取决于运行时级联、无法静态确定时报告具体特性，不猜测。无规则不能引用已删除的 namespace。自定义全局资源（如 @font-face）由应用静态 asset bundle 管理，runtime 不自动注册网络资源。测试 group/peer、任意 selector 与两层 scope，不只测简单 descendant。
7. 在未 adopted 的 staging CSSStyleSheet 中完成 parse/replace，再按固定位置替换旧 sheet。静态层继续链接；动态输出沿用相同 @layer 序，不用“永远最后插入”规避排序。浏览器可能静默丢弃不支持规则，必须用 M1 特性 probe 与相关 CSSOM/computed-style 验证，不能以 replaceSync 未抛错当全语义通过。CSS 安装失败 nack，不更新 GPU/revision。
8. 安装及同任务内主线程快照切换成功后，发布同 revision 的 GPU programs、候选与配置，发送 ack 并兑现 ensure。该同步提交中任一步失败恢复旧 sheet/快照；ack 响应若丢失或 Worker 崩溃，保留已成功安装的新快照，以它恢复 Worker。尚未完成安装的 prepared 不能成为重放来源。主线程当前 revision 才可交给 RegionApp，GPU 在安全的 apply_props 阶段接收。DOM 与 GPU 未必同一时刻完成 draw；报告最后已绘制 revision，终验等待两者一致。
9. Worker trap/超时 terminate；保留最后成功 sheet/programs，明确返回失败；显式 retry 使用新 epoch 重放 committed 输入。无无限自动重启。scope dispose/instance abort 用不依赖死 WASM 的宿主回调清理 Worker 与 stylesheet。

配置替换也走 staging 全量重建；失败保留旧 CSS 配置、旧 cn 配置与旧 GPU programs，不能只换其中一个。cn 的 callback 句柄留在调用 realm，由主线程 Rust facade 使用，不序列化/传递 JS 函数给 Worker；准备新 cn 实例成功后，与 CSS/programs 一起切换。运行时值主题变化保持 CSS 配置 epoch，重新求值含 var 的 GPU programs；编译主题定义变化才 replace_config。

### 5.5 GPU：支持矩阵与 cascade

GPU public API 分两步：编译阶段产出每类的声明/条件程序；`resolve_gpu(programs, resolved_theme, context)` 产出完整的 `GpuComputedStyle` 和诊断。context 包含 viewport CSS px（浏览器 viewport，响应式与 DOM 一致）、控件状态、主题模式、父尺寸、rem、字体及逻辑方向。原生纯求值可通过 host tests 验证；本仓 native Makepad backend 没有运行验收基础，不承诺已验证原生窗口呈现。

| utility/条件 | 本期 GPU 支持 | 限定与验证方式 |
| --- | --- | --- |
| bg/text/border color、alpha | 内置色值、主题语义色、受支持颜色 arbitrary 与 var fallback | 复用 parse_color/Rgba；动态主题及边界重新求值，不从旧缓存取 RGBA |
| p/px/py/pt/pr/pb/pl、gap、w/h/min/max/size | 数值、px/rem、auto 与明确可求值的比例；数值不可解则诊断 | Layout padding/spacing、Walk 尺寸；分轴/边的 cascade，比例需显式父尺寸 |
| flex-row/flex-col、items/justify | demo 容器有限方向与对齐映射 | 不承诺 CSS flex shrink/basis/wrap 与 Makepad 完全等价；未列项 UnsupportedGpuUtility |
| rounded、border-width、opacity | 圆角/统一边框/alpha，Button 与色块 | 非均匀角/边无绘制能力时拒绝；opacity 只在不需 group compositing 的 leaf 上支持 |
| font family/size/weight、leading/tracking | 文本样本与 Button，已打包字体及明确数值 | font-size 复用 css_px_to_points；family 复用 GPU font resolver；weight 缺字体资源时诊断，不冒充支持 |
| shadow/ring | 已有 ShadowLayer 能表示的外阴影、ring 宽/颜色与 offset | 层合成顺序来自 IR；inset/mix-blend/filter 类不支持 |
| dark、hover、active、focus/focus-visible、disabled | Button/样本的显式状态条件 | focus-visible 增加键盘/指针输入 modality；不能把所有 focus 当 focus-visible |
| sm/md/lg 等 responsive | 使用同 compiler breakpoint 表和 viewport 数值 | Resize 使条件重新求值；不以 canvas 宽偷换 DOM viewport；container query 后续 |
| arbitrary value/property、自定义 @utility | 仅当编译声明和值均落入上述集合，且 selector 是可识别 self 条件 | 通过声明 capability 检查，禁止仅凭 class 前缀猜支持 |
| grid/table、复杂 flex、position/overflow DOM 模型、group/peer/has、任意关系 selector、container query、filter/backdrop/3D transform、伪元素、动画、任意 calc/颜色函数 | UnsupportedGpuUtility/Variant/Value | DOM 仍正常编译；GPU 默认保留上一次成功样式，并给出具体类/属性/原因 |

GPU cascade 使用编译器同一 order key，加重要性与已支持条件；不以类列表最后一个 token 推断属性最终值。仅在可证明等价的 self-selector 范围支持 specificity；复杂 CSS 选择器/任意用户 CSS 不进入有限求值器。cn 合并只做输入合并，不代替 shader 侧 cascade。

`GpuResolution` 是完整快照，不是只含新增字段的补丁：以控件既有 apply_theme 默认样式为底，叠加 utility，再赋值；删除 class 后恢复该字段，hover 离开也恢复。不每帧解析 class，program 只在类/配置变化时换，值求解在 theme/state/viewport 变化时缓存更新；仅数值真的变化时 redraw。

默认 apply 策略是严格模式：列表内出现 GPU 不支持项则拒绝整次 GPU 更新，保持旧快照并提示；应用可明确选择 partial 模式应用支持声明，但返回 Partial 并显示缺口，demo 提供切换。不得把 partial 结果标成 DOM/GPU 一致。混合列表不会因一个未知类而清掉所有已有 GPU 样式。

## 6. 验收 demo 的交互设计

目标对象是 Rustify SDK 使用者，进入 utility-lab 验证运行时类名效果；沿用 DESIGN.md 的中性编辑工具、语义色、紧凑字段与清晰焦点，不改变现有主题站。

- 顺序：输入 base/override/条件 → 看合并结果 → 生成/应用完整类名 → 并排 DOM/GPU 预览。源码扫描与输出 CSS 放折叠面板；将开发细节保留在这个明确的开发工具里。
- 支持输入源码和运行时构造类，例如初始构建语料中不存在的 w-[137px]、bg-[#123456]；应用动作等待 ensure 成功，GPU 以 drawn revision 标明已更新。
- 单独展示 cn 结果、CSS 候选诊断、GPU 支持诊断，错误落到输入旁；unsupported 不使用“编译失败”混淆三条能力。
- ready/loading/empty/error/partial 状态明确；失败仍保留上一预览并提供重试。两 scope 不同主题和同名类的实例隔离作为可操作 fixture。
- 桌面预览并列，窄屏纵向；页面外壳用严格 CSP 下由页面脚本驱动的原生表单控件，按 DESIGN.md 的中性样式与清晰焦点呈现，预览内部组件仍是 SDK 组件；标签、错误、loading 由页面中/英词表提供，初值按读者语言子标签选择，SDK 自带消息沿用 SDK 的 i18n；aria-live 不重复朗读每个编译 token。
- 键盘应用、tab/focus-visible、暗色、长类换行、error 恢复与 GPU context 恢复列入真实浏览器验收；无装饰动效，reduce-motion 沿用已有主题。

## 7. NFR、安全与运行保障

| ID | 机制与明确边界 | 观测/验收 |
| --- | --- | --- |
| NFR-1 | Limits 覆盖所有输入、资源映射、候选数、AST 深度、输出/队列/缓存字节；先校验再分配；超限不改 committed 状态 | limit 前/等于/超过边界；长任意值、循环 import/apply、未知类 churn；V-9 |
| NFR-2 | Worker 隔离重型编译；cn/render 同步短路径；重复请求 no-op；AST 缓存；GPU 值改变才 redraw | V-11 记录 cold/warm/cache hit、首次/新增/配置重建、主线程 apply、GPU draw、Worker memory、gzip/WASM 大小 |
| C-2 | 同源 worker/glue/WASM/stylesheet，构建产物无 eval/new Function、JS compiler、SAB；输入不变成 innerHTML | strict CSP securitypolicyviolation、网络请求来源、产物依赖检查；V-1/V-9/V-10 |
| R-5 | 颜色/尺寸求值拒绝非有限值，未支持条件显式拒绝；主题 unit 与 gap 分离 | host/WASM parity 与 GPU 数值/绘制；V-7/V-8 |

应用输入是开发者配置，不把 arbitrary CSS 当作安全隔离语言。compiler 不主动加载 CSS url/import 网络资源；runtime scope 尽量隔离 utility 影响，但 CSP 和受信应用资源负责网络/字体权限。不引入账号/鉴权系统。

数字延迟/包体 SLO 未提供。M1 固定机器、浏览器、release 模式、输入集合及测量口径，M7 以同条件比较；数值与未达到的目标写记录，不临时用宣传倍数当门槛。若产品后续指定硬预算，更新 NFR 与受影响里程碑；本计划的功能正确性、strict CSP、队列/内存边界及主线程不执行编译为硬门。

runtime statistics 以开发 API/Performance marks 提供 input bytes、new/admitted/rejected candidates、cache hit、compile/apply duration、queue bytes、CSS bytes、revision；生产日志不记录完整源码或类值。不同 runtime/epoch 的数据分开统计。

## 8. 失败模式、发布与回滚

| 触发 | 影响/后果 | 表现与检测 | 恢复 | 验证 |
| --- | --- | --- | --- | --- |
| Worker WASM/协议不匹配或资源 404 | 当前 scope 新类不可用，静态样式不变 | 初始化错误/manifest hash | 修正同一批静态产物，重新初始化 | V-9、V-10 |
| CSS 配置语法错、缺 import、apply cycle | 当前请求无提交 | 定位 span/资源 ID；保留旧预览 | 修输入后 replace_config | V-5、V-9 |
| CSSOM 失败/部分 CSS 不被浏览器支持 | 当前 CSS/GPU revision 不更新 | staging 失败/兼容 probe | nack；不重用错误 prepared；报告具体特性 | V-1、V-6 |
| 新候选超过 quota、缓存 churn | 当前批拒绝，无候选暗中丢失 | LimitExceeded/计量 | 减少输入或显式 rebuild(active) | V-9 |
| 旧 epoch 回复、连续快速编辑 | 仅丢弃过期结果，不覆盖新配置 | epoch/request/revision | 新请求继续；最后成功状态保留 | V-6、V-9 |
| Worker trap/挂起或安装后ack丢失 | 隔离到该 Worker；应用/GPU 仍运行 | error、调用方 deadline | terminate；以runtime最后成功安装快照新epoch重放，不以未提交prepared恢复 | V-9 |
| GPU 无法求值/不支持、GL context lost | CSS 可成功，GPU 明确失败或恢复中 | diagnostics、drawn revision/RegionState | 修类/明确 partial；context 恢复后重放当前快照 | V-7、V-9 |
| scope dispose 或主应用 trap | 该 scope/实例资源必须释放 | abort cleanup、Worker/sheet计数 | 宿主移除持有资源；其他实例不变 | V-9 |

交付顺序：先核心+facade+组件所需 cn 类工具 → 全部组件静态合并迁移并删除旧包 → Worker 产物与 runtime → 动态 scope 上下文与 demo → 回归/文档。组件切换在 cn 契约及类工具 fixture 通过后集成，不等待重型编译器完成。worker/glue/WASM 与应用 build manifest 作为同一批产物交付，子路径资源保持一致。现有 CLI 构建产物不替换、不重新生成无关 CSS。

回滚：应用移除动态 feature/UtilityScope，dispose 动态资源，回到已有静态 stylesheet 与新 cn 默认配置；不恢复 tw_merge 依赖或旧合并引擎。已只存在于运行时的类将失去规则，应用需回到静态基础类，不能宣称功能无损回滚。没有数据库或用户数据回填；公开发布/部署不在本期。

## 9. 验证

### 9.1 检查定义

所有 ★ 命令/项目均为拟新增入口，必须在指定里程碑实现后运行；不能误报现仓已有。预期结果由固定上游源码生成并提交，CI 日常不访问 upstream main。参考工具的依赖独立锁定，不进入生产 runtime；使用 Node 测 TS oracle 不违反全 Rust 生产契约。

| 检查 | 需求与可观察断言 | 环境/入口 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | A-1/A-2/C-1–3 分三组：V-1.extractor 最小非线程 WASM/fixture，无 FS/Rayon；V-1.worker 锁定 bindgen 独立 Worker，无 scanner 的启动/候选消息可用；V-1.css 在 strict CSP 下安装构建时缺失 utility 并核对 computed style，验证 staging/adopted sheet、@scope/@property/嵌套及清理 | ★ `mbx xtask utility-baseline` 实现可分别运行/记录三组的入口；已有 build-web/serve + Chrome，截图/console/request evidence | M1 分组风险门；扫描只依赖 extractor，动态 DOM 只依赖 worker/css；各组通过后可供下游使用，不等待整项 V-1 |
| V-2 | R-1/R-7/R-8：cn/clsx/twJoin/twMerge、完整config/extend/override/prefix、callback、engine/table/build逐例相等；方向冲突、任意值、重要性、modifier 顺序、postfix、未知类、Unicode及缓存有/无一致；迁入的类工具保留variant/size/首项默认/caller覆盖，组件raw片段与直接cn输出一致；自定义不冲突分组不丢类，配置成功切换重算已挂载class、失败不变；JS/WASM 参数差分 | ★ `mbx test -p rustify-style --lib cn::`；`mbx test -p rustify-components --lib`；★ `node tests/utility/reference.mjs --check cn`；WASM facade/动态组件配置切换用例在浏览器 | M2 cn/类工具与静态组件local；M6 crossing/配置切换；M7 收口 |
| V-3 | R-2/R-7：完整extractor/preprocessor、Scanner源选择与增量fixture的candidates/variables相等；Rust raw string 等源码、空/非法/大输入；动态拼接不能被预测，scan_content 抽取结果与直接提交同一候选列表的编译结果一致 | ★ `mbx test -p rustify-style --lib scanner::`；★ reference --check scanner；native/WASM真实/虚拟目录及同一JSON语料 | M2 local、M6 扫描输入集成；不作 M3 前置 |
| V-4 | R-3/R-4：scanner 未启用时 ensure 完整类名可用，静态不存在的 w-[137px]/bg-[#123456] 无需重建生效；重复 ensure 无新增解析/规则生成；逐候选 IR/CSS 与 official core 对照；不冒充完整兼容 | ★ `mbx test -p rustify-style --lib compiler::`；★ reference --check compiler-smoke；utility-lab root 的 compiler-only 构建及 DOM 样本 | M3 |
| V-5 | R-3：§4 所有范围内 utility/variant/directive 在分类账有 fixture；官方有效输入零未解释差异；排序/变量/keyframes/property/nesting、错误语义等价；随机批次 union 与一次性编译一致 | `node tests/utility/reference.mjs --check compiler-full`；`mbx xtask utility-conformance --check`（默认 compiler-full = compiler-api + compiler-slice + compiler-upstream，后者用上游锁定的 vitest 执行固定上游测试文件并核对逐测试处置账本）；测试分组来自固定 upstream registry/tests，文件扩展名与资源都已锁定 | M4 local，M7 full native/WASM |
| V-6 | R-4/C-2/NFR-1：ensure 后 class 应用；ensure/classList 不调用 extractor，scanner 关闭时仍可补规则；Observer attribute/新增子树补充、no-op、随机到达顺序、相同类多 scope 不串；配置失败保留旧 revision；静态/动态同层与 variant 优先级对照官方 CSS，新增规则含所需依赖 | ★ `npx playwright test --project=utility-lab --workers=1 utility-runtime.spec.ts`；现有 strict serve | M3 初闭环；M6 完整运行时；M7 回归 |
| V-7 | R-5：矩阵每行的真实控件与数值断言；DOM/GPU颜色/alpha、布局、文本、ring/shadow 对照；主题/hover/keyboard focus-visible/resize/删除类恢复；strict/partial与不支持项 | ★ host utility GPU tests；★ `npx playwright test --project=utility-lab --workers=1 utility-gpu.spec.ts`；真实 Chrome/WebGL2，几何/采样和截图并存 | M5 local/真实绘制；M6 demo；M7 完整 |
| V-8 | R-6/R-8：原主题值/暗色/spacing unit、boundary overlay、字体、既有组件 class override 行为保持；默认组件已用新cn，dynamic feature未开时无compiler Worker/编译器额外包体 | 既有 `npx playwright test --project=component-catalog theme-compat.spec.ts`、`--project=theme-studio` 受影响 specs，逐个执行；workspace tests | M2静态组件迁移局部；M6动态接入；M7 收口 |
| V-9 | NFR-1/C-2/R-4/5：limits 边界、循环/深嵌套、队列重入、worker error/timeout/late reply、两实例/两scope、dispose/trap清理、GL restore、不发未授权网络资源 | ★ `tests/browser/utility-faults.spec.ts`，utility-lab 单 project；明确 failure injection 与最后成功 revision | M6 运行闭环齐备；M7 |
| V-10 | C-1/C-3/R-8：host/wasm checks、来源/生成表摘要、静态与动态双构建、root/subpath无漂移/资源404，产物无TS compiler/JS引擎/SAB；Cargo manifests/lock及所有features/targets的normal/build/dev依赖图均无tw_merge/tw_merge_variants，源码/公开导出无旧包调用，无改名或vendored旧引擎 | §9.3 既有入口 + ★ worker/glue conformance；`mbx tree --workspace --all-features --target all --edges normal,build,dev --locked`与源码/锁检查；★ utility-lab-deep project；build manifest compare | M1 worker局部；M2旧包删除局部；M7完整 |
| V-11 | NFR-2：同机器前后数字，cn crossing与内部独立计量、scanner bytes/s、cold/warm compile p50/p95、no-op、CSSOM主线程时长、GPU frame、memory/包体 | ★ `mbx xtask utility-bench`（native）与 utility-lab 浏览器计量；headed Chrome 真 GPU，沿用 RUSTIFY_TIER=evidence/report-size | M1基线，M7对照；不以Playwright耗时充当性能 |

### 9.2 差分策略与验收纪律

- cn 的 oracle 是固定 cn commit，不以当前 tw_merge 或 CSS 计算结果代替；未知类的“合并通过”不代表“CSS 可编译”。
- 旧包迁移只复用冻结的 fixture 与必要来源记录，不让测试继续依赖旧包。cn::class_variants 是本仓新增能力，不伪装成上游 cn 已有 API；旧公开导出到 cn 的迁移、已用组件语法/类型/方法的覆盖逐项记账。组件实际呈现与固定 cn 输出都要验证，不为维持旧引擎错误规则修改 cn 默认表。
- scanner 的 oracle 是纯 extractor/对应 preprocessing；不拿整个 CLI 的 FS 发现结果与内存扫描混比。
- CSS 首先与官方未 minify compile/compileAst 对照：AST 规范化只忽略 banner、格式化空白，不排序声明/规则、不消除重复值或自定义属性；保留 invalid candidates 与异常分类。实际 computed style 使用官方 fixture CSS 的并行样本作第二条证据，覆盖浏览器语义。
- 固定 upstream utility/variant 注册项与测试按“范围内已通过/范围内未实现/明确范围外”分类；范围内未实现不允许 xfail 绿灯、降低抽样或改 expected 消除差异。新增 case 回到 upstream 原文确认再分类。
- 上游兼容账本中的公开入口、配置与Scanner行为也按同一规则验收；新增能力账本单独记录动态扩表、Worker/恢复、scope隔离和GPU。不能以新增能力demo通过抵扣上游缺口。生产构建不依赖TS，开发期oracle允许运行上游原实现。
- 所有范围内注册 family 与合法/非法边界均需覆盖；任意值无限空间用固定种子语法 fuzz、性质测试（幂等、批次顺序不变、缓存不变）补充。宣称的是固定范围契约，不能声称数学上覆盖所有 CSS 字符串。
- V-6/7/9 在同一 release demo 可合并走查，保留各检查证据；M5局部不替代M7的全范围CSS/运行时/旧主题终验。GPU真实呈现不以纯数值测试替代。

### 9.3 实际基础命令与浏览器入口

已有仓库入口（按受影响范围执行，不因计划文档本身执行长时间全仓套件）：

```sh
cargo fmt --all -- --check
mbx test --workspace --lib --bins
mbx clippy --workspace --all-targets -- -D warnings
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins
sh tests/theme-text/run.sh --lib
```

拟新增 crate 就绪后：

```sh
mbx check -p rustify-style-wasm --target wasm32-unknown-unknown
mbx xtask build-web --example utility-lab --release
mbx xtask build-web --example utility-lab --release --base /tools/demo/
npx playwright test --project=utility-lab --workers=1
npx playwright test --project=utility-lab-deep --workers=1
mbx xtask report-size --example utility-lab --release --compressed
```

utility-lab build-web 自动构建独立 Worker module，不能仅 check target 算 WASM 运行验证。浏览器 root/subpath 项目与端口须先加入 playwright.config.ts，再执行上述新项目名。主路径保存截图、console/failed requests、manifest摘要、CSS/GPU revision及测量环境。每次只跑一个 Playwright project，性能测量期间不并行 cargo。已有浏览器/Node/依赖缺失时明确标未验证；不私自安装服务或容器。`makepad/` 如无改动不重复其全套测试，如发生实质改动须执行其独立 workspace 检查。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置成果 | 内容与工作边界 | 退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 固定基线与分组技术风险门 | 本计划范围已确认 | 建完整上游API/规则/测试兼容账本及新增能力账本、参考锁/fixture；盘点旧包实际join/variant/trait用法和公开导出，冻结迁移fixture与默认组件表现；分别实施 extractor WASM、无scanner的独立Worker/glue与strict CSP样式安装probe；记录性能基线。共享 Cargo/xtask/sources.lock 由主实施者单写 | V-1各组通过、V-11基线可重放才记M1完成；单组通过可支持依赖该组的下游，extractor失败不阻塞动态主路径；确认无CSP绕过或JS生产编译，按实际解除项就地更新风险/状态；回写「实施进度」 |
| M2 | Rust/WASM cn、旧包移除与上游扫描基线 | M1固定cn/扫描完整API和迁移fixture；scanner要求extractor目标闭包通过V-1.extractor，cn替换不依赖scanner/CSP风险结果 | cn完整默认表/config/callback/表编译与facade参数；所需join/纯variant宏与trait迁入cn，迁移默认组件/测试/公开导出并删除tw_merge/tw_merge_variants；scanner/preprocessor/真实与虚拟源选择；核心features与来源。cn与scanner在稳定入口下可独立开发；cn类工具契约和fixture通过后集成组件，lib.rs/Cargo/锁由主实施者单写 | V-2/V-3局部、V-8静态组件迁移和V-10旧包删除通过，host/WASM语料一致；cn相关检查通过可供M3使用，不等待扫描检查或M2整体验收；重型compiler未开启的组件仍能构建与运行；JS facade和动态配置切换终验留M6；回写「实施进度」 |
| M3 | 首个动态 DOM 闭环 | M1 Worker/CSP 的 V-1.worker、V-1.css；M2 cn final输出及相关V-2可用；compiler开发可先做，不要求extractor/V-3或M2整体完成 | parser/theme/candidate和常用基础utility；完整类名直接进入compiler，prepare/commit、ensure与sheet安装；utility-lab最小输入/DOM预览；先证w-[137px]/bg-[#123456]等静态不存在类，不依赖源码扫描 | V-4和V-6基础通过，包括关闭scanner的完整类名动态路径；记录明确的剩余兼容family，不宣称全范围或GPU完成；回写「实施进度」 |
| M4 | 固定版本compiler完整兼容范围 | M3的AST、排序、session与IR；M1固定完整上游账本 | 完成§4/§5.3全部范围内utility、variant、theme、directive、函数、源输入及错误/API差分；M3切片不降低最终范围 | V-5局部完整范围检查通过、未解释有效输入差异归零；full native/WASM终验由M7负责；回写「实施进度」 |
| M5 | GPU utility求值与实际控件 | M3 IR与所需M4 family已验证，可与其余DOM family并行 | 完成§5.5支持矩阵、完整快照cascade与strict/partial；接入RustifyButton及demo容器/文本/色块，保留None基线与旧主题 | V-7局部host与真实控件绘制、V-8基线/恢复及相关V-9通过；完整集成与终验由M6/M7负责；回写「实施进度」 |
| M6 | SDK、运行恢复与双预览 demo | M4的完整CSS和M5GPU协议、M1样式路径；M2 cn类工具/组件迁移及旧包删除检查；仅scan_content/源码面板集成依赖M2扫描V-3，其余runtime工作不等待扫描 | 组件上下文cn与已提交epoch接入，从保留raw片段重算；Observer/batching/epoch、config原子替换、abort清理；接入独立scan_content/源码面板，demo完整交互/root/subpath；GPU状态/恢复与跨scope。runtime/宿主清理共享资源串行集成，浏览器仅一人运行 | V-2 crossing/组件自定义配置切换、V-3扫描输入集成、V-6/V-7集成、V-8/V-9通过；各模块状态与drawn revision可观测，无旧包fallback；回写「实施进度」 |
| M7 | 全范围终验与开发者交付 | M2–M6全部需要成果与检查就绪 | 同一release构建跑完整差分/故障/浏览器/旧行为必要回归；性能与体积对照；API/兼容矩阵/接入文档、CI/来源校验。主实施者单写最终证据和计划状态 | V-2至V-11受影响完整检查通过，V-1前置证据可复用且未变；缺口如实列明，全部功能满足才记整期完成；回写「实施进度」 |

M2 的 cn/扫描与 M4 各 utility family 只在公开契约冻结且文件互不重叠时委派；M5 可消费已通过的单个 family，不等待 DOM 全量完成，但最终 R-3/R-5 的验收在 M7。共享 workspace、锁、xtask、来源锁、protocol、playwright.config.ts 由主实施者负责；不同时运行两份 Playwright，也不让GPU测量和编译争用机器。上游语义变更先更新契约/fixture再推进，不让子任务自行选择兼容版本。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除方式 | 最晚点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| A-1 extractor闭包 | 是否可复用纯Rust状态机；macro和测试依赖需核到目标 | 实施负责人，V-1.extractor 最小WASM实编译/fixture；无法复用则提出等价移植代价 | M1，扫描集成前 | 仅阻塞扫描WASM集成及其完整验收；不阻塞cn、compiler、ensure/classList动态入口 |
| A-2 CSSOM/scoping严格CSP | 生成后的CSS在本仓CSP/作用域下能否正确安装；不是动态CSS有无实现路径的问题 | 实施负责人，V-1.css最小新增utility的真实Chrome/CSP probe及scope/property/nesting；无可用路径时停并回报 | M1，DOM安装集成前 | 阻塞DOM样式安装集成；不阻塞compiler开发，失败不放宽CSP |
| 全量TS语义移植量 | 最大成本在M4，非单纯WASM打包；未指定工期和人员 | 先按family分类账盘点，M1后根据已知差分和实做吞吐给工期，不虚构天数 | M1后排期 | 不阻塞M1；不得用缩小范围完成M4 |
| GPU能力限制 | 与DOM区别易被误解；CSS成功但GPU不支持 | 实施负责人，§5.5矩阵与V-7诊断/对照 | M5 | 已有明确契约，非用户待决 |
| 性能/包体数值预算未知 | 无法给出延迟与加速倍数承诺 | 实施负责人提供V-11基线/对照；用户若新增硬SLO则修订NFR | M7 | 不改变现形态；无数值SLO发布承诺 |
| cn新版表与TW4.1.13范围不完全重合 | cn可能识别不能编译的类 | cn按固定cn oracle；CSS按固定TW oracle；分别诊断并测试，不能通过改cn规则掩盖 | M2/M4 | 设计已隔离两个契约 |
| 旧包公开导出与默认合并语义迁移 | 直接引用rustify_components::tw_merge的代码需改导入；新旧合并结果可能不同 | 实施负责人，M1冻结实际使用清单；M2迁移所需类工具/组件并删除旧包，按固定cn判定差异、文档说明迁移，不保留兼容引擎 | M2，M6动态配置验证 | 用户已确认删除旧包，非待决；对应检查失败阻塞组件切换 |
| 原生Makepad呈现 | 本仓fork裁剪到浏览器目标，没有原生验收环境 | 纯GPU resolver跑native；原生窗口适配另需明确目标/环境 | 后续 | 本期排除原生窗口呈现承诺 |

- 就绪状态：**已完成**；M1–M7 退出检查均已通过，未验证项与性能解读见 [M7 记录](../validation/wasm-tailwind/M7.md)。
- 定级理由：产品范围和全 Rust/WASM 路线已确认，无用户待拍板项。M1 的非线程 extractor WASM、独立 Worker 与严格 CSP 样式安装风险门均已通过并留有实测证据，可以继续扫描与动态 DOM 集成。M2–M7 的完整功能与验收仍须逐项实现，Ready 不表示完整兼容或交付已完成；不改成混合 TS、不放宽 CSP、不缩减上游能力。

## 12. 已知坑与历史教训

- `crates/rustify-components/src/macros/variants.rs`、`crates/rustify-components/src/macros/clx.rs`、`lib.rs` 依赖旧包derive/join/公开导出：所需能力迁入cn后连同测试导入一起清除；不把已合并字符串再交cn，不仅改merge函数，也不只删Cargo直接依赖。
- `xtask/src/tailwind.rs` 和 `mise.toml` 锁定4.1.13：oracle必须同版本，不能把latest产生的CSS当旧CLI无漂移依据。
- `crates/rustify-components/css/sdk.css` 将layout gap与utility spacing分离；动态编译延续inline/reference，否则p-4等会读错单位。
- `docs/plan/DX-REFINE.md` §5.2记录普通源码词也会被扫描成utility：候选抽取不是Rust语义分析，静态/动态不同入口分别验证。
- `makepad/tools/cargo_makepad/src/wasm/bindgen_glue.rs` patch断言env import形状：纯compiler Worker用独立glue，不假装Makepad产物通用。
- `CLAUDE.md` 明确panic=abort、页面资源用release_on_abort：Worker/Observer/sheet清理必须有宿主路径，Drop不够。
- `CLAUDE.md` 要求Playwright单project串行；serve不重建：先build，保存manifest和revision，避免验旧产物或测试目录相互清空。
- `PRODUCT.md`/`DESIGN.md` 区分CSS与GPU能力：unsupported与partial必须可见，不能只有CSS导出截图就宣称双渲染器通过。
