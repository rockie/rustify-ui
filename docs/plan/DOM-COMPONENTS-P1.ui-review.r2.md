# DOM-COMPONENTS-P1 · UI 验收评审 R2

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R2（全量；重建基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [DOM-COMPONENTS-P1.ui-target.md](DOM-COMPONENTS-P1.ui-target.md) · `24f1e44d673e` | HEAD `f5613b47`，dirty，产品路径无 diff（tracked diff `2ddfa1105665`，只含 SDLC 台账、两张旧验证 PNG 与目标包「采集」节）；Studio `f83c6ff130d88c2d` http://127.0.0.1:4181/ ，catalog `07b789e3e401b12a` http://127.0.0.1:4176/ | [r2](DOM-COMPONENTS-P1.ui-captures/r2/) | [R1](DOM-COMPONENTS-P1.ui-review.md) |

## 验收判定

**需要修改 · 待裁定（UI-88、UI-89、UI-97、UI-156）**

- 本轮分数 7.1（U7），门槛 8（沿用 R1，默认值）。历史最佳 7.1（本轮重建基线，从本轮起算）。
- 未关闭的门槛项共 4 条 P1，没有 P0：
  - UI-101：200% 下粘性头封顶，关键动作不可见；
  - UI-152：Text 四档字重渲染相同；
  - UI-88、UI-89：Heading、Strong 不粗（待裁定）。
- 停滞：重建基线后的第一轮，不判停滞。

七个单元都比 R1 高，U5（9.1）、U6（9.7）、U3（8.35）已过门槛。U1、U2、U4、U7 仍低于 8，原因如下：

- **U7**：页面只加载了 IBM Plex Sans 400 字面，字重整体失效；非法属性没有可见诊断。
- **U1、U2**：新加的整块粘性头在 200% 下封顶，在 390 下漏出内容，并把来源块挤出屏外。
- **U4**：状态与决策 callout 叠放且重复，命名动作反馈仍是裸红字。

### 用户裁定（2026-10-27，本轮报告完成后追加）

本轮原判定、分数与差距不改写。三项裁定的处理如下：

- **UI-88、UI-89**：用户原话「当作缺陷补齐字面」。
  - 期望不变，Heading 与 Strong 应为粗体；两条不再是待裁定，转为可修复的 P1 门槛项。
  - 修复方处理时，与 UI-152 一起补齐 IBM Plex Sans 的静态字面：public 目录的 woff2，按 URL 引用，不嵌入 WASM。UI-09、UI-26 的 600 字重同一根因。
  - 目标包「单元」「规范要点」不改。
- **UI-97**：用户原话「改采集方式」。
  - 目标包「采集」节已改：U7 整段夹具帧（默认、110%、95%、invalid）不再用元素截图，改为页面裁切，四边向外取整到整数 CSS px，再四周各留 8px。
  - 修复方不处理此条，下一轮由评审按新采集复核。指纹不受影响。
- **UI-156**：用户原话「把接受范围扩到 U5、U7」。
  - U5、U7 比较口径已追加与 U6 相同的条款：按参考选阶或旧 token 对得到的对比度低于 4.5:1 时不计差距；实现偏离参考选阶的仍计差距。
  - 修复方不处理此条；下一轮复审标「不再适用」。
  - 这一改动把目标指纹从 `24f1e44d673e…` 改为 `567117725f56…`，R3 按重建基线评审。
- 裁定后再无待裁定事项。剩余门槛项为 UI-101、UI-152、UI-88、UI-89，都交给修复方处理。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 各评审单元分（取较低者） | R1 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 2.3 | 2.4 | 2.6 | 0.5 | 7.8 | a 8.1 · b 7.8 | 6.8 |
| U2 Studio 精调与来源解释 | 2.2 | 2.4 | 2.65 | 0.5 | 7.75 | a 7.75 | 5.8 |
| U3 Studio 导入与导出 | 2.4 | 2.65 | 2.8 | 0.5 | 8.35 | a 8.35 · b 8.45 | 7.0 |
| U4 Studio 本地库迁移与存储反馈 | 2.4 | 2.3 | 2.4 | 0.45 | 7.55 | a 7.55 · b 7.8 | 6.0 |
| U5 Studio 预览 | 2.9 | 2.85 | 2.6 | 0.75 | 9.1 | a 9.35 · b 9.1 | 6.35 |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.85 | 3.0 | 0.85 | 9.7 | a 9.7 | 7.75 |
| U7 布局与排版 | 2.5 | 1.8 | 2.3 | 0.5 | **7.1** | a 7.1 | 5.2 |

- 本轮分数：**7.1**（U7）。
- 历史最佳：7.1。R1 的 5.2 属于旧基线，不参与比较。
- 门槛：8。门槛不写入目标包，也不告诉评审。
- 单元达标：U3、U5、U6。这三个单元没有 P0/P1 门槛项，且单元分 ≥ 8。
- 各维度理由见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r2/judge.md)。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |

## 上轮差距复核

R1 的 100 条差距中：

- 已解决 65 条，不再适用 2 条（UI-85、UI-100）；
- 部分解决 26 条，仍存在 7 条，没有回归。

处置取自 [M7「UI 验收修复」](DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)。评审来源的差距由本轮评审复核；UI-98 至 UI-100 由主 agent 在运行页面上重新检查。

