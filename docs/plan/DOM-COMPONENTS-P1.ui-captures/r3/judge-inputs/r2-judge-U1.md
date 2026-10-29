# R2 评审结论 · U1 摘录

摘自 R2 合并结论 judge.md，只含 U1 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U1 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.3 | 2.4 | 2.6 | 0.5 | **7.8** | b | a 8.1 |


### 各维度理由

**U1（b）**

- 布局 2.3：双栏、320px 编辑列、独立滚动、768 堆叠、≤800 padding、390 切换与「Back to edit」、Host control 回到文档流都对。扣分原因：粘性头在 200% 下封顶于 y=449、裁掉「Preview mode」并藏起模式与撤销/保存/导出；390 下滚动内容从标签条与粘性头之间透出；粘性头占视口 31–38%。
- 视觉 2.4：分隔色、搜索框边框、选中三色正确，壳层扁平，深色预览时编辑壳保持中性浅色。扣分原因：英文 label、预览标题、选中字段标签页看不出 600；辅助文字为 #667085；禁用按钮边框比可用按钮更重；未选分段带浅蓝内框；状态胶囊用选中蓝。
- 交互 2.6：键盘依次到达预设搜索、品牌色选择器、展开列表、Dark 与 Colors 标签页，焦点清楚；选中与未选可分；Overridden/Custom 有文字加标识；中文完整。扣分原因：200% 下关键动作藏在封顶头区里；390 预览页不能切模式、没有保存/导出；焦点环压到相邻分段；顶部「Modified」有歧义。
- 细节 0.5：复数文案、mauve 小写未翻译、头区垂直节奏、两栏标题基线、行尾悬挂分隔点、两种焦点环色、768 下主题名比主标题大。


## R2 未关闭差距（U1）

#### UI-101 · [P1] 200% 文本下粘性头被封顶为内滚动区，「Preview mode」被切半，模式、撤销、保存、导出不可见（U1 · 布局、交互 · 可挽回 a 0.55 / b 0.5）

- 位置：U1-zoom200-720-light 编辑列 y=0–449；U1-zoom200-export-720-light 遮罩后方头区已内部滚动。
- 现状 → 期望：头区限高约 50vh，关键动作只能靠键盘焦点带动内滚动露出 → G5 顶部保留主题名、模式、撤销/重做/保存/导出；G6 200% 下主路径可达，不靠隐藏的嵌套滚动。
- 修法：去掉头区 `max-height`/`overflow`；只让一条紧凑命令条吸顶（主题名 + 状态、Preview mode、Undo、Redo、Save、Export，其余收进 More），产品名、副标题、语言切换放回文档流；高度超过面板约 40% 时降为非吸顶。
- 来源：a 新-2；b 新-1（同一现象、同一帧、同一坐标 y=449）。

#### UI-102 · [P2] 390 编辑页的 Edit/Preview 标签条与粘性头之间有空隙，滚动内容从中透出（U1/U2 · 布局 · 可挽回 U1 a 0.2 / b 0.15，U2 a 0.25）

- 位置：U1-configured-390-light y≈45–85（「Presets … 44」半截）；U2-source-overridden-390-light y≈45–85（「sidebar-border #d0d0d0」被上下两层切开）。
- 现状 → 期望：两层吸顶之间漏出内容 → G1、G2（≤600px 编辑/预览分区稳定，不遮挡、不截断）。
- 修法：标签条与头区放进同一个不透明粘性容器，或头区 `top` 设为标签条高度并把间距改为粘性元素自身的 `padding-top`；或 ≤600px 时头区不吸顶。
- 来源：U1 a 新-1、b 新-2；U2 a 新-1（同一机制：两层吸顶之间的空隙，同在 390、同 y 范围）。

#### UI-103 · [P2] 粘性头过高，占视口 31–38%，挤占编辑区（U1 · 布局 · 可挽回 b 0.1）

- 位置：1440/768 各帧 y=0–314；390 标签条加头区 y=0–318。
- 现状 → 期望：头区 8 行 → G2 编辑区独立滚动；G5 只要求保留主题名、模式与撤销/重做/保存/导出。
- 修法：同 UI-101 的单行命令条方案，1440 下吸顶区控制在约 96px 内。
- 来源：b 新-3（a 在建议 1 提出同一点，未计分）。U2 a 在 UI-17 中也指出常驻区压缩可用高度。

#### UI-02 · [P2] 未选分段带 #8AB7F7 蓝内框、段间 3px 双线，焦点环压到相邻段（U1/U5 · 视觉、交互、细节 · 部分解决 · 可挽回 U1 a 0.2 / b 0.15，U5 a 0.15 / b 0.15）

