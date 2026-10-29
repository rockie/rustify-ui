# U5 Studio 预览（真实组件、嵌套边界、浮层与拾取）· R2 独立评审

本轮按重建基线评审，评审单元为 U5。我看了 r2 中 14 张 U5 截图，并与 r1 的同名截图逐张对照；r1 拾取态以 `-a2` 为准。坐标都是 CSS px，取自 DPR 1 截图的像素采样。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.9 | 2.8 | 2.9 | 0.75 | **9.35**（R1 为 6.35） |

- **布局 2.9**
  - 以下各项都已正确：
    - 根预览与对比区的深色 scope 都是连续的整块，内边距约 24px，「Provider & overlay checks」行已移进 scope。
    - 菜单左缘与 Actions 触发器左缘对齐（x=524），紧贴在触发器下方。
    - 对话框在 390 宽度下左右各留 16px。
    - Host control 回到文档流，放在预览末尾，不再遮挡内容。
    - 390 宽度下 DOM 与 GPU 上下堆叠。
  - 没给 3 分：选 DOM + GPU 时，526px 宽的 DOM 列里卡片仍是两列，每张约 226px。标题「Create a local / profile」、开关标签「Local / notifications」都被挤成两行（新-3）。
- **视觉 2.8**
  - 以下各项都符合规范：
    - 编辑壳保持中性浅色，作者颜色只出现在 scope 和浮层里。
    - 对话框遮罩约为 #101828 的 28%，对应 30% foreground。
    - 选中态为 #eff8ff 底、#84caff 边框、#175cd3 文字，另加 2px 底条作按下态。
  - 没给 3 分有两个原因：
    - 未选中的切换按钮在 #d0d5dd 外框内多了一圈 #8AB7F7 浅蓝内描边，违反「蓝色只用于选中与焦点」（UI-02 残留）。
    - 拾取矩形与蓝色主按钮同为 #1570ef，又紧贴按钮边缘，几乎分辨不出（新-4）。
- **交互与状态 2.9**
  - 以下路径都走通了：菜单、对话框、局部边界选「Opposite to root」、对比区切 Dark、已声明部件的拾取、未声明部件的拾取（悬停时有虚线框和「Not annotated」标签，点击被拦截，计数仍是「Create · 0」）、拾取后跳到编辑器、DOM + GPU、390 下的预览与对话框。
  - 没给 3 分：拾取后编辑器滚动的落点不对。来源说明的第一行被置顶头部的下沿切成半行，「Fine adjustment source」标题不在视野内（UI-70 残留）。
- **细节 0.75**
  - 共五处小差异：
    - 拾取标签被截成「primary, primary-f…」；
    - 诊断条写成「1 text pairs」；
    - DOM 列与 GPU 列的内容内缩相差约 8px；
    - 未选中按钮的浅蓝内描边；
    - 编辑器里被切掉半行的文字。

## 2. 上一轮差距复核

