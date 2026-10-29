# DOM-COMPONENTS-P1 UI 验收 R4 · 评审结论（合并）

本文件合并本轮 7 份评审原文，每单元一位全新评审。原文逐字保存在同目录：[judge-U1-a.md](judge-U1-a.md)、[judge-U2-a.md](judge-U2-a.md)、[judge-U3-a.md](judge-U3-a.md)、[judge-U4-a.md](judge-U4-a.md)、[judge-U5-a.md](judge-U5-a.md)、[judge-U6-a.md](judge-U6-a.md)、[judge-U7-a.md](judge-U7-a.md)。

合并规则：

- 每个单元只有一位评审，单元分即该评审的四维分之和，理由取其原文。
- 跨单元的旧差距（UI-106、UI-107、UI-160 涉及 U1、U2；UI-161 涉及 U1、U5）由各单元评审分别复核，结论不同时按 回归 > 仍存在 > 部分解决 > 待核实 > 已解决 > 不再适用 取最靠前的一项。
- 新差距从 UI-193 起编号，按单元、再按评审的临时编号顺序分配，不复用旧编号。
- 每条差距的完整取样数值、坐标和修法见对应原文，下文只摘要点。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 3.0 | 2.9 | 3.0 | 0.9 | **9.8** | 8.7 |
| U2 Studio 精调与来源解释 | 2.85 | 2.9 | 2.85 | 0.8 | **9.4** | 8.7 |
| U3 Studio 导入与导出 | 2.9 | 2.85 | 2.9 | 0.7 | **9.35** | 9.05 |
| U4 Studio 本地库迁移与存储反馈 | 2.9 | 2.95 | 2.85 | 0.85 | **9.55** | 8.6 |
| U5 Studio 预览 | 2.95 | 2.8 | 2.8 | 0.9 | **9.45** | 9.45 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.95 | 3.0 | 1.0 | **9.95** | 9.9 |
| U7 布局与排版 | 3.0 | 3.0 | 3.0 | 0.9 | **9.9** | 9.3 |

### 各维度理由

**U1**

- 布局 3.0：1440 顶栏与两栏左右对齐（Export 右边框与预览分隔线都止于 x=1415），768 顶栏两行、左右 16px，390 Edit/Preview 切换与「Back to edit」正常；200% 顶栏占 28%，渲染器控件右缘落在预览列内，导出对话框 Output 改为上下排列；状态胶囊与解除按钮间 8px，两栏标题中线差 0.5px。
- 视觉 2.9：分隔、次级文字、搜索框边框与占位符、选中三色、诊断色、200% 对话框次级按钮都对上 token。扣分：EN/中文、Edit/Preview 段间仍是两条线，与 Light/Dark 单线画法不同；中文态预览诊断标题汉字常规、拉丁字符半粗。
- 交互 3.0：键盘依次到达预设搜索、品牌色选择器、展开列表、Dark、Colors 标签页，焦点环清楚；Dark 焦点环内收，不再压邻段；保存胶囊按实际结果显示；200% 关键动作可达。
- 细节 0.9：R3 的复数、两种中文译法、「Light Light preview」、顶栏内边距与基线、搜索框与 Clear 不等高都已修好。扣分：分段接缝两种画法；中文诊断标题字重混排。

**U2**

- 布局 2.85：网格改为单列，状态标识不再折行；1440 下所选格与完整来源块同屏，390 下所选格完整露出；配置组状态行与按钮间距 8px。扣分：单列仍沿用两列时的配对行，来源块挂在配对伙伴之后，配对内间距 6px 大于配对间 4px；390 下「Semantic colors」组标题被吸顶条切掉上半部。
- 视觉 2.9：Typography、Geometry、Theme configuration 节标题统一为 14px/600 #1D2939；字体下拉值改回常规；状态标识与配置组同形；错误边框与焦点环符合 G3。扣分：「Fine adjustment source」与 Colors 页分组标题仍是 #475467；所选格的模式文字「Light」选中时变粗。
- 交互 2.85：两类来源及有效值、路径、「If cleared」都能读出；单项解除写明 Light only，不可解除的 ↺ 置灰；分组解除写明 both modes；Restore document 补了范围说明；`abc` 贴字段报错；低对比保留作者色并可定位。扣分：分组解除只给计数；网格 ↺ 仍只有图标。
- 细节 0.8：R3 的等宽「Light:」、重复计数、44px 空白、双线、键值结构都修好。扣分：配对内外间距倒置；390 组标题半截；选中格「Light」变粗；「Fonts · 3」带计数而同级「Text contrast pairs」不带。

