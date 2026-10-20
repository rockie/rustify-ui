# DOCS-SSG-P4 · UI 验收复审 R3

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R3 · 全量复审，沿用规范基线 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | [目标包](DOCS-SSG-P4.ui-target.md) · `604274d613d8` | HEAD `93c4d101`，dirty `5b56a9c61406`；实装C `1bbc54b0` / R `740610db`；官方 `http://127.0.0.1:4199/rustify-ui/`、搜索 `http://127.0.0.1:4205/rustify-ui/` | [R3采集清单](DOCS-SSG-P4.ui-captures/r3/capture.md) | [R2](DOCS-SSG-P4.ui-review.r2.md) |

## 验收判定

**需补证**。

U1、U2、U3在本轮已拍、已操作范围均为 **10.0/10**；UI-01–UI-04维持已解决，未见新增可扣分差距，未关闭P0/P1为0。沿用门槛 **8/10**。

U4仍缺当前稳定桌面浏览器的完整任务矩阵、两实体手机和VoiceOver实际朗读。本轮补了原生Safari阅读/搜索/缺译锚点/主题/菜单，以及官方中文quickstart的原生Chrome200%阅读与搜索；局部证据不能补满U4。因此**没有全量轮分数或历史最佳**，U4不补零分，不判全量通过。没有临界通过确认条件、停滞预警或需用户裁定事项。

本轮采用DOM路径：验收静态阅读站、搜索与语言界面，计划未要求GPU呈现。产品代码、计划、目标包、修复记录及历史报告未修改；没有提交、推送、发布或启动修复。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | R1 → R2 → R3 | 原文 |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| U1 搜索主路径 | 3.0 | 3.0 | 3.0 | 1.0 | 9.0 → 10.0 → 10.0 | [U1](DOCS-SSG-P4.ui-captures/r3/judge-U1-a.md) |
| U2 搜索加载与恢复 | 3.0 | 3.0 | 3.0 | 1.0 | 9.4 → 10.0 → 10.0 | [U2](DOCS-SSG-P4.ui-captures/r3/judge-U2-a.md) |
| U3 官方双语阅读与入口 | 3.0 | 3.0 | 3.0 | 1.0 | 9.7 → 10.0 → 10.0 | [U3](DOCS-SSG-P4.ui-captures/r3/judge-U3-a.md) |
| U4 原生设备与辅助技术 | — | — | — | — | 未评分 → 未评分 → 未评分 | [缺证清单](DOCS-SSG-P4.ui-captures/r3/capture-notes.md) |

三位全新上下文评审各负责一个单元，逐图查看其全部本轮材料，完整采用各自四维组。逐维理由见[合并结论](DOCS-SSG-P4.ui-captures/r3/judge.md)及原文。已评最低10.0，三单元并列；分数不代表未拍过程、精确CSS数值或辅助技术已验收。主agent没有改分或删除差距。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | ---: | --- | --- |
| R1 | 无全量分；已评最低9.0（U1） | 未建立 | 1 | 无 | U4缺证；U2补采后9.8→9.4取较低组 |
| R2 | 无全量分；已评最低10.0（U1/U2/U3） | 未建立 | 0 | 无 | UI-01–04已解决；U4仍缺证，新增原生Chrome200%部分冒烟 |
| R3 | 无全量分；已评最低10.0（U1/U2/U3） | 未建立 | 0 | 无 | 未见回归；新增Safari部分原生操作、官方页Chrome200%及键盘/链接/内部滚动实测；U4仍不全 |

