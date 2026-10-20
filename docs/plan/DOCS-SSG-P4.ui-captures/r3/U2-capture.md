# R3 U2 实拍清单

人工历史续接R3，全量复审；仅DOM静态阅读与搜索。

- HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main；初始tracked diff SHA256 5b56a9c61406a18ce50c9bac1a2cbdb3913bd31f3604907cc0cf4ce9e3b43042，不含未跟踪文件；完整状态见baseline.json。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；相关产品相对C无diff。
- 目标沿用R2：manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4。目标全文hash也相同，未重建基线；延续人工历史算法，不冒称Phoenix协议指纹。本循环没有managed索引。
- 按serve.sh重新构建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath及meta 176bea4ed7e4d96f；启动日志保留原文。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1，1280×900/375×812/320×812；站内按钮设置light/dark，reduce motion，无CPU/网络节流（逐图故障干预除外），等待字体和两次rAF；禁脚本新建禁用JS上下文。
- 已有固定种子100页/语种语料和官方站，所有91张均本轮重拍，不是评审自制界面，未复制旧图；仅延迟/失败真实module或索引请求，逐图注明。没有目标图附件，仅规范模式。

实际操作与页面观测见[U2-operations.md](U2-operations.md)；逐图hash见U2-capture.json。

|单元|状态|原图|URL|视口/主题|UTC|干预|
|---|---|---|---|---|---|---|
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
