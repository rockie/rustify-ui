**1. 评分**

仅评 U6。已阅读冻结目标的完整变更记录，并按 U6 比较口径排除夹具排列、说明文字、测试控制按钮、品牌色色相和字体字族；遵循 UI-100 的接受范围及 B2 的 Radix 焦点规则。以下分数针对本轮可辨画面，未展示项不算通过，也不据此推断缺陷。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | **3.0**：已见按钮四档高度、内边距梯度、ghost 与 ghost-offset 几何、IconButton 正方形和 full 圆角符合目标；110% 与 390px 图中未发现范围内的遮挡或溢出，达到上限。 | **3.0**：classic 的明暗边缘、solid 填充、soft 底色、surface 与 outline 的区别，以及已见 high-contrast、disabled 和深浅色关系符合目标，未发现可支持扣分的外观偏差，达到上限。 | **3.0**：已拍焦点的线宽与内外偏移符合 B2，组件选色焦点可辨，Link 聚焦隐藏下划线；loading 保宽且显示 spinner，Unavailable 的键盘焦点可见，已见状态达到上限。 | **1.0**：逐图核对已见组件的图标居中、边缘、文字截断及中英文面板换行，没有发现比较范围内的细微差异，达到上限。 | **10.0 / 10** |

**2. 上一轮差距复核**

| 上一轮条目 | 本轮结论 |
| --- | --- |
| R6 提供的“上一轮未关闭差距”为“无”，没有可逐条沿用的旧编号 | 本轮未发现新的、可由目标与原图直接支持的可见差距。 |

**3. 未关闭差距**

无。

可挽回分合计 **0.0**，与 `10 − 10.0 = 0.0` 一致。

**4. 建议（不扣分）**

仅有取证建议：若需要证明目标附件中所有组件子矩阵均已还原，应补拍下面列出的未展示组合。当前没有依据要求改动产品布局、组件或素材。

**5. 看不出的格子与证据边界**

冻结表所列 U6 七种主状态，在 **1280×900 light、1280×900 dark、390×844 light** 下均有可辨画面，没有整格缺图或因模糊而完全无法判断的情况。

但以下格内内容不能由现有截图确认：

| 对应目标矩阵格 | 对应原图 | 看不出的内容 |
| --- | --- | --- |
| U6 × 加载中 × 1280 light / 1280 dark / 390 light | `U6-loading-1280-light.png`、`U6-matrix-loading-1280-dark.png`、`U6-matrix-loading-390-light.png` | 能看见 spinner、保留的宽度和 disabled 外观；截图与日志没有实际点击结果，无法确认 loading 阻止点击、disabled 优先级及原可访问名称是否保留。 |
| U6 × 键盘焦点帧 × 三种必需视口/主题 | `U6-focus-unavailable-1280-light.png`、`U6-matrix-focus-unavailable-1280-dark.png`、`U6-matrix-focus-unavailable-390-light.png` | Unavailable 可被键盘聚焦且焦点清晰；没有触发后的记录，不能确认说明性请求的处理结果。 |
| U6 × 夹具默认/深色 × 三种必需视口/主题，目标 IconButton 外观子矩阵 | `U6-fixture-1280-light.png`、`U6-fixture-1280-dark.png`、`U6-fixture-390-light.png`及对应 matrix 图 | 已见单个 solid IconButton 和其焦点、full 圆角；未展示 IconButton 四档尺寸及 classic/soft/surface/outline/ghost-offset/ghost、gray/high-contrast/disabled 的完整组合，不能逐格判定这些组合。 |
| U6 × 夹具默认/深色 × 三种必需视口/主题，目标 Button 附加列 | 同上 | 七种 Button 变体的 size 1–4 已清楚展示；独立色族、solid High contrast 和 disabled 样例可见，但目标图中每种变体分别对应的 gray、gray high-contrast、high-contrast、disabled 并未完整展开。 |
| U6 × 夹具默认/深色 × 三种必需视口/主题，目标 Link 外观子矩阵 | `U6-links-1280-light.png`、`U6-links-390-light.png`、`U6-matrix2-link-studio-1280-dark.png`及其 hover/focus 图 | 普通、截断、disabled 链接以及已拍 hover/focus 可见；未展示 Link size 1–9 与 underline auto/always/hover/none 的完整组合，不能据一个链接确认整套字级和下划线选项。 |
| U6 × ThemePanel 中文 × 三种必需视口/主题，C3 非零覆盖状态 | `U6-panel-zh-1280-light.png`、`U6-matrix-panel-zh-1280-dark.png`、`U6-matrix-panel-zh-390-light.png` | 中英文标签、当前值、无覆盖标识和范围说明清楚；画面均为零覆盖，不能确认 Custom/overridden 及非零覆盖数量的实际呈现。 |

以上是“未见”，不是差距，也没有算作通过。

主状态矩阵之外的证据边界：没有按下态或连续交互视频，不能判断按下反馈及时间变化；没有可访问性树或导航执行结果，不能由像素确认 IconButton 名称、disabled Link 移除 `href` 等语义。未对这些内容作通过或失败结论。

**6. 总评**

本次只评 U6，得分 **10.0/10**。本轮可见组件外观和焦点状态与冻结目标一致，没有需要结构性改变的证据。此分数不代表上述未展示组件组合或语义行为已经验收通过。

**实际开图记录**

工具：`tools.view_image`，经 `functions.exec` 调度，逐张以 `detail: "original"` 打开；共 **4 张目标图、50 张 R7 原图**。

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/`

```text
radix-sink-button.png
radix-sink-icon-button.png
radix-classic-button.png
radix-sink-link.png
```

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

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
