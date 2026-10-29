# U7 布局与排版 · R3 UI 验收评审

评审依据：目标包 U7 行与规范要点 L1–L6（含 2026-10-27 R2 之后的 UI-156、UI-97、UI-88/89 裁定）、9 张目标图、r3 的 `U7-*` 截图与 U7-actions.log、r2 的 `U7-*` 截图，以及 r2-judge-U7.md。像素数值是我对 PNG 裁切后取样、计算墨量得到的。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 2.6 | 3.0 | 3.0 | 0.7 | **9.3** |

各维度理由：

- **布局 2.6**
  - 做对的部分：
    - Container 实测宽 448/688/880/1136px，left/center/right 三种对齐正确。110% 下仍是 448px。
    - Heading 九档 wrap 现在都折成两行及以上。实测行距 60/40/36/30/26/24/22/18/16px，与 radix-sink-heading 逐档一致。
    - Inset top/bottom/x/y 与嵌套样例正确。Section 在 1280 为 64px，在 390 为 24px，符合断点。
    - 390 下 Kbd 键帽不再相贴。显式 240×72px 在 110%/95% 下不变。
  - 不到 3 的原因：Inset 新增的 side=left/right 演示里，正文压在贴边色块边上（1280 与 390 都有）；Reset 的 input/textarea 两行脱离了值列，占满整行。
- **视觉 3.0（满档）**
  - 上一轮的三个 P1 都已修复：Heading 为粗体，size 9 的「T」竖笔约 9–10px，与目标同量级；Text 的 light/regular/medium/bold 墨量依次为 0.089/0.115/0.144/0.181；Strong 墨量 0.231，相邻正文 0.159。
  - 字级梯度、Code 四变体 × high-contrast × gray、Kbd classic 斜面与 soft、Blockquote 边框宽随字号（4/4/7px）及 accent-a6 用色，都能与目标逐项对应。
  - 没有发现超出「细节」档的视觉差异。
- **交互与状态 3.0（满档）**
  - 110%/95% 只缩放一次：size 9 wrap 行距 66/57px，size 8 为 44/38px。
  - 点 Invalid prop 后出现带属性名、收到值和回退值的红字诊断，样例照常渲染。与默认态逐像素比，只多出诊断这一段及其后的位移。
  - wrap/nowrap/balance/pretty/truncate 与四档 trim 都有可辨效果；pretty 已与 wrap 折行不同。
  - 未拍到的 Restore 不计分。
- **细节 0.7**
  - 有若干小差异：Reset 区 button 行有一个没有标注的「0」；诊断文案把拒收值写了两遍，属性名带字面反引号；显式 CSS 37×19 样例完全看不见。

## 2. 上一轮差距复核表

