# U3 Studio 导入与导出 · R3 独立评审

本轮评审依据以下材料：
- 目标包规范要点 X1–X6、G1–G4。U3 为规范模式，没有目标图。
- r3 中以 `U3-` 开头的 19 张 PNG、`U3-actions.log` 和 `capture.md`。
- r2 中以 `U3-` 开头的 PNG 与 r2 的操作记录。
- `r2-judge-U3.md`（R2 采用 a，8.35 分）。

尺寸与颜色在原图上逐像素读取，方法是用 ImageMagick 取色和扫行列。`U3-export-rust-mid-1440-light` 按 capture.md 的说法与顶部帧相同，不作中段证据；代码中段只用 `-a2` 帧。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.9 | 2.9 | 2.9 | 0.6 | **9.3** |

各维度理由（每条说明为什么不是更高一档）：

- **布局 2.9**
  - 已对上的部分：
    - 对话框宽 1024px，即 64rem（x 208–1231），上沿统一在 y=16/17，高为视口减 2rem（17–883）。
    - 切换导出类型时 Output 选择器固定在 y 78–113。
    - 差异表不再嵌套滚动，随对话框本体滚动。
    - ≤560px 时表单单列、分组行铺满。
  - 不到 3 分的原因：
    - 导出对话框按钮下方留白 32px，不是 X6 的 1.5rem（导入对话框是 24px）。
    - Import candidate 两组行距不一致：27px 对 29px，组间 33px。
    - Changed 徽标与表头、卡片正文的左缘差 3–11px。
- **视觉 2.9**
  - 已对上的部分：
    - label、表头、节标题全部改为 600。
    - 次级文字 #475467，分隔 #d0d5dd / #eaecf0。
    - 选中项 #eff8ff 底、#175cd3 字；错误 #b42318（文字与边框）。
    - 代码 13px、行距 20.8px 等宽。
    - 说明里的标识符全部等宽。
  - 不到 3 分的原因：
    - 损失清单与 Import notes 的正文从 R2 的 14px 降到 13px/400，G3 里没有这个角色。
    - 备注里的作者值 `ui-serif` 用的是比例字。
- **交互与状态 2.9**
  - 以下各项都走通了：
    - 导入审阅：版本、名称、配置、六项分组差异与备注完整，Apply 前一屏滚动即可看全。
    - 未知版本错误：红框加文案，附 Technical details。
    - 兼容 v1：七条损失，色族与 Auto（Mauve）、appearance、radius、scaling、来源、派生角色全覆盖。
    - CSS：两种 profile、四种颜色记法。
    - Rust：修订号与 0.2.1 拒收说明。
    - 复制：绿色「Copied to clipboard.」，#067647。
  - 不到 3 分的原因：Import candidate 里箭头的含义不统一。Version 行是格式迁移，配置行是「当前 → 导入」且没有图例，Name 行只列导入值。读者要靠猜才知道哪边是当前主题。
- **细节 0.6**
  - 比 R2 少了 10 处以上的小差异。
  - 仍有若干处没对上：
    - 字节计数「22 / 262144 bytes」与「256 KiB」单位不一致，也没有千分位。
    - Rust 说明「Keep AppHandle.」丢了「alive」；「beside the Rust source: theme.json · theme.rs」没说清哪个文件放在哪个旁边。
    - CSS 说明写的「data-theme=dark」不是选择器写法。
    - 备注与表格对 token 路径的写法不同。
    - 390 下「ui-」「serif」在连字符处折行。
    - 徽标基线低约 2px，表头下方是 2px 双线。

## 2. 上一轮差距复核表

