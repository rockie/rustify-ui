# U2 评审原文 · R3（Studio 精调与来源解释）

只评 U2。依据是 r3 下 10 张 `U2-` 截图和 `U2-actions.log`，对照 r2 的 9 张 `U2-` 截图和 `r2-judge-U2.md`。颜色和尺寸是用 ImageMagick 在 DPR 1 原图上取像素、量文字包围盒得到的，以下坐标都是 CSS px。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.7 | 2.65 | 2.75 | 0.6 | **8.7**（R2 为 7.75） |

各维度理由（每条都说明为什么不是更高一档）：

- **布局 2.7**
  - 已做对的部分：
    - 来源块和拾色区插在所选格所在行的正下方。1440 下标题在 y=643，「Clear primary · Light only」在 y=841–872，都在首屏内。
    - 常驻壳层收成 57px 的顶栏。编辑列宽 320px，padding 24/20。
    - 390 下吸顶区不再漏出内容。
  - 不到 3 分的原因：
    - 每个被覆盖的格里，状态胶囊折成两行的椭圆。
    - 配置组的状态胶囊和解除按钮之间只有 0–1px。
    - 390 下所选格被吸顶区盖住。
    - 诊断区有 44px 空白。
- **视觉 2.65**
  - 已对上 G3 的 token：
    - 搜索框边框 #98A2B3，helper #475467；
    - 选中态 #EFF8FF / #84CAFF / #175CD3，分体控件两段一起高亮；
    - 错误边框 #B42318；
    - 标签页下划线 2px #1570EF。
  - 不到 3 分的原因：
    - 小节标题比字段 label 更浅、但字号字重相同，层级倒置。
    - 诊断区外层 summary 是常规字重，内层是 600。
    - 同一种状态用了胶囊和 4px 矩形两种形状。
    - 字体 select 的值是 600 字重。
    - 计数文字仍是 #667085。
- **交互与状态 2.75**
  - 已走通的主路径：
    - 选中格后显示来源（显式覆盖、配置生成两类都有）；
    - 单项解除写明 Light only，解除后按钮禁用；
    - 分组解除写明 both modes，其他组计数不变；
    - 输入 `abc` 后保留草稿，错误贴近字段，预览保持 0.35rem；
    - 低对比主题保留作者色，预览侧有可定位的诊断，Export 可用。
  - 不到 3 分的原因：
    - 所选格没有可解除的覆盖时，↺ 仍呈启用的蓝色。
    - 分组解除只给计数，不列出会影响哪些 token。
    - 「Restore document」没有写范围。
    - 网格里的 ↺ 仍只有图标。
- **细节 0.6**
  - R2 的 8 条细节差距（UI-117、118、120–124 和 UI-119）全部关闭。
  - 仍有若干小差异：
    - 来源块的键值结构不一致，「Light:」用了等宽字；
    - 诊断条末行与外框形成双线；
    - 搜索框 36px 与 Clear 按钮 32px 不等高；
    - 嵌套 summary 重复同一个计数；
    - 胶囊字号约 11px。

## 2. 上一轮差距复核表

