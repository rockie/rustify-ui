# U4 评审 · R3（Studio 本地库迁移与存储反馈）

评审依据只有以下材料：目标包、r3 和 r2 中以 `U4-` 开头的截图、capture.md、U4-actions.log，以及 `r3/judge-inputs/r2-judge-U4.md`。r3 目录里其他评审文件（judge-U4-a 等）没有打开。像素值都是在 DPR 1 原图上用 ImageMagick 取样得到的。字号由字形高度、行距，以及与 R2 中同一句英文（已知 14px）的宽度比（0.92）推算。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.75 | 2.7 | 2.65 | 0.6 | **8.7**（R2：7.55） |

**布局 2.75**
- 现状：待确认、取消、冲突三个状态都已合并成一张 callout。冲突面板在 1440×900 下完整可见（y244–790）。升级对话框两组键值的值列都对齐在 x=405，主题表有了表头。
- 不给 3 的原因：
  - 冲突 callout 所在位置与其他状态不同。其他状态是分隔线在上；冲突态没有上方分隔线，分隔线改到了下方 y=806。
  - 配额态的「Technical details」脱离了错误 callout。
  - 冲突版本值在 x=157 处被折成两行（「Rustify / Light」）。
  - 对话框表格末行下方仍是双细线，差异条目仍占 4 行。

**视觉 2.7**
- 现状：标题已改为 13px/600 并按变体着色（#93370D、#B42318、#067647）。成功态容器着色为 #ECFDF3/#ABEFC6。禁用样式统一为 #98A2B3/#F9FAFB/#EAECF0，与 Undo 一致。原文框有了边框。
- 不给 3 的原因：
  - callout 正文仍有两套规格：12px #475467 和 13px #1D2939。相邻两张卡片也会不同，而且 13px 常规字重不在 G3 的字级上。
  - 对话框描边按钮是蓝边蓝字，壳层描边按钮是灰边灰字，两套样式并存。

**交互与状态 2.65**
- 现状：七个状态都走得通。每个状态都有常驻反馈、继续动作和下载出口。冲突的三个决策有了后果说明。命名动作改为带 Dismiss 的内联提示，升级后还提供「Save now」。无锁态的标题、按钮和文案已经一致。
- 不给 3 的原因：
  - 配额态的命名失败卡片说「retained for retry」，但卡片里没有重试按钮。
  - 取消后的文案要求「Download your current theme」，callout 里却没有对应按钮。这是相对 R2 的回归。
  - 处于「Saving paused」的状态时，页头的「Save locally」仍是实心主按钮。
  - 页头基准写成「旧草稿 Matches “旧草稿”」，读起来像同义反复。

**细节 0.6**
- R2 列出的十处文案问题大多已修，但仍有若干小差异：
  - 取消 callout 中「reopening restores…」重复了两遍。
  - 升级后的文案时态不对，并指向页头按钮。
  - 「(draft / revision 2)」被断行。
  - 操作说明提到当前状态不存在的「Loading the other version」。
  - 「Saved theme count 0 → 0」。

## 2. 上一轮差距复核表

