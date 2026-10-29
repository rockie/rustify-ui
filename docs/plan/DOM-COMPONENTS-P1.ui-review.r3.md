# DOM-COMPONENTS-P1 · UI 验收评审 R3

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R3（全量；重建基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [DOM-COMPONENTS-P1.ui-target.md](DOM-COMPONENTS-P1.ui-target.md) · `567117725f56` | HEAD `d5775d8a`，dirty，产品路径无 diff（tracked diff `b04d5427f42e`，只含 SDLC 台账与两张旧验证 PNG）；Studio `80cad3c530b54d9f` http://127.0.0.1:4181/ ，catalog `7ea7f541a819b220` http://127.0.0.1:4176/ | [r3](DOM-COMPONENTS-P1.ui-captures/r3/) | [R2](DOM-COMPONENTS-P1.ui-review.r2.md) |

## 验收判定

**需要修改**

- 本轮分数 8.6（U4），门槛 8（沿用 R1、R2，默认值）。历史最佳 8.6，本轮重建基线，从本轮起算。
- 未关闭的门槛项 1 条，没有 P0：UI-192（P1，机器检查），即 200% 文本下精调 Colors 网格与来源块丢失内容。
- 停滞：重建基线后的第一轮，不判停滞。没有待裁定事项。

七个单元的分数都不低于 8.6，评审来源的差距里已没有 P0/P1。R2 的四条 P1 都已解决：UI-101 200% 头区，UI-152、UI-88、UI-89 字重。

挡住通过的只有 UI-192：机器检查测到 200% 下编辑列横向溢出 79px，主 agent 实拍确认 token 名逐字断行、状态胶囊被盖住、有效值看不到。冻结矩阵里 U1 · 200% 只拍了首屏和导出对话框，所以评审没有看到这一块。修好 UI-192 并在下一轮复核通过，才能判通过。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 各评审单元分（取较低者） | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.6 | 2.8 | 2.7 | 0.6 | 8.7 | a 8.7 | 7.8 |
| U2 Studio 精调与来源解释 | 2.7 | 2.65 | 2.75 | 0.6 | 8.7 | a 8.85 · b 8.7 | 7.75 |
| U3 Studio 导入与导出 | 2.85 | 2.85 | 2.85 | 0.5 | 9.05 | a 9.05 · b 9.3 | 8.35 |
| U4 Studio 本地库迁移与存储反馈 | 2.75 | 2.7 | 2.5 | 0.65 | **8.6** | a 8.6 · b 8.7 | 7.55 |
| U5 Studio 预览 | 2.9 | 2.95 | 2.8 | 0.8 | 9.45 | a 9.45 | 9.1 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.9 | 3.0 | 1.0 | 9.9 | a 9.9 | 9.7 |
| U7 布局与排版 | 2.6 | 3.0 | 3.0 | 0.7 | 9.3 | a 9.3 | 7.1 |

- 本轮分数：**8.6**（U4）。
- 历史最佳：8.6。R2 的 7.1 属于旧基线，不参与比较。
- 门槛：8。门槛不写入目标包，也不告诉评审。
- 单元达标：U3、U4、U5、U6、U7。U1、U2 的单元分已过 8，但 UI-192 是这两个单元的门槛项，所以未达标。
- 本轮判「需要修改」，不需要确认评审。
- 各维度理由见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r3/judge.md)。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |

## 上轮差距复核

R2 的 89 条差距中：

- 已解决 60 条，不再适用 1 条（UI-156）；
- 部分解决 27 条，仍存在 1 条（UI-106），没有回归。

处置取自 [M7「UI 验收修复」](DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)中的「R2 → 修复」：87 条已修改待评审，UI-97、UI-156 按用户裁定不采纳。

