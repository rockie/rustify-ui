# DOCS-SSG-P4 · UI 验收评审 R1

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R1 · 首次规范基线 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | [目标包](DOCS-SSG-P4.ui-target.md) · `604274d613d8` | HEAD `a6c076a7`，dirty `945d85fd8092`；实装 C₅ `936149b9` / R₅ `58b7a0d9`；[官方入口](http://127.0.0.1:4199/rustify-ui/) | [采集清单](DOCS-SSG-P4.ui-captures/r1/capture.md) | 无 |

## 验收判定

**需补证**。

U4 缺完整原生稳定浏览器、实体手机、原生200%及VoiceOver证据，因此全量轮没有本轮总分或历史最佳；不把U4记为0分。已评单元为 U1 **9.0**、U2 **9.4**、U3 **9.7**，门槛为默认 **8/10**。同时存在 **1项P1、1项P2、2项P3**，其中UI-01的键盘焦点丢失属于门槛项，不能因分数较高而通过。

本轮只读验收DOM阅读站和搜索，不改产品代码、计划、历史报告或里程碑进度；没有提交、推送或发布。用户已明确确认按现有规范继续。

## 评分

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 | 采用材料 |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| U1 搜索主路径 | 3.0 | 3.0 | 2.1 | 0.9 | 9.0 | [评审A](DOCS-SSG-P4.ui-captures/r1/judge-U1-a.md)，28张图 |
| U2 搜索加载与恢复 | 2.9 | 3.0 | 3.0 | 0.5 | 9.4 | [补采评审B](DOCS-SSG-P4.ui-captures/r1/judge-U2-b.md)，23张图；[A](DOCS-SSG-P4.ui-captures/r1/judge-U2-a.md)为9.8 |
| U3 官方双语阅读与入口 | 3.0 | 2.7 | 3.0 | 1.0 | 9.7 | [评审A](DOCS-SSG-P4.ui-captures/r1/judge-U3-a.md)，26张图 |
| U4 原生设备与辅助技术 | — | — | — | — | 未评分 | 原生冒烟不能代替完整矩阵 |

U2取较低评审的一整组四维分与理由，差距取并集，不逐维拼分、不因分歧删除差距。每维理由及正式编号清单在[合并judge.md](DOCS-SSG-P4.ui-captures/r1/judge.md)。已评最低为U1的9.0，不是全量轮分数；本轮不满足通过条件，无需触发临界通过确认评审。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | ---: | --- | --- |
| R1 | 无全量分；已评最低9.0（U1） | 未建立 | 1 | 无 | U4缺证；U2补采后9.8→9.4取较低组 |

这是该前缀首次评分，不承接或改写旧的 bounded finish-review“ship”结论，也不把其当成本循环的历史分数。未发现本前缀UI修复记录；首轮没有停滞判断、历史差距复核或重构预警。

## 差距清单

### UI-01 · [P1] Show more 激活后键盘焦点丢失（U1 · 交互与状态 · 可挽回0.9）

- 位置：[激活前](DOCS-SSG-P4.ui-captures/r1/U1-keyboard-more-focus-03-1280-light.png)、[激活后](DOCS-SSG-P4.ui-captures/r1/U1-keyboard-more-04-1280-light.png)的底部按钮；[操作记录](DOCS-SSG-P4.ui-captures/r1/U1-operations.md)。
- 现状 → 期望：Tab 到 Show more 后按 Enter，结果20→40且当前选择保留，但活动元素变成BODY、焦点环消失 → 焦点应留在可见可操作的对话框控件。依据目标G3、S2、S3。
- 修法：保留更多按钮节点；若更新替换节点，完成后恢复焦点。最后一批移除按钮时送回输入或等价控件，不重置当前选择。
- 复验：真实键盘连续20→40→60；每步焦点可见，Tab/Shift+Tab正常，最后一批也有明确落点。
- 来源：独立U1 A新-1。主agent在运行页面重新复现：再次Enter仍为40条且BODY仍获焦，见[机器原始结果](DOCS-SSG-P4.ui-captures/r1/runtime-checks.json)的`UI-01-*`及[重拍](DOCS-SSG-P4.ui-captures/r1/M1-show-more-after-1280-light.png)。未改评审分数。

### UI-03 · [P2] 缺译 note 未呈现 code-bg 填充（U3 · 视觉 · 可挽回0.3）

- 位置：[1280浅色](DOCS-SSG-P4.ui-captures/r1/U3-fallback-1280-light.png)、[320深色](DOCS-SSG-P4.ui-captures/r1/U3-fallback-320-dark.png)，Architecture 标题下的“此页面尚未翻译。”。
- 现状 → 期望：提示与页面同底，仅细边框 → O3/G2规定code-bg：浅色#f2f4f7、深色#161b26，保持安静的稳定阅读提示。
- 修法：显式应用既有code-bg token，保留标题后位置、尺寸、圆角和文案；复拍两格。
- 来源：独立U3 A新-1。运行页面计算背景实际为`rgba(0,0,0,0)`，两主题的code-bg token分别存在且正确，见机器结果`U3-fallback-*`；不凭CSS源码推断。

### UI-04 · [P3] 单条英文结果显示“1 results”（U2 · 细节 · 可挽回0.5）

- 位置：[375重试成功](DOCS-SSG-P4.ui-captures/r1/U2-module-retry-375-light.png)、[1280重试成功](DOCS-SSG-P4.ui-captures/r1/U2-index-retry-1280-dark.png)，输入下的总数。
- 现状 → 期望：“1 results” → “1 result”，复数结果继续用results；数字本身正确。依据S2/S3及评审细节维度的文案准确性。
- 修法：在现有数量格式化/本地化处选择英文单复数，保留全部匹配页数和live更新位置。
- 来源：U2补采B新-1。A将其列为建议，B将其列为差距；原文均保留，主agent未改分或删项。

### UI-02 · [P3] 窄屏 dialog 比规定宽度少6px（U1、U2）

- 位置：[375搜索](DOCS-SSG-P4.ui-captures/r1/U1-results-375-light.png)、[320搜索](DOCS-SSG-P4.ui-captures/r1/U1-results-320-dark.png)、[375长列表](DOCS-SSG-P4.ui-captures/r1/U2-supp-longlist-04-375-light.png)。
- 现状 → 期望：375px时外宽337px、320px时282px，左右均19px → S1要求左右各16px，即343/288px；桌面576px正确。
- 修法：统一dialog最终inline-size/max-inline-size与border-box，落实`min(36rem, calc(100vw - 2rem))`，排除原生最大宽度的二次收窄；保留窄屏1rem padding。
- 可挽回：U1细节0.1、U2采用B组布局0.1；A的U2细节0.2留在原文，不重复累计。
- 来源：U1 A新-2、U2 A新-1、U2 B新-2合并。运行页实测x=19、width=337/282、max-width=`calc(100% - 38px)`；依据见机器结果`U1-results-375-light`与`U1-results-320-dark`。

编号映射按首次分配保持稳定：UI-01/02来自U1，UI-03来自U3，UI-04来自U2补采B；清单按严重度和可挽回分排序，不重排编号。

## 待裁定事项

无。规范来源及范围已由用户确认，未修改期望。原生与设备不足是需补证，不是豁免或目标冲突。四项差距均可修复，没有应由用户决定后才能修的设计取舍。

## 建议（不扣分）

- 摘要可考虑高亮所有可见命中、对截取末尾加省略号；没有相应明确规范，未额外扣分。
- 窄屏顶栏可更紧凑，长导航可考虑内部滚动，禁脚本说明可更接近入口；现有规则允许当前排布，不要求借此重设计。

## 机器检查

全部在独立评分和原文保存之后运行。工具：Playwright1.63.0、Chromium153.0.8010.12、axe4.13.0。完整事实为[runtime-checks.json](DOCS-SSG-P4.ui-captures/r1/runtime-checks.json)，原始采集日志为[browser-raw.jsonl](DOCS-SSG-P4.ui-captures/r1/browser-raw.jsonl)。

| 检查 | 实际结果 | 限制 / 差距 |
| --- | --- | --- |
| 页面横向溢出 | 57个页面/状态样本全部为0px；代码等宽内容有自身滚动区域 | 不替代原生200%或实体手机 |
| axe扫描 | 30次扫描，报告的violations为0 | 37个incomplete规则实例：color-contrast13、aria-required-children2、aria-valid-attr-value22；未逐节点完成人工复核，不能称全部可访问性已通过 |
| 真实键盘 | 快捷键、选中、Esc归焦、Retry/Close键盘、锚点进入均有实际操作；Show more丢焦点已复现 | UI-01；未测完整原生键盘矩阵与IME实际输入法 |
| 恢复补采 | module响应尚未执行时输入已聚焦；loading继续编辑最终查询；Tab/Enter重试、Shift+Tab/Enter关闭；新结果之后释放旧请求不覆盖 | U2补采由新评审B独立看图；不改A原文 |
| 运行期尺寸与颜色 | dialog宽度收窄6px；缺译note背景透明；焦点色/面板色在实测样本正确 | UI-02、UI-03 |
| 语言与404闭环 | 机器实测中文缺译激活进入`/zh/page-0051/`；官方中文404搜索激活进入`/zh/quickstart/` | 后采事实未交给U1/U3评分者，不擅自改其分数或将未看区域视为通过 |
| 控制台与失败请求 | 采集346个请求；无pageerror；8条console error对应3次故障注入503与5次深404；3条requestfailed均是注入模块503 | 未见额外脚本异常或CSP违规；原生冒烟没有完整console/network日志 |
| 减少动态效果 | 记录的reduce模式下scrollBehavior均为auto | 未采正常动态效果与真实系统切换全过程 |
| 性能 | 未重跑1000页首次/热查询、移动节流或构建基准 | 本轮100页用于视觉/交互，不借旧性能结论判分 |

## 采集与覆盖

- 原文计划SHA-256：`94dc01c9d1e6db07f68fa276278c0a40a5abfb031950b1fd884d458f8714990f`；全程未改该计划。当前工作树原有M7与SDLC变更保留。
- C₅为`936149b91f57f6dbbdb43969c8843f940348a11e`，R₅为`58b7a0d9eb913b7656120693905d66d70ab208d8`，CLI0.1.26。当前UI源码、配置、内容及相关测试路径与候选无diff；本轮重新构建官方与搜索产物。官方build_id=`ae467c9c97900d12`，搜索root=`f63a1f2403c3a37f`，subpath/meta=`e92daa8a0efb7328`。
- 目标规范指纹为`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`。Phoenix启动器实际返回“binary digest or size differs from the package manifest”，退出4；本前缀无managed索引或旧评分历史，采用人工流程。没有冒称接管managed循环，也没有把人工指纹冒充engine的`phoenix-ui-target-v1`；算法及字节范围见[baseline.json](DOCS-SSG-P4.ui-captures/r1/baseline.json)。恢复engine需按实际报错的固定包0.7.3恢复，不自动下载安装。
- 本轮82张原始PNG：U1 28、U2 23、U3 26；原生Chrome/Safari各2张补充，以及主agent键盘复现1张。逐图URL、视口、DPR、主题、时间、干预和hash在[采集清单](DOCS-SSG-P4.ui-captures/r1/capture.md)及分组JSON。主agent已逐张开图核对，未合成、拼接或修图，未覆盖首采。
- 4位全新上下文评审：U1、U2 A、U3、U2补采B；均只读、不派生，模型/effort继承。材料边界为目标包、相应单元截图/采集/操作记录，不含代码、修复自述、机器结果或门槛。保存每位原文与开图清单；这是信息白名单，不声称宿主提供文件权限沙箱。
- U1与U3仍有未看区域/操作：中间宽度两栏、正文下半部与代码/表格实际横滑、quickstart锚点、完整Tab环路、实际IME、hover/按下态、正常动态效果；其余缺证逐项见原评审原文。既不据此开产品差距，也不认定通过。机器补测的404和缺译导航另外明确列出。
- U4：真实Chrome扩展取得中文首页与搜索结果（UA为Chrome154.0.0.0，1272×773、DPR2）；原生Safari27.0.1经CUA新页签打开本轮官方站、Cmd+K输入ThemeScope得到3个结果，截图可用，见[原生记录](DOCS-SSG-P4.ui-captures/r1/U4-native-safari-operations.md)。因此不沿用旧记录的“Safari无可见通道”为当前事实。但本轮只冒烟，版本是否最新稳定及完整矩阵未验证；本机Firefox.app不存在，两实体手机/VoiceOver资源缺口沿计划保留；原生200%未测。
- 环境噪声：首次启动使用系统Python3.9缺tomllib，改本轮PATH至已有Python3.12.13后成功，无安装；禁脚本采集的rAF等待不前进，调整采集器仅在启用脚本时等待动画帧，原有图保留。Kimi截图带扩展边缘提示，原生补充不参与U1–U3打分。细节/来源hash及启动停止记录见采集清单。
- 原始浏览器证据与报告当前仅在本工作树，未提交/推送。台账只登记本轮UI评审与待处理项，不把它写为计划完成、已合并、已发布或PRD终验通过。

## 下一步

可先修复已核实差距：

```text
$apply-ui-review docs/plan/DOCS-SSG-P4.ui-review.md
```

补齐U4和上述未覆盖操作后，以相同目标、条件和全部矩阵复审：

```text
$review-ui docs/plan/DOCS-SSG-P4.md
```

补证需要已有的可操作原生稳定浏览器、iOS/Android实体设备或远程原生会话，以及VoiceOver+Safari实际朗读记录。此次没有要求变更或豁免发布前门槛，也不自行启动修复。
