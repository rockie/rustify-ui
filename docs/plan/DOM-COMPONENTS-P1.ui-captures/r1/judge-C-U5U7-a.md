# DOM-COMPONENTS-P1 · UI 验收 R1 · U5、U7 独立评审

范围：只评 U5、U7。首轮，没有上一轮结论。下面的结论只依据本轮 17 张截图、9 张目标图、`U5-actions.log`、`U7-actions.log` 和目标包的「单元」「规范要点」两节。为了看清细节，我在 scratchpad 的 `judge-C/` 下生成过裁剪图，也读过像素值，其他位置没有写入任何文件。

## 1. 评分表

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 /10 |
| --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.0 | 2.5 | 1.5 | 0.6 | **6.6** |
| U7 布局与排版 | 2.0 | 1.5 | 2.5 | 0.75 | **6.75** |

**U5**
- **布局 2.0**：以下几项都正确：左编辑右预览、各区独立滚动、1440 下 DOM 与 GPU 并排、390 下上下堆叠、Edit/Preview 标签和「Back to edit」。不到 3 分有两个原因：
  - 「Host control」固定在视口角落，每一帧都压着编辑区或预览内容。
  - 深色 scope 没有内边距，文字和卡片边框直接贴在 scope 的边缘。
- **视觉 2.5**：预览切到深色时编辑壳仍是中性浅色；拾取的蓝色轮廓和浅蓝面板符合 P2、P4。不到 3 分有两个原因：
  - 深色下「Caller class override」按钮的文字对比度约 1.8:1。
  - 深色预览被一条浅色的壳层条带切成两块。
- **交互 1.5**：渲染器切换、菜单、对话框、对比区切深色、拾取后编辑器定位到 token，都能走通。没给到 2 分，是因为拾取这条主路径只部分正确：
  - 选中矩形套在了错误的部件上。
  - 点击未声明部件时，真实动作被执行，也没有「未标注」提示。
  - 拾取后看不到来源说明。
  - 「相反外观」只以一个「independent scope」的形式出现，不是嵌套在根里的边界。
- **细节 0.6**：拾取标签压住上一行文字；被选中的 token 卡片贴在视口顶边，顶部被裁掉。

**U7**
- **布局 2.0**：缩放后，Container 的显式宽度（1136px）不变，主题间距随缩放变化（16→18→15px）；390 下没有溢出。不到 3 分的原因：
  - Inset 演示里没有任何一边是贴边的，侧向样例也看不到容器面。
  - Container、换行、Reset 的样例不够，无法和目标图逐项对照。
- **视觉 1.5**：已出现的字号都落在目标刻度上（60、24、16、14px），Blockquote 和 outline 款 Code 的外观接近目标。不到 2 分的原因：
  - 缺 Heading 和 Text 的 1–9 字级梯度，也缺 Code、Kbd 的变体矩阵。
  - Strong 没有加粗，Quote 没有斜体，Kbd 是 soft 款的平铺外观。
  - 24px 的 Heading 用的是正文字重。
- **交互 2.5**：110% 和 95% 缩放都只生效一次（H1 墨迹高 58→64 / 55px；caller 固定字号行高 10px、1136px 容器不变）；非法属性之后组件仍可用。不到 3 分：整张截图里看不到「属性定位诊断」。
- **细节 0.75**：Blockquote 左边框不随字号加粗；夹具顶边混入了外来的 1px 线段。

## 2. 上一轮差距复核表

首轮，省略。

## 3. 差距清单

每个单元各条差距的可挽回分之和：U5 为 3.4，等于 10 − 6.6；U7 为 3.25，等于 10 − 6.75。

### P1 严重

**新-1 · U5 · 布局 · P1 · 可挽回 0.6**
- **位置**：
  - 1440 的全部 U5 截图，左下 x≈22–97、y≈868–890：
    - dom、inspector 系列：盖住「Appearance」说明文字的开头。
    - menu、dialog、compare：盖在 primary 拾色面板上。
    - picked：盖在 success token 上。
  - 390 的截图，右下 x≈297–371、y≈816–838：
    - preview-390：压在预览内容上。
    - dialog-390：压在 GPU 柱状图上。
  - 编辑区滚动后，它的视口位置不变，说明是固定定位。
