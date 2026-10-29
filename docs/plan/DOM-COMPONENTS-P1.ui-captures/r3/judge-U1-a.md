# U1 评审 · R3（Studio 工作台框架与基础配置）

依据：目标包规范要点 G1–G6、C1–C4，以及变更记录（R2 之后的变更没有改动 U1 的期望）；本轮 `r3/U1-*.png` 共 21 张和 `r3/U1-actions.log`；上一轮的 `r2/U1-*.png` 和 `r2-judge-U1.md`。像素值是在 DPR 1 截图上取样得到的（ImageMagick），坐标单位为 CSS px。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.6 | 2.8 | 2.7 | 0.6 | **8.7**（R2：7.8） |

- **布局 2.6**
  - 已到位：
    - 顶栏横跨两栏，各视口下都有主题名、保存状态、预览模式、Undo/Redo/Save/Import/Export（G5）；
    - 编辑列 319px + 1px #D0D5DD 分隔线；两区独立滚动；
    - 1440 下编辑区内边距 24/20，预览区 24/24；768 下 DOM/GPU 上下堆叠，编辑、预览水平内边距 16，主标题 16px；
    - 390 下有 Edit/Preview 分区和「Back to edit」，两层固定区之间不再透出内容。
  - 不是 3 分的原因：
    - 200% 下渲染器分段控件越出预览列，右缘被视口截掉；导出对话框的 Output 值被挤成「Theme…」；
    - 顶栏在 768 多占一行，在 200% 下固定占视口 39%；
    - 状态胶囊与解除按钮之间 0px。
- **视觉 2.8**
  - 已到位：分隔 #D0D5DD/#EAECF0、次级文字 #475467、搜索框边框 #98A2B3、选中 #EFF8FF/#84CAFF/#175CD3、错误 #B42318；label 和选中标签页为 600；状态胶囊改为中性；按钮与禁用态统一；焦点环统一为单一 token。
  - 不是 3 分的原因：
    - 200% 导出对话框里仍是旧的蓝边蓝字次级按钮（#8AB7F7/#1570EF），选择器边框是非 token 的 #858F9E；
    - 「Light N · Dark N」计数仍为 #667085；
    - 分段控件有两种接缝画法。
- **交互与状态 2.7**
  - 已到位：键盘依次到达预设搜索、品牌色选择器、展开的列表、Dark、Colors 标签页，焦点都清楚；保存胶囊按实际结果显示（Not saved yet / Saved on this device）；页头写明「Matches/Modified from “X”」；390 预览页可以切模式、保存、导出；200% 下所有关键动作可见；中文完整。
  - 不是更高一档的原因：
    - Dark 的焦点环压进相邻的 Light 段约 5px；
    - 200% 下看不出当前选中的导出格式；
    - 200% 下渲染器控件被截。
- **细节 0.6**：有若干处小差异：
  - 编辑列「Diagnostics · 1 text pairs」复数错误；
  - 中文「已保存到本机 / 已保存到此设备」两种译法；
  - 预览标题旁读成「Rustify Light Light preview」；
  - 顶栏左右内边距与两栏差 4px，主题名与说明不在同一基线；
  - 颜色搜索框 36px 与 Clear 32px 并排。

## 2. 上一轮差距复核表

