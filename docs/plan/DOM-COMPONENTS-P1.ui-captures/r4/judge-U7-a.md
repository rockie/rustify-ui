# U7 布局与排版 · R4 评审

本轮证据：`r4/U7-*.png` 26 张，以及 `r4/U7-actions.log`。被测版本为 HEAD `5d122d62`，catalog build `32b52b66f04701ad`。

我先逐字节比对了 R3 与 R4 的同名截图。Heading、Text、Code、Blockquote、Wrap 五个分段在 1280 和 390 下都和 R3 完全相同，Kbd 1280 与 Container 390 也相同。其余变化集中在三处：Inset 演示、Reset 对照、布局原语区的显式尺寸样例与诊断行。

不变的分段我仍按目标图重新量了关键值：

- Heading 九档折行的行距为 60/40/36/30/26/24/22/18/16，与 radix-sink-heading 逐档相同。
- 缩放 110% 时 size 9 和 size 8 的行距为 66 和 44；缩放 95% 时 size 9 为 57。
- Code 四个变体的颜色处理与目标逐格对应。Code ghost 九档的行间距为 66/55/49/45/43/41/38/36，目标为 66/55/49/45/43/40/38/34。
- Kbd size 9 键帽为 86×85，目标为 88×86；size 1 两边都是 16px 高。
- Blockquote 在 size 7 下边条 7px、内距 14px。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 3.0 | 3.0 | 3.0 | 0.9 | **9.9** | 9.3 |

**布局 3.0（满档）**

- Container 四档内容宽度实测为 448/688/880/1136px，left、center、right 三种对齐正确（`U7-layout-container-matrix-1280`）。
- Inset 的 all、x、y、top、bottom、left、right、responsive 实测都只消去最近容器声明的那几边 padding。
- side=left/right 演示已改为并排：贴边块贴住三边，正文单列。1280、390、110%、95% 四种状态下都不再相压，卡片圆角处贴边块也按卡片圆角裁切。
- Reset 的 input、textarea 已回到值列。
- 显式尺寸在 110% 与 95% 下不变：240px、160×90、37×19、Container 1136 都保持原值。
- 剩下的只有像素级越界，归入细节。

**视觉 3.0（满档）**

- Heading 和 Strong 为粗体。Text 的 light、regular、medium、bold 四档，墨量均值依次为 0.908、0.881、0.851、0.814，逐档递增。
- 字级梯度与行高逐档等于目标。
- Code 的 solid、soft、outline、ghost × 主色、高对比、gray、gray 高对比，Kbd 的 classic、soft，Blockquote 的 accent-a6 边条与 0.5em 内距，都能与目标图逐项对应。
- 没有发现选阶偏差。

**交互与状态 3.0（满档）**

- 110%、95% 只缩放一次：Heading 行距按 1.10 和 0.95 变化，Section 的 64px 在 110% 下约为 70px，显式尺寸不变。
- 点 Invalid prop 后出现带属性定位、拒收值和回退值的诊断，样例照常渲染，并且回退后可以看出 auto 宽度。
- 比对 invalid 帧与默认帧：顶部 1640px 内只有 240px 盒那一行（y≈1628）不同；按底边对齐比下方 8000 行，只差 24 个像素，说明诊断没有扰动其他区域。
- wrap、nowrap、balance、pretty、truncate 与四档 trim 都有可辨效果。

**细节 0.9**

不是 1，因为逐处比对还剩两处像素级差异：

- Reset 的 input、textarea 虚线标记越过内容右沿约 5px，在元素截图里右边线被裁掉。
- 240px 样例的正文贴着左边框。

## 2. 上一轮差距复核表

