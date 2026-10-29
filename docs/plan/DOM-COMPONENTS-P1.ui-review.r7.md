# DOM-COMPONENTS-P1 · UI 验收评审 R7

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R7（全量；沿用 R6 基线） | [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) | [R7 目标快照](DOM-COMPONENTS-P1.ui-target.r7.md) · `6c8b16d50cf7`；规范与 R6 相同 | HEAD `b218075a`；产品路径无 diff；Studio `6d1605ccec8b9f78` http://127.0.0.1:4181/；catalog `18ca82701d674efd` http://127.0.0.1:4176/ | [r7](DOM-COMPONENTS-P1.ui-captures/r7/) · [逐图清单](DOM-COMPONENTS-P1.ui-captures/r7/capture.md) | [R6](DOM-COMPONENTS-P1.ui-review.r6.md) |

## 验收判定

**通过验收**

本轮分数 **9.8/10（U5）**，沿用 R6 门槛 **8/10**；本基线历史最佳 **9.8**。七个单元均有有效独立结论，固定的单元、主状态、视口与主题矩阵均有可用实拍；**P0/P1 为 0，剩余 P2 1 条（UI-218）**。UI-216、UI-217 均由独立评审确认已解决；机器检查支持活动项滚入列表可视区。没有停滞预警或待裁定事项。

最低分高于门槛 + 0.5，不触发临界确认评审。U3、U4、U7 的第二位评审用于补采复评，按较低整组分合并；本轮不是用户豁免通过。

本判定限定于冻结 UI 矩阵和所测版本。完整 IconButton/Link 参数组合、14 个共享字段的全部编辑结果、所有断点和未操作行为没有外推为通过；不替代完整 UAT、代码复审、WCAG 认证或发布验收。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 | R6 | 采用原文 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U1 工作台框架与基础配置 | 3 | 3 | 3 | 1 | **10** | 9.5 | [judge-U1-a.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U1-a.md) |
| U2 精调与来源解释 | 3 | 3 | 3 | 1 | **10** | 10 | [judge-U2-a.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U2-a.md) |
| U3 导入与导出 | 3 | 3 | 3 | 1 | **10** | 10 | [judge-U3-b.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U3-b.md) |
| U4 迁移与存储反馈 | 3 | 3 | 3 | 1 | **10** | 10 | [judge-U4-b.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U4-b.md) |
| U5 预览 | 3 | 2.8 | 3 | 1 | **9.8** | 10 | [judge-U5-a.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U5-a.md) |
| U6 Button / IconButton / Link / ThemePanel | 3 | 3 | 3 | 1 | **10** | 10 | [judge-U6-a.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U6-a.md) |
| U7 布局与排版 | 3 | 3 | 3 | 1 | **10** | 9.7 | [judge-U7-b.md](DOM-COMPONENTS-P1.ui-captures/r7/judge-U7-b.md) |

U3、U4、U7 首评/补评均为 10；同分采用覆盖补采的后评审整组理由。其余各一位评审。完整四维理由、未关闭差距及建议见 [judge.md](DOM-COMPONENTS-P1.ui-captures/r7/judge.md)，10 份原文分别保存，未改分或逐维拼分。

U5 从 R6 的 10 降至 9.8，原因是本轮新识别了已存在的未标注拾取框偏差；评审对照 R6 原图后未将其记为代码回归。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | --- | --- | --- |
| R1 | 5.2（U7） | 5.2 | 0 / 18 | — | 首轮、首次规范基线；补采后重评 U3–U7 |
| R2 | 7.1（U7） | 7.1 | 0 / 4 | — | 重建基线（用户裁定 UI-85、UI-100 改了目标）；补采后重评 U1、U3、U4、U5 |
| R3 | 8.6（U4） | 8.6 | 0 / 1 | — | 重建基线（用户裁定 UI-156 改了 U5、U7 口径）；补采后重评 U2、U3、U4；唯一的 P1 来自机器检查 |
| R4 | 9.35（U3） | 9.35 | 0 / 1 | — | 同一基线；唯一的 P1 是 U5 回归 UI-207；停滞预警 UI-128（U3） |
| R5 | **9.6（U7）** | **9.6** | **0 / 1** | **UI-128 所在 U3** | 同一基线；UI-128、UI-207 已解决；新增 P1 UI-215 来自纯键盘实测 |
| R6 | **9.5（U1）** | **9.5** | **0 / 1** | — | 重建基线（用户接受 UI-115 的现状）；六条修复关闭；补采发现 P1 UI-217，另有 P2 UI-216；需要修改 |
| R7 | **9.8（U5）** | **9.8** | **0 / 0** | — | 同一基线；UI-216/UI-217 关闭；新增识别 P2 UI-218；通过验收 |

