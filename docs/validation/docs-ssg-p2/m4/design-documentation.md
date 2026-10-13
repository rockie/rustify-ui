# DOCS-SSG-P2 M4 设计同步记录

## Overview

本次覆盖 `rustify-docs dev` 的首次 building 壳、last-good 上的 building 反馈、多行 Rust 诊断与状态恢复。呈现路径为 DOM：静态标题、状态文本与既有诊断层不依赖岛或 GPU。反馈层沿用 [dev surface](../../../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md) 的 Operate 模式，文档页面仍保持 Read 身份。

这是已选表面的普通扩展，继续使用 `user-pinned-docs-ssg-p1-dev` 的角落面板、可收起 marker 和首次失败时的 inline 区域。[DESIGN.docs-ssg.md](../../../../DESIGN.docs-ssg.md) 仅同步受 M4 影响的 dev 正文规则；frontmatter、P1 阅读身份、M2/M3 扩展及其他设计文件保持原位，没有新增 token、视觉世界或产品素材。

### 直接核对的依据

| 依据 | 本次核对内容 |
| --- | --- |
| [dev.css](../../../../crates/rustify-docs/assets/dev.css) | 现有 panel / marker 的尺寸、语义色、inline / sheet、内部滚动与焦点；message 和 suggestion 的 pre-wrap |
| [dev.js](../../../../crates/rustify-docs/assets/dev.js) | building / phase / counts / last-good 组成；显示与清除条件；原文本节点、文件分组、换行；阶段与结果通知；重连和滚动恢复 |
| [shell.rs](../../../../crates/rustify-docs/src/shell.rs)、[generation.rs](../../../../crates/rustify-docs/src/generation.rs) | `render_dev_shell_with_status` / `dev_shell_with_status` 的静态首次构建与失败壳、noindex、基础资产和语言 |
| [labels.rs](../../../../crates/rustify-docs/src/labels.rs) | en/zh 的首次构建标题、说明、三个阶段、last-good、错误修复和 revision 原文 |
| [finish-review.md](finish-review.md) | persistence、fidelity、ceiling、material_fixes、keep 全部已读取；disposition 为 ship，material_fixes 为 none；结论限九张图的视觉与有效性 |
| [ui-batch-r1](ui-batch-r1/) | 九张 PNG 均直接打开；逐字节比较原浏览器附件，SHA-256 一致；检查 PNG 尺寸为桌面 1280×900、窄 viewport 375×812 |

本次没有运行 context loader、detector、Cargo、浏览器或服务。源代码说明已实现的状态逻辑；截图说明这些状态在这批视图中的外观，两者的证据范围分开记录。

## Colors

新增 building 和 phase 文本继承既有 foreground，不借用错误红或警告色。错误与警告 count / rule 继续使用原语义色；last-good 与 revision 保持次要文字。popover、border、内部分隔线和 focus ring 均使用既有变量，没有新的色值。

首次壳截图使用默认阅读主题；Rust 页面与诊断层随 fixture 的导入主题显示。该适配已有 M3 正文规则，截图中的导入色值不写成默认主题 token。本次未独立测量对比度、computed style 或字体请求。

## Typography

首次壳保留既有 H1、正文和 36rem 阅读栏。阶段、revision 与多行诊断没有建立新字级：dev 正文仍为既有 0.875rem / 1.45，state 为 0.8125rem，revision 为 0.75rem；文件名与行列继续用 mono，正文和建议保持各自的语义文字色。

Rust 消息与建议使用 pre-wrap 保留换行，并允许长文本在列表内换行；backtick 引用仍由既有 prose 函数输出代码节点。它们没有被重绘成独立终端面板，也不把编译器源码片段提升为新的全局代码样式。

## Layout

首次构建使用原有 inline 诊断区，跟随静态 H1 和说明，默认不设高度上限；没有浮动 marker 或关闭入口。画面不依赖 Rust 岛启动，外部 dev 脚本提供阶段与 revision。

已经有 last-good 时保留原页面，building 复用现有角落 marker 或打开的 panel。默认桌面 panel 仍为 26.25rem 上限、60vh 最大高度；小于 640px 为 50vh 底部 sheet，列表内部滚动。窄屏 marker 沿用盒状外观和可换行文本。九张图中的 Rust 视图是滚动位置上的 viewport capture，没有被记作 full-page；列表截住后续诊断符合其滚动边界。

**The Last-Good Feedback Rule.** building 反馈在现有 dev 层中变化，文档继续可读。building 结束后由真实诊断决定保留面板/marker或清空它们，不把编译中当成新的失败。

## Elevation & Depth

