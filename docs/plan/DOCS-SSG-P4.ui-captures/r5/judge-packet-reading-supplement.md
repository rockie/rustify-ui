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

## 本评审只读取以下分组

|序号|原图/AX|UTC|实际动作|取证边界|
|---|---|---|---|---|
|1|[U4-iPhone-01-zh-home.jpg](U4-iPhone-01-zh-home.jpg) · [AX](U4-iPhone-01-zh-home.ax.txt)|2026-10-16T08:01:27.846Z|Safari通过当前局域网地址实际打开本轮官方站中文首页。|按实际图像判断；动作意图不代替结果。|
|6|[U4-iPhone-06-fallback-destination.jpg](U4-iPhone-06-fallback-destination.jpg) · [AX](U4-iPhone-06-fallback-destination.ax.txt)|2026-10-16T08:03:28.085Z|点选中文结果中的Architecture缺译条目，记录实际落地页面与锚点。|按实际图像判断；动作意图不代替结果。|
|7|[U4-iPhone-07-reading-top.jpg](U4-iPhone-07-reading-top.jpg) · [AX](U4-iPhone-07-reading-top.ax.txt)|2026-10-16T08:04:46.178Z|点击Safari系统状态栏返回阅读页顶部，记录实际阅读状态。|状态栏回顶后的即时帧；顶部与note以08继续核对。|
|8|[U4-iPhone-08-fallback-top.jpg](U4-iPhone-08-fallback-top.jpg) · [AX](U4-iPhone-08-fallback-top.ax.txt)|2026-10-16T08:05:00.869Z|向上实际滚动阅读页，查看Architecture标题、缺译说明与导航。|按实际图像判断；动作意图不代替结果。|
|9|[U4-iPhone-09-dark-reading.jpg](U4-iPhone-09-dark-reading.jpg) · [AX](U4-iPhone-09-dark-reading.ax.txt)|2026-10-16T08:05:10.710Z|点击深色主题控件，查看真实iPhone上的正文、note和导航。|按实际图像判断；动作意图不代替结果。|
|10|[U4-iPhone-10-navigation-open.jpg](U4-iPhone-10-navigation-open.jpg) · [AX](U4-iPhone-10-navigation-open.ax.txt)|2026-10-16T08:05:11.265Z|点击阅读页菜单summary，查看真实窄屏导航。|按实际图像判断；动作意图不代替结果。|
|11|[U4-iPhone-11-quickstart-navigation.jpg](U4-iPhone-11-quickstart-navigation.jpg) · [AX](U4-iPhone-11-quickstart-navigation.ax.txt)|2026-10-16T08:05:22.646Z|在已展开菜单中点击快速开始，查看真实中文阅读页。|按实际图像判断；动作意图不代替结果。|
|16|[U4-iPhone-16-language-menu.jpg](U4-iPhone-16-language-menu.jpg) · [AX](U4-iPhone-16-language-menu.ax.txt)|2026-10-16T08:07:41.646Z|关闭搜索后点击中文语言summary，核对菜单的实际显示与边界。|按实际图像判断；动作意图不代替结果。|
|17|[U4-iPhone-17-english-reading.jpg](U4-iPhone-17-english-reading.jpg) · [AX](U4-iPhone-17-english-reading.ax.txt)|2026-10-16T08:07:49.244Z|点击语言菜单English，核对同一阅读页的英文版本与界面。|按实际图像判断；动作意图不代替结果。|
|18|[U4-iPhone-18-toc-open.jpg](U4-iPhone-18-toc-open.jpg) · [AX](U4-iPhone-18-toc-open.ax.txt)|2026-10-16T08:08:01.087Z|展开On this page目录，检查真实章节导航。|按实际图像判断；动作意图不代替结果。|
|19|[U4-iPhone-19-toc-destination.jpg](U4-iPhone-19-toc-destination.jpg) · [AX](U4-iPhone-19-toc-destination.ax.txt)|2026-10-16T08:08:07.826Z|点选Build and preview an example，记录实际章节落地和代码面板。|按实际图像判断；动作意图不代替结果。|
|20|[U4-iPhone-20-code-horizontal-scroll.jpg](U4-iPhone-20-code-horizontal-scroll.jpg) · [AX](U4-iPhone-20-code-horizontal-scroll.ax.txt)|2026-10-16T08:08:30.278Z|在代码面板内实际横向滚动，观察代码与页面其他区域。|横滚尝试，没有从此图宣称已横滚成功。|
|21|[U4-iPhone-21-copy-feedback.jpg](U4-iPhone-21-copy-feedback.jpg) · [AX](U4-iPhone-21-copy-feedback.ax.txt)|2026-10-16T08:08:30.867Z|点击代码Copy，记录本机局域网HTTP访问下的实际反馈。|按实际图像判断；动作意图不代替结果。|
|22|[U4-iPhone-22-code-scroll-opposite.jpg](U4-iPhone-22-code-scroll-opposite.jpg) · [AX](U4-iPhone-22-code-scroll-opposite.ax.txt)|2026-10-16T08:09:20.771Z|清除代码选择后反向横滚，记录代码首尾是否移动，不预写横滚成功。|反向横滚尝试，仍未确认代码首尾位移。|
|27|[U4-iPhone-27-404-destination.jpg](U4-iPhone-27-404-destination.jpg) · [AX](U4-iPhone-27-404-destination.ax.txt)|2026-10-16T08:10:38.857Z|点选中文404搜索的快速开始结果，记录实际落地阅读页。|按实际图像判断；动作意图不代替结果。|
|28|[U4-iPhone-28-device-os-version.jpg](U4-iPhone-28-device-os-version.jpg) · [AX](U4-iPhone-28-device-os-version.ax.txt)|2026-10-16T08:12:33.864Z|实际打开系统版本页面，仅保存不含序列号的版本信息。|按实际图像判断；动作意图不代替结果。|
|37|[U4-iPhone-37-home-restored.jpg](U4-iPhone-37-home-restored.jpg) · [AX](U4-iPhone-37-home-restored.ax.txt)|2026-10-16T08:17:27.447Z|回到官方中文首页并恢复跟随系统主题，结束本轮手机采集。|按实际图像判断；动作意图不代替结果。|

