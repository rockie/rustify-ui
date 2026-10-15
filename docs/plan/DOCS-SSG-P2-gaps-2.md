# DOCS-SSG-P2-gaps-2 · Rustify UI · 诊断代码片段在 dev 面板逐字显示（DOM · 开发预览 · 本地固定版本）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-15 · `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` · dirty；未提交项只有既有台账、评审/差异报告、验证证据、fixture 日志与缓存，产品源码与 HEAD 一致。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P2（第 2 期）**；补齐[原计划](DOCS-SSG-P2.md)及[首个增量计划](DOCS-SSG-P2-gaps.md)已交付的诊断路径。增量来源：[差异复审报告](DOCS-SSG-P2.prd-dev-gaps.r2.md)的 G-06 及用户 2026-10-15 决定 D-05（在 P2 现在补齐）。
>
> 本期交付：**诊断消息与修复建议中的代码片段在任何内容下都无歧义；dev 面板显示的文字与 text/JSON 在去掉片段分隔后逐字一致，含字面量反引号；以本地 R″/C″ 固定配对交付。**
> 本期独特职责：给全部自有诊断模板一个统一的代码片段写法，覆盖代码片段及普通文本中的作者值，并让面板按同一规则读取；逐字保证限于这些受控内容，外部原文边界见 D4。不新增诊断、规则或字段。
> **顶层排除：不重写第三方或编译器原文（serde、YAML 解析器、rustc、IO）；不改 JSON 字段/schema、面板布局与交互；原 M6/M7 缺证、U-01–U-03 及 P3/P4 功能不在本计划内。**

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

- 最近更新：2026-10-15T15:37:33+08:00
- 当前进度：2/2 个里程碑完成
- 当前状态：PG-01 已修复并验证；新 R‴/C‴ 固定安装出口完成，待独立代码复审
- 最近完成：M2 · 本地固定配对与真实出口
- 下一步：代码评审：`review-code docs/plan/DOCS-SSG-P2-gaps-2.md`（由用户发起）
- 当前阻塞：无
- 代码基线：R‴ `6942ec0d23812ecda61492d9db5c0046f481edfc`；C‴ `b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d`（0.1.19）；原 R″ `e20daf7f` / C″ `758def9c`（0.1.18）配对保留

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-15T15:37:33+08:00 | PG-01 消息截断已修复；完整源门及独立两图、工作树 docs-dev 32/32 通过，安装出口见 M2 | [M1 记录](DOCS-SSG-P2-gaps-2.records/M1.md) | `e20daf7f`；修复 `6942ec0d` |
| M2 | 已完成 | 2026-10-15T15:37:33+08:00 | C‴ 五文件审计与真实安装、六阶段/24 视图、CLI 37/37 和安装入口 docs-dev 32/32 通过；身份 SHA 稳定 | [M2 记录](DOCS-SSG-P2-gaps-2.records/M2.md) | C″ `758def9c`；证据 `6c011792`；修复 `b31e28b7`（C‴）；修复 `925b3b57`（证据） |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

