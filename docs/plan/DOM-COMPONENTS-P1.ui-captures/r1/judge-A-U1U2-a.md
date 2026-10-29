# DOM-COMPONENTS-P1 UI 验收 R1 · U1、U2 评审

U1 得 **6.8** 分，U2 得 **5.8** 分，U2 分低。U2 的主要问题是精调这条主路径上的状态表达：token 网格不标覆盖，非法输入会被冲掉，低对比诊断不在预览侧也不能定位。两个单元各有几处要改结构，见第 6 节。

首轮，没有上一轮评审，所以省略复核表。U1、U2 是规范模式，期望来自目标包 G1–G6、C1–C4、F1–F4。判断只依据 26 张截图和 `U1-actions.log`、`U2-actions.log`；需要放大的局部，在 scratchpad `judge-A/` 下用 magick 裁剪查看，并对少量像素取了色。

## 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | **2.0** | **2.2** | **2.3** | **0.3** | **6.8** |
| U2 Studio 精调与来源解释 | **2.0** | **2.0** | **1.5** | **0.3** | **5.8** |

为什么不是更高一档：

**U1**
- **布局 2.0**：双栏结构、320px 编辑列、24/20 内边距、≤800 与 ≤600 的断点和「Back to edit」都对。不到 3 分的原因：
  - 「Host control」浮签在所有视口都压住编辑区内容；
  - 顶部没有主题名和模式（G5）；
  - 语言切换挤在一起、互相遮挡；
  - 品牌色下拉面板比触发器窄一半；
  - 200% 文本下壳层缩放混乱；
  - 390 底部多出一条残带。
- **视觉 2.2**：IBM Plex Sans、#d0d5dd 分隔、#98a2b3 搜索框边框、#eff8ff/#84caff/#175cd3 选中三色都对。不到 3 分的原因：
  - 未选中的切换按钮同样用蓝字蓝框，选中态只差一层几乎看不出的底色；
  - 字段 label 是 14px、中等字重，规范是 13px/600；
  - 选中的字段标签页不是半粗；
  - overridden 只有灰色说明文字，没有状态标识。
- **交互 2.3**：键盘能走到预设搜索、品牌色选择器、展开列表和模式按钮，焦点帧可见；切深色后编辑壳保持中性浅色；中文切换生效。不到 3 分的原因：
  - 选中的模式和渲染器没有可辨认的按下态（G4）；
  - 预览模式藏在编辑列中部，顶部看不到；
  - 中文态下「EN」被裁成「E|」，回切入口难以辨认。
- **细节 0.3**：差异较多。EN 被裁切；「Host control」未翻译；色族选项大小写不统一；Undo/Redo 是按钮而 Reset 是链接；标签页下有双分隔线。

**U2**
- **布局 2.0**：Colors / Type & other 两页、拾色区、分体 token 控件、390 可达都成立。不到 3 分的原因：
  - 来源面板与 token 网格之间隔着拾色器，选中网格里的 token 时来源在屏外；
  - Type & other 页的段落、小节标题、字段 label 层级倒置且没有间距；
  - 浮签遮挡。
- **视觉 2.0**：主体配色符合壳层 token。不到 3 分的原因：
  - 错误色用的是 #D92D20，规范是 #b42318；
  - 诊断中失败项与通过项的样式完全一样；
  - 网格里看不出哪些 token 被覆盖；
  - 分体控件两段边框颜色不同；
  - 说明段落（14px）比小节标题（13px）还大。
- **交互 1.5**：单项解除、分组解除都能完成，计数也会更新，所以不是 1 分。但以下几处都是规范明确要求的交互，所以只给 1.5：
  - 非法输入回车后被冲回有效值，红框配在合法值上，容易误解；
  - 低对比诊断只在编辑列，没有「定位来源」，预览侧没有任何提示（F3）；
  - 网格不标覆盖；
  - 解除后来源文案仍在讲「移除此覆盖」；
  - 切到 Type & other 页后来源面板仍显示颜色 token；
  - Reset 没有标明作用域。
- **细节 0.3**：差异较多。文案不随来源变化；「current mode」没有写明是哪个模式；字号不统一；标签名有歧义。

## 2. 上一轮差距复核

