# DOM-COMPONENTS-P1 · UI 验收评审 R6

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R6（全量；重建基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [冻结目标包](DOM-COMPONENTS-P1.ui-target.r6.md) · `6c8b16d50cf7` | HEAD `d7ff86ce`；产品路径无 diff；Studio `dff085ce38bbff64` http://127.0.0.1:4181/；catalog `32b52b66f04701ad` http://127.0.0.1:4176/ | [r6](DOM-COMPONENTS-P1.ui-captures/r6/) · [采集清单](DOM-COMPONENTS-P1.ui-captures/r6/capture.md) | [R5](DOM-COMPONENTS-P1.ui-review.r5.md) |

## 验收判定

**需要修改**

本轮分数 **9.5/10（U1）**，沿用 R5 门槛 **8/10**；本基线历史最佳 **9.5**。未关闭差距 **2 条：P1 1 条、P2 1 条，P0 为 0**。虽然分数达标，**UI-217** 的键盘活动位置不可见仍是门槛项：已选 Pink 时展开品牌色菜单并下移，活动项到了列表可视区之外，用户看不到当前位置。

上轮六条修复均已解决，包含 UI-215 的四个导入/导出选择器焦点；UI-115 按用户确认接受偏差，记为不再适用，不冒充已修复。新发现的 UI-217 与 UI-215 是不同入口和问题，不复用编号。另保留 **UI-216（P2）** 的 Code ghost 前景用色偏差。

本轮为全量 UI 验收、重建基线，没有停滞预警或待裁定事项。最低分不低于门槛 + 0.5，且当前因 P1 不通过，不触发临界确认评审。本轮只评审与留证，没有修产品代码、修改计划进度或提交。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R5 |
| --- | --- | --- | --- | --- | --- | --- |
| U1 工作台框架与基础配置 | 3.0 | 3.0 | 2.5 | 1.0 | **9.5** | 10.0 |
| U2 精调与来源解释 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.95 |
| U3 导入与导出 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.90 |
| U4 迁移与存储反馈 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.95 |
| U5 预览 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.95 |
| U6 Button / IconButton / Link / ThemePanel | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.99 |
| U7 布局与排版 | 3.0 | 2.7 | 3.0 | 1.0 | **9.7** | 9.6 |

| 单元 | 首评 a | 补采复评 b | 再补预览复评 c | 最终采用 |
| --- | --- | --- | --- | --- |
| U1 | 10.0 | 9.6 | 9.5 | c 整组 9.5 |
| U2 | 10.0 | 10.0 | 10.0 | c 整组 10.0 |
| U3 | 10.0 | — | — | a 整组 10.0 |
| U4 | 10.0 | — | — | a 整组 10.0 |
| U5 | 10.0 | 10.0 | — | b 整组 10.0 |
| U6 | 10.0 | 10.0 | — | b 整组 10.0 |
| U7 | 9.7 | 10.0 | — | a 整组 9.7 |

各次评分、逐维理由和完整原文见[合并 judge](DOM-COMPONENTS-P1.ui-captures/r6/judge.md)与[派发记录](DOM-COMPONENTS-P1.ui-captures/r6/judge-dispatch.json)。取各单元最低总分评审的一整组四维分；同分取覆盖完整材料的后评审；差距取全部评审并集。U7-b 未另列 Code ghost，但没有据此删除 U7-a 的发现。主 agent 不改分，不逐维拼分，不因争议换评审。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |
| R5 | **9.6（U7）** | **9.6** | **0 / 1** | **UI-128 所在 U3** | 同一基线；UI-128、UI-207 已解决；新增 P1 UI-215 来自纯键盘实测 |
| R6 | **9.5（U1）** | **9.5** | **0 / 1** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；补采发现 P1 UI-217，另有 P2 UI-216；需要修改 |

R1–R5 行保留上一轮表格。R5 后用户就 UI-115 确认“保留现在这样就可以”，目标规范指纹由 `567117725f56` 改为 `6c8b16d50cf7`，本轮按合同重建基线；历史评分不重算。历史最佳、停滞及重构记录从 R6 重新计，旧 U3 重构仍保留在历史记录中。本轮没有停滞或停滞预警。

## 上轮差距复核

