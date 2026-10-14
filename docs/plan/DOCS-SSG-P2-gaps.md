# DOCS-SSG-P2-gaps · Rustify UI · 嵌入诊断与减少动效补齐（DOM · 静态文档站 · 本地固定版本）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-15 +08:00 · `d2f20b03cf58c1d1ced3785397c501d412cd4167` · dirty；事实来自当前工作树，未提交项为既有状态/评审、缓存和本计划/差异报告，产品源码未改。
>
> 需求来源：[DOCS-SSG PRD](../PRD-DOCS-SSG.md) · **P2（第 2 期）**；补齐[原计划](DOCS-SSG-P2.md)的已交付部分。增量来源：[差异报告](DOCS-SSG-P2.prd-dev-gaps.md)的诊断、减少动效及已批准Firefox口径同步。
>
> 本期交付：**组件/属性错误能给出可用名称和修复建议；系统减少动效时文档岛控件没有过渡；PRD明确已批准的P2 Firefox延期边界；以本地R/C固定配对交付。**
> 本期独特职责：补齐已有拒绝路径和已有动效规则的实际行为；现有演示、阅读布局、主题和失败保全继续使用。
> **顶层排除：原M6设备/正式性能缺证、原M7公开安装/Pages/回滚、P3/P4功能、GPU能力、配色重设计和已完成里程碑重做。**

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

- 最近更新：2026-10-15 02:52 +08:00
- 当前进度：2/2 个里程碑完成
- 当前状态：全部完成：R=`cc5fded3`、C=`7a42230b`（0.1.16）本地固定配对，已安装 C 的 new 工程与浏览器固定出口通过
- 最近完成：M2 · C 审计/提交/真实安装，new 工程 CLI 验收与按 C 投影的 V-2/V-3 复验通过
- 下一步：代码评审：用 review-code 以 `docs/plan/DOCS-SSG-P2-gaps.md` 为参数评审本计划提交（R、C 与证据提交）；原 M6/M7 缺证与整 P2 终验仍由原计划承接
- 当前阻塞：无
- 代码基线：C `7a42230b` 之后的证据提交（见完成记录）；用户既有台账、评审/差异报告与夹具缓存改动保持未提交原位

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-15 02:24 +08:00 | 诊断建议、CRLF/容器位置与 dev 去重；reduce 规则；PRD/作者参考/DESIGN 同步；V-1–V-4 与 §9.2 门通过 | [M1 记录](DOCS-SSG-P2-gaps.records/M1.md) | cc5fded3 |
| M2 | 已完成 | 2026-10-15 02:52 +08:00 | C=0.1.16 五文件配对 R；本地 Git 真实安装、new 工程 CLI 验收、按 C 投影的 docs-dev/components/official 复验通过 | [M2 记录](DOCS-SSG-P2-gaps.records/M2.md) | C 7a42230b、待提交 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

本表保留原PRD工作号；只规划已确定的P2增量。WORK-F4终验P2，WORK-F15终验P3、WORK-N5终验P4，本增量完成不关闭其他期次或原M6缺证。

