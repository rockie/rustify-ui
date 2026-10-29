# R2 评审结论 · U2 摘录

摘自 R2 合并结论 judge.md，只含 U2 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U2 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U2 Studio 精调与来源解释 | 2.2 | 2.4 | 2.65 | 0.5 | **7.75** | a | — |


### 各维度理由

**U2（a）**

- 布局 2.2：「搜索 → 筛选 → 网格 → 来源 → 拾色」顺序、预览侧诊断条、字段顺序与分隔线已对。扣分原因：选中 token 后来源块仍在屏外，且常驻壳层占 1440 编辑列约 315/900px；390 下两层吸顶之间漏出被裁切的网格内容。
- 视觉 2.4：错误色 #b42318、焦点环、标签页下划线、分体控件边框已对上 token。扣分原因：小节标题与字段 label 同样式；诊断区嵌套标题比外层大、正文比标题大；Override 胶囊借用选中蓝；选中分体控件只高亮一半；搜索框边框与 caption 灰偏离 G3。
- 交互 2.65：选中显示来源、单项解除、分组解除写明 both modes、非法输入保留 `abc` 并贴字段报错、预览侧诊断带 Locate 都走通。扣分原因：选中后不把来源带进视口；网格内 ↺ 只有图标，看不到作用范围。
- 细节 0.5：比值精度不一、错误文字重复 helper、句中色值非等宽、Override/Overridden 两种措辞、范围文案像被截断、token 名重复、诊断标题重复。


## R2 未关闭差距（U2）

#### UI-102 · [P2] 390 编辑页的 Edit/Preview 标签条与粘性头之间有空隙，滚动内容从中透出（U1/U2 · 布局 · 可挽回 U1 a 0.2 / b 0.15，U2 a 0.25）

- 位置：U1-configured-390-light y≈45–85（「Presets … 44」半截）；U2-source-overridden-390-light y≈45–85（「sidebar-border #d0d0d0」被上下两层切开）。
- 现状 → 期望：两层吸顶之间漏出内容 → G1、G2（≤600px 编辑/预览分区稳定，不遮挡、不截断）。
- 修法：标签条与头区放进同一个不透明粘性容器，或头区 `top` 设为标签条高度并把间距改为粘性元素自身的 `padding-top`；或 ≤600px 时头区不吸顶。
- 来源：U1 a 新-1、b 新-2；U2 a 新-1（同一机制：两层吸顶之间的空隙，同在 390、同 y 范围）。

#### UI-106 · [P3] 编辑区顶部 padding 16px、两栏标题不对齐，组内状态行与按钮间距 0px（U1/U2 · 布局、细节 · 可挽回 U1 a 0.15 + 0.1 / b 0.15，U2 a 0.05）

- 位置：U1-default-1440-light 编辑列首元素顶在 y=16，左右标题中线差约 8px；U1-configured-1440-light「• Overridden」胶囊底 y=745 与「Use configuration values · both modes」顶 y=746 相接；U2-group-reset-1440-light Brand color 下「● No overrides」胶囊与按钮相距约 1–2px。
- 现状 → 期望：G2 编辑区 padding 24px 20px 48px、栈间距 16px。
- 修法：编辑区（或粘性头内）`padding-top: 1.5rem`；标题行 `align-items:center`；配置组内 `display:grid; gap:.5rem`，状态行与按钮至少 8px。
- 来源：U1 a 新-6、a 新-9、b 新-7（b 把两点并为一条）；U2 a 新-3（同一组件：配置组状态行与解除按钮贴合）。

#### UI-107 · [P3] 次级文字用 #667085、颜色搜索框边框用 #D0D5DD，偏离 G3 token（U1/U2 · 视觉 · 可挽回 U1 a 0.1 / b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light 的 Theme configuration 说明、Appearance/Brand color 帮助文字、「Light 6 · Dark 6」、「Resolved neutral: mauve.」；U2-controls-1440-light「Choose a bundled font」caption；U2-source-overridden-1440-light「Search colors…」边框。
- 现状 → 期望：两种次级灰并存，颜色搜索框与预设搜索框边框不同 → G3 次级文字 #475467、搜索框边框 #98A2B3。
- 修法：helper/caption 统一 `--shell-text-secondary: #475467`；颜色搜索框复用预设搜索框的 #98A2B3 边框；#667085 只留给禁用态。
- 来源：U1 a 新-3、b 新-5；U2 a 新-6（同一 G3 token 偏差）。

#### UI-109 · [P3] 状态胶囊（Overridden、Custom、网格 Override）借用选中态蓝色（U1/U2 · 视觉 · 可挽回 U1 b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light、U1-configured-lower-1440-light「• Overridden」「• Custom」（#EFF8FF/#84CAFF/#175CD3）；U2-source-overridden-1440-light 网格「Override」胶囊（选中的 primary 格内只剩一道描边）。
- 现状 → 期望：状态与选中、动作同色 → G3 蓝色用于选中与焦点；C3 状态以文字 + 标识表达（已满足）。
- 修法：状态胶囊改中性（#F9FAFB 底、#D0D5DD 边、#475467 字，12px/600，保留前导圆点）。
- 来源：U1 b 新-10（a 在建议 2 提出）；U2 a 新-4（并注明 U1 胶囊同色，建议同步）。

