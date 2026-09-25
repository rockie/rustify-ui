# DX-REFINE · Rustify UI · 测试分层、Vellum 组件沉淀与 Tailwind v4 无前缀化（仓库工具链 + SDK 增量 · 不动 Makepad 分叉）

> **计划状态：Ready**
>
> 调查基线：2026-09-25 · `a0560408d0eb64ce53286f3fabb3642404721bcf` · 工作区 clean（除本计划）。`ref/` 与 `node_modules/` 在本工作树中不存在；Tailwind 行为由本次在隔离目录用 `@tailwindcss/cli` 4.1.13 与 `tailwindcss-linux-x64` v4.1.13 独立二进制实测（§1.1 标注「实测」的条目），浏览器测试未在本次运行，耗时数字取自已提交的验收记录。
> 输入：用户三条诉求——「目前的自动化测试特别耗时间，感觉意义也不是很大，优化一下」「看看从 examples/vellum 怎么提取一些组件作为本项目的公共组件」「更友好和高效地支持 tailwindcss v4」。2026-09-25 拍板：度量/证据类 spec **移出 PR、按需触发**；应用与 SDK **去掉 `rui:` 前缀**（推翻 P2 D13/ADR-5 的样式隔离决策）；Tailwind CLI 由 **mise 固定独立二进制**提供。同日补充：Vellum 的视觉对比以 **`examples/vellum` 自身的改造前构建**为基线（它对 `ref/` 原版已接近 100% 复刻）；mise 注册表只收 bun/node 这类工具简写，Tailwind 独立二进制须用显式的 `github:` 后端从 GitHub Release 安装（与 npm 包无关）。同日确认：耗时目标为最长 CI 浏览器 job ≤ 10 min、本地单示例 ≤ 5 min；接受「输入草稿」作为受控值契约的例外（ADR-4）；接受新增 Toast、数字输入、颜色输入、切换组四个目录类别（D19）；回归层改为「一次启动、多次检查」（ADR-6）。
>
> 本期交付：**（一）测试分回归层与证据层：PR 只跑去重、降回合、复用页面后的行为回归，按示例并行；预算、长跑、内存、空闲、基线测量与孪生视觉移入按需/定时触发的证据层，阈值与回合数不变；CI 补跑一直漏掉的 xtask 与示例 bin 单测。（二）从 Vellum 抽出通用件：SDK 行为原语（作用域监听守卫、快捷键匹配、模态 Tab 循环、可中止的延时/帧回调、Toast 队列、输入草稿）进 `rustify-ui`，Vellum 与 property-workbench 改用；在其上新增四个目录类别（Toast、数字输入、颜色输入、切换组/工具栏）并扩展 Menu/Dialog。（三）Tailwind v4：组件类去前缀，`tw_merge` 覆盖语义对应用类生效；SDK 提供可组合 CSS 入口，`build-web` 编译示例自带的 Tailwind 输入；CLI 由 mise 固定、只扫显式源，生成 CSS 不再需要 Node；编辑器补全配置与应用作者文档。**
> 本期独特职责：P1–P3 与 VELLUM 都在「加能力并证明它」；本期第一次反向——把证明能力的成本降到日常可承受，把示例里被验证过的通用件沉淀回 SDK，并把 SDK 的样式模型从「与宿主隔离」改为「与应用同构」。
> **顶层排除：不改 Makepad 分叉；不改任何预算阈值与证据层回合数；不让 Vellum 改用带样式的目录组件（其 ≤2% 孪生视觉门与 VELLUM ADR-3 不变）；不引入新测试框架（wasm-bindgen-test 等）；不换 CI 操作系统。**

## 实施者定位

执行本计划的 agent 是**资深软件工程师**：Kent Beck 式的 TDD 纪律加上《程序员修炼之道》式的精确。本节由骨架原样带入，不随项目改写；开始任何里程碑前先接受以下约定。

- **表达方式**：极简，每句话都可引用。说到代码给文件路径，说到需求或验收给本计划的 ID（需求账本 R-* / NFR-* / C-* / A-*，决策 D* / ADR-*，里程碑 M*）。不写铺垫、不写感想、不复述计划。
- **完成的定义**：没有通过验证的任务不算完成。里程碑退出条件里的测试、断言和走查全部通过，才能在「实施进度」记为完成；验证没跑、失败或环境缺失，就如实记为缺口或阻塞。
- **工作顺序**：需要行为测试的改动先红、再绿、最后重构。按前置依赖推进；同一或不同里程碑中，前置已满足、契约稳定、文件与运行资源互不干扰的任务，应启动多个 subagents 并发完成。共享文件单人负责，共享数据库、端口、浏览器会话等资源须隔离或串行；工具不可用或无法安全拆分时说明原因并串行。
- **并行交付**：无特别要求时 subagent 使用与主 agent 相同的模型，启动时默认继承，不主动覆盖模型。主 agent 划定各任务的目标、文件/资源边界、验收条件与独立记录路径；subagent 只写自己的改动和记录，回报简要结论与链接。主 agent 统一集成验证，单独负责计划回写和里程碑记录；不以子任务通过代替整个里程碑验收。
- **源码干净**：注释解释为什么，不解释是什么。源码里不出现工单或需求编号（如 `# FR-12`、`// BUG-42`）、本计划的 R-* / M* 编号、agent 工作流标记或任何规划元数据；追溯关系记在计划摘要、独立实现/验收记录和提交说明里。交付的是可直接上生产的代码：干净、最小，没有多余防御、空洞注释或重复样板这类 AI 生成痕迹。

## 实施进度（实施期持续更新）

本节由骨架原样带入，不随项目改写。它是跨对话恢复的**唯一汇总入口**：先读仓库规则、本节、当前任务相关正文和工作树；只有继续未完任务、核验证据、排错或审查时，才按链接读取对应实现/验收记录。不要默认加载全部记录或原始日志。

**回写时机**：每完成一个里程碑，立即先保存独立实现/验收记录，再回写本节的简要进度与引用；早于报告完成、提交代码或启动依赖该成果的任务，不等整批并行任务结束。已独立运行的任务可继续。实质进展后暂停、受阻、切换任务、发现偏差或结束对话时，也须保存已有成果与缺口，刷新快照。

**一次回写**：由主 agent 先更新对应里程碑的独立记录，再同步「恢复快照」全部 7 行及「完成记录」中该里程碑的一行；状态/摘要就地更新，不追加流水。详情按里程碑保存在计划旁的同名 `.records/` 目录，以 `M1.md` 等命名，分“实现记录”和“验收记录”写交付行为、关键路径、偏差、验证命令/步骤、环境、结果和代码基线，未完成时写缺口与继续动作。subagent 详情用独立任务文件，由里程碑记录引用。记录链接只放在完成记录中，计划不复制详情。实现偏差涉及范围、决策、接口、数据、风险或退出条件时，同时修订正文对应章节，快照只点明变化。

**记完成的门槛**：退出条件全部通过才可记完成。验证没跑、跑红或因环境缺失跳过，就如实写进「当前状态」/「当前阻塞」，不写“基本完成”“应该可用”。

**回写后自查**：「当前进度 n/N」的 N 等于里程碑总数，n 只计状态为「已完成」的行；每个里程碑最多一行，转为已完成时将原行移至已完成记录末尾，按实际完成先后排列；「最近完成」对应最后一条已完成记录，不按编号推算；链接指向已落盘文件，完成判定有真实验收证据。校验只读取计划与引用文件存在性，详情按需复核。

本仓库的里程碑证据目录约定是 `docs/validation/<计划>/`（`CLAUDE.md`「Plans」），因此上文的独立记录落在 `docs/validation/dx/m<n>.md`，完成记录用 `[M1 记录](../validation/dx/m1.md)` 这样的相对链接。

### 恢复快照

