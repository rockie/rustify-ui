1. 评分表

仅评 U1。已读取冻结目标包及完整变更记录；UI-115 的接受范围属于 U2，不改变 U1 的 G4 可见焦点要求。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 2.5 | 1.0 | **9.5/10** |

- **布局 3.0**：已达上限。桌面双栏、768px 换行、390px 编辑/预览切换及返回编辑、200% 文本重排均未发现应扣分的结构、遮挡或横向可达性问题。
- **视觉 3.0**：已达上限。浅色中性编辑壳、深色预览隔离、选中态、扁平字段标签页和焦点样式符合本单元规范，未发现新的静态视觉偏差。
- **交互与状态 2.5**：未达 3.0，因为已配置为 Pink 后展开品牌色选择器并进行键盘移动的补采帧，没有显示任何可见的键盘活动项，详见「新-1」。
- **细节 1.0**：已达上限。按规范模式比较，已展示的中英文文案、覆盖数量、auto 解析结果、接缝和对齐未发现另外应扣分的差异。

2. 上一轮差距复核表

R5 原审没有未关闭差距；以下复核其已关闭项目是否保持。

| 编号 | 本轮状态 | 证据与结论 | 本轮扣分 |
| --- | --- | --- | ---: |
| UI-02 | 已解决，保持关闭 | R6 的 `U1-default-1440-light.png`、`U1-zh-top-1440-light.png`、`U1-edit-390-light.png`、`U1-preview-390-light.png` 与对应 R5 原图对照，语言及编辑/预览分段接缝未见相邻第二条边线回归；模式按钮焦点仍内收。 | 0 |
| UI-193 | 已解决，保持关闭 | R6 的两张桌面中文浅色图，以及新增 768px、390px 中文帧中，诊断标题的汉字、数字与拉丁字符未见此前描述的明显字重不一致。 | 0 |

「新-1」属于新增覆盖状态发现的问题；R5 没有提供已配置 Pink 后展开列表的对应帧，不能据此断言发生了代码回归。

3. 差距清单

**新-1 · U1 · 已配置品牌色后的选择列表缺少可见键盘活动项**

- **维度／严重度／可挽回分：交互与状态／P1／0.5 分。**
- **截图与位置：**
  - `U1-matrix-keyboard-03-768-light.png`：左侧 Brand color 展开列表。
  - `U1-matrix-keyboard-03-390-light.png`：编辑区 Brand color 展开列表。
  - `U1-matrix2-keyboard-03-1440-dark.png`：左侧 Brand color 展开列表。
- **现状：**三张图的触发器均显示 `Pink`，可见列表均从 `Custom` 排到 `Orange`，可见选项没有活动项蓝底、蓝字或其他键盘焦点标识；触发器原有焦点环也已消失。操作记录将这组流程标为真实键盘聚焦以及 `Enter/ArrowDown/Escape`。键盘用户在这个已拍状态无法从画面辨认当前正在移动到哪个选项。
- **期望与依据：**目标 **G4「新增控件键盘可用、焦点可见」**要求键盘活动位置能够被看见。对照本轮默认状态的 `U1-keyboard-03-1440-light.png`，其中 `Custom` 有选中勾，`Gray` 有清楚的蓝色活动行；已配置状态也应提供同等可辨认反馈。
- **具体修法：**修改 Studio 基础配置的品牌色选择器及其列表弹层：展开时定位当前值；每次方向键改变活动项后，将该项滚入列表可视区，例如对活动项执行 `scrollIntoView({ block: "nearest" })`；活动项保留蓝色背景／文字反馈，当前值保留独立选中标识。修后从 Pink 这类位于首屏列表之外的值开始，补拍展开、连续方向键移动、确认选择和 Escape 返回焦点。
- **判断边界：**截图不能确定活动项是落在列表可视区外，还是活动样式未呈现；这两种实现原因需要修复方定位。本项只扣已经可见的反馈缺失，不推断键盘确认必然失败。

可挽回分合计 **0.5**，等于 **10 − 9.5**。

4. 建议（不扣分）

补拍一次键盘确认选项及键盘激活预览模式后的结果，并增加一次分隔条拖拽记录。这些未展示行为不计失败。

5. 看不出的格子

| 图片或记录 | 无法确认的内容 |
| --- | --- |
| 所有品牌色展开帧 | 仅展示列表一部分，无法核实完整 26 项；也未展示其他基础选择器的完整选项。 |
| 三张「新-1」证据图 | 无法确定不可见活动项的内部原因，也无法判定随后按 Enter 会选择哪个值。 |
| `keyboard-03`、`keyboard-04` 各组与操作记录 | 未展示键盘确认选项后的结果；模式焦点帧不等于键盘激活模式后的结果。 |
| 中文配置帧 | 未完整展示中文基础配置末尾的缩放区及所有选项，不能确认这些未展示文字。 |
| 桌面、768px、200% 框架图 | 无法核实分隔条的 8px 命中范围、拖拽行为及 260–520px 限制。 |
| `U1-zoom200-export-end-720-light.png` | Copy code、Select text、Download 可见，但没有对应执行结果。 |
| U1 全部材料 | 未展示保存失败、撤销／重做的执行结果；截图也不能证明预览模式切换前后作者数据内部完全相同。 |

6. 总评

本次唯一受评单元 U1 为 **9.5/10**。工作台布局与静态视觉不需要结构性重做；需要修复品牌色列表在已有配置下的键盘活动项可见性。补采增加了有效覆盖，也暴露了默认 Custom 状态没有体现的问题。

实际使用开图工具：**`tools.view_image`，`detail: "original"`**。实际打开 **52 张**：R6 当前原图 46 张、R5 对照原图 6 张。U1 为规范模式，没有对应目标 PNG；未打开其他单元的目标图。未写文件，未读取代码、机器检查或其他报告。

实际打开的图片绝对路径：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-default-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zh-top-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zh-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-default-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-edit-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-colors-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light-a2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-01-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-02-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-03-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-04-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-lower-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-configured-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-01-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-02-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-04-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-configured-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-01-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-02-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-04-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix3-zh-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-default-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-zh-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-zh-configured-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-01-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-02-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-03-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-04-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-default-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-zh-top-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-zh-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-edit-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-keyboard-03-1440-light.png
```