| 编号 | 状态 | 依据（本轮截图） |
| --- | --- | --- |
| UI-125 差异表内层限高滚动 | 已解决 | U3-import-review-end/bottom-1440 完整露出 Identity 到 Dark · overrides 六行、备注与 Apply；390 bottom 卡片完整。actions.log 只剩对话框本体（`div.relative.w-full` 0→626 / 0→1429）和 textarea 两个滚动区，`div.tools-scroll` 已不存在 |
| UI-26 label、表头、节标题字重 400 | 已解决 | Input format、Theme source、Import candidate、Version、Name、各配置名、Changes (6)、表头、Import notes、Output、CSS profile、Color notation、Exported code、Required source revision、390 卡片的 Current / Imported 全部为粗体（#1d2939） |
| UI-126 切换 Output 时对话框重新居中 | 已解决 | json、v1、css、rust、copied、导入审阅、错误态的对话框上沿都在 y=16/17，Output 选择器都在 78–113 |
| UI-127 390 分组行只铺约 105px | 已解决 | U3-import-review-mid/end-390「Identity (2)」「Light · overrides (1)」等铺满 x 41–349 |
| UI-128 配置没有与当前主题对比 | 部分解决 | 配置行改为「Inherit → Inherit」「Custom → Custom」「100% → 100%」。残留：箭头没有图例；Version 行的箭头含义不同；Name 行只有导入值（见差距清单） |
| UI-40 说明里标识符部分等宽，`16.0` 与 `16.` 不一致 | 已解决 | Rust 说明中 `theme.json`、`theme.rs`、`AppHandle` 以及整条 `mount_exported_preview(ThemeMode::Light, &font_context(16., true))` 等宽，与代码 `16.` 一致；CSS 说明中 `data-theme=dark`、`.dark` 等宽 |
| UI-42 字体路径两种拼法 | 已解决 | CSS 注释写明 license 文档「is in the SDK source checkout (not a deployed URL): makepad/widgets/…」，字体 URL「relative to this deployed CSS file」用 `makepad_widgets/…`。两种拼法分属两种路径，已在原文说明 |
| UI-25 Version/Name 与配置两种排法 | 部分解决 | 已统一为「172px label 列 + 值」同行排。残留：label 大写字母顶边依次为 594 / 621 / 654 / 683 / 712 / 741 / 770 / 799，两组行距 27 对 29，组间 33 |
| UI-44 palette/color 混用、引号不统一 | 已解决 | 「Brand color “Pink”」「Neutral color “Auto (Mauve)”」「Scaling intent “100%”」 |
| UI-129 备注框内多一条横线 | 已解决 | U3-import-review-end-1440 与 390 的备注框内没有横线 |
| UI-130 导出按钮行下方留白大于 1.5rem | 部分解决 | 390 从 51px 降到 32px，与 1440 一致，但仍比 24px 多 8px（导入对话框正好 24px） |
| UI-131 390「256 / KiB」拆行 | 已解决 | U3-import-review-390「up to / 256 KiB.」，数字与单位同行 |
| UI-132 Auto 没写实际灰阶 | 已解决 | 「Auto (Mauve)」 |
| UI-133 备注与表格的命名、字体不一致 | 部分解决 | 备注路径已改等宽，`name` 行作者文字已改比例字。残留：路径写法不同；备注里的作者值是比例字 |
| UI-134 表头「Mode / token」不准 | 已解决 | 「Field / token」 |
| UI-135 吸顶表头没有底边 | 已解决 | 表头不再吸顶，随对话框滚动，底边 #d0d5dd 在 y=301 |
| UI-136 Changed 与 label 同样式 | 已解决 | 改为 #f9fafb 底、#d0d5dd 边、#475467 字的徽标 |
| UI-137 22 字节显示为 0.02 KiB | 已解决（引出新-1） | 「22 / 262144 bytes」 |
| UI-138 修订号行没对齐表单列 | 已解决 | 修订号 x=405，与 Output 选择器左缘同列 |

## 3. 差距清单

所有剩余差距都是 P3，按可挽回分从高到低排列。合计 0.70，即 10 − 9.3。