- 最近更新：尚未开始
- 当前进度：0/8 个里程碑完成
- 当前状态：尚未开始；计划 Ready（2026-09-25 用户确认 D9 目标 10/5 min、ADR-4、D19；同日修订 A-1 为显式 `github:` 后端、A-5/D8 为 Vellum 改造前构建作孪生基线、新增 ADR-6 一次启动多次检查）
- 最近完成：无
- 下一步：M1 · 在 `tests/tier.ts` 建 `RUSTIFY_TIER` 分层与 `rounds()`，给 §5.1 表中证据类用例打 `@evidence`，按需起 webServer；退出条件是两份 config 的 `--list` 计数守恒、component-catalog 回归层跑绿、四种隔离方式的耗时探针（A-2）与 A-4/A-7 有结论
- 当前阻塞：无
- 代码基线：`a0560408d0eb64ce53286f3fabb3642404721bcf`

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 |
| --- | --- | --- | --- | --- |
| — | — | — | 尚未开始任何里程碑 | — |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能（开发流程） | 用户「特别耗时间」 | PR 与日常迭代的测试墙钟时间显著下降：按示例并行、按需起服务、去重、回归层降回合、一次启动多次检查 | ADR-1、ADR-6、D2–D7、§5.1、M1/M2 | 已确认 |
| R-2 | 功能（开发流程） | 用户「意义也不是很大」+ 2026-09-25 拍板「移出 PR，按需触发」 | 度量/证据类 spec 移出 PR，保留为手动 + 定时触发的证据层（阈值、回合数不变）；删除与 host 单测重复的浏览器断言；CI 补跑 xtask 与示例 bin 的单测 | ADR-1、§5.1、M1/M2 | 已确认 |
| R-3 | 功能（SDK） | 用户「从 examples/vellum 提取一些组件作为本项目的公共组件」 | 按 D16 准入：行为原语进 `rustify-ui` 并由 Vellum/property-workbench 改用；新增目录类别与 Menu/Dialog 扩展进 `rustify-components` 并在 component-catalog 展示 | ADR-5、D16–D19、§5.3、M6/M7 | 已确认 |
| R-4 | 功能（样式） | 2026-09-25 拍板「去掉 rui: 前缀」 | SDK 组件类与应用类同为无前缀 Tailwind v4 utility；调用方 `class` 与组件类经 `tw_merge` 正确合并覆盖 | ADR-2、D12、§5.2、M4 | 已确认 |
| R-5 | 功能（样式） | 用户「更友好和高效地支持 tailwindcss v4」 | 应用作者写一份 Tailwind 输入即可：SDK 可组合入口（令牌、暗色变体、作用域规则、组件类源）、`build-web` 自动编译、编辑器补全、文档 | ADR-3、D13–D15、§5.2、M5 | 已确认 |
| R-6 | 约束（工具链） | 2026-09-25 拍板「mise 固定独立二进制」 | `tailwindcss` 4.1.13 由 `mise.toml` 固定；xtask 校验版本；生成/校验 CSS 不需要 Node | D10、§5.2、M3 | 已确认 |
| NFR-1 | 开发效率 | 基线：CI 跑的 9 个 project 串行 122.6 min（`docs/validation/p3/m7.md:61-67`、`docs/validation/p3/m8.md:136-138`，早于 review-fixes 新增的 21 项）；vellum 80 项无耗时记录；另有 5 次 release 构建 | 用户确认（D9）：PR 上最长的浏览器 CI job（含构建）≤ 10 min；本地单示例回归层 ≤ 5 min | D2–D7、D9、§9.3 | 已确认 |
| NFR-2 | 证据保真 | P1–P3 验收依赖这些度量（`tests/browser/budgets.ts`、`loads.ts`） | 证据层保留全部度量用例、阈值与原回合数；`verify --suite` 仍跑全集；回归层 ∪ 证据层 ∪ 删除清单 = 改造前用例集 | ADR-1、§9.2 | 已知 |
| NFR-3 | 构建效率 | 实测：当前 SDK CSS 构建冷 6.6 s / 热 0.66 s，加 `source(none)` 后 0.29 s 且产物逐字节相同 | SDK CSS 只含 SDK 类；应用 CSS 压缩；两次构建产物一致（`xtask verify` 的 double build） | D11、D14、§9.5 | 已知 |
| NFR-4 | 视觉不回归 | 用户未给新阈值；沿用各计划既有门 | 去前缀前后，component-catalog 全部页面亮/暗两态的计算样式差异 0；Vellum 采用原语后，与改造前 `examples/vellum` 构建（基线孪生，A-5）在 `visual.spec.ts` 的 8 张视图上每张差异 ≤2%（沿用 `docs/plan/VELLUM.md` R-2 的门），实测值写入记录，非 0 差异逐项说明来源 | §9.4、M4/M6/M7 | 已知 |
| NFR-5 | 隔离（修订） | P2 D13（`docs/plan/P2-WASM-UI.md:116`）+ ADR-2 | 保留：无 preflight、无裸元素选择器、`dark` 绑定作用域、令牌只写作用域根；放弃：「宿主页自带 Tailwind 构建与 SDK 样式互不干扰」 | ADR-2、§7 | 已确认 |
| NFR-6 | 可访问性 | `tests/browser/p2-a11y.spec.ts`、`p2-semantics.spec.ts` 的既有口径 | 新组件有 role/可访问名、完整键盘路径；模态层 Tab 首尾循环；Toast `role=status` + `aria-live=polite` 且不进覆盖层栈 | §6、M6/M7 | 已知 |
| NFR-7 | 可维护性 | Vellum 手写 6 份监听守卫、2 份 `trap_tab`；示例直连私有 crate 的 `listener_options`/`defer`/`defer_after` 共 16 处（Vellum 14、data-workbench 1、fusion-basic 1） | 示例不再为监听/延时直连 `rustify_makepad`；同一原语只有 SDK 一份实现 | D17、M6 | 已知 |
| C-1 | 约束 | `CLAUDE.md`「Toolchain」 | 所有构建走 `mbx`；启动嵌套构建的代码显式调 `mbx` | §5.2、§9 | 已确认 |
| C-2 | 约束 | `CLAUDE.md`「Conventions」 | SDK 样式表提交入库、`xtask css --check` 防漂移；改目录后重新生成 `docs/components.md` | M3–M5、M7 | 已确认 |
| C-3 | 约束 | `xtask/src/serve.rs:33-40` | 严格 CSP：`style-src 'self'`，不得内联 `<style>`/`style=""`；组件样式走类或 CSSOM | §6 | 已确认 |
| C-4 | 约束 | `CLAUDE.md` 运行时契约；`crates/rustify-ui/src/listeners.rs:22` | 页面级监听必须带实例的 abort signal（`panic = "abort"` 不跑析构） | D17、§5.3 | 已确认 |
| C-5 | 约束 | `docs/architecture.md:65` | 受控值：控件不持有它显示的值 | ADR-4 | 已确认 |
| C-6 | 约束 | `CLAUDE.md`「Browser tests」 | Playwright 永远带 `--project`；不同时开两个 Playwright 运行；性能数字只在页内测 | §9、§12 | 已确认 |
| C-7 | 约束 | `CLAUDE.md`「Plans」「Conventions」 | 计划中文；证据进 `docs/validation/`；源码不带计划/需求编号；scripted 改写需断言命中数 | 全文、M4 | 已确认 |
| C-8 | 约束 | `CLAUDE.md`「缺服务、缺容器就停下来问」「不新开分支」 | 不擅自下载 `ref/Vellum-main` 或装未经确认的工具；改动提交在当前分支 | A-5、§11 | 已确认 |
| C-9 | 约束 | `docs/plan/VELLUM.md` ADR-3、R-2；`tests/vellum/*` 的 DOM 钩子 | Vellum 保留自有标记与 CSS；孪生视觉门 ≤2%；spec 依赖的 id/class/data-* 不变 | ADR-5、D18 | 已确认 |
| A-1 | 假设 | D10 依赖 | mise 的 `github:tailwindlabs/tailwindcss` 后端（不依赖注册表简写）能为 macOS arm64 与 linux x64 选中 v4.1.13 的对应裸二进制并以 `tailwindcss` 暴露；macOS 版产物与已验证的 linux 版逐字节一致。已知：该 release 有 `tailwindcss-{macos-arm64,macos-x64,linux-x64,linux-x64-musl,linux-arm64,linux-arm64-musl}`、`windows-x64.exe` 与 `sha256sums.txt`（本次逐个探测均 200）；mise 文档写明 `github:` 后端支持裸二进制、自动去掉 OS/arch 后缀、`bin` 可改名 | M3 开工 `mise install` 后看 `mise which tailwindcss` 与横幅；linux 若误选 `-musl` 或选择有歧义，加 `platforms` 下逐平台 `asset_pattern`；CI 的 `css --check` 在 macOS 上验证一致性。责任人：M3 实施者；最晚 M3 退出 | 开放 |
| A-2 | 假设 | ADR-6/D4 依赖 | 各示例的应用状态能在页内复位到首载状态（目前重新挂载 scope 不复位应用状态，`tests/browser/p3-faults.spec.ts:162-165`），且页内复位的单次墙钟在亚秒级 | M1 探针在 property-workbench 与 Vellum 上各测四种隔离方式从开始到可操作（ready + quiet）的墙钟与页内时间：新 context（现状）、共享 context 新页、同页 `dispose()`+`mount()`（复位成本下限）、同页新实例（对照）；M2 实现 `reset()` 后以「复位后快照 = 首载快照」用例验证可行性。责任人：M1/M2；最晚 M2 退出 | 开放 |
| A-3 | 假设 | D3 依赖 | 对确定性行为，回归层 3 回合与原 20 回合抓到同类回归；竞态/泄漏类靠证据层的原回合数 | 不做前置验证，接受该取舍；重新评估触发：证据层出现回归层复现不了的失败 | 开放（接受） |
| A-4 | 假设 | D6 依赖 | Playwright config 在 runner 进程里可按 `process.argv` 的 `--project` 只起需要的 webServer，且不影响 worker 里的 projects 定义 | M1 用 `--project=component-catalog` 起跑，确认只启动一个 server；失败则改用环境变量 `RUSTIFY_SERVERS` 显式选择 | 开放 |
| A-5 | 假设 | NFR-4 的 Vellum 部分；用户 2026-09-25「可以用 examples 中的 Vellum 作对比」 | 改造前的 `examples/vellum` release 构建可作孪生：`visual.spec.ts` 的截图流程（`prepareVisual`）对两边同样适用；`twin.ts` 目前只认 `ref/`（`tests/vellum/twin.ts:6`）且断言孪生后端为「Canvas 2D」（`tests/vellum/twin.ts:9-13`），基线是「Makepad WebGL2」，须加基线模式；`m3-raster`/`m4-pointer`/`m6-edit`/`m7-files` 里的孪生用例能否对基线运行未知 | M6 开工先接入基线模式并跑 `visual.spec.ts`；其余孪生用例逐个试跑，能跑的纳入 M6/M7 退出条件，依赖原版专有行为的记录原因并保持跳过。责任人：M6 实施者；最晚 M6 退出 | 开放 |
| A-6 | 假设 | D7 依赖 | GitHub Actions 的 macOS runner 能同时跑 4–5 个 matrix job，且 mbx 缓存对并行 job 生效 | M2 在 PR 上实跑一次看排队与缓存命中；不成立则合并 job 或改为两段 | 开放 |
| A-7 | 假设 | D9 依赖 | 本地单示例 ≤ 5 min 在 property-workbench 上可达（ADR-6 后只剩标了 `fresh` 的用例付导航成本，M1 统计其数量）；单 project `workers: 2` 不触发 `CLAUDE.md` 所说的内存被杀 | M1 探针同时测单 project 的峰值内存与 `workers: 2` 的墙钟；M2 按 D9 手段实测。责任人：M1/M2 实施者；最晚 M2 退出 | 开放 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 形态 | 三条工作线（测试 / Tailwind / Vellum 抽取）、一份计划、8 个里程碑；只改仓库工具链、SDK 两个公开 crate、示例与测试 | 零改动：`makepad/`、`crates/rustify-makepad` 的运行时语义（只新增公开出口所需的最小函数）、`web/loader.js` 的实例/中止模型 | R-1–R-6；C-1 |
| D2 | 测试分层开关 | `RUSTIFY_TIER=regression`（默认）/`evidence`/`all`；证据用例用 Playwright 标签 `@evidence`；两份 config 读同一个 `tests/tier.ts` ★ | 本地直接跑就是回归层；CI PR 跑回归层；证据 workflow 跑 `evidence`；`xtask verify` 跑 `all` | ADR-1；R-1/R-2 |
| D3 | 回合与矩阵 | `rounds(n)`：回归层 `min(n, 3)`、证据/全量层 `n`；参数矩阵用 `pick(all, representative)`：回归层取代表子集 | 20 回合循环在 PR 里降为 3；`m4-geometry` 的 9 组视口×DPR 在回归层取 2 组 | A-3；§5.1 |
| D4 | 页面复用 | ADR-6：worker 级 fixture 为每个 project 只开一页，每个用例前调示例句柄的 `reset()`；以下用例声明 `test.use({ fresh: true })` 拿同 context 的新页：冷加载/首载字节、服务器故障开关（`__fault/*`）、需要 `addInitScript` 改写环境（如拒绝 WebGL2）、深链接首载、会让实例 trap 或耗尽重启次数；用例后 fixture 检查运行时健康（`data-status`、区域与错误计数），不健康就丢掉该页，下个用例重开 | 用例改从 `tests/browser/support.ts`（Vellum：`tests/vellum/support.ts`）导入 `test`；每个示例的 `window.__<example>` 增加 `reset()`，Vellum 基于既有 `window.vellum.actions.resetStarter()` 并清空 IndexedDB/localStorage | ADR-6；A-2；`tests/browser/support.ts:511-536`；`tests/browser/p3-faults.spec.ts:162-165` |
| D5 | 去重 | §5.1「去重」表逐项执行：保留一处、其余删或移入证据层；删除项在 M2 记录里列出「由谁保留」 | 回归层不再有同一行为的多处 20 次重复 | R-2 |
| D6 | webServer 按需 | config 只为本次 `--project` 起对应 server（A-4）；两个子路径 server 不再在启动命令里 `build-web`，改为显式构建步骤，`serve` 找不到产物即失败并提示命令 | 单 project 运行不再起 6 个 server、不再每次重建 2 个子路径产物 | `playwright.config.ts:111-138`；`CLAUDE.md`「Browser tests」 |
| D7 | CI 编排 | host job 加 `--bins`；浏览器改 matrix：fusion-basic+deployment、property-workbench+workbench-deep、component-catalog+data-workbench、vellum，各自只构建需要的示例；超 D9 的 project 用 `--shard=i/n` 拆成多个 job；新增 `.github/workflows/evidence.yml` ★（`workflow_dispatch` + 每周一次） | PR 关键路径变为最慢的一个 job；证据层不再拖 PR | A-6；D9；`.github/workflows/verify.yml` |
| D8 | Vellum 测试配置 | 保留 `tests/vellum/playwright.config.ts`，接入同一分层开关；孪生来源由 `VELLUM_TWIN=original\|baseline` 选择（默认有 `ref/` 用 original），`baseline` 时 4180 端口由 `VELLUM_BASELINE_DIR` 指向的基线工作树 `mbx xtask serve --example vellum --release --port 4180` 提供 | 视口/SwiftShader 启动参数与 `VELLUM_ARTIFACT_SCOPE` 不动；没有 `ref/` 也能跑视觉门 | 改动最小；A-5 |
| D9 | NFR-1 目标 | 最长浏览器 CI job ≤ 10 min：从 job 开始到 Playwright 结束，含 `build-web`，按 mbx 缓存命中计时（冷缓存另记，不作判定）；本地单示例 ≤ 5 min：在已有 release 构建上跑该示例的主 project（`fusion-basic`、`property-workbench`、`component-catalog`、`data-workbench`，vellum 用自己的 config），不含构建。达标手段按序使用：① ADR-6 一次启动、多次检查（D4）② §5.1 分层、去重与 D3 降回合 ③ CI 用 Playwright `--shard` 把最慢的 project 拆到多个 matrix job ④ 本地单 project `workers: 2`（A-7 证明内存有余量时）⑤ 仍超标的 spec 逐个复核是否属于证据层 | 四步用尽仍不达标：M2 记为阻塞，带最慢 spec 清单向用户报告，不私自放宽目标 | 用户 2026-09-25 确认 |
| D10 | Tailwind CLI 来源 | `mise.toml` 以显式后端固定 `"github:tailwindlabs/tailwindcss" = { version = "4.1.13", bin = "tailwindcss" }`（mise 注册表不收这类工具的简写，不依赖它）；`xtask/src/tailwind.rs` ★ 解析 `RUSTIFY_TAILWIND` 或 PATH 上的 `tailwindcss`，校验横幅版本；删除 `package.json` 的 `@tailwindcss/cli`、`tailwindcss` 与 `css` 脚本 | 生成/校验 CSS 无需 `npm ci`；Node 只剩 Playwright 需要 | R-6；实测独立二进制产物与 npm CLI 逐字节一致 |
| D11 | 源扫描 | 所有 Tailwind 输入的 utilities 导入带 `source(none)`，源一律 `@source` 显式列出 | 去前缀后，仓库里任何写着 `flex`/`table` 的文字都不会再被编成规则 | 实测：当前构建会把 `docs/` 里的 `rui:` 字符串编进产物 |
| D12 | 去前缀 | 机械去掉 `rui:`（组件 src 673 处、component-catalog 42 处、data-workbench 1 处、测试 5 处——M2 删掉 `p2-catalog.spec.ts:57-74` 后剩 `p3-table.spec.ts:31` 1 处）；Tailwind 输入去 `prefix(rui)`；`@theme inline` 加 `--spacing: 0.25rem`；CSS 默认令牌值对齐 `Theme::light()` | 组件外观不变（NFR-4）；`p-4` 不再读 SDK 的 `--spacing: 8px` | ADR-2；实测 `--spacing` 冲突与修复 |
| D13 | SDK CSS 入口拆分 | 新增 `crates/rustify-components/css/sdk.css` ★（`@source "../src"`、暗色变体、`@theme inline`、`slider.css`/`controls.css`、作用域默认令牌）；`rustify.tailwind.css` 只剩 Tailwind 导入 + `@import "./sdk.css"` | 预编译产物与应用构建共用同一份 SDK 层 | 实测：被导入文件里的 `@source` 相对该文件解析 |
| D14 | 应用构建 | 示例目录有 `tailwind.css` 时，`build-web` 用 D10 的 CLI 以 `--minify` 编译到产物的 `tailwind.css`，页面链接它；否则沿用复制已提交的 `rustify.css`（`xtask/src/build.rs:137-145`） | component-catalog 首个采用（它独有 4 个类：`bg-success`、`bg-warning`、`h-24`、`h-48`）；随后 SDK 入口删掉 `@source "../../../examples"` | ADR-3 |
| D15 | 编辑器支持 | 提交 `.vscode/settings.json` ★，只含 Tailwind IntelliSense 键：`experimental.configFile` 指向 SDK 入口、`includeLanguages` 把 rust 当 html、`experimental.classRegex` 覆盖 `clx!`/`variants!`/`merge(`/`const X: &str = "..."` | 其他编辑器在 quickstart 给等价片段 | Tailwind 调查第 7 点第 10 条 |
| D16 | 抽取准入 | 同时满足：接口不含 Vellum 模型类型；SDK 无等价物（或等价物缺该能力）；抽取后仓库内 ≥2 个真实调用方（目录类别的 component-catalog 页面算一个） | §5.3 候选表逐项判定，未过的进 §0.5 | 三次再抽象；删除测试 |
| D17 | 放置 | 行为核心进 `rustify-ui`；带样式组件进 `rustify-components` | Vellum 只依赖 `rustify-ui`，不因此拿到用不上的 `rustify.css`（`xtask/src/build.rs:236-241` 按依赖名复制） | ADR-5 |
| D18 | Vellum 采用范围 | Vellum 改用行为核心，保留自有标记与 CSS；不改用带样式组件 | 孪生门与 DOM 钩子不受影响（C-9） | VELLUM ADR-3 |
| D19 | 新目录类别 | `toast`、`number field`、`color field`、`toggle group`（含工具栏）；Menu 的分组标题/分隔线/快捷键提示/视口夹取/按点打开与 Dialog 的标题栏+关闭按钮进既有类别 | `CATALOG` 20 → 24 类；`docs/components.md`、`CLAUDE.md`、`docs/architecture.md` 里的「twenty」随之更新 | D16；用户 2026-09-25 确认 |

