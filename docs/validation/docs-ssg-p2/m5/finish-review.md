disposition: fix

## verdict

本次是 M5 官方英文站的独立首批 finish review，范围仅截图有效性与视觉。采用 DOM 的 Read 呈现路径：静态首页、指南与分类正文沿用 P1 阅读世界，分类演示采用真实 DOM 组件；本站无 GPU 呈现需求。结论基于本次九图，独立于 M4 的反馈层评审。

设计依据为 [原 shell brief 的 “P2 extension: the English SDK site”](../../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md)、[DESIGN.docs-ssg.md](../../../../DESIGN.docs-ssg.md) 与 [PRODUCT.md](../../../../PRODUCT.md)。已读取实际 [pages.rs](../../../../site/src/pages.rs)、[collections.rs](../../../../site/src/demos/collections.rs)、[README 原介绍](../../../../README.md) 和 [截图捕获逻辑](../../../../tests/docs-ssg/official.spec.ts)。这是既有阅读世界的扩展，不适用新概念、approved comp 或新 QUALITY BAR 卡；FORM 保留 `user-pinned-docs-ssg-p1`。

截图有效性 **pass，9/9**：逐张用 `view_image` 查看，全部尺寸符合捕获逻辑；文件字节数与 SHA-256 符合 [manifest](official-subpath-r3/capture-batch/manifest.json)，并与 [results.json](official-subpath-r3/results.json) 中该 capture test 的内嵌原始浏览器 PNG 逐张一致。该捕获记录为 official-subpath、passed；manifest 记录本地 Chromium `153.0.8010.12`。

| 页面 | 桌面浅色 1280×900 | 桌面深色 1280×900 | 窄屏浅色 375×812 |
| --- | --- | --- | --- |
| 首页 | [home-light](official-subpath-r3/capture-batch/official-home-desktop-light.png) | [home-dark](official-subpath-r3/capture-batch/official-home-desktop-dark.png) | [home-narrow](official-subpath-r3/capture-batch/official-home-narrow-light.png) |
| Data table 分类 | [category-light](official-subpath-r3/capture-batch/official-category-desktop-light.png) | [category-dark](official-subpath-r3/capture-batch/official-category-desktop-dark.png) | [category-narrow](official-subpath-r3/capture-batch/official-category-narrow-light.png) |
| Quick start 指南 | [guide-light](official-subpath-r3/capture-batch/official-guide-desktop-light.png) | [guide-dark](official-subpath-r3/capture-batch/official-guide-desktop-dark.png) | [guide-narrow](official-subpath-r3/capture-batch/official-guide-narrow-light.png) |

页面、主题与文件名相符，无黑屏、整页渲染缺失或可见页面横向溢出；首页与正文标题可见。捕获代码明确回到页面顶部并使用 viewport screenshot，未宣称 full-page。分类深色图记录了一处真实的可读性缺陷，详见 M5-F1；它有完整页面和原始捕获来源，因此属于需要修复的视觉结果。

本批代表首页、Data table 和 Quick start 的首屏。其余分类/指南、下方 Capabilities 表、GPU 链接、README 后续段落及窄屏 viewport 以下的 source 不在本次目视覆盖内。源代码确认有相应内容不等于这些区域已获视觉验收。本次未运行 context loader、detector、Cargo、build 或浏览器，也未独立验证行为、性能、字体加载、实体手机、Safari 或线上部署。Root/subpath/meta 的既有测试结果不能替代本批视觉结论。

## fidelity

| 元素 | 判定 | 证据与依据 |
| --- | --- | --- |
| TYPE | match | 首页 title/tagline、正文 H1/H2、16px 阅读正文、安静的小号导航和系统 mono 源码保持 P1 的系统字体与层级；指南窄屏正文及 inline code 自然换行。没有新增 display 字体。 |
| MATERIAL | match | 既有平面表面、hairline 分隔、6px 文档 panel 和简洁描边图标；首页 features 是标题与一句说明，没有装饰卡片、图像或伪物理材质。 |
| GROUND | match | 从三类桌面 PNG 的页面空白处取样，浅色均为 `#f9fafb`，深色均为 `#0c111d`，符合 P1 OWN-WORLD。局部演示表体的主题冲突另列 M5-F1。 |
| 首页身份与入口 | match | 1280 首屏显示 “Rustify UI”、定位句、“Quick start” 与 “Components”；两入口均完整位于首屏，并保留既有 primary filled / secondary outline 处理。375px 两入口仍同排且文字完整。 |
| 首页 features 与 README | match | 三项 DOM、按需 GPU、静态部署说明与实际 pages.rs 一致；桌面三栏、375px 一栏，沿用 hairline feature list。桌面后接的两个介绍段与已读取 README 对应，且 pages.rs 通过 `context.markdown_intro("README.md")` 引入正文。 |
| 分类正文与真实 source | match | Data table 有单一正文标题、说明、Example 与真实表格演示；桌面接项目相对路径、Function / Full file / Copy 和实际 `data_table_demo` 源码。与 category/context.demo 及 collections.rs 对应，未以静态图片替代组件。 |
| 分类表格的深色文字 | contradicted | [category-dark](official-subpath-r3/capture-batch/official-category-desktop-dark.png) 的表头可读，但四个数据行的文字不可辨认。浅色与窄屏浅色图同位置显示 Button/Actions、Text field/Input、Data table/Collections、Dialog/Overlays；collections.rs 也明确提供这四项。深色图表体取样仍为 `#f9fafb`，首行标签区域几乎全为近白像素；浅色同区域有 `#1d2939` 文字。违背主题切换后演示可读与 AA 对比度契约，见 M5-F1。 |
| 指南阅读结构 | match | Quick start、Prerequisites、正文列表与代码块在浅/深色均可读；1280 保留左 sidebar / 36rem 正文 / 右 TOC。导航 label 的多行换行和当前项蓝色标记安静且清楚。 |
| 窄屏层级与间距 | match | 三页均使用 16px 页面 gutter；分类与指南按 header、GitHub、Menu、On this page、正文排列。标题前后有清楚层级，features、列表、table 不跨出页面右边界。 |

THESIS、FIRST VIEWPORT 和 FORM 保留既有 Read 拓扑；静态首页的两条进入路径清楚。OWN-WORLD 的分类 demo 深色可读性由 M5-F1 局部违背。STORY 中的导航、复制和组件操作属于主实施者的行为验证，本报告不作独立通过判断。

## ceiling

既有 Read 世界的构图、字体、留白与导航密度已到位；M5-F1 使本批尚未达到浅/深色皆可读的质量底线。无需引入新字体、插图、品牌世界或页面装饰。修复只应恢复真实表格数据在深色下的可见性。

## material_fixes

1. **M5-F1 — 深色表格可读性（主题 / contrast）**：修复 Data table 演示在文档深色模式中的表体与数据行文字配色，让上述四行两列内容都可辨认并符合正文对比度要求。以 [category-dark](official-subpath-r3/capture-batch/official-category-desktop-dark.png) 为失败证据、[category-light](official-subpath-r3/capture-batch/official-category-desktop-light.png) 为内容对照；完成后在同一批确认中提供实际浅/深色与窄屏 recapture，确认该修复及其引入的回归。当前截图不能确定缺陷是持续状态还是主题同步时序，需主实施者在可用浏览器中查证后处理。

## keep

保留 P1 冷中性色、系统字体、36rem 阅读栏、首屏两入口、README 直接引入、真实 DOM demo 与精确 source；本次视觉修复不扩为行为、性能、真机、Safari 或线上验收。