| 编号 | 上一轮 | 本轮状态 | 依据 |
| --- | --- | --- | --- |
| UI-152 | P1 Text 四档字重完全相同 | **已解决** | U7-type-text-matrix-1280 y=533–670 四行墨量 0.089/0.115/0.144/0.181，依次递增；四行最深像素都是 #1C2024，没有用改色冒充改字重 |
| UI-88 | P1 Heading 不是粗体 | **已解决** | U7-type-heading-matrix-1280 size 9 竖笔约 9–10px，与 radix-sink-heading 一致；U7-scale110/95-heading 同样是粗体 |
| UI-89 | P1 Strong 没有加粗 | **已解决** | U7-type-code-matrix-1280 内联行「strong importance」墨量 0.231，相邻「Inline…」0.159；U7-fixture-1280「Strong importance」也是粗体 |
| UI-91 | P2 非法属性后看不到诊断 | **已解决** | U7-invalid-detail-1280 y≈487 出现红字诊断：属性 `width`、收到值、Fallback width=auto、Component remains usable。#D92D20 对白底约 4.8:1。与默认态逐像素比，差异只在 y≈1670–1891，即诊断插入及其后位移。Restore 后是否消失没有拍到，见第 5 节 |
| UI-153 | P2 非法样例塌成一列一词，有孤立 `px` | **已解决** | U7-fixture-1280 y≈1620 处「A bad prop keeps this view alive.」为单行，上方有标注；`px` 已消失。剩下的「显式盒不可见」并入 UI-95 余项 |
| UI-92 | P2 Reset 只覆盖 3 种元素 | **已解决** | U7-layout-reset-comparison 现有 21 对原生/Reset：button、h2、ul、a、strong、em、code、kbd、mark、sub、sup、input、select、textarea、fieldset、ol、dl、table、pre、blockquote、img，与上轮要求的清单一致。修复带出的排版问题另列为新-2 |
| UI-95 | P3 布局原语区样例缺 prop 标注 | **部分解决** | 每个样例都有标注，Section 也有「size=1 / xs=2 / md=3 / xl=4 (24/40/64/80px)」。余项：37×19 显式 CSS 样例没有任何可见像素 |
| UI-86 | P3 Heading wrap 行未收窄 | **已解决** | 1280 下九档全部折两行；390 下 size 9 折四行。逐档行高与目标一致（数值见第 1 节「布局」） |
| UI-87 | P3 Inset left/right 没有干净的展示 | **部分解决，严重度升为 P2** | Inset 区新增了 side=left/right 卡片，但正文与贴边块重叠，详见下方清单 |
| UI-97 | P3 夹具根顶边混入外来灰线 | **已解决**（采集口径按裁定改变后） | U7-fixture-1280 第 9 行起整行 #FFFFFF；灰像素 #767676 只在 y=8，属于根外留边里的宿主控件下沿 |
| UI-154 | P3 wrap=pretty 与 wrap 折行相同 | **已解决** | U7-type-wrap-matrix-1280：pretty 折为「…room / for words…」，wrap 折为「…room for / words…」 |
| UI-155 | P3 390 下 Kbd size 9 上下相贴 | **已解决** | U7-type-kbd-matrix-390 size 9：soft 上下键约 8px，classic 斜面阴影到下一键约 3–4px（目标横向键距约 4px） |

## 3. 差距清单（未关闭）

#### UI-87 · [P2] Inset side=left/right 演示中正文压在贴边块边上（布局 · 可挽回 0.3）

- **位置**
  - U7-layout-inset-demonstration-1280-light，side=left 卡（x=817–1199，y=47–284）：贴边块 x=817–1175、y=47–207。下方正文「Content keeps the card's declared padding.」大写字母顶部约在 y=203，压进色块约 4–5px。
  - 同图 side=right 卡（x=0–383，y=310–522）：正文在 y≈352–366，贴边块顶边在 y≈362，p、g 的下伸部落在色块里。
  - 390 同样复现：U7-layout-inset-demonstration-390-light 的 side=left 卡在 y≈777 处，side=right 卡在 y≈954 处。
  - 整段帧：U7-fixture-1280-light 约 y=4740–5220，110% 帧同样存在。
- **现状 → 期望**
  - 现状：left/right 的贴边块和正文上下堆叠。贴边块在顶边（left）或底边（right）消掉了 padding，把相邻正文吸到块边上。结果是 left 块不到底，right 块不到顶，文字与色块重叠。
  - 期望（radix-inset.png）：
    - side=left 对应 Checkbox Card：图块 x=72–136 贴左、上、下三边，正文在右侧，与图块留出间距。
    - side=right 对应 Radio Card：图块 x=744–807 贴右、上、下三边。
    - L3「使内容贴边」，L5「容器边界不遮住内容」。
- **修法**
  - 只改夹具的 Inset 区，不改 Inset 组件：
    - 把 side=left/right 两张卡的内容改为 `Flex direction=row align=stretch gap=3`。
    - side=left 卡依次放：`Inset side=left`，给定宽度（如 width=120px）、gray/accent-a3 底或一张 public 图片，高度随卡片拉伸；再放一个只装正文的 Text 列。
    - side=right 卡镜像放置。
  - 验收：
    - 贴边块 bbox 三边与卡片边框内沿重合；
    - 贴边块 bbox 内没有正文像素；
    - 1280、390 与 110% 帧都满足。