| 编号 | 上轮严重度与摘要 | 上轮处置（M7） | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-01 | [P1] 固定定位的「Host control」浮签压住编辑区和预览内容（U1/U2/U4/U5 · 布局） | 已修改待评审 | 已解决 | U1/U2/U4/U5 各评审：浮签不再出现，Host control 在预览末尾文档流（中文文案未采） |
| UI-02 | [P1] 选中与未选中的切换按钮几乎无法区分（U1/U5 · 视觉、交互） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-03 | [P2] 顶部缺主题名和预览模式（U1 · 布局、交互 · 可挽回 0.5） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-04 | [P2] 语言切换无间距，中文态下「EN」被裁成「E\|」（U1 · 布局、细节 · 可挽回 0.3） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-05 | [P2] 200% 文本下壳层缩放不一致（U1 · 布局 · 可挽回 0.3） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-06 | [P2] 基础配置的 Overridden 与 Custom 没有状态标识（U1 · 视觉、交互 · 可挽回 0.3） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-07 | [P2] 品牌色下拉面板只有触发器一半宽，键盘高亮为灰色（U1 · 布局 · 可挽回 0.2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-08 | [P2] 390 编辑页底部 y=832 有一条 1px 横线和 12px 色带，切断 label（U1 · 布局 · 可挽回 0.2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-09 | [P3] label、小节标题、选中标签页的字号和字重与 G3 不符（U1 · 视觉 · 可挽回 0.2） | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-10 | [P3] Undo/Redo 是描边按钮，Reset 是链接（U1 · 视觉、细节 · 可挽回 0.1） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-11 | [P3] 字段标签页下有双分隔线（U1 · 细节 · 可挽回 0.1） | 已修改待评审 | 已解决 | U1 b 用补采帧 U1-field-tabs-1440-light 判已解决（只有一条 #D0D5DD 线）；a 补采前判待核实 |
| UI-12 | [P3] 色族选项是小写，其余选项首字母大写，中文态下「pink」与已翻译项混排（U1 · 细节 · 可挽回 0.1） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-13 | [P1] token 网格看不出覆盖状态和有效值（U2 · 交互、视觉、布局 · 可挽回 0.7） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-14 | [P1] 非法输入回车后被冲回有效值，红框配在合法值上（U2 · 交互 · 可挽回 0.6） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-15 | [P1] 低对比诊断不在预览侧，失败项不醒目，不能定位来源（U2 · 交互、视觉 · 可挽回 0.6） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-16 | [P2] 来源文案不随来源类型变化，信息不完整（U2 · 细节、交互 · 可挽回 0.5） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-17 | [P2] 来源与拾色面板远离 token 网格，选中 token 时来源在屏外（U2 · 布局 · 可挽回 0.4） | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-18 | [P2] 字体与其他页层级倒置、间距缺失、label 顺序颠倒（U2 · 布局、视觉 · 可挽回 0.4） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-19 | [P2] 顶部 Reset 和字段 Reset 都没标作用域（U2 · 交互 · 可挽回 0.3） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-20 | [P2] 切到字体与其他页后，来源面板仍显示颜色 token（U2 · 交互 · 可挽回 0.2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-21 | [P2] 字段错误色是 #D92D20 而非 #b42318；出错字段焦点为 2px 无 offset，与搜索框、选择器的焦点样式三种不一致（U2 · 视觉 · 可挽回 0.2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-22 | [P3] 同级 disclosure 和按钮字号不一；标签页名「Type & other」有歧义（U2 · 细节 · 可挽回 0.15） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-23 | [P3] 分体控件两段边框色不同（#D0D5DD / #ECEEF1）（U2 · 视觉 · 可挽回 0.1） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-24 | [P1] 未知版本的导入错误给出误导性诊断，界面没有显示版本（U3 · 交互 · 可挽回 0.45 B / 0.45 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-25 | [P2] 审阅结果中的版本、名称、配置是三段无标签正文（U3 · 布局 · 可挽回 0.25 B / 0.35 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-26 | [P2] 对话框表单 label、差异表表头、备注标题都是 14px/400（U3 · 视觉 · 可挽回 0.25 B / 0.2 + 0.05 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-27 | [P2] 差异表行线、备注框、代码框描边是 #858F9E（U3 · 视觉 · 可挽回 0.2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-28 | [P2] Rust 说明没写所需 SDK/CLI 的具体版本（U3 · 交互 · 可挽回 0.2 B / 0.1 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-29 | [P2] 差异表里的颜色值是比例字体（U3 · 视觉 · 可挽回 0.15 / 0.15） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-30 | [P2] 导入错误文字是 #D92D20，文本域没有错误态边框（U3 · 视觉 · 可挽回 0.15 B / 0.2 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-31 | [P2] 配置行是 snake_case、`null`、等号串联的开发者格式（U3 · 视觉、细节 · 可挽回 0.15 / 0.15） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-32 | [P2] v1 损失清单缺 appearance 意图和 scaling 意图（U3 · 交互 · 可挽回 0.15 / 0.15） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-33 | [P2] 差异表不区分新增、删除和修改（U3 · 交互 · 可挽回 0.3 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-34 | [P2] 390 下差异表 token 列在字符中间断行（U3 · 布局 · 可挽回 0.25 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-35 | [P2] 导出对话框底部「Saved on this device」与复制反馈夹着按钮，看不出各属于谁（U3 · 交互 · 可挽回 0.1 B / 0.1 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-36 | [P3] 复制反馈插在代码框和按钮之间，按钮下移约 15px，对话框重新居中（U3 · 布局 · 可挽回 0.15 B / 0.05 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-37 | [P3] 复制成功只是一行灰色 caption，没有成功语义（U3 · 交互 · 可挽回 0.1） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-38 | [P2] 次级文字是 #667085（U3 · 视觉 · 可挽回 0.1 B / 0.15 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-39 | [P3] 字节计数「3332 / 262144 bytes」与「256 KiB」单位不统一（U3 · 细节 · 可挽回 0.1 / 0.05） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-40 | [P3] Rust 说明写 `font_context(16, true)`，代码是 `16.`；标识符用比例字（U3 · 细节 · 可挽回 0.1） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-41 | [P3] Output「Tailwind v4 CSS」与 profile「Rustify scoped」命名读起来冲突（U3 · 细节 · 可挽回 0.1） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-42 | [P3] CSS 注释中的字体路径有两种拼法（U3 · 细节 · 可挽回 0.1） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-43 | [P3] 导出表单行距 12px 与 8px 不一（U3 · 细节 · 可挽回 0.05 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-44 | [P3] 损失清单取值写法生硬（U3 · 细节 · 可挽回 0.05 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-45 | [P1] 升级 Review 塞在编辑列里，内容是原始迁移 JSON（U4 · 布局、交互） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-46 | [P1] 存储状态没有语义变体（U4 · 视觉、交互） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-47 | [P2] U4 的次级文字 #667085、错误 #D92D20、callout 边框 #858F9E 不是 G3 token；灰底上对比 4.51:1 / 4.38:1（U4 · 视觉 · 可挽回 0.3） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-48 | [P2] 配额失败的灰字与红字几乎同句重复，异常原文直接拼接（U4 · 视觉、细节 · 可挽回 0.15 B / 0.2 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-49 | [P2] 命名库动作的反馈覆盖通用状态行，不点名、不说结果，与操作列表矛盾；配额失败不说明仍可导出（U4 · 交互） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-50 | [P2] callout 没有标题，状态行与 callout 首句重复，Review 标题弱于正文（U4 · 视觉 · 可挽回 0.25） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-51 | [P2] 同级决策混用小 outline、大 outline 与纯链接；推荐的下一步反而最弱（U4 · 视觉 · 可挽回 0.25 B / 0.3 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-52 | [P2] 冲突面板先给决策再给版本信息；原文是压缩 JSON，在字符中间断行（U4 · 布局 · 可挽回 0.2 B / 0.35 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-53 | [P2] 冲突不显示双方差异（U4 · 交互 · 可挽回 0.2 B / 0.3 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-54 | [P2] 升级成功后只显示通用的「Saved on this device」（U4 · 交互 · 可挽回 0.2 B / 0.25 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-55 | [P2] 取消升级后的状态与文案过期（U4 · 交互、细节） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-56 | [P3] 操作列表计数与条目不符，「Draft · confirmed」重复且无法区分，主题名未加引号（U4 · 细节 · 可挽回 0.15 / 0.15） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-57 | [P3] 「Download original before upgrade」在升级 callout 外（U4 · 布局 · 可挽回 0.1） | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-58 | [P3] 预设选择框占位词是小写「choose」（U4 · 细节 · 可挽回 0.1 / 0.05） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-59 | [P3] 按钮名「Review / recheck upgrade」用斜杠合并两个动作（U4 · 细节 · 可挽回 0.1 / 0.05） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-60 | [P3] 版本卡「External revision 3 · Rustify Light ·」中的「·」悬挂在行尾（U4 · 细节 · 可挽回 0.1） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-61 | [P2] 状态文案大小写和句号错乱，内部前缀「storage:」暴露在用户文案中（U4 · 细节 · 可挽回 0.15 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-62 | [P3] 保留项为 0 时「Retry retained saves」仍可点（U4 · 交互 · 可挽回 0.1 B2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-63 | [P3] 冲突提示说有保留动作，实际计数是 0（U4 · 细节 · 可挽回 0.05 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-64 | [P3] 「No old bytes, backup or v2 library…」是实现行话（U4 · 细节 · 可挽回 0.05 B2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-65 | [P1] 深色预览 scope 被拆成没有内边距的深色岛（U5 · 布局、视觉、细节） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-66 | [P1] 拾取未声明的部件时没有「未标注」反馈，点击穿透并执行了真实动作（U5 · 交互） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-67 | [P1] 已声明样本的拾取轮廓比部件宽很多（U5 · 交互） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-68 | [P2] 预览对话框没有遮罩；390 下面板左右各只留 1px（U5 · 布局、视觉 · 可挽回 0.4 C2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-69 | [P2] 预览菜单相对 Actions 触发按钮左偏约 113px，并盖住边界标题（U5 · 布局 · 可挽回 0.3 C2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-70 | [P2] 拾取后来源说明不在视野内，被选中的卡片贴着视口顶边（U5 · 交互、细节） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-71 | [P2] 默认预览场景缺 IconButton、Link、Code、Kbd、Heading 等 P1 新组件，Button 只有一种尺寸（U5 · 交互 · 可挽回 0.15 C2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-72 | [P2] 相反外观只以「Compare an independent scope」出现，根内的局部边界没有外观选项（U5 · 交互 · 可挽回 0.3 C） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-73 | [P2] 深色下「Caller class override」文字对比约 1.8:1（U5 · 视觉 · 可挽回 0.25 C） | 已修改待评审 | 已解决 | U5 b 用补采帧 U5-dom-caller-1440-dark 判已解决；a 补采前判待核实 |
| UI-74 | [P3] 拾取标签压住上一行文字的下伸部（U5 · 细节 · 可挽回 0.1 C2 / 0.25 C） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-75 | [P3] GPU 色样标签写 fg、card-fg 等，与 token 名不一致（U5 · 细节 · 可挽回 0.1 C2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-76 | [P3] DOM 与 GPU 两列内容起点差 22px（U5 · 细节 · 可挽回 0.05 C2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-77 | [P1] 非 solid 变体与 Link 的文字用了填充级（第 9 级）（U6 · 视觉） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-78 | [P1] 可聚焦的「Unavailable request」焦点环几乎不可见（U6 · 交互 · 可挽回 0.5 D2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-79 | [P2] 深色 soft/surface 填充与 surface/outline 描边过暗，surface 比页面还暗（U6 · 视觉） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-80 | [P2] surface 与 outline 在浅色下完全相同（U6 · 视觉 · 可挽回 0.2 D2） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-81 | [P2] classic 渐变方向与目标相反，多了外投影（U6 · 视觉 · 可挽回 0.2 D2 / 0.25 D） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-82 | [P2] disabled 带品牌色并整体半透明，不是中性灰（U6 · 视觉 · 可挽回 0.15 D2 / 0.25 D） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-83 | [P2] loading 的 spinner 几乎不可见（约 1.2:1），底色与 disabled 相同（U6 · 交互 · 可挽回 0.3 D） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-84 | [P3] ThemePanel 中文说明「使用基础或配置值会清除该组两模式覆盖…」与英文原意不对应（U6 · 细节 · 可挽回 0.25 D2 / 0.15 D） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-85 | [P3] 〔待裁定〕 Link 焦点环是 2px 实线、无间隙，与按钮的 3px 环不一致（U6 · 细节 · 可挽回 0.1 D） | 已修改待评审 | 不再适用 | B2 已按用户裁定改为 Radix 规则；U6 a 按新 B2 逐张核验 Link 焦点符合；机器检查同 |
| UI-86 | [P1] 夹具缺目标图的主体：字级梯度、字重、变体矩阵、wrap、trim（U7 · 视觉、布局、交互） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-87 | [P1] 截图里看不到 Inset 贴边（U7 · 布局） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-88 | [P1] Heading 不是粗体（U7 · 视觉 · 可挽回 0.5 C2 / 0.2 C） | 不采纳 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-89 | [P1] Strong 没有加粗（U7 · 视觉 · 可挽回 0.3 C2 / 0.25 C） | 不采纳 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-90 | [P2] Code、Kbd 的样例不是 L1 默认变体：Code 呈 outline 外观，Kbd 是平铺的 soft 外观（U7 · 视觉） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-91 | [P2] 非法属性后页面上看不到属性定位诊断（U7 · 交互 · 可挽回 0.5 C2 / 0.5 C） | 已修改待评审 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-92 | [P2] Reset 只有一个蓝字、缩进 8px 的按钮样例，无法看出作用范围（U7 · 布局、交互、细节 · 可挽回 0.4 C2 / 0.15 C） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-93 | [P2] Container 只看得到 1136px 一档，灰带左右不对称且没有标注（U7 · 布局 · 可挽回 0.4 C2 / 0.2 C） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-94 | [P2] Quote 不是斜体（U7 · 视觉 · 可挽回 0.2 C2 / 0.15 C） | 已修改待评审 | 已解决 | 本轮评审逐条复核，见对应单元原文的复核表 |
| UI-95 | [P3] 样例都没有标注 props（U7 · 细节 · 可挽回 0.2 C2） | 已修改待评审 | 部分解决 | 见本报告差距清单同编号条目及其来源 |
| UI-96 | [P3] Blockquote 左边框固定 4px，不随字号变化（U7 · 细节 · 可挽回 0.15 C） | 不采纳 | 已解决 | U7 a：size 2/4 边框 4px、size 7 为 7px，判已解决 |
| UI-97 | [P3] 夹具根元素顶边混入外来的 1px 灰色线段（U7 · 细节 · 可挽回 0.1 C） | 不采纳 | 仍存在 | 见本报告差距清单同编号条目及其来源 |
| UI-98 | [P1] 预览工具栏的描边按钮文字对比 4.37:1，低于 AA（U1/U5 · 视觉 · 机器检查） | 已修改待评审 | 已解决 | 主 agent 机器检查：Studio 1440/768/390 axe 不再报工具栏按钮 |
| UI-99 | [P2] 语言切换「EN」的点击目标被「中文」遮住，剩 16.2×29px（U1 · 布局 · 机器检查） | 已修改待评审 | 已解决 | 主 agent 机器检查：EN 50×38，中英两态中心命中自身 |
| UI-100 | [P2] 〔待裁定〕色族 solid 按钮与语义 accent 对的文字对比低于 AA（U6 · 视觉 · 机器检查） | 不采纳 | 不再适用 | 用户已接受（目标包 U6 比较口径）；catalog 色族 solid 与 accent 对仍低于 4.5:1，属已接受口径 |

