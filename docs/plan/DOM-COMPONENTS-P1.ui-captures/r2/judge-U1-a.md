# U1 Studio 工作台框架与基础配置 · R2 评审（重建基线）

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.4 | 2.5 | 2.7 | 0.5 | **8.1**（R1 6.8） |

- **布局 2.4。** 以下几处与规范一致：双栏结构，编辑列 320px（分隔线在 x=319，#D0D5DD），粘性顶栏放主题名、模式、撤销、保存、导出，≤800 时水平 padding 16、主标题 16px，390 下用 Edit/Preview 切换并有 Back to edit，Host control 已回到预览底部的文档流。不到 3 分的原因有四处。第一，200% 下顶栏被截在 y≈449，「Preview mode」被切掉半行，模式按钮和 Undo/Save/Export 藏进了没有可见滚动提示的内滚动区。第二，390 编辑页顶栏上方漏出滚动内容。第三，工具栏控件高度有 28、36、38px 三档。第四，编辑列标题比预览标题高 9px。
- **视觉 2.5。** 以下符合 G3：字族，分隔线 #D0D5DD / #EAECF0，搜索框边框 #98A2B3，选中三色 #EFF8FF / #84CAFF / #175CD3；预览切深色时编辑壳仍是中性浅色。不到 3 分的原因有三处。第一，字段说明文字是 #667085，不是 G3 的 #475467。第二，闲置按钮和未选分段带 #8AB7F7 浅蓝框（与焦点环同色），次级按钮有 4 套样式。第三，英文 label 没有呈现 600 字重。
- **交互与状态 2.7。** 已做到：键盘能依次到达搜索框、品牌色选择器、展开的列表和 Dark；搜索框焦点为 3px #8AB7F7、2px offset，符合 G4；列表与触发器等宽，有 #EFF8FF 高亮；按下态可辨；Overridden、Custom 都有文字加标识（C3）；「Preview mode」与外观已分开（C4）；中文切换完整。不到 3 分的原因有两处。第一，200% 下 G5 的顶部控件只能靠键盘焦点带动内滚动才露出。第二，Dark 的焦点环压在相邻的 Light 段上。
- **细节 0.5。** 有若干小差异：标题基线错位；顶栏下有双细线；Overridden 胶囊和下方按钮之间间距为 0px；「mauve」是小写且未翻译；焦点环有三种透明度；中英文 label 的字重不一致。

## 2. 上一轮差距复核（只按 U1 截图）

