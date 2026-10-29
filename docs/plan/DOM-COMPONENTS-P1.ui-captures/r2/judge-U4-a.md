# U4 Studio 本地库迁移与存储反馈 · R2 评审（重建基线）

评审依据：目标包规范要点 S1–S6、G1–G4；本轮 `r2/U4-*.png` 共 13 张、`capture.md`、`U4-actions.log`；上一轮 `r1/U4-*.png` 与 `r1/judge.md` 中指定的行。
测量方法：用 ImageMagick 对截图逐像素取色、放大裁切。对照了三组疑似相同的帧：`U4-conflict` 与 `U4-conflict-detail` 像素差为 0，`U4-upgrade-review` 与 `U4-upgrade-review-end` 像素差为 0，两组完全相同；`U4-no-locks-initial` 与 `U4-no-locks` 有差异。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.4 | 2.3 | 2.4 | 0.45 | **7.55** |

- **布局 2.4**：升级 Review 已改为对话框，摘要按结构展示；冲突面板先给版本与差异，再给决策；浮签不再遮挡。不到 3 分的原因有三处：
  - 状态 callout 与决策 callout 叠放、内容重复，两者的上下顺序在升级和冲突中正好相反；
  - 命名动作的红字夹在 callout 之间；
  - 对话框里两组键值的值列没有对齐。
- **视觉 2.3**：有了警告琥珀、错误红、成功绿三种语义变体，次级文字与错误色已换成 G3 token。不到 3 分的原因：
  - 决策 callout 的标题和对话框分节标题是 13px 常规体 #475467，比 14px #1D2939 的正文还弱；
  - 两个并排 callout 的正文一个 12px、一个 14px；
  - 按钮仍有四套描边/禁用样式；
  - 命名动作反馈是不在任何容器里的红字。
- **交互与状态 2.4**：七个状态都能走通，每个状态都有常驻反馈、继续动作和下载出口（S1–S6 的主干都满足）。不到 3 分的原因：
  - 冲突的三个决策都没说明后果；
  - 升级成功后，过期的红色命名失败仍挂着，且没有内联动作；
  - 无锁态标题写「paused」，主按钮仍是实心可点；
  - 取消后的 callout 沿用了待确认时的文案；
  - 页头「Modified / Matches preset」与存储状态的基准不一致。
- **细节 0.45**：仍有约十处文案与像素问题，包括「1 recent operations」单复数错误、「edit 2」折行孤字、缺句号、时态错误、修订号没有标签、对话框双细线等。

## 2. 上一轮差距复核