**U3**

- 布局 2.9：所有对话框按钮行到下边框统一 24px；差异表表头下双线去掉，Change 列徽标与表头对齐。扣分：Import candidate 与 Changes 两张表上下叠放但列位置不同（Current 405 对 728，Imported 812 对 971，左缘 233 对 241）；Source 行比其他行矮 3px。
- 视觉 2.85：v1 损失清单和 Import notes 回到 14px/21px，token 与 G3 一致。扣分：同一对话框两套表头样式；6 个「Unchanged」与 1 个「Changed」徽标同样式；备注把整条路径排成等宽字。
- 交互 2.9：合法 v1 审阅、未知版本错误、v2/v1/CSS/Rust 导出、两个 CSS 下拉、复制成功都走通；候选区有「Field | Current | Imported」图例，Name 行两边都有值。扣分：「Theme JSON v1 · saved as v2」排在 Current 列下，像当前主题的属性。
- 细节 0.7：千分位、Rust 说明、CSS 选择器、ui-serif 断行、备注命名都已修好。扣分：差异表徽标基线低 2px；390 候选区徽标缩进 8px；Name 行两值基线差 1px；行距 29 对 32；备注路径的字体处理。

**U4**

- 布局 2.9：六种状态的存储 callout 位置与分隔统一；冲突键值改为上下两行；升级 Review 只剩一条分节线；390 页头按钮一行、callout 不溢出。扣分：冲突面板比 900px 视口高 35px，默认帧切掉「Download other version」；两处展开内容左缘不统一。
- 视觉 2.95：callout 与命名卡文字统一为两级（14px #1D2939 / 12px #475467）；对话框底部、无锁 callout 与 callout 内按钮样式统一；文字按钮与正文左缘对齐。扣分：配额态视口内三个实心蓝按钮抢主次。
- 交互 2.85：七个状态都能走通；取消后可下载当前主题；页头胶囊与 callout 区分「草稿已存、命名动作失败」；暂停与仅内存态页头 Save locally 降为描边；配额卡和升级后卡片有内联动作。扣分：配额态同一失败动作三个入口、三种叫法；取消态和无锁态命名卡的指引与按钮分在两张卡。
- 细节 0.85：R3 的文案问题都已修掉。扣分：callout 与命名卡成句重复；配额标题折行剩孤词「memory」且没写失败原因；「not saved earlier」措辞别扭。

**U5**

- 布局 2.95：拾取面板改为预览列底部整行，被拾取部件与面板互不遮挡；标签收进矩形内侧；工具条三态位置不变；两框首行横向内缩已对齐。扣分：两框上边框同在 y=335，首行大写字母顶部一个 357、一个 366，纵向差 9px。
- 视觉 2.8：深色预览与相反外观边界下编辑壳保持中性浅色；拾取面板浅蓝带 token 按钮；选中框蓝色双圈，未标注改灰色虚线；相反外观边界与其菜单正确变暗。扣分：相反外观（深色）边界内打开的对话框表面已是深色 #1D2939，Confirm 仍是根主题浅色 primary #1570EF 底、白字（R3 同帧为 #53B1FD 底、#0C111D 字）。
- 交互 2.8：菜单、对话框、相反外观及其浮层、对比区 Dark、DOM + GPU、拾取（悬停、点击后焦点进编辑器、未标注说明）都走通，标签不再盖住相邻控件。扣分：P3「局部浮层随创建处的有效主题更新」只部分成立，对话框主按钮取根主题，R3 时正确，本轮回归。
- 细节 0.9：预览模式改为「Preview: Light / Dark」；R3 的断词、切下伸部、分隔符、按下态横移都消失。扣分：对话框 Confirm 由 77×33 变为 79×35，同预览 solid 按钮为 32px 高；卡片选中框内圈在滚动区边缘画出闭合底边。

