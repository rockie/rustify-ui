# UI 验收 R8 · U2 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 24 张。操作记录：[U2-actions.log](U2-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:02.116Z | color-token-primary | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:02.685Z | color-token-primary | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:04.687Z | — | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:05.496Z | — | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:06.111Z | color-token-ring | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:06.711Z | editor-controls | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:07.294Z | edit-radius | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:18.380Z | editor-shell | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:49:20.750Z | color-token-primary | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:55:15.345Z | studio-config-gray_color | none |
| U2 | search-row | [U2-search-row-1440-light.png](U2-search-row-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:00.539Z | editor-colors | none |
| U2 | group-reset-tokens | [U2-group-reset-tokens-1440-light.png](U2-group-reset-tokens-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:10.447Z | group-override-tokens | none |
| U2 | group-reset-tokens-end | [U2-group-reset-tokens-end-1440-light.png](U2-group-reset-tokens-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:11.015Z | group-override-tokens | none |
| U2 | token-reset-keyboard-focus | [U2-token-reset-keyboard-focus-1440-light.png](U2-token-reset-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:38.416Z | color-reset-primary | none |
| U2 | matrix-source-overridden | [U2-matrix-source-overridden-390-light.png](U2-matrix-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:10.082Z | color-token-primary | none |
| U2 | matrix-token-released | [U2-matrix-token-released-390-light.png](U2-matrix-token-released-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:10.668Z | — | none |
| U2 | matrix-group-reset-tokens | [U2-matrix-group-reset-tokens-390-light.png](U2-matrix-group-reset-tokens-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:11.260Z | group-override-tokens | none |
| U2 | matrix-group-reset-tokens-end | [U2-matrix-group-reset-tokens-end-390-light.png](U2-matrix-group-reset-tokens-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:11.810Z | group-override-tokens | none |
| U2 | matrix-group-reset | [U2-matrix-group-reset-390-light.png](U2-matrix-group-reset-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:12.376Z | — | none |
| U2 | matrix-source-generated | [U2-matrix-source-generated-390-light.png](U2-matrix-source-generated-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:13.466Z | color-token-ring | none |
| U2 | matrix-controls | [U2-matrix-controls-390-light.png](U2-matrix-controls-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:14.061Z | editor-controls | none |
| U2 | matrix-field-error | [U2-matrix-field-error-390-light.png](U2-matrix-field-error-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:14.624Z | edit-radius | none |
| U2 | matrix-low-contrast | [U2-matrix-low-contrast-390-light.png](U2-matrix-low-contrast-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:16.751Z | editor-shell | none |
| U2 | matrix3-low-contrast-preview | [U2-matrix3-low-contrast-preview-390-light.png](U2-matrix3-low-contrast-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:36.905Z | mobile-preview | none |
