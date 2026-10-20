# R5 · U4 iPhone 原生补证采集

本轮仅U4原生iPhone范围，U1-U3及其他原生平台本轮未复核；U4整体矩阵不全，不补零分或完整单元分。

- 真实设备：本轮CUA读取设置的型号名称为iPhone14 Pro，iOS26.6.2有安全版本页原图；型号字段原整页含序列号，未保存。系统Safari独立patch与实际CSS视口未读取。
- CUA通过用户已接入的本机iPhone Mirroring操作真实设备；全部图本轮新拍，不是模拟器，不修改、裁剪或拼图。AX只提供镜像窗口，不能代替手机DOM、焦点或朗读。
- 固定CLI0.1.27 C1bbc54b0/R740610db，HEAD93c4d101；相关产品与C无diff。复用同字节真实生成产物：官方4199、100页/语言语料4205；LAN当前192.168.123.44，base /rustify-ui/。
- 主题通过可见站内控件切换，最后恢复跟随系统。Safari自身缩放、手机旋转与系统动效偏好未改。镜像物理像素不当作CSS视口。
- 只有搜索4205隔离副本暂扣module造成实际HTTP404，恢复相同字节再同页Retry；无页面/脚本内容替换。
- 批量typeText在镜像中曾漏字或漏冒号；后续采用逐键输入。根据画面实际查询计证，24/25不算ThemeScope查询失败，26才是正确themescope。
- 搜索输入只显示工具栏，未拍到完整软件键盘；Eject与XF86Eject返回工具keyNotFound。横滚与列表内滚动也只按实际画面判断，不从操作意图推成功。
- 当前完整URL未从镜像AX读取，入口与点击链在操作中，实际页面/语言/标题看原图。不从代码补URL或行为。

## 全部原图

