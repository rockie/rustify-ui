# R4 · U4 原生补充采集

- 本轮限定U4原生证据，U1-U3未复核；目标沿用既有目标包，没有更改期望。
- HEAD 93c4d101055f16c4493c15abde7b43141f1f8a47，固定CLI0.1.27 C1bbc54b0/R740610db；当前相关产品路径与C无diff。复用R3同字节已构建真实产物，HTTP入口4199官方、4205真实100页搜索语料。build_id与校验见baseline.json。
- 真实macOS Safari，通过CUA本轮实拍，非Playwright/WebKit引擎模拟。原生About显示27.0.1 (22625.1.29.11.28)。窗口从原尺寸扩大后恢复；100%与明确原生200%分别记录；主题最后恢复跟随系统。没有字体/样式/DOM替换。
- 初次module失败只暂扣真实CLI产物隔离副本中的search module，实际HTTP404；恢复完全相同字节后同页点击Retry。不是模拟页面。
- 初始9帧AX为增量，之后保存全量AX；逐帧原图与AX共同取证。尝试时仍loading、工具输入未传入、未完成键盘重试及一次索引变化造成的再次Increment均如实保留，最终演示实际归零。
- iPhone实体镜像已观察到连接，尚无文档站实机图；当前显示iPhone使用中，用户正在处理入口。连接状态图不是产品验收图。不把原生Safari桌面当作手机。
- U4仍缺完整Chrome/Firefox/Android/VoiceOver、部分Safari任务与手机证据，单元不能据局部补零或补全分数。

