# DOCS-SSG-P4 UI验收R2 · 采集清单

- 人工R2，全量复审，仅DOM静态阅读与搜索。HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main，dirty tracked diff SHA256 4a112900b88a157f42e6bca69d17e897dd6c5e2f805da137c97ca9c095e2662c；完整status与指纹定义见baseline.json。tracked diff不含未跟踪文件。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；被测相关产品路径相对C无diff。
- 目标指纹 manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4，与R1相同，无目标附件，仅规范模式。
- 本轮按serve.sh重建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath与meta 176bea4ed7e4d96f。真实构建身份日志在本目录。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1；1280×900/375×812/320×812；站内按钮设定light/dark，reduce motion；无CPU/网络节流（逐图故障干预除外）。等待字体、网络和两次rAF，禁脚本不等待rAF。窄屏仅CSS视口。
- 数据：项目已有固定种子100页/语种语料和官方站。只干预真实请求，未替换DOM、CSS或页面脚本，不是临时模拟界面。

|单元|状态|截图|实际URL|视口/DPR|主题|时间UTC|干预|
|---|---|---|---|---|---|---|---|---|
|U1|idle|[U1-idle-1280-light.png](U1-idle-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|light|2026-10-16T03:38:37.823Z|none|
|U1|results|[U1-results-1280-light.png](U1-results-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|light|2026-10-16T03:38:37.922Z|none|
|U1|empty|[U1-empty-1280-light.png](U1-empty-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|light|2026-10-16T03:38:37.989Z|none|
|U1|idle|[U1-idle-1280-dark.png](U1-idle-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:38.718Z|none|
|U1|results|[U1-results-1280-dark.png](U1-results-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:38.801Z|none|
|U1|empty|[U1-empty-1280-dark.png](U1-empty-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:38.868Z|none|
|U1|idle|[U1-idle-375-light.png](U1-idle-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:39.568Z|none|
|U1|results|[U1-results-375-light.png](U1-results-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:39.651Z|none|
|U1|empty|[U1-empty-375-light.png](U1-empty-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:39.701Z|none|
|U1|idle|[U1-idle-375-dark.png](U1-idle-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|dark|2026-10-16T03:38:40.405Z|none|
|U1|results|[U1-results-375-dark.png](U1-results-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|dark|2026-10-16T03:38:40.471Z|none|
|U1|empty|[U1-empty-375-dark.png](U1-empty-375-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|dark|2026-10-16T03:38:40.521Z|none|
|U1|idle|[U1-idle-320-light.png](U1-idle-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|light|2026-10-16T03:38:41.217Z|none|
|U1|results|[U1-results-320-light.png](U1-results-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|light|2026-10-16T03:38:41.300Z|none|
|U1|empty|[U1-empty-320-light.png](U1-empty-320-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|light|2026-10-16T03:38:41.350Z|none|
|U1|idle|[U1-idle-320-dark.png](U1-idle-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|dark|2026-10-16T03:38:42.045Z|none|
|U1|results|[U1-results-320-dark.png](U1-results-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|dark|2026-10-16T03:38:42.112Z|none|
|U1|empty|[U1-empty-320-dark.png](U1-empty-320-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|320×812/1|dark|2026-10-16T03:38:42.162Z|none|
|U1|keyboard-open-01|[U1-keyboard-open-01-1280-light.png](U1-keyboard-open-01-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:38:42.901Z|none|
|U1|keyboard-select-02|[U1-keyboard-select-02-1280-light.png](U1-keyboard-select-02-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:38:42.991Z|none|
|U1|keyboard-more-focus-03|[U1-keyboard-more-focus-03-1280-light.png](U1-keyboard-more-focus-03-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:38:43.057Z|none|
|U1|keyboard-more-04|[U1-keyboard-more-04-1280-light.png](U1-keyboard-more-04-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:38:43.140Z|none|
|U1|keyboard-close-05|[U1-keyboard-close-05-1280-light.png](U1-keyboard-close-05-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:38:43.199Z|none|
|U1|keyboard-destination-06|[U1-keyboard-destination-06-1280-light.png](U1-keyboard-destination-06-1280-light.png)|http://127.0.0.1:4205/rustify-ui/#custom-section|1280×900/1|light|2026-10-16T03:38:43.283Z|none|
|U1|zh-translated|[U1-zh-translated-375-dark.png](U1-zh-translated-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812/1|dark|2026-10-16T03:38:44.070Z|none|
|U1|zh-fallback|[U1-zh-fallback-375-dark.png](U1-zh-fallback-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0001/|375×812/1|dark|2026-10-16T03:38:44.120Z|none|
|U1|root-results|[U1-root-results-1280-light.png](U1-root-results-1280-light.png)|http://127.0.0.1:4204/page-0001/|1280×900/1|light|2026-10-16T03:38:58.295Z|none|
|U1|meta-results|[U1-meta-results-1280-light.png](U1-meta-results-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0001/|1280×900/1|light|2026-10-16T03:38:59.028Z|none|
|U1|more-20-focus|[U1-more-20-focus-1280-light.png](U1-more-20-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.368Z|none|
|U1|more-40-focus|[U1-more-40-focus-1280-light.png](U1-more-40-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.463Z|none|
|U1|more-60-focus|[U1-more-60-focus-1280-light.png](U1-more-60-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.547Z|none|
|U1|more-80-focus|[U1-more-80-focus-1280-light.png](U1-more-80-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.630Z|none|
|U1|more-98-focus|[U1-more-98-focus-1280-light.png](U1-more-98-focus-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.710Z|none|
|U1|more-final-keyboard|[U1-more-final-keyboard-1280-light.png](U1-more-final-keyboard-1280-light.png)|http://127.0.0.1:4205/rustify-ui/|1280×900/1|light|2026-10-16T03:41:25.777Z|none|
|U1|subpath-activation-a2|[U1-subpath-activation-a2-1280-light.png](U1-subpath-activation-a2-1280-light.png)|http://127.0.0.1:4205/rustify-ui/page-0002/|1280×900/1|light|2026-10-16T03:42:00.361Z|none|
|U1|root-activation-a2|[U1-root-activation-a2-1280-light.png](U1-root-activation-a2-1280-light.png)|http://127.0.0.1:4204/page-0002/|1280×900/1|light|2026-10-16T03:42:01.628Z|none|
|U1|meta-activation-a2|[U1-meta-activation-a2-1280-light.png](U1-meta-activation-a2-1280-light.png)|http://127.0.0.1:4206/rustify-ui/page-0002/|1280×900/1|light|2026-10-16T03:42:02.951Z|none|
|U1|zh-fallback-destination|[U1-zh-fallback-destination-375-dark.png](U1-zh-fallback-destination-375-dark.png)|http://127.0.0.1:4205/rustify-ui/zh/page-0051/|375×812/1|dark|2026-10-16T03:42:03.710Z|none|
|U2|loading|[U2-loading-1280-dark.png](U2-loading-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:44.824Z|hold first en search JSON response|
|U2|closed-01|[U2-closed-01-1280-dark.png](U2-closed-01-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:44.887Z|hold first en search JSON response|
|U2|reopen-02|[U2-reopen-02-1280-dark.png](U2-reopen-02-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:45.022Z|first JSON request still held; new session uses real next response|
|U2|late-response-03|[U2-late-response-03-1280-dark.png](U2-late-response-03-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:45.088Z|release held first JSON response|
|U2|module-error|[U2-module-error-1280-dark.png](U2-module-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:45.818Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-1280-dark.png](U2-module-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:45.985Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-1280-dark.png](U2-index-error-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:46.714Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-1280-dark.png](U2-index-retry-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:38:46.880Z|retry uses real resource; only first request was faulted|
|U2|loading|[U2-loading-375-light.png](U2-loading-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:47.561Z|hold first en search JSON response|
|U2|module-error|[U2-module-error-375-light.png](U2-module-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:48.337Z|first search module HTTP 503|
|U2|module-retry|[U2-module-retry-375-light.png](U2-module-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:48.470Z|retry uses real resource; only first request was faulted|
|U2|index-error|[U2-index-error-375-light.png](U2-index-error-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:49.195Z|first search JSON schema_version=99|
|U2|index-retry|[U2-index-retry-375-light.png](U2-index-retry-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:38:49.345Z|retry uses real resource; only first request was faulted|
|U2|supp-open-before-module-01|[U2-supp-open-before-module-01-375-light.png](U2-supp-open-before-module-01-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:09.702Z|hold first search module response|
|U2|supp-loading-edit-02|[U2-supp-loading-edit-02-375-light.png](U2-supp-loading-edit-02-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:09.767Z|hold first search module response|
|U2|supp-loading-result-03|[U2-supp-loading-result-03-375-light.png](U2-supp-loading-result-03-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:09.851Z|release held module; real Worker/index|
|U2|supp-longlist-04|[U2-supp-longlist-04-375-light.png](U2-supp-longlist-04-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:09.918Z|none|
|U2|supp-retry-focus-05|[U2-supp-retry-focus-05-375-light.png](U2-supp-retry-focus-05-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:10.640Z|first module HTTP503|
|U2|supp-keyboard-retry-06|[U2-supp-keyboard-retry-06-375-light.png](U2-supp-keyboard-retry-06-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:10.723Z|none|
|U2|supp-close-focus-07|[U2-supp-close-focus-07-375-light.png](U2-supp-close-focus-07-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:10.773Z|none|
|U2|supp-close-return-08|[U2-supp-close-return-08-375-light.png](U2-supp-close-return-08-375-light.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T03:39:10.822Z|none|
|U2|supp-new-result-before-old-09|[U2-supp-new-result-before-old-09-1280-dark.png](U2-supp-new-result-before-old-09-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:40:23.001Z|first old-session JSON still held|
|U2|supp-old-released-after-new-10|[U2-supp-old-released-after-new-10-1280-dark.png](U2-supp-old-released-after-new-10-1280-dark.png)|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T03:40:23.329Z|release old-session JSON after new query results|
|U3|en-home|[U3-en-home-1280-light.png](U3-en-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/|1280×900/1|light|2026-10-16T03:40:41.529Z|none|
|U3|zh-home|[U3-zh-home-1280-light.png](U3-zh-home-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900/1|light|2026-10-16T03:40:42.223Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-light.png](U3-zh-quickstart-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900/1|light|2026-10-16T03:40:42.911Z|none|
|U3|en-home|[U3-en-home-1280-dark.png](U3-en-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/|1280×900/1|dark|2026-10-16T03:40:43.580Z|none|
|U3|zh-home|[U3-zh-home-1280-dark.png](U3-zh-home-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|1280×900/1|dark|2026-10-16T03:40:44.249Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-1280-dark.png](U3-zh-quickstart-1280-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900/1|dark|2026-10-16T03:40:44.941Z|none|
|U3|en-home|[U3-en-home-375-light.png](U3-en-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/|375×812/1|light|2026-10-16T03:40:45.606Z|none|
|U3|zh-home|[U3-zh-home-375-light.png](U3-zh-home-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812/1|light|2026-10-16T03:40:46.258Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-light.png](U3-zh-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T03:40:46.928Z|none|
|U3|en-home|[U3-en-home-375-dark.png](U3-en-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/|375×812/1|dark|2026-10-16T03:40:47.581Z|none|
|U3|zh-home|[U3-zh-home-375-dark.png](U3-zh-home-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|375×812/1|dark|2026-10-16T03:40:48.232Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-375-dark.png](U3-zh-quickstart-375-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|dark|2026-10-16T03:40:48.904Z|none|
|U3|en-home|[U3-en-home-320-light.png](U3-en-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/|320×812/1|light|2026-10-16T03:40:49.552Z|none|
|U3|zh-home|[U3-zh-home-320-light.png](U3-zh-home-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812/1|light|2026-10-16T03:40:50.201Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-light.png](U3-zh-quickstart-320-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812/1|light|2026-10-16T03:40:50.869Z|none|
|U3|en-home|[U3-en-home-320-dark.png](U3-en-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/|320×812/1|dark|2026-10-16T03:40:51.516Z|none|
|U3|zh-home|[U3-zh-home-320-dark.png](U3-zh-home-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/|320×812/1|dark|2026-10-16T03:40:52.163Z|none|
|U3|zh-quickstart|[U3-zh-quickstart-320-dark.png](U3-zh-quickstart-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812/1|dark|2026-10-16T03:40:52.841Z|none|
|U3|fallback|[U3-fallback-1280-light.png](U3-fallback-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|1280×900/1|light|2026-10-16T03:40:53.504Z|none|
|U3|404-search|[U3-404-search-1280-light.png](U3-404-search-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900/1|light|2026-10-16T03:40:54.209Z|none|
|U3|fallback|[U3-fallback-320-dark.png](U3-fallback-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/architecture/|320×812/1|dark|2026-10-16T03:40:54.855Z|none|
|U3|404-search|[U3-404-search-320-dark.png](U3-404-search-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812/1|dark|2026-10-16T03:40:55.549Z|none|
|U3|noscript-quickstart|[U3-noscript-quickstart-375-light.png](U3-noscript-quickstart-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T03:40:56.104Z|browser JavaScript disabled|
|U3|noscript-404|[U3-noscript-404-375-light.png](U3-noscript-404-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|375×812/1|light|2026-10-16T03:40:56.659Z|browser JavaScript disabled|
|U3|language-open|[U3-language-open-375-light.png](U3-language-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T03:40:57.354Z|none|
|U3|navigation-open|[U3-navigation-open-375-light.png](U3-navigation-open-375-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T03:40:57.434Z|none|
|U3|404-closed|[U3-404-closed-1280-light.png](U3-404-closed-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900/1|light|2026-10-16T03:42:04.369Z|none|
|U3|404-destination|[U3-404-destination-1280-light.png](U3-404-destination-1280-light.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900/1|light|2026-10-16T03:42:04.526Z|none|
|U3|404-closed|[U3-404-closed-320-dark.png](U3-404-closed-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812/1|dark|2026-10-16T03:42:05.198Z|none|
|U3|404-destination|[U3-404-destination-320-dark.png](U3-404-destination-320-dark.png)|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812/1|dark|2026-10-16T03:42:05.342Z|none|

实际操作见各U*-operations.md。browser-raw.jsonl只在独立评审后分析。缺证与原生补充见[capture-notes.md](capture-notes.md)。

## 本轮原生与事后机器补充

逐图来源、文件时间与hash见[supplement-captures.json](supplement-captures.json)。文件时间仅辅助定位；真实性由本轮工具调用、原生动作及机器运行记录支持。

|图像|来源|实际格式与像素|
|---|---|---|
|[U4-native-chrome-zoom200-read.jpg](U4-native-chrome-zoom200-read.jpg)|CUA原生Chrome，200%阅读|JPEG 3024×1752|
|[U4-native-chrome-zoom200-search.jpg](U4-native-chrome-zoom200-search.jpg)|CUA原生Chrome，200%搜索|JPEG 3024×1752|
|[M1-show-more-after-1280-light.png](M1-show-more-after-1280-light.png)|独立评分后运行期真实键盘复核|PNG 1280×900/DPR1|

主agent已在工具中打开全部91张登记页面图、两张原生图与一张机器补充，共94张有效原图；另有一张采集器导航竞争中的未登记图明确排除。没有合成、拼接、裁图或修图。