| 编号 | R1 可挽回（U1） | R2 结论 | 依据 |
| --- | --- | --- | --- |
| UI-01 Host control 浮签遮挡 | 0.5 | **已解决**（文案部分待核实） | 所有 R2 U1 视口帧的左下、右下都没有浮签。`U1-keyboard-04` 中它在预览底部的文档流里（x≈344, y≈821），不压内容。中文态下它是否还是英文：zh 帧没拍到它。 |
| UI-02 选中/未选中切换不可分 | 0.5 | **部分解决**（残留 0.2，见下） | 选中段为 #EFF8FF 底、#84CAFF 边、#175CD3 字并加粗；未选段为白底、#475467 字，可以分清。残留问题：Light/Dark、DOM/GPU、Inspect 的未选段在 #D0D5DD 外框里多一道 1px #8AB7F7 内框，EN/中文、Edit/Preview 却没有；Dark 的焦点环压到 Light 上。 |
| UI-03 顶部缺主题名与模式 | 0.5 | **已解决** | 1440、768、390 的编辑页顶栏都常驻主题名、Matches preset/Modified、Preview mode、Undo/Redo/Restore、Save/Import/Export，滚动后仍在（`U1-configured-1440-light`、`U1-configured-390-light`）。200% 下的新问题记为新-2。 |
| UI-04 语言切换挤压，「EN」被裁 | 0.3 | **已解决** | `U1-zh-top-1440-light` 中 EN/中文为两段分段控件，各约 49px 宽，没有裁切。两组分段控件高度不一致的问题记入新-4。 |
| UI-05 200% 壳层缩放不一致 | 0.3 | **已解决** | `U1-zoom200-720-light` 中文字与盒子同比放大，「Responsive」不再截断；导出对话框的「Output」「Exported code」也随之放大。顶栏截断是新问题，记为新-2。 |
| UI-06 Overridden/Custom 无状态标识 | 0.3 | **已解决** | 「● Overridden · Light 6 · Dark 6」胶囊（#EFF8FF / #84CAFF / #175CD3）；「Panel background ● Custom」「Brand color ● Custom」带徽标；中文为「● 已覆盖」「● 自定义」。 |
| UI-07 品牌色下拉过窄、高亮为灰 | 0.2 | **已解决** | `U1-keyboard-03`：面板与触发器等宽（x 20–299），限高并内滚（Orange 露半行），当前项「Gray」为 #EFF8FF 底、蓝字，已选「Custom」带 ✓。 |
| UI-08 390 底部 1px 线与色带 | 0.2 | **已解决** | `U1-edit-390`、`U1-configured-390` 底部 60px 放大后无残带。 |
| UI-09 label、小节标题、选中标签页字号字重 | 0.2 | **部分解决**（残留 0.15） | label 字号由约 14px 降到 13px（「Appearance」宽 75→69px）。英文 label 笔画仍是 1px，和选择器值「Inherit」一样，而中文 label「外观」的笔画是 2px，说明英文没有呈现 600。字段标签页本轮没拍到。 |
| UI-10 Undo/Redo 描边、Reset 是链接 | 0.1 | **已解决** | Undo/Redo/Restore document 已统一为同一种描边按钮。另有次级按钮样式不统一的问题，记为新-5。 |
| UI-11 字段标签页下双分隔线 | 0.1 | **待核实** | R2 的 U1 帧都没拍到 Colors / Type & other 标签页。 |
| UI-12 色族大小写与中英混排 | 0.1 | **部分解决**（残留 0.1） | 选项已改为首字母大写（Pink、Gray、Gold…），中文为「粉色」。但「Resolved neutral: mauve.」仍是小写原始键，中文态显示「已解析中性色：mauve。」。 |

## 3. 差距清单（先按严重度，再按可挽回分排序）

### 新-2 · [P1] 200% 文本下顶栏被截成约 50vh 的内滚动区，「Preview mode」被切掉半行，模式、撤销、保存、导出都不可见

- **单元与截图：** U1 · `U1-zoom200-720-light.png` 编辑列；`U1-zoom200-export-720-light.png` 遮罩后方。
- **维度：** 布局、交互。
- **现状：**
  - 编辑列顶栏底边在 y=449（整列宽，正好是视口高 900 的一半）。
  - 顶栏只露出两行标题、EN/中文、两行副标题、「Rustify Light / Matches preset」。「Preview mode」在 y≈440–449 被切掉下半。
  - Light/Dark、Undo/Redo/Restore、Save locally/Import/Export、Local themes 都在截断线以下，没有滚动条或渐隐等可见提示。
  - 在导出帧里，遮罩后方顶栏底边仍在 y=449，但顶栏内容已经滚到 Light|Dark 位于 y≈0–20。这说明顶栏是一个限高的内滚动容器，只是因为键盘焦点落到了 Export 才滚动过去。
- **期望：**
  - 200% 下主路径可达，不靠隐藏的嵌套滚动到达关键动作，文字不被截断（G6）。
  - 顶部保留主题名、模式、撤销/重做/保存/导出（G5）。
- **修法：**
  1. 去掉顶栏上的 `max-height` 和 `overflow`。任何含 label 的容器都不要同时设限高和 overflow。
  2. 把顶栏拆成两段：
     - 非粘性的引导段：应用名、EN/中文、副标题，随内容滚走。
     - 粘性的命令条：第一行为主题名 + 状态 + Preview mode 分段控件；第二行为 Undo、Redo、Save、Export，紧凑按钮，高 2rem，`flex-wrap: nowrap`。Restore document、Import、Local themes 收进「More」菜单。
  3. 用 ResizeObserver 比较命令条高度与编辑面板 `clientHeight`：命令条高度超过面板 40% 时改为 `position: static`。这样 200% 下不会截断，内容按正常流滚动。
  4. 用 720×900、根字号 200% 复拍，确认 Light/Dark 和 Save/Export 在首屏或随正常滚动可见。
- **可挽回分：** 0.55（布局 0.3，交互 0.25）。

### 新-1 · [P2] 390 编辑页的 Edit/Preview 标签条和粘性顶栏之间有约 30px 空隙，滚动内容从这里漏出

