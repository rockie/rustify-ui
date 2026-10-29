# U2 Studio 精调与来源解释 · R3 评审原文

- 被测：HEAD `d5775d8a`，Theme Studio build `80cad3c530b54d9f`（据 r3/capture.md）。
- 依据：r3 中以 `U2-` 开头的 9 张图、r3/U2-actions.log；对照 r2 的同名 9 张图与 r2-judge-U2.md。
- 目标：规范要点 F1–F4、G1–G4。本次只评 U2。
- 测量方法：对截图做像素取色、放大裁切。下文坐标都是 CSS px（DPR 1）。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.6 | 2.55 | 2.9 | 0.8 | **8.85** |

R2 为 7.75，本轮升 1.1。

**各维度为什么不是更高一档**

- **布局 2.6**
  - 已做对的：壳层收进 56px 顶栏；选中 token 后，来源块就地插在所选行下方并进入视口；390 两层吸顶之间不再漏出内容；预览诊断改为单列逐行。
  - 不到 3 的原因：1440 两列网格里，「Overridden · Light」胶囊折成两行、撑高单元格，一眼就能看出。另外还有三处：390 下选中格被吸顶区遮住；配置组的状态胶囊与按钮之间 0px；来源块的键值列只做了一半。
- **视觉 2.55**
  - 已做对的：状态胶囊改为中性色；选中分体控件两段同时高亮；错误边框 #B42318；焦点环 3px #A1C6F9、2px offset；搜索框边框 #98A2B3。以上都对上了 token。
  - 不到 3 的原因：折行的胶囊文字压到了圆角边；小节标题与次级标签仍同样式；诊断区的子 summary 比父 summary 更重；计数文字仍用 #667085；字体下拉的值是粗体。
- **交互与状态 2.9**：选中显示来源、单项解除、解除后按钮置灰并说明原因、分组解除写明 both modes、非法输入贴字段报错并替换 helper、预览侧诊断带 Locate，全部走通。不到 3 的原因有两处：选中格里的 ↺ 在不可解除时仍是选中蓝，看起来可用；选中后的滚动位置没有同时露出所选格和完整来源块。
- **细节 0.8**：R2 的 7 处细节差异大多已修。还剩三处：诊断区有一段约 44px 的空白；句中「Light:」也被设成等宽；同一计数有三种措辞。

## 2. 上一轮差距复核表

