# DOM-COMPONENTS-P1 UI 验收 R2 · 评审结论（合并）

本文件合并本轮 11 份评审原文。原文逐字保存在同目录：

- 首批（每单元一位）：[judge-U1-a.md](judge-U1-a.md)、[judge-U2-a.md](judge-U2-a.md)、[judge-U3-a.md](judge-U3-a.md)、[judge-U4-a.md](judge-U4-a.md)、[judge-U5-a.md](judge-U5-a.md)、[judge-U6-a.md](judge-U6-a.md)、[judge-U7-a.md](judge-U7-a.md)。
- 补采后重评（全新评审，同一批材料加补采帧）：[judge-U1-b.md](judge-U1-b.md)、[judge-U3-b.md](judge-U3-b.md)、[judge-U4-b.md](judge-U4-b.md)、[judge-U5-b.md](judge-U5-b.md)。

合并规则：

- 每个单元取总分较低那位评审的一整组四维分与理由，不逐维拼接。
- 差距取全部评审的并集。判为同一问题时合并到一个正式编号，「来源」列出各评审的临时编号或旧编号，并写明判为同一问题的依据。
- 同一条旧差距结论不同时，按 回归 > 仍存在 > 部分解决 > 待核实 > 已解决 > 不再适用 取最靠前的一项。例外：首批评审因缺帧判「待核实」、补采后的重评用补采帧判定的，以重评结论为准（UI-11、UI-73）。
- 合并条目的严重度取各评审中最高的一级；可挽回分按评审分别列出（a、b 指上面的原文）。
- 新差距从 UI-101 起编号，不复用旧编号。每条的完整取样数值、坐标和修法见对应原文。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.3 | 2.4 | 2.6 | 0.5 | **7.8** | b | a 8.1 |
| U2 Studio 精调与来源解释 | 2.2 | 2.4 | 2.65 | 0.5 | **7.75** | a | — |
| U3 Studio 导入与导出 | 2.4 | 2.65 | 2.8 | 0.5 | **8.35** | a | b 8.45 |
| U4 Studio 本地库迁移与存储反馈 | 2.4 | 2.3 | 2.4 | 0.45 | **7.55** | a | b 7.8 |
| U5 Studio 预览 | 2.9 | 2.85 | 2.6 | 0.75 | **9.1** | b | a 9.35 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.85 | 3.0 | 0.85 | **9.7** | a | — |
| U7 布局与排版 | 2.5 | 1.8 | 2.3 | 0.5 | **7.1** | a | — |

### 各维度理由（取自采用的那位评审）

**U1（b）**

- 布局 2.3：双栏、320px 编辑列、独立滚动、768 堆叠、≤800 padding、390 切换与「Back to edit」、Host control 回到文档流都对。扣分原因：粘性头在 200% 下封顶于 y=449、裁掉「Preview mode」并藏起模式与撤销/保存/导出；390 下滚动内容从标签条与粘性头之间透出；粘性头占视口 31–38%。
- 视觉 2.4：分隔色、搜索框边框、选中三色正确，壳层扁平，深色预览时编辑壳保持中性浅色。扣分原因：英文 label、预览标题、选中字段标签页看不出 600；辅助文字为 #667085；禁用按钮边框比可用按钮更重；未选分段带浅蓝内框；状态胶囊用选中蓝。
- 交互 2.6：键盘依次到达预设搜索、品牌色选择器、展开列表、Dark 与 Colors 标签页，焦点清楚；选中与未选可分；Overridden/Custom 有文字加标识；中文完整。扣分原因：200% 下关键动作藏在封顶头区里；390 预览页不能切模式、没有保存/导出；焦点环压到相邻分段；顶部「Modified」有歧义。
- 细节 0.5：复数文案、mauve 小写未翻译、头区垂直节奏、两栏标题基线、行尾悬挂分隔点、两种焦点环色、768 下主题名比主标题大。

**U2（a）**

- 布局 2.2：「搜索 → 筛选 → 网格 → 来源 → 拾色」顺序、预览侧诊断条、字段顺序与分隔线已对。扣分原因：选中 token 后来源块仍在屏外，且常驻壳层占 1440 编辑列约 315/900px；390 下两层吸顶之间漏出被裁切的网格内容。
- 视觉 2.4：错误色 #b42318、焦点环、标签页下划线、分体控件边框已对上 token。扣分原因：小节标题与字段 label 同样式；诊断区嵌套标题比外层大、正文比标题大；Override 胶囊借用选中蓝；选中分体控件只高亮一半；搜索框边框与 caption 灰偏离 G3。
- 交互 2.65：选中显示来源、单项解除、分组解除写明 both modes、非法输入保留 `abc` 并贴字段报错、预览侧诊断带 Locate 都走通。扣分原因：选中后不把来源带进视口；网格内 ↺ 只有图标，看不到作用范围。
- 细节 0.5：比值精度不一、错误文字重复 helper、句中色值非等宽、Override/Overridden 两种措辞、范围文案像被截断、token 名重复、诊断标题重复。

**U3（a）**

- 布局 2.4：对话框宽高上限、≤560px 单列、390 差异卡片正确。扣分原因：差异表被限高裁切（1440 截掉最后一组，390 只完整露出 2/6 项）；切换导出类型时对话框重新居中，Output 选择器位移 12–109px；390 分组行只铺约 105px。
- 视觉 2.65：次级文字、分隔、错误、选中色均对上，颜色值等宽。扣分原因：label、表头、节标题仍是 13px/400；备注框内多一条线；Rust 说明里的调用表达式只有函数名是等宽字。
- 交互 2.8：审阅（版本、名称、配置、分组差异、备注）、未知版本诊断与错误边框、v1 七条损失、CSS 两 profile 四记法、Rust 修订号与 0.2.1 拒收说明、绿色复制成功都走通。扣分原因：Apply 前候选差异无法一眼看全，被裁切且没有滚动提示。
- 细节 0.5：「256 / KiB」拆行；palette 与 color 混用、引号不统一；Auto 没写实际灰阶；备注与表格 token 命名不一致；表头「Mode / token」不准；`16.0` 与代码 `16.` 不一致。

**U4（a）**

- 布局 2.4：升级 Review 改为对话框且结构化，冲突面板先版本与差异后决策，浮签不再遮挡。扣分原因：状态 callout 与决策 callout 叠放、内容重复，上下顺序在升级与冲突中相反；命名动作红字夹在 callout 之间；对话框两组键值的值列没对齐。
- 视觉 2.3：有警告、错误、成功三种语义变体，次级文字与错误色符合 G3。扣分原因：决策 callout 标题与对话框分节标题是 13px 常规 #475467，弱于 14px 正文；并排 callout 正文一个 12px 一个 14px；按钮有四套描边/禁用样式；命名动作反馈是不在容器里的红字。
- 交互 2.4：七个状态都能走通，每个状态都有常驻反馈、继续动作和下载出口。扣分原因：冲突三个决策没说明后果；升级成功后过期的红色命名失败仍挂着、无内联动作；无锁态标题写「paused」而主按钮仍实心；取消后 callout 沿用待确认文案；页头「Modified / Matches preset」与存储状态基准不一致。
- 细节 0.45：约十处文案与像素问题，包括「1 recent operations」、「edit 2」孤字、缺句号、时态、修订号无标签、对话框双细线。

