# U5 Studio 预览：真实组件、嵌套边界、浮层与拾取（R2 评审）

U5 本轮得 **9.1 / 10**（上一轮 6.35）。上一轮的 12 条 U5 差距中，10 条已解决，UI-70 部分解决。UI-01、UI-02 中涉及 U5 的部分：UI-01 已解决，UI-02 部分解决。剩下 1 条 P2、4 条 P3，没有需要推倒重做的地方。

评审只用了指定材料：目标包、r2 中 U5 开头的截图、capture.md、U5-actions.log，以及 r1 的 U5 截图和 judge.md 指定的行。像素值是在截图上用 ImageMagick 取的样。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.9 | 2.85 | 2.6 | 0.75 | **9.1** |

- **布局 2.9**
  - 已对齐的部分：预览 scope 连续成块并有 24px 内边距；菜单对齐触发器；对话框在 390 下左右各留 16px；Host control 回到文档流；DOM 与 GPU 两列起点一致。
  - 不给 3 的原因：开启拾取时信息面板挤进文档流，把预览整体下推 175px。拾取后编辑区滚动的落点把来源块顶部压在粘性页头下面，切掉了半行字。
- **视觉 2.85**
  - 已对齐的部分：编辑壳保持中性；作者色只出现在 scope 及其浮层内；遮罩约 30%；深色下 Caller class override 的对比度已修复。选中态三色实测为 #EFF8FF / #84CAFF / #175CD3，与 G3 一致。
  - 不给 3 的原因：未选中的分段按钮和 Inspect 仍带一圈 #8AB7F7 蓝色内描边，违反 G3「蓝色用于选中与焦点」。未标注态用中性虚线框，标签却仍用选中蓝。
- **交互与状态 2.6**
  - 已走通：菜单、对话框、局部边界选 Opposite to root、浮层随局部主题、对比区 Dark、未标注反馈、拾取时拦截点击、DOM + GPU。
  - 不给更高的原因：拾取信息面板会随预览滚走。拾取页面较低处的部件时，token 按钮和来源摘要都不在视野内（补采时得手动把面板滚回来才能看到）。编辑区的来源标题被页头遮住。拾取标签截掉了 token 列表，较高的部件标签落在视口外。
- **细节 0.75**
  - 逐处比对，仍有几处小差异：标签被截成「primary, primary-f…」；未标注面板多出约 8px 空行；分段之间是 3px 的双线分隔；拾取后编辑区第一行字被切了一半。

## 2. 上一轮差距复核

| 编号 | 上一轮问题 | 本轮状态 | 依据（本轮截图） |
| --- | --- | --- | --- |
| UI-01（U5 部分） | 固定定位的 Host control 遮挡内容 | 已解决 | 在 `U5-dom-caller-1440-dark`、`U5-local-opposite*`、`U5-menu`、`U5-dialog`、`U5-compare-nested` 中，它位于对比区折叠行下方的文档流里（约 x=344, y=821），在 scope 之外，不压内容。390 下也没有浮签。中文态没采，G6 那一半待核实 |
| UI-02（U5 部分） | 选中与未选中几乎分不出 | 部分解决 | 选中态：#EFF8FF 底、#84CAFF 边、#175CD3 字，另有 2px #175CD3 下边条；未选中文字 #475467，已能一眼分辨。残留问题：未选中段在 #D0D5DD 外框内还有一圈 1px #8AB7F7 内圈，段与段之间叠成 3px 分隔（x=649–651）。见下方 UI-02 |
| UI-65 | 深色预览被拆成没有内边距的深色岛 | 已解决 | `U5-dom-1440-dark`：scope 根从 x=344 到 1416、从 y=264 起连续为 #0C111D，内容起点 x=369（24px）。「Provider & overlay checks」折叠行在 scope 内。`U5-compare-nested` 的对比区同样连续 |
| UI-66 | 未声明部件没有「未标注」反馈，点击还执行了真实动作 | 已解决 | 悬停时画出 #475467 虚线框（带 1px 白晕）并显示「Not annotated」标签。点击后按钮仍是「Create · 0」。面板写「Not annotated · this part has no declared token mapping.」 |
| UI-67 | 拾取轮廓比部件宽很多 | 已解决 | `U5-inspector-1440-light`：轮廓 x=394–510、y=757–789，与按钮一致 |
| UI-68 | 对话框没有遮罩；390 下贴边 | 已解决 | 遮罩把白底压到 #BBBEC3，约为 30% foreground。390 下面板在 x=16–373，即视口宽减 2rem |
| UI-69 | 菜单相对触发器左偏，盖住边界标题 | 已解决 | 菜单左缘 x=525，触发器左缘 x=524，在触发器正下方展开 |
| UI-70 | 拾取后来源说明不在视野内 | 部分解决 | 面板新增「Editor source: primary · Light · #1570ef · Override」，编辑区也会滚到 primary。残留：来源标题被粘性页头遮住，第一行被切一半；面板会随预览滚出视野（另列为新-1） |
| UI-71 | 预览缺少 P1 新组件，Button 只有一种尺寸 | 已解决 | 「Component samples」里有：✓ 方形图标按钮、Compact（小号）、Large action（大号）、Edit theme 链接、`ThemeDocument` Code、`Tab` Kbd 和标题。加上 Create profile，共三种按钮尺寸 |
| UI-72 | 根内局部边界没有外观选项 | 已解决 | 「Boundary appearance」可选 Inherit root / Opposite to root。选 Opposite 后边界变深色，边界外与 Host control 不受影响（`U5-local-opposite-1440-light`） |
| UI-73 | 深色下 Caller class override 对比约 1.8:1 | 已解决 | 现为 #FDB022 底、近黑字，对比度远超 4.5:1 |
| UI-74 | 拾取标签压住上一行文字的下伸部 | 已解决 | 标签改放在部件下方 2px 处，不再压字。新出现的截断问题另列为新-2 |
| UI-75 | GPU 色样标签名与 token 名不一致 | 已解决 | 已改为 foreground、card-foreground、popover-foreground 等 token 名 |
| UI-76 | DOM 与 GPU 两列起点差 22px | 已解决 | 两列标签都在 y=244，两个框都从 y=264 开始 |

