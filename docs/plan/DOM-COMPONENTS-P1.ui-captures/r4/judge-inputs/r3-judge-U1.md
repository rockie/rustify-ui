# R3 评审结论 · U1 摘录

摘自 R3 合并结论 judge.md，只含 U1 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U1 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.6 | 2.8 | 2.7 | 0.6 | **8.7** | a | — | 7.8 |

### 各维度理由

**U1（a）**

- 布局 2.6：顶栏横跨两栏，各视口都有主题名、保存状态、预览模式和 Undo/Redo/Save/Import/Export；1440 与 768 的内边距、768 的 DOM/GPU 上下堆叠都对，390 两层固定区之间不再透出内容。扣分原因：200% 下渲染器分段控件右缘被视口截掉；导出对话框的 Output 值被挤成「Theme…」；顶栏在 768 多占一行、在 200% 下固定占视口 39%；状态胶囊与解除按钮之间 0px。
- 视觉 2.8：分隔、次级文字、搜索框边框、选中三色、错误色都对上 token；label 和选中标签页为 600；状态胶囊改为中性；焦点环统一为一个 token。扣分原因：200% 导出对话框仍是旧的蓝边蓝字次级按钮，选择器边框是 #858F9E；「Light N · Dark N」计数仍为 #667085；分段控件有两种接缝画法。
- 交互 2.7：键盘依次到达预设搜索、品牌色选择器、展开的列表、Dark、Colors 标签页，焦点清楚；保存胶囊按实际结果显示；页头写明比较基准；390 预览页能切模式、保存、导出；200% 下关键动作都可见。扣分原因：Dark 的焦点环压进 Light 段约 5px；200% 下看不出当前导出格式；200% 下渲染器控件被截。
- 细节 0.6：编辑列「1 text pairs」复数错误；中文「已保存到本机 / 已保存到此设备」两种译法；预览标题旁读成「Rustify Light Light preview」；顶栏内边距与两栏差 4px、主题名与说明不同基线；颜色搜索框 36px 与 Clear 32px 并排。

## R3 未关闭差距（U1）

#### UI-157 · [P2] 200% 下渲染器分段控件（DOM / GPU / DOM + GPU）宽于预览列，右缘被视口截掉（U1 · 布局、交互 · 可挽回 a 0.2）

- 位置：U1-zoom200-720-light 预览工具栏第三行（y≈655–716），控件从 x=352 一直到视口边缘 x=719，没有右边框，右圆角被切。R2 同帧完整显示，属回归。
- 现状 → 期望：被截断 → G6 200% 下主路径可达、不依靠横向滚动；评分口径要求无溢出、截断。
- 修法：分段控件 `max-width:100%`，每段 `flex:1 1 auto; min-width:0`、允许换行；或给预览工具栏设 `container-type:inline-size`，用 rem 写容器查询，窄时改三列等分网格，再窄改为带 label 的下拉。确认预览列网格项 `min-width:0`。
- 来源：U1 a 新-1。

#### UI-158 · [P2] 200% 下导出对话框的 Output 选择器被压窄，当前值截成「Theme…」（U1 · 布局、交互、细节 · 可挽回 a 0.2）

- 位置：U1-zoom200-export-720-light「Output」行，选择器约 213px 宽。R2 同帧显示完整「Theme JSON v2」，属回归。
- 现状 → 期望：看不出当前导出格式 → G6 对话框在可用视口内可操作；X6 窄时表单行单列；X2 须区分 v2 与兼容 v1。
- 修法：对话框表单行 `flex-wrap:wrap`，选择器 `flex:1 1 14rem; min-width:14rem`，不够时换到 label 下方占满；或按对话框宽度用容器查询切单列，不只依赖 ≤560px 视口断点。
- 来源：U1 a 新-2。

#### UI-02 · [P3] 分段控件残留：焦点环压到相邻段，两种接缝画法并存（U1 · 交互、视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U1-keyboard-04-1440-light Dark 焦点环左缘 x≈988–990 落进 Light 段；U1-default-1440-light EN/中文、U1-edit-390 / U1-preview-390 Edit/Preview 段间为 #84CAFF + #D0D5DD 两条线。
- 修法：EN/中文、Edit/Preview 改用 Light/Dark 的同一分段组件（相邻段 `margin-inline-start:-1px` 合并边框，选中段 `z-index:1`）；分段内 `:focus-visible` 用内收焦点环（`outline-offset:-3px`），或焦点段 `z-index:2` 并给组容器留 2px 内边距。
- 来源：U1 a UI-02（部分解决）；U5 a 判已解决，焦点部分待核实。

#### UI-103 · [P3] 顶栏在 768 多一行，在 200% 固定占视口 39%（U1 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-default-768 / U1-configured-768 顶栏 y=0–160 三行，第 2、3 行右半空置；U1-zoom200-720 顶栏 y=0–348 共 5 行且固定。
- 修法：Undo/Redo 与 Save/Import/Export 放进同一个可换行的动作行；200% 下把 Undo/Redo 放到 Preview mode 下方右列；顶栏超过视口 30% 时改为不吸顶，或把 Import/Export 收进 More。
- 来源：U1 a UI-103（部分解决；1440 已解决）。

