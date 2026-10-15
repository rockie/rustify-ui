# DOCS-SSG-P2-gaps · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 · 初审 | [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md) · M1/M2 | `d2f20b03cf58c1d1ced3785397c501d412cd4167..958c9bd902fd847b59093c31d83468fd7bae2b83` 的累计净差异；当前未提交内容单列于 §3 | 无；原 DOCS-SSG-P2 的代码报告属于另一计划 |

- 日期：2026-10-15 +08:00；仓根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。
- 计划版本：HEAD `f5d303d61acf294dff3c3ee75a589085622700b5`，计划 SHA-256 `b193ee9dce5e2ba8ff68b9e3ecc280396232ef67423ab11a94bd784e06dc62e9`。
- 模式：仅提供计划，采用开发计划模式；从完成记录解析实现、配对和证据提交，不改审最近一个提交。
- 呈现路径：DOM 静态文档站与 Leptos 岛。变更未新增 GPU 或跨区协作；本轮未重验 GPU。
- 评审方式：当前主 agent 单人评审；源码、计划和既有报告只读。新增本报告、验证证据及台账记录，未修复、提交或发布。

## 1. 确认问题

### F-01 · [P2] 按完整 raw string 分隔符保留默认值字面量

- 定位：[crates/rustify-docs/src/embed.rs](../../crates/rustify-docs/src/embed.rs)，target 侧 **357–358 行**，版本 `958c9bd902fd847b59093c31d83468fd7bae2b83`；这两行在本次新增 diff 内。
- 问题：作者在 `site!` 中使用合法的 raw string 默认值，例如 `label: String = r#"a"   b   "c"#.to_string()`，随后触发未知属性或属性类型错误时，新的 `pieces()` 将 raw string 内的引号当作普通字符串的终止符，字面量中部分文本随即被作为普通 token 拆分、压缩空格，最终建议显示 `r#"a" b "c"#.to_string()`。这两个表达式的字符串值不同，导致修复建议陈述了错误的默认值；共用这段建议生成逻辑的 check/build/dev 均受影响。注册解析器接受该声明；字面量内部空白属于值的内容，该结果违反本计划 §5.1 第 1 条“字符串/字符字面量原样”及默认表达式忠实显示的承诺。
- 证据：真实 `rustify-docs-syntax::parse_registry` 的 manifest 保留原字面量；将本轮 `Field`、建议生成和 `display/pieces` 函数原样提取到隔离工程运行后，`manifest_default` 保留三个空格，`unknown_property` 输出一个空格，`literal_preserved=false`，见[执行输出](../validation/docs-ssg-p2-gaps/code-review-r1/manifest-probe.log)、[实验源码快照](../validation/docs-ssg-p2-gaps/code-review-r1/default-literal-probe.rs)及[来源/命令记录](../validation/docs-ssg-p2-gaps/code-review-r1/provenance.json)。支持链为 [crates/rustify-docs-syntax/src/lib.rs](../../crates/rustify-docs-syntax/src/lib.rs) 99–102、493–527 行 → [crates/rustify-docs-cli/src/rendering.rs](../../crates/rustify-docs-cli/src/rendering.rs) 660–664 行 → [crates/rustify-docs/src/embed.rs](../../crates/rustify-docs/src/embed.rs) 233–236、343–378 行；计划依据为 [§5.1](DOCS-SSG-P2-gaps.md#51-可执行的嵌入诊断) **154、164 行**。普通 quoted string 对照组保留内部空格，见[直接格式化实验](../validation/docs-ssg-p2-gaps/code-review-r1/display-probe.log)。
- 修复与验证：仅在字面量外收紧 token 分隔；识别 raw string 的前缀与 hash 数量，直到完整闭合分隔符才结束该字面量，或采用保留 literal token 的解析方式。增加含内部引号、重复空格及不同 hash 数量的回归，断言 manifest 的字面量在未知属性和类型错误建议中逐字保留，并检查 text/JSON/dev 一致性。产品修复后按计划 §5.4 制作新的 R′/C′ 配对，重验受影响的固定出口。
- 置信度：高（0.99）；合法输入经实际注册解析器确认，根因由目标函数的直接执行及完整调用链共同证明。实验没有运行该 raw string 场景的完整 CLI/浏览器链路，不将静态推导计为已执行的端到端测试。

本轮确认问题：P0 0、P1 0、P2 1、P3 0。

## 2. 待核实项与范围限制

没有影响 F-01 裁定的待核实项。以下是本轮验证边界，不作为新增缺陷或额外开工条件：

- 未重新执行安装 C、模板两目标的完整离线 check、全部 CLI 152 项或全部浏览器回归；读取并核对 [M1](DOCS-SSG-P2-gaps.records/M1.md)、[M2](DOCS-SSG-P2-gaps.records/M2.md) 的限定证据，与本轮重跑区分。
- 官方站本轮实际重跑正常/减少动效，以及 dark/375px 的全部 27 条扫描路由；其余主题/视口组合核对既有 M2 扫描附件。组件 fixture 的四个主题/视口组合及相关状态全部重跑。自动化 axe 的 incomplete 保留在证据中，不将零 violations 等同于完整 WCAG AA 认证。
- 原 M6 的真实设备、稳定浏览器/VoiceOver、正式性能及原 M7 的公共 Git 来源、Pages/回滚仍由原计划承接；Firefox 例外按已批准口径保留。未重启这些工作，未声明整 P2 或 PRD 终验通过。
- 既有未提交编译缓存和生成二进制只作环境/来源上下文，未逐个审查内部实现；本轮新生成的报告与证据不纳入审查基线。

## 3. 覆盖与验证

变更意图：补全嵌入拒绝的可用名称、字段、类型、位置及建议，使文档岛实际遵守减少动效偏好，并以固定 R/C 本地交付，同时同步既有 Firefox 决定。

### 3.1 Git 范围与输入固定

| 纳入提交 | 作用 |
| --- | --- |
| `cc5fded38a0cfb24ee8f132bb70f19a80adb6ecc` | M1 功能库 R：诊断、位置换算、dev 去重、Docs CSS、测试及相关文档 |
| `7a42230b29ba3fc5c71c0c85d69913c54ac0bff6` | M2 CLI C 0.1.16：恰好五个配对文件 |
| `958c9bd902fd847b59093c31d83468fd7bae2b83` | M2 验收证据和计划完成记录；无追加产品代码变化 |

以上均在当前 main 可达，区间没有未登记的产品提交。`51dbaee62098a75004c02052f34357c881860612` 和 HEAD `f5d303d61acf294dff3c3ee75a589085622700b5` 仅修改本计划、回填提交，排除代码 findings 归因。实际比较使用 `git diff --no-ext-diff --no-textconv d2f20b03… 958c9bd9…`；累计 106 个变更文件，包含生成证据附件。

工作树初始 dirty、index 无差异。未提交内容为既有台账/经验记录、原计划的评审/差异报告，以及 `site/.rustify-docs`、组件 fixture 的日志和缓存；未发现待纳入的新增产品源码。原始 tracked diff SHA-256 为 `a842a17c0ae6465265eb5d5692b336fb4891146e4ae06b2938e2a0402e771156`；初始 status 清单 SHA-256 为 `16865ab11c9f05103bbf54d5f44bd43b167bd75ed5cf0084db7f2f474a84ac13`。

对变更文件及必要上下游共 213 个文件记录内容 SHA-256，清单按路径排序后以紧凑 JSON 计算集合指纹 `fd481704ff43dd4e953f367afd08aba0230d192a23d5673a717421c1d6b1e5a6`，见[输入清单](../validation/docs-ssg-p2-gaps/code-review-r1/inputs.json)。结束前复核这些文件全部未变化。当前 checkout 与所审 target 的产品源码、测试、锁及相关文档等价；差异仅为计划 SHA 回填，故本轮工作树测试不用于证明其他历史实现。

### 3.2 已审行为链

| 范围 | 核查结果 |
| --- | --- |
| 注册/内置名称与顶层字段 → 建议 → report/dev | 候选排序去重、空注册、保留名、未知属性、标量/Option/Vec、作者复合/别名、缺字段与完整 serde path 均有对应处理；默认表达式 literal 保真存在 F-01 |
| Markdown 原始事件 → payload 原点 → YAML key → 文件行列 | 核对 CRLF、引用/列表前缀、中文与同值字段；新增原点映射和位置回落有直接测试，内置仍定位围栏 |
| 冷预检/复用代 → helper → 失败守卫 | 预检拒绝项不再重复送 helper；类型错误保留 helper 原说明并加建议；独立 Rust 错误、旧 dist、last-good 与恢复路径实际通过 |
| Docs CSS → 压缩 → SDK 元素/Spinner | scope 和后代 duration/delay 的 important 零值及 Spinner 静止规则与实际标记匹配；正常偏好保留原时长和默认主题 Spinner 转动 |
| R → C → 两份锁 → 已安装入口 → 投影站点 | C 文件范围正确；模板仅六框架 source 变化、187 个第三方记录相同，根锁仅 CLI 版本变化；实际安装二进制摘要与 M2 记录一致，浏览器使用同一 C |
| PRD/作者参考/Read 规范 | Firefox 只同步 P2→P4 的已批准例外，D15/D16 和其他环境缺口保留；诊断与减少动效文案对应本次实现 |

完整阅读产品、测试、配置及文档 diff；大型生成 JSON 附件按结构、路由/状态、来源、violations/incomplete 核对；人工查看相关窄屏截图。未将生成附件的每个 axe 节点或每个二进制内部实现计为逐行代码覆盖。

### 3.3 本轮实际执行

环境：macOS arm64；Rust 1.99.0、mbx 1.15.0、Node 26.1.0、Python 3.12.13；Playwright 1.63.0 的 Chromium 通道。浏览器命令均使用计划 §9.2 的临时 python3 入口和进程级 Git remap，并显式绑定已安装 C 的 `RUSTIFY_DOCS_BIN`，未修改全局 Git/Python 配置。

安装二进制为 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-docs-gaps-installed.VV1r7t/bin/rustify-docs`，运行前后 SHA-256 均为 `c0dd693c69e5e1fd77472024bf557e88a239b2861b7d46884912a740e8389cc4`；`--build-info` 为 0.1.16、panic unwind、source C `7a42230b…`。官方 prepare 回执明确 candidate=C、framework=R。入口及原始命令记录见[provenance.json](../validation/docs-ssg-p2-gaps/code-review-r1/provenance.json)。

| 命令/实验 | 目标与实际结果 | 证据 |
| --- | --- | --- |
| `mbx test --locked -p rustify-docs` | target 等价源码；275 项通过（lib 202、集成 73），0 失败 | [docs-test.log](../validation/docs-ssg-p2-gaps/code-review-r1/docs-test.log) |
| `mbx test --locked -p rustify-docs-cli --test check --test dev --test components` | target 等价源码；33 项通过（check 16、components 2、dev 15）；真实组件测试包括编译错误、props/YAML、位置、输出保全和恢复 | [cli-test.log](../validation/docs-ssg-p2-gaps/code-review-r1/cli-test.log) |
| `components-subpath` · gaps reduced motion / gaps a11y | 已安装 C 重新构建的 `/rustify-ui/` fixture；10/10 通过，32 次扫描；loading/ready/client-only/failed、定制页/静态页/深 404，light/dark × 1280/375px；减少动效及正常动效通过 | [components-browser.log](../validation/docs-ssg-p2-gaps/code-review-r1/components-browser.log) |
| `official-subpath` · gaps reduced motion / gaps a11y dark 375px | 已安装 C 按 C/R 投影并重新构建的官方站；3/3 通过，正常/减少动效及 dark/375px 全 27 路由扫描 | [official-browser.log](../validation/docs-ssg-p2-gaps/code-review-r1/official-browser.log) |
| `docs-dev` · gaps embedded diagnostics | 已安装 C 的真实 dev 临时工程；2/2 通过，注册/内置/空 demo 建议同文、last-good、长建议换行、Hide/Enter/Esc/焦点与修复清除 | [dev-browser.log](../validation/docs-ssg-p2-gaps/code-review-r1/dev-browser.log) |
| 隔离 manifest + 目标建议函数实验 | 真实注册 parser 接受 raw string，manifest 保真，而新建议改变字面量；确认 F-01。普通 quoted string 为对照 | [manifest-probe.log](../validation/docs-ssg-p2-gaps/code-review-r1/manifest-probe.log) |
| impeccable context + `impeccable detect --json crates/rustify-docs/assets/docs.css` | 依据既有 Read 表面审查。检测器 4 条字号 advisory 均在未改行，且已由 DESIGN.docs-ssg.md 的字号规范覆盖；没有归因为本次缺陷 | [ui-detector.json](../validation/docs-ssg-p2-gaps/code-review-r1/ui-detector.json) |
| 两份 lock 的结构比较、C 文件级差异、入口摘要/来源 | 第三方无变化、六框架均固定 R、C 恰好五文件、实际二进制与 M2 摘要一致 | 本节、[M2 pair 审计](DOCS-SSG-P2-gaps.records/m2/pair-audit.json)及 provenance |

浏览器选择命令如下；环境绑定按上文和计划 §9.2 执行：

```sh
npx playwright test -c playwright.docs.config.ts --project=components-subpath islands.spec.ts custom.spec.ts --grep 'gaps reduced motion|gaps a11y' --workers=1
npx playwright test -c playwright.docs.config.ts --project=official-subpath official.spec.ts --grep 'gaps reduced motion|gaps a11y.*dark 375px' --workers=1
npx playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'gaps embedded diagnostics' --workers=1
```

浏览器三个 project 串行运行，合计 **15 通过、0 失败、0 skipped、0 flaky**；扫描合计 **59 实例、violations 0**。console、CSP 与异常请求断言通过，故障注入的预期异常另列。实际命令还指定隔离 output 和 JSON reporter，以保留既有测试结果；完整参数保存在执行日志，结构化结果见[浏览器摘要](../validation/docs-ssg-p2-gaps/code-review-r1/browser-summary.json)和[15 份浏览器审计](../validation/docs-ssg-p2-gaps/code-review-r1/browser-audits.json)。本轮截图：[注册错误 light/375px](../validation/docs-ssg-p2-gaps/code-review-r1/gaps-registered-375-light.png)、[内置错误 dark/320px](../validation/docs-ssg-p2-gaps/code-review-r1/gaps-builtins-320-dark.png)。

既有证据复用：M1/M2 各 140 次扫描附件均读取其扫描元数据及 violations/incomplete，二进制摘要分别为 `c8d10d5b…`、`c0dd693c…`，violations 均为 0；M2 官方站 108 次和 fixture 32 次的其余组合仅作历史限定证据。未重跑的 fmt/clippy、其余 CLI、安装/new 工程及两图离线检查继续引用其原记录，不计为本轮执行。

## 4. 计划符合性

| 本期承诺 | 实现/证据 | 本轮判断 |
| --- | --- | --- |
| V-1 · WORK-F4/F15/N6：可执行建议、忠实的类型/默认表达式、位置和三入口一致 | embed/builtins/markdown/rendering；275 Docs、33 CLI 及 dev 浏览器 | 通常分支和失败保全已核实；raw string 默认值失真，F-01 未满足 literal 保真承诺 |
| V-2 · WORK-N5：减少动效真实归零、Spinner 静止、正常偏好保留 | Docs CSS、压缩单测、两站即时 computed/动画采样 | 本轮定向复验通过；未新增 SDK/GPU 实现 |
| V-3 · 错误面板与稳态状态矩阵 | 真实 dev、fixture 32 次及官方 27 次实际扫描；其他官方组合核对原附件 | 已覆盖样例通过；F-01 的错误建议仍需修复后的回归，不因已有 UI 绿色关闭 |
| V-4 · Firefox/D15/D16、作者与设计文档同步 | PRD WORK-N3/AC-4/Q3 的本次 diff、作者参考及 Read 规范 | 在本增量范围内符合；其他环境与分期终验保留 |
| V-5 · R/C/锁/实际安装身份与固定出口 | C 五文件、锁结构、已安装摘要、prepare 回执、M2 两图/new 原件 | 配对与来源已核实；本轮未重新安装/完整重跑 new 工程，不代表公开可获取 |

本报告审查两个已登记里程碑的交付，不修改计划“2/2 完成”记录，不重新裁定原 M6/M7，也不把代码评审当作 PRD 全量覆盖核查。

## 5. 总体结论与修复顺序

**验收判定：需要修改。**

**正确性结论：`patch is incorrect`。** 一条已确认 P2 缺陷 F-01 使本期核心诊断承诺中的默认表达式内容不可靠；测试和浏览器绿色只证明已覆盖输入，无法关闭这个反例。其余已审增量未发现应另列的可确认缺陷。

先修 F-01 并添加 literal 保真回归，再按计划 §5.4 完成受影响的 R′/C′ 固定配对和出口重验，最后逐项复审。本轮不授予合并、发布或整 P2 验收结论。

```text
$apply-code-review docs/plan/DOCS-SSG-P2-gaps.code-review.md
```

修复完成后，由用户发起：

```text
$review-code docs/plan/DOCS-SSG-P2-gaps.md
```
