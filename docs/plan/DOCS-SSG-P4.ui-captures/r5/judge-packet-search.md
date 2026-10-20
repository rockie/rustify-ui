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
|2|[U4-iPhone-02-search-open.jpg](U4-iPhone-02-search-open.jpg) · [AX](U4-iPhone-02-search-open.ax.txt)|2026-10-16T08:01:38.909Z|点击页头搜索按钮，记录打开与软键盘状态。|按实际图像判断；动作意图不代替结果。|
|3|[U4-iPhone-03-search-input-tap.jpg](U4-iPhone-03-search-input-tap.jpg) · [AX](U4-iPhone-03-search-input-tap.ax.txt)|2026-10-16T08:02:05.294Z|点击已聚焦输入框，核对实际屏幕键盘状态。|按实际图像判断；动作意图不代替结果。|
|4|[U4-iPhone-04-theme-query.jpg](U4-iPhone-04-theme-query.jpg) · [AX](U4-iPhone-04-theme-query.ax.txt)|2026-10-16T08:03:06.177Z|向搜索输入ThemeScope，记录即时状态和实际查询文字。|按实际图像判断；动作意图不代替结果。|
|5|[U4-iPhone-05-theme-results.jpg](U4-iPhone-05-theme-results.jpg) · [AX](U4-iPhone-05-theme-results.ax.txt)|2026-10-16T08:03:14.998Z|查询值保持不变，按Left检查输入仍可编辑；观察加载后的实际结果。|按实际图像判断；动作意图不代替结果。|
|12|[U4-iPhone-12-empty-query-attempt.jpg](U4-iPhone-12-empty-query-attempt.jpg) · [AX](U4-iPhone-12-empty-query-attempt.ax.txt)|2026-10-16T08:07:08.580Z|在深色阅读页输入不存在的查询，保存实际即时状态。|按实际图像判断；动作意图不代替结果。|
|13|[U4-iPhone-13-empty-result.jpg](U4-iPhone-13-empty-result.jpg) · [AX](U4-iPhone-13-empty-result.ax.txt)|2026-10-16T08:07:09.059Z|查询保持不变，核对实际无结果建议及输入焦点。|按实际图像判断；动作意图不代替结果。|
|14|[U4-iPhone-14-search-closed.jpg](U4-iPhone-14-search-closed.jpg) · [AX](U4-iPhone-14-search-closed.ax.txt)|2026-10-16T08:07:25.968Z|点击对话框关闭按钮，记录真实关闭后的阅读页。|按实际图像判断；动作意图不代替结果。|
|15|[U4-iPhone-15-search-reopened.jpg](U4-iPhone-15-search-reopened.jpg) · [AX](U4-iPhone-15-search-reopened.ax.txt)|2026-10-16T08:07:26.652Z|重新点击搜索，核对查询与无结果状态是否保留。|按实际图像判断；动作意图不代替结果。|
|23|[U4-iPhone-23-zh-deep-404.jpg](U4-iPhone-23-zh-deep-404.jpg) · [AX](U4-iPhone-23-zh-deep-404.ax.txt)|2026-10-16T08:09:44.705Z|通过当前局域网官方入口访问中文深404，核对实际错误页和搜索入口。|按实际图像判断；动作意图不代替结果。|
|24|[U4-iPhone-24-404-query.jpg](U4-iPhone-24-404-query.jpg) · [AX](U4-iPhone-24-404-query.ax.txt)|2026-10-16T08:09:59.556Z|从中文404点击搜索并输入ThemeScope，保存真实即时状态。|批量typeText的实际输入有漏字，图中不是ThemeScope；不能记目标词搜索失败。|
|25|[U4-iPhone-25-404-results.jpg](U4-iPhone-25-404-results.jpg) · [AX](U4-iPhone-25-404-results.ax.txt)|2026-10-16T08:10:00.016Z|保持查询不变，核对404内当前中文界面的真实结果。|仍为漏字查询，实际0结果；校正后的成功依据是26。|
|26|[U4-iPhone-26-404-query-corrected.jpg](U4-iPhone-26-404-query-corrected.jpg) · [AX](U4-iPhone-26-404-query-corrected.ax.txt)|2026-10-16T08:10:26.019Z|将工具漏字的查询改为逐键输入themescope，按实际文字与结果计证。|按实际图像判断；动作意图不代替结果。|
|29|[U4-iPhone-29-fault-fixture-entry.jpg](U4-iPhone-29-fault-fixture-entry.jpg) · [AX](U4-iPhone-29-fault-fixture-entry.ax.txt)|2026-10-16T08:14:14.739Z|进入真实100页搜索语料的隔离副本；只暂扣module造成HTTP404。|按实际图像判断；动作意图不代替结果。|
|30|[U4-iPhone-30-module-error.jpg](U4-iPhone-30-module-error.jpg) · [AX](U4-iPhone-30-module-error.ax.txt)|2026-10-16T08:14:25.386Z|点击搜索并逐键输入customonlytoken；暂扣module期间记录真实失败状态。|按实际图像判断；动作意图不代替结果。|
|31|[U4-iPhone-31-retry-attempt.jpg](U4-iPhone-31-retry-attempt.jpg) · [AX](U4-iPhone-31-retry-attempt.ax.txt)|2026-10-16T08:14:45.222Z|恢复原module字节后，实际点击Retry，记录即时恢复状态。|按实际图像判断；动作意图不代替结果。|
|32|[U4-iPhone-32-retry-result.jpg](U4-iPhone-32-retry-result.jpg) · [AX](U4-iPhone-32-retry-result.ax.txt)|2026-10-16T08:14:45.640Z|查询值保持customonlytoken，核对同页重试后的真实结果和输入。|按实际图像判断；动作意图不代替结果。|
|33|[U4-iPhone-33-many-results.jpg](U4-iPhone-33-many-results.jpg) · [AX](U4-iPhone-33-many-results.ax.txt)|2026-10-16T08:15:13.923Z|逐键输入搜索语料共同词，记录大量结果和Show more控件。|按实际图像判断；动作意图不代替结果。|
|34|[U4-iPhone-34-show-more.jpg](U4-iPhone-34-show-more.jpg) · [AX](U4-iPhone-34-show-more.ax.txt)|2026-10-16T08:15:27.911Z|实际点击Show more，核对全局总数与顶部控件。|按实际图像判断；动作意图不代替结果。|
|35|[U4-iPhone-35-result-list-scroll.jpg](U4-iPhone-35-result-list-scroll.jpg) · [AX](U4-iPhone-35-result-list-scroll.ax.txt)|2026-10-16T08:15:28.388Z|在结果列表内实际向下滚动，核对输入、关闭与更多是否仍可达。|列表滚动即时帧；稳定状态另见36。|
|36|[U4-iPhone-36-list-settled.jpg](U4-iPhone-36-list-settled.jpg) · [AX](U4-iPhone-36-list-settled.ax.txt)|2026-10-16T08:17:11.018Z|保存滚动尝试后的稳定结果列表画面；本次不据按键推断滚动成功。|按实际图像判断；动作意图不代替结果。|
