# DOM-COMPONENTS-P1 UI 验收 R1：U5、U7 评审

依据：目标包 `DOM-COMPONENTS-P1.ui-target.md`（U5 对应 P1–P5、G1，U7 对应 L1–L6 和 9 张 radix 目标图），`r1/capture.md`，`U5-actions.log`，`U7-actions.log`，以及下面「开图清单」里列出的截图。按 capture.md「未采与补采」的说明，U5 拾取态以 `U5-inspector-1440-light-a2.png` 及它前后的关键帧为证据，原图 `U5-inspector-1440-light.png` 只作旁证。U7 的数值是在 `scratchpad/judge-C2/` 下用 magick 裁剪、采样得到的，像素坐标以原图为准。

---

## 1. 评分表

| 单元 | 布局 (0–3) | 视觉 (0–3) | 交互与状态 (0–3) | 细节 (0–1) | 总分 |
| --- | --- | --- | --- | --- | --- |
| U5 预览 | 2.0 | 2.1 | 1.75 | 0.5 | **6.35** |
| U7 布局与排版 | 1.6 | 1.0 | 2.1 | 0.5 | **5.2** |

**U5 各维度为什么没到更高一档**
- 布局 2.0：结构是对的。工具栏下是 DOM / DOM+GPU 并排，390 宽时上下堆叠，并有「Back to edit」。没到 3 分，是因为四处一眼可见的偏差：深色预览被拆成两块没有内边距的深色岛，中间露出浅色缝；菜单相对触发按钮水平偏了约 113px；390 宽时对话框离屏幕边只有 1px；固定定位的「Host control」压在内容上。
- 视觉 2.1：做对的部分有两点：编辑壳一直是中性浅色，作者色只出现在 scope 内（符合 G1、P4）。没到 3 分的原因：深色 scope 里的文字贴着色块边；选中和未选中的渲染器 / 模式按钮只差 #EFF8FF 与 #F9FAFB 两种底色；对话框没有遮罩。
- 交互 1.75：菜单、对话框、对比区切 Dark、已声明样本的拾取、GPU Ready 都走得通。没到 2 分，是因为 P2 明确要求的「未标注」态完全没有：悬停和点击都没有反馈，点击还穿透，真实执行了动作（「Create · 0」变成「Create · 1」）。另外，拾取后来源说明不在视野里。
- 细节 0.5：小差异有几处：拾取标签压住上一行文字的下伸部；GPU 色样标签写「fg」，与 token 名不一致；DOM、GPU 两列的起点差 22px；「Local demonstration」贴着深色块边。

**U7 各维度为什么没到更高一档**
- 布局 1.6：Container 宽 1136px，在三档缩放下不变；Section 纵向间距约 64px（灰带 150px 减一行文字后除以 2）；390 宽无溢出，响应式改向正确。没到 2 分，是因为目标点名比较的「Inset 贴边」在截图里不存在：唯一有可见容器的样例四周各留约 32px，8 个 side 样例都没有可见容器。Reset 没有对照样例；Container 其他档位和对齐方式看不到。
- 视觉 1.0：Blockquote 与目标一致（4px 竖条在 x=16–19，文字从 x≈36 起，与 radix-sink-blockquote 中间那条的度量相同），Em 是斜体，也对。只得 1 分，是因为目标图的主体内容（Heading、Text、Code、Kbd 的 9 档梯度和变体矩阵）在夹具里没有。默认 Heading 和 Strong 都不是粗体；Code、Kbd 各只有一个样例，外观也不是 L1 规定的默认变体；Quote 没有斜体。
- 交互 2.1：缩放成比例。大标题行带在 100/110/95% 下分别为 58/64/55px，「Text follows…」的宽度分别为 407/447/387px。调用方写死的字号（「A caller utility…」，行带一直是 11px）、1136px 的 Container、160px 的 Shared fields 都不随缩放变化，符合 L6。非法属性之后组件仍然可用。没到 3 分，是因为页面上看不到属性定位诊断，wrap、trim 这些状态也没有呈现。
- 细节 0.5：小差异有几处：样例都没有标注 props；Reset 按钮比列左缘多缩进 8px、蓝字，没有任何说明；Container 说明文字留下孤词「layer.」；Quote 没有放大；Kbd 缺 classic 的键帽斜面。

