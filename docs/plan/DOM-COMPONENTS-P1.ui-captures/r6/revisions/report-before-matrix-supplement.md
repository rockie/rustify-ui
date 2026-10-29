# DOM-COMPONENTS-P1 · UI 验收评审 R6

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R6（全量；重建基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [冻结目标包](DOM-COMPONENTS-P1.ui-target.r6.md) · `6c8b16d50cf7` | HEAD `d7ff86ce`；产品路径无 diff；Studio `dff085ce38bbff64` http://127.0.0.1:4181/；catalog `32b52b66f04701ad` http://127.0.0.1:4176/ | [r6](DOM-COMPONENTS-P1.ui-captures/r6/) · [采集清单](DOM-COMPONENTS-P1.ui-captures/r6/capture.md) | [R5](DOM-COMPONENTS-P1.ui-review.r5.md) |

## 验收判定

**通过验收**

本轮分数 **9.7/10（U7）**，沿用 R5 门槛 **8/10**；本基线历史最佳 **9.7**。七个单元全部达标，未关闭 P0/P1 为 **0**。上轮唯一 P1 **UI-215 已解决**：真实键盘路径到达的四个导入/导出选择器在停留 700ms 后均显示 3px 蓝色焦点环。最低分不低于门槛 + 0.5，不需要确认评审。

R5 待修的六条差距全部关闭；UI-115 按已确认目标变更记为**不再适用（接受偏差）**，不冒充已修复。新增 **UI-216（P2）**：Code ghost 的强调色前景偏离参考，属于可选打磨项，不阻止通过。

本轮只验收冻结矩阵中的真实界面。未拍组合和未执行的功能结果仍为未知，详见各位评审原文；本结论不代表完整 UAT、合并或发布完成。本轮不修产品代码、不提交、不改计划进度，也不自行启动下一步修复或代码复审。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R5 |
| --- | --- | --- | --- | --- | --- | --- |
| U1 工作台框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 10.0 |
| U2 精调与来源解释 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** | 9.95 |
| U3 导入与导出 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** | 9.90 |
| U4 迁移与存储反馈 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** | 9.95 |
| U5 预览 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** | 9.95 |
| U6 Button / IconButton / Link / ThemePanel | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.99 |
| U7 布局与排版 | 3.0 | 2.7 | 3.0 | 1.0 | **9.7** | 9.6 |

分数及逐维理由来自 [合并 judge](DOM-COMPONENTS-P1.ui-captures/r6/judge.md)。七份独立原文全部保留，主 agent 没有改分、删差距或因争议更换评审。U5 的一次续接只补齐遗漏的开图路径记录，不重评、不改分。

U7 的 Kbd 圆角差距关闭，但本轮新发现 Code ghost 的用色偏差，因此视觉为 2.7；评审已实际打开 R5 Code 原图，确认旧版也存在该问题，记为新增发现，不记回归。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |
| R5 | **9.6（U7）** | **9.6** | **0 / 1** | **UI-128 所在 U3** | 同一基线；UI-128、UI-207 已解决；新增 P1 UI-215 来自纯键盘实测 |
| R6 | **9.7（U7）** | **9.7** | **0 / 0** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；新增可选 P2 UI-216；通过验收 |

R1–R5 行保留上一轮表格。R5 后用户就 UI-115 确认“保留现在这样就可以”，目标规范指纹由 `567117725f56` 改为 `6c8b16d50cf7`，本轮按合同重建基线；历史评分不重算。历史最佳、停滞及重构记录从 R6 重新计，旧 U3 重构仍保留在历史记录中。本轮没有停滞或停滞预警。

## 上轮差距复核