### 0.3 ADR-lite

#### ADR-1：测试按「回归 / 证据」分层，用同一份 spec + 标签 + 回合函数实现

- 状态：Accepted（用户 2026-09-25 选「移出 PR，按需触发」）
- 背景与驱动：R-1/R-2/NFR-2。P1–P3 把「验收证据」写成了 Playwright 用例（预算 30/60 次冷热加载、2 min 长跑、185 s 空闲、20 回合故障），每个 PR 都付这份成本；同时同一行为在多处各重复 20 次（§5.1 去重表）。
- 备选：A（选）同一 spec，标签 + `RUSTIFY_TIER` + `rounds()` 决定跑什么、跑几回合；B 把证据用例拆到独立目录与独立 config——要复制 webServer 与 fixtures，且同一行为的「回归版」和「证据版」会分叉漂移；C 直接删除证据用例——用户已否决，重测需重写。
- 决策：A。
- 正面后果：一份代码两种强度；`verify` 不变语义（跑全集）；证据层随时可手动或定时触发。
- 负面/中性后果：PR 不再能发现只在 20 回合或长跑下出现的竞态/泄漏，发现延迟到下一次证据层运行（最长一周）；新增度量用例要打 `@evidence` 标签，靠评审把关（`tests/tier.ts` 的注释写明判定规则，见 §5.1）。
- 重新评估触发：证据层失败而回归层复现不了连续出现两次；或证据层单次运行超过 GitHub Actions 单 job 上限。

#### ADR-2：去掉 `rui:` 前缀，SDK 与应用共享无前缀 Tailwind v4

- 状态：Accepted（用户 2026-09-25 拍板；推翻 `docs/plan/P2-WASM-UI.md:116` 的 D13 与 ADR-5 样式隔离）
- 背景与驱动：R-4/R-5。实测 Tailwind v4 一次构建只接受一个前缀（前缀类与无前缀类同入一个构建，产物为空）；`tw_merge` 把 `rui` 当作首个变体，因此应用传给组件的无前缀 `class` 与组件类不冲突也不覆盖（`crates/rustify-components/src/macros/mod.rs:62-71`），而写 `rui:` 覆盖类又只有被 SDK 构建扫到才有规则。
- 备选：A（选）全部无前缀；B 双构建（应用无前缀 + SDK `rui:`，SDK 构建额外扫应用源）——覆盖组件样式仍须写 `rui:`，悬停等变体的特异性漏覆盖；C 单构建、应用也写 `rui:`——语义正确但应用标记冗长。
- 决策：A。配套 D11（`source(none)`，否则无前缀后会把仓库里的普通单词编成规则）、D12（`--spacing` 内联，否则所有间距 ×2）。
- 正面后果：调用方 `class="p-6"` 通过 `tw_merge` 真正替换组件的 `p-4`；应用与组件一份构建、一套 IntelliSense；类名与 Rust/UI 上游一致。
- 负面/中性后果：宿主页若自带 Tailwind 构建并同时链接预编译 `rustify.css`，两边同名 utility（如 `bg-primary`）会互相覆盖——文档改为推荐「宿主把 `sdk.css` 并入自己的构建」，不再承诺共存；`:root` 上出现无前缀的 Tailwind 主题变量；历史计划 P2 的 D13 文字保留原样，本计划记录推翻。
- 重新评估触发：出现必须嵌入第三方 Tailwind 宿主且不能改宿主构建的真实用户场景——届时考虑恢复前缀或 `@layer` + 作用域包裹。

#### ADR-3：应用 Tailwind 由 `build-web` 编译示例自带的输入文件

- 状态：Accepted
- 背景与驱动：R-5/NFR-3。当前示例自身标记全靠手写 `app.css`；component-catalog 的应用级类只因 SDK 输入扫 `examples/` 才有规则（`crates/rustify-components/css/rustify.tailwind.css:15-16`），把示例的类打进了发给所有应用的 SDK 样式表。
- 备选：A（选）约定式：示例目录有 `tailwind.css` 就由 `build-web` 编译；B 只写文档，应用作者自己跑 CLI——每个应用都要重复配置、容易忘记重编；C 新增 `xtask css --example <ex>` 子命令由人手动跑并提交产物——多一份提交产物和一道漂移检查。
- 决策：A。类名改动本来就要重编 wasm，把 CSS 编译放进同一条 `build-web` 不增加步骤；产物不入库，不需要漂移检查。
- 正面后果：零额外命令；SDK 样式表只含 SDK 类；应用 CSS 压缩且只含用到的类。
- 负面/中性后果：带 `tailwind.css` 的示例构建需要 mise 固定的二进制（不再是「构建 wasm 永远不需要额外工具」）；缺失时构建明确失败。
- 重新评估触发：出现仓库外应用（当前 `publish = false`，`Cargo.toml`）——届时需要把 `sdk.css` 路径解析（`cargo metadata`）做进工具。

#### ADR-4：「输入草稿」作为受控值契约的显式例外

- 状态：Accepted（用户 2026-09-25 确认；改变 `docs/architecture.md:65` 的运行时契约，只影响新增的数字/颜色输入，既有 `TextField` 不变）
- 背景与驱动：R-3/C-5。严格受控的 `TextField` 每次按键都回到应用决定的值（`crates/rustify-components/src/input.rs:45`），数字输入中间态（`-`、`1e`、`#1f`）无法解析就会被应用拒绝并抹掉；Vellum 为此在 `examples/vellum/src/shell/fields.rs:137-200` 自带「聚焦期间保留草稿」的实现。
- 备选：A（选）`rustify_ui::Draft`：未聚焦时显示应用值（受控）；聚焦时显示草稿，每次可解析的输入作为预览请求发给应用，应用拒绝不抹草稿；Enter/失焦提交一次、不可解析则回到应用值；Escape 撤回并请求取消预览；B 继续严格受控，由应用自己容忍中间态——每个应用重复一遍 Vellum 的逻辑；C 数字输入改用原生 `type=number`——各浏览器中间态行为不一，且颜色 hex 无对应。
- 决策：A，并在 `docs/architecture.md` 的「Controlled values」条目写明例外边界：只在聚焦的文本类输入内，失焦即回到受控。
- 正面后果：Vellum 与目录组件共用一份草稿规则；应用仍是值的唯一权威。
- 负面/中性后果：契约多一个例外，文档与测试要覆盖「聚焦时外部值变化」的规则（草稿未改动则跟随外部值，改动过则保留并在失焦时以应用值为准）。
- 重新评估触发：第三类输入（日期、时长）需要不同的草稿规则时，考虑把 `parse/format` 抽成 trait。

#### ADR-5：抽取形态——行为核心进 SDK，Vellum 保留自有标记

- 状态：Accepted
- 背景与驱动：R-3/C-9。Vellum 不依赖 `rustify-components`（VELLUM ADR-3），所有外壳组件以 `editor: Editor` 为参数并由 `.vellum` 作用域 CSS 定样式；孪生视觉门与 spec 钉住了它的 DOM。
- 备选：A（选）行为核心（监听、快捷键、Tab 循环、帧/延时、Toast 队列、草稿）进 `rustify-ui`，Vellum 调核心、保留标记；带样式组件建在同一核心上进目录；B Vellum 直接改用带样式组件并用 CSS 追平——VELLUM ADR-3 已淘汰，追不平即视觉差异；C 把 Vellum 的组件原样复制进目录——两份实现，Vellum 那份不会被删，删除测试不过。
- 决策：A。
- 正面后果：真实去重（Vellum 删掉自己的实现）；目录组件与 Vellum 在行为上同源。
- 负面/中性后果：核心接口要允许调用方自带标记（Toast 用渲染闭包，草稿返回文本信号与事件处理器），比纯样式组件多一层 API。
- 重新评估触发：用户要求 Vellum 展示组件库（VELLUM ADR-3 的触发条件）。

#### ADR-6：回归层「一次启动、多次检查」

