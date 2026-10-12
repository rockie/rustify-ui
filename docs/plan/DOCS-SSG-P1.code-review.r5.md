# DOCS-SSG-P1 · 代码评审 R5

验收判定：**需要修改**。R4 的 20 个旧项均在原问题范围内已解决；本轮新增 **1 项 P2（F-21）**：article 片段解析忽略了完整页面中真实存在的 body 锚点。修复效果 **未完全解决**，修复增量 `patch is incorrect`。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R5 | [DOCS-SSG-P1](DOCS-SSG-P1.md)，对象 `526c88c7-a4b3-4ea9-9193-371420b66d1d` | `540462a895a48385761f8db6e3271a714957d818..3069107cf4990e15db6ad3ddddddd520bcc86838`；代码修复 `f7a3e941cba709a6fd2669f0656414473a08ce55`；输入指纹 `f2ddf52d45d4dcd5a6048812d152e367b0133d450efa728f95be6de1da040ce8` | [R4](DOCS-SSG-P1.code-review.r4.md)，对象 `1996cbdf-4d81-4ba7-abac-d31ad78019a5` |

本轮对象 ID：`53350840-75ac-44ea-82e1-3b8072575e78`。模式为**修复效果复审**，覆盖 R4 的全部旧项与直接受修复影响的回归；未重新完整审查 M1–M8，M9 排除。

