# DOCS-SSG-P2-gaps-2 · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，首次代码评审 | [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) | `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd..6c011792056f05c5c9837d316ac6fb00cd8fdfac` 净差异；另取证当前未提交内容，输入集合 SHA-256 `a9bd912778eeea0148b317879280bdc997a4ec14a2f2c11ac53804125682ce86` | 无；同名 `.review.md` 是计划文档评审 |

- 评审日期：2026-10-15T14:37:29+08:00；评审对象 ID：`8248f8f2-341a-4bfc-98d0-b893b5be44ac`。
- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；观察 checkout：`main / 97c9946c6a325bed676a92dbd7bcea038db4ea69 / dirty`。产品文件与目标提交一致，HEAD 后续差异仅为计划 SHA 回填。
- 计划版本：上述 HEAD 中的计划；SHA-256 `866198aa1ae4eeeebf6f57996d23e0aabae0d8a0ea08f816761f5e0c663404c2`。本轮核对实施与验收，不重新裁定计划架构，不修改代码或源计划。
- 意图：按用户指定计划，审查诊断生产方的逐字片段写法、dev DOM 面板读取，以及本地固定 R″/C″ 交付。采用 DOM/SSG 路径；未扩大到 GPU。

## 1. 确认问题

未发现满足“本次引入或实际激活”门槛的新增代码缺陷。另确认 **1 项 P2 本期交付合同缺口 PG-01**，详见 §4；其截断根因在基线中已存在，因此不冒充本轮新增 finding，但本计划对自有作者值的完整片段保证尚未闭合。

## 2. 待核实与范围限制

无影响 PG-01 裁定的待核实项。其 CLI、dev 终端和真实浏览器结果均已观察。

- 本轮未重跑完整 docs-dev 31 项、重新安装 CLI、独立 SSR/hydrate 检查、Clippy 或正式性能测试。既有记录按下文的版本适用性核对，未计为本轮新通过。
- 原 M6/M7、U-01–U-03、真机/稳定浏览器/VoiceOver、公开安装/Pages/回滚和 P3/P4 保持计划的排除范围；本轮不作整 P2 验收结论。
- 原有生成缓存、二进制和历史验证目录仅作为背景清单取证，未逐个反汇编或重放全部历史日志；产品、测试和作者参考的净 diff 已完整读取。未提交背景只保留哈希，非全部文件的可同步快照。

## 3. 覆盖与验证

### 固定范围与版本

| 提交 | 登记用途 | 本轮处理 |
| --- | --- | --- |
| `e20daf7f07179d2bd24700c345f577fb9c8dbc15` | M1 / 库 R″ | 完整产品与测试净 diff |
| `3567285195a1c149f0497e5b274068d7d8828039` | 仅计划回填 | 排除为进度提交，不增加代码范围 |
| `758def9cb3b88a1d06282ddfdb3e8708c8529cdc` | M2 / CLI C″ 0.1.18 | 配对五文件审计 |
| `6c011792056f05c5c9837d316ac6fb00cd8fdfac` | M2 验收证据 | 目标端点；读取 M2 验收脚本、记录与必要证据 |
| `97c9946c6a325bed676a92dbd7bcea038db4ea69` | 仅计划 SHA 回填 / 当前 HEAD | 排除为进度提交；读取最终计划版本 |

登记提交均可解析且为当前分支祖先，中间无未登记产品提交。基线是最早登记实现提交的第一父提交，比较采用两端树净差异，未叠加中间状态。

开始时共 406 个 dirty 文件：20 个 tracked 修改、386 个 untracked；无 staged 差异。内容为台账、历史报告、验证资料、fixture 日志与缓存，未发现未提交产品源码。取证清单仍纳入并单列，未将这些背景内容扩成新代码 findings。写报告前复核，原始 status、HEAD、439 个已固定文件与已安装入口均未变化，见[最终核对](../validation/docs-ssg-p2-gaps-2/code-review-r1/final-verification.json)。报告及本轮证据不纳入自身输入。

[scope.json](../validation/docs-ssg-p2-gaps-2/code-review-r1/scope.json) 枚举 32 个产品/测试/作者参考文件及所有 dirty 路径。439 文件由该集合、dirty 文件及源计划组成，逐文件 SHA-256；集合指纹是对 `hashes` 映射按键排序、用无空白 JSON 序列化后再做 SHA-256。index 与 tracked diff 另存独立指纹。[原始产品 diff](../validation/docs-ssg-p2-gaps-2/code-review-r1/product.patch)及[提交文件清单](../validation/docs-ssg-p2-gaps-2/code-review-r1/committed-files.txt)可复查边界。

