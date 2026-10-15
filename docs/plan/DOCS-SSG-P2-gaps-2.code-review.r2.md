# DOCS-SSG-P2-gaps-2 · 代码评审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) | 修复增量 `6c011792056f05c5c9837d316ac6fb00cd8fdfac..789003cc5ad58bd734f5a8c483f8fec475c2afe9`；另单列未提交内容，输入集合 SHA-256 `59616e0ae8cf0366b93d74dc5c95a01ce8949a7a76a097cfa0fab87e654a6d17` | [R1](DOCS-SSG-P2-gaps-2.code-review.md)，对象 `8248f8f2-341a-4bfc-98d0-b893b5be44ac` |

- 日期：2026-10-15T16:14:49+08:00；本轮评审对象 ID：`1b86dee0-14ed-45c9-8800-e3c18a9c4439`。
- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；观察 checkout：`main / 789003cc5ad58bd734f5a8c483f8fec475c2afe9 / dirty`。
- 计划 SHA-256：`a8976ea5e9a063f6e8f804287cc39a35fe6962801deacc9889aa7be1778a68e8`；R1 报告 SHA-256：`57349a6152eaee15c32b75679a60549086fc6929d6668f0fbd5fe5cf7b4d970a`。
- R1 原审为 `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd..6c011792056f05c5c9837d316ac6fb00cd8fdfac`。本轮逐项复核其唯一合同缺口 PG-01，并检查修复增量及直接相关回归；没有重新完整审查原始全部变更。
- 处置来源：[M1 / PG-01](DOCS-SSG-P2-gaps-2.records/M1.md#评审修复--pg-01)、[M2 / PG-01](DOCS-SSG-P2-gaps-2.records/M2.md#评审修复--pg-01)。采用 DOM/SSG 路径：诊断生产方保留完整编码，现有 dev DOM 面板读取；不涉及 GPU。

## 1. 确认问题

本轮新增 findings：**0**。未发现修复增量引入的可确认缺陷。

### 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1 §4 / PG-01](DOCS-SSG-P2-gaps-2.code-review.md#4-计划符合性)，`8248f8f2…` | P2 合同缺口；frontmatter lang 的作者值含 ` at line ` 时，既有 split 截断完整片段；需要修改 | R‴ `6942ec0d23812ecda61492d9db5c0046f481edfc`；C‴ `b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d`，CLI 0.1.19 | **已解决** | [frontmatter.rs L192–199](../../crates/rustify-docs/src/frontmatter.rs#L192) 只移除与实际 location 一致的末尾后缀；CLI 四输入回归与已安装 C‴ 的真实 dev 面板通过，详见下文 | 无 |

实际解决 1 项，不再适用 0 项；仍存在、部分解决、回归、待核实及未复核旧项均为 0。R1 的原判与基线归因保持，不将其既有截断根因改称新增回归。

修复保留 `visit_str` 生成的全部作者值与说明，位置计算、rule、severity 和建议未变。已阅读本机锁定依赖 `serde_yaml_ng 0.10.0` 的 `src/error.rs:248–262` 与 `src/libyaml/error.rs:136–148`：自定义错误的末尾位置由同一个 mark 格式化，支持此次精确后缀移除。没有依赖未核实的外部版本假设。

原反例在本轮真实浏览器中的精确结果：

| 观察点 | 实际值 |
| --- | --- |
| message | ```lang: `` a` at line `b `` is not a language tag``` |
| DOM textContent | ``lang: a` at line `b is not a language tag`` |
| code 文本列表 | ``["a` at line `b"]`` |
| 位置 | `content/index.md:3:7` |

上述表格中的反引号本身是数据，原始值以 [lang-prose.json](../validation/docs-ssg-p2-gaps-2/code-review-r2/browser/t02/00-lang-prose.json) 为准。含反引号与普通 `at line` 两阶段均核对 SSE、终端、独立手写 DOM 期望、HTTP last-good 原字节、原页面对象、键盘焦点与恢复；[用例 L1050–1095](../../tests/docs-ssg/dev-components.spec.ts#L1050) 本轮实际通过。

## 2. 待核实与范围限制

无阻塞本轮 PG-01 关闭或修复增量判断的待核实项。

- 本轮没有重新安装 CLI、重跑完整 Rust/Clippy/独立 SSR/hydrate、完整 docs-dev 32 项或六阶段 new 工程。已核对修复实施阶段的源指纹、命令结果、安装身份与配对差异，按下文限定复用；不记为本轮新通过。
- 原始 writer/reader 与全部生产方的完整评审沿 R1；本轮只复核修复涉及的消息链和必要回归。验证资料已盘点，读取必要日志、JSON 和截图，没有逐份重放全部历史产物或审查生成二进制内部。
- 原 M6/M7、U-01–U-03、真机/稳定浏览器/VoiceOver、正式性能、公开安装/Pages/回滚与 P3/P4 保持排除；本轮不代表整 P2 终验、合并或发布。

## 3. 覆盖与验证

### 固定范围

意图：修复 frontmatter 自有语言诊断的后处理截断，并以新的固定 R/C 安装出口验证完整消息。

| 提交 | 本轮处理 |
| --- | --- |
| `6942ec0d23812ecda61492d9db5c0046f481edfc` | 库修复 R‴；完整读取 frontmatter、CLI/浏览器测试及相关记录的修复差异 |
| `b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d` | CLI C‴；独立核对五文件、版本、框架 revision 和模板锁 |
| `925b3b57bfe9b3618673bd05ae40b9998df2dc50` | 验收证据提交；读取作者脚本增量和必要原始结果 |
| `97c9946c6a325bed676a92dbd7bcea038db4ea69`、`90daea30fce621f5694f1d72b34cc3c532508725`、`789003cc5ad58bd734f5a8c483f8fec475c2afe9` | 仅计划 SHA 回填，不作为代码 findings 对象；末项为本轮目标端点 |

计划登记的实现/修复提交均解析到完整 SHA 且是 HEAD 祖先，区间没有未纳入的产品提交。比较采用两端树净差异。466 个提交差异文件中，完整读取 8 个产品/测试/作者参考文件的 diff，另读取作者验收脚本增量；其余为计划、记录与验证资料，按用途盘点并阅读相关证据。[产品补丁](../validation/docs-ssg-p2-gaps-2/code-review-r2/product.patch)、[完整文件清单](../validation/docs-ssg-p2-gaps-2/code-review-r2/committed-files.json)可复查。

开始时 20 个 tracked 修改、445 个 untracked，无 staged 差异；内容为既有台账、报告/证据、fixture 日志与缓存，没有未提交产品实现。全部单列于 [scope.json](../validation/docs-ssg-p2-gaps-2/code-review-r2/scope.json)。输入集合固定 1005 个文件的 SHA-256，集合指纹为 `hashes` 映射按键排序、无空白 JSON 序列化后的 SHA-256；index 与 tracked diff 另存指纹。报告写入前，全部固定文件、HEAD、index 和已安装入口均未变，见[最终核对](../validation/docs-ssg-p2-gaps-2/code-review-r2/final-verification.json)。本轮报告、证据与协议要求的台账更新不纳入自身输入。

已追踪 `content.rs` 的语言校验入口 → `frontmatter::language_issue` → 自定义 YAML 错误 → Issue/Diagnostic → check/build/终端/dev SSE → 既有 `prose()`。另外核查四输入回归的 dist 保持与恢复、DOM 独立期望和布局断言，以及 C‴ 的安装/模板配对。本轮由当前 agent 独立完成，未委派 reviewer；使用 review-code、rust 与 impeccable 的既有产品/设计上下文，没有改产品、计划或 R1 报告。

### 本轮实际执行

Rust 验证执行于当前目标 checkout；浏览器显式绑定已安装 C‴。仅使用计划约定的进程级本地 Git remap、既有 Python 3.12 PATH shim 与 mbx；未安装工具或基础设施。[执行脚本](../validation/docs-ssg-p2-gaps-2/code-review-r2/run-checks.py)保留完整命令和环境设置。

| 命令 / 入口 | 实际结果 | 证据 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | exit 0 | [日志](../validation/docs-ssg-p2-gaps-2/code-review-r2/fmt.log) |
| `mbx test --locked -p rustify-docs-cli --test diagnostic_text` | 6 passed，0 failed/ignored；新增用例四输入的 check/build JSON、text、3:7、exit 1、dist 保持与恢复均通过 | [日志](../validation/docs-ssg-p2-gaps-2/code-review-r2/cli-diagnostic-text.log)、[检查结果](../validation/docs-ssg-p2-gaps-2/code-review-r2/checks.json) |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=docs-dev dev-components.spec.ts --grep 'frontmatter language preserves\|plain author paths\|real DOM reader' --workers=1 --reporter=json --output=<本轮隔离目录>` | 3/3；PG-01 两阶段 8 视图，路径/anchor/YAML 键/空语言 12 视图，手写读取表 4 视图；1280/375 × 浅/深均无横向溢出；console/pageerror/CSP/失败请求/4xx+ 响应全为空 | [完整 argv](../validation/docs-ssg-p2-gaps-2/code-review-r2/browser-command.json)、[结果与附件索引](../validation/docs-ssg-p2-gaps-2/code-review-r2/browser-summary.json)、[故障核对](../validation/docs-ssg-p2-gaps-2/code-review-r2/final-verification.json) |
| 已安装 C‴ 的前后身份 | 0.1.19，source=`b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d`，panic=unwind；SHA-256 均为 `c51f563d608a31bd56c09ae2ef393cdb93558ba3fa17472c8c0833f1a10ac95d` | [运行前](../validation/docs-ssg-p2-gaps-2/code-review-r2/installed-before.json)、[运行后](../validation/docs-ssg-p2-gaps-2/code-review-r2/installed-after.json) |

已实际查看本轮[反引号值手机深色](../validation/docs-ssg-p2-gaps-2/code-review-r2/browser/t02/08-gaps2-frontmatter-ticks-375-dark.png)和[普通值桌面浅色](../validation/docs-ssg-p2-gaps-2/code-review-r2/browser/t02/11-gaps2-frontmatter-plain-1280-light.png)：完整作者值与说明可见，等宽、位置、面板组成及换行保持。

### 配对与历史证据适用性

[本轮独立审计](../validation/docs-ssg-p2-gaps-2/code-review-r2/pair-and-evidence-audit.json)确认：

- C‴ 恰好修改五个配对文件；模板锁 193 条中，仅六框架包 source 更新为完整 R‴，其余 187 条不变；根锁只更新 CLI 0.1.18 → 0.1.19。R‴ 到 C‴ 另有的计划文件差异来自纯 SHA 回填，没有夹带功能源码改动。
- 当前 67 个相关功能源码、资源和测试文件与 C‴ 完全相同。M1 输入表 8 项中 7 项原字节相同，唯一不同是已独立核对的根锁 CLI 版本；因此源门适用性成立，未用修复前绿色替代修复后证据。
- 修复阶段 [13 门结果](../validation/docs-ssg-p2-gaps-2/apply-r1/m1-checks.json)全部 exit 0；已读取原始测试结果，docs/CLI/syntax/独立 SSR 分别为 277/158/31/30 passed，CLI 原有 1 ignored；独立 hydrate、Clippy、构建及两图见 [M1 处置](DOCS-SSG-P2-gaps-2.records/M1.md#评审修复--pg-01)。完整工作树与安装入口 docs-dev 均为 32/32。
- 修复阶段 [真实安装](../validation/docs-ssg-p2-gaps-2/apply-r1/install.json)与[同一 new 工程六阶段/24 视图](../validation/docs-ssg-p2-gaps-2/apply-r1/author/acceptance.json)均对应上述 C‴/R‴ 和当前二进制 hash；包含 PG-01 两阶段、三直接依赖固定 R‴、两目标无 Makepad、锁/dist/last-good 保持与恢复。本轮核对其记录及脚本，没有重新执行 install 或整套作者脚本。

## 4. 计划符合性

| 需求 / 本次任务 | 实现与证据 | 本轮状态 |
| --- | --- | --- |
| D2 / §5.2 / V-2 / PG-01 返回链 | 完整编码不再被作者值中的 ` at line ` 截断；其余生产方未改 | 已核实修复；原完整生产方审计未重做 |
| WORK-F13 / D5 / V-3 | 安装入口的完整 SSE/DOM/终端消息、单个完整 code、原位置、键盘与恢复 | 本轮实际通过 |
| WORK-F15/N6 / V-1、V-4、V-5 | JSON、错误退出、dist/last-good 与恢复合同；相关 CLI 和 DOM 回归 | 本轮局部通过，修复阶段全量证据适用 |
| C-2 / §5.5 新 R/C / M2 | R‴/C‴、五文件/锁/安装身份一致；新配对验收记录 | 已核实；本轮复用安装，未重新安装 |
| §0.5 / 原 M6/M7 与后续期 | 排除范围保持 | 未扩大验收 |

## 5. 总体结论与下一步

**验收判定：通过验收（PG-01 及本次修复增量）。**

修复结论：**已解决**。`patch is correct`，仅覆盖声明的修复增量；旧问题实际解决 1 项，新增问题 0，未发现须继续 apply 的意见。静态返回链、真实 CLI 和已安装新配对的真实 DOM 结果一致，原缺口的关闭判据已满足。

本轮没有重新完整审查原始变更，不将本次通过扩大成整 P2、合并或发布结论。报告及证据另存保留 R1；源计划与产品未修改，未提交或推送。

计划的需求来源为 PRD，可选下一步核对本增量的 PRD 覆盖（不自动执行）：

```text
$review-prd-dev-gaps docs/plan/DOCS-SSG-P2-gaps-2.md
```
