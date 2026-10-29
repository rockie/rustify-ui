# DOM-COMPONENTS-P1 · PRD 与交付差异核查

> 审查日期：2026-10-29（Asia/Shanghai）。完整首次核查，范围为 [PRD v0.2](../PRD-DOM-COMPONENTS.md) §7.2 的 **P1：16 条需求、30 条 AC**；不是此前代码或 UI 报告的修复复审。
> 代码：main / `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；工作树 dirty，产品源码无未提交改动。PRD、部分评审和台账来自未提交工作树，不能仅凭 HEAD 重建全部输入。
> 结论：**存在明确遗漏，P1 尚不能记为 PRD 全覆盖。A 2 / B 0 / C 0；待决 0；待核实 1（历史可选性能归因，不阻塞补齐计划）。**

## 1. 结论与边界

颜色、配置与显式覆盖、两模式来源、基础按钮/布局/排版、历史和本地迁移均有实现与对应证据。本次逐行核查得到 **28 条 AC 已满足、2 条部分满足**；“已满足”限定于下表注明的代码、当前实测或历史证据范围，不表示本次重跑了整期 V1–V10，也不表示已公开发布。

两项遗漏都在接入交付：默认 JSON 导出没有版本/外部字体接入说明，公开说明又混有旧参数和不完整调用。另有三个旧存储回归用例仍断言已调整的提示词；实际数据保护和重试经专项验证成立，但原测试仍失败，必须同步测试，不能把原运行改记为通过。

[原计划](DOM-COMPONENTS-P1.md#完成记录)自报 7/7，各里程碑记录和提交存在；但 M6/M7 的 V6/V7/V10 通过声明没有覆盖 G-01/G-02 的反例。因此“已完成计划”的输入前提仅在历史登记层成立，不能据此推导全部 P1 验收条件已成立。两项均属原 P1 范围，没有新增产品要求。原 PRD、计划、记录与代码在本次保持只读。

后续：独立 [DOM-COMPONENTS-P1-gaps](DOM-COMPONENTS-P1-gaps.md)，**Ready，实施 0/2**，承接两个 G-ID 和一项验证同步任务 V-01；没有查到已承接这些事项的 DOM-COMPONENTS gaps plan。到计划交付为止，不开始修复。

## 2. 三类差异

### A · 明确遗漏（2）

#### G-01 · 默认 JSON 导出缺少消费者版本与外部字体说明 · A

- **PRD 期望**：[WORK-F20/AC-3](../PRD-DOM-COMPONENTS.md#work-f20-无损交换主题并在独立应用中消费)要求独立消费，并在导出标明所需 SDK/CLI 版本与外部字体资源。原计划 §5.8、M6/V6/V8 承接该要求。
- **实际交付**：从 Studio 正常入口打开 Export，默认选择 Theme JSON v2，能直接复制/下载 `theme.json`。此状态只有格式、代码和操作按钮，没有源码修订、公开 0.2.1 不支持 v2 的提示、字体路径或许可说明。仅切换到 Rust integration 后才显示源码修订与版本限制；CSS 导出提供字体 URL，但 JSON 作者必须自己发现另一输出选项。
- **证据链**：[M6](DOM-COMPONENTS-P1.records/M6.md#交付)声称交付完整导出与版本接入；`examples/theme-studio/src/data_tools.rs::export_artifact` 的 JSON 分支直接调用 `export_json`，导出 UI 的修订与 SDK/CLI 说明位于 `export_kind == "rust"` 条件内。当前真实浏览器的 [记录](../validation/dom-components-p1/prd-dev-gaps/browser-probe.json) `export-json` 与 [默认导出实拍](../validation/dom-components-p1/prd-dev-gaps/default-json-export.png)确认缺口；同记录 `export-rust` 确认替代信息只在 Rust 分支，不是全站完全没有说明。
- **影响**：仅下载 JSON 的外部开发者拿不到必需接入前提，可能误用同为 0.2.1 的 registry 包；字体名称也不能替代文件路径和许可。JSON 本身的 v2 往返与独立消费能力已交付，此处不判为格式失效。
- **分类依据**：要求明确，信息已有可靠来源，补充常驻说明和可携带接入说明不改变产品或格式承诺，无需产品 Triage。
- **处置 / follow-up**：所有导出格式均可看到适用的源码/版本与资源要求，提供可随导出带走的接入说明；完整 JSON 保持合法、可往返，不往作者 token 或旧 schema 塞元数据。对应 gaps M1、V-1/V-3。

#### G-02 · 当前公开接入说明含旧 ThemeBoundary 契约与不完整调用 · A

- **PRD 期望**：WORK-N5/AC-1 要求目录、示例、文档的交付声明一致；WORK-F20/AC-3 的独立接入依赖正确说明。原计划 §5.9、M6/V7 和 M7 角色验收均承接。
- **实际交付**：`docs/themes.md` 的 “Root and local providers” 已正确说明 `ThemeBoundaryPatch`，后面的 “Rust snippet” 段却仍说 `ThemeBoundary` 接收 `Signal<ThemeValuePatch>`、始终保持外层模式并用 `ResolvedTheme::patched` 校验；当前组件接收 `Signal<ThemeBoundaryPatch>`，能指定局部 appearance，诊断指向 `with_boundary`。同页还称旧简版片段是 Studio 导出的确切片段。另在 Rust 导出页，可见提示为 `mount_exported_preview(ThemeMode::Light, &font_context(16., false))`，实际导出函数还需要前置的 `container` 和 `MountConfig` 两个参数。
- **证据链**：[M4](DOM-COMPONENTS-P1.records/M4.md#交付)、[M6](DOM-COMPONENTS-P1.records/M6.md#交付)记载新入口及说明；对照 [主题指南](../themes.md#root-and-local-providers)、[后段旧说明](../themes.md#L267)、[当前 provider](../../crates/rustify-dom/src/theme/provider.rs#L186)和 `data_tools.rs::rust_snippet` / 导出提示。当前 [浏览器记录](../validation/dom-components-p1/prd-dev-gaps/browser-probe.json) `export-rust` 同时保存了两参数提示和四参数签名；[实拍](../validation/dom-components-p1/prd-dev-gaps/rust-export-guidance.png)可复核。
- **影响**：消费者沿不同段落会得到互相冲突的类型、模式语义和校验入口；照抄屏内调用无法直接调用已导出的函数。真实独立 fixture 编译原片段的历史通过，不证明这条屏内提示正确。
- **分类依据**：实际新行为满足预期，无已批准决定要求保留旧说明；同步文档与调用示例属于明确遗漏，不需要重新决定公共 API。
- **处置 / follow-up**：按现有实际 API 修正指南与导出提示，明确模式无关旧 patch 的 `.into()` 适配，区分普通 provider 与显式 preview；指南若声称片段完全一致，就须同步确切片段，否则明确它是简例并链接完整入口。对应 gaps M1、V-2/V-3。

### B · 合理变更（0）

没有需要作为 B 登记的产品交付差异。source-only 交付、Custom 紧凑 v2、Rust 类型命名及 P3 静态资源边界均落在 PRD 明确允许的实现/分期范围，不为增加数量而登记成差异。旧颜色负载的成本按原记录保留，也不等于已批准产品降级。

### C · 需 Triage 的实现差异（0）

无。现有证据足以确定上述最小补齐范围，无需用户重新选择是否满足已写明的要求。

## 3. 完整需求覆盖账本

证据等级：**T**＝本次实际验证；**S**＝本次代码/文档核对；**H**＝已有测试/运行记录，保留其原版本；**D**＝仅完成声明。下表没有仅靠 D 判定已满足的行。M1–M7 均链接到原计划同名记录；本次命令及结果见 [验证记录](../validation/dom-components-p1/prd-dev-gaps/checks.md)。

### 3.1 P1 的 30 条 AC（逐条）

| PRD 需求/验收及定位（§3、§7.2） | 本期范围/批准变更 | 计划/里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F1/AC-1 | Colors 独立静态消费 | §5.1 / M1、M7 | [M1](DOM-COMPONENTS-P1.records/M1.md)、[M7](DOM-COMPONENTS-P1.records/M7.md) V1 | S：无依赖数据 crate、包内 CSS；T：Colors 全量生成检查；H：禁 JS 页面/无 wasm 请求 | 已满足 | AC-2→P2、AC-3→P4 |
| WORK-F2/AC-1 | 31 色族、全部模式/alpha/色域及黑白 | §5.1 / M1 | [M1](DOM-COMPONENTS-P1.records/M1.md) V1 | S：vendor/生成器/API；T：256 产物确定性检查、P3 单测与文档例；H：参考指纹逐项校准 | 已满足 | 本条终验 P1 |
| WORK-F2/AC-2 | 显式按需资源与 sRGB 回退 | §5.1 / M1、M7 | [M1](DOM-COMPONENTS-P1.records/M1.md) V1 | S/T：非默认 Rust P3、CSS 相对 import 与双门；H：sRGB/P3 网络及 emulation gate | 已满足 | 不宣称显示器色准或 GPU P3 |
| WORK-F3/AC-1 | 六类配置与 auto 结果 | §5.2 / M2、M4–M6 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M5](DOM-COMPONENTS-P1.records/M5.md)、[M6](DOM-COMPONENTS-P1.records/M6.md) | T：主题内核、Studio 六配置/ThemePanel；H：独立导出两文档/两模式实际值 | 已满足 | 本条终验 P1 |
| WORK-F3/AC-2 | 根/局部 appearance 与恢复继承 | §3.1、§5.4 / M2–M3 | [M3](DOM-COMPONENTS-P1.records/M3.md) V3 | S：完整 ThemeSource/provider；T：两根、相反模式、局部层、移除/无效 patch 回归 | 已满足 | 文档偏差另见 G-02 |
| WORK-F3/AC-3 | full 必要形状、单次缩放 | §5.3 / M2、M4 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M4](DOM-COMPONENTS-P1.records/M4.md) | T：全部配置/尺度内核及 Button/布局浏览器；H：Checkbox/Radio/Switch 保形与 GPU 共享几何 | 已满足 | 新控件契约仍留 P2 |
| WORK-F4/AC-1 | 配置不删作者覆盖 | §5.2、§5.7 / M2、M5 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M5](DOM-COMPONENTS-P1.records/M5.md) | T：组责任键、精确解除、切模式、undo/redo；S：compose 优先级 | 已满足 | 本条终验 P1 |
| WORK-F4/AC-2 | 两 scope/嵌套/宿主/浮层隔离 | §5.4 / M3 | [M3](DOM-COMPONENTS-P1.records/M3.md) V3 | T：theme-compat 三项，父更新/解除与 host sentinel；H：Studio DOM/GPU 作用域 | 已满足 | 不扩为后期全浮层验收 |
| WORK-F4/AC-3 | 分别解除字体/度量/阴影/颜色；unit/gap | §5.2–5.3 / M2、M5 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M5](DOM-COMPONENTS-P1.records/M5.md) | T：组/单 token 清除、共同字段与单次解析；S：双模式 patch、间距分离 | 已满足 | 本条终验 P1 |
| WORK-F15/AC-1 | Button/IconButton/Link | §5.5 / M4 | [M4](DOM-COMPONENTS-P1.records/M4.md) V4 | T：七变体/尺寸/原生 submit/reset、loading 拒绝、命名 icon、Link 与局部视觉；H：R8 UI 矩阵 | 已满足 | AC-2/3/4→P2/P3/P4 |
| WORK-F16/AC-1 | 七布局、八排版、HTML 语义 | §5.6 / M4 | [M4](DOM-COMPONENTS-P1.records/M4.md) V4 | T：组件 SSR 169、浏览器原生标签/文本/Reset 事件；S：公开入口 | 已满足 | 本条终验 P1 |
| WORK-F16/AC-2 | 六断点、长文/链接、局部 Reset | §5.6 / M4 | [M4](DOM-COMPONENTS-P1.records/M4.md)及 R2 | T：断点上下界、长文/截断、caller utilities、Inset 与宿主不变 | 已满足 | R1 单边误报已撤回，不沿用旧 oracle |
| WORK-F18/AC-1 | 44 预设、同源配置/精调、中性壳 | §5.7、§6 / M5 | [M5](DOM-COMPONENTS-P1.records/M5.md)、[M7](DOM-COMPONENTS-P1.records/M7.md) | T：69 模型、44 预设与六配置 UI/覆盖计数；H：R8 中性壳/错误矩阵 | 已满足 | 本条终验 P1 |
| WORK-F18/AC-2 | 共同字段双模式、导入差异/预览独立 | §5.7 / M2、M5 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M5](DOM-COMPONENTS-P1.records/M5.md) | T：模式选择不改文档/历史，单 token/共同 token 解除；S/H：导入两模式作者值 | 已满足 | 本条终验 P1 |
| WORK-F20/AC-1 | v1 升级、v2 意图/未知字符串往返 | §3.1、§5.8 / M2、M6 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M6](DOM-COMPONENTS-P1.records/M6.md) | T：严格 codec/author tests、真实下载 v2/v1；S：Custom 迁移与 serde 版本分派 | 已满足 | 不声称 registry 0.2.1 支持 |
| WORK-F20/AC-2 | 版本/diff/review、取消/非法/超限不应用 | §5.8 / M5 | [M5](DOM-COMPONENTS-P1.records/M5.md) V6 | T：JSON/CSS 候选、stale Apply、恶意 URL/字体、大小边界；当前主题保留 | 已满足 | 本条终验 P1 |
| WORK-F20/AC-3 | 独立消费且导出标明版本/字体 | §5.8 / M6–M7 | [M6](DOM-COMPONENTS-P1.records/M6.md)、[M7](DOM-COMPONENTS-P1.records/M7.md#最终源码配对) | H：独立 DOM/CSS/Rust 根/子路径与 source Docs；T/S：默认 JSON 缺说明，调用提示不完整 | **部分满足** | **G-01、G-02 → gaps** |
| WORK-F20/AC-4 | 兼容损失先展示、CSS 语义、真实反馈 | §5.8 / M6 | [M6](DOM-COMPONENTS-P1.records/M6.md) V6 | T：loss 字段、实际下载/剪贴板字节、复制延迟与拒绝；S：CSS 双模式/资源输出 | 已满足 | 不保证有损 v1 恢复意图 |
| WORK-F21/AC-1 | 整次历史、手势/取消、原生 undo | §5.7 / M5 | [M5](DOM-COMPONENTS-P1.records/M5.md) V5 | T：配置/覆盖独立历史、no-op、手势后配置、原生草稿 undo；H：取消回滚 | 已满足 | 本条终验 P1 |
| WORK-F21/AC-2 | 原库迁移/原文、坏缓存、写失败可导出 | §3.3 / M5、M7 | [M5](DOM-COMPONENTS-P1.records/M5.md)、[M7](DOM-COMPONENTS-P1.records/M7.md) | T：坏库下载、取消/刷新升级、队列与无锁；专项 quota/denied 保留草稿和原 bytes；H：备份全量/旧站回滚 | 已满足 | V-01 同步旧断言；不是数据丢失 |
| WORK-F21/AC-3 | 命名动作/跨标签显式冲突 | §3.3 / M5 | [M5](DOM-COMPONENTS-P1.records/M5.md) V6 | T：save/reload/copy/rename/delete，Load/Keep/copy 第三写与队列；专项命名失败重试成功 | 已满足 | V-01 保留原失败运行 |
| WORK-F23/AC-1 | 可用/部分/未交付与入口/路径 | §5.9 / M6 | [M6](DOM-COMPONENTS-P1.records/M6.md) V7 | S/T：59 条目、97 参考名、24 分类分列，catalog 无漂移；Primitives/后期能力标明边界 | 已满足 | AC-2/3→P5；不把分类数当全量组件数 |
| WORK-N2/AC-1 | 每帧一次/最终值/idle/不重建 | §5.4、§7 / M7 | [M7](DOM-COMPONENTS-P1.records/M7.md#性能与体积) V9 | S：发布/投影链与缓存；H：240 inputs→30 publications、final/drawn、对象身份和 5s idle；本次非性能运行 | 已满足 | AC-2→P5；U-01 可选归因保留 |
| WORK-N3/AC-1 | strict CSP、局部样式/无宿主污染 | §5.1、§5.4 / M3、M7 | [M3](DOM-COMPONENTS-P1.records/M3.md)、[M7](DOM-COMPONENTS-P1.records/M7.md) | T：Catalog CSP/host/CSSOM 清理与导出专项零 CSP；S：显式 Colors 根/组件层 | 已满足 | AC-3→P4 |
| WORK-N3/AC-2 | 导入不执行、不绕过校验/不应用非法值 | §5.8 / M2、M5 | [M2](DOM-COMPONENTS-P1.records/M2.md)、[M5](DOM-COMPONENTS-P1.records/M5.md) | T：恶意 CSS/字体/版本拒绝、零攻击 URL 请求、保留当前文档；S：数据解析器 | 已满足 | AC-3→P4 |
| WORK-N5/AC-1 | 目录/示例/文档声明一致 | §5.9 / M6 | [M6](DOM-COMPONENTS-P1.records/M6.md) V7 | T：catalog 同源且无漂移；S/T：指南/导出提示仍与实际 API 不同 | **部分满足** | **G-02 → gaps** |
| WORK-N6/AC-1 | 失败真实反馈、有效主题和原数据仍在 | §3.3、§8 / M5–M6 | [M5](DOM-COMPONENTS-P1.records/M5.md)、[M6](DOM-COMPONENTS-P1.records/M6.md) | T：导入/复制失败；专项 quota、命名失败/重试与 denied 均符合语义；H：全库备份/迁移失败矩阵 | 已满足 | V-01 同步断言；AC-2→P4 |
| WORK-N7/AC-1 | 中性壳、基础/精调/来源/保存可见 | §6 / M5、M7 | [M7](DOM-COMPONENTS-P1.records/M7.md)及 [UI R8](DOM-COMPONENTS-P1.ui-review.r8.md) | H：R8 当前同产品源码完整冻结 UI 矩阵；T：六配置与失败的常驻标题/详情/出口，实际导出截图 | 已满足 | AC-2/3→P4/P5；不作全 WCAG 声明 |
| WORK-N8/AC-1 | 来源/指纹/许可与输入对齐 | §5.1 / M1、M6 | [M1](DOM-COMPONENTS-P1.records/M1.md) V7 | T：sources verify，Colors5/Themes39 无漂移；S：vendor/许可；H：PRD 目录指纹校准 | 已满足 | 本条终验 P1 |
| WORK-N8/AC-2 | 正常构建/外部消费不依赖 ref | §4、§5.1 / M1、M6 | [M1](DOM-COMPONENTS-P1.records/M1.md)、[M6](DOM-COMPONENTS-P1.records/M6.md) | S/T：生成器读正式 vendor、Colors 包/API；H：离线归档重编译、DOM/Docs 独立消费者 | 已满足 | 本次未重做干净归档/外部构建 |

### 3.2 其余 52 条 AC 的去向

以下均为 **范围外**，不是本期遗漏，也不算已交付。以 PRD §7.2/§7.3 为依据；本次未评估这些期次的实现覆盖。

| PRD 条目 | P1 之外的 AC / 交付期 | 整条终验期 |
| --- | --- | --- |
| WORK-F1 | AC-2 P2；AC-3 P4 | P4 |
| WORK-F5 | AC-1—AC-4 P2 | P2 |
| WORK-F6 | AC-1—AC-3 P2 | P2 |
| WORK-F7 | AC-1—AC-2 P2 | P2 |
| WORK-F8 | AC-1—AC-3 P2 | P2 |
| WORK-F9 | AC-1—AC-2 P2 | P2 |
| WORK-F10 | AC-1—AC-2 P3 | P3 |
| WORK-F11 | AC-1—AC-3 P3 | P3 |
| WORK-F12 | AC-1—AC-2 P3 | P3 |
| WORK-F13 | AC-1—AC-3 P4 | P4 |
| WORK-F14 | AC-1—AC-2 P3 | P3 |
| WORK-F15 | AC-2 P2；AC-3 P3；AC-4 P4 | P4 |
| WORK-F17 | AC-1—AC-2 P3 | P3 |
| WORK-F19 | AC-1 P2；AC-2 P4；AC-3 P5 | P5 |
| WORK-F22 | AC-1—AC-2 P5 | P5 |
| WORK-F23 | AC-2—AC-3 P5 | P5 |
| WORK-N1 | AC-1 P2；AC-2 P4；AC-3 P5 | P5 |
| WORK-N2 | AC-2 P5 | P5 |
| WORK-N3 | AC-3 P4 | P4 |
| WORK-N4 | AC-1 P2；AC-2—AC-3 P5 | P5 |
| WORK-N5 | AC-2 P5 | P5 |
| WORK-N6 | AC-2 P4 | P4 |
| WORK-N7 | AC-2 P4；AC-3 P5 | P5 |

### 3.3 用户旅程 / UAT 的 P1 投影

| PRD UAT | 本次对应的子路径与结论 | 后期去向 |
| --- | --- | --- |
| UAT-1 | Colors/能力目录成立；三层完整入口并非 P1 交付 | Primitives P2、行为等价 P4 |
| UAT-2 | 配置→精调→按钮/排版可操作；导出接入说明仍有 G-01/G-02 | 后期新组件不提前记通过 |
| UAT-3 | 两模式/覆盖/v1/本地迁移取消与恢复有证据 | 保留各证据版本 |
| UAT-4 | 本轮不验收完整表单/Primitives | P2 |
| UAT-5 | 本轮范围外 | P3 |
| UAT-6 | 仅 F4/AC-2、N3/AC-1 的局部主题/作用域链成立 | 其余 P2–P4 |
| UAT-7 | 本轮范围外 | P4/P5 |
| UAT-8 | 本轮范围外 | P3 |
| UAT-9 | 仅 P1 操作/布局/排版；文档一致性受 G-02 影响 | 完整参考/Inspector 到 P4/P5 |
| UAT-10 | 历史、队列、保存失败/冲突可恢复；原三项测试失败另记 V-01；性能本次未测 | 完整性能终验 P5，U-01 保留 |
| UAT-11 | 独立 DOM/CSS/Rust/source Docs 有历史运行；版本/资源随导出交付仍缺 G-01/G-02 | 完整部署/迁移终验 P5 |
| UAT-12 | P1 N2/AC-1 及既有 GPU 局部失败/fatal 回归按 M7 证据；本次未重新注入 | F19、N3、N6 的对应后期 AC 不结算 |
| UAT-13 | P1 只核目录状态；指南现有矛盾归 G-02 | 全量旧入口迁移 P5 |
| UAT-14 | 本次核 CSP/宿主隔离；R8 的响应式证据保留 | SSR/hydrate 全组件、辅助技术和完整多宿主到 P2–P5 |

## 4. Triage 与决策账本

本次无 C，无新增 D-ID，无待用户回答的问题。G-01/G-02 按原要求补齐，不以超时或默认选项视为批准。

历史 UI 的参考对比度接受及 UI-115 接受现状，分别按原 UI 报告/目标包保留；本次不重开其产品决定，也不将其算作新增 B 或擅自扩大到全部 WCAG 条款。PRD 的 D1/D2、源模式和 GPU 非对等边界继续有效。

## 5. 验证、待核实项与基线

### 5.1 本次实际验证

| 验证 | 实际结果 | 覆盖/限制 |
| --- | --- | --- |
| `mbx test --locked -p rustify-dom --lib` | 258 passed / 0 failed | 主题 codec/config/source/尺度等 host 逻辑；不代替浏览器 |
| `mbx test --locked -p theme-studio --bins` | 69 passed / 0 failed | 模型、历史、预设、队列与存储纯逻辑 |
| 组件独立 SSR `--lib` | 169 passed / 0 failed | 公开参数/语义与静态规则；CSR/SSR 分开 |
| Colors `--features p3` | 3 单测 + 1 doc-test passed | typed 资源/API 和回退 |
| colors / sources / css / catalog 检查 | 全部 exit 0 | 256 确定性产物；59/97/24 目录；CSS 70,314 B、4,475 candidates；既有来源 drift 不是新采用输入漂移 |
| `theme-studio` 四 spec | **22 passed / 3 failed** | 六配置、导入/导出、命名与跨页；三个旧文案断言失败，详见 V-01 |
| `colors-static` 两种 profile | **2 passed / 0 failed** | 禁 JS 的 sRGB/P3 按需网络、回退、两模式/多根与宿主隔离 |
| `component-catalog` 三 spec | **18 passed / 0 failed** | Button/布局/排版、两根与边界/浮层、CSP/host/清理 |
| `theme-v2-storage` 定向 | **9 passed / 0 failed** | 取消/刷新升级、队列、quota 保留、第三写、无锁降级 |
| 导出/失败专项实际浏览器 | **5 组完成，errors/CSP 0** | G-01/G-02 反例；quota 保留原 bytes/新草稿可导出，命名失败后显式重试成功，denied 后可编辑/导出 |

完整命令和日志入口在 [checks.md](../validation/dom-components-p1/prd-dev-gaps/checks.md)；截图是当前实际页面，未修改或拼接。历史 V8 独立构建、GPU/Docs、性能和角色验收分别引用 M1–M7 的原版本，没有当作本次运行。

### 5.2 V-01 · 存储行为成立，但三个回归用例仍断言旧文案

这是**验证维护 follow-up，不是第三个产品差异 G-ID**。`theme-storage.spec.ts:86` 在 `storage-status` 查找 quota/failed；`:104` 在同段查找命名失败；`:118` 在同段查找 unavailable/refused。当前 `storage_controls.rs` 已把失败标题、草稿状态、命名操作结果拆开。新段落分别是内存草稿说明、已保存草稿说明和可内存编辑说明，导致三条超时。

本次没有改测试或产品。实际 [专项记录](../validation/dom-components-p1/prd-dev-gaps/browser-probe.json)与当前代码同时证实：quota 后原 bytes 不变，新草稿可导出；命名失败独立显示 `failed; retained for retry`，恢复权限并明确重试后保存成功；denied 仍能编辑和导出。不能据旧文本断言误报产品丢数据，也不能把这三个原失败测试标成绿。

最小后续是在真实反馈区域断言语义并继续验证原字节、草稿、重试与下载，不单纯删除失败断言；由 gaps M1/M2 承接。

### 5.3 U-01 · 历史可选性能归因仍待核实（1）

- 来源：[代码复审 R3 / V-1](DOM-COMPONENTS-P1.code-review.r3.md#2-待核实--待决策项)保留 R1 单对测量中 CSS/GPU p50 上升的归因不确定性；不是本次新发现的产品回退。
- 已知：M7 保留同条件旧负载/新配置与 idle/最终值证据，并明确旧颜色 CSS 提交 p95 从配对旧站 15.4ms 到 26.4ms；没有新增毫秒 SLO，也没有全面提速结论。
- 未知：可选 R1 p50 差异在多轮交替测量下是否稳定、原因是什么；本次没有性能复测，不能用本轮功能测试关闭它。
- 补证：若需关闭，按 R3 约定交替至少三轮同环境修前/修后测量；正式完整性能范围归 WORK-N2/AC-2 的 P5。本项不阻塞 G-01/G-02，也不纳入当前增量实施。

人工 VoiceOver、真实 OS IME、Safari/Firefox、真机与真人研究本次均未验证；原记录的缺失不改写为通过。它们在本次 P1 子集之外的整体终验不因目录/组件存在而提前完成。旧临时日志是否能跨机器取回不作为本次通过证明；原结果以仓内记录、JSON 与截图为可取回证据。

### 5.4 审查基线

- PRD：v0.2，SHA-256 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`。
- 原计划：SHA-256 `94dfaa72fdc7a7b0321ea9282d2c103b7e510feb1f9118bc53747aac394ea6ba`。完成记录 M1–M7 均已读；后续 UI 只引用与当前能力/限制相关的处置，不把历次局部复审替代全 PRD 核查。
- 输入/关键记录指纹见 [baseline.json](../validation/dom-components-p1/prd-dev-gaps/baseline.json)。PRD 及未跟踪报告当前只能从本工作树取回；报告与后续计划未提交/推送。
- macOS；mise Rust 1.99.0、mbx 1.15.0，Node 26.12.1 / Playwright 1.63.0；本次真实 Chrome **156.0.8078.48 headless**，不是实机 headed 性能验收。
- 复用本地最新 release：Studio `105da89e37304db9`、Catalog `18ca82701d674efd`，与 [UI R8 被测产品记录](DOM-COMPONENTS-P1.ui-captures/r8/tested-version.json)同 HEAD、产品源码无 dirty 差异；本次未修改 Rust/CSS，无需为审查重复构建。Rust 导出实际修订 header 为当前完整 HEAD。
- 既有未提交 `.xgent-ai` 材料、PRD/评审及两张布局图保留。Catalog 测试写到旧 `source-boundaries.png` 后已恢复开工前字节；本次证据另存新目录，未覆盖历史结论。结束前再次核对输入 hash、HEAD 与产品 diff。

