## 1. 评分表

仅评 U4，采用冻结目标的规范模式。本轮 23 张原图已逐张实际查看。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | **3.00**：桌面反馈区、Review 分组及动作区层级清楚，390px 待确认态的说明与关键操作完整可见；未见有规范依据的布局偏差，已达本维度上限。 | **3.00**：编辑壳保持中性，警告、失败、成功的用色及标题、正文、辅助文字层级清楚；未见可据 G1、G3 扣分的视觉差距，已达上限。 | **3.00**：记录与画面支持 Review、取消后继续编辑、重新确认和升级成功；键盘入口、确认焦点及 Escape 返回入口均有证据，已取证范围内达到上限。 | **1.00**：所见版本、修订号、命名动作反馈、草稿保存状态及按钮对齐未发现规范差异，已达本维度上限。 | **10.00 / 10** |

可挽回分合计 **0.00**，等于 `10 − 10.00`。分数仅覆盖截图与操作记录能够支持的范围。

## 2. 上一轮差距复核表

| 编号 | 状态 | 本轮依据 |
| --- | --- | --- |
| UI-203 | **已解决（维持）** | 对照实际打开的 R7 配额失败、无锁原图，本轮对应反馈的文案分层和动作结构一致，未观察到上一轮所述重复文案问题的回归。 |
| 上一轮未关闭差距 | **无** | 所提供的 R7 独立结论没有列出未关闭差距；本轮也未发现新的规范差距。 |

## 3. 所有仍未关闭的差距

**无。** 未发现能够同时给出明确规范依据、可执行修法和扣分理由的未关闭差距。

具体核对依据：

- **S1、S2**：待确认态说明旧草稿已恢复到内存，并提供 Review、下载原始数据和取消入口；取消后常驻“Memory only · upgrade cancelled”，提供重新审阅、下载当前主题及原始数据。尝试命名保存后明确显示未保存。
- **S3**：Review 展示 `v1 → v2`、原始指纹、两个保留主题、将保存的当前草稿及偏好；完整升级 JSON 的首段、中段和末段可见。滚动 JSON 后，确认、下载原始数据和取消动作仍可见。升级成功反馈出现在确认操作之后。
- **S4**：配额流程记录先完成草稿保存，再触发命名保存失败；顶部“Draft saved · action failed”与操作列表中的草稿成功、命名失败相符，不能据此认定虚报保存成功。
- **S5**：冲突区展示本页与另一页的修订号；展开差异后，`primary` 的两边值分别可读，并明确提供 Load、Keep、Save copy 三种决策。
- **S6**：无锁初始态及尝试保存后均显示仅内存状态和下载入口；命名动作明确反馈“not saved; no local write was made”。
- **G4**：Review 入口及确认按钮有独立可见焦点；记录支持 Enter 打开、Escape 关闭，关闭后入口焦点在图中可见。

## 4. 建议（不扣分）

无另列建议。

## 5. 看不出的格子与证据边界

**冻结矩阵内的必需格子：**

- 桌面待确认、Review、取消、成功、配额失败、冲突、无锁，以及 390px 待确认均有清楚原图，没有整个必需状态因缺图或模糊而无法评价。
- Review／确认键盘焦点、Escape 返回、升级 JSON 中末段及冲突原文末尾均有对应画面。
- `U4-conflict-raw-end-1440-light-a2.png` 中，`preview_width` 长行的右端超出文本框当前可见宽度，无法读全该行；末尾闭合结构与下载按钮可见。没有横向滚动记录，故右端内容能否通过滚动查看仍无法判断，既不据此认定不可访问，也不将整行可读性记为已证明。
- JSON 首、中、末段截图不能穷尽核对每一行及迁移数据保真性；这里只确认所拍位置的呈现。

**正常滚动裁切：**

- 展开升级 JSON 后，Review 标题和部分上文已滚出视口；冲突详情的不同截图也处于编辑列不同滚动位置。这些与操作记录相符，不能作为内容缺失或遮挡扣分。
- 视口底部未展示完的后续工作台内容，不构成该状态格缺证。

**矩阵外扩展行为或记录未执行的行为：**

- Load／Keep／Save copy 执行后的结果，Retry／Dismiss／取消待写入的实际结果，以及下载入口是否真正产出文件，未由本包记录证明。
- 完整 Tab 循环、焦点限制、其余存储控件的键盘状态，以及等待锁、排队和处理中继续编辑的行为，未充分取证。
- 上述边界既不判为成功，也不判为失败，不据此扩大本轮必需截图矩阵。

## 6. 总评

本次仅评 U4，得分 **10.00/10**，不作跨单元排序。已呈现的迁移闭环、存储失败与冲突反馈符合冻结规范，没有观察到需要重做布局、替换组件或重做素材的问题。未执行动作的结果仍保留为证据边界。

## 实际开图工具与清单

工具：`functions.exec` 调用 `tools.view_image`，全部使用 `detail: "original"`，实际查看返回图片。共 **26 张：R8 23 张、R7 对照 3 张**。

绝对目录前缀：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/
```

该目录下实际打开：

```text
U4-upgrade-pending-1440-light.png
U4-upgrade-review-1440-light.png
U4-upgrade-review-end-1440-light.png
U4-upgrade-cancelled-1440-light.png
U4-upgraded-1440-light.png
U4-upgrade-pending-390-light.png
U4-quota-failure-1440-light.png
U4-quota-operations-1440-light.png
U4-conflict-1440-light.png
U4-conflict-detail-1440-light.png
U4-conflict-raw-1440-light.png
U4-no-locks-initial-1440-light.png
U4-no-locks-1440-light.png
U4-conflict-detail-end-1440-light.png
U4-conflict-after-1440-light.png
U4-conflict-diff-1440-light.png
U4-upgrade-review-json-1440-light.png
U4-upgrade-review-keyboard-focus-1440-light.png
U4-upgrade-confirm-keyboard-focus-1440-light.png
U4-upgrade-review-keyboard-escape-1440-light.png
U4-upgrade-json-middle-1440-light.png
U4-upgrade-json-end-1440-light.png
U4-conflict-raw-end-1440-light-a2.png
```

上一轮对照图的绝对目录前缀：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/
```

该目录下实际打开：

```text
U4-quota-failure-1440-light.png
U4-no-locks-1440-light.png
U4-conflict-raw-end-1440-light.png
```
