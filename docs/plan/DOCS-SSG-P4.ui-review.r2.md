# DOCS-SSG-P4 · UI 验收复审 R2

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R2 · 全量复审，沿用规范基线 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | [目标包](DOCS-SSG-P4.ui-target.md) · `604274d613d8` | HEAD `93c4d101`，dirty `4a112900b88a`；实装 C `1bbc54b0` / R `740610db`；官方 `http://127.0.0.1:4199/rustify-ui/`、搜索 `http://127.0.0.1:4205/rustify-ui/` | [R2采集清单](DOCS-SSG-P4.ui-captures/r2/capture.md) | [R1](DOCS-SSG-P4.ui-review.md) |

## 验收判定

**需补证**。

U1、U2、U3在本轮已观察范围均为 **10.0/10**，首轮 **UI-01–UI-04全部已解决**，未发现新增可扣分差距，未关闭P0/P1为0。沿用R1门槛 **8/10**。

U4仍缺完整原生稳定浏览器任务矩阵、两实体手机和VoiceOver实际朗读；本轮新增的原生Chrome 200%阅读与搜索冒烟只覆盖部分任务。因此没有全量轮分数或历史最佳，不将U4补零，也不将三个单元满分解释为全量通过。无需临界通过确认评审；没有停滞预警或待用户裁定事项。

本轮只评审DOM静态阅读站与搜索对话框，不涉及GPU。未修改产品代码、计划、修复记录或历史报告；未提交、推送或发布。目标仅更新采集节的新CLI身份，期望、单元和比较口径未变。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | R1 → R2 | 原始结论 |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| U1 搜索主路径 | 3.0 | 3.0 | 3.0 | 1.0 | 9.0 → 10.0 | [U1评审](DOCS-SSG-P4.ui-captures/r2/judge-U1-a.md) |
| U2 搜索加载与恢复 | 3.0 | 3.0 | 3.0 | 1.0 | 9.4 → 10.0 | [U2评审](DOCS-SSG-P4.ui-captures/r2/judge-U2-a.md)、[比较图补充](DOCS-SSG-P4.ui-captures/r2/judge-U2-a-supplement.md) |
| U3 官方双语阅读与入口 | 3.0 | 3.0 | 3.0 | 1.0 | 9.7 → 10.0 | [U3评审](DOCS-SSG-P4.ui-captures/r2/judge-U3-a.md) |
| U4 原生设备与辅助技术 | — | — | — | — | 未评分 → 未评分 | 原生部分冒烟不能代替完整矩阵 |

三位全新上下文评审各负责一个单元，四维组完整采用，主agent未改分。布局、视觉、交互与细节的逐维理由及建议见[合并judge.md](DOCS-SSG-P4.ui-captures/r2/judge.md)与各原文。已评最低分10.0，三个单元并列；这不是全量轮分数。评审明确保留未拍区域与操作的未知状态。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | ---: | --- | --- |
| R1 | 无全量分；已评最低9.0（U1） | 未建立 | 1 | 无 | U4缺证；U2补采后9.8→9.4取较低组 |
| R2 | 无全量分；已评最低10.0（U1/U2/U3） | 未建立 | 0 | 无 | UI-01–04已解决；U4仍缺证，新增原生Chrome200%部分冒烟 |