**U5（b）**

- 布局 2.9：预览 scope 连续并有 24px 内边距，菜单对齐触发器，390 对话框两侧各留 16px，Host control 回到文档流，DOM 与 GPU 起点一致。扣分原因：开启拾取时信息面板挤进文档流，把预览下推 175px；拾取后编辑区滚动落点把来源块顶部压在粘性页头下，切掉半行字。
- 视觉 2.85：编辑壳中性，作者色只在 scope 及其浮层内，遮罩约 30%，深色下 Caller class override 对比已修复，选中态三色符合 G3。扣分原因：未选分段与 Inspect 带 #8AB7F7 蓝色内描边；未标注态用中性虚线框而标签仍用选中蓝。
- 交互 2.6：菜单、对话框、局部边界 Opposite to root、浮层随局部主题、对比区 Dark、未标注反馈、拾取拦截点击、DOM + GPU 都走通。扣分原因：拾取信息面板随预览滚走，拾取较低部件时 token 按钮与来源摘要不在视野；编辑区来源标题被页头遮住；拾取标签截掉 token 列表，较高部件的标签落在视口外。
- 细节 0.75：标签被截成「primary, primary-f…」；未标注面板多约 8px 空行；分段之间 3px 双线；拾取后编辑区第一行被切半。

**U6（a）**

- 布局 3.0：四档高度 24/32/40/48，110% 为 26/36/44/52，内边距、IconButton 正方形、ghost/ghost-offset 几何、size 4 圆角、full 圆角、390 换行都与目标一致。
- 视觉 2.85：各变体选阶（soft 第 3 阶、surface 第 2 阶 + 第 7 阶描边、outline 第 8 阶描边、文字第 11 阶、disabled 中性灰）对上目标。扣分原因：classic 上暗下亮渐变幅度只有目标约 1/3，主体与 solid 几乎同色。
- 交互 3.0：13 张 DPR 2 特写与 gold、Unavailable、Link 焦点帧逐张量环宽、颜色、offset，全部符合 B2；Unavailable 焦点环清晰；loading 保留宽度与填充、spinner 可辨；110%、full、中文可用。
- 细节 0.85：ThemePanel 品牌色已本地化为「靛蓝色」/「Indigo」，「已解析中性色」仍显示原始 id「slate」。

**U7（a）**

- 布局 2.5：Container 四档 448/688/880/1136px 与三种对齐正确，Inset top/bottom 贴边，390 无溢出，240px 显式尺寸在 110%/95% 下不变。扣分原因：非法属性样例默认态塌成一列一词并有孤立 `px`；Heading wrap 行、Inset 左右贴边、Reset 元素覆盖面未达目标结构。
- 视觉 1.8：字级梯度、已测行高、Code 四变体、Kbd classic 斜面与 soft、Blockquote 边框随字号、Quote 衬线斜体正确。扣分原因：Heading 只比正文略粗；Text 的 light/regular/medium/bold 四行完全一样；Strong 不加粗。
- 交互 2.3：缩放 110%/95% 只生效一次，字号与行高按比例变化，wrap/trim/truncate 有呈现。扣分原因：非法属性态完全没有属性定位诊断；pretty 效果看不出。
- 细节 0.5：顶边外来灰线、孤立 `px`、布局原语区无 prop 标注、390 下 Kbd 键帽上下相贴。

## 未关闭差距

严重度取各评审最高的一级。「可挽回」后的 a、b 对应原文。

### U1

#### UI-101 · [P1] 200% 文本下粘性头被封顶为内滚动区，「Preview mode」被切半，模式、撤销、保存、导出不可见（U1 · 布局、交互 · 可挽回 a 0.55 / b 0.5）

- 位置：U1-zoom200-720-light 编辑列 y=0–449；U1-zoom200-export-720-light 遮罩后方头区已内部滚动。
- 现状 → 期望：头区限高约 50vh，关键动作只能靠键盘焦点带动内滚动露出 → G5 顶部保留主题名、模式、撤销/重做/保存/导出；G6 200% 下主路径可达，不靠隐藏的嵌套滚动。
- 修法：去掉头区 `max-height`/`overflow`；只让一条紧凑命令条吸顶（主题名 + 状态、Preview mode、Undo、Redo、Save、Export，其余收进 More），产品名、副标题、语言切换放回文档流；高度超过面板约 40% 时降为非吸顶。
- 来源：a 新-2；b 新-1（同一现象、同一帧、同一坐标 y=449）。

#### UI-102 · [P2] 390 编辑页的 Edit/Preview 标签条与粘性头之间有空隙，滚动内容从中透出（U1/U2 · 布局 · 可挽回 U1 a 0.2 / b 0.15，U2 a 0.25）

- 位置：U1-configured-390-light y≈45–85（「Presets … 44」半截）；U2-source-overridden-390-light y≈45–85（「sidebar-border #d0d0d0」被上下两层切开）。
- 现状 → 期望：两层吸顶之间漏出内容 → G1、G2（≤600px 编辑/预览分区稳定，不遮挡、不截断）。
- 修法：标签条与头区放进同一个不透明粘性容器，或头区 `top` 设为标签条高度并把间距改为粘性元素自身的 `padding-top`；或 ≤600px 时头区不吸顶。
- 来源：U1 a 新-1、b 新-2；U2 a 新-1（同一机制：两层吸顶之间的空隙，同在 390、同 y 范围）。

#### UI-103 · [P2] 粘性头过高，占视口 31–38%，挤占编辑区（U1 · 布局 · 可挽回 b 0.1）

- 位置：1440/768 各帧 y=0–314；390 标签条加头区 y=0–318。
- 现状 → 期望：头区 8 行 → G2 编辑区独立滚动；G5 只要求保留主题名、模式与撤销/重做/保存/导出。
- 修法：同 UI-101 的单行命令条方案，1440 下吸顶区控制在约 96px 内。
- 来源：b 新-3（a 在建议 1 提出同一点，未计分）。U2 a 在 UI-17 中也指出常驻区压缩可用高度。

#### UI-02 · [P2] 未选分段带 #8AB7F7 蓝内框、段间 3px 双线，焦点环压到相邻段（U1/U5 · 视觉、交互、细节 · 部分解决 · 可挽回 U1 a 0.2 / b 0.15，U5 a 0.15 / b 0.15）

- 位置：U1-default-1440-light 的 Light/Dark、DOM/GPU/DOM + GPU、Inspect；U1-keyboard-04 Dark 焦点；U5-dom-1440-light、U5-compare-nested、U5-gpu-compare 工具栏。
- 现状 → 期望：未选段在 #D0D5DD 外框内多一圈 1px #8AB7F7，EN/中文、Edit/Preview 没有；选中段另加 2px 下划线 → G3 蓝色只用于选中与焦点；G4 选中的模式/渲染器按钮用按下态。
- 修法：所有分段控件共用一个组件：未选段白底 #475467，去掉 #8AB7F7；段间单条 1px #D0D5DD；焦点环内收或 `z-index:1`，不压邻段。
- 来源：U1 a UI-02、b UI-02；U5 a UI-02（P2）、b UI-02（P3）。严重度取 P2。

