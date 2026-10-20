# DOCS-SSG-P4 · UI 验收复审 R6

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R6 · 全量复审，沿用规范基线 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | [目标包](DOCS-SSG-P4.ui-target.md) · `604274d613d8` | HEAD `93c4d101`，dirty `a3feecba9c72`；实装C `1bbc54b0` / R `740610db`；官方 `http://127.0.0.1:4199/rustify-ui/`、搜索 `http://127.0.0.1:4205/rustify-ui/` | [R6采集清单](DOCS-SSG-P4.ui-captures/r6/capture.md) | [R5](DOCS-SSG-P4.ui-review.r5.md)（U1–U3 的上一评审轮为 [R3](DOCS-SSG-P4.ui-review.r3.md)） |

## 验收判定

**需补证**。

U1、U2、U3 在本轮已拍、已操作范围均为 **10.0/10**；UI-01–UI-04 维持已解决，未见新增可扣分差距，未关闭 P0/P1 为 0。沿用门槛 **8/10**。

U4 本轮没有新证据：用户说明本机桌面正被其他 agent 用于其他项目的 UI 验收，原生窗口采集会互相污染两边证据，本轮只做不抢前台的无头采集；iPhone 镜像未在运行，Firefox 未安装，Android 与 VoiceOver 会话仍无资源。因此 U4 未评分、不补零分，**没有全量轮分数或历史最佳**，不判全量通过。没有临界通过确认条件、停滞预警或需用户裁定事项。

