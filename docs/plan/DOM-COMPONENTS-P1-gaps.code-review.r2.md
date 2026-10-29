# DOM-COMPONENTS-P1-gaps · 代码复审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 · 修复效果复审 | [DOM-COMPONENTS-P1-gaps](DOM-COMPONENTS-P1-gaps.md) | `0abf0f397b343b803cd6a375155a4d0550ab7fe2..244f58fc9ba38144099950acf33eb8aa83190768`；另核对当前工作树，tracked diff SHA-256 `96203f9a59f3058882ff3ce498618579c6ce1c07c86c482c652c9f8819003adc` | [代码初审 R1](DOM-COMPONENTS-P1-gaps.code-review.md) |

日期：2026-10-30（Asia/Shanghai）。评审 ID：`5307296b-348d-4666-bda3-55b208901877`。仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`。由主 agent 单人完成，无并行 reviewer。

本轮是**修复效果复审**，逐项复核 R1 全部两项问题及其边界，并检查修复引入的回归；未重新完整审查原始实现。R1 审查 `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e..0abf0f397b343b803cd6a375155a4d0550ab7fe2`，原评审对象 ID 为 `7e8ce16a-6aaa-4ab6-a01c-3404b94fa652`，报告 SHA-256 为 `0aa7af2c1f798b68c64821c347ff83c974324ad34c4c8a06d12459153193fdd3`。修复提交为 `17544a64cc70604f914b245b009cabd6554499bf`；当前核查版本为 `244f58fc9ba38144099950acf33eb8aa83190768`，产品源码与修复提交一致。已有处置见 [M1 修复记录](DOM-COMPONENTS-P1-gaps.records/M1.md#代码评审-r1-修复)和 [M2 局部补验](DOM-COMPONENTS-P1-gaps.records/M2.md#代码评审-r1-局部补验)。原报告、实现及计划未修改。

## 1. 确认问题

本轮未发现可确认的新增问题。P0/P1/P2/P3 均为 0。

### 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1](DOM-COMPONENTS-P1-gaps.code-review.md) / F-01 | P2；静态说明缺少 `data-rustify-scope`，照抄后 CSS 不匹配 | M1 记录；`17544a64cc70604f914b245b009cabd6554499bf` | **已解决** | [data_tools.rs:354–359](../../examples/theme-studio/src/data_tools.rs#L354)提供完整 section，并说明两属性须在同一元素、dark 标记也放该根。5 项导出浏览器测试通过；额外从本版实际下载资料复制根元素，按说明编译实际下载 CSS，普通和特殊 ID 的 Light/Dark 变量及按钮背景均正确，见 [静态消费结果](../validation/dom-components-p1-gaps/code-review-r2/probe-result.json) | 无 |
| [R1](DOM-COMPONENTS-P1-gaps.code-review.md) / F-02 | P2；合法 ID 中的引号/字符引用改变 HTML 属性值，导致选择器不匹配 | M1 记录；同上修复版本 | **已解决** | [data_tools.rs:338–343](../../examples/theme-studio/src/data_tools.rs#L338)先转义 `&`，再转义双引号和尖括号，且只用于 HTML 属性。真实导入 `brand"&copy;<dark>` 后，下载示例含 `brand&quot;&amp;copy;&lt;dark&gt;`；浏览器解析出的属性与原 ID 完全相等，两模式已编译 CSS 均生效，JSON 与 history 不变。见 [浏览器回归:161–193](../../tests/browser/theme-export-ui.spec.ts#L161)及上述静态消费结果 | 无 |

实际解决 2 项；仍存在、部分解决、回归、待核实、不再适用、本轮未复核的旧 finding 均为 0。R1 没有需要本轮补齐的待核实项；其 §2 的范围边界保留如下。

## 2. 待核实项与边界

没有阻止本轮修复裁定的缺证或待决事项。R1 所列历史 U-01 性能归因、VoiceOver、OS IME/真机、原生 Chrome 200% zoom、GPU/完整 Docs 及公开发布仍不在本轮范围，未记为通过。导出 spec 的 200% 检查仍是缩小视口和根字号放大的 reflow 模拟。

本轮不是独立 `review-ui` 评分或完整 PRD 覆盖核查；下面的真实浏览器检查只证明本次资料修复及直接相关交互。