- 评审来源的差距由本轮评审复核；UI-156 由主 agent 在运行页面上重新检查。
- 同一单元的两位评审结论不同时，取未关闭的一方，涉及 UI-17、UI-115、UI-128、UI-25、UI-114、UI-139、UI-55、UI-144。
- UI-97 本轮判已解决，UI-156 判不再适用，两条都没有转为待裁定。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-101 | [P1] 200% 文本下粘性头被封顶为内滚动区，「Preview mode」被切半，模式、撤销、保存、导出不可见 | 已修改待评审 | 已解决 | U1 评审：200% 头区不再内滚动，「Preview mode」完整，Light/Dark、Undo/Redo、Save/Import/Export 全部可见（U1-zoom200-720-light） |
| UI-88 | [P1] 〔待裁定〕Heading 不是粗体 | 已修改待评审 | 已解决 | U7 评审：Heading size 9 竖笔约 9–10px，与目标一致 |
| UI-152 | [P1] Text 的四档字重渲染完全相同 | 已修改待评审 | 已解决 | U7 评审：Text 四档墨量 0.089/0.115/0.144/0.181 依次递增；机器检查 300/400/500/700 字面已加载 |
| UI-89 | [P1] 〔待裁定〕Strong 没有加粗 | 已修改待评审 | 已解决 | U7 评审：「strong importance」墨量 0.231，相邻正文 0.159 |
| UI-17 | [P2] 选中 token 后来源与解除按钮不在屏内 | 已修改待评审 | 部分解决 | U2 a 部分解决；U2 b 判已解决、另把 390 所选格被遮列为新-4（同一问题）；取部分解决 |
| UI-91 | [P2] 非法属性后看不到属性定位诊断 | 已修改待评审 | 已解决 | U7 评审：点 Invalid prop 后出现带属性、收到值与回退值的诊断（U7-invalid-detail-1280）；文案问题另记 UI-189 |
| UI-125 | [P2] 差异表放在限高的内层滚动区，Apply 前看不全改动，且无滚动提示 | 已修改待评审 | 已解决 | U3 评审：只剩对话框本体一层滚动，1440 一帧看全 6 项差异与 Apply |
| UI-139 | [P2] 状态 callout 与决策 callout 叠放、内容重复，状态区与决策区的顺序随状态颠倒；冲突面板超出可视高度 | 已修改待评审 | 部分解决 | U4 a 已解决；U4 b 部分解决，取消 callout 同句两遍、冲突面板位置与其他状态不同；取部分解决 |
| UI-140 | [P2] 命名动作反馈是裸红字、位置不固定；被流程阻止的保存也用错误红，升级成功后仍残留且无内联动作，时态不对 | 已修改待评审 | 部分解决 | U4 评审，见本报告差距清单同编号条目 |
| UI-50 | [P2] callout 标题与对话框分节标题弱于正文，两类 callout 正文字号不一 | 已修改待评审 | 部分解决 | U4 评审，见本报告差距清单同编号条目 |
| UI-147 | [P2] 拾取信息面板在文档流里：开启时预览下推 175px，拾取较低部件时面板不在视野 | 已修改待评审 | 已解决 | U5 评审：开启拾取前后「DOM」标签都在 y=308；修法带出 UI-184 |
| UI-26 | [P2] 对话框 label、表头、节标题字重仍为 400 | 已修改待评审 | 已解决 | U3 评审：对话框 label、表头、节标题都是 13px 粗体 |
| UI-51 | [P2] 推荐动作与放弃动作同权；编辑列有多套描边按钮 | 已修改待评审 | 部分解决 | U4 评审：callout 内已分三级；升级 Review 对话框底部仍为同权蓝边按钮（条目见 judge-U4-a-addendum.md） |
| UI-102 | [P2] 390 编辑页的 Edit/Preview 标签条与粘性头之间有空隙，滚动内容从中透出 | 已修改待评审 | 已解决 | U1、U2 评审：390 两层固定区之间逐列扫描无透出（U1-configured-390、U2-source-overridden-390） |
| UI-153 | [P2] 非法属性样例默认态塌成一列一个词，上方有孤立的 `px` | 已修改待评审 | 已解决 | U7 评审逐条复核，见 judge-U7-a.md 复核表 |
| UI-02 | [P2] 未选分段带 #8AB7F7 蓝内框、段间 3px 双线，焦点环压到相邻段 | 已修改待评审 | 部分解决 | U1 评审部分解决（焦点环压邻段、两种接缝）；U5 评审已解决（焦点部分待核实）；取部分解决 |
| UI-92 | [P2] Reset 只覆盖 button、h2、ul 三种元素 | 已修改待评审 | 已解决 | U7 评审逐条复核，见 judge-U7-a.md 复核表 |
| UI-141 | [P2] 冲突的三个决策没有说明后果 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-104 | [P2] 壳层次级与禁用按钮有 4 套样式，闲置按钮用焦点同色 #8AB7F7 做边框，禁用按钮比可用按钮更重 | 已修改待评审 | 部分解决 | U1 评审：壳层已统一；200% 导出对话框仍为蓝边次级按钮、#858F9E 选择器边框 |
| UI-115 | [P2] 网格内 ↺ 只有图标，看不到解除范围 | 已修改待评审 | 部分解决 | U2 a 已解决（可见部分）；U2 b 部分解决，↺ 仍只有图标；取部分解决 |
| UI-126 | [P2] 切换 Output 时对话框重新垂直居中，选择器跳动 12–109px | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-127 | [P2] 390 卡片模式下分组行只铺约 105px 并折行 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-128 | [P2] 配置只列导入值，没有与当前主题的对比 | 已修改待评审 | 部分解决 | U3 a 已解决（主体）、另列新-1 缺图例；U3 b 部分解决，残留同一问题；取部分解决，沿用原编号 |
| UI-103 | [P2] 粘性头过高，占视口 31–38%，挤占编辑区 | 已修改待评审 | 部分解决 | U1 评审：1440 顶栏 56px 已解决；768 三行、200% 固定占 39% 仍在 |
| UI-148 | [P2] 拾取矩形在蓝色部件上看不出 | 已修改待评审 | 已解决 | U5 评审逐条复核，见 judge-U5-a.md 复核表 |
| UI-156 | [P2] 〔待裁定〕Code 变体在 U5 预览与 U7 夹具中的文字对比低于 4.5:1，与 UI-100 同属「参考选阶 / 旧 token 对」冲突 | 不采纳 | 不再适用 | 主 agent 页面实测：Studio Code soft 用 --primary 压其 12% 浅底（目标包明列的旧 token 对），catalog Code solid gray 为 a9 底配 contrast 白字（参考选阶）；属 U5/U7 已接受口径 |
| UI-09 | [P3] 英文 label、预览标题、选中字段标签页没有呈现 600 字重 | 已修改待评审 | 已解决 | U1 评审：label、预览标题、选中标签页呈现为半粗 |
| UI-70 | [P3] 拾取后编辑区滚动落点被粘性页头遮住，来源标题不在视野、第一行切半 | 已修改待评审 | 已解决 | U5 评审逐条复核，见 judge-U5-a.md 复核表 |
| UI-12 | [P3] 自动配对的中性色名显示为小写原始 id 且不翻译，与品牌色显示规则不一致 | 已修改待评审 | 已解决 | U1、U6 评审：中性色名本地化为「Mauve / 紫灰色」「Slate / 蓝灰色」 |
| UI-18 | [P3] 小节标题与字段 label 同样式，层级平 | 已修改待评审 | 部分解决 | U2 评审，见本报告差距清单同编号条目 |
| UI-22 | [P3] 诊断区嵌套标题倒置、正文大于标题、等宽字号混用 | 已修改待评审 | 部分解决 | U2 评审，见本报告差距清单同编号条目 |
| UI-40 | [P3] Rust/CSS 说明里的标识符只有部分等宽，`16.0` 与代码 `16.` 不一致 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-81 | [P3] classic 渐变幅度只有目标约 1/3，主体与 solid 几乎同色 | 已修改待评审 | 部分解决 | U6 评审：主体渐变达目标约 80%，上缘与侧缘高光仍偏弱 |
| UI-95 | [P3] 布局原语区仍有样例没有 prop 标注 | 已修改待评审 | 部分解决 | U7 评审：标注已补齐，37×19 显式样例仍不可见 |
| UI-105 | [P3] 同一行壳层控件高度有 28/36/38px 三档 | 已修改待评审 | 已解决 | U1 评审：原位置控件统一 32px |
| UI-106 | [P3] 编辑区顶部 padding 16px、两栏标题不对齐，组内状态行与按钮间距 0px | 已修改待评审 | 仍存在 | U1 评审部分解决、U2 评审仍存在（状态胶囊与按钮 0px）；取仍存在 |
| UI-142 | [P3] 升级 Review 对话框栅格不齐：值列错位 25px、主题表无表头、双细线 | 已修改待评审 | 部分解决 | U4 评审，见本报告差距清单同编号条目 |
| UI-143 | [P3] 无锁态：标题写「saving paused」，主按钮仍实心，正文是行话、缺句号 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-144 | [P3] 文案细节：修订号无标签、「Saved themes 0 → 0」、行话与含混措辞 | 已修改待评审 | 部分解决 | U4 a 部分解决；U4 b 判已解决、另列新-5 中的同一两处文案；取部分解决 |
| UI-149 | [P3] 拾取标签截掉 token 列表；较高部件的标签落在视口外 | 已修改待评审 | 已解决 | U5 评审：token 列表完整、标签在视口内；修法带出 UI-185 |
| UI-03 | [P3] 390 预览页顶部没有预览模式切换与保存/导出 | 已修改待评审 | 已解决 | U1 评审逐条复核，见 judge-U1-a.md 复核表 |
| UI-46 | [P3] 成功态容器不着色；同一琥珀底上两种标题色 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-47 | [P3] 禁用或空闲的存储操作按钮用非 token 色 #667085/#858F9E，且与 Undo 的禁用样式并存 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-55 | [P3] 取消后 callout 第一段沿用待确认时的文案 | 已修改待评审 | 部分解决 | U4 a 部分解决；U4 b 判已解决、把段内重复记入 UI-139；取部分解决 |
| UI-56 | [P3] 操作列表：「1 recent operations」单复数、「edit 2」孤字、失败项与成功项同色、说明段答非所问 | 已修改待评审 | 部分解决 | U4 评审，见本报告差距清单同编号条目 |
| UI-57 | [P3] 升级成功后「Download original before upgrade」在成功 callout 之外，且与其他状态的「Download original data」不同名 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-62 | [P3] 没有待处理项时 Retry / Cancel / Save current draft 看起来可点 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-86 | [P3] Heading wrap 行没有收窄宽度，1–3 档多行行高无法比对 | 已修改待评审 | 已解决 | U7 评审逐条复核，见 judge-U7-a.md 复核表 |
| UI-87 | [P3] Inset 的 left、right 贴边没有干净的展示 | 已修改待评审 | 部分解决 | U7 评审：新增 side=left/right 卡，但正文与贴边块重叠；严重度升为 P2 |
| UI-97 | [P3] 〔待裁定〕夹具根顶边混入外来灰线 | 不采纳 | 已解决 | U7 评审：按裁定后的页面裁切复核，夹具根第 9 行起为白，灰像素只在根外 8px 留边内 |
| UI-107 | [P3] 次级文字用 #667085、颜色搜索框边框用 #D0D5DD，偏离 G3 token | 已修改待评审 | 部分解决 | U1、U2 评审：帮助文字与搜索框边框已对，计数仍为 #667085 |
| UI-109 | [P3] 状态胶囊（Overridden、Custom、网格 Override）借用选中态蓝色 | 已修改待评审 | 已解决 | U1、U2 评审：状态胶囊改为中性色 |
| UI-110 | [P3] 「1 text pairs below 4.5:1」单复数错误 | 已修改待评审 | 部分解决 | U1 评审：预览条已改单数，编辑列摘要仍为「1 text pairs」；U5 评审已解决；取部分解决 |
| UI-111 | [P3] 768 下主题名（18px）比主标题（16px）大；1440 下两个 18px 标题并列 | 已修改待评审 | 已解决 | U1 评审逐条复核，见 judge-U1-a.md 复核表 |
| UI-112 | [P3] 分隔点「·」悬挂在行尾，Overridden 胶囊后多一个点 | 已修改待评审 | 已解决 | U1 评审逐条复核，见 judge-U1-a.md 复核表 |
| UI-114 | [P3] 页头「Modified / Matches preset」的比较基准不清，与存储状态读法冲突；吸顶区没有保存状态 | 已修改待评审 | 部分解决 | U1 a、U4 a 已解决；U4 b 部分解决，页头基准同义反复、胶囊用语与 callout 不一致；取部分解决 |
| UI-116 | [P3] 预览侧诊断条中 Locate 与所属色对的对应关系不清 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-117 | [P3] 同一对比数据两处精度不同 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-118 | [P3] 错误文字重复了上方 helper | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-145 | [P3] 冲突差异块纵向松散，组头配色与琥珀底混搭 | 已修改待评审 | 部分解决 | U4 评审，见本报告差距清单同编号条目 |
| UI-150 | [P3] DOM + GPU 时 DOM 列仍为两列卡片，标题与标签被挤成两行 | 已修改待评审 | 已解决 | U5 评审逐条复核，见 judge-U5-a.md 复核表 |
| UI-151 | [P3] 未标注态标签仍用选中蓝；面板为空 token 列表留一行空位 | 已修改待评审 | 已解决 | U5 评审逐条复核，见 judge-U5-a.md 复核表 |
| UI-154 | [P3] wrap=pretty 与 wrap=wrap 折行完全相同 | 已修改待评审 | 已解决 | U7 评审逐条复核，见 judge-U7-a.md 复核表 |
| UI-05 | [P3] 200% 下导出对话框动作按钮高度固定 32px，没有随字号放大 | 已修改待评审 | 已解决 | U1 评审逐条复核，见 judge-U1-a.md 复核表 |
| UI-25 | [P3] 「Import candidate」节里 Version/Name 行内排、配置字段上下排，两种排法 | 已修改待评审 | 部分解决 | U3 a 已解决；U3 b 部分解决，Import candidate 两组行距 27/29/33px；取部分解决 |
| UI-42 | [P3] CSS 注释中字体路径仍有 `makepad/widgets/…` 与 `makepad_widgets/…` 两种拼法 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-44 | [P3] 损失清单用 palette 而基础配置用 color，引号用法不统一 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-61 | [P3] 句末标点与行话 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-76 | [P3] DOM 与 GPU 两框内的首行内缩相差约 8px | 已修改待评审 | 部分解决 | U5 评审：纵向已对齐，横向内缩仍差 8px |
| UI-108 | [P3] 粘性头下边框下方 17px 又有一条分隔线，形成双细线 | 已修改待评审 | 已解决 | U1、U4 评审：顶栏底线下不再有第二条线 |
| UI-113 | [P3] 壳层焦点环有多种颜色/透明度 | 已修改待评审 | 已解决 | U1 评审逐条复核，见 judge-U1-a.md 复核表 |
| UI-119 | [P3] 选中的分体控件只高亮左半段 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-120 | [P3] 句中色值不是等宽字 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-121 | [P3] 同一概念两种措辞：Override / Overridden | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-122 | [P3] 范围文案「−0.5… 0.5em」像被截断 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-123 | [P3] 来源块里 token 名出现两次 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-124 | [P3] 诊断区「Diagnostics」与「Preview diagnostics」标题重复 | 已修改待评审 | 已解决 | U2 评审逐条复核，见 judge-U2-a.md 复核表 |
| UI-129 | [P3] 「Import notes」框内多一条横线 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-130 | [P3] 导出对话框按钮行下方留白大于 1.5rem | 已修改待评审 | 部分解决 | U3 评审，见本报告差距清单同编号条目 |
| UI-131 | [P3] 390 下「256 KiB」被拆到两行 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-132 | [P3] v1 损失清单里的 Auto 没写出实际配对的灰阶 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-133 | [P3] 备注与差异表对 token 的命名和字体不一致，`name` 行的作者文字用了等宽 | 已修改待评审 | 部分解决 | U3 评审，见本报告差距清单同编号条目 |
| UI-134 | [P3] 差异表首列表头「Mode / token」已不准确 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-135 | [P3] 吸顶表头没有底边，滚动时下方行像叠字 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-136 | [P3] 改动类型「Changed」与 label 同样式，390 下像无值的 label | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-146 | [P3] 冲突原文 textarea 无边框、透明底，截断处无滚动提示 | 已修改待评审 | 已解决 | U4 评审逐条复核，见 judge-U4-a.md 复核表 |
| UI-155 | [P3] 390 下 Kbd size 9 行换行后键帽上下相贴 | 已修改待评审 | 已解决 | U7 评审逐条复核，见 judge-U7-a.md 复核表 |
| UI-137 | [P3] 22 字节显示为「0.02 / 256 KiB」 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |
| UI-138 | [P3] 「Required source revision」行没有对齐表单列 | 已修改待评审 | 已解决 | U3 评审逐条复核，见 judge-U3-a.md 复核表 |