- **现状 → 期望**：现状是宿主元素一直遮住内容。期望依据 G1「编辑壳不随预览主题失去可读性」和 P3「修改任一区域不改变其边界外的…宿主元素」：宿主元素放在 scope 外，但不能遮住编辑区或预览。
- **修法**：
  1. 取消 Host control 的视口固定定位，改为普通文档流。
  2. 放进「Provider & overlay checks」区块、ThemeScope 之外的一行，加标签「Host element · outside scope」。
  3. 390 下也随文档流排列。

**新-2 · U7 · 布局 · P1 · 可挽回 0.5（这个演示需要推倒重做）**
- **位置**：
  - U7-fixture-1280 的「Nearest padding is consumed once.」：内块 #fbfbfc（x48–283、y885–945）坐在外框 #f0f0f3（x16–315、y853–977）里，四边各留 32px（110% 时 35px，95% 时约 30px），没有任何一边贴边。
  - all、x、y、top、bottom、left、right、responsive 这组样例是白底白字，看不到容器面。其中 all、x、top、left 从 x=19 起，比 16px 的内容边多出 3px；y、right 缩进到约 x=35 和 30。
- **现状 → 期望**：现状是看不出哪一边被 Inset 吃掉了。期望依据 radix-inset：上方卡片的图片贴住卡片的左、右、下边，Checkbox Card 的图片贴住左、上、下边，另外几边保留内边距；以及 L3「只消除最近容器声明的 padding，使内容贴边」。
- **修法**：
  1. 每个 side 样例都做成可见容器：1px 灰色描边、圆角、面板底色、16px 左右的 padding。
  2. Inset 子元素用对比填充，比如 accent-a3 底色，或从 public 目录按 URL 引用一张图片。这样 side=all 时四边都贴住边框，x 时贴左右，top 时贴上和两侧，依此类推。
  3. 「consumed once」样例让外层和内层容器都显示描边，能看出吃掉的是哪一层 padding。
  4. 同时排查 x=19 那 3px 残留是从哪里来的。

**新-3 · U7 · 视觉 · P1 · 可挽回 0.4**
- **位置**：整张 U7-fixture-1280 只出现这几种字号：
  - Heading 约 60px（H1 大写字母高 43px）和约 24px（高 17px）。
  - Text 约 24px（居中那行）、16px、14px。
  - 一个由 caller 固定的约 12px。
  - 没有 Heading 或 Text 的 1–9 梯度，也没有 Code 的字号梯度。
- **现状 → 期望**：期望依据 radix-sink-heading 的 9→1 粗体梯度加多行换行样本（用来看行高）、radix-sink-text 的 9→1 常规字重梯度、radix-sink-code 下半部分的 Code 9→1 梯度。比较口径点名要比「字级梯度、字重、行高」。
- **修法**：
  1. 在 `mount_layout_fixtures` 里加一个「Type scale」块。
  2. 对 size 9 到 1，各渲染一行 Heading 单行样本，外加一段窄宽度（约 26ch）的多行样本，用来暴露行高。
  3. Text 9→1、Code 9→1 同样处理。

**新-4 · U5 · 交互 · P1 · 可挽回 0.4**
- **位置**：U5-inspector-1440-light-a2。悬停「Create profile」时，按钮实际范围是 x369–485、y600–632，但 2px 蓝色轮廓画在 x369–843，也就是卡片里一整行的宽度。
- **现状 → 期望**：期望依据 P2「从实际部件定位…选中矩形为固定蓝色 outline」：矩形应等于部件自身的边框盒。同一套机制在 U5-inspector-card 的卡片上是贴合的。
- **修法**：
  1. 拾取层量的应该是已声明样本自身元素的矩形（按钮根元素），不是它的块级包裹层。
  2. 如果声明属性挂在全宽包裹层上，就移到 Button 根元素，或把包裹层改成宽度随内容。
  3. 轮廓和标签都锚定到这个矩形。

**新-5 · U5 · 交互 · P1 · 可挽回 0.4**
- **位置**：U5-inspector-1440-light.png 加日志。日志顺序是「click Inspect」，然后「click preview primary button」。截图里 Inspect 处于按下态，面板只有通用提示，没有轮廓，也没有「未标注」，但计数从「Create · 0」变成了「Create · 1」；U5-gpu-compare 里仍是 1。日志显示两次点击只隔 33ms，建议复采时在两次点击之间等待，确认时序。
- **现状 → 期望**：期望依据 P2「找不到映射时明确显示未标注，不编造 token」。拾取模式下不应执行真实动作。
- **修法**：
  1. 拾取模式下给预览加一层捕获层，或在捕获阶段拦截点击。
  2. 命中检测找最近的已声明样本。
  3. 找不到时，用中性虚线框标出悬停元素，面板显示「Not annotated — no token mapping for this part」，中文「未标注：此部件没有 token 映射」，不显示 token 按钮。