## 3. 覆盖与验证

意图：在既有 **DOM** 导出说明中补齐静态根元素，并正确编码合法主题 ID；无需新增 GPU 呈现路径，不改变主题 schema、解析器、provider、存储或字体资源。

### 范围与版本

- 原输入：`$review-code docs/plan/DOM-COMPONENTS-P1-gaps.md`。按计划完成记录解析实现和修复提交，均已确认是当前 HEAD 的祖先。R1 已覆盖原始实现，本轮修复净差异按上轮目标 `0abf0f397b343b803cd6a375155a4d0550ab7fe2` 比较至当前 HEAD。
- 区间中 `dc9df3547c4c164af7cf3433bef883332659b6f6` 和 `244f58fc9ba38144099950acf33eb8aa83190768` 仅回写本计划的进度/SHA，排除出代码 findings。没有未登记、需要另行决定是否纳入的代码提交。
- 净差异共 9 文件：2 个代码/测试文件（`examples/theme-studio/src/data_tools.rs`、`tests/browser/theme-export-ui.spec.ts`）、计划和两份记录、4 个验证产物。完整阅读两处代码 diff、说明生成函数及显示/下载调用链，核对实际 codec 选择器、identity 校验、公开静态用法和修复记录；读取两个验证 JSON，实际查看两张作者截图。计划与记录作为契约和历史证据，不将其回写当作产品代码。
- 当前工作树没有额外产品源码变更，index diff 为空。既有 dirty 的台账/经验、PRD/历史评审/截图等已盘点，只作上下文或既有资料，不扩成整仓审查。tracked diff 指纹见评审对象；index diff SHA-256 为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。
- 计划 SHA-256 为 `052883342c8a0fd9fa60adc951cc63c821f80f3ccb64f002abc31985b3aecc7e`；含 tracked/untracked 的初始 21,737 文件内容快照指纹为 `8e16c1655ce6672c75c9b4a00c6137cf8b586cd69f149b35fae202f8cb663133`。验证后、写报告前逐项回查，原文件变化 0、新增输入 0、HEAD 未变；本轮报告、证据与后续台账写入不作为审查输入。源码及测试指纹与作者修复证据一致，详见 [本轮身份和结果摘要](../validation/dom-components-p1-gaps/code-review-r2/review-evidence.json)。

### 本轮实际执行

全部在上述当前版本执行；Studio release 重新构建，build ID 为 `2e09a56d5617614f`，屏内和实际下载资料的源码修订均为当前完整 HEAD。浏览器项目串行，使用本机 Chrome，无安装、部署或外部服务写入。

| 检查 | 命令 / 步骤 | 实际结果 |
| --- | --- | --- |
| 资料生成与失败路径 | `mise exec -- mbx test --locked -p theme-studio --bin theme-studio data_tools::` | **9 passed**；原 JSON、显式 mode/context、字体资源和非法文档行为通过 |
| 静态检查 | `mise exec -- cargo fmt --all -- --check`；`mise exec -- mbx clippy --locked -p theme-studio --all-targets -- -D warnings`；`git diff --check` | **均退出 0** |
| 当前 Studio | `mise exec -- mbx xtask build-web --example theme-studio --release` | **退出 0**；存在未改 Makepad/DOM 路径的 unused/dead-code warnings |
| 导出回归 | `RUSTIFY_XTASK=target/debug/xtask RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_UI_EVIDENCE_OUT="$review_tmp/studio" mise exec -- npx --no-install playwright test --project=theme-studio theme-export-ui.spec.ts --workers=1 --output="$review_tmp/studio-results"` | **5 passed，26.95s**；包括新增两种 ID、实际下载/剪贴板、各格式资料、预览模式、原 JSON/history、资源 HTTP、中英文/1440/768/390/模拟 200%、焦点和 console/请求/CSP |
| 独立生成器接缝 | `RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mise exec -- mbx check --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bin theme-export-generator` | **退出 0**；同一说明源码被 include 后仍能编译 |
| 编译后静态消费 | 启动当前 `target/debug/xtask serve --example theme-studio --release --port 4181`；`mise exec -- node "$review_tmp/probe.cjs"` 真实导入两种 ID，下载说明与 CSS；按说明依次导入 `sdk.css`、`theme-v4.css`、下载的 `theme.css`，通过本仓 Tailwind 4.1.13 编译，复制资料根元素并放入 `bg-primary` 按钮 | **退出 0；2 个 ID × 2 模式通过**。背景分别为 `rgb(21, 112, 239)` 和 `rgb(83, 177, 253)`；ID 完整、变量正确，作者 document/history 不变。不是重新运行全部八种 CSS 消费矩阵 |
| 展开资料走查 | 上述同版 strict CSP 页面，1440 英文和 390 中文；查看转义后的示例、长行换行、横向溢出、下载动作滚动可达与 Escape 回焦 | **通过**；console/pageerror、失败请求及 CSP 数组均为空；已实际查看 [桌面](../validation/dom-components-p1-gaps/code-review-r2/expanded-1440-en.png)和 [窄屏](../validation/dom-components-p1-gaps/code-review-r2/expanded-390-zh.png)截图 |

