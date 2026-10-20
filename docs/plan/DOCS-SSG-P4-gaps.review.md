# DOCS-SSG-P4-gaps · 开发计划完整评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md) | SHA-256 `741694ad3ab36d7118b5b6cab04da7fa817eb7f830505644185e74c13418e0e9` | 全文；G-01、G-02/D-01；M1–M3、V-1–V-9；首次评审 |

评审日期：2026-10-20（Asia/Shanghai）。代码基线：`main` / `4894d4ccc7c462a344489748553615a352031192` / dirty。调查事实源为当前工作树；初始未提交项限于 `.xgent-ai/` 与 `docs/plan/`，计划本身未跟踪，相关产品源码、指南、构建配置与 HEAD 一致。报告交付前再次核对，计划哈希、HEAD 和相关产品路径未变。未提交的计划、报告和台账仅在本工作树可用，哈希不代替内容同步。

背景来源：[PRD](../PRD-DOCS-SSG.md)（SHA-256 `ab9b1e870b42337a471da1a57ecabe85934b6a7124ae6b4f5e7ea3aa698fb584`）、[P4 覆盖核查](DOCS-SSG-P4.prd-dev-gaps.md)（SHA-256 `f2c4d814059d971ac88704b14cf7df56321470cac858735cd4b085e69997ade6`）及其 D-01、[原 P4 M7 记录](DOCS-SSG-P4.records/M7.md)。这些是需求和证据来源，不是本轮修订对象。

## 1. 确认问题

### [P2] F-01 · V-4 应使用真正的本地 C1 归档

