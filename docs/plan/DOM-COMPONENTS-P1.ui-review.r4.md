# DOM-COMPONENTS-P1 · UI 验收评审 R4

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R4（全量；与 R3 同一基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [DOM-COMPONENTS-P1.ui-target.md](DOM-COMPONENTS-P1.ui-target.md) · `567117725f56` | HEAD `5d122d62`，dirty，产品路径无 diff（tracked diff 只含 SDLC 台账与两张旧验证 PNG）；Studio `28ae38531384c902` http://127.0.0.1:4181/ ，catalog `32b52b66f04701ad` http://127.0.0.1:4176/ | [r4](DOM-COMPONENTS-P1.ui-captures/r4/) | [R3](DOM-COMPONENTS-P1.ui-review.r3.md) |

## 验收判定

**需要修改 · 停滞预警（UI-128 所在 U3）**

- 本轮分数 9.35（U3），门槛 8（沿用 R1–R3，默认值）。历史最佳 9.35；本轮与 R3 同一基线（目标指纹未变）。
- 七个单元的分数都在 9.35 以上，但还有 1 条门槛项：UI-207（P1，U5）。在深色局部边界内打开的预览对话框，表面已随边界变深，主按钮 Confirm 仍取根主题的浅色 primary。R3 同一帧是正确的，所以这是本轮回归。主 agent 在运行页面上实测确认：按钮自身作用域的 `--primary` 是深色值 `rgb(83 177 253)`，背景却解析为 `rgb(21, 112, 239)`。
- **停滞预警（UI-128，U3）**：UI-128 是 P2，自 R3 重建基线以来已连续两轮被判「部分解决」（R3、R4）。这两轮之前，修复记录都把它的最终处置登记为「已修改待评审」（R2 → 修复、R3 → 修复），满足报告合同「顽固差距」的条件。U3 还没有做过重构轮，下一轮修复须对 U3 做一次重构轮。U3 评审指出的结构性做法是：把 Import candidate 的配置行并入或复用 Changes 表格组件，统一列定义、表头样式和独立的 Change 列。UI-198、UI-199、UI-25 和 UI-176 的大部分会随之关闭。
- R3 唯一的门槛项 UI-192 由主 agent 在页面上复核，判已解决。没有待裁定事项。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 评审 | R3 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 3.0 | 2.9 | 3.0 | 0.9 | 9.8 | a | 8.7 |
| U2 Studio 精调与来源解释 | 2.85 | 2.9 | 2.85 | 0.8 | 9.4 | a | 8.7 |
| U3 Studio 导入与导出 | 2.9 | 2.85 | 2.9 | 0.7 | **9.35** | a | 9.05 |
| U4 Studio 本地库迁移与存储反馈 | 2.9 | 2.95 | 2.85 | 0.85 | 9.55 | a | 8.6 |
| U5 Studio 预览 | 2.95 | 2.8 | 2.8 | 0.9 | 9.45 | a | 9.45 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.95 | 3.0 | 1.0 | 9.95 | a | 9.9 |
| U7 布局与排版 | 3.0 | 3.0 | 3.0 | 0.9 | 9.9 | a | 9.3 |

- 本轮分数：**9.35**（U3）。
- 历史最佳：9.35（R3 为 8.6，同一基线）。
- 门槛：8。门槛不写入目标包，也不告诉评审。
- 单元达标：U1、U2、U3、U4、U6、U7。U5 分数过门槛，但有门槛项 UI-207，所以未达标。
- 每单元一位评审，没有确认评审或重评。本轮判「需要修改」，不需要确认评审。
- 各维度理由见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r4/judge.md)。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |

## 上轮差距复核

R3 共 64 条差距：评审来源 63 条，机器检查 1 条（UI-192）。本轮结果：

- 已解决 53 条：评审来源 52 条，加上主 agent 复核的 UI-192；
- 部分解决 7 条：UI-02、UI-18、UI-128、UI-176、UI-25、UI-56、UI-189；
- 仍存在 3 条：UI-172、UI-115、UI-76；
- 待核实 1 条：UI-160；
- 没有回归，没有不再适用。

处置取自 [M7「UI 验收修复」](DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)中的「R3 → 修复」：64 条都是已修改待评审，没有不采纳、待决定或阻塞。

