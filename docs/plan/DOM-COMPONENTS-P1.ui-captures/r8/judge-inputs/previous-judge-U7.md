## 1. 评分表

仅评 U7。已读冻结目标及完整变更记录，并实际打开本轮全部 42 张原图。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U7 布局与排版 | 3.0 / 3 | 3.0 / 3 | 3.0 / 3 | 1.0 / 1 | **10.0 / 10** |

- **布局 3.0**：可清晰查看的 Container 四档宽度与三种对齐、Inset 四边贴边、最近容器 padding 消除及窄屏重排符合 L3–L5，未见目标范围内的遮挡或内容丢失，达到本维度上限。
- **视觉 3.0**：Heading/Text/Code 的字级梯度、字重和行高表现，Kbd 两种外观、Blockquote 竖线与间距均对应目标规则；Code ghost 的强调色偏差已消除，没有剩余的可定位视觉失分项。
- **交互与状态 3.0**：操作记录与 110%／95% 帧一致，非法属性帧清楚显示 `Box · width`、拒收值及 `fallback width=auto`，内容继续可见，达到已采状态的本维度上限。
- **细节 1.0**：在可清晰比对且属于目标比较口径的区域，未发现需要独立扣分的细节差异；不把未知格子视为已通过。

未关闭差距可挽回分合计 **0.0**，等于 **10 − 10.0**。

## 2. 上一轮差距复核

| 编号 | R6 差距 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| UI-216 | Code ghost 的 color／high-contrast 文字仍带强调色 | **已解决** | 已打开 R6 两视口 Code 原图与 R7 对应原图。R6 ghost 前两项分别呈靛蓝与深靛蓝；R7 均为中性前景，对应目标 `radix-sink-code.png` 的 ghost 前两列；其他三种变体仍保留各自的强调色处理，gray 分支也未被一并改成强调色。 |

R6 摘录仅列 UI-216 为未关闭差距。本轮没有重开其他旧编号。

## 3. 未关闭差距清单

**无。**

没有发现需要推倒布局、替换组件或重做素材的目标差距。未采行为和截图无法证明的实现事实列在下方，不据此补扣分。

## 4. 建议，不扣分

若后续要声明更完整的交互或逐像素覆盖，可补采：

- 95% 的 Layout primitives 与 Native structure 独立裁切，以及其他排版矩阵的缩放分段图。
- Reset 原生按钮的实际点击结果、输入／选择控件操作与键盘焦点帧。
- 520／768／1024／1640px 附近的断点前后帧。

这些属于覆盖范围建议，不是本轮已发现的产品缺陷。

## 5. 看不出的格子

- `U7-scale95-1280-light.png` 与 `U7-matrix-scale95-390-light.png` 已实际打开，但整段长图在整体显示下不能支持其他所有排版区的逐像素轮廓判断；独立 Heading 图及移动端 Kbd 图可清晰核对。没有把其余 95% 细节外推为一致或缺陷。
- 110% 的 Layout primitives 与 Native structure 已有独立原生裁切，可核对主要布局和文本；110% 完整 Code／Text／Blockquote 等矩阵的微小轮廓仍主要位于长图，未声明这些格子逐像素一致。
- Reset 对照图显示原生兄弟元素保留默认外观、Reset 元素外观被清除，但截图不能证明 DOM 没有新增包裹层、实际标签语义或其他 scope 完全未受影响。
- Wrap 图可见省略号和不同换行表现，但静态图片不能证明被省略的完整文本仍保留在 DOM 中。
- Reset 按钮均显示 `clicks: 0`，本轮日志没有对应点击；不能判断点击后的反馈。
- 本轮没有 U7 专门的键盘走查、焦点、悬停或按下态证据；这些行为保持未知。
- 只采到 1280 与 390 两种宽度，不能据此判定其余断点及断点边界行为。

## 6. 总评

本次仅评 U7，得 **10.0 / 10**。R6 唯一未关闭差距 UI-216 已有两视口的直接图像证据支持关闭；本轮未发现新增目标差距，也没有需要结构性改变的区域。

满分限于本轮可清晰比对的范围，不表示未操作行为、DOM 实现事实和其他断点已验收通过。

## 实际开图记录

工具：**`tools.view_image`**，通过 `functions.exec` 调用；补充原生裁切使用 `detail: "original"` 查看。本轮 **42 / 42** 张原图均实际打开，另打开目标图 9 张、R6 原图 2 张。没有使用尺寸统计代替开图。

本轮原图：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-fixture-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-heading-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-text-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-code-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-kbd-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-blockquote-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-wrap-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-container-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-inset-demonstration-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-reset-comparison-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-scale110-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-scale110-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-scale95-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-scale95-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-invalid-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-invalid-detail-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-heading-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-text-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-code-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-kbd-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-blockquote-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-type-wrap-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-container-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-inset-demonstration-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-layout-reset-comparison-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale110-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale110-heading-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale110-kbd-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale95-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale95-heading-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-scale95-kbd-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-invalid-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-matrix-invalid-detail-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-default-primitives-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-default-mixed-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-scale110-primitives-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-scale110-mixed-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-default-primitives-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-default-mixed-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-scale110-primitives-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U7-supp-scale110-mixed-390-light.png
```

目标原图：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-heading.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-text.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-code.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-kbd.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-blockquote.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-text-wrap.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-container.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-inset.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-reset.png
```

R6 对照原图：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-390-light.png
```