## 差距清单

共 89 条：P1 4 条、P2 22 条、P3 63 条，没有 P0。其中评审来源 88 条，机器检查 1 条（UI-156）。按 P0 → P3 排列，同一严重度内按可挽回分从高到低。

- 「来源」中的 a、b 指本单元的两份评审原文：[judge-U<n>-a.md](DOM-COMPONENTS-P1.ui-captures/r2/) 为首批，`judge-U<n>-b.md` 为补采后重评。
- 每条完整的取样数值和修法以原文为准。
- 截图在 [r2](DOM-COMPONENTS-P1.ui-captures/r2/) 目录中，文件名即证据位置。

### UI-101 · [P1] 200% 文本下粘性头被封顶为内滚动区，「Preview mode」被切半，模式、撤销、保存、导出不可见（U1 · 布局、交互 · 可挽回 a 0.55 / b 0.5）

- 位置：U1-zoom200-720-light 编辑列 y=0–449；U1-zoom200-export-720-light 遮罩后方头区已内部滚动。
- 现状 → 期望：头区限高约 50vh，关键动作只能靠键盘焦点带动内滚动露出 → G5 顶部保留主题名、模式、撤销/重做/保存/导出；G6 200% 下主路径可达，不靠隐藏的嵌套滚动。
- 修法：去掉头区 `max-height`/`overflow`；只让一条紧凑命令条吸顶（主题名 + 状态、Preview mode、Undo、Redo、Save、Export，其余收进 More），产品名、副标题、语言切换放回文档流；高度超过面板约 40% 时降为非吸顶。
- 来源：a 新-2；b 新-1（同一现象、同一帧、同一坐标 y=449）。
- 核对说明（主 agent，页面实测，不改分）：720×900 + 根字号 200% 下，吸顶头区为 `div.studio-editor-top`，`position: sticky; max-height: 450px; overflow-y: auto`，scrollHeight 964 / clientHeight 449；指针在头区上滚轮 200px 后 scrollTop 0→200，「Preview mode」标签从 top 432 移到 232。即头区可用滚轮滚动，但截图里没有可见的滚动提示，首屏仍裁掉一半标签。

### UI-88 · [P1] 〔待裁定〕Heading 不是粗体（U7 · 视觉 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-type-heading-matrix-1280/390、U7-scale110/95-heading，夹具「Native structure, flexible composition」。
- 现状 → 期望：60px 下 Heading 笔画 7px、Text 6px；目标 Heading 9px、Text 5px（L1 正确语义外观）。
- 修法：Heading 默认 bold（700）并确认 700 字面可加载，验收为 60px 下 Heading 笔画约为 Text 的 1.5 倍以上。
- 来源：a UI-88（仍存在）。
- 核对说明（主 agent，页面实测，不改分）：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r2/machine-check.json) `fonts` 显示，Text 四行计算字重为 300/400/500/700，Heading（h2）与 Strong 为 700，`font-synthesis-weight: auto`；但 `document.fonts` 中 IBM Plex Sans 只有 weight 400 一个字面（loaded），没有 300/500/600/700。上轮修复记录「计算样式均为 700」属实，评审看到的「不粗」也属实：浏览器只能用 400 字面渲染或合成粗体。修复方向是补齐 Plex 的静态字面（public 目录 woff2，按 URL 引用）。

### UI-152 · [P1] Text 的四档字重渲染完全相同（U7 · 视觉 · 可挽回 a 0.5）

- 位置：U7-type-text-matrix-1280-light y=537–666 的 weight=light/regular/medium/bold 四行；390 同。
- 现状 → 期望：四行墨量 0.8541/0.8541/0.8541/0.8536 → L2 字重 light/regular/medium/bold 依次加重（比较口径「字重」）。
- 修法：先查四行 computed `font-weight`；数值相同则补齐字重 class 并 `mbx xtask css`；数值不同但渲染相同则补齐 IBM Plex Sans 300/400/500/700 的 `@font-face`（public 静态 woff2）。修后 UI-88、UI-89 应同时恢复。
- 来源：a 新-1。
- 核对说明（主 agent，页面实测，不改分）：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r2/machine-check.json) `fonts` 显示，Text 四行计算字重为 300/400/500/700，Heading（h2）与 Strong 为 700，`font-synthesis-weight: auto`；但 `document.fonts` 中 IBM Plex Sans 只有 weight 400 一个字面（loaded），没有 300/500/600/700。上轮修复记录「计算样式均为 700」属实，评审看到的「不粗」也属实：浏览器只能用 400 字面渲染或合成粗体。修复方向是补齐 Plex 的静态字面（public 目录 woff2，按 URL 引用）。

### UI-89 · [P1] 〔待裁定〕Strong 没有加粗（U7 · 视觉 · 仍存在 · 可挽回 a 0.2）

- 位置：U7-type-code-matrix-1280「strong importance」墨量 0.8595 对相邻 0.8577；夹具「Strong importance」。
- 来源：a UI-89（仍存在）。
- 核对说明（主 agent，页面实测，不改分）：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r2/machine-check.json) `fonts` 显示，Text 四行计算字重为 300/400/500/700，Heading（h2）与 Strong 为 700，`font-synthesis-weight: auto`；但 `document.fonts` 中 IBM Plex Sans 只有 weight 400 一个字面（loaded），没有 300/500/600/700。上轮修复记录「计算样式均为 700」属实，评审看到的「不粗」也属实：浏览器只能用 400 字面渲染或合成粗体。修复方向是补齐 Plex 的静态字面（public 目录 woff2，按 URL 引用）。

### UI-17 · [P2] 选中 token 后来源与解除按钮不在屏内（U2 · 布局、交互 · 仍存在 · 可挽回 a 0.55）

- 位置：U2-source-overridden-1440-light 选中 primary 后整屏看不到「Fine adjustment source」；U2-source-overridden-detail-1440-light 来源块在整片网格之后（Shadow 组后）；U2-source-overridden-390-light 同样。1440 编辑列 0–315px 为常驻壳层。
- 现状 → 期望：要滚过约 50 个 token 才能读到来源 → F1 展示有效值、来源、覆盖与重置；F2 来源回答三个问题。
- 修法：来源 + 拾色块作为整行插入所选格所在行下方（`grid-column: 1 / -1`），或做成编辑列内 `position: sticky; bottom: 0` 的可折叠抽屉；选中时 `scrollIntoView({block:'nearest'})`；同时压缩常驻区（见 UI-101/UI-103）。
- 来源：a UI-17。

### UI-91 · [P2] 非法属性后看不到属性定位诊断（U7 · 交互 · 仍存在 · 可挽回 a 0.5）

- 位置：U7-invalid-1280-light 与默认态逐像素比对，只有该样例重新排版；U7-invalid-detail-1280-light 视口里没有诊断。
- 现状 → 期望：L6 非法值给出属性定位诊断，组件默认保持可用。
- 修法：样例下方渲染 `role="status"` 诊断行（属性、非法值、回退默认值），Restore 后消失。
- 来源：a UI-91。

### UI-125 · [P2] 差异表放在限高的内层滚动区，Apply 前看不全改动，且无滚动提示（U3 · 布局、交互 · 可挽回 a 0.45 / b 0.35）

- 位置：U3-import-review-end-1440-light 在 y≈715 把「Dark · overrides (1)」拦腰截断；U3-import-review-end-390-light 只完整露出 2/6 项；U3-import-review-bottom-1440/390 显示内层（`div.tools-scroll`，`overflow-y:auto`）滚到底后的内容；U3-actions.log 记录两层滚动。
- 现状 → 期望：对话框本体与差异表两层纵向滚动 → X1 Apply 前完整呈现候选差异；X6 对话框内部只有一层纵向滚动。
- 修法：去掉审阅中 `.tools-scroll` 的 `max-height`/`overflow`，表格随对话框本体滚动；极长差异才启用内层滚动，并加滚动阴影与「Showing N of M changes」；≤560px 不嵌套。
- 来源：a 新-1；b 新-1（同一容器、同一帧）。补采确认容器是 `overflow-y:auto`，a 提出的「若是 hidden 应升 P1」不成立。

### UI-139 · [P2] 状态 callout 与决策 callout 叠放、内容重复，状态区与决策区的顺序随状态颠倒；冲突面板超出可视高度（U4 · 布局、视觉、交互 · 可挽回 a 0.35 / b 0.15）

- 位置：U4-upgrade-pending-1440-light y316–439 与 y452–778 两只琥珀 callout 都说「review the complete library」，首屏没有编辑控件；U4-upgrade-cancelled 同样重复「Memory only」「reopening restores」；U4-conflict-after-1440-light-a2 状态 callout 排在冲突 callout 之后；U4-conflict-detail-1440-light-a3 冲突面板约 620px，超出粘性头下约 586px 可视高度。
- 现状 → 期望：S1 每个异常状态一处常驻反馈并紧跟继续动作；S2、S5。
- 修法：存储区固定 slot：①状态与决策 callout（每状态一张）→ ②命名动作反馈 → ③操作计数 → ④备份；待确认、取消、冲突各合并为一张 callout；冲突面板压到 1440×900 可同屏看到决策与下载。
- 来源：a 新-1；b UI-50 中的叠放与重复部分、b 新-2（顺序与面板高度）。

### UI-140 · [P2] 命名动作反馈是裸红字、位置不固定；被流程阻止的保存也用错误红，升级成功后仍残留且无内联动作，时态不对（U4 · 视觉、交互、布局 · 可挽回 a 0.25 / b 0.35）

