U6 本轮独立复核得分 **10 / 10**。上轮唯一未关闭项 UI-210 已解决；本轮 50 张当前原图未发现新的可扣分差距。未展示的行为与组合仍保留为未知。

**1. 评分表**

仅评 U6。已读取冻结目标包的完整变更记录；UI-115 的接受范围属于 U2，没有将其扩为 U6 豁免。按 U6 比较口径排除夹具排列、说明文字、控制按钮、品牌色色相与字体字族，并遵循 UI-100 的参考选阶对比度接受及 B2 的 Radix 焦点规则。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | **3.0**：已见四档按钮、正方形 IconButton、110% 缩放及 full 圆角几何符合目标，390px 下控件与面板文字可换行，无范围内的遮挡或溢出，达到上限。 | **3.0**：已见变体填充、描边、文字、深浅色关系与目标一致；classic 底边剩余明暗差已关闭，没有可支持进一步扣分的偏差，达到上限。 | **3.0**：已拍键盘焦点符合 B2 的线宽与内外偏移；Link 聚焦隐藏下划线，深色 Link 普通／悬停／焦点及窄屏状态可辨；loading 保宽、Unavailable 可聚焦，已见状态达到上限。 | **1.0**：在比较范围内逐图核对，没有发现新的图标对齐、截断、间距或中英文面板细节差异，达到上限。 | **10.0 / 10** |

**2. 上一轮差距复核表**

| 编号 | 上轮差距 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| UI-210 | 深色 classic 最后一行仍略浅 | **已解决** | 已实际打开 R5、R6 深色夹具与目标图。R6 classic 四档平直底边最后一行已加深，顶部高光和侧边轮廓未见异常；补采深色夹具显示相同结果。 |

为核对这个单行差异，在开图后补充读取上述白名单 PNG 的像素：

- R6 四档取样点为 `(179,172)`、`(280,176)`、`(400,180)`、`(540,184)`，坐标相对于原图左上角。
- R5 各档上一行为 `#4F70E0`，末行为 `#3B53A3`，末行／上一行明度比为 **0.740012**。
- R6 首采及补采各档上一行仍为 `#4F70E0`，末行变为 `#3951A0`，比例为 **0.721685**。
- 目标 `radix-sink-button.png` 四档相应底边比例为 **0.717441–0.722372**。R6 已进入目标范围，无需再调整底边。

这里沿用上轮的像素明度指标 `Y = 0.2126R + 0.7152G + 0.0722B`，用于比较相邻两行的压暗关系，不是 WCAG 对比度，也未据此要求改变已排除比较的品牌色色相。

**3. 差距清单**

无仍未关闭的可扣分差距；无新增编号。

扣分核对：未关闭差距的可挽回分合计 **0**，等于 `10 − 10`。

**4. 建议（不扣分）**

没有额外的产品设计修改建议。未展示内容列入下表，不将其转为缺陷或修复要求。

**5. 看不出的格子**

| 图片或记录 | 已能确认 | 仍看不出的内容 |
| --- | --- | --- |
| `U6-loading-1280-light.png`、`U6-matrix-loading-390-light.png`、`U6-matrix-loading-1280-dark.png` 与操作日志 | spinner 可见，按钮保持原宽度，窄屏能够换行。 | 可访问名称是否保留、spinner 是否为装饰内容、loading 点击是否被阻止，以及 disabled 与 loading 同时设置时的优先级。 |
| 三种已拍主题／视口下的 `focus-unavailable` 帧 | Unavailable 保持可获得键盘焦点，没有表现成不可聚焦的原生 disabled。 | 激活后的说明性请求与反馈；日志没有记录激活结果。 |
| `U6-focus-icon-1280-light-dpr2.png` 及夹具默认／full 帧 | 已见 IconButton 的正方形、圆角与焦点外观。 | 可访问名称，以及目标图中完整四档 × 各变体 × high-contrast／gray／disabled 的组合外观。 |
| 全部 Link 帧，包括补采的 390px 和深色 Studio Link | 已见普通、悬停、焦点、截断和窄屏外观；深色 Link 悬停显示下划线、聚焦隐藏下划线。 | size 1–9 完整梯度、全部 underline／weight／trim／wrap／high-contrast 组合；disabled 是否移除了 href；激活后实际导航结果。 |
| 默认及中文 ThemePanel 帧 | 当前选中配置、已解析 Auto 中性色、无覆盖标识、浅深色计数及中英文说明可见。 | 本单元产生 Custom／overridden 后的状态；选择器未展开，不能确认完整选项数量；没有配置请求被拒绝后的结果帧。 |
| 全部静态 PNG 与当前日志 | 可确认已记录操作到达的静态外观。 | spinner 动画、完整按下过程、未记录的真实点击结果及读屏输出。 |

上述未知项不算通过，也不算失败。本轮补采已补足原先缺少的窄屏主要状态与 1280px 深色 Link 普通／悬停／焦点，不再把这些组合列作缺图。

**6. 总评**

本次只评 U6，不评价其他单元。U6 已见样本达到目标；UI-210 的底边微差已经关闭，没有需要重排布局、替换组件或重做素材的地方。该结论限于本轮图像和操作记录能够证明的范围。

实际开图工具为 **`tools.view_image`（`detail: "original"`，通过 `functions.exec` 调用并输出图像）**。共实际打开 **55 张**：本轮 50 张、目标 4 张、上轮对照 1 张。未写入文件，未读取白名单之外的代码、报告或机器检查结果。

实际打开的图片绝对路径清单：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-icon-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-classic-button.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-link.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U6-fixture-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-loading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-02-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-unavailable-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-scale110-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-radius-full-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-panel-zh-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-hover-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-focus-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-links-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-classic-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-solid-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-soft-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-surface-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-outline-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-ghost-offset-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-ghost-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-color-red-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-high-contrast-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-icon-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-solid-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-soft-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-outline-1280-dark-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-focus-link-1280-light-dpr2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-loading-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-focus-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-focus-unavailable-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-scale110-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-radius-full-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-panel-zh-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-fixture-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-loading-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-focus-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-focus-unavailable-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-scale110-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-radius-full-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-panel-zh-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-link-studio-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-link-studio-hover-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix-link-studio-focus-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix2-links-hover-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix2-links-focus-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix2-link-studio-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix2-link-studio-hover-1280-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U6-matrix2-link-studio-focus-1280-dark.png
```