## 3. 差距清单（按严重度、再按可挽回分排序）

### 新-1 · [P2] 拾取信息面板在文档流里：开启时预览下推 175px，拾取较低部件时面板不在视野内

- **单元与截图**：U5
  - `U5-dom-1440-light` 与 `U5-inspector-card-1440-light` 对比：「DOM」标签从 y=244 被推到 y=419。
  - `U5-inspector-undeclared-hover/-undeclared-1440-light`：预览滚到「Create · 0」时看不到面板。
  - `U5-inspector-undeclared-panel-1440-light`：补采时得把面板滚回顶部，才看到「Not annotated · …」。
- **维度**：交互、布局。
- **现状 → 期望**
  - 现状：「Inspect preview tokens」面板（x=344–1416，高约 158px）插在诊断条和 DOM 预览之间，会随预览滚动。只要被拾取的部件需要往下滚，「then choose a token」要用的 token 按钮和「Editor source」摘要就离开了视野。
  - 期望：在 P2 的拾取流程里，用户能从实际部件看到语义角色和覆盖来源，信息面板带 token 按钮。拾取任何位置的部件时，面板都应可见、可点；开启拾取不应让预览跳位。
- **修法**
  - 方案一：把面板改为预览滚动容器内的 `position: sticky; top: 0; z-index` 层，用 #eff8ff 底和 #b2ddff 下边框。或者改为停靠在预览列底部的 overlay，不占文档流。两种做法都消除 175px 的下推。
  - 方案二：把 token 按钮做成锚在拾取标签旁的小浮层，顶部面板只保留说明和「All drawn tokens」。
  - 无论选哪种，面板高度变化都不能引起预览内容重排。
- **可挽回分**：0.3（交互 0.25，布局 0.05）

### UI-70（部分解决）· [P3] 拾取后编辑区把来源块压在粘性页头下

- **单元与截图**：U5，`U5-inspector-picked-1440-light` 左列 y≈300–330。
- **维度**：交互、布局、细节。
- **现状 → 期望**
  - 现状：编辑列顶部有一块高约 314px 的粘性页头，底边分隔线在 y≈314。拾取后编辑区滚到 primary 拾色区。「Clearing this override reveals Light: #1570ef. Other」这一行被页头切掉上半。「Fine adjustment source / primary / Explicit document override」标题完全看不到，只露出「tokens stay unchanged.」和「Clear primary · Light only」。
  - 期望：拾取能定位覆盖来源（P2），来源块应完整出现在页头下方。
- **修法**
  - 在编辑区滚动容器上设 `scroll-padding-top: <粘性页头高度> + 16px`，或在来源块上设对应的 `scroll-margin-top`。
  - 滚动目标从拾色器改为来源块的标题，用 `scrollIntoView({block: 'start'})`。
- **可挽回分**：0.2（交互 0.1，布局 0.05，细节 0.05）

### UI-02（部分解决）· [P3] 未选中的分段按钮和 Inspect 仍带蓝色内圈，分段之间是 3px 双线