#### UI-104 · [P3] 200% 导出对话框仍用旧的蓝边次级按钮和非 token 边框（U1 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-zoom200-export-end-720-light「Select text」「Download」为 #8AB7F7 边、#1570EF 字；U1-zoom200-export-720-light Output 选择器边框 #858F9E。
- 修法：对话框动作区复用壳层次级按钮（白底、1px #D0D5DD、#475467 字），选择器边框改 #D0D5DD；只有 Copy code 保留实心主按钮。与 UI-51 是同一套样式问题，可一起改。
- 来源：U1 a UI-104（部分解决）。

#### UI-106 · [P3] 状态行与解除按钮之间 0px；两栏标题中线差约 4.5px（U1/U2 · 布局、细节 · 仍存在 · 可挽回 U1 a 0.1、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light 与 U2-group-reset-1440-light，Brand、Neutral、Panel 各组的状态胶囊底边与「Use configuration values · both modes」顶边相接（如 y=488/489）；U1-default-1440-light 应用名与预览标题中线差约 4.5px。
- 修法：配置组容器 `display:grid; gap:.5rem`（或状态行下边距 8px）；预览标题行 `min-height:2rem; align-items:center`，与编辑列首行同高同中线。
- 来源：U1 a UI-106（部分解决）；U2 a、b UI-106（仍存在）。

#### UI-107 · [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一（U1/U2 · 视觉 · 部分解决 · 可挽回 U1 a 0.05、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light、U2-group-reset-1440-light 的「Light N · Dark N」计数与「No overrides」胶囊文字为 #667085；U1-field-tabs-1440-light「Search colors…」占位符 #667085，U1-default-1440-light「Search presets…」约 #8E949C。
- 修法：计数与胶囊文字改用 #475467，#667085 只留给禁用态和占位符；两个搜索框共用一个 `::placeholder` token。
- 来源：U1 a、U2 a、U2 b UI-107（均部分解决）。

#### UI-110 · [P3] 编辑列诊断摘要仍是「1 text pairs」（U1 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U1-keyboard-04-1440-light 编辑列底部 y≈783「Diagnostics · 1 text pairs below 4.5:1」；预览诊断条已改为单数。
- 修法：编辑列摘要与预览诊断条共用同一个复数函数。
- 来源：U1 a UI-110（部分解决）；U5 a 判已解决。

#### UI-159 · [P3] 中文「Saved on this device」有两种译法（U1 · 细节 · 可挽回 a 0.05）

- 位置：U1-zh-top-1440-light 顶栏胶囊「已保存到本机」，同帧编辑列状态块「已保存到此设备」。
- 修法：两处共用同一个文案 key，统一为「已保存到本机」。
- 来源：U1 a 新-3。

#### UI-161 · [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」（U1/U5 · 细节 · 可挽回 U1 a 0.05、U5 a 0.05）

- 位置：U1-default-1440-light 等帧及 U5-dom-1440-light、U5-gpu-compare 右上 x≈1266–1416、y≈88–98；U5-dom-1440-dark 为「Rustify Light　Dark preview」；U5-preview-390-light y≈229。R2 同位置是「Rustify Light · Light preview」。
- 现状 → 期望：主题名与预览模式连读 → C4 显示「预览模式」以免与作者外观混淆。
- 修法：恢复只在同一行时显示的「 · 」分隔（换行时去掉悬挂点），或把模式写成「Preview: Light」或中性胶囊；中文同步。
- 来源：U1 a 新-5、U5 a 新-3。判为同一问题的依据：同一位置、同一文字、同一原因（R2 的分隔点被去掉）。

#### UI-162 · [P3] 顶栏左右内边距与两栏不一致，主题名与说明按中线而非基线对齐（U1 · 布局、细节 · 可挽回 a 0.05）

- 位置：U1-default-1440-light「Export」右缘 x=1419、预览分隔线右端 x=1416；U1-default-768-light 顶栏左 20 / 右 747，编辑与预览为 16 / 751；1440 主题名基线 y≈33、「Matches…」基线 y≈31。
- 修法：顶栏 `padding-inline: 20px 24px`，≤800px 改 16px 16px；主题名组 `align-items:baseline`。
- 来源：U1 a 新-6。

#### UI-160 · [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高（U1/U2 · 布局、细节 · 可挽回 U1 a 0.03、U2 b 0.05）

- 位置：U1-field-tabs-1440-light「Search colors…」y=537/572，「Clear」y=539/570；U2-source-overridden-1440-light 搜索框 y=73–108、「Clear」y=75–106。
- 修法：输入框与按钮同高，统一用壳层控件高度。
- 来源：U1 a 新-4、U2 b 新-9。判为同一问题的依据：同一对控件、同一高度差。

### U2

## 建议（不扣分，U1）

  - 编辑列顶部存储状态块与顶栏保存胶囊重复，可压成一行摘要加展开，让预设搜索更早出现。
  - 1440 下两个 18px 标题字重不同，可把应用名降为 14px/600 或统一字重。
  - 保存状态胶囊加前导标识，区分未保存与已保存的形状。
  - 品牌色、中性色选项加色块。
  - 加跳到预览工具栏的链接或 landmark（品牌色选择器到 Dark 要 Tab ×114）。
  - 200% 导出对话框标题行与关闭按钮吸顶。
  - 帮助文字行距约 19.5px（13px × 1.5），确认归哪一档字级。