## 差距清单

共 64 条：P1 1 条、P2 9 条、P3 54 条，没有 P0。其中评审来源 63 条，机器检查 1 条（UI-192）。按 P0 → P3 排列，同一严重度内按可挽回分从高到低。

- 「来源」中的 a、b 指本单元的评审原文：[judge-U<n>-a.md](DOM-COMPONENTS-P1.ui-captures/r3/) 为首批，`judge-U<n>-b.md` 为补采后重评（U2、U3、U4）。
- 每条完整的取样数值和修法以原文为准。
- 截图在 [r3](DOM-COMPONENTS-P1.ui-captures/r3/) 目录中，文件名即证据位置。

### UI-192 · [P1] 200% 文本下精调 Colors 网格与来源块丢失内容：token 名逐字断行，「Overridden · Light」胶囊被相邻格盖住，有效值落到编辑列可见区外（U1/U2 · 布局 · 机器检查）

- 位置：[MC-root200-colors-grid-720-light.png](DOM-COMPONENTS-P1.ui-captures/r3/MC-root200-colors-grid-720-light.png)、[MC-root200-source-720-light.png](DOM-COMPONENTS-P1.ui-captures/r3/MC-root200-source-720-light.png)。条件同 U1 · 200%：720×900、根字号 200%，点 Colors 标签页。
- 现状（页面实测，[machine-check-followup3.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup3.json)）：
  - token 格只有 96px 宽：「background」断成 3 行（图上为「back / grou / nd」），「card-foreground」断成 4 行，色值「#f9fafb」断成 3 行；
  - 「Overridden · Light」胶囊宽 120px，越出 96px 的格子，被右侧 ↺ 段与相邻格盖住，图上只剩「Overr…len · Ligh」；
  - 选中 primary 后，来源块「Effective value」的值 `#1570ef` 位于 x=352，编辑列可见区右界为 x=319，图上「Effective value」下方是一片空白；
  - 编辑列出现横向滚动：`#studio-editor` scrollWidth 398、clientWidth 319（R2 同条件下为 327 / 319）。两张实拍里编辑列的 scrollLeft 已是 32：点 Colors 标签页时被带着横向滚动了，所以画面左缘也被裁掉一截。
- 期望：G6「200% 放大下主路径可达，不依靠整页横向滚动到达关键动作」；F1 展示有效值、来源、覆盖；WCAG 1.4.4（AA）文本放大到 200% 不丢失内容。精调属于主路径，所以按 P1 计。
- 修法：与 UI-163、UI-165 一起改。token 网格按编辑列宽自动降为单列（如 `repeat(auto-fill, minmax(13.5rem, 1fr))`，用 rem 写，使 200% 下也落到单列），胶囊 `white-space:nowrap` 并单独成行；来源块的键值网格在窄列下改为 dt 在上、dd 在下；编辑列内任何子项都设 `min-width:0`，验收为 200% 下 `#studio-editor` 的 scrollWidth 等于 clientWidth。
- 来源：机器检查。横向溢出测量发现编辑列 scrollWidth 398 > clientWidth 319，主 agent 在同条件下逐元素定位并实拍确认。冻结矩阵的 U1 · 200% 格只拍了首屏和导出对话框，评审没有看到这块。

### UI-163 · [P2] 1440 网格中「Overridden · Light」胶囊折成两行，文字压到圆角边，与配置组状态标识形状不一致（U2 · 布局、视觉 · 可挽回 a 0.35、b 0.2）

- 位置：U2-source-overridden、-detail、U2-token-released、U2-source-generated 四张 1440 图；如 background 格 x=20–157，胶囊 x≈50–125、y≈239–268，第二行「Light」离左侧弧边约 3px。有胶囊的格高约 80px，无胶囊的 ring 格约 45px。胶囊字号约 11px；同一状态在配置组里是 4px 圆角矩形加前导圆点。
- 现状 → 期望：圆角胶囊内文字折行贴边、行高参差、一个概念两种形状 → F4 分体控件；C3 状态标识完整可读；G2 分组行；G3 caption 12px。
- 修法：A 网格按编辑列宽自动降为单列（`repeat(auto-fill, minmax(13.5rem, 1fr))`），胶囊 `white-space:nowrap`；B 格内复用配置组的状态标识组件（4px 圆角、12px/600、前导 ●），拆成「● Overridden」「· Light」两个不换行片段，放在格内整行。
- 来源：U2 a 新-1、U2 b 新-1。判为同一问题的依据：同一批四张图、同一元素、同一折行现象。a 注明这是 UI-115 加了模式文字之后的副作用。

### UI-87 · [P2] Inset side=left/right 演示中正文压在贴边块边上（U7 · 布局 · 部分解决 · 可挽回 a 0.3）

- 位置：U7-layout-inset-demonstration-1280-light side=left 卡贴边块 y=47–207，正文大写字母顶部约 y=203；side=right 卡正文 y≈352–366，贴边块顶边 y≈362；390 与 U7-fixture-1280 y≈4740–5220、110% 帧同样存在。
- 现状 → 期望：贴边块与正文上下堆叠且重叠 → radix-inset 中 side=left/right 的图块贴三边、正文并排；L3 使内容贴边；L5 容器边界不遮住内容。
- 修法：只改夹具 Inset 区：side=left/right 两张卡改为 `Flex direction=row align=stretch gap=3`，贴边块给定宽度并随卡片拉伸，正文单独一列；right 镜像。验收为贴边块三边与卡片内沿重合、块内没有正文像素。
- 来源：U7 a UI-87（部分解决；评审将严重度从 P3 升为 P2）。

### UI-186 · [P2] 拾取浮动面板固定在预览右下角，遮住被拾取部件和预览内容；选中矩形与标签画在面板之上（U5 · 交互、布局、视觉 · 可挽回 a 0.25）

- 位置：U5-inspector-undeclared-hover / -undeclared / -undeclared-panel 面板 x 1000–1416、y 740–883；U5-inspector-card / U5-inspector / U5-inspector-picked 面板 y 707–883。「Create · 0」主体被面板压住；「Not annotated」标签（y 849–869）压过面板文字；刚开启时面板挡住右卡「Sample actions: 0」「ThemeDocument」。
- 现状 → 期望：被定位的部件和信息面板要同时可见、互不遮挡 → P2。
- 修法（需要结构性调整）：推荐把面板从覆盖层移进预览列网格的最后一行（`grid-template-rows: auto minmax(0,1fr) auto`，面板 `max-height:40vh; overflow-y:auto`），选中矩形与标签的覆盖层放在预览滚动区内并 `overflow:clip`。坚持浮动面板则至少给滚动容器留出面板高度的底部内边距、与目标相交时把面板翻到右上角、覆盖层 `z-index` 低于面板。
- 来源：U5 a 新-1（评审注明这是 UI-147 修法带出的新问题）。

### UI-157 · [P2] 200% 下渲染器分段控件（DOM / GPU / DOM + GPU）宽于预览列，右缘被视口截掉（U1 · 布局、交互 · 可挽回 a 0.2）

- 位置：U1-zoom200-720-light 预览工具栏第三行（y≈655–716），控件从 x=352 一直到视口边缘 x=719，没有右边框，右圆角被切。R2 同帧完整显示，属回归。
- 现状 → 期望：被截断 → G6 200% 下主路径可达、不依靠横向滚动；评分口径要求无溢出、截断。
- 修法：分段控件 `max-width:100%`，每段 `flex:1 1 auto; min-width:0`、允许换行；或给预览工具栏设 `container-type:inline-size`，用 rem 写容器查询，窄时改三列等分网格，再窄改为带 label 的下拉。确认预览列网格项 `min-width:0`。
- 来源：U1 a 新-1。

### UI-158 · [P2] 200% 下导出对话框的 Output 选择器被压窄，当前值截成「Theme…」（U1 · 布局、交互、细节 · 可挽回 a 0.2）

- 位置：U1-zoom200-export-720-light「Output」行，选择器约 213px 宽。R2 同帧显示完整「Theme JSON v2」，属回归。
- 现状 → 期望：看不出当前导出格式 → G6 对话框在可用视口内可操作；X6 窄时表单行单列；X2 须区分 v2 与兼容 v1。
- 修法：对话框表单行 `flex-wrap:wrap`，选择器 `flex:1 1 14rem; min-width:14rem`，不够时换到 label 下方占满；或按对话框宽度用容器查询切单列，不只依赖 ≤560px 视口断点。
- 来源：U1 a 新-2。

### UI-128 · [P2] Import candidate 的「→」没有「当前 / 导入」图例；Version 行的「→」含义与配置行不同；Name 行只有导入值（U3 · 交互、细节 · 部分解决 · 可挽回 a 0.2、b 0.15）

