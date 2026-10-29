# U6 UI 验收评审 · R1（首轮）

评审范围只有 U6：Button、IconButton、Link 和内嵌的 ThemePanel。依据是 14 张 `U6-*.png`、`U6-actions.log`，以及目标包 U6 行列出的 4 张 `radix-*.png`。像素量值是我在 `/private/tmp/claude-501/-Users-rockie-Documents-gh-rockie-rustify-ui/0bc04ccb-5f93-4892-a089-e1cb25f9419c/scratchpad/judge-D/` 下用 `magick` 裁剪、取色、量边得到的。

关于色相：radix-classic-button.png 里有一整行 indigo（y≈752–783），它的 solid 填充 #3E63DD 与我方 solid 完全相同。所以 indigo 的同色相对照值都从这一行取，其余变体规则取自 violet 的 radix-sink-button.png。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | --- | --- | --- | --- | --- |
| U6 | 3.0 | 1.5 | 2.7 | 0.6 | **7.8** |

**布局 3.0（已是最高档）**：实测各项与目标一致，没有找到布局层面的差距：
- 四档高度为 24/32/40/48px，目标 solid 列同为 24/32/40/48。
- 左右内边距约 8/12/16/24px，目标以 ghost 与 ghost-offset 的文字位移反推，同为 8/12/16/24。
- 四档圆角约 3/4/6/8px，与目标一致。
- 110% 缩放下高度约 26/35/44/52px。
- ghost-offset 的文字与其他变体对齐，ghost 的文字贴列起点，与目标规则一致。
- loading 前后宽度不变：两个按钮都是 110 和 94px。
- IconButton 为 32×32 的正方形，full 圆角下变成正圆。
- 390 宽度下换行正常，没有溢出。

**视觉 1.5（不到 2）**：solid、high-contrast solid 和尺寸体系是对的，但 7 个变体里有 4 处颜色规则错了，不是个别色值偏差：
- 深色下 soft、surface、outline、ghost 的文字用了第 9 阶，对比度只有 3.3–3.8:1。
- surface 没有着色底，和 outline 难以区分。
- classic 的渐变方向与目标相反。
- disabled 带品牌色，而目标是中性灰。

深色矩阵和目标图放在一起看，一眼就能看出不同。

**交互与状态 2.7（不到 3）**：焦点环（3px 半透明、2px 间隙）、unavailable 可聚焦、loading 保宽、缩放、full 圆角、中文切换都能走通。不到 3 分只因为一处：loading 的 spinner 几乎看不见，而且按钮底色和 disabled 完全一样，加载态读起来像禁用态。

**细节 0.6（不到 1）**：还有 3 处小差异：
- Link 文字用第 9 阶，不是第 11 阶。
- ThemePanel 中文说明的一句语义走样。
- Link 焦点环为 2px 实线，与按钮的 3px 环不一致。

## 2. 上一轮差距复核

首轮，略过。

## 3. 差距清单

排序先按严重度，再按可挽回分。可挽回分合计 2.2 = 10 − 7.8。

### U6-01 · 次级变体文字用了第 9 阶，深色下不达 WCAG AA（视觉 · P1 · 可挽回 0.6）

**位置**：U6-fixture-1280-dark.png 的 soft、surface、outline、ghost-offset、ghost 五行，所有尺寸；U6-fixture-1280-light.png 的 soft 行。

**现状**：两种模式下文字都是 #3E63DD，也就是 solid 填充用的那一阶。深色下的对比度：

| 底色 | 色值 | 对比度 |
| --- | --- | --- |
| 页面 | #111113 | ≈3.6:1 |
| soft 底 | #161A2B | ≈3.3:1 |
| surface 底 | #0C0C0E | ≈3.8:1 |

size 1/2 的 12/14px 文字都低于 4.5:1。浅色 soft 为 #3E63DD 压在 #E8ECFB 上，≈4.4:1，也略低于 4.5:1。

**期望**：
- radix-sink-button.png 的 soft、surface、outline、ghost-offset、ghost 文字都是 #BAA7FF，明显比 solid 填充 #6E56CF 亮，即第 11 阶。
- 同色相对照见 radix-classic-button.png 的 indigo 行第 5 列：soft 文字 #9EB1FF，压在 #18244A 上。
- 依据 B1 的变体规则；浅色按同一规则也用第 11 阶。

