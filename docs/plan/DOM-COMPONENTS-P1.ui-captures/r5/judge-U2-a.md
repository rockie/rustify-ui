# R5 独立评审原文 · U2

仅评 U2，依据冻结的 11 张 R5 原始截图、对应操作记录、R4 白名单原图与结论摘录。U2 为规范模式，比较依据是目标包 F1–F4、G1–G4；未读取代码、修复记录或机器检查。

## 1. 评分表

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 /10 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U2 Studio 精调与来源解释 | 3.00 | 3.00 | 2.95 | 1.00 | **9.95** |

- **布局 3.00**：已到该维度上限；所选 token 后直接衔接来源说明，列表间距一致，390px 下分组标题完整、吸顶边界可见，已呈现内容未见遮挡或横向溢出。
- **视觉 3.00**：已到该维度上限；来源节标题与字体页层级一致，颜色分组改为清楚的眉题，选中项的 Light 不再额外加粗，字段错误文字与边框的原图实色为目标规定的 `#b42318`。
- **交互与状态 2.95**：来源、有效值、路径、解除后的结果以及单项和文档恢复范围均可读，非法值保留有效预览；未满分的原因是列表中的快捷解除仍只有 ↺，静态可见界面未直接说明该快捷动作的范围。
- **细节 1.00**：已到该维度上限；上轮确认的间距、标题截断、模式字重、summary 计数不一致和搜索行等高问题均已消除，未发现新的、可确证的细节差异。

本分数只覆盖已呈现状态。UI-172 的展开内容缺少截图，既不算通过，也不因看不到内容扣分。

## 2. 上一轮差距复核表

| 编号 | 本轮状态 | 复核依据 |
| --- | --- | --- |
| UI-194 | **已解决** | `U2-source-overridden-1440-light.png`、detail、token-released 及 390 图中，来源块均紧接所选 primary，primary-foreground 不再夹在二者之间；可见列表项间距一致。 |
| UI-172 | **待核实** | group-reset 图仍显示 Neutral 的 `Light 15 · Dark 15` 等计数；但 search-row 图已能看到 `Tokens affected by a group reset` 折叠入口，因此不能继续认定“没有展开入口”。入口未展开，无法确认内部是否按组和模式列出受影响 token。 |
| UI-195 | **已解决** | 390 图中 Edit/Preview 吸顶区下方已有分隔线，`SEMANTIC COLORS` 标题完整可见，不再只剩下半截。 |
| UI-196 | **已解决** | 1440 和 390 选中 primary 的 Light 与未选中项模式文字保持相同的常规字重。 |
| UI-18 | **已解决** | `Fine adjustment source`、`Color controls` 与字体页节标题形成一致层级；`SEMANTIC COLORS`、`BORDERS & FOCUS` 已统一为眉题。 |
| UI-115 | **仍存在（可见部分）** | 桌面与 390 token 列表的快捷解除段仍只显示 ↺；没有本轮悬停或键盘焦点帧，不能判断其隐藏提示或可访问名称。 |
| UI-197 | **已解决** | low-contrast 图中 Fonts 已去掉 `· 3`，与同级 `Text contrast pairs` 的 summary 格式一致。 |
| UI-160 | **已解决** | search-row 图中 Search colors 与 Clear 的上下边缘对齐、等高；本轮终于直接覆盖了 U2 的这条搜索行。 |

## 3. 未关闭差距清单

### UI-115 · 列表快捷解除的范围仍未直接可见

- **单元与截图位置**：U2；`U2-source-overridden-1440-light.png` primary 行最右侧、`U2-source-overridden-390-light.png` primary 行最右侧，以及 search-row 图中各 token 行的 ↺ 段。
- **维度**：交互与状态。
- **严重度**：P3。
- **现状 → 期望**：快捷操作只显示 ↺；来源块中的独立按钮已写明 `Clear primary · Light only`，但列表快捷操作没有同等可见的范围解释。目标 F1、F2 要求重置有明确作用域、能够知道解除影响哪些项。
- **具体修法**：在颜色列表头或搜索行附近增加常驻说明，例如 `↺ clears this token’s Light override only`，随当前模式更新；也可在解除段直接显示简短范围文字，同时保留 F4 的分体控件外形。可访问名称是否已具备无法由截图判断，应另用键盘焦点帧核实，不据此重复扣分。
- **可挽回分**：**0.05**。