上轮所审 `1f0fc5ed21d7937a1297528ad38cdefe6b95f721..540462a895a48385761f8db6e3271a714957d818`，报告 sha256 `3c3c60669f478b5290299df69e2a23055ef4d3e8884e482facd89b1e2753ffa9`。本轮从该目标起算，当前核查版本为 `3069107cf4990e15db6ad3ddddddd520bcc86838`，未提交代码差异为空。已有处置见 [M3](../validation/docs-ssg-p1/M3.md#评审修复)和 [M8 的 R4 修复后复验](../validation/docs-ssg-p1/M8.md#r4-修复后复验)。本轮不改写原报告或原计划。

## 确认问题

### 上轮问题复审

F-01–F-14 沿用 [R1](DOCS-SSG-P1.code-review.md)（对象 `65ddeb43-1845-4bee-b70c-bbf9647aa8b7`）；F-15–F-17 沿用 [R2](DOCS-SSG-P1.code-review.r2.md)（对象 `ff41c49f-1b63-4b5b-85f2-4dbdfb96d73d`）；F-18/F-19 沿用 [R3](DOCS-SSG-P1.code-review.r3.md)（对象 `7b6a9d1a-088d-4213-be21-74348a929584`）；F-20 属于 R4。以下结果均在本轮 target 核查；测试日志和浏览器证据见后文。

| 来源 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01 | 输入真实路径出界；R4 已解决 | `12113ab` / M7 | **已解决** | `real_source/read_source` 授权边界未改；库与 CLI 工程外链接、读取安全及旧产物保全回归通过 | 无 |
| R1 / F-02 | 私有来源经别名/品牌发布；R4 已解决 | `12113ab` / M7 | **已解决** | 别名与真实路径的 private 校验保留；库/CLI 私有文件、品牌与 include 拒绝回归通过 | 无 |
| R1 / F-03 | 私有 Markdown 被当页面；R4 已解决 | `12113ab` / M7 | **已解决** | 遍历过滤未改，`private_markdown_is_never_a_page` 及 CLI 私有发布测试通过 | 无 |
| R1 / F-04 | 顶栏/社交入口缺失；R4 已解决 | `12113ab` / M5 | **已解决** | 阅读壳未改，docs-subpath 的页头、导航、首页/内容/404、窄屏与无脚本回归通过 | 原真机页头复走限制保留，本轮未重验 |
| R1 / F-05 | 仅首页操作引用的附件未发布；R4 已解决 | `12113ab` / M4 | **已解决** | 操作/页头资源图分支未改，库及 CLI 首页附件发布测试通过 | 无 |
| R1 / F-06 | CSS 资源缺校验/改写；R4 已解决 | `12113ab`、`a297dbd` / M7 | **已解决** | 生成前缀仍须有真实发布文件；CSS 生成与拒绝负例通过，docs-subpath SVG/class 资源回归通过 | 无；新增文档上下文条件另见 F-21 |
| R1 / F-07 | URL 后缀丢失；R4 已解决 | `12113ab` / M4 | **已解决** | URL 解析分支未改，CLI 附件与真实 query/fragment 后缀回归通过 | 无 |
| R1 / F-08 | 无换行开栏 panic；R4 已解决 | `12113ab` / M3 | **已解决** | frontmatter 保护未改，库/CLI 单独开栏诊断测试通过 | 无 |
| R1 / F-09 | 忽略实际 --config；R4 已解决 | `12113ab` / M4 | **已解决** | check/dev 指定配置回归通过；本轮隔离项目也显式使用各自配置 | 无 |
| R1 / F-10 | 空站成功替换旧产物；R4 已解决 | `12113ab` / M4 | **已解决** | no-pages 闸门未改，空站、全草稿及旧输出保全回归通过 | 无 |
| R1 / F-11 | 前置发布失败无 JSON；R4 已解决 | `12113ab` / M4 | **已解决** | `a_refused_output_is_reported_in_json_as_well`、输出归属和锁相关测试通过 | 无 |
| R1 / F-12 | 百分号标题中断脚本；R4 已解决 | `12113ab` / M5 | **已解决** | 交互脚本未改；docs-subpath 百分号标题、目录与窄屏菜单测试通过 | 无 |
| R1 / F-13 | 附件失效后丢恢复监听；R4 已解决 | `016110f` / M6 | **已解决** | snapshot 的安全恢复目标登记保留；CLI/dev 浏览器验证 content/static 附件仅恢复目标即可清错/更新，无关页稳定 | 无 |
| R1 / F-14 | 未输出脚注贡献幽灵 ID；R4 已解决 | `12113ab`、`a297dbd` / M3 | **已解决** | ID 收集接线改变后，未使用脚注、同页/跨页、合法脚注与重复引用回归均通过 | 无 |
| R2 / F-15 | 解码后拆问号误伤文件名；R4 已解决 | `a297dbd` / M4 | **已解决** | 先拆后缀再解码的分支未改，CLI 编码问号文件名及后缀回归通过 | 无 |
| R2 / F-16 | 数字实体后等号吞 URL 外分隔符；R4 已解决 | `016110f` / M7 | **已解决** | 实体/span 映射未改，库生成与 CLI CSS 字符串/实体测试通过 | 无；本轮未另重跑原生产 CSS 20 请求脚本 |
| R2 / F-17 | CSS 文档内 SVG 片段被拒；R4 已解决 | `a297dbd` / M7 | **已解决** | 普通 SVG 同页片段及缺目标拒绝测试通过，docs-subpath SVG/class 资源回归通过 | 本轮未重测 mask 像素 |
| R3 / F-18 | HTML template 内容误作文档锚点；R4 原条件已解决 | `016110f`、`f7a3e94` / M3 | **已解决** | 原同页/跨页、嵌套 template 正反例通过；20 份 production 产物确认 ghost/deep 不在文档树，holder/visible 可导航，错误 build 保全旧输出 | 无 |
| R3 / F-19 | CSS 资源扫描与 compiler 任意值解码不同；R4 已解决 | `016110f` / M7 | **已解决** | CSS 解码代码未改，库/CLI 解码与生成回归及 docs-dev/docs-subpath CSS 浏览器测试通过 | 无 |
| R4 / F-20 | SVG 同名元素误入 HTML template 计数 | `f7a3e94` / M3、M8 | **已解决** | 独立基线拒绝原三种输入，target 接受；两 base 的 20 份产物确认 namespace、HTML integration point、同页/跨页导航和幽灵拒绝均与浏览器一致 | 原条件关闭；新的上下文误拒见 F-21 |

### F-21（本轮新增）· [P2] 在完整页面上下文中收集锚点

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) **157–160**，target `3069107cf4990e15db6ad3ddddddd520bcc86838`，位于本次修复 diff。
- 问题：这里用 article 上下文的 `parse_fragment` 构树，但 SSR 把作者 HTML 直接写入已有 body 的完整文档，两种解析对重复 `<body>` 的处理不同。正文含 `<body id="target">` 与 `[Target](#target)` 时，片段解析忽略该 start tag，`Fragment.anchors` 丢失 target，check/build 报 `broken-anchor` 并拒绝原来可构建的站点；跨页引用也同样失败。实际浏览器会将缺少的 id 合并到页面现有 BODY 上，`document.getElementById('target')` 找到真实目标，基线链接能导航到 `#target`。此输入没有触发当前原始 HTML 禁止策略，公开作者合同也包含畸形 markup 的浏览器语义；仅设置 article 上下文不能保证片段构树与实际整页解析等价。
- 证据：[双版本 CLI 复现](../validation/docs-ssg-p1/code-review-r5/body-reproduction.json) 在 `/` 与 `/docs/` 下均为 base check/build 退出 0、target 退出 1，两个同页/跨页引用各产生一条 broken-anchor，旧输出仍保全；[浏览器对照](../validation/docs-ssg-p1/code-review-r5/structural-browser.json) 的 body-id 确认 base 和 target 的真实 BODY 均带 target，base 链接 hash 为 `#target`。查看 target 文档时只去掉被拒的引用，markup 保留；见 [基线截图](../validation/docs-ssg-p1/code-review-r5/body-id-base.png)。调用链为 [markdown.rs](../../crates/rustify-docs/src/markdown.rs) 1182 → [content.rs](../../crates/rustify-docs/src/content.rs) 1313–1320；完整页的 body 与直接插入位置见 [shell.rs](../../crates/rustify-docs/src/shell.rs) 324、370。契约依据为 [作者参考](../docs-ssg.md#raw-html) 292–307 和 [计划 §5.2](DOCS-SSG-P1.md#52-单一内容图可定位语法与安全检查) 的链接/原始 HTML 检查。[WHATWG 的 in body 规则](https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody)分别规定 fragment case 忽略 body token、完整文档把未存在的属性合并到现有 body，符合本轮浏览器结果。
- 修复与验证：让 ID 构树拥有与 SSR 完整页面一致的 body/article 上下文，并继续排除 HTML template 的独立片段；补这里的 body 输入同页/跨页 check/build 与真实浏览器 ID 对照，保留 SVG 同名元素、integration point、脚注和 HTML template 幽灵负例。关闭条件是两 base 的已保存输入成功生成，锚点集合与实际文档一致，原模板幽灵引用仍被拒绝。
- 置信度：**高**；独立基线快照编译、当前重建二进制、真实 Chromium 文档查询与标准规则共同确认。

## 待核实项与覆盖限制

没有缺证的候选 finding；F-21 已确认，不需要先补环境才能处理。

- 本轮浏览器为已安装 Playwright Chromium `153.0.8010.12`，375px 是仿真视口。未重验用户 Chrome、真实 Safari、手机、WebKit 或远端 CI；原真机页头复走限制保留。
- 本轮未重跑 SDK/GPU 全矩阵、7 示例、theme-export、500 页性能、LCP/CLS、千次保存或安装演练。新增依赖的历史最小工具链和性能证据见 M8；已核对其中 8 个代码/配置/测试指纹与当前完全一致，见 [匹配结果](../validation/docs-ssg-p1/code-review-r5/prior-evidence-match.json)，这些不记为本轮执行通过。
- M9 的最终 CLI 版本/SHA、文档、仓外消费者和 GitHub 同 SHA 安装仍属后续范围。Ready 和 8/9 不因本轮局部评审改写。

## 覆盖与验证

变更意图：修复 R4/F-20，将标签计数替换为真实树构建，保留 HTML template 片段边界并接受 foreign 同名元素。呈现路径为 **DOM 静态 HTML/SSR**，CLI 在宿主执行，没有 GPU 或跨区协作改动。全部审查、复现与裁定由当前主 agent 完成，未委派 reviewer。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；日期：2026-10-11，+08:00；原输入：`$review-code docs/plan/DOCS-SSG-P1.md`。
- 计划 sha256：`cdbc7b32c990900213c8ffe087bcf0539a18534e43e4335b7d1cd83841ae32da`。完成记录登记的 8 个提交均已解析并确认在当前分支可达；前七个由前轮覆盖，本轮复核旧项，`f7a3e941cba709a6fd2669f0656414473a08ce55` 纳入修复增量。完整 SHA、提交文件和工作树初始清单见 [scope.json](../validation/docs-ssg-p1/code-review-r5/scope.json)。
- 比较方式：固定 base→target 的累计净差异。增量包含一份代码修复及两份只改计划的提交 `1a43294fd6f60ed89aa3b6ae680a454f9e667782`、`3069107cf4990e15db6ad3ddddddd520bcc86838`，后两者排除代码 findings。计划历史中的其他未登记提交也均只改计划；无未登记代码提交需要扩范围。
- 8 个代码/配置/测试路径的 diff 已完整阅读：Cargo.lock、docs 两包 Cargo.toml、html_check/content/markdown、CLI tests/check、浏览器 dev.spec.ts。实施记录与生成证据用于核实处置，作为上下文读取。重点追踪树 sink、template 片段、最终锚点、同页/跨页校验、SSR 插入上下文、缓存版本与 CLI 版本；旧项按其相关回归核对。
- index 无 staged 差异；tracked/untracked 已枚举，现存未提交内容只有原报告、证据、SDLC 与经验记录。冻结 368 个相关输入，按排序的文件 sha256 映射生成整体指纹；作者参考在读取时补入，已与 target 树逐字节核对。[scope-end.json](../validation/docs-ssg-p1/code-review-r5/scope-end.json) 确认 HEAD 与固定输入未变。本轮新报告与证据不纳入原审查对象。
- 基线通过 `git archive` 提取到仓外临时目录，独立 CARGO_TARGET_DIR、offline/locked、docs-cli profile 构建。初次调用因临时 mise 配置未信任而失败；在 PATH 前置同一已安装 Rust 1.99.0 的真实 bin 后通过，未修改工具链或全局信任配置。未 checkout/reset/stash 用户工作树；目标验证对应当前 target 代码。
- 临时实验的内容与产物均在隔离目录；产品源码、计划和正式测试没有添加实验修复。二进制 hash、临时入口与实际命令见 [verification.json](../validation/docs-ssg-p1/code-review-r5/verification.json)、[commands.json](../validation/docs-ssg-p1/code-review-r5/commands.json)。

| 实际命令 / 步骤 | 本轮结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-docs` | 132 项通过，121 单测 + 11 生成集成；[日志](../validation/docs-ssg-p1/code-review-r5/library-test.log) |
| `mbx test --locked -p rustify-docs-cli` | 72 项通过；[日志](../validation/docs-ssg-p1/code-review-r5/cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 当前 CLI 重新构建通过；[日志](../validation/docs-ssg-p1/code-review-r5/cli-build.log) |
| docs 两包分别 Clippy `--all-targets -- -D warnings`；`cargo fmt --all -- --check` | 均退出 0；[库 Clippy](../validation/docs-ssg-p1/code-review-r5/docs-clippy.log)、[CLI Clippy](../validation/docs-ssg-p1/code-review-r5/cli-clippy.log)、[格式](../validation/docs-ssg-p1/code-review-r5/fmt.log) |
| CLI normal/build tree 与锁文件比较 | 18 个新增依赖包，另有 CLI 0.1.0→0.1.1；既有 registry 依赖版本均保留。图无 makepad-* 包；路径中的 bitflags/smallvec 是既有 generic patch，不当作 GPU 引擎；[图](../validation/docs-ssg-p1/code-review-r5/cli-tree.log)、[差异](../validation/docs-ssg-p1/code-review-r5/dependency-check.json) |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=docs-dev`，显式传当前 CLI | 19/19，包括 foreign template、integration point 错误恢复、原模板、真实附件恢复、CSS 与诊断；[日志](../validation/docs-ssg-p1/code-review-r5/docs-dev.log) |
| 同配置 `--project=docs-subpath`，与其他浏览器运行串行 | 24/24，包括页头/导航/404、主题、复制、分组、窄屏、无脚本、百分号标题、SVG/class 资源与 axe；[日志](../validation/docs-ssg-p1/code-review-r5/docs-subpath.log) |
| 隔离 base：`mbx build --offline --locked -p rustify-docs-cli --profile docs-cli` | 修正临时 PATH 后通过；[首轮环境失败](../validation/docs-ssg-p1/code-review-r5/baseline-metadata.log)、[构建成功](../validation/docs-ssg-p1/code-review-r5/baseline-build-retry.log) |
| [verify.cjs](../validation/docs-ssg-p1/code-review-r5/verify.cjs) 对比两版 CLI 并逐个 preview | 两 base 共 20 份 production 产物通过；原 F-20 三种输入由拒绝变为成功，namespace/ID/同页跨页导航正确，幽灵/缺失引用拒绝且旧输出逐字节保全；console/pageerror/失败请求为 0；[结果](../validation/docs-ssg-p1/code-review-r5/verification.json) |
| [structural-browser.cjs](../validation/docs-ssg-p1/code-review-r5/structural-browser.cjs) | 六种输入共 12 份产物的文档查询：body-id 确认新增误拒；html-id、noscript、table/foreign、MathML 自闭合、SVG CDATA 对照未发现新增问题；[结果](../validation/docs-ssg-p1/code-review-r5/structural-browser.json) |
| [reproduce-body.py](../validation/docs-ssg-p1/code-review-r5/reproduce-body.py)，传 base-bin / target-bin / 结果路径 | 根与子路径均确认 F-21，同页/跨页共 2 错，原成功产物保全；[结果](../validation/docs-ssg-p1/code-review-r5/body-reproduction.json) |

已查看本轮 SVG 自闭合页面与 body 基线截图；其余未查看的截图不计视觉证据。43 项标准浏览器测试与 204 项 Rust 测试全绿没有覆盖 F-21。复现脚本支持在新临时目录重建两个 body 项目；重新编译上述精确 SHA 后，从仓根执行 `python3 docs/validation/docs-ssg-p1/code-review-r5/reproduce-body.py <base-bin> <target-bin> <结果.json>` 即可再次取得同一失败条件。

## 计划符合性

| 本次承担范围 | 独立核查 | 状态 |
| --- | --- | --- |
| WORK-F2 / M3 / V-4：真实锚点、HTML/SVG 内容 | F-20 的 foreign 同名元素和 integration point 正反例通过；完整文档与片段解析仍在 body ID 上不同（F-21） | 存在缺陷 |
| M3/M6：缓存、check/build/dev 共享内容图与恢复 | writer 版本升为 7 并进入 FragmentKey；原模板与 dev 恢复回归通过 | 本轮所测已核实 |
| WORK-N4 / §4.2 / §8：新依赖与 CLI 版本 | CLI 独立图与 profile 构建通过，版本升为 0.1.1；历史安装/性能输入 8/8 指纹与当前匹配 | 本轮静态/构建已核实，安装/性能未重跑 |
| M1–M8 全期及真实环境 | 本轮是局部修复复审，不重签整期浏览器/性能/GPU 出口 | 未重新完整验收 |
| M9 | 没有纳入本轮修复差异 | 后续范围 |

## 总体结论与修复顺序

验收判定：**需要修改**。

旧问题 **20/20 已解决**，无“不再适用”项；本轮新增 **1 项 P2 F-21**，没有缺证候选。由于修复引入相关阻塞回归，修复效果 **未完全解决**，所声明的修复增量 `patch is incorrect`。此结论仅覆盖本轮增量及逐项旧问题复核，不是 M1–M8 完整重审、合并或发布授权。

先消除完整文档与片段的解析上下文差异，再验证已保存的根/子路径 body 锚点和原模板/SVG/脚注回归；当真实文档 ID 与分析结果一致、合法引用成功且幽灵引用仍被拒绝时再次独立复审。通过后按计划继续 M9。

下一步：`$apply-code-review docs/plan/DOCS-SSG-P1.code-review.r5.md`；修后 `$review-code docs/plan/DOCS-SSG-P1.md`。本轮只保存报告、证据和派生台账，不执行修复或改变原计划进度。
