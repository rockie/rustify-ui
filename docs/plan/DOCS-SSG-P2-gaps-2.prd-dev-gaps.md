# DOCS-SSG-P2-gaps-2 · PRD 交付差异复审（P2 链 R3）

## 1. 结论与范围

**G-06 已解决，D-05 要求的本地固定版本补齐已交付；本轮新增 A 0、B 0、C 0，待决 0，无需新的 gaps plan。** 本增量待核实 0；原 P2 的 U-01–U-03 共 3 项仍待核实，K-01/K-02 继续由原计划承接。原 P2 仍为 **5/7**，本结论不关闭整期或整份 PRD。

| 项目 | 本轮锚点 |
| --- | --- |
| 输入 / 需求来源 | [gaps-2 计划](DOCS-SSG-P2-gaps-2.md)，Ready、实施 2/2；[PRD](../PRD-DOCS-SSG.md) P2；补齐[原计划](DOCS-SSG-P2.md)与[首个 gaps](DOCS-SSG-P2-gaps.md)的诊断路径 |
| 上轮报告 / 版本 | [P2 差异复审 R2](DOCS-SSG-P2.prd-dev-gaps.r2.md)，SHA-256 `c1212c1b4088670f27b343ee5712d76202c12c980480c581c22ad18ac80d2085`；对象 `c8480584-6578-4587-972a-8294319685fc` |
| 类型 / 本轮范围 | **修复效果复审**：G-06/D-05 全部关闭条件；G-01 直接诊断回归；其他 G-ID、U-ID 与 K-ID 保留逐项去向。不是全 PRD 重审 |
| 前后版本 | 上轮 C′ `79a77825` / R′ `745ba6b7` → 本轮 C‴ `b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d`（0.1.19）/ R‴ `6942ec0d23812ecda61492d9db5c0046f481edfc`；中间 C″ 0.1.18 的 PG-01 缺口与修复历史保留 |
| 已有处置 | [M1](DOCS-SSG-P2-gaps-2.records/M1.md)、[M2](DOCS-SSG-P2-gaps-2.records/M2.md)；[代码评审 R1 / PG-01](DOCS-SSG-P2-gaps-2.code-review.md#4-计划符合性)、[代码复审 R2](DOCS-SSG-P2-gaps-2.code-review.r2.md) |
| 本轮身份 / 基线 | 对象 `e9ce88ff-9dee-4eee-afc5-eeb044bb2b7e`；2026-10-15 +08:00；`main / 789003cc5ad58bd734f5a8c483f8fec475c2afe9 / dirty`；本机当前工作树参与调查 |

按 gaps 计划的来源链启用复审，原 PRD、计划、完成记录、代码与前轮报告保持只读。呈现路径为 Rust 诊断工具链与 dev DOM 面板，不涉及 GPU 或 Portal。实际验证使用既有安装的 C‴，没有重新安装、推送、部署或验证公开可获取性。

### 上轮问题复审

本表沿用同一 P2 链的原始编号；前五项最初定义在[首轮报告](DOCS-SSG-P2.prd-dev-gaps.md)，G-06 定义在 R2。`本轮未复核`不改写既有关闭事实。

| 来源 / ID | 上轮结论与已有处置 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- |
| R2 / G-06 | C，D-05 选择现在补齐；gaps-2 M1/M2 与 PG-01 修复后形成 R‴/C‴ | **已解决** | T 原反例默认值的 unknown-property、invalid_props；普通路径、完整 anchor、YAML 键、空语言与 lang 后处理；真实面板精确文字/code、位置、last-good、焦点和恢复均通过。S/H 固定配对、安装、new 工程与全部生产方适用性见 §2 | 无；原分类与 D-05 保留 |
| R2 / G-01 | A，上一轮已解决：嵌入诊断有候选与建议 | **已解决**（直接回归） | T 两个 `gaps embedded diagnostics` 用例重放注册组件/演示与无 Rust 内置分支，候选、声明、位置、终端与面板一致；新 writer 没有使建议消失 | 无 |
| R2 / G-02 | B，Firefox P2 例外已同步 PRD | **已解决**（文档核对） | S PRD 哈希与 R2 相同，WORK-N3 与 Q3 仍明确 P2 未验证、P4 恢复，其他设备不豁免 | Firefox 验证仍在 P4，不记通过 |
| R2 / G-03 | B，D15 本人验收取代五人限时研究，无剩余任务 | **本轮未复核** | 只确认 PRD D15 决定仍有效；没有重新开展本人 UAT，也未把自动化当用户确认 | 沿用 R2；本人 C4 结论不扩到本轮版本 |
| R2 / G-04 | B，D16 公开安装/部署/回滚留 P4 | **本轮未复核** | 只确认 PRD D16 与本地交付边界仍有效；本轮没有公开渠道证据 | 原 M7/V-15，P4 |
| R2 / G-05 及 N5/AC-1、AC-6 关联 | A，上一轮 reduce 与稳定状态 AA 已解决 | **本轮未复核** | 本轮没有重跑 reduce、axe 或官方站；诊断增量未改 `docs.css`，不据此扩为当前完整视觉验收 | 沿用 R2 的版本及范围 |
| R2 / U-01 | 最新稳定浏览器、实体设备、VoiceOver 与原生缩放缺证 | **待核实** | 本轮 Chromium 153 与尺寸仿真不补齐这些证据 | 原 M6/V-11 |
| R2 / U-02 | 正式 A/B 冷样本、演示类型移动端性能、new 五轮缺证 | **待核实** | 本轮未测性能，功能测试耗时不作预算证据 | 原 M6/V-12 |
| R2 / U-03 | 组件围栏真实 GitHub 文件预览缺证 | **待核实** | 本轮未访问该平台或发布新来源 | 原 M7/V-15，P4 |
| R2 / K-01、K-02 | 原 M6 未完成、M7 按 D16 后移 | **仍存在** | S 原计划哈希未变，恢复快照仍 5/7 | 继续原计划，不重复建计划 |

旧问题本轮复核解决 G-06、确认 G-01/G-02，G-03–G-05 明确未重新验收；没有确认的新增回归。PG-01 使用代码评审自己的来源身份，不重编号成新的 G-ID，详见下节。

## 2. 三类差异与 G-06 的关闭依据

### A · 明确遗漏（本轮新增 0）

无。本轮没有未被已有计划承接的确定修复事项。

### B · 合理变更（本轮新增 0）

无新增。既有 G-02–G-04 与决定轨迹继续保留，不重复计数。gaps-2 D1/D2/D4 已确定的逐字片段规则、普通作者值含反引号时整段等宽、外部原文边界仍按已批准范围核对。

### C · 需 Triage 的实现差异（本轮新增 0）

无新增、无待决。沿用上轮 **G-06 · C（D-05 已决）**，本轮结果为已解决：

- **PRD 期望**：WORK-F13「出错」与 AC-2 要求终端和浮层显示错误及修复建议；WORK-F15 的诊断格式与 AC-2 要求说明、建议和机器可读格式；WORK-N6/AC-2、AC-3 要求拒绝时保全旧产物、保留位置与建议。D-05 和 gaps-2 §5.1–§5.4 将本次边界确定为自有诊断及作者值逐字显示，不扩张到外部工具原文。
- **实际交付**：两个 Rust writer 对完整值编码，空串有独立围栏，含反引号时使用更长围栏；`literal` 只在普通作者值含反引号时编码。`lib.rs` 无条件导出 std-only 模块。面板匹配同长的最大围栏，只剥除合同约定的填充，仍通过文本节点/code/wbr 创建 DOM。终端与 JSON 直接输出同一 message/suggestion，JSON schema 仍为 1。
- **证据链**：gaps-2 M1 的生产方/读取/作者参考 → M2 的 C″ 配对 → R1 发现 PG-01 → M1/M2 修复为 R‴/C‴ → 代码 R2 → 本轮 CLI 与安装入口 DOM。已核对的源码包括 [writer](../../crates/rustify-docs/src/diagnostic_text.rs)、[syntax writer](../../crates/rustify-docs-syntax/src/lib.rs)、[embed 建议](../../crates/rustify-docs/src/embed.rs)、[lang 后处理](../../crates/rustify-docs/src/frontmatter.rs)、[report](../../crates/rustify-docs-cli/src/report.rs)、[面板](../../crates/rustify-docs/assets/dev.js)和[作者合同](../docs-ssg.md#rust-components-and-demos)。
- **原反例**：默认表达式 ``"a`b".to_string()`` 的反引号现在保留在一个完整 code 节点中，unknown-property 与 invalid_props 的其余名称也没有颠倒排版。原始 SSE、手写期望和实际 DOM 分列于[unknown-property](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser/t04/00-unknown-property-prose.json)，没有以被测解析器生成全部期望。
- **PG-01**：自有 lang 值中的 ` at line ` 不再截断消息。当前只移除与解析器实际 line/column 一致的末尾后缀。T CLI 四输入回归与真实 dev 的含/不含反引号两阶段通过；完整值、说明和 `3:7` 均保持。R1 的原判与归因不变。
- **生产方审计适用性**：本轮没有重新逐处审计全部模板。S 独立比较 55 个相关源码/资源/测试文件与 C‴，均一致；从 R″ 到当前的生产方/读取源码仅 `frontmatter.rs` 变化，已读其完整返回链。因此 H M1 分类审计与 R1 的完整生产方核查在其余路径继续适用，原 V-2 对 PG-01 的遗漏由修复证据单独覆盖。[输入审计](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/input-audit.json)列出真实文件集合。
- **交付身份**：S C‴ 提交恰好五文件；模板锁 193 条中仅六框架 source 更新为 R‴，其余 187 条不变；根锁仅 CLI 版本变化。T 既有安装入口前后 SHA-256 均为 `c51f563d608a31bd56c09ae2ef393cdb93558ba3fa17472c8c0833f1a10ac95d`，source=C‴、panic=unwind。H [同一 new 工程验收](../validation/docs-ssg-p2-gaps-2/apply-r1/author/acceptance.json)为六阶段/24 视图、两目标零 Makepad、锁/dist/last-good 保持与恢复；本轮未重新安装或重跑整套 new 脚本。
- **影响与处置**：作者不再从受控诊断中抄到丢字符的表达式。位置、严重级别、拒绝和恢复保持。G-06 的确定 follow-up 已由当前 gaps-2 完成，无剩余增量；D-05 无需再次决策。

外部 serde/YAML/rustc/I/O 原文仍按 D4 传递；其中自身成对反引号可能仍有歧义。这是原计划明确排除，不记为新遗漏或已实现的逐字承诺。面板的独立手写表保留该例，防止验收范围被扩大。

## 3. 需求覆盖账本

证据等级：**S**=本次静态核实；**H**=已读且核对适用性的历史运行记录；**T**=本次实际执行。H 不写成本轮重新通过。

| PRD 需求/验收及定位 | 本轮边界 | 计划 / 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- |
| WORK-F4/AC-3 · PRD L261 | 嵌入错误、权威候选与建议的直接回归 | 原 M2；首个 gaps M1/M2；gaps-2 M1/M2 · V-1/V-3/V-4 | T 注册/内置分支、未知字段与类型错误保留位置和可用名称；H 新 C‴ 作者出口 | 已满足（本轮诊断范围） | G-01 保持解决；G-06 已解决 |
| WORK-F13/AC-2 · PRD L494 | 错误浮层/终端建议、修复清除及 D-05 字面量边界 | gaps-2 M1/M2 · V-3/V-4/V-5 | T broken-anchor、lang、默认值诊断与面板精确同文；隐藏/重开/Esc 与清除、last-good 通过 | 已满足（本轮诊断范围） | G-06/D-05 |
| WORK-F15/AC-2 · PRD L543 | P1 格式的 P2 诊断增量；整条终验 P3 | gaps-2 M1/M2 · V-1/V-4 | T CLI JSON 被标准解析器读取，text/JSON 一致；S report 字段/schema 不变；H new 工程 check/build/dev 同文同位 | 已满足（本轮格式范围） | G-06 已解决；不关闭 P3 AC |
| WORK-N6/AC-2 · PRD L730 | 本次错误出口的失败安全回归 | gaps-2 M1/M2 · V-1/V-3/V-4 | T CLI 失败后 dist 树不变，dev HTTP last-good 原字节与原页面保持；H 六阶段同一 new 工程 | 已满足（本轮错误路径） | G-06 直接回归 |
| WORK-N6/AC-3 · PRD L731 | 完整自有诊断、位置与非零退出 | gaps-2 M1/M2 · V-1/V-3/V-4 | T PG-01 全文、3:7、check/build exit 1、建议非空；H 原修复全量诊断组 | 已满足（本轮诊断范围） | G-01/G-06 |

**完整去向账本为[逐 AC 附件](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/coverage.md)**：从当前 PRD 提取全部 111 个 AC，逐行列当前 PRD 位置、上轮状态、计划/记录、证据与去向；上述 5 行为本轮复核，其余 106 行明确标「本轮未复核」，保留历史状态和跨期终验，不用局部通过覆盖它们。

另按增量合同核对：gaps-2 D1–D5 / V-1–V-3 对应 writer/reader、完整片段、普通作者值、外部原文和作者参考；D6 / V-4 对应配对与安装；V-5 的完整源门与 docs-dev 32 项按 H 复用。本轮 6 个浏览器用例不等于完整 32 项重跑。

作者旅程：从已可用页面保存错误 → SSE/终端和面板给出完整诊断 → 页面继续保留旧版本 → 键盘关闭/重开 → 修复后 errors 归零。T 两个组件错误用例及内容站真实输入走通该链。手写读取表通过受控 SSE 单独验证 DOM 解析，明确不把它当真实输入集成证据。UAT-15 本人验收、官方站完整读者旅程与原设备/性能终验不在本轮。

## 4. Triage 与决策账本

待决 **0**；本轮没有新增问题卡或用户确认。已有决定沿用其来源，不把选项默认值或无回答当批准。

| 决策 ID | 关联 G-ID | 已有用户选择 / 日期与来源 | 当前验收边界与去向 |
| --- | --- | --- | --- |
| D-01 | G-02 | 2026-10-13，原计划 C-6/§11；R2 §4 | Firefox P2 保留未验证、P4 恢复；不豁免其他设备 |
| D-02 | G-03 | 2026-10-14，PRD D15；R2 §4 | 本人三个可见成果，历史 C4 确认保留；本轮未代填新 UAT |
| D-03 | G-04 | 2026-10-14，PRD D16；R2 §4 | 公开安装/Pages/回滚在 P4，原 M7/V-15 |
| D-05 | G-06 | 2026-10-15，R2 §4 记录用户明确选择「现在补齐」 | 按 gaps-2 D1/D2/D4 的受控内容边界完成逐字显示，并以 R‴/C‴ 固定安装验证；本轮关闭，无新任务 |

gaps-2 D1/D2 的空串、自有读法和普通作者值分类另见该计划 §0.2 及[同轮计划评审处置](DOCS-SSG-P2-gaps-2.review.md)，属于已有批准边界；本轮未重新裁定 CommonMark 等价、第三方原文或显示分类。

## 5. 待核实项、验证与基线

| ID | 尚缺事实及影响 | 具体补证方式 | 现有去向 |
| --- | --- | --- | --- |
| U-01 | 原 N3/AC-4 稳定版浏览器与实体手机完整证据；VoiceOver、原生 200% 缩放 | 在原生渠道记录实际版本、入口、当期操作、CSP/请求和结果；Chromium 尺寸仿真不替代 | 原 M6/V-11；Firefox 另按 D-01 在 P4 |
| U-02 | 当前候选正式 A/B 各类最大页五冷样本；P2 演示类型的移动端 LCP/CLS；内容 new 五轮 | 按 PRD N1/N2 与原 M6/V-12 的样本合同补当前候选，或给出受影响输入等价证据 | 原 M6；先补证，不预判预算失败 |
| U-03 | 当前组件围栏在真实 GitHub 文件预览的效果 | 在 P4 公开来源上核对固定版本原 Markdown 的占位及其余正文，记录 URL/版本/截图 | 原 M7/V-15 |

上述 3 项全部继承 R2，不阻塞本增量 G-06 的分类或关闭；继续限制整 P2 终验。缺的不是偏好，本轮不重复要求用户裁定事实。

### 本轮实际验证

[执行脚本](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/run-checks.py)使用计划 §9.3 的进程级本地 Git remap、Python 3.12 PATH shim 和 mbx；没有安装工具或改全局设置。环境为 macOS arm64、Rust 1.99.0、mbx 1.15.0、Node 26.1.0、Playwright 1.63.0 / Chromium 153.0.8010.12。Chromium 是测试引擎，未充作最新稳定版或实体设备。

| 实际命令 / 入口 | 本轮结果 | 证据 |
| --- | --- | --- |
| `mbx test --locked -p rustify-docs-cli --test diagnostic_text` | **6/6**；两份 writer 手写表、完整默认值与键、路径/anchor/空语言、region 前缀、site! 路由、lang 后处理 | [日志](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/cli-diagnostic-text.log)、[命令与退出](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/checks.json) |
| 已安装 C‴；`npx --no-install playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'gaps embedded diagnostics\|gaps-2 code spans' --workers=1 --reporter=json --output=<隔离目录>` | **6/6**，0 failed/skipped/flaky；gaps-2 的 36 个 1280/375 × 浅/深布局视图，另有原两个 gaps 用例的 8 个 375/320 × 浅/深视图。六份 audit 的 console/pageerror/CSP/失败请求/4xx+ 均空 | [原始 reporter](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser.json)、[附件与审计索引](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser-summary.json) |
| 安装身份、提交与文件比较 | 安装入口前后哈希/source 一致；C‴ 五文件与六框架锁配对正确；当前 55 个相关文件等于 C‴；C‴ 至 HEAD 的产品差异 0 | [前](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/installed-before.json)、[后](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/installed-after.json)、[独立审计](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/input-audit.json) |

已实际查看本轮[默认值手机深色](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser/t04/08-gaps2-default-unknown-375-dark.png)、[lang 桌面浅色](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser/t03/02-gaps2-frontmatter-ticks-1280-light.png)、[路径/anchor 手机深色](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/browser/t02/08-gaps2-anchor-375-dark.png)：字符完整，等宽分类和换行符合已有合同，关闭入口可见，未见横向溢出。截图数量不当作独立设备数量。

### 历史证据的适用性与未执行项

- **H 源门**：已读取 PG-01 修复阶段 [13 门](../validation/docs-ssg-p2-gaps-2/apply-r1/m1-checks.json)和原始 Rust 结果；docs/CLI/syntax/独立 SSR 为 277/158/31/30 passed，CLI 原有 1 ignored；hydrate、Clippy、build 与两图在其记录基线上通过。本轮没有重新执行这些全量门。R‴ 到 C‴ 只变配对元数据，当前产品与 C‴ 一致，根锁只有 CLI 版本差异，按此范围复用。
- **H 浏览器/安装**：工作树和安装入口完整 docs-dev 各 32/32、真实安装与 new 工程六阶段/24 视图沿 M1/M2 PG-01 记录；本轮没有重跑完整项目、重新安装或重跑该 new 脚本。当前二进制哈希与这些记录相同。
- **范围限制**：未重新完整核查官方站、减少动效/axe、Rust 编译失败完整旅程、用户 UAT、稳定浏览器/真机、性能或公开渠道。所有结论限上述 T/S/H 边界。

| 输入 | SHA-256 / 状态 |
| --- | --- |
| PRD | `22f4fac99b06a23cda3c34f7ee0c66c11369ccd60a34d71fecee2617aa4c16c1`，与 R2 相同 |
| 原计划 / 首个 gaps | `0e2b26d03aa30b2a8cc8bf6baa80eb21b0d043a281e8953e4a513ba3c29b921f`（5/7）/ `397a1ebe5129ce66d2a607649c6d44065f87a6252313ad61f6d662723e3000ff`（2/2） |
| 本次 gaps-2 | `a8976ea5e9a063f6e8f804287cc39a35fe6962801deacc9889aa7be1778a68e8`（2/2） |
| M1 / M2 完成记录 | `4fc230e040d599676ccdd8253099f489af8c40fc286fa425c46aa91aca6033d9` / `4fb6448fb83656e17f46aace828efd8eb85cd1cd36cd5d284dfee9bfeefdc5aa` |
| 上轮/代码报告及完整输入 | 版本表见[输入审计](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/input-audit.json)；初始 HEAD、dirty 清单、已存在文件的逐项 SHA-256、index/工作树差异摘要见[基线](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/baseline.json)。哈希清单不表示逐文件审阅 |

测试后输入漂移为 0、安装身份不变，见[核对](../validation/docs-ssg-p2-gaps-2/prd-dev-gaps/verification-after-tests.json)。交付前另核对本轮允许写入的报告、证据和 SDLC 派生页之外的输入；最终结果保存在同目录 `final-verification.json`。既有 dirty 为台账、报告/证据、fixture 日志与缓存，产品源码无未提交实现；本轮产物也未提交或推送，未声明远端同步。

## 6. Follow-up 与 gaps plan

| G-ID / D-ID / PRD | 当前结论 / 已确定工作 | 验收或解除条件 | 计划去向 |
| --- | --- | --- | --- |
| G-06 / D-05；F13/F15/N6 | 确定增量已完成，无剩余实现或补验任务 | 本轮原反例与扩展边界通过，固定 R‴/C‴ 与已安装出口一致 | [既有 gaps-2](DOCS-SSG-P2-gaps-2.md) Ready、实施 2/2；无需新计划 |
| G-01/G-02 | 当前直接回归与文档核对无新缺口 | 前轮关闭保持 | 首个 gaps 2/2，无新任务 |
| G-03–G-05 | 本轮不重新验收 | 沿 R2 原结论、决定及版本；不得从本轮 6 项测试扩成完整重验 | 原路径保留 |
| U-01–U-03 / K-01、K-02 | 既有缺证与后移公开发布 | 见 §5 的具体补证方式 | 原 M6/V-11/V-12、M7/V-15；不重复承接 |

**无需 gaps plan**：没有尚未被计划承接的已确定增量，因此未调用 dev-plan，也不创建空计划。报告已完成；既有用户决定继续有效；本增量覆盖核查完成，整 P2 及跨期终验保持未完成。SDLC 登记本轮独立对象并关联 R2，只回写本轮覆盖范围，历史报告和原完成事实保留。

原 M6/M7 补证完成后，再核查整期范围：

```text
$review-prd-dev-gaps docs/plan/DOCS-SSG-P2.md
```
