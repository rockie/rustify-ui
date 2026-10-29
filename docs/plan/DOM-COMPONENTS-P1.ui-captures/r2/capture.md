# DOM-COMPONENTS-P1 UI 验收 R2 · 采集清单

- 模式：人工循环（本机没有 `phoenix-ui`，未自动下载）。R2 为**重建基线**：目标指纹自 R1 起改变（用户裁定 UI-100、UI-85 后改了「单元」与「规范要点」），全量轮。
- 被测版本：HEAD `f5613b47cebcebd3a0acd47184534297bc39a22b`（main）。工作树 dirty 仅限 SDLC 台账、两张旧验证 PNG 与未跟踪的评审材料；产品路径 `crates examples xtask web makepad Cargo.toml Cargo.lock` 相对 HEAD 无 diff、无未跟踪文件。
- 构建：本轮从 HEAD 重新 `mbx xtask build-web --release`（`CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0`）。Theme Studio build `f83c6ff130d88c2d`（WASM 9,735,795 B），component-catalog build `07b789e3e401b12a`（WASM 8,172,292 B）。采集前用 `curl …/build-manifest.json` 确认两服务发出的正是这两个 build id。
- 访问：`mbx xtask serve --example theme-studio --release --port 4181`、`--example component-catalog --release --port 4176`，响应头 strict CSP。
- 目标包：[DOM-COMPONENTS-P1.ui-target.md](../../DOM-COMPONENTS-P1.ui-target.md)；目标指纹 `manual-sha256-normative-sections-v1:24f1e44d673e725ea822b8d3e90d2a9924fb1a7250128f4a4016516f5caba967`（算法同 R1，见 [target-fingerprint.py](target-fingerprint.py)）。R1 为 `0370d573f009…`。本轮只改了目标包「采集」节并记入变更记录，指纹不变。
- 条件：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，`reducedMotion: reduce`，`colorScheme: light`，`locale: en-US`；每图前等待 `document.fonts.ready`、networkidle 与连续 500ms 无 >100ms 帧。DPR 1，文件名带 `-dpr2` 的特写为 DPR 2。默认截可视区域；U6/U7 组件夹具用元素截图（区域见下）；B2 焦点特写为控件四周 24px 的裁切（clip）。每个流程新开 browser context，localStorage 为空（U4 例外，见干预列）。
- 工具：[capture-runner.cjs](capture-runner.cjs)（以 R1 脚本为底）；原始日志 [capture-log.json](capture-log.json)（首采 3 次运行 121 张、补采 3 次运行 14 张，共 135 张，0 失败）；每单元实际操作记录 `U<n>-actions.log`。
- 采集期页面健康：全部页面 pageerror 0、console error 0、失败请求/HTTP≥400 0。

## 与 R1 采集的差异

- R1 的补采做法本轮并入首采，文件名不再带 `-a2`：对话框与长列表滚到动作区（`U3-import-review-end-*`、`U3-export-rust-end`、`U4-upgrade-review-end`）、下拉列表展开（`U3-export-css-profile-open`、`U3-export-css-notation-open`）、冲突原文展开（`U4-conflict-raw`）、无 Web Locks 初始帧（`U4-no-locks-initial`）、拾取开启后等 800ms 再悬停或点击（`U5-inspector*`）、焦点帧与缩放帧前把指针移到 (0,0)。
- 本轮新增：
  - `U5-local-opposite-1440-light`：预览内「A local theme boundary」的 Boundary appearance 选 Opposite to root。
  - U6 B2 焦点特写（DPR 2）：classic/solid/soft/surface/outline/ghost-offset/ghost 的 size 2、red、High contrast、IconButton（×）与 Link，每个都先程序聚焦再按 Shift+Tab、Tab，确保是键盘焦点；夹具切 Dark 后再拍 solid/soft/outline（`U6-focus-<名>-1280-dark-dpr2`）。
  - `U6-links-focus` 改为键盘聚焦（R1 为程序 `focus()`）。
  - U7 分段元素图（1280 与 390）：`type-heading-matrix`、`type-text-matrix`、`type-code-matrix`、`type-kbd-matrix`、`type-blockquote-matrix`、`type-wrap-matrix`、`layout-container-matrix`、`layout-inset-demonstration`、`layout-reset-comparison`；缩放 110%/95% 另截 Heading 分段（`U7-scale110-heading`、`U7-scale95-heading`）；非法属性拍整段夹具（`U7-invalid`）与以 `layout-invalid-prop` 居中的视口帧（`U7-invalid-detail`）。