### 已审行为链

| 范围 | 核查内容 |
| --- | --- |
| `diagnostic_text.rs`、syntax helper、`lib.rs` | 空值/空白/LF/CRLF/最长反引号串、两份实现一致性、std-only 无条件导出，未增加 Cargo 特性边 |
| docs、CLI、syntax 生产方 | 完整动态片段、固定前后缀、列表组合、普通作者值、既有错误包装与返回链；重点追踪 `content`、`embed`、`frontmatter`、`generation`、`html_check`、`include`、registry、rendering 与 rust_site |
| `dev.js`、`dev-components.spec.ts` | 同长最大围栏匹配、填充剥离、未配对串、空 code 节点、原始 textContent/code 断言；节点仍经 textContent/text node 创建，未引入 HTML 解释 |
| report/SSE/状态 | `report.rs` 保持原样输出；dev 与面板同一诊断、last-good、隐藏/重开/焦点/恢复合同；不解析外部原文以推测修复 |
| C″ 与作者参考 | CLI 0.1.18、固定 R″、模板锁、根锁、两处候选说明；逐字协议及外部原文边界 |
| 交付证据 | M1/M2 记录、220 行 M2 作者/浏览器脚本、命令摘要、安装身份、锁与必要原始结果；未将作者“已完成”代替实现核查 |

本轮由当前 agent 独立完成，未委派 reviewer。使用 review-code、Rust 指南和 impeccable 的既有产品/设计/面板上下文；不将一般设计偏好作为缺陷。

### 本轮实际运行

Rust 命令执行于当前 checkout；该 checkout 的产品文件与目标提交相同。CLI/浏览器使用进程级本地 Git remap，未改全局配置。组件测试使用仓库已有 Python 3.12.13 与 mbx；没有安装工具或基础设施。

