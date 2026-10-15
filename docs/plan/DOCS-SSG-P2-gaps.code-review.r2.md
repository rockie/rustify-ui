# DOCS-SSG-P2-gaps · 代码评审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 · 修复效果复审 | [DOCS-SSG-P2-gaps](DOCS-SSG-P2-gaps.md) · M1/M2 | 修复增量 `958c9bd902fd847b59093c31d83468fd7bae2b83..8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd`，以及当前未提交上下文；输入集合 SHA-256 `143951ec2aecc85d2124c07dbf544409b2ecab7441d57df1bbe7ef4b4c06f1d3` | [R1](DOCS-SSG-P2-gaps.code-review.md) |

- 日期：2026-10-15 +08:00；仓根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；checkout：main / `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` / dirty。
- 评审对象 ID：`1f739015-f48c-4e90-8370-093b2257e67f`；上轮对象 ID：`ee23493d-e277-489f-b5a5-5fd7b2ef763d`。
- 计划 SHA-256：`397a1ebe5129ce66d2a607649c6d44065f87a6252313ad61f6d662723e3000ff`；上轮报告 SHA-256：`11e1e615da9dc9218d5fc4e553b25719c6e40b8fda416cd49a448a2c6e7dba51`。
- 上轮完整审查基线为 `d2f20b03cf58c1d1ced3785397c501d412cd4167..958c9bd902fd847b59093c31d83468fd7bae2b83`。本轮核对其全部确认问题 F-01 和相关修复回归，不声称重新完整审查原始变更。
- 已有处置：[M1 / 评审修复](DOCS-SSG-P2-gaps.records/M1.md#评审修复)、[M2 / 评审修复](DOCS-SSG-P2-gaps.records/M2.md#评审修复)。
- 呈现路径：DOM 文档站与 Leptos 岛；修复发生在 CLI 的建议显示管线，未增加 GPU 或跨区协作。当前主 agent 单人评审，使用 review-code、Rust 和既有 impeccable 表面约束；未修改产品代码、原计划或历史报告。

## 上轮问题复审

| 来源报告 / 问题 | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1 / F-01](DOCS-SSG-P2-gaps.code-review.md#f-01--p2-按完整-raw-string-分隔符保留默认值字面量)；对象 `ee23493d-e277-489f-b5a5-5fd7b2ef763d` | P2：raw string 内部引号被误认作结束符，默认表达式中的重复空格被压缩 | 库 R′ `745ba6b720aa6ed042dcf5d4f763641fc2d05165`；CLI C′ `79a77825eadd07207154348ee9ffba48195d7766`（0.1.17）；证据 `53197198f2d8b12c63e26f93f96741a4ed0a453c` | **已解决** | `embed.rs` 358–361、395–431 行按 raw 前缀及完整 hash 分隔符保留整个 literal；203 项库单测包含真实 registry manifest 的逐字段建议断言。已安装 C′ 的真实 new 工程中，unknown-property 与 invalid_props 的 check 文本、JSON、build、dev SSE 均逐字含 `r#"a"   b   "c"#.to_string()`；浏览器 DOM 文本与空白样式也通过 | 无；本轮无需 apply |

关闭依据为当前源码及本轮实际执行，未仅采用实施记录的“已修复”自述。实际解决 1 项、不再适用 0 项、仍存在 0 项、待核实 0 项。

## 1. 确认问题

**未发现可确认的新增问题。** 新增 findings：P0 0、P1 0、P2 0、P3 0。

已核对 raw literal 的零/多个 hash、内部引号及不完整结束前缀、反斜线、byte/C raw 前缀、普通字符串、字符和 raw identifier 的相关处理。新增回归同时覆盖真实 manifest 的未知属性建议、缺值字段清单和字段类型建议；真实组件集成用例覆盖包含 raw 默认值的 check/build 同文与恢复。未发现修复改变注册事实、原错误位置、拒绝语义或产物保全。

## 2. 待核实项与范围限制

本轮修复范围没有阻断性缺证。R1 §2 的内容是验证边界，并非未关闭的独立缺陷；本轮分别处理如下：

| R1 边界 | 本轮处理与仍保留的限制 |
| --- | --- |
| 未重装 C、未重跑完整离线 check / CLI 152 项 | 本轮实际调用已安装 C′、新建工程，完成两目标 locked/offline metadata、check/build/dev 与 raw 反例；额外执行 CLI 33 项。核对原安装和两目标离线 check 的记录，但未再次安装、未再次执行全部 152 项或模板两目标完整离线 check |
| 浏览器 / AA 覆盖范围 | 本轮新增实际验证 raw 两种错误 × 1280/375px × light/dark，共 8 个组合。C′ 原 docs-dev 2/2、components 10/10、official 6/6 及 140 次 axe 扫描只复核既有摘要、来源及身份；未计为本轮重跑。未重新裁定 axe incomplete 或宣称完整 WCAG AA 认证 |
| 原 M6/M7 和环境缺证 | 实体设备、稳定浏览器/VoiceOver、正式性能、公共 Git 来源、Pages 与回滚仍由原计划承接，Firefox 例外保持；本轮不关闭这些范围 |
| 生成缓存与未提交内容 | 枚举并记录指纹；缓存、生成二进制不逐个审查内部实现。未发现待纳入的未提交产品源码；本轮报告和证据不纳入审查输入 |

M1 记录另有含反引号字面量的 dev 排版观察。其相关 `dev.js` 在本轮 base/target 间没有变化；本轮没有扩展为该历史行为的完整审查，不把它算作此次修复引入的 finding。

## 3. 覆盖与验证

变更意图：保留嵌入诊断中 raw string 默认表达式的字面量内容，并以新的本地 R′/C′ 固定配对交付。

### 3.1 范围与输入固定

| 提交 | 作用 / 本轮处理 |
| --- | --- |
| `745ba6b720aa6ed042dcf5d4f763641fc2d05165` | M1 修复：`embed.rs`、真实组件测试及处置记录 |
| `79a77825eadd07207154348ee9ffba48195d7766` | M2 C′：CLI 0.1.17 的五文件配对 |
| `53197198f2d8b12c63e26f93f96741a4ed0a453c` | M2 固定出口证据；读取脚本及结构化结果，未将历史通过计为本轮执行 |
| `f5d303d61acf294dff3c3ee75a589085622700b5`、`8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` | 仅计划 SHA/进度回填，排除代码 findings 归因 |

计划登记提交均解析为完整 SHA 且在 HEAD 可达，区间没有未登记的产品提交。比较方式为 `git diff --no-ext-diff --no-textconv BASE TARGET`；增量共 52 文件：产品/测试/配对 7、计划和里程碑记录 3、生成验证证据 42。完整读取产品、测试、配对和相关文档 diff，以及新增验收脚本；生成日志/JSON 按来源、退出值、断言和结构化摘要检查，截图按视觉证据检查。详见[范围清单](../validation/docs-ssg-p2-gaps/code-review-r2/scope.json)。

初始 index 无差异；tracked dirty 20 项，均为既有台账/经验记录及 fixture 日志。untracked 263 个文件中，38 项为台账、157 项为报告/验证资料、68 项为生成缓存；无新增产品源码。初始 tracked 净 diff SHA-256 为 `34fea5ad7562a375beeb9a0f77697fb8ee3c0f7be6981b13de3c442180486583`，清单见[初始 status](../validation/docs-ssg-p2-gaps/code-review-r2/initial-status.txt)。

对变更及必要上下游、测试、配置、计划和 R1 报告共 239 个文件逐个计算 SHA-256，按路径排序的紧凑 JSON 集合指纹为报告头所列值；见[inputs.json](../validation/docs-ssg-p2-gaps/code-review-r2/inputs.json)。验证后 239 个文件、HEAD 和安装二进制摘要全部未变，见[结束核对](../validation/docs-ssg-p2-gaps/code-review-r2/final-input-check.json)。未提交输入没有另行制作完整可同步快照，跨机器须按清单重新核对；已提交修复可从上述 SHA 取回。

### 3.2 行为链与反证核查

- `rustify-docs-syntax` 的 literal token → `rendering::fields` → `embed::boundary/display/pieces` → report 的 text/JSON 与 dev：只在显示层识别 raw literal；原 manifest、类型和默认值不求值、不改写。
- `raw_literal_end` 在 quote 后要求完整数量的 hash，内容不按普通字符串转义处理；只对合法开头进入扫描，保留 `r#type` 等标识符路径。新增分支不改变已有错误收集、原位置或失败守卫。
- `embed` 受 `pipeline` feature 控制；本轮未改变 helper/hydrate 协议、CSS、dev.js/dev.css、站点源码或 GPU。对应未变行为复用证据时保留原版本和验证边界。
- C′ 产品提交恰好五个允许文件；模板锁 193 包仅六框架 source 移到 R′，187 个第三方记录不变；根锁仅 CLI 0.1.16→0.1.17，其余 428 包记录不变。见[本轮锁与文件审计](../validation/docs-ssg-p2-gaps/code-review-r2/pair-audit.json)。

### 3.3 本轮实际执行

环境：macOS arm64；Rust 1.99.0、mbx 1.15.0、Node 26.1.0、Python 3.12.13、Playwright 1.63.0；真实浏览器为其 Chromium 153.0.8010.12。使用临时 python3 入口和进程级 Git remap，未更改全局 Git/Python 配置。环境与二进制信息见[provenance.json](../validation/docs-ssg-p2-gaps/code-review-r2/provenance.json)。

已安装入口为 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-docs-gaps-installed.eV4AfO/bin/rustify-docs`；执行前后 SHA-256 均为 `47486f96ff0985f63d36590d13f5bbd7ddbcbba3d501f40f6f561f00be8c9d7c`，与 M2 原安装记录一致；`--build-info` 为 0.1.17、panic unwind、source C′。本轮实际运行该入口，但不把核对记录描述为重新安装。

| 命令 / 实验 | 目标版本与实际结果 | 证据 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 当前 HEAD 对应源码，exit 0 | [执行汇总](../validation/docs-ssg-p2-gaps/code-review-r2/checks.json) |
| `mbx test --locked -p rustify-docs` | 当前 HEAD 对应源码；276 通过（lib 203、集成 73），0 失败 | [Docs 日志](../validation/docs-ssg-p2-gaps/code-review-r2/docs-test.log) |
| `mbx test --locked -p rustify-docs-cli --test check --test components --test dev` | 当前 HEAD 对应源码；33 通过（16+2+15），0 失败；包含 raw 默认值的真实 host/wasm 组件场景 | [CLI 日志](../validation/docs-ssg-p2-gaps/code-review-r2/cli-test.log) |
| 已安装 C′ 的临时 new 工程：原作者验收脚本，加 raw 浏览器断言 | 两图 189/180 包、零 Makepad、三直接依赖固定 R′；正常 check/build 成功，3 页 16 文件。未知名/属性/YAML/类型与中文 CRLF 回归通过；raw unknown-property、invalid_props 的 check 文本/JSON、build、dev SSE 同文同位（10:3），拒绝退出 1、旧 dist 不变、last-good 原字节保持、修复后 errors 0，输入 manifest/lock/src 保持 | [完整验收结果](../validation/docs-ssg-p2-gaps/code-review-r2/author-acceptance.json)、[执行脚本](../validation/docs-ssg-p2-gaps/code-review-r2/author-acceptance.py)、[raw 冷错误](../validation/docs-ssg-p2-gaps/code-review-r2/build-raw-cold.stdout)、[raw 类型错误](../validation/docs-ssg-p2-gaps/code-review-r2/build-raw-typed.stdout) |
| Playwright：真实 raw dev 面板，两个错误 × 四个主题/视口组合 | 8 组合通过：DOM `textContent` 逐字一致，literal 三空格保留、`pre-wrap` 生效，无页面/建议横向溢出；Hide/Enter/Esc 焦点通过。console、page error、异常请求和 CSP 违规均为空 | [未知属性审计](../validation/docs-ssg-p2-gaps/code-review-r2/browser-raw-1.json)、[类型错误审计](../validation/docs-ssg-p2-gaps/code-review-r2/browser-raw-2.json)、[未知属性截图](../validation/docs-ssg-p2-gaps/code-review-r2/browser-raw-1.png)、[类型错误截图](../validation/docs-ssg-p2-gaps/code-review-r2/browser-raw-2.png)、[浏览器脚本](../validation/docs-ssg-p2-gaps/code-review-r2/raw-browser.cjs) |
| impeccable context / dev 表面检测 | 按已有文档站 Read、dev Operate 表面核查；`detect --json crates/rustify-docs/assets/dev.js` 返回空数组 | [检测结果](../validation/docs-ssg-p2-gaps/code-review-r2/ui-detector.json) |
| C′ 历史出口证据复核 | 六条 project 前后身份记录均为同一安装摘要、candidate C′、framework R′；历史浏览器 18 项及 140 次扫描摘要保留原版本，未计为本轮重跑 | [历史证据审计](../validation/docs-ssg-p2-gaps/code-review-r2/prior-evidence-audit.json) |

首次临时浏览器脚本把全局站点配置警告的建议也计入数量，导致预期 1、实际 2 的断言失败；核对原始 CLI 诊断及既有 `expectRepairs` 的文件分组方式后，仅收窄临时脚本的选择范围，原产品没有改动。重跑全部临时作者验收及两个 raw 浏览器场景通过；[失败及修正记录](../validation/docs-ssg-p2-gaps/code-review-r2/harness-correction.json)保留原输出，不将它作为产品缺陷或隐藏首次失败。

补充：`git diff --check` 报告两个生成历史测试日志的 EOF 空行；这不构成本轮代码缺陷。未重跑 clippy、完整 CLI 152 项、GPU、全部组件/官方站或正式性能；相关历史结果仅在声明范围复用。所有复现写入隔离临时工程，结束后停止本轮 dev/browser 进程；未安装服务或部署。

## 4. 计划符合性

| 本轮相关承诺 | 实现 / 验证 | 判断 |
| --- | --- | --- |
| §5.1 第 1 条及默认表达式边界：字符串/字符 literal 原样，紧凑显示只作用于外部 token；V-1 | `embed.rs` 的完整 raw 分隔符扫描；真实 registry、CLI 与安装候选反例 | **已核实**；F-01 的内部引号和重复空格反例已关闭 |
| V-1/V-3：text/JSON/dev 一致、原位置、失败保全及浏览器显示 | 本轮 check/build/dev SSE、8 个真实浏览器组合、33 项 CLI 回归 | **已核实本轮受影响范围**；未扩为全部原始 V-3 矩阵重审 |
| §5.4 / V-5：R′/C′ 五文件、锁、实际安装身份、固定出口 | 本轮文件/锁审计、安装摘要核对、新工程两图及真实 CLI/browser；原安装与离线 check 记录 | **已核实修复交付链**；未重新安装或声称公共 Git 可获取 |
| V-2/V-4：减少动效、已批准 Firefox 与分期边界 | 本轮相关 CSS、dev 资源、PRD/设计内容未变；C′ 原出口摘要与原记录 | **限定复用**；未重新执行全部动效/AA 矩阵，不关闭原 M6/M7 |

本轮不改计划完成记录，不把实施“2/2”或此次修复通过扩大为整 P2 / PRD 终验。

## 5. 总体结论与下一步

**验收判定：通过验收（F-01 修复及声明的修复增量范围）。**

**修复结论：已解决。** 上轮唯一问题 F-01 实际解决，未发现修复引入的新问题。

**正确性结论：`patch is correct`。** 对上述增量，源码、实际测试、已安装固定候选及真实浏览器证据足以支撑本轮判断。此结论不代表重新完整审查原变更，也不授予合并、发布或整 P2 验收。

无需执行 apply-code-review。可选核对本增量的 PRD 覆盖：

```text
$review-prd-dev-gaps docs/plan/DOCS-SSG-P2-gaps.md
```