| 编号 | 上一轮内容 | 本轮状态 | 依据（R2 截图） |
| --- | --- | --- | --- |
| UI-01（U4 部分） | Host control 浮签遮挡 | **已解决** | 1440 各帧左下（x≈22–97, y≈868–890）显示正常内容，pending 帧此处是「Cancel pending writes」；390 帧右下没有浮签 |
| UI-45 | Review 塞在窄列、内容是原始 JSON | **已解决** | `U4-upgrade-review`：对话框 x209–1231、y77–822。依次是版本/指纹、命名主题表、当前草稿、偏好；JSON 折叠在「Complete migration JSON」里；底部动作行为「Upgrade and save current draft / Download original data / Cancel upgrade」。剩余的栅格问题记为新-5 |
| UI-46 | 存储状态没有语义变体 | **已解决** | 警告 #FFFAEB 底 / #FEDF89 边 / #93370D 标题；错误 #FEF3F2 / #FECDCA / #B42318；成功是 #F9FAFB 底加 #067647 标题；无锁在载入时就有常驻 callout（`U4-no-locks-initial`）。命名动作反馈仍是裸红字，另记为新-2 |
| UI-47 | 次级文字、错误色、边框不是 G3 token | **部分解决** | 次级文字已是 #475467，错误已是 #B42318。禁用的存储操作按钮仍是 #667085 字、#858F9E 边，而且与 Undo 的禁用样式不同 |
| UI-48 | 配额失败灰字红字重复、异常原文直接拼接 | **已解决** | `U4-quota-failure`：只有一个红 callout，异常折叠进「Technical details」 |
| UI-49 | 命名动作反馈混在通用状态里、与操作列表矛盾、没说仍可导出 | **已解决** | 单独一行 `Save “Retained name” · failed; retained for retry. Export remains available.`，与「1 pending」一致，Retry 变为蓝色可用 |
| UI-50 | callout 无标题、状态行与首句重复、Review 标题弱 | **部分解决** | 已有标题，对话框标题 18px。决策 callout 标题和对话框分节标题仍比正文弱，两个 callout 正文字号不一 |
| UI-51 | 同级动作混用三种按钮、推荐下一步最弱 | **部分解决** | 链接与大小混用已消除。推荐步骤与 Cancel 同权；callout 按钮是灰字加蓝边，与壳层按钮（蓝字蓝边）不同 |
| UI-52 | 冲突先决策后版本、原文压缩 JSON | **已解决** | 版本表 y438–492 排在按钮 y746 之前；原文已格式化（`U4-conflict-raw`） |
| UI-53 | 冲突不显示双方差异 | **已解决** | 「Draft differences」中 overrides.primary 显示 This tab #123456 / Other tab #654321 |
| UI-54 | 升级成功只显示通用文案 | **已解决** | `U4-upgraded`：标题「Library upgraded to v2」，并写明 2 个主题已保留 |
| UI-55 | 取消后状态与文案过期 | **部分解决** | 「Cancel upgrade」、「No old bytes」、「storage:」都已去掉，失败文案点了主题名。但取消后的 callout 第一段原样沿用待确认时的文案 |
| UI-56 | 计数与条目不符、条目无法区分 | **已解决** | 「1 pending · 3 recent operations」与 #1–#3 一致，条目带 edit 序号，主题名加了引号 |
| UI-57 | 「Download original before upgrade」在 callout 外 | **仍存在** | `U4-upgraded` y490–517：按钮在成功 callout 外，中间还隔着一行红字 |
| UI-58 | 占位词小写「choose」 | **已解决** | `U4-upgraded` y855 显示「Choose」 |
| UI-59 | 「Review / recheck upgrade」 | **已解决** | 改为「Review upgrade」与「Review upgrade again」 |
| UI-60 | 「·」悬挂在行尾 | **已解决** | 版本行改为「2 · Rustify Light」 |
| UI-61 | 大小写和句号错乱、暴露「storage:」 | **部分解决** | 前缀已去掉。无锁 callout 正文缺句号 |
| UI-62 | 保留项为 0 时 Retry 仍可点 | **已解决（视觉）** | 0 项时是灰色 #667085，配额失败时是蓝色 #1570EF。是否真正不可点从图上看不出 |
| UI-63 | 冲突提示说有保留动作、计数却是 0 | **已解决** | 「No pending actions」与「0 pending」一致 |
| UI-64 | 「No old bytes…」行话 | **已解决** | 已消失 |

## 3. 差距清单（按严重度、再按可挽回分排序）

### 新-1 · [P2] 状态 callout 与决策 callout 叠放，文案重复，上下顺序随状态颠倒（布局、视觉、交互 · 可挽回 0.35）

**位置**
- `U4-upgrade-pending-1440`：
  - 上方「Memory only · saving paused」y316–439，正文「Legacy draft restored in memory. Review the complete library before upgrading.」；
  - 下方「Review the library upgrade」y452–778，正文「…Review the complete library before saving…」，同一意思说了两遍。
- `U4-upgrade-cancelled-1440`：
  - 上方正文「Upgrade cancelled. Memory-only editing; reopening restores the original legacy draft.」；
  - 下方标题「Memory only · upgrade cancelled」，正文「Reopening restores the original draft.」；
  - 「Memory only」与「reopening restores」各出现两次。
- `U4-conflict-raw-1440`：
  - 冲突 callout 在上，状态 callout「Saving paused while another tab's version awaits your decision. No pending writes.」在下（y610–733），中间隔一条 #EAECF0 分隔线；
  - 它与冲突 callout 内的「No pending actions. Your draft stays in memory until you choose.」（y706–724）重复；
  - 在冲突首帧 `U4-conflict` 中，这个状态 callout 位于首屏之外。
- 升级态是「状态在上、决策在下」，冲突态是「决策在上、状态在下」。

**现状 → 期望**：同一状态由两张卡片各说一半、互相重复，顺序也不固定 → 每个异常状态只有一处常驻反馈，并紧跟可继续的动作（S1）。取消后常驻「仅内存编辑」与重新检查入口（S2），冲突显式决策（S5）。