R1–R6 行保留 R6 报告原表。R6 因用户接受 UI-115 重建基线；本轮仅补充采集做法，规范与附件指纹未变，因此历史最佳从 R6 的 9.5 提高为 9.8。没有重构轮或停滞预警。

## 上轮差距复核

[M7 的 R6 修复记录](DOM-COMPONENTS-P1.records/M7.md)在“R6 → 修复 · 2026-10-29T09:49:46+08:00”将两条登记为“已修改待评审”。本轮独立复核结果如下；原计划和记录保持只读。

| 编号 | 上轮严重度与摘要 | 上轮处置 | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-217 | P1 · 已选 Pink 后菜单活动项不可见 | 已修改待评审 | **已解决** | [U1 原文](DOM-COMPONENTS-P1.ui-captures/r7/judge-U1-a.md)；[390](DOM-COMPONENTS-P1.ui-captures/r7/U1-matrix-keyboard-03-390-light.png)、[768](DOM-COMPONENTS-P1.ui-captures/r7/U1-matrix-keyboard-03-768-light.png)、[1440 深色](DOM-COMPONENTS-P1.ui-captures/r7/U1-matrix2-keyboard-03-1440-dark.png)均可见 Pink 勾选及 Plum 蓝色活动行；[实测](DOM-COMPONENTS-P1.ui-captures/r7/machine-interactions.json)的上下移、Home/End 活动行均在列表内部，Esc 返回触发器焦点 |
| UI-216 | P2 · Code ghost color/high-contrast 带强调色 | 已修改待评审 | **已解决** | [U7 首评](DOM-COMPONENTS-P1.ui-captures/r7/judge-U7-a.md)与[补评](DOM-COMPONENTS-P1.ui-captures/r7/judge-U7-b.md)对照 R6、目标及本轮[1280](DOM-COMPONENTS-P1.ui-captures/r7/U7-type-code-matrix-1280-light.png)/[390](DOM-COMPONENTS-P1.ui-captures/r7/U7-type-code-matrix-390-light.png)：前两列均为中性前景，无填充/描边，其他变体与 gray 分支保留差别 |

旧机器检查项和主 agent 核实项中，R6 没有独立于上述两项之外的未关闭差距。四个导入/导出选择器的焦点在本轮原图和主路径键盘检查中未见回归。

## 差距清单

当前 **1 条 P2**；没有 P0/P1、待核实门槛项或待裁定事项。

### UI-218 · [P2] 未标注部件的拾取框没有使用固定蓝色（U5 · 视觉 · 可挽回 0.20）

- 位置：[点击后原图](DOM-COMPONENTS-P1.ui-captures/r7/U5-inspector-undeclared-1440-light.png)、[面板入画](DOM-COMPONENTS-P1.ui-captures/r7/U5-inspector-undeclared-panel-1440-light.png)的 Provider & overlay checks 区右上角 **Create · 0** 按钮外缘；[悬停帧](DOM-COMPONENTS-P1.ui-captures/r7/U5-inspector-undeclared-hover-1440-light.png)可见同样处理。
- 现状 → 期望：点击未标注部件后，外缘为灰色虚线，面板正确显示“Not annotated · this part has no declared token mapping.”。按[目标 P2](DOM-COMPONENTS-P1.ui-target.r7.md)，保留真实的未标注说明，同时让被拾取部件的选中矩形使用固定蓝色 outline；目标没有对未标注部件豁免选中框颜色。
- 修法：Inspector 的未映射部件选中标识复用已映射部件的固定蓝色 outline，不继承作者 border/foreground。未标注状态继续由面板文字表达，不编造 token，不扩展完整 Inspector 功能。
- 验证：开启 Inspect，点击 Create · 0；未标注说明仍在，按钮外缘改为与 Create profile 拾取态一致的固定蓝色。
- 来源：[U5 独立原文](DOM-COMPONENTS-P1.ui-captures/r7/judge-U5-a.md)“新-1”→ UI-218；[编号映射](DOM-COMPONENTS-P1.ui-captures/r7/judge-results.json)。评审已对照 R6 对应原图，记为新识别的既有偏差，不记回归。
- 主 agent 核对：实际打开面板入画原图，确认按钮外缘为灰色虚线、未标注说明可见。此说明不改分、不删差距或改变严重度。