首轮，省略。

## 3. 差距清单

先按严重度、再按可挽回分排序。可挽回分合计：U1 为 3.3（10 − 6.8 = 3.2），U2 为 4.35（10 − 5.8 = 4.2），都在 ±0.5 以内。

### P1

**U2-01 · token 网格看不出覆盖状态和有效值**
- 单元与截图：U2；U2-source-overridden-1440-light（Surfaces / Semantic colors 两列网格，y≈590–900）、U2-source-overridden-390-light。
- 维度：交互、视觉、布局。
- 现状：Graphite 的 primary 是「Explicit document override」，但它的格子与生成值格子长得一样，只多了选中的蓝底。每个格子右侧的 ↺ 图标都是同一种浅灰，看不出哪个可以解除。格子只有色块和名称，没有色值。用户要逐个点开才能知道哪些被覆盖。
- 期望：F1 要求展示有效值、来源、覆盖与重置。C3 要求 overridden 用文字加状态标识表达，不能只靠颜色或悬停。F4 要求色值用等宽字。
- 修法：
  1. 改 token 分体控件。名称下加一行等宽 12px 的有效值，例如 `#606060`，颜色 #475467。
  2. 被覆盖的 token 在名称后加文字型状态标识，例如小号胶囊「Override」，1px #84caff 边框、#175cd3 文字，或者圆点加可见文字，不能只用颜色。
  3. 重置段只在有覆盖时可用。无覆盖时 `aria-disabled`，图标降为 #d0d5dd，并在 title/aria-label 写明「No override」。
  4. 搜索框旁加「Overridden only」筛选，计数与基础配置的「Overridden · Light 6」保持一致。
- 严重度：P1。可挽回分：**0.7**。

**U1-01 / U2-12 ·「Host control」浮签压住编辑区内容**
- 单元与截图：U1 和 U2，以下截图都能看到：
  - U1-default-1440-light (22,870)，压住「Inherit follows…」helper；
  - U1-top-1440-dark，压住外观选择器的值，「Inherit」几乎不可读；
  - U1-configured-1440-light，压住 Radius 的 helper；
  - U1-configured-1440-dark，压住拾色区；
  - U1-zh-configured，压住「缩放」label；
  - U1-default-768、U1-configured-768，压住 Brand/Scaling 区；
  - U1-edit-390 等，右下角；
  - U1-zoom200；
  - U2-source-overridden-390，压住 HEX 输入框；
  - U2-field-error，压住 Shadow opacity。

  中文态下它仍是英文。
- 维度：布局、交互（G6）。
- 现状：一个固定定位的小标签浮在编辑列左下角（390 时在右下角），随滚动遮挡任意内容。
- 期望：G1/G2 要求编辑壳稳定、各区独立滚动、不遮挡。G6 要求新增文案中英同步。规范中没有这个元素。
- 修法：从 Studio 壳中移除这个浮签。如果它是宿主测试入口，只在调试开关下渲染，或者放进顶部操作行的正常文档流（`position: static`），并补上中文文案「宿主控制」。如果必须悬浮，给编辑列滚动容器加等高的 `padding-bottom` 和 `scroll-padding-bottom`，且不得盖住表单控件。
- 严重度：P1。可挽回分：U1 **0.5**，U2 **0.2**。

**U2-02 · 非法输入被冲回有效值，红框配在合法值上**
- 单元与截图：U2；U2-field-error-1440-light，Geometry › Corner radius（y≈430–520）。
- 维度：交互。
- 现状：输入 `abc` 回车后，输入框显示 `0.35rem`，加红框，下方红字「Complete a valid value. The effective theme is unchanged.」。用户看不到自己输错了什么，看到的是一个合法值却被标错，无法就地修正。
- 期望：F1 要求错误贴近字段，未完成的输入不破坏有效预览。预览确实没变，这一半做对了，但草稿应保留在字段里。
- 修法：
  1. 这个字段采用 Draft 模式：非法文本保留在 input 中，设 `aria-invalid="true"`，用 `aria-describedby` 指向错误文字。
  2. 预览继续使用最后的有效值，并在错误文字中写明，例如「"abc" is not a length. Use px or rem, e.g. 0.375rem. Preview keeps 0.35rem.」。
  3. Esc 或失焦时才回到有效值，与现有的「Esc: cancel」习惯一致。
  4. 中文文案同步更新。