**修法**
- 存储区固定为一个 slot 序列：①状态与决策 callout（每个状态只有一张）→ ②命名动作反馈（见新-2）→ ③「N pending · N recent operations」→ ④「Cache and original backups」。
- 待确认：合并为一张琥珀 callout。标题「Memory only · upgrade pending」，正文一句（「Your legacy draft and 2 saved themes are open in memory. Nothing is written until you review and confirm.」），动作行为「Review upgrade」（主）、「Download current theme」、「Download original data」，最后是「Cancel upgrade」。删除单独的「saving paused」卡片。
- 取消后：同样合并成一张，正文只保留一句，动作为「Download current theme」（主）、「Review upgrade again」、「Download original data」。
- 冲突：「saving paused」并入冲突 callout 的标题（如「Saving paused · choose a version」），删除下方那张卡片和分隔线。

### 新-2 · [P2] 命名动作反馈是裸红字，位置不固定；升级成功后仍以红色错误挂着，且没有内联动作（视觉、交互、布局 · 可挽回 0.25）

**位置**
- `U4-upgrade-cancelled` y453–486：红字夹在两张琥珀 callout 之间。
- `U4-upgraded` y443–476：`Save “Memory only” · not saved earlier. Use Save locally to try again.`，夹在绿色成功 callout 与它的下载按钮之间。
- `U4-quota-failure` y537–570 与 `U4-no-locks` y485–518：同类红字，#B42318，无底色、无边框、无图标。

**现状 → 期望**：一行无容器的红字，与 callout 混排；成功之后的页面上仍出现红色错误，严重度信号互相冲突 → 命名库动作单独反馈（S4），与草稿保存状态分开，且「未处理的命名动作保持可见」（S5）。可见性要保留，但要有固定位置和语义容器。

**修法**
- 新建小号 inline alert（命名动作反馈），固定放在新-1 的 slot ②。
- 失败时用错误变体：#FEF3F2 底、#FECDCA 边、#B42318 标题，标题写「Not saved: “Memory only”」，正文一句原因。
- 附内联动作：「Save “Memory only” now」，直接以原名重试，不必重新输入名称；另加「Dismiss」。
- 升级成功后，这一行降为中性变体（#F9FAFB 底、#D0D5DD 边），文字改为「“Memory only” was not saved during the pending upgrade」，再加「Save now」。

### 新-3 · [P2] 冲突的三个决策没有说明后果（交互 · 可挽回 0.2）

**位置**：`U4-conflict-1440` y746–844 的「Load other version」「Keep current draft」「Save current as a copy」，三个按钮等权纵向堆叠，旁边没有说明。只有在配额帧展开的操作列表说明（`U4-quota-operations` y612–680）里，才提到「Load other version discards all pending actions」。

**现状 → 期望**：用户无法判断「Keep current draft」会不会在下次保存时覆盖修订 3，也无法判断「Load other version」会不会丢掉 #123456 → 显式决策、不静默覆盖或合并（S5），用户应能在点之前知道结果。

**修法**：每个按钮下加一行 12px #475467 说明，或改成单选组加「Apply」：
- Load other version：「Replace this tab's draft with revision 3. Your #123456 edit is discarded. Download first to keep it.」
- Keep current draft：「Keep this draft. Your next save replaces revision 3.」
- Save current as a copy：「Save this draft as a new named theme. Revision 3 stays.」

### UI-50 · [P2] 决策 callout 标题与对话框分节标题弱于正文；两种 callout 正文字号不一（视觉 · 可挽回 0.2）

**位置**
- 「Review the library upgrade」（pending y473）、「Memory only · upgrade cancelled」（cancelled y521）、「Choose which version to keep」（conflict y352）：都是 13px 常规体 #475467，下面的正文却是 14px #1D2939。
- 状态 callout 正文是 12px #475467（如 pending y363–381，行高约 18px），紧邻的决策 callout 正文是 14px（行高约 21px）。
- 对话框分节标题「Saved themes retained (2)」「Current draft to save」「Preferences to save」也是 13px 常规 #475467，比 14px #1D2939 的值还弱。

**现状 → 期望**：标题层级倒挂 → G3「label 13px / 600」；标题应强于正文。

**修法**
- 所有 callout 标题统一为 13px / 600，颜色按变体：警告 #93370D、错误 #B42318、成功 #067647、中性 #1D2939。
- 所有 callout 正文统一为 13px（或统一 14px），颜色 #475467，行高 1.5。
- 对话框分节标题改为 13px / 600、#1D2939。

### UI-51 · [P2] 推荐的下一步与次要/放弃动作同权；callout 按钮又是一套样式（视觉、交互 · 可挽回 0.2）

