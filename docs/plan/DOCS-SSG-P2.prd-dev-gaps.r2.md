# DOCS-SSG-P2 · PRD 与开发交付差异复审（R2 · 修复效果复审）

| 项目 | 内容 |
| --- | --- |
| 复审对象 | [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md)（2/2，SHA-256 `397a1ebe5129ce66d2a607649c6d44065f87a6252313ad61f6d662723e3000ff`）对 [PRD](../PRD-DOCS-SSG.md) **P2（第 2 期）**的补齐；连同[原计划](DOCS-SSG-P2.md)（SHA-256 `0e2b26d03aa30b2a8cc8bf6baa80eb21b0d043a281e8953e4a513ba3c29b921f`，与上轮相同，仍 5/7） |
| 上轮报告 | [DOCS-SSG-P2.prd-dev-gaps.md](DOCS-SSG-P2.prd-dev-gaps.md)，SHA-256 `341778d2179cf441ea175fdfa0dd1d149ecdd71946bc1391d65a494b76041436`，评审对象 `5fdbf99f-8f49-4629-b1c8-89b2335d0a6b`；原文与结论保持不变 |
| 前后版本 | PRD `12f83716…` → `22f4fac99b06a23cda3c34f7ee0c66c11369ccd60a34d71fecee2617aa4c16c1`（只增 P2 Firefox 同步）；代码 `d2f20b03`（固定 C5 0.1.15 / R5 `7930fd9a`）→ `8a8268e6`（固定 C′ `79a77825` 0.1.17 / R′ `745ba6b7`）。R′ 之后产品差异只有 C′ 的配对文件 |
| 本轮范围 | 上轮全部 G-ID（G-01–G-05）、待核实 U-01–U-03、原计划未完 K-01/K-02，以及受修复直接影响的用户路径：嵌入诊断三入口与 dev 面板、减少动效、默认主题稳定状态 AA |
| 已有处置 | [gaps M1 记录](DOCS-SSG-P2-gaps.records/M1.md)、[gaps M2 记录](DOCS-SSG-P2-gaps.records/M2.md)；代码评审 [R1](DOCS-SSG-P2-gaps.code-review.md)（F-01）与 [R2](DOCS-SSG-P2-gaps.code-review.r2.md)（F-01 已解决，通过验收） |
| 类型 | **修复效果复审**，不是完整重审；上轮其余覆盖行标「本轮未复核」 |

- 评审对象 ID：`c8480584-6578-4587-972a-8294319685fc`，复审（`rechecks`）上轮 `5fdbf99f-8f49-4629-b1c8-89b2335d0a6b`。
- 审查日期：2026-10-15 +08:00。checkout：`main` / `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` / dirty（台账、评审/差异报告、fixture 日志与缓存；产品源码无未提交改动，见 [initial-status](../validation/docs-ssg-p2/prd-dev-gaps-r2/initial-status.txt)）。
- 实测对象是 M2 已安装的 C′：`rustify-docs` 0.1.17、panic unwind、source C′，SHA-256 `47486f96ff0985f63d36590d13f5bbd7ddbcbba3d501f40f6f561f00be8c9d7c`，与 M2 安装记录一致。本轮未重新安装。
- 输入为 gaps 计划，按其「需求来源」定位到 PRD P2、原计划和上轮报告。上轮报告的下一步与 gaps 代码评审 R2 都指向这次复审。原 PRD、计划、记录和代码都保持只读。

## 1. 结论与范围

**旧问题：上轮的 2 项明确遗漏（G-01、G-05）和 1 项待同步的合理变更（G-02）已解决；G-03、G-04 的已有决定仍然有效，无需动作。U-01–U-03 仍待核实，原计划的 K-01/K-02 仍未完成。**

**本轮新增 1 项需 Triage 的实现差异（G-06），2026-10-15 已由用户决定（D-05）：现在补齐。** 新增差异的分类：A 0、B 0、C 1（已决 1、待决 0）。待核实 3 项（U-01–U-03，全部沿用上轮）。需要跟进 1 项：G-06 由新的增量计划 [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md)（**Ready**，0/2，尚未开始）承接，见 §6。

