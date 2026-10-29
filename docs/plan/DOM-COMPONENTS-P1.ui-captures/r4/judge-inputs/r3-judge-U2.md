# R3 评审结论 · U2 摘录

摘自 R3 合并结论 judge.md，只含 U2 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U2 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.7 | 2.65 | 2.75 | 0.6 | **8.7** | b | a 8.85 | 7.75 |

### 各维度理由

**U2（b）**

- 布局 2.7：来源块和拾色区插在所选格所在行的正下方，1440 下「Clear primary · Light only」在首屏内；常驻壳层收成 57px 顶栏；390 下吸顶区不再漏出内容。扣分原因：每个被覆盖格的状态胶囊折成两行椭圆；配置组状态胶囊与解除按钮之间只有 0–1px；390 下所选格被吸顶区盖住；诊断区有 44px 空白。
- 视觉 2.65：搜索框边框、helper、选中态、分体控件两段高亮、错误边框、标签页下划线都对上 G3。扣分原因：小节标题比字段 label 更浅而字号字重相同，层级倒置；诊断区外层 summary 常规、内层 600；同一种状态用了胶囊和 4px 矩形两种形状；字体 select 的值是 600；计数文字仍是 #667085。
- 交互 2.75：选中格后显示来源（显式覆盖、配置生成两类）；单项解除写明 Light only 且解除后禁用；分组解除写明 both modes；输入 `abc` 保留草稿并贴字段报错；低对比主题保留作者色并有可定位诊断。扣分原因：所选格无可解除覆盖时 ↺ 仍呈启用的蓝色；分组解除只给计数、不列出 token；「Restore document」没有写范围；网格 ↺ 仍只有图标。
- 细节 0.6：来源块键值结构不一致、「Light:」用了等宽；诊断条末行与外框形成双线；搜索框 36px 与 Clear 32px 不等高；嵌套 summary 重复同一计数；胶囊字号约 11px。

## R3 未关闭差距（U2）

#### UI-106 · [P3] 状态行与解除按钮之间 0px；两栏标题中线差约 4.5px（U1/U2 · 布局、细节 · 仍存在 · 可挽回 U1 a 0.1、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light 与 U2-group-reset-1440-light，Brand、Neutral、Panel 各组的状态胶囊底边与「Use configuration values · both modes」顶边相接（如 y=488/489）；U1-default-1440-light 应用名与预览标题中线差约 4.5px。
- 修法：配置组容器 `display:grid; gap:.5rem`（或状态行下边距 8px）；预览标题行 `min-height:2rem; align-items:center`，与编辑列首行同高同中线。
- 来源：U1 a UI-106（部分解决）；U2 a、b UI-106（仍存在）。

#### UI-107 · [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一（U1/U2 · 视觉 · 部分解决 · 可挽回 U1 a 0.05、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light、U2-group-reset-1440-light 的「Light N · Dark N」计数与「No overrides」胶囊文字为 #667085；U1-field-tabs-1440-light「Search colors…」占位符 #667085，U1-default-1440-light「Search presets…」约 #8E949C。
- 修法：计数与胶囊文字改用 #475467，#667085 只留给禁用态和占位符；两个搜索框共用一个 `::placeholder` token。
- 来源：U1 a、U2 a、U2 b UI-107（均部分解决）。

#### UI-160 · [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高（U1/U2 · 布局、细节 · 可挽回 U1 a 0.03、U2 b 0.05）

- 位置：U1-field-tabs-1440-light「Search colors…」y=537/572，「Clear」y=539/570；U2-source-overridden-1440-light 搜索框 y=73–108、「Clear」y=75–106。
- 修法：输入框与按钮同高，统一用壳层控件高度。
- 来源：U1 a 新-4、U2 b 新-9。判为同一问题的依据：同一对控件、同一高度差。

### U2

#### UI-163 · [P2] 1440 网格中「Overridden · Light」胶囊折成两行，文字压到圆角边，与配置组状态标识形状不一致（U2 · 布局、视觉 · 可挽回 a 0.35、b 0.2）

- 位置：U2-source-overridden、-detail、U2-token-released、U2-source-generated 四张 1440 图；如 background 格 x=20–157，胶囊 x≈50–125、y≈239–268，第二行「Light」离左侧弧边约 3px。有胶囊的格高约 80px，无胶囊的 ring 格约 45px。胶囊字号约 11px；同一状态在配置组里是 4px 圆角矩形加前导圆点。
- 现状 → 期望：圆角胶囊内文字折行贴边、行高参差、一个概念两种形状 → F4 分体控件；C3 状态标识完整可读；G2 分组行；G3 caption 12px。
- 修法：A 网格按编辑列宽自动降为单列（`repeat(auto-fill, minmax(13.5rem, 1fr))`），胶囊 `white-space:nowrap`；B 格内复用配置组的状态标识组件（4px 圆角、12px/600、前导 ●），拆成「● Overridden」「· Light」两个不换行片段，放在格内整行。
- 来源：U2 a 新-1、U2 b 新-1。判为同一问题的依据：同一批四张图、同一元素、同一折行现象。a 注明这是 UI-115 加了模式文字之后的副作用。