| 编号 | R1 结论（U5 部分） | 本轮状态 | 依据（本轮截图） |
| --- | --- | --- | --- |
| UI-01 | Host control 浮签固定在视口上，遮挡内容 | **已解决**（遮挡）；中文文案**待核实** | local-opposite、menu、compare-nested 三张图中，Host control 在文档流里（x=344–425，y=821–843），位于「Compare an independent scope」下方，不压任何内容。390 两帧右下角已没有浮签。U5 没有中文帧，看不出文案是否已翻译。 |
| UI-02 | 选中与未选中几乎无法区分 | **部分解决** | 渲染器（DOM/GPU/DOM + GPU）、对比区 Light/Dark 和 Inspect 的选中态已清楚：#eff8ff 底、#84caff 边框、#175cd3 文字、2px 底条。未选中按钮的文字已是 #475467，但外框内仍多一圈 #8AB7F7 浅蓝内描边，见 dom-light 的 GPU、DOM + GPU、Inspect 和 compare-nested 的 Light。 |
| UI-65 | 深色 scope 被拆成没有内边距的深色岛 | **已解决** | dom-dark：深色 scope 从 x=344 到 1416、自 y=264 起连续，说明文字内缩约 26px，「Provider & overlay checks」在 scope 内。compare-nested 也一样（y=230–787）。 |
| UI-66 | 未声明部件没有「未标注」反馈，点击会穿透 | **已解决** | undeclared-hover：「Create · 0」外有中性虚线框，下方标「Not annotated」。undeclared：点击后计数仍是「Create · 0」。 |
| UI-67 | 已声明部件的拾取轮廓比部件宽很多 | **已解决** | inspector：轮廓 x=394–510，贴合「Create profile」按钮。 |
| UI-68 | 对话框没有遮罩；390 下左右只留 1px | **已解决** | dialog-1440 有遮罩（壳底由 255 变为 187）。dialog-390 的面板 x=16–373，宽 358 = 390 − 32。 |
| UI-69 | 菜单相对触发器左偏，盖住边界标题 | **已解决** | menu：菜单 x=524–684、y=540–618，左缘与 Actions（x=524–599）对齐。 |
| UI-70 | 拾取后来源说明不在视野内 | **部分解决** | picked：拾取面板里新增「Editor source: primary · Light · #1570ef · Override」，在视野内。编辑器只滚到 HEX 字段，来源段第一行「Clearing this override reveals Light: #1570ef. Other」在 y≈314 的置顶头部下沿被切成半行，标题「Fine adjustment source」不在视野。 |
| UI-71 | 默认场景缺 P1 新组件，Button 只有一种尺寸 | **已解决** | 新增「Component samples」：IconButton（✓）、size 1 的「Compact」、size 3 的「Large action」、Link「Edit theme」、Code「ThemeDocument」、Kbd「Tab」。 |
| UI-72 | 根内的局部边界没有外观选项 | **已解决** | local-opposite：边界里有「Boundary appearance」选择器，选「Opposite to root」后边界变深色，根仍为浅色。 |
| UI-73 | 深色下「Caller class override」对比约 1.8:1 | **待核实** | 本轮没有任何一帧能在深色下看到这个按钮。dom-dark 没滚到那里；compare-nested 的 Provider 区折叠着；local-opposite 里它在深色边界之外。 |
| UI-74 | 拾取标签压住上一行文字 | **已解决**（有新的截断问题，见新-1） | inspector、undeclared-hover：标签放在部件下方，不再压字。 |
| UI-75 | GPU 色样标签写 fg、card-fg 等 | **已解决** | gpu-compare：标签已是 foreground、card-foreground、popover-foreground 等完整名称。 |
| UI-76 | DOM 与 GPU 两列内容起点差 22px | **部分解决** | 两个框的上沿都对齐在 y=264。但框内首行起点不同：DOM 在 (370, 293)，GPU 在 (908, 285)，内缩分别约 26/29px 和 18/21px，还差约 8px。 |

## 3. 差距清单（未关闭，按严重度、再按可挽回分排序）

### UI-02（残留）· P2 · 视觉、细节 · 可挽回 0.15
- **位置**
  - U5-dom-1440-light：工具栏 GPU（x≈608–650）、DOM + GPU（x≈651–735）、Inspect（x≈745–806），y=65–92。
  - U5-compare-nested：对比区 Light（x≈345–389，y≈185–211）。
  - U5-gpu-compare：DOM、GPU 两个未选中段。
- **现状 → 期望**
  - 现状：未选中段在 #d0d5dd 外框内还有一圈 1px #8AB7F7 浅蓝内描边，看上去像弱化的选中或焦点。
  - 期望：「蓝色用于选中与焦点」（G3）；未选中是中性外观，选中用按下态（G4）。
- **修法**
  - 在 Studio 壳层的切换按钮或分段控件样式里，删掉 `aria-pressed="false"` 态上的 inset 描边或 ring：截图里的 #8AB7F7 来自一层 `box-shadow: inset 0 0 0 1px` 或 ring 类 utility。
  - 未选中态只保留：白底、`border: 1px solid #d0d5dd`、文字 #475467。
  - 单独的 Inspect 切换和预览模式 Light/Dark 用同一套规则。

### 新-4 · P2 · 视觉 · 可挽回 0.1
- **位置**：U5-inspector 与 U5-inspector-picked 中的「Create profile」（x=394–510，y=757–789）。
- **现状 → 期望**
  - 现状：拾取矩形是 2px #1570ef，按钮填充也是 #1570ef，而且矩形贴着按钮外缘。只能靠按钮底色略微变浅（#2C7EF1）和下方标签判断哪个部件被选中，矩形本身看不出来。
  - 期望：「选中矩形为固定蓝色 outline」（P2），而且矩形要能让人看出选中的是哪个部件。
- **修法**：outline 颜色不变（规范要求固定蓝色），从两点入手：
  - 加 `outline-offset: 2px`，让矩形离开部件 2px；
  - 在 outline 内侧垫一圈 2px 白色或画布色的光环，例如 `box-shadow: 0 0 0 2px #fff`。
  这样在蓝色、深色部件上也能分辨。矩形仍等于部件 rect 加 offset，不回到 UI-67 那种过宽的轮廓。