- 位置：U4-upgrade-cancelled y453–486、U4-upgraded y443–476（紧贴绿色成功 callout）、U4-quota-failure y537–570、U4-no-locks y485–518。
- 现状 → 期望：S4 命名动作单独反馈且结果真实；S5 未处理的命名动作保持可见；S1、S6。
- 修法：固定位置的 inline alert；被阻止用警告样式、真实写失败用错误样式；附「Save “…” now」「Dismiss」；升级成功后降为中性提示；无锁文案改现在时。
- 来源：a 新-2；b 新-1。

### UI-50 · [P2] callout 标题与对话框分节标题弱于正文，两类 callout 正文字号不一（U4 · 视觉 · 部分解决 · 可挽回 a 0.2 / b 0.3 中的标题部分）

- 位置：「Review the library upgrade」「Memory only · upgrade cancelled」「Choose which version to keep」为 13px 常规 #475467，正文 14px #1D2939；状态 callout 正文 12px；对话框「Saved themes retained (2)」等分节标题同为 13px 常规。
- 修法：callout 标题 13px/600、按变体着色；正文统一字号与 #475467 或 #1D2939；对话框分节标题 13px/600 #1D2939。
- 来源：a UI-50；b UI-50（叠放部分归 UI-139）、b 新-3 的分节标题部分。

### UI-147 · [P2] 拾取信息面板在文档流里：开启时预览下推 175px，拾取较低部件时面板不在视野（U5 · 交互、布局 · 可挽回 b 0.3）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light「DOM」标签 y=244→419；U5-inspector-undeclared-hover/-undeclared 看不到面板；U5-inspector-undeclared-panel-1440-light 需滚回顶部才看到「Not annotated · …」。
- 现状 → 期望：P2 信息面板带 token 按钮，从实际部件定位角色与来源。
- 修法：面板改为预览滚动容器内的 sticky 层或底部 overlay，不占文档流；或把 token 按钮做成锚在拾取标签旁的小浮层。
- 来源：b 新-1（a 在建议 5 提出下推 175px，未计分）。

### UI-26 · [P2] 对话框 label、表头、节标题字重仍为 400（U3 · 视觉 · 部分解决 · 可挽回 a 0.25 / b 0.25）

- 位置：导入的 Input format、Theme source、Import candidate、Version、Name、配置字段名、Changes (6)、表头、Import notes；导出的 Output、CSS profile、Color notation、Exported code、Required source revision。
- 现状 → 期望：13px/400/#475467 → G3 label 13px/600。
- 修法：数据工具对话框的 label、`dt`、`th`、节标题统一 `font-weight:600`（并确认 600 字面已加载，见 UI-09）。
- 来源：a UI-26；b UI-26。
- 核对说明（主 agent）：根因同 UI-152 的核对说明，页面只加载了 IBM Plex Sans 400 字面，600 无法呈现。

### UI-51 · [P2] 推荐动作与放弃动作同权；编辑列有多套描边按钮（U4 · 视觉、交互 · 部分解决 · 可挽回 a 0.2 / b 0.25）

- 位置：U4-upgrade-pending「Review upgrade / Download original data / Cancel upgrade」同为蓝边灰字；壳层 Import/Export 为蓝边蓝字；空闲 Retry/Cancel 为 #858F9E/#667085。
- 修法：每张 callout 一个实心推荐动作，其余统一壳层描边，放弃动作放末尾改文字按钮。
- 来源：a UI-51；b UI-51。

### UI-102 · [P2] 390 编辑页的 Edit/Preview 标签条与粘性头之间有空隙，滚动内容从中透出（U1/U2 · 布局 · 可挽回 U1 a 0.2 / b 0.15，U2 a 0.25）

- 位置：U1-configured-390-light y≈45–85（「Presets … 44」半截）；U2-source-overridden-390-light y≈45–85（「sidebar-border #d0d0d0」被上下两层切开）。
- 现状 → 期望：两层吸顶之间漏出内容 → G1、G2（≤600px 编辑/预览分区稳定，不遮挡、不截断）。
- 修法：标签条与头区放进同一个不透明粘性容器，或头区 `top` 设为标签条高度并把间距改为粘性元素自身的 `padding-top`；或 ≤600px 时头区不吸顶。
- 来源：U1 a 新-1、b 新-2；U2 a 新-1（同一机制：两层吸顶之间的空隙，同在 390、同 y 范围）。

### UI-153 · [P2] 非法属性样例默认态塌成一列一个词，上方有孤立的 `px`（U7 · 布局、细节 · 可挽回 a 0.25）

- 位置：U7-fixture-1280-light 约 y=1395–1590；U7-fixture-390-light 约 y=1940–2120；U7-invalid-* 中「px」仍在。
- 修法：默认态给合法值并标注（如 `width=240px`），补全或删除「px」。
- 来源：a 新-2。

### UI-02 · [P2] 未选分段带 #8AB7F7 蓝内框、段间 3px 双线，焦点环压到相邻段（U1/U5 · 视觉、交互、细节 · 部分解决 · 可挽回 U1 a 0.2 / b 0.15，U5 a 0.15 / b 0.15）

- 位置：U1-default-1440-light 的 Light/Dark、DOM/GPU/DOM + GPU、Inspect；U1-keyboard-04 Dark 焦点；U5-dom-1440-light、U5-compare-nested、U5-gpu-compare 工具栏。
- 现状 → 期望：未选段在 #D0D5DD 外框内多一圈 1px #8AB7F7，EN/中文、Edit/Preview 没有；选中段另加 2px 下划线 → G3 蓝色只用于选中与焦点；G4 选中的模式/渲染器按钮用按下态。
- 修法：所有分段控件共用一个组件：未选段白底 #475467，去掉 #8AB7F7；段间单条 1px #D0D5DD；焦点环内收或 `z-index:1`，不压邻段。
- 来源：U1 a UI-02、b UI-02；U5 a UI-02（P2）、b UI-02（P3）。严重度取 P2。

### UI-92 · [P2] Reset 只覆盖 button、h2、ul 三种元素（U7 · 布局、交互 · 部分解决 · 可挽回 a 0.2）

- 位置：U7-layout-reset-comparison-1280/390；对照 radix-reset 约 60 种原生元素。
- 修法：沿用「原生 / Reset」双行，至少补 a、strong、em、code、kbd、mark、sub、sup、input、select、textarea、fieldset、ol、dl、table、pre、blockquote、img。
- 来源：a UI-92 余项。

### UI-141 · [P2] 冲突的三个决策没有说明后果（U4 · 交互 · 可挽回 a 0.2）

- 位置：U4-conflict-1440-light「Load other version / Keep current draft / Save current as a copy」。
- 修法：每个按钮下加一行后果说明，或改单选组加 Apply（S5）。
- 来源：a 新-3。

### UI-104 · [P2] 壳层次级与禁用按钮有 4 套样式，闲置按钮用焦点同色 #8AB7F7 做边框，禁用按钮比可用按钮更重（U1 · 视觉 · 可挽回 a 0.15 / b 0.1）

- 位置：U1-default-1440-light、U1-configured-1440-light 编辑列：Undo/Restore（#8AB7F7 边 #475467 字）、Import/Export（#8AB7F7 边 #1570EF 字）、Cancel pending writes/Retry（#858F9E 边 #667085 字）、Focus preview（#D0D5DD 边）；禁用 Redo 为 #858F9E 边、#F2F4F7 底。
- 现状 → 期望：闲置控件像半选中，禁用比可用更醒目 → G3 壳层扁平、蓝色只给选中与焦点、分隔 #D0D5DD/#EAECF0、次级文字 #475467。
- 修法：统一 ShellButton 次级样式（白底、1px #D0D5DD、#344054 或 #475467 字）与一套禁用样式（#F9FAFB 底、#EAECF0 边、#98A2B3 字）；#8AB7F7 只留给 `:focus-visible`。
- 来源：a 新-5；b 新-4（同一按钮体系、同样引用 #858F9E 非 token 边框）。

### UI-115 · [P2] 网格内 ↺ 只有图标，看不到解除范围（U2 · 交互 · 可挽回 a 0.15）

- 位置：U2-source-overridden-1440-light 及 detail 图每个 token 格右侧 ↺ 段。
- 现状 → 期望：来源块写「Clear primary · Light only」，网格里同一动作只有图标 → F2 解除须显示范围；C3 不只靠颜色或悬停。
- 修法：胶囊文字带模式（「Override · Light」）；↺ 加与来源块同文案的 `aria-label`；未覆盖格 `aria-disabled`。
- 来源：a 新-8。

### UI-126 · [P2] 切换 Output 时对话框重新垂直居中，选择器跳动 12–109px（U3 · 布局 · 可挽回 a 0.15）

- 位置：U3-export-json/v1/css/rust 对话框上沿 179/70/107/95，Output 选择器 241/132/169/157；导入错误态与审阅态上沿 148/17。
- 现状 → 期望：X6 对话框宽高为视口减 2rem、内部纵向滚动，切换内容时头部与已操作控件不动。
- 修法：导入导出对话框 `height: calc(100dvh - 2rem)` 正文内滚，或顶部锚定（`align-items:flex-start; margin-top:1rem`）。
- 来源：a 新-9（b 在建议 8 提出同一现象，未计分）。

### UI-127 · [P2] 390 卡片模式下分组行只铺约 105px 并折行（U3 · 布局 · 可挽回 a 0.1 / b 0.15）

- 位置：U3-import-review-end-390-light「Identity (2)」「Light · base (1)」；U3-import-review-bottom-390-light「Dark · / overrides (1)」折两行。
- 修法：≤560px 时分组行 `display:block; width:100%`（Grid 用 `grid-column:1/-1`）。
- 来源：a 新-2；b 新-2。

### UI-128 · [P2] 配置只列导入值，没有与当前主题的对比（U3 · 交互 · 可挽回 b 0.15）

- 位置：U3-import-review-1440-light y=656–791、U3-import-review-390-light 六个配置字段只有一个值。
- 现状 → 期望：X1 导入先呈现候选差异——版本、名称、配置……
- 修法：每个字段显示「当前 → 导入」，变化的加「Changed」，配置改动并入 Changes 计数。
- 来源：b 新-3。