## 待裁定事项

无。UI-218 本轮首次作为差距登记，没有修复尝试或不采纳处置，不触发顽固差距。其 P2 严重度不阻止本轮通过，是否继续打磨由用户选择。

## 建议（不扣分）

- U1：额外的 Enter 即时帧应等待列表真正可见；已有方向键帧和稳定后专项实测提供有效展开证据。
- U5：如需更广行为覆盖，可补固定内容宽度选择、浮层内拾取和真实键盘操作；本轮没有将其外推为通过。
- U6/U7：完整组件参数组合、更多缩放分段、Reset 点击和其余断点边界可另取证；现有分数只覆盖已展示样本。
- 沿用 R6 的 Caller utilities 建议：人为覆盖 class 的演示样例仍有低对比，不作为默认组件 AA 通过证据；可选择更易读的演示配色。

## 机器检查

所有检查都在上述真实 release 页面执行，位于独立原文及合并 judge 落盘之后。机器结果未交给评审调分。

| 检查 | 实际结果 | 证据 |
| --- | --- | --- |
| 采集健康日志 | 74 页：pageerror、console error、失败请求/HTTP ≥400、记录的 CSP 事件均为 0；74 页回收 CSP 日志；10 个采集 run 无失败 | [健康汇总](DOM-COMPONENTS-P1.ui-captures/r7/capture-health-summary.json)、[原始轨迹](DOM-COMPONENTS-P1.ui-captures/r7/capture-log.json) |
| 后置日志 | 基础检查 7 个 session 的 console/CSP 均为 0；补充确认 4 页的 pageerror、console、失败请求/HTTP ≥400、CSP 均为 0；菜单/复制检查另 4 页的 console/pageerror 和失败请求为 0，该脚本未监视 CSP，不外推 | [基础](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json)、[确认](DOM-COMPONENTS-P1.ui-captures/r7/machine-confirmation.json)、[交互](DOM-COMPONENTS-P1.ui-captures/r7/machine-interactions.json) |
| 横向溢出与对话框 | 9 个页面测量的 pageOverflow 均 0（Studio 1440/768/390 编辑和预览/720-root200，catalog 两档按钮与布局）；3 档导出对话框边界均在视口内 | [基础](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json) |
| 键盘主路径 | 连续 Tab 70 步均在屏内；导出 25、导入 15 次 Tab 均留在对话框；Esc 关闭并回到对应触发器；品牌键盘确认 Custom→Gold | [基础键盘记录](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json) |
| UI-217 专项 | 三组合上下移、Home/End 后活动项均在列表边界内；Plum 时列表 scrollTop=198，Home 时=4，End 时=614；Esc 后触发器 focus-visible=true | [逐步位置](DOM-COMPONENTS-P1.ui-captures/r7/machine-interactions.json) |
| Enter/Space 补充确认 | 快速串行首测的 1440 深色、390 浅色 Enter 后仍是关闭触发器；这些帧不作展开证据。等待页面静止后，三组合的 Enter、Esc→Space、Esc→Enter 均实际展开且列表可见，不推断内部原因 | [保留首测](DOM-COMPONENTS-P1.ui-captures/r7/machine-interactions.json)、[稳定后确认](DOM-COMPONENTS-P1.ui-captures/r7/machine-confirmation-summary.json)、[390 原图](DOM-COMPONENTS-P1.ui-captures/r7/MC-settled-enter-390-light.png) |
| Button 焦点 | 七变体 size2 均 :focus-visible、2px 实线；classic/solid offset 2px，其余 −1px；未外推为所有尺寸/颜色组合 | [基础焦点实测](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json) |
| 200% | 720×900、根字号 200% 时，未隐藏 DOM surface 的 scrollWidth/clientWidth 均 300，scrollLeft=0；页面级横向溢出为 0。另一个隐藏后代的 243/236 预布局尺寸不计可见溢出；没有把离屏按钮几何写成屏内通过 | [基础](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json)、[实拍](DOM-COMPONENTS-P1.ui-captures/r7/MC-root200-preview-720-light.png) |
| 复制与下载 | 剪贴板实际读回与可见输出一致；下载 theme.json，2494 B、schema_version=2、failure=null，内容与可见 JSON 一致；注入 clipboard 拒绝后，失败说明、可选择文本和 Download 仍可用 | [交互](DOM-COMPONENTS-P1.ui-captures/r7/machine-interactions.json)、[下载](DOM-COMPONENTS-P1.ui-captures/r7/MC-export-theme.json)、[拒绝实拍](DOM-COMPONENTS-P1.ui-captures/r7/MC-copy-denied-1440-light.png) |
| 可访问性扫描 | axe-core 4.13.0，wcag2a/2aa/21a/21aa/22aa：7 个 Studio 框架/导出状态违规 0；两档按钮夹具各19个对比度节点，16个色族 solid 的 a9 合成背景及 contrast 前景逐项相符，2个为用户接受的旧 accent 配对，另1个为 Caller utilities 建议；两档布局夹具各1个 Code solid gray 3.31:1，按 U7 已确认口径接受 | [扫描](DOM-COMPONENTS-P1.ui-captures/r7/machine-check.json)、[19 节点明细](DOM-COMPONENTS-P1.ui-captures/r7/machine-confirmation.json)、[选阶核对](DOM-COMPONENTS-P1.ui-captures/r7/machine-confirmation-summary.json) |
| 未检查部分 | axe incomplete 未全量人工复核；未重跑整期 V1–V10、真实性能、VoiceOver、OS IME、Safari/Firefox、真机或真人研究。未记通过 | 本轮为冻结 UI 矩阵验收 |