**位置**
- 推荐步骤与放弃动作同权：pending 的「Review upgrade」与「Cancel upgrade」（y667–765），cancelled 的「Review upgrade again」（y736），在视觉上与其他按钮完全相同。
- 左列同时存在四种按钮样式：
  - 壳层 Import/Export：蓝字 #1570EF、边框 #8AB7F7；
  - callout 内按钮：灰字 #475467、边框 #8AB7F7；
  - 禁用的存储操作：#667085 字、#858F9E 边、白底；
  - 禁用的 Undo/Redo：#F2F4F7 填充、#98A2B3 字。

**现状 → 期望**：用户看不出该先点哪个，左列视觉也显得杂乱 → G3 规定「蓝色用于选中与焦点」，壳层应扁平统一；S2 要求显式确认路径清晰。对话框内已有实心主按钮「Upgrade and save current draft」，可以作为参照。

**修法**
- 每张 callout 只设一个推荐动作，用壳层「Save locally」的实心小号样式。
- 其余动作统一用壳层 outline（#1570EF 字、#8AB7F7 边），不再用灰字蓝边。
- 「Cancel upgrade」这类放弃动作放在动作行末尾，改为文字按钮。

### 新-5 · [P3] 升级 Review 对话框栅格不齐：值列错位、主题表无表头、双细线（布局、细节 · 可挽回 0.15）

**位置**：`U4-upgrade-review-1440`
- 「Library version / Original fingerprint」的值起点 x≈360，而「Name / Preview mode / Draft state」和偏好的值起点 x≈333，同一对话框里有两条值列。
- 命名主题表三列（名称 x233、ID x≈673、「Retained as v2」右对齐 x≈1207）没有表头，ID 列悬在中间。
- 表末行底线 y≈350 与分节线 y≈363 相距 13px，形成双细线。

**现状 → 期望**：结构化摘要应便于读取（S3），壳层用细分隔线与分组行（G2）。

**修法**
- 所有键值块共用一个 `grid-template-columns: 9rem 1fr`。
- 主题表加表头行「Name | ID | Result」，列宽设为 `1fr 12rem auto`。
- 去掉末行 border-bottom，或去掉下一节的 border-top，只保留一条 #EAECF0。

### 新-7 · [P3] 无锁态：标题说「paused」，主按钮仍是实心可点，失败文案时态错误（交互、细节 · 可挽回 0.15）

**位置**
- `U4-no-locks-initial` y369：标题「Memory only · saving paused」。无锁在整个会话里都不会恢复，「paused」暗示暂停后会自动恢复。
- 页头「Save locally」（y240–266）仍是实心 #1570EF。
- `U4-no-locks` y485–518：刚刚尝试的动作被写成「not saved earlier」。

**现状 → 期望**：S6 要求明确「仅内存编辑 / 可下载」；本地写操作要么禁用，要么报真实失败。现在报的是真实失败，但表述误导。

**修法**
- 标题改为「Memory only · local saving unavailable」。
- 「Save locally」降为 outline 并附 `title`/说明「Unavailable in this tab」，或保持可点但不再用主色。
- 失败文案改为「Save “Not persisted” · not saved: local storage is unavailable in this tab. Download or export a copy.」

### 新-10 · [P3] 文案细节（细节 · 可挽回 0.15）

- 单复数错误：「0 pending · 1 recent operations」，见 `U4-upgrade-cancelled` y840 与 `U4-no-locks` y548。改为「1 recent operation」。
- `U4-upgraded` y389–407 的「the original download remains available」语义含混。改为「Your original v1 data can still be downloaded.」
- 对话框 y719 的「Ready for explicit upgrade confirmation.」是实现腔调。可删除，或改为「Nothing is saved until you confirm.」
- `U4-conflict-raw` y537 的「Download displayed original」含混。改为「Download other tab's version (JSON)」。
- 冲突版本行：
  - 「This tab 2 · Rustify Light」中的修订号没有标签，改为「Revision 2 · Rustify Light」；
  - 「Saved themes 0 → 0」箭头含义不明、无变化时是噪音，无变化就隐藏，有变化写「0 (this tab) / 1 (other tab)」。
- 「Secure Web Locks unavailable」是技术名词。改为「This browser can't coordinate saves between tabs」。

### UI-47 · [P3] 禁用的存储操作按钮用了非 token 色，且与另一套禁用样式并存（视觉 · 可挽回 0.1）

**位置**：`U4-upgrade-pending` y829–891、`U4-upgraded` y568–630、`U4-no-locks-initial` y522–548：#667085 字、#858F9E 边、白底，看起来像可用的中性按钮。同一列的 Undo/Redo 禁用态却是 #F2F4F7 填充、#98A2B3 字。

