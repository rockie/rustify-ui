# UI 验收 R8 · U6 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 50 张。操作记录：[U6-actions.log](U6-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:51:37.296Z | — | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:51:37.961Z | button-mode-dark | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:51:38.617Z | button-toggle-loading | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:51:39.238Z | button-default | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:51:39.786Z | button-color-gold | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:51:40.337Z | button-native-unavailable | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:51:40.976Z | button-scale-110 | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:51:41.689Z | button-theme-full | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:51:42.338Z | button-panel-chinese | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:51:55.694Z | — | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:52:09.957Z | — | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:52:10.559Z | — | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:52:11.123Z | type-link | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:52:24.973Z | — | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:40.397Z | button-classic-2 | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:40.973Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:42.230Z | button-soft-2 | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:42.963Z | button-surface-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:43.530Z | button-outline-2 | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:44.097Z | button-ghost-offset-2 | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:44.663Z | button-ghost-2 | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:45.230Z | button-color-red | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:45.797Z | button-high-contrast | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:52:46.363Z | button-icon | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-29T09:52:46.980Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-29T09:52:47.546Z | button-soft-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-29T09:52:48.113Z | button-outline-2 | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-29T09:53:01.363Z | type-link | none |
| U6 | matrix-fixture | [U6-matrix-fixture-390-light.png](U6-matrix-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:03.224Z | — | none |
| U6 | matrix-loading | [U6-matrix-loading-390-light.png](U6-matrix-loading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:03.859Z | button-toggle-loading | none |
| U6 | matrix-focus | [U6-matrix-focus-390-light.png](U6-matrix-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:58:04.463Z | button-default | none |
| U6 | matrix-focus-unavailable | [U6-matrix-focus-unavailable-390-light.png](U6-matrix-focus-unavailable-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:58:04.997Z | button-native-unavailable | none |
| U6 | matrix-scale110 | [U6-matrix-scale110-390-light.png](U6-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:05.633Z | button-scale-110 | none |
| U6 | matrix-radius-full | [U6-matrix-radius-full-390-light.png](U6-matrix-radius-full-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:06.328Z | button-theme-full | none |
| U6 | matrix-panel-zh | [U6-matrix-panel-zh-390-light.png](U6-matrix-panel-zh-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:06.968Z | button-panel-chinese | none |
| U6 | matrix-fixture | [U6-matrix-fixture-1280-dark.png](U6-matrix-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:58:19.942Z | button-mode-dark | none |
| U6 | matrix-loading | [U6-matrix-loading-1280-dark.png](U6-matrix-loading-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:58:20.562Z | button-toggle-loading | none |
| U6 | matrix-focus | [U6-matrix-focus-1280-dark.png](U6-matrix-focus-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | viewport | 2026-10-29T09:58:21.163Z | button-default | none |
| U6 | matrix-focus-unavailable | [U6-matrix-focus-unavailable-1280-dark.png](U6-matrix-focus-unavailable-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | viewport | 2026-10-29T09:58:21.720Z | button-native-unavailable | none |
| U6 | matrix-scale110 | [U6-matrix-scale110-1280-dark.png](U6-matrix-scale110-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:58:22.355Z | button-scale-110 | none |
| U6 | matrix-radius-full | [U6-matrix-radius-full-1280-dark.png](U6-matrix-radius-full-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:58:23.060Z | button-theme-full | none |
| U6 | matrix-panel-zh | [U6-matrix-panel-zh-1280-dark.png](U6-matrix-panel-zh-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-29T09:58:23.719Z | button-panel-chinese | none |
| U6 | matrix-link-studio | [U6-matrix-link-studio-1440-dark.png](U6-matrix-link-studio-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:58:33.577Z | mode-dark | none |
| U6 | matrix-link-studio-hover | [U6-matrix-link-studio-hover-1440-dark.png](U6-matrix-link-studio-hover-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:58:34.188Z | mode-dark | none |
| U6 | matrix-link-studio-focus | [U6-matrix-link-studio-focus-1440-dark.png](U6-matrix-link-studio-focus-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:58:34.753Z | cards-editor-link | none |
| U6 | matrix2-links-hover | [U6-matrix2-links-hover-390-light.png](U6-matrix2-links-hover-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:17.241Z | — | none |
| U6 | matrix2-links-focus | [U6-matrix2-links-focus-390-light.png](U6-matrix2-links-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:17.791Z | type-link | none |
| U6 | matrix2-link-studio | [U6-matrix2-link-studio-1280-dark.png](U6-matrix2-link-studio-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-29T09:59:27.132Z | mode-dark | none |
| U6 | matrix2-link-studio-hover | [U6-matrix2-link-studio-hover-1280-dark.png](U6-matrix2-link-studio-hover-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-29T09:59:27.730Z | mode-dark | none |
| U6 | matrix2-link-studio-focus | [U6-matrix2-link-studio-focus-1280-dark.png](U6-matrix2-link-studio-focus-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-29T09:59:28.298Z | cards-editor-link | none |
