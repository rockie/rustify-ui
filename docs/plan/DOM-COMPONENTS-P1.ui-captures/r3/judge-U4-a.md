# DOM-COMPONENTS-P1 UI 验收 R3 · U4 评审原文

- 评审范围：只评 U4「Studio 本地库迁移与存储反馈」，规范模式，没有目标图。期望依据是目标包规范要点 S1–S6、G1–G4，以及「变更记录」节的全部条目。R2 之后新增的三条变更是 UI-156、UI-97、UI-88/89，都不涉及 U4 的单元或比较口径。
- 证据：R3 的 16 张 `U4-` 开头的 PNG、`r3/U4-actions.log`、`r3/capture.md`，对照 R2 的同名帧与 `r2-judge-U4.md`。颜色是用 ImageMagick 在原图上取的像素值，字号由行距推算（12px×1.5=18px，13px×1.5=19.5px）。
- 采集说明：`U4-upgrade-review-end` 与 `U4-upgrade-review` 逐像素相同（md5 一致），原因是对话框在 900px 高度内放得下，不需要滚动。`U4-conflict-detail-end` 与 `U4-conflict` 也逐像素相同，因为冲突面板（y244–789）已经整块在视口里。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.75 | 2.7 | 2.5 | 0.65 | **8.6**（R2 为 7.55） |

各维度理由（每条都说明为什么没有到更高一档）：

- **布局 2.75**
  - 做对的部分：每个状态现在只有一张状态/决策 callout，存储区的顺序在六种状态下都固定为：callout → 命名动作卡 → 操作计数 → 备份。冲突面板在 1440×900 下整块可见。升级 Review 对话框的值列对齐到 x=405，主题表也有了表头。页头吸顶并带保存状态胶囊。
  - 不到 3 的原因：
    - 配额态的「Technical details」被移到了红色 callout 之外，比 R2 退步；
    - 冲突键值栅格的值列只有 128px 宽，「Revision 2 · Rustify / Light」折成两行，「Light」成了孤词；
    - 对话框里仍有一对双细线；
    - 差异块里一处改动仍占四行。
- **视觉 2.7**
  - 做对的部分：callout 标题统一为 13px/600，并按变体着色（警告 #93370D、错误 #B42318、成功 #067647）。成功态有了绿色底和绿色边框。禁用样式与 Undo 统一为 #98A2B3 / #EAECF0 / #F9FAFB。原文 textarea 有了边框和底色。
  - 不到 3 的原因：
    - 各 callout 的正文在 12px #475467 与 13px #1D2939 之间来回切换，同一张冲突 callout 内部也混用两种；
    - 对话框的次级按钮是蓝边蓝字，callout 和页头的次级按钮是灰边灰字，两套样式并存；
    - 文字按钮的标签比正文缩进 11px。
- **交互与状态 2.5**
  - 做对的部分：七个状态都能走通，各自有常驻反馈和继续动作。冲突的三个决策都写了后果。升级成功后，命名动作降为中性提示，并附「Save now」「Dismiss」。无锁态的「Save locally」降为描边按钮。
  - 不到 3 的原因：
    - 取消后 callout 的正文让用户「Download your current theme」，callout 里却只有「Download original data」；
    - 待确认、取消、冲突这几个「Saving paused」状态下，页头「Save locally」仍是实心主按钮，点了只会被拦下；
    - 配额态页头「Save failed」与 callout「Your draft … stay in memory」，和操作列表里的「#2 Draft · confirmed (draft revision 2)」互相矛盾；
    - 配额命名卡写着「retained for retry」，卡里却只有 Dismiss。
- **细节 0.65**：约六类文案与像素问题，所以不是 1：
  - 取消态 callout 内两段互相重复；
  - 「Saved theme count 0 → 0」；
  - 成功态写「before upgrade」，时态不对；
  - 胶囊与 callout 用词不一致，「upgrade」和「migration」混用；
  - 操作说明段讲的是本状态没有的「Load other version」；
  - 双细线与缩进问题。

## 2. 上一轮差距复核表

