## 1. 评分表

仅评 U7。已读冻结目标的完整变更记录，实际打开本轮全部 42 张原图、9 张 U7 目标图及 2 张 R7 对照图。

评分遵守 U7 口径：不比较页面排列、示例文案与图片、深浅色和字族；不因已接受的参考选阶或旧 token 对比度扣分。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U7 布局与排版 | 3.0 / 3 | 3.0 / 3 | 3.0 / 3 | 1.0 / 1 | **10.0 / 10** |

- **布局 3.0**：清晰分段图中的 Container 四档宽度、三种对齐、窄屏收缩，Inset 各方向贴边及最近容器 padding 消除，均未见偏离 L3–L5 的可定位问题，达到本维度上限。
- **视觉 3.0**：Heading/Text/Code 的字级梯度、Heading 与 Strong 的粗体、Text 四档字重、Code 四种变体、Kbd 两种外观及 Blockquote 竖线与间距，对应目标规则，未找到应单独扣分的视觉差距，达到本维度上限。
- **交互与状态 3.0**：操作记录与 110%／95% 图像变化相符；两视口非法属性特写均明确显示组件、属性、拒收值和 `fallback width=auto`，对应内容保持可见，已采状态达到本维度上限。
- **细节 1.0**：在清晰可比且属于目标比较口径的区域，没有发现可引用目标说明的细小差异；下列未知项没有被外推为一致。

未关闭产品差距的可挽回分合计 **0.0**，等于 **10 − 10.0**。分数仅覆盖可判断部分，不代表未知格子已获得结论。

## 2. 上一轮差距复核表

| 编号 | 上轮所列差距 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| UI-216 | Code ghost 的 color／high-contrast 文字带强调色 | **已解决，本轮未回归** | 已对照 R7、R8 两视口的 `U7-type-code-matrix-*.png`。R8 ghost 的 color 与 high-contrast 均保持中性前景，gray 分支保留灰阶差异；对应目标 `radix-sink-code.png` 的 ghost 行处理。 |

上一轮结论没有未关闭差距。本轮没有重开其他旧编号。

## 3. 未关闭差距清单

**无可由当前截图和记录确认的未关闭产品差距。**

没有发现需要重做布局、替换组件或重做素材的区域。证据无法判断的事项列在第 5 节，不将其转换成推测性产品缺陷。

## 4. 建议，不扣分

- 可在冻结矩阵之外增加 Reset 按钮点击、输入与选择控件操作，以及键盘焦点帧，补充实际行为证据。
- 可增加 520／768／1024／1640px 附近的断点前后图，检查 L5 的其余断点；本轮冻结视口只有 1280 与 390，不能据此推断其他断点。
- 390px、110% 时，示例长词出现字内换行，例如 Native structure 区的 `composition` 与 Heading 的 `Typographic`。当前文字仍完整可见，且没有对应窄屏目标要求禁止字内换行，因此不扣分；若调整演示阅读体验，可另选更适合窄屏的示例标题。

## 5. 看不出的格子与证据边界

### 冻结矩阵内仍无法充分判断的部分

| 对应原图 | 无法判断的内容 |
| --- | --- |
| `U7-scale95-1280-light.png`、`U7-matrix-scale95-390-light.png` | 整段长图已打开，但整体显示不足以支持其余所有区域的逐像素轮廓、细边框及微小间距判断；95% 的独立 Heading 图和移动端 Kbd 图可以清晰核对，不能向其他区域外推。 |
| `U7-scale110-1280-light.png`、`U7-matrix-scale110-390-light.png` | 已有独立 Heading、移动端 Kbd、Layout primitives 和 mixed 补图；完整 Text／Code／Blockquote／Wrap 等矩阵的缩放微细外观仍主要落在长图中，不能声明这些部分逐像素一致。 |
| 两视口 `U7-layout-reset-comparison-*.png` 及 primitives 补图 | 可以看到 Reset 元素失去相应默认外观，旁边原生兄弟元素仍保留默认表现；静态图不能证明真实标签语义、没有新增包裹层，或其他 scope 完全未受影响。 |
| 两视口 `U7-type-wrap-matrix-*.png` 及 mixed 补图 | 可以看到省略号和不同换行结果；静态图不能证明被省略的完整文本仍保留。 |

若需要关闭前两项，应在相同状态下补充原生分段截图；后两项需要与语义及实际结构相适应的证据，不能仅靠增加整页截图得出结论。

### 矩阵外行为与正常裁切

- 本轮日志没有 Reset 按钮点击；图中 `clicks: 0` 只能说明拍摄时的初始状态。点击反馈、键盘、悬停和按下态属于本轮未采的扩展行为。
- 未拍摄的其他断点及断点边界行为保持未知，不作为本轮冻结视口缺证。
- Wrap 的 `nowrap` 示例在边界裁切，与目标 `radix-text-wrap.png` 的同类示例一致；`truncate` 另有可见省略号，不将两者混为内容丢失。
- `U7-invalid-detail-1280-light.png`、`U7-matrix-invalid-detail-390-light.png` 上下边缘的局部裁切属于可视区截图边界；对应区域另有完整分段图，不据此判为页面遮挡。
- 整段图保留的夹具根外留边不是组件内容，不将留边内的宿主片段当成 U7 溢出。

## 6. 总评

本次只评 U7，得 **10.0 / 10**，不存在跨单元最低分比较。清晰证据中的排版梯度、组件变体、Container、Inset、Reset 可见对照及非法属性反馈均未发现新增目标差距，UI-216 也未回归。

当前没有需要结构性改变的区域。尚未形成结论的是缩放长图中的细微外观和静态图片无法证明的语义、结构及文本保留事实。

## 实际开图记录

工具：**`tools.view_image`**，通过 `functions.exec` 调用，均使用 `detail: "original"` 并实际显示图像。共打开 **42 张 R8 原图、9 张目标图、2 张 R7 原图**；没有以文件列表或尺寸读取代替开图。

以下每组均为“绝对目录前缀 + 逐个文件名”。

**R8 原图目录前缀**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/
```

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
U7-supp-default-primitives-1280-light-a2.png
U7-supp-default-mixed-1280-light-a2.png
U7-supp-scale110-primitives-1280-light-a2.png
U7-supp-scale110-mixed-1280-light-a2.png
U7-supp-default-primitives-390-light-a2.png
U7-supp-default-mixed-390-light-a2.png
U7-supp-scale110-primitives-390-light-a2.png
U7-supp-scale110-mixed-390-light-a2.png
```

**目标图目录前缀**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/
```

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

**R7 对照图目录前缀**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/
```

```text
U7-type-code-matrix-1280-light.png
U7-type-code-matrix-390-light.png
```
