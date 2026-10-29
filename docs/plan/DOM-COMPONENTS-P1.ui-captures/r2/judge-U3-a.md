# U3 Studio 导入与导出 · R2 评审（重建基线）

本轮只评 U3。只看了指定材料：目标包、r2 中 U3 开头的 PNG、capture.md、U3-actions.log；r1 中 U3 开头的 PNG 与 capture.md；r1/judge.md 的第 15–25、70–84、286–390 行。像素值由截图取样得出（ImageMagick 读色、逐行找边界）。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.4 | 2.65 | 2.8 | 0.5 | **8.35**（R1 7.0） |

- **布局 2.4**：对话框宽高上限、≤560px 单列、390 下差异表改成卡片，这些都对了。到不了 3，原因有三：
  - 差异表被限高并裁切。1440 截掉最后一组；390 只完整露出 2/6 项，第 3 项只剩 token 名。
  - 切换导出类型时，对话框重新垂直居中，Output 选择器位移 12–109px。
  - 390 下分组行背景只铺约 105px，没有铺满卡片宽度。
- **视觉 2.65**：壳层色值已全部对上，包括次级文字 #475467、分隔 #D0D5DD/#EAECF0、错误 #B42318、选中 #EFF8FF/#175CD3，颜色值也改成了等宽字。到不了 3，原因如下：
  - 所有 label、表头、节标题仍是 13px/400，规范是 13px/600。字段标签和节标题同一样式，层级平。
  - 备注框内多出一条线。
  - Rust 说明里的调用表达式只有函数名用了等宽字。
- **交互与状态 2.8**：以下主路径都走得通：
  - 审阅显示版本、名称、配置、分组差异和备注；
  - 未知版本给出正确诊断，文本域出现错误边框；
  - v1 导出列出 7 条损失，覆盖 X2 的六类；
  - CSS 有 2 种 profile、4 种颜色记法，并标明「resolved values」；
  - Rust 说明写出修订号、0.2.1 拒收 v2、字体外置、ThemeMode::Light；
  - 复制成功为绿色文字，按钮不再被挤动。

  到不了 3：Apply 前，候选差异无法一眼看全，被裁切且没有任何滚动提示（新-1）。
- **细节 0.5**：有若干小差异：
  - 「256 / KiB」被拆到两行；
  - palette 与 color 混用，引号用法不统一；
  - Auto 没写出实际灰阶；
  - 备注与表格对 token 的命名不一致；
  - 表头「Mode / token」已不准确；
  - `16.0` 与代码里的 `16.` 写法不一致。

## 2. 上一轮差距复核