| 编号 | R2 内容 | 本轮结论 | 依据（R3 截图） |
| --- | --- | --- | --- |
| UI-102 | 390 下标签条与吸顶头之间漏出内容 | **已解决** | U2-source-overridden-390：头区（底边 y=152）和 Edit/Preview 标签（y=160–192）是同一个不透明区块，到 y≈200 为止；滚动内容从 y≈201 起被干净地裁掉，中间没有漏出。 |
| UI-106 | 编辑区顶部 padding；配置组状态行与按钮相距 0px（U2 部分） | **仍存在**（U2 部分） | U2-group-reset：Brand 的「● No overrides」胶囊在 y=465–488，「Use configuration values · both modes」在 y=489–520，相距 0–1px；Neutral、Panel 两组同样。顶部 padding 已改为 24px（U2-restore 中首元素在 y=81 = 57+24），该部分属 U1，这里不计。 |
| UI-107 | 次级文字 #667085；搜索框边框 #D0D5DD | **部分解决** | 搜索框边框已是 #98A2B3；helper、caption、来源说明均为 #475467。U2-group-reset 中「No overrides」胶囊文字，以及「Light 0 · Dark 0」「Light 15 · Dark 15」「Light 2 · Dark 2」仍是 #667085。 |
| UI-109 | 状态胶囊借用选中蓝 | **已解决** | 网格胶囊为 #F9FAFB 底、#D0D5DD 边、#475467 字；配置组的「● Overridden」「● Custom」也改成了中性色。 |
| UI-17 | 选中 token 后来源与解除按钮不在屏内 | **已解决** | 来源块插在所选行之后。1440 下 primary 的来源在 y=643–873，ring 的来源从 y=697 起；390 下来源从 y=258 起，全部在视口内。 |
| UI-115 | 网格内 ↺ 只有图标，看不到范围 | **部分解决** | 胶囊写出了模式（「Overridden · Light」）；未覆盖格的 ↺ 变灰（#98A2B3）。↺ 本身仍没有可见文字或提示，aria-label 待核实。 |
| UI-18 | 小节标题与字段 label 同样式 | **部分解决** | 现在只有颜色不同：U2-controls 中「Typography」是 #475467 13px/600，「Sans font stack」是 #1D2939 13px/600，标题反而比 label 弱。来源块标题「Fine adjustment source」与键「Effective value」完全同样式（#475467 13px/600）。 |
| UI-22 | 诊断区嵌套标题倒置、正文大于标题 | **部分解决** | 重复标题已去掉，字体条目之间加了分隔线，正文改为 12px。仍有问题：外层 summary「Diagnostics · 5 text pairs…」是常规字重，内层「Text contrast pairs · 5 below 4.5:1」是 600；内外层重复计数；y=131–175 有 44px 空白；字体组既没有标题也没有缩进（x=20），对比组缩进到 x=32。 |
| UI-116 | Locate 与色对的对应关系不清 | **已解决** | 改为每行一对、行间分隔线、Locate 右对齐（U2-group-reset、U2-low-contrast）。末行双线见 新-6。 |
| UI-117 | 两处比值精度不同 | **已解决** | 预览条和编辑列都显示「1.00:1」「2.62:1」。 |
| UI-118 | 错误文字重复上方 helper | **已解决** | U2-field-error：出错时错误文字替换了 helper。 |
| UI-119 | 选中的分体控件只高亮左半段 | **已解决** | 左段、↺ 段和中间分隔都是 #84CAFF，底色都是 #EFF8FF。 |
| UI-120 | 句中色值不是等宽字 | **已解决** | 「Light: #d6409f」已用等宽字（连同「Light:」一起等宽，见 新-5）。 |
| UI-121 | Override / Overridden 两种措辞 | **已解决** | 网格胶囊、「Overridden only (23)」、配置胶囊都统一为 Overridden。 |
| UI-122 | 范围文案「−0.5… 0.5em」像被截断 | **已解决** | 现为「resolved range −0.5em to 0.5em.」 |
| UI-123 | 来源块里 token 名出现两次 | **已解决** | 拾色区标题从「primary」改成了「Color controls」。 |
| UI-124 | Diagnostics 与 Preview diagnostics 标题重复 | **已解决** | U2-low-contrast 中不再有「Preview diagnostics」。 |

R2 的三条建议都还没落实：来源块只给当前模式；primary 是中性灰时拾色平面仍是饱和蓝（U2-source-overridden-detail）；「Overridden only (23)」没有分模式计数。

## 3. 差距清单

先按严重度、再按可挽回分排序。

#### 新-1 · [P2] 网格状态胶囊折成两行椭圆，字号小于 caption，与配置组状态标识形状不一致

- **单元与截图**：U2-source-overridden-1440-light、-detail、-generated、-token-released 中所有被覆盖的格。例如 background 格的胶囊在 x≈50–125、y≈239–268：「Overridden ·」和「Light」上下两行挤在全圆角胶囊里，「L」贴着左侧弧边。
- **维度**：视觉、布局
- **现状**：
  - 胶囊文字包围盒高 8px，约等于 11px 字号。
  - 同一种状态在 U2-group-reset 的配置组里是 4px 圆角矩形加前导圆点（「● Overridden」「● No overrides」），一个概念用了两种形状。
  - 390 下格子更宽，胶囊能放在一行内，所以只有 1440 的 320px 编辑列出问题。
- **期望**：
  - C3：状态以文字加标识清楚呈现，壳层用统一的状态标识；
  - G3：caption 12px；
  - F4：允许换行的是长 token 名和错误文字，状态标识不应碎成椭圆。
- **修法**：网格格内直接复用配置组的状态标识组件（4px 圆角、#F9FAFB 底、#D0D5DD 边、#475467、12px/600、前导 ●）。
  - 内部拆成两个 `display:inline-block` 的 span：「● Overridden」和「· Light」，各自 `white-space:nowrap`。这样换行只发生在两段之间，两行时呈矩形标签而不是椭圆。
  - 标识放在格内容的整行（从色块左缘 x≈26 起，`grid-column: 1 / -1`），宽度可多出约 23px。
  - 也可以改成不带外框的 12px caption 行「● Overridden · Light」，配 `text-wrap: balance`。
