# DOM-COMPONENTS-P1 UI 验收 R3 · U5 评审原文

- 评审范围：只评 U5「Studio 预览：真实组件、嵌套边界、浮层与拾取」，规范模式，无目标图。
- 依据：目标包 P1–P5、G1，以及 U5 比较口径（含 2026-10-27 R2 之后 UI-156 扩到 U5 的对比度条款）。
- 材料：
  - 本轮：r3 中 `U5-*.png` 共 18 张、`U5-actions.log`、`capture.md`。被测版本 HEAD `d5775d8a`，Theme Studio build `80cad3c530b54d9f`。
  - 上轮：r2 中同名 18 张、`r2-judge-U5.md`。上轮 HEAD `f5613b47`。
- 测量方法：坐标和颜色用 ImageMagick 逐像素取值，单位是 CSS px（DPR 1）。

---

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.9 | 2.95 | 2.8 | 0.8 | **9.45** |

上轮（R2，采用 b）为 9.1（2.9 / 2.85 / 2.6 / 0.75）。

### 各维度理由（以及为什么不是更高一档）

**布局 2.9**
- 做到的：
  - 拾取面板已移出文档流。U5-dom-1440-light 与 U5-inspector-card-1440-light 的「DOM」标签都在 y=308，开启拾取不再下推预览。
  - DOM + GPU 时 DOM 列改为单列。
  - DOM 与 GPU 两框的首行上沿对齐，都在 y 349–362。
  - 390 下对话框两侧各留 16px。
  - Host control 不受预览主题影响。
- 不是 3 的原因：
  - 拾取浮动面板（x 1000–1416，y 707/740–883）固定压在预览右下角，挡住被拾取的「Create · 0」和右卡的「Sample actions」「ThemeDocument」。
  - 拾取标签盖住相邻的复选框与文字。
  - 两框首行左内缩仍相差 8px。

**视觉 2.95**
- 做到的：
  - 分段控件三色正确：未选白底、1px #D0D5DD；选中 #EFF8FF / #84CAFF / #175CD3。#8AB7F7 内框和段间双线都消失了。
  - 拾取矩形改为「2px #1570EF + 2px 白 + 2px #1570EF」，在蓝色按钮上清晰可辨。
  - 未标注标签改用 #475467。
  - 面板为浅蓝 #EFF8FF，边框 #B2DDFF。
  - 编辑壳在深色预览下保持中性浅色。
- 不是 3 的原因：未标注目标被面板遮住时，矩形描边和「Not annotated」标签画在面板之上，压过面板文字，形成蓝灰线条交叠。

**交互与状态 2.8**
- 走通的：
  - 菜单、对话框、局部边界「Opposite to root」，以及局部浮层随深色更新。
  - 对比区 Dark、DOM + GPU。
  - 拾取：悬停、点击后焦点进入编辑器 #1570ef 字段；来源块标题完整可见。
  - 未标注说明、Inspect 按下态。
- 不是 3 的原因：
  - 三张未标注帧里，被拾取的部件大半藏在面板下面，用户看不到自己选中了什么。
  - 拾取 Create profile 时，标签（两行，36px 高）整片盖住相邻的复选框和「I understand」，拾取后这一状态一直保留。

**细节 0.8**
- 不是 1 的原因：
  - 标签在连字符处断成「primary-」与「foreground」。
  - 卡片标签切掉上一行文字的下伸部分。
  - 预览头部「Rustify Light」与「Light preview」之间丢了分隔符。
  - 渲染器分段和 Inspect 切换后，相邻控件横移 1–2px。
  - 两框首行左内缩差 8px。

---

## 2. 上一轮差距复核表

