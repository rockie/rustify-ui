# UI 验收 R8 · U1 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 62 张。操作记录：[U1-actions.log](U1-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:47:45.538Z | — | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:47:47.813Z | studio-config-gray_color | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:47:48.569Z | mode-dark | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:47:49.144Z | mode-dark | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:47:50.084Z | locale-zh | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:47:50.685Z | locale-zh | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:48:00.878Z | — | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:48:02.063Z | studio-config-gray_color | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:48:03.993Z | — | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:48:04.876Z | studio-config-gray_color | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:48:12.971Z | mobile-preview | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:48:23.520Z | — | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:48:25.566Z | data-tools-dialog-close | none |
| U1 | zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:48:36.393Z | editor-colors | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:48:37.011Z | color-token-primary | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:48:47.807Z | preset-search | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:48:48.384Z | studio-config-accent_color | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:48:49.933Z | studio-config-accent_color-list | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:48:50.676Z | mode-dark | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:53:55.814Z | data-tools-dialog-close | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:05.103Z | — | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:05.669Z | editor-colors | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:06.885Z | studio-config-gray_color | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:55:50.214Z | color-token-primary | none |
| U1 | zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T09:55:51.281Z | color-token-primary | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-768-light.png](U1-matrix-zh-top-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:48.607Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-768-light.png](U1-matrix-zh-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:49.173Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-768-light.png](U1-matrix-keyboard-01-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:49.787Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-768-light.png](U1-matrix-keyboard-02-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:50.338Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-768-light.png](U1-matrix-keyboard-03-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:50.886Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-768-light.png](U1-matrix-keyboard-04-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T09:56:51.438Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-390-light.png](U1-matrix-zh-top-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:53.717Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-390-light.png](U1-matrix-zh-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:54.269Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-390-light.png](U1-matrix-keyboard-01-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:54.865Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-390-light.png](U1-matrix-keyboard-02-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:55.399Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-390-light.png](U1-matrix-keyboard-03-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:55.948Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-390-light.png](U1-matrix-keyboard-04-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:56:56.482Z | mode-dark | none |
| U1 | matrix-default | [U1-matrix-default-1440-dark.png](U1-matrix-default-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:05.533Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-1440-dark.png](U1-matrix-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:06.717Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-1440-dark.png](U1-matrix-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:07.286Z | locale-zh | none |
| U1 | matrix2-zh-top | [U1-matrix2-zh-top-1440-dark.png](U1-matrix2-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:01.310Z | locale-zh | none |
| U1 | matrix2-zh-configured | [U1-matrix2-zh-configured-1440-dark.png](U1-matrix2-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:01.878Z | locale-zh | none |
| U1 | matrix2-keyboard-01 | [U1-matrix2-keyboard-01-1440-dark.png](U1-matrix2-keyboard-01-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:02.506Z | preset-search | none |
| U1 | matrix2-keyboard-02 | [U1-matrix2-keyboard-02-1440-dark.png](U1-matrix2-keyboard-02-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:03.076Z | studio-config-accent_color | none |
| U1 | matrix2-keyboard-03 | [U1-matrix2-keyboard-03-1440-dark.png](U1-matrix2-keyboard-03-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:03.642Z | studio-config-accent_color-list | none |
| U1 | matrix2-keyboard-04 | [U1-matrix2-keyboard-04-1440-dark.png](U1-matrix2-keyboard-04-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:59:04.209Z | mode-dark | none |
| U1 | matrix3-zh-preview | [U1-matrix3-zh-preview-390-light.png](U1-matrix3-zh-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:46.496Z | mobile-preview | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-1440-dark.png](U1-configured-keyboard-open-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T10:00:04.483Z | studio-config-accent_color | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-1440-dark.png](U1-configured-keyboard-down-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T10:00:05.050Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-1440-dark.png](U1-configured-keyboard-down5-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T10:00:05.618Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-1440-dark.png](U1-configured-keyboard-selected-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T10:00:06.183Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-1440-dark.png](U1-configured-keyboard-escape-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T10:00:06.752Z | studio-config-accent_color | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-768-light.png](U1-configured-keyboard-open-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T10:00:38.168Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-768-light.png](U1-configured-keyboard-down-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T10:00:38.740Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-768-light.png](U1-configured-keyboard-down5-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T10:00:39.321Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-768-light.png](U1-configured-keyboard-selected-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T10:00:40.346Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-768-light.png](U1-configured-keyboard-escape-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T10:00:40.923Z | studio-config-accent_color | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-390-light.png](U1-configured-keyboard-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:00:43.372Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-390-light.png](U1-configured-keyboard-down-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:00:43.944Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-390-light.png](U1-configured-keyboard-down5-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:00:44.524Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-390-light.png](U1-configured-keyboard-selected-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:00:45.148Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-390-light.png](U1-configured-keyboard-escape-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:00:45.722Z | studio-config-accent_color | none |