- 严重度：P1。可挽回分：**0.6**。

**U2-03 · 低对比诊断不在预览侧，失败项不醒目，不能定位来源**
- 单元与截图：U2；U2-low-contrast-1440-light。左列 Diagnostics › Text contrast pairs；右侧预览中「Build something that feels yours」「GPU · Cards」「Color tokens」等白字已经看不见。
- 维度：交互、视觉。
- 现状：
  - 诊断只在编辑列的折叠区里；
  - 「foreground / background 1.000:1 · Below text thresholds」「accent-foreground / accent 2.623:1 · Below text thresholds」与「Meets body-text threshold」的字号、颜色、字重完全相同，没有图标或强调；
  - 失败项没有排在前面；
  - 没有「定位 / Locate」动作；
  - 预览侧没有任何诊断提示。
- 期望：F3 要求在预览侧给出诊断和可定位的来源，同时保留作者颜色，编辑壳保持可读。编辑壳可读这一项已满足。
- 修法：
  1. 在预览头部、作者 scope 之外，用壳层样式放一条诊断条，例如「2 text pairs below 4.5:1」，列出失败的配对和比值。每项带一个「Locate」按钮，点击后切到 Colors 页、选中对应 token 并滚动到可见处。
  2. 在编辑列的诊断列表里，失败项加图标和文字「Fails」，用 #b42318，并排在最前。
  3. 折叠态的 summary 上显示失败计数。
- 严重度：P1。可挽回分：**0.6**。

**U1-02 · 选中与未选中的切换按钮几乎无法区分**
- 单元与截图：U1；以下几组按钮：
  - U1-keyboard-04（Light / Dark）；
  - U1-default-1440 工具栏的 DOM / GPU / DOM + GPU / Inspect；
  - U1-edit-390、U1-preview-390 的 Edit / Preview；
  - U1-default、U1-zh-top 的 EN / 中文。
- 维度：视觉、交互。
- 现状：未选中按钮是蓝字 #1570EF、浅蓝框 #87B5F5。选中按钮只多一层 #EFF8FF 底色，文字换成 #175CD3。选中与未选中的底色对比约 1.02–1.07:1，不足以作为状态指示。在 1440 首屏上看不出当前是哪个渲染器、哪个模式。
- 期望：G4 要求选中的模式和渲染器按钮使用按下态。G3 规定蓝色用于选中与焦点，并给出选中三色。状态指示的非文本对比度属于 WCAG AA 要求。
- 修法：把这几组改为分段控件，用 `role="radiogroup"` 或 `aria-pressed`，相邻段共用 1px 边框、间距为 0：
  - 未选中：白底或透明底，边框 #d0d5dd，文字 #344054 或 #475467，字重 500；
  - 选中：底 #eff8ff，边框 #84caff，文字 #175cd3，字重 600，可加 2px 内侧底边或内描边作为按下态。

  这样焦点环（3px、2px offset）不会再压到相邻按钮（U1-keyboard-04 中 Dark 的焦点环碰到了 Light）。
- 严重度：P1。可挽回分：**0.5**。

### P2

**U1-03 · 顶部缺少主题名和预览模式（G5）**
- 单元与截图：U1；U1-default-1440、U1-top-1440-dark、U1-configured-390、U1-keyboard-04。
- 维度：布局、交互。
- 现状：
  - 顶部只有标题、Undo/Redo/Reset、状态文字、Save/Import/Export。
  - 当前主题名只以 13px 小字出现在预设选择器下方（「Graphite」，y≈612）。
  - 预览模式按钮在编辑列中部，基础配置的 Scaling 之后（y≈777，要滚动才能看到）。
  - 预览头右侧只有「Live theme」。
  - 390 下编辑列滚动后，顶部只剩 Edit/Preview。