**U6**

- 布局 3.0：四档高度 24/32/40/48，110% 下 26/35/44/53；IconButton 32×32；loading 宽度不变；full 圆角与 390 换行正确；classic 行本轮只改明暗，轮廓逐像素不变。
- 视觉 2.95：UI-81 已解决，上缘高光四行 +80/+20/+11/+4、两侧亮边 +18/+4，与目标差 ±2 以内。扣分：本轮修 UI-81 时深色 classic 底边暗线变浅，压暗幅度从约 −31% 变为约 −20%（目标约 −28%）。
- 交互 3.0：焦点特写除 classic 外与 R3 字节相同，B2 各项复量符合；loading、Unavailable、Link 聚焦隐藏下划线、110%、full、中文都可用。
- 细节 1.0：ThemePanel 中文文案逐处比对无差异。

**U7**

- 布局 3.0：Container 四档宽度与三种对齐正确；Inset 各 side 只消去最近容器声明的 padding，side=left/right 改为并排、四种状态下不再相压；Reset 的 input、textarea 回到值列；显式尺寸在缩放下不变。
- 视觉 3.0：Heading、Strong 粗体；Text 四档墨量递增；字级梯度、行高、Code、Kbd、Blockquote 与目标逐项对应。
- 交互 3.0：110%、95% 只缩放一次；Invalid prop 诊断带属性定位、拒收值与回退值，且不扰动其他区域；wrap、truncate、trim 各档可辨。
- 细节 0.9：Reset 的 input、textarea 虚线标记越过内容右沿约 5px；240px 样例正文贴左边框。

## 上一轮差距复核结果

R3 judge.md 列出的 63 条评审来源差距，本轮结果如下（逐条依据见各单元原文的复核表）：

- **仍未关闭（11 条）**：
  - 部分解决 7 条：UI-02、UI-18、UI-128、UI-176、UI-25、UI-56、UI-189；
  - 仍存在 3 条：UI-172、UI-115、UI-76；
  - 待核实 1 条：UI-160。
- **已解决（52 条）**：其余各条。
- 跨单元的旧差距：
  - UI-106、UI-107：U1、U2 两位都判已解决（各自可见部分）；
  - UI-160：U1 判已解决（U1-field-tabs-1440-light 搜索框与 Clear 同为 y=536–567），U2 判待核实（U2 各帧中搜索行已被选中后的滚动移出视口），按顺序取待核实；
  - UI-161：U1、U5 都判已解决。
- 判已解决但带出新问题的：UI-17 → UI-195；UI-51、UI-140 → UI-201；UI-81 → UI-210；UI-163 的单列修法 → UI-194；UI-188 的副作用只记为建议。

## 未关闭差距

### U1

#### UI-02 · [P3] EN/中文、Edit/Preview 段间仍画两条线，与 Light/Dark、DOM/GPU 的单线画法不一致（U1 · 视觉、细节 · 部分解决 · 可挽回 0.1）

- 位置：U1-default-1440-light y=85 x=250 #84CAFF、x=251 #D0D5DD；U1-zh-top-1440-light 反之；U1-edit-390-light、U1-preview-390-light y≈140–145 x=61/62 同样。对照：Light/Dark 接缝只有 x=993 一列，390 渲染器接缝也是单列。焦点部分已解决（Dark 焦点环内收）。
- 修法：EN/中文与 Edit/Preview 复用 Light/Dark 的分段组件；或非首段 `margin-inline-start:-1px`、选中段 `z-index:1`。
- 来源：U1 UI-02（部分解决）。

