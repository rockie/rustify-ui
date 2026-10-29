# U4 评审 · R4（Studio 本地库迁移与存储反馈）

本次只评 U4，用规范模式，依据 S1–S6、G1–G4。证据是 r4 本层 17 张 `U4-` 开头的图和 `U4-actions.log`。上一轮的对照材料是 r3 的 U4 图、r3 的 `U4-actions.log` 和 `judge-inputs/r3-judge-U4.md`。

两轮的操作步骤一致。把两轮 actions.log 去掉时间戳后对比，只有两行不同：
- 第 35 行：冲突面板底边对齐视口底边所需的滚动量，R3 是 `scrollTop 0→300`，R4 是 `35→335`。
- 第 41 行：JSON 展开节的界面名称改了（「Complete migration JSON」改为「Complete library upgrade JSON」）。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.9 | 2.95 | 2.85 | 0.85 | **9.55** | 8.6 |

各维度理由（每条都写明为什么不是更高一档）：

- **布局 2.9**
  - 改进：
    - 六种状态的存储 callout 都在 y=323 起、上方有分隔线，结构统一；冲突面板下面那条多余的分隔线已删。
    - 冲突键值改成标签在上、值在下，修订号不再折行。
    - 升级 Review 对话框只剩一条分节线，宽 1022px，内边距 24px。
    - 390 宽度下，页头五个按钮排成一行，callout 不溢出。
  - 不到 3 分的原因：
    - 冲突面板现在比 900px 视口高 35px，默认帧切掉了「Download other version」。R3 时整块可见。
    - 两处展开内容的左缘不统一。
- **视觉 2.95**
  - 改进：
    - 所有 callout 和命名卡的文字统一为两级（实测）：状态句 14px/#1D2939，补充说明 12px/#475467。
    - 对话框底部、无锁 callout 和 callout 内的按钮样式统一。
    - 文字按钮标签与正文左缘对齐（x=34）。
  - 不到 3 分的原因：配额态视口内有三个实心蓝色按钮在抢主次。
- **交互与状态 2.85**
  - 改进：
    - 七个状态都能走通。
    - 取消后可以下载当前主题。
    - 页头胶囊和 callout 分开说明「草稿已存、命名动作失败」。
    - 暂停态和仅内存态下，页头的 Save locally 已降为描边。
    - 配额卡和升级后的卡片都有了内联动作。
  - 不到 3 分的原因：
    - 配额态里同一个失败动作有三个入口、三种叫法。
    - 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里。
- **细节 0.85**
  - R3 列出的文案问题已全部修掉。
  - 不到 1 分的原因：
    - callout 与命名卡之间仍有成句重复（升级成功、无锁、配额三处）。
    - 配额标题折行后第二行只剩孤词「memory」，也没有写出失败原因。
    - 「not saved earlier」措辞别扭。

## 2. 上一轮差距复核表