| 编号 | R2 严重度 | 本轮状态 | 依据（R3 图与位置） |
| --- | --- | --- | --- |
| UI-101 200% 粘性头封顶、关键动作不可见 | P1 | **已解决** | U1-zoom200-720：头区不再内滚动。「Preview mode」完整，Light/Dark、Undo/Redo、Save locally/Import/Export 全部可见，底边在 y=348。残留的高度问题并入 UI-103 |
| UI-102 390 标签条与头区之间透出内容 | P2 | **已解决** | U1-configured-390：y=153–216 纵向逐列扫描，只有 y=152 的 #D0D5DD 和内容自身 y=217 的 #EAECF0 分组线，没有透出内容 |
| UI-103 吸顶区过高 | P2 | **部分解决** | 1440 下顶栏 0–56，占 6.3%，已解决。768 下 0–160（15.7%）分三行，第 2、3 行右半空置；200% 下 0–348 固定（38.8%，导出帧遮罩后编辑列已滚动而 y=348 的线不动） |
| UI-02 未选分段蓝内框、双线、焦点压邻段 | P2 | **部分解决** | 蓝内框和下划线已去掉；Light/Dark、DOM/GPU 段间是单条 1px。残留两处：U1-keyboard-04 中 Dark 的焦点环左缘落在 x≈988–990，压进 Light 段；EN/中文、Edit/Preview 段间是 #84CAFF + #D0D5DD 两条线，与 Light/Dark 的单线不一致 |
| UI-104 次级/禁用按钮 4 套样式 | P2 | **部分解决** | 壳层已统一：次级按钮为白底、#D0D5DD 边、#475467 字；禁用为 #F9FAFB 底、#EAECF0 边、#98A2B3 字（Undo、Redo、Restore、Cancel、Retry、Clear 都已取样核对）。残留在 U1-zoom200-export-end：「Select text」「Download」为 #8AB7F7 边、#1570EF 字；U1-zoom200-export 的 Output 选择器边框为 #858F9E |
| UI-09 label、预览标题、选中标签页没有呈现 600 | P3 | **已解决** | U1-configured-1440 中 Appearance、Brand color 等，U1-default-1440 的「Preview」，U1-field-tabs 的「Colors」都是半粗 |
| UI-105 同行控件 28/36/38 三档 | P3 | **已解决** | 原位置全部为 32px：Light/Dark、Undo、Save 在 y=12–43；Cards、DOM、Inspect、Focus preview 在 y=117–148；EN/中文在 y=81–112。新出现的同类问题另记为新-4 |
| UI-106 编辑区顶部 padding、两栏标题、状态行与按钮间距 | P3 | **部分解决** | 编辑区首行 y=81，距顶栏 24px，已对；两栏标题中线差从 8px 降到约 4.5px（「R」大写在 91–103，「P」在 87–98）。U1-configured-1440 中 Overridden 胶囊底边 y=488 与「Use configuration values」按钮顶边 y=489 仍然相接 |
| UI-107 次级文字 #667085、颜色搜索框边框 | P3 | **部分解决** | 帮助文字、「Resolved neutral」已是 #475467；「Search colors…」边框已是 #98A2B3。「Light 6 · Dark 6」「Light 2 · Dark 2」仍为 #667085；两个搜索框占位符颜色不同（#667085 与约 #8E949C） |
| UI-108 头区下方双细线 | P3 | **已解决** | U1-default-1440、768、zh-top、zoom200：顶栏底线下方的第一个元素不再带上边框 |
| UI-109 状态胶囊借用选中蓝 | P3 | **已解决** | Overridden、Custom 胶囊为 #F9FAFB 底、#D0D5DD 边、#475467 字，保留前导圆点 |
| UI-110「1 text pairs」 | P3 | **部分解决** | 预览诊断条已改为「1 text pair below 4.5:1」（U1-default-1440 y=208）；编辑列底部 U1-keyboard-04 y≈783 仍是「Diagnostics · 1 text pairs below 4.5:1」 |
| UI-111 768 主题名比主标题大 | P3 | **已解决** | 768 下主题名和应用名的字面都按 16px 计（墨迹宽 104 和 146，按 1440 下 114/163 的 16/18 比例缩放） |
| UI-112 行尾悬挂分隔点 | P3 | **已解决** | 「Overridden」胶囊后不再有点；200% 下「Rustify Light / Light preview」两行没有悬挂点。去掉分隔点后的新问题另记为新-5 |
| UI-113 焦点环多色 | P3 | **已解决** | 预设搜索、品牌色选择器、Dark、Colors 标签页的焦点环都是 #A1C6F9（#1570EF 40% 叠在白底上），3px，留 2px 间隙 |
| UI-114 Modified 的比较基准不清，吸顶区没有保存状态 | P3 | **已解决** | 顶栏为「Matches “Rustify Light”」「Modified from “Graphite”」加「Not saved yet / Saved on this device」胶囊，与编辑列的存储状态块一致 |
| UI-03 390 预览页没有模式与保存/导出 | P3 | **已解决** | U1-preview-390 顶栏含 Preview mode Light/Dark、Undo/Redo、Save locally/Import/Export |
| UI-05 200% 导出按钮固定 32px | P3 | **已解决** | U1-zoom200-export-end 中 Copy code 高约 64px（y 659–722），随字号放大 |
| UI-12 自动中性色名小写且未翻译 | P3 | **已解决** | 「Resolved neutral: Mauve.」「已解析中性色：紫灰色。」「Resolved neutral: Slate.」 |

