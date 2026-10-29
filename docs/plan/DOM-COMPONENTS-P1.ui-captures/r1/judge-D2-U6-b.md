# U6 评审：Button / IconButton / Link 与内嵌 ThemePanel（R1 首轮）

浅色下，按钮的尺寸、内边距、圆角和缩放基本对上了目标图。主要差距有两处：一是深色模式下的配色体系，二是"不可用"按钮的焦点环几乎看不见。

## 1. 评分表

| 单元 | 布局 (0–3) | 视觉 (0–3) | 交互与状态 (0–3) | 细节 (0–1) | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 | 3.0 | 1.5 | 2.5 | 0.75 | **7.75** |

各维度理由（为什么不是更高一档；布局为什么给满分）：

- **布局 3.0**
  - 四档高度实测与目标图一致：
    - 100%：24/32/40/48px；
    - 110%：26/35/44/53px。
  - 左侧内边距与目标图逐档一致：8/12/16/24px。
  - ghost 文字贴列边，ghost-offset 文字保留内边距位置，和目标图两行的几何一致。
  - 圆角：full 下变为胶囊；单实例 none/small/medium/large 分别为 0/≈3/≈4/≈6px。
  - 390 宽下换行，没有溢出或截断。
  - 拍到的部分没有布局偏差，所以给满分。
- **视觉 1.5**（不到 2 分）
  - 目标图是深色。在深色夹具里，soft/surface/outline/ghost/ghost-offset 五行的文字、填充、描边和 classic 的渐变都偏离了目标规则，而且其中的文字对比度不达 WCAG AA。
  - 浅色下也有问题：surface 与 outline 无法区分；disabled 用整体半透明，而不是中性灰。
  - 这是多处、成体系的 token 映射错误，不只是零星偏差。
- **交互与状态 2.5**（不到 3 分）
  - 已走通：加载（只显示 spinner，宽度逐像素保持）、主按钮与 gold 按钮的焦点环（3px 半透明环、2px 间隙，与 B2 一致）、缩放 110%、主题圆角 full、ThemePanel 切中文、Link 焦点。
  - 失分点：可聚焦的 "Unavailable request" 获得焦点后，焦点环对比度只有约 1.2:1，几乎看不见。
- **细节 0.75**（不到 1 分）
  - ThemePanel 中文版有一句说明的措辞和英文原意不对应（见 新-6）。
  - 其余逐处比对没有发现文案、数字或省略号方面的差异。

## 2. 上一轮差距复核表

首轮，省略。

## 3. 差距清单

按严重度排序，同级再按可挽回分排序。可挽回分合计 2.25 = 10 − 7.75。

### 新-1：非 solid 变体与 Link 的文字用了"填充级"而不是"文字级"，深色下对比度不达 AA

- **单元 / 截图**：U6
  - `U6-fixture-1280-dark.png`：soft、surface、outline、ghost-offset、ghost 五行全部文字；
  - `U6-fixture-1280-light.png`：同样五行；
  - `U6-links-1280-light.png`："Open a native link"、"https://example.test/…"。
- **维度**：视觉。**严重度 P1**。
- **现状**：五种变体和 Link 的文字取样都是 `#3E63DD`，和 solid 按钮填充色完全相同，两种模式下都一样。
  - 深色 soft 4：`#3E63DD` 在 `#161A2B` 上 ≈ 3.3:1；
  - 深色 ghost / outline：`#3E63DD` 在页面 `#111113` 上 ≈ 3.6:1；
  - 两者都低于 4.5:1（12–18px 正文字级）；
  - 浅色 soft 1（12px）：`#3E63DD` 在 `#E8ECFB` 上 ≈ 4.42:1，也略低于 4.5:1。
- **期望**（依据 `radix-sink-button.png`）：
  - soft / surface / outline / ghost-offset / ghost 行的文字取样为 `#BAA7FF`，明显比 solid 填充 `#6E56CF` 亮，是独立的"文字级"；
  - soft 4 上的对比度约 7.3:1；
  - `radix-sink-link.png` 中链接色同样是浅色文字级，不是填充色。
  - 浅色按"同一变体规则"对应，也应改用文字级（浅色下比填充更深）。
- **修法**：
  - 在 `crates/rustify-components` 的 Button / IconButton / Link `variants!` 类串里，把 soft / surface / outline / ghost / ghost-offset 和 Link 默认态的文字色，从 accent 第 9 级改为 accent 第 11 级文字 token（Radix 语义的 `accent-a11`，或本仓对应名）；
  - 在 `crates/rustify-components/css/sdk.css` 里确认深色 scope 为第 11 级定义了深色数值；
  - 改完运行 `mbx xtask css` 并提交 `rustify.css`；
  - 验收：深色 soft 4 文字对背景 ≥ 4.5:1，浅色 soft 1 ≥ 4.5:1。