| 编号 | R2 内容 | R3 状态 | 依据 |
| --- | --- | --- | --- |
| UI-102 | 390 两层吸顶之间漏出内容 | **已解决** | U2-source-overridden-390：分隔线在 y=152，y=153–200 全为 #FFFFFF，内容在吸顶区下沿处干净裁切，两层之间不再透出内容 |
| UI-106（U2 部分） | 配置组状态胶囊与按钮相贴 | **仍存在** | U2-group-reset-1440：Brand 胶囊底边 y=488、按钮顶边 y=489；Neutral 为 658/659；Panel 胶囊底边 y=853，按钮紧随其下 |
| UI-107（U2 部分） | 「Choose a bundled font」用 #667085；颜色搜索框边框 #D0D5DD | **部分解决** | 「Choose a bundled font」与 helper 已是 #475467，搜索框边框已是 #98A2B3。仍有偏差：U2-group-reset 的「Light 0 · Dark 0」「Light 15 · Dark 15」「Light 2 · Dark 2」和「No overrides」胶囊文字是 #667085 |
| UI-109 | 状态胶囊借用选中蓝 | **已解决** | 网格胶囊与配置组胶囊都是 #F9FAFB 底、#D0D5DD 边、#475467 字，并保留前导圆点 |
| UI-17 | 选中 token 后来源与解除按钮不在屏内 | **部分解决** | 主体已解决：U2-source-overridden-1440 的来源块（y=618–873）与「Clear primary · Light only」都在屏内，1440 常驻壳层也已压成 56px 顶栏。余项：390 下所选格被吸顶区遮住；ring 的来源块下半部在折线外（见差距清单） |
| UI-115 | 网格 ↺ 只有图标，看不到范围 | **已解决（可见部分）** | 胶囊写明「Overridden · Light」；未覆盖格的 ↺ 为 #98A2B3（accent、destructive），已覆盖格为 #475467。`aria-label` 从截图看不出 |
| UI-18 | 小节标题与字段 label 同样式 | **部分解决** | 「Typography」「Geometry」已改为 12px/600 #475467，与 13px/600 #1D2939 的字段 label 区分开了。仍同样式的有两处：「Typography」与次级标签「Choose a bundled font」；「Theme configuration」与「Appearance」 |
| UI-22 | 诊断区嵌套标题倒置、正文大于标题 | **部分解决** | 正文已降为 12px，字体诊断条目之间已加分隔线。余项：外层 summary 是 13px/400，内层是 13px/600，子级仍比父级重 |
| UI-116 | Locate 与色对的对应关系不清 | **已解决** | 预览诊断改为单列，每行一对，Locate 右对齐，行间有分隔线 |
| UI-117 | 同一比值两处精度不同 | **已解决** | 编辑列与预览条都写「1.00:1」「2.62:1」 |
| UI-118 | 错误文字重复上方 helper | **已解决** | 出错时 helper 被错误文字替换 |
| UI-119 | 选中分体控件只高亮左半段 | **已解决** | 外框与中间分隔都是 #84CAFF，两段都是 #EFF8FF |
| UI-120 | 句中色值不是等宽字 | **已解决** | 「#d6409f」已用等宽字。但模式名「Light:」也一起变成了等宽，另记为新-5 |
| UI-121 | Override / Overridden 两种措辞 | **已解决** | 状态统一写「Overridden」 |
| UI-122 | 范围文案像被截断 | **已解决** | 改为「resolved range −0.5em to 0.5em」 |
| UI-123 | 来源块里 token 名出现两次 | **已解决** | 拾色区标签改为「Color controls」 |
| UI-124 | 「Diagnostics」与「Preview diagnostics」重复 | **已解决** | 已删去「Preview diagnostics」 |

## 3. 差距清单

先按严重度、再按可挽回分排序。可挽回分合计 1.15，等于 10 − 8.85。

### 新-1 · [P2] 1440 网格中「Overridden · Light」胶囊折成两行，文字压到圆角边，单元格被撑高

- **维度**：布局、视觉。可挽回 0.35（布局 0.2，视觉 0.15）。
- **位置**：出现在 U2-source-overridden、U2-source-overridden-detail、U2-token-released、U2-source-generated 四张 1440 图。
  - 以 U2-source-overridden-1440 的 background 格为例：格子 x=20–157、y=195–274；胶囊 x≈50–125、y≈239–268；第二行「Light」从 x≈54 起笔，离胶囊左侧弧边只有约 3px。
  - 有胶囊的格高约 80px；没有胶囊的 ring 格（U2-source-generated y=614–659）只有约 45px，同组行高明显不齐。Surfaces 6 个 token 占了 278px。
  - 390 下格子更宽，胶囊在一行内放得下（U2-source-overridden-390 y≈201–212）。
- **现状 → 期望**：圆角 full 的胶囊里文字折行、贴边，看起来像是坏了。期望依据：
  - F4：颜色 token 与重置按钮合成一个外缘圆角的分体控件，长 token 名可以换行，但没有允许状态标识折行；
  - C3：Custom 与 overridden 以文字 + 状态标识显示，标识须完整可读；
  - G2：壳层是分组行，不应出现参差的卡片高度。
- **修法**（任选其一，推荐 A）：
  - A. 让 token 网格按编辑列宽自动降为单列：`grid-template-columns: repeat(auto-fill, minmax(13.5rem, 1fr))`。在 320px 编辑列（内容约 280px）下就是一列，行内顺序为：色块、名称、等宽值、胶囊、↺ 段。胶囊加 `white-space: nowrap`，padding `2px 8px`。顺序保持「base、foreground」相邻，这样也顺带消除「card-/foreground」这类断行。
  - B. 保留两列，但把胶囊移到格子底部单独一行，横跨文字列和 ↺ 段下方，宽度 100%，`white-space: nowrap`，必要时字号降到 11px/600。注意不要用省略号截掉模式名。

