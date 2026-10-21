# DOCS-LANDING · Rustify UI · 投放落地页模式（rustify-docs · 按页命名 CSP 配置 + 落地页布局）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-21 · `b45e1ab41dc44ee10d5832365d3acfb5637fee77` · clean。
>
> 需求来源：用户需求，无 PRD。2026-10-21 用户确认三项取舍：CSP 按页、由命名配置放宽；表单包括原生表单和第三方 iframe 嵌入表单；首期包含广告参数透传。同日确认兼容边界：透传写在全站共享的 `site.js` 里，没有落地页的站点允许 `site.js` 及其指纹引用变化（NFR-4）；V-13 所需的厂商 ID 由用户提供。
>
> 本期交付：**rustify-docs 支持 `layout: landing` 落地页。页面通过 `landing: <配置名>` 选用 `rustify-docs.toml` 中声明的命名配置，只有这类页面放宽 CSP：放行列出的第三方脚本、统计请求、像素、iframe 和表单目标，在 `<head>` 挂载外部或自托管追踪脚本，可提交原生表单、嵌入第三方 iframe 表单，并把 gclid、bd_vid、utm_* 等参数透传到表单隐藏字段和 CTA。落地页头部没有搜索和导航。**
> **开发范围：`crates/rustify-docs`（配置、frontmatter、内容分析、HTML 策略检查、外壳、生成、manifest、主题脚本）、`crates/rustify-docs-cli`（build、preview、dev 的按页响应头）、`tests/docs-ssg/` 下新增的 fixture、浏览器 spec 与本地第三方替身，以及 `docs/docs-ssg.md`。依据见 §0。**
> 本期独特职责：在“每页严格 CSP”的现有承诺内划出一块按页、可审计的例外。文档页、首页和 404 的策略逐字节不变。
> **顶层排除：不做 Rust 注册页的落地页布局，不做线索接收后端、表单服务、同意管理、A/B 测试，不支持内联脚本、nonce、hash，不放宽 style-src，不发布新 CLI 版本，不更新官方站点。**

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