#### UI-193 · [P3] 中文态预览诊断标题字重混排：汉字常规，数字与拉丁字符半粗（U1 · 视觉、细节 · 可挽回 0.1）

- 位置：U1-zh-top-1440-light、U1-zh-configured-1440-light，「▼ 2 组文字对比低于 4.5:1 · 作者颜色保持不变」x≈357–622、y≈210–222；英文同元素整句半粗，中文壳层标签能正常加粗。
- 期望：G6 中英文同步；G3 label 600。
- 修法：诊断条标题用与壳层标签相同的字体栈与 CJK 回退，确保回退字体有 600 字面或允许合成粗体；不继承预览作用域的作者字体。
- 来源：U1 新-1。

### U2

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

### U3

#### UI-128 · [P2] Source 行的值排在「Current」列下，读起来像是当前主题的属性（U3 · 交互、细节 · 部分解决 · 可挽回 0.1）

- 位置：U3-import-review-1440-light 行 y≈627–636，「Theme JSON v1 · saved as v2」从 x=405（Current 列）起排，Imported 列为空；390 单列卡片无此歧义。图例「Field | Current | Imported」与 Name 行两侧值已补上。
- 期望：X1 版本清楚归属导入候选。
- 修法：Source 行 Current 格写当前文档格式、Imported 格写「Theme JSON v1 · saved as v2」；或把 Source 改为表格上方跨列的说明行「Imported file: Theme JSON v1 · will be saved as v2」。中文同步。
- 来源：U3 UI-128（部分解决）。

#### UI-198 · [P3] 同一对话框里两张差异表的列结构、列位置和表头样式不一致（U3 · 布局、视觉、细节 · 可挽回 0.15）

- 位置：U3-import-review-1440-light y 595–607 候选区表头 12px 常规、无底线，列 x 233/405/812；U3-import-review-bottom-1440-light y 252–274 Changes 表头 13px/600、#D0D5DD 底线，列 x 241/572/728/971，另有独立 Change 列；390 小标题字重也不一。
- 期望：G3 label 13px/600；X1 同一套结构。
- 修法：候选配置行改用 Changes 的表格组件，两表共用 `Field | Change | Current | Imported` 列定义与 `th` 样式；或并成一张表，「Identity & configuration」作第一个分组行。
- 来源：U3 新-1。

#### UI-176 · [P3] 徽标基线偏低 2px；390 候选区徽标缩进 8px；Name 行两边基线差 1px（U3 · 布局、细节 · 部分解决 · 可挽回 0.14）

- 位置：U3-import-review-bottom-1440-light id / name / primary 行文字与徽标基线 332/334、373/375、451/453；U3-import-review-390-light Name 行徽标 x=207，同列值在 x=199；U3-import-review-1440-light Name 行两值基线 665/666。徽标列对齐与表头双线已修好。
- 修法：徽标 `inline-flex` 居中、固定高 24px；同行 `td` 统一 `vertical-align`；值与徽标包进 `flex-wrap` 容器，去掉徽标 `margin-left`。
- 来源：U3 UI-176（部分解决）。

#### UI-199 · [P3] 6 个「Unchanged」与 1 个「Changed」徽标同样式，且左缘参差（U3 · 布局、视觉、交互 · 可挽回 0.1）

- 位置：U3-import-review-1440-light y 651–866、U3-import-review-mid-390-light y 33–346；两种徽标同底同边同字色，Unchanged 左缘 456/461/451 三种 x。
- 期望：X1 突出候选差异。
- 修法：状态放进独立 Change 列并左对齐；「Unchanged」改纯文字 caption（或「—」）；只有 Changed/Added/Removed 保留带边框徽标，不染蓝。
- 来源：U3 新-2。

#### UI-200 · [P3] Import notes 把整条路径排成等宽字，与表格写法不一致（U3 · 视觉、细节 · 可挽回 0.08）

