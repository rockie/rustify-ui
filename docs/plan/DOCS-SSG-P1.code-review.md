# DOCS-SSG-P1 · 代码评审 R1

验收判定：**需要修改**。`patch is incorrect`；确认 14 项问题（P1 × 3，P2 × 11）。结论只覆盖本轮 M1–M8 变更，M9 尚未实施，不作为缺陷。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 | [DOCS-SSG-P1](DOCS-SSG-P1.md) | `91e81b81ecee53153b9d5df7b74fd843bfafb8c8..72418b31c9acd9a1e893950fc45505fed694f05c`；计划登记的四个实施提交及本地未提交状态核对 | 无；现有 `.review.md` 是开发计划文档评审 |

- 评审日期：2026-10-11（Asia/Shanghai）；review object ID：`65ddeb43-1845-4bee-b70c-bbf9647aa8b7`。
- 计划版本：本地 HEAD `851d964c539bc28a0dddc94df5e3bfbd05293864` 中的计划；SHA-256 `71108cf3efa59c923a047c1fdd03859b1d439e422756522779f73ebe830fc751`。
- 所审代码 target：`72418b31c9acd9a1e893950fc45505fed694f05c`。target 到当前 HEAD 只有计划进度变化；本轮运行验证使用当前 checkout，与所审代码相同。下文源码定位均为 target 侧新增行。
- 实施提交：`179c5ec590d6121efc8c52526e402f7118d1bb07`（M1）、`9442f5dc55d76aa5b0b4badc7be74a3f15f39403`（M2–M4）、`1d239ffd84b951b184cefe1738cf9eb55e579841`（M5–M7）、`72418b31c9acd9a1e893950fc45505fed694f05c`（M5/M8）。期间其他提交仅登记计划进度，因此使用上述累计净 diff。

## 确认问题

### F-01 · [P1] 在所有输入读取入口验证真实路径是否留在工程内

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 206–210。
- 问题：把工程的 `static` 目录本身链接到工程外目录，构建仍会读取并发布外部文件。`walk` 只对枚举到的子项检查 symlink，未验证传入的根目录；另外，正文图片解析允许直接从 `project.root.join(file)` 读取未在快照中的文件，该路径也未验证真实来源。两个复现均成功发布工程外的合成标记，破坏发布范围约束，并可能把构建机器上的其他文件带入公开产物。这是新增输入收集与资源读取路径引入的问题。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 177–192、1248–1252；[generation.rs](../../crates/rustify-docs/src/generation.rs) 437–450；[计划](DOCS-SSG-P1.md) 217、220、295。复现 `external-static-root`、`external-image`：退出 0、`success=true`，分别发布 `outside.txt` 和 `assets/files/public.<hash>.svg`，详见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：读取配置根目录、目录下文件及快照外引用前，都对规范化后的真实路径执行工程边界检查；测试根目录 symlink、路径中间目录 symlink 和工程内合法 symlink。拒绝出界输入时不能发布新代，保留已有产物。
- 置信度：高，两个实际 CLI 复现和完整读取链一致。

### F-02 · [P1] 将私有来源校验应用到别名、片段和品牌资源

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 428–432。
- 问题：私有文件判断使用引用中的名字，而不是被读取文件的真实来源。工程内 `static/public.txt -> .env` 会被原路径复制，`public.txt -> .env` 经 include 会把内容写进正文；甚至 `theme.logo=".env"` 可以直接把 `.env` 按指纹复制，因为 logo 分支只检查存在性。这三个入口均实际泄露合成私有标记。它们破坏已有 `is_private` 明确规定的私有文件不发布策略；F-01 的工程内边界校验无法阻止这些工程内部的泄露。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 129–139、426–438、909–914；[generation.rs](../../crates/rustify-docs/src/generation.rs) 428–450；[计划](DOCS-SSG-P1.md) 220、310。复现 `private-symlink`、`private-include-alias`、`private-logo` 全部退出 0，分别在公开附件、HTML、`assets/files/.env.<hash>` 中找到标记，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。现有 [artifacts 测试](../../crates/rustify-docs-cli/tests/artifacts.rs) 203–241 只覆盖直接位于 static 下的私有名字。
- 修复与验证：对引用路径与真实来源都执行私有路径策略，并在 include、普通附件、logo、分享图入口一致使用；不能仅按生成后的文件名过滤。上述三个复现均应拒绝或排除私有输入，任何产物不得含其标记。
- 置信度：高，三个实际 CLI 复现；未读取真实密钥。

