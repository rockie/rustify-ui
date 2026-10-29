# U4 Studio 本地库迁移与存储反馈 · R2 评审

按目标包「变更记录」所述的重建基线评审。依据只有 r2 的 U4 截图、capture.md 和 U4-actions.log，另对照 r1 的 U4 截图和 judge.md 中指定的行。颜色是从 PNG 取的像素值（magick），位置为 CSS 像素，DPR 1。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.6 | 2.3 | 2.5 | 0.4 | **7.8** |

R1 为 6.0（2.0 / 2.0 / 1.75 / 0.25）。

- **布局 2.6**：升级 Review 已改为宽 1022px（约 64rem）、上下居中的对话框，内容是结构化摘要加底部动作行。390 宽度下可用，「Host control」浮签不再出现。不给 3 分的原因有三个：
  - 待确认态和取消态有两只琥珀色 callout 上下叠放，内容重复，首屏完全看不到编辑控件；
  - 冲突态的「状态区 → 决策区」顺序与升级态相反；
  - 冲突面板约 620px 高，超出 sticky 头部下方约 586px 的可视高度。
- **视觉 2.3**：次级文字 #475467、错误色 #B42318 已符合 G3；警告、错误、成功三种色调也已用上。不给更高分的原因：
  - callout 标题是 13px 常规字重，比 14px 深色正文还弱；
  - 两类 callout 的正文一个 12px、一个 14px；
  - 编辑列有三种描边按钮外观；
  - 被流程阻止的命名保存用了错误红，成功态 callout 不着色。
- **交互与状态 2.5**：以下路径都能从截图读通——Review → 显式「Upgrade and save current draft」→ 成功宣告；取消 → 常驻仅内存提示并可「Review upgrade again」；配额失败时保留草稿、可重试、可导出；冲突时显示双方差异，可在 Load / Keep / Save copy 中选择，并有原文和下载；无锁状态在载入时就有常驻提示。不给 3 分的原因：
  - 升级成功后，绿色成功 callout 下方仍留着红色的「not saved earlier」，旁边没有就地动作；
  - 无锁态标题写的是「saving paused」，意思不对；
  - 没有待处理项时，Retry / Cancel / Save current draft 三个按钮看不出能不能点；
  - callout 内的推荐动作没有强调。
- **细节 0.4**：差异不少但都小，包括：复数写成「1 recent operations」、操作条目换行剩孤字、修订号没有标签、缺句号和行话、同一动作有两个名称、双分隔线、对话框值列错位 25px。

## 2. 上一轮差距复核