- **单元与截图**：U5，`U5-dom-1440-light` 工具栏 x=561–806、y=65–92；`U5-gpu-compare-1440-light`；`U5-preview-390-light` 工具栏；左列 Preview mode Light/Dark 同样如此。
- **维度**：视觉、细节。
- **现状 → 期望**
  - 现状：未选中的 GPU、DOM + GPU、Inspect（关闭态）是白底、1px #D0D5DD 外框，里面还有一圈 1px #8AB7F7。相邻两段的分隔是 #8AB7F7 | #D0D5DD | #8AB7F7 三条线，每段又各自带圆角，整组看起来是三颗拼在一起的按钮，不像一个分段控件。
  - 期望：G3「蓝色用于选中与焦点」，未选中态不应带蓝色；G4 选中态用按下态（这一点已满足）。
- **修法**
  - 删掉未选中态的内圈，也就是 #8AB7F7 的 inset box-shadow 或内层 border。
  - 分段组改为只有外框描边加圆角，内部段与段之间用一条 1px #d0d5dd 分隔。从第二段起用 `margin-left: -1px`，或只画 `border-left`。只有首尾两段保留外侧圆角。
  - Inspect 关闭态改用同一套中性样式。
- **可挽回分**：0.15（视觉 0.1，细节 0.05）

### 新-2 · [P3] 拾取标签截掉 token 列表；较高部件的标签落在视口外

- **单元与截图**：U5
  - `U5-inspector-1440-light` / `-picked`：标签「primary, primary-f…」（x≈396–508，宽度被限制为与按钮同宽），第三个 token「ring」看不到。
  - `U5-inspector-card-1440-light`：卡片轮廓从 y=500 一直延伸到视口底部以外，标签放在部件下方，画面里看不到。
- **维度**：细节、交互。
- **现状 → 期望**：P2 要求从实际部件定位语义角色。就地标签至少应完整显示角色名，或给出「primary +2」这类计数，并且始终出现在可见区域内。
- **修法**
  - 标签宽度不跟随部件宽度，改为 `max-width: 320px; white-space: normal`，最多两行；超出时显示「primary, primary-foreground +1」。
  - 定位加翻转逻辑：部件底边超出预览滚动区可见范围时，标签放到部件上方，或贴在可见区域底边内侧。
- **可挽回分**：0.15（细节 0.1，交互 0.05）

### 新-3 · [P3] 未标注态语义混用；面板多出一行空位

- **单元与截图**：U5，`U5-inspector-undeclared-1440-light`（x=1276–1366, y=433–488）、`U5-inspector-undeclared-panel-1440-light` 面板。
- **维度**：视觉、细节。
- **现状 → 期望**
  - 现状：
    - 未标注时，轮廓是 #475467 虚线并带白晕，但「Not annotated」标签底色是 #175CD3，与已标注的标签完全相同。只看标签分不出两种状态。
    - 面板里「Not annotated …」到「All drawn tokens」的行距约 36px，正常约 28px，等于空着的 token 按钮行还占着位置。
    - 另外，未标注的轮廓有 1px 白晕，已标注的轮廓（#1570EF）没有。
  - 期望：P2「找不到映射时明确显示未标注」，未标注与已标注应在标签上就能区分，面板不留空位。
- **修法**
  - 未标注标签改为中性样式：#475467 底配白字，或白底、#475467 虚线边、#475467 字。
  - token 列表为空时不渲染该容器，或去掉它的 margin。
  - 已标注轮廓同样加 1px 白晕，两种状态的几何保持一致。
- **可挽回分**：0.1（视觉 0.05，细节 0.05）

**可挽回分合计 0.9 = 10 − 9.1**。按维度：布局 0.1、视觉 0.15、交互 0.4、细节 0.25。

## 4. 建议（不扣分）