- **单元与截图：** U1 · `U1-configured-390-light.png`，y≈45–85；对照 `U1-edit-390-light.png`。
- **维度：** 布局、细节。
- **现状：**
  - 滚动后，「Presets」（x≈16–60）和计数「44」（x≈360–374）出现在 y≈68–76，下半被顶栏遮住，夹在标签条和「Rustify Theme Studio」之间。
  - 未滚动时同一区域是空白带。
  - 1440 和 768 没有这个问题（顶栏上方 15px 内全是白色）。
- **期望：** 编辑壳稳定，各区独立滚动，不出现半截文字（G1、G2）。
- **修法：**
  - 把编辑滚动容器的 `padding-top: 24px` 移到粘性顶栏内部：容器 `padding-top: 0`，顶栏 `top: 0; padding-top: 1.5rem; background: #fff`。
  - 如果标签条也在同一个滚动容器里，就把两者叠成粘性栈：标签条 `top: 0`，顶栏 `top: var(--tabbar-h)`；标签条加 1px #EAECF0 下边框。
- **可挽回分：** 0.2（布局 0.15，细节 0.05）。

### 新-5 · [P2] 壳层次级按钮有 4 套边框和文字配色，闲置按钮用焦点同色 #8AB7F7 做边框

- **单元与截图：** U1 · `U1-default-1440-light.png`、`U1-configured-1440-light.png` 的编辑列。
- **维度：** 视觉。
- **现状（逐像素取色）：**

  | 按钮 | 边框 | 文字 |
  | --- | --- | --- |
  | Undo（可用）、Restore document、Download current theme | #8AB7F7 | #475467 |
  | Import、Export、Favorites、Save current draft、Use configuration values · both modes | #8AB7F7 | #1570EF |
  | Cancel pending writes、Retry retained saves | #858F9E | #667085 |
  | Focus preview | #D0D5DD | #475467 |

  #8AB7F7 正是搜索框焦点环的颜色，所以闲置的 Undo、Import 看起来像半选中或已聚焦。
- **期望：** 蓝色只用于选中与焦点；壳层扁平、中性，分隔用 #D0D5DD / #EAECF0（G3）。
- **修法：**
  - 统一为一个 ShellButton 次级样式：白底，`1px solid #D0D5DD`，文字 #344054 或 #475467，字重 500，hover 底 #F9FAFB，disabled 保持现有灰底。
  - 闲置态去掉 #8AB7F7 边框，它只留给 `:focus-visible`。
  - Import、Export、Favorites、Use … values 改为中性文字。蓝色只保留给主操作 Save locally（实心）和真正的文字链接（Local themes、Favorite）。
- **可挽回分：** 0.15（视觉）。

### UI-02（残留）· [P3] 分段控件样式不统一，未选段带蓝色内框，焦点环压住相邻段

- **单元与截图：** U1 · `U1-default-1440-light.png` 中的 Light/Dark（y 127–155）、DOM/GPU/DOM+GPU 与 Inspect（y 65–92）；`U1-keyboard-04-1440-light.png` 中的 Dark 焦点。
- **维度：** 视觉、交互、细节。
- **现状：**
  - Dark、GPU、DOM、Inspect 的未选态是 #D0D5DD 外框里再套 1px #8AB7F7 内框（放大后是双框）。EN/中文、Edit/Preview 的未选态只有 #D0D5DD。
  - 选中段除了按下态底色，还多一条 2px #175CD3 下划线，这是字段标签页的写法。
  - Dark 获焦时，3px 焦点环的左边压在 Light 段右缘上，左侧是直角、右侧是圆角。
  - 焦点环颜色有三种：搜索框 #8AB7F7，品牌色选择器 #A1C6F9，Dark 为 #97C2F9/#A1C6F9。
- **期望：**
  - 选中的模式/渲染器按钮用按下态；下划线只用于字段型标签页（G4）。
  - 选中三色按 G3；焦点可见，且不遮挡相邻控件。