- 位置：U1-default-1440-light 的 Light/Dark、DOM/GPU/DOM + GPU、Inspect；U1-keyboard-04 Dark 焦点；U5-dom-1440-light、U5-compare-nested、U5-gpu-compare 工具栏。
- 现状 → 期望：未选段在 #D0D5DD 外框内多一圈 1px #8AB7F7，EN/中文、Edit/Preview 没有；选中段另加 2px 下划线 → G3 蓝色只用于选中与焦点；G4 选中的模式/渲染器按钮用按下态。
- 修法：所有分段控件共用一个组件：未选段白底 #475467，去掉 #8AB7F7；段间单条 1px #D0D5DD；焦点环内收或 `z-index:1`，不压邻段。
- 来源：U1 a UI-02、b UI-02；U5 a UI-02（P2）、b UI-02（P3）。严重度取 P2。

#### UI-104 · [P2] 壳层次级与禁用按钮有 4 套样式，闲置按钮用焦点同色 #8AB7F7 做边框，禁用按钮比可用按钮更重（U1 · 视觉 · 可挽回 a 0.15 / b 0.1）

- 位置：U1-default-1440-light、U1-configured-1440-light 编辑列：Undo/Restore（#8AB7F7 边 #475467 字）、Import/Export（#8AB7F7 边 #1570EF 字）、Cancel pending writes/Retry（#858F9E 边 #667085 字）、Focus preview（#D0D5DD 边）；禁用 Redo 为 #858F9E 边、#F2F4F7 底。
- 现状 → 期望：闲置控件像半选中，禁用比可用更醒目 → G3 壳层扁平、蓝色只给选中与焦点、分隔 #D0D5DD/#EAECF0、次级文字 #475467。
- 修法：统一 ShellButton 次级样式（白底、1px #D0D5DD、#344054 或 #475467 字）与一套禁用样式（#F9FAFB 底、#EAECF0 边、#98A2B3 字）；#8AB7F7 只留给 `:focus-visible`。
- 来源：a 新-5；b 新-4（同一按钮体系、同样引用 #858F9E 非 token 边框）。

#### UI-09 · [P3] 英文 label、预览标题、选中字段标签页没有呈现 600 字重（U1 · 视觉、细节 · 仍存在 · 可挽回 a 0.15 / b 0.2）

- 位置：U1-configured-1440-light 的 Appearance、Brand color、Neutral color、Preview mode、Presets；U1-default-1440-light 预览「Preview」；U1-field-tabs-1440-light「Colors」。中文 label「外观」「预览模式」明显是粗体。
- 现状 → 期望：字号已是 13px，字重仍为常规 → G3 label 13px/600、预览标题 16px/600；G4 选中标签页 13px 半粗。
- 修法：确认 label 类为 600，并确认 IBM Plex Sans 600 字面以独立 woff2 从 public 加载（`document.fonts` 中有 600 条目）。
- 来源：a UI-09（部分解决）、b UI-09（仍存在），取仍存在。

#### UI-105 · [P3] 同一行壳层控件高度有 28/36/38px 三档（U1 · 布局、视觉、细节 · 可挽回 a 0.15 / b 0.05）

- 位置：1440 预览工具栏 Cards/Responsive 36px、分段与 Inspect 28px、Focus preview 38px；头区 EN/中文 38px、Light/Dark 28px；768/390 第二行。
- 修法：定义 `--shell-control-h: 2rem`，工具栏与头区所有控件统一，`align-items:center`。
- 来源：a 新-4；b 新-6。

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

#### UI-108 · [P3] 粘性头下边框下方 17px 又有一条分隔线，形成双细线（U1/U4 · 细节 · 可挽回 U1 a 0.05，U4 b 0.05）

- 位置：U1-default-1440-light y=314 与 y=331；U1-default-768-light、U1-zh-top-1440-light、U1-zoom200-720-light 同理；U4 各 1440 帧 y=282/299 或 314/331。
- 修法：滚动区第一个分组去掉上边框，或头区只在内容滚动后显示下边框。
- 来源：U1 a 新-7；U4 b 新-5（同一对线）。U4 a 在建议中指出同一现象并归 U1。

#### UI-109 · [P3] 状态胶囊（Overridden、Custom、网格 Override）借用选中态蓝色（U1/U2 · 视觉 · 可挽回 U1 b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light、U1-configured-lower-1440-light「• Overridden」「• Custom」（#EFF8FF/#84CAFF/#175CD3）；U2-source-overridden-1440-light 网格「Override」胶囊（选中的 primary 格内只剩一道描边）。
- 现状 → 期望：状态与选中、动作同色 → G3 蓝色用于选中与焦点；C3 状态以文字 + 标识表达（已满足）。
- 修法：状态胶囊改中性（#F9FAFB 底、#D0D5DD 边、#475467 字，12px/600，保留前导圆点）。
- 来源：U1 b 新-10（a 在建议 2 提出）；U2 a 新-4（并注明 U1 胶囊同色，建议同步）。