| 编号 | 上轮 | 本轮状态 | 依据（本轮截图） |
| --- | --- | --- | --- |
| UI-02 | P2 · 可挽回 0.15 | **已解决**（焦点部分待核实） | U5-dom-1440-light 工具栏 y 117–148：未选段 #FFFFFF 底、#D0D5DD 边，无 #8AB7F7；段间只有单条 1px（x=610、657）；选中段为 #EFF8FF 底、#84CAFF 边。U5-compare-nested 的 Light/Dark 也一样。「焦点环压邻段」这部分在 U5 没有键盘焦点帧，见第 5 节 |
| UI-110 | P3 · 0.05 | **已解决** | 浅色为「1 text pair below 4.5:1」，深色为「2 text pairs」，「1 recent operation」也用了单数 |
| UI-147 | P2 · 0.3 | **已解决**，但修法带出新问题 新-1 | 「DOM」标签开启拾取前后都在 y=308；6 张拾取帧里面板都在视野内 |
| UI-148 | P2 · 0.1 | **已解决** | U5-inspector-1440-light 的 Create profile：x 390–391 为 #1570EF，392–393 为 #FFFFFF，394–395 为 #1570EF，在 #2C7EF1 按钮上可辨 |
| UI-70 | P3 · 0.2 | **已解决** | U5-inspector-picked-1440-light：「Fine adjustment source」在 y≈97，页头底边 y=56，标题和第一行完整 |
| UI-149 | P3 · 0.15 | **已解决**，但带出新问题 新-2 | 标签 token 列表完整（「primary, primary-foreground, ring」）；卡片标签在 y 361–383，位于视口内 |
| UI-150 | P3 · 0.1 | **已解决** | U5-gpu-compare-1440-light 的 DOM 列：「Create a local profile」卡片宽 369–845，「Preview preferences」排在其下 |
| UI-151 | P3 · 0.1 | **已解决** | 「Not annotated」标签底色 #475467；面板中「Not annotated · …」（y≈831）到「All drawn tokens」（y≈859）的间距，与卡片面板 token 行到「All drawn tokens」的 28px 节奏一致，没有空行 |
| UI-76 | P3 · 0.05 | **部分解决** | 纵向已对齐：两框首行都在 y 349–362。横向仍差：DOM 首行 x=370（框边 344，内缩 26），GPU「GPU · Cards」x=908（框边 890，内缩 18），相差 8px |

---

## 3. 差距清单（先按严重度，再按可挽回分排序）

### 新-1 · [P2] 拾取浮动面板固定在预览右下角，遮住被拾取部件和预览内容；选中矩形与标签画在面板之上

- **单元与截图**：U5。
  - U5-inspector-undeclared-hover / U5-inspector-undeclared / U5-inspector-undeclared-panel：面板为 x 1000–1416、y 740–883。
  - U5-inspector-card / U5-inspector / U5-inspector-picked：面板为 x 1000–1416、y 707–883。
- **维度**：交互、布局、视觉。
- **现状**：
  - 悬停或点击预览主按钮「Create · 0」后，按钮位于 x≈1277–1366、y≈866–898，主体被面板压住。在 x=1282 取色：y 871–882 是面板底色 #EFF8FF，只有 y 884 以下露出按钮。
  - 选中矩形（外圈 #1570EF、内圈虚线 #475467）和「Not annotated」标签（y 849–869）反而画在面板上层，压过面板里的「All drawn tokens · keyboard selection」一行（y≈859）。
  - 刚开启拾取、还没滚动时，面板也挡住右卡的「Sample actions: 0」（只剩「Sample actio」）和「ThemeDocument」chip（只剩「ThemeDocume」）。
