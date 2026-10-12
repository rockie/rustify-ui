# DOCS-SSG-P1 · 代码评审 R2

验收判定：**需要修改**。修复效果 **未完全解决**；修复增量 `patch is incorrect`。R1 的 14 项中 **11 项已解决、3 项部分解决**（F-06、F-13、F-14）；本轮新增 **3 项 P2**（F-15–F-17）。M9 未实施，继续排除。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [DOCS-SSG-P1](DOCS-SSG-P1.md) | `72418b31c9acd9a1e893950fc45505fed694f05c..bfbee6d3342b69e062ed513e05e7f812373124bc`；工作树无代码增量，输入指纹 `00463efe17844a3ed64741c46ce4051d2791deeb0631fcd9acb7b9d58a5f7a20` | [R1](DOCS-SSG-P1.code-review.md)，评审对象 `65ddeb43-1845-4bee-b70c-bbf9647aa8b7` |

- 日期：2026-10-11（Asia/Shanghai）；本轮评审对象 ID：`ff41c49f-1b63-4b5b-85f2-4dbdfb96d73d`。
- 上轮基线：`91e81b81ecee53153b9d5df7b74fd843bfafb8c8..72418b31c9acd9a1e893950fc45505fed694f05c`；上轮报告 SHA-256：`627b456531d17b70b42405bf2ec699dab7c3cfd16e0501e6ded0fc314e7af3d9`。
- 修复提交：`12113aba10eea8f4454fe540aaab083e4b5a4042`。当前核查版本为 HEAD `bfbee6d3342b69e062ed513e05e7f812373124bc`；其代码与修复提交相同，后者之后仅回写计划。
- 计划版本为当前 HEAD 中的文件，SHA-256：`7ff0e593612202b15b50cc15f7e3a487bd976eab9a192c4432b8ac0d29fe3d9c`。本轮复核全部旧项及修复的直接回归，**未重新完整评审 M1–M8 原累计变更**。
- 处置材料：[M3](../validation/docs-ssg-p1/M3.md#评审修复)、[M4](../validation/docs-ssg-p1/M4.md#评审修复)、[M5](../validation/docs-ssg-p1/M5.md#评审修复)、[M6](../validation/docs-ssg-p1/M6.md#评审修复)、[M7](../validation/docs-ssg-p1/M7.md#评审修复)、[M8 复验](../validation/docs-ssg-p1/M8.md#评审修复后复验)。其中“已修复并验证”是作者处置声明，以下独立复核结果优先。

## 确认问题

### 上轮问题复审

所有来源 ID 均属于上表的 R1 报告；修后版本均为 `12113aba10eea8f4454fe540aaab083e4b5a4042`，本轮在当前 HEAD 核查。共同复现见 [R1 CLI 重跑](../validation/docs-ssg-p1/code-review-r2/r1-reproductions.json)、[R1 浏览器重跑](../validation/docs-ssg-p1/code-review-r2/r1-browser-results.json)；旧项不作为本轮新增 finding。

| 来源 / ID | 上轮问题 | 已有处置 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01 | P1，输入真实路径出界 | M7：统一真实来源校验，生成器只用已读字节 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 189–211、310–341；`external-static-root`、`external-image` 均退出 1，无外部标记；库与 CLI 覆盖中间目录链接、工程内合法链接及旧输出保全 | 无 |
| R1 / F-02 | P1，私有来源经别名或品牌资源发布 | M7：引用名与真实来源都校验 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 189–211、1115–1143；`private-symlink`、`private-logo`、`private-include-alias` 全部拒绝，无合成私有标记 | 无 |
| R1 / F-03 | P1，私有 Markdown 被当页面 | M7：遍历跳过私有路径 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 279–284；`private-markdown` 不再发布标记；`private_markdown_is_never_a_page` 和 CLI 用例覆盖 `.env.md`、`.env/notes.md`、导航和站点文件 | 无 |
| R1 / F-04 | P2，顶栏与社交入口未渲染 | M5：向壳传 `HeaderLink` | **已解决** | [shell.rs](../../crates/rustify-docs/src/shell.rs) 440–465；原浏览器两个入口均为 1。三个部署项目均验证首页/内容/404、当前分区、320/375px、无页面横滚；无脚本路径通过 | 无；真机复走未在本轮执行 |
| R1 / F-05 | P2，仅首页操作引用的附件未发布 | M4：操作与配置链接登记资源 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 1061–1065、1105–1109；下载返回 200 且含正确合成内容；库及 CLI 覆盖操作/页头单独引用附件 | 无 |
| R1 / F-06 | P2，CSS 资源未校验与改写 | M7：ClassImage 槽及最终 CSS 检查 | **部分解决** | 原跨源资源被定位拒绝，普通本地背景指纹化并加载 200；但 `css-unwatched-guard` 仍零错误发布缺失背景，浏览器请求 404，详见下文 | 校验实际 CSS URL 与发布文件集合，不只检查前缀 |
| R1 / F-07 | P2，资源 URL 后缀丢失 | M4：正文、srcset、操作和页头保留后缀 | **已解决** | [generation.rs](../../crates/rustify-docs/src/generation.rs) 150–154、202–215、258–264、292–297；原 SVG 发布时已为 40×40，`#mark` 保留；`suffix-content` 保留 `?download=1#page=2` | 独立新增回归 F-15 另行处理，不将后缀修复误记未生效 |
| R1 / F-08 | P2，无换行开栏触发 panic | M3：无下一行时报未闭合 | **已解决** | [frontmatter.rs](../../crates/rustify-docs/src/frontmatter.rs) 29–42；`---`、`---\r` 用例退出 1，完整 JSON、位置 1:1，CRLF 正向测试通过 | 无 |
| R1 / F-09 | P2，忽略实际 `--config` 文件 | M4：load_file，dev/preview/诊断统一 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 100–130；[main.rs](../../crates/rustify-docs-cli/src/main.rs) 250–251、570–571；指定有效 custom、损坏/缺失默认文件时正常；dev 实际监听 custom 的集成测试通过 | 无 |
| R1 / F-10 | P2，空站成功替换旧产物 | M4：分析阶段 no-pages 闸门 | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 908–935；缺目录/空目录/全草稿库用例通过；CLI 删除至全草稿时拒绝且原输出逐字节不变 | 无 |
| R1 / F-11 | P2，发布前置失败不输出 JSON | M4：拒绝路径走 report | **已解决** | [main.rs](../../crates/rustify-docs-cli/src/main.rs) 408–425；输出重叠、preview 持锁的 CLI 测试通过；原 `json-output-error` 返回一个完整失败 JSON | 无 |
| R1 / F-12 | P2，百分号标题中断增强脚本 | M5：fragment 编码与容错解码 | **已解决** | [site.js](../../crates/rustify-docs/assets/site.js) 219–233；原浏览器无 pageerror、目录高亮 2、Escape 关闭菜单；`100%`、类似 `%20` 的 ID 与中文由 escape/shell 测试覆盖 | 无 |
| R1 / F-13 | P2，快照外资源保存不刷新 | M6：referenced 与 snapshot.linked 加入 filter | **部分解决** | 普通 `../images/mark.svg` 已刷新；工程内合法 symlink 的真实目标单独保存仍不刷新，详见下文 | 收集所有被读取路径的真实依赖，同时保留别名与失败引用 |
| R1 / F-14 | P2，脚注前缀被整体放行 | M3：Fragment.anchors 集合校验 | **部分解决** | 原不存在的 `fn-never-defined` 已拒绝；未渲染的脚注定义仍贡献 phantom `fnref-b`，构建成功但无对应 ID，详见下文 | 以最终写入 HTML 的 ID 判定锚点 |

### F-06（持续问题）· [P2] 用真实发布集合校验最终 CSS 资源

- 定位：[generation.rs](../../crates/rustify-docs/src/generation.rs) 651–654，当前 target。
- 问题：原小写 `bg-[url(...)]` 路径已修好，但 `[background-image:URL(/docs/assets/missing.svg)]` 中大写函数绕过原始 class 的小写子串扫描，经编译器规范化成 `url(...)` 后，又因 `/docs/assets/` 前缀被最终闸门放行。文件根本没有发布，`build --format json` 仍退出 0，读者背景请求返回 404；这仍是 R1 F-06 的资源图与 CSS 校验缺口，未算新增问题。
- 证据：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 174–189；[CLI 结果](../validation/docs-ssg-p1/code-review-r2/boundaries.json) 的 `css-unwatched-guard`、[浏览器结果](../validation/docs-ssg-p1/code-review-r2/extra-browser.json)。[基线对照](../validation/docs-ssg-p1/code-review-r2/baseline-comparison.json) 表明该场景前后都成功构建；[计划 §5.2](DOCS-SSG-P1.md#52-单一内容图可定位语法与安全检查) 要求 CSS 资源经过校验。
- 修复与验证：按 CSS 语义识别资源，把本地引用接入资源图；最终检查其路径确实存在于该代发布文件集合。覆盖大小写函数、缺失但前缀合法的文件及正常已发布文件，缺失输入必须诊断失败，已有产物保全。
- 置信度：高，实际 CLI 与浏览器 404 一致。

### F-13（持续问题）· [P2] 监听快照外符号链接的实际目标

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 586–595；支持位置 [dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 119–133，当前 target。
- 问题：`images/mark.svg -> brand/real.svg` 完全留在工程内，正文 `../images/mark.svg` 正常读取发布；`Sources` 却只登记别名，`read_source` 的规范化真实路径只用于安全校验后丢弃，而 `Snapshot.linked` 仅覆盖 content/static 遍历。直接保存 `brand/real.svg` 不匹配 filter，2 秒后重新加载仍是旧指纹、旧红色字节；随后保存正文才换成蓝色。这是原 F-13 在合法链接上的未关闭范围。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 202–209；[真实浏览器结果](../validation/docs-ssg-p1/code-review-r2/extra-browser.json) 的 `watch-symlink`：资源目标保存前后均为 `mark.106eb81194.svg`，正文保存后才为 `mark.c763668ac8.svg`。普通资源原复现现在通过，故判“部分解决”。[计划 §5.5](DOCS-SSG-P1.md#55-dev-失效错误恢复与滚动) 是本期依赖刷新合同。
- 修复与验证：登记快照外资源、品牌文件及 include 的真实来源和别名，失效引用也保留可恢复的监听路径。只保存合法链接的真实目标就应构建新代、刷新引用页；无关页保持稳定，覆盖原子替换/删除恢复。
- 置信度：高，资源保存与随后正文保存构成直接反证。

### F-14（持续问题）· [P2] 不把未渲染脚注缓冲中的 ID 当作有效锚点

- 定位：[markdown.rs](../../crates/rustify-docs/src/markdown.rs) 961–964；支持位置 1161–1171，当前 target。
- 问题：`footnote_reference` 在解析时就把 ID 加入全局 `anchors`，即使引用位于最终没有输出的脚注定义缓冲中。正文写 `[Missing reference](#fnref-b)`，另有未被正文引用的 `[^unused]: Never shown, but has a note[^b].` 和 `[^b]` 定义时，构建退出 0、零错误，发布链接却没有 `id="fnref-b"`。前缀整体放行虽已删除，集合仍不等于真实生成 ID，故 F-14 尚未完全关闭。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 1264–1268；[前后版本结果](../validation/docs-ssg-p1/code-review-r2/unused-footnote.json) 中 `linked=true`、`target_exists=false`，只有 `fn-b`；[目标 HTML](../validation/docs-ssg-p1/code-review-r2/target-output/unused-footnote/index.html)。普通缺失脚注已诊断，合法同页/跨页脚注测试通过；本场景基线也失败，按旧问题记录。
- 修复与验证：锚点随真正合并进正文的缓冲登记，或从最终生成片段取得实际 ID 集合；保留合法脚注和重复引用。覆盖未使用定义内嵌引用、其跨页链接及实际已输出的引用，缺失目标必须报 `broken-anchor`。
- 置信度：高，直接比较发布 HTML 与链接目标。

### F-15（本轮新增）· [P2] 先拆查询部分，再解码文件名

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 1307–1309，target，位于修复 diff。
- 问题：新增 `file_part` 在 `percent_decode` 之后拆 `?`，把文件名中合法的编码问号误当查询分隔符。工程内 `content/manual?v.pdf` 的链接 `manual%3Fv.pdf` 在基线可正常构建，修后却寻找 `content/manual` 并报 `broken-link`；static 绝对链接也同样失败。这会阻断包含这类合法附件文件名的原有站点，独立于已生效的 F-07 后缀保留修复。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 1320、1355；[基线对照](../validation/docs-ssg-p1/code-review-r2/baseline-comparison.json) 的 `encoded-question-content/static` 均为基线退出 0、目标退出 1。基线二进制实际从 `72418b31c9acd9a1e893950fc45505fed694f05c` 提取源码构建，未以当前 checkout 冒充历史版本。
- 修复与验证：在原始 URL 上拆出 path/query/fragment，再只解码 path 用于查找文件；改写时保留真正的后缀。覆盖 `%3F` 文件名、普通查询、两者同时存在，以及内容附件/static/操作/页头入口。
- 置信度：高，有对应版本的实际二进制对照。

### F-16（本轮新增）· [P2] 按 CSS 语义提取资源，跳过字符串里的 url 文本

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 177–181，target，位于修复 diff。
- 问题：新增扫描把 class 属性中任何 `url(` 子串都当加载函数。合法的 `before:content-['url(foo)']` 只想显示文字，基线 CSS 为 `--tw-content:"url(foo)"`、构建成功；修后却把 `foo` 加入资源解析并报 `missing-image`，导致合法站点不能构建。最终 `style::urls` 也会在 CSS 字符串里做同类误判，所以只在入口增加一个特例不足以关闭根因。
- 证据：[style.rs](../../crates/rustify-docs/src/style.rs) 273–290；[基线对照](../validation/docs-ssg-p1/code-review-r2/baseline-comparison.json) 的 `css-content-text`；[实际基线 CSS](../validation/docs-ssg-p1/code-review-r2/baseline-output/css-content-text/site.css)。同一原始文本扫描还未按 HTML 属性的实体解码语义取值：`css-entity` 的合法 `&#47;bg.svg` 被当作文件名文字拒绝，见同一对照。
- 修复与验证：解析解码后的属性及实际任意值表达式中的 CSS URL token，并维护原始源码范围；最终 CSS 检查也须区分字符串与加载函数。覆盖 content 字符串中的 url 文本、HTML 实体、实际背景/蒙版资源和多个引用，不改写可见文字。
- 置信度：高，基线与目标实际构建结果及基线 CSS 一致。

### F-17（本轮新增）· [P2] 保留 CSS 对当前文档 SVG 定义的片段引用

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 314–317，target，位于修复 diff。
- 问题：新增 ClassImage 槽把所有非 data URL 都当独立文件，合法 `[mask:url(#m)]` 因而按空文件路径查找并报 `missing-image`。当同一页 SVG 已有 `<mask id="m">` 时，`#m` 是当前文档定义，不是缺失图片；基线成功生成 `mask:url(#m)`，修后却拒绝整个构建。即使入口允许它，新增最终 CSS 前缀闸门仍会拒绝 `#m`，因此两处需共同保留这种引用。
- 证据：[generation.rs](../../crates/rustify-docs/src/generation.rs) 651–654；[基线对照](../validation/docs-ssg-p1/code-review-r2/baseline-comparison.json) 的 `css-fragment-only` 为退出 0 → 1；[完整合成输入](../validation/docs-ssg-p1/code-review-r2/fixtures/css-fragment-only/content/index.md)、[基线 CSS](../validation/docs-ssg-p1/code-review-r2/baseline-output/css-fragment-only/site.css)。该引用指向已发布页面内部的 SVG 定义，不产生跨源请求。
- 修复与验证：把纯片段 URL 作为文档内引用处理，与“文件路径加 fragment”的资源分开解析；最终 CSS 允许已确认的文档内引用。覆盖同页 mask/filter、外部 SVG 指纹文件带 fragment，以及不存在的目标。
- 置信度：高，已构建两个版本，实际输出与错误条件一致。

## 待核实与覆盖限制

本轮六个未关闭问题均已确认，无需先补环境才能修复。未将缺证线索降为低优先级 finding。

- R1 未重验的 GPU/DOM 全矩阵、7 示例、theme-export、冻结 LCP/CLS、500 页性能、千次保存、远端 CI，本轮也未重签；相关 SDK、组件、依赖锁和 GPU 启动代码在修复增量中未变。
- 本轮实际浏览器为已安装的 Playwright Chromium `153.0.8010.12`。未重跑 WebKit，未在真实 Safari、实体手机或用户安装的 Chrome 中验证新页头；此前作者的设备走查只作为历史证据链接，不记本轮通过。
- M9 的安装 SHA/独立 CLI 版本、仓外安装、GitHub 可达性与推送授权仍属于后续实施，不作为本轮遗漏。Firefox 已由计划移出本期，不追加要求。
- 历史日志/截图和 SDLC 进度文档不逐行重审；本轮结论针对修复代码、全部旧项及其直接回归，不能扩大成整仓质量保证。

## 覆盖与验证

变更意图是关闭 R1 的输入安全、资源发布、页头、诊断及依赖刷新问题。文档站采用 **DOM 静态 HTML/SSR 与原生 CLI**；本轮未触及 GPU 呈现路径。单 reviewer，未委派 agent，未修复代码、改计划、提交或访问外部业务数据。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；原输入：`$review-code docs/plan/DOCS-SSG-P1.md`，按已有报告默认采用修复效果复审。
- 计划登记提交 `179c5ec590d6121efc8c52526e402f7118d1bb07`、`9442f5dc55d76aa5b0b4badc7be74a3f15f39403`、`1d239ffd84b951b184cefe1738cf9eb55e579841`、`72418b31c9acd9a1e893950fc45505fed694f05c` 已被 R1 覆盖，本轮仅核对旧项涉及的当前实现；新增 `12113aba10eea8f4454fe540aaab083e4b5a4042` 纳入修复 diff。
- 逐个验证提交在当前分支；`39978d1`、`d0e6913`、`bbbfb44`、`851d964`、`bfbee6d` 均只改本计划文件，作为进度回写排除。无未登记代码提交。
- 净 diff 60 个文件，其中产品代码/测试/fixture 24 个，另核查 `DESIGN.docs-ssg.md` 和 `docs/docs-ssg.md` 的相关变化。已完整读取上述修复代码 diff，追踪输入 → 分析/缓存 → 资源 → SSR/样式 → 发布/诊断 → preview/dev watcher，并读取必要上下游。
- 初始工作树仅 SDLC 文件、R1 报告和其证据未提交，index 为空，无代码增量。[scope.json](../validation/docs-ssg-p1/code-review-r2/scope.json) 固定 161 个输入/上下文文件；指纹算法为排序后的 `path + NUL + SHA-256 hex + newline` 再做 SHA-256。保存前重核这些文件及 HEAD，均未变；本轮报告、证据和随后台账写入不进入审查对象。
- 当前测试使用当前 HEAD 的源码；当前 `docs-cli` 二进制重建后才运行复现/浏览器。基线从 `72418b31c9acd9a1e893950fc45505fed694f05c` 的根清单及依赖源码隔离提取，使用独立 `CARGO_TARGET_DIR` 构建，见 [baseline.json](../validation/docs-ssg-p1/code-review-r2/baseline.json)；没有 checkout/reset/stash 用户工作树。
- 全部附加输入是临时工程中的合成内容。真实浏览器检查 pageerror、console、CSP 与失败请求，保留截图，并实际查看了[页头](../validation/docs-ssg-p1/code-review-r2/screenshots/header-links.png)和[窄屏菜单](../validation/docs-ssg-p1/code-review-r2/screenshots/toc-percent.png)。非 Portal App，无宿主入口要求。UI 使用 impeccable 的已有 Read brief 与实际源码判断；没有以 Theme Studio 的 DESIGN 覆盖文档主题。

| 本轮实际命令 / 检查 | 结果 | 证据 |
| --- | --- | --- |
| `mbx test --locked -p rustify-docs` | 108 单测 + 9 集成测试通过，doc-test 0 | [库日志](../validation/docs-ssg-p1/code-review-r2/library-test.log) |
| `mbx test --locked -p rustify-docs-cli` | 65 项通过 | [CLI 日志](../validation/docs-ssg-p1/code-review-r2/cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 退出 0，随后复现使用该二进制 | [构建](../validation/docs-ssg-p1/code-review-r2/cli-build.log) |
| 两包各 `mbx clippy --locked -p <package> --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 三项均退出 0 | [库](../validation/docs-ssg-p1/code-review-r2/library-clippy.log)、[CLI](../validation/docs-ssg-p1/code-review-r2/cli-clippy.log)、[fmt](../validation/docs-ssg-p1/code-review-r2/fmt.log) |
| R1 修复记录的 `reproduce.py`，隔离目录，当前 CLI | 原 18 个场景全部重跑；具体发布与错误字段独立核对 | [结果](../validation/docs-ssg-p1/code-review-r2/r1-reproductions.json) |
| R1 修复记录的 `browser-cases.cjs`，当前 CLI | 6 个站点实际服务，跨源 CSS 构建拒绝；原导航、下载、背景、SVG、目录、普通资源刷新均改善 | [结果](../validation/docs-ssg-p1/code-review-r2/r1-browser-results.json) |
| `node_modules/.bin/playwright test -c playwright.docs.config.ts --project=docs-root --reporter=list --output <临时目录>`；随后单独 `docs-subpath`、`docs-meta` | 各 24/24；含无脚本、响应式、导航、AA/axe、CSP 与 HTTP 爬取 | [根路径](../validation/docs-ssg-p1/code-review-r2/docs-root.log)、[子路径](../validation/docs-ssg-p1/code-review-r2/docs-subpath.log)、[meta](../validation/docs-ssg-p1/code-review-r2/docs-meta.log) |
| 同命令单独 `--project=docs-dev` | 10/10；last-good、错误面板、panic 恢复等通过 | [dev](../validation/docs-ssg-p1/code-review-r2/docs-dev.log) |
| 隔离提取基线后 `mbx build --locked --manifest-path <baseline>/Cargo.toml -p rustify-docs-cli --profile docs-cli` | 退出 0；对同输入比较两个版本 | [构建](../validation/docs-ssg-p1/code-review-r2/baseline-build.log)、[对照](../validation/docs-ssg-p1/code-review-r2/baseline-comparison.json) |
| 额外边界 fixture、unused-footnote；随后落盘 `replay.py` 在新临时目录重放两个二进制 | 11 个边界场景各测两个版本；确认三项新增回归及旧 F-06/F-14 未闭合，正向查询/百分号案例保留 | [边界](../validation/docs-ssg-p1/code-review-r2/boundaries.json)、[脚注](../validation/docs-ssg-p1/code-review-r2/unused-footnote.json)、[重放结果](../validation/docs-ssg-p1/code-review-r2/replay-results.json)、[配方](../validation/docs-ssg-p1/code-review-r2/replay.py) |
| 额外 `extra-browser.cjs` | 确认 F-13 实际目标保存不刷新和 F-06 404；服务随后关闭 | [结果](../validation/docs-ssg-p1/code-review-r2/extra-browser.json)、[脚本](../validation/docs-ssg-p1/code-review-r2/extra-browser.cjs) |

三个静态部署项目与 dev 串行运行，共 **82 项浏览器测试通过**。它们没有覆盖上述新增输入与剩余边界，不抵消复现出的六个问题。原 SVG 配方随后人为再追加 `#mark` 的数据是浏览器内实验，关闭 F-07 只使用其 `asPublished` 原样 40×40 的结果。

### 可复现配方

从仓库根运行；`replay.py` 不向实现目录添加测试，也不修改保存的 fixture：

```sh
mbx build --locked -p rustify-docs-cli --profile docs-cli
python3 docs/validation/docs-ssg-p1/code-review-r2/replay.py
node docs/validation/docs-ssg-p1/code-review-r2/extra-browser.cjs <上一命令输出的 work>
```

传入 `--baseline <上轮基线构建出的二进制>` 可同时比较两个版本。基线提取范围与实际构建命令保存在 `baseline.json`；新运行建立新临时目录，不能复用已运行过 `extra-browser.cjs` 的目录。保存的对照结果来自本轮实际执行。

## 计划符合性

| 范围 / 承诺 | 实现与证据 | 本轮判定 |
| --- | --- | --- |
| M3/M7，WORK-N7，安全读取与发布白名单 | F-01–F-03；真实路径/私有来源及旧输出保全测试，合成标记复现 | 已核实；三个原 P1 问题关闭 |
| M3/M4/M7，WORK-F2/F14/F15，V-4/V-6，链接、资源及诊断 | F-05、F-07–F-11 正向修复通过；F-06/F-14 未完全关闭，F-15–F-17 引入正常输入失败 | 存在缺陷；不能把该部分合同记为复审通过 |
| M5，WORK-F7/N5，V-8/V-13，页头、目录、窄屏和无脚本 | F-04/F-12 已解决，三部署项目各 24 项通过 | 本轮自动化通过；真实设备未重验 |
| M6，WORK-F13/N6，V-9，真实依赖保存刷新 | 普通资源与 dev 项目通过；F-13 的合法 symlink 目标仍漏监听 | 部分实现，需补真实来源依赖 |
| M1 与 M8 原全范围证据 | 修复未改 SDK、GPU、锁；本轮不重复原完整矩阵/性能验收 | 本轮未重新签署，不将历史记录冒充实际重跑 |
| M9 | 计划恢复快照明确待复审通过后实施 | 后续范围；无推送、安装交付或版本完成声明 |

## 总体结论与修复顺序

验收判定：**需要修改**。

修复效果 **未完全解决**：11 个旧项已关闭、3 个旧项部分解决，无“不再适用”或用户豁免；新增 F-15–F-17 为可确认 P2 回归。修复增量 `patch is incorrect`。182 项 Rust 测试、82 项浏览器测试与构建/静态检查通过，说明通常路径和大部分修复有效，不能替代这些边界合同。

1. 一起处理 F-06/F-16/F-17 的 CSS 资源识别、文档内片段及发布集合校验，确保安全检查不会误判合法内容，也不再发布缺失资源。
2. 分别修复 F-15 的 URL 拆分顺序、F-13 的真实来源依赖、F-14 的实际生成锚点集合。
3. 对本报告的六个剩余条件补有行为断言的回归，重跑受影响库/CLI/浏览器检查；复审确认关闭后再继续 M9。

下一步：`$apply-code-review docs/plan/DOCS-SSG-P1.code-review.r2.md`，修后 `$review-code docs/plan/DOCS-SSG-P1.md`。报告与证据保存不等于代码已验收、已合并或已发布；本轮保持代码及计划只读，SDLC 只追加本轮独立评审事实。