**修法**：
- 在 `crates/rustify-components` 的 Button 和 IconButton `variants!` 里，把 soft、surface、outline、ghost、ghost-offset 的文字色从第 9 阶 token 改为第 11 阶（Radix `--accent-11` 的对应物），high-contrast 时用第 12 阶。
- 确认深色 scope 重新定义了第 11 阶。
- IconButton 的图标用 currentColor 跟随。
- 改完运行 `mbx xtask css`，提交 `rustify.css`，并重建 component-catalog release 后重拍。

### U6-02 · soft、surface、outline 的填充与描边层级不对（视觉 · P2 · 可挽回 0.4）

**位置**：U6-fixture-1280-light.png 与 U6-fixture-1280-dark.png 的 soft、surface、outline 三行。

**现状**：

| 模式 | soft 底 | surface 底 | surface 描边 | outline 描边 |
| --- | --- | --- | --- | --- |
| 浅色 | — | #FFFFFF（与页面相同） | #A5B7EF | #9EB1EE |
| 深色 | #161A2B（比页面 #111113 仅亮一线） | #0C0C0E（比页面更暗、无色相） | #23356D | #273A78 |

浅色下 surface 与 outline 几乎分不出；深色下两种描边都偏暗且相近。

**期望**：

| 变体 | 目标值 | 出处 |
| --- | --- | --- |
| soft 底 | 明显着色，#2A1F45 压在 #121113 上；indigo 为 #18244A | radix-sink-button.png；radix-classic-button.png indigo 行 |
| surface 底 | 比页面亮的着色底 #1C1526，描边 alpha 第 7 阶 #5B4896 | radix-sink-button.png |
| outline | 透明底，更强的第 8 阶描边 #6958AD | radix-sink-button.png |

surface 与 outline 应在填充和描边强度两方面都可区分。

**修法**：
- soft：底色改为 accent alpha 第 3 阶。
- surface：底色改为 accent surface token（Radix `--accent-surface` 的对应物），描边改为 inset 1px 的 accent alpha 第 7 阶。
- outline：透明底，描边改为 inset 1px 的 accent alpha 第 8 阶。
- 核对深色 scope 是否给 alpha 阶和 surface 阶定义了深色值。现状像是用了浅色 alpha 或较低的实色阶。

### U6-03 · loading 的 spinner 几乎不可见，状态读作 disabled（交互与状态 · P2 · 可挽回 0.3）

**位置**：U6-loading-1280-light.png 中第 2、3 个按钮。

**现状**：
- 按钮底色 #9DB0ED，与 Disabled request 完全相同。
- spinner 约 16px，轨道 ≈#B5C4F1，与底色对比约 1.2:1；弧段只有少数几个像素接近白色。
- 文字被隐藏。宽度保持得对（110 和 94px）。

**期望**：B3 要求 loading 保留名称与宽度、加一个装饰性 spinner，并且「disabled 永远优先」，说明两种状态应能区分。所以加载中必须一眼可辨，不能和禁用同貌。

**修法**：
- spinner 弧段用按钮当前文字色（currentColor），不透明，与填充对比 ≥3:1；轨道用同色约 25–35% 透明度。
- spinner 尺寸随档位走：size1 12px、size2 14–16px、size3/4 16–18px。
- loading 的底色不要套 disabled 样式。如果按 U6-05 改成中性灰，spinner 用对应的文字阶，并保证对比。
- 保留文字的 `visibility:hidden` 占位。

### U6-04 · classic 的渐变方向与目标相反，且多了外投影（视觉 · P2 · 可挽回 0.25）

**位置**：U6-fixture-1280-dark.png 与 U6-fixture-1280-light.png 的 classic 行；两种模式像素相同。

**现状**：顶边 1px #879EEA，往下从 #5475E1 线性变到 #3E63DD，即上亮下暗；按钮下方另有 1–2px 外投影（浅色 #DBE2F8/#F2F4FC，深色 #181F37/#13151F）。

**期望**：radix-classic-button.png 的 indigo 行第 1 列（同色相）：

| 位置 | 目标色值 |
| --- | --- |
| 顶部 1px 内高光 | #9EB1EE |
| 约 5px 内降到 | #3C5FD5（比 solid 略暗） |
| 中部 | #3E63DD |
| 底部渐亮到 | #5172E0 |
| 最后一行内暗边 | #3D5298 |
| 按钮下方 | 直接是页面色，无外投影 |

radix-sink-button.png 的 classic 行同理（#6952C6 → #7C66D4）。

**修法**：
- 在第 9 阶底色上叠一层 `linear-gradient(to bottom, …)`，做成上暗下亮。
- 顶部用 `inset 0 1px` 浅色高光，底部用 `inset 0 -1px` 暗边。
- 去掉外部 drop shadow。
- 浅色按同一结构实现。