- 状态：Accepted（用户 2026-09-25）
- 背景与驱动：R-1/NFR-1。应用本身不慢：页内冷启动 140–190 ms（`docs/reports/p3/performance.md:12-13`），挂载到首帧中位 55 ms（`docs/reports/p1/performance.md:21`），`dispose()` 7–17 ms（`docs/reports/p2/performance.md:78-79`）。慢在测试环境：每次页面导航固定约 10.9 s 墙钟，与页面做什么无关（`docs/reports/p2/performance.md:76-86`）；软件光栅下字体图集每页再阻塞约 6 s（`tests/browser/support.ts:58-64`）。现状每个用例一个新 context、一次导航——property-workbench 约 118 次，按 10.9 s 计约 21 min，超过其 37.6 min 的一半。
- 备选：A（选）每个 worker 每个 project 只开一页，用例之间用示例句柄的 `reset()` 在页内复位（卸载全部 scope → 应用状态与 URL 回到首载值 → 重新挂载并等到 ready），只有必须从页面加载开始的用例开新页；B 共享 context、每用例新页——只省编译缓存，仍付每次导航约 10.9 s；C 组件级 wasm-bindgen-test——新基础设施，且测不到 DOM/GPU 跨边界契约（§0.5）；D 每用例在同页新开一个实例（loader 按 URL 共享已编译模块，`web/loader.js:30-45`；fusion-basic 已这样起第二实例，`examples/fusion-basic/app.js:155,170`）——状态天然干净，但 loader 没有卸载实例的公开入口，旧实例的线性内存与区域会累积。
- 决策：A；B、D 的实测数字由 M1 探针给出并记入记录，作为对照。
- 正面后果：每用例固定开销从约 11 s 降到亚秒级（估算：dispose + 挂载 + 等待安静）；字体图集每页只建一次；5 min 目标主要靠这一项，而不是靠拆 CI 与多 worker。
- 负面/中性后果：用例之间不再天然隔离，复位不彻底会造成顺序相关的失败；每个示例要实现并维护 `reset()`，且 `reset()` 自身要有「复位后快照 = 首载快照」用例；trap 等弄坏实例的用例由 fixture 发现后换页，代价只在那时支付。
- 重新评估触发：复位导致的顺序性失败连续两轮回归出现且无法在 `reset()` 内修复；或某示例的应用状态无法在页内复位——该示例退回 B，其耗时单独记录并按 D9 手段处理。

### 0.4 职责与事实所有权

|  | `tests/` + CI | `xtask` | `rustify-ui` | `rustify-components` | 示例 |
| --- | --- | --- | --- | --- | --- |
| 拥有 | 用例分层归属、回合数、页面复用方式、CI 拓扑、证据层调度 | Tailwind 二进制解析与版本校验、SDK/应用 CSS 编译、`verify` 分层、子路径构建 | 监听守卫、快捷键匹配、模态 Tab 循环、帧/延时、Toast 队列、草稿规则 | 带样式组件、`sdk.css` 与预编译 `rustify.css`、目录与文档表 | 各自的快捷键表、Toast 文案、字段绑定、标记与 CSS；测试句柄 `reset()` 的实现（ADR-6） |
| 不拥有 | 预算阈值与度量方法（沿用 `budgets.ts`/`loads.ts`） | 组件类内容 | **任何带样式的标记；Vellum 规则** | 行为规则的第二份实现 | **SDK 原语的私有副本** |

一句话：阈值属于证据层、调度属于 CI；样式属于组件 crate、行为属于 SDK；示例只保留自己的表与文案。

### 0.5 明确不在本期

- **CI 换 Linux runner**——去向：D9 的五步手段用尽仍不达标时，随 M2 的阻塞报告一起请用户决定。
- **Rust 级组件 DOM 测试（wasm-bindgen-test、SSR 渲染）**——需要新基础设施；Leptos 只开了 `csr`（根 `Cargo.toml`）。
- **Vellum 改用带样式目录组件或重做其视觉基线**——VELLUM ADR-3；等用户触发。
- **未过 D16 准入的 Vellum 件**：深层/多选/拖放图层树（扩 `Tree`）、相机与平移缩放、快照撤销历史、字节预算 LRU、标尺、62 个图标、提示输入对话框、持久化——单一调用方或 Vellum 专属；三次再抽象。
- **CSS watch 模式**——类名在 wasm 里，改类必重编 wasm，`build-web` 已编译 CSS。
- **按组件裁剪 SDK 样式表**——SDK 源整体扫描；需要按组件清单时另立项。
- **手写规则分层（`slider.css`/`controls.css` 进 `@layer components` 以便调用方 utility 覆盖）**——会改变现有级联，需逐组件视觉核对，另立项。
- **`ThemePatch` 扩到 23 个令牌、`--spacing` 令牌驱动 Tailwind 间距（密度缩放）、`prefers-color-scheme` 自动暗色**——样式模型扩展，另立项。
- **改写 P1–P3、VELLUM 的历史计划与验收记录**——保持原样；本计划记录推翻 D13 的事实。

## 1. 当前事实与改动面

### 1.1 现状与缺口

**测试**

- [已核实·缺口] 每个 Playwright 用例默认新 BrowserContext，几乎每个用例都冷启动一次 wasm：fusion-basic 约 71 次、property-workbench 约 118 次、data-workbench 约 59 次；只有 10 个 p2 spec 用 `sharedPage`，没有 worker 级 fixture（`tests/browser/support.ts:511-536`）→ D4 → 不做则 R-1 无从谈起。
- [已核实·缺口] 证据类成本集中在：`p2-budget` 65 次加载、`p3-budget-minimal` 64 次、`p3-faults` 单用例 20 次加载、`p3-idle` 约 185 s 固定空闲、`m8-endurance`/`p3-endurance` 各 2 min、`p3-budget-data` 页内滚动 355 s、`m4-geometry` 约 220 张截图 → ADR-1/D3。
- [已核实·缺口] 同一行为多处重复（§5.1 去重表），另有两处浏览器断言重复 host 单测：`tests/browser/p2-catalog.spec.ts:57-74`（= `crates/rustify-components/src/lib.rs` 的 stylesheet 测试）、`tests/browser/m6-components.spec.ts:21-92`（= `crates/rustify-components/src/catalog.rs` 的目录表测试）→ D5。
- [已核实·缺口] `playwright.config.ts` 不论 `--project` 都起 6 个 webServer，其中两个每次先 `build-web --base /tools/demo/`（`playwright.config.ts:129,135`）→ D6。
- [已核实·缺口] CI 的 `mbx test --workspace --lib` 跳过没有 lib 目标的 crate：xtask（无 `xtask/src/lib.rs`）约 24 项、data-workbench 39、property-workbench 7、component-catalog 5、vellum `shaders.rs` 5 项从未在 CI 跑过 → D7。
- [已核实·缺口] Vellum 在无 `ref/` 时 18/80 项跳过（含整个视觉门），而 `examples/vellum/README.md:84` 称视觉检查「always checks」→ M2 修文档；M6 的基线孪生模式（D8）让视觉门不再依赖 `ref/`。
- [已核实·足够] 已记录的耗时基线：`docs/validation/p3/m7.md:61-67`、`docs/validation/p3/m8.md:136-138`；每次导航约 10.9 s 墙钟（`docs/reports/p2/performance.md:76`）。
- [已核实·缺口] 应用页内启动与复位都很快（ADR-6 背景所列数字），loader 按 URL 缓存已编译模块（`web/loader.js:30-45`），各示例句柄已有 `mount()`/`dispose()`（如 `examples/property-workbench/app.js:127-137`）；缺的是「应用状态复位」：重新挂载 scope 后应用持有的对象仍在（`tests/browser/p3-faults.spec.ts:162-165`）→ 各示例补 `reset()`（ADR-6）→ 不补则只能每用例开新页，D9 的 5 min 目标不可达。
- [已核实·足够] `xtask verify --suite` 逐 project 调一次 Playwright（`xtask/src/verify.rs:396-408`），且不含 `css --check`（`xtask/src/verify.rs:224-281`），而 `docs/quickstart.md:47` 说它覆盖「everything above」→ M3 补上并修文档。

**Tailwind**

- [已核实·足够·实测] Tailwind v4 一次构建只接受一个前缀：`prefix(rui)` 与无前缀导入同入一个输入，产物只剩 117 字节的层声明。
- [已核实·缺口·实测] 当前输入未写 `source(none)`，CLI 以仓库根为基准自动扫描：放进 `docs/` 的 `rui:w-[123px]` 被编进产物；冷构建 6.6 s，热 0.66 s；加 `source(none)` 后 0.29 s 且产物与已提交 `rustify.css` 逐字节相同 → D11。
- [已核实·缺口·实测] 去前缀后 `p-4` 编译为 `calc(var(--spacing) * 4)`，而 SDK 在作用域根写 `--spacing: 8px`（`crates/rustify-ui/src/theme.rs:142`、`crates/rustify-components/css/rustify.tailwind.css` 的作用域默认值）→ 所有间距翻倍；`@theme inline { --spacing: 0.25rem }` 使产物内联为 `calc(0.25rem * 4)` → D12。
- [已核实·足够·实测] 被导入文件中的 `@source "../src"` 相对该文件解析；应用输入导入库入口后同时收到库与应用的类 → D13/D14 可行。
- [已核实·足够·实测] `tailwindcss-linux-x64` v4.1.13 独立二进制在没有 `node_modules` 时产物与已提交 `rustify.css` 逐字节相同（0.97 s 含进程启动）→ D10。`--minify` 使 SDK 样式表 26,644 → 21,228 字节（gzip 4,229 → 3,994）。
- [已核实·缺口] `xtask/src/css.rs:29-35` 只找 `node_modules/.bin/tailwindcss`，缺失即报错要求 `npm ci`；`--check` 只报两个字节数（`xtask/src/css.rs:71-77`）。
- [已核实·缺口] 组件类全带 `rui:`：`crates/rustify-components/src` 673 处（198 个不同类）、`examples/component-catalog` 42 处、`examples/data-workbench` 1 处、`tests/browser/p3-table.spec.ts:31`、`tests/browser/p2-catalog.spec.ts:64-67`；Rust/UI 导入文件均已标 `rewritten`，`sources verify` 只拦「标 verbatim 却改了」与「标 rewritten 却与原件相同」（`xtask/src/sources.rs:195-223`）→ 去前缀不需改锁文件摘要。
- [已核实·缺口] CSS 默认令牌与 `Theme::light()` 不一致：`--primary`/`--ring` 为 `#2e90fa`，代码为 `0x1570ef`；`--border` 为 `#d0d5dd`，代码为 `0x858f9e`（`crates/rustify-ui/src/theme.rs:62,79,81`）；现有单测只比名字（`crates/rustify-components/src/lib.rs:170-178`）→ D12 顺带修并加值比对。
- [已核实·足够] 示例自身标记用手写语义类；把 5 个示例的类名与 Tailwind 候选求交，只有 Vellum 的 `.hidden`（`examples/vellum/app.css:155-157`，`display:none!important`，与 utility 同义）→ 去前缀不引入类名撞车（抽取范围为 `class="…"`/`class:x` 与 CSS 选择器，M4 用构建产物复核）。
- [已核实·足够] 只有 component-catalog 用了 SDK 源里没有的类（`bg-success`、`bg-warning`、`h-24`、`h-48`，`examples/component-catalog/src/main.rs`）；data-workbench 的 `flex-1` SDK 源已有 → D14 只迁 component-catalog。

**Vellum 抽取**