本表沿用 PRD 工作号，只规划 G-06 的确定增量；标「上游编号」的行只登记原计划出处，不是本计划的里程碑或检查。WORK-F13 终验 P2，WORK-F15 终验 P3；本计划完成不关闭原 M6/M7 缺证或整 P2 终验。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F13 | 功能 | PRD WORK-F13 行为「出错」、AC-2、AC-4 | 浏览器浮层与终端显示同一诊断，含修复建议 | §5.3；V-3/V-4 | 已确认；P2 终验 |
| WORK-F15 | 功能 | PRD WORK-F15 诊断格式、AC-2 | 每条诊断带说明与修复建议，机器可读格式不变 | §5.1/§5.2；V-1/V-2 | 已确认；整条终验 P3 |
| WORK-N6 | 可靠性 | PRD WORK-N6 AC-2、AC-3 | 拒绝语义、位置、旧产物与 last-good 保持 | §5；V-1/V-3/V-4 | 已确认；本期回归 |
| G-06 | 差异 | [差异复审报告 G-06](DOCS-SSG-P2.prd-dev-gaps.r2.md#g-06--dev-面板把建议中字面量自带的反引号当作代码分隔符显示的修复建议与-checkbuild终端不一致--c) | 面板把内容自带的反引号当作分隔符，丢字符并颠倒排版 | §1.1；V-1/V-3 | C；D-05 决定现在补齐 |
| D-05 | 用户决定 | 同报告 §4，用户 2026-10-15 | 在 P2 补齐：面板显示的 suggestion/message 与 text/JSON 逐字一致（含字面量反引号），新 R/C 配对安装复验 | 全文；V-4 | 已确认 |
| C-1 | 项目约束 | `AGENTS.md`「前端 UI 开发与验收」；[dev 面板 brief](../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md) | UI 改动先用 impeccable 读设计上下文，再在真实浏览器验收；单测/build 不替代浏览器验收 | §6；V-3/V-4 | 已确认 |
| C-2 | 版本/工具链 | `mise.toml`；`CLAUDE.md`「Toolchain」「Browser tests」；首个增量计划的 R/C 配对流程 | mbx、`--locked`、docs 两包分包；库 R″ → CLI C″ 两个真实提交，C″ 只改五个配对文件；Playwright 单 project 串行 | §5.5、§9.3；V-4/V-5 | 已确认 |
| C-3 | 工作范围 | PRD D16；差异复审报告 §5、§6 | U-01–U-03、原 M6/M7、P3/P4 不移入；不推送、不发布 | §0.5 | 已确认 |
| M6 | 上游编号 | [原计划](DOCS-SSG-P2.md) §10 | 原期完整本地/设备/性能验收与缺证 | §0.5 | 原计划承接，本计划排除 |
| M7 | 上游编号 | 原计划 §10 | 原期公开安装/发布与回滚，按 D16 留 P4 | §0.5 | 原计划承接，本计划排除 |
| V-11 | 上游编号 | 原计划 §9.1 | 原期真实浏览器/设备矩阵 | §0.5 | 原计划承接，本计划排除 |
| V-12 | 上游编号 | 原计划 §9.1 | 原期正式性能与样本合同 | §0.5 | 原计划承接，本计划排除 |
| V-15 | 上游编号 | 原计划 §9.1 | 原期公开来源/Pages/回滚 | §0.5 | 原计划承接，本计划排除 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 代码片段写法 | 非空且不含反引号时保持单反引号；空内容用双反引号围栏、内侧两个 ASCII 空格；含反引号时用更长围栏、内侧各加一个空格（§5.1） | Rustify 自有逐字片段读法，保留空格和 LF/CRLF；不承诺与 CommonMark 等价。仅非空且不含反引号的代码值、以及不含反引号的普通作者值保持原字节输出；空代码值是明确例外 | G-06/D-05 的逐字目标；`report.rs` 原样输出；用户 2026-10-15 确认保留逐字目标并撤掉 CommonMark 等价承诺 |
| D2 | 生产方覆盖 | 自有诊断中所有动态代码片段用 `code_span` 编码完整内容；原为普通文本的作者值统一用 `literal`：含反引号时转成完整代码片段，否则原样 | 普通输入文字和样式不变；含反引号的普通作者值整段等宽显示。审计覆盖固定前后缀、多插值和未包围的作者值，原约 150 处统计只是下界 | ADR-1；§1.1；用户 2026-10-15 选择含反引号时显示为完整代码片段，未选新增普通文本转义规则 |
| D3 | 写法函数位置 | `rustify_docs::diagnostic_text` 无条件导出 `code_span`/`literal`，供 pipeline、独立 site-ssr/hydrate 与 CLI；`rustify-docs-syntax` 一份供 `site!` 解析错误；CLI 测试断言两者对同一输入表输出相同 | docs 函数只依赖 std，不依赖 `Issue` 或 pipeline；不新增 crate/特性依赖边，不改模板锁的包集合；以受控重复换取分层不变 | `crates/rustify-docs/Cargo.toml` 只以 dev-dependency 引用 syntax；`rust_site.rs` L343–356 把 syntax 错误转为诊断 |
| D4 | 外部原文 | serde、YAML 解析器、rustc、IO 等原文不改写不解析；面板对它们用同一读法 | 未配对的反引号原样显示；它们自己的成对反引号仍按代码显示 | 首个增量计划“不解析错误文本”；§5.3 |
| D5 | 面板读取 | 按 §5.1 匹配同长围栏、只剥除长围栏的各一个填充空格；保留原文空格与换行，空内容读回空串，未配对串原样显示 | 复用既有 code 样式，普通作者值按 D2 分类；其余视觉/交互、终端排版、JSON 结构不变 | `dev.js` `prose()` L208–218；C-1；D1/D2 |
| D6 | 固定交付 | 先提交库 R″，再制作 CLI C″（0.1.18）并本地真实安装复验 | 与已交付的 R′/C′ 同一纪律；旧版本证据保留原版本 | C-2；`docs/plan/DOCS-SSG-P2-gaps.md` §5.4 |

### 0.3 ADR-lite

#### ADR-1：诊断代码片段的无歧义表示

- 状态：Accepted
- 背景与驱动：D-05 要求面板显示与 text/JSON 逐字一致。现有诊断文字由模板直接写 `` `{x}` ``，内容含反引号时分隔符与内容无法区分，面板只能猜。JSON 已有 `schema_version: 1` 的 message/suggestion 字符串，属于对外契约。
- 备选：
  1. 模板统一用无歧义写法（D1/D2），面板按同规则读：改动面大，但机械；JSON 字段不变；含反引号的内容与空代码值改变字符串编码。
  2. 只改“内容可能来自作者或外部”的插值点：改动少，但每处要人工判断来源，漏判会悄悄留下同类缺陷，审计无法机械化。
  3. 在诊断或 SSE 中增加结构化片段字段：面板不再猜，但生产方仍要逐处改成结构化拼装，并且改变对外 JSON 或另开一套 dev 协议，比方案 1 更重。
  4. 面板不再格式化反引号，直接显示原文：最省事，天然逐字一致；但推翻已确认的面板表面（名字用系统等宽），对所有诊断都是视觉退化，需要新的设计决定。
- 决策：方案 1。该方案保持逐字目标、JSON 结构和面板布局/交互；作者值按 D2 使用既有等宽样式。
- 正面后果：自有模板中动态代码片段与普通作者值均无歧义；非空无反引号的代码值及无反引号的普通作者值保持原输出，既有精确断言作为该范围的回归基线。空代码值用独立编码，单列精确新期望。
- 负面/中性后果：原约 150 处直接插值之外，还须审计前后缀和普通作者值；含反引号的普通作者值改用既有等宽样式。`site!` 解析错误保留一份重复实现，靠一致性测试约束。自有读法不等价于通用 Markdown，作者参考须给出完整规则。
- 重新评估触发：出现必须由面板区分片段类型（例如可点击路径）的需求时，改走方案 3。

### 0.4 职责与事实所有权

|  | 诊断生产方（docs、CLI、syntax） | dev 面板（`dev.js`） | 外部工具原文 |
| --- | --- | --- | --- |
| 拥有 | 片段边界的正确写法；message/suggestion 的全部文字 | 按同一读法显示，保持换行与断行 | 自身文字 |
| 不拥有 | 面板排版 | **不改写、不补全、不猜测诊断文字** | 本计划不改写其内容 |

一句话：哪些字是代码片段由生产方写清，面板只按规则读；外部原文原样进入同一读法。

### 0.5 明确不在本期

- 改写或解析 serde、YAML 解析器、rustc、IO 原文中的反引号——它们自身就有歧义，本计划只保证原样显示其中未配对的反引号。
- JSON 字段、`schema_version`、终端排版与面板布局、配色、间距、交互、固定文案的变化；D1/D2 所定的片段编码与含反引号作者值的等宽分类属于本期增量。
- 原 M6/V-11 的真机/稳定浏览器/VoiceOver、V-12 正式性能，原 M7/V-15 的公开安装/Pages/回滚（D16 留 P4），差异报告的 U-01–U-03。
- P3 多语言、P4 搜索/AI 索引、GPU。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·缺口]** `dev.js` 的 `prose()`（L208–218）按单个反引号切分并交替渲染为文本和 `code`，`fill()` 对 message（L259）和 suggestion（L260）都用它。默认值 `"a`b".to_string()` 时面板显示 `"ab".to_string()`，排版颠倒；终端与 JSON 正确（[复审证据](../validation/docs-ssg-p2/prd-dev-gaps-r2/tick-panel.json)）。
- **[已核实·缺口]** 诊断模板直接写 `` `{x}` ``，不处理内容中的反引号。原调查按开反引号后紧接插值统计（含测试中的同类字符串，仅为改动面下界，不是 V-2 的覆盖清单）：
  - `rustify-docs`：`content.rs` 37、`markdown.rs` 17、`embed.rs` 14、`config.rs` 13、`include.rs` 9、`generation.rs` 7、`crates/rustify-docs/src/site/registry.rs` 6、`html_check.rs` 4，其余 6 个文件各 1。
  - `rustify-docs-cli`：`rust_site.rs` 9、`rust_source.rs` 6、`theme.rs` 5、`rendering.rs` 5、`main.rs` 4、`links.rs` 2、`preview.rs`/`new.rs` 各 1。
  - `rustify-docs-syntax/src/lib.rs` 12。
  - 插值内容大多来自作者：YAML 键/值与组件名、路径、路由、配置值、默认表达式。