---

## 2. 上一轮差距复核

首轮，略。

---

## 3. 差距清单

编号作为本轮基线，下一轮沿用。排序先按严重度，再按可挽回分。

### P1

**U7-1 · 夹具缺目标图的主体：字级梯度、字重和变体矩阵**
- 截图：U7 全部截图的排版区（1280 原图 y≈1668–2335）
- 维度：视觉 0.6 + 布局 0.3 + 交互 0.3。可挽回 **1.2**
- 现状：排版区只有这些：1 个大标题、1 个 24px 标题，正文 16/14/约 12px 三种，Code、Kbd、Blockquote 各 1 个，以及 1 个 truncate（「This long sentence is …」）。没有 Heading 1–9、Text 1–9；没有 light/regular/medium/bold 四档字重；没有 Code 的 4 种变体 × 4 种色彩 × 9 档尺寸；没有 Kbd 的 classic/soft × 9 档；没有 wrap/nowrap/balance/pretty；没有 trim。
- 期望：
  - radix-sink-heading：单行 9 档，加一组限宽换行的 9 档，用来看行高；
  - radix-sink-text：9 档；
  - radix-sink-code：4×4 矩阵、段落内联、9 档；
  - radix-sink-kbd：classic/soft × size 9→1，以及行内的 Enter / Tab / Shift + Tab；
  - radix-text-wrap：6 张卡片；
  - 目标包 U7 的比较口径是「字级梯度、字重、行高、Code/Kbd/Blockquote 的外观」；另见 L1、L2。
- 修法：在 component-catalog 的 `mount_layout_fixtures()` 里，按目标图逐段加 sink 区块：
  - Heading 1–9 单行一组，再放一组 `max-width: 448px` 的换行组；
  - Text 1–9；
  - Text 的 weight light/regular/medium/bold 各一行；
  - Code 做成表格：行为 solid/soft/outline/ghost，列为 color、+high-contrast、gray、gray+high-contrast，另加 size 1–9 一列，再加一段含内联 Code、Kbd、Em、Strong、Quote 的段落；
  - Kbd 做 classic/soft 两列 × size 9→1；
  - Blockquote 做 3 个尺寸；
  - wrap 做 6 张卡片：宽约 362px，1px 描边，内容依次是 Default、wrap、nowrap、balance、pretty、truncate；
  - trim 的 normal/start/end/both 各一个，加浅底，让裁掉的行距看得见。
  - 每行行首用 12px 灰字写出 prop 值。

**U5-1 · 深色预览 scope 被拆成没有内边距的深色岛**
- 截图：U5-dom-1440-dark、U5-compare-nested-1440-light
- 维度：布局 0.4 + 视觉 0.4 + 细节 0.1。可挽回 **0.9**
- 现状：
  - 作者背景只画在两个矩形上。第一个在 x=344–1416、y≈160–525，包括「Local demonstration…」这一行和两张卡片；第二个从 y≈576 开始。中间 y≈526–575 露出壳层的浅底 #F9FAFB，「Provider & overlay checks」摘要就落在这片浅底上。
  - 「Local demonstration · changes stay in this preview.」的文字左缘和深色块左缘都在 x=344，内边距为 0。
  - 卡片描边和深色块边缘重合，卡片圆角外面露出深色的方角。
  - 对比区也一样：深色块在 y≈437–801，文字贴边，它自己的「Provider & overlay checks」摘要落在浅底上。
