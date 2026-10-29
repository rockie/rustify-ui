# DOM-COMPONENTS-P1 UI 验收 R4 · 采集清单

- 模式：人工循环（本机没有 `phoenix-ui`，未自动下载）。R4 是全量轮，**不是重建基线**：目标指纹与 R3 相同。
- 被测版本：HEAD `5d122d6231c690a61080b91c680e9a514d2034b8`（main）。工作树 dirty 只涉及 SDLC 台账、两张旧验证 PNG 与未跟踪的评审材料；产品路径 `crates examples xtask web makepad public Cargo.toml Cargo.lock sources.lock.json` 相对 HEAD 无 diff，也没有未跟踪文件。
- 构建：本轮从 HEAD 重新 `mbx xtask build-web --release`。Theme Studio build `28ae38531384c902`（WASM 9,893,303 B），component-catalog build `32b52b66f04701ad`（WASM 8,203,814 B）。
- 访问：`mbx xtask serve --dir <产物副本> --port 4181`（Studio）、`--port 4176`（catalog），响应头为 strict CSP。产物副本是构建完成后从 `target/makepad-wasm-app/release/<example>` 原样复制出的目录，原因见「未采与补采」中的中断说明；采集前用 `curl …/build-manifest.json` 确认两个服务发出的正是上面两个 build id。
- 目标包：[DOM-COMPONENTS-P1.ui-target.md](../../DOM-COMPONENTS-P1.ui-target.md)；目标指纹 `manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`（算法见 [target-fingerprint.py](target-fingerprint.py)，附件清单见 [target-files.txt](target-files.txt)），与 R3 相同。
- 条件：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，`reducedMotion: reduce`，`colorScheme: light`，`locale: en-US`。每张图拍摄前等待 `document.fonts.ready`、networkidle，以及连续 500ms 没有超过 100ms 的帧。DPR 1；文件名带 `-dpr2` 的特写为 DPR 2。默认截可视区域。U6、U7 的分段用元素截图；U7 整段夹具帧用页面裁切（夹具根 boundingBox 向外取整，四周各留 8px）；B2 焦点特写是控件四周 24px 的裁切。每个流程新开 browser context，localStorage 为空（U4 例外，见「干预」列）。
- 工具：[capture-runner.cjs](capture-runner.cjs)（以 R3 脚本为底）。原始日志 [capture-log.json](capture-log.json)：2 次运行，首采 140 张（2026-10-28T04:00:48Z–04:08:21Z），补采 2 张（04:22:01Z–04:22:02Z），0 失败。每单元的实际操作记录在 `U<n>-actions.log`。
- 采集期页面健康：两次运行共 43 个页面，pageerror、console error、失败请求、HTTP ≥ 400、CSP 违规均为 0。

## 与 R3 采集的差异

只改了采集做法，目标包的「单元」「规范要点」没有改变，见目标包变更记录末三条。

- **R3 的补采帧并入首采**：`U2-restore-1440-light`、`U3-import-review-mid-390-light`、`U4-upgrade-review-json-1440-light`；`U3-export-rust-mid-1440-light` 直接把代码框滚到纵向中点（R3 首次用的「滚到首个字体文件」没有滚动，R3 以 `-a2` 为准），本轮不再有 `-a2`。
- 「Complete … JSON」展开节按界面现名选取：本轮界面写作「Complete library upgrade JSON」（R3 为「Complete migration JSON」）。
- **U1 · 200% 新增两帧**（R3 机器检查在这里发现 UI-192，R3 报告建议并入）：
  - `U1-zoom200-colors-720-light`：另开一页，同样 720×900 + 根字号 200%，点 Colors 标签页，把首个 token（background）滚到编辑列顶部；
  - `U1-zoom200-source-720-light-a2` 与 `U1-zoom200-source-end-720-light`：再选 primary，等页面静止后把来源块滚到顶部；来源块高于视口，另拍其末尾（含解除按钮）。见下文补采。

## 逐图登记

| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 截图方式 | 键盘焦点所在 | 拍摄时间（UTC） | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:00:48.355Z | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-28T04:00:49.636Z | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-28T04:00:50.236Z | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-28T04:00:50.800Z | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-28T04:00:51.571Z | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | locale-zh | 2026-10-28T04:00:52.136Z | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | — | 2026-10-28T04:01:02.040Z | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | studio-config-gray_color | 2026-10-28T04:01:03.474Z | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-28T04:01:05.438Z | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | studio-config-gray_color | 2026-10-28T04:01:06.288Z | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-28T04:01:14.420Z | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | — | 2026-10-28T04:01:23.927Z | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-28T04:01:24.548Z | none |
| U1 | zoom200-colors | [U1-zoom200-colors-720-light.png](U1-zoom200-colors-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | editor-colors | 2026-10-28T04:01:34.138Z | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light.png](U1-zoom200-source-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | color-token-primary | 2026-10-28T04:01:34.752Z | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preset-search | 2026-10-28T04:01:44.055Z | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color | 2026-10-28T04:01:44.619Z | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-accent_color-list | 2026-10-28T04:01:45.187Z | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | mode-dark | 2026-10-28T04:01:45.962Z | none |
| U1 | zoom200-export-end | [U1-zoom200-export-end-720-light.png](U1-zoom200-export-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-28T04:06:40.570Z | none |
| U1 | field-tabs | [U1-field-tabs-1440-light.png](U1-field-tabs-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:06:50.225Z | none |
| U1 | field-tabs-focus | [U1-field-tabs-focus-1440-light.png](U1-field-tabs-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-colors | 2026-10-28T04:06:50.804Z | none |
| U1 | configured-lower | [U1-configured-lower-1440-light.png](U1-configured-lower-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-28T04:06:52.069Z | none |
| U1 | zoom200-source | [U1-zoom200-source-720-light-a2.png](U1-zoom200-source-720-light-a2.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | color-token-primary | 2026-10-28T04:22:01.917Z | none |
| U1 | zoom200-source-end | [U1-zoom200-source-end-720-light.png](U1-zoom200-source-end-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | color-token-primary | 2026-10-28T04:22:02.981Z | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-28T04:01:56.652Z | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-primary | 2026-10-28T04:01:57.232Z | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:01:57.823Z | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:01:58.554Z | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-token-ring | 2026-10-28T04:01:59.167Z | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-controls | 2026-10-28T04:01:59.767Z | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | edit-radius | 2026-10-28T04:02:00.369Z | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | editor-shell | 2026-10-28T04:02:10.893Z | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | color-token-primary | 2026-10-28T04:02:13.212Z | none |
| U2 | restore | [U2-restore-1440-light.png](U2-restore-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | studio-config-gray_color | 2026-10-28T04:07:57.640Z | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-28T04:02:23.182Z | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-28T04:02:23.763Z | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-28T04:02:24.359Z | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | data-tools-dialog-close | 2026-10-28T04:02:25.757Z | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-28T04:02:26.396Z | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-28T04:02:27.054Z | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-profile-list | 2026-10-28T04:02:27.652Z | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-color-format-list | 2026-10-28T04:02:28.253Z | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-28T04:02:28.918Z | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-28T04:02:29.821Z | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-copy | 2026-10-28T04:02:30.399Z | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-28T04:02:32.497Z | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-28T04:02:33.046Z | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | data-tools-dialog-close | 2026-10-28T04:02:33.929Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | import-review | 2026-10-28T04:07:02.005Z | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-28T04:07:04.142Z | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | import-review | 2026-10-28T04:07:59.723Z | none |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | export-kind | 2026-10-28T04:08:21.736Z | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:02:44.452Z | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-28T04:02:45.070Z | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review-close | 2026-10-28T04:02:45.638Z | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-28T04:02:46.402Z | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:02:48.583Z | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | — | 2026-10-28T04:02:51.730Z | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-28T04:03:01.938Z | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-operations | 2026-10-28T04:03:02.537Z | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-28T04:03:13.829Z | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-28T04:03:14.394Z | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-external-raw | 2026-10-28T04:03:14.995Z | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | — | 2026-10-28T04:03:24.230Z | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | theme-save | 2026-10-28T04:03:25.128Z | navigator.locks removed before load |
| U4 | conflict-detail-end | [U4-conflict-detail-end-1440-light.png](U4-conflict-detail-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-28T04:07:15.696Z | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-28T04:07:16.246Z | second same-origin tab wrote v2 |
| U4 | conflict-diff | [U4-conflict-diff-1440-light.png](U4-conflict-diff-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | storage-conflict-diff | 2026-10-28T04:07:16.861Z | second same-origin tab wrote v2 |
| U4 | upgrade-review-json | [U4-upgrade-review-json-1440-light.png](U4-upgrade-review-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | upgrade-review | 2026-10-28T04:08:10.763Z | localStorage v1 seeded |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-dom | 2026-10-28T04:03:34.702Z | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-28T04:03:35.287Z | none |
| U5 | local-opposite | [U5-local-opposite-1440-light.png](U5-local-opposite-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | local-appearance | 2026-10-28T04:03:36.000Z | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-28T04:03:36.651Z | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-28T04:03:37.235Z | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | compare-dark | 2026-10-28T04:03:37.904Z | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | renderer-compare | 2026-10-28T04:03:38.754Z | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-28T04:03:48.682Z | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-28T04:03:49.283Z | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | color-value | 2026-10-28T04:03:49.906Z | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-28T04:04:00.482Z | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-28T04:04:01.580Z | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | mobile-preview | 2026-10-28T04:04:10.327Z | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | preview-dialog-close | 2026-10-28T04:04:10.894Z | none |
| U5 | dom-caller | [U5-dom-caller-1440-dark.png](U5-dom-caller-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | mode-dark | 2026-10-28T04:07:26.260Z | none |
| U5 | local-opposite-menu | [U5-local-opposite-menu-1440-light.png](U5-local-opposite-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | menu-item-duplicate | 2026-10-28T04:07:35.572Z | none |
| U5 | local-opposite-dialog | [U5-local-opposite-dialog-1440-light.png](U5-local-opposite-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | preview-dialog-close | 2026-10-28T04:07:36.171Z | none |
| U5 | inspector-undeclared-panel | [U5-inspector-undeclared-panel-1440-light.png](U5-inspector-undeclared-panel-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | inspector-toggle | 2026-10-28T04:07:47.315Z | none |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:04:24.907Z | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | dark | element | button-mode-dark | 2026-10-28T04:04:25.591Z | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-toggle-loading | 2026-10-28T04:04:26.262Z | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-default | 2026-10-28T04:04:26.881Z | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-color-gold | 2026-10-28T04:04:27.431Z | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | button-native-unavailable | 2026-10-28T04:04:27.982Z | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-scale-110 | 2026-10-28T04:04:28.622Z | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-theme-full | 2026-10-28T04:04:29.325Z | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | button-panel-chinese | 2026-10-28T04:04:29.964Z | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:04:43.238Z | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-28T04:04:57.772Z | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | — | 2026-10-28T04:04:58.372Z | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | type-link | 2026-10-28T04:04:58.942Z | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | viewport | — | 2026-10-28T04:05:12.862Z | none |
| U6 | focus-classic | [U6-focus-classic-1280-light-dpr2.png](U6-focus-classic-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-classic-2 | 2026-10-28T04:05:26.733Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-light-dpr2.png](U6-focus-solid-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-solid-2 | 2026-10-28T04:05:27.299Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-light-dpr2.png](U6-focus-soft-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-soft-2 | 2026-10-28T04:05:27.866Z | none |
| U6 | focus-surface | [U6-focus-surface-1280-light-dpr2.png](U6-focus-surface-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-surface-2 | 2026-10-28T04:05:28.432Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-light-dpr2.png](U6-focus-outline-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-outline-2 | 2026-10-28T04:05:28.998Z | none |
| U6 | focus-ghost-offset | [U6-focus-ghost-offset-1280-light-dpr2.png](U6-focus-ghost-offset-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-offset-2 | 2026-10-28T04:05:29.566Z | none |
| U6 | focus-ghost | [U6-focus-ghost-1280-light-dpr2.png](U6-focus-ghost-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-ghost-2 | 2026-10-28T04:05:30.133Z | none |
| U6 | focus-color-red | [U6-focus-color-red-1280-light-dpr2.png](U6-focus-color-red-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-color-red | 2026-10-28T04:05:30.699Z | none |
| U6 | focus-high-contrast | [U6-focus-high-contrast-1280-light-dpr2.png](U6-focus-high-contrast-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-high-contrast | 2026-10-28T04:05:31.265Z | none |
| U6 | focus-icon | [U6-focus-icon-1280-light-dpr2.png](U6-focus-icon-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | button-icon | 2026-10-28T04:05:31.832Z | none |
| U6 | focus-solid | [U6-focus-solid-1280-dark-dpr2.png](U6-focus-solid-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-solid-2 | 2026-10-28T04:05:32.449Z | none |
| U6 | focus-soft | [U6-focus-soft-1280-dark-dpr2.png](U6-focus-soft-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-soft-2 | 2026-10-28T04:05:33.015Z | none |
| U6 | focus-outline | [U6-focus-outline-1280-dark-dpr2.png](U6-focus-outline-1280-dark-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | dark | clip | button-outline-2 | 2026-10-28T04:05:33.582Z | none |
| U6 | focus-link | [U6-focus-link-1280-light-dpr2.png](U6-focus-link-1280-light-dpr2.png) | http://127.0.0.1:4176/ | 1280x900 / 2 | light | clip | type-link | 2026-10-28T04:05:47.343Z | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10314} root {"left":24,"top":1517.953125,"right":1256,"bottom":11814.578125} | — | 2026-10-28T04:06:01.574Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-1280-light.png](U7-type-heading-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:02.183Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-1280-light.png](U7-type-text-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:02.772Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-1280-light.png](U7-type-code-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:03.374Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-1280-light.png](U7-type-kbd-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:03.966Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-1280-light.png](U7-type-blockquote-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:04.541Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-1280-light.png](U7-type-wrap-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:05.133Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-1280-light.png](U7-layout-container-matrix-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:05.719Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-1280-light.png](U7-layout-inset-demonstration-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:06.313Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-1280-light.png](U7-layout-reset-comparison-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | — | 2026-10-28T04:06:06.923Z | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10923} root {"left":24,"top":1517.953125,"right":1256,"bottom":12423.90625} | layout-scale-110 | 2026-10-28T04:06:07.893Z | none |
| U7 | scale110-heading | [U7-scale110-heading-1280-light.png](U7-scale110-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-110 | 2026-10-28T04:06:08.501Z | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10016} root {"left":24,"top":1517.953125,"right":1256,"bottom":11516.25} | layout-scale-95 | 2026-10-28T04:06:09.380Z | none |
| U7 | scale95-heading | [U7-scale95-heading-1280-light.png](U7-scale95-heading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | layout-scale-95 | 2026-10-28T04:06:09.981Z | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | page-clip {"x":16,"y":1509,"width":1248,"height":10351} root {"left":24,"top":1517.953125,"right":1256,"bottom":11851.578125} | layout-invalid | 2026-10-28T04:06:10.931Z | none |
| U7 | invalid-detail | [U7-invalid-detail-1280-light.png](U7-invalid-detail-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | layout-invalid | 2026-10-28T04:06:11.488Z | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | page-clip {"x":4,"y":2444,"width":382,"height":15439} root {"left":12,"top":2452.453125,"right":378,"bottom":17874.5625} | — | 2026-10-28T04:06:25.602Z | none |
| U7 | type-heading-matrix | [U7-type-heading-matrix-390-light.png](U7-type-heading-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:26.208Z | none |
| U7 | type-text-matrix | [U7-type-text-matrix-390-light.png](U7-type-text-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:26.816Z | none |
| U7 | type-code-matrix | [U7-type-code-matrix-390-light.png](U7-type-code-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:27.419Z | none |
| U7 | type-kbd-matrix | [U7-type-kbd-matrix-390-light.png](U7-type-kbd-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:28.009Z | none |
| U7 | type-blockquote-matrix | [U7-type-blockquote-matrix-390-light.png](U7-type-blockquote-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:28.590Z | none |
| U7 | type-wrap-matrix | [U7-type-wrap-matrix-390-light.png](U7-type-wrap-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:29.183Z | none |
| U7 | layout-container-matrix | [U7-layout-container-matrix-390-light.png](U7-layout-container-matrix-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:29.777Z | none |
| U7 | layout-inset-demonstration | [U7-layout-inset-demonstration-390-light.png](U7-layout-inset-demonstration-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:30.359Z | none |
| U7 | layout-reset-comparison | [U7-layout-reset-comparison-390-light.png](U7-layout-reset-comparison-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | — | 2026-10-28T04:06:30.955Z | none |

## 未采与补采

- 目标包矩阵的全部格子都已采到，没有 missing。
- **补采**：`U1-zoom200-source-720-light` 首拍没到达定义的步骤。脚本在点选 primary 后立刻把来源块滚到顶部，界面随后自己把所选格滚回视野，来源块只露出标题，有效值、路径与解除按钮都在折线以下。原图保留，不作证据。补采 `-a2` 先等页面静止再滚动，来源块从 y=317 开始、到 y=1207 结束，高于 900 的视口，所以另拍 `U1-zoom200-source-end-720-light`，让块末尾对齐视口底边。
- **中断的两次运行**（不作证据，原图与日志保留在子目录）：
  - [aborted-run1/](aborted-run1/)：69 张。服务当时直接读 mbx 托管的 `target/`，采集到 U6 时托管目录被 mbx 回收，此后页面全部 404，U6、U6F、U7、EXTRA、SUP、SUP2 失败；
  - [aborted-run2/](aborted-run2/)：48 张。重建并复制产物后，旧服务进程没有停干净，仍占着端口读托管目录，新的 `--dir` 服务绑定失败；托管目录在 U4 时再次被回收。
  - 停掉旧进程、改用 `--dir` 服务产物副本后，第 3 次运行 140 张全部成功，即本清单登记的首采。三次运行的 build id 相同。

## 主 agent 开图核对

按单元拼成缩略图逐张查看，`U1-zoom200-source` 三张按原尺寸查看。确认这些图都是本轮在 4181、4176 的真实页面上拍的，视口图的像素 = CSS 视口 × DPR（140 张逐张核对，0 不符），不是报错页、空白页或旧图。各状态都已到达：

- U1：默认首屏；Graphite + Pink + Auto 后的基础配置区及其下半段；预览 Dark；中文；768；390 的编辑页与预览页；720×900 + 根字号 200% 的首屏、导出对话框（含滚到动作区）、Colors 网格、primary 来源块（`-a2` 与末尾）；键盘帧依次为预设搜索、品牌色选择器、展开的列表、模式 Dark；字段标签页及其键盘焦点。
- U2：Colors 页选中 primary 及其来源区；单项解除后；品牌分组解除后；ring 的来源；字体与其他页；radius 输入 `abc` 回车后；background 与 foreground 都设为 #ffffff 并展开诊断；390 的来源；「Restore document」入口。
- U3：导入审阅（1440、390，滚到 Apply、滚到底，390 中段）；未知版本错误；导出 JSON v2、兼容 v1、CSS（profile 与颜色记法列表展开）、Rust（含复制后、中段、末尾）；390 导出。
- U4：升级待确认（1440、390）；升级 Review、其底部、展开完整迁移 JSON；取消后；升级后；配额失败及操作列表；跨标签冲突、面板底部、面板之后、草稿差异展开、原文展开；无 Web Locks 的初始态与命名保存后。
- U5：DOM 预览浅色与深色；深色滚到 Caller class override；局部边界相反外观，及在边界内打开菜单与对话框；预览菜单；预览对话框（1440、390）；对比区嵌套 Dark；拾取（悬停卡片角、悬停并点击 Create profile、悬停并点击预览主按钮、拾取面板滚入视野）；DOM + GPU；390 预览页。
- U6：按钮夹具浅色、深色、390；加载；Default action、gold、Unavailable request 的键盘焦点；缩放 110%；主题圆角 full；ThemePanel 中文；Link 的默认、悬停、键盘焦点；DPR 2 焦点特写（日志 `focused` 字段与 testid 一致）。
- U7：布局夹具 1280、390 的整段页面裁切与各分段；缩放 110% 与 95%；点 Invalid prop 之后的整段与局部视口帧。