| 编号 | R3 严重度 | 状态 | R4 依据 |
| --- | --- | --- | --- |
| UI-179 | P2 | 已解决 | U4-upgrade-cancelled 琥珀 callout 的按钮依次为「Review upgrade again」（实心）→「Download current theme」→「Download original data」，正文是「Download your current theme to keep these memory-only edits.」 |
| UI-50 | P2 | 已解决 | 实测：各态状态句和命名卡正文是 14px #1D2939，补充说明是 12px #475467。U4-no-locks 相邻两张卡都是 14px 深色字 |
| UI-51 | P2 | 已解决（副作用见新-1） | 升级 Review 底部：「Download original data」改为灰描边，「Cancel upgrade」改为文字按钮并放到最后。无锁 callout 的「Download current theme」改为实心（#1570EF），配额 callout 加了实心「Retry save」 |
| UI-140 | P3 | 已解决（副作用见新-1） | 配额命名卡有「Retry retained saves · “Retained name”」。升级后命名卡正文是「Save “Memory only” · not saved earlier.」，按钮是「Save “Memory only” now」和「Dismiss」，二者同一行 |
| UI-139 | P3 | 已解决 | 取消态正文并为一句。冲突 callout 从 y=323 起，前面有分隔线，与其他状态一致。U4-conflict-after 面板底边 y=599 之后直接接操作摘要，没有分隔线 |
| UI-181 | P3 | 已解决（措辞余项见新-2） | 页头胶囊「Draft saved · action failed」。正文「The current draft is saved on this device. A later library action failed and is retained for retry」，与操作列表「#1/#2 · Draft · confirmed」不再矛盾 |
| UI-182 | P3 | 已解决 | 页头「Save locally」在待确认、取消、冲突、无锁态下都是白底描边（取 (1195,16) 像素为 #FFFFFF） |
| UI-184 | P3 | 已解决 | 「This tab / Revision 2 · Rustify Light」「Other tab / …」「Saved themes / 0 (unchanged)」都改为上下两行，修订号不折行，「Draft differences」前面留了间距 |
| UI-55 | P3 | 已解决 | 取消态只剩标题和一句正文，不再重复 |
| UI-144 | P3 | 已解决 | 冲突态是「0 (unchanged)」。成功态是「Original data keeps a copy of the library from before the upgrade.」 |
| UI-185 | P3 | 已解决 | 对话框标题「Review the library upgrade」，展开节「Complete library upgrade JSON」，成功 callout 不再写「Saved on this device」 |
| UI-114 | P3 | 已解决 | 页头改为「Matches reference document」/「Modified from reference document」。取消态胶囊是「Memory only」，与 callout 标题一致 |
| UI-142 | P3 | 已解决 | U4-upgrade-review 在 x=300 纵向只剩 y=321 一条 #EAECF0；JSON 展开帧只剩 y=142 一条 |
| UI-145 | P3 | 已解决（另有对齐余项见新-6） | U4-conflict-diff 中，路径「primary」和「Changed」徽标在同一行，徽标右对齐；下一行是 This tab / Other tab 两列，一处改动由 4 行减为 3 行 |
| UI-180 | P3 | 已解决 | 「Technical details」放回红色 callout 内（y=571），位于「Download current theme」下方 |
| UI-183 | P3 | 已解决 | 「Cancel upgrade」「Dismiss」标签左缘都在 x=34，与正文对齐 |
| UI-56 | P3 | 部分解决 | 说明段不再提本状态没有的「Load other version」，列表已缩进到 x=34。但说明段仍从 x=21 起；#3 行在折线以下，断行问题看不出 |

## 3. 差距清单

#### 新-1 · [P2] 配额态：同一个失败的命名保存有三个重试入口、三种叫法，其中两个是实心按钮，加上页头一共三个实心蓝按钮（U4 · 交互 0.1 + 视觉 0.05 · 可挽回 0.15）

- **位置**：U4-quota-failure-1440-light，共四个蓝色/重试按钮：
  - 页头「Save locally」实心 #1570EF（x≈1190–1281）；
  - 红色 callout「Retry save」实心（y≈472–503）；
  - 红色命名卡「Retry retained saves · “Retained name”」实心（y≈674–706）；
  - 操作区「Retry retained saves」描边（y≈809–840）。
- **现状 → 期望**：
  - 现状：此时只有 1 个 pending（「1 pending · 3 recent operations」），三个重试按钮做的是同一件事，但叫法各不相同。用户会以为 callout 重试的是另一件事。页头 Save locally 仍是实心，可 callout 已经说「new saves stay in memory」，点了只会再失败一次。
  - 期望：
    - 依据 S4「命名库动作单独反馈」，命名动作的继续动作只在命名卡里出现一次。
    - 依据 S1，常驻反馈给出一个明确的继续动作。
    - 状态级 callout 只管存储层：下载当前主题、查看原因。
- **修法**：
  - 这一条是 R3 UI-51（给配额 callout 加实心 Retry save）和 UI-140（给配额卡加内联重试）两条修法叠加造成的，只保留一处，不要来回改。
  - 红色 callout 去掉「Retry save」，改由「Download current theme」做本区唯一的实心按钮，下面保留「Technical details」。
  - 命名卡按钮统一叫「Retry “Retained name”」（实心）。
  - 操作区按钮改叫「Retry all retained saves」（描边），作批量入口。
  - 页头胶囊处于失败态（「Draft saved · action failed」）时，「Save locally」用与「Saving paused」「Memory only」相同的规则降为描边。

#### 新-3 · [P3] callout 和命名卡之间成句重复（U4 · 细节 · 可挽回 0.1）

- **位置与现状**：
  - U4-upgraded：状态句（y≈371–393）「All saved themes and your current draft were retained.」紧跟说明（y≈420）「2 saved themes retained. …」，同一件事说了两遍。命名卡（y≈536）写「not saved earlier.」，措辞别扭。
  - U4-no-locks：标题「Memory only · local saving unavailable」、正文「Local saving is unavailable in this browser…」、命名卡「…not saved; local saving is unavailable. Download or export a copy.」，相邻两张卡里同一个意思出现三次。
  - U4-quota-failure：标题「…new saves stay in memory」，正文末尾又写「; new saves stay in memory.」。
