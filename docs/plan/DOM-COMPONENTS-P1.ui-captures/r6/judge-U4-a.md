仅评 U4。已读取冻结目标包及完整变更记录、R6 清单与操作记录、R5 独立评审；实际打开 R6 全部 17 张原图及 R5 的 8 张对应原图。未读代码、修复记录、机器检查或其他单元材料，未修改文件。

U4 为规范模式，没有本单元目标 PNG；以下按 S1–S6、G1–G4 及既定比较口径评分。UI-115 的范围接受不影响 U4。

**1. 评分表**

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | **3.00**：桌面状态区、冲突详情及升级审阅层次清楚，390px 待确认态的主要操作完整可见；所见范围没有遮挡、异常换行或出口出屏的扣分项。 | **3.00**：中性壳层、黄色待确认／仅内存反馈、红色失败反馈、绿色成功反馈及按钮主次保持一致，所见范围没有违反规范的视觉差距。 | **3.00**：记录支持升级审阅、取消、重新确认及成功反馈，失败与冲突状态也展示了明确的继续入口；未执行动作的结果列入缺证据，不补造通过结论。 | **1.00**：R5 剩余的配额与无锁文案重复已经删除，所见文案、动作名称、修订号及对齐未发现新的可扣分差异。 | **10.00 / 10** |

沿用 R5 的评判尺度，R5 为 **9.95**，本轮关闭 UI-203 剩余 **0.05** 分。当前可挽回分合计 **0.00**，等于 `10 − 10.00`。满分仅覆盖本次可见、可从记录核实的范围。

**2. 上一轮差距复核表**

| 编号 | 本轮结论 | R6 复核结果 |
| --- | --- | --- |
| UI-201 | **已解决（保持）** | 两张配额图中，整体存储卡提供下载；命名卡提供 `Retry “Retained name”`；操作列表外提供 `Retry all retained saves`；顶部 `Save locally` 保持描边，职责仍可区分。 |
| UI-203 | **已解决** | 配额卡已删除点名命名保存失败的重复句，只保留当前草稿是否已保存的信息，具体失败与重试由下面命名卡承担；无锁正文直接写可在内存编辑及下载／导出，不再复述标题中的不可本地保存原因。升级成功文案保持精简。 |
| UI-202 | **已解决（保持）** | 两张配额图的 `Storage full · saving in memory` 完整位于同一行；具体失败对象由命名卡清楚标明，没有孤立的 `memory` 尾词。 |
| UI-204 | **已解决（保持）** | `conflict` 与 `conflict-detail-end` 中，冲突说明、三个决策及 `Download other version` 完整可见，面板底边也在视口内；未恢复冗余的无 pending 说明。 |
| UI-205 | **已解决（保持）** | 取消升级的命名反馈卡仍有就地 `Review upgrade`；无锁命名反馈卡仍有 `Download current theme`，无需离开反馈卡寻找继续动作。 |
| UI-56 | **已解决（保持）** | `quota-operations` 的说明与列表统一对齐到摘要文字起点，三条操作完整可读，`(draft revision 2)` 没有拆成不完整片段。 |
| UI-206 | **已解决（保持）** | `conflict-diff` 的分组标题、分隔线、token 与左列值保持同一左缘，未恢复额外内缩。 |

**3. 差距清单**

未发现仍未关闭的扣分差距，也未发现需要新增临时编号的问题。

- 未关闭差距：**0 条**
- 可挽回分合计：**0.00**
- 无需结构性改造。

**4. 建议（不扣分）**

延续 R5 的可选建议：`U4-upgrade-review-json-1440-light.png` 中，展开完整 JSON 后，审阅标题与关闭按钮已随内容滚出视口。可以将审阅对话框标题栏固定在内部滚动容器顶部，帮助长内容审阅时保留上下文。目标未要求固定标题，底部确认、下载、取消仍完整可见，因此不扣分。

**5. 看不出的格子**

以下均为证据边界，既不算通过，也不算失败。

| 图片或记录范围 | 看不出的内容 |
| --- | --- |
| 所有冲突截图与操作记录 | 未点击 Load / Keep / Save copy，无法确认三条决策执行后的反馈与数据结果；`conflict-after` 记录的是进一步滚动，并非完成冲突处理。 |
| `quota-failure`、`quota-operations` | 未点击单项重试、批量重试或取消待写，无法确认恢复存储后的处理结果；Technical details 未展开。 |
| `upgrade-review-json` | JSON 仅显示开头，不能逐项核对两份命名主题、两模式值及全部偏好的完整迁移正文。 |
| `conflict-raw` | 存储原文仅显示开头，不能核对完整原始文本。 |
| `upgraded` | 可确认显式确认后界面显示升级成功；没有重新载入后的截图或操作结果，无法确认重载后的持久化内容。 |
| 所有下载入口 | 可以确认入口可见；没有实际下载及产物结果，不能确认文件内容。 |
| U4 全部截图与记录 | 没有专门的键盘走查、键盘焦点帧、悬停／按下状态、待锁排队过程、中文迁移文案、无锁时偏好切换结果，无法判断这些状态。 |
| U4 全部截图与记录 | 没有本单元缓存损坏、storage 完全不可用等额外故障帧，无法判断对应反馈。 |

**6. 总评**

本次仅评 U4，得分 **10.00**，不对其他单元作结论。R5 唯一剩余扣分项 UI-203 已解决；已关闭问题在本轮所见范围内没有回归。当前无需重做布局、替换组件或重做素材。未实际执行或未完整拍摄的状态，仍以上述缺证据表为准。

实际开图工具：**`tools.view_image`**，通过 `functions.exec` 调用并显示返回图像。实际打开 **25 张**，其中 R6 为 17 张、R5 为 8 张；绝对路径如下：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-pending-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-pending-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-review-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-review-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-review-json-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgrade-cancelled-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-upgraded-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-quota-failure-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-quota-operations-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-no-locks-initial-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-no-locks-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-detail-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-after-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-diff-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U4-conflict-raw-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-quota-failure-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-quota-operations-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-no-locks-initial-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-no-locks-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-conflict-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-conflict-diff-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-upgrade-cancelled-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U4-upgraded-1440-light.png
```
