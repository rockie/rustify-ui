# R2 U3 采集清单

- 人工R2，全量复审，仅DOM静态阅读与搜索。HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main，dirty tracked diff SHA256 4a112900b88a157f42e6bca69d17e897dd6c5e2f805da137c97ca9c095e2662c；完整status与指纹定义见baseline.json。tracked diff不含未跟踪文件。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；被测相关产品路径相对C无diff。
- 目标指纹 manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4，与R1相同，无目标附件，仅规范模式。
- 本轮按serve.sh重建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath与meta 176bea4ed7e4d96f。真实构建身份日志在本目录。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1；1280×900/375×812/320×812；站内按钮设定light/dark，reduce motion；无CPU/网络节流（逐图故障干预除外）。等待字体、网络和两次rAF，禁脚本不等待rAF。窄屏仅CSS视口。
- 数据：项目已有固定种子100页/语种语料和官方站。只干预真实请求，未替换DOM、CSS或页面脚本，不是临时模拟界面。

实际操作与观察见[U3-operations.md](U3-operations.md)，图像hash见本单元capture.json。

|单元|状态|截图|实际URL|视口/DPR|主题|时间UTC|干预|
|---|---|---|---|---|---|---|---|---|
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