- 位置：U3-import-review-bottom-1440-light y 764–794、-390 y 664–740，「Light · base · font-serif」整段等宽，比同行 14px 正文小一号；表格写成无衬线分组头加等宽 token。
- 修法：模式与分组用正文字体，只有 token 用 `code`，`code` 字号 .9286em。中文同步。
- 来源：U3 新-3。

#### UI-25 · [P3] Source 行比其余候选行矮 3px（U3 · 布局、细节 · 部分解决 · 可挽回 0.06）

- 位置：U3-import-review-1440-light label 大写字母顶边 596/628/657/689/720/752/784/816/848，只有 Source→Name 为 29px。
- 修法：候选行 `min-height: 2rem` 或 `grid-auto-rows: minmax(2rem, auto)`，不靠徽标撑高；并表后统一 `td` 高度。
- 来源：U3 UI-25（部分解决）。

### U4

#### UI-201 · [P2] 配额态：同一个失败的命名保存有三个重试入口、三种叫法，加上页头一共三个实心蓝按钮（U4 · 交互、视觉 · 可挽回 0.15）

- 位置：U4-quota-failure-1440-light 页头「Save locally」实心；红色 callout「Retry save」实心（y≈472–503）；命名卡「Retry retained saves · “Retained name”」实心（y≈674–706）；操作区「Retry retained saves」描边（y≈809–840）。此时只有 1 个 pending。
- 期望：S4 命名库动作单独反馈；S1 一个明确的继续动作；状态 callout 只管存储层。
- 修法：callout 去掉「Retry save」，由「Download current theme」做唯一实心按钮；命名卡按钮叫「Retry “Retained name”」；操作区改「Retry all retained saves」（描边）；页头在失败态降为描边。
- 来源：U4 新-1（评审注明由 R3 UI-51、UI-140 两条修法叠加造成）。

#### UI-203 · [P3] callout 和命名卡之间成句重复（U4 · 细节 · 可挽回 0.1）

- 位置：U4-upgraded「All saved themes and your current draft were retained.」后紧跟「2 saved themes retained. …」，命名卡「not saved earlier.」；U4-no-locks 标题、正文、命名卡三处同一意思；U4-quota-failure 标题与正文末尾都写「new saves stay in memory」。
- 修法：各处只说一次，按原文给出的改写；无锁命名卡可按 UI-205 并入 callout。
- 来源：U4 新-3。

#### UI-202 · [P3] 配额 callout 标题折行后第二行只剩孤词，也没有写出失败原因（U4 · 细节 · 可挽回 0.05）

- 位置：U4-quota-failure / U4-quota-operations 标题「Local save failed · new saves stay in」/「memory」（y=344/362）；正文「A later library action failed」。
- 期望：S6 明确原因；S4 点名失败的动作。
- 修法：标题改「Storage full · saving in memory」；正文点名「Saving “Retained name” failed and is kept for retry.」；标题加 `text-wrap: balance`。
- 来源：U4 新-2。

#### UI-204 · [P3] 冲突面板比 900px 视口高，默认帧切掉了「Download other version」（U4 · 布局 · 可挽回 0.05）

- 位置：U4-conflict-1440-light 面板从 y=323 起，「View stored text」在 y=867，下一个按钮只露顶边；底边对齐需 `scrollTop 35→335`（R3 为 `0→300`）。
- 期望：S1 出口始终可达；S5 展示冲突版本。
- 修法：首句与第一段补充说明并为一句；删去单独的「No pending actions…」段，有 pending 时再用一行 caption。
- 来源：U4 新-4。

#### UI-205 · [P3] 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里（U4 · 交互 · 可挽回 0.05）

- 位置：U4-upgrade-cancelled 命名卡（y≈580–711）只有「Dismiss」，正文让用户先 review upgrade；U4-no-locks 命名卡（y≈491–622）只有「Dismiss」，正文让用户 download。
- 期望：S1 每条常驻反馈带继续动作；S4。
- 修法：命名失败原因与 callout 状态相同时，并入 callout 末尾一行「Not saved: “…” · Dismiss」；或卡里放对应的描边按钮。
- 来源：U4 新-5。