R5 原审有 7 条未关闭差距，随后用户接受其中 UI-115 的偏差；[M7 的 R5 修复记录](DOM-COMPONENTS-P1.records/M7.md#r5--修复--2026-10-28t2355520800)将其余 6 条登记为“已修改待评审”。本轮结果为 **6 条已解决、1 条不再适用**。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7 / 用户裁定） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-215 | P1 · 四个导入/导出选择器无可见键盘焦点 | 已修改待评审 | 已解决 | [纯键盘实测](DOM-COMPONENTS-P1.ui-captures/r6/machine-focus-confirm.json)；四触发器均 `:focus-visible=true`、3px 蓝 outline、offset 2px，停留 700ms 后实拍 |
| UI-214 | P2 · Kbd 大字级圆角偏小 | 已修改待评审 | 已解决 | [U7 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-a.md)及[补评](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-b.md)；当前、目标及 R5 两尺寸原图对照 |
| UI-213 | P3 · 导入审阅滚动表头上方露出残行 | 已修改待评审 | 已解决 | [U3 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U3-a.md)、[本轮 bottom 帧](DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-1440-light.png)；表头上方无残字 |
| UI-203 | P3 · 配额与无锁反馈重复文案 | 已修改待评审 | 已解决 | [U4 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U4-a.md)；整体存储反馈与命名卡分工明确 |
| UI-76 | P3 · DOM/GPU 两框首行起点不一致 | 已修改待评审 | 已解决 | [U5 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U5-a.md)；[本轮比较帧](DOM-COMPONENTS-P1.ui-captures/r6/U5-gpu-compare-1440-light.png)内缩及字墨上沿一致 |
| UI-210 | P3 · 深色 classic 底边略浅 | 已修改待评审 | 已解决 | [U6 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U6-a.md)；末行相对明度约 0.7217，对应目标约 0.7174–0.7224；该指标不是 WCAG 对比度 |
| UI-115 | P3 · 网格 ↺ 未常显作用域 | 用户接受现状；跳过实施 | 不再适用 | [冻结目标 U2 口径](DOM-COMPONENTS-P1.ui-target.r6.md)、[U2 原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U2-a.md)；保留图标、模式、title/可访问名称与来源区说明，不要求常显范围文字 |

评审原文中的 U3“新-1（R5）”映射到 UI-213，U7“R5 新-1”映射到 UI-214；依据为 R5 合并 judge 已登记的正式编号。评审原文均保留；映射记录见 [judge-results.json](DOM-COMPONENTS-P1.ui-captures/r6/judge-results.json)。

## 差距清单

当前 **2 条：P1 1 条、P2 1 条**。两项均可执行，无待核实门槛项或待裁定事项。

### UI-217 · [P1] 已选品牌色后的菜单活动项不在可视区域（U1 · 交互与状态 · 可挽回 0.5）

- 位置：[768 浅色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-768-light.png)、[390 浅色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-390-light.png)、[1440 深色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-03-1440-dark.png)的 Brand color 展开列表。
- 现状 → 期望：触发器为 Pink，真实 Enter/ArrowDown 后列表仍从 Custom 排到 Orange，可视区域没有活动行；默认 Custom 起点的同轮图能看到 Gray 蓝色活动行。按[目标 G4](DOM-COMPONENTS-P1.ui-target.r6.md)，已配置状态也必须让键盘活动位置可见。
- 修法：在基础配置品牌色选择器展开和方向键更新时，将当前活动项滚入弹层自己的滚动视窗，例如 nearest 对齐；保留活动行蓝色反馈与当前值的独立选中标识。修改选择器的定位/滚动行为，无需重排工作台。
- 复验：从 Graphite → Pink → Auto 出发，覆盖上述三个组合，键盘展开、连续上下移、Enter 确认、Escape 回焦；每个关键帧能直接辨认当前活动项。只给不可见选项加样式不足以关闭。
- 来源：U1-b 与 [U1-c 独立原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U1-c.md)的“新-1”→ UI-217；相同位置与图证合并，采用 c 的 0.5 可挽回分。R5 没有 Pink 起点对应帧，记为新增发现，不断言代码回归。
- 主 agent 核对：三个组合的实际测量均显示活动项为 Plum、`data-active=true`、蓝底，但列表 `scrollTop=0`，活动行位于列表滚动视窗之外；Enter 能实际选择 Plum。[测量及失败重试](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu-confirm.json)支持可见性缺陷，未改变独立分数或严重度。

### UI-216 · [P2] Code ghost 的 color／high-contrast 文字仍带强调色（U7 · 视觉 · 可挽回 0.3）

- 位置：[1280 原图](DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-1280-light.png) ghost 行前两列，约 y294、x112/x388；[390 原图](DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-390-light.png)的 `color=indigo` 与 `indigo · high_contrast`。
- 现状 → 期望：当前两列为靛蓝／深靛蓝文字；[Code 目标图](DOM-COMPONENTS-P1.ui-target/radix-sink-code.png)的 ghost 行前两列约 y309 使用接近正文的中性前景。期望来自该目标图、U7 的 Code 外观比较口径与 L1。这是前景角色偏差，不是对已接受的低对比重新扣分。
- 修法：调整 Code ghost 的前景分支，让强调色 ghost 与 high-contrast ghost 消费有效主题的中性 foreground；保留无填充、无描边外观，单独复核显式 gray 分支。并排复查 1280、390 四列矩阵和下方 ghost 字级阶梯，避免同时改动 solid/soft/outline。
- 来源：[U7 独立原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-a.md)“新-2”→ UI-216；未复用 R5 Kbd 问题的编号。R5 原图已有，新增发现，不记回归。
- 评审合并：U7-b 给出 10.0 且未另列此项；按全部评审差距并集保留 U7-a 的 UI-216，并采用其完整的 9.7 四维分，不删差距。
- 主 agent 核对：已实际打开当前 1280 原图及 Code 目标图，确认前两列确有上述前景处理差异；未改分或严重度。

## 待裁定事项

无。UI-115 已有用户方向；UI-217 与 UI-216 均可局部修复，尚未经历针对它们的修复轮，不触发顽固差距或重构要求。

## 建议（不扣分）

- U2：Neutral 影响清单可按 Light/Dark 分节，当前范围已完整表达。
- U4：长迁移 JSON 审阅可固定标题栏；目标未要求，底部操作仍可达。
- U1：可增加键盘激活模式及分隔条拖拽记录；UI-217 的后置实测已经补充 Enter 选择结果，但不回写成评审新分数。
- 沿用既有建议：Caller utilities 的人为 class 覆盖样例对比度为 1.24:1，可换成更易读的演示配色；它不作为默认组件的 AA 通过证据。

## 机器检查

全部在本轮真实 release 页面上执行。基础检查位于首批七份独立原文落盘之后；补评发现 UI-217 后追加专项测量。所有机器结果均与独立评审白名单隔离，没有反馈给评审调分。

| 检查 | 实际结果 | 证据 |
| --- | --- | --- |
| 采集日志 | 65 个受监视页面：pageerror、console error、失败请求/HTTP ≥400 均为 0；62 页成功回收 CSP 日志，事件 0。三次采集脚本失败的 3 页未回收 CSP，不写为这些页已通过；相关界面状态在后续成功补采 | [capture-health-summary.json](DOM-COMPONENTS-P1.ui-captures/r6/capture-health-summary.json) |
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
| UI-217 活动项可见性 | 768 浅色、390 浅色、1440 深色均实测：Pink 后 Enter/ArrowDown，aria-activedescendant 指向 Plum，活动行有蓝底但位于列表滚动视窗之外，scrollTop 均为 0；Enter 实际选中 Plum。首测 390/1440 未等稳定而定位超时，增加等待后的两页复测成功；失败原始记录保留 | [首测](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu.json)、[复测](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu-confirm.json)、[窄屏实拍](DOM-COMPONENTS-P1.ui-captures/r6/MC-pink-menu-confirm-390-light.png) |

UI-217 是独立评审发现的 P1，机器检查复现并定位到活动项未进入列表可视区，没有另扣分；不把键盘能够确认选项误写为焦点反馈合格。专项两个尝试共 5 页的 console/CSP/失败请求均为 0，只有其中 3 页完成上述选项测量。

## 采集与覆盖

- 路径选择：以 DOM 实现验收为主；U5 只覆盖本期已有 DOM/GPU 预览协作，不新增 GPU 能力。当前 HEAD `d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`；构建时源码指纹 `22ecc24aabb4c44bc427f7308b484720912f48fbef9befc422a722415fd56ad3`，结束前再次核对 HEAD、产品 tracked diff 与 untracked 均未改变，见[初始版本](DOM-COMPONENTS-P1.ui-captures/r6/tested-version.json)与[最终版本核对](DOM-COMPONENTS-P1.ui-captures/r6/version-final-check.json)。原有工作树资料保持原位。
- Studio 与 catalog 均由本轮 `mbx xtask build-web --example … --release` 成功重建，构建 ID 分别为 `dff085ce38bbff64`、`32b52b66f04701ad`；strict CSP 服务经 HTTP manifest 校验。构建日志、原 manifest、临时副本及身份核对见[r6 目录](DOM-COMPONENTS-P1.ui-captures/r6/)。
- Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13、Node 26.12.1、Playwright 1.63.0、Chromium 153.0.8010.12。headless、locale en-US、reducedMotion reduce；DPR1，焦点特写 DPR2。200% 为规定的 720×900 + 根字号 200% 手法，不等同 OS 缩放。
- **236 张独立评审原图**：U1 46、U2 24、U3 26、U4 17、U5 39、U6 50、U7 34；另有 **15 张后置机器原图**。[采集清单](DOM-COMPONENTS-P1.ui-captures/r6/capture.md)逐图记录状态、入口、视口、主题、时间和环境干预；[机器图出处](DOM-COMPONENTS-P1.ui-captures/r6/machine-captures.json)另列。原图没有合成、修饰或覆盖。
- 首批 154 张只覆盖步骤，漏了部分窄屏/深色组合。交付前回读矩阵发现后，保留[初版报告快照](DOM-COMPONENTS-P1.ui-captures/r6/revisions/report-before-matrix-supplement.md)、初版 judge 和清单；追加 82 张并由新评审复核，最终发现 UI-217，判定由暂存的通过更正为需要修改。初版不再作为当前全量判定，历史事件用后续补证事件接续，不覆盖旧事件。
- 补采覆盖 U1 的 768/390 中文与焦点、1440 深色及 390 中文预览；U2 的 390 精调及低对比预览；U5 深色/窄屏边界、浮层、拾取和 DOM+GPU；U6 深色/窄屏主要状态及 Link 悬停/焦点；U7 窄屏 110%/95%/invalid。U3/U4 原指定的窄屏子集已采，不扩大为其他状态的窄屏验收。
- catalog 布局夹具没有深色操作入口；U6 深色 Link 改从同 SDK 的 Studio DOM 真实预览进入，在 1280×900 拍普通/悬停/键盘焦点，1440 额外帧也保留。仅变更采集入口，没有注入组件样式或修改目标，不把单一 Link 结果外推为全部参数组合。
- 14 位全新 `fork_turns=none` 只读 subagent：首评 7 位，矩阵补评 5 位，再补预览复评 2 位；模型与 effort 继承宿主、无覆盖，工具未回显具体型号。每次只读目标、本轮实际材料和 R5 对应 judge/原图，不读取代码、修复记录、门槛或机器结果。U5 原评审的轨迹续接只补漏列路径。
- [开图核对](DOM-COMPONENTS-P1.ui-captures/r6/judge-evidence-check.json)确认每个 packet 的当前原图全部实际打开，最终 236 张均有独立开图轨迹。主 agent 检查 20 张联系图及后两张补图原图，并开关键原图；联系图仅核真实性，没有交给评审打分。
- 目标与 13 张附件 hash 见[target-hashes.json](DOM-COMPONENTS-P1.ui-captures/r6/target-hashes.json)；所有当前原图的尺寸及 hash 见[image-integrity-final.json](DOM-COMPONENTS-P1.ui-captures/r6/image-integrity-final.json)，首批 154 张 hash 均未变化。人工目标指纹算法 `manual-sha256-normative-sections-v1`；没有 managed 索引，未静默 import。
- 采集脚本的旧 import-kind 定位、中文预设名称定位、窄屏拾取后未返回 Preview，以及专项测量未等待稳定的首次失败均保留并补齐。浮层辅助探针曾量到 X，后按可访问名称单独核对真正 Confirm；不以失败探针或错误元素证明产品效果。
- 必需状态/视口/主题矩阵已采，各单元都有有效结论；更多参数组合、完整 14 字段编辑结果、完整断点、模拟预览宽度切换、菜单动作、浮层动态换主题、冲突决策执行结果、读屏/OS IME/真机等仍未验证。独立原文的证据边界全部保留，不把本轮分数视作完整 UAT 或 WCAG 认证。
- 两次启动的本轮临时服务均已停止，4181/4176 无监听，见[服务记录](DOM-COMPONENTS-P1.ui-captures/r6/services.json)。没有产品代码、原计划或 R1–R5 报告改动，无提交、推送或发布。

## 下一步

```text
$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md
```

处理 UI-217，并按本报告复查 UI-216；修复后再运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`。本轮不自行启动修复或代码复审。

## 状态登记

本轮评审 ID：`4096c476-5ee3-44a5-a195-09a5183542ca`，见[R6 状态](../../.xgent-ai/sdlc/status/review/4096c476-5ee3-44a5-a195-09a5183542ca.md)。按 v3 协议在同对象、同 UI-R6 范围追加补证/纠错事件，保留初版报告快照及原事件；当前为 `review-recorded` / `review-pending`。R5 六条修复关闭、UI-115 接受偏差的处置保持，不改 R5 原评分与判定。Ready、7/7、合并及发布状态均未改变；报告、截图与台账未提交、未推送。
