**1. 评分**

仅评 U2。已读完整 R6 目标包、变更记录、采集清单、操作记录及 R5 独立评审，并实际打开本轮全部 23 张原图。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00 / 10** |

- **布局 3.00**：1440px 与 390px 均保持所选 token 后紧接来源说明；来源、分组影响清单、字体字段及字段错误在已拍范围内正常换行，没有遮挡关键内容，达到本维度上限。
- **视觉 3.00**：中性浅色编辑壳、蓝色选择和焦点、来源标题、分组眉题、颜色分体控件及错误呈现符合 G1–G4、F4；已拍范围未发现有规范依据的视觉扣分项。
- **交互与状态 3.00**：覆盖与生成来源、单项解除结果、分组范围及解除结果、非法 radius 的就近错误均有证据；新增键盘焦点帧清楚显示 ↺ 的独立蓝色焦点环，UI-115 原剩余文案差距已被目标变更接受，达到本维度上限。
- **细节 1.00**：长路径与 token 名正常换行，搜索框和 Clear 对齐，模式文字、标题、诊断分组及范围文案在可见范围内未发现可扣分差异；U2 为规范模式，没有用于逐像素还原的目标 PNG。

满分限定于本次材料能观察的状态，未采到的行为不据此判为通过。

**2. 上一轮差距复核**

| 编号 | 状态 | 本轮依据 |
| --- | --- | --- |
| UI-194 | **已解决，本轮保持** | 桌面 source-overridden、detail、token-released 及两张 390 来源图中，来源块紧跟所选 primary，没有被其他 token 隔开。 |
| UI-172 | **已解决，本轮保持** | 桌面与 390 的 group-reset-tokens 首尾帧均列出 Brand、Neutral、Panel、Radius，包含 Light、Dark token 清单及 both modes 范围。 |
| UI-195 | **已解决，本轮保持** | 390 来源图的 “SEMANTIC COLORS”完整位于 Edit/Preview 分隔线以下。 |
| UI-196 | **已解决，本轮保持** | 选中 primary 的 “Light”与其他可见 token 的模式文字保持一致字重。 |
| UI-18 | **已解决，本轮保持** | “Fine adjustment source”“Color controls”与分组眉题层级清楚；桌面和窄屏一致。 |
| UI-115 | **不再适用，目标变更接受** | 直接入口仍为 ↺，来源区仍有 “Clear primary · Light only”；R6 比较口径明确接受不常显范围文字。这是范围接受，不是声称该文案已改。新增 keyboard-focus 帧显示 ↺ 有独立可见焦点，未因接受而豁免 G4。 |
| UI-197 | **已解决，本轮保持** | 两种视口低对比图均使用 “Fonts”“Text contrast pairs”，未出现同级标题计数口径冲突。 |
| UI-160 | **已解决，本轮保持** | 桌面 search-row 与 390 展开清单后的搜索行中，搜索框及 Clear 顶底对齐。 |

**3. 未关闭差距**

本轮未发现有当前目标依据、且能由白名单图片或操作记录证实的未关闭差距。无新增编号。

可挽回分合计 **0.00**，与 `10 − 10.00` 一致。

**4. 建议，不扣分**

分组影响清单中的 Neutral 段较密，可将 Light、Dark 改为各自独立的小节或列表，便于扫描。现状已经完整表达范围且正常换行，目标没有指定该结构，因此不扣分。

**5. 看不出的格子**

- `U2-token-reset-keyboard-focus-1440-light.png` 能确认焦点可见；不能确认 ↺ 的悬停提示、实际可访问名称、键盘激活结果，390px 也没有对应焦点帧。
- 来源图能看到 picker，但没有 picker 本体的键盘焦点帧，也没有非法颜色字符串的错误帧；不能将 radius 的错误结果外推至颜色字段。
- 两张 controls 图仅展示字体页上部，未覆盖全部 14 个共享字段、字体下拉展开选项或有效编辑后的字体结果。
- `U2-restore-1440-light.png` 能确认恢复入口及“配置、两模式全部值、舍弃后续调整”的范围说明；记录没有点击 Restore，不能确认恢复结果。
- 低对比桌面图能确认预览侧诊断及 Locate 入口；记录没有点击 Locate，不能确认实际定位。390px 低对比图只展示编辑侧诊断，没有窄屏预览诊断帧。
- 分组解除图能看到 Brand 的 Light/Dark 覆盖计数归零及 Neutral 覆盖仍保留；没有切换至 Dark 后逐项核验的画面。
- 静态截图不能独立证明拖拽、连续滚动、过渡或完整键盘顺序。

这些属于证据边界，不补造通过或失败，也不作为已证实缺陷扣分。

**6. 总评**

本次唯一受评单元 U2 为 **10.00 / 10**。本轮补齐了窄屏来源、解除、分组清单、字体、错误和诊断状态，直接重置按钮的键盘焦点也清楚可见。上一轮唯一剩余扣分项 UI-115 已经用户接受并写入冻结目标；当前证据未显示需要结构性改变的区域。

实际开图工具为 **`functions.exec` 调用 `tools.view_image`**。本轮实际打开 23 张当前原图、5 张 R5 对照图；未把目录、尺寸或文件读取算作开图。U2 是规范模式，无需引用其他单元的目标 PNG。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-detail-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-released-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-generated-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-controls-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-low-contrast-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-token-reset-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-restore-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-search-row-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-group-reset-tokens-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-token-released-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-tokens-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-tokens-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-group-reset-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-source-generated-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-controls-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-field-error-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U2-matrix-low-contrast-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-group-reset-tokens-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-field-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U2-low-contrast-1440-light.png
```