- 期望：G5 要求顶部保留主题名、模式、撤销/重做/保存/导出。C4 要求界面显示「预览模式」。
- 修法：在编辑列顶部做一个 sticky 头（滚动容器内 `position: sticky; top: 0`，白底，下边框 #eaecf0），包含：
  - 主题名，18px，后面跟状态文字，例如「Graphite · Modified」；
  - 「Preview mode」分段控件 Light/Dark，原位置的按钮移到这里，保留说明文字；
  - Undo/Redo、Save、Export。

  390 下把这一行并入 Edit/Preview 粘性条下方，至少保留主题名和模式。
- 严重度：P2。可挽回分：**0.5**。

**U2-04 · 来源文案不随来源类型变化，信息不完整**
- 单元与截图：U2；U2-token-released、U2-source-generated、U2-source-overridden-detail 的「Fine adjustment source」区。
- 维度：细节、交互。
- 现状：
  - primary 解除后、以及 ring 这种生成值，来源都显示「Generated by base configuration」，下面却仍写着「Removing this override reveals the nearest configured or authored base value…」，可此时并没有覆盖。
  - 「Generated by base configuration」没有说明由哪个配置项、哪个色阶生成，例如 Brand pink 9。
  - 显式覆盖时，不显示解除后会露出的值（解除后实际是 #d6409f）。
  - 按钮写「current mode」，没写是 Light。
- 期望：F2 要求来源说明能回答「值来自哪里」「哪个覆盖在生效」「解除将影响哪些项」，并显示范围。
- 修法：按来源类型给出不同模板。
  - 覆盖：「Explicit override · Light: #606060. Clearing reveals Brand pink · step 9 (#d6409f). Dark mode and other tokens unchanged.」，按钮文案「Clear primary · Light only」。
  - 生成：「Generated from Brand color pink · step 8 (Light). No override to clear.」，并隐藏或禁用解除按钮，同时说明原因。
  - 中英文同步。
- 严重度：P2。可挽回分：**0.5**。

**U2-05 · 来源与拾色面板远离 token 网格**
- 单元与截图：U2；U2-source-overridden-1440 与 U2-source-overridden-detail-1440 对比。
- 维度：布局。
- 现状：Colors 页的纵向顺序是：来源面板 → 搜索 → 拾色器 → 格式/HEX → 七条滑杆 → 网格。在网格中选中 primary（y≈766）时，来源面板大约在上方 700px、屏外，同一屏内不能同时看到「选了谁」和「它来自哪里」。
- 期望：F1/F2 要求展示有效值、来源与重置，让来源说明可读；G2 要求分组行结构。
- 修法：改成「搜索 → token 网格 → 选中 token 的编辑块」，编辑块包括来源、值和拾色器。编辑块有两种做法：
  - 内联展开在选中格子所在行下方，跨两列；
  - 或者在编辑列滚动容器内 sticky 在网格上方。

  两种做法都要保证选中 token 后，来源和解除按钮在同一视口内可见。
- 严重度：P2。可挽回分：**0.4**。

**U2-06 · Type & other 页层级倒置、间距缺失、label 顺序颠倒**
- 单元与截图：U2；U2-controls-1440（y≈285–420）、U2-field-error-1440（Geometry y≈392、Shadow y≈769–830）。
- 维度：布局、视觉。
- 现状：
  - 14px 的说明段「All 14 shared fields apply…」比后面的小节标题「Typography」（13px）还大，而且两者之间间距为 0。
  - 「Shadow」段落和「Shadow opacity」之间同样没有间距。
  - 「Geometry」与「Corner radius」只隔约 4px。
  - 「Choose a bundled font」选择器排在字段名「Sans font stack」之前，字段名夹在两个控件中间。
- 期望：G2 规定编辑区栈间距 16px，用细分隔线和分组行。G3 规定 label 13px/600、caption 12px。
- 修法：
  1. 小节标题统一为 13px/600、#101828，上方留 24px，加一条 #eaecf0 分隔线，与 Colors 页的「Surfaces」「Semantic colors」一致。
  2. 说明段改为 caption：12px/1.5、#475467，与下一个字段间隔 16px。
  3. 每个字段固定顺序：标题行（label 加右侧 Reset）→ bundled font 选择器 → 字体栈输入框 → helper。
- 严重度：P2。可挽回分：**0.4**。

