# CRATES-IO-P1 · Rustify UI · Docs 优先的 crates.io 首发（DOM 核心拆分 · registry 配对 · 项目自供字体）

> **实施须知：直接依据本计划、仓库规则和当前代码实施或续做。** 先读下方「实施者定位」「实施进度」，核对工作树，再按相关正文与里程碑推进、验证、回写进度并按约定提交；使用 goal 执行时也遵循这些约定。
>
> **计划状态：Ready**
>
> 调查基线：2026-10-22 · `2c0897c67347f23cca48e85e0bc2665f0f2d2f86` · dirty（计划、评审与 SDLC 台账未提交；相关代码以当前工作树为准）。
>
> 需求来源：用户需求，无 PRD。用户要求将本仓的库发布到 crates.io，优先 Rustify Docs；2026-10-22 确认先拆出独立 DOM 核心，GPU 发布随后推进；同日提出字体可由使用者自行放置。本期采用默认系统字体、项目按需提供本地字体的方案，取舍见 §0.3。
>
> 本期交付：**可从 crates.io 安装的 `rustify-docs-cli`，以及完整的 Docs/DOM/组件/样式依赖链；安装、两类模板、landing、主题和 islands 都能在仓库外使用。**
> **开发范围：本仓 `crates/` 的首批发布包与 DOM 拆分所需的私有 SDK/桥接改造、发布与验证脚本、CI、作者文档和迁移说明；精确包清单见 §1.2。**
> 本期独特职责：把现有 Git R/C 配对交付扩展为 registry 版本与校验和交付，不改变内容站的静态生成定位；采用 DOM 路径，无新增 GPU 呈现能力。
> **顶层排除：本期不把私有 Makepad 分支及 GPU SDK 上传 crates.io，不重做组件或主题编辑器，不建设 registry 服务，不发布预编译安装器，不部署官方站点。后续 GPU 发布边界见 §0.5。**

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

- 最近更新：2026-10-22 21:16 +0800
- 当前进度：4/6 个里程碑完成
- 当前状态：M1–M4 已完成，M5 进行中
- 最近完成：M4
- 下一步：本地收口全部通过（含 333 项浏览器检查）；九包 R1/C3 归档与 384 个锁定输入已备齐。等待推送 main 和无凭证 macOS/Linux dispatch 授权，执行 V-12 后进入 M6 发布门。
- 当前阻塞：V-12 尚未取得 main 推送/工作流授权；本地工作无未解决缺口。M6 的九包上传及公开说明/标签推送授权、账号、凭证与名称在发布门另行确认，见 §4、§11
- 代码基线：修正库候选 R `4deb101dff9227ac4f0f313c10dc4c7b5815640c`，CLI C3 `ba9260ee08855a7469e4728403c087f35c8406d0`（初始 R0 `37c77f5e`、C1 `6142e85e`、C2 `a9cfa79a` 留档；均未推送）

### 完成记录

| Milestone | 状态 | 更新时间 | 简要记录 | 实现与验收记录 | 提交 |
| --- | --- | --- | --- | --- | --- |
| M1 | 已完成 | 2026-10-22 13:03 +0800 | 新增独立 DOM 核心 `rustify-dom`；`rustify-ui` 变为复用同一份类型与状态的 GPU 门面 | [M1 记录](CRATES-IO-P1.records/M1.md) | `37c77f5e` |
| M2 | 已完成 | 2026-10-22 17:29 +0800 | 九包发布边界、元数据、CSS API、corpus 门与 inspect 已交付；真实归档消费、同源类型和局部门通过 | [M2 记录](CRATES-IO-P1.records/M2.md) | `37c77f5e` |
| M3 | 已完成 | 2026-10-22 18:15 +0800 | Css/Packaged、本地字体快照/产物、完整示例及迁移已交付；局部测试/快速门通过，浏览器用例留 M5 | [M3 记录](CRATES-IO-P1.records/M3.md) | `37c77f5e` |
| M4 | 已完成 | 2026-10-22 19:03 +0800 | registry 配对/缓存、发行/续接、三模式 helper/CI 及候选文档局部门通过，库候选 R 已提交 | [M4 记录](CRATES-IO-P1.records/M4.md) | `37c77f5e` |
| M5 | 进行中 | 2026-10-22 21:16 +0800 | 本地回归、九包独立归档消费、迁移/回滚、作者模拟及 333 项浏览器检查通过；等待 V-12 推送/dispatch 授权 | [M5 记录](CRATES-IO-P1.records/M5.md) | `4deb101d`、`6142e85e`、`a9cfa79a`、`ba9260ee`、`cc10ad5e` |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能 | 用户要求优先发布 Rustify Docs | 从 crates.io 安装 CLI，无需 Git revision；包含已实现的 landing | §5.1、§5.4；V-8、V-14、V-15 | 已确认 |
| R-2 | 功能 | 用户要求发布本仓的库，确认 GPU 随后推进 | 首批发布 Docs 的完整依赖链及独立样式 WASM 库；其余包有明确去向 | §1.2、§5.4、§0.5；V-2、V-7、V-14 | 已确认 |
| R-3 | 功能 | 用户确认独立 DOM 核心 | DOM 核心不依赖 Makepad；SDK 门面复用同一份类型与状态，保留 GPU 实现 | §2；V-1、V-10、V-11 | 已确认 |
| R-4 | 功能 | 当前作者契约：`docs/docs-ssg.md` 的 Commands、组件模板 | content 的 new/check/build 不编译站点 Rust；components 能生成、SSR、hydrate；new 不因生成锁文件而联网 | §5.2；V-8、V-9、V-16 | 继承 |
| R-5 | 功能 | 用户提出字体由使用者放置；本计划选定方案 | 默认系统字体，CLI 不内嵌生产字体；可声明项目本地字体，构建后仍可离线托管 | §0.3、§3；V-4、V-7、V-10 | 方案已定义 |
| R-6 | 功能 | 将 Git 分发改为 registry 分发所需的维护能力 | 版本配对、来源识别、升级、回滚、部分发布续接均有确定行为 | §5、§8；V-5、V-13 至 V-16 | 派生需求 |
| NFR-1 | 可移植性 | crates.io 打包规则与当前跨包资源引用 | 发布包及消费者构建不读取仓根、相邻 crate 或 Makepad 源码，不依赖根 patch | §2.3、§5.3；V-3、V-7 至 V-9、V-12 | 已定义 |
| NFR-2 | 可靠性 | 现有分模式验证、输出事务、islands 配对 | 拆分不制造双份运行时；保留 CSR/SSR/hydrate 隔离、失败保留旧输出及版本拒绝机制 | §2、§3.3、§8；V-1、V-4、V-5、V-10、V-11 | 继承并扩展 |
| NFR-3 | 分发与安全 | Cargo 发布规则、仓库凭证规则 | 包体积满足 registry 限制；上传有白名单、版本及校验和；凭证不进入产物或日志；重复执行不覆盖版本 | §5.3、§7、§8；V-7、V-13、V-14 | 已定义 |
| C-1 | 约束 | `mise.toml`、`CLAUDE.md` Toolchain/Commands | 使用 Rust 1.99.0、mbx 和现有锁文件；通用库依赖采用现有版本作 caret 下限，不升级已锁版本或第三方 Major，不增设基础设施服务 | §2.3、§5.3；V-7、V-11 | 已确认 |
| C-2 | 约束 | `AGENTS.md` 前端验收、README 支持矩阵 | 真实浏览器检查字体、DOM 和 islands；维持 macOS 支持声明，Linux 先验证发布包编译与文档，不宣称完整平台支持 | §6、§9；V-10、V-12 | 已确认 |
| A-1 | 外部前置 | crates.io 首发需要账号与未被占用的名称 | 正式上传时发布人拥有可用账号、邮箱验证和对应权限，名称仍可用或已由本项目持有 | §4、§11；发布人及 Agent 在 V-13 解除，仅阻塞 M6 | 发布前核对 |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 首批发布范围 | Docs、DOM、组件及样式链；CLI 先于独立 style-wasm 上传 | Makepad 分支不挡 Docs 首发；GPU 包另排后续 | R-1 至 R-3、用户确认 |
| D2 | 命名与版本 | 新核心叫 `rustify-dom`；首批统一 `0.2.0`；CLI 包名仍为 `rustify-docs-cli`，命令仍为 `rustify-docs` | 用新次版本明确字体行为及组件 feature 迁移；第三方依赖不升级 | §1.2、§2.2、R-5 |
| D3 | 发布开关 | 根 `publish = false` 保留，逐包显式 `publish = ["crates-io"]` | examples、xtask、私有 SDK/桥接不会被整仓发布 | `Cargo.toml`、NFR-3 |
| D4 | 字体 | 默认系统字体；显式配置的本地文件由站点发布 | 不申请单包上限，不增加字体资源 crate；旧主题的精确字体效果需要作者提供文件 | R-5、§3 |
| D5 | 配对 | registry 精确版本、同源和锁文件校验和；保留受限的源码开发路径 | 不把任意 Git/path 源当作 registry 配对成功 | §5.2、现有 `validate_framework` |
| D6 | 冻结顺序 | 库源码 R → 库归档/索引 → CLI 的模板锁与发行记录 → CLI 源码 C | 不要求库归档和内嵌其校验和的 CLI 出自同一提交 | §5.3、ADR-3 |
| D7 | 自动化 | 复用 Cargo，多包打包、本地临时 registry 和一个标准库 Python 脚本 | 无常驻 registry、容器或新发布平台；CI 先做无凭证验证 | NFR-1、C-1 |

### 0.3 ADR-lite

#### ADR-1：Docs 依赖独立 DOM 核心

- 状态：Accepted，用户 2026-10-22 确认。
- 驱动：R-1、R-3。可选依赖也要满足打包规则，关闭 `gpu` 不会消除 manifest 的发布义务。
- 备选：A 保留现有包结构，先注册并发布 Makepad 分支完整依赖；B 将现有 DOM 与共享逻辑提取为 `rustify-dom`，私有 SDK 门面继续组合它与 GPU；C 发行时裁剪同名 SDK 的 GPU feature，造成 Git 和 registry 同版本能力不同。
- 决策：B。A 需要先处理上游包名、可选扩展和 GPU 资源闭包，不符合优先级；C 会掩盖兼容差异，不采用。
- 收益：Docs 有完整、独立的 registry 依赖链，应用状态和主题仍只有一份实现。
- 代价：需要移动源码并验证类型身份、运行时导出和 GPU 桥接；组件包的 GPU 便捷 feature 需要显式迁移（§2.2）。
- 重评条件：开始 GPU registry 发布时重新核对 Makepad 闭包；不据此把 GPU 依赖加回 DOM 核心。

#### ADR-2：字体由站点拥有