### UI-103 · [P2] 粘性头过高，占视口 31–38%，挤占编辑区（U1 · 布局 · 可挽回 b 0.1）

- 位置：1440/768 各帧 y=0–314；390 标签条加头区 y=0–318。
- 现状 → 期望：头区 8 行 → G2 编辑区独立滚动；G5 只要求保留主题名、模式与撤销/重做/保存/导出。
- 修法：同 UI-101 的单行命令条方案，1440 下吸顶区控制在约 96px 内。
- 来源：b 新-3（a 在建议 1 提出同一点，未计分）。U2 a 在 UI-17 中也指出常驻区压缩可用高度。

### UI-148 · [P2] 拾取矩形在蓝色部件上看不出（U5 · 视觉 · 可挽回 a 0.1）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light「Create profile」：2px #1570EF 轮廓贴着同色按钮。
- 修法：保持固定蓝色，加 `outline-offset: 2px` 与内侧 2px 白色光环（P2）。
- 来源：a 新-4（b 在建议 8 提出同一点，未计分）。

### UI-156 · [P2] 〔待裁定〕Code 变体在 U5 预览与 U7 夹具中的文字对比低于 4.5:1，与 UI-100 同属「参考选阶 / 旧 token 对」冲突（U5/U7 · 视觉 · 机器检查）

- 位置：
  - U5：Studio 预览「Component samples」中的 Code（soft）「ThemeDocument」，#1570EF 压 #E3EEFD，3.89:1（axe，1440 与 768）；对应 U5-dom-1440-light 等帧。
  - U7：布局夹具 Code 矩阵 solid · gray「console.log()」，#FFFFFF 压 #8D8D8D，3.31:1（axe，1280 与 390）；对应 U7-type-code-matrix-*。
- 现状 → 期望：两处都是按参考选阶（solid 白字压第 9 阶）或作者主题旧 token 对（soft 用 primary 压其浅色）得到的配色。用户对 UI-100 的接受写在 U6 的比较口径里，只覆盖 Button/IconButton/Link；U5、U7 的比较口径没有这条，而评审标尺把主路径 AA 不足定为 P1。
- 处理：待用户决定是否把 UI-100 的口径扩展到 U5、U7 的 Code（及其他 P1 排版组件）。修复方先不处理。
- 来源：机器检查（@axe-core/playwright 4.13.0）。

### UI-09 · [P3] 英文 label、预览标题、选中字段标签页没有呈现 600 字重（U1 · 视觉、细节 · 仍存在 · 可挽回 a 0.15 / b 0.2）

- 位置：U1-configured-1440-light 的 Appearance、Brand color、Neutral color、Preview mode、Presets；U1-default-1440-light 预览「Preview」；U1-field-tabs-1440-light「Colors」。中文 label「外观」「预览模式」明显是粗体。
- 现状 → 期望：字号已是 13px，字重仍为常规 → G3 label 13px/600、预览标题 16px/600；G4 选中标签页 13px 半粗。
- 修法：确认 label 类为 600，并确认 IBM Plex Sans 600 字面以独立 woff2 从 public 加载（`document.fonts` 中有 600 条目）。
- 来源：a UI-09（部分解决）、b UI-09（仍存在），取仍存在。
- 核对说明（主 agent）：根因同 UI-152 的核对说明，页面只加载了 IBM Plex Sans 400 字面，600 无法呈现。

### UI-70 · [P3] 拾取后编辑区滚动落点被粘性页头遮住，来源标题不在视野、第一行切半（U5 · 交互、布局、细节 · 部分解决 · 可挽回 a 0.15 / b 0.2）

- 位置：U5-inspector-picked-1440-light 左栏 y≈300–330。
- 修法：滚动目标改为来源块标题；滚动容器 `scroll-padding-top` = 页头高度 + 16px。
- 来源：a UI-70；b UI-70。

### UI-12 · [P3] 自动配对的中性色名显示为小写原始 id 且不翻译，与品牌色显示规则不一致（U1/U6 · 细节 · 部分解决 · 可挽回 U1 a 0.1 / b 0.05，U6 a 0.15）

- 位置：U1-configured-1440-light「Resolved neutral: mauve.」；U1-zh-configured-1440-light「已解析中性色：mauve。」；U6-fixture-1280-light「Resolved neutral: slate.」；U6-panel-zh-1280-light「已解析中性色：slate。」（品牌色已显示「靛蓝色」/「Indigo」）。
- 修法：auto 结果与选项列表共用同一个本地化显示名函数。
- 来源：U1 a UI-12、b UI-12；U6 a 新-1（同一 ThemePanel 字段的同一显示规则问题）。

### U2

### UI-18 · [P3] 小节标题与字段 label 同样式，层级平（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-controls-1440-light「Typography」对「Sans font stack」；U2-field-error-1440-light「Geometry」对「Corner radius」，同为 #1D2939、同字号字重。
- 修法：小节标题用壳层分组眉题样式（12px/600 大写、#475467，上分隔线 24px、下 12px），字段 label 保持 13px/600。
- 来源：a UI-18 余项。

### UI-22 · [P3] 诊断区嵌套标题倒置、正文大于标题、等宽字号混用（U2 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U2-low-contrast-1440-light 编辑列 y≈352–700：外层 summary 约 12px、内层约 13px；「Authored colors fit…」约 14px 大于其标题；字体诊断条目之间无间距。
- 修法：两级 summary 统一 13px/600、内层缩进 12px；正文 12px #475467；token 名统一 12px 等宽；条目间 8px 加分隔线。
- 来源：a UI-22 余项。

### UI-40 · [P3] Rust/CSS 说明里的标识符只有部分等宽，`16.0` 与代码 `16.` 不一致（U3 · 视觉、细节 · 部分解决 · 可挽回 a 0.15 / b 0.05）

- 位置：U3-export-rust-1440-light 第 4 行调用表达式，第 2 行 theme.json、theme.rs、AppHandle；U3-export-css 说明行 data-theme=dark、.dark。
- 来源：a UI-40；b UI-40。

### UI-81 · [P3] classic 渐变幅度只有目标约 1/3，主体与 solid 几乎同色（U6 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U6-fixture-1280-dark/-light classic 行，以 classic 4 的 x=500 列为准；对照 radix-sink-button classic 4 的 x=420 列。
- 现状 → 期望：方向、顶部 1px 高光、底部 1px 暗线、无外投影已对；深色下内容区相对 solid 仅 −1…+6，目标为 −5…+17。
- 修法：只调 classic 背景渐变层的亮度差约为现在 3 倍，其余保持。
- 来源：a UI-81。

### U7

### UI-95 · [P3] 布局原语区仍有样例没有 prop 标注（U7 · 细节 · 部分解决 · 可挽回 a 0.15）

- 位置：U7-fixture-1280 约 y=120–1380；Section 只有一条无标注灰带。
- 来源：a UI-95 余项。

### UI-105 · [P3] 同一行壳层控件高度有 28/36/38px 三档（U1 · 布局、视觉、细节 · 可挽回 a 0.15 / b 0.05）

- 位置：1440 预览工具栏 Cards/Responsive 36px、分段与 Inspect 28px、Focus preview 38px；头区 EN/中文 38px、Light/Dark 28px；768/390 第二行。
- 修法：定义 `--shell-control-h: 2rem`，工具栏与头区所有控件统一，`align-items:center`。
- 来源：a 新-4；b 新-6。

### UI-106 · [P3] 编辑区顶部 padding 16px、两栏标题不对齐，组内状态行与按钮间距 0px（U1/U2 · 布局、细节 · 可挽回 U1 a 0.15 + 0.1 / b 0.15，U2 a 0.05）

- 位置：U1-default-1440-light 编辑列首元素顶在 y=16，左右标题中线差约 8px；U1-configured-1440-light「• Overridden」胶囊底 y=745 与「Use configuration values · both modes」顶 y=746 相接；U2-group-reset-1440-light Brand color 下「● No overrides」胶囊与按钮相距约 1–2px。
- 现状 → 期望：G2 编辑区 padding 24px 20px 48px、栈间距 16px。
- 修法：编辑区（或粘性头内）`padding-top: 1.5rem`；标题行 `align-items:center`；配置组内 `display:grid; gap:.5rem`，状态行与按钮至少 8px。
- 来源：U1 a 新-6、a 新-9、b 新-7（b 把两点并为一条）；U2 a 新-3（同一组件：配置组状态行与解除按钮贴合）。

### UI-142 · [P3] 升级 Review 对话框栅格不齐：值列错位 25px、主题表无表头、双细线（U4 · 布局、细节 · 可挽回 a 0.15 / b 0.15）

- 位置：U4-upgrade-review-1440-light 值列 x≈360 对 x≈333；主题表三列无表头；y≈350 与 y≈363 两条线。
- 来源：a 新-5；b 新-3（分节标题部分并入 UI-50）。

### UI-143 · [P3] 无锁态：标题写「saving paused」，主按钮仍实心，正文是行话、缺句号（U4 · 交互、细节 · 可挽回 a 0.15 / b 0.1）

- 位置：U4-no-locks-initial-1440-light y369–413；页头「Save locally」实心；U4-no-locks「not saved earlier」。
- 来源：a 新-7；b 新-4。UI-61 的句号问题同在此处。

### UI-144 · [P3] 文案细节：修订号无标签、「Saved themes 0 → 0」、行话与含混措辞（U4 · 细节 · 可挽回 a 0.15 / b 0.05）

- 位置：U4-conflict「This tab 2 · Rustify Light」；U4-upgraded「the original download remains available」；对话框「Ready for explicit upgrade confirmation.」；U4-conflict-raw「Download displayed original」；「Secure Web Locks unavailable」。
- 来源：a 新-10（单复数部分归 UI-56）；b 新-7（修订号标签）。

### UI-149 · [P3] 拾取标签截掉 token 列表；较高部件的标签落在视口外（U5 · 细节、交互 · 可挽回 a 0.05 / b 0.15）

