# R3 评审结论 · U7 摘录

摘自 R3 合并结论 judge.md，只含 U7 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U7 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 2.6 | 3.0 | 3.0 | 0.7 | **9.3** | a | — | 7.1 |

### 各维度理由

**U7（a）**

- 布局 2.6：Container 四档宽度与三种对齐正确；Heading 九档 wrap 都折成多行，行距与目标逐档一致；Inset top/bottom/x/y 与嵌套正确；Section 断点正确；390 下 Kbd 键帽不再相贴；显式尺寸在缩放下不变。扣分原因：Inset side=left/right 演示里正文压在贴边色块边上；Reset 的 input、textarea 两行脱离值列、占满整行。
- 视觉 3.0：Heading 为粗体；Text 四档字重墨量依次递增；Strong 加粗；字级梯度、Code 四变体、Kbd、Blockquote 都能与目标逐项对应。
- 交互 3.0：110%/95% 只缩放一次；点 Invalid prop 后出现带属性名、收到值和回退值的诊断，样例照常渲染；wrap、nowrap、balance、pretty、truncate 与四档 trim 都有可辨效果。
- 细节 0.7：Reset 区 button 行有一个没有标注的「0」；诊断文案把拒收值写了两遍、属性名带字面反引号；37×19 显式 CSS 样例完全看不见。

## R3 未关闭差距（U7）

#### UI-87 · [P2] Inset side=left/right 演示中正文压在贴边块边上（U7 · 布局 · 部分解决 · 可挽回 a 0.3）

- 位置：U7-layout-inset-demonstration-1280-light side=left 卡贴边块 y=47–207，正文大写字母顶部约 y=203；side=right 卡正文 y≈352–366，贴边块顶边 y≈362；390 与 U7-fixture-1280 y≈4740–5220、110% 帧同样存在。
- 现状 → 期望：贴边块与正文上下堆叠且重叠 → radix-inset 中 side=left/right 的图块贴三边、正文并排；L3 使内容贴边；L5 容器边界不遮住内容。
- 修法：只改夹具 Inset 区：side=left/right 两张卡改为 `Flex direction=row align=stretch gap=3`，贴边块给定宽度并随卡片拉伸，正文单独一列；right 镜像。验收为贴边块三边与卡片内沿重合、块内没有正文像素。
- 来源：U7 a UI-87（部分解决；评审将严重度从 P3 升为 P2）。

#### UI-189 · [P3] Reset 的 input、textarea 脱离值列，占满整行（U7 · 布局 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light「input · Reset」值列空，input 落到下一行 x=0–1200、y≈1070–1101；textarea 同样（y≈1277–1330）；390 同；390 下原生 blockquote 内容也掉到下一行。
- 修法：对照行改为两列 grid（`112px minmax(0,1fr)`，390 用 96px）；查 Reset 后 input/textarea 的宽度来源，Reset 不应附加原生没有的尺寸。
- 来源：U7 a 新-2。

#### UI-95 · [P3] 布局原语区仍有样例没有可见呈现（U7 · 细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light y≈1567「Box · width=37px · height=19px · padding=3px」下方全白；240px 非法样例在默认态与回退后完全一样。
- 修法：37×19 盒加 gray-a3 底或 gray-a7 边框并标「37×19」；240px 样例同样加底色，让 240px 与 auto 宽度可区分。
- 来源：U7 a UI-95（部分解决；标注已补齐）。

#### UI-190 · [P3] Reset 区 button 行右侧有没有标注的「0」（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light x≈259、y≈131；390 x≈245、y≈150；U7-fixture-1280 y≈1814。
- 修法：若是计数器，改为「clicks: 0」并用标注同款样式；否则删除。
- 来源：U7 a 新-3。

#### UI-191 · [P3] 非法属性诊断文案重复，并带字面反引号（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-invalid-detail-1280-light y=487、U7-invalid-1280-light y≈1700：「Box · invalid prop \`width\`: … received "12px;display:none". Rejected value: 12px;display:none. Fallback: width=auto. …」
- 修法：改成一句，如「Box · width: rejected "12px;display:none" (expected one CSS value) · fallback width=auto」；属性名用 Code soft size 1 而非字面反引号。
- 来源：U7 a 新-4。

## 建议（不扣分，U7）

  - Heading/Text/Blockquote 补色族行与 high-contrast 行。
  - Em 改用与 Quote 相同的衬线斜体。
  - Kbd 补多字符键，顶行键距加大。
  - Container 加嵌套样例；说明 `margin=3px` 的用意。
  - 「Your name」标签与输入框、Kbd 标注与键帽之间留 8px。
  - Code 表行高收紧；Wrap 与 Trim 组间距统一；夹具控制按钮显示当前状态；table 的 Reset 对照换成能看出效果的样例；「Box · 160 × 90px」盒子靠近标注；Code 表头 1280 与 390 写法统一。
