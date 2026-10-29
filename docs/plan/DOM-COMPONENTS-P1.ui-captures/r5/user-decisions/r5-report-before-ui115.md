# DOM-COMPONENTS-P1 · UI 验收评审 R5

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R5（全量；与 R3/R4 同一基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [目标包](DOM-COMPONENTS-P1.ui-target.md) · `567117725f56` | HEAD `941a8043`；产品路径无 diff；Studio `5b7550dc7339ced2` http://127.0.0.1:4181/；catalog `32b52b66f04701ad` http://127.0.0.1:4176/ | [r5](DOM-COMPONENTS-P1.ui-captures/r5/) · [采集清单](DOM-COMPONENTS-P1.ui-captures/r5/capture.md) | [R4](DOM-COMPONENTS-P1.ui-review.r4.md) |

## 验收判定

**需要修改 · 待裁定（UI-115，非门槛项）**

本轮分数 **9.6/10（U7）**，门槛 **8/10**，沿用 R4；历史最佳 **9.6**。独立评审的七个单元都超过门槛，但交卷后的真实键盘检查发现 **UI-215（P1）**：导入格式、导出格式、CSS profile、颜色记法选择器获得键盘焦点后没有可见焦点标识，等待 700ms 后仍然如此。它阻止本轮通过；机器检查不改独立评分。

R4 的唯一 P1 **UI-207 已解决**，主 agent 实测 Confirm 现在正确消费局部深色主题。上轮主 agent 的 UI-212（200% DOM 预览裁切）也已解决。UI-128 所在 U3 已按修复记录完成一次重构轮，独立评审判该差距已解决，原停滞预警解除。

本轮没有停滞或新的停滞预警。UI-115 的修复记录为“不采纳”，本轮两位评审仍判可见范围说明不足，因此按合同转为待裁定；它是 P3，不阻塞其余修复。**本轮不修产品代码、不提交、不改计划进度，也不启动下一步修复。**

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R4 |
| --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.8 |
| U2 精调与来源 | 3.00 | 3.00 | 2.95 | 1.00 | **9.95** | 9.4 |
| U3 导入与导出 | 2.95 | 3.00 | 3.00 | 0.95 | **9.90** | 9.35 |
| U4 迁移与存储反馈 | 3.00 | 3.00 | 3.00 | 0.95 | **9.95** | 9.55 |
| U5 预览 | 2.95 | 3.00 | 3.00 | 1.00 | **9.95** | 9.45 |
| U6 Button / IconButton / Link / ThemePanel | 3.0 | 2.99 | 3.0 | 1.0 | **9.99** | 9.95 |
| U7 布局与排版 | 3.0 | 2.6 | 3.0 | 1.0 | **9.6** | 9.9 |

- 单元分与逐维理由来自 [judge.md](DOM-COMPONENTS-P1.ui-captures/r5/judge.md)，八份原文各自保留。主 agent 没有调分或删除差距。
- U2 A（首采）与 B（补采后重评）都是 **3 / 3 / 2.95 / 1 = 9.95**；选 B 的整组评分和理由，没有逐维拼接。A 的 UI-172 待核实来自入口未展开；新增两张真实截图后，B 核实清单并判已解决。这不是同批材料的确认分歧，A 原文与原清单未覆盖。
- U7 从 9.9 降到 9.6，原因是新发现了 R4 原图也已有的 Kbd 圆角差异；不是目标变更，也没有声称该差异是回归。
- 本轮将判“需要修改”，不触发通过确认评审；最低分 9.6 也不在门槛 + 0.5 以下。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |
| R5 | **9.6（U7）** | **9.6** | **0 / 1** | **UI-128 所在 U3** | 同一基线；UI-128、UI-207 已解决；新增 P1 UI-215 来自纯键盘实测 |

R1–R4 行沿用上轮轨迹。当前基线从 R3 起计算；R5 的 U3 重构登记取自 [M7 · R4 → 修复](DOM-COMPONENTS-P1.records/M7.md)，不是本轮主 agent 新做的实现。

## 上轮差距复核

