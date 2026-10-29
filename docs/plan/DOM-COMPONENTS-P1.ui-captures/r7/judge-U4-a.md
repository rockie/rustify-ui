## 1. 评分表

只评 U4；按冻结目标 S1–S6、G1–G4 的规范模式比较。已实际查看本轮全部 17 张原图，并对照 R6 的 4 张相关原图。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
|---|---|---|---|---|---|
| U4 本地库迁移与存储反馈 | **3.00**：桌面状态区、冲突详情和升级审阅分组清楚；390px 待确认态的 Review、原始数据下载及取消入口完整可见，所见范围没有遮挡或异常溢出。 | **3.00**：编辑壳保持中性，待确认／仅内存、失败、成功分别使用一致的黄、红、绿反馈，文字层级及蓝色主要动作未见违反规范的偏差。 | **3.00**：记录与截图对应升级审阅、取消后继续编辑、重新确认及成功；配额失败、无锁、冲突均呈现明确状态和继续动作，未观察到可据规范扣分的问题。 | **1.00**：可见的动作名称、修订号、命名保存反馈和对齐未发现差异；U4 没有逐像素目标稿，不对规范未指定的像素值补造要求。 | **10.00 / 10** |

以上维度均已到评分上限。满分覆盖本轮可见、可由操作记录支持的范围；未拍摄或未执行的状态见第 5 节。

## 2. 上一轮差距复核

R6 提供的“上一轮未关闭差距”为**无**。

| 编号 | 状态 | 复核依据 |
|---|---|---|
| UI-203（R6 提及的历史关闭项） | **已解决，保持关闭** | 对照 R6／R7 的配额失败、无锁截图：配额面板说明草稿状态，命名动作面板单独说明失败与重试；无锁面板说明仅内存，命名动作面板说明该次操作没有本地写入，未见重复解释回归。 |

未发现需要重新打开的旧差距。

## 3. 未关闭差距清单

**无。**

当前可挽回分合计 **0.00**，等于 `10 − 10.00`。

具体判断：

- 升级待确认态明确旧草稿已恢复到内存，提供 Review、下载原始数据、取消；390px 下这些入口均可见。
- Review 展示 v1 → v2、原始指纹、两个保留的命名主题、当前草稿与偏好；确认按钮明确为“Upgrade and save current draft”，确认前显示等待确认。
- 取消后保留“Memory only · upgrade cancelled”、再次审阅及两种下载出口；尝试命名保存另行提示没有保存。
- 成功态显示库已升级、原主题和当前草稿保留；此前未保存的命名动作仍单独提供“Save … now”，没有把库升级与命名保存混为一次成功。
- 配额失败态顶部为“Draft saved · action failed”，操作列表对应两次已确认草稿和一次失败命名保存，符合本次先保存草稿、再注入配额错误的记录。
- 冲突态显示本标签 Revision 2、另一标签 Revision 3，以及 Load／Keep／Save copy 的结果语义；展开差异后能读到两侧 primary 值，原始文本和下载另一版本入口也可见。
- 无锁时初始即显示仅内存与下载出口；尝试命名保存后明确“not saved; no local write was made”，没有成功保存提示。

## 4. 建议（不扣分）

没有需要另行扩大实现范围的产品建议。后续宜优先补齐下列行为证据，而不是继续调整已符合规范的静态呈现。

## 5. 看不出的格子

| 图片或记录范围 | 无法据此确认的内容 |
|---|---|
| 本轮全部 U4 图片 | 没有真实 Tab／Shift+Tab 焦点帧，不能确认 G4 的完整键盘路径、焦点可见性及对话框焦点返回；采集清单中的焦点元素名称本身不证明键盘操作。 |
| `U4-upgrade-review-json-1440-light.png` | JSON 只显示开头；能确认完整迁移 JSON 的展开入口与文本容器，不能逐字段核对滚动区域内全部主题、草稿和偏好。 |
| `U4-conflict-raw-1440-light.png` | 原始文本只拍到前段，不能检查全文及后段长行的阅读情况。 |
| 冲突系列图片及 `U4-actions.log` | 没有实际执行 Load／Keep／Save copy 后的结果；也未覆盖冲突期间存在未处理命名保存的情形。 |
| 配额失败、操作列表及无锁图片 | 能确认继续入口可见，不能确认 Retry、Cancel pending、Download／Export 的实际执行结果及生成文件。 |
| `U4-upgraded-1440-light.png` | 能确认提交后的成功反馈，不能确认重新载入后的持久化内容及备份文件完整性。 |
| 本轮 U4 全部材料 | 未呈现待锁排队、先前快照成功但当前草稿仍待保存、缓存损坏和一般 storage 不可用的独立状态；不计通过或失败。 |
| 本轮 U4 全部材料 | U4 截图为英文、light；迁移反馈的中文文案没有本轮可见证据。390px 只采集待确认态，其他窄屏状态不作结论。 |

上述均为证据边界，未转换成产品缺陷或扣分。

## 6. 总评

本次只有 U4，得分 **10.00／10**；所见范围未发现需要结构性改变的布局或组件。当前主要限制是部分决策、恢复及键盘路径尚未形成可见的执行证据。

## 实际开图清单

工具：**`tools.view_image`，`detail: "original"`**。以下均实际开图查看，未用尺寸检查替代。

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

- `U4-upgrade-pending-1440-light.png`
- `U4-upgrade-review-1440-light.png`
- `U4-upgrade-review-end-1440-light.png`
- `U4-upgrade-cancelled-1440-light.png`
- `U4-upgraded-1440-light.png`
- `U4-upgrade-pending-390-light.png`
- `U4-quota-failure-1440-light.png`
- `U4-quota-operations-1440-light.png`
- `U4-conflict-1440-light.png`
- `U4-conflict-detail-1440-light.png`
- `U4-conflict-raw-1440-light.png`
- `U4-no-locks-initial-1440-light.png`
- `U4-no-locks-1440-light.png`
- `U4-conflict-detail-end-1440-light.png`
- `U4-conflict-after-1440-light.png`
- `U4-conflict-diff-1440-light.png`
- `U4-upgrade-review-json-1440-light.png`

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`

- `U4-quota-failure-1440-light.png`
- `U4-no-locks-1440-light.png`
- `U4-upgrade-review-json-1440-light.png`
- `U4-upgrade-pending-390-light.png`

U4 为规范模式，没有对应目标 PNG，未打开其他单元的目标图。