- **[已核实·缺口]** `content.rs` 的 broken-anchor 消息直接插入普通文本 `{file}`，同一消息的 `#{anchor}` 带固定前缀；`generation.rs` 的 `#{id}`/`#{anchor}` 与 `markdown.rs` 的 region/endregion 也有前缀。仅改直接插值片段会漏掉这两类；按 D2 分别处理，并在 V-2 单列审计。`config.rs` 的空 `site.lang` 可达，按 D1 单列空代码值。
- **[已核实·缺口]** `site!` 的冷态解析错误经 `rust_site.rs` L343–356 直接成为 `rust-registry` 诊断，所以 syntax 的消息也进入面板；`rustify-docs` 只以 dev-dependency 引用 syntax（`crates/rustify-docs/Cargo.toml`），三者没有共同的运行期依赖。
- **[已核实·足够]** `report.rs` 文本模式原样输出 `= help: {suggestion}`（L57），JSON 原样输出字段（L71、`schema_version` L84）；dev SSE 与终端用同一 `Diagnostic`。不需要新字段或协议。
- **[已核实·足够]** `dev.js` 只由 dev 服务提供（`generation.rs` L33、`dev.rs` L881）。生产产物不含 `dev.js` 或其引用：复审时 C′ 构建的 fixture 产物中计数为 0。因此 components/official 浏览器项目不受影响。
- **[已核实·缺口]** 测试辅助 `expectRepairs`（`tests/docs-ssg/dev-components.spec.ts` L824–838）按“去掉全部反引号”“匹配 `` `([^`]+)` ``”断言面板，编码的正是旧读法，需随 D5 更新。新边界用例须另写独立的精确文字/code 列表，不能只以该辅助解析器为期望来源。
- **[已核实·足够]** 首个增量计划的配对与安装流程可复用：[pair-audit.py](DOCS-SSG-P2-gaps.records/m2/pair-audit.py) 以 `--r/--previous-r` 审计模板锁六框架包（含 `rustify-docs-syntax`）；[author-acceptance.py](DOCS-SSG-P2-gaps.records/m2/f01/author-acceptance.py) 是已安装 CLI 的 new 工程验收基础。

### 1.2 文件与边界

| 路径 | 精确增量 |
| --- | --- |
| ★ `crates/rustify-docs/src/diagnostic_text.rs`；`crates/rustify-docs/src/lib.rs` | 新增仅依赖 std 的 `code_span`/`literal` 及单测；`lib.rs` 无条件 `pub mod diagnostic_text`，不放入仅 pipeline 可见的 `diagnostic` |
| `crates/rustify-docs-syntax/src/lib.rs` | 同一 `code_span`/`literal` 的独立实现，供本 crate 的错误消息；单测 |
| `crates/rustify-docs/src/*.rs`、`crates/rustify-docs/src/site/registry.rs`、`crates/rustify-docs-cli/src/*.rs`、`crates/rustify-docs-syntax/src/lib.rs` | 自有诊断的全部动态代码片段和普通文本作者值按 D2 改写；非诊断文字（CLI 用法、生成工程文件、日志）不改，在 V-2 审计中列明 |
| `crates/rustify-docs/assets/dev.js` | `prose()` 按 D5 读取；`breakable()`、换行与样式类不变 |
| `crates/rustify-docs-cli/tests/`（新增或扩展 `check.rs`/`components.rs`/`dev.rs`） | 两份 helper 一致性测试；含反引号、空代码值和普通文本作者值的真实命令断言 |
| `tests/docs-ssg/dev-components.spec.ts` | `expectRepairs` 按新读法核对常规回归；新增 `gaps-2 code spans` 独立精确期望用例 |
| `docs/docs-ssg.md` | 诊断段补 Rustify 逐字片段规则、空值、普通作者值与外部原文边界；明确不能用通用 Markdown 读法替代 |
| `crates/rustify-docs-cli/Cargo.toml`、`crates/rustify-docs-cli/assets/framework-revision.txt`、`crates/rustify-docs-cli/assets/components-Cargo.lock`、根 `Cargo.lock`、`docs/docs-ssg.md` 候选行 | M2 的 C″ 五文件配对 |
| ★ `docs/plan/DOCS-SSG-P2-gaps-2.records/M1.md`、★ `docs/plan/DOCS-SSG-P2-gaps-2.records/M2.md` | 实施发生后才创建 |

无需修改：`dev.css`、`report.rs`、SSE 协议、`Diagnostic`/`Issue` 结构、模板源码、CSS 与主题、GPU。站点与岛只改 `crates/rustify-docs/src/site/registry.rs` 的自有消息写法及其轻量 helper 调用，不改渲染、状态或水合行为；独立特性检查见 §9.3。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| `--format json` 消费者 | 已核实足够 | message/suggestion 为字符串，`schema_version` 1 | 字段不变；按 D1/D2 编码动态代码片段和普通作者值，空代码值也改变字符串写法；消费者遵循 §5.1，不承诺 Markdown 等价 | 不改退出码、rule、位置、顺序 |
| text 终端 | 已核实足够 | 原样打印 | 打印方式不变；字符串按 D1/D2 编码 | 同上 |
| dev SSE/面板 | 已核实缺口 | `prose()` 单反引号切分 | §5.3 读法 | 不改焦点、隐藏/重开、清除、last-good |
| 外部原文（serde、YAML、rustc、IO） | 已核实足够（不改） | 作为 message 原样传递 | 不改写；面板同一读法 | 未配对反引号原样显示 |
| R/C 配对与安装 | 已核实足够 | 首个增量计划的 M2 记录 | §5.5 | 身份/锁不符时拒绝验收 |

## 5. 核心机制

### 5.1 写法与读法

本节定义 **Rustify 逐字片段**，用于 message/suggestion 字符串，不是通用 Markdown。以 ASCII 反引号与空格为语法字符；不做反斜杠转义或空白归一化。

`code_span(value)` 的两份实现输出必须相同：

- 非空且不含反引号：单反引号 + 原内容 + 单反引号，不添加填充；保持现有字节。
- 空串：两个反引号 + 两个 ASCII 空格 + 两个反引号，即 ```"``  ``"```。内侧两个空格都是填充，保证开/闭围栏不合并；不再写成相邻的两个反引号。
- 含反引号：N = 内容最长连续反引号串长度 + 1；输出 N 个反引号 + 一个 ASCII 空格 + 原内容 + 一个 ASCII 空格 + N 个反引号。原内容可为空格、LF/CRLF、反斜杠或首尾反引号，均不归一化。

`literal(value)` 供原为普通文本的作者值使用：含反引号时返回 `code_span(value)`，否则逐字返回原值（含空串）；不能把已编码的片段再次传入。用户 2026-10-15 选定该分类，含反引号时整值使用既有等宽样式，未选为普通文本另增转义规则。

读法（面板与常规测试辅助遵循同一合同）：

- 从左到右扫描最大连续反引号串；长度为 N 的串只与其后第一个长度恰为 N 的最大串配对，内部不同长度的串都是内容。找不到关闭串时，该串原样作为普通文本，继续扫描。
- 配对内为一个代码片段。N = 1 时内文全部保留；N ≥ 2、内文至少有两个字符且首尾都是 ASCII 空格时，各去掉一个空格（包括全为空格的情况）；否则不裁剪。除这两个填充字符外逐字保留，LF/CRLF 不转成空格。空编码读出空字符串和空代码节点。
- 生产方组合完整消息时，片段之间保留模板已有的非反引号分隔文字；不得直接拼接两段围栏使反引号串合并。确有相邻代码值时，将其完整显示内容合并为一次 `code_span` 调用，不新增可见字符。

性质：对任意片段内容，读回 `code_span` 得到原内容；`literal` 同样逐字可读回。空片段与后续片段组合不吞并。常规兼容仅限非空且无反引号的代码值、无反引号的普通作者值；空代码值的 text/JSON 写法变化，面板仍显示空内容。

与 [CommonMark 0.31.2 §6.1](https://spec.commonmark.org/0.31.2/#code-spans) 的区别：该标准会将代码内的行结束符转为空格，并按内容条件裁剪首尾空格；这里保留 LF/CRLF，单围栏从不裁剪，长围栏只去上述填充（全为空格也适用）。JSON 只保证标准 JSON 可解析，消息内容的消费者须按本节读取，不能用通用 Markdown 得到相同文字。

### 5.2 生产方改写与审计

1. 在新增 `crates/rustify-docs/src/diagnostic_text.rs` 定义 `pub fn code_span(value: &str) -> String` 与 `pub fn literal(value: &str) -> String`，`lib.rs` 无条件公开该模块。只用 std，不引用 `Issue`、pipeline 或 SSR 类型；pipeline 内部与 `crates/rustify-docs/src/site/registry.rs` 经 `crate::diagnostic_text::code_span` 调用，CLI 经 `rustify_docs::diagnostic_text` 调用同两函数。`rustify-docs-syntax/src/lib.rs` 公开同签名的独立实现，供本 crate 的错误消息，并由 CLI 测试比较两函数在同一输入表上的输出。不为复用打开 pipeline，也不改变 Cargo 特性边。
2. 把诊断 message/suggestion 模板中所有含插值的代码片段改为调用写法函数，包括组合清单中的每个片段。先格式化完整片段再传入 helper：固定前后缀、多个插值及格式说明符都属于片段内容；例如 `format!("#{anchor}")`、`format!("rustify-region:{region}")` 及 endregion，同纯插值一起覆盖。不含插值的固定片段（如 `` `site!` ``）保持原样。
3. 自有模板中原为普通文本的作者值（路径、路由、键/值等）统一调用 `literal`；不是第三方原文，不适用 D4 豁免。先取得完整值再编码，例如 broken-anchor 的 `file`；不把整条消息包成代码，也不对已经编码的片段再次编码。固定描述和数字不需改写；外部原文继续原样传递。只改 D1/D2 的写法与分类，不改其余文字、rule、位置、顺序、去重或严重级别。D1 的常规范围内，既有精确期望不改即通过；空代码值单列新编码和旧显示等价断言。
4. 脚本化替换须断言每处旧文本确实被找到；`cargo fmt` 之后重新核对。
5. V-2 审计：在三个 crate 的源码中搜索全部反引号（命令见 V-2），逐处区分动态诊断片段、固定诊断片段、非诊断文字（CLI 用法、生成工程内容、日志等）、测试期望与 helper 内部实现；动态片段须已改写，不得因固定前缀、后缀、多个插值或多行格式字符串漏掉。另从全部自有 message/suggestion 构造点沿插值、拼接和列表组合审计普通文本作者值；记录每处使用 `literal` 或属于固定/数字/外部原文的依据。反引号搜索不能发现没有围栏的作者值，不能独自证明覆盖。记录分类及路径/符号，不以旧正则零命中作为全覆盖依据。

### 5.3 面板读取

`prose()` 改为 §5.1 的读法，继续产出原有两类 DOM 节点：文字节点与 `code.rd-dev-code`（内部仍经 `breakable()` 在 `/` 后插入 `wbr`）。空代码片段产出空 `code` 节点，不吞并后续片段；`literal` 输出的完整围栏同样用该 code 样式，不增加转义语法。`fill()`、样式类、焦点、隐藏/重开、Esc、修复后清除和 live 区域都不变。外部原文中成对的反引号仍显示为代码，未配对的原样显示。

### 5.4 作者参考

`docs/docs-ssg.md` 的嵌入诊断段说明并列出 §5.1 的写/读规则：这是 Rustify 逐字片段，保留空格和 LF/CRLF，不承诺 CommonMark 等价；JSON 结构可用标准 JSON 解析器读取，但消息内容须按该规则解码。给出空代码值、更长围栏及普通作者值含反引号时整段等宽的例子，明确外部原文未改写、其成对反引号仍可能歧义；不对第三方原文扩张逐字保证。原有“check、build 与 dev（终端和页面问题面板）报告同一位置和修复”的表述保持，适用范围与 D4 一并说明。

### 5.5 库 R″ 与 CLI C″ 本地固定配对

1. M1 通过 V-1/V-2/V-3 与 §9.3 的 M1 门后提交库 R″（产品源码、测试、作者参考，以及 M1 记录和快照）。
2. M2 首先制作 C″：
   - 改动只有五个配对文件：CLI 版本 0.1.17 → 0.1.18；`framework-revision.txt` 指向完整 R″；`components-Cargo.lock` 中六框架包的 Git source 指向 R″；根 `Cargo.lock` 只改 CLI 版本；`docs/docs-ssg.md` 的两处开发候选版本与配对 R。
   - README 中已公开验证的 P1 安装命令不改。
3. 制作 C″ 前保存 `git status --porcelain=v1 --untracked-files=all`、已有 staged/unstaged 差异与相关文件摘要；只暂存本次五文件。提交前用准备版 CLI 运行 [pair-audit.py](DOCS-SSG-P2-gaps.records/m2/pair-audit.py) `--r <R″> --previous-r 745ba6b720aa6ed042dcf5d4f763641fc2d05165`，并审计本次拟提交产品差异：
   - `git diff --cached --name-only` 恰好是上述五文件，逐一核对 `git diff --cached`，只暂存本次相关 hunks；当前基线无 staged 内容。若提交时发现与保存基线冲突的他人暂存内容，先停止该提交并核对归属，不取消暂存或清理用户差异以凑此门；
   - 相对 R″ 核对拟提交树（含 staged 五文件）的产品差异：库、运行资源、CLI 功能 src、模板源与 `site/` 均为 0；R″ 与 C″ 之间协议要求的纯进度/证据提交单列，不夹带产品变化。工作树原有台账、报告、fixture 日志与缓存不计入本次五文件差异，须按制作前清单核对保留；
   - 第三方锁记录不变；
   - 两目标 `--locked --offline` metadata/check 通过且无 Makepad；
   - `cargo update --workspace --offline` 不改锁。
4. 提交 C″ 后按 §9.3 真实安装到新的隔离根。记录完整 C″/R″、入口、SHA-256 与 `--build-info`（source=C″、panic=unwind）。之后所有 dev/浏览器命令显式继承该入口。
5. 本地 Git 传输不等于公开可获取。回滚时恢复 R′/C′（`745ba6b7`/`79a77825`）及其锁/产物。M2 若发现产品缺陷，回到 M1 修复并冻结 R‴，递增 patch 重做 C 与固定出口；旧版本证据保留原版本。

## 6. 前端与交互

- M1 开头用 impeccable 读取 PRODUCT/DESIGN 与 [dev 面板 brief](../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md)，确认本增量不改层级、字号、配色、间距和交互，只按 D1/D2 改片段边界与作者值分类。确认含反引号的值同样以系统等宽显示、反引号可见、在既有 `overflow-wrap` 下换行。结论写入 M1 记录；brief 只在出现新规则时追加。
- 状态：错误、警告、长候选、空错与修复后清除沿用现有面板；不新增状态。
- 浏览器验收：docs-dev project 的真实 dev 服务，1280/375 × 浅/深；记录 console/CSP/失败请求。M1 用工作树 CLI，M2 用已安装 C″。原 M6 环境项不由本计划冒作通过。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N6 | 既有拒绝/last-good | 错误仍非零、位置不变、旧产物与 last-good 保持 | 任何变化都不收口 | 只改文字写法 | V-1/V-3/V-4 |
| WORK-F15 | `schema_version` 1 | JSON 结构不变，可用标准 JSON 解析器读取；内容遵循 §5.1 | 结构变化即拒绝 | D1/D2 | V-1 |

面板继续只用 `textContent` 与 DOM 节点渲染，不引入 `innerHTML`，因此作者内容中的标记不会被解释。不新增网络请求、存储或依赖；CSP 不变。

## 8. 失败模式与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 改写误改了 D1 常规范围内的诊断文字 | 全部诊断 | 无持久数据 | 文案变化，机器消费者比对失败 | 既有精确字符串测试 | 回退该处改写 | V-1/V-5 |
| 漏改某处模板 | 该类诊断 | 无 | 含反引号时面板仍可能失真 | V-2 审计 | 补改并加用例 | V-2 |
| 两份写法不一致 | `site!` 解析错误 | 无 | 同一内容两种围栏 | 一致性测试 | 修正后重测 | V-1 |
| 面板读法与写法不一致 | dev 面板 | 无 | 片段边界错乱 | 浏览器精确文本断言 | 修 `prose()` | V-3 |
| 候选版本/锁不成对 | 本地新版本工程 | 新代拒绝，旧产物保留 | 配对诊断 | build-info、锁审计 | 恢复 R′/C′ | V-4 |

没有公开发布动作；不改输入闸、失败状态机或 SSE 协议。

## 9. 验证

### 9.1 行为与接口验证

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F15/N6；§5.1 两份 `code_span`/`literal` 对同一手写输入/编码/显示表一致并可读回：空串、空格串、首尾空格、LF/CRLF、反斜杠、无反引号、单个/连续/首尾/全为反引号；空片段与后续片段、普通作者值与正式代码片段组合不串段。代表性诊断的精确文字：<br>- 含反引号默认值的 unknown-property、invalid_props 与缺值字段清单；<br>- 含反引号的 YAML 未知键；name 含反引号仍为 `embed-payload`，合法 `MissingCounter` 才为 `unregistered-component`，不放宽接受范围；<br>- 无 Rust 内容站：临时 ``content/a`b`c.md`` 与链接 `a%60b%60c.md#a%60b`，broken-anchor 的普通 file 与完整 `#anchor` 都保真；<br>- 空 `site.lang` 仍为 `config-lang`，消息代码节点内容为空，后续建议的 en/zh-CN 边界正确；<br>- region/endregion 固定前缀、配置值、`site!` 重复路由。<br>手写原始编码、显示文字与 code 列表，不能用被测解析器生成全部期望；D1 常规范围既有期望不改即通过，空代码值单列新编码/旧显示兼容 | `mbx test --locked -p rustify-docs`；`mbx test --locked -p rustify-docs-cli`（含 `check`/`components`/`dev` 与一致性测试）；`mbx test --locked -p rustify-docs-syntax` | M1 先红后绿；M2 在 C″ 上重跑 CLI 受影响测试 |
| V-2 | D2 覆盖：全部动态代码片段（含固定前后缀、多插值及格式说明符）经完整内容编码；全部自有普通文本作者值经 literal。原统计不是覆盖清单，剩余命中与未包围作者值均按 §5.2 逐处归类 | `rg -n '\x60' crates/rustify-docs/src crates/rustify-docs-cli/src crates/rustify-docs-syntax/src`；另从自有诊断构造点沿插值/拼接审计普通作者值。路径/符号、分类、处理或豁免依据写入 M1 记录 | M1 提交 R″ 前 |
| V-3 | WORK-F13/N6；真实 dev 面板的 `.rd-dev-suggestion`/`.rd-dev-message` 与 code 节点以原始 `textContent` 精确比较，不使用归一化空白的断言，也不以同一个解析器生成所有期望：<br>- 含反引号默认值（未知属性、类型错误各一）、YAML 未知键；<br>- 含单个反引号的值（serde 原文原样，保持 D4 限定）；<br>- V-1 的真实 broken-anchor：显示文字为 ``"content/a`b`c.md has no heading #a`b"``，code 列表精确为 ``["content/a`b`c.md", "#a`b"]``，并与 SSE 的完整片段编码核对；<br>- V-1 的空语言：message 为 `" is not a language tag"`、code 列表 `[""]`，建议的 code 列表 `["en", "zh-CN"]`。<br>既有两个 `gaps embedded diagnostics` 用例按新读法通过；1280/375 × 浅/深无横向溢出；Hide/Enter/Esc 焦点不变；修复后清除；console/CSP/失败请求为空 | `npx playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'gaps embedded diagnostics\|gaps-2 code spans' --workers=1` | M1 用工作树 CLI（`RUSTIFY_DOCS_BIN` 未设）；M2 绑定已安装 C″，重放本行全部输入 |
| V-4 | D-05/C-2：按 §5.5 审计本次五文件拟提交树并保留制作前 dirty；锁与两目标离线检查。真实安装后 new 工程三依赖固定 R″、锁不变、两图零 Makepad。重放 V-3 的反引号默认值、普通 file 与前缀 anchor、空语言：check 文本/JSON、build、dev SSE 与终端同文同位；面板对独立精确文字/code 表通过。错误退出 1、dist 不变、last-good 原字节、修复后 errors 0 | §5.5 审计；§9.3 安装配方；在首个增量计划 new 工程验收脚本基础上增加上述阶段的脚本（存于 M2 记录目录），同一 new 工程用真实浏览器复验 | M2 |
| V-5 | 回归：dev 面板其余用例（含多行 Rust 失败、链接诊断）在新读法下通过；docs 与 CLI 全量测试、clippy；docs-cli 构建 | `npx playwright test -c playwright.docs.config.ts --project=docs-dev --workers=1`；§9.3 的 M1 门 | M1 全量；M2 以已安装 C″ 重跑 docs-dev project |

components-subpath、official-subpath 等产物项目不重跑：产物不含 `dev.js`，生产页面结构、CSS、模板和交互不改；registry 增量只涉及自有错误字符串。当前工作树的 registry 由 §9.3 独立两图检查覆盖，M2 再以固定 R″ 的 new 工程覆盖真实命令出口。实施时若超出 §1.2，再按实际影响补跑。

### 9.2 证据要求

每个浏览器用例保存实际 `textContent`、`code` 文本、视口/主题与截图；Playwright 输出用 `--output` 隔离目录，不清理既有 `test-results/`。M2 每次 project 运行前后记录入口、SHA-256、`--build-info`。

### 9.3 集成与检查边界

M1 退出、提交 R″ 前的门：

- `cargo fmt --all -- --check`；
- `mbx test --locked -p rustify-docs`、`mbx test --locked -p rustify-docs-cli`、`mbx test --locked -p rustify-docs-syntax`；
- 当前工作树独立 `site-ssr`：`mbx test --locked -p rustify-docs --no-default-features --features site-ssr` 与 `mbx clippy --locked -p rustify-docs --no-default-features --features site-ssr --all-targets -- -D warnings`；
- 当前工作树独立 hydrate/wasm：`mbx check --locked -p rustify-docs --no-default-features --features hydrate --target wasm32-unknown-unknown`，保存对应特性图并确认没有 pipeline/site-ssr；不得把两图合并执行或强开 pipeline。以上沿用 `.github/workflows/verify.yml` 的隔离入口，必须先于 R″ 冻结；
- `mbx clippy --locked -p rustify-docs --all-targets -- -D warnings`、`mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings`；
- `mbx build --locked -p rustify-docs-cli --profile docs-cli`；
- V-1/V-2/V-3/V-5 通过。

两包不放进 CSR workspace union。`dev-components.spec.ts` 会经 `scripts/docs-pages-prepare.py` 按 CLI 所报提交的配对 R 投影 fixture，M1 仍可能使用 R′；其浏览器绿色不代替当前工作树的独立 site-ssr/hydrate 检查。M2 继续用固定 R″ 的真实 new 工程验证两目标。

M1/M2 的 CLI 与浏览器命令在同一验证 shell 中运行，先设置以下环境。工具缺失时报告缺口，不安装；Git 重映射只改本进程读取，不改产品 URL/锁或全局设置。

```sh
set -eu
DOCS_GAPS2_REPO=$(git rev-parse --show-toplevel)
DOCS_GAPS2_PYTHON=$(command -v python3.12)
DOCS_GAPS2_TOOLS=$(mktemp -d "${TMPDIR:-/tmp}/rustify-docs-gaps2-tools.XXXXXX")
ln -s "$DOCS_GAPS2_PYTHON" "$DOCS_GAPS2_TOOLS/python3"
export PATH="$DOCS_GAPS2_TOOLS:$PATH"
python3 --version
export GIT_CONFIG_COUNT=1
export GIT_CONFIG_KEY_0="url.file://$DOCS_GAPS2_REPO/.insteadOf"
export GIT_CONFIG_VALUE_0="https://github.com/rockie/rustify-ui"
export CARGO_NET_GIT_FETCH_WITH_CLI=true
export RUSTIFY_DOCS_CARGO=mbx
```

M1 在该 shell 中 `unset RUSTIFY_DOCS_BIN`。M2 在 C″ 提交并回填后：

```sh
: "${DOCS_GAPS2_C:?从M2记录填入已提交的完整C″ SHA}"
DOCS_GAPS2_INSTALL=$(mktemp -d "${TMPDIR:-/tmp}/rustify-docs-gaps2-installed.XXXXXX")
mbx install --git https://github.com/rockie/rustify-ui --rev "$DOCS_GAPS2_C" --locked --profile docs-cli --root "$DOCS_GAPS2_INSTALL" rustify-docs-cli
export RUSTIFY_DOCS_BIN="$DOCS_GAPS2_INSTALL/bin/rustify-docs"
shasum -a 256 "$RUSTIFY_DOCS_BIN"
"$RUSTIFY_DOCS_BIN" --build-info
"$RUSTIFY_DOCS_BIN" new "$DOCS_GAPS2_INSTALL/author-site" --template components
```

局部绿色不是整 P2 完成：原 M6/M7 仍未关闭，WORK-F15 整条终验在 P3。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 无歧义代码片段与面板读法 | 当前 R′/C′；G-06 复审反例 | 开头完成 §6 的 impeccable 确认；先补 V-1/V-3 红测试，再按 §5.1–§5.4 实施。模板改写与 `dev.js`/spec 可分两人，`diagnostic_text.rs` 无条件导出与 §5.1 合同先定；Cargo 与浏览器串行 | V-1、V-2、V-3（工作树 CLI）、V-5 与 §9.3 M1 门通过后提交库 R″；记录实际改写清单与审计归类；回写「实施进度」 |
| M2 | 本地固定配对与真实出口 | M1 已完成，R″ 已提交并回填 | 首先按 §5.5 制作 C″ 并完成提交前审计 → 提交 C″ → §9.3 真实安装 → V-4 new 工程验收 → 绑定已安装 C″ 重跑 V-3 与 V-5 的 docs-dev project；主实施者独占 lock/target/浏览器，不推送/部署 | V-4 通过；V-3、V-5 在已安装 C″ 上通过；入口 SHA/source、C″/R″/锁一致；产品缺陷按 §5.5 回路处理后重验；回写「实施进度」 |

提交点：M1 收口后提交库 R″，回填其短 SHA 并单独提交该回填。M2 审计通过后单独提交五文件 C″，立即在 M2 记录登记 C″。M2 固定出口完成后提交证据与最终进度，再按顶部协议另提交 SHA 回填。R″ 与 C″ 之间的纯进度/证据提交单列，C″ 产品提交仍恰好五文件。只暂存本计划相关改动，用户既有台账、报告、fixture 日志与缓存按 §5.5 清单保留，不纳入、不清理。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 全部动态片段及普通作者值改写误伤文案 | 机器消费者或测试回归 | 实施者：按 D1/D2 分类，常规范围既有精确期望不改；空代码值单列；V-2 全量审计并核对 diff | M1 提交 R″ 前 | 否；有回归门 |
| 第三方原文中的成对反引号仍按代码显示 | 例如 serde 的 “unknown field `a`b`” 本身有歧义 | 按 D4 接受，作者参考不承诺改写外部原文 | — | 否；已排除 |
| 新 R″/C″ 配对 | 旧候选不能证明新行为 | 按 §5.5 实际提交/安装，不预填通过 | M2 出口 | 否；流程已在首个增量计划验证 |
| 原 M6/M7 与 U-01–U-03 | 整 P2 不能宣称完成 | 原计划按其解除条件继续 | 原期出口 | 否 |

- 最终状态：**Ready**。
- 定级理由：用户已决定在 P2 补齐（D-05）。空值和逐字读法、完整片段与普通作者值合同、跨特性函数位置均已确定；普通作者值分类已有用户 2026-10-15 决定。新源码两图在冻结 R″ 前检查，C″ 审计限定拟提交产品树并保留既有 dirty。不改 JSON 结构、SSE 协议、面板布局/交互或数据；配对与安装沿用已验证流程。剩余的测试、审计与浏览器结果是实施退出门，不预填。

## 12. 已知坑与历史教训

- 只验证“建议非空”或固定样例会漏掉边界内容：首个增量计划的用例都不含反引号，因而没有覆盖到 G-06。V-1/V-3 必须含反引号内容并断言精确文字。
- `CLAUDE.md`「Conventions」：`cargo fmt` 会重排行，脚本化替换必须断言旧文本被找到。
- `CLAUDE.md`「Browser tests」：浏览器 project 一次只跑一个，不并发；mbx 可能回收 `target/`，运行前确认产物与 worktree 登记（`git worktree list --porcelain`），只清理本次无源文件的过期测试登记。
- mbx 下工程首次以 Development profile 编译时可能报一次 `rust-generation-changed` 并丢弃该代，第二次正常；测试先在良好状态预热（首个增量计划 §12 已登记）。
- R/C、模板锁和安装身份成对；本地 Git remap 不代表公开可获取，旧候选证据保留原版本。
