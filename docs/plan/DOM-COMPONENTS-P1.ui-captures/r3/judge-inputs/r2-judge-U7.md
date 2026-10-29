# R2 评审结论 · U7 摘录

摘自 R2 合并结论 judge.md，只含 U7 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U7 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 2.5 | 1.8 | 2.3 | 0.5 | **7.1** | a | — |


### 各维度理由

**U7（a）**

- 布局 2.5：Container 四档 448/688/880/1136px 与三种对齐正确，Inset top/bottom 贴边，390 无溢出，240px 显式尺寸在 110%/95% 下不变。扣分原因：非法属性样例默认态塌成一列一词并有孤立 `px`；Heading wrap 行、Inset 左右贴边、Reset 元素覆盖面未达目标结构。
- 视觉 1.8：字级梯度、已测行高、Code 四变体、Kbd classic 斜面与 soft、Blockquote 边框随字号、Quote 衬线斜体正确。扣分原因：Heading 只比正文略粗；Text 的 light/regular/medium/bold 四行完全一样；Strong 不加粗。
- 交互 2.3：缩放 110%/95% 只生效一次，字号与行高按比例变化，wrap/trim/truncate 有呈现。扣分原因：非法属性态完全没有属性定位诊断；pretty 效果看不出。
- 细节 0.5：顶边外来灰线、孤立 `px`、布局原语区无 prop 标注、390 下 Kbd 键帽上下相贴。


## R2 未关闭差距（U7）

#### UI-152 · [P1] Text 的四档字重渲染完全相同（U7 · 视觉 · 可挽回 a 0.5）

- 位置：U7-type-text-matrix-1280-light y=537–666 的 weight=light/regular/medium/bold 四行；390 同。
- 现状 → 期望：四行墨量 0.8541/0.8541/0.8541/0.8536 → L2 字重 light/regular/medium/bold 依次加重（比较口径「字重」）。
- 修法：先查四行 computed `font-weight`；数值相同则补齐字重 class 并 `mbx xtask css`；数值不同但渲染相同则补齐 IBM Plex Sans 300/400/500/700 的 `@font-face`（public 静态 woff2）。修后 UI-88、UI-89 应同时恢复。
- 来源：a 新-1。

#### UI-88 · [P1] 〔待裁定〕Heading 不是粗体（U7 · 视觉 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-type-heading-matrix-1280/390、U7-scale110/95-heading，夹具「Native structure, flexible composition」。
- 现状 → 期望：60px 下 Heading 笔画 7px、Text 6px；目标 Heading 9px、Text 5px（L1 正确语义外观）。
- 修法：Heading 默认 bold（700）并确认 700 字面可加载，验收为 60px 下 Heading 笔画约为 Text 的 1.5 倍以上。
- 来源：a UI-88（仍存在）。

#### UI-89 · [P1] 〔待裁定〕Strong 没有加粗（U7 · 视觉 · 仍存在 · 可挽回 a 0.2）

- 位置：U7-type-code-matrix-1280「strong importance」墨量 0.8595 对相邻 0.8577；夹具「Strong importance」。
- 来源：a UI-89（仍存在）。

#### UI-91 · [P2] 非法属性后看不到属性定位诊断（U7 · 交互 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-invalid-1280-light 与默认态逐像素比对，只有该样例重新排版；U7-invalid-detail-1280-light 视口里没有诊断。
- 现状 → 期望：L6 非法值给出属性定位诊断，组件默认保持可用。
- 修法：样例下方渲染 `role="status"` 诊断行（属性、非法值、回退默认值），Restore 后消失。
- 来源：a UI-91。

#### UI-153 · [P2] 非法属性样例默认态塌成一列一个词，上方有孤立的 `px`（U7 · 布局、细节 · 可挽回 a 0.25）

- 位置：U7-fixture-1280-light 约 y=1395–1590；U7-fixture-390-light 约 y=1940–2120；U7-invalid-* 中「px」仍在。
- 修法：默认态给合法值并标注（如 `width=240px`），补全或删除「px」。
- 来源：a 新-2。

#### UI-92 · [P2] Reset 只覆盖 button、h2、ul 三种元素（U7 · 布局、交互 · 部分解决 · 可挽回 a 0.2）

- 位置：U7-layout-reset-comparison-1280/390；对照 radix-reset 约 60 种原生元素。
- 修法：沿用「原生 / Reset」双行，至少补 a、strong、em、code、kbd、mark、sub、sup、input、select、textarea、fieldset、ol、dl、table、pre、blockquote、img。
- 来源：a UI-92 余项。

#### UI-95 · [P3] 布局原语区仍有样例没有 prop 标注（U7 · 细节 · 部分解决 · 可挽回 a 0.15）

- 位置：U7-fixture-1280 约 y=120–1380；Section 只有一条无标注灰带。
- 来源：a UI-95 余项。

#### UI-86 · [P3] Heading wrap 行没有收窄宽度，1–3 档多行行高无法比对（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-type-heading-matrix-1280 size=7·wrap 至 size=1·wrap 只有一行；390 下 3–1 档一行。
- 来源：a UI-86 余项（主体已补齐，余项按 P3 计）。

#### UI-87 · [P3] Inset 的 left、right 贴边没有干净的展示（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-layout-inset-demonstration 只有 top/bottom；U7-fixture-1280 约 y=1015–1320 的 8 个 side 样例都带 `margin=3px`。
- 来源：a UI-87 余项。

#### UI-97 · [P3] 〔待裁定〕夹具根顶边混入外来灰线（U7 · 细节 · 仍存在 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light 第 0 行 x=138–288、295–374（rgb 118–161）；390 图没有。
- 来源：a UI-97（仍存在）。

#### UI-154 · [P3] wrap=pretty 与 wrap=wrap 折行完全相同（U7 · 交互 · 可挽回 a 0.1）

- 位置：U7-type-wrap-matrix-1280/390。
- 修法：换一段在当前卡宽下 wrap 会留孤词的文案，或收窄卡片。
- 来源：a 新-5。

#### UI-155 · [P3] 390 下 Kbd size 9 行换行后键帽上下相贴（U7 · 布局 · 可挽回 a 0.05）

- 位置：U7-type-kbd-matrix-390-light 约 y=172–345。
- 来源：a 新-4。


## R2 建议（U7，不扣分）

- U7：
  - Heading/Text/Blockquote 补色族行与 high-contrast 行。
  - Em 与 Quote 字族一致。
  - Kbd 补多字符键。
  - 顶行 Kbd 间距加大。
  - Container 加嵌套样例。
  - 说明 `margin=3px` 样例是有意的。
  - 「Your name」标签与输入框留间距。
  - Code 表格行高收紧。