上轮共 **31 条**：独立评审 30 条、主 agent 1 条。修复记录的最终处置为 **28 条已修改待评审、3 条不采纳**（UI-172、UI-115、UI-160）。本轮结果为 **27 条已解决、2 条部分解决、1 条仍存在、1 条回归**；另新增 3 条。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-207 | [P1] 深色局部边界内打开的对话框：表面随边界变深，主按钮 Confirm 仍取根主题浅色 primary（回归） | 已修改待评审 | 已解决 | [judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U5-a.md)；[machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup.json) |
| UI-201 | [P2] 配额态：同一个失败的命名保存有三个重试入口、三种叫法，加上页头一共三个实心蓝按钮 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-128 | [P2] 〔停滞预警〕Source 行的值排在「Current」列下，读起来像是当前主题的属性 | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-212 | [P2] 200% 文本下 DOM 预览场景被裁：标题断在字中间，「Create · 0」完全在可视区外，卡片挤成约 65px | 已修改待评审 | 已解决 | [machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup2.json)；DOM surface 300/300，Create 完整可见 |
| UI-198 | [P3] 同一对话框里两张差异表的列结构、列位置和表头样式不一致 | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-194 | [P3] 单列网格仍沿用「配对行」结构：来源块没有紧跟所选格，配对内外间距倒置 | 已修改待评审 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-176 | [P3] 徽标基线偏低 2px；390 候选区徽标缩进 8px；Name 行两边基线差 1px | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-02 | [P3] EN/中文、Edit/Preview 段间仍画两条线，与 Light/Dark、DOM/GPU 的单线画法不一致 | 已修改待评审 | 已解决 | [judge-U1-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U1-a.md) |
| UI-193 | [P3] 中文态预览诊断标题字重混排：汉字常规，数字与拉丁字符半粗 | 已修改待评审 | 已解决 | [judge-U1-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U1-a.md) |
| UI-172 | [P3] 分组解除只给计数，看不出会影响哪些 token | 不采纳 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md)；补采展开后核实两模式 token 清单 |
| UI-195 | [P3] 390 下吸顶的 Edit/Preview 条没有下分隔线，「Semantic colors」组标题被切成半截 | 已修改待评审 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-199 | [P3] 6 个「Unchanged」与 1 个「Changed」徽标同样式，且左缘参差 | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-203 | [P3] callout 和命名卡之间成句重复 | 已修改待评审 | 部分解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-196 | [P3] 所选格里的模式文字「Light」在选中时变粗 | 已修改待评审 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-200 | [P3] Import notes 把整条路径排成等宽字，与表格写法不一致 | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-18 | [P3] Colors 页节标题、分组标题仍是旧样式 | 已修改待评审 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-25 | [P3] Source 行比其余候选行矮 3px | 已修改待评审 | 已解决 | [judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) |
| UI-115 | [P3] 网格 ↺ 仍只有图标 | 不采纳 | 仍存在 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-197 | [P3] 诊断区两个同级 summary 一个带计数、一个不带 | 已修改待评审 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |
| UI-202 | [P3] 配额 callout 标题折行后第二行只剩孤词，也没有写出失败原因 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-204 | [P3] 冲突面板比 900px 视口高，默认帧切掉了「Download other version」 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-205 | [P3] 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-76 | [P3] DOM 与 GPU 两框首行纵向差 9px（横向已对齐） | 已修改待评审 | 回归 | [judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U5-a.md)；纵向差缩小，横向内缩回归 |
| UI-208 | [P3] 预览对话框 Confirm 为 79×35，与同一预览里 32px 高的 solid 按钮不一致（R3 为 77×33） | 已修改待评审 | 已解决 | [judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U5-a.md) |
| UI-209 | [P3] 卡片选中框内圈在滚动区边缘画出闭合底边，看上去像卡片在 y=720 处结束 | 已修改待评审 | 已解决 | [judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U5-a.md) |
| UI-210 | [P3] 深色下 classic 底边暗线变浅 | 已修改待评审 | 部分解决 | [judge-U6-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U6-a.md) |
| UI-189 | [P3] Reset 的 input、textarea 虚线标记越过内容右沿 | 已修改待评审 | 已解决 | [judge-U7-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U7-a.md) |
| UI-211 | [P3] 240px 显式宽度样例的正文贴着左边框 | 已修改待评审 | 已解决 | [judge-U7-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U7-a.md) |
| UI-56 | [P3] 操作展开内容：说明段没有缩进，列表已缩进 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-206 | [P3] 草稿差异块有三条左缘 | 已修改待评审 | 已解决 | [judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) |
| UI-160 | [P3] 颜色搜索框与 Clear 不等高 | 不采纳 | 已解决 | [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md) |