### F-03 · [P1] 在生成页面之前排除私有 Markdown 路径

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 229–232。
- 问题：快照把内容目录下所有 `.md` 文件直接登记为页面，未应用 `is_private`。合法 Markdown 放在 `content/.env.md` 后会生成 `.env/index.html`，标题还出现在首页和 404 页的导航里。该名字符合当前私有路径策略的 `.env.*` 规则，却只在普通文件复制阶段被过滤；页面链路绕过了规则。这会把本应私有的内容和页面身份发布出去，独立于 F-02 的 symlink 和品牌资源入口。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 131–139、225–244；[generation.rs](../../crates/rustify-docs/src/generation.rs) 481–483；[计划](DOCS-SSG-P1.md) 220、310。复现 `private-markdown` 退出 0、零错误，标记出现在 `index.html`、`404.html`、`.env/index.html`，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：在收集/分析页面前过滤私有路径，并保持页面、导航、sitemap 与普通资源的策略一致；覆盖 `.env.md` 和 `.env/notes.md`。如果过滤后没有可发布页面，还需满足 F-10 的诊断要求。
- 置信度：高，实际发布文件与导航泄露均已核对。

### F-04 · [P2] 把配置的顶栏和社交链接传入页面壳并渲染

- 定位：[generation.rs](../../crates/rustify-docs/src/generation.rs) 397–404。
- 问题：`nav.top`、`nav.social` 会被解析并校验成 `analysis.top/social`，但生成器只把侧栏 `analysis.nav` 传入 `Site`，页面壳没有这两组链接的数据。因此有效配置被静默忽略，读者无法使用作者配置的顶栏或社交入口；这不是后续主题定制，PRD 已将按配置显示顶栏列为 P1 行为。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 870–904；[shell.rs](../../crates/rustify-docs/src/shell.rs) 65–73；[PRD](../PRD-DOCS-SSG.md) 316–337；[计划](DOCS-SSG-P1.md) 217。复现 `header-links` 构建成功，HTML 中两个配置 label 均不存在；真实浏览器两个链接的数量均为 0，见[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)和[截图](../validation/docs-ssg-p1/code-review-r1/screenshots/header-links.png)。
- 修复与验证：将 label 与已解析 URL 一起传给共享页头，输出真实链接及当前分区状态；内容页、首页、404、窄屏和无脚本状态均核对这些入口。
- 置信度：高，静态数据链与浏览器一致。

### F-05 · [P2] 将首页操作引用的附件纳入资源发布集合

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 849–853。
- 问题：首页 `actions` 指向存在的内容附件（例如 `manual.pdf`）时，链接会被解析成 `Target::Resource`，但该分支未把资源加入 `resources`，只有正文引用会执行登记。生成器因此找不到发布 URL，退回原始 `manual.pdf`，也不复制附件。构建报告成功，点击首页下载操作却返回 404；正文另有同一附件引用时才会偶然正常。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 822–831、833–867；[generation.rs](../../crates/rustify-docs/src/generation.rs) 274–276、514–518；[计划](DOCS-SSG-P1.md) 218、220、298。复现 `home-resource` 零错误、附件未发布；浏览器点击后请求 `/docs/manual.pdf` 返回 404，见[CLI 结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)、[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)。
- 修复与验证：为首页操作以及配置链接登记其资源目标，使用同一发布 URL 映射；覆盖“附件只被首页操作引用”和子路径构建，确认下载 200 且 URL 指向已发布的指纹资源。
- 置信度：高，端到端点击已复现。

### F-06 · [P2] 校验并改写生成 CSS 中的资源 URL

