# UI 验收 R8 · U3 采集清单

- 被测版本：HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `105da89e37304db9`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

- 补采环境：初次对应流程因构建输出目录缺失而收到 HTTP 404，未拍到的格子登记为 attempt 1 缺证；本轮从同一源码重建，两个 build_id 与初采一致，逐文件校验后复制到独立临时目录，仍通过仓库 `mbx xtask serve --dir ... --csp strict` 在相同 URL 服务。文件名 `-a2` 为这些格子的第二次采集；视口、DPR、语言、主题、字体等待及操作步骤保持原条件。原图和失败轨迹保留。

共 38 张。操作记录：[U3-actions.log](U3-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U3 | import-review | [U3-import-review-1440-light.png](U3-import-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:31.291Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-1440-light.png](U3-import-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:31.872Z | import-review | none |
| U3 | import-error | [U3-import-error-1440-light.png](U3-import-error-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:32.469Z | import-review | none |
| U3 | export-json | [U3-export-json-1440-light.png](U3-export-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:34.089Z | data-tools-dialog-close | none |
| U3 | export-v1 | [U3-export-v1-1440-light.png](U3-export-v1-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:34.747Z | export-kind | none |
| U3 | export-css | [U3-export-css-1440-light.png](U3-export-css-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:35.417Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-1440-light.png](U3-export-css-profile-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:36.016Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-1440-light.png](U3-export-css-notation-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:36.629Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-1440-light.png](U3-export-rust-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:37.280Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-1440-light.png](U3-export-copied-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:38.184Z | export-copy | none |
| U3 | export-rust-end | [U3-export-rust-end-1440-light.png](U3-export-rust-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:49:38.761Z | export-copy | none |
| U3 | import-review | [U3-import-review-390-light.png](U3-import-review-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:49:40.892Z | import-review | none |
| U3 | import-review-end | [U3-import-review-end-390-light.png](U3-import-review-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:49:41.443Z | import-review | none |
| U3 | export-json | [U3-export-json-390-light.png](U3-export-json-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:49:42.340Z | data-tools-dialog-close | none |
| U3 | import-review-bottom | [U3-import-review-bottom-1440-light.png](U3-import-review-bottom-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:54:17.054Z | import-review | none |
| U3 | import-review-bottom | [U3-import-review-bottom-390-light.png](U3-import-review-bottom-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:54:19.131Z | import-review | none |
| U3 | import-review-mid | [U3-import-review-mid-390-light.png](U3-import-review-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:55:17.421Z | import-review | none |
| U3 | export-rust-mid | [U3-export-rust-mid-1440-light.png](U3-export-rust-mid-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:55:38.803Z | export-kind | none |
| U3 | export-kind-keyboard-focus | [U3-export-kind-keyboard-focus-1440-light.png](U3-export-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:21.000Z | export-kind | none |
| U3 | export-kind-keyboard-open | [U3-export-kind-keyboard-open-1440-light.png](U3-export-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:21.581Z | export-kind-list | none |
| U3 | export-profile-keyboard-focus | [U3-export-profile-keyboard-focus-1440-light.png](U3-export-profile-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:22.948Z | export-profile | none |
| U3 | export-color-format-keyboard-focus | [U3-export-color-format-keyboard-focus-1440-light.png](U3-export-color-format-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:24.231Z | export-color-format | none |
| U3 | export-next-keyboard-focus | [U3-export-next-keyboard-focus-1440-light.png](U3-export-next-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:25.496Z | export-source | none |
| U3 | import-kind-keyboard-focus | [U3-import-kind-keyboard-focus-1440-light.png](U3-import-kind-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:26.819Z | import-format | none |
| U3 | import-kind-keyboard-open | [U3-import-kind-keyboard-open-1440-light.png](U3-import-kind-keyboard-open-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T09:56:27.385Z | import-format-list | none |
| U3 | export-v1 | [U3-export-v1-390-light.png](U3-export-v1-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:49.154Z | export-kind | none |
| U3 | export-css | [U3-export-css-390-light.png](U3-export-css-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:49.785Z | export-kind | none |
| U3 | export-css-profile-open | [U3-export-css-profile-open-390-light.png](U3-export-css-profile-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:50.369Z | export-profile-list | none |
| U3 | export-css-notation-open | [U3-export-css-notation-open-390-light.png](U3-export-css-notation-open-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:50.950Z | export-color-format-list | none |
| U3 | export-rust | [U3-export-rust-390-light.png](U3-export-rust-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:51.570Z | export-kind | none |
| U3 | export-rust-mid | [U3-export-rust-mid-390-light.png](U3-export-rust-mid-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:52.135Z | export-kind | none |
| U3 | export-rust-end | [U3-export-rust-end-390-light.png](U3-export-rust-end-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:52.685Z | export-kind | none |
| U3 | export-copied | [U3-export-copied-390-light.png](U3-export-copied-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:53.559Z | export-copy | none |
| U3 | import-error | [U3-import-error-390-light.png](U3-import-error-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T09:59:54.200Z | import-review | none |
| U3 | export-copied-feedback | [U3-export-copied-feedback-390-light-a2.png](U3-export-copied-feedback-390-light-a2.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:16:05.933Z | export-copy | none |
| U3 | import-review-config-appearance | [U3-import-review-config-appearance-390-light-a2.png](U3-import-review-config-appearance-390-light-a2.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:16:08.852Z | import-review | none |
| U3 | import-review-config-scaling | [U3-import-review-config-scaling-390-light-a2.png](U3-import-review-config-scaling-390-light-a2.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:16:09.399Z | import-review | none |
| U3 | export-v1-actions | [U3-export-v1-actions-390-light-a2.png](U3-export-v1-actions-390-light-a2.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T10:16:10.370Z | export-kind | none |
