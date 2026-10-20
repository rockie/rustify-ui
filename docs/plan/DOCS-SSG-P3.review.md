# DOCS-SSG-P3 · 开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P3 · 多语言路由、缺译回退与检查](DOCS-SSG-P3.md) | v1；SHA-256 `a64aa3f443a7a92b30fd50712125bf21be36334464de42b3799d2976de85d1a9` | 全文，M1–M5、V-1–V-10，以及直接关联的 PRD、P2 前置、P4 解锁关系 |

评审日期：2026-10-15 +08:00。评审 ID：`c34cd2c3-3731-433a-90c7-8bb70fdbc9e8`。本轮是首次完整计划评审；原计划、PRD 和产品代码保持只读。

## 1. 确认问题

确认 3 项 P2；无 P0、P1 或 P3。

### [P2] F-01 · 补齐产物语言前缀到 preview 的读取合同

- **定位**：[计划 §4，第 279 行](DOCS-SSG-P3.md#L279)、[§5.2，第 299–301 行](DOCS-SSG-P3.md#L299)；关联 WORK-F10/AC-4、M4、V-5。
- **问题**：计划要求 preview 用冻结的语言前缀和错误文件选择 404，但没有确定这份数据在产物中的位置、字段、写入方和读取方。`preview --dir` 可以只拿到 dist，不能依赖当前工程配置；既有 manifest 的文件列表也不能区分已配置语言的错误页和作者普通资源。实施者仍须自行决定这条产物接口。读取当前配置会使已构建产物受配置漂移影响；扫描任意 `*/404.html` 则可能把静态资源误认成语言声明。
- **证据**：[main.rs:744](../../crates/rustify-docs-cli/src/main.rs#L744) 的 `--dir` 分支不读工程配置，无 `--dir` 时也只借配置取得输出目录；[main.rs:781](../../crates/rustify-docs-cli/src/main.rs#L781) 与 [841](../../crates/rustify-docs-cli/src/main.rs#L841) 随后读取产物 manifest、根 404 并构造 preview Site。[publish.rs:36](../../crates/rustify-docs/src/publish.rs#L36) 的 manifest 只有 schema、mode、base、CLI 版本、build_id 和无类型文件列表；[generation.rs:858](../../crates/rustify-docs/src/generation.rs#L858) 会按相对目录发布普通 assets。计划允许 `lang` 和 `prefix` 分别配置，也没有在新增 template 中定义供 preview 共用的前缀到错误文件映射。
- **最小修订**：在 §2/§5.2 选定一份随产物冻结的 `prefix → 404 文件` 数据，明确字段、校验、写入和读取落点。可以复用根 404 的新增元数据，或扩展现有 manifest，不要求新建站点文件。明确 preview 仅消费产物数据，dev 使用 last-good 同代数据；旧单语言产物缺少清单时回到根 404。将实际改动落点纳入 M4。
- **复核**：改稿后可追通唯一写入方和所有读取方；实现后把 dist 移到没有源码和配置的目录，禁用脚本请求语言前缀下的深层不存在地址，断言对应语言和 HTTP 404。修改工程配置但不重建不得改变旧 dist 的选择；普通 `foo/404.html` 不得创建虚假语言；旧单语言 dist 继续可预览。
- **置信度**：高，已全文核对计划并追到产物与 preview 入口。**阻塞范围**：M4 的语言 404 接缝；M1–M3 可独立推进。

### [P2] F-02 · 为多语言水合与 404 岛安排真实浏览器载体

- **定位**：[计划 §1.2，第 146 行](DOCS-SSG-P3.md#L146)、[V-4/V-7，第 361–364 行](DOCS-SSG-P3.md#L361)、[§9.2，第 371 行](DOCS-SSG-P3.md#L371)；关联 M3–M5、WORK-F10、WORK-N3。
- **问题**：V-7 承诺验证岛语言，§5.2 又改变单根错误页的模板激活与岛扫描顺序；但三个新增托管项目明确使用内容站 fixture，真实多语言 Rust 消费者只安排在 CLI 测试的临时副本中。`dev-i18n` 只声明归属 docs-dev，没有指定真实 Rust 消费者和多语言水合、404 slot 岛的测试路径。照当前分工，内容站项目可以全部通过，却没有执行这些新增浏览器行为。
- **证据**：[内容 fixture 配置](../../tests/docs-ssg/fixtures/content/rustify-docs.toml) 没有 `[rust]`；[serve.sh:40](../../tests/docs-ssg/serve.sh#L40) 直接构建内容站，而具有 `[rust]` 的 [components 配置](../../tests/docs-ssg/fixtures/components/rustify-docs.toml#L14) 经 [serve.sh:46](../../tests/docs-ssg/serve.sh#L46) 的独立分支构建。浏览器实际执行的是 [islands.js:33](../../crates/rustify-docs/assets/islands.js#L33) 的 `parsed → boot → docs_hydrate → ready` 链路；V-4 的 CLI HTML/metadata 检查覆盖不到该链路和模板激活后的活动 DOM。
- **最小修订**：保留内容站检查，并明确复用 components 临时副本，给响应头 CSP 的 preview/subpath 和单根错误页的 meta host 提供多语言 Rust 浏览器入口。`dev-i18n` 明确复用哪一载体。可复用现有项目/serve 分支，不增加产品模板或组件体系；补齐入口、配对方式、执行命令和断言。
- **复核**：改稿后能逐项指出中英注册页、缺译页、第三语言 SDK 英文回退和 404 共享 slot 岛所在的真实页面；实现后从 SSR 进入 ready 并实际点击。单根错误页选中语言后仅活动树水合，slot/key/seed 不重复，目标环境正确；禁脚本仍可读，并检查 console、CSP 和失败请求。
- **置信度**：高，计划的 fixture 分工与现有两类构建路径明确。**阻塞范围**：M5 对这些行为的浏览器验收；不阻塞 M1–M4 的独立本地开发。

### [P2] F-03 · 将 V-9 工具链验收绑定到真实交付提交 C

- **定位**：[计划 §3.2，第 268–270 行](DOCS-SSG-P3.md#L268)、[§9.1，第 354/366 行](DOCS-SSG-P3.md#L354)；关联 C-2、WORK-N4、M5、V-9。
- **问题**：计划要求固定 C 实装，并声明 `RUSTIFY_DOCS_BIN` 绑定 C，但列出的 `mbx xtask docs-verify --group toolchain` 不读取该变量。无 `--git/--rev` 时，它会从当前工作树另造一个合成提交，再安装该提交的 CLI。受限 PATH、离线及新增语言 fixture 的通过结果因此属于另一个来源，不能直接记为实际 C 的验收，工作树含后续文档或并发改动时尤其明显。
- **证据**：[xtask/src/docs.rs:1352](../../xtask/src/docs.rs#L1352) 的默认分支调用 `candidate()`；[1111–1148](../../xtask/src/docs.rs#L1111) 将 tracked/untracked 工作树写成新提交；[1372–1390](../../xtask/src/docs.rs#L1372) 自行安装并选用该二进制；[1425](../../xtask/src/docs.rs#L1425) 将它交给 walkthrough。此路径没有读取 `RUSTIFY_DOCS_BIN`。
- **最小修订**：V-9 使用现有参数显式传入本地 Git URL 与 C 的完整 SHA，即 `mbx xtask docs-verify --group toolchain --git <本地Git URL> --rev <C完整SHA>`；新增语言检查沿该组内部安装的二进制执行。其他开发态包测试保持现有入口。
- **复核**：改稿后工具链命令来源唯一；实现收口记录中的 `CLI source`、`--build-info`、`cargo install --list` 均对应 C，模板框架对应 R，受限工具链和离线检查使用该二进制。
- **置信度**：高，已核到默认来源、安装及调用链。**阻塞范围**：M5 固定来源验收；M1–M4 可独立推进。

## 2. 待核实与证据边界

| 项目 | 缺少的证据与影响 | 提供方与最晚确认点 |
| --- | --- | --- |
| U-01 · 真实浏览器、手机与辅助技术资源 | 计划已登记尚缺的稳定浏览器渠道、两类手机和 VoiceOver/Safari 实测；本轮未重新核验资源可用性。缺证时 V-10、完整矩阵及 M5 完成不能记通过 | 维护者提供现有设备或远程入口，Agent 操作；M5 对应验收前。按 D16 不阻塞独立本地开发 |

P2 的历史 5/7、设备、正式性能和公开发布欠证仍属于原计划/P4；本轮只核对 P3 实际依赖，没有关闭它们。公开 Git、Pages 与线上回滚不在本轮审查或执行范围内。没有发现必须新增外部服务或 GPU 路径的技术前提。

## 3. 覆盖、基线与实际验证

### 3.1 基线

- 代码：`main` / `3803148929ebb2f69d7549c9f054ff81eb9a6d20`；dirty，事实源为当前工作树。开始时未提交项是 P3/P4 新计划和已有 SDLC 登记；相关产品源码及 PRD 与 HEAD 一致。
- PRD：[PRD-DOCS-SSG](../PRD-DOCS-SSG.md)，SHA-256 `22f4fac99b06a23cda3c34f7ee0c66c11369ccd60a34d71fecee2617aa4c16c1`；核对 P3 需求、跨期终验、DEP-7/9、D15/D16 和 Q3/Q6。
- 设计：[DESIGN.docs-ssg.md](../../DESIGN.docs-ssg.md)，SHA-256 `ce9afe161f4d33d72107942cb0f66976cdf3683c9b3fe3b40c94cf39f26804b1`；结合 PRODUCT、DESIGN 和已有 Read surface brief。呈现路径为 DOM 构建期 HTML 与既有 DOM 岛，符合本期范围。
- 收口复核：原计划 hash、HEAD 和相关源码未变。本报告和台账登记未提交、未推送；原计划尚为未跟踪文件，跨机器复现须先同步该版本。

### 3.2 四个审查视角

| 视角 | 实际负责人及状态 | 覆盖结论与缺口 |
| --- | --- | --- |
| facts | 独立 subagent `/root/facts`，完成 | 核实单份导航、EN/ZH 文案、helper 环境、SDK locale、缓存、根 404、SEO、R/C 及 P2 前置；无独立事实 finding |
| architecture | 独立 subagent `/root/architecture`，完成 | 统一语言图、不可变 Catalog、按 ui_lang 批次、parsed 屏障及 publish 事务可沿既有边界落地；确认 F-01 |
| schedule | 独立 subagent `/root/schedule`，完成 | 需求映射、局部门/集中终验、依赖、部分解锁与 0/5 自洽；确认 F-03，提出 F-02 供交付视角交叉核对 |
| delivery | 首批结束后复用 `/root/schedule` 单独执行该视角，完成 | 从安装、配置到阅读、404、岛交互、恢复及回滚推演；确认 F-02；UI 证据为文档推演 |
| 综合裁定 | 主 agent，完成 | 逐项回读计划和原始代码，复核三项证据与影响、去重、写入并回读完整报告 |

总计主 agent 加 3 个 subagent；首批并行事实、架构、编排，随后复用空闲 reviewer 覆盖交付。没有多个 agent 同写报告或共享浏览器会话。

关键现状已核对：[content.rs:904](../../crates/rustify-docs/src/content.rs#L904) 当前只有一份 nav/top/social；[labels.rs:228](../../crates/rustify-docs/src/labels.rs#L228) 的第三语言整体回英文；[rendering.rs:139](../../crates/rustify-docs-cli/src/rendering.rs#L139) 当前按站点单一语言构造环境；[registry.rs:316](../../crates/rustify-docs/src/site/registry.rs#L316) 当前拒绝 SDK 未支持语言。计划已明确这些是待修改的缺口，不能据此判定未来设计错误。根 404 在 parsed 前激活模板的路线与既有 [mode.js:93](../../crates/rustify-docs/assets/mode.js#L93)、[islands.js:33](../../crates/rustify-docs/assets/islands.js#L33) 接入点相容。

### 3.3 工具与 UI 证据

| 检查 | 实际结果 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P3.md <仓库根>` | 检查 28 项、7 项 MISSING。人工核对：4 项为明确新增的 i18n 文件/目录，2 项为源路径示例，1 项为旧台账兼容路径；未发现虚构复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P3.md <仓库根>` | ERROR 0、WARN 1；0/5、五行里程碑、空完成记录一致。警告由已有 dirty 状态及未跟踪初稿触发，无产品实施痕迹，不据此认定漏写进度 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P3.md` | ERROR 0、WARN 8；八个 WORK 自定义需求号已沿 PRD 人工核对 |
| impeccable context | 已运行 `.agents/skills/impeccable/scripts/impeccable context --target crates/rustify-docs/src/shell.rs`，结合现有 Read 设计约束检查语言入口、缺译提示、焦点、窄屏、主题和验收安排；未改设计 |
| 产品测试、构建与安装 | 本轮未运行；未来 P3 测试/产物尚未实现，没有用历史测试替代本轮功能验证 |
| 真实浏览器 | **本轮未在浏览器中验证**。本期多语言页面尚未实现；本轮没有视觉、键盘、水合、真机或朗读通过结论。UI 判断仅限文档推演，F-02 要补的是后续实现验收的载体 |

脚本结果只证明其结构检查范围，不证明行为、功能完成或验收通过。本轮关键结论来自仓内代码与合同，没有依赖未核实的外部版本能力或时效性限制。

## 4. 需求映射与实施编排

### 4.1 映射结论

| 范围 | 已有设计与验收落点 | 本轮结论 |
| --- | --- | --- |
| WORK-F10/AC-1–3 | §2、§5.1/5.2、§6；V-1–V-4、V-6–V-9 | 配置、真实译文、回退、文案与静态切换闭环；岛的浏览器覆盖须按 F-02 补齐 |
| WORK-F10/AC-4 | §5.2；V-5、V-7、V-10 | 两类托管的目标明确；产物到 preview 合同缺 F-01，含岛验收缺 F-02 |
| WORK-F11/AC-1–4 | §5.3；V-5、V-7/V-8 | 真实版本互指、回退 canonical/noindex、sitemap 过滤和无 URL 语义明确；计划没有把 P1 AC 从终验中移除 |
| WORK-F15/AC-1–5 | §5.4；V-6、V-8/V-9 | 3 页+2 文案、位置/建议、JSON、deny-warnings 与前期错误检查有落点；离线固定来源证据须修 F-03 |
| WORK-N3/N5 | §6/§7、§9.2；V-7/V-10 | 无脚本、CSP、主题、窄屏/缩放、焦点与语言标注已列；设备缺证不冒充通过 |
| WORK-N4/N6/N7 | §2.2、§3、§5、§8；V-3、V-8/V-9 | 工具链分图、收据/指纹、整代产物与输入资源隔离有机制；固定交付验收修 F-03 |
| C-1–C-5 与范围排除 | §4、§11；V-2/V-4/V-7–V-10 | P2 实际前置、R/C、DOM、SDK 第三语言和资源门有去向；P4 搜索/AI/官方中文/公开发布未被纳入 P3 |

除三项已列问题外，没有确认新的需求断链；集中验收和只保留一份完整映射合理，无需新增里程碑或重复任务台账。

### 4.2 依赖边与波次

| 波次/依赖边 | 解锁证据 | 可并行工作与共享负责人 | 汇合门/资源 |
| --- | --- | --- | --- |
| P2 本地能力与本人验收 → M1 | 原计划、本人验收记录及 D16；不等历史 5/7 关闭 | config/labels 与注册语法/宏可按文件分工；主实施者拥有公共类型与 content 初始图 | V-1/V-2；不新增服务或容器 |
| M1 → M2 → M3 | PageLocale/LocaleView + V-2；完整投影图 + V-3 | 主实施者顺序集成 content/rendering/shell；契约稳定后可并行独立测试、作者说明 | V-3/V-4；同文件单写 |
| M3 → M4 | 语言 shell、批次与目录收据 | 主实施者先补 F-01 并拥有 generation/404 数据接缝；独立诊断测试可分工 | V-5/V-6；回收全部子任务后集成 |
| V-1–V-6 → M5；R → C → 固定 C 安装 | 本地实现与 fixture 就绪；真实 R/C 及锁配对 | 主实施者串行处理版本、锁、安装与来源；按 F-02 补真实浏览器载体、按 F-03 绑定工具链来源 | V-7–V-9；一次一个 Playwright project，独占输出目录、端口和浏览器会话 |
| 维护者真实资源 → V-10 → M5 完整终验 | 实际渠道/设备与版本可用 | Agent 操作；模拟与真机分别登记 | 缺资源保留未验证，不把 M5 记完成 |
| 已记录 V-7–V-9 本地成果 → P4 对应开发 | P3 §11 的本地记录和固定配对；含上述修订后的实际证据 | 依赖只细到所需成果，不要求 M5 整行完成 | V-10/历史设备补证继续单独跟踪 |

共享图、渲染和生成流水线连续改动，主要瓶颈是契约与顺序集成；不能仅按目录把 M2–M4 同时开写。现有一阶段集中终验安排成立，没有依据估算工期或声称并行节省比例。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked。** 统一语言图、缓存、批量 helper 和静态降级的技术路线可保留；但 F-01 的产物/preview 接口尚未冻结，F-02/F-03 又使收口证据不能覆盖计划承诺的行为与交付来源，因此当前完整计划不宜按原文 Ready 直接交接。这一建议针对 M4 接缝和 M5 完整交付，不否定 M1–M3 的已确定本地前置，也不以设备欠证或 P2 历史未完成作为新的本地开发阻塞。

建议修订顺序：先补 F-01 的权威数据与兼容合同，再补 F-02 的真实浏览器载体和断言，最后修正 F-03 的固定 C 命令并同步 M4/M5 退出条件。改稿后可追通前缀数据、浏览器中的真实多语消费者及验收二进制身份，即可复核文档就绪；实际 V-1–V-10 仍在实施时执行，不能把计划评审当作功能验收。

下一步：

```text
$apply-doc-review docs/plan/DOCS-SSG-P3.review.md
```

本轮仅建议下一步，不自动修订计划或启动实施。SDLC 将独立保留本轮评审与待处置事实，原计划自报 Ready、实施 0/5 和既有历史记录不改。

## 修订处置与复核

- 来源：本报告追加处置前 SHA-256 `f4c529d3937501f8164471c996949d9afccd9250ded76c1f96d05866ae15ec5b`；沿用评审 ID `c34cd2c3-3731-433a-90c7-8bb70fdbc9e8` 和原问题编号。上文原问题、所审基线及“需要修改 / 建议 Blocked”结论完整保留。
- 目标：[DOCS-SSG-P3](DOCS-SSG-P3.md)，修前 v1 / SHA-256 `a64aa3f443a7a92b30fd50712125bf21be36334464de42b3799d2976de85d1a9`，与所审版本一致；修后 v2 / SHA-256 `7fe7661c44ab0160151315ddf7d97a79fd307b4e31c86d41319458cae0003528`。
- 复核日期与代码基线：2026-10-15 +08:00；`main` / `3803148929ebb2f69d7549c9f054ff81eb9a6d20` / dirty。开始时已有 P3/P4 计划、P3 评审及 SDLC 未提交项；本次产品源码、PRD 与设计文档未修改。相关产品源码与 HEAD 一致，核实依据来自当前工作树。
- 本次范围：全部 F-01–F-03、U-01，以及受修订影响的需求映射/依赖/就绪结论；默认修订模式。无此前处置，本次是同轮文档修订复核，未重跑完整独立评审。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。亲读 [publish.rs](../../crates/rustify-docs/src/publish.rs) 的 Manifest/add_manifest、[main.rs](../../crates/rustify-docs-cli/src/main.rs) 的 build/preview、[preview.rs](../../crates/rustify-docs-cli/src/preview.rs) 的 Site，以及 [dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 的 Good/needs_reload：现有文件列表无语言声明，--dir 不读取配置，dev 的错误文件也固定为根 404 | 已修订并复核 | §0.2/D5、§1.1/1.2、§2 模块表和新增 §2.4、§3.1、§4、§5.2、§8、V-5、M4、§11；冻结可选 manifest.locale_not_found，生成方唯一，preview 在共享锁下读产物，dev 同代持有映射与文件 | **已解决（文档层）**。可追通 Generation → build/add_manifest → preview::Site、Generation → Good → HTTP/SSE，以及同源根 404 template；旧单语言空表、非法映射/文件、配置漂移、普通资源和失败代语义均已定义。V-5 安排移出源码/配置后的深层 HTTP 404、兼容及 last-good 检查，实际结果待实施 |
| F-02 | 成立。亲读 [content 配置](../../tests/docs-ssg/fixtures/content/rustify-docs.toml)、[components 配置](../../tests/docs-ssg/fixtures/components/rustify-docs.toml)、[serve.sh](../../tests/docs-ssg/serve.sh)、[Playwright 配置](../../playwright.docs.config.ts)、[dev-components.spec.ts](../../tests/docs-ssg/dev-components.spec.ts)、[islands.js](../../crates/rustify-docs/assets/islands.js) 和 [mode.js](../../crates/rustify-docs/assets/mode.js)：内容站无 Rust，CLI HTML 检查不能执行 parsed/boot/hydrate。原 [custom.spec.ts](../../tests/docs-ssg/custom.spec.ts) 对 404 保留无岛断言，故使用独立临时语言载体 | 已修订并复核 | §1.2、V-7、§9.2、M5、§11；保留内容站三项目，新增真实 components 语言叠加件及三个专用项目，dev-i18n 复用同一叠加件；列出配对、端口、七条串行命令与页面断言 | **已解决（文档层）**。可逐项定位 en/zh 注册页、fr 缺译及 SDK 英文回退、zh 静态回退和 zh/fr 404 的共享 slot/NotFound 岛；SSR→ready→点击、只水合活动树、slot/key/seed、CSP/console/无脚本均有真实浏览器入口。未创建这些测试或执行浏览器验收 |
| F-03 | 成立。亲读 [xtask/src/docs.rs](../../xtask/src/docs.rs) 的 verify/toolchain/candidate/plain_cargo 及 walkthrough 调用：默认路径合成工作树提交，toolchain 自行安装并使用内部 binary，不以 RUSTIFY_DOCS_BIN 决定来源 | 已修订并复核 | §3.2、§9.1 引导、V-9、M5、§11；本地安装变量定义清楚，toolchain 命令显式传 --git 与 C 完整 SHA；新增语言 walkthrough 沿内部安装二进制，开发态 features/artifacts 另记来源 | **已解决（文档层）**。两处 toolchain 命令一致绑定 DOCS_LOCAL_GIT/DOCS_C，CLI source、build-info、install --list 共同核对 C，模板依赖核对 R。实际安装、受限 PATH 和离线检查待 M5 执行 |
| U-01 / §2 证据边界 | 保留。已回读计划 C-5、§4/§9.2/§11 与 PRD D16/Q3；本次未重新核验稳定浏览器、两手机或 VoiceOver 资源的实际可用性 | 待核实（验收资源，非新增文档阻塞） | 保留 V-10 和原资源责任/最晚确认点，不改成通过，不增加实施完成数 | **待核实**。维护者提供实际设备或远程入口后由 Agent 执行；未齐时完整矩阵与 M5 不记完成，按 D16 可继续独立本地开发 |

### 决策与未完项

- 沿用既定产品目标，无新增待用户拍板项。错误映射放入既有 manifest 的可选字段，利用 preview 已有产物读取边界；根 404 的模板属性复用同一生成表，浏览器无需另取 manifest。没有新增站点文件、运行服务或 GPU 路径。
- Rust 浏览器载体复用现有 components 的 Cargo 包、真实 SSR/hydrate 图和演示，仅在临时副本叠加测试语言数据；三个专用项目隔离原单语言断言。4207–4209 为本计划新增分配，与现有 docs 端口及 P4 已规划的 4204–4206 不重叠；未实际启动服务验证端口可用性。
- 原计划 0/5、未开始和空完成记录保持。V-1–V-10 是后续实施检查；U-01、P2 历史 5/7、设备/正式性能和公开发布欠证未关闭。P4 的必要本地前置须以实际 P3 成果及记录解除。
- 关联但未授权修改：[P4 计划](DOCS-SSG-P4.md) §9.1/V-9 也保留不带 --git/--rev 的 toolchain 命令；P4 自身修订时应绑定其交付 C。本次未改 P4、PRD 或设计文档，也未实施计划中的作者/设计文档同步任务。

### 修订后结论与验证

F-01–F-03 共 **3 项已修订并复核**；无需修改 0、未采纳 0、待决策 0、未完成的文档修订项 0。U-01 仍为独立验收资源待核实项，不计入三项问题关闭数。修后计划判定 **Ready**：文档接缝与验收来源可执行，已知设备依赖继续约束完整终验；不是功能完成或新一轮独立评审通过。

| 本次实际检查 | 结果与人工处置 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P3.md <仓库根>` | 检查 30 项、MISSING 9 项：5 项为明确新增的两份 Rust i18n 测试、语言 fixture 目录及两份浏览器 spec；3 项为 guide/start.md、zh/guide/start.md 和 foo/404.html 示例；1 项为旧台账兼容路径。未发现错误复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P3.md <仓库根>` | ERROR 0、WARN 1；0/5、五行里程碑、空完成记录一致。警告来自既有 dirty 状态与未跟踪计划导致的陈旧提示；没有产品实施改动 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P3.md` | ERROR 0、WARN 8；八个 WORK 自定义 ID 均已沿 PRD 的对应需求与分期核对，未改换编号 |
| 文档回读与一致性检查 | 映射的唯一写入/读取链、页面/项目/命令、C/R 来源、M4/M5 退出门及就绪理由一致；本地链接目标均存在，无尾随空白；原报告历史正文与实施进度逐字保留，最终 diff 未见无关改动 |
| 产品测试、构建、安装与 UI | 本次均未运行，文档修订不执行计划。**未在浏览器中验证**；多语言产物与专项载体尚待实现，不能据此宣称水合、视觉、真机或朗读通过 |

下一步仅建议开始实施，不自动启动：

```text
/goal 按 docs/plan/DOCS-SSG-P3.md 开发
```
