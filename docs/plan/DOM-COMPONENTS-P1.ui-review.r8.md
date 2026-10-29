# DOM-COMPONENTS-P1 · UI 验收评审 R8

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R8（全量；沿用 R6 基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [冻结目标](DOM-COMPONENTS-P1.ui-target.r7.md) · `6c8b16d50cf7` | HEAD `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；Studio `http://127.0.0.1:4181/`，catalog `http://127.0.0.1:4176/` | [r8 采集清单](DOM-COMPONENTS-P1.ui-captures/r8/capture.md) | [R7](DOM-COMPONENTS-P1.ui-review.r7.md) |

## 验收判定

**通过验收。**

本轮 **10/10**，七个单元并列最低分；沿用门槛 **8/10**，同一基线的历史最佳为 **10/10**。未关闭 P0/P1 为 **0/0**，全部严重度的未关闭差距为 **0**；无停滞预警或待裁定事项。

UI-218 经独立评审关闭：未标注部件的拾取框已呈固定蓝色，面板仍保留真实的未标注说明。全量冻结矩阵及补采材料均已评审，机器检查未增加差距。结论仅适用于本轮版本、目标与所测条件，不表示整期产品终验、合并或发布授权。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 | R7 | 采用原文 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| U1 Studio 框架与基础配置 | 3 | 3 | 3 | 1 | **10** | 10 | [U1-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U1-a.md) |
| U2 精调与来源解释 | 3 | 3 | 3 | 1 | **10** | 10 | [U2-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U2-a.md) |
| U3 导入与导出 | 3 | 3 | 3 | 1 | **10** | 10 | [U3-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U3-a.md) |
| U4 本地库迁移与存储反馈 | 3 | 3 | 3 | 1 | **10** | 10 | [U4-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U4-a.md) |
| U5 预览 | 3 | 3 | 3 | 1 | **10** | 9.8 | [U5-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U5-a.md) |
| U6 Button / IconButton / Link / ThemePanel | 3 | 3 | 3 | 1 | **10** | 10 | [U6-A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U6-a.md) |
| U7 布局与排版 | 3 | 3 | 3 | 1 | **10** | 10 | [U7-B](DOM-COMPONENTS-P1.ui-captures/r8/judge-U7-b.md) |

U7 [首评 A](DOM-COMPONENTS-P1.ui-captures/r8/judge-U7-a.md)与补采后的 B 均为 10；同分采用覆盖全部 104 张原图的 B 整组四维分与理由。其余单元各由一位新评审打分，未逐维拼分；全部评审未关闭差距的并集为空。完整理由见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r8/judge.md)，精确十进制合并见 [judge-results.json](DOM-COMPONENTS-P1.ui-captures/r8/judge-results.json)。

最低分 10 高于门槛加 0.5，不触发临界通过确认。U7-B 是材料补采后的新上下文重评，并非门槛确认。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |
| R5 | **9.6（U7）** | **9.6** | **0 / 1** | **UI-128 所在 U3** | 同一基线；UI-128、UI-207 已解决；新增 P1 UI-215 来自纯键盘实测 |
| R6 | **9.5（U1）** | **9.5** | **0 / 1** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；补采发现 P1 UI-217，另有 P2 UI-216；需要修改 |
| R7 | **9.8（U5）** | **9.8** | **0 / 0** | — | 同一基线；UI-216/UI-217 关闭；新增识别 P2 UI-218；通过验收 |
| R8 | **10（U1–U7 并列）** | **10** | **0 / 0** | — | 同一基线；UI-218 关闭；U7 原生分段补采后重评；通过验收 |

R1–R7 行原样保留上轮表格。本轮目标规范与 13 张附件未变，不重建基线；R6 → R7 → R8 的历史最佳为 9.5 → 9.8 → 10。没有低于门槛的平台期或修后仍存在的顽固差距。

## 上轮差距复核

[M7 修复记录](DOM-COMPONENTS-P1.records/M7.md)的“R7 → 修复 · 2026-10-29T17:05:58+08:00”将 UI-218 登记为“已修改待评审”。关闭依据来自本轮独立实拍评审及页面实测，修复自述仅用于核对处置。