## 6. Follow-up 与 gaps plan

| G-ID / 其他 / PRD ID | 确定的增量工作 | 可观察验收 | 依赖/阻塞 | 去向 |
| --- | --- | --- | --- | --- |
| G-01 / WORK-F20/AC-3 | 导出通用接入说明与可携带版本/资源资料 | 默认 JSON、CSS、Rust 均可知适用版本；离开 Studio 仍能按配套说明消费；JSON 往返不变 | 无 | gaps M1 → M2，V-1/V-3 |
| G-02 / WORK-N5/AC-1、WORK-F20/AC-3 | 同步 provider/模式/片段与实际调用说明 | 指南不再给冲突类型/模式，屏内调用包含实际签名所需参数，普通/preview 接入分别准确 | 无 | gaps M1 → M2，V-2/V-3 |
| V-01 / WORK-F21、WORK-N6 的验证维护 | 三条旧提示断言同步反馈区域，保留数据断言 | 原四 spec 全部运行通过，失败/已保存/命名重试分别准确 | 无 | gaps M1 → M2，V-3 |
| U-01 / 历史代码 R3 V-1 | 可选多轮性能归因 | 真实配对测量才能关闭，不以功能/视觉成功代替 | 非阻塞、非本增量必做 | 原 R3 / P5；不纳入 gaps |