| 编号 | R2 问题摘要 | R3 状态 | 依据（R3 截图） |
| --- | --- | --- | --- |
| UI-108 | 粘性头下方双细线 | 已解决（U4 部分） | 改为全宽页头后，U4 各 1440 帧在 x=250 列逐像素扫描：头部下边框 y=56 之后，下一条线是 y=244 的 #EAECF0，没有第二条线 |
| UI-114 | 页头「Modified / Matches preset」基准不清，吸顶区没有保存状态 | 已解决 | 页头写明基准：「Matches “旧草稿”」「Modified from “Rustify Light”」。并有状态胶囊：「Saving paused / Saved on this device / Save failed / Memory only」。对话框「Draft state: Matches “旧草稿”」用了同样措辞。主题名与基准同名时的读法另列建议 |
| UI-139 | 状态 callout 与决策 callout 叠放重复、顺序颠倒，冲突面板超高 | 已解决 | 待确认、取消、冲突都只剩一张 callout。顺序在各状态一致。冲突面板 y244–789，决策与「Download other version」同屏可见 |
| UI-140 | 命名动作反馈是裸红字、位置不定、残留、时态错 | 部分解决 | 已改为卡片：取消态为警告样式，配额态为错误样式，升级后为中性样式并附「Save now “Memory only”」「Dismiss」，无锁态是现在时。余项：配额卡没有内联 Retry；升级后的卡片正文要人去点页头 Save locally，卡里却已经有 Save now 按钮（见下文 UI-140） |
| UI-50 | callout 标题弱于正文，正文字号不一 | 部分解决 | 标题与对话框分节标题已是 13px/600 并着色。正文仍在 12px #475467（成功、配额、无锁、冲突后两段）与 13px #1D2939（待确认、取消、冲突首段、命名卡）之间切换 |
| UI-51 | 推荐与放弃同权，多套描边按钮 | 部分解决 | callout 内已是一个实心推荐，其余为灰描边，放弃动作用文字按钮。对话框底部「Download original data」「Cancel upgrade」仍是同权的蓝边蓝字，与壳层灰描边并存 |
| UI-141 | 冲突三个决策没有后果说明 | 已解决 | U4-conflict y526–567：「Load replaces your draft and pending saves. Keep saves this tab over the other version. Save a copy retains both themes.」 |
| UI-142 | Review 对话框值列错位、没有表头、双细线 | 部分解决 | 值列统一在 x=405，表头「Name / ID / Result」已加。y=309 与 y=322 仍是两条 #EAECF0 线 |
| UI-143 | 无锁态标题写「paused」、主按钮实心、行话、缺句号 | 已解决 | 标题「Memory only · local saving unavailable」。「Save locally」改为描边。正文是完整句子并带句号 |
| UI-144 | 修订号无标签、「0 → 0」、行话与含混措辞 | 部分解决 | 已改：「Revision 2 · …」「Ready to upgrade after your confirmation.」「Download other version」。仍在：「Saved theme count 0 → 0」，以及成功态「…to back up the library before upgrade.」的时态问题 |
| UI-145 | 差异块纵向松散，组头配色混搭 | 部分解决 | 组头已改为透明底。路径、「Changed」徽标、列标签、值仍各占一行（y147/185/220/240） |
| UI-146 | 原文 textarea 无边框、透明底 | 已解决 | U4-conflict-raw：#D0D5DD 边框、#F9FAFB 底，有缩放手柄，底部半行截断提示还能滚动 |
| UI-46 | 成功态容器不着色，琥珀标题两色 | 已解决 | 成功 callout 为 #ECFDF3 底、#ABEFC6 边、#067647 标题。各警告 callout 标题统一为 #93370D |
| UI-47 | 禁用存储按钮用非 token 色，与 Undo 禁用样式并存 | 已解决 | Retry、Cancel、Restore、Undo 都是 #98A2B3 字、#EAECF0 边、#F9FAFB 底 |
| UI-55 | 取消后 callout 沿用待确认文案 | 部分解决 | 已不再沿用。但取消 callout 的第 1 段与第 2 段互相重复（见下文 UI-55） |
| UI-56 | 操作列表单复数、「edit 2」孤字、失败项与成功项同色、说明段答非所问 | 部分解决 | 已改：「1 recent operation」；改为「(draft revision 2)」；失败项红、成功项绿。仍在：说明段最后一句讲「Loading the other version」；说明段与列表没有缩进到摘要文字下 |
| UI-57 | 升级后下载按钮在 callout 外，而且不同名 | 已解决 | U4-upgraded y389–419 的「Download original data」已在成功 callout 内，名字与其他状态一致 |
| UI-61 | 句末标点与行话 | 已解决 | 无锁正文带句号。操作说明改为「Retry saves the listed actions in order.」 |
| UI-62 | 没有待处理项时 Retry/Cancel/Save current draft 看起来可点 | 已解决 | 待确认、取消、升级后、冲突、无锁各帧中，这几个按钮都是统一的禁用样式 |

