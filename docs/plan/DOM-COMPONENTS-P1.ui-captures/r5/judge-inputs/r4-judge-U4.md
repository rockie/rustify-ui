# R4 独立评审结论摘录 · U4

仅摘录 R4 judge.md 中本单元的评分、理由和全部未关闭差距；不含主 agent 裁定、机器检查和修复记录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U4 Studio 本地库迁移与存储反馈 | 2.9 | 2.95 | 2.85 | 0.85 | **9.55** | 8.6 |


- 布局 2.9：六种状态的存储 callout 位置与分隔统一；冲突键值改为上下两行；升级 Review 只剩一条分节线；390 页头按钮一行、callout 不溢出。扣分：冲突面板比 900px 视口高 35px，默认帧切掉「Download other version」；两处展开内容左缘不统一。
- 视觉 2.95：callout 与命名卡文字统一为两级（14px #1D2939 / 12px #475467）；对话框底部、无锁 callout 与 callout 内按钮样式统一；文字按钮与正文左缘对齐。扣分：配额态视口内三个实心蓝按钮抢主次。
- 交互 2.85：七个状态都能走通；取消后可下载当前主题；页头胶囊与 callout 区分「草稿已存、命名动作失败」；暂停与仅内存态页头 Save locally 降为描边；配额卡和升级后卡片有内联动作。扣分：配额态同一失败动作三个入口、三种叫法；取消态和无锁态命名卡的指引与按钮分在两张卡。
- 细节 0.85：R3 的文案问题都已修掉。扣分：callout 与命名卡成句重复；配额标题折行剩孤词「memory」且没写失败原因；「not saved earlier」措辞别扭。

## 待复核旧差距


#### UI-201 · [P2] 配额态：同一个失败的命名保存有三个重试入口、三种叫法，加上页头一共三个实心蓝按钮（U4 · 交互、视觉 · 可挽回 0.15）

- 位置：U4-quota-failure-1440-light 页头「Save locally」实心；红色 callout「Retry save」实心（y≈472–503）；命名卡「Retry retained saves · “Retained name”」实心（y≈674–706）；操作区「Retry retained saves」描边（y≈809–840）。此时只有 1 个 pending。
- 期望：S4 命名库动作单独反馈；S1 一个明确的继续动作；状态 callout 只管存储层。
- 修法：callout 去掉「Retry save」，由「Download current theme」做唯一实心按钮；命名卡按钮叫「Retry “Retained name”」；操作区改「Retry all retained saves」（描边）；页头在失败态降为描边。
- 来源：U4 新-1（评审注明由 R3 UI-51、UI-140 两条修法叠加造成）。

#### UI-203 · [P3] callout 和命名卡之间成句重复（U4 · 细节 · 可挽回 0.1）

- 位置：U4-upgraded「All saved themes and your current draft were retained.」后紧跟「2 saved themes retained. …」，命名卡「not saved earlier.」；U4-no-locks 标题、正文、命名卡三处同一意思；U4-quota-failure 标题与正文末尾都写「new saves stay in memory」。
- 修法：各处只说一次，按原文给出的改写；无锁命名卡可按 UI-205 并入 callout。
- 来源：U4 新-3。

#### UI-202 · [P3] 配额 callout 标题折行后第二行只剩孤词，也没有写出失败原因（U4 · 细节 · 可挽回 0.05）

- 位置：U4-quota-failure / U4-quota-operations 标题「Local save failed · new saves stay in」/「memory」（y=344/362）；正文「A later library action failed」。
- 期望：S6 明确原因；S4 点名失败的动作。
- 修法：标题改「Storage full · saving in memory」；正文点名「Saving “Retained name” failed and is kept for retry.」；标题加 `text-wrap: balance`。
- 来源：U4 新-2。

#### UI-204 · [P3] 冲突面板比 900px 视口高，默认帧切掉了「Download other version」（U4 · 布局 · 可挽回 0.05）

- 位置：U4-conflict-1440-light 面板从 y=323 起，「View stored text」在 y=867，下一个按钮只露顶边；底边对齐需 `scrollTop 35→335`（R3 为 `0→300`）。
- 期望：S1 出口始终可达；S5 展示冲突版本。
- 修法：首句与第一段补充说明并为一句；删去单独的「No pending actions…」段，有 pending 时再用一行 caption。
- 来源：U4 新-4。

#### UI-205 · [P3] 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里（U4 · 交互 · 可挽回 0.05）

- 位置：U4-upgrade-cancelled 命名卡（y≈580–711）只有「Dismiss」，正文让用户先 review upgrade；U4-no-locks 命名卡（y≈491–622）只有「Dismiss」，正文让用户 download。
- 期望：S1 每条常驻反馈带继续动作；S4。
- 修法：命名失败原因与 callout 状态相同时，并入 callout 末尾一行「Not saved: “…” · Dismiss」；或卡里放对应的描边按钮。
- 来源：U4 新-5。

#### UI-56 · [P3] 操作展开内容：说明段没有缩进，列表已缩进（U4 · 布局 · 部分解决 · 可挽回 0.03）

- 位置：U4-quota-operations 说明段（y≈805–841）从 x=21 起，列表从 x=34 起；#3 行在折线以下，修订号断行看不出。
- 修法：details 的内容容器整体缩进到摘要文字起点；修订号片段 `white-space:nowrap`。
- 来源：U4 UI-56（部分解决）。

#### UI-206 · [P3] 草稿差异块有三条左缘（U4 · 布局 · 可挽回 0.02）

- 位置：U4-conflict-diff-1440-light，x=33/34（callout 正文、三角、分隔线）、x=42（差异行）、x=49（摘要文字）。
- 修法：差异行去掉 8px `padding-inline`，与分隔线同起点；或整体缩进到 x=49。与 UI-56 用同一规则。
- 来源：U4 新-6。