目标指纹与R1相同，不重建基线。[M6 UI验收修复](DOCS-SSG-P4.records/M6.md#ui-验收修复)登记一次普通修复、重构轮无；四项未再成立，不触发顽固差距；没有三个具备全量分数的轮次，不判全局平台期。

## 上轮差距复核

上轮处置均取自M6“UI验收修复”节：**已修改待评审**，本轮独立看图后作如下复核。

| 编号 | 上轮严重度与摘要 | 上轮处置 | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-01 | P1 · Show more后键盘焦点丢失 | 保留可聚焦节点，末批归焦输入，已修改待评审 | **已解决** | [40组焦点](DOCS-SSG-P4.ui-captures/r2/U1-more-40-focus-1280-light.png)、[60组](DOCS-SSG-P4.ui-captures/r2/U1-more-60-focus-1280-light.png)、[末批](DOCS-SSG-P4.ui-captures/r2/U1-more-98-focus-1280-light.png)：20→40→60→80均为Show more BUTTON获焦，98组后INPUT获焦，selected与activeDescendant始终为`rd-search-option-1-2-1`；U1独立复核，运行期另确认40组按钮保留3px可见焦点 |
| UI-02 | P3 · 窄屏dialog少6px（U1/U2） | 显式max-width，已修改待评审 | **已解决** | U1/U2分别独立复核；[375浅色](DOCS-SSG-P4.ui-captures/r2/U1-results-375-light.png)、[320深色](DOCS-SSG-P4.ui-captures/r2/U1-results-320-dark.png)、[375长列表](DOCS-SSG-P4.ui-captures/r2/U2-supp-longlist-04-375-light.png)。运行期375/320双主题均x=16、width=343/288；桌面576 |
| UI-03 | P2 · 缺译note缺code-bg | 引用现有code-bg，已修改待评审 | **已解决** | U3独立复核[1280浅色](DOCS-SSG-P4.ui-captures/r2/U3-fallback-1280-light.png)/[320深色](DOCS-SSG-P4.ui-captures/r2/U3-fallback-320-dark.png)；运行页背景为rgb(242,244,247)/rgb(22,27,38)，保持14px、12px 16px padding与中文note |
| UI-04 | P3 · “1 results” | 增加单数模板，已修改待评审 | **已解决** | U2独立复核[375模块恢复](DOCS-SSG-P4.ui-captures/r2/U2-module-retry-375-light.png)、[1280索引恢复](DOCS-SSG-P4.ui-captures/r2/U2-index-retry-1280-dark.png)、晚响应等均为“1 result”；长列表仍“98 results” |

UI-01/02/03在R1同时有机器实测依据，主agent已在当前运行页重新检查；没有仅凭修复者自述或代码关闭问题。UI-02由两个单元分别覆盖对应视口，不将某单元结论外推至未看条件。

## 差距清单

本轮没有已证实的未关闭产品差距；无新增编号。UI-01–04保留历史身份与上述关闭证据，不重排或复用编号。U4与未操作状态属于缺证，不登记为假想产品缺陷。

## 待裁定事项

无。既有规范已确认，期望未变化；未发现目标冲突、被拒绝的修法或停滞项。环境不足不记为用户豁免。

## 建议（不扣分）

- 可考虑高亮摘要中全部可见命中、为截取摘要加省略号。
- 窄屏次要入口可更紧凑，长导航可考虑内部滚动，禁脚本说明可更接近入口。

以上均无强制规范依据，不要求借此重新设计。

## 机器检查

三位评审原文、合并judge保存并回读后运行。工具为Playwright1.63.0 / Chromium153.0.8010.12 / axe4.13.0；完整数据见[runtime-checks.json](DOCS-SSG-P4.ui-captures/r2/runtime-checks.json)、[摘要](DOCS-SSG-P4.ui-captures/r2/runtime-summary.json)。所有测量来自运行中的页面。

| 检查 | 实际结果 | 限制 |
| --- | --- | --- |
| 横向页面溢出 | 57个页面/状态样本均0px | 不是完整页面下半部、原生缩放或真机走查；宽内容的内部容器尺寸已保存，未完整手动横滑 |
| axe | 30次扫描，violations 0 | 37个incomplete规则实例：color-contrast13、aria-required-children2、aria-valid-attr-value22；尚未逐节点人工确认，不称WCAG全量通过 |
| 键盘 | 4组模块/索引错误下Tab准确到Retry、Enter恢复且保留查询；U1连续分页与末批焦点由独立材料确认，运行期40组再复核按钮可见3px ring | `UI-01-second-enter`即时观测发生在下一批加载中，记录仍40组；不冒称该瞬时样本已完成60组。等待实际组数后的60/80/98证据在U1采集与评审中；完整原生Tab/IME仍未验 |
| 尺寸/颜色 | 375/320双主题dialog=343/288px、x=16；note两主题为规定code-bg | 只报告已测节点，未据源码外推 |
| 中文与404 | 中文缺译搜索实际到`/rustify-ui/zh/page-0051/`；两视口中文深404搜索实际到`/rustify-ui/zh/quickstart/`；root/meta激活保持对应base | 首页动作、语言链接、quickstart各章节锚点等未逐项操作 |
| 禁脚本 | quickstart与深404搜索button均disabled，页面实际存在“启用 JavaScript 后可搜索。你仍可通过导航链接阅读文档。” | 阅读下半部与演示初帧未完整走查 |
| 日志与网络 | 采集361个请求，无pageerror/CSP错误；9条console error=4次注入503+5次深404；5条requestfailed=4次503模块+1次采集器导航竞争后主动关闭产生的page-0002 ERR_ABORTED | 失败attempt保留；对应a2重拍已导航成功，无额外脚本异常。原生Chrome补充没有完整网络日志 |
| 减少动态效果 | 57个实测样本reduce=true、scrollBehavior=auto | 未操作真实系统偏好切换或正常动效全过程 |
| 原生200% | CUA实际Cmd++五次，Chrome AX显示200%，阅读/搜索可见，查询返回1 result；结束恢复100% | 仅部分冒烟；未测完整官方阅读/代码/宽表路径，未补齐U4 |
| 性能、Rust、lint、全套E2E | 本轮未重跑 | 本次没有产品修复；100页语料只用于视觉/交互，不替代1000页性能门，也不把M6修复自检算本轮执行 |

## 采集与覆盖

- 被测HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`，main/dirty；初始tracked diff SHA256 `4a112900b88a157f42e6bca69d17e897dd6c5e2f805da137c97ca9c095e2662c`。该指纹不包含未跟踪文件；原有计划/SDLC/历史报告工作树保留。原计划SHA256 `4ec47edde1ede1fbcd5527a4df0db7d61261cc8d5422206ca94afa5d17b87871`，本轮未改。
- 固定实装CLI0.1.27，C `1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a` / R `740610db164b3f396b0fcc551f5a5ef8f15da716`；当前相关产品源码、配置、内容与候选无diff。按仓库serve.sh重新生成官方与100页/语种固定语料；official build_id `dd5022c567200779`、search root `80da98ab7be5b50d`、subpath/meta `176bea4ed7e4d96f`。身份与manifest hash见[baseline](DOCS-SSG-P4.ui-captures/r2/baseline.json)、[manifest清单](DOCS-SSG-P4.ui-captures/r2/build-manifests.json)。
- 沿用人工算法 `manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`，唯一“单元”“规范要点”节按LF和长度前缀hash，无附件。它是既有人工历史的算法，不冒称Phoenix兼容。engine-probe仍退出4（digest/size不匹配）；无managed索引，继续人工，未安装、绕过校验或begin清零。原始探测见[tool-probes](DOCS-SSG-P4.ui-captures/r2/tool-probes.md)。
- 正式评分材料91张：U1=38、U2=23、U3=30；另有原生Chrome2张JPEG和事后机器1张PNG，共94张有效本轮原图。主agent全部实际开图；另1张导航竞争中的未登记图排除，不用旧图/模拟页/拼图补格。逐图URL、时间、视口、DPR、主题、请求干预、动作及hash见[采集清单](DOCS-SSG-P4.ui-captures/r2/capture.md)和各单元JSON；来源文件hash在[source-hashes](DOCS-SSG-P4.ui-captures/r2/source-hashes.json)。
- 3位全新上下文subagent，model/effort继承、无override，不派生；U1读本轮38/上轮28，U2读本轮23/上轮23（补充只补开图覆盖），U3读本轮30/上轮2张note比较图。材料只含目标、单元图片/采集/操作、上一轮本单元结论；无代码、完整报告、修复记录、机器结果或门槛。原文与工具轨迹见[judging-provenance](DOCS-SSG-P4.ui-captures/r2/judging-provenance.json)及各judge；白名单是信息边界，不声称权限沙箱。无需确认评审，未以争议换评审。
- U1/U2部分互补证据可在报告层确认：加载前聚焦、窄屏长列表、Close键盘归焦、重开保留查询有U2证据；不回改U1原评审对其材料边界的说明。未操作的IME、hover/按下态、完整Tab、768–1279两栏、阅读页下半部/宽表横滑、quickstart章节和首页/语言入口等保持未知，详见原文。没有产品差距待核实到P0/P1的情况。
- U4仅本轮Chrome原生200%定制页阅读/搜索；Kimi当前UA Chrome154.0.0.0（100%时CSS1331×773、DPR2）不证明完整最新稳定身份；Chrome UI提示重启可更新，本机未见Firefox App。Safari本轮未完整重验，R1可操作通道不冒称本轮矩阵。iOS/Android实体设备、软键盘和VoiceOver记录仍不可用。所需获取方式见[capture-notes](DOCS-SSG-P4.ui-captures/r2/capture-notes.md)。
- 环境噪声：延迟响应场景首次未确认旧请求实际发出，采集器改为先等真实拦截再关闭重开，保留原失败日志；导航补采首次旧URL过早满足等待，a2等URL变化与networkidle后成功。只修采集器，未改产品。原生截图保留Kimi边缘提示；返回JPEG按实际格式保存，不裁图。
- 四个临时服务已按实际PID/命令核对后停止，端口4199/4204/4205/4206无监听，其他服务未动；[生命周期](DOCS-SSG-P4.ui-captures/r2/server-lifecycle.md)。本轮产物和既有未提交记录仍仅在当前工作树，未共享到远端。计划6/7、M7和PRD终验状态不因本次分数改变。

## 下一步

本轮无产品差距需要apply。需提供可操作的当前稳定Chrome/Firefox/Safari、iOS Safari和Android Chrome实体设备或远程原生会话，并补VoiceOver+Safari实际朗读记录；补齐后仍按全部目标矩阵重拍复审：

```text
$review-ui docs/plan/DOCS-SSG-P4.md
```

已登记的UI修复代码尚不能沿用修复前代码评审结论；全量UI通过后，再按该技能流程由用户发起`$review-code docs/plan/DOCS-SSG-P4.md`复审UI修复。本轮不自行启动修复、代码复审或发布。
