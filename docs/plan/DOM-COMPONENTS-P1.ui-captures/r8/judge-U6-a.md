**1. 评分**

仅评 U6。已读冻结目标及完整变更记录，按还原口径排除夹具排列、说明文字、测试控制按钮、品牌色色相与字体字族；遵循 UI-100 已接受条款及 B2 的 Radix 焦点规则。分数仅针对本轮可辨画面。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | **3.0**：已见四档按钮的高度、内边距梯度、ghost/ghost-offset 几何、IconButton 正方形及 full 圆角符合目标，390px 和 110% 帧没有范围内的遮挡、截断或溢出，达到本维上限。 | **3.0**：classic 边缘与明暗、solid 填充、soft 底色、surface/outline 区别，以及已见选色、high-contrast、disabled 和深浅色关系未发现有目标依据的偏差，达到本维上限。 | **3.0**：已拍变体焦点的实线、内外偏移与组件选色关系符合 B2，Link 聚焦后隐藏下划线，Unavailable 有可见键盘焦点，loading 帧显示 spinner 并保留对应宽度；可见状态达到本维上限。 | **1.0**：在约定比较范围内，逐图未发现图标居中、边缘、文字截断或中英文面板换行的可支持扣分的小差异，达到本维上限。 | **10.0 / 10** |

**2. 上一轮差距复核**

| 上一轮条目 | 本轮复核 |
| --- | --- |
| R7 独立结论的未关闭差距为“无”，没有待沿用编号 | 本轮逐张检查后，未发现新的、有目标依据的可见差距；没有需要逐条关闭的旧项。 |

R7 未列未关闭问题，本轮没有需要回查旧图才能判定的旧差距，因此未重复打开 R7 原图。

**3. 全部未关闭差距**

无。

可挽回分合计 **0.0**，与 `10 − 10.0 = 0.0` 一致。未展示或无法从像素确认的内容列在下方，不作为差距扣分。

**4. 建议（不扣分）**

- 若后续需要确认目标附件的完整组件矩阵，可补充 IconButton 四档尺寸与各变体、Button 各变体的 gray/high-contrast/disabled 列，以及 Link 字级和下划线选项。现有画面不足以代表这些组合。
- 若需要确认 B3、B4、B5 的行为与语义，可补充 loading/disabled 的实际触发记录、Unavailable 请求结果，以及可访问名称和 disabled Link 链接语义记录。当前没有依据要求修改产品布局或素材。

**5. 看不出的格子**

50 张本轮原图均已实际打开，没有整张缺失或模糊到无法辨识的情况。以下是格内证据边界：

| 对应状态或内容 | 原图 | 看不出的内容 |
| --- | --- | --- |
| loading：1280 light、1280 dark、390 light | `U6-loading-1280-light.png`、`U6-matrix-loading-1280-dark.png`、`U6-matrix-loading-390-light.png` | spinner 与对应保宽可见，但没有加载期间的实际点击结果或名称记录，不能确认阻止点击、disabled 优先级及原可访问名称保留。 |
| Unavailable：三种必需视口/主题 | `U6-focus-unavailable-1280-light.png`、`U6-matrix-focus-unavailable-1280-dark.png`、`U6-matrix-focus-unavailable-390-light.png` | 能确认真实键盘焦点可见；未记录触发后的说明性请求结果。 |
| Button 附加状态矩阵 | `U6-fixture-1280-light.png`、`U6-fixture-1280-dark.png`、`U6-fixture-390-light.png`及对应 matrix 图 | 七种变体的 size 1–4 清楚；独立选色、solid high-contrast 和禁用样例可见，但目标图中每种变体分别对应的 gray、gray high-contrast、high-contrast、disabled 未完整展开。 |
| IconButton 外观矩阵 | 同上及 `U6-focus-icon-1280-light-dpr2.png` | 已见单个 solid IconButton、其焦点和 full 圆角；四档尺寸及其余变体、gray/high-contrast/disabled 组合未展示。 |
| Link 外观矩阵 | `U6-links-1280-light.png`、`U6-links-390-light.png`、对应 hover/focus 图，以及 Studio 深色 Link 图 | 普通、截断、disabled 链接及已拍悬停/焦点可辨；没有 size 1–9、weight、trim、wrap、underline auto/always/hover/none 的完整对照。 |
| 110% 缩放的组件覆盖范围 | `U6-scale110-1280-light.png`、`U6-matrix-scale110-390-light.png`、`U6-matrix-scale110-1280-dark.png` | 这些帧展示按钮变体矩阵，无法据此确认 Link 与内嵌 ThemePanel 在 110% 下的完整表现。 |
| ThemePanel 的 Custom/overridden 非零状态 | `U6-panel-zh-1280-light.png`、`U6-matrix-panel-zh-390-light.png`、`U6-matrix-panel-zh-1280-dark.png`及默认面板图 | 已见中英文标签、当前值、有效 auto 结果、零覆盖标识及范围说明；没有非零覆盖画面，不能确认 Custom/overridden 和覆盖数量的实际呈现。 |
| ThemePanel 完整选项 | 同上 | 选择器未展开，只能确认当前显示值，不能逐项确认内嵌面板与 Studio 表达相同的完整选项。 |
| 按下反馈、动态过程及不可见语义 | 全部截图与 `U6-actions.log` | 没有按下态或连续过程记录；像素不能证明 IconButton 可访问名称、disabled Link 移除 `href` 等语义。 |