- [已核实·足够] Vellum 不依赖 `rustify-components`，0 个 `rui:` 类，样式全在 `examples/vellum/app.css`（1,839 行，`.vellum` 作用域）；`examples/vellum/shell-inspector.css`、`examples/vellum/text-session.css`、`examples/vellum/src/shell/layers.css` 是 `app.css:1803-1839` 的死副本，`build-web` 只复制 `app.js`/`app.css`（`xtask/src/build.rs:124`）→ M6 删除。
- [已核实·缺口] 带 Drop 的监听守卫在 Vellum 手写 6 份：`pointer.rs:178-218`、`keys.rs:9`、`fileio.rs:565`、`shell/menus.rs:157`、`shell/text_session.rs:166`、`storage.rs:708`；SDK 的 `page_level()` 是 `pub(crate)`（`crates/rustify-ui/src/listeners.rs:22`），示例只能直连 `rustify_makepad::listener_options`（Vellum 9 个文件）。
- [已核实·缺口] property-workbench 的窗口级 Escape 用 Leptos `window_event_listener`（`examples/property-workbench/src/main.rs:1409`），不带实例 abort signal，违反 C-4；⌘K 在容器上手写 `add_event_listener_with_callback` + `on_cleanup`（`examples/property-workbench/src/main.rs:1553-1580`）→ M6 改用 `listen`/`Shortcut`。
- [已核实·缺口] `Layer` 模态只做 inert，没有 Tab 首尾循环（`crates/rustify-ui/src/overlay.rs:286-293`）；Vellum 两份 `trap_tab`（`shell/dialogs.rs:381`、`shell/presentation.rs:293`）。
- [已核实·缺口] 可中止的 rAF 依赖 `app.js` 的 `window.__vellumAbortResource` 胶水（`examples/vellum/src/browser_frame.rs:8-12`、`examples/vellum/app.js:12`）；`defer`/`defer_after` 只在私有 crate（`crates/rustify-makepad/src/wasm/host.rs:131,141`）。
- [已核实·缺口] Toast 只有 Vellum 有（`examples/vellum/src/shell/toast.rs`：序号、3,200 ms 自动隐藏、Portal 到覆盖层根、不进 `Layer` 栈、`role=status`）；SDK 与目录均无。
- [已核实·缺口] 目录 `Menu` 条目只有 id/label/disabled/reason，无分组标题、分隔线、快捷键列与视口夹取（`crates/rustify-components/src/menu.rs:50`）；Vellum 与 property-workbench 的右键菜单都需要（`examples/vellum/src/shell/menus.rs:59-155`、`examples/property-workbench/src/main.rs:2107-2246`）。
- [已核实·缺口] 数字/颜色输入：Vellum `NumberField`/`ColorField`（`shell/fields.rs:137-200`、`228-271`，hex 解析 `118-135`）；目录无颜色输入，`TextField` 严格受控（ADR-4）。

### 1.2 拓扑与文件清单

```text
tests/tier.ts ★                         分层开关、rounds()、pick()
tests/browser/support.ts                 导出扩展后的 test（每 worker 一页 + reset/健康检查 fixture）
tests/browser/*.spec.ts                  打标签、rounds()/pick()、去重删改
tests/vellum/{support.ts,twin.ts,playwright.config.ts,*.spec.ts}  同一分层开关；twin.ts/config 加基线孪生模式
playwright.config.ts                     projects 读分层；webServer 按需；子路径不再现场构建
.github/workflows/verify.yml             host 加 --bins、去 npm ci（CSS）；浏览器 matrix
.github/workflows/evidence.yml ★         证据层：手动 + 每周
xtask/src/tailwind.rs ★                  CLI 解析、版本校验、compile()
xtask/src/{css.rs,build.rs,verify.rs,doctor.rs,serve.rs,main.rs}
mise.toml                                tailwindcss 4.1.13
package.json / package-lock.json         删 Tailwind 依赖与 css 脚本
crates/rustify-components/css/sdk.css ★  可组合的 SDK 层
crates/rustify-components/css/{rustify.tailwind.css,rustify.css}
crates/rustify-components/src/**         去前缀；toast.rs ★ number_field.rs ★ color_field.rs ★ toggle_group.rs ★；menu.rs、dialog.rs、catalog.rs、macros/mod.rs
crates/rustify-ui/src/{listeners.rs,overlay.rs,lib.rs}；shortcut.rs ★ frame.rs ★ toast.rs ★ draft.rs ★
crates/rustify-makepad/src/wasm/host.rs  仅在 next_frame 需要 abort signal 时加最小出口
examples/component-catalog/{tailwind.css ★,index.html,src/main.rs}
examples/*/app.js 与各自 src/          window.__<example>.reset()（ADR-6）
examples/data-workbench/src/main.rs、examples/property-workbench/src/main.rs
examples/vellum/src/**、examples/vellum/app.js；删 shell-inspector.css、text-session.css、src/shell/layers.css（均在 examples/vellum/ 下）
.vscode/settings.json ★
docs/{architecture.md,quickstart.md,components.md,workspace.md,vellum.md}、CLAUDE.md、examples/vellum/README.md
docs/validation/dx/m<n>.md ★
```

无需修改：`makepad/**`（D1）、`web/loader.js`（实例中止模型已存在，`next_frame` 复用 `listener_options` 背后的同一 signal）、`sources.lock.json`（§1.1）、`tests/browser/budgets.ts`/`loads.ts`（NFR-2）。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| `tests/tier.ts` ★ | 两份 config、两个 support.ts | `TIER: "regression"\|"evidence"\|"all"`（非法值抛错）；`rounds(n)`；`pick(all, representative)`；`EVIDENCE = "@evidence"` | 无 | 分层语义一处定义 | 进程内 | `--list` 计数（§9.2） |
| 示例句柄 `reset()` ★ | 回归层 fixture（`tests/browser/support.ts`、`tests/vellum/support.ts`） | 卸载本实例全部 scope → 应用状态、存储与 URL 回到首载值 → 重新挂载并等到 ready；幂等；实例已 trap 时抛错，由 fixture 换页 | 无 | 各示例自己的应用状态布局 | 进程内 | 每示例「复位后快照 = 首载快照」用例 |
| `xtask/src/tailwind.rs` ★ | `css.rs`、`build.rs`、`doctor.rs` | `VERSION = "4.1.13"`；`cli()`：`RUSTIFY_TAILWIND` → PATH 上 `tailwindcss`，横幅不是 `v4.1.13` 即错误并提示 `mise install`；`compile(input, output, minify)` 失败返回 stderr | `RUSTIFY_TAILWIND` 环境变量（测试与排障） | 版本漂移、找不到二进制的诊断 | 本地可替代（外部二进制） | 单测：横幅解析、`mise.toml` 与 `VERSION` 一致；`css --check` |
| `crates/rustify-components/css/sdk.css` ★ | `rustify.tailwind.css`、示例 `tailwind.css` | 不含 Tailwind 导入；`@source "../src"`；`@custom-variant dark`；`@theme inline`（19 色 + radius + `--spacing: 0.25rem`）；作用域默认令牌 = `Theme::light()` | 应用可在自己输入里覆盖 `@theme` | 令牌桥接与暗色绑定 | 进程内 | 单测：令牌名与值、无 `var(--spacing)`、无 `@layer base`、含 `source(none)` |
| `rustify_ui::listen` | Vellum ×6、property-workbench ×2 | `listen(target, event, ListenOptions, handler) -> Listener`；总是带实例 abort signal（无宿主时退回裸选项，同 `page_level()`）；`Listener` Drop 即移除；`passive`/`capture` 由选项给 | 无 | 闭包生命周期、abort、移除 | 进程内（web-sys） | 浏览器：示例的监听在 dispose/trap 后清零（`p3-instances` 已测窗口监听数回到基线） |
| `rustify_ui::shortcut` ★ | Vellum `keys.rs`、property-workbench ⌘K | `Shortcut::parse("Mod+Shift+K")`（`Mod` = macOS Meta / 其他 Ctrl）；`matches(&KeyboardEvent)`；`is_text_entry(target)`；纯函数部分 host 可测 | 无 | 平台修饰键、文本输入防误触 | 进程内 | host 单测：解析与匹配表 |
| `Layer` 模态 Tab 循环 | 所有模态 `Layer`（Dialog、CommandPalette、Vellum 对话框/演示） | 模态层内 Tab 从最后一个可聚焦元素回到第一个，Shift+Tab 反向；非模态层不变 | 无 | 可聚焦元素枚举（排除 `inert`/`disabled`/`hidden`） | 进程内 | `p2-a11y`、`m4-overlay`、Vellum `m5-shell` |
| `rustify_ui::{defer, defer_after, next_frame}` | Vellum（14 处）、data-workbench 与 fusion-basic 的 `defer` | `next_frame(f) -> FrameHandle`（`cancel()`；实例 abort 时自动取消）；`defer_after(ms, f)` 同样随 abort 失效 | 无 | abort signal 接线 | 进程内 | Vellum `m7-files` 的 trap/重启用例 |
| `rustify_ui::toast` ★ | Vellum、`rustify_components::Toaster` | `provide_toasts()`；`use_toasts().show(msg, ToastOptions { duration_ms, tone })` 递增序号；`ToastRegion` 渲染到 `overlay_root()`，`role=status`、`aria-live=polite`，不进 `Layer` 栈；自定义标记用渲染闭包 | 渲染闭包 | 序号去抖、自动隐藏计时 | 进程内 | host：队列状态机；浏览器：Vellum 与目录各一 |
| `rustify_ui::Draft<T>` ★ | Vellum `NumberField`/`ColorField`、目录 `NumberField`/`ColorField` | ADR-4 规则；`Draft::new(value: Signal<T>, format, parse)`；返回显示文本信号与 `on_focus/on_input/on_keydown/on_blur` 处理器；预览与提交通过调用方回调发出请求 | `format`/`parse` 函数 | 聚焦期间外部值变化、非法中间态 | 进程内 | host：状态机表驱动测试 |
| 目录新组件 | component-catalog；property-workbench（Menu 扩展） | 受控 `Signal` + 请求回调，同既有目录约定（`crates/rustify-components/src/button.rs`）；`class` 经 `macros::merge`；`test_id` | 无 | 样式与 ARIA | 进程内 | 目录单测 + component-catalog 回归 spec |

删除测试：`listen`、`Draft`、`toast` 删除后复杂度会散回 ≥2 个调用方（Vellum 与 property-workbench / 目录组件），保留；`tests/tier.ts` 删除后分层判断会散进 50+ 个 spec，保留。没有为「将来第二个实现」预留的接缝；`RUSTIFY_TAILWIND` 是唯一环境接缝，只用于测试与排障。

## 3. 数据模型与迁移

不涉及持久化与数据迁移。唯一「迁移」是类名去前缀（§5.2 第 2 步），属源码机械改写，回滚即回退该里程碑的提交。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| Tailwind CLI 4.1.13 | 已核实足够（实测） | 单前缀、`source(none)`、`@source` 相对导入文件、`@theme inline` 内联、`--minify` 均在本次实测 | D10–D14 | 二进制缺失/版本不符 → xtask 报错退出，不产出 |
| mise `github:` 后端 + Tailwind release 裸二进制 | 已核实足够（文档与资产探测）/ 未核实（实装，A-1） | mise 官方仓库（`jdx/mise`）的 github 后端文档：裸二进制、`bin`、`platforms.*.asset_pattern`；v4.1.13 各平台资产存在 | M3 开工 `mise install` 核实 | `mise install` 失败即停在 M3，向用户报告（C-8） |
| Playwright 1.63 | 已核实缺口 | 标签/`grep`/`grepInvert` 是该版本能力；argv 过滤 webServer 未验（A-4） | D2/D6 | argv 方案失败 → 环境变量显式选择 |
| `tw_merge` 0.1.21（`variant` 特性） | 已核实足够 | 无前缀时冲突判定即 Tailwind 原义（`macros/mod.rs:31-59` 的前缀用例同构） | 去掉前缀相关用例，加「调用方无前缀类替换组件类」用例 | — |
| 实例 abort signal | 已核实足够 | `rustify_makepad::listener_options()` 取实例 signal（`crates/rustify-makepad/src/wasm/host.rs:154`），loader 在 trap 时 abort | `listen`/`next_frame`/`defer_after` 复用 | 无宿主时退回裸选项（与 `page_level()` 同） |
| GitHub Actions macOS runner | 未核实阻塞（A-6，仅 M2） | — | M2 在 PR 上实跑 | 排队过久 → 合并 job |
| Vellum 孪生（`ref/` 原版 / `examples/vellum` 基线） | 已核实缺口：`ref/` 不存在，`twin.ts` 只认原版且断言 Canvas 2D 后端 | `tests/vellum/twin.ts:6-13`、`tests/vellum/playwright.config.ts:34-40` | D8/A-5：加基线模式 | 两者都没有时孪生用例照旧跳过并在记录里写明 |

新增登记点：环境变量 `RUSTIFY_TIER`、`RUSTIFY_TAILWIND`（`CLAUDE.md` Commands 节与 `docs/quickstart.md`）；Playwright 标签 `@evidence`（`tests/tier.ts`）；目录类别 4 个（`crates/rustify-components/src/catalog.rs` + `docs/components.md`）；SDK 公共导出（`crates/rustify-ui/src/lib.rs`）。

## 5. 核心机制

### 5.1 测试分层

