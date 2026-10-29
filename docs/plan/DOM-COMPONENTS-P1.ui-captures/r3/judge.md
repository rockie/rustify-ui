# DOM-COMPONENTS-P1 UI 验收 R3 · 评审结论（合并）

本文件合并本轮 10 份评审原文。原文逐字保存在同目录：

- 首批（每单元一位）：[judge-U1-a.md](judge-U1-a.md)、[judge-U2-a.md](judge-U2-a.md)、[judge-U3-a.md](judge-U3-a.md)、[judge-U4-a.md](judge-U4-a.md)、[judge-U5-a.md](judge-U5-a.md)、[judge-U6-a.md](judge-U6-a.md)、[judge-U7-a.md](judge-U7-a.md)。
- [judge-U4-a-addendum.md](judge-U4-a-addendum.md)：U4 首批评审的复核表把 UI-51 判为部分解决，但差距清单漏了这一条。主 agent 请同一位评审补齐，补充原文另存，首份原文不改。
- 补采后重评（全新评审，同一批材料加补采帧）：[judge-U2-b.md](judge-U2-b.md)、[judge-U3-b.md](judge-U3-b.md)、[judge-U4-b.md](judge-U4-b.md)。

合并规则：

- 每个单元取总分较低那位评审的一整组四维分与理由，不逐维拼接。
- 差距取全部评审的并集。判为同一问题时合并到一个正式编号，「来源」列出各评审的临时编号或旧编号，并写明判为同一问题的依据。合并条目的严重度取各评审中最高的一级，可挽回分按评审分别列出（a、b 指上面的原文）。
- 同一条旧差距结论不同时，按 回归 > 仍存在 > 部分解决 > 待核实 > 已解决 > 不再适用 取最靠前的一项。跨单元的旧差距（如 UI-106 涉及 U1、U2）由各单元评审分别复核，也按这一顺序取舍。
- 新差距从 UI-157 起编号，不复用旧编号。
- 每条差距的完整取样数值、坐标和修法见对应原文，下文只摘要点。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.6 | 2.8 | 2.7 | 0.6 | **8.7** | a | — | 7.8 |
| U2 Studio 精调与来源解释 | 2.7 | 2.65 | 2.75 | 0.6 | **8.7** | b | a 8.85 | 7.75 |
| U3 Studio 导入与导出 | 2.85 | 2.85 | 2.85 | 0.5 | **9.05** | a | b 9.3 | 8.35 |
| U4 Studio 本地库迁移与存储反馈 | 2.75 | 2.7 | 2.5 | 0.65 | **8.6** | a | b 8.7 | 7.55 |
| U5 Studio 预览 | 2.9 | 2.95 | 2.8 | 0.8 | **9.45** | a | — | 9.1 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.9 | 3.0 | 1.0 | **9.9** | a | — | 9.7 |
| U7 布局与排版 | 2.6 | 3.0 | 3.0 | 0.7 | **9.3** | a | — | 7.1 |

### 各维度理由（取自采用的那位评审）

**U1（a）**

- 布局 2.6：顶栏横跨两栏，各视口都有主题名、保存状态、预览模式和 Undo/Redo/Save/Import/Export；1440 与 768 的内边距、768 的 DOM/GPU 上下堆叠都对，390 两层固定区之间不再透出内容。扣分原因：200% 下渲染器分段控件右缘被视口截掉；导出对话框的 Output 值被挤成「Theme…」；顶栏在 768 多占一行、在 200% 下固定占视口 39%；状态胶囊与解除按钮之间 0px。
- 视觉 2.8：分隔、次级文字、搜索框边框、选中三色、错误色都对上 token；label 和选中标签页为 600；状态胶囊改为中性；焦点环统一为一个 token。扣分原因：200% 导出对话框仍是旧的蓝边蓝字次级按钮，选择器边框是 #858F9E；「Light N · Dark N」计数仍为 #667085；分段控件有两种接缝画法。
- 交互 2.7：键盘依次到达预设搜索、品牌色选择器、展开的列表、Dark、Colors 标签页，焦点清楚；保存胶囊按实际结果显示；页头写明比较基准；390 预览页能切模式、保存、导出；200% 下关键动作都可见。扣分原因：Dark 的焦点环压进 Light 段约 5px；200% 下看不出当前导出格式；200% 下渲染器控件被截。
- 细节 0.6：编辑列「1 text pairs」复数错误；中文「已保存到本机 / 已保存到此设备」两种译法；预览标题旁读成「Rustify Light Light preview」；顶栏内边距与两栏差 4px、主题名与说明不同基线；颜色搜索框 36px 与 Clear 32px 并排。

**U2（b）**

- 布局 2.7：来源块和拾色区插在所选格所在行的正下方，1440 下「Clear primary · Light only」在首屏内；常驻壳层收成 57px 顶栏；390 下吸顶区不再漏出内容。扣分原因：每个被覆盖格的状态胶囊折成两行椭圆；配置组状态胶囊与解除按钮之间只有 0–1px；390 下所选格被吸顶区盖住；诊断区有 44px 空白。
- 视觉 2.65：搜索框边框、helper、选中态、分体控件两段高亮、错误边框、标签页下划线都对上 G3。扣分原因：小节标题比字段 label 更浅而字号字重相同，层级倒置；诊断区外层 summary 常规、内层 600；同一种状态用了胶囊和 4px 矩形两种形状；字体 select 的值是 600；计数文字仍是 #667085。
- 交互 2.75：选中格后显示来源（显式覆盖、配置生成两类）；单项解除写明 Light only 且解除后禁用；分组解除写明 both modes；输入 `abc` 保留草稿并贴字段报错；低对比主题保留作者色并有可定位诊断。扣分原因：所选格无可解除覆盖时 ↺ 仍呈启用的蓝色；分组解除只给计数、不列出 token；「Restore document」没有写范围；网格 ↺ 仍只有图标。
- 细节 0.6：来源块键值结构不一致、「Light:」用了等宽；诊断条末行与外框形成双线；搜索框 36px 与 Clear 32px 不等高；嵌套 summary 重复同一计数；胶囊字号约 11px。

**U3（a）**

- 布局 2.85：差异表只剩对话框本体一层滚动、一帧看全；对话框顶部锚定，切换 Output 时选择器不动；390 分组行铺满；修订号与表单列对齐。扣分原因：导出对话框按钮行到下边框 32px，导入为 24px；Changed 徽标比列首文字右移 3px。
- 视觉 2.85：label、表头、节标题都是 13px/600；备注框内多余横线已去掉；Changed 改为带边框徽标；标识符、调用表达式、选择器都换成等宽；各 token 与 G3 一致。扣分原因：v1 损失清单和 Import notes 正文由 14px 降到 13px，不在 G3 字级里；备注里的字体族值是比例字，390 下从连字符处断行。
- 交互 2.85：合法 v1 审阅、未知版本错误、v2/v1/CSS/Rust 导出、复制成功都走通。扣分原因：配置对照没有注明哪边是当前、哪边是导入；Version 行的「→」与配置行含义不同；Name 行只列导入值。
- 细节 0.5：字节计数无千分位且与「256 KiB」单位不一；Rust 说明「theme.json · theme.rs」有歧义、「Keep AppHandle.」丢了「alive」；备注与差异表的 token 命名方式不一；CSS 提示写 `data-theme=dark` 而代码是 `[data-theme="dark"]`；徽标偏移 3px；「ui-」处断行。

**U4（a）**

