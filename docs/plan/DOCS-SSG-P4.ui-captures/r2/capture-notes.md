# R2 采集补充与缺证

## 原生补充（U4未评分）

- CUA在原生Google Chrome本任务tab1319606376上先Cmd+0，再五次Cmd++，AX显示“缩放比例：200%”。截图U4-native-chrome-zoom200-read.jpg。
- Esc关闭缩放浮层、Cmd+K、键入CustomOnlyToken，AX显示1 result；截图U4-native-chrome-zoom200-search.jpg。之后Esc关闭搜索，Cmd+0，AX显示“缩放比例：100%”。工具返回JPEG，按实际格式保存.jpg，3024×1752像素；未改图。
- Kimi会话docs-ssg-p4-ui-review-r2，标签组“P4 UI 复审 R2”。恢复100%后实测UA Chrome/154.0.0.0、CSS1331×773、DPR2，2026-10-16T03:39:51.173Z，URL http://127.0.0.1:4205/rustify-ui/ 。主题system，实际light。200%时CSS视口/DPR未取得，不从截图反推。搜索AX另存U4-native-chrome-zoom200-ax.txt。

## 没采到与未验证

- U4缺完整原生稳定Chrome/Firefox/Safari任务矩阵；本机未见Firefox.app，当前Chrome显示“重新启动即可更新”，未确认最新稳定身份。本轮Chrome原生200%只是定制页阅读与搜索冒烟，未覆盖完整官方站阅读、代码、宽表等任务，不计U4分。
- iOS Safari、Android Chrome实体设备与软键盘会话、VoiceOver+Safari实际朗读不可用。需维护者提供可操作实体设备/远程原生会话，以及音频、转录或原生朗读操作记录。未自行安装浏览器、购买服务或以引擎/视口模拟代填。
- 实际IME、hover/pressed瞬态、完整原生任务等未操作项保持未知，不据代码推断通过或开差距。

## 采集器补正与排除

- u2-extra attempt1尚未确认旧JSON请求发出便关闭，可能把新会话首个请求延迟，最后两帧未取得。attempt2先等待旧请求实际命中拦截器，再关闭/重开；09/10帧记录新结果先出现、再释放旧JSON。日志均保留，此采集前置失配不作产品差距。
- r2-extra attempt1的waitForURL被旧地址满足，截图后发生导航，观察抛Execution context destroyed。未登记的U1-subpath-activation-1280-light.png不交评审；a2等待URL变化和networkidle后重拍。原图和失败日志保留。
- 首次write-capture的内联Python因命令输入编码失败，未成功写入清单；改用落盘Node脚本生成并校验图像尺寸、hash。没有产品修复。
- 本轮只写采集、报告和SDLC；不修改产品代码、计划与历史报告。临时服务结束前停止，见server-lifecycle.md。