**判定规则**：用例测的是「行为是否正确」→ 回归层；测的是「数字是否达标/是否随时间漂移」（百分位、字节数、内存尾部、帧率、长跑、空闲 CPU）或需要外部参照/有头浏览器 → 证据层。

| 归属 | spec（`tests/browser` / `tests/vellum`） | 处理 |
| --- | --- | --- |
| 证据层（整文件） | `p2-budget`、`p3-budget-minimal`、`p3-budget-data`、`m8-baseline`、`m8-endurance`、`m8-network`、`p3-endurance`、`p3-idle`、`p3-memory`、`p3-diagnostics`、`p3-probes`、`p3-faults`；vellum `smoke`、`visual`、`m8-metrics` | 文件级 `@evidence`；预算类 project（`budget`、`budget-minimal`、`budget-data`）只在证据层存在 |
| 仅手动 | `p3-budget-scene`（有头 + 真 GPU）；vellum `m3-webgpu`、`m8-ui`（已有环境变量门） | 保持现状，列入证据 workflow 的说明但不自动跑 |
| 证据层（单用例） | `m2-runtime` 的 300+100 回合挂载泄漏（`m2-runtime.spec.ts:195`）；vellum `m4-pointer` 平移 30 样本 p95（`m4-pointer.spec.ts:217`）；`p3-jobs` 的 20 次取消 p95 | 用例级 `@evidence` |
| 回归层（降回合） | `m1-probes`、`m3-state`、`m3-workbench`、`m4-geometry`（9 组 → 2 组）、`m4-overlay`、`m6-async`、`m6-components`、`m6-theme`、`m7-recovery`、`p2-*`（`p2-budget` 除外）、`p3-scene`、`p3-table` | 循环次数改 `rounds(n)`，矩阵改 `pick()` |
| 回归层（原样） | `m2-runtime`（除上行的单用例）、`m5-*`、`m6-mainpath`、`m7-deployment`、`p3-b0`、`p3-instances`、`p3-jobs`（除上行的单用例）、`p3-policy`、`p3-restart`；vellum `m2-scene`、`m3-raster`、`m4-pointer`（除上行的单用例）、`m5-shell`、`m6-edit`、`m7-files` | 只换页面复用方式（D4） |

`p3-faults` 整体进证据层的依据：它是四类故障各 20 次的放大版，单次版本已在回归层：过期答复 `m6-async.spec.ts:51-61`、上下文丢失 `m7-recovery.spec.ts:66-107`、动作内关闭 `m3-workbench.spec.ts:350-379`、字体缺失 `m7-deployment.spec.ts:73-120`。

**去重表**（保留处 ← 删除/移出处）：

| 行为 | 保留（回归层） | 删除或移入证据层 |
| --- | --- | --- |
| 样式表无 preflight/裸选择器 | host 单测 `crates/rustify-components/src/lib.rs` | 删 `p2-catalog.spec.ts:57-74` |
| 目录表 20×9 无空格 | host 单测 `crates/rustify-components/src/catalog.rs` | 删 `m6-components.spec.ts:21-92` 中的表格断言（保留两半都能画的浏览器部分） |
| 表格定位器 14×30 | `p3-table.spec.ts:279-312`（`rounds(30)`） | 删 `p3-scene.spec.ts:508-515` 的表格定位器段 |
| 1,000 次跨区域动作 | —（纯度量） | `p2-budget` R30、`p3-diagnostics`、`m8-baseline` 均在证据层 |
| 挂载/释放泄漏循环 | `m1-probes.spec.ts:104`（`rounds(20)`） | `m2-runtime:195`、`m8-baseline:102`、`p3-memory:108` 在证据层 |
| 隐藏/恢复 ×100 | —（纯度量） | `m8-baseline:150`、`p3-idle:206` 在证据层 |
| Vellum 36 项冒烟 | m2–m7 各自的用例 | `smoke.spec.ts` 在证据层 |

**页面复用**（ADR-6/D4）：`tests/browser/support.ts` 导出 `test = base.extend(...)`：worker 级 fixture 为当前 project 打开一页并等到 ready + quiet；测试级 `page` fixture 在用例前调 `window.__<example>.reset()` 并等安静，用例后检查健康（`data-status`、区域与错误计数），不健康就关掉该页、下一个用例重开。`test.use({ fresh: true })` 的用例拿到同 context 的新页（D4 列出的五类）。`tests/vellum/support.ts` 同样处理。既有 `sharedPage`（`tests/browser/support.ts:511-536`）由新 fixture 取代。M2 记录逐 spec 写明哪些用例标了 `fresh` 及原因，并统计每个 project 剩下的导航次数。

**webServer 按需**（D6）：config 从 `process.argv` 解析 `--project`，映射到所需 server；不带 `--project`（CLAUDE.md 已禁止）时保持全起。子路径 server 命令改为只 `serve`；`xtask serve` 在 `target/makepad-wasm-app/<profile>/<example>@tools-demo` 不存在时退出并打印 `mbx xtask build-web --example <ex> --release --base /tools/demo/`。

**CI 拓扑**（D7）：

```text
PR / push:  host ─┐
            web-fusion      (fusion-basic, deployment)            ┐
            web-workbench   (property-workbench, workbench-deep)  ├ 并行，各自 build-web 所需示例
            web-catalog     (component-catalog, data-workbench)   │
            web-vellum      (vellum)                              ┘
evidence.yml（workflow_dispatch + 每周）: RUSTIFY_TIER=evidence，逐 project 串行（预算与长跑不与其他负载同机并行，P3 D13）
```

`xtask verify --suite` 设 `RUSTIFY_TIER=all`、并调 `css --check` 与 `catalog --check`；子路径产物在浏览器步骤前构建。

### 5.2 Tailwind v4 管线

去前缀之后的目标结构：

```text
crates/rustify-components/css/sdk.css   ← @source "../src"; @custom-variant dark; @theme inline {...; --spacing: 0.25rem}
         ▲                                  @import slider.css, controls.css; [data-rustify-scope]{默认令牌}
         │ @import
         ├── css/rustify.tailwind.css    ← theme.css layer(theme) + utilities.css layer(utilities) source(none)
         │        └─ mbx xtask css  → css/rustify.css（提交；--check 防漂移；无 Tailwind 的应用链接它）
         └── examples/<ex>/tailwind.css  ← 同样两行 Tailwind 导入 [+ 应用自选 preflight] + @source "./src"
                  └─ mbx xtask build-web → <out>/tailwind.css（--minify；不提交；页面链接它）
```

执行顺序：

1. **M3 工具链**：`mise.toml` 加 `"github:tailwindlabs/tailwindcss" = { version = "4.1.13", bin = "tailwindcss" }`（A-1）；`xtask/src/tailwind.rs` 解析与校验；`css.rs` 改用它，`--check` 失败时额外打印首个差异行；两份输入加 `source(none)`（产物不变，已实测）；删除 `package.json` 里的 Tailwind 依赖与 `css` 脚本并更新 lock；`doctor` 报 Tailwind 版本；CI host job 的 CSS 检查不再需要 `npm ci`；`verify` 加 `css --check`/`catalog --check`。此步仍保留 `@source "../../../examples"` 与 `prefix(rui)`。
2. **M4 去前缀**：脚本去掉 `rui:`，按文件断言命中数（§1.1 的 673/42/1/5；`cargo fmt` 之后再核一次无残留，C-7）；输入去 `prefix(rui)`，`@theme inline` 加 `--spacing: 0.25rem`；作用域默认令牌改为 `Theme::light()` 的值；`macros/mod.rs` 删掉前缀相关用例、新增「调用方 `p-6` 替换组件 `p-4`」「调用方 `hover:bg-x` 替换组件同变体类」；stylesheet 单测加：无 `.rui\:` 选择器、无 `var(--spacing)`、默认令牌值与 `Theme::light()` 相等；`tests/browser/p3-table.spec.ts:31` 与剩余 `p2-catalog` 引用同步。
3. **M5 应用侧**：抽出 `sdk.css`；`build-web` 编译示例 `tailwind.css`；component-catalog 新增 `tailwind.css` 并改链接；SDK 入口删掉 `@source "../../../examples"`（产物少 4 个类）；`.vscode/settings.json`；文档：`docs/quickstart.md` 新节「Using Tailwind v4 in your app」（输入模板、可用令牌 utility、`dark:` 语义、`class` 覆盖规则、宿主已有 Tailwind 时的并入方式）、`docs/architecture.md` 的样式模型、`CLAUDE.md` Conventions 的「`rui:` prefix」条目。

### 5.3 Vellum 抽取

**候选判定**（D16）：

| 候选 | Vellum 位置 | 其他调用方 | 判定 | 去向 |
| --- | --- | --- | --- | --- |
| 作用域监听守卫 | 6 处（§1.1） | property-workbench ×2 | 准入 | `rustify_ui::listen`（M6） |
| 快捷键匹配 + 文本输入防护 | `keys.rs:67-295` 的防护与解析部分（按键表留在 Vellum） | property-workbench ⌘K | 准入 | `rustify_ui::shortcut`（M6） |
| 模态 Tab 循环 | `dialogs.rs:381`、`presentation.rs:293` | 目录 Dialog、CommandPalette | 准入 | `Layer`（M6） |
| 可中止帧/延时 | `browser_frame.rs`、Vellum 14 处直连 | data-workbench、fusion-basic 的 `defer` | 准入 | `rustify_ui::{defer, defer_after, next_frame}`（M6） |
| Toast | `shell/toast.rs` | 目录 `toast` 类别 | 准入 | 核心 M7 + 目录组件 M7 |
| 草稿数字输入 / hex 颜色输入 | `shell/fields.rs:118-271` | 目录 `number field`、`color field` | 准入 | `Draft` + 两个目录类别（M7） |
| 切换组 / 工具栏 | `toolbar.rs`、检查器分段按钮 | 目录 `toggle group` | 准入 | 目录类别（M7）；Vellum 不采用（D18） |
| Menu 分组/分隔/快捷键/夹取/按点打开 | `shell/menus.rs:59-312` | property-workbench 右键菜单、component-catalog | 准入 | 扩 `Menu`（M7）；Vellum 不采用（D18） |
| Dialog 标题栏 + 关闭按钮 | `shell/dialogs.rs:81-119` | component-catalog、fusion-basic（裸 `Layer` 3 处） | 准入 | 扩 `Dialog`（M7） |
| 提示输入对话框、图层树、相机、历史、LRU、标尺、图标集、持久化 | — | 无或语义不同 | 不准入 | §0.5 |

**Vellum 改用顺序**：监听守卫 → 快捷键防护 → Tab 循环（删两份 `trap_tab`）→ 帧/延时（删 `browser_frame.rs` 与 `app.js` 的 `__vellumAbortResource`）→ Toast 核心 → 草稿核心。每步后跑对应 Vellum 回归 spec 与基线孪生 `visual.spec.ts`（§9.4）。

**property-workbench 改用**：`main.rs:1409` 换成 `listen(window, "keydown", …)`；`main.rs:1553-1580` 换成 `listen` + `Shortcut::parse("Mod+K")`；右键菜单用 Menu 分隔线与快捷键提示（M7）。

## 6. 前端与交互