- **可挽回分**：0.6

### 新-2：可聚焦的 "Unavailable request" 焦点环几乎不可见

- **单元 / 截图**：U6，`U6-focus-unavailable-1280-light.png`，第 450 行 "Unavailable request"。
- **维度**：交互与状态。**严重度 P1**。
- **现状**：
  - 焦点环取样为 `#E3E8FB`，对白底约 1.2:1；
  - 对照：同一套环在 "Default action" 上是 `#C6D1F7`（`U6-focus-1280-light-a2.png`）。
  - 证据显示整个元素被设成了 `opacity: .5`，环也一起变淡。四组色值都正好是 50% 混合：
    - 浅色按钮 `#9DB0ED` = `#3E63DD` 与白底各半；
    - 深色按钮 `#273A78` = `#3E63DD` 与 `#111113` 各半；
    - 深色文字 `#888889` = 白色与 `#111113` 各半；
    - 浅色焦点环 `#E3E8FB` = `#C6D1F7` 与白底各半。
- **期望**（依据 B2、B3）：
  - unavailable 是"保持可聚焦的说明性请求"，焦点应和其他按钮一样，是可见的 scope ring 色 3px 环、2px 间隙；
  - 键盘用户必须能看见焦点（WCAG 2.4.7）。
- **修法**：
  - 去掉 unavailable 与 disabled 状态上的元素级 `opacity`；
  - 改用显式颜色 token 表达不可用外观（见 新-5），让 ring 的 `box-shadow` 或 `outline` 保持原强度；
  - 验收：Tab 到 "Unavailable request"，环色与 "Default action" 的焦点帧逐像素相同。
- **可挽回分**：0.5

### 新-3：深色模式的 soft 填充、surface 填充和 surface/outline 描边过暗，像是把浅色 alpha 色阶叠在深底上

- **单元 / 截图**：U6，`U6-fixture-1280-dark.png` 的 soft、surface、outline 三行。
- **维度**：视觉。**严重度 P2**。
- **现状**：

  | 位置 | 夹具取样 | 问题 |
  | --- | --- | --- |
  | soft 填充 | `#161A2B` | 对页面 `#111113` 只有 1.09:1，填充几乎看不出 |
  | surface 填充 | `#0C0C0E` | 比页面更暗，方向反了 |
  | surface 描边 | `#253876` | 发灰、发暗 |
  | outline 描边 | `#273A78` | 发灰、发暗 |

- **期望**（依据 `radix-sink-button.png`）：

  | 位置 | 目标取样 | 说明 |
  | --- | --- | --- |
  | soft 填充 | `#2A1F45` | 对页面 1.24:1，明显带色 |
  | surface 填充 | `#1C1526` | 比页面亮、带 accent 色调 |
  | surface 描边 | `#5B4896` | — |
  | outline 描边 | `#6958AD` | 比 surface 高一级 |

- **修法**：
  - 在 `css/sdk.css` 的深色 variant / scope 中，为 accent 的 a1–a12 与 surface 色定义深色专用数值，不要复用浅色 alpha；
  - 变体映射为：soft 填充 → `accent-a3`，surface 填充 → `accent-surface`，surface 描边 → `accent-a7`，outline 描边 → `accent-a8`；
  - 用深色夹具复拍，逐行和目标图对色。
- **可挽回分**：0.35

### 新-4：深色 classic 沿用了浅色的立体处理，渐变方向与目标相反

- **单元 / 截图**：U6，`U6-fixture-1280-dark.png` 的 classic 1–4（放大比对的是 classic 4）。
- **维度**：视觉。**严重度 P2**。
- **现状**（深色与浅色逐行相同）：
  - 顶部 1px 高光 `#889EEA`；
  - 主体上亮下暗（R 通道 84→62）；
  - 按钮下方有 2px 外投影（`#181F37`、`#13151F`）。
- **期望**（依据 `radix-sink-button.png` 的 classic 4 纵向剖面）：
  - 顶部 1px 强高光 `#B7ABE6`；
  - 主体上暗下亮（R 通道 105→124）；
  - 底边 1px 深线 `#584B90`；
  - 没有外投影。
- **修法**：在 classic 变体的深色分支里重写叠加层，按以上四点实现（反转渐变方向、加强顶部内高光、加底部内暗线、去掉外投影）。
- **可挽回分**：0.2

### 新-5：surface 与 outline 在浅色下完全相同