使用已记录的 dev popover 柔和阴影、细边界与 tonal surface；首次 inline 区域保持无阴影。没有为 building 增加深度、遮罩、spinner 或百分比。原 120ms 进入动效仍受 reduced-motion 条件限制；此次静态截图不构成动效验收。

## Shapes

panel、窄屏 sheet、marker 和描边 SVG 复用既有形状。默认圆角与导入主题的半径关系由既有 M3 规则约束，本次不把 fixture 的圆角或 palette 固定到所有文档站。文件长路径的按段换行和诊断文本的保留换行承担信息表达，不新增图标或操作。

## Components

### 已实现的状态语义

| 状态 | 源码中的实现与本次证据 |
| --- | --- |
| First building | 静态壳选取 “Building your site” 与首次依赖构建说明；无 TOC、noindex。dev.js 在 inline 区域显示 Building、当前阶段与 revision。三张 first-building 图直接支持其外观 |
| Building over last-good | 保留页面、前一轮诊断与当前打开/收起选择；Building 和 phase 为中性文字，counts 仍按真实严重度着色。building 本身不强制打开浮层。三张 rust-building 图支持 marker 与页面并存 |
| Failed | building 提示收起，保留 last-good；按文件列出原位置、规则、完整多行消息与建议。未被作者收起时错误打开 panel。三张 rust-error 图支持位置、多行和受限列表的外观 |
| Success | building 结束且零 errors / warnings 时隐藏 panel / marker，并清除收起选择；若有 warnings，保留 warning marker。被标为需刷新的页面或新 session 使用已有 reload / scroll-restoration 路径。这些转换依据源码，未由本次截图独立验收 |
| Reconnect | EventSource 掉线自行重连；CLOSED 时 1s 后重新连接，携带 path / good revision / session。收到新状态再同步反馈；未增加连接 banner 或重试按钮。依据源码，未在本次操作验证 |

已有 live region 在 building 首次出现或 phase 改变时通知摘要；错误到达或 revision 更新时通知摘要与 revision，错误清除时通知修复文本。构建状态不主动移动阅读焦点；原有清空诊断时从 dev 控件转回 main、Hide / Escape 到 marker 与 marker 到 Hide 的行为继续保留。本次没有独立键盘或读屏测试。

### 截图来源与实际范围

| 状态 | Desktop light | Desktop dark | Narrow light | 原始附件 |
| --- | --- | --- | --- | --- |
| First building | [PNG](ui-batch-r1/first-building-desktop-light.png) | [PNG](ui-batch-r1/first-building-desktop-dark.png) | [PNG](ui-batch-r1/first-building-narrow-light.png) | [browser-r2 first-build](browser-r2/attachments/first-build/) |
| Rust building | [PNG](ui-batch-r1/rust-building-desktop-light.png) | [PNG](ui-batch-r1/rust-building-desktop-dark.png) | [PNG](ui-batch-r1/rust-building-narrow-light.png) | [browser-r3 rust-save](browser-r3/attachments/rust-save/) |
| Rust error | [PNG](ui-batch-r1/rust-error-desktop-light.png) | [PNG](ui-batch-r1/rust-error-desktop-dark.png) | [PNG](ui-batch-r1/rust-error-narrow-light.png) | [browser-r3 rust-save](browser-r3/attachments/rust-save/) |

九张均与原附件逐字节一致，桌面和窄 viewport 尺寸相符。首次壳从文档顶部可见；Rust error 的核心位置为截图中的 `src/demos.rs:5:41`，桌面长路径与窄屏多行消息在既有面板中可读。finish review 的 ship 仅覆盖该 visual packet，没有因此宣称行为、性能、字体加载或完整 P2 通过。

## Do's and Don'ts

- **Do** 用中性 Building / phase 区分编译与失败，保留真实 error / warning count 和 last-good 文档。
- **Do** 保留消息、建议和原文件位置的完整文本与换行，让列表承担滚动。
- **Do** 沿用既有 inline / corner / sheet 结构、收起选择、stage / result 通知和滚动恢复。
- **Don't** 把导入主题 fixture 的色值、字体或圆角写成默认 reading token。
- **Don't** 用顶栏、全屏遮挡、进度百分比或额外连接操作替换既有反馈层。

保留范围：默认主题字体段落和 M3 显式导入主题扩展继续分别适用，未清理其他既有措辞或 detector / sidecar 配置。此轮只读源码与九张本地 Chromium 图片；没有窄屏 dark 图、首次失败壳、成功/重连转换、原生 200% zoom、实体手机、五位首次用户或读屏的独立本轮证据。M6 的手机、真人和其余浏览器出口仍保留未验证状态；此记录不关闭这些门，也不形成发布结论。