### UI-17（余项）· [P3] 选中后的滚动位置没有同时露出所选格与完整来源块

- **维度**：布局、交互。可挽回 0.15（布局 0.1，交互 0.05）。
- **位置**：
  - U2-source-overridden-390：吸顶的顶栏与 Edit/Preview 标签占 y=0–约 200。所选 primary 格只露出底部约 20px（y≈200–221，只看得到胶囊和下边框），色块、名称、值都在吸顶区下面。
  - U2-source-generated-1440：ring 格在 y=614–659，来源块从 y=673 开始，「Clear ring · Light only」与拾色区都在折线以下。
- **现状 → 期望**：用户看得到来源块，却看不到正在编辑的是哪一格；或者看得到来源，却看不到解除按钮。期望依据：F1 的有效值、来源、覆盖、重置要同屏可读；F2 的「解除将影响哪些项」与解除入口要一起出现。
- **修法**：
  - 给 token 格设 `scroll-margin-top: calc(吸顶区高度 + 8px)`。390 下吸顶区约 200px，建议用 CSS 变量，由吸顶容器的实测高度写入。
  - 选中后先对来源块末尾的解除按钮做 `scrollIntoView({block:'nearest'})`，再保证所选格顶边不高于吸顶区下沿。放不下时优先保住所选格的顶边。

### UI-18（余项）· [P3] 小节标题仍与次级标签或字段标签同样式

- **维度**：视觉。可挽回 0.1。
- **位置**：
  - U2-controls-1440：「Typography」（y≈251）与「Choose a bundled font」（y≈332、570、807）都是 12px/600 #475467。
  - U2-field-error-1440：「Geometry」（y≈394）同上。
  - U2-group-reset-1440：「Theme configuration」（y≈98）与「Appearance」「Brand color」（y≈234、360）都是 13px/600 #1D2939。
- **现状 → 期望**：标题、字段标签、子标签三层里总有两层重合 → G3 的 label 13px/600、caption 12px，以及 G2 的「细分隔线与分组行」层级。
- **修法**：
  - 小节标题统一用一种眉题样式：12px/600、`text-transform: uppercase`、`letter-spacing: .04em`、#475467，上方加 #EAECF0 分隔线，上间距 24px、下间距 12px。「Theme configuration」也归入这一层。
  - 字段 label 保持 13px/600 #1D2939。
  - 子标签「Choose a bundled font」降为 12px/400 #475467。

### 新-3 · [P3] 来源块的键值结构只做了一半，行标签与块标题同样式

- **维度**：布局、视觉。可挽回 0.1（布局 0.05，视觉 0.05）。
- **位置**：U2-source-overridden-1440，y=637–873，390 图同。
  - 「Fine adjustment source」（y≈643）、「Effective value」（y≈709）、「Color controls」（y≈895）三处都是 13px/600 #475467。
  - 只有「Effective value」一行是键值两列，值「#606060」漂在 x=196，与上下都不对齐。
  - 「Explicit document override」和「overrides.light.primary」没有对应的「Source」标签，直接堆在下面。
- **现状 → 期望**：读不出哪一行是标题、哪一行是字段名 → F2 的来源说明须逐项回答「值来自哪里」「哪个覆盖在生效」「解除将影响哪些项」。
- **修法**：
  - 来源块改为 `<dl>`，`display: grid; grid-template-columns: 7rem 1fr; gap: 6px 12px`。
  - 行依次为：Token、Effective value、Source（Explicit document override 或 Generated by base configuration）、Path（等宽）、If cleared。
  - dt 用 12px/400 #475467，dd 用 13px #1D2939，色值与路径用 12px 等宽。
  - 块标题沿用 UI-18 的眉题样式。

### 新-7 · [P3] 诊断区「Authored colors fit the sRGB preview.」下方有约 44px 空白