- 布局 2.75：每个状态只有一张 callout，存储区顺序在六种状态下固定；冲突面板在 1440×900 下整块可见；升级 Review 对话框值列对齐、主题表有表头；页头吸顶并带保存状态胶囊。扣分原因：配额态「Technical details」移到了红色 callout 之外（比 R2 退步）；冲突键值栅格值列只有 128px，修订行折出孤词；对话框里仍有一对双细线；差异块一处改动占四行。
- 视觉 2.7：callout 标题统一为 13px/600 并按变体着色；成功态有绿色底和边框；禁用样式统一；原文 textarea 有边框和底色。扣分原因：callout 正文在 12px #475467 与 13px #1D2939 之间切换，同一张冲突 callout 内也混用；对话框次级按钮是蓝边蓝字，与壳层灰描边两套样式并存；文字按钮标签比正文缩进 11px。
- 交互 2.5：七个状态都能走通，各有常驻反馈和继续动作；冲突的三个决策写明后果；升级成功后命名动作降为中性提示并附 Save now、Dismiss；无锁态 Save locally 降为描边。扣分原因：取消后 callout 让用户「Download your current theme」，callout 里却只有「Download original data」；暂停态页头 Save locally 仍是实心主按钮；配额态整体说法与操作列表矛盾；配额命名卡没有内联重试。
- 细节 0.65：取消态 callout 两段互相重复；「Saved theme count 0 → 0」；成功态「before upgrade」时态不对；胶囊与 callout 用词不一致，「upgrade」「migration」混用；操作说明段讲本状态没有的「Load other version」；双细线与缩进问题。

**U5（a）**

- 布局 2.9：拾取面板移出文档流，开启拾取不再下推预览；DOM + GPU 时 DOM 列单列；两框首行上沿对齐；390 对话框两侧各留 16px。扣分原因：拾取浮动面板固定压在预览右下角，挡住被拾取的「Create · 0」和右卡内容；拾取标签盖住相邻复选框与文字；两框首行左内缩差 8px。
- 视觉 2.95：分段控件三色正确，#8AB7F7 内框和段间双线消失；拾取矩形在蓝色按钮上清晰可辨；未标注标签改用 #475467；面板浅蓝；编辑壳在深色预览下保持中性浅色。扣分原因：未标注目标被面板遮住时，矩形描边和标签画在面板之上，与面板文字交叠。
- 交互 2.8：菜单、对话框、局部边界相反外观及其浮层、对比区 Dark、DOM + GPU、拾取后焦点进入编辑器字段、未标注说明都走通。扣分原因：三张未标注帧里被拾取部件大半藏在面板下；拾取 Create profile 时两行标签盖住相邻复选框和「I understand」，并在拾取后一直保留。
- 细节 0.8：标签在连字符处断成「primary-」「foreground」；卡片标签切掉上一行文字的下伸部；预览头部「Rustify Light」与「Light preview」之间丢了分隔符；按下态加粗使相邻控件横移 1–2px；两框首行左内缩差 8px。

**U6（a）**

- 布局 3.0：四档高度 24/32/40/48，110% 下 26/36/44/52；内边距、IconButton 正方形、ghost/ghost-offset 几何、full 圆角、390 换行都与目标一致。
- 视觉 2.9：各变体选阶与目标对应；classic 主体渐变已达目标约 80%；ghost 与非 ghost 的笔画粗细比与目标一致。扣分原因：classic 上缘高光带与两侧亮边只有目标的 1/3 到 2/3。
- 交互 3.0：13 张 DPR 2 特写与 gold、Unavailable、Link 焦点帧逐张测量，焦点环宽、间隙、色族第 8 阶都符合 B2；Link 聚焦时隐藏下划线；loading 保留宽度与填充、spinner 可辨；110%、full、中文可用。
- 细节 1.0：ThemePanel「Resolved neutral: Slate.」「已解析中性色：蓝灰色。」已与品牌色同一规则，多余的「·」已去掉，逐处比对未见其他差异。

**U7（a）**

- 布局 2.6：Container 四档宽度与三种对齐正确；Heading 九档 wrap 都折成多行，行距与目标逐档一致；Inset top/bottom/x/y 与嵌套正确；Section 断点正确；390 下 Kbd 键帽不再相贴；显式尺寸在缩放下不变。扣分原因：Inset side=left/right 演示里正文压在贴边色块边上；Reset 的 input、textarea 两行脱离值列、占满整行。
- 视觉 3.0：Heading 为粗体；Text 四档字重墨量依次递增；Strong 加粗；字级梯度、Code 四变体、Kbd、Blockquote 都能与目标逐项对应。
- 交互 3.0：110%/95% 只缩放一次；点 Invalid prop 后出现带属性名、收到值和回退值的诊断，样例照常渲染；wrap、nowrap、balance、pretty、truncate 与四档 trim 都有可辨效果。
- 细节 0.7：Reset 区 button 行有一个没有标注的「0」；诊断文案把拒收值写了两遍、属性名带字面反引号；37×19 显式 CSS 样例完全看不见。

## 上一轮差距复核结果

R2 judge.md 列出的 88 条评审来源差距，本轮结果如下（逐条依据见各单元原文的复核表）：

- **仍未关闭（28 条）**：UI-106 仍存在；UI-02、UI-103、UI-104、UI-107、UI-110、UI-17、UI-115、UI-18、UI-22、UI-128、UI-25、UI-130、UI-133、UI-114、UI-139、UI-140、UI-50、UI-51、UI-142、UI-144、UI-145、UI-55、UI-56、UI-76、UI-81、UI-95、UI-87 部分解决。
- **已解决（60 条）**：其余各条。其中 UI-97 按用户裁定后的采集方式复核（U7 整段帧改为留边 8px 的页面裁切），灰像素只在根外留边内，判已解决。
- 结论不同、按顺序取舍的旧差距：
  - UI-106：U1 判部分解决，U2 两位都判仍存在，取仍存在；
  - UI-02：U1 判部分解决，U5 判已解决（焦点部分待核实），取部分解决；
  - UI-110：U1 判部分解决，U5 判已解决，取部分解决；
  - UI-17：U2 a 判部分解决，U2 b 判已解决、另把 390 所选格被遮列为新-4，取部分解决；
  - UI-115：U2 a 判已解决（可见部分），U2 b 判部分解决，取部分解决；
  - UI-128：U3 a 判已解决（主体）、另把缺图例列为新-1，U3 b 判部分解决，取部分解决；
  - UI-25：U3 a 判已解决，U3 b 判部分解决，取部分解决；
  - UI-114：U1 判已解决，U4 a 判已解决，U4 b 判部分解决，取部分解决；
  - UI-139：U4 a 判已解决，U4 b 判部分解决，取部分解决；
  - UI-55、UI-144：U4 a 判部分解决，U4 b 判已解决，取部分解决；
  - UI-102、UI-108、UI-109、UI-12：涉及的各单元、各评审都判已解决。

## 未关闭差距

共 63 条：P2 9 条，P3 54 条。旧差距沿用编号并附本轮状态；新差距已换成正式编号。按单元排列，单元内先 P2 后 P3、再按可挽回分。

### U1

#### UI-157 · [P2] 200% 下渲染器分段控件（DOM / GPU / DOM + GPU）宽于预览列，右缘被视口截掉（U1 · 布局、交互 · 可挽回 a 0.2）

