# U2 Studio 精调与来源解释 · R4 评审原文

评审范围：只评 U2，依据只有 r4 本层 10 张 `U2-*.png`、`r4/U2-actions.log`、`r4/capture.md`，以及上一轮的 r3 U2 图和 `r3-judge-U2.md` 摘录。被测版本 HEAD `5d122d6`，Studio build `28ae38531384c902`（见 `r4/capture.md` 第 4–5 行）。像素坐标是对原图逐像素取样得到的 CSS px（DPR 1）。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.85 | 2.9 | 2.85 | 0.8 | **9.4** | 8.7 |

**各维度理由**

- **布局 2.85**：网格已改为单列，状态标识不再折行。1440 下所选格与完整来源块（含「Clear … · Light only」）同屏，390 下所选格完整露在吸顶区下方，配置组状态行与按钮间距为 8px。不到 3 的原因有两处：
  - 单列仍沿用两列时代的「配对行」结构。primary 的来源块插在 primary-foreground 之后，而不是紧跟 primary；配对内间距 6px，反而大于配对间的 4px。
  - 390 下「Semantic colors」组标题被吸顶条切掉上半部。
- **视觉 2.9**：以下各项都对上 G3：
  - Typography、Geometry、Theme configuration 三个节标题已统一为 14px/600 #1D2939；
  - 字体下拉值改回常规字重；
  - 状态标识与配置组同形（4px 圆角、前导圆点、12px/600 #475467、底 #F9FAFB）；
  - 错误边框 #B42318，焦点环 3px #A1C6F9、2px offset。

  不到 3 的原因：Colors 页的「Fine adjustment source」和各分组标题仍是 #475467，与其他页同级标题的 #1D2939 不一致；所选格里的模式文字「Light」在选中时变为粗体。
- **交互与状态 2.85**：显式覆盖、配置生成两类来源，以及有效值、路径和「If cleared」后果都能读出。单项解除写明 Light only，解除后变灰；不可解除的 ↺ 已置灰。分组解除写明 both modes，Restore document 也补上了范围说明。`abc` 保留草稿并在字段旁报错。低对比主题保留作者色，预览侧有可定位的诊断条。不到 3 的原因：分组解除仍只给计数、不列出受影响的 token；网格里的 ↺ 仍只有图标。
- **细节 0.8**：R3 的五处细节都已修好（等宽「Light:」、重复计数、44px 空白、双线、键值结构）。不到 1 是因为还有四处小差异：
  - 配对内与配对间的间距倒置（6px 对 4px）；
  - 390 组标题被切成半截；
  - 选中格的「Light」变粗；
  - 诊断区「Fonts · 3」带计数，同级的「Text contrast pairs」不带。

## 2. 上一轮差距复核表