R5 原审有 7 条未关闭差距，随后用户接受其中 UI-115 的偏差；[M7 的 R5 修复记录](DOM-COMPONENTS-P1.records/M7.md#r5--修复--2026-10-28t2355520800)将其余 6 条登记为“已修改待评审”。本轮结果为 **6 条已解决、1 条不再适用**。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7 / 用户裁定） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-215 | P1 · 四个导入/导出选择器无可见键盘焦点 | 已修改待评审 | 已解决 | [纯键盘实测](DOM-COMPONENTS-P1.ui-captures/r6/machine-focus-confirm.json)；四触发器均 `:focus-visible=true`、3px 蓝 outline、offset 2px，停留 700ms 后实拍 |
| UI-214 | P2 · Kbd 大字级圆角偏小 | 已修改待评审 | 已解决 | [U7 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-a.md)；当前、目标及 R5 两尺寸原图对照 |
| UI-213 | P3 · 导入审阅滚动表头上方露出残行 | 已修改待评审 | 已解决 | [U3 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U3-a.md)、[本轮 bottom 帧](DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-1440-light.png)；表头上方无残字 |
| UI-203 | P3 · 配额与无锁反馈重复文案 | 已修改待评审 | 已解决 | [U4 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U4-a.md)；整体存储反馈与命名卡分工明确 |
| UI-76 | P3 · DOM/GPU 两框首行起点不一致 | 已修改待评审 | 已解决 | [U5 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U5-a.md)；[本轮比较帧](DOM-COMPONENTS-P1.ui-captures/r6/U5-gpu-compare-1440-light.png)内缩及字墨上沿一致 |
| UI-210 | P3 · 深色 classic 底边略浅 | 已修改待评审 | 已解决 | [U6 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U6-a.md)；末行相对明度约 0.7217，对应目标约 0.7174–0.7224；该指标不是 WCAG 对比度 |
| UI-115 | P3 · 网格 ↺ 未常显作用域 | 用户接受现状；跳过实施 | 不再适用 | [冻结目标 U2 口径](DOM-COMPONENTS-P1.ui-target.r6.md)、[U2 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U2-a.md)；保留图标、模式、title/可访问名称与来源区说明，不要求常显范围文字 |

评审原文中的 U3“新-1（R5）”映射到 UI-213，U7“R5 新-1”映射到 UI-214；依据为 R5 合并 judge 已登记的正式编号。原文未覆盖，映射记录见 [judge-results.json](DOM-COMPONENTS-P1.ui-captures/r6/judge-results.json)。

## 差距清单

当前 **1 条：P2 1 条；P0/P1 为 0**。没有待核实门槛项。

### UI-216 · [P2] Code ghost 的 color／high-contrast 文字仍带强调色（U7 · 视觉 · 可挽回 0.3）

- 位置：[1280 原图](DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-1280-light.png) ghost 行前两列，约 y294、x112/x388；[390 原图](DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-390-light.png)的 `color=indigo` 与 `indigo · high_contrast`。
- 现状 → 期望：当前两列为靛蓝／深靛蓝文字；[Code 目标图](DOM-COMPONENTS-P1.ui-target/radix-sink-code.png)的 ghost 行前两列约 y309 使用接近正文的中性前景。期望来自该目标图、U7 的 Code 外观比较口径与 L1。这是前景角色偏差，不是对已接受的低对比重新扣分。
- 修法：调整 Code ghost 的前景分支，让强调色 ghost 与 high-contrast ghost 消费有效主题的中性 foreground；保留无填充、无描边外观，单独复核显式 gray 分支。并排复查 1280、390 四列矩阵和下方 ghost 字级阶梯，避免同时改动 solid/soft/outline。
- 来源：[U7 独立原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-a.md)“新-2”→ UI-216；未复用 R5 Kbd 问题的编号。R5 原图已有，新增发现，不记回归。
- 主 agent 核对：已实际打开当前 1280 原图及 Code 目标图，确认前两列确有上述前景处理差异；未改分或严重度。

## 待裁定事项

无。UI-115 的用户方向已经明确，本轮按新目标复核。UI-216 为可执行的可选打磨项，没有阻塞或不采纳处置。

## 建议（不扣分）

- U2：Neutral 影响清单可按 Light/Dark 分节，提升扫描速度；当前内容已经完整。
- U4：长迁移 JSON 审阅可固定标题栏；目标没有要求，底部操作仍可达。
- U1：可补充键盘确认配置、键盘激活模式与分隔条拖拽证据。
- 沿用既有建议：Caller utilities 人为 class 覆盖样例对比度为 1.24:1，可换成更易读的演示配色；它不作为默认组件的 AA 通过证据。

## 机器检查

全部在本轮真实 release 页面上执行，晚于七份独立原文和合并 judge 的落盘。检查结果没有反馈给评审调分。

| 检查 | 实际结果 | 证据 |
| --- | --- | --- |
| 采集日志 | 48 个受监视页面：pageerror、console error、失败请求/HTTP ≥400 均为 0；47 页成功回收 CSP 日志，事件 0。首次定位失败的 1 页未回收 CSP，不写为该页已通过；相同选择器状态在后续实测重新覆盖 | [capture-health-summary.json](DOM-COMPONENTS-P1.ui-captures/r6/capture-health-summary.json) |
| 后置检查日志 | 基础检查 7 个 session 与补充检查 3 个 session 的 console/CSP 均为 0；补充检查监视的失败请求为 0。专门的焦点、Confirm 与对比度明细页不另作完整日志覆盖声明 | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-check.json)、[machine-followup.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json) |
| 页面溢出/对话框 | 所测 Studio 1440/768/390/720-root200 与 catalog 1280/390 的页面级横向溢出均 0；导出对话框均在视口内 | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-check.json) |
| UI-215 可见焦点 | 文档起点 Tab 5 次→Export→Enter→Tab 1 次；导入 Tab 4 次→Import→Enter→Tab 1 次。停留 700ms 后，格式及两项 CSS 选择器均为 3px 蓝 outline、offset 2px；四张原图均已开图核对 | [纯键盘实测](DOM-COMPONENTS-P1.ui-captures/r6/machine-focus-confirm.json)、[导出焦点图](DOM-COMPONENTS-P1.ui-captures/r6/MC-export-format-pure-keyboard-1440-light.png)、[导入焦点图](DOM-COMPONENTS-P1.ui-captures/r6/MC-import-format-pure-keyboard-1440-light.png) |
| 键盘主路径 | Studio 连续 Tab 70 次的焦点均在屏内；导出 25、导入 15 次 Tab 均留在对话框；Esc 关闭并回到对应触发按钮；品牌选择器键盘提交 Custom→Gold | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-check.json) |
| Button 焦点 | 七变体 size 2 均为 2px 实线；classic/solid offset 2px，其余 −1px。IconButton/Link 的视觉焦点见独立原图评审，未把这次七按钮测量外推为全组合检查 | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-check.json)、[U6 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U6-a.md) |
| 200% 预览 | 可见 DOM surface 为 300/300、scrollLeft 0；实际滚动到 Create，完整按钮位于 x386–499.4、y544.7–608.7。关闭 details 后代的 243/236 预布局尺寸不作为可见溢出 | [machine-followup.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json)、[实拍](DOM-COMPONENTS-P1.ui-captures/r6/MC-root200-preview-detail-720-light.png) |
| 局部深色浮层 | 实际 Confirm 为浅蓝 `rgb(83,177,253)` / 深字 `rgb(12,17,29)`，77.34×32px；Esc 关闭后回到 open-preview-dialog | [Confirm 专项](DOM-COMPONENTS-P1.ui-captures/r6/machine-confirm-button.json)、[实拍](DOM-COMPONENTS-P1.ui-captures/r6/MC-confirm-button-1440-light.png)、[Esc 结果](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json) |
| 重置入口 | primary 重置的 title 与 aria-label 均为 `Clear primary · Light only`，真实键盘焦点 3px 蓝 outline、offset 2px | [machine-followup.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json) |
| 复制与下载 | 剪贴板读回与可见输出一致；实际下载 theme.json，2,494 B、schema_version=2、failure=null，与可见 JSON 一致。注入 clipboard 拒绝后显示失败说明，文本和下载仍可用 | [machine-followup.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json)、[复制失败实拍](DOM-COMPONENTS-P1.ui-captures/r6/MC-copy-denied-1440-light.png)、[下载产物](DOM-COMPONENTS-P1.ui-captures/r6/MC-export-theme.json) |
| 可访问性扫描 | axe 4.13.0，wcag2a/2aa/21a/21aa/22aa：Studio 所测框架/导出态违规 0；按钮夹具 1280/390 各 19 个对比度节点，18 个为用户已接受的参考选阶/语义配对，1 个为上述 Caller utilities 建议；布局夹具各 1 个 Code solid gray 3.31:1，按已确认口径接受。incomplete 未全量人工复核，不宣称全站 WCAG 合规 | [基础扫描](DOM-COMPONENTS-P1.ui-captures/r6/machine-check.json)、[19 节点逐项明细](DOM-COMPONENTS-P1.ui-captures/r6/machine-followup.json) |
| 性能与人工辅助技术 | 未重跑 V1–V10、真实性能、VoiceOver、OS IME、Safari/Firefox、真机/真人研究；均不记通过 | 本轮为冻结 UI 矩阵验收 |