#### UI-56 · [P3] 操作展开内容：说明段没有缩进，列表已缩进（U4 · 布局 · 部分解决 · 可挽回 0.03）

- 位置：U4-quota-operations 说明段（y≈805–841）从 x=21 起，列表从 x=34 起；#3 行在折线以下，修订号断行看不出。
- 修法：details 的内容容器整体缩进到摘要文字起点；修订号片段 `white-space:nowrap`。
- 来源：U4 UI-56（部分解决）。

#### UI-206 · [P3] 草稿差异块有三条左缘（U4 · 布局 · 可挽回 0.02）

- 位置：U4-conflict-diff-1440-light，x=33/34（callout 正文、三角、分隔线）、x=42（差异行）、x=49（摘要文字）。
- 修法：差异行去掉 8px `padding-inline`，与分隔线同起点；或整体缩进到 x=49。与 UI-56 用同一规则。
- 来源：U4 新-6。

### U5

#### UI-207 · [P1] 深色局部边界内打开的对话框：表面随边界变深，主按钮 Confirm 仍取根主题浅色 primary（回归）（U5 · 视觉、交互与状态 · 可挽回 0.4）

- 位置：U5-local-opposite-dialog-1440-light 对话框表面 #1D2939，Confirm（x 233–311、y 118–152）为 #1570EF 底、#FFFFFF 字。对照：同一边界内 Actions 文字 #53B1FD；U5-dom-1440-dark Create profile 为 #53B1FD；R3 同帧 Confirm 为 #53B1FD 底、#0C111D 字；浅色根下 U5-dialog-1440-light 的 #1570EF 是正确的。
- 期望：P3 局部浮层随创建处的有效主题更新；对话框正文本身写「This layer follows the theme where it was opened.」；B6 solid Button 消费有效 primary/foreground。
- 修法：Dialog 的 portal 容器带上创建处 ThemeBoundary 的完整作用域（含 Button solid 用到的 primary、前景、第 9/10 阶与 contrast 变量）；Button 颜色从最近作用域取变量，不在挂载时固化；对照 R3 与 R4 之间对 Dialog、Button、ThemeBoundary、portal 的改动；补一条「相反外观边界内浮层 Confirm computed 背景等于边界内 solid 按钮」的浏览器回归。
- 来源：U5 新-1。

#### UI-76 · [P3] DOM 与 GPU 两框首行纵向差 9px（横向已对齐）（U5 · 布局 · 仍存在 · 可挽回 0.05）

- 位置：U5-gpu-compare-1440-light 两框上边框都在 y=335，DOM 首行大写字母顶部 y=357，「GPU · Cards」y=366；横向内缩已对齐（26 对 25–26px）。R3 时纵向对齐、横向差 8px。
- 修法：两框用同一 padding token，按首行基线对齐调整 GPU 场景根 `padding-top`，不动左右边距。
- 来源：U5 UI-76（评审判仍存在：横向已解决，纵向回归）。

#### UI-208 · [P3] 预览对话框 Confirm 为 79×35，与同一预览里 32px 高的 solid 按钮不一致（R3 为 77×33）（U5 · 细节 · 可挽回 0.05）

- 位置：U5-dialog-1440-light、U5-local-opposite-dialog-1440-light Confirm x 233–311、y 118–152；U5-dialog-390-light 79×35；文字墨迹两轮都是 51×11。对照 Create profile 115×32。
- 期望：P1 真实组件与尺寸；P1 Button size 2 参考高 32px。
- 修法：对话框操作按钮用与 Create profile 相同的 Button size 2，去掉多出的 1px；与 UI-207 一起排查。
- 来源：U5 新-2。

#### UI-209 · [P3] 卡片选中框内圈在滚动区边缘画出闭合底边，看上去像卡片在 y=720 处结束（U5 · 细节 · 可挽回 0.05）