|序号|原图/AX|UTC|实际动作|取证边界|
|---|---|---|---|---|
|1|[U4-iPhone-01-zh-home.jpg](U4-iPhone-01-zh-home.jpg) · [AX](U4-iPhone-01-zh-home.ax.txt)|2026-10-16T08:01:27.846Z|Safari通过当前局域网地址实际打开本轮官方站中文首页。|按实际图像判断；动作意图不代替结果。|
|2|[U4-iPhone-02-search-open.jpg](U4-iPhone-02-search-open.jpg) · [AX](U4-iPhone-02-search-open.ax.txt)|2026-10-16T08:01:38.909Z|点击页头搜索按钮，记录打开与软键盘状态。|按实际图像判断；动作意图不代替结果。|
|3|[U4-iPhone-03-search-input-tap.jpg](U4-iPhone-03-search-input-tap.jpg) · [AX](U4-iPhone-03-search-input-tap.ax.txt)|2026-10-16T08:02:05.294Z|点击已聚焦输入框，核对实际屏幕键盘状态。|按实际图像判断；动作意图不代替结果。|
|4|[U4-iPhone-04-theme-query.jpg](U4-iPhone-04-theme-query.jpg) · [AX](U4-iPhone-04-theme-query.ax.txt)|2026-10-16T08:03:06.177Z|向搜索输入ThemeScope，记录即时状态和实际查询文字。|按实际图像判断；动作意图不代替结果。|
|5|[U4-iPhone-05-theme-results.jpg](U4-iPhone-05-theme-results.jpg) · [AX](U4-iPhone-05-theme-results.ax.txt)|2026-10-16T08:03:14.998Z|查询值保持不变，按Left检查输入仍可编辑；观察加载后的实际结果。|按实际图像判断；动作意图不代替结果。|
|6|[U4-iPhone-06-fallback-destination.jpg](U4-iPhone-06-fallback-destination.jpg) · [AX](U4-iPhone-06-fallback-destination.ax.txt)|2026-10-16T08:03:28.085Z|点选中文结果中的Architecture缺译条目，记录实际落地页面与锚点。|按实际图像判断；动作意图不代替结果。|
|7|[U4-iPhone-07-reading-top.jpg](U4-iPhone-07-reading-top.jpg) · [AX](U4-iPhone-07-reading-top.ax.txt)|2026-10-16T08:04:46.178Z|点击Safari系统状态栏返回阅读页顶部，记录实际阅读状态。|状态栏回顶后的即时帧；顶部与note以08继续核对。|
|8|[U4-iPhone-08-fallback-top.jpg](U4-iPhone-08-fallback-top.jpg) · [AX](U4-iPhone-08-fallback-top.ax.txt)|2026-10-16T08:05:00.869Z|向上实际滚动阅读页，查看Architecture标题、缺译说明与导航。|按实际图像判断；动作意图不代替结果。|
|9|[U4-iPhone-09-dark-reading.jpg](U4-iPhone-09-dark-reading.jpg) · [AX](U4-iPhone-09-dark-reading.ax.txt)|2026-10-16T08:05:10.710Z|点击深色主题控件，查看真实iPhone上的正文、note和导航。|按实际图像判断；动作意图不代替结果。|
|10|[U4-iPhone-10-navigation-open.jpg](U4-iPhone-10-navigation-open.jpg) · [AX](U4-iPhone-10-navigation-open.ax.txt)|2026-10-16T08:05:11.265Z|点击阅读页菜单summary，查看真实窄屏导航。|按实际图像判断；动作意图不代替结果。|
|11|[U4-iPhone-11-quickstart-navigation.jpg](U4-iPhone-11-quickstart-navigation.jpg) · [AX](U4-iPhone-11-quickstart-navigation.ax.txt)|2026-10-16T08:05:22.646Z|在已展开菜单中点击快速开始，查看真实中文阅读页。|按实际图像判断；动作意图不代替结果。|
|12|[U4-iPhone-12-empty-query-attempt.jpg](U4-iPhone-12-empty-query-attempt.jpg) · [AX](U4-iPhone-12-empty-query-attempt.ax.txt)|2026-10-16T08:07:08.580Z|在深色阅读页输入不存在的查询，保存实际即时状态。|按实际图像判断；动作意图不代替结果。|
|13|[U4-iPhone-13-empty-result.jpg](U4-iPhone-13-empty-result.jpg) · [AX](U4-iPhone-13-empty-result.ax.txt)|2026-10-16T08:07:09.059Z|查询保持不变，核对实际无结果建议及输入焦点。|按实际图像判断；动作意图不代替结果。|
|14|[U4-iPhone-14-search-closed.jpg](U4-iPhone-14-search-closed.jpg) · [AX](U4-iPhone-14-search-closed.ax.txt)|2026-10-16T08:07:25.968Z|点击对话框关闭按钮，记录真实关闭后的阅读页。|按实际图像判断；动作意图不代替结果。|
|15|[U4-iPhone-15-search-reopened.jpg](U4-iPhone-15-search-reopened.jpg) · [AX](U4-iPhone-15-search-reopened.ax.txt)|2026-10-16T08:07:26.652Z|重新点击搜索，核对查询与无结果状态是否保留。|按实际图像判断；动作意图不代替结果。|
|16|[U4-iPhone-16-language-menu.jpg](U4-iPhone-16-language-menu.jpg) · [AX](U4-iPhone-16-language-menu.ax.txt)|2026-10-16T08:07:41.646Z|关闭搜索后点击中文语言summary，核对菜单的实际显示与边界。|按实际图像判断；动作意图不代替结果。|
|17|[U4-iPhone-17-english-reading.jpg](U4-iPhone-17-english-reading.jpg) · [AX](U4-iPhone-17-english-reading.ax.txt)|2026-10-16T08:07:49.244Z|点击语言菜单English，核对同一阅读页的英文版本与界面。|按实际图像判断；动作意图不代替结果。|
|18|[U4-iPhone-18-toc-open.jpg](U4-iPhone-18-toc-open.jpg) · [AX](U4-iPhone-18-toc-open.ax.txt)|2026-10-16T08:08:01.087Z|展开On this page目录，检查真实章节导航。|按实际图像判断；动作意图不代替结果。|
|19|[U4-iPhone-19-toc-destination.jpg](U4-iPhone-19-toc-destination.jpg) · [AX](U4-iPhone-19-toc-destination.ax.txt)|2026-10-16T08:08:07.826Z|点选Build and preview an example，记录实际章节落地和代码面板。|按实际图像判断；动作意图不代替结果。|
|20|[U4-iPhone-20-code-horizontal-scroll.jpg](U4-iPhone-20-code-horizontal-scroll.jpg) · [AX](U4-iPhone-20-code-horizontal-scroll.ax.txt)|2026-10-16T08:08:30.278Z|在代码面板内实际横向滚动，观察代码与页面其他区域。|横滚尝试，没有从此图宣称已横滚成功。|
|21|[U4-iPhone-21-copy-feedback.jpg](U4-iPhone-21-copy-feedback.jpg) · [AX](U4-iPhone-21-copy-feedback.ax.txt)|2026-10-16T08:08:30.867Z|点击代码Copy，记录本机局域网HTTP访问下的实际反馈。|按实际图像判断；动作意图不代替结果。|
|22|[U4-iPhone-22-code-scroll-opposite.jpg](U4-iPhone-22-code-scroll-opposite.jpg) · [AX](U4-iPhone-22-code-scroll-opposite.ax.txt)|2026-10-16T08:09:20.771Z|清除代码选择后反向横滚，记录代码首尾是否移动，不预写横滚成功。|反向横滚尝试，仍未确认代码首尾位移。|
|23|[U4-iPhone-23-zh-deep-404.jpg](U4-iPhone-23-zh-deep-404.jpg) · [AX](U4-iPhone-23-zh-deep-404.ax.txt)|2026-10-16T08:09:44.705Z|通过当前局域网官方入口访问中文深404，核对实际错误页和搜索入口。|按实际图像判断；动作意图不代替结果。|
|24|[U4-iPhone-24-404-query.jpg](U4-iPhone-24-404-query.jpg) · [AX](U4-iPhone-24-404-query.ax.txt)|2026-10-16T08:09:59.556Z|从中文404点击搜索并输入ThemeScope，保存真实即时状态。|批量typeText的实际输入有漏字，图中不是ThemeScope；不能记目标词搜索失败。|
|25|[U4-iPhone-25-404-results.jpg](U4-iPhone-25-404-results.jpg) · [AX](U4-iPhone-25-404-results.ax.txt)|2026-10-16T08:10:00.016Z|保持查询不变，核对404内当前中文界面的真实结果。|仍为漏字查询，实际0结果；校正后的成功依据是26。|
|26|[U4-iPhone-26-404-query-corrected.jpg](U4-iPhone-26-404-query-corrected.jpg) · [AX](U4-iPhone-26-404-query-corrected.ax.txt)|2026-10-16T08:10:26.019Z|将工具漏字的查询改为逐键输入themescope，按实际文字与结果计证。|按实际图像判断；动作意图不代替结果。|
|27|[U4-iPhone-27-404-destination.jpg](U4-iPhone-27-404-destination.jpg) · [AX](U4-iPhone-27-404-destination.ax.txt)|2026-10-16T08:10:38.857Z|点选中文404搜索的快速开始结果，记录实际落地阅读页。|按实际图像判断；动作意图不代替结果。|
|28|[U4-iPhone-28-device-os-version.jpg](U4-iPhone-28-device-os-version.jpg) · [AX](U4-iPhone-28-device-os-version.ax.txt)|2026-10-16T08:12:33.864Z|实际打开系统版本页面，仅保存不含序列号的版本信息。|按实际图像判断；动作意图不代替结果。|
|29|[U4-iPhone-29-fault-fixture-entry.jpg](U4-iPhone-29-fault-fixture-entry.jpg) · [AX](U4-iPhone-29-fault-fixture-entry.ax.txt)|2026-10-16T08:14:14.739Z|进入真实100页搜索语料的隔离副本；只暂扣module造成HTTP404。|按实际图像判断；动作意图不代替结果。|
|30|[U4-iPhone-30-module-error.jpg](U4-iPhone-30-module-error.jpg) · [AX](U4-iPhone-30-module-error.ax.txt)|2026-10-16T08:14:25.386Z|点击搜索并逐键输入customonlytoken；暂扣module期间记录真实失败状态。|按实际图像判断；动作意图不代替结果。|
|31|[U4-iPhone-31-retry-attempt.jpg](U4-iPhone-31-retry-attempt.jpg) · [AX](U4-iPhone-31-retry-attempt.ax.txt)|2026-10-16T08:14:45.222Z|恢复原module字节后，实际点击Retry，记录即时恢复状态。|按实际图像判断；动作意图不代替结果。|
|32|[U4-iPhone-32-retry-result.jpg](U4-iPhone-32-retry-result.jpg) · [AX](U4-iPhone-32-retry-result.ax.txt)|2026-10-16T08:14:45.640Z|查询值保持customonlytoken，核对同页重试后的真实结果和输入。|按实际图像判断；动作意图不代替结果。|
|33|[U4-iPhone-33-many-results.jpg](U4-iPhone-33-many-results.jpg) · [AX](U4-iPhone-33-many-results.ax.txt)|2026-10-16T08:15:13.923Z|逐键输入搜索语料共同词，记录大量结果和Show more控件。|按实际图像判断；动作意图不代替结果。|
|34|[U4-iPhone-34-show-more.jpg](U4-iPhone-34-show-more.jpg) · [AX](U4-iPhone-34-show-more.ax.txt)|2026-10-16T08:15:27.911Z|实际点击Show more，核对全局总数与顶部控件。|按实际图像判断；动作意图不代替结果。|
|35|[U4-iPhone-35-result-list-scroll.jpg](U4-iPhone-35-result-list-scroll.jpg) · [AX](U4-iPhone-35-result-list-scroll.ax.txt)|2026-10-16T08:15:28.388Z|在结果列表内实际向下滚动，核对输入、关闭与更多是否仍可达。|列表滚动即时帧；稳定状态另见36。|
|36|[U4-iPhone-36-list-settled.jpg](U4-iPhone-36-list-settled.jpg) · [AX](U4-iPhone-36-list-settled.ax.txt)|2026-10-16T08:17:11.018Z|保存滚动尝试后的稳定结果列表画面；本次不据按键推断滚动成功。|按实际图像判断；动作意图不代替结果。|
|37|[U4-iPhone-37-home-restored.jpg](U4-iPhone-37-home-restored.jpg) · [AX](U4-iPhone-37-home-restored.ax.txt)|2026-10-16T08:17:27.447Z|回到官方中文首页并恢复跟随系统主题，结束本轮手机采集。|按实际图像判断；动作意图不代替结果。|