#### UI-104 · [P2] 壳层次级与禁用按钮有 4 套样式，闲置按钮用焦点同色 #8AB7F7 做边框，禁用按钮比可用按钮更重（U1 · 视觉 · 可挽回 a 0.15 / b 0.1）

- 位置：U1-default-1440-light、U1-configured-1440-light 编辑列：Undo/Restore（#8AB7F7 边 #475467 字）、Import/Export（#8AB7F7 边 #1570EF 字）、Cancel pending writes/Retry（#858F9E 边 #667085 字）、Focus preview（#D0D5DD 边）；禁用 Redo 为 #858F9E 边、#F2F4F7 底。
- 现状 → 期望：闲置控件像半选中，禁用比可用更醒目 → G3 壳层扁平、蓝色只给选中与焦点、分隔 #D0D5DD/#EAECF0、次级文字 #475467。
- 修法：统一 ShellButton 次级样式（白底、1px #D0D5DD、#344054 或 #475467 字）与一套禁用样式（#F9FAFB 底、#EAECF0 边、#98A2B3 字）；#8AB7F7 只留给 `:focus-visible`。
- 来源：a 新-5；b 新-4（同一按钮体系、同样引用 #858F9E 非 token 边框）。

#### UI-09 · [P3] 英文 label、预览标题、选中字段标签页没有呈现 600 字重（U1 · 视觉、细节 · 仍存在 · 可挽回 a 0.15 / b 0.2）

- 位置：U1-configured-1440-light 的 Appearance、Brand color、Neutral color、Preview mode、Presets；U1-default-1440-light 预览「Preview」；U1-field-tabs-1440-light「Colors」。中文 label「外观」「预览模式」明显是粗体。
- 现状 → 期望：字号已是 13px，字重仍为常规 → G3 label 13px/600、预览标题 16px/600；G4 选中标签页 13px 半粗。
- 修法：确认 label 类为 600，并确认 IBM Plex Sans 600 字面以独立 woff2 从 public 加载（`document.fonts` 中有 600 条目）。
- 来源：a UI-09（部分解决）、b UI-09（仍存在），取仍存在。

#### UI-105 · [P3] 同一行壳层控件高度有 28/36/38px 三档（U1 · 布局、视觉、细节 · 可挽回 a 0.15 / b 0.05）

- 位置：1440 预览工具栏 Cards/Responsive 36px、分段与 Inspect 28px、Focus preview 38px；头区 EN/中文 38px、Light/Dark 28px；768/390 第二行。
- 修法：定义 `--shell-control-h: 2rem`，工具栏与头区所有控件统一，`align-items:center`。
- 来源：a 新-4；b 新-6。

#### UI-106 · [P3] 编辑区顶部 padding 16px、两栏标题不对齐，组内状态行与按钮间距 0px（U1/U2 · 布局、细节 · 可挽回 U1 a 0.15 + 0.1 / b 0.15，U2 a 0.05）

- 位置：U1-default-1440-light 编辑列首元素顶在 y=16，左右标题中线差约 8px；U1-configured-1440-light「• Overridden」胶囊底 y=745 与「Use configuration values · both modes」顶 y=746 相接；U2-group-reset-1440-light Brand color 下「● No overrides」胶囊与按钮相距约 1–2px。
- 现状 → 期望：G2 编辑区 padding 24px 20px 48px、栈间距 16px。
- 修法：编辑区（或粘性头内）`padding-top: 1.5rem`；标题行 `align-items:center`；配置组内 `display:grid; gap:.5rem`，状态行与按钮至少 8px。
- 来源：U1 a 新-6、a 新-9、b 新-7（b 把两点并为一条）；U2 a 新-3（同一组件：配置组状态行与解除按钮贴合）。

#### UI-107 · [P3] 次级文字用 #667085、颜色搜索框边框用 #D0D5DD，偏离 G3 token（U1/U2 · 视觉 · 可挽回 U1 a 0.1 / b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light 的 Theme configuration 说明、Appearance/Brand color 帮助文字、「Light 6 · Dark 6」、「Resolved neutral: mauve.」；U2-controls-1440-light「Choose a bundled font」caption；U2-source-overridden-1440-light「Search colors…」边框。
- 现状 → 期望：两种次级灰并存，颜色搜索框与预设搜索框边框不同 → G3 次级文字 #475467、搜索框边框 #98A2B3。
- 修法：helper/caption 统一 `--shell-text-secondary: #475467`；颜色搜索框复用预设搜索框的 #98A2B3 边框；#667085 只留给禁用态。
- 来源：U1 a 新-3、b 新-5；U2 a 新-6（同一 G3 token 偏差）。

#### UI-108 · [P3] 粘性头下边框下方 17px 又有一条分隔线，形成双细线（U1/U4 · 细节 · 可挽回 U1 a 0.05，U4 b 0.05）

- 位置：U1-default-1440-light y=314 与 y=331；U1-default-768-light、U1-zh-top-1440-light、U1-zoom200-720-light 同理；U4 各 1440 帧 y=282/299 或 314/331。
- 修法：滚动区第一个分组去掉上边框，或头区只在内容滚动后显示下边框。
- 来源：U1 a 新-7；U4 b 新-5（同一对线）。U4 a 在建议中指出同一现象并归 U1。

#### UI-109 · [P3] 状态胶囊（Overridden、Custom、网格 Override）借用选中态蓝色（U1/U2 · 视觉 · 可挽回 U1 b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light、U1-configured-lower-1440-light「• Overridden」「• Custom」（#EFF8FF/#84CAFF/#175CD3）；U2-source-overridden-1440-light 网格「Override」胶囊（选中的 primary 格内只剩一道描边）。
- 现状 → 期望：状态与选中、动作同色 → G3 蓝色用于选中与焦点；C3 状态以文字 + 标识表达（已满足）。
- 修法：状态胶囊改中性（#F9FAFB 底、#D0D5DD 边、#475467 字，12px/600，保留前导圆点）。
- 来源：U1 b 新-10（a 在建议 2 提出）；U2 a 新-4（并注明 U1 胶囊同色，建议同步）。

#### UI-110 · [P3] 「1 text pairs below 4.5:1」单复数错误（U1/U5 · 细节 · 可挽回 U1 b 0.1，U5 a 0.05）

- 位置：U1-default-1440/768、U1-field-tabs、U1-keyboard-01 至 03、U1-zoom200；U5-dom-1440-light、U5-inspector*、U5-gpu-compare、U5-preview-390 诊断条标题。
- 修法：按数量选择单复数（`Intl.PluralRules`）。
- 来源：U1 b 新-8；U5 a 新-2。U4 a 在建议中指出同一处。

