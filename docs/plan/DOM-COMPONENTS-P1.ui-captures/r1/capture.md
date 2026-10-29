# DOM-COMPONENTS-P1 UI 验收 R1 · 采集清单

- 模式：人工循环（本机未安装 Phoenix engine，`phoenix-ui engine-probe` 报缺二进制；未自动下载）。首轮、首次规范基线。
- 被测版本：HEAD `13ab1a5b5b0d99a08d0b482db452273f713e10ba`（main，dirty）。tracked diff SHA-256 `bbbeb25fde338b3c13efe2af43bfd188e859594ab90507fea81ade4a88ddf30c`（只含 SDLC 台账与两张旧验证 PNG）；产品路径 `crates examples xtask web makepad Cargo.toml Cargo.lock` 相对 HEAD diff 0 行、无未跟踪文件。
- 构建：本轮从 HEAD 重新 `mbx xtask build-web --release`。Theme Studio build `f7f81f184ae45ccd`（WASM 9,307,019 B），component-catalog build `dd1a11d8f8c000cb`（WASM 8,080,867 B），rustc 1.99.0。首次构建后 mbx 低空间 GC 立即回收了托管 `target/`（悬空 symlink，磁盘 98%）；按 M7 记录的处理把悬空 symlink 换成普通目录，以 `CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0` 重建，两次 build id 相同。
- 访问：`mbx xtask serve --example theme-studio --release --port 4181`、`--example component-catalog --release --port 4176`，strict CSP。
- 目标包：[DOM-COMPONENTS-P1.ui-target.md](../../DOM-COMPONENTS-P1.ui-target.md)；目标指纹 `manual-sha256-normative-sections-v1:0370d573f009a93136f30a52a0df584b141d8aa4a6c7b8766c4601a74b2bd324`（算法见同目录 [target-fingerprint.py](target-fingerprint.py)：「单元」「规范要点」两节 LF 文本 + 直接链接的 13 张目标图按路径排序、长度前缀组合；不声称与 Phoenix 算法兼容）。
- 条件：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，DPR 1，`reducedMotion: reduce`，`colorScheme: light`，`locale: en-US`；每图前等待 `document.fonts.ready`、networkidle 与连续 500ms 无 >100ms 帧。默认截可视区域；U6/U7 组件夹具为夹具根元素截图（整段，相当于目标图的整页口径）。新 browser context，localStorage 为空（U4 例外，见干预列）。
- 工具：[capture-runner.cjs](capture-runner.cjs)；原始采集日志 [capture-log.json](capture-log.json)（7 次运行，0 失败）；每单元实际操作记录 `U<n>-actions.log`（只记执行的动作与产出图）。
- 采集期页面健康：全部页面 pageerror 0、console error 0、失败请求/HTTP≥400 0。CSP 事件在页内收集但未回读，留作机器检查。

## 逐图登记

| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 截图方式 | 拍摄时间（UTC） | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 | default | [U1-default-1440-light.png](U1-default-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:51:46.274Z | none |
| U1 | configured | [U1-configured-1440-light.png](U1-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:51:47.295Z | none |
| U1 | configured | [U1-configured-1440-dark.png](U1-configured-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | 2026-10-27T05:51:47.875Z | none |
| U1 | top | [U1-top-1440-dark.png](U1-top-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | 2026-10-27T05:51:48.421Z | none |
| U1 | zh-top | [U1-zh-top-1440-light.png](U1-zh-top-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:51:49.106Z | none |
| U1 | zh-configured | [U1-zh-configured-1440-light.png](U1-zh-configured-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:51:49.656Z | none |
| U1 | default | [U1-default-768-light.png](U1-default-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | 2026-10-27T05:51:57.054Z | none |
| U1 | configured | [U1-configured-768-light.png](U1-configured-768-light.png) | http://127.0.0.1:4181/ | 768x1024 / 1 | light | viewport | 2026-10-27T05:51:58.036Z | none |
| U1 | edit | [U1-edit-390-light.png](U1-edit-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:51:59.793Z | none |
| U1 | configured | [U1-configured-390-light.png](U1-configured-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:52:00.560Z | none |
| U1 | preview | [U1-preview-390-light.png](U1-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:52:08.642Z | none |
| U1 | zoom200 | [U1-zoom200-720-light.png](U1-zoom200-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | 2026-10-27T05:52:19.752Z | none |
| U1 | zoom200-export | [U1-zoom200-export-720-light.png](U1-zoom200-export-720-light.png) | http://127.0.0.1:4181/ | 720x900 / 1 | light | viewport | 2026-10-27T05:52:20.342Z | none |
| U1 | keyboard-01 | [U1-keyboard-01-1440-light.png](U1-keyboard-01-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:52:30.031Z | none |
| U1 | keyboard-02 | [U1-keyboard-02-1440-light.png](U1-keyboard-02-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:52:30.602Z | none |
| U1 | keyboard-03 | [U1-keyboard-03-1440-light.png](U1-keyboard-03-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:52:31.164Z | none |
| U1 | keyboard-04 | [U1-keyboard-04-1440-light.png](U1-keyboard-04-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:52:31.750Z | none |
| U2 | source-overridden | [U2-source-overridden-1440-light.png](U2-source-overridden-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:15.900Z | none |
| U2 | source-overridden-detail | [U2-source-overridden-detail-1440-light.png](U2-source-overridden-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:16.458Z | none |
| U2 | token-released | [U2-token-released-1440-light.png](U2-token-released-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:17.064Z | none |
| U2 | group-reset | [U2-group-reset-1440-light.png](U2-group-reset-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:17.793Z | none |
| U2 | source-generated | [U2-source-generated-1440-light.png](U2-source-generated-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:18.412Z | none |
| U2 | controls | [U2-controls-1440-light.png](U2-controls-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:19.022Z | none |
| U2 | field-error | [U2-field-error-1440-light.png](U2-field-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:19.604Z | none |
| U2 | low-contrast | [U2-low-contrast-1440-light.png](U2-low-contrast-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:53:29.125Z | none |
| U2 | source-overridden | [U2-source-overridden-390-light.png](U2-source-overridden-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:53:31.242Z | none |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:00.291Z | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:00.868Z | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:01.463Z | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:02.594Z | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:03.233Z | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:03.875Z | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:04.505Z | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:05.405Z | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:54:07.354Z | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:54:08.137Z | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T06:20:50.064Z | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:20:59.936Z | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:00.535Z | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:01.179Z | none |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:32.819Z | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:33.421Z | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:34.134Z | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:36.251Z | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:54:39.112Z | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:47.920Z | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:48.502Z | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:58.443Z | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:54:59.010Z | second same-origin tab wrote v2 |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:08.218Z | navigator.locks removed before load |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:11.961Z | localStorage v1 seeded |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:23.627Z | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:33.177Z | navigator.locks removed before load |
| U5 | dom | [U5-dom-1440-light.png](U5-dom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:31.553Z | none |
| U5 | dom | [U5-dom-1440-dark.png](U5-dom-1440-dark.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | dark | viewport | 2026-10-27T05:55:32.136Z | none |
| U5 | menu | [U5-menu-1440-light.png](U5-menu-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:32.753Z | none |
| U5 | dialog | [U5-dialog-1440-light.png](U5-dialog-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:33.337Z | none |
| U5 | compare-nested | [U5-compare-nested-1440-light.png](U5-compare-nested-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:34.003Z | none |
| U5 | inspector | [U5-inspector-1440-light.png](U5-inspector-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:34.617Z | none |
| U5 | gpu-compare | [U5-gpu-compare-1440-light.png](U5-gpu-compare-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:55:35.475Z | none |
| U5 | preview | [U5-preview-390-light.png](U5-preview-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:55:43.696Z | none |
| U5 | dialog | [U5-dialog-390-light.png](U5-dialog-390-light.png) | http://127.0.0.1:4181/ | 390x844 / 1 | light | viewport | 2026-10-27T05:55:44.263Z | none |
| U5 | inspector-card | [U5-inspector-card-1440-light.png](U5-inspector-card-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:56:27.806Z | none |
| U5 | inspector | [U5-inspector-1440-light-a2.png](U5-inspector-1440-light-a2.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:56:28.418Z | none |
| U5 | inspector-picked | [U5-inspector-picked-1440-light.png](U5-inspector-picked-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T05:56:29.015Z | none |
| U5 | inspector-undeclared-hover | [U5-inspector-undeclared-hover-1440-light.png](U5-inspector-undeclared-hover-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:44.465Z | none |
| U5 | inspector-undeclared | [U5-inspector-undeclared-1440-light.png](U5-inspector-undeclared-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 / 1 | light | viewport | 2026-10-27T06:21:45.550Z | none |
| U6 | fixture | [U6-fixture-1280-light.png](U6-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:56:52.699Z | none |
| U6 | fixture | [U6-fixture-1280-dark.png](U6-fixture-1280-dark.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | dark | element | 2026-10-27T05:56:53.370Z | none |
| U6 | loading | [U6-loading-1280-light.png](U6-loading-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:56:54.042Z | none |
| U6 | focus | [U6-focus-1280-light.png](U6-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:56:54.644Z | none |
| U6 | focus-02 | [U6-focus-02-1280-light.png](U6-focus-02-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:56:55.195Z | none |
| U6 | focus-unavailable | [U6-focus-unavailable-1280-light.png](U6-focus-unavailable-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:56:55.761Z | none |
| U6 | scale110 | [U6-scale110-1280-light.png](U6-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:56:56.383Z | none |
| U6 | radius-full | [U6-radius-full-1280-light.png](U6-radius-full-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:56:57.071Z | none |
| U6 | panel-zh | [U6-panel-zh-1280-light.png](U6-panel-zh-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:56:57.696Z | none |
| U6 | fixture | [U6-fixture-390-light.png](U6-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | 2026-10-27T05:57:11.239Z | none |
| U6 | links | [U6-links-1280-light.png](U6-links-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:57:25.030Z | none |
| U6 | links-hover | [U6-links-hover-1280-light.png](U6-links-hover-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:57:25.630Z | none |
| U6 | links-focus | [U6-links-focus-1280-light.png](U6-links-focus-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:57:26.180Z | none |
| U6 | links | [U6-links-390-light.png](U6-links-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | viewport | 2026-10-27T05:57:39.662Z | none |
| U6 | focus | [U6-focus-1280-light-a2.png](U6-focus-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T06:22:00.491Z | none |
| U6 | focus-02 | [U6-focus-02-1280-light-a2.png](U6-focus-02-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T06:22:01.040Z | none |
| U6 | scale110 | [U6-scale110-1280-light-a2.png](U6-scale110-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T06:22:01.681Z | none |
| U7 | fixture | [U7-fixture-1280-light.png](U7-fixture-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:57:53.505Z | none |
| U7 | scale110 | [U7-scale110-1280-light.png](U7-scale110-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:57:54.158Z | none |
| U7 | scale95 | [U7-scale95-1280-light.png](U7-scale95-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T05:57:54.805Z | none |
| U7 | invalid | [U7-invalid-1280-light.png](U7-invalid-1280-light.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | viewport | 2026-10-27T05:57:55.432Z | none |
| U7 | fixture | [U7-fixture-390-light.png](U7-fixture-390-light.png) | http://127.0.0.1:4176/ | 390x844 / 1 | light | element | 2026-10-27T05:58:09.224Z | none |
| U7 | invalid | [U7-invalid-1280-light-a2.png](U7-invalid-1280-light-a2.png) | http://127.0.0.1:4176/ | 1280x900 / 1 | light | element | 2026-10-27T06:22:15.852Z | none |

## 未采与补采

- 目标包矩阵的全部格子均已采到，无 missing。
- U5 拾取：首采 `U5-inspector-1440-light.png` 点击的预览节点不是已声明样本，画面只有拾取提示条、没有选中轮廓或面板——不能证明该状态。保留原图，同轮补采 `U5-inspector-1440-light-a2.png`（悬停「Create profile」按钮），并补拍前后关键帧 `U5-inspector-card-1440-light.png`（悬停卡片角）与 `U5-inspector-picked-1440-light.png`（点击后编辑器聚焦 token）。评审以 a2 及前后帧为准，原图不作为该状态证据。
- 首批评审后补采（同轮，评审报告看不出的目标格子；原图全部保留）：
  - U3：`U3-import-review-end-390-light.png`（390 导入审阅滚到 Apply）、`U3-export-css-profile-open-1440-light.png` 与 `U3-export-css-notation-open-1440-light.png`（CSS profile 与颜色记法列表展开）、`U3-export-rust-end-1440-light.png`（Rust 接入代码滚到末尾）。
  - U4：`U4-upgrade-review-end-1440-light.png`（升级 Review 滚到确认按钮）、`U4-conflict-raw-1440-light.png`（冲突原文展开）、`U4-no-locks-initial-1440-light.png`（无 Web Locks 打开后、任何动作之前）。
  - U5：`U5-inspector-undeclared-hover-1440-light.png`、`U5-inspector-undeclared-1440-light.png`（开启拾取后等待 800ms，悬停并点击预览主按钮 `preview-primary`，再等 500ms）。
  - U6：`U6-focus-1280-light-a2.png`、`U6-focus-02-1280-light-a2.png`、`U6-scale110-1280-light-a2.png`——首采焦点帧与缩放帧拍摄时指针仍停在先前点击的按钮上，可能带悬停色；补采前把指针移到 (0,0)。
  - U7：`U7-invalid-1280-light-a2.png`（点 Invalid prop 后整段夹具元素截图，指针移开）。
- U3「导入错误」按目标包采集节使用 `{"schema_version":999}`；画面显示的是该输入的实际错误文案。

## 主 agent 开图核对

逐张打开（大图用缩略拼版与局部放大核对），确认均为本轮在 4181/4176 真实页面拍摄，尺寸与视口一致（元素截图为元素实际尺寸），不是报错页、空白页或旧图：

- U1：默认首屏、Graphite+pink+Auto 后的基础配置区（出现 Overridden 计数与「Use configuration values」）、预览 Dark、中文、768、390 编辑/预览页、720×900+根字号 200% 及其导出对话框；键盘帧依次为预设搜索、品牌色选择器、展开的列表、模式 Dark 获得焦点。
- U2：primary 显示「Explicit document override」及「Clear this token · current mode」；解除后 primary 显示「Generated by base configuration」；品牌分组解除；ring 来源；字体与其他页；radius 输入 `abc` 回车后字段为红框并显示「Complete a valid value. The effective theme is unchanged.」，此时输入框内显示 `0.35rem`；低对比诊断「1.000:1 · Below text thresholds」。
- U3：导入审阅显示「JSON v1 → v2」、名称与差异表、导入备注与 Apply；错误态显示红色失败文案；导出 JSON v2 / 兼容 v1（损失清单）/ Tailwind v4 CSS（profile 与颜色记法）/ Rust 接入；复制后「Copied to clipboard.」。
- U4：待确认、Review（页内迁移候选）、取消后仅内存与命名保存受阻提示、升级成功（Saved on this device、下载升级前原始数据）、配额失败红字与操作列表、跨标签冲突三按钮、无 Web Locks 提示，均可见。
- U5：DOM 预览浅/深、预览菜单、预览对话框、对比区嵌套 Dark、拾取（a2）、DOM+GPU 对比（GPU Ready）、390 预览页与对话框。
- U6：按钮矩阵浅/深、加载（两个按钮显示 spinner 并保留宽度）、Default action / gold / Unavailable request 的键盘焦点环、缩放 110% 矩阵、主题 full 圆角、ThemePanel 中文、Link 默认/悬停/聚焦（布局夹具）。
- U7：布局与排版夹具 1280/390、缩放 110%/95%、非法属性诊断。