## 3. 差距清单

排序：先按严重度，同级内按可挽回分从高到低。各条可挽回分合计 1.4 分，等于 10 − 8.6。

#### 新-1 · [P2] 取消后 callout 让用户「Download your current theme」，callout 里却没有这个按钮（U4 · 交互 · 可挽回 0.2）

- **位置**：U4-upgrade-cancelled-1440-light，琥珀 callout y261–529。
  - 第 2 段 y383–422 写着「Download your current theme to keep these memory-only edits.」；
  - 下面只有「Review upgrade again」（y445–476，实心）和「Download original data」（y485–516）。
  - R2 同一状态有「Download current theme」按钮；配额态（y362–393）和无锁态（y362–393）的 callout 也都有。
- **现状 → 期望**：用户想保住仅内存的编辑，手边唯一的下载按钮却会下载旧的原始数据，容易拿错文件，编辑就丢了。期望是：取消后在常驻反馈里直接提供「当前主题」出口。依据 S1「下载原始数据与当前主题出口始终可达」、S2「取消后常驻『仅内存编辑』」、S6「仅内存编辑/可下载」。
- **修法**：
  - 在取消态 callout 的动作行加「Download current theme」，用壳层灰描边，复用配额/无锁 callout 的同一组件；
  - 「Download original data」降为第三位，或改名「Download pre-upgrade data」，与当前主题明确区分；
  - 推荐顺序：「Review upgrade again」（实心）→「Download current theme」（描边）→「Download original data」（描边或文字按钮）。

#### UI-50 · [P3] callout 正文字号与颜色在状态之间、甚至同一张 callout 内来回切换（U4 · 视觉 · 部分解决 · 可挽回 0.15）

- **位置**（按行距推算字号，取像素定颜色）：
  - 13px #1D2939：U4-upgrade-pending y313/332 与 y364/383；U4-upgrade-cancelled y313–422；U4-conflict 首段 y296/315；所有命名动作卡（如 U4-quota-failure y479/499）；
  - 12px #475467：U4-upgraded y308、y334–370；U4-quota-failure y308–344；U4-no-locks-initial y308–344；U4-conflict 后两段 y483–501、y531–567。
  - 配额态里，次要的命名卡正文（13px 深色）反而比主状态 callout 的正文（12px 灰）更醒目，层级倒置。
- **现状 → 期望**：同类反馈容器使用同一套正文样式，扁平、一致的壳层排版（G3：body 14px/1.5、caption 12px、次级文字 #475467）。
- **修法**：
  - callout 组件只保留两级文字：正文统一 13px/1.5 #1D2939，辅助说明统一 12px #475467（例如「No pending actions…」这类状态注释）；
  - 成功、配额、无锁 callout 的主说明改为正文级；
  - 冲突 callout 只把「No pending actions…」保留为 caption，「Load replaces…」后果说明改为正文级；
  - 命名动作卡与状态 callout 共用同一正文样式。

#### UI-140 · [P3] 命名动作卡余项：配额卡没有内联重试；升级后卡片的文案与按钮重复（U4 · 交互 · 部分解决 · 可挽回 0.1）

- **位置**：
  - U4-quota-failure 红卡 y457–565 写着「Save “Retained name” · failed; retained for retry.」，卡里只有「Dismiss」（y536），实际的「Retry retained saves」在两行之外的操作区（y617–648）；
  - U4-upgraded 中性卡 y445–553 正文写「Use Save locally to try again.」，同一张卡里已经有实心「Save now “Memory only”」。
- **现状 → 期望**：命名动作单独反馈，紧跟继续动作（S1「常驻反馈与继续动作」、S4「命名库动作单独反馈」、S5「未处理的命名动作保持可见」）。
- **修法**：
  - 配额卡加「Retry “Retained name”」描边按钮，与「Dismiss」同一行；
  - 升级后卡片正文改为「“Memory only” was not saved earlier.」，删掉「Use Save locally to try again.」；
  - 按钮文字改为「Save “Memory only” now」，动词在前、名称居中。

#### 新-3 · [P3] 配额态页头与 callout 说「草稿留在内存」，操作列表却显示当前草稿已确认保存（U4 · 交互 · 可挽回 0.1）

