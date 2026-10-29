# U3 Studio 导入与导出 · R4 独立评审原文

评审范围只有 U3。证据包括 r4 本层的 18 张 `U3-*.png` 和 `U3-actions.log`，对照 r3 的同名图和 `r3-judge-U3.md`。像素数值都是我用 pngjs 在原图上逐像素量出来的，坐标单位是 CSS px（DPR 1）。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.9 | 2.85 | 2.9 | 0.7 | **9.35** |

**各维度理由（为什么不是更高一档）**

- **布局 2.9**：所有对话框按钮行到下边框现在统一为 24px。差异表表头下的双线已去掉，Change 列徽标与表头对齐。不到 3 分有两个原因：
  - Import candidate 与 Changes 两张表上下叠放，但列位置不同：Current 列 x=405 对 728，Imported 列 x=812 对 971，左缘 233 对 241。
  - 候选区 Source 行比其他行矮 3px（29 对 32）。
- **视觉 2.85**：v1 损失清单和 Import notes 已回到 14px/21px，所有 token 与 G3 一致。不到 3 分的原因：
  - 同一对话框里有两套表头样式。候选区是 12px 常规 #475467、无底线；差异表是 13px/600、底线 #d0d5dd。
  - 6 个「Unchanged」与 1 个「Changed」徽标完全同样式，真正的差异不突出。
  - 备注里把整条路径排成等宽字，和表格里「组名用无衬线、token 用等宽」的写法不一致。
- **交互与状态 2.9**：合法 v1 审阅、未知版本错误、v2/v1/CSS/Rust 导出、两个 CSS 下拉展开、复制成功都走通。候选区已经有「Field | Current | Imported」图例，Name 行两边都有值。不到 3 分的原因：「Source: Theme JSON v1 · saved as v2」排在 Current 列下，读起来像是当前主题的属性。
- **细节 0.7**：千分位、Rust 说明的措辞、CSS 选择器、ui-serif 断行、备注命名都已修好。仍有若干像素级差异：差异表徽标基线比同行低 2px；390 候选区 Changed 徽标缩进 8px；Name 行两个值的基线差 1px；行距 29 对 32；备注路径的字体处理。

## 2. 上一轮差距复核表

| 编号 | R3 问题 | R4 状态 | 依据（r4 截图与坐标） |
| --- | --- | --- | --- |
| UI-128 | 候选区没有「当前 / 导入」图例；Version 行的「→」含义不同；Name 行只有导入值 | **部分解决** | `U3-import-review-1440` 在 y≈600 新增列头「Field / Current / Imported」。Name 行两边都有值（405「Rustify Light」，812「Imported legacy 旧主题」+ Changed）。相同的行标「Unchanged」，Changes (6) 计入了 name。残留问题：Source 行的值排在 Current 列（x=405），见下文差距清单 |
| UI-130 | 导出对话框按钮下留白 32px，导入为 24px | **已解决** | 留白一律 24px：`export-json` 519–542、`export-v1` 737–760、`export-css` 664–687、`export-rust` 709–732、`import-error` 599–622、`import-review-bottom-1440` 859–882、`import-review-bottom-390` 803–826、`export-json-390` 549–572 |
| UI-175 | v1 损失清单和备注正文降为 13px | **已解决** | `export-v1` 清单 7 行大写字母顶边为 177/198/219/240/261/282/303，行距 21px，字母 B 高 10px，即 14px/1.5。`import-review-bottom-1440` 备注两行行距 21px |
| UI-173 | 字节计数没有千分位，单位不一致 | **已解决** | 说明文字写「up to 262,144 bytes (256 KiB)」（y=88）。计数为「3,332 / 262,144 bytes」（1440 y=481，390 y=553），错误态为「22 / 262,144 bytes」 |
| UI-174 | Rust 说明有歧义，丢了「alive」 | **已解决** | `export-rust-1440` y=167：「Put theme.json next to theme.rs. … Keep the returned AppHandle alive while the UI is mounted.」 |
| UI-177 | 字体族值是比例字，390 下从「ui-」处断开 | **已解决** | `import-review-bottom-390` 第 1 行止于「Requested」，第 2 行以「ui-serif;」开头（y 686–698）。ui-serif、Noto Serif 都改成了等宽 |
| UI-176 | Changed 徽标右移 3px、基线偏低、表头下双线 | **部分解决** | 已修：徽标边框 x=572，与表头「Change」字首 x=572/573 对齐；y=274 只剩一条 #d0d5dd 线，y=275 起就是分组行底色 #f9fafb；390 差异卡片中徽标 x=49、token x=50、「Current」x=50。仍存在：徽标基线比同行文字低 2px。另发现 390 候选区徽标缩进 8px |
| UI-133 | 备注与差异表的 token 命名不一致 | **已解决**（命名部分） | 备注写「Light · base · font-serif」，表格写分组头「Light · base (1)」加行名「primary」（390 卡片也从「base.primary」改成了「primary」）。字体处理不一致另列为新-3 |
| UI-25 | 候选区两组行距不一致 | **部分解决** | 大写字母顶边：表头 596 → Source 628 → Name 657 → Appearance 689 → 720 → 752 → 784 → 816 → 848。只有 Source→Name 是 29px，其余都是 32px |
| UI-178 | CSS 提示里的选择器不可用 | **已解决** | `export-css-1440` y=230 写作 `[data-theme="dark"]`，与代码 y=510 一致 |

