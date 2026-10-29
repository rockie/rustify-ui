# U7 布局与排版 · R2 评审

U7 本轮 **7.1 分**，R1 是 5.2。U7 只有一个单元，所以它也是最低分单元。

R1 指出的主要缺口大多补上了：
- 字级梯度、Code/Kbd 变体矩阵、wrap/trim 卡片和 Container 全档都已出现。
- Inset 贴边做出了可见的对照。
- Reset 有了原生元素与重置后的对照。

剩下两个问题拉低了分数：
- **字重整体失效**：Heading 不粗，Text 的四档字重渲染相同，Strong 不加粗。
- **非法属性后看不到任何诊断。**

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 2.5 | 1.8 | 2.3 | 0.5 | **7.1** |

- **布局 2.5**：做对的部分：
  - Container 四档实测为 448、688、880、1136px，左、中、右对齐对称。
  - Inset 在 top、bottom 两侧贴边。
  - 390 宽下没有溢出。
  - 240px 的显式尺寸在 110% 和 95% 缩放下不变。

  没给 3 分的原因：非法属性样例在默认态塌成一列一个词，上方还有一个孤立的 `px`；Heading 的 wrap 行、Inset 左右贴边和 Reset 的元素覆盖面都没有达到目标图的结构。
- **视觉 1.8**：做对的部分：
  - 字级梯度与目标的偏差在 1–3px 内，差异来自字族。
  - 已实测的行高（Heading 4–9 档、Blockquote）都与目标一致。
  - Code 四种变体矩阵、Kbd 的 classic 斜面与 soft 外观、Blockquote 边框随字号变化、Quote 的衬线斜体都正确。

  没给 2 分的原因：radix-sink-heading 整张图的主特征是粗体，而 Heading 只比正文略粗；Text 的 light、regular、medium、bold 四行完全一样；Strong 也没有加粗。
- **交互与状态 2.3**：缩放 110% 和 95% 都只生效一次，字号与行高按比例变化；wrap、trim、truncate 都有呈现。没给更高分的原因：非法属性态是 U7 四个状态之一，它的核心要求「属性定位诊断」完全没有出现；pretty 的效果也看不出来。
- **细节 0.5**：有几处小差异：
  - 顶边的外来灰线还在。
  - 有孤立的 `px` 文本。
  - 布局原语区的样例没有 prop 标注。
  - 390 宽下 Kbd 键帽上下相贴。

## 2. 上一轮差距复核

| 编号 | 上一轮问题 | 本轮结论 | 依据（R2 截图） |
| --- | --- | --- | --- |
| UI-86 | 夹具缺目标图的主体 | **部分解决** | Heading、Text、Code、Kbd 的 1–9 档、变体矩阵、wrap 六卡和 trim 四卡都已出现。剩下的问题：Heading wrap 行没有收窄宽度，1280 宽下 7–1 档只有一行，1–3 档在两个视口里都没换行（转入下方 UI-86 余项） |
| UI-87 | 看不到 Inset 贴边 | **部分解决** | `U7-layout-inset-demonstration-*` 的卡片有可见边框，top 和 bottom 两侧的填充贴住边框与圆角。left、right、x、y、all 只出现在带 `margin=3px` 的样例里，填充离边框 3px，没有贴边的展示 |
| UI-88 | Heading 不是粗体 | **仍存在** | 60px 字号下 Heading 的笔画宽 7px，Text 是 6px。目标里 Heading 是 9px，Text 是 5px |
| UI-89 | Strong 没加粗 | **仍存在** | `U7-type-code-matrix-1280` 中「strong importance」与相邻文字的墨量相同（0.8595 对 0.8577）；整段夹具里的「Strong importance」也是常规字重 |
| UI-90 | Code、Kbd 不是默认变体 | **已解决** | 行内 `code` 是 soft 外观（标注「variant=soft (default)」）；`Ctrl+K` 和 `Enter` 是带边框和底部阴影的 classic |
| UI-91 | 非法属性后看不到诊断 | **仍存在** | 默认态与 `U7-invalid-1280` 逐像素比对，唯一的变化是该样例重新排版（整图从 8281px 变为 8146px 高），夹具里没有新增任何文字；`U7-invalid-detail` 视口里也没有诊断 |
| UI-92 | Reset 无法看出作用范围 | **部分解决** | 现在有 button、h2、ul 三组「原生 / Reset」对照，有虚线标记，并注明不加包裹层。目标 radix-reset 覆盖约 60 种原生元素，现在只有 3 种 |
| UI-93 | Container 只看得到一档 | **已解决** | 四档宽度与 left、center、right 对齐的实测值全部正确，左右对称 |
| UI-94 | Quote 不是斜体 | **已解决** | Quote 是衬线斜体，字号略大于正文 |
| UI-95 | 样例没有标注 props | **部分解决** | 排版各段与 Container、Inset、Reset 都有标注。布局原语区（夹具顶部约 y=120–1500）仍有多处没有标注 |
| UI-96 | Blockquote 左边框固定 4px | **已解决** | size 2 和 size 4 的边框是 4px，size 7 是 7px；左内边距分别为 12、12、14px |
| UI-97 | 夹具顶边混入 1px 灰色线段 | **仍存在** | `U7-fixture-1280-light.png` 第 0 行，x=138–288 和 x=295–374 有 rgb(118–161) 的线段。390 宽的图上没有 |