| 编号 | 结论 | 依据（R3 截图） |
| --- | --- | --- |
| UI-108 双细线 | 已解决（U4 帧） | 各 1440 帧在编辑列 x=250 纵向扫描：只有页头下边框 y=56 和分组线 y=244，原来相隔 17px 的第二条线已消失 |
| UI-114 页头基准与保存状态 | 部分解决 | 页头新增状态胶囊（Saving paused / Saved on this device / Save failed / Memory only），基准写明了名字，对话框「Draft state」用词一致。剩余问题：「旧草稿 Matches “旧草稿”」没有说明基准是已保存文档；取消态胶囊写「Saving paused」，而 callout 写「Memory only」 |
| UI-139 callout 叠放、重复、顺序 | 部分解决 | 待确认、取消、冲突各只剩一张 callout，冲突面板能放进一屏。剩余问题：取消 callout 内同一句话写了两遍；冲突面板所在位置与其他状态不同 |
| UI-140 命名动作裸红字 | 部分解决 | 已改为带容器的提示卡片：被阻止用琥珀色，真实失败用红色，升级后用中性色；有 Dismiss，升级后有「Save now」。剩余问题：配额卡片没有内联重试；升级后卡片的文案仍让用户去点页头「Save locally」 |
| UI-50 标题与正文字号 | 部分解决 | 标题和对话框分节标题已改为 13px/600。正文仍是 12px #475467 与 13px #1D2939 混用 |
| UI-51 推荐与放弃同权、多套描边 | 部分解决 | callout 有实心推荐动作，Cancel 改成了文字按钮，壳层描边统一为 #D0D5DD/#475467。剩余问题：对话框按钮是 #8AB7F7/#1570EF 蓝边蓝字，对话框里的「Cancel upgrade」仍是描边按钮；配额和无锁 callout 没有实心按钮 |
| UI-141 冲突决策无后果说明 | 已解决 | U4-conflict y527–573：「Load replaces… Keep saves… Save a copy retains…」 |
| UI-142 对话框栅格 | 部分解决 | 值列已对齐（x=405），主题表有了表头。y=309 的行线与 y=322 的分节线仍构成双细线 |
| UI-143 无锁态 | 已解决 | 标题「local saving unavailable」，「Save locally」改为描边，正文是白话并以句号结尾 |
| UI-144 文案细节 | 已解决 | 「Revision 2」有了标签；「Ready to upgrade after your confirmation.」；「Download other version」；「Local saving is unavailable in this browser.」。新出现的文案问题记为新-5 |
| UI-145 差异块松散 | 部分解决 | 组头已去掉灰底（y128 只有一条 #EAECF0 线）。一条差异仍占键、Changed、列标签、值四行（y147–247） |
| UI-146 原文框 | 已解决 | U4-conflict-raw：#F9FAFB 底、#D0D5DD 边框，有缩放手柄 |
| UI-46 成功态不着色 | 已解决 | #ECFDF3/#ABEFC6 容器，标题 #067647；琥珀标题统一为 #93370D |
| UI-47 非 token 禁用色 | 已解决 | Retry、Cancel、Restore、Undo 都是 #98A2B3 字、#F9FAFB 底、#EAECF0 边框 |
| UI-55 取消后沿用待确认文案 | 已解决 | 第一段改为「Upgrade cancelled. Memory-only editing…」。段内重复并入 UI-139 |
| UI-56 操作列表 | 部分解决 | 单复数已修，「edit 2」改为「draft revision 2」，成功和失败分别着色。剩余问题：「(draft / revision 2)」被断行，说明段提到冲突态才有的动作，展开内容没有缩进 |
| UI-57 升级后下载在 callout 外且改名 | 已解决 | U4-upgraded：「Download original data」在绿色 callout 内，名称与其他状态一致 |
| UI-61 标点与行话 | 已解决 | 无锁正文以句号结尾；操作说明改为「Retry saves the listed actions in order.」 |
| UI-62 空闲按钮看似可点 | 已解决 | 空闲时为禁用样式，配额态有待处理项时恢复为 #475467/#D0D5DD 描边 |

## 3. 差距清单

#### UI-50 · [P2] callout 正文有两套字号和颜色，且 13px 常规字重不在 G3 字级上（U4 · 视觉 · 部分解决 · 可挽回 0.15）

- **位置**
  - 13px #1D2939（行距约 19.5–20px）的地方：
    - U4-upgrade-pending y308–390；
    - U4-upgrade-cancelled y308–429；
    - U4-conflict 首段 y291–322；
    - 各命名动作卡片，如 U4-quota-failure y474–506、U4-no-locks y436–487。
  - 12px #475467（行距 18px）的地方：
    - U4-quota-failure 错误 callout y304–350；
    - U4-no-locks 状态 callout y304–350；
    - U4-upgraded 成功 callout y304–376；
    - U4-conflict 后两段 y479–573。
  - 典型冲突：U4-no-locks 中上下相邻的两张琥珀卡片，一张 12px 灰字，一张 13px 深字。
- **现状 → 期望**：同一类反馈容器内的正文字号和颜色随变体变化 → 全部落在 G3 字级上：body 14px/1.5，caption 12px，13px 只用于 label/600。
- **修法**：给存储 callout 组件定两条固定规则，四种变体（warning、error、success、neutral）和命名动作卡片共用：
  - 状态句（首段）用 body，14px/1.5 #1D2939；
  - 补充说明（后续段、「No pending actions…」、后果说明）用 caption，12px/1.5 #475467。
  - 删除 13px 常规正文。在 280px 列宽下如果要控制高度，就把首段压到一句话，不要缩小字号。