- 评审来源的差距由本轮评审复核，UI-192 由主 agent 在运行页面上重新检查。
- 跨单元的旧差距：UI-106、UI-107、UI-161 涉及的单元都判已解决。UI-160 的 U1 评审判已解决、U2 评审判待核实，按 回归 > 仍存在 > 部分解决 > 待核实 > 已解决 的顺序取待核实。
- 判已解决、但修法带出新问题的：UI-163 → UI-194，UI-17 → UI-195，UI-51 与 UI-140 → UI-201，UI-81 → UI-210。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-192 | [P1] 200% 文本下精调 Colors 网格与来源块丢失内容 | 已修改待评审 | 已解决 | 主 agent 页面实测：720×900 + 根字号 200%，`#studio-editor` scrollWidth 319 = clientWidth 319、scrollLeft 0；token 格单列 229px，格内无溢出；选中 primary 后有效值在 x 32–133、解除按钮在 x 32–287，都在 320px 编辑列内（[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check.json) `zoom200`）。实拍见 U1-zoom200-colors、U1-zoom200-source-a2、-end |
| UI-157 | [P2] 200% 下渲染器分段控件（DOM / GPU / DOM + GPU）宽于预览列，右缘被视口截掉 | 已修改待评审 | 已解决 | U1：U1-zoom200-720 渲染器控件 x=352–687，右边框落在预览列内，右圆角完整 |
| UI-158 | [P2] 200% 下导出对话框的 Output 选择器被压窄，当前值截成「Theme…」 | 已修改待评审 | 已解决 | U1：U1-zoom200-export-720 中 Output 标签在上、选择器占满宽，完整显示「Theme JSON v2」 |
| UI-02 | [P3] 分段控件残留：焦点环压到相邻段，两种接缝画法并存 | 已修改待评审 | 部分解决 | U1：Dark 焦点环已内收，不再压邻段；EN/中文、Edit/Preview 接缝仍为两条线。见本报告差距清单同编号条目 |
| UI-103 | [P3] 顶栏在 768 多一行，在 200% 固定占视口 39% | 已修改待评审 | 已解决 | U1：768 顶栏两行（y=0–88）；200% 顶栏占视口 28% |
| UI-104 | [P3] 200% 导出对话框仍用旧的蓝边次级按钮和非 token 边框 | 已修改待评审 | 已解决 | U1：200% 对话框次级按钮为 #D0D5DD 边、#475467 字，选择器边框 #D0D5DD |
| UI-106 | [P3] 状态行与解除按钮之间 0px；两栏标题中线差约 4.5px | 已修改待评审 | 已解决 | U1、U2：三组状态胶囊与按钮均隔 8px；两栏标题中线差 0.5px |
| UI-107 | [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一 | 已修改待评审 | 已解决 | U1、U2：计数与胶囊文字为 #475467；两个搜索框占位符同为 #667085 |
| UI-110 | [P3] 编辑列诊断摘要仍是「1 text pairs」 | 已修改待评审 | 已解决 | U1：编辑列摘要改为「Diagnostics」，不再带计数 |
| UI-159 | [P3] 中文「Saved on this device」有两种译法 | 已修改待评审 | 已解决 | U1：顶栏胶囊与状态块都是「已保存到本机」 |
| UI-161 | [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」 | 已修改待评审 | 已解决 | U1、U5：预览页眉为「Preview: Light / Dark」，中文「预览：浅色」 |
| UI-162 | [P3] 顶栏左右内边距与两栏不一致，主题名与说明按中线而非基线对齐 | 已修改待评审 | 已解决 | U1：Export 右边框与预览分隔线同在 x=1415；768 顶栏 16/751；主题名与说明基线同在 y=32 |
| UI-160 | [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高 | 已修改待评审 | 待核实 | U1 判已解决（U1-field-tabs 搜索框与 Clear 同为 y=536–567）；U2 判待核实（U2 各帧中搜索行被选中后的滚动移出视口）；按复核顺序取待核实 |
| UI-163 | [P2] 1440 网格中「Overridden · Light」胶囊折成两行，文字压到圆角边，与配置组状态标识形状不一致 | 已修改待评审 | 已解决 | U2：网格改为单列，「● Overridden」与配置组同形，不再折行；单列带出 UI-194 |
| UI-17 | [P3] 选中后的滚动位置没有同时露出所选格与完整来源块 | 已修改待评审 | 已解决 | U2：1440 所选格与「Clear … · Light only」同屏，390 所选格完整露出；带出 UI-195 |
| UI-18 | [P3] 小节标题仍与次级标签或字段标签同样式，或比 label 更弱 | 已修改待评审 | 部分解决 | U2：Typography、Geometry、Theme configuration 已统一；Colors 页节标题与分组标题仍是旧样式。见本报告差距清单同编号条目 |
| UI-165 | [P3] 来源块的键值结构只做了一半，行标签与块标题同样式 | 已修改待评审 | 已解决 | U2：来源块改为 Token / Effective value / Source / Path / If cleared 五组键值 |
| UI-169 | [P3] 诊断区「Authored colors fit the sRGB preview.」下方有约 44px 空白 | 已修改待评审 | 已解决 | U2：「Authored colors fit…」与下一条之间的空白已去掉 |
| UI-172 | [P3] 分组解除只给计数，看不出会影响哪些 token | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目 |
| UI-164 | [P3] 选中格里不可解除的 ↺ 仍显示为选中蓝，看起来可用 | 已修改待评审 | 已解决 | U2：不可解除的 ↺ 为 #98A2B3 |
| UI-22 | [P3] 诊断区子 summary 比父 summary 更重，两组缩进不一 | 已修改待评审 | 已解决 | U2：外层 summary 13px/600 #1D2939，内层 #475467；字体组包进同级 details，同为 x=32 |
| UI-115 | [P3] 网格 ↺ 仍只有图标 | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目 |
| UI-166 | [P3] 字体下拉的显示值是粗体，与其他下拉、输入框不一致 | 已修改待评审 | 已解决 | U2：「Custom author stack」恢复常规字重 |
| UI-167 | [P3] 句中把模式名「Light:」也设成了等宽 | 已修改待评审 | 已解决 | U2：「Light:」为无衬线，只有色值等宽 |
| UI-168 | [P3] 同一对比计数在三处用了三种措辞，父子 summary 重复同一个数字 | 已修改待评审 | 已解决 | U2：外层只写「Diagnostics」，内层不带计数，预览条保留一种句式 |
| UI-170 | [P3] 预览侧诊断条末行分隔线与外框形成双线 | 已修改待评审 | 已解决 | U2：诊断条末行与外框之间不再有分隔线 |
| UI-171 | [P3] 「Restore document · Graphite」没有写范围 | 已修改待评审 | 已解决 | U2：按钮下新增范围说明「Restores the reference document's configuration and all values in both modes…」 |
| UI-128 | [P2] Import candidate 的「→」没有「当前 / 导入」图例；Version 行的「→」含义与配置行不同；Name 行只有导入值 | 已修改待评审 | 部分解决 | U3：已补「Field | Current | Imported」图例与 Name 两侧值；Source 行仍排在 Current 列下。见本报告差距清单同编号条目 |
| UI-130 | [P3] 导出对话框按钮行下方留白 32px，导入对话框为 24px | 已修改待评审 | 已解决 | U3：所有对话框按钮行到下边框统一 24px |
| UI-175 | [P3] v1 损失清单与 Import notes 正文降为 13px/400，不在 G3 字级里 | 已修改待评审 | 已解决 | U3：v1 损失清单与 Import notes 恢复 14px/21px |
| UI-173 | [P3] 字节计数「3332 / 262144 bytes」没有千分位，与说明里的「256 KiB」单位不一 | 已修改待评审 | 已解决 | U3：「up to 262,144 bytes (256 KiB)」「3,332 / 262,144 bytes」 |
| UI-174 | [P3] Rust 接入说明有歧义：「theme.json · theme.rs」读成并列；「Keep AppHandle.」丢了「alive」 | 已修改待评审 | 已解决 | U3：「Put theme.json next to theme.rs. … Keep the returned AppHandle alive while the UI is mounted.」 |
| UI-177 | [P3] 备注中的字体族值是比例字，390 下「ui-serif」从连字符处断开 | 已修改待评审 | 已解决 | U3：ui-serif、Noto Serif 改为等宽，390 不再从「ui-」处断开 |
| UI-176 | [P3] 「Changed」徽标与列、基线没对齐，表头下方是双线 | 已修改待评审 | 部分解决 | U3：徽标列对齐与表头双线已修好；徽标基线仍低 2px，另有 390 缩进 8px。见本报告差距清单同编号条目 |
| UI-133 | [P3] 备注与差异表对 token 的命名方式不一致 | 已修改待评审 | 已解决 | U3：备注与表格的 token 命名一致；字体处理不一致另记 UI-200 |
| UI-25 | [P3] Import candidate 两组行距不一致 | 已修改待评审 | 部分解决 | U3：只剩 Source→Name 为 29px，其余 32px。见本报告差距清单同编号条目 |
| UI-178 | [P3] CSS 提示中的「Scoped dark selector: data-theme=dark」不是可用的选择器 | 已修改待评审 | 已解决 | U3：提示写作 `[data-theme="dark"]`，与代码一致 |
| UI-179 | [P2] 取消后 callout 让用户「Download your current theme」，callout 里却没有这个按钮 | 已修改待评审 | 已解决 | U4：取消态 callout 有「Download current theme」 |
| UI-50 | [P2] callout 正文字号与颜色在状态之间、甚至同一张 callout 内来回切换 | 已修改待评审 | 已解决 | U4：状态句与命名卡正文 14px #1D2939，补充说明 12px #475467 |
| UI-51 | [P2] 升级 Review 对话框底部放弃动作与次级动作同权，且用蓝边蓝字，与壳层灰描边是两套样式；配额、无锁 callout 没有推荐动作 | 已修改待评审 | 已解决 | U4：对话框底部改灰描边与文字按钮，无锁 callout 实心下载；与 UI-140 的修法叠加带出 UI-201 |
| UI-140 | [P3] 命名动作卡余项：配额卡没有内联重试；升级后卡片的文案与按钮重复 | 已修改待评审 | 已解决 | U4：配额卡有内联重试，升级后卡片按钮与 Dismiss 同行；与 UI-51 的修法叠加带出 UI-201 |
| UI-139 | [P3] 取消 callout 内同一句话写了两遍；冲突面板所在位置与其他状态不同 | 已修改待评审 | 已解决 | U4：取消态正文并为一句；冲突 callout 位置与其他状态一致 |
| UI-181 | [P3] 配额态页头与 callout 说「草稿留在内存」，操作列表却显示当前草稿已确认保存 | 已修改待评审 | 已解决 | U4：页头「Draft saved · action failed」，正文区分草稿已存与后续动作失败 |
| UI-182 | [P3] 「Saving paused」状态下页头「Save locally」仍是实心主按钮，点了只会被拦下 | 已修改待评审 | 已解决 | U4：待确认、取消、冲突、无锁态页头 Save locally 为白底描边 |
| UI-184 | [P3] 冲突键值栅格值列过窄，修订行折行出孤词，标签贴着值 | 已修改待评审 | 已解决 | U4：冲突键值改为上下两行，修订号不再折行 |
| UI-55 | [P3] 取消态 callout 两段互相重复，并与标题重复 | 已修改待评审 | 已解决 | U4：取消态只剩标题与一句正文 |
| UI-144 | [P3] 文案余项：「0 → 0」无变化行、成功态时态错 | 已修改待评审 | 已解决 | U4：「0 (unchanged)」；成功态「Original data keeps a copy of the library from before the upgrade.」 |
| UI-185 | [P3] 状态用词不统一：「upgrade」和「migration」混用，成功态重复「Saved on this device」 | 已修改待评审 | 已解决 | U4：标题与展开节统一为「library upgrade」，成功 callout 不再重复保存状态 |
| UI-114 | [P3] 页头基准用词同义反复，胶囊用语与 callout 不一致 | 已修改待评审 | 已解决 | U4：页头「Matches / Modified from reference document」；取消态胶囊与 callout 同为 Memory only |
| UI-142 | [P3] 升级 Review 对话框主题表底边与分节线形成双细线 | 已修改待评审 | 已解决 | U4：升级 Review 对话框只剩一条分节线 |
| UI-145 | [P3] 冲突差异块一处改动占四行，徽标左缘不齐 | 已修改待评审 | 已解决 | U4：路径与徽标同行，一处改动由 4 行减为 3 行；左缘余项另记 UI-206 |
| UI-180 | [P3] 配额态「Technical details」被移到错误 callout 之外 | 已修改待评审 | 已解决 | U4：「Technical details」回到红色 callout 内 |
| UI-183 | [P3] 文字按钮的标签比正文缩进 11px，单独一行的「Dismiss」像是悬空 | 已修改待评审 | 已解决 | U4：文字按钮标签与正文同在 x=34 |
| UI-56 | [P3] 操作说明段讲本状态没有的动作，说明与列表未缩进，修订号被断行 | 已修改待评审 | 部分解决 | U4：说明段不再提不可用动作，列表已缩进；说明段仍未缩进。见本报告差距清单同编号条目 |
| UI-186 | [P2] 拾取浮动面板固定在预览右下角，遮住被拾取部件和预览内容；选中矩形与标签画在面板之上 | 已修改待评审 | 已解决 | U5：拾取面板改为预览列底部整行，与被拾取部件互不遮挡 |
| UI-187 | [P3] 拾取标签盖住相邻内容，并在连字符处断开 token 名 | 已修改待评审 | 已解决 | U5：标签单行「card +3」在矩形内侧，小部件不画标签，确认后只留矩形 |
| UI-188 | [P3] 选中或按下后字重变粗，按钮变宽，右侧控件横移 1–2px | 已修改待评审 | 已解决 | U5：三帧工具条各按钮 x 完全一致；副作用（同行字重不一）记入建议 |
| UI-76 | [P3] DOM 与 GPU 两框首行左内缩相差 8px | 已修改待评审 | 仍存在 | U5：横向内缩已对齐，纵向首行差 9px（R3 时纵向对齐）。见本报告差距清单同编号条目 |
| UI-81 | [P3] classic 的内高光（上缘高光带与两侧亮边）只有目标的 1/3 到 2/3 | 已修改待评审 | 已解决 | U6：上缘四行 +80/+20/+11/+4、两侧 +18/+4，与目标差 ±2 以内；修法带出 UI-210 |
| UI-87 | [P2] Inset side=left/right 演示中正文压在贴边块边上 | 已修改待评审 | 已解决 | U7：side=left/right 改为并排，1280、390、110%、95% 下都不再相压 |
| UI-189 | [P3] Reset 的 input、textarea 脱离值列，占满整行 | 已修改待评审 | 部分解决 | U7：input、textarea 已回到值列；虚线标记越过内容右沿约 5px。见本报告差距清单同编号条目 |
| UI-95 | [P3] 布局原语区仍有样例没有可见呈现 | 已修改待评审 | 已解决 | U7：37×19 填充区实测 37×19 并带边线；240px 带底色，Invalid 后变为整宽 |
| UI-190 | [P3] Reset 区 button 行右侧有没有标注的「0」 | 已修改待评审 | 已解决 | U7：改为「clicks: 0」并落在值列 |
| UI-191 | [P3] 非法属性诊断文案重复，并带字面反引号 | 已修改待评审 | 已解决 | U7：诊断改为一句，属性名以 Code soft 呈现 |

## 差距清单

共 31 条：P1 1 条、P2 3 条、P3 27 条，没有 P0。来源：评审 30 条（旧差距 11 条、新差距 19 条），主 agent 核实 1 条（UI-212）。按 P0 → P3 排列，同一严重度内按可挽回分从高到低；主 agent 核实与待核实的条目没有可挽回分，排在同级最后。

- 新差距从 UI-193 起编号，按单元、再按评审临时编号顺序分配；与临时编号的对应写在各条「来源」里。
- 每条完整的取样数值和修法以评审原文 [judge-U<n>-a.md](DOM-COMPONENTS-P1.ui-captures/r4/) 为准。
- 截图在 [r4](DOM-COMPONENTS-P1.ui-captures/r4/) 目录中，文件名即证据位置。

### UI-207 · [P1] 深色局部边界内打开的对话框：表面随边界变深，主按钮 Confirm 仍取根主题浅色 primary（回归）（U5 · 视觉、交互与状态 · 可挽回 0.4）

- 位置：[U5-local-opposite-dialog-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r4/U5-local-opposite-dialog-1440-light.png) 对话框表面 #1D2939，Confirm（x 233–311、y 118–152）为 #1570EF 底、#FFFFFF 字。
- 对照：
  - 同一边界内的 Actions 文字为 #53B1FD；
  - [U5-dom-1440-dark](DOM-COMPONENTS-P1.ui-captures/r4/U5-dom-1440-dark.png) 中 Create profile 为 #53B1FD；
  - R3 同帧 Confirm 为 #53B1FD 底、#0C111D 字；
  - 浅色根下 [U5-dialog-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U5-dialog-1440-light.png) 的 #1570EF 是对的。
- 现状 → 期望：同一个浮层里，表面和文字跟随了深色边界，主按钮却取根主题。期望整个浮层按创建处的有效主题渲染。依据：
  - P3「局部浮层随创建处的有效主题更新」；
  - 对话框正文本身写着「This layer follows the theme where it was opened.」；
  - B6「未选 color 的 solid Button 消费有效 primary/foreground」。
- 修法：
  - Dialog 的 portal 容器带上创建处 ThemeBoundary 的完整作用域，包括 Button solid 用到的 primary、前景、第 9/10 阶与 contrast 变量；
  - Button 颜色从最近的作用域取变量，不在挂载时固化，也不从根 scope 读取；
  - 逐一对照 R3（`d5775d8a`）与 R4（`5d122d62`）之间对 Dialog、Button、ThemeBoundary、portal 的改动；
  - 补一条浏览器回归：在「Opposite to root」边界内打开对话框，断言 Confirm 的 computed 背景等于边界内 solid 按钮的背景；「Inherit root」时为 #1570EF。
- 来源：U5 新-1。
- 主 agent 核对：页面实测与评审结论一致（[machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup.json)）。「Opposite to root」时，对话框层 `data-theme="dark"`，Confirm 带 `data-appearance="dark"`，它自身计算出的 `--primary` 为 `rgb(83 177 253)`、`--primary-foreground` 为 `rgb(12 17 29)`，但 `background-color` 是 `rgb(21, 112, 239)`、`color` 是 `rgb(255, 255, 255)`。按钮的填充并没有取所在作用域的 `--primary`。

### UI-201 · [P2] 配额态：同一个失败的命名保存有三个重试入口、三种叫法，加上页头一共三个实心蓝按钮（U4 · 交互、视觉 · 可挽回 0.15）

- 位置：[U4-quota-failure-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r4/U4-quota-failure-1440-light.png)：
  - 页头「Save locally」实心；
  - 红色 callout「Retry save」实心（y≈472–503）；
  - 命名卡「Retry retained saves · “Retained name”」实心（y≈674–706）；
  - 操作区「Retry retained saves」描边（y≈809–840）。
  - 此时只有 1 个 pending。
- 现状 → 期望：三个按钮做同一件事、叫法各不相同；callout 已说「new saves stay in memory」，页头 Save locally 仍是实心。期望：S4 命名动作单独反馈；S1 一个明确的继续动作；状态 callout 只管存储层。
- 修法：
  - callout 去掉「Retry save」，由「Download current theme」做唯一的实心按钮；
  - 命名卡按钮改为「Retry “Retained name”」；
  - 操作区改为「Retry all retained saves」（描边）；
  - 页头在失败态降为描边。
- 来源：U4 新-1。评审注明这是 R3 UI-51、UI-140 两条修法叠加造成的。

### UI-128 · [P2] 〔停滞预警〕Source 行的值排在「Current」列下，读起来像是当前主题的属性（U3 · 交互、细节 · 部分解决 · 可挽回 0.1）

- 位置：[U3-import-review-1440-light.png](DOM-COMPONENTS-P1.ui-captures/r4/U3-import-review-1440-light.png) 行 y≈627–636。「Theme JSON v1 · saved as v2」从 x=405（Current 列）起排，Imported 列为空。图例「Field | Current | Imported」与 Name 两侧的值已经补上；390 单列卡片没有这个歧义。
- 现状 → 期望：版本归属不清 → X1 版本清楚归属于导入候选。
- 修法：
  - Source 行的 Current 格写当前文档格式，Imported 格写「Theme JSON v1 · saved as v2」；
  - 或者把 Source 改为表格上方跨列的说明行「Imported file: Theme JSON v1 · will be saved as v2」；
  - 中文同步。
- 停滞预警：本条按重构轮处理，与 UI-198、UI-199、UI-25、UI-176 一起把候选区并入 Changes 的表格组件。只改 Source 一行的文案不算重构。
- 来源：U3 UI-128（部分解决）。

### UI-212 · [P2] 200% 文本下 DOM 预览场景被裁：标题断在字中间，「Create · 0」完全在可视区外，卡片挤成约 65px（U1/U5 · 布局 · 主 agent 核实）

- 位置：[MC-root200-preview-clip-720-light.png](DOM-COMPONENTS-P1.ui-captures/r4/MC-root200-preview-clip-720-light.png)。条件同 U1 · 200%：720×900、根字号 200%、渲染器 DOM。只纵向滚动预览列，预览面板自身的 scrollLeft 保持 0。
- 现状（页面实测，[machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup2.json)、[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check.json) `zoom200.preview`）：
  - `section.preview-surface` 宽 236px，内容宽 389px，`overflow-x: auto`；
  - 标题「Build something that feels yours」在 x=639 处被切成「somethin|g」；
  - 「Create · 0」在 x 677–791，与可视区交集为 0；
  - 「Disabled」同样在面板右界之外；
  - Small、Medium 等卡片挤成约 65px 宽，「Large · shadow-lg」的文字溢出卡片边。
  - 400px 的预览列里有三层嵌套边框和内边距，留给场景的只有 238px。
- 期望：
  - G6「200% 放大下主路径可达，不依靠整页横向滚动到达关键动作」。这里不是整页滚动，但预览里的真实操作要靠一个看不见滚动条的内层横向滚动才能看到；
  - P1 预览的真实操作可用；
  - P5「自动宽度跟随可用空间」。
- 严重度按 P2 计：编辑、保存、导出等编辑器主路径不受影响，受影响的是预览场景的可读和可操作。
- 修法：
  - 预览场景在窄容器下重排：头部 `flex-wrap: wrap`，主按钮可换到标题下方；卡片网格用 `repeat(auto-fill, minmax(12rem, 1fr))`；
  - 窄宽度下（按预览列容器查询）去掉或收小嵌套的边框与内边距，让场景拿到预览列的大部分宽度；
  - 确实需要横向滚动时，给出可见的滚动提示。
- 来源：主 agent 核实。R3 报告把它留作本轮的补拍线索，本轮在运行页面上实测并实拍确认。冻结矩阵的 U1 · 200% 格只拍到预览工具栏，评审没有看到这一块。主 agent 核实的差距不改分数。

### UI-198 · [P3] 同一对话框里两张差异表的列结构、列位置和表头样式不一致（U3 · 布局、视觉、细节 · 可挽回 0.15）

- 位置：[U3-import-review-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U3-import-review-1440-light.png) y 595–607：候选区表头 12px 常规、无底线，列 x 233/405/812。[U3-import-review-bottom-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U3-import-review-bottom-1440-light.png) y 252–274：Changes 表头 13px/600、#D0D5DD 底线，列 x 241/572/728/971，另有独立的 Change 列。390 下两处小标题字重也不一样。
- 期望：G3 label 13px/600；X1 用同一套结构呈现。
- 修法：候选配置行改用 Changes 的表格组件，两表共用 `Field | Change | Current | Imported` 的列定义与 `th` 样式；或并成一张表，「Identity & configuration」作第一个分组行。
- 来源：U3 新-1。

### UI-194 · [P3] 单列网格仍沿用「配对行」结构：来源块没有紧跟所选格，配对内外间距倒置（U2 · 布局、细节 · 可挽回 0.15）

- 位置：[U2-source-overridden-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U2-source-overridden-1440-light.png) primary 格（y=342–418）之后先是 primary-foreground 格（y=425–502），再是来源块（y=535 起）。U2-token-released、U2-source-overridden-390 同样。card-foreground → popover 间距 4px，popover → popover-foreground 6px。
- 期望：F1、F2 来源说明直接附在所选 token 下；G2 用间距表达分组。
- 修法：拆掉配对行容器（或 `display: contents`），让 token 格成为列表的直接子项；来源块与 Color controls 插在所选格之后。要保留配对，就让配对内 4px、配对间 12px。
- 来源：U2 新-1（评审注明这是 UI-163 单列修法的副作用）。

### UI-176 · [P3] 徽标基线偏低 2px；390 候选区徽标缩进 8px；Name 行两边基线差 1px（U3 · 布局、细节 · 部分解决 · 可挽回 0.14）

- 位置：
  - [U3-import-review-bottom-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U3-import-review-bottom-1440-light.png) 中 id、name、primary 三行的文字与徽标基线分别为 332/334、373/375、451/453；
  - [U3-import-review-390-light](DOM-COMPONENTS-P1.ui-captures/r4/U3-import-review-390-light.png) Name 行徽标在 x=207，同列的值在 x=199；
  - U3-import-review-1440-light Name 行两个值的基线为 665/666。
  - 徽标列对齐与表头双线已经修好。
- 修法：徽标 `inline-flex` 居中、固定高 24px；同行 `td` 统一 `vertical-align`；值与徽标包进 `flex-wrap` 容器，去掉徽标的 `margin-left`。
- 来源：U3 UI-176（部分解决）。

### UI-02 · [P3] EN/中文、Edit/Preview 段间仍画两条线，与 Light/Dark、DOM/GPU 的单线画法不一致（U1 · 视觉、细节 · 部分解决 · 可挽回 0.1）

- 位置：
  - [U1-default-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U1-default-1440-light.png) y=85：x=250 为 #84CAFF，x=251 为 #D0D5DD；
  - U1-zh-top-1440-light 相反；
  - U1-edit-390-light、U1-preview-390-light 在 y≈140–145、x=61/62 同样。
  - 对照：Light/Dark 接缝只有 x=993 一列；390 的渲染器接缝也是单列。焦点部分已经解决（Dark 的焦点环内收）。
- 修法：EN/中文与 Edit/Preview 复用 Light/Dark 的分段组件；或者非首段 `margin-inline-start:-1px`、选中段 `z-index:1`。
- 来源：U1 UI-02（部分解决）。

### UI-193 · [P3] 中文态预览诊断标题字重混排：汉字常规，数字与拉丁字符半粗（U1 · 视觉、细节 · 可挽回 0.1）

- 位置：[U1-zh-top-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U1-zh-top-1440-light.png)、U1-zh-configured-1440-light，「▼ 2 组文字对比低于 4.5:1 · 作者颜色保持不变」，x≈357–622、y≈210–222。英文同一元素整句半粗；中文壳层标签能正常加粗。
- 期望：G6 中英文同步；G3 label 600。
- 修法：诊断条标题用与壳层标签相同的字体栈与 CJK 回退，回退字体要有 600 字面或允许合成粗体；不要继承预览作用域的作者字体。
- 来源：U1 新-1。

### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（U2 · 交互与状态 · 仍存在 · 可挽回 0.1）

- 位置：[U2-group-reset-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U2-group-reset-1440-light.png)，Neutral「Light 15 · Dark 15」（y=644–667）、Panel「Light 2 · Dark 2」，都没有列表或展开入口。
- 期望：F2「解除将影响哪些项」。
- 修法：计数做成 `<details>` 的 summary，展开后按模式列出 token 名；或者加「Show in list」，把网格筛到「Overridden only + 该组」。
- 来源：U2 UI-172（仍存在）。
- 核对说明：本轮截图中，U2-group-reset-1440-light 与 U1-configured-lower-1440-light 的编辑列都只拍到 Scaling 及以上，各组计数旁确实没有展开入口。主 agent 在运行页面上实测（[machine-check-followup3.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup3.json)）：界面有一个默认关闭的「Tokens affected by a group reset」disclosure，位于整个基础配置区之后、Scaling 选择器下方 63px，不在各组计数旁。本轮冻结矩阵里没有一帧拍到它，所以评审无从判断它是否满足 F2。请修复方决定：是把影响清单挪到各组计数旁（评审给出的修法），还是保留现位置；后者需要在下一轮的 U2 · 分组解除采集里展开这个 disclosure 再拍（只改采集节），由评审复核。

### UI-195 · [P3] 390 下吸顶的 Edit/Preview 条没有下分隔线，「Semantic colors」组标题被切成半截（U2 · 布局、细节 · 可挽回 0.1）

- 位置：[U2-source-overridden-390-light](DOM-COMPONENTS-P1.ui-captures/r4/U2-source-overridden-390-light.png)。标签在 y=136–167，内容从 y≈176 露出，「Semantic colors」只剩 y=176–182 的下半部；吸顶条与内容之间没有分隔线。
- 期望：G2 各区独立滚动、边界清晰；G1。
- 修法：窄屏吸顶条加 1px #EAECF0 下边框。所选格是组内首格时，滚动组标题，`scroll-margin-top` 取吸顶条实测高度 + 8px。
- 来源：U2 新-2（UI-17 修法之后出现）。

### UI-199 · [P3] 6 个「Unchanged」与 1 个「Changed」徽标同样式，且左缘参差（U3 · 布局、视觉、交互 · 可挽回 0.1）

- 位置：U3-import-review-1440-light y 651–866、U3-import-review-mid-390-light y 33–346。两种徽标底色、边框、字色都相同；Unchanged 徽标的左缘有 456、461、451 三种 x。
- 期望：X1 突出候选差异。
- 修法：状态放进独立的 Change 列并左对齐；「Unchanged」改为纯文字 caption（或写「—」）；只有 Changed、Added、Removed 保留带边框的徽标，不染蓝。
- 来源：U3 新-2。

### UI-203 · [P3] callout 和命名卡之间成句重复（U4 · 细节 · 可挽回 0.1）

- 位置：
  - [U4-upgraded](DOM-COMPONENTS-P1.ui-captures/r4/U4-upgraded-1440-light.png)：「All saved themes and your current draft were retained.」后面紧跟「2 saved themes retained. …」；命名卡写「not saved earlier.」；
  - U4-no-locks：标题、正文、命名卡三处说同一件事；
  - U4-quota-failure：标题与正文末尾都写「new saves stay in memory」。
- 修法：每件事只说一次，照原文给出的改写；无锁态的命名卡可以按 UI-205 并入 callout。
- 来源：U4 新-3。

### UI-196 · [P3] 所选格里的模式文字「Light」在选中时变粗（U2 · 视觉、细节 · 可挽回 0.08）

- 位置：U2-source-overridden-1440-light，选中的 primary 格「Light」（x=122–148，y=394–406）暗像素 94 个，未选中的格是 60 个；390 同样。
- 期望：G3 选中态只改背景、边框与 token 名颜色；C3 状态标识形态一致。
- 修法：去掉选中态作用于模式后缀的 `font-weight`，固定为 12px/400 #475467。
- 来源：U2 新-3。

### UI-200 · [P3] Import notes 把整条路径排成等宽字，与表格写法不一致（U3 · 视觉、细节 · 可挽回 0.08）

- 位置：U3-import-review-bottom-1440-light y 764–794、-390 y 664–740。「Light · base · font-serif」整段等宽，比同行 14px 正文小一号；表格里写成无衬线的分组头加等宽 token。
- 修法：模式与分组用正文字体，只有 token 用 `code`，`code` 字号设为 .9286em；中文同步。
- 来源：U3 新-3。

### UI-18 · [P3] Colors 页节标题、分组标题仍是旧样式（U2 · 视觉 · 部分解决 · 可挽回 0.07）

- 位置：
  - U2-source-overridden-1440-light：「Fine adjustment source」为 14px/600 #475467，「Semantic colors」为 13px/600 #475467；
  - U2-source-overridden-detail：「Color controls」同样；
  - U2-source-generated：「Borders & focus」同样。
  - 对照：其他页的节标题为 14px/600 #1D2939。
- 修法：所选 token 的详情节标题统一为 14px/600 #1D2939；列表分组改为眉题（12px/600、大写、字距 .04em、#475467）。
- 来源：U2 UI-18（部分解决）。

### UI-25 · [P3] Source 行比其余候选行矮 3px（U3 · 布局、细节 · 部分解决 · 可挽回 0.06）

- 位置：U3-import-review-1440-light，label 大写字母顶边依次为 596/628/657/689/720/752/784/816/848，只有 Source→Name 是 29px。
- 修法：候选行设 `min-height: 2rem`，不靠徽标撑高；并表后统一 `td` 高度。
- 来源：U3 UI-25（部分解决）。

### UI-115 · [P3] 网格 ↺ 仍只有图标（U2 · 交互与状态 · 仍存在 · 可挽回 0.05）

- 位置：U2-source-overridden-1440-light、-390 各格右侧 x=273–297 的 ↺ 段。aria-label 与 title 在截图上看不到，本轮也没有悬停或聚焦帧。
- 修法：可解除的 ↺ 加与来源块按钮同文案的 `aria-label` 和 `title`；不可解除的设 `aria-disabled`；聚焦时在状态行显示范围；补拍悬停帧与聚焦帧。
- 来源：U2 UI-115（仍存在，可见部分）。
- 核对说明：修复记录附了键盘聚焦帧 `r3-apply/confirm/APPLY-reset-focus-1440-light.png`，但冻结矩阵里没有这一格。下一轮建议把「↺ 键盘聚焦」并入 U2 的采集步骤（只改采集节），评审才能复核。

### UI-197 · [P3] 诊断区两个同级 summary 一个带计数、一个不带（U2 · 细节 · 可挽回 0.05）

- 位置：U2-low-contrast-1440-light，「Fonts · 3」（y=154–164）与「Text contrast pairs」（y=345–357）。
- 修法：两者统一为「名称 · 总数」，或者都去掉计数。
- 来源：U2 新-4。

### UI-202 · [P3] 配额 callout 标题折行后第二行只剩孤词，也没有写出失败原因（U4 · 细节 · 可挽回 0.05）

- 位置：U4-quota-failure / U4-quota-operations，标题「Local save failed · new saves stay in」/「memory」（y=344/362）；正文写「A later library action failed」。
- 期望：S6 写明原因；S4 点名失败的动作。
- 修法：标题改为「Storage full · saving in memory」；正文点名「Saving “Retained name” failed and is kept for retry.」；标题加 `text-wrap: balance`。
- 来源：U4 新-2。

### UI-204 · [P3] 冲突面板比 900px 视口高，默认帧切掉了「Download other version」（U4 · 布局 · 可挽回 0.05）

- 位置：[U4-conflict-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U4-conflict-1440-light.png)，面板从 y=323 起，「View stored text」在 y=867，下一个按钮只露出顶边。底边对齐需要 `scrollTop 35→335`，R3 是 `0→300`。
- 期望：S1 出口始终可达；S5 展示冲突版本。
- 修法：首句与第一段补充说明并为一句；删掉单独的「No pending actions…」段，有 pending 时再用一行 caption。
- 来源：U4 新-4。

### UI-205 · [P3] 取消态和无锁态的命名卡让用户去做的事，按钮在另一张卡里（U4 · 交互 · 可挽回 0.05）

- 位置：U4-upgrade-cancelled 命名卡（y≈580–711）、U4-no-locks 命名卡（y≈491–622）都只有「Dismiss」。
- 期望：S1 每条常驻反馈带继续动作；S4。
- 修法：命名失败的原因就是 callout 描述的状态时，并入 callout 末尾一行「Not saved: “…” · Dismiss」；或者在卡里放对应的描边按钮。
- 来源：U4 新-5。

### UI-76 · [P3] DOM 与 GPU 两框首行纵向差 9px（横向已对齐）（U5 · 布局 · 仍存在 · 可挽回 0.05）

- 位置：[U5-gpu-compare-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U5-gpu-compare-1440-light.png)。两框上边框都在 y=335；DOM 首行大写字母顶部在 y=357，「GPU · Cards」在 y=366。横向内缩已对齐。R3 时纵向是对齐的、横向差 8px。
- 修法：两框用同一个 padding token，按首行基线对齐调整 GPU 场景根的 `padding-top`，不动左右边距。
- 来源：U5 UI-76（仍存在：横向已解决，纵向回归）。

### UI-208 · [P3] 预览对话框 Confirm 为 79×35，与同一预览里 32px 高的 solid 按钮不一致（R3 为 77×33）（U5 · 细节 · 可挽回 0.05）

- 位置：U5-dialog-1440-light、U5-local-opposite-dialog-1440-light 中 Confirm 在 x 233–311、y 118–152；U5-dialog-390-light 为 79×35。对照：Create profile 为 115×32。
- 期望：P1 真实组件与尺寸；Button size 2 参考高 32px。
- 修法：对话框操作按钮用与 Create profile 相同的 Button size 2，去掉多出的那 1px；与 UI-207 一起排查。
- 来源：U5 新-2。
- 主 agent 核对：页面实测 Confirm 高 35px、宽 79.3px（machine-check-followup.json），与截图一致。

### UI-209 · [P3] 卡片选中框内圈在滚动区边缘画出闭合底边，看上去像卡片在 y=720 处结束（U5 · 细节 · 可挽回 0.05）

- 位置：U5-inspector-card-1440-light。外圈向下延伸到滚动区边缘、不闭合；内圈在 y 719–720 画出底边（x 369–867）。这张卡实际延伸到 y≈787。
- 期望：P2 选中矩形如实框出实际部件。
- 修法：两圈都按部件完整矩形绘制，覆盖层放进预览滚动容器并 `overflow: clip`；或者钳制时去掉超出一侧的描边，内外两圈同样处理。
- 来源：U5 新-3。

### UI-210 · [P3] 深色下 classic 底边暗线变浅（UI-81 修复引入的回归）（U6 · 视觉 · 可挽回 0.05）

- 位置：[U6-fixture-1280-dark](DOM-COMPONENTS-P1.ui-captures/r4/U6-fixture-1280-dark.png) classic 1–4 的底边。以 classic 4 在 x=490 为例：末行 #425AAA 压在 #4F70E0 上，约 −20%；目标约 −28%；R3 时末行是 #334DA3，约 −31%。
- 期望：radix-sink-button.png、radix-classic-button.png 的 classic 行末行压暗约 28%。
- 修法：只改 classic 深色阴影层。让侧边亮边不作用到底边，或者把底部暗线排到 `box-shadow` 列表最前面，使末行亮度约为上一行的 0.72 倍；保住已对齐的上缘与两侧增量；改完运行 `mbx xtask css`。
- 来源：U6 新-1。
- 主 agent 核对：r3 与 r4 同一像素（x=490，y=184）分别为 #334DA3、#425AAA，y=182–183 两轮相同。

### UI-189 · [P3] Reset 的 input、textarea 虚线标记越过内容右沿（U7 · 细节 · 部分解决 · 可挽回 0.05）

- 位置：
  - U7-fixture-1280-light 中两行虚线框右缘在 x=1229，Container 分隔线止于 x=1224；
  - U7-fixture-390-light 中右缘在 x≈363；
  - U7-layout-reset-comparison-1280/390 中右边线被裁掉。
  - 脱离值列的问题已修复。
- 修法：值列单元格加 `padding-inline-end: 5px`，或者块级 Reset 元素的标记改为 `outline-offset: -1px`；不加包裹层（L4）。
- 来源：U7 UI-189（部分解决）。

### UI-211 · [P3] 240px 显式宽度样例的正文贴着左边框（U7 · 细节 · 可挽回 0.05）

- 位置：U7-fixture-1280-light 第 1628–1651 行，「A bad prop keeps this view alive.」首字距左边框约 1px；U7-invalid-detail-1280、95%、110%、390 帧同样。
- 修法：样例 Box 加显式 `padding: 2px 8px`、`box-sizing: border-box`，标注与盒之间留 4px。
- 来源：U7 新-1。

### UI-56 · [P3] 操作展开内容：说明段没有缩进，列表已缩进（U4 · 布局 · 部分解决 · 可挽回 0.03）

- 位置：U4-quota-operations，说明段（y≈805–841）从 x=21 起，列表从 x=34 起。#3 行在折线以下，修订号是否断行看不出。
- 修法：details 的内容容器整体缩进到摘要文字的起点；修订号片段 `white-space:nowrap`。
- 来源：U4 UI-56（部分解决）。

### UI-206 · [P3] 草稿差异块有三条左缘（U4 · 布局 · 可挽回 0.02）

- 位置：U4-conflict-diff-1440-light，三条左缘：x=33/34（callout 正文、展开三角、分隔线）、x=42（差异行）、x=49（摘要文字）。
- 修法：差异行去掉 8px 的 `padding-inline`，与分隔线同起点；或者整体缩进到 x=49。与 UI-56 用同一条规则。
- 来源：U4 新-6。

### UI-160 · [P3] 颜色搜索框与 Clear 不等高（U1/U2 · 布局、细节 · 待核实）

- 状态：U1 评审判已解决（[U1-field-tabs-1440-light](DOM-COMPONENTS-P1.ui-captures/r4/U1-field-tabs-1440-light.png) 中搜索框与 Clear 都在 y=536–567，都是 32px）；U2 评审判待核实（U2 各帧中搜索行被选中后的滚动移出了视口）。按复核顺序取待核实。
- 修法：无需改动界面。下一轮在 U2 的采集里加一帧「Colors 页搜索行」（只改采集节），由评审复核。
- 来源：U1 UI-160、U2 UI-160。

## 待裁定事项

无。

- 本轮没有停滞，只有停滞预警：UI-128 所在的 U3 尚未做过重构轮。下一轮修复须对 U3 做一次重构轮；重构后仍满足预警条件时，U3 判停滞，UI-128 转为待裁定事项。
- 修复记录里，R3 → 修复的 64 条最终处置都是已修改待评审，没有不采纳、待决定或阻塞的差距。
- 目标图与规范之间没有新的冲突。

## 建议（不扣分）

汇总自各评审，原文见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r4/judge.md) 的「建议」节。下面列出跨单元重复出现、影响较大的几条：

- **实心蓝按钮的数量（U1、U4）**：「Download current theme」本轮改为实心，与页头「Save locally」两个实心蓝按钮同屏。G3 写的是「蓝色用于选中与焦点」，但前几轮从未就「Save locally」扣分，为保持尺度一致只作建议。配额态的同类问题已列为 UI-201。
- **编辑列顶部存储状态块（U1、U2）**：本轮变高，1440 首屏的「Presets」从 y=550 下移到 y=618。可以压成一行摘要加展开（R3 已提）。
- **按钮字重（U1、U5）**：UI-188 的修法让切换类按钮常驻半粗，同一行的选择器和「Back to edit」仍是常规。按类统一，并用伪元素预留粗体宽度。
- **导出代码（U3）**：JSON 导出格式化，代码框改为 `white-space: pre` 加横向滚动；390 下键名和字符串会在中间断行（R2、R3 已提）。
- **U6 夹具补齐矩阵**：IconButton 各变体与尺寸，各变体的 high-contrast、gray、disabled，Link 的字级与 underline 模式。补齐后，「与目标一致」的结论才能覆盖整张目标图（R3 已提）。
- **「Caller utilities」示例改为可读配色**：axe 测得 1.24:1，与 R1–R3 相同。

## 机器检查

全部在运行中的 release 页面上执行，在 7 位评审交卷之后进行，结果没有交给评审。

| 检查 | 工具与版本 | 结果 | 对应差距 |
| --- | --- | --- | --- |
| 控制台错误、失败请求、pageerror | Playwright 1.63.0 / Chromium 153.0.8010.12 事件监听；采集 2 次运行与机器检查各会话 | 均为 0 | — |
| CSP 违规 | 页内 `securitypolicyviolation` 监听，strict CSP 服务 | 0（采集 43 个页面、机器检查各会话） | — |
| 横向溢出 | 页内测量 `scrollWidth` 与越界元素 | 页面级全部为 0；200% 列内溢出见下方「200% 复核」 | UI-192 已解决；新增 UI-212 |
| 对话框是否在视口内 | 页内 boundingBox | 导出对话框：1440 下 x208、y16、w1024、h528；768 下 x16、w736；390 下 x16、w358、h558。都在视口内 | — |
| 可访问性扫描 | @axe-core/playwright 4.13.0，wcag2a/2aa/21a/21aa/22aa | 见下方「axe 结果」。incomplete 项未逐项人工复核 | 无新增 |
| 语言切换点击区 | 页内 boundingBox + `elementFromPoint` | EN 39×32、中文 48×32，中英两态中心都命中自身 | — |
| 键盘走查（真实按键） | Playwright keyboard | 见下方「键盘走查」 | — |
| B2 焦点样式 | 键盘聚焦后读计算样式（Shift+Tab、Tab） | 见下方「B2 焦点样式」 | 与 U6 评审结论一致，无新增 |
| 字面加载 | catalog 布局夹具页内计算样式 + `document.fonts` | Text 四行计算字重 300/400/500/700，Heading（h2）与 Strong 为 700。Studio 页没有做此项；UI-193 的中文诊断标题字重只依据截图 | — |
| 200% 吸顶头区 | 页内测量 + 滚轮 | 编辑列内没有 sticky 或内滚动区；「Preview mode」在 y=30–69；在头区和下方滚轮，编辑列 scrollTop 依次为 0→200→600 | — |
| 预览对话框作用域（UI-207） | 页内读取计算样式与作用域属性 | 见下方「预览对话框」 | 佐证 UI-207、UI-208 |
| 性能 | — | 未检查。本轮 UI 验收不测性能，没有新增毫秒 SLO | — |

**200% 复核（720×900 + 根字号 200%）**

- 首屏：`#studio-editor` 319 / 319，`.studio-previews` 400 / 400（R3 为 398 / 319、405 / 400）。
- 渲染器分段：DOM、GPU、DOM + GPU 分别在 x 352–443、442–529、528–688，都在预览列内（预览列 320–720）。
- 精调 Colors：编辑列 319 / 319，scrollLeft 0；token 格单列，宽 229，格内 scrollWidth 等于 clientWidth。
- 选中 primary：来源块在 x 32–287；有效值 `#1570ef` 在 x 32–133；「Clear primary · Light only」在 x 32–287；块内没有横向溢出的子元素。据此 UI-192 判已解决。
- 导出对话框 Output：显示「Theme JSON v2」，宽 558，scrollWidth 556 等于 clientWidth 556。
- DOM 预览：`section.preview-surface` 宽 236、内容 389、`overflow-x: auto`。「Create · 0」在 x 677–791、「Disabled」在 x 650–757，都在面板右界 639 之外。实拍确认后列为 UI-212。

**axe 结果**

- Studio 1440、768、390 编辑与预览、导出对话框：0 个违规节点。
- Studio 1440 DOM 预览：1 个节点，Code soft「ThemeDocument」3.89:1（#1570EF 对 #E3EEFD），属 U5 已接受口径（Code soft 用 primary 压其浅色底）。
- catalog 按钮夹具（1280、390 各 19 个节点）：与 R3 相同，都是 UI-100 已接受口径（色族 solid 白字压第 9 阶，以及 Accent pair 与 Semantic accent 2.62:1），加上「Caller utilities」示例，按 R1–R3 做法归入建议。
- catalog 布局夹具：1 个节点，Code solid gray 3.31:1，属 U7 已接受口径。
- 逐节点清单见 [machine-check.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check.json)。

**键盘走查**

- Studio 从文档起点 Tab 70 次，焦点都在屏内且有可见指示。
- 导出对话框 Tab 25 次、导入对话框 Tab 15 次，焦点都留在对话框内；Esc 关闭后焦点回到触发按钮。
- 品牌色选择器：Enter → ↓↓ → Enter，从 Custom 改为 Gold，保留 3px rgba(21,112,239,.4) 焦点环。
- catalog 按钮夹具 Tab 61 次，焦点都可见。

**B2 焦点样式**

- 浅色 12 个、深色 7 个按钮都是 2px solid、第 8 阶色：
  - classic、solid、red、gray、High contrast、IconButton、Unavailable 的 offset 为 2px；
  - soft、surface、outline、ghost-offset、ghost 的 offset 为 −1px；
  - 浅色默认 rgb(141,164,239)，深色默认 rgb(67,93,177)，red rgb(235,142,144)，High contrast rgb(94,177,239)。
- Link：Tab 后立即读取，读到的是 0.15s 过渡的中间值（3px、offset 0），与 R2、R3 相同。等 1s 后复测，两条链接都是 2px solid rgb(141,164,239)、offset 2px、圆角 1.26px 与 1.05px、下划线隐藏（[machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup2.json)）。

**预览对话框（UI-207、UI-208）**

| 边界外观 | 对话框层 | Confirm 作用域 | Confirm 自身的 `--primary` | Confirm 背景 / 文字 | Confirm 尺寸 |
| --- | --- | --- | --- | --- | --- |
| Opposite to root | `data-theme="dark"` | `data-appearance="dark"` | rgb(83 177 253) | rgb(21,112,239) / rgb(255,255,255) | 79.3 × 35 |
| Inherit root | `data-theme="light"` | `data-appearance="light"` | rgb(21 112 239) | rgb(21,112,239) / rgb(255,255,255) | 79.3 × 35 |

同一边界内「Actions」的文字色，深色时为 rgb(83,177,253)，浅色时为 rgb(21,112,239)，都与各自的 `--primary` 一致。只有对话框里的 Confirm 背景与自身作用域的 `--primary` 不一致。

原始结果：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check.json)、[machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup.json)、[machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup2.json)，脚本为同目录同名 `.cjs`。[machine-check-followup3.json](DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup3.json) 是 UI-172 核对说明用的一次页内只读查询，结果与条件写在文件里。

## 采集与覆盖

### 环境与被测版本

- HEAD `5d122d6231c690a61080b91c680e9a514d2034b8`（main，dirty）。产品路径（crates、examples、xtask、web、makepad、public、Cargo.toml、Cargo.lock、sources.lock.json）相对 HEAD 无 diff，也没有未跟踪文件。tracked diff 只含 SDLC 台账与两张旧验证 PNG。
- 本轮从 HEAD 重建两个示例的 release（`mbx xtask build-web --release`）：
  - Studio `28ae38531384c902`，WASM 9,893,303 B；
  - catalog `32b52b66f04701ad`，WASM 8,203,814 B。
- 服务：`mbx xtask serve --dir <产物副本>`，strict CSP，Studio 端口 4181，catalog 端口 4176。采集前用 `curl …/build-manifest.json` 确认服务发出的正是这两个 build id。
- 服务改用 `--dir` 的原因：本轮 mbx 两次回收了托管的 `target/` 目录，一次在构建之后、一次在采集途中，`serve --example` 随即对 `/` 返回 404。产物副本是构建完成后原样复制出来的目录；重建三次，build id 都相同。

### 采集

- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，脚本 [capture-runner.cjs](DOM-COMPONENTS-P1.ui-captures/r4/capture-runner.cjs)。
- 条件：DPR 1，B2 焦点特写为 DPR 2；`reducedMotion: reduce`；等待字体就绪、网络空闲、500ms 无长帧。
- 数量：首采 140 张，补采 2 张，共 142 张，0 失败。140 张视口图逐张核对，像素尺寸都等于 CSS 视口 × DPR。
- 与 R3 采集的差异（只改了目标包「采集」节，并记入变更记录，目标指纹不变）：
  - R3 的补采帧并入首采；
  - Rust 中段直接滚到代码纵向中点；
  - 「Complete … JSON」按界面现名「Complete library upgrade JSON」选取；
  - U1 · 200% 增拍精调 Colors 网格与所选 token 的来源块。
- 补采：`U1-zoom200-source-720-light` 首拍时，界面自身的选中滚动在脚本滚动之后发生，来源块落到折线以下。原图保留、不作证据。另拍 `-a2`（先等页面静止再滚动），并因为来源块高于视口，再拍 `U1-zoom200-source-end-720-light`。
- 中断的两次运行保留在 [aborted-run1/](DOM-COMPONENTS-P1.ui-captures/r4/aborted-run1/)（69 张）与 [aborted-run2/](DOM-COMPONENTS-P1.ui-captures/r4/aborted-run2/)（48 张），不作证据。原因见上文「环境与被测版本」；第二次是因为旧服务进程没有停干净，仍占着端口。
- 逐图登记见 [capture.md](DOM-COMPONENTS-P1.ui-captures/r4/capture.md)。

### 目标文件 hash

| 文件 | SHA-256 前缀 |
| --- | --- |
| 目标包（含 R4「采集」节两条变更记录） | `7b859bd61692` |
| 13 张目标图 | 与 R1–R3 相同，本轮逐张复算一致（清单见 [target-files.txt](DOM-COMPONENTS-P1.ui-captures/r4/target-files.txt)） |

- 目标指纹：`manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`，算法见 [target-fingerprint.py](DOM-COMPONENTS-P1.ui-captures/r4/target-fingerprint.py)。与 R3 相同，本轮不重建基线。

### 未覆盖范围

- 冻结矩阵的全部格子都已采到。
- 评审报告的看不出项，都不是 P0/P1 待核实，本轮没有补采重评：
  - **UI-160**：U2 各帧没有搜索行，但 U1-field-tabs 已拍到这一行，U1 评审判已解决。下一轮在 U2 采集里加一帧搜索行（只改采集节）。
  - **矩阵外的状态**：各单元的悬停、键盘焦点帧（U1 的除外）；U2 拾色焦点与 Locate 点击结果；U3 的中文、下载与失败路径、语法错误、超限与不支持颜色；U4 的排队、待锁、带 pending 的冲突、待确认时切模式；U5 浮层内拾取、键盘打开菜单、GPU 失败与中文 GPU 文案、宽度档；U6 夹具未提供的矩阵与深色 loading；U7 Restore prop 之后、390 下的缩放与非法态。
  - 把这些加入矩阵要改目标包的「单元」节，需要用户确认，之后按重建基线评审。
- 截图来自 headless Chromium，不代表 Firefox、Safari、真机、人工读屏或 OS IME。

下一轮的补拍线索（本轮不计差距）：

- 在 U2 采集里加「Colors 页搜索行」一帧，用于复核 UI-160；加「网格 ↺ 键盘聚焦」一帧，用于复核 UI-115；分组解除后展开「Tokens affected by a group reset」再拍一帧，用于复核 UI-172。
- 在 U1 · 200% 的采集里加「DOM 预览场景」一帧（只纵向滚动），让评审也能看到 UI-212 所在的区域。

### 环境噪声

- `U4-upgrade-review-end-1440-light` 与 `U4-upgrade-review-1440-light` 字节相同，因为对话框在 900px 高内不需要滚动（U4 评审核对 md5）。
- `U6-links-hover-1280-light` 与 `U6-links-1280-light` 逐像素相同（U6 评审差分为空）；U6 有 21 张图与 R3 字节相同。
- U5 三张拾取未声明部件的帧（hover、点击、面板滚入）SHA-256 相同，因为本轮面板已固定在预览列底部，「把面板滚入视野」这一步是空操作。
- 同一版本重拍没有出现不一致的结果（三次运行 build id 相同）。

### 评审方式

- 共派 7 位全新上下文的只读评审，每单元一位，并行。模型继承主 agent。
- 评审只拿到目标包、目标图、本单元的 r4 与 r3 截图、两轮 capture.md、两轮 actions.log，以及 R3 judge.md 中本单元的摘录（[judge-inputs/](DOM-COMPONENTS-P1.ui-captures/r4/judge-inputs/)）。没有拿到代码、计划、修复记录、门槛、机器检查结果或其他评审的结论。
- 这是提示里给的白名单，不是文件权限沙箱。评审交卷时，r4 目录里还没有评审原文和机器检查结果；提示也要求不打开 r4 的子目录。
- 没有补采重评，没有确认评审（本轮判「需要修改」）。

### 主 agent 核对说明

- **事实抽查**：开图或取样核对了以下评审描述，均属实：
  - UI-207：r3 与 r4 的 `U5-local-opposite-dialog-1440-light` 中，Confirm 在 (250,135) 处分别为 #53B1FD 与 #1570EF，两轮表面都是 #1D2939；页面实测见上文「预览对话框」；
  - UI-210：r3 与 r4 的 `U6-fixture-1280-dark` 在 (490,184) 处分别为 #334DA3 与 #425AAA；
  - UI-201：`U4-quota-failure-1440-light` 编辑列有两段实心 #1570EF（y 472–503、674–705），页头 y=16 处也有 #1570EF；
  - UI-128：`U3-import-review-1440-light` 中「Theme JSON v1 · saved as v2」在 Current 列下，Imported 列为空；Unchanged 与 Changed 徽标同样式。
- **核对说明**：UI-172、UI-115 两条附在差距下，请修复方核实。
- **页面实测**：UI-192 的复核与 UI-212 的发现写在机器检查与对应差距下。主 agent 没有改分、删差距或改门槛项。
