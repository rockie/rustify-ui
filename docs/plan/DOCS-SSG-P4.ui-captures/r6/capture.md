# DOCS-SSG-P4 R6 · 采集清单

人工历史续接 R6，全量轮；仅 DOM 静态阅读与搜索。目标包：`docs/plan/DOCS-SSG-P4.ui-target.md`。

## 本轮身份与共同条件

- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`（main）；开始时 tracked diff SHA256 `a3feecba9c72699241545463fdfe93f89752df78cef3a4fc2b78b2b78b77ecb0`（仅 `git diff --binary`，不含未跟踪文件），完整状态见 [baseline.json](baseline.json)。与 R5 的 diff 指纹差异只来自台账/计划登记文本，产品路径经核对干净。
- 固定实装 CLI 0.1.27，C `1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a` / R `740610db164b3f396b0fcc551f5a5ef8f15da716`；`target/docs-ssg/p4/install-1bbc54b0` 因 mbx 缓存目录曾被清空，本轮按 M6 同一命令与进程级 remap 原样重装，`--build-info` 为 0.1.27 / source 完整 C / panic unwind，install --list 对应，`productDiffFromCandidate` 为空。安装日志 `target/docs-ssg/p4/r6-install.log`。
- 目标文件 SHA256 `ed08081e68367b02d7ad695a041909e2c8ec26d78abe377376d1e1b0b7a6ad02`，规范节指纹 `manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`，与 R1–R5 相同：期望未变，不是重建基线。
- 实际 build_id：官方站 subpath `dd5022c567200779`（4199）；搜索站 subpath/meta `176bea4ed7e4d96f`（4205/4206）；搜索站 root `80da98ab7be5b50d`（4204）。与 R2–R5 逐位一致（确定性产物）。dist index.html SHA256：官方 `804eb73c471d402568e29887d053753bb32182b5b584afde3b2091e2d224dec4`，搜索 `6896d185511b4f58768b8c2db5aa54a88d92ed7f011c60bca3ac1b17f8c01d7a`。
- 采集工具 Playwright 1.63.0 / Chromium 153.0.8010.12（性能合同固定引擎，不冒称最新稳定），DPR 1，`prefers-reduced-motion: reduce`，等待字体与 networkidle，截图前双 rAF；主题通过页面真实主题按钮设定。窄屏（375/320）是 CSS 视口，不是实体手机。
- 全部截图本轮实拍于上述真实服务；无任何合成、修图、旧图复用。逐图清单见下方三节（机器可读原件 [U1-capture.json](U1-capture.json)、[U2-capture.json](U2-capture.json)、[U3-capture.json](U3-capture.json)），每图记录实际 URL、视口、主题、拍摄时间与干预；交互关键帧的实际动作与实测见 [U1-operations.md](U1-operations.md)、[U2-operations.md](U2-operations.md)、[U3-operations.md](U3-operations.md)。采集日志：capture-search.log / capture-u2-extra.log / capture-official.log / capture-hosts.log / capture-r2-extra.log；控制台与请求原文 browser-raw.jsonl。
- 干预只有数据层：U2 module error 仅首次模块请求 503，index error 仅返回 schema_version 99，Retry 恢复真实请求；U1 loading 仅延迟索引响应 3s，晚响应延迟 3.5s 后输入新查询。界面代码未改。
- 服务器为评审临时启动，停止方式见 [server-lifecycle.json](server-lifecycle.md)（写于本轮结束时）；构建与启动日志 startup-*.log。

## U4 原生设备与辅助技术：本轮未采到

- 用户说明本机桌面正被其他 agent 用于其他项目的 UI 验收（前台窗口、焦点与键盘输入都被占用）；本评审只做不抢前台的无头采集，原生窗口采集整体推迟，避免互相污染两边的证据。
- 探测事实（未做任何设置变更）：macOS 27.0.1 arm64；本机 Chrome 155.0.8059.82（较 M7 记录的 .41 已更新；是否为计划口径的「最新稳定」未在本轮核对，M7 当时记录官方稳定为 .84/.85）；Safari 27.0.1 已安装；/Applications 无 Firefox；iPhone 镜像进程未在运行；无 Android 设备；VoiceOver 会话不可用且不擅自开启系统设置。
- 曾尝试开起一个 Chrome 专用窗口指向官方中文首页，采集帧被其他应用的窗口遮挡，文件 `U4-chrome-01-zh-home-INVALID-obstructed.jpg` 保留为无效尝试记录，按规则**当作没采到**，不作证据；该窗口仍开着（Chrome 窗口 id 1319606462），留待用户处置或下轮继续使用。
- 因此 U4 全部格子（桌面稳定 Chrome/Firefox/Safari 原生矩阵重拍、iOS Safari、Android Chrome、原生 200%、VoiceOver 朗读）本轮一律 missing，不用引擎、视口或 DOM 属性代替。R3/R4/R5 的历史局部原生证据保留在原轮目录，不搬入本轮充数。

## 机器检查（评审结束后由主 agent 分析，不给评审）

- runtime-checks.json：57 项检查、8 条流程、0 pageerror；全部视口 pageOverflow 为 0；axe（wcag2a/2aa/21a/21aa/22aa）0 violations；37 条 incomplete 与 R3 逐项相同（color-contrast 因元素重叠无法判定、error 态 listbox 的 aria-required-children、aria-keyshortcuts 的 aria-valid-attr-value），存 [axe-incomplete.json](axe-incomplete.json)。
- keyboard-checks.json：跳过链接、语言菜单 Esc、对话框 Tab/Shift+Tab、Esc 归焦、重开保留查询、主页主次操作、语言链接、目录锚点、代码横向滚动、禁脚本导航、错误态编辑后键盘 Retry，全部实测通过，关键帧 M2-*.png。
- browser-raw.jsonl：无 CSP 违规；11 条 http-error/requestfailed 全部为计划内干预（模块 503 ×6、深 404 路径 ×5）。

## U1 逐图清单（38 张）

|单元|状态|文件|实际URL|视口/DPR|主题|拍摄时间|干预|
|---|---|---|---|---|---|---|---|
|U1|idle|[U1-idle-1280-light.png](U1-idle-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T08:59:18.385Z|none|
|U1|results|[U1-results-1280-light.png](U1-results-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T08:59:18.482Z|none|
|U1|empty|[U1-empty-1280-light.png](U1-empty-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T08:59:18.549Z|none|
|U1|idle|[U1-idle-1280-dark.png](U1-idle-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:19.285Z|none|
|U1|results|[U1-results-1280-dark.png](U1-results-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:19.369Z|none|
|U1|empty|[U1-empty-1280-dark.png](U1-empty-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:19.435Z|none|
|U1|idle|[U1-idle-375-light.png](U1-idle-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:20.136Z|none|
|U1|results|[U1-results-375-light.png](U1-results-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:20.203Z|none|
|U1|empty|[U1-empty-375-light.png](U1-empty-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:20.254Z|none|
|U1|idle|[U1-idle-375-dark.png](U1-idle-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T08:59:20.959Z|none|
|U1|results|[U1-results-375-dark.png](U1-results-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T08:59:21.042Z|none|
|U1|empty|[U1-empty-375-dark.png](U1-empty-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T08:59:21.091Z|none|
|U1|idle|[U1-idle-320-light.png](U1-idle-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T08:59:21.795Z|none|
|U1|results|[U1-results-320-light.png](U1-results-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T08:59:21.878Z|none|
|U1|empty|[U1-empty-320-light.png](U1-empty-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T08:59:21.928Z|none|
|U1|idle|[U1-idle-320-dark.png](U1-idle-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T08:59:22.628Z|none|
|U1|results|[U1-results-320-dark.png](U1-results-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T08:59:22.694Z|none|
|U1|empty|[U1-empty-320-dark.png](U1-empty-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T08:59:22.744Z|none|
|U1|keyboard-open-01|[U1-keyboard-open-01-1280-light.png](U1-keyboard-open-01-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T08:59:23.486Z|none|
|U1|keyboard-select-02|[U1-keyboard-select-02-1280-light.png](U1-keyboard-select-02-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T08:59:23.576Z|none|
|U1|keyboard-more-focus-03|[U1-keyboard-more-focus-03-1280-light.png](U1-keyboard-more-focus-03-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T08:59:23.642Z|none|
|U1|keyboard-more-04|[U1-keyboard-more-04-1280-light.png](U1-keyboard-more-04-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T08:59:23.725Z|none|
|U1|keyboard-close-05|[U1-keyboard-close-05-1280-light.png](U1-keyboard-close-05-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T08:59:23.784Z|none|
|U1|keyboard-destination-06|[U1-keyboard-destination-06-1280-light.png](U1-keyboard-destination-06-1280-light.png)|http://127.0.0.1:4205/rustify-ui/#custom-section|1280×900 / 1|light|2026-10-16T08:59:23.867Z|none|
|U1|zh-translated|[U1-zh-translated-375-dark.png](U1-zh-translated-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 / 1|dark|2026-10-16T08:59:24.654Z|none|
|U1|zh-fallback|[U1-zh-fallback-375-dark.png](U1-zh-fallback-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 / 1|dark|2026-10-16T08:59:24.703Z|none|
|U1|root-results|[U1-root-results-1280-light.png](U1-root-results-1280-light.png)|http://127.0.0.1:4204/page-0001/|1280×900 / 1|light|2026-10-16T09:01:21.370Z|none|
|U1|meta-results|[U1-meta-results-1280-light.png](U1-meta-results-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T09:01:22.157Z|none|
|U1|more-20-focus|[U1-more-20-focus-1280-light.png](U1-more-20-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.494Z|none|
|U1|more-40-focus|[U1-more-40-focus-1280-light.png](U1-more-40-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.575Z|none|
|U1|more-60-focus|[U1-more-60-focus-1280-light.png](U1-more-60-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.659Z|none|
|U1|more-80-focus|[U1-more-80-focus-1280-light.png](U1-more-80-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.742Z|none|
|U1|more-98-focus|[U1-more-98-focus-1280-light.png](U1-more-98-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.825Z|none|
|U1|more-final-keyboard|[U1-more-final-keyboard-1280-light.png](U1-more-final-keyboard-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T09:01:23.891Z|none|
|U1|subpath-activation-a2|[U1-subpath-activation-a2-1280-light.png](U1-subpath-activation-a2-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0002/|1280×900 / 1|light|2026-10-16T09:01:25.134Z|none|
|U1|root-activation-a2|[U1-root-activation-a2-1280-light.png](U1-root-activation-a2-1280-light.png)|http://127.0.0.1:4204/page-0002/|1280×900 / 1|light|2026-10-16T09:01:26.424Z|none|
|U1|meta-activation-a2|[U1-meta-activation-a2-1280-light.png](U1-meta-activation-a2-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0002/|1280×900 / 1|light|2026-10-16T09:01:27.717Z|none|
|U1|zh-fallback-destination|[U1-zh-fallback-destination-375-dark.png](U1-zh-fallback-destination-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0051/|375×812 / 1|dark|2026-10-16T09:01:28.516Z|none|

## U2 逐图清单（23 张）

|单元|状态|文件|实际URL|视口/DPR|主题|拍摄时间|干预|
|---|---|---|---|---|---|---|---|
|U2|loading|[U2-loading-1280-dark.png](U2-loading-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:25.406Z|hold first en search JSON response|
|U2|closed-01|[U2-closed-01-1280-dark.png](U2-closed-01-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:25.470Z|hold first en search JSON response|
|U2|reopen-02|[U2-reopen-02-1280-dark.png](U2-reopen-02-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:25.588Z|first JSON request still held; new session uses real next response|
|U2|late-response-03|[U2-late-response-03-1280-dark.png](U2-late-response-03-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:25.654Z|release held first JSON response|
|U2|module-error|[U2-module-error-1280-dark.png](U2-module-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:26.389Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-1280-dark.png](U2-module-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:26.538Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-1280-dark.png](U2-index-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:27.273Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-1280-dark.png](U2-index-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:27.439Z|retry uses real resource; only first request was faulted|
|U2|loading|[U2-loading-375-light.png](U2-loading-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:28.127Z|hold first en search JSON response|
|U2|module-error|[U2-module-error-375-light.png](U2-module-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:28.962Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-375-light.png](U2-module-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:29.095Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-375-light.png](U2-index-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:29.812Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-375-light.png](U2-index-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:29.945Z|retry uses real resource; only first request was faulted|
|U2|supp-open-before-module-01|[U2-supp-open-before-module-01-375-light.png](U2-supp-open-before-module-01-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:52.318Z|hold first search module response|
|U2|supp-loading-edit-02|[U2-supp-loading-edit-02-375-light.png](U2-supp-loading-edit-02-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:52.383Z|hold first search module response|
|U2|supp-loading-result-03|[U2-supp-loading-result-03-375-light.png](U2-supp-loading-result-03-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:52.467Z|release held module; real Worker/index|
|U2|supp-longlist-04|[U2-supp-longlist-04-375-light.png](U2-supp-longlist-04-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:52.534Z|none|
|U2|supp-retry-focus-05|[U2-supp-retry-focus-05-375-light.png](U2-supp-retry-focus-05-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:53.253Z|first module HTTP503|
|U2|supp-keyboard-retry-06|[U2-supp-keyboard-retry-06-375-light.png](U2-supp-keyboard-retry-06-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:53.337Z|none|
|U2|supp-close-focus-07|[U2-supp-close-focus-07-375-light.png](U2-supp-close-focus-07-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:53.386Z|none|
|U2|supp-close-return-08|[U2-supp-close-return-08-375-light.png](U2-supp-close-return-08-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T08:59:53.435Z|none|
|U2|supp-new-result-before-old-09|[U2-supp-new-result-before-old-09-1280-dark.png](U2-supp-new-result-before-old-09-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:54.319Z|first old-session JSON still held|
|U2|supp-old-released-after-new-10|[U2-supp-old-released-after-new-10-1280-dark.png](U2-supp-old-released-after-new-10-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T08:59:54.618Z|release old-session JSON after new query results|

## U3 逐图清单（30 张）

|单元|状态|文件|实际URL|视口/DPR|主题|拍摄时间|干预|
|---|---|---|---|---|---|---|---|
|U3|en-home|[U3-en-home-1280-light.png](U3-en-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 / 1|light|2026-10-16T09:00:06.385Z|none|
|U3|zh-home|[U3-zh-home-1280-light.png](U3-zh-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 / 1|light|2026-10-16T09:00:07.055Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-light.png](U3-zh-quickstart-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 / 1|light|2026-10-16T09:00:07.741Z|none|
|U3|en-home|[U3-en-home-1280-dark.png](U3-en-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 / 1|dark|2026-10-16T09:00:08.404Z|none|
|U3|zh-home|[U3-zh-home-1280-dark.png](U3-zh-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 / 1|dark|2026-10-16T09:00:09.102Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-dark.png](U3-zh-quickstart-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 / 1|dark|2026-10-16T09:00:09.806Z|none|
|U3|en-home|[U3-en-home-375-light.png](U3-en-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/|375×812 / 1|light|2026-10-16T09:00:10.457Z|none|
|U3|zh-home|[U3-zh-home-375-light.png](U3-zh-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 / 1|light|2026-10-16T09:00:11.107Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-light.png](U3-zh-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T09:00:11.776Z|none|
|U3|en-home|[U3-en-home-375-dark.png](U3-en-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/|375×812 / 1|dark|2026-10-16T09:00:12.427Z|none|
|U3|zh-home|[U3-zh-home-375-dark.png](U3-zh-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 / 1|dark|2026-10-16T09:00:13.077Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-dark.png](U3-zh-quickstart-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|dark|2026-10-16T09:00:13.747Z|none|
|U3|en-home|[U3-en-home-320-light.png](U3-en-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/|320×812 / 1|light|2026-10-16T09:00:14.394Z|none|
|U3|zh-home|[U3-zh-home-320-light.png](U3-zh-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 / 1|light|2026-10-16T09:00:15.050Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-light.png](U3-zh-quickstart-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 / 1|light|2026-10-16T09:00:15.723Z|none|
|U3|en-home|[U3-en-home-320-dark.png](U3-en-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/|320×812 / 1|dark|2026-10-16T09:00:16.394Z|none|
|U3|zh-home|[U3-zh-home-320-dark.png](U3-zh-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 / 1|dark|2026-10-16T09:00:17.050Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-dark.png](U3-zh-quickstart-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 / 1|dark|2026-10-16T09:00:17.736Z|none|
|U3|fallback|[U3-fallback-1280-light.png](U3-fallback-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|1280×900 / 1|light|2026-10-16T09:00:18.424Z|none|
|U3|404-search|[U3-404-search-1280-light.png](U3-404-search-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900 / 1|light|2026-10-16T09:00:19.136Z|none|
|U3|fallback|[U3-fallback-320-dark.png](U3-fallback-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|320×812 / 1|dark|2026-10-16T09:00:19.802Z|none|
|U3|404-search|[U3-404-search-320-dark.png](U3-404-search-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812 / 1|dark|2026-10-16T09:00:20.505Z|none|
|U3|noscript-quickstart|[U3-noscript-quickstart-375-light.png](U3-noscript-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T09:00:21.063Z|browser JavaScript disabled|
|U3|noscript-404|[U3-noscript-404-375-light.png](U3-noscript-404-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|375×812 / 1|light|2026-10-16T09:00:21.629Z|browser JavaScript disabled|
|U3|language-open|[U3-language-open-375-light.png](U3-language-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T09:00:22.333Z|none|
|U3|navigation-open|[U3-navigation-open-375-light.png](U3-navigation-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T09:00:22.413Z|none|
|U3|404-closed|[U3-404-closed-1280-light.png](U3-404-closed-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900 / 1|light|2026-10-16T09:01:29.173Z|none|
|U3|404-destination|[U3-404-destination-1280-light.png](U3-404-destination-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 / 1|light|2026-10-16T09:01:29.376Z|none|
|U3|404-closed|[U3-404-closed-320-dark.png](U3-404-closed-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812 / 1|dark|2026-10-16T09:01:30.041Z|none|
|U3|404-destination|[U3-404-destination-320-dark.png](U3-404-destination-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 / 1|dark|2026-10-16T09:01:30.181Z|none|
