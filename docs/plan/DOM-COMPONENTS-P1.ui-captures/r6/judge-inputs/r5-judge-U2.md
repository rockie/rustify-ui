**1. 评分**

仅评 U2，依据白名单中的截图与操作记录评分；未拍到的行为不算通过，也不算失败。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 2.95 | 1.00 | **9.95 / 10** |

- **布局 3.00**：所选 token 后直接跟随来源说明；桌面与 390px 均可读，窄屏组标题完整，搜索框与 Clear 等高；可见范围内未发现布局失分项。
- **视觉 3.00**：来源节标题、分组标题、选中状态和字段错误样式已统一；像素抽样确认错误文字与边框为目标规定的 `#b42318`，可见范围内未发现应扣分的视觉偏差。
- **交互与状态 2.95**：覆盖来源、生成来源、单项解除结果、分组解除范围清单和字段错误均有证据；未满分的原因是网格内直接重置入口仍只有 ↺，动作自身未显示范围。
- **细节 1.00**：上一轮的配对间距倒置、窄屏标题半截、选中模式文字变粗及诊断标题计数不一致均已消除；本次未发现其他有规范依据的细节差异。

**2. 上一轮差距复核**

| 编号 | 状态 | 本轮依据 |
| --- | --- | --- |
| UI-194 | **已解决** | `U2-source-overridden-1440-light.png`、detail、390 与 token-released 中，来源块均直接位于所选 primary 后，不再隔着 primary-foreground；可见列表行间距一致。 |
| UI-172 | **已解决** | 两张补采图展开了 “Tokens affected by a group reset”，Brand、Neutral、Panel、Radius 均列出 Light/Dark 的 token 名及 both modes 范围，不再只有计数。 |
| UI-195 | **已解决** | 390 图的 Edit/Preview 条有下分隔线，“SEMANTIC COLORS”完整位于其下，未被切掉上半部。 |
| UI-196 | **已解决** | 桌面与 390 图中，选中 primary 的 “Light”保持常规字重，与其他 token 的模式文字一致。 |
| UI-18 | **已解决** | “Fine adjustment source”“Color controls”已使用深色标题；“SEMANTIC COLORS”“BORDERS & FOCUS”等采用一致的眉题处理。 |
| UI-115 | **仍存在，可见部分** | 网格右侧仍只有 ↺；来源块中的 “Clear primary · Light only”已明确，但该说明没有出现在直接重置入口上。没有 ↺ 的悬停、键盘聚焦帧。 |
| UI-197 | **已解决** | low-contrast 图中同级 summary 为 “Fonts”与 “Text contrast pairs”，已统一不带计数。 |
| UI-160 | **已解决** | 新增 search-row 图完整显示搜索行；搜索框和 Clear 的顶底对齐，均为 32px 高。 |

**3. 未关闭差距**

**UI-115 · U2 · 网格直接重置入口仍未显示动作范围**

- **截图与位置**：`U2-source-overridden-1440-light.png`、`U2-source-overridden-detail-1440-light.png`、`U2-source-overridden-390-light.png`，token 分体控件最右侧的 ↺ 段；其他可解除 token 同样。
- **维度 / 严重度**：交互与状态 / **P3**。
- **现状 → 期望**：重置动作自身只有 ↺；“Light”目前属于覆盖状态行，完整的 “Clear primary · Light only”要在来源块中另读。按 F1、F2，单 token 重置入口应明确表达解除范围。
- **具体修法**：在可解除的重置段中加入常驻的短范围文案，例如“↺ Light only”，必要时让右段按内容扩宽；保留 F4 要求的外缘圆角、内部单条分隔线，390px 下让 token 名和值正常换行。可访问名称应与可见范围文案一致，但本轮没有证据判断其当前实现。
- **可挽回分：0.05**，沿用 R4 尺度。

可挽回分合计 **0.05**，与 `10 − 9.95` 一致。未发现新的扣分差距。

**4. 建议，不扣分**

展开后的 Neutral 清单可以将 Light、Dark 分成更清晰的小节，减少连续长段落的扫描负担。当前已经完整列出范围，目标包未规定必须采用某一种列表形式，因此不扣分。

**5. 看不出的格子与实际查看记录**

以下状态不能从本次材料确认：

- 网格 ↺ 的悬停、键盘焦点、聚焦时提示及可访问名称。
- picker 的键盘焦点环，以及非法颜色字符串的字段错误；本次非法输入证据是 Corner radius 的 `abc`。
- `U2-controls-1440-light.png` 未覆盖全部共享字段、字体下拉展开选项及有效编辑后的结果。
- `U2-low-contrast-1440-light.png` 显示诊断和 Locate 入口，但记录未包含点击 Locate，不能确认其实际定位结果。
- `U2-restore-1440-light.png` 显示恢复入口及完整范围说明，未记录点击后的恢复结果。
- 390px 仅覆盖所选 token 与来源区，未覆盖窄屏字段错误、展开后的分组清单或键盘走查。
- 静态截图不能独立证明滚动、拖拽和过渡行为。

本轮 **13 张原图均逐张打开**：

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
```

R4 白名单中的 **10 张原图均逐张打开**：

```text
U2-controls-1440-light.png
U2-field-error-1440-light.png
U2-group-reset-1440-light.png
U2-low-contrast-1440-light.png
U2-restore-1440-light.png
U2-source-generated-1440-light.png
U2-source-overridden-1440-light.png
U2-source-overridden-390-light.png
U2-source-overridden-detail-1440-light.png
U2-token-released-1440-light.png
```

所有已打开图片均足够清晰；上述未确认项来自状态未拍摄或操作未记录。

**6. 总评**

本次只评 U2，得分 **9.95**。来源说明与所选 token 的关系、分组解除影响清单、窄屏标题和字段错误呈现均达到本单元的可见要求。剩余差距是直接重置入口的范围文案，属于局部打磨，**不需要结构性重做**。