| 编号 | R1 问题 | 本轮状态 | 依据（R2 截图） |
| --- | --- | --- | --- |
| UI-24 | 未知版本给出误导性诊断 | **已解决** | U3-import-error：「Unsupported Theme JSON version 999. This Studio reads v1 and v2.」，下有「Technical details」折叠项 |
| UI-25 | 版本、名称、配置是无标签散文 | **部分解决** | 已有「Import candidate」小标题、dl 标签和「Changes (6)」。但 Version/Name 是左右排的「标签 值」，配置项却是两列「标签在上、值在下」，同一节用了两种模式 |
| UI-26 | label、表头、备注标题为 14px/400 | **部分解决** | 字号已是 13px，颜色已是 #475467；字重仍为 400。与 R1 的 14px/400 比较「I」竖笔，粗细按字号同比例缩小，说明字重没变 |
| UI-27 | 行线、备注框、代码框描边为 #858F9E | **已解决** | 表行线 #EAECF0；备注框、损失框、代码框描边 #D0D5DD |
| UI-28 | Rust 说明没写所需版本 | **已解决** | 「Required source revision f5613b47…」，代码首行注释同步 |
| UI-29 | 差异表颜色值是比例字体 | **已解决** | 改为等宽约 12px（13 个字符宽 93px） |
| UI-30 | 错误文字 #D92D20，文本域无错误边框 | **已解决** | 错误文字与文本域 1px 边框均为 #B42318 |
| UI-31 | 配置行是 snake_case / null 格式 | **已解决** | 改为 Appearance/Brand color/… 加 Inherit/Custom/100% |
| UI-32 | v1 损失缺 appearance、scaling | **已解决** | 7 条损失含「Appearance intent」「Scaling intent」 |
| UI-33 | 差异不区分增删改 | **已解决**（Added/Removed 待核实） | 新增 Change 列；按 Identity / Light·base / Light·overrides / Dark·base / Dark·overrides 分组计数。本轮数据只有「Changed」 |
| UI-34 | 390 token 列在字符中间断行 | **已解决** | 改为卡片，`base.primary` 完整显示；但分组行宽度出了新问题，见新-2 |
| UI-35 | 「Saved on this device」和复制反馈夹着按钮 | **已解决** | 库状态行已移出对话框，反馈与按钮同行 |
| UI-36 | 复制反馈挤动按钮、对话框重新居中 | **已解决**（复制部分） | rust 与 copied 两帧只在 233,741 起 402×32 的按钮行内有差别。切换 Output 时的重新居中仍在，见新-9 |
| UI-37 | 复制成功没有成功语义 | **已解决** | 「Copied to clipboard.」为 #067647，对白底约 5.7:1 |
| UI-38 | 次级文字为 #667085 | **已解决** | label、字节计数、Technical details、Change 列、说明文字均为 #475467 |
| UI-39 | 字节单位不统一 | **已解决** | 「3.25 / 256 KiB」「0.02 / 256 KiB」 |
| UI-40 | `font_context(16, true)` 与代码 `16.` 不符，标识符用比例字 | **部分解决** | 函数名 `mount_exported_preview` 改成了等宽，但括号内 `(ThemeMode::Light, &font_context(16.0, true))` 仍是比例字；`16.0` 与代码 `16.` 写法仍不同 |
| UI-41 | 「Tailwind v4 CSS」与「Rustify scoped」命名冲突 | **已解决** | 改为「CSS · resolved values」与「Rustify scoped · data-theme」/「Tailwind v4 · .dark」 |
| UI-42 | 字体路径两种拼法 | **已解决** | 注释已分别注明「relative to this deployed CSS file」和「relative to the SDK source checkout」 |
| UI-43 | 表单行距 12px 与 8px 不一 | **已解决** | 三个选择器 169–204 / 217–252 / 265–300，间隔都是 12px |
| UI-44 | 损失清单取值写法生硬 | **部分解决** | 取值已改为带引号的界面名（“Pink”“Auto”“Inherit”）。但 palette 与 color 混用，100% 没加引号 |

## 3. 差距清单

### 新-1 · [P2] 差异表被限高、二级滚动，Apply 前看不全（布局 + 交互）

- **单元与位置**：
  - U3-import-review-end-1440-light：差异表在 y≈715 处把「Dark · overrides (1)」一行横腰截断，下方的 `overrides.primary` 那一项完全看不到；离备注框只隔 13px 白边。
  - U3-import-review-end-390-light：卡片列表在 y≈610 处截断。只完整显示 Identity 下 2 项；「Light · base (1)」后只露出 `base.primary`，看不到 Changed、Current、Imported，后面 3 组都不可见。截断恰好落在两行之间，看起来像列表到此结束。
  - 对话框自身也在滚动，形成套嵌滚动。
- **现状 → 期望**：现状是差异表为对话框内的第二个限高区，没有边框、渐隐或「还有 N 项」提示。期望依据 X1（导入先呈现候选差异，Apply 后才替换，差异须在 Apply 前可读）和 X6（对话框内部纵向滚动，只有一层）：用户沿对话框这一个滚动条往下滚，能依次看到全部 6 项再到 Apply。
- **修法**：
  - 去掉差异表外层（`import-review` 结果区里的 table wrapper）上的 `max-height` / `overflow`，让表格在对话框正文里自然流动。
  - 只有条目非常多（例如 >50 行）才需要限高时，同时做到：
    - 容器加 1px #D0D5DD 描边和圆角；
    - 表头 `position: sticky; top: 0`；
    - 底部显示「Showing 20 of N · Show all」文字按钮；
    - 截断只落在组边界上，不切半行。
  - 390 卡片模式同样处理。
  - 验证：1440 与 390 滚到 Apply 时，「Dark · overrides (1) / overrides.primary」完整位于备注框上方。
- **可挽回分**：0.45（布局 0.25、交互 0.2）

### UI-26 · [P2] 对话框 label、表头、节标题字重仍为 400，层级平（视觉）

- **单元与位置**：U3 全部帧，涉及以下文字：
  - 导入：「Input format」「Theme source」「Import candidate」「Version」「Name」「Appearance」…「Changes (6)」；
  - 差异表表头：「Mode / token / Change / Current / Imported」；
  - 「Import notes」；
  - 导出：「Output」「CSS profile」「Color notation」「Exported code」「Required source revision」。