- **期望**：P2「Inspector（拾取）从实际部件定位……选中矩形为固定蓝色 outline，信息面板为浅蓝并带 token 按钮」。被定位的部件和信息面板要同时可见、互不遮挡；布局维度要求无遮挡。
- **修法**（需要结构性调整，不是调 z-index 就能解决）：
  1. 推荐把面板从覆盖层移进预览列的网格。拾取开启时，预览列改为 `display:grid; grid-template-rows: auto minmax(0,1fr) auto`，三行依次是标题与工具栏、预览滚动区、拾取面板。
     - 面板放在最后一行，不覆盖内容，样式为 `border-top: 1px solid #d0d5dd; max-height: 40vh; overflow-y: auto`。
     - 滚动区可视高度相应缩短，任何部件都能滚到面板上方。
  2. 选中矩形和标签的覆盖层放在预览滚动区内部，滚动区设 `overflow: clip`，覆盖层不可能画到面板上。
  3. 如果坚持浮动面板，至少做三件事：
     - 拾取期间给预览滚动容器加 `padding-bottom` 和 `scroll-padding-bottom`，值为面板高度 + 16px；
     - 悬停矩形与面板矩形相交时，把面板翻到预览右上角（工具栏下方）；
     - 覆盖层的 `z-index` 低于面板。
  - R2 给的「底部 overlay」选项本身就会带来这种遮挡，这里直说：应改用方案 1。
- **可挽回分**：0.25（交互 0.15、布局 0.05、视觉 0.05）。

### 新-2 · [P3] 拾取标签盖住相邻内容，并在连字符处断开 token 名

- **单元与截图**：U5。
  - U5-inspector-1440-light、U5-inspector-picked-1440-light：Create profile 上方的标签占 x 396–507、y 603–639，分两行「primary, primary-」和「foreground, ring」。
  - U5-inspector-card-1440-light：标签占 x 370–565、y 361–383。
- **维度**：细节、交互、布局。
- **现状**：
  - 标签宽度被限制成目标宽度（约 110px），文字换成两行，而且把「primary-foreground」断在连字符处。
  - 两行标签整片盖住复选框（x≈395–410）和「I understand」（x≈418–505），只露出「this is a local demo」。拾取后这一状态一直保留。
  - 卡片标签上沿从 y=361 开始，而上一行「Local demonstration · changes stay in this preview.」的字形在 y 349–362，「g」「p」「y」的下伸部分被切掉。
  - 标签还压住选中矩形的上边。
- **期望**：P2「从实际部件定位」时，不能把相邻的可拾取部件和预览文字遮掉；token 名是作者数据，应按原样完整显示。
- **修法**：
  - 标签单行，设 `white-space: nowrap; width: max-content; max-width: calc(100% - 16px)`（相对预览滚动区）。每个 token 包一层 `<span style="white-space:nowrap">`，只允许在「, 」处换行。
  - 标签内容缩成「首个 token +N」，例如「primary +2」；完整列表已经在面板的 token 按钮里。
  - 摆放规则：
    - 高度 ≥ 64px 的大部件（如卡片，内边距 24px），标签放在矩形**内侧**左上角，偏移 4px；
    - 小部件的标签放在矩形外侧、贴着上边，空间不够时翻到下边；
    - 设 `pointer-events: none`。
  - 拾取后（点击确认）只保留矩形，标签降为 hover 才出现，避免长期遮住邻近控件。
- **可挽回分**：0.15（细节 0.07、交互 0.05、布局 0.03）。

### 新-3 · [P3] 预览头部「Rustify Light」与「Light preview」之间丢了分隔符，读成「Rustify Light Light preview」

- **单元与截图**：U5-dom-1440-light、U5-gpu-compare 等 1440 帧，预览标题行右侧（y≈93，x 1266–1416）；U5-dom-1440-dark 为「Rustify Light　Dark preview」；U5-preview-390-light 在 y≈229。R2 同一位置是「Rustify Light · Light preview」。
- **维度**：细节。
- **现状**：两段纯文字只隔约 12px 空白，没有分隔，两个「Light」挨在一起。
- **期望**：C4（经 U5 预览头部体现）「界面显示『预览模式』以免与作者外观混淆」。主题名和预览模式要能一眼分开。
- **修法**：恢复「 · 」分隔（caption 12px #475467）；或把模式写成带标签的「Preview: Light」。两种都要同步中文文案。
- **可挽回分**：0.05。

