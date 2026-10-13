# M3 · 定制静态页、布局插槽与主题

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-13T20:43:09+08:00 · 状态：进行中，浏览器与阶段出口待验。

基线：main / dirty@2d6b0a73ad887769c0bad209acabd47382659df5。采用 DOM 静态页和局部交互岛；无新增 GPU 路径、线上 SSR 或基础设施。M3–M5 按计划保留待提交，候选冻结归 M6。

## 当前成果与证据

| 范围 | 实际验证 | 证据 |
| --- | --- | --- |
| 配置、注册页与七插槽 | `theme.document` 与 `nav.routes`；注册 HTML 进入同一页图，模板内容不贡献页面引用；配置新例先红后绿 | [配置红](m3/config-red.log)、[配置绿 13/13](m3/config-green.log)、[注册面红](m3/surfaces-red.log)、[模板引用红](m3/template-urls-red.log) |
| 内容与静态主题上下文 | 完整 pipeline 原生单元 174/174；含新增两模式、字体、响应式主题上下文及清理测试，SVG use、foreign title、主题输入跟踪与禁止发布反例 | [174/174](m3/pipeline-unit-r2.log)；此前 [164/164](m3/pipeline-unit-r1.log)、[SVG 红](m3/svg-use-red.log)、[foreign title 红](m3/foreign-title-red.log)、[主题源红](m3/theme-source-red.log) |
| 最终文档与输出空间 | 新集成 10/10：页与插槽引用互通、实际 DOM IDs、模板/textarea/foreign integration、标题及文件目录冲突、静态目录内主题不发布 | [初跑红](m3/final-generation-red.log)、[绿](m3/final-generation-green-r1.log)。初跑 SVG 缺失目标已被内容阶段 `broken-anchor` 拒绝，修正测试断言；其余五类为真实未实现输出闸反例 |
| CLI 主题与字体 | 14/14：双模式、只打包引用字面与既有 glyph 回退、许可/字体摘要、离线资源、路径边界、未知 fallback、非法与超限文档 | [CLI 绿](m3/theme-cli-green.log)；[首次编译失败](m3/theme-cli-r1.log)、[测试措辞错误](m3/theme-cli-r2.log)保留。后续阅读面 radius 映射和整体管线仍须阶段复验 |
| 同版资产与许可来源 | 七字体 31,057,076 bytes；15 个内置字体/许可/源码文件合计 33,361,104 bytes。与 SDK 七项摘要配对；字体原字节未改 | [实际资产摘要](m3/font-assets.json)、[SFNT 版权/版本原文](m3/font-metadata.json)、仓根 `sources.lock.json` 的 `docs-static-fonts` 条目 |

Liberation Mono 实际为 Version 1.04、2007 Ascender 字面，使用 GPL v2 与原发行版字体例外。原仓“所有文本字体 OFL”说明已在字体说明和两份 compatibility 文档同步纠正。完整许可和对应 SFD/build 源码取自官方 1.04.93.devel 发行材料；仅引用此字面时随静态字体输出源码归档。原 Makepad 导入清单的收到时摘要保留，后续修订按原台账记录漂移。

## 待完成与恢复入口

- CLI 已连接 Page/Slot 批量 SSR、嵌套 `RenderContext::demo`、主题 metadata 与文章生成；真实组件 fixture 的 check/build、两目标、浏览器尚待验。
- SSR 正在将同一批次的主题解析结果跨 Owner 复用；新代码尚不计验证通过。
- 浏览器需完成 V-6/V-7：插槽与导航、no-script、实际字体请求/字节、light/dark/system 首帧与浮层、隐藏状态 class，以及受影响的 V-5 故障回归。未在浏览器中验证本里程碑。
- 复核 docs/themes.md、隔离 features/clippy、sources/css/catalog；完成 bounded finish review 与设计同步。
- 新主题和插槽路径完成后复测 release 与开发档时延代理。M2 增量中位 1944.6ms 仅有 55.4ms 余量，不能沿用旧结果关闭新路径。
- 用户目前无法安排五位首次用户与两类实体手机；M6 保留验收缺口。候选分发、公开 R/C 和 Pages 授权尚未进入发布步骤。
