# DOM-COMPONENTS-P1 UI 验收 R3 · 采集清单

- 模式：人工循环（本机没有 `phoenix-ui`，未自动下载）。R3 为**重建基线**、全量轮：目标指纹自 R2 起改变（用户在 R2 后裁定 UI-156，U5、U7 的比较口径追加了对比度条款）。
- 被测版本：HEAD `d5775d8a661ae8832188362a3fdb00488ec34ac9`（main）。工作树 dirty 只涉及 SDLC 台账、两张旧验证 PNG 与未跟踪的评审材料；产品路径 `crates examples xtask web makepad public Cargo.toml Cargo.lock sources.lock.json` 相对 HEAD 无 diff，也没有未跟踪文件。
- 构建：本轮从 HEAD 重新 `mbx xtask build-web --release`（`CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0`）。Theme Studio build `80cad3c530b54d9f`（WASM 9,802,741 B），component-catalog build `7ea7f541a819b220`（WASM 8,197,818 B）。采集前用 `curl …/build-manifest.json` 确认两个服务发出的正是这两个 build id。
- 访问：`mbx xtask serve --example theme-studio --release --port 4181`、`--example component-catalog --release --port 4176`，响应头为 strict CSP。
- 目标包：[DOM-COMPONENTS-P1.ui-target.md](../../DOM-COMPONENTS-P1.ui-target.md)；目标指纹 `manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`（算法同 R2，见 [target-fingerprint.py](target-fingerprint.py)）。R2 为 `24f1e44d673e…`。
- 条件：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，`reducedMotion: reduce`，`colorScheme: light`，`locale: en-US`。每张图拍摄前等待 `document.fonts.ready`、networkidle，以及连续 500ms 没有超过 100ms 的帧。DPR 1；文件名带 `-dpr2` 的特写为 DPR 2。默认截可视区域。U6、U7 的分段用元素截图（区域见下表）；U7 整段夹具帧用页面裁切（见下文）；B2 焦点特写是控件四周 24px 的裁切。每个流程新开 browser context，localStorage 为空（U4 例外，见「干预」列）。
- 工具：[capture-runner.cjs](capture-runner.cjs)（以 R2 脚本为底）。原始日志 [capture-log.json](capture-log.json)：1 次运行，134 张，0 失败，拍摄时间 2026-10-27T15:17:46Z–15:25:07Z。每单元的实际操作记录在 `U<n>-actions.log`。
- 采集期页面健康：37 个页面的 pageerror、console error、失败请求、HTTP ≥ 400 均为 0。

## 与 R2 采集的差异

只改了采集做法，目标包的「单元」「规范要点」没有因此改变。

- **R2 的补采帧并入首采**，文件名不再带 attempt 后缀：
  - U1：`U1-zoom200-export-end`、`U1-field-tabs`、`U1-field-tabs-focus`、`U1-configured-lower`；
  - U3：`U3-import-review-bottom-1440`、`-390`；
  - U5：`U5-dom-caller-1440-dark`、`U5-local-opposite-menu`、`U5-local-opposite-dialog`、`U5-inspector-undeclared-panel`。
- **U4 冲突面板下半部**，与 R2 文件的对应关系：

  | R3 | R2 对应帧 | 做法 |
  | --- | --- | --- |
  | `U4-conflict-detail-end-1440-light` | `U4-conflict-detail-1440-light-a3` | 冲突面板底边对齐视口底部 |
  | `U4-conflict-after-1440-light` | `U4-conflict-after-1440-light-a2` | 在上一帧基础上把编辑列（`aside#studio-editor`）再滚 300px，看面板之后的状态区 |
  | `U4-conflict-diff-1440-light` | 无（本轮新增） | 冲突面板里有可折叠的「Draft differences」，点开后把它滚到顶端再拍 |

  R2 的 `U4-conflict-detail-1440-light-a2` 与无后缀的 `U4-conflict-after-1440-light` 当时就没拍到目标，不作对照。
