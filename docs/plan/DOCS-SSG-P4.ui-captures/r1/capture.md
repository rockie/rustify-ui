# DOCS-SSG-P4 UI 验收 R1 · 采集清单

- 模式：人工评审；首次规范基线。主agent只采集/核查/裁定，四位全新上下文agent评分。
- 范围：DOM静态阅读站与原生搜索；U1/U2/U3为本地可运行单元；U4原生设备与辅助技术缺证。
- 被测 HEAD：`a6c076a7ec669aede7ef2ff019ee79cdbbdb102a`，main，dirty。初始tracked diff SHA-256：`945d85fd809289533db0ecd2384fd123c2778be790b23ca1c232da6fea6a1493`；原始status与算法见 baseline.json。此指纹仅为tracked diff，不冒充未跟踪文件的完整快照。被测UI产品路径与可取回固定C₅无diff；原有未提交项为计划/SDLC/历史报告等元数据。
- 固定 CLI C₅：`936149b91f57f6dbbdb43969c8843f940348a11e`，框架R₅：`58b7a0d9eb913b7656120693905d66d70ab208d8`，CLI0.1.26；panic unwind。
- 构建：official/subpath `ae467c9c97900d12`；search/root `f63a1f2403c3a37f`；search/subpath、meta `e92daa8a0efb7328`。本轮重新构建，没有拿旧截图冒充新采集。
- 目标指纹：`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`；详见baseline.json，不声称与Phoenix engine指纹算法兼容。
- 主采集：Playwright1.63.0/Chromium153.0.8010.12，DPR1，1280×900/375×812/320×812；reduce motion，显式主题按钮设定；等待字体、网络和两次rAF。禁脚本不等待不运行的rAF。
- 数据：计划既有100页同构搜索语料与真实官方站，未改变界面实现。故障仅通过真实请求延迟、首次模块503、首次索引schema99注入；每图有干预记录。示例站不是为评审模拟的HTML。
- 用户确认：本轮用户回复“按现有规范继续（推荐）”；单元与规范要点未变。
- 原图：82张；U1 28、U2 23、U3 26、原生补充4、机器复现1。主agent在工具中实际打开全部82张，核对目标画面与尺寸；没有合成、拼接或修图。DPR1图像像素与登记视口一一相符；补充原生图保留实际像素，未凭图像反推未知CSS视口。

## 逐图登记

| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 拍摄时间（UTC） | 干预 |
| --- | --- | --- | --- | --- | --- | --- | --- |
|U1|idle|[U1-idle-1280-light.png](U1-idle-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T02:19:12.492Z|none|
|U1|results|[U1-results-1280-light.png](U1-results-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T02:19:12.573Z|none|
|U1|empty|[U1-empty-1280-light.png](U1-empty-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T02:19:12.641Z|none|
|U1|idle|[U1-idle-1280-dark.png](U1-idle-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:13.386Z|none|
|U1|results|[U1-results-1280-dark.png](U1-results-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:13.477Z|none|
|U1|empty|[U1-empty-1280-dark.png](U1-empty-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:13.559Z|none|
|U1|idle|[U1-idle-375-light.png](U1-idle-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:14.311Z|none|
|U1|results|[U1-results-375-light.png](U1-results-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:14.389Z|none|
|U1|empty|[U1-empty-375-light.png](U1-empty-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:14.436Z|none|
|U1|idle|[U1-idle-375-dark.png](U1-idle-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T02:19:15.171Z|none|
|U1|results|[U1-results-375-dark.png](U1-results-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T02:19:15.254Z|none|
|U1|empty|[U1-empty-375-dark.png](U1-empty-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|dark|2026-10-16T02:19:15.320Z|none|
|U1|idle|[U1-idle-320-light.png](U1-idle-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T02:19:16.057Z|none|
|U1|results|[U1-results-320-light.png](U1-results-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T02:19:16.138Z|none|
|U1|empty|[U1-empty-320-light.png](U1-empty-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|light|2026-10-16T02:19:16.206Z|none|
|U1|idle|[U1-idle-320-dark.png](U1-idle-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T02:19:16.946Z|none|
|U1|results|[U1-results-320-dark.png](U1-results-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T02:19:17.029Z|none|
|U1|empty|[U1-empty-320-dark.png](U1-empty-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 / 1|dark|2026-10-16T02:19:17.095Z|none|
|U1|keyboard-open-01|[U1-keyboard-open-01-1280-light.png](U1-keyboard-open-01-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T02:19:17.809Z|none|
|U1|keyboard-select-02|[U1-keyboard-select-02-1280-light.png](U1-keyboard-select-02-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T02:19:17.898Z|none|
|U1|keyboard-more-focus-03|[U1-keyboard-more-focus-03-1280-light.png](U1-keyboard-more-focus-03-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T02:19:17.963Z|none|
|U1|keyboard-more-04|[U1-keyboard-more-04-1280-light.png](U1-keyboard-more-04-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T02:19:18.047Z|none|
|U1|keyboard-close-05|[U1-keyboard-close-05-1280-light.png](U1-keyboard-close-05-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 / 1|light|2026-10-16T02:19:18.102Z|none|
|U1|keyboard-destination-06|[U1-keyboard-destination-06-1280-light.png](U1-keyboard-destination-06-1280-light.png)|http://127.0.0.1:4205/rustify-ui/#custom-section|1280×900 / 1|light|2026-10-16T02:19:18.185Z|none|
|U1|zh-translated|[U1-zh-translated-375-dark.png](U1-zh-translated-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 / 1|dark|2026-10-16T02:19:18.970Z|none|
|U1|zh-fallback|[U1-zh-fallback-375-dark.png](U1-zh-fallback-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 / 1|dark|2026-10-16T02:19:19.037Z|none|
|U1|root-results|[U1-root-results-1280-light.png](U1-root-results-1280-light.png)|http://127.0.0.1:4204/page-0001/|1280×900 / 1|light|2026-10-16T02:23:40.900Z|none|
|U1|meta-results|[U1-meta-results-1280-light.png](U1-meta-results-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0001/|1280×900 / 1|light|2026-10-16T02:23:41.651Z|none|
|U2|loading|[U2-loading-1280-dark.png](U2-loading-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:19.757Z|hold first en search JSON response|
|U2|closed-01|[U2-closed-01-1280-dark.png](U2-closed-01-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:19.837Z|hold first en search JSON response|
|U2|reopen-02|[U2-reopen-02-1280-dark.png](U2-reopen-02-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:19.952Z|first JSON request still held; new session uses real next response|
|U2|late-response-03|[U2-late-response-03-1280-dark.png](U2-late-response-03-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:20.020Z|release held first JSON response|
|U2|module-error|[U2-module-error-1280-dark.png](U2-module-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:20.777Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-1280-dark.png](U2-module-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:20.976Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-1280-dark.png](U2-index-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:21.767Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-1280-dark.png](U2-index-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:19:21.970Z|retry uses real resource; only first request was faulted|
|U2|loading|[U2-loading-375-light.png](U2-loading-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:22.674Z|hold first en search JSON response|
|U2|module-error|[U2-module-error-375-light.png](U2-module-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:23.490Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-375-light.png](U2-module-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:23.673Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-375-light.png](U2-index-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:24.419Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-375-light.png](U2-index-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:19:24.592Z|retry uses real resource; only first request was faulted|
|U2|supp-open-before-module-01|[U2-supp-open-before-module-01-375-light.png](U2-supp-open-before-module-01-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:49.044Z|hold first search module response|
|U2|supp-loading-edit-02|[U2-supp-loading-edit-02-375-light.png](U2-supp-loading-edit-02-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:49.109Z|hold first search module response|
|U2|supp-loading-result-03|[U2-supp-loading-result-03-375-light.png](U2-supp-loading-result-03-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:49.194Z|release held module; real Worker/index|
|U2|supp-longlist-04|[U2-supp-longlist-04-375-light.png](U2-supp-longlist-04-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:49.259Z|none|
|U2|supp-retry-focus-05|[U2-supp-retry-focus-05-375-light.png](U2-supp-retry-focus-05-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:50.051Z|first module HTTP503|
|U2|supp-keyboard-retry-06|[U2-supp-keyboard-retry-06-375-light.png](U2-supp-keyboard-retry-06-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:50.166Z|none|
|U2|supp-close-focus-07|[U2-supp-close-focus-07-375-light.png](U2-supp-close-focus-07-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:50.233Z|none|
|U2|supp-close-return-08|[U2-supp-close-return-08-375-light.png](U2-supp-close-return-08-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 / 1|light|2026-10-16T02:41:50.298Z|none|
|U2|supp-new-result-before-old-09|[U2-supp-new-result-before-old-09-1280-dark.png](U2-supp-new-result-before-old-09-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:41:51.193Z|first old-session JSON still held|
|U2|supp-old-released-after-new-10|[U2-supp-old-released-after-new-10-1280-dark.png](U2-supp-old-released-after-new-10-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 / 1|dark|2026-10-16T02:41:51.499Z|release old-session JSON after new query results|
|U3|en-home|[U3-en-home-1280-light.png](U3-en-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 / 1|light|2026-10-16T02:20:02.898Z|none|
|U3|zh-home|[U3-zh-home-1280-light.png](U3-zh-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 / 1|light|2026-10-16T02:20:03.591Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-light.png](U3-zh-quickstart-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 / 1|light|2026-10-16T02:20:04.303Z|none|
|U3|en-home|[U3-en-home-1280-dark.png](U3-en-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 / 1|dark|2026-10-16T02:20:04.972Z|none|
|U3|zh-home|[U3-zh-home-1280-dark.png](U3-zh-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 / 1|dark|2026-10-16T02:20:05.659Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-dark.png](U3-zh-quickstart-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 / 1|dark|2026-10-16T02:20:06.346Z|none|
|U3|en-home|[U3-en-home-375-light.png](U3-en-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/|375×812 / 1|light|2026-10-16T02:20:07.015Z|none|
|U3|zh-home|[U3-zh-home-375-light.png](U3-zh-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 / 1|light|2026-10-16T02:20:07.690Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-light.png](U3-zh-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T02:20:08.371Z|none|
|U3|en-home|[U3-en-home-375-dark.png](U3-en-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/|375×812 / 1|dark|2026-10-16T02:20:09.030Z|none|
|U3|zh-home|[U3-zh-home-375-dark.png](U3-zh-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 / 1|dark|2026-10-16T02:20:09.686Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-dark.png](U3-zh-quickstart-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|dark|2026-10-16T02:20:10.364Z|none|
|U3|en-home|[U3-en-home-320-light.png](U3-en-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/|320×812 / 1|light|2026-10-16T02:20:11.021Z|none|
|U3|zh-home|[U3-zh-home-320-light.png](U3-zh-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 / 1|light|2026-10-16T02:20:11.697Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-light.png](U3-zh-quickstart-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 / 1|light|2026-10-16T02:20:12.377Z|none|
|U3|en-home|[U3-en-home-320-dark.png](U3-en-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/|320×812 / 1|dark|2026-10-16T02:20:13.027Z|none|
|U3|zh-home|[U3-zh-home-320-dark.png](U3-zh-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 / 1|dark|2026-10-16T02:20:13.669Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-dark.png](U3-zh-quickstart-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 / 1|dark|2026-10-16T02:20:14.337Z|none|
|U3|fallback|[U3-fallback-1280-light.png](U3-fallback-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|1280×900 / 1|light|2026-10-16T02:20:15.054Z|none|
|U3|404-search|[U3-404-search-1280-light.png](U3-404-search-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900 / 1|light|2026-10-16T02:20:15.782Z|none|
|U3|fallback|[U3-fallback-320-dark.png](U3-fallback-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|320×812 / 1|dark|2026-10-16T02:20:16.453Z|none|
|U3|404-search|[U3-404-search-320-dark.png](U3-404-search-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812 / 1|dark|2026-10-16T02:20:17.148Z|none|
|U3|noscript-quickstart|[U3-noscript-quickstart-375-light.png](U3-noscript-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T02:23:07.220Z|browser JavaScript disabled|
|U3|noscript-404|[U3-noscript-404-375-light.png](U3-noscript-404-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|375×812 / 1|light|2026-10-16T02:23:07.798Z|browser JavaScript disabled|
|U3|language-open|[U3-language-open-375-light.png](U3-language-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T02:23:08.549Z|none|
|U3|navigation-open|[U3-navigation-open-375-light.png](U3-navigation-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 / 1|light|2026-10-16T02:23:08.646Z|none|

逐图动作/观察在U1/U2/U3-operations.md，单元采集条件与图像sha256在对应capture.json；原始console/network为browser-raw.jsonl，未交给评审。

## 原生与事后补充

| 图像 | 出处 | 实际图像像素 | 时间出处 |
| --- | --- | --- | --- |
|[M1-show-more-after-1280-light.png](M1-show-more-after-1280-light.png)|Playwright runtime keyboard reproduction|1280×900|2026-10-16T02:54:12.735657+00:00（文件时间；原生操作的工具回显为真实性证据）|
|[U4-native-safari-home.png](U4-native-safari-home.png)|CUA native Safari|4718592×4292935744|2026-10-16T02:45:25.379975+00:00（文件时间；原生操作的工具回显为真实性证据）|
|[U4-native-safari-search.png](U4-native-safari-search.png)|CUA native Safari|4718592×4292935744|2026-10-16T02:50:49.138763+00:00（文件时间；原生操作的工具回显为真实性证据）|
|[U4-native-supplement-home-1272-dpr2.png](U4-native-supplement-home-1272-dpr2.png)|Kimi native Chrome|2544×1546|2026-10-16T02:23:40.786268+00:00（文件时间；原生操作的工具回显为真实性证据）|
|[U4-native-supplement-search-1272-dpr2.png](U4-native-supplement-search-1272-dpr2.png)|Kimi native Chrome|2544×1546|2026-10-16T02:41:48.861714+00:00（文件时间；原生操作的工具回显为真实性证据）|

- Chrome扩展：Kimi会话docs-ssg-p4-ui-review，tabId1319606361，中文官方首页及查询ThemeScope的真实搜索结果；实际evaluate返回CSS1272×773、DPR2、UA Chrome154.0.0.0。主题system，实拍为light。原生截图有扩展的边缘提示，不参与U1–U3评分；没有擅自去掉提示或裁图。
- Safari：CUA com.apple.Safari，新建本任务页签；版本27.0.1（读取本地App元数据）。本轮原生截图可用，Cmd+K输入ThemeScope得到3个结果，见U4-native-safari-operations.md。CSS视口/DPR与原生缩放值未取得，不按像素反推。原生冒烟不填完整稳定矩阵或VoiceOver。
- 主agent机器复现：M1图来自runtime-check-runner.cjs，URL为http://127.0.0.1:4205/rustify-ui/，1280×900/DPR1/light；实际步序及时间为runtime-checks.json中UI-01-before/after/second-enter。

## 没采到与未验证

| 单元/条件 | 缺口 | 获取方式 |
| --- | --- | --- |
| U4 最新稳定桌面矩阵 | Chrome/Safari只作冒烟，未确认最新稳定及完整任务；Firefox.app不存在 | 维护者提供已安装的可操作稳定浏览器或远程原生会话后完整走查，不擅自安装 |
| U4 两实体手机与软键盘 | 无本轮设备会话；CSS375px不代替真机 | 按计划已有资源依赖提供iOS Safari、Android Chrome实体设备/远程真机会话 |
| U4 原生200% | 没有原生缩放操作与确认读数 | 在原生浏览器设置200%并记录真实读数/操作/截图；不能使用CSS或DPR模拟代填 |
| U4 VoiceOver | 没有实际朗读或转录；AX/aria不能替代 | 提供VoiceOver+Safari可操作会话与音频/转录或原生操作证据 |
| U1/U3未拍区域 | 中间宽度两栏、下半部代码/宽表横滑、完整键盘环路、实际IME与动态瞬时状态等 | 逐项见judge-U1-a.md、judge-U3-a.md；不得据未拍状态推断通过或开产品差距 |

U2 A指出的加载中编辑、早期焦点、键盘恢复、长列表滚动与严格晚响应顺序已追加10张supp；新评审B看完整23张，未覆盖状态仍保留未知。U1/U3事后机器实测了中文缺译/404导航，但该材料未交给其原评分者，不替他们改判。

## 环境、来源与启动

- 服务使用项目tests/docs-ssg/serve.sh和固定实装CLI：official/subpath4199、search/root4204、search/subpath4205、search/meta4206。只有meta使用项目原生static-host.mjs无响应头路径。
- 启动命令前置环境：PATH=/opt/homebrew/opt/python@3.12/libexec/bin:$PATH；RUSTIFY_DOCS_BIN=/Users/rockie/Documents/gh-rockie/rustify-ui/target/docs-ssg/p4/install-936149b9/bin/rustify-docs。没有新容器或基础设施。
- 启动失败attempt1为系统Python3.9缺tomllib；改用已有Python3.12.13后成功；日志按attempt保存，没有产品修复。
- 源材料hash见source-hashes.json；CLI身份、初始dirty情况见baseline.json。采集器与运行期检查器已保存在同目录，包含本机绝对路径，跨机器需按当前仓库位置调整后重拍。
- 为验证当前CLI另执行mbx build --locked --profile docs-cli -p rustify-docs-cli，成功；主采集仍明确绑定固定实装C₅。未因报告工作重跑全量Rust/lint/性能。
- Phoenix实际probe失败，详见tool-probes.md；未建managed索引，未自动修复engine。impeccable context按真实目标加载一次；本轮没有设计/产品编辑。
- 所有本轮临时服务结束前停止；实际PID/命令和检查结果另存server-lifecycle.md。原有其他端口与服务未动。