- 最近更新：2026-10-21 20:20 +0800
- 当前进度：5/5 个里程碑完成
- 当前状态：全部完成。M5 收口的 V-7、V-9 到 V-14 都有证据，A-1、A-3 成立，A-2 部分不成立并已修订文档示例；收口中修复了 iframe 透传（改为替换元素）和 JS 关表单测试，Firefox 证据经 BiDi 驱动的稳定版取得，偏差与遗留见 [M5 记录](DOCS-LANDING.records/M5.md)
- 最近完成：M5 阶段收口
- 下一步：代码评审。由用户发起 review-code，以本计划路径为参数
- 当前阻塞：无
- 环境要点：系统 `python3` 是 3.9，需要 Python 3.12 的检查，以及 Playwright 自带 Firefox 不能启动时的替代做法，见 M5 记录头部与「偏差与修复」
- 代码基线：M1–M4 在 `b49aa8fe`，M5 收口在 `1f49749c`；均未推送

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-21 17:26 +0800 | `landing.rs`、配置解析、`Layout::Landing` 与页面交叉校验落地；V-1、V-2 通过 | [M1 记录](DOCS-LANDING.records/M1.md) | b49aa8fe |
| M2 | 已完成 | 2026-10-21 17:37 +0800 | 落地页外壳、按页 meta 与 `page_policies`、preview/dev 按页头落地；V-3、V-4、V-5 通过 | [M2 记录](DOCS-LANDING.records/M2.md) | b49aa8fe |
| M3 | 已完成 | 2026-10-21 17:50 +0800 | iframe 与表单的页面级判定和最终闸门、`site.js` 透传、第三方替身与 `landing.spec.ts` 落地；V-6、V-15 通过 | [M3 记录](DOCS-LANDING.records/M3.md) | b49aa8fe |
| M4 | 已完成 | 2026-10-21 17:54 +0800 | `docs/docs-ssg.md` 新增 Landing pages 一节并增量修改七处；V-8 通过，厂商示例待 V-13 定稿 | [M4 记录](DOCS-LANDING.records/M4.md) | b49aa8fe |
| M5 | 已完成 | 2026-10-21 20:20 +0800 | V-7、V-9 到 V-14 通过；iframe 透传改为替换元素，JS 关表单测试改用减少动态；百度示例补 `connect`；Firefox 经 BiDi 驱动的稳定版取证 | [M5 记录](DOCS-LANDING.records/M5.md) | 1f49749c |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能 | 用户 2026-10-21：“无顶栏搜索/导航的落地页布局” | `layout: landing` 页面：头部只有品牌，没有搜索、`nav.top`/`nav.social`、语言切换和模式切换；没有侧栏、目录、上一页/下一页、编辑链接和日期；主体沿用首页的 hero（标题、tagline、actions、features），后接正文；页脚只列配置给出的链接；页面不进侧栏，默认不被搜索引擎收录、不进站内搜索、不进 `llms.txt`，三项都可以显式打开 | §5.4、§6；V-3、V-7、V-10 | 已确认 |
| R-2 | 功能 | 用户 2026-10-21 选择“按页·命名配置” | 在 `rustify-docs.toml` 声明命名配置，页面在 frontmatter 用 `landing:` 写配置名来选用；只有这类页面放宽策略，其余页面仍是严格 CSP；Markdown 作者只能选择已声明的配置，不能自己添加来源 | §5.1、§5.2；V-1、V-2、V-3 | 已确认 |
| R-3 | 功能 | 用户 2026-10-21：“head 外部脚本引用” | 配置中的脚本（外部 https 地址，或项目内文件）按顺序挂载到 `<head>`，每个都是 `async` 或 `defer`；不允许内联脚本 | §5.3；V-1、V-3、V-7 | 已确认 |
| R-4 | 功能 | 用户 2026-10-21：“可配置的第三方源白名单 CSP” | 白名单分五类：`script`、`connect`、`img`、`frame`、`form`，覆盖追踪脚本、统计请求、像素、追踪 iframe 和表单目标 | §5.2；V-1、V-15、V-7 | 已确认 |
| R-5 | 功能 | 用户 2026-10-21 选择“原生表单 + 第三方嵌入表单” | 落地页正文里的原生 `<form>`：`action`（以及 `formaction`）在构建期校验，必须指向配置列出的来源或站内页面；禁用 JS 也能提交；提交后可以 303 跳回站内感谢页 | §5.5；V-6、V-7 | 已确认 |
| R-6 | 功能 | 同 R-5 | 落地页正文的 `<iframe src>` 指向配置 `frame` 白名单中的来源时允许（用于金数据、腾讯问卷、Tally 等嵌入表单）；其余页面仍禁止 iframe | §5.5；V-6、V-7 | 已确认 |
| R-7 | 功能 | 用户 2026-10-21 选择“包含”参数透传 | 配置的 `carry` 列出参数名。主题脚本把当前 URL 中这些参数的值写入 `input[type=hidden][data-rd-carry]`，并追加到 `a[data-rd-carry]`、`iframe[data-rd-carry]` 和落地页 hero 的 actions；不写 cookie，不写存储 | §5.6；V-3、V-15、V-7 | 已确认 |
| R-8 | 功能 | 由 R-2 推出：响应头 CSP 与 meta CSP 同时生效，任一侧严格就会拦截 | 每页的 `<meta>` CSP、`preview`/`dev` 的响应头和 manifest 的按页记录保持一致；文档说明托管方应当如何发送响应头 | §5.7；V-5、V-15、V-7、V-10 | 已确认 |
| R-9 | 功能 | 作者面向的功能必须有参考文档（仓库惯例：`docs/docs-ssg.md` 是作者参考） | 文档写明配置、frontmatter、布局、表单、嵌入、透传、托管要求、诊断规则，并给出 Google tag 与百度统计的配置示例 | §5.8；V-8、V-14 | 已确认 |
| NFR-1 | 安全·隔离 | 现状 `shell.rs:42` 全站同一严格策略 | 没有配置的页面，策略字节等于 `shell::CSP`；放宽只发生在“落地页 + 配置”的页面 | §5.2、§5.7；V-1、V-3、V-15、V-10 | 已知 |
| NFR-2 | 安全·最小放开 | 由 R-4 推出 | 来源语法收窄：拒绝通配 scheme、关键字（`'unsafe-*'`、nonce、hash）、`data:`、`blob:` 和带路径的来源；不放宽内联脚本与样式；保留 `frame-ancestors 'none'`；配置值与 manifest 不可能把换行注入响应头 | §5.1、§5.2、§5.7；V-1、V-5、V-15、V-7 | 已知 |
| NFR-3 | 性能 | `docs/docs-ssg.md`「Browser and measurement scope」的预算：LCP ≤2.5 s、CLS ≤0.1、HTML+CSS+基础脚本 gzip-9 合计 ≤100 KB；`xtask/src/docs_perf.rs:22` `PAGE_LIMIT_BYTES = 100_000` | 落地页框架部分（页面 HTML、站点 CSS、`mode.js`、`site.js`）沿用上述预算；第三方与作者脚本不计入，但必须是 `async`/`defer`，不阻塞渲染 | §5.3、§7；V-3、V-11、V-12 | 已知 |
| NFR-4 | 兼容 | `publish.rs:32-35` 输出确定性；现有站点不应受影响；用户 2026-10-21 确认兼容边界 | 没有 `[landing]` 的站点：每页 CSP 逐字节等于 `shell::CSP`，manifest 不出现 `page_policies`，`schema_version` 仍为 1，既有功能不变（既有测试照常通过）。全站共享的 `site.js` 因透传而改变内容，它的指纹文件名、各页对它的引用、manifest 中的对应条目与 `build_id` 随之变化，这是允许的变化；不承诺与旧版 CLI 的输出逐字节相同。同一版本对同一输入仍写出相同字节（C-6） | §3、§5.6、§5.7；V-3、V-5、V-9 | 已确认 |
| NFR-5 | 可用性 | 主题承诺：内容不依赖脚本（`shell.rs` 模块注释、`docs/docs-ssg.md`「The default theme」） | 禁用 JS 时原生表单可以提交，落地页内容完整；透传缺席只影响归因 | §5.5、§5.6；V-7 | 已知 |
| NFR-6 | 隐私 | 由 R-7 推出 | 透传只复制配置列出的参数，不持久化；第三方脚本的同意管理由作者负责（§0.5） | §5.6、§5.8；V-15、V-7 | 已知 |
| C-1 | 约束 | `CLAUDE.md` Conventions | 计划用中文；源码不写需求、计划、里程碑编号 | 全文 | 已确认 |
| C-2 | 约束 | `CLAUDE.md` Commands | 构建与测试走 `mbx`；两个 docs 包各自单独 test/clippy，不参与 workspace 联合构建 | §9 | 已确认 |
| C-3 | 约束 | `CLAUDE.md` Browser tests | Playwright 每条命令只跑一个 `--project`，不并发，不裸跑 | §9 | 已确认 |
| C-4 | 约束 | `CLAUDE.md`：“Performance figures are measured inside the page” | 性能数字在页面内测量（`tests/docs-ssg/harness/measure-load.mjs`），不用 Playwright 计时 | V-12 | 已确认 |
| C-5 | 约束 | `crates/rustify-docs/src/config.rs:1-7,19` | 配置的每个键都是已知键（`deny_unknown_fields`）；`SCHEMA_VERSION = 1` | §5.1 | 已确认 |
| C-6 | 约束 | `crates/rustify-docs/src/publish.rs:32-35` | 同一输入写出相同字节；manifest 不含时间、路径、环境 | §5.7 | 已确认 |
| C-7 | 约束 | `CLAUDE.md`：“不新开分支” | 改动直接提交到 `main` | §10 提交点 | 已确认 |
| C-8 | 约束 | `docs/docs-ssg.md`「Custom pages and layout slots」：注册页元数据只接受字面量 `draft`/`index`/`search`/`ai` | 本期落地页只支持 Markdown 页面 | §0.5 | 已确认 |
| A-1 | 假设 | Google 的 CSP 指南只对内联容器代码要求 nonce 或 hash（<https://developers.google.com/tag-platform/security/guides/csp>） | 把 gtag 初始化代码放进自托管的外部文件，再配合指南列出的来源，Google tag 即可运行，不需要内联 | V-13 在线核对（判据见 V-13：非空脚本、没有初始化错误、观察到统计请求）；不成立时文档写明本期不支持 Google tag，并另立“hash 内联”计划，本期其余交付不受影响；缺 ID 或无法联网属于缺证，A-1 保持开放，不按“不成立”处理；责任：实施 agent，测试 ID 不触发请求时所需的真实 ID 由用户提供（§4）；最晚 M5 | 已解除：成立（M5 V-13，测试 ID 触发 GA4 与 Ads 请求） |
| A-2 | 假设 | 百度统计没有官方 CSP 文档（2026-10-21 检索未找到）；全零测试 ID 的 `hm.js` 返回 0 B（2026-10-21 实测） | `hm.js` 只需要 `script` 和 `img` 两类来源放行 `https://hm.baidu.com` | V-13 用用户提供的获授权站点 ID 观察统计请求与违规报告，据此修订文档示例；缺 ID 时 A-2 保持开放；责任：实施 agent，站点 ID 由用户提供（§4）；最晚 M5 | 已解除：部分不成立，还需 `connect`（离开页面时的停留时长请求）；`hm.js` 用 `new Function` 解析数据被策略拒绝，统计照常；文档示例已据此修订（M5 V-13） |
| A-3 | 假设 | CSP3 对重定向是否检查 `form-action` 未作统一规定 | `form-action` 包含 `'self'` 时，Chromium、WebKit、Firefox 都允许 POST 之后 303 跳回站内感谢页 | V-10 在 `docs-webkit`、`docs-firefox` 下运行 `landing.spec.ts`；某引擎行为不同时，文档写明该引擎的限制；责任：实施 agent；最晚 M5 | 已解除：成立，Chromium、WebKit、Firefox 在 JS 开与关时都跳回感谢页（M5 V-10；Firefox 为 BiDi 驱动的稳定版，见 M5 记录） |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 形态 | 在 rustify-docs 内增量：新模块 `landing.rs`，加上现有管线（配置 → 分析 → 外壳 → 生成 → 发布）的扩展；不新增包，也不新增命令 | 落地页走与普通页面相同的构建、检查、预览、dev 流程，不需要另学一套 | R-1–R-8；`crates/rustify-docs/src/generation.rs`、`content.rs` |
| D2 | 放宽的作用域 | 命名配置放在 `[landing.profiles.<name>]`，页面用 `landing: <name>` 选用 | 见 ADR-1 | R-2、用户确认 |
| D3 | 策略投递 | 每页写 `<meta>`；`Generation` 与 manifest 增加 `page_policies`；`preview`/`dev` 按页发送响应头 | 见 ADR-2 | R-8、NFR-4 |
| D4 | 脚本接入 | 只接受 `src`（外部 https，或项目文件按内容指纹发布），加载方式是 `async` 或 `defer`，按配置顺序挂载；不支持内联、nonce、hash | 见 ADR-3 | R-3、NFR-2 |
| D5 | 外部脚本的来源 | 外部脚本的 origin 自动并入 `script-src`，不要求在 `sources.script` 里重复写 | 少写一处重复配置；合成后的策略在页面 HTML 和 manifest 里都可以审计 | §5.2 |
| D6 | 来源语法 | 只接受 `https://host[:port]` 和 `https://*.suffix[:port]`；host 是 ASCII 域名，IP 字面量只接受 `127.0.0.1`；`http://` 只允许回环主机 `127.0.0.1`、`localhost`；拒绝 IPv6 字面量（如 `[::1]`）、其他 IP 字面量、路径、查询、通配 scheme、关键字、`data:`、`blob:`、非 ASCII 主机 | 回环 http 让本地替身可以测试；生产页面在 https 下加载 http 资源本来就会被浏览器拦截。CSP3 的 host-source 语法中 host 字符不含方括号和冒号，写不出 IPv6；host 匹配只认域名，规范注释只承认 `127.0.0.1` 这一个 IP 能匹配，其他 IP 写进策略也不能保证放行 | NFR-2；[CSP3 §2.3.1](https://www.w3.org/TR/CSP3/#framework-directive-source-list)、[§6.7.2.10](https://www.w3.org/TR/CSP3/#match-hosts) |
| D7 | `form-action` | 配置有 `form` 来源时为 `'self' <来源…>`，否则为 `'none'` | `'self'` 覆盖 POST 之后跳回站内感谢页和站内 GET 表单两种情况 | R-5、A-3 |
| D8 | iframe 与表单的检查位置 | Markdown 片段扫描只记录 URL 槽位（`UrlRole::Frame`、`UrlRole::FormTarget`），放不放行在页面级的引用解析（`resolve_references`）里决定；注册页与 slot 的渲染面仍然直接禁止 iframe | 片段缓存键 `FragmentKey` 只含正文、语言、标签和 include（`content.rs:894-904`），不含 frontmatter，所以片段级检查必须与配置无关，否则缓存会过期 | `content.rs:894-904`、`html_check.rs:521-592` |
| D9 | 非落地页上的表单 | 只给警告 `form-blocked`，输出不变 | 这类表单今天在浏览器里本来就提交不了（`form-action 'none'`），但构建从不报错；改成错误会让现有站点构建失败 | NFR-4；`html_check.rs:68-81` |
| D10 | 落地页的 `index`/`search`/`ai` 默认值 | 默认都是 `false`，frontmatter 写 `true` 可以覆盖 | 投放页通常不该被收录，避免和 SEO 页重复；新布局没有旧行为需要兼容 | R-1 |
| D11 | 落地页的品牌区 | `<div class="rd-brand">`，不是链接；`header` slot 仍可替换它 | 页面只留一个出口；样式复用 slot 版本已有的 `div.rd-brand`（`shell.rs:790`、`docs.css:299-312`）；如果投放实测需要回站链接，改成可配置 | R-1 |
| D12 | `schema_version` | 保持 1 | 旧 CLI 读到 `[landing]` 会报 unknown field，这是显式失败，不会被误读 | C-5 |
| D13 | 透传实现 | 由主题 `site.js` 读取 `<html data-rd-carry="…">` 执行；目标只有显式标注的元素和落地页 hero actions；URL 里的值覆盖字段原值 | 不新增脚本文件，不访问网络，不存储；代价是所有站点的 `site.js` 内容及其指纹引用都会变化，NFR-4 按此界定兼容边界 | R-7、NFR-4、NFR-6 |
| D14 | `style-src` | 不放宽 | 依赖内联样式的脚本式嵌入表单不支持；受支持的嵌入方式是 iframe（iframe 内的页面受它自己的 CSP 约束） | NFR-2 |

### 0.3 ADR-lite

#### ADR-1：放宽策略的作用域是“按页 + 命名配置”

- 状态：Accepted（用户 2026-10-21 选定）
- 背景与驱动：R-2、NFR-1、NFR-2。rustify-docs 承诺每页严格 CSP（`shell.rs:42`）。投放页必须加载第三方代码，文档页不需要。
- 备选：
  - A. 全站统一放宽，一份配置作用于所有页面。实现最简单，但每个文档页都暴露给第三方脚本，严格 CSP 承诺整体作废。
  - B. 每页在 frontmatter 直接写来源。最灵活，但策略散在各个 Markdown 文件里，难以审计；写正文的人就能放开脚本来源。
  - C. **配置文件声明命名配置，页面只选名字**（选定）。
- 决策：C。来源只能写在 `rustify-docs.toml`，配置的维护者对它负责；页面只做选择。
- 正面后果：例外集中在一处可审计；没有配置的页面策略逐字节不变（NFR-1）；同一站点可以有多个投放渠道的配置（如 `google-ads`、`baidu`）。
- 负面/中性后果：每加一个渠道都要改配置文件；配置只对 `layout: landing` 页面生效，想给文档页挂统计做不到（排除，见 §0.5）。
- 重新评估触发：用户明确要求在文档页上挂统计，或者出现第三个需要“按页放宽”的非落地页场景。

#### ADR-2：策略按页投递，响应头取自 manifest

- 状态：Accepted
- 背景与驱动：R-8、NFR-4。浏览器会同时执行 meta CSP 和响应头 CSP；`preview`/`dev` 现在给每个响应都发严格头（`preview.rs:163`、`dev.rs:791,838`），落地页即使 meta 放宽了也照样被拦截。
- 备选：
  - A. 只在 meta 放宽，preview 与 dev 不再发 CSP 头。这会丢掉 `frame-ancestors 'none'`（只有响应头能携带），文档页也一起失去防嵌套保护。
  - B. preview 与 dev 每次读页面 HTML，从 meta 里解析出策略。需要在请求路径上解析 HTML，而且 meta 被篡改时头会跟着变。
  - C. **构建时把每页的头部策略写进 `Generation.page_policies` 和 manifest 的 `page_policies`；preview 与 dev 按文件取用，没有记录的一律发严格头**（选定）。
- 决策：C。这与 `locale_not_found` 的“manifest 可选字段 + preview 启动时校验并冻结”是同一模式（`publish.rs:44-45,118-167`，`main.rs:849`）。
- 正面后果：头与 meta 来自同一次构建；能设置响应头的托管方可以照 manifest 配置按页头；manifest 里这个字段为空时省略，没有落地页的站点不多出字段。
- 负面/中性后果：manifest 多一个字段；旧版 CLI 的 preview 会忽略这个字段（serde 默认忽略未知字段），继续发严格头，落地页因此失效，所以版本必须配对。
- 重新评估触发：需要为某个托管平台生成 `_headers` 一类按页头配置文件时，从 `page_policies` 派生。

#### ADR-3：第三方脚本只按 src 挂载，不支持内联

- 状态：Accepted
- 背景与驱动：R-3、NFR-2。厂商给的接入代码通常是一段内联脚本加一个外部脚本。
- 备选：
  - A. 放行 `'unsafe-inline'`。等于同时放开 XSS 面，违背 NFR-2。
  - B. 构建期计算内联片段的 sha256 并写进 `script-src`。静态站点做得到，但要新增“内联片段”配置项，并与 `html_check` 禁止内联的规则协调，复杂度和审计成本更高。
  - C. **只接受外部 `src`：厂商代码的内联部分由作者放进项目里的 `.js` 文件，构建时按内容指纹发布**（选定）。
- 决策：C。gtag 的内联部分只是定义 `dataLayer`/`gtag()` 并调用 `config`，放进外部文件也能执行（A-1）。
- 正面后果：不新增任何内联执行面；`html_check` 的 `inline-script` 规则保持不变；脚本文件随内容指纹缓存。
- 负面/中性后果：作者要做一步改写，文档提供示例（§5.8）；依赖内联的嵌入（部分 HubSpot 脚本嵌入）不支持（D14）。
- 重新评估触发：A-1 不成立，或者有主流投放平台只提供内联接入方式。

### 0.4 职责与事实所有权

|  | 站点作者与配置维护者 | rustify-docs（本期） | 广告与统计平台、线索端点、表单服务 | 静态托管方 |
| --- | --- | --- | --- | --- |
| 拥有 | 配置写哪些来源、选哪些脚本、透传哪些参数；落地页文案与表单字段；隐私政策与同意管理 | 来源语法校验；按页合成策略；脚本挂载；构建期校验 iframe 与表单目标；meta、头部与 manifest 三者一致；透传行为；布局 | 自家脚本行为与域名清单；线索的接收、存储与 303 重定向；嵌入表单能否被 iframe 嵌入；转化计数 | 按 manifest 发送按页 CSP 头，或者完全不发 CSP 头 |
| 不拥有 | — | **线索后端、表单服务、同意管理、厂商域名清单的时效性、转化是否被平台计入** | 站点的 CSP | 页面内容 |

一句话：rustify-docs 负责让“配置说放行什么，浏览器就放行什么，其余一律拦截”在构建、预览和托管各环节都成立；第三方服务能否工作由各提供方负责。

### 0.5 明确不在本期

- **Rust 注册页（`site!` 的 `pages`）的落地页布局**：宏只接受字面量元数据（C-8），需要另立项。
- **线索接收后端、表单服务、CRM、离线转化上传**：外部系统，见 §4。
- **同意管理与 Consent Mode 集成**：作者在自托管脚本里处理；文档会提示合规责任。
- **A/B 测试与多变体分流**：明确放弃。
- **内联脚本、nonce、hash 支持，以及放宽 `style-src`**：见 ADR-3、D14。
- **文档页、首页、404 挂统计**：见 ADR-1。
- **为托管平台生成按页头配置文件（`_headers` 等）**：本期只在文档里说明怎么从 `page_policies` 配置。
- **外部脚本的 SRI（`integrity`）**：gtag 等脚本内容频繁变化，SRI 不可行，明确放弃。
- **发布新 CLI 版本（C3 配对）与更新官方站点**：之后走既有的 Pages 发布流程另行安排。`docs/docs-ssg.md` 不在官方站点的 `content.include` 里（`rustify-docs.toml`），本期改它不影响线上。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **已核实·足够**
  - 任意 Markdown 页都可以用 `layout: home`：无侧栏，也不进导航（`content.rs:3555-3560`；夹具 `tests/docs-ssg/fixtures/home/content/explicit.md`）。落地页沿用这套 hero 与正文布局（`shell.rs:644-715`）。
  - 正文原始 HTML 的 `class` 会被收进站点样式表（`classes.rs:1-27`），作者可以用 Tailwind 工具类排版落地页的区块和表单。
  - `site.js` 在页面没有搜索或模式控件时会跳过对应逻辑（`site.js:77-78,404-406`），落地页去掉这些控件不需要改脚本守卫。
  - 配置里引用的项目文件（logo）会检查是否存在，并按内容指纹发布（`content.rs:2529-2555`、`generation.rs:930-960`、`resource_output` 位于 `generation.rs:238`）。本地脚本走同一路径。
  - 配置里的链接（`nav.top`）由 `config_links` 解析和校验（`content.rs:2354-2383`），渲染时用 `header_links`（`generation.rs:501`）。落地页页脚复用这两处。
  - manifest 的可选字段，以及 preview 启动时的校验与冻结，已有先例 `locale_not_found`（`publish.rs:44-45,118-167`、`main.rs:849`）。
  - 页面内性能测量工具 `tests/docs-ssg/harness/measure-load.mjs` 按冻结合同采样。
  - 浏览器 spec 自建 fixture 加 preview 的模式见 `tests/docs-ssg/home.spec.ts:13-48`。它会在 `docs-root`、`docs-subpath`、`docs-meta`、`docs-firefox`、`docs-webkit` 项目下运行（`playwright.docs.config.ts:133-176`）。CI 运行 docs-root、docs-subpath、docs-meta、docs-dev、docs-webkit（`.github/workflows/verify.yml:209-220`）。
  - `rustify-docs` 有 `flate2` dev 依赖（`crates/rustify-docs/Cargo.toml:62`），测试可以直接算 gzip-9 大小。
- **已核实·缺口**
  - CSP 是全站唯一的常量（`shell.rs:42-50`），每页 meta 都用它（`shell.rs:491`）→ 新增按页合成（§5.2）→ 不改就无法放宽。
  - preview 与 dev 对每个响应都发固定的 `CSP` 头（`preview.rs:163`、`dev.rs:791,838`）→ 改为按页取用（§5.7）→ 不改的话，头与 meta 叠加后仍然拦截。
  - 最终闸门 `check_document` 拒绝跨源脚本和任何 iframe（`html_check.rs:772-...`）→ 新增感知策略的 `check_document_under`（§5.5）→ 不改则落地页构建失败。
  - `<iframe>` 在片段级被禁止（`html_check.rs:51-56` 的 `FORBIDDEN`）→ 改为在落地页的页面级判定（D8）。
  - `form action` 不是 URL 角色（`html_check.rs:68-81`），不解析也不校验，浏览器运行时被 `form-action 'none'` 静默拦截 → 新增角色，并给出警告或校验（§5.5）。新角色还会进入最终产物的引用检查：`final_documents` 消费 `document_references` 返回的全部 URL 槽位，并要求站内目标存在（`generation.rs:699,721-792`），所以必须在那里排除，否则普通页上指向站内端点的表单会被报 `output-check`。
  - 布局只有 `Doc`/`Home` 两种（`frontmatter.rs:88-92`）；`index`/`search`/`ai` 的 `== Some(false)` 判定散在 5 处（`generation.rs:812,1139,1172`、`search.rs:113`、`ai.rs:125`）→ 统一成辅助方法，落地页默认 `false`。
  - 头部固定渲染搜索、链接、语言和模式切换（`shell.rs:761-809`）；搜索对话框和默认页脚无条件渲染（`shell.rs:534-545`）。
- **未核实阻塞**：无。三项外部行为假设见 A-1–A-3，均不阻塞开工。

### 1.2 文件清单

本期都在本仓库内，★ 表示新增。

| 路径 | 改动 |
| --- | --- |
| `crates/rustify-docs/src/landing.rs` ★ | 配置类型、来源语法、策略合成、host-source 匹配（§5.1、§5.2） |
| `crates/rustify-docs/src/lib.rs` | `#[cfg(feature = "pipeline")] pub mod landing;` |
| `crates/rustify-docs/src/config.rs` | `RawConfig.landing`，以及解析为 `Config.landing` |
| `crates/rustify-docs/src/frontmatter.rs` | `Layout::Landing`、`Front.landing`、辅助方法 |
| `crates/rustify-docs/src/content.rs` | 页面与配置交叉校验、本地脚本资源、页脚链接、未用配置警告、iframe/表单的页面级判定、`has_hero` 替换 |
| `crates/rustify-docs/src/markdown.rs` | `RefKind::Frame`、`RefKind::Form` 的映射（两处：`markdown.rs:325`、`markdown.rs:725` 附近） |
| `crates/rustify-docs/src/html_check.rs` | 新 URL 角色、Markdown 片段中的 iframe 处理、`check_document_under` |
| `crates/rustify-docs/src/shell.rs` | `Landing` 结构、按页 meta 策略、落地页头部与页脚、脚本挂载、透传属性 |
| `crates/rustify-docs/src/generation.rs` | 组装落地页、`Generation.page_policies`、按策略执行最终闸门、改用辅助方法、`final_documents` 跳过表单目标 |
| `crates/rustify-docs/src/search.rs`、`ai.rs` | 改用辅助方法；`ai.rs` 的首页描述回退也覆盖落地页 |
| `crates/rustify-docs/src/publish.rs` | `Manifest.page_policies` 与校验，`add_manifest` 增加参数 |
| `crates/rustify-docs/assets/site.js` | 参数透传 |
| `crates/rustify-docs/Cargo.toml` | `[[test]] name = "landing"`，`required-features = ["pipeline"]` |
| `crates/rustify-docs/tests/landing.rs` ★ | 生成集成测试 |
| `crates/rustify-docs-cli/src/main.rs` | build 传入 `page_policies`；preview 读取并校验 |
| `crates/rustify-docs-cli/src/preview.rs` | 按文件发 CSP 头 |
| `crates/rustify-docs-cli/src/dev.rs` | 按页发 CSP 头 |
| `crates/rustify-docs-cli/tests/landing.rs` ★ | preview 与 dev 的头部集成测试 |
| `tests/docs-ssg/fixtures/landing/` ★ | 落地页 fixture（§9.1） |
| `tests/docs-ssg/landing-third-party.mjs` ★ | 本地第三方替身（§9.1） |
| `tests/docs-ssg/landing.spec.ts` ★ | 浏览器 spec |
| `tests/docs-ssg/serve.sh` | 新增 `landing` 来源 |
| `docs/docs-ssg.md` | 作者参考（§5.8） |

**无需修改的邻接面**：
- `playwright.docs.config.ts`：docs 项目的 `testMatch` 已覆盖 `docs-ssg/**/*.spec.ts`。
- `.github/workflows/verify.yml`：已在运行 docs 项目。
- `crates/rustify-docs-syntax` 与 `crates/rustify-docs-macros`：注册页不在本期范围。
- `site/`：官方站点不使用新功能，只作回归对象。
- `crates/rustify-docs/assets/docs.css`：落地页复用 `rd-home`、`rd-hero`、`rd-brand`、`rd-site-footer`。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| `landing.rs` ★ | `config.rs`（解析）、`content.rs`（页面级判定）、`generation.rs`（合成）、`html_check.rs`（最终闸门） | `policy(None) == shell::CSP`；合成结果确定；来源已规范为小写 origin；`allows` 只实现 §5.2 的 host-source 子集 | 无 | 来源语法、策略字符串的顺序与去重、通配匹配、默认端口 | 进程内 | 单元测试：接受/拒绝表、合成串逐字节、匹配表 |
| `html_check::check_document_under` | `generation.rs` 的最终闸门 | `check_document(html) == check_document_under(html, None)`；只放行配置允许的脚本、iframe、表单目标 | 无 | 闸门与页面级判定必须一致 | 进程内 | 单元测试：同一份 HTML 在有无配置时的结果 |
| `Manifest::checked_page_policies` | CLI `preview` | 键是 manifest 中列出的 `.html` 文件；值是可打印 ASCII，以 `default-src 'none'; ` 开头，以 `; frame-ancestors 'none'` 结尾 | 无 | 防止换行注入响应头、防止损坏的 manifest | 本地文件 | 单元测试：合法、非法、缺字段 |
| `site.js` 透传 | 浏览器 | 只读取 `location.search`；只写 `input.value` 和 URL 的 searchParams；从不使用 `innerHTML`，不访问存储 | 无 | URL 编码与解码，已有参数不重复追加 | 浏览器 | `landing.spec.ts` |

只有 `landing.rs` 是新模块。它集中了四个调用方都要用的“来源与策略”规则，把它删掉，这些规则会散回四处，所以它通过删除测试。不新增其他接缝或适配器：第三方平台只出现在测试替身和文档里，代码里没有任何厂商特化。

## 3. 数据与兼容（构建产物）

| 对象 | 关键字段与约束 | 生命周期与权威来源 | 读者 |
| --- | --- | --- | --- |
| `rustify-docs-manifest.json` 的 `page_policies` ★ | `{"<页面输出路径>": "<响应头策略>"}`；只记录策略不等于 `shell::CSP` 的页面；为空时省略（`skip_serializing_if`） | 每次 build 由 `Generation.page_policies` 写入；与页面 meta 来自同一次合成 | `preview`、托管配置者 |

- **兼容**：没有落地页的站点，manifest 不出现 `page_policies`，`schema_version` 仍为 1；`site.js` 的变化会改变 manifest 中它的文件条目和 `build_id`，这是 NFR-4 允许的变化。新 CLI 读取旧 manifest 时字段缺省为空。旧 CLI 的 preview 读取新 manifest 时会忽略该字段并发严格头，落地页因此被拦截，所以 CLI 版本必须与输出配对。
- **回滚**：回退代码提交即可，没有不可逆点。使用了 `[landing]` 的站点，在旧 CLI 下会得到显式的 unknown-field 错误。

## 4. 集成与契约

| 依赖 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| Google tag（GA4、Google Ads 转化与再营销） | 已核实·足够（来源清单）；A-1 待在线确认 | Google CSP 指南：GA4 需要 `script` 放行 `https://www.googletagmanager.com`；Ads 另需 `www.googleadservices.com`、`googleads.g.doubleclick.net`、`www.google.com`、`www.google.<TLD>`（逐个列出，不能写通配）、`ad.doubleclick.net`，以及 `frame` 放行 `www.googletagmanager.com`。2026-10-21：`gtag/js?id=G-TEST000000` 返回 200 与 443,317 B 脚本，只证明取到文件 | 范围外。本方只提供白名单与脚本挂载，文档给出示例；V-13 在线核对：先用测试 ID，没有触发统计请求时改用用户提供的真实 ID | 域名有遗漏时浏览器报违规，数据缺失；作者更新配置后重新构建 |
| 百度统计 | 未核实（无官方 CSP 文档），不阻塞开工（A-2）；在线验证需要用户提供的站点 ID | 2026-10-21：全零测试 ID 的 `hm.js` 返回 200、`text/plain`、0 B，测试 ID 不能证明接入 | 范围外。V-13 用用户提供的获授权站点 ID 观察统计请求与违规报告后定稿文档示例 | 同上 |
| 线索接收端点（作者后端或表单服务） | 外部，不阻塞 | 契约：接受 `application/x-www-form-urlencoded` 的 POST；成功后 303 重定向到站内感谢页，或者 `form` 白名单内的 origin | 范围外，不开发。测试用本地替身 `landing-third-party.mjs`（只实现上述契约） | 端点不可用时用户看到端点的错误页，线索丢失，由端点方监控 |
| 嵌入式表单服务（金数据、腾讯问卷、Tally…） | 外部，不阻塞 | 契约：允许被第三方页面 iframe 嵌入（其 `X-Frame-Options`/`frame-ancestors` 不拒绝）；URL 参数能否预填因服务而异 | 范围外。本方放行 iframe，并按 R-7 透传参数到 `src` | 服务拒绝嵌入时 iframe 显示空白，由作者改用原生表单 |
| 静态托管 | 已核实·足够（GitHub Pages 只用 meta） | `docs/docs-ssg.md`「Previewing and hosting」 | 文档说明：能设置头的托管必须按 `page_policies` 发头，或者对这些路径不发 CSP 头 | 托管方全站发严格头时，落地页的追踪与表单全部被拦（见 §8） |
| 真实广告账户的转化计入 | 外部，由用户执行，上线后验证 | — | 不作为本期退出条件 | — |

V-13 所需的厂商 ID 由用户（站点作者，持有厂商账户）在 M5 收口前提供：GA4 网站数据流的衡量 ID（`G-…`）、Google Ads 转化操作的代码 ID（`AW-…`，以及示例用到的转化标签），只在测试 ID 不触发统计请求时需要；百度统计新增网站后统计代码里 `hm.js?` 之后的 32 位站点 ID，必须提供。建议用专门测试的媒体资源与站点，避免测试流量混入正式数据。这些 ID 本来就会公开写在页面里，记入 M5 记录即可。

需要登记的新增项：配置键见 §5.1；frontmatter 键见 §5.4；诊断规则见 §5.9；manifest 字段见 §3；HTML 属性 `data-rd-carry`（`<html>`、`input`、`a`、`iframe` 上）见 §5.6。

## 5. 核心机制

### 5.1 配置（`rustify-docs.toml`）

```toml
[landing.profiles.ads]                     # 名称：^[a-z0-9][a-z0-9-]{0,31}$
carry = ["gclid", "gbraid", "wbraid", "bd_vid", "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content"]

[[landing.profiles.ads.scripts]]
src = "https://www.googletagmanager.com/gtag/js?id=AW-000000000"
load = "async"                             # async | defer，缺省为 defer

[[landing.profiles.ads.scripts]]
src = "static/landing/gtag-init.js"        # 项目相对路径，必须存在，按内容指纹发布

[landing.profiles.ads.sources]             # 五类均可省略，缺省为空
script = []                                # 外部脚本的 origin 自动并入（D5）
connect = ["https://*.google-analytics.com", "https://www.google.com"]
img = ["https://www.googletagmanager.com", "https://www.google.com"]
frame = ["https://www.googletagmanager.com", "https://jinshuju.net"]
form = ["https://leads.example.com"]

[[landing.profiles.ads.footer]]            # 与 nav.top 相同的 label/link 语义
label = "隐私政策"
link = "privacy.md"
```

Raw 类型加在 `config.rs` 的 `RawConfig` 中，写法照搬 `RawNav`/`RawLink`：每层都是 `#[serde(deny_unknown_fields)]`，并用 `Spanned` 保留定位。

- `landing: Option<RawLanding>`
- `RawLanding { profiles: BTreeMap<String, Spanned<RawProfile>> }`
- `RawProfile { scripts: Vec<Spanned<RawScript>>, sources: RawSources, carry: Vec<Spanned<String>>, footer: Vec<RawLink> }`
- `RawScript { src: Spanned<String>, load: Option<Load> }`
- `RawSources { script, connect, img, frame, form: Vec<Spanned<String>> }`

校验后的类型定义在 `landing.rs`，`Config` 增加字段 `pub landing: BTreeMap<String, landing::Profile>`：

```rust
pub enum Load { Async, Defer }               // serde: rename_all = "lowercase"
pub enum ScriptSrc { Local(String), External(String) }   // Local：项目相对路径（经 config::relative_path 规范化）
pub struct Script { pub src: ScriptSrc, pub load: Load, pub span: Range<usize> }
pub struct Sources { pub script: Vec<String>, pub connect: Vec<String>, pub img: Vec<String>, pub frame: Vec<String>, pub form: Vec<String> }
pub struct Profile {
    pub name: String,
    pub scripts: Vec<Script>,
    pub sources: Sources,                   // 已规范化的 origin：小写、去掉默认端口、去重并保持书写顺序
    pub carry: Vec<String>,
    pub footer: Vec<config::NavLink>,
    pub span: Range<usize>,                 // 配置表的位置，供未使用警告定位
}
pub fn parse_source(text: &str) -> Result<String, String>;
pub fn parse_script_src(text: &str) -> Result<ScriptSrc, String>;
```

校验规则（错误都定位在 `rustify-docs.toml`，规则名见 §5.9）：

1. 配置名不符合正则 → `landing-profile-name`。
2. 来源：按 D6，`parse_source` 不通过 → `landing-source`，并在建议里给出合法写法。
3. 脚本 `src`：以 `https://`，或以回环主机的 `http://` 开头，作为外部地址，主机规则与 D6 相同（禁止 userinfo、非 ASCII 主机、IPv6 字面量和 `127.0.0.1` 以外的 IP 字面量），因为它的 origin 要并入 `script-src`（D5）；否则按项目相对路径处理；两者都不成立 → `landing-script`。`load` 写了未知值时，serde 报错，沿用现有的 frontmatter/配置报错路径。
4. `carry` 的每项必须匹配 `^[A-Za-z0-9_.-]{1,64}$`，且不能重复 → `landing-carry`。
5. 本地脚本文件不存在（在内容分析阶段检查，同构于 `content.rs:2529-2555` 的 logo 检查）→ `landing-script-missing`。
6. 页脚链接用 `config_links` 解析，沿用 `broken-link` 等现有规则。

### 5.2 策略合成与匹配

`landing::policy(profile: Option<&Profile>) -> String` 生成响应头用的策略；`landing::meta_policy` 是同一串去掉 `frame-ancestors` 段，同构于 `shell.rs:45-50`。模板中方括号内的部分仅在非空时出现，各段之间用 `"; "` 连接：

```text
default-src 'none'; script-src 'self' 'wasm-unsafe-eval'[ S…]; style-src 'self'; img-src 'self' data:[ I…]; font-src 'self'; connect-src 'self'[ C…]; media-src 'self'; worker-src 'self';[ frame-src F…;] object-src 'none'; base-uri 'none'; form-action <'none' | 'self' A…>; frame-ancestors 'none'
```

- S 是 `sources.script` 的书写顺序，后接外部脚本的 origin（按脚本顺序，跳过已出现的）；I、C、F、A 分别是 `img`、`connect`、`frame`、`form` 的书写顺序，各自去重。
- `F` 为空时整段 `frame-src` 不出现，由 `default-src 'none'` 拦截全部 frame。
- `profile == None` 时返回的串与 `shell::CSP` 逐字节相同（NFR-1 的单元断言）。

`landing::allows(sources: &[String], url: &str) -> bool` 在构建期判断某个 URL 是否被允许，规则如下：

- URL 拆成 scheme、host、port，没写端口时取 scheme 的默认端口；
- 来源与 URL 的 scheme 必须相同；
- host 完全相等，或者来源是 `*.suffix` 且 URL 的 host 以 `.suffix` 结尾（不匹配 suffix 本身）；
- 来源写了端口则端口必须相等，没写则只匹配默认端口；
- URL 的 host 是 IPv6 字面量，或是 `127.0.0.1` 以外的 IP 字面量时一律不匹配（D6 不接受这类来源）。

这是 CSP3 host-source 的子集，与 D6 的来源语法一一对应；最终裁决仍以浏览器为准。不引入 `url` crate：仓库现有的 URL 处理都是手写（`content.rs:1273` `has_scheme`、`generation.rs:209` `site_origin`）。

### 5.3 脚本挂载

在 `<head>` 中，紧跟主题的 `site_script`（`shell.rs:513`）之后，按配置顺序输出：

- `Load::Async` → `<script src="…" async=""></script>`
- `Load::Defer` → `<script src="…" defer=""></script>`

本地脚本的 URL 是 `site.url("/" + outputs[path])`，即 `assets/files/<名>.<hash>.js`，路径来自 §5.1 第 5 步登记的资源。外部脚本按原样输出，由 Leptos 属性转义。两种加载方式都不阻塞解析，满足 NFR-3。

### 5.4 落地页布局与默认值

Frontmatter（`frontmatter.rs`）：

```yaml
---
title: 企业版 30 天试用
layout: landing          # 新增枚举值
landing: ads             # 新增键：可选，必须是已声明的配置名
tagline: …
actions: [{ text: 免费试用, link: "#lead" }]
features: [...]
# index / search / ai 缺省为 false，写 true 可覆盖
---
```

- `Layout` 增加 `Landing`，并新增 `Layout::has_hero(self) -> bool`（`Home | Landing`）。以下 5 处改用它：`content.rs:1805`（H1 并入 hero）、`content.rs:1851`（features 之后的标题层级）、`content.rs:1877`（description 可省略）、`content.rs:3558`（不进侧栏）、`generation.rs:1082`（构造 `Home`）。`ai.rs:36` 的首页描述回退也改用它。
- `Front` 增加 `pub landing: Option<String>`，并新增三个方法：
  - `indexed()`：`index.unwrap_or(layout != Landing)`
  - `searchable()`：`search.unwrap_or(layout != Landing)`
  - `discoverable()`：`ai.unwrap_or(layout != Landing)`

  它们替换 `generation.rs:812,1139,1172`、`search.rs:113`、`ai.rs:125` 的判定。`noindex` 的写法改成 `!page.front.indexed() || page.locale.is_fallback`。
- 内容校验（`content.rs`，放在 description 判定附近）：
  - 写了 `landing:` 但布局不是 `landing` → `landing-without-layout`；
  - 配置名未声明 → `landing-profile`，建议里列出已声明的名字。

  两者都定位到 frontmatter 中该值的位置，查找方式同构于 actions 链接的定位（`content.rs:2285-2296`）。
- 全部页面分析完之后，未被任何页面（包括草稿）引用的配置 → 警告 `landing-profile-unused`，定位到配置表。
- `Analysis` 增加 `pub landing_footers: BTreeMap<String, Vec<Target>>`，由 `config_links(&profile.footer, None)` 解析，并像 `top` 一样 `collect_resources`。

外壳（`shell.rs`）新增：

```rust
/// What a landing page adds to the shell.
pub struct Landing<'a> {
    /// The page's policy as its `<meta>` carries it.
    pub meta_policy: String,
    /// The profile's scripts in order: resolved URL and how each loads.
    pub scripts: Vec<(String, crate::landing::Load)>,
    /// Query parameter names the theme script carries; empty for none.
    pub carry: &'a [String],
    pub footer: Vec<HeaderLink>,
}
```

`Page` 增加字段 `pub landing: Option<Landing<'a>>`。落地页的 `home` 与 `landing` 同时为 `Some`。`Home` 增加 `pub carry: bool`，hero 的每个 action 在为真时带 `data-rd-carry=""`。`page_view` 在 `landing.is_some()` 时与普通页面有以下不同：

1. `<html>` 在 `carry` 非空时增加 `data-rd-carry="<以空格连接的参数名>"`；
2. meta CSP 用 `landing.meta_policy`；
3. 按 §5.3 挂载脚本；
4. 头部改用新函数 `landing_header(site, slots)`：
   - 沿用 `header` 的 `<header>` 与 `.rd-header-bar` 外层；
   - 品牌区是 `<div class="rd-brand">{logo}<span>{name}</span></div>`；如果有 `header` slot，则输出 slot；
   - 没有链接、搜索、语言切换、模式切换，也没有窄屏链接行；
5. 不渲染 `search_dialog` 及其 `<noscript>`；
6. 页脚：有 `footer` slot 时输出 slot；否则 `landing.footer` 非空时输出 `<footer class="rd-site-footer …"><nav aria-label=footer_links>…</nav></footer>`；两者都没有时不渲染页脚；
7. 主体用 `home_layout`，与 `layout: home` 相同。

`#rd-status` 保留，供正文里代码块的复制按钮使用。

生成（`generation.rs`，位于 `generate_extended` 的页面闭包内，`generation.rs:1082-1180`）：

- `let profile = page.front.landing.as_deref().and_then(|name| config.landing.get(name));`
- `layout == Landing` 时构造 `Landing`：
  - `meta_policy = landing::meta_policy(profile)`；
  - 脚本：本地脚本经 `outputs` 映射为指纹 URL，外部脚本保持原样；
  - `carry` 取 `profile.map_or(&[], …)`；
  - `footer` 用 `header_links(analysis, &profile.footer, &analysis.landing_footers[name], site, &outputs)`。
- `profile.is_some()` 且 `landing::policy(profile) != CSP` 时，把 `(page_path(&page.route), landing::policy(profile))` 写入 `page_policies`。
- 最终闸门（`generation.rs:1421`）改为 `check_document_under(&html, profile)`，配置按页面输出路径查找。
- `404.html` 永远使用严格策略。

### 5.5 表单与嵌入

`html_check.rs`：

- 新增 `UrlRole::Frame`（`iframe src`）和 `UrlRole::FormTarget`（`form action`、`button`/`input` 的 `formaction`），在 `url_role`（`html_check.rs:68-81`）中登记。两者都不进入 `is_cross_origin` 的资源报错分支（`html_check.rs:738-750`）。
- `scan_tokens` 在 Markdown 片段模式下（`tags.is_none()`，即 `check()` 的路径）不对 `iframe` 应用 `FORBIDDEN`，改为：
  - 有 `srcdoc`，或者缺 `src` → 错误 `embedded-frame`；
  - 缺 `title` → 警告 `frame-title`；
  - `src` 作为 `Frame` 槽位记录。

  渲染面模式（`tags.is_some()`，即注册页和 slot）对 iframe 仍按 `FORBIDDEN` 报错。`frame` 元素在两种模式下都禁止。
- `pub fn check_document_under(html: &str, profile: Option<&landing::Profile>) -> Vec<String>`，`check_document(html)` 改为调用 `check_document_under(html, None)`。与 `check_document` 的差别：
  - 跨源 `<script src>`：`allows(script 来源, src)` 为真则放行；
  - `<iframe>` 从 `"style" | "iframe" | …` 分支中拆出：`profile` 存在且 `allows(frame 来源, src)` 为真则放行，否则照旧报为问题；
  - `profile` 存在时，检查跨源的 `form action`/`formaction`，`allows(form 来源, url)` 为假即为问题；`profile` 为 `None` 时不检查表单（D9）。

  `check_document` 的属性循环（`html_check.rs:844-854`）对这两种角色不做通用的跨源判定（与 `Link` 相同），由上面按元素的分支处理。

`markdown.rs`：把 `UrlRole::Frame` 映射为 `RefKind::Frame`，`UrlRole::FormTarget` 映射为 `RefKind::Form`（`markdown.rs:325` 与 `markdown.rs:725` 两处 match）。

`generation.rs` 的 `final_documents`（`generation.rs:658`）对 `document_references` 返回的每个 URL 槽位要求站内目标存在（`generation.rs:721-792`）。`UrlRole::FormTarget` 槽位在这个循环里跳过：普通页的表单只警告、原值不变（D9），它指向的站内端点（如 `/api/lead`）本来就不是站点发布的文件；落地页的站内表单目标已经由页面级的 `resolve_html_link` 校验。`UrlRole::Frame` 槽位只可能是绝对 http(s) 地址（页面级已拒绝其他形式），落入现有的 `has_scheme` 分支。

`content.rs` 的 `resolve_references`（`content.rs:2972`）增加参数 `landing: Option<Option<&landing::Profile>>`：外层 `None` 表示页面不是落地页，`Some(None)` 表示落地页但没有配置。调用方：Markdown 页面按自身 frontmatter 传入；注册页与 slot 传 `None`。新增两个分支：

- `RefKind::Frame`：
  - 不是落地页、没有配置，或者配置的 `frame` 为空 → 错误 `embedded-frame`，提示 iframe 只能出现在 `sources.frame` 列出该来源的落地页上；
  - URL 不是绝对的 http(s)，或者 `allows` 为假 → 错误 `frame-origin`，提示需要在配置 `<名>` 的 `sources.frame` 中列出该 origin；
  - 目标为 `Target::Keep`。
- `RefKind::Form`：
  - 不是落地页 → 警告 `form-blocked`，目标为 `Target::Keep`，输出不变；
  - 落地页但没有配置，或者 `form` 为空 → 错误 `form-origin`（“配置 `<名>` 没有列出表单目标”）；
  - 跨源且 `allows` 为假 → 错误 `form-origin`；
  - 空串或纯 `#…` → `Target::Keep`；
  - 站内地址 → 与同一页面上的 `<a href>` 同路解析（Markdown 页面走 `resolve_link`，按页面目录解析 `thanks.md`、`/thanks/` 等写法；渲染面走 `resolve_html_link`），加 base 并校验目标存在。

### 5.6 参数透传（`site.js`）

插入到 `enhance` 函数开头的公共变量之后（`site.js:15-18`），沿用文件现有的 ES5 风格（`var`、`function`）：

1. `names` = `<html data-rd-carry>` 按空格拆分；为空则不做任何事。
2. 用 `params = new URLSearchParams(location.search)` 取参数，`present` = `names` 中在 `params` 里存在且值非空的那些。`present` 为空时结束。
3. 对 `input[type="hidden"][data-rd-carry]`：`name` 在 `present` 中时，令 `input.value = params.get(name)`，URL 中的值覆盖原值。
4. 对 `a[data-rd-carry][href]` 和 `iframe[data-rd-carry][src]`：
   - `url = new URL(属性值, location.href)`；
   - 协议不是 http(s) 时跳过；
   - `present` 中 `url.searchParams` 还没有的参数，用 `set` 补上；
   - 有变化时，链接回写 `href`（`url.href`）；iframe 不改原元素的 `src`，而是 `cloneNode(true)` 复制一份、把副本的 `src` 设为 `url.href`，再替换原元素。直接改 `src` 在 frame 加载后会多一条会话历史（返回键先退回嵌入），Chromium 在懒加载刚开始时还会丢掉这次改动（M5 实测）；副本按首次加载处理，只加载一次，并遵循自身的 `loading`。文档建议页面下方的嵌入使用 `loading="lazy"`，并提醒持有该元素的脚本在页面加载后重新查找。
5. 不读写 cookie、`localStorage`、`sessionStorage`，不发起网络请求，不使用 `innerHTML`。

这段代码随 `site.js` 发给所有站点：没有 `data-rd-carry` 的页面在第 1 步就返回；`site.js` 内容变化带来的指纹文件名与引用变化在 NFR-4 的允许范围内。

### 5.7 策略投递

- `Generation`（`generation.rs:130`）增加 `pub page_policies: BTreeMap<String, String>`；`generate_extended` 与 `dev_shell*` 都要填充（dev 的外壳没有落地页时为空）。
- `publish.rs`：
  - `Manifest` 增加 `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")] pub page_policies: BTreeMap<String, String>`；
  - `add_manifest` 增加参数 `page_policies: &BTreeMap<String, String>`，调用方是 `main.rs:665`（build）与 `compile_proxy.rs:231`（测试用的组件站点发布），两处都传 `generation.page_policies`；
  - 新增 `Manifest::checked_page_policies(&self) -> Result<BTreeMap<String, String>, String>`，按 §2 的不变量校验，错误文本形如 `invalid page_policies: …; run \`rustify-docs build\` again`，同构于 `locale_not_found_files`。
- `preview.rs`：
  - `Site` 增加 `pub policies: BTreeMap<String, String>`，键是输出相对路径，用 `/` 分隔；
  - `respond` 增加参数 `csp: &str`；
  - `Answer::File(real)` 时，键为 `real.strip_prefix(&site.dir)` 转成的 `/` 路径，取 `policies.get(key)`，缺省用 `CSP`；
  - 404、405、400 和重定向一律用 `CSP`。
- `main.rs` 的 `preview()`：在 `locale_not_found_files` 之后调用 `checked_page_policies`，失败时用 `fail("preview", …)` 拒绝启动。
- `dev.rs`：
  - `Outcome::Built` 增加 `page_policies`；
  - `respond` 增加 `csp` 参数，仅在 `Answer::File(key)` 且 `key` 以 `.html` 结尾时查表；
  - 事件流（`dev.rs:791`）照旧用 `CSP`。
- 托管说明见 §5.8：
  - 能设置响应头的托管方，必须对 `page_policies` 中的路径发送对应策略，或者对这些路径不发 CSP 头；其余路径发严格策略；
  - GitHub Pages 这类不能设头的托管只依赖 meta，可以直接工作。

### 5.8 作者文档（`docs/docs-ssg.md`）

文档是英文作者参考，沿用现有文风。

- 新增「Landing pages」一节，内容依次为：
  - 用途与边界：哪些页面放宽、放宽了什么、哪些永远不放宽；
  - 配置表与 frontmatter；
  - 布局说明；
  - 原生表单：端点契约、感谢页、`'self'` 的作用；
  - iframe 嵌入，及 `loading="lazy"` 建议；
  - 参数透传；
  - 托管要求（按页头）；
  - 隐私与同意管理的作者责任；
  - 两个示例配置：
    - Google tag（GA4 + Ads）：来源清单引自 Google CSP 指南，注明 `www.google.<TLD>` 需要逐个列出，初始化代码以自托管文件给出，按 V-13 的观察修订；
    - 百度统计：来源按 V-13 的观察定稿。
- 增量修改其他章节：
  - 「`rustify-docs.toml`」：`[landing.profiles]`；
  - 「Frontmatter」表：`layout: landing`、`landing`，以及 `index`/`search`/`ai` 在落地页上的缺省值；
  - 「Raw HTML」：iframe 与表单规则；
  - 「What a build writes」：`page_policies`；
  - 「Previewing and hosting」：按页 CSP；
  - 「The default theme」：落地页布局；
  - 「Diagnostics」：新增规则。
- 注明这一功能需要包含它的 CLI 版本，即 0.1.32 之后的下一版。

### 5.9 诊断规则

| 规则 | 级别 | 位置 | 触发 |
| --- | --- | --- | --- |
| `landing-profile-name` | 错误 | 配置 | 配置名不合法 |
| `landing-source` | 错误 | 配置 | 来源不符合 D6 |
| `landing-script` | 错误 | 配置 | 脚本 `src` 既不是合法外部地址，也不是合法相对路径 |
| `landing-script-missing` | 错误 | 配置 | 本地脚本文件不存在 |
| `landing-carry` | 错误 | 配置 | 参数名不合法或重复 |
| `landing-profile-unused` | 警告 | 配置 | 没有页面引用该配置 |
| `landing-profile` | 错误 | 页面 frontmatter | 配置名未声明 |
| `landing-without-layout` | 错误 | 页面 frontmatter | 写了 `landing:` 但布局不是 landing |
| `embedded-frame` | 错误（沿用旧规则名） | 页面正文 | iframe 出现在非落地页，或者缺 `src`，或者有 `srcdoc`；注册页与 slot 中的 iframe |
| `frame-origin` | 错误 | 页面正文 | 落地页 iframe 的来源不在 `sources.frame` 中 |
| `frame-title` | 警告 | 页面正文 | iframe 缺 `title` |
| `form-origin` | 错误 | 页面正文 | 落地页表单目标未被放行 |
| `form-blocked` | 警告 | 页面正文 | 非落地页上的表单 |

## 6. 前端与交互

- **主路径**：广告点击 → 落地页（带 gclid/utm）→ CTA 锚点跳到表单，或者跳到站外。原生表单 POST 到端点后 303 回到感谢页，感谢页的配置脚本上报转化。嵌入表单的转化由服务方在 iframe 内负责。
- **状态**：
  - 透传：无参数时不改动任何元素；有参数时只改标注过的元素。
  - 无 JS：hero、正文、原生表单完整可用，透传与追踪不生效（NFR-5）。
- **可访问性**：
  - 地标为 header、main，有页脚链接时还有 footer；跳到正文的链接保留；
  - iframe 缺 `title` 时给警告；
  - `landing.spec.ts` 用 `tests/docs-ssg/a11y.ts` 的 axe 检查落地页。
- **响应式**：在 1280、768、375、320 px 四个宽度下都不出现横向滚动，复用 home 布局的规则。
- **主题**：没有模式切换控件；颜色跟随系统设置，或者读取 `mode.js` 已存的选择（`mode.js` 仍然加载）。
- **真实浏览器走查**：
  - 本地替身：M3 只跑安全门 V-15；完整走查 V-7 在 M5 随 V-10 的各项目运行一次；
  - 厂商脚本在线走查：M5 的 V-13。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| NFR-1 | `shell.rs:42` | 没有配置的页面，策略逐字节等于 `CSP` | 构建期断言失败 | `policy(None) == CSP`；只有 `landing + profile` 才进入 `page_policies` | V-1、V-3、V-15、V-10 |
| NFR-2 | 由 R-4 推出 | 只放行配置列出的来源；不可能注入响应头 | 构建报错，或 preview 拒绝启动 | D6 的来源语法；`checked_page_policies`；最终闸门与页面级判定一致 | V-1、V-5、V-15、V-7 |
| NFR-3 | `docs/docs-ssg.md` 预算；`docs_perf.rs:22` | 框架部分 gzip-9 ≤100,000 B；LCP ≤2.5 s、CLS ≤0.1（中位数，第三方除外） | 测试失败 | `async`/`defer`；复用 home 布局与主题资源 | V-3（重量）、V-12（页面内测量）、V-11（`site.js` 变大后文档页仍达标） |
| NFR-4 | `publish.rs:32-35`；用户 2026-10-21 确认的兼容边界 | 没有 `[landing]` 的站点：每页 CSP 等于 `CSP`，manifest 没有 `page_policies`、`schema_version` 为 1，既有功能不变；只有 `site.js` 及其指纹引用允许变化 | 测试失败 | 字段为空时省略；辅助方法的默认值与旧判定等价；落地页专属的外壳输出只在 `landing.is_some()` 时出现；`final_documents` 跳过表单目标 | V-3（既有 fixture 的策略与确定性断言）、V-5（空字段省略）、V-6（普通页表单的生成回归）、V-9（既有测试，含 artifacts、reliability 两组） |
| NFR-5 | 主题承诺 | 无 JS 时原生表单可以提交 | 测试失败 | 原生 `<form>`；透传只是增强 | V-7 |
| NFR-6 | 由 R-7 推出 | 不持久化，只复制列出的参数 | 测试失败 | §5.6 第 5 步 | V-15、V-7 |

安全项：

- **XSS**：配置值全部经 Leptos 属性转义输出；内联脚本与内联样式仍然禁止；透传只写 `value` 和 searchParams。V-15 验证含 `"><script>` 的参数值保持字面量。
- **头注入**：来源语法排除空白与控制字符；`checked_page_policies` 拒绝不可打印字符。
- **点击劫持**：落地页的响应头仍带 `frame-ancestors 'none'`。
- **第三方代码风险**：放行的脚本可以读取落地页的全部内容，包括表单输入。这是放宽的固有代价，只限于引用了配置的落地页；配置集中在一个文件中审计，文档写明这一点。

可观测性：本期新增的可观测面只有构建诊断（§5.9），以及 manifest 中的 `page_policies`。运行期由浏览器的 `securitypolicyviolation` 事件和控制台报告违规，`landing.spec.ts` 以此为断言依据。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 托管方对全站发送严格 CSP 头 | 所有落地页 | 追踪与线索丢失 | 表单提交无反应，统计无数据 | 浏览器控制台报违规 | 按 `page_policies` 配置按页头，或者对这些路径不发 CSP 头 | V-7 在 `docs-meta`（只用 meta）下通过；文档 §5.8 |
| 厂商新增或变更域名 | 使用该配置的页面 | 部分数据缺失 | 部分请求被拦截 | 控制台违规；平台告警 | 更新配置来源后重新构建 | V-13 |
| 线索端点不可用，或重定向到未放行的 origin | 该表单 | 线索丢失 | 端点错误页，或浏览器拦截重定向 | 端点方监控 | 端点修复；把重定向目标改为站内或已放行的来源 | V-7 覆盖“回到 `'self'`”的路径 |
| manifest 的 `page_policies` 损坏 | 该输出的 preview | 无 | preview 拒绝启动并提示重新构建 | `checked_page_policies` | `rustify-docs build` | V-5 |
| 旧 CLI 读取含 `[landing]` 的配置 | 该站点的构建 | 无 | unknown-field 错误 | CLI 报错 | 使用包含本功能的 CLI | 设计如此（D12），不另测 |
| 透传替换 iframe 导致二次加载 | 该嵌入 | 服务方的浏览量偏高 | 透传前已开始加载的嵌入（非懒加载，或已在懒加载范围内）再加载一次 | — | 文档建议 `loading="lazy"`；接受这一代价 | V-7 断言参数到达、会话历史不增加 |
| 配置的第三方脚本被劫持 | 引用该配置的落地页 | 表单输入可能泄露 | 无可见表现 | 本方无法检测 | 撤下该配置的脚本并重新构建 | 不在本期（SRI 已放弃，见 §0.5） |

- **发布顺序**：本期只合入 `main`。CLI 版本发布与官方站点不在本期（§0.5），没有灰度门。
- **回滚**：回退提交；输出没有不可逆点（§3）。

## 9. 验证

### 9.1 行为与接口验证

公共环境：

- 仓库根目录；`mise` 提供 Rust 1.99.0 与 Node；
- 浏览器检查需要 `npm ci && npx playwright install chromium firefox webkit`（Firefox 与 WebKit 只在 V-10 使用）；
- CLI 二进制由 `tests/docs-ssg/serve.sh` 或 `mbx build --locked -p rustify-docs-cli --profile docs-cli` 生成到 `target/docs-cli/rustify-docs`。

fixture 与替身的约定（两者都只实现本节列出的契约行为）。

`tests/docs-ssg/fixtures/landing/`（M2 建，M3 扩充）：

- `rustify-docs.toml`：
  - `url = "https://example.com/docs/"` 与 `base = "/docs/"` 两行相邻、顺序与 `content` fixture 相同，因为 `serve.sh` 的 perl 改写依赖这一格式；
  - `theme.logo`，以及一个 `nav.top` 链接，用来证明落地页不显示它；
  - 配置 `ads`：外部脚本 `http://127.0.0.1:4211/tag.js`（`async`）；本地 `static/landing/init.js`；五类来源都是 `http://127.0.0.1:4211`；`carry = ["gclid", "utm_source", "utm_campaign"]`；页脚链接 `privacy.md`；
  - 配置 `plain`：只有 `carry = ["gclid"]`。
- `content/` 下的页面：
  - `index.md`：home 布局；
  - `guide.md`、`privacy.md`：doc 布局；
  - `lp.md`：landing + `ads`；有 hero 和 features；有 POST 到 `http://127.0.0.1:4211/lead` 的原生表单（含 `<input type="hidden" name="gclid" data-rd-carry>`）；有 `<iframe data-rd-carry title="…" src="http://127.0.0.1:4211/embed">`；有 `<a data-rd-carry href="https://example.org/signup">`；
  - `thanks.md`：landing + `ads`；
  - `offer.md`：landing + `plain`，没有第三方资源，供性能测量。

`tests/docs-ssg/landing-third-party.mjs`：

- 启动参数：`<listedPort> <blockedPort> <sitePath>`。在两个端口上用 Node `http` 提供服务，并在 `GET /log` 返回记录到的事件 JSON。
- `/tag.js` 加载后消费 `window.fakeTagQueue` 中已排队的事件，并把 `push` 换成立即上报（与 gtag 的 `dataLayer` 同一模式；`tag.js` 是 `async`，`init.js` 是 `defer`，两者的执行顺序不确定）。每个事件：
  - 依次通过 `fetch`、`sendBeacon`、`new Image()` 上报到 listed 端口的 `/collect` 与 `/pixel.gif`；
  - 尝试从 blocked 端口加载一个脚本和一个 iframe，用来产生违规事件。
- `POST /lead`：记录请求体，然后 `303 Location: <请求头 Origin><sitePath>thanks/`。
- `GET /embed`：一个含表单的 HTML 页面，并记录查询串。

`init.js` 执行 `(window.fakeTagQueue = window.fakeTagQueue || []).push("page_view")`；当 `<main>` 中有 `[data-conversion]` 元素（`thanks.md` 中有一个）时，再 `push("conversion")`。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | R-2、R-3、R-4、NFR-1、NFR-2：配置解析的接受与拒绝表（§5.1 第 1–4 条、未知键）；`parse_source` 的接受与拒绝表（D6 的每一类）；显式来源与外部脚本 URL 中的 IPv6 字面量（如 `http://[::1]:4211`）和 `127.0.0.1` 以外的 IP 字面量分别报 `landing-source`、`landing-script`，`allows` 对这类 URL 返回假；`policy(None) == CSP`；`ads` 配置的合成串逐字节相等；`allows` 的匹配表（通配、端口、scheme）；frontmatter 解析 `layout: landing` 和 `landing:`；`indexed`/`searchable`/`discoverable` 的默认值与覆盖 | `mbx test --locked -p rustify-docs --lib landing::`；`mbx test --locked -p rustify-docs --lib config::`；`mbx test --locked -p rustify-docs --lib frontmatter::`；`cargo fmt --all -- --check`；`mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` | M1 |
| V-2 | R-1、R-2：`landing-profile`、`landing-without-layout` 的定位；`landing-profile-unused`；`landing-script-missing`；落地页不进侧栏；落地页可以省略 description；H1 并入 hero；页脚链接解析出 `broken-link` | `mbx test --locked -p rustify-docs --lib content::tests::landing`（新测试函数统一以 `landing_` 开头） | M1 |
| V-3 | R-1、R-3、R-7、NFR-1、NFR-3、NFR-4：在 fixture 上生成后检查：<br>· `lp/index.html` 的 meta 等于 `meta_policy(ads)`；脚本按顺序带 `async`/`defer`；`init.js` 以 `assets/files/init.<hash>.js` 发布；<br>· 没有 `data-rd-search`、`rd-header-links`、`rd-sidebar`、`data-rd-mode-switch`；品牌区不是链接；页脚有 Privacy 链接；<br>· `<html data-rd-carry="gclid utm_source utm_campaign">`；hero actions 带 `data-rd-carry`；<br>· `noindex`；不在 sitemap、搜索数据、`llms.txt` 中；<br>· `guide/`、`index`、`404` 的 meta 与 `meta_csp()` 逐字节相同；<br>· `page_policies` 只包含 `lp/`、`thanks/` 两项；<br>· 没有 `[landing]` 的 `content` fixture：`Generation.page_policies` 为空，每页 meta 等于 `meta_csp()`，没有元素带 `data-rd-carry`；<br>· 两次生成逐字节相同；<br>· `offer/` 的 HTML + 站点 CSS + `mode.js` + `site.js` gzip-9 合计 ≤100,000 B；<br>· 已有的 generate、i18n、search、ai 测试照常通过 | `mbx test --locked -p rustify-docs --test landing --test generate --test i18n --test search --test ai`；`mbx test --locked -p rustify-docs --lib shell::`；`cargo fmt --all -- --check`；`mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` | M2 |
| V-4 | R-3、NFR-2：`check_document_under`：跨源脚本在有配置时放行、无配置时拒绝；`check_document(html) == check_document_under(html, None)` | `mbx test --locked -p rustify-docs --lib html_check::` | M2 |
| V-5 | R-8、NFR-2：manifest 的 `page_policies` 读写；空字段省略；`checked_page_policies` 拒绝换行、非 `.html`、未列出的文件、错误首尾；preview：落地页的头等于 manifest、`guide/` 的头等于 `CSP`、404 用 `CSP`、manifest 损坏时拒绝启动；dev：落地页 HTML 的头是按页策略；既有 reliability、dev 测试照常通过 | `mbx test --locked -p rustify-docs --lib publish::`；`mbx test --locked -p rustify-docs-cli --test landing --test reliability --test dev`；`mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings` | M2 |
| V-6 | R-5、R-6：<br>· 片段：iframe 记为 `Frame` 槽位；`srcdoc` 或缺 `src` → `embedded-frame`；缺 `title` → `frame-title`；<br>· 渲染面：iframe 仍报错；<br>· 页面级：iframe 在 doc 页 → `embedded-frame`；落地页上来源已列出 → 通过，未列出 → `frame-origin`；<br>· 表单：非落地页 → `form-blocked`，输出不变；落地页的跨源目标已列出 → 通过，未列出 → `form-origin`；`plain` 配置下的表单 → `form-origin`；站内 `action` 加 base 并校验目标；<br>· 最终闸门对表单与 iframe 的判定与页面级一致；<br>· 完整生成回归（`tests/landing.rs` 在 fixture 副本上改写页面）：doc 页的 `<form action="/api/lead">` 与 `<button formaction="/api/x">` 指向站点不发布的地址 → 只有 `form-blocked` 警告，生成成功，输出中 `action`/`formaction` 保持原值；落地页表单指向缺失的站内页面 → `broken-link`，生成失败；<br>· fixture 生成照常通过 | `mbx test --locked -p rustify-docs --lib html_check::`；`mbx test --locked -p rustify-docs --lib markdown::`；`mbx test --locked -p rustify-docs --lib content::tests::landing`；`mbx test --locked -p rustify-docs --lib content::tests::form`（非落地页表单的新测试函数以 `form_` 开头）；`mbx test --locked -p rustify-docs --test landing`；`cargo fmt --all -- --check`；`mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` | M3 |
| V-7 | R-1、R-3–R-8、NFR-2、NFR-5、NFR-6，浏览器 spec `landing.spec.ts` 的全部断言（含 V-15 的 `@policy` 测试；每个测试复制 fixture、改写端口与 base、构建；`docs-meta` 下用 `static-host.mjs`，其余项目用 `preview --port 0`）：<br>· `lp/` 上 listed 端口收到 `page_view`，以及 `fetch`、`beacon`、`pixel` 三种上报，listed 来源没有违规；blocked 来源的脚本与 iframe 各产生一条违规；<br>· `guide/` 上没有 tag 请求；在 preview 下，`guide/` 的响应头等于 `CSP`，`lp/` 的响应头等于 manifest 中的策略；<br>· 打开 `lp/?gclid=g1&utm_source=s&x=1`：隐藏字段、CTA、外链和 iframe `src` 都带上 `gclid` 与 `utm_source`，都不带 `x`；含 `"><script>` 的值保持字面量；无参数时这些元素都不变；不写 storage 和 cookie；嵌入带着参数加载，会话历史条数与不带参数时相同；<br>· 表单提交，两个独立浏览器上下文：JS 开时 `/lead` 收到含 `gclid` 的请求体 → 303 → `thanks/` 加载，listed 端口收到 `conversion`；JS 关时 `/lead` 收到原生 POST → 303 → `thanks/` 内容完整显示，listed 端口没有收到 `/collect`、`/pixel.gif` 上报，也没有 `page_view`、`conversion` 事件；<br>· 布局：1280、768、375、320 px 下没有横向滚动；没有搜索、导航、模式切换；页脚链接可以点击；axe 没有违规 | 随 V-10 运行：`docs-root`、`docs-subpath`、`docs-meta`、`docs-webkit`、`docs-firefox` 各收集一次 `landing.spec.ts`，不单独再跑 | M5 收口（M3 只跑其中的 V-15） |
| V-8 | R-9：`docs/docs-ssg.md` 新增的每段配置与页面示例，都能在临时项目中通过 `check` | `mbx build --locked -p rustify-docs-cli --profile docs-cli`，然后把每个示例复制到 `$TMPDIR` 下的新项目，运行 `target/docs-cli/rustify-docs check --config <项目>/rustify-docs.toml`；错误为 0，警告只能是文档中写明的那些 | M4 |
| V-9 | 全部 R、NFR 的主机端全量回归 | `cargo fmt --all -- --check`；`mbx test --locked -p rustify-docs`；`mbx test --locked -p rustify-docs-cli`；`mbx clippy --locked -p rustify-docs --all-targets -- -D warnings`；`mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings`；`mbx build --locked -p rustify-docs-cli --profile docs-cli` | M5 收口 |
| V-10 | R-1、R-8、NFR-1、A-3 以及 V-7 全部断言的浏览器矩阵与回归。`landing.spec.ts`（V-7）在 `docs-root`、`docs-subpath`、`docs-meta`、`docs-webkit`、`docs-firefox` 中各运行一次；`docs-dev` 只收集 `dev*.spec.ts`（`playwright.docs.config.ts:178-180`），这里只算既有回归，dev 的按页响应头由 V-5 覆盖。依次运行，一次一个项目：<br>`npx playwright test -c playwright.docs.config.ts --project=docs-root`<br>`npx playwright test -c playwright.docs.config.ts --project=docs-subpath`<br>`npx playwright test -c playwright.docs.config.ts --project=docs-meta`<br>`npx playwright test -c playwright.docs.config.ts --project=docs-dev`<br>`npx playwright test -c playwright.docs.config.ts --project=docs-webkit`<br>`npx playwright test -c playwright.docs.config.ts --project=docs-firefox`<br>`npx playwright test -c playwright.docs.config.ts --project=components-subpath`<br>`npx playwright test -c playwright.docs.config.ts --project=official-subpath` | 同左；每次运行前确认 `target/docs-cli/rustify-docs` 存在（`serve.sh` 会重建）。Playwright 自带的 Firefox 在本机不能启动时（M5：1543 版在 macOS 27.0.1 上报 “Could not find profile folder”），用只把该项目改为 `channel: "moz-firefox"` 的包装配置，经 WebDriver BiDi 驱动已安装的稳定版 Firefox 运行同一项目；BiDi 驱动不实现 `colorScheme`、`reducedMotion`、`javaScriptEnabled` 仿真，依赖它们的测试不作为 Firefox 证据，JS 关的表单改用 `javascript.enabled=false` 首选项单独验证 | M5 收口 |
| V-11 | NFR-3：已有特性与性能门禁不回退（`site.js` 变大后文档页仍在预算内）。`artifacts`、`reliability` 两组只是 V-9 已全量运行的测试子集（`xtask/src/docs.rs:62-80`），直接引用 V-9 的通过证据，不再单独运行 | `mbx xtask docs-verify --group features`；`mbx xtask docs-verify --group performance` | M5 收口 |
| V-12 | NFR-3：落地页 `offer/` 的 LCP ≤2.5 s、CLS ≤0.1（5 次冷启动取中位数，冻结合同，在页面内测量） | `sh tests/docs-ssg/serve.sh landing subpath 4212`（后台运行），然后 `node tests/docs-ssg/harness/measure-load.mjs --origin http://127.0.0.1:4212 --page /rustify-ui/offer/ --samples 5 --out <记录目录>/landing-load.json` | M5 收口 |
| V-13 | A-1、A-2、R-9：真实厂商脚本在线验证，由 Agent 在联网环境执行 | 用 §5.8 的两个文档示例配置构建临时站点，`preview` 后在 Chromium 中打开落地页，监听 `securitypolicyviolation`、`pageerror` 与网络请求 10 秒。ID（§4）：Google 先用测试 ID `G-TEST000000`、`AW-000000000`，没有触发统计请求时改用用户提供的真实 ID；百度统计必须用用户提供的获授权站点 ID（全零测试 ID 返回 0 B 脚本）。逐厂商判据：脚本响应 200 且响应体非空；没有初始化运行错误；观察到本次配置应触发的统计请求实际发出（如 GA4 的 `collect`、百度的 `hm.gif`；Ads 以示例实际调用的 `config`/`event` 为准）；记录实际请求的域名和对应的 CSP 结果，文档列出的来源没有违规。空响应、没有统计请求、初始化报错都不算通过；只要求请求发出，不要求平台计入。有违规时补进文档示例，再重跑 V-8 和该厂商的 V-13。缺 ID 或无法联网时该厂商记为缺口，对应假设保持开放，M5 不记完成；结论只覆盖实测触发过的厂商功能 | M5 收口 |
| V-14 | R-9 的角色模拟，由 Agent 执行：<br>· 角色：增长/投放工程师，会写 Markdown、HTML，了解 CSP 概念，不会 Rust；<br>· 资料：只读 `docs/docs-ssg.md`，不读源码与本计划；<br>· 权限：`target/docs-cli/rustify-docs`、Node、Chromium；<br>· 任务：在空目录用 `rustify-docs new` 建站，按文档做出一个落地页，包含配置（本地替身 tag，或在线 gtag）、原生表单（替身端点）、gclid 透传和感谢页；`check`、`build`、`preview` 都通过；在浏览器中确认表单带着 gclid 提交、没有违规 | 判据：不读源码即可完成。记录每个卡点，并在本期内修订文档；结果标注为“角色模拟” | M5 收口 |
| V-15 | R-4、R-7、R-8、NFR-1、NFR-2、NFR-6 的浏览器安全门（`landing.spec.ts` 中带 `@policy` 标签的测试，是 V-7 的子集）：<br>· `lp/` 上 listed 来源 `http://127.0.0.1:4211` 的脚本、`fetch`、`beacon`、`pixel`、iframe 都没有违规；blocked 来源的脚本与 iframe 各产生一条违规；<br>· `guide/` 上没有 tag 请求；在 preview 下，`guide/` 的响应头等于 `CSP`，`lp/` 的响应头等于 manifest 中的策略；<br>· 打开 `lp/?gclid=g1&utm_source=s&x=1`：只复制列出的参数，不带 `x`；含 `"><script>` 的值保持字面量；不写 storage 和 cookie | `npx playwright test -c playwright.docs.config.ts --project=docs-subpath landing.spec.ts --grep @policy` | M3（安全风险门，只跑这一组）；其余 V-7 断言在 M5 随 V-10 运行 |

### 9.2 集成与回归

- 直接受影响模块的既有测试已排进 V-3、V-5。全量回归是 V-9 到 V-11，完整浏览器走查是 V-7（随 V-10 运行），都只在 M5 运行；M3 只跑安全门 V-15。
- 如果收口修复改动了 `shell.rs`、`generation.rs` 或 `site.js`，重跑 V-3 和受影响的 V-10 项目（其中的 V-7 随之重跑）。

### 9.3 NFR 与故障注入

- 头注入、manifest 损坏：V-5。
- 未放行来源被拦截：V-15 的 blocked 端口（M3）；M5 由 V-7 在各引擎下覆盖。
- 无 JS：V-7。
- 性能：V-3（重量）、V-12（页面内测量）。

### 9.4 真实环境与 UI

- 本地替身：V-15（M3 安全门）、V-7（M5，随 V-10）。
- 真实厂商：V-13。
- 角色走查：V-14。
- 真实广告账户的转化计入不在本期退出条件中（§4）。

### 9.5 静态校验

`cargo fmt --all -- --check`，以及两个 docs 包各自的 clippy，已分别写进 V-1、V-3、V-5、V-6、V-9。

## 10. 里程碑与验收安排

全期只有一个阶段，M5 是阶段收口。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 配置与页面模型 | 无 | ★`landing.rs`（§5.1 的类型与校验、§5.2 的 `policy`/`meta_policy`/`allows`）；`lib.rs` 注册模块；`config.rs` 的 `RawConfig.landing` 与 `Config.landing`（同构于 `RawNav`/`RawLink`，位于 `config.rs:22-160` 和 `parse`）；`frontmatter.rs` 的 `Layout::Landing`、`Front.landing`、辅助方法；`content.rs` 的交叉校验、本地脚本资源、`landing_footers`、未用配置警告、`has_hero` 替换（§5.4）。单人串行，共享文件是 `content.rs` | V-1、V-2；回写「实施进度」 |
| M2 | 落地页渲染与策略投递 | V-1、V-2 | 见 §10.1 的 T-1、T-2：§5.3、§5.4 的外壳与生成部分、§5.7；★`tests/docs-ssg/fixtures/landing/`（M2 部分，不含表单与 iframe）；★`crates/rustify-docs/tests/landing.rs`；★`crates/rustify-docs-cli/tests/landing.rs`；`Cargo.toml` 的 `[[test]]` | V-3、V-4、V-5；回写「实施进度」 |
| M3 | 表单、嵌入与参数透传 | V-3、V-4 | 见 §10.1 的 T-3–T-5：§5.5、§5.6；fixture 补上表单与 iframe；★`landing-third-party.mjs`；★`landing.spec.ts`；`serve.sh` 新增 `landing` 来源（照搬 `content` 分支：复制 fixture、改写 base 和 url、构建、preview） | V-6、V-15；回写「实施进度」 |
| M4 | 作者文档 | §5 的接口与 §5.9 的规则名（已冻结），可与 M3 并行；V-8 需要 M3 的产物 | `docs/docs-ssg.md` 按 §5.8 修改。负责人只写这一个文件 | V-8；回写「实施进度」 |
| M5 | 阶段收口 | M1–M4 全部完成；V-13 需要用户提供的厂商 ID（§4） | 主 agent 统一负责：先跑 V-9、V-11；然后 V-10，逐个项目串行，V-7 在其中运行；然后 V-12；然后 V-13、V-14。收口中发现的文档缺口在 `docs/docs-ssg.md` 中就地修订，再重跑 V-8 | V-7、V-9 到 V-14；A-1、A-2、A-3 各自解除，或者按 §0.1 的路线处理并记录（缺证不算解除）；回写「实施进度」 |

提交点：M5（全部完成）提交一次。里程碑只有 5 个，且只有一个阶段收口，所以中间不设阶段提交。

### 10.1 内部任务

| 任务 ID | 所属里程碑 | 交付物/影响路径 | 前置成果 | 检查 ID |
| --- | --- | --- | --- | --- |
| T-1 | M2 | `shell.rs`（`Landing`、`landing_header`、脚本、透传属性、页脚）、`generation.rs`（组装、`page_policies`、最终闸门、辅助方法）、`search.rs`、`ai.rs`、`html_check.rs` 的 `check_document_under`（只做脚本部分）、fixture（M2 部分）、`tests/landing.rs`。负责人独占 `shell.rs`、`generation.rs`、`html_check.rs` 和 fixture | V-1、V-2 | V-3、V-4 |
| T-2 | M2 | `publish.rs`、CLI 的 `main.rs`、`preview.rs`、`dev.rs`、`crates/rustify-docs-cli/tests/landing.rs` | `Generation.page_policies` 字段已由 T-1 加入（可以先是空映射），契约见 §5.7 | V-5 |
| T-3 | M3 | `html_check.rs` 的角色与片段规则，以及最终闸门中的表单与 iframe 部分；`markdown.rs`；`content.rs` 的 `resolve_references`；`generation.rs` 的 `final_documents` 跳过 `FormTarget`；`tests/landing.rs` 的表单生成回归；fixture 补上表单与 iframe | V-3、V-4 | V-6 |
| T-4 | M3 | `site.js` 的透传（§5.6） | M2 已在页面上输出 `data-rd-carry` | V-15；完整断言在 M5 的 V-7 |
| T-5 | M3 | `landing-third-party.mjs`、`landing.spec.ts`（安全门测试标 `@policy`）、`serve.sh` 的 `landing` 来源 | 可以与 T-3、T-4 并行编写；运行 V-15 需要 T-3、T-4 完成 | V-15；完整断言在 M5 的 V-7 |

T-1 与 T-2 写入的文件互不重叠，T-2 只依赖 §5.7 的字段契约。T-3、T-4、T-5 写入的文件也互不重叠：fixture、`tests/landing.rs` 和 `generation.rs` 的 `final_documents` 在 M2 完成后归 T-3，spec 和替身归 T-5。浏览器运行（M3 的 V-15、M5 的 V-10）由主 agent 统一执行，一次一个项目。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| A-1：gtag 从外部文件初始化 | 文档示例与 Google tag 的可用性 | 实施 agent 做 V-13；若不成立，按 §0.1 的路线处理；缺证不算解除 | M5 | 否 |
| A-2：百度统计的来源清单 | 文档示例 | 实施 agent 用用户提供的站点 ID 做 V-13；缺证不算解除 | M5 | 否 |
| A-3：各引擎对重定向执行 `form-action` 的行为 | 感谢页跳转 | 实施 agent 在 V-10 运行 webkit、firefox 项目 | M5 | 否 |
| V-13 需要联网和厂商 ID | 在线核对 | 用户在 M5 收口前提供百度统计站点 ID，以及测试 ID 不触发请求时所需的 GA4/Ads ID（§4）；缺 ID 或不能联网时该厂商记为缺口，A-1、A-2 保持开放，M5 不记完成，并在交付报告中写明 | M5 | 不阻塞开工；阻塞 M5 完成 |
| 托管方全站发送严格头 | 落地页失效 | 文档 §5.8；V-7 在 `docs-meta` 下覆盖只用 meta 的情况 | M4 | 否 |
| 第三方脚本可以读取落地页内容 | 隐私 | 这是设计的固有代价，只限落地页（ADR-1），文档写明 | M4 | 否 |

- 最终状态：**Ready**。
- 定级理由：范围、配置接口、来源语法与匹配、策略合成、投递方式、表单目标在各道检查中的处理、诊断规则和验证都已定义；用户 2026-10-21 确认了三项功能取舍和兼容边界（透传留在 `site.js`，NFR-4）。剩下的三个假设只影响文档示例和引擎兼容说明，每种结果都有对应路线，不影响开工；V-13 所需的厂商 ID 由用户在 M5 前提供，只影响 M5 能否记完成。外部端点与平台不属于本方交付。

## 12. 已知坑与历史教训

- 片段缓存键不含 frontmatter（`content.rs:894-904`）：任何依赖配置的检查都不能放进片段级扫描（D8）。
- `target/` 是 mbx 托管的符号链接，可能被 gc 清空（项目记忆 mbx-prunes-managed-target）：浏览器运行前确认 `target/docs-cli/rustify-docs` 存在；证据存到 `docs/plan/DOCS-LANDING.records/`，不要放在 `target/` 下。
- mbx 按内容判断是否重编，`touch` 不算修改（项目记忆 build-through-mise-exec-mbx）：确认 `site.js` 已重新打包，要看的是产物里的指纹文件名变化。
- Playwright 每次只跑一个项目，不能并发，否则会互相清空 `test-results/`（`CLAUDE.md`）。
- 性能数字只在页面内测量，Playwright 的计时只反映测试框架开销（`CLAUDE.md`、项目记忆 browser-timings-are-harness-not-runtime）。
- `cargo fmt` 会重排代码行：脚本化的查找替换必须断言旧文本确实存在（`CLAUDE.md` Scripted edits）。
- 两个 docs 包必须各自单独 test 和 clippy，不能放进 workspace 联合命令（`CLAUDE.md` Commands）。
