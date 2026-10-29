# U3、U4 UI 验收评审 · R1（首轮）

评审只依据目标包 `DOM-COMPONENTS-P1.ui-target.md`、`capture.md`、U3/U4 的 27 张截图和两份 actions.log。U3、U4 是规范模式，没有目标图。颜色和间距是在原图上取样或用 `magick` 裁剪放大后量的。

## 1. 评分表

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 /10 |
| --- | --- | --- | --- | --- | --- |
| U3 导入与导出 | 2.4 | 2.3 | 1.9 | 0.6 | **7.2** |
| U4 本地库迁移与存储反馈 | 2.0 | 2.0 | 1.6 | 0.5 | **6.1** |

**U3 各维度理由**
- 布局 2.4：对话框几何符合 X6。实测宽 x208–1231 约 64rem，高为视口减 32px，内边距 24px，内部纵向滚动，390 下表单行改单列。不到 3 分是因为导入审阅的「版本 / 名称 / 配置」只是三段没有标签的正文，390 下差异表的 token 列被挤到字符级断行。
- 视觉 2.3：中性壳、蓝色主按钮 #1570ef、代码区等宽、遮罩约 30% 都对。不到 3 分是因为多处偏离 G3 token。对话框标签是 14px/400（规范 13px/600）。次级文字是 #667085（规范 #475467）。错误色是 #d92d20（规范 #b42318）。差异表里的颜色值不是等宽 12px。
- 交互 1.9：导入审阅、Apply、四类导出、两种 CSS profile、四种记法、复制反馈都能走通。不到 2 分有几个原因：
  - 未知版本的错误给了错误诊断（P1）。
  - 差异表不区分新增和删除。
  - v1 损失清单缺 appearance 和 scaling 意图。
  - Rust 说明没写具体 SDK/CLI 版本。
- 细节 0.6：有若干小差异。配置行是原始键名加 `null`，表单行距 12/8 不一，复制反馈把按钮顶下去，字节数没有分组，表头和正文同样式，损失清单里取值的写法生硬。

**U4 各维度理由**
- 布局 2.0：每个状态都放在编辑列同一位置，有常驻卡片。不到 2.5 分是因为升级 Review 被塞进 280px 编辑列，内容是原始 JSON 文本框；冲突面板把决策按钮放在冲突版本信息之前，原文是压缩 JSON 加字符级断行。这两处都是结构性问题。
- 视觉 2.0：中性浅色壳保持住了。不到 2.5 分是因为几种状态长得一样：已保存、仅内存、冲突暂停、升级待确认，全都用同一行 12px #667085 灰色小字（只有配额失败是红字，而且是 #d92d20）。同一组决策按钮还混用了描边、裸链接和染色描边三种样式。
- 交互 1.6：七个状态都能到达，S2、S3、S5、S6 要求的出口基本都在。不到 2 分是因为多处只做到一部分：
  - Review 没有可读地列出命名主题、草稿和偏好。
  - 冲突不显示两边差在哪里。
  - 升级成功后没有对应的提示。
  - 取消后仍显示「Cancel upgrade」，提示里还提到界面上不存在的「resume saving」。
  - 配额失败的提示没说可以导出，也没点出是哪个命名动作失败。
- 细节 0.5：差异较多。状态文案大小写和句号错乱，操作列表计数和条目对不上、有重复行，有小写占位「choose」，按钮文案用斜杠拼了两个动词，还有「old bytes」这种行话。

## 2. 上一轮差距复核表

首轮，省略。

## 3. 差距清单

按严重度排序，同级按可挽回分排序。U3 合计 2.8（10 − 7.2），U4 合计 3.9（10 − 6.1）。

### P1

