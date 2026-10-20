# R2 U2 采集清单

- 人工R2，全量复审，仅DOM静态阅读与搜索。HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main，dirty tracked diff SHA256 4a112900b88a157f42e6bca69d17e897dd6c5e2f805da137c97ca9c095e2662c；完整status与指纹定义见baseline.json。tracked diff不含未跟踪文件。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；被测相关产品路径相对C无diff。
- 目标指纹 manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4，与R1相同，无目标附件，仅规范模式。
- 本轮按serve.sh重建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath与meta 176bea4ed7e4d96f。真实构建身份日志在本目录。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1；1280×900/375×812/320×812；站内按钮设定light/dark，reduce motion；无CPU/网络节流（逐图故障干预除外）。等待字体、网络和两次rAF，禁脚本不等待rAF。窄屏仅CSS视口。
- 数据：项目已有固定种子100页/语种语料和官方站。只干预真实请求，未替换DOM、CSS或页面脚本，不是临时模拟界面。

实际操作与观察见[U2-operations.md](U2-operations.md)，图像hash见本单元capture.json。

|单元|状态|截图|实际URL|视口/DPR|主题|时间UTC|干预|
|---|---|---|---|---|---|---|---|---|
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
