# UI 验收 R6 · U1 补采后完整采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。


- 本单元当前共 46 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。

- 实际操作：[记录](U1-actions-c.log)。

| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:08:52.588Z | viewport | none |
| configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:08:53.755Z | viewport | none |
| configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:08:54.337Z | viewport | none |
| top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:08:54.886Z | viewport | none |
| zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:08:55.603Z | viewport | none |
| zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:08:56.154Z | viewport | none |
| default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:09:04.765Z | viewport | none |
| configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:09:05.999Z | viewport | none |
| edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:09:07.920Z | viewport | none |
| configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:09:08.736Z | viewport | none |
| preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:09:15.019Z | viewport | none |
| zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:09:22.969Z | viewport | none |
| zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:09:23.554Z | viewport | none |
| zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:09:31.771Z | viewport | none |
| zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:09:32.387Z | viewport | none |
| keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:40.965Z | viewport | none |
| keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:41.516Z | viewport | none |
| keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:42.066Z | viewport | none |
| keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:42.729Z | viewport | none |
| zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:14:21.591Z | viewport | none |
| field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:14:30.207Z | viewport | none |
| field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:14:30.771Z | viewport | none |
| configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:14:31.873Z | viewport | none |
| zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:16:02.034Z | viewport | none |
| zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | 2026-10-28T16:16:03.101Z | viewport | none |
| matrix-zh-top | [U1-matrix-zh-top-768-light.png](U1-matrix-zh-top-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:54.329Z | viewport | none |
| matrix-zh-configured | [U1-matrix-zh-configured-768-light.png](U1-matrix-zh-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:54.891Z | viewport | none |
| matrix-keyboard-01 | [U1-matrix-keyboard-01-768-light.png](U1-matrix-keyboard-01-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:55.508Z | viewport | none |
| matrix-keyboard-02 | [U1-matrix-keyboard-02-768-light.png](U1-matrix-keyboard-02-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:56.058Z | viewport | none |
| matrix-keyboard-03 | [U1-matrix-keyboard-03-768-light.png](U1-matrix-keyboard-03-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:56.607Z | viewport | none |
| matrix-keyboard-04 | [U1-matrix-keyboard-04-768-light.png](U1-matrix-keyboard-04-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | 2026-10-28T16:50:57.155Z | viewport | none |
| matrix-zh-top | [U1-matrix-zh-top-390-light.png](U1-matrix-zh-top-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:50:59.331Z | viewport | none |
| matrix-zh-configured | [U1-matrix-zh-configured-390-light.png](U1-matrix-zh-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:50:59.881Z | viewport | none |
| matrix-keyboard-01 | [U1-matrix-keyboard-01-390-light.png](U1-matrix-keyboard-01-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:51:00.465Z | viewport | none |
| matrix-keyboard-02 | [U1-matrix-keyboard-02-390-light.png](U1-matrix-keyboard-02-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:51:00.999Z | viewport | none |
| matrix-keyboard-03 | [U1-matrix-keyboard-03-390-light.png](U1-matrix-keyboard-03-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:51:01.532Z | viewport | none |
| matrix-keyboard-04 | [U1-matrix-keyboard-04-390-light.png](U1-matrix-keyboard-04-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:51:02.066Z | viewport | none |
| matrix-default | [U1-matrix-default-1440-dark.png](U1-matrix-default-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:51:10.541Z | viewport | none |
| matrix-zh-top | [U1-matrix-zh-top-1440-dark.png](U1-matrix-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:51:11.163Z | viewport | none |
| matrix2-zh-top | [U1-matrix2-zh-top-1440-dark.png](U1-matrix2-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:15.582Z | viewport | none |
| matrix2-zh-configured | [U1-matrix2-zh-configured-1440-dark.png](U1-matrix2-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:16.144Z | viewport | none |
| matrix2-keyboard-01 | [U1-matrix2-keyboard-01-1440-dark.png](U1-matrix2-keyboard-01-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:16.756Z | viewport | none |
| matrix2-keyboard-02 | [U1-matrix2-keyboard-02-1440-dark.png](U1-matrix2-keyboard-02-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:17.306Z | viewport | none |
| matrix2-keyboard-03 | [U1-matrix2-keyboard-03-1440-dark.png](U1-matrix2-keyboard-03-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:17.857Z | viewport | none |
| matrix2-keyboard-04 | [U1-matrix2-keyboard-04-1440-dark.png](U1-matrix2-keyboard-04-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:55:18.408Z | viewport | none |
| matrix3-zh-preview | [U1-matrix3-zh-preview-390-light.png](U1-matrix3-zh-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T17:04:05.459Z | viewport | none |