#### UI-17 · [P2] 选中 token 后来源与解除按钮不在屏内（U2 · 布局、交互 · 仍存在 · 可挽回 a 0.55）

- 位置：U2-source-overridden-1440-light 选中 primary 后整屏看不到「Fine adjustment source」；U2-source-overridden-detail-1440-light 来源块在整片网格之后（Shadow 组后）；U2-source-overridden-390-light 同样。1440 编辑列 0–315px 为常驻壳层。
- 现状 → 期望：要滚过约 50 个 token 才能读到来源 → F1 展示有效值、来源、覆盖与重置；F2 来源回答三个问题。
- 修法：来源 + 拾色块作为整行插入所选格所在行下方（`grid-column: 1 / -1`），或做成编辑列内 `position: sticky; bottom: 0` 的可折叠抽屉；选中时 `scrollIntoView({block:'nearest'})`；同时压缩常驻区（见 UI-101/UI-103）。
- 来源：a UI-17。

#### UI-115 · [P2] 网格内 ↺ 只有图标，看不到解除范围（U2 · 交互 · 可挽回 a 0.15）

- 位置：U2-source-overridden-1440-light 及 detail 图每个 token 格右侧 ↺ 段。
- 现状 → 期望：来源块写「Clear primary · Light only」，网格里同一动作只有图标 → F2 解除须显示范围；C3 不只靠颜色或悬停。
- 修法：胶囊文字带模式（「Override · Light」）；↺ 加与来源块同文案的 `aria-label`；未覆盖格 `aria-disabled`。
- 来源：a 新-8。

#### UI-18 · [P3] 小节标题与字段 label 同样式，层级平（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-controls-1440-light「Typography」对「Sans font stack」；U2-field-error-1440-light「Geometry」对「Corner radius」，同为 #1D2939、同字号字重。
- 修法：小节标题用壳层分组眉题样式（12px/600 大写、#475467，上分隔线 24px、下 12px），字段 label 保持 13px/600。
- 来源：a UI-18 余项。

#### UI-22 · [P3] 诊断区嵌套标题倒置、正文大于标题、等宽字号混用（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-low-contrast-1440-light 编辑列 y≈352–700：外层 summary 约 12px、内层约 13px；「Authored colors fit…」约 14px 大于其标题；字体诊断条目之间无间距。
- 修法：两级 summary 统一 13px/600、内层缩进 12px；正文 12px #475467；token 名统一 12px 等宽；条目间 8px 加分隔线。
- 来源：a UI-22 余项。

#### UI-116 · [P3] 预览侧诊断条中 Locate 与所属色对的对应关系不清（U2 · 布局 · 可挽回 a 0.1）

- 位置：U2-source-overridden-1440-light 诊断条（色对文字到 Locate 相隔约 220px）；U2-group-reset、U2-controls 三列时 Locate 与下一对只隔 16px。
- 修法：单列 `grid-template-columns: 1fr auto`，每行一对，行间分隔线；超过 4 对时折叠。
- 来源：a 新-2。

#### UI-117 · [P3] 同一对比数据两处精度不同（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440-light 预览条「1.00:1」「2.62:1」，编辑列「1.000:1」「2.623:1」。
- 来源：a 新-10。

#### UI-118 · [P3] 错误文字重复了上方 helper（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-field-error-1440-light Corner radius 下方。
- 修法：出错时用错误文字替换 helper（同一 `aria-describedby` 节点）。
- 来源：a 新-11。

#### UI-119 · [P3] 选中的分体控件只高亮左半段（U2 · 视觉 · 可挽回 a 0.05）

- 位置：U2-source-overridden-1440-light primary 格：左段 #84CAFF/#EFF8FF，↺ 段仍 #D0D5DD/白。
- 修法：选中样式加在分体控件外层，外框与中间分隔统一为 #84CAFF（F4 外缘圆角的分体控件）。
- 来源：a 新-5。

#### UI-120 · [P3] 句中色值不是等宽字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-source-overridden-detail-1440-light「Clearing this override reveals Light: #d6409f.」。
- 来源：a 新-12。

#### UI-121 · [P3] 同一概念两种措辞：Override / Overridden（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-13。

#### UI-122 · [P3] 范围文案「−0.5… 0.5em」像被截断（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-field-error-1440-light Letter spacing helper。
- 来源：a 新-14。

#### UI-123 · [P3] 来源块里 token 名出现两次（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-15。

#### UI-124 · [P3] 诊断区「Diagnostics」与「Preview diagnostics」标题重复（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-16。


## R2 建议（U2，不扣分）

- U2：
  - 来源块同时给出另一模式的状态，并附分组级解除入口。
  - primary 为中性灰时拾色平面不要显示饱和蓝。
  - 「Overridden only」写出分模式计数。