- 状态：Accepted，作为本计划设计选择；用户提出项目自供字体。
- 驱动：R-5。基础文档站本就使用系统字体；CLI 内嵌字体用于导入主题时复现字体，不是静态生成的必要条件。
- 备选：A 保留全量内嵌字体并申请上传上限；B 分字体包并保留内嵌资源契约；C 系统字体为默认，自供本地文件为显式能力。
- 决策：C。A 依赖 registry 管理方，B 为字体增加发布和版本协调成本；C 使资源归属与站点一致。
- 收益：不下载字体即可安装和生成基础站点，不依赖在线字体服务，部署文件仍完整自包含。
- 代价：旧 `theme.document` 不再自动获得特定字体字节；未提供文件的字体由浏览器及系统回退决定，不能承诺与 Theme Studio 像素一致。
- 兼容：配色、间距、圆角与主题数据结构继续工作；新 CLI 输出说明性诊断并提供迁移步骤，保留旧 Git CLI 回滚入口。GPU/Theme Studio 的打包字体策略不改变。
- 重评条件：用户明确需要开箱即用的完整字体分发时，再评估可选资源包，不恢复为 CLI 默认负担。

#### ADR-3：库归档与 CLI 模板锁分步冻结

- 状态：Accepted，沿用现有库 R / CLI C 的单向配对思路。
- 驱动：R-4、R-6。Cargo 归档包含 `.cargo_vcs_info.json`，模板锁又必须包含库归档的 registry checksum；将锁写回同一提交再重包库会改变归档身份。
- 备选：A 要求一个最终 SHA 同时生成库归档和引用这些归档的模板锁；B `new` 时联网生成锁；C 库 R 与 CLI C 分步冻结。
- 决策：C。A 形成循环，B 破坏离线 new 的已有契约。库先冻结归档，CLI 再记录这些确定的版本与 checksum。
- 代价：发行记录必须同时保留 R、C；实际上传分别在对应 detached worktree 执行，不能在 C 重包库冒充 R 的归档。
- 可复现前提：M2 按 §2.3 移除根 bitflags/smallvec patch 及其直接 path 消费边，让根锁保存真实 registry source/checksum。保留各归档的 Cargo.lock，以固定 Cargo 版本、R/C 和已锁 registry 输入重包；不让打包临时选择未经验证的新版本。脚本逐包核对归档锁 digest，并用两份可见兼容版本不同的本地索引验证同一 R 的归档 SHA 不变。
- 重评条件：若未来明确取消内嵌锁文件契约，再评估单提交流程。

### 0.4 职责与事实所有权

| 对象 | 拥有 | 不拥有 |
| --- | --- | --- |
| 本仓库 | 代码、包边界、白名单、配对记录、发行物、验证和迁移文档 | 不替使用者提供业务后端或站点字体 |
| 站点作者 | 内容、主题声明、本地字体文件与实际部署配置 | 不维护 SDK 的版本配对算法 |
| 发布人 | crates.io 账号、邮箱验证、token 与包所有权 | 凭证不写入仓库、聊天或构建产物 |
| crates.io / docs.rs | 名称分配、索引、不可变版本归档和托管文档 | 本仓不重建这些外部服务 |

### 0.5 分期与明确排除

- **本期必须交付**：§1.2 首批九个包的真实发布与安装终验；其中 Docs 可用后才继续上传独立 `rustify-style-wasm`。不能仅以 dry-run 通过宣称完成。
- **后续 GPU 发布**：`rustify-ui`、`rustify-makepad` 和所需 Makepad 分支依赖保留 Git 使用，作为后续独立计划。该期必须核对每个可选依赖、上游重名处理、资源路径、声明许可及 wasm 消费者；本期仅保证提取后现有 GPU 功能不回退。
- 不上传 Makepad 原包名占位，不把上游 registry 同版本静默替换为本仓分支，不发行裁剪后的同名 GPU SDK。
- 不新增字体下载器、CDN、字体子集工具、字体管理 UI、预编译安装器或自动升级守护进程。
- 不自动部署官方站。只适配本仓消费与验证脚本，使后续 Pages 构建能选择新配对；原版本的部署与回滚能力保留。
- 不把发布准备视为账号创建、权限授予、消息发送或外部基础设施安装的授权。

## 1. 当前事实与改动面

### 1.1 已核实的事实

| 状态 | 事实与落点 | 本期增量 / 漏做后果 |
| --- | --- | --- |
| 已核实·缺口 | 根 `Cargo.toml` 设 `publish = false`；主要库继承版本 `0.1.0`，CLI 自有版本 `0.1.32`；内部依赖大量只有 path | 增加版本和逐包发布白名单，否则无法上传 |
| 已核实·缺口 | `rustify-ui` 的可选 `rustify-makepad` 延伸到分支 Makepad；最小实验确认 optional path-only 仍被 `cargo package` 拒绝 | §2 提取核心；不靠关闭 feature 绕过 |
| 已核实·足够 | `binding`、调度、主题和 runtime 大部分不依赖 GPU；`Binding`、`submit_all`、`record` 等已是公开入口 | 保持单一类型，实现 SDK 门面与组件互通 |
| 已核实·缺口 | 生产代码跨包 `include_str!`：`rustify-docs/src/style.rs` 读取组件 CSS；CLI `src/theme.rs` 读取组件 theme-v4 CSS | 改为所属包的常量导出，否则 registry 中无法编译 |
| 已核实·缺口 | CLI `src/new.rs` 要求源码 SHA；`src/rust_site.rs::validate_framework` 只接受指定 Git revision 或严格限定的开发路径 | 新增 registry 配对分支，否则新模板或组件构建失败 |
| 已核实·缺口 | CLI `build.rs` 只向上查 `.git`；最小实验中打包 manifest 不保留 workspace 的 `profile.docs-cli` | 读取本包 VCS 元数据；registry 安装使用标准 release，不要求不存在的自定义 profile |
| 已核实·缺口 | `src/theme.rs` 内嵌七份字体和 Liberation 源码归档；逐文件 gzip-6 合计 24,634,070 字节（估算，不是最终 `.crate` 大小） | §3 去掉生产字体内嵌，不能仅改 include 清单让编译失败 |
| 已核实·缺口 | style、components、style-wasm、现 SDK 和 CLI 的源码测试含仓根 fixture / native 文件引用；components 还读取 docs/components.md | §2.3 逐包划清测试边界，§9.3 保留 corpus 门与哨兵，不把外部资料塞进发布包或悄悄取消覆盖 |
| 已核实·缺口 | 根 patch 把 bitflags/smallvec 换为无 registry source/checksum 的路径包；GPU 闭包的 rustybuzz、pulldown-cmark 也直接读这些路径 | §2.3 消除路径覆盖，避免打包去掉 patch 后按可见索引换版；独立核对源码与归档锁 |
| 已核实·足够 | Rust 1.99.0 的多包 package 会建立 `target/package/tmp-registry`；最小实验中 source replacement 保持 metadata / lock 的 canonical crates.io source | 可在上传前用真实归档验证首发依赖，无需常驻服务 |
| 已核实·足够 | `tests/docs-ssg/serve.sh` 接受 `RUSTIFY_DOCS_BIN`，已有 landing、内容、组件、dev 浏览器项目 | 验收明确注入归档安装的 CLI，防止回退到仓内二进制 |

crates.io API 在调查时对首批已查询名称返回 404；`rustify-style-wasm` 的一次查询遇到 TLS 错误，未确认。404 不预留名称，也不证明账号所有权；统一在 V-13 重查。以上最小实验均在临时目录完成，没有上传。

### 1.2 包清单与发布拓扑

首批版本统一为 `0.2.0`。以下顺序是依赖拓扑和 Docs 优先级，不是并行上传顺序：

| 顺序 | registry 包名 | 源码目录 | 前置本仓包 | 交付 |
| --- | --- | --- | --- | --- |
| 1 | `rustify-classification-macros` | `crates/rustify-style/upstream/classification-macros/` | 无 | 重命名 vendored host 宏包；本地依赖键可继续叫 classification-macros |
| 2 | `rustify-style` | `crates/rustify-style/` | 上一行 | 类合并、扫描与编译核心 |
| 3 | `rustify-docs-syntax` | `crates/rustify-docs-syntax/` | 无 | Docs 声明解析 |
| 4 | `rustify-docs-macros` | `crates/rustify-docs-macros/` | syntax | 声明宏 |
| 5 | `rustify-dom` ★ | `crates/rustify-dom/` ★ | style（utility 可选） | 无 Makepad 的共享核心与 DOM runtime |
| 6 | `rustify-components` | `crates/rustify-components/` | dom、style | DOM 组件和样式 |
| 7 | `rustify-docs` | `crates/rustify-docs/` | dom、components、macros、style | 静态站库 |
| 8 | `rustify-docs-cli` | `crates/rustify-docs-cli/` | docs、dom、syntax、style | `rustify-docs` 命令；Docs 首发完成点 |
| 9 | `rustify-style-wasm` | `crates/rustify-style-wasm/` | style | 独立 WASM 库，Docs 安装通过后发布 |

`rustify-ui` 与 `rustify-makepad` 继续 `publish = false`。根 workspace 默认仍禁止发布；首批包逐个显式允许 crates.io。版本从各 manifest / workspace 读取，发行白名单不维护第二份手工版本表。