### U6-05 · disabled 带品牌色，不是中性灰（视觉 · P2 · 可挽回 0.25）

**位置**：U6-fixture-1280-light.png 与 U6-fixture-1280-dark.png 的「Disabled request」，「Unavailable request」外观相同。

**现状**：浅色为淡化的品牌色底 #9DB0ED 配白字；深色为品牌深蓝底 #273A78 配 #888889 文字。

**期望**：radix-sink-button.png 的 disabled 列是中性灰：
- solid、classic、soft：底 #232225，即灰 alpha 第 3 阶；文字 #6D6974，即灰 alpha 第 8 阶。
- surface：底 #1A191B，配灰描边。
- outline：只有灰描边。
- ghost：只有灰字。
- 都不带品牌色相，也不用整体透明度。

**修法**：在 Button 和 IconButton 的 `:disabled`、`[data-disabled]` 规则里覆盖所有变体：
- 填充用 gray alpha 第 3 阶，surface 用第 2 阶加灰描边，outline 只留灰描边，ghost 无底。
- 文字和图标用 gray alpha 第 8 阶。
- 去掉 opacity 淡化方案。

unavailable 是否沿用这套外观，见建议 4。

### U6-06 · Link 文字用了第 9 阶（细节 · P3 · 可挽回 0.15）

**位置**：U6-links-1280-light.png 中的「Open a native link」和「https://example.test/…」。

**现状**：文字 #3E63DD，与 solid 填充同阶。浅色下对白底约 5.2:1，可读性没问题。

**期望**：radix-sink-link.png 的默认 Link 文字是 #BAA7FF，与 soft 按钮文字同为第 11 阶；high-contrast 时为第 12 阶。依据 B5。

**修法**：Link 的颜色 token 改为第 11 阶，与 U6-01 同步修改。深色 Link 这轮没有拍到；不改的话，深色下会复现 U6-01 的对比度问题。

### U6-07 · ThemePanel 中文说明语义走样（细节 · P3 · 可挽回 0.15）

**位置**：U6-panel-zh-1280-light.png，标题下第二行。

**现状**：「使用基础或配置值会清除该组两模式覆盖；其他组和独立精调保持不变。」其中「基础或」意义不明，又漏掉了「只」。

**期望**：与英文原句等义。英文为「A group reset clears only that group's overrides in both modes. Other groups and independent adjustments stay unchanged.」（见 U6-fixture-1280-light.png）。

**修法**：改为「分组重置只清除该组在两种模式下的覆盖；其他组和独立精调保持不变。」

### U6-08 · Link 焦点环与按钮焦点环不一致（细节 · P3 · 可挽回 0.1）

**位置**：U6-links-focus-1280-light.png 中的「Open a native link」。

**现状**：2px 实线 #8DA4EF，几乎贴着行盒，没有 2px 间隙。按钮焦点环见 U6-focus-1280-light.png，实测为 3px #C6D1F7 环加 2px 白色间隙。

**期望**：B2 写的是「scope ring 色半透明 3px 环、2px offset」，它位于 Button/IconButton/Link 这一节下。不过出处是 DESIGN.md 的 Buttons 段，是否适用于 Link 请负责人确认。

**修法**：Link 的 `:focus-visible` 复用按钮的 ring token（3px、半透明、2px offset，圆角随字号）。

## 4. 建议（不扣分）

1. **焦点环对比度**：按钮焦点环 #C6D1F7 对白底约 1.5:1。它符合 B2 的「半透明」，但有 WCAG 1.4.11 非文本对比度的风险。建议负责人考虑提高 alpha，或者加一道 1px 深色内环。这是规范层面的问题，不算实现差距。
2. **扩充夹具，让下一轮能逐格比对**：
   - 按目标矩阵补齐「变体 × size1–4 + high-contrast + gray + gray + high-contrast + disabled」。
   - 补 IconButton 矩阵和「文字 + 图标」样本，用来核对 gap。
   - Link 补 size 1–9、颜色、high-contrast，并给 underline 模式加标注。
   - 拍下点「Override primary」「Seed both-mode overrides」后的 ThemePanel。