**U4-01 · 升级 Review 塞在编辑列里，内容是原始 JSON**
- 单元与位置：U4，`U4-upgrade-review-1440-light.png`、`U4-upgrade-review-end-1440-light.png`，左栏「Review the complete migration」卡片。
- 维度：布局。严重度 P1。可挽回分 0.55。
- 现状：点「Review / recheck upgrade」后，Review 在 280px 编辑列里展开成第二张灰卡。卡里有「Saved themes retained: 2」、一段说明，以及一个约 230px 高、可拖拽调整大小的文本框，里面是格式化后的 JSON。sha256 和 backup_key 的十六进制串被拆成三四行。要拉过整段 Review 才能看到「Upgrade and save current draft」；这时「Cancel upgrade」在上方卡片里，已经滚出视野。
- 期望：S3 要「展示原 v1 库的完整迁移结果及将保存的当前草稿/偏好」，并在同一处提供「下载升级前原始数据」。G2 要求壳层用分组行，内容可读。
- 修法：
  - 把 Review 从编辑列移到对话框，复用导入对话框的 X6 外壳：宽 min(100vw−2rem, 64rem)，高 100dvh−2rem，内部纵向滚动，内边距 1.5rem。
  - 正文分节：
    - 「Library v1 → v2」：两列 dl，`grid-template-columns:max-content 1fr`。
    - 「Saved themes (2)」表格：名称、id、迁移结果。
    - 「Current draft」：名称、Modified 状态。
    - 「Preferences」。
    - 「Exact JSON」：折叠在 `<details>` 里，等宽 12px，`white-space:pre; overflow:auto`。
  - 哈希显示为前 12 位加「复制」按钮。
  - 底部固定动作行：主按钮「Upgrade and save current draft」、描边按钮「Download original before upgrade」、描边按钮「Cancel upgrade」。

**U4-02 · 状态反馈没有语义区分，仅内存状态和已保存看起来一样**
- 单元与位置：U4，`U4-upgraded-1440`（「Saved on this device」）、`U4-no-locks-initial-1440`（「Secure Web Locks unavailable; memory editing and downloads only」）、`U4-conflict-1440`（「Saving paused. …」）、`U4-upgrade-pending-1440`（「Legacy draft restored in memory. …」），左栏 Local themes 下方那一行。
- 维度：视觉。严重度 P1，存在把仅内存误认为已保存、进而丢数据的风险。可挽回分 0.5。
- 现状：成功、降级（仅内存）、冲突暂停、待确认四种状态用的是完全一样的 12px #667085 灰色说明文字。没有标题，没有图标，也没有状态标签，扫一眼分不出「已保存」和「仅内存」。
- 期望：S1/S6 要求常驻反馈并「明确『仅内存编辑/可下载』」。G3 规定次级文字是 #475467、字段错误是 #b42318。C3 的原则是状态用文字加标识，不只靠颜色。
- 修法：给存储状态行做一个带语义变体的组件。
  - 成功：#475467 文字，前置 ✓ 图标和「Saved」标签。
  - 仅内存/警告：常驻提示块，左边框 3px 警示色、浅底，13px/600 标题写「Memory only — not saved on this device」，下接正文和「Export…」按钮。
  - 错误：#b42318 文字，浅红底。
  - 每个变体都带文字标签。普通说明文字统一改成 #475467。