- 定位：[style.rs](../../crates/rustify-docs/src/style.rs) 273–277。
- 问题：作者在允许的原始 HTML 中使用 `bg-[url(...)]` 时，类会进入样式编译，但产物 CSS 没有资源 URL 校验或改写。跨源背景图在 `build` 成功后被浏览器 CSP 阻止；本地 `bg-[url(/bg.svg)]` 在 `/docs/` 部署时仍请求 `/bg.svg`，实际返回 404。HTML 属性检查没有覆盖 CSS 内部引用，导致同源、安全诊断和基路径资源承诺同时失效。
- 证据：[style.rs](../../crates/rustify-docs/src/style.rs) 233–268；[计划](DOCS-SSG-P1.md) 301、308–309。复现 `css-external`、`css-local` 构建均零错误；浏览器分别记录 `img-src` 违规和 `/bg.svg` 404，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)、[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)。
- 修复与验证：把最终 CSS 的 `url()` 等资源引用接入校验和资源图，在 CSS hash 计算前完成本地引用改写。跨源输入应有定位诊断；根路径和子路径下的本地背景图应实际加载成功。
- 置信度：高，两类输入均有真实浏览器后果。

### F-07 · [P2] 改写资源文件名时保留 URL 的 fragment 和 query

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 1272–1275。
- 问题：资源解析用去掉 fragment/query 的路径查找文件，最终却只保留文件路径，丢失原 URL 后缀。`<use href="/sprite.svg#mark">` 会变成 `/docs/assets/files/sprite.<hash>.svg`，缺少 symbol ID，SVG 图形不显示。正确做法应只改写文件路径，保留引用目标；同一信息丢失也影响媒体等资源的 URL 后缀。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 1242–1257；[generation.rs](../../crates/rustify-docs/src/generation.rs) 186–190；[计划](DOCS-SSG-P1.md) 301、309。复现 `svg-fragment` 构建零错误；浏览器 `use.getBBox()` 为 0×0，只在浏览器 DOM 中恢复 `#mark` 后变为 40×40，见[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)和[原始截图](../validation/docs-ssg-p1/code-review-r1/screenshots/svg-fragment-before.png)。未修改产品代码。
- 修复与验证：资源目标保留路径之外的 URL 部分，srcset、普通附件和媒体使用同一规则；验证改写后的 `#mark` 仍在且图形可见，并覆盖合法 query/fragment 组合。
- 置信度：高，浏览器中的最小反证确认了丢失 fragment 的因果关系。

### F-08 · [P2] 将没有换行的 frontmatter 开始标记报告为语法错误

- 定位：[frontmatter.rs](../../crates/rustify-docs/src/frontmatter.rs) 29–31。
- 问题：Markdown 文件恰为三个字节 `---`、没有换行时，`split` 将起点计算为 4，再切片长度为 3 的字符串，直接 panic。`check --format json` 退出 101、stdout 为空，作者得不到预期的 `frontmatter-unclosed` 诊断，CI 也不能解析报告。这是正常可达的未闭合 frontmatter 输入，不应走内部异常路径。
- 证据：[frontmatter.rs](../../crates/rustify-docs/src/frontmatter.rs) 21–46；[计划](DOCS-SSG-P1.md) 279、296、302。复现 `frontmatter-no-newline` 的 stderr 明确指向第 31 行越界，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：只有找到换行时才推进到下一行；无换行的开标记直接返回带源码位置的未闭合诊断。覆盖 `---`、`---\r`、已闭合 CRLF，验证 CLI 非零退出但有完整 JSON。
- 置信度：高，实际 panic 与源码越界条件一致。

### F-09 · [P2] 读取 `--config` 指定的文件而不是固定默认文件

