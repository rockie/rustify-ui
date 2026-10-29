# UI 验收 R7 · U1 采集清单

- 被测版本：HEAD `b218075a3a2107ab511da932f831f42adfa9390c`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `1fdbbbc39758b56531e996384ad7397645d59a7115ea3fcb6c1b39b326a98600`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `6d1605ccec8b9f78`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R6 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r6.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R6，无 managed 索引；本轮 R7，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 62 张。操作记录：[U1-actions.log](U1-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:21:34.072Z | — | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:21:35.301Z | studio-config-gray_color | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:21:35.886Z | mode-dark | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:21:36.450Z | mode-dark | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:21:37.148Z | locale-zh | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:21:37.699Z | locale-zh | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:21:46.870Z | — | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:21:48.007Z | studio-config-gray_color | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:21:49.929Z | — | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:21:50.831Z | studio-config-gray_color | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:22:01.643Z | mobile-preview | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:22:11.809Z | — | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:22:12.414Z | data-tools-dialog-close | none |
| U1 | zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:22:21.825Z | editor-colors | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:22:22.439Z | color-token-primary | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:22:31.810Z | preset-search | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:22:32.376Z | studio-config-accent_color | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:22:32.945Z | studio-config-accent_color-list | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:22:33.680Z | mode-dark | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:27:20.356Z | data-tools-dialog-close | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:29.466Z | — | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:30.029Z | editor-colors | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:31.150Z | studio-config-gray_color | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:29:11.387Z | color-token-primary | none |
| U1 | zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-29T02:29:12.457Z | color-token-primary | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-768-light.png](U1-matrix-zh-top-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:36.465Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-768-light.png](U1-matrix-zh-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:37.028Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-768-light.png](U1-matrix-keyboard-01-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:37.640Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-768-light.png](U1-matrix-keyboard-02-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:38.191Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-768-light.png](U1-matrix-keyboard-03-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:38.740Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-768-light.png](U1-matrix-keyboard-04-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:30:39.291Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-390-light.png](U1-matrix-zh-top-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:41.470Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-390-light.png](U1-matrix-zh-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:42.023Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-390-light.png](U1-matrix-keyboard-01-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:42.621Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-390-light.png](U1-matrix-keyboard-02-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:43.171Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-390-light.png](U1-matrix-keyboard-03-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:43.704Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-390-light.png](U1-matrix-keyboard-04-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:30:44.255Z | mode-dark | none |
| U1 | matrix-default | [U1-matrix-default-1440-dark.png](U1-matrix-default-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:30:53.186Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-1440-dark.png](U1-matrix-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:30:54.271Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-1440-dark.png](U1-matrix-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:30:54.837Z | locale-zh | none |
| U1 | matrix2-zh-top | [U1-matrix2-zh-top-1440-dark.png](U1-matrix2-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:11.515Z | locale-zh | none |
| U1 | matrix2-zh-configured | [U1-matrix2-zh-configured-1440-dark.png](U1-matrix2-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:12.080Z | locale-zh | none |
| U1 | matrix2-keyboard-01 | [U1-matrix2-keyboard-01-1440-dark.png](U1-matrix2-keyboard-01-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:12.709Z | preset-search | none |
| U1 | matrix2-keyboard-02 | [U1-matrix2-keyboard-02-1440-dark.png](U1-matrix2-keyboard-02-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:13.277Z | studio-config-accent_color | none |
| U1 | matrix2-keyboard-03 | [U1-matrix2-keyboard-03-1440-dark.png](U1-matrix2-keyboard-03-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:13.842Z | studio-config-accent_color-list | none |
| U1 | matrix2-keyboard-04 | [U1-matrix2-keyboard-04-1440-dark.png](U1-matrix2-keyboard-04-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:33:14.413Z | mode-dark | none |
| U1 | matrix3-zh-preview | [U1-matrix3-zh-preview-390-light.png](U1-matrix3-zh-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:33:56.128Z | mobile-preview | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-1440-dark.png](U1-configured-keyboard-open-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:34:13.626Z | studio-config-accent_color | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-1440-dark.png](U1-configured-keyboard-down-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:34:14.190Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-1440-dark.png](U1-configured-keyboard-down5-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:34:14.757Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-1440-dark.png](U1-configured-keyboard-selected-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:34:15.391Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-1440-dark.png](U1-configured-keyboard-escape-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T02:34:15.958Z | studio-config-accent_color | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-768-light.png](U1-configured-keyboard-open-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:34:25.893Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-768-light.png](U1-configured-keyboard-down-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:34:26.443Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-768-light.png](U1-configured-keyboard-down5-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:34:26.992Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-768-light.png](U1-configured-keyboard-selected-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:34:27.544Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-768-light.png](U1-configured-keyboard-escape-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-29T02:34:28.094Z | studio-config-accent_color | none |
| U1 | configured-keyboard-open | [U1-configured-keyboard-open-390-light.png](U1-configured-keyboard-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:30.230Z | studio-config-accent_color | none |
| U1 | configured-keyboard-down | [U1-configured-keyboard-down-390-light.png](U1-configured-keyboard-down-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:30.762Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-down5 | [U1-configured-keyboard-down5-390-light.png](U1-configured-keyboard-down5-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:31.296Z | studio-config-accent_color-list | none |
| U1 | configured-keyboard-selected | [U1-configured-keyboard-selected-390-light.png](U1-configured-keyboard-selected-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:31.846Z | studio-config-accent_color | none |
| U1 | configured-keyboard-escape | [U1-configured-keyboard-escape-390-light.png](U1-configured-keyboard-escape-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:32.396Z | studio-config-accent_color | none |
