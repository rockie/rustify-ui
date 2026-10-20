# U2 本轮采集清单（补采追加，不覆盖原图）

- HEAD a6c076a7ec669aede7ef2ff019ee79cdbbdb102a；固定CLI C 936149b91f57f6dbbdb43969c8843f940348a11e，产品路径无diff。
- 目标指纹 manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4
- 真实100页生成语料站；Playwright 1.63.0 / Chromium 153.0.8010.12、DPR1、reduce motion、主题按钮设定，字体/网络完成。
- supp 为首位评审指出的证据缺口所补关键帧；首位评分与差距不在本包内。

|截图|状态|URL|视口/DPR|主题|时间|请求干预|
|---|---|---|---|---|---|---|
|[U2-loading-1280-dark.png](U2-loading-1280-dark.png)|loading|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:19.757Z|hold first en search JSON response|
|[U2-closed-01-1280-dark.png](U2-closed-01-1280-dark.png)|closed-01|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:19.837Z|hold first en search JSON response|
|[U2-reopen-02-1280-dark.png](U2-reopen-02-1280-dark.png)|reopen-02|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:19.952Z|first JSON request still held; new session uses real next response|
|[U2-late-response-03-1280-dark.png](U2-late-response-03-1280-dark.png)|late-response-03|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:20.020Z|release held first JSON response|
|[U2-module-error-1280-dark.png](U2-module-error-1280-dark.png)|module-error|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:20.777Z|first search module HTTP 503|
|[U2-module-retry-1280-dark.png](U2-module-retry-1280-dark.png)|module-retry|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:20.976Z|retry uses real resource; only first request was faulted|
|[U2-index-error-1280-dark.png](U2-index-error-1280-dark.png)|index-error|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:21.767Z|first search JSON schema_version=99|
|[U2-index-retry-1280-dark.png](U2-index-retry-1280-dark.png)|index-retry|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:19:21.970Z|retry uses real resource; only first request was faulted|
|[U2-loading-375-light.png](U2-loading-375-light.png)|loading|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:19:22.674Z|hold first en search JSON response|
|[U2-module-error-375-light.png](U2-module-error-375-light.png)|module-error|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:19:23.490Z|first search module HTTP 503|
|[U2-module-retry-375-light.png](U2-module-retry-375-light.png)|module-retry|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:19:23.673Z|retry uses real resource; only first request was faulted|
|[U2-index-error-375-light.png](U2-index-error-375-light.png)|index-error|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:19:24.419Z|first search JSON schema_version=99|
|[U2-index-retry-375-light.png](U2-index-retry-375-light.png)|index-retry|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:19:24.592Z|retry uses real resource; only first request was faulted|
|[U2-supp-open-before-module-01-375-light.png](U2-supp-open-before-module-01-375-light.png)|supp-open-before-module-01|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:49.044Z|hold first search module response|
|[U2-supp-loading-edit-02-375-light.png](U2-supp-loading-edit-02-375-light.png)|supp-loading-edit-02|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:49.109Z|hold first search module response|
|[U2-supp-loading-result-03-375-light.png](U2-supp-loading-result-03-375-light.png)|supp-loading-result-03|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:49.194Z|release held module; real Worker/index|
|[U2-supp-longlist-04-375-light.png](U2-supp-longlist-04-375-light.png)|supp-longlist-04|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:49.259Z|none|
|[U2-supp-retry-focus-05-375-light.png](U2-supp-retry-focus-05-375-light.png)|supp-retry-focus-05|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:50.051Z|first module HTTP503|
|[U2-supp-keyboard-retry-06-375-light.png](U2-supp-keyboard-retry-06-375-light.png)|supp-keyboard-retry-06|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:50.166Z|none|
|[U2-supp-close-focus-07-375-light.png](U2-supp-close-focus-07-375-light.png)|supp-close-focus-07|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:50.233Z|none|
|[U2-supp-close-return-08-375-light.png](U2-supp-close-return-08-375-light.png)|supp-close-return-08|http://127.0.0.1:4205/rustify-ui/page-0001/|375×812/1|light|2026-10-16T02:41:50.298Z|none|
|[U2-supp-new-result-before-old-09-1280-dark.png](U2-supp-new-result-before-old-09-1280-dark.png)|supp-new-result-before-old-09|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:41:51.193Z|first old-session JSON still held|
|[U2-supp-old-released-after-new-10-1280-dark.png](U2-supp-old-released-after-new-10-1280-dark.png)|supp-old-released-after-new-10|http://127.0.0.1:4205/rustify-ui/page-0001/|1280×900/1|dark|2026-10-16T02:41:51.499Z|release old-session JSON after new query results|

实际动作与观察见 U2-operations.md；逐图sha256见 U2-capture.json。
