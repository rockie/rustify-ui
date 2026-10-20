# DOCS-SSG-P4 R3 · 采集清单

人工历史续接R3，全量复审；仅DOM静态阅读与搜索。

- HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main；初始tracked diff SHA256 5b56a9c61406a18ce50c9bac1a2cbdb3913bd31f3604907cc0cf4ce9e3b43042，不含未跟踪文件；完整状态见baseline.json。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；相关产品相对C无diff。
- 目标沿用R2：manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4。目标全文hash也相同，未重建基线；延续人工历史算法，不冒称Phoenix协议指纹。本循环没有managed索引。
- 按serve.sh重新构建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath及meta 176bea4ed7e4d96f；启动日志保留原文。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1，1280×900/375×812/320×812；站内按钮设置light/dark，reduce motion，无CPU/网络节流（逐图故障干预除外），等待字体和两次rAF；禁脚本新建禁用JS上下文。
- 已有固定种子100页/语种语料和官方站，所有91张均本轮重拍，不是评审自制界面，未复制旧图；仅延迟/失败真实module或索引请求，逐图注明。没有目标图附件，仅规范模式。

|单元|状态|原图|URL|视口/主题|UTC|干预|
|---|---|---|---|---|---|---|
|U1|idle|[U1-idle-1280-light.png](U1-idle-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 light|2026-10-16T04:14:12.163Z|none|
|U1|results|[U1-results-1280-light.png](U1-results-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 light|2026-10-16T04:14:12.263Z|none|
|U1|empty|[U1-empty-1280-light.png](U1-empty-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 light|2026-10-16T04:14:12.329Z|none|
|U1|idle|[U1-idle-1280-dark.png](U1-idle-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:13.067Z|none|
|U1|results|[U1-results-1280-dark.png](U1-results-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:13.167Z|none|
|U1|empty|[U1-empty-1280-dark.png](U1-empty-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:13.251Z|none|
|U1|idle|[U1-idle-375-light.png](U1-idle-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:13.996Z|none|
|U1|results|[U1-results-375-light.png](U1-results-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:14.080Z|none|
|U1|empty|[U1-empty-375-light.png](U1-empty-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:14.129Z|none|
|U1|idle|[U1-idle-375-dark.png](U1-idle-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 dark|2026-10-16T04:14:14.848Z|none|
|U1|results|[U1-results-375-dark.png](U1-results-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 dark|2026-10-16T04:14:14.931Z|none|
|U1|empty|[U1-empty-375-dark.png](U1-empty-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 dark|2026-10-16T04:14:14.981Z|none|
|U1|idle|[U1-idle-320-light.png](U1-idle-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 light|2026-10-16T04:14:15.701Z|none|
|U1|results|[U1-results-320-light.png](U1-results-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 light|2026-10-16T04:14:15.784Z|none|
|U1|empty|[U1-empty-320-light.png](U1-empty-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 light|2026-10-16T04:14:15.834Z|none|
|U1|idle|[U1-idle-320-dark.png](U1-idle-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 dark|2026-10-16T04:14:16.536Z|none|
|U1|results|[U1-results-320-dark.png](U1-results-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 dark|2026-10-16T04:14:16.602Z|none|
|U1|empty|[U1-empty-320-dark.png](U1-empty-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812 DPR1 dark|2026-10-16T04:14:16.653Z|none|
|U1|keyboard-open-01|[U1-keyboard-open-01-1280-light.png](U1-keyboard-open-01-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:14:17.379Z|none|
|U1|keyboard-select-02|[U1-keyboard-select-02-1280-light.png](U1-keyboard-select-02-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:14:17.469Z|none|
|U1|keyboard-more-focus-03|[U1-keyboard-more-focus-03-1280-light.png](U1-keyboard-more-focus-03-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:14:17.535Z|none|
|U1|keyboard-more-04|[U1-keyboard-more-04-1280-light.png](U1-keyboard-more-04-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:14:17.619Z|none|
|U1|keyboard-close-05|[U1-keyboard-close-05-1280-light.png](U1-keyboard-close-05-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:14:17.677Z|none|
|U1|keyboard-destination-06|[U1-keyboard-destination-06-1280-light.png](U1-keyboard-destination-06-1280-light.png)|http://127.0.0.1:4205/rustify-ui/#custom-section|1280×900 DPR1 light|2026-10-16T04:14:17.761Z|none|
|U1|zh-translated|[U1-zh-translated-375-dark.png](U1-zh-translated-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 DPR1 dark|2026-10-16T04:14:18.549Z|none|
|U1|zh-fallback|[U1-zh-fallback-375-dark.png](U1-zh-fallback-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812 DPR1 dark|2026-10-16T04:14:18.599Z|none|
|U1|root-results|[U1-root-results-1280-light.png](U1-root-results-1280-light.png)|http://127.0.0.1:4204/page-0001/|1280×900 DPR1 light|2026-10-16T04:14:55.373Z|none|
|U1|meta-results|[U1-meta-results-1280-light.png](U1-meta-results-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0001/|1280×900 DPR1 light|2026-10-16T04:14:56.107Z|none|
|U1|more-20-focus|[U1-more-20-focus-1280-light.png](U1-more-20-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:26.707Z|none|
|U1|more-40-focus|[U1-more-40-focus-1280-light.png](U1-more-40-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:26.790Z|none|
|U1|more-60-focus|[U1-more-60-focus-1280-light.png](U1-more-60-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:26.873Z|none|
|U1|more-80-focus|[U1-more-80-focus-1280-light.png](U1-more-80-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:26.955Z|none|
|U1|more-98-focus|[U1-more-98-focus-1280-light.png](U1-more-98-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:27.039Z|none|
|U1|more-final-keyboard|[U1-more-final-keyboard-1280-light.png](U1-more-final-keyboard-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:16:27.105Z|none|
|U1|subpath-activation-a2|[U1-subpath-activation-a2-1280-light.png](U1-subpath-activation-a2-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0002/|1280×900 DPR1 light|2026-10-16T04:16:28.331Z|none|
|U1|root-activation-a2|[U1-root-activation-a2-1280-light.png](U1-root-activation-a2-1280-light.png)|http://127.0.0.1:4204/page-0002/|1280×900 DPR1 light|2026-10-16T04:16:29.562Z|none|
|U1|meta-activation-a2|[U1-meta-activation-a2-1280-light.png](U1-meta-activation-a2-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0002/|1280×900 DPR1 light|2026-10-16T04:16:30.798Z|none|
|U1|zh-fallback-destination|[U1-zh-fallback-destination-375-dark.png](U1-zh-fallback-destination-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0051/|375×812 DPR1 dark|2026-10-16T04:16:31.572Z|none|
|U2|loading|[U2-loading-1280-dark.png](U2-loading-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:19.296Z|hold first en search JSON response|
|U2|closed-01|[U2-closed-01-1280-dark.png](U2-closed-01-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:19.361Z|hold first en search JSON response|
|U2|reopen-02|[U2-reopen-02-1280-dark.png](U2-reopen-02-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:19.479Z|first JSON request still held; new session uses real next response|
|U2|late-response-03|[U2-late-response-03-1280-dark.png](U2-late-response-03-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:19.545Z|release held first JSON response|
|U2|module-error|[U2-module-error-1280-dark.png](U2-module-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:20.283Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-1280-dark.png](U2-module-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:20.449Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-1280-dark.png](U2-index-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:21.187Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-1280-dark.png](U2-index-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:21.355Z|retry uses real resource; only first request was faulted|
|U2|loading|[U2-loading-375-light.png](U2-loading-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:22.051Z|hold first en search JSON response|
|U2|module-error|[U2-module-error-375-light.png](U2-module-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:22.828Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-375-light.png](U2-module-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:22.978Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-375-light.png](U2-index-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:23.678Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-375-light.png](U2-index-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:23.828Z|retry uses real resource; only first request was faulted|
|U2|supp-open-before-module-01|[U2-supp-open-before-module-01-375-light.png](U2-supp-open-before-module-01-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:35.703Z|hold first search module response|
|U2|supp-loading-edit-02|[U2-supp-loading-edit-02-375-light.png](U2-supp-loading-edit-02-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:35.783Z|hold first search module response|
|U2|supp-loading-result-03|[U2-supp-loading-result-03-375-light.png](U2-supp-loading-result-03-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:35.867Z|release held module; real Worker/index|
|U2|supp-longlist-04|[U2-supp-longlist-04-375-light.png](U2-supp-longlist-04-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:35.934Z|none|
|U2|supp-retry-focus-05|[U2-supp-retry-focus-05-375-light.png](U2-supp-retry-focus-05-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:36.653Z|first module HTTP503|
|U2|supp-keyboard-retry-06|[U2-supp-keyboard-retry-06-375-light.png](U2-supp-keyboard-retry-06-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:36.737Z|none|
|U2|supp-close-focus-07|[U2-supp-close-focus-07-375-light.png](U2-supp-close-focus-07-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:36.787Z|none|
|U2|supp-close-return-08|[U2-supp-close-return-08-375-light.png](U2-supp-close-return-08-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812 DPR1 light|2026-10-16T04:14:36.836Z|none|
|U2|supp-new-result-before-old-09|[U2-supp-new-result-before-old-09-1280-dark.png](U2-supp-new-result-before-old-09-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:37.741Z|first old-session JSON still held|
|U2|supp-old-released-after-new-10|[U2-supp-old-released-after-new-10-1280-dark.png](U2-supp-old-released-after-new-10-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900 DPR1 dark|2026-10-16T04:14:38.040Z|release old-session JSON after new query results|
|U3|en-home|[U3-en-home-1280-light.png](U3-en-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 DPR1 light|2026-10-16T04:15:45.318Z|none|
|U3|zh-home|[U3-zh-home-1280-light.png](U3-zh-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 DPR1 light|2026-10-16T04:15:45.979Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-light.png](U3-zh-quickstart-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 DPR1 light|2026-10-16T04:15:46.657Z|none|
|U3|en-home|[U3-en-home-1280-dark.png](U3-en-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/|1280×900 DPR1 dark|2026-10-16T04:15:47.312Z|none|
|U3|zh-home|[U3-zh-home-1280-dark.png](U3-zh-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900 DPR1 dark|2026-10-16T04:15:47.970Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-dark.png](U3-zh-quickstart-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 DPR1 dark|2026-10-16T04:15:48.648Z|none|
|U3|en-home|[U3-en-home-375-light.png](U3-en-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/|375×812 DPR1 light|2026-10-16T04:15:49.294Z|none|
|U3|zh-home|[U3-zh-home-375-light.png](U3-zh-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 DPR1 light|2026-10-16T04:15:49.984Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-light.png](U3-zh-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 DPR1 light|2026-10-16T04:15:50.647Z|none|
|U3|en-home|[U3-en-home-375-dark.png](U3-en-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/|375×812 DPR1 dark|2026-10-16T04:15:51.293Z|none|
|U3|zh-home|[U3-zh-home-375-dark.png](U3-zh-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812 DPR1 dark|2026-10-16T04:15:51.938Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-dark.png](U3-zh-quickstart-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 DPR1 dark|2026-10-16T04:15:52.603Z|none|
|U3|en-home|[U3-en-home-320-light.png](U3-en-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/|320×812 DPR1 light|2026-10-16T04:15:53.248Z|none|
|U3|zh-home|[U3-zh-home-320-light.png](U3-zh-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 DPR1 light|2026-10-16T04:15:53.894Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-light.png](U3-zh-quickstart-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 DPR1 light|2026-10-16T04:15:54.541Z|none|
|U3|en-home|[U3-en-home-320-dark.png](U3-en-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/|320×812 DPR1 dark|2026-10-16T04:15:55.185Z|none|
|U3|zh-home|[U3-zh-home-320-dark.png](U3-zh-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812 DPR1 dark|2026-10-16T04:15:55.850Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-dark.png](U3-zh-quickstart-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 DPR1 dark|2026-10-16T04:15:56.513Z|none|
|U3|fallback|[U3-fallback-1280-light.png](U3-fallback-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|1280×900 DPR1 light|2026-10-16T04:15:57.175Z|none|
|U3|404-search|[U3-404-search-1280-light.png](U3-404-search-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900 DPR1 light|2026-10-16T04:15:57.877Z|none|
|U3|fallback|[U3-fallback-320-dark.png](U3-fallback-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|320×812 DPR1 dark|2026-10-16T04:15:58.527Z|none|
|U3|404-search|[U3-404-search-320-dark.png](U3-404-search-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812 DPR1 dark|2026-10-16T04:15:59.221Z|none|
|U3|noscript-quickstart|[U3-noscript-quickstart-375-light.png](U3-noscript-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 DPR1 light|2026-10-16T04:15:59.776Z|browser JavaScript disabled|
|U3|noscript-404|[U3-noscript-404-375-light.png](U3-noscript-404-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|375×812 DPR1 light|2026-10-16T04:16:00.336Z|browser JavaScript disabled|
|U3|language-open|[U3-language-open-375-light.png](U3-language-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 DPR1 light|2026-10-16T04:16:01.024Z|none|
|U3|navigation-open|[U3-navigation-open-375-light.png](U3-navigation-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812 DPR1 light|2026-10-16T04:16:01.104Z|none|
|U3|404-closed|[U3-404-closed-1280-light.png](U3-404-closed-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900 DPR1 light|2026-10-16T04:16:32.229Z|none|
|U3|404-destination|[U3-404-destination-1280-light.png](U3-404-destination-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900 DPR1 light|2026-10-16T04:16:32.379Z|none|
|U3|404-closed|[U3-404-closed-320-dark.png](U3-404-closed-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812 DPR1 dark|2026-10-16T04:16:33.021Z|none|
|U3|404-destination|[U3-404-destination-320-dark.png](U3-404-destination-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812 DPR1 dark|2026-10-16T04:16:33.158Z|none|

实际操作与页面观测见各U*-operations.md；逐图hash见各U*-capture.json。原生和事后检查补充见[capture-notes.md](capture-notes.md)与supplement-captures.json，不混入独立评分材料。