- 期望：P3 要求嵌套边界可选、可辨，修改不越过边界；G1 要求预览使用作者主题、编辑壳保持中性。边界内的文字不应贴边，作者主题区应是一块连续的面。
- 修法：
  - 把作者的 background、foreground 和内边距统一放在包住整段场景的那一个 ThemeScope 根上：`background: var(--background)`，`padding: 24px`（与预览区 padding 同档），圆角取作者 radius，加 1px 的 border token。
  - 把「Local demonstration」说明和「Provider & overlay checks」disclosure 都移进这个根，不再给子块单独画背景。
  - 对比区的 ThemeBoundary 同样处理，把它自己的 disclosure 也包进去。
  - 浅色模式也走同一结构。

**U7-2 · 截图里找不到 Inset 贴边**
- 截图：U7-fixture-1280 原图 y≈853–1310，110%、95%、390 同样
- 维度：布局。可挽回 **0.6**
- 现状：
  - 唯一有可见容器的 Inset 样例是「Nearest padding is consumed once.」。外框在 x=16–315、y=853–977（底色 #EFEFF1），内块在 x=48–284、y=885–946，四周各留约 32px，没有一边贴到外框。
  - all/x/y/top/bottom/left/right/responsive 这 8 个 side 样例只有文字，容器既没底色也没描边。只能看出文字缩进是 0 还是 16px，看不出内容有没有贴到容器边。
- 期望：radix-inset 中，卡片里的图片与卡片左、右、下边完全贴合，Checkbox/Radio Card 的图片贴到左/右缘。L3：「Inset 只消除最近容器声明的 padding，使内容贴边」。
- 修法：
  - 加一个与目标同构的样例：一个带 1px border、圆角、`padding: 24px` 的 Box 当卡片，里面放 Inset side="top" 和 side="bottom"，内容用实色块或 public 目录里的图片（通过 URL 引用，不嵌入 wasm）。色块的上/下边和左右边应与卡片描边重合。
  - 8 个 side 样例都给容器加 1px 描边，给 Inset 子元素加浅色底，让哪几边贴合一眼可见。
  - 「consumed once」样例要把「最近那层 padding 容器」画成虚线框，并标出外层和最近层的 padding 值。

**U5-2 · 拾取未声明的部件：没有任何反馈，点击还穿透执行了真实动作**
- 截图：U5-inspector-undeclared-hover、U5-inspector-undeclared
- 维度：交互。可挽回 **0.5**
- 现状：
  - 据 U5-actions.log 第 28–32 行：开启 Inspect，等 800ms 后悬停预览主按钮「Create · 0」。悬停帧里没有轮廓、没有标签，面板也没变化。
  - 点击后按钮变成「Create · 1」，说明真实动作被执行了；面板仍只有说明文字，没有「未标注」提示。
  - 原图 U5-inspector-1440-light 也是「Create · 1」，与此一致。
- 期望：P2「找不到映射时明确显示未标注，不编造 token」。
- 修法：
  - 拾取模式下，在预览 scope 根上用捕获阶段监听 pointerdown/click，调用 `preventDefault()` 和 `stopPropagation()`，让任何点击都只做拾取。
  - 悬停未声明的部件时，画中性灰的虚线 outline，加标签「Not annotated / 未标注」。
  - 点击后，面板显示「This part has no declared token mapping」，不出 token 按钮。
  - 中英文同步（G6）。

**U7-3 · Heading 不是粗体**
- 截图：U7 排版区，四张图一致
- 维度：视觉。可挽回 **0.5**
- 现状：
  - 默认尺寸的「A heading keeps its own level」字号 24px，竖笔约 2.1px，约 0.09em；下方 16px 正文的竖笔 1.45px，也约 0.09em。比例相同，说明是常规字重。
  - 大标题「Native structure, flexible composition」竖笔约 7px，字号约 62px，比例约 0.11em，介于常规和粗体之间。
  - 「Layout primitives」同样偏细。
- 期望：radix-sink-heading 两组 9 档全是粗体；L1 要求 Heading「具有正确语义外观」。
- 修法：
  - Heading 默认 `font-weight: 700`（对应 Radix 的 `--font-weight-bold`），只有显式传 weight 时才覆盖。
  - 同时核对当前主题字体在 public 资源里有没有 700 字面的 `@font-face`。如果只加载了 400/500，700 会回落到最近的字重，也会同时造成 U7-4。

