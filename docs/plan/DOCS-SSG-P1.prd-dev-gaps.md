# DOCS-SSG-P1 · PRD 与开发交付差异核查

## 1. 结论与范围

**P1 尚未完整交付：确认 4 项明确遗漏、2 项有依据的合理变更；需 Triage 的产品差异 0 项，待决 0 项；待补证 2 类。** 差异按 G-ID 计数，待补证按 U-ID 计数，均不等于需求或任务数。本报告完成后仍不能宣称整期通过。

新计划承接 7 个 G/U 跟进来源（4项补齐、2项文档同步、1项真机补证），合为3类增量工作；K-01/U-02另由原M9承接。

输入：[原计划](DOCS-SSG-P1.md) → [PRD](../PRD-DOCS-SSG.md) §7.1–7.3 的 **P1（第 1 期）**：18 条需求的 65 条 AC。计划自述 M1–M8 已完成、8/9，M9 未完成；因此“已完成计划”的输入前提不成立，本次继续核查可确认的交付。M9 的安装、最终文档、GitHub 来源预览及远端同 SHA 验证是原计划已知未完工作，列为 K-01，不包装为新发现的 A，也不重复建任务。

本次是首次完整 P1 覆盖核查。已有 [代码复审 R6](DOCS-SSG-P1.code-review.r6.md)只关闭其 21 项代码问题；本报告不改写其原结论，也不把局部修复通过等同于整期 PRD 覆盖。呈现路径是宿主 Rust CLI + DOM 静态 HTML/SSR；仅沿用历史证据核对 SDK 的 GPU opt-in 兼容，不新增 GPU 能力。

审查基线：2026-10-12 +08:00，`main` / `aabcd95810d4036bd4203aea87a0934c8d50f603`，dirty。原 dirty 是报告、证据、经验和 SDLC 资料，没有未提交产品代码；本次产品、PRD、原计划和 M1–M8 记录保持只读，无提交、推送或发布。输入指纹见 [scope-start](../validation/docs-ssg-p1/prd-dev-gaps/scope-start.json)，交付前复核见本报告 §5。

本次执行：库 134 项、选定 CLI 集成 51 项 Rust 测试通过；docs-cli 构建通过；Chromium 的 root/subpath/meta/dev 四项目分别 24/24、24/24、24/24、21/21，共 93 项通过。另用独立临时工程复现 G-01–G-04。这些反例表明现有绿测试没有完整覆盖 PRD 的语义要求。

## 2. 三类差异

### A · 明确遗漏（4）

#### G-01 · 默认页面缺少导航或页脚地标 · A

