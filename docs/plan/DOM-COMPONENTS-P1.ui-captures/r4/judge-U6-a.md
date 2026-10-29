# U6 评审原文 · R4（Button / IconButton / Link 与内嵌 ThemePanel）

只评 U6，其他单元未评。依据：
- 目标包 `docs/plan/DOM-COMPONENTS-P1.ui-target.md`。
- 4 张目标图。
- r4 本层的 27 张 `U6-*.png`、`r4/U6-actions.log` 和 `r4/capture.md`。
- r3 的 `U6-*.png` 与 `r4/judge-inputs/r3-judge-U6.md`。

**R4 与 R3 的哈希比对**：27 张图里有 21 张与 R3 字节完全相同，有变化的 6 张是：
- `fixture-1280-dark`
- `fixture-1280-light`
- `fixture-390-light`
- `focus-classic-…-dpr2`
- `radius-full`
- `scale110`

用 `magick compare` 做差分，这 6 张的差异都只落在 classic 一行，例如 1280-dark 的差异框是 452×48+148+137。classic 行的非背景轮廓（silhouette）与 R3 逐像素相同（AE = 0），所以几何没有变，变的只是填充的明暗。

**下文亮度的算法**：Y ≈ 0.2126R + 0.7152G + 0.0722B，直接用 sRGB 值计算。

---

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.95 | 3.0 | 1.0 | **9.95** | 9.9 |

**布局 3.0**
- 我在 `U6-fixture-1280-dark` 上逐列量了高度，全部与目标一致：
  - classic 四档：24 / 32 / 40 / 48。
  - 4 档的 solid、soft、surface、outline：都是 48。
  - 抽查 solid 3 = 40、soft 2 = 32、surface 1 = 24、outline 2 = 32。
- 110% 下，`U6-scale110`：classic 26 / 35 / 44 / 53，solid 4 和 outline 4 都是 52，与 48×1.1 = 52.8 的取整一致。
- IconButton「×」为 32×32 的正方形。
- loading 时三个按钮宽 123 / 111 / 95，与默认态逐像素相同。
- full 圆角时成胶囊形；390 下正常换行。
- classic 行本轮只改了明暗，轮廓没变。没有可扣分的地方，已是最高档。

**视觉 2.95**
- 上一轮的 UI-81 已解决：上缘高光、两侧亮边与目标的差都在 ±2 个亮度单位以内。
- 不到 3 的原因是本轮修 UI-81 时出现回归：深色下 classic 底边暗线变浅。暗线压暗的幅度从 R3 的约 −31%（目标约 −28%）变成约 −20%，底边的明暗台阶只有目标的约 2/3（新-1）。

**交互与状态 3.0**
- 焦点特写全部与 R3 字节相同，classic 特写除外，它只有按钮填充变了。我按 B2 复量了焦点环：
  - 浅色：indigo 第 8 阶 #8DA4EF，宽 4 设备 px，即 2 CSS px。
  - 深色：#435DB1。
  - red 按钮：red 第 8 阶 #EB8E90。
  - High contrast：按钮底是 blue 第 12 阶 #113264，环是 blue 第 8 阶 #5EB1EF，符合「设了 color 的用自身色族第 8 阶」。
  - classic 和 solid 的环与按钮之间有 4 设备 px（2px offset）的空隙；soft、surface、outline、ghost、ghost-offset 的环直接贴着控件。
- loading 保留宽度并显示 spinner。
- Unavailable 可以聚焦，并显示焦点环。
- Link 聚焦时隐藏下划线。
- 110%、full、中文都能用。
- 没发现不符，已是最高档。

**细节 1.0**
- `U6-panel-zh-1280-light` 与 R3 字节相同。文案逐处比对没有差异，例如：
  - 「已解析中性色：蓝灰色。」
  - 「无覆盖　浅色 0 · 深色 0」
  - 「使用配置值 · 两模式」
  - 「全圆角」

---

## 2. 上一轮差距复核表

| 编号 | R3 结论 | R4 状态 | 依据 |
| --- | --- | --- | --- |
| UI-81 · classic 内高光（上缘高光带、两侧亮边）只有目标的 1/3–2/3 | P3，部分解决，可挽回 0.1 | **已解决** | 见下 |

UI-81 的依据：