## 3. 差距清单（未关闭，按严重度、再按可挽回分排序）

### 新-1 · [P2] 200% 下渲染器分段控件（DOM / GPU / DOM + GPU）宽于预览列，右缘被视口截掉
- **单元与截图**：U1 · U1-zoom200-720-light 的预览工具栏第三行（y≈655–716）。
- **维度**：布局、交互。
- **现状 → 期望**：
  - 现状：分段控件从 x=352 起一直到视口边缘 x=719（y=670 整行都是 #EFF8FF），没有右边框，右圆角被切掉。预览列内容区按 2rem 内边距算，应在 x≈688 结束。R2 同帧的分段控件在 x≈697 结束、能完整显示；本轮控件统一到 2rem 高、横向内边距加大之后才溢出，属于回归。
  - 期望：G6「200% 放大下主路径可达，不依靠整页横向滚动到达关键动作」；评分口径要求没有溢出和截断。
- **修法**：给分段控件组件加 `max-width:100%`，每段 `flex:1 1 auto; min-width:0`，允许段内文字换行（`white-space:normal`）；或者给预览工具栏设 `container-type:inline-size`，用 rem 写容器查询（`@container (max-width: 22rem)`），窄时把三段改成 `grid-template-columns:repeat(3,minmax(0,1fr))`，再窄就改成同样带 label 的 `<select>`。另外确认预览列网格项有 `min-width:0`，不被子项撑宽。
- **可挽回分**：0.2（布局 0.15、交互 0.05）。

### 新-2 · [P2] 200% 下导出对话框的 Output 选择器被压窄，当前值截成「Theme…」
- **单元与截图**：U1 · U1-zoom200-export-720-light 的对话框「Output」行（选择器在 x≈426–639、y≈156–226）。
- **维度**：布局、交互与状态、细节。
- **现状 → 期望**：
  - 现状：选择器只有约 213px 宽，值显示为「Theme…」，看不出当前是 Theme JSON v2、兼容 v1 还是 CSS。R2 同帧显示完整的「Theme JSON v2」，选择器约 342px 宽，属于回归。
  - 期望：G6「菜单和对话框在可用视口内可操作」；X6 窄时表单行单列；用户在复制前要能读出导出格式（X2 区分 v2 与兼容 v1）。
- **修法**：对话框表单行用 `display:flex; flex-wrap:wrap; gap:.5rem 1rem`，选择器设 `flex:1 1 14rem; min-width:14rem`，宽度不够时整行换到 label 下方占满宽度；或者按对话框宽度（`container-type:inline-size`，`@container (max-width: 36rem)`）切成 label 在上、选择器在下的单列，不要只依赖 `≤560px` 的视口断点。
- **可挽回分**：0.2（布局 0.1、交互 0.05、细节 0.05）。

### UI-02 · [P3] 分段控件残留：焦点环压到相邻段，两种接缝画法并存（部分解决）
- **单元与截图**：U1 · U1-keyboard-04-1440-light 顶栏 Light/Dark（Dark 焦点环左缘 x≈988–990 落进 Light 段，Light 段范围 x=942–993）；U1-default-1440-light 的 EN/中文（x=250–251 为 #84CAFF + #D0D5DD）；U1-edit-390、U1-preview-390 的 Edit/Preview（同样是两条线）。
- **维度**：交互、视觉。
- **现状 → 期望**：
  - 现状：Dark 的 3px 焦点环外扩 2px，盖住 Light 段右侧约 5px。Light/Dark、DOM/GPU 段间是单条 1px 线，EN/中文、Edit/Preview 段间却是蓝灰两条线。
  - 期望：G4 新增控件焦点可见、选中段用按下态；G3 蓝色只用于选中与焦点，壳层分隔为单一细线。
- **修法**：
  - EN/中文、Edit/Preview 改用顶栏 Light/Dark 的同一个分段组件：相邻段 `margin-inline-start:-1px` 合并边框，选中段 `position:relative; z-index:1`，让接缝只剩一条 #84CAFF。
  - 分段内的段在 `:focus-visible` 时用内收焦点环（`outline-offset:-3px`，颜色仍为 #1570EF 40%）；或者焦点段 `z-index:2`，同时 `outline-offset:0`，并给组容器留 `padding:2px`，让焦点环不进入相邻段。独立的搜索框和选择器保持现在的 2px 外偏移。
