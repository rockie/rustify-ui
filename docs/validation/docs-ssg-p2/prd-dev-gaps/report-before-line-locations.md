# DOCS-SSG-P2 · PRD 与开发交付差异核查

## 1. 结论与范围

**存在 2 项明确遗漏（A）、3 项已有决定支持的合理变更（B），没有需要重新拍板的实现差异（C）。待核实 3 类，待决 0 项。** 3 项确定增量由独立 gaps plan 承接：嵌入诊断、减少动效、Firefox 例外的文档同步。报告完成不表示 P2 已终验。

审查对象：[原计划 v2](DOCS-SSG-P2.md) → [PRD](../PRD-DOCS-SSG.md) §7.1–7.3 的 **P2（第 2 期）**。从 PRD 提取本期 22 条 AC，逐项核查，并复核跨期终验和受影响 NFR；其他期次保留去向。本次是首次 P2 覆盖核查，不是 [代码复审 R2](DOCS-SSG-P2.code-review.r2.md) 的再复审，也不是 P1 覆盖报告的复审。

原计划快照为 **5/7**：M1–M5 已完成，M6 进行中，M7 按 D16 留 P4。因而“已完成计划”的输入前提不成立。M6 环境/验证缺证与 M7 后移工作列为 K-01/K-02，不包装为完成后新发现的功能遗漏；不复制进 gaps plan。

呈现路径为 **构建期 Rust 工具链 + DOM 静态 HTML + 局部交互岛**。没有新增 GPU、Portal、在线 SSR 或远端写。审查使用 HEAD 与当前工作树，产品源码与原 PRD、计划、M1–M6 记录保持只读。用户已有 dirty 文件未回退；本次产物未提交/推送。

评审对象 ID：`5fdbf99f-8f49-4629-b1c8-89b2335d0a6b`。审查日期：2026-10-14 +08:00。代码：`main` / `d2f20b03cf58c1d1ced3785397c501d412cd4167` / dirty；当前产品与固定 C5 的 `crates/`、`site/src/`、根站点配置无净差异。固定 CLI 为 0.1.15，C=`45878ce31763fe21b9276e563e2920d180787dee`，R=`7930fd9a978563e76468fef69d14f444159c47b5`。

本次实测：Docs **265** 项 Rust 测试通过；C5 重建官方子路径产物，浏览器 **46/46** 通过；独立 CLI 反例确认 G-01；P2 新页面类型的 108 次 axe 扫描及定向 CSSOM 复核发现 G-05。没有运行完整 C5 三入口矩阵、正式 A/B 性能、最新稳定浏览器/实体设备或公开发布。

## 2. 三类差异

### A · 明确遗漏（2）

#### G-01 · 嵌入错误没有可用名称与具体修复建议 · A

- **PRD 期望**：§3 WORK-F4/AC-3 要求名称/属性/类型错误包含 Markdown 位置及可用组件名或属性名；WORK-F15 行为、AC-2 和 WORK-N6/AC-3 要求可执行的建议，机器格式也可消费。
- **实际交付**：固定 C5 对 `MissingCounter` 和 `Counter.unexpected` 正确拒绝，但只说未注册/不存在，没有列出已有 `Counter`、`BrowserCounter` 或 `initial`、`label`；`suggestion` 均为 null。`Counter.initial: wrong-type` 定位到第 9 行第 3 列，提示 expected i32，但建议仍为 null。纯内容内置 Steps 的类型错误同样没有具体修复建议。
- **证据链**：原 M2/V-3、M4/V-9 → [M2](../validation/docs-ssg-p2/M2.md)、[M4](../validation/docs-ssg-p2/M4.md) → [rendering.rs](../../crates/rustify-docs-cli/src/rendering.rs) `preflight`（L568）、`located`（L649）和 helper 错误映射（L246）；[markdown.rs](../../crates/rustify-docs/src/markdown.rs) 内置错误分支（L1204） → [实际 C5 反例](../validation/docs-ssg-p2/prd-dev-gaps/component-probe.json)、[内置反例](../validation/docs-ssg-p2/prd-dev-gaps/content-diagnostics.stdout)。先在同一组件工程成功 check，再分别制造名称与类型错误；异常 check 均 exit 1，锁不变。
- **影响**：内容贡献者/编码代理必须另找注册声明，错误浮层与终端拿不到可直接执行的修复信息。正确拒绝和精确位置不等于满足诊断合同。
- **分类依据**：名字、字段、类型已有权威清单，恢复原要求无需产品取舍。[作者参考](../docs-ssg.md) L154–158 允许“没有更多具体信息”时建议为空，不适用于已有组件/字段清单和明确类型的这些反例。P1 报告 G-04 修的是 frontmatter 诊断，不承接 P2 新嵌入分支，不称其回归。
- **处置 / Follow-up**：用现有注册清单与内置 schema 补可用名称及针对缺值/未知字段/错类型的建议，保持原位置、错误收集、退出状态、JSON schema 与失败保全。check/build/dev 三个入口、普通/中文/CRLF 输入验证一致。去向：gaps M1/M2 的 V-1/V-3。