这些对比度处理沿用目标包的 UI-100/UI-156 接受范围，不等于 WCAG 合规认证。没有新增机器检查差距，机器结果不改变独立分数。

## 采集与覆盖

- **路径与版本**：以 DOM 为主；U5 只验收既有 DOM/GPU 预览协作，不新增 GPU。HEAD `b218075a3a2107ab511da932f831f42adfa9390c`，产品源码指纹 `1fdbbbc39758b56531e996384ad7397645d59a7115ea3fcb6c1b39b326a98600`；[初始](DOM-COMPONENTS-P1.ui-captures/r7/tested-version.json)和[结束核对](DOM-COMPONENTS-P1.ui-captures/r7/version-final-check.json)的 HEAD、产品 diff/untracked 与指纹相同。
- **期望与来源**：沿用既有目标包；来源为 PRD §2.5、计划 §5–§6/§9.3、DESIGN.md 与本地 Radix 目标 PNG。`manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4` 与 R6 相同；当前目标只更新采集节及变更记录，评审使用的 R6/R7 快照未覆盖。没有 managed 索引，未 import 或假称接管。
- **构建身份**：两项 `mbx xtask build-web --example … --release` 均退出0，HTTP manifest 与本地逐字节一致；[构建记录](DOM-COMPONENTS-P1.ui-captures/r7/builds.json)、[Studio manifest](DOM-COMPONENTS-P1.ui-captures/r7/build-studio-manifest.json)、[catalog manifest](DOM-COMPONENTS-P1.ui-captures/r7/build-catalog-manifest.json)。Studio 完整 stdout 未另存文件，原回显保留在本轮工具记录；catalog stdout 已保存。
- **工具与条件**：Rust manifest 为 1.99.0；mbx 1.15.0，Node 26.12.1、Playwright 1.63.0、axe-core 4.13.0、Chromium 153.0.8010.12；headless、en-US、reducedMotion reduce。DPR1，焦点特写DPR2；逐帧等待字体/networkidle/500ms静止，200% 按目标规定的 720×900 + 根字号方式。[浏览器包版本](DOM-COMPONENTS-P1.ui-captures/r7/browser-toolchain.json)。
- **原图**：278 张独立评审原图：U1 62、U2 24、U3 38、U4 23、U5 39、U6 50、U7 42；另有14张后置机器原图。[逐图出处](DOM-COMPONENTS-P1.ui-captures/r7/capture.md)和[图像指纹](DOM-COMPONENTS-P1.ui-captures/r7/image-integrity.json)逐项登记；机器图见[单独清单](DOM-COMPONENTS-P1.ui-captures/r7/machine-captures.json)，基础检查的一张图仅有文件mtime时间，未冒充脚本逐帧UTC。
- **真实性**：均为真实入口和当前 release 的原生截图，无模拟页面、合成、修饰或旧图冒充。U4 仅在独立 context 注入种子/存储环境，冲突使用真实同源第二页；没有网络拦截。主 agent 实际打开24张联系图及关键原图核对；联系图仅验真实性，没有提供给独立评审评分，见[QA记录](DOM-COMPONENTS-P1.ui-captures/r7/capture-qa.json)。
- **补采**：保留首采和互补帧。U3 补窄屏成功反馈、v1动作区及候选配置中段；U4补键盘焦点/Escape返回与JSON中末段；U7补两视口100/110%的布局及混排分段。U1两张名含open的额外帧实际未展开，已明确排除为展开证据；所需主状态有其他有效帧与后置确认。[补采/替代对应](DOM-COMPONENTS-P1.ui-captures/r7/capture-resolutions.json)。
- **独立性**：10个全新 `fork_turns=none` 只读评审（首评7位、补评3位），模型/effort继承宿主、无覆盖，工具未回显具体型号。每位只获目标、该单元本轮原图/操作和R6独立材料；未给代码、diff、修复记录、完整报告、门槛或机器结果。U3-a因容量失败续接原任务一次，未换模型，之后返回有效完整结论。[派发与packet](DOM-COMPONENTS-P1.ui-captures/r7/judge-dispatch.json)；首批清单的原始hash及归档副本保留，不让补采覆盖历史输入。白名单是信息边界，不冒充文件权限沙箱。
- **覆盖界限**：所有原图均有独立实际开图记录；完整参数组合、14字段全部编辑、95%所有细微轮廓、其余断点、固定宽度、浮层动态主题、冲突决策执行结果等未外推。各原文保留具体未知内容。本轮通过表示当前固定主矩阵达标，不能写为 SDK 每个参数或整期所有需求均已验收。
- **核对与收尾**：主 agent 已开图确认 UI-218 灰色虚线、UI-217 蓝色活动行及 Code ghost 中性前景，未改独立结论。两项本轮服务已停止，4181/4176 无监听，见[服务记录](DOM-COMPONENTS-P1.ui-captures/r7/services.json)。没有修改产品代码、计划或历史报告，无提交、推送或发布。

## 下一步

是否继续打磨 UI-218：

```text
$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md
```

如果不继续打磨，本循环已有登记为“UI修复”的代码变更，下一步先复审这些改动：

```text
$review-code docs/plan/DOM-COMPONENTS-P1.md
```

本轮不自行启动修复、代码复审或 PRD 差异核查。

## 状态登记

本轮独立评审 ID：`8abc8d80-fa38-4e09-b066-929980601837`；[状态页](../../.xgent-ai/sdlc/status/review/8abc8d80-fa38-4e09-b066-929980601837.md)。复审关联 R6 `4096c476-5ee3-44a5-a195-09a5183542ca`。UI通过不表示已合并、已发布或产品终验通过；计划原文保持只读。

按仓内 v3 / state-model v2 的通过优先规则登记为 `review-recorded` / `review-not-required`，含义为没有强制 apply；UI-218 仍为未关闭的可选 P2，未记修复或豁免。R6 两条由本轮独立复核关闭，接续原 `UI-R6` 处置；Ready、7/7 与历史报告不变。