- **U7 整段夹具帧改为页面裁切**（R2 之后用户裁定 UI-97，原话「改采集方式」）。涉及 `U7-fixture-1280/390`、`U7-scale110-1280`、`U7-scale95-1280`、`U7-invalid-1280`。做法：
  - 夹具根 `catalog-layout-fixture` 的 boundingBox 换算成文档坐标；
  - 四边向外取整到整数 CSS px，再各留 8px，截整页的这一块；
  - 留边内的像素在夹具根之外（页面背景，以及夹具上方宿主元素的下沿）。

  每张图的实际裁切框与夹具根坐标写在下表「截图方式」列。所以这几张比 R2 的元素截图宽 16px、高 16px 以上；夹具本身也变长了（1280 下，R2 元素截图高 8,281px，本轮夹具根高约 10,402px）。

## 元素截图的区域

| 图像 | 截取的元素 |
| --- | --- |
| `U6-fixture-*`、`U6-radius-full-*` | 按钮夹具根 `button-fixture-body` |
| `U6-loading-*` | 原生表单行 `button-native-form` |
| `U6-scale110-*` | 变体 × 尺寸矩阵 `button-matrix` |
| `U6-panel-zh-*` | ThemePanel 夹具 `button-panel-fixture` |
| `U7-fixture-*`、`U7-scale110-1280`、`U7-scale95-1280`、`U7-invalid-1280` | 布局夹具根 `catalog-layout-fixture` 的页面裁切（四周留 8px） |
| `U7-<分段名>-*` | 同名分段元素 |

## 逐图登记

| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 截图方式 | 键盘焦点所在 | 拍摄时间（UTC） | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:17:46.020Z | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T15:17:47.251Z | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T15:17:47.852Z | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T15:17:48.414Z | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-27T15:17:49.132Z | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-27T15:17:49.698Z | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | — | 2026-10-27T15:17:59.376Z | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T15:18:00.595Z | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-27T15:18:02.483Z | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T15:18:03.304Z | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-27T15:18:13.351Z | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | — | 2026-10-27T15:18:23.494Z | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T15:18:24.099Z | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preset-search | 2026-10-27T15:18:34.203Z | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color | 2026-10-27T15:18:34.787Z | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color-list | 2026-10-27T15:18:35.351Z | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | mode-dark | 2026-10-27T15:18:36.113Z | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T15:23:58.045Z | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:24:08.308Z | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-colors | 2026-10-27T15:24:08.883Z | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T15:24:10.616Z | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-27T15:18:46.325Z | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-27T15:18:46.896Z | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:18:47.596Z | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:18:48.190Z | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-ring | 2026-10-27T15:18:48.826Z | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-controls | 2026-10-27T15:18:49.489Z | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | edit-radius | 2026-10-27T15:18:50.131Z | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-shell | 2026-10-27T15:19:02.361Z | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | color-token-primary | 2026-10-27T15:19:04.609Z | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-27T16:05:16.989Z | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T15:19:14.788Z | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T15:19:15.357Z | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T15:19:15.960Z | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T15:19:17.537Z | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T15:19:18.194Z | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T15:19:18.862Z | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-profile-list | 2026-10-27T15:19:19.462Z | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-color-format-list | 2026-10-27T15:19:20.061Z | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T15:19:20.710Z | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-27T15:19:21.612Z | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-27T15:19:22.175Z | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T15:19:24.223Z | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T15:19:24.771Z | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | data-tools-dialog-close | 2026-10-27T15:19:25.624Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-27T15:24:21.312Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T15:24:23.380Z | none |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T16:05:28.559Z | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-27T16:05:30.714Z | none |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light-a2.png](U3-export-rust-mid-1440-light-a2.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-27T16:06:46.687Z | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:19:37.358Z | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-27T15:19:37.977Z | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-27T15:19:38.545Z | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T15:19:39.309Z | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:19:41.526Z | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-27T15:19:44.520Z | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T15:19:54.592Z | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-operations | 2026-10-27T15:19:55.178Z | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T15:20:07.756Z | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T15:20:08.323Z | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-external-raw | 2026-10-27T15:20:08.941Z | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-27T15:20:20.217Z | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-27T15:20:21.206Z | navigator.locks removed before load |
| U4 | conflict-detail-end | [U4-conflict-detail-end-1440-light.png](U4-conflict-detail-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T15:24:35.743Z | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T15:24:36.292Z | second same-origin tab wrote v2 |
| U4 | conflict-diff | [U4-conflict-diff-1440-light.png](U4-conflict-diff-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-conflict-diff | 2026-10-27T15:24:36.893Z | second same-origin tab wrote v2 |
| U4 | upgrade-review-json | [U4-upgrade-review-json-1440-light.png](U4-upgrade-review-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review | 2026-10-27T16:05:45.862Z | localStorage v1 seeded |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-dom | 2026-10-27T15:20:32.149Z | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T15:20:32.747Z | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | local-appearance | 2026-10-27T15:20:33.512Z | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-27T15:20:34.184Z | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-27T15:20:34.798Z | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | compare-dark | 2026-10-27T15:20:35.480Z | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-compare | 2026-10-27T15:20:36.300Z | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T15:20:48.901Z | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T15:20:49.533Z | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-27T15:20:50.194Z | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T15:21:06.949Z | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T15:21:08.047Z | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-27T15:21:19.244Z | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | preview-dialog-close | 2026-10-27T15:21:19.843Z | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-27T15:24:45.750Z | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-27T15:24:55.728Z | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-27T15:24:56.313Z | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-27T15:25:07.224Z | none |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:21:36.346Z | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | dark | element | button-mode-dark | 2026-10-27T15:21:37.043Z | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-toggle-loading | 2026-10-27T15:21:37.732Z | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-default | 2026-10-27T15:21:38.351Z | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-color-gold | 2026-10-27T15:21:38.899Z | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-native-unavailable | 2026-10-27T15:21:39.466Z | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-scale-110 | 2026-10-27T15:21:40.105Z | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-theme-full | 2026-10-27T15:21:40.815Z | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-panel-chinese | 2026-10-27T15:21:41.552Z | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:21:56.228Z | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-27T15:22:10.061Z | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-27T15:22:10.664Z | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | type-link | 2026-10-27T15:22:11.226Z | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | viewport | — | 2026-10-27T15:22:25.952Z | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-classic-2 | 2026-10-27T15:22:40.660Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-solid-2 | 2026-10-27T15:22:41.227Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-soft-2 | 2026-10-27T15:22:41.793Z | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-surface-2 | 2026-10-27T15:22:42.359Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-outline-2 | 2026-10-27T15:22:42.926Z | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-offset-2 | 2026-10-27T15:22:43.493Z | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-2 | 2026-10-27T15:22:44.059Z | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-color-red | 2026-10-27T15:22:44.626Z | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-high-contrast | 2026-10-27T15:22:45.194Z | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-icon | 2026-10-27T15:22:45.759Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-solid-2 | 2026-10-27T15:22:46.375Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-soft-2 | 2026-10-27T15:22:46.942Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-outline-2 | 2026-10-27T15:22:47.510Z | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | type-link | 2026-10-27T15:23:01.927Z | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10419} root {"left":24,"top":1517.953125,"right":1256,"bottom":11919.578125} | — | 2026-10-27T15:23:17.480Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:18.089Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:18.695Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:19.291Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:19.884Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:20.475Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:21.071Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:21.684Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:22.280Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-27T15:23:22.900Z | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":11036} root {"left":24,"top":1517.953125,"right":1256,"bottom":12536.40625} | layout-scale-110 | 2026-10-27T15:23:23.864Z | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-110 | 2026-10-27T15:23:24.491Z | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10117} root {"left":24,"top":1517.953125,"right":1256,"bottom":11617.5} | layout-scale-95 | 2026-10-27T15:23:25.446Z | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-95 | 2026-10-27T15:23:26.050Z | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10456} root {"left":24,"top":1517.953125,"right":1256,"bottom":11956.578125} | layout-invalid | 2026-10-27T15:23:27.120Z | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | layout-invalid | 2026-10-27T15:23:27.673Z | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15685} root {"left":12,"top":2452.453125,"right":378,"bottom":18120.5625} | — | 2026-10-27T15:23:42.997Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:43.602Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:44.192Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:44.781Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:45.371Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:45.950Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:46.544Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:47.137Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:47.719Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-27T15:23:48.313Z | none |

## 未采与补采

- 目标包矩阵的全部格子首采均已采到，没有 missing。
- 首批评审后补采（同轮，原图全部保留）。只补评审报告看不出、且落在冻结矩阵内的格子：
  - U2：`U2-restore-1440-light`。Graphite + Pink + Auto 后，把编辑列滚到「Restore document」入口（F2 第三档重置）。
  - U3：
    - `U3-export-rust-mid-1440-light` 原想滚到首个字体文件处，但导出的 Rust 正文里没有 `.woff2`，脚本没有滚动，代码框停在顶部：与 `U3-export-rust-1440-light` 全图只有 15 个像素不同，代码区逐像素相同。这张不作中段证据；
    - `U3-export-rust-mid-1440-light-a2` 把代码框滚到纵向中点（scrollTop 531 / 1061），显示代码中段；
    - `U3-import-review-mid-390-light` 把 390 导入审阅滚到 Identity 组，显示配置下半段与 Identity 卡片。
  - U4：`U4-upgrade-review-json-1440-light`。升级 Review 中展开「Complete migration JSON」后滚到该节。
  - 补采 2 次运行，共 5 张，0 失败；页面健康同首采（pageerror、console error、失败请求均为 0）。操作记录已追加到各单元的 `U<n>-actions.log`。
- 首批评审报告的其他看不出项不在冻结矩阵内，未补采。包括：
  - 各单元的键盘焦点帧（U1 已有的除外）、悬停与按下态；
  - U1 以外单元的中文态；
  - GPU 失败态、预览宽度档；
  - 导入的语法错误、超限、不支持颜色，复制或下载失败，排队态，Locate 点击结果；
  - U4 除待确认外的 390 帧；
  - U6 夹具未提供的 IconButton 矩阵、多色 classic、gray / high-contrast / disabled 列、Link 字级与下划线模式；
  - U7 Restore prop 之后的帧，以及 390 下的缩放与非法态（目标包「采集」节只定义了 1280）。

## 主 agent 开图核对

逐张打开：视口图与元素图看缩略拼版；焦点特写、U7 分段和页面裁切的顶边按原尺寸查看。确认这些图都是本轮在 4181、4176 的真实页面上拍的，尺寸与视口或元素一致（视口图的像素 = CSS 视口 × DPR），不是报错页、空白页或旧图。各状态都已到达：

- U1：默认首屏；Graphite + Pink + Auto 后的基础配置区及其下半段；预览 Dark；中文；768；390 的编辑页与预览页；720×900 + 根字号 200% 及其导出对话框（含滚到动作区）；键盘帧依次为预设搜索、品牌色选择器、展开的列表、模式 Dark；字段标签页及其键盘焦点。
- U2：Colors 页选中 primary 及其来源区；单项解除后；品牌分组解除后；ring 的来源；字体与其他页；radius 输入 `abc` 回车后；background 与 foreground 都设为 #ffffff 并展开诊断；390 的来源。
- U3：导入审阅（1440、390，滚到 Apply，以及内层滚动区滚到底）；未知版本错误；导出 JSON v2、兼容 v1、CSS（profile 与颜色记法列表展开）、Rust（含复制后、滚到末尾）；390 导出。
- U4：升级待确认（1440、390）；升级 Review 及其底部；取消后；升级后；配额失败及操作列表；跨标签冲突、面板底部、面板之后、草稿差异展开、原文展开；无 Web Locks 的初始态与命名保存后。
- U5：DOM 预览浅色与深色；深色滚到 Caller class override；局部边界相反外观，及在边界内打开菜单与对话框；预览菜单；预览对话框（1440、390）；对比区嵌套 Dark；拾取（悬停卡片角、悬停并点击 Create profile、悬停并点击预览主按钮、拾取面板滚入视野）；DOM + GPU；390 预览页。
- U6：按钮夹具浅色、深色、390；加载；Default action、gold、Unavailable request 的键盘焦点；缩放 110%；主题圆角 full；ThemePanel 中文；Link 的默认、悬停、键盘焦点。DPR 2 特写中焦点都落在所列控件上（日志 `focused` 字段与 testid 一致）。
- U7：布局夹具 1280、390 的整段页面裁切与各分段；缩放 110% 与 95%；点 Invalid prop 之后的整段与局部视口帧。整段裁切的顶边 8px 内是夹具外的页面背景与宿主控件下沿。