- **上缘**：取目标 `radix-sink-button.png` classic 4 的 x = 415（y 186–233），对照 R4 `U6-fixture-1280-dark` classic 4 的 x = 490（y 137–184）。主体之上 4 行的亮度增量：

  | | 第 1 行 | 第 2 行 | 第 3 行 | 第 4 行 |
  | --- | --- | --- | --- | --- |
  | 目标 | +83（#B7ABE6） | +20 | +11 | +4 |
  | R4 | +80（#9EB1EE） | +20 | +11 | +4 |
  | R3 | +53 | — | — | — |

- **两侧**：在中线 y = 209 和 y = 160 上，外缘一列和次一列的增量：
  - 目标：+16.5 / +3.5（#7C69CD / #715AD0）
  - R4：+18 / +4（#5675DB / #4367DD）
  - R3：+5 / +1
- `radix-classic-button.png` 的 indigo classic 是第二个参考，上缘为 #ADADEA，相对主体 #5757CE 的增量与 R4 同级。
- size 1–3 的各列结果相同。

R3 的建议没有被采纳：夹具除 classic 填充外字节未变，各条原样移入第 4 节（不扣分）。

---

## 3. 差距清单

#### 新-1 · [P3] 深色下 classic 底边暗线变浅（UI-81 修复引入的回归）

**单元与截图**
- `U6-fixture-1280-dark.png` 的 classic 1–4 底边，以及 `fixture-390`、`scale110`、`radius-full` 中同一规则的深色渲染。
- 量点：
  - classic 4：x = 490 和 x = 540，y = 184。
  - classic 1：x = 152，y = 172。

**维度**：视觉。

**现状**
- 目标 classic 4 的末行是 #584B90，其上一行是主体终点 #7C67D4，亮度 115 → 83，压暗约 28%。逐通道差是 (−36, −28, −68)。
- 目标 size 1–3 相同；indigo 参考是 #4E4E95 压在 #6B6BDA 上，也是 −28%。
- R4 末行是 #425AAA，压在 #4F70E0 上，亮度 113 → 91，只压暗约 20%，逐通道差 (−13, −22, −54)，R 通道差得最多。
- R3 当时是 #334DA3（−31%），与目标相符；R3 评审还明确写了「底部暗线保持」。
- 从 R3 到 R4，深色 classic 只有这几处变化：上 4 行、左右 1–2 列和末行，主体各行完全相同。所以是本轮新加的亮边层把末行一起提亮了。
- 浅色末两行 #3656C5 / #3454C4 与 R3 相同，没有受影响。
- 同层还有一处次要偏差，R3 已记录但没扣分：主体从上到下的提亮幅度是目标的约 80%。目标 +20（95→115，indigo 参考 +19），R4 +16（97→113）。
- 两者叠加，classic 下缘的「亮→暗」台阶在 R4 是 −22，目标是 −32，下缘看起来比目标更平。

**期望**：目标图 `radix-sink-button.png` classic 行，以及 `radix-classic-button.png` 的 indigo / blue 行。末行相对上一行压暗约 28%，主体渐变终点比起点亮约 +20。规范见 B1 的变体外观；比较口径中的「填充/描边/文字处理」。

**修法**：只改 Button classic 变体在深色下的阴影层。
1. 让新增的侧边亮边不作用到底边。下面是从像素推断的，没读代码：如果亮边用的是四周 1px 的 inset（例如 `inset 0 0 0 1px &lt;白色 alpha&gt;`），把它改成只作用于左右两侧的 inset（`inset 1px 0 …, inset -1px 0 …`），上缘高光保留现在的 `inset 0 1px …` 加带模糊的亮带。或者把底部暗线那层阴影排到 `box-shadow` 列表最前面，因为第一层画在最上面，这样它就不会被亮边覆盖。
2. 把底部暗线恢复到 R3 的强度，目标是末行 ≈ 上一行亮度 × 0.72，例如上一行为 #4F70E0 时，末行约为 #344FA4。
3. 可选：把主体渐变终点的白色叠加提高约 1.25 倍，使从上到下的提亮约 +20。
4. 浅色的末两行和外缘暗线不要动。
5. 改动如果涉及 `crates/rustify-components` 里的 class 字符串，按仓库约定运行 `mbx xtask css` 并提交 `rustify.css`。

