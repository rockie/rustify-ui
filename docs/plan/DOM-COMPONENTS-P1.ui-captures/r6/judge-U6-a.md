**1. 评分表**

仅评 U6，分数覆盖本轮已拍到的状态。夹具排列、示例文字、测试控制按钮、品牌色色相及字体字族按冻结目标包排除。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | **3.0** | **3.0** | **3.0** | **1.0** | **10.0 / 10** |

- **布局 3.0**：已见四档 Button、IconButton、110% 缩放、full 圆角及 390px 换行保持目标要求的组件几何，未发现比较范围内的溢出、遮挡或尺寸偏差，达到本维度上限。
- **视觉 3.0**：已见变体的填充、描边、文字处理及深浅色关系符合目标；UI-210 的深色 classic 底边明暗差已收敛到参考范围，未发现仍需扣分的视觉差距。
- **交互与状态 3.0**：已见焦点的线宽与内外偏移符合 B2，设色按钮呈现相应色族焦点，Link 聚焦隐藏下划线，Unavailable 可获得键盘焦点，loading 显示 spinner 并保持宽度；达到已拍状态的评分上限。
- **细节 1.0**：在比较口径排除项之外，逐张查看未发现可确认的控件对齐、截断、中文面板或其他细节差异；未知行为单独列出，不外推为通过。

**2. 上一轮差距复核表**

| 编号 | R5 差距 | 本轮状态 | 复核结果 |
| --- | --- | --- | --- |
| UI-210 | 深色 classic 最后一行仍略浅 | **已解决** | 实际打开 R6、R5 深色原图及目标图后复核：R6 四档 classic 平直底边的压暗已接近目标，上缘、侧边及轮廓未看到随之变差。补充原始 PNG 取样也表明，末行与上一行的明度比由 R5 约 **0.7400** 降至 R6 约 **0.7217**，落入目标四档约 **0.7174–0.7224** 的范围。 |

补充取样沿用上一轮的计算式 `Y = 0.2126R + 0.7152G + 0.0722B`，仅比较底边相对压暗，避开已排除的品牌色色相差异；它不是 WCAG 对比度计算。

- R6 四档取样：末行坐标分别为 `(179,172)`、`(280,176)`、`(400,180)`、`(540,184)`；上一行为 `#4F70E0`，末行为 `#3951A0`，四档比值均约 **0.721685**。
- 同坐标 R5 末行为 `#3B53A3`，比值约 **0.740012**。
- 目标 `radix-sink-button.png` 的对应末行坐标为 `(131,209)`、`(225,217)`、`(335,225)`、`(455,233)`，比值约 **0.722372 / 0.717441 / 0.717441 / 0.717441**。

**3. 差距清单**

本轮没有确认仍未关闭的差距，也没有新增差距。

未关闭差距可挽回分合计 **0.00**，等于 `10 − 10.00`。

**4. 建议（不扣分）**

没有额外的产品设计建议。未拍到的组合与行为属于证据边界，不转成推测性的缺陷或修改要求。

**5. 看不出的格子**

| 图片或记录 | 看不出的内容 |
| --- | --- |
| `U6-loading-1280-light.png`、`U6-actions.log` | 可以确认 spinner 与保留的按钮宽度；无法判断加载时是否保留可访问名称、spinner 是否为装饰内容，以及点击是否确实被阻止。 |
| `U6-focus-unavailable-1280-light.png`、操作记录 | 可以确认 Unavailable 获得键盘焦点；没有激活后的说明性反馈帧。 |
| `U6-focus-icon-1280-light-dpr2.png` | 可以确认已见 IconButton 的正方形外观及焦点；PNG 无法证明其可访问名称。 |
| `U6-links-1280-light.png`、`U6-links-hover-1280-light.png`、`U6-links-focus-1280-light.png`、`U6-links-390-light.png`、`U6-focus-link-1280-light-dpr2.png` | 可以确认已见链接的普通、悬停、焦点、截断及窄屏外观；未展示 Link size 1–9 完整梯度、全部 underline／weight／trim／wrap／high-contrast 组合及深色状态，也无法判断 disabled 是否移除了 href。 |
| 默认深浅色夹具及 IconButton 焦点特写 | 未展示与目标图同样完整的 IconButton 四档 × 变体 × disabled 矩阵，不能把单个样本外推到全部组合。 |
| 默认 ThemePanel 帧、`U6-panel-zh-1280-light.png` | 已见无覆盖状态；没有产生 Custom／overridden 后的帧，也没有展开选项列表，不能确认完整选项数量及覆盖状态变化。 |
| 全部静态 PNG、当前操作记录 | 无法判断 spinner 动画、完整按下过程、普通业务按钮的激活结果、读屏输出及未记录的键盘路径。 |

**6. 总评**

本次只评 U6，得分 **10.0 / 10**。上一轮唯一未关闭项 UI-210 已解决；本轮已见样本没有需要重排布局、替换组件或重做素材的问题。这个分数不代表未拍组合和未记录行为已经得到验证。

**实际开图工具及路径**

实际使用 **`view_image`（`detail: original`）** 打开以下 **33 张原图**：本轮 28 张、上一轮对照 1 张、目标 4 张。另对已打开的三张原始 PNG 作只读像素取样；没有生成、修改或覆盖图片。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-02-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-classic-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-color-red-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-ghost-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-ghost-offset-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-high-contrast-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-icon-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-link-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-outline-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-outline-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-soft-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-soft-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-solid-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-solid-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-surface-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-unavailable-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-focus-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-hover-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-loading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-panel-zh-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-radius-full-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-scale110-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U6-fixture-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-icon-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-classic-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-link.png
```