#### UI-110 · [P3] 「1 text pairs below 4.5:1」单复数错误（U1/U5 · 细节 · 可挽回 U1 b 0.1，U5 a 0.05）

- 位置：U1-default-1440/768、U1-field-tabs、U1-keyboard-01 至 03、U1-zoom200；U5-dom-1440-light、U5-inspector*、U5-gpu-compare、U5-preview-390 诊断条标题。
- 修法：按数量选择单复数（`Intl.PluralRules`）。
- 来源：U1 b 新-8；U5 a 新-2。U4 a 在建议中指出同一处。

#### UI-111 · [P3] 768 下主题名（18px）比主标题（16px）大；1440 下两个 18px 标题并列（U1 · 细节 · 可挽回 b 0.1）

- 修法：每栏只保留一个 18px 标题（建议主题名为主标题，≤800 时 16px），或两者都遵守 ≤800 的 16px。
- 来源：b 新-9（a 在建议 5 提出两个 18px 标题竞争）。

#### UI-112 · [P3] 分隔点「·」悬挂在行尾，Overridden 胶囊后多一个点（U1 · 细节 · 可挽回 b 0.1）

- 位置：U1-zoom200-720-light 预览右上「Rustify Light ·」；U1-configured-1440-light「• Overridden · Light 6 · Dark 6」。
- 来源：b 新-13。

#### UI-113 · [P3] 壳层焦点环有多种颜色/透明度（U1 · 细节 · 可挽回 b 0.05）

- 位置：U1-keyboard-01 预设搜索 #8AB7F7；U1-keyboard-02 品牌色选择器 #A1C6F9；U1-keyboard-04 Dark #97C2F9/#A1C6F9。
- 修法：搜索框、选择器、分段控件共用一个焦点 token（3px、40%、2px offset）。
- 来源：b 新-11；a 在 UI-02 余项中列出同一现象。

#### UI-114 · [P3] 页头「Modified / Matches preset」的比较基准不清，与存储状态读法冲突；吸顶区没有保存状态（U1/U4 · 交互、细节 · 可挽回 U1 b 0.05，U4 a 0.1）

- 位置：U1-top-1440-dark「Graphite Modified」与下方「Saved on this device」；U4-upgrade-pending 页头「Matches preset」对话框「Draft state: Matches saved document」；U4-upgraded「Modified」紧挨「Saved on this device」。
- 修法：页头写明基准（「Modified from preset · N overrides」），吸顶区加按实际写入结果显示的保存状态胶囊；对话框用相同措辞。
- 来源：U1 b 新-12；U4 a 新-12（同一页头标签）。

#### UI-03 · [P3] 390 预览页顶部没有预览模式切换与保存/导出（U1 · 交互 · 部分解决 · 可挽回 b 0.1）

- 位置：U1-preview-390-light 顶部只有「Graphite · Light preview」文字。
- 修法：UI-101 的紧凑命令条放在 Edit/Preview 标签条下，两个标签页共用；或预览工具栏首行放 Light/Dark 与溢出菜单。
- 来源：b UI-03（部分解决）；a 判已解决。取部分解决。

#### UI-05 · [P3] 200% 下导出对话框动作按钮高度固定 32px，没有随字号放大（U1 · 布局 · 部分解决 · 可挽回 b 0.05）

- 位置：U1-zoom200-export-end-720-light 的 Copy code、Select text、Download（y 726–757）。
- 修法：`min-height: 2.25rem; padding: .375rem .75rem`，单位用 rem。
- 来源：b UI-05（部分解决，基于补采帧）；a 在补采前判已解决。取部分解决。

#### UI-12 · [P3] 自动配对的中性色名显示为小写原始 id 且不翻译，与品牌色显示规则不一致（U1/U6 · 细节 · 部分解决 · 可挽回 U1 a 0.1 / b 0.05，U6 a 0.15）

- 位置：U1-configured-1440-light「Resolved neutral: mauve.」；U1-zh-configured-1440-light「已解析中性色：mauve。」；U6-fixture-1280-light「Resolved neutral: slate.」；U6-panel-zh-1280-light「已解析中性色：slate。」（品牌色已显示「靛蓝色」/「Indigo」）。
- 修法：auto 结果与选项列表共用同一个本地化显示名函数。
- 来源：U1 a UI-12、b UI-12；U6 a 新-1（同一 ThemePanel 字段的同一显示规则问题）。


## R2 建议（U1，不扣分）

- U1：
  - 存储状态块压成工具栏状态胶囊，让基础配置早点出现在首屏（a、b）。
  - 品牌色、中性色选项加色块（b）。
  - 加跳转链接或 landmark，缩短到预览工具栏的 Tab 距离（b）。
  - 200% 导出对话框头部吸顶（b）。
  - 应用名降为 14px/600，让主题名成为唯一 18px 标题（a）。
  - 搜索框与相邻选择器边框统一（a）。