- 位置：U1-zoom200-720-light 预览工具栏第三行（y≈655–716），控件从 x=352 一直到视口边缘 x=719，没有右边框，右圆角被切。R2 同帧完整显示，属回归。
- 现状 → 期望：被截断 → G6 200% 下主路径可达、不依靠横向滚动；评分口径要求无溢出、截断。
- 修法：分段控件 `max-width:100%`，每段 `flex:1 1 auto; min-width:0`、允许换行；或给预览工具栏设 `container-type:inline-size`，用 rem 写容器查询，窄时改三列等分网格，再窄改为带 label 的下拉。确认预览列网格项 `min-width:0`。
- 来源：U1 a 新-1。

#### UI-158 · [P2] 200% 下导出对话框的 Output 选择器被压窄，当前值截成「Theme…」（U1 · 布局、交互、细节 · 可挽回 a 0.2）

- 位置：U1-zoom200-export-720-light「Output」行，选择器约 213px 宽。R2 同帧显示完整「Theme JSON v2」，属回归。
- 现状 → 期望：看不出当前导出格式 → G6 对话框在可用视口内可操作；X6 窄时表单行单列；X2 须区分 v2 与兼容 v1。
- 修法：对话框表单行 `flex-wrap:wrap`，选择器 `flex:1 1 14rem; min-width:14rem`，不够时换到 label 下方占满；或按对话框宽度用容器查询切单列，不只依赖 ≤560px 视口断点。
- 来源：U1 a 新-2。

#### UI-02 · [P3] 分段控件残留：焦点环压到相邻段，两种接缝画法并存（U1 · 交互、视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U1-keyboard-04-1440-light Dark 焦点环左缘 x≈988–990 落进 Light 段；U1-default-1440-light EN/中文、U1-edit-390 / U1-preview-390 Edit/Preview 段间为 #84CAFF + #D0D5DD 两条线。
- 修法：EN/中文、Edit/Preview 改用 Light/Dark 的同一分段组件（相邻段 `margin-inline-start:-1px` 合并边框，选中段 `z-index:1`）；分段内 `:focus-visible` 用内收焦点环（`outline-offset:-3px`），或焦点段 `z-index:2` 并给组容器留 2px 内边距。
- 来源：U1 a UI-02（部分解决）；U5 a 判已解决，焦点部分待核实。

#### UI-103 · [P3] 顶栏在 768 多一行，在 200% 固定占视口 39%（U1 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-default-768 / U1-configured-768 顶栏 y=0–160 三行，第 2、3 行右半空置；U1-zoom200-720 顶栏 y=0–348 共 5 行且固定。
- 修法：Undo/Redo 与 Save/Import/Export 放进同一个可换行的动作行；200% 下把 Undo/Redo 放到 Preview mode 下方右列；顶栏超过视口 30% 时改为不吸顶，或把 Import/Export 收进 More。
- 来源：U1 a UI-103（部分解决；1440 已解决）。

#### UI-104 · [P3] 200% 导出对话框仍用旧的蓝边次级按钮和非 token 边框（U1 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-zoom200-export-end-720-light「Select text」「Download」为 #8AB7F7 边、#1570EF 字；U1-zoom200-export-720-light Output 选择器边框 #858F9E。
- 修法：对话框动作区复用壳层次级按钮（白底、1px #D0D5DD、#475467 字），选择器边框改 #D0D5DD；只有 Copy code 保留实心主按钮。与 UI-51 是同一套样式问题，可一起改。
- 来源：U1 a UI-104（部分解决）。

#### UI-106 · [P3] 状态行与解除按钮之间 0px；两栏标题中线差约 4.5px（U1/U2 · 布局、细节 · 仍存在 · 可挽回 U1 a 0.1、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light 与 U2-group-reset-1440-light，Brand、Neutral、Panel 各组的状态胶囊底边与「Use configuration values · both modes」顶边相接（如 y=488/489）；U1-default-1440-light 应用名与预览标题中线差约 4.5px。
- 修法：配置组容器 `display:grid; gap:.5rem`（或状态行下边距 8px）；预览标题行 `min-height:2rem; align-items:center`，与编辑列首行同高同中线。
- 来源：U1 a UI-106（部分解决）；U2 a、b UI-106（仍存在）。

#### UI-107 · [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一（U1/U2 · 视觉 · 部分解决 · 可挽回 U1 a 0.05、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light、U2-group-reset-1440-light 的「Light N · Dark N」计数与「No overrides」胶囊文字为 #667085；U1-field-tabs-1440-light「Search colors…」占位符 #667085，U1-default-1440-light「Search presets…」约 #8E949C。
- 修法：计数与胶囊文字改用 #475467，#667085 只留给禁用态和占位符；两个搜索框共用一个 `::placeholder` token。
- 来源：U1 a、U2 a、U2 b UI-107（均部分解决）。

#### UI-110 · [P3] 编辑列诊断摘要仍是「1 text pairs」（U1 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U1-keyboard-04-1440-light 编辑列底部 y≈783「Diagnostics · 1 text pairs below 4.5:1」；预览诊断条已改为单数。
- 修法：编辑列摘要与预览诊断条共用同一个复数函数。
- 来源：U1 a UI-110（部分解决）；U5 a 判已解决。

#### UI-159 · [P3] 中文「Saved on this device」有两种译法（U1 · 细节 · 可挽回 a 0.05）

- 位置：U1-zh-top-1440-light 顶栏胶囊「已保存到本机」，同帧编辑列状态块「已保存到此设备」。
- 修法：两处共用同一个文案 key，统一为「已保存到本机」。
- 来源：U1 a 新-3。

#### UI-161 · [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」（U1/U5 · 细节 · 可挽回 U1 a 0.05、U5 a 0.05）

- 位置：U1-default-1440-light 等帧及 U5-dom-1440-light、U5-gpu-compare 右上 x≈1266–1416、y≈88–98；U5-dom-1440-dark 为「Rustify Light　Dark preview」；U5-preview-390-light y≈229。R2 同位置是「Rustify Light · Light preview」。
- 现状 → 期望：主题名与预览模式连读 → C4 显示「预览模式」以免与作者外观混淆。
- 修法：恢复只在同一行时显示的「 · 」分隔（换行时去掉悬挂点），或把模式写成「Preview: Light」或中性胶囊；中文同步。
- 来源：U1 a 新-5、U5 a 新-3。判为同一问题的依据：同一位置、同一文字、同一原因（R2 的分隔点被去掉）。

#### UI-162 · [P3] 顶栏左右内边距与两栏不一致，主题名与说明按中线而非基线对齐（U1 · 布局、细节 · 可挽回 a 0.05）

- 位置：U1-default-1440-light「Export」右缘 x=1419、预览分隔线右端 x=1416；U1-default-768-light 顶栏左 20 / 右 747，编辑与预览为 16 / 751；1440 主题名基线 y≈33、「Matches…」基线 y≈31。
- 修法：顶栏 `padding-inline: 20px 24px`，≤800px 改 16px 16px；主题名组 `align-items:baseline`。
- 来源：U1 a 新-6。

#### UI-160 · [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高（U1/U2 · 布局、细节 · 可挽回 U1 a 0.03、U2 b 0.05）

- 位置：U1-field-tabs-1440-light「Search colors…」y=537/572，「Clear」y=539/570；U2-source-overridden-1440-light 搜索框 y=73–108、「Clear」y=75–106。
- 修法：输入框与按钮同高，统一用壳层控件高度。
- 来源：U1 a 新-4、U2 b 新-9。判为同一问题的依据：同一对控件、同一高度差。

### U2