目标全文hash及规范指纹与R2一致，未重建基线。[M6修复记录](DOCS-SSG-P4.records/M6.md#ui-验收修复)登记一次普通修复、无重构轮；没有差距重新成立。三个轮次均无全量分，不触发全局平台期。

## 上轮差距复核

R2没有未关闭产品差距。以下对历史关闭项检查可见回归；原处置继续取M6记录“已修改待评审”，不回改该记录。

| 编号 | 上轮严重度与摘要 | 上轮处置 | 本轮结果 | 本轮依据 |
| --- | --- | --- | --- | --- |
| UI-01 | P1 · Show more后键盘焦点丢失；R2已解决 | 保留可聚焦节点、末批归焦输入 | **已解决，未见回归** | U1独立复核连续20→40→60→80→98；[40组](DOCS-SSG-P4.ui-captures/r3/U1-more-40-focus-1280-light.png)、[末批](DOCS-SSG-P4.ui-captures/r3/U1-more-98-focus-1280-light.png)。事后运行期等待60组后再测，Show more仍有3px可见焦点，选择保留 |
| UI-02 | P3 · 窄屏dialog少6px（U1/U2）；R2已解决 | 显式max-width | **已解决，未见回归** | U1/U2各自独立核对；[320深色](DOCS-SSG-P4.ui-captures/r3/U1-results-320-dark.png)、[375长列表](DOCS-SSG-P4.ui-captures/r3/U2-supp-longlist-04-375-light.png)；运行期375/320双主题为343/288px、x=16，桌面576px |
| UI-03 | P2 · 缺译note缺code-bg；R2已解决 | 引用既有code-bg | **已解决，未见回归** | U3独立比对R2同名图；[浅色](DOCS-SSG-P4.ui-captures/r3/U3-fallback-1280-light.png)、[深色](DOCS-SSG-P4.ui-captures/r3/U3-fallback-320-dark.png)。运行期背景为rgb(242,244,247)/rgb(22,27,38)，14px、12px 16px padding |
| UI-04 | P3 · “1 results”；R2已解决 | 增加单数模板 | **已解决，未见回归** | U2独立核对[模块恢复](DOCS-SSG-P4.ui-captures/r3/U2-module-retry-375-light.png)，另实测错误期间改词后Retry显示“1 result”；长列表保持“98 results” |

评审只收到R2关闭摘要，未补造原问题定义；主agent在独立评分后读取R2完整报告及M6原记录，并复测原机器差距。没有只凭代码或修复者自述关闭问题。

## 差距清单

本轮没有已证实的未关闭产品差距；没有新增编号。UI-01–04保留原编号及历史关闭证据。缺少操作与环境的证据不登记成假想产品缺陷。

## 待裁定事项

无。目标沿用既有用户确认，未修改期望或发现冲突；无不采纳、停滞或豁免记录。U4环境不足保持需补证。

## 建议（不扣分）

- 可选高亮同一摘要的全部可见命中，并为截取摘要增加省略号。
- 可选压缩窄屏次要入口、为长导航提供内部滚动、将禁脚本说明放近搜索入口。

均无强制规范依据，不要求为这些建议重新设计。

## 机器检查

各独立原文及合并judge保存后执行。工具为Playwright1.63.0、Chromium153.0.8010.12、axe4.13.0；全部测量来自真实运行页面。

| 检查 | 实际结果 | 限制/证据 |
| --- | --- | --- |
| 页面横向溢出 | 57个页面/状态样本均0px；补充操作样本也为0px | [runtime-checks.json](DOCS-SSG-P4.ui-captures/r3/runtime-checks.json)、[keyboard-checks.json](DOCS-SSG-P4.ui-captures/r3/keyboard-checks.json)；未代替原生完整矩阵 |
| axe | 30次扫描，violations 0 | 仍有37个incomplete规则实例：color-contrast13、aria-required-children2、aria-valid-attr-value22；[原始节点/原因](DOCS-SSG-P4.ui-captures/r3/axe-incomplete.json)。未逐节点人工确认，不称WCAG全量通过 |
| 键盘与恢复 | 4组两类故障的Tab→Retry/Enter恢复；追加错误中改词再Retry，保留新查询并输入获焦；搜索Close/Esc归焦、重开保留查询、语言菜单Esc即时归焦有实测和关键帧 | [菜单归焦](DOCS-SSG-P4.ui-captures/r3/M2-language-escape.png)、[关闭归焦](DOCS-SSG-P4.ui-captures/r3/M2-close-return.png)、[改词恢复](DOCS-SSG-P4.ui-captures/r3/M2-edited-error-retry-result.png)。完整IME/全部控件路径仍未验 |
| Tab边界核实 | 页面activeElement为BODY时，原生Chrome实际焦点位于有边界的地址栏；Shift+Tab可返回输入 | [原生图](DOCS-SSG-P4.ui-captures/r3/U4-native-chrome-tab-boundary.jpg)及AX记录；不将浏览器工具栏焦点误记为产品焦点丢失 |
| 导航与内部滚动 | 首页两个操作实际到quickstart/components/button；中文quickstart切English；首目录锚点到#prerequisites；真实横向wheel滚动代码；禁脚本导航到中文Architecture | [代码内部滚动](DOCS-SSG-P4.ui-captures/r3/M2-code-horizontal.png)及keyboard-checks；未逐章/逐表走查 |
| 中文、404与base | 中文缺译搜索保持zh/page-0051；两视口中文深404搜索到中文quickstart；root/subpath/meta激活保持base | 各单元操作记录与本轮原图 |
| 主题/尺寸/减少动态 | 窄屏dialog尺寸与note色值复测符合已列要求；57个样本reduce=true、scrollBehavior=auto | 未操作真实系统偏好或全部正常动画过程 |
| 正式采集日志 | 323个请求；无pageerror或CSP错误；8条console error均为3次注入module503及5次深404；3条requestfailed均对应注入module503 | [runtime-summary.json](DOCS-SSG-P4.ui-captures/r3/runtime-summary.json)及browser-raw.jsonl；原生部分没有完整网络日志 |
| 运行产物身份 | 四入口HTTP manifest与本轮本地构建逐字节相同 | [build-manifests.json](DOCS-SSG-P4.ui-captures/r3/build-manifests.json)，不是只看首页200 |
| 原生/真机/朗读 | Chrome200%及Safari部分原生任务已补；U4未完成 | [逐项缺证与获取方式](DOCS-SSG-P4.ui-captures/r3/capture-notes.md) |
| Rust/lint/全套E2E/1000页性能 | 本轮未重跑 | 没有产品修复；本轮100页语料只用于界面与交互，不代替M6正式性能门 |

## 采集与覆盖

- 正式矩阵91张本轮原图（U1=38、U2=23、U3=30），另24张原生/事后检查补充。逐图URL、时间、视口、主题、干预、动作和hash见[清单](DOCS-SSG-P4.ui-captures/r3/capture.md)及各单元JSON，补充见supplement-captures.json。没有旧图、模拟页或合成图补格。主agent抽查关键窄屏、错误、缺译、焦点与内部滚动图；原生图在工具中实际查看。
- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`，初始tracked diff SHA256 `5b56a9c61406a18ce50c9bac1a2cbdb3913bd31f3604907cc0cf4ce9e3b43042`，不含未跟踪文件；固定C/R与当前相关产品路径无diff。[baseline](DOCS-SSG-P4.ui-captures/r3/baseline.json)保留原工作树。计划hash `4ec47edde1ede1fbcd5527a4df0db7d61261cc8d5422206ca94afa5d17b87871`，本轮未改。
- 官方build_id `dd5022c567200779`、搜索root `80da98ab7be5b50d`、subpath/meta `176bea4ed7e4d96f`。按既有serve.sh及固定实装CLI重建，无新容器/基础设施。来源文件hash见[source-hashes.json](DOCS-SSG-P4.ui-captures/r3/source-hashes.json)。
- 目标全文及规范指纹与R2一致。既有人工算法`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`保持原样；不声称Phoenix兼容。已有人工历史且无managed索引，本轮未接管、重置或重新探测engine。
- 3位全新上下文subagent，模型/effort继承，未覆盖或派生；每位只读目标、其单元的本轮采集/操作/截图及R2同单元结论/图片。本轮逐图覆盖38/23/30，另看R2 7/5/2图。不是单上下文评分；无确认或因争议换评审。原文中的未证行为保留，不把机器补充回写为评分。
- 本轮机器补充覆盖了部分原先未证过程；仍未覆盖实际IME组合输入、完整hover/pressed/动画、全部章节/代码/宽表及复制/演示全键盘路径。U4原生部分不扩成完整稳定矩阵，实体手机/软键盘/VoiceOver仍缺。本轮缺证不涉及待核实P0/P1产品差距。
- 环境噪声：故障注入503和深404为预定采集；Kimi Browser.getVersion返回不支持，未据此伪造版本；一次长stdin清单生成器遇到编码错误，改用落盘Node脚本后完成，未改变产品或任何截图。原生Chrome仍提示重启可更新，未重启用户浏览器。
- 四个临时服务按实际PID与命令核对后SIGTERM停止；4199/4204/4205/4206已无监听，[生命周期](DOCS-SSG-P4.ui-captures/r3/server-lifecycle.json)。Chrome缩放恢复100%，Safari恢复跟随系统；任务页签保留。既有未提交工作树保留，本轮产物未提交到远端。
- 计划仍为6/7，M7和PRD终验不因本轮分数关闭。后续状态登记只索引本次独立评审，不修改计划实施进度。

## 下一步

本轮没有产品差距需要apply。需接入可操作的当前稳定桌面浏览器、iOS Safari/Android Chrome实体设备和可记录朗读的VoiceOver+Safari会话，完成capture-notes列出的任务，再做全量复审：

```text
$review-ui docs/plan/DOCS-SSG-P4.md
```

全量UI通过后，已有UI修复代码仍须由用户发起`$review-code docs/plan/DOCS-SSG-P4.md`复审；本轮不自行启动修复、代码复审、PRD差异核查或发布。
