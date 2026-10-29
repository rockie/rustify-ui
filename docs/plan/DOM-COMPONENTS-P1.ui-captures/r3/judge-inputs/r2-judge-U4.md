# R2 评审结论 · U4 摘录

摘自 R2 合并结论 judge.md，只含 U4 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U4 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.4 | 2.3 | 2.4 | 0.45 | **7.55** | a | b 7.8 |


### 各维度理由

**U4（a）**

- 布局 2.4：升级 Review 改为对话框且结构化，冲突面板先版本与差异后决策，浮签不再遮挡。扣分原因：状态 callout 与决策 callout 叠放、内容重复，上下顺序在升级与冲突中相反；命名动作红字夹在 callout 之间；对话框两组键值的值列没对齐。
- 视觉 2.3：有警告、错误、成功三种语义变体，次级文字与错误色符合 G3。扣分原因：决策 callout 标题与对话框分节标题是 13px 常规 #475467，弱于 14px 正文；并排 callout 正文一个 12px 一个 14px；按钮有四套描边/禁用样式；命名动作反馈是不在容器里的红字。
- 交互 2.4：七个状态都能走通，每个状态都有常驻反馈、继续动作和下载出口。扣分原因：冲突三个决策没说明后果；升级成功后过期的红色命名失败仍挂着、无内联动作；无锁态标题写「paused」而主按钮仍实心；取消后 callout 沿用待确认文案；页头「Modified / Matches preset」与存储状态基准不一致。
- 细节 0.45：约十处文案与像素问题，包括「1 recent operations」、「edit 2」孤字、缺句号、时态、修订号无标签、对话框双细线。


## R2 未关闭差距（U4）

#### UI-108 · [P3] 粘性头下边框下方 17px 又有一条分隔线，形成双细线（U1/U4 · 细节 · 可挽回 U1 a 0.05，U4 b 0.05）

- 位置：U1-default-1440-light y=314 与 y=331；U1-default-768-light、U1-zh-top-1440-light、U1-zoom200-720-light 同理；U4 各 1440 帧 y=282/299 或 314/331。
- 修法：滚动区第一个分组去掉上边框，或头区只在内容滚动后显示下边框。
- 来源：U1 a 新-7；U4 b 新-5（同一对线）。U4 a 在建议中指出同一现象并归 U1。

#### UI-114 · [P3] 页头「Modified / Matches preset」的比较基准不清，与存储状态读法冲突；吸顶区没有保存状态（U1/U4 · 交互、细节 · 可挽回 U1 b 0.05，U4 a 0.1）

- 位置：U1-top-1440-dark「Graphite Modified」与下方「Saved on this device」；U4-upgrade-pending 页头「Matches preset」对话框「Draft state: Matches saved document」；U4-upgraded「Modified」紧挨「Saved on this device」。
- 修法：页头写明基准（「Modified from preset · N overrides」），吸顶区加按实际写入结果显示的保存状态胶囊；对话框用相同措辞。
- 来源：U1 b 新-12；U4 a 新-12（同一页头标签）。

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


## R2 建议（U4，不扣分）

- U4：
  - 冲突差异加色块（a、b）。
  - 对话框 × 与「Cancel upgrade」区分（a）。
  - 「Preview mode」移到偏好区（a）。
  - 成功 callout 加 Dismiss（a）。
  - 主题表加列头（b）。
  - 无锁态「Save locally」标注 memory only（b）。