#### UI-163 · [P2] 1440 网格中「Overridden · Light」胶囊折成两行，文字压到圆角边，与配置组状态标识形状不一致（U2 · 布局、视觉 · 可挽回 a 0.35、b 0.2）

- 位置：U2-source-overridden、-detail、U2-token-released、U2-source-generated 四张 1440 图；如 background 格 x=20–157，胶囊 x≈50–125、y≈239–268，第二行「Light」离左侧弧边约 3px。有胶囊的格高约 80px，无胶囊的 ring 格约 45px。胶囊字号约 11px；同一状态在配置组里是 4px 圆角矩形加前导圆点。
- 现状 → 期望：圆角胶囊内文字折行贴边、行高参差、一个概念两种形状 → F4 分体控件；C3 状态标识完整可读；G2 分组行；G3 caption 12px。
- 修法：A 网格按编辑列宽自动降为单列（`repeat(auto-fill, minmax(13.5rem, 1fr))`），胶囊 `white-space:nowrap`；B 格内复用配置组的状态标识组件（4px 圆角、12px/600、前导 ●），拆成「● Overridden」「· Light」两个不换行片段，放在格内整行。
- 来源：U2 a 新-1、U2 b 新-1。判为同一问题的依据：同一批四张图、同一元素、同一折行现象。a 注明这是 UI-115 加了模式文字之后的副作用。

#### UI-17 · [P3] 选中后的滚动位置没有同时露出所选格与完整来源块（U2 · 布局、交互 · 部分解决 · 可挽回 a 0.15、b 0.05）

- 位置：U2-source-overridden-390-light 吸顶区占 y=0–约 200，所选 primary 格只露出胶囊那一行；U2-source-generated-1440-light ring 来源块下半部与「Clear ring · Light only」在折线以下。
- 修法：token 格 `scroll-margin-top` 设为吸顶区实测高度 + 8px；选中后先保证所选格顶边不高于吸顶区下沿，再让解除按钮 `scrollIntoView({block:'nearest'})`。
- 来源：U2 a UI-17（部分解决）；U2 b 判 UI-17 已解决，另列新-4（390 所选格被吸顶区盖住），判为同一问题：同一帧、同一现象。

#### UI-18 · [P3] 小节标题仍与次级标签或字段标签同样式，或比 label 更弱（U2 · 视觉 · 部分解决 · 可挽回 a 0.1、b 0.1）

- 位置：U2-controls-1440「Typography」与「Choose a bundled font」同为 12px/600 #475467，「Sans font stack」为 #1D2939 13px/600；U2-field-error-1440「Geometry」同上；U2-group-reset-1440「Theme configuration」与「Appearance」「Brand color」同为 13px/600 #1D2939；来源块标题「Fine adjustment source」与键「Effective value」同样式。
- 修法：小节标题统一为一种眉题样式（12px/600 大写、字距 .04em、#475467，上方 24px、下方 12px，上加 #EAECF0 分隔线），或改为 14px/600 #1D2939；字段 label 保持 13px/600 #1D2939；子标签降为 12px/400 #475467。
- 来源：U2 a、b UI-18（均部分解决）。

#### UI-165 · [P3] 来源块的键值结构只做了一半，行标签与块标题同样式（U2 · 布局、视觉、细节 · 可挽回 a 0.1、b 0.1 中的结构部分）

- 位置：U2-source-overridden-1440 y=637–873（-detail 图 y≈389–620 同）：只有「Effective value」一行是键值两列；token 名、「Explicit document override」、「overrides.light.primary」三行没有键名。
- 修法：来源块改为 `<dl>`，`grid-template-columns: 7rem 1fr`，行依次为 Token、Effective value、Source、Path、If cleared；dt 12px #475467，dd 13px #1D2939，路径 12px 等宽。
- 来源：U2 a 新-3、U2 b 新-5 的键值部分（同一区域、同一结构问题）。

#### UI-169 · [P3] 诊断区「Authored colors fit the sRGB preview.」下方有约 44px 空白（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440，该行墨迹 y=119–131，下一条从 y≈175 开始；其余条目间约 16px。
- 修法：内容为空时不渲染该节点，或统一由父级 `gap:16px` 控制、去掉子元素 margin。
- 来源：U2 a 新-7；U2 b 在 UI-22 条下记了同一空白。

#### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（U2 · 交互与状态 · 可挽回 b 0.1）

- 位置：U2-group-reset-1440-light Neutral「Light 15 · Dark 15」+「Use configuration values · both modes」（y≈635–690），Panel「Light 2 · Dark 2」（y≈841）。网格按 Surfaces、Semantic colors、Borders & focus 分组，与配置组不对应。
- 现状 → 期望：F2「解除将影响哪些项」。
- 修法：把计数做成 disclosure，展开后按模式列出 token 名；或点击计数时把网格按该组筛选到「Overridden only」。
- 来源：U2 b 新-8。

#### UI-164 · [P3] 选中格里不可解除的 ↺ 仍显示为选中蓝，看起来可用（U2 · 交互与状态 · 可挽回 a 0.05、b 0.1）

- 位置：U2-source-generated-1440 ring 格 ↺ 为 #175CD3，同屏未覆盖格为 #98A2B3；U2-token-released-1440 primary 已解除，↺ 仍是 #175CD3，而来源块的解除按钮已置灰。
- 修法：选中样式只改外框和底色；↺ 段图标色由自身状态决定，`aria-disabled` 时固定 #98A2B3。
- 来源：U2 a 新-2、U2 b 新-2（同一两帧、同一元素）。

#### UI-22 · [P3] 诊断区子 summary 比父 summary 更重，两组缩进不一（U2 · 视觉、布局 · 部分解决 · 可挽回 a 0.05、b 0.15）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs…」13px/400，内层「Text contrast pairs · 5 below 4.5:1」13px/600；字体诊断三条从 x=20 起且没有自己的标题，对比组从 x=32 起。
- 修法：外层改 13px/600 #1D2939，内层用 #475467；字体三条包进同级的 `<details>`（「Fonts · 3」），与对比组同样缩进。
- 来源：U2 a、b UI-22（均部分解决）。b 的这一条还写了 44px 空白与重复计数，分别归入 UI-169、UI-168。

#### UI-115 · [P3] 网格 ↺ 仍只有图标（U2 · 交互与状态 · 部分解决 · 可挽回 b 0.05）

- 位置：U2-source-overridden-1440-light 各格右侧的 ↺ 段。
- 现状 → 期望：模式已由相邻胶囊写出，但 ↺ 本身没有可见文字或提示 → F2 显示范围；C3 不只靠悬停。
- 修法：给 ↺ 加与来源块按钮同文案的 `aria-label` 和 `title`（如「Clear primary · Light only」）；补拍悬停与键盘聚焦帧。
- 来源：U2 b UI-115（部分解决）；U2 a 判已解决（可见部分，aria-label 截图不可见）。

#### UI-166 · [P3] 字体下拉的显示值是粗体，与其他下拉、输入框不一致（U2 · 视觉 · 可挽回 a 0.05、b 0.05）

- 位置：U2-controls-1440 三处「Custom author stack」为 600，同页与 U2-group-reset 的其他下拉值为 400。
- 修法：去掉字体下拉值的 `font-weight`；要提示自定义栈就在 label 旁加中性「Custom」标识。
- 来源：U2 a 新-4、U2 b 新-3。

#### UI-167 · [P3] 句中把模式名「Light:」也设成了等宽（U2 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U2-source-overridden-1440 y≈800、U2-source-overridden-390 y≈414「Clearing this override reveals Light: #d6409f.」
- 修法：`<code>` 只包住色值。
- 来源：U2 a 新-5、U2 b 新-5 的等宽部分。