#### G-05 · 减少动效时岛控件仍运行 150ms 过渡 · A

- **PRD 期望**：WORK-N5/AC-5 在每期新增页面类型复核；系统请求减少动态效果时不使用平滑滚动与过渡动画。计划 §6、V-5/V-11 同样要求新演示满足此边界。
- **实际交付**：C5 的 Tabs 页面在 375px、`reducedMotion: reduce` 下，媒体查询为 true、岛为 ready，但未选中且可操作的 tab 的 computed `transition-property: all`、`transition-duration: 0.15s`；仍有 running 的 opacity CSSTransition（150ms）。Button 也出现启动 opacity 过渡。
- **证据链**：M2/M5 的 UI 出口 → [M5](../validation/docs-ssg-p2/M5.md)、[M6](../validation/docs-ssg-p2/M6.md) → [docs.css](../../crates/rustify-docs/assets/docs.css) L1260 的 reduce 规则、[style.rs](../../crates/rustify-docs/src/style.rs) L225 的层次、[controls.css](../../crates/rustify-components/css/controls.css) L8 的 `var(--motion-duration)`、[Theme::properties](../../crates/rustify-ui/src/theme.rs) L176 → [CSSOM/动画原始证据](../validation/docs-ssg-p2/prd-dev-gaps/reduced-motion.json)、[定向确认](../validation/docs-ssg-p2/prd-dev-gaps/contrast-confirmation.json)、[375px 实际截图](../validation/docs-ssg-p2/prd-dev-gaps/tabs-375-rest.png)。0s 规则位于 components 层，同选择器的 SDK 未分层规则仍把 duration 设为 motion-duration，utilities 也声明了 transition。
- **影响**：对动态效果敏感的读者已选择减少动效，却仍受到岛接管及控件状态的渐变影响。
- **分类依据**：系统偏好与实际动画均已确认，无豁免；可在 Docs 样式范围修正优先级，不需要改产品风格或扩到 GPU。
- **处置 / Follow-up**：让现有 Docs reduce 规则实际优先于 utility 与 SDK 控制规则；真实页面验证接管、hover、切页签时 computed duration 为 0 且不创建过渡，正常动效偏好保持原语义。默认/导入主题、浅/深、1280/375px、加载/就绪/失败及源码控制在同一产物走查。去向：gaps M1/M2 的 V-2/V-3。

**对比度证据校准**：首轮 [108 次 axe 扫描](../validation/docs-ssg-p2/prd-dev-gaps/accessibility.json)在 8 个页面/视口实例报告 serious color-contrast，含 4.44–4.45:1 主按钮和 3.41:1 tab。定向确认的 Button/Tab 稳态颜色达标，六次定向扫描无 color-contrast 违例；原扫描紧随 ready，捕获了从 fieldset 禁用变为启用后的 opacity 过渡。保留原红，但不将其报成八项永久配色缺陷，也不因局部稳定通过宣称全部 P2 状态 AA 已通过。G-05 的确定判据是 **减少动效时实际仍有 150ms 动画**，无需依赖这批瞬时色值。

### B · 合理变更（3）

下面保留原承诺变更的决定轨迹；G-03/G-04 已进入当前 PRD，不是仍待批准的偏差。延期不记为实现通过。

#### G-02 · Firefox 的 P2 矩阵延期到 P4，PRD 尚未同步具体例外 · B

- **PRD 期望**：WORK-N3/AC-4 要求各期桌面完整稳定版矩阵，当前文本只明确 P1 D-04；Q3 仍用未指定恢复期的旧表述。
- **实际交付**：P2 计划 C-6、§9.3、§11 明确记录用户 2026-10-13 选择 Firefox 稳定版和引擎延期 P4，P2 保留未验证。当前未交付 Firefox 通过证据。
- **证据链 / 依据**：[P2 计划](DOCS-SSG-P2.md) C-6、§11「Firefox」与 [M6](../validation/docs-ssg-p2/M6.md)的未验证/开发优先记录；PRD WORK-N3/AC-4、Q3 原文未含这一 P2 例外。现有 P1-gaps 的 V-7 只承接原期口径，不能替代本次同步。
- **影响 / 处置**：接受已批准延期，不重复询问或在 P2补测 Firefox；保留 P4 完整矩阵与其他未批准设备的缺证。Follow-up 仅同步 PRD WORK-N3、Q3 的具体范围和恢复期，不改历史验收记录。去向：gaps M1 的 V-4；D-01。

#### G-03 · 五人限时研究改为本项目用户验收 · B