**U1-04 · 语言切换挤压、遮挡，中文态下 EN 被裁切**
- 单元与截图：U1；U1-default-1440、U1-top-1440-dark（「EN中文」无间隙，选中底色压到「中」上）、U1-zh-top-1440（「EN」被盖成「E|」）。
- 维度：布局、细节。
- 现状：两个选项之间没有间距，选中底块的宽度覆盖到了相邻选项。
- 期望：G6 要求中英文主路径可达。回切英文的入口应完整可读。
- 修法：与 U1-02 统一，改为分段控件。每段 `padding: 0 8px; min-width: 32px; height: 28px`，共用 1px #d0d5dd 边框，选中段使用 #eff8ff/#84caff/#175cd3。两种语言下两个标签都必须完整显示。
- 严重度：P2。可挽回分：**0.3**。

**U1-05 · 200% 文本下壳层缩放不一致**
- 单元与截图：U1；U1-zoom200-720-light、U1-zoom200-export-720-light。
- 维度：布局。
- 现状：
  - 标题、按钮、label、选择器里的文字都没放大，仍是 px。
  - 选择器和渲染器按钮的高度与内边距却翻倍。「All sources」框高约 72px，文字仍是 14px。
  - 「Responsive」被截成「Respo…」。
  - 状态段「Draft remains in memory…」放大到约 28px，比 16px 的主标题还大，层级倒置。
  - 导出对话框的「Output」label 与选择器里的文字不放大，标题和代码放大。
- 期望：G6 要求 200% 放大下主路径可达，菜单和对话框可操作；G3 定义了壳层字级。
- 修法：
  1. 壳层字级统一用 rem 定义：body 0.875rem、label 0.8125rem、caption 0.75rem、标题 1.125rem、预览标题 1rem。
  2. 控件的 padding 和高度用同一单位体系，避免只放大盒子不放大字。
  3. 预览工具栏的选择器改成 `width: auto; min-width: max-content`，允许换行，不再截断。
- 严重度：P2。可挽回分：**0.3**。

**U1-06 · 基础配置的 overridden 和 Custom 状态没有状态标识（C3）**
- 单元与截图：U1；U1-configured-1440-light（「Overridden · Light 6 · Dark 6」等）、U1-keyboard-02（选择器值「Custom」）。
- 维度：视觉、交互。
- 现状：覆盖计数只是 12px 灰字，与上方 helper 同色同字号，夹在两行说明文字之间。选择器值「Custom」也没有任何标识。
- 期望：C3 要求 Custom 与 overridden 用文字加状态标识显示，不能只靠颜色或悬停，并显示「配置已选择，N 项被覆盖」类说明。
- 修法：把计数行改成一个独立的状态行，放在 helper 之前：
  - 左边是文字徽标「Overridden」，1px #84caff 边框、#175cd3 文字、11–12px/600、圆角 999px；
  - 右边写「6 light · 6 dark」；
  - 解除按钮与这一行同行或紧跟其后。

  选择器值为 Custom 时，在选择器内或 label 后加「Custom」徽标。无覆盖时显示「No overrides」并用中性灰。
- 严重度：P2。可挽回分：**0.3**。

**U2-07 · 重置作用域没有标出**
- 单元与截图：U2，证据来自 U1 截图：
  - U1-top-1440-dark 顶部的「Reset」；
  - U2-controls、U2-field-error 各字段右侧的「Reset」。
- 维度：交互。
- 现状：顶部「Reset」不说明是恢复预设还是恢复文档，也不说明影响范围。字段级「Reset」不说明两模式共用。
- 期望：F2 要求重置分为「解除该 token」「解除分组覆盖」「恢复预设/文档」三类，都要显示范围。
- 修法：
  - 顶部按钮改为「Restore preset · Graphite」，或在文档来源时写「Revert to saved document」，悬停和焦点时用 helper 列出影响项；
  - 字段级改为「Reset · both modes」，或在字段 helper 中写明「Shared by light and dark」；
  - 中文同步。
- 严重度：P2。可挽回分：**0.3**。