#### UI-168 · [P3] 同一对比计数在三处用了三种措辞，父子 summary 重复同一个数字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs below 4.5:1」、内层「Text contrast pairs · 5 below 4.5:1」、预览条「5 text pairs below 4.5:1 · author colors preserved」。
- 修法：外层只写「Diagnostics」或「Diagnostics · 5 issues」，内层只写「Text contrast pairs」，预览条保留一种固定句式。
- 来源：U2 a 新-6；U2 b 在 UI-22 条下记了同一重复计数。

#### UI-170 · [P3] 预览侧诊断条末行分隔线与外框形成双线（U2 · 细节 · 可挽回 b 0.05）

- 位置：U2-group-reset-1440-light 末行分隔线 y=418、外框底边 y=431；U2-source-overridden、U2-low-contrast 同样。
- 修法：`.diag-row:last-child{border-bottom:0}`。
- 来源：U2 b 新-6。

#### UI-171 · [P3] 「Restore document · Graphite」没有写范围（U2 · 交互与状态 · 可挽回 b 0.05）

- 位置：U2-restore-1440-light，按钮在 y≈196–227，附近没有说明文字。
- 现状 → 期望：另两档重置都带范围后缀（「· Light only」「· both modes」），第三档只写了目标名 → F2「恢复预设/文档」须显示范围。
- 修法：按钮下加 12px #475467 说明，例如「Discards configuration changes and all overrides in both modes」；或把范围写进按钮文案。
- 来源：U2 b 新-7（补采帧）。

（U2 另有 UI-106、UI-107、UI-160 三条，见 U1 节。）

### U3

#### UI-128 · [P2] Import candidate 的「→」没有「当前 / 导入」图例；Version 行的「→」含义与配置行不同；Name 行只有导入值（U3 · 交互、细节 · 部分解决 · 可挽回 a 0.2、b 0.15）

- 位置：U3-import-review-1440-light y=598–803；U3-import-review-390-light y=648–781；U3-import-review-mid-390-light y 58–338。
- 现状 → 期望：看不出哪边是当前、哪边是导入，未变化项写成「X → X」 → X1 导入先呈现候选差异（版本、名称、配置）。
- 修法：配置与 Name 行改成与 Changes 表同一套「Field | Current | Imported」列结构；相同的行只显示一个值并标「Unchanged」，不同的行加「Changed」徽标并计入 Changes (N)；Version 行改写为「Source: Theme JSON v1 · Saved as v2」；390 卡片同样用两列小标题。
- 来源：U3 a 判 UI-128 已解决（主体）、另列新-1；U3 b 判 UI-128 部分解决，残留即这一条。判为同一问题的依据：同一区域、同一缺图例现象。按跨轮沿用原编号的规则用 UI-128，严重度取 a 的 P2。

#### UI-130 · [P3] 导出对话框按钮行下方留白 32px，导入对话框为 24px（U3 · 布局 · 部分解决 · 可挽回 a 0.1、b 0.04）

- 位置：U3-export-json-1440、U3-export-rust-1440、U3-export-json-390 留白 32px；U3-import-review-bottom-1440/390、U3-import-error-1440 为 24px。
- 修法：空的复制/下载状态行不占间距，或把状态 live region 移进按钮行作为行尾项。
- 来源：U3 a、b UI-130（均部分解决）。

#### UI-175 · [P3] v1 损失清单与 Import notes 正文降为 13px/400，不在 G3 字级里（U3 · 视觉 · 可挽回 a 0.1、b 0.07）

- 位置：U3-export-v1-1440 y=148–297 行距 19.5px；U3-import-review-bottom-1440 y=771/791 行距 20px；R2 同区域为 14px/21px。
- 修法：损失框与备注框正文用 body 14px/1.5；如果有意做成辅助说明，改用 caption 12px #475467。
- 来源：U3 a 新-4（标为回归）、U3 b 新-3。

#### UI-173 · [P3] 字节计数「3332 / 262144 bytes」没有千分位，与说明里的「256 KiB」单位不一（U3 · 细节 · 可挽回 a 0.1、b 0.08）

- 位置：U3-import-review-1440 y=479、U3-import-error-1440 y=479、U3-import-review-390 y=530；同对话框 y=88 写「up to 256 KiB」。
- 修法：用 `Intl.NumberFormat` 加千分位，上限与说明用同一单位；中文同步。
- 来源：U3 a 新-2、U3 b 新-1。

#### UI-174 · [P3] Rust 接入说明有歧义：「theme.json · theme.rs」读成并列；「Keep AppHandle.」丢了「alive」（U3 · 细节 · 可挽回 a 0.1、b 0.1 中的这一部分）

- 位置：U3-export-rust-1440、U3-export-copied-1440 y=167。
- 修法：改为「Put `theme.json` next to `theme.rs`.」「Keep the returned `AppHandle` alive while the UI is mounted.」；中文同步。
- 来源：U3 a 新-3、U3 b 新-2 的 Rust 部分。

#### UI-177 · [P3] 备注中的字体族值是比例字，390 下「ui-serif」从连字符处断开（U3 · 视觉、细节 · 可挽回 a 0.08、b 0.04）

- 位置：U3-import-review-end-390 y≈699/719、U3-import-review-bottom-390 y≈675/695；1440 y=771/791。
- 修法：requested/resolved 两个值用与路径相同的 inline code 样式并 `white-space:nowrap`。
- 来源：U3 a 新-6、U3 b 新-5；U3 b 在 UI-133 条下也记了作者值为比例字。

#### UI-176 · [P3] 「Changed」徽标与列、基线没对齐，表头下方是双线（U3 · 布局、细节 · 可挽回 a 0.05、b 0.08）

- 位置：U3-import-review-bottom-1440 / -end-1440 表头「Change」x=573，徽标边框 x=576、文字 x=584；徽标基线比同行文字低约 2px；y=301 表头底边与 y=302 分组行上边形成 2px 双线；U3-import-review-mid-390 徽标 x=53，同卡 token 与「Current」x=50。
- 修法：去掉徽标左缩进或让该单元格 padding 与其他 `td` 相同；徽标 `inline-flex` 居中、单元格 `vertical-align:middle`；`thead + tbody > tr:first-child{border-top:0}`。
- 来源：U3 a 新-5、U3 b 新-4（同一徽标；b 补了基线、双线与 390）。

#### UI-133 · [P3] 备注与差异表对 token 的命名方式不一致（U3 · 细节 · 部分解决 · 可挽回 a 0.06、b 0.08）

- 位置：U3-import-review-bottom-1440 y=771「base.light.font-serif」；同帧分组头「Light · base (1)」加行名「base.primary」。
- 修法：两处统一用同一套路径格式：行名只写 token、备注写「Light · base · font-serif」；或都写完整路径。
- 来源：U3 a、b UI-133（均部分解决）。

#### UI-25 · [P3] Import candidate 两组行距不一致（U3 · 布局、细节 · 部分解决 · 可挽回 b 0.06）

- 位置：U3-import-review-1440-light label 大写字母顶边依次为 594、621、654、683、712、741、770、799：Version → Name 27px，配置各行 29px，Name → Appearance 33px。
- 修法：Version、Name 与六个配置字段放进同一个 `dl` grid，只用一个 `row-gap`，去掉两块之间的额外 margin。
- 来源：U3 b UI-25（部分解决）；U3 a 判已解决。

