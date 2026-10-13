# M3 · 定制静态页、布局插槽与主题

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-13T22:33:32+08:00 · 状态：进行中，功能/字体/阶段检查已过，开发档代理超预算待修。首个检查点原文保留在 [checkpoint-2043.md](m3/checkpoint-2043.md)。

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

## 新增验证与性能出口

| 范围 | 实际结果 | 证据 |
| --- | --- | --- |
| 真实页面/插槽/主题浏览器 | subpath 定制用例 12/12；meta-only 静态主机定制12 + 岛故障/恢复24，共36/36。覆盖导航、七slot、深404、实际SSR节点/源码、字体字节、四种首帧、mode/context/浮层、隐藏分支图片与样式、no-script/375/200%布局缩放模拟 | [subpath](m3/custom-subpath-r3/results.json)、[meta](m3/custom-islands-meta-r1/results.json)。字体五文件实际响应30,767,116 bytes；[字体/缩放摘要](m3/custom-subpath-r3/font-and-zoom-summary.json) |
| 浏览器失败处置 | r1 的 19MB文楷响应被Chromium默认inspector缓存驱逐；改为同一响应的独立CDP采集。r2只剩预期深404控制台消息和错误假定TextField16px；限定记录实际404URL并依据14px→28 physical px验证2x布局缩放 | [r1](m3/custom-subpath-r1.log)、[r2](m3/custom-subpath-r2.log)、[r3](m3/custom-subpath-r3.log)。未忽略非预期网络/CSP错误，未用另一次fetch替代字体实际响应 |
| 候选资源图 | 源码完整class的隐藏图片进入统一资源图，CSS仅重写URL值、保留原选择器；四内容反例和两生成反例先红后绿。查询/片段、引号、转义下划线与空格保留 | [内容红](m3/candidate-urls-red.log)、[内容绿4/4](m3/candidate-urls-green.log)、[生成红](m3/candidate-css-red-r2.log)、[生成绿并入12/12](m3/final-generation-green-r2.log) |
| Rust与隔离 | pipeline 181 unit；16旧generate +12新registered +1metadata +19registry，共229通过。CLI64 unit +67 integration，共131通过，另1performance默认ignored已单测量。site-ssr独立9+1+19，hydrate wasm32 check；SDK字体3例、docs/pipeline/site-ssr/CLI strict clippy通过 | [181 unit](m3/docs-full-r1.log)、[48 integration](m3/docs-integration-r2.log)、[CLI前段](m3/cli-full-r1.log)、[真实integration修复1/1](m3/components-cli-integration-r2.log)、[其余38](m3/cli-remaining-r1.log)、[site-ssr](m3/site-ssr-r1.log)、[hydrate](m3/hydrate-check-r1.log)、[SDK fonts](m3/sdk-fonts-r1.log)、各clippy日志 |
| 旧测试适配 | 真实components harness需同步复制新增static/theme输入；旧Readable-marker测试原先用整段替换抹掉有效headings，引发新终页闸正确拒绝TOC断链，改为追加可读文本保留原文章 | [旧CLI失败](m3/cli-full-r1.log)、[旧generate失败](m3/docs-full-r1.log)、上述修复后集成日志；未削弱最终引用验证 |
| 来源、CSS、资源组 | sources验证15字体/许可/源码资产无漂移；SDK候选新增5条，3783/53,873bytes，基础CSS28,814bytes无漂移，CATALOG24无漂移；完整features含独立SSR/hydrate无private pipeline与真实CLI完整6文件岛组通过 | [sources](m3/sources-verify-r1.log)、[CSS初次漂移](m3/css-check-r1.log)、[重生成检查](m3/css-check-r2.log)、[catalog](m3/catalog-check-r1.log)、[features](m3/features-r1.log) |
| Git安装/禁网字体 | 临时Rtest97d0d77、Ctesta71b6b8，库文件逐字节相同；git安装CLI0.1.6后，纯内容作者目录无Makepad、Cargo入口故意不可用、禁网check/build均零错误，五字体逐字节一致、无wasm或theme.json发布 | [安装](m3/font-git-install-r2.log)、[check](m3/font-git-check-r1.json)、[build](m3/font-git-build-r1.json)、[身份与字体](m3/font-git-summary-r1.json)。首次fileGit未缓存，offline安装拒绝；[原日志](m3/font-git-install-r1.log)保留。该安装排练不是M6冻结或GitHub交付 |
| 体积 | 七字体31,057,076bytes，连许可/首选源码共33,361,104bytes；相同docs-cli档已安装CLI从M2的23,527,680变为57,594,608bytes，增量34,066,928bytes，含主题/管线代码变化 | [体积](m3/cli-font-footprint.json)、[资产](m3/font-assets.json)。主题只发布/请求引用五字体30,767,116bytes，未擅自另设预算 |
| 设计收口 | 一次material修正将测试图形空框改为三色填充条并补可见caption；固定第二批七图补404与缩放，复核ship。设计规范和themes文档同步 | [finish review](m3/finish-review.md)、[设计同步](m3/design-documentation.md)。DPR2/640为layout zoom emulation；原生zoom、真人/真机边界保留 |
| release代理 | 五次有效；FCP→28输入中位255.9ms，余量744.1ms，身份与最大框位移0全部通过；同Chromium153.0.8010.12、CPU1x/禁缓存/无节流条件 | [release](m3/release-proxy-r1.json)。M2中位371.9ms；正式最大页仍归M6 |
| 开发档代理 | 冷轮2304.5ms单列；增量2381.4/2209.8/2385.6/2369.4/2198.9ms，中位2369.4ms，超≤2s预算369.4ms。全部有效，源/实际HTTP HTML与wasm同代、时钟/原输入与新行为正确；dev wasm22,465,759bytes。native编译后耗时1691–1874ms，是下一步诊断重点 | [失败代理](m3/compile-proxy-r1.json)、[native](m3/compile-proxy-native-r1.log)、[原始Cargo历史](m3/compile-proxy-raw-r1/)、[分段初值](m3/compile-proxy-raw-r1/stage-summary.json)、[恢复源指纹](m3/perf-baseline-sources.json)。不因功能通过记M3完成 |

## 待完成与恢复入口

- 继续autoresearch/Hillclimb：固定五次增量、≤2s阈值、源/产物/节点/输入/零位移底线；先诊断native编译后阶段，按一次机制改动、同条件重测，保留失败与恢复点。不得删引用/资源/同代检查或降低指标。未闭合预算前M3仍进行中，正式dev/M6测量未验。
- 当前M3代码工作树尚未提交；临时Git字体排练早于SDK候选重生成，字体与主题内容相同，仅用于其限定字体门，不能冒作最终候选。R/C模板/README冻结仍在M6。
- 用户目前无法安排五位首次用户与两类实体手机；M6 保留验收缺口。候选分发、公开 R/C 和 Pages 授权尚未进入发布步骤。