- **维度**：细节。可挽回 0.1。
- **位置**：U2-low-contrast-1440。该行文字墨迹在 y=119–131，下一条「font-sans」的墨迹从 y=176 开始，中间约 44px 没有任何内容或分隔线。其余条目之间约 16px。
- **现状 → 期望**：栈间距不均，像是漏渲染了一项 → G2 的栈间距 16px。
- **修法**：查这里是否有空容器（例如没有色域警告时仍渲染了空的 `<ul>`，或带 margin 的占位元素）。内容为空时不渲染该节点，或统一由父级 `gap: 16px` 控制，去掉子元素的 margin。

### UI-22（余项）· [P3] 诊断区子 summary 比父 summary 更重

- **维度**：视觉。可挽回 0.05。
- **位置**：U2-low-contrast-1440。外层「▼ Diagnostics · 5 text pairs below 4.5:1」（y≈94）是 13px/400 #475467；内层「▼ Text contrast pairs · 5 below 4.5:1」（y≈362）是 13px/600。
- **现状 → 期望**：子级比父级重，层级倒置 → G3 层级，以及 R2 修法「两级 summary 统一 13px/600、内层缩进 12px」。
- **修法**：外层 summary 改为 13px/600 #1D2939，内层保持 13px/600 但用 #475467，或改为 13px/500。内层缩进 12px 维持不变。

### UI-106（余项）· [P3] 配置组状态胶囊与按钮 0px 相贴

- **维度**：布局。可挽回 0.05。
- **位置**：U2-group-reset-1440。「No overrides」胶囊底边 y=488，「Use configuration values · both modes」顶边 y=489；「Overridden」胶囊与按钮为 658/659；Panel 胶囊底边 y=853，按钮紧随其下。
- **现状 → 期望**：两者直接相接 → G2 栈间距。
- **修法**：配置组内用 `display: grid; gap: .5rem`，或者给按钮加 `margin-top: 8px`。

### UI-107（余项）· [P3] 分组计数与「No overrides」文字用 #667085

- **维度**：视觉。可挽回 0.05。
- **位置**：U2-group-reset-1440 的「Light 0 · Dark 0」（y≈476）、「Light 15 · Dark 15」（y≈646）、「Light 2 · Dark 2」（y≈841），以及「No overrides」胶囊文字。
- **现状 → 期望**：同一区域的 helper 已是 #475467，计数仍用另一种灰 → G3 的次级文字 #475467。
- **修法**：计数与状态胶囊文字改用 `--shell-text-secondary`（#475467）。#667085 只留给真正禁用的文字。

### 新-2 · [P3] 选中格里不可解除的 ↺ 仍显示为选中蓝，看起来可用

- **维度**：交互与状态。可挽回 0.05。
- **位置**：
  - U2-source-generated-1440：ring 格（生成值，没有覆盖）的 ↺ 是 #175CD3；同屏未覆盖的 accent、destructive 的 ↺ 是 #98A2B3。
  - U2-token-released-1440：primary 已解除覆盖，↺ 仍是 #175CD3。来源块里的「Clear primary · Light only」已经置灰，两处信号互相矛盾。
- **现状 → 期望**：选中态盖过了禁用态 → F2 的解除须显示真实范围；C3 的状态不只靠颜色，也不能被颜色误导。
- **修法**：选中样式只改外框和底色。↺ 段的图标色由自身状态决定：`aria-disabled="true"` 时固定 #98A2B3，在 #EFF8FF 底上同样适用；可用时才用 #175CD3。

### 新-4 · [P3] 字体下拉的显示值是粗体，与其他下拉、输入框不一致

- **维度**：视觉。可挽回 0.05。
- **位置**：U2-controls-1440 的三处「Custom author stack」（y≈357、595、832）都是 600。同页「Montserrat, sans-serif」输入框，以及 U2-group-reset 的「Inherit」「Pink」「Auto」「Custom」下拉，都是 400。
- **现状 → 期望**：同类控件的字重不一致 → G3 的 body 14px / 1.5。
- **修法**：去掉字体下拉值的 `font-weight`。如果要提示这是自定义栈，按 Panel background 的做法在字段 label 旁加中性「Custom」胶囊。

### 新-5 · [P3] 句中把模式名「Light:」也设成了等宽