- **新目录组件的状态**：Toast（出现/替换/自动隐藏/手动关闭，`tone` 仅影响样式）；NumberField/ColorField（未聚焦、聚焦草稿、非法草稿 `aria-invalid`、提交被拒回到应用值、disabled/read-only 不发请求）；ToggleGroup（单选/多选、`aria-pressed`、方向键漫游，复用 `crates/rustify-components/src/roving.rs`）；Menu 扩展（标题与分隔线不可聚焦、快捷键列 `aria-keyshortcuts`、超出视口时夹取）；Dialog 标题栏（关闭按钮有可访问名，Escape 与按钮走同一关闭请求）。
- **样式**：全部无前缀 utility + 已有令牌；严格 CSP 下定位用 CSSOM（C-3）；暗色只靠令牌与作用域 `dark:`。
- **i18n**：SDK 自带文案（如 Toast 关闭按钮、Dialog 关闭按钮的可访问名）进 SDK 文案目录（`docs/i18n.md` 的既有机制），应用文案由应用提供。
- **Vellum**：标记、id、class、`data-*` 不变；允许新增的只有 ARIA 属性。
- **真实浏览器走查**：component-catalog 四个新类别页亮/暗两态各走一遍键盘路径；环境不可用时记为未验证。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| NFR-1 | 122.6 min 串行（§0.1） | CI 最长 job ≤ 10 min；本地单示例 ≤ 5 min（D9） | 按 D9 手段顺序处理；用尽仍超标则 M2 阻塞并报告 | D2–D7、D9 | M2/M8：每个 project 回归层墙钟、CI 各 job 墙钟（温缓存） |
| NFR-2 | 改造前 `--list` | 守恒 | 计数不守恒即 M1/M2 不退出 | ADR-1 | §9.2 计数脚本 |
| NFR-3 | 0.29 s（`source(none)`） | SDK 产物只含 SDK 类；应用产物压缩；double build 一致 | 版本不符即失败 | D10/D11/D14 | `css --check`；`verify` double build |
| NFR-4 | 现有外观 | 计算样式差异 0；Vellum DOM 仅增 ARIA | 差异非 0 即回查 D12 | D12/D18 | §9.4 |
| NFR-5 | P2 D13 | 保留三条隔离，放弃宿主 Tailwind 共存 | — | ADR-2 | host 单测（无 preflight/裸选择器/作用域 dark）；`p2-catalog` 的宿主外观用例保留 |
| NFR-6 | 既有 a11y 口径 | 新组件 role/名/键盘；模态 Tab 循环 | — | §6 | component-catalog 新 spec；`p2-a11y` |
| NFR-7 | 16 处私有 crate 直连（§0.1） | 示例对 `rustify_makepad::{listener_options, defer, defer_after}` 的直接引用为 0 | — | D17 | `rg` 检查写入 M6 记录 |

安全：本期不引入网络、鉴权与数据面变化；严格 CSP 不放宽（C-3），Tailwind 产物不含内联样式；`listen` 保证 trap 后监听随实例中止（C-4），这是本期唯一与运行时安全相关的行为，由 `p3-instances` 的窗口监听基线用例覆盖。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 证据层长期不跑，度量回归潜伏 | 预算/内存/长跑 | 无 | PR 全绿但性能退化 | 每周定时 run 的 GitHub 通知 | 手动触发 evidence workflow 二分 | M2 手动触发一次 |
| 共享页面上复位不彻底，用例相互污染 | 单 project | 无 | 顺序相关的偶发失败 | 每示例的复位等价用例；连续两次全量回归结果一致 | 修 `reset()`；修不了的用例标 `fresh` 并在记录说明 | M2 退出条件 |
| 回归层超 D9 目标 | CI 时长 / 本地迭代 | 无 | PR 等待变长 | CI job 时长；本地 JSON reporter | 按 D9 手段顺序：页面复用 → 分层去重降回合 → `--shard` → 本地 `workers: 2` → 逐 spec 复核归属；用尽即阻塞并报告 | M2 记录 |
| Tailwind 二进制缺失或版本不符 | 构建 | 无 | `css`/`build-web` 失败并提示 `mise install` | xtask 错误 | `mise install` | M3 单测 + 手动去掉 PATH 复现 |
| 自动扫描被重新打开（漏写 `source(none)`） | 所有应用样式 | 无 | 产物混入无关规则 | stylesheet 单测检查输入含 `source(none)`；`css --check` 差异 | 补回 | M3/M5 单测 |
| `--spacing` 内联丢失 | 所有间距 | 无 | 组件间距翻倍 | 单测「无 `var(--spacing)`」；计算样式对比 | 补回 `@theme inline` | M4 |
| 宿主页自带 Tailwind 又链接 `rustify.css` | 嵌入场景 | 无 | 同名 utility 相互覆盖 | 无自动检测（ADR-2 接受） | 按文档把 `sdk.css` 并入宿主构建 | 文档审阅 |
| 模态 Tab 循环改变既有焦点路径 | 所有模态 | 无 | 焦点顺序变化 | `p2-a11y`、`m4-overlay`、`p2-semantics` | 修循环的可聚焦元素判定 | M6 |
| Vellum 采用原语后行为或 DOM 变化 | Vellum | IndexedDB 保存路径不变 | 快捷键/菜单/文本会话异常 | Vellum 回归 spec + 基线孪生视觉对比 | 回退该步提交 | M6/M7 |

- 发布顺序：按里程碑合入当前分支；M4 是破坏性改动（类名全变），与其依赖的测试字符串同一提交。
- 回滚：每个里程碑独立可回退；M4 回退需同时回退 M5/M7（它们建立在无前缀类上）；无数据与外部契约需要补偿（`publish = false`，无仓库外使用者）。

## 9. 验证

### 9.1 行为与接口验证

- host 单测：`tests/tier.ts` 无需单测（由 `--list` 计数覆盖）；`xtask/src/tailwind.rs` 横幅解析与 `mise.toml` 一致性；`shortcut` 解析/匹配表；`Draft` 状态机（未聚焦跟随、聚焦草稿、非法中间态、提交被拒、Escape、聚焦时外部值变化两种情形）；Toast 队列（序号、覆盖、自动隐藏时限）；`macros::merge` 无前缀覆盖；stylesheet 令牌名与值、无 `var(--spacing)`、含 `source(none)`、无前缀选择器残留。
- 浏览器（回归层）：component-catalog 四个新类别页各一个 spec 块（键盘、ARIA、受控/草稿语义、亮暗）；property-workbench 右键菜单分隔线/快捷键提示；窗口 Escape 在 trap 后不再触发（`p3-instances` 同型断言）。

### 9.2 集成与回归

- **计数守恒**：M1 前后各跑一次两份 config 的 `npx playwright test --list`（`RUSTIFY_TIER=all` 与改造前比），结果 = 改造前集合 − 删除清单；删除清单逐条写「由谁保留」。
- **复位等价**：每个示例一条回归用例——首载后取应用状态快照（示例句柄的 snapshot/diagnostics），做一组改动后调 `reset()` 再取快照，两者相等；Vellum 另核对 IndexedDB/localStorage 已清空、URL 回到入口。
- 每个 project 回归层：`npx playwright test --project=<p>`（一次一个，C-6）；Vellum：`npx playwright test -c tests/vellum/playwright.config.ts`。
- 证据层：`RUSTIFY_TIER=evidence npx playwright test --project=<p>` 逐 project；长跑与预算不在其他负载后立即运行（§12）。
- host：`mbx test --workspace --lib --bins`；`(cd makepad && mbx test)`；`(cd makepad && mbx test --manifest-path platform/script/Cargo.toml)`；`mbx clippy --workspace --all-targets -- -D warnings`；`cargo fmt --all -- --check`。
- 全量：`mbx xtask verify --suite p3`（改造后含分层全集、CSS/目录检查、子路径预构建）。

### 9.3 NFR 与故障注入

- NFR-1：M1 探针（A-2）与 M2 的每 project 回归层墙钟用 JSON reporter 记录；CI 各 job 墙钟取 Actions 运行记录。页内数字仍只在页内测（C-6）。
- 故障注入：去掉 PATH 上的 `tailwindcss` 跑 `mbx xtask css` 与 `build-web --example component-catalog`，确认错误信息；把版本改成 4.1.12 的假二进制（`RUSTIFY_TAILWIND` 指向打印旧横幅的脚本）确认拒绝。

### 9.4 真实环境与 UI

- **计算样式对比（M4）**：去前缀前用 release 构建跑一次探针脚本，逐页收集 component-catalog 每个类别页亮/暗两态 `[data-rustify-scope]` 下全部元素的 `getComputedStyle` 快照；去前缀后重跑并比对，差异 0。脚本与结果存 `docs/validation/dx/`，不进回归层。
- **Vellum 基线孪生（M6/M7）**：M6 开工时记录 HEAD 为基线 SHA（M1–M5 不改 Vellum 源码与产物，基线即改造前的 Vellum；M7 沿用同一基线，保证累计不回归）。在仓库外建 `git worktree`（如 `../rustify-ui-vellum-baseline`）检出基线并 `mbx xtask build-web --example vellum --release`；以 `VELLUM_TWIN=baseline VELLUM_BASELINE_DIR=<工作树> npx playwright test -c tests/vellum/playwright.config.ts visual.spec.ts` 对比 8 张视图（6 张启动页亮/暗 + 主菜单 + 帮助），每张 ≤2%，实测值写入记录；同一渲染器对同一文档，预期接近 0，非 0 差异逐项说明。A-5 判定可跑的其余孪生用例一并运行。
- component-catalog 新类别页人工键盘走查（§6）。

### 9.5 静态校验

`cargo fmt --all -- --check`；`mbx clippy --workspace --all-targets -- -D warnings`；`mbx xtask css --check`；`mbx xtask catalog --write docs/components.md --check`；`mbx xtask sources verify`；`mbx xtask doctor`；计划自检 `check_paths`/`check_progress`（M8）。

## 10. 里程碑

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 测试分层骨架与探针 | 无 | `tests/tier.ts`；两份 config 读分层；§5.1 证据类打 `@evidence`；预算 project 只在证据层；webServer 按需与子路径预构建（D6）；CI host job 改 `--lib --bins`；`verify` 设 `RUSTIFY_TIER=all`；A-2 四种隔离方式耗时探针（只用既有句柄，不改示例代码）、A-4、A-7 探针。单人负责 `playwright.config.ts`、`tests/tier.ts`、`xtask/src/serve.rs`、`xtask/src/verify.rs`、`.github/workflows/verify.yml`；可与 M3 并行（边界见表下说明） | `--list` 计数守恒（§9.2）；`--project=component-catalog` 回归层全绿且只起一个 server；`mbx test --workspace --lib --bins` 通过；四种隔离方式的墙钟与页内时间、各 project 需 `fresh` 的用例数、A-4/A-7 结论写入记录；回写「实施进度」 |
| M2 | 回归层瘦身与 CI 重排 | M1、A-2 结论 | `rounds()`/`pick()` 落到 §5.1 表；ADR-6/D4：support fixture 与各示例 `reset()`（`examples/*/app.js` 与各自 `src/`，按示例分给 subagents）及复位等价用例；去重表执行；Vellum `smoke`/`visual`/`m8-metrics` 进证据层；CI matrix（D7）与 `evidence.yml`；修 `examples/vellum/README.md:84`；更新 `CLAUDE.md`「Browser tests」与 `docs/quickstart.md` 的测试命令。spec 按 project 分给 subagents（各自只改自己 project 的 spec，`support.ts` 与 config 归主 agent）；浏览器验证串行 | 每个示例的复位等价用例通过；每个 project 回归层连续两次全绿；四个示例主 project 与 vellum 的本地回归层墙钟均 ≤ 5 min（D9 口径）；删除清单与守恒计数；证据层逐 project 各跑一次全绿（`budget-scene` 除外）；PR 上 CI matrix 全绿且最长 job ≤ 10 min（温缓存；冷缓存时长另记，A-6）；回写「实施进度」 |
| M3 | Tailwind 工具链：mise 二进制与精确扫描 | 无（CI 改动在 M1 之后） | A-1 核实；`mise.toml`；`xtask/src/tailwind.rs`；`css.rs` 改用并改进 `--check` 输出；`source(none)`；删 npm Tailwind 依赖；`doctor`；`verify` 加 CSS/目录检查；修 `docs/quickstart.md:47`。单人负责 `xtask/src/{tailwind,css,doctor,main}.rs`、`mise.toml`、`package*.json`；`verify.rs`/`verify.yml` 在 M1 合入后再改 | `mbx xtask css --check` 无漂移（无 `node_modules`）；新单测通过；缺二进制/错版本两种故障注入给出预期错误；`mbx xtask doctor` 显示版本；CI host job 不再为 CSS 跑 `npm ci` 且全绿；回写「实施进度」 |
| M4 | 去掉 `rui:` 前缀 | M2、M3 | §5.2 第 2 步；改前先跑 §9.4 计算样式快照。组件 crate、component-catalog、data-workbench、两处测试字符串同一提交，单人负责（机械改写不拆） | 命中数断言与残留为 0；host 单测（含新增的覆盖、令牌值、无 `var(--spacing)`）通过；`css --check`；计算样式差异 0；component-catalog、property-workbench、data-workbench 回归层全绿；回写「实施进度」 |
| M5 | 应用侧 Tailwind v4 | M4 | `sdk.css` 拆分；`build-web` 编译示例 `tailwind.css`；component-catalog 采用并删除 SDK 的 examples 源；`.vscode/settings.json`；quickstart/architecture/`CLAUDE.md` 样式章节。`xtask/src/build.rs` 与 CSS 文件单人负责，文档可并行 subagent | 全部 5 个示例 `build-web --release` 成功；`mbx xtask verify --suite p3 --no-browser` 的 double build 一致；`css --check`（SDK 产物少 4 个类）；component-catalog 回归层全绿；新增「调用方类覆盖组件类」浏览器断言通过；文档路径检查通过；回写「实施进度」 |
| M6 | Vellum 抽取（一）：SDK 行为原语 | M2（可与 M4/M5 并行：文件集不相交，浏览器验证与 M4/M5 串行排队） | `listen`、`shortcut`、`Layer` Tab 循环、`defer`/`defer_after`/`next_frame`；Vellum 按 §5.3 顺序改用并删除 `trap_tab`、`browser_frame.rs`、`__vellumAbortResource`、三份死 CSS；property-workbench 两处改用；data-workbench、fusion-basic 的 `defer` 改用 SDK 出口；开工先接入 Vellum 基线孪生模式（D8：`twin.ts`、`tests/vellum/playwright.config.ts`）并按 A-5 试跑孪生用例；`evidence.yml` 的手动触发加可选输入 `vellum_baseline`（commit SHA），给出时构建该 commit 的 Vellum 作孪生。SDK 原语与 Vellum 改用可拆给两个 subagent（先定接口，SDK 先合） | host 单测（shortcut 表）；Vellum 回归层全绿；property-workbench、component-catalog、fusion-basic 回归层全绿（`Layer` 变化影响面）；基线孪生 `visual.spec.ts` 8 张视图每张 ≤2% 并记录实测值（§9.4）；示例对私有出口的直连为 0（NFR-7）；回写「实施进度」 |
| M7 | Vellum 抽取（二）：目录新组件 | M4、M6 | Toast 核心与组件、`Draft` 与 NumberField/ColorField、ToggleGroup、Menu 与 Dialog 扩展；`CATALOG` 24 类与 `docs/components.md`；component-catalog 四个新页与 spec；Vellum 改用 Toast/草稿核心；property-workbench 右键菜单改用扩展；`docs/architecture.md` 受控值例外。四个组件可分给 subagents（各自文件；`catalog.rs`、`lib.rs` 导出与 component-catalog `main.rs` 归主 agent） | host 单测（Draft、Toast、目录）；`css --check`、`catalog --check`；component-catalog、property-workbench、Vellum 回归层全绿；基线孪生 `visual.spec.ts` 每张 ≤2%（同 M6 基线）；新类别页人工键盘走查记录；回写「实施进度」 |
| M8 | 收尾：全量验收与文档 | M1–M7 | 全部回归层 + 证据层各一轮；`verify --suite p3`；`docs/vellum.md` 写明 Vellum 现在用了哪些 SDK 原语；`docs/architecture.md` 测试分层节；`CLAUDE.md` 同步（命令、twenty→twenty-four、前缀、mise Tailwind）；计划自检 | 每个 project 回归层与证据层全绿；`mbx xtask verify --suite p3` 自动项 0 失败；CI（PR matrix + 一次手动 evidence）全绿；NFR-1 实测对照 D9 写入记录；`check_paths`/`check_progress` 通过；回写「实施进度」 |