`$review_tmp` 实际为 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-gaps-review-r2-niafxlml`；临时脚本、下载样本、编译 CSS、日志及完整快照保留于该目录，少量共享结果保存到上述 `code-review-r2`。验证结束已停止本轮预览服务。

### 历史证据复用与未覆盖范围

原 R1 在等价产品基线上实际执行的存储/配置/交换、八种 CSS 和默认 DOM 消费，按 [R1 §3](DOM-COMPONENTS-P1-gaps.code-review.md#3-覆盖与验证)限定复用。本轮 diff 没有修改这些行为依赖的 codec、provider、存储、运行时、字体或两份指南；未重跑原四 spec 全组、完整独立 DOM/GPU 构建、八种 CSS 矩阵、全 workspace 或性能测试。本轮重新编译生成器和两种静态 CSS 用例，覆盖说明变更的实际消费接缝。没有把历史结果计入本轮 9/5 项通过数。

反证复核：原 schema 仍允许引号等字符，修复没有通过收紧 ID 限制规避问题；新转义值只用于资料 HTML，不写回作者 ID；scope 与模式标记和实际导出的双属性选择器一致。字符串处理限于低频说明生成，未进入输入/存储热路径。未发现修复新增的兼容性、资源或正确性缺陷；未使用外部技术资料。

## 4. 计划符合性

本轮仅核对 M1/M2 的修复职责和相关回归，不重结算原 P1 或 P2–P5。

| 本期承诺 | 本轮证据 | 结果 |
| --- | --- | --- |
| WORK-F20/AC-3、G-01、V-1/V-3：静态消费者能按导出说明使用相同模式和有效主题 | 完整静态根 + 属性转义；两种实际导入 ID、两次 Tailwind 编译和四个模式投影，JSON/history 不变 | **已核实；R1 F-01/F-02 均关闭** |
| WORK-N5/AC-1、G-02、V-2：当前 API/调用与资料一致，版本和资源说明保留 | 9 项模块及 5 项导出回归、当前修订核对、生成器编译；两份公开指南本轮无 diff | **修复相关部分已核实；其余沿用 R1 的既有核查** |
| WORK-F21/AC-2—AC-3、WORK-N6/AC-1、V-01：真实存储反馈与数据/重试保障 | 存储实现、测试及相关配置本轮未改；原 R1 完整专项证据 | **历史通过范围不受本修复影响，本轮未重跑存储 spec** |
| C-1/C-2：DOM、strict CSP、资源外置、双语/窄屏/焦点 | 当前 release、5 项导出浏览器及展开资料走查；没有 SDK class 或 schema 改动 | **所审修复范围已核实** |

## 5. 总体结论与下一步

**验收判定：通过验收。**

修复效果：**已解决**（实际解决 2 项，不再适用 0 项）。修复增量正确性结论：**patch is correct**；新增 findings 0，未发现阻塞回归。本结论只覆盖本轮声明的修复增量与旧问题核查，不宣称重新完整审查原始实现、全 PRD 终验、合并或发布。R1 原“需要修改”结论和处置历史保留。

无需再执行 `apply-code-review`。本计划本轮代码复审完成；如需独立 UI/UX 验收或本增量 PRD 覆盖核查，可由用户选择调用：

```text
$review-ui docs/plan/DOM-COMPONENTS-P1-gaps.md
$review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P1-gaps.md
```