- 定位：[main.rs](../../crates/rustify-docs-cli/src/main.rs) 248–250。
- 问题：`load` 只取传入配置路径的父目录，随后 `Project::load` 固定读取其中的 `rustify-docs.toml`。在合法 `custom.toml` 与无效默认文件共存时，`check --config custom.toml` 仍报默认文件错误；默认文件若合法，则会静默采用错误配置。dev 重读和 preview 的默认目录推导也使用这一固定文件入口，与通用 `--config <path>` 的语义不符。
- 证据：[content.rs](../../crates/rustify-docs/src/content.rs) 91–93；[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 225、767；[main.rs](../../crates/rustify-docs-cli/src/main.rs) 552–553；[计划](DOCS-SSG-P1.md) 279。复现 `alternate-config` 退出 1，诊断 file 为 `rustify-docs.toml`，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：保留实际配置路径，并统一用于初始读取、诊断 file、dev watch/重读和 preview 配置推导；覆盖两份有效但内容不同的配置，以及指定文件有效、默认文件不存在或损坏的情况。
- 置信度：高，实际读取对象已由 CLI 诊断确认。

### F-10 · [P2] 在没有可发布内容时阻止成功发布空站

- 定位：[generation.rs](../../crates/rustify-docs/src/generation.rs) 481–484。
- 问题：可发布页面列表为空时，生成器仍继续输出 404、样式和站点文件，分析入口也没有添加错误。一个有合法配置、空 `content/` 的已有工程会退出 0、报告 `pages=0, success=true`，写出 production manifest。如果作者删除最后一页，这条路径会将已有站点成功替换成只剩 404 的产物，违背计划明确规定的“无可发布内容报可修复诊断”。
- 证据：[main.rs](../../crates/rustify-docs-cli/src/main.rs) 286–295；[计划](DOCS-SSG-P1.md) 224。复现 `no-content` 退出 0、零错误、`pages=0`，最大页面只有 `404.html`，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：在共享分析/发布闸门检查当前模式下的可发布页面数量，生产模式为空时输出可修复诊断并拒绝替换。覆盖空目录、缺失内容目录、全草稿与删除最后一页，核对已有产物字节保全。
- 置信度：高，空目录实际构建结果已核对；已有输出替换后果由同一发布调用链确定。

### F-11 · [P2] 让发布前置失败也遵守 JSON 输出合同

- 定位：[main.rs](../../crates/rustify-docs-cli/src/main.rs) 409–414。
- 问题：JSON 格式已解析后，输出目录校验、目录创建、锁和部分发布事务失败仍直接调用只写 stderr 的 `fail`。例如 `build --format json --out <工程/content>` 正确拒绝危险目标，却留下空 stdout。调用方无法读取统一的 `success/diagnostics/summary`，也没有错误数与首错摘要；这与 F-08 的解析越界是两个独立失败入口。
- 证据：[main.rs](../../crates/rustify-docs-cli/src/main.rs) 417–430；[report.rs](../../crates/rustify-docs-cli/src/report.rs) 的 `json`；[计划](DOCS-SSG-P1.md) 279、285。复现 `json-output-error` 退出 1，stdout 为空，stderr 仅有文本，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：将格式确定后的运行/发布故障统一转换为诊断，经既有 report 入口输出；继续拒绝危险目录。覆盖输出重叠、锁占用和 I/O/事务失败，断言 stdout 恰好可解析为一个完整 JSON 对象。
- 置信度：高，实际失败输出已记录。

### F-12 · [P2] 避免合法百分号标题 ID 中断页面增强脚本

- 定位：[site.js](../../crates/rustify-docs/assets/site.js) 219–220。
- 问题：自定义标题 `{#100%}` 被正常接受，目录直接输出 `href="#100%"`，脚本却把这个未编码 ID 交给 `decodeURIComponent`，抛出 `URI malformed`。这会中断整个 IIFE 剩余初始化：目录不再高亮，窄屏菜单的 Escape 处理也没有注册。页面构建仍报告零错误，读者只能在运行时遇到缺失交互。
- 证据：[markdown.rs](../../crates/rustify-docs/src/markdown.rs) 871–890；[shell.rs](../../crates/rustify-docs/src/shell.rs) 526–528；[site.js](../../crates/rustify-docs/assets/site.js) 275–286；[计划](DOCS-SSG-P1.md) 297、344–345；[PRD](../PRD-DOCS-SSG.md) 336。复现 `toc-percent` 的浏览器 pageerror 为 `URI malformed`、活动目录数 0，在 375px 打开菜单后按 Escape 仍为 open，见[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)。
- 修复与验证：统一目录 fragment 的编码与读取规则，或独立携带原始 ID；保证合法 ID 不会中断其他增强初始化。覆盖字面 `%`、类似 `%20` 的 ID 与中文 ID，检查目录高亮和窄屏 Escape。
- 置信度：高，完整浏览器操作已复现。

### F-13 · [P2] 将快照目录之外的已引用资源加入 dev watch

- 定位：[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 104–109。
- 问题：正文图片允许引用工程根下的 `../images/mark.svg`，生成器也会读取并发布它；但 watcher 的过滤器只登记 content、static 和代码 include 文件，没有 `analysis.resources`。因此单独修改这张图片不会构建新代，连手动刷新也仍得到旧资源。只有再修改正文触发构建，图片才更新；配置允许的目录外 logo/资源有同类缺口。
- 证据：[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 260、121–123；[content.rs](../../crates/rustify-docs/src/content.rs) 1248–1257；[generation.rs](../../crates/rustify-docs/src/generation.rs) 437–450；[计划](DOCS-SSG-P1.md) 217、324。复现 `watch-resource`：资源单独保存后等待 1.6 秒并重载，仍是 `mark.106eb81194.svg`；正文保存后成为 `mark.c763668ac8.svg`，见[浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)及[服务日志](../validation/docs-ssg-p1/code-review-r1/watch-resource-server.log)。
- 修复与验证：watch 过滤器登记每轮实际使用的资源与片段来源，包含失败后待恢复的引用；删除/原子替换资源也要触发。验证资源单独保存就改变指纹并刷新引用页，无关页保持稳定。
- 置信度：高，浏览器重载与随后正文保存构成直接反证。

### F-14 · [P2] 根据真实脚注 ID 校验锚点而不是放行整个前缀

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 1069–1071。
- 问题：链接锚点只要以 `fn-` 或 `fnref-` 开头就被视为存在。完全没有脚注的页面写 `[Missing footnote](#fn-never-defined)` 时，构建零错误并输出该链接，HTML 中却没有对应 ID。作者把脚注标签写错或跨页链接失效时，内容检查同样会放行，导致 CI 的失效锚点检测出现确定的漏报。
- 证据：[markdown.rs](../../crates/rustify-docs/src/markdown.rs) 936–956、1153–1166 展示真实生成 ID；[计划](DOCS-SSG-P1.md) 298、302。复现 `missing-footnote-anchor` 退出 0，`link_published=true`、`target_exists=false`，见[原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)。
- 修复与验证：在片段中记录实际生成的脚注与引用 ID，和标题 ID 一起按集合校验；保留真实脚注链接可用。覆盖有效脚注、不存在的 `fn-/fnref-` 及跨页脚注锚点。
- 置信度：高，目标 HTML 和 CLI 结果均已核对。

## 待核实与范围限制

以上 14 项都已有足够证据，不需要先补环境或由作者猜意图。仍有以下覆盖边界，不能把本轮检查当作整期全部验收：

- M9 的最终安装 SHA/版本收口、仓外 GitHub 安装及推送流程尚未实施，按[计划恢复快照](DOCS-SSG-P1.md) 48–53 排除；未将这些未完成项列成 finding。Firefox 稳定版已由计划明确移出本期，也未重新提出要求。
- 未重跑 7 个 GPU 示例及 theme-export 的全套既有回归、完整 feature 矩阵、真实 Safari/手机、冻结性能预算和千次保存、远端 CI。历史 M1–M8 记录只用于定位承诺，不计作本轮通过证据。
- 静态审查按风险覆盖核心代码与必要上下游；新增大型测试、语料/性能 harness、依赖锁文件、vendored CSS、全部历史截图/日志未逐行穷尽。全部变更路径和输入指纹见 [scope.json](../validation/docs-ssg-p1/code-review-r1/scope.json)，不声称 485 个变更文件已逐一完整审阅。

## 覆盖与验证

变更意图是交付默认 DOM SDK 与原生静态内容站；内容站采用 DOM/SSR 静态 HTML，SDK 的 GPU 开关与启动适配按兼容链审查。本轮没有新增呈现路径或实现修复。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。模式为开发计划代码初审，单 reviewer；未委派独立 reviewer，未 fetch/pull。
- 累计 diff 共 485 个文件；排除 `.impeccable/`、`.xgent-ai/` 与 `docs/validation/` 后，核心候选路径 142 个。纳入本期已登记的 M1–M8 实现、配置、CLI、SDK 特性/启动/生命周期变化及相应测试；排除历史文档评审、计划进度文案与待做 M9。
- 初始 worktree dirty，只有 SDLC 状态/事件改动，未发现代码未提交增量；index diff 为空。完整输入/上下文快照共 498 个文件，指纹 `495e30cf09b40faf700861c5c4bbe8f3c84e8654b58f2fed490da107b6229202`，算法为排序后的 UTF-8 `path + NUL + SHA-256 hex + newline` 再做 SHA-256。保存报告前核对 498 个原输入指纹，无变化；此次新增报告、复现证据与随后 SDLC 登记不进入所审代码范围。
- 已静态核查：workspace/包特性与 CI 分组，SDK 中性 runtime、两种 loader、mount/abort/listener/timer/resize；文档配置、frontmatter、内容图、链接/资源/导航、Markdown/原始 HTML、SSR 壳、样式、高亮、发布事务、preview、dev watch/SSE 和 CLI 报告链。涉及问题的源码、校验入口和现有相关用例均独立反证核对。
- 复现全部使用隔离临时目录及合成内容；未访问业务数据、真实私有文件或部署系统。浏览器由安装好的 Playwright Chromium `153.0.8010.12` 实际启动，通过本轮重建的 CLI preview/dev 访问最新产物；检查 pageerror、console、CSP、失败响应，保存截图。没有 Portal 宿主接入场景。

| 本轮实际命令 / 操作 | 结果 | 证据 |
| --- | --- | --- |
| `mbx test --locked -p rustify-docs` | 94 单测 + 7 集成测试通过；doc-test 0 项 | [日志](../validation/docs-ssg-p1/code-review-r1/library-test.log) |
| `mbx test --locked -p rustify-docs-cli` | 56 项通过 | [日志](../validation/docs-ssg-p1/code-review-r1/cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 成功；用于全部 CLI 边界复现 | [日志](../validation/docs-ssg-p1/code-review-r1/cli-build.log) |
| `mbx fmt --all -- --check` | 退出 0 | [日志](../validation/docs-ssg-p1/code-review-r1/fmt.log) |
| `mbx clippy --locked -p rustify-docs --all-targets -- -D warnings`；CLI 单包同命令 | 两次独立 invocation 均退出 0 | [库日志](../validation/docs-ssg-p1/code-review-r1/library-clippy.log)、[CLI 日志](../validation/docs-ssg-p1/code-review-r1/cli-clippy.log) |
| `mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml --output <临时目录>/dom-product` | 重建当前 DOM fixture 成功 | [日志](../validation/docs-ssg-p1/code-review-r1/dom-build.log) |
| `node_modules/.bin/playwright test -c playwright.docs.config.ts --project=docs-subpath --reporter=list --output <临时目录>/browser-suite` | 21 项通过，含无脚本、主题/导航、响应式、axe | [日志](../validation/docs-ssg-p1/code-review-r1/docs-subpath-browser.log) |
| `DOM_ONLY_DIR=<上述当前 DOM 产物> node_modules/.bin/playwright test -c playwright.docs.config.ts --project=dom-only --reporter=list --output <临时目录>/dom-suite` | 12 项通过，含协议拒启、两实例与各 trap 路径隔离 | [日志](../validation/docs-ssg-p1/code-review-r1/dom-browser.log) |
| 本报告附带 `reproduce.py` | 在新临时工程中执行 18 个 CLI 场景；确认上述失败行为，未作为“测试通过”计数 | [原始结果](../validation/docs-ssg-p1/code-review-r1/reproductions.json)、[摘要](../validation/docs-ssg-p1/code-review-r1/reproduction-summary.json) |
| 本报告附带 `browser-cases.cjs` | 顺序核对 7 个隔离站点，确认 F-04–F-07/F-12/F-13；服务关闭后保留日志 | [浏览器结果](../validation/docs-ssg-p1/code-review-r1/browser-results.json)、[截图目录](../validation/docs-ssg-p1/code-review-r1/screenshots/) |

复现配方（从仓库根运行，脚本自动建立新临时目录）：

```sh
mbx build --locked -p rustify-docs-cli --profile docs-cli
python3 docs/validation/docs-ssg-p1/code-review-r1/reproduce.py --binary target/docs-cli/rustify-docs
node docs/validation/docs-ssg-p1/code-review-r1/browser-cases.cjs . <上一命令输出的 work> target/docs-cli/rustify-docs
```

浏览器脚本仅编辑临时 fixture 和该浏览器内的 SVG DOM，不改仓库实现。当前保留的 JSON、日志、截图都是本轮实际执行产物。源码属于新增库/CLI，基线没有这些命令，因此不把旧版本的“无法运行该命令”伪装成行为对照。

## 计划符合性

| 本轮范围 | 契约与检查 | 本轮核查结论 |
| --- | --- | --- |
| M1 默认 DOM SDK、GPU opt-in | WORK-F17 / C-2，V-2；计划 §4.1 | 当前 DOM 构建及 12 项生命周期浏览器检查通过；GPU 既有全回归未重跑，不能据此重签整条 WORK-F17 |
| M2–M4 分析、CLI、内容和产物 | WORK-F2/F6/F7/F14/F15，V-4/V-6/V-7/V-10；计划 §3、§5.1–5.3 | F-04–F-11/F-14 表明链接、资源、输入和诊断合同有缺口；已有正向测试通过不足以关闭这些条件 |
| M5 阅读主题与交互 | WORK-F7/N3/N5，V-8；计划 §6 | 通常 fixture 的 21 项浏览器检查通过；F-04 的入口缺失与 F-12 的有效 ID 边界需修复 |
| M6 dev | WORK-F13/N2/N6，V-9；计划 §5.5 | CLI 既有 dev 测试通过；F-13 证明实际接受的资源输入未被完整监听 |
| M7 安全和可靠性 / M8 收口 | WORK-N7，V-10；计划 §5.2–5.4 | F-01–F-03 为实际发布范围/私有内容泄露，F-06 为 CSS 资源漏检；应先修复后复审，本轮不改原里程碑完成声明 |
| M9 | 计划恢复快照 48–53 | 未实施，明确排除；安装提交、版本和远端授权不作为本轮 finding |

## 总体结论与修复顺序

验收判定：**需要修改**。

`patch is incorrect`。14 项问题均与所审新增代码重叠，已用实际 CLI 或浏览器复现；其中 F-01–F-03 可将工程外或私有数据写入公开产物，应在发布/安装候选收口前处理。157 项 Rust 测试与 33 项既有浏览器用例通过，说明通常路径工作正常，不消除这些已确认缺陷。

1. 先关闭 F-01–F-03 的读取边界和私有路径缺口，并验证失败时保留旧产物。
2. 修复 F-04–F-07/F-13 的入口、资源图和 watch，补真实子路径浏览器回归。
3. 修复 F-08–F-12/F-14 的配置选择、诊断、空站和锚点边界，再执行受影响检查并逐条复审。

下一步：`$apply-code-review docs/plan/DOCS-SSG-P1.code-review.md`。修复后用 `$review-code docs/plan/DOCS-SSG-P1.md` 逐条复审。此次只交付评审与证据，不修改代码、计划进度或提交；验收判定不代表合并、发布授权。

UI 核查使用了 impeccable 的已有上下文。其 sidecar 提示 DESIGN 缓存落后、缺少 `buildPath`；本轮以实际源码和浏览器产物判断，未修订该上下文。