**U1-07 · 品牌色下拉面板比触发器窄一半**
- 单元与截图：U1；U1-keyboard-03-1440（面板约 158px，触发器约 279px）。
- 维度：布局。
- 现状：面板只占左半边，右侧露出 helper 文字「values. Explicit」，与面板边缘相接。键盘当前项的高亮是灰色 #f2f4f7。
- 期望：G4 要求控件键盘可用，G3 规定蓝色用于选中与焦点。
- 修法：
  - 面板 `min-width: anchor-width`（等于触发器宽度），`max-height` 加内部滚动；
  - 键盘当前项改用 #eff8ff 底和 #175cd3 文字，已选项保留 ✓。
- 严重度：P2。可挽回分：**0.2**。

**U1-08 · 390 编辑页底部多出一条残带**
- 单元与截图：U1；U1-edit-390、U1-configured-390、U1-preview-390、U2-source-overridden-390。
- 维度：布局。
- 现状：y=832 处有一条 1px #d0d5dd 横线，下方是 12px 的 #f9fafb 色带，把「Appearance」「Radius」label 拦腰切断。看起来是被隐藏的预览区或分隔条露出来了。
- 期望：G2 规定 ≤600 时用 Edit/Preview 标签替换双栏，并隐藏分隔条。
- 修法：
  - 非活动面板 `display: none`，分隔条 `display: none`；
  - 活动面板高度为 `100dvh` 减去粘性标签条高度，自己滚动；
  - 去掉这条横线和色带。
- 严重度：P2。可挽回分：**0.2**。

**U2-08 · 来源面板跨页残留**
- 单元与截图：U2；U2-controls-1440（Type & other 页顶部仍是「Fine adjustment source · ring · Generated by base configuration」和「Clear this token · current mode」）。
- 维度：交互。
- 现状：切到字体与其他页后，来源区仍显示一个颜色 token。
- 期望：F1/F2 要求来源说明对应当前编辑的项。
- 修法：来源面板按标签页作用域渲染。
  - Type & other 页显示当前聚焦字段的来源，例如「Corner radius · preset Graphite」或「Overridden」，并配该字段的解除按钮。
  - 没有聚焦字段时不显示。
- 严重度：P2。可挽回分：**0.2**。

**U2-09 · 错误色与输入框焦点样式不符合 token**
- 单元与截图：U2；U2-field-error-1440。
- 维度：视觉。
- 现状：
  - 错误边框和错误文字取色都是 #D92D20，规范是 #b42318。
  - 出错字段的焦点是 2px 实心 #1570EF、无 offset。
  - 同一套壳层中，搜索框焦点是 3px 半透明加 2px offset（U1-keyboard-01，取样为 #8AB7F7）；选择器焦点是 3px 半透明但没有 offset（U1-keyboard-02）。三种不一致。
- 期望：G3 规定字段错误色 #b42318。G4 规定 3px 半透明蓝 outline、2px offset。F4 规定非法输入使用壳层错误边框。
- 修法：
  - 统一使用 `--shell-error: #b42318` 作为边框和文字色；
  - 所有壳层输入和选择器的焦点统一为 `outline: 3px solid rgb(21 112 239 / .4); outline-offset: 2px`；
  - 错误态时焦点环与红框并存。
- 严重度：P2。可挽回分：**0.2**。

### P3

**U1-09 · 字级、字重与规范不符**
- 单元与截图：U1；U1-keyboard-02、U1-keyboard-04。
- 维度：视觉。
- 现状：
  - 字段 label「Brand color」「Appearance」大写字母高约 10px，约合 14px，字重约 500；
  - 小节标题「Presets」「Theme configuration」约 13px，也是约 500；
  - 选中的「Colors」标签页是常规字重。
- 期望：G3 规定 label 13px/600；G4 规定字段型标签页选中时 13px 半粗、2px 蓝色下边框。下边框已满足。
- 修法：
  - label 改为 `font: 600 13px/1.4`；
  - 标签页统一 13px，选中 600、未选中 500，并为两种字重预留宽度，避免切换时跳动。
- 严重度：P3。可挽回分：**0.2**。

**U2-10 · 同类元素字号不统一，标签页名有歧义**
- 单元与截图：U2；U2-low-contrast（「Diagnostics」12px，「Text contrast pairs」14px）；U2-source-overridden-detail（「Clear this token · current mode」12px，基础配置的解除按钮 14px）；标签页名「Type & other」，采集日志写的是「Fonts & other」。
- 维度：细节。
- 现状：同一层级的 disclosure 和按钮字号各不相同；「Type」这个词有歧义。
- 期望：F1 的分区名是 Colors / Typography / Other（字体与其他）。G3 有字级规定。
- 修法：
  - disclosure summary 统一 13px/600；
  - 次级按钮统一 13px；
  - 标签页改名「Typography & other」，中文「字体与其他」。
