# R4 独立评审结论摘录 · U3

仅摘录 R4 judge.md 中本单元的评分、理由和全部未关闭差距；不含主 agent 裁定、机器检查和修复记录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.9 | 2.85 | 2.9 | 0.7 | **9.35** | 9.05 |


- 布局 2.9：所有对话框按钮行到下边框统一 24px；差异表表头下双线去掉，Change 列徽标与表头对齐。扣分：Import candidate 与 Changes 两张表上下叠放但列位置不同（Current 405 对 728，Imported 812 对 971，左缘 233 对 241）；Source 行比其他行矮 3px。
- 视觉 2.85：v1 损失清单和 Import notes 回到 14px/21px，token 与 G3 一致。扣分：同一对话框两套表头样式；6 个「Unchanged」与 1 个「Changed」徽标同样式；备注把整条路径排成等宽字。
- 交互 2.9：合法 v1 审阅、未知版本错误、v2/v1/CSS/Rust 导出、两个 CSS 下拉、复制成功都走通；候选区有「Field | Current | Imported」图例，Name 行两边都有值。扣分：「Theme JSON v1 · saved as v2」排在 Current 列下，像当前主题的属性。
- 细节 0.7：千分位、Rust 说明、CSS 选择器、ui-serif 断行、备注命名都已修好。扣分：差异表徽标基线低 2px；390 候选区徽标缩进 8px；Name 行两值基线差 1px；行距 29 对 32；备注路径的字体处理。

## 待复核旧差距


#### UI-128 · [P2] Source 行的值排在「Current」列下，读起来像是当前主题的属性（U3 · 交互、细节 · 部分解决 · 可挽回 0.1）

- 位置：U3-import-review-1440-light 行 y≈627–636，「Theme JSON v1 · saved as v2」从 x=405（Current 列）起排，Imported 列为空；390 单列卡片无此歧义。图例「Field | Current | Imported」与 Name 行两侧值已补上。
- 期望：X1 版本清楚归属导入候选。
- 修法：Source 行 Current 格写当前文档格式、Imported 格写「Theme JSON v1 · saved as v2」；或把 Source 改为表格上方跨列的说明行「Imported file: Theme JSON v1 · will be saved as v2」。中文同步。
- 来源：U3 UI-128（部分解决）。

#### UI-198 · [P3] 同一对话框里两张差异表的列结构、列位置和表头样式不一致（U3 · 布局、视觉、细节 · 可挽回 0.15）

- 位置：U3-import-review-1440-light y 595–607 候选区表头 12px 常规、无底线，列 x 233/405/812；U3-import-review-bottom-1440-light y 252–274 Changes 表头 13px/600、#D0D5DD 底线，列 x 241/572/728/971，另有独立 Change 列；390 小标题字重也不一。
- 期望：G3 label 13px/600；X1 同一套结构。
- 修法：候选配置行改用 Changes 的表格组件，两表共用 `Field | Change | Current | Imported` 列定义与 `th` 样式；或并成一张表，「Identity & configuration」作第一个分组行。
- 来源：U3 新-1。

#### UI-176 · [P3] 徽标基线偏低 2px；390 候选区徽标缩进 8px；Name 行两边基线差 1px（U3 · 布局、细节 · 部分解决 · 可挽回 0.14）

- 位置：U3-import-review-bottom-1440-light id / name / primary 行文字与徽标基线 332/334、373/375、451/453；U3-import-review-390-light Name 行徽标 x=207，同列值在 x=199；U3-import-review-1440-light Name 行两值基线 665/666。徽标列对齐与表头双线已修好。
- 修法：徽标 `inline-flex` 居中、固定高 24px；同行 `td` 统一 `vertical-align`；值与徽标包进 `flex-wrap` 容器，去掉徽标 `margin-left`。
- 来源：U3 UI-176（部分解决）。

#### UI-199 · [P3] 6 个「Unchanged」与 1 个「Changed」徽标同样式，且左缘参差（U3 · 布局、视觉、交互 · 可挽回 0.1）

- 位置：U3-import-review-1440-light y 651–866、U3-import-review-mid-390-light y 33–346；两种徽标同底同边同字色，Unchanged 左缘 456/461/451 三种 x。
- 期望：X1 突出候选差异。
- 修法：状态放进独立 Change 列并左对齐；「Unchanged」改纯文字 caption（或「—」）；只有 Changed/Added/Removed 保留带边框徽标，不染蓝。
- 来源：U3 新-2。

#### UI-200 · [P3] Import notes 把整条路径排成等宽字，与表格写法不一致（U3 · 视觉、细节 · 可挽回 0.08）

- 位置：U3-import-review-bottom-1440-light y 764–794、-390 y 664–740，「Light · base · font-serif」整段等宽，比同行 14px 正文小一号；表格写成无衬线分组头加等宽 token。
- 修法：模式与分组用正文字体，只有 token 用 `code`，`code` 字号 .9286em。中文同步。
- 来源：U3 新-3。

#### UI-25 · [P3] Source 行比其余候选行矮 3px（U3 · 布局、细节 · 部分解决 · 可挽回 0.06）

- 位置：U3-import-review-1440-light label 大写字母顶边 596/628/657/689/720/752/784/816/848，只有 Source→Name 为 29px。
- 修法：候选行 `min-height: 2rem` 或 `grid-auto-rows: minmax(2rem, auto)`，不靠徽标撑高；并表后统一 `td` 高度。
- 来源：U3 UI-25（部分解决）。