**U7-4 · Strong 没有加粗**
- 截图：U7「Emphasis · “Native quotation marks” · Strong importance」这一行
- 维度：视觉。可挽回 **0.3**
- 现状：放大看，「Strong importance」的竖笔是 1.05–1.3px，前面常规文字是 1.3–1.45px，没有任何强调效果。
- 期望：radix-sink-code 段落里的「greyscale antialiasing method」是粗体；L1 要求 Strong 有正确的语义外观。
- 修法：Strong 设 `font-weight: 700`，并确认 700 字面可用（同 U7-3）。

### P2

**U7-5 · Code、Kbd 各只有一个样例，外观都不是 L1 默认变体**
- 维度：视觉 0.45 + 细节 0.05。可挽回 **0.5**
- 现状：
  - Code「let layout = Responsive::from(value);」：1px 描边 #95A9EC，白底，蓝字约 #4165DD，高 19px，是 outline 的外观。
  - Kbd「Ctrl + K」：平铺底 #F0F0F3，无描边、无阴影，高 20px，是 soft 的外观。
  - 夹具没有标注 variant。
- 期望：L1 规定 Code 默认 soft（对应 radix-sink-code 的 soft 行：淡色底、无描边），Kbd 默认 classic（对应 radix-sink-kbd 的 classic 列：有描边和底部内阴影的凸起键帽）。
- 修法：
  - 如果这两个样例没传 variant，说明组件默认值错了：Code soft 改为 accent-a3 底加 accent-a11 字、无描边；Kbd classic 按 Radix 3.3.0 classic 的多层 inset 加外阴影组合，做出凸起键帽。
  - 如果是显式设置的，就补上不带 variant 的默认样例，并给每个样例写出 variant。

**U7-6 · 非法属性之后，页面上看不到属性定位诊断**
- 截图：U7-invalid-1280-light-a2、U7-invalid-1280-light
- 维度：交互。可挽回 **0.5**
- 现状：点「Invalid prop」后，唯一的变化是「A bad prop keeps this view alive.」从每词一行变成一行。组件保持可用，这部分是对的。但整段夹具里没有诊断文字：没有组件名、属性名、非法值，也没有回退值。
- 期望：L6「非法值给出属性定位诊断，组件默认保持可用」。
- 修法：在该样例下方放一行 `role="status"` 的诊断，渲染诊断接口返回的「Box · width: invalid value "…" → default」。如果诊断只走 console，也要在夹具里把同一条渲染出来，这样截图才能验收。

**U7-7 · 从画面看不出 Reset 的作用范围**
- 截图：「Native reset action 0」
- 维度：布局 0.2 + 交互 0.1 + 细节 0.1。可挽回 **0.4**
- 现状：只有一个被 Reset 的按钮，显示为蓝字，x=24，而其他样例的左缘是 x=16。旁边是计数 0。没有未经 Reset 的同类原生元素作对照，也看不出有没有包裹层。
- 期望：radix-reset 里，被 Reset 的原生元素（Button、heading、列表等）都是纯文本：没有 UA 自带的边距、边框和底色，颜色继承。L4：只作用于单个元素，不加包裹层，不影响宿主和兄弟元素。
- 修法：
  - 并排放三组：①原生 `<button>`，保留 UA 外观；②同一个按钮加 Reset，应无边框、无底色、无内边距、`color: inherit`；③被 Reset 的 `<h2>` 和 `<ul>`，与未 Reset 的同类兄弟元素对照。
  - 给 Reset 元素加 1px 虚线 outline，证明没有包裹层。
  - 蓝字和 8px 缩进如果来自调用方 class，要在标签里写明；否则去掉。