#### UI-111 · [P3] 768 下主题名（18px）比主标题（16px）大；1440 下两个 18px 标题并列（U1 · 细节 · 可挽回 b 0.1）

- 修法：每栏只保留一个 18px 标题（建议主题名为主标题，≤800 时 16px），或两者都遵守 ≤800 的 16px。
- 来源：b 新-9（a 在建议 5 提出两个 18px 标题竞争）。

#### UI-112 · [P3] 分隔点「·」悬挂在行尾，Overridden 胶囊后多一个点（U1 · 细节 · 可挽回 b 0.1）

- 位置：U1-zoom200-720-light 预览右上「Rustify Light ·」；U1-configured-1440-light「• Overridden · Light 6 · Dark 6」。
- 来源：b 新-13。

#### UI-113 · [P3] 壳层焦点环有多种颜色/透明度（U1 · 细节 · 可挽回 b 0.05）

- 位置：U1-keyboard-01 预设搜索 #8AB7F7；U1-keyboard-02 品牌色选择器 #A1C6F9；U1-keyboard-04 Dark #97C2F9/#A1C6F9。
- 修法：搜索框、选择器、分段控件共用一个焦点 token（3px、40%、2px offset）。
- 来源：b 新-11；a 在 UI-02 余项中列出同一现象。

#### UI-114 · [P3] 页头「Modified / Matches preset」的比较基准不清，与存储状态读法冲突；吸顶区没有保存状态（U1/U4 · 交互、细节 · 可挽回 U1 b 0.05，U4 a 0.1）

- 位置：U1-top-1440-dark「Graphite Modified」与下方「Saved on this device」；U4-upgrade-pending 页头「Matches preset」对话框「Draft state: Matches saved document」；U4-upgraded「Modified」紧挨「Saved on this device」。
- 修法：页头写明基准（「Modified from preset · N overrides」），吸顶区加按实际写入结果显示的保存状态胶囊；对话框用相同措辞。
- 来源：U1 b 新-12；U4 a 新-12（同一页头标签）。

#### UI-03 · [P3] 390 预览页顶部没有预览模式切换与保存/导出（U1 · 交互 · 部分解决 · 可挽回 b 0.1）

- 位置：U1-preview-390-light 顶部只有「Graphite · Light preview」文字。
- 修法：UI-101 的紧凑命令条放在 Edit/Preview 标签条下，两个标签页共用；或预览工具栏首行放 Light/Dark 与溢出菜单。
- 来源：b UI-03（部分解决）；a 判已解决。取部分解决。

#### UI-05 · [P3] 200% 下导出对话框动作按钮高度固定 32px，没有随字号放大（U1 · 布局 · 部分解决 · 可挽回 b 0.05）

- 位置：U1-zoom200-export-end-720-light 的 Copy code、Select text、Download（y 726–757）。
- 修法：`min-height: 2.25rem; padding: .375rem .75rem`，单位用 rem。
- 来源：b UI-05（部分解决，基于补采帧）；a 在补采前判已解决。取部分解决。

#### UI-12 · [P3] 自动配对的中性色名显示为小写原始 id 且不翻译，与品牌色显示规则不一致（U1/U6 · 细节 · 部分解决 · 可挽回 U1 a 0.1 / b 0.05，U6 a 0.15）

- 位置：U1-configured-1440-light「Resolved neutral: mauve.」；U1-zh-configured-1440-light「已解析中性色：mauve。」；U6-fixture-1280-light「Resolved neutral: slate.」；U6-panel-zh-1280-light「已解析中性色：slate。」（品牌色已显示「靛蓝色」/「Indigo」）。
- 修法：auto 结果与选项列表共用同一个本地化显示名函数。
- 来源：U1 a UI-12、b UI-12；U6 a 新-1（同一 ThemePanel 字段的同一显示规则问题）。

### U2

#### UI-17 · [P2] 选中 token 后来源与解除按钮不在屏内（U2 · 布局、交互 · 仍存在 · 可挽回 a 0.55）

- 位置：U2-source-overridden-1440-light 选中 primary 后整屏看不到「Fine adjustment source」；U2-source-overridden-detail-1440-light 来源块在整片网格之后（Shadow 组后）；U2-source-overridden-390-light 同样。1440 编辑列 0–315px 为常驻壳层。
- 现状 → 期望：要滚过约 50 个 token 才能读到来源 → F1 展示有效值、来源、覆盖与重置；F2 来源回答三个问题。
- 修法：来源 + 拾色块作为整行插入所选格所在行下方（`grid-column: 1 / -1`），或做成编辑列内 `position: sticky; bottom: 0` 的可折叠抽屉；选中时 `scrollIntoView({block:'nearest'})`；同时压缩常驻区（见 UI-101/UI-103）。
- 来源：a UI-17。

#### UI-115 · [P2] 网格内 ↺ 只有图标，看不到解除范围（U2 · 交互 · 可挽回 a 0.15）

- 位置：U2-source-overridden-1440-light 及 detail 图每个 token 格右侧 ↺ 段。
- 现状 → 期望：来源块写「Clear primary · Light only」，网格里同一动作只有图标 → F2 解除须显示范围；C3 不只靠颜色或悬停。
- 修法：胶囊文字带模式（「Override · Light」）；↺ 加与来源块同文案的 `aria-label`；未覆盖格 `aria-disabled`。
- 来源：a 新-8。

#### UI-18 · [P3] 小节标题与字段 label 同样式，层级平（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-controls-1440-light「Typography」对「Sans font stack」；U2-field-error-1440-light「Geometry」对「Corner radius」，同为 #1D2939、同字号字重。
- 修法：小节标题用壳层分组眉题样式（12px/600 大写、#475467，上分隔线 24px、下 12px），字段 label 保持 13px/600。
- 来源：a UI-18 余项。

#### UI-22 · [P3] 诊断区嵌套标题倒置、正文大于标题、等宽字号混用（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-low-contrast-1440-light 编辑列 y≈352–700：外层 summary 约 12px、内层约 13px；「Authored colors fit…」约 14px 大于其标题；字体诊断条目之间无间距。
- 修法：两级 summary 统一 13px/600、内层缩进 12px；正文 12px #475467；token 名统一 12px 等宽；条目间 8px 加分隔线。
- 来源：a UI-22 余项。

#### UI-116 · [P3] 预览侧诊断条中 Locate 与所属色对的对应关系不清（U2 · 布局 · 可挽回 a 0.1）

- 位置：U2-source-overridden-1440-light 诊断条（色对文字到 Locate 相隔约 220px）；U2-group-reset、U2-controls 三列时 Locate 与下一对只隔 16px。
- 修法：单列 `grid-template-columns: 1fr auto`，每行一对，行间分隔线；超过 4 对时折叠。
- 来源：a 新-2。

#### UI-117 · [P3] 同一对比数据两处精度不同（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440-light 预览条「1.00:1」「2.62:1」，编辑列「1.000:1」「2.623:1」。
- 来源：a 新-10。

