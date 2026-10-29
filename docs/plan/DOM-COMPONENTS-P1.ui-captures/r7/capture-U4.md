# UI 验收 R7 · U4 采集清单

- 被测版本：HEAD `b218075a3a2107ab511da932f831f42adfa9390c`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `1fdbbbc39758b56531e996384ad7397645d59a7115ea3fcb6c1b39b326a98600`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `6d1605ccec8b9f78`，catalog `18ca82701d674efd`。
- 目标指纹：`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，与 R6 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r6.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R6，无 managed 索引；本轮 R7，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。

共 23 张。操作记录：[U4-actions.log](U4-actions.log)。

| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U4 | upgrade-pending | [U4-upgrade-pending-1440-light.png](U4-upgrade-pending-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:29.453Z | — | localStorage v1 seeded before reload |
| U4 | upgrade-review | [U4-upgrade-review-1440-light.png](U4-upgrade-review-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:30.072Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-review-end | [U4-upgrade-review-end-1440-light.png](U4-upgrade-review-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:30.638Z | upgrade-review-close | localStorage v1 seeded |
| U4 | upgrade-cancelled | [U4-upgrade-cancelled-1440-light.png](U4-upgrade-cancelled-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:31.370Z | theme-save | localStorage v1 seeded |
| U4 | upgraded | [U4-upgraded-1440-light.png](U4-upgraded-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:33.553Z | — | localStorage v1 seeded |
| U4 | upgrade-pending | [U4-upgrade-pending-390-light.png](U4-upgrade-pending-390-light.png) | http://127.0.0.1:4181/ | 390x844 | 1 | light | viewport | 2026-10-29T02:23:36.686Z | — | localStorage v1 seeded before reload |
| U4 | quota-failure | [U4-quota-failure-1440-light.png](U4-quota-failure-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:46.706Z | theme-save | setItem throws QuotaExceededError |
| U4 | quota-operations | [U4-quota-operations-1440-light.png](U4-quota-operations-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:47.290Z | storage-operations | setItem throws QuotaExceededError |
| U4 | conflict | [U4-conflict-1440-light.png](U4-conflict-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:58.582Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-detail | [U4-conflict-detail-1440-light.png](U4-conflict-detail-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:59.146Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-raw | [U4-conflict-raw-1440-light.png](U4-conflict-raw-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:23:59.743Z | storage-external-raw | second same-origin tab wrote v2 |
| U4 | no-locks-initial | [U4-no-locks-initial-1440-light.png](U4-no-locks-initial-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:24:08.631Z | — | navigator.locks removed before load |
| U4 | no-locks | [U4-no-locks-1440-light.png](U4-no-locks-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:24:09.399Z | theme-save | navigator.locks removed before load |
| U4 | conflict-detail-end | [U4-conflict-detail-end-1440-light.png](U4-conflict-detail-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:53.739Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-after | [U4-conflict-after-1440-light.png](U4-conflict-after-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:54.289Z | color-value | second same-origin tab wrote v2 |
| U4 | conflict-diff | [U4-conflict-diff-1440-light.png](U4-conflict-diff-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:27:54.905Z | storage-conflict-diff | second same-origin tab wrote v2 |
| U4 | upgrade-review-json | [U4-upgrade-review-json-1440-light.png](U4-upgrade-review-json-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:28:50.633Z | upgrade-review | localStorage v1 seeded |
| U4 | upgrade-review-keyboard-focus | [U4-upgrade-review-keyboard-focus-1440-light.png](U4-upgrade-review-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:36:56.567Z | upgrade-review-open | localStorage v1 seeded |
| U4 | upgrade-confirm-keyboard-focus | [U4-upgrade-confirm-keyboard-focus-1440-light.png](U4-upgrade-confirm-keyboard-focus-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:36:57.163Z | upgrade-confirm | localStorage v1 seeded |
| U4 | upgrade-review-keyboard-escape | [U4-upgrade-review-keyboard-escape-1440-light.png](U4-upgrade-review-keyboard-escape-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:36:57.724Z | upgrade-review-open | localStorage v1 seeded |
| U4 | upgrade-json-middle | [U4-upgrade-json-middle-1440-light.png](U4-upgrade-json-middle-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:36:58.397Z | upgrade-review | localStorage v1 seeded |
| U4 | upgrade-json-end | [U4-upgrade-json-end-1440-light.png](U4-upgrade-json-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:36:58.961Z | upgrade-review | localStorage v1 seeded |
| U4 | conflict-raw-end | [U4-conflict-raw-end-1440-light.png](U4-conflict-raw-end-1440-light.png) | http://127.0.0.1:4181/ | 1440x900 | 1 | light | viewport | 2026-10-29T02:37:10.165Z | storage-external-raw | second same-origin tab wrote v2 |