### P2 一般

**新-6 · U7 · 交互 · P2 · 可挽回 0.5**
- **位置**：U7-invalid-1280。点 invalid 之后，唯一可见的变化是「A bad prop keeps this view alive.」变成一行，即回到了默认值。整个 1280×900 视口里没有任何诊断文字；日志也没有记录 console。
- **现状 → 期望**：期望依据 L6「非法值给出属性定位诊断」，以及目标状态「非法属性诊断」。
- **修法**：
  1. 在样例旁渲染一行带 `role="status"` 的诊断文字，写明组件、属性和值，例如：Box · width: "…" is not a valid length; default kept。
  2. 用错误色、size 2。
  3. console 警告保留。

**新-7 · U5 · 交互 · P2 · 可挽回 0.4**
- **位置**：U5-inspector-picked。拾取后编辑器跳到 Colors token 网格，primary 被选中并贴在视口顶部。U5-menu 里能看到的来源块「Fine adjustment source · primary · Explicit document override」此时在视口上方，看不到。拾取面板只列 primary、primary-foreground、ring 三个名字，没有来源。
- **现状 → 期望**：期望依据比较口径「拾取只验收既有 token 入口与 P1 来源说明」，以及 P2 中的「覆盖来源」。
- **修法**：
  1. 拾取面板每个 token 按钮下加一行来源，例如「primary · Explicit document override」，并附「Clear this token」的入口。
  2. 拾取后把编辑区滚到该 token 的来源块，留 16px 的顶部间距。

**新-8 · U5 · 布局 · P2 · 可挽回 0.4**
- **位置**：
  - U5-dom-1440-dark：深色 scope（#0c111d）占 x344–1415、y160–525。「Local demonstration…」从 x=344，也就是色块左缘起笔，离色块上沿只有约 10px。卡片左边框和色块边缘重合；色块下沿和卡片下边框重合，卡片下方的两个圆角被切掉。
  - U5-compare-nested（y437–801）同样如此。
- **现状 → 期望**：期望依据 G2「预览 padding 24px 24px 48px」。浅色时 scope 底色和壳层一样，看起来有 24px 留白；切到深色，实际留白是 0。
- **修法**：给预览 ThemeScope 根自身加 24px（底部 48px）的 padding、壳层圆角和 1px #d0d5dd 描边，做成有边框的面板。和新-12 合并处理。

**新-9 · U5 · 交互 · P2 · 可挽回 0.3**
- **位置**：U5-compare-nested。相反外观的区域标题是「Compare an independent scope」，位于根预览框（下边框 y=313）和分隔线（y=346）之后，自带 Light/Dark。根内部的「A local theme boundary」在任何一帧里都没有外观选项，深色时也跟着根一起变深（U5-dom-1440-dark）。
- **现状 → 期望**：期望依据 P3「嵌套边界可选择与根相反的外观」，以及目标状态「对比区嵌套相反模式」。
- **修法**：二选一：
  - 在「A local theme boundary」上加 Inherit/Light/Dark 选择（按下态），演示根为 Light 时边界为 Dark。
  - 把对比区放进根框内，改名为「Nested boundary · opposite of root」，并加说明「Root: Light · Boundary: Dark」。

**新-10 · U7 · 视觉 · P2 · 可挽回 0.3**
- **位置**：U7-fixture-1280 的「Ctrl + K」：平铺的 #f0f0f3 填充，高 20px，没有外轮廓、没有底部投影，像素从填充色直接过渡到白色。
- **现状 → 期望**：期望依据 radix-sink-kbd：classic 列是凸起的键帽，有细外轮廓、底部投影和顶部高光；soft 列才是平铺填充。L1 规定 Kbd 默认 classic，而现状和 soft 一致。
- **修法**：
  1. 默认 variant=classic 按 `ref/themes-main` 的 classic 配方实现：浅底、细外环、底边内阴影、底部投影、顶部高光。
  2. 夹具里并排放 classic 和 soft，并加 size 1–9 的行。