- **期望 / 实际**：原五人、首次用户资格及十五分钟门已被 PRD D15 撤销；WORK-F1/AC-5、UAT-15 改为本人确认新建、改页及同源可交互演示三个成果。本人在 C4 材料交付后明确确认通过。
- **依据**：[PRD D15](../PRD-DOCS-SSG.md)、[决定原记录](../../.xgent-ai/sdlc/events/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735/5d6fb51a-64cf-4085-93a0-8f9b2781af97.md)、[M6「本人验收通过」](../validation/docs-ssg-p2/M6.md)。确认时间为 2026-10-14T17:41:31+08:00。
- **影响 / 处置**：不再要求招募或限时；保留原研究提案为历史。该结论属于 C4，C5 的三项修复和实际 dev 证据另列，不伪造 C5 本人重验。引导流程/模板未发生相应行为改变，不重新索要已经给出的决定。无新增 follow-up；D-02。

#### G-04 · 官方站公开安装、部署和回滚后移到 P4 · B

- **期望 / 实际**：原 P2 英文站上线出口改为本地 12 指南/24 类演示；WORK-F16/AC-3 及公开分发留 P4，P3/P4 本地开发不等待发布或既有环境补验。
- **依据**：[PRD D16、§7.1–7.3](../PRD-DOCS-SSG.md)、[决定记录](../../.xgent-ai/sdlc/events/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735/44303026-e66c-4cd8-b47d-7dde4ce62426.md)、[M6「开发优先与公开发布后移」](../validation/docs-ssg-p2/M6.md)。本次只在 localhost 使用已有固定 C5 和本地 Git 传输，不验证公开可获取。
- **影响 / 处置**：保留原 M7/V-15，待 P4 当时版本完成真正安装、Pages 全页/CSP/资源/回滚。不生成重复发布任务，不要求现在推送。无新增 follow-up；D-03。

### C · 需 Triage 的实现差异（0）

未发现需要新产品裁定的差异。已有 D15/D16 与 Firefox 决定继续有效；未回答、默认选项或超时均未用于批准任何变更。没有调用新的用户问答。

## 3. 完整需求覆盖账本

证据等级：**S**=本次静态核实；**H**=已读的历史/既有运行记录（保留版本）；**T**=本次实际执行；**U**=尚需补证。T 不自动覆盖 H 的完整范围。以下各行均从原 PRD 条目向下追踪；P1 独立功能没有整体重审，范围外行只作去向索引。新增 AC 与跨期终验/当期复核分别标注。

共同证据入口：本次 [Rust 日志](../validation/docs-ssg-p2/prd-dev-gaps/docs-test.log)、[C5 浏览器 JSON](../validation/docs-ssg-p2/prd-dev-gaps/official-subpath-results.json)、[产物 manifest](../validation/docs-ssg-p2/prd-dev-gaps/browser-manifest.json)；历史 [M1](../validation/docs-ssg-p2/M1.md)、[M2](../validation/docs-ssg-p2/M2.md)、[M3](../validation/docs-ssg-p2/M3.md)、[M4](../validation/docs-ssg-p2/M4.md)、[M5](../validation/docs-ssg-p2/M5.md)、[M6](../validation/docs-ssg-p2/M6.md)。以下简写均指这些实际入口与本报告 §2、§5，不能当成本次全部重跑。