- PRD 期望：[WORK-N5/AC-3](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)：每页具有页头、导航、主体、页脚四类地标；适用 P1 已交付的首页、内容页和 404。
- 实际交付：`new` 原样生成的首页没有 `nav`；首页、内容页和 404 的 `contentinfo` 均为 0。内容页的 `page_footer` 只在有编辑链接/更新时间时生成，还位于 main 内，不能替代稳定的站点页脚地标。W3C 的[地标映射](https://www.w3.org/WAI/ARIA/apg/patterns/landmarks/examples/contentinfo.html)也明确 main 内的 footer 不具有默认 contentinfo 语义。首页和 404 没有独立站点页脚。
- 证据链：原计划 M5/V-13 → [M5 记录](../validation/docs-ssg-p1/M5.md)与 [M8](../validation/docs-ssg-p1/M8.md) → [shell::home_layout](../../crates/rustify-docs/src/shell.rs)、`header`、`page_footer` → [本次反例](../validation/docs-ssg-p1/prd-dev-gaps/probe-results.json)的 `template-landmarks`：首页 banner/navigation/main/contentinfo = 1/0/1/0；内容页和真实 404 也无 contentinfo。当前 [theme.spec.ts](../../tests/docs-ssg/theme.spec.ts)“one h1, landmarks…”只断言 banner/main，未断言 navigation/contentinfo。
- 影响：使用地标导航的读者无法定位站点导航或页脚；无 git/编辑配置的普通新站即可触发。
- 分类依据：这是本期明确的默认壳验收条件，未发现豁免；补齐已有导航语义和稳定页脚不需要新的产品取舍。不将“缺地标”直接等同于自动化 WCAG 严重违规。
- 处置与 Follow-up：首页已有阅读入口承担导航语义；所有页面提供实际可访问的站点页脚地标，不新增装饰或营销文案。三类页面在有/无 git、编辑链接、顶栏配置下核验四类地标。交给 gaps M1；无需再次询问是否恢复原要求。

#### G-02 · 原始 HTML 标题绕过 H1 与标题跳级检查 · A

- PRD 期望：[WORK-F2 边界](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)允许合规原始 HTML；[WORK-N5/AC-3](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)要求每页恰有一个 H1、作者标题跳级由 check 报出；[WORK-F15 行为](../PRD-DOCS-SSG.md#work-f15-检查内容并给出可执行的诊断)要求检查内容质量。
- 实际交付：同页写 `# Markdown title`、`<h1 id="second">Second title</h1>`、`<h4 id="skipped">Skipped heading</h4>`，check/build 均成功，错误 0，未报 heading-skip；浏览器实际呈现两个 H1 和跳级的 H4。
- 证据链：M3/V-4、M5/V-13 → 完成记录 → [markdown::heading_end](../../crates/rustify-docs/src/markdown.rs)只统计 Markdown heading 事件，`Event::Html` 走另一入口；[content::analyze](../../crates/rustify-docs/src/content.rs)只用该 h1_count/headings 校验 → [反例 `raw-heading`](../validation/docs-ssg-p1/prd-dev-gaps/probe-results.json)与[375px 实际截图](../validation/docs-ssg-p1/prd-dev-gaps/raw-heading-375.png)。原始 HTML 安全检查没有补齐这些语义。
- 影响：允许的作者输入会发布错误的标题结构，check 不能帮助作者定位、修复；屏幕阅读器的标题导航与默认主题承诺不一致。
- 分类依据：合规 HTML 与单 H1/跳级检查都已有明确要求，补齐实际可见标题的语义校验无需禁用 HTML或重定义产品范围。
- 处置与 Follow-up：合并 Markdown 与原始 HTML 的有效标题语义，保留源码位置；多 H1 按现有错误语义拒绝，跳级按现有警告及 deny-warnings 语义处理。排除 template.content 的隐藏标题，保留合法 raw HTML、SVG/body/template 锚点边界。交给 gaps M1。

#### G-03 · 首页标题合并后丢失已被 check 接受的锚点 · A

- PRD 期望：[WORK-F2/AC-2、AC-3](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)的页面标题链接与自定义锚点可用；[WORK-F7/AC-2](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)到达目标标题；[WORK-N3/AC-1](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容)失效引用为 0；F15 行为要求失效锚点被检查。
- 实际交付：`layout: home` 的开头 `# Fixture Docs` 或 `# Fixture Docs {#welcome}` 被合并为 hero。check/build 接受同页与跨页 `#fixture-docs`/`#welcome`，但发布的 hero H1 没有 id，浏览器目标不存在。
- 证据链：M3 的 Fragment.anchors → M4/M5 默认首页 → [generation::generate](../../crates/rustify-docs/src/generation.rs)删除开头 H1；[shell::home_layout](../../crates/rustify-docs/src/shell.rs)另写无 id 的 H1 → [本次 root/subpath × 自动/显式四反例](../validation/docs-ssg-p1/prd-dev-gaps/probe-results.json)。均 check/build exit 0；跨页点击后 URL 的 fragment 正确、document.getElementById 为 null，实际 H1 仍无 id。
- 影响：读者从同页、其他页面或分享地址跳转首页标题时定位失败；作者得到“检查成功”却发布坏引用。
- 分类依据：首页合并标题是既有设计，保留被合并标题的原 id 可同时维持单 H1 与锚点契约，无需新的产品决策。
- 处置与 Follow-up：合并时把自动或显式 id 转交 hero，保证分析、最终 HTML 与 DOM 一致。两 base 的同页、跨页和直接深链接都验；含模板内幽灵 id 的拒绝/旧产物保全继续成立。交给 gaps M1。

#### G-04 · 可定位的内容错误仍输出空行列与空修复建议 · A

- PRD 期望：[WORK-F15 行为及 AC-2](../PRD-DOCS-SSG.md#work-f15-检查内容并给出可执行的诊断)、[WORK-N6/AC-3](../PRD-DOCS-SSG.md#work-n6-构建可复现与失败安全)：诊断有适用的源码行列、问题说明和修复建议，机器消费者也可用。
- 实际交付：frontmatter 第 4 行 `lang: zh_CN` 被正确拒绝，但 `lang` 错误的 line、column、suggestion 全为 null。该字段有确定的源码位置，也可给出使用 `zh-CN` 等合法语言标签的具体建议。
- 证据链：M3/V-4、M7/V-10 → [content::analyze 的 lang 分支](../../crates/rustify-docs/src/content.rs)使用 `Diagnostic::in_file`；该构造沿用 general 的空位置/建议 → [本次命令与源文件](../validation/docs-ssg-p1/prd-dev-gaps/invalid-lang.json)。CLI JSON 可解析、exit 1，拒绝本身有效；缺口是诊断可操作性。
- 影响：内容贡献者和编码代理无法直接跳到出错字段，dev 面板也拿不到行列和修复步骤。
- 分类依据：原计划 §5.1 对非源码故障允许 null，不能据此豁免存在明确源码位置的语义错误；作者参考关于“无更具体建议时可为 null”的说明也不适用于本例。
- 处置与 Follow-up：保留 frontmatter 语义错误的 span，沿用 LineIndex/Diagnostic::located；有明确修复方法的规则提供建议。审计本次触及的同类内容诊断，不强行为没有源码位置的 I/O 故障造位置。check/build/dev、中文前缀列号、JSON/text 均验。交给 gaps M1。

### B · 合理变更（2）

#### G-05 · Firefox 从 P1 验收矩阵后移 · B

- PRD 期望：WORK-N3/AC-4 和 §5.2 Q3 包含桌面 Firefox 最新稳定版。
- 实际交付：P1 保留 Chrome、macOS Safari 和手机；Firefox 稳定版与 Playwright Firefox 引擎验证都后移。不能把 Firefox 记为通过。
- 证据链：原计划 §9.3、§11 → [M5 的用户决定](../validation/docs-ssg-p1/M5.md#v-12-早期冒烟与复验2026-10-1011)、[M8 未完项](../validation/docs-ssg-p1/M8.md#未完项与继续动作) → 用户 2026-10-11 明确决定本期不做 Firefox。已验证的替代矩阵见 M5/M8；其中手机细节仍有 U-01。
- 影响与分类依据：P1 不承诺 Firefox 可用，后续需恢复验证；这是有明确用户批准的承诺变更，环境启动失败本身不是批准理由。
- 处置：沿用 D-01，不重复提问。Follow-up 是把本期例外同步到 PRD/验收口径、保留后续追踪及 P4 N3/AC-6 的 Firefox 终验。没有依据指定它在 P2 恢复。交给 gaps M2 文档同步。

#### G-06 · 性能浏览器和 Slow 4G 口径改为冻结的 Chromium/Lighthouse 参数 · B

- PRD 期望：N1 的 Chrome 稳定版、DevTools Slow 4G；Q5 允许在计划时确认测量合同。
- 实际交付：性能用 Playwright 1.63.0 固定的 Chromium 153.0.8010.12；通过 CDP 施加 latency 150 ms、下行 209,715 B/s、上行 96,000 B/s、CPU 4 倍。它不是 Google Chrome 稳定版或未经改动的 DevTools Slow 4G。兼容性稳定版走查另记，数值预算和五次中位数不放宽。
- 证据链：原计划 §9.3 的用户 2026-10-10 确认 → [M2](../validation/docs-ssg-p1/M2.md)、[M8](../validation/docs-ssg-p1/M8.md) → [实际测量条件](../validation/docs-ssg-p1/m8/load.json)与[harness](../../tests/docs-ssg/harness/measure-load.mjs)。R5 的性能与本次 binary hash 相同，见 §5。
- 分类依据：有明确用户批准，冻结条件可复现，保留冷缓存、375px、CPU4倍、每类最大页和既有预算。不得在报告中把历史 Chromium 数据改称品牌 Chrome 实测。
- 处置：沿用 D-02。Follow-up 是同步 PRD 的正式测量合同与兼容性矩阵的区别；交给 gaps M2。

### C · 需 Triage（0）

没有尚待产品取舍的确认差异。U-01 是缺少实际设备验证事实，用户的矩阵偏好不能证明过去已验证；U-02 是原计划 M9 已登记的未完验证。两者均不强行归 A/B/C，不因未知宣称功能未实现。

## 3. 完整需求覆盖账本

证据标记：**本测** = 本次实际命令/浏览器；**静核** = 当前源码核实；**历史** = 原记录及其基线，只在相关路径/输入未变的范围复用。下表逐行覆盖 65 条 P1 AC；没有把跨期需求整体记为完成。范围外去向和 UAT 见本节末尾。

本次共同证据：[Rust 与构建](../validation/docs-ssg-p1/prd-dev-gaps/checks.json)、[四个浏览器项目](../validation/docs-ssg-p1/prd-dev-gaps/browser-commands.json)、[反例](../validation/docs-ssg-p1/prd-dev-gaps/probe-results.json)、[诊断反例](../validation/docs-ssg-p1/prd-dev-gaps/invalid-lang.json)、[依赖图](../validation/docs-ssg-p1/prd-dev-gaps/dependency-checks.json)。

| PRD 需求/AC及定位 | 本期条件/已批准变更 | 计划/里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| [WORK-F1/AC-1](../PRD-DOCS-SSG.md#work-f1-安装-cli-并新建可运行的站点工程)（PRD 第187行） | 原样 new→dev，首页及两指南，无站点编译 | M4/M6/M8 | [M4](../validation/docs-ssg-p1/M4.md)、[M6](../validation/docs-ssg-p1/M6.md)、[M8](../validation/docs-ssg-p1/M8.md) | 静核 new/dev；本测 new_build/docs-dev/dev-uat | 已满足 | U-02 安装行为仍属 K-01 |
| [WORK-F1/AC-3](../PRD-DOCS-SSG.md#work-f1-安装-cli-并新建可运行的站点工程)（PRD 第189行） | 缺 rustc 时先失败并给补装入口，不留残留 | M4/M6/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M6](../validation/docs-ssg-p1/M6.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测 new_build/dev 的受限 PATH 与无残留 | 已满足 | — |
| [WORK-F1/AC-4](../PRD-DOCS-SSG.md#work-f1-安装-cli-并新建可运行的站点工程)（PRD 第190行） | 非空目录拒绝且原树不变 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测 new_build | 已满足 | — |
| [WORK-F2/AC-1](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)（PRD 第214行） | GFM/五 alert/脚注/任务的语义与样式；GitHub同源阅读 | M3/M5/M9 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md)、M9 未建记录 | 本测库/生成及93浏览器；历史 M3 本地格式检查；远端 GitHub未验 | 部分满足 | U-02 / 原 M9 |
| [WORK-F2/AC-2](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)（PRD 第215行） | 相对 .md#anchor 到目标标题，GitHub也可达 | M3/M5/M9 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md)、M9 未建记录 | 本测普通/中文锚点有效；额外 home 四反例目标丢失 | 部分满足 | G-03；U-02 / M9 |
| [WORK-F2/AC-3](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)（PRD 第216行） | 中文/重复/显式标题 id 稳定且优先 | M3/M5 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测库的 slug/显式/重复通过；home显式id在生成后丢失 | 部分满足 | G-03 |
| [WORK-F2/AC-4](../PRD-DOCS-SSG.md#work-f2-用-markdown-编写内容页)（PRD 第217行） | 草稿 dev 标识可见、production/sitemap排除 | M3/M4/M6 | [M3](../validation/docs-ssg-p1/M3.md)、[M4](../validation/docs-ssg-p1/M4.md)、[M6](../validation/docs-ssg-p1/M6.md) | 本测 artifacts/dev/docs-dev；静核 production筛选 | 已满足 | 搜索/AI排除在P4复核 |
| [WORK-F3/AC-1](../PRD-DOCS-SSG.md#work-f3-展示可读可复制与源码同步的代码块)（PRD 第237行） | 11语言构建期高亮、双态配色及对比度 | M2/M3/M5 | [M2](../validation/docs-ssg-p1/M2.md)、[M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测134库测试含11语法与配色；浏览器无脚本/span/双态 | 已满足 | — |
| [WORK-F3/AC-2](../PRD-DOCS-SSG.md#work-f3-展示可读可复制与源码同步的代码块)（PRD 第238行） | 复制精确原文；成功/拒绝状态可被朗读、可手选 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测浏览器copy/refused；静核独立raw template与aria-live | 已满足 | VoiceOver专项终验见N5/P4 |
| [WORK-F3/AC-3](../PRD-DOCS-SSG.md#work-f3-展示可读可复制与源码同步的代码块)（PRD 第239行） | 标记片段随源码变化；缺区间定位失败 | M3/M6 | [M3](../validation/docs-ssg-p1/M3.md)、[M6](../validation/docs-ssg-p1/M6.md) | 本测include/生成/CLI与docs-dev、dev-uat片段保存 | 已满足 | — |
| [WORK-F3/AC-4](../PRD-DOCS-SSG.md#work-f3-展示可读可复制与源码同步的代码块)（PRD 第240行） | 分组选择、同名同步、跨页记忆 | M3/M5 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测三种部署的code group/键盘/跨页 | 已满足 | — |
| [WORK-F6/AC-1](../PRD-DOCS-SSG.md#work-f6-用定制页和布局插槽扩展站点)（PRD 第310行） | 首页frontmatter标题/标语/两操作/特性，1280首屏可见 | M4/M5 | [M4](../validation/docs-ssg-p1/M4.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测new及默认home；实际1280截图 | 已满足 | 定制页/插槽P2 |
| [WORK-F7/AC-1](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)（PRD 第334行） | 侧栏声明分组/排序，当前页标识与可见 | M3/M5 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md) | 静核 navigation/scrollTop；本测nav/frontmatter与sidebar | 已满足 | — |
| [WORK-F7/AC-2](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)（PRD 第335行） | 目录点击与直接fragment位于页头下方 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测普通/中文标题；home已接受fragment无目标 | 部分满足 | G-03 |
| [WORK-F7/AC-3](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)（PRD 第336行） | 滚动目录高亮首/末及当前节 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测scrollspy；静核site.js规则 | 已满足 | — |
| [WORK-F7/AC-4](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)（PRD 第337行） | 禁脚本时导航/目录/前后页是普通链接 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测root/subpath/meta禁脚本路径 | 已满足 | — |
| [WORK-F7/AC-5](../PRD-DOCS-SSG.md#work-f7-浏览站点导航与页内目录)（PRD 第338行） | 任意深缺失地址真404、同样式/脚本/回首页 | M4/M5 | [M4](../validation/docs-ssg-p1/M4.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测三种部署深404及独立模板404 | 已满足 | G-01 地标单独归N5 |
| [WORK-F8/AC-1](../PRD-DOCS-SSG.md#work-f8-统一站点外观主题与暗色模式)（PRD 第361行） | 模式持久、首帧、system及存储拒绝 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测三个部署的first frame/system/storage | 已满足 | — |
| [WORK-F8/AC-2](../PRD-DOCS-SSG.md#work-f8-统一站点外观主题与暗色模式)（PRD 第362行） | 1280三栏、768–1279两栏、更窄单栏 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测1280/1024/768/640/375/320及截图 | 已满足 | — |
| [WORK-F8/AC-3](../PRD-DOCS-SSG.md#work-f8-统一站点外观主题与暗色模式)（PRD 第363行） | 成对品牌主色/logo，链接/焦点/按钮双态 | M4/M5 | [M4](../validation/docs-ssg-p1/M4.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测artifacts logo pair、品牌/双态contrast/focus | 已满足 | 原D11结构 |
| [WORK-F8/AC-4](../PRD-DOCS-SSG.md#work-f8-统一站点外观主题与暗色模式)（PRD 第364行） | 正文英文60–80/中文30–45及层级 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测当前CSS的Range分行：英文65–73、中位69；中文36；实际截图 | 已满足 | 当前macOS/Chromium字体与样本范围 |
| [WORK-F11/AC-1](../PRD-DOCS-SSG.md#work-f11-生成搜索引擎与分享所需的元数据)（PRD 第442行） | 每页title/description/canonical/share、首段摘要 | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 静核 document_titles/page_view；本测artifacts/生成 | 已满足 | 最后更新时间见行为补充 |
| [WORK-F11/AC-2](../PRD-DOCS-SSG.md#work-f11-生成搜索引擎与分享所需的元数据)（PRD 第443行） | sitemap恰为已发布可索引页，robots指向它 | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 本测artifacts；静核sitemap排除draft/noindex/404 | 已满足 | — |
| [WORK-F11/AC-3](../PRD-DOCS-SSG.md#work-f11-生成搜索引擎与分享所需的元数据)（PRD 第444行） | 无site.url可建、无canonical/sitemap且说明配置 | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 本测artifacts及new模板JSON warning | 已满足 | — |
| [WORK-F13/AC-1](../PRD-DOCS-SSG.md#work-f13-在开发预览中随改随看)（PRD 第493行） | 保存Markdown刷新受影响页、保滚动、无编译≤1秒 | M6/M8 | [M6](../validation/docs-ssg-p1/M6.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测docs-dev/dev-uat；历史R5保存中位168.3ms | 已满足 | — |
| [WORK-F13/AC-2](../PRD-DOCS-SSG.md#work-f13-在开发预览中随改随看)（PRD 第494行） | 坏链接诊断面板/终端定位，修复自动清除 | M6 | [M6](../validation/docs-ssg-p1/M6.md) | 本测CLI/dev及浮层恢复；静核SSE状态 | 已满足 | 其他语义诊断缺口G-04 |
| [WORK-F13/AC-3](../PRD-DOCS-SSG.md#work-f13-在开发预览中随改随看)（PRD 第495行） | 默认端口占用自动回退、保留占用者 | M6 | [M6](../validation/docs-ssg-p1/M6.md) | 本测CLI dev端口回退 | 已满足 | — |
| [WORK-F14/AC-1](../PRD-DOCS-SSG.md#work-f14-构建静态产物并按生产条件预览)（PRD 第516行） | 子路径所有页面/资源/链接生成目录index | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 本测artifacts与root/subpath/meta HTTP；静核URL builder | 已满足 | — |
| [WORK-F14/AC-2](../PRD-DOCS-SSG.md#work-f14-构建静态产物并按生产条件预览)（PRD 第517行） | 资源内容指纹稳定，只影响引用链 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测artifacts图片/logo/CSS引用链，静核资源图 | 已满足 | 固定地址static副本是已记录兼容用途 |
| [WORK-F14/AC-3](../PRD-DOCS-SSG.md#work-f14-构建静态产物并按生产条件预览)（PRD 第518行） | preview完整CSP、真404、base mismatch拒启 | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 本测HTTP与reliability，生产manifest模式/锁 | 已满足 | — |
| [WORK-F14/AC-4](../PRD-DOCS-SSG.md#work-f14-构建静态产物并按生产条件预览)（PRD 第519行） | 成功结论页数/总大小/最大5页/警告；失败错误/首错 | M4 | [M4](../validation/docs-ssg-p1/M4.md) | 本测reliability摘要、JSON/失败保全 | 已满足 | — |
| [WORK-F15/AC-1](../PRD-DOCS-SSG.md#work-f15-检查内容并给出可执行的诊断)（PRD 第542行） | 已知失效锚点/缺图/内联script恰3错、定位且build失败 | M3 | [M3](../validation/docs-ssg-p1/M3.md) | 本测CLI check three-errors | 已满足 | home失效锚点的全量检查行为仍有G-03 |
| [WORK-F15/AC-2](../PRD-DOCS-SSG.md#work-f15-检查内容并给出可执行的诊断)（PRD 第543行） | 每条机器诊断含严重度/规则/源码行列/说明/建议 | M3 | [M3](../validation/docs-ssg-p1/M3.md) | 本测标准JSON可解析；lang错误位置/建议为null | 部分满足 | G-04 |
| [WORK-F15/AC-5](../PRD-DOCS-SSG.md#work-f15-检查内容并给出可执行的诊断)（PRD 第546行） | 默认check断网完成、不发外部请求 | M7 | [M7](../validation/docs-ssg-p1/M7.md) | 静核external旗标门；历史M7/R4禁网五命令及默认零请求 | 已满足 | U-02 最终安装交付单独重验 |
| [WORK-F17/AC-1](../PRD-DOCS-SSG.md#work-f17-应用开发者按需启用-gpu-region)（PRD 第592行） | 默认DOM正常/build图与产物无Makepad | M1 | [M1](../validation/docs-ssg-p1/M1.md) | 静核features；历史M1仓内外graph/产物，后续匹配依据 | 已满足 | — |
| [WORK-F17/AC-2](../PRD-DOCS-SSG.md#work-f17-应用开发者按需启用-gpu-region)（PRD 第593行） | toast/resize/监听随abort，DOM与GPU同合同 | M1 | [M1](../validation/docs-ssg-p1/M1.md) | 静核中性runtime接线；历史M1仓内外DOM12/12 | 已满足 | — |
| [WORK-F17/AC-3](../PRD-DOCS-SSG.md#work-f17-应用开发者按需启用-gpu-region)（PRD 第594行） | plain Cargo/bindgen启动、无Makepad打包，实例失败隔离 | M1 | [M1](../validation/docs-ssg-p1/M1.md) | 静核DOM loader/snippet；历史M1仓外配方/两实例 | 已满足 | 远端最终同SHA消费者仍归M9 |
| [WORK-F17/AC-4](../PRD-DOCS-SSG.md#work-f17-应用开发者按需启用-gpu-region)（PRD 第595行） | 7 GPU示例显式gpu、全部原浏览器回归通过 | M1 | [M1](../validation/docs-ssg-p1/M1.md) | 静核7清单；历史604适用原回归，3 headed-only按原范围跳过 | 已满足 | 本次未重跑GPU |
| [WORK-F17/AC-5](../PRD-DOCS-SSG.md#work-f17-应用开发者按需启用-gpu-region)（PRD 第596行） | 无gpu的GPU API编译失败，quickstart/API写开关 | M1 | [M1](../validation/docs-ssg-p1/M1.md) | 静核feature gate/API/quickstart；历史negative/opt-in编译 | 已满足 | — |
| [WORK-N1/AC-1](../PRD-DOCS-SSG.md#work-n1-读者端加载与交互性能)（PRD 第622行） | 内容页首次wasm0、五种基础交互可用 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测三部署模式/copy/groups/menu/spy及wasm0断言 | 已满足 | — |
| [WORK-N1/AC-2](../PRD-DOCS-SSG.md#work-n1-读者端加载与交互性能)（PRD 第623行） | 冻结移动条件下各类最大页LCP≤2.5s、CLS≤0.1 | M5/M8 | [M5](../validation/docs-ssg-p1/M5.md)、[M8](../validation/docs-ssg-p1/M8.md) | 历史原始load5次/页：首页724ms、内容740ms、404532ms，CLS0 | 已确认变更 | G-06 / 冻结测量合同 |
| [WORK-N1/AC-3](../PRD-DOCS-SSG.md#work-n1-读者端加载与交互性能)（PRD 第624行） | HTML+CSS+JS gzip-9≤100KB，排除项遵PRD | M8 | [M8](../validation/docs-ssg-p1/M8.md) | 历史R5同binary性能：最大内容26388B，首页22475B | 已满足 | 历史语料与原机器范围 |
| [WORK-N2/AC-1](../PRD-DOCS-SSG.md#work-n2-作者迭代与构建速度)（PRD 第646行） | B站正文/配置/静态保存≤1秒 | M6/M8 | [M6](../validation/docs-ssg-p1/M6.md)、[M8](../validation/docs-ssg-p1/M8.md) | 历史R5同binary5次中位168.3/159.2/157.8ms；本测dev主路径 | 已满足 | — |
| [WORK-N2/AC-2](../PRD-DOCS-SSG.md#work-n2-作者迭代与构建速度)（PRD 第647行） | new→可打开首页≤60秒 | M8 | [M8](../validation/docs-ssg-p1/M8.md) | 本测独立new/dev五次，页内绘制见证、校准时钟，中位137.3ms | 已满足 | 历史单样本333ms不作为五次合同证明 |
| [WORK-N2/AC-3](../PRD-DOCS-SSG.md#work-n2-作者迭代与构建速度)（PRD 第648行） | B500页空输出完整build≤20秒 | M8 | [M8](../validation/docs-ssg-p1/M8.md) | 历史R5同binary完整5次中位422.7ms | 已满足 | — |
| [WORK-N3/AC-1](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容)（PRD 第669行） | 静态根/子路径可用、深404、失效引用0 | M4/M8 | [M4](../validation/docs-ssg-p1/M4.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测三部署普通路径通过；四home反例发布坏锚点 | 部分满足 | G-03 |
| [WORK-N3/AC-2](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容)（PRD 第670行） | header完整/meta去frame-ancestors零违规、无需隔离头 | M4/M8 | [M4](../validation/docs-ssg-p1/M4.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测root/subpath/meta CSP、全部基础交互 | 已满足 | — |
| [WORK-N3/AC-3](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容)（PRD 第671行） | 默认浏览只向本站源请求 | M4/M8 | [M4](../validation/docs-ssg-p1/M4.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测HTTP爬取/同源与请求失败检查 | 已满足 | — |
| [WORK-N3/AC-4](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容)（PRD 第672行） | 桌面稳定版及iOS Safari/Android Chrome本期路径 | M5/M8 | [M5](../validation/docs-ssg-p1/M5.md)、[M8](../validation/docs-ssg-p1/M8.md) | 历史Chrome/Safari/未知手机；本测Chromium仿真；Firefox有豁免，修复后真机缺证 | 部分满足 | G-05 已批准；U-01待补证 |
| [WORK-N4/AC-1](../PRD-DOCS-SSG.md#work-n4-不含-makepad最小工具链)（PRD 第690行） | CLI/库/生成站正常/build依赖无Makepad | M1/M7 | [M1](../validation/docs-ssg-p1/M1.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测两docs包all-target图0禁用包；历史M1 DOM图、静核features | 已满足 | P2组件模板/官方站须重验 |
| [WORK-N4/AC-2](../PRD-DOCS-SSG.md#work-n4-不含-makepad最小工具链)（PRD 第691行） | 站点产物无Makepad/WebGL/GPU资源 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测artifacts a_built_site_carries_no_gpu_runtime | 已满足 | P2新增站点仍须重验 |
| [WORK-N4/AC-3](../PRD-DOCS-SSG.md#work-n4-不含-makepad最小工具链)（PRD 第692行） | macOS只Rust+CLI、无Node/浏览器/容器五命令 | M7 | [M7](../validation/docs-ssg-p1/M7.md) | 历史M7/R4受限PATH+plain Cargo安装后五命令；静核原生资产/服务 | 已满足 | 当前0.1.2最终主树安装仍有U-02 |
| [WORK-N4/AC-4](../PRD-DOCS-SSG.md#work-n4-不含-makepad最小工具链)（PRD 第693行） | 获取依赖后断网build/check成功 | M7 | [M7](../validation/docs-ssg-p1/M7.md) | 历史M7/R4 sandbox禁网、external默认门未变 | 已满足 | M9安装候选重验 |
| [WORK-N5/AC-1](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)（PRD 第709行） | 浅深默认正文/界面4.5:1、大字/图形3:1 | M5/M8 | [M5](../validation/docs-ssg-p1/M5.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测库调色板、三个部署contrast/hover/focus | 已满足 | — |
| [WORK-N5/AC-3](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)（PRD 第711行） | 单H1、页头/nav/main/footer地标、lang、跳级check | M3/M5 | [M3](../validation/docs-ssg-p1/M3.md)、[M5](../validation/docs-ssg-p1/M5.md) | 本测lang与默认单H1通过；raw多H1/跳级及地标反例失败 | 部分满足 | G-01、G-02 |
| [WORK-N5/AC-5](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)（PRD 第713行） | 320px/200%无页面横滚，code/table内部滚，减动效 | M5 | [M5](../validation/docs-ssg-p1/M5.md) | 本测三部署宽度/200%等效布局/reduced motion；历史Safari真实缩放 | 已满足 | 仿真宽度与历史真实缩放分开 |
| [WORK-N5/AC-6](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验)（PRD 第714行） | 默认各页类型自动化AA重大/严重0 | M5/M8 | [M5](../validation/docs-ssg-p1/M5.md)、[M8](../validation/docs-ssg-p1/M8.md) | 本测三部署及dev axe规则0；不覆盖未断言的地标/作者raw输入语义 | 已满足 | G-01/G-02仍需独立语义验收 |
| [WORK-N6/AC-1](../PRD-DOCS-SSG.md#work-n6-构建可复现与失败安全)（PRD 第728行） | 同输入版本连续构建逐字节相同 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测reliability两构建与生成稳定 | 已满足 | — |
| [WORK-N6/AC-2](../PRD-DOCS-SSG.md#work-n6-构建可复现与失败安全)（PRD 第729行） | 错误失败旧树不变，不部分发布 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测reliability/check/artifacts/dev；历史M7杀进程/磁盘/权限注入 | 已满足 | — |
| [WORK-N6/AC-3](../PRD-DOCS-SSG.md#work-n6-构建可复现与失败安全)（PRD 第730行） | 每条错误有适用位置/说明/修复建议、非零退出 | M3/M7 | [M3](../validation/docs-ssg-p1/M3.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测standard errors通过；lang第4行却缺位置/建议 | 部分满足 | G-04 |
| [WORK-N6/AC-4](../PRD-DOCS-SSG.md#work-n6-构建可复现与失败安全)（PRD 第731行） | 1000次逐次保存可见、不重启 | M8 | [M8](../validation/docs-ssg-p1/M8.md) | 历史M8原始1000/1000、未重启；R5相同watch/harness+短轮补验 | 已满足 | 本次未重跑长轮 |
| [WORK-N7/AC-1](../PRD-DOCS-SSG.md#work-n7-产物安全与读者隐私)（PRD 第745行） | 发布白名单，无源Markdown/私有文件/草稿/未引用源码 | M4/M7 | [M4](../validation/docs-ssg-p1/M4.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测artifacts/库私有/出界拒绝，静核snapshot/generate | 已满足 | AI输出P4复核 |
| [WORK-N7/AC-2](../PRD-DOCS-SSG.md#work-n7-产物安全与读者隐私)（PRD 第746行） | 产物无机器绝对路径/用户名/env值 | M7 | [M7](../validation/docs-ssg-p1/M7.md) | 静核相对诊断/manifest；历史M7金丝雀及R6安全复核 | 已满足 | 不把作者主动正文中的字符串当框架泄漏 |
| [WORK-N7/AC-3](../PRD-DOCS-SSG.md#work-n7-产物安全与读者隐私)（PRD 第747行） | raw HTML内联script/event/style拒绝且定位，CSP不放宽 | M3/M7 | [M3](../validation/docs-ssg-p1/M3.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测html_check/CLI三个错误及安全/旧树保全 | 已满足 | — |
| [WORK-N7/AC-4](../PRD-DOCS-SSG.md#work-n7-产物安全与读者隐私)（PRD 第748行） | 无第三方追踪/外部字体，新窗外链noopener | M5/M7 | [M5](../validation/docs-ssg-p1/M5.md)、[M7](../validation/docs-ssg-p1/M7.md) | 本测三部署同源/noopener；静核系统字体/生成与作者HTML拒绝 | 已满足 | — |

### 范围外需求与跨期终验

| 需求/剩余 AC | 本期边界与去向 |
| --- | --- |
| WORK-F1 / AC-2→P2、AC-5→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F4 / AC-1→P2、AC-2→P2、AC-3→P2、AC-4→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F5 / AC-1→P2、AC-2→P2、AC-3→P2、AC-4→P2、AC-5→P2、AC-6→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F6 / AC-2→P2、AC-3→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F8 / AC-5→P2、AC-6→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F9 / AC-1→P4、AC-2→P4、AC-3→P4、AC-4→P4、AC-5→P4、AC-6→P4、AC-7→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F10 / AC-1→P3、AC-2→P3、AC-3→P3、AC-4→P3 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F11 / AC-4→P3 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F12 / AC-1→P4、AC-2→P4、AC-3→P4、AC-4→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F13 / AC-4→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F15 / AC-3→P2、AC-4→P3 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-F16 / AC-1→P2、AC-2→P2、AC-3→P2、AC-4→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-N1 / AC-4→P2、AC-5→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-N2 / AC-4→P2 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-N3 / AC-5→P4、AC-6→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |
| WORK-N5 / AC-2→P4、AC-4→P4 | 范围外；遵PRD §7.2–7.3，不算P1遗漏，不用本期局部通过代替终验 |

WORK-F17/WORK-N4 在 P1 验收，P2 新模板、官方站与后续各期仍须复核其依赖和产物。PRD 非目标 N1–N15 保留；没有为组件演示、布局插槽、搜索、多语言、llms.txt、SPA、部署上传或 GPU 组件新增任务。

### 用户旅程与行为补充

| 场景/UAT | 核对结果 | 去向 |
| --- | --- | --- |
| S1/UAT-1 | new→dev→首页及两内容页可用；当前正式安装交付未完成 | K-01/U-02，原M9 |
| S2/UAT-2 | 改段落/图片/源码片段、保滚动、错误收起/重存/恢复通过；GitHub预览未验 | 原M9；G-04补其他语义错误诊断 |
| S4/UAT-4 P1子集 | 无脚本阅读、高亮、代码分组、普通导航与wasm0通过 | 演示与完整降级终验P2/P4 |
| S7/UAT-7 | header/meta/root/subpath/404主路径通过；home坏锚点反例违反零失效引用 | G-03 |
| S10/UAT-10 P1子集 | 恰三错、JSON、非零退出通过；其他有效输入语义和诊断未完整覆盖 | G-02/G-03/G-04 |
| S12/UAT-12 P1子集 | Chromium窄屏/等效缩放通过；旧真机版本不明、修复后页头未复走 | U-01；搜索P4 |
| S13/UAT-13 | 错误build旧树不变、错误摘要通过 | 保留 |
| S14/UAT-14 | 默认DOM/opt-in/negative/GPU回归由M1历史及不变接线支撑 | 原M9最终远端配方 |
| S3、S9、S11及UAT-3/9/11/15 | 演示、官方站、用户15分钟门不在P1 | P2/P4，不生成对应修复任务 |
| S5/S6/S8及UAT-5/6/8 | 搜索、语言、AI索引不在P1 | P3/P4 |

F11 的“最后更新/编辑此页”行为另静核 `history::read` 的一次 git log 与 generation 的UTC日期/编辑前缀，本测 new_build/artifacts 的git日期与无仓库省略覆盖；不把它们遗漏在仅AC元数据的追踪之外。PRD目标G1/G5终局留P2/P4；G2/G3/G4的本期部分仍受G-01–G-04、U-01和K-01限制。

## 4. 决策账本

| 决策 ID | 关联项 | 选择与理由 | 日期与来源 | 验收边界 | 去向 |
| --- | --- | --- | --- | --- | --- |
| D-01 | G-05 | 本期不做 Firefox，留后续期 | 用户 2026-10-11；原计划 §9.3、M5/M8 同轮记录 | P1 Firefox 不计通过；P4 全矩阵终验保留 | gaps M2；后续期追踪 |
| D-02 | G-06 | 固定 Playwright Chromium 153 和 Lighthouse 模拟参数，维持预算 | 用户 2026-10-10；原计划 §9.3、M2 记录 | 性能浏览器与兼容性稳定版分开登记 | gaps M2 |
| D-03 | U-01 | 由 agent 推荐主流手机组合 | 用户 2026-10-12 异步答复：“你推荐一些主流的行了” | 推荐 iPhone/Safari 与 Android/Chrome；记录实际版本；不豁免真机验收，不证明旧走查覆盖两端 | gaps M2 的复走与补证 |

另沿用[原决策 B-03](DOCS-SSG-P1.decisions.md#b-03--p1-cli-支持平台与原生依赖约束d10-第二部分)：P1 CLI 仅承诺 macOS，Linux/Windows 未验证。PRD 原文没有明确 OS 清单，这里作为已确认范围边界登记，不额外造一个产品差异。

推荐手机矩阵与 PRD N3 原范围一致。Safari 随 iPhone 的 iOS 更新交付；Android Chrome 可在其设置中查看实际版本、通过 Play Store 检查更新。这里仅给出验收选择，不在用户设备上执行更新。[Apple 官方说明](https://support.apple.com/en-gb/102665)、[Chrome 官方说明](https://support.google.com/chrome/answer/95414?co=GENIE.Platform%3DAndroid&hl=en)。实际手机型号、版本、测试结果仍待记录。

## 5. 待核实项与审查基线

| ID | 缺失事实与影响 | 已有证据与本次边界 | 补证与去向 |
| --- | --- | --- | --- |
| U-01 | N3/AC-4、UAT-12：旧手机记录未标 iOS/Android、型号和浏览器版本；Safari/手机走查早于 R1 页头链接修复 | 用户确认旧路径通过是有效历史事实；M5/M8/R6 明确页头复走未验。本次 375/320px、theme/CSP/axe 是 Chromium 自动化，不能证明实体设备 | 修复后同一 production 产物，以 D-03 矩阵走阅读、导航、页头、锚点、主题和 404；真实 macOS Safari 补页头。记录 OS/browser/build/入口/结果/截图/console 与失败请求；gaps M2 |
| U-02 / K-01 | M9：当前 0.1.2 的正式主树 SHA 安装、安装后 panic 子集、仓外远端 DOM 配方、GitHub 同源格式/安装仍未完成 | R4 的 0.1.1 临时 file:// 快照安装和 M1 仓外 DOM 是历史演练；本测 0.1.2 是 build 的二进制，没有把它记为 cargo install 交付 | 保留原计划 M9/V-4/V-11/V-2/V-9/V-15；新的候选在 gaps 修复后重定版。先本地具体候选，再按原授权门进行远端；本报告不执行推送或请求泛化授权 |

本任务按[核查技能](../../.agents/skills/review-prd-dev-gaps/SKILL.md)的要求：“完成记录是证据线索，不是免检证明。”、“事实与处置分开”。因此用户选择验收组合后，U-01 仍保留为未补齐的验证事实；设备验收和 M9 在后续计划中承接。

### 输入与验证版本

| 来源 | 版本 / SHA-256 |
| --- | --- |
| `docs/PRD-DOCS-SSG.md` | `c588c505348107acf7e90f1b9bf0a941d28e158abd85a3aeec62a57a130bfbd5` |
| `docs/plan/DOCS-SSG-P1.md` | `566264d75f2ee58b854dd02f04dbbf63e6922301924d68b3ee631cc256e4e7d4` |
| `docs/plan/DOCS-SSG-P1.decisions.md` | `23a95f396d9c00209068994b204300ece6f0f7682abe280801eb719ef84cc2ad` |
| `docs/validation/docs-ssg-p1/M1.md` | `92a248e344fe53ee4dd21f868c2dc4ca5501703917cf8c6738445a0e426930db` |
| `docs/validation/docs-ssg-p1/M2.md` | `3ae28151b77a10d25f82ac7732b22d39d9c326f9160de38744f3c392608a7436` |
| `docs/validation/docs-ssg-p1/M3.md` | `4759d2c56dff8a6f4bd7262104fcee8a823062fa1822d2d306a104e9753cc551` |
| `docs/validation/docs-ssg-p1/M4.md` | `d94666be1585bd4616f42058600bafacb6eebf161a1b9f01769f3bc26ee8a060` |
| `docs/validation/docs-ssg-p1/M5.md` | `2059efcb8e445a44117891689de976d41606c607c413500d811b1815ecdeaf3c` |
| `docs/validation/docs-ssg-p1/M6.md` | `a1ec8d7f91415592be53a6ce0470e743b75a7a487e114e5fd615b673d07a259b` |
| `docs/validation/docs-ssg-p1/M7.md` | `e610026c0279c80e624fcc1717aa8b48a270e92517951161b1b787ed41cb9a55` |
| `docs/validation/docs-ssg-p1/M8.md` | `aa233150f3f6493a5b7b0292cd4570ffdf29fec9e620bb33b17c3762b42ccb03` |
| `docs/plan/DOCS-SSG-P1.code-review.r6.md` | `4d01d77d716785485c3c93207530e55b99901c18544f873ad5ccfe3a5838bf0e` |

398 个相关输入的逐路径 SHA-256 及初始 dirty 清单已保存；集合涵盖 docs/CLI/SDK/components、相关示例、tests、xtask、配置、PRD、原计划及记录，不含本次新报告/计划/证据。聚合采用排序、紧凑 JSON 的 path→hash 映射再 SHA-256。本轮有 dirty 资料参与读取，明确为本地工作树可见；未声明远端同步。

[交付前指纹](../validation/docs-ssg-p1/prd-dev-gaps/scope-end.json)确认 HEAD 相同、398个输入均未变化；原PRD、原计划、完成记录和产品实现未被本次修改。计划校验最终ERROR 0；8个WORK编号告警已按上游账本人工复核，0/2与dirty的提示不代表已开始实施。路径脚本所报旧台账协议是原样协议中的兼容说明，本仓实际入口为.xgent-ai，未使用或创建另一套台账。

当前环境为 Rust 1.99.0、mbx 1.15.0、Node 26.1.0、Playwright 1.63.0 / Chromium 153.0.8010.12。本次使用真实 Playwright 浏览器进程；375px 是仿真视口。浏览器入口来自原项目配置：root 4191、subpath 4192、meta-only 4193；dev 用项目测试创建的临时工程/系统分配端口；额外反例用 `preview --port 0`。

### 本次实际验证与历史复用

| 证据 | 执行/观察 | 结果及适用范围 |
| --- | --- | --- |
| [library-test.log](../validation/docs-ssg-p1/prd-dev-gaps/library-test.log) | `mbx test --locked -p rustify-docs` | 123 单测 + 11 生成集成，134 通过；不证明遗漏的契约已覆盖 |
| [cli-test.log](../validation/docs-ssg-p1/prd-dev-gaps/cli-test.log) | CLI 的 check/new_build/artifacts/reliability/dev 五个集成目标 | 13+5+13+7+13 = 51 通过；未重跑 faults/external/CLI 单元，沿用 R6 与 M7 的相关历史范围 |
| [binary.json](../validation/docs-ssg-p1/prd-dev-gaps/binary.json) | `mbx build --locked -p rustify-docs-cli --profile docs-cli`，version/build-info | 0.1.2、panic unwind；hash `f281a1bd80c44f7032a129bfd93cef37df4b3abdf34aed6de43d08a9282aebb7` |
| [browser-commands.json](../validation/docs-ssg-p1/prd-dev-gaps/browser-commands.json) | 原 docs 配置四项目，显式当前 binary，每命令一个 project，串行 | root/subpath/meta 各24；dev21；93 通过、无失败/跳过。含导航、保存/错误恢复、主题、复制、代码分组、窄屏、无脚本、CSP、axe；真机、VoiceOver 未验 |
| [probe.cjs](../validation/docs-ssg-p1/prd-dev-gaps/probe.cjs)、[结果](../validation/docs-ssg-p1/prd-dev-gaps/probe-results.json) | new 模板、原始 HTML、root/subpath 的首页自动/显式锚点六个临时工程 | G-01–G-03 均复现。锚点 case 的 console/pageerror/失败请求均0；模板故意访问404有相应 console404日志，不算意外资源失败 |
| [invalid-lang.json](../validation/docs-ssg-p1/prd-dev-gaps/invalid-lang.json) | 第4行非法语言的 check JSON | exit1，line/column/suggestion 为 null；G-04 复现 |
| [reading-measure.json](../validation/docs-ssg-p1/prd-dev-gaps/reading-measure.json) | 当前内容页1280px，在已渲染 prose 内放入记录的中英文本，Range逐字符分行；最后不足一行排除 | 576px、16px/28px 系统字体；英文各行65–73字符，中位69；中文36字；满足该环境/样本的 F8/AC-4，不外推到所有 OS 字体 |
| [new-budget.json](../validation/docs-ssg-p1/prd-dev-gaps/new-budget.json) | 实际rustc/CLI的受限PATH，五个独立new/dev工程、冷浏览器上下文；执行new前起计，DOM加载后双rAF页内见证，校准Node/browser epoch偏移 | 192.1/133.9/141.5/137.3/134.7ms，中位137.3ms，≤60s；harness的Node/浏览器不进入作者PATH。补齐N2/AC-2五样本证据，不把历史UAT单样本外推成中位数 |
| [dependency-checks.json](../validation/docs-ssg-p1/prd-dev-gaps/dependency-checks.json) | docs 库/CLI 分别 normal+build、all targets 的实际 tree | makepad*/rustify-makepad/cargo-makepad 为0；未用根 Cargo.lock 文本替代独立图 |
| [detector.json](../validation/docs-ssg-p1/prd-dev-gaps/detector.json) | impeccable context 已解析既有 Read brief；detector 扫 shell/CSS | 主要 findings0、字号 advisory4；阅读主题已有独立字阶，非 PRD 遗漏；不以 detector0 当地标通过 |
| [M1](../validation/docs-ssg-p1/M1.md)、[R5 复用依据](../validation/docs-ssg-p1/code-review-r5-fix/prior-evidence-match.json) | 静核 feature/boot 接线，沿用 SDK 两实例与7示例/GPU历史 | M1 仓内外 DOM12/12、适用 GPU回归604通过是历史结果，本次未重跑 GPU |
| [M8 load](../validation/docs-ssg-p1/m8/load.json)、[R5 性能](../validation/docs-ssg-p1/code-review-r5-fix/performance.json)、[前后对照](../validation/docs-ssg-p1/code-review-r5-fix/performance-comparison.json)、[M8 save](../validation/docs-ssg-p1/m8/save.json) | 阅读原始五次/千次数据与适用性，不重跑性能长轮 | R5 binary与本测hash一致，完整build中位422.7ms、最大内容26388B；M8历史LCP首页724/内容740/404532ms、CLS0，1000/1000保存不重启；R5保留相关路径未变及短轮补验。只在原测环境/语料范围复用，不称本次性能或千次实测 |

已实际查看本次[首页1280px](../validation/docs-ssg-p1/prd-dev-gaps/root-home-auto-1280.png)与[raw-heading 375px](../validation/docs-ssg-p1/prd-dev-gaps/raw-heading-375.png)截图。其他截图仅保存。反例脚本首轮跨页读取有一次早于新 DOM 就绪；已等待 load/可见H1后复测，最终四个锚点反例一致。保留 [首轮结果](../validation/docs-ssg-p1/prd-dev-gaps/probe-results-first.json)，首轮空H1不作为产品事实。

## 6. Follow-up 与 gaps plan

[独立 gaps plan](DOCS-SSG-P1-gaps.md)已生成，状态 **Ready**，实施进度 **0/2，尚未开始**。结构、进度与路径校验见[检查记录](../validation/docs-ssg-p1/prd-dev-gaps/plan-checks.json)及[人工复核](../validation/docs-ssg-p1/prd-dev-gaps/artifact-checks.json)。Ready只表示目标与设计可执行；真机环境是M2出口门。

| G-ID / D-ID / PRD | 确定的增量 | 验收条件 | 依赖/阻塞 | 去向 |
| --- | --- | --- | --- | --- |
| G-01 / N5 AC-3 | 默认三类页完整地标 | 有/无git/edit/top配置，实际banner/navigation/main/contentinfo可用 | 与旧阅读主题集成 | gaps M1 / V-2、V-4 |
| G-02 / N5 AC-3、F15行为 | raw/Markdown有效标题统一校验 | 多H1拒绝、跳级定位及deny-warnings、template隐藏不误报 | 保留HTML安全和foreign边界 | gaps M1 / V-2 |
| G-03 / F2 AC-2/3、F7 AC-2、N3 AC-1 | 首页合并保留自动/显式id | 两base、同页/跨页/直接fragment和无脚本目标真实存在 | generation/shell公共接线 | gaps M1 / V-1、V-4 |
| G-04 / F15 AC-2、N6 AC-3 | 源码语义错误位置与具体建议 | 非法lang指向源字段，JSON/text/dev一致；IO不造假位置 | 原span机制 | gaps M1 / V-3 |
| G-05 / D-01 / N3 AC-4 | 文档同步Firefox本期例外及后续去向 | PRD与已批准计划口径一致，Firefox未记通过，P4终验保留 | 决定已存在 | gaps M2 / V-7 |
| G-06 / D-02 / N1测量合同 | 文档同步冻结测量条件 | Chromium与稳定版验收分开、CDP参数/样本/预算一致 | 决定已存在 | gaps M2 / V-5、V-7 |
| U-01 / D-03 / N3 AC-4 | 主流手机组合与Safari补证 | 修后同产物、实际设备/版本/入口/主路径/页头/console/请求结果齐 | M2前确认设备/真人，缺环境则出口阻塞 | gaps M2 / V-6 |
| K-01 / U-02 / F1行为、F2 GitHub条件、N4 | 原计划已登记的安装/文档/远端交付 | 原M9的本地与远端、安装后和仓外验证 | 推送按原授权门 | **原M9，不重复进gaps plan** |

本次未发现其他已有 gaps plan 承接上述增量。K-01/U-02 全部继续指向原 M9。原计划 8/9 与各里程碑历史自述保留；G-01–G-04 说明其完成证据没有完全满足当前 PRD，本报告和派生台账明确记录该限制，不修改旧完成事实。

下一步先评审独立 gaps plan；修复和对应验收完成、原 M9 收口后，再以原计划、gaps plan 和本报告为输入复核 P1 覆盖。

```text
$review-dev-plan docs/plan/DOCS-SSG-P1-gaps.md
```
