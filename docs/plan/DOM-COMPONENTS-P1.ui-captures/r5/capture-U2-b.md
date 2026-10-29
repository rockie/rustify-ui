# UI 验收 R5 · U2 采集清单

- 被测版本：HEAD `941a8043903c244dc2b72e1e90c38b49e193c20c`，产品路径相对 HEAD 无 diff、无未跟踪文件。
- 构建：本轮从当前源码重新构建，Studio `5b7550dc7339ced2`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`，沿用已有人工循环算法，与 R4 相同。本轮不是重建基线。
- Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；默认 DPR 1，焦点特写为 DPR 2。
- 每帧等待 document.fonts.ready、networkidle 及连续 500ms 无超过 100ms 的帧。每流程新 browser context，空 localStorage（U4 种子与环境干预见逐图表）。未拦截网络请求，未改变产品 UI 代码。
- 页面截图为可视区域；U6 分段为元素截图；U7 整段为夹具根向外取整并四周留 8px 的页面裁切；焦点特写为控件四周 24px 的页面裁切。原始 PNG 未做合成、修饰或覆盖。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/；均由本轮复制的 release 产物通过 xtask serve 提供 strict CSP。catalog 单元经页面既定 mount_button_fixtures / mount_layout_fixtures 入口进入。

- 本单元共 13 张本轮截图。操作记录：[U2-actions.log](U2-actions.log)。

| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:47.282Z | color-token-primary | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:47.835Z | color-token-primary | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:48.419Z | — | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:49.004Z | — | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:49.600Z | color-token-ring | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:50.200Z | editor-controls | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:50.781Z | edit-radius | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:59.925Z | editor-shell | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:02.150Z | color-token-primary | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:01.271Z | studio-config-gray_color | none |
| U2 | search-row | [U2-search-row-1440-light.png](U2-search-row-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:45.935Z | editor-colors | none |

## 评审指出缺证后的补采

原 11 张清单与图片保持不变。评审指出分组重置影响列表未展开，另开真实默认页选择 Graphite、Pink、Auto 后展开并拍下首尾两帧；未改界面代码。新增动作见 U2-supplement-actions.log。

| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U2 | group-reset-tokens | [U2-group-reset-tokens-1440-light.png](U2-group-reset-tokens-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:13:48.264Z | group-override-tokens | none |
| U2 | group-reset-tokens-end | [U2-group-reset-tokens-end-1440-light.png](U2-group-reset-tokens-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:13:48.828Z | group-override-tokens | none |