- **现状 → 期望**：现状是 13px / 400 / #475467，与 12px 说明文字几乎同一层级。「Import candidate」「Changes (6)」这类节标题与字段标签完全同样式。期望依据 G3：label 为 13px / 600。
- **修法**：
  - 对话框的标签、`dt`、`th` 和节标题统一用壳层 label 样式：`font-size:13px; font-weight:600; color:#475467`。
  - 值、正文保持 14px / 400 / #1D2939。
  - 验证：「Input format」中「I」的竖笔覆盖宽度从约 1.4px 增加到约 1.9px 以上。
- **可挽回分**：0.25（视觉）

### 新-9 · [P2] 切换 Output 时对话框重新垂直居中，选择器跳动 12–109px（布局）

- **单元与位置**：四个导出帧的对话框上沿与 Output 选择器上沿：

  | 帧 | 对话框上沿 y | Output 选择器上沿 y |
  | --- | --- | --- |
  | U3-export-json | 179 | 241 |
  | U3-export-v1 | 70 | 132 |
  | U3-export-css | 107 | 169 |
  | U3-export-rust | 95 | 157 |

  JSON 切到 v1 时，刚操作的选择器上移 109px。导入时，错误态上沿 148 与审阅态上沿 17 也是同一机制。
- **现状 → 期望**：现状是高度随内容变化、垂直居中。期望依据 X6：对话框宽高为视口减 2rem，内部纵向滚动，即高度固定、只有正文滚动，切换内容时头部和 Output 位置不变。R1 的 UI-36 把同类「重新居中」计为差距，本条沿用该口径。
- **修法**：二选一。
  - 导入导出对话框设 `height: calc(100dvh - 2rem)`，正文 `overflow-y:auto`，按钮行留在正文末尾；
  - 或对话框改为顶部锚定，overlay 用 `align-items:flex-start`，对话框 `margin-top:1rem`。

  验证：四种 Output 帧中 Output 选择器的 y 相同。
- **可挽回分**：0.15（布局）

### 新-2 · [P2] 390 卡片模式下分组行只铺约 105px（布局）

- **单元与位置**：U3-import-review-end-390-light 的「Identity (2)」「Light · base (1)」分组行。#F9FAFB 背景与下方 #EAECF0 线只覆盖 x 41–145，而卡片宽到 x 348。
- **现状 → 期望**：现状是分组行像一个孤立的灰色小块。期望依据 X1 与 G2 的 ≤560px 单列：分组行与 1440 一致，横贯整行（1440 下 #F9FAFB 横贯 233–1207）。
- **修法**：≤560px 把表格行改成 block/grid 时，分组单元格（`th[colspan]`）设 `display:block; width:100%`，或在 grid 中设 `grid-column:1 / -1`，让背景和下边框都铺满卡片宽度。
- **可挽回分**：0.1（布局）

### UI-40 · [P3] Rust 与 CSS 说明里的标识符只有部分用等宽字，数值写法与代码不一致（视觉、细节）

- **单元与位置**：
  - U3-export-rust-1440 第 4 行「To match this preview, call `mount_exported_preview`(ThemeMode::Light, &font_context(16.0, true))」：只有函数名是等宽，参数是 Plex Sans。
  - 同帧第 2 行的 theme.json、theme.rs、AppHandle、0.2.1 是比例字。
  - U3-export-css 说明行的 data-theme=dark、.dark 是比例字。
  - 代码里写 `font_context(16., false)`，说明里写 `16.0`。
- **现状 → 期望**：期望依据 G3（值类文字用等宽）与 X4（接入说明可照抄）：完整调用表达式、文件名、类型名、选择器都在同一个等宽 `<code>` 里，数值写法与代码一致。
- **修法**：
  - 说明文案模板中，把整段 `mount_exported_preview(ThemeMode::Light, &font_context(16.0, true))` 包进同一个 `<code>`（等宽 12px）；
  - `theme.json`、`theme.rs`、`AppHandle`、`data-theme="dark"`、`.dark` 同样处理；
  - 生成代码与说明统一用 `16.0`，或统一用 `16.`。
- **可挽回分**：0.15（视觉 0.05、细节 0.1）

### 新-3 · [P3] 「Import notes」框内多一条横线（视觉）

