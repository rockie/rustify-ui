# UI 验收 R6 · U7 补采后完整采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。


- 本单元当前共 34 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。

- 实际操作：[记录](U7-actions-b.log)。

| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:48.075Z | page-clip {"x":16,"y":1509,"width":1248,"height":10322} root {"left":24,"top":1517.953125,"right":1256,"bottom":11822.578125} | none |
| type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:48.675Z | element | none |
| type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:49.265Z | element | none |
| type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:49.850Z | element | none |
| type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:50.444Z | element | none |
| type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:51.021Z | element | none |
| type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:51.618Z | element | none |
| layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:52.213Z | element | none |
| layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:52.793Z | element | none |
| layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:53.398Z | element | none |
| scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:54.241Z | page-clip {"x":16,"y":1509,"width":1248,"height":10931} root {"left":24,"top":1517.953125,"right":1256,"bottom":12431.90625} | none |
| scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:54.841Z | element | none |
| scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:55.660Z | page-clip {"x":16,"y":1509,"width":1248,"height":10024} root {"left":24,"top":1517.953125,"right":1256,"bottom":11524.25} | none |
| scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:56.257Z | element | none |
| invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:57.117Z | page-clip {"x":16,"y":1509,"width":1248,"height":10359} root {"left":24,"top":1517.953125,"right":1256,"bottom":11859.578125} | none |
| invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:13:57.665Z | viewport | none |
| fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:08.595Z | page-clip {"x":4,"y":2444,"width":382,"height":15447} root {"left":12,"top":2452.453125,"right":378,"bottom":17882.5625} | none |
| type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:09.191Z | element | none |
| type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:09.780Z | element | none |
| type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:10.385Z | element | none |
| type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:10.973Z | element | none |
| type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:11.553Z | element | none |
| type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:12.144Z | element | none |
| layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:12.723Z | element | none |
| layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:13.306Z | element | none |
| layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:14:13.902Z | element | none |
| matrix-scale110 | [U7-matrix-scale110-390-light.png](U7-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:09.909Z | page-clip {"x":4,"y":2444,"width":382,"height":16691} root {"left":12,"top":2452.453125,"right":378,"bottom":19126.53125} | none |
| matrix-scale110-heading | [U7-matrix-scale110-heading-390-light.png](U7-matrix-scale110-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:10.518Z | element | none |
| matrix-scale110-kbd | [U7-matrix-scale110-kbd-390-light.png](U7-matrix-scale110-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:11.100Z | element | none |
| matrix-scale95 | [U7-matrix-scale95-390-light.png](U7-matrix-scale95-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:11.850Z | page-clip {"x":4,"y":2444,"width":382,"height":14882} root {"left":12,"top":2452.453125,"right":378,"bottom":17317.90625} | none |
| matrix-scale95-heading | [U7-matrix-scale95-heading-390-light.png](U7-matrix-scale95-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:12.439Z | element | none |
| matrix-scale95-kbd | [U7-matrix-scale95-kbd-390-light.png](U7-matrix-scale95-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:13.030Z | element | none |
| matrix-invalid | [U7-matrix-invalid-390-light.png](U7-matrix-invalid-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:13.820Z | page-clip {"x":4,"y":2444,"width":382,"height":15505} root {"left":12,"top":2452.453125,"right":378,"bottom":17940.5625} | none |
| matrix-invalid-detail | [U7-matrix-invalid-detail-390-light.png](U7-matrix-invalid-detail-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:54:14.364Z | viewport | none |