设备信息的原生观察摘要见device-environment.json；手机原有应用、账号信息与设备序列号未落盘。

## 本轮身份与共同条件

- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`；开始时tracked diff SHA256为`f826227f71ce67a5e4fcb9de710fb46974ff52dd58fd5e33a593b4b1f88ad67d`，仅指`git diff --binary`，不包含未跟踪文件。原有工作树见[baseline.json](baseline.json)。
- 固定CLI为`target/docs-ssg/p4/install-1bbc54b0/bin/rustify-docs`，0.1.27，C `1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a` / R `740610db164b3f396b0fcc551f5a5ef8f15da716`。相关产品路径与C无diff，未重新构建。
- 实际官网初始入口`http://192.168.123.44:4199/rustify-ui/zh/`；搜索语料入口`http://192.168.123.44:4205/rustify-ui/`。官网build_id `dd5022c567200779`，搜索语料`176bea4ed7e4d96f`。点击后完整URL无法由镜像AX取得，逐图不猜写。
- 目标全文SHA256 `ed08081e68367b02d7ad695a041909e2c8ec26d78abe377376d1e1b0b7a6ad02`；沿用规范指纹`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`，规范未改，不重建基线。
- 全部手机画面为同一实体设备竖屏；实际CSS视口未测量。01–08画面浅色；09–27深色；28为系统版本页；29–36浅色；37–41回到跟随系统控件、画面浅色。没有据静态主题按钮宣称系统切换跟随已验。
- 只读站点任务，无账号或业务数据写入。用户原有私人应用画面未保存。镜像曾暂停，补采前用原生继续按钮恢复；不把连接等待当作产品状态。
- 正式图共41张，其中40张产品图、1张安全系统版本图；无编辑/裁剪/拼接。每图及原始AX的hash见[phone-captures.json](phone-captures.json)，原始两组packet保留01–37的采集快照，补采packet追加38–41。采集helper追加时重写的元数据已由冻结packet恢复并重新核对，图片与AX字节未变。