- **可挽回分**：0.15（交互 0.1、视觉 0.05）。

### UI-103 · [P3] 顶栏在 768 多一行，在 200% 固定占视口 39%（部分解决）
- **单元与截图**：U1 · U1-default-768-light、U1-configured-768-light 顶栏 y=0–160：第 1 行是主题名、说明、胶囊，右侧上下叠放 Preview mode；第 2 行只有 Undo/Redo（y=76–107）；第 3 行只有 Save/Import/Export（y=116–147），两行的右半都空着。U1-zoom200-720-light 顶栏 y=0–348 共 5 行，右侧 x>485 的第 3–5 行空置。U1-zoom200-export*：遮罩后编辑列已滚动，y=348 的顶栏底线不动，说明顶栏固定。
- **维度**：布局。
- **现状 → 期望**：两区可用高度在 768 下少约 40px，在 200% 下只剩 552px → G2「两区独立滚动」；G5 只要求顶部保留主题名、模式与撤销/重做/保存/导出。
- **修法**：
  - 把 Undo/Redo 组和 Save/Import/Export 组放进同一个 `flex-wrap:wrap; gap:.5rem 1.5rem` 的动作行，768 下压成两行，总高约 104px；
  - 200% 下让「Not saved yet」胶囊与「Matches…」在同一行换行，把 Undo/Redo 组放到 Preview mode 下方的右列；
  - 顶栏高度超过视口 30% 时（用 rem 写的容器查询，或 `@media (max-height: …)`）改为 `position:static`，随编辑列滚动；或者把 Import/Export 收进「More」菜单。
- **可挽回分**：0.1。

### UI-104 · [P3] 200% 导出对话框仍用旧的蓝边次级按钮和非 token 边框（部分解决）
- **单元与截图**：U1 · U1-zoom200-export-end-720-light 的「Select text」「Download」（边框 #8AB7F7、文字 #1570EF）；U1-zoom200-export-720-light 的 Output 选择器（边框 #858F9E）。
- **维度**：视觉。
- **现状 → 期望**：
  - 现状：壳层次级按钮已统一，对话框内仍是 R2 的蓝边样式，闲置按钮看起来像半选中；#858F9E 不是壳层 token。
  - 期望：G3 蓝色只给选中与焦点，分隔与边框用 #D0D5DD/#EAECF0。
- **修法**：对话框动作区复用壳层次级按钮（白底、1px #D0D5DD、#475467 字），选择器边框改为 #D0D5DD，与基础配置的选择器一致。只有 Copy code 保留 #1570EF 实心主按钮。
- **可挽回分**：0.1。

### UI-106 · [P3] 状态行与解除按钮之间 0px，两栏标题中线仍差约 4.5px（部分解决）
- **单元与截图**：
  - U1-configured-1440-light：Brand color 下「• Overridden」胶囊（y=465–488）与「Use configuration values · both modes」（y=489–520）上下相接；Neutral color、Panel background 同样如此。
  - U1-configured-lower-1440、U1-keyboard-02「Use base values · both modes」同样如此。
  - U1-default-1440-light：应用名大写 R 的墨迹在 y=91–103，预览标题 P 在 y=87–98。
- **维度**：布局、细节。
- **现状 → 期望**：分组内状态行与动作没有间距；两栏首行标题不齐 → G2 栈间距与分组行。
- **修法**：
  - 配置组容器改为 `display:grid; gap:.5rem`（或状态行 `margin-block-end:.5rem`）。
  - 预览标题行加 `min-height:2rem; align-items:center`，与编辑列「Rustify Theme Studio + EN/中文」行同高、同中线。
- **可挽回分**：0.1（布局 0.05、细节 0.05）。

### UI-107 · [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一（部分解决）
- **单元与截图**：U1-configured-1440-light「Light 6 · Dark 6」「Light 15 · Dark 15」「Light 2 · Dark 2」（#667085）；U1-field-tabs-1440-light「Search colors…」占位符为 #667085，U1-default-1440-light「Search presets…」占位符约为 #8E949C。
- **维度**：视觉。
- **现状 → 期望**：同一壳层里有两种次级灰和两种占位符灰 → G3 次级文字 #475467。
- **修法**：
  - 胶囊旁的计数改用 `--shell-text-secondary:#475467`，12px。
  - 两个搜索框共用同一个 `::placeholder` token（建议 #667085，在白底上约 4.8:1），去掉 #8E949C。