**新-11 · U5 · 视觉 · P2 · 可挽回 0.25**
- **位置**：U5-dom-1440-dark 的「Caller class override」：#f9fafb 文字在 #fdb022 上，对比约 1.8:1。浅色时是 #1d2939 在 #f79009 上，约 6.3:1。
- **现状 → 期望**：期望依据 P1「预览使用真实组件」，按钮文字可读，达到 WCAG AA 4.5:1。
- **修法**：演示用的 caller class 要成对覆盖背景和前景，例如 warning 和 warning-foreground 这对 token；或者改用 color 属性来演示。

**新-12 · U5 · 视觉 · P2 · 可挽回 0.25**
- **位置**：U5-dom-1440-dark。深色预览被拆成两个色块：y160–525 一块，「Provider & overlay checks」摘要行（约 y540–570）用的是浅色壳层 #f9fafb，y577 起又是一块深色。
- **现状 → 期望**：期望依据 G1「用户预览使用作者自己的主题」和 P4，预览内容应是一块连续的作者表面。
- **修法**：把整列 DOM 预览（说明行、卡片、disclosure、provider 区）包进同一个 scope 根，disclosure 用作者 token 着色；provider 自己的 scope 嵌在里面。

**新-13 · U7 · 视觉 · P2 · 可挽回 0.25**
- **位置**：U7-fixture-1280 的「Strong importance」：笔画约 1.3–1.4px，旁边的常规文字约 1.5px，平均灰度相同（0.864 对 0.864），没有加粗。
- **现状 → 期望**：期望依据 radix-sink-code 中明显加粗的「greyscale antialiasing method」，以及 L1「具有正确语义外观」。
- **修法**：Strong 用 bold 字重 token（700），并确认当前主题字体的粗体字面已从 public 目录加载；补一个对应的检查。

**新-14 · U7 · 视觉 · P2 · 可挽回 0.2**
- **位置**：「A heading keeps its own level」（24px，大写高 17px）的笔画约 2.5px，和同为 24px 的 Text「Text follows…」（约 2.4–2.5px）一样。
- **现状 → 期望**：期望依据 radix-sink-heading：Heading 各级都是粗体，24px 那一级也是。
- **修法**：Heading 的默认字重在所有 size 上都用 bold。如果夹具对这个 Heading 显式设了 weight，就另外加一个默认字重、size 6 的样本。

**新-15 · U7 · 视觉 · P2 · 可挽回 0.2**
- **位置**：只有一个 Code，是 outline 外观：1px #95a9ec 边、无填充、accent 色文字。没有 soft（默认）、solid、ghost，也没有 gray 和 high-contrast。
- **现状 → 期望**：期望依据 radix-sink-code 的 4 种变体 × 4 列颜色矩阵，以及 L1「Code…默认 soft」。
- **修法**：
  1. 加 4 种 variant × {accent, accent+hc, gray, gray+hc} 的表。
  2. 另放一个不设 variant 的 Code，证明默认是 soft：浅色 accent 底、无边。

**新-16 · U7 · 布局 · P2 · 可挽回 0.2**
- **位置**：只有一个 #f0f0f3 容器（x80–1215，1136px，缩放下不变，这点正确），贴住内容区右缘。「Dimensions describe this content layer.」看不到边界。448、688、880px 三档都看不到。
- **现状 → 期望**：期望依据 radix-sink-container 中带描边的嵌套 size 4 与 size 1（448px 居中），以及 L3 的四档宽度和 left/center/right 对齐。
- **修法**：给 size 1–4 各画一个带描边的容器，标出预期宽度；再在 size 1 上加 left/center/right 三个对齐样例。

**新-17 · U7 · 视觉 · P2 · 可挽回 0.15**
- **位置**：「“Native quotation marks”」是正体。
- **现状 → 期望**：期望依据 radix-sink-code：Quote 是斜体，见「“I believe this…”」。
- **修法**：Quote 用 `font-style: italic`（通过 quote token），字族仍跟随主题。

**新-18 · U7 · 布局 · P2 · 可挽回 0.15**
- **位置**：只能看到 truncate（「This long sentence is ...」），没有 wrap、nowrap、balance、pretty 的样例。
- **现状 → 期望**：期望依据 radix-text-wrap 的 6 张卡，以及 L2。
- **修法**：加 6 个约 360px 宽的面板，用同一句话分别演示 default、wrap、nowrap、balance、pretty、truncate；nowrap 那张溢出隐藏。