- **位置**：
  - U4-quota-failure / U4-quota-operations 页头胶囊「Save failed」（x298–369）；
  - callout y308–344「Local save failed. Your draft and pending actions stay in memory.」；
  - 操作列表 y692「#2 · Draft · confirmed (draft revision 2)」与 y744「#3 · Save “Retained name” · failed; retained (draft revision 2)」。
- **现状 → 期望**：失败的是命名保存，草稿第 2 版已确认保存，界面却笼统地说「Save failed」「draft … in memory」。依据 S4：保存状态要区分「先前快照成功而当前仍待保存」与「当前草稿确已保存」，命名动作单独反馈。
- **修法**：
  - callout 标题改为「Storage full · new saves stay in memory」；
  - 正文分两句写：「Draft revision 2 is saved on this device. Saving “Retained name” failed and is kept for retry.」；
  - 页头胶囊按最近一次草稿写入的结果显示「Draft saved · library save failed」，或者只在草稿写入本身失败时显示「Save failed」。

#### 新-4 · [P3] 「Saving paused」状态下页头「Save locally」仍是实心主按钮，点了只会被拦下（U4 · 交互 · 可挽回 0.1）

- **位置**：
  - U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 的页头「Save locally」（x1195–1284，#1570EF 实心），同屏胶囊是「Saving paused」；
  - 日志第 9 行在取消态点了 Save locally，结果是命名卡「not saved; review the library upgrade before saving」；
  - 无锁态（U4-no-locks-initial）同一按钮已降为描边。
- **现状 → 期望**：被流程暂停的保存不应作为视觉上的首要动作；同一个按钮在同类受限状态下应该一致。依据 S2「模式/偏好变化不能绕过确认」、S1。尺度上与 R2 对无锁态 UI-143 的判法一致。
- **修法**：胶囊为「Saving paused」或「Memory only」时，页头 Save locally 统一切到壳层描边样式（与无锁态相同）。callout 内的推荐动作（「Review upgrade」「Keep current draft」）保持本区唯一的实心按钮。

#### 新-6 · [P3] 冲突键值栅格值列过窄，修订行折行出孤词，标签贴着值（U4 · 布局 · 可挽回 0.1）

- **位置**：
  - U4-conflict-1440-light y341–440：标签列 x33–150，值列 x157–285；
  - 「Revision 2 · Rustify / Light」在 y347/366 折行，「Revision 3 · Rustify / Light」在 y390/409 折行；
  - 「Saved theme count」标签右端约 x149，与「0 → 0」（x157）只隔约 8px；
  - 「Draft differences」摘要（y452）紧贴上一行（y433），中间没有分组间距。
- **现状 → 期望**：冲突版本信息一眼可读（S5「展示冲突版本」），壳层分组行间距清楚（G2）。
- **修法**：
  - 在 280px 编辑列内把键值改成上下两行（标签 12px #475467 在上，值 13px 在下），或者改用 `grid-template-columns: max-content 1fr; column-gap: 12px`，并把标签缩短为「This tab」「Other tab」「Themes」；
  - 修订号与主题名分两行排：「Rev 2」/「Rustify Light」；
  - 「Draft differences」前加 8px 间距。

#### UI-55 · [P3] 取消态 callout 两段互相重复，并与标题重复（U4 · 细节 · 部分解决 · 可挽回 0.1）

- **位置**：U4-upgrade-cancelled。
  - 标题 y282「Memory only · upgrade cancelled」；
  - 第 1 段 y313–352「Upgrade cancelled. Memory-only editing; reopening restores the original legacy draft.」；
  - 第 2 段 y383–422「Reopening restores the original draft. Download your current theme to keep these memory-only edits.」
  - 「upgrade cancelled」「memory-only」「reopening restores」各出现两次。
- **现状 → 期望**：取消后常驻「仅内存编辑」与重新检查入口，用一段说清楚（S2）。
- **修法**：并为一段：「Edits stay in this tab only. Reopening Studio restores the original legacy draft — download your current theme to keep these edits.」，同时补上新-1 的按钮。

#### UI-144 · [P3] 文案余项：「0 → 0」无变化行、成功态时态错（U4 · 细节 · 部分解决 · 可挽回 0.1）

- **位置**：
  - U4-conflict y433「Saved theme count 0 → 0」；
  - U4-upgraded y334–370「2 saved themes retained. Use “Download original data” to back up the library before upgrade.」：升级已经完成，「before upgrade」读起来像还没升级。
