1. 评分表

仅评 U1，依据冻结目标中的 G1–G6、C1–C4。U1 是规范模式，没有用于还原比较的目标 PNG。本次已实际打开全部 25 张 R6 原图，并打开 5 张 R5 原图复核旧差距。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 | R5 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 10.0 |

- **布局 3.0**：已到维度上限。桌面编辑列与弹性预览、768px 顶栏换行、390px 编辑/预览切换符合 G1–G2；200% 下已拍摄的框架、颜色网格、来源说明和导出操作均可在各自区域内到达，没有发现需要横向移动整页才能操作的内容。
- **视觉 3.0**：已到维度上限。浅色编辑工具与作者预览保持明确边界，深色预览没有改变编辑壳；选中、焦点、分隔线及扁平字段标签页的可见表现符合 G3–G4，未发现可扣分的规范偏差。
- **交互与状态 3.0**：已到维度上限。记录与画面共同展示了预设搜索、配置变更、auto 解析结果、分模式覆盖数量、预览深浅色切换、中英文切换、手机预览入口及清晰的键盘焦点；作者外观与 Preview mode 的语义区分明确。
- **细节 1.0**：已到维度上限。已拍摄范围内未发现应扣分的接缝、对齐、换行、状态文案或多语言差异；未展示的执行结果列入“看不出的格子”，不据此补造通过或失败。

2. 上一轮差距复核表

| 编号 | 状态 | 本轮证据与结论 | 本轮扣分 |
| --- | --- | --- | ---: |
| UI-02 | **已解决，无回归** | 对照 R5 与 R6 的 `U1-default-1440-light.png`、`U1-zh-top-1440-light.png`、`U1-edit-390-light.png`、`U1-preview-390-light.png`：EN/中文及 Edit/Preview 的相邻接缝未重新出现第二条灰色边线；R6 的 `U1-keyboard-04-1440-light.png` 中 Dark 的独立焦点也保持可见。 | 0 |
| UI-193 | **已解决，无回归** | 对照 R5 与 R6 的 `U1-zh-top-1440-light.png`、`U1-zh-configured-1440-light.png`：「2 组文字对比低于 4.5:1 · 作者颜色保持不变」的汉字、数字及拉丁字符保持一致的半粗层级，没有重新出现中西文字重分裂。 | 0 |

3. 差距清单

**无仍未关闭的差距，无新增差距。**

可挽回分合计 **0.0**，与 `10 − 10.0 = 0.0` 一致。

本轮目标变化中，UI-115 的用户接受只适用于 U2 的重置范围文字，没有改变 U1 的评分依据，也没有放宽 U1 的可见键盘焦点要求。

4. 建议（不扣分）

可补充一次键盘确认品牌色、一次键盘激活 Dark，以及一次分隔条拖拽的操作证据。现有材料没有显示这些行为失败，不因此扣分。没有需要另行实施的结构或视觉建议。

5. 看不出的格子

| 图片或记录 | 无法据此确认的内容 |
| --- | --- |
| `U1-keyboard-03-1440-light.png` | 品牌色展开列表只显示当前可见部分，不能核实完整 26 项；其他基础配置选择器的完整选项也没有展开。 |
| `U1-keyboard-03-1440-light.png`、`U1-keyboard-04-1440-light.png` 与操作记录 | 展示了 Enter 展开、ArrowDown、Escape 和 Dark 焦点，未展示键盘确认选项后的结果，也未展示键盘激活 Dark 后的结果；鼠标点击切换深色已有其他画面支持。 |
| 桌面、768px 与 200% 框架帧 | 静态画面不能证明分隔条的 8px 命中范围、拖拽行为及 260–520px 限制；日志没有对应拖拽步骤。 |
| `U1-zoom200-export-end-720-light.png` | Copy code、Select text、Download 完整可见，但 U1 记录没有执行这些按钮，不能判断其执行结果或成功反馈。 |
| `U1-preview-390-light.png` | Back to edit 入口可见，但日志没有点击返回；不能把该入口的可见性写成已完成往返操作。 |
| U1 全部图片与记录 | 撤销、重做、命名保存和保存失败后的执行结果未展示；这里只确认入口与已拍摄的保存状态文案，不替代相应功能验收。 |

`U1-zoom200-source-720-light.png` 首拍不足以单独展示完整来源块，但补拍的 `U1-zoom200-source-720-light-a2.png` 与 `U1-zoom200-source-end-720-light.png` 已共同展示来源、路径、解除影响及解除按钮，因此该处不再构成证据缺口。

6. 总评

本次唯一评审单元 U1 为 **10.0/10**。已拍摄状态中，未发现与冻结规范不符的可见差距；R5 的两项历史差距保持关闭，**不需要结构性改变**。该结论限于截图和日志覆盖的状态，不表示未操作行为已通过验收。

实际使用的开图工具：**`tools.view_image`，通过 `functions.exec` 调用并显示原图。** 共实际打开 30 张图片；没有把文件列表、尺寸或其他检查当作开图。

R6 原图，25 张：

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
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-lower-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-colors-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light-a2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-01-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-02-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-03-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-04-1440-light.png
```

R5 对照原图，5 张：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-default-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-zh-top-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-zh-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-edit-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U1-preview-390-light.png
```
