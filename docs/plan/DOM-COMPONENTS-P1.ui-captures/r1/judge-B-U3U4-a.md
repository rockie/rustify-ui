# U3 / U4 UI 验收评审（R1，规范模式）

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.5 | 2.0 | 2.0 | 0.5 | **7.0** |
| U4 Studio 本地库迁移与存储反馈 | 2.0 | 2.0 | 1.75 | 0.25 | **6.0** |

**U3**
- 布局 2.5：对话框几何符合 X6。宽 1022px（约 64rem），导入审阅高 866px（视口减 2rem），四周 padding 24px，内部纵向滚动，390 下表单行为单列。没到 3 分有两个原因：审阅结果的版本、名称、配置是三段没有标签的散文，和输入区之间没有分组；点复制后反馈插在按钮上方，按钮被挤下约 15px。
- 视觉 2.0：popover 白底带边框和圆角、30% 遮罩（取样 #B7BBC0）、代码约 13px / 行距约 20.8px 都对。但 label 是常规字重，表格里的颜色值不是等宽字，分隔线、次级文字、错误色都偏离 G3 token，设计师一眼能看出来。
- 交互 2.0：导入（版本、名称、配置、差异、备注、Apply）和导出（v2、兼容 v1 损失清单、CSS profile 与记法、Rust 接入、复制反馈）的主路径都能走通。不能更高的原因：未知版本的错误给出了误导性的诊断（P1）；Rust 说明没写所需 SDK/CLI 版本；v1 损失清单少了两类。
- 细节 0.5：有若干文案和数字格式的小差异。

**U4**
- 布局 2.0：状态区放在保存动作下方，位置对，390 下单列可用。不能更高的原因：升级 Review 是原始 JSON，塞在 260px 的编辑列里；冲突的版本信息排在决策按钮之后；浮动的「Host control」遮住了编辑列内容。
- 视觉 2.0：callout 是扁平灰底。不能更高的原因：次级文字、错误色、callout 边框都不是 G3 token；Review 标题比正文还小还轻；失败态的严重度表现不一致；同一级动作混用三种按钮样式。
- 交互 1.75：每个状态都有常驻文字和继续动作（Review/取消、重试/取消写入、Load/Keep/Save copy、下载原始数据）。但迁移 Review 读不懂（P1），升级成功没有宣告，命名动作的反馈混在通用状态里并且和操作列表自相矛盾，冲突只给版本元数据，无 Web Locks 时没有常驻模式指示。
- 细节 0.25：差异很多：小写开头的句子片段、重复消息、重复且分不开的操作行、占位词「choose」、行尾悬挂的「·」。

## 2. 上一轮差距复核表

首轮，省略。

## 3. 差距清单

按严重度、再按可挽回分排序。编号均为首轮临时编号。几条颜色 token 差距（新-3 / 新-9 / 新-16 / 新-26）同根，修一次壳层 token 即可同时关闭，分别记在各自单元。

### P1

**新-1 · U4 · U4-upgrade-review-1440-light.png，编辑列「Review the complete migration」框 · 布局 + 交互 · P1 · 可挽回 0.8（布局 0.5、交互 0.3）**
- 现状：Review 直接展开在 320px 编辑列（内容宽约 260px）里。先是一行 13px 常规字重的标题，然后是「Saved themes retained: 2」和一段说明，接着是原始迁移信封 JSON：`"envelope_version": 2`、`"revision": 0`、`"migrated_from_v1": {"sha256": "d262e6a0…` 64 位哈希折成两行，后面还有 `"backup_key"`。首帧看不到两个命名主题的名称，看不到将保存的草稿和偏好，也看不到确认按钮。用户只能在窄列里逐行读 JSON。
- 期望：S3 要求「升级 Review 展示原 v1 库的完整迁移结果及将保存的当前草稿/偏好」，并提供「下载升级前原始数据」；S2 要求显式「升级并保存当前草稿」或取消。
- 修法：这里要换容器、重做结构，不是调样式。
  1. 把 Review 移到与 Import 审阅相同的对话框外壳：宽 `min(100vw - 2rem, 64rem)`，高上限 `100dvh - 2rem`，内部滚动，padding 1.5rem。
  2. 内容顺序：
     - 标题（18px / 1.3）「升级本地库到 v2」。
     - 带 13px/600 标签的摘要描述列表：来源「v1 库 · sha256 d262e6a0…」，「将迁移的命名主题：2」后接表格（名称 | id | 结果），「将保存的当前草稿：<名称>（已修改/未修改）」，「将保存的偏好：语言、预览模式…」，以及迁移诊断。
     - 原始 JSON 收进 `<details>「查看原始迁移候选」`，内容用等宽 12px 缩进 JSON。
  3. 底部动作行：solid「升级并保存当前草稿」、outline「下载升级前原始数据」、outline「取消升级」。
  4. ≤600px 时对话框占满视口减 2rem，单列。