- 位置：U5-inspector-1440-light / -picked「primary, primary-f…」；U5-inspector-card-1440-light 卡片底边在视口外。
- 来源：a 新-1；b 新-2。

### UI-03 · [P3] 390 预览页顶部没有预览模式切换与保存/导出（U1 · 交互 · 部分解决 · 可挽回 b 0.1）

- 位置：U1-preview-390-light 顶部只有「Graphite · Light preview」文字。
- 修法：UI-101 的紧凑命令条放在 Edit/Preview 标签条下，两个标签页共用；或预览工具栏首行放 Light/Dark 与溢出菜单。
- 来源：b UI-03（部分解决）；a 判已解决。取部分解决。

### UI-46 · [P3] 成功态容器不着色；同一琥珀底上两种标题色（U4 · 视觉 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgraded 成功 callout 为 #F9FAFB/#D0D5DD，只有标题 #067647；「Memory only · saving paused」标题 #93370D，其他琥珀 callout 标题 #475467。
- 来源：b UI-46（部分解决）；a 判已解决。取部分解决。

### UI-47 · [P3] 禁用或空闲的存储操作按钮用非 token 色 #667085/#858F9E，且与 Undo 的禁用样式并存（U4 · 视觉 · 部分解决 · 可挽回 a 0.1 / b 0.05）

- 来源：a UI-47；b UI-47。

### UI-55 · [P3] 取消后 callout 第一段沿用待确认时的文案（U4 · 交互、细节 · 部分解决 · 可挽回 a 0.1）

- 位置：U4-upgrade-cancelled y553–637 与 U4-upgrade-pending y505–589 逐字相同。
- 来源：a UI-55（部分解决）；b 判已解决。取部分解决。

### UI-56 · [P3] 操作列表：「1 recent operations」单复数、「edit 2」孤字、失败项与成功项同色、说明段答非所问（U4 · 细节、视觉 · 部分解决 · 可挽回 a 0.1 / b 0.1）

- 位置：U4-upgrade-cancelled、U4-no-locks「0 pending · 1 recent operations」；U4-quota-operations y750–768、y612–680。
- 来源：b UI-56（部分解决）；a 新-9 与 a 新-10 的单复数部分（同一操作列表）。a 判 UI-56 已解决，但另列同一问题为新差距。取部分解决。

### UI-57 · [P3] 升级成功后「Download original before upgrade」在成功 callout 之外，且与其他状态的「Download original data」不同名（U4 · 布局、细节 · 仍存在 · 可挽回 a 0.1 / b 0.1）

- 位置：U4-upgraded y490–517。
- 来源：a UI-57（仍存在）；b UI-57（部分解决）。取仍存在。

### UI-62 · [P3] 没有待处理项时 Retry / Cancel / Save current draft 看起来可点（U4 · 交互、视觉 · 部分解决 · 可挽回 b 0.1）

- 位置：U4-upgrade-pending y≈830–890、U4-no-locks y≈571–597。
- 来源：b UI-62（部分解决）；a 判已解决（视觉）并注明是否真正禁用看不出。取部分解决。

### U5

### UI-86 · [P3] Heading wrap 行没有收窄宽度，1–3 档多行行高无法比对（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-type-heading-matrix-1280 size=7·wrap 至 size=1·wrap 只有一行；390 下 3–1 档一行。
- 来源：a UI-86 余项（主体已补齐，余项按 P3 计）。

### UI-87 · [P3] Inset 的 left、right 贴边没有干净的展示（U7 · 布局 · 部分解决 · 可挽回 a 0.1）

- 位置：U7-layout-inset-demonstration 只有 top/bottom；U7-fixture-1280 约 y=1015–1320 的 8 个 side 样例都带 `margin=3px`。
- 来源：a UI-87 余项。

### UI-97 · [P3] 〔待裁定〕夹具根顶边混入外来灰线（U7 · 细节 · 仍存在 · 可挽回 a 0.1）

- 位置：U7-fixture-1280-light 第 0 行 x=138–288、295–374（rgb 118–161）；390 图没有。
- 来源：a UI-97（仍存在）。

### UI-107 · [P3] 次级文字用 #667085、颜色搜索框边框用 #D0D5DD，偏离 G3 token（U1/U2 · 视觉 · 可挽回 U1 a 0.1 / b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light 的 Theme configuration 说明、Appearance/Brand color 帮助文字、「Light 6 · Dark 6」、「Resolved neutral: mauve.」；U2-controls-1440-light「Choose a bundled font」caption；U2-source-overridden-1440-light「Search colors…」边框。
- 现状 → 期望：两种次级灰并存，颜色搜索框与预设搜索框边框不同 → G3 次级文字 #475467、搜索框边框 #98A2B3。
- 修法：helper/caption 统一 `--shell-text-secondary: #475467`；颜色搜索框复用预设搜索框的 #98A2B3 边框；#667085 只留给禁用态。
- 来源：U1 a 新-3、b 新-5；U2 a 新-6（同一 G3 token 偏差）。

### UI-109 · [P3] 状态胶囊（Overridden、Custom、网格 Override）借用选中态蓝色（U1/U2 · 视觉 · 可挽回 U1 b 0.1，U2 a 0.1）

- 位置：U1-configured-1440-light、U1-configured-lower-1440-light「• Overridden」「• Custom」（#EFF8FF/#84CAFF/#175CD3）；U2-source-overridden-1440-light 网格「Override」胶囊（选中的 primary 格内只剩一道描边）。
- 现状 → 期望：状态与选中、动作同色 → G3 蓝色用于选中与焦点；C3 状态以文字 + 标识表达（已满足）。
- 修法：状态胶囊改中性（#F9FAFB 底、#D0D5DD 边、#475467 字，12px/600，保留前导圆点）。
- 来源：U1 b 新-10（a 在建议 2 提出）；U2 a 新-4（并注明 U1 胶囊同色，建议同步）。

### UI-110 · [P3] 「1 text pairs below 4.5:1」单复数错误（U1/U5 · 细节 · 可挽回 U1 b 0.1，U5 a 0.05）

- 位置：U1-default-1440/768、U1-field-tabs、U1-keyboard-01 至 03、U1-zoom200；U5-dom-1440-light、U5-inspector*、U5-gpu-compare、U5-preview-390 诊断条标题。
- 修法：按数量选择单复数（`Intl.PluralRules`）。
- 来源：U1 b 新-8；U5 a 新-2。U4 a 在建议中指出同一处。

### UI-111 · [P3] 768 下主题名（18px）比主标题（16px）大；1440 下两个 18px 标题并列（U1 · 细节 · 可挽回 b 0.1）

- 修法：每栏只保留一个 18px 标题（建议主题名为主标题，≤800 时 16px），或两者都遵守 ≤800 的 16px。
- 来源：b 新-9（a 在建议 5 提出两个 18px 标题竞争）。

### UI-112 · [P3] 分隔点「·」悬挂在行尾，Overridden 胶囊后多一个点（U1 · 细节 · 可挽回 b 0.1）

- 位置：U1-zoom200-720-light 预览右上「Rustify Light ·」；U1-configured-1440-light「• Overridden · Light 6 · Dark 6」。
- 来源：b 新-13。

### UI-114 · [P3] 页头「Modified / Matches preset」的比较基准不清，与存储状态读法冲突；吸顶区没有保存状态（U1/U4 · 交互、细节 · 可挽回 U1 b 0.05，U4 a 0.1）

- 位置：U1-top-1440-dark「Graphite Modified」与下方「Saved on this device」；U4-upgrade-pending 页头「Matches preset」对话框「Draft state: Matches saved document」；U4-upgraded「Modified」紧挨「Saved on this device」。
- 修法：页头写明基准（「Modified from preset · N overrides」），吸顶区加按实际写入结果显示的保存状态胶囊；对话框用相同措辞。
- 来源：U1 b 新-12；U4 a 新-12（同一页头标签）。

### UI-116 · [P3] 预览侧诊断条中 Locate 与所属色对的对应关系不清（U2 · 布局 · 可挽回 a 0.1）

- 位置：U2-source-overridden-1440-light 诊断条（色对文字到 Locate 相隔约 220px）；U2-group-reset、U2-controls 三列时 Locate 与下一对只隔 16px。
- 修法：单列 `grid-template-columns: 1fr auto`，每行一对，行间分隔线；超过 4 对时折叠。
- 来源：a 新-2。

### UI-117 · [P3] 同一对比数据两处精度不同（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-low-contrast-1440-light 预览条「1.00:1」「2.62:1」，编辑列「1.000:1」「2.623:1」。
- 来源：a 新-10。

### UI-118 · [P3] 错误文字重复了上方 helper（U2 · 细节 · 可挽回 a 0.1）

- 位置：U2-field-error-1440-light Corner radius 下方。
- 修法：出错时用错误文字替换 helper（同一 `aria-describedby` 节点）。
- 来源：a 新-11。

### UI-145 · [P3] 冲突差异块纵向松散，组头配色与琥珀底混搭（U4 · 布局、视觉 · 可挽回 a 0.1）

- 位置：U4-conflict-1440-light「overrides.primary」y577 至值 y666 各占一行；组头 #F9FAFB 在 #FFFAEB 内。
- 来源：a 新-6；b 新-2 中「差异拆成四行竖排」的部分。

### UI-150 · [P3] DOM + GPU 时 DOM 列仍为两列卡片，标题与标签被挤成两行（U5 · 布局 · 可挽回 a 0.1）

- 位置：U5-gpu-compare-1440-light DOM 列 526px，卡片各约 226px。
- 修法：预览 scope 根 `container-type: inline-size`，用容器查询在窄容器下单列（P5 自动宽度跟随可用空间）。
- 来源：a 新-3（b 在建议 4 提出同一点，未计分）。

### UI-151 · [P3] 未标注态标签仍用选中蓝；面板为空 token 列表留一行空位（U5 · 视觉、细节 · 可挽回 b 0.1）

- 位置：U5-inspector-undeclared-1440-light、U5-inspector-undeclared-panel-1440-light。
- 来源：b 新-3（a 在建议 3 提出标签配色，未计分）。

### UI-154 · [P3] wrap=pretty 与 wrap=wrap 折行完全相同（U7 · 交互 · 可挽回 a 0.1）

