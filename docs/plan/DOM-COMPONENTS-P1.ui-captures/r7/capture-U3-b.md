# UI 验收 R7 · U3 采集清单

- 被测版本：HEAD `b218075a3a2107ab511da932f831f42adfa9390c`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `1fdbbbc39758b56531e996384ad7397645d59a7115ea3fcb6c1b39b326a98600`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `6d1605ccec8b9f78`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R6 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r6.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R6，无 managed 索引；本轮 R7，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 38 张。操作记录：[U3-actions.log](U3-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:08.282Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:08.850Z | import-review | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:09.447Z | import-review | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:10.729Z | data-tools-dialog-close | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:11.370Z | export-kind | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:12.051Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:12.642Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:13.241Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:13.893Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:14.791Z | export-copy | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:15.351Z | export-copy | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:23:17.389Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:23:17.937Z | import-review | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:23:18.822Z | data-tools-dialog-close | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:40.275Z | import-review | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:27:42.325Z | import-review | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:28:39.322Z | import-review | none |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:00.866Z | export-kind | none |
| U3 | export-kind-keyboard-focus | [U3-export-kind-keyboard-focus-1440-light.png](U3-export-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:41.886Z | export-kind | none |
| U3 | export-kind-keyboard-open | [U3-export-kind-keyboard-open-1440-light.png](U3-export-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:42.452Z | export-kind-list | none |
| U3 | export-profile-keyboard-focus | [U3-export-profile-keyboard-focus-1440-light.png](U3-export-profile-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:43.820Z | export-profile | none |
| U3 | export-color-format-keyboard-focus | [U3-export-color-format-keyboard-focus-1440-light.png](U3-export-color-format-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:45.087Z | export-color-format | none |
| U3 | export-next-keyboard-focus | [U3-export-next-keyboard-focus-1440-light.png](U3-export-next-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:46.351Z | export-source | none |
| U3 | import-kind-keyboard-focus | [U3-import-kind-keyboard-focus-1440-light.png](U3-import-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:47.673Z | import-format | none |
| U3 | import-kind-keyboard-open | [U3-import-kind-keyboard-open-1440-light.png](U3-import-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:29:48.226Z | import-format-list | none |
| U3 | export-v1 | [U3-export-v1-390-light.png](U3-export-v1-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:33:58.791Z | export-kind | none |
| U3 | export-css | [U3-export-css-390-light.png](U3-export-css-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:33:59.424Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-390-light.png](U3-export-css-profile-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:33:59.991Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-390-light.png](U3-export-css-notation-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:00.572Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-390-light.png](U3-export-rust-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:01.190Z | export-kind | none |
| U3 | export-rust-mid | [U3-export-rust-mid-390-light.png](U3-export-rust-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:01.740Z | export-kind | none |
| U3 | export-rust-end | [U3-export-rust-end-390-light.png](U3-export-rust-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:02.289Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-390-light.png](U3-export-copied-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:03.156Z | export-copy | none |
| U3 | import-error | [U3-import-error-390-light.png](U3-import-error-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:34:03.787Z | import-review | none |
| U3 | export-copied-feedback | [U3-export-copied-feedback-390-light.png](U3-export-copied-feedback-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:46:45.187Z | export-copy | none |
| U3 | import-review-config-appearance | [U3-import-review-config-appearance-390-light.png](U3-import-review-config-appearance-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:50:50.010Z | import-review | none |
| U3 | import-review-config-scaling | [U3-import-review-config-scaling-390-light.png](U3-import-review-config-scaling-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:50:50.557Z | import-review | none |
| U3 | export-v1-actions | [U3-export-v1-actions-390-light.png](U3-export-v1-actions-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:50:51.544Z | export-kind | none |