| 编号 | 上轮严重度与摘要 | 上轮处置（取自修复记录） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-218 | P2 · 未标注部件的拾取框没有使用固定蓝色 | 已修改待评审 | **已解决** | [U5 原文](DOM-COMPONENTS-P1.ui-captures/r8/judge-U5-a.md)；[未标注点击帧](DOM-COMPONENTS-P1.ui-captures/r8/U5-inspector-undeclared-1440-light.png)、[说明面板入画](DOM-COMPONENTS-P1.ui-captures/r8/U5-inspector-undeclared-panel-1440-light.png)；另在[桌面浅色](DOM-COMPONENTS-P1.ui-captures/r8/MC-inspector-unmapped-1440-light.png)、[深色](DOM-COMPONENTS-P1.ui-captures/r8/MC-inspector-unmapped-1440-dark.png)、[390](DOM-COMPONENTS-P1.ui-captures/r8/MC-inspector-unmapped-390-light.png)实测为 `2px solid rgb(21, 112, 239)`，未标注说明保留 |

R7 没有其他未关闭差距，也没有独立于 UI-218 的未关闭机器检查项。U1、U7 的本轮评审未发现 UI-217、UI-216 回归，原文保留了各自复核依据。

## 差距清单

无。独立评审未关闭差距为 0，机器检查和主 agent 实拍核对未增加差距；没有新编号、豁免或不采纳处置。

## 待裁定事项

无。UI-218 已解决；本轮未改变目标期望或历史处置。

## 建议（不扣分）

- 采集命名：`U1-configured-keyboard-open-1440-dark.png` 实际仍是聚焦且关闭的触发器，不作为菜单展开证据。后续列表帧与本轮等待稳定后的 Enter/Space 实测提供展开证据。后续拍摄应等待列表可见再命名为 open。
- 额外行为取证：完整组件参数组合、其余断点、Reset 控件实际输入与点击、浮层内拾取等可按需要扩展；它们不构成本轮冻结矩阵的缺证或产品差距。下述 DOM 语义抽样不替代这些行为检查。
- 沿用 R7 的 Caller utilities 建议：人为覆盖 class 的演示样例仍有低对比，可另选更易读的演示配色；不作为默认组件 AA 通过证据。

## 机器检查

全部在真实 release 页面上执行，位于八份独立原文及合并 judge 落盘之后；未将机器结果交给评审调分。工具为 Node 26.12.1、Playwright 1.63.0、Chromium 153.0.8010.12 headless、axe-core / @axe-core/playwright 4.13.0。

| 检查 | 实际结果 | 证据／差距关联 |
| --- | --- | --- |
| 采集日志 | 13 个 run、82 页中，76 页完整回收日志：pageerror、console error、失败请求及 CSP 事件均 0。另 6 页属于下述失败采集，未混入无错误结论 | [健康汇总](DOM-COMPONENTS-P1.ui-captures/r8/capture-health-summary.json)、[原始轨迹](DOM-COMPONENTS-P1.ui-captures/r8/capture-log.json) |
| 机器运行日志 | 基础 7 页 errors/CSP 事件均 0；稳定键盘／对比度确认 4 页 pageerror、console error、失败请求和 CSP 均 0；Inspector／语义补查 errors 数组为空，后者未单独回收 CSP，不外推 | [基础](DOM-COMPONENTS-P1.ui-captures/r8/machine-check.json)、[确认](DOM-COMPONENTS-P1.ui-captures/r8/machine-confirmation.json)、[补查](DOM-COMPONENTS-P1.ui-captures/r8/machine-extra.json) |
| 页面溢出与导出框 | 9 个页面状态 pageOverflow=0、offenderCount=0：Studio 1440/768/390 编辑与预览/720-root200，以及两档按钮和布局夹具。1440、768、390 导出框均在视口内 | [摘要](DOM-COMPONENTS-P1.ui-captures/r8/machine-summary.json)；无新增差距 |
| 键盘主路径 | 70 次 Tab 焦点均在屏内；导出 25、导入 15 次 Tab 均留在对话框，Esc 后关闭并回到对应触发器；品牌选择 Custom → Gold | [基础键盘记录](DOM-COMPONENTS-P1.ui-captures/r8/machine-check.json) |
| 稳定后展开 | 1440 深色、768 浅色、390 浅色三组合，Enter、Esc→Space、Esc→Enter 共 9 步均展开且列表可见。不从前述瞬时关闭帧推断内部原因 | [9 步确认](DOM-COMPONENTS-P1.ui-captures/r8/machine-confirmation-summary.json)、[390 原图](DOM-COMPONENTS-P1.ui-captures/r8/MC-settled-enter-390-light.png) |
| Button 焦点 | 七变体 size2 均 `:focus-visible`、2px 实线；classic/solid offset=2px，其余 −1px；未外推为所有尺寸与颜色组合 | [焦点样式实测](DOM-COMPONENTS-P1.ui-captures/r8/machine-check.json) |
| 200% | 720×900、根字号 200% 时，未隐藏 DOM surface 的 scrollWidth/clientWidth 均 300、scrollLeft=0。另一隐藏后代的 243/236 预布局尺寸不计可见溢出；未将离屏按钮几何写成屏内通过 | [实测](DOM-COMPONENTS-P1.ui-captures/r8/machine-check.json)、[原图](DOM-COMPONENTS-P1.ui-captures/r8/MC-root200-preview-720-light.png) |
| UI-218 专项 | 三组合均为固定蓝色实线框，矩形与 Create · 0 对齐；拾取后计数仍 0、未标注说明保留；真实 Escape 后 Inspect 的 aria-pressed 均为 false | [Inspector 实测](DOM-COMPONENTS-P1.ui-captures/r8/machine-extra.json)；UI-218 已解决 |
| 可访问性扫描 | wcag2a/2aa/21a/21aa/22aa：7 个 Studio 状态违规 0。两档按钮各 19 个 color-contrast 节点，无其他违规；其中 16 个色族 solid 在 1280 专项中逐项核对 a9 合成背景和 contrast 前景相符，2 个是已接受的旧 accent 配对，1 个是 Caller utilities 建议。两档布局各 1 个 Code solid gray 3.31:1，按已确认口径接受 | [扫描](DOM-COMPONENTS-P1.ui-captures/r8/machine-check.json)、[19 节点明细](DOM-COMPONENTS-P1.ui-captures/r8/machine-confirmation.json)、[运行时配对核对](DOM-COMPONENTS-P1.ui-captures/r8/machine-confirmation-summary.json)；沿用 UI-100/UI-156 范围 |
| DOM 语义抽样 | 真实布局夹具读到 H1=1/H2=12/H3=19、STRONG=4/EM=4/CODE=29/KBD=43/BLOCKQUOTE=6/Q=2；21 个 Reset 样本的 class 直接落在原生元素；3 个省略样本保留完整文本节点；挂载前后所采 6 项 body 样式相同。仅证明这些样本，未证明所有 scope 隔离。该探针挂载后 fonts.status 为 loading，不用于字体载入判定 | [语义与省略摘要](DOM-COMPONENTS-P1.ui-captures/r8/machine-extra-summary.json) |
| 未检查部分 | axe incomplete 未全量人工复核；未重跑整期 V1–V10、完整下载／剪贴板行为、真实性能、VoiceOver、OS IME、Safari/Firefox、真机或真人研究。未记通过 | 本轮为冻结 UI 矩阵验收 |