#### 新-2 · [P3] Reset 的 input、textarea 脱离值列，占满整行（布局 · 可挽回 0.1）

- **位置**
  - U7-layout-reset-comparison-1280-light：「input · Reset」标签在 y≈1048，同行值列是空的；Reset 后的 input 落到下一行，dashed 框 x=0–1200、y≈1070–1101。「textarea · Reset」同样落行，y≈1277–1330，宽 1200、高约 53px。
  - 390：同图 y≈1063–1115 与 y≈1270–1350。
  - 同类：390 下「blockquote · native」内容掉到标签下一行，y≈1908–1966。
- **现状 → 期望**
  - 现状：21 对里只有这两对（以及 390 下的原生 blockquote）离开 x=108 的值列，从 x=0 起排，破坏了「原生 / Reset」逐行对照的结构。
  - 期望：每个 Reset 元素都留在自己标签同一行的值列里。在 radix-reset.png 里，「Input control」「Multi-line textarea」同其他元素一样处在正文流中（L4：只作用于单个原生元素，不改变宿主结构）。
- **修法**
  - 把对照行从会换行的 flex 改为 `display:grid; grid-template-columns: 112px minmax(0,1fr)`，390 用 96px 首列，值固定在第 2 列。
  - 再查 Reset 后 input/textarea 的 computed `display`/`width`：
    - 宽度来自 Reset 规则（例如 width:100%）：Reset 不应附加原生没有的尺寸，应改为 `width:auto`，或保持原生 size。
    - 宽度来自夹具：在夹具里给这两个元素 `max-width` 与原生同宽。

#### UI-95 · [P3] 布局原语区仍有样例没有可见呈现（细节 · 部分解决 · 可挽回 0.1）

- **位置**
  - U7-fixture-1280-light y≈1567 标注「Box · width=37px · height=19px · padding=3px (explicit CSS)」，下方 y≈1575–1610 全白，没有任何像素。
  - U7-invalid-detail-1280 y=376–420 相同；390 整段帧对应位置相同。
  - 同类：「Box · width=240px」非法样例在默认态（240px）与回退后（auto）完全一样，回退结果看不出来（U7-fixture 与 U7-invalid 在 y≈1600–1660 逐像素相同）。
- **现状 → 期望**
  - 现状：显式尺寸盒没有底色、没有边框，L6「显式 CSS 尺寸不随缩放变化」在 110%/95% 帧里无从核对。
  - 期望：每个布局原语样例都能从截图上读出它的尺寸与效果（上轮 UI-95 要求的延续）。
- **修法**
  - 给 37×19 盒加 `gray-a3` 底或 1px `gray-a7` 边框，内放「37×19」字样（Text size 1）。
  - 给 240px 非法样例同样加 `gray-a3` 底，让 240px 与回退后的 auto 宽度在图上能区分。
  - 验收：110%/95% 帧中该盒仍是 37×19。

#### 新-3 · [P3] Reset 区 button 行右侧有没有标注的「0」（细节 · 可挽回 0.1）

- **位置**：U7-layout-reset-comparison-1280-light x≈259、y≈131，「Native reset action」右侧；390 图 x≈245、y≈150；U7-fixture-1280 y≈1814。r2 图里已存在。
- **现状 → 期望**：一个孤立数字，读者无法知道它的含义（点击计数？），不在任何目标图与规范中。期望：样例区不出现没有标签的数值。
- **修法**
  - 如果是演示 Reset 后按钮仍可点击的计数器，改成「clicks: 0」，用 Text size 1、gray-11，与左侧标注同一样式；
  - 否则删除。

#### 新-4 · [P3] 非法属性诊断文案重复，并带字面反引号（细节 · 可挽回 0.1）

