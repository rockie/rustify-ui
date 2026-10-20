# U3 本轮采集清单

- 被测 HEAD：a6c076a7ec669aede7ef2ff019ee79cdbbdb102a；固定CLI候选：936149b91f57f6dbbdb43969c8843f940348a11e，当前相关产品路径与候选没有 diff。
- 构建：search/subpath e92daa8a0efb7328，official/subpath ae467c9c97900d12；root/meta 构建身份另见总 capture.md。
- 工具 Playwright 1.63.0 / Chromium 153.0.8010.12；DPR 1、reduce motion；真实生成产品，不是静态模拟页面。
- 目标指纹：manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4；人工协议，无 Phoenix managed 索引。
- 主题通过现有主题按钮设定；禁脚本时沿系统 light。截图仅可视视口，正文下方需实际滚动的内容不能从这些图片推断。
- 原生手机、原生缩放和VoiceOver无本轮采集，不由本组图像代替。

|截图|状态|真实URL|视口/DPR|主题|时间|请求干预|
|---|---|---|---|---|---|---|
|[U3-en-home-1280-light.png](U3-en-home-1280-light.png)|en-home|http://127.0.0.1:4199/rustify-ui/|1280×900/1|light|2026-10-16T02:20:02.898Z|none|
|[U3-zh-home-1280-light.png](U3-zh-home-1280-light.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|1280×900/1|light|2026-10-16T02:20:03.591Z|none|
|[U3-zh-quickstart-1280-light.png](U3-zh-quickstart-1280-light.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900/1|light|2026-10-16T02:20:04.303Z|none|
|[U3-en-home-1280-dark.png](U3-en-home-1280-dark.png)|en-home|http://127.0.0.1:4199/rustify-ui/|1280×900/1|dark|2026-10-16T02:20:04.972Z|none|
|[U3-zh-home-1280-dark.png](U3-zh-home-1280-dark.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|1280×900/1|dark|2026-10-16T02:20:05.659Z|none|
|[U3-zh-quickstart-1280-dark.png](U3-zh-quickstart-1280-dark.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|1280×900/1|dark|2026-10-16T02:20:06.346Z|none|
|[U3-en-home-375-light.png](U3-en-home-375-light.png)|en-home|http://127.0.0.1:4199/rustify-ui/|375×812/1|light|2026-10-16T02:20:07.015Z|none|
|[U3-zh-home-375-light.png](U3-zh-home-375-light.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|375×812/1|light|2026-10-16T02:20:07.690Z|none|
|[U3-zh-quickstart-375-light.png](U3-zh-quickstart-375-light.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T02:20:08.371Z|none|
|[U3-en-home-375-dark.png](U3-en-home-375-dark.png)|en-home|http://127.0.0.1:4199/rustify-ui/|375×812/1|dark|2026-10-16T02:20:09.030Z|none|
|[U3-zh-home-375-dark.png](U3-zh-home-375-dark.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|375×812/1|dark|2026-10-16T02:20:09.686Z|none|
|[U3-zh-quickstart-375-dark.png](U3-zh-quickstart-375-dark.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|dark|2026-10-16T02:20:10.364Z|none|
|[U3-en-home-320-light.png](U3-en-home-320-light.png)|en-home|http://127.0.0.1:4199/rustify-ui/|320×812/1|light|2026-10-16T02:20:11.021Z|none|
|[U3-zh-home-320-light.png](U3-zh-home-320-light.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|320×812/1|light|2026-10-16T02:20:11.697Z|none|
|[U3-zh-quickstart-320-light.png](U3-zh-quickstart-320-light.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812/1|light|2026-10-16T02:20:12.377Z|none|
|[U3-en-home-320-dark.png](U3-en-home-320-dark.png)|en-home|http://127.0.0.1:4199/rustify-ui/|320×812/1|dark|2026-10-16T02:20:13.027Z|none|
|[U3-zh-home-320-dark.png](U3-zh-home-320-dark.png)|zh-home|http://127.0.0.1:4199/rustify-ui/zh/|320×812/1|dark|2026-10-16T02:20:13.669Z|none|
|[U3-zh-quickstart-320-dark.png](U3-zh-quickstart-320-dark.png)|zh-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|320×812/1|dark|2026-10-16T02:20:14.337Z|none|
|[U3-fallback-1280-light.png](U3-fallback-1280-light.png)|fallback|http://127.0.0.1:4199/rustify-ui/zh/architecture/|1280×900/1|light|2026-10-16T02:20:15.054Z|none|
|[U3-404-search-1280-light.png](U3-404-search-1280-light.png)|404-search|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|1280×900/1|light|2026-10-16T02:20:15.782Z|none|
|[U3-fallback-320-dark.png](U3-fallback-320-dark.png)|fallback|http://127.0.0.1:4199/rustify-ui/zh/architecture/|320×812/1|dark|2026-10-16T02:20:16.453Z|none|
|[U3-404-search-320-dark.png](U3-404-search-320-dark.png)|404-search|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|320×812/1|dark|2026-10-16T02:20:17.148Z|none|
|[U3-noscript-quickstart-375-light.png](U3-noscript-quickstart-375-light.png)|noscript-quickstart|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T02:23:07.220Z|browser JavaScript disabled|
|[U3-noscript-404-375-light.png](U3-noscript-404-375-light.png)|noscript-404|http://127.0.0.1:4199/rustify-ui/zh/__ui_review_missing__/deep/|375×812/1|light|2026-10-16T02:23:07.798Z|browser JavaScript disabled|
|[U3-language-open-375-light.png](U3-language-open-375-light.png)|language-open|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T02:23:08.549Z|none|
|[U3-navigation-open-375-light.png](U3-navigation-open-375-light.png)|navigation-open|http://127.0.0.1:4199/rustify-ui/zh/quickstart/|375×812/1|light|2026-10-16T02:23:08.646Z|none|

实际动作与观察见 U3-operations.md；逐图sha256见 U3-capture.json。