机器检查没有新增扣分或门槛项，不改变独立评分。

## 采集与覆盖

- 当前源码 HEAD 为 `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`；产品源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`，结束前复算一致，见 [版本记录](DOM-COMPONENTS-P1.ui-captures/r6/tested-version.json)与[最终核对](DOM-COMPONENTS-P1.ui-captures/r6/version-final-check.json)。原有未提交 PRD、评审、台账及旧验证截图保持原位。
- Studio 与 catalog 均由本轮 `mbx xtask build-web --example … --release` 成功重建。构建日志、原 manifest、临时产品路径及 HTTP 身份核对见 [r6 目录](DOM-COMPONENTS-P1.ui-captures/r6/)。两服务以 xtask serve 提供 strict CSP，实际 HTTP manifest 与本轮副本一致。
- Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13、Node 26.12.1、Playwright 1.63.0、Chromium 153.0.8010.12。headless、locale en-US、reducedMotion reduce；DPR1，组件焦点特写 DPR2。200% 采用目标规定的 720×900 + 根字号 200% 手法，不冒充 OS 缩放或真机。
- **154 张独立评审原图**：U1 25、U2 14、U3 26、U4 17、U5 18、U6 28、U7 26。另有 **12 张交卷后的机器原图**，逐张出处见 [machine-captures.json](DOM-COMPONENTS-P1.ui-captures/r6/machine-captures.json)。所有必需单元、状态、视口、主题格子及既定采集步骤均已覆盖；没有旧图、设计图或模拟页冒充本轮实拍。
- 每帧等待字体、网络空闲与稳定帧；新 context、测试种子及环境干预逐图登记。原图未合成、修饰或覆盖。主 agent 用缩略总览核对全部图片真实性，并打开关键原图；七位评审实际打开全部 154 张当前原图，另读对应目标/旧图，见各原文与[开图覆盖核对](DOM-COMPONENTS-P1.ui-captures/r6/judge-evidence-check.json)。QA 联系图仅作真实性检查，没有交给评审评分。
- 目标与 13 张附件 hash 见 [target-files.txt](DOM-COMPONENTS-P1.ui-captures/r6/target-files.txt)、[target-hashes.json](DOM-COMPONENTS-P1.ui-captures/r6/target-hashes.json)；原图尺寸及 hash 见 [image-integrity.json](DOM-COMPONENTS-P1.ui-captures/r6/image-integrity.json)。沿用旧人工循环算法 `manual-sha256-normative-sections-v1`，无 managed 索引，未静默 import 或另建循环。
- 评审方式：七单元各一位全新 `fork_turns=none` subagent，只读、不再派生，模型/effort 继承宿主且未覆盖；工具未回显具体型号。U5 续接只确认一条漏列的实际开图轨迹；没有确认评审或重评。白名单、派发与原文 hash 见 [judge-dispatch.json](DOM-COMPONENTS-P1.ui-captures/r6/judge-dispatch.json)，白名单不冒充文件权限沙箱。
- 环境噪声：首次补焦点帧使用了不存在的采集定位名 import-kind，后按页面实际 import-format 入口补齐四张，历史失败与原图保留，当前无必需格子缺失。浮层辅助探针最初量到 X 关闭按钮，随后按可访问名称单独核对真正的 Confirm；两份原始数据均保留，报告只采用后者说明 Confirm。
- 评审原文列出的矩阵外状态仍未知，包括完整 Link/IconButton 参数组合、更多断点、完整迁移正文与三条冲突决策的执行结果等。后置机器检查已补充复制、下载、部分键盘与回焦结果，但没有回写成独立评审的新分数。
- 本轮临时启动的两个服务已停止，端口 4181/4176 已复核无监听；见 [services.json](DOM-COMPONENTS-P1.ui-captures/r6/services.json)。产品代码、计划及历史报告均未修改，无提交。

## 下一步

是否继续打磨 UI-216，由用户选择。若继续：

```text
$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md
```

修复后再运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`。

若结束 UI 打磨，本循环已有 `UI修复` 代码提交，应先复审这些代码改动：

```text
$review-code docs/plan/DOM-COMPONENTS-P1.md
```

计划来自 PRD，代码复审后可选 `$review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P1.md`。以上仅建议命令，本轮未启动。

## 状态登记

本轮独立评审 ID：`4096c476-5ee3-44a5-a195-09a5183542ca`，见 [R6 评审状态](../../.xgent-ai/sdlc/status/review/4096c476-5ee3-44a5-a195-09a5183542ca.md)。按仓内 v3 协议登记为 `review_type: other`、`review-recorded` / `review-not-required`；UI-216 仍为未关闭的可选 P2，没有伪记已解决。R5 的六条修复已由本轮复核关闭，UI-115 按用户接受保留；只续接其处置状态，不改 R5 原判定或评分。Ready、7/7、合并与发布状态均未改变。原报告、截图及台账未提交、未推送。