UI-76 原文写“部分解决，横向回归”，按更严格的“回归”登记，纵向改善仍保留。UI-172、UI-160 不因修复者不采纳就自动关闭，而是由本轮独立评审核实后关闭。

## 差距清单

共 **7 条**：P1 1 条、P2 1 条、P3 5 条；没有 P0。独立评审来源 6 条，机器检查来源 1 条。新编号固定为 **U3 新-1 → UI-213，U7 新-1 → UI-214，机器检查 → UI-215**；旧编号不复用。

### UI-215 · [P1] 导入/导出选择器没有可见键盘焦点（U3 · 交互与状态 · 机器检查）

- 位置：[MC-export-format-pure-keyboard-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-export-format-pure-keyboard-1440-light.png) 的 Output；[MC-import-format-pure-keyboard-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-import-format-pure-keyboard-1440-light.png) 的 Input format；CSS 的 [MC-export-profile-keyboard-focus-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-export-profile-keyboard-focus-1440-light.png)、[MC-export-color-format-keyboard-focus-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-export-color-format-keyboard-focus-1440-light.png)。
- 实际复现：新页从文档起点 **Tab 5 次 → Export → Enter → Tab 1 次**；导入为 **Tab 4 次 → Import → Enter → Tab 1 次**。停留 700ms 后拍摄，焦点分别仍在 `export-kind`、`import-format`。CSS 两个选择器通过 Shift+Tab、Tab 聚焦并同样停留 700ms。
- 页面实测：四个触发器均 `:focus-visible=true`，但 `outline-style:none`、`box-shadow:none`，边框仍是普通 `#d0d5dd`；截图也没有可见焦点。不是过渡瞬时值。相邻控件作对照：[MC-export-next-control-keyboard-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-export-next-control-keyboard-1440-light.png) 的代码区有 3px 蓝 outline，Choose file 有 2px outline。完整数据：[machine-focus-confirm.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-focus-confirm.json)。
- 现状 → 期望：键盘用户切到格式选择器时无法辨认当前焦点 → G4 要求新增控件键盘可用、焦点可见；DESIGN.md Inputs 要求 Select 有可见焦点环。
- 修法：给导入/导出中的 Select trigger 恢复清楚的 `:focus-visible` 样式，覆盖 Output、Input format、CSS profile、Color notation，可复用壳层的蓝色 outline 与 offset。用同一条纯键盘路径复验，保留现有 Tab 限制、Enter 操作和 Esc 回焦。
- 来源：主 agent 在独立评审交卷后运行机器检查并实拍核实；首次核实，不据旧截图推断引入轮次。**不改 U3 的独立评分，但计入门槛项。**

### UI-214 · [P2] Kbd 大字级圆角偏小（U7 · 视觉 · 可挽回 0.4）

- 位置：[U7-type-kbd-matrix-1280-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-1280-light.png)、[U7-type-kbd-matrix-390-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-390-light.png) 的 classic/soft size 7–9；对照 [Kbd 目标图](DOM-COMPONENTS-P1.ui-target/radix-sink-kbd.png)。
- 现状 → 期望：size 9 soft 同为约 84×84px，当前圆角约 3px，目标约 18px；size 8 也明显偏方，classic 外框及内高光同样偏方 → U7 比较口径与 L1、L6 的 Kbd 外观/参考尺度。
- 修法：调整 Kbd 的圆角随字级递增的尺度映射，以 size 9 约 18px、size 8 约 10px 校准；classic 外轮廓、内高光和阴影同步，缩放只应用一次，复核全部 1–9 档。
- 来源：[judge-U7-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U7-a.md) 新-1；R4 原图已有，新增发现，不判回归。

### UI-213 · [P3] 导入审阅滚动表头上方露出残行（U3 · 布局、细节 · 可挽回 0.1）

- 位置：[U3-import-review-bottom-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U3-import-review-bottom-1440-light.png) 对话框顶部约 x233–1207、y17–74；end 帧同一区域。
- 现状 → 期望：粘性表头上方 24px 留白仍绘制被遮行的 Source、Changed 与版本残字 → X6 的内部滚动中，表头上沿应有完整不透明遮挡面。
- 修法：将表头锚定实际滚动视口上缘并使用不透明 popover 背景和正确层级；如保留上方 24px 留白，为留白增加同色遮挡，保留初始 1.5rem 内边距。
- 来源：[judge-U3-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U3-a.md) 新-1；未据 R4 断言回归。