**U7-8 · Container 只能看到一档，而且看起来没居中**
- 维度：布局 0.3 + 细节 0.1。可挽回 **0.4**
- 现状：
  - 灰带「The inner content has a responsive maximum width.」宽 1136px（x=80–1215，三档缩放都是 1136，这一点正确）。但它在 1232px 宽的夹具里左边空 80px、右边空 17px，又没有标注。
  - 「Dimensions describe this content layer.」只有文字，没有边界，在约 230px 处换行，留下孤词「layer.」。
- 期望：radix-sink-container 里，外层 size 4 的盒子满宽，内层 size 1（448px）在父级中居中，两层都有可见边框。L3：size 1–4 对应 448/688/880/1136px，并支持 left/center/right。
- 修法：每档 Container 的内层加 1px 描边和「size=n · align=…」标签，放在同一个满宽父级里。如果这条灰带本来是 align=center，就修正为左右各 48px 居中。

**U5-3 · 预览对话框没有遮罩，390 宽时贴边**
- 截图：U5-dialog-1440、U5-dialog-390
- 维度：布局 0.2 + 视觉 0.2。可挽回 **0.4**
- 现状：
  - 1440 宽：对话框在 x=527–914、y=371–529，位于视口中心。背后没有遮罩，也没有阴影，白色面板直接叠在两张白卡片上，只靠 1px 灰边区分；后面的「I understand this is」被截断。
  - 390 宽：面板在 x=1–389，左右各只留 1px，圆角贴着屏幕边。
- 期望：X6（DESIGN Dialogs）要求 30% foreground 遮罩，宽度为视口减 2rem；G6 要求对话框在可用视口内可操作。
- 修法：
  - 在预览对话框下面加遮罩，颜色用作者 foreground 的 30%，范围限定在预览 scope 内，以免作者色进入编辑壳（P4）。
  - 面板宽度设为 `min(calc(100vw - 2rem), 当前宽)`，390 宽时左右各留 16px。

**U5-4 · 渲染器、Inspect 和对比区模式按钮的选中态几乎看不出来**
- 截图：U5 各帧的工具栏，U5-compare-nested
- 维度：视觉 0.25 + 交互 0.1。可挽回 **0.35**
- 现状：采样结果如下。DOM 选中时，DOM 按钮底色 #EFF8FF，未选中的按钮底色 #F9FAFB。两者的描边都约为 #82B4F7，文字都是 #1570EF。DOM + GPU 选中、Inspect 开启时同样只多这一层极淡的底色。对比区 Dark 的文字是 #175CD3，Light 是 #1570EF，差别肉眼难辨。
- 期望：G4「选中的模式/渲染器按钮使用按下态」；G3 规定选中背景 #eff8ff、选中边框 #84caff、选中文字 #175cd3。
- 修法：
  - 未选中的按钮改为中性：白底，边框 #d0d5dd，文字 #475467。
  - 选中的按钮用 G3 的三个颜色，并加 `aria-pressed="true"`。
  - 让选中和未选中在边框、文字、底色三项上都不同。

**U5-5 · 预览菜单相对触发按钮水平偏移约 113px，并盖住边界标题**
- 截图：U5-menu-1440
- 维度：布局 0.2 + 交互 0.1。可挽回 **0.3**
- 现状：触发按钮 Actions 在 x≈499，打开后带一层淡底。弹出的菜单（Duplicate / Archive）左缘却在 x≈386，与 Open dialog 对齐。菜单向上弹出，盖住了「A local theme boundary」标题，顶边还与上方 Small 卡片的底边重叠 1px。
- 期望：P1 要求真实组件表现真实的交互状态；G6 要求菜单在可用视口内可操作。目标包里没有「菜单锚定触发器」这样更细的条文，这里按 P1 计分。
- 修法：以触发按钮的 `getBoundingClientRect()` 为锚点：start 对齐到 x=499，主轴偏移 4px，空间不够时翻转。偏移量正好等于 Open dialog 的宽度加间距，应检查定位参照物是不是用成了边界容器而不是触发按钮（scope 上的 transform 或 contain 会改变 containing block）。