| PRD 需求/AC及原行 | 本期范围/变更 | 计划/里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F1/AC-1 · PRD L187 | P1条件的P2跨期终验/回归 | M4/M6 · V-8/V-9/V-13 | M4/M6 | S new/doctor/原子写入；H C4未改模板/本人、C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F1/AC-2 · PRD L188 | P2新增/交付段 | M4/M6 · V-8/V-9/V-13 | M4/M6 | S new/doctor/原子写入；H C4未改模板/本人、C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F1/AC-3 · PRD L189 | P1条件的P2跨期终验/回归 | M4/M6 · V-8/V-9/V-13 | M4/M6 | S new/doctor/原子写入；H C4未改模板/本人、C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F1/AC-4 · PRD L190 | P1条件的P2跨期终验/回归 | M4/M6 · V-8/V-9/V-13 | M4/M6 | S new/doctor/原子写入；H C4未改模板/本人、C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F1/AC-5 · PRD L191 | P2新增/交付段 | M4/M6 · V-8/V-9/V-13 | M4/M6 | H D15本人C4明确通过；C5不冒称本人重验 | 已确认变更 | G-03/D-02，无新增跟进 |
| WORK-F2/AC-1 · PRD L214 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F2/AC-2 · PRD L215 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F2/AC-3 · PRD L216 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F2/AC-4 · PRD L217 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F3/AC-1 · PRD L237 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F3/AC-2 · PRD L238 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F3/AC-3 · PRD L239 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F3/AC-4 · PRD L240 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F4/AC-1 · PRD L259 | P2新增/交付段 | M1/M2/M6 · V-2/V-3/V-5 | M1/M2/M6 | S typed registry/SSR/围栏；T registry tests；H三同类岛 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F4/AC-2 · PRD L260 | P2新增/交付段 | M1/M2/M6 · V-2/V-3/V-5 | M1/M2/M6 | S typed registry/SSR/围栏；T registry tests；H三同类岛 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F4/AC-3 · PRD L261 | P2新增/交付段 | M1/M2/M6 · V-2/V-3/V-5 | M1/M2/M6 | T 正确拒绝/定位，但缺可用名字/属性及建议 | 部分满足 | G-01 → gaps M1/M2 |
| WORK-F4/AC-4 · PRD L262 | P2新增/交付段 | M1/M2/M6 · V-2/V-3/V-5 | M1/M2/M6 | S 普通围栏占位可读；U 当前版本真实GitHub预览 | 未核实 | U-03 → 原M7/V-15，保留P2 AC |
| WORK-F5/AC-1 · PRD L283 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F5/AC-2 · PRD L284 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F5/AC-3 · PRD L285 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F5/AC-4 · PRD L286 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F5/AC-5 · PRD L287 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F5/AC-6 · PRD L288 | P2新增/交付段 | M2/M4/M6 · V-4/V-5/V-9 | M2/M4/M6 | S 源切片/外部状态；T C5原文和禁脚本；H C4故障/C5 dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F6/AC-1 · PRD L310 | P1条件的P2跨期终验/回归 | M3/M6 · V-6 | M3/M6 | S 页图/七slot；T registered_generation/C5定制页；H footer含404 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F6/AC-2 · PRD L311 | P2新增/交付段 | M3/M6 · V-6 | M3/M6 | S 页图/七slot；T registered_generation/C5定制页；H footer含404 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F6/AC-3 · PRD L312 | P2新增/交付段 | M3/M6 · V-6 | M3/M6 | S 页图/七slot；T registered_generation/C5定制页；H footer含404 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F7/AC-1 · PRD L334 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F7/AC-2 · PRD L335 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F7/AC-3 · PRD L336 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F7/AC-4 · PRD L337 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F7/AC-5 · PRD L338 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F8/AC-1 · PRD L361 | P1条件的P2跨期终验/回归 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F8/AC-2 · PRD L362 | P1条件的P2跨期终验/回归 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F8/AC-3 · PRD L363 | P1条件的P2跨期终验/回归 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F8/AC-4 · PRD L364 | P1条件的P2跨期终验/回归 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F8/AC-5 · PRD L365 | P2新增/交付段 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F8/AC-6 · PRD L366 | P2新增/交付段 | M3/M5/M6 · V-7/V-11 | M3/M5/M6 | S ThemeDocument/候选/字体；T C5模式；H M3导入主题/新类 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F9/AC-1 · PRD L389 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-2 · PRD L390 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-3 · PRD L391 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-4 · PRD L392 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-5 · PRD L393 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-6 · PRD L394 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F9/AC-7 · PRD L395 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F10/AC-1 · PRD L417 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F10/AC-2 · PRD L418 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F10/AC-3 · PRD L419 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F10/AC-4 · PRD L420 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F11/AC-1 · PRD L442 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F11/AC-2 · PRD L443 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F11/AC-3 · PRD L444 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F11/AC-4 · PRD L445 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F12/AC-1 · PRD L468 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F12/AC-2 · PRD L469 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F12/AC-3 · PRD L470 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F12/AC-4 · PRD L471 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F13/AC-1 · PRD L493 | P1条件的P2跨期终验/回归 | M4/M6 · V-9 | M4/M6 | S 输入指纹/SSE/last-good；H C5 dev4/4及原生3/3 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F13/AC-2 · PRD L494 | P1条件的P2跨期终验/回归 | M4/M6 · V-9 | M4/M6 | S 输入指纹/SSE/last-good；H C5 dev4/4及原生3/3 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F13/AC-3 · PRD L495 | P1条件的P2跨期终验/回归 | M4/M6 · V-9 | M4/M6 | S 输入指纹/SSE/last-good；H C5 dev4/4及原生3/3 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F13/AC-4 · PRD L496 | P2新增/交付段 | M4/M6 · V-9 | M4/M6 | S 输入指纹/SSE/last-good；H C5 dev4/4及原生3/3 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F14/AC-1 · PRD L516 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F14/AC-2 · PRD L517 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F14/AC-3 · PRD L518 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F14/AC-4 · PRD L519 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F15/AC-1 · PRD L542 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F15/AC-2 · PRD L543 | P1已交付格式的P2嵌入增量 | M2/M6 · V-3/V-4 | M2/M6 | T 嵌入diagnostic_json有字段但suggestion为空 | 部分满足 | G-01 → gaps M1/M2 |
| WORK-F15/AC-3 · PRD L544 | P2新增/交付段 | M2/M6 · V-3/V-4 | M2/M6 | S 冷态预检+两目标；H C5 components2/2；T 诊断反例 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F15/AC-4 · PRD L545 | P3非本期新增；仅记录去向 | P3后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P3原范围/终验追踪 |
| WORK-F15/AC-5 · PRD L546 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F16/AC-1 · PRD L570 | P2新增/交付段 | M5/M6 · V-10/V-11 | M5/M6 | S 12文件白名单/同源README/CATALOG；T C5 12+24/源码/零Makepad请求；H 同源修改dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F16/AC-2 · PRD L571 | P2新增/交付段 | M5/M6 · V-10/V-11 | M5/M6 | S 12文件白名单/同源README/CATALOG；T C5 12+24/源码/零Makepad请求；H 同源修改dev | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-F16/AC-3 · PRD L572 | D16已批准移P4 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | G-04/D-03 → 原M7/V-15，P4承接 |
| WORK-F16/AC-4 · PRD L573 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-F17/AC-1 · PRD L592 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F17/AC-2 · PRD L593 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F17/AC-3 · PRD L594 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F17/AC-4 · PRD L595 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-F17/AC-5 · PRD L596 | P1非本期新增；仅记录去向 | P1原计划 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P1原范围/终验追踪 |
| WORK-N1/AC-1 · PRD L622 | P2当期NFR复核 | M6 · V-12 | M6 | H C4 A/B五冷样本与静态预算；T C5内容页0wasm/静态体积 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N1/AC-2 · PRD L623 | P2当期NFR复核 | M6 · V-12 | M6 | H C3/C4纯静态最大页LCP/CLS；U P2演示类型Slow4G样本 | 部分满足 | U-02 → 原M6/V-12 |
| WORK-N1/AC-3 · PRD L624 | P2当期NFR复核 | M6 · V-12 | M6 | T C5 14静态HTML最大gzip9总35585B；H B纯内容原预算 | 已满足 | 原M6保留完整A/B复核 |
| WORK-N1/AC-4 · PRD L625 | P2新增/交付段 | M6 · V-12 | M6 | H C4 A最大227ms/B352.4ms、位移0；U C5正式release A/B | 未核实 | U-02 → 原M6/V-12 |
| WORK-N1/AC-5 · PRD L626 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-N2/AC-1 · PRD L646 | P1条件的P2跨期终验/回归 | M4/M6 · V-12 | M4/M6 | H C4保存/全构建与C5编译后五样本；S 纯内容不编译Rust | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N2/AC-2 · PRD L647 | P1条件的P2跨期终验/回归 | M4/M6 · V-12 | M4/M6 | H C4 new→首页398ms为单轮；U 同条件五样本合同 | 未核实 | U-02 → 原M6/V-12 |
| WORK-N2/AC-3 · PRD L648 | P1条件的P2跨期终验/回归 | M4/M6 · V-12 | M4/M6 | H C4保存/全构建与C5编译后五样本；S 纯内容不编译Rust | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N2/AC-4 · PRD L649 | P2新增/交付段 | M4/M6 · V-12 | M4/M6 | H C5正式五样本中位1163.4ms；起于两目标增量编译完成 | 已满足 | 本期性能交付段已满足，非保存全程≤2s |
| WORK-N3/AC-1 · PRD L669 | P2当期NFR复核 | M6/M7 · V-11/V-15 | M6 | T C5子路径/header CSP/404；H C4 meta矩阵；U 稳定版/设备 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N3/AC-2 · PRD L670 | P2当期NFR复核 | M6/M7 · V-11/V-15 | M6 | T C5子路径/header CSP/404；H C4 meta矩阵；U 稳定版/设备 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N3/AC-3 · PRD L671 | P2当期NFR复核 | M6/M7 · V-11/V-15 | M6 | T C5子路径/header CSP/404；H C4 meta矩阵；U 稳定版/设备 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N3/AC-4 · PRD L672 | P2当期NFR复核 | M6/M7 · V-11/V-15 | M6 | T C5子路径/header CSP/404；H C4 meta矩阵；U 稳定版/设备 | 部分满足 | G-02/D-01：Firefox P4；U-01其他缺证 |
| WORK-N3/AC-5 · PRD L673 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-N3/AC-6 · PRD L674 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-N4/AC-1 · PRD L690 | P2当期NFR复核 | M4/M5/M6 · V-8/V-10/V-14 | M4/M5/M6 | S feature/图边界；H C5配对图/离线安装；T C5离线构建、零Makepad请求 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N4/AC-2 · PRD L691 | P2当期NFR复核 | M4/M5/M6 · V-8/V-10/V-14 | M4/M5/M6 | S feature/图边界；H C5配对图/离线安装；T C5离线构建、零Makepad请求 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N4/AC-3 · PRD L692 | P2当期NFR复核 | M4/M5/M6 · V-8/V-10/V-14 | M4/M5/M6 | S feature/图边界；H C5配对图/离线安装；T C5离线构建、零Makepad请求 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N4/AC-4 · PRD L693 | P2当期NFR复核 | M4/M5/M6 · V-8/V-10/V-14 | M4/M5/M6 | S feature/图边界；H C5配对图/离线安装；T C5离线构建、零Makepad请求 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N5/AC-1 · PRD L709 | P2当期NFR复核 | M2/M5/M6 · V-5/V-11 | M2/M5/M6 | T 首轮axe含启动过渡瞬时失败，定向稳态6扫描无违例；不泛化 | 未核实 | G-05修后稳定帧/完整状态复验 |
| WORK-N5/AC-2 · PRD L710 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-N5/AC-3 · PRD L711 | P2当期NFR复核 | M2/M5/M6 · V-5/V-11 | M2/M5/M6 | S 语义/减少动效；T C5地标/窄屏/axe/CSSOM；H 设计；U 真环境 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N5/AC-4 · PRD L712 | P4非本期新增；仅记录去向 | P4后续分期 | 原期记录，不计本次通过 | 非本轮完整复核，不能据P2通过关闭 | 范围外 | P4原范围/终验追踪 |
| WORK-N5/AC-5 · PRD L713 | P2当期NFR复核 | M2/M5/M6 · V-5/V-11 | M2/M5/M6 | T 窄屏/布局缩放仿真通过；reduce=true仍有150ms过渡 | 部分满足 | G-05 → gaps M1/M2；原生缩放U-01 |
| WORK-N5/AC-6 · PRD L714 | P2当期NFR复核 | M2/M5/M6 · V-5/V-11 | M2/M5/M6 | T 108扫描原8瞬时serious；定向稳定通过不覆盖全部状态 | 未核实 | G-05修后完整新类型复验 → gaps V-3 |
| WORK-N6/AC-1 · PRD L728 | P2当期NFR复核 | M4/M6 · V-4/V-9/V-14 | M4/M6 | S staging/锁/输入守卫；H C5失败保全/native与C4reliability；T 诊断反例 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N6/AC-2 · PRD L729 | P2当期NFR复核 | M4/M6 · V-4/V-9/V-14 | M4/M6 | S staging/锁/输入守卫；H C5失败保全/native与C4reliability；T 诊断反例 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N6/AC-3 · PRD L730 | P2当期NFR复核 | M4/M6 · V-4/V-9/V-14 | M4/M6 | T 名称/类型错误行列齐全，修复建议仍空 | 部分满足 | G-01 → gaps M1/M2 |
| WORK-N6/AC-4 · PRD L731 | P2当期NFR复核 | M4/M6 · V-4/V-9/V-14 | M4/M6 | S staging/锁/输入守卫；H C5失败保全/native与C4reliability；T 诊断反例 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N7/AC-1 · PRD L745 | P2当期NFR复核 | M2/M3/M5/M6 · V-3/V-4/V-6/V-10 | M2/M3/M5/M6 | S 最终HTML/CSS/源码闸；T 安全集成/产物集合/同源浏览；H故障输入 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N7/AC-2 · PRD L746 | P2当期NFR复核 | M2/M3/M5/M6 · V-3/V-4/V-6/V-10 | M2/M3/M5/M6 | S 最终HTML/CSS/源码闸；T 安全集成/产物集合/同源浏览；H故障输入 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N7/AC-3 · PRD L747 | P2当期NFR复核 | M2/M3/M5/M6 · V-3/V-4/V-6/V-10 | M2/M3/M5/M6 | S 最终HTML/CSS/源码闸；T 安全集成/产物集合/同源浏览；H故障输入 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |
| WORK-N7/AC-4 · PRD L748 | P2当期NFR复核 | M2/M3/M5/M6 · V-3/V-4/V-6/V-10 | M2/M3/M5/M6 | S 最终HTML/CSS/源码闸；T 安全集成/产物集合/同源浏览；H故障输入 | 已满足 | 本期边界；跨期终验遵PRD§7.3 |


