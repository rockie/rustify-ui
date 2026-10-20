# R3 · 原生补充、缺证与事后实测

## 实拍与证据边界

正式独立评分材料为本轮重新拍摄的91张原图：U1=38、U2=23、U3=30。三位全新上下文评审逐图打开本单元全部本轮图，分别复看R2同名图7、5、2张。没有给评审代码、diff、修复记录、完整报告、机器结果或门槛。原始结论分别保存在judge-U1-a.md、judge-U2-a.md、judge-U3-a.md；合并只归纳理由，四维组与分数未改。

延续既有人工循环。没有DOCS-SSG-P4.ui-review.json，已有R1/R2 Markdown历史；本轮未import、begin、安装或修改engine。本轮未重新探测CLI，不把R2的能力异常当成本轮新探测结果。目标包全文hash与R2相同，保留原人工指纹算法，不冒称Phoenix协议兼容。

## 本轮原生操作

原图、保存时间和hash见[supplement-captures.json](supplement-captures.json)。保存时间仅辅助定位；真实性来自本轮工具操作，未裁图、拼图或改图。CUA返回的JPEG按真实格式保存。

- Kimi原生Chrome：新建任务组“P4 UI 验收 R3”，从4205中文页面点搜索，输入“主题切换”。首张图仍在loading，第二张真实返回1个结果，两次attempt均保留。UA为Chrome/154.0.0.0；100%时实测CSS视口1331×773、DPR2。Browser.getVersion不受扩展CDP支持，返回-32601，不据此声称取得完整patch版本。
- 原生Chrome200%：CUA在本任务页转到4199中文quickstart，Cmd+0后五次Cmd+=；AX明确显示“缩放比例：200%”。保存阅读图，Cmd+K打开搜索并输入ThemeScope，实际显示3个结果，保存搜索图；Escape关闭、Cmd+0后AX显示100%。这只是阅读/搜索关键帧，不覆盖代码/宽表、全部页面和原生任务矩阵。
- 原生Chrome Tab边界：在搜索输入按Tab后，页面内activeElement为BODY的现象在原生窗口中对应地址栏获得焦点，地址栏有清晰焦点边界；原图U4-native-chrome-tab-boundary.jpg。Shift+Tab返回ThemeScope输入，Escape关闭。未把浏览器工具栏焦点当作产品内焦点丢失。
- 原生Safari：CUA新建本轮页签，普通访问4199中文quickstart；点搜索并实际键入ThemeScope，显示3个结果；Down、Enter导航到`/rustify-ui/zh/architecture/#versioned-themes`，对话框关闭且目标章节可见。随后点深色、展开中文语言菜单并实拍，Escape关闭，恢复“跟随系统”。截图为1280×1600物理像素，未测其CSS视口和原生缩放，不据截图像素猜缩放比例。
- 本机安装包：Safari27.0.1、Chrome155.0.8059.82；OS为macOS27.0.1/build26A434。安装包版本不代替当前运行版本；Chrome窗口提示“重新启动即可更新”，本轮没有重启用户浏览器或升级任何工具。Safari未另核对当前稳定渠道完整版本。

原生操作对应AX原文和本轮截图在同目录U4-*。没有完整原生网络/console采集，不能把Playwright日志结论外推至全部原生操作。

## 仍未采到的U4格子

| 格子 | 本轮证据 | 仍缺与获取方式 |
| --- | --- | --- |
| 当前稳定桌面Chrome | 已有Chrome154族部分原生冒烟；磁盘有新版本且提示重启 | 维护者提供可操作的当前稳定会话，再完成阅读/导航/中英文查询/失败恢复/主题/复制与演示等完整任务；本轮不重启共享浏览器 |
| 当前稳定Firefox | 本轮没有可操作的Firefox会话，/Applications清单未见Firefox | 提供已有本机或远程原生Firefox会话；不安装浏览器或把Playwright引擎当原生 |
| 当前稳定Safari | 阅读、搜索、缺译锚点、主题和菜单部分原生操作已补 | 核对运行版本并完成完整稳定浏览器任务、失败恢复、查询词首及复制/演示路径 |
| iOS Safari实体设备 | 未接入 | 提供可操作设备或远程原生会话，补阅读、导航、搜索、主题、软键盘和动态视口 |
| Android Chrome实体设备 | 未接入 | 同上；375px/320px只作为CSS视口 |
| 原生200%完整走查 | Chrome阅读与搜索关键帧已补 | 补各页面类型、长代码/宽表内部滚动及完整操作；不把局部冒烟视为全格通过 |
| VoiceOver+Safari | 没有实际朗读记录 | 提供可操作且可记录朗读的原生会话，取得结果数量、当前选择、复制成功/失败的音频/转录或原生操作证据 |

用户本轮未提供新增原生连接信息，沿用已有环境继续采集；未获得整体现状豁免。U4不补零分，也不以已有局部证据补满矩阵。

## 独立评审之后的补充实测

原文中的未证状态不回改为评分结论。本轮随后在真实页面执行：

- runtime-check-runner.cjs：57个页面/状态实测，30次axe扫描；数据见runtime-checks.json、runtime-summary.json及axe-incomplete.json。
- keyboard-check.cjs：真实Tab/Shift+Tab/Enter/Escape，记录跳到正文、语言菜单Escape即时归焦、搜索关闭归焦/重开保留查询、关闭按钮焦点与归焦；实际点击首页Quick start/Components、中文quickstart的English链接和首章节目录；代码面板以真实横向wheel滚动；禁脚本导航到中文Architecture；module失败中把词改为ReachableCustomToken再键盘Retry，显示1 result且输入获焦。各步骤、原图与实测值见keyboard-checks.json及M2-*.png。
- 完整IME组合输入、全部hover/pressed/动画过程、所有章节/代码/宽表、复制与演示的全键盘路径仍未覆盖，不推断通过。原生辅助技术缺证不由DOM属性代填。
- 13个color-contrast、2个aria-required-children、22个aria-valid-attr-value仍为axe incomplete规则实例，不是0问题的人工认证。原始节点和原因保存，未把未知项改成已通过。

本轮没有产品修复，不重跑Rust/lint、既有全套E2E或1000页性能门；100页语料仅用于真实界面采集与交互。
