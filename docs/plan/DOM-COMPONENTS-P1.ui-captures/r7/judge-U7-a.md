## 1. 评分表

本次只评 U7，依据冻结目标、实际操作记录和截图中可视觉判定的范围给分。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U7 布局与排版 | 3.0 / 3 | 3.0 / 3 | 3.0 / 3 | 1.0 / 1 | **10.0 / 10** |

- **布局 3.0**：四档 Container 的宽度层级与三种对齐、Inset 四向贴边、390px 下的堆叠与换行均符合所拍目标范围，未发现遮挡或非预期截断，达到该维度上限。
- **视觉 3.0**：Heading/Text 字级梯度及字重、Code 四种外观、Kbd classic 的立体轮廓与 soft 填充、Blockquote 竖线均与目标规则对应；Code ghost 已恢复中性前景，未发现仍需扣分的可见偏差，达到该维度上限。
- **交互与状态 3.0**：110%／95% 操作记录与对应 Heading、Kbd 帧一致；非法属性帧明确显示组件、属性、拒收值及回退值，示例继续可见，已采集状态达到该维度上限。
- **细节 1.0**：逐张对照分段原图后，未发现比较口径内需要另行扣分的微小差异；未将允许的字族、明暗模式、示例文字及页面排列差异计为缺陷。

`nowrap` 示例中的裁切与目标示例一致；`truncate` 示例具有省略号，不作为内容丢失扣分。

## 2. 上一轮差距复核

| 编号 | 状态 | 本轮复核 |
| --- | --- | --- |
| UI-216：Code ghost 的 color／high-contrast 文字仍带强调色 | **已解决** | 已实际对照 R6、R7 两个视口的 Code 原图与 `radix-sink-code.png`。R6 前两列分别呈靛蓝／深靛蓝；R7 两列均为中性正文前景，ghost 仍保持无填充、无描边，gray 两列与上方其他变体也保持区分。 |

R6 摘录提到 UI-214 已关闭。本轮 Kbd classic 轮廓未见相对目标的可见回归；本次没有重新打开 UI-214。

## 3. 未关闭差距清单

**无。**

本轮未发现需要沿用或新增编号的可执行扣分项。可挽回分合计 **0.0**，等于 **10 − 10.0**。

## 4. 建议，不扣分

后续采集可为 **Layout primitives** 和 **Native structure, flexible composition 混排区域**补独立分段原图，尤其覆盖 390px／110% 状态。这两部分目前主要依赖超长整段图，细小间距和局部边界的审阅便利度低于已提供的九类分段图。此项属于证据采集建议，不是本轮产品差距。

## 5. 看不出的格子

| 图片或记录 | 无法据此判定的内容 |
| --- | --- |
| 两视口的 `fixture`、110%、95%、invalid 整段图 | 长图展示下无法独立可靠地细抠每个小字号和像素间距；已提供的 Heading/Text/Code/Kbd/Blockquote/Wrap/Container/Inset/Reset 分段原图用于复核对应区域。未给出分段特写的 Layout primitives 与混排区域，其细小几何不据长图猜测。 |
| 所有静态截图 | 不能证明实际 HTML 标签层级、Reset 是否增加包裹层、截断文本是否完整保留，也不能证明精确 CSS 数值和断点实现；这些项目未补成视觉验收通过。 |
| 110%／95% 帧 | 可以确认已拍文字与控件的可见缩放、换行结果；不能仅凭截图证明所有参考尺度均恰好缩放一次、所有显式 CSS 尺寸绝对不变。 |
| `U7-actions.log` 与 Reset 分段图 | 没有 Reset 按钮点击后的计数帧、Restore prop 操作后帧或键盘焦点走查帧；这些未采集状态既不算通过，也不算失败。 |

## 6. 总评

本次仅评 U7，其得分为 **10.0 / 10**。上一轮唯一未关闭差距 UI-216 已从两视口原图中确认消除，未发现新的可见差距。已采集范围内无需结构性改变；该分数不扩展为对未拍状态或 DOM 实现细节的结论。

## 实际开图工具与清单

工具：**`tools.view_image({ path, detail: "original" })`**，通过 `functions.exec` 调用并实际显示图像。共打开 **45 张**：本轮 34 张、目标 9 张、前轮 2 张。

以下文件名均相对于各组明确列出的绝对目录。

**本轮目录：`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`**

```text
U7-fixture-1280-light.png
U7-type-heading-matrix-1280-light.png
U7-type-text-matrix-1280-light.png
U7-type-code-matrix-1280-light.png
U7-type-kbd-matrix-1280-light.png
U7-type-blockquote-matrix-1280-light.png
U7-type-wrap-matrix-1280-light.png
U7-layout-container-matrix-1280-light.png
U7-layout-inset-demonstration-1280-light.png
U7-layout-reset-comparison-1280-light.png
U7-scale110-1280-light.png
U7-scale110-heading-1280-light.png
U7-scale95-1280-light.png
U7-scale95-heading-1280-light.png
U7-invalid-1280-light.png
U7-invalid-detail-1280-light.png
U7-fixture-390-light.png
U7-type-heading-matrix-390-light.png
U7-type-text-matrix-390-light.png
U7-type-code-matrix-390-light.png
U7-type-kbd-matrix-390-light.png
U7-type-blockquote-matrix-390-light.png
U7-type-wrap-matrix-390-light.png
U7-layout-container-matrix-390-light.png
U7-layout-inset-demonstration-390-light.png
U7-layout-reset-comparison-390-light.png
U7-matrix-scale110-390-light.png
U7-matrix-scale110-heading-390-light.png
U7-matrix-scale110-kbd-390-light.png
U7-matrix-scale95-390-light.png
U7-matrix-scale95-heading-390-light.png
U7-matrix-scale95-kbd-390-light.png
U7-matrix-invalid-390-light.png
U7-matrix-invalid-detail-390-light.png
```

**目标目录：`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/`**

```text
radix-sink-heading.png
radix-sink-text.png
radix-sink-code.png
radix-sink-kbd.png
radix-sink-blockquote.png
radix-text-wrap.png
radix-sink-container.png
radix-inset.png
radix-reset.png
```

**前轮目录：`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`**

```text
U7-type-code-matrix-1280-light.png
U7-type-code-matrix-390-light.png
```