上述对比度处理沿用冻结目标的用户接受范围，不等于 WCAG 合规认证。Caller utilities 保留为原有、不扣分建议；没有新增机器检查差距，机器结果不改变独立分数。

## 采集与覆盖

**版本与目标。** 本轮从当前源码经 `mbx xtask build-web --example theme-studio --release` 与 catalog 同类命令重新构建，两项均退出 0。build_id 分别为 `105da89e37304db9`、`18ca82701d674efd`；HTTP manifest 与本地产物一致。产品路径 diff／未跟踪文件均为空，源码指纹为 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`；工作树已有的非产品改动保留。见[构建身份](DOM-COMPONENTS-P1.ui-captures/r8/products.json)、[被测版本](DOM-COMPONENTS-P1.ui-captures/r8/tested-version.json)、[结束核对](DOM-COMPONENTS-P1.ui-captures/r8/version-final-check.json)。

继续既有纯 Markdown 循环，未建立 managed 索引或声称 Phoenix engine 接管。使用已知的 legacy 人工算法 `manual-sha256-normative-sections-v1`，完整目标指纹为 `6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`；规范段落与 13 张附件保持 R7 一致，见[目标身份](DOM-COMPONENTS-P1.ui-captures/r8/target.json)及[附件 hash 清单](DOM-COMPONENTS-P1.ui-captures/r8/target-fingerprint-final.txt)。本次以 DOM 工作台、DOM 组件夹具为主，U5 只验收已有 GPU 对比路径，没有新增呈现路径。

**覆盖与真实性。** 共 **340 张本轮原图**：U1 62、U2 24、U3 38、U4 23、U5 39、U6 50、U7 104。包含与 R7 对应的 278 帧，以及 U7 缩放状态新增的 62 张原生分段页面裁切（每段最多 900 CSS px、相邻重叠 24px），没有拼接或后期修改。必需未采格子为 0；原图 SHA-256／尺寸逐张核对。机器检查另有 13 张截图，独立列于[机器截图清单](DOM-COMPONENTS-P1.ui-captures/r8/machine-captures.json)，不进入评分。

使用项目现有 Playwright 真实浏览器入口，en-US、reducedMotion=reduce，常规 DPR1、焦点特写 DPR2；作者预览 light/dark 与编辑壳按实际状态记录。每帧等待字体、networkidle 和稳定窗口；200% 沿用 720×900 与根字号 200% 的冻结采集条件。U4 的旧库种子、配额／锁异常和第二页同源写入逐帧注明；没有网络拦截。工具实际进程为 Chromium headless shell，未混写成 headed Chrome。完整条件、UTC、URL、动作与环境干预见 [capture.md](DOM-COMPONENTS-P1.ui-captures/r8/capture.md)、[工具记录](DOM-COMPONENTS-P1.ui-captures/r8/browser-toolchain.json)及各单元 actions.log。

**环境噪声与恢复。** 首批已取得 265 帧后，两项本地构建输出目录缺失，运行中的静态服务返回 404；5 次就绪超时影响 13 帧，6 个未完成页面留下 console error 7 条、失败请求 9 条，缺失原因未查明。未把失败页当产品状态，也未把不完整日志写成零错误。相同源码重建后两项 build_id 与初采相同，逐文件验证临时副本，以现有 `xtask serve --dir` 在同 URL、strict CSP 下补采全部 13 帧；使用 `-a2` 新文件名，保留初次失败日志与已有原图。见 [404 实测](DOM-COMPONENTS-P1.ui-captures/r8/capture-timeout-http.json)、[恢复产物](DOM-COMPONENTS-P1.ui-captures/r8/recovery-products.json)、[逐帧补采对应](DOM-COMPONENTS-P1.ui-captures/r8/capture-resolutions.json)。

**独立评审。** 共 8 位 `fork_turns=none` 的新上下文评审：七单元各一位，U7 首评提出缩放细节看不清后，补采并另派一位新评审读取完整 104 张。各自只收到冻结目标、本单元本轮原图／采集与操作记录、R7 对应独立结论与原图；没有收到源码、diff、修复记录、完整报告、机器检查或门槛。model／effort 继承，工具未返回精确名称，未虚构。八组 packet、任务及所有白名单文件的 hash 复核通过，见[派发记录](DOM-COMPONENTS-P1.ui-captures/r8/judge-dispatch.json)、[结束核对](DOM-COMPONENTS-P1.ui-captures/r8/version-final-check.json)。八份原文独立保存，没有以合并稿覆盖；U5 同评审对矩阵边界的[澄清](DOM-COMPONENTS-P1.ui-captures/r8/judge-U5-a-clarification.md)不属于新增评分。

**主 agent 核对。** 检查 29 张派生联系表仅用于采集真实性与覆盖 QA，未交给评审或用于打分；另实际查看 14 张代表原图、13 张目标原图，并在机器检查后查看三张 UI-218 实测原图。独立评审逐图查看了各自原始材料，U7-B 明确记录 104/104；主 agent 没有自行替评审补分或删差距。U5 澄清的未展示行为处于冻结矩阵之外或为正常裁切；U7 首评所缺缩放细节已补齐，没有悬而未决的必需缺证。见 [QA 清单](DOM-COMPONENTS-P1.ui-captures/r8/capture-qa.json)、[原图完整性](DOM-COMPONENTS-P1.ui-captures/r8/image-integrity.json)。

计划、PRODUCT.md、DESIGN.md、当前及冻结目标、历史报告与里程碑记录共 21 个文件与开工 hash 一致。未改产品、计划或历史记录，未提交。两项本次临时服务已停止，4181／4176 无监听、进程组无残留；仅移除了本次拥有的临时产物副本，见[清理证据](DOM-COMPONENTS-P1.ui-captures/r8/cleanup.json)。

## 下一步

当前没有剩余差距。若结束 UI 打磨，本循环已有 UI 修复代码，下一步为：

```text
$review-code docs/plan/DOM-COMPONENTS-P1.md
```

若用户选择继续打磨，可另指定希望扩大取证或改善的范围；本轮不自行启动修复或代码复审。


## 状态登记

按仓内协议 v3 / state-model v2 登记独立 UI 验收评审 ID `52acf57f-e15a-4bc4-b538-209c3e21aeab`，`review_type: other`，关联 R7 `8abc8d80-fa38-4e09-b066-929980601837`。见[本轮状态页](../../.xgent-ai/sdlc/status/review/52acf57f-e15a-4bc4-b538-209c3e21aeab.md)。本轮为 `review-recorded` / `review-not-required`，表示报告已记录且无需 apply；R7 的 UI-218 处置已接续独立复核关闭，原 R7 分数与结论保留。

仅同步受影响的评审状态、计划状态导航与目录索引；原计划 Ready、实施 7/7 及实施事件集合不变，未登记产品终验、合并或发布。报告、证据和本次台账回写留在未提交工作树，未推送；下一步由用户选择是否结束打磨并启动代码复审。