#### UI-17 · [P3] 选中后的滚动位置没有同时露出所选格与完整来源块（U2 · 布局、交互 · 部分解决 · 可挽回 a 0.15、b 0.05）

- 位置：U2-source-overridden-390-light 吸顶区占 y=0–约 200，所选 primary 格只露出胶囊那一行；U2-source-generated-1440-light ring 来源块下半部与「Clear ring · Light only」在折线以下。
- 修法：token 格 `scroll-margin-top` 设为吸顶区实测高度 + 8px；选中后先保证所选格顶边不高于吸顶区下沿，再让解除按钮 `scrollIntoView({block:'nearest'})`。
- 来源：U2 a UI-17（部分解决）；U2 b 判 UI-17 已解决，另列新-4（390 所选格被吸顶区盖住），判为同一问题：同一帧、同一现象。

#### UI-18 · [P3] 小节标题仍与次级标签或字段标签同样式，或比 label 更弱（U2 · 视觉 · 部分解决 · 可挽回 a 0.1、b 0.1）

- 位置：U2-controls-1440「Typography」与「Choose a bundled font」同为 12px/600 #475467，「Sans font stack」为 #1D2939 13px/600；U2-field-error-1440「Geometry」同上；U2-group-reset-1440「Theme configuration」与「Appearance」「Brand color」同为 13px/600 #1D2939；来源块标题「Fine adjustment source」与键「Effective value」同样式。
- 修法：小节标题统一为一种眉题样式（12px/600 大写、字距 .04em、#475467，上方 24px、下方 12px，上加 #EAECF0 分隔线），或改为 14px/600 #1D2939；字段 label 保持 13px/600 #1D2939；子标签降为 12px/400 #475467。
- 来源：U2 a、b UI-18（均部分解决）。

#### UI-165 · [P3] 来源块的键值结构只做了一半，行标签与块标题同样式（U2 · 布局、视觉、细节 · 可挽回 a 0.1、b 0.1 中的结构部分）

- 位置：U2-source-overridden-1440 y=637–873（-detail 图 y≈389–620 同）：只有「Effective value」一行是键值两列；token 名、「Explicit document override」、「overrides.light.primary」三行没有键名。
- 修法：来源块改为 `<dl>`，`grid-template-columns: 7rem 1fr`，行依次为 Token、Effective value、Source、Path、If cleared；dt 12px #475467，dd 13px #1D2939，路径 12px 等宽。
- 来源：U2 a 新-3、U2 b 新-5 的键值部分（同一区域、同一结构问题）。

#### UI-169 · [P3] 诊断区「Authored colors fit the sRGB preview.」下方有约 44px 空白（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440，该行墨迹 y=119–131，下一条从 y≈175 开始；其余条目间约 16px。
- 修法：内容为空时不渲染该节点，或统一由父级 `gap:16px` 控制、去掉子元素 margin。
- 来源：U2 a 新-7；U2 b 在 UI-22 条下记了同一空白。

#### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（U2 · 交互与状态 · 可挽回 b 0.1）

- 位置：U2-group-reset-1440-light Neutral「Light 15 · Dark 15」+「Use configuration values · both modes」（y≈635–690），Panel「Light 2 · Dark 2」（y≈841）。网格按 Surfaces、Semantic colors、Borders & focus 分组，与配置组不对应。
- 现状 → 期望：F2「解除将影响哪些项」。
- 修法：把计数做成 disclosure，展开后按模式列出 token 名；或点击计数时把网格按该组筛选到「Overridden only」。
- 来源：U2 b 新-8。

#### UI-164 · [P3] 选中格里不可解除的 ↺ 仍显示为选中蓝，看起来可用（U2 · 交互与状态 · 可挽回 a 0.05、b 0.1）

- 位置：U2-source-generated-1440 ring 格 ↺ 为 #175CD3，同屏未覆盖格为 #98A2B3；U2-token-released-1440 primary 已解除，↺ 仍是 #175CD3，而来源块的解除按钮已置灰。
- 修法：选中样式只改外框和底色；↺ 段图标色由自身状态决定，`aria-disabled` 时固定 #98A2B3。
- 来源：U2 a 新-2、U2 b 新-2（同一两帧、同一元素）。