**6. 总评**

本次仅评 U6，得分 **10.0/10**。已见按钮、链接、焦点和面板状态未发现可由冻结目标支持的差距，没有需要结构性改变的证据。上述未展示组合和语义行为仍保持未判断。

**实际开图记录**

工具：`tools.view_image`，经 `functions.exec` 调用，均以 `detail: "original"` 实际打开。共 **4 张目标图、50 张 R8 原图**。

目标图共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/`

```text
radix-sink-button.png
radix-sink-icon-button.png
radix-classic-button.png
radix-sink-link.png
```

本轮原图共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/`

```text
U6-fixture-1280-light.png
U6-fixture-1280-dark.png
U6-loading-1280-light.png
U6-focus-1280-light.png
U6-focus-02-1280-light.png
U6-focus-unavailable-1280-light.png
U6-scale110-1280-light.png
U6-radius-full-1280-light.png
U6-panel-zh-1280-light.png
U6-fixture-390-light.png
U6-links-1280-light.png
U6-links-hover-1280-light.png
U6-links-focus-1280-light.png
U6-links-390-light.png
U6-focus-classic-1280-light-dpr2.png
U6-focus-solid-1280-light-dpr2.png
U6-focus-soft-1280-light-dpr2.png
U6-focus-surface-1280-light-dpr2.png
U6-focus-outline-1280-light-dpr2.png
U6-focus-ghost-offset-1280-light-dpr2.png
U6-focus-ghost-1280-light-dpr2.png
U6-focus-color-red-1280-light-dpr2.png
U6-focus-high-contrast-1280-light-dpr2.png
U6-focus-icon-1280-light-dpr2.png
U6-focus-solid-1280-dark-dpr2.png
U6-focus-soft-1280-dark-dpr2.png
U6-focus-outline-1280-dark-dpr2.png
U6-focus-link-1280-light-dpr2.png
U6-matrix-fixture-390-light.png
U6-matrix-loading-390-light.png
U6-matrix-focus-390-light.png
U6-matrix-focus-unavailable-390-light.png
U6-matrix-scale110-390-light.png
U6-matrix-radius-full-390-light.png
U6-matrix-panel-zh-390-light.png
U6-matrix-fixture-1280-dark.png
U6-matrix-loading-1280-dark.png
U6-matrix-focus-1280-dark.png
U6-matrix-focus-unavailable-1280-dark.png
U6-matrix-scale110-1280-dark.png
U6-matrix-radius-full-1280-dark.png
U6-matrix-panel-zh-1280-dark.png
U6-matrix-link-studio-1440-dark.png
U6-matrix-link-studio-hover-1440-dark.png
U6-matrix-link-studio-focus-1440-dark.png
U6-matrix2-links-hover-390-light.png
U6-matrix2-links-focus-390-light.png
U6-matrix2-link-studio-1280-dark.png
U6-matrix2-link-studio-hover-1280-dark.png
U6-matrix2-link-studio-focus-1280-dark.png
```