### 新-4 · [P3] 选中或按下后字重变粗，按钮变宽，右侧控件横移 1–2px

- **单元与截图**：U5，测量取 y=122 的横向像素。
  - U5-dom-1440-light：Inspect 为 x 756–822，Focus preview 为 831–936。
  - U5-inspector-card-1440-light（Inspect 按下）：Inspect 为 756–824，Focus preview 为 833–937，右移 2px。
  - U5-gpu-compare-1440-light（选 DOM + GPU）：渲染器分段右缘从 747 变成 748，Inspect 起点变成 757，右移 1px。
- **维度**：细节。
- **现状**：选中或按下时标签改为粗体，按钮自身变宽，挤动后面的控件。
- **期望**：G4「选中的模式/渲染器按钮使用按下态」。按下态只改颜色和字重，不应造成布局位移；细节维度要求一个像素都不差。（Preview mode 的 Light/Dark 在两种状态下都是 942–1042，没有位移，可作对照。）
- **修法**：在分段和切换按钮的标签上预留粗体宽度。例如 `::after { content: attr(data-label); font-weight: 600; height: 0; visibility: hidden; overflow: hidden; display: block }`，或给每段按粗体宽度设 `min-width`。共用的分段组件改一次，渲染器分段、Inspect、对比区 Light/Dark 都会生效。
- **可挽回分**：0.05。

### UI-76 · [P3] DOM 与 GPU 两框首行左内缩相差 8px（部分解决）

- **单元与截图**：U5-gpu-compare-1440-light。
  - DOM 首行「Local demonstration…」从 x=370 起，框边 344，内缩 26。
  - GPU 首行「GPU · Cards」从 x=908 起，框边 890，内缩 18。
  - 两行纵向已对齐，都在 y 349–362。
- **维度**：细节、布局。
- **期望**：P5 下 DOM 与 GPU 并排比较，两框内容起点一致（沿用上轮口径）。
- **修法**：把 GPU region 场景根的 padding 改为 24px，与 DOM 预览 scope 的 24px 内边距一致；或者两边统一取同一个 token 值。
- **可挽回分**：0.05（细节 0.03、布局 0.02）。

**可挽回分合计**：0.25 + 0.15 + 0.05 + 0.05 + 0.05 = **0.55**，等于 10 − 9.45。

---

## 4. 建议（目标与规范里找不到依据，不扣分）

1. **简单对话框限宽、垂直定位**（沿用 R2 a、b，本轮有变化）：
   - 「Theme preview」对话框仍为 1023px 宽，内容只有一行文字加一个按钮。
   - 本轮从 R2 的垂直居中（y 372–528）改成贴顶：1440 下为 y 16–176，390 下为 y 16–197，下方留着大片空遮罩。
   - 建议简单对话框限宽 28–32rem，并垂直居中；导入导出类大对话框继续按 X6。