### UI-70（残留）· P3 · 交互、细节 · 可挽回 0.15（交互 0.1、细节 0.05）
- **位置**：U5-inspector-picked 左栏，y≈306–318。
- **现状 → 期望**
  - 现状：拾取后编辑器滚到 HEX 输入框。来源段第一行被 y≈314 的置顶头部下沿切成半行，来源标题「Fine adjustment source / primary / Explicit document override」在视野外。
  - 期望：拾取能把用户带到相关色阶、语义角色和覆盖来源（P2）；编辑壳文字不被遮住（G1）。
- **修法**
  - 拾取后的 `scrollIntoView` 目标改为来源块的根元素，不要定位到颜色字段。
  - 在编辑区滚动容器上设 `scroll-padding-top`，值为置顶头部高度（1440 下约 314px）加 16px；或者在来源块上设同值的 `scroll-margin-top`。
  - 保证任何内容都不会在置顶头部下沿露出半行。

### 新-3 · P3 · 布局 · 可挽回 0.1
- **位置**：U5-gpu-compare 的 DOM 列（x=344–870）。两张卡片约 226px 宽（x=369–595、x=619–845）。
- **现状 → 期望**
  - 现状：DOM 列只有 526px，场景仍是两列卡片，导致多处换行：
    - 「Create a local / profile」「Preview / preferences」「Local / notifications」都折成两行；
    - 勾选框文字折成两行；
    - Component samples 那一行拆成了三行。
  - 期望：「自动宽度跟随可用空间」，预览宽度选择器按 1024/768/390 模拟内容宽度（P5）。场景布局应随 scope 的内容宽度变化，而不是随视口宽度变化。
- **修法**：在预览 scope 根上设 `container-type: inline-size`。双卡网格改用容器查询，例如 `@container (max-width: 640px) { grid-template-columns: 1fr }`。这样 DOM + GPU 并排时，以及模拟 768/390 宽度时，卡片都会单列堆叠。

### UI-76（残留）· P3 · 细节 · 可挽回 0.05
- **位置**：U5-gpu-compare，DOM 首行「Local demonstration…」在 (370, 293)，GPU 首行「GPU · Cards」在 (908, 285)。
- **现状 → 期望**
  - 现状：两个框的上沿已对齐，但 DOM 内缩约 26/29px，GPU 内缩约 18/21px，差约 8px。
  - 期望：预览各区内边距一致（G2 预览 padding 24px）。
- **修法**：GPU 区宿主容器的内边距改成与 DOM scope 一致（24px），让两列首行左缘、上缘同线。也可以两边都用 16px，但要统一。

### 新-1 · P3 · 细节 · 可挽回 0.05
- **位置**：U5-inspector 与 U5-inspector-picked，按钮下方的蓝色标签（x≈396–508，y≈791–811）。
- **现状 → 期望**
  - 现状：标签宽度被限制成按钮宽度，被截成「primary, primary-f…」，「primary-foreground」和「ring」看不到。
  - 期望：标签是拾取结果的就地说明，应与面板列出的 token（primary / primary-foreground / ring）一致（P2）。
- **修法**：标签改为 `width: max-content; max-width: min(20rem, 预览区宽度)`，超长时换行，不要用 `text-overflow: ellipsis`。也可以写成「primary +2」这样的可读摘要。标签放在视口内，靠近部件。

### 新-2 · P3 · 细节 · 可挽回 0.05
- **位置**：U5-dom-1440-light、U5-inspector*、U5-gpu-compare、U5-preview-390 顶部的诊断条标题。
- **现状 → 期望**：「1 text pairs below 4.5:1 · author colors preserved」→ 「1 text pair below 4.5:1 …」。深色帧里的「2 text pairs」是正确的。
- **修法**：英文按数量选用单数或复数（用 `Intl.PluralRules` 或 one/other 两套键），中文不受影响。

可挽回分合计 0.65，对应 10 − 9.35。

## 4. 建议（不扣分，目标与规范里没有依据）

1. **工具栏控件高度不一**
   - 1440 宽度下：Cards、Responsive 选择器高 36px（y=61–96），渲染器分段和 Inspect 高 28px（y=65–92），Focus preview 高 38px（y=60–97）。390 宽度下，Inspect 和 Focus preview、Back to edit 在同一行，高度也不一致。
   - 建议统一为 32px 或 36px，垂直居中对齐。
