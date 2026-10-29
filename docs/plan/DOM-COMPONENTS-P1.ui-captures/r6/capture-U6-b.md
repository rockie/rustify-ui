# UI 验收 R6 · U6 补采后完整采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。


- 本单元当前共 50 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。
- U6 深色 Link 由实际 Studio DOM 预览进入，拍 1280×900；catalog 布局夹具没有外观操作。仅改采集入口，不改组件或目标期望。
- 实际操作：[记录](U6-actions-b.log)。

| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:17.912Z | element | none |
| fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:12:18.609Z | element | none |
| loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:19.264Z | element | none |
| focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:19.885Z | viewport | none |
| focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:20.433Z | viewport | none |
| focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:20.985Z | viewport | none |
| scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:21.624Z | element | none |
| radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:22.365Z | element | none |
| panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:23.020Z | element | none |
| fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:12:36.784Z | element | none |
| links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:50.295Z | viewport | none |
| links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:50.895Z | viewport | none |
| links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | 2026-10-28T16:12:51.464Z | viewport | none |
| links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:13:04.766Z | viewport | none |
| focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:18.218Z | clip | none |
| focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:18.784Z | clip | none |
| focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:19.351Z | clip | none |
| focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:19.917Z | clip | none |
| focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:20.484Z | clip | none |
| focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:21.052Z | clip | none |
| focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:21.617Z | clip | none |
| focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:22.187Z | clip | none |
| focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:22.769Z | clip | none |
| focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:23.333Z | clip | none |
| focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | 2026-10-28T16:13:23.952Z | clip | none |
| focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | 2026-10-28T16:13:24.518Z | clip | none |
| focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | 2026-10-28T16:13:25.084Z | clip | none |
| focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | 2026-10-28T16:13:36.451Z | clip | none |
| matrix-fixture | [U6-matrix-fixture-390-light.png](U6-matrix-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:31.699Z | element | none |
| matrix-loading | [U6-matrix-loading-390-light.png](U6-matrix-loading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:32.324Z | element | none |
| matrix-focus | [U6-matrix-focus-390-light.png](U6-matrix-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:32.913Z | viewport | none |
| matrix-focus-unavailable | [U6-matrix-focus-unavailable-390-light.png](U6-matrix-focus-unavailable-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:33.444Z | viewport | none |
| matrix-scale110 | [U6-matrix-scale110-390-light.png](U6-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:34.083Z | element | none |
| matrix-radius-full | [U6-matrix-radius-full-390-light.png](U6-matrix-radius-full-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:34.765Z | element | none |
| matrix-panel-zh | [U6-matrix-panel-zh-390-light.png](U6-matrix-panel-zh-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:53:35.398Z | element | none |
| matrix-fixture | [U6-matrix-fixture-1280-dark.png](U6-matrix-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:45.928Z | element | none |
| matrix-loading | [U6-matrix-loading-1280-dark.png](U6-matrix-loading-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:46.560Z | element | none |
| matrix-focus | [U6-matrix-focus-1280-dark.png](U6-matrix-focus-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:47.156Z | viewport | none |
| matrix-focus-unavailable | [U6-matrix-focus-unavailable-1280-dark.png](U6-matrix-focus-unavailable-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:47.707Z | viewport | none |
| matrix-scale110 | [U6-matrix-scale110-1280-dark.png](U6-matrix-scale110-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:48.350Z | element | none |
| matrix-radius-full | [U6-matrix-radius-full-1280-dark.png](U6-matrix-radius-full-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:49.048Z | element | none |
| matrix-panel-zh | [U6-matrix-panel-zh-1280-dark.png](U6-matrix-panel-zh-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | 2026-10-28T16:53:49.694Z | element | none |
| matrix-link-studio | [U6-matrix-link-studio-1440-dark.png](U6-matrix-link-studio-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:53:58.030Z | viewport | none |
| matrix-link-studio-hover | [U6-matrix-link-studio-hover-1440-dark.png](U6-matrix-link-studio-hover-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:53:58.631Z | viewport | none |
| matrix-link-studio-focus | [U6-matrix-link-studio-focus-1440-dark.png](U6-matrix-link-studio-focus-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:53:59.179Z | viewport | none |
| matrix2-links-hover | [U6-matrix2-links-hover-390-light.png](U6-matrix2-links-hover-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:55:29.202Z | viewport | none |
| matrix2-links-focus | [U6-matrix2-links-focus-390-light.png](U6-matrix2-links-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | 2026-10-28T16:55:29.751Z | viewport | none |
| matrix2-link-studio | [U6-matrix2-link-studio-1280-dark.png](U6-matrix2-link-studio-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | 2026-10-28T16:55:37.818Z | viewport | none |
| matrix2-link-studio-hover | [U6-matrix2-link-studio-hover-1280-dark.png](U6-matrix2-link-studio-hover-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | 2026-10-28T16:55:38.419Z | viewport | none |
| matrix2-link-studio-focus | [U6-matrix2-link-studio-focus-1280-dark.png](U6-matrix2-link-studio-focus-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | 2026-10-28T16:55:38.987Z | viewport | none |