**新-19 · U7 · 布局 · P2 · 可挽回 0.15**
- **位置**：「Native reset action」是蓝色 #3e63dd，起笔在 x=24，比内容边多出 8px；整个夹具只有一个 Reset 样例（按钮）。
- **现状 → 期望**：期望依据 radix-reset：被 Reset 的元素都是继承色的普通文字，和其他内容同一左缘；以及 L4「单个原生元素、不加包裹」。
- **修法**：
  1. 加一个不带 caller class 的 Reset 按钮样例。
  2. 如果那 8px 是原生按钮残留的 padding 或边框，Reset 要把按钮的 padding、border、background、color 全部清掉。
  3. 再各自单独 Reset 一个 h2、一个 ul、一个 a 作为样例。

### P3 打磨

**新-20 · U5 · 细节 · P3 · 可挽回 0.25**
- **位置**：U5-inspector-card 的标签「card, card-foreground, border, radius」压住「Local demonstration…」的下伸部；a2 的标签压住「I understand this is a local demo」。
- **现状 → 期望**：期望依据 P2，选中矩形和面板都要可读，不遮挡内容。
- **修法**：上方空间小于标签高度时，把标签放进轮廓内的左上角，或翻到轮廓下方；token 名很长时只在面板里列出。

**新-21 · U5 · 细节 · P3 · 可挽回 0.15**
- **位置**：U5-inspector-picked。被选中卡片的顶边（#84caff）在 y=0，贴着视口顶；所属分组的标题在屏幕外。
- **现状 → 期望**：期望依据 P2：拾取后要能看出定位到的是哪个分组、哪个 token，而不是卡片被裁掉、分组标题在屏外。
- **修法**：滚动目标加 16px 的顶部滚动间距，并把分组标题一起带进视口。

**新-22 · U7 · 细节 · P3 · 可挽回 0.15**
- **位置**：Blockquote 左边框为 4px，正文约 20px（大写高 14px）。
- **现状 → 期望**：目标 radix-sink-blockquote 中，边框随字号变化：大号引文 6px，约 18px 的引文 4px。
- **修法**：边框宽度跟随字号，约为 max(4px, 0.25em)。

**新-23 · U7 · 细节 · P3 · 可挽回 0.1**
- **位置**：U7-fixture、scale110、scale95 三张图的第 0 行，在 x≈138–289 和 295–375 有灰色 1px 线段；390 那张没有。
- **现状 → 期望**：期望依据评分标尺「有没有溢出、遮挡」：夹具根元素顶边应干净，不混入根外元素的像素。
- **修法**：查出是哪个元素压进了夹具根的顶边，用 margin 或 clear 把夹具和前面的内容隔开。

## 4. 建议（不扣分）

1. **U5 菜单没对齐触发按钮**：菜单在 x386–545，「Actions」按钮在 x499–573；菜单顶边压在「Small」卡片的下边框上，底边离「Open dialog」只有 1px。建议左缘或右缘对齐触发按钮，并留 4px 间隙。
2. **U5 预览对话框**：1440 和 390 下都没有遮罩和阴影；390 下对话框左右贴到 x=1 和 x=389。建议加遮罩和阴影，390 下左右各留 16px。
3. **U5 拾取时的布局跳动**：开启拾取会在顶部插入约 124px 的面板；悬停样本时面板再长出 24px 的 token 行，悬停目标在指针下跟着下移。建议面板高度固定，或改成浮层。
4. **U5 DOM+GPU 顶部不齐**：GPU 列的标题和「Ready」把 GPU 框推到 y=219，DOM 卡片从 y=196 开始，建议顶部对齐。
5. **U5 类型样例间距**：「Caller class override」这一行和下面的衬线样例行之间只有约 4px。
6. **跨单元观察，属于 U1 的 G4，没有在 U5 扣分**：
   - 渲染器按钮的选中态只靠底色区分（#eff8ff 对 #f9fafb）和文字色（#175cd3 对 #1570ef），按下态很弱。
   - 所有按钮的边框都是约 #82b4f7，不是 G3 规定的选中边框 #84caff。
   - 「EN」和「中文」两个按钮紧贴在一起。
