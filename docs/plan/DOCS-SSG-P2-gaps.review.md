# DOCS-SSG-P2-gaps · 开发计划完整评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md) | SHA-256 `7dbbbfbb76374f5e0d19426909d56e26b40e32948421360168279c2cbc714d05`（未跟踪，评审结束时未变） | 全文：§0–§12，M1/M2，V-1–V-5，以及所引用的需求和原计划交接 |

- 评审日期：2026-10-15 +08:00。
- 评审对象 ID：`e50f2cbd-5734-4670-a4d2-31fdae182094`。
- 本次是这份 gaps 计划的首次完整评审，不是任何前轮评审的复审。
- [差异报告](DOCS-SSG-P2.prd-dev-gaps.md)、[原计划](DOCS-SSG-P2.md)和 [PRD](../PRD-DOCS-SSG.md) 只作背景材料，不是本轮的修订目标。

## 1. 确认问题

### [P1] F-01 · 按计划修复后，减少动效对 Switch、Checkbox、Radio、Progress、ColorField 和 Spinner 仍不生效

- **定位**：计划中有五处共同造成这个问题。关联 G-05、WORK-N5/AC-5、M1/M2。
  - 第 117 行（§1.1）和第 239 行（§11）把 CSS 问题只归因于“优先级”，并称“CSS 成因明确”。
  - 第 92 行（D2）和第 159 行（§5.2）要求“复用现有 `[data-rustify-scope] [data-name]` 范围”，并在这条规则上提高优先级。
  - 第 161 行（§5.2 验收）和第 208 行（V-2）只列出 ready/field 启用、hover、页签切换。
- **问题**：照计划实施后，修复只覆盖带 `data-name` 的元素。
  - 只给这条规则加局部优先级，能让 Tabs、Button 这类带 `data-name` 的根元素降到 0ms，因为它们和 G-05 的取证对象是同一类元素。
  - 下面五个子元素不带 `data-name`，自己挂着 Tailwind 过渡类，时长取 `--default-transition-duration: 150ms`，这条规则匹配不到它们：
    - Switch 的滑块 `KNOB`，类为 `transition-transform`；
    - Checkbox 的 `BOX` 和 Radio 的 `DOT`，类为 `transition-colors`；
    - Progress 的填充条 `FILL`，类为 `transition-[width]`；
    - ColorField 的 `input`，类为 `transition-colors disabled:opacity-50`。它在岛接管时会出现和 G-05 Button 相同的启动 opacity 过渡。
  - Spinner 是无限循环的 `rui-spin` 关键帧动画，改 `transition-duration` 停不下它。
  - 这些组件都在官方站演示中。V-2 的场景清单不含它们；而现有 reduce 用例遇到过渡是等它结束，而不是断言没有过渡。结果是 G-05 会被记为完成，而“系统减少动效时文档岛控件没有过渡”这项本期交付在至少五类演示上不成立。
- **级联事实**：“局部优先级”实际可用的手段只有一种。
  - docs.css 以 `layer(components)` 导入，SDK 的 `controls.css` 不分层。按 CSS Cascade 5，提高选择器特异性无法跨层生效，只有分层的 `!important` 能压过未分层规则和 utilities。
  - 另一种思路是在 reduce 下改 `--motion-duration`。但导入主题的投影排在后面并重写这个 token，它也读不到 Tailwind 默认时长，所以不能单独使用。
