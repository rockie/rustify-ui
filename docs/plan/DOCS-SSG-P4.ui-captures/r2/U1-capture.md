# R2 U1 采集清单

- 人工R2，全量复审，仅DOM静态阅读与搜索。HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，main，dirty tracked diff SHA256 4a112900b88a157f42e6bca69d17e897dd6c5e2f805da137c97ca9c095e2662c；完整status与指纹定义见baseline.json。tracked diff不含未跟踪文件。
- 固定实装CLI0.1.27，C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716；被测相关产品路径相对C无diff。
- 目标指纹 manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4，与R1相同，无目标附件，仅规范模式。
- 本轮按serve.sh重建：official/subpath dd5022c567200779；search/root 80da98ab7be5b50d；search/subpath与meta 176bea4ed7e4d96f。真实构建身份日志在本目录。
- Playwright1.63.0 / Chromium153.0.8010.12，DPR1；1280×900/375×812/320×812；站内按钮设定light/dark，reduce motion；无CPU/网络节流（逐图故障干预除外）。等待字体、网络和两次rAF，禁脚本不等待rAF。窄屏仅CSS视口。
- 数据：项目已有固定种子100页/语种语料和官方站。只干预真实请求，未替换DOM、CSS或页面脚本，不是临时模拟界面。

实际操作与观察见[U1-operations.md](U1-operations.md)，图像hash见本单元capture.json。

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