**期望**：G3 token（次级文字 #475467、边框 #D0D5DD/#98A2B3），禁用样式全壳只有一套。

**修法**：存储操作按钮的禁用态复用 Undo 的禁用 token（填充 #F2F4F7、字 #98A2B3），同时设 `disabled` / `aria-disabled`。

### UI-55 · [P3] 取消后的 callout 第一段沿用了待确认时的文案（交互、细节 · 可挽回 0.1）

**位置**：`U4-upgrade-cancelled` y553–637 的「Your previous draft and saved themes are available for editing. Review the complete library before saving this browser's upgraded copy; the original data stays unchanged.」，与 pending 帧 y505–589 逐字相同。

**期望**：取消后常驻「仅内存编辑」与重新检查入口（S2），文案应说明当前状态，而不是升级前的说明。

**修法**：这一段换成取消态专用的一句话（并入新-1 的单卡片），例如「Upgrade cancelled. Edits stay in this tab only; reloading restores the original legacy draft.」

### UI-57 · [P3] 升级成功后，「Download original before upgrade」在成功 callout 之外（布局 · 可挽回 0.1）

**位置**：`U4-upgraded` y490–517。按钮位于成功 callout（y316–429）下方，中间还隔着红字。

**期望**：提供「下载升级前原始数据」（S3），并与对应的状态放在一起。

**修法**：把按钮放进「Library upgraded to v2」callout 的动作行，文案改为「Download original v1 data」。

### 新-6 · [P3] 冲突差异块纵向松散，配色混搭（布局、视觉 · 可挽回 0.1）

**位置**：`U4-conflict-1440`
- 「overrides.primary」y577、「Changed」y613、「This tab / Other tab」y648、值 y666：单格内容各占一行，行距约 36px。
- 「Draft differences」y513 与上方「Saved themes」y492 只差约 21px，而其他行距约 27px。
- 组头带「Light · overrides (1)」是中性 #F9FAFB 底，放在琥珀 #FFFAEB 的 callout 里。

**期望**：展示冲突版本（S5）。壳层用分组行，而不是零散单元格（G2）。

**修法**
- 每条变更压成两行：第一行「overrides.primary · Changed」；第二行两列「This tab #123456 | Other tab #654321」。行距 8px。
- 「Draft differences」上方留 12px。
- 组头带改用透明底加 #EAECF0 底线，或用琥珀浅一阶的底色。

### 新-9 · [P3] 操作列表的失败条目没有区分；「edit 2」折行；说明段答非所问（视觉、细节 · 可挽回 0.1）

**位置**：`U4-quota-operations`
- y750–768：「#3 · Save “Retained name” · failed; retained · edit」后，「2」单独折到下一行。
- 失败条目与 #1、#2 的「confirmed」同为 #1D2939 常规体。
- y612–680 的说明段在配额场景里提到「Load other version」。

**期望**：命名动作的结果要清楚（S4），保存失败不能看起来像已保存（S1）。

**修法**
- 每行末尾加状态 chip：「Confirmed」用 #475467；「Failed · retained」用 #B42318 文字加 #FEF3F2 底。
- 「edit N」用 `white-space: nowrap`。
- 说明段按当前状态显示，只在冲突时提 Load other version。

### 新-12 · [P3] 页头草稿状态与存储状态的基准不一致（交互、细节 · 可挽回 0.1）

**位置**
- `U4-upgrade-pending` 页头 y104 写「旧草稿 Matches preset」，而对话框 y470 写「Draft state: Matches saved document」，描述的是同一个草稿。
- `U4-upgraded` 页头 y104 写「Modified」，紧挨着 y363 的「Saved on this device」，容易读成「有未保存修改」。

**期望**：S4 要求能区分「当前草稿确已保存」。

**修法**：页头标签写明基准，如「Modified from preset」，或在旁边并列「Saved」/「Unsaved」两个独立标识；对话框里的「Draft state」用相同措辞。

### UI-61 · [P3] 句末标点不一致（细节 · 可挽回 0.05）

**位置**：`U4-no-locks-initial` y395–413 的「…memory editing and downloads only」没有句号，其余 callout 正文都以句号结尾。

**修法**：补句号（文案可按新-10 一起改写）。

### 新-11 · [P3] 冲突原文 textarea 无边框、透明底，截断处没有滚动提示（布局 · 可挽回 0.05）