本轮采用 DOM 路径：验收静态阅读站、搜索与语言界面，计划未要求 GPU 呈现。产品代码、计划、目标包、修复记录及历史报告未修改；没有提交、推送、发布或启动修复。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | R1 → R2 → R3 → R6 | 原文 |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| U1 搜索主路径 | 3.0 | 3.0 | 3.0 | 1.0 | 9.0 → 10.0 → 10.0 → 10.0 | [U1](DOCS-SSG-P4.ui-captures/r6/judge-U1-a.md) |
| U2 搜索加载与恢复 | 3.0 | 3.0 | 3.0 | 1.0 | 9.4 → 10.0 → 10.0 → 10.0 | [U2](DOCS-SSG-P4.ui-captures/r6/judge-U2-a.md) |
| U3 官方双语阅读与入口 | 3.0 | 3.0 | 3.0 | 1.0 | 9.7 → 10.0 → 10.0 → 10.0 | [U3](DOCS-SSG-P4.ui-captures/r6/judge-U3-a.md) |
| U4 原生设备与辅助技术 | — | — | — | — | 未评分（R4/R5 局部补证，不计分） | [R6缺证说明](DOCS-SSG-P4.ui-captures/r6/capture.md#u4-原生设备与辅助技术本轮未采到) |

三位全新上下文评审各负责一个单元，逐图查看其全部本轮材料，完整采用各自四维组。逐维理由见[合并结论](DOCS-SSG-P4.ui-captures/r6/judge.md)及原文。已评最低 10.0，三单元并列；分数不代表未拍过程、精确 CSS 数值或辅助技术已验收。主 agent 没有改分或删除差距。本轮不判通过，未触发临界确认评审。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | ---: | --- | --- |
| R1 | 无全量分；已评最低 9.0（U1） | 未建立 | 1 | 无 | U4 缺证；U2 补采后 9.8→9.4 取较低组 |
| R2 | 无全量分；已评最低 10.0（U1/U2/U3） | 未建立 | 0 | 无 | UI-01–04 已解决；U4 仍缺证 |
| R3 | 无全量分；已评最低 10.0（U1/U2/U3） | 未建立 | 0 | 无 | 未见回归；新增 Safari/Chrome200% 部分原生证据；U4 仍不全 |
| R4 | 限定 U4（Safari 36 图）；无单元分 | 未建立 | 0 | 无 | iPhone 接入超时；U4 未评分 |
| R5 | 限定 U4（iPhone 41 图）；无单元分 | 未建立 | 0 | 无 | 无确认新增产品差距；软键盘/IME、内部滚动、VoiceOver 等仍缺 |
| R6 | 无全量分；已评最低 10.0（U1/U2/U3） | 未建立 | 0 | 无 | 未见回归；U4 本轮无新采集（桌面共享冲突，资源未变） |

目标全文 hash 及规范指纹与 R1–R5 一致，未重建基线。[M6 修复记录](DOCS-SSG-P4.records/M6.md#ui-验收修复)登记一次普通修复、无重构轮；没有差距重新成立。各轮均无全量分，不触发全局平台期。

## 上轮差距复核

R3/R5 没有未关闭产品差距。以下对历史关闭项检查可见回归；R5 的三项 iPhone 待核实因设备不可用保持原状。

| 编号 | 上轮严重度与摘要 | 上轮处置 | 本轮结果 | 本轮依据 |
| --- | --- | --- | --- | --- |
| UI-01 | P1 · Show more 后键盘焦点丢失；R2 已解决 | 保留可聚焦节点、末批归焦输入 | **已解决，未见回归** | U1 评审独立复核连续 20→40→60→80→98（[40组](DOCS-SSG-P4.ui-captures/r6/U1-more-40-focus-1280-light.png)、[末批](DOCS-SSG-P4.ui-captures/r6/U1-more-98-focus-1280-light.png)）；运行期 before/after/secondEnter 焦点均在 Show more、3px primary 焦点环、总数恒 98 |
| UI-02 | P3 · 窄屏 dialog 少 6px（U1/U2）；R2 已解决 | 显式 max-width | **已解决，未见回归** | U1/U2 评审各自核对；运行期实测 375/320 双主题宽 343/288px、x=16，桌面 576px |
| UI-03 | P2 · 缺译 note 缺 code-bg；R2 已解决 | 引用既有 code-bg | **已解决，未见回归** | U3 评审比对 R3 同名图；运行期背景 rgb(242,244,247)/rgb(22,27,38)、14px、12px 16px padding，lang=zh |
| UI-04 | P3 · “1 results”；R2 已解决 | 增加单数模板 | **已解决，未见回归** | U2 评审核对恢复帧；运行期 module-retry 总数为“1 result”，长列表保持“98 results”；键盘走查改词后 Retry 显示新查询结果 |

评审只收到 R3 关闭摘要，未补造原问题定义；主 agent 在独立评分后读取 R5/R3 完整报告及 M6 原记录，并复测原机器差距。没有只凭代码或修复者自述关闭问题。

## 差距清单

本轮没有已证实的未关闭产品差距；没有新增编号。UI-01–04 保留原编号及历史关闭证据。缺少操作与环境的证据不登记成假想产品缺陷。

## 待裁定事项

无。目标沿用既有用户确认，未修改期望或发现冲突；无不采纳、停滞或豁免记录。U4 环境不足保持需补证。

## 建议（不扣分）

- 摘要内多次命中可全部高亮；截断摘要可加省略号（U1）。
- 后续轮可补「错误态改词后再 Retry」与「IME 组合输入」两帧关闭长期未证项；确定性产物下可在 capture.md 预注「逐位相同即预期」口径（U2）。
- 窄屏次要入口可收紧；长导航可内部滚动；禁脚本说明可靠近搜索入口；摘要可按词边界截断（U3）。

均无强制规范依据，不要求为这些建议重新设计。

## 机器检查

各独立原文及合并 judge 保存后执行。工具为 Playwright 1.63.0、Chromium 153.0.8010.12、axe 4.13.0；全部测量来自真实运行页面。结果不交给评审。

| 检查 | 实际结果 | 限制/证据 |
| --- | --- | --- |
| 页面横向溢出 | 57 个页面/状态样本均 0px；键盘走查 23 条样本也为 0px | [runtime-checks.json](DOCS-SSG-P4.ui-captures/r6/runtime-checks.json)、[keyboard-checks.json](DOCS-SSG-P4.ui-captures/r6/keyboard-checks.json)；未代替原生完整矩阵 |
| axe | 30 次扫描，violations 0 | 37 个 incomplete 规则实例与 R3 逐项相同（color-contrast 13、aria-required-children 2、aria-valid-attr-value 22）；[原始节点/原因](DOCS-SSG-P4.ui-captures/r6/axe-incomplete.json)。未逐节点人工确认，不称 WCAG 全量通过 |
| 键盘与恢复 | 4 组两类故障 Tab→Retry/Enter 恢复（retryFocus 均 true、查询保留、结果归焦输入）；跳过链接、语言菜单 Esc、对话框 Tab/Shift+Tab、Esc 归焦、重开保留查询、主页主次操作、语言链接、目录锚点、代码横向滚动、禁脚本导航、错误态改词后键盘 Retry 均实测通过 | [keyboard-checks.json](DOCS-SSG-P4.ui-captures/r6/keyboard-checks.json) 及 M2-*.png 关键帧。完整 IME/全部控件路径仍未验 |
| UI-01 运行期复测 | Show more 连续 Enter 20→40→60，焦点恒在 Show more（3px primary 环）、总数恒 98、选择保留 | runtime-checks.json flows |
| 正式采集日志 | 323 个请求；0 pageerror、0 CSP；8 条 console error 与 8 条 http-error 均为 3 次注入 module 503 及 5 次深 404；3 条 requestfailed 均对应注入 module 503 | [runtime-summary.json](DOCS-SSG-P4.ui-captures/r6/runtime-summary.json) 及 browser-raw.jsonl |
| 运行产物身份 | 四入口 HTTP manifest 与本轮本地构建逐字节相同；build_id 与 R2–R5 逐位一致 | [build-manifests.json](DOCS-SSG-P4.ui-captures/r6/build-manifests.json) |
| 原生/真机/朗读 | 本轮未执行（原因见判定）；R3/R4/R5 历史局部证据保留在原轮 | [capture.md U4 节](DOCS-SSG-P4.ui-captures/r6/capture.md#u4-原生设备与辅助技术本轮未采到) |
| Rust/lint/全套 E2E/1000 页性能 | 本轮未重跑 | 无产品改动；100 页语料只用于界面与交互，不代替 M6 正式性能门 |

## 采集与覆盖

- 正式矩阵 91 张本轮原图（U1=38、U2=23、U3=30），与 R3 同名图逐位一致——产物四入口 build_id 与 R2–R5 逐位相同（确定性），渲染工具与条件同版本，逐位相同为预期；新鲜度由本轮采集进程、逐图拍摄时间与请求日志佐证（U2 评审提出的流程层待核实已由主 agent 核对，非产品差距）。逐图 URL、时间、视口、主题、干预、动作和 hash 见[清单](DOCS-SSG-P4.ui-captures/r6/capture.md)及各单元 JSON。没有旧图、模拟页或合成图补格。主 agent 抽查了窄屏、错误、缺译、焦点与落地关键图。
- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`，初始 tracked diff SHA256 `a3feecba9c72699241545463fdfe93f89752df78cef3a4fc2b78b2b78b77ecb0`，不含未跟踪文件；与 R5 的 diff 差异只来自台账/计划登记文本，产品路径干净；固定 C/R 与相关产品路径无 diff。[baseline](DOCS-SSG-P4.ui-captures/r6/baseline.json) 保留原工作树。计划 hash `4ec47edde1ede1fbcd5527a4df0db7d61261cc8d5422206ca94afa5d17b87871` 与 R3 相同，本轮未改计划。来源文件 hash 见 [source-hashes.json](DOCS-SSG-P4.ui-captures/r6/source-hashes.json)（与 R3 全同）。
- 固定实装目录 `target/docs-ssg/p4/install-1bbc54b0` 因 mbx 缓存目录曾被清空，本轮按 M6 同一命令与进程级 remap 原样重装并核对 `--build-info` 与 install --list（0.1.27 / source 完整 C / panic unwind）。无新容器/基础设施。
- 目标全文及规范指纹与 R1–R5 一致：`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`；不声称 Phoenix 兼容。既有人工历史且无 managed 索引，本轮未接管、重置或重新探测 engine。
- 3 位全新上下文 subagent（delegate），模型/effort 继承未覆盖（返回标识 kimi-coding/k3），只读、不派生；每位只读目标包、其单元本轮采集/操作/截图及 R3 同单元结论/图片。本轮逐图覆盖 38/23/30。不是单上下文评分；无确认或因争议换评审。派发与材料边界见 [judging-provenance](DOCS-SSG-P4.ui-captures/r6/judging-provenance.md)。
- 机器补充未覆盖：实际 IME 组合输入、完整 hover/pressed/动画、768–1279px 两栏、复制/演示全键盘路径及全部章节/代码/宽表走查。U4 原生部分整体缺证。本轮缺证不涉及待核实 P0/P1 产品差距。
- 环境噪声：系统 python3 缺 tomllib 致官方站与语料首次构建失败，改用 mise python@3.14.0 重建成功；失败遗留的已注册 git worktree 经 prune 清理；自写启动脚本曾误用 wait 阻塞 4204/4206，改为直接启动；mbx 缓存目录曾被清空致固定实装缺失，按合同重装。以上均不改变产品或任何截图。为 U4 尝试开起的 Chrome 窗口（id 1319606462）首帧被其他应用遮挡，文件保留为无效尝试记录、按规则当作没采到；窗口留待用户处置。
- 四个临时服务已按实际 PID 核对后 SIGTERM 停止，4199/4204/4205/4206 无监听，[生命周期](DOCS-SSG-P4.ui-captures/r6/server-lifecycle.json)。既有未提交工作树保留，本轮产物未提交到远端。
- 计划仍为 6/7，M7 和 PRD 终验不因本轮分数关闭。后续状态登记只索引本次独立评审，不修改计划实施进度。

## 下一步

本轮没有产品差距需要 apply。需可操作的当前稳定桌面浏览器窗口通道（桌面空闲时）、iOS Safari/Android Chrome 实体设备或可记录朗读的 VoiceOver+Safari 会话，完成 U4 矩阵后做全量复审：

```text
$review-ui docs/plan/DOCS-SSG-P4.md
```

桌面空闲后可只补原生桌面部分（Safari/Chrome 真实窗口、200% 缩放），命令相同并说明范围。全量 UI 通过后，已有 UI 修复代码仍须由用户发起 `$review-code docs/plan/DOCS-SSG-P4.md` 复审；本轮不自行启动修复、代码复审、PRD 差异核查或发布。