- **单元 / 截图**：U6，`U6-fixture-1280-light.png` 的 surface 行与 outline 行，以及 `U6-scale110-1280-light-a2.png`。
- **维度**：视觉。**严重度 P2**。
- **现状**：
  - 浅色下 surface 4 与 outline 4 都是白底 `#FFFFFF`、描边 `#9EB1EE`，取样完全一致；
  - 深色下两者也只差 1–2 个色值。
- **期望**（依据 `radix-sink-button.png`）：
  - surface 有带色填充（`#1C1526`，区别于页面），描边低一级（`#5B4896`）；
  - outline 是透明底，描边高一级（`#6958AD`）；
  - 两行一眼就能区分。
- **修法**：
  - surface 改为 `accent-surface` 填充加 `accent-a7` 内描边；
  - outline 保持透明底加 `accent-a8` 内描边；
  - 浅色与深色都按这条规则取各自的色阶。
- **可挽回分**：0.2

### 新-6：disabled 用强调色整体半透明，而不是中性灰

- **单元 / 截图**：U6，"Disabled request"
  - `U6-fixture-1280-light.png`：`#9DB0ED` 底、白字；
  - `U6-fixture-1280-dark.png`：`#273A78` 底、`#888889` 字。
- **维度**：视觉。**严重度 P2**。
- **现状**：disabled 仍带明显的 indigo 色相；浅色下看起来像"浅色 solid"或悬停态。
- **期望**（依据 `radix-sink-button.png` 的 disabled 列，各变体都去掉了强调色）：
  - classic / solid / soft：中性填充 `#232225` + 灰字 `#6D6974`；
  - surface / outline：灰描边 `#49474E` + 灰字；
  - ghost：只有灰字。
- **修法**：
  - disabled 状态类改为 `gray-a3` 填充、`gray-a8` 文字；
  - surface / outline 用 `gray-a6` 或 `gray-a7` 描边；
  - classic 去掉渐变和阴影；
  - 不使用 `opacity`（与 新-2 一起改）。
  - loading 如果复用 disabled 外观，也一起改。
- **可挽回分**：0.15

### 新-7：ThemePanel 中文说明句措辞与英文原意不对应

- **单元 / 截图**：U6，`U6-panel-zh-1280-light.png`，标题下第二行"使用基础或配置值会清除该组两模式覆盖；其他组和独立精调保持不变。"
- **维度**：细节。**严重度 P3**。
- **现状**：
  - "使用基础或配置值"语义含混；
  - 漏掉了英文 "A group reset clears **only** that group's overrides in both modes" 中的"仅"；
  - 和下面按钮"使用配置值 · 两模式"对不上。
- **期望**（依据 G6、C3）：中英文语义同步，说明能对应到具体的重置按钮。
- **修法**：改为"「使用配置值」仅清除该组在两种模式下的覆盖；其他组与独立精调保持不变。"
- **可挽回分**：0.25

**合计**：0.6 + 0.5 + 0.35 + 0.2 + 0.2 + 0.15 + 0.25 = **2.25**。

## 4. 建议（不扣分）

1. **补全夹具，下一轮才能对照目标（优先级最高）**
   - IconButton：补 7 个变体 × 4 个尺寸，加 high-contrast / gray / gray+high-contrast / disabled 四列；
   - Button：每个变体都补 high-contrast / gray / disabled 列，并补"文字 + 图标"的按钮（用来核对图标间距）；
   - Link：补字号 1–9 阶梯、underline auto 与 always 两行、high-contrast 和有色上下文。
   - 现在这些格子都看不出（见第 5 节）；补齐后如果暴露新问题，分数可能下降。
2. **默认焦点环对比度偏低**
   - 按钮焦点环 `#C6D1F7` 对白底约 1.5:1；Link 焦点轮廓 `#8DA4EF` 约 2.4:1。
   - B2 只规定"半透明 3px 环、2px 间隙"，没有规定 alpha；建议把 alpha 提到环对相邻背景 ≥ 3:1。
3. **"Unavailable request" 和 "Disabled request" 外观完全一样。** B3 说它是"说明性请求"，建议给它不同的外观或常驻说明（例如通过 `aria-describedby` 关联的提示文字），不要让人误以为是原生禁用。
4. **solid 悬停变亮**：首采帧 `U6-scale110-1280-light.png` 中 solid 2 悬停时为 `#5172E0`。目标图没有拍悬停态，这里只是提示：建议核实悬停色阶是否符合设计意图。
5. **"Caller utilities" 示例的标签不可读**：`#C8070A` 字在 `#D92D20` 底上约 1.2:1。建议换一组调用方 class 让标签可读，或者核实 `tw_merge` 是否丢掉了调用方的文字色 class。
6. **ThemePanel 布局**：内嵌时下拉框横向拉满约 1110px，只装 "indigo"、"100%" 这类短值。可考虑给一个最大宽度（例如 20–24rem），或参考 `radix-sink-button.png` 右缘的 Radix ThemePanel，用色块网格和分段控件。比较口径不含这一项。