#### UI-118 · [P3] 错误文字重复了上方 helper（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-field-error-1440-light Corner radius 下方。
- 修法：出错时用错误文字替换 helper（同一 `aria-describedby` 节点）。
- 来源：a 新-11。

#### UI-119 · [P3] 选中的分体控件只高亮左半段（U2 · 视觉 · 可挽回 a 0.05）

- 位置：U2-source-overridden-1440-light primary 格：左段 #84CAFF/#EFF8FF，↺ 段仍 #D0D5DD/白。
- 修法：选中样式加在分体控件外层，外框与中间分隔统一为 #84CAFF（F4 外缘圆角的分体控件）。
- 来源：a 新-5。

#### UI-120 · [P3] 句中色值不是等宽字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-source-overridden-detail-1440-light「Clearing this override reveals Light: #d6409f.」。
- 来源：a 新-12。

#### UI-121 · [P3] 同一概念两种措辞：Override / Overridden（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-13。

#### UI-122 · [P3] 范围文案「−0.5… 0.5em」像被截断（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-field-error-1440-light Letter spacing helper。
- 来源：a 新-14。

#### UI-123 · [P3] 来源块里 token 名出现两次（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-15。

#### UI-124 · [P3] 诊断区「Diagnostics」与「Preview diagnostics」标题重复（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-16。

### U3

#### UI-125 · [P2] 差异表放在限高的内层滚动区，Apply 前看不全改动，且无滚动提示（U3 · 布局、交互 · 可挽回 a 0.45 / b 0.35）

- 位置：U3-import-review-end-1440-light 在 y≈715 把「Dark · overrides (1)」拦腰截断；U3-import-review-end-390-light 只完整露出 2/6 项；U3-import-review-bottom-1440/390 显示内层（`div.tools-scroll`，`overflow-y:auto`）滚到底后的内容；U3-actions.log 记录两层滚动。
- 现状 → 期望：对话框本体与差异表两层纵向滚动 → X1 Apply 前完整呈现候选差异；X6 对话框内部只有一层纵向滚动。
- 修法：去掉审阅中 `.tools-scroll` 的 `max-height`/`overflow`，表格随对话框本体滚动；极长差异才启用内层滚动，并加滚动阴影与「Showing N of M changes」；≤560px 不嵌套。
- 来源：a 新-1；b 新-1（同一容器、同一帧）。补采确认容器是 `overflow-y:auto`，a 提出的「若是 hidden 应升 P1」不成立。

#### UI-26 · [P2] 对话框 label、表头、节标题字重仍为 400（U3 · 视觉 · 部分解决 · 可挽回 a 0.25 / b 0.25）

- 位置：导入的 Input format、Theme source、Import candidate、Version、Name、配置字段名、Changes (6)、表头、Import notes；导出的 Output、CSS profile、Color notation、Exported code、Required source revision。
- 现状 → 期望：13px/400/#475467 → G3 label 13px/600。
- 修法：数据工具对话框的 label、`dt`、`th`、节标题统一 `font-weight:600`（并确认 600 字面已加载，见 UI-09）。
- 来源：a UI-26；b UI-26。

#### UI-126 · [P2] 切换 Output 时对话框重新垂直居中，选择器跳动 12–109px（U3 · 布局 · 可挽回 a 0.15）

- 位置：U3-export-json/v1/css/rust 对话框上沿 179/70/107/95，Output 选择器 241/132/169/157；导入错误态与审阅态上沿 148/17。
- 现状 → 期望：X6 对话框宽高为视口减 2rem、内部纵向滚动，切换内容时头部与已操作控件不动。
- 修法：导入导出对话框 `height: calc(100dvh - 2rem)` 正文内滚，或顶部锚定（`align-items:flex-start; margin-top:1rem`）。
- 来源：a 新-9（b 在建议 8 提出同一现象，未计分）。

#### UI-127 · [P2] 390 卡片模式下分组行只铺约 105px 并折行（U3 · 布局 · 可挽回 a 0.1 / b 0.15）

- 位置：U3-import-review-end-390-light「Identity (2)」「Light · base (1)」；U3-import-review-bottom-390-light「Dark · / overrides (1)」折两行。
- 修法：≤560px 时分组行 `display:block; width:100%`（Grid 用 `grid-column:1/-1`）。
- 来源：a 新-2；b 新-2。

#### UI-128 · [P2] 配置只列导入值，没有与当前主题的对比（U3 · 交互 · 可挽回 b 0.15）

- 位置：U3-import-review-1440-light y=656–791、U3-import-review-390-light 六个配置字段只有一个值。
- 现状 → 期望：X1 导入先呈现候选差异——版本、名称、配置……
- 修法：每个字段显示「当前 → 导入」，变化的加「Changed」，配置改动并入 Changes 计数。
- 来源：b 新-3。

#### UI-40 · [P3] Rust/CSS 说明里的标识符只有部分等宽，`16.0` 与代码 `16.` 不一致（U3 · 视觉、细节 · 部分解决 · 可挽回 a 0.15 / b 0.05）

- 位置：U3-export-rust-1440-light 第 4 行调用表达式，第 2 行 theme.json、theme.rs、AppHandle；U3-export-css 说明行 data-theme=dark、.dark。
- 来源：a UI-40；b UI-40。

#### UI-42 · [P3] CSS 注释中字体路径仍有 `makepad/widgets/…` 与 `makepad_widgets/…` 两种拼法（U3 · 细节 · 部分解决 · 可挽回 b 0.05）

- 位置：U3-export-css-1440-light y=496 与 y=621、663。
- 来源：b UI-42（部分解决）；a 判已解决。取部分解决。

#### UI-25 · [P3] 「Import candidate」节里 Version/Name 行内排、配置字段上下排，两种排法（U3 · 布局 · 部分解决 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-1440-light y 598–625 与 y 656–791。
- 来源：a UI-25 余项；b 新-8（同一处，b 判 UI-25 本身已解决）。取部分解决。

#### UI-44 · [P3] 损失清单用 palette 而基础配置用 color，引号用法不统一（U3 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U3-export-v1-1440-light 损失框「Brand palette “Pink”」「Scaling intent 100%」。
- 来源：a UI-44 余项（部分解决）；b 判已解决。取部分解决。

#### UI-129 · [P3] 「Import notes」框内多一条横线（U3 · 视觉 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440-light 框边 y=728、框内 y=741；390 同。
- 来源：a 新-3；b 新-5。

#### UI-130 · [P3] 导出对话框按钮行下方留白大于 1.5rem（U3 · 布局 · 可挽回 a 0.05）

- 位置：U3-export-json-390-light 按钮底到下边框 51px；1440 为 32px。
- 来源：a 新-8。

#### UI-131 · [P3] 390 下「256 KiB」被拆到两行（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-4。

#### UI-132 · [P3] v1 损失清单里的 Auto 没写出实际配对的灰阶（U3 · 细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-export-v1-1440-light 第 2 条（R1 写过「resolved mauve」）。
- 来源：a 新-5；b 新-9。

