## 1. 评分表

仅评 U7，依据冻结目标、完整变更记录、采集清单和操作记录。已实际打开本轮 **104 / 104 张原图**、9 张目标图及两张 R7 Code 对照图。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U7 布局与排版 | 3.0 / 3 | 3.0 / 3 | 3.0 / 3 | 1.0 / 1 | **10.0 / 10** |

- **布局 3.0**：两视口下 Container 的宽度层级与对齐、Inset 四边贴边和最近容器 padding 边界、排版换行及缩放后的重排均符合 L2–L6，未发现目标范围内的内容丢失或遮挡，已达本维上限。
- **视觉 3.0**：Heading、Text、Code 的字级梯度和字重表现，以及 Code 四变体、Kbd classic／soft、Blockquote 竖线与文字间距均对应目标规则，未找到可定位的剩余视觉差距，已达本维上限。
- **交互与状态 3.0**：操作记录与 95%／110% 图像相符，非法属性两视口均显示属性定位、拒收值和默认回退，组件内容继续可见；已采集状态中未发现需要扣分的问题，已达本维上限。
- **细节 1.0**：在比较口径涵盖且能清晰查看的区域，未发现可独立定位的字形裁断、边框、间距或省略处理差距；未知行为和实现事实不作一致性推断。

按 U7 比较口径，未将夹具排列、示例文案、字族、深浅主题差异及已接受的参考选阶对比度计为差距。

未关闭差距可挽回分合计 **0.0**，等于 **10 − 10.0**。

## 2. 上一轮差距复核

R7 独立结论没有未关闭差距。其复核过的旧问题如下：

| 编号 | 问题 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| UI-216 | Code ghost 的 color／high-contrast 文字带强调色 | **已解决，保持关闭** | 实际对照 R7 与 R8 两视口 `U7-type-code-matrix-*`：ghost 前两项均保持中性前景；solid、soft、outline 仍保留各自色彩处理，gray 分支也保持区分，对应目标 `radix-sink-code.png` 的 ghost 行。 |

R7 的非缺陷证据边界也已复核：

| R7 证据边界 | 本轮变化 |
| --- | --- |
| 95% Layout primitives 与 Native structure 缺少清晰独立图 | 新增 `scale95-layout-geometry` 和 `scale95-native-mixed` 原生分段，两视口均可清晰查看。 |
| 95%／110% 多个排版矩阵主要位于长图，微小轮廓不清 | 新增 Text、Code、Kbd、Blockquote、Wrap、Container、Inset、Reset 分段，已结合重叠区域查看完整内容。 |
| Reset 操作结果、DOM 语义、其他断点未知 | 仍属证据边界，见第 5 节；没有据此新增产品缺陷。 |

## 3. 全部未关闭差距

**无。**

未发现需要新增临时编号的问题，也未发现需要推倒布局、替换组件或重做素材的目标差距。

## 4. 建议，不扣分

- 补充 Reset 按钮点击后的计数反馈、Reset 输入与选择控件的实际操作，以及键盘焦点帧。
- 若后续需要覆盖完整响应式契约，补采 520／768／1024／1280／1640px 断点前后的成对截图；当前两种宽度不能覆盖所有断点切换。
- 若后续需要证明 L2、L4 的实现事实，可另提供完整文本保留、原生标签、无包裹层及跨 scope 隔离的专门证据。

以上是覆盖范围建议，本轮不扣分。

## 5. 必需格子缺证与其他证据边界

**本轮规定的 U7 状态 × 视口 × 主题格子未发现缺图。** 默认、95%、110%、非法属性诊断均有 1280 与 390 浅色图像。

具体边界如下：

- 八张整段长图已打开，但整体缩略显示不能独立支持每个区域的逐像素判断：`U7-fixture-*`、`U7-scale110-1280-light.png`、`U7-scale95-1280-light.png`、`U7-invalid-1280-light.png`、`U7-matrix-scale110-390-light.png`、`U7-matrix-scale95-390-light.png`、`U7-matrix-invalid-390-light.png`。相应区域已结合独立元素图及本轮原生分段判断，没有从长图缩略图外推微小轮廓。
- `U7-zoomdetail-scale95-layout-container-matrix-part02-1280-light-a2.png` 和 `U7-zoomdetail-scale110-type-code-matrix-part02-1280-light-a2.png` 是极薄的末尾分段，本身看不出新的组件内容；前段已包含完整可见内容，不能把这些尾段当作独立缺证格子。
- 其他原生分段的切口已与相邻重叠分段合看，包括移动端 Text、Code、Kbd、Wrap、Container、Inset、Reset、Layout geometry 和 Native mixed；没有将分段边缘的截断判为界面截断。
- Reset 对照图能显示指定元素外观被清除、原生兄弟元素仍保留默认外观，但不能证明实际标签语义、没有新增包裹层或所有其他 scope 均未受影响；图中的说明文字不能替代这些实现证据。
- Wrap 图能显示换行、nowrap 和省略号，但不能证明被省略的完整文本仍保留在 DOM 中。
- Reset 按钮图中均为 `clicks: 0`，操作日志没有对应点击；点击结果未知。输入、选择控件的实际编辑行为同样未采。
- 本轮没有 U7 专门的键盘走查、可见焦点、悬停或按下态证据；采集清单中的焦点标识不能替代相应状态图。
- 非法属性证据覆盖本次 `Box · width` 样例：拒收 `12px;display:none` 并显示 `fallback width=auto`。不能据此推广到所有属性的非法输入处理。
- 95%／110% 图像支持已展示尺度的相对变化和显式尺寸边界表现，但静态图片不能证明内部只执行一次缩放计算。
- 仅查看了提供的截图与记录，没有独立运行浏览器或核对 console、失败请求和构建过程；不对这些未提供的运行信息作结论。

