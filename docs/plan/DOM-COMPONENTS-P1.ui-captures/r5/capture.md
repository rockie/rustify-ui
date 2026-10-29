# DOM-COMPONENTS-P1 UI 验收 R5 · 采集清单

- 被测版本：HEAD `941a8043903c244dc2b72e1e90c38b49e193c20c`，产品路径相对 HEAD 无 diff、无未跟踪文件。
- 构建：本轮从当前源码重新构建，Studio `5b7550dc7339ced2`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`，沿用已有人工循环算法，与 R4 相同。本轮不是重建基线。
- Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；默认 DPR 1，焦点特写为 DPR 2。
- 每帧等待 document.fonts.ready、networkidle 及连续 500ms 无超过 100ms 的帧。每流程新 browser context，空 localStorage（U4 种子与环境干预见逐图表）。未拦截网络请求，未改变产品 UI 代码。
- 页面截图为可视区域；U6 分段为元素截图；U7 整段为夹具根向外取整并四周留 8px 的页面裁切；焦点特写为控件四周 24px 的页面裁切。原始 PNG 未做合成、修饰或覆盖。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/；均由本轮复制的 release 产物通过 xtask serve 提供 strict CSP。catalog 单元经页面既定 mount_button_fixtures / mount_layout_fixtures 入口进入。

## 覆盖与出处

- 模式：已有纯 Markdown 历史，无 managed 索引；继续人工 R5，未 import、未重置历史。
- U1: 25, U2: 13, U3: 18, U4: 17, U5: 18, U6: 28, U7: 26；合计 145 张、4 次采集运行。
- 脚本报告失败：[]。全部 7 单元及目标包采集步骤已拍摄；评审若指出看不清的格子，按合同补采。原图完整性与尺寸见 image-integrity.json，主 agent 图像真实性检视见 capture-qa.json。
- 原始操作、时间与干预逐帧见下表及 U<n>-actions.log；完整采集轨迹为 capture-log.json，脚本为 capture-runner.cjs。
- 为避免本地 target 回收影响服务，本轮将构建完成的两份产品复制到 products.json 记录的临时目录；服务 PID 与停止方式见 services.txt。
- 初次构建因本地 makepad 原生缓存中存在 Rust 1.99.1 的 rlib 而 E0514；已保留原目录，使用 mise 固定工具重新构建成功，见 build-theme-studio-final.log、build-component-catalog.log、cache-workaround.json。
- R5 增拍 U2-search-row，以完整呈现跨单元颜色搜索行。U1-zoom200-source 首拍之后沿用 R4 的稳定后补采，复核采用 -a2 与 -end，原图保留。

## 逐图登记

| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:02:42.864Z | — | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:02:44.099Z | studio-config-gray_color | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T14:02:44.695Z | mode-dark | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T14:02:45.245Z | mode-dark | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:02:45.980Z | locale-zh | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:02:46.529Z | locale-zh | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T14:02:55.508Z | — | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T14:02:56.641Z | studio-config-gray_color | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:02:58.565Z | — | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:02:59.432Z | studio-config-gray_color | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:03:07.197Z | mobile-preview | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:03:16.547Z | — | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:03:17.151Z | data-tools-dialog-close | none |
| U1 | zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:03:26.344Z | editor-colors | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:03:26.958Z | color-token-primary | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:36.040Z | preset-search | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:36.606Z | studio-config-accent_color | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:37.155Z | studio-config-accent_color-list | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:37.834Z | mode-dark | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:47.282Z | color-token-primary | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:47.835Z | color-token-primary | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:48.419Z | — | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:49.004Z | — | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:49.600Z | color-token-ring | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:50.200Z | editor-controls | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:50.781Z | edit-radius | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:03:59.925Z | editor-shell | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:02.150Z | color-token-primary | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:10.814Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:11.381Z | import-review | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:11.978Z | import-review | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:13.011Z | data-tools-dialog-close | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:13.648Z | export-kind | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:14.288Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:14.890Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:15.489Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:16.138Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:17.036Z | export-copy | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:17.599Z | export-copy | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:19.598Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:20.146Z | import-review | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:21.032Z | data-tools-dialog-close | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:30.833Z | — | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:31.453Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:32.018Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:32.748Z | theme-save | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:34.932Z | — | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:04:37.971Z | — | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:46.912Z | theme-save | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:47.497Z | storage-operations | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:57.879Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:58.445Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:04:59.048Z | storage-external-raw | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:07.367Z | — | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:05:08.136Z | theme-save | navigator.locks removed before load |
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
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:05:57.242Z | data-tools-dialog-close | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:05.319Z | — | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:05.870Z | editor-colors | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:06.971Z | studio-config-gray_color | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:14.734Z | import-review | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:06:16.712Z | import-review | none |
| U4 | conflict-detail-end | [U4-conflict-detail-end-1440-light.png](U4-conflict-detail-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:26.381Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:26.930Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-diff | [U4-conflict-diff-1440-light.png](U4-conflict-diff-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:27.529Z | storage-conflict-diff | second same-origin tab wrote v2 |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T14:06:35.961Z | mode-dark | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:43.667Z | menu-item-duplicate | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:44.250Z | preview-dialog-close | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:06:53.126Z | inspector-toggle | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:01.271Z | studio-config-gray_color | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T14:07:03.255Z | import-review | none |
| U4 | upgrade-review-json | [U4-upgrade-review-json-1440-light.png](U4-upgrade-review-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:12.114Z | upgrade-review | localStorage v1 seeded |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:20.498Z | export-kind | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:07:29.876Z | color-token-primary | none |
| U1 | zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T14:07:30.941Z | color-token-primary | none |
| U2 | search-row | [U2-search-row-1440-light.png](U2-search-row-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:07:45.935Z | editor-colors | none |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:08:18.940Z | — | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T14:08:19.619Z | button-mode-dark | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:08:20.285Z | button-toggle-loading | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:20.900Z | button-default | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:21.449Z | button-color-gold | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:22.000Z | button-native-unavailable | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:08:22.643Z | button-scale-110 | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:08:23.344Z | button-theme-full | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:08:23.985Z | button-panel-chinese | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:08:34.358Z | — | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:44.586Z | — | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:45.185Z | — | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:08:45.752Z | type-link | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T14:08:56.239Z | — | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:06.986Z | button-classic-2 | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:07.552Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:08.118Z | button-soft-2 | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:08.685Z | button-surface-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:09.252Z | button-outline-2 | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:09.818Z | button-ghost-offset-2 | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:10.385Z | button-ghost-2 | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:10.954Z | button-color-red | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:11.518Z | button-high-contrast | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:12.085Z | button-icon | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T14:09:12.701Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T14:09:13.268Z | button-soft-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T14:09:13.835Z | button-outline-2 | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T14:09:24.134Z | type-link | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10322} root {"left":24,"top":1517.953125,"right":1256,"bottom":11822.578125} | 2026-10-28T14:09:34.589Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:35.188Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:35.777Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:36.364Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:36.957Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:37.534Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:38.123Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:38.710Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:39.290Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:39.892Z | — | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10931} root {"left":24,"top":1517.953125,"right":1256,"bottom":12431.90625} | 2026-10-28T14:09:40.701Z | layout-scale-110 | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:41.305Z | layout-scale-110 | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10024} root {"left":24,"top":1517.953125,"right":1256,"bottom":11524.25} | 2026-10-28T14:09:42.104Z | layout-scale-95 | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T14:09:42.700Z | layout-scale-95 | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10359} root {"left":24,"top":1517.953125,"right":1256,"bottom":11859.578125} | 2026-10-28T14:09:43.562Z | layout-invalid | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T14:09:44.111Z | layout-invalid | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15447} root {"left":12,"top":2452.453125,"right":378,"bottom":17882.5625} | 2026-10-28T14:09:54.517Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:55.113Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:55.707Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:56.292Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:56.869Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:57.450Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:58.040Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:58.620Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:59.203Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T14:09:59.797Z | — | none |

## U2 缺证补采（第 4 次运行）

- 应独立评审的缺证说明，增加分组重置影响列表展开首尾两帧；原图与原单元清单不覆盖。重评采用 capture-U2-b.md，补采轨迹为 U2-supplement-actions.log。

| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U2 | group-reset-tokens | [U2-group-reset-tokens-1440-light.png](U2-group-reset-tokens-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:13:48.264Z | group-override-tokens | none |
| U2 | group-reset-tokens-end | [U2-group-reset-tokens-end-1440-light.png](U2-group-reset-tokens-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T14:13:48.828Z | group-override-tokens | none |

## 独立评审交卷后的页面实测截图

这些图是主 agent 机器检查的真实页面证据，不提供给独立评审、不进入独立评分。完整登记见 [machine-captures.json](machine-captures.json)。

| 图像 | URL | 视口 / DPR | 主题 | 落盘 UTC |
| --- | --- | --- | --- | --- |
| [MC-export-color-format-keyboard-focus-1440-light.png](MC-export-color-format-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:17.597715+00:00 |
| [MC-export-format-pure-keyboard-1440-light.png](MC-export-format-pure-keyboard-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:15.238608+00:00 |
| [MC-export-kind-keyboard-focus-1440-light.png](MC-export-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:38:14.579036+00:00 |
| [MC-export-next-control-keyboard-1440-light.png](MC-export-next-control-keyboard-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:16.001756+00:00 |
| [MC-export-profile-keyboard-focus-1440-light.png](MC-export-profile-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:16.846858+00:00 |
| [MC-import-format-pure-keyboard-1440-light.png](MC-import-format-pure-keyboard-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:24.836777+00:00 |
| [MC-import-kind-keyboard-focus-1440-light.png](MC-import-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:38:21.792328+00:00 |
| [MC-import-next-control-keyboard-1440-light.png](MC-import-next-control-keyboard-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:40:25.569743+00:00 |
| [MC-root200-nested-surface-720-light.png](MC-root200-nested-surface-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | 2026-10-28T14:38:44.605882+00:00 |
| [MC-root200-preview-clip-720-light.png](MC-root200-preview-clip-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | 2026-10-28T14:35:23.128096+00:00 |
| [MC-token-reset-keyboard-focus-1440-light.png](MC-token-reset-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | 2026-10-28T14:38:29.574246+00:00 |