### UI-115 · [P3] 网格直接重置入口未显示作用域〔待裁定〕（U2 · 交互与状态 · 可挽回 0.05）

- 位置：[U2-source-overridden-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U2-source-overridden-1440-light.png)、390 对应帧中 token 最右侧的 ↺ 段。
- 现状 → 期望：动作自身只有 ↺，完整 `Clear primary · Light only` 要在来源块另读 → 评审按 F1/F2 要求重置入口明确表达范围。
- 修法（如采纳）：增加常驻短范围文案，如“↺ Light only”，或列表附近常驻同等解释，保留分体控件外形与窄屏换行。
- 来源：两位独立评审均判仍存在，采用 B 的原文 [judge-U2-b.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U2-b.md)。
- 核对说明：主 agent 的真实页内复测确认按钮已有 `aria-label` 与 `title`，两者均为 `Clear primary · Light only`，并有 3px 蓝色键盘焦点，见 [MC-token-reset-keyboard-focus-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-token-reset-keyboard-focus-1440-light.png)、[machine-check-extra.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-extra.json)。这支持修复记录的可访问名称/焦点事实，但不能替评审决定“是否必须常显范围文字”；不改分、不删差距。

### UI-203 · [P3] 存储反馈仍有少量重复文案（U4 · 细节 · 部分解决 · 可挽回 0.05）

- 位置：[U4-quota-failure-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U4-quota-failure-1440-light.png)、quota-operations 的 callout/命名卡；no-locks 两帧的标题/正文。
- 现状 → 期望：同一 Retained name 的失败与保留状态连续重述，无锁原因在标题和首句重复 → 按 S4，整体存储状态与单独命名动作分工明确，不连续重述同一原因。
- 修法：配额 callout 只保留整体状态、草稿保存事实、下载与技术详情，由命名卡独立说明具体动作失败；无锁正文直接说明可在内存编辑并导出，保留内联继续动作。
- 来源：[judge-U4-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U4-a.md) UI-203。

### UI-76 · [P3] DOM/GPU 比较框首行起点不一致（U5 · 布局 · 回归 · 可挽回 0.05）

- 位置：[U5-gpu-compare-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r5/U5-gpu-compare-1440-light.png) 两框首行。
- 现状 → 期望：纵向差由约 8–9px 缩到约 4px，但 DOM 横向内缩约 13px，GPU 约 25px，原已对齐的横向起点回归 → 按既有 UI-76 口径及 G1/P5，比对框首行内缩与基线一致，不要求内部组件布局相同。
- 修法：仅统一两侧比较框首行容器的横向内缩和基线，保留窄容器内卡片所需 padding，避免整体平移 GPU 内容。
- 来源：[judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U5-a.md) UI-76（原文“部分解决，横向回归”）。

### UI-210 · [P3] 深色 classic 最后一行仍略浅（U6 · 视觉 · 部分解决 · 可挽回 0.01）

- 位置：[U6-fixture-1280-dark.png](DOM-COMPONENTS-P1.ui-captures/r5/U6-fixture-1280-dark.png) classic 1–4 底边，四档采样点及原始 RGB 见 [judge-U6-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U6-a.md)。
- 现状 → 期望：四档末行像素明度约为上一行的 0.740，目标约 0.717–0.722；R4 约 0.802，已大幅改善 → 保留顶部/侧边效果，仅再压暗底部最后一行。
- 修法：微调 classic 深色底部 inset shadow 的 alpha 或叠层顺序，四档共享修正并复拍上下/侧边。此处明度是评审对原始 RGB 的加权明度指标，不是 WCAG 对比度。
- 来源：[judge-U6-a.md](DOM-COMPONENTS-P1.ui-captures/r5/judge-U6-a.md) UI-210。

## 待裁定事项

**UI-115（P3，可选打磨）**：M7 本轮最终处置为“不采纳”，理由是已有 aria-label/title、模式标识和键盘焦点；两位独立评审仍要求动作范围常显。可以选择：