| 编号 | R3 摘要 | R4 判定 | 依据（R4 截图） |
| --- | --- | --- | --- |
| UI-106 | 状态胶囊与解除按钮间 0px | **已解决**（U2 部分） | U2-group-reset：Brand 徽标 y=466–489、按钮自 498；Neutral 644–667 → 676；Panel 848–871 → 880，间距都是 8px。「两栏标题中线差」属 U1，本次不评 |
| UI-107 | 计数文字 #667085；占位符不一 | **已解决**（U2 可见部分） | group-reset「Light 0 · Dark 0」「Light 15 · Dark 15」「No overrides」最深像素都是 #475467。占位符部分本轮 U2 无图，见第 5 节 |
| UI-160 | 搜索框 36px 与 Clear 32px 不等高 | **待核实** | 本轮选中 token 后编辑列自动滚走，所有 U2 帧都看不到「Search colors…」行 |
| UI-163 | 胶囊折两行、形状不一 | **已解决** | 单列。「● Overridden」为 4px 圆角矩形，12px/600 #475467，底 #F9FAFB、边 #D0D5DD；「Light」在框外同行。与配置组同形，不再折行 |
| UI-17 | 选中后没有同时露出所选格与完整来源块 | **已解决** | 1440：primary 格 y=342–418 与「Clear primary · Light only」y=851–883 同屏；ring 格 434–482 与「Clear ring · Light only」同屏。390：primary 格 193–270 完整，Clear 在 702–734。相关的新问题见 新-2 |
| UI-18 | 小节标题层级倒置 | **部分解决** | 已改：Typography（y=246–259）、Geometry（388–400）、Theme configuration（94–107）为 14px/600 #1D2939；「Choose a bundled font」降为 12px/400 #475467。仍存：「Fine adjustment source」为 14px/600 **#475467**；「Semantic colors」「Borders &amp; focus」「Color controls」为 13px/600 #475467，与字段 label（13/600 #1D2939）同号同重、只是更浅 |
| UI-165 | 来源块键值结构只做一半 | **已解决** | 改为 Token / Effective value / Source / Path / If cleared 五组。键 12px #475467，值 13px #1D2939；token、色值和路径用等宽字 |
| UI-169 | 诊断区 44px 空白 | **已解决** | low-contrast：「Authored colors fit…」y=121–133，下一条「Fonts · 3」从 y=154 开始 |
| UI-172 | 分组解除只给计数 | **仍存在** | group-reset：Neutral 仍只有「Light 15 · Dark 15」，Panel 只有「Light 2 · Dark 2」，没有列表，也没有展开入口 |
| UI-164 | 不可解除的 ↺ 仍是选中蓝 | **已解决** | source-generated 的 ring ↺ 为 #98A2B3；token-released 的 primary ↺ 为 #98A2B3。可解除的 ↺ 为 #475467，选中且可解除时为 #175CD3 |
| UI-22 | 子 summary 比父更重，缩进不一 | **已解决** | 外层「Diagnostics」13px/600 #1D2939；内层「Fonts · 3」「Text contrast pairs」为 #475467，都从 x=32 开始，字体组已包进同级 details |
| UI-115 | 网格 ↺ 只有图标 | **仍存在**（可见部分） | 各格 ↺ 段外观不变。aria-label 和 title 截图不可见，本轮也没有悬停或聚焦帧 |
| UI-166 | 字体下拉值为粗体 | **已解决** | controls：「Custom author stack」墨迹宽 120px（R3 为 125px），已是常规字重 |
| UI-167 | 「Light:」被设为等宽 | **已解决** | 「Light:」为无衬线，只有「#d6409f」等宽，单独成行（y=805–817） |
| UI-168 | 同一计数三处三种措辞 | **已解决** | 外层只写「Diagnostics」，内层「Text contrast pairs」不带计数，预览条保留「5 text pairs below 4.5:1 · author colors preserved」 |
| UI-170 | 诊断条末行双线 | **已解决** | group-reset 末行从 y=378 开始，到外框底边 y=438 之间没有分隔线；low-contrast 同样 |
| UI-171 | Restore document 没写范围 | **已解决** | restore 帧按钮下新增 12px #475467 说明「Restores the reference document's configuration and all values in both modes, discarding later adjustments.」（y=239–287） |

## 3. 差距清单

全部为 P3，按可挽回分从高到低排列。可挽回分合计 0.6，等于 10 − 9.4。

#### 新-1 · [P3] 单列网格仍沿用「配对行」结构：来源块没有紧跟所选格，配对内外间距倒置

- **单元与截图**：
  - U2-source-overridden-1440-light：primary 格 y=342–418，primary-foreground 格 y=425–502，分隔线 y=515，来源块标题从 y=535 开始。
  - U2-token-released-1440-light：primary 123–171 → primary-foreground 178–254 → 来源块。
  - U2-source-overridden-390-light：primary 193–270 → primary-foreground 277–353 → 分隔线 366 → 来源块。
  - 间距：card-foreground（61–138）到 popover（143–220）为 4px；popover 到 popover-foreground（227–304）为 6px。source-generated 中 accent-foreground 到 destructive 为 4px，destructive 到 destructive-foreground 为 6px，border、input、ring 之间都是 4px。
- **维度**：布局、细节。
- **现状 → 期望**：
  - 现状：解释 primary 来源的块与 primary 格之间隔着另一个可操作的格（primary-foreground，有自己的 ↺ 和「● Overridden」）。同一配对内的间距反而大于配对之间，按距离读会把 popover-foreground 看成和下一组更近。这是 R3 把两列改单列（UI-163 修法 A）之后的副作用：R3 的两列里，配对行就是一行，块插在行后是对的。
  - 期望：F1「展示有效值、来源、覆盖与重置」、F2「来源说明能回答值来自哪里」，来源说明应直接附在所选 token 下面；G2「细分隔线与分组行」要求间距表达分组。
- **修法**：
  1. 拆掉配对行容器，或给它设 `display: contents`，让每个 token 格成为列表 `display: grid; gap: 6px`（或统一 8px）的直接子项。
  2. 渲染时把来源块（以及其后的 Color controls）插在所选格之后，DOM 顺序与视觉顺序一致。
  3. 若要保留「底色 / 前景」配对，就让配对内 `gap: 4px`、配对间 `margin-top: 12px`，这样间距表达分组；来源块仍插在所选格之后，不插在配对之后。
- **可挽回分**：0.15（布局 0.1，细节 0.05）。

#### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（仍存在）

- **单元与截图**：U2-group-reset-1440-light。
  - Neutral：「● Overridden」「Light 15 · Dark 15」在 y=644–667，下方按钮「Use configuration values · both modes」在 676–708。
  - Panel：「Light 2 · Dark 2」在 y≈850–870，下方按钮「Use base values · both modes」。
