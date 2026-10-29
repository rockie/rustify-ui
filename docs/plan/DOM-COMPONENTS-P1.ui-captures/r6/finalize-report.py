from pathlib import Path

p = Path(__file__).resolve().parent
report = p.parent.parent / 'DOM-COMPONENTS-P1.ui-review.r6.md'
base = (p / 'revisions/report-before-matrix-supplement.md').read_text()
opening = base.split('## 验收判定', 1)[0]
history = base.split('## 轮次轨迹', 1)[1].split('## 上轮差距复核', 1)[0]
history = history.replace('| R6 | **9.7（U7）** | **9.7** | **0 / 0** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；新增可选 P2 UI-216；通过验收 |', '| R6 | **9.5（U1）** | **9.5** | **0 / 1** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；补采发现 P1 UI-217，另有 P2 UI-216；需要修改 |')
old_gaps = base.split('## 上轮差距复核', 1)[1].split('## 差距清单', 1)[0]
old_gaps = old_gaps.replace('judge-U7-a.md)；当前', 'judge-U7-a.md)及[补评](DOM-COMPONENTS-P1.ui-captures/r6/judge-U7-b.md)；当前')
old_gaps = old_gaps.replace('原文未覆盖，映射记录见', '评审原文均保留；映射记录见')
code_gap = base.split('### UI-216', 1)[1].split('## 待裁定事项', 1)[0]
code_gap = code_gap.replace('- 主 agent 核对：', '- 评审合并：U7-b 给出 10.0 且未另列此项；按全部评审差距并集保留 U7-a 的 UI-216，并采用其完整的 9.7 四维分，不删差距。\n- 主 agent 核对：')
machine = base.split('## 机器检查', 1)[1].split('## 采集与覆盖', 1)[0]
machine = machine.replace('全部在本轮真实 release 页面上执行，晚于七份独立原文和合并 judge 的落盘。检查结果没有反馈给评审调分。', '全部在本轮真实 release 页面上执行。基础检查位于首批七份独立原文落盘之后；补评发现 UI-217 后追加专项测量。所有机器结果均与独立评审白名单隔离，没有反馈给评审调分。')
machine = machine.replace('48 个受监视页面：pageerror、console error、失败请求/HTTP ≥400 均为 0；47 页成功回收 CSP 日志，事件 0。首次定位失败的 1 页未回收 CSP，不写为该页已通过；相同选择器状态在后续实测重新覆盖', '65 个受监视页面：pageerror、console error、失败请求/HTTP ≥400 均为 0；62 页成功回收 CSP 日志，事件 0。三次采集脚本失败的 3 页未回收 CSP，不写为这些页已通过；相关界面状态在后续成功补采')
machine = machine.replace('机器检查没有新增扣分或门槛项，不改变独立评分。', '''| UI-217 活动项可见性 | 768 浅色、390 浅色、1440 深色均实测：Pink 后 Enter/ArrowDown，aria-activedescendant 指向 Plum，活动行有蓝底但位于列表滚动视窗之外，scrollTop 均为 0；Enter 实际选中 Plum。首测 390/1440 未等稳定而定位超时，增加等待后的两页复测成功；失败原始记录保留 | [首测](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu.json)、[复测](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu-confirm.json)、[窄屏实拍](DOM-COMPONENTS-P1.ui-captures/r6/MC-pink-menu-confirm-390-light.png) |

UI-217 是独立评审发现的 P1，机器检查复现并定位到活动项未进入列表可视区，没有另扣分；不把键盘能够确认选项误写为焦点反馈合格。专项两个尝试共 5 页的 console/CSP/失败请求均为 0，只有其中 3 页完成上述选项测量。''')

text = opening + '''## 验收判定

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
''' + history + '''## 上轮差距复核
''' + old_gaps + '''## 差距清单

当前 **2 条：P1 1 条、P2 1 条**。两项均可执行，无待核实门槛项或待裁定事项。

### UI-217 · [P1] 已选品牌色后的菜单活动项不在可视区域（U1 · 交互与状态 · 可挽回 0.5）

- 位置：[768 浅色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-768-light.png)、[390 浅色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-390-light.png)、[1440 深色](DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-03-1440-dark.png)的 Brand color 展开列表。
- 现状 → 期望：触发器为 Pink，真实 Enter/ArrowDown 后列表仍从 Custom 排到 Orange，可视区域没有活动行；默认 Custom 起点的同轮图能看到 Gray 蓝色活动行。按[目标 G4](DOM-COMPONENTS-P1.ui-target.r6.md)，已配置状态也必须让键盘活动位置可见。
- 修法：在基础配置品牌色选择器展开和方向键更新时，将当前活动项滚入弹层自己的滚动视窗，例如 nearest 对齐；保留活动行蓝色反馈与当前值的独立选中标识。修改选择器的定位/滚动行为，无需重排工作台。
- 复验：从 Graphite → Pink → Auto 出发，覆盖上述三个组合，键盘展开、连续上下移、Enter 确认、Escape 回焦；每个关键帧能直接辨认当前活动项。只给不可见选项加样式不足以关闭。
- 来源：U1-b 与 [U1-c 独立原文](DOM-COMPONENTS-P1.ui-captures/r6/judge-U1-c.md)的“新-1”→ UI-217；相同位置与图证合并，采用 c 的 0.5 可挽回分。R5 没有 Pink 起点对应帧，记为新增发现，不断言代码回归。
- 主 agent 核对：三个组合的实际测量均显示活动项为 Plum、`data-active=true`、蓝底，但列表 `scrollTop=0`，活动行位于列表滚动视窗之外；Enter 能实际选择 Plum。[测量及失败重试](DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu-confirm.json)支持可见性缺陷，未改变独立分数或严重度。

### UI-216''' + code_gap + '''## 待裁定事项

无。UI-115 已有用户方向；UI-217 与 UI-216 均可局部修复，尚未经历针对它们的修复轮，不触发顽固差距或重构要求。

## 建议（不扣分）

- U2：Neutral 影响清单可按 Light/Dark 分节，当前范围已完整表达。
- U4：长迁移 JSON 审阅可固定标题栏；目标未要求，底部操作仍可达。
- U1：可增加键盘激活模式及分隔条拖拽记录；UI-217 的后置实测已经补充 Enter 选择结果，但不回写成评审新分数。
- 沿用既有建议：Caller utilities 的人为 class 覆盖样例对比度为 1.24:1，可换成更易读的演示配色；它不作为默认组件的 AA 通过证据。

## 机器检查
''' + machine + '''## 采集与覆盖

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
'''
assert '9.7（U7）** | **9.7** | **0 / 0' not in text
assert '**需要修改**' in text and 'UI-217' in text
text = text.replace('本轮为冻结 UI 矩阵验收 |\n\n| UI-217', '本轮为冻结 UI 矩阵验收 |\n| UI-217')
text = text.replace('\n\n\n', '\n\n')
report.write_text(text)
print(report)