- **可挽回分**：0.2

#### UI-22 · [P3] 诊断区层级倒置、空白和缩进不一（部分解决）

- **单元与截图**：U2-low-contrast-1440-light 编辑列 y≈84–900。
- **维度**：视觉、布局、细节
- **现状**：
  - 外层 summary「▼ Diagnostics · 5 text pairs below 4.5:1」是 13px 常规，内层「▼ Text contrast pairs · 5 below 4.5:1」是 13px/600，子级比父级重。
  - 内外两层重复「5 … below 4.5:1」。
  - 「Authored colors fit the sRGB preview.」之后 y=131–175 有 44px 空白，而栈间距应为 16px。
  - 字体诊断三条没有自己的标题，从 x=20 起；对比组是嵌套 details，从 x=32 起，两组结构不同。
- **期望**：G2 栈间距 16px；G3 label 13px/600、caption 12px，层级清楚。
- **修法**：
  - 外层 summary 改为 13px/600 #1D2939；内层 summary 改为 13px/600 #475467，只写「Text contrast pairs」，不再重复计数。
  - 字体三条包进同级的 `<details>`（「Fonts · 3」），与对比组同样缩进 12px。
  - 查掉 sRGB 说明后面那个空容器：给它加 `:empty{display:none}`，或把间距统一为 `gap:16px`。
- **可挽回分**：0.15

#### UI-18 · [P3] 小节标题弱于字段 label，来源块标题与键同样式（部分解决）

- **单元与截图**：
  - U2-controls-1440-light：「Typography」在 y≈251，「Sans font stack」在 y≈291；
  - U2-field-error-1440-light：「Geometry」在 y≈394，「Corner radius」在 y≈434；
  - U2-source-overridden-1440-light：「Fine adjustment source」在 y≈643，「Effective value」在 y≈709。
- **维度**：视觉
- **现状**：小节标题是 #475467 13px/600，字段 label 是 #1D2939 13px/600，字号字重相同，标题因为颜色更浅反而退后。来源块标题与它内部的键完全同样式。
- **期望**：G3 label 13px/600；壳层用分隔线与分组行表达层级（G2）。
- **修法**：二选一。
  - 小节标题改为分组眉题：12px/600、`text-transform:uppercase`、`letter-spacing:.04em`、#475467，上方 24px、下方 12px，上方加 #EAECF0 分隔线。
  - 或者小节标题改为 14px/600 #1D2939，明显大于 label。
  - 来源块的键另行降级（见 新-5）。
- **可挽回分**：0.1

#### 新-2 · [P3] 所选格无覆盖可解除时，↺ 仍呈启用的蓝色

- **单元与截图**：
  - U2-source-generated-1440-light：ring 格的 ↺（x≈143、y≈637）是 #175CD3；同屏未覆盖的 accent、destructive 的 ↺（x≈143、y≈310 / 375）是 #98A2B3。
  - U2-token-released-1440-light：primary 已解除，↺（y≈303）仍是蓝色，下方「Clear primary · Light only」却已禁用（#F9FAFB 底、#98A2B3 字）。
- **维度**：交互与状态
- **现状 → 期望**：选中态盖掉了禁用态，网格和来源块给出相反的可操作信号 → F2 重置须有明确作用域和可用性；C3 状态不只靠颜色。
- **修法**：让禁用态优先。例如 `.token-cell[aria-selected=true] .token-reset[aria-disabled=true]{color:#98A2B3;cursor:default}`，选中态只作用于外框和名称。
- **可挽回分**：0.1

#### 新-8 · [P3] 分组解除只给计数，看不出会影响哪些 token

- **单元与截图**：U2-group-reset-1440-light。
  - Neutral：「● Overridden · Light 15 · Dark 15」+「Use configuration values · both modes」（y≈635–690）；
  - Panel：「Light 2 · Dark 2」（y≈841）。
- **维度**：交互与状态
- **现状**：
  - 网格按 Surfaces、Semantic colors、Borders & focus 分组，和配置组（Brand、Neutral、Panel）不对应。用户无法知道这 15 项是哪些。
  - 本轮可见的佐证：Brand 分组解除后，预览侧多出 `sidebar-primary-foreground / sidebar-primary`，说明 Brand 组也包含 sidebar 系 token。