1. 接受当前图标 + 模式上下文 + title/可访问名称，对这一处偏差予以接受，下一轮按用户确认改目标口径并重建基线；
2. 要求常显范围文字，交 `apply-ui-review` 按本条修复。

本轮没有用户豁免原话，不把这一项写成已解决或已接受；它不阻碍先修 UI-215 和其他差距。

U3 的一次停滞重构已在 M7 登记，UI-128 本轮已解决；不再发预警，也不新增重构次数。其余未关闭旧差距均为 P3，不触发合同中的 P0–P2 顽固差距条件。全局分数高于门槛，不判全局平台期。

## 建议（不扣分）

- U2 的 Neutral 清单可将 Light/Dark 分成更清楚的小节；当前内容已完整，未扣分。
- U4 长迁移 JSON 审阅时可固定标题栏；目标未要求，不扣分。
- U1 可补键盘确认选项、模式激活和分隔条拖拽记录。
- 沿用 R4 建议：Caller utilities 人为 class 覆盖样例的对比度为 1.24:1，应换为可读的演示配色；不将该演示样例写为默认组件通过 AA。

## 机器检查

所有项目都在本轮真实 release 页面上执行，晚于八份评审原文与合并 judge 的落盘，没有把结果反馈给评审调分。

| 检查 | 工具/证据 | 实际结果 |
| --- | --- | --- |
| 采集健康 | Playwright 1.63.0 / Chromium 153.0.8010.12；[capture-health-summary.json](DOM-COMPONENTS-P1.ui-captures/r5/capture-health-summary.json) | 45 个页面，pageerror、console error、失败请求、HTTP ≥400、CSP 违规均为 0 |
| 机器检查健康 | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check.json)、followup、extra/confirm 执行记录 | 基础检查 console/CSP 0，所有本轮检查命令 exit 0；没有用 exit 0 代替验收通过 |
| 溢出与对话框 | 页内 `scrollWidth`、boundingBox | 所测 1440/768/390/720-root200 页面级横向溢出均为 0；主编辑列 319/319；导出对话框均在视口内 |
| UI-212 | [machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup2.json) + [MC-root200-preview-clip-720-light.png](DOM-COMPONENTS-P1.ui-captures/r5/MC-root200-preview-clip-720-light.png) | 200% DOM surface 300/300、scrollLeft 0；Create 全宽约 113px 可见，旧裁切已解决 |
| 隐藏比较区 | [machine-check-extra.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-extra.json) `nested200` | 一处关闭的 details 后代返回 243/236，但 `checkVisibility=false`；不拿未显示内容的预布局尺寸判可见溢出，正常显示的 DOM surface 为 300/300 |
| 键盘主路径 | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check.json) | Studio Tab 70 次、catalog 61 个焦点均在屏内；导出 25、导入 15 次 Tab 均留在对话框内，Esc 回触发按钮；品牌选择器 Custom → Gold |
| 选择器焦点 | [machine-focus-confirm.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-focus-confirm.json)；4 张格式/CSS 焦点图 | 纯键盘和停留 700ms 后复核，四个选择器没有可见焦点；新增 UI-215（P1）。不能把“焦点在屏内”误写为“有可见标识” |
| B2 Button/IconButton/Link | [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check.json)、[machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup2.json) | 按钮 2px 第8阶 outline，偏移按变体；Link 等过渡稳定后为 2px、offset 2px，聚焦无下划线；不把刚聚焦的过渡中间值当最终样式 |
| UI-207/208 | [machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup.json) | 深色 Confirm 背景 rgb(83,177,253)、字 rgb(12,17,29)，浅色正确；两者高32px、宽约77.3px；原主题门槛项关闭 |
| 下载 | [machine-check-extra.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-extra.json)、[MC-export-theme.json](DOM-COMPONENTS-P1.ui-captures/r5/MC-export-theme.json) | 实际下载 theme.json，2,494 B、schema_version=2，无 download failure，与可见导出 JSON 一致 |
| 可访问性扫描 | @axe-core/playwright 4.13.0；wcag2a/2aa/21a/21aa/22aa；[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check.json)、[machine-check-extra.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-check-extra.json) | Studio 框架/导出态违规0；DOM 预览 Code soft 1节点3.89:1；按钮夹具1280/390各19节点；布局夹具各1个Code solid gray节点3.31:1。参考选阶/旧token对按已确认目标口径接受；Caller utilities 1节点1.24:1沿用建议。axe incomplete 未全部人工复核，不声称全量 WCAG 合规 |
| 性能与人工辅助技术 | 未运行 | 本轮不重跑 V1–V10、真实性能、VoiceOver/OS IME、Safari/Firefox、真机/真人研究；这些不记通过 |