- **修法：**
  - 做一个 SegmentedControl 组件，所有分段控件共用（EN/中文、Light/Dark、Edit/Preview、DOM/GPU/DOM+GPU、Inspect）：
    - 外框 `1px solid #D0D5DD`，圆角 6px，段间 1px #D0D5DD 分隔。
    - 未选段：透明底，#475467，字重 500。
    - 选中段：#EFF8FF 底，`box-shadow: inset 0 0 0 1px #84CAFF`，#175CD3，字重 600，去掉 2px 下划线。
    - 语义用 `aria-pressed`。
  - 焦点环：`outline: 3px solid rgb(21 112 239 / .4); outline-offset: 2px; position: relative; z-index: 1`，`border-radius` 与段一致；或者改用 `outline-offset: -3px` 的内嵌环，避免压到邻段。
  - 搜索框、选择器、分段控件共用一个焦点 token。
- **可挽回分：** 0.2（视觉 0.1，交互 0.05，细节 0.05）。

### 新-4 · [P3] 同一行的壳层控件高度有三档，工具栏上下没有对齐

- **单元与截图：** U1 · `U1-default-1440-light.png`、`U1-default-768-light.png`、`U1-preview-390-light.png`、`U1-zoom200-720-light.png`。
- **维度：** 布局、视觉、细节。
- **现状：**
  - 1440 预览工具栏：Cards、Responsive 高 36px（y 61–96）；DOM/GPU/DOM+GPU、Inspect 高 28px（y 65–92）；Focus preview 高 38px（y 60–97）。
  - 768 和 390 的第二行里，Inspect（28px）与 Focus preview、Back to edit（38px）顶边相差约 6px。
  - 编辑顶栏：EN/中文高 38px（y 16–53），Light/Dark 高 29px（y 127–155）。
  - 200% 下各控件分别为 72、54、52、72px。
- **期望：** 主要组件的尺寸和对齐一致；壳层用分组行（G2）。
- **修法：**
  - 定义 `--shell-control-h: 2rem`。工具栏的 Select 触发器、分段控件、Inspect、Focus preview、Back to edit，以及顶栏两组分段控件，都用 `height: var(--shell-control-h)`，水平 padding 0.75rem。
  - 工具栏 flex 设 `align-items: center; gap: .5rem`。
  - Focus preview、Back to edit 与 Inspect 用同一个按钮类。
- **可挽回分：** 0.15（布局 0.05，视觉 0.05，细节 0.05）。

### 新-6 · [P3] 编辑区顶部 padding 为 16px，编辑列标题比预览标题高约 9px（相对 R1 回归）

- **单元与截图：** U1 · `U1-default-1440-light.png` 顶部；`U1-default-768-light.png`。
- **维度：** 布局、细节。
- **现状：**
  - 「Rustify Theme Studio」字形顶在 y=20，「Preview」字形顶在 y=30。R1 是 28 与 30，基线基本对齐。
  - EN/中文顶边在 y=16，说明编辑顶栏的上 padding 约为 16px。
  - 标题行与 EN/中文分段控件没有垂直居中：标题中线约 y=29，控件中线约 y=34.5。
- **期望：** 编辑区 padding 为 24px 20px 48px，预览区为 24px 24px 48px（G2）。
- **修法：**
  - 粘性顶栏 `padding-top: 1.5rem`，与预览区一致；≤800 时两侧同步改为 20px。
  - 标题行用 `display: flex; align-items: center`，让 EN/中文与标题垂直居中。
  - 编辑标题（18px/1.3）与预览标题（16px/600）所在行设相同的 `min-height`，使基线对齐。
- **可挽回分：** 0.15（布局 0.05，细节 0.1）。

### UI-09（残留）· [P3] 英文 label 没有呈现 600 字重，中英文 label 粗细不一致

- **单元与截图：** U1 · `U1-configured-1440-light.png` 中的「Appearance」「Brand color」「Neutral color」「Theme configuration」「Preview mode」「Presets」；对照 `U1-zh-configured-1440-light.png` 中的「外观」「品牌色族」「预览模式」。
- **维度：** 视觉、细节。
- **现状：**
  - 字号已是 13px，颜色为 #1D2939。
  - 放大 8 倍后，英文 label 的竖笔约 1px，与选择器值「Inherit」相同；中文 label 竖笔约 2px，明显是半粗。
  - 字段标签页的选中字重本轮看不出。