- 位置：U3-import-review-1440-light y=598–803；U3-import-review-390-light y=648–781；U3-import-review-mid-390-light y 58–338。
- 现状 → 期望：看不出哪边是当前、哪边是导入，未变化项写成「X → X」 → X1 导入先呈现候选差异（版本、名称、配置）。
- 修法：配置与 Name 行改成与 Changes 表同一套「Field | Current | Imported」列结构；相同的行只显示一个值并标「Unchanged」，不同的行加「Changed」徽标并计入 Changes (N)；Version 行改写为「Source: Theme JSON v1 · Saved as v2」；390 卡片同样用两列小标题。
- 来源：U3 a 判 UI-128 已解决（主体）、另列新-1；U3 b 判 UI-128 部分解决，残留即这一条。判为同一问题的依据：同一区域、同一缺图例现象。按跨轮沿用原编号的规则用 UI-128，严重度取 a 的 P2。

### UI-179 · [P2] 取消后 callout 让用户「Download your current theme」，callout 里却没有这个按钮（U4 · 交互 · 可挽回 a 0.2、b 0.1）

- 位置：U4-upgrade-cancelled-1440-light 琥珀 callout，第 2 段 y≈383–422；按钮只有「Review upgrade again」和「Download original data」。R2 同状态有「Download current theme」，配额态、无锁态 callout 也有。
- 现状 → 期望：用户想保住仅内存编辑，手边唯一的下载按钮会下载旧原始数据 → S1 原始数据与当前主题出口始终可达；S2 取消后常驻仅内存编辑；S6。
- 修法：取消态 callout 加「Download current theme」（复用配额/无锁 callout 的组件），顺序为「Review upgrade again」→「Download current theme」→「Download original data」（或改名「Download pre-upgrade data」）。
- 来源：U4 a 新-1、U4 b 新-3（b 标为相对 R2 回归）。

### UI-50 · [P2] callout 正文字号与颜色在状态之间、甚至同一张 callout 内来回切换（U4 · 视觉 · 部分解决 · 可挽回 a 0.15、b 0.15）

- 位置：13px #1D2939 见 U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 首段、各命名卡；12px #475467 见 U4-upgraded、U4-quota-failure、U4-no-locks-initial、U4-conflict 后两段。U4-no-locks 中相邻两张琥珀卡片一张 12px 灰字、一张 13px 深字。13px 常规字重不在 G3 字级上。
- 修法：callout 只保留两级文字：状态句用 body（b 建议 14px/1.5，a 建议 13px/1.5）#1D2939，补充说明用 caption 12px #475467；命名卡与状态 callout 共用。
- 来源：U4 a UI-50（部分解决，P3）、U4 b UI-50（部分解决，P2）；严重度取 P2。

### UI-51 · [P2] 升级 Review 对话框底部放弃动作与次级动作同权，且用蓝边蓝字，与壳层灰描边是两套样式；配额、无锁 callout 没有推荐动作（U4 · 视觉、交互 · 部分解决 · 可挽回 a 0.1、b 0.15）

- 位置：U4-upgrade-review-1440-light 底部 y712–743，「Download original data」「Cancel upgrade」同为 #1570EF 字、#8AB7F7 边；同流程 callout 里前者是灰描边、后者是文字按钮；U4-quota-failure、U4-no-locks callout 只有描边的「Download current theme」。
- 修法：对话框底部与 callout 用同一按钮组件：「Download original data」改壳层描边，「Cancel upgrade」改文字按钮放最后；无锁 callout 把「Download current theme」改为实心；配额 callout 加实心「Retry save」。导入导出对话框若用同一蓝边样式一并改（与 UI-104 同一套样式问题）。
- 来源：U4 a UI-51（部分解决，条目见 judge-U4-a-addendum.md，P3）、U4 b UI-51（部分解决，P2）；严重度取 P2。

### UI-02 · [P3] 分段控件残留：焦点环压到相邻段，两种接缝画法并存（U1 · 交互、视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U1-keyboard-04-1440-light Dark 焦点环左缘 x≈988–990 落进 Light 段；U1-default-1440-light EN/中文、U1-edit-390 / U1-preview-390 Edit/Preview 段间为 #84CAFF + #D0D5DD 两条线。
- 修法：EN/中文、Edit/Preview 改用 Light/Dark 的同一分段组件（相邻段 `margin-inline-start:-1px` 合并边框，选中段 `z-index:1`）；分段内 `:focus-visible` 用内收焦点环（`outline-offset:-3px`），或焦点段 `z-index:2` 并给组容器留 2px 内边距。
- 来源：U1 a UI-02（部分解决）；U5 a 判已解决，焦点部分待核实。

### UI-17 · [P3] 选中后的滚动位置没有同时露出所选格与完整来源块（U2 · 布局、交互 · 部分解决 · 可挽回 a 0.15、b 0.05）

- 位置：U2-source-overridden-390-light 吸顶区占 y=0–约 200，所选 primary 格只露出胶囊那一行；U2-source-generated-1440-light ring 来源块下半部与「Clear ring · Light only」在折线以下。
- 修法：token 格 `scroll-margin-top` 设为吸顶区实测高度 + 8px；选中后先保证所选格顶边不高于吸顶区下沿，再让解除按钮 `scrollIntoView({block:'nearest'})`。
- 来源：U2 a UI-17（部分解决）；U2 b 判 UI-17 已解决，另列新-4（390 所选格被吸顶区盖住），判为同一问题：同一帧、同一现象。

### UI-22 · [P3] 诊断区子 summary 比父 summary 更重，两组缩进不一（U2 · 视觉、布局 · 部分解决 · 可挽回 a 0.05、b 0.15）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs…」13px/400，内层「Text contrast pairs · 5 below 4.5:1」13px/600；字体诊断三条从 x=20 起且没有自己的标题，对比组从 x=32 起。
- 修法：外层改 13px/600 #1D2939，内层用 #475467；字体三条包进同级的 `<details>`（「Fonts · 3」），与对比组同样缩进。
- 来源：U2 a、b UI-22（均部分解决）。b 的这一条还写了 44px 空白与重复计数，分别归入 UI-169、UI-168。

### UI-187 · [P3] 拾取标签盖住相邻内容，并在连字符处断开 token 名（U5 · 细节、交互、布局 · 可挽回 a 0.15）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light Create profile 上方标签 x 396–507、y 603–639，两行「primary, primary-」「foreground, ring」，盖住复选框与「I understand」；U5-inspector-card-1440-light 标签 y 361–383 切掉上一行文字的下伸部。
- 修法：标签单行 `white-space:nowrap; width:max-content`，token 不在内部断行；内容缩成「首个 token +N」；大部件把标签放进矩形内侧左上角，小部件放外侧、空间不够时翻到下边；`pointer-events:none`；拾取确认后只保留矩形。
- 来源：U5 a 新-2（评审注明这是 UI-149 修法带出的新问题）。

### UI-103 · [P3] 顶栏在 768 多一行，在 200% 固定占视口 39%（U1 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-default-768 / U1-configured-768 顶栏 y=0–160 三行，第 2、3 行右半空置；U1-zoom200-720 顶栏 y=0–348 共 5 行且固定。
- 修法：Undo/Redo 与 Save/Import/Export 放进同一个可换行的动作行；200% 下把 Undo/Redo 放到 Preview mode 下方右列；顶栏超过视口 30% 时改为不吸顶，或把 Import/Export 收进 More。
- 来源：U1 a UI-103（部分解决；1440 已解决）。

### UI-104 · [P3] 200% 导出对话框仍用旧的蓝边次级按钮和非 token 边框（U1 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U1-zoom200-export-end-720-light「Select text」「Download」为 #8AB7F7 边、#1570EF 字；U1-zoom200-export-720-light Output 选择器边框 #858F9E。
- 修法：对话框动作区复用壳层次级按钮（白底、1px #D0D5DD、#475467 字），选择器边框改 #D0D5DD；只有 Copy code 保留实心主按钮。与 UI-51 是同一套样式问题，可一起改。
- 来源：U1 a UI-104（部分解决）。

### UI-106 · [P3] 状态行与解除按钮之间 0px；两栏标题中线差约 4.5px（U1/U2 · 布局、细节 · 仍存在 · 可挽回 U1 a 0.1、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light 与 U2-group-reset-1440-light，Brand、Neutral、Panel 各组的状态胶囊底边与「Use configuration values · both modes」顶边相接（如 y=488/489）；U1-default-1440-light 应用名与预览标题中线差约 4.5px。
- 修法：配置组容器 `display:grid; gap:.5rem`（或状态行下边距 8px）；预览标题行 `min-height:2rem; align-items:center`，与编辑列首行同高同中线。
- 来源：U1 a UI-106（部分解决）；U2 a、b UI-106（仍存在）。

### UI-18 · [P3] 小节标题仍与次级标签或字段标签同样式，或比 label 更弱（U2 · 视觉 · 部分解决 · 可挽回 a 0.1、b 0.1）

- 位置：U2-controls-1440「Typography」与「Choose a bundled font」同为 12px/600 #475467，「Sans font stack」为 #1D2939 13px/600；U2-field-error-1440「Geometry」同上；U2-group-reset-1440「Theme configuration」与「Appearance」「Brand color」同为 13px/600 #1D2939；来源块标题「Fine adjustment source」与键「Effective value」同样式。
- 修法：小节标题统一为一种眉题样式（12px/600 大写、字距 .04em、#475467，上方 24px、下方 12px，上加 #EAECF0 分隔线），或改为 14px/600 #1D2939；字段 label 保持 13px/600 #1D2939；子标签降为 12px/400 #475467。
- 来源：U2 a、b UI-18（均部分解决）。

### UI-165 · [P3] 来源块的键值结构只做了一半，行标签与块标题同样式（U2 · 布局、视觉、细节 · 可挽回 a 0.1、b 0.1 中的结构部分）

- 位置：U2-source-overridden-1440 y=637–873（-detail 图 y≈389–620 同）：只有「Effective value」一行是键值两列；token 名、「Explicit document override」、「overrides.light.primary」三行没有键名。
- 修法：来源块改为 `<dl>`，`grid-template-columns: 7rem 1fr`，行依次为 Token、Effective value、Source、Path、If cleared；dt 12px #475467，dd 13px #1D2939，路径 12px 等宽。
- 来源：U2 a 新-3、U2 b 新-5 的键值部分（同一区域、同一结构问题）。