|原图/原生AX|UTC|实际动作与结果边界|
|---|---|---|
|[U4-Safari-01-about.jpg](U4-Safari-01-about.jpg) · [AX](U4-Safari-01-about.ax.txt)|2026-10-16T04:34:14.478Z|Safari菜单→关于Safari，直接读取运行版本。|
|[U4-Safari-02-en-home.jpg](U4-Safari-02-en-home.jpg) · [AX](U4-Safari-02-en-home.ax.txt)|2026-10-16T04:34:43.046Z|新页键入官方根路径并Cmd+0；本次实际页面为/zh/中文首页，未把入口意图当作已到英文页。|
|[U4-Safari-03-english-home.jpg](U4-Safari-03-english-home.jpg) · [AX](U4-Safari-03-english-home.ax.txt)|2026-10-16T04:40:34.162Z|从中文首页语言菜单点击English普通链接，实际切换到英文首页。|
|[U4-Safari-04-maximized-home.jpg](U4-Safari-04-maximized-home.jpg) · [AX](U4-Safari-04-maximized-home.ax.txt)|2026-10-16T04:40:49.123Z|使用Safari窗口缩放动作扩大原生窗口，实拍首页；不是CSS视口模拟。|
|[U4-Safari-05-search-idle.jpg](U4-Safari-05-search-idle.jpg) · [AX](U4-Safari-05-search-idle.ax.txt)|2026-10-16T04:41:00.965Z|在英文首页实际按Cmd+K，搜索对话框打开并输入获焦。|
|[U4-Safari-06-search-results.jpg](U4-Safari-06-search-results.jpg) · [AX](U4-Safari-06-search-results.ax.txt)|2026-10-16T04:41:10.705Z|键入ThemeScope后本帧仍显示Preparing search；记录为真实loading attempt，不记结果成功。|
|[U4-Safari-07-search-selected.jpg](U4-Safari-07-search-selected.jpg) · [AX](U4-Safari-07-search-selected.ax.txt)|2026-10-16T04:41:11.111Z|loading期间按Down，本帧仍为Preparing search，尚无可选结果。|
|[U4-Safari-08-search-destination.jpg](U4-Safari-08-search-destination.jpg) · [AX](U4-Safari-08-search-destination.ax.txt)|2026-10-16T04:41:11.858Z|按Return后本帧出现3 results且对话框仍开；不记导航成功。|
|[U4-Safari-09-ready-selected.jpg](U4-Safari-09-ready-selected.jpg) · [AX](U4-Safari-09-ready-selected.ax.txt)|2026-10-16T04:41:23.880Z|在3 results已可见后实际按Down，记录当前选择。|
|[U4-Safari-10-ready-destination.jpg](U4-Safari-10-ready-destination.jpg) · [AX](U4-Safari-10-ready-destination.ax.txt)|2026-10-16T04:41:32.584Z|已选Architecture后实际按Enter，记录本次真实导航URL与对话框状态。|
|[U4-Safari-01-quickstart.jpg](U4-Safari-01-quickstart.jpg) · [AX](U4-Safari-01-quickstart.ax.txt)|2026-10-16T04:42:18.698Z|点击侧栏Quick start，实测普通链接阅读页。|
|[U4-Safari-02-copy-feedback.jpg](U4-Safari-02-copy-feedback.jpg) · [AX](U4-Safari-02-copy-feedback.ax.txt)|2026-10-16T04:42:25.274Z|点击第一块代码Copy，记录实际按钮与状态反馈。|
|[U4-Safari-03-zoom-attempt.jpg](U4-Safari-03-zoom-attempt.jpg) · [AX](U4-Safari-03-zoom-attempt.ax.txt)|2026-10-16T04:42:44.520Z|原生Cmd+0后连续五次Cmd+=，先记录实际布局，缩放数值另从原生控件确认。|
|[U4-Safari-04-zoom-native-value.jpg](U4-Safari-04-zoom-native-value.jpg) · [AX](U4-Safari-04-zoom-native-value.ax.txt)|2026-10-16T04:42:55.762Z|打开当前网站的Safari原生设置，仅查看页面缩放数值，不改权限。|
|[U4-Safari-05-zoom-200-setting.jpg](U4-Safari-05-zoom-200-setting.jpg) · [AX](U4-Safari-05-zoom-200-setting.ax.txt)|2026-10-16T04:43:23.099Z|在Safari网站设置明确选择200%，保存原生百分比证据。|
|[U4-Safari-06-zoom-200-read.jpg](U4-Safari-06-zoom-200-read.jpg) · [AX](U4-Safari-06-zoom-200-read.ax.txt)|2026-10-16T04:43:31.871Z|关闭原生设置后实拍200%阅读页，观察文字重排与页面边界。|
|[U4-Safari-07-zoom-200-search-open.jpg](U4-Safari-07-zoom-200-search-open.jpg) · [AX](U4-Safari-07-zoom-200-search-open.ax.txt)|2026-10-16T04:43:32.839Z|200%原生缩放下Cmd+K打开搜索，检查输入和关闭控件可达。|
|[U4-Safari-08-zoom-200-query-attempt.jpg](U4-Safari-08-zoom-200-query-attempt.jpg) · [AX](U4-Safari-08-zoom-200-query-attempt.ax.txt)|2026-10-16T04:43:48.396Z|200%下输入不存在查询，记录即时状态，不提前假定已完成。|
|[U4-Safari-09-zoom-200-empty.jpg](U4-Safari-09-zoom-200-empty.jpg) · [AX](U4-Safari-09-zoom-200-empty.ax.txt)|2026-10-16T04:44:09.772Z|输入值不变，实际按Left检查输入仍可编辑；此时记录完成后的搜索状态。|
|[U4-Safari-10-fault-entry.jpg](U4-Safari-10-fault-entry.jpg) · [AX](U4-Safari-10-fault-entry.ax.txt)|2026-10-16T04:45:02.537Z|新建Safari页签进入真实100页搜索语料站；仅该产物隔离副本暂扣module以获得真实404。|
|[U4-Safari-11-module-error-attempt.jpg](U4-Safari-11-module-error-attempt.jpg) · [AX](U4-Safari-11-module-error-attempt.ax.txt)|2026-10-16T04:45:09.190Z|在首次模块404时Cmd+K打开并输入CustomOnlyToken，记录实际即时错误/加载状态。|
|[U4-Safari-12-retry-focus.jpg](U4-Safari-12-retry-focus.jpg) · [AX](U4-Safari-12-retry-focus.ax.txt)|2026-10-16T04:45:21.175Z|从输入实际按Tab，本轮原生焦点到HTML内容，未记录为Retry获焦。|
|[U4-Safari-13-retry-attempt.jpg](U4-Safari-13-retry-attempt.jpg) · [AX](U4-Safari-13-retry-attempt.ax.txt)|2026-10-16T04:45:21.643Z|焦点停在HTML内容时按Enter，仍显示error；未记录为重试成功。|
|[U4-Safari-14-option-tab-focus.jpg](U4-Safari-14-option-tab-focus.jpg) · [AX](U4-Safari-14-option-tab-focus.ax.txt)|2026-10-16T04:45:37.670Z|按Safari原生Option+Tab检查可达控件；不修改系统键盘偏好。|
|[U4-Safari-15-retry-click.jpg](U4-Safari-15-retry-click.jpg) · [AX](U4-Safari-15-retry-click.ax.txt)|2026-10-16T04:45:49.948Z|点击实际可见Retry，同页重新加载恢复后的原module，记录状态与输入。|
|[U4-Safari-16-zh-button-demo.jpg](U4-Safari-16-zh-button-demo.jpg) · [AX](U4-Safari-16-zh-button-demo.ax.txt)|2026-10-16T04:46:26.644Z|普通导航到官方中文界面的Button分类，记录真实DOM演示初始状态。|
|[U4-Safari-17-demo-increment.jpg](U4-Safari-17-demo-increment.jpg) · [AX](U4-Safari-17-demo-increment.ax.txt)|2026-10-16T04:48:36.508Z|实际点击Increment，记录真实演示输出，不以源码推断。|
|[U4-Safari-18-demo-reset.jpg](U4-Safari-18-demo-reset.jpg) · [AX](U4-Safari-18-demo-reset.ax.txt)|2026-10-16T04:48:37.088Z|前一动作后AX索引变化，本次实际再次激活Increment；保留该采集attempt，不记Reset成功。|
|[U4-Safari-19-demo-reset-confirmed.jpg](U4-Safari-19-demo-reset-confirmed.jpg) · [AX](U4-Safari-19-demo-reset-confirmed.ax.txt)|2026-10-16T04:48:55.331Z|按本次最新AX中的Reset按钮执行，核对计数归零。|
|[U4-Safari-20-zh-dark.jpg](U4-Safari-20-zh-dark.jpg) · [AX](U4-Safari-20-zh-dark.ax.txt)|2026-10-16T04:49:37.233Z|在实际中文界面点击深色，记录缺译正文与主题。|
|[U4-Safari-21-zh-deep-404.jpg](U4-Safari-21-zh-deep-404.jpg) · [AX](U4-Safari-21-zh-deep-404.ax.txt)|2026-10-16T04:49:40.598Z|在已构建官方站普通访问中文深404，记录当前语言与搜索入口。|
|[U4-Safari-22-zh-404-query.jpg](U4-Safari-22-zh-404-query.jpg) · [AX](U4-Safari-22-zh-404-query.ax.txt)|2026-10-16T04:49:45.555Z|Cmd+K后尝试typeText中文，实际输入仍为空、显示idle；保留工具输入attempt，不记为已查询。|
|[U4-Safari-23-zh-404-pasted-query.jpg](U4-Safari-23-zh-404-pasted-query.jpg) · [AX](U4-Safari-23-zh-404-pasted-query.ax.txt)|2026-10-16T04:49:55.726Z|用原生粘贴输入快速开始，记录搜索状态；该操作不证明IME组合输入。|
|[U4-Safari-24-zh-404-destination.jpg](U4-Safari-24-zh-404-destination.jpg) · [AX](U4-Safari-24-zh-404-destination.ax.txt)|2026-10-16T04:50:01.072Z|在2个中文结果已可见后实际Enter，核对中文quickstart目标与关闭状态。|
|[U4-Safari-25-settings-restored.jpg](U4-Safari-25-settings-restored.jpg) · [AX](U4-Safari-25-settings-restored.ax.txt)|2026-10-16T04:50:33.479Z|恢复跟随系统与原生100%缩放，不改其他网站权限。|
|[U4-Safari-26-window-restored.jpg](U4-Safari-26-window-restored.jpg) · [AX](U4-Safari-26-window-restored.ax.txt)|2026-10-16T04:50:34.541Z|结束Safari补充采集，使用原窗口缩放动作恢复原窗口尺寸。|
