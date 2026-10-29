**U2 本轮评分：10.00 / 10。** 这是对白名单可见状态的评分，不代表未拍摄或未操作的状态已通过。

### 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00** |

- **布局 3.00**：已到上限；来源块直接跟随所选 token，桌面与 390px 的来源、长路径、分组清单和字段错误均正常换行，可见范围没有横向溢出或关键文字遮挡。
- **视觉 3.00**：已到上限；编辑壳保持中性浅色，选中状态、标题层级、分体控件、错误边框和焦点环清楚，可见范围没有能由 G1–G4、F4 支持的视觉扣分项。
- **交互与状态 3.00**：已到上限；覆盖来源、单项解除后的生成来源、分组解除结果、两模式范围清单、非法 radius 和低对比诊断均有截图及操作记录；新增键盘帧中的 primary 重置按钮具有明显蓝色焦点环。
- **细节 1.00**：已到上限；本单元是规范模式，没有对应目标像素图；逐处检查所示文案、状态标识、对齐和换行，未发现有规范依据的剩余差异。

### 2. 上一轮差距复核

| 编号 | 本轮状态 | 复核依据 |
| --- | --- | --- |
| UI-194 | **已解决，保持** | 桌面 source-overridden、detail、token-released 与 390px 对应图中，来源说明紧接所选 primary，没有被其他 token 隔开。 |
| UI-172 | **已解决，保持** | 桌面与 390px 的 group-reset-tokens 首尾图明确列出 Brand、Neutral、Panel、Radius，以及各自 Light、Dark 的 token 名和 both modes 范围。 |
| UI-195 | **已解决，保持** | 两张 390px source-overridden 图中，“SEMANTIC COLORS”完整位于 Edit/Preview 下方。 |
| UI-196 | **已解决，保持** | 桌面与 390px 所选 primary 的 “Light”字重与邻近模式文字一致。 |
| UI-18 | **已解决，保持** | “Fine adjustment source”“Color controls”与各颜色分组标题层级清楚，未见旧问题回归。 |
| UI-115 | **不再适用：目标已接受** | R6 比较口径及完整变更记录明确接受当前 ↺ 图标、模式标识和来源区范围说明，不再要求按钮或列表常显范围文字；本轮不继续扣除原来的 0.05 分。 |
| UI-197 | **已解决，保持** | 桌面和 390px 诊断中，同级标题均为 “Fonts”与 “Text contrast pairs”，没有旧计数格式不一致问题。 |
| UI-160 | **已解决，保持** | search-row 桌面图及 390px 分组清单图中，搜索框与 Clear 顶底对齐。 |

目标变更记录提及 **UI-215**，但白名单中的 R5 U2 评审原文没有该条正文，不能替它补写历史描述或结案理由。就本次可直接核验的 G4 要求而言，`U2-token-reset-keyboard-focus-1440-light.png` 中 primary 的 ↺ 段有完整、清楚的蓝色键盘焦点环。

### 3. 未关闭差距

**未发现有目标依据的未关闭扣分差距。**

可挽回分合计 **0.00**，与 `10 − 10.00 = 0.00` 一致。没有新增临时编号。

### 4. 建议，不扣分

展开后的 Neutral 影响清单仍是连续长段落。可将 Light、Dark 做成更明确的小节，提升扫描速度；当前内容完整且正常换行，目标未规定必须采用特定列表结构，因此不扣分。

### 5. 看不出的格子

以下内容不计通过，也不计失败：

- **定位行为**：两种宽度的低对比预览均拍到 Locate，但操作记录没有点击 Locate，无法确认其实际定位、滚动及来源选择结果。
- **恢复行为**：`U2-restore-1440-light.png` 拍到恢复入口及“配置、两模式全部值、丢弃后续调整”的范围说明，没有点击后的恢复结果。
- **其他键盘状态**：当前真实键盘焦点证据覆盖 primary 的 ↺；没有 picker、Colors/Typography 标签页、字体选择器及其余重置入口的独立键盘帧。
- **颜色非法输入**：错误实拍为 Corner radius 的 `abc`；不能外推非法颜色字符串的边框、提示与恢复行为。
- **共享字段完整性**：controls 图仅覆盖字体区的一部分，没有完整呈现全部 14 个共享字段、字体下拉展开项及有效修改后的结果。
- **窄屏导航位置**：`U2-matrix-controls-390-light.png` 没有显示 Edit/Preview 切换条；仅凭该帧无法判断它是滚出截图还是存在其他原因，不据此推断导航失效。其他窄屏编辑帧显示切换条，低对比预览帧显示 Preview 选中态及 Back to edit。
- **运行时行为**：静态图不能独立验证滚动、拖拽、过渡、可访问名称或所有模式/分组是否完全未受影响。
- 所有已打开原图均足够清晰；上述限制来自状态或操作未覆盖。

### 6. 总评

本次只评 U2，不能作跨单元最低分排序。U2 的来源解释、单项与分组解除范围、错误位置、窄屏换行和低对比编辑壳均达到本轮可见目标；UI-115 按接受后的目标处理，重置入口的可见键盘焦点也已有实拍证据。**没有需要结构性重做的区域。**

实际开图工具：**`tools.view_image`，通过 `functions.exec` 调用并显示原图**。未用目录、尺寸或像素统计代替开图。U2 为规范模式，没有对应目标 PNG；未打开其他单元的目标图。

实际打开的图片绝对路径，共 **28 张**：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-released-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-generated-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-controls-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-low-contrast-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-restore-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-search-row-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-reset-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-token-released-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-tokens-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-tokens-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-source-generated-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-controls-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-field-error-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-low-contrast-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix3-low-contrast-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-group-reset-tokens-1440-light.png
```
