# DOCS-SSG-P2 M2 设计同步记录

## Overview

本记录覆盖 M2 已实现的 DOM 演示外壳、状态与同源源码，以及静态 Tabs、CardGrid、Steps。呈现路径为 DOM：文档、状态、源码和静态内容块由 HTML/CSS 与基础脚本承担，注册的交互视图由 DOM 岛接管；本次没有 GPU 呈现需求。

这是对 P1 阅读主题的普通扩展。[阅读表面 brief](../../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 的 Read 模式、OWN-WORLD 和正文优先层级继续约束本次工作。扩展让读者在正文中操作一个真实示例，再查看其函数或完整文件；页面导航、状态和源码不进入该示例的失败域。没有建立新的视觉世界或全仓设计系统。

设计事实以完成版代码和生成产物为准；本记录采用 impeccable document 的章节顺序，未生成第二份 token 定义。已完整读取现有 [PRODUCT.md](../../../../PRODUCT.md)、[DESIGN.md](../../../../DESIGN.md) 与 [DESIGN.docs-ssg.md](../../../../DESIGN.docs-ssg.md)：前两份限定产品与 SDK / Theme Studio 世界，第三份记录阅读主题的独立 token、字级、measure 和组件规则。按计划 §6，仅在 DESIGN.docs-ssg.md 的 Components 追加 M2 增量正文，记录外置状态与源码、恢复尺寸预留和静态块语义；既有 frontmatter 与其余规则保留。PRODUCT.md、根 DESIGN.md、[Docs sidecar](../../../../.impeccable/docs-ssg.design.json)、[SDK / Theme Studio sidecar](../../../../.impeccable/design.json) 未改动，没有把工作台的字体、布局和阴影移植到文档。

### 核对证据与结论边界

| 证据 | 本次直接核对的内容 |
| --- | --- |
| [demo.rs](../../../../crates/rustify-docs/src/demo.rs)、[builtins.rs](../../../../crates/rustify-docs/src/builtins.rs) | 外置状态与源码、禁脚本默认态、静态内容语义、作者文本转义和 children 保留 |
| [docs.css](../../../../crates/rustify-docs/assets/docs.css)、[style.rs](../../../../crates/rustify-docs/src/style.rs) | 既有 token 复用、字级、间距、状态预留、内部滚动、焦点与内容块断点 |
| [mode.js](../../../../crates/rustify-docs/assets/mode.js)、[site.js](../../../../crates/rustify-docs/assets/site.js)、[islands.js](../../../../crates/rustify-docs/assets/islands.js) | 输入闸、终止态恢复、页签键盘/ARIA、原文复制与反馈；三份脚本与当前生成资产逐字节一致 |
| [当前生成目录](../../../../target/docs-ssg/http/components/subpath/dist/) | manifest 的 base 为 `/rustify-ui/`，build_id 为 `eb0630d32488043c`；检查 counter、failure、client-only、builtins 的 HTML 与 `assets/site.c502b85b59.css`。前两页各有两份可聚焦的源码面板，Function 原文是 Full file 原文的子串；builtins 无岛模块或 wasm 入口 |
| [完整 finish review](../../../../.impeccable/review/docs-ssg-p2-m2/finish-review.md)、[修复复核](../../../../.impeccable/review/docs-ssg-p2-m2/finish-review-fix.md) | 完整审阅的 persistence、fidelity、ceiling、material_fixes、keep 均已读取；原 disposition 为 fix，唯一列出的移动端状态预留已在复核中 resolved，复核 disposition 为 ship，范围限于该修正。已核实初评的根文档输入描述更正为“未独立审计”，与本记录直接读取根文档的范围区分 |
| [当前截图包](../../../../.impeccable/review/docs-ssg-p2-m2/)、[r6 静态块禁脚本截图](islands-r6/docs-ssg-islands-static-bu-8c38b-native-link-and-child-block-components-subpath/) | 直接打开 20 张图：ready / failure / client-only / no-script 的 1280 与 375 图，静态块增强与禁脚本的 1280 与 375 图，以及 light/dark 的 loading/failure 375 与 640 图 |
| [visual-capture-r2.json](visual-capture-r2.json)、[islands-r6/results.json](islands-r6/results.json)、[响应式断言](../../../../tests/docs-ssg/islands.spec.ts) | 八张主要截图的状态行均为 72px、页面 overflow 为 0；现有 components-subpath 报告为 expected 20、skipped / unexpected / flaky 均为 0。已读状态高度与源码位置在 loading、ready、failure 间保持相等的断言；本次未重跑浏览器或 Cargo |

截图与既有测试是本地 fixture 的证据。640 图的“200%”来自测试约定的半宽 CSS viewport 加双倍根字号；它不是实体设备或浏览器原生缩放测量。现有结果覆盖异步操作、三个同类岛、源码切换/复制、共享故障、资源失败、迟到回调、浮层关闭与焦点返回。本记录不把 M2 的范围扩成 P2 全期、线上发布、完整浏览器矩阵、读屏或真人验收。

## Colors

新增外壳复用文档语义色，没有新增调色板。主色仍用于链接、页签选中线、当前导航和焦点；状态和项目相对路径使用次要文字，故障复用 caution 文本色，复制成功复用 tip 文本色。画布与恢复按钮沿用页面底色，源码沿用较深的中性代码底色。

以下是当前生成 CSS 的观察值，事实源仍为 `style.rs`、主题和生成 CSS：

| 用途 / 既有变量 | Light | Dark |
| --- | --- | --- |
| 页面底色 `--background` | `#f9fafb` | `#0c111d` |
| 正文 `--foreground` | `#1d2939` | `#f9fafb` |
| 次要文字 `--muted-foreground` | `#667085` | `#98a2b3` |
| 细分隔线 `--secondary` | `#eaecf0` | `#1d2939` |
| 主色 / 焦点 `--primary` / `--ring` | `#175cd3` | `#53b1fd` |
| 代码底色 `--rd-code-bg` | `#f2f4f7` | `#161b26` |
| 故障文字 `--rd-caution-text` | `#a40e26` | `#ffa198` |

light/dark 加载与故障截图显示状态、恢复入口和源码分别可辨。finish review 的生成 token 对比度核对为：light 次要文字在页面上 4.76:1、在代码底色上 4.51:1，故障文字 light 7.53:1、dark 9.70:1。这是已提供审阅的 token 检查结果，本次未进行独立浏览器 computed-style 或读屏审计。

## Typography

既有系统 sans 与系统 mono 保持原位。文章 H1（2rem / 700 / 1.2）、H2（1.375rem / 650 / 1.3）、H3（1.125rem / 600 / 1.3）与正文（1rem / 1.75）没有为演示重设。代码仍使用 mono（0.875rem / 1.65）。这些值与 DESIGN.docs-ssg.md 的 frontmatter 和阅读字级一致，不是从 Theme Studio 字级迁移。

新增状态行和源码路径采用既有代码工具栏的细字级（0.8125rem），状态行行高为 1.5。源码页签保持同样字级；静态 Tabs 用稍大的标签（0.875rem），选中项为 600 字重。CardGrid 标题与 Steps 标题仅用正文 600 字重，它们在源码中是链接或段落，不生成额外标题层级。Tabs 在禁脚本时的面板标题是帮助分辨内容的标签，增强后收起。

## Layout

原阅读布局、导航与 TOC 折叠继续生效；演示没有改变文章宽度。CSS 的既有阅读 measure 为 36rem，保留了它与 brief 中约 720px 描述之间的既有差异。1280 截图仍为三栏，375 截图仍由原生 details 收纳导航与 TOC。

演示沿用代码块的上下节奏（1.5rem），内部顺序为画布、状态、源码。画布内边距为 1rem；状态内边距为 0.75rem / 1rem，采用“可换行的文案 + 恢复按钮”两列。源码路径使用 0.625rem / 1rem 内边距；源码滚动区上限为 24rem，长行与长文件在区域内部滚动，pre 可获得键盘焦点。

**The Stable Recovery Footprint Rule.** 状态行在同一布局下预留完整故障文案和 Reload 按钮的内在尺寸，并保留紧凑最小高度（4.5rem）。未激活的恢复项采用不可见与 `aria-hidden`，继续参与网格尺寸计算；ready 隐藏提示而不挪动源码。72px 是当前主要截图在默认字号下的观察值，不能把它当作所有语言、字号和宽度的硬高度。窄屏旧 8rem 下限已移除，原 128px 空带不再成为可复用规则。

静态块同样使用上下节奏（1.5rem）。CardGrid 默认两列，40rem 起在允许时为两列，64rem 起 `columns=3` 才为三列；小于 40rem 或 `columns=1` 保持一列。列间距为 1.5rem，每个条目以顶部分隔线和 1rem 上下留白划分。Steps 使用原生有序列表和 1.5rem 缩进，标题到描述为 0.375rem；序号表达步骤顺序。

## Elevation & Depth

文档外壳与静态块保持平面层级：底色、细分隔线和稍深的代码底色区分区域，没有新增投影。源码边缘已有的渐变只提示内部尚有可滚动内容，不构成卡片阴影。岛中实际 SDK 控件和其拥有的浮层继续使用 SDK 自身语义与行为，不把其样式提升为文档 chrome 的通用规则。

## Shapes

演示整体沿用既有面板圆角（0.375rem / 6px）与 1px 边界；内部状态与源码由细横线分隔，不重复套圆角卡片。恢复按钮也沿用这一圆角和中性边界，hover 将边界变为主色。

源码页签和静态 Tabs 复用细下划线选中态（2px），未选中项保持次要文字。CardGrid 是分隔线列表，不新增包裹每张卡的圆角、背景或阴影。全局可交互目标沿用可见焦点（3px 不透明 ring、2px 间隔）；源码滚动区和源码页签使用内收的 outline offset，保持轮廓可见。

## Components

### DOM 演示、状态与源码

一个有边界的正文示例，恢复动作与源码始终在画布外。[demo.rs](../../../../crates/rustify-docs/src/demo.rs) 把原 SSR 视图放入 disabled fieldset，再输出独立状态与可选源码；[mode.js](../../../../crates/rustify-docs/assets/mode.js) 的锁与恢复仅作用于画布。

| 状态 | 已实现语义 |
| --- | --- |
| no-script | 原 SSR 视图可读，外层 fieldset 禁用原生控件；noscript 显示 “Enable JavaScript to interact”。源码两面板顺序呈现，可选择原文，增强页签与复制按钮隐藏 |
| loading | 仅画布 inert 并标记 aria-busy，外置 role=status / polite 显示 “Loading interactive example…”；源码增强和普通导航可用 |
| ready | 解除外层闸，保留组件自身 disabled 状态；加载与故障提示收起，预留行保留 |
| failed | 克隆恢复原始视图，画布 disabled / inert；外置状态改为 role=alert / assertive，显示故障文案与 “Reload page”。共享实例内全部块成为终止态，迟到完成不能改回 ready；基础脚本、导航和源码仍可用 |
| client-only | 生成 HTML 保留作者声明的 `min-h-48` 占位与 “This counter starts in the browser.”，接管后在该区域挂载真实计数器、typed props 与 child 内容；ready 截图和现有接管测试已核对。独立的 client-only 禁脚本截图未提供，不声称该视觉态已经单独验收 |

**The External State and Source Rule.** 锁住或恢复岛内容时，保留状态、Reload 和源码节点；浏览器故障不能消除读者查看原文和继续阅读的路径。

Function / Full file 使用原有 code-group 页签，组名含该演示的 key；每个页签与自己的面板配对。源码路径仅显示项目相对路径。Copy 读取 raw-text template，而非高亮后的行；成功和失败沿用原地文字与 live-region 反馈，拒绝剪贴板时选择代码以供手动复制。源码的 CRLF 保留由 `demo.rs` 现有测试说明，本次生成 fixture 直接核对了函数是完整文件子串、两块原文与可聚焦 pre；未审计原始源码提取器。

### 静态 Tabs / CardGrid / Steps

**The Complete Static Content Rule.** 静态块的完整内容先进入 HTML，再由基础脚本提供局部选择；它们不创建交互岛，不要求页面加载 wasm。

Tabs 默认隐藏操作条并展示全部带标签的面板。site.js 增强后为 tablist、tab、tabpanel 建立全页唯一的 ID / ARIA 配对，选中项使用 roving tabindex，支持 Left / Right / Home / End。选择只影响最近的 Tabs 容器；两个并列组的状态、嵌套代码组和追加 child 内容各自保留。当前增强截图分别选中了两组中的不同项；禁脚本截图能阅读四个面板和追加 Markdown / 代码。

CardGrid 由 ul / li、普通 a 与描述组成，`columns` 仅接受 1–3。标题、描述是转义后的文字，链接继续进入页面正常链接校验与部署 base 映射；当前产物链接到 counter 与 proxy。Steps 由 ol / li、段落标题和描述组成，使用主题主色序号表达次序。两者保留 checked child 内容，禁脚本时没有损失阅读或导航路径。

## Do's and Don'ts

以下规则仅约束本次阅读表面的扩展：

- **Do** 复用文档语义色、细分隔线、6px 面板圆角和现有代码工具栏字级。
- **Do** 通过完整恢复文案与按钮的尺寸预留稳定布局，让大字号和换行自然增加行高。
- **Do** 保留禁脚本的 SSR 视图、全部静态面板、原生链接与可选择的源码。
- **Do** 将恢复入口与源码留在岛输入锁之外，为长文件提供可聚焦的内部滚动区域。
- **Don't** 把旧窄屏 8rem 状态空带写成设计 token 或沿用到新的示例。
- **Don't** 把 CardGrid 渲染成有装饰阴影的入口卡墙，或把 Steps 序号当作装饰图标。
- **Don't** 为本次 DOM 扩展新增 GPU 版本，或把 Theme Studio 的工作台结构移入文章。

既有记录范围与未扩大范围：根 PRODUCT.md / DESIGN.md 和阅读主题的 DESIGN.docs-ssg.md 实际存在；初评已将根文档描述更正为本轮 UI reviewer 未独立审计，本记录则已直接核对。Docs sidecar 当前只记录 footer 与地标/标题规则，DESIGN.docs-ssg.md 明确将它定义为 incremental component sidecar；不把局部覆盖误写成缺失全仓设计系统。本次未提供独立 QualityBar，不对其作认证。[现有 detector](design-detector.json) 的四项均为 advisory 字级匹配提示，四个值均已记录在 DESIGN.docs-ssg.md；其中两个首页值不在本次截图范围。未为匹配 SDK / Theme Studio 的 root 字级修改阅读主题，也未借此修复全局 detector 配置或登记新的禁令。源码提取器、导入主题、完整跨浏览器/实体设备与 M6/M7 出口不在本记录的核对范围。没有新增 shipping raster；截图作为验收证据保留，不成为产品素材。