- 界面文案变了的选项按现名选取：品牌色「Pink」（R1 为「pink」）；导出类型「CSS · resolved values」（R1 为「Tailwind v4 CSS」）。
- U6 深色特写首次落盘时主题标签写成了 light（`U6-focus-solid-dark-1280-light-dpr2.png` 等 3 张），未重拍，改名为 `U6-focus-{solid,soft,outline}-1280-dark-dpr2.png`，日志记录保留 `renamedFrom`。

## 元素截图的区域

| 图像 | 截取的元素 |
| --- | --- |
| `U6-fixture-*`、`U6-radius-full-*` | 按钮夹具根 `button-fixture-body` |
| `U6-loading-*` | 原生表单行 `button-native-form` |
| `U6-scale110-*` | 变体 × 尺寸矩阵 `button-matrix` |
| `U6-panel-zh-*` | ThemePanel 夹具 `button-panel-fixture` |
| `U7-fixture-*`、`U7-scale110-1280`、`U7-scale95-1280`、`U7-invalid-1280` | 布局夹具根 `catalog-layout-fixture`（整段，很长） |
| `U7-<分段名>-*` | 同名分段元素 |

## 逐图登记

| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 截图方式 | 键盘焦点所在 | 拍摄时间（UTC） | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:49:08.050Z | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T09:49:09.826Z | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T09:49:10.607Z | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T09:49:11.179Z | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-27T09:49:12.121Z | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-27T09:49:12.691Z | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | — | 2026-10-27T09:49:25.877Z | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T09:49:27.777Z | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-27T09:49:29.863Z | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T09:49:30.697Z | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-27T09:49:42.926Z | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | — | 2026-10-27T09:49:56.785Z | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T09:49:57.407Z | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preset-search | 2026-10-27T09:50:10.417Z | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color | 2026-10-27T09:50:10.984Z | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color-list | 2026-10-27T09:50:11.548Z | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | mode-dark | 2026-10-27T09:50:12.293Z | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T10:23:54.670Z | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T10:24:04.906Z | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-colors | 2026-10-27T10:24:05.470Z | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T10:24:06.821Z | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-27T09:50:27.647Z | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-27T09:50:28.237Z | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:50:29.067Z | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:50:29.836Z | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-ring | 2026-10-27T09:50:30.459Z | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-controls | 2026-10-27T09:50:31.071Z | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | edit-radius | 2026-10-27T09:50:31.699Z | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-shell | 2026-10-27T09:50:47.166Z | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | color-token-primary | 2026-10-27T09:50:49.532Z | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T09:51:07.200Z | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T09:51:07.780Z | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T09:51:08.400Z | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T09:51:10.223Z | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T09:51:10.893Z | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T09:51:11.584Z | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-profile-list | 2026-10-27T09:51:12.212Z | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-color-format-list | 2026-10-27T09:51:12.828Z | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T09:51:13.506Z | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-27T09:51:14.426Z | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-27T09:51:15.000Z | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T09:51:17.244Z | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T09:51:39.694Z | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T09:51:40.593Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T10:24:17.418Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T10:24:19.498Z | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:51:56.075Z | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-27T09:51:56.730Z | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-27T09:51:57.313Z | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T09:51:58.094Z | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:52:00.263Z | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-27T09:52:03.458Z | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T09:52:17.068Z | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-operations | 2026-10-27T09:52:17.672Z | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T09:52:33.751Z | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T09:52:34.347Z | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-external-raw | 2026-10-27T09:52:34.961Z | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T09:52:47.722Z | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T09:52:48.773Z | navigator.locks removed before load |
| U4 | conflict-detail | [U4-conflict-detail-1440-light-a2.png](U4-conflict-detail-1440-light-a2.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T10:24:31.650Z | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light-a3.png](U4-conflict-detail-1440-light-a3.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T10:25:58.780Z | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T10:25:59.354Z | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light-a2.png](U4-conflict-after-1440-light-a2.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T10:26:40.153Z | second same-origin tab wrote v2 |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-dom | 2026-10-27T09:53:03.158Z | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T09:53:03.758Z | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | local-appearance | 2026-10-27T09:53:04.506Z | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-27T09:53:05.172Z | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-27T09:53:05.775Z | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | compare-dark | 2026-10-27T09:53:06.455Z | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-compare | 2026-10-27T09:53:07.266Z | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T09:53:21.136Z | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T09:53:21.768Z | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T09:53:22.412Z | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T09:53:34.955Z | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T09:53:36.053Z | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-27T09:53:47.315Z | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | preview-dialog-close | 2026-10-27T09:53:47.899Z | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T10:24:41.530Z | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-27T10:24:51.029Z | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-27T10:24:51.633Z | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T10:25:02.596Z | none |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:54:07.158Z | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | dark | element | button-mode-dark | 2026-10-27T09:54:07.866Z | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-toggle-loading | 2026-10-27T09:54:08.557Z | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-default | 2026-10-27T09:54:09.171Z | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-color-gold | 2026-10-27T09:54:09.723Z | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-native-unavailable | 2026-10-27T09:54:10.288Z | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-scale-110 | 2026-10-27T09:54:10.932Z | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-theme-full | 2026-10-27T09:54:11.674Z | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-panel-chinese | 2026-10-27T09:54:12.325Z | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:54:28.981Z | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-27T09:54:45.533Z | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-27T09:54:46.133Z | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | type-link | 2026-10-27T09:54:46.699Z | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | viewport | — | 2026-10-27T09:55:03.227Z | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-classic-2 | 2026-10-27T09:55:19.331Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-solid-2 | 2026-10-27T09:55:19.891Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-soft-2 | 2026-10-27T09:55:20.458Z | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-surface-2 | 2026-10-27T09:55:21.032Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-outline-2 | 2026-10-27T09:55:21.622Z | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-offset-2 | 2026-10-27T09:55:22.201Z | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-2 | 2026-10-27T09:55:22.777Z | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-color-red | 2026-10-27T09:55:23.344Z | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-high-contrast | 2026-10-27T09:55:23.912Z | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-icon | 2026-10-27T09:55:24.493Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-solid-2 | 2026-10-27T09:55:25.128Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-soft-2 | 2026-10-27T09:55:25.694Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-outline-2 | 2026-10-27T09:55:26.261Z | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | type-link | 2026-10-27T09:55:42.968Z | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:55:59.935Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:00.574Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:01.179Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:01.782Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:02.379Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:02.965Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:03.575Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:04.175Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:04.765Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T09:56:05.349Z | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-110 | 2026-10-27T09:56:06.301Z | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-110 | 2026-10-27T09:56:06.931Z | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-95 | 2026-10-27T09:56:07.900Z | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-95 | 2026-10-27T09:56:08.526Z | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-invalid | 2026-10-27T09:56:09.491Z | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | layout-invalid | 2026-10-27T09:56:10.040Z | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:26.741Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:27.356Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:27.962Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:28.562Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:29.153Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:29.730Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:30.328Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:30.919Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:31.497Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T09:56:32.093Z | none |

## 未采与补采

- 目标包矩阵的全部格子首采均已采到，无 missing。
- 首批评审后补采（同轮，原图全部保留；只补评审报告看不出、且在冻结矩阵内的格子）：
  - U1：`U1-zoom200-export-end-720-light`（200% 导出对话框内各滚动区滚到底，看动作区）；`U1-field-tabs-1440-light`、`U1-field-tabs-focus-1440-light`（字段标签页 Colors / Typography & other，及其键盘焦点）；`U1-configured-lower-1440-light`（Graphite + Pink + Auto 后基础配置下半段，到 Scaling）。
  - U3：`U3-import-review-bottom-1440-light`、`U3-import-review-bottom-390-light`（导入审阅时把对话框内所有被裁切的滚动区滚到底，含差异表容器；被滚动的元素及其 `overflow-y` 写在 U3-actions.log）。
  - U4：冲突面板下半部与其后内容。`U4-conflict-detail-1440-light-a2` 的滚动目标落到了颜色编辑区，没有拍到冲突面板，不作该状态证据；`U4-conflict-detail-1440-light-a3` 把冲突面板底边对齐视口底部；`U4-conflict-after-1440-light` 的再滚动没有生效（与 `-a3` 逐像素相同），不作证据；`U4-conflict-after-1440-light-a2` 在 `-a3` 基础上把编辑列再滚 300px，显示冲突面板之后的状态区。评审以 `-a3` 与 `conflict-after -a2` 为准。
  - U5：`U5-dom-caller-1440-dark`（深色预览滚到 Caller class override）；`U5-local-opposite-menu-1440-light`、`U5-local-opposite-dialog-1440-light`（局部边界选 Opposite to root 后，从边界内打开 Actions 菜单与对话框）；`U5-inspector-undeclared-panel-1440-light`（拾取时点击预览主按钮后，把拾取面板滚入视野）。
- 首批评审报告的其他看不出项不在冻结矩阵内，未补采：U1/U3/U4/U5 的键盘焦点帧（U1 已有的 4 帧除外）、悬停与按下态、中文态（U1 除外）、GPU 失败态、预览宽度 1024/768/390 档、导入语法错误/超限/不支持颜色、复制或下载失败、排队态、模式变化绕过确认、Locate 点击结果、U6 的 high-contrast/gray/disabled 列与 IconButton 矩阵、Link 下划线模式、U7 的其他响应式断点。

## 主 agent 开图核对

逐张打开（视口图与元素图用缩略拼版，焦点特写、U7 分段与补采帧原尺寸查看），确认均为本轮在 4181/4176 真实页面拍摄、尺寸与视口或元素一致（视口图像素 = CSS 视口 × DPR），不是报错页、空白页或旧图。各状态均已到达：

- U1：默认首屏、Graphite + Pink + Auto 后的基础配置区、预览 Dark、中文、768、390 编辑页与预览页、720×900 + 根字号 200% 及其导出对话框；键盘帧依次为预设搜索、品牌色选择器、展开的列表、模式 Dark 获得焦点。补采帧见上。
- U2：Colors 页选中 primary 及其来源区、单项解除后、品牌分组解除后、ring 来源、字体与其他页、radius 输入 `abc` 回车后、background 与 foreground 均为 #ffffff 并展开诊断、390 来源。
- U3：导入审阅（1440/390 及滚到 Apply）、未知版本错误、导出 JSON v2 / 兼容 v1 / CSS（profile 与颜色记法列表展开）/ Rust（含复制后与滚到末尾）、390 导出。补采帧见上。
- U4：升级待确认（1440/390）、升级 Review 及其底部、取消后、升级后、配额失败及操作列表、跨标签冲突及原文展开、无 Web Locks 初始与命名保存后。`U4-conflict-detail-1440-light` 与 `U4-conflict-1440-light` 逐像素相同，`U4-upgrade-review-end-1440-light` 与 `U4-upgrade-review-1440-light` 逐像素相同（对话框在 900 高内无需滚动）。补采帧见上。
- U5：DOM 预览浅/深、局部边界相反外观、预览菜单、预览对话框（1440/390）、对比区嵌套 Dark、拾取（悬停卡片角、悬停与点击 Create profile、悬停与点击预览主按钮）、DOM + GPU、390 预览页。补采帧见上。
- U6：按钮夹具浅/深/390、加载、Default action / gold / Unavailable request 键盘焦点、缩放 110%、主题 full 圆角、ThemePanel 中文、Link 默认/悬停/键盘焦点；DPR 2 特写中焦点均落在所列控件上（日志 `focused` 字段与 testid 一致）。`U6-links-hover-1280-light` 与 `U6-links-1280-light` 逐像素相同。
- U7：布局夹具 1280/390 整段与各分段、缩放 110%/95%、点 Invalid prop 之后的整段与局部视口帧。