| 编号 | 结论 | 本轮依据 |
| --- | --- | --- |
| UI-01（U4 部分） | 已解决 | r2 所有 U4 帧（1440 左下、390 右下）都没有「Host control」浮签 |
| UI-45 | 已解决 | `U4-upgrade-review`：对话框依次显示 Library version v1 → v2、Original fingerprint、Saved themes retained (2) 表、Current draft to save、Preferences to save；「Complete migration JSON」折叠；底部有主按钮「Upgrade and save current draft」和「Download original data」「Cancel upgrade」 |
| UI-46 | 部分解决 | 警告（#FFFAEB / #FEDF89 / #93370D）、错误（#FEF3F2 / #FECDCA / #B42318）已分开；无锁状态载入时就常驻（`no-locks-initial`）。仍有两点：成功态只有标题是绿色，容器是中性灰；同一琥珀底上有两种标题色 → 见下方 UI-46 |
| UI-47 | 部分解决 | 正文 #475467、错误 #B42318、callout 边框均合规；空闲态 Retry / Cancel / Save current draft 仍用 #667085 文字和 #858F9E 边框 |
| UI-48 | 已解决 | 配额失败的异常原文收进了「Technical details」；callout 正文和命名动作行不再同句重复 |
| UI-49 | 已解决 | 命名动作单独一行：「Save “Retained name” · failed; retained for retry. Export remains available.」，与操作列表 #3 一致 |
| UI-50 | 部分解决 | callout 有了标题，但决策类 callout 的标题仍弱于正文；状态 callout 与升级 callout 内容重复 |
| UI-51 | 部分解决 | 对话框里有主次层级了；编辑列 callout 内的三个按钮同一样式，没有推荐项；整列仍有三种描边外观 |
| UI-52 | 已解决 | 版本行和差异放在决策按钮之前；原文按缩进格式化，不再在字符中间断行（`conflict-raw`） |
| UI-53 | 已解决 | Draft differences 显示 overrides.primary · Changed · This tab #123456 / Other tab #654321 |
| UI-54 | 已解决 | 显示「Library upgraded to v2 / Saved on this device / 2 saved themes retained; the original download remains available.」 |
| UI-55 | 已解决 | 取消后没有「Cancel upgrade」，改为「Review upgrade again」；被拦下的命名保存点名并加引号；「storage:」前缀和「resume saving」都已去掉 |
| UI-56 | 部分解决 | 「1 pending · 3 recent operations」与 #1–#3 一致，条目有编号、加了引号；仍有复数错误、孤字换行，失败项没有区分 |
| UI-57 | 部分解决 | 待确认和取消态的下载已在 callout 内；升级成功态的「Download original before upgrade」仍在成功 callout 外，名称也与其他状态不一致 |
| UI-58 | 已解决 | 占位词为「Choose」 |
| UI-59 | 已解决 | 「Review upgrade」/「Review upgrade again」 |
| UI-60 | 已解决 | 不再有悬挂的「·」 |
| UI-61 | 部分解决 | 大小写和前缀已修；无锁正文缺句号，「explicit / explicitly」这类行话还在 |
| UI-62 | 部分解决 | 0 项时按钮变灰，1 项时为蓝色（`quota-failure`）；但灰色样式不像 Redo 的禁用外观，看起来像可点 |
| UI-63 | 已解决 | 「No pending actions. Your draft stays in memory until you choose.」 |
| UI-64 | 已解决 | 原句已删；无锁态新出现了行话，见新-4 |

## 3. 差距清单

单元均为 U4。按严重度、再按可挽回分排序。

### 新-1 · [P2] 被流程阻止的命名保存显示为错误红，升级成功后仍残留，时态不对（交互、视觉、细节）

- **截图位置**：
  - `U4-upgrade-cancelled-1440-light`：y≈452–486，红字「Save “Memory only” · not saved; review the library upgrade before saving this named theme.」；
  - `U4-upgraded-1440-light`：y≈443–476，紧贴绿色「Library upgraded to v2」下方的红字「Save “Memory only” · not saved earlier. Use Save locally to try again.」；
  - `U4-no-locks-1440-light`：y≈485–518，「Save “Not persisted” · not saved earlier; local writing is unavailable…」。三处都是 #B42318，没有容器和就地动作。
- **现状 → 期望**：
  - 取消态的拦截是设计内的门控，不是写失败，却用了字段错误红（G3 的 #B42318 是字段错误色）；
  - 升级成功后，阻止原因已经消失，红字仍在，且只叫用户回到顶部的「Save locally」，用户会误以为升级只成功了一部分；
  - 无锁态刚刚尝试保存，却写「not saved earlier」。
  - 期望（S4 / S6）：命名库动作单独反馈，结果要真实；写失败时保留内存草稿、仍可导出，同时有可继续的动作（S1）。
- **修法**：
  - 命名动作反馈按类型分样式：被阻止的用警告样式，放进升级 callout 内一行，例如「“Memory only” isn’t saved yet — saving starts after you upgrade.」；真正的写失败（配额、无锁）才用错误样式，带 12px 图标和浅色容器 #FEF3F2 / #FECDCA；
  - 升级提交成功后，把被阻止的命名保存转成操作列表里的待处理项，并在成功 callout 内给「Save “Memory only” now」按钮；或者显示中性提示「“Memory only” is still unsaved · [Save now] [Dismiss]」，不再用红字；
  - 无锁文案改为现在时：「“Not persisted” couldn’t be saved: this browser can’t write locally. [Download current theme]」；
  - 中英文同步。