**U5-6 · 拾取之后，来源说明不在视野里**
- 截图：U5-inspector-picked
- 维度：交互 0.25 + 细节 0.05。可挽回 **0.3**
- 现状：点击已声明样本后，编辑区滚到 token 网格，primary 被选中（浅蓝底），紧贴视口顶边，上方没有留白。「Fine adjustment source / Explicit document override / Clear this token · current mode」这段来源说明在它上方、视野之外。拾取面板只列出 primary、primary-foreground、ring 三个按钮，不显示来源。
- 期望：P2 要求拾取能定位到「覆盖来源」；U5 口径是验收「既有 token 入口与 P1 来源说明」。
- 修法：拾取确认后，对来源说明块调用 `scrollIntoView({ block: 'start' })`，并设 `scroll-margin-top: 16px`，让来源说明和选中的 token 同屏出现。或者在拾取面板的 token 按钮旁加一行来源摘要，例如「primary · Explicit document override」。

**U5-7 · 固定定位的「Host control」压住内容**
- 截图：U5 所有 1440 帧的左下角；U5-preview-390、U5-dialog-390 的右下角
- 维度：布局 0.2 + 细节 0.05。可挽回 **0.25**
- 现状：
  - 1440 宽：按钮在 x≈22–97、y≈868–890，盖住了编辑区 Appearance 说明的头几个词（只剩「the containing theme…」）；在 picked 帧里盖住「Feedback」标题。
  - 390 宽：按钮在 x≈297–372、y≈817–838，压在预览卡片右下角和 GPU 柱状图上。
- 期望：G6 要求关键内容可达、不被遮挡；P3 要求宿主元素存在并与 scope 隔离，但不应盖住内容。
- 修法：把它放回正常文档流，例如预览工具栏末尾或编辑区底部；或者给编辑区和预览区两个滚动容器加不小于「按钮高度 + 16px」的 `padding-bottom`。

**U7-9 · Quote 没有斜体**
- 截图：U7 的 Em / Quote / Strong 行
- 维度：视觉 0.15 + 细节 0.05。可挽回 **0.2**
- 现状：「“Native quotation marks”」是直立的，字号与周围相同。
- 期望：radix-sink-code 段落里的引语是斜体，并略大；L1 要求 Quote 有正确的语义外观。字族不比较。
- 修法：Quote 加 `font-style: italic`，按 Radix 再乘 1.1em 的字号系数。

**U5-8 · 已声明样本的拾取轮廓比部件宽很多**
- 截图：U5-inspector-1440-light-a2、U5-inspector-picked
- 维度：交互。可挽回 **0.15**
- 现状：悬停「Create profile」时（按钮本身在 x=369–485），蓝色 outline 画满了 x=369–843 一整行，右侧约 360px 是空白。
- 期望：P2 要求「从实际部件定位」，选中矩形是固定的蓝色 outline，应只包住这个部件。
- 修法：把样本声明挂在按钮元素本身上，或者 outline 直接取按钮自己的 rect，而不是外层满宽的包裹 div。

**U5-9 · 默认预览场景缺少 P1 新组件和排版组件**
- 截图：U5-dom、U5-compare-nested
- 维度：交互。可挽回 **0.15**
- 现状：Cards 场景里能看到 Button 的多个变体，但没有 IconButton，也没有 Link。「let theme = resolve(document);」是无底色的等宽字，看不出 Code 的外观；「Serif · Shape a thoughtful interface 中文」是普通文字。Button 只出现了一种尺寸。
- 期望：P1 要求「预览使用真实组件、变体、尺寸」；U5 口径把 Button / IconButton / Link / 排版列为 P1 新组件。
- 修法：
  - 在卡片标题右侧加一个带 aria-label 的 IconButton；
  - 在表单说明里加一个 Link；
  - 代码行改用 Code（默认 soft），快捷键用 Kbd，标题用 Heading；
  - 加一行 Button size 1–3。
  - 如果其他场景里已经有这些组件，至少让默认场景也出现，或者下一轮补拍。

### P3