- **可挽回分**：0.05。

### UI-110 · [P3] 编辑列诊断摘要仍是「1 text pairs」（部分解决）
- **单元与截图**：U1-keyboard-04-1440-light 编辑列底部 y≈783「Diagnostics · 1 text pairs below 4.5:1」。
- **维度**：细节。
- **现状 → 期望**：预览诊断条已改为单数，编辑列摘要没有跟着改，同一数字两处写法不同。
- **修法**：编辑列的「Diagnostics · …」摘要与预览诊断条共用同一个复数函数（`Intl.PluralRules`），输出「1 text pair below 4.5:1」。
- **可挽回分**：0.05。

### 新-3 · [P3] 中文「Saved on this device」有两种译法
- **单元与截图**：U1-zh-top-1440-light 顶栏胶囊「已保存到本机」（x≈205–288，y≈16–39）；同帧编辑列状态块「已保存 / 已保存到此设备」（y≈240–270）。英文两处都是「Saved on this device」。
- **维度**：细节。
- **现状 → 期望**：同一状态的两个译名并存 → G6 文案同步中英文，状态标识一致。
- **修法**：两处共用同一个文案 key。建议统一为「已保存到本机」，与「保存到本机」按钮呼应。
- **可挽回分**：0.05。

### 新-5 · [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」
- **单元与截图**：U1-default-1440-light、U1-keyboard-01 等帧的右上 x≈1266–1416、y≈88–98：「Rustify Light」和「Light preview」之间只有约 12px 空隙。
- **维度**：细节。
- **现状 → 期望**：主题名与预览模式连读，「Light」重复，容易误读 → C4「界面显示『预览模式』以免与作者外观混淆」。
- **修法**：第二项加只在同一行时显示的分隔：`.meta > * + *::before{content:"·"; margin-inline:.375rem}`，换行时用 `:first-child` 规则或 `display:contents` 的方案去掉悬挂点。或者把「Light preview」做成与状态胶囊同样式的中性胶囊（#F9FAFB / #D0D5DD / #475467），从视觉上分开。
- **可挽回分**：0.05。

### 新-6 · [P3] 顶栏左右内边距与两栏不一致，主题名与说明按中线对齐而不是按基线
- **单元与截图**：
  - U1-default-1440-light：「Export」右缘 x=1419（顶栏右内边距 20），预览内容右缘与工具栏分隔线在 x=1416（预览右内边距 24）。
  - U1-default-768-light：顶栏 Undo 左缘 x=20、Dark 右缘 x=747；编辑内容左缘 x=16，预览分隔线右端 x=751。
  - 390 下两边已对齐（16/16）。
  - 1440 顶栏「Rustify Light」基线在 y≈33，「Matches…」基线在 y≈31。
- **维度**：布局、细节。
- **现状 → 期望**：两种视口下各有 4px 错位；两种字号按中线对齐，小字比大字高 2px → G2 编辑区 padding 20 / 预览 24；≤800 时编辑与预览水平都是 16。
- **修法**：顶栏 `padding-inline: 20px 24px`；≤800px 改为 `16px 16px`。主题名组用 `align-items:baseline`，整行仍按中线对齐按钮。
- **可挽回分**：0.05。

### 新-4 · [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高
- **单元与截图**：U1-field-tabs-1440-light：「Search colors…」边框在 y=537/572（36px），「Clear」在 y=539/570（32px），两者并排。预设搜索框同为 36px，而它下面的选择器都是 32px。
- **维度**：布局、细节。
- **现状 → 期望**：除这里外壳层控件已统一为 32px，这一行仍有 4px 落差 → G2/G4 扁平分组行（延续 UI-105 的统一高度）。
- **修法**：输入框也使用 `--shell-control-h:2rem`（`height:2rem; padding-block:0`），与按钮、选择器同高。
- **可挽回分**：0.03。

**可挽回分合计**：0.2 + 0.2 + 0.15 + 0.1 + 0.1 + 0.1 + 0.05 × 5 + 0.03 = **1.13**。10 − 8.7 = 1.3，差 0.17，在 0.5 以内。

