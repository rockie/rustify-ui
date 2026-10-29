1. 评分表

仅评 U1。已读取冻结目标包及完整变更记录；U1 为规范模式，没有对应目标 PNG。UI-115 的范围接受不改变 G4 的可见键盘焦点要求。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 2.6 | 1.0 | **9.6/10** |

- **布局 3.0**：已达上限；桌面双栏、768px 重排、390px 编辑/预览切换及返回入口、200% 来源区和导出动作区，未发现违反 G1–G2、G6 的遮挡或横向溢出。
- **视觉 3.0**：已达上限；浅色中性壳未被深色预览污染，文字层级、选中处理、分隔与扁平字段页签未见可辨的规范偏差。
- **交互与状态 2.6**：未到 3 分，因为 Pink 配置下的三组键盘展开截图均没有可见的菜单活动行；搜索、选择器触发器、模式按钮和字段页签自身的焦点则清楚可见。
- **细节 1.0**：已达上限；在已拍状态中，未发现需要另扣细节分的接缝、对齐、文字截断或中英文状态文案差异。

2. 上一轮差距复核表

| 编号 | 状态 | 复核结果 | 本轮扣分 |
| --- | --- | --- | ---: |
| UI-02 | **已解决，维持关闭** | 对照 R5 与 R6 的桌面默认、中文顶部、390px 编辑及预览原图，语言与 Edit/Preview 分段接缝没有恢复相邻双线；模式按钮焦点仍清楚可见。 | 0 |
| UI-193 | **已解决，维持关闭** | 对照 R5、R6 中文顶部原图，并检查新增的 768px、桌面深色中文帧，诊断标题中的汉字、数字与拉丁字符未见先前描述的字重分裂。 | 0 |

R5 没有未关闭差距。以下问题来自本轮新增的 Pink 起点键盘帧，不能据此认定是产品回归。

3. 差距清单

| 编号 | 单元与截图位置 | 维度 | 严重度 | 可挽回分 |
| --- | --- | --- | --- | ---: |
| **新-1** | U1；`U1-matrix-keyboard-03-768-light.png`、`U1-matrix-keyboard-03-390-light.png`、`U1-matrix2-keyboard-03-1440-dark.png` 左侧 **Brand color** 展开菜单 | 交互与状态 | **P1** | **0.4** |

**现状 → 期望：**三张图的触发器均显示 Pink，展开菜单却停留在 Custom 至 Orange 的片段，片段内没有任何可见的活动行高亮。操作记录说明这些流程使用了真实键盘展开及 ArrowDown。相比之下，`U1-keyboard-03-1440-light.png` 的 Custom 起点能明确看到 Gray 的蓝色活动行。当前配置状态没有同等可辨的键盘位置反馈，不符合 **G4「新增控件键盘可用、焦点可见」**。

**具体修法：**修正 Studio 基础配置品牌色选择器的选项弹层：打开时定位当前活动项，方向键更新活动项后让它进入弹层内部的可视范围，并确保活动行具有清楚的蓝色焦点/活动样式。若实际焦点已在列表外的项目上，应滚动弹层；若活动项在视野内但样式缺失，应补齐对应状态。仅凭原图不能确定这两者中的代码原因。

**复验：**从 Graphite → Pink → Auto 状态出发，在 768px 浅色、390px 浅色及 1440px 深色分别键盘展开并下移；截图应能直接辨认当前活动选项，继续验证 Enter 确认结果。

可挽回分合计 **0.4**，与 **10 − 9.6 = 0.4** 一致。

4. 建议（不扣分）

补拍键盘 Enter 确认选项、键盘激活预览模式，以及分隔条拖拽到最小/最大宽度的操作结果。这些目前属于证据空格，不据此判定功能失败。

5. 看不出的格子

| 图片或记录 | 无法确认的内容 |
| --- | --- |
| 三张 `matrix…keyboard-03` 展开帧 | 可确认菜单可视片段没有活动行，但无法确定实际焦点在哪个元素，以及问题具体来自滚动还是状态样式；也不能核实完整 26 个品牌色选项。 |
| 其他基础配置静态帧 | 中性色、外观、面板、圆角和缩放的完整选项未展开，不能逐项核实选项全集。 |
| 各组 `keyboard-04` 与操作记录 | 展示了模式按钮焦点，未展示键盘激活后的结果；不能把鼠标点击 Dark 的成功等同于键盘激活通过。 |
| 桌面、768px 与 200% 框架帧 | 静态截图不能确认分隔条的 8px 命中范围、拖拽过程和 260–520px 限制。 |
| `U1-zoom200-export-end-720-light.png` | Copy code、Select text、Download 完整可见，但未记录点击结果及反馈。 |
| U1 全部材料 | 未展示撤销/重做执行结果、保存失败及其后导出；只确认这些入口和已拍到的保存状态文字。 |
| 390px、200% 材料 | 390px 中文预览页和中文 200% 状态未拍摄，不能补判通过或失败。 |

6. 总评

本次只评 U1，得分 **9.6/10**。工作台结构、响应式布局、中性编辑壳与配置状态表达已经贴合目标，上一轮两项修复保持关闭。主要剩余问题是 **Pink 配置下展开菜单缺少可见键盘活动位置**，需要局部修正选择器的焦点呈现或滚动定位，**不需要重做工作台布局**。

实际开图工具：`functions.exec` 调用 `tools.view_image({detail:"original"})`，逐张显示原图。共打开 **50 张：R6 45 张、R5 5 张**；未用尺寸或像素统计替代开图。

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
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-01-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-02-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-03-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-04-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-lower-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light-a2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-end-720-light.png
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
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-edit-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-keyboard-03-1440-light.png
```