- **可挽回分**：0.35（交互 0.25、视觉 0.05、细节 0.05）

### UI-50 · [P2] 两只琥珀 callout 叠放并重复；决策类 callout 标题弱于正文，两类 callout 字号不一（布局、视觉）

- **截图位置**：
  - `U4-upgrade-pending-1440-light` y≈317–778：「Memory only · saving paused / Legacy draft restored in memory. Review the complete library before upgrading.」，紧接着「Review the library upgrade / … Review the complete library before saving…」。两次说「review the complete library」，两只 callout 合计约 460px，1440×900 下首屏没有任何编辑控件（预设和基础配置都在 900 以下；R1 的同一帧 y≈739 已能看到 Presets）。`U4-upgrade-cancelled` 同样如此，两处都出现「Memory only」和「reopening restores…」。
  - 字体：状态 callout 标题 13px 常规字重 #93370D、正文 12px #475467；决策 callout（升级、冲突）标题 13px 常规字重 #475467、正文 14px #1D2939（行高 21px）。决策 callout 的标题比正文还弱。
- **现状 → 期望**：期望每个状态一条常驻反馈加继续动作（S1）；label 为 13px / 600、body 14px / 1.5、caption 12px（G3）。
- **修法**：
  - 升级待确认或已取消时，把存储状态与升级说明合成一只 callout：标题「Library upgrade needs review · memory only」，正文一句，动作行为 Review upgrade、Download original data、Download current theme（取消态再加 Review upgrade again）；
  - 所有 callout 统一为：标题 13px / 600，正文 14px / 1.5 #1D2939，辅助说明 12px #475467；
  - 目标：1440×900 的待确认首屏在 callout 之后至少露出 Presets 标题和搜索框。
- **可挽回分**：0.3（布局 0.15、视觉 0.15）

### UI-51 · [P2] callout 内的推荐动作没有强调，编辑列有三种描边按钮（视觉、交互）

- **截图位置**：
  - `upgrade-pending` 中「Review upgrade / Download original data / Cancel upgrade」三颗按钮外观完全相同，都是 #8AB7F7 蓝边配 #475467 灰字，推荐下一步与取消同权重；
  - 同一列的 Import / Export / Retry（有待处理项时）是 #8AB7F7 蓝边配 #1570EF 蓝字；
  - 空闲态 Retry / Cancel pending writes 是 #858F9E 灰边配 #667085 灰字。
  - 冲突态三选一的同权重可以保留。
- **现状 → 期望**：S2 要求有显式的升级入口；G3 规定蓝色用于选中与焦点，壳层扁平。对话框里已经做到主次分明，编辑列应与之一致。
- **修法**：
  - callout 内的推荐动作（待确认态的 Review upgrade、取消态的 Review upgrade again）改用与「Save locally」同族的实心小号按钮；其余为描边按钮，文字色统一为 Import / Export 的蓝字；
  - 去掉「蓝边灰字」这一变体；
  - 「Cancel upgrade」降为 ghost 按钮。
- **可挽回分**：0.25（视觉 0.2、交互 0.05）

### 新-2 · [P3] 冲突面板过高，状态区排到决策区之后，与升级态顺序相反（布局、交互）

- **截图位置**：`U4-conflict-1440-light`、`-a3`、`U4-conflict-after-1440-light-a2`。冲突 callout 从 y≈331 开始约 620px 高；sticky 头部占 0–314，可视高度只剩约 586px，「Download displayed original」与之后的「Memory only · saving paused」状态 callout 要滚动才看得到（after-a2 中位于 y≈634–756）。差异项拆成四行竖排：路径 / Changed / 标签 / 值。升级态则是状态 callout 在上、决策 callout 在下。
- **现状 → 期望**：期望不论哪种情况，状态与可继续动作都可见（S1）；冲突的版本和决策要显式呈现（S5）。
- **修法**：
  - 定下固定顺序：存储状态 callout 永远紧接 sticky 头部，冲突或升级的决策面板放在其后；也可以把「Saving is paused until you choose」并进冲突 callout 的首句，删掉单独的状态 callout；
  - 差异行改成单行网格：路径 | This tab | Other tab，值用 12px 等宽字；
  - 「View stored text」和「Download displayed original」放在同一行页脚；
  - 目标：冲突面板不超过 520px，在 1440×900 下决策和下载同屏可见。