- **期望**：F2「解除将影响哪些项」。
- **修法**：把「Light 15 · Dark 15」做成 disclosure 按钮，展开后按模式列出 token 名（等宽 12px、可换行）；或者点击计数时，把网格按该组筛选到「Overridden only」。
- **可挽回分**：0.1

#### 新-5 · [P3] 来源块键值结构不一致

- **单元与截图**：U2-source-overridden-1440-light y≈643–873（-detail 图 y≈389–620 同）。
- **维度**：细节
- **现状**：
  - 只有「Effective value」是双列键值行（键 #475467 600，值从 x=196 起）。
  - token 名 `primary`、来源类型「Explicit document override」、路径 `overrides.light.primary` 是三行没有键名的单行，行距相同，彼此看不出关联。
  - 「Light: #d6409f」中的模式名「Light:」也用了等宽字。
- **期望**：F2 来源说明分别回答「来自哪里 / 哪个覆盖生效 / 解除影响」；G3 颜色值等宽 12px，模式名不是值。
- **修法**：改为 `<dl>`，`grid-template-columns: 7.5rem 1fr; gap: 6px 12px`。
  - 键依次为 Token / Effective value / Source / If cleared。
  - 路径作为 Source 下的第二行，用 12px 等宽 #475467。
  - 键用 12px/600 #475467，值用 13px #1D2939。
  - 「Light」用正文字体，只有 `#d6409f` 用等宽。
- **可挽回分**：0.1

#### UI-107 · [P3] 计数与「No overrides」仍用 #667085（部分解决）

- **单元与截图**：U2-group-reset-1440-light：「● No overrides」的文字和「Light 0 · Dark 0」（y≈476）、「Light 15 · Dark 15」（y≈646）、「Light 2 · Dark 2」（y≈841）。
- **维度**：视觉
- **现状 → 期望**：这些是正常状态信息，不是禁用 → G3 次级文字 #475467。
- **修法**：统一用 `--shell-text-secondary: #475467`，#667085 只留给禁用态和占位符。
- **可挽回分**：0.05

#### 新-3 · [P3] 内置字体 select 的值是 600 字重

- **单元与截图**：U2-controls-1440-light 三处「Custom author stack」（y≈357 / 595 / 832）。对照 U2-group-reset 的「Inherit」「Pink」「Auto」都是常规字重。
- **维度**：视觉
- **现状 → 期望**：同类 select 的值字重不同 → G3 body 14px/1.5 常规。
- **修法**：这个 select 的值改为 `font-weight:400; color:#1D2939`，与其他 select 一致。
- **可挽回分**：0.05

#### UI-106 · [P3] 配置组状态行与解除按钮贴合（U2 部分，仍存在）

- **单元与截图**：U2-group-reset-1440-light。Brand 的胶囊在 y=465–488、按钮在 y=489–520；Neutral、Panel 同样贴合。
- **维度**：布局
- **期望**：G2 栈间距（组内至少 8px）。
- **修法**：给配置组加 `display:grid; gap:8px`，或给解除按钮加 `margin-top:8px`。
- **可挽回分**：0.05

#### 新-4 · [P3] 390 下所选格被吸顶区盖住

- **单元与截图**：U2-source-overridden-390-light。吸顶头和标签占 y=0–200，所选 primary 格只露出胶囊那一行（y≈198–220），名称和值都在吸顶区下面。
- **维度**：布局
- **现状 → 期望**：所选格与来源块失去视觉上的对应 → G1、G2（≤600px 下编辑区稳定，不遮挡）；F1 来源对应到所选 token。
- **修法**：给 token 格（或滚动容器）设置 `scroll-margin-top`（或 `scroll-padding-top`）为 `calc(var(--mobile-sticky-h) + 8px)`；选中时滚到所选行（`scrollIntoView({block:'start'})`），不要滚到来源块。
- **可挽回分**：0.05

#### UI-115 · [P3] 网格 ↺ 仍只有图标（部分解决）

- **单元与截图**：U2-source-overridden-1440-light 各格右侧的 ↺ 段。
- **维度**：交互与状态
- **现状 → 期望**：模式已经由相邻胶囊写出，但 ↺ 本身没有可见文字或提示 → F2 显示范围；C3 不只靠悬停。
- **修法**：给 ↺ 加 `aria-label` 和 `title`，内容与来源块按钮同文案（「Clear primary · Light only」）；补拍悬停和键盘聚焦帧以证明可见。
- **可挽回分**：0.05