## 3. 差距清单

### 新-1 · [P1] Text 的四档字重渲染完全相同（视觉 · 可挽回 0.5）

- **位置**：`U7-type-text-matrix-1280-light.png`，y=537–666 的四行 weight=light、regular、medium、bold；390 宽的图同样如此。
- **现状 → 期望**：四行的墨量分别为 0.8541、0.8541、0.8541、0.8536，放大后笔画也看不出区别。期望是 light、regular、medium、bold 依次加重（规范 L2 的字重四档，比较口径「字重」）。
- **修法**：
  1. 先在浏览器里看这四行的 computed `font-weight`。
  2. 如果四行数值相同：说明字重 class 没有生成或没挂上。补齐组件字重 class 对应的规则后运行 `mbx xtask css`，并提交 `rustify.css`。
  3. 如果数值是 300、400、500、700 但渲染相同：说明当前主题字体只加载了一个或两个字面。为 IBM Plex Sans 补齐 300、400、500、700 四个 `@font-face`（woff2 放在 public 静态目录，按 URL 引用），或换成支持 `font-weight: 300 700` 的可变字体。
  4. 修完后 UI-88 和 UI-89 应该一起恢复，三处一起验收。

### UI-88 · [P1] Heading 不是粗体（视觉 · 可挽回 0.5）

- **位置**：`U7-type-heading-matrix-1280/390`、`U7-scale110/95-heading`，以及夹具里的「Native structure, flexible composition」。
- **现状 → 期望**：Heading 的笔画只比 Text 粗 1px，接近 medium。目标 radix-sink-heading 的所有行都是粗体，笔画宽约为正文的 1.8 倍（规范 L1「正确语义外观」，比较口径「字重」）。
- **修法**：Heading 的默认字重用 bold（700），并确认 700 字面真的能加载，做法同新-1。验收方法：在 60px 字号下，Heading 的笔画宽应约为 Text 的 1.5 倍以上。

### UI-89 · [P1] Strong 没有加粗（视觉 · 可挽回 0.2）

- **位置**：`U7-type-code-matrix-1280` 行内句子中的「strong importance」；夹具里的「Strong importance」。
- **现状 → 期望**：现在是常规字重。目标 radix-sink-code 段落里的「greyscale antialiasing method」是明显粗体。
- **修法**：Strong 使用 bold 字重，字面来源同新-1。

### UI-91 · [P2] 非法属性后看不到属性定位诊断（交互 · 可挽回 0.5）

- **位置**：`U7-invalid-1280-light.png`、`U7-invalid-detail-1280-light.png`。
- **现状 → 期望**：点 Invalid prop 后，只有「A bad prop keeps this view alive.」从一列一词变成一整行，页面上没有任何提示。期望页面写明是哪个属性、什么值非法、改用了什么默认值，同时组件保持可用（规范 L6）。
- **修法**：在该样例下方渲染 `role="status"`（`aria-live="polite"`）的诊断行，内容形如「`width`: invalid value "…", using default」，用组件错误色。点 Restore prop 后诊断行消失。

### 新-2 · [P2] 非法属性样例在默认态塌成一列一个词，上方有孤立的 `px`（布局 0.15 + 细节 0.1 · 可挽回 0.25）

- **位置**：
  - `U7-fixture-1280-light.png` 约 y=1395–1590：「px」单独一行，下面是「A / bad / prop / keeps / this / view / alive.」七行。
  - `U7-fixture-390-light.png` 约 y=1940–2120：同样如此。
  - `U7-invalid-*` 里「px」仍在。