- **可挽回分**：0.15（布局 0.1、交互 0.05）

### 新-3 · [P3] 升级 Review 对话框：分节标题不像标题，值列错位，表格下方双线（视觉、布局、细节）

- **截图位置**：`U4-upgrade-review-1440-light`。
  - 「Saved themes retained (2)」「Current draft to save」「Preferences to save」都是 13px 常规 #475467，与「Name」「Language」等字段标签同一样式；
  - 值列起点：Library version 区 x≈359，草稿区和偏好区 x≈333，差 25px；
  - 主题表最后一行底线 y=350 与分节线 y=363 相距 13px，形成双线。
- **现状 → 期望**：G3 规定 label 为 13px / 600；期望把完整迁移结果清楚呈现（S3）。
- **修法**：
  - 分节标题用 13px / 600 #101828，上方留 16px；
  - 三个键值区共用 `grid-template-columns: 136px 1fr`；
  - 主题表 `tr:last-child { border-bottom: 0 }`，或者去掉其后的分节线。
- **可挽回分**：0.15（布局 0.05、视觉 0.05、细节 0.05）

### UI-46 · [P3] 状态变体的余项：成功态不着色，同一琥珀底用两种标题色（视觉）

- **截图位置**：
  - `U4-upgraded`：成功 callout 底色 #F9FAFB、边框 #D0D5DD，只有标题是 #067647；
  - 「Memory only · saving paused」标题为 #93370D，而同为琥珀底的「Review the library upgrade」「Choose which version to keep」「Memory only · upgrade cancelled」标题为 #475467。
- **现状 → 期望**：期望常驻反馈的语义一致（S1 / S6）。
- **修法**：
  - 成功态改为与警告、错误同构的着色容器：#ECFDF3 底、#ABEFC6 边、#067647 标题；
  - 决策类面板二选一：要么同用警告标题色 #93370D，要么改为中性容器（#F9FAFB / #D0D5DD，标题 #101828）。不要在琥珀底上配灰色标题。
- **可挽回分**：0.1（视觉）

### UI-62 · [P3] 没有待处理项时，Retry / Cancel / Save current draft 看起来像可点（交互、视觉）

- **截图位置**：`upgrade-pending` y≈830–890、`no-locks` y≈571–597。灰色描边白底按钮；壳层的禁用外观是 Redo 那种 #F2F4F7 底配 #98A2B3 字，两者不一致。
- **现状 → 期望**：期望本地写操作要么禁用，要么报真实失败（S6）。
- **修法**：0 项时收起这一行（只在展开操作列表时出现）；或者用 `disabled` 加壳层禁用样式，并用 `title` 或描述说明「Nothing to retry」。无锁态的「Save current draft」同样处理。
- **可挽回分**：0.1（交互 0.05、视觉 0.05）

### 新-4 · [P3] 无锁态文案：标题「saving paused」不准确，正文是行话且缺句号（交互、细节）

- **截图位置**：`U4-no-locks-initial` / `U4-no-locks`，y≈369–414：「Memory only · saving paused / Secure Web Locks unavailable; memory editing and downloads only」。
- **现状 → 期望**：本次会话中本地写入是不可用，不是暂停；期望明确告诉用户「仅内存编辑 / 可下载」（S6）；新增文案中英同步（G6）。
- **修法**：标题改为「Memory only · local saving unavailable」；正文改为「This browser can’t safely coordinate saves between tabs, so changes stay in this tab. Download or export to keep them.」；补句号，中文同步。
- **可挽回分**：0.1（交互 0.05、细节 0.05）