### 连续用户旅程与依赖

- **作者 UAT-3**：H C4 同一工程七阶段：原样模板→新增demo→定制页→footer/deep404→坏Rust保留旧树/产物→修复→check/build；[原报告](../validation/docs-ssg-p2/m6/uat-author-continuous-c4-r1/report.json) `passed=true`。C5 的新两图/同名包修复另有真实 dev4/4、native2+3 和 R2 复审，不将各局部通过虚构成“C5本人又走过完整七阶段”。
- **读者 UAT-4/UAT-9**：源码/导航在 wasm 外，输入闸与所有失效块由 JS 管理；T C5 官方所有演示无脚本原图/源码，H M2/M3/C4 故障注入；G-05 是新增动效边界缺口，不否认现有故障隔离实现。
- **UAT-10**：冷态未注册名与编译错误可以同次报告，位置与JSON拒绝语义已交付；G-01 是同一路径仍缺可操作提示，不把整个检查器判未实现。
- **官方 UAT-11**：T C5 37页面加404的子路径产物、12指南与24分类可达/实际输入/同源源码；S 白名单先于读取和监视，H M5真实README/指南修改；P3语言、P4搜索/AI与发布不在本期遗漏集合。
- **UAT-12**：T 320/375px及200%布局/文字仿真通过；仿真不当作实体手机或原生缩放/VoiceOver通过；减少动效实际失败见 G-05。
- **UAT-15**：D15本人C4确认保留，不恢复旧研究门。
- **DEP-1–DEP-5**：默认DOM/模式分图、真实SSR+hydrate、内置样式编译和外部启动已接线；S 确定初态/显式client-only边界符合已记录的SSG决定，真实异步在浏览器运行；未把占位计作官方24演示覆盖。DEP-6原生锚点演示已明确 `_self`；DEP-7语言能力留P3；DEP-8 git时间按环境省略；DEP-9公共托管由原M7/P4验证。