- **现状 → 期望**：默认态是合法属性，本该是正常可读的排版，标注出当前 prop 的值；现在看起来像宽度塌缩的故障，「px」也像格式化时丢了数字（规范 L5「容器边界不遮住内容」、L6）。
- **修法**：
  - 默认态给该 Box 一个合理的合法值（例如 width=240px），并用和其他段相同的灰色标注写出 `width=240px`。
  - 「px」要么补全成「<数值>px」，要么删除。

### UI-92（余项）· [P2] Reset 只覆盖 3 种元素（布局 0.1 + 交互 0.1 · 可挽回 0.2）

- **位置**：`U7-layout-reset-comparison-1280/390`。
- **现状 → 期望**：现在只有 button、h2、ul 三组对照。radix-reset 演示了约 60 种原生元素，包括：
  - 行内语义：Bold text、Emphasized text、Computer code、Keyboard input、Marked、Sub/Superscripted；
  - 表单：Input control、Select、Multi-line textarea、fieldset/legend；
  - 列表：dl/dt/dd、ol；
  - 其他：Table、Preformatted text、Block quote、图片。

  比较口径是「Reset 的作用范围」（规范 L4）。
- **修法**：沿用现在的「原生 / Reset」双行结构，至少补上 a、strong、em、code、kbd、mark、sub、sup、input、select、textarea、fieldset、ol、dl、table、pre、blockquote、img。每组只给 Reset 那一个元素加虚线标记，相邻元素保持浏览器默认样式。

### UI-95（余项）· [P3] 布局原语区仍有样例没有 prop 标注（细节 · 可挽回 0.15）

- **位置**：`U7-fixture-1280` 约 y=120–1380。没有标注的样例：
  - 「A semantic span starts as a block.」和蓝色条；
  - 右侧孤立的「Shared fields」灰框（x=1040–1200），下方有大片空白；
  - 「Third / Second / First」；
  - 「One–Five」；
  - 「Semantic section spacing」灰带（只看得出约 64px 的纵向内边距，看不出 size）；
  - 「Theme space scales once.」。
- **现状 → 期望**：排版各段已经用灰色小字写出 `size=…`、`align=…`，这一区却看不出 Box、Flex、Grid、Section 各自用了什么属性。
- **修法**：给每个样例加同款标注（例如 `Section size=3 · padding-y=64px`、`Flex direction=row-reverse (md+)`、`Grid columns=3`）。Section 补齐 24、40、64、80px 四档（规范 L3）。

### UI-86（余项）· [P3] Heading 的 wrap 行没收窄宽度，小字号的多行行高无法比对（布局 · 可挽回 0.1）

- **位置**：`U7-type-heading-matrix-1280` 的 size=7·wrap 到 size=1·wrap 都只有一行；390 宽下 3–1 档也只有一行。
- **现状 → 期望**：目标 radix-sink-heading 下半段每档都换行：9 档 3 行，8–5 档 2 行，4 档 4 行，3 档 3 行，2 档 2 行，1 档 4 行，用来展示每档行高。已实测部分全部吻合：60、40、36、30、26、24px。1–3 档的 22、18、16px 还没有被展示。
- **修法**：给每个 wrap 行一个固定 CSS 像素的 max-width，按目标图让每档都至少换成两行。用 px 不用 em，这样缩放时仍能验证「显式尺寸不随缩放变化」。

### UI-87（余项）· [P3] Inset 的 left、right 贴边没有干净的展示（布局 · 可挽回 0.1）

- **位置**：
  - 贴边演示段只有 side=top 和 side=bottom。
  - `U7-fixture-1280` 约 y=1015–1320 的 8 个 side 样例都带 `margin=3px`。以 side=all 为例，卡片边框在 x=15、y=1037，填充从 x=19、y=1041 开始，离边框 3px。
- **现状 → 期望**：目标 radix-inset 里，Checkbox Card 的 side=left 图片和 Radio Card 的 side=right 图片都贴住卡片边框和圆角。
- **修法**：在演示段补 side=left 和 side=right 两张卡片（一张横排、贴左；一张贴右）。卡片和 Inset 都不加额外 margin，Inset 内用对比色填充或 public 目录里的图片。

### UI-97 · [P3] 夹具根顶边混入外来灰线（细节 · 可挽回 0.1）

