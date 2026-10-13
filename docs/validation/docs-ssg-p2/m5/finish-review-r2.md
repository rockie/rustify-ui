## verdict

本次为 [首批报告](finish-review.md) 中 **M5-F1 — 深色表格可读性** 的唯一修复确认，仅评分该项及修复引入的可见回归。原报告和 r3 失败截图保留原位。本结论只覆盖已评分修复及本批截图可见回归。

1. **M5-F1：resolved。** [r3 深色失败图](official-subpath-r3/capture-batch/official-category-desktop-dark.png) 中不可辨认的数据行，在 [r5 深色确认图](official-subpath-r5/capture-batch/official-category-desktop-dark.png) 中已完整显示 Button/Actions、Text field/Input、Data table/Collections、Dialog/Overlays。表体底色取样由 `#f9fafb` 变为文档深色 `#0c111d`，行文字实色取样为 `#f9fafb`，该主色组合的对比度为约 18.05:1。[桌面浅色](official-subpath-r5/capture-batch/official-category-desktop-light.png) 和 [375px 浅色](official-subpath-r5/capture-batch/official-category-narrow-light.png) 仍清楚显示相同数据及状态行。

实际修复源码为 [style.rs 的 tokens_css](../../../../crates/rustify-docs/src/style.rs)：在 `:root[data-rustify-scope] [data-rustify-scope]` 上为 SDK theme property 输出 `inherit`，让文档里的后代 scope 继承当前页面调色板。截图体现的是表体与真实数据文字恢复可读，demo 与 source 的组成、尺寸和位置保持原样。

确认图有效性 **pass，9/9**：本轮九图均重新用 `view_image` 查看，内容、主题与命名相符，无黑屏、整页渲染缺失或可见页面横向溢出；桌面均为 1280×900，窄屏均为 375×812。每张文件的字节数和 SHA-256 均符合 [r5 manifest](official-subpath-r5/capture-batch/manifest.json)，并与 [r5 results.json](official-subpath-r5/results.json) 中 capture test 的内嵌原始浏览器 PNG 一致。该 capture test 记录 official-subpath、passed；本轮来源为本地 Chromium `153.0.8010.12`。

| 页面 | 桌面浅色 | 桌面深色 | 窄屏浅色 |
| --- | --- | --- | --- |
| 首页 | [home-light](official-subpath-r5/capture-batch/official-home-desktop-light.png) | [home-dark](official-subpath-r5/capture-batch/official-home-desktop-dark.png) | [home-narrow](official-subpath-r5/capture-batch/official-home-narrow-light.png) |
| Data table 分类 | [category-light](official-subpath-r5/capture-batch/official-category-desktop-light.png) | [category-dark](official-subpath-r5/capture-batch/official-category-desktop-dark.png) | [category-narrow](official-subpath-r5/capture-batch/official-category-narrow-light.png) |
| Quick start 指南 | [guide-light](official-subpath-r5/capture-batch/official-guide-desktop-light.png) | [guide-dark](official-subpath-r5/capture-batch/official-guide-desktop-dark.png) | [guide-narrow](official-subpath-r5/capture-batch/official-guide-narrow-light.png) |

manifest 标记 CLI 0.1.9。只读计算 [command.json](official-subpath-r5/command.json) 指定二进制的当前 SHA-256 为 `bf39e6ea48bac44d590344e7e72fc3ed2b6f547d869670f9e4b46a260eab60d8`，与本轮交接指纹一致，未执行该二进制。

**修复引入的可见回归：none。** 除 category-dark 外，其余八张 r5 PNG 与首批对应 PNG 字节一致；深色分类图中的正文、导航、source 与状态行仍保持原有布局和可读性。

## remaining

clear，限 M5-F1 与本次可见回归。未重新寻找风格问题，也未运行浏览器、Cargo、build、端口服务、detector 或 context loader。此 ship 不扩为全表面、全部分类、行为、性能、字体加载、导入主题、实体手机、Safari 或线上通过；Root/meta 复验结果不纳入本报告。首批报告列明的未目视区域及 M6 真实用户、实体设备缺口继续保留。

disposition: ship