没有发现回归。另外，「Choose file」「Review changes」在 R3 是蓝字蓝边，R4 改成了中性描边。这一项不列为差距。

## 3. 差距清单

顺序为先按严重度，再按可挽回分。

### UI-128 · [P2] Source 行的值排在「Current」列下，读起来像是当前主题的属性

- **单元与截图**：U3 · `U3-import-review-1440-light`，行 y≈627–636。
- **维度**：交互与状态 0.07，细节 0.03。
- **现状**：列头 Current 在 x=405，Imported 在 x=812。「Theme JSON v1 · saved as v2」从 x=405 起排，落在 Current 列下，Imported 列这一行是空的。读者会以为当前主题是 Theme JSON v1。390 版是单列卡片（`U3-import-review-390-light` y 668–704），没有这个歧义。另外文案「saved as v2」中的 s 是小写。
- **期望**：版本必须清楚归属于导入候选（X1：先呈现候选差异——版本、名称、配置）。
- **修法**（任选其一）：
  - Source 行的 Current 格写当前文档格式（例如「Theme JSON v2 · current draft」），Imported 格写「Theme JSON v1 · saved as v2」；
  - Source 不再作为表格行，改为表格上方的 `&lt;caption&gt;` 或说明行，跨列（`grid-column: 1 / -1`），文案为「Imported file: Theme JSON v1 · will be saved as v2」。中文同步。
- **可挽回分**：0.10

### 新-1 · [P3] 同一对话框里两张差异表的列结构、列位置和表头样式不一致

- **单元与截图**：U3 · `U3-import-review-1440-light` y 595–607（候选区表头）；`U3-import-review-bottom-1440-light` y 252–274（Changes 表头）；390 版见 `U3-import-review-390-light` y 748–760 和 `U3-import-review-mid-390-light` y 515–523。
- **维度**：布局 0.05，视觉 0.07，细节 0.03。
- **现状**：

  | 项 | 候选区 | Changes 表 |
  | --- | --- | --- |
  | 表头样式 | 12px 常规 #475467，无底线 | 13px/600 #475467，底线 #d0d5dd（y=274） |
  | 列 x | Field 233 / Current 405 / Imported 812 | Field / token 241 / Change 572 / Current 728 / Imported 971 |
  | 状态列 | 徽标紧跟在值后面 | 独立的 Change 列 |

  两张表上下紧挨，Current 列错开 323px，Imported 列错开 159px，左缘差 8px。390 版也一样：候选区小标题「Current / Imported」是常规字重，差异卡片的同名小标题是 600。
- **期望**：表头属于 label，应为 13px/600（G3）；同一对话框的候选差异用一套结构呈现（X1）。
- **修法**：候选配置行改用与 Changes 相同的表格组件。列定为 `Field | Change | Current | Imported`，两表共用同一 `&lt;colgroup&gt;` 或 `grid-template-columns`，`th` 统一 13px/600 并加 #d0d5dd 底线，单元格统一 `padding-inline: 8px`。更简单的做法是并成一张表：「Identity &amp; configuration」作为第一个分组行，下面接现有的 token 分组。390 卡片小标题统一用 600。
- **可挽回分**：0.15

### UI-176 · [P3] 徽标基线偏低 2px；390 候选区徽标缩进 8px；Name 行两边基线差 1px