**位置**：`U4-conflict-raw` y355–515。textarea 背景与 callout 同为 #FFFAEB，四边取色都没有边框，只有右下角的 resize 柄；内容截在 `"name": "Rustify Light",`。

**修法**：`background:#fff; border:1px solid #D0D5DD; border-radius:6px; max-height:12rem; overflow:auto;`，等宽 12px。

可挽回分合计：0.35 + 0.25 + 0.2 + 0.2 + 0.2 + 0.15 + 0.15 + 0.15 + 0.1 × 6 + 0.05 × 2 = **2.35**。10 − 7.55 = 2.45，差 0.1，在 0.5 误差内。

## 4. 建议（不扣分）

- 冲突差异与命名主题表里的颜色值旁加 12px 色块，便于直接比较 #123456 和 #654321。
- 对话框里「×」与「Cancel upgrade」容易被当成同一个动作。可以给 × 加可见说明「Close – decide later」，或去掉 ×，只保留「Not now」。
- 对话框「Current draft to save」下的「Preview mode」更适合移到「Preferences to save」，避免与作者数据混淆。
- 升级成功的 callout 可加「Dismiss」，避免它长期占据存储区顶部。
- 跨单元，不计入 U4：
  - 编辑列顶部有双细线：1440 pending 帧 y282（页头底线 #EAECF0）与 y298（分节线 #EAECF0），相距 16px，属于 U1 壳层；
  - 预览诊断里的「1 text pairs below 4.5:1」单复数有误，属于 U2/F3。

## 5. 看不出的格子

- **`U4-conflict-detail-1440`**：与 `U4-conflict-1440` 像素差为 0，没有提供额外信息。冲突 callout 未展开前的底部（y≈890 起那颗被截断的按钮），以及它下方状态区的首帧样子，在冲突首帧里都看不到；只有展开原文后的 `U4-conflict-raw` 能看到。
- **`U4-upgrade-review-end-1440`**：与 `U4-upgrade-review-1440` 像素差为 0。对话框在 900 高度内不需要滚动，所以视口不足时的内部滚动与动作行可见性没有验证。
- **键盘与焦点**：U4 没有任何键盘帧。`capture.md` 登记打开 Review 后焦点在 `upgrade-review-close`，但画面上看不到焦点环；对话框是用鼠标打开的，`:focus-visible` 不一定触发。焦点陷阱、Esc 关闭、关闭后焦点回到哪里，callout 按钮的焦点帧（G4），都看不出。
- **提交中状态**：从点击「Upgrade and save current draft」到拍 `U4-upgraded` 隔了约 2 秒，看不出提交过程中是否没有提前显示成功（S3）。
- **S4 排队状态**：「排队中」与「先前快照成功而当前仍待保存」两种状态没有采到。
- **S2 模式/偏好变化**：没有采到模式或偏好变化时能否绕过确认。
- **配额失败的技术细节**：`U4-quota-failure` 中「Technical details」没有展开，异常原文的呈现看不出。
- **「Cache and original backups」**：在所有帧中都没有展开，看不出里面的内容。
- **中文态**：U4 新增的迁移与存储文案（G6）没有中文帧，看不出中英文是否同步。
- **禁用是否真实**：只能从颜色推断 0 项时 Retry 已禁用，看不出它实际能否点击。

## 6. 总评

U4 本轮 **7.55**，上一轮 6.0。上一轮最重的结构问题都已解决：Review 改为对话框并给出结构化摘要，存储状态有了语义变体，冲突面板给出版本与差异，升级成功有了宣告，浮签不再遮挡。本轮没有 P0 或 P1。

主要失分点：

1. **存储区的信息结构还要再改一次**（新-1、新-2）。同一状态拆成两张 callout，互相重复、顺序不固定；命名动作反馈是位置随状态漂移的裸红字。需要把存储区改成固定 slot（状态与决策卡 → 命名动作反馈 → 操作 → 备份），每个状态只用一张卡片，命名动作反馈改为有变体的 inline alert。这是结构性调整，不是调样式能解决的。
2. **层级与按钮体系**（UI-50、UI-51、UI-47）。标题比正文弱、正文字号不一、按钮有四套样式。统一 callout 的标题与正文规格，每张卡只留一个主动作，全壳只用一套 outline 和一套禁用样式。
3. **冲突决策的后果说明**（新-3）。这是剩下的交互缺口里最影响理解的一条。

其余都是 P3 级的栅格、文案与像素问题（新-5、新-6、新-7、新-9 至新-12、UI-55、UI-57、UI-61），可以照单逐条修。
