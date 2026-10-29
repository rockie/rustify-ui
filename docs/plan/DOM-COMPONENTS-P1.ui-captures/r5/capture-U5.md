# UI 验收 R5 · U5 采集清单

- 被测版本：HEAD `941a8043903c244dc2b72e1e90c38b49e193c20c`，产品路径相对 HEAD 无 diff、无未跟踪文件。
- 构建：本轮从当前源码重新构建，Studio `5b7550dc7339ced2`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`，沿用已有人工循环算法，与 R4 相同。本轮不是重建基线。
- Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；默认 DPR 1，焦点特写为 DPR 2。
- 每帧等待 document.fonts.ready、networkidle 及连续 500ms 无超过 100ms 的帧。每流程新 browser context，空 localStorage（U4 种子与环境干预见逐图表）。未拦截网络请求，未改变产品 UI 代码。
- 页面截图为可视区域；U6 分段为元素截图；U7 整段为夹具根向外取整并四周留 8px 的页面裁切；焦点特写为控件四周 24px 的页面裁切。原始 PNG 未做合成、修饰或覆盖。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/；均由本轮复制的 release 产物通过 xtask serve 提供 strict CSP。catalog 单元经页面既定 mount_button_fixtures / mount_layout_fixtures 入口进入。

- 本单元共 18 张本轮截图。操作记录：[U5-actions.log](U5-actions.log)。

| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:16.397Z | renderer-dom | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T14:05:16.980Z | mode-dark | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:17.662Z | local-appearance | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:18.312Z | menu-item-duplicate | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:18.896Z | preview-dialog-close | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:19.561Z | compare-dark | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:20.381Z | renderer-compare | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:29.658Z | inspector-toggle | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:30.258Z | inspector-toggle | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:30.865Z | color-value | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:39.964Z | inspector-toggle | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:41.044Z | inspector-toggle | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:05:48.675Z | mobile-preview | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:05:49.241Z | preview-dialog-close | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T14:06:35.961Z | mode-dark | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:43.667Z | menu-item-duplicate | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:44.250Z | preview-dialog-close | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:53.126Z | inspector-toggle | none |
