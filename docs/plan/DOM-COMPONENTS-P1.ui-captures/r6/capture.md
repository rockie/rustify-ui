# DOM-COMPONENTS-P1 UI 验收 R6 · 采集清单

**最终范围：236 张独立评审原图，15 张后置机器原图。首批与补采记录依次保留；以末尾补采节和各单元 -b/-c 完整清单为准。**

- 被测版本：HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`。
- 当前源码重新构建并经 HTTP 核对：Studio `dff085ce38bbff64`，catalog `32b52b66f04701ad`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。

## 覆盖与出处

- 旧 Markdown 循环，无 managed 索引。继续人工 R6；未 import 或 begin 新循环。
- U1: 25, U2: 14, U3: 26, U4: 17, U5: 18, U6: 28, U7: 26；共 154 张。
- 历史采集尝试失败：[{"unit": "FOCUS6", "error": "locator.focus: Timeout 30000ms exceeded.\nCall log:\n\u001b[2m  - waiting for getByTestId('import-kind')\u001b[22m\n\n    at focused (/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-runner.cjs:839:28)\n    at cells.FOCUS6 (/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-runner.cjs:854:11)\n    at async /Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-runner.cjs:874:15"}]。旧定位名 import-kind 不存在，已改用真实 import-format 入口在 FOCUS6B 补齐 4 张；原图未覆盖，当前无必需格子缺失，见 capture-resolutions.json。
- 每帧出处见下表与 capture-log.json；U1-source 的稳定后 a2 帧与条件性 end 帧用于核对，首图保留。服务与停止方式见 services.json，产品路径见 products.json。

| 单元 | 状态 | 原始截图 | URL | CSS视口 | DPR | 主题 | 截图方式 | 拍摄UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:08:52.588Z | — | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:08:53.755Z | studio-config-gray_color | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:08:54.337Z | mode-dark | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:08:54.886Z | mode-dark | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:08:55.603Z | locale-zh | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:08:56.154Z | locale-zh | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:09:04.765Z | — | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:09:05.999Z | studio-config-gray_color | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:09:07.920Z | — | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:09:08.736Z | studio-config-gray_color | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:09:15.019Z | mobile-preview | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:09:22.969Z | — | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:09:23.554Z | data-tools-dialog-close | none |
| U1 | zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:09:31.771Z | editor-colors | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:09:32.387Z | color-token-primary | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:40.965Z | preset-search | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:41.516Z | studio-config-accent_color | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:42.066Z | studio-config-accent_color-list | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:42.729Z | mode-dark | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:51.338Z | color-token-primary | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:51.894Z | color-token-primary | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:52.478Z | — | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:53.059Z | — | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:53.655Z | color-token-ring | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:54.257Z | editor-controls | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:09:54.838Z | edit-radius | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:04.580Z | editor-shell | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:10:06.918Z | color-token-primary | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:16.625Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:17.195Z | import-review | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:17.788Z | import-review | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:19.037Z | data-tools-dialog-close | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:19.685Z | export-kind | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:20.352Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:20.950Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:21.548Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:22.199Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:23.100Z | export-copy | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:23.661Z | export-copy | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:10:25.743Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:10:26.291Z | import-review | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:10:27.176Z | data-tools-dialog-close | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:38.702Z | — | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:39.337Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:39.903Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:40.649Z | theme-save | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:42.868Z | — | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:10:46.006Z | — | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:55.652Z | theme-save | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:10:56.238Z | storage-operations | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:06.810Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:07.377Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:07.994Z | storage-external-raw | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:17.158Z | — | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:11:18.042Z | theme-save | navigator.locks removed before load |
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
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:12:17.912Z | — | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:12:18.609Z | button-mode-dark | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:12:19.264Z | button-toggle-loading | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:19.885Z | button-default | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:20.433Z | button-color-gold | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:20.985Z | button-native-unavailable | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:12:21.624Z | button-scale-110 | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:12:22.365Z | button-theme-full | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:12:23.020Z | button-panel-chinese | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:12:36.784Z | — | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:50.295Z | — | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:50.895Z | — | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:12:51.464Z | type-link | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:13:04.766Z | — | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:18.218Z | button-classic-2 | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:18.784Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:19.351Z | button-soft-2 | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:19.917Z | button-surface-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:20.484Z | button-outline-2 | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:21.052Z | button-ghost-offset-2 | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:21.617Z | button-ghost-2 | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:22.187Z | button-color-red | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:22.769Z | button-high-contrast | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:23.333Z | button-icon | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T16:13:23.952Z | button-solid-2 | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T16:13:24.518Z | button-soft-2 | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | dark | clip | 2026-10-28T16:13:25.084Z | button-outline-2 | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 | 2 | light | clip | 2026-10-28T16:13:36.451Z | type-link | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10322} root {"left":24,"top":1517.953125,"right":1256,"bottom":11822.578125} | 2026-10-28T16:13:48.075Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:48.675Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:49.265Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:49.850Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:50.444Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:51.021Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:51.618Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:52.213Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:52.793Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:53.398Z | — | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10931} root {"left":24,"top":1517.953125,"right":1256,"bottom":12431.90625} | 2026-10-28T16:13:54.241Z | layout-scale-110 | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:54.841Z | layout-scale-110 | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10024} root {"left":24,"top":1517.953125,"right":1256,"bottom":11524.25} | 2026-10-28T16:13:55.660Z | layout-scale-95 | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | element | 2026-10-28T16:13:56.257Z | layout-scale-95 | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10359} root {"left":24,"top":1517.953125,"right":1256,"bottom":11859.578125} | 2026-10-28T16:13:57.117Z | layout-invalid | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | light | viewport | 2026-10-28T16:13:57.665Z | layout-invalid | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15447} root {"left":12,"top":2452.453125,"right":378,"bottom":17882.5625} | 2026-10-28T16:14:08.595Z | — | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:09.191Z | — | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:09.780Z | — | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:10.385Z | — | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:10.973Z | — | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:11.553Z | — | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:12.144Z | — | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:12.723Z | — | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:13.306Z | — | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:14:13.902Z | — | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:14:21.591Z | data-tools-dialog-close | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:30.207Z | — | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:30.771Z | editor-colors | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:31.873Z | studio-config-gray_color | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:40.367Z | import-review | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:14:42.342Z | import-review | none |
| U4 | conflict-detail-end | [U4-conflict-detail-end-1440-light.png](U4-conflict-detail-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:52.444Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:52.995Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-diff | [U4-conflict-diff-1440-light.png](U4-conflict-diff-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:14:53.596Z | storage-conflict-diff | second same-origin tab wrote v2 |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:15:02.131Z | mode-dark | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:11.275Z | menu-item-duplicate | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:11.858Z | preview-dialog-close | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:21.314Z | inspector-toggle | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:30.030Z | studio-config-gray_color | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:15:32.026Z | import-review | none |
| U4 | upgrade-review-json | [U4-upgrade-review-json-1440-light.png](U4-upgrade-review-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:41.841Z | upgrade-review | localStorage v1 seeded |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:15:51.484Z | export-kind | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:16:02.034Z | color-token-primary | none |
| U1 | zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 | 1 | light | viewport | 2026-10-28T16:16:03.101Z | color-token-primary | none |
| U2 | search-row | [U2-search-row-1440-light.png](U2-search-row-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:11.333Z | editor-colors | none |
| U2 | group-reset-tokens | [U2-group-reset-tokens-1440-light.png](U2-group-reset-tokens-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:19.826Z | group-override-tokens | none |
| U2 | group-reset-tokens-end | [U2-group-reset-tokens-end-1440-light.png](U2-group-reset-tokens-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:20.376Z | group-override-tokens | none |
| U3 | export-kind-keyboard-focus | [U3-export-kind-keyboard-focus-1440-light.png](U3-export-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:29.334Z | export-kind | none |
| U3 | export-kind-keyboard-open | [U3-export-kind-keyboard-open-1440-light.png](U3-export-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:29.899Z | export-kind-list | none |
| U3 | export-profile-keyboard-focus | [U3-export-profile-keyboard-focus-1440-light.png](U3-export-profile-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:31.249Z | export-profile | none |
| U3 | export-color-format-keyboard-focus | [U3-export-color-format-keyboard-focus-1440-light.png](U3-export-color-format-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:32.516Z | export-color-format | none |
| U3 | export-next-keyboard-focus | [U3-export-next-keyboard-focus-1440-light.png](U3-export-next-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:16:33.782Z | export-source | none |
| U3 | import-kind-keyboard-focus | [U3-import-kind-keyboard-focus-1440-light.png](U3-import-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:17:50.494Z | import-format | none |
| U3 | import-kind-keyboard-open | [U3-import-kind-keyboard-open-1440-light.png](U3-import-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:17:51.049Z | import-format-list | none |
| U3 | import-format-keyboard-selected | [U3-import-format-keyboard-selected-1440-light.png](U3-import-format-keyboard-selected-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:17:51.598Z | import-format | none |
| U2 | token-reset-keyboard-focus | [U2-token-reset-keyboard-focus-1440-light.png](U2-token-reset-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-28T16:18:01.260Z | color-reset-primary | none |

## 独立交卷后的机器实拍

以下 12 张来自后置机器检查，没有提供给独立评审，不改变独立评分。逐张 URL、视口、时间、工具和 hash 见 [machine-captures.json](machine-captures.json)。

| 图像 | 视口 / DPR | 拍摄 UTC |
| --- | --- | --- |
| [MC-confirm-button-1440-light.png](MC-confirm-button-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:36:49.730Z |
| [MC-copy-denied-1440-light.png](MC-copy-denied-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:35:22.260Z |
| [MC-export-color-format-keyboard-focus-1440-light.png](MC-export-color-format-keyboard-focus-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:13.297639+00:00 |
| [MC-export-format-pure-keyboard-1440-light.png](MC-export-format-pure-keyboard-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:10.908896+00:00 |
| [MC-export-next-control-keyboard-1440-light.png](MC-export-next-control-keyboard-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:11.668144+00:00 |
| [MC-export-profile-keyboard-focus-1440-light.png](MC-export-profile-keyboard-focus-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:12.531562+00:00 |
| [MC-import-format-pure-keyboard-1440-light.png](MC-import-format-pure-keyboard-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:21.940116+00:00 |
| [MC-import-next-control-keyboard-1440-light.png](MC-import-next-control-keyboard-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:34:22.689227+00:00 |
| [MC-local-opposite-dialog-1440-light.png](MC-local-opposite-dialog-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:35:31.704Z |
| [MC-root200-preview-720-light.png](MC-root200-preview-720-light.png) | 720×900 / 1 | 2026-10-28T16:32:27.877145+00:00 |
| [MC-root200-preview-detail-720-light.png](MC-root200-preview-detail-720-light.png) | 720×900 / 1 | 2026-10-28T16:35:42.520Z |
| [MC-token-reset-keyboard-focus-1440-light.png](MC-token-reset-keyboard-focus-1440-light.png) | 1440×900 / 1 | 2026-10-28T16:35:33.020Z |

## 交付前矩阵补采与范围更正

首批 154 张覆盖了全部步骤，但遗漏了部分状态 × 视口 × 主题组合。先前“无必需格子缺失”的范围判断在交付前回读矩阵时更正；初版清单、评分和报告保留在 revisions/，不覆盖任何原图。追加 82 张后共 **236 张**：U1 46, U2 24, U3 26, U4 17, U5 39, U6 50, U7 34。本节及各单元 -b 清单为完整补采索引，旧 -a 清单保持原样。

- U1：补 768/390 中文与四个键盘焦点帧，1440 深色默认/中文/键盘帧。
- U2：补 390 的来源、单项/分组解除、展开影响清单首尾、字段错误与低对比。
- U5：补 1440 深色和 390 浅色的嵌套相反边界、菜单、对话框、拾取、DOM+GPU；窄屏拾取后按真实导航返回 Preview 再继续。
- U6：补 1280 深色、390 浅色的 loading/键盘/110%/full/中文状态和 Link 悬停/焦点。catalog 的布局夹具没有深色入口；深色 Link 使用同 SDK 的 Studio DOM 预览中 Edit theme 真实链接，在 1280×900 实拍 normal/hover/keyboard-focus。只改变采集入口，不改组件样式或目标，不把这一链接的结果外推到所有参数组合。1440 的同链接帧作为额外证据保留。
- U7：补 390 的 110%/95% 整体、Heading/Kbd 分段及 invalid 整体/诊断。
- 独立补评后再追加 U2 390 低对比预览诊断，以及 U1 390 中文预览页，各 1 张；U1/U2 再由新的独立评审读取本单元完整材料复核。中文与 200% 是两个独立状态，不假定所有状态彼此做组合；额外中文 200% 未采，不外推通过。
- 两次补采脚本超时保留：中文界面中仍用英文预设名定位、窄屏拾取后未回 Preview 而点隐藏按钮。分别通过先选预设再切中文、真实返回 Preview 补齐；它们是采集脚本问题，不登记产品故障。最终 followup 13 帧无失败，见 matrix-followup-log.json。
- 补采后的 U1/U2/U5/U6/U7 各交给新的独立评审，只给目标、本轮完整原图和上一轮 R5 评审材料；不提供本轮初评分、报告或机器检查。

| 单元 | 状态 | 原始截图 | URL | CSS视口 | DPR | 主题 | 截图方式 | 拍摄UTC | 焦点 | 数据/环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | matrix-zh-top | [U1-matrix-zh-top-768-light.png](U1-matrix-zh-top-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:54.329Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-768-light.png](U1-matrix-zh-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:54.891Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-768-light.png](U1-matrix-keyboard-01-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:55.508Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-768-light.png](U1-matrix-keyboard-02-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:56.058Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-768-light.png](U1-matrix-keyboard-03-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:56.607Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-768-light.png](U1-matrix-keyboard-04-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | 1 | light | viewport | 2026-10-28T16:50:57.155Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-390-light.png](U1-matrix-zh-top-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:50:59.331Z | locale-zh | none |
| U1 | matrix-zh-configured | [U1-matrix-zh-configured-390-light.png](U1-matrix-zh-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:50:59.881Z | locale-zh | none |
| U1 | matrix-keyboard-01 | [U1-matrix-keyboard-01-390-light.png](U1-matrix-keyboard-01-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:51:00.465Z | preset-search | none |
| U1 | matrix-keyboard-02 | [U1-matrix-keyboard-02-390-light.png](U1-matrix-keyboard-02-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:51:00.999Z | studio-config-accent_color | none |
| U1 | matrix-keyboard-03 | [U1-matrix-keyboard-03-390-light.png](U1-matrix-keyboard-03-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:51:01.532Z | studio-config-accent_color-list | none |
| U1 | matrix-keyboard-04 | [U1-matrix-keyboard-04-390-light.png](U1-matrix-keyboard-04-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:51:02.066Z | mode-dark | none |
| U1 | matrix-default | [U1-matrix-default-1440-dark.png](U1-matrix-default-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:51:10.541Z | mode-dark | none |
| U1 | matrix-zh-top | [U1-matrix-zh-top-1440-dark.png](U1-matrix-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:51:11.163Z | locale-zh | none |
| U2 | matrix-source-overridden | [U2-matrix-source-overridden-390-light.png](U2-matrix-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:13.994Z | color-token-primary | none |
| U2 | matrix-token-released | [U2-matrix-token-released-390-light.png](U2-matrix-token-released-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:14.562Z | — | none |
| U2 | matrix-group-reset-tokens | [U2-matrix-group-reset-tokens-390-light.png](U2-matrix-group-reset-tokens-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:15.157Z | group-override-tokens | none |
| U2 | matrix-group-reset-tokens-end | [U2-matrix-group-reset-tokens-end-390-light.png](U2-matrix-group-reset-tokens-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:15.706Z | group-override-tokens | none |
| U2 | matrix-group-reset | [U2-matrix-group-reset-390-light.png](U2-matrix-group-reset-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:16.273Z | — | none |
| U2 | matrix-source-generated | [U2-matrix-source-generated-390-light.png](U2-matrix-source-generated-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:17.361Z | color-token-ring | none |
| U2 | matrix-controls | [U2-matrix-controls-390-light.png](U2-matrix-controls-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:17.939Z | editor-controls | none |
| U2 | matrix-field-error | [U2-matrix-field-error-390-light.png](U2-matrix-field-error-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:18.505Z | edit-radius | none |
| U2 | matrix-low-contrast | [U2-matrix-low-contrast-390-light.png](U2-matrix-low-contrast-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:20.557Z | editor-shell | none |
| U5 | matrix-dom | [U5-matrix-dom-1440-dark.png](U5-matrix-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:29.247Z | mode-dark | none |
| U5 | matrix-local-opposite | [U5-matrix-local-opposite-1440-dark.png](U5-matrix-local-opposite-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:29.897Z | local-appearance | none |
| U5 | matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-1440-dark.png](U5-matrix-local-opposite-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:30.481Z | menu-item-duplicate | none |
| U5 | matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-1440-dark.png](U5-matrix-local-opposite-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:31.062Z | preview-dialog-close | none |
| U5 | matrix-menu | [U5-matrix-menu-1440-dark.png](U5-matrix-menu-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:31.710Z | menu-item-duplicate | none |
| U5 | matrix-dialog | [U5-matrix-dialog-1440-dark.png](U5-matrix-dialog-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:32.296Z | preview-dialog-close | none |
| U5 | matrix-compare-nested | [U5-matrix-compare-nested-1440-dark.png](U5-matrix-compare-nested-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:32.929Z | compare-light | none |
| U5 | matrix-inspector | [U5-matrix-inspector-1440-dark.png](U5-matrix-inspector-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:34.380Z | inspector-toggle | none |
| U5 | matrix-inspector-picked | [U5-matrix-inspector-picked-1440-dark.png](U5-matrix-inspector-picked-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:35.501Z | color-value | none |
| U5 | matrix-gpu-compare | [U5-matrix-gpu-compare-1440-dark.png](U5-matrix-gpu-compare-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:52:36.216Z | renderer-compare | none |
| U5 | matrix-dom | [U5-matrix-dom-390-light.png](U5-matrix-dom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:44.012Z | renderer-dom | none |
| U5 | matrix-local-opposite | [U5-matrix-local-opposite-390-light.png](U5-matrix-local-opposite-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:44.662Z | local-appearance | none |
| U5 | matrix-local-opposite-menu | [U5-matrix-local-opposite-menu-390-light.png](U5-matrix-local-opposite-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:45.229Z | menu-item-duplicate | none |
| U5 | matrix-local-opposite-dialog | [U5-matrix-local-opposite-dialog-390-light.png](U5-matrix-local-opposite-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:45.795Z | preview-dialog-close | none |
| U5 | matrix-menu | [U5-matrix-menu-390-light.png](U5-matrix-menu-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:46.429Z | menu-item-duplicate | none |
| U5 | matrix-dialog | [U5-matrix-dialog-390-light.png](U5-matrix-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:46.996Z | preview-dialog-close | none |
| U5 | matrix-compare-nested | [U5-matrix-compare-nested-390-light.png](U5-matrix-compare-nested-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:47.612Z | compare-dark | none |
| U5 | matrix-inspector | [U5-matrix-inspector-390-light.png](U5-matrix-inspector-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:49.044Z | inspector-toggle | none |
| U5 | matrix-inspector-picked | [U5-matrix-inspector-picked-390-light.png](U5-matrix-inspector-picked-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:52:50.153Z | color-value | none |
| U6 | matrix-fixture | [U6-matrix-fixture-390-light.png](U6-matrix-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:53:31.699Z | — | none |
| U6 | matrix-loading | [U6-matrix-loading-390-light.png](U6-matrix-loading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:53:32.324Z | button-toggle-loading | none |
| U6 | matrix-focus | [U6-matrix-focus-390-light.png](U6-matrix-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:53:32.913Z | button-default | none |
| U6 | matrix-focus-unavailable | [U6-matrix-focus-unavailable-390-light.png](U6-matrix-focus-unavailable-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:53:33.444Z | button-native-unavailable | none |
| U6 | matrix-scale110 | [U6-matrix-scale110-390-light.png](U6-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:53:34.083Z | button-scale-110 | none |
| U6 | matrix-radius-full | [U6-matrix-radius-full-390-light.png](U6-matrix-radius-full-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:53:34.765Z | button-theme-full | none |
| U6 | matrix-panel-zh | [U6-matrix-panel-zh-390-light.png](U6-matrix-panel-zh-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:53:35.398Z | button-panel-chinese | none |
| U6 | matrix-fixture | [U6-matrix-fixture-1280-dark.png](U6-matrix-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:53:45.928Z | button-mode-dark | none |
| U6 | matrix-loading | [U6-matrix-loading-1280-dark.png](U6-matrix-loading-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:53:46.560Z | button-toggle-loading | none |
| U6 | matrix-focus | [U6-matrix-focus-1280-dark.png](U6-matrix-focus-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | viewport | 2026-10-28T16:53:47.156Z | button-default | none |
| U6 | matrix-focus-unavailable | [U6-matrix-focus-unavailable-1280-dark.png](U6-matrix-focus-unavailable-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | viewport | 2026-10-28T16:53:47.707Z | button-native-unavailable | none |
| U6 | matrix-scale110 | [U6-matrix-scale110-1280-dark.png](U6-matrix-scale110-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:53:48.350Z | button-scale-110 | none |
| U6 | matrix-radius-full | [U6-matrix-radius-full-1280-dark.png](U6-matrix-radius-full-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:53:49.048Z | button-theme-full | none |
| U6 | matrix-panel-zh | [U6-matrix-panel-zh-1280-dark.png](U6-matrix-panel-zh-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 | 1 | dark | element | 2026-10-28T16:53:49.694Z | button-panel-chinese | none |
| U6 | matrix-link-studio | [U6-matrix-link-studio-1440-dark.png](U6-matrix-link-studio-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:53:58.030Z | mode-dark | none |
| U6 | matrix-link-studio-hover | [U6-matrix-link-studio-hover-1440-dark.png](U6-matrix-link-studio-hover-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:53:58.631Z | mode-dark | none |
| U6 | matrix-link-studio-focus | [U6-matrix-link-studio-focus-1440-dark.png](U6-matrix-link-studio-focus-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:53:59.179Z | cards-editor-link | none |
| U7 | matrix-scale110 | [U7-matrix-scale110-390-light.png](U7-matrix-scale110-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":16691} root {"left":12,"top":2452.453125,"right":378,"bottom":19126.53125} | 2026-10-28T16:54:09.909Z | layout-scale-110 | none |
| U7 | matrix-scale110-heading | [U7-matrix-scale110-heading-390-light.png](U7-matrix-scale110-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:54:10.518Z | layout-scale-110 | none |
| U7 | matrix-scale110-kbd | [U7-matrix-scale110-kbd-390-light.png](U7-matrix-scale110-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:54:11.100Z | layout-scale-110 | none |
| U7 | matrix-scale95 | [U7-matrix-scale95-390-light.png](U7-matrix-scale95-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":14882} root {"left":12,"top":2452.453125,"right":378,"bottom":17317.90625} | 2026-10-28T16:54:11.850Z | layout-scale-95 | none |
| U7 | matrix-scale95-heading | [U7-matrix-scale95-heading-390-light.png](U7-matrix-scale95-heading-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:54:12.439Z | layout-scale-95 | none |
| U7 | matrix-scale95-kbd | [U7-matrix-scale95-kbd-390-light.png](U7-matrix-scale95-kbd-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | element | 2026-10-28T16:54:13.030Z | layout-scale-95 | none |
| U7 | matrix-invalid | [U7-matrix-invalid-390-light.png](U7-matrix-invalid-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15505} root {"left":12,"top":2452.453125,"right":378,"bottom":17940.5625} | 2026-10-28T16:54:13.820Z | layout-invalid | none |
| U7 | matrix-invalid-detail | [U7-matrix-invalid-detail-390-light.png](U7-matrix-invalid-detail-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:54:14.364Z | layout-invalid | none |
| U1 | matrix2-zh-top | [U1-matrix2-zh-top-1440-dark.png](U1-matrix2-zh-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:15.582Z | locale-zh | none |
| U1 | matrix2-zh-configured | [U1-matrix2-zh-configured-1440-dark.png](U1-matrix2-zh-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:16.144Z | locale-zh | none |
| U1 | matrix2-keyboard-01 | [U1-matrix2-keyboard-01-1440-dark.png](U1-matrix2-keyboard-01-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:16.756Z | preset-search | none |
| U1 | matrix2-keyboard-02 | [U1-matrix2-keyboard-02-1440-dark.png](U1-matrix2-keyboard-02-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:17.306Z | studio-config-accent_color | none |
| U1 | matrix2-keyboard-03 | [U1-matrix2-keyboard-03-1440-dark.png](U1-matrix2-keyboard-03-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:17.857Z | studio-config-accent_color-list | none |
| U1 | matrix2-keyboard-04 | [U1-matrix2-keyboard-04-1440-dark.png](U1-matrix2-keyboard-04-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | dark | viewport | 2026-10-28T16:55:18.408Z | mode-dark | none |
| U6 | matrix2-links-hover | [U6-matrix2-links-hover-390-light.png](U6-matrix2-links-hover-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:55:29.202Z | — | none |
| U6 | matrix2-links-focus | [U6-matrix2-links-focus-390-light.png](U6-matrix2-links-focus-390-light.png) | http://127.0.0.1:4176/ | 390x844 | 1 | light | viewport | 2026-10-28T16:55:29.751Z | type-link | none |
| U6 | matrix2-link-studio | [U6-matrix2-link-studio-1280-dark.png](U6-matrix2-link-studio-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-28T16:55:37.818Z | mode-dark | none |
| U6 | matrix2-link-studio-hover | [U6-matrix2-link-studio-hover-1280-dark.png](U6-matrix2-link-studio-hover-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-28T16:55:38.419Z | mode-dark | none |
| U6 | matrix2-link-studio-focus | [U6-matrix2-link-studio-focus-1280-dark.png](U6-matrix2-link-studio-focus-1280-dark.png) | http://127.0.0.1:4181/ | 1280x900 | 1 | dark | viewport | 2026-10-28T16:55:38.987Z | cards-editor-link | none |
| U5 | matrix2-gpu-compare | [U5-matrix2-gpu-compare-390-light.png](U5-matrix2-gpu-compare-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:55:47.637Z | renderer-compare | none |
| U5 | matrix2-gpu-compare-end | [U5-matrix2-gpu-compare-end-390-light.png](U5-matrix2-gpu-compare-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T16:55:48.172Z | renderer-compare | none |
| U2 | matrix3-low-contrast-preview | [U2-matrix3-low-contrast-preview-390-light.png](U2-matrix3-low-contrast-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T17:02:21.020Z | mobile-preview | none |
| U1 | matrix3-zh-preview | [U1-matrix3-zh-preview-390-light.png](U1-matrix3-zh-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-28T17:04:05.459Z | mobile-preview | none |

## 品牌色键盘活动项的后置实测

初次测量 768 成功，390/1440 因未等待界面稳定而未找到展开列表；保留原失败。增加稳定等待后，390/1440 成功。三个视口均实测到 Plum 已为活动项、列表 scrollTop=0、活动项在滚动视窗之外；Enter 实际确认 Plum。结果只用于主 agent 核对 UI-217，不反馈给独立评审改分。

| 截图 | URL | 视口 | 主题 | UTC | 证据 |
| --- | --- | --- | --- | --- | --- |
| [MC-pink-menu-768-light.png](MC-pink-menu-768-light.png) | http://127.0.0.1:4181/ | 768x1024 | light | 2026-10-28T17:06:16.483Z | [machine-pink-menu.json](machine-pink-menu.json) |
| [MC-pink-menu-confirm-390-light.png](MC-pink-menu-confirm-390-light.png) | http://127.0.0.1:4181/ | 390x844 | light | 2026-10-28T17:08:30.672Z | [machine-pink-menu-confirm.json](machine-pink-menu-confirm.json) |
| [MC-pink-menu-confirm-1440-dark.png](MC-pink-menu-confirm-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 | dark | 2026-10-28T17:08:42.753Z | [machine-pink-menu-confirm.json](machine-pink-menu-confirm.json) |