- **期望：** label 为 13px / 600；选中标签页为 13px 半粗（G3、G4）。
- **修法：**
  - 确认 label 类为 `font-weight: 600`。
  - 确认 IBM Plex Sans 的 600 字面已通过 `@font-face`（URL 指向 public 静态资源）加载，并检查 `document.fonts` 里有 weight 600 的条目。缺少这个字面时浏览器会回退或合成，英文就呈现不出 600。
  - 小节标题「Theme configuration」与 label 同级时，同样用 600。
- **可挽回分：** 0.15（视觉 0.1，细节 0.05）。

### 新-3 · [P3] 字段说明和计数文字用 #667085，不是 G3 的次级文字色 #475467

- **单元与截图：** U1 · `U1-configured-1440-light.png`。
  - 涉及 #667085 的文字：
    - Theme configuration 说明（y 368–455）；
    - Appearance 帮助文字（y 557–584）；
    - Brand color 帮助文字（y 686–713）；
    - 「Light 6 · Dark 6」（y 727）；
    - 「Resolved neutral: mauve.」（y 866）；
    - `U1-default-1440-light.png` 中的「Cancel pending writes」按钮文字。
  - 同一列里，副标题、「Matches preset」「Modified」和 pending 行用的是 #475467。
- **维度：** 视觉。
- **现状 → 期望：** 同一壳层里并存两种次级灰 → 次级文字统一为 #475467（G3）。
- **修法：** 字段 help 和 description 的 token 改为 `--shell-text-secondary: #475467`，删除 #667085 这一档。
- **可挽回分：** 0.1（视觉）。

### 新-9 · [P3] Overridden 状态行与下方解除按钮间距为 0px，与说明文字之间也只有约 2px

- **单元与截图：** U1 · `U1-configured-1440-light.png`，x≈20–260、y≈713–772。768、390、中文帧相同。
- **维度：** 布局、细节。
- **现状：** 胶囊下边框在 y=745，「Use configuration values · both modes」按钮上边框在 y=746，两者贴在一起。帮助文字末行与胶囊之间只有约 2px。
- **期望：** 壳层用细分隔线与分组行，栈间距 16px，组内间距要一致（G2）。
- **修法：**
  - 每个配置组内部用 `display: grid; gap: .5rem`，依次排列：label 行、选择器、帮助文字、状态行、解除按钮。
  - 状态行改为 `display: flex; align-items: center; gap: .375rem`，去掉「·」两侧多余的空格。
- **可挽回分：** 0.1（布局 0.05，细节 0.05）。

### UI-12（残留）· [P3] 自动配对的中性色名显示为小写原始键，中文态也不翻译

- **单元与截图：** U1 · `U1-configured-1440-light.png`（y≈873）；`U1-configured-768-light.png`；`U1-configured-390-light.png`（y≈836）；`U1-zh-configured-1440-light.png`（「已解析中性色：mauve。」，y≈749）。
- **维度：** 细节。
- **现状 → 期望：**
  - 现状：品牌色显示为「Pink」或「粉色」，自动中性色却显示「mauve」。
  - 期望：auto 显示实际配对的灰阶名（C2），呈现方式与选项列表一致；配置文案中英同步（G6）。
- **修法：** auto 结果用与中性色选项相同的显示名和 i18n 映射，英文为「Mauve」，中文用中性色选项中对应的译名。
- **可挽回分：** 0.1（细节）。

### 新-7 · [P3] 粘性顶栏下边框下方 17px 又有一条分隔线，形成双细线

- **单元与截图：** U1 · `U1-default-1440-light.png`（y=314 整列 #EAECF0，y=331 x20–299 #EAECF0）；`U1-default-768-light.png` 同位置；`U1-zh-top-1440-light.png`（y≈248、265）；`U1-zoom200-720-light.png`（y=449、≈482）。
- **维度：** 细节。
- **现状 → 期望：** 两条平行细线 → 一处分组只用一条细分隔线（G2）。
- **修法：** 滚动区第一个分组去掉上边框（`.edit-section:first-child { border-top: 0 }`）；或者顶栏只在内容滚动后才显示下边框（JS 判断 `scrollTop > 0` 时加 class）。
- **可挽回分：** 0.05（细节）。

**合计可挽回 1.9。** 分维度：布局 0.6，视觉 0.5，交互 0.3，细节 0.5。10 − 8.1 = 1.9，两者一致。

## 4. 建议（规范中没有直接依据，不扣分）