#### UI-178 · [P3] CSS 提示中的「Scoped dark selector: data-theme=dark」不是可用的选择器（U3 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U3-export-css-1440 y=230；同帧代码 y=510 为 `[data-theme="dark"]`。
- 修法：提示改为 `[data-theme="dark"]`。
- 来源：U3 a 新-7、U3 b 新-2 的 CSS 部分。

### U4

#### UI-179 · [P2] 取消后 callout 让用户「Download your current theme」，callout 里却没有这个按钮（U4 · 交互 · 可挽回 a 0.2、b 0.1）

- 位置：U4-upgrade-cancelled-1440-light 琥珀 callout，第 2 段 y≈383–422；按钮只有「Review upgrade again」和「Download original data」。R2 同状态有「Download current theme」，配额态、无锁态 callout 也有。
- 现状 → 期望：用户想保住仅内存编辑，手边唯一的下载按钮会下载旧原始数据 → S1 原始数据与当前主题出口始终可达；S2 取消后常驻仅内存编辑；S6。
- 修法：取消态 callout 加「Download current theme」（复用配额/无锁 callout 的组件），顺序为「Review upgrade again」→「Download current theme」→「Download original data」（或改名「Download pre-upgrade data」）。
- 来源：U4 a 新-1、U4 b 新-3（b 标为相对 R2 回归）。

#### UI-50 · [P2] callout 正文字号与颜色在状态之间、甚至同一张 callout 内来回切换（U4 · 视觉 · 部分解决 · 可挽回 a 0.15、b 0.15）

- 位置：13px #1D2939 见 U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 首段、各命名卡；12px #475467 见 U4-upgraded、U4-quota-failure、U4-no-locks-initial、U4-conflict 后两段。U4-no-locks 中相邻两张琥珀卡片一张 12px 灰字、一张 13px 深字。13px 常规字重不在 G3 字级上。
- 修法：callout 只保留两级文字：状态句用 body（b 建议 14px/1.5，a 建议 13px/1.5）#1D2939，补充说明用 caption 12px #475467；命名卡与状态 callout 共用。
- 来源：U4 a UI-50（部分解决，P3）、U4 b UI-50（部分解决，P2）；严重度取 P2。

#### UI-51 · [P2] 升级 Review 对话框底部放弃动作与次级动作同权，且用蓝边蓝字，与壳层灰描边是两套样式；配额、无锁 callout 没有推荐动作（U4 · 视觉、交互 · 部分解决 · 可挽回 a 0.1、b 0.15）

- 位置：U4-upgrade-review-1440-light 底部 y712–743，「Download original data」「Cancel upgrade」同为 #1570EF 字、#8AB7F7 边；同流程 callout 里前者是灰描边、后者是文字按钮；U4-quota-failure、U4-no-locks callout 只有描边的「Download current theme」。
- 修法：对话框底部与 callout 用同一按钮组件：「Download original data」改壳层描边，「Cancel upgrade」改文字按钮放最后；无锁 callout 把「Download current theme」改为实心；配额 callout 加实心「Retry save」。导入导出对话框若用同一蓝边样式一并改（与 UI-104 同一套样式问题）。
- 来源：U4 a UI-51（部分解决，条目见 judge-U4-a-addendum.md，P3）、U4 b UI-51（部分解决，P2）；严重度取 P2。

#### UI-140 · [P3] 命名动作卡余项：配额卡没有内联重试；升级后卡片的文案与按钮重复（U4 · 交互 · 部分解决 · 可挽回 a 0.1、b 0.1）

- 位置：U4-quota-failure 红卡 y457–565 只有「Dismiss」，「Retry retained saves」在操作区 y617–648；U4-upgraded 中性卡正文写「Use Save locally to try again.」，卡里已有「Save now “Memory only”」。
- 修法：配额卡加「Retry “Retained name”」；升级后卡片删去「Use Save locally to try again.」，按钮改为「Save “Memory only” now」。
- 来源：U4 a、b UI-140（均部分解决）。

#### UI-139 · [P3] 取消 callout 内同一句话写了两遍；冲突面板所在位置与其他状态不同（U4 · 布局、细节 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgrade-cancelled 标题「Memory only · upgrade cancelled」与正文「Upgrade cancelled. Memory-only editing; reopening restores…」「Reopening restores the original draft.」；U4-conflict callout 顶边紧贴 y=244（其他状态这里是分隔线、callout 从 y=261 开始），面板下方另有 y=806 分隔线。
- 修法：取消 callout 正文并为一句；冲突面板放回与其他状态相同的位置，删去面板下方那条分隔线（或所有状态统一用「上线、下线」结构）。
- 来源：U4 b UI-139（部分解决）；U4 a 判已解决。重复句与 UI-55 重叠，修 UI-55 时一并处理。

#### UI-181 · [P3] 配额态页头与 callout 说「草稿留在内存」，操作列表却显示当前草稿已确认保存（U4 · 交互 · 可挽回 a 0.1）

- 位置：U4-quota-failure / U4-quota-operations 页头「Save failed」、callout「Your draft and pending actions stay in memory」、操作列表「#2 · Draft · confirmed (draft revision 2)」。
- 现状 → 期望：失败的是命名保存，界面笼统说草稿没存 → S4 区分先前快照成功与当前仍待保存、命名动作单独反馈。
- 修法：callout 标题改为「Storage full · new saves stay in memory」，正文分开写草稿已存、命名保存失败待重试；页头胶囊按最近一次草稿写入的结果显示。
- 来源：U4 a 新-3。

#### UI-182 · [P3] 「Saving paused」状态下页头「Save locally」仍是实心主按钮，点了只会被拦下（U4 · 交互 · 可挽回 a 0.1、b 0.1）

- 位置：U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 页头「Save locally」（#1570EF 实心），紧挨「Saving paused」胶囊；无锁态已降为描边。
- 修法：胶囊为「Saving paused」或「Memory only」时，页头 Save locally 统一切到描边样式；callout 内推荐动作保持本区唯一实心按钮。
- 来源：U4 a 新-4、U4 b 新-4。

#### UI-184 · [P3] 冲突键值栅格值列过窄，修订行折行出孤词，标签贴着值（U4 · 布局、细节 · 可挽回 a 0.1、b 0.1）

- 位置：U4-conflict-1440-light y341–440，值列从 x≈157 起只剩约 128px；「Revision 2 · Rustify / Light」折行；「Saved theme count」与「0 → 0」只隔约 8px；「Draft differences」紧贴上一行。
- 修法：280px 编辑列内键值改为标签在上、值在下，或 `max-content 1fr` 并把标签缩短为「Saved themes」；修订号与主题名分两行；「Draft differences」前加 8–12px。
- 来源：U4 a 新-6、U4 b 新-1。

#### UI-55 · [P3] 取消态 callout 两段互相重复，并与标题重复（U4 · 细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U4-upgrade-cancelled 标题 y282 与两段正文 y313–422，「upgrade cancelled」「memory-only」「reopening restores」各出现两次。
- 修法：并为一段，同时补上 UI-179 的按钮。
- 来源：U4 a UI-55（部分解决）；U4 b 判已解决，把段内重复记入 UI-139。

#### UI-144 · [P3] 文案余项：「0 → 0」无变化行、成功态时态错（U4 · 细节 · 部分解决 · 可挽回 a 0.1、b 0.1 中的这一部分）

