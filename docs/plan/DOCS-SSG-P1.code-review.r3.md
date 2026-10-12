# DOCS-SSG-P1 · 代码评审 R3

验收判定：**需要修改**。修复效果 **未完全解决**；修复增量 `patch is incorrect`。上轮 17 项中 **15 项已解决、2 项部分解决**（F-13、F-16）；本轮新增 **2 项 P2**（F-18、F-19）。共四个剩余 P2；M9 继续排除。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3 | [DOCS-SSG-P1](DOCS-SSG-P1.md)，计划对象 `526c88c7-a4b3-4ea9-9193-371420b66d1d` | `bfbee6d3342b69e062ed513e05e7f812373124bc..1f0fc5ed21d7937a1297528ad38cdefe6b95f721`；实现修复 `a297dbd98f39f87910b1ad4100370dd9f630dae1`；输入指纹 `9d1ac4b2d5d4dd8fc4c2aa33cb385cc7a45997bfca975a98406fb29ebd368261` | [R2](DOCS-SSG-P1.code-review.r2.md)，对象 `ff41c49f-1b63-4b5b-85f2-4dbdfb96d73d` |

本轮评审对象 ID：`7b6a9d1a-088d-4213-be21-74348a929584`。本轮是**修复效果复审**，不是原 M1–M8 完整重审。上轮比较范围为 `72418b31c9acd9a1e893950fc45505fed694f05c..bfbee6d3342b69e062ed513e05e7f812373124bc`；本轮从其目标起算。R2 报告 sha256 为 `10e7649afbabd7ffd64732ea28a81eabbcc27e8ab3a8305728466a93348413ed`，已有处置见 [M8 的 R2 修复后复验](../validation/docs-ssg-p1/M8.md#r2-修复后复验)及 M3/M4/M6/M7。历史报告与其原结论不改写。

## 确认问题

### 上轮问题复审

F-01–F-14 沿用 [R1](DOCS-SSG-P1.code-review.md)（对象 `65ddeb43-1845-4bee-b70c-bbf9647aa8b7`）身份；F-15–F-17 属于 R2。所有结果在本轮 target 核查，未将持续旧问题冒充新增 finding。共同验证见下方命令表。

| 来源 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01 | 输入真实路径出界；R2 已解决 | M7 / `12113ab` | **已解决** | `read_source` 仍核验真实根；本次 track 只登记依赖、不授权读取。库与 CLI 私有/工程外链接及旧产物保全测试通过 | 无 |
| R1 / F-02 | 私有来源经别名或品牌发布；R2 已解决 | M7 / `12113ab` | **已解决** | 别名与真实路径的 private 校验保持；品牌/include 新增依赖登记不绕过读取闸门。相关库、CLI 测试通过 | 无 |
| R1 / F-03 | 私有 Markdown 被当页面；R2 已解决 | M7 / `12113ab` | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 309–311 保持过滤；库 `private_markdown_is_never_a_page`、CLI 私有发布测试通过 | 无 |
| R1 / F-04 | 顶栏与社交入口未渲染；R2 已解决 | M5 / `12113ab` | **已解决** | 壳接线未改；docs-subpath 页头、首页/内容/404、窄屏与无脚本路径通过 | 无；真机复走未重验 |
| R1 / F-05 | 仅首页操作引用的附件未发布；R2 已解决 | M4 / `12113ab` | **已解决** | CLI 原首页附件用例及新增操作/页头 `%3F` 附件用例通过 | 无 |
| R1 / F-06 | CSS 资源缺校验/改写；R2 部分解决 | M7 / `a297dbd` | **已解决** | [generation.rs](../../crates/rustify-docs/src/generation.rs) 650–675 使用真实发布集合；原前缀绕过输入 target 报 missing-image，base 成功；直接注入 CSS URL 的生成器测试报 output-check；合法背景真实加载 200 | 无；独立解码回归另见 F-19 |
| R1 / F-07 | URL 后缀丢失；R2 已解决 | M4 / `12113ab` | **已解决** | 原 suffix-content 保留 `?download=1#page=2`；新增正文/操作/页头/内容/static 附件测试通过，浏览器获取正确字节 | 无 |
| R1 / F-08 | 无换行开栏 panic；R2 已解决 | M3 / `12113ab` | **已解决** | frontmatter 修复实现未变，当前库/CLI 相应用例通过 | 无 |
| R1 / F-09 | 忽略实际 --config；R2 已解决 | M4 / `12113ab` | **已解决** | 指定配置的 check/dev 集成测试通过；本轮复现也均用隔离配置启动 | 无 |
| R1 / F-10 | 空站成功替换旧产物；R2 已解决 | M4 / `12113ab` | **已解决** | no-pages 闸门未改；空/缺目录/全草稿库测试、旧输出保全 CLI 测试通过 | 无 |
| R1 / F-11 | 发布前置失败无 JSON；R2 已解决 | M4 / `12113ab` | **已解决** | CLI `a_refused_output_is_reported_in_json_as_well`、输出归属/持锁测试通过 | 无 |
| R1 / F-12 | 百分号标题中断脚本；R2 已解决 | M5 / `12113ab` | **已解决** | 原 percent-headings 构建通过，docs-subpath 百分号标题、目录与窄屏菜单测试通过，无脚本异常 | 无 |
| R1 / F-13 | 快照外资源保存不刷新；R2 部分解决 | M6 / `a297dbd` | **部分解决** | 原图片真实目标的替换/删除恢复与无关页稳定已通过；但 snapshot 内附件 symlink 删除后丢失目标监听，见下文 | 保留失效 snapshot 链接依赖 |
| R1 / F-14 | 未输出脚注贡献 phantom ID；R2 部分解决 | M3 / `a297dbd` | **已解决** | [markdown.rs](../../crates/rustify-docs/src/markdown.rs) 1180–1183 从合并后的 HTML 取 ID、缓存版本提升；unused-footnote target 拒绝，base 放行；同页/跨页合法脚注测试通过 | 无；新增 template 回归另见 F-18 |
| R2 / F-15 | 解码后拆问号误伤附件名 | M4 / `a297dbd` | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 1352–1353 先拆查询再解码；两份 `%3F` fixture base 失败、target 成功；CLI 全入口/真实后缀断言及浏览器正确下载通过 | 无 |
| R2 / F-16 | 字符串误作 URL、实体/原始 span 不一致 | M7 / `a297dbd` | **部分解决** | 原 content 文字、普通实体、多 URL 均通过；`before` 可见文字未改。无分号数字实体后接 `=` 的 span 仍吞 URL 外括号，见下文 | 精确映射字符引用的实际消费范围 |
| R2 / F-17 | CSS 文档内 SVG 片段被拒绝 | M7 / `a297dbd` | **已解决** | 同页 mask/filter 校验 ID、最终 CSS 保留片段；缺目标被拒且旧产物不变。真实 mask 截图中心为红色、角落非红色，非仅检查声明存在 | 无 |

### F-13（持续问题）· [P2] 保留快照内失效符号链接的恢复依赖

- 定位：[content.rs](../../crates/rustify-docs/src/content.rs) 312–320、1399–1404；支持 [dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 285–300，当前 target；旧问题定位不要求与本轮 diff 重叠。
- 问题：正文 `[Download](manual.pdf)` 引用合法的 `content/manual.pdf -> ../brand/real.pdf` 时，初始 snapshot 登记真实目标；删除 `brand/real.pdf` 后，walk 在 metadata 失败处提前跳过该链接，附件 resolve_link 又不走 Sources.track。失败轮仍用空缺依赖的新 filter 替换原 filter，因此仅恢复真实文件不会重建，页面一直显示旧产物及 broken-link 错误，必须额外保存 Markdown 才恢复。图片/include 的新增 track 修好原复现，但没有覆盖 snapshot 内附件的失效恢复，仍属于原 F-13 依赖监听根因，不计新增。
- 证据：[真实浏览器及两版结果](../validation/docs-ssg-p1/code-review-r3/candidate-results.json) 的 attachment-symlink-recovery：target 初始 revision 1，删除后失败 revision 2；恢复后连续三个一秒观测仍为 2、错误面板仍可见；保存正文才到 revision 3 并读到 restored attachment。隔离 base 二进制呈相同失败，排除新增归因。[计划 §5.5](DOCS-SSG-P1.md#55-dev-失效错误恢复与滚动) 325、330 要求 watch 输入、删除文件重新诊断依赖者，并在修复后清除错误。
- 修复与验证：在 snapshot 遍历跳过失效链接前保留安全、工程内的可恢复真实依赖，或让附件也经过统一依赖登记；不能把依赖推断当作读取授权。补附件、static 链接的删除/恢复及无关页稳定测试；只恢复目标应清除错误并刷新引用页。
- 置信度：高；两版 CLI 的真实浏览器观测、失败轮 filter 构造及 Markdown 保存阳性对照一致。

### F-16（持续问题）· [P2] 不把数字实体后面的等号并入改写 span

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 256–271、411–415，当前 target。
- 问题：合法 class `[background-image:url&#40=/bg.svg)]` 被浏览器解码为 `url(=/bg.svg)`，资源存在于 `content/=/bg.svg`。attribute_spans 却把数字字符引用后面的 `=` 也吞进 entity，把解码出的 `(` 与 `=` 映射到同一原始范围；改写从 URL 参数的 `=` 开始时连左括号一起替换。target 构建成功、零错误，但发布 class 变成 `[background-image:url/docs/assets/files/bg.106eb81194.svg)]`，实际 computed backgroundImage 为 none。原 F-16 要求实体解码与 raw span 一致，这个范围仍未修好；基线本来也不正确处理此实体 class，故不计新增。
- 证据：[两版 CLI/浏览器结果](../validation/docs-ssg-p1/code-review-r3/css-candidate-results.json) 的 entity-paren-equals 与 entity-semicolon-control。同一资源、仅给 `&#40` 增加分号后，浏览器解码值完全相同，target 正常指纹化且加载背景；无分号版本左括号丢失。[合成输入及发布 HTML](../validation/docs-ssg-p1/code-review-r3/candidate-fixtures/entity-paren-equals-target/) 和[截图](../validation/docs-ssg-p1/code-review-r3/entity-paren-equals-target.png)保留。
- 修复与验证：按 HTML 字符引用实际消费的字符建立 raw span，数字实体后的普通 `=` 独立映射，不让 URL 改写吞掉函数分隔符/引号。覆盖有无分号、数字括号/引号实体、后接 `=`，断言最终 class、CSS 声明及真实加载结果。
- 置信度：高；浏览器解码与改写结果直接对比，分号阳性对照通过。

### F-18（本轮新增）· [P2] 排除 template 内容中的非文档锚点

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 160–163，target，位于修复 diff。
- 问题：新增 ids 扫描无条件收集所有 StartTag 的 ID，包括 `<template><span id="ghost">…</span></template>` 内不属于 document 树的 ID。随后 `[Ghost](#ghost)` 被 resolve_link 放行，build 成功且零错误，但读者点击该链接没有文档内目标。原策略允许 template，主题脚本也不挂载其内容；基线会报 broken-anchor，是本次把所有词法 ID 当最终可导航 ID 新引入的漏检，独立于已关闭的未输出脚注问题。
- 证据：[candidate-results.json](../validation/docs-ssg-p1/code-review-r3/candidate-results.json) 的 template-anchor：base 退出 1 / broken-anchor，target 退出 0；通过 target preview 在 Chromium 读到 document.getElementById('ghost') 为 null、template.content 内确有 ghost；点击后 hash 为 #ghost。template 元素自身的 holder ID 正常可见，证明不是 HTML 被整体丢弃。支持 [markdown.rs](../../crates/rustify-docs/src/markdown.rs) 1182、[content.rs](../../crates/rustify-docs/src/content.rs) 1311–1317；契约为[计划 §5.2](DOCS-SSG-P1.md#52-单一内容图可定位语法与安全检查)的失效锚点检查。
- 修复与验证：收集 ID 时尊重 template 内容与 document 树的边界，保留 template 元素自身 ID；补嵌套 template、同页/跨页引用、普通原始 HTML ID 的正反测试，并在浏览器验证实际目标。
- 置信度：高；固定版本二进制对照与真实 document 查询一致。

### F-19（本轮新增）· [P2] 让 CSS 资源扫描与编译器使用相同的任意值解码

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 216–218，target，位于修复 diff。
- 问题：新的 class_urls 直接把未做任意值解码的内容送给 CSS tokenizer，而现有 compiler 会解码 underscore。配置 base='/'、文件 `static/assets/bg one.svg`、class `[background-image:URL('/assets/bg_one.svg')]` 在基线正常构建，最终 CSS 实际指向已发布的空格文件且浏览器加载成功；target 却按原文去找 `bg_one.svg`，报 missing-image，拒绝原本可用的站点。问题不是要求扩大 CSS 支持，而是新增资源解析与本仓已有编译语义不一致造成兼容性回归；只支持普通小写 url 或简单大写路径的测试不能覆盖这一差异。
- 证据：[两版构建及浏览器结果](../validation/docs-ssg-p1/code-review-r3/css-candidate-results.json) 的 uppercase-underscore：base 退出 0、computed URL 为 `/assets/bg%20one.svg`、无失败响应；target 退出 1 / missing-image。支持 [value.rs](../../crates/rustify-style/src/compiler/value.rs) 359–375：只对小写 url 内参数保留 underscore，其他函数参数经过 decode；该编译器实现本轮未改。[基线截图](../validation/docs-ssg-p1/code-review-r3/uppercase-underscore-baseline.png)与双版本合成产物保留。
- 修复与验证：提取加载 URL 时复用或精确匹配 compiler 的任意值解码，并维护可改写的原始范围，不各写一份不一致的规则。覆盖 URL/url、带 underscore/转义 underscore/%20 的路径、带空格的真实文件，断言分析目标与最终 CSS 加载目标一致。
- 置信度：高；从固定 base 隔离构建的二进制确实成功，target 确实拒绝，未用当前 checkout 冒充基线。

## 待核实与覆盖限制

四个剩余问题均已确认，不需要先补环境才能修复；无仅凭猜测升级的 finding。

- 实际浏览器为已安装的 Playwright Chromium `153.0.8010.12`；本轮未重验真实 Safari、实体手机或用户安装的 Chrome，也未重跑 WebKit。窄屏检查是浏览器仿真视口，不代表真机验收。
- 本轮未重跑 docs-root/meta、GPU/DOM 全矩阵、7 示例、theme-export、500 页冻结性能与千次保存、远端 CI。相关历史证据不记本轮通过；本次 CSS/native 库增量的正确性通过对应测试与真实浏览器核验，不扩展为整期性能结论。
- M9 的安装 SHA/独立 CLI 版本、仓外安装、安装后二进制验证、GitHub 可达性及推送授权仍是后续范围，不作为本轮实现遗漏。计划 8/9 不因这次报告改写。
- 历史验证文档和 SDLC 的进度措辞不作为代码 findings 对象；对处置“已修复”的自述，以上独立结果优先。只新增评审与派生状态，不改原报告、计划或实现。

## 覆盖与验证

变更意图：关闭 R2 的六个剩余条件。呈现采用 **纯 DOM 静态 HTML/SSR + 原生 CLI**，无需 GPU。主 agent 整合两个只读 reviewer：一位追踪 CSS token/实体/span/发布集合，一位追踪依赖/锚点/URL 后缀；所有候选由主 agent 再读源码、运行固定版本 CLI 和 Chromium 对照裁定，不以 reviewer 的结论代替证据。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；日期：2026-10-11，+08:00；原输入：`/review-code docs/plan/DOCS-SSG-P1.md`。
- 计划 sha256：`dfef91894461fbf32b8f9ad44b65473fbe28073cc60834a5548ab1cd7d9121d2`。登记的 `179c5ec590d6121efc8c52526e402f7118d1bb07`、`9442f5dc55d76aa5b0b4badc7be74a3f15f39403`、`1d239ffd84b951b184cefe1738cf9eb55e579841`、`72418b31c9acd9a1e893950fc45505fed694f05c`、`12113aba10eea8f4454fe540aaab083e4b5a4042` 已被前轮覆盖，本轮按旧项核对相关当前实现；`a297dbd98f39f87910b1ad4100370dd9f630dae1` 纳入新修复差异。上述提交全部解析并确认在 HEAD 祖先链。
- 代码比较是固定 base→target 的累计净差异；`1f0fc5ed21d7937a1297528ad38cdefe6b95f721` 仅修改计划，作为进度回写排除。未发现本区间未登记代码提交。
- 纳入 12 个代码/配置/测试路径：Cargo.lock、rustify-docs/Cargo.toml、classes/content/generation/html_check/markdown/style 六个源文件、docs generate 集成测试、CLI artifacts/dev 测试、浏览器 dev.spec.ts。读取必要 compiler/value、CLI filter、壳与脚本等上下游。cssparser 已在锁中，本次仅新增直接使用，不涉及 Major 升级。
- 当前 index 无 staged 差异；tracked 工作树差异与 untracked 均枚举，只有既有报告、证据与 SDLC，未提交代码差异为空。保留它们，不纳入新增代码 findings。快照、清单与输入内容 hash 见 [scope.json](../validation/docs-ssg-p1/code-review-r3/scope.json)，结束核对见 [scope-end.json](../validation/docs-ssg-p1/code-review-r3/scope-end.json)：HEAD 和全部固定输入未变；本轮新增报告/证据不污染输入。
- 当前验证在 target checkout 执行；基线用 git archive 提取到隔离目录，以独立 CARGO_TARGET_DIR、offline/locked 和 docs-cli profile 编译，不 checkout/reset/stash 用户工作树。两二进制内容 hash、命令与结果见 [commands.json](../validation/docs-ssg-p1/code-review-r3/commands.json)。

| 命令 / 实际步骤 | 结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-docs` | 123 项通过（113 单测 + 10 集成）；[日志](../validation/docs-ssg-p1/code-review-r3/rustify-docs-test.log) |
| `mbx test --locked -p rustify-docs-cli` | 69 项通过；含发布保全、真实 watch、输入安全及新增附件/CSS 回归；[日志](../validation/docs-ssg-p1/code-review-r3/rustify-docs-cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | target 重建通过；[日志](../validation/docs-ssg-p1/code-review-r3/cli-build.log) |
| 隔离 base：`mbx build --offline --locked -p rustify-docs-cli --profile docs-cli` | 通过；[日志](../validation/docs-ssg-p1/code-review-r3/baseline-build.log) |
| `mbx clippy --locked -p rustify-docs -p rustify-docs-cli --all-targets -- -D warnings`；`mbx fmt --all -- --check` | 均通过；[Clippy](../validation/docs-ssg-p1/code-review-r3/clippy.log)、[fmt](../validation/docs-ssg-p1/code-review-r3/fmt.log) |
| `RUSTIFY_DOCS_BIN=<target binary> npx --no-install playwright test --config playwright.docs.config.ts --project docs-dev` | 11/11；实际删除/恢复、无关页保持、失败/panic/重启恢复与键盘/axe；[日志](../validation/docs-ssg-p1/code-review-r3/docs-dev.log) |
| `npx --no-install playwright test --config playwright.docs.config.ts --project docs-subpath` | 24/24；子路径、导航/页头/404、主题、复制、代码组、窄屏、无 JS、对比与 axe；[日志](../validation/docs-ssg-p1/code-review-r3/docs-subpath.log) |
| 原 R2 replay.py，分别传 target 与隔离 baseline | 11 场景双版本对照，合法输入成功、原两个应拒场景 target 拒绝；[结果](../validation/docs-ssg-p1/code-review-r3/replay-comparison.json) |
| 临时复制 browser-cases.cjs，逐一 preview 新产物 | 8/8 浏览器边界断言通过：可见 url 文字、背景真实加载、SVG mask 实际像素、编码附件下载；console/失败请求检查通过；[结果](../validation/docs-ssg-p1/code-review-r3/browser-results.json) |
| 临时 candidates.cjs / css-candidates.cjs | 10 个版本/输入结果；确认 F-13/F-16/F-18/F-19 及阳性对照，浏览器入口/版本/请求与 revision 记录见两份结果 JSON；脚本和合成产物已保存 |

测试全绿不否定以上缺陷：现有新增测试覆盖了普通图片 symlink、带分号实体和普通 CSS URL，没有覆盖失败轮丢附件依赖、数字实体后接等号、template 非文档 ID 或任意值 underscore 解码差异。复现全部在隔离目录，不添加或修改产品测试。

## 计划符合性

| 计划范围 → 本次修复 | 实现与独立结果 | 状态 |
| --- | --- | --- |
| WORK-F2 / M3 / V-4：有效锚点、同页/跨页脚注 | 未输出脚注旧问题修好；新增 template 非文档 ID 漏检（F-18） | **存在缺陷** |
| WORK-F2、WORK-F14 / M4 / V-6：附件资源与 URL | 编码问号、普通查询/fragment、操作/页头发布通过；合法 CSS 资源解码存在 F-19 | **部分已核实，存在缺陷** |
| WORK-F13 / M6 / V-9：依赖刷新及错误恢复 | 快照外图片真实目标正确；快照内附件失效依赖仍丢失（F-13） | **存在缺陷** |
| M7 / §5.2 / V-4、V-6：CSS 资源图、严格 CSP、实体跨度 | 实际发布集合、字符串与文档 mask 修好；数字实体跨度仍破坏合法样式（F-16） | **部分已核实，存在缺陷** |
| M5 受影响阅读交互与 M8 局部集成 | 本轮 35 项浏览器测试及 8 个生产边界场景通过；不重签真机/完整性能门 | **本轮所测已核实，其余未重验** |
| M9：安装 SHA、GitHub 与最终交付 | 未实施，不属于本轮修复范围 | **后续范围** |

## 总体结论与修复顺序

验收判定：**需要修改**。修复效果 **未完全解决**，修复增量 `patch is incorrect`。15 个旧项关闭；F-13/F-16 仍有同根因剩余条件，新增 F-18/F-19 有固定版本对照和真实浏览器证据。192 项 Rust 测试与 35 项标准浏览器测试全过，但不能替代缺失的边界断言。

1. 补齐失效 snapshot 链接的依赖登记及恢复，保留读取安全边界（F-13）。
2. 校准 HTML 实体消费跨度、任意值解码和实际 document 锚点语义（F-16、F-19、F-18）。
3. 为四个条件补行为回归并重跑受影响库/CLI/浏览器检查；独立复审关闭后再继续 M9。

下一步：`/apply-code-review docs/plan/DOCS-SSG-P1.code-review.r3.md`；修后 `/review-code docs/plan/DOCS-SSG-P1.md`。本报告不授权合并、发布、推送或 PRD 整期验收；未修改代码、计划或前轮报告，SDLC 仅登记本次独立评审事实。