- **证据**：
  - 规则选择器：[docs.css](../../crates/rustify-docs/assets/docs.css) 第 1260–1264 行和 [controls.css](../../crates/rustify-components/css/controls.css) 第 8–10 行都只匹配 `[data-name]`；Spinner 动画在 controls.css 第 14–16 行。
  - 不带 `data-name` 的过渡元素：
    - [switch.rs](../../crates/rustify-components/src/switch.rs) 第 7、54–58 行；
    - [checkbox.rs](../../crates/rustify-components/src/checkbox.rs) 第 6、77 行；
    - [radio.rs](../../crates/rustify-components/src/radio.rs) 第 12 行；
    - [progress.rs](../../crates/rustify-components/src/progress.rs) 第 6、50–54 行；
    - [color_field.rs](../../crates/rustify-components/src/color_field.rs) 第 41 行。
  - 默认时长：[rustify.css](../../crates/rustify-components/css/rustify.css) 第 18、599–614 行。
  - 导入顺序：[style.rs](../../crates/rustify-docs/src/style.rs) 第 217–221 行，sdk.css 不分层，docs.css 进 components 层。
  - 官方站演示：[choices.rs](../../site/src/demos/choices.rs) 第 14、27 行，[feedback.rs](../../site/src/demos/feedback.rs) 第 12 行，[fields.rs](../../site/src/demos/fields.rs) 第 68 行，[basics.rs](../../site/src/demos/basics.rs) 的 Spinner。
  - 现有 reduce 用例先等过渡结束：[islands.spec.ts](../../tests/docs-ssg/islands.spec.ts) 第 773–783 行，[official.spec.ts](../../tests/docs-ssg/official.spec.ts) 第 281–285 行。
  - 已定的设计合同比计划范围更宽：
    - [DESIGN.docs-ssg.md](../../DESIGN.docs-ssg.md) 第 352 行：“Under `reduce`, SDK component transitions drop to 0ms”；
    - [阅读 brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 第 40 行：“any spinner respects reduced motion”；
    - controls.css 第 12–13 行的注释写明 Spinner 在 scope 无动效时静止。
  - 差异报告原始 CSSOM 只采集了 Tabs 触发器：[reduced-motion.json](../validation/docs-ssg-p2/prd-dev-gaps/reduced-motion.json)。
  - 级联规范：W3C CSS Cascade 5（https://www.w3.org/TR/css-cascade-5/ ，2026-10-15 读取）。
  - facts、architecture、delivery 三个视角各自独立得出同一结论；主 agent 回读了上述代码行。
- **修订**：
  - §1.1 和 §11 补上第二个成因：现有选择器覆盖不全。
  - D2/§5.2 写明以下几点：
    - 机制是在 docs.css 的 reduce 媒体查询内使用 `transition-duration: 0s !important`，并注明改特异性无效。不能用 0.01ms，否则仍会创建 CSSTransition。
    - 作用对象扩大到 scope 内全部后代，例如 `[data-rustify-scope], [data-rustify-scope] *`；或者显式列出上述五个子元素。扩大到全部后代时，作者组件也会一起归零，应在作者参考中说明。
    - 按 DESIGN 与 brief 的既有合同，Spinner 在 reduce 下静止，并写明用哪种声明实现。
  - V-2 改为：在 reduce 下，交互前后对上述控件和 Tabs、Button、ready 接管都断言 `document.getAnimations()` 中有限时长的 CSSTransition 为 0，Spinner 的 CSSAnimation 符合静止约定，断言放在任何“等过渡结束”的辅助函数之前；同时写明现有那两个 reduce 用例要改为“断言无过渡”。正常偏好下仍为 150ms。
- **复核**：
  - 改稿后：D2、§5.2 和 V-2 的机制、范围、Spinner 处理和控件清单，与 DESIGN 第 352 行能够对上。
  - 实施后：在 `components-subpath`（导入主题）和 `official-subpath`（默认主题）的相应页面上，reduce 下交互前后这些元素的 computed `transition-duration` 为 0s，没有 CSSTransition，Spinner 静止；正常偏好下 Switch 等仍有 150ms 过渡。
- **置信度**：高。依据是标记、类串和级联规则这类确定性的静态事实，另有三个视角独立复核。本轮的浏览器复现没有完成，见 §3.2。
- **阻塞**：M1 的 Docs CSS 任务开工前需修订。

### [P2] F-02 · “按声明类型给输入建议”所需的元数据只有顶层字段，内置组件的嵌套字段形状在任何清单里都没有

- **定位**：计划中有四处相关表述。关联 G-01、WORK-F4/AC-3、WORK-F15、M1。
  - 第 141 行（§4 判“已核实足够：冷态 names/fields/types”）。
  - 第 91 行（D1“不维护第二份名字/字段/类型字典”）。
  - 第 135 行（§1.2“宏/parser…不需要新增能力”）。
  - 第 151–152 行（§5.1 要求建议包含“声明类型/默认或必填边界”和“与该字段声明相符的序列化输入建议”）。
- **问题**：
  - 权威清单的字段只有 `name`、规范化后的类型串 `ty` 和默认表达式 `default`，没有必填标志。必填只能从“无默认值且不是 `Option`”推断，遇到类型别名就会失效。
  - 内置组件的 `Vec<TabSpec>`、`Vec<CardSpec>`、`Vec<StepSpec>` 只有顶层字段登记在清单里，内部的 label/content、title/link/description、title/description 不在任何清单中。
  - 差异报告自己的反例 Steps `items` 只报了 “expected a sequence”。要给出“相符的序列化输入”，实施者只有三条路，每条都会和计划冲突或重新做决定：
    - 手写示例，违反 D1；
    - 只回显类型名，计划没说这算不算“具体建议”；
    - 引入 schema 推导，超出 §1.2 的范围。
  - 还有两处未定：
    - 内置组件的错误丢掉了 `SiteError.property`，只定位到围栏起点（第 10 行第 1 列），而注册组件定位到属性行。计划一边要求“保留原行列”，一边要求“字段路径”，没说内置组件用哪种。
    - 顶层缺字段时，serde 的路径只有 `"."`，“保留字段路径”无法照字面实现。
- **证据**：
  - [manifest.rs](../../crates/rustify-docs-syntax/src/manifest.rs) 第 23–27 行：`FieldManifest { name, ty, default }`。
  - [builtins.rs](../../crates/rustify-docs/src/builtins.rs) 第 8–40 行：三个 Spec 是独立的 serde 结构体，`site!` 中只声明 `Vec<…Spec>`。
  - [markdown.rs](../../crates/rustify-docs/src/markdown.rs) 第 1205–1209 行：`Issue::error(declaration.span, "embed-props", error.message)`，没有使用 `error.property`。
  - [site.rs](../../crates/rustify-docs/src/site.rs) 第 25–30 行：`SiteError.property` 是存在的。
  - [rendering.rs](../../crates/rustify-docs-cli/src/rendering.rs) 第 652–676 行：路径为空时回落到声明位置。
  - 反例：[content-diagnostics.stdout](../validation/docs-ssg-p2/prd-dev-gaps/content-diagnostics.stdout)、[component-probe.json](../validation/docs-ssg-p2/prd-dev-gaps/component-probe.json)。
- **修订**：
  - §4 第 1 行改为“已核实缺口：只有顶层”。
  - §5.1 按类型写建议口径：
    - 标量、String、bool、数值、`Option<T>`、`Vec<标量>`：由 `ty` 和 `default` 生成 YAML 形状，并说明是否必填（规则写明为“无默认值且不是 Option”）。
    - 内置复合类型：二选一并写进 D1。一是允许 `builtins.rs` 为每个内置组件就近维护一条示例，并用测试证明它能经真实 `registry()` 渲染成功；二是明确只给类型名加作者参考的链接。
    - 作者自定义复合类型：给出声明类型、默认或必填、serde 期望和注册文件位置，不编造示例。
  - 写明内置组件错误是保留块位置，还是改用 `property` 定位到属性行。
- **复核**：
  - 改稿后：D1、§1.2、§4 和 §5.1 对嵌套类型的口径一致。
  - 实施后：Counter.initial（i32）、Steps.items（内置）和一个作者复合类型，各有一条精确的建议断言和位置断言。
- **置信度**：高，数据形态已逐字段核实。
- **阻塞**：M1 的 G-01 诊断任务实现前需修订。

### [P2] F-03 · 哪些规则要补建议、建议文本长什么样，计划都没定，V-1 只能断言“非空”

- **定位**：
  - 第 151–155 行（§5.1）。
  - 第 207 行（V-1 要求“注册/内置名称、字段/类型/必填/重复键错误有权威候选与建议”）。
  - 第 125–126 行（§1.2 文件面）。
  - 关联 WORK-N6/AC-3、WORK-F15/AC-2、G-01、M1。
- **问题**：
  - 规则覆盖面没有定。同一条预检和嵌入链路上，`unregistered-demo`、`reserved-component` 和 `embed-payload`（包括 V-1 点名的重复键）的 suggestion 目前都为空。
    - 计划正文只覆盖“未注册名和未知字段”，V-1 却要求重复键也有“权威候选”。重复键没有候选可列。
    - 重复键错误来自 `embed.rs`，而这个文件不在 §1.2 的文件面里。
  - 文本形状没有约定：
    - 候选多时怎么处理（官方站有 24 个 demo）：近似名优先、按清单稳定排序、上限加“另有 N 个”，都没说；
    - 是否单行：终端用单行 `= help:` 输出，多行会打乱缩进；
    - 候选是否用反引号：dev 面板会把反引号里的内容渲染成代码。
  - 实施者需要自己决定这些产品文案。V-1 无法写出能区分实现好坏的断言，PRD“每条错误都有修复建议”的覆盖面也无法判定。
- **证据**：
  - [rendering.rs](../../crates/rustify-docs-cli/src/rendering.rs) 第 567–623 行，`located(..., None, ...)` 一律不带建议。
  - [embed.rs](../../crates/rustify-docs/src/embed.rs) 第 28–42 行。
  - [report.rs](../../crates/rustify-docs-cli/src/report.rs) 第 56–57 行。
  - [dev.js](../../crates/rustify-docs/assets/dev.js) 中建议的渲染。
  - 现有建议都是单句祈使句加反引号：[作者参考](../docs-ssg.md) 第 140–160 行。
- **修订**：
  - 在 §5.1 补一张“规则 → 建议模板”短表，覆盖 unregistered-component、unregistered-demo、unknown-property、invalid_props/embed-props（与 F-02 的类型口径合并）、embed-payload 重复键、reserved-component。
  - 约定文本形状：单行，候选用反引号；排序与上限按上文；没有任何注册项时，给出启用 `[rust]` 和注册的步骤。
  - §1.2 补上 `crates/rustify-docs/src/embed.rs`。
  - V-1 改为对每类规则断言精确的模板和候选；或者把重复键从“候选”中拿掉。
- **复核**：
  - 改稿后：每条规则都有可断言的建议形状。
  - 实施后：text、JSON 和 dev 面板对同一错误给出逐字相同的单行建议。
- **置信度**：中。规则缺建议这一点为高；排序、上限属于需要补的契约细节。
- **阻塞**：M1 的诊断实现和 V-1 红例。

### [P2] F-04 · M2 的“已安装 C”复验没有绑定执行入口，命令照抄测到的会是工作树版 CLI

- **定位**：
  - 第 94 行（D4“不拿 dirty 开发版或旧安装充新候选”）。
  - 第 171 行（§5.4“用已有 `docs-pages-prepare.py` 和原 M6 的进程内 Git 传输方式，实际从完整 C 本地安装”）。
  - 第 173 行（“M2 使用实际新安装和真实生成工程跑 V-3/V-5”）。
  - 第 208–211 行（V-2、V-3、V-5）和第 217 行（§9.2）。
  - 关联 C-2、M2。
- **问题**：计划写的入口与实际工具对不上，有四处断点。
  - **安装工具写错**：`docs-pages-prepare.py` 不负责安装，它要求先装好 C。实际安装用的是 `mbx install --git … --rev <C> --locked --profile docs-cli --root <隔离根> rustify-docs-cli`，加进程内 Git insteadOf 重映射，再加 git-fetch-with-cli。这套配方只记在 M6 记录里。
  - **复验默认用工作树版**：V-2、V-3 的命令都没写 `RUSTIFY_DOCS_BIN`。`serve.sh` 和 `dev-components.spec.ts` 在没设这个变量时会重建工作树 CLI。
  - **身份校验拦不住**：CLI 的 `build.rs` 从 `.git/HEAD` 读取 source，不标记 dirty。所以提交 C 之后，工作树构建出的二进制同样报告 source=C，能通过 `cli_pair` 校验，D4 禁止的替代会悄悄发生。
  - **Python 版本**：本机默认 `python3` 是 3.9.6，而 prepare 脚本要 `tomllib`（需要 3.11 及以上），必须把 Python 3.12 前置到 PATH。
  - 另外，“浏览器入口是新的本地固定候选生成站”（第 179 行）在仓内没有现成入口：浏览器 project 测的是按配对 C 投影后的 fixture 和官方站，不是 `new` 生成的工程。
- **证据**：
  - [docs-pages-prepare.py](../../scripts/docs-pages-prepare.py) 第 1–8、136–146 行：报错文字为 “installed CLI --build-info source must equal candidate C”。
  - [serve.sh](../../tests/docs-ssg/serve.sh) 第 8–9、21–26 行。
  - [dev-components.spec.ts](../../tests/docs-ssg/dev-components.spec.ts) 第 14、33 行。
  - [build.rs](../../crates/rustify-docs-cli/build.rs) 第 15 行。
  - [M6 记录](../validation/docs-ssg-p2/M6.md) 第 239 行：实际安装命令，以及 libgit2 失败后改走 git CLI。
  - 差异报告实测用的是已安装的二进制：[browser-command.json](../validation/docs-ssg-p2/prd-dev-gaps/browser-command.json)。
  - 本机 `/usr/bin/python3` 为 3.9.6，`/opt/homebrew/bin/python3.12` 存在。
- **修订**：在 §5.4 和 §9.2 写明：
  - 安装命令和进程级环境：remap、git-fetch-with-cli、PATH 前置 Python 3.12，不改全局 git config。
  - prepare 脚本的定位是“按已安装 C 准备工程”，不是安装工具。
  - M2 的所有 serve/dev 复验都设置 `RUSTIFY_DOCS_BIN=<隔离根>/bin/rustify-docs`，并记录二进制 SHA 和 `--build-info`。
  - 两类工程的分工：`new` 生成工程承担 V-5 的 CLI 级 check/build/dev 和锁不变；浏览器复验承担按 C 投影的 fixture 和官方子路径。
  - 可顺带把差异报告遇到的“过期 worktree 登记导致 server 起不来”（[browser-stale-worktree.json](../validation/docs-ssg-p2/prd-dev-gaps/browser-stale-worktree.json)）写进 §12。
- **复核**：
  - 改稿后：V-2、V-3、V-5 的 M2 命令都带上述前置。
  - 实施后：M2 记录中每个 project 的二进制 SHA 和 source 都等于新 C。
- **置信度**：高，只有“工作树构建能冒充”一项为中高。
- **阻塞**：M2 开工前。

### [P2] F-05 · 制作和提交 C 这一步不归任何里程碑，提交前也没有检查

- **定位**：
  - 第 226 行：M2 的前置写“完整C/锁/模板已准备”，同一行的内容又写“只做允许的C身份差异”。
  - 第 169 行（§5.4“随后制作C”）和第 228 行（提交点）。
  - 第 78 行（C-2“C只改身份/模板锁/安装文案”）。
  - 关联 M1/M2、V-5。
- **问题**：
  - M1 的退出条件只到“库 R 功能冻结”，M2 又要求 C 已经准备好。制作 C 的几步（递增版本、内置 R、生成并审计模板锁、离线 locked metadata/check）不属于任何里程碑，也没有检查 ID 判定，M2 的前置无法由前一个里程碑满足。
  - C-2 的“C 只改五个允许文件”没有检查承载。
  - 完成记录中 C 的 SHA 写在哪一行也没有规定，review-code 按完成记录找提交时可能漏掉 C。
- **证据**：
  - 历史先例：[M6 记录](../validation/docs-ssg-p2/M6.md) 第 24–28 行，C 在提交前单独做过准备与审计（锁只改六个框架源，“允许C产品diff仅五文件”）。
  - 提交 `45878ce3` 恰好改了 5 个文件（root/CLI Cargo、模板锁、framework-revision、`docs/docs-ssg.md`）。
- **修订**：
  - M2 的前置改为“M1 已完成，R 已提交并回填”。
  - M2 内容第一步写成：按 §5.4 制作 C → 审计相对 R 的允许文件和锁（只改六个框架源，第三方不变），并跑 locked offline 检查 → 提交 C → 真实安装。
  - V-5 增加“C 提交前”这个执行时点和对应的审计。
  - 提交点写明 M1 行记 R 的 SHA，M2 行记 C 和证据提交的 SHA。
- **复核**：
  - 改稿后：M2 的前置只引用 M1 的产物。
  - 实施后：`git diff R C --stat` 恰好是允许的文件，M2 记录中有锁审计和离线检查的证据。
- **置信度**：高，同一行内直接矛盾，并有历史流程佐证。
- **阻塞**：M2 开工前。

### [P2] F-06 · dev 错误浮层、稳态 AA 和 impeccable 设计确认，第一次验证都排在 R 冻结之后

- **定位**：
  - 第 225 行：M1 退出只要求 V-1、V-2、V-4，并“库R功能冻结”。
  - 第 228 行：M1 收口时提交 R。
  - 第 209 行：V-3 只在“M2 固定候选集中”执行。
  - 第 177 行：§6 写“设计阶段…用impeccable确认”，但没有对应的里程碑或检查。
  - 第 77 行：C-1。
- **问题**：
  - 以下几项是本次新 UI 风险，第一次验证都安排在 R、C 已提交并安装之后：
    - G-01 的 dev 入口：错误 → 建议 → last-good → 修复后清除，以及长候选在浮层中的表现；
    - G-05 关联的 N5/AC-1、AC-6 稳态 axe 和对比度；
    - 仓库规则要求的 impeccable 设计确认。
  - 计划第 234 行自己也承认，稳态复验可能失败、需要核实成因。
  - 一旦发现要改产品，就得重新冻结 R、重新制作并安装 C，M2 的证据随之作废。原 M6 曾为同类问题连续做了五轮 R/C 配对。
- **证据**：
  - [AGENTS.md](../../AGENTS.md) 第 116–124 行：先用 impeccable 做设计，再在浏览器验收。
  - [serve.sh](../../tests/docs-ssg/serve.sh) 第 21–26 行和 dev spec 默认使用工作树二进制，所以 V-3 的子集在 M1 就能跑。
  - 原 M6 的五轮配对：`8e86626f`、`c1061ced`、`6721dc73`、`d301344e`、`45878ce3`。
- **修订**：
  - M1 开头加入 impeccable 确认：沿用 Read 表面，确认错误建议的展示和 reduce 下的表现。
  - M1 退出时，在提交 R 之前，用工作树二进制跑 V-3 的风险子集：
    - `docs-dev` 的错误路径；
    - `components-subpath`、`official-subpath` 的稳态 axe，覆盖 375/1280、浅/深、默认/导入主题。
  - 把 §9.2 的 fmt、clippy、docs-cli build 写成 M1 退出的明确门。
  - M2 的 V-3 改为在固定候选上复验。
  - 补一条回路规则：M2 发现产品缺陷时回到 M1 修复，形成新的 R′/C′ 并递增 patch 版本，旧证据保留原版本。
- **复核**：改稿后，M1 退出覆盖所有新 UI 行为的首次浏览器证据，M2 只做固定候选复验和配对检查。
- **置信度**：中高。编排事实确定，失败概率是推断。
- **阻塞**：M1 退出（R 冻结）前。

### [P2] F-07 · P2 新类型的稳态 axe 复验既没有仓内入口，也没有扫描清单和稳定判据

- **定位**：
  - 第 130 行（§1.2“新类型稳定帧axe按需要接入”）。
  - 第 161 行（axe 前等待）、第 181 行（§6 走查组合）、第 187 行（§7 N5 目标）。
  - 第 209 行（V-3“用现有@axe-core/playwright”）。
  - 关联 WORK-N5/AC-1、AC-6。
- **问题**：
  - P2 的三个 spec（`official`、`islands`、`custom`）都没有接 axe，现有 axe 只在 P1 的 `dev`、`home`、`theme` spec 里。差异报告的 108 次扫描来自验证目录下的一次性脚本。
  - V-3 写“用现有”，§1.2 又写“按需要”，实施者无从判断下面几件事：
    - 扫哪些路由、状态、视口和主题；
    - 稳定边界用什么条件；
    - 是新写 spec，还是把脚本纳入仓库。
  - AC-6“新增类型严重/重大违规为 0”的覆盖面因此无法复现，最容易出现的结果是只扫 Tabs 和 Button 两页。
- **证据**：
  - 在 `tests/docs-ssg/{official,islands,custom}.spec.ts` 中检索 `AxeBuilder`/`axe-core` 无结果；`dev`、`home`、`theme` 三个 spec 有。
  - 一次性脚本 [check-accessibility.mjs](../validation/docs-ssg-p2/prd-dev-gaps/check-accessibility.mjs)，以及 [accessibility.json](../validation/docs-ssg-p2/prd-dev-gaps/accessibility.json)。
  - `package.json` 中 `@axe-core/playwright` 为 4.13.0。
- **修订**：在 V-3 定义三件事。
  - **扫描清单**：可沿用差异报告的 108 个实例（官方站全部路由 + quickstart + 深层 404，× 浅/深，× 1280/375）；再加上 components fixture 的 loading、failed、client-only、导入主题状态。
  - **稳定边界**：岛 ready，加上 `document.fonts.ready`；正常偏好下有限时长的 CSSTransition 全部 finished；reduce 下不等待，直接扫描。
  - **入口**：写明新 spec 测试放在哪个 project 下，或者把脚本纳入仓库并给出命令。
  - 保留首轮瞬时失败的原红记录。
- **复核**：
  - 改稿后：V-3 有可枚举的清单、入口和判据。
  - 实施后：记录逐实例列出 serious/critical 违规为 0，没覆盖到的明确写成缺口。
- **置信度**：中高。
- **阻塞**：V-3 执行前（按 F-06 的修订会提前到 M1 退出）。

### [P3] F-08 · 诊断的“三入口 × 三种输入”矩阵只写在正文里，V-1 没有承接

- **定位**：
  - 第 154–155 行（§5.1 的第 4、5 条）。
  - 第 207 行（V-1 只写了 check/components 相关用例）。
  - 差异报告第 28 行承诺“check/build/dev 三个入口、普通/中文/CRLF 输入验证一致”。
- **问题**：
  - dev 入口要到 V-3（M2）才第一次被覆盖。
  - build 入口和中文前缀、CRLF、多个同值字段没有落到任何检查的可观察结果里。
- **修订**：V-1 的可观察结果写成：check/build/dev × 普通、中文前缀、CRLF、多个同值字段，原位置、候选和建议一致。命令补上 dev/build 的相关用例，例如 `crates/rustify-docs-cli/tests/dev.rs`。
- **复核**：M1 记录逐格可以追溯。
- **置信度**：中。
- **阻塞**：否。

### [P3] F-09 · 没写明 M1 期间组件工程是按 Git R5 配对运行的，helper 侧的改动在 M1 看不到

- **定位**：
  - 第 98–99 行（§0.4）。
  - 第 169–173 行（§5.4）。
  - 第 207–208 行（V-1、V-2 在 M1 红绿）。
- **问题**：
  - HEAD 的 `framework-revision.txt` 是 R5，工作树 CLI 的 source 取 HEAD。因此 M1 里的组件测试和 `components-subpath` 都会被投影到 Git R5 的 helper（site-ssr、hydrate）和 wasm 上。
  - §1.2 列出的改动都在 CLI 进程内，能在 M1 测到。但如果实施者为了带出类型信息去改 `site/registry.rs` 或 SiteError 这类 helper 侧代码，M1 的红绿结果会是假的。
- **证据**：
  - [framework-revision.txt](../../crates/rustify-docs-cli/assets/framework-revision.txt)。
  - [docs-pages-prepare.py](../../scripts/docs-pages-prepare.py) 的 `cli_pair`。
  - [M6 记录](../validation/docs-ssg-p2/M6.md) 第 10 行：非空 R 会拒绝 path fixture。
- **修订**：在 §0.4 或 §5.4 加一句：G-01、G-05 只改 CLI 进程内的代码；如果必须改 helper，M1 改用临时 R/C 或空 R 的 path 模式验证，并在记录中写明实际配对的 R。
- **复核**：M1 记录注明组件测试实际用的 R。
- **置信度**：高。触发条件是偏离 §1.2，所以定为 P3。
- **阻塞**：否。

### [P3] F-10 · §6 要求“保持”dev 浮层的“复制位置”，这个能力并不存在

- **定位**：第 179 行（“关闭、复制位置、焦点与修复后清错保持”）。
- **问题**：
  - dev 浮层只有这些能力：隐藏或用标记重新打开、Esc 关闭、焦点归还、清错后把焦点交回 main。没有任何复制或剪贴板功能。
  - 它的 brief 也明写不新增动作。实施者要么去测一个不存在的行为，要么在范围外新增 UI。
- **证据**：
  - [dev.js](../../crates/rustify-docs/assets/dev.js) 全文检索 `copy`/`clipboard` 无结果。
  - [dev brief](../../.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md) 第 38 行：“No editor-opening links or extra actions are introduced”。
- **修订**：改为“隐藏/标记重新打开、Esc、焦点不被抢、修复后自动清除”。
- **置信度**：高。
- **阻塞**：否。

## 2. 待核实与保留的验证门

| 项目 | 缺失证据与影响 | 提供者与最晚确认点 | 本轮裁定 |
| --- | --- | --- | --- |
| F-01 的浏览器实测 | 本轮在 C5 官方子路径产物上的 reduce 探针没有执行：产物在评审中途被 mbx 回收了（见 §3.2）。代码和级联事实已足以确认问题 | M1 实施者在红基线时补采，覆盖 Switch、Checkbox、Radio、Progress、ColorField、Spinner 的 computed 值和 `getAnimations()`；最晚在 V-2 红例前 | 不影响 F-01 成立。计划第 49 行本来就要求先复现 |
| 压缩后的 `!important` | rustify_style 的 `@import … layer(components)` 经 lightningcss 压缩后，是否保留 `@media` 内的 `!important` | M1 实施者；V-2 由红转绿之前，用 style.rs 单元测试断言 | 实现期检查，不阻塞改稿 |
| 受影响的 docs-verify 组 | CSS 和诊断的改动是否改变 `artifacts`、`reliability` 组的预期，需要并入 V-5 | M2 实施者按 V-14 和真实影响判断；最晚在 M2 出口前 | 计划第 215 行已有“按实际影响追加”的约定，不另报 |
| 原 M6/M7 缺证 | U-01 至 U-03、K-01/K-02 由原计划承接 | 原计划；原期出口或 P4 发布前 | 计划第 105–107、236 行的排除和交接有效，不阻塞本增量 |

未发现需要用户重新拍板的产品取舍。Spinner 在 reduce 下怎么处理已有设计合同（DESIGN 第 352 行、brief 第 40 行），改稿时按合同写明即可。

## 3. 覆盖、基线与实际验证

调查事实源是 `main` / `d2f20b03cf58c1d1ced3785397c501d412cd4167` 的当前工作树，状态 dirty：

- 未提交的内容是 SDLC 台账、差异报告及其证据、fixture 日志和目标计划本身；
- `crates/`、`site/src/`、脚本和 spec 都没有未提交改动；
- 评审开始和结束时，目标计划、PRD 和原计划的 SHA-256 都没有变化。

背景报告当前的 SHA-256 是 `341778d2…`，与台账形成事件里记录的 `005f957a…` 不同。这份报告只作背景材料，不影响结论。

本轮没有 fetch/pull、提交、推送、部署或实施计划。

| 维度 | 实际负责人与方法 | 结论与限制 |
| --- | --- | --- |
| facts | 独立 agent 静态核查代码、证据和 git 历史；主 agent 回读每个结论依赖的代码行 | §1.1 中的诊断通道、JSON 不加新键、期次账本、Playwright project/spec 名都成立。问题在 CSS 成因不完整、元数据只到顶层、安装入口不对、“复制位置”不存在（F-01、F-02、F-04、F-10） |
| architecture | 独立 agent，查了 CSS Cascade 5 和 Transitions 1 规范（2026-10-15） | D4 的 R→C 配对和 §8 的回滚可行且已验证过；D2 只能用 `!important` 并需扩大范围；D1 对嵌套类型不可行（F-01、F-02、F-09） |
| schedule | 独立 agent 做双向映射和依赖边分析，并把协议原文与骨架逐行对照 | 顶部协议是骨架原文，进度块一致，ID 无悬空，AC 粒度可判定；问题在 C 的制作归属和首验时点（F-05、F-06、F-08） |
| delivery | 独立 agent 做作者、读者、维护者三条主路径推演，读了 UI 上下文和 impeccable 的 audit 条目 | 作者路径的 JSON/text/浮层通道可用；读者 reduce 路径和 M2 的二进制来源有断点（F-01、F-03、F-04、F-07、F-10） |

### 3.1 脚本与命令

| 检查 | 结果 | 人工核对 |
| --- | --- | --- |
| `check_paths.sh` | 共 17 个路径，MISSING 2 个 | `DOCS-SSG-P2-gaps.records/M1.md` 是标了 ★ 的新增记录；`docs/project-status/protocol.md` 是协议中的兼容说明。都不是错误复用 |
| `check_progress.sh` | 0/2，里程碑 2 行，ERROR 0，WARN 1 | 这条 WARN 指工作树里有 65 个未提交改动，都与本计划无关，计划尚未实施 |
| `check_plan.py` | ERROR 0，WARN 5 | 5 个 WORK-* 自定义 ID 都在 §0.1 登记过，PRD 中也能找到出处 |

这些脚本通过，不证明行为真实或验收完成。

### 3.2 实际浏览器证据等级

- **本轮实际执行的浏览器探针：无。**
  - 主 agent 是唯一指定的浏览器操作者，准备了只读探针：在 reduce 下打开 C5 官方子路径的 switch、checkbox、radio、progress、tabs、loading、color-field 页面，注入计划首选的 `[data-name]` + `!important` 修复后，采集交互前后的动画。
  - 确认产物存在（build_id `b9267374bcc32cc6`）之后、复制之前，它被 mbx 的 target 回收删除了。
  - 重新构建需要配对安装和 wasm 编译，超出计划评审的成本，所以没有重建。
- **历史运行记录**：差异报告 2026-10-14 在 C5 上的 [reduced-motion.json](../validation/docs-ssg-p2/prd-dev-gaps/reduced-motion.json)、[accessibility.json](../validation/docs-ssg-p2/prd-dev-gaps/accessibility.json)、[contrast-confirmation.json](../validation/docs-ssg-p2/prd-dev-gaps/contrast-confirmation.json)。只适用于 C5，不代表修复之后。
- **静态截图**：delivery 查看了 `tabs-375-rest.png` 和 `button-375-hover.png`，只能说明布局，不能证明动效。
- 计划描述的修复后行为全部是文档推演，要在实施后由浏览器验收。

## 4. 需求映射与实施编排

**已确认完整的链**：

- G-02 / WORK-N3/AC-4、Q3 → §5.3 → V-4 → M1（M2 再复核）。
- WORK-N6/AC-2、AC-3 → §5.1/§8 → V-1/V-3/V-5。
- C-3 及上游 M6/M7/V-11/V-12/V-15 → §0.5/§11 → V-4（反向确认“没有新增完成声明”）。
- V-14 → §9.2 → V-5。

账本用需求级 ID，但来源列都写到了 AC，可以按 AC 判定。F15/N5/N3 的终验期写在第 63 行，不会被误读为整条需求关闭。

**断链和弱链**：

| 链 | 断点 | 问题 |
| --- | --- | --- |
| G-05 → 全部岛控件 | 选择器、Spinner 和验收清单不全 | F-01 |
| G-01 → 具体建议 | 嵌套类型元数据缺失，规则和文本契约未定 | F-02、F-03 |
| G-01 的 build/dev 入口和中文/CRLF 输入 → V-1 | 只写在正文里 | F-08 |
| C-2 → 制作 C 和提交前审计 | 没有里程碑归属和检查 | F-05 |
| C-1 设计确认、dev 错误路径、稳态 AA → 首次验证 | 晚于 R 冻结 | F-06、F-07 |
| D4 → M2 浏览器证据的二进制来源 | 没有绑定已安装的 C | F-04 |

**修订后建议的依赖与波次**：

| 波次 | 前置及解锁证据 | 可同时进行 | 写入边界 / 负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| W0 红基线 + impeccable 确认（M1） | 无 | W1-C | 主实施者只写记录 | 浏览器每次一个 project；mbx target 串行 | M1 记录中有 G-01/G-05 的红证据和 F-01 的控件清单 |
| W1-A 诊断 | W0；F-02/F-03 契约已改稿 | W1-C | `rendering.rs`、`markdown.rs`、`builtins.rs`、`embed.rs`、CLI 测试；一人负责 | 与 W1-B 共用 rustify-docs crate 和 target，构建串行 | V-1 |
| W1-B reduce | W0；F-01 改稿 | W1-C | `docs.css`、`style.rs`、islands/official spec | `serve.sh` 会重建工作树 CLI，与 W1-A 的编译串行 | V-2 |
| W1-C PRD 同步 | 无，决定已确认 | W1-A、W1-B | 只改 `docs/PRD-DOCS-SSG.md` | 无 | V-4 的 PRD 部分 |
| G1 冻结 R | W1 全绿；作者参考和 DESIGN 已同步；用工作树二进制跑过 V-3 子集；fmt/clippy/build 门通过 | 无 | 主实施者 | 浏览器串行 | 提交 R 并回填 |
| W2 制作 C（M2） | R 的 SHA | 无 | 五个允许文件，单人负责 | target 和 lock 独占 | 审计 diff 和锁、locked offline 检查之后提交 C |
| W3 安装与出口 | C 已提交 | 无 | 只写记录和计划 | 隔离安装根、remap、Python 3.12、`RUSTIFY_DOCS_BIN`；浏览器串行 | V-5、V-3 复验、V-1/V-2 受影响重验、V-4 |
| G2 收口 | W3 全绿 | — | 主实施者 | — | 证据提交并回填；发现产品缺陷时回到 W1，形成 R′/C′ |

真正能安全并行的只有 W1-C：PRD 同步不依赖任何代码，也不占构建资源。W1-A 和 W1-B 共用 crate、target，浏览器运行还要重建 CLI，要并行就得分开 worktree 和 target，代价高于收益。计划第 225 行“由一人改，浏览器与Cargo串行”成立。

瓶颈链：W0 → W1-A/B → G1(R) → W2(C) → W3 → G2。

## 5. 总体结论与修订顺序

**是否需要修改：需要。** 本轮有 1 项 P1、6 项 P2、3 项 P3。

**建议状态：`Blocked`，与计划自报的 `Ready` 不一致。** 阻塞都可以通过改稿解除，不需要用户另行裁定。

- **阻塞依据**：照计划实施 G-05，官方站的五类控件和 Spinner 仍违反 WORK-N5/AC-5，本期三项交付之一不成立（F-01）；G-01 的建议契约在内置复合类型上，与 D1 冲突（F-02）。
- **各项阻塞范围**：
  - M1 的 Docs CSS 开工前：F-01；
  - M1 的诊断开工前：F-02、F-03；
  - M1 退出、冻结 R 之前：F-06、F-07；
  - M2 开工前：F-04、F-05。
- **现在就能开始的工作**：W0 红基线复现，以及 W1-C 的 PRD 同步。它们不受上述问题影响。

**修订顺序**：

1. **先纠正前提**：§1.1、§11 补上 CSS 成因中的选择器覆盖；§4 第 1 行改为“只有顶层”。
2. **再修设计**：D2/§5.2 写明 `!important`、范围和 Spinner 处理（F-01）；D1/§5.1 写明类型分类、内置复合类型的取舍、位置口径和规则模板表，§1.2 补上 `embed.rs`（F-02、F-03）。
3. **然后改验证**：V-2 的控件清单和断言时机；V-1 的入口 × 输入矩阵和精确模板；V-3 的扫描清单、稳定判据、入口和二进制；V-5 的 C 提交前审计（F-01、F-03、F-04、F-05、F-07、F-08）。
4. **最后改编排**：M1 加入 impeccable 确认和 V-3 子集，M2 的前置和内容改写，提交点写明各 SHA 记在哪一行，补上 R′/C′ 回路；§5.4、§9.2 写明安装配方和环境（F-04、F-05、F-06）。
5. **P3 收尾**：F-09 的 M1 测试拓扑，F-10 删掉“复制位置”。

**解除 Blocked 的可观察条件**：改稿后，每条问题的“复核（改稿后）”断言都成立，并且 §11 的定级理由不再写“CSS 成因均明确”。计划评审通过不代表功能已实现，也不代表可以上线。

**下一步**：

```text
/apply-doc-review docs/plan/DOCS-SSG-P2-gaps.review.md
```

## 修订处置与复核

- 来源：本报告首次完整评审，评审对象ID `e50f2cbd-5734-4670-a4d2-31fdae182094`；追加前SHA-256 `f8b6ddc69a9f9ba7cda3ba41b75ebbabab3f8fc978a2ee833adb3fc6019b2777`。以上原问题、所审基线与“需要修改/建议Blocked”结论完整保留。
- 目标：[DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md)；修前SHA-256 `7dbbbfbb76374f5e0d19426909d56e26b40e32948421360168279c2cbc714d05`，与所审版本一致；修后SHA-256 `f75590ffad5d8bbafc1a273c47947c6c24aaa89501fd454a6191f48ec06ddf90`。
- 复核日期与代码基线：2026-10-15 +08:00；`main/d2f20b03cf58c1d1ced3785397c501d412cd4167`，dirty。亲自回读相关代码/配置、需求、设计及历史证据；产品源码无本次改动。
- 范围：F-01–F-10、§2待核实项的实施门及§4断链；首次apply，无上次处置。只修订该增量计划及本处置节，关联PRD、原计划、作者参考、DESIGN和历史证据保持原位。

| 来源问题 | 核实结论与依据 | 当前处置 | 目标修改与关联落点 | 本次复核结果 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| F-01 | 成立。回读docs.css、controls.css、style.rs导入层及switch/checkbox/radio/progress/color_field/spinner源码：子元素漏选与独立动画均存在；历史CSSOM只证明C5的Tabs反例 | 已修订并复核 | 账本G-05、D2、§1.1/§1.2/§4/§5.2/§7/§8、V-2、§11/§12：scope全部后代零duration/零delay的重要声明，Spinner用animation:none；8类控件的即时断言与压缩CSS门 | 已解决（文档）。机制、覆盖面、Spinner静止与既有Read合同一致；reduce断言先于等待/重试 | M1补采漏选控件/Spinner红基线，验证压缩后!important及真实computed/getAnimations；M2固定复验 |
| F-02 | 成立。FieldManifest只有name/ty/default；builtins只登记顶层Vec<Spec>，SiteError.property存在，Markdown当前保留块位置；类型别名无法仅靠ty确定缺值语义 | 已修订并复核 | D1、§0.4/§1.1/§1.2/§4/§5.1、V-1：直接类型/Option/Vec与不透明类型分开；内置复合类型给类型及Static builtins链接；注册错误保留属性定位，内置保留块位置，空/点路径回落 | 已解决（文档）。没有假设嵌套schema或必填标志；完整serde路径在建议附句保留，不伪造内层类型/位置 | M1精确断言Counter.initial、Steps.items及作者复合类型/别名的建议与位置 |
| F-03 | 成立。回读rendering::preflight/located、embed::parse、report::text及dev.js建议展示；名称、字段、reserved、YAML/重复键分支缺少完整建议契约 | 已修订并复核 | §1.2纳入embed.rs；§5.1规则模板、候选/边界/形状及替换项固定；V-1/V-3承接精确文本及长候选/空注册；重复键只给去重步骤 | 已解决（文档）。单行英文、反引号、完整稳定排序与各规则精确展开均可断言，终端/JSON/dev共用一份建议 | M1对全部模板先红后绿，dev只格式化、不另造文案 |
| F-04 | 成立。prepare要求已安装C；serve.sh/dev-components缺少RUSTIFY_DOCS_BIN时会重建CLI；build.rs只读Git HEAD；历史安装记录证明进程remap/git-fetch-with-cli流程，已有Python3.12入口可用 | 已修订并复核 | §5.4/§6、V-2/V-3/V-5、§9.1/§9.2：真实隔离安装命令、Python入口、进程Git环境、二进制SHA/source逐project核对；new工程CLI验收与投影fixture/官方浏览器分工；§12记过期worktree边界 | 已解决（文档）。M2所有浏览器入口必须继承已安装二进制，不能只以source=C认定安装；prepare不再承担安装 | M2保存实际安装命令、摘要及每个project/prepare回执的C/R；本次未安装或启动服务 |
| F-05 | 成立。原M1止于冻结R，M2却同时要求C已准备和制作C；历史M6/C5五文件差异与锁审计流程可回读 | 已修订并复核 | §5.4、V-5、M2及提交点：M2前置只取M1/R，制作C→五文件/六框架source审计→两目标locked offline门→提交→安装；M1行记R，M2行记C及证据SHA | 已解决（文档）。制作C有明确归属和提交前检查，不存在循环前置；进度/证据提交单列，产品差异仍限五文件 | M2实施审计并记录C/证据提交；不把文档修订当提交或交付完成 |
| F-06 | 成立。原V-3首次在M2，而M1已冻结R；AGENTS要求设计与浏览器验收，现有工作树入口能提前验证CLI内增量 | 已修订并复核 | 恢复快照下一步、§5.4/§6/§9.2、V-3、M1/M2：M1开头impeccable，R前完成错误浮层/完整稳态矩阵及fmt/test/clippy/build；M2固定复验，产品缺陷走R′/C′并递增patch | 已解决（文档）。全部本次新UI风险首次验证在R冻结前，有明确失败回路；0/2及尚未开始保留 | M1完成设计与首次浏览器门；M2发现缺陷时保留旧证据并形成新候选 |
| F-07 | 成立。P2三个spec没有axe入口；package.json已有4.13.0依赖；一次性脚本及accessibility.json可枚举27路由/108实例 | 已修订并复核 | §1.2、V-3、§9.1、M1：在既有islands/official/custom新增gaps a11y用例；全部官方路由+quickstart+深层404 ×主题/视口，并枚举fixture的loading/ready/failed/client-only/定制页/404；明确fonts/目标状态/有限过渡边界与命令 | 已解决（文档）。入口、清单、稳定判据与逐实例产物可复现；默认主题门与作者导入主题范围分别记录 | M1实现用例并保存逐实例结果/incomplete，M2在已安装C重验；原瞬时红不改写 |
| F-08 | 成立。原§5.1承诺三入口/多种输入，V-1没有逐格可观察结果；check/components/dev测试文件可用 | 已修订并复核 | V-1、§5.1、M1：check/build/dev × 普通、中文前缀、CRLF、多个同值字段，每格候选、建议与位置一致；命令包含check/components/dev，M1记录逐格追溯 | 已解决（文档）。build/dev与中文/CRLF/同值字段不再仅留正文 | M1真实CLI及dev状态的矩阵证据；M2已安装入口受影响重验 |
| F-09 | 成立。framework-revision为R5；cli_pair从CLI source提交读取配对R，helper/wasm会投影至Git R5；CLI source不代表工作树helper | 已修订并复核 | §0.4、§9.1/§9.2、M1：本增量限CLI进程内；记录实际CLI/R/dirty范围，helper变化须另用临时R/C或独立空R path模式，不以旧R绿例证明 | 已解决（文档）。本期默认测试拓扑和偏离时的验证方式明确，与最终真实R/C出口兼容 | M1记录实际配对；如确需helper改动，先回写影响并补正确拓扑的证据 |
| F-10 | 成立。dev.js没有copy/clipboard动作；回读其隐藏/重新打开、Escape、焦点清理及dev brief；PRD另有位置文字可复制的体验要求 | 已修订并复核 | §6及V-3：删除虚构的“复制位置”动作，列明隐藏/标记/Esc/不抢焦点/修复清除，保留位置文本选中复制 | 已解决（文档）。不再要求保持不存在的按钮，也未删除上游文本可复制要求 | M1/M2浏览器检查现有操作与位置文本，不增新动作 |

### 取舍与未完项

- 无需新的产品决策。内置复合类型采用报告给出的“声明类型+作者参考链接”方案，沿用D1单一事实源；未采纳另写嵌套示例或schema推导。自定义/别名不伪造必填语义。
- 候选采用已有有序集合适合的完整字典序，不引入近似排序、数目上限或“另有N个”。完整清单与长候选换行仍纳入V-1/V-3，不用截断隐藏候选。
- C审计区分五个产品文件与顶部协议要求的独立进度/证据提交，避免把SHA回填误判为产品漂移；这些非产品提交必须逐项列出，不能夹带库/运行资源/功能变更。
- reduce同时归零delay，以保证组合时长为零；级联/零组合时长分别回读[CSS Cascade 5 §6.1](https://www.w3.org/TR/css-cascade-5/#cascade-sort)和[CSS Transitions 1 §3](https://www.w3.org/TR/css-transitions-1/#starting)。这是设计依据，不是修复后浏览器证据。
- §2保留的F-01红基线、压缩CSS和受影响docs-verify组已落实到M1/M2验证门。原U-01–U-03/K-01/K-02、设备/正式性能/公开发布仍在原计划，不因本次处置关闭。
- 关联PRD的WORK-N3/AC-4、Q3 Firefox同步，以及作者参考的诊断/作用域说明、DESIGN的真实reduce规则，仍由本计划M1/V-4实施；它们是背景文档，未在本次apply中修改。未运行impeccable实施设计、产品测试/build、真实安装或浏览器，未在浏览器中验证修复后的行为。

### 修订后结论与实际检查

本次10项均为**已修订并复核 / 已解决（文档）**；无需修改0项、不采纳整条问题0项、待决/待补文档证据0项。没有重跑完整独立评审，不改写原结论。当前计划判定**Ready**，设计/前置/检查归属已明确，实施仍为**0/2、尚未开始**；Ready不代表实现、AA或上线完成。

| 本次检查 | 实际结果与范围 |
| --- | --- |
| check_paths.sh（实际skill目录脚本，参数为目标计划/仓根） | 24个路径，MISSING 3：两份★未来M1/M2记录及docs/project-status/protocol.md兼容说明；人工核对不是错误复用，当前台账为.xgent-ai/sdlc |
| check_progress.sh（同上） | 0/2，里程碑2行，完成记录0行，ERROR 0 / WARN 1；警告来自目标未跟踪及既有dirty工作树，不表示本次已实施 |
| check_plan.py（同上） | ERROR 0 / WARN 5；五个WORK自定义ID已在账本登记，逐条与读过的PRD需求/AC核对 |
| shell代码块sh -n、协议/进度与链接人工复核 | 3个shell块语法通过（仅解析，没有执行安装/测试）；实施协议与修前逐字一致，M1/M2及V-1–V-5 ID/0/2保持，需求→设计→任务→验证链已补齐 |
| 差异与边界核对 | 目标按修前快照比较；原报告正文完整保留，关联PRD/原计划/设计/参考与产品源码不改；本次未提交/推送/部署，SDLC处置接续原评审对象，不新增独立评审 |

**判定：Ready，可按计划开始M1。下一步仅建议，不自动实施：**

```text
/goal 按 docs/plan/DOCS-SSG-P2-gaps.md 开发
```
