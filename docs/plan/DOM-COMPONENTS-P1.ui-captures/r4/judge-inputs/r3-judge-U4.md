# R3 评审结论 · U4 摘录

摘自 R3 合并结论 judge.md，只含 U4 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U4 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.75 | 2.7 | 2.5 | 0.65 | **8.6** | a | b 8.7 | 7.55 |

### 各维度理由

**U4（a）**

- 布局 2.75：每个状态只有一张 callout，存储区顺序在六种状态下固定；冲突面板在 1440×900 下整块可见；升级 Review 对话框值列对齐、主题表有表头；页头吸顶并带保存状态胶囊。扣分原因：配额态「Technical details」移到了红色 callout 之外（比 R2 退步）；冲突键值栅格值列只有 128px，修订行折出孤词；对话框里仍有一对双细线；差异块一处改动占四行。
- 视觉 2.7：callout 标题统一为 13px/600 并按变体着色；成功态有绿色底和边框；禁用样式统一；原文 textarea 有边框和底色。扣分原因：callout 正文在 12px #475467 与 13px #1D2939 之间切换，同一张冲突 callout 内也混用；对话框次级按钮是蓝边蓝字，与壳层灰描边两套样式并存；文字按钮标签比正文缩进 11px。
- 交互 2.5：七个状态都能走通，各有常驻反馈和继续动作；冲突的三个决策写明后果；升级成功后命名动作降为中性提示并附 Save now、Dismiss；无锁态 Save locally 降为描边。扣分原因：取消后 callout 让用户「Download your current theme」，callout 里却只有「Download original data」；暂停态页头 Save locally 仍是实心主按钮；配额态整体说法与操作列表矛盾；配额命名卡没有内联重试。
- 细节 0.65：取消态 callout 两段互相重复；「Saved theme count 0 → 0」；成功态「before upgrade」时态不对；胶囊与 callout 用词不一致，「upgrade」「migration」混用；操作说明段讲本状态没有的「Load other version」；双细线与缩进问题。

## R3 未关闭差距（U4）

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

## 建议（不扣分，U4）

  - 页头状态胶囊按语义着色。
  - 主题名与基准同名时写明基准类型。
  - 冲突推荐动作放在首位或末位；冲突差异颜色值旁加色块。
  - 升级 Review 对话框的 × 与「Cancel upgrade」语义区分。
  - 对话框分节标题与字段标签区分层级；成功 callout 加 Dismiss。
  - 差异条目很少（≤3 条）时默认展开「Draft differences」（b）。
  - 升级 Review 展开 JSON 后标题行吸顶（b）。
  - 主题表 ID 列改为左对齐（b）。