#### UI-51 · [P2] 对话框描边按钮与壳层是两套样式；同一个「Cancel upgrade」在 callout 和对话框中权重不同；配额和无锁 callout 没有推荐动作（U4 · 视觉、交互 · 部分解决 · 可挽回 0.15）

- **位置**
  - U4-upgrade-review y712–743：「Download original data」「Cancel upgrade」都是 #8AB7F7 边框、#1570EF 字。
  - 同屏页头的 Import/Export 和 callout 内的「Download original data」是 #D0D5DD 边框、#475467 字。
  - 待确认 callout 中的「Cancel upgrade」是无边框文字按钮（y501），对话框中却是描边按钮。
  - U4-quota-failure 与 U4-no-locks 的 callout 只有描边的「Download current theme」（y362–393）。
- **现状 → 期望**：S2 要求「显式升级并保存」与「取消」区分清楚；G3 要求壳层扁平、样式统一。
- **修法**
  - 对话框底栏改用壳层描边样式（白底、#D0D5DD 边框、#475467 字）。
  - 「Cancel upgrade」改为与 callout 一致的文字按钮，放在最右。
  - 无锁 callout 把「Download current theme」改为实心按钮，因为它是该状态唯一的推荐动作。
  - 配额 callout 增加实心「Retry save」，并把「Download current theme」放在其后，保持描边。

#### UI-139 · [P3] 取消 callout 内同一句话写了两遍；冲突面板所在位置与其他状态不同（U4 · 布局、细节 · 部分解决 · 可挽回 0.1）

- **位置**
  - U4-upgrade-cancelled y308–429：标题已是「Memory only · upgrade cancelled」，正文又写「Upgrade cancelled. Memory-only editing; reopening restores the original legacy draft.」，第二段又写「Reopening restores the original draft.」。
  - U4-conflict：callout 顶边紧贴 y=244（其他状态这里是分隔线，callout 从 y=261 开始），面板下方另多出一条 y=806 分隔线。U4-conflict-after 中对应 y=506。
- **现状 → 期望**：S1 要求每个状态只有一处反馈；存储区的位置顺序应固定：状态 callout → 命名动作 → 操作计数 → 备份。
- **修法**
  - 取消 callout 的正文改为一句：「Edits stay in memory only. Reloading restores the original legacy draft — download your current theme to keep them.」
  - 冲突面板放回与其他状态相同的位置：y=244 分隔线之后、上间距 17px，并删除面板下方那条分隔线。
  - 也可以反过来让所有状态都用「上线、下线」的结构，二者选一即可。

#### UI-140 · [P3] 配额态命名失败卡片没有内联重试；升级后卡片仍指向页头按钮（U4 · 交互 · 部分解决 · 可挽回 0.1）

- **位置**
  - U4-quota-failure y458–565：卡片写「failed; retained for retry. Export remains available.」，卡片内只有 Dismiss。重试按钮「Retry retained saves」在两块之下（y617）。
  - U4-upgraded y461–494：写「Use Save locally to try again.」，而卡片内就有「Save now “Memory only”」。
- **现状 → 期望**：S4 要求命名动作单独反馈；S1 要求有继续动作。
- **修法**
  - 配额卡片增加「Retry “Retained name”」，排在 Dismiss 前。它只重试这一条操作，与操作区的批量重试分开。
  - 升级后卡片的文案改为「“Memory only” was blocked before the upgrade and is not saved yet.」，删除对页头按钮的引用。

#### 新-1 · [P3] 冲突面板值列太窄，版本值被拆成两行（U4 · 布局、细节 · 可挽回 0.1）

