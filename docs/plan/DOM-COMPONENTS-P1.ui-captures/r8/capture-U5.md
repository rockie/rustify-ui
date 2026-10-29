# UI 验收 R8 · U5 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 39 张。操作记录：[U5-actions.log](U5-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:44.355Z | renderer-dom | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:50:44.954Z | mode-dark | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:45.652Z | local-appearance | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:46.302Z | menu-item-duplicate | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:46.886Z | preview-dialog-close | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:47.552Z | compare-dark | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:48.288Z | renderer-compare | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:58.503Z | inspector-toggle | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:59.136Z | inspector-toggle | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:50:59.758Z | color-value | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:51:10.266Z | inspector-toggle | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:51:11.367Z | inspector-toggle | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:51:21.888Z | mobile-preview | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:51:22.470Z | preview-dialog-close | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:54:41.836Z | mode-dark | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:52.311Z | menu-item-duplicate | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:52.909Z | preview-dialog-close | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:55:04.367Z | inspector-toggle | none |
| U5 | matrix-dom | [U5-matrix-dom-1440-dark.png](U5-matrix-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:26.138Z | mode-dark | none |
| U5 | matrix-local-opposite | [U5-matrix-local-opposite-1440-dark.png](U5-matrix-local-opposite-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:26.803Z | local-appearance | none |
| U5 | matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-1440-dark.png](U5-matrix-local-opposite-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:27.384Z | menu-item-duplicate | none |
| U5 | matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-1440-dark.png](U5-matrix-local-opposite-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:27.966Z | preview-dialog-close | none |
| U5 | matrix-menu | [U5-matrix-menu-1440-dark.png](U5-matrix-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:28.618Z | menu-item-duplicate | none |
| U5 | matrix-dialog | [U5-matrix-dialog-1440-dark.png](U5-matrix-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:29.203Z | preview-dialog-close | none |
| U5 | matrix-compare-nested | [U5-matrix-compare-nested-1440-dark.png](U5-matrix-compare-nested-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:29.850Z | compare-light | none |
| U5 | matrix-inspector | [U5-matrix-inspector-1440-dark.png](U5-matrix-inspector-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:31.304Z | inspector-toggle | none |
| U5 | matrix-inspector-picked | [U5-matrix-inspector-picked-1440-dark.png](U5-matrix-inspector-picked-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:32.443Z | color-value | none |
| U5 | matrix-gpu-compare | [U5-matrix-gpu-compare-1440-dark.png](U5-matrix-gpu-compare-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-29T09:57:33.205Z | renderer-compare | none |
| U5 | matrix-dom | [U5-matrix-dom-390-light.png](U5-matrix-dom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:42.294Z | renderer-dom | none |
| U5 | matrix-local-opposite | [U5-matrix-local-opposite-390-light.png](U5-matrix-local-opposite-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:42.942Z | local-appearance | none |
| U5 | matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-390-light.png](U5-matrix-local-opposite-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:43.508Z | menu-item-duplicate | none |
| U5 | matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-390-light.png](U5-matrix-local-opposite-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:44.076Z | preview-dialog-close | none |
| U5 | matrix-menu | [U5-matrix-menu-390-light.png](U5-matrix-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:44.725Z | menu-item-duplicate | none |
| U5 | matrix-dialog | [U5-matrix-dialog-390-light.png](U5-matrix-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:45.292Z | preview-dialog-close | none |
| U5 | matrix-compare-nested | [U5-matrix-compare-nested-390-light.png](U5-matrix-compare-nested-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:45.909Z | compare-dark | none |
| U5 | matrix-inspector | [U5-matrix-inspector-390-light.png](U5-matrix-inspector-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:47.342Z | inspector-toggle | none |
| U5 | matrix-inspector-picked | [U5-matrix-inspector-picked-390-light.png](U5-matrix-inspector-picked-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:48.469Z | color-value | none |
| U5 | matrix-gpu-compare | [U5-matrix-gpu-compare-390-light.png](U5-matrix-gpu-compare-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:49.292Z | renderer-compare | none |
| U5 | matrix-gpu-compare-end | [U5-matrix-gpu-compare-end-390-light.png](U5-matrix-gpu-compare-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:57:49.825Z | renderer-compare | none |