#### UI-128 · [P3] Import candidate 的箭头有三种含义，没有「当前 / 导入」图例（交互、细节 · 可挽回 0.15）
- **位置**：U3-import-review-1440-light y 598–803；U3-import-review-mid-390-light y 58–338。
- **现状**：
  - Version「JSON v1 → v2」表示文件格式迁移。
  - Name「Imported legacy 旧主题」只列导入值，当前名只出现在下方表格的 Identity 组。
  - Appearance 至 Scaling「Inherit → Inherit」「Custom → Custom」表示当前 → 导入，但全节没有说明哪边是当前。
  - 未变化的行与变化的行样式相同。
- **期望**：X1「导入先呈现候选差异——版本、名称、配置」，读者能直接读出当前值和导入值。
- **修法**：
  1. 配置与 Name 行改为两列，表头沿用差异表的「Current」「Imported」。≤560px 时按差异表卡片的方式上下排「Current … / Imported …」。
  2. Name 行写成「Rustify Light → Imported legacy 旧主题」。
  3. Version 的 label 改为「Source version」，值写「JSON v1 (upgraded to v2 on apply)」，与比较箭头区分开。
  4. 未变化的值用 #475467，并标「Unchanged」。变化的值加与表格相同的 Changed 徽标，并计入 Changes (N)。

#### 新-2 · [P3] 导出说明有三处措辞不准（细节 · 可挽回 0.10）
- **位置**：
  - U3-export-rust-1440-light y 167–186：「Place the matching JSON beside the Rust source: theme.json · theme.rs.」「Keep AppHandle.」
  - U3-export-css-1440-light y 230：「Scoped dark selector: data-theme=dark」
- **现状**：
  - 用「·」并列两个文件名，没说哪个是 JSON、哪个是 Rust 源文件。
  - R2 原文是「Keep AppHandle alive.」，本轮丢了「alive」，读者不知道要保留它做什么。
  - `data-theme=dark` 不是选择器，代码里实际写的是 `[data-theme="dark"]`。
- **期望**：X4 接入说明要写清楚；细节维度要求文案与代码一致。
- **修法**：
  - 第一句改为「Place `theme.json` beside `theme.rs`.」
  - 第二句改为「Keep the returned `AppHandle` alive while the preview is mounted.」
  - CSS 说明改为「Scoped dark selector: `[data-theme="dark"]`; standard selector: `.dark`.」

#### 新-1 · [P3] 字节计数与描述单位不一致，也没有千分位（细节 · 可挽回 0.08）
- **位置**：U3-import-review-1440/390「3332 / 262144 bytes」；U3-import-error-1440 y 479「22 / 262144 bytes」；同一对话框 y 88 写的是「up to 256 KiB」。
- **现状**：上限在描述里用 KiB，在计数里用字节；6 位数没有分组。
- **期望**：同一对话框内同一上限用同一单位，数字格式随界面语言。
- **修法**：用 `Intl.NumberFormat(locale)` 格式化。不足 1 KiB 显示「22 B / 256 KiB」，超过显示「3.3 KiB / 256 KiB」。也可以保留字节，写成「3,332 / 262,144 bytes (256 KiB)」。

#### UI-133（续） · [P3] 备注与差异表的 token 路径写法不同，备注里的作者值是比例字（视觉、细节 · 可挽回 0.08）
- **位置**：U3-import-review-end-1440 y 795–815；U3-import-review-bottom-390 y 676–736。
- **现状**：
  - 备注写 `base.light.font-serif`，模式写在路径里。
  - 表格写「Light · base (1)」组标题加 `base.primary`，模式写在组标题里，「base」重复出现。
  - 备注中请求的值 `ui-serif` 用比例字，而表格里的作者值（#1570ef、rustify-light）都是等宽。
- **期望**：X1 的诊断要能对上差异表的行；G3「颜色值等宽」，作者值统一用等宽。
- **修法**：两处统一用同一套 path 格式化函数。可以让表格首列也写全路径 `base.light.primary`，也可以让备注写「Light · base · `font-serif`」。`ui-serif` 用 `<code>` 包起来。