3. **改进采集**：重拍时先把指针移开（见第 5 节的悬停污染），并补拍深色下的焦点帧和 loading。
4. **unavailable 与 disabled 外观完全相同**：unavailable 可聚焦，是「说明性请求」，建议加一个可见提示（说明文字或图标），免得用户以为它只是禁用。
5. **ThemePanel 的形态**：目标图右缘露出的 Radix ThemePanel 用色板网格、圆角预览方块和分段式缩放；我方是整宽的原生下拉框加说明文字。目标包的比较口径没有把 ThemePanel 外观列为还原项，所以不扣分。若要求与 Radix 外观一致，需要重做为色板、方块、分段控件，属于结构性改动。
6. **中文措辞**：ThemePanel 用的是「品牌色族」「中性色族」，C2 的措辞是「品牌色」「中性色」。建议与 Studio 基础区中文统一（C3 要求「表达相同选项」）。U1 不在本次范围，我没有核对。
7. **夹具样本可读性**：「Caller utilities」是深红字压红底，几乎读不出。它是调用方覆盖的演示，建议换一组可读的颜色，免得被误认为缺陷。

## 5. 看不出的格子

- **变体 × 状态矩阵（U6-fixture-1280-light/dark.png）**：soft、surface、outline、ghost 的 high-contrast 列，各变体的 gray、gray + high-contrast，以及 solid 以外变体的 disabled，夹具里都没有，无法比对。
- **「High contrast」按钮的色阶**：浅色为 #113264，深色为 #C2E6FF，与目标 indigo high-contrast 的 #D6E1FF 色阶不同。看不出夹具是否给它设了 color，所以不判。
- **IconButton**：只有一个 solid size 2 的「×」，实测 32×32、正确。其余变体、尺寸 1/3/4、深色都没拍到。
- **文字 + 图标按钮**：没有样本，图标与文字的 gap 无法核对。
- **Link 悬停**：U6-links-hover 与 U6-links 逐像素相同（AE = 0），看不出悬停是否生效，也看不出「Open a native link」用的是哪种 underline 模式。目标里默认 auto 的链接静止时没有下划线，而我方这个链接静止时有，是否属于模式差异无法判定。
- **Link 其他维度**：size 1–9、weight、trim、颜色、high-contrast、深色、正文内嵌链接都没拍到。
- **悬停污染**：U6-focus-1280-light.png 和 U6-focus-02 里的 Default action，以及 U6-scale110 的 solid 2，都是 #5172E0（悬停色），像是残留指针造成的。系统性的悬停和按下态没有拍到。
- **深色下的状态**：焦点环和 loading 只有浅色。
- **loading 的可访问名称与可聚焦性**：截图无法体现。
- **unavailable 的说明反馈**：U6-focus-unavailable 只看到焦点环，聚焦或激活后是否给出说明没有拍到。
- **B6**：没有点「Override primary」，无法验证未设 color 的 solid 和 Link 是否消费有效的 primary。
- **C3**：ThemePanel 只拍到无覆盖状态，显示「No overrides · Light 0 · Dark 0」或「无覆盖 · 浅色 0 · 深色 0」。被覆盖和 Custom 时的文字与状态标识、「N 项被覆盖」说明都没拍到。
- **C2**：下拉列表没有展开，无法核对选项数量（品牌色 26 项等）。

## 6. 总评

本次只评 U6，单元分 7.8。

**主要失分原因**：次级变体的颜色 token 映射有系统性偏差。深色下 soft、surface、outline、ghost 的文字用第 9 阶，导致对比度不达标；surface 和 soft 的底色、描边层级不对；classic 渐变方向反了；disabled 不是中性灰；loading 又借用了 disabled 的外观。

**做得好的部分**：结构和几何都对，包括尺寸、内边距、圆角、缩放、ghost-offset 几何和保宽。

**改动性质**：按钮部分不需要推倒重做，修改集中在 Button、IconButton、Link 的 variant class 和 token 映射。真正结构性的事只有两件：
- 扩充夹具矩阵，否则 high-contrast、disabled 各列和 IconButton、Link 的大部分规格仍无法验收。
- 如果负责人要求 ThemePanel 与 Radix 外观一致，需要重做为色板、方块、分段控件。

---

**开图清单**：radix-sink-button.png、radix-sink-icon-button.png、radix-classic-button.png、radix-sink-link.png、U6-fixture-1280-light.png、U6-fixture-1280-dark.png、U6-loading-1280-light.png、U6-focus-1280-light.png、U6-focus-02-1280-light.png、U6-focus-unavailable-1280-light.png、U6-scale110-1280-light.png、U6-radius-full-1280-light.png、U6-panel-zh-1280-light.png、U6-fixture-390-light.png、U6-links-1280-light.png、U6-links-hover-1280-light.png、U6-links-focus-1280-light.png、U6-links-390-light.png