#### UI-133 · [P3] 备注与差异表对 token 的命名和字体不一致，`name` 行的作者文字用了等宽（U3 · 视觉、细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440/390 备注「base.light.font-serif」（比例字）对表格「Light · base」+ 等宽 `base.primary`；差异表 `name` 行「Rustify Light」「Imported legacy 旧主题」为等宽。
- 来源：a 新-6；b 新-6（同指备注 token 路径与表格写法不一致）。

#### UI-134 · [P3] 差异表首列表头「Mode / token」已不准确（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-7。

#### UI-135 · [P3] 吸顶表头没有底边，滚动时下方行像叠字（U3 · 布局 · 可挽回 b 0.05）

- 位置：U3-import-review-bottom-1440-light y≈340。
- 来源：b 新-4。

#### UI-136 · [P3] 改动类型「Changed」与 label 同样式，390 下像无值的 label（U3 · 视觉 · 可挽回 b 0.05）

- 来源：b 新-7。

#### UI-137 · [P3] 22 字节显示为「0.02 / 256 KiB」（U3 · 细节 · 可挽回 b 0.03）

- 位置：U3-import-error-1440-light y=611。
- 来源：b 新-10。

#### UI-138 · [P3] 「Required source revision」行没有对齐表单列（U3 · 布局 · 可挽回 b 0.03）

- 位置：U3-export-rust-1440-light y=215，修订号 x=394 对 Output x=341。
- 来源：b 新-11。

### U4

#### UI-139 · [P2] 状态 callout 与决策 callout 叠放、内容重复，状态区与决策区的顺序随状态颠倒；冲突面板超出可视高度（U4 · 布局、视觉、交互 · 可挽回 a 0.35 / b 0.15）

- 位置：U4-upgrade-pending-1440-light y316–439 与 y452–778 两只琥珀 callout 都说「review the complete library」，首屏没有编辑控件；U4-upgrade-cancelled 同样重复「Memory only」「reopening restores」；U4-conflict-after-1440-light-a2 状态 callout 排在冲突 callout 之后；U4-conflict-detail-1440-light-a3 冲突面板约 620px，超出粘性头下约 586px 可视高度。
- 现状 → 期望：S1 每个异常状态一处常驻反馈并紧跟继续动作；S2、S5。
- 修法：存储区固定 slot：①状态与决策 callout（每状态一张）→ ②命名动作反馈 → ③操作计数 → ④备份；待确认、取消、冲突各合并为一张 callout；冲突面板压到 1440×900 可同屏看到决策与下载。
- 来源：a 新-1；b UI-50 中的叠放与重复部分、b 新-2（顺序与面板高度）。

#### UI-140 · [P2] 命名动作反馈是裸红字、位置不固定；被流程阻止的保存也用错误红，升级成功后仍残留且无内联动作，时态不对（U4 · 视觉、交互、布局 · 可挽回 a 0.25 / b 0.35）

- 位置：U4-upgrade-cancelled y453–486、U4-upgraded y443–476（紧贴绿色成功 callout）、U4-quota-failure y537–570、U4-no-locks y485–518。
- 现状 → 期望：S4 命名动作单独反馈且结果真实；S5 未处理的命名动作保持可见；S1、S6。
- 修法：固定位置的 inline alert；被阻止用警告样式、真实写失败用错误样式；附「Save “…” now」「Dismiss」；升级成功后降为中性提示；无锁文案改现在时。
- 来源：a 新-2；b 新-1。

#### UI-50 · [P2] callout 标题与对话框分节标题弱于正文，两类 callout 正文字号不一（U4 · 视觉 · 部分解决 · 可挽回 a 0.2 / b 0.3 中的标题部分）

- 位置：「Review the library upgrade」「Memory only · upgrade cancelled」「Choose which version to keep」为 13px 常规 #475467，正文 14px #1D2939；状态 callout 正文 12px；对话框「Saved themes retained (2)」等分节标题同为 13px 常规。
- 修法：callout 标题 13px/600、按变体着色；正文统一字号与 #475467 或 #1D2939；对话框分节标题 13px/600 #1D2939。
- 来源：a UI-50；b UI-50（叠放部分归 UI-139）、b 新-3 的分节标题部分。

#### UI-51 · [P2] 推荐动作与放弃动作同权；编辑列有多套描边按钮（U4 · 视觉、交互 · 部分解决 · 可挽回 a 0.2 / b 0.25）

- 位置：U4-upgrade-pending「Review upgrade / Download original data / Cancel upgrade」同为蓝边灰字；壳层 Import/Export 为蓝边蓝字；空闲 Retry/Cancel 为 #858F9E/#667085。
- 修法：每张 callout 一个实心推荐动作，其余统一壳层描边，放弃动作放末尾改文字按钮。
- 来源：a UI-51；b UI-51。

#### UI-141 · [P2] 冲突的三个决策没有说明后果（U4 · 交互 · 可挽回 a 0.2）

- 位置：U4-conflict-1440-light「Load other version / Keep current draft / Save current as a copy」。
- 修法：每个按钮下加一行后果说明，或改单选组加 Apply（S5）。
- 来源：a 新-3。

#### UI-142 · [P3] 升级 Review 对话框栅格不齐：值列错位 25px、主题表无表头、双细线（U4 · 布局、细节 · 可挽回 a 0.15 / b 0.15）

- 位置：U4-upgrade-review-1440-light 值列 x≈360 对 x≈333；主题表三列无表头；y≈350 与 y≈363 两条线。
- 来源：a 新-5；b 新-3（分节标题部分并入 UI-50）。

#### UI-143 · [P3] 无锁态：标题写「saving paused」，主按钮仍实心，正文是行话、缺句号（U4 · 交互、细节 · 可挽回 a 0.15 / b 0.1）

- 位置：U4-no-locks-initial-1440-light y369–413；页头「Save locally」实心；U4-no-locks「not saved earlier」。
- 来源：a 新-7；b 新-4。UI-61 的句号问题同在此处。

#### UI-144 · [P3] 文案细节：修订号无标签、「Saved themes 0 → 0」、行话与含混措辞（U4 · 细节 · 可挽回 a 0.15 / b 0.05）

- 位置：U4-conflict「This tab 2 · Rustify Light」；U4-upgraded「the original download remains available」；对话框「Ready for explicit upgrade confirmation.」；U4-conflict-raw「Download displayed original」；「Secure Web Locks unavailable」。
- 来源：a 新-10（单复数部分归 UI-56）；b 新-7（修订号标签）。

#### UI-145 · [P3] 冲突差异块纵向松散，组头配色与琥珀底混搭（U4 · 布局、视觉 · 可挽回 a 0.1）

- 位置：U4-conflict-1440-light「overrides.primary」y577 至值 y666 各占一行；组头 #F9FAFB 在 #FFFAEB 内。
- 来源：a 新-6；b 新-2 中「差异拆成四行竖排」的部分。

#### UI-146 · [P3] 冲突原文 textarea 无边框、透明底，截断处无滚动提示（U4 · 视觉 · 可挽回 a 0.05 / b 0.05）