### UI-172 · 分组解除的 token 清单入口已可见，展开结果缺证

- **单元与截图位置**：U2；`U2-group-reset-1440-light.png` 左列 Neutral、Panel 的覆盖计数；`U2-search-row-1440-light.png` 左列约 y=374 的 `Tokens affected by a group reset` 折叠入口。
- **维度**：交互与状态。
- **严重度**：P3，沿用原问题级别，当前为证据待核实。
- **现状 → 期望**：已有明确的清单入口，但截图只拍到折叠状态。目标 F2 要求能回答“解除将影响哪些项”，目前无法确认入口内部呈现的内容。
- **具体修法**：先展开现有入口，在有覆盖的配置下拍摄组名、模式和 token 名清单。若现有内容已能说明对应分组解除的影响范围，则无需改产品；若只有说明文字或计数，再补具体 token 名及模式范围。
- **可挽回分**：**0.00**；缺证不作为产品故障扣分，展开内容补齐后才能关闭。

已确认差距可挽回分合计 **0.05**，等于 `10 − 9.95`。

## 4. 建议（不扣分）

无额外产品建议。当前应优先补齐既有入口的展开证据。

## 5. 看不出的格子与实际打开图片

- `U2-search-row-1440-light.png`：看不出 `Tokens affected by a group reset` 展开后的清单及各组、各模式的对应关系。
- token 列表各图：看不出 ↺ 的悬停提示、键盘焦点样式和可访问名称。
- `U2-source-overridden-detail-1440-light.png`：底部颜色格式和值字段仅部分露出，无法核实该处输入焦点或非法颜色状态。
- `U2-field-error-1440-light.png`：能确认 `abc` 就地报错且预览保持有效；未拍到 Escape 或失焦后的恢复结果。
- `U2-low-contrast-1440-light.png`：能确认作者低对比颜色保留、编辑壳可读及 Locate 入口可见；没有 Locate 操作后的目标定位帧。
- 390px 本轮只有所选 token 来源状态；没有窄屏分组解除展开、字体页或字段错误帧。这些状态不作通过或失败判断。

实际逐张打开的 **R5 图片共 11 张**：

1. `U2-source-overridden-1440-light.png`
2. `U2-source-overridden-detail-1440-light.png`
3. `U2-token-released-1440-light.png`
4. `U2-group-reset-1440-light.png`
5. `U2-source-generated-1440-light.png`
6. `U2-controls-1440-light.png`
7. `U2-field-error-1440-light.png`
8. `U2-low-contrast-1440-light.png`
9. `U2-source-overridden-390-light.png`
10. `U2-restore-1440-light.png`
11. `U2-search-row-1440-light.png`

实际逐张打开的 **R4 图片共 10 张**：

1. `U2-controls-1440-light.png`
2. `U2-field-error-1440-light.png`
3. `U2-group-reset-1440-light.png`
4. `U2-low-contrast-1440-light.png`
5. `U2-restore-1440-light.png`
6. `U2-source-generated-1440-light.png`
7. `U2-source-overridden-1440-light.png`
8. `U2-source-overridden-390-light.png`
9. `U2-source-overridden-detail-1440-light.png`
10. `U2-token-released-1440-light.png`

所有原图均可辨认；上述限制来自状态未展开或未拍摄，并非图片模糊。

## 6. 总评

本次只评 U2，得分 **9.95/10**。上轮的可见布局与视觉差距已经解决，无需结构性重做。剩余可确认问题是快捷解除的范围说明；分组解除已有清单入口，其展开内容仍须核实。
