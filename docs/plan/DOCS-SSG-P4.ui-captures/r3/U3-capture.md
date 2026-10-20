# R3 U3 实拍清单

人工历史续接R3，全量复审；仅DOM静态阅读与搜索。

- HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main；初始tracked diff SHA256 5b56a9c61406a18ce50c9bac1a2cbdb3913bd31f3604907cc0cf4ce9e3b43042，不含未跟踪文件；完整状态见baseline.json。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；相关产品相对C无diff。
- 目标沿用R2：manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4。目标全文hash也相同，未重建基线；延续人工历史算法，不冒称Phoenix协议指纹。本循环没有managed索引。
- 按serve.sh重新构建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath及meta 176bea4ed7e4d96f；启动日志保留原文。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1，1280×900/375×812/320×812；站内按钮设置light/dark，reduce motion，无CPU/网络节流（逐图故障干预除外），等待字体和两次rAF；禁脚本新建禁用JS上下文。
- 已有固定种子100页/语种语料和官方站，所有91张均本轮重拍，不是评审自制界面，未复制旧图；仅延迟/失败真实module或索引请求，逐图注明。没有目标图附件，仅规范模式。

实际操作与页面观测见[U3-operations.md](U3-operations.md)；逐图hash见U3-capture.json。

|单元|状态|原图|URL|视口/主题|UTC|干预|
|---|---|---|---|---|---|---|
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