2. **菜单与触发器之间留 4px，加层级阴影**（沿用 R2 b）：U5-menu / U5-local-opposite-menu 中，Actions 底边 y≈538，菜单顶边 y≈539，几乎贴着，菜单只有描边。
3. **局部边界标题升级，外观选择器限宽**（沿用 R2 a、b）：「A local theme boundary」仍是 14px 常规字，与正文无区别；三选项的 select 横跨 937px。
4. **GPU 色样标签不要在连字符处断开**（沿用 R2 a、b）：U5-gpu-compare 中仍是「card-」与「foreground」、「primary-」与「foreground」、「popover-」与「foreground」。
5. **Host control 加一行说明**（沿用 R2 a）：说明它是边界外的宿主元素、不受预览主题影响。
6. **深色局部对话框的遮罩**（沿用 R2 b）：U5-local-opposite-dialog-1440-light 的遮罩取局部深色主题的 foreground，即约 30% 的 #F9FAFB。结果是边界外的浅色页面完全没变暗（F9FAFB 仍为 F9FAFB），只有深色区被冲淡（#101828 变成 #565C68），模态层次弱。建议遮罩取根主题或中性深色。
7. **面板补色阶信息**（沿用 R2 b）：面板目前列出语义角色和来源（例如「Editor source: primary · Light · #1570ef · Override」），没有色阶（如 blue 9）。按比较口径这属于 F19，只作建议。
8. **拾取面板提供折叠或最小化**（新）：面板只能靠 Esc 退出，没有收起入口。若采纳新-1 的方案 1，可在面板行头加折叠按钮。
9. **390 下的粘性页头**（新，属 U1 范围，这里只记对预览的影响）：U5-dialog-390 中页头 0–152px 一直固定，占 844 高视口的 18%，预览可视区相应变小。可考虑在 ≤600px 的 Preview 页把页头收成一行，或改为不粘顶。
10. ✓ R2 建议「工具栏控件高度统一」本轮已做到：Cards、Responsive、渲染器分段、Inspect、Focus preview 都在 y 117–148，高 32px。

---

## 5. 看不出的格子

| 截图 | 看不出什么 |
| --- | --- |
| U5-menu-1440-light、U5-local-opposite-menu-1440-light | capture.md 记录焦点在 menu-item-duplicate，但「Duplicate」没有高亮或焦点环。记录没写菜单是键盘打开还是指针打开，所以判断不了菜单项的键盘焦点是否可见 |
| U5-dialog-1440/390-light、U5-local-opposite-dialog | 焦点在 preview-dialog-close，「×」没有可见环。同样不知道打开方式，看不出对话框内的焦点是否可见 |
| U5-compare-nested-1440-light | 焦点在 compare-dark，但没有可见环，像是指针点击。UI-02 的「焦点环压邻段」在 U5 无法核实 |
| 全部拾取帧 | 「All drawn tokens · keyboard selection」始终折叠，没有展开帧。键盘拾取路径（G4、P2）看不出 |
| 无对应帧 | 拾取浮层内部件（P2「浮层同样可检查」）；深色预览下拾取；GPU 侧拾取；Esc 退出后 Inspect 恢复未按下 |
| 无对应帧 | P4：GPU 失败或恢复文字，以及中文态下的 GPU 状态文字 |
| 无对应帧 | P5：预览宽度 1024 / 768 / 390 档（Responsive 下拉未展开） |
| 无对应帧 | 预览控件的悬停与按下态；390 下的拾取面板位置 |

---

## 6. 总评

U5 的分数是 **9.45**，高于上轮 9.1。R2 留下的 9 条差距中，8 条已解决，1 条部分解决：
- 分段控件去掉了蓝色内框和双线；
- 拾取面板不再挤占文档流；
- 拾取矩形在蓝色按钮上可辨；
- 来源块不再被页头遮住；
- 标签 token 列表完整；
- DOM + GPU 时 DOM 列改为单列；
- 未标注态改用中性色；
- 单复数已修正。

剩下的主要失分集中在拾取的「定位」体验，而且是两项修复带出来的：
1. **新-1（P2）**：浮动面板固定压在预览右下角，被拾取部件反而藏在面板下面，选中矩形和标签画到面板上。这需要**结构性调整**：把面板从覆盖层移进预览列网格的最后一行（`grid-template-rows: auto minmax(0,1fr) auto`），覆盖层裁剪在滚动区内。只调 z-index 或位置解决不了。
2. **新-2（P3）**：标签按目标宽度换行，盖住邻近控件，还在连字符处断开 token 名。改成单行、缩写为「首个 token +N」、大部件把标签放进矩形内侧即可，不需要重做。

其余三条都是像素级细节：预览头部丢了分隔符、按下态加粗引起 1–2px 位移、GPU 框内缩差 8px。照单修完，U5 可以接近满分。