#### 新-4 · [P3] Changed 徽标与列、基线没对齐，表头下方是双线（布局、细节 · 可挽回 0.08）
- **位置**：
  - U3-import-review-end-1440：表头「Change」文字左缘 x=573，徽标边框 x=576，徽标文字 x=584，相差 11px。
  - 同帧徽标字形 y 354–364，同行「rustify-light」y 352–362，基线约低 2px。
  - y=301 的表头底边 #d0d5dd 紧接着 y=302 的分组行上边 #eaecf0，形成 2px 双线。
  - U3-import-review-mid-390：徽标框 x=53，同卡的 token 与「Current」x=50。
- **期望**：同列的文字左对齐、同行的文字基线一致；X6 / G2 要求细分隔线。
- **修法**：
  - Change 列的 `th` 加上与徽标水平内边距相同的 padding-inline-start（约 8px），让表头与徽标文字同列；或者把徽标改为无左缩进。
  - 徽标改为 `display:inline-flex; align-items:center; height:20px; line-height:1`，单元格 `vertical-align: middle`，让徽标文字基线落在行文字基线上。
  - 加 `thead + tbody > tr:first-child { border-top: 0 }`，去掉双线。
  - 390 卡片里徽标 `margin-inline-start: 0`。

#### 新-3 · [P3] 损失清单与 Import notes 的正文是 13px/400，不是正文 14px（视觉 · 可挽回 0.07）
- **位置**：
  - U3-export-v1-1440 损失框 y 148–297，行距 19.5px，即 13px。同一框在 R2（r2 y 203–362）行距 21px，即 14px。
  - U3-import-review-end-1440 备注正文 13px，而同一对话框的描述（y 88）是 14px。
- **期望**：G3 正文 14px / 1.5，caption 12px，label 13px / 600。13px / 400 不在壳层字级里。
- **修法**：损失框与备注框的正文和列表改为 `font-size:14px; line-height:1.5`。「Import notes」小标题保持 label 的 13 / 600。

#### UI-25（续） · [P3] Import candidate 两组行距不一致（布局、细节 · 可挽回 0.06）
- **位置**：U3-import-review-1440-light。label 大写字母顶边依次为 594、621、654、683、712、741、770、799。
- **现状**：Version → Name 27px；配置各行 29px；Name → Appearance 33px，看得出原来是两个块。
- **期望**：同一节内的 label / 值行节奏一致（G2 分组行）。
- **修法**：Version、Name 与六个配置字段放进同一个 `dl` grid（`grid-template-columns: 172px 1fr`），只用一个 `row-gap`，去掉两块之间的额外 margin。

#### UI-130（续） · [P3] 导出对话框按钮行下方仍多 8px（布局 · 可挽回 0.04）
- **位置**：
  - 导出：U3-export-json-1440 按钮底 516、边框 549，留白 32px；U3-export-json-390 按钮底 547、边框 580，留白 32px。
  - 导入：U3-import-review-bottom-1440 为 858→883，留白 24px；U3-import-error-1440 为 595→620，留白 24px。
- **原因**：U3-export-copied-1440 的状态行出现后，按钮底 705、消息顶约 713、边框 754，说明空的状态 live region 仍占了 8px 栈间距。
- **期望**：X6 对话框 padding 1.5rem。
- **修法**：把复制 / 下载状态的 live region 移进按钮行，作为 flex 行尾项，并设 `flex-wrap: wrap`，这样空时不产生额外的行间距。不要用 `display:none` 切换，以免影响读屏播报。

#### 新-5 · [P3] 390 备注里「ui-serif」在连字符处折行（细节 · 可挽回 0.04）
- **位置**：U3-import-review-end-390 与 bottom-390，备注两条都折成「Requested ui- / serif; preview uses Noto Serif」。
- **现状**：CSS 关键字被拆成两行。
- **期望**：作者值不应在内部断开。
- **修法**：和 UI-133 一起处理，`ui-serif` 用 `<code class="whitespace-nowrap">` 包起来。