## 5. 看不出的格子

| 截图 | 看不出什么 |
| --- | --- |
| `U6-fixture-1280-light.png` / `-dark` / `-390` | IconButton 只有一个样本：右侧带 × 图标、32×32 的 solid size 2，正方形、居中，这一个合格。`radix-sink-icon-button.png` 的其余变体 × 尺寸 × 列都无法比对 |
| 同上 | 除 solid 外，各变体的 high-contrast / gray / disabled 没有呈现。只有 solid 的 "High contrast"、"gray"、"Disabled request" 可核对；其中 high-contrast 两种模式都正确：浅色深底白字，深色浅底深字 |
| 同上 | 没有"文字 + 图标"的按钮，无法核对图标与文字的间距 |
| 同上 | "Responsive action" 在 1280 和 390 下都像 size 1，不知道它配置的断点值，判断不了响应式是否生效 |
| `U6-links-hover-1280-light.png` | 与 `U6-links-1280-light.png` 像素完全相同（AE = 0）。不知道悬停的是哪个链接、它的 underline 设置是什么，无法判断 auto 下划线在悬停时是否出现 |
| `U6-links-*` | 只有 3 个 Link 实例，`radix-sink-link.png` 里的字号 1–9、auto 与 always 两行、high-contrast、有色段落里的链接都无法比对 |
| `U6-loading-1280-light.png` | 可访问名称是否保留、点击是否被阻止、disabled 是否优先于 loading，截图里都看不出 |
| U6 全部 | 没有点 "Override primary"（B6），没有点 "Seed both-mode overrides"（C3 的覆盖态与 "N 项被覆盖" 说明）；ThemePanel 中文只拍到"无覆盖"状态 |
| U6 全部 | 深色模式下的键盘焦点帧没拍到 |
| `U6-panel-zh-1280-light.png` | 下拉都是收起状态，无法核对选项数量（C2：品牌色 26 项等），也无法和 Studio 基础区逐项对照选项（C3，按限定不打开 U1 的截图） |
| `U6-focus-unavailable-1280-light.png` | 激活 Unavailable 之后的说明反馈没有拍到 |

另：首采的 `U6-focus-1280-light.png`、`U6-focus-02-1280-light.png`、`U6-scale110-1280-light.png` 拍摄时指针仍停在按钮上（solid 2 显示悬停色 `#5172E0`）。按 capture.md，焦点与缩放的判断以 `-a2` 帧为准。

## 6. 总评

- U6 是本次唯一评分单元，得 **7.75**。失分主要来自两个结构性根因：
  1. **配色 token 映射**：非 solid 变体的文字直接用了 accent 第 9 级（填充级），深色 scope 又没有提供深色专用的 alpha 色阶和 surface 色。后果是深色下五行变体的文字对比度不达 AA、填充和描边发暗；surface 与 outline 无法区分；classic 在深色下没有切换立体处理。要在组件变体映射和 `sdk.css` 的深色 token 层面系统改，不能逐个按钮修补。
  2. **不可用外观用整体 `opacity` 实现**：disabled 和 unavailable 带着强调色调；可聚焦的 unavailable 焦点环被一起变淡到几乎不可见（P1）。要改成显式中性 token，不要再用 opacity。
- 布局几何（高度、内边距、圆角、缩放、ghost 偏移、窄屏换行）不需要改。ThemePanel 的结构和中文切换可用，只需修一句文案。
- 下一轮前务必补全夹具里的 IconButton 矩阵、各变体的 high-contrast / gray / disabled 列和 Link 字号阶梯，否则目标图里大半格子仍无法验收。

开图清单：radix-sink-button.png、radix-sink-icon-button.png、radix-classic-button.png、radix-sink-link.png、U6-fixture-1280-light.png、U6-fixture-1280-dark.png、U6-fixture-390-light.png、U6-loading-1280-light.png、U6-radius-full-1280-light.png、U6-focus-1280-light.png、U6-focus-1280-light-a2.png、U6-focus-02-1280-light.png、U6-focus-02-1280-light-a2.png、U6-focus-unavailable-1280-light.png、U6-scale110-1280-light.png、U6-scale110-1280-light-a2.png、U6-panel-zh-1280-light.png、U6-links-1280-light.png、U6-links-hover-1280-light.png、U6-links-focus-1280-light.png、U6-links-390-light.png（另读 U6-actions.log、capture.md、目标包；放大裁剪图都在 /private/tmp/claude-501/-Users-rockie-Documents-gh-rockie-rustify-ui/0bc04ccb-5f93-4892-a089-e1cb25f9419c/scratchpad/judge-D2/）