- **位置**：`U7-fixture-1280-light.png` 第 0 行，x=138–288、295–374。
- **修法**：
  1. 查出夹具根上方哪个元素的底边压进了根元素 1px，可能是 catalog 控件或标签的下划线。
  2. 给夹具根加上外边距或隔离层，让 `catalog-layout-fixture` 的包围盒不和它相交。
  3. 修完后在 1280 宽下重新截图确认第 0 行是纯底色。

### 新-5 · [P3] wrap=pretty 与 wrap=wrap 的折行完全相同（交互 · 可挽回 0.1）

- **位置**：`U7-type-wrap-matrix-1280/390` 的 pretty 卡和 wrap 卡，两者都折成「…meaning / and a reader's changing viewport.」。
- **现状 → 期望**：目标 radix-text-wrap 里，wrap 的末行只剩「sojourn.」，pretty 的末行是「another sojourn.」，能看出孤词被避免。
- **修法**：换一段在当前卡宽下 wrap 会留下单词孤行的文案，或者把这一行卡片收窄，让 pretty 的效果看得出来。

### 新-4 · [P3] 390 宽下 Kbd 的 size 9 行换行后键帽上下相贴（布局 · 可挽回 0.05）

- **位置**：`U7-type-kbd-matrix-390-light.png`，size=9 行（约 y=172–345）。
- **现状 → 期望**：「/」和「X」上下堆叠，间距约 1px；soft 两键看起来像一个高键。目标里键帽之间都有约 8px 间隔。
- **修法**：给夹具里放键帽的 Flex 加上行间距与列间距（8px），或在 390 宽下让 classic 和 soft 两列上下排列，使单行能放下。

**可挽回分合计**：视觉 1.2，交互 0.7，布局 0.5，细节 0.35，共 2.75。10 − 7.1 = 2.9，差 0.15，在 0.5 的允许误差内。

## 4. 建议（不扣分）

- Heading、Text、Blockquote 的目标图里有色族行（Red、Yellow、Green、Gray）以及嵌套 high-contrast 的行；比较口径没有列入颜色，可以按目标补上。
- Em 现在是无衬线斜体，Quote 是衬线斜体。目标里两者都是衬线斜体，Em 约放大 1.18 倍。字族不在比较范围内，但两者应保持一致。
- Kbd 每行可以按目标补上「⇧⌘V」符号键，展示多字符键的内边距。
- 顶行「Enter / Tab / Shift+Tab」之间只有约 2px，目标约 15px。
- Container 可以按目标加一个嵌套样例：size 4 里嵌一个 size 1。
- 「margin=3px」样例应说明那 3px 间隙是有意为之，否则容易被读成 Inset 没贴边。
- 「Your name」标签和输入框之间几乎没有间距。
- Code 表格的行高约 64px，目标约 44px；行标签比代码块高约 3px。
- wrap 卡的内边距是 16px，目标是 24px（Card 不是 P1 组件，仅供参考）。

## 5. 看不出的格子

- **Heading 1–3 档的多行行高**：两个视口里这三档都只有一行。
- **wrap=pretty**：文案里没有孤词，看不出效果。
- **非法属性态**：看不出设了非法值的是哪个属性、什么值、回退到了什么默认值；Restore prop 没拍到。
- **Section 的另外三档**（24、40、80px）：只有一条没标注的灰带。
- **Reset 按钮旁的「0」**：没拍点击，看不出计数是否工作。
- **六个响应式断点**：只拍了 390 和 1280，看不出 520、768、1024、1640 下的表现。
- **Kbd classic 的浅色斜面**：目标只有深色，只能按同一规则推断，没有可直接比对的浅色目标。

## 6. 总评

U7 是本单元唯一被评的单元，得 7.1。主要失分是视觉（字重）和交互（非法属性诊断）。

需要结构性修改的有两处：
1. **字体字重链路**：先判定是字重 class 缺失还是字体只加载了一两个字面，再补齐 300、400、500、700。修好后 Heading、Text 字重和 Strong 应同时恢复，可回收约 1.2 分。
2. **非法属性诊断**：要新增一个可见的诊断区，同时把默认态改成正常、带标注的合法样例，可回收约 0.75 分。

其余都是在现有夹具上补样例、补标注：Heading wrap 收窄、Inset 左右贴边、Reset 扩展元素种类、pretty 文案、390 下 Kbd 的间距、顶边灰线。不需要推倒重做。