### UI-169 · [P3] 诊断区「Authored colors fit the sRGB preview.」下方有约 44px 空白（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440，该行墨迹 y=119–131，下一条从 y≈175 开始；其余条目间约 16px。
- 修法：内容为空时不渲染该节点，或统一由父级 `gap:16px` 控制、去掉子元素 margin。
- 来源：U2 a 新-7；U2 b 在 UI-22 条下记了同一空白。

### UI-172 · [P3] 分组解除只给计数，看不出会影响哪些 token（U2 · 交互与状态 · 可挽回 b 0.1）

- 位置：U2-group-reset-1440-light Neutral「Light 15 · Dark 15」+「Use configuration values · both modes」（y≈635–690），Panel「Light 2 · Dark 2」（y≈841）。网格按 Surfaces、Semantic colors、Borders & focus 分组，与配置组不对应。
- 现状 → 期望：F2「解除将影响哪些项」。
- 修法：把计数做成 disclosure，展开后按模式列出 token 名；或点击计数时把网格按该组筛选到「Overridden only」。
- 来源：U2 b 新-8。

### UI-164 · [P3] 选中格里不可解除的 ↺ 仍显示为选中蓝，看起来可用（U2 · 交互与状态 · 可挽回 a 0.05、b 0.1）

- 位置：U2-source-generated-1440 ring 格 ↺ 为 #175CD3，同屏未覆盖格为 #98A2B3；U2-token-released-1440 primary 已解除，↺ 仍是 #175CD3，而来源块的解除按钮已置灰。
- 修法：选中样式只改外框和底色；↺ 段图标色由自身状态决定，`aria-disabled` 时固定 #98A2B3。
- 来源：U2 a 新-2、U2 b 新-2（同一两帧、同一元素）。

### UI-167 · [P3] 句中把模式名「Light:」也设成了等宽（U2 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U2-source-overridden-1440 y≈800、U2-source-overridden-390 y≈414「Clearing this override reveals Light: #d6409f.」
- 修法：`<code>` 只包住色值。
- 来源：U2 a 新-5、U2 b 新-5 的等宽部分。

### UI-130 · [P3] 导出对话框按钮行下方留白 32px，导入对话框为 24px（U3 · 布局 · 部分解决 · 可挽回 a 0.1、b 0.04）

- 位置：U3-export-json-1440、U3-export-rust-1440、U3-export-json-390 留白 32px；U3-import-review-bottom-1440/390、U3-import-error-1440 为 24px。
- 修法：空的复制/下载状态行不占间距，或把状态 live region 移进按钮行作为行尾项。
- 来源：U3 a、b UI-130（均部分解决）。

### UI-175 · [P3] v1 损失清单与 Import notes 正文降为 13px/400，不在 G3 字级里（U3 · 视觉 · 可挽回 a 0.1、b 0.07）

- 位置：U3-export-v1-1440 y=148–297 行距 19.5px；U3-import-review-bottom-1440 y=771/791 行距 20px；R2 同区域为 14px/21px。
- 修法：损失框与备注框正文用 body 14px/1.5；如果有意做成辅助说明，改用 caption 12px #475467。
- 来源：U3 a 新-4（标为回归）、U3 b 新-3。

### UI-173 · [P3] 字节计数「3332 / 262144 bytes」没有千分位，与说明里的「256 KiB」单位不一（U3 · 细节 · 可挽回 a 0.1、b 0.08）

- 位置：U3-import-review-1440 y=479、U3-import-error-1440 y=479、U3-import-review-390 y=530；同对话框 y=88 写「up to 256 KiB」。
- 修法：用 `Intl.NumberFormat` 加千分位，上限与说明用同一单位；中文同步。
- 来源：U3 a 新-2、U3 b 新-1。

### UI-174 · [P3] Rust 接入说明有歧义：「theme.json · theme.rs」读成并列；「Keep AppHandle.」丢了「alive」（U3 · 细节 · 可挽回 a 0.1、b 0.1 中的这一部分）

- 位置：U3-export-rust-1440、U3-export-copied-1440 y=167。
- 修法：改为「Put `theme.json` next to `theme.rs`.」「Keep the returned `AppHandle` alive while the UI is mounted.」；中文同步。
- 来源：U3 a 新-3、U3 b 新-2 的 Rust 部分。

### UI-178 · [P3] CSS 提示中的「Scoped dark selector: data-theme=dark」不是可用的选择器（U3 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U3-export-css-1440 y=230；同帧代码 y=510 为 `[data-theme="dark"]`。
- 修法：提示改为 `[data-theme="dark"]`。
- 来源：U3 a 新-7、U3 b 新-2 的 CSS 部分。

### UI-140 · [P3] 命名动作卡余项：配额卡没有内联重试；升级后卡片的文案与按钮重复（U4 · 交互 · 部分解决 · 可挽回 a 0.1、b 0.1）

- 位置：U4-quota-failure 红卡 y457–565 只有「Dismiss」，「Retry retained saves」在操作区 y617–648；U4-upgraded 中性卡正文写「Use Save locally to try again.」，卡里已有「Save now “Memory only”」。
- 修法：配额卡加「Retry “Retained name”」；升级后卡片删去「Use Save locally to try again.」，按钮改为「Save “Memory only” now」。
- 来源：U4 a、b UI-140（均部分解决）。

### UI-139 · [P3] 取消 callout 内同一句话写了两遍；冲突面板所在位置与其他状态不同（U4 · 布局、细节 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgrade-cancelled 标题「Memory only · upgrade cancelled」与正文「Upgrade cancelled. Memory-only editing; reopening restores…」「Reopening restores the original draft.」；U4-conflict callout 顶边紧贴 y=244（其他状态这里是分隔线、callout 从 y=261 开始），面板下方另有 y=806 分隔线。
- 修法：取消 callout 正文并为一句；冲突面板放回与其他状态相同的位置，删去面板下方那条分隔线（或所有状态统一用「上线、下线」结构）。
- 来源：U4 b UI-139（部分解决）；U4 a 判已解决。重复句与 UI-55 重叠，修 UI-55 时一并处理。

### UI-181 · [P3] 配额态页头与 callout 说「草稿留在内存」，操作列表却显示当前草稿已确认保存（U4 · 交互 · 可挽回 a 0.1）

- 位置：U4-quota-failure / U4-quota-operations 页头「Save failed」、callout「Your draft and pending actions stay in memory」、操作列表「#2 · Draft · confirmed (draft revision 2)」。
- 现状 → 期望：失败的是命名保存，界面笼统说草稿没存 → S4 区分先前快照成功与当前仍待保存、命名动作单独反馈。
- 修法：callout 标题改为「Storage full · new saves stay in memory」，正文分开写草稿已存、命名保存失败待重试；页头胶囊按最近一次草稿写入的结果显示。
- 来源：U4 a 新-3。

### UI-182 · [P3] 「Saving paused」状态下页头「Save locally」仍是实心主按钮，点了只会被拦下（U4 · 交互 · 可挽回 a 0.1、b 0.1）

- 位置：U4-upgrade-pending、U4-upgrade-cancelled、U4-conflict 页头「Save locally」（#1570EF 实心），紧挨「Saving paused」胶囊；无锁态已降为描边。
- 修法：胶囊为「Saving paused」或「Memory only」时，页头 Save locally 统一切到描边样式；callout 内推荐动作保持本区唯一实心按钮。
- 来源：U4 a 新-4、U4 b 新-4。

### UI-184 · [P3] 冲突键值栅格值列过窄，修订行折行出孤词，标签贴着值（U4 · 布局、细节 · 可挽回 a 0.1、b 0.1）

- 位置：U4-conflict-1440-light y341–440，值列从 x≈157 起只剩约 128px；「Revision 2 · Rustify / Light」折行；「Saved theme count」与「0 → 0」只隔约 8px；「Draft differences」紧贴上一行。
- 修法：280px 编辑列内键值改为标签在上、值在下，或 `max-content 1fr` 并把标签缩短为「Saved themes」；修订号与主题名分两行；「Draft differences」前加 8–12px。
- 来源：U4 a 新-6、U4 b 新-1。

### UI-55 · [P3] 取消态 callout 两段互相重复，并与标题重复（U4 · 细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U4-upgrade-cancelled 标题 y282 与两段正文 y313–422，「upgrade cancelled」「memory-only」「reopening restores」各出现两次。
- 修法：并为一段，同时补上 UI-179 的按钮。
- 来源：U4 a UI-55（部分解决）；U4 b 判已解决，把段内重复记入 UI-139。

### UI-144 · [P3] 文案余项：「0 → 0」无变化行、成功态时态错（U4 · 细节 · 部分解决 · 可挽回 a 0.1、b 0.1 中的这一部分）

- 位置：U4-conflict y433「Saved theme count 0 → 0」；U4-upgraded y334–370「…back up the library before upgrade.」
- 修法：数量没变时不显示或写「(unchanged)」；成功态改为「Original data keeps a copy of the library from before the upgrade.」
- 来源：U4 a UI-144（部分解决）；U4 b 判已解决，另列新-5，其中这两处判为同一问题（同一文字、同一位置）。

### UI-185 · [P3] 状态用词不统一：「upgrade」和「migration」混用，成功态重复「Saved on this device」（U4 · 细节 · 可挽回 a 0.1）

- 位置：「Review the library upgrade」（callout）、「Review upgrade」（按钮）、「Review the complete migration」（对话框标题）、「Complete migration JSON」混用；U4-upgraded 页头胶囊与 callout 第二行都写「Saved on this device」。
- 修法：全流程统一用「library upgrade」；成功 callout 删去「Saved on this device」这一行，由页头胶囊承担。
- 来源：U4 a 新-7；U4 b 新-5 也指出成功态重复「Saved on this device」。a 新-7 中「取消态胶囊写 Saving paused、callout 写 Memory only」这一点归入 UI-114。