- 位置：U4-conflict y433「Saved theme count 0 → 0」；U4-upgraded y334–370「…back up the library before upgrade.」
- 修法：数量没变时不显示或写「(unchanged)」；成功态改为「Original data keeps a copy of the library from before the upgrade.」
- 来源：U4 a UI-144（部分解决）；U4 b 判已解决，另列新-5，其中这两处判为同一问题（同一文字、同一位置）。

#### UI-185 · [P3] 状态用词不统一：「upgrade」和「migration」混用，成功态重复「Saved on this device」（U4 · 细节 · 可挽回 a 0.1）

- 位置：「Review the library upgrade」（callout）、「Review upgrade」（按钮）、「Review the complete migration」（对话框标题）、「Complete migration JSON」混用；U4-upgraded 页头胶囊与 callout 第二行都写「Saved on this device」。
- 修法：全流程统一用「library upgrade」；成功 callout 删去「Saved on this device」这一行，由页头胶囊承担。
- 来源：U4 a 新-7；U4 b 新-5 也指出成功态重复「Saved on this device」。a 新-7 中「取消态胶囊写 Saving paused、callout 写 Memory only」这一点归入 UI-114。

#### UI-114 · [P3] 页头基准用词同义反复，胶囊用语与 callout 不一致（U4 · 交互、细节 · 部分解决 · 可挽回 b 0.05）

- 位置：U4-upgrade-pending 页头「旧草稿 Matches “旧草稿”」；U4-quota-failure「Rustify Light Modified from “Rustify Light”」看不出基准是预设还是已保存文档；U4-upgrade-cancelled 胶囊「Saving paused」、callout 标题「Memory only」。
- 修法：基准改写为「Matches saved document」或「Modified from saved document」；取消态胶囊改为「Memory only」。
- 来源：U4 b UI-114（部分解决）、U4 a 新-7 的胶囊用语部分；U1 a、U4 a 判已解决。

#### UI-142 · [P3] 升级 Review 对话框主题表底边与分节线形成双细线（U4 · 布局、细节 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-upgrade-review-1440-light，y=309 与 y=322 两条 #EAECF0。
- 修法：表格最后一行去掉下边框，或分节容器紧跟表格时不画上边框。
- 来源：U4 a、b UI-142（均部分解决）。

#### UI-145 · [P3] 冲突差异块一处改动占四行，徽标左缘不齐（U4 · 布局 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-conflict-diff-1440-light 路径 y147、徽标 y185、列标签 y220、值 y240；徽标左边框约 x45、路径文字 x41。
- 修法：路径与徽标同一行，下一行写「This tab → Other tab」两列；徽标左缘与路径对齐。
- 来源：U4 a、b UI-145（均部分解决）。

#### UI-180 · [P3] 配额态「Technical details」被移到错误 callout 之外（U4 · 布局 · 可挽回 a 0.05、b 0.05）

- 位置：U4-quota-failure / U4-quota-operations，红色 callout y261–406，「Technical details」在 y435，位于命名卡之上；R2 时在 callout 内。
- 修法：放回错误 callout 内、「Download current theme」下方，与正文左对齐。
- 来源：U4 a 新-2、U4 b 新-2（两位都标为相对 R2 回归）。

#### UI-183 · [P3] 文字按钮的标签比正文缩进 11px，单独一行的「Dismiss」像是悬空（U4 · 视觉 · 可挽回 a 0.05）

- 位置：「Cancel upgrade」标签左缘 x=45、正文 x=34（U4-upgrade-pending y501）；「Dismiss」在 U4-upgrade-cancelled y641、U4-quota-failure y536、U4-no-locks y518。
- 修法：文字按钮用 ghost-offset 几何（`margin-inline:-11px`），或让 Dismiss 与本卡主动作同一行。
- 来源：U4 a 新-5。

#### UI-56 · [P3] 操作说明段讲本状态没有的动作，说明与列表未缩进，修订号被断行（U4 · 细节 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-quota-operations y612–666 最后一句「Loading the other version removes all pending actions.」；说明段与列表从 x=20 起；y744–762「(draft / revision 2)」被断开。
- 修法：说明段按当前状态只列可用动作；展开内容整体缩进到摘要文字起点；修订号片段 `white-space:nowrap`。
- 来源：U4 a、b UI-56（均部分解决；断行由 b 补充）。

### U5

#### UI-186 · [P2] 拾取浮动面板固定在预览右下角，遮住被拾取部件和预览内容；选中矩形与标签画在面板之上（U5 · 交互、布局、视觉 · 可挽回 a 0.25）

- 位置：U5-inspector-undeclared-hover / -undeclared / -undeclared-panel 面板 x 1000–1416、y 740–883；U5-inspector-card / U5-inspector / U5-inspector-picked 面板 y 707–883。「Create · 0」主体被面板压住；「Not annotated」标签（y 849–869）压过面板文字；刚开启时面板挡住右卡「Sample actions: 0」「ThemeDocument」。
- 现状 → 期望：被定位的部件和信息面板要同时可见、互不遮挡 → P2。
- 修法（需要结构性调整）：推荐把面板从覆盖层移进预览列网格的最后一行（`grid-template-rows: auto minmax(0,1fr) auto`，面板 `max-height:40vh; overflow-y:auto`），选中矩形与标签的覆盖层放在预览滚动区内并 `overflow:clip`。坚持浮动面板则至少给滚动容器留出面板高度的底部内边距、与目标相交时把面板翻到右上角、覆盖层 `z-index` 低于面板。
- 来源：U5 a 新-1（评审注明这是 UI-147 修法带出的新问题）。

#### UI-187 · [P3] 拾取标签盖住相邻内容，并在连字符处断开 token 名（U5 · 细节、交互、布局 · 可挽回 a 0.15）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light Create profile 上方标签 x 396–507、y 603–639，两行「primary, primary-」「foreground, ring」，盖住复选框与「I understand」；U5-inspector-card-1440-light 标签 y 361–383 切掉上一行文字的下伸部。
- 修法：标签单行 `white-space:nowrap; width:max-content`，token 不在内部断行；内容缩成「首个 token +N」；大部件把标签放进矩形内侧左上角，小部件放外侧、空间不够时翻到下边；`pointer-events:none`；拾取确认后只保留矩形。
- 来源：U5 a 新-2（评审注明这是 UI-149 修法带出的新问题）。

#### UI-188 · [P3] 选中或按下后字重变粗，按钮变宽，右侧控件横移 1–2px（U5 · 细节 · 可挽回 a 0.05）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light，Inspect 由 x 756–822 变为 756–824，Focus preview 右移 2px；U5-gpu-compare 选 DOM + GPU 后 Inspect 右移 1px。
- 修法：给分段与切换按钮的标签预留粗体宽度（隐藏的粗体伪元素或 `min-width`），在共用的分段组件里改一次。
- 来源：U5 a 新-4。

#### UI-76 · [P3] DOM 与 GPU 两框首行左内缩相差 8px（U5 · 细节、布局 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 x=370（内缩 26），GPU「GPU · Cards」x=908（内缩 18）；纵向已对齐。
- 修法：GPU 场景根 padding 改为 24px，或两边统一取同一 token。
- 来源：U5 a UI-76（部分解决）。

（U5 另有 UI-161，见 U1 节。）

### U6