| 路径 / 模块 | 改动 |
| --- | --- |
| `Cargo.toml`、`Cargo.lock` | workspace 新版本、DOM 成员、仍保留私有默认与原 GPU release profile；锁定发布依赖 |
| `makepad/platform/Cargo.toml`、`makepad/platform/script/Cargo.toml`、`makepad/platform/studio/Cargo.toml`、`makepad/libs/rustybuzz/Cargo.toml`、`makepad/libs/pulldown-cmark/Cargo.toml`、`makepad/libs/unicode/unicode-bidi/Cargo.toml`、`makepad/Cargo.lock`、`makepad/platform/script/Cargo.lock` | M2 仅将 bitflags/smallvec 消费边改为同版本 registry 精确依赖；同步独立锁与 provenance，渲染器源码不变 |
| `crates/rustify-dom/` ★、`crates/rustify-ui/src/`、`crates/rustify-ui/web/` | §2 的源码移动、门面与重导出；DOM web 资源移到新包 |
| `crates/rustify-makepad/src/pace.rs`、其 manifest | 共用核心 Pace；不改 Makepad 分支渲染器 |
| 首批各 `Cargo.toml`、各包 README / LICENSE ★（缺少时新增） | 版本依赖、元数据、包内容清单、docs.rs 配置；宏包别名与许可 |
| `crates/rustify-components/src/lib.rs`、`crates/rustify-docs/src/style.rs` | 组件 CSS 的具名导出，替换跨包文件读取 |
| `crates/rustify-docs/src/config.rs`、`content.rs`、`site.rs`、`crates/rustify-docs/src/site/registry.rs` | 本地字体配置、输入追踪和 SSR/hydrate 字体策略 |
| `crates/rustify-docs-cli/src/theme.rs`、`rendering.rs`、`assets/fonts/` | 本地字体产物；删除 CLI 专用的生产字体内嵌副本及无用代码 |
| `crates/rustify-docs-cli/build.rs`、`src/main.rs`、`src/new.rs`、`src/rust_site.rs`、`src/release.rs` ★ | registry 安装、来源识别、版本配对、模板与诊断 |
| `crates/rustify-docs-cli/assets/framework-release.json` ★、`assets/components-Cargo.lock`、`templates/`、`crates/rustify-docs-cli/templates/theme.example.json` ★ | 固定版本元组、库 R 和 checksum；完整系统字体示例；两个模板 README 的 Theme/fonts、配对与迁移章节在 C 前改好 |
| `release/crates-io.toml` ★、`scripts/crates-io.py` ★ | M2 交付白名单与 inspect；M4 交付其余打包、冻结、验证、上传与续接子命令 |
| `tests/crates-io/` ★、`tests/docs-ssg/dev-fonts.spec.ts` ★、`crates/rustify-docs-cli/tests/fonts.rs` ★、现有 crates 的相关测试 | 包边界、脚本故障分支、配对投影、混源类型探针、旧项目迁移与自供字体 fixture；逐包保留 corpus |
| `xtask/src/css.rs`、`xtask/src/docs.rs`、`xtask/src/docs_corpus.rs`、`xtask/src/utility.rs` | M1 同步 DOM 源目录、snippet 前缀、框架包清单及其单测；css 生成的组件资源必须在 R 前稳定 |
| `crates/rustify-docs-cli/src/compile_proxy.rs`、`crates/rustify-docs-cli/tests/components.rs`、`crates/rustify-docs-cli/tests/rust_dev.rs` | M1 更新框架路径与包清单；M4 同步源码/registry 消费模式 |
| `scripts/docs-pages-prepare.py`、`tests/docs-ssg/serve.sh`、`tests/docs-ssg/dev-components.spec.ts`、`tests/docs-ssg/islands.spec.ts` | M1 调整核心包名与实际 snippet 前缀；M3 字体断言；M4 处理配对、CLI 身份与临时 Cargo 配置继承 |
| `tests/docs-ssg/custom.spec.ts`、`tests/docs-ssg/fixtures/components/`、`tests/docs-ssg/fixtures/fonts/` ★、`playwright.docs.config.ts` | M3 改为有许可的小型作者字体，生产 root/subpath/meta 验证；另含导入主题但不声明字体的场景 |
| `.github/workflows/verify.yml`、`.github/workflows/docs-pages.yml`、`.github/workflows/crates-io.yml` ★ | 更新 feature 矩阵、保留旧 Pages 配对，增加无凭证发行预检 |
| `site/Cargo.toml`、独立 fixture manifest/锁、`docs/quickstart.md`、`docs/docs-ssg.md`、`docs/themes.md`、`README.md`、`CHANGELOG.md` ★、`docs/crates-io-release.md` ★ | M4 完成迁移/字体/诊断说明与 runbook；上传前保留当前可用安装入口，把 0.2.0 标为候选；M6 上传成功后提交公开默认安装/升级/回滚说明并推送，不部署 Pages |
| `sources.lock.json`、`tests/utility/scanner/inventory.json`、`crates/rustify-style/upstream/README.md` | M2 登记宏包 manifest 变化和 Makepad 依赖来源调整（保留导入指纹，记录 drift）；M3 移除 docs-static-fonts vendor 条目，不改 Makepad/Theme Studio 字体出处 |

无需修改的邻接面：`makepad/` 渲染器源码、广告厂商服务、业务表单后端及 Portal 集成。Makepad 仅调整上表的依赖来源；根 patch 当前也影响 Docs，必须按 §2.3 消除，不能作为发布包的隐含前提。

## 2. DOM 核心与发布包边界

### 2.1 只移动实现，保持同一份类型

1. 将当前 `rustify-ui` 的非 GPU 模块、DOM web/snippet 文件和 utility 的 DOM 部分移动到 `rustify-dom`。`binding`、`mount`、scheduler、runtime、主题、上下文、浮层、请求和诊断的状态均由核心唯一持有；不复制源码、不使用跨 crate `#[path]` 拼装发布包。
2. `rustify-ui` 保留 `region.rs`、`gpu/`、`utility/gpu*` 及 Makepad 接口，重导出核心 API。`theme`、`diagnostics`、`utility` 用局部门面模块重导出核心内容，再加入各自 GPU 部分；原 `rustify_ui::Theme`、`ActionSink` 等路径和类型身份保持一致。
3. `Pace` 的唯一枚举放核心 scheduler；`rustify-makepad` 以 `default-features = false` 依赖核心并重导出该类型。这样 `RegionApp::pace` 与 DOM 调度器无需重复枚举或转换层；核心绝不反向依赖桥接。
4. `gpu_font_family` 和 GPU 字体资源存在性测试留在 SDK 门面；核心保留中立字体目录和 digest 元数据，不带字体字节。GPU `watch_assets` 继续通过核心公开的 `note` / `record` 写同一份日志，无新增全局状态或虚构接口。
5. 核心保留 `csr`、`ssr`、`hydrate`、`utility` 的现有意义，并加只用于宿主集成的 `external-host = []`：GPU 门面启用它，将 runtime 原 `not(gpu)` 的 DOM host 导出判定改为 `not(external-host)`。共享 boot、协议、abort 和 listeners 仍由核心持有。该 feature 不引入 GPU 依赖。
6. `rustify-ui` 的原 feature 转发到核心；`gpu` 另外启用桥接和 `rustify-dom/external-host`。只保留 GPU 代码确实使用的 style 依赖，清理因移动而孤立的旧模块和 imports。

### 2.2 组件与消费者迁移

- `rustify-components`、Docs 和 CLI 的内部 manifest 可用 `rustify-ui = { package = "rustify-dom", path = "../rustify-dom", version = "=0.2.0", default-features = false }` 保持现有源码局部导入名，减少无意义改名；registry 实际包身份是 rustify-dom。
- 新生成的 components 项目使用 canonical `rustify-dom` 依赖名与 `rustify_dom` 导入，本期 Docs components 项目不支持 GPU。GPU 应用使用同一完整 Git rev 的 rustify-ui、components（如使用 Docs，也取同一 rev），由 SDK 显式启用 gpu，使核心来源一致。不能把 Git SDK 与 registry 组件直接组合并声称单核心；Cargo 允许同名同版本异源包并存，跨核心类型不能互传。
- GPU 应用若需 registry 组件，须在应用根以 `[patch.crates-io] rustify-dom = { git = "https://github.com/rockie/rustify-ui", rev = "<同一完整rev>" }` 将核心统一到该 Git rev，版本须满足组件的精确要求；树中仅一个核心才受支持。这不放宽 Docs 的 registry 同源门。混源与修正后的真实类型编译探针纳入 V-3；没有 wasm 链接实测前不把符号冲突写成已发生事实。
- 移除组件包当前仅作便捷转发的 `gpu = ["rustify-ui/gpu"]`，不保留无效的同名空 feature。迁移说明明确：GPU 由应用的 SDK 依赖启用。现有 examples 已在 rustify-ui 上显式开启 GPU；调整 CI 中给组件开启 gpu 的旧循环。
- 更新所有直接受移动影响的资源读取者及独立 fixture 锁。现有 pinned Git `0.1.32` 及其库 R 保持历史可取回，旧项目不被原地改写。
- 迁移配方在 docs/docs-ssg.md、docs/themes.md、quickstart 与 CHANGELOG 中分三类：① 旧 0.1.32 组件项目先备份，用新 CLI 生成 components 项目，保留新 manifest/lock，将作者 src/content/static、配置和主题迁入；手工迁移时同步三条依赖、`rustify_ui` 导入及 feature 转发为 dom，再显式生成锁。② 旧导入主题站点选择系统通用栈以消除 theme-font-system warning，或提供已获使用许可的本地文件并声明；`--deny-warnings` 仍拒绝剩余警告。列明原字体来源/notice，Rustify WenKai 是仓内修复改名版本，Liberation 的历史源码/许可义务不因升级消失；CJK/emoji 回退变化见 §3.2。③ GPU 按上述同源组合迁移。V-9 在上传前从旧 fixture 实际完成前两类升级并保留回滚配方。

### 2.3 可独立打包的源文件与资源