- **期望**：G3 规定 callout 只用两级文字，状态句不重复标题。S6 要求「明确仅内存编辑/可下载」：说一次就够了。
- **修法**：
  - 成功态状态句改为「2 saved themes and your current draft were retained.」，说明只留「Original data keeps a copy of the library from before the upgrade.」。
  - 命名卡改为「Save “Memory only” · wasn't saved before the upgrade.」。
  - 无锁正文改为「Keep editing in memory; download or export to keep your theme.」，命名卡只留「Save “Not persisted” · not saved.」。也可以按新-5 并入 callout。
  - 配额正文删去末尾的「; new saves stay in memory」。

#### 新-2 · [P3] 配额 callout 标题折行后第二行只剩孤词，也没有写出失败原因（U4 · 细节 · 可挽回 0.05）

- **位置**：U4-quota-failure / U4-quota-operations 红色 callout 的标题。第一行（y=344）是「Local save failed · new saves stay in」，第二行（y=362）只有「memory」。正文用的是笼统的「A later library action failed」。
- **现状 → 期望**：
  - 现状：标题既没说失败原因是存储已满，也没说失败的是哪个动作。「Local save failed」容易被理解成草稿也没存上，而胶囊写的是「Draft saved」。
  - 期望：依据 S6「配额失败时明确…」，标题应说出原因；依据 S4，失败的动作要点名。
- **修法**：
  - 标题改为「Storage full · saving in memory」，在 253px 内容宽度内一行放得下。
  - 正文改为「The current draft is saved on this device. Saving “Retained name” failed and is kept for retry.」
  - 标题加 `text-wrap: balance`，作为孤词的兜底。

#### 新-4 · [P3] 冲突面板比 900px 视口高，默认帧切掉了「Download other version」（U4 · 布局 · 可挽回 0.05）

- **位置**：
  - U4-conflict-1440-light：面板从 y=323 起，「View stored text」在 y=867，下一个按钮只露出顶边（y≈890）。
  - U4-actions.log 第 35 行：底边对齐需要 `scrollTop 35→335`；R3 同一步是 `0→300`。
  - U4-conflict-detail 帧中面板高 612px（y73–685）。
- **现状 → 期望**：
  - 现状：R3 时整块面板可见。R4 键值改为上下两行，编辑列顶部又多了 3 行「Restores the reference document's…」说明，面板被挤出视口 35px。
  - 期望：依据 S1「原始数据与当前主题出口始终可达」、S5「展示冲突版本」，冲突决策和取回出口最好在首屏完整可见。
- **修法**：把首句和第一段补充说明并为一句：「Another tab saved a different version. Your draft is unchanged and stays in memory until you choose.」删去单独的「No pending actions…」段（约 2 行 + 12px 间距，可省约 48px，多于需要的 35px）。有 pending 动作时再用一行 caption 列出。

#### 新-5 · [P3] 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里（U4 · 交互 · 可挽回 0.05）

- **位置**：
  - U4-upgrade-cancelled 命名卡（y≈580–711）：「…review the library upgrade before saving this named theme.」，卡里只有「Dismiss」。
  - U4-no-locks 命名卡（y≈491–622）：「Download or export a copy.」，卡里只有「Dismiss」。
- **现状 → 期望**：
  - 现状：配额卡和升级后卡片都已经有内联动作，这两张卡没有，同一种卡片有两种模式。对应的按钮在上面那张同色 callout 里，两张同色卡上下叠着，用户读到的指引和可点的按钮分在两处。
  - 期望：依据 S1，每条常驻反馈都要带继续动作；依据 S4，命名动作单独反馈。
- **修法**：两种做法选一个。
  - 做法一：命名失败的原因就是 callout 描述的那个状态时，不再单独渲染一张卡，改在 callout 末尾列一行「Not saved: “Memory only” · Dismiss」（12px caption 加文字按钮）。这样也能消掉新-3 中无锁的重复。
  - 做法二：保留卡片，卡里放对应的描边按钮（取消态「Review upgrade」，无锁态「Download current theme」），callout 里的同名按钮保持实心。

#### UI-56 · [P3] 操作展开内容：说明段没有缩进，列表已缩进（U4 · 布局 · 部分解决 · 可挽回 0.03）

- **位置**：U4-quota-operations。展开三角在 x=21；说明段「Retry saves the listed actions in order…」（y≈805–841）从 x=21 起；列表「#1 · Draft · confirmed…」从 x=34 起，与摘要文字对齐。
- **现状 → 期望**：同一个展开块里有两条左缘 → 展开内容整体对齐摘要文字的起点（R3 修法原文）。
- **修法**：说明段加 `padding-inline-start` 与列表一致（13px），也就是让 details 的内容容器整体缩进，不要分别给子元素设缩进。修订号片段 `white-space:nowrap` 待核实（见第 5 节）。