下表标「上游编号」的行只登记原计划出处，不是本计划的里程碑或检查；本计划只有 §10 的 M1/M2 和 §9 的 V-1–V-5。

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| WORK-F4 | 功能 | PRD §3 AC-3；报告诊断差异 | 嵌入错误的可用组件/字段、位置与修复路径 | §5.1；V-1/V-3 | 已确认；P2增量 |
| WORK-F15 | 功能 | PRD行为、AC-2/AC-3；报告诊断差异 | text/JSON/dev的可执行建议；独立内容与Rust错误仍同次收集 | §5.1；V-1/V-3 | 已确认；整条终验P3 |
| WORK-N6 | 可靠性 | PRD AC-2/AC-3；报告诊断差异 | 拒绝语义、原位置、旧产物及last-good保全 | §5.1/§8；V-1/V-3 | 已确认；本期回归 |
| WORK-N5 | 可用性 | PRD当期复核 AC-5及受影响AC-1/AC-6；报告动效差异 | reduce实际无过渡；稳定帧、新类型/状态的对比度与AA复核 | §5.2/§6；V-2/V-3 | 已确认；整条终验P4 |
| WORK-N3 | 兼容 | PRD AC-4/Q3；原计划C-6/§11用户决定 | 只同步Firefox稳定版与引擎P2未验证/P4恢复，不扩大设备豁免 | §5.3；V-4 | 已确认；验证留原分期 |
| G-01 | 差异 | [来源报告](DOCS-SSG-P2.prd-dev-gaps.md#2-三类差异) | 缺少候选与具体建议 | §5.1；V-1/V-3 | A；需要补齐 |
| G-02 | 差异 | 同报告合理变更；2026-10-13用户决定 | Firefox延期的PRD同步 | §5.3；V-4 | B；决定已确认 |
| G-05 | 差异 | 同报告动效反例/CSSOM；当前组件标记 | reduce规则被有效样式覆盖且漏掉控件子元素/Spinner动画 | §5.2；V-2/V-3 | A；需要补齐 |
| C-1 | 项目约束 | AGENTS.md前端设计/浏览器验收；既有Read设计 | DOM路径，保留阅读世界，真实浏览器确认本次错误反馈/动效 | §6；V-3 | 已确认 |
| C-2 | 版本/工具链 | mise.toml；原计划§2.1；固定C5/R5 | mbx/锁定依赖、分包；库R→CLIC两个真实提交固定配对，C只改身份/模板锁/安装文案 | §5.4；V-5 | 已确认 |
| C-3 | 工作范围 | PRD D15/D16；报告待核实/原未完范围 | 本人C4结论保持；原M6/M7缺证与P3/P4不移入本计划，不要求现在发布 | §0.5/§11；V-4/V-5 | 已确认 |
| M6 | 上游编号 | 原计划 §10 | 原期完整本地/用户验收与缺证，沿原位追踪 | §0.5/§11；V-4 | 原计划承接，本计划排除 |
| M7 | 上游编号 | 原计划 §10 | 原期公开安装/发布与回滚，按D16后移 | §0.5/§11；V-4 | 原计划承接，本计划排除 |
| V-11 | 上游编号 | 原计划 §9.1 | 原期真实浏览器/设备完整矩阵 | §0.5；V-4 | 原计划承接，本计划排除 |
| V-12 | 上游编号 | 原计划 §9.1 | 原期完整性能与样本合同 | §0.5；V-4 | 原计划承接，本计划排除 |
| V-14 | 上游编号 | 原计划 §9.1 | 实际共享改动的原回归约束 | §9.2；V-5 | 按真实影响引用，不重做未变部分 |
| V-15 | 上游编号 | 原计划 §9.1 | 原期公开来源/平台预览/Pages与回滚 | §0.5；V-4 | 原计划承接，本计划排除 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 诊断事实源 | 复用site!顶层清单、helper类型错误和内置registry；内置复合类型给声明类型及作者参考链接 | 标量给YAML形状，复合类型不推导嵌套schema；不维护第二份名字/字段/类型字典或手写嵌套示例，不依赖旧缓存 | G-01；§5.1；manifest.rs、builtins.rs、site/registry.rs |
| D2 | 动效修复范围 | Docs reduce媒体查询对scope及全部后代用零duration/零delay的!important；SDK Spinner单独取消动画 | 覆盖无data-name子元素及scope内作者组件过渡；正常偏好、SDK其他应用、颜色/结构/GPU保持；作者参考说明范围 | G-05；§5.2；docs.css/controls.css、DESIGN.docs-ssg.md的既有reduce合同 |
| D3 | 变更口径 | 沿用已有Firefox、D15/D16决定 | 文档同步无需再裁定，不把后移当通过 | G-02、C-3；原计划§11、PRD D15/D16 |
| D4 | 固定版本交付 | 先提交功能库R，再制作CLIC并本地真实安装 | 生成工程rev与lock与CLI配对；不拿dirty开发版或旧安装充新候选 | C-2；原计划§2.1、M6已有配对流程 |

### 0.4 职责与事实所有权

- `site!`及内置registry拥有可用名称、顶层字段/类型串与默认表达式；不包含嵌套schema或独立必填标志。宿主serde拥有实际类型失败原因。
- CLI/Markdown管线拥有源码位置、诊断建议、text/JSON/dev一致性；既有report/dev只展示这些事实。
- Docs外部CSS拥有站点reduce规则；沿用基础脚本的loading/ready/failed、输入闸及静态恢复，不改变它们的状态机。
- 记录/计划拥有历史判定与原缺证；源代码不放需求或评审编号。

G-01/G-05的增量限定在CLI进程内的诊断/Markdown/样式管线，不改helper的注册协议、site-ssr或hydrate行为。M1的工作树CLI以HEAD为source，`framework-revision.txt`仍为R5，组件测试/浏览器消费者实际配对Git R5的helper/wasm；M1记录必须写明CLI来源、dirty改动和实际R。若确需改helper侧行为，先回写影响范围，改用临时真实R/C配对或独立空R的path开发模式验证，不能拿旧R的绿色证明新helper；最终仍走§5.4。

### 0.5 明确不在本期

- 原M6/V-11的实体手机、最新稳定版、VoiceOver与原生缩放，以及V-12的正式A/B性能/完整样本合同，继续由原计划承接。局部Chromium不替代它们。
- 原M7/V-15的公开GitHub固定来源、围栏平台预览、Pages全部页面/资源/回滚，按D16留P4。
- P3多语言/回退、P4搜索/AI索引，以及改变本人验收口径、招募研究、GPU或其他示例样式重构。
- 不按首轮瞬时axe色值改配色：报告已证明启动opacity过渡与永久调色板不同；稳态复验失败时先核实真实成因。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **[已核实·缺口]** `rendering::preflight`使用冷态组件/字段集合，但未注册名和未知字段只给失败说明；`located`一律设置空建议。helper返回的`invalid_props`映射也没补可执行步骤。固定C5先成功check，再制造错误，已复现候选/建议缺失，位置和锁正确（报告原始反例）。
- **[已核实·缺口]** `markdown::embedded_block`把内置组件错误变为`Issue::error`，未填建议且不使用`SiteError.property`定位，当前保留围栏块位置；纯内容Steps反例不需要Rust工程也能触发。内置registry只有顶层声明，TabSpec/CardSpec/StepSpec内部字段未进入manifest。
- **[已核实·足够]** `report::diagnostic_json/text`与dev展示同一Diagnostic.suggestion。字段已有，不需要新JSON键、私有helper版本或新插件；拒绝/错误收集/事务已存在。
- **[已核实·缺口]** `docs.css`的reduce规则在components层；utilities及SDK未分层规则覆盖其普通声明，默认时长150ms。选择器还只命中`[data-name]`：Switch滑块、Checkbox BOX、Radio DOT、Progress FILL、ColorField input自带过渡却没有该标记；Spinner另有无限`rui-spin`动画。历史CSSOM已见reduce=true仍有150ms opacity过渡；上述漏选元素及Spinner需在M1红基线补采computed/动画证据。
- **[已核实·足够]** 已批准Read设计使用短反馈及原主题；当前0.1.15官方站子路径46项行为通过，稳态定向对比度通过，可作为本增量的限定基线。
- **[已核实·缺口]** 原计划明确Firefox稳定版/引擎P2后移P4，PRD只写P1例外和未指定恢复期。决定唯一，缺的是同步。

### 1.2 文件与边界

| 路径 | 精确增量 |
| --- | --- |
| `crates/rustify-docs-cli/src/rendering.rs` | 冷态候选/字段/类型及helper错误映射的建议，沿用原位置和错误收集 |
| `crates/rustify-docs/src/markdown.rs`、`crates/rustify-docs/src/builtins.rs`、`crates/rustify-docs/src/embed.rs` | 内置错误、YAML/重复键等拒绝路径补建议，复用顶层声明与原serde信息；不新增嵌套schema |
| `crates/rustify-docs/assets/docs.css` | reduce覆盖scope全部后代过渡，并取消SDK Spinner动画；不改调色板/布局 |
| `crates/rustify-docs/src/style.rs` | 必要的生成CSS回归；只有实际修复需要才改生成机制 |
| `crates/rustify-docs-cli/tests/components.rs`、`crates/rustify-docs-cli/tests/check.rs`、`crates/rustify-docs-cli/tests/dev.rs` | 从真实命令检查候选/建议/位置、冷态联合错误、失败保全及恢复 |
| `tests/docs-ssg/dev-components.spec.ts`、`tests/docs-ssg/islands.spec.ts`、`tests/docs-ssg/official.spec.ts`、`tests/docs-ssg/custom.spec.ts` | 新增诊断/reduce定向例及§9.1的稳态axe矩阵，使用既有project；不是复用尚不存在的P2 axe用例 |
| `docs/PRD-DOCS-SSG.md`、`docs/docs-ssg.md`、`DESIGN.docs-ssg.md` | 已批准Firefox边界、实际嵌入诊断说明、必要的reduce实现规则；历史完成记录保持原位 |
| `crates/rustify-docs-cli/Cargo.toml`、`crates/rustify-docs-cli/assets/framework-revision.txt`、`crates/rustify-docs-cli/assets/components-Cargo.lock`、根`Cargo.lock` | 最后固定新R/C、CLI patch及成对模板锁，无第三方升级 |
| ★ `docs/plan/DOCS-SSG-P2-gaps.records/M1.md`、★ `docs/plan/DOCS-SSG-P2-gaps.records/M2.md` | 仅在实际实施发生后创建独立证据记录 |

宏/parser/SDK运行时、主题token值和GPU实现不需要新增能力。共享文件单人负责；本计划规模不另设内部任务状态表。

## 4. 集成与契约

| 依赖/调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| 静态声明/内置registry | 已核实缺口：只有顶层 | names及FieldManifest的name/ty/default可读；没有嵌套字段或必填标志 | 归集一次；按§5.1区分可识别类型与不透明类型，内置复合类型指向现有作者参考 | 不伪造嵌套示例或类型别名的必填语义；无可用注册时给注册步骤 |
| Diagnostic/report/dev | 已核实缺口 | 位置可用、suggestion为空，text/JSON/dev同形 | 增量填字段，不新造协议 | errors仍非零，不用建议覆盖原说明或真实位置 |
| SSR/helper/源码/发布 | 已核实足够 | C5运行/原生及现有联合错误/旧产物不变用例 | 保留接口、编译顺序、守卫和事务；回归实际失败链 | 失败新代不发布，修好恢复，冷态仍收集独立Rust错误 |
| Docs CSS/SDK controls | 已核实缺口 | reduce=true仍有过渡；现有选择器漏控件子元素，Spinner有独立动画 | §5.2明确!important、scope后代范围与Spinner静止声明 | 不改正常偏好或让作者依赖内联样式 |
| 本地固定C5环境 | 已核实足够 | 当前真实安装、warm反例及official-subpath46/46 | 用于红基线；新版本单独固定/实际安装 | 旧证据保留版本，不能冒作新候选通过 |

## 5. 核心机制

### 5.1 可执行的嵌入诊断

1. 从本轮manifest和内置registry一次归集名称及顶层fields；demo候选只取manifest.demos。名称/字段去重后按Rust标识符字典序列出完整集合，不做近似匹配或截断，不维护第二份字典。候选以反引号包围、以逗号加空格分隔；字段候选附规范化类型和下表的边界。`{ty}`与默认表达式`{expr}`显示为规范化token的紧凑写法（去掉标点两侧的分隔空格、逗号后留一空格，字符串/字符字面量原样），如`Vec<StepSpec>`、`"Counter".to_string()`；只改显示，不求值、不改manifest。
2. 错类型与缺值保留原serde说明。只识别清单中直接写出的标准标量、String、Option和Vec形式；类型别名/自定义复合类型视为不透明，不靠名字推断必填或嵌套结构，不求值Rust默认表达式。
3. 注册组件继续按已有property的首个顶层字段定位到Markdown属性行；路径为空、`.`、缺失键没有span时回落到围栏块起点。内置组件继续保留围栏块位置，property只用于选择建议/保留真实字段路径，不改成未经验证的属性行；reserved-component沿用注册文件及原有空行列。不解析错误文本来编造缺字段位置。载荷内位置按Markdown解析器的原始文本段换算回文件偏移：CRLF文件中被丢弃的CR、引用块/列表的容器前缀不再使属性/载荷错误前移（M1红基线：CRLF下`unknown:`键报到上一行行尾、重复键列号少1）。重复键由YAML遍历识别，不读错误文本。
4. helper错误、冷态预检、内置错误及embed-payload使用现有suggestion字段。dev复用已编译代时，预检已拒绝的嵌入（未注册名或未声明属性）不再送helper渲染，避免同一拒绝再以`unknown_component`/`invalid_props`无建议地报第二次（冷路径本就不调用helper）；只在已有错误时生效，成功构建不变。建议为单行英文（沿用现有诊断语言），text的`= help:`、JSON和dev面板的建议文本逐字一致，面板只格式化反引号；原message、rule/severity、位置、去重/顺序、非零拒绝语义与独立Rust错误收集保持。无更多具体信息的IO错误仍可null。
5. 按V-1验证check/build/dev × 普通、中文前缀、CRLF、多个同值字段；同一错误的建议与原位置一致。失败保留旧dist/浏览器last-good，修复后建议/错误清除。

| 声明类型/边界 | 输入建议 |
| --- | --- |
| String、bool、直接声明的数值/char | 固定形状：String用`"text"`，bool用`false`，整数用`0`、浮点用`0.0`、char用`"a"`；示例只说明形状，实际值域仍按原错误修正 |
| `Option<T>`、`Vec<T>`（T为可识别标量） | Option形状固定为`null`并说明可省略/接受T；Vec用对应标量的单元素YAML序列，如`[0]` |
| default存在 | 写明默认表达式原文；可省略，不把Rust表达式当YAML执行 |
| 无default、直接声明Option | 写明可省略/为null；无default且直接声明非Option的已知类型写明required |
| Vec<TabSpec>/Vec<CardSpec>/Vec<StepSpec> | 声明类型及上行适用的必填边界，链接作者参考Static builtins；嵌套对象字段只在该参考维护，不另手写示例或推导schema |
| 作者复合类型/类型别名 | 声明类型、默认表达式（若有）、原serde期望及项目相对注册文件；无default只陈述“未声明默认值”，要求核对注册类型的缺值语义，不武断标required |

作者参考入口沿用生成README的`https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md`，内置类型使用`#static-builtins`；建议输出链接文本，不新增面板动作或构建时联网。引用正文的维护落点是[Static builtins](../docs-ssg.md#static-builtins)。

| 规则/情况 | 单行建议模板（花括号由权威数据替换） |
| --- | --- |
| unregistered-component | Use a component from {names}; for an author component, enable `[rust]` and register its name in `site!`. |
| unregistered-demo | Use a demo from {names}; register demos in `site!`. 无demo时固定为：Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block. |
| unknown-property | Use one of {fields} for `{component}`; rename or remove `{field}`. 无声明字段时：Remove `{field}`; `{component}` declares no props. |
| invalid_props / embed-props，字段可识别 | Write `{field}: {shape}` for declared type `{ty}` ({boundary}). 内置复合类型固定为：Write `{field}` as `{ty}` ({boundary}); see {builtin-reference} for its object fields. 作者不透明类型固定为：Check `{field}` of declared type `{ty}` ({boundary}) in `{registry}`; follow the reported serde requirement. |
| invalid_props / embed-props，缺值/无可定位字段 | Add or correct `props` using the declared fields {fields}; {reference-step}. 内置reference-step指向Static builtins，作者组件指向注册文件；保留原missing-field说明与块位置，不把`.`当字段名 |
| embed-payload，重复键 | Keep each YAML mapping key once; remove or rename the duplicate key. 不编造候选或重复键名，原message/span仍指明实际错误 |
| embed-payload，其他无效YAML/结构 | Write valid YAML for `rustify-component` (`name`, optional `props` mapping and `children` string) or `rustify-demo` (`name` only); correct the reported payload error. |
| reserved-component | Rename `{name}` in `site!` and its Markdown references; builtin component names are reserved. |
| CardGrid.columns值域（builtins.rs自有规则，非serde类型错误） | Set `columns` to 1, 2 or 3, or omit it for the default expression `{expr}`. 形状模板会给出`columns: 0`而与值域矛盾，故由该规则所在模块给出；不追加props path句 |

`{boundary}`统一填`required`、`optional; omission is null`、“default expression `{expr}`”或`no declared default; confirm omission semantics in the registration`；`{shape}`按类型表。`{fields}`每项固定为“`{name}` (`{ty}`; {boundary})”，按完整稳定顺序以逗号加空格连接，空列表明确写`none`。`{builtin-reference}`为上述完整URL加anchor；`{reference-step}`固定为“see {builtin-reference} for its object fields”或“check the registration in `{registry}`”。`{field}`使用可确定的顶层键；invalid_props/embed-props有非空且非`.`的serde property时，建议追加固定句“Reported props path: `{path}`.”保留完整原路径，不把嵌套path当已知类型声明。模板覆盖所有本次嵌入拒绝分支；V-1必须断言具体展开文本，而非仅断言非空。

### 5.2 减少动效实际生效

在docs.css现有`@media (prefers-reduced-motion: reduce)`内，把选择器改为`[data-rustify-scope], [data-rustify-scope] *`，设置`transition-duration: 0s !important`及`transition-delay: 0s !important`。docs.css仍在components层；普通声明的层级先于特异性比较，因此单纯增加特异性不能压过utilities/未分层SDK声明。duration不能用0.01ms，delay同时归零使组合时长为零；不单独覆写会被导入主题重写的motion token。

同一媒体查询对`[data-rustify-scope] [data-name="Spinner"] > svg`设`animation: none !important`，按既有Read合同静止显示Spinner而保留状态文字。过渡归零包含scope内作者组件，作者参考必须说明；不扩为禁用作者所有关键帧动画，不改变SDK其他应用或正常偏好的150ms语义。生成/压缩CSS单测断言媒体查询、选择器及!important保留，再由V-2验证真实级联。

V-2覆盖Switch滑块、Checkbox BOX、Radio DOT、Progress FILL、ColorField input，以及Tabs、Button和Spinner；默认官方站与导入主题fixture都覆盖。对ready/fieldset启用、hover、切tab、切换选择/值、Progress宽度变化，在事件发生前布置观察并在样式刷新后立即采集computed duration/delay及`document.getAnimations()`，断言CSSTransition为0；断言在任何“等过渡结束”辅助函数或重试轮询之前，防止等150ms后假绿。Spinner的computed animation-name为none且无rui-spin CSSAnimation。正常偏好检查原150ms过渡及Spinner仍转动。

把现有islands/official中reduce下“等待过渡结束”的用例改为先断言无过渡。shell/源码控件、loading/failed静态恢复和布局按§9.1复核；只有正常偏好的axe允许等待有限过渡终态，reduce不靠等待隐藏违例。原瞬时失败记录保留原条件。

### 5.3 已批准合同的文档同步

只将原计划C-6/§9.3/§11已记录的Firefox决定同步到PRD WORK-N3/AC-4与Q3：P2稳定版和引擎未验证，恢复期P4，其他桌面与手机缺口不被扩张豁免。保留P1 D-04历史、D15本人C4确认、D16开发/发布顺序及所有原始证据。作者参考的诊断空值边界与实现一致；设计规范只同步真实减少动效规则，不另写新风格。

### 5.4 库R与CLIC本地固定配对

M1通过V-1/V-2/V-4、V-3首次浏览器门及§9.2的fmt/分包test/clippy/docs-cli build后，按原计划§2.1提交真实新库R。M2前置仅为M1完成及R已提交回填；制作C是M2首项工作，不能把“C已准备”作为未归属的前置。

M2递增CLI patch、内置完整expected R、准备模板锁/根版本锁和候选安装说明。C产品提交只允许五文件：`crates/rustify-docs-cli/Cargo.toml`、`crates/rustify-docs-cli/assets/framework-revision.txt`、`crates/rustify-docs-cli/assets/components-Cargo.lock`、`Cargo.lock`、`docs/docs-ssg.md`。提交前用`git diff --name-only`及相对R的diff逐一审计；若中间有协议要求的进度/证据提交，单列它们，产品差异仍限五文件，库/运行资源/功能src/模板源均为0差异。

模板锁只把rustify-components/docs/docs-macros/docs-syntax/style/ui六框架包的Git source配到R，第三方版本/checksum不变；根锁只改CLI版本。用同一完整模板取齐依赖后，分别跑ssr宿主及hydrate/wasm的`--locked --offline` metadata与check，断言无Makepad及锁字节不变；审计/离线检查证据保存到M2后才提交C。无需第三方升级，不用path/file依赖替代产品manifest/lock里的公共Git URL。

提交C后才按§9.2的进程级Git remap + git-fetch-with-cli配方，用`mbx install --git https://github.com/rockie/rustify-ui --rev "$DOCS_GAPS_C" --locked --profile docs-cli --root "$DOCS_GAPS_INSTALL" rustify-docs-cli`真实安装到新隔离根。PATH中的python3须为已有Python3.12；不改全局Git配置或系统Python。`scripts/docs-pages-prepare.py`只是按已安装C准备工程，不负责安装。

记录安装命令、完整C/R、绝对二进制入口、SHA-256及`--build-info`（source=C、panic=unwind）。M2所有serve/dev/Playwright复验显式继承`RUSTIFY_DOCS_BIN="$DOCS_GAPS_INSTALL/bin/rustify-docs"`，逐project核对同一摘要；build-info不记录dirty，故source=C本身不能证明二进制来自安装。

两类消费者分工：真实new生成工程承担V-5的CLI check/build/dev、失败保全、三直接依赖固定R及锁不变/两目标无Makepad；浏览器V-2/V-3使用按C投影的components fixture及官方子路径（prepare保留其余C源字节），不是尚无入口的new工程浏览器项目。准备版/dirty只作M1局部证据，不能冒作M2固定交付。

只把开发候选文案更新到当前配对，README已公开验证的P1安装命令在D16发布前不冒改为未公开C。本地Git传输不等于公共可获取/发布，回滚须恢复完整R5/C5及锁/产物。M2若发现产品缺陷，回到M1修复并重跑受影响首次门，冻结R′、递增CLI patch制作/审计/安装C′；旧候选与证据保留原版本，新的固定出口重新验证。

## 6. 前端与交互

沿用[阅读brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md)、[dev brief](../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md)、[设计规范](../../DESIGN.docs-ssg.md)的Read表面：不改层级、字号、配色、间距或演示/源码结构。M1开头用impeccable确认错误建议的层级、长候选换行、焦点/空错态与reduce下反馈，并在M1记录保存设计结论；V-3首次浏览器门在R冻结前完成。

错误建议由现有dev面板展示，长候选/字段信息在其自身区域换行；保持隐藏/标记重新打开、Esc关闭、读取期间不抢焦点及修复后自动清除（面板内焦点交还main）。位置文本可选中复制，不新增复制位置按钮。reduce下控件状态变化直接显示，正常模式保持既有反馈。浏览器入口为docs-dev自有临时工程及components-subpath/official-subpath，M1使用工作树CLI、M2绑定已安装C；没有Portal宿主。

1280/375px、浅/深、默认/导入主题，以及loading/ready/failed、未知名/属性/类型/修正路径在同一部署合并走查。保留console/CSP/失败请求与必要截图。手机视口/布局缩放、Chromium与真实设备/原生Safari分别记录；原M6环境项不冒作本计划通过。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| WORK-N5 | PRD原阈值及G-05真实动画 | reduce无过渡、SDK Spinner静止；默认新类型严重/重大axe0，文字4.5:1/图形3:1；正常模式不退化 | 实际违例不收口，不改阈值或用截图替代 | Docs reduce的scope后代规则+Spinner规则；§9.1可枚举稳态扫描 | V-2/V-3 |
| WORK-N6 | 既有last-good与publish | 错误仍非零、位置正确、旧产物/浏览器状态不丢失 | 不替换新代/旧产物，不清空原诊断 | 同一Diagnostic及现有失败守卫 | V-1/V-3 |
| C-2 | 固定工具链/锁/来源身份 | 新R/C真实配对、普通内容不新增Rust编译；依赖/产物仍DOM | 身份/锁不符拒绝验收 | 现有生成/metadata/安装流程 | V-5 |

不引入新的远端调用、数据库、遥测或运行协议。建议只使用注册元数据，不暴露机器绝对路径/env值；新CSS仍外部同源，不放宽CSP。原性能数字和当前缺证保留在原计划，本增量不另造性能预算。

## 8. 失败模式与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| 空/错误候选或建议与位置错配 | 本次诊断 | 不改源/旧产物 | 提示无法指导作者或指错字段 | 真实命令+原位置/清单断言 | 修诊断映射，原拒绝/last-good保持 | V-1/V-3 |
| reduce规则被覆盖、漏选控件或未停Spinner | P2文档控件 | 无持久数据变化 | 仍动画、用户偏好未执行 | computed duration/delay/animation-name与即时getAnimations | 修Docs媒体规则的!important/覆盖面/Spinner声明，保持正常偏好；固定候选缺陷走R′/C′回路 | V-2/V-3 |
| 候选版本/锁/资源不成对 | 本地新版本工程 | 新代拒绝，旧产物保留 | 配对诊断；不能验收新版本 | build-info、metadata、锁/完整产物摘要 | 恢复完整R5/C5与模板lock/产物 | V-5 |

仅增加提示与局部CSS优先级，不改输入闸/失败状态机。没有公开发布动作；M2完成只证明增量本地固定交付，原M6/M7及全PRD终验保持各自范围。

## 9. 验证

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | WORK-F4/F15/N6；§5.1各规则逐字模板/完整排序候选；check/build/dev × 普通、中文前缀、CRLF、多个同值字段，每格原位置/建议一致；Counter.initial、Steps.items、作者复合类型/别名含精确类型与位置断言；重复键给去重步骤而非候选；独立Rust错误/旧产物保全 | `mbx test --locked -p rustify-docs`；`mbx test --locked -p rustify-docs-cli --test check --test components --test dev`及CLI其余回归；临时真实工程，text/JSON及dev诊断状态交叉断言；M1记录逐格证据 | M1先红后绿，R冻结前完成三入口矩阵；M2用已安装C重跑受影响真实命令 |
| V-2 | WORK-N5；§5.2全部控件及子元素在reduce接管/各自交互前后duration/delay=0，无CSSTransition，Spinner无rui-spin且静止；正常偏好150ms/Spinner转动；生成压缩后!important保留 | style.rs生成用例；§9.1标记为`gaps reduced motion`的islands/official定向例；M1工作树CLI，M2继承§9.2的PATH/remap/`RUSTIFY_DOCS_BIN`及同一摘要 | M1先补全红基线再绿，断言早于等待/重试；M2已安装C新产物复验 |
| V-3 | WORK-F4/F15/N5/N6；真实dev错误→同文建议→last-good→修复清除，含长候选/空注册/Esc/焦点；§9.1完整可枚举稳态矩阵，默认新类型严重/重大axe0、对比度/键盘/窄屏及console/CSP正常，导入主题另记回归范围 | §9.1给出新增用例所在spec、现有project、命令/清单/稳定判据；M1工作树CLI首验，M2所有serve/dev命令继承§9.2已安装C入口并逐project记SHA/source | M1 R冻结前首次通过全部新增UI风险门；M2固定候选复验，不以原46功能绿代替AA/动效 |
| V-4 | WORK-N3/C-3；Firefox例外与P1历史/D15/D16一致，未新增完成声明；诊断/设计文档吻合实际行为 | 原PRD对应段落、原计划C-6/§11、作者参考/Read规范逐处对照；相关同义文字搜索，原历史记录不改 | M1文档收口；M2最终版本复核 |
| V-5 | C-2及WORK-N6；C提交前五文件允许差异/六框架source及第三方不变审计，模板两目标locked offline metadata/check；提交后真实安装C、new工程三依赖固定R、锁不变/两图/零Makepad及check/build/dev失败保全，实际安装SHA/source与所有浏览器入口一致 | §5.4审计及§9.2安装配方；prepare在安装后用于投影fixture/官方站；new工程独立CLI验收；M1分包检查可按源等价复用，M2重跑实际受影响回归，每个project串行 | M2制作C时先审计/离线检查，提交后安装及固定出口；M2行登记C与证据提交SHA |

### 9.1 V-2/V-3的浏览器入口与扫描清单

沿用`@axe-core/playwright` 4.13.0；当前P2的islands/official/custom没有axe入口，M1需在这些既有spec中新增用例并标记`gaps a11y`，不新增project。dev-components新增`gaps embedded diagnostics`；动效例标记`gaps reduced motion`，同时修正已有reduce等待用例。以下命令在仓根运行，每次只一个project；M1用默认工作树CLI并记录实际R5，M2先完成§9.2环境/安装步骤，所有命令须继承已安装C的`RUSTIFY_DOCS_BIN`，没有该绑定不算固定出口。

```sh
npx playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'gaps embedded diagnostics' --workers=1
npx playwright test -c playwright.docs.config.ts --project=components-subpath islands.spec.ts custom.spec.ts --grep 'gaps reduced motion|gaps a11y' --workers=1
npx playwright test -c playwright.docs.config.ts --project=official-subpath official.spec.ts --grep 'gaps reduced motion|gaps a11y' --workers=1
```

V-3官方清单从本次实际投影的`site/src/lib.rs`枚举全部pages路由（含首页及24类别），再加`quickstart/`和`__gaps_missing__/deeper/`；记录枚举结果并逐路由×light/dark×1280/375px扫描。当前基线为27路由×4组合=108实例，不能只扫Button/Tabs；新增/变动路由以候选源码清单为准。默认主题使用正常动效偏好获取稳态，减少动效另由V-2即时断言，不能用稳态扫描代替。

components-subpath在相同浅/深与1280/375组合扫描下表，使用既有资源延迟/失败注入，记录默认官方主题与导入fixture主题各自范围；导入主题不扩张PRD对任意作者配色的承诺。

| 路由 | 可观察状态/扫描时点 | 用例落点 |
| --- | --- | --- |
| counter/ | wasm保持未释放的loading；释放后全部相关岛ready，各扫描一次 | islands.spec.ts |
| failure/ | 按既有故障注入/触发使岛failed，失败文字/恢复入口和静态画面可见 | islands.spec.ts |
| client-only/ | wasm未释放的固定占位；接管后ready，各扫描一次 | islands.spec.ts |
| custom/ | 定制页/布局slots中的全部岛ready，导入主题已投影 | custom.spec.ts |
| plain/ | 无岛的静态定制页 | custom.spec.ts |
| missing/a/b/ | 深层静态404及共享slots | custom.spec.ts |

稳定边界：逐实例断言表中全部相关岛的目标状态（loading/failed/client-only不能一律等ready），并等待`document.fonts.ready`。正常偏好只等待当前有限CSSTransition的`finished`（dev面板还等其有限入场动画），排除无限Spinner；等待后再次确认目标状态，使用测试已有期限，不加任意sleep。reduce实例不等待动效，先按V-2即时断言无过渡/Spinner静止再扫描。

V-3复用WCAG 2.2 AA tags（wcag2a、wcag2aa、wcag21a、wcag21aa、wcag22aa），逐实例保存route/状态/视口/主题/偏好、build_id、二进制来源、violations与incomplete。默认主题serious/critical均为0；incomplete及对比度需人工核实，键盘/焦点与长建议另走查，不把自动化当完整AA。未覆盖实例明确缺口；原108扫描的瞬时红记录不改写。

### 9.2 集成与检查边界

复用原宏、共享parser、岛状态机与未变SDK旧示例证据，并记录源等价依据；不重新跑GPU整套。更改实际共享组件/主题API时按原V-14追加受影响回归，不能只因原计划绿色省略新影响。

M1退出、提交R之前的明确门：`cargo fmt --all -- --check`；docs与CLI各自`mbx test --locked -p <包>`及`mbx clippy --locked -p <包> --all-targets -- -D warnings`；`mbx build --locked -p rustify-docs-cli --profile docs-cli`；V-3首次设计/浏览器证据。两包不放到CSR workspace union。V-14按实际影响追加artifacts/reliability等docs-verify组并记录取舍。

M1/M2的测试均先在专用验证shell设置以下环境。使用已有Python3.12建立临时python3入口，使serve/dev spec的子进程实际解析同一版本；工具缺失时报告缺口，不安装。Git重映射仅改变本进程读取传输，不改产品URL/锁或全局Git设置。

```sh
set -eu
DOCS_GAPS_REPO=$(git rev-parse --show-toplevel)
DOCS_GAPS_PYTHON=$(command -v python3.12)
DOCS_GAPS_TOOLS=$(mktemp -d "${TMPDIR:-/tmp}/rustify-docs-gaps-tools.XXXXXX")
ln -s "$DOCS_GAPS_PYTHON" "$DOCS_GAPS_TOOLS/python3"
export PATH="$DOCS_GAPS_TOOLS:$PATH"
python3 --version
export GIT_CONFIG_COUNT=1
export GIT_CONFIG_KEY_0="url.file://$DOCS_GAPS_REPO/.insteadOf"
export GIT_CONFIG_VALUE_0="https://github.com/rockie/rustify-ui"
export CARGO_NET_GIT_FETCH_WITH_CLI=true
export RUSTIFY_DOCS_CARGO=mbx
```

M1在该shell中先`unset RUSTIFY_DOCS_BIN`，让serve/dev用例重建工作树CLI；记录其实际R与dirty范围。M2则按下方安装步骤显式绑定新入口，不能继承上轮或全局安装路径。

M2在C提交并回填后，设置`DOCS_GAPS_C`为该完整SHA，再执行真实安装；下列安装步骤不是prepare脚本的职责。同一shell继续跑§9.1的V-2/V-3全部命令，安装前的准备版二进制不复用。

```sh
: "${DOCS_GAPS_C:?从M2记录填入已提交的完整C SHA}"
DOCS_GAPS_INSTALL=$(mktemp -d "${TMPDIR:-/tmp}/rustify-docs-gaps-installed.XXXXXX")
mbx install --git https://github.com/rockie/rustify-ui --rev "$DOCS_GAPS_C" --locked --profile docs-cli --root "$DOCS_GAPS_INSTALL" rustify-docs-cli
export RUSTIFY_DOCS_BIN="$DOCS_GAPS_INSTALL/bin/rustify-docs"
shasum -a 256 "$RUSTIFY_DOCS_BIN"
"$RUSTIFY_DOCS_BIN" --build-info
"$RUSTIFY_DOCS_BIN" new "$DOCS_GAPS_INSTALL/author-site" --template components
```

V-5用该new工程的实际配置进行check/build/dev、两目标图/锁与失败保全检查；serve.sh自动调用prepare投影fixture/官方站，所有子进程继承上述PATH/remap/已安装入口。每个project启动前后记录绝对入口、二进制SHA/source及prepare回执的C/R，确认摘要始终与安装记录一致；不只用source=C认定安装。缺少既有缓存或服务时按仓库规则处理，不另起基础设施。

局部绿色不是整P2完成。原M6的性能/真环境和M7平台预览/实际发布仍未关闭；本计划重验自身新增行为和直接回归，无变化历史门只在限定版本/范围内复用。

## 10. 里程碑与验收安排

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 可执行诊断、减少动效与已批准口径补齐 | 当前C5/R5、报告真实反例及顶层清单/Read表面 | 开头impeccable设计确认，补采全部动效控件红基线；按§5.1/§5.2修订诊断/Docs CSS、新增§9.1可复现用例并同步文档；诊断/CSS共用crate/target由一人负责、Cargo与浏览器串行，已决定的PRD同步可独立进行 | V-1/V-2/V-4、V-3首次错误浮层及全部稳态矩阵、§9.2 fmt/分包test/clippy/docs-cli build均通过后才冻结库R；记录实际CLI/R及真实差异，原输出/已批准决定保持；回写「实施进度」 |
| M2 | 本地固定配对与真实错误/动效出口 | M1已完成，真实R已提交并回填 | 首先按§5.4制作C/锁，完成V-5提交前审计及locked offline检查→提交C→§9.2真实安装→new工程CLI验收与按C投影的fixture/官方站V-3/V-2复验；主实施者独占lock/target/浏览器收口，不推送/部署 | V-1/V-2受影响重验及V-3/V-4/V-5固定出口通过，安装入口/SHA、C/R/锁/完整产物一致，全部确定增量可追溯；产品缺陷先走R′/C′回路再重验；回写「实施进度」 |

提交点：M1全部首次门收口提交功能库R，将R短SHA写入M1完成记录行，按顶部协议单独提交该回填。M2提交前审计通过后单独提交五文件配对CLI C，立即在M2进行中记录行及独立记录登记C短/完整SHA；M2固定出口完成后提交实施证据与最终进度，把证据提交SHA追加到同一M2行的C之后，再按顶部协议另提交SHA回填。R与C之间的纯进度/证据提交在审计中单列，不能夹带产品变化。只暂存本计划相关改动，用户既有状态与缓存不纳入。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| 瞬时axe色值误作永久调色板缺陷 | 会扩大无关配色改动 | 实施者读取报告原红与定向CSSOM确认，按稳定状态复验；若仍失败核实真实成因 | M1/M2视觉验收 | 无产品决策阻塞；必须保留真实测量条件 |
| 选择器漏选/压缩后!important丢失或Spinner仍转动 | reduce交付可能不成立 | M1先补采子元素/Spinner红基线，生成CSS断言及V-2即时浏览器断言确认范围/级联；不以等待结束代替 | M1 V-2转绿、R冻结前 | 已定机制；实际失败不得冻结R |
| 新R/C配对准备 | 旧CLI或旧helper不能证明新发布候选 | 实施者按§5.4实际提交/安装，不把准备版身份充最终C | M2出口 | 已有可用流程；不得预填通过 |
| 原M6/M7尚未全部终验 | 整P2/全PRD不能声明完成 | 原计划按报告待核实项解除条件继续，D16保持 | 原期出口/后续发布前 | 不阻塞本增量或后续本地开发 |

- 最终状态：**Ready**。
- 定级理由：诊断仅依赖现有顶层元数据，类型分类/位置回落/建议模板已定，不假设嵌套schema；reduce的级联与漏选两个成因、scope后代覆盖和Spinner静止机制已定。M1承载设计/全部新UI首验及R冻结门，M2承载C制作/审计/真实安装和固定复验，前置与入口闭合；剩余computed、压缩CSS、axe及新R/C运行结果是实施退出门，不预填通过。Firefox决定沿用，原真环境/性能/公开发布仍由原计划承接，不用本增量关闭。

## 12. 已知坑与历史教训

- CSS中有reduce规则不等于生效：层级竞争与无data-name子元素漏选要分别检查，Spinner还需查CSSAnimation；验证必须看computed/即时动画。
- 初始fieldset解除禁用会触发控件opacity transition，ready不等于所有视觉动画已结束；axe使用真实稳定状态，但减少动效不允许靠等待隐藏违例。
- 组件诊断当前已有文件/行列不等于完整AC：必须校验可用名字、字段与非空具体建议，不重复P1仅验证字段存在的遗漏。
- R/C、模板锁和实际安装身份成对；本地Git remap不意味着公开可获取，原M6来源与失败原件保留。
- mbx回收target后，official预览可能留下指向不存在路径的worktree登记；先用git worktree list --porcelain核对，只解除本次无源文件的过期测试登记，保留用户有效worktree，再重跑prepare/serve，见[历史证据](../validation/docs-ssg-p2/prd-dev-gaps/browser-stale-worktree.json)。
- 原D15本人结果与D16发布后移属于已确认合同，不再次索要招募或现在上线。
- mbx下一个工程首次以Development profile编译（check/dev）时，可能把构建脚本写入mbx out-dirs的文件及新增环境依赖当作输入变化，报`rust-generation-changed`并丢弃该代，第二次即正常（差异报告的冷反例同此）。这是既有行为、不属本计划范围；测试先在良好状态预热开发profile，再比对check与build。