#### 新-7 · [P3] 「Restore document · Graphite」没有写范围

- **单元与截图**：U2-restore-1440-light，按钮在 y≈196–227，附近没有说明文字。
- **维度**：交互与状态
- **现状 → 期望**：另两档重置都带范围后缀（「· Light only」「· both modes」），第三档只写了目标名 → F2「恢复预设/文档」须显示范围。
- **修法**：按钮下加 12px #475467 的说明，例如「Discards configuration changes and all 23 overrides in both modes」；或把按钮文案改为「Restore document · Graphite · all groups, both modes」。
- **可挽回分**：0.05

#### 新-6 · [P3] 预览侧诊断条末行分隔线与外框形成双线

- **单元与截图**：U2-group-reset-1440-light，末行分隔线在 y=418，外框底边在 y=431；U2-source-overridden、U2-low-contrast 同样。
- **维度**：细节
- **修法**：`.diag-row:last-child{border-bottom:0}`。
- **可挽回分**：0.05

#### 新-9 · [P3] 颜色搜索框与 Clear 按钮不等高

- **单元与截图**：U2-source-overridden-1440-light，搜索框 y=73–108（36px），「Clear」y=75–106（32px）。
- **维度**：细节
- **修法**：两者同高 36px（按钮 `height:36px`，或行容器 `align-items:stretch`）。
- **可挽回分**：0.05

可挽回分合计 1.15，与 10 − 8.7 = 1.3 相差 0.15，在 0.5 的误差内。

## 4. 建议（不扣分）

- 来源块同时给出另一模式的状态，例如「Dark: not overridden」（R2 提出，仍未做）。
- primary 为中性灰时，拾色平面不要显示饱和蓝（U2-source-overridden-detail 中 #606060 对应蓝色平面，R2 提出，仍未做）。
- 「Overridden only (23)」写出分模式计数（R2 提出，仍未做）。
- 预览侧诊断条超过 4 对时折叠或限高。U2-low-contrast 中 5 对占 y=187–481，把预览往下推了约 300px。
- Borders & focus 等没有配对的组只占左列，右半列空着（U2-source-generated y≈443–660）。可以让无配对组按两列流式排布。
- 390 下吸顶区占 200/844px（24%）。可以滚动时收起头区、只保留 Edit/Preview 标签。这条主要属 U1 的 G2。
- 「Restore document」放在预设区之上，离精调区很远。可以在精调区末尾放一个同功能入口。

## 5. 看不出的格子

- **U2-source-overridden-1440 / -390**：token 格是点击选中的，没有拍到 token 格和 ↺ 的键盘焦点帧，判断不了 G4 的焦点可见。
- **拾色字段焦点**：没有拍到 F4 的焦点帧（picker 蓝 40%、3px、2px offset）。
- **↺ 的悬停提示与 aria-label**：截图看不出。
- **Locate 点击后的定位结果**：没有拍到。
- **分组解除之前的状态**：U2-group-reset 只拍了解除之后，看不到 Brand 组原来的计数和启用态按钮。
- **ring 的「Clear ring · Light only」**：在 U2-source-generated 中落在视口之外（y>900），看不到它的禁用态。
- **390 下的其他状态**：390 只拍了来源这一帧，没有单项解除、分组解除、字段错误、低对比。
- **Dark 模式下的覆盖与来源**：没有拍到。

## 6. 总评

- U2 从 R2 的 7.75 升到 **8.7**。R2 最主要的问题 UI-17（选中后看不到来源）已解决，常驻壳层也压到了 57px。R2 的 17 条差距中 11 条已关闭。
- 剩余的主要失分：
  - 新-1：1440 网格里每个被覆盖格的状态胶囊都折成两行椭圆，形状也和配置组的状态标识不一致。这是本轮唯一的 P2，也是最显眼的问题。
  - 层级：小节标题比字段 label 弱（UI-18），诊断区外层比内层轻（UI-22）。
  - 重置的状态与范围表达不完整：选中格的 ↺ 在无可解除时仍像可用（新-2）；分组解除不列出 token（新-8）；文档恢复没有写范围（新-7）。
- 关于是否需要结构性改动：
  - 不需要推倒重做布局。
  - 新-1 需要改组件：状态胶囊换成配置组那套分段、不换行的标识组件，或改成不带外框的 caption 行。只调 padding 修不好，因为 320px 编辑列里两列配对网格每格的文字区只有约 76px。
  - 新-5 和 UI-18 需要把来源块改成 `<dl>` 键值结构，并重新定义小节标题样式。
