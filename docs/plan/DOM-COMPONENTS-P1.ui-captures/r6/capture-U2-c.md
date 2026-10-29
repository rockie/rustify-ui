# UI 验收 R6 · U2 补采后完整采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。


- 本单元当前共 24 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。

- 实际操作：[记录](U2-actions-c.log)。

| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:51.338Z | viewport | none |
| source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:51.894Z | viewport | none |
| token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:52.478Z | viewport | none |
| group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:53.059Z | viewport | none |
| source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:53.655Z | viewport | none |
| controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:54.257Z | viewport | none |
| field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:09:54.838Z | viewport | none |
| low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:10:04.580Z | viewport | none |
| source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:10:06.918Z | viewport | none |
| restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:15:30.030Z | viewport | none |
| search-row | [U2-search-row-1440-light.png](U2-search-row-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:16:11.333Z | viewport | none |
| group-reset-tokens | [U2-group-reset-tokens-1440-light.png](U2-group-reset-tokens-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:16:19.826Z | viewport | none |
| group-reset-tokens-end | [U2-group-reset-tokens-end-1440-light.png](U2-group-reset-tokens-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:16:20.376Z | viewport | none |
| token-reset-keyboard-focus | [U2-token-reset-keyboard-focus-1440-light.png](U2-token-reset-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:18:01.260Z | viewport | none |
| matrix-source-overridden | [U2-matrix-source-overridden-390-light.png](U2-matrix-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:13.994Z | viewport | none |
| matrix-token-released | [U2-matrix-token-released-390-light.png](U2-matrix-token-released-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:14.562Z | viewport | none |
| matrix-group-reset-tokens | [U2-matrix-group-reset-tokens-390-light.png](U2-matrix-group-reset-tokens-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:15.157Z | viewport | none |
| matrix-group-reset-tokens-end | [U2-matrix-group-reset-tokens-end-390-light.png](U2-matrix-group-reset-tokens-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:15.706Z | viewport | none |
| matrix-group-reset | [U2-matrix-group-reset-390-light.png](U2-matrix-group-reset-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:16.273Z | viewport | none |
| matrix-source-generated | [U2-matrix-source-generated-390-light.png](U2-matrix-source-generated-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:17.361Z | viewport | none |
| matrix-controls | [U2-matrix-controls-390-light.png](U2-matrix-controls-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:17.939Z | viewport | none |
| matrix-field-error | [U2-matrix-field-error-390-light.png](U2-matrix-field-error-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:18.505Z | viewport | none |
| matrix-low-contrast | [U2-matrix-low-contrast-390-light.png](U2-matrix-low-contrast-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:20.557Z | viewport | none |
| matrix3-low-contrast-preview | [U2-matrix3-low-contrast-preview-390-light.png](U2-matrix3-low-contrast-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T17:02:21.020Z | viewport | none |