- 位置：U4-conflict-raw-1440-light y355–515。
- 来源：a 新-11；b 新-6。

#### UI-46 · [P3] 成功态容器不着色；同一琥珀底上两种标题色（U4 · 视觉 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgraded 成功 callout 为 #F9FAFB/#D0D5DD，只有标题 #067647；「Memory only · saving paused」标题 #93370D，其他琥珀 callout 标题 #475467。
- 来源：b UI-46（部分解决）；a 判已解决。取部分解决。

#### UI-47 · [P3] 禁用或空闲的存储操作按钮用非 token 色 #667085/#858F9E，且与 Undo 的禁用样式并存（U4 · 视觉 · 部分解决 · 可挽回 a 0.1 / b 0.05）

- 来源：a UI-47；b UI-47。

#### UI-55 · [P3] 取消后 callout 第一段沿用待确认时的文案（U4 · 交互、细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U4-upgrade-cancelled y553–637 与 U4-upgrade-pending y505–589 逐字相同。
- 来源：a UI-55（部分解决）；b 判已解决。取部分解决。

#### UI-56 · [P3] 操作列表：「1 recent operations」单复数、「edit 2」孤字、失败项与成功项同色、说明段答非所问（U4 · 细节、视觉 · 部分解决 · 可挽回 a 0.1 / b 0.1）

- 位置：U4-upgrade-cancelled、U4-no-locks「0 pending · 1 recent operations」；U4-quota-operations y750–768、y612–680。
- 来源：b UI-56（部分解决）；a 新-9 与 a 新-10 的单复数部分（同一操作列表）。a 判 UI-56 已解决，但另列同一问题为新差距。取部分解决。

#### UI-57 · [P3] 升级成功后「Download original before upgrade」在成功 callout 之外，且与其他状态的「Download original data」不同名（U4 · 布局、细节 · 仍存在 · 可挽回 a 0.1 / b 0.1）

- 位置：U4-upgraded y490–517。
- 来源：a UI-57（仍存在）；b UI-57（部分解决）。取仍存在。

#### UI-61 · [P3] 句末标点与行话（U4 · 细节 · 部分解决 · 可挽回 a 0.05 / b 0.05）

- 位置：U4-no-locks-initial 正文缺句号；「Retry explicitly approves retained actions…」。
- 来源：a UI-61；b UI-61。

#### UI-62 · [P3] 没有待处理项时 Retry / Cancel / Save current draft 看起来可点（U4 · 交互、视觉 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgrade-pending y≈830–890、U4-no-locks y≈571–597。
- 来源：b UI-62（部分解决）；a 判已解决（视觉）并注明是否真正禁用看不出。取部分解决。

### U5

#### UI-147 · [P2] 拾取信息面板在文档流里：开启时预览下推 175px，拾取较低部件时面板不在视野（U5 · 交互、布局 · 可挽回 b 0.3）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light「DOM」标签 y=244→419；U5-inspector-undeclared-hover/-undeclared 看不到面板；U5-inspector-undeclared-panel-1440-light 需滚回顶部才看到「Not annotated · …」。
- 现状 → 期望：P2 信息面板带 token 按钮，从实际部件定位角色与来源。
- 修法：面板改为预览滚动容器内的 sticky 层或底部 overlay，不占文档流；或把 token 按钮做成锚在拾取标签旁的小浮层。
- 来源：b 新-1（a 在建议 5 提出下推 175px，未计分）。

#### UI-148 · [P2] 拾取矩形在蓝色部件上看不出（U5 · 视觉 · 可挽回 a 0.1）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light「Create profile」：2px #1570EF 轮廓贴着同色按钮。
- 修法：保持固定蓝色，加 `outline-offset: 2px` 与内侧 2px 白色光环（P2）。
- 来源：a 新-4（b 在建议 8 提出同一点，未计分）。

#### UI-70 · [P3] 拾取后编辑区滚动落点被粘性页头遮住，来源标题不在视野、第一行切半（U5 · 交互、布局、细节 · 部分解决 · 可挽回 a 0.15 / b 0.2）

- 位置：U5-inspector-picked-1440-light 左栏 y≈300–330。
- 修法：滚动目标改为来源块标题；滚动容器 `scroll-padding-top` = 页头高度 + 16px。
- 来源：a UI-70；b UI-70。

#### UI-149 · [P3] 拾取标签截掉 token 列表；较高部件的标签落在视口外（U5 · 细节、交互 · 可挽回 a 0.05 / b 0.15）

- 位置：U5-inspector-1440-light / -picked「primary, primary-f…」；U5-inspector-card-1440-light 卡片底边在视口外。
- 来源：a 新-1；b 新-2。

#### UI-150 · [P3] DOM + GPU 时 DOM 列仍为两列卡片，标题与标签被挤成两行（U5 · 布局 · 可挽回 a 0.1）

- 位置：U5-gpu-compare-1440-light DOM 列 526px，卡片各约 226px。
- 修法：预览 scope 根 `container-type: inline-size`，用容器查询在窄容器下单列（P5 自动宽度跟随可用空间）。
- 来源：a 新-3（b 在建议 4 提出同一点，未计分）。

#### UI-151 · [P3] 未标注态标签仍用选中蓝；面板为空 token 列表留一行空位（U5 · 视觉、细节 · 可挽回 b 0.1）

- 位置：U5-inspector-undeclared-1440-light、U5-inspector-undeclared-panel-1440-light。
- 来源：b 新-3（a 在建议 3 提出标签配色，未计分）。

#### UI-76 · [P3] DOM 与 GPU 两框内的首行内缩相差约 8px（U5 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 (370, 293)，GPU 首行 (908, 285)。
- 来源：a UI-76（部分解决）；b 判已解决（两框上沿对齐）。取部分解决。

### U6

#### UI-81 · [P3] classic 渐变幅度只有目标约 1/3，主体与 solid 几乎同色（U6 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U6-fixture-1280-dark/-light classic 行，以 classic 4 的 x=500 列为准；对照 radix-sink-button classic 4 的 x=420 列。
- 现状 → 期望：方向、顶部 1px 高光、底部 1px 暗线、无外投影已对；深色下内容区相对 solid 仅 −1…+6，目标为 −5…+17。
- 修法：只调 classic 背景渐变层的亮度差约为现在 3 倍，其余保持。
- 来源：a UI-81。

### U7

#### UI-152 · [P1] Text 的四档字重渲染完全相同（U7 · 视觉 · 可挽回 a 0.5）

- 位置：U7-type-text-matrix-1280-light y=537–666 的 weight=light/regular/medium/bold 四行；390 同。
- 现状 → 期望：四行墨量 0.8541/0.8541/0.8541/0.8536 → L2 字重 light/regular/medium/bold 依次加重（比较口径「字重」）。
- 修法：先查四行 computed `font-weight`；数值相同则补齐字重 class 并 `mbx xtask css`；数值不同但渲染相同则补齐 IBM Plex Sans 300/400/500/700 的 `@font-face`（public 静态 woff2）。修后 UI-88、UI-89 应同时恢复。
- 来源：a 新-1。