## 补采 attempt · 缺译结果稳定落点

阅读评审看不清06是否已停止滚动。按其要求，从同一官方中文首页重走themescope→Architecture触摸激活，并连续拍首帧及无触碰稳定帧；未改产品、缩放或主题。最后恢复中文首页。原01–37及其原始评审保留。

|序号|原图/AX|UTC|实际动作|
|---|---|---|---|
|38|[U4-iPhone-38-fallback-recheck-results.jpg](U4-iPhone-38-fallback-recheck-results.jpg) · [AX](U4-iPhone-38-fallback-recheck-results.ax.txt)|2026-10-16T08:24:08.426Z|补采：从官方中文首页打开搜索，逐键输入themescope；重走缺译结果激活。|
|39|[U4-iPhone-39-fallback-recheck-destination.jpg](U4-iPhone-39-fallback-recheck-destination.jpg) · [AX](U4-iPhone-39-fallback-recheck-destination.ax.txt)|2026-10-16T08:24:14.144Z|补采：触摸Architecture缺译结果；此为激活后的第一帧，后续不触碰继续记录停稳状态。|
|40|[U4-iPhone-40-fallback-recheck-settled.jpg](U4-iPhone-40-fallback-recheck-settled.jpg) · [AX](U4-iPhone-40-fallback-recheck-settled.ax.txt)|2026-10-16T08:24:20.076Z|补采：保持39的落地页面不触碰、不滚动，拍摄后续稳定帧以核对章节标题和顶栏。|
|41|[U4-iPhone-41-supplement-home-restored.jpg](U4-iPhone-41-supplement-home-restored.jpg) · [AX](U4-iPhone-41-supplement-home-restored.ax.txt)|2026-10-16T08:24:27.535Z|补采结束：通过原官方地址回到中文首页，保持跟随系统主题。|
