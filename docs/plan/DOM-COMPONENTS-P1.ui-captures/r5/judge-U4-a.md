仅评 U4。已逐张打开 R5 全部 17 张原图，并打开 R4 白名单中的 17 张同名原图复核；未读取代码、修复记录或机器检查。U4 为规范模式，按目标包 S1–S6、G1–G4 评判。

1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | **3.00**：桌面与 390px 待确认态均无关键内容遮挡；冲突下载出口及展开内容对齐问题已经修正，所见范围没有布局失分项。 | **3.00**：中性壳层、状态用色、文字层级和按钮主次符合规范；配额态的两颗实心按钮分别承担下载与单项重试，所见范围没有视觉失分项。 | **3.00**：已记录的升级、取消、成功、配额失败、冲突与无锁状态均给出真实状态及继续入口，所见证据没有额外失分项；未执行的动作另列待核实。 | **0.95**：升级成功文案已精简，配额与无锁反馈仍有少量重复，尚未达到逐处无差异。 | **9.95 / 10** |

R4 为 9.55。本轮可挽回分合计 **0.05**，与 `10 − 9.95` 一致。

2. 上一轮差距复核表

| 编号 | 结论 | R5 原图复核结果 |
| --- | --- | --- |
| UI-201 | **已解决** | `quota-failure`、`quota-operations`：callout 已移除「Retry save」；命名卡改为「Retry “Retained name”」；操作区为「Retry all retained saves」；页头「Save locally」降为描边。单项、批量和下载职责已能区分。 |
| UI-203 | **部分解决** | `upgraded` 已将保留数量与草稿合成一句，「wasn't saved before the upgrade」也明确了时间；无锁命名卡不再复述浏览器不支持保存的原因。但配额 callout 与命名卡仍各说一次同一保存失败，无锁标题和正文仍重复同一原因。 |
| UI-202 | **已解决** | 两张配额图均显示单行「Storage full · saving in memory」，正文点名失败的「Retained name」，不再剩孤词「memory」。 |
| UI-204 | **已解决** | `conflict`、`conflict-detail-end`：冲突说明已合并，移除了无 pending 时的额外说明；默认帧内「Download other version」按钮与面板底边均完整可见。 |
| UI-205 | **已解决** | `upgrade-cancelled` 命名卡内新增描边「Review upgrade」；`no-locks` 命名卡内新增描边「Download current theme」。每张反馈卡都有就地继续动作。 |
| UI-56 | **已解决** | `quota-operations`：说明与列表左缘统一到摘要文字起点；第三条操作及完整的「(draft revision 2)」均可读，修订号片段未被拆开。 |
| UI-206 | **已解决** | `conflict-diff`：分组标题、分隔线、token 与左列值统一左缘，不再保留原来的额外 8px 内缩；披露三角与披露文字的缩进属于正常控件结构。 |

3. 差距清单

**UI-203 · U4 · 细节 · P3 · 可挽回 0.05**

- **位置与截图**
  - `U4-quota-failure-1440-light.png`、`U4-quota-operations-1440-light.png`：上方红色存储 callout 的正文与紧接其后的命名卡。
  - `U4-no-locks-initial-1440-light.png`、`U4-no-locks-1440-light.png`：黄色 callout 标题与正文第一句。
- **现状 → 期望**
  - 配额 callout 写「Saving “Retained name” failed and is kept for retry.」，下面命名卡又写「Save “Retained name” · failed; retained for retry.」；同一对象、结果与保留状态连续重复。
  - 无锁标题已有「local saving unavailable」，正文又以「Local saving is unavailable in this browser」开头。
  - 期望按 **S4 的“命名库动作单独反馈”**组织信息：存储 callout 说明整体存储状态，命名卡说明具体动作；同一原因无需连续重述。沿用 R4 UI-203 的判定尺度，仅扣剩余文案打磨分。
- **具体修法**
  - 配额 callout 保留「Storage full · saving in memory」、当前草稿是否已保存的说明、下载入口与技术详情；删除其中点名「Retained name」失败的句子，由下面命名卡独立承担该反馈。
  - 无锁 callout 保留现有标题，正文直接给继续方式，例如「You can edit in memory and download or export your theme.」。
  - 保留现有命名卡内联动作，无需改布局或状态色。

未发现需要新增编号的扣分问题。

4. 建议（不扣分）

目标包没有规定迁移对话框标题必须固定。`upgrade-review-json` 滚动后标题与关闭按钮移出可视区，但底部确认、下载和取消仍完整可见；若希望长内容审阅时持续保留上下文，可以固定标题栏。这属于可选交互改进，不计差距。

5. 看不出的格子

以下缺证据项既不算通过，也不算失败：

| 图片或记录范围 | 无法确认的内容 |
| --- | --- |
| 所有冲突截图及操作记录 | 展示了 Load / Keep / Save copy，但没有实际点击后的结果，无法确认三条决策的最终反馈及数据结果。 |
| `quota-failure`、`quota-operations` | 未点击单项或批量重试；无法确认恢复存储后的重试结果。Technical details 也未展开。 |
| `upgrade-review-json` | JSON 只显示开头；无法从该帧检查两份命名主题、两模式值及全部偏好的完整迁移正文。 |
| `conflict-raw` | 原始存储文本只显示开头，无法检查全部文本。 |
| `upgraded` | 可确认界面在确认后显示升级成功；记录未包含重新加载后的持久化结果。 |
| 所有下载入口 | 截图证明入口可见；记录没有实际下载及产物结果。 |
| U4 全部截图 | 没有 U4 控件的键盘走查、焦点帧、待锁排队过程、中文迁移文案或无锁状态下的偏好切换结果，不能据此确认这些行为。 |

实际打开的图片如下；**R5、R4 两个目录中的每个同名文件均已打开，共 34 张**：

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
```

6. 总评

本次仅评 U4，得分 **9.95**。R4 的动作重复入口、失败原因不清、冲突下载按钮出屏、卡内缺少继续动作及展开内容对齐问题均已解决；剩余失分是两类反馈中的少量文案重复。所见界面无需结构性重做，精简 UI-203 对应文字即可。
