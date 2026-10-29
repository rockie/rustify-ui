1. **评分表**

本轮仅评 U4，采用目标包的规范模式；本轮 23 张原图已全部实际打开。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | **3.00**：桌面反馈区、升级审阅与冲突详情层级清楚；390px 待确认态的说明及主要操作完整可见，所见范围没有阻挡主路径的溢出、遮挡或截断，达到本维度上限。 | **3.00**：编辑壳保持中性，待确认／仅内存、失败和成功反馈的颜色语义明确，文字与按钮可辨；未发现可依据 G1–G4 扣分的视觉差距，达到本维度上限。 | **3.00**：记录与截图支持审阅、取消后继续编辑、重新确认及成功反馈；补采明确显示 Review／确认按钮的键盘焦点、Enter 打开、Escape 关闭并返回入口焦点，达到已取证范围的上限。 | **1.00**：所见迁移文案、版本、修订号、命名保存反馈及按钮对齐未发现新的规范差异，达到本维度上限。 | **10.00 / 10** |

满分仅覆盖本次可见且有操作记录支持的范围。可挽回分合计 **0.00**，等于 `10 − 10.00`。

2. **上一轮差距复核**

| 编号 | 状态 | 本轮依据 |
| --- | --- | --- |
| UI-203 | **已解决（维持）** | 对照实际打开的 R6 配额失败／无锁原图，本轮对应反馈未出现上一轮结论所述的文案重复回归。 |
| R6 未关闭差距 | **无** | 所提供 R6 独立结论明确记载“上一轮未关闭差距：无”。 |

3. **仍未关闭的差距清单**

**无。** 本轮没有发现能够同时给出明确规范依据、可执行修法与扣分理由的未关闭差距。

核对要点：

- **S1–S3**：待确认态明确说明旧草稿已恢复到内存；Review 展示版本、原始指纹、两个保留主题、当前草稿和偏好；确认动作与取消动作明确，原始数据下载入口可见。取消后常驻仅内存提示、重新审阅入口及当前主题出口均可见。
- **S3**：完整升级 JSON 的首段、中段和末段均已实际查看；末段显示偏好及闭合结构，滚到中末段后确认、下载原始数据和取消动作仍可见。
- **S4**：配额流程先完成草稿保存，再触发命名保存失败；因此顶部的草稿已保存／动作失败与操作列表中的草稿成功、命名失败相互一致，不能将其判作虚报保存成功。
- **S5**：冲突显示两边修订号、差异字段及两边值，并明确提供 Load、Keep、Save copy 三个决策；没有从这些入口帧推断其执行结果。
- **S6**：无锁初始态及尝试命名保存后的状态均明确为仅内存，并提供下载；命名动作反馈明确表示未保存、未作本地写入。

4. **建议（不扣分）**

无另列建议。

5. **看不出的格子与未测边界**

**冻结矩阵内：**

- 桌面待确认、Review、取消、成功、配额失败、冲突、无锁，以及 390px 待确认均有清楚原图；没有因缺图或模糊而无法判断的必需状态格。
- 补采的 Review／确认键盘焦点、Escape 返回、升级 JSON 中末段及冲突原文末尾均可辨认。
- `U4-conflict-raw-end-1440-light.png` 中，原文区域的 `preview_width` 长行右端超出当前可见宽度，无法从该帧读全该行；末尾闭合结构及下载按钮可见。截图不能证明横向滚动是否可达，因此不将它推断为不可访问或数据丢失。
- 分段截图不能穷尽核对 JSON 的每一行及数据保真性；本结论只确认相应呈现与已拍位置，没有替代数据检查。

**矩阵外或记录未执行的行为：**

- 冲突 Load／Keep／Save copy 执行后的结果，失败后的 Retry／Dismiss／取消待写入，以及各下载入口是否真正产出文件，均未由本包记录证明。
- 完整 Tab 循环、焦点限制、其余存储控件的键盘状态、等待锁与排队过程、处理中继续编辑的行为未充分取证。
- 上述项目保留为未测边界，既不判通过，也不判失败；不据此扩大本轮必需截图矩阵。

6. **总评**

本轮只评 U4，得分 **10.00/10**。未发现需要调整布局结构、替换组件或重做素材的问题。本轮补采进一步支持了键盘返回和长文本滚动位置的可见性；尚未执行的动作结果仍保留为未测。

**实际开图工具：** `functions.exec` 调用 `tools.view_image`，全部使用 `detail: "original"` 并实际查看返回图片。共 **25 张：R7 23 张，R6 对照 2 张**。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-pending-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-review-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-review-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-cancelled-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgraded-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-pending-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-quota-failure-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-quota-operations-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-raw-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-no-locks-initial-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-no-locks-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-detail-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-after-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-diff-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-review-json-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-review-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-confirm-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-review-keyboard-escape-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-json-middle-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-upgrade-json-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U4-conflict-raw-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-quota-failure-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-no-locks-1440-light.png
```
