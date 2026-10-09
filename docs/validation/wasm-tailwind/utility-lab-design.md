# Utility Lab · M3 设计验收

本记录限定于 [WASM-TAILWIND 计划](../../plan/WASM-TAILWIND.md) M3 的完整类名输入、scanner-free 动态 DOM 预览与作用域隔离。它记录已实现的示例扩展，沿用 [PRODUCT.md](../../../PRODUCT.md) 与 [DESIGN.md](../../../DESIGN.md) 的中性工具语言；不修改根设计文件或 `.impeccable/design.json`，不建立新的产品身份。

## 实际界面

事实源为 [app.css](../../../examples/utility-lab/app.css)、[index.html](../../../examples/utility-lab/index.html)、[app.js](../../../examples/utility-lab/app.js) 与 [main.rs](../../../examples/utility-lab/src/main.rs)。

| 项目 | 已实现规则 |
| --- | --- |
| 色彩 | 工具壳背景 `#f9fafb`、正文 `#1d2939`、说明 `#475467`；白色输入与预览面，字段边界 `#858f9e`、区域分隔 `#d0d5dd`。主操作和焦点为 `#1570ef`，主操作 hover 为 `#175cd3`，错误文本为 `#b42318`。 |
| 字体 | 工具壳与按钮为 IBM Plex Sans。主标题 30px/600、分区标题 18px/600、字段标签 14px/600、按钮 14px/500。类名输入为 Liberation Mono 14px/1.6，CSS 输出为同字体 13px/1.6。字体来自已有 [theme-fonts.css](../../../web/theme-fonts.css)。 |
| 布局 | 主内容最大宽度 1080px，桌面 padding 为 40px 24px 64px；两预览并列、间隔 32px。640px 以下改为一列、间隔 24px，主内容 padding 为 24px 16px 40px。操作行可换行。 |
| 形状与层次 | 输入和按钮 4px 圆角；预览为白底、1px 边界、24px 内边距。工具壳无装饰阴影或动效；通过留白、细边界和字号区分层级。样本的颜色、尺寸和圆角来自输入类名。 |
| 焦点与状态 | `:focus-visible` 为 2px 蓝色 outline、3px offset。启动时三操作禁用；状态使用 `role="status"`、`aria-live="polite"`。失败、unsupported 和重试反馈保持可见。 |

界面顺序是类名输入 → Apply classes / Restart compiler / Copy CSS → 应用状态 → DOM preview 与 Independent scope → 折叠的生成 CSS 和诊断 → scope 外样本。Apply 等待 `ensure` 成功后才替换样本 class；Copy CSS 的剪贴板失败反馈提示从输出区选择复制。Independent scope 是第二个 DOM 编译作用域，不是 GPU 预览。

30px 主标题是 Utility Lab 的局部页面层级。DESIGN.md 的 Overview 明确将 Theme Studio 的字号、密度和布局限定于其工具壳，允许仓库中的其他示例不同；该标题不改变全局 `title` token。

## 浏览器证据与新鲜度

独立 reviewer 检查的是 2026-10-04 07:10 UTC 那轮 root/subpath 共12项检查的桌面/手机图。随后主线补 Worker 构造失败清理及 fatal 动作禁用，重建同一壳并逐 project 验证；本节引用最终稳定归档，避免临时 test-results 覆盖历史路径。界面 CSS、HTML、happy-path composition 未改变，原 ship 结论仍限于 M3 外观。

终验 [root JSON](utility-lab/root-browser.json) 记录9项通过、0 unexpected，起始 `2026-10-04T07:21:32.274Z`；[subpath JSON](utility-lab/subpath-browser.json) 记录4项通过。详细行为与来源身份以 [M3记录](M3.md) 为准；本设计文档没有另跑浏览器。

| 入口 | 桌面全页图 | 手机全页图 | 文件采集时间（2026-10-04 UTC） |
| --- | --- | --- | --- |
| `/` | [1280 × 826](utility-lab/root-desktop.png) | [390 × 1151](utility-lab/root-mobile.png) | 07:21:37.617229 / 07:21:37.697733 |
| `/tools/utility-lab/` | [1280 × 826](utility-lab/subpath-desktop.png) | [390 × 1151](utility-lab/subpath-mobile.png) | 07:22:52.261446 / 07:22:52.340758 |

手机测试视口390 × 844，全页高度1151是内容高度。测试在字体就绪与两帧绘制后截图并断言无横向溢出。root与subpath的 [build manifest](utility-lab/root-manifest.json) / [subpath manifest](utility-lab/subpath-manifest.json) 记录 build id `f109f79b90c6ecb3`，base分别为 `/` 与 `/tools/utility-lab/`；独立 Worker的资源SHA另见对应 utility manifest。

## Finish reviewer 与 detector 处置

已读取独立 [finish reviewer 原文](utility-lab-review.md)，结论为 **ship**，仅覆盖本次 M3 最小输入与 DOM 预览。评审按局部扩展目标与既有设计约束检查，未获得独立 QUALITY BAR 卡、comp 或持久化五块方向契约。其五段处置为：`persistence` 通过既有中性色/字体/紧凑控件的延续；`fidelity` 判定输入、提交状态、scope 对照和移动布局匹配，30px 标题为局部 adaptation；`ceiling` 达到最小 DOM fixture 目标且截图完整有效；`material_fixes` 无；`keep` 保留单一主操作、真实提交状态和作用域对照。本记录没有重新评分完整计划或 Theme Studio。

已读取 [detector 原文](utility-lab-detect.json)，它以 `design-system-font` warning 报告 textarea 使用未记录的 `mono`。源码实际声明为 `font: 14px/1.6 "Liberation Mono", monospace`，DESIGN.md 的 Typography 已记录 Liberation Mono 的技术代码用途，已有字体 CSS 也声明对应资源。reviewer 还核对构建后字体声明，归类为解析误报；本记录同意其处置，不修改字体，也不将 `mono` 提升为新的设计 token。现有 `.impeccable/review/finish-verdict.md` 与 `detector.json` 是 Theme Studio 的历史记录，不能作为这次 Utility Lab 的 reviewer 原始文件引用。

本轮未确认 incumbent 系统的实质漂移。M3 的 30px 主标题、14px 等宽输入和原生示例按钮属于局部实现，不将它们写成 SDK 或 Theme Studio 的新规范；预存系统与 sidecar 保持原样。

## 后续 demo/UI 边界

M3 的 ship 不代表全范围 Tailwind CSS 编译或完整计划交付。当前 CSS 编译仍是切片，合法但待移植的 family 必须保留明确诊断。

- GPU utility 投影、DOM/GPU 双预览、drawn revision、strict/partial 控件和 GPU context 恢复留待 M5/M6，当前没有 GPU 呈现交付声明。
- 源码输入、scanner 面板及源码扫描结果与运行时类名的对照留待后续集成；本次证明完整类名路径无需 scanner。
- base/override/条件的独立 cn 输入、主题与配置切换、配置原子失败恢复、暗色和语言切换等完整 demo 交互仍待对应里程碑；当前页面只提供已实现的英文最小工具壳。

上述边界来自计划 §6 与 M3–M7 的分期，不能用两张 DOM 预览图抵扣未来 GPU、scanner 或主题/config 验收。