7. **U7 示例文案和排列**：
   - 「A bad prop keeps this view alive.」在默认状态下一词一行。
   - 「Your name」标签和输入框之间 0 间距。

   这两点不在比较口径里，但会让夹具看起来像出了故障。

## 5. 看不出的格子

**U5**
- **嵌套关系**：U5-compare-nested 看不出对比区在 DOM 上是否嵌在根 scope 里。
- **未采到的状态**：深色 scope 里打开的菜单和对话框没拍到（P3 浮层随创建处主题）；对浮层做拾取没拍到（P2「浮层同样可检查」）；中文下的 GPU 状态和失败、恢复文案没拍到（P4）；预览宽度 1024/768/390 档没拍到（P5）。
- **禁用态**：U5-dom 的「Clear local draft」是浅色、类似禁用的样子，但没有标注，看不出是真实禁用还是演示状态（P1）。
- **新组件覆盖**：只拍了 Cards 场景，看不出预览里是否有 IconButton、Link、Kbd、Code 等 P1 新组件；也看不出「let theme = resolve(document);」是不是 Code 组件。
- **焦点**：U5-inspector-picked 看不出被选 token 卡片是否拿到了键盘焦点，画面上没有焦点环。

**U7**
- **未出现的字号**：Text 的 1、2、4、5、7、8 档和 Heading 的 1–5、7、8 档都没出现，这些档的字号和行高无法核对。
- **Code、Kbd 变体**：Code 的 solid、soft、ghost，Kbd 的 classic 和各档尺寸，都看不到。
- **显式属性**：看不出现有的 Code、Kbd、第二个 Heading 有没有显式设 variant 或 weight，所以新-10、新-14 的根因是默认值错误还是样例没覆盖，要看源码才能确认。修法已经覆盖两种情况。
- **Container 对齐**：看不出 1136px 容器靠右是有意设的 align right 还是错误。
- **Reset 的来源**：看不出「Native reset action」的蓝色和 8px 缩进来自 Reset 本身还是 caller class。
- **非法属性**：U7-invalid 是从夹具中段截的视口图，看不到上方的控制区，也看不出是哪个属性被设成了非法值；console 诊断没有记录。

## 6. 总评

- **最低分是 U5（6.6）**，主要失分在拾取这条主路径：选中矩形套错部件、未声明部件的点击执行了真实动作、没有「未标注」提示、拾取后看不到来源说明。另外，Host control 在每一帧都遮住编辑区或预览。
- **U7（6.75）的核心问题是夹具无法和 Radix 目标对照**：缺字级梯度和变体矩阵，Inset 演示看不出贴边；Strong、Quote、Kbd、Heading 的字重和字形也和目标不一致。
- **需要结构性改动的地方**：
  1. **U7 夹具按 Radix 目标图的结构重建**：Heading、Text、Code 的 1–9 梯度；Code 4×4 矩阵；Kbd classic/soft × 1–9；6 张换行卡片；size 1–4 带描边的嵌套 Container；用可见容器面重做 Inset 演示（推倒重做）；多元素的 Reset 样例；可见的属性诊断行。
  2. **U5 拾取层重做命中与测量**：命中的是已声明部件本身、未命中时显示「未标注」、拦截真实点击、面板里给出来源说明。
  3. **U5 预览 scope 做成一块连续、有边框的作者面板**：带 padding 和描边，把 disclosure 一并包进去；Host control 移出固定定位，放回文档流。

开图清单：U5-dom-1440-light.png、U5-dom-1440-dark.png、U5-menu-1440-light.png、U5-dialog-1440-light.png、U5-compare-nested-1440-light.png、U5-inspector-1440-light.png、U5-inspector-card-1440-light.png、U5-inspector-1440-light-a2.png、U5-inspector-picked-1440-light.png、U5-gpu-compare-1440-light.png、U5-preview-390-light.png、U5-dialog-390-light.png、U7-fixture-1280-light.png、U7-scale110-1280-light.png、U7-scale95-1280-light.png、U7-invalid-1280-light.png、U7-fixture-390-light.png；目标图 radix-sink-heading.png、radix-sink-text.png、radix-sink-code.png、radix-sink-kbd.png、radix-sink-blockquote.png、radix-text-wrap.png、radix-sink-container.png、radix-inset.png、radix-reset.png