- **维度**：交互与状态。
- **现状 → 期望**：现状只有两个计数。期望按 F2 回答「解除将影响哪些项」，即受影响的 token 要能读出。Colors 网格按 Surfaces、Semantic colors、Borders &amp; focus 分组，与配置组对不上，用户无法自己推出这 15 项是哪些。
- **修法**：把「Light 15 · Dark 15」做成 `&lt;details&gt;` 的 summary，展开后按模式分两列列出 token 名（12px 等宽 #475467，每行一个，超过 8 个时滚动或折叠）。或者在按钮旁加「Show in list」，点击后切到 Colors 页，并把网格筛选为「Overridden only + 该组」。
- **可挽回分**：0.1。

#### 新-2 · [P3] 390 下吸顶的 Edit/Preview 条没有下分隔线，「Semantic colors」组标题被切成半截

- **单元与截图**：U2-source-overridden-390-light。头区分隔线在 y=127，Edit/Preview 标签在 136–167，内容从 y≈176 起露出。「Semantic colors」墨迹只剩 y=176–182（完整高约 11px），上半部被吸顶条盖住；吸顶条与内容之间没有任何分隔线。
- **维度**：布局、细节。
- **现状 → 期望**：现状是组标题露出半截文字，滚动边界也看不出来。期望按 G2「各区独立滚动」和 G1「稳定中性编辑壳」，滚动区的边界应清晰，文字不被切成半截。
- **修法**：
  1. 窄屏吸顶条加 `border-bottom: 1px solid #EAECF0`，背景保持 #FFFFFF。
  2. 选中时的滚动目标改为：所选格是本组首格时，滚动该组标题，`scroll-margin-top` 等于吸顶条实测高度 + 8px；否则滚动所选格本身，用同样的 scroll-margin。
- **可挽回分**：0.1（布局 0.05，细节 0.05）。

#### 新-3 · [P3] 所选格里的模式文字「Light」在选中时变粗

- **单元与截图**：
  - U2-source-overridden-1440-light：选中的 primary 格「Light」（x=122–148，y=394–406）暗像素 94 个；未选中的 card-foreground、primary-foreground 格「Light」都是 60 个。目视为 600 对 400。
  - U2-source-overridden-390-light 的 primary 格同样如此。
  - 同一格里的「● Overridden」徽标在选中前后不变。
- **维度**：视觉、细节。
- **现状 → 期望**：现状是选中会改变状态行的字重，同一状态标识在不同格里不一样。期望按 G3，选中态只体现为背景 #EFF8FF、边框 #84CAFF、文字 #175CD3（作用于 token 名）；按 C3，状态标识形态一致，与配置组的「Light 15 · Dark 15」（400）同样式。
- **修法**：删除选中态选择器里作用于模式后缀的 `font-weight`，模式后缀固定为 12px/400 #475467。
- **可挽回分**：0.08（视觉 0.03，细节 0.05）。

#### UI-18 · [P3] Colors 页的节标题、分组标题仍是旧样式（部分解决）

- **单元与截图**：
  - U2-source-overridden-1440-light：「Fine adjustment source」（y=535–548）为 14px/600 **#475467**；「Semantic colors」（321–331）为 13px/600 #475467。
  - U2-source-overridden-detail-1440-light：「Color controls」（683–693）为 13px/600 #475467。
  - U2-source-generated-1440-light：「Borders &amp; focus」（250–260）同上。
  - 对照：同一产品里「Theme configuration」「Typography」「Geometry」为 14px/600 #1D2939，字段 label（Appearance、Brand color）为 13px/600 #1D2939。
- **维度**：视觉。
- **现状 → 期望**：现状是同级的 14px 节标题在不同页颜色不同；Colors 页分组标题与字段 label 同号同重，只是更浅，仍有 R3 说的层级倒置。期望按 G3 的层级：节标题 &gt; label 13px/600 &gt; caption 12px，同级标题同样式。
- **修法**：
  - 「Fine adjustment source」与「Color controls」都是所选 token 的详情节，统一用现有节标题样式 14px/600 #1D2939。
  - 「Surfaces」「Semantic colors」「Borders &amp; focus」这类列表分组改为眉题：12px/600、大写、`letter-spacing: .04em`、#475467，与 13px 字段 label 拉开差别。
- **可挽回分**：0.07。

#### UI-115 · [P3] 网格 ↺ 仍只有图标（仍存在，可见部分）