- 位置：U7-type-wrap-matrix-1280/390。
- 修法：换一段在当前卡宽下 wrap 会留孤词的文案，或收窄卡片。
- 来源：a 新-5。

### UI-05 · [P3] 200% 下导出对话框动作按钮高度固定 32px，没有随字号放大（U1 · 布局 · 部分解决 · 可挽回 b 0.05）

- 位置：U1-zoom200-export-end-720-light 的 Copy code、Select text、Download（y 726–757）。
- 修法：`min-height: 2.25rem; padding: .375rem .75rem`，单位用 rem。
- 来源：b UI-05（部分解决，基于补采帧）；a 在补采前判已解决。取部分解决。

### UI-25 · [P3] 「Import candidate」节里 Version/Name 行内排、配置字段上下排，两种排法（U3 · 布局 · 部分解决 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-1440-light y 598–625 与 y 656–791。
- 来源：a UI-25 余项；b 新-8（同一处，b 判 UI-25 本身已解决）。取部分解决。

### UI-42 · [P3] CSS 注释中字体路径仍有 `makepad/widgets/…` 与 `makepad_widgets/…` 两种拼法（U3 · 细节 · 部分解决 · 可挽回 b 0.05）

- 位置：U3-export-css-1440-light y=496 与 y=621、663。
- 来源：b UI-42（部分解决）；a 判已解决。取部分解决。

### UI-44 · [P3] 损失清单用 palette 而基础配置用 color，引号用法不统一（U3 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U3-export-v1-1440-light 损失框「Brand palette “Pink”」「Scaling intent 100%」。
- 来源：a UI-44 余项（部分解决）；b 判已解决。取部分解决。

### UI-61 · [P3] 句末标点与行话（U4 · 细节 · 部分解决 · 可挽回 a 0.05 / b 0.05）

- 位置：U4-no-locks-initial 正文缺句号；「Retry explicitly approves retained actions…」。
- 来源：a UI-61；b UI-61。

### UI-76 · [P3] DOM 与 GPU 两框内的首行内缩相差约 8px（U5 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 (370, 293)，GPU 首行 (908, 285)。
- 来源：a UI-76（部分解决）；b 判已解决（两框上沿对齐）。取部分解决。

### U6

### UI-108 · [P3] 粘性头下边框下方 17px 又有一条分隔线，形成双细线（U1/U4 · 细节 · 可挽回 U1 a 0.05，U4 b 0.05）

- 位置：U1-default-1440-light y=314 与 y=331；U1-default-768-light、U1-zh-top-1440-light、U1-zoom200-720-light 同理；U4 各 1440 帧 y=282/299 或 314/331。
- 修法：滚动区第一个分组去掉上边框，或头区只在内容滚动后显示下边框。
- 来源：U1 a 新-7；U4 b 新-5（同一对线）。U4 a 在建议中指出同一现象并归 U1。

### UI-113 · [P3] 壳层焦点环有多种颜色/透明度（U1 · 细节 · 可挽回 b 0.05）

- 位置：U1-keyboard-01 预设搜索 #8AB7F7；U1-keyboard-02 品牌色选择器 #A1C6F9；U1-keyboard-04 Dark #97C2F9/#A1C6F9。
- 修法：搜索框、选择器、分段控件共用一个焦点 token（3px、40%、2px offset）。
- 来源：b 新-11；a 在 UI-02 余项中列出同一现象。

### UI-119 · [P3] 选中的分体控件只高亮左半段（U2 · 视觉 · 可挽回 a 0.05）

- 位置：U2-source-overridden-1440-light primary 格：左段 #84CAFF/#EFF8FF，↺ 段仍 #D0D5DD/白。
- 修法：选中样式加在分体控件外层，外框与中间分隔统一为 #84CAFF（F4 外缘圆角的分体控件）。
- 来源：a 新-5。

### UI-120 · [P3] 句中色值不是等宽字（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-source-overridden-detail-1440-light「Clearing this override reveals Light: #d6409f.」。
- 来源：a 新-12。

### UI-121 · [P3] 同一概念两种措辞：Override / Overridden（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-13。

### UI-122 · [P3] 范围文案「−0.5… 0.5em」像被截断（U2 · 细节 · 可挽回 a 0.05）

- 位置：U2-field-error-1440-light Letter spacing helper。
- 来源：a 新-14。

### UI-123 · [P3] 来源块里 token 名出现两次（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-15。

### UI-124 · [P3] 诊断区「Diagnostics」与「Preview diagnostics」标题重复（U2 · 细节 · 可挽回 a 0.05）

- 来源：a 新-16。

### U3

### UI-129 · [P3] 「Import notes」框内多一条横线（U3 · 视觉 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440-light 框边 y=728、框内 y=741；390 同。
- 来源：a 新-3；b 新-5。

### UI-130 · [P3] 导出对话框按钮行下方留白大于 1.5rem（U3 · 布局 · 可挽回 a 0.05）

- 位置：U3-export-json-390-light 按钮底到下边框 51px；1440 为 32px。
- 来源：a 新-8。

### UI-131 · [P3] 390 下「256 KiB」被拆到两行（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-4。

### UI-132 · [P3] v1 损失清单里的 Auto 没写出实际配对的灰阶（U3 · 细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-export-v1-1440-light 第 2 条（R1 写过「resolved mauve」）。
- 来源：a 新-5；b 新-9。

### UI-133 · [P3] 备注与差异表对 token 的命名和字体不一致，`name` 行的作者文字用了等宽（U3 · 视觉、细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440/390 备注「base.light.font-serif」（比例字）对表格「Light · base」+ 等宽 `base.primary`；差异表 `name` 行「Rustify Light」「Imported legacy 旧主题」为等宽。
- 来源：a 新-6；b 新-6（同指备注 token 路径与表格写法不一致）。

### UI-134 · [P3] 差异表首列表头「Mode / token」已不准确（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-7。

### UI-135 · [P3] 吸顶表头没有底边，滚动时下方行像叠字（U3 · 布局 · 可挽回 b 0.05）

- 位置：U3-import-review-bottom-1440-light y≈340。
- 来源：b 新-4。

### UI-136 · [P3] 改动类型「Changed」与 label 同样式，390 下像无值的 label（U3 · 视觉 · 可挽回 b 0.05）

- 来源：b 新-7。

### UI-146 · [P3] 冲突原文 textarea 无边框、透明底，截断处无滚动提示（U4 · 视觉 · 可挽回 a 0.05 / b 0.05）

- 位置：U4-conflict-raw-1440-light y355–515。
- 来源：a 新-11；b 新-6。

### UI-155 · [P3] 390 下 Kbd size 9 行换行后键帽上下相贴（U7 · 布局 · 可挽回 a 0.05）

- 位置：U7-type-kbd-matrix-390-light 约 y=172–345。
- 来源：a 新-4。

### UI-137 · [P3] 22 字节显示为「0.02 / 256 KiB」（U3 · 细节 · 可挽回 b 0.03）

- 位置：U3-import-error-1440-light y=611。
- 来源：b 新-10。

### UI-138 · [P3] 「Required source revision」行没有对齐表单列（U3 · 布局 · 可挽回 b 0.03）

- 位置：U3-export-rust-1440-light y=215，修订号 x=394 对 Output x=341。
- 来源：b 新-11。

### U4

## 待裁定事项

### UI-88、UI-89 · Heading 与 Strong 不粗（P1，门槛项）

- **原因**：R1 修复方附计算样式（均为 700）判不采纳；R2 评审仍判「仍存在」。按合同，不采纳后仍被判存在的差距转为待裁定。
- **主 agent 实测**：计算字重确实是 700。但页面里 IBM Plex Sans 只有 400 一个字面，同一根因还造成：
  - UI-152：Text 四档字重相同；
  - UI-09、UI-26：label 不是 600。
- **相关差距**：UI-152（P1，可修复，不在待裁定内）、UI-09、UI-26。
- **已有尝试**：R1 修复核实了计算样式，未改实现。
- **可选项**：
  1. 认定为缺陷：修复方在处理 UI-152 时补齐 Plex 300/500/600/700 静态字面，下一轮由评审复核 UI-88、UI-89；
  2. 接受当前渲染：在 U7 比较口径中写明字重以计算值为准，下一轮重建基线；
  3. 提供其他方向。

### UI-97 · 夹具根顶边的外来灰线（P3）

- **原因**：R1 修复方判为亚像素取整噪声，不采纳；R2 评审在 `U7-fixture-1280-light.png` 第 0 行仍看到 x=138–288、295–374 的灰色线段，390 图没有。
- **已有尝试**：修复方核对了根元素与内容层没有边框、坐标一致。
- **可选项**：
  1. 接受为采集噪声：改为留边截取夹具根，只改采集节；
  2. 要求修复：隔离夹具根与上方控件；
  3. 不处理，记为豁免。

### UI-156 · Code 变体的「参考选阶 / 旧 token 对」对比低于 AA（P2，机器检查）

- **原因**：这与 UI-100 是同一类「参考与 AA 冲突」。用户对 UI-100 的接受只写进了 U6 比较口径；本轮在 U5 预览（Code soft 3.89:1）和 U7 夹具（Code solid gray 3.31:1）测到同类组合。
- **已有尝试**：无。
- **可选项**：
  1. 把 UI-100 的口径扩展到 U5、U7 的 Code 及其他 P1 排版组件，写入目标包，下一轮重建基线；
  2. 要求这些组件在默认主题下满足 AA，修改 Code 的文字角色，并同步计划与 DESIGN.md。

## 建议（不扣分）

汇总自各评审，原文见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r2/judge.md) 的「建议」节。下面列出跨单元重复出现、影响较大的几条：

- 存储状态块压成工具栏状态胶囊，让基础配置在首屏出现（U1 a、b；U2 a 的 UI-17 也指出常驻区挤占高度）。
- 导出 JSON 格式化，代码框用 `white-space: pre` 加横向滚动（U3 a、b）。
- U6 夹具补 high-contrast、gray、disabled 列，IconButton 矩阵，Link 字级与 underline 模式。补齐后，「与目标一致」的结论才能覆盖整张目标图。
- 「Caller utilities」示例改为可读配色（U6 a；axe 1.24:1，与 R1 同）。
- Link 的焦点环有 0.15s 过渡，在 `prefers-reduced-motion: reduce` 下仍在播放（主 agent 实测，见机器检查）。目标包没有相关要求，只作建议。

