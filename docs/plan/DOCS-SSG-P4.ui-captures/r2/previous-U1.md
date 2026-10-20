# R1 U1 原结论逐字摘录

| U1 搜索主路径 | 3.0 | 3.0 | 2.1 | 0.9 | 9.0 |
- U1：已见输入、状态、总数、列表、更多按钮、窄屏换行符合布局；浅深主题及焦点色符合规范；Show more 激活后丢失实际键盘焦点，交互扣 0.9；窄屏宽度细节扣 0.1。完整理由见 judge-U1-a.md。
- U1：可考虑高亮摘要中全部可见命中、为截取摘要加省略号；规范没有要求，不额外扣分。

### UI-01 · [P1] Show more 激活后键盘焦点丢失（U1 · 交互与状态 · 可挽回 0.9）

- 位置：U1-keyboard-more-focus-03-1280-light.png 与 U1-keyboard-more-04-1280-light.png，底部 Show more；对应操作记录的 focused.tag 从 BUTTON 变为 BODY。
- 现状 → 期望：20→40 条结果已经追加，但可见焦点环消失 → 焦点应保留在对话框内可继续操作的控件，且保留当前结果选择。依据 G3、S2、S3。
- 修法：更新结果时保留按钮节点，或在更新后恢复至新的 Show more；最后一批移除按钮时明确把焦点送回输入或等价位置。不重置当前选择。
- 复验：Tab 到 Show more，连续 Enter 完成20→40→60；每步焦点可见、选择不变、Tab/Shift+Tab仍可用。
- 来源：U1 评审 A 新-1。



### UI-02 · [P3] 窄屏 dialog 比规定宽度少 6px（U1、U2）

- 位置：U1-idle/results/empty 的375/320浅深图，U2-loading-375-light.png、U2-supp-longlist-04-375-light.png等375图。
- 现状 → 期望：375视口外宽约337px、320约282px，两侧约19px → S1规定左右各16px，即343px、288px；桌面576px符合。
- 修法：统一 dialog 的最终 inline-size/max-inline-size，采用 min(36rem, calc(100vw - 2rem)) 与 border-box，排除原生最大宽度的二次收窄；保留窄屏1rem内边距及换行。
- 可挽回分：U1细节0.1；U2采用B组的布局0.1。A原文U2细节0.2保留，不与B重复相加。
- 来源与编号映射：U1 A 新-2、U2 A 新-1、U2 B 新-2；同一原生 dialog 在同类窄视口出现相同6px收窄，合并同一差距。