- **单元与截图**：U3 · `U3-import-review-bottom-1440-light`、`U3-import-review-390-light`、`U3-import-review-1440-light`。
- **维度**：布局 0.02，细节 0.12。
- **现状**：
  - **差异表基线**：同一行文字与徽标「Chan」的基线位置如下，徽标一律低 2px。
    - id 行：332 对 334；
    - name 行：373 对 375；
    - Light·base primary 行：451 对 453。
  - **390 候选区缩进**：Name 行的 Changed 徽标边框 x=207，同列的「Imported」小标题和值都在 x=199（y 784–807）。原因是值换行后，徽标的左外边距仍然保留。
  - **1440 Name 行基线**：「Rustify Light」基线 665，「Imported legacy 旧主题」和其徽标基线 666，差 1px。
- **期望**：徽标与同行文字基线对齐，与列首对齐（R3 已立的标准，细节维度要求逐像素一致）。
- **修法**：
  - 徽标设为 `display:inline-flex; align-items:center; height:24px; line-height:1`；
  - 所在 `td` 与同行其他 `td` 统一 `vertical-align: middle`，文本格 `line-height: 24px`；或者全部用 `baseline`，并去掉徽标的上下 padding 偏移；
  - 值和徽标包进 `display:flex; flex-wrap:wrap; column-gap:8px; align-items:center`，删掉徽标的 `margin-left`，这样换行后徽标贴着列左缘。
- **可挽回分**：0.14

### 新-2 · [P3] 6 个「Unchanged」与 1 个「Changed」徽标同样式，且左缘参差

- **单元与截图**：U3 · `U3-import-review-1440-light` y 651–866；`U3-import-review-mid-390-light` y 33–346。
- **维度**：布局 0.02，视觉 0.05，交互 0.03。
- **现状**：
  - 两种徽标完全同样式：底 #f9fafb、边框 #d0d5dd、字 #475467、高 24px。唯一的差异「Changed」（x 969–1030）淹没在 6 个同样醒目的「Unchanged」里。
  - Unchanged 徽标紧跟值文字的宽度，左缘依次为 456（Inherit）、461（Custom ×4）、451（100%），三种 x。
- **期望**：导入先突出候选差异，未变化的项应弱化（X1）。
- **修法**：
  - 状态放进新-1 所说的独立 Change 列，列内左对齐；
  - 「Unchanged」改为纯文字 caption 12px #475467，不加边框和底色（或写「—」）；
  - 只有 Changed/Added/Removed 保留带边框的徽标。不要把差异染成蓝色，G3 规定蓝色只用于选中和焦点。
- **可挽回分**：0.10

### 新-3 · [P3] Import notes 把整条路径排成等宽字，与表格写法不一致

- **单元与截图**：U3 · `U3-import-review-bottom-1440-light` y 764–794；`U3-import-review-bottom-390-light` y 664–740。
- **维度**：视觉 0.03，细节 0.05。
- **现状**：「Light · base · font-serif」整段用等宽字，中点两侧的空格各占 1ch，间隔约为无衬线字体中的两倍。等宽字（约 13px）也明显比同行 14px 的「Requested … preview uses」小一号。同一对话框里，表格把同一路径写成无衬线 600 的分组头「Light · base」加等宽的 token「primary」。
- **期望**：备注与差异表用同一套路径排法。这一要求来自 UI-133 的修复口径；G3 只要求颜色值和标识符用等宽。
- **修法**：输出 `&lt;span&gt;Light · base · &lt;/span&gt;&lt;code&gt;font-serif&lt;/code&gt;`，模式和分组用正文字体，只有 token 用等宽。行内 `code` 设 `font-size: .9286em`（13/14），保证 x-height 接近。中文同步。
- **可挽回分**：0.08

### UI-25 · [P3] Source 行比其余候选行矮 3px

- **单元与截图**：U3 · `U3-import-review-1440-light`，label 大写字母顶边 596 / 628 / 657 / 689 / 720 / 752 / 784 / 816 / 848。
- **维度**：布局 0.01，细节 0.05。
- **现状**：Source→Name 为 29px，其余各行都是 32px。原因是 Source 行没有 24px 高的徽标，行高被内容撑得更矮。
- **期望**：整组只有一个行距（R3 的修法口径）。
- **修法**：候选行设 `grid-auto-rows: minmax(2rem, auto); align-items: center`，或给每行 `min-height: 2rem`；不要靠徽标撑高行。按新-1 并表后，统一用 `td` 的 `height`。
- **可挽回分**：0.06

**合计**：0.10 + 0.15 + 0.14 + 0.10 + 0.08 + 0.06 = **0.63**，与 10 − 9.35 = 0.65 相差 0.02。

按维度核对：