#### UI-22 · [P3] 诊断区子 summary 比父 summary 更重，两组缩进不一（U2 · 视觉、布局 · 部分解决 · 可挽回 a 0.05、b 0.15）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs…」13px/400，内层「Text contrast pairs · 5 below 4.5:1」13px/600；字体诊断三条从 x=20 起且没有自己的标题，对比组从 x=32 起。
- 修法：外层改 13px/600 #1D2939，内层用 #475467；字体三条包进同级的 `<details>`（「Fonts · 3」），与对比组同样缩进。
- 来源：U2 a、b UI-22（均部分解决）。b 的这一条还写了 44px 空白与重复计数，分别归入 UI-169、UI-168。

#### UI-115 · [P3] 网格 ↺ 仍只有图标（U2 · 交互与状态 · 部分解决 · 可挽回 b 0.05）

- 位置：U2-source-overridden-1440-light 各格右侧的 ↺ 段。
- 现状 → 期望：模式已由相邻胶囊写出，但 ↺ 本身没有可见文字或提示 → F2 显示范围；C3 不只靠悬停。
- 修法：给 ↺ 加与来源块按钮同文案的 `aria-label` 和 `title`（如「Clear primary · Light only」）；补拍悬停与键盘聚焦帧。
- 来源：U2 b UI-115（部分解决）；U2 a 判已解决（可见部分，aria-label 截图不可见）。

#### UI-166 · [P3] 字体下拉的显示值是粗体，与其他下拉、输入框不一致（U2 · 视觉 · 可挽回 a 0.05、b 0.05）

- 位置：U2-controls-1440 三处「Custom author stack」为 600，同页与 U2-group-reset 的其他下拉值为 400。
- 修法：去掉字体下拉值的 `font-weight`；要提示自定义栈就在 label 旁加中性「Custom」标识。
- 来源：U2 a 新-4、U2 b 新-3。

#### UI-167 · [P3] 句中把模式名「Light:」也设成了等宽（U2 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U2-source-overridden-1440 y≈800、U2-source-overridden-390 y≈414「Clearing this override reveals Light: #d6409f.」
- 修法：`<code>` 只包住色值。
- 来源：U2 a 新-5、U2 b 新-5 的等宽部分。

#### UI-168 · [P3] 同一对比计数在三处用了三种措辞，父子 summary 重复同一个数字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs below 4.5:1」、内层「Text contrast pairs · 5 below 4.5:1」、预览条「5 text pairs below 4.5:1 · author colors preserved」。
- 修法：外层只写「Diagnostics」或「Diagnostics · 5 issues」，内层只写「Text contrast pairs」，预览条保留一种固定句式。
- 来源：U2 a 新-6；U2 b 在 UI-22 条下记了同一重复计数。

#### UI-170 · [P3] 预览侧诊断条末行分隔线与外框形成双线（U2 · 细节 · 可挽回 b 0.05）

- 位置：U2-group-reset-1440-light 末行分隔线 y=418、外框底边 y=431；U2-source-overridden、U2-low-contrast 同样。
- 修法：`.diag-row:last-child{border-bottom:0}`。
- 来源：U2 b 新-6。

#### UI-171 · [P3] 「Restore document · Graphite」没有写范围（U2 · 交互与状态 · 可挽回 b 0.05）

- 位置：U2-restore-1440-light，按钮在 y≈196–227，附近没有说明文字。
- 现状 → 期望：另两档重置都带范围后缀（「· Light only」「· both modes」），第三档只写了目标名 → F2「恢复预设/文档」须显示范围。
- 修法：按钮下加 12px #475467 说明，例如「Discards configuration changes and all overrides in both modes」；或把范围写进按钮文案。
- 来源：U2 b 新-7（补采帧）。

（U2 另有 UI-106、UI-107、UI-160 三条，见 U1 节。）

### U3

## 建议（不扣分，U2）

  - 来源块同时给出另一模式的状态并附分组解除入口（沿用 R2）。
  - primary 为中性灰时拾色平面不要显示饱和蓝（沿用 R2）。
  - 「Overridden only」写出分模式计数（沿用 R2）。
  - 预览诊断条超过 3–4 对时折叠（a、b）。
  - Borders & focus 等没有配对的组只占左列，可按两列流式排布（b）。
  - 390 下滚动时收起头区、只保留 Edit/Preview 标签（a、b，属 U1 范围）。
  - 在精调区末尾也放一个「Restore document」入口（b）。