- **位置**：U4-conflict y342–416。「Revision 2 · Rustify / Light」「Revision 3 · Rustify / Light」各占两行。键列被「Saved theme count」撑到约 124px，值列从 x=157 开始，只剩约 128px。「Draft differences」紧贴上一行（y428 → y447），与段间 20px 的节奏不一致。
- **现状 → 期望**：S5 要求清楚展示冲突版本；一个版本标识不应在名称中间断开。
- **修法**
  - 键值改为上下堆叠：键 12px #475467 在上，值 13px/600 #1D2939 在下。
  - 或者每个值分两行：「Rustify Light」一行，「Revision 2」以 caption 另起一行。
  - 把「Saved theme count」缩短为「Saved themes」。
  - 键值块与「Draft differences」之间留 8–12px。

#### 新-3 · [P3] 取消后 callout 要求下载当前主题，却没有对应按钮（U4 · 交互 · 相对 R2 回归 · 可挽回 0.1）

- **位置**：U4-upgrade-cancelled y378–429 写「Download your current theme to keep these memory-only edits.」，按钮只有「Review upgrade again」和「Download original data」。R2 同一状态有「Download current theme」。
- **现状 → 期望**：S1 要求当前主题的出口始终可达，并且与提示对应；配额和无锁 callout 都放了这个按钮。
- **修法**：在「Download original data」前加描边按钮「Download current theme」，复用配额和无锁 callout 的同一动作。

#### 新-4 · [P3] 处于「Saving paused」的状态时，页头「Save locally」仍是实心主按钮（U4 · 交互 · 可挽回 0.1）

- **位置**
  - U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 页头 x1195–1284：实心 #1570EF，紧挨着「Saving paused」胶囊。
  - 同屏 callout 也有实心按钮（Review upgrade、Keep current draft），两个主操作同时争夺注意力。
  - 日志 15:19:38.753 显示，此时命名保存会被拒绝：「not saved; review the library upgrade…」。
  - U4-no-locks 已把同一按钮改为描边。
- **现状 → 期望**：S2 要求升级确认前的主路径是 Review → 确认，不应诱导用户去点一个必定被拒绝的保存。
- **修法**：存储状态为 paused 或 memory-only 时，页头「Save locally」与无锁态一样渲染为描边按钮，只在保存会被真正接受时才用实心。

#### 新-5 · [P3] 新出现的文案问题（U4 · 细节 · 可挽回 0.1）

- **位置**
  - U4-upgraded y330–376：「Use “Download original data” to back up the library before upgrade.」升级已经发生，这句读起来却像要求在升级前备份，而且缺冠词。
  - 同一 callout 的 y304「Saved on this device」与页头胶囊重复。
  - U4-conflict y428：「Saved theme count 0 → 0」没有提供信息。
- **修法**
  - 改为「Original data keeps a copy of the v1 library as it was before the upgrade.」
  - 成功 callout 删除「Saved on this device」这一行，由页头胶囊承担。
  - 计数不变时写「Saved themes: 0 (unchanged)」。

#### UI-114 · [P3] 页头基准用词同义反复，胶囊用语与 callout 不一致（U4 · 交互、细节 · 部分解决 · 可挽回 0.05）

- **位置**
  - U4-upgrade-pending 页头：「旧草稿 Matches “旧草稿”」。
  - U4-quota-failure：「Rustify Light Modified from “Rustify Light”」，看不出基准是预设还是已保存文档。
  - U4-upgrade-cancelled 胶囊写「Saving paused」，callout 标题写「Memory only」。
- **修法**
  - 基准改写为「Matches saved document」或「Modified from saved document」，名字左侧已经显示，不再重复。
  - 取消态胶囊改为「Memory only」，与 S2 的「仅内存编辑」和 callout 标题一致。

#### UI-142 · [P3] 对话框主题表末行下方仍是双细线（U4 · 布局 · 部分解决 · 可挽回 0.05）

- **位置**：U4-upgrade-review x=900 处，y=309（「Legacy two」行线）与 y=322（分节线）两条 #EAECF0 线相隔 13px。
- **修法**：去掉表格最后一行的下边框，只保留分节线。

#### UI-145 · [P3] 一条差异仍占四行（U4 · 布局 · 部分解决 · 可挽回 0.05）

- **位置**：U4-conflict-diff y147–247，依次是 `overrides.primary`、「Changed」胶囊、「This tab / Other tab」列标签、两个值。
- **修法**
  - 键与「Changed」放在同一行，用 flex 两端对齐。
  - 值改为一行「This tab #123456 → Other tab #654321」，或用两列加上方小标签，合计两行。