1. **顶栏和存储卡占用首屏过多。** 1440 下粘性顶栏高 314px，占视口 35%；390 下标签条加顶栏约 318px，占 38%。默认态的预设搜索从 y≈662（1440）或 y≈646（390）才开始，基础配置在首屏以下。C1 要求的顺序本身满足。建议做新-2 时一并把顶栏压成两行命令条，把「Memory only · not saved yet」这张琥珀色卡片收成主题名旁的一行状态，详情放进展开区。
2. **状态徽标借用了选中蓝。** Overridden、Custom 徽标用的是 G3 选中三色。这是按 R1 修法做的，所以不扣分，但与「蓝色用于选中与焦点」有冲突。可以考虑改用中性或琥珀色的状态色，例如底 #FFFAEB、边 #FEDF89、字 #B54708，并加一个图标。
3. **390 预览页顶部只有文字。** 只有「Graphite · Light preview」，没有模式切换、Save、Export。可以考虑在预览工具栏加一个 Light/Dark 分段控件。
4. **顶栏文档状态是纯灰字。** 「Matches preset / Modified」可以做成与 Custom 同族的小徽标，和基础配置区的状态表达一致。
5. **两个 18px 标题上下竞争。** 应用名「Rustify Theme Studio」和主题名都是约 18px。可以把应用名降为 14px/600，让主题名成为唯一的 18px 标题。
6. **中文态下 GPU 区仍是英文。** 「GPU · Cards」「Workspace」「Color tokens」等没有翻译。这属于 U5/P4 的范围，建议在那个单元核对。
7. **相邻输入框边框色不同。** 搜索框是 #98A2B3，紧挨着的 All sources 和 Built-in 选择器是 #858F9E，可以统一。

## 5. 看不出的格子

| 图 | 看不出什么 |
| --- | --- |
| R2 全部 U1 帧 | 字段标签页（Colors / Type & other）没出现，选中下划线、13px 半粗、标签页焦点、双分隔线（UI-11）都无法判断。 |
| `U1-keyboard-03-1440-light` | 品牌色列表只露出 8 项，26 项是否齐全看不出。中性色、外观、面板、圆角、缩放的选项列表都没展开；Radius、Scaling 两个字段在 R2 的 U1 帧里根本没出现（C2 选项集无法核对）。 |
| `U1-zoom200-720-light` | 顶栏内滚动区能否用指针或滚轮滚动看不出：headless 下没有滚动条，只能确认键盘焦点能带动它滚动。 |
| `U1-zoom200-export-720-light` | 代码框被对话框底边截住，Copy、Download 等动作在可视区外，200% 下能否到达看不出。 |
| zh 帧 | Host control 在预览底部，中文态是否翻译没拍到。 |
| 全部 | 只拍到 4 个键盘焦点帧（搜索框、品牌色选择器、展开列表、Dark）。Undo、Save、Export、EN/中文、Edit/Preview、渲染器按钮的焦点帧，以及悬停态、分隔条拖拽，都没有采集。 |

## 6. 总评

本轮只评 U1，得分 8.1，比 R1 的 6.8 高。R1 失分最多的几项已经关闭：浮签遮挡、顶部缺主题名和模式、语言切换被裁、下拉过窄、390 残带、覆盖标识缺失、200% 缩放不一致。

剩余失分主要来自两个地方：

1. **新粘性顶栏的结构问题。** 顶栏被限高为约 50vh 的内滚动区，200% 下截断文字，并把 G5 的控件藏起来（P1，0.55）。390 下顶栏上方还漏出滚动内容（P2，0.2）。

   这需要结构性修改，不能只调样式。顶栏要拆成「随内容滚走的引导段」和「紧凑的粘性命令条」两部分，去掉限高，并在高度过大时降级为不粘性。
2. **壳层控件缺少统一组件。** 次级按钮有 4 套配色，分段控件有 3 种外观，控件高度有 28、36、38px 三档，焦点环有 3 种透明度，闲置控件用焦点蓝做边框。这一部分合计约 0.7，需要收敛为两个组件：ShellButton 和 SegmentedControl，配一组共享 token（控件高度 2rem，边框 #D0D5DD，次级文字 #475467，焦点环 3px、透明度 40%、2px offset）。

其余各项（字重、#667085、间距、双细线、mauve 显示名）都是 P3 打磨，逐条可以修掉。