| 编号 | R3 严重度 / 可挽回 | 本轮判定 | 依据（R4 截图位置） |
| --- | --- | --- | --- |
| UI-87 Inset side=left/right 正文压在贴边块上 | P2 / 0.3 | **已解决** | 见下方「UI-87 复核细节」 |
| UI-189 Reset 的 input、textarea 脱离值列、占满整行 | P3 / 0.1 | **部分解决** | 「脱离值列」已修复，残余为虚线标记越界，见下方「UI-189 复核细节」 |
| UI-95 布局原语区样例没有可见呈现 | P3 / 0.1 | **已解决** | 37×19 样例：`U7-invalid-detail-1280` 中填充区为 x40–76、y386–404，正好 37×19，外加 1px 边线。240px 样例：默认态填充 x24–263，正好 240，带底色和边线；点 Invalid 后变为 x39–1241 的整宽，能与 240px 区分 |
| UI-190 Reset 区 button 行有无标注的「0」 | P3 / 0.1 | **已解决** | `U7-layout-reset-comparison-1280` 第 167 行左右、x=112 处，改为「clicks: 0」，与标注同款小字，位于值列；390 分段与整段帧相同 |
| UI-191 诊断文案重复并带字面反引号 | P3 / 0.1 | **已解决** | `U7-invalid-detail-1280` 第 487 行改为一句：「Box · `width`: rejected "12px;display:none" (expected one CSS value) · fallback width=auto」。`width` 以 Code soft 小号芯片呈现。红字最深处约 #D92D20，对白底约 4.8:1 |

**UI-87 复核细节**

- `U7-layout-inset-demonstration-1280`：side=left 卡贴边块位于 x817–951、y48–284，与卡片内沿三边重合；正文从 x=963 开始，独立成列。side=right 卡贴边块位于 x249–383，正文从 x=25 开始。
- `-390` 分段：side=left 块位于 x1–115，正文从 x=127 开始；side=right 块位于 x219–333。
- `U7-scale110-1280` 第 4990–5400 行、`U7-scale95-1280` 第 4440–4700 行同样不再相压。
- 放大到 10 倍看卡片角，贴边块随卡片圆角裁切。

**UI-189 复核细节**

- 已修复部分：1280 分段 input · Reset 行的值从 x=112、y≈1084 开始，textarea 从 y≈1257 开始；390 分段同样落在值列。
- 残余：两者的虚线标记右缘越过内容区右沿约 5px。
  - 1280 整段帧里，虚线框为 x131–1229，而 Container 分隔线止于 x=1224。
  - 390 整段帧里，虚线框右缘在 x≈363，内容右沿约为 358。
  - 1280 和 390 的元素截图里，虚线框右边线都被裁掉。
- Reset 后控件拉满值列这一点，目标图 radix-reset 只显示为纯文字，看不出宽度，本轮不计。

## 3. 差距清单

两条都是 P3、可挽回分相同，按编号顺序列出。合计 0.1，等于 10 − 9.9。

#### UI-189（残余）· [P3] Reset 的 input、textarea 虚线标记越过内容右沿

- **单元与位置**
  - U7 · `U7-fixture-1280-light` 中 Reset 区 input · Reset 行（clip 第 2015 行左右）与 textarea · Reset 行（第 2170 行左右），虚线框右缘在 x=1229。
  - `U7-fixture-390-light` 同两行，虚线右缘在 x≈363。
  - `U7-layout-reset-comparison-1280/390` 中，两行虚线框的右边线被元素截图裁掉。
- **维度**：细节。
- **现状 → 期望**
  - 现状：其他行的虚线框四边都在内容区内，这两行右边线跑到内容区外，在分段截图里缺一条边。
  - 期望：每个 Reset 标记框四边完整、落在值列内。依据为 L5「容器边界不遮住内容」，以及比较口径中的「Reset 的作用范围」，标记框正是用来表明这个范围的。
- **修法**
  - 从截图推断：虚线框起点 x=131，比文字起点 x=136 左移 5px，标记应是画在元素外侧的 outline 加 offset。
  - 做法一：给 Reset 对照的值列单元格加 `padding-inline-end: 5px`，即 outline 宽度加 offset。
  - 做法二：只对块级 Reset 元素（input、textarea）把标记改为 `outline-offset: -1px`。
  - 两种做法都不要加包裹层，否则违反 L4。
  - 验收：1280 下虚线右缘 ≤ x=1224；390 下 ≤ 358；分段元素截图里能看到右边线。
- **可挽回**：0.05

#### 新-1 · [P3] 240px 显式宽度样例的正文贴着左边框

- **单元与位置**
  - U7 · `U7-fixture-1280-light` 第 1628–1651 行。「A bad prop keeps this view alive.」的首字 A 距左边框 x=23 只有约 1px，标注行与盒顶之间也只有 2–3px。
  - `U7-invalid-detail-1280` 第 438–461 行回退为整宽后同样如此；95%、110% 帧和 390 帧（clip 第 2245 行左右）都一样。