## 4. Triage 与决策账本

本次不产生新的待决卡；三项已有明确决定直接沿用。来源是仓内已记录的用户确认，不声称在当前会话重新获得了这些回答。

| 决策 ID | G-ID | 用户选择与理由 | 确认日期/来源 | 新验收边界 | 去向 |
| --- | --- | --- | --- | --- | --- |
| D-01 | G-02 | Firefox稳定版与引擎延期P4，P2保留未验证 | 2026-10-13；原计划C-6/§11用户决定 | 不豁免其他P2设备；P4完整矩阵 | gaps V-4同步PRD，验证留P4 |
| D-02 | G-03 | 开发阶段由本人验收，不另招五人或设限时门 | 2026-10-14；PRD D15及其事件；M6本人确认 | 新建/改页/同源交互三个成果；确认属于C4 | 无新任务；引导/产品行为改变时重验相应成果 |
| D-03 | G-04 | 先P3/P4能力，公开发布留P4收口 | 2026-10-14；PRD D16及其事件 | 本地开发不等待公开安装/Pages或既有环境补验；不把延期记通过 | 原M7/V-15 |

## 5. 待核实项与审查基线

| ID | 缺失事实/材料及影响 | 补证方式 | 去向/是否阻塞新计划 |
| --- | --- | --- | --- |
| U-01 | WORK-N3/AC-4 的P2最新稳定Chrome/可见真实Safari与实体手机完整证据；VoiceOver与原生200%缩放也未由本次Chromium仿真证明 | 原M6/V-11在可用原生渠道记录版本、入口、当期交互、CSP/请求；当前支持人员/设备未变不重复催问；Firefox另按D-01 | 原M6保留，发布时完整矩阵必须补；D16明确不阻塞后续本地开发；不阻塞诊断/Docs CSS增量 |
| U-02 | N1/AC-4当前C5正式A/B release五冷样本未重测且未提供完整等价证明；N1/AC-2现有Slow4G报告只列静态类型，尚缺P2演示类型；N2/AC-2的C4 398ms文件是单轮，未满足五次取中位 | 原M6/V-12按原PRD条件补当前候选或真实字节/代码等价证明，加入每类最大演示页的移动端LCP/CLS，content new独立测五轮；不把单轮、dev指标或普通功能时长替代 | 已被原M6明确承接；这是补证，不先判预算失败，不重复建验证计划 |
| U-03 | WORK-F4/AC-4 当前组件围栏的真实GitHub文件预览未取得；静态合法围栏只能证明写法，不代替该平台预览 | 原M7/V-15在当时可访问固定来源上检查原Markdown位置，其余正文与占位；记录源版本/URL/截图 | 沿原计划登记；保留P2 AC未核实，不将D16解释为已经通过或删除AC |