- 严重度：P3。可挽回分：**0.15**。

**U1-10 · 顶部操作样式不统一**
- 单元与截图：U1；U1-top-1440-dark 的 Undo/Redo/Reset。
- 维度：视觉、细节。
- 现状：Undo/Redo 是描边按钮，同一行的 Reset 是无边框链接。
- 期望：G3 要求壳层扁平且一致。
- 修法：Reset 使用与 Undo/Redo 相同的按钮样式（作用域文案见 U2-07）。
- 严重度：P3。可挽回分：**0.1**。

**U1-11 · 字段标签页下有双分隔线**
- 单元与截图：U1；U1-configured-1440-dark（y≈520 和 y≈537）、U1-keyboard-04（y≈847 和 y≈864）。
- 维度：细节。
- 现状：标签条自带一条下边框，下面 16px 处又有一条 hr。
- 期望：G2 要求细分隔线。
- 修法：删掉多出的 hr，只保留标签条的 1px #eaecf0 下边框。
- 严重度：P3。可挽回分：**0.1**。

**U1-12 · 选项名大小写不统一**
- 单元与截图：U1；U1-configured-1440、U1-keyboard-03、U1-zh-configured。
- 维度：细节。
- 现状：品牌色选项是小写「pink / gray / gold」，其他选项是首字母大写的「Custom / Auto / Inherit」。中文态下「自动」已翻译，「pink」仍是小写英文。
- 期望：G6 要求不翻译主题名和 token 名，但壳层文案风格应一致。
- 修法：把色族名作为 token 名呈现（等宽 12px，前面加色点），或统一首字母大写的显示名「Pink」并保留原值。中文态下保持不翻译，但样式要与其他选项区分开。
- 严重度：P3。可挽回分：**0.1**。

**U2-11 · 分体控件两段边框颜色不同**
- 单元与截图：U2；U2-source-overridden-1440 的 token 格子。主段边框取样 #D0D5DD，重置段边框 #ECEEF1。
- 维度：视觉。
- 现状：外轮廓一半深一半浅，看起来像两个控件，或者让人以为重置段被禁用了。
- 期望：F4 要求 token 与重置按钮合成一个外缘圆角的分体控件。
- 修法：两段都用 #d0d5dd 外框，中间用 1px 内分隔。可用与否只靠图标颜色和 `aria-disabled` 表达。
- 严重度：P3。可挽回分：**0.1**。

## 4. 建议（不扣分，目标包中没有依据）

1. 品牌色列表的每个选项前加对应 9 阶色点，让 26 个色族更容易挑选（U1-keyboard-03）。
2. 存储与操作区（Pending / recent operations、Retry retained saves、Cancel pending writes、Cache and original backups）占了编辑列最上方约 200px，把预设搜索推到 1440 的 y≈483、390 的 y≈531。可以把它收进顶部状态行的展开面板。另外，计数为 0 时「Retry retained saves」仍是可用态的蓝框按钮（U1-default-1440），这属于 U4/S4 的范围，交给 U4 评审核实。
3. 分组解除、单项解除之后，给一条可撤销的轻提示，例如「Cleared 12 brand overrides · Undo」（U2-group-reset、U2-token-released 都没有操作反馈）。
4. 预览头右侧的「Live theme」含义不清，可以改为「Graphite · Dark preview」，与 U1-03 的顶部信息呼应。
5. 交给 U5 评审：预览深色时，「Local demonstration · changes stay in this preview.」紧贴深色作用域色带的左缘，没有内边距（U1-configured-1440-dark、U1-top-1440-dark）；中文态下预览中的示例文案和 GPU 样本仍是英文。这些属于预览演示数据或 U5 的范围，本次不扣分。

## 5. 看不出的格子