- 位置：U5-inspector-card-1440-light 外圈向下延伸到滚动区边缘不闭合，内圈在 y 719–720 画出底边（x 369–867）；该卡实际延伸到 y≈787。
- 期望：P2 选中矩形如实框出实际部件。
- 修法：两圈都按部件完整矩形绘制，覆盖层放进预览滚动容器并 `overflow: clip`；或钳制时去掉超出一侧的描边，内外圈同样处理。
- 来源：U5 新-3。

### U6

#### UI-210 · [P3] 深色下 classic 底边暗线变浅（UI-81 修复引入的回归）（U6 · 视觉 · 可挽回 0.05）

- 位置：U6-fixture-1280-dark classic 1–4 底边，如 classic 4 x=490/540、y=184；fixture-390、scale110、radius-full 的深色渲染同一规则。末行 #425AAA 压在 #4F70E0 上，亮度 113 → 91（约 −20%）；目标 classic 4 末行 #584B90 压在 #7C67D4 上（约 −28%），R3 为约 −31%。浅色不受影响。
- 期望：radix-sink-button.png、radix-classic-button.png classic 行末行压暗约 28%。
- 修法：只改 classic 深色阴影层，让侧边亮边不作用到底边（或把底部暗线排到 `box-shadow` 最前）；恢复末行 ≈ 上一行亮度 × 0.72；保住已对齐的上缘与两侧增量；改完 `mbx xtask css`。
- 来源：U6 新-1。

### U7

#### UI-189 · [P3] Reset 的 input、textarea 虚线标记越过内容右沿（U7 · 细节 · 部分解决 · 可挽回 0.05）

- 位置：U7-fixture-1280-light Reset 区两行虚线框右缘 x=1229，Container 分隔线止于 x=1224；U7-fixture-390-light 右缘 x≈363（内容右沿约 358）；U7-layout-reset-comparison-1280/390 元素截图里右边线被裁掉。脱离值列已修复。
- 期望：L5 容器边界不遮住内容；比较口径中 Reset 的作用范围。
- 修法：值列单元格加 `padding-inline-end: 5px`，或块级 Reset 元素的标记改 `outline-offset: -1px`；不加包裹层（L4）。
- 来源：U7 UI-189（部分解决）。

#### UI-211 · [P3] 240px 显式宽度样例的正文贴着左边框（U7 · 细节 · 可挽回 0.05）

- 位置：U7-fixture-1280-light 第 1628–1651 行，「A bad prop keeps this view alive.」首字距左边框约 1px，标注与盒顶 2–3px；U7-invalid-detail-1280、95%、110%、390 帧同样。
- 修法：样例 Box 加显式 `padding: 2px 8px`、`box-sizing: border-box`，标注与盒之间 4px。
- 来源：U7 新-1。

## 建议（不扣分）

摘自各评审原文的建议节，详见原文。

- U1：
  - 「Download current theme」本轮改为实心，与顶栏「Save locally」两个实心蓝按钮同屏；只保留一个实心主按钮。
  - 存储状态块与顶栏胶囊重复且本轮变高，「Presets」从 y=550 下移到 y=618，可压成一行摘要加展开（沿用 R3）。
  - 两个 18px 标题字重不同；保存胶囊加前导标识；品牌色、中性色选项加色块；加跳到预览工具栏的链接（Tab ×115）；200% 导出对话框标题行吸顶（沿用 R3）。
  - 帮助文字 13px 与 12px 两档并存，统一字级；帮助文字加 `text-wrap: pretty` 避免孤字。
  - 200% 下「DOM + GPU」段内折成两行，每段 `nowrap`、不够时改网格；200% Colors 网格色块与解除列不随根字号缩放；Path 在「.」后加 `<wbr>`。
  - 壳层按钮间距 5/6/8/16/20px 混用，统一 gap token；预览工具栏切换类按钮半粗而选择器与动作按钮常规，按类统一字重。
  - 壳层焦点环对白底约 1.76:1，DESIGN.md 再修订时可提高不透明度。
  - 「Overridden · Light 6 · Dark 6」的数字补名词。