## 4. 建议（不扣分）

R2 留下、本轮仍然适用的：
- JSON 导出按 2 空格缩进，代码框用 `white-space: pre` 加横向滚动。当前代码会在 token 名的连字符处折断，例如「accent- / foreground」「IBMPlexSans- / Text.ttf」。
- Rust 页底部的「Download」改名为「Download theme.rs」，与上方「Download matching JSON」区分。
- 「0.2.1 SDK/CLI reject v2」单独成一行警示。
- 差异表的颜色值旁加色块。
- Apply / Cancel 吸底。1440 滚到 Apply 后，标题和 × 已滚出视野（U3-import-review-end-1440），可以考虑让头部也固定。
- Cancel 改为描边次级按钮。
- 损失框与备注框区分样式。现在两者都是 #f9fafb 底、#d0d5dd 边，「会丢失」的警示和普通备注长得一样。
- 补一条普通 Cargo 的构建命令。

本轮新增：
- 390 卡片把 Changed 徽标放到 token 名的右侧，每张卡省一行。现在 6 张卡大约多出 180px 纵向长度。
- 「To match this preview」后面用文字标出预览模式，例如「Light preview」，不只依靠代码参数 `ThemeMode::Light`。
- 初始帧对话框底部被截住的表头（U3-import-review-1440 y≈880）没有滚动阴影；可以给对话框本体加底部渐隐，提示下面还有内容。

## 5. 看不出的格子

- 全部 U3 帧都没有键盘焦点。G4 的焦点可见，以及 Output、Copy、Apply、Choose file 的 focus-visible，都无从判断。
- 悬停与按下态只在以下两处顺带出现，看不出系统性的状态：
  - U3-import-review-1440 / 390 中「Review changes」浅蓝底。
  - U3-export-copied-1440 中「Copy code」#2c7ef1。
- 以下 X1 状态没有拍到：
  - Cancel 之后当前主题与历史是否保留；
  - Apply 之后的结果；
  - 语法错误、超过 256 KiB、不支持的颜色输入；
  - Technical details 展开后的内容。
- 差异类型只出现了 Changed。Added / Removed 的呈现，以及配置项变化时是否标 Changed、是否计入 Changes (N)，样例里都没有出现（当前与导入的配置相同）。
- X5 的下载是否实际发起、复制失败或下载失败时是否保留可选文本与下载路径、「Download matching JSON」点击后的结果，都没有拍到。
- U3-export-rust 代码约第 16–24 行、第 41–49 行没有拍到，看不出 Rust 导出里有没有真正列出注释所说的「listed same-origin font files」。U3-export-rust-mid-1440-light 与顶部帧相同，不作证据。
- U3 的中文态没有拍到，G6 的中英文同步看不出。
- 390 的兼容 v1、CSS、Rust 导出不在采集矩阵内，没有评。

## 6. 总评

- **得分**：本次只评 U3，得 9.3 分，比 R2 的 8.35 明显提高。
- **已解决**：R2 的 19 条差距中，14 条已解决，5 条部分解决，没有回归。其中最重的 UI-125（差异表嵌套限高滚动）、UI-26（label 字重）、UI-126（对话框跳动）、UI-127（390 分组行）全部关闭。
- **主要失分**：
  - Import candidate 的「当前 → 导入」箭头没有图例，而且和 Version、Name 行的含义混在一起（UI-128 残留）。
  - 导出说明与字节计数的措辞、单位不一致（新-1、新-2）。
  - 若干 2–11px 的对齐与字级残差（新-3、新-4、UI-25、UI-130）。
- **结构性改动**：不需要。对话框结构、单列断点、差异表卡片模式和导出四种类型的信息架构都已符合 X1–X6。剩下的都是文案、格式化函数和 CSS 局部调整，照单修完后细节维度可以回到 0.9 以上。
