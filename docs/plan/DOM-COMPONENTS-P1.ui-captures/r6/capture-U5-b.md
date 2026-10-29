# UI 验收 R6 · U5 补采后完整采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。


- 本单元当前共 39 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。

- 实际操作：[记录](U5-actions-b.log)。

| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:27.421Z | viewport | none |
| dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:11:28.007Z | viewport | none |
| local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:28.739Z | viewport | none |
| menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:29.390Z | viewport | none |
| dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:29.987Z | viewport | none |
| compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:30.655Z | viewport | none |
| gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:31.357Z | viewport | none |
| inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:41.665Z | viewport | none |
| inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:42.263Z | viewport | none |
| inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:42.869Z | viewport | none |
| inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:53.273Z | viewport | none |
| inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:11:54.357Z | viewport | none |
| preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:12:03.530Z | viewport | none |
| dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:12:04.097Z | viewport | none |
| dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:15:02.131Z | viewport | none |
| local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:15:11.275Z | viewport | none |
| local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:15:11.858Z | viewport | none |
| inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | 2026-10-28T16:15:21.314Z | viewport | none |
| matrix-dom | [U5-matrix-dom-1440-dark.png](U5-matrix-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:29.247Z | viewport | none |
| matrix-local-opposite | [U5-matrix-local-opposite-1440-dark.png](U5-matrix-local-opposite-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:29.897Z | viewport | none |
| matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-1440-dark.png](U5-matrix-local-opposite-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:30.481Z | viewport | none |
| matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-1440-dark.png](U5-matrix-local-opposite-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:31.062Z | viewport | none |
| matrix-menu | [U5-matrix-menu-1440-dark.png](U5-matrix-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:31.710Z | viewport | none |
| matrix-dialog | [U5-matrix-dialog-1440-dark.png](U5-matrix-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:32.296Z | viewport | none |
| matrix-compare-nested | [U5-matrix-compare-nested-1440-dark.png](U5-matrix-compare-nested-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:32.929Z | viewport | none |
| matrix-inspector | [U5-matrix-inspector-1440-dark.png](U5-matrix-inspector-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:34.380Z | viewport | none |
| matrix-inspector-picked | [U5-matrix-inspector-picked-1440-dark.png](U5-matrix-inspector-picked-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:35.501Z | viewport | none |
| matrix-gpu-compare | [U5-matrix-gpu-compare-1440-dark.png](U5-matrix-gpu-compare-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | 2026-10-28T16:52:36.216Z | viewport | none |
| matrix-dom | [U5-matrix-dom-390-light.png](U5-matrix-dom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:44.012Z | viewport | none |
| matrix-local-opposite | [U5-matrix-local-opposite-390-light.png](U5-matrix-local-opposite-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:44.662Z | viewport | none |
| matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-390-light.png](U5-matrix-local-opposite-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:45.229Z | viewport | none |
| matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-390-light.png](U5-matrix-local-opposite-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:45.795Z | viewport | none |
| matrix-menu | [U5-matrix-menu-390-light.png](U5-matrix-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:46.429Z | viewport | none |
| matrix-dialog | [U5-matrix-dialog-390-light.png](U5-matrix-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:46.996Z | viewport | none |
| matrix-compare-nested | [U5-matrix-compare-nested-390-light.png](U5-matrix-compare-nested-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:47.612Z | viewport | none |
| matrix-inspector | [U5-matrix-inspector-390-light.png](U5-matrix-inspector-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:49.044Z | viewport | none |
| matrix-inspector-picked | [U5-matrix-inspector-picked-390-light.png](U5-matrix-inspector-picked-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:52:50.153Z | viewport | none |
| matrix2-gpu-compare | [U5-matrix2-gpu-compare-390-light.png](U5-matrix2-gpu-compare-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:55:47.637Z | viewport | none |
| matrix2-gpu-compare-end | [U5-matrix2-gpu-compare-end-390-light.png](U5-matrix2-gpu-compare-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | 2026-10-28T16:55:48.172Z | viewport | none |