- **单元与位置**：U3-import-review-end-1440 的备注框上边框 #D0D5DD 在 y=728，框内 y=741 另有一条 #EAECF0 横线；390 帧在 y≈643 也有。
- **现状 → 期望**：现状是带框面板顶部出现双线。期望依据 G3：壳层扁平，用细分隔线和分组行，不叠加装饰。
- **修法**：备注标题复用了节分隔样式（border-top 加 padding-top）。在备注框内取消该 border-top（例如 `.import-notes > :first-child { border-top:0; padding-top:0 }`），或改用不带分隔线的标题样式。
- **可挽回分**：0.05（视觉）

### UI-25（余项）· [P3] 同一「Import candidate」节里有两种标签与值的排法（布局）

- **单元与位置**：U3-import-review-1440 的 y 598–625 与 y 656–791 两段。
- **现状 → 期望**：现状是 Version/Name 左右排（标签 60px 宽，值紧随其后）；Appearance…Scaling 是两列、标签在上值在下，两列相距 500px。期望依据 X1：候选信息按版本、名称、配置用同一种 dl 结构呈现。
- **修法**：把 Version、Name 并入同一个两列 grid，作为第一行（Version | Name），与配置项一样标签在上、值在下。也可以全部改成左右排，二选一。
- **可挽回分**：0.05（布局）

### 新-8 · [P3] 导出对话框按钮行下方留白大于 1.5rem（布局）

- **单元与位置**：
  - U3-export-json-390：按钮底 y=661，对话框下边框 y=712，留白 51px；左右内边距为 24px（17→41）。
  - U3-export-json-1440：按钮底 688，下边框 720，留白 32px。
- **现状 → 期望**：期望依据 X6，内边距 1.5rem。按钮下方多出的部分疑似空的复制状态行在 390 下换到了下一行。
- **修法**：复制状态元素为空时 `display:none`。新-9 改为顶部锚定后，状态行出现在按钮下方不会挤动任何上方元素。同时去掉 1440 下多出的 8px 外边距。
- **可挽回分**：0.05（布局）

### 新-4 · [P3] 390 下「256 KiB」被拆到两行（细节）

- **单元与位置**：U3-import-review-390 的说明文字「…up to 256」与下一行「KiB. Review changes before applying.」。
- **现状 → 期望**：期望依据 G6：窄屏文案可读，数字与单位不分离。
- **修法**：中英文案中数字与单位之间用不换行空格（`256\u00A0KiB`）；字节计数「3.25 / 256 KiB」同样处理。
- **可挽回分**：0.05（细节）

### UI-44（余项）· [P3] 损失清单用词与基础配置不一致，引号用法不统一（细节）

- **单元与位置**：U3-export-v1-1440 损失框。
- **现状 → 期望**：
  - 现状：损失框写「Brand palette “Pink”」「Neutral palette “Auto”」，导入审阅写「Brand color」「Neutral color」。「Scaling intent 100%」没加引号，其他取值都加了引号。
  - 期望依据 C2：基础配置标签为品牌色、中性色；G6：文案同步。同一概念在 Studio 内只用一个名称。
- **修法**：损失清单的字段名直接取基础配置的标签（Brand color / Neutral color / Appearance / Radius / Scaling），取值统一加引号（“100%”），中文同步。
- **可挽回分**：0.05（细节）

### 新-5 · [P3] v1 损失里的 Auto 没写出实际配对的灰阶（细节，较 R1 退步）

- **单元与位置**：U3-export-v1-1440 第 2 条「Neutral palette “Auto”. v1 cannot retain Auto pairing or the selected neutral family.」。R1 写的是「auto (resolved mauve)」。
- **现状 → 期望**：期望依据 C2：中性色 auto 要显示实际配对的灰阶名。X2 的损失要具体。
- **修法**：写成「Neutral color “Auto” (resolves to Mauve)…」，取值与基础配置选择器上显示的有效 auto 结果为同一来源。
- **可挽回分**：0.05（细节）

### 新-6 · [P3] 导入备注与差异表对 token 的命名不一致（细节）

- **单元与位置**：U3-import-review-end-1440 / 390：备注写「base.light.font-serif」（比例字）；差异表写分组「Light · base」加等宽 `base.primary`。
- **现状 → 期望**：期望依据 X1：差异与诊断要能对照阅读，诊断能定位到表里的项。
- **修法**：备注里的 token 路径改用与表格相同的写法（「Light · base · `font-serif`」），token 段用等宽 `<code>`。
- **可挽回分**：0.05（细节）