### UI-81 · [P3] classic 的内高光（上缘高光带与两侧亮边）只有目标的 1/3 到 2/3（U6 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U6-fixture-1280-dark classic 4 对 radix-sink-button classic 4。主体渐变已达目标约 80%；上缘第 1 行 +50（目标 +78），其下衰减只有 1px（目标约 3px 的 +15、+7、+2）；左右侧缘 +6（目标 +18、+4）。
- 修法：只改 Button classic 变体的 inset 阴影与背景渐变层：上缘高光不透明度提高约 1.5 倍并在其下加 2–3px 带模糊的亮带；左右亮边不透明度提高约 3 倍并向内多衰减 1px；底部暗线保持。改完运行 `mbx xtask css`。
- 来源：U6 a UI-81（部分解决）。

### UI-189 · [P3] Reset 的 input、textarea 脱离值列，占满整行（U7 · 布局 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light「input · Reset」值列空，input 落到下一行 x=0–1200、y≈1070–1101；textarea 同样（y≈1277–1330）；390 同；390 下原生 blockquote 内容也掉到下一行。
- 修法：对照行改为两列 grid（`112px minmax(0,1fr)`，390 用 96px）；查 Reset 后 input/textarea 的宽度来源，Reset 不应附加原生没有的尺寸。
- 来源：U7 a 新-2。

### UI-95 · [P3] 布局原语区仍有样例没有可见呈现（U7 · 细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light y≈1567「Box · width=37px · height=19px · padding=3px」下方全白；240px 非法样例在默认态与回退后完全一样。
- 修法：37×19 盒加 gray-a3 底或 gray-a7 边框并标「37×19」；240px 样例同样加底色，让 240px 与 auto 宽度可区分。
- 来源：U7 a UI-95（部分解决；标注已补齐）。

### UI-190 · [P3] Reset 区 button 行右侧有没有标注的「0」（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-layout-reset-comparison-1280-light x≈259、y≈131；390 x≈245、y≈150；U7-fixture-1280 y≈1814。
- 修法：若是计数器，改为「clicks: 0」并用标注同款样式；否则删除。
- 来源：U7 a 新-3。

### UI-191 · [P3] 非法属性诊断文案重复，并带字面反引号（U7 · 细节 · 可挽回 a 0.1）

