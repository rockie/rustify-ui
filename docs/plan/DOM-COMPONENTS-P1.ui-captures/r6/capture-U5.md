# UI 验收 R6 · U5 采集清单

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。

- 共 18 张。实际操作见 [U5-actions.log](U5-actions.log)。

| 单元 | 状态 | 原始截图 | URL | CSS视口 | DPR | 主题 | 截图方式 | 拍摄UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:27.421Z | renderer-dom | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:11:28.007Z | mode-dark | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:28.739Z | local-appearance | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:29.390Z | menu-item-duplicate | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:29.987Z | preview-dialog-close | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:30.655Z | compare-dark | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:31.357Z | renderer-compare | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:41.665Z | inspector-toggle | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:42.263Z | inspector-toggle | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:42.869Z | color-value | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:53.273Z | inspector-toggle | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:54.357Z | inspector-toggle | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:12:03.530Z | mobile-preview | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:12:04.097Z | preview-dialog-close | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:15:02.131Z | mode-dark | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:11.275Z | menu-item-duplicate | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:11.858Z | preview-dialog-close | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:21.314Z | inspector-toggle | none |