### 新-7 · [P3] 差异表表头「Mode / token」已不准确（细节）

- **单元与位置**：U3-import-review-end-1440 的表头首列。模式已移到分组行，该列只显示 `id`、`base.primary` 等。
- **现状 → 期望**：期望依据 X1：列名与内容一致。
- **修法**：首列表头改为「Token」（中文「Token」或「键」），分组行保留模式信息。
- **可挽回分**：0.05（细节）

**合计可挽回**：0.45 + 0.25 + 0.15 + 0.1 + 0.15 + 0.05 × 8 = **1.50**。10 − 8.35 = 1.65，差 0.15，在允许误差 0.5 以内。

## 4. 建议（不扣分）

- JSON 导出正文现为单行压缩 JSON 软换行，`"accent-` 与 `foreground"`、`"chart-` 与 `1"` 被断开。可以改为 2 空格缩进输出，可读性和 diff 友好度都更好。
- CSS / Rust 代码框目前软换行，出现了「IBMPlexSans-」与「Text.ttf. */」断开、`*/` 单独成行的情况。可以改为 `white-space: pre` 加横向滚动，保留作者的换行。
- Rust 页有两个下载按钮：「Download matching JSON」与底部「Download」。建议底部改名为「Download theme.rs」，避免误解。
- 「the published 0.2.1 SDK/CLI reject v2」埋在 12px 灰色段落中间，可以单独成一行警示。「Build the CLI from that revision」只给了构建命令，没给检出该修订的步骤。
- 差异表的颜色值旁可以加 12px 色块，便于比对 #1570ef 与 #7c3aed。
- 导出代码框右下角有 textarea 拖拽柄，可以限为 `resize: vertical`，或去掉。
- 导入对话框的「Cancel」是纯文字按钮，「Choose file」「Review changes」是描边按钮，次级按钮样式可以统一。

## 5. 看不出的格子

- **键盘焦点（G4）**：所有 U3 帧的焦点都来自鼠标点击或程序设置：close、export-kind、export-copy、import-review、export-profile-list 等。放大后看不到焦点框，无法判断对话框内控件的键盘焦点可见性。
- **Added / Removed 标签**：本轮候选只有「Changed」，新增或删除项的文字标签与「—」占位都看不出。
- **Apply 之后、取消、语法错误、超限、不支持的颜色输入**：是否保留当前主题与历史（X1）未采到。未知版本帧背景仍显示「Rustify Light · Matches preset」，只能证明这一种情况。
- **下载**：下载实际发起后的反馈，以及复制或下载失败的状态（X5）未采到。
- **中文**：U3 的中文文案（G6）未采到。
- **滚到底的底部内边距**：U3-import-review-end 的 1440 与 390 两帧，「Apply import」都贴着对话框下边框。看不出是采集用 scrollIntoView 没滚到底，还是正文底部 1.5rem 内边距真的丢了。
- **新-1 的容器类型**：headless 隐藏了滚动条，看不出差异表是 `overflow:auto` 还是 `overflow:hidden`。若是 hidden，用户根本无法看到后几项，应升为 P1。
- **其余未采状态**：390 下的 v1、CSS、Rust 导出与复制反馈换行未采到。「Technical details」展开后的内容未采到。U3-export-copied 中 Copy 按钮为 #2C7EF1，比静止时的 #1570EF 浅，疑为指针仍停在按钮上的悬停色，不作评判。

## 6. 总评

- 本轮只评 U3，得 **8.35**，比 R1 的 7.0 明显提高。R1 的 21 条差距里，17 条已解决，4 条部分解决（UI-25、UI-26、UI-40、UI-44），没有回归。
- 主要失分：
  - 导入审阅最关键的「候选差异」被限高裁切（新-1）。1440 少了最后一组，390 只完整露出 2/6 项且截断点像列表结尾，直接削弱 X1 的「Apply 前审阅」。
  - 对话框 label 字重仍为 400，层级扁平（UI-26）。
- 需要结构性改动的地方：
  - 差异表从「限高子滚动区」改为在对话框单一滚动流里展开（新-1）。
  - 导入导出对话框从「随内容高度垂直居中」改为「固定为视口减 2rem 或顶部锚定」（新-9）。这样切换导出类型、复制反馈、审阅结果出现时，头部和已操作的控件都不会移动。
- 其余各条都是局部样式与文案的微调。