当前N5完整状态对比度/axe在G-05修后由gaps V-3承接，属于有确定范围的验证工作，不另创造未知修复。两原计划未完登记：**K-01=M6既有验收缺证/上述U项；K-02=M7公开分发/Pages/回滚，D16后移P4**。

| 来源/本次验证 | 版本/结果与边界 |
| --- | --- |
| PRD | sha256 `12f83716ec2b33546cec8706b4769d1798201fd7e0b919eb1e25794ffc7c5075`，当前含D15/D16；未改原文 |
| 原计划 v2 | sha256 `0e2b26d03aa30b2a8cc8bf6baa80eb21b0d043a281e8953e4a513ba3c29b921f`，5/7；未改原文 |
| M1–M6 / R2 | 完整来源hash保存在 [baseline.json](../validation/docs-ssg-p2/prd-dev-gaps/baseline.json)；R2只关闭原3代码问题，不证明本报告新发现已解决 |
| 本次Rust | `mbx test --locked -p rustify-docs`：192单元+14白名单+17生成+6README+16注册生成+1清单+19registry=265，0失败/忽略；[日志](../validation/docs-ssg-p2/prd-dev-gaps/docs-test.log) |
| 本次浏览器/产物 | 已安装C5、R5，Playwright Chromium；[命令/身份](../validation/docs-ssg-p2/prd-dev-gaps/browser-command.json)，official-subpath46/46、0skip/flaky，产物base=/rustify-ui/、build_id=`b9267374bcc32cc6`。该组不包含九图capture case，不声称新增视觉终审；console/CSP/pageerror/非预期请求断言通过 |
| 本次体积 | C5的14个无岛HTML首载HTML/CSS/脚本gzip9总量最大35585B；[静态计算](../validation/docs-ssg-p2/prd-dev-gaps/static-weight.json)，不覆盖图片/字体/演示资源或B的新候选 |
| 本次反例 | [warm成功→名称/类型失败](../validation/docs-ssg-p2/prd-dev-gaps/component-probe.json)；两目标均可编译。最初冷反例还出现build-input变化，原[日志](../validation/docs-ssg-p2/prd-dev-gaps/diagnostic-probe.json)保留；最终归因使用无该附加错误的warm反例 |
| 本次N5/设计支持 | [108扫描](../validation/docs-ssg-p2/prd-dev-gaps/accessibility.json)、[6定向确认](../validation/docs-ssg-p2/prd-dev-gaps/contrast-confirmation.json)、[reduce CSSOM](../validation/docs-ssg-p2/prd-dev-gaps/reduced-motion.json)。impeccable context按已有Read表面加载，detector为0反模式+4 advisory；字号提示对照DESIGN.docs-ssg已记录角色，不当作新的配色/风格缺陷。检测/截图不能证明设备或全部NFR通过 |
| 运行范围 | 本机macOS arm64；未安装工具、改变浏览器设置、触碰外部服务或公开安装/部署。首轮浏览器server未起：既有测试缓存worktree登记指向空目录；仅清除该空测试登记后原入口通过，保留[r1](../validation/docs-ssg-p2/prd-dev-gaps/official-subpath-r1.log)及[定向处理](../validation/docs-ssg-p2/prd-dev-gaps/browser-stale-worktree.json)；不记作产品功能失败 |