| 维度 | 失分 | 各条差距的该维度分合计 |
| --- | --- | --- |
| 布局 | 0.10 | 0.05 + 0.02 + 0.02 + 0.01 = 0.10 |
| 视觉 | 0.15 | 0.07 + 0.05 + 0.03 = 0.15 |
| 交互与状态 | 0.10 | 0.07 + 0.03 = 0.10 |
| 细节 | 0.30 | 0.03 + 0.03 + 0.12 + 0.05 + 0.05 = 0.28 |

## 4. 建议（不扣分）

1. **导出代码换行**（沿用 R2/R3）：JSON 导出改为 2 空格缩进，代码框 `white-space: pre`，加横向滚动。390 下问题更明显：`U3-export-json-390` y≈203–300 在键名中间（`"chart-` / `1"`）和字符串中间（`"background":"` / `#f0f0f0"`）断行。CSS 导出 y=552–615 也把 `IBMPlexSans-` / `Text.ttf`、`*/` 拆开了。
2. **复制状态**：「Copied to clipboard.」出现后，对话框从下边 733 长到 759，向下跳 26px。可以给状态行预留高度，或把它放进按钮行作为行尾项（R3 修法的另一个选项）。X5 只要求「实际成功后才报成功」，没有规定位置。
3. **v1 损失清单顺序**：按 X2 的列举顺序、也与配置区一致，排成 Brand → Neutral → Appearance → Radius → Scaling → Override sources → Derived roles。现在两类条目交错排列。
4. **沿用 R3 的其余建议**：
   - 对话框标题吸顶，Apply / Cancel 吸底；Cancel 改为描边次级按钮；
   - Rust 页的「Download」改名为「Download theme.rs」；0.2.1 拒收单独成行警示；补普通 Cargo 构建命令；
   - 差异表的颜色值旁加色块；
   - 「To match this preview」后用文字标出预览模式（例如「Preview: Light」）；
   - 对话框本体加底部渐隐，提示下面还有内容。

## 5. 看不出的格子

- **中文**：U3 所有帧都是英文。新文案（千分位、Source、Unchanged、Rust 说明）的中文版本无法判断，G6 也没有在 U3 帧上被验证到。
- **下载与失败路径**（X5）：没有拍下载实际发起后的反馈，也没有拍复制或下载失败时保留可选文本与下载路径的状态。
- **键盘焦点**：capture.md 记了焦点所在元素，但这些焦点都是鼠标点击得到的，截图中没有可见焦点环。对话框内的焦点可见性和 Escape 行为都无法判断。
- **X1 的其余分支**：候选只有「修改」，没有 base/override 的新增或删除行；语法错误、超限、不支持的颜色输入这几种状态都没拍。「取消后保留当前主题与历史」也只能从背景主题名仍是 Rustify Light 间接看出，看不到历史。
- **Rust 字体资源清单**（X4）：代码共约 66 行，三帧（顶、中点 scrollTop 531/1061、末尾）大约覆盖第 1–15、26–41、51–66 行，中间约第 16–25、42–50 行没拍到。代码注释写了「Copy the listed same-origin font files」，但这些字体文件是否真的在 Rust 正文里列出，看不出来。CSS 导出里能看到 `makepad_widgets/resources/*.ttf`。
- **`U3-import-review-end-1440/390`**：脚本用滚动到 Apply 的方式定位，Apply 按钮正好贴着对话框下边框（1440 下 850–882 对 883；390 下 813–826 对 827）。这是采集方式造成的，留白以 `-bottom` 帧为准。
- **X6 高度的口径**：导入审阅对话框是 17→883，约等于视口减 2rem；导出和错误态对话框按内容取高（例如 17→543）。我按「最大宽高」理解 X6，与 R3 口径一致，不扣分。如果负责人认为 X6 要求固定高度，导出和错误态都需要另行判断。

## 6. 总评

- **分数**：U3 为 **9.35**（R3 采用 9.05，另一评审 9.3）。R3 的 10 条差距里，7 条已解决（UI-130、175、173、174、177、133、178），3 条部分解决（UI-128、176、25）。
- **主要扣分**：这次修复把候选区改成了三列对照，但没有复用 Changes 表的结构，带来三个新问题：
  - 两张表列位、表头样式和状态列位置不一致；
  - Unchanged 与 Changed 徽标同等醒目；
  - Source 行落在 Current 列下。
- **需要的结构性改变**：只有一项，属于局部、小范围——把候选配置行并入或复用 Changes 的表格组件（统一 `colgroup`、`th` 样式和独立的 Change 列）。改完后，新-1、新-2、UI-25 以及 UI-176 的大部分会一起关闭。其余都是基线、外边距、文案这类打磨。