#### UI-81 · [P3] classic 的内高光（上缘高光带与两侧亮边）只有目标的 1/3 到 2/3（U6 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U6-fixture-1280-dark classic 4 对 radix-sink-button classic 4。主体渐变已达目标约 80%；上缘第 1 行 +50（目标 +78），其下衰减只有 1px（目标约 3px 的 +15、+7、+2）；左右侧缘 +6（目标 +18、+4）。
- 修法：只改 Button classic 变体的 inset 阴影与背景渐变层：上缘高光不透明度提高约 1.5 倍并在其下加 2–3px 带模糊的亮带；左右亮边不透明度提高约 3 倍并向内多衰减 1px；底部暗线保持。改完运行 `mbx xtask css`。
- 来源：U6 a UI-81（部分解决）。

### U7

#### UI-87 · [P2] Inset side=left/right 演示中正文压在贴边块边上（U7 · 布局 · 部分解决 · 可挽回 a 0.3）

- 位置：U7-layout-inset-demonstration-1280-light side=left 卡贴边块 y=47–207，正文大写字母顶部约 y=203；side=right 卡正文 y≈352–366，贴边块顶边 y≈362；390 与 U7-fixture-1280 y≈4740–5220、110% 帧同样存在。
- 现状 → 期望：贴边块与正文上下堆叠且重叠 → radix-inset 中 side=left/right 的图块贴三边、正文并排；L3 使内容贴边；L5 容器边界不遮住内容。
- 修法：只改夹具 Inset 区：side=left/right 两张卡改为 `Flex direction=row align=stretch gap=3`，贴边块给定宽度并随卡片拉伸，正文单独一列；right 镜像。验收为贴边块三边与卡片内沿重合、块内没有正文像素。
- 来源：U7 a UI-87（部分解决；评审将严重度从 P3 升为 P2）。

#### UI-189 · [P3] Reset 的 input、textarea 脱离值列，占满整行（U7 · 布局 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light「input · Reset」值列空，input 落到下一行 x=0–1200、y≈1070–1101；textarea 同样（y≈1277–1330）；390 同；390 下原生 blockquote 内容也掉到下一行。
- 修法：对照行改为两列 grid（`112px minmax(0,1fr)`，390 用 96px）；查 Reset 后 input/textarea 的宽度来源，Reset 不应附加原生没有的尺寸。
- 来源：U7 a 新-2。

#### UI-95 · [P3] 布局原语区仍有样例没有可见呈现（U7 · 细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light y≈1567「Box · width=37px · height=19px · padding=3px」下方全白；240px 非法样例在默认态与回退后完全一样。
- 修法：37×19 盒加 gray-a3 底或 gray-a7 边框并标「37×19」；240px 样例同样加底色，让 240px 与 auto 宽度可区分。
- 来源：U7 a UI-95（部分解决；标注已补齐）。

#### UI-190 · [P3] Reset 区 button 行右侧有没有标注的「0」（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light x≈259、y≈131；390 x≈245、y≈150；U7-fixture-1280 y≈1814。
- 修法：若是计数器，改为「clicks: 0」并用标注同款样式；否则删除。
- 来源：U7 a 新-3。

#### UI-191 · [P3] 非法属性诊断文案重复，并带字面反引号（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-invalid-detail-1280-light y=487、U7-invalid-1280-light y≈1700：「Box · invalid prop \`width\`: … received "12px;display:none". Rejected value: 12px;display:none. Fallback: width=auto. …」
- 修法：改成一句，如「Box · width: rejected "12px;display:none" (expected one CSS value) · fallback width=auto」；属性名用 Code soft size 1 而非字面反引号。
- 来源：U7 a 新-4。

## 建议（不扣分）

摘自各评审原文的建议节，详见原文。

- U1：
  - 编辑列顶部存储状态块与顶栏保存胶囊重复，可压成一行摘要加展开，让预设搜索更早出现。
  - 1440 下两个 18px 标题字重不同，可把应用名降为 14px/600 或统一字重。
  - 保存状态胶囊加前导标识，区分未保存与已保存的形状。
  - 品牌色、中性色选项加色块。
  - 加跳到预览工具栏的链接或 landmark（品牌色选择器到 Dark 要 Tab ×114）。
  - 200% 导出对话框标题行与关闭按钮吸顶。
  - 帮助文字行距约 19.5px（13px × 1.5），确认归哪一档字级。
- U2：
  - 来源块同时给出另一模式的状态并附分组解除入口（沿用 R2）。
  - primary 为中性灰时拾色平面不要显示饱和蓝（沿用 R2）。
  - 「Overridden only」写出分模式计数（沿用 R2）。
  - 预览诊断条超过 3–4 对时折叠（a、b）。
  - Borders & focus 等没有配对的组只占左列，可按两列流式排布（b）。
  - 390 下滚动时收起头区、只保留 Edit/Preview 标签（a、b，属 U1 范围）。
  - 在精调区末尾也放一个「Restore document」入口（b）。
- U3：
  - JSON 导出 2 空格缩进，代码框 `white-space:pre` 加横向滚动（沿用 R2）。
  - 对话框标题行吸顶；Apply / Cancel 吸底；Cancel 改描边次级按钮。
  - Rust 页「Download」改名「Download theme.rs」；0.2.1 拒收单独成行警示；补普通 Cargo 构建命令。
  - 差异表颜色值旁加色块；损失框与备注框视觉区分；390 卡片「Changed」徽标放到 token 名右侧；节标题与字段 label 区分层级。
  - 「To match this preview」后用文字标出预览模式（b）。
  - 对话框本体加底部渐隐，提示下面还有内容（b）。
- U4：
  - 页头状态胶囊按语义着色。
  - 主题名与基准同名时写明基准类型。
  - 冲突推荐动作放在首位或末位；冲突差异颜色值旁加色块。
  - 升级 Review 对话框的 × 与「Cancel upgrade」语义区分。
  - 对话框分节标题与字段标签区分层级；成功 callout 加 Dismiss。
  - 差异条目很少（≤3 条）时默认展开「Draft differences」（b）。
  - 升级 Review 展开 JSON 后标题行吸顶（b）。
  - 主题表 ID 列改为左对齐（b）。
- U5：
  - 简单对话框限宽 28–32rem 并垂直居中（本轮改为贴顶）。
  - 菜单与触发器留 4px 并加层级阴影；局部边界标题升级、外观选择器限宽。
  - GPU 色样标签不在连字符处断开；Host control 加一行说明。
  - 深色局部对话框的遮罩取根主题或中性深色。
  - 面板补色阶信息（属 F19，只作建议）；拾取面板提供折叠入口。
  - 390 下 Preview 页的粘性页头可收成一行（属 U1 范围）。
- U6：
  - 夹具补 high-contrast、gray、gray + high-contrast、disabled 各列，IconButton size × variant 矩阵，各色 classic 与 classic high-contrast，Link 1–9 字级与 underline 四模式。
  - 补「文字 + 图标」Button。
  - 「Caller utilities」示例改为可读配色。
  - 补拍 Unavailable 激活后的反馈、ThemePanel 覆盖态；Link 悬停帧悬停在 auto/hover 链接上并标注 underline 属性。
- U7：
  - Heading/Text/Blockquote 补色族行与 high-contrast 行。
  - Em 改用与 Quote 相同的衬线斜体。
  - Kbd 补多字符键，顶行键距加大。
  - Container 加嵌套样例；说明 `margin=3px` 的用意。
  - 「Your name」标签与输入框、Kbd 标注与键帽之间留 8px。
  - Code 表行高收紧；Wrap 与 Trim 组间距统一；夹具控制按钮显示当前状态；table 的 Reset 对照换成能看出效果的样例；「Box · 160 × 90px」盒子靠近标注；Code 表头 1280 与 390 写法统一。