- U2：
  - 来源块给出另一模式的状态并附分组解除入口；primary 为中性灰时拾色平面不显示饱和蓝；「Overridden only」写分模式计数（沿用 R2）。
  - 预览诊断条超过 3–4 对时折叠；390 滚动时收起头区；精调区末尾也放「Restore document」（沿用 R3）。
  - 有覆盖的格高 78px，可把状态标识放到色值同一行右侧，格高降到约 50px。
  - R3 的「无配对组只占左列」已改单列，不再适用。
- U3：
  - JSON 导出 2 空格缩进、代码框 `white-space: pre` 加横向滚动，390 下键名与字符串中间断行明显（沿用 R2、R3）。
  - 「Copied to clipboard.」出现后对话框长 26px，给状态行预留高度或放进按钮行。
  - v1 损失清单按 X2 列举顺序排列。
  - 对话框标题吸顶、Apply/Cancel 吸底、Cancel 改描边；Rust 页「Download」改名「Download theme.rs」、0.2.1 拒收单独警示、补普通 Cargo 命令；差异颜色值旁加色块；「To match this preview」后标出预览模式；对话框底部渐隐（沿用 R3）。
- U4：
  - 页头状态胶囊按语义着色。
  - 升级 Review：ID 列左对齐；分节标题与字段标签拉开层级；展开 JSON 后标题行吸顶（顶边切掉「Saved themes retained (2)」一半）；× 与「Cancel upgrade」语义区分。
  - 冲突：推荐动作放首位或末位；差异颜色值旁加色块；差异 ≤3 条时默认展开。
  - callout 内外展开摘要字号统一；成功 callout 加「Dismiss」；取消态「Download original data」改名「Download pre-upgrade data」。
- U5：
  - 简单对话框限宽 28–32rem 并垂直居中；菜单与触发器留 4px 并加阴影；局部边界标题升级、外观选择器限宽；GPU 色样标签不在连字符处断开；Host control 加说明；深色局部对话框遮罩取根主题或中性深色；面板补色阶信息、提供折叠入口；390 Preview 页粘性页头收成一行（沿用 R3）。
  - 拾取面板高度随内容变化 40px，设 `min-height`；滚动区与面板之间留 8px 或加淡出阴影。
  - UI-188 的修法让同一行按钮字重不一（切换类半粗、选择器与「Back to edit」常规），统一字重并用伪元素预留宽度。
  - 面板标题与正文同样式，面板色值用比例字，可按 G3 统一。
- U6：
  - 夹具补 high-contrast、gray、gray + high-contrast、disabled 各列，IconButton size × variant 矩阵，各色 classic 与 classic high-contrast，Link 1–9 字级与 underline 四模式；补「文字 + 图标」Button；「Caller utilities」改可读配色；补拍 Unavailable 激活反馈、ThemePanel 覆盖态、auto/hover 链接悬停帧（沿用 R3）。
  - 「High contrast」按钮实为 blue 色族，标签写明或另加不设 color 的 accent high-contrast。
  - 补拍深色 loading 帧与深色 classic 的 DPR 2 焦点特写。
- U7：
  - Heading/Text/Blockquote 补色族与 high-contrast 行；Em 改衬线斜体；Kbd 补多字符键、顶行键距加大；Container 加嵌套样例、说明 `margin=3px`；「Your name」与输入框、Code/Kbd 说明与样例之间留 8px；Code 表行高收紧；Wrap 与 Trim 组间距统一；夹具控制按钮显示当前状态；table 的 Reset 对照换样例；「Box · 160 × 90px」盒子靠近标注；Code 表头 1280 与 390 写法统一（沿用 R3）。
  - Reset 对照的标签「blockquote · native」等折成两行、行尾悬「·」，加宽标签列或 `nowrap`。
  - Reset 对照多行值行的标签与值对齐方式统一。