**U7-10 · 现有样例都没有标注 props**
- 维度：细节。可挽回 **0.2**
- 现状：Code、Kbd、Inset 的 side、Container、Text 等样例都没写 variant、size、side、align，逐档对照时只能靠猜。
- 期望：要能和目标图逐档对应（radix sink 页每行都有「size n」、「soft」之类的标签）。
- 修法：每个样例前加 12px 灰字标签，例如「Code · variant=outline」、「Inset · side=top（贴上/左/右）」。

**U5-10 · 拾取标签压住上一行文字**
- 截图：U5-inspector-card
- 维度：细节。可挽回 **0.1**
- 现状：标签「card, card-foreground, border, radius」在 x≈344–540、y≈325–343，盖住了上一行「changes stay in this preview.」里 g、y 的下伸部。
- 修法：当矩形上方的空隙小于标签高度时，把标签放进矩形内的左上角（内缩 2px），或者翻到矩形下方。

**U5-11 · GPU 色样标签和 token 名不一致**
- 截图：U5-gpu-compare
- 维度：细节。可挽回 **0.1**
- 现状：GPU 的色样写的是 fg、card-fg、popover-fg、primary-fg、secondary-fg、accent-fg、destructive-fg，而编辑区和拾取面板用的是 foreground、card-foreground、primary-foreground。
- 期望：P2 要求能定位到语义角色，两端的名称应能直接对上。
- 修法：GPU 标签改用完整的 token 名（可以换行或缩小字号），或者两种写法同时显示。

**U5-12 · DOM 与 GPU 两列的内容起点错开 22px**
- 截图：U5-gpu-compare
- 维度：细节。可挽回 **0.05**
- 现状：DOM 列第一张卡片的顶边在 y≈197，GPU 列的框在 y≈219，差别来自 GPU 多出一行「Ready」。
- 修法：把「Ready」移到「GPU theme samples」同一行并右对齐，或者给 DOM 列加一行等高的状态行。

**分数核对**
- U5：各条可挽回分合计 3.55，10 − 6.35 = 3.65，误差 0.1。按维度：布局 1.0、视觉 0.85、交互 1.25、细节 0.45。
- U7：各条可挽回分合计 4.8，10 − 5.2 = 4.8。按维度：布局 1.4、视觉 2.0、交互 0.9、细节 0.5。

---

## 4. 建议（不扣分）

**U5**
- 渲染器和 Inspect 按钮高约 24px，旁边的两个选择器高 36px，同一行高度不齐。建议统一为 32 或 36px。
- 菜单和对话框没有阴影，层级只靠 1px 边框区分。如果作者主题提供 shadow token，可以用上。
- 「Clear local draft」是禁用态但没说原因，外观和演示用的「Disabled」一样。截图判断不了它是真实禁用还是演示禁用，所以不扣分；建议加一行辅助文字，呼应 P1 的「演示与真实禁用区分」。
- 390 宽时「Large · shadow-lg」在连字符处断成「shadow-」和「lg」。这是示例数据，不扣分。
- 范围外观察（属 U1，不计分）：语言切换处「EN」和「中文」挤在一起；编辑区 Light/Dark 预览模式按钮也有 U5-4 同样的选中态问题。

**U7**
- 默认态下「A bad prop keeps this view alive.」每词一行，读不顺。建议标出它用的合法属性值，或者给一个可读的宽度。
- 夹具的 Scale 100/110/95 控制按钮看不出当前是哪一档。
- 「Your name」标签和输入框之间是 0 间距。页面排列不在比较范围内，所以不扣分。
- 每段建议加上与目标图同名的小标题（Heading、Text、Code、Kbd、Blockquote、Container、Inset、Reset），方便逐图对照。

---

## 5. 截图看不出的地方