| 图 | 看不出什么 |
| --- | --- |
| U1-default-768-light、U1-configured-768-light | 只拍到 DOM 区。≤1100 时 GPU 是否堆叠到 DOM 下方，不可见。 |
| U1 全部 | 分隔条 8px 的拖拽目标、编辑列 260–520px 的拖拽范围（G2）是静态图无法证明的，未验证。 |
| U1 全部 | ≤800 时「指标改两列」（G2），截图中没有指标区。 |
| U1-keyboard-01～04 | 字段型标签页（Colors / Type & other）和 390 下 Edit/Preview 标签的键盘焦点帧没有拍到（G4 要求有独立可见的焦点）。 |
| U1-keyboard-03 | 只展开了品牌色列表，看到前 8 项。中性色、外观、面板、圆角、缩放的选项全集（C2）没有展开，无法核对。 |
| U1-zoom200-export-720-light | 导出对话框内的复制、下载等动作不在可视区，看不出是否能经内部滚动到达。 |
| U2-source-overridden-1440-light | 网格里 ↺ 图标视觉上都一样，看不出 primary 的重置段是否可用（禁用与否没有区别）。 |
| U2-low-contrast-1440-light | 只拍了展开态。诊断折叠时是否有失败计数或警告，未拍到。 |
| U2 · 390 | 只有 source-overridden 一张。390 下的分组解除、字段错误、低对比都没有拍到。 |
| U1、U2 | 没有悬停态截图（U1-configured-768 中「Use configuration values」的浅蓝底疑似悬停残留，状态无法确认）。 |

## 6. 总评

- **最低分单元是 U2（5.8）。** 失分集中在精调这条主路径的状态表达：
  - token 网格不标覆盖和有效值（U2-01）；
  - 非法输入被冲掉（U2-02）；
  - 低对比诊断不在预览侧，失败项不醒目，也不能定位（U2-03）；
  - 来源文案不随来源类型变化（U2-04）。

  功能本身能走通：单项解除、分组解除、计数更新都正确。问题在于界面没有把状态说清楚。
- **U1（6.8）** 骨架、断点、token 色、键盘焦点基本到位。失分来自：
  - 遮挡内容的「Host control」浮签；
  - 切换按钮没有可辨认的选中态；
  - 顶部缺少主题名和预览模式；
  - 语言切换和 200% 文本下的排版问题。
- **需要结构性改动的地方：**
  1. **Colors 页重排**（U2-05）：改成「搜索 → 网格 → 选中 token 的编辑块」，编辑块内联展开或 sticky，同时在网格格子里加上覆盖标识和等宽色值（U2-01）。
  2. **编辑列顶部做 sticky 头**（U1-03）：承载主题名、Preview mode、撤销/保存/导出；预览模式按钮从基础配置区迁出。
  3. **所有切换按钮统一为分段控件**（U1-02、U1-04），选中态改用规范的选中三色并做出按下态。
  4. **预览侧加诊断条，并接通「定位」回 Colors 页的流程**（U2-03）。
  5. **壳层字级统一改用 rem**（U1-05），并移除「Host control」浮签（U1-01）。

开图清单：U1-default-1440-light.png、U1-configured-1440-light.png、U1-configured-1440-dark.png、U1-top-1440-dark.png、U1-zh-top-1440-light.png、U1-zh-configured-1440-light.png、U1-default-768-light.png、U1-configured-768-light.png、U1-edit-390-light.png、U1-configured-390-light.png、U1-preview-390-light.png、U1-zoom200-720-light.png、U1-zoom200-export-720-light.png、U1-keyboard-01-1440-light.png、U1-keyboard-02-1440-light.png、U1-keyboard-03-1440-light.png、U1-keyboard-04-1440-light.png、U2-source-overridden-1440-light.png、U2-source-overridden-detail-1440-light.png、U2-token-released-1440-light.png、U2-group-reset-1440-light.png、U2-source-generated-1440-light.png、U2-controls-1440-light.png、U2-field-error-1440-light.png、U2-low-contrast-1440-light.png、U2-source-overridden-390-light.png。U1、U2 是规范模式，没有目标图。另外打开过 DOM-COMPONENTS-P1.ui-target.md、capture.md、U1-actions.log、U2-actions.log，以及上述截图在 scratchpad `judge-A/` 下的裁剪放大图。