- **修法**：
  - 数量没变时不显示这一行，或改为「Saved themes: 0 (unchanged)」；
  - 成功态改为「2 saved themes retained. Download original data keeps a copy of the library from before the upgrade.」

#### 新-7 · [P3] 状态用词不统一：胶囊与 callout 各说一套，「upgrade」和「migration」混用，成功态重复「Saved on this device」（U4 · 细节 · 可挽回 0.1）

- **位置**：
  - U4-upgrade-cancelled 页头胶囊「Saving paused」，callout 标题「Memory only · upgrade cancelled」；
  - U4-upgraded 页头胶囊「Saved on this device」，callout 第二行 y308 又是「Saved on this device」；
  - 「Review the library upgrade」（callout）、「Review upgrade」（按钮）、「Review the complete migration」（对话框标题）、「Complete migration JSON」混用。
- **现状 → 期望**：同一状态在同一屏只用一个名字（G6 文案同步的前提；S2「取消后常驻『仅内存编辑』」）。
- **修法**：
  - 取消态胶囊改为「Memory only」，与 callout 标题和无锁态胶囊一致；
  - 成功 callout 删掉「Saved on this device」这一行，由页头胶囊承担；
  - 全流程统一用「library upgrade」：对话框标题改为「Review the library upgrade」，折叠项改为「Complete upgrade JSON」。

#### UI-142 · [P3] 升级 Review 对话框主题表底边与分节线形成双细线（U4 · 布局、细节 · 部分解决 · 可挽回 0.05）

- **位置**：U4-upgrade-review-1440-light，x=500 列逐像素扫描：y=309（表格最后一行下边框）与 y=322（分节上边框），两条 #EAECF0 线相隔 13px。
- **修法**：表格最后一行去掉 `border-bottom`，即 `tr:last-child td { border-bottom: 0 }`；或者分节容器在紧跟表格时不画上边框。

#### UI-145 · [P3] 冲突差异块一处改动占四行，徽标左缘不齐（U4 · 布局 · 部分解决 · 可挽回 0.05）

- **位置**：U4-conflict-diff-1440-light。
  - 「overrides.primary」在 y147，「Changed」徽标在 y185，「This tab / Other tab」在 y220，值在 y240，一处改动约 95px 高；
  - 徽标左边框约 x45，路径文字在 x41。
- **修法**：路径与「Changed」徽标放在同一行（`display:flex; justify-content:space-between; align-items:center`），下面一行用两列「This tab #123456 → Other tab #654321」；徽标左缘与路径对齐。

#### 新-2 · [P3] 配额态「Technical details」被移到错误 callout 之外，夹在 callout 与命名卡之间（U4 · 布局 · 回归 · 可挽回 0.05）

- **位置**：U4-quota-failure / U4-quota-operations。红色 callout y261–406，「Technical details」在 y435，位于 callout 边框之外、命名红卡（y457）之上。R2 时它在 callout 内（R2 y501）。
- **现状 → 期望**：每个异常状态只有一处常驻反馈，附属信息归在该反馈内（S1；沿用 UI-139 的固定 slot 顺序）。
- **修法**：把 Technical details 的 `<details>` 放回错误 callout 内，放在「Download current theme」下方，并与 callout 正文左对齐（x=33）。

#### 新-5 · [P3] 文字按钮的标签比正文缩进 11px，单独一行的「Dismiss」像是悬空（U4 · 视觉 · 可挽回 0.05）

- **位置**：
  - 「Cancel upgrade」标签左缘 x=45，正文左缘 x=34（U4-upgrade-pending y501）；
  - 「Dismiss」在 U4-upgrade-cancelled y641、U4-quota-failure y536、U4-no-locks y518 都是单独一行，标签在 x=45。
- **修法**：给文字按钮使用 ghost-offset 几何，即左右负外边距等于内边距（`margin-inline: -11px`），让标签与正文左缘对齐；或者让「Dismiss」与本卡的主动作同一行（像 U4-upgraded 那样）。

#### UI-56 · [P3] 操作说明段讲本状态没有的动作，说明与列表未缩进（U4 · 细节 · 部分解决 · 可挽回 0.05）

