# UI 验收 R8 · U7 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

- 补采环境：初次对应流程因构建输出目录缺失而收到 HTTP 404，未拍到的格子登记为 attempt 1 缺证；本轮从同一源码重建，两个 build_id 与初采一致，逐文件校验后复制到独立临时目录，仍通过仓库 `mbx xtask serve --dir ... --csp strict` 在相同 URL 服务。文件名 `-a2` 为这些格子的第二次采集；视口、DPR、语言、主题、字体等待及操作步骤保持原条件。原图和失败轨迹保留。

共 42 张。操作记录：[U7-actions.log](U7-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10322} root {"left":24,"top":1517.953125,"right":1256,"bottom":11822.578125} | 2026-10-29T09:53:15.634Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:16.235Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:16.841Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:17.442Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:18.784Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:19.359Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:19.950Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:20.540Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:21.132Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:21.748Z | — | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10931} root {"left":24,"top":1517.953125,"right":1256,"bottom":12431.90625} | 2026-10-29T09:53:22.667Z | layout-scale-110 | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:23.286Z | layout-scale-110 | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10024} root {"left":24,"top":1517.953125,"right":1256,"bottom":11524.25} | 2026-10-29T09:53:24.159Z | layout-scale-95 | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T09:53:24.775Z | layout-scale-95 | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10359} root {"left":24,"top":1517.953125,"right":1256,"bottom":11859.578125} | 2026-10-29T09:53:25.794Z | layout-invalid | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-29T09:53:26.340Z | layout-invalid | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15447} root {"left":12,"top":2452.453125,"right":378,"bottom":17882.5625} | 2026-10-29T09:53:40.985Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:41.590Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:42.181Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:42.785Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:43.374Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:43.954Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:44.547Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:45.141Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:45.724Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:53:46.319Z | — | none |
| U7 | matrix-scale110 | [U7-matrix-scale110-390-light.png](U7-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":16691} root {"left":12,"top":2452.453125,"right":378,"bottom":19126.53125} | 2026-10-29T09:58:46.724Z | layout-scale-110 | none |
| U7 | matrix-scale110-heading | [U7-matrix-scale110-heading-390-light.png](U7-matrix-scale110-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:47.336Z | layout-scale-110 | none |
| U7 | matrix-scale110-kbd | [U7-matrix-scale110-kbd-390-light.png](U7-matrix-scale110-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:47.915Z | layout-scale-110 | none |
| U7 | matrix-scale95 | [U7-matrix-scale95-390-light.png](U7-matrix-scale95-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":14882} root {"left":12,"top":2452.453125,"right":378,"bottom":17317.90625} | 2026-10-29T09:58:48.704Z | layout-scale-95 | none |
| U7 | matrix-scale95-heading | [U7-matrix-scale95-heading-390-light.png](U7-matrix-scale95-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:49.294Z | layout-scale-95 | none |
| U7 | matrix-scale95-kbd | [U7-matrix-scale95-kbd-390-light.png](U7-matrix-scale95-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T09:58:49.881Z | layout-scale-95 | none |
| U7 | matrix-invalid | [U7-matrix-invalid-390-light.png](U7-matrix-invalid-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15505} root {"left":12,"top":2452.453125,"right":378,"bottom":17940.5625} | 2026-10-29T09:58:50.810Z | layout-invalid | none |
| U7 | matrix-invalid-detail | [U7-matrix-invalid-detail-390-light.png](U7-matrix-invalid-detail-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-29T09:58:51.349Z | layout-invalid | none |
| U7 | supp-default-primitives | [U7-supp-default-primitives-1280-light-a2.png](U7-supp-default-primitives-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T10:15:46.694Z | layout-scale-100 | none |
| U7 | supp-default-mixed | [U7-supp-default-mixed-1280-light-a2.png](U7-supp-default-mixed-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":36,"y":6657,"width":1208,"height":724} | 2026-10-29T10:15:47.279Z | layout-scale-100 | none |
| U7 | supp-scale110-primitives | [U7-supp-scale110-primitives-1280-light-a2.png](U7-supp-scale110-primitives-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-29T10:15:48.054Z | layout-scale-110 | none |
| U7 | supp-scale110-mixed | [U7-supp-scale110-mixed-1280-light-a2.png](U7-supp-scale110-mixed-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":37,"y":6951,"width":1205,"height":789} | 2026-10-29T10:15:48.615Z | layout-scale-110 | none |
| U7 | supp-default-primitives | [U7-supp-default-primitives-390-light-a2.png](U7-supp-default-primitives-390-light-a2.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T10:16:01.693Z | layout-scale-100 | none |
| U7 | supp-default-mixed | [U7-supp-default-mixed-390-light-a2.png](U7-supp-default-mixed-390-light-a2.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":24,"y":8823,"width":342,"height":1050} | 2026-10-29T10:16:02.246Z | layout-scale-100 | none |
| U7 | supp-scale110-primitives | [U7-supp-scale110-primitives-390-light-a2.png](U7-supp-scale110-primitives-390-light-a2.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-29T10:16:02.968Z | layout-scale-110 | none |
| U7 | supp-scale110-mixed | [U7-supp-scale110-mixed-390-light-a2.png](U7-supp-scale110-mixed-390-light-a2.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":25,"y":9208,"width":339,"height":1272} | 2026-10-29T10:16:03.534Z | layout-scale-110 | none |