- **维度**：细节。可挽回 0.05。
- **位置**：U2-source-overridden-1440 y≈800，以及 U2-source-overridden-390 y≈414，「Clearing this override reveals `Light: #d6409f`.」整段都是等宽字。
- **现状 → 期望**：只有颜色值该用等宽 → G3「颜色值等宽 12px」。
- **修法**：`<code>` 只包住色值，写成「reveals Light: `#d6409f`.」。

### 新-6 · [P3] 同一对比计数在三处用了三种措辞，父子 summary 还重复同一个数字

- **维度**：细节。可挽回 0.05。
- **位置**：U2-low-contrast-1440 的外层「Diagnostics · 5 text pairs below 4.5:1」、内层「Text contrast pairs · 5 below 4.5:1」，以及预览条「5 text pairs below 4.5:1 · author colors preserved」。
- **修法**：外层只写「Diagnostics」或「Diagnostics · 5 issues」；内层保留「Text contrast pairs · 5 below 4.5:1」；预览条沿用同一句式，写成「5 text pairs below 4.5:1 · author colors preserved」。

## 4. 建议（不扣分）

1. 来源块同时给出另一模式的状态（例如「Dark: not overridden」），并附该 token 所属分组的解除入口。沿用 R2 建议。
2. primary 为中性灰（#606060）时，U2-source-overridden-detail-1440 的拾色平面仍显示饱和蓝。建议色相为零或无色时平面显示灰阶，或保留上次的色相。沿用 R2 建议。
3. 「Overridden only (23)」写出分模式计数，例如「Light 23 · Dark 0」。沿用 R2 建议。
4. 预览诊断条有 5 对时约 290px 高（U2-low-contrast-1440 y=187–480），把 DOM 预览压到 y≈525。建议超过 3–4 对时折叠，提供「Show all」。
5. 390 下吸顶的顶栏和标签约占 200/844px。建议滚动时把顶栏压成单行（属 U1 范围，这里只作提示）。

## 5. 看不出的格子

- **「恢复预设/文档」入口及其范围说明**（F2 第三档）：9 张 U2 图里都看不到。R2 时这个入口在常驻壳层里，R3 的顶栏里没有，编辑列都处在滚动后的位置。是否存在、写了什么范围，都无法判断。
- **token 格的键盘焦点**：U2-source-overridden-1440 与 -390 登记的焦点在 `color-token-primary`，但格子只有选中样式，看不到焦点环。可能是点击触发、没有走 :focus-visible，无法判定焦点是否可见。
- **拾色字段的焦点**（F4：picker 蓝 40% outline、3px、2px offset）：没有一张图把焦点放在平面或 HEX 输入上。
- **Locate 的定位结果**：没有点击记录，也没有对应截图。
- **↺ 的 `aria-label` 与 `aria-disabled`**：截图无法读出。
- **两种模式都被覆盖、或只覆盖 Dark 时胶囊的写法**：没有对应帧。
- **U2-low-contrast-1440**：background、foreground 两个字段本身不在视口内，看不出字段旁是否也有提示。
- **U2-source-generated-1440**：ring 的「Clear ring · Light only」在折线以下，看不出是否禁用。

## 6. 总评

- **分数**：U2 为 8.85，最低的维度是视觉（2.55）。主要失分是新-1：1440 两列网格里状态胶囊折行、文字压边、行高参差，四张 Colors 图都有，一眼可见。其余是层级样式（UI-18、新-3、UI-22）和 token 用色（UI-107、新-4）的收尾。
- **需要结构性改变的地方**：只有 token 网格的单元格布局（新-1）。要么在编辑列宽下改成单列逐行，要么把胶囊移到单独一行。这是对该组件的重新排布，不是整页重做。来源块改成 `<dl>` 键值网格（新-3）是组件内部重排。其余各项都是 token、间距或滚动参数的调整。
- **与 R2 的差别**：R2 的两项主要问题（UI-17 来源不在屏内、UI-102 390 吸顶漏缝）已基本解决，R2 列出的 9 项细节问题解决了 8 项。新-1 是 UI-115 加了模式文字之后才出现的副作用，修复时要连同胶囊的排版一起处理。