**新-2 · U3 · U3-import-error-1440-light.png，Review 按钮上方红字 · 交互 · P1 · 可挽回 0.45**
- 现状：输入 `{"schema_version":999}` 后报「Import failed: json: missing field \`id\` at line 1 column 22」。真实原因是版本不受支持，界面却在指缺字段，用户会以为补一个 id 就能导入。文案还直接暴露了解析器内部格式（`json:`、反引号、行列号）。
- 期望：X1「导入先呈现…版本和诊断」；本单元状态写明是「导入错误（未知版本）」；G6 要求错误文案中英同步。
- 修法：
  1. 导入时先读取 `schema_version`，不是 1 或 2 就直接给版本诊断，例如「不支持的主题 JSON 版本 999。本 Studio 可读取 v1、v2。当前主题未改变。」（中英两套）。
  2. 只有语法错误才附行列号，并改成人话：「第 1 行第 22 列附近 JSON 语法错误」。
  3. 错误同时绑定到 Theme source 文本域：`aria-invalid`、`aria-describedby`，并加错误边框（颜色见新-16）。

### P2

**新-3 · U4 · 全部 U4 图的状态行、callout · 视觉 · P2 · 可挽回 0.3**
- 现状：
  - 次级文字取样为 #667085（「Legacy draft restored…」「Decisions use the version displayed here…」）。
  - 错误红为 #D92D20（U4-quota-failure 红字）。
  - callout 边框为 #858F9E，底色 #F2F4F7（升级、冲突、版本框）。
  - 次级字在灰底上的对比度只有 4.51:1，红字在灰底上是 4.38:1，低于 AA 的 4.5。
- 期望：G3 规定次级文字 #475467、字段错误 #b42318、壳层分隔 #d0d5dd / #eaecf0，壳层扁平。
- 修法：
  1. 把壳层 caption 色改为 #475467，失败文字改为 #b42318。
  2. callout 边框改为 #d0d5dd；底色保留浅中性，或改白底只留细边框。
  3. 改完复测：#475467 和 #b42318 在 #F2F4F7 上都能 ≥ 4.5:1。

**新-4 · U4 · U4-upgrade-cancelled、U4-quota-failure、U4-quota-operations · 交互 · P2 · 可挽回 0.3**
- 现状：
  - 命名保存「Memory only」被拦截后，反馈直接覆盖通用状态行：「Review the storage warning before this library action. storage: …」。没有点名主题，也没说结果（未保存）。
  - 配额失败时，顶层两条消息都写「Local save failed…」，没提「Retained name」。读者无法判断是草稿还是命名保存失败了。
  - 展开操作列表后却显示「Draft · confirmed」×2 和「Save Retained name · failed; retained」，与顶层「draft … remain in memory」的说法互相矛盾。
- 期望：S4「保存状态区分排队、先前快照成功而当前仍待保存、当前草稿确已保存；命名库动作单独反馈」。
- 修法：拆成两条独立的反馈位。
  1. 草稿状态行固定在保存按钮旁，三选一：「当前草稿已保存（rev N）」「先前快照已保存，当前修改待保存」「排队中」。
  2. 命名库动作反馈放在「Local themes (N)」下方，单独一行，点名并给出结果和动作，例如「“Retained name”未保存：存储已满 · 重试 / 放弃」。
  3. 失败消息不得暗示草稿未保存，除非草稿确实没保存。

**新-5 · U3 · 导入/导出对话框全部表单标签与表头 · 视觉 · P2 · 可挽回 0.25**
- 现状：「Input format」「Theme source」「Output」「CSS profile」「Color notation」「Exported code」、差异表表头「Mode / token / Current / Imported」、「Import notes」标题，都是约 14px 常规字重，颜色 #1D2939，和正文没有区分。
- 期望：G3「label 13px / 600」。
- 修法：给对话框表单 label、表头、备注标题统一用壳层 label 样式：`font-size: 13px; font-weight: 600; line-height: 1.4`。

**新-6 · U3 · U3-import-review-1440/-end，「Review changes」按钮下方结果区 · 布局 · P2 · 可挽回 0.25**
- 现状：「JSON v1 → v2」「Imported legacy 旧主题 · 6 changes」「appearance=inherit · …」是三段裸文本，没有「版本 / 名称 / 配置」标签，也没有区块标题或分隔线，读起来像输入区的延续。
- 期望：X1 要求呈现版本、名称、配置、两种模式的 base/override 差异和诊断；G3 规定 label 13px/600；G2 规定壳层用细分隔线和分组行。
- 修法：
  1. 在 Review 按钮下方加一条 #eaecf0 分隔线和小标题「导入候选」。
  2. 版本、名称、配置改成两列描述列表（`dl`，标签 13px/600、#475467），版本显示为「v1 → v2（将升级）」。
  3. 差异表与 Import notes 放在同一组内。

**新-7 · U4 · U4-upgrade-pending / review / cancelled、U4-conflict 的 callout · 视觉 · P2 · 可挽回 0.25**
- 现状：
  - callout 没有标题，第一句就是长段正文。
  - 状态行「Legacy draft restored in memory…」和 callout 第一句意思重复。
  - Review 框标题「Review the complete migration」约 13px、常规字重、偏灰，比下面 14px 的正文还弱，层级倒置。
- 期望：G3 字级层级（标题 18px/1.3，label 13px/600，body 14px/1.5）；S1 要求常驻反馈可读。
- 修法：
  1. 每个 callout 首行加 13px/600 标题，如「本地库升级待确认」「另一个标签页保存了不同版本」「本地存储不可写」，正文 14px/1.5。
  2. 删掉状态行里和 callout 重复的句子，让状态行只说保存结果。
  3. Review 标题用 18px/1.3 或至少 13px/600。

**新-8 · U4 · U4-upgrade-pending、U4-conflict 的动作按钮 · 视觉 · P2 · 可挽回 0.25**
- 现状：同一级动作混用三种样式。
  - 约 24px 的小 outline：「Review / recheck upgrade」「Download original before upgrade」「Retry retained saves」。
  - 约 32px 的 outline：「Load other version」「Save current as a copy」。
  - 纯文字链接：「Cancel upgrade」「Keep current draft」「Cancel pending writes」。
  - 冲突的三个决策里，Keep 是链接，另外两个是按钮，看上去不对等。
  - 升级待确认时，唯一的 solid 主按钮是「Save locally」，必经的下一步「Review」反而是最弱的小 outline。
- 期望：S5「Load / Keep / Save copy 显式决策」；S2 Review 是继续路径；G4 选中/按下态语义。
- 修法：
  1. 定一套壳层按钮角色：推荐下一步用 solid 32px，并列选项用 outline 32px，不用文字链接承载决策。
  2. 冲突三选项做成同尺寸 outline 横排，可换行。
  3. 待升级时，callout 内的「Review upgrade」用 solid。
  4. 「Cancel upgrade」「Cancel pending writes」用 outline。

**新-9 · U3 · U3-import-review-end 差异表、Import notes 框、对话框和代码框描边 · 视觉 · P2 · 可挽回 0.2**
- 现状：表格行分隔线取样为 #858F9E，Import notes 和导出代码框描边也是 #858F9E。行线很重，表格显得发黑。
- 期望：G3 壳层分隔 #d0d5dd / #eaecf0。
- 修法：表头下边线用 #d0d5dd，行分隔用 #eaecf0，备注框和代码框边框用 #d0d5dd。对话框外框是否用 popover border token 不在此限，但内部分隔必须用壳层分隔色。

**新-10 · U3 · U3-export-rust-1440，Output 下方说明 · 交互 · P2 · 可挽回 0.2**
- 现状：「Use the matching source SDK and CSS; the published 0.2.1 SDK/CLI reject v2.」写了旧版本不兼容，但没给出所需的 SDK/CLI 版本或修订号。
- 期望：X4「接入说明标明所需 SDK/CLI 版本」。
- 修法：在说明首行明确写「需要 rustify-ui <实际版本或 git rev>、CLI <实际版本>」。生成代码顶部注释同步写同一个值，取值来源要与构建一致。

**新-11 · U4 · U4-conflict-1440 / -detail，两张卡片的顺序 · 布局 · P2 · 可挽回 0.2**
- 现状：决策卡（三个动作）在上，冲突版本卡（「External revision 3 · Rustify Light · Saved themes: 0」和说明）在下，中间还隔了一条分隔线。用户要先往下读完版本信息，才知道该点上面哪个按钮。
- 期望：S5「显式决策，展示冲突版本」。
- 修法：合成一个冲突 callout，顺序为：标题 → 冲突版本摘要（含差异，见新-14）→ 三个并列决策按钮 →「查看原始存储文本」和「下载显示的原始数据」。

**新-12 · U4 · U4-quota-failure（红字）、U4-no-locks（灰字）、U4-conflict / upgrade（中性灰框） · 视觉 · P2 · 可挽回 0.2**
- 现状：同属「本地写不了」的两种失败，配额失败用红色正文，无 Web Locks 只是一行灰色 caption，严重度表现不一致。无锁这类阻断写入的状态在视觉上和「Saved on this device」同一级。
- 期望：S1 / S6 常驻反馈；G3 字段错误 #b42318。
- 修法：给失败态统一用一个 callout 变体：标题 13px/600、#b42318，正文 #475467，边框 #d0d5dd，加文字前缀「错误」「需要操作」，不只靠颜色区分。配额失败、无锁、存储不可用都套这一变体；升级待确认和冲突用中性变体。

**新-13 · U4 · U4-upgraded-1440，状态行 · 交互 · P2 · 可挽回 0.2**
- 现状：确认升级后，callout 消失，只剩「Saved on this device」，没有说明库已升级、迁移了几个主题。
- 期望：S3 要求提交成功后才显示升级/保存成功，也就是成功后应当明确宣告。
- 修法：提交成功后显示一次「本地库已升级到 v2：已迁移 2 个主题，当前草稿已保存」，并保留「下载升级前原始数据」入口。之后状态行回到常规保存状态。

**新-14 · U4 · U4-conflict-1440，版本卡 · 交互 · P2 · 可挽回 0.2**
- 现状：冲突版本只显示修订号、主题名、命名主题数。两个标签页的实际差异（本页 primary #123456，另一页 #654321）只能展开「View exact stored text」看原文。
- 期望：S5「展示冲突版本」，即用户能据此选择 Load / Keep / Save copy。
- 修法：复用导入审阅的差异表（`Mode / token | 本标签页 | 另一标签页`），列出 identity 与 base/override 的变化。原文仍放在 details 里。

**新-15 · U3 · U3-import-review 差异表的 Current / Imported 列 · 视觉 · P2 · 可挽回 0.15**
- 现状：#1570ef、#7c3aed 等颜色值用 IBM Plex Sans 比例字，约 13px，和 token 名同一字体。
- 期望：G3「颜色值等宽 12px」。
- 修法：颜色值单元格改用等宽 12px，前面加 12×12 的色块，色块带 1px #d0d5dd 边框。

**新-16 · U3 · U3-import-error-1440 红字与文本域 · 视觉 · P2 · 可挽回 0.15**
- 现状：错误文字取样为 #D92D20；Theme source 文本域保持普通边框，没有错误态。
- 期望：G3 字段错误 #b42318；F4 的壳层错误边框规则。
- 修法：错误文字改为 #b42318；出错时文本域边框改为 #b42318，并加 `aria-invalid="true"`。

**新-17 · U3 · U3-import-review，配置行 · 视觉 · P2 · 可挽回 0.15**
- 现状：「appearance=inherit · accent_color=null · gray_color=null · panel_background=null · radius=null · scaling=100」是开发者格式：snake_case 键、`null`、等号串联。
- 期望：X1 呈现配置；C2 要求选择器有明确标签和有效 auto 结果，命名为品牌色、中性色、外观、面板、圆角、缩放。
- 修法：改成描述列表，用与基础配置一致的标签与值，例如「外观：继承 · 品牌色：未设置（v1 无此项）· 中性色：未设置 · 面板：未设置 · 圆角：未设置 · 缩放：100%」，标签中英同步。

**新-18 · U3 · U3-export-v1-1440，损失清单 · 交互 · P2 · 可挽回 0.15**
- 现状：四条损失覆盖了色族/auto、覆盖来源、非 49 键派生角色、圆角和组件尺度，没有提到 appearance 意图和 scaling 意图。
- 期望：X2 要求列出「色族/auto 选择、appearance 意图、radius full/组件尺度角色、scaling 意图、覆盖来源、非旧 49 键派生角色」。
- 修法：清单固定包含 appearance 与 scaling 两条，例如「外观意图 inherit 不会写入 v1」「缩放 100% 意图丢失，v1 只含解析后尺寸」。若当前值不构成损失，也写一行「当前为默认值，导入 v1 后等价」。
- 说明：清单里能看出带当前值（「pink」「resolved mauve」），可能是有意只列有损项。本图无法区分，需用非默认的 appearance / scaling 补采复核。

### P3

**新-19 · U3 · U3-export-rust → U3-export-copied，复制前后对比 · 布局 · P3 · 可挽回 0.15**
- 现状：复制后「Copied to clipboard.」插在代码框和按钮行之间。对话框变高约 31px 并重新居中（顶边从 106 移到 90），「Copy code」按钮从 y≈719 移到 y≈734，刚点完的按钮在指针下移动了。
- 期望：X5 复制成功后报告；G4 无装饰动画依赖，反馈不应打乱操作位置。
- 修法：给反馈预留固定高度的 `role="status"` 区，放在按钮行右侧与按钮同行，或者一直占位，避免布局位移。

**新-22 · U4 · 全部 U4 图左下（1440）/ 右下（390）的「Host control」浮标 · 布局 · P3 · 可挽回 0.15**
- 现状：固定定位的「Host control」小标签压在编辑列内容上：
  - U4-upgrade-review 里遮住了 JSON 行；
  - U4-upgrade-pending-390 里压住了「Favorites」按钮的下边；
  - 其他图里遮住了 Appearance 选择器或说明文字。
- 期望：G2 布局不遮挡；G6 关键动作可达。
- 修法：把它移出编辑列覆盖区，放进页面头部或预览工具栏的文档流；如果必须固定，给编辑列底部留出不少于浮标高度加 8px 的安全区，并避开可交互元素。这是壳层问题，U1 也受影响。

**新-23 · U4 · U4-upgrade-cancelled，状态行 · 细节 · P3 · 可挽回 0.15**
- 现状：「Review the storage warning before this library action. storage: persistent writes are protected; review the migration or explicitly resume saving」。第二句以小写「storage:」开头，像拼进来的内部前缀；句末没有句号；提到的「storage warning」「resume saving」在界面上找不到同名的元素或按钮。
- 修法：改写为「未保存“Memory only”：库升级已取消，当前仅内存编辑。请重新检查升级后再保存。」，并配上指向「Review upgrade」的动作。中英同步。

**新-24 · U4 · U4-quota-failure / operations，两条失败消息 · 细节 · P3 · 可挽回 0.15**
- 现状：灰字「Local save failed; memory and pending actions are preserved. Injected full device」和红字「Local save failed; draft and pending actions remain in memory: Injected full device」几乎同义却重复出现。灰字里原始异常文本直接拼在后面，没有标点。
- 修法：只保留一条失败消息（样式按新-12）。异常原文放进「详情」，以「浏览器返回：…」的形式呈现。

**新-25 · U4 · U4-quota-operations，操作列表 · 细节 · P3 · 可挽回 0.15**
- 现状：两行完全相同的「Draft · confirmed」，无法区分；「Save Retained name · failed; retained」里主题名没有引号，和动词连在一起。
- 修法：每行加修订号或时间（如「草稿 rev 2 · 已确认 · 05:54:47」），主题名加引号「保存“Retained name” · 失败，已保留」，状态用文字标签。

**新-26 · U3 · 字节计数、导出说明、「Saved on this device」「Copied to clipboard.」等 caption · 视觉 · P3 · 可挽回 0.1**
- 现状：caption 取样为 #667085。
- 期望：G3 次级文字 #475467。
- 修法：同新-3，统一壳层 caption token。

**新-27 · U4 · U4-upgrade-pending / cancelled，「Download original before upgrade」 · 布局 · P3 · 可挽回 0.1**
- 现状：这个按钮在升级 callout 外面，和 callout 之间隔着空白，看起来属于下面的「Pending / recent operations」。
- 期望：S2 / S3 的升级相关出口应与升级状态放在一起。
- 修法：移进升级 callout 的动作行，或放进新-1 的 Review 对话框底部。

**新-28 · U4 · U4-upgrade-cancelled，callout · 交互 · P3 · 可挽回 0.1**
- 现状：取消之后，callout 还显示「Cancel upgrade」链接，第一段仍写「No old bytes… are written until you review and confirm」，属于过期的动作和文案。
- 期望：S2 取消后常驻「仅内存编辑」与重新检查入口。
- 修法：取消后的 callout 只保留标题「已取消升级 · 仅内存编辑」、说明，以及「重新检查升级」「下载升级前原始数据」两个动作，移除「Cancel upgrade」。

**新-29 · U4 · U4-upgraded-1440，预设选择器 · 细节 · P3 · 可挽回 0.1**
- 现状：选择器显示小写占位词「choose」，下方是「旧草稿」。
- 修法：占位改为「选择预设…」/「Choose a preset…」；当前主题不来自预设时，显示「自定义：旧草稿」。

**新-30 · U4 · 「Review / recheck upgrade」 · 细节 · P3 · 可挽回 0.1**
- 现状：用斜杠把两个动作合成一个按钮名。
- 修法：按状态切换单一标签：待确认时叫「审阅升级」，取消后叫「重新检查升级」。

**新-31 · U4 · U4-conflict，版本卡首行 · 细节 · P3 · 可挽回 0.1**
- 现状：「External revision 3 · Rustify Light ·」在「·」处折行，分隔符悬挂在行尾。
- 修法：改成描述列表（修订号 / 名称 / 命名主题数 各占一行），或者用不可断行的分隔片段。

**新-32 · U3 · 导入对话框计数 · 细节 · P3 · 可挽回 0.1**
- 现状：说明写「up to 256 KiB」，计数显示「3332 / 262144 bytes」，单位不一致，也没有千分位。
- 修法：计数改为「3.3 KiB / 256 KiB」，或带千分位「3,332 / 262,144 字节」，与说明使用同一单位。

**新-33 · U3 · U3-export-rust，「To match this preview…」行 · 细节 · P3 · 可挽回 0.1**
- 现状：说明里写 `font_context(16, true)`，代码里是 `font_context(16., false)`，字面量不一致，照抄说明可能得到类型不符的代码。标识符用的是正文比例字。
- 修法：说明里的调用与代码保持一致（`16.`），并用等宽字的 inline code 呈现 `mount_exported_preview`、`ThemeMode::Light`、`font_context(16., true)`。

**新-34 · U3 · 导出对话框底部两行状态 · 细节 · P3 · 可挽回 0.1**
- 现状：按钮上方是「Copied to clipboard.」，下方是「Saved on this device」，两行都是灰色 caption，夹着按钮，读者分不清哪条属于复制、哪条属于保存。
- 修法：复制反馈与按钮同行（见新-19）。保存状态如果要在对话框里出现，加前缀「草稿：已保存到本设备」，放在对话框页脚的固定位置。

**新-35 · U3 · U3-export-css，Output 与 CSS profile 的命名 · 细节 · P3 · 可挽回 0.1**
- 现状：Output 选「Tailwind v4 CSS」，下面 profile 又是「Rustify scoped」，读起来像两者冲突。
- 期望：X3 两种 profile 是「标准 Tailwind v4」与「Rustify scoped」。
- 修法：Output 项改名为「CSS（已解析）」，profile 选项命名为「标准 Tailwind v4（.dark）」「Rustify scoped（data-theme）」。

**新-36 · U3 · U3-export-css，代码注释里的字体路径 · 细节 · P3 · 可挽回 0.1**
- 现状：同一段注释里出现两种拼法：「makepad/widgets/resources/FONT-LICENSES.md」和「makepad_widgets/resources/IBMPlexSans-Text.ttf」。
- 期望：X4 外部字体资源要标明。
- 修法：统一成一种对使用者可解析的路径或 URL 写法，并说明这是相对什么位置的路径。

**新-21 · U3 · U3-export-copied，复制反馈的样式 · 交互 · P3 · 可挽回 0.1**
- 现状：成功反馈只是一行灰色 caption（#667085），没有成功语义，也看不出它属于哪个按钮。
- 期望：X5 复制成功后报告成功。
- 修法：反馈用 `role="status"`，文字带「已复制」前缀，与 Copy 按钮同行。不要只靠颜色，可以在按钮旁显示「已复制 ✓」文字，并在约 2s 后恢复。

**新-20 · U4 · U4-no-locks-1440，状态行 · 交互 · P2 · 可挽回 0.1**
- 现状：只有一行动作级 caption：「Local writing is unavailable. This action was not queued; memory editing and downloads remain available.」。没有不依赖于上一次动作的常驻模式指示，「Save locally」仍是 solid 主按钮。
- 期望：S1「只读持久化…常驻反馈」；S6「明确『仅内存编辑/可下载』」。
- 修法：页面一载入、检测到无锁就显示常驻 callout（失败变体）：「此浏览器不支持 Web Locks：本次仅内存编辑，可导出/下载」。「Save locally」改为 outline，并在按钮旁注明「不可用」，或继续保持可点但如实报失败。

**新-37 · U4 · U4-quota-failure 消息 · 交互 · P3 · 可挽回 0.05**
- 现状：只说「remain in memory」，没告诉用户仍可导出或下载。
- 期望：S6「写失败保留内存草稿且仍可导出」，并明确「可下载」。
- 修法：失败文案末尾加「仍可导出或下载当前主题」，并附「导出」按钮。

**合计核对**
- U3 可挽回合计 2.8（新-2、5、6、9、10、15、16、17、18、19、21、26、32–36），10 − 7.0 = 3.0，差 0.2。
- U4 可挽回合计 3.95（新-1、3、4、7、8、11–14、20、22–25、27–31、37），10 − 6.0 = 4.0，差 0.05。

## 4. 建议（不扣分）

1. 导出 JSON（v2 / v1）目前是压缩成一行的文本，换行处会切断键名和字符串，例如 1440 的「"accent-｜foreground"」、390 的「"card":"｜#f5f5f5"」。建议按 2 空格缩进输出，与导入源和升级 Review 一致；或者代码框设 `white-space: pre` 并允许内部横向滚动。目标包没有对此作规定。
2. 导入审阅和导出对话框的标题行和动作行（Apply import / Cancel、Copy / Select / Download）可以固定为 sticky 头尾。现在 1440 导入审阅首帧看不到 Apply，滚到底时关闭按钮又滚出了视野。
3. Rust 视图有两个下载按钮，「Download matching JSON」在代码上方，底部的「Download」指向不明。建议把底部按钮改名为「下载 theme.rs」，两者并入同一个动作行。
4. v1 损失清单框和「Import notes」用的是同一种中性灰框。建议加文字标签「有损导出」作区分。
5. 导入对话框里「Cancel」是文字链接，同框的其他次要动作（Choose file、Review changes）是 outline 按钮。建议统一。
6. 歧义一：X6「对话框宽高为视口减 2rem」可以读作上限，也可以读作固定尺寸。现实现把它当上限：导出对话框高 562px，代码框只有约 315px 可视。若本意是固定高度，代码区应撑满。请目标包负责人明确。
7. 歧义二：X2「需用户显式选择后才导出」。现实现把在 Output 里选「Compatible JSON v1 · lossy」当作显式选择，选完立即可以复制和下载。若本意是要用户先确认读过损失，可以加一个「我已了解上述损失」复选框，勾选后才启用复制和下载。

## 5. 看不出的格子

**U3**
- 所有 U3 图：看不出导入、导出对话框里的键盘焦点环（G4）。本单元没有键盘帧。
- 所有 U3 图：看不出中文界面下的导入、导出、错误文案（G6）。
- U3-import-review / -end：样本只含「修改」类差异，看不出 override 新增和删除如何呈现（X1）。
- U3-import-review-390：看不出 390 下差异表下半部分、Import notes 和 Apply / Cancel 是否可达且不溢出。只拍了首帧。
- 未采：语法错误、超过 256 KiB、不支持的颜色输入这三种导入错误（X1）。
- U3-export-css：Output、Profile、Color notation 三个下拉都没展开，看不出是否真有四种颜色记法和「标准 Tailwind v4」profile（X3）。
- U3-export-rust：代码只显示到第 2 个函数，看不出后文是否列出了「the listed same-origin font files」（X4 外部字体资源）。
- 未采：复制失败、下载失败时是否保留可选文本和下载路径，下载实际发起后的反馈（X5）。
- U3-export-json-390：只拍了 v2，看不出 390 下 v1 损失清单、CSS 三个选择器、Rust 说明的排布。
- U3-export-v1：看不出损失清单在 appearance / scaling 为非默认值时是否会补上这两条（与新-18 相关）。

**U4**
- U4-upgrade-review：确认按钮在视口下方，看不出它的文案是否为「升级并保存当前草稿」（S2），也看不出 Review 是否完整列出两个命名主题和将保存的偏好（S3）。
- 未采：S2「模式/偏好变化不能绕过确认」。
- 未采：S4 的「排队」「先前快照成功而当前仍待保存」两种保存状态，以及「待锁不禁用编辑/历史/导出」。
- U4-conflict：操作数为 0，看不出 S5「未处理的命名动作保持可见」。
- U4-conflict-detail：「View exact stored text」没有展开，看不出原文的呈现方式。
- U4-no-locks：只拍了动作之后的一帧，看不出动作之前是否已经有常驻提示（与新-20 相关）。
- 未采：S1 的缓存损坏、只读持久化、storage 完全不可用三种状态。
- 所有 U4 图：看不出 U4 状态在中文界面（G6）和键盘焦点（G4）下的表现。
- 未采：390 下除「待确认」外的 U4 状态。目标包只要求待确认，不扣分。

## 6. 总评

- **最低分单元：U4，6.0 分。** 主因是升级 Review（新-1）：它把原始迁移信封 JSON 塞进 260px 编辑列，用户无法据此判断迁移会带来什么。其次是保存反馈：草稿状态和命名库动作共用一个状态位，配额失败时顶层消息和操作列表互相矛盾（新-4）。另外，失败、冲突、待确认三类状态缺少统一的 callout 层级与严重度体系，按钮角色混乱（新-7、新-8、新-12）。
- **需要结构性改变：**
  1. 升级 Review 要换容器、重做结构。改用 64rem 审阅对话框，结构为「结构化摘要 + 可折叠原文 + 底部三动作」，并与导入审阅共用差异表。冲突版本卡也应复用这张差异表（新-14）。
  2. 保存反馈拆成「草稿状态」和「命名库动作」两个独立反馈位。
  3. 建一套壳层状态 callout，分中性和失败两种变体，统一标题、正文、动作行；同时建一套按钮角色（solid / outline，不用文字链接承载决策）。
  4. 壳层 token 统一回 G3（#475467、#b42318、#d0d5dd / #eaecf0、label 13px/600）。这一项同时影响 U3 和 U4。
- **U3（7.0 分）结构基本到位。** 优先修两项：未知版本的错误诊断（新-2，P1），以及 Rust 说明缺所需 SDK/CLI 版本（新-10）。其余多为 token 和排版层级的修整：label 字重、颜色值等宽、分隔色、审阅区分组、复制反馈不位移。

开图清单：U3-import-review-1440-light.png、U3-import-review-end-1440-light.png、U3-import-error-1440-light.png、U3-export-json-1440-light.png、U3-export-v1-1440-light.png、U3-export-css-1440-light.png、U3-export-rust-1440-light.png、U3-export-copied-1440-light.png、U3-import-review-390-light.png、U3-export-json-390-light.png、U4-upgrade-pending-1440-light.png、U4-upgrade-review-1440-light.png、U4-upgrade-cancelled-1440-light.png、U4-upgraded-1440-light.png、U4-upgrade-pending-390-light.png、U4-quota-failure-1440-light.png、U4-quota-operations-1440-light.png、U4-conflict-1440-light.png、U4-conflict-detail-1440-light.png、U4-no-locks-1440-light.png。U3、U4 是规范模式，没有目标图。另读了 DOM-COMPONENTS-P1.ui-target.md、capture.md、U3-actions.log、U4-actions.log。放大用的裁剪图在 /private/tmp/claude-501/-Users-rockie-Documents-gh-rockie-rustify-ui/0bc04ccb-5f93-4892-a089-e1cb25f9419c/scratchpad/judge-B/，共 8 张：u3-390-bottom、u3-hostcontrol、u3-err、u3-table、u3-copied、u3-css-head、u4-callout、u4-390-bottom、u4-conflict、u4-review 中实际打开了这些（u3-rust-btn 生成后未打开）。