## 机器检查

全部在运行中的 release 页面上执行，结果没有交给评审。

| 检查 | 工具与版本 | 结果 | 对应差距 |
| --- | --- | --- | --- |
| 控制台错误、失败请求、pageerror | Playwright 1.63.0 / Chromium 153.0.8010.12 事件监听；采集 6 次运行与机器检查会话 | 均为 0 | — |
| CSP 违规 | 页内 `securitypolicyviolation` 监听，strict CSP 服务 | 0 | — |
| 横向溢出 | 页内测量 `scrollWidth` 与越界元素 | 0。覆盖：Studio 1440、768、390 编辑与预览、720 + 根字号 200%；catalog 按钮与布局夹具 1280、390 | — |
| 对话框是否在视口内 | 页内 boundingBox | 导出对话框：1440 下 x208、w1024、h542；768 下 x16、w736；390 下 x16、w358、h583。都在视口内 | — |
| 可访问性扫描 | @axe-core/playwright 4.13.0，wcag2a/2aa/21a/21aa/22aa | 见本表下方「axe 结果」。incomplete 项未逐项人工复核 | UI-98 已解决；UI-100 不再适用；新增 UI-156 |
| 语言切换点击区 | 页内 boundingBox + `elementFromPoint` | EN、中文各 50×38，中英两态中心都命中自身 | UI-99 已解决 |
| 键盘走查（真实按键） | Playwright keyboard | 见本表下方「键盘走查」 | — |
| B2 焦点样式 | 键盘聚焦后读计算样式（Shift+Tab、Tab） | 见本表下方「B2 焦点样式」 | 与 U6 评审结论一致，无新增 |
| 字重 | 页内计算样式 + `document.fonts` | 计算字重：Text 四行 300/400/500/700，Heading 700，Strong 700。`document.fonts` 中 IBM Plex Sans 只有 400 一个字面 | 核对说明附在 UI-88、UI-89、UI-152、UI-09、UI-26 |
| 200% 吸顶头区 | 页内测量 + 滚轮 | `div.studio-editor-top` 为 sticky、`max-height: 450px`、`overflow-y: auto`，scrollHeight 964；在其上滚轮 200px 后 scrollTop 0→200 | 核对说明附在 UI-101 |
| 性能 | — | 未检查。本轮 UI 验收不测性能，没有新增毫秒 SLO | — |

**axe 结果**

- Studio 1440、768：1 个节点，是 U5 预览里的 Code soft「ThemeDocument」，3.89:1。390 编辑与预览、导出对话框中没有工具栏按钮违规。
- catalog 按钮夹具：19 个节点。
  - 16 个色族 solid 白字压第 9 阶（2.97–4.12:1），加上 Accent pair 与 Semantic accent（2.62:1），属用户已接受的 UI-100 口径。
  - 1 个是「Caller utilities」示例（1.24:1），按 R1 做法归入建议。
- catalog 布局夹具：1 个节点，Code solid gray 3.31:1。

**键盘走查**

- Studio 从文档起点 Tab 70 次，焦点都在屏内且有可见指示。
- 导出对话框 Tab 25 次、导入对话框 Tab 15 次，焦点都留在对话框内；Esc 关闭后焦点回到触发按钮。
- 品牌色选择器：Enter → ↓↓ → Enter，从 Custom 改为 Gold，保留 3px rgba(21,112,239,.4) 焦点环。
- catalog 按钮夹具 Tab 61 次，焦点都可见。

**B2 焦点样式**

- 浅色 12 个、深色 7 个按钮都是 2px solid，第 8 阶色：
  - classic、solid、IconButton、Unavailable 的 offset 为 2px；
  - soft、surface、outline、ghost-offset、ghost 的 offset 为 −1px；
  - red 用 rgb(235,142,144)，High contrast 用 rgb(94,177,239)。
- Link：0.15s 过渡结束后为 2px solid rgb(141,164,239)、offset 2px、圆角 1.26px（0.07em）、下划线隐藏。
- 机器检查脚本是在 Tab 后立即读取的，读到了过渡中间值（3px、offset 0），已用等待 1s 的复测更正。

原始结果：[machine-check.json](DOM-COMPONENTS-P1.ui-captures/r2/machine-check.json)；脚本：[machine-check.cjs](DOM-COMPONENTS-P1.ui-captures/r2/machine-check.cjs)。

## 采集与覆盖

### 环境与被测版本

- HEAD `f5613b47cebcebd3a0acd47184534297bc39a22b`（main，dirty）。产品路径（crates、examples、xtask、web、makepad、Cargo.toml、Cargo.lock）相对 HEAD 无 diff，也没有未跟踪文件。
- 本轮从 HEAD 重建两个示例的 release（`CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0 mbx xtask build-web --release`）：
  - Studio `f83c6ff130d88c2d`，与修复方最终的 `2be2f32fbf78c5f3` 不同。Studio 构建会注入源码修订，修复提交后 HEAD 已变，这是预期的差异；
  - catalog `07b789e3e401b12a`，与修复方相同。
- 服务：`mbx xtask serve` strict CSP，Studio 端口 4181，catalog 端口 4176。采集前用 `curl …/build-manifest.json` 确认服务发出的正是这两个 build id。

### 采集

- 工具：Playwright 1.63.0 / Chromium 153.0.8010.12 headless。
- 条件：DPR 1；B2 焦点特写为 DPR 2。`reducedMotion: reduce`；等待字体就绪、网络空闲、500ms 无长帧。
- 数量：首采 121 张，首批评审后补采 14 张，共 135 张，0 失败。原图全部保留。
- 补采中有两张没拍到目标，已在 capture.md 注明不作证据：
  - `U4-conflict-detail-1440-light-a2`；
  - `U4-conflict-after-1440-light`。
- 采集做法有三处变化，都只改了目标包「采集」节，并记入变更记录：
  - R1 的补采做法并入首采；
  - 新增 U5 局部相反外观帧、U6 B2 焦点特写、U7 分段元素图；
  - 补采见下。

  目标指纹不变。
- 逐图登记见 [capture.md](DOM-COMPONENTS-P1.ui-captures/r2/capture.md)。

### 目标文件 hash

| 文件 | SHA-256 前缀 |
| --- | --- |
| 目标包（含 R2「采集」节变更） | `b8bf9643d3f7` |
| 13 张目标图 | 与 R1 相同（见 [R1 报告](DOM-COMPONENTS-P1.ui-review.md#目标文件-hash)），本轮逐张复算一致 |

- 目标指纹：`manual-sha256-normative-sections-v1:24f1e44d673e725ea822b8d3e90d2a9924fb1a7250128f4a4016516f5caba967`，算法见 [target-fingerprint.py](DOM-COMPONENTS-P1.ui-captures/r2/target-fingerprint.py)。
- R1 的指纹是 `0370d573f009…`。变化来自用户裁定 UI-100、UI-85 后对「单元」与「规范要点」的修改，所以本轮重建基线。

### 补采与未覆盖范围

首批评审报告了冻结矩阵内看不出的格子，主 agent 补采后各派一位全新评审重评：

- **U1**：200% 导出对话框底部、字段标签页及其焦点、基础配置下半段。
- **U3**：对话框内各滚动区滚到底。
- **U4**：冲突面板下半部及其后的状态区。
- **U5**：深色 Caller class override、相反外观边界内的菜单与对话框、未声明拾取后的面板。

评审报告的下列看不出项不在冻结矩阵内，没有补采：

- 键盘焦点帧（U1 的 4 帧除外）、悬停与按下态；
- 中文态（U1 除外）；
- GPU 失败态；
- 预览宽度档；
- 导入的语法错误、超限、不支持颜色；
- 复制或下载失败、排队态；
- Locate 点击结果；
- U6 的 high-contrast、gray、disabled 列，IconButton 矩阵，Link 下划线模式；
- U7 的其他断点。

把这些加入矩阵要改目标包的「单元」节，需要用户确认，下一轮按重建基线评审。截图来自 headless Chromium，不代表 Firefox、Safari、真机、人工读屏或 OS IME。

### 环境噪声

- `U4-conflict-detail-1440-light` 与 `U4-conflict-1440-light` 逐像素相同；`U4-upgrade-review-end` 与 `U4-upgrade-review` 逐像素相同，因为对话框在 900px 高内不需要滚动。
- `U6-links-hover-1280-light` 与 `U6-links-1280-light` 逐像素相同。
- 同一版本重拍没有出现不一致的结果。

### 评审方式

- 共派 11 位全新上下文的只读评审，模型继承主 agent：
  - 首批 7 位并行，每单元一位；
  - 补采后为 U1、U3、U4、U5 各派 1 位全新评审重评。
- 每个单元取总分较低那位评审的一整组四维分与理由，差距取并集，同一问题按显式映射合并，映射写在 judge.md 各条的「来源」里。
- 例外：首批评审因缺帧判「待核实」、重评用补采帧判定的，以重评结论为准。涉及 UI-11、UI-73，以及 UI-125 的容器类型。
- 评审只拿到目标包、目标图、本单元 r1/r2 截图、两轮 capture.md、actions.log 和 R1 judge.md 的指定行。没有拿到代码、计划、修复记录、门槛、机器检查结果或其他评审的结论。
- 本轮将判「需要修改」，不需要确认评审。

### 主 agent 核对说明

- **事实抽查**：开图核对了以下评审描述，均属实：
  - UI-102：U2-source-overridden-390-light 中漏出「sidebar-border」；
  - UI-101：U1-zoom200-720-light 中「Preview mode」在 y≈449 被切半；
  - UI-152：U7-type-text-matrix-1280-light 中四行字重看不出差别；
  - 评审给出的逐像素相同帧，用 `magick compare` 复核为 0。
- **页面实测**：结论写在 UI-88、UI-89、UI-152、UI-09、UI-26、UI-101 条下。主 agent 没有改分、删差距或改门槛项。
- **UI-156**：来自机器检查。本轮没有「主 agent 核实」来源的差距。