- 在组件包新增 `styles` 模块，导出 `SDK`、`SLIDER`、`CONTROLS`、`THEME_V4` 四个 `&'static str`；现有 `CSS` / `CANDIDATES_JSON` 保持。Docs 和 CLI 经 crate API 读取，资源仍只有组件包这一份事实源。
- 每个首批包的非 dev 路径依赖都同时带匹配的 version；配对紧密的本仓边使用精确版本。helper 的 registry 名加 rustify 前缀，依赖通过 `package` 别名保留上游 Rust 宏导入名。
- serde、serde_json、syn、quote、proc-macro2 的 manifest 要求移除 `=`，以当前下限 1.0.229、1.0.151、2.0.119、1.0.47、1.0.107 使用 caret；CLI 同类要求一并调整，Cargo.lock 保持当前实际版本。不放宽本仓配对、leptos 及其内部配对依赖、wasm-bindgen/js-sys/web-sys 的精确约束。V-3 断言五个范围及根锁版本未意外变化；不要求存在尚未发布的更高补丁，也不在本次顺便升级锁。
- 根 patch 的 bitflags/smallvec 两项移除；§1.2 所列六个 Makepad manifest 中对应 path 边全部改为 registry `=2.10.0` / `=1.15.1`，其余 feature 不变，同时更新根锁与 Makepad 独立锁。现有路径副本保留原位，不删第三方源码。sources.lock.json 保留原始导入指纹并说明这次依赖来源调整，sources verify 应报告对应 drift。V-3 检查 GPU wasm 图中 bitflags 2.x 与 smallvec 1.x 各仅一份 registry 身份并编译现有 GPU 入口；正常的 bitflags 1.x 不算重复。失败则停止冻结 R，不能偷偷恢复 patch 或改选未定义路线。
- 各包显式设置内容清单，包含构建需要的 `src`、本包 web/CSS/模板/上游源码及许可。不要把 `target`、本机目录、生成站点、大型 corpus、其他 workspace 或凭证加入归档。逐项核对 production `include_*`、wasm-bindgen module 和脚本资源都留在包内。
- 仓内约 304 MB 的外部对照测试资料继续留原位。把直接引用仓根的测试函数/模块限定到 `cfg(all(test, rustify_repo_tests))`，在相关 manifest 的 unexpected_cfgs 白名单登记；普通算法单测继续默认运行。CI 明确增加带 `--cfg rustify_repo_tests` 的专门 corpus 门，并断言该门确实运行了 fixture 检查。不能只加 cfg 后丢失原覆盖，也不增加公开的不可移植 feature。包装后默认 `cargo test --lib` 不得再遇到缺失的外部 include。
- `sources.lock.json` 与 scanner inventory 登记真实改变的宏包 manifest/许可元数据；Makepad 依赖来源变化按上条记录，CLI 字体 vendor 条目随 §3.3 删除；不把修改后的文件写成未改上游副本。
- library docs.rs 选一个有效呈现路径：dom/components 用关闭默认 feature 的 ssr，docs 用 pipeline 默认；禁止 all-features 使 SSR/hydrate 或 CSR 混合。各库显式设 `default-target = "x86_64-unknown-linux-gnu"`、`targets = []`；style-wasm 则设 `default-target = "wasm32-unknown-unknown"`、`targets = []` 及 compiler,scanner。README 写明呈现模式、wasm 专属项在 host SSR 文档中不可见及实际支持范围。V-12 的 stable 单目标离线门只验证可移植性，docs.rs 的 nightly 沙箱结果另由 V-15 核对，不将两者视作相同环境（[metadata](https://docs.rs/about/metadata)、[builds](https://docs.rs/about/builds)）。

### 2.4 验证接缝

- 新增 `tests/crates-io/compat/Cargo.toml` ★，作为不发布的独立 workspace：dom、SDK 门面和组件均关闭默认 feature；csr/ssr/hydrate 分别转发，gpu 只转发 SDK 的 gpu。其 lib 测试用真实类型赋值、ActionSink 投递与共享 Pace 验证，禁止再造简化核心当替身。
- 真正的 registry 消费者由被测 CLI 在仓外临时目录生成，不在其 manifest 添加本仓 path 或 Git，也不复用上述兼容 fixture 冒充 registry 安装。源码兼容和发行安装是两个不同观察面。
- 新增 `tests/docs-ssg/dev-fonts.spec.ts` ★，复用现有 dev 测试的临时项目/port 0 启动模式，纳入 docs-dev。它从本地字体 fixture 构建、预览与改文件，验证缺失/恢复；不为字体单独占用固定端口或常驻服务。

## 3. 默认系统字体与本地字体

### 3.1 作者配置

沿用 `schema_version = 1`，在已知 theme 配置中增加可选数组：

```toml
[theme]
document = "theme.json"          # 需要导入主题时填写；不填时仍用默认阅读主题

[[theme.fonts]]
family = "Brand Sans"
src = "fonts/brand-sans.woff2"    # 相对于项目根，建议放在 static/ 之外
weight = "100 900"               # 可省略，默认 "400"；一个整数或升序的两个整数，范围 1..1000
style = "normal"                 # 可省略；normal 或 italic
```

- 数组只注册字体，选用它仍通过 theme.document 的 `font-sans` / `font-serif` / `font-mono` 字体栈。不能把注册字体误当作自动换全站字体。
- 两类 `new` 都写出未启用的 `theme.example.json`，来自 CLI 的单一完整模板文件（§1.2 ★）；其值以 `ThemeDocument::from_legacy(Theme::light(), Theme::dark())` 为基础，两种模式的三个字体槽均改为 `sans-serif` / `serif` / `monospace`，保留全部必需 token。作者复制为 theme.json，在两种模式修改需要的字体栈，再设置 document 与 fonts；README 给出完整步骤，无需构建 Theme Studio。未启用示例不改变默认阅读主题，也不新增 `[theme]` 字体选择键。V-4 校验模板两模式可解析，V-16 只凭这个入口接入字体。
- `src` 仅接受项目内非空本地文件，支持 woff2、woff、ttf、otf；禁止 URL、绝对路径、越界/私有路径、输出目录和逃逸 symlink，复用现有 theme.document 的路径防护。重复 family/weight/style 是配置错误。
- `family` 是非空字体族名，拒绝控制字符，序列化 CSS 字符串时正确转义；weight/style 按上述有限语法验证，不拼接任意作者 CSS。
- 文件不存在、不可读、格式扩展名不支持或配置非法：定位到 `theme.fonts` 对应项的 `theme-font` 错误，check/build/dev 一致。声明的字体必须可发布，不允许静默忽略。

字体族、src 和 weight 描述符以 [CSS Fonts 的 @font-face 定义](https://www.w3.org/TR/css-fonts-4/#font-prop-desc)为语义依据；上述配置刻意只暴露本期所需的本地字体和 normal/italic，不承诺完整 CSS 字体描述符编辑能力。

### 3.2 不要求浏览器拥有工具的字体目录

- 核心新增 `FontResolution::{Packaged, Css}`，`ResolveContext.font_resolution` 默认 `Packaged`；当前 `font_context` 显式保持 Packaged。所有现有上下文字面量补默认值，GPU/Theme Studio 的解析结果保持原行为。
- Css 模式安全解析并保留作者字体栈，不将未识别族名改成 IBM Plex，不依赖静态 FontFace 的资源路径，也不把声明族名当作实际已加载字体。栈末没有未加引号的通用族时，按 Sans/Serif/Mono 槽补 `sans-serif` / `serif` / `monospace`；引号内同名字体仍是具名字体。`ResolvedFont` 记录该策略，`dom_font_stack` 输出此栈；`ResolvedTheme::patched` 沿用策略。不再隐式追加 Rustify WenKai/Noto Color Emoji；CJK/emoji 由作者栈和系统回退承担，GPU 的 Packaged 行为不变。
- Docs 的 `ThemeInput` 增加有 serde 默认值的 `font_resolution`。CLI 对导入主题显式传 Css；SSR 的 prepare_environment 和浏览器 hydrate 读取同一字段。旧字段默认 Packaged 保持协议的增量兼容，不随意改 runtime 协议号；版本元组仍会阻止跨代 CLI/库混用。
- 无 theme.document 时继续使用现有系统字体阅读主题，不生成字体请求。导入主题中未随站点声明的非通用族名给 `theme-font-system` warning，说明将使用本机字体和 CSS 回退、旧内置 CJK/emoji 回退已移除，并给出声明本地文件或改用通用栈的动作；不声称一定缺字或已成功复现，同一族名去重。`--deny-warnings` 保持原命令语义。
- `Description.fonts` 继续表示 SDK 逻辑字体目录的身份，校验含义不扩张为 CLI 内嵌字节；作者字体由 config 和生成输入负责，不伪装成 Makepad 目录资源。

### 3.3 输入与发布结果

1. 在内容依赖图和 snapshot 中登记配置的字体文件；字体变动、删除和恢复触发 dev 重建。跟随现有 snapshot 校验，防止构建期间文件替换造成混合产物。
2. 将文件字节按 SHA-256 发布为 `assets/fonts/<digest>.<ext>`，生成匹配格式的 `@font-face` 和 `font-display: swap`；URL 使用 site.base，内容相同的文件去重。不增加在线请求或构建期下载。
3. CLI 删除 FONTS、生产字体 include_bytes、专用 Liberation 源码内嵌与 `assets/fonts/` 下的工具内嵌副本，同步移除 sources.lock.json 的 docs-static-fonts vendor 条目；只清理这次变化造成的无用代码。Makepad / Theme Studio 的资源和许可不删除。docs/themes.md 的静态站字体段、docs/docs-ssg.md 的导入/诊断段、两类模板 README 同步新契约，不能把旧内嵌字节说明带进 R/C。
4. 小型本地字体测试 fixture 附其许可，仅供测试。用户自行声明的文件与其他显式站点资源一样按字节发布；本期不增加字体许可管理系统。
5. 失败沿用原子生成事务：保留上一份 dist / dev generation，不输出成功但引用缺失字体的页面。主题 token、语言、模式切换与 islands 的实际浏览器表现一起验收。

## 4. 集成与外部契约

| 依赖 / 调用方 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| Cargo 打包与 registry | 已核实足够 | [package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)、[依赖位置](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations)；本地 optional / 多包 / source replacement 实验 | 用归档验证，不用根 patch 或未发布路径冒充成功 | 缺版本、缺文件、checksum 不符即失败 |
| crates.io 名称、账号 | 上传前待核对，仅影响 M6 | [发布前置](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish)；本次名称查询不预留 | 发布人确认登录身份、邮箱、token 的 publish-new 与九包名称模式；已有 crate 的新版本还需 publish-update，安全配置 Cargo 凭证；Agent 重查名称和已有 owner | 未就绪时停在发布门；不另造名字或占位包 |
| 推送与发布授权 | 尚未取得覆盖本期的操作授权；不阻塞本地 M1–M4 | 当前任务仅改稿；[workflow_dispatch 前置](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#workflow_dispatch)要求 workflow 已在默认分支 | 用户在 M5 远端门前确认推送 main（workflow、R、C）及无凭证 dispatch，在 M6 前确认九包上传和成功后的标签/公开说明推送；Agent 先准备可审阅提交、归档和命令，沿用届时已明确覆盖的授权 | 未授权时只暂停依赖该操作的门，不把写计划当作授权；yank 另需当次明确确认 |
| CLI 与组件项目 | 已核实缺口 | `new.rs`、`rust_site.rs::validate_framework`、模板三条 Git 依赖 | §5.2 改为精确 registry 元组及锁文件，保留严格开发分支 | `rust-framework-pair`，附实际与期望版本及升级/回滚动作 |
| 宿主与 hydrate | 已核实足够，字体需扩展 | `crates/rustify-docs/src/site/registry.rs::prepare_environment`、`crates/rustify-docs-cli/src/rendering.rs`、`crates/rustify-docs/src/site/hydrate.rs` | 同一 ThemeInput 字体策略；资源仍由页面 CSS 拥有 | 协议或配对错误拒绝整代产物 |
| 浏览器验证 | 已有入口 | Playwright 配置、`serve.sh` 的 RUSTIFY_DOCS_BIN | 强制使用归档安装二进制，串行运行项目 | 不存在时失败，不回退仓内 build |
| docs.rs / Linux CI | 真实发布尚未验证 | [docs.rs 构建配置](https://docs.rs/about/metadata)；现有 CI 主要 macos-14 | 无凭证 Linux 文档/编译门；上传后核对实际文档版本 | 明确区分 docs 编译与完整平台支持，不以 macOS 代替 |

凭证只由 Cargo 的既有凭证机制或受控环境传入，不作为脚本参数、白名单字段、日志或测试数据。默认 PR/预检工作流不带发布凭证。缺少 crates.io 账号或权限不阻塞本地 M1–M5；缺少 GitHub 推送/dispatch 授权则 M5 不能收口，缺少上传授权或 A-1 则 M6 不得开始。

## 5. registry 配对、打包与发行

### 5.1 使用者入口与来源识别

```sh
cargo install rustify-docs-cli --version 0.2.0 --locked
rustify-docs --version
rustify-docs --build-info
rustify-docs new my-site
```

- registry 安装不带 `--profile docs-cli`：workspace profile 不进入规范化归档；标准 release 的 unwind 行为由归档安装实测。仓内/Git 构建仍使用现有 docs-cli profile，不修改 GPU 的 abort 策略。
- `build.rs` 先读本包 `.cargo_vcs_info.json` 的 sha1，再尝试实际源码 checkout 的 `.git`，最后报告 unknown。使用 JSON 解析器（可复用已有 serde_json 版本作为 build-dependency），不手写截取 JSON。
- 归档元数据优先于祖先目录的 `.git`，防止解压包位于另一个 Git 目录时认错来源。source 仅用于可追溯性，不以源码 SHA 是否存在决定 registry 模板可用性。
- `--build-info` 保持既有 version、panic、grammars、source 行，增加 `distribution` 与一行 `framework: <JSON>`。JSON 记录 registry、精确包版本、wasm-bindgen 版本及库 R；源码开发可标明开发模式，不暴露本机根目录。
- `--help`、包 README 和两个模板的 README 默认给 registry 安装/升级命令；旧 `0.1.32` 的 Git 安装与回滚配方单列为历史兼容入口。包内文案在 R/C 前完成；仓根 README 和作者指南上传前保留现有可用安装及字体说明，把新配方单列为 0.2.0 候选，需引用已移动源码时将旧版链接固定到旧 Git rev；上传后再切换公开默认入口（§5.4）。

### 5.2 模板、校验与缓存

- 首批 three-package 元组改为 `rustify-docs`、`rustify-dom`、`rustify-components`。CLI 的 `assets/framework-release.json` ★ 是发行时冻结的配对记录：schema、精确版本、库 R、wasm-bindgen；与 manifest 和模板锁一致性由脚本验证。
- registry 分支要求作者直接依赖这三个包、各恰好一个版本，来自 canonical crates.io source，并匹配发行记录；整个已解析图不能混入第二个 dom 或同名异源框架。依赖键的 Cargo alias 不改变实际包名判定。
- 开发分支仅允许源码 checkout 构建的 CLI（distribution: source）使用其真实仓根下对应包的 canonical manifest，且版本元组匹配。归档安装的 CLI（distribution: registry）不开放该分支。0.2.0 CLI 对 0.1.32 Git 项目给出 rust-framework-pair 及 §2.2 的迁移/回滚动作，不继续接受旧 Git 元组；“保留旧记录”仅指 helper/Pages 读取旧 C 时继续按旧 R/C 处理，不是 0.2.0 的通用 Git 兼容分支。不接受任意 Git rev、任意 path 或环境变量绕过。
- 配对失败继续用 `rust-framework-pair`；消息列实际和期望，提示一起恢复/升级 CLI、manifest 和 lock，不暴露 token。
- components 模板用 `=0.2.0` registry 依赖和真实 registry checksum 的内嵌 Cargo.lock。`new` 只替换根包名，不联网重新解算，不把 Git source 文本替换成 registry 文本来伪造锁。
- 新旧配对都进入组件编译缓存上下文；更换版本、来源、字体策略或模板锁会失效相关缓存。已有运行时与 helper 结构校验保留。
- `scripts/docs-pages-prepare.py`、fixture helper、toolchain 检查先读真实 CLI 的 distribution/framework JSON，再分支：registry CLI 使用其生成的精确 registry manifest/内嵌锁，保留其余作者字段，不投影成本仓 path/Git；source CLI 的仓内集成测试将三条框架依赖投影到该 CLI 真实 checkout 的 canonical manifest，单独生成 fixture 的 path 锁，不修改发行内嵌锁；旧 CLI 没有 JSON 时才沿用已核实的 Git R/C 分支。三种模式都拒绝身份/版本不一致。这样 C 上源码 test 不依赖尚未上传的九包，归档/browser 验证则只使用临时 source replacement。
- 保留 `mbx xtask docs-verify --group toolchain` 和 `--group features --external` 的历史 Git 消费能力，更新新 JSON 解析及 DOM/snippet 判据；它们不重复充当本次 registry 安装门，本期对应断言分别由 V-8/V-9、V-7/V-10 覆盖，V-6 测其模式选择与旧记录兼容。官方站可继续选择旧配对，不自动部署。

### 5.3 发布工具与 R/C 冻结

新增 `release/crates-io.toml` ★ 只保存包名、目录、拓扑和 docs/remaining 分组；版本仍来自 Cargo。`scripts/crates-io.py` ★ 使用 Python 3.12 标准库编排现有 Cargo/mbx、tar 和公开 registry 元数据，不引入发布框架。

| 子命令 | 输入 / 产物 | 必须保证 |
| --- | --- | --- |
| `inspect` | 当前 manifests；输出发布闭包和缺口 | 包白名单、内部 path+version、无 GPU 依赖、元数据及凭证排除；除本次白名单 workspace 包外，发布闭包不能有 path 源/被 patch 的包；只读 |
| `package-libraries --rev R --out DIR` | 冻结的库 R；产出除 CLI 外八包、临时索引和 receipt | 在 clean 的 detached R worktree 打包，记录版本/文件/归档及其锁 SHA-256；归档锁除自身根项外只有 registry source/checksum；不选 examples/SDK/Makepad |
| `freeze-cli --libraries DIR --library-rev R` | 库归档与索引；更新 CLI framework-release.json 和 components-Cargo.lock | 锁来自真实解算；其他库源码不得改变；写完由实施者提交 C |
| `package-cli --rev C --libraries DIR --out DIR` | clean 的 CLI C；补入 CLI 归档与 receipt | 在 detached C worktree 打包；库文件内容相对 R 未变，沿用冻结的第三方版本集合及真实校验和；不在 C 重包 R 的库 |
| `verify --release DIR` | 完整发行目录；输出独立验证结果 | §9 上传前检查，不上传；源码树不可作为缺文件时的补充 |
| `publish --release DIR --group docs\|remaining` | 完整 receipt；在 R/C worktree 上传白名单包 | 默认无上传副作用，只有该子命令在授权后上传；逐包核对、429 等待与续接；不通过换锁/重包绕过 byte mismatch |

发行目录使用实施者创建的持久路径，放在本仓 target/ 和所有包目录之外；里程碑记录、receipt、截图都不能落进 crate 清单，避免记录本身改变 R/C。receipt 至少含 schema、Cargo 版本、R、C、包名/版本、归档路径及 SHA-256、每个归档 Cargo.lock 的 digest、第三方 registry 包的版本/checksum 集合、拓扑、模板锁 digest、逐包上传/等待状态（含下次可重试时间）和验证引用；不存 token。

脚本与 helper 测试放 `tests/crates-io/test_inspect.py` ★、`test_boundaries.py` ★、`test_release.py` ★、`test_pairing.py` ★。M2 先交付 inspect/边界测试；M4 补齐其余子命令，以本地文件索引、固定归档和注入的上传/查询响应验证：同 checksum 续接、异 checksum 停止、上传超时先查状态、429 等待后续接、不重新解算/打包及两种索引视图下归档身份稳定。替身只测既定协议，不搭 registry 服务、不用生产上传试错。

具体顺序：

1. M1–M4 的 V-1–V-6 和 §9.4 快速门通过后提交库候选 R（含版本、DOM 拆分、自供字体、CLI registry 实现，但尚不把未知的 R 写进自身）。这只是候选冻结，不代表整期验收。
2. M5 先在 detached R 运行 V-11 的库与非 CLI 部分（§9.3 中除 docs-cli test/clippy/corpus 三行），通过后才打包八个库；利用 Cargo 多包打包的临时索引解决未上传内部版本。`--no-verify` 只可用于汇集归档，不能作为可发布结论。
3. 为仓外验证补齐第三方 registry 包，版本/checksum 集合取 R/C 根锁、全部归档锁及 V-10 消费 fixture 锁的 registry 条目并集：复用 Cargo 已下载的 `.crate` 并校验，必要的预取经 Cargo/公开 sparse index 完成。临时 local-registry 只加入这些已锁版本与本次八库/CLI 归档，不运行 HTTP 服务，不加入私有 path 包。
4. 通过 Cargo source replacement 使用临时 registry；已验证该方式保留 canonical crates.io source 和 checksum。生成真实模板锁和冻结配对，写入 CLI 后提交 C；C 中库源码必须与 R 内容一致。
5. C 提交后先运行 V-11 的 CLI 三行；源码 helper 按 §5.2 使用严格 path 模式。通过后只从 C 打包 CLI，检查标准 registry 安装所需的 Cargo.lock、模板、许可、JS、CSS 及新来源标识。复制发行归档到仓外，以临时 CARGO_HOME 的 config.toml 安装和验证；不使用指向仓内的 patch。依次完成 V-7–V-10、V-16，再在授权后完成 V-12。
6. 任一库归档包含的源码/manifest/README/资源/锁改动要求重新提交 R、重打八包、模板锁和 C；只改 CLI 包可保留原 R 和八包归档，重新冻结 C。失效检查按 §9.2 执行。R 未变而 dry-run 归档或归档锁 digest 变化时直接停止上传，报告实际差异与依赖来源；不得自动 freeze 成 C′ 掩盖漂移。修正锁/打包前提后重新走受影响的冻结链。已上传版本不可改字节，已上传包需修改时必须使用新版本并重新配对。
7. 归档体积先保守限制为每包不超过 10,000,000 字节，并核对当前 registry 限制。超限时失败并给文件清单，不自动删必要资源、降低功能或申请上限。

### 5.4 正式发布与升级

- M5 通过且 V-13 及上传授权就绪后才执行 M6。按 `docs/crates-io-release.md` ★ 的 runbook，逐包从冻结 R worktree（CLI 用 C）运行 `mbx publish --registry crates-io --locked -p <白名单包>`；M6 使用真实 crates.io，不携带 staging source replacement。发布前 dry-run 生成归档并与 receipt 的归档及锁 digest 比较，不加 allow-dirty/no-verify 绕过门。
- 按 §1.2 顺序，每个依赖索引可见且下载 checksum 匹配才继续。docs 组八包上传后，V-14 内的公开安装门用全新 CARGO_HOME 和独立 --root、无 source replacement 安装 0.2.0，核对 build-info 的 registry/版本/R/C 与 Cargo listing，跑通 new content → check/build 和 components 的 SSR/hydrate。此门通过才上传 remaining 组 style-wasm；V-15 在九包齐全后只跑一次最终公开终验。出现等待索引超时，先查询 registry 实际状态，不盲目重传。
- 续接时若 name/version 已存在且 checksum 相同，记录已上传并继续；不同则停止，不能覆盖或换一份字节沿用版本号。部分成功没有跨包事务，不自动 yank 已被下游使用的正常依赖。
- 429 表示该次上传未被接受，查询实际版本状态后在 receipt 记 waiting-rate-limit 与可重试时间；按响应的 Retry-After/时间等待，同一 R/C 与归档续接，不重冻、不立即循环重传。没有可解析的等待时间时安全停在该包并要求发布人确认重试时间。脚本允许退出后续接，不要求 Agent 阻塞等待长窗口。crates.io 源码默认新包 burst 5、每 600 秒补一个，九包约需额外 40 分钟等待（不含编译、索引、docs.rs），实际以服务端响应为准；可由发布人提前申请临时提额（[限流实现](https://github.com/rust-lang/crates.io/blob/main/src/rate_limiter.rs)、[上传权限检查](https://github.com/rust-lang/crates.io/blob/main/src/controllers/krate/publish.rs)）。
- V-14 九包可获取后，提交并在授权内推送公开说明：README、quickstart、docs-ssg 的默认安装/升级/回滚段与 CHANGELOG 切到 0.2.0；包内 README/模板仍保持已验收的 R/C 字节。V-15 从公共链接确认是这份说明、按包 README 安装并完成终验。首发标签 `rustify-libs-v0.2.0` 指向 R、`rustify-docs-cli-v0.2.0` 指向 C；V-15 成功后创建并推送标签，记录公开说明 SHA、标签目标和推送结果，不提前宣称发布完成。说明写清九包、字体和组件 feature 迁移、验证平台及限制。
- 后续升级修改实际变化包及其配对记录，重新冻结库/CLI 和锁；不强制所有未来版本永远同步。首次统一版本是本次降低配对复杂度的选择。

runbook 必含：九包拓扑与 R/C/receipt 检查、推送和 dispatch 的具体命令与授权范围、最小 token 权限/包名模式/覆盖等待窗口的过期时间（由发布人选择并记录，不能存 token 值）、本地交互 cargo login 与受控环境 CARGO_REGISTRY_TOKEN 的二选一用法、超时/429/同异 checksum 的续接、公开安装门和最终公开链接核验、标签推送、结束后撤销临时 token 并清理本地凭证。`cargo logout` 只清本地凭证，不代替服务端撤销。yank 仅在发布人当次明确确认包/版本/原因后执行，另记授权与结果。M6 记录回链 runbook 的实际执行版本。

## 6. 作者体验与浏览器验收

本期不设计新页面。字体输出和核心拆分会影响既有 DOM，实施时按仓库要求使用现有产品/设计上下文核对排版与可访问性。

- M5 首次使用者模拟只读归档 README、生成的模板 README、--help 和 C 树中的 quickstart/docs-ssg/themes 作者指南，使用已安装归档 CLI 完成 new content、改成 landing、check/build/preview。安装与 source replacement 由验收环境预先准备，对角色不可见；无需了解 R/C。M6 再从真实公开源按包 README 安装复验，不以 staging 当作上传成功。
- 进阶作者按文档添加本地字体和 components 项目，检查 SSR 初始内容、hydrate 操作、明暗模式以及错误后恢复。无自供字体时页面可读，不出现幽灵字体请求。
- 实测 root 与子路径的字体请求、CSP、console 和失败请求，验证 `document.fonts.load/check` 及实际 computed font-family；截图留在里程碑证据中。
- 核心拆分检查 DOM-only 和现有融合页：DOM/GPU 更新共享状态、卸载/重挂、动作顺序和主题同步。只验证原有能力，不新增 GPU 组件。
- 使用已安装归档 CLI 的绝对路径作为 RUSTIFY_DOCS_BIN；缺二进制直接失败。Playwright 一次一个项目，不并发共享结果目录。环境无法运行时保留未验证项，不以编译或截图代码推断验收。

## 7. NFR 与运行保障

| ID | 基线 / 来源 | 目标 | 失败行为与机制 | 验证 |
| --- | --- | --- | --- | --- |
| NFR-1 | 当前仓根 include、path 依赖和根 patch | 发行目录复制到仓外后仍能构建、安装和生成站点；registry 链不含 Makepad/私有 SDK | 归档独立安装、canonical source 和路径审计；不从源码树补文件 | V-3、V-7、V-8、V-9、V-12 |
| NFR-2 | runtime、调度、SSR/hydrate、原子发布事务 | 单核心实例；字体/主题状态在 SSR/hydrate 一致；失败保持上代产物 | 共用类型、同一环境、按版本拒绝及原事务恢复 | V-1、V-4、V-5、V-10、V-11 |
| NFR-3 | registry 上限、不可变版本与 Cargo 凭证 | 每包低于 §5.3 门限；所有上传字节可追溯；普通 CI 无发布权限 | 白名单、clean R/C、checksum 比较；日志不输出 token；部分成功可续接 | V-7、V-13、V-14 |

不新设吞吐、p95 或安装时长承诺；本期不改请求/worker 热路径算法，原性能预算仍沿用已有验证。记录实际归档大小和干净安装耗时作为发行信息，不伪造目标。

## 8. 失败模式、发布与回滚

| 失败 / 触发 | 影响与数据后果 | 用户表现 | 检测 | 恢复 | 验证 |
| --- | --- | --- | --- | --- | --- |
| 名称被占用、账号未就绪 | 相关包不能首次上传；已生成本地站点不受影响 | 无 registry 版本 | 名称与 owner/账号预检 | 发布门暂停，明确受影响包；未获确认不改公共包名 | V-13 |
| 归档漏 CSS/JS/模板、依赖仓根 | 外部安装失败；无用户数据丢失 | 编译或 new 失败 | 仓外归档安装 | 发布前修复并重新冻结；已上传则新版本修复 | V-7、V-8 |
| 配对或缓存混合 | 拒绝组件编译/整代更新 | rust-framework-pair，保留旧产物 | tuple/source/checksum、helper 和缓存键 | 一起恢复 CLI、manifest、lock；重建，不使用旧缓存冒充新版本 | V-5、V-9 |
| 字体文件缺失、构建中改变 | 新生成失败；已有 dist/generation 保留 | 明确路径错误 | 配置、snapshot、dev 恢复测试 | 提供文件或取消显式声明；未声明字体走系统回退 | V-4、V-10 |
| 中途上传失败或索引超时 | registry 留下已上传的部分包，无原子撤回 | 上层包暂不可安装 | API/索引和归档 checksum | 同一 R/C 续接；成功项不重复上传 | V-14 |
| 新包限流 429 | 当前包未被接受，已上传包保留；九包默认约额外等待 40 分钟 | 本批发布暂停在指定包 | 响应时间、registry 实际状态与 receipt | 按 §5.4 记等待状态，时间到后同一归档续接；token 过期先由发布人更换，不重包 | V-6、V-14 |
| R 未变但 dry-run 字节或归档锁漂移 | 模板与上传 checksum 不再一致 | 停止上传，不能以新字节沿用 receipt | 逐包归档/锁 digest 比对 | 调查锁或路径/patch 源，修正后按 §5.3 重冻；不自动替换 receipt | V-6、V-7、V-14 |
| 已发布版本发生缺陷 | 新消费者可能失败；既有锁仍可能使用原包 | 安装/运行错误 | 公开 registry 安装终验 | 发布修正版；只在发布人当次明确确认后 yank 指定问题版本，不自动 yank 正常依赖 | V-14、V-15 |

回滚使用者环境：首次可回到旧 Git CLI `0.1.32` 的固定提交 `88566ba1ed08223a5834f794840fff5e48f4ba8a`（继续带 `--profile docs-cli`），同时恢复其对应的项目 manifest/lock；后续 registry 版本用 `cargo install rustify-docs-cli --version <已验证版本> --locked --force` 并恢复配对。字体文件和主题声明一起恢复。站点部署按原规则替换完整 dist/manifest。

registry 版本不能覆写；yank 只改变新解算，不删除旧字节或废除已有 lock。维护者在发行记录写清影响和修复版本，不把 yank 当作擦除凭证的手段。

## 9. 验证

### 9.1 检查表

命令中的新增脚本、测试和 workflow 在本计划中均标 ★；参数路径由实际发行目录填写。下表是执行时点的唯一登记，不在其他里程碑重复全量回归。

| 检查 ID | 需求/验收行与可观察结果 | 命令/入口与环境 | 执行时点 |
| --- | --- | --- | --- |
| V-1 | R-3、NFR-2：门面与组件可互传同一 Theme/ActionSink/Pace；各模式单独编译，DOM 图无 Makepad，原 GPU API 有效；CSS/snippet 读取者同步 | ★`mbx test --locked --manifest-path tests/crates-io/compat/Cargo.toml --no-default-features --features csr`，分别换 ssr/gpu；`mbx test --locked -p rustify-dom --lib`；`mbx check --locked -p rustify-dom --no-default-features --features hydrate --target wasm32-unknown-unknown`；`mbx tree --locked -p rustify-dom --target wasm32-unknown-unknown --no-default-features --features csr -e normal,build`，核对无 makepad/私有 SDK；`mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml` 实测 snippet 前缀并同步相关断言；§9.4 快速门 | M1 局部；解除包边界和字体开发前置 |
| V-2 | R-2、C-1：首批恰为九包，所有非 dev 依赖可 registry 解算，私有包不允许上传 | ★`python3.12 scripts/crates-io.py inspect`；★`python3.12 -m unittest discover -s tests/crates-io -p test_inspect.py`；宏包/语法包局部测试及 source inventory；§9.4 快速门 | M2 局部；脚本与白名单在本里程碑交付 |
| V-3 | NFR-1、NFR-2：生产资源均包内；默认单测无外部 include，corpus 哨兵实际运行；registry 依赖与 GPU 单核心边界有效 | `mbx test --locked -p rustify-style --all-features --lib`；`mbx test --locked -p rustify-components --lib`；★`python3.12 -m unittest discover -s tests/crates-io -p test_boundaries.py`（含逐包资源/cfg 哨兵、metadata 对应 doc、工作树库打包 smoke、混源类型失败及同源/patch 后成功探针、五个通用库 caret 下限断言）；`mbx check --locked -p rustify-ui --features gpu --target wasm32-unknown-unknown` 与 bitflags@2.10.0/smallvec@1.15.1 的 `cargo tree --locked -p rustify-ui --features gpu --target wasm32-unknown-unknown -i <包@版本>`；§9.4 快速门 | M2 局部；不复制大型 corpus；源码图与原 GPU 入口验证先于 R |
| V-4 | R-5、NFR-2：系统默认、路径/转义/格式/重复项、Css/Packaged、补通用栈及 CJK/emoji 变化、示例两模式、snapshot 和恢复 | `mbx test --locked -p rustify-dom --no-default-features --features ssr --lib theme::`；`mbx test --locked -p rustify-docs --lib config::`；`mbx test --locked -p rustify-docs-cli --bin rustify-docs theme::`；★`mbx test --locked -p rustify-docs-cli --test fonts`（本地输出/原子生成）；§9.4 快速门。完整 theme.example.json 必须经真实解析器验证 | M3 局部；生产预览及 dev 浏览器在 V-10 |
| V-5 | R-6：registry 正确/错误版本、混源/重复核心、严格开发路径、归档来源优先、缓存身份 | ★`mbx test --locked -p rustify-docs-cli --bin rustify-docs registry_pair`；`mbx test --locked -p rustify-docs-cli --bin rustify-docs release::tests`；测试覆盖 build.rs 来源解析，模拟 Cargo metadata，不上传 | M4 局部 |
| V-6 | R-1、R-4、R-6、NFR-3：帮助/模板、锁替换、三种 helper 模式、脚本失败和续接可执行 | `mbx test --locked -p rustify-docs-cli --bin rustify-docs new::`；★`python3.12 -m unittest discover -s tests/crates-io`（含 test_release/test_pairing，覆盖 §5.3 续接/超时/429/漂移及旧记录）；断言包内/模板 README 和新发行作者指南章节不再声称 CLI 内嵌字体或新项目依赖 framework revision，明确标为 0.1.32 的历史说明允许保留；§9.4 快速门 | M4 局部；真实模板锁在 V-8/V-9；脚本故障分支在上传前完成 |
| V-7 | NFR-1、NFR-3：真实归档齐全、无生产内嵌字体/私有依赖、大小达标，R/C 和真实 checksum 一致 | ★package-libraries → freeze-cli → package-cli → verify；复制归档到仓外；检查归档列表、规范化 manifests、锁、许可和 receipt；用两份兼容新版可见性不同、均含已锁版本的本地索引从同一 R 重包，八包归档 SHA/锁 digest 均不变；不是仅 list 或 no-verify | M5 阶段收口 |
| V-8 | R-1、R-4：归档安装无自定义 profile，panic=unwind；content/landing 的 new/check/build 不编译站点 Rust，可离线 | ★verify 用临时 source replacement 和 `mbx install --path <解压CLI> --locked --root <独立目录>`；实际 build-info；提供仅 Rust 与 CLI 的作者环境，new/check/build/preview/dev 主路径 | M5 阶段收口 |
| V-9 | R-4、R-6：真实 components 锁、宿主/wasm 构建、错误配对拒绝；旧项目可迁移，项目位于仓外 | ★verify 从已安装 CLI 在离线条件下 new --template components，确认无联网/重解算且只改锁根包名；预取后 check/build，核对 SSR/hydrate metadata/checksum；切错版本或旧 Git 元组失败并给迁移动作。用固定 0.1.32 Git CLI 的真实模板项目和一个导入主题站点按 §2.2 升级，检查 --deny-warnings、字体结果及备份回滚；旧 CLI 可从指定 Git C 只读安装到独立 --root | M5 阶段收口 |
| V-10 | R-3、R-5、C-2：页面、字体、islands、子路径/CSP 和 GPU 共享状态无回退 | §9.5 的固定命令/spec 清单；归档 CLI 注入 RUSTIFY_DOCS_BIN；临时 CARGO_HOME 的 source replacement 由 Playwright/webServer/dev 子进程继承；PATH 中 python3 使用 3.12；保留步骤、console、请求与必要截图 | M5 阶段收口；统一浏览器环境，项目串行 |
| V-11 | NFR-2、C-1：既有 workspace、分模式与六包 corpus 覆盖不减少；CSS/来源一致 | §9.3 完整命令组及现有 CI 分模式矩阵；docs 两包独立执行，六包 corpus 哨兵必须实际运行 | M5 阶段收口；库/非 CLI 部分在 R 打包前，CLI 三行在 C 打包前，各一次 |
| V-12 | NFR-1、C-2：包外文档和安装可编译，无仓根/Makepad 资源依赖 | ★crates-io.yml 仅 workflow_dispatch，完整 SHA 输入 library_rev/cli_rev；授权后先推送含 workflow、R/C 的 main，再 `gh workflow run crates-io.yml --ref main -f library_rev="$R" -f cli_rev="$C"`。校验 R/C/库内容和重包 checksum；macOS 归档安装 smoke，Linux 按 §2.3 metadata 分别 cargo doc/check 及 CLI 安装 smoke，依赖预取后 --offline，style-wasm 用 wasm32；保存 run ID/结论 | M5 阶段收口；不重复本地浏览器全量，不推导完整 Linux/Windows 支持或 docs.rs nightly 已通过 |
| V-13 | A-1、NFR-3：授权、包名/版本、身份/owner、token 范围/过期时间、归档和 clean R/C 就绪 | 用户确认 §4 的上传与后续推送范围；发布人完成登录/邮箱，核对 publish-new、覆盖九包的名称模式和可覆盖限流窗口的有效期，已有 crate 上传还需 publish-update；Agent 重查包名/owner、receipt 与 V-6 通过证据；不打印凭证。上层包真实 dry-run 在 V-14 按依赖就绪执行 | M6 发布风险门；先于任何上传 |
| V-14 | R-1、R-2、R-6：真实上传/续接、索引与字节正确、两组之间的公开门明确 | ★publish --group docs；公开下载校验；执行 §5.4 的独立 CARGO_HOME/--root 公开安装门（含 content 与 components），通过后 publish --group remaining；429 按服务端时间记 receipt 并续接。引用 M4 V-6 的故障测试，不在生产注入故障；九包可获取后提交/推送公开说明 | M6 真实发布风险门 |
| V-15 | R-1、R-6：完全不使用 staging、Git 或 path 的真实安装、公开说明、升级/回滚配方 | 九包齐全且公开说明已推送后，全新 CARGO_HOME/独立目录按包 README 执行 `cargo install rustify-docs-cli --version 0.2.0 --locked --root <目录>`（仓内验证用 mbx 包装）；核对 build-info/listing、content/components/landing 主路径和 §8 回滚；查询九包及其 docs.rs 精确版本页面/构建状态，等待成功，失败或仍排队保留未通过项；公开 README/模板链接须读到 0.2.0 说明 | M6 阶段收口；一次最终终验，与 V-14 的组间门及 staging 证据分开 |
| V-16 | R-4、R-5：角色模拟只凭发行候选文档完成站点与自供字体接入 | V-8 通过后，Agent 模拟会 Markdown/HTML、不会 Rust 的作者，仅可读 §6 的归档/模板 README、--help、C 树作者指南；已安装 staging CLI 与替换源由环境准备，角色从空目录创建 landing，复制生成的 theme.example.json、声明本地字体、构建预览并确认加载。卡点先修文档，再按 §5.3 重冻及 §9.2 重验 | M5 阶段收口，早于任何上传；不虚构真人成功率 |

### 9.2 证据边界

- M5 的临时 registry 验证证明归档与配对可用，不证明包名已归属、真实上传成功或 crates.io 安装可用。M6 单独保留这些实际证据。
- 发行物每个包的 compressed bytes、SHA-256、R/C、安装命令与模式写入 receipt；未运行的平台和 browser 项目明确列缺口。
- 重冻 R 会改变各库归档的 VCS 身份和 checksum，八包、模板锁与 C 全部重建，V-7–V-10、V-12、V-16 整组失效；V-11 按实际改变的代码/依赖部分重跑。只改 CLI 包而 R 不变，则 V-7 的 CLI 部分、V-8–V-10、V-12、V-16 失效，库归档证据保留。只改仓库层公开说明不改变归档，仅使对应链接/文案证据失效；未变化的 corpus 或浏览器证据不重跑。

### 9.3 源码回归命令组

以下命令属于 V-11；不与 docs 包合并运行，不在中间里程碑执行全量：

```sh
cargo fmt --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-docs
mbx clippy --locked -p rustify-docs --all-targets -- -D warnings
mbx test --locked -p rustify-docs-cli
mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib
mbx test --locked -p rustify-dom --no-default-features --features ssr --lib
mbx test --locked -p rustify-components --no-default-features --features ssr --lib
mbx check --locked -p rustify-dom --no-default-features --features hydrate --target wasm32-unknown-unknown
mbx check --locked -p rustify-components --no-default-features --features hydrate --target wasm32-unknown-unknown
mbx check --locked -p rustify-docs --no-default-features --features hydrate --target wasm32-unknown-unknown
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-style --all-features --lib
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-dom --no-default-features --features ssr --lib
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-components --lib
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-docs --lib
RUSTFLAGS="${RUSTFLAGS:-} --cfg rustify_repo_tests" mbx test --locked -p rustify-docs-cli --bin rustify-docs
mbx xtask sources verify
mbx xtask css --check
```

corpus 门需增加哨兵测试，确认 flag 生效、原 fixture 清单可读；原生 fixture 对照测试保留其原断言。更新矩阵时只移除组件包已删除的 gpu 便捷 feature 检查，不删除 SDK 的 GPU 检查。此组不取代 V-7 至 V-10 的归档和浏览器验证。

### 9.4 R 前的快速检查

这些是 V-1–V-6 的局部退出条件，与 M5 全量回归分开，不等冻结后才发现静态问题：

- M1–M4 各自退出前运行 `cargo fmt --all -- --check`、`mbx xtask css --check`，并对本里程碑触及包运行限定包的 `mbx clippy --locked -p <包> --all-targets -- -D warnings`。dom/SDK/components 分 csr、ssr，SDK 另测 gpu；Docs/CLI 各自独立，不与 CSR workspace 混跑。M1 包含核心/门面/桥接/组件/xtask 与路径调整涉及的 Docs/CLI；M2 包含八库及 metadata 触及包；M3 为 dom/docs/cli；M4 为 cli/xtask 和实际触及库。
- M2、M3 退出前运行 `mbx xtask sources verify`，分别核对 Makepad 依赖 drift/宏包登记与字体 vendor 删除；不重写原始导入指纹。
- M2 的 test_boundaries.py 对 §9.3 六包按各自 features 加 `--cfg rustify_repo_tests` 并过滤统一新增测试名 `repo_corpus_sentinel` ★，检查每包确实运行了哨兵且 fixture 可读，拒绝“0 tests”成功；完整 corpus 仍只在 V-11 跑。它按各 manifest 的 docs.rs features/target 调用限定包 `mbx doc --locked --no-deps`，dom/components 关闭默认 feature；并对八库做工作树 `mbx package --locked --allow-dirty --no-verify -p <包>` 多包 smoke。这里 allow-dirty/no-verify 仅探测内容清单，不产生可上传 receipt，不替代 R 上的真实打包与包外构建。

### 9.5 V-10 的浏览器用例与环境

验收入口继承同一临时 CARGO_HOME，其中 config.toml 将 canonical crates.io source 替换为 V-7 的 local-registry；显式预取依赖后按锁离线执行。CLI 的 Cargo 子进程、serve.sh、Playwright webServer 与 dev 临时项目均须继承它，不只给父命令传 `--config`。PATH 放置指向已有 python3.12 的临时 python3 入口，预检 `python3 --version` 为 3.12；不修改系统 Python。设置 RUSTIFY_DOCS_BIN 为归档安装的绝对路径，禁止 fallback 源码 CLI；源码 GPU 示例保留仓内实际路径与 Makepad 构建。

以下命令依次执行，一次一个项目，运行后将该次结果/截图保存到 M5 记录所链的独立目录，防止下一条覆盖报告：

```sh
npx playwright test -c playwright.docs.config.ts --project=docs-root
npx playwright test -c playwright.docs.config.ts --project=docs-subpath
npx playwright test -c playwright.docs.config.ts --project=docs-meta
npx playwright test -c playwright.docs.config.ts --project=components-root islands.spec.ts custom.spec.ts
npx playwright test -c playwright.docs.config.ts --project=components-subpath islands.spec.ts custom.spec.ts
npx playwright test -c playwright.docs.config.ts --project=components-meta islands.spec.ts custom.spec.ts
RUSTIFY_DOCS_FONT_SCENARIO=system npx playwright test -c playwright.docs.config.ts --project=components-root custom.spec.ts --grep 'system font fallback'
RUSTIFY_DOCS_FONT_SCENARIO=system npx playwright test -c playwright.docs.config.ts --project=components-subpath custom.spec.ts --grep 'system font fallback'
RUSTIFY_DOCS_FONT_SCENARIO=system npx playwright test -c playwright.docs.config.ts --project=components-meta custom.spec.ts --grep 'system font fallback'
npx playwright test -c playwright.docs.config.ts --project=docs-dev dev.spec.ts dev-components.spec.ts dev-i18n.spec.ts dev-uat.spec.ts dev-fonts.spec.ts
mbx xtask docs-verify --group features
npx playwright test -c playwright.docs.config.ts --project=dom-only dom-only.spec.ts
npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts
mbx xtask build-web --example fusion-basic --release
npx playwright test --project=fusion-basic m2-runtime.spec.ts m3-state.spec.ts p3-instances.spec.ts
mbx xtask build-web --example property-workbench --release
npx playwright test --project=property-workbench m6-theme.spec.ts
```

components fixture 显式注册 `tests/docs-ssg/fixtures/fonts/` ★ 的有许可小字体（至少覆盖 woff2 和 ttf），custom.spec.ts 比对作者文件 SHA、`assets/fonts/<digest>.<ext>` 的 root/base 路径、对应 MIME、CSP 与 document.fonts.load/check。serve.sh 和 custom.spec.ts 增加仅供测试的 RUSTIFY_DOCS_FONT_SCENARIO：默认 declared 使用此配置；system 在临时副本保留 document、移除 fonts 声明，以新增的 system font fallback 测试断言系统回退、字体请求为零且无 404。上述三个额外命令分别重建/预览该变体，不把两份输出混放进同一 manifest。dev-fonts 验证修改/删除/恢复时保留或替换整代产物。components-webkit 的 custom 断言同步新资源契约，其运行不计入本期 Chrome 门。融合页覆盖 mount/dispose、动作/共享状态/实例隔离，property-workbench 的 m6-theme 覆盖 DOM/GPU 同步与宿主主题隔离；不运行与本次无关的耐久或性能组。

## 10. 里程碑与验收安排

本期两个阶段：M1–M5 产出可公开交付的首批归档并收口；M6 完成不可替代的真实发布与公开消费者终验。M4 额外冻结 R 是消除 checksum 循环所需的提交点，不冒称阶段终验。

| # | 里程碑 | 前置依赖 | 内容与并行边界 | 验证/退出条件 |
| --- | --- | --- | --- | --- |
| M1 | 独立 DOM 核心与 SDK 互通 | 当前 SDK 源码和本计划 §2 | 新增核心并移动实现；单人负责根 manifest、核心、门面、Pace、CSS 工具与 §1.2 路径读取者；同步依赖、feature 和 snippet 断言 | V-1（含 §9.4）；回写「实施进度」 |
| M2 | 首批包自包含与元数据 | M1 核心类型和路径经 V-1 验证 | §2.3 内容/corpus/依赖边界、宏包、provenance、README/docs.rs；先交付 inventory/inspect 和测试，再 CSS 导出。单人负责全部 manifest/锁及 sources.lock | V-2、V-3（含 §9.4）；回写「实施进度」 |
| M3 | 系统字体与项目本地字体 | V-1；CLI theme.rs 改动另需 styles 导出合入且 `mbx test --locked -p rustify-components --lib` 通过 | 核心与 Docs 字体可与 M2 先做；theme.rs 在 M2 替换跨包 include 后移交 M3。M3 负责 §3、字体 fixture/spec/文档、vendor 删除；manifest/锁仍由 M2 负责人单写，sources.lock 按 M2 后再 M3 串行合入 | V-4（含 §9.4）；回写「实施进度」 |
| M4 | registry CLI 契约与库候选冻结 | V-2、V-3，字体环境字段经 V-4 验证 | §5.1/§5.2 来源/配对/模板/缓存；交付发行脚本其余命令及故障测试、消费 helper、dispatch CI、迁移与候选文档/runbook。包内文案随 R/C，公开默认安装入口留 M6；局部快速门通过后提交 R | V-5、V-6（含 §9.4）；R 已有真实提交，不预写自身 SHA；回写「实施进度」 |
| M5 | 上传前发行收口 | V-1 至 V-6；R 可取回；Rust/Node/浏览器/现有依赖就绪；远端 V-12 前取得推送与 dispatch 授权 | 严格串行：V-11 库/非 CLI → 库归档 → 模板锁/C → V-11 CLI → CLI 归档 → V-7–V-10/V-16 → 授权推送 main 及 V-12；冻结 receipt，不上传。包内文档卡点按 §5.3 重冻 | `4deb101d`、`6142e85e`、`a9cfa79a`、`ba9260ee`、`cc10ad5e` |
| M6 | 真实 crates.io 发布与公开终验 | M5 发行目录、V-13；覆盖上传与后续标签/公开说明推送的授权 | 按 runbook 从 R/C 串行上传，Docs 组公开门通过后发 style-wasm；推送公开说明、V-15、成功后推送标签/最终记录。凭证与生产 registry 单人负责，等待/续接状态入 receipt | V-13 至 V-15；九包及文档实际可获取，未完不得视为发布成功；回写「实施进度」 |

提交点：M4（库候选 R）；M5 中冻结模板锁后提交 CLI C，再打包/验收，授权后推送含 workflow、R/C 的 main 以运行 V-12，阶段完成提交证据回填；M6 按 §5.4 提交/推送公开说明，终验后标签推送，再最终提交发行记录与进度回填。修复库时按 §5.3 重建 R/C，不用 dirty 或修改归档逃过冻结。当前分支工作，不新开分支；detached worktree 仅用于冻结和隔离检查；提交与推送授权分别核对。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人 / 解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| A-1：账号、权限、包名竞争 | 正式上传 | 发布人配置账号凭证；Agent 按 V-13 核对真实名称/版本/owner，冲突时停止并说明 | M6 上传前 | 只阻塞 M6，不阻塞开发 |
| 推送/dispatch/发布授权 | 远端 V-12、上传和公开链接/标签 | 用户确认 §4 范围；Agent 先准备提交、归档、命令及差异供审阅，届时沿用已覆盖的授权；本轮仅改稿 | M5 V-12 前 / M6 上传前 | 不阻塞 M1–M4；未取得则阻塞对应门，不能记该阶段完成 |
| 拆分后的类型身份或 runtime 导出 | GPU/组件互通 | M1 先验证，M5 实际浏览器覆盖；不复制状态或临时禁用 GPU 测试 | V-1 / M5 | 失败时阻塞依赖任务 |
| 旧主题字体视觉变化 | 升级后的排版 | §3 明确 CSS 策略、自供文件、诊断与旧 Git CLI 回滚；在发行说明突出 | M3 / M6 | 设计已定义，验收未执行 |
| R/C 库源码漂移 | checksum 和模板锁失效 | receipt 检查库内容相对 R，变化就重新冻结，不在 C 重新打包库 | M5 / M6 | 失败时阻塞上传 |
| 首发依赖尚未在 registry | 不能直接用真实源验收上层包 | M5 用 Cargo 的归档和临时 local-registry，M6 再做真实源安装 | M5 / M6 | 技术路线已用最小实验验证 |
| Linux / docs.rs 当前尚无实际发行证据 | 公开构建可移植性 | 新 CI 文档/安装门、真实上传后核对 docs.rs；不声称旧版本已支持所有平台 | M5 / M6 | 对应阶段必须补证 |

- 待用户拍板：无影响开工的未决设计。DOM 核心路线已确认；字体以完整示例加现有 document 入口，不新增选择键；发布依赖来源和 caret 边界已定。§4 操作授权留待具体产物可审阅时确认，不能提前推定。
- 最终状态：**Ready**。
- 定级理由：包边界、同源组合、字体入口/回退、锁及归档来源、快速检查、上传前作者验证和续接行为均有确定路线，可从 M1 开工。GPU registry 发布留后续；M5 的远端操作授权、M6 的账号/名称/上传与推送授权是明确的阶段门，尚未取得不代表已满足。本状态只表示文档可执行；评审时实施为 0/6，当前实施以「实施进度」为准，浏览器与发行终验仍按对应阶段检查。

## 12. 已知坑与历史教训

- optional path-only 仍是发布依赖；不能以 DOM-only 构建无 Makepad 证明 manifest 可发布（本次最小打包实验；§2）。
- workspace 的 docs-cli profile 不随 `.crate` 保留；registry 安装命令不能照抄 Git 配方（本次规范化 manifest 实验；§5.1）。
- 根 patch 和相邻 crate 的文件在真实安装中不存在；只有包外安装能证明自包含（`Cargo.toml`、当前 CSS include；V-7 至 V-9）。
- Cargo 去掉 patch 后会为归档重建锁；只冻结源码 SHA 不足以解释路径源变成 registry 的版本。§2.3 先消除该覆盖，归档锁与字节都要比对，漂移即停（ADR-3、V-3、V-7）。
- 扫描发现的大型 fixture 不是发行资源；保留独立 corpus 门，不能通过裁掉测试取得虚假绿色（style 的外部 include、§2.3）。
- CLI 版本号不能区分旧 `0.1.32` 与带 landing 的 main；新发行同时记录版本、R/C 和 checksum（`docs/docs-ssg.md` Landing pages、`DOCS-LANDING.md` 恢复快照）。
- `target/` 可能被 mbx GC；最终 receipt、必要截图和验证结论留在持久的里程碑证据目录，不只放 target（`DOCS-LANDING.md` 已知坑）。
- Docs 两包不得与 CSR workspace 联合 test/clippy；Playwright 项目必须串行（`CLAUDE.md` Commands/Browser tests）。
- 本机 PATH 的 python3 是 3.9.6，serve.sh/CLI 集成 helper 使用 tomllib，当前脚本还用 hashlib.file_digest；V-10 必须把已有 Python 3.12 暴露为子进程的 python3，不能只在父命令写 python3.12。
- 上传超时不等于未上传，yank 不等于删除；先查实际 registry 版本及 checksum 再恢复（[Cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)、[yank](https://doc.rust-lang.org/cargo/reference/publishing.html#cargo-yank)）。