**U3-01 · 未知版本的导入错误给了错误诊断，没有显示版本**
- 单元与位置：U3，`U3-import-error-1440-light.png`，字节计数下方的红字。
- 维度：交互。严重度 P1。可挽回分 0.45。
- 现状：输入 `{"schema_version":999}`，显示「Import failed: json: missing field \`id\` at line 1 column 22」。用户会以为是缺了 id 字段，而真正的问题是版本不受支持。界面没有出现「999」。这是 serde 原文，带反引号，没法中英同步。
- 期望：X1「导入先呈现候选差异、版本和诊断」。目标包 U3 的比较口径要求「版本……与诊断的呈现须符合规范」。G6 要求新增错误文案中英同步。
- 修法：
  - 解析时先用只含 `schema_version: Option<u32>` 的探测结构读版本。不在 {1, 2} 内的，直接返回带类型的「不支持的版本」错误，携带实际版本号和支持的版本。
  - 界面文案（中英各一）：「Unsupported Theme JSON version 999. This Studio reads v1 and v2.」并在审阅区「JSON v1 → v2」同一位置显示「JSON v999 · unsupported」。
  - 其他 serde 错误映射成本地化的「Not valid Theme JSON (line 1, column 22)」，原文放进可展开的详情。

### P2

**U3-02 · 审阅摘要没有结构，版本、名称、配置是三段无标签正文**
- 单元与位置：U3，`U3-import-review-1440`、`U3-import-review-end-1440`、`U3-import-review-end-390`，「Review changes」按钮下方三段文字。
- 维度：布局。可挽回分 0.35。
- 现状：「JSON v1 → v2」「Imported legacy 旧主题 · 6 changes」「appearance=inherit · accent_color=null · …」三段都是 14px #1d2939 正文，段距一样，没有任何标签或标题，读的人认不出哪段是版本、哪段是配置。差异表上方也没有「Changes」标题。
- 期望：X1 要按「版本、名称、配置、两模式 base 与 override 新增/删除及诊断」逐项呈现。
- 修法：改成两列定义列表，`grid-template-columns:max-content 1fr; gap:4px 16px`，标签 13px/600 #475467。行依次为 Version（v1 → v2 · will be saved as v2）、Name、Configuration。表格前加 13px/600 小标题「Changes (6)」，「Import notes」与它同级。

**U4-03 · 冲突面板先给决策再给信息，原文不可读**
- 单元与位置：U4，`U4-conflict-1440`、`U4-conflict-detail-1440`、`U4-conflict-raw-1440`，左栏两张卡片。
- 维度：布局。可挽回分 0.35。
- 现状：第一张卡是三个决策动作，中间隔一条分隔线，第二张卡才是「External revision 3 · Rustify Light · Saved themes: 0」和说明。用户还没看到冲突版本就得做决定。「View exact stored text」展开后是压缩成一行的 JSON，塞在约 260px 宽的文本框里，断成「rustif / y-light」「draf / t」这样。
- 期望：S5 要「展示冲突版本」后显式决策 Load / Keep / Save copy。
- 修法：合成一个冲突面板，顺序是：标题「Another tab saved revision 3」→ 双方对比（见 U4-05）→ 动作行 → 折叠的原文。原文按 2 空格缩进格式化，等宽 12px，`white-space:pre; overflow-x:auto`，不在字符中间断行。窄列放不下时，把面板移进 X6 对话框。

**U3-03 · 差异表不区分新增、删除和修改**
- 单元与位置：U3，`U3-import-review-end-1440`，差异表。
- 维度：交互。可挽回分 0.3。
- 现状：表头只有「Mode / token | Current | Imported」。「light / overrides.primary」一行的 Current 写着 #1570ef，可左栏状态是「Matches preset」。读的人分不出这条 override 是新加的还是改的。
- 期望：X1「两模式 base 与 override 新增/删除」。
- 修法：
  - 加一列「Change」，填文字标签 Added / Removed / Changed，别只靠颜色。
  - 不存在的一侧显示「—」，并给屏幕阅读器写「none」。
  - 行按 Light、Dark 分组，组内再分 base 和 overrides，每组标计数。

**U4-04 · 升级 Review 没有可读地列出将要保存的草稿和偏好**
- 单元与位置：U4，`U4-upgrade-review-1440`、`U4-upgrade-review-end-1440`。
- 维度：交互。可挽回分 0.3。
- 现状：只有「Saved themes retained: 2」这个计数，和一句「This candidate contains every legacy saved theme plus the current draft and preferences」。两个命名主题叫什么、草稿是哪个、偏好有哪些值，只能去翻 JSON 文本框。
- 期望：S3「展示原 v1 库的完整迁移结果及将保存的当前草稿/偏好」。
- 修法：在 U4-01 的对话框里，用表格逐条列出命名主题（名称、id、是否有迁移备注）、当前草稿（名称、模式、是否修改过）、偏好（键加人话描述）。迁移备注沿用导入对话框「Import notes」的样式。

**U4-05 · 冲突不显示双方差在哪里**
- 单元与位置：U4，`U4-conflict-1440`、`U4-conflict-raw-1440`。
- 维度：交互。可挽回分 0.3。
- 现状：只写了外部版本的元数据（revision 3、名称、Saved themes: 0）。本页的修订号和内容都没给。本例中两边 primary 实际是 #123456 和 #654321，界面上看不到。
- 期望：S5「展示冲突版本，不静默覆盖或合并」，前提是让用户知道冲突是什么。
- 修法：复用导入差异表组件，列为「This tab (rev N) | Other tab (rev 3)」，列出 token 差异和命名主题数量的差异。

**U4-06 · 同一组决策按钮用了三种样式**
- 单元与位置：U4，`U4-conflict-1440` 第一张卡；`U4-upgrade-pending-1440` 卡内按钮。
- 维度：视觉。可挽回分 0.3。
- 现状：「Load other version」是透明底描边按钮，32px 高。「Keep current draft」是没有边框的链接。「Save current as a copy」是 #e1e9f6 染色描边，看起来像被选中了。三者纵向堆叠，间距不均。升级卡里也一样：「Review / recheck upgrade」是描边，「Cancel upgrade」是裸链接。
- 期望：S5 要求 Load / Keep / Save copy 是同等级的显式选择。G4 规定按下态只留给选中的模式或渲染器。
- 修法：三个决策统一成同尺寸描边按钮（32px，14px 字），用 `display:flex; flex-wrap:wrap; gap:8px` 排成一行，默认都不染色。升级卡的「Cancel upgrade」同样改成描边按钮。截图里看不出染色是不是焦点态（见第 5 节）；如果是焦点态，就改成 G4 规定的 3px 半透明蓝 outline，而不是染底色。

**U3-04 · 390 下差异表 token 列太挤，在字符中间断行**
- 单元与位置：U3，`U3-import-review-end-390-light.png`，差异表。
- 维度：布局。可挽回分 0.25。
- 现状：三列等宽，各约 100px。「overrides.primary」被断成「overrides.prima / ry」，「identity / name」也折了行，而 Current 和 Imported 列都还有空。
- 期望：G2「≤560px 导入导出表单行单列」。F4 允许长 token 名换行，但要读得出来。
- 修法：≤560px 时每条差异改成两行卡片式排列。第一行是 token，等宽 12px，只在「/」和「.」后插 `<wbr>`。第二行是「#1570ef → #7c3aed」。或者把列宽改成 `minmax(0,1.4fr) 1fr 1fr`，token 单元格用 `overflow-wrap:normal`。

**U4-07 · 升级成功后只显示通用的「已保存」**
- 单元与位置：U4，`U4-upgraded-1440-light.png`。
- 维度：交互。可挽回分 0.25。
- 现状：确认后升级卡直接消失，状态行写「Saved on this device」，和平时自动保存一模一样。用户确认不了升级完成了。
- 期望：S3「提交成功前不显示升级/保存成功」，也就是提交成功后应当显示升级成功。
- 修法：用成功变体显示「Library upgraded to v2 · 2 saved themes kept · current draft saved」，保留到下一次存储事件。旁边保留「Download original before upgrade」。

**U4-08 · 取消升级后的状态不对，文案也有问题**
- 单元与位置：U4，`U4-upgrade-cancelled-1440-light.png`，状态行和升级卡。
- 维度：交互。可挽回分 0.25。
- 现状：
  - 已经取消了，「Cancel upgrade」还在。
  - 命名保存被拦下后，状态行写「Review the storage warning before this library action. storage: persistent writes are protected; review the migration or explicitly resume saving」，要求用户去「resume saving」，可界面上没有这个动作。
  - 也没提到被拦的主题名「Memory only」。
- 期望：S2「取消后常驻『仅内存编辑』与重新检查入口」。S4「命名库动作单独反馈」。
- 修法：
  - 取消后升级卡改成：标题「Memory only — upgrade cancelled」，按钮「Review upgrade again」「Download original」，隐藏「Cancel upgrade」。
  - 被拦的命名保存单独给一行：「“Memory only” was not saved: the library upgrade is waiting for review.」，旁边放「Review upgrade」按钮。

**U3-05 · 对话框表单标签字号字重不对**
- 单元与位置：U3，所有导入、导出帧，标签「Input format」「Theme source」「Output」「CSS profile」「Color notation」「Exported code」。
- 维度：视觉。可挽回分 0.2。
- 现状：14px、400 字重、#1d2939，和正文一样。
- 期望：G3 规定 label 为 13px/600。
- 修法：对话框里的 `<label>` 套用壳层 label 样式（13px、600 字重），左列标签和控件文字基线对齐。

**U3-06 · 导入错误的颜色和字段边框不对**
- 单元与位置：U3，`U3-import-error-1440-light.png`。
- 维度：视觉。可挽回分 0.2。
- 现状：错误文字取样是 #d92d20。文本框边框仍是中性的 #858f9e，没有错误态。
- 期望：G3 字段错误为 #b42318。F4 的原则是非法输入用壳层错误边框。
- 修法：错误色 token 改成 #b42318。文本框加 `aria-invalid="true"`，边框 1px #b42318。用 `aria-describedby` 关联错误文字。

**U4-09 · 配额失败的提示出现两遍，红色也不对**
- 单元与位置：U4，`U4-quota-failure-1440`、`U4-quota-operations-1440`，左栏状态行和红字。
- 维度：视觉。可挽回分 0.2。
- 现状：灰字「Local save failed; memory and pending actions are preserved. Injected full device」和红字「Local save failed; draft and pending actions remain in memory: Injected full device」几乎是同一句话，上下各一遍。红色是 #d92d20。
- 期望：G3 字段错误为 #b42318。S1 要求一条常驻反馈加继续动作。
- 修法：只保留一条错误变体提示（见 U4-02），颜色 #b42318，下接动作按钮。

**U4-10 · 配额失败没说可以导出，也没点名失败的动作**
- 单元与位置：U4，`U4-quota-failure-1440-light.png`。
- 维度：交互。可挽回分 0.2。
- 现状：提示末尾直接拼了异常原文「Injected full device」。没说可以导出或下载。命名动作「Retained name」只出现在折叠的操作列表里。
- 期望：S6「配额失败时明确『仅内存编辑/可下载』……仍可导出」。S4「命名库动作单独反馈」。
- 修法：遇到 QuotaExceededError 时显示本地化文案「Couldn't save “Retained name”: browser storage is full. Your draft stays in memory — Export or download to keep a copy.」，带「Retry」「Export…」两个按钮。异常原文放进详情。

**U3-07 · 次级文字用错了色值**
- 单元与位置：U3，字节计数、CSS 说明「CSS contains both resolved modes in sRGB…」、Rust 说明、「Copied to clipboard.」、「Saved on this device」。
- 维度：视觉。可挽回分 0.15。
- 现状：#667085。
- 期望：G3 次级文字为 #475467。
- 修法：说明文字的 token 改成 #475467。X3、X4 要求出现的关键说明就写在这些文字里，加深后也更容易读到。

**U3-08 · 差异表里的颜色值不是等宽字**
- 单元与位置：U3，`U3-import-review-end-1440`、`-390`，Current 和 Imported 列的 #1570ef、#7c3aed 等。
- 维度：视觉。可挽回分 0.15。
- 现状：14px 比例字体。
- 期望：G3「颜色值等宽 12px」。
- 修法：颜色值用 `<code>` 包起来，等宽 12px。

**U3-09 · v1 损失清单少了 appearance 和 scaling 意图**
- 单元与位置：U3，`U3-export-v1-1440-light.png`，损失清单。
- 维度：交互。可挽回分 0.15。
- 现状：四条分别覆盖色族、auto、覆盖来源与字体意图、49 键与圆角/尺度角色。没有 appearance 意图和 scaling 意图。
- 期望：X2 列出的六类损失里都包括「appearance 意图」和「scaling 意图」。
- 修法：六类固定都列。当前主题某类没有损失时，也要明写，例如「Appearance intent (inherit) is not stored in v1」「Scaling 100% is baked into resolved values」。

**U3-10 · 配置行是原始键名和 null**
- 单元与位置：U3，导入审阅，「appearance=inherit · accent_color=null · gray_color=null · panel_background=null · radius=null · scaling=100」。
- 维度：细节。可挽回分 0.15。
- 现状：snake_case 内部键名，`null` 不说明含义，没法翻译。
- 期望：C2 和 G6 要求用和基础配置一致的标签（外观、品牌色、中性色、面板、圆角、缩放），配置文案中英同步。
- 修法：映射成「Appearance: Inherit · Brand color: none (legacy colors) · Neutral: auto · Panel: — · Radius: — · Scaling: 100%」。

**U4-11 · 状态文案大小写和标点错乱**
- 单元与位置：U4，`U4-upgrade-cancelled`「…library action. storage: persistent writes…」（句号后接小写「storage:」，句末无句号）；`U4-quota-failure`「…Injected full device」（无句号）；`U4-no-locks-initial`「…memory editing and downloads only」（无句号）。
- 维度：细节。可挽回分 0.15。
- 修法：状态文案统一用完整句子并以句号结尾，不把内部前缀「storage:」拼进用户文案。中英两份一起改。

**U3-11 · Rust 接入说明没写具体的 SDK/CLI 版本**
- 单元与位置：U3，`U3-export-rust-1440-light.png`，说明文字。
- 维度：交互。可挽回分 0.1。
- 现状：只写了「Use the matching source SDK and CSS; the published 0.2.1 SDK/CLI reject v2」。0.2.1 不能用写了，但所需的版本或修订号没给。
- 期望：X4「接入说明标明所需 SDK/CLI 版本」。
- 修法：加一行「Requires rustify-ui ≥ x.y (rev abcdef1) and CLI ≥ x.y」，版本号由构建时注入。

**U3-12 · 导出对话框底部的存储状态没有前缀，容易误读**
- 单元与位置：U3，所有导出帧，底部「Saved on this device」。
- 维度：交互。可挽回分 0.1。
- 现状：这行在 Copy/Download 下面，没有任何前缀。点完 Download 再看到它，容易理解成「导出的文件已经存到本机」。
- 期望：X5 规定导出反馈只在复制或下载实际成功后才报成功，不应和库保存状态混在一起。
- 修法：加前缀「Local library: Saved on this device」，和导出反馈分开放；或者干脆从导出对话框里拿掉这行。

### P3

**U4-12 · 操作列表的计数和条目对不上**
- 单元与位置：U4，`U4-quota-operations-1440`。
- 维度：细节。可挽回分 0.15。
- 现状：
  - 标题是「Pending / recent operations: 1」，下面列了三条。
  - 两条「Draft · confirmed」完全一样，没法区分。
  - 「Save Retained name · failed; retained」里主题名没加引号，失败条目和成功条目同样式。
- 期望：S4 要区分排队中、已确认、失败。
- 修法：
  - 标题改成「Pending 1 · Recent 2」。
  - 每条带修订号或时间，例如「Draft saved · rev 4 · 05:54」。
  - 失败条写成「Save “Retained name” — failed, kept for retry」，加文字标签和 #b42318。

**U4-13 · 「Host control」浮层挡住编辑控件**
- 单元与位置：U4，`U4-upgrade-pending-390`（压在「Favorites」按钮下边框上）、`U4-upgrade-cancelled-1440` 等（压在左下角选择框上）。
- 维度：布局。可挽回分 0.1。和 U1 共用一个壳层问题，若 U1 评审已列入，合并处理。
- 修法：给编辑列加 `padding-bottom`，值为浮层高度加 16px；或者把宿主控件改到不重叠的位置。

**U4-14 · 没有待处理项时「Retry retained saves」仍可点**
- 单元与位置：U4，pending、cancelled、upgraded、conflict、no-locks 各帧，计数都是 0。
- 维度：交互。可挽回分 0.1。
- 修法：保留项为 0 或没有 Web Locks 时禁用该按钮，并给出原因。

**U3-13 · 导出表单行距不一致**
- 单元与位置：U3，`U3-export-css-1440`。实测 Output 到 CSS profile 间距 12px，CSS profile 到 Color notation 间距 8px。
- 维度：细节。可挽回分 0.05。
- 修法：表单容器用 `display:grid; row-gap:12px`。

**U3-14 · 复制反馈把按钮行顶下去**
- 单元与位置：U3，`U3-export-rust` 对比 `U3-export-copied`。「Copied to clipboard.」插在代码框和按钮之间，按钮行下移约 16px，整个对话框也跟着重新居中。
- 维度：细节。可挽回分 0.05。
- 修法：预留固定高度的反馈行，或把反馈放在按钮右侧，加 `aria-live="polite"`。

**U3-15 · 字节数格式和单位不统一**
- 单元与位置：U3，导入帧。说明写「up to 256 KiB」，计数却是「3332 / 262144 bytes」，单位不一，也没有千分位。
- 维度：细节。可挽回分 0.05。
- 修法：改成「3.3 KiB / 256 KiB」，数字按界面语言格式化。

**U3-16 · 差异表表头和正文同样式**
- 单元与位置：U3，差异表表头「Mode / token / Current / Imported」。
- 维度：细节。可挽回分 0.05。
- 现状：#1d2939、400 字重，和正文一样。
- 修法：表头用 13px/600 #475467（G3 的 label 样式）。

**U3-17 · 损失清单里的取值写法生硬**
- 单元与位置：U3，v1 损失清单「Brand palette selection pink…」「Neutral selection auto (resolved mauve)…」。
- 维度：细节。可挽回分 0.05。
- 修法：用 C2 的字段名，取值加引号，例如「Brand color “pink” …」「Neutral color “auto” (resolves to mauve) …」。

**U4-15 · 预设选择框占位符是小写「choose」**
- 单元与位置：U4，`U4-upgrade-review-end-1440`、`U4-upgraded-1440`，预设选择框。
- 维度：细节。可挽回分 0.05。
- 修法：改成「Choose a preset」，中英同步。

**U4-16 · 按钮文案用斜杠拼了两个动词**
- 单元与位置：U4，升级卡「Review / recheck upgrade」。
- 维度：细节。可挽回分 0.05。
- 修法：首次显示「Review upgrade」，看过一次后改为「Recheck」。

**U4-17 · 冲突提示说有保留动作，实际计数是 0**
- 单元与位置：U4，`U4-conflict-1440`。状态行「Pending actions are retained; … retry/cancel them.」，下方计数是「Pending / recent operations: 0」。
- 维度：细节。可挽回分 0.05。
- 修法：计数为 0 时不出现后半句。

**U4-18 · 升级卡里有实现层面的行话**
- 单元与位置：U4，升级卡「No old bytes, backup or v2 library are written…」。
- 维度：细节。可挽回分 0.05。
- 修法：改成「Nothing is written to this device until you confirm the upgrade.」。

**合计核对**
- U3：布局 0.6 + 视觉 0.7 + 交互 1.1 + 细节 0.4 = 2.8，对应 7.2 分。
- U4：布局 1.0 + 视觉 1.0 + 交互 1.4 + 细节 0.5 = 3.9，对应 6.1 分。

## 4. 建议（不扣分）

- 导入和升级 Review 对话框的标题和动作行可以做成 sticky。现在导入审阅在 1440 下要滚到底才看到「Apply import / Cancel」，标题也跟着滚走了。
- 导出 Theme JSON 时默认格式化输出，至少给个切换。现在是一整行压缩 JSON 自动折行，在连字符处断开（「accent- / foreground」「chart- / 1」），像是两个键。代码区可以考虑 `white-space:pre` 加横向滚动。
- CSS profile 和颜色记法的下拉列表宽约 158px，远窄于触发框（约 865px），可以和触发框等宽对齐。
- Rust 视图里有两个下载按钮，「Download」可以改成「Download theme.rs」，和「Download matching JSON」区分开。
- 差异表的颜色值前可以加 12×12 色块，便于直观比对。
- v1 损失说明块可以用警示样式，让「lossy」不只是下拉框里的一个词。
- 导入失败后可以补一句「Current theme unchanged」，让 X1 规定的「保留当前主题」也看得见。
- 跨单元观察（依据 C3，属 U1 范围，请 U1 评审核对）：恢复旧草稿后，顶部状态是「Matches preset」，而预设选择框显示的是「choose / 旧草稿」。这可能就是 C3 禁止的「伪装成未修改预设」。

## 5. 看不出的格子

- `U3-import-review-end-1440`、`U3-import-review-end-390`：「Apply import」紧贴对话框底边，按钮底部被裁平，下方没有 24px 留白。截图判断不了是对话框真的缺底部内边距，还是采集时滚动只把按钮滚到可视区边缘，没滚到最底。这条没有计入扣分，需要补一张真正滚到底的帧。
- `U3-export-rust-1440`、`U3-export-rust-end-1440`：代码中段没有拍到。注释说「Copy the listed same-origin font files」，但两帧里都看不到这份字体清单，X4 要求的「外部字体资源」在 Rust 视图里是否完整列出无法判断。
- `U3-import-error-1440`：「Review changes」有浅蓝底和 #81b2f7 边框，分不出是悬停、焦点还是按下态，也没有 3px 焦点环。U3、U4 都没有键盘帧，对话框和本地库动作的焦点可见性（G4）无法评价。
- `U4-conflict-1440`：「Save current as a copy」的 #e1e9f6 染色看不出是默认样式、焦点态还是推荐态。U4-06 只按「同组样式不一」扣分。
- 以下状态都没拍到，不算通过也不算失败：
  - X5 复制失败、下载失败，以及下载实际发起后的反馈。
  - X1 的取消、语法错误、超限、不支持的颜色输入。
  - S1 的缓存损坏、只读持久化、导入失败、复制失败。
  - S2 改模式或偏好能否绕过确认。
  - S4 的排队态、「先前快照成功而当前仍待保存」、待锁时编辑和导出不被禁用。
  - U3、U4 的中文界面（G6）。
  - U4 除「待确认」以外各状态在 390 下的样子。
  - 导出代码框是否只读。
- `U4-no-locks-1440`：「Save locally」在无锁时仍可点击，点击后报了真实失败。S6 允许「禁用或报真实失败」，所以不扣分，但看不出编辑区其他本地写入入口（命名库的重命名、删除）在无锁时的表现。

## 6. 总评

U4 分最低（6.1）。U4 的问题主要不在缺功能：七个状态都能到达，S2、S3、S5、S6 要求的出口基本都在。问题在呈现方式。

需要结构性改动的地方有四处：
1. **U4 升级 Review 要整体重做**（U4-01、U4-04）：从 280px 编辑列移进 X6 对话框，原始 JSON 换成结构化的迁移摘要（命名主题、当前草稿、偏好），JSON 降级为折叠项，动作行固定在底部。
2. **新建带语义变体的存储状态组件**（U4-02、U4-09、U4-07）：成功、仅内存、冲突、错误各有文字标签和样式，不能再共用一行 12px 灰字。
3. **冲突面板重排**（U4-03、U4-05、U4-06）：信息在前，然后是双方差异表，最后是三个同级按钮。差异表可以和 U3 导入审阅共用一个组件，它本身也要补上新增/删除标记（U3-03）。
4. **U3 导入流程**：先探测版本再完整解析，未知版本给出明确的版本诊断（U3-01）；审阅摘要改成带标签的定义列表（U3-02）。

U3 的对话框外壳已经符合 X6。剩下的主要是 G3 token 收口：标签 13px/600，次级文字 #475467，错误 #b42318，颜色值等宽 12px。另外补齐 X2 损失清单的两类和 X4 的版本号。

开图清单：U3-import-review-1440-light.png、U3-import-review-end-1440-light.png、U3-import-error-1440-light.png、U3-import-review-390-light.png、U3-import-review-end-390-light.png、U3-export-json-1440-light.png、U3-export-v1-1440-light.png、U3-export-css-1440-light.png、U3-export-css-profile-open-1440-light.png、U3-export-css-notation-open-1440-light.png、U3-export-rust-1440-light.png、U3-export-rust-end-1440-light.png、U3-export-copied-1440-light.png、U3-export-json-390-light.png、U4-upgrade-pending-1440-light.png、U4-upgrade-review-1440-light.png、U4-upgrade-review-end-1440-light.png、U4-upgrade-cancelled-1440-light.png、U4-upgraded-1440-light.png、U4-upgrade-pending-390-light.png、U4-quota-failure-1440-light.png、U4-quota-operations-1440-light.png、U4-conflict-1440-light.png、U4-conflict-detail-1440-light.png、U4-conflict-raw-1440-light.png、U4-no-locks-initial-1440-light.png、U4-no-locks-1440-light.png（共 27 张；U3/U4 为规范模式，没有目标图，未打开任何目标图；辅助裁剪图在 /private/tmp/claude-501/-Users-rockie-Documents-gh-rockie-rustify-ui/0bc04ccb-5f93-4892-a089-e1cb25f9419c/scratchpad/judge-B2/）