#### UI-56 · [P3] 操作列表的说明与断行（U4 · 细节 · 部分解决 · 可挽回 0.05）

- **位置**
  - U4-quota-operations y612–674：说明第三句「Loading the other version removes all pending actions.」在配额态不存在对应动作。
  - y744–762：「(draft / revision 2)」被断开。
  - 展开内容起于 x=20，与摘要文字的 x=33 不对齐。
- **修法**
  - 第三句只在冲突态显示。
  - 修订号片段加 `white-space: nowrap`。
  - 展开区左缩进 13px，与摘要文字对齐。

#### 新-2 · [P3] 「Technical details」脱离了错误 callout（U4 · 布局 · 相对 R2 回归 · 可挽回 0.05）

- **位置**：U4-quota-failure y436。它悬在错误 callout（y262–406）与命名卡片（y458）之间，位于列边 x=20；R2 中它在 callout 内部。
- **修法**：移回错误 callout，放在「Download current theme」下方，用 callout 的内边距（x=33）。

**可挽回分合计**：0.15 + 0.15 + 0.1 × 6 + 0.05 × 5 = 1.15。10 − 8.7 = 1.3，差 0.15，在 0.5 以内。

## 4. 建议（不扣分）

- 冲突差异的 #123456、#654321 前加 12px 色块。
- 差异条目很少（≤3 条）时，默认展开「Draft differences」。
- 页头胶囊按语义着色：Save failed 用 #FEF3F2/#B42318，Memory only 和 Saving paused 用琥珀色，Saved 保持中性。
- 升级 Review 展开 JSON 后整框滚动，标题和 × 会滚出视野，「Saved themes retained (2)」在框顶被切掉一半（U4-upgrade-review-json y≈15）。建议标题行吸顶。
- 冲突的三个按钮把推荐动作「Keep current draft」放在第一位，而不是夹在中间。
- 对话框分节标题与字段键都是 13px/600，层级相同。可以把分节标题之上的间距加到 20px 以上，或改用 caption 大写样式来区分。
- 主题表的 ID 列改为左对齐，以缩短 Name 与 ID 之间约 440px 的空白。
- 沿用 R2 的建议：区分对话框 × 与「Cancel upgrade」；成功 callout 增加 Dismiss。

## 5. 看不出的格子

- **所有 U4 帧的键盘焦点**：capture.md 记录的焦点在 upgrade-review-close 和 theme-save 上，但 × 周围和「Save locally」外沿都没有蓝色像素，因为焦点来自点击或程序设置。新按钮、Dismiss、Save now、各 disclosure 的焦点帧（G4）因此看不出。悬停和按下态也没有拍到。
- **中文态**：U4 没有中文帧，迁移和存储文案是否同步中英文（G6）看不出。
- **390**：只拍了待确认态。390 下的 Review 对话框、冲突面板、配额、无锁布局不在本轮矩阵内。
- **S2「模式/偏好变化不能绕过确认」**：没有拍到。
- **S4 的排队态，以及「先前快照成功、当前仍待保存」**：没有拍到。
- **动作结果**：Load、Keep、Save copy 的结果，Dismiss 和 Save now 点击后的状态，下载是否真正发起，都没有拍到。
- **配额态的「Technical details」**：没有展开，内容看不出。

## 6. 总评

U4 是本次唯一评审的单元，得分 8.7，比 R2 的 7.55 有明显提高。R2 的 19 条差距中 11 条已解决，8 条部分解决，无「仍存在」。新出现 4 条，其中新-2、新-3 是相对 R2 的回归，都是把 callout 合并时丢掉了内容（Technical details 的归属、当前主题的下载按钮）。

剩余失分主要来自两处：
- **视觉规格还没收口**：callout 正文混用 12px 灰和 13px 深，对话框按钮自成一套蓝色描边。
- **几处交互措辞和按钮权重不一致**：处于 paused 状态时页头仍是实心「Save locally」；配额态命名失败没有内联重试；取消后提示了下载当前主题却没有按钮。

这些都不需要推倒重做。只需在存储 callout 组件上统一两条正文规则，让对话框按钮复用壳层描边样式，并把冲突面板放回统一的位置。改完后预计可以达到 9.5 以上。