- **维度**：细节。
- **现状 → 期望**
  - 现状：文字压在边线上，读起来像被裁切。
  - 期望：样例的可读性与同区其他样例一致。「Box · padding=4」「160×90」都有内距。依据为 L6「组件默认保持可用」的演示要求，以及 R3 对 UI-95「让 240px 与 auto 可区分」的修法方向。
- **修法**
  - 给这个样例 Box 加显式 CSS 内距 `padding: 2px 8px`，不要用会随缩放变化的主题间距。
  - 保持 `box-sizing: border-box`，填充区仍为 240px。
  - 标注与盒之间加 4px 间距。
  - 验收：默认态填充仍是 x24–263；文字距左边框不少于 8px；Invalid 后仍为整宽。
- **可挽回**：0.05

## 4. 建议（不扣分）

以下各条在目标图里或者有对应但属于「示例文案、页面排列、字族」，或者目标图和规范要点里都找不到依据。

1. **R3 留下、仍然适用的建议**
   - Heading、Text、Blockquote 补色族行与 high-contrast 行。
   - Em 改用与 Quote 相同的衬线斜体。目标图 radix-sink-code 的 Em 为衬线斜体，但字族不在比较范围。
   - Kbd 补多字符键（⇧⌘V），顶行三键加大间距。目标约 15px，现状约 3px。
   - Container 加嵌套样例；说明 `margin=3px` 的用意。
   - 「Your name」标签与输入框之间、Code 和 Kbd 的说明与样例之间留 8px。目前两者相贴（`U7-fixture-1280` clip 第 5375、5495、5535 行左右）。
   - Code 表行高收紧：现状每行约 63px，目标约 44px。
   - Wrap 与 Trim 组间距统一。
   - 夹具控制按钮显示当前状态。
   - table 的 Reset 对照换成能看出效果的样例。
   - 「Box · 160 × 90px」的盒子靠近标注：现状盒子在 x=1048、离标注约 40px 且在最右端。
   - Code 表头写法统一：1280 写「color」，390 写「color=indigo」。
2. **新增：Reset 对照的标签换行。**「blockquote · native」「blockquote · Reset」在 1280 和 390 下都折成两行，行尾悬着「·」。可以把标签列从 112px 加宽到 128px，或给标签加 `white-space: nowrap`。
3. **新增：Reset 对照的行内对齐。** 多行值行（dl、textarea、blockquote）的标签按行居中，值按顶对齐，例如 1280 分段 textarea · Reset 标签在第 245 行，值在第 233 行。建议统一用 `align-items: start` 或基线对齐。

## 5. 看不出的格子

- **radix-reset 目标图**：所有元素都被 Reset 成纯文字，看不出 input、textarea 被 Reset 后的宽度。因此 Reset 后控件拉满值列这一点不判对错，也不扣分。
- **`U7-type-code-matrix-1280`**：表头只写「color」，看不出主色列是否显式传了 color。目标图主色列的 ghost 继承正文色，本轮主色列 ghost 为 accent-a11 蓝。390 分段同列写明「color=indigo」，据此认为 1280 也是显式传色，所以 ghost 显示蓝色与参考选阶一致，不计差距。若 1280 实际没有传色，就会和目标不一致，需要核实。
- **truncate 行**：截图只能看到省略号，看不出 DOM 是否保留完整文本（L2 要求）。
- **语义排版区**：「The native quotation remains a block quotation.」呈中粗，截图看不出是夹具传了 weight，还是 Blockquote 默认字重。分段里三档 Blockquote 都是常规字重，倾向前者。
- **未定义、未采的状态**：Restore prop 之后的帧，以及 390 下的缩放帧与非法态，目标包没有定义，本轮也没采，不作判断。

## 6. 总评

- U7 本轮 9.9，高于 R3 的 9.3。R3 的五条差距中四条已解决：UI-87、UI-95、UI-190、UI-191。UI-189 的结构问题（脱离值列）已修复，只剩虚线标记越界 5px。
- 最低分维度是细节（0.9），原因是两处像素级问题：Reset 块级控件的虚线标记越过内容右沿，240px 样例正文贴边。
- 两处都只需改夹具样式：值列加 5px 右内距或调整 outline-offset，240px 样例加显式内距。
- 字级梯度、行高、Code、Kbd、Blockquote 外观、Inset 贴边与 Reset 作用范围已与目标一致，不需要结构性改动。