### UI-56 · [P3] 操作列表的余项（细节）

- **截图位置**：
  - `upgrade-cancelled`、`no-locks`：「0 pending · 1 recent operations」，复数错误；
  - `quota-operations` y≈750–768：「#3 · Save “Retained name” · failed; retained · edit」后换行只剩「2」；
  - 失败项与「confirmed」项同色，无法区分；
  - 配额帧的说明文字带着与当前情境无关的「Load other version discards all pending actions」。
- **修法**：
  - 单复数按计数选择；
  - 「edit 2」包进 `white-space: nowrap`，或把编辑序号移到行首；
  - 失败项前加「Failed」标签（#B42318，12px / 600）；
  - 说明文字按当前情境只显示相关的句子。
- **可挽回分**：0.1（细节）

### UI-57 · [P3] 升级成功态的原始数据下载仍在 callout 外，同一动作有两个名称（布局、细节）

- **截图位置**：`U4-upgraded` y≈490–517 的「Download original before upgrade」位于成功 callout 外，中间隔着红字；而待确认态、取消态和对话框里都叫「Download original data」。
- **现状 → 期望**：期望下载原始数据的出口始终可达（S1 / S3），且同一动作同一名称。
- **修法**：把它放进成功 callout 作为动作，全站统一叫「Download original data」。
- **可挽回分**：0.1（布局 0.05、细节 0.05）

### 新-5 · [P3] sticky 头部下方的双分隔线（视觉、细节）

- **截图位置**：所有 U4 1440 帧。头部底线 y=282（升级态，x 0–318）或 y=314（冲突、配额、无锁态），17px 下方又有一条 x 20–298 的线（y=299 / 331），两条都是 #EAECF0。
- **现状 → 期望**：期望壳层用细分隔线和分组行（G2），不出现两道线。
- **修法**：当存储区紧接 sticky 头部时去掉它自身的顶部分隔线（例如 `:first-child { border-top: 0 }`），保留头部的 border-bottom。
- **可挽回分**：0.05

### 新-6 · [P3] 冲突原文区没有容器，看不出可以滚动（视觉）

- **截图位置**：`U4-conflict-raw-1440-light`。只读文本框直接压在琥珀底上，没有边框和底色，只有右下角的 resize 拖柄，内容在「"name": "Rustify Light",」处被截断。
- **修法**：给原文区白底、1px #EAECF0 边、6px 圆角、8px 12px 内边距、`max-height: 240px; overflow: auto`，用等宽 12px（G3 规定颜色值等宽 12px）。
- **可挽回分**：0.05

### 新-7 · [P3] 冲突版本行的修订号没有标签（细节）

- **截图位置**：`U4-conflict-1440-light` y≈438–465：「This tab 2 · Rustify Light」「Other tab 3 · Rustify Light」，读者不知道「2」「3」指什么。
- **修法**：改为「Revision 2 · Rustify Light」。
- **可挽回分**：0.05

### UI-47 · [P3] 空闲态按钮的颜色不是 G3 token（视觉）

- **截图位置**：Retry / Cancel / Save current draft 空闲态文字 #667085、边框 #858F9E。
- **现状 → 期望**：G3 规定次级文字 #475467、壳层分隔 #D0D5DD / #EAECF0。
- **修法**：如果保留可用的中性样式，改为 #475467 字、#D0D5DD 边；如果是禁用，按 UI-62 使用壳层禁用样式。
- **可挽回分**：0.05

### UI-61 · [P3] 剩余的行话（细节）

- **截图位置**：
  - 对话框里的「Ready for explicit upgrade confirmation.」；
  - 操作说明「Retry explicitly approves retained actions…」。
- **修法**：分别改为「Nothing is saved until you choose Upgrade and save current draft.」和「Retry saves the retained actions in their original order.」，中英同步。
- **可挽回分**：0.05

