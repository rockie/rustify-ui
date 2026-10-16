# DOCS-SSG-P4 UI R1 · 修复自检

本轮为实现者自检，不打分，不构成独立验收；UI-01–UI-04均为「已修改待评审」。

- 产品R：`740610db164b3f396b0fcc551f5a5ef8f15da716`；配对C：`1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a`，CLI0.1.27，真实Git安装，panic=unwind。
- 环境：Playwright1.63.0、Chromium153.0.8010.12完整可执行文件（headless），DPR1，reduce motion；显式点击主题按钮，字体就绪并等待两帧。窄视口不算实体手机。
- 官方入口：`http://127.0.0.1:4199/rustify-ui/`；搜索100页同构语料入口：`http://127.0.0.1:4205/rustify-ui/`。两者均由新C重建，未用旧CLI投影修复。
- build_id：官方`dd5022c567200779`，搜索`176bea4ed7e4d96f`。
- [运行实测](runtime.json)保存逐图时间、URL、CSS视口、DPR、主题、焦点、选中、宽度与背景；[检查摘要](checks.json)保留四项旧C₅红灯及新C三托管/官方断言，原报告在`target/docs-ssg/p4/ui-r1/results/`。
- 修复前官方C₅：1280浅色/320深色note均为透明，见[实测](before.json)、[浅色图](before-fallback-1280-light.png)、[深色图](before-fallback-320-dark.png)。旧C₅的其余三项由本轮真实浏览器实测及红灯报告定位。

| 图像 | CSS视口 | 主题 | 实测 |
| --- | --- | --- | --- |
| [keyboard-40-1280-light.png](keyboard-40-1280-light.png) | 1280×900 | light | 40组；焦点BUTTON，rgb(23, 92, 211) solid 3px；选中rd-search-option-1-2-1 |
| [keyboard-98-1280-light.png](keyboard-98-1280-light.png) | 1280×900 | light | 98组；焦点INPUT，rgb(23, 92, 211) solid 3px；选中rd-search-option-1-2-1 |
| [search-1280-light.png](search-1280-light.png) | 1280×900 | light | dialog外宽576px，x=352；border-box |
| [search-1280-dark.png](search-1280-dark.png) | 1280×900 | dark | dialog外宽576px，x=352；border-box |
| [search-375-light.png](search-375-light.png) | 375×812 | light | dialog外宽343px，x=16；border-box |
| [search-375-dark.png](search-375-dark.png) | 375×812 | dark | dialog外宽343px，x=16；border-box |
| [search-320-light.png](search-320-light.png) | 320×812 | light | dialog外宽288px，x=16；border-box |
| [search-320-dark.png](search-320-dark.png) | 320×812 | dark | dialog外宽288px，x=16；border-box |
| [count-375-dark.png](count-375-dark.png) | 375×812 | dark | 1 result |
| [fallback-1280-light.png](fallback-1280-light.png) | 1280×900 | light | note背景rgb(242, 244, 247) |
| [fallback-320-dark.png](fallback-320-dark.png) | 320×812 | dark | note背景rgb(22, 27, 38) |

真实键盘：Ctrl+K打开→输入SearchCorpusToken→下键选择→Tab到Show more→Enter连续20→40→60→80→98；每次两帧后焦点仍在可见控件，最后一批落到输入，选择保留，Esc回到实际触发点。测试另覆盖Tab/Shift+Tab。单数、0/2复数及中文数量均通过。

其他单元快速回归：三托管搜索恢复/旧响应/禁脚本/404/CSP及双主题布局共73项；官方英文首页、真实中文首页/quickstart与锚点、375/320导航/源码、深404及home/category/guide双主题采集6项通过。自检采集无console error、pageerror或requestfailed；测试审计保留故障注入和预期404的限定。

本目录共13张原始PNG（修复前2、修复后11），已逐张开图对照本轮规范；未合成、裁切或修图。R1报告、目标包、r1评审截图及judge未改动。U4的原生稳定浏览器完整矩阵、实体手机、原生200%及VoiceOver仍未验证。

1000页性能门同候选通过：4场景×6查询，每项冷/热各5样本，最慢中位数176.9/63.1ms，预算1000/100ms。见[完整报告](search-performance.json)；此前C₅同合同历史值为164.0/49.4ms，仅作历史对照。