- **单元与截图**：U2-source-overridden-1440-light、-390 中各格右侧 x=273–297 的 ↺ 段。
- **维度**：交互与状态。
- **现状 → 期望**：现状是 ↺ 本身没有可见文字或提示，只靠相邻的「Light」推断范围。期望按 F2「须显示范围」、C3「不只靠颜色或悬停」，范围应该能直接读到。
- **修法**：
  - 给可解除的 ↺ 加与来源块按钮同文案的 `aria-label` 和 `title`（如「Clear primary · Light only」）。
  - 不可解除的 ↺ 设 `aria-disabled="true"`，`title` 为「No override to clear」。
  - 聚焦时在格内状态行末尾显示「↺ Light only」文字提示。
  - 补拍悬停帧和键盘聚焦帧，使这一条能核实。
- **可挽回分**：0.05。

#### 新-4 · [P3] 诊断区两个同级 summary 一个带计数、一个不带

- **单元与截图**：U2-low-contrast-1440-light，「Fonts · 3」（y=154–164）与「Text contrast pairs」（y=345–357）同为 x=32 起、#475467 的同级 summary。
- **维度**：细节。
- **现状 → 期望**：现状是同级标题格式不一。期望同级 summary 用同一种句式。按 UI-168 的已有修法，不能再重复「5 below 4.5:1」。
- **修法**：两者统一为「名称 · 总数」，即「Text contrast pairs · N」，N 为已检查的配对总数，不是不达标数，这样不与预览条重复；或者两者都去掉计数。
- **可挽回分**：0.05。

## 4. 建议（不扣分）

- 来源块同时给出另一模式（Dark）的状态，并附所属分组的解除入口（沿用 R2）。
- primary 为中性灰（#606060）时，拾色平面不要显示饱和蓝；U2-source-overridden-detail 的平面仍是蓝色色相（沿用 R2）。
- 「Overridden only」写出分模式计数（沿用 R2；本轮看不到该控件）。
- 预览侧诊断条超过 3–4 对时折叠；U2-low-contrast 中有 5 行，占到预览首屏约 300px（沿用 R3）。
- 单列后有覆盖的格高 78px，左侧内容只占约 150px。可把「● Overridden · Light」放到色值同一行的右侧，格高降到约 50px，缩短整列滚动距离。
- 390 下滚动时收起头区、只保留 Edit/Preview 标签；当前吸顶区占 0–175px，约为视口的 21%（属 U1，沿用 R3）。
- 在精调区末尾也放一个「Restore document」入口（沿用 R3）。
- R3 建议「Borders &amp; focus 等没有配对的组只占左列」：已改为单列，**不再适用**。

## 5. 看不出的格子

| 截图 | 看不出什么 |
| --- | --- |
| 全部 U2 Colors 帧（source-overridden、-detail、token-released、source-generated、390） | 「Search colors…」与「Clear」那一行已被选中后的自动滚动移出视口，UI-160（36px 对 32px）和 UI-107 的占位符部分无法核实 |
| 全部 U2 帧 | 网格 ↺ 和来源块解除按钮的悬停、键盘焦点态，以及 aria-label、title（UI-115），都没有拍到 |
| U2-source-overridden-detail-1440-light | F4 拾色平面和 HEX 字段的焦点（picker 蓝 40%、3px、2px offset）没有聚焦帧；HEX 字段只露出顶边 |
| U2-low-contrast-1440-light | 点击「Locate」后能否定位到来源（F3「可定位来源」）没有拍到 |
| 全部 U2 帧 | F4「长 token 名换行」：本轮可见的最长名为 destructive-foreground，单列下没有出现需要换行的长名 |
| U2-field-error-1440-light | 「Preview keeps 0.35rem」只能从文案读出，预览侧圆角没有对照帧，无法逐像素确认未完成输入没有改变预览 |
| 矩阵 | 字段错误、低对比两态只有 1440；390 只采了来源态（目标包本来如此，不计缺失） |

## 6. 总评

U2 本轮得 9.4（R3 为 8.7）。R3 遗留的 17 条中，12 条已解决（UI-106、UI-107 为 U2 部分）。UI-18 部分解决；UI-172、UI-115 仍存在；UI-160 本轮无图、待核实。新增 4 条，都是 P3。

现在最主要的失分是**新-1**：两列改单列时，没有把「配对行」结构一起拆掉。结果是来源块挂在配对伙伴之后，配对内外间距倒置。这需要一次小的结构调整：把 token 列表拍平成直接子项，并按所选格插入来源块。只改样式修不好。其余都是局部修正：

- 分组解除的展开列表（UI-172）；
- Colors 页的标题样式统一（UI-18）；
- 390 吸顶条下分隔线和滚动目标（新-2）；
- 选中态不改字重（新-3）；
- ↺ 的可见或可访问名称（UI-115）；
- 诊断 summary 句式统一（新-4）。

照单修完，并补拍搜索行、↺ 焦点和拾色焦点帧之后，U2 预计能到 9.9 以上。