2. **预览对话框过宽**：1440 下宽 1023px，正文只有一句话，Confirm 与 × 相距约 970px。X6 只约束导入/导出对话框，建议预览对话框按内容取宽，例如 `max-width: 28rem`。
3. **「Not annotated」标签用了选中蓝**（#175CD3），而对应的虚线框是中性灰。建议标签也改为中性色，例如 #475467 底配白字，与「已定位」的蓝色区分开。
4. **高部件的悬停标签看不到**：悬停「Create a local profile」卡片时，卡片底边在视口外，标签也在视口外（inspector-card）。建议部件底边超出视口时，把标签移到部件顶部内侧。
5. **开启拾取时内容整体下移约 175px**：拾取面板插入在预览顶部，指针下的目标会跳走。建议面板改为覆盖层，或预留它的位置。
6. **Host control 缺少说明**：它现在是预览末尾一个孤立的按钮。建议加一行说明，例如「Host element · outside the preview scope」，并补中文。
7. **IconButton 像已勾选的复选框**：✓ IconButton 紧贴「Compact」，会被读成「☑ Compact」。建议换一个不表示状态的图标（例如加号或齿轮），间距加到 8–12px。
8. **局部边界的层级倒置**：标题「A local theme boundary」约 14px/500，比它下面的字段标签「Boundary appearance」（约 15px/500）还小。建议标题升到 16px/600。
9. **外观选择器过宽**：「Boundary appearance」只有两个选项，选择器却横贯 938px。建议改为分段控件，或限制 `max-width: 15rem`。
10. **GPU 色样标签在连字符处折行**：例如「card- / foreground」，第二行容易被读成独立的 foreground token。建议缩小标签字号，或在窄列时改为两列网格。
11. **面板 token 按钮不显示当前打开的 token**：拾取后编辑器打开了 primary，但面板里的 token 按钮都没有显示为选中。建议对编辑器当前打开的 token 设 `aria-pressed=true`。

## 5. 看不出的格子

- **U5-menu-1440-light**：登记的焦点在 `menu-item-duplicate`，但 Duplicate 没有任何高亮或焦点框。动作记录只写了「open preview menu」，看不出是鼠标还是键盘打开的，因此无法判断焦点可见性是否达标。
- **U5-dialog-1440/390**：登记的焦点在 `preview-dialog-close`，× 周围看不到焦点环，原因同上。× 的点击区域大小也看不出。
- **P3「局部浮层随创建处的有效主题更新」**：菜单和对话框都只在「Inherit root」、根为浅色时打开过。没有从「Opposite to root」的深色边界或深色预览里打开浮层的帧，所以浮层跟随主题这一点无法验证。
- **P2「浮层同样可检查」**：没有拾取菜单或对话框内部件的帧。
- **U5-inspector-undeclared**：点击未声明部件后，拾取面板在视口外，看不出面板是否写明「无 token 映射」。
- **UI-73**：所有深色帧里都看不到「Caller class override」。
- **P4、G6**：U5 没有中文帧，也没有 GPU 失败或恢复态。GPU 状态文字（R1 的「Ready」本轮不显示）、Host control 和诊断条的中文文案都看不出。
- **P5**：只拍了 Responsive，1024/768/390 三档模拟宽度都没拍。
- **键盘走查**：U5 没有 Tab 遍历预览控件的帧。
- **深色根下的局部边界**（Inherit root 或 Opposite to root）：没拍。

## 6. 总评

- U5 是本次唯一评审的单元，R2 得 9.35，比 R1 的 6.35 明显提高。
- R1 的主要结构性问题都已关闭：
  - 深色 scope 不连续、没有内边距；
  - 未标注态缺失、点击穿透；
  - 拾取轮廓过宽；
  - 对话框没有遮罩、窄屏贴边；
  - 菜单偏位；
  - 局部边界没有相反外观；
  - 缺 P1 组件样例；
  - Host control 遮挡。
- 剩余失分没有一项需要推倒重做，都是样式或定位层面的修正：
  1. 壳层切换按钮未选中态多出的浅蓝内描边（UI-02 残留）；
  2. 拾取矩形在蓝色部件上看不出（新-4），改为加 offset 和白色光环；
  3. 拾取后编辑器滚动落点被置顶头部遮住（UI-70 残留），改用 `scroll-padding-top` 并改滚动目标；
  4. 场景布局没有按容器宽度单列（新-3），改用容器查询；
  5. 三处细节：标签截断、「1 text pairs」、DOM 与 GPU 内边距不一致。
- 下一轮应补拍「看不出」中列出的状态，尤其是从相反外观边界打开浮层、键盘打开菜单和对话框、中文与 GPU 失败态。没有这些帧，P3 和 P4 的关键条款只能维持未验证。