## 4. 建议（不扣分）

- 编辑列顶部的存储状态块（「Draft remains in memory…」、Download、0 pending、Save current draft、Cancel、Cache…）在 1440 下占 y=244–520，与顶栏的保存胶囊重复，把「Presets」推到 y=550。可以压成一行摘要加展开，让预设搜索更早出现（C1 的顺序已满足，这条只关乎首屏效率）。
- 1440 下有两个 18px 标题，字重不同：顶栏主题名为粗体，编辑列「Rustify Theme Studio」为常规。可以把应用名降为 14px/600，或两者统一字重。
- 保存状态胶囊（Not saved yet / Saved on this device）没有前导标识，Overridden、Custom 胶囊有。可以统一加前导圆点或图标，并区分「未保存」和「已保存」的标识形状，不依赖颜色。
- 品牌色、中性色选项前加色块。
- 加跳到预览工具栏的链接或 landmark：日志里从品牌色选择器到 Dark 要按 Tab ×114。
- 200% 导出对话框的标题行和关闭按钮吸顶（U1-zoom200-export-end 中标题已滚出可见区）。
- 中文界面下 GPU 预览场景里的「GPU · Cards」「Workspace」「Overview」「Reports」「Activity」「Color tokens」仍是英文。按 U5 口径这些是演示数据，不计入 U1；如果 P4 要求的「GPU 状态文字」包含这些标题，请由 U5 评审确认。
- 帮助、说明文字测得的行距约 19.5px（13px × 1.5），不在 G3 的 body 14/1.5 和 caption 12 两档里。建议确认这类文字归哪一档，再统一。

## 5. 看不出的格子

- **G2 分隔条 8px 拖拽目标与 260–520px 范围**：所有帧都只能看到 x=319 的 1px #D0D5DD，没有悬停、拖拽帧，看不出拖拽目标与宽度限制。
- **悬停、按下态**：没有任何壳层按钮、分段的悬停或按下帧。选中段是否带 `aria-pressed`，截图里看不出。
- **200% 预览列是否可以横向滚动**：截图里滚动条都不显示，看不出被截的分段控件能否靠滚动到达（新-1 按「被截断」计）。
- **基础配置选项完整性（C2）**：U1-keyboard-03 只露出品牌色列表的前 8 项（Custom、Gray、Gold、Bronze、Brown、Yellow、Amber、Orange）；中性色、外观、面板、圆角、缩放的下拉都没有展开，看不出选项是否与 C2 一致。
- **字号精确值**：DPR 1 下帮助文字和选择器值的大写字母都是 9px 墨迹高，分不清是 12px 还是 13px；标题的 -0.02em 字距也无法从截图读出。
- **≤800 下「指标改两列」**：U1 各帧里没有出现指标区。

## 6. 总评

- **分数**：U1 从 7.8 升到 **8.7**。
- **改进**：R2 的 P1（200% 粘性头封顶）和吸顶区过高这一结构问题，由「横跨两栏的单行命令顶栏 + 编辑列回到文档流」解决。390 预览页、保存状态语义、字重、token、状态胶囊、焦点 token 也一并修好。
- **最低的维度**：布局 2.6。主要原因集中在 200% 文本放大：
  - 渲染器分段控件越出预览列被截（新-1，回归）；
  - 导出 Output 值被截成「Theme…」（新-2，回归）；
  - 顶栏固定占 39% 高度（UI-103 残留）。

  这几处都是因为布局断点只按视口 px 写，没有随文字放大而改变。
- **是否要结构性改动**：不需要推倒重做。有一处需要成体系地改：给顶栏、预览工具栏和对话框表单行加上用 rem 写的容器查询，或允许换行，让 200% 文本时这些区域换行或改成单列，而不是被挤压或截断。
- **其余**：都是 P3 级打磨，按差距清单逐条修即可：
  - 分段组件统一（UI-02）；
  - 对话框按钮样式并入壳层（UI-104）；
  - 状态行间距（UI-106）；
  - 计数色和占位符（UI-107）；
  - 复数（UI-110）；
  - 中文译名（新-3）；
  - 「Light preview」分隔（新-5）；
  - 内边距与基线（新-6）；
  - 输入框高度（新-4）。