实际计划：[DOM-COMPONENTS-P1-gaps.md](DOM-COMPONENTS-P1-gaps.md)，Ready、0/2。没有未决 C 混入计划，也没有修改原计划 7/7 的历史记录。后续开发完成并通过独立评审后，才能关闭 G-01/G-02 和重新判断两条 AC；计划 Ready 本身不代表修复已交付。

下一步：

```text
$review-dev-plan docs/plan/DOM-COMPONENTS-P1-gaps.md
```

本报告完成不代表整份 DOM-COMPONENTS PRD 交付；跨期需求终验仍以 PRD §7.2/§7.3 为准。


## 7. 状态登记

按仓内协议 v3 / state-model v2 登记独立覆盖核查对象和独立 gaps plan，关联 PRD 与原计划；见[核查状态](../../.xgent-ai/sdlc/status/review/03d811fc-1289-4de6-aba0-59837b7ca681.md)及[增量计划状态](../../.xgent-ai/sdlc/status/plan/bba16439-b67d-4fa5-bbff-fddfc2e1317a.md)。核查为 `review-recorded` / `review-pending`，表示报告已完成、两项产品遗漏尚未修复。新计划原文 Ready，规范文档状态 `dev-plan-drafted`（尚未独立评审），实施 `dev-plan-not-started`。

原计划的历史 7/7、各里程碑及 UI R8 结论均保留；当前台账按新核实的必需交付缺口将原范围的实施可完成性记为 `dev-plan-in-progress`，明确剩余工作由独立 gaps 计划承接，不虚构原里程碑被重新执行或已修复。所有新材料及台账变更未提交/推送。