- **位置**：U7-invalid-detail-1280-light y=487（x=40–1128）；U7-invalid-1280-light y≈1700。
- **现状 → 期望**
  - 现状：「Box · invalid prop \`width\`: expected one valid CSS value, received "12px;display:none". Rejected value: 12px;display:none. Fallback: width=auto. Component remains usable.」
    - 同一个值出现两次，一次带引号、一次不带；
    - Markdown 反引号直接显示在界面上。
  - 期望：L6「属性定位诊断」用一句话说清楚属性、非法值、回退值。
- **修法**：改成一条，例如「Box · width: rejected "12px;display:none" (expected one CSS value) · fallback width=auto」。属性名要强调时用 Code soft size 1，不要用字面反引号。

**可挽回分合计**：0.3 + 0.1 + 0.1 + 0.1 + 0.1 = 0.7，等于 10 − 9.3。

## 4. 建议（不扣分）

- Heading/Text/Blockquote 补色族行（Red/Yellow/Green/Gray）与 high-contrast 句子行，对应 radix-sink-heading y≈1575–1718、radix-sink-text y≈578–702、radix-sink-blockquote 第 2、3 段。
- Em 改用与 Quote 相同的衬线斜体。目标 radix-sink-code 中「subpixel rendering」是衬线斜体；字族不在比较范围内。
- Kbd 矩阵补多字符键（目标是「⇧⌘V」）；顶行 Enter/Tab/Shift+Tab 间距由约 3px 加到目标的 8–15px。
- Container 加嵌套样例：size 4 内嵌 size 1，对应 radix-sink-container。
- side=* 样例的 `margin=3px` 已写在标注里，可再补一句「3px 外边距用于露出卡片边」。
- 「Your name」标签与输入框之间间距为 0；「Kbd · variant=classic (default) · size=2」标注与键帽只隔约 2px。建议都留出 8px 间距。
- Code 表行高约 64px，目标约 44px，可收紧。
- Wrap 组与 Trim 组之间间距 16px，组内 24px，可统一。
- 夹具控制按钮（Scale 100/110/95、Invalid/Restore）没有显示当前生效状态，可加 `aria-pressed` 和选中态。
- table 的 Reset 对照与原生几乎没有差别（只差 3px），可换成带边框或 caption 的表格，让 Reset 效果可见。
- 「Box · 160 × 90px」样例的盒子在 x=1048 远离标注，左侧空出约 1000px，可说明偏移原因，或让盒子靠近标注。
- Code 表头在 1280 写「color」，在 390 写「color=indigo」，建议统一。

## 5. 看不出的格子

- **U7-invalid**：没有点 Restore prop 之后的帧，诊断能否消失、样例能否恢复 240px 看不出。U7-actions.log 只记录到「click Invalid prop」。
- **诊断的语义**：截图看不出诊断行是否为 `role="status"` 或已关联到样例。
- **Text weight=medium**：墨量位于 regular 与 bold 之间，但从像素无法判定渲染的是 500 还是 600，目标图也没有 medium 样例可比。
- **truncate**：「保留完整文本」（L2）属于 DOM/可访问名称层面，截图只能看到省略号。
- **语义结构**：截图无法验证 L1/L3/L4 的语义部分，包括 Heading 层级与视觉大小无关、Box/Flex/Grid 保持 div/span 语义、Reset 不加包裹层。
- **缩放态与非法态的视口覆盖**：缩放与非法态只在 1280 采集（采集表本来只定义了 1280），390 下的缩放与非法态没有图。

## 6. 总评

- **分数与主因**：U7 为 9.3，比上一轮 7.1 明显上升。上轮三个 P1 字重问题（UI-152、UI-88、UI-89）全部修复，诊断、Reset 覆盖面、Heading wrap 等 P2/P3 也基本关闭。剩下的失分主要来自 UI-87：Inset side=left/right 的演示结构不对，正文与贴边块重叠。
- **需要结构性改变的地方**
  - Inset 区 left/right 两张卡要从纵向堆叠改为横向 Flex，做成 radix-inset 里 Checkbox/Radio Card 的结构：贴边块占满一侧全高，正文并排。只调间距修不好。
  - Reset 对照区的行容器要从会换行的 flex 改为两列 grid，并确认 Reset 后的 input/textarea 没有被加上整行宽度。
- 其余都是文案和样例可见性的小修。