- 上轮因 G-05 而「未核实」的 WORK-N5/AC-1、AC-6，本轮复核后**已满足**（限本期新增页面类型与默认主题）。实测补齐了 gaps 计划只在导入主题下扫描的 loading/failed/client-only 状态。
- 上轮「部分满足」的 F4/AC-3、F15/AC-2、N6/AC-3、N5/AC-5 转为已满足。N3/AC-4 仍是部分满足：Firefox 例外已同步进 PRD，其他稳定版浏览器与实体设备仍缺证（U-01）。
- **整 P2 尚未终验**：原计划仍为 5/7，M6 缺证与 M7 公开发布由原计划承接，D16 保持不变。本报告完成不等于产品交付完成，也不授予合并或发布。

呈现路径为构建期 Rust 工具链 + DOM 静态 HTML + 局部交互岛，本轮不涉及 GPU、Portal 宿主或远端写。本次实测范围：已安装 C′ 的内容站和组件 fixture 反例（check JSON/文本、build、dev SSE 与面板），以及由 C′ 构建的官方子路径站（build_id 与 M2 相同）上的减少动效和默认主题状态 axe 扫描。均在 localhost 进行，没有公开安装、推送或部署。

### 上轮问题复审

| 来源报告 / G-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [上轮](DOCS-SSG-P2.prd-dev-gaps.md#g-01--嵌入错误没有可用名称与具体修复建议--a) `341778d2…` / G-01 | A：嵌入错误正确拒绝、位置正确，但没有列出可用组件或属性名，`suggestion` 为 null | gaps M1（R `cc5fded3`）、F-01 修复 R′ `745ba6b7`；M2 C′ `79a77825`（0.1.17） | **已解决** | **T** 在 C′ 上重放原反例，见 [diagnostics.json](../validation/docs-ssg-p2/prd-dev-gaps-r2/diagnostics.json)。<br>内容站：`MissingCounter` 列出 `CardGrid`、`Steps`、`Tabs`；Steps 类型错误给出 `Vec<StepSpec>`（required）和 Static builtins 链接。<br>组件 fixture：`MissingCounter` 列出 `BrowserCounter`、`CardGrid`、`Counter`、`Steps`、`Tabs`；`Counter.unexpected` 列出 `initial`（`i32`，默认 `2`）和 `label`；`initial: wrong-type` 给出 ``Write `initial: 0` …``。<br>位置与上轮完全相同（6:1、10:1、6:1、13:3、9:3）。每例 check JSON、文本 `= help:` 与 build 同文，null 为 0，退出 1，dist 与锁不变。<br>**H** M1/M2 三入口 × 四变体矩阵、真实 new 工程（含 raw 默认值）、R2 复审 | 无；dev 面板含反引号字面量的残留见 G-06 |
| 上轮 / G-05 | A：系统请求减少动效时，岛控件仍有 150ms 过渡（Tabs 375px 的 computed 为 `0.15s`，且有 running 的 opacity CSSTransition） | gaps M1 `docs.css` reduce 规则（scope 及全部后代的 `0s !important`，Spinner `animation: none`）；M2 C′ 复验 | **已解决** | **T** [reduced-motion.json](../validation/docs-ssg-p2/prd-dev-gaps-r2/reduced-motion.json)：C′ 官方站，tabs/button/switch/loading × 375/1280。<br>reduce 下原反例 Tab 的 computed 为 `0s/0s`；从首个脚本起整页 0 次 `transitionrun`/`animationstart`，0 个 running 动画；Spinner 为 `none`；`scroll-behavior: auto`。<br>正常偏好对照：切 tab 触发 6 次 transitionrun，Spinner 转动，`smooth`。<br>外壳 `<html>` 自带 `data-rustify-scope`（`shell.rs` L391），规则覆盖整页。<br>**H** M2 F-01：默认主题 17 次、导入主题 9 次采样全部静止，正常偏好原语义 | 无 |
| 上轮 / G-05 关联：N5/AC-1、AC-6 未核实 | 首轮 108 次扫描含启动过渡的瞬时 serious；完整状态未复验 | gaps V-3：官方 108 次 + fixture 导入主题 32 次 | **已解决** | **H** M2 F-01：官方 27 路由 × 浅/深 × 1280/375 共 108 次，0 violations。本轮由 C′ 重建，build_id `66edda8b1a7cba74` 与之相同。<br>**T** gaps 只在导入主题下扫过 loading/failed/client-only，本轮在默认主题下补扫：<br>[state-axe](../validation/docs-ssg-p2/prd-dev-gaps-r2/state-axe.json) 官方站 loading/failed 16 次；[fixture-default-axe](../validation/docs-ssg-p2/prd-dev-gaps-r2/fixture-default-axe.json) 去掉导入主题的 fixture 中 loading/client-only/ready/failed 共 20 次。均 0 violations。<br>incomplete 只有 color-contrast，按区域归类为代码块渐变底和侧栏被吸顶页头遮挡（[归类](../validation/docs-ssg-p2/prd-dev-gaps-r2/state-incomplete.log)），即 M1 人工核实过的①②类 | 无；全 PRD 的 AA 在 P4 终验 |
| 上轮 / G-02 | B：Firefox P2 矩阵已批准延期 P4，但 PRD 未同步 | gaps M1 改 PRD WORK-N3 浏览器范围、AC-4、Q3 及状态行 | **已解决** | **S** `git diff d2f20b03 HEAD -- docs/PRD-DOCS-SSG.md`：只增 P2 Firefox 未验证、P4 恢复；明确 PRD 的 D-04（P1 浏览器例外）不延伸到 P2，其他浏览器与设备不豁免；PRD D-04/D15/D16 原文不变 | 无；Firefox 验证留 P4 |
| 上轮 / G-03 | B：五人限时研究改为本人验收（D15/D-02） | 无跟进 | **已解决**（无跟进项；决定继续有效） | **S** C5 以来 `crates/`、`site/`、README 的产品差异只有诊断/CSS/测试与配对锁，模板源码与引导未变，未触发 D-02 的重验条件 | 无 |
| 上轮 / G-04 | B：公开安装、Pages 与回滚后移 P4（D16/D-03） | 无跟进 | **已解决**（无跟进项；决定继续有效） | **S** C′ 仍只通过本地 Git 传输安装；作者参考写明公开可获取仍在准备，没有新增完成声明 | 无；原 M7/V-15 承接 |
| 上轮 / U-01 | 待核实：最新稳定 Chrome、真实 Safari、实体手机、VoiceOver、原生 200% 缩放 | gaps 显式排除，原 M6/V-11 承接 | **待核实** | 原计划仍 5/7、M6 进行中，本轮未取得新设备证据 | 原 M6 按解除条件补证 |
| 上轮 / U-02 | 待核实：当前候选正式 A/B 五冷样本、P2 演示类型 Slow4G、new 五轮 | 原 M6/V-12 承接 | **待核实** | 本轮未测性能；R′ 的 CSS 只在 reduce 媒体下变化，与预算无关的推断不当作证据 | 原 M6 补证 |
| 上轮 / U-03 | 待核实：组件围栏的真实 GitHub 文件预览 | 原 M7/V-15 承接 | **待核实** | 未取得平台预览 | P4 发布时按原计划核对 |
| 上轮 / K-01、K-02 | 原计划 M6 进行中、M7 按 D16 后移 | 原计划承接 | **仍存在** | 原计划 SHA-256 与上轮相同，快照仍 5/7 | 原计划续做；不进 gaps |

小结：旧问题 G-01、G-02、G-05 实际解决；G-03、G-04 的决定仍然有效；U-01–U-03 待核实，K-01/K-02 仍存在，回归 0 项。修复引入或暴露的新问题 1 项（G-06）。未覆盖/缺证范围见 §5。

## 2. 三类差异（本轮新增）

### A · 明确遗漏（0）

无。

### B · 合理变更（0）

无。

### C · 需 Triage 的实现差异（1，已决）

#### G-06 · dev 面板把建议中字面量自带的反引号当作代码分隔符，显示的修复建议与 check/build/终端不一致 · C

- **PRD 期望**：
  - WORK-F13 行为要求内容或编译错误显示在浏览器浮层和终端里；AC-2 要求浮层显示修复建议。
  - WORK-F15 诊断格式要求每条带修复建议。
  - 本期承诺写在两处：gaps 计划 §5.1 第 4 条「text 的 `= help:`、JSON 和 dev 面板的建议文本逐字一致，面板只格式化反引号」；作者参考 [docs-ssg.md](../docs-ssg.md) 嵌入诊断段「`check`, `build` and `dev` (the terminal and the page's problem panel) report the same location and repair」。
- **实际交付**：把注册默认值改为 `label: String = "a`b".to_string()` 后触发未知属性。
  - check JSON、文本、build、dev SSE 与 dev 终端都逐字保留 `` default expression `"a`b".to_string()` ``，位置 9:3。
  - 浏览器面板显示为 `default expression "ab".to_string()`：字面量中的反引号丢失。面板还把 `") for "`、`"; rename or remove "` 渲染成代码片段，把 `Counter`、`unexpected` 渲染成普通文本，排版颠倒。
  - 位置、拒绝、last-good 与修复后清除都不受影响。
- **证据链**：
  - gaps M1 记录「未完项与继续动作」把它列为观察，交复审判断是否立项。代码评审 R2 认为它不属于 F-01 修复引入的问题，没有计为 finding。
  - 本轮 **T**：[diagnostics.json](../validation/docs-ssg-p2/prd-dev-gaps-r2/diagnostics.json) 的 `tick`/`tick_dev`、[tick-panel.json](../validation/docs-ssg-p2/prd-dev-gaps-r2/tick-panel.json)、[截图](../validation/docs-ssg-p2/prd-dev-gaps-r2/tick-panel.png)、[dev 终端](../validation/docs-ssg-p2/prd-dev-gaps-r2/tick-dev.stderr)。
  - 根因 **S**：[dev.js](../../crates/rustify-docs/assets/dev.js) `prose()`（L208–218）按 `` ` `` 切分并交替渲染为文本/`code`。`fill()` 对 `message`（L259）和 `suggestion`（L260）都用它。
  - 同根因的 `message` 分支（例如 serde 报告的值本身含反引号）只做了静态阅读，**本轮未实测**。
- **影响**：只影响已注册组件中默认值为含反引号的字符串（例如默认值是 Markdown 片段）。作者在面板上会看到被误抄的默认表达式，终端和 JSON 仍正确；不阻塞主路径或任何 AC 的原例。gaps 的 V-1/V-3 用例都不含反引号字面量，因此没有覆盖到。
- **分类依据**：
  - PRD AC 的原例都满足：AC-2 是失效链接的建议，F4/AC-3、N6/AC-3 在终端和 JSON 中完整。差异出在计划与作者参考的「逐字一致」承诺上的边界输入。
  - 修复要改动建议的显示协议：面板无法从纯文本可靠区分字面量反引号与分隔符，需要换分隔写法或结构化片段。因此要走新的 R/C 配对与安装复验。
  - 是否在 P2 收口前处理，以及作者参考是否需要标注限制，属于验收取舍，现有决定不能确定，实施者也已交给复审。因此归 C，不归 A。
- **处置**：D-05（2026-10-15 用户选择）——**在 P2 现在补齐**：新建增量计划，修显示协议与面板，配对新 R/C 并安装复验。
- **Follow-up**：
  - 内容：dev 面板显示的 `suggestion` 与 `message` 和 text/JSON 逐字一致，含字面量反引号的情形也不例外；不改变位置、拒绝语义、错误收集与 last-good。
  - 验收：已安装的新 C 在真实 dev 面板中，对含反引号的默认值字面量（未知属性与 invalid_props 各一）和含反引号的值，显示文本等于 text/JSON 去掉分隔后的原文；在 1280/375 × 浅/深下不溢出，焦点行为不变。
  - 去向：[DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) M1/M2，V-1–V-5。

## 3. 需求覆盖账本（本轮范围）

证据等级：**S**=本次静态核实；**H**=已读的历史/既有运行记录，保留其版本；**T**=本次实际执行。T 不自动覆盖 H 的完整范围。下表是本轮复核的 12 行；上轮其余 99 行列在下方「本轮未复核」，按上轮状态沿用，不代表本轮重验。

| PRD 需求/验收及定位 | 本期范围/已批准变更 | 计划与里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F4/AC-3 · PRD L261 | P2 新增 | 原 M2/M4；gaps M1/M2 · V-1/V-3 | gaps M1/M2 | T 原反例给出完整名称/字段清单与类型建议，位置不变；H 三入口矩阵 | 已满足（上轮部分满足） | G-01 已解决 |
| WORK-F15/AC-2 · PRD L543 | P1 格式的 P2 嵌入增量 | gaps M1/M2 · V-1 | gaps M1/M2 | T JSON 每条含 severity/rule/file/line/column/message/suggestion，嵌入建议非空，可被标准解析器解析 | 已满足（上轮部分满足） | G-01 已解决；G-06 只影响面板显示 |
| WORK-N6/AC-3 · PRD L730 | P2 当期 NFR 复核 | gaps M1/M2 · V-1/V-3 | gaps M1/M2 | T 各嵌入错误都有文件/行列/说明/建议，退出 1 | 已满足（上轮部分满足） | G-01 已解决 |
| WORK-N5/AC-5 · PRD L713 | P2 当期 NFR 复核 | gaps M1/M2 · V-2 | gaps M1/M2 | T reduce 下 0 过渡/0 动画、Spinner 静止、无平滑滚动；H 窄屏/200% 仿真 | 已满足（上轮部分满足；原生缩放仍 U-01） | G-05 已解决 |
| WORK-N5/AC-1 · PRD L709 | P2 当期 NFR 复核 | gaps M1/M2 · V-3 | gaps M1/M2 | H 稳态 108 + 32 次 0 violations；T 默认主题官方站 loading/failed 16 次 + fixture loading/client-only/ready/failed 20 次，共 36 次 0 violations；incomplete 按 M1 人工依据 | 已满足（上轮未核实） | G-05 关联已解决 |
| WORK-N5/AC-6 · PRD L714 | P2 当期 NFR 复核（新页面类型） | gaps M1/M2 · V-3 | gaps M1/M2 | 同上；默认主题严重/重大违规 0 | 已满足（上轮未核实） | 同上；终验 P4 |
| WORK-N3/AC-4 · PRD L673 | P2 当期 NFR；Firefox 已批准延期 | gaps M1 · V-4；原 M6 · V-11 | gaps M1；原 M6 | S PRD 同步完成；U 其他稳定版浏览器/设备 | 部分满足 | G-02 已解决；U-01 → 原 M6 |
| WORK-F4/AC-1 · PRD L259 | P2 新增（回归） | 原 M1/M2；gaps V-14 | 原 M2/M6；gaps M1/M2 | T C′ 正常构建 fixture 与官方站（build_id 不变）；H components 46/46、official 53/53（R） | 已满足 | 直接回归 |
| WORK-F4/AC-2 · PRD L260 | P2 新增（回归） | 同上 | 同上 | H R 上全量浏览器；R′ 相对 R 只改 `embed.rs` 显示，水合/岛未变 | 已满足 | 直接回归，沿用 H |
| WORK-F13/AC-2 · PRD L494 | P1 条件的 P2 跨期终验（浮层建议） | gaps M1/M2 · V-3 | gaps M1/M2 | T SSE 与终端建议同文；面板正常建议同文（H docs-dev 2/2、R2 raw 8 组合）；字面量反引号失真 | 已满足（原例）；残留差异见 G-06 | G-06/D-05 → [gaps-2](DOCS-SSG-P2-gaps-2.md) |
| WORK-F15/AC-3 · PRD L544 | P2 新增 | 原 M2；gaps M1 · V-1 | gaps M1/M2 | H 真实 host/wasm 组件测试同次报告未注册名与独立 Rust 错误（C′ 上 CLI 152 项） | 已满足 | 直接回归 |
| WORK-N6/AC-2 · PRD L729 | P2 当期 NFR 复核 | gaps M1/M2 · V-1/V-5 | gaps M1/M2 | T 内容站、fixture 与反引号三组全部拒绝后 dist 逐文件不变；H dev last-good 原字节 | 已满足 | 直接回归 |

**本轮未复核（沿用上轮状态与去向，不代表本轮重验）**：

- **已满足（43）**：WORK-F1/AC-1–AC-4；F5/AC-1–AC-6；F6/AC-1–AC-3；F8/AC-1–AC-6；F13/AC-1、AC-3、AC-4；F16/AC-1、AC-2；N1/AC-1、AC-3；N2/AC-1、AC-3、AC-4；N3/AC-1–AC-3；N4/AC-1–AC-4；N5/AC-3；N6/AC-1、AC-4；N7/AC-1–AC-4。
  - 其中 N4 的两图零 Makepad 与 N7 的 CSP 有 C′ 上的 H 证据（M2 F-01、代码评审 R2）。
  - N1/AC-3 的静态体积为 C5 数字；之后 `docs.css` 只增 reduce 规则，本轮未重测。
- **已确认变更（1）**：WORK-F1/AC-5（G-03/D-02）。
- **部分满足（1）**：WORK-N1/AC-2（U-02 → 原 M6/V-12）。
- **未核实（3）**：WORK-F4/AC-4（U-03 → 原 M7/V-15）、N1/AC-4（U-02）、N2/AC-2（U-02）。
- **范围外（51）**：WORK-F2/AC-1–AC-4；F3/AC-1–AC-4；F7/AC-1–AC-5；F9/AC-1–AC-7；F10/AC-1–AC-4；F11/AC-1–AC-4；F12/AC-1–AC-4；F14/AC-1–AC-4；F15/AC-1、AC-4、AC-5；F16/AC-3（D16）、AC-4；F17/AC-1–AC-5；N1/AC-5；N3/AC-5、AC-6；N5/AC-2、AC-4。去向同上轮。

**连续用户旅程**：

- **作者 UAT-10（错误路径）**：H 已安装 C′ 的 new 工程从冷错误到类型错误，再到 dev 修复后清除。T 本轮内容站、fixture 与反引号反例三入口同文。唯一残留是 G-06 的面板显示。
- **读者 UAT-12（减少动效）**：T 默认主题下控件接管、hover、切换、加载状态都没有动效，平滑滚动关闭；导入主题沿用 H。
- 其余旅程沿用上轮，本轮未复核。

## 4. Triage 与决策账本

### 待决

无。G-06 的问题卡保留如下：2026-10-15 在本会话以交互问答询问，用户当日选择了「现在补齐」，没有使用默认选项或超时。

- **差异**：dev 面板丢失建议中字面量自带的反引号，并颠倒代码排版；终端与 JSON 正确。
- **影响**：只在默认字符串含反引号时出现，罕见，不阻塞。
- **待决定**：是否在 P2 收口前修复？修复需新 R/C 配对（0.1.18）。
- **选项**：
  1. 延期到 P3：随 P3 的第一次 R/C 配对补齐，P2 记为已知限制。（推荐：影响面小，终端/JSON 正确，P3 本来就要冻结新 R/C，可省掉 P2 的第三次配对与安装复验）
  2. 现在补齐：新建 P2 增量计划，修显示协议与面板，配对 0.1.18 并安装复验。
  3. 接受为永久限制：只在作者参考注明。

### 已决

本报告的决策编号跳过 D-04，以免与 PRD 的 D-04（P1 浏览器例外）混淆。

| 决策 ID | 关联 G-ID | 用户选择与理由 | 确认日期与来源 | 新目标/验收边界 | 跟进/延期/豁免去向 |
| --- | --- | --- | --- | --- | --- |
| D-01 | G-02 | 沿用上轮：Firefox 稳定版与引擎延期 P4，P2 保留未验证 | 2026-10-13；原计划 C-6/§11 | 不豁免其他 P2 设备 | 已同步 PRD；验证留 P4 |
| D-02 | G-03 | 沿用上轮：开发阶段由本人验收 | 2026-10-14；PRD D15 | 三个成果；引导/模板行为改变时重验 | 无任务；本轮未触发重验条件 |
| D-03 | G-04 | 沿用上轮：公开发布留 P4 | 2026-10-14；PRD D16 | 本地开发不等待发布 | 原 M7/V-15 |
| D-05 | G-06 | 选择「现在补齐」，未选推荐的延期到 P3：在 P2 内修好 dev 面板对建议与消息的逐字显示 | 2026-10-15；本会话交互问答 | dev 面板显示的 `suggestion`/`message` 与 text/JSON 逐字一致（含字面量反引号）；位置、拒绝、last-good 不变；按新 R/C 配对安装复验 | [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) |

## 5. 待核实项与审查基线

| ID | 缺失事实/材料及影响 | 补证方式 | 去向/是否阻塞 |
| --- | --- | --- | --- |
| U-01 | 沿用上轮：N3/AC-4 的最新稳定 Chrome、真实 Safari 与实体手机；VoiceOver 与原生 200% 缩放 | 原 M6/V-11 在原生渠道记录版本、入口和当期交互 | 原 M6；发布前必须补齐；不阻塞后续本地开发 |
| U-02 | 沿用上轮：N1/AC-4 当前候选正式 A/B、N1/AC-2 的 P2 演示类型 Slow4G、N2/AC-2 五轮 | 原 M6/V-12 | 原 M6；本轮的 CSS 变化不构成等价证明 |
| U-03 | 沿用上轮：F4/AC-4 真实 GitHub 预览 | 原 M7/V-15 | P4 |

本轮的覆盖限制（只是复用证据的边界，不是新的未知缺陷）：

- 导入主题下的 reduce/axe、官方站 ready 状态 108 次扫描，以及 CLI 全量 152 项，都沿用 M2 F-01 的 C′ 记录（同一安装 SHA）。官方站重建的 build_id 相同，fixture 导入主题未重跑。
- dev 的 Rust 编译错误浮层（F13/AC-4）未重跑。
- G-06 的 `message` 分支只做了静态阅读。
- 未测 Firefox/WebKit、实体设备和正式性能。

| 来源/本次验证 | 版本/结果与边界 |
| --- | --- |
| PRD | SHA-256 `22f4fac99b06a23cda3c34f7ee0c66c11369ccd60a34d71fecee2617aa4c16c1`；与上轮的差异只有 P2 Firefox 同步 |
| 原计划 / gaps 计划 | `0e2b26d0…`（5/7，未变）/ `397a1ebe…`（2/2） |
| gaps 记录 | M1 `10ce69bd…`、M2 `6cb26101…`；代码评审 R1 `11e1e615…`、R2 `e0614a52…` |
| 产品基线 | R′ `745ba6b7`、C′ `79a77825`（0.1.17）；HEAD `8a8268e6`，R′ 之后只有 C′ 配对文件与计划提交 |
| 本次诊断实测 | [probe-diagnostics.py](../validation/docs-ssg-p2/prd-dev-gaps-r2/probe-diagnostics.py)，在 gaps §9.2 shell 中运行（临时 python3→3.12.13、进程级 Git remap、`RUSTIFY_DOCS_CARGO=mbx`），exit 0。fixture 由 `docs-pages-prepare.py --project` 投影到 C′；首次 check 未出现 `rust-generation-changed`；锁不变，注册源码已还原 |
| 本次浏览器实测 | Playwright 1.63.0 / Chromium 153.0.8010.12。官方站：`tests/docs-ssg/serve.sh official subpath 4299` 绑定已安装 C′，build_id `66edda8b1a7cba74`。fixture 默认主题：临时副本去掉 `[theme]`，base `/`，build `37e545ee7cf387b9`，preview 4298。四个探针 exit 0，结束后关闭两个服务 |
| 环境 | macOS arm64、Rust 1.99.0、mbx 1.15.0、Node 26.1.0；完整命令与输入摘要见 [provenance.json](../validation/docs-ssg-p2/prd-dev-gaps-r2/provenance.json)。未安装工具，未改全局 Git/Python 或浏览器设置。官方站 worktree 登记沿用 serve.sh 的既有流程（C′） |

## 6. Follow-up 与 gaps plan

| G-ID / D-ID / PRD ID | 确定的增量工作 | 验收条件 | 依赖/阻塞 | gaps plan 任务或排除去向 |
| --- | --- | --- | --- | --- |
| G-06 / D-05；F13 浮层、F15 诊断 | dev 面板对 `suggestion`/`message` 的显示与 text/JSON 逐字一致，含字面量反引号 | 见 §2 G-06 Follow-up | 无产品阻塞；需新 R″/C″ 配对（0.1.18） | [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) M1（写法、面板、测试 → 库 R″）、M2（C″ 配对、安装、固定出口） |
| U-01–U-03、K-01/K-02 | 原期完整验收与后移发布 | 按原 M6/M7/V-11/V-12/V-15 | 设备/当时版本/发布条件 | 原计划承接，不进 gaps |
| G-01、G-02、G-05 | — | 已解决 | — | 已由 [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md) 交付，无剩余 |
| G-03/D-02、G-04/D-03 | — | 决定继续有效 | — | 无任务 |

**gaps plan：[DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md)，Ready，0/2，尚未开始。** 范围只含 G-06/D-05：自有诊断代码片段改为无歧义写法，dev 面板按同一规则读取，并以 R″/C″（0.1.18）本地固定配对复验。外部工具原文、JSON 结构、U-01–U-03 与原 M6/M7 明确排除。默认名 `DOCS-SSG-P2-gaps` 已被首个增量计划占用，因此加后缀 `-2`，不覆盖它。现有 [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md) 保持 2/2，不修改。没有实施代码、创建 issue、推送或发布。