## 新增补采

本次以相同官方中文首页重走同一结果激活，38为前态，39为激活后首帧，40为不触碰续帧，41为回首页。只按新旧图评审，不接收原评审结论。本组共21张原图，U4仍不完整，未评分。

|序号|原图/AX|UTC|实际动作|
|---|---|---|---|
|38|[U4-iPhone-38-fallback-recheck-results.jpg](U4-iPhone-38-fallback-recheck-results.jpg) · [AX](U4-iPhone-38-fallback-recheck-results.ax.txt)|2026-10-16T08:24:08.426Z|补采：从官方中文首页打开搜索，逐键输入themescope；重走缺译结果激活。|
|39|[U4-iPhone-39-fallback-recheck-destination.jpg](U4-iPhone-39-fallback-recheck-destination.jpg) · [AX](U4-iPhone-39-fallback-recheck-destination.ax.txt)|2026-10-16T08:24:14.144Z|补采：触摸Architecture缺译结果；此为激活后的第一帧，后续不触碰继续记录停稳状态。|
|40|[U4-iPhone-40-fallback-recheck-settled.jpg](U4-iPhone-40-fallback-recheck-settled.jpg) · [AX](U4-iPhone-40-fallback-recheck-settled.ax.txt)|2026-10-16T08:24:20.076Z|补采：保持39的落地页面不触碰、不滚动，拍摄后续稳定帧以核对章节标题和顶栏。|
|41|[U4-iPhone-41-supplement-home-restored.jpg](U4-iPhone-41-supplement-home-restored.jpg) · [AX](U4-iPhone-41-supplement-home-restored.ax.txt)|2026-10-16T08:24:27.535Z|补采结束：通过原官方地址回到中文首页，保持跟随系统主题。|