#### UI-88 · [P1] 〔待裁定〕Heading 不是粗体（U7 · 视觉 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-type-heading-matrix-1280/390、U7-scale110/95-heading，夹具「Native structure, flexible composition」。
- 现状 → 期望：60px 下 Heading 笔画 7px、Text 6px；目标 Heading 9px、Text 5px（L1 正确语义外观）。
- 修法：Heading 默认 bold（700）并确认 700 字面可加载，验收为 60px 下 Heading 笔画约为 Text 的 1.5 倍以上。
- 来源：a UI-88（仍存在）。

#### UI-89 · [P1] 〔待裁定〕Strong 没有加粗（U7 · 视觉 · 仍存在 · 可挽回 a 0.2）

- 位置：U7-type-code-matrix-1280「strong importance」墨量 0.8595 对相邻 0.8577；夹具「Strong importance」。
- 来源：a UI-89（仍存在）。

#### UI-91 · [P2] 非法属性后看不到属性定位诊断（U7 · 交互 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-invalid-1280-light 与默认态逐像素比对，只有该样例重新排版；U7-invalid-detail-1280-light 视口里没有诊断。
- 现状 → 期望：L6 非法值给出属性定位诊断，组件默认保持可用。
- 修法：样例下方渲染 `role="status"` 诊断行（属性、非法值、回退默认值），Restore 后消失。
- 来源：a UI-91。

#### UI-153 · [P2] 非法属性样例默认态塌成一列一个词，上方有孤立的 `px`（U7 · 布局、细节 · 可挽回 a 0.25）

- 位置：U7-fixture-1280-light 约 y=1395–1590；U7-fixture-390-light 约 y=1940–2120；U7-invalid-* 中「px」仍在。
- 修法：默认态给合法值并标注（如 `width=240px`），补全或删除「px」。
- 来源：a 新-2。

#### UI-92 · [P2] Reset 只覆盖 button、h2、ul 三种元素（U7 · 布局、交互 · 部分解决 · 可挽回 a 0.2）

- 位置：U7-layout-reset-comparison-1280/390；对照 radix-reset 约 60 种原生元素。
- 修法：沿用「原生 / Reset」双行，至少补 a、strong、em、code、kbd、mark、sub、sup、input、select、textarea、fieldset、ol、dl、table、pre、blockquote、img。
- 来源：a UI-92 余项。

#### UI-95 · [P3] 布局原语区仍有样例没有 prop 标注（U7 · 细节 · 部分解决 · 可挽回 a 0.15）

- 位置：U7-fixture-1280 约 y=120–1380；Section 只有一条无标注灰带。
- 来源：a UI-95 余项。

#### UI-86 · [P3] Heading wrap 行没有收窄宽度，1–3 档多行行高无法比对（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-type-heading-matrix-1280 size=7·wrap 至 size=1·wrap 只有一行；390 下 3–1 档一行。
- 来源：a UI-86 余项（主体已补齐，余项按 P3 计）。

#### UI-87 · [P3] Inset 的 left、right 贴边没有干净的展示（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-layout-inset-demonstration 只有 top/bottom；U7-fixture-1280 约 y=1015–1320 的 8 个 side 样例都带 `margin=3px`。
- 来源：a UI-87 余项。

#### UI-97 · [P3] 〔待裁定〕夹具根顶边混入外来灰线（U7 · 细节 · 仍存在 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light 第 0 行 x=138–288、295–374（rgb 118–161）；390 图没有。
- 来源：a UI-97（仍存在）。

#### UI-154 · [P3] wrap=pretty 与 wrap=wrap 折行完全相同（U7 · 交互 · 可挽回 a 0.1）

- 位置：U7-type-wrap-matrix-1280/390。
- 修法：换一段在当前卡宽下 wrap 会留孤词的文案，或收窄卡片。
- 来源：a 新-5。

#### UI-155 · [P3] 390 下 Kbd size 9 行换行后键帽上下相贴（U7 · 布局 · 可挽回 a 0.05）

- 位置：U7-type-kbd-matrix-390-light 约 y=172–345。
- 来源：a 新-4。

## 建议（不扣分）

摘自各评审原文的建议节，详见原文。

- U1：
  - 存储状态块压成工具栏状态胶囊，让基础配置早点出现在首屏（a、b）。
  - 品牌色、中性色选项加色块（b）。
  - 加跳转链接或 landmark，缩短到预览工具栏的 Tab 距离（b）。
  - 200% 导出对话框头部吸顶（b）。
  - 应用名降为 14px/600，让主题名成为唯一 18px 标题（a）。
  - 搜索框与相邻选择器边框统一（a）。
- U2：
  - 来源块同时给出另一模式的状态，并附分组级解除入口。
  - primary 为中性灰时拾色平面不要显示饱和蓝。
  - 「Overridden only」写出分模式计数。
- U3：
  - JSON 导出按 2 空格缩进；代码框用 `white-space: pre` 加横向滚动（a、b）。
  - Rust 页底部「Download」改名「Download theme.rs」（a）。
  - 0.2.1 拒收 v2 单独成行警示（a）。
  - 差异表颜色值旁加色块（a、b）。
  - Apply 吸底（b）。
  - Cancel 改描边次级按钮（a、b）。
  - 损失框与备注框区分（b）。
  - 补普通 Cargo 构建命令（b）。
- U4：
  - 冲突差异加色块（a、b）。
  - 对话框 × 与「Cancel upgrade」区分（a）。
  - 「Preview mode」移到偏好区（a）。
  - 成功 callout 加 Dismiss（a）。
  - 主题表加列头（b）。
  - 无锁态「Save locally」标注 memory only（b）。
- U5：
  - 工具栏控件高度统一（a、b）。
  - 简单对话框限宽（a、b）。
  - 菜单与触发器留 4px 并加层级（b）。
  - 局部边界标题升级、外观选择器限宽（a、b）。
  - GPU 色样标签不在连字符处断开（a、b）。
  - ✓ IconButton 换不像复选框的图标（a、b）。
  - Host control 加说明（a）。
  - 深色局部对话框遮罩用根主题或中性深色（b）。
  - 面板补色阶信息（b）。
- U6：
  - 夹具补 high-contrast、gray、gray + high-contrast、disabled 列，IconButton size × variant 矩阵，Link 1–9 字级与 underline 四模式。
  - 补「文字 + 图标」Button。
  - 「Caller utilities」示例改为可读配色。
  - 拍出 Unavailable 被激活后的说明性反馈。
  - Link 悬停帧悬停 auto/hover 下划线链接并标注属性。
- U7：
  - Heading/Text/Blockquote 补色族行与 high-contrast 行。
  - Em 与 Quote 字族一致。
  - Kbd 补多字符键。
  - 顶行 Kbd 间距加大。
  - Container 加嵌套样例。
  - 说明 `margin=3px` 样例是有意的。
  - 「Your name」标签与输入框留间距。
  - Code 表格行高收紧。