#### 新-6 · [P3] 草稿差异块有三条左缘（U4 · 布局 · 可挽回 0.02）

- **位置**：U4-conflict-diff-1440-light 的三条左缘：
  - x=33/34：callout 正文、展开三角、分隔线（x=33–285）和下方说明；
  - x=42：差异行「Light · overrides (1)」「primary」「This tab」；
  - x=49：摘要文字「Draft differences」。
- **现状 → 期望**：一处展开内容出现 3 条左缘 → 内容与分隔线同起点，或整体对齐摘要文字。
- **修法**：差异行去掉 8px 的 `padding-inline`，让路径、列标签与分隔线都从 x=34 起，「Changed」徽标右缘对齐分隔线右端（x=285）。也可以把整个展开内容（含分隔线）缩进到 x=49。修法与 UI-56 用同一条规则。

**可挽回分合计**：0.15 + 0.1 + 0.05 + 0.05 + 0.05 + 0.03 + 0.02 = **0.45**，正好等于 10 − 9.55。

## 4. 建议（不扣分）

- 页头状态胶囊按语义着色：失败用红，仅内存和暂停用琥珀，已保存用绿。目前全是中性灰边。
- 升级 Review 对话框：
  - ID 列改为左对齐，目前表头「ID」和值都居中在 x≈708。
  - 分节标题（「Saved themes retained (2)」等）与字段标签（「Library version」等）现在同为 13px/600，可以拉开层级。
  - 展开 JSON 后标题行吸顶。U4-upgrade-review-json 的对话框顶边（y≈13）把「Saved themes retained (2)」切掉了一半。
  - 右上角 × 与「Cancel upgrade」的语义要区分开。
- 冲突面板：
  - 推荐动作「Keep current draft」目前排在三个决策中间，可以放到首位或末位。
  - 差异里的颜色值 #123456 / #654321 旁边加色块。
  - 差异 ≤3 条时默认展开「Draft differences」。
- 展开摘要的字号统一：callout 内的「Technical details」「Draft differences」「View stored text」是 14px（大写字母高 10px），callout 外的「0 pending · …」「Cache and original backups」字高 9px，偏小。
- 成功 callout 加「Dismiss」。
- 取消态 callout 的两个下载按钮只靠「current」和「original」区分，后者可以改名为「Download pre-upgrade data」。

## 5. 看不出的格子

- **U4-quota-operations**：#3 行（失败的命名保存）及其「(draft revision 2)」是否断行，在 y&gt;900 处看不到。UI-56 断行余项无法核实。
- **U4-quota-failure / U4-quota-operations**：「Technical details」没有展开，看不出里面是否写了 QuotaExceededError 或存储已满。新-2 只按可见文字判断。
- **U4-upgrade-review**：焦点在 × 上（upgrade-review-close），但对话框是鼠标点开的，看不出键盘焦点环。对话框内的键盘路径没有拍到。
- **S2「模式/偏好变化不能绕过确认」**：没有拍到待确认期间切换预览模式或语言的帧。
- **S5「未处理的命名动作保持可见」**：冲突帧显示「No pending actions」，没有覆盖带 pending 命名动作的冲突。
- **S4「排队」「待锁」状态**：没有对应帧。
- **U4-upgrade-review-end**：与 U4-upgrade-review 字节相同（md5 都是 468b7f95…），因为对话框没有溢出、滚动无效。这不算缺帧，但 Review 底部只有这一帧可看。
- **U4-conflict-raw**：原文 textarea 宽约 253px，可见几行都放得下；更下方的长行是否横向溢出，看不出。
- 390 宽度只有待确认一帧（矩阵也只要求这一帧），其他状态在窄屏下的表现不在本轮证据里。

## 6. 总评

- 本次只评了 U4，所以 U4 也就是最低分单元：**9.55**（R3 为 8.6）。
- R3 遗留的 17 条差距，16 条已解决，UI-56 部分解决，没有回归。
- 剩下的失分主要集中在**配额态的动作层级**（新-1）。R3 的 UI-51 和 UI-140 两条修法叠加，造成同一个动作出现三个入口、三个实心蓝按钮。照新-1 收成「callout 管存储、命名卡管重试、页头降为描边」即可。
- 其余是文案去重、标题折行、冲突面板高出视口 35px，以及两处展开内容的左缘问题。
- 不需要结构性改动：布局、组件和状态模型都已符合 S1–S6，剩下的都是同一套组件内的取舍和文案。
