disposition: ship

## persistence

pass。本次为独立、只读的 M4 视觉与截图有效性评审，采用 DOM 反馈层的 Operate 模式，继承文档页面的 Read 身份；构建提示和编译诊断属于文档文本与浮层，无 GPU 呈现需求。

已读取 [PRODUCT.md](../../../../PRODUCT.md)、[DESIGN.docs-ssg.md 的 dev panel 与导入主题规则](../../../../DESIGN.docs-ssg.md)、[dev surface 的 P2 M4 扩展](../../../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md)、[计划 §5.4 / §6](../../../plan/DOCS-SSG-P2.md)，并抽查 dev.js/dev.css、静态 shell、generation 与 labels。依据既有界面扩展契约评审，不适用新世界、approved comp 或新 QUALITY BAR 卡。FORM 的既定 seed 为 `user-pinned-docs-ssg-p1-dev`。M4 设计文档同步由主实施者按计划 §6 收口。

截图有效性 pass，9/9 已用 `view_image` 实际查看。每张 PNG 与下列原始浏览器附件的 SHA-256 一致，尺寸也与 [layouts 捕获逻辑](../../../../tests/docs-ssg/dev-components.spec.ts) 一致。

| 状态 | desktop-light | desktop-dark | narrow-light | 原始来源 |
| --- | --- | --- | --- | --- |
| 首次 building | [1280×900](ui-batch-r1/first-building-desktop-light.png) | [1280×900](ui-batch-r1/first-building-desktop-dark.png) | [375×812](ui-batch-r1/first-building-narrow-light.png) | browser-r2/attachments/first-build |
| Rust building | [1280×900](ui-batch-r1/rust-building-desktop-light.png) | [1280×900](ui-batch-r1/rust-building-desktop-dark.png) | [375×812](ui-batch-r1/rust-building-narrow-light.png) | browser-r3/attachments/rust-save |
| Rust error | [1280×900](ui-batch-r1/rust-error-desktop-light.png) | [1280×900](ui-batch-r1/rust-error-desktop-dark.png) | [375×812](ui-batch-r1/rust-error-narrow-light.png) | browser-r3/attachments/rust-save |

九张均显示文件名所指状态与主题，无黑屏、缺失渲染区域或可见横向溢出。首次 shell 的文档顶部可见；Rust 截图是滚动位置上的 viewport，捕获代码使用 `page.screenshot()`，未宣称 full-page。诊断面板底部截住后续条目符合内部滚动容器，不是截图损坏。r2 首次路径与 r3 Rust 路径各自结果为 passed；这只用于核对捕获来源，本评审没有重跑行为验收。

本次未启动浏览器、服务器、Cargo、context loader 或 detector；dev.css 的既有 hook 无确定性问题由交接包报告。本 disposition 只覆盖这批截图的视觉与有效性，不覆盖行为、性能、字体加载或完整 P2。375px 截图是本地 Chromium 窄 viewport；计划 §9.3 / §11 的 M6 五位首次用户、实体手机及其余缺证浏览器渠道仍保留未验证状态。

## fidelity

| 元素 | 判定 | 证据与依据 |
| --- | --- | --- |
| TYPE | adaptation | 首次 shell 延续 P1 的系统 sans/mono 与既有标题、正文层级；Rust fixture 延续已导入的文档字体与圆角。后者的适配由 DESIGN.docs-ssg.md 的 P2 M3 导入主题规则与 [fixture theme](../../../../tests/docs-ssg/fixtures/components/theme.json) 明确允许；本次未新增 display 字体或独立验证字体加载。 |
| MATERIAL | match | 原有平面文档、hairline 分隔和 dev popover；浮层仅有已记录的 SDK 柔和阴影，图标为一致的描边 SVG。未出现伪物理材质、装饰图片或额外视觉世界。 |
| GROUND | match | PNG 取样：首次 shell 底色 `#f9fafb` / `#0c111d`、inline 表面 `#ffffff` / `#1d2939`，与 P1 token 一致；Rust 页底色 `#f6faff` / `#061321`、error 面板 `#e6f2ff` / `#102c45`，与 fixture theme 的 background/popover 一致，属于上述 M3 主题适配。 |
| 首次构建 shell | match | 简明 H1、首次依赖构建说明、当前 native-renderer 阶段与 revision 位于原 36rem 栏；窄屏说明自然换行，状态行与 revision 均完整可读。符合 M4 brief 的静态 DOM 状态。 |
| 增量 building 标记 | match | 原文档和控件仍在画面中；右下角中性标记区分 building、当前阶段、warning 数和 last-good。窄屏沿用既有盒状 marker 并换行，没有顶栏、全屏遮挡、spinner 或百分比。 |
| Rust error 组成 | match | 桌面保留约 420px、60vh 的角落面板；375px 为 50vh 底部 sheet。Counts、last-good、revision、关闭 affordance 与文件分组维持原阅读顺序。 |
| 文件行列与多行消息 | match | `src/demos.rs` 与 `5:41`、完整位置 `src/demos.rs:5:41` 可见；编译消息和源码片段保留换行。桌面长 warning 路径按段换行，窄屏核心错误没有从页面右侧溢出；与 dev.css 的 `pre-wrap` 及既有可滚动列表一致。 |

THESIS 与 OWN-WORLD 保持角落反馈、中性主题和有语义的错误/警告颜色；FIRST VIEWPORT 与既有面板/marker 拓扑一致；FORM 延续用户已选的 dismiss-to-marker。STORY 中恢复、清空和滚动保持属于主实施者的行为证据，本次静态视觉评审不作独立通过判断。

## ceiling

reached，限本次既有界面扩展。层级、间距、主题适配、真实文件位置与受限浮层已完成所需表达；没有需要追加的视觉装置。P1 明确规定的系统字体、36rem 阅读栏和 SDK popover 深度继续作为质量依据。静态截图不形成动效、焦点或实体设备结论。

## material_fixes

none。

## keep

保留原阅读栏与主题、完整 last-good 页面、安静的角落 building marker、可关闭的诊断层和原 Rust 行列/多行文本；维持 M6 真人与实体手机的未验证缺口。