## 6. 总评

本次仅评 U7，得 **10.0 / 10**。UI-216 保持关闭，新增原生分段补足了上一轮多处缩放长图的清晰度边界，未发现新的目标差距或需要结构性改变的区域。

分数限于本轮可观察且属于冻结比较口径的范围；未操作行为、DOM 实现事实和其他断点保持未知。本评审不提供验收判定。

## 实际开图工具与清单

工具：**`tools.view_image`**，经 `functions.exec` 调用并输出图像。整段长图使用 `high`，目标图、元素图和新增原生分段使用 `original`。未用文件名或尺寸统计代替开图。

共实际打开 **115 张**：本轮 104 张、目标图 9 张、R7 对照图 2 张。

以下每个文件名均与所属的绝对目录前缀连接，构成实际打开的绝对路径。

### 本轮原图：104 张

绝对目录前缀：

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
U7-zoomdetail-scale95-type-text-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-type-code-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-type-kbd-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-type-blockquote-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-type-wrap-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-layout-container-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale95-layout-container-matrix-part02-1280-light-a2.png
U7-zoomdetail-scale95-layout-inset-demonstration-part01-1280-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part01-1280-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part02-1280-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part03-1280-light-a2.png
U7-zoomdetail-scale95-layout-geometry-part01-1280-light-a2.png
U7-zoomdetail-scale95-layout-geometry-part02-1280-light-a2.png
U7-zoomdetail-scale95-native-mixed-part01-1280-light-a2.png
U7-zoomdetail-scale110-type-text-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-type-code-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-type-code-matrix-part02-1280-light-a2.png
U7-zoomdetail-scale110-type-kbd-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-type-blockquote-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-type-wrap-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-layout-container-matrix-part01-1280-light-a2.png
U7-zoomdetail-scale110-layout-container-matrix-part02-1280-light-a2.png
U7-zoomdetail-scale110-layout-inset-demonstration-part01-1280-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part01-1280-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part02-1280-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part03-1280-light-a2.png
U7-zoomdetail-scale95-type-text-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-type-text-matrix-part02-390-light-a2.png
U7-zoomdetail-scale95-type-code-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-type-code-matrix-part02-390-light-a2.png
U7-zoomdetail-scale95-type-kbd-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-type-blockquote-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-type-wrap-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-type-wrap-matrix-part02-390-light-a2.png
U7-zoomdetail-scale95-layout-container-matrix-part01-390-light-a2.png
U7-zoomdetail-scale95-layout-container-matrix-part02-390-light-a2.png
U7-zoomdetail-scale95-layout-inset-demonstration-part01-390-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part01-390-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part02-390-light-a2.png
U7-zoomdetail-scale95-layout-reset-comparison-part03-390-light-a2.png
U7-zoomdetail-scale95-layout-geometry-part01-390-light-a2.png
U7-zoomdetail-scale95-layout-geometry-part02-390-light-a2.png
U7-zoomdetail-scale95-layout-geometry-part03-390-light-a2.png
U7-zoomdetail-scale95-native-mixed-part01-390-light-a2.png
U7-zoomdetail-scale95-native-mixed-part02-390-light-a2.png
U7-zoomdetail-scale110-type-text-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-type-text-matrix-part02-390-light-a2.png
U7-zoomdetail-scale110-type-code-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-type-code-matrix-part02-390-light-a2.png
U7-zoomdetail-scale110-type-code-matrix-part03-390-light-a2.png
U7-zoomdetail-scale110-type-kbd-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-type-kbd-matrix-part02-390-light-a2.png
U7-zoomdetail-scale110-type-blockquote-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-type-wrap-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-type-wrap-matrix-part02-390-light-a2.png
U7-zoomdetail-scale110-layout-container-matrix-part01-390-light-a2.png
U7-zoomdetail-scale110-layout-container-matrix-part02-390-light-a2.png
U7-zoomdetail-scale110-layout-inset-demonstration-part01-390-light-a2.png
U7-zoomdetail-scale110-layout-inset-demonstration-part02-390-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part01-390-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part02-390-light-a2.png
U7-zoomdetail-scale110-layout-reset-comparison-part03-390-light-a2.png
```

### 目标图：9 张

绝对目录前缀：

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

### R7 对照图：2 张

绝对目录前缀：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/
```

```text
U7-type-code-matrix-1280-light.png
U7-type-code-matrix-390-light.png
```
