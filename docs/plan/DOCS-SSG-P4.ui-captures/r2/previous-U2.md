# R1 U2 原结论逐字摘录

| U2 搜索加载与恢复 | 2.9 | 3.0 | 3.0 | 0.5 | 9.4 |
- U2：采用补采评审 B 的完整四维组，未逐维拼接。A 为 3/3/3/0.8＝9.8；B 为 2.9/3/3/0.5＝9.4。状态顺序、按钮、内部滚动与换行正确；窄屏宽度扣布局 0.1；主题/焦点/选择符合目标，已见加载中编辑、两类恢复及键盘闭环符合交互；“1 results”扣细节 0.5。完整理由分别见 judge-U2-a.md、judge-U2-b.md。

### UI-04 · [P3] 单条英文结果显示“1 results”（U2 · 细节 · 可挽回 0.5）

- 位置：U2-module-retry-375-light.png、U2-index-retry-1280-dark.png、U2-supp-old-released-after-new-10-1280-dark.png，输入框下方总数。
- 现状 → 期望：“1 results” → “1 result”，98条仍为“98 results”；依据 S2/S3 及细节维度的文案准确性。
- 修法：数量格式化处选择正确英文单复数，保持全部匹配页数与 live 更新语义。
- 来源：U2 补采评审 B 新-1。评审 A 将其列为建议；两份原文均保留，合并保留 B 的差距。



### UI-02 · [P3] 窄屏 dialog 比规定宽度少 6px（U1、U2）

- 位置：U1-idle/results/empty 的375/320浅深图，U2-loading-375-light.png、U2-supp-longlist-04-375-light.png等375图。
- 现状 → 期望：375视口外宽约337px、320约282px，两侧约19px → S1规定左右各16px，即343px、288px；桌面576px符合。
- 修法：统一 dialog 的最终 inline-size/max-inline-size，采用 min(36rem, calc(100vw - 2rem)) 与 border-box，排除原生最大宽度的二次收窄；保留窄屏1rem内边距及换行。
- 可挽回分：U1细节0.1；U2采用B组的布局0.1。A原文U2细节0.2保留，不与B重复相加。
- 来源与编号映射：U1 A 新-2、U2 A 新-1、U2 B 新-2；同一原生 dialog 在同类窄视口出现相同6px收窄，合并同一差距。