**验收**：
- 重拍 `U6-fixture-1280-dark`，在 x = 490 量 classic 4 的 y = 183/184，比值约 0.72。
- 上 4 行和左右两列的增量应保持在 +80 / +20 / +11 / +4 和 +18 / +4。

**可挽回分**：0.05。

&gt; 可挽回分合计 0.05 = 10 − 9.95。

---

## 4. 建议（不扣分）

R3 的建议，本轮夹具没有变，原样保留：
- 夹具补齐几组矩阵：
  - 按钮矩阵的 high-contrast、gray、gray + high-contrast、disabled 各列。
  - IconButton 的 size × variant 矩阵，对应 `radix-sink-icon-button.png`。
  - 各色 classic 与 classic high-contrast，对应 `radix-classic-button.png`。
  - Link 的 1–9 字级和 underline 四种模式，对应 `radix-sink-link.png` 前两行。
- 补一个「文字 + 图标」的 Button。
- 「Caller utilities」现在是红底深红字，几乎读不出，换成可读的配色。
- 补拍 Unavailable 激活后的反馈、ThemePanel 的覆盖态，以及悬停在 underline = auto / hover 的链接上的帧，并标出它的 underline 属性。

本轮新增：
- 「High contrast」按钮实际是 blue 色族（底 #113264 / #C2E6FF，环 #5EB1EF）。建议标签写成「blue · High contrast」，或另加一个不设 color 的 accent high-contrast，对应目标的「+ high-contrast」列，免得读图的人误以为 accent 第 12 阶选错了。
- 补拍深色下的 loading 帧，以及 classic 在深色下 DPR 2 的焦点特写。现在只有浅色的 classic 特写，而 classic 的明暗细节以深色目标为准。

---

## 5. 看不出的格子

| 图 | 看不出什么 |
| --- | --- |
| `U6-fixture-1280-light/dark`、`U6-fixture-390-light` | IconButton 只有 solid size 2 的「×」一个。`radix-sink-icon-button.png` 里 classic、soft、surface、outline、ghost-offset、ghost 各变体和 size 1、3、4 都没拍到，无法比对。 |
| 同上 | high-contrast 只有 solid 一种，而且是 blue 色族；disabled 只有 solid。classic、soft、surface、outline、ghost 的 high-contrast、gray 和 disabled 外观都没出现。 |
| `U6-links-hover-1280-light` 对 `U6-links-1280-light` | 两张图逐像素相同（差分为空），被悬停的「Open a native link」静止时就有下划线。看不出它是 always 还是别的模式，也看不出 auto / hover 模式下悬停是否出现下划线。Link 的字级 1–9、color / high_contrast、truncate 以外的属性也没拍到。 |
| `U6-fixture-1280-light` 的 classic 行 | 浅色 classic 外侧有 1px 暗边，内侧一行亮色高光（#385ACD / #C3CFF5）。没有浅色目标图，无法判断是否符合「同一变体规则」。 |
| 各按钮文字 | 字族随主题、与目标不同；solid 和 ghost 之间的笔画粗细比与目标一致。但截图读不出声明的字重 token，绝对字重无法判断。 |
| `U6-loading-1280-light` | 只有浅色。spinner 在 reducedMotion 下是静帧，看不出转动；也看不出点击是否真被阻止，日志里没有点击 loading 按钮的记录。 |

---

## 6. 总评

- U6 是本次唯一评审的单元，得 **9.95**，比 R3 的 9.9 和 R2 的 9.7 都高。
- 唯一扣分是新-1：本轮修 UI-81 时，深色 classic 底边暗线被一起提亮，底边明暗台阶只剩目标的约 2/3。只要调整 classic 变体阴影层的作用范围和叠放顺序，就能恢复 R3 的底边，同时保住这次已经与目标对齐的上缘高光和侧边亮边。
- 不需要任何结构性改动：布局、选阶、焦点规则、loading / disabled / unavailable 状态和 ThemePanel 中文都已与目标一致。
- 剩下没覆盖的主要是夹具缺少的矩阵（IconButton 各变体和尺寸、各变体的 high-contrast / gray / disabled、Link 的 underline 模式与字级）。这些格子本轮既不算通过也不算失败；补进夹具后才能在下一轮核实。