- **定位**：[计划 §9 V-4](DOCS-SSG-P4-gaps.md#9-验证) L283；关联 D5（L91）、异常变化处理（L261）、本机与 CI 边界说明（L325）；影响 M2。
- **问题**：V-4 要求本机 C2 与“C1 本地合格归档”比较，却指定 `c1-qualified-archive.json`。该收据指向 CI 归档。按此执行，即使本次内容修改完全正确，也会把既有的跨环境 Wasm 与组件页面差异判为新增变化，触发停止发布或无必要的补验，M2 无法按既定比较门退出。这与 L325 的“本机 C2 与本机 C1”要求不能同时成立。
- **证据**：[qualified 收据](../validation/docs-ssg-p4/m7/publication-20261019/c1-qualified-archive.json) L2–10 记录 run `39219151112`、build `9e8ea80d57750509` 和 CI `artifact.tar`；真正的[本地 C1 收据](../validation/docs-ssg-p4/m7/release-20261019-c31/c1-archive.json) L2–10 记录 build `7e1e5c9e7e7392df` 和本地 `c1-1146b7d8-site.tar.gz`。[M7](DOCS-SSG-P4.records/M7.md) L88、L94、L111–124 明确区分两类归档。
- **本轮复现**：主 agent 用 Python `tarfile` 直接读取两份现存归档，未解包写盘；各 97 文件，归档 SHA-256 均符合收据。共有路径中 49 文件字节不同，其中 48 个组件 HTML 加 manifest；两边另各有 6 个不同路径的岛资源。本地岛目录为 `b9140be3…`，CI 为 `916dc515…`。因此这是实际输入错误，不能通过调整描述文字消除。
- **最小修订**：V-4 精确链接本地 `release-20261019-c31/c1-archive.json`，标出本地 build 身份。保留 qualified CI 归档供线上证据使用。若实施时本地归档不可用或环境已变化，先按同一环境重建 C1，再比较 C2；不能以 CI 归档替代。V-7 继续独立核对 CI 同 run 与线上产物。
- **复核**：改稿后沿链接确认收据、归档、manifest 均为本地 C1；实施时本机 C2 对同环境 C1 仅出现计划允许的文件变化。
- **置信度：高**；阻塞 M2/V-4，M1 可独立推进。

### [P2] F-02 · 将发布后说明的公开推送与读回纳入 M3 出口

- **定位**：[计划 §5.4](DOCS-SSG-P4-gaps.md#54-发布与公开说明m3) L240–245、[§8 发布顺序](DOCS-SSG-P4-gaps.md#8-失败模式发布与回滚) L271–274、[M3 退出条件](DOCS-SSG-P4-gaps.md#10-里程碑与验收安排) L298。
- **问题**：唯一明确的 `main` 推送安排在发布前；README 和作者参考的 C2 安装、C1 回滚及新 run 说明则要在发布成功后才修改。后续步骤与 M3 出口只要求“说明已提交”。严格完成所有现有检查后，Pages 可以已经发布 C2，而公共 `main` 的安装说明仍指向 C1，公开说明这一交付物没有真正交到读者手里。
- **证据**：[README](../../README.md) L205–208 是公开安装入口，当前明确写 CLI `0.1.31` 和 C1；[作者参考](../docs-ssg.md) L20–41、L59–81、L1385–1416 承载安装、回滚及线上交付说明。[工作流](../../.github/workflows/docs-pages.yml) L6–12 仅由 dispatch 触发；全文没有提交或推送源文档的步骤。计划全文也没有发布后第二次推送与公共读回任务，现有 V-6–V-9 不检查这些后写的说明。
- **最小修订**：在 C-2 的授权范围中明确包含发布后说明提交的快进推送；§8 在说明提交后增加该推送，并从公共 `main` 读回 README 与作者参考，核对 C2/0.1.32、last-good C1/0.1.31、实际发布 run。M3 出口改为说明已公开且读回通过。无需因说明提交重新 dispatch 已批准的 C2。
- **复核**：公共 `main` 包含说明提交，公开安装与回滚身份正确；Pages manifest 仍对应批准的 C2。推送受阻时 M3 保持未完成，不能只凭本地提交收口。
- **置信度：高**；阻塞 M3 的公开说明交付。

未发现 P0/P1。两条问题根因不同：F-01 是候选验收输入错误，F-02 是公开交付任务缺失。

## 2. 待核实与待决策

没有新增的产品取舍需要用户现在决定。以下是已排入计划的未来验证门，不把尚未实施当作缺陷：

| 项目 | 当前证据与缺口 | 责任 / 最晚确认点 |
| --- | --- | --- |
| C2 内容与候选产物 | 新 frontmatter、两处新增断言、C2 及其产物尚未形成；当前只能评估路线与验收设计 | 主实施者；M1/M2，V-1–V-5 |
| A-1 / C2 GitHub 阅读 | 已实看 C1 中文 quickstart 的 frontmatter 表格、H1、正文及代码块，支持同构方案；未检查未来 C2 的三份指南 | 主实施者；M3 推送后、dispatch 前，V-6 |
| 发布授权与外部状态 | 现有公共站可读；本轮未核对 Pages 后台权限、未取得或执行本次发布授权 | 用户给出覆盖范围的授权，主实施者执行；M3 发布门 |
| C2 线上及公共安装 | 本轮没有 C2 部署、公共安装或回退实跑 | 主实施者；M3，V-7–V-9；失败按固定 C1 恢复 |

原报告 U-01/U-02、手机实机及 CLI 回滚提示修订仍按本计划排除，不扩成本轮阻塞项。VoiceOver 保留 D18 的本人验收归属，不改称本轮 Agent 采证。

## 3. 覆盖、事实与验证

### 3.1 实际分工与任务台账

有 4 个总槽位，先并行启动三个只读 reviewer，再复用已结束的 architecture agent 审 delivery；各视角按技能要求返回专属结果，主 agent 逐条回读原证据并单独写报告。第四视角复用了上下文，不宣称四个完全隔离会话。所有 reviewer 均已结束。

| 维度 / 负责人 | 覆盖与结论 | 状态 / 证据缺口 |
| --- | --- | --- |
| facts / `facts` | 描述与回退语义、catalog 保留前文、固定语料、路径和候选事实；确认 F-01 | 完成；未验证未来 C2 |
| architecture / `architecture` | frontmatter 路线、R0/C2 边界、安装、构建、归档与回退；保留技术路线，确认 F-01 | 完成；未执行构建或发布 |
| schedule / `schedule` | 双向需求映射、候选消费链、M1→M2→M3、发布风险门与资源边界；确认 F-01 | 完成；无须新增里程碑 |
| delivery / 复用 `architecture` | 作者安装、AI 发现、GitHub 阅读、发布、恢复、公开说明；确认 F-02 | 完成；C2 的实际效果待实施 |
| 综合 / 主 agent | 复核原始代码、两份 TAR、现有站与 GitHub；去重裁定、报告写入与回读 | 完成；不代替未来 C2 验收 |

### 3.2 保留的关键技术判断

| 计划选择 / 主张 | 已核实依据 | 判断 |
| --- | --- | --- |
| 在指南 frontmatter 写描述，不改框架语义 | [ai.rs](../../crates/rustify-docs/src/ai.rs) L27–34 优先显式描述；L62–93 处理缺译标注、源链接与输出 | 满足 D-01；无须新增生成逻辑或 GPU 路径 |
| meta、OG、Twitter 同源 | [content.rs](../../crates/rustify-docs/src/content.rs) L1872–1876、[shell.rs](../../crates/rustify-docs/src/shell.rs) L494、L503、L508 | 现有链路支持；可在 V-4 的 HTML 差异核查中明确逐项对源描述断言，属于非阻塞补强 |
| components 指南不改生成器 | [catalog 生成器](../../xtask/src/catalog.rs) L25–33、L53–69；[同步测试](../../crates/rustify-components/src/catalog.rs) L728–735 | 保留标记外内容，新增 frontmatter 不要求扩大改动 |
| 内容提交 + C2，框架保持 R0 | [prepare](../../scripts/docs-pages-prepare.py) L136–184 从 C 构造 detached worktree，仅投影消费文件；不要求 C 的父提交为 R | 可行，保留补丁配对方案 |
| 测试真正消费固定 C2 | [toolchain](../../xtask/src/docs.rs) L1354–1394、L1437 按指定 Git/rev 安装并校验 source；[serve.sh](../../tests/docs-ssg/serve.sh) L85–114 从 CLI 身份取 C 并构建 detached C | 身份链成立；C2 后不再修改受测指南源内容 |
| 未改搜索与岛逻辑的证据复用 | [search.rs](../../crates/rustify-docs/src/search.rs) 的索引字段不含 description；[M7](DOCS-SSG-P4.records/M7.md) 保留历史范围 | 可以复用，须先修 F-01；本地字节不能冒充 CI 字节 |
| 线上完整核验 | [verifier](../../scripts/docs-pages-verify.py) L54–91、L135–159、L263–264 核对候选、同 run 归档、manifest 和线上文件 | V-7 能核验发布身份；恢复时针对 C1 与实际回退 run 代入参数 |

12 条计划描述的句末、长度、普通标量禁用字符规则已用只读检查核对，均符合 §5.1。增加 frontmatter 也可能自然更新这些指南页的“最后更新”日期（[history.rs](../../crates/rustify-docs/src/history.rs) L18–31、L47–58）；仍在允许变化的同一批 HTML 内，V-4 不应将正常日期变化误作无关正文修改。

### 3.3 脚本与浏览器实际执行

| 检查 | 本轮结果与限制 |
| --- | --- |
| `check_paths.sh <计划> <仓根>` | exit 0，检查 23 个路径 token、提示 6 个缺失；逐项判断为计划新增记录/产物目录及条件兼容的 `docs/project-status/protocol.md`，不报现状复用缺陷 |
| `check_progress.sh <计划> <仓根>` | exit 0；0/3、3 行里程碑、0 行完成记录；ERROR 0、WARN 1。警告来自已有 dirty 工作树，相关产品路径无实施痕迹，未伪填进展 |
| `check_plan.py <计划>` | exit 0；ERROR 0、WARN 4，均为上游 WORK 需求 ID 的人工复核提示，已追到 PRD 与 D-01 来源 |
| TAR 身份与差异 | 只读检验两份收据、归档哈希、成员路径和成员 SHA，结果见 F-01 |
| 浏览器 | 主 agent 使用 `kimi-webbridge`，当前 C1 的真实页面；具体范围如下。未运行未来 C2 的 Playwright、安装、构建或发布检查 |

本期是 **DOM 文档内容和元数据**，不新增布局、组件、状态交互或 GPU 能力。按仓库规则加载了 `impeccable` 的现有产品、设计和 Read 场景上下文，只审阅读与验收方案，没有重设计界面。

本轮真实浏览器只读范围：

- 打开[中文首页](https://rockie.github.io/rustify-ui/zh/)，点击“快速开始”进入中文指南；标题与正文可读。[现场截图](DOCS-SSG-P4-gaps.review-assets/c1-zh-quickstart.jpg)。
- 通过页面同源读取 manifest 与 llms.txt：CLI `0.1.31`、build `9e8ea80d57750509`，manifest 列 96 文件（加 manifest 自身共 97）。Quick start 与 Architecture 的索引描述仍是原首段，确认 G-02 的现状；没有把未来描述记为已生效。
- 打开 [C1 GitHub 中文 quickstart](https://github.com/rockie/rustify-ui/blob/1146b7d8febdf53ca1bc1a4c01946604e7625a95/docs/zh/quickstart.md)，实见 YAML 表格、H1、正文与 16 个代码块。[frontmatter 实拍](DOCS-SSG-P4-gaps.review-assets/c1-github-frontmatter.jpg)。GitHub 的[官方说明](https://github.blog/news-insights/product-news/viewing-yaml-metadata-in-your-documents/)也支持这种呈现，但 C2 三份指南仍须按 V-6 实看。

证据等级是**现有 C1 的浏览器抽查 + 未来 C2 的文档推演**。本轮未采集完整 console/失败请求报告，未重做原生矩阵、手机、VoiceOver 或独立 UI 验收；不将上述抽查写成 C2 验收通过。

## 4. 需求映射与实施编排

| 需求 / 约束 | 设计 → 任务 → 可观察验收 | 核对结果 |
| --- | --- | --- |
| G-02 / D-01 / WORK-F12、F16/AC-4 | §5.1 → 12 指南和源/产物断言 → M1/M2/M3，V-1、V-5、V-8 | 描述与 en/zh 回退映射完整；meta 有既有同源链 |
| G-01 / N3/AC-4、AC-6 / N5/AC-4 | §5.2 → 两处结论文档 → M1、V-3 | 与 D17/D18 的边界一致，不宣称手机通过 |
| C-1 / D1 / D5 | §5.3 → 内容提交、C2 配对、安装与归档比较 → M2、V-4 | 比较输入断链 F-01，其他配对与候选消费链成立 |
| C-2 / D16 / 公开说明 | §5.4、§8 → 授权、发布、线上/安装核验、说明 → M3、V-7–V-9 | 缺说明公开推送与读回，见 F-02 |
| C-3 | mbx、单个 Playwright project、共享 main → §9/§10 | 编排符合本仓工具与资源约束 |
| C-4 / A-1 | frontmatter → GitHub 三份源页实看 → M3、V-6 | 判据明确；当前类比证据不替代 C2 |
| N1/AC-3、N3/AC-2–3、N7 | §7 → gzip 增量、完整预检、线上读者任务 → V-4、V-7、V-8 | 有检测与失败去向，复用以实际未变边界为限 |

建议保留三里程碑，不因修订另增阶段：

| 波次 | 前置与解锁证据 | 可并行工作 / 写入负责人 | 运行资源与汇合门 |
| --- | --- | --- | --- |
| M1 | D-01 与文字规则已定；新增断言先红 | 按原计划由一名实施者写指南、测试和结论同步；规模小，委派收益有限 | Rust 检查与线上红灯顺序执行；V-1–V-3 + V-5 红灯，形成内容提交 |
| M2 | M1 内容提交及 V-1–V-3；先修 F-01 | 版本、根锁、配对文档同一人负责；固定 C2 后冻结受测内容 | mbx、构建与 Playwright 串行，只跑 `official-subpath`；V-4/V-5 绿后形成可审阅候选 |
| M3 | V-4/V-5 绿 + 覆盖本次范围的发布授权 | 主实施者独占 main、dispatch、回退、说明发布；先修 F-02 | V-6 在 dispatch 前；V-7–V-9 后提交并推送说明、公共读回，再完成最终回写 |

依赖边：`红灯 → 内容与结论同步 → V-1–V-3 → 内容提交 → C2 → 实装/同环境比较/官方回归 → 授权 → 推送 → GitHub 预览 → dispatch → 同 run 核验 → 线上回归 → 公共安装 → 说明提交与推送/公共读回 → M3 完成`。失败分支继续使用固定 C1 与该回退 run 核验。

没有建议额外并行实施：C2 依赖已提交的 M1 内容，M3 依赖合格候选；版本/锁/main、构建和浏览器是共享资源。独立的文字与测试可以先准备，但不需要为此拆任务或新开环境。M2 的官方回归服务于发布前候选资格门，保留有理由；不扩大成原 P4 全量回归。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked**，具体阻塞为 M2/V-4 的比较基线与 M3 公开说明出口；原计划自报 `Ready`，本轮不修改其状态或正文。M1 的内容与结论同步没有这两项前置阻塞。

先修 F-01，确定本机与 CI 证据边界；再修 F-02，将发布后说明的推送、授权范围和公共读回写入 §8、检查表与 M3 出口。两项均是局部修订，不需要更换 frontmatter 方案、推进新的框架 R、改生成器或重做旧设备检查。

解除条件：V-4 明确引用可取回的本地 C1 基线并可按同环境比较；公开说明只有推送和公共读回成功才计完成；相应映射与退出条件一致。修订复核后可按证据重新判定 Ready。计划就绪不代表 C2 已实现、部署或得到发布授权。

下一步：

```text
$apply-doc-review docs/plan/DOCS-SSG-P4-gaps.review.md
```

## 修订处置与复核

- 来源：本报告原始版本 SHA-256 `66e80e7e6c336fb09b75aafcb04a369f68b6dcbaca9f38a054e579f262d6f5d9`（首次完整评审）；问题 ID F-01、F-02 保持不变。
- 目标：[DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md)；修前即所审版本 `741694ad3ab36d7118b5b6cab04da7fa817eb7f830505644185e74c13418e0e9`，修后 `78683c30844190137ab13a2af679b71dfd619b364cb0a6508605bfc0f44b7a6d`。
- 复核日期与代码基线：2026-10-20 14:51（Asia/Shanghai）；`main` / `4894d4ccc7c462a344489748553615a352031192` / dirty。未提交项仍只在 `.xgent-ai/` 与 `docs/plan/`，产品源码、指南、README、作者参考、脚本与工作流和 HEAD 一致。
- 本次范围：全部 findings（F-01、F-02），以及复核 F-01 时在同一 V-4 行发现的关联遗漏。§2 的待核实项仍是计划内的未来验证门，未改。

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果 / 后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。亲读两份收据：qualified 收据是 CI run 39219151112、build `9e8ea80d57750509`、同 run `artifact.tar`；本机收据是 build `7e1e5c9e7e7392df`、`c1-1146b7d8-site.tar.gz`，该归档仍在，SHA-256 `20101d9a…` 与收据一致。用 Python `tarfile` 只读复现：各 97 文件，91 个共有路径中 49 个不同（en/zh 各 24 个组件页 + manifest），双方各有 6 个岛资源路径不同 | 已修订并复核 | §0.2 D5 写明本机 C2 对同环境本机 C1；§1.1 新增两类 C1 归档的事实（含两份收据链接和 build 身份）；§9 V-4 的基线改为本机收据所记归档，先核哈希，归档缺失、哈希不符或工具链变化时同环境重建 C1，CI 归档只用于线上证据；§12 改为两份收据的链接。V-7 不变 | 文档判据通过：V-4 唯一基线指向本机 C1，与 §12 一致；CI 归档在全文只作线上证据。“本机 C2 对同环境 C1 只出现允许变化”属 M2 实施判据，未执行 |
| F-01 关联：V-4 允许变化集漏了 `sitemap.xml` | 成立，复核中新发现。`crates/rustify-docs/src/history.rs` 用最近一次触及文件的提交日期（UTC）生成“最后更新”与 sitemap `lastmod`。本机 C1 的 `sitemap.xml` 为 12 份 en 指南和 zh 快速开始写了 `lastmod`（最新 2026-10-15），zh 回退页沿用 en 日期且不在 sitemap 中。M1 内容提交会触及这 12 份指南，它们的日期必然变为提交当天。原 V-4 只允许“23 页 HTML、两份 llms.txt、manifest”变化，会把 sitemap 判为预期外变化，按 §8 停止发布。原报告 §3.2 提到了 HTML 日期，没有覆盖 sitemap | 已修订并复核 | V-4 允许集加入 sitemap 的 12 条指南 `lastmod`，并把指南页 HTML 内的允许变化限定为 meta/og/twitter 三处描述（等于源 frontmatter）与“最后更新”日期，一并吸收原报告 §3.2 的非阻塞补强；§0.3 ADR-1 正面后果、§1.1、§7 WORK-N1/AC-3 同步 | 抽查本机 C1 产物：quickstart 页的日期只在一个 `<time>` 中；HTML 不含候选 SHA，与原 P4 C0/C1 的 96 个读者文件逐字节相同的结论一致；quickstart、architecture 的原首段只出现在本页、zh 回退页、llms.txt 和搜索正文（正文不变）。实测留待 M2 |
| F-02 | 成立。§8 唯一的推送在 dispatch 前（原第 3 步），原第 6 步只做说明提交，M3 出口只要求“已提交”；`docs-pages.yml` L6–12 仅 `workflow_dispatch`；README L205 当前写 0.1.31，并给出 C1 的 `--rev` | 已修订并复核 | §0.1 C-2 写明授权范围含说明推送，落点加 V-10；§4 公共 Git 行；§5.4 增加推送、读回与“不重新 dispatch”；§8 第 2 步列出三类待授权动作，新增第 7、8 步，以及“说明推送失败或读回不符”失败行（不回退 Pages、不强推，M3 阻塞）；§9 新增 V-10；§10 M3 内容、出口与提交点；§11 授权风险行；页首本期交付 | 文档判据通过：M3 出口要求 V-10 通过且公共 `main` 包含说明提交。V-10 用公共 HTTPS 的 `git ls-remote` 取 main，按该 SHA 固定 raw 读回 README 与作者参考，核对安装 C2/0.1.32、回滚 C1/0.1.31、C2 发布 run；推送受阻时 M3 保持未完成。实施判据留待 M3 |

与原建议的差异：线上 manifest 只有 `cli_version`、`build_id` 等字段，没有候选 SHA，所以 V-10 用“`cli_version` 为 0.1.32、`build_id` 等于 V-7 那次 run 所记”核对 Pages 仍是 C2；候选身份仍由 V-7 的同 run 核验负责。最终回填类提交的推送不计入 M3 的公开说明出口，计划没有新增这项要求。

### 决策与未完项

- 没有新的用户决策。三项都是事实修正，或把既有约束 C-2 写明确；不改变范围、frontmatter 方案、R0、生成器或检查分层。
- §2 的待核实项（C2 内容与产物、A-1/V-6、发布授权、C2 线上与公共安装）仍按计划在 M1–M3 验证。本轮没有实施、构建、安装、浏览器测试或发布。
- 关联文档无需同步：PRD、覆盖核查、原 P4 计划与 M7 记录只读，未改。

### 修订后结论与验证

- 3 项已修订并复核（F-01、F-02、F-01 关联发现）；不采纳 0，待决 0。
- `check_paths.sh <计划> <仓根>`：exit 0，25 个路径 token，6 个 MISSING，都是计划新增（★）的记录/产物目录和条件兼容的 `docs/project-status/protocol.md`。修订过程中 §12 的简写路径曾多报 2 条，已改成可解析的相对链接。
- `check_progress.sh <计划> <仓根>`：exit 0；0/3，ERROR 0，WARN 1（既有 dirty 工作树；未填写进展）。
- `check_plan.py <计划>`：exit 0；ERROR 0，WARN 4，均为上游 WORK 需求 ID 的人工复核提示，与评审时相同。
- 只读事实检查：两份 C1 归档的哈希和成员差异，本机 C1 的 `sitemap.xml`、guide HTML、manifest 字段与 `llms.txt` 抽样，`history.rs`、README、工作流触发方式。
- 就绪判断：V-4 比较基线和 M3 公开说明出口这两个阻塞在文档层面已经解除，修订后判 **Ready**。原文状态行本来就是 Ready，恢复快照和 0/3 不变。Ready 不代表 C2 已实现、已发布或已获授权；M3 开始时仍需一次覆盖 §8 第 2 步全部动作的用户授权。
- 这是修订复核，不是新一轮独立评审。报告、计划与台账尚未提交或推送。
