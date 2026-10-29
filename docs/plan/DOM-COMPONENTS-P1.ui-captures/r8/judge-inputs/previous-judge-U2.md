## 1. 评分表

仅评 U2。依据冻结目标的 F1–F4、G1–G4 和 UI-115 接受条款；本单元为规范模式，不按其他单元的目标图片比较。

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** |

- **布局 3.00**：已达标尺上限；1440 与 390 的 token、来源说明、分组影响清单和字段错误均在可用宽度内换行，来源紧随所选 token，未见横向溢出、遮挡或影响阅读的截断。
- **视觉 3.00**：已达标尺上限；可见的中性编辑壳、蓝色选中与焦点、细分隔线、等宽颜色值和颜色项分体控件符合 G1–G4、F4，低对比作者主题未使编辑工具失去可读性。
- **交互与状态 3.00**：已达标尺上限；覆盖来源、单项解除后的生成来源、分组解除结果、两模式作用域、非法 radius 和低对比诊断均有对应截图与操作记录，重置按钮的键盘焦点清楚可见。
- **细节 1.00**：已达标尺上限；逐张检查可见文案、状态标识、路径、按钮与字段对齐，未发现有目标依据的剩余差异；颜色项右侧仅显示 ↺ 属于 UI-115 明确接受的形式。

具体可见依据：

- primary 被覆盖时，来源区显示有效值、`Explicit document override`、`overrides.light.primary`、解除后的 Light 值及“其他 token 不变”；单项解除后显示生成路径，并禁用无覆盖可解除的操作。
- 品牌分组解除后显示 `No overrides · Light 0 · Dark 0`；中性色仍显示两模式各 15 项覆盖，未见把其他分组一并清空的结果。
- 展开的分组清单分别列明 Brand、Neutral、Panel、Radius 及 Light / Dark 的 token；长名称在两档视口中均能阅读。
- `Restore document · Graphite` 下方明确说明恢复配置及两模式全部值、丢弃后续调整，作用域与单项、分组重置有明确区别。
- 非法 radius 保留 `abc` 草稿，错误紧贴字段，解释允许格式、继续使用的有效预览值及恢复方式。
- 两档低对比诊断都显示作者颜色保留、具体颜色对和比值；390 的预览侧补图显示全部五条诊断及 Locate 入口。

## 2. 上一轮差距复核

| 上轮项目 | 本轮状态 | 复核结论 |
| --- | --- | --- |
| R6 未关闭差距 | 无可续编号项目 | R6 原文列“无”。本轮独立检查未发现需要新增的规范差距。 |
| R6 已给满分的窄屏字体页、字段错误与重置焦点 | 保持 | 实际打开对应 R6 原图对照，未见这些位置发生可见退化；这不作为其他未对照图片完全一致的声明。 |

## 3. 未关闭差距清单

**无。**

可挽回分合计 **0.00**，与 `10 − 10.00 = 0.00` 一致。不因未拍摄或未执行的行为补造缺陷。

## 4. 建议（不扣分）

无必须追加的设计建议；现有证据不支持要求重做布局、替换组件或增加新的界面结构。

## 5. 看不出的格子与验证边界

**目标矩阵没有整格缺图或不可辨认的状态。** 以下为格子内部的可见范围限制：

| 目标矩阵具体格子 | 对应图片 | 看不出的内容 |
| --- | --- | --- |
| U2 · 字体与其他页 · 1440×900 · light | `U2-controls-1440-light.png`、`U2-field-error-1440-light.png` | 长页仅展示字体上段及 radius 附近，不能逐项判断界面所述全部 14 个共享字段。可见部分未发现缺陷。 |
| U2 · 字体与其他页 · 390×844 · light | `U2-matrix-controls-390-light.png`、`U2-matrix-field-error-390-light.png` | 同样不能覆盖全部 14 个共享字段；错误帧下缘只露出 Layout gap 的部分输入区。不能将这些未显示部分记为通过或失败。 |

**矩阵外、或本轮未执行的行为边界：**

- Restore document 本轮仅拍摄入口和范围说明，未执行恢复后的完整结果验证。
- 低对比诊断拍到了 Locate 入口，操作记录未包含点击后的定位结果。
- 本轮未实际验证拾色器方向键、Escape、全部字段的键盘路径、悬停态或低对比情况下的 JSON 导出。这些不计为本轮通过，也不据此扣分。
- primary 单项解除的记录与可见说明支持其 Light 作用域；未展示完整 dark token 值前后表，不能宣称逐值完成了另一模式的数据复核。

## 6. 总评

本次仅评 U2，得分 **10.00/10**。在所拍摄状态中，来源解释、三层重置范围、输入错误和低对比诊断均满足冻结目标，未发现需要结构性改变的地方。满分限定于此次可见证据，不扩展为对未拍摄字段或未执行行为的保证。

## 实际开图记录

工具：`functions.exec → tools.view_image`，均使用 `detail: "original"` 实际打开；未以文件清单或尺寸读取代替看图。

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

实际打开本轮全部 **24 张**：

```text
U2-source-overridden-1440-light.png
U2-source-overridden-detail-1440-light.png
U2-token-released-1440-light.png
U2-group-reset-1440-light.png
U2-source-generated-1440-light.png
U2-controls-1440-light.png
U2-field-error-1440-light.png
U2-low-contrast-1440-light.png
U2-source-overridden-390-light.png
U2-restore-1440-light.png
U2-search-row-1440-light.png
U2-group-reset-tokens-1440-light.png
U2-group-reset-tokens-end-1440-light.png
U2-token-reset-keyboard-focus-1440-light.png
U2-matrix-source-overridden-390-light.png
U2-matrix-token-released-390-light.png
U2-matrix-group-reset-tokens-390-light.png
U2-matrix-group-reset-tokens-end-390-light.png
U2-matrix-group-reset-390-light.png
U2-matrix-source-generated-390-light.png
U2-matrix-controls-390-light.png
U2-matrix-field-error-390-light.png
U2-matrix-low-contrast-390-light.png
U2-matrix3-low-contrast-preview-390-light.png
```

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`

实际打开上轮 **3 张**：

```text
U2-matrix-controls-390-light.png
U2-matrix-field-error-390-light.png
U2-token-reset-keyboard-focus-1440-light.png
```