**可挽回分合计**：0.35 + 0.3 + 0.25 + 0.15 + 0.15 + 0.1 × 5 + 0.05 × 5 = **1.95**。10 − 7.8 = 2.2，差 0.25，在允许误差内。

## 4. 建议（不扣分）

- 冲突差异的 #123456 / #654321 前加 12px 色块，比对更直观。
- 升级 Review 的主题表可以加列头（Name / ID / Result），三列定宽。现在 ID 列在 x≈673，状态右对齐到 x≈1207，视线要跨很大的空白。也可以为每个主题列出迁移诊断数。
- 无锁态载入时，顶部「Save locally」可以标注「memory only」或直接禁用。S6 允许点击后报真实失败，所以不扣分。
- 头部「Matches preset」与对话框的「Draft state: Matches saved document」比较的对象不同，可以统一措辞，或加悬停说明。
- 「Restore document · Rustify Light」单独占一行，把 sticky 头部从 282px 撑到 314px，挤压了 U4 的可视高度。这属于 U1 范围，转交 U1 处理。

## 5. 看不出的格子

- **键盘与焦点**：没有 U4 键盘帧。`U4-upgrade-review` 登记焦点在 `upgrade-review-close`，但放大后 × 周围没有焦点环。对话框是鼠标点击打开的，`:focus-visible` 本来就可能不显示，所以看不出焦点可见性、焦点陷阱，以及关闭后焦点回到哪里。
- **中文**：没有 U4 中文帧，G6 中英同步无法核对。
- **390 下的对话框**：390 只拍了待确认态，看不出升级 Review 在 390 下的排布与滚动。
- **S4 保存状态细分**：没有拍到「排队」和「先前快照成功、当前仍待保存」两态。
- **S2 绕过确认**：看不出模式或偏好变化能否绕过升级确认。
- **S3 提交中间态**：日志显示从点确认到截图隔了约 2 秒，看不出提交成功前有没有提前显示成功。
- **折叠内容**：「Technical details」（配额）和「Cache and original backups」全程折叠，看不到里面的内容。
- **空闲按钮的真实状态**：只能看出视觉样式不同，看不出这三颗按钮实际是否禁用。
- **S5 冲突时的待处理命名动作**：本次冲突帧里待处理为 0，看不出未处理的命名动作是否保持可见。
- **S1 其他状态**：缓存损坏、只读持久化、导入失败、复制失败这几态不在冻结矩阵内，没有拍。
- **悬停与按下态**：没有拍。
- **不作证据的帧**：按 capture.md 说明，`U4-conflict-detail-1440-light-a2` 与 `U4-conflict-after-1440-light`（不带后缀）不作证据；`U4-conflict-detail-1440-light` 与 `U4-conflict-1440-light`、`U4-upgrade-review-end` 与 `U4-upgrade-review` 经 magick 比对逐像素相同，后者没有带来额外信息。

## 6. 总评

U4 从 6.0 升到 7.8。R1 唯一的 P1（UI-45：窄列里塞原始 JSON）已经改成规范要求的结构化迁移对话框；冲突差异、升级成功宣告、命名动作单独反馈、无锁时的常驻提示都已到位，浮签也消失了。剩下的问题没有阻断，集中在三处：

1. **反馈层级与措辞（新-1）**：被阻止和真正失败没有区分，都用错误红；升级成功后残留的红字与成功提示自相矛盾。这是本单元最显眼的残留问题。
2. **callout 结构（UI-50、UI-46、新-2）**：待确认和取消态的两只 callout 要合并成一只；状态区与决策区要固定顺序；标题与正文的字号字重要统一。这需要重组 callout 结构，不是只调颜色。
3. **按钮体系（UI-51、UI-62、UI-47）**：编辑列要收敛到实心、描边、禁用三种外观，并强调推荐动作。

以上做完，再把对话框对齐和细节文案收干净，U4 可以到 9 分以上。
