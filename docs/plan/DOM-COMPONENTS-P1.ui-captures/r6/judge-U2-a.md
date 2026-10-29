**1. 评分**

仅评 U2，依据冻结目标、白名单内截图及操作记录判断。未拍到的状态不算通过，也不算失败。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00 / 10** |

- **布局 3.00**：桌面与 390px 的所选 token、来源说明、解除动作关系清楚；来源块紧接所选项，长路径和说明保持可读，当前可见范围没有需要扣分的结构、对齐或截断问题，已达该维度上限。
- **视觉 3.00**：编辑壳保持中性浅色，标题、分组、选中和字段错误具有一致层级；错误文字与边框的原图像素为规范规定的 `#b42318`，当前可见范围没有有依据的视觉偏差，已达该维度上限。
- **交互与状态 3.00**：覆盖来源、解除后的生成来源、分组解除结果及影响清单、字段错误与保留有效预览的提示均有证据；新增键盘帧中的 ↺ 具有清晰可见的蓝色焦点，UI-115 的常显范围文字要求已由冻结目标取消，当前没有有依据的扣分项。
- **细节 1.00**：当前图片中未发现符合本单元比较口径的文案、模式标识、图标对齐或微小间距差异；本单元采用规范模式，没有独立目标 PNG 可作逐像素还原比较。

**2. 上一轮差距复核**

| 编号 | 状态 | 本轮依据 |
| --- | --- | --- |
| UI-194 | **已解决** | 两张桌面 overridden 图、390 图与 token-released 图中，来源块都紧接所选 primary，没有被 primary-foreground 隔开。 |
| UI-172 | **已解决** | group-reset-tokens 与 group-reset-tokens-end 展开 Brand、Neutral、Panel、Radius 的影响清单，明确标示 both modes，并分别列出 Light、Dark 的 token。 |
| UI-195 | **已解决** | 390 图中 “SEMANTIC COLORS”完整显示在 Edit/Preview 栏下方，没有标题上半部被遮住的问题。 |
| UI-196 | **已解决** | 桌面与 390 图中，选中 primary 的 “Light”保持常规字重，没有随 token 名一起加粗。 |
| UI-18 | **已解决** | “Fine adjustment source”“Color controls”与分组眉题层级清楚；当前未发现原问题回归。 |
| UI-115 | **不再适用：目标变更** | R6 冻结目标明确接受右端 ↺、现有模式标识及来源区范围说明，不再要求按钮或列表常显范围文字；当前来源区仍明确显示 “Clear primary · Light only”。这属于接受范围变化，不记作界面新增修复。 |
| UI-197 | **已解决** | low-contrast 图中的同级诊断摘要为 “Fonts”“Text contrast pairs”，没有一处带计数、另一处不带计数的不一致。 |
| UI-160 | **已解决** | search-row 图完整显示搜索行，搜索框与 Clear 顶底对齐、等高。 |

新增的 `U2-token-reset-keyboard-focus-1440-light.png` 显示 primary 右端 ↺ 周围有独立蓝色焦点轮廓；操作记录包含 Shift+Tab、Tab 及等待后拍摄，符合 G4 对可见键盘焦点的要求。

**3. 未关闭差距清单**

**无。**

本轮可挽回分合计 **0.00**，与 `10 − 10.00` 一致。没有发现应新增编号的差距。

**4. 建议，不扣分**

展开的 Neutral 影响清单可以进一步把 Light、Dark 拆成更易扫描的小节。当前范围与 token 已列出，冻结目标未规定必须采用这种排版，因此不扣分。

**5. 看不出的格子**

- `U2-token-reset-keyboard-focus-1440-light.png` 能确认 ↺ 的键盘焦点可见，不能确认其悬停提示内容或可访问名称。
- overridden-detail 与 390 图没有 picker 的独立键盘焦点状态，也没有非法颜色字符串的错误状态；不能据 Corner radius 的错误推定所有颜色字段都已验证。
- controls 图只显示部分共享字段，未覆盖全部 14 个字段、字体下拉展开内容及有效编辑后的结果。
- low-contrast 图能确认诊断与 Locate 入口可见，但日志没有点击 Locate，不能确认其实际定位结果。
- restore 图能确认恢复入口及“两模式配置和所有值”的作用域说明，日志没有执行恢复，不能确认恢复结果。
- 390 图只覆盖所选 token、来源区及部分拾色区，未覆盖窄屏字段错误、展开的分组影响清单与完整键盘走查。
- 单项解除与分组解除有结果帧，但没有完整展示另一模式的逐项结果，不能单凭当前截图证明所有模式、所有分组的值均保持正确。
- 静态截图不能独立证明拖拽、连续滚动或过渡行为。
- 目标变更记录提到 UI-215，但白名单内上一轮 U2 评审原文没有该条定义，因此不凭编号猜测其内容；本轮按 G4 独立核对了新增的重置按钮键盘帧。

已打开图片均足够清晰，以上限制来自状态未拍摄、操作未记录或材料未提供。

**6. 总评**

本次只评 U2，得分 **10.00 / 10**。在已采集范围内，来源解释、单项与分组解除作用域、字段错误、中性编辑壳以及新增重置按钮键盘焦点均符合 R6 冻结目标。**没有发现需要结构性重做或继续扣分的差距。** 满分仅表示本轮可见材料内没有未关闭差距，不代表上述未拍状态已验证。

实际开图工具：`functions.exec` 调用 `tools.view_image`，全部使用 `detail: "original"`。另以 Python 标准库只读核对了已打开错误截图的像素配色；没有修改文件。U2 为规范模式，无对应目标 PNG，未打开其他单元目标图。

实际打开的图片绝对路径如下，共 **16 张：本轮 14 张，上一轮 2 张**。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-released-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-generated-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-controls-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-low-contrast-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-restore-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-search-row-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-reset-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-1440-light.png
```