最终保护范围及清理检查见 [scope-end.json](../validation/docs-ssg-p2/prd-dev-gaps/scope-end.json)：原输入和523个受保护产品文件需保持相同，HEAD需不变。本次补齐计划尚未实施，不能用这些审查测试冒作其未来绿测。

## 6. Follow-up 与 gaps plan

已核对 [P1-gaps](DOCS-SSG-P1-gaps.md)及P2原计划的实际范围：前者不承接P2新嵌入与岛UI，后者没有登记这两项已确认遗漏的修复，也没有具体Firefox文档同步任务。因此只规划下列增量，不重做已完成里程碑。

| G-ID/D-ID/PRD | 确定增量 | 验收条件 | 依赖/阻塞 | 真实去向 |
| --- | --- | --- | --- | --- |
| G-01；F4/AC-3、F15诊断、N6/AC-3 | 当前嵌入错误的权威候选与可执行建议 | check/build/dev对应位置、候选、类型建议一致；失败保全/独立错误收集仍成立 | 无新产品决策；复用现有schema/管线 | [gaps M1/M2](DOCS-SSG-P2-gaps.md)，V-1/V-3 |
| G-05；N5/AC-5及受影响AC-1/AC-6 | Docs减少动效真实生效及稳定帧AA复验 | reduce下接管/hover/选择无过渡；正常偏好/主题/布局/故障不回退；默认新类型稳定状态重大/严重违规0 | C5本地环境可用；不依赖真机/发布 | [gaps M1/M2](DOCS-SSG-P2-gaps.md)，V-2/V-3 |
| G-02/D-01；N3/AC-4、Q3 | PRD同步已批准P2 Firefox例外 | P1/D-04历史不改；P2明确未验证/P4恢复，其他设备不被豁免 | 决定已有明确来源 | [gaps M1](DOCS-SSG-P2-gaps.md)，V-4 |
| U-01/U-02/U-03、K-01/K-02 | 既有完整验收及后移公开发布 | 按原M6/M7/V-11/V-12/V-15及PRD实际补证 | 设备/当前版本与发布条件 | 原计划承接，排除出gaps增量 |
| G-03/D-02、G-04/D-03 | 已确认变更保持 | 保留本人结论/阶段边界；不伪造后续通过 | 无新增任务 | 不进新计划 |

已实际使用 dev-plan 的写计划流程与骨架生成 [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md)，**Ready、0/2、尚未开始**；范围为上面三项确定增量，独立于原计划。没有自动修复代码/PRD、创建issue、发布或启动实施。

下一步：`$review-dev-plan docs/plan/DOCS-SSG-P2-gaps.md`。补齐并通过代码验收后，可用 `$review-prd-dev-gaps docs/plan/DOCS-SSG-P2.md docs/plan/DOCS-SSG-P2.prd-dev-gaps.md 复审` 核对本报告；原期未决证据仍按其解除条件补齐。