**U5**
- U5-menu-1440、U5-dialog-1440/390：菜单和对话框只在浅色根下打开过，看不出它们在深色或嵌套边界里会不会跟着作者主题变（P3）；也看不出菜单的键盘焦点、高亮项，以及对话框关闭后焦点是否回到触发按钮。
- P5 的宽度选择器：所有帧都停在「Responsive」，没有展开，也没有切到 1024、768、390。
- P2 的「浮层同样可检查」：没有在菜单或对话框打开时做过拾取。
- P4 的 GPU 失败态和中文状态文字：只拍到英文的「Ready」。
- 拾取面板的「All drawn tokens · keyboard selection」没有展开，Esc 退出也没拍到。
- U5-inspector-1440-light（原图）：按 capture.md 不作为证据。

**U7**
- 目标图里的 Heading、Text 9 档，Code 变体矩阵，Kbd classic，wrap 的 nowrap/balance/pretty，trim，四档字重：夹具里都没有，这一项已作为 U7-1 计分。
- Code、Kbd 的样例是显式设置了 variant，还是走的默认值。
- Inset 的 side 样例看不到容器边界；「consumed once」样例里，最近那层 padding 容器看不到。
- Reset 按钮的蓝字和 8px 缩进是不是来自调用方 class。
- Container 的 1–3 档和对齐方式；那条 1136px 灰带左 80px、右 17px，是不是有意的 align=right。
- 非法属性的诊断有没有输出到控制台：截图不包含控制台，capture.md 只记录了 console error 为 0。
- 大标题的精确字号：「N」的字帽高约 44px，换算约 62px，和 size 9 的 60px 之差在字体度量误差内，不下结论。390 宽时四行基线间距约 60px，与 size 9 的行高一致。
- U7-invalid-1280-light（视口截图）只拍到中段，以 a2 为准。

---

## 6. 总评

- **最低分单元：U7（5.2）。** 主要原因是夹具没有呈现目标图的主体内容：Heading、Text、Code、Kbd 的梯度和变体矩阵、wrap 卡片、可见容器下的 Inset 贴边、Reset 对照，都无法与目标逐档比对。能看到的部分里，又有两处组件级的错误：Heading 和 Strong 都不是粗体。缩放、Container 宽度、Section 间距、Blockquote、响应式这些做对了。
- **需要结构性改动的地方：**
  1. U7 夹具要按 radix sink 页的结构重建区块（U7-1、U7-2、U7-7、U7-8），这不是微调能解决的；同时要修正 Heading、Strong 的默认字重，并核对 700 字面是否可用（U7-3、U7-4）。
  2. U5 预览的 ThemeScope 要改成一块连续、带内边距的作者主题面（U5-1）。
  3. 拾取模式要拦截点击，并补上「未标注」态（U5-2）。
- U5 其余的问题（选中态、菜单锚点、对话框遮罩与 390 留白、拾取后的来源可见性、Host control 遮挡）都是局部修复。

---

开图清单：U5-dom-1440-light.png、U5-dom-1440-dark.png、U5-menu-1440-light.png、U5-dialog-1440-light.png、U5-compare-nested-1440-light.png、U5-inspector-1440-light.png、U5-inspector-1440-light-a2.png、U5-inspector-card-1440-light.png、U5-inspector-picked-1440-light.png、U5-inspector-undeclared-hover-1440-light.png、U5-inspector-undeclared-1440-light.png、U5-gpu-compare-1440-light.png、U5-preview-390-light.png、U5-dialog-390-light.png、U7-fixture-1280-light.png、U7-scale110-1280-light.png、U7-scale95-1280-light.png、U7-invalid-1280-light.png、U7-invalid-1280-light-a2.png、U7-fixture-390-light.png；目标图 radix-sink-heading.png、radix-sink-text.png、radix-sink-code.png、radix-sink-kbd.png、radix-sink-blockquote.png、radix-text-wrap.png、radix-sink-container.png、radix-inset.png、radix-reset.png（另读 U5-actions.log、U7-actions.log、capture.md、目标包；辅助裁剪图均在 /private/tmp/claude-501/-Users-rockie-Documents-gh-rockie-rustify-ui/0bc04ccb-5f93-4892-a089-e1cb25f9419c/scratchpad/judge-C2/）