| 命令 / 实验 | 实际结果 | 证据 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | exit 0 | [日志](../validation/docs-ssg-p2-gaps-2/code-review-r1/fmt-check.log) |
| `mbx test --locked -p rustify-docs` | 277 passed，0 failed | [日志](../validation/docs-ssg-p2-gaps-2/code-review-r1/rustify-docs-test.log) |
| `mbx test --locked -p rustify-docs-syntax` | 31 passed，0 failed | [日志](../validation/docs-ssg-p2-gaps-2/code-review-r1/rustify-docs-syntax-test.log) |
| `mbx test --locked -p rustify-docs-cli`，Python 3.12.13 PATH shim | 157 passed，0 failed；原有 compile proxy 1 ignored；包括新 diagnostic_text、check、components、dev、rust_dev | [最终日志](../validation/docs-ssg-p2-gaps-2/code-review-r1/rustify-docs-cli-test-python312.log)、[命令/环境结果](../validation/docs-ssg-p2-gaps-2/code-review-r1/cli-test-python312.json) |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'gaps-2 code spans: the real DOM reader' --workers=1 --reporter=json --output=<隔离目录>` | 1/1；原有手写读取表覆盖空值、空白、LF/CRLF、反引号、未配对原文及 HTML 文本 | [结果](../validation/docs-ssg-p2-gaps-2/code-review-r1/browser-table-results.json)、[完整 argv](../validation/docs-ssg-p2-gaps-2/code-review-r1/browser-table-command.json) |
| 已安装 C″ 的隔离内容站 `dev` + Playwright Chromium 153.0.8010.12 | anchor、YAML 键、空语言三阶段预期通过；第四阶段确认 PG-01；4 阶段 × 1280/375 × 浅/深共 16 视图；无横向溢出，Hide/Enter/Esc 和修复后隐藏通过；console/pageerror/CSP/失败请求/4xx+ 响应全为空 | [脚本](../validation/docs-ssg-p2-gaps-2/code-review-r1/content-browser.cjs)、[原始文字/code 与布局](../validation/docs-ssg-p2-gaps-2/code-review-r1/content-browser.json)、[dev 终端](../validation/docs-ssg-p2-gaps-2/code-review-r1/content-dev.stderr) |
| 同一临时站 C″ check JSON / build JSON / check text，以及 C′/C″ 对照 | PG-01 均可达；check/build 诊断相同、exit 1、位置 3:7，10 个既有 dist 文件原字节保持 | [真实出口](../validation/docs-ssg-p2-gaps-2/code-review-r1/frontmatter-exits.json)、[四输入/两版本对照](../validation/docs-ssg-p2-gaps-2/code-review-r1/frontmatter-probe.json) |

首次 CLI 命令未使用计划的 Python PATH 配方，组件测试因系统 Python 无 `tomllib` 失败；[原日志](../validation/docs-ssg-p2-gaps-2/code-review-r1/rustify-docs-cli-test.log)保留。只调整本次验证进程的 PATH 后，完整 CLI 重跑通过，不作为产品缺陷。首次临时浏览器探针误断言恢复时应删除隐藏列表的旧节点；既有 `show()` 在零问题时隐藏 root。按项目 `cleared()` 的 hidden/revision 合同修正探针后通过；[首次结果](../validation/docs-ssg-p2-gaps-2/code-review-r1/content-browser.json.first)保留，产品未改。

已实际查看本轮 anchor 手机深色、YAML 桌面浅色、空语言手机浅色及 PG-01 手机深色截图。代表图：[正常 anchor](../validation/docs-ssg-p2-gaps-2/code-review-r1/anchor-375-dark.png)、[截断片段](../validation/docs-ssg-p2-gaps-2/code-review-r1/frontmatter-at-line-375-dark.png)。这建立本轮 Chromium 证据，不冒充稳定浏览器或真机验收。

### 配对与历史证据适用性

- 独立[静态配对审计](../validation/docs-ssg-p2-gaps-2/code-review-r1/pair-static-audit.json)确认 C″ 恰好五文件；模板锁 193 个 package records 中只有六个框架 source 改为完整 R″，其余字段和记录不变；根锁只改 CLI 版本。
- 本轮复用已存在的 C″ 安装，未重新执行 install。`--build-info` 为 0.1.18、source=`758def9cb3b88a1d06282ddfdb3e8708c8529cdc`、panic=unwind；前后 SHA-256 均为 `6c0b025dfc4e73ec2dfbf519bd353ae795700e29f91f8613aa6ff3cc9ddac1d1`，见[前](../validation/docs-ssg-p2-gaps-2/code-review-r1/installed-before.json)/[后](../validation/docs-ssg-p2-gaps-2/code-review-r1/installed-after.json)。
- [M1 输入适用性](../validation/docs-ssg-p2-gaps-2/code-review-r1/m1-source-reuse-audit.json)：28 项中 27 项完全一致，唯一变化是作者参考在 C″ 中更新配对身份，已审 diff。结合冻结 R″/C″，可引用 M1 的独立 SSR/hydrate、Clippy、构建与 M2 真实 new/完整 docs-dev 31/31 的历史证据；这些记录不能覆盖本轮新增反例 PG-01。原始结果见 [M1 checks](../validation/docs-ssg-p2-gaps-2/m1/checks.json)、[M2 验收](DOCS-SSG-P2-gaps-2.records/M2.md)、[M2 浏览器结果](../validation/docs-ssg-p2-gaps-2/m2/browser/results.json)。

## 4. 计划符合性

| 需求/契约 → 任务 | 实现与验证 | 状态 |
| --- | --- | --- |
| D1/D3、§5.1 → M1 writer / V-1 | 两份 helper、无条件导出、精确手写表与 CLI 全量 | 已核实 |
| D2、§5.2 → M1 全部生产方与作者值 / V-2 | 净 diff 与构造/包装链已核对；frontmatter 自有 lang 片段在包装后丢失边界 | **存在缺口 PG-01** |
| D5、WORK-F13 → M1 DOM 读取 / V-3 | 同长围栏、空 code、完整路径/anchor、YAML 键、原文边界、键盘/恢复；本轮浏览器及原配对证据 | 指定常规场景已核实；不能宣称覆盖任意自有内容 |
| WORK-F15/N6 → V-1/V-4/V-5 | JSON 结构、rule、位置、非零退出、拒绝/dist/last-good 保持 | 已核实相关回归；诊断完整性受 PG-01 限制 |
| D6、C-2 → M2 固定交付 / V-4 | R″/C″ 五文件、锁与实际安装身份；原 new 工程记录 | 该固定配对已核实；修复 PG-01 后须按 §5.5 重做新配对出口 |
| §0.5、C-3 | 外部原文、原 M6/M7、U-01–U-03、P3/P4 与公开发布排除 | 未扩大验收范围 |

### PG-01 · [P2] 保持 frontmatter 自有语言片段的完整编码

- **定位**：[frontmatter.rs:166](../../crates/rustify-docs/src/frontmatter.rs#L166)，target `6c011792056f05c5c9837d316ac6fb00cd8fdfac` 的改动入口；同函数 [L196–200](../../crates/rustify-docs/src/frontmatter.rs#L196) 的后处理为基线既有代码。本项属于本期交付缺口，不计新增 finding。
- **问题**：Markdown frontmatter 的非法 `lang` 取值为 `a` + 反引号 + ` at line ` + 反引号 + `b` 时，`visit_str` 已对完整作者值调用 `code_span`，但 `language_issue` 随后对 YAML 错误字符串执行 `.split(" at line ").next()`，在作者值内部截断，输出只剩 ``` lang: `` a` ```。这破坏了自有片段的关闭围栏，面板显示编码残片、code 列表为空，作者值后半段和“不是语言标签”的说明均消失；本次新增写法没有沿完整返回链保持有效编码。自有错误由 Rustify 的 `E::custom` 创建，[M1 审计 L52](DOCS-SSG-P2-gaps-2.records/M1.md#L52) 也将它列为自有 lang 模板；这项完整性不能由“第三方原文不改写”的排除项代替。
- **合同依据**：[计划 L11–12](DOCS-SSG-P2-gaps-2.md#L11) 的自有诊断/作者值范围、[§5.1 L178、L188](DOCS-SSG-P2-gaps-2.md#L178) 的完整片段与任意内容读回性质、[§5.2 L195–198](DOCS-SSG-P2-gaps-2.md#L195) 的生产方与组合链审计。这里只要求保住本次已编码的自有片段，不要求改写一般 serde/YAML 原文。
- **最小输入**：无 Rust 内容站，配置使用有效 site.name/site.url，`content/index.md` 为下段。已安装 C″ 的 `check --format json`、`build --format json`、text 与真实 dev 均复现，JSON 仍可解析，诊断位置/拒绝语义正常。

```markdown
---
description: Home
lang: "a` at line `b"
---
# Home
```

| 观察点 | 实际结果 | 合同所需结果 |
| --- | --- | --- |
| message | ``` lang: `` a` ```（原始精确值见 JSON 证据） | 保留完整值及说明，两侧双围栏均存在 |
| 面板 textContent | 与上行相同的编码残片 | `lang: a` + 字面量反引号 + ` at line ` + 字面量反引号 + `b is not a language tag` |
| code 文本列表 | `[]` | 单个完整值 `a` + 字面量反引号 + ` at line ` + 字面量反引号 + `b` |

- **归因边界**：基线 `8a8268e6...:crates/rustify-docs/src/frontmatter.rs:193–197` 已有同一截断逻辑。C′ `79a77825...` 与该基线的 frontmatter 文件 Git blob 均为 `6f353c3d5bd370604df530f23f13bf879d1c75ef`；已核对 C′ 二进制身份并复现旧版本也截断。故不声称本轮新引入截断，也不把它扩成与计划无关的既有缺陷普查；阻塞的是本轮宣称的自有片段全覆盖交付。
- **修复与复核**：仅移除解析器依据真实 location 添加的末尾位置后缀，或将自有诊断内容与取位置过程分开；避免按作者内容中的固定子串截断。补含/不含字面量反引号及 ` at line ` 的 frontmatter lang 回归，断言完整 message、原始 DOM textContent/code、3:7 位置、exit 1、dist/last-good 与恢复。按计划 §5.5 冻结新的 R/C 并从安装入口复验，不自动沿用当前 C″ 的绿色证据。
- **置信度**：高。生产方、后处理、基线对照、三个 CLI 出口和真实浏览器结果相互印证。[CLI 证据](../validation/docs-ssg-p2-gaps-2/code-review-r1/frontmatter-exits.json) 给出精确预期编码、显示文字和 code 列表；[浏览器证据](../validation/docs-ssg-p2-gaps-2/code-review-r1/content-browser.json) 的 `frontmatter-at-line` 阶段记录实际值，测试观察通过不表示该阶段满足合同。

## 5. 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`，依据是本期明确的自有片段交付合同仍有 PG-01；本轮新增回归 findings 为 0。Writer/reader 机制、指定常规场景与固定配对已核实，现有全量测试通过不能关闭未覆盖的反例。该结论只覆盖所列计划与版本，不代表整 P2、合并或发布状态。

先修 PG-01 的自有消息返回链并补真实出口回归，再按计划重做固定 R/C 安装及受影响浏览器验收，最后发起复审。源代码、源计划和原有报告均未改；本报告、证据及台账为本轮新增交付，未提交或推送。

下一步：

```text
$apply-code-review docs/plan/DOCS-SSG-P2-gaps-2.code-review.md
```

修复后由用户发起：

```text
$review-code docs/plan/DOCS-SSG-P2-gaps-2.md
```
