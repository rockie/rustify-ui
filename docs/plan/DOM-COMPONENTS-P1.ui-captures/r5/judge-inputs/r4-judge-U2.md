# R4 独立评审结论摘录 · U2

仅摘录 R4 judge.md 中本单元的评分、理由和全部未关闭差距；不含主 agent 裁定、机器检查和修复记录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.85 | 2.9 | 2.85 | 0.8 | **9.4** | 8.7 |


- 布局 2.85：网格改为单列，状态标识不再折行；1440 下所选格与完整来源块同屏，390 下所选格完整露出；配置组状态行与按钮间距 8px。扣分：单列仍沿用两列时的配对行，来源块挂在配对伙伴之后，配对内间距 6px 大于配对间 4px；390 下「Semantic colors」组标题被吸顶条切掉上半部。
- 视觉 2.9：Typography、Geometry、Theme configuration 节标题统一为 14px/600 #1D2939；字体下拉值改回常规；状态标识与配置组同形；错误边框与焦点环符合 G3。扣分：「Fine adjustment source」与 Colors 页分组标题仍是 #475467；所选格的模式文字「Light」选中时变粗。
- 交互 2.85：两类来源及有效值、路径、「If cleared」都能读出；单项解除写明 Light only，不可解除的 ↺ 置灰；分组解除写明 both modes；Restore document 补了范围说明；`abc` 贴字段报错；低对比保留作者色并可定位。扣分：分组解除只给计数；网格 ↺ 仍只有图标。
- 细节 0.8：R3 的等宽「Light:」、重复计数、44px 空白、双线、键值结构都修好。扣分：配对内外间距倒置；390 组标题半截；选中格「Light」变粗；「Fonts · 3」带计数而同级「Text contrast pairs」不带。

## 待复核旧差距


#### UI-194 · [P3] 单列网格仍沿用「配对行」结构：来源块没有紧跟所选格，配对内外间距倒置（U2 · 布局、细节 · 可挽回 0.15）

- 位置：U2-source-overridden-1440-light primary 格 y=342–418 之后是 primary-foreground 格 y=425–502，再是来源块（y=535 起）；U2-token-released、U2-source-overridden-390 同样。card-foreground → popover 4px，popover → popover-foreground 6px。
- 期望：F1、F2 来源说明直接附在所选 token 下；G2 间距表达分组。
- 修法：拆掉配对行容器（或 `display: contents`），token 格成为列表直接子项；来源块与 Color controls 插在所选格之后；保留配对就让配对内 4px、配对间 12px。
- 来源：U2 新-1（评审注明这是 UI-163 单列修法的副作用）。

#### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（U2 · 交互与状态 · 仍存在 · 可挽回 0.1）

- 位置：U2-group-reset-1440-light Neutral「Light 15 · Dark 15」（y=644–667）、Panel「Light 2 · Dark 2」，没有列表或展开入口。
- 期望：F2「解除将影响哪些项」。
- 修法：计数做成 `<details>` 的 summary，展开按模式列出 token 名；或加「Show in list」把网格筛到「Overridden only + 该组」。
- 来源：U2 UI-172（仍存在）。

#### UI-195 · [P3] 390 下吸顶的 Edit/Preview 条没有下分隔线，「Semantic colors」组标题被切成半截（U2 · 布局、细节 · 可挽回 0.1）

- 位置：U2-source-overridden-390-light，标签在 y=136–167，内容从 y≈176 露出，「Semantic colors」只剩 y=176–182 的下半部，吸顶条与内容之间无分隔线。
- 期望：G2 各区独立滚动、边界清晰；G1。
- 修法：窄屏吸顶条加 1px #EAECF0 下边框；所选格是组内首格时滚动组标题，`scroll-margin-top` 取吸顶条实测高度 + 8px。
- 来源：U2 新-2。

#### UI-196 · [P3] 所选格里的模式文字「Light」在选中时变粗（U2 · 视觉、细节 · 可挽回 0.08）

- 位置：U2-source-overridden-1440-light 选中的 primary 格「Light」（x=122–148，y=394–406）暗像素 94 个，未选中格 60 个；390 同样。
- 期望：G3 选中态只改背景、边框与 token 名颜色；C3 状态标识形态一致。
- 修法：去掉选中态作用于模式后缀的 `font-weight`，固定 12px/400 #475467。
- 来源：U2 新-3。

#### UI-18 · [P3] Colors 页节标题、分组标题仍是旧样式（U2 · 视觉 · 部分解决 · 可挽回 0.07）

- 位置：U2-source-overridden-1440-light「Fine adjustment source」14px/600 #475467、「Semantic colors」13px/600 #475467；U2-source-overridden-detail「Color controls」、U2-source-generated「Borders & focus」同。对照：其他页节标题 14px/600 #1D2939，字段 label 13px/600 #1D2939。
- 修法：所选 token 的详情节标题统一 14px/600 #1D2939；列表分组改眉题（12px/600 大写、字距 .04em、#475467）。
- 来源：U2 UI-18（部分解决；Typography、Geometry、Theme configuration 已改好）。

#### UI-115 · [P3] 网格 ↺ 仍只有图标（U2 · 交互与状态 · 仍存在 · 可挽回 0.05）

- 位置：U2-source-overridden-1440-light、-390 各格右侧 x=273–297 的 ↺ 段；aria-label、title 截图不可见，没有悬停或聚焦帧。
- 期望：F2 显示范围；C3 不只靠悬停。
- 修法：可解除的 ↺ 加与来源块按钮同文案的 `aria-label` 与 `title`；不可解除的 `aria-disabled` 并写「No override to clear」；聚焦时在状态行末尾显示「↺ Light only」；补拍悬停与聚焦帧。
- 来源：U2 UI-115（仍存在，可见部分）。

#### UI-197 · [P3] 诊断区两个同级 summary 一个带计数、一个不带（U2 · 细节 · 可挽回 0.05）

- 位置：U2-low-contrast-1440-light「Fonts · 3」（y=154–164）与「Text contrast pairs」（y=345–357）。
- 修法：统一为「名称 · 总数」（N 为已检查的配对总数，不重复不达标数），或都去掉计数。
- 来源：U2 新-4。

#### UI-160 · [P3] 颜色搜索框与 Clear 不等高（U1/U2 · 布局、细节 · 待核实）

- 状态：U1 评审判已解决（U1-field-tabs-1440-light 搜索框 y=536–567、Clear y=536–567，都是 32px）；U2 评审判待核实（U2 各帧中搜索行被选中后的自动滚动移出视口）。按复核顺序取待核实。
- 来源：U1 UI-160（已解决）、U2 UI-160（待核实）。