- **位置**：U4-quota-operations。y612–666 说明段最后一句是「Loading the other version removes all pending actions.」，但配额态没有「other version」。说明段与 #1–#3 列表从 x=20 起，与展开三角对齐，没有对齐摘要文字（x=33）。
- **修法**：说明段按当前状态只列可用动作（配额态只讲 Retry / Cancel），「Load other version」那句只在冲突态出现。展开内容整体 `padding-left` 到摘要文字的起点（13px）。

## 4. 建议（不扣分，规范要点里没有直接依据）

- 页头状态胶囊按语义着色：「Save failed」用错误色（#FEF3F2 / #FECDCA / #B42318），「Saving paused」「Memory only」用警告色，「Saved on this device」保持中性或用成功色。现在四种状态都是同一个灰胶囊。
- 主题名与基准同名时（「旧草稿 Matches “旧草稿”」），写明基准类型，例如「Matches saved document」或「Matches preset “Rustify Light”」。
- 冲突决策的实心「Keep current draft」夹在两个描边按钮中间。可以把推荐动作放在第一位，或改为横排，推荐动作放末尾。
- 冲突差异的两个颜色值旁加色块（R2 a、b 已提出）。
- 升级 Review 对话框的「×」与「Cancel upgrade」语义要区分：例如 × 只关闭对话框、保持待确认，「Cancel upgrade」降为文字按钮放最右（R2 a 已提出）。
- 对话框里分节标题（「Saved themes retained (2)」）与字段标签（「Library version」）同为 13px/600 #1D2939，层级无差别。可以给分节标题加 8px 上间距，或改用 caption 大写字距。
- 成功 callout 加「Dismiss」（R2 a 已提出）。

## 5. 看不出的格子

- U4-upgrade-cancelled、U4-quota-failure、U4-no-locks：登记的焦点在 `theme-save`，但焦点是点击得来的，没有焦点环。看不出键盘到达 callout 按钮、命名卡按钮、冲突决策按钮时的焦点帧（G4）。
- 所有 U4 帧：看不出悬停与按下态。
- 中文：没有拍 U4 任何状态的中文帧，看不出迁移、冲突、配额文案是否中英同步（G6）。
- 390×844：只有待确认态（规范只要求这一格）。看不出 390 下升级对话框、冲突面板、命名卡的排布，也看不出 390 页头在滚动后是否吸顶。
- 「Cache and original backups」在各帧都没展开，看不出其中缓存损坏、只读持久化的反馈（S1）。
- 「Complete migration JSON」没展开，看不出 S3 要求的「完整迁移结果」正文。
- 看不出点击确认后到成功前那段「已确认待提交」的中间态，所以无法判断提交成功前是否不显示成功（S3）。
- 没拍排队态，看不出「先前快照成功而当前仍待保存」的呈现（S4 前两种状态）。
- 看不出模式/偏好变化能否绕过升级确认（S2 末句）。
- 看不出下载是否真的发起，也看不出下载失败的反馈（S1、X5）。
- 命名保存的输入对话框（日志第 9、18、30 行的 name/confirm）没拍到。
- 冲突态的「Save current as a copy」「Load other version」点击后的结果没拍到。

## 6. 总评

- **U4 本轮 8.6 分**，比 R2 的 7.55 有明显提高。R2 的 19 条差距中，11 条已解决，8 条部分解决，没有仍存在的，只有一处小回归（新-2）。最大的变化是 R2 要求的结构性改动已经落地：
  - 页头改为全局吸顶条，带基准说明与保存状态胶囊；
  - 每个状态合并为一张 callout，存储区顺序固定；
  - 命名动作反馈装进卡片，并按「被阻止 / 真失败 / 已过期」分样式；
  - 冲突面板压进一屏，并写明后果；
  - 禁用样式统一。
- **主要扣分在交互与状态（2.5）**：
  - 取消态的文案和按钮对不上（新-1，P2）。这是本轮唯一可能导致用户拿错文件、丢失仅内存编辑的问题，应优先修；
  - 暂停态的主按钮没有降级（新-4）；
  - 配额态的整体说法与操作列表矛盾（新-3）；
  - 配额命名卡缺内联重试（UI-140 余项）。
- **视觉**：剩余问题集中在 callout 正文的两套字号与颜色（UI-50），以及对话框与壳层两套次级按钮样式（UI-51）。都属于组件级的统一工作，在 callout 与按钮组件上各收口一次即可，不需要改布局。
- 当前没有需要推倒重做的结构。剩下的差距都可以在现有 callout、命名卡、对话框组件内修掉。