并行说明：M1 ∥ M3——M1 负责 `tests/`、`playwright.config.ts`、`xtask/src/serve.rs`，M3 负责 `xtask/src/{tailwind,css,doctor,main}.rs`、`mise.toml`、`package*.json`；两者都要改的 `xtask/src/verify.rs` 与 `.github/workflows/verify.yml` 先 M1 后 M3；M6 ∥ M4/M5 的代码编写（`crates/rustify-ui` + `examples/vellum`、`examples/property-workbench` 与 `crates/rustify-components` + `xtask` 不相交），但任何时刻只有一个 Playwright 运行、测时长时不并行跑 cargo（C-6、§12）。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| D9 目标偏紧（A-7）：property-workbench 今 137 项 37.6 min，本地 ≤ 5 min 约需 7.5 倍压缩；约 118 次冷加载按每次约 10.9 s 计已占约 21 min，ADR-6 后只剩 `fresh` 用例付这笔 | M2 能否退出 | M1 探针量化 A-2/A-7；M2 按 D9 手段顺序推进，用尽即阻塞报告 | M2 退出 | 否（手段明确；未达标时的处理已约定） |
| A-1 mise `github:` 后端选资产 | M3 工具来源 | M3 开工 `mise install` 核实；歧义时逐平台 `asset_pattern` | M3 | 否（有备选） |
| A-2 页内复位可行性与成本 | ADR-6 能否成立 | M1 耗时探针；M2 复位等价用例 | M2 退出 | 否（不可复位的示例按 ADR-6 触发条件退回 B） |
| A-5 基线孪生的覆盖面 | 哪些孪生用例能对 `examples/vellum` 基线运行 | M6 开工逐个试跑 | M6 | 否（`visual.spec.ts` 只依赖截图流程） |
| A-6 CI 并行容量 | D7 形态 | M2 实跑 | M2 | 否 |
| 证据层最长一周的发现延迟 | 性能回归晚发现 | ADR-1 接受；里程碑退出仍跑证据层 | — | 否 |

- 最终状态：**Ready**。
- 定级理由：范围、决策与验证闭环；D9 目标值、ADR-4、D19 已于 2026-09-25 由用户确认；其余开放项（A-1/A-2/A-4–A-7）都有核实时点与备选，不影响开工；无安全、数据或外部契约未决项。

## 12. 已知坑与历史教训

- 不带 `--project` 跑全套会因内存被杀（`CLAUDE.md`「Browser tests」、`docs/plan/P2-WASM-UI.md:529`）→ 所有命令逐 project。
- 两个 Playwright 运行同时开会互相清空 `test-results/`（`CLAUDE.md`）→ subagent 并行只写代码，浏览器验证由主 agent 串行。
- 两小时长跑后立刻跑 `m8-baseline` 曾失败、重跑通过（`docs/validation/p3/m7.md:52`）→ 证据层把长跑排在最后。
- 从 Playwright 进程测延迟是页内的 10–30 倍（`docs/plan/P1-WASM-UI.md:56`）→ 证据层度量方法不动。
- 导航本身约 10.9 s、与页面做什么无关（`docs/reports/p2/performance.md:76-86`）；SwiftShader 字体图集每页再阻塞约 6 s（`tests/browser/support.ts:58-64`）→ ADR-6 让每页只付一次，不以「共享 context 新页」为主手段。
- 重新挂载 scope 不复位应用状态（`tests/browser/p3-faults.spec.ts:162-165`）→ `reset()` 必须复位应用层，而不只是 dispose + mount。
- `mbx test --workspace --lib` 跳过 bin crate（本次核实：无 `xtask/src/lib.rs`）→ D7 加 `--bins`。
- Tailwind 默认自动扫描仓库根（本次实测）→ D11。
- Tailwind v4 一次构建只有一个前缀（本次实测）→ ADR-2 的前提。
- 去前缀后 `--spacing` 与 SDK 令牌同名（本次实测）→ D12。
- 脚本化替换后 `cargo fmt` 会重排行（`CLAUDE.md`「Scripted edits」）→ M4 断言命中数并在 fmt 后复查。
- Vellum spec 钉住 DOM 钩子（`tests/vellum/*`）；`openTwin` 用 `?canvas` 打开孪生并断言后端为「Canvas 2D」（`tests/vellum/twin.ts:9-13`），对 `examples/vellum` 基线必须改为「Makepad WebGL2」→ D8、D18、A-5。
- `Layer` 靠 inert 而非 Tab 循环（`crates/rustify-ui/src/overlay.rs:286-293`）；`p2-a11y` 断言「键盘永不被困」（`tests/browser/p2-a11y.spec.ts:9-11`）→ Tab 循环只在模态层内，关闭后焦点归还不变。
- Leptos `window_event_listener` 不带实例 abort signal（`examples/property-workbench/src/main.rs:1409`）→ M6 改用 `listen`。

## 13. 需求 → 设计 → 验证映射

| ID | 需求/约束/假设 | 设计落点 | 验证/解除办法 | 结果 |
| --- | --- | --- | --- | --- |
| R-1 | 测试显著变快 | ADR-1、ADR-6、D2–D7、§5.1 | M2 各 project 墙钟、CI job 时长 | 覆盖 |
| R-2 | 证据类移出 PR、去重、补单测 | ADR-1、§5.1 去重表、D7 | `--list` 守恒 + 删除清单；`--lib --bins` | 覆盖 |
| R-3 | 从 Vellum 抽公共组件 | ADR-5、D16–D19、§5.3 | M6/M7 单测与回归、基线孪生视觉、NFR-7 检查 | 覆盖 |
| R-4 | 去 `rui:` 前缀 | ADR-2、D12、§5.2 | M4 命中数、计算样式差异 0、`merge` 覆盖单测 | 覆盖 |
| R-5 | 应用侧友好的 Tailwind v4 | ADR-3、D13–D15、§5.2 | M5 构建、double build、覆盖断言、文档 | 覆盖 |
| R-6 | mise 固定 CLI，无 Node | D10、§5.2 | M3 无 `node_modules` 的 `css --check`、故障注入 | 覆盖 |
| NFR-1 | 墙钟目标（CI ≤ 10 min / 本地 ≤ 5 min） | D9 | M2/M8 实测对照 | 覆盖 |
| NFR-2 | 证据保真 | ADR-1 | 计数守恒；证据层全绿 | 覆盖 |
| NFR-3 | 构建效率与产物 | D10/D11/D14 | `css --check`、double build | 覆盖 |
| NFR-4 | 视觉不回归 | D12、D18、§9.4 | 计算样式快照；Vellum 基线孪生视觉（A-5） | 覆盖 |
| NFR-5 | 隔离修订 | ADR-2 | host 单测；宿主外观用例 | 覆盖 |
| NFR-6 | 可访问性 | §6、`Layer` Tab 循环 | 新 spec；`p2-a11y` | 覆盖 |
| NFR-7 | 去掉私有直连 | D17 | M6 `rg` 检查 | 覆盖 |
| C-1 | mbx | §5.2、§9 | 命令审阅 | 覆盖 |
| C-2 | 提交产物与目录文档 | M3–M5、M7 | `css --check`、`catalog --check` | 覆盖 |
| C-3 | 严格 CSP | §6、§7 | 既有 CSP 用例（`p3-policy` 等） | 覆盖 |
| C-4 | 页面级监听带 abort | `listen` | `p3-instances` 同型断言 | 覆盖 |
| C-5 | 受控值 | ADR-4（已确认的例外） | Draft 单测；架构文档 | 覆盖 |
| C-6 | Playwright 使用纪律 | §9、§10 并行说明 | 执行审阅 | 覆盖 |
| C-7 | 中文计划、证据位置、源码无编号、脚本断言 | 全文、M4 | 记录审阅 | 覆盖 |
| C-8 | 不擅自装服务；不新开分支 | A-1、A-5 | 执行审阅 | 覆盖 |
| C-9 | Vellum 视觉与钩子 | D18、ADR-5 | 基线孪生视觉；Vellum 回归 spec | 覆盖 |
| A-1 | mise `github:` 后端 | D10 | M3 开工 `mise install` 核实 | 开放 |
| A-2 | 页内复位可行性与成本 | ADR-6、D4 | M1 探针 + M2 复位等价用例 | 开放 |
| A-3 | 3 回合足够 | D3 | 接受；证据层兜底 | 接受 |
| A-4 | argv 过滤 webServer | D6 | M1 | 开放 |
| A-5 | `examples/vellum` 基线作孪生 | D8、§9.4 | M6 开工接入并试跑 | 开放 |
| A-6 | CI 并行容量 | D7 | M2 实跑 | 开放 |
| A-7 | 本地 5 min 可达与 `workers: 2` 内存余量 | D9 | M1 探针 + M2 实测 | 开放 |
| — | **不在本期：CI 换 OS、Rust 级 DOM 测试、Vellum 改用样式组件、未准入的 Vellum 件、CSS watch、按组件裁剪、手写规则分层、主题模型扩展、改写历史计划** | §0.5 | 各自触发条件 | 排除 |
