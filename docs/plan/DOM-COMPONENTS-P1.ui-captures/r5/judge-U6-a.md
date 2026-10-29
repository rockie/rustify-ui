## 评分表

仅评 U6；分数只覆盖已拍到的状态。夹具排列、示例文字、品牌色色相及字体字族按目标包口径排除。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | **3.0** | **2.99** | **3.0** | **1.0** | **9.99 / 10** |

- **布局 3.0**：已见四档按钮、IconButton、110% 缩放、full 圆角及 390px 换行保持正确几何，没有发现属于本单元比较范围的溢出、遮挡或尺寸偏差，达到该维度上限。
- **视觉 2.99**：变体的填充、描边、文字处理与深浅色关系符合目标；尚未满分的原因是深色 classic 最后一行的压暗仍略弱于参考，见 UI-210。
- **交互与状态 3.0**：已拍焦点的线宽、内外偏移和颜色关系符合 B2；Link 聚焦隐藏下划线，Unavailable 保持可聚焦，loading 保持按钮宽度并显示 spinner；已见状态没有新增可扣分差距。
- **细节 1.0**：在排除项之外，已见文案、ThemePanel 中英文状态、控件对齐及截断处未发现新的细节差异；UI-210 沿用上一轮归入视觉，避免重复扣分。

## 上一轮差距复核表

| 编号 | R4 差距 | 本轮状态 | 复核结果 |
| --- | --- | --- | --- |
| UI-210 | 深色 classic 底边暗线变浅 | **部分解决** | 四档底边均已加深，压暗幅度由约 19.81% 提升至约 26.00%；目标四档约为 27.76%–28.26%。旧问题的大部分已消除，尚余约 2 个百分点的单行明暗差。 |

复量使用原始 PNG 的 RGB 值，按同一计算式 `Y = 0.2126R + 0.7152G + 0.0722B` 比较末行与其上一行，避免直接比较被豁免的蓝色／紫色色相。此处 Y 是一致使用的像素明度指标，不是 WCAG 对比度计算。

## 差距清单

### UI-210 · 深色 classic 最后一行仍略浅

- **单元与截图位置**：U6；`r5/U6-fixture-1280-dark.png` 的 classic 1–4 底边。四档取样位置分别为 `(179,172)`、`(280,176)`、`(400,180)`、`(540,184)`，坐标相对于 PNG 左上角。
- **维度／严重度／可挽回分**：视觉；**P3 打磨；0.01 分**。
- **现状 → 期望**：
  - R5 四档倒数第二行均为 `#4F70E0`，最后一行为 `#3B53A3`，末行 Y 约为上一行的 **0.740**。
  - 参考 `radix-sink-button.png` classic 行四档，末行 Y 约为上一行的 **0.717–0.722**。例如 size 4 的 `(455,232)` 为 `#7C67D4`，`(455,233)` 为 `#584B90`。
  - 期望保留当前已经接近目标的上缘高光、侧边亮线及轮廓，只使底部最后一行再略深一点；不要求把本组件蓝色改成参考紫色。
- **具体修法**：微调 **Button 的 classic 深色底部 inset shadow** 的 alpha 或叠层顺序，使平直底边末行 Y 从上一行的约 0.74 接近约 0.72；让 size 1–4 共享这项修正。限定修改底部暗线，保留当前顶部和侧边效果。
- **验收**：重新拍深色 classic 四档，在各档平直底边的中心位置比对最后两行，并复看上缘及左右边没有随之加深。
- **旧差距状态**：部分解决，沿用 UI-210，不另建编号。

**扣分核对：**未关闭差距可挽回分合计 **0.01**，等于 `10 − 9.99`。

## 建议（不扣分）

没有额外的产品设计建议。本轮观察到的其他差异属于目标包排除项，或缺少状态证据，不转成修复要求。

## 看不出的格子

以下内容没有充分的截图／操作结果证据，既不计通过，也不计失败：

| 图片或记录 | 看不出的内容 |
| --- | --- |
| `U6-loading-1280-light.png`、`U6-actions.log` | 能看见 spinner 和保留的宽度；看不出 loading 时的可访问名称是否保留、spinner 是否为装饰内容，以及点击是否确实被阻止。 |
| `U6-focus-unavailable-1280-light.png` | 能确认 Unavailable 可获得键盘焦点；没有激活后的说明性反馈帧。 |
| `U6-focus-icon-1280-light-dpr2.png` | 能确认已见 IconButton 的正方形外观与焦点；PNG 无法证明其可访问名称。 |
| `U6-links-*.png`、`U6-focus-link-1280-light-dpr2.png` | 能确认已见链接的普通、悬停、焦点、截断及窄屏外观；未展示 Link 的 size 1–9 完整梯度、全部 underline／weight／trim／wrap／high-contrast 组合，也看不出 disabled 是否移除了 href。 |
| `U6-fixture-1280-light.png`、`U6-fixture-1280-dark.png`、`U6-focus-icon-1280-light-dpr2.png` | IconButton 未展示与参考图同样完整的四档 × 变体 × disabled 矩阵；不能将已见单个样本外推到所有组合。 |
| `U6-panel-zh-1280-light.png`、默认 ThemePanel 帧 | 已见的是无覆盖状态；没有本单元中产生 Custom／overridden 后的帧，也没有展开选项列表，不能确认完整选项数量及覆盖状态变化。 |
| 全部静态 PNG | 不能判断 spinner 动画、按下过程、真实点击结果及读屏输出。 |

实际逐张打开的原图如下。下面 **28 个同名文件在 `r5/` 与白名单中的 `r4/` 都已分别打开**，没有用哈希替代视觉检查：

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
```

另已逐张打开全部四张 U6 目标图：

```text
radix-sink-button.png
radix-sink-icon-button.png
radix-classic-button.png
radix-sink-link.png
```

为检查 UI-210，另外打开了仅写入 `/tmp` 的 R4、R5、目标 classic size 4 原像素放大裁切；这些裁切未作为产品截图，也未覆盖原始证据。

## 总评

本次只评 U6，得分 **9.99**。已见样本没有需要重排布局、替换组件或重做素材的问题。唯一未关闭项是 UI-210 的极小底边明暗差，继续限定在 classic 深色底部阴影内微调即可；未拍到的组合与行为仍保留为未知。