1. **预览对话框过宽**：1440 下面板宽 1024px（x=208–1231），里面只有一句话和一个按钮，关闭 × 离内容约 900px。X6 的「视口减 2rem、最大 64rem」只约束导入/导出对话框。建议简单对话框取 `max-width: min(100vw - 2rem, 32rem)`。
2. **菜单紧贴触发器**：`U5-menu` 中菜单顶边直接贴住 Actions 底边，间距 0，也没有层级区分。建议加 4px 的 side offset，再加一层轻阴影或更明显的边框，把浮层和页面内容分开。
3. **工具栏控件高度不一**：Cards/Responsive 下拉 36px，分段控件与 Inspect 28px，Focus preview 38px。390 下行距分别是 6px 和 8px。建议统一为 32px（或 36px），行距统一 8px。
4. **DOM + GPU 下 DOM 列过挤**：1440 DOM + GPU 时 DOM 列宽约 526px，场景仍是两列卡片（每张约 226px）。「Create a local profile」「Local notifications」等都被迫折行，左卡底部还空出约 134px。建议场景栅格用容器查询，容器宽度低于约 600px 时改单列。
5. **GPU 色样标签在连字符处断开**：「card-/foreground」这样拆成两行。建议加宽标签格，或改成两列色样。
6. **✓ 样本读起来像复选框**：「Component samples」中的 ✓ 方形按钮紧挨着「Compact」，容易被读成已勾选的选项。建议换一个不会被当成复选框的图标，并给出可见的 tooltip 或无障碍名称。
7. **390 下内边距叠加**：scope 根在 390 下仍有 24px 内边距，与列内边距 16px、卡片 24px 叠加后，内容只剩 258px。建议 ≤800px 时 scope 根内边距改为 16px。
8. **已标注轮廓在蓝色部件上不明显**：轮廓 #1570EF 与 Rustify Light 的 primary 按钮底色相同，offset 为 0，在蓝色部件上几乎融为一体。建议加 1px 白晕或 2px offset。
9. **同类按钮标注不一致**：「Create · 0」和「Create profile」都是 primary 实心按钮，一个未标注、一个已标注，容易让人误解。按目标包，完整覆盖不属于 F19 验收范围；建议按组件变体推导映射。
10. **深色局部对话框的遮罩几乎不可见**：从深色局部边界打开的对话框，遮罩是 30% 的浅色 foreground，盖在白色编辑壳上只差 2 个色阶（255 → 253），编辑区看上去没有被遮住。建议遮罩用根主题或中性深色。
11. **面板可显示色阶**：token 来自色阶时，面板可以补一行色阶信息，例如「blue · 9」，对应 P2 中的「相关色阶」。按比较口径不强制。
12. **边界外观选择器过宽**：「Boundary appearance」只有两个选项，却占满整行 938px。建议 `width: fit-content` 或 `max-width: 280px`。

## 5. 看不出的格子

- **`U5-menu-1440-light`、`U5-local-opposite-menu-1440-light`**：capture.md 记录焦点在 menu-item-duplicate，但「Duplicate」没有任何高亮。操作记录只写了「open preview menu」，看不出是键盘还是指针打开，因此无法判断键盘打开时焦点项是否可见。
- **`U5-dialog-*`**：焦点在 preview-dialog-close，但 × 上没有焦点环。同样看不出是不是键盘路径。
- **`U5-local-opposite-1440-light`**：焦点在 local-appearance，选择器上没有焦点环。看不出是否经由键盘选择。
- **只出现在截图外的状态**：
  - 浮层可拾取（P2「浮层同样可检查」）、GPU 样本拾取、390 下拾取都没采到。
  - GPU 失败与恢复文字，以及它是否跟随编辑器语言（P4），没采到。另外 r1 的 GPU 列有「Ready」状态行，r2 没有了，无法判断状态显示移到了哪里。
  - 预览宽度 1024/768/390 三档（P5）没采到。
  - 深色根下的菜单和对话框没采到。
  - 预览控件的悬停和按下态没采到。
  - 新文案的中文态没采到，包括 Boundary appearance、Opposite to root、Not annotated、Inspect 面板文字、Host control（G6）。
- **`U5-inspector-*`**：主题显示「Matches preset」，面板却写「Editor source: … · Override」。看不出这是不是预设文档本身存的显式值，所以无法判断来源说明是否准确。
- **`U5-inspector-picked-1440-light`**：拾取后键盘焦点进入 HEX 输入框（capture.md 记为 color-value）。面板写「Esc exits」，拾色区写「Esc: cancel」，看不出此时按 Esc 会退出拾取还是取消颜色输入。
- **「Component samples」中的 ✓ 方块**：从截图看不出它是 IconButton 还是复选控件。

## 6. 总评

U5 是本次唯一评审的单元，得 9.1，比上一轮 6.35 提高了 2.75。上一轮的结构性问题都已解决：
- 深色预览不再是一块块深色岛，scope 成为连续的一块；
- 加上了未标注态，拾取时点击不再穿透；
- 根内的局部边界可以选相反外观，菜单和对话框会跟随局部主题；
- 菜单、对话框、Host control 的位置都已修正；
- P1 新组件已补进预览场景。

剩下的失分集中在拾取的信息面板。它是文档流里的一块普通区块，开启时把预览下推 175px，拾取页面较低处的部件时又会滚出视野，编辑区的落点也没算粘性页头的高度。要做一处结构性调整：把面板改成预览容器内的 sticky 或 overlay 层，不占文档流。其余都是 P3 级的局部样式问题：
- 未选中分段的蓝色内圈；
- 拾取标签的截断和定位；
- 未标注标签的配色和空行。

这些不需要换组件或重做布局。