- 位置：U7-invalid-detail-1280-light y=487、U7-invalid-1280-light y≈1700：「Box · invalid prop \`width\`: … received "12px;display:none". Rejected value: 12px;display:none. Fallback: width=auto. …」
- 修法：改成一句，如「Box · width: rejected "12px;display:none" (expected one CSS value) · fallback width=auto」；属性名用 Code soft size 1 而非字面反引号。
- 来源：U7 a 新-4。

### UI-177 · [P3] 备注中的字体族值是比例字，390 下「ui-serif」从连字符处断开（U3 · 视觉、细节 · 可挽回 a 0.08、b 0.04）

- 位置：U3-import-review-end-390 y≈699/719、U3-import-review-bottom-390 y≈675/695；1440 y=771/791。
- 修法：requested/resolved 两个值用与路径相同的 inline code 样式并 `white-space:nowrap`。
- 来源：U3 a 新-6、U3 b 新-5；U3 b 在 UI-133 条下也记了作者值为比例字。

### UI-176 · [P3] 「Changed」徽标与列、基线没对齐，表头下方是双线（U3 · 布局、细节 · 可挽回 a 0.05、b 0.08）

- 位置：U3-import-review-bottom-1440 / -end-1440 表头「Change」x=573，徽标边框 x=576、文字 x=584；徽标基线比同行文字低约 2px；y=301 表头底边与 y=302 分组行上边形成 2px 双线；U3-import-review-mid-390 徽标 x=53，同卡 token 与「Current」x=50。
- 修法：去掉徽标左缩进或让该单元格 padding 与其他 `td` 相同；徽标 `inline-flex` 居中、单元格 `vertical-align:middle`；`thead + tbody > tr:first-child{border-top:0}`。
- 来源：U3 a 新-5、U3 b 新-4（同一徽标；b 补了基线、双线与 390）。

### UI-133 · [P3] 备注与差异表对 token 的命名方式不一致（U3 · 细节 · 部分解决 · 可挽回 a 0.06、b 0.08）

- 位置：U3-import-review-bottom-1440 y=771「base.light.font-serif」；同帧分组头「Light · base (1)」加行名「base.primary」。
- 修法：两处统一用同一套路径格式：行名只写 token、备注写「Light · base · font-serif」；或都写完整路径。
- 来源：U3 a、b UI-133（均部分解决）。

### UI-25 · [P3] Import candidate 两组行距不一致（U3 · 布局、细节 · 部分解决 · 可挽回 b 0.06）

- 位置：U3-import-review-1440-light label 大写字母顶边依次为 594、621、654、683、712、741、770、799：Version → Name 27px，配置各行 29px，Name → Appearance 33px。
- 修法：Version、Name 与六个配置字段放进同一个 `dl` grid，只用一个 `row-gap`，去掉两块之间的额外 margin。
- 来源：U3 b UI-25（部分解决）；U3 a 判已解决。

### UI-107 · [P3] 计数文字仍为 #667085，两个搜索框占位符颜色不一（U1/U2 · 视觉 · 部分解决 · 可挽回 U1 a 0.05、U2 a 0.05、U2 b 0.05）

- 位置：U1-configured-1440-light、U2-group-reset-1440-light 的「Light N · Dark N」计数与「No overrides」胶囊文字为 #667085；U1-field-tabs-1440-light「Search colors…」占位符 #667085，U1-default-1440-light「Search presets…」约 #8E949C。
- 修法：计数与胶囊文字改用 #475467，#667085 只留给禁用态和占位符；两个搜索框共用一个 `::placeholder` token。
- 来源：U1 a、U2 a、U2 b UI-107（均部分解决）。

### UI-110 · [P3] 编辑列诊断摘要仍是「1 text pairs」（U1 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U1-keyboard-04-1440-light 编辑列底部 y≈783「Diagnostics · 1 text pairs below 4.5:1」；预览诊断条已改为单数。
- 修法：编辑列摘要与预览诊断条共用同一个复数函数。
- 来源：U1 a UI-110（部分解决）；U5 a 判已解决。

### UI-159 · [P3] 中文「Saved on this device」有两种译法（U1 · 细节 · 可挽回 a 0.05）

- 位置：U1-zh-top-1440-light 顶栏胶囊「已保存到本机」，同帧编辑列状态块「已保存到此设备」。
- 修法：两处共用同一个文案 key，统一为「已保存到本机」。
- 来源：U1 a 新-3。

### UI-161 · [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」（U1/U5 · 细节 · 可挽回 U1 a 0.05、U5 a 0.05）

- 位置：U1-default-1440-light 等帧及 U5-dom-1440-light、U5-gpu-compare 右上 x≈1266–1416、y≈88–98；U5-dom-1440-dark 为「Rustify Light　Dark preview」；U5-preview-390-light y≈229。R2 同位置是「Rustify Light · Light preview」。
- 现状 → 期望：主题名与预览模式连读 → C4 显示「预览模式」以免与作者外观混淆。
- 修法：恢复只在同一行时显示的「 · 」分隔（换行时去掉悬挂点），或把模式写成「Preview: Light」或中性胶囊；中文同步。
- 来源：U1 a 新-5、U5 a 新-3。判为同一问题的依据：同一位置、同一文字、同一原因（R2 的分隔点被去掉）。

### UI-162 · [P3] 顶栏左右内边距与两栏不一致，主题名与说明按中线而非基线对齐（U1 · 布局、细节 · 可挽回 a 0.05）

- 位置：U1-default-1440-light「Export」右缘 x=1419、预览分隔线右端 x=1416；U1-default-768-light 顶栏左 20 / 右 747，编辑与预览为 16 / 751；1440 主题名基线 y≈33、「Matches…」基线 y≈31。
- 修法：顶栏 `padding-inline: 20px 24px`，≤800px 改 16px 16px；主题名组 `align-items:baseline`。
- 来源：U1 a 新-6。

### UI-160 · [P3] 颜色搜索框 36px 与同一行的 Clear 32px 不等高（U1/U2 · 布局、细节 · 可挽回 U1 a 0.03、U2 b 0.05）

- 位置：U1-field-tabs-1440-light「Search colors…」y=537/572，「Clear」y=539/570；U2-source-overridden-1440-light 搜索框 y=73–108、「Clear」y=75–106。
- 修法：输入框与按钮同高，统一用壳层控件高度。
- 来源：U1 a 新-4、U2 b 新-9。判为同一问题的依据：同一对控件、同一高度差。

### UI-115 · [P3] 网格 ↺ 仍只有图标（U2 · 交互与状态 · 部分解决 · 可挽回 b 0.05）

- 位置：U2-source-overridden-1440-light 各格右侧的 ↺ 段。
- 现状 → 期望：模式已由相邻胶囊写出，但 ↺ 本身没有可见文字或提示 → F2 显示范围；C3 不只靠悬停。
- 修法：给 ↺ 加与来源块按钮同文案的 `aria-label` 和 `title`（如「Clear primary · Light only」）；补拍悬停与键盘聚焦帧。
- 来源：U2 b UI-115（部分解决）；U2 a 判已解决（可见部分，aria-label 截图不可见）。

### UI-166 · [P3] 字体下拉的显示值是粗体，与其他下拉、输入框不一致（U2 · 视觉 · 可挽回 a 0.05、b 0.05）

- 位置：U2-controls-1440 三处「Custom author stack」为 600，同页与 U2-group-reset 的其他下拉值为 400。
- 修法：去掉字体下拉值的 `font-weight`；要提示自定义栈就在 label 旁加中性「Custom」标识。
- 来源：U2 a 新-4、U2 b 新-3。

### UI-168 · [P3] 同一对比计数在三处用了三种措辞，父子 summary 重复同一个数字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-low-contrast-1440 外层「Diagnostics · 5 text pairs below 4.5:1」、内层「Text contrast pairs · 5 below 4.5:1」、预览条「5 text pairs below 4.5:1 · author colors preserved」。
- 修法：外层只写「Diagnostics」或「Diagnostics · 5 issues」，内层只写「Text contrast pairs」，预览条保留一种固定句式。
- 来源：U2 a 新-6；U2 b 在 UI-22 条下记了同一重复计数。

### UI-170 · [P3] 预览侧诊断条末行分隔线与外框形成双线（U2 · 细节 · 可挽回 b 0.05）

- 位置：U2-group-reset-1440-light 末行分隔线 y=418、外框底边 y=431；U2-source-overridden、U2-low-contrast 同样。
- 修法：`.diag-row:last-child{border-bottom:0}`。
- 来源：U2 b 新-6。

### UI-171 · [P3] 「Restore document · Graphite」没有写范围（U2 · 交互与状态 · 可挽回 b 0.05）

- 位置：U2-restore-1440-light，按钮在 y≈196–227，附近没有说明文字。
- 现状 → 期望：另两档重置都带范围后缀（「· Light only」「· both modes」），第三档只写了目标名 → F2「恢复预设/文档」须显示范围。
- 修法：按钮下加 12px #475467 说明，例如「Discards configuration changes and all overrides in both modes」；或把范围写进按钮文案。
- 来源：U2 b 新-7（补采帧）。

### UI-114 · [P3] 页头基准用词同义反复，胶囊用语与 callout 不一致（U4 · 交互、细节 · 部分解决 · 可挽回 b 0.05）

- 位置：U4-upgrade-pending 页头「旧草稿 Matches “旧草稿”」；U4-quota-failure「Rustify Light Modified from “Rustify Light”」看不出基准是预设还是已保存文档；U4-upgrade-cancelled 胶囊「Saving paused」、callout 标题「Memory only」。
- 修法：基准改写为「Matches saved document」或「Modified from saved document」；取消态胶囊改为「Memory only」。
- 来源：U4 b UI-114（部分解决）、U4 a 新-7 的胶囊用语部分；U1 a、U4 a 判已解决。

### UI-142 · [P3] 升级 Review 对话框主题表底边与分节线形成双细线（U4 · 布局、细节 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-upgrade-review-1440-light，y=309 与 y=322 两条 #EAECF0。
- 修法：表格最后一行去掉下边框，或分节容器紧跟表格时不画上边框。
- 来源：U4 a、b UI-142（均部分解决）。

### UI-145 · [P3] 冲突差异块一处改动占四行，徽标左缘不齐（U4 · 布局 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-conflict-diff-1440-light 路径 y147、徽标 y185、列标签 y220、值 y240；徽标左边框约 x45、路径文字 x41。
- 修法：路径与徽标同一行，下一行写「This tab → Other tab」两列；徽标左缘与路径对齐。
- 来源：U4 a、b UI-145（均部分解决）。

### UI-180 · [P3] 配额态「Technical details」被移到错误 callout 之外（U4 · 布局 · 可挽回 a 0.05、b 0.05）

- 位置：U4-quota-failure / U4-quota-operations，红色 callout y261–406，「Technical details」在 y435，位于命名卡之上；R2 时在 callout 内。
- 修法：放回错误 callout 内、「Download current theme」下方，与正文左对齐。
- 来源：U4 a 新-2、U4 b 新-2（两位都标为相对 R2 回归）。

### UI-183 · [P3] 文字按钮的标签比正文缩进 11px，单独一行的「Dismiss」像是悬空（U4 · 视觉 · 可挽回 a 0.05）

- 位置：「Cancel upgrade」标签左缘 x=45、正文 x=34（U4-upgrade-pending y501）；「Dismiss」在 U4-upgrade-cancelled y641、U4-quota-failure y536、U4-no-locks y518。
- 修法：文字按钮用 ghost-offset 几何（`margin-inline:-11px`），或让 Dismiss 与本卡主动作同一行。
- 来源：U4 a 新-5。

### UI-56 · [P3] 操作说明段讲本状态没有的动作，说明与列表未缩进，修订号被断行（U4 · 细节 · 部分解决 · 可挽回 a 0.05、b 0.05）

- 位置：U4-quota-operations y612–666 最后一句「Loading the other version removes all pending actions.」；说明段与列表从 x=20 起；y744–762「(draft / revision 2)」被断开。
- 修法：说明段按当前状态只列可用动作；展开内容整体缩进到摘要文字起点；修订号片段 `white-space:nowrap`。
- 来源：U4 a、b UI-56（均部分解决；断行由 b 补充）。

### UI-188 · [P3] 选中或按下后字重变粗，按钮变宽，右侧控件横移 1–2px（U5 · 细节 · 可挽回 a 0.05）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light，Inspect 由 x 756–822 变为 756–824，Focus preview 右移 2px；U5-gpu-compare 选 DOM + GPU 后 Inspect 右移 1px。
- 修法：给分段与切换按钮的标签预留粗体宽度（隐藏的粗体伪元素或 `min-width`），在共用的分段组件里改一次。
- 来源：U5 a 新-4。

### UI-76 · [P3] DOM 与 GPU 两框首行左内缩相差 8px（U5 · 细节、布局 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 x=370（内缩 26），GPU「GPU · Cards」x=908（内缩 18）；纵向已对齐。
- 修法：GPU 场景根 padding 改为 24px，或两边统一取同一 token。
- 来源：U5 a UI-76（部分解决）。

## 待裁定事项

无。

- R2 的待裁定事项都已了结：UI-88、UI-89 本轮判已解决；UI-97 按改后的采集方式复核为已解决；UI-156 因目标口径已改，判不再适用。
- 本轮重建基线，不判停滞；修复记录里没有最终处置为不采纳、待决定或阻塞而本轮仍判存在的差距。

## 建议（不扣分）

汇总自各评审，原文见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r3/judge.md) 的「建议」节。下面列出跨单元重复出现、影响较大的几条：

- 编辑列顶部的存储状态块与顶栏保存胶囊重复，可压成一行摘要加展开，让预设搜索与精调更早出现（U1 a；U2 a、b 也指出 390 下吸顶区约占 24%）。
- 导出 JSON 格式化，代码框用 `white-space: pre` 加横向滚动（U3 a、b；R2 已提）。
- 对话框标题行吸顶、动作行吸底（U1 a、U3 a、U3 b、U4 b）。
- U6 夹具补 high-contrast、gray、disabled 列，IconButton 矩阵，各色 classic，Link 字级与 underline 模式。补齐后，「与目标一致」的结论才能覆盖整张目标图。
- 「Caller utilities」示例改为可读配色（U6 a；axe 1.24:1，与 R1、R2 相同）。
- Link 的焦点环有 0.15s 过渡，在 `prefers-reduced-motion: reduce` 下仍在播放（主 agent 实测，见机器检查）。目标包没有相关要求，只作建议。

## 机器检查

全部在运行中的 release 页面上执行，在首批评审交卷之后进行，结果没有交给评审。

| 检查 | 工具与版本 | 结果 | 对应差距 |
| --- | --- | --- | --- |
| 控制台错误、失败请求、pageerror | Playwright 1.63.0 / Chromium 153.0.8010.12 事件监听；采集 3 次运行与机器检查各会话 | 均为 0 | — |
| CSP 违规 | 页内 `securitypolicyviolation` 监听，strict CSP 服务 | 0 | — |
| 横向溢出 | 页内测量 `scrollWidth` 与越界元素 | 见本表下方「横向溢出」 | 新增 UI-192；佐证 UI-157 |
| 对话框是否在视口内 | 页内 boundingBox | 导出对话框：1440 下 x208、y16、w1024、h534；768 下 x16、w736；390 下 x16、w358、h565。都在视口内 | — |
| 可访问性扫描 | @axe-core/playwright 4.13.0，wcag2a/2aa/21a/21aa/22aa | 见本表下方「axe 结果」。incomplete 项未逐项人工复核 | UI-156 不再适用；无新增 |
| 语言切换点击区 | 页内 boundingBox + `elementFromPoint` | EN 39×32、中文 48×32，中英两态中心都命中自身 | — |
| 键盘走查（真实按键） | Playwright keyboard | 见本表下方「键盘走查」 | — |
| B2 焦点样式 | 键盘聚焦后读计算样式（Shift+Tab、Tab） | 见本表下方「B2 焦点样式」 | 与 U6 评审结论一致，无新增 |
| 字面加载 | catalog 布局夹具页内计算样式 + `document.fonts` | Text 四行计算字重 300/400/500/700，Heading（h2）与 Strong 为 700；IBM Plex Sans 300、400、500、700 已加载，600 未加载（该页不用 600）。Studio 页没做此项 | 佐证 UI-152、UI-88、UI-89 已解决 |
| 200% 吸顶头区 | 页内测量 + 滚轮 | 编辑列内不再有 sticky 或内滚动区；「Preview mode」标签在 y=38–77，在头区上滚轮不动，在下方滚轮时编辑列 scrollTop 0→400 | 佐证 UI-101 已解决 |
| Code 选阶（UI-156） | 页内读取计算色与组件色阶变量 | 见本表下方「Code 选阶」 | UI-156 不再适用 |
| 性能 | — | 未检查。本轮 UI 验收不测性能，没有新增毫秒 SLO | — |

**横向溢出**

- 页面级横向溢出全部为 0。覆盖：Studio 1440、768、390 编辑与预览、720 + 根字号 200%；catalog 按钮与布局夹具 1280、390。
- 720×900 + 根字号 200% 下有两处列内溢出：
  - `#studio-editor` scrollWidth 398 / clientWidth 319（R2 同条件为 327 / 319）；
  - `.studio-previews` scrollWidth 405 / clientWidth 400（R2 为 400 / 400）。
- 主 agent 逐元素定位后实拍：
  - 编辑列：精调 Colors 网格的 token 名与色值逐字断行、「Overridden · Light」胶囊越出格子被盖住，来源块的有效值落在可见区外。列为 UI-192，证据见 [machine-check-followup3.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup3.json) 与两张 `MC-root200-*` 实拍；
  - 预览列：渲染器分段控件右缘越界，与评审的 UI-157 是同一现象，不另列。