## 采集与覆盖

- 当前源码从 HEAD `941a8043903c244dc2b72e1e90c38b49e193c20c` 重新构建；两项构建成功日志为 [build-theme-studio-final.log](DOM-COMPONENTS-P1.ui-captures/r5/build-theme-studio-final.log)、[build-component-catalog.log](DOM-COMPONENTS-P1.ui-captures/r5/build-component-catalog.log)。初次构建因本地原生缓存中混有 Rust 1.99.1 的 rlib 而 E0514；原目录保留，固定 mise 工具重建原生辅助产物后成功。未安装服务/镜像、未升级依赖。
- 工具实值：Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13、Node 26.12.1、Playwright 1.63.0、Chromium 153.0.8010.12；headless、DPR1，B2 特写 DPR2；reducedMotion reduce、locale en-US。字体/网络/帧稳定等待和故障干预逐图见采集清单。
- **145 张独立评审截图**：U1 25、U2 13（11 + 补采2）、U3 18、U4 17、U5 18、U6 28、U7 26。另有 **11 张交卷后的机器实拍**，登记在 [machine-captures.json](DOM-COMPONENTS-P1.ui-captures/r5/machine-captures.json)；这些没有交给评审或混入其分数。
- 全部目标矩阵单元与既定采集步骤已覆盖，没有拿旧图、设计图、模拟页冒充当前截图。主 agent 用 QA 缩略检视全部当前图片，并逐张打开关键原图；独立评审按原图评分，原始 PNG 未修改。检查用 contact sheet/裁切只留在临时目录，不作为产品截图。
- U2 首采看不出影响清单时已补采并另派评审；本轮未拿不到的必需截图格子。其余未拍的行为/组合按各原文“看不出的格子”列为未知，不将这轮矩阵外推为完整 UAT 或全组件组合通过。原文中的实际开图清单可复核。
- 目标与附件 hash：[target-files.txt](DOM-COMPONENTS-P1.ui-captures/r5/target-files.txt)、[target-hashes.json](DOM-COMPONENTS-P1.ui-captures/r5/target-hashes.json)；原图尺寸/sha256：[image-integrity.json](DOM-COMPONENTS-P1.ui-captures/r5/image-integrity.json)。沿用人工循环的 `manual-sha256-normative-sections-v1`，规范指纹与 R4 相同；只追加采集步骤，未改期望或目标图。
- 评审方式：七单元分别使用全新 `fork_turns=none` 上下文，U2 补采增加一位，共八份有效结论；U7 一次容量错误后按原模型重试成功。模型继承宿主默认，具体型号/effort 工具未回显。派发、白名单与 hash 见 [judge-dispatch.json](DOM-COMPONENTS-P1.ui-captures/r5/judge-dispatch.json)，开图由各原文记录；白名单是信息边界，不冒充文件权限沙箱。
- 没有改分、删差距或因争议换评审。主 agent 的新 P1 只依据真实按键、稳定后的截图和页面实测；UI-115 的核对说明不改其分数、严重度或待裁定状态。
- 无产品/计划变更、无提交。为本轮临时启动的两个服务在最终回读前停止，记录于 [services.txt](DOM-COMPONENTS-P1.ui-captures/r5/services.txt)。

## 下一步

先修 UI-215；其余可修复差距可同批处理。UI-115 等用户给出方向，其余修复无需等待它。

```text
$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r5.md
```

修复完成后，再用 `$review-ui docs/plan/DOM-COMPONENTS-P1.md` 发起新一轮真实界面复审。

## 状态登记

本轮独立评审 ID：`2cc80246-5d61-4370-bbd7-462827ecbce3`，见 [SDLC 评审状态](../../.xgent-ai/sdlc/status/review/2cc80246-5d61-4370-bbd7-462827ecbce3.md)。按仓内 v3 协议登记为 `review_type: other`、`review-recorded` / `review-pending`；原判定与 UI 范围照实保留，不登记为合并、发布或产品终验。