- `MC-root200-pill-720-light`、`MC-root200-effective-720-light` 是定位时的初拍，UI-192 以同条件重拍的 `MC-root200-colors-grid`、`MC-root200-source` 两张为证据。
- 预览里「Create · 0」「Disabled」也测到越过预览列右界，但它们在 `section.preview-surface` 内，该容器有自己的横向滚动。主 agent 为拍它们用了 `scrollIntoView`，连带把这个容器横向滚动了，所以两张 `MC-root200-preview-*` 不能说明默认状态下是否被截，不列差距，留作下一轮补拍线索。

**axe 结果**

- Studio 1440、768、390 编辑与预览、导出对话框：0 个违规节点。
- Studio 1440 DOM 预览：1 个节点，Code soft「ThemeDocument」3.89:1，属 UI-156，按已接受口径不计。
- catalog 按钮夹具（1280、390 各 19 个节点）：
  - 18 个属用户已接受的 UI-100 口径：16 个色族 solid 白字压第 9 阶（2.97–4.12:1），加上 Accent pair 与 Semantic accent（2.62:1）；
  - 1 个是「Caller utilities」示例（1.24:1），按 R1、R2 做法归入建议。
- catalog 布局夹具：1 个节点，Code solid gray 3.31:1，属 UI-156 口径。
- 逐节点清单见 [machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup2.json)。

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
- Link：等 1s 过渡结束后，两条链接都是 2px solid rgb(141,164,239)、offset 2px、圆角 1.26px 与 1.05px、下划线隐藏。Tab 后立即读取会读到过渡中间值（3px、offset 0），与 R2 相同，已用等待 1s 的复测更正（[machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup.json)）。

**Code 选阶（UI-156）**

- Studio 预览「ThemeDocument」（Code soft，未设 color）：组件色阶变量未定义，文字用 `--primary`（浅色 rgb(21,112,239)、深色 rgb(83,177,253)），底色为同色 12%。这是目标包 U5 口径点名的「Code soft 用 primary 压其浅色底」，属作者主题旧 token 对。
- catalog Code（indigo、gray，四种变体及 high-contrast）：soft 与 outline 文字取 a11，soft 底取 a3，solid 底取 a9、文字取 contrast 白，high-contrast 文字取第 12 阶。axe 报的 Code solid gray（3.31:1）是 a9 底配 contrast 白，属参考选阶。
- 两处都落在 U5、U7 已接受的口径内，UI-156 判不再适用。

原始结果：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check.json)、[machine-check-followup.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup.json)、[machine-check-followup2.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup2.json)、[machine-check-followup3.json](DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup3.json)。脚本为同目录同名 `.cjs`。

## 采集与覆盖

### 环境与被测版本

- HEAD `d5775d8a661ae8832188362a3fdb00488ec34ac9`（main，dirty）。产品路径（crates、examples、xtask、web、makepad、public、Cargo.toml、Cargo.lock、sources.lock.json）相对 HEAD 无 diff，也没有未跟踪文件。tracked diff 指纹 `b04d5427f42e`，只含 SDLC 台账与两张旧验证 PNG。
- 本轮从 HEAD 重建两个示例的 release（`CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0 mbx xtask build-web --release`）：
  - Studio `80cad3c530b54d9f`，WASM 9,802,741 B，与修复记录写的大小一致；
  - catalog `7ea7f541a819b220`，WASM 8,197,818 B。
- 服务：`mbx xtask serve` strict CSP，Studio 端口 4181，catalog 端口 4176。采集前用 `curl …/build-manifest.json` 确认服务发出的正是这两个 build id。

### 采集

- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless，脚本 [capture-runner.cjs](DOM-COMPONENTS-P1.ui-captures/r3/capture-runner.cjs)。
- 条件：DPR 1，B2 焦点特写为 DPR 2；`reducedMotion: reduce`；等待字体就绪、网络空闲、500ms 无长帧。
- 数量：首采 134 张，补采 5 张，共 139 张，0 失败，原图全部保留。补采中 `U3-export-rust-mid-1440-light` 没滚到中段，已在 capture.md 注明不作证据。
- 与 R2 采集的差异（只改了目标包「采集」节，并记入变更记录，目标指纹不变）：
  - R2 的补采帧并入首采；
  - U4 冲突面板增拍「Draft differences」展开帧；
  - U7 整段夹具帧按用户裁定 UI-97 改为页面裁切，四周留 8px。
- 逐图登记见 [capture.md](DOM-COMPONENTS-P1.ui-captures/r3/capture.md)，其中有 R2 与 R3 冲突帧的对应表。

### 目标文件 hash

| 文件 | SHA-256 前缀 |
| --- | --- |
| 目标包（含 R3「采集」节两条变更记录） | `4dff4aeda6f5` |
| 13 张目标图 | 与 R1、R2 相同（见 [R1 报告](DOM-COMPONENTS-P1.ui-review.md#目标文件-hash)），本轮逐张复算一致 |

- 目标指纹：`manual-sha256-normative-sections-v1:567117725f56f04104860e0b77173cb2e37913b14014c530158f7d989b1dd5d8`，算法见 [target-fingerprint.py](DOM-COMPONENTS-P1.ui-captures/r3/target-fingerprint.py)。
- R2 的指纹是 `24f1e44d673e…`。变化来自 R2 之后用户裁定 UI-156，对 U5、U7 比较口径的修改，所以本轮重建基线。

### 补采与未覆盖范围

首批评审报告了冻结矩阵内看不出的格子，主 agent 补采后各派一位全新评审重评：

- **U2**：编辑列顶部的「Restore document」入口（F2 第三档重置）。
- **U3**：Rust 接入说明的代码中段；390 导入审阅中段的 Identity 卡片。
- **U4**：升级 Review 中展开的「Complete migration JSON」。

评审报告的下列看不出项不在冻结矩阵内，没有补采：

- 键盘焦点帧（U1 的除外）、悬停与按下态；
- 中文态（U1 除外）；
- GPU 失败态、预览宽度档；
- 导入的语法错误、超限、不支持颜色；复制或下载失败；排队态；Locate 点击结果；
- U4 除待确认外的 390 帧；
- U6 夹具未提供的 IconButton 矩阵、多色 classic、gray / high-contrast / disabled 列、Link 字级与下划线模式；
- U7 Restore prop 之后的帧，以及 390 下的缩放与非法态。

把这些加入矩阵要改目标包的「单元」节，需要用户确认，下一轮按重建基线评审。截图来自 headless Chromium，不代表 Firefox、Safari、真机、人工读屏或 OS IME。

下一轮补拍或实测的线索（本轮不计差距）：

- 200% 下 DOM 预览内容是否被截。见上文「横向溢出」末条。
- 200% 下精调 Colors 网格不在冻结矩阵的 U1 · 200% 格里，本轮由机器检查发现（UI-192）。下一轮建议把「200% 下 Colors 页选中一个 token」并入 U1 · 200% 的采集步骤，让评审也能看到。这一步只改「采集」节，不影响指纹。

### 环境噪声

- `U4-upgrade-review-end-1440-light` 与 `U4-upgrade-review-1440-light` 逐像素相同，因为对话框在 900px 高内不需要滚动；`U4-conflict-detail-end-1440-light` 与 `U4-conflict-1440-light` 逐像素相同，因为冲突面板已整块在视口里。两组都由 U4 评审核对（前一组注明 md5 一致）。
- `U6-links-hover-1280-light` 与 `U6-links-1280-light` 逐字节相同，由 U6 评审用 `cmp` 核对。
- 同一版本重拍没有出现不一致的结果。

### 评审方式

- 共派 10 位全新上下文的只读评审，模型继承主 agent：
  - 首批 7 位并行，每单元一位；
  - 补采后为 U2、U3、U4 各派 1 位全新评审重评，提示与材料同首批，另加本单元补采帧。
- U4 首批评审的差距清单漏了 UI-51 一条，主 agent 请同一位评审只补这一条，补充原文另存为 [judge-U4-a-addendum.md](DOM-COMPONENTS-P1.ui-captures/r3/judge-U4-a-addendum.md)，没有改动已给出的分数与其他条目。
- 每个单元取总分较低那位评审的一整组四维分与理由：U2 取 b，U3、U4 取 a。差距取并集，同一问题按显式映射合并，映射写在 judge.md 各条的「来源」里。
- 评审只拿到目标包、目标图、本单元 r3 与 r2 截图、两轮 capture.md、actions.log，以及 R2 judge.md 中本单元的摘录（[judge-inputs/](DOM-COMPONENTS-P1.ui-captures/r3/judge-inputs/)）。没有拿到代码、计划、修复记录、门槛、机器检查结果或其他评审的结论。这是提示里给的白名单，不是文件权限沙箱：r3 目录里同时放着其他评审原文与机器检查结果。U4 b 在原文中写明「r3 目录里其他评审文件（judge-U4-a 等）没有打开」。
- 本轮将判「需要修改」，不需要确认评审。

### 主 agent 核对说明

- **事实抽查**：开图核对了以下评审描述，均属实：
  - UI-163：U2-source-overridden-1440-light 中「Overridden · Light」胶囊折成两行；
  - UI-179：U4-upgrade-cancelled-1440-light 的 callout 只有「Review upgrade again」「Download original data」两个按钮；
  - UI-186：U5-inspector-undeclared-1440-light 中拾取面板压住「Create · 0」，「Not annotated」标签画在面板之上；
  - UI-87：U7-layout-inset-demonstration-1280-light 中 side=left、side=right 的正文压到贴边块；
  - UI-157、UI-158：U1-zoom200-720-light 渲染器分段右缘越出视口，U1-zoom200-export-720-light 的 Output 值显示为「Theme…」；
  - UI-171：U2-restore-1440-light 的「Restore document · Graphite」附近没有范围说明。
- **页面实测**：UI-192 由机器检查发现，结论与证据写在该条下。UI-156 的复查写在机器检查「Code 选阶」。主 agent 没有改分、删差距或改门槛项。
- **UI-192**：来自机器检查，按影响定为 P1。本轮没有「主 agent 核实」来源的差距。
