# DOCS-SSG-P4 · UI 原生补证 R4

| 轮次 | 开发计划 | 目标包 | 被测版本 | 截图 | 上轮报告 |
| --- | --- | --- | --- | --- | --- |
| R4 · 限定U4原生补证；U1–U3本轮未复核 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | [目标包](DOCS-SSG-P4.ui-target.md) · `604274d613d8` | HEAD `93c4d101`，dirty `3d2f056d84b6`；CLI0.1.27，C `1bbc54b0` / R `740610db`；官方4199与搜索4205的`/rustify-ui/` | [R4采集清单](DOCS-SSG-P4.ui-captures/r4/capture.md) | [R3](DOCS-SSG-P4.ui-review.r3.md) |

## 验收判定

**需补证**。

本轮补采36张真实macOS Safari原图及对应AX，独立评审未发现已确认的新增产品差距。U4整体仍未评分；U1–U3本轮未复核，R3历史分数保留，不外推成本轮通过。沿用门槛8/10，但本轮没有可比较的完整单元分或全量轮分数。

用户已提供iPhone与本机Safari。iPhone Mirroring实际接入后，先因手机使用而结束，重连又提示麦克风使用中，最新状态为连接期间手机被使用导致超时。尚未进入本轮文档站，没有手机阅读、搜索或软键盘证据；这是环境缺证，不是产品失败。Chrome/Firefox、Android、VoiceOver及部分Safari原生任务仍缺。

本轮为DOM阅读站、搜索与语言/演示的原生验收；不涉及GPU。没有修改产品代码、计划、目标包、历史报告或修复记录，没有提交或发布。

## 评分与定性观察

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 单元总分 |
| --- | --- | --- | --- | --- | --- |
| U4 原生设备与辅助技术 | — | — | — | — | 未评分：矩阵不全 |
| U1–U3 | 本轮未复核 | 本轮未复核 | 本轮未复核 | 本轮未复核 | 不继承为本轮分数 |

一位全新上下文评审逐图打开36张Safari图及两张手机接入窗口图；[原文](DOCS-SSG-P4.ui-captures/r4/judge-U4-native.md)与[合并结论](DOCS-SSG-P4.ui-captures/r4/judge.md)已保存。

- 布局：所拍宽窗口三栏、窄窗口单栏/折叠菜单、原生200%正文换行与搜索控件可达；未代填全部长内容的横滚检查。
- 视觉：双主题层级、输入焦点、选中与命中、安静的缺译note清楚；未代替全部对比度测量。
- 交互：语言链接、Cmd+K、加载后选择与Enter章节导航、Copied可见反馈、点击Retry同页恢复、演示0→1→2→0、中文深404搜索与中文落地有真实证据。键盘Retry、剪贴板内容和朗读未证。
- 细节：所拍idle/loading/empty/error明确区分，中文控件、数量、404及缺译提示相符。

没有给局部Safari补总分，也没有将U4补零。无需临界通过确认评审。

## 轮次轨迹

| 轮次 | 本轮分数（最低单元） | 历史最佳 | 未关闭 P0/P1 | 重构轮 | 备注 |
| --- | --- | --- | ---: | --- | --- |
| R1 | 无全量分；已评最低9.0（U1） | 未建立 | 1 | 无 | U4缺证；U2补采后9.8→9.4取较低组 |
| R2 | 无全量分；已评最低10.0（U1/U2/U3） | 未建立 | 0 | 无 | UI-01–04已解决；U4仍缺证，新增原生Chrome200%部分冒烟 |
| R3 | 无全量分；已评最低10.0（U1/U2/U3） | 未建立 | 0 | 无 | 未见回归；新增Safari及官方页Chrome200%等部分证据；U4仍不全 |
| R4 | 限定U4；未评分，无全量分 | 未建立 | 0个已确认项 | 无 | 原生Safari补36图；iPhone接入超时，尚无产品图 |

目标全文hash与规范指纹未变，不重建基线。没有足够的全量分数计算平台期，也无本轮产品修复或重构轮，不触发停滞预警。

## 上轮差距复核

| 编号/范围 | 上轮结论 | 上轮处置 | 本轮结果 | 依据 |
| --- | --- | --- | --- | --- |
| UI-01–UI-04（U1–U3） | R3维持已解决，未见回归 | M6既有修复记录保留 | 本轮未复核 | 本轮只补U4，不关闭或重开范围外问题 |
| U4环境证据 | R3未评分、缺完整原生矩阵 | 等待可操作原生资源 | 部分资源已提供，仍缺产品任务证据 | Safari运行版本及实际任务已补；iPhone最新接入窗口显示超时 |

U4没有可继承的旧评分组或产品差距编号。本轮不以局部素材关闭全量环境缺口。

## 差距清单

无已确认的新增产品差距，不分配新编号。普通Tab未到Retry、输入工具未送入中文、加载期间尚不能选择等未完成attempt保留为证据边界；不能据此猜测完整产品路径不可达。

## 待裁定事项

无目标冲突、停滞或用户整体豁免。设备接入受阻保持需补证，不改目标或验收要求。

## 建议（不扣分）

无新增设计修法；优先补齐独立评审列出的原生任务，不为缺少环境而改产品。

## 检查与未验证项

独立原文和合并judge保存后，完成下列身份与恢复核对；[完整记录](DOCS-SSG-P4.ui-captures/r4/post-judge-checks.json)。

| 检查 | 实际结果 | 限制 |
| --- | --- | --- |
| 运行产物 | 官方与搜索入口的HTTP manifest均200，hash与本轮采集产品一致 | 只确认实际被测产物，不代表全量页面通过 |
| 故障恢复 | 模块恢复后HTTP200，SHA256与原文件一致；隔离产物228文件与真实CLI原产物字节一致，差异0 | 只在真实产物副本暂扣module获得404，没有替换界面或脚本内容 |
| 原生版本/缩放 | Safari About明确27.0.1（22625.1.29.11.28）；原生网站设置明确选择200%，并拍阅读及搜索 | 不用窗口像素推CSS视口；完整200%长代码/宽表/列表滚动仍未证 |
| 键盘 | Cmd+K、结果Down/Enter已操作；普通Tab/Option+Tab未完成Retry路线，实际恢复是点击Retry | Safari键盘导航偏好未核对，不能登记完整键盘通过或不可达 |
| 复制/演示/中文404 | Copied可见；演示最终回0；404中文搜索到中文quickstart | 未核对剪贴板实际字节、VoiceOver或全部会话/IME边界 |
| 原生溢出、axe、console/网络审计 | 本轮未通过Safari CUA通道执行测量或注入扫描；没有完整日志 | R3的Chromium结果不冒称本轮Safari结果；AX属性不代替实际朗读 |
| Rust/lint/全套E2E/性能 | 本轮未重跑 | 没有产品改动；100页语料仅用于真实交互，非1000页性能证据 |

Safari版本来自原生About实拍；另查[Apple安全更新页](https://support.apple.com/en-us/100100)，查阅时列最新macOS为27.0.1。该版本信息不单独证明全部Safari兼容任务已完成。

## 采集与覆盖

- 真实Safari36张原图和对应AX、两张iPhone接入窗口图；[逐图清单](DOCS-SSG-P4.ui-captures/r4/U4-Safari-capture.md)、[设备接入记录](DOCS-SSG-P4.ui-captures/r4/U4-device-access.md)、[hash清单](DOCS-SSG-P4.ui-captures/r4/evidence-hashes.json)。没有旧图、设计图、模拟页或拼图代替本轮产品图。
- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`，初始tracked diff SHA256 `3d2f056d84b67c11f5d6e75cc2f74c87632170cd047a5507a411e1314ef1e87f`，不含未跟踪文件；[baseline](DOCS-SSG-P4.ui-captures/r4/baseline.json)。固定CLI/C/R及相关源码未变，官方build_id `dd5022c567200779`、搜索`176bea4ed7e4d96f`。
- 沿既有人工指纹`manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`，目标全文hash `ed08081e68367b02d7ad695a041909e2c8ec26d78abe377376d1e1b0b7a6ad02`。无managed索引，未接管或清零。
- 一位全新上下文只读subagent，模型与effort继承、无覆盖或派生。只给目标、当前图/AX/采集和设备接入记录，没有代码、完整报告、修复记录、机器结果或门槛。不是单上下文评分；U4未打分，不做通过确认。
- 主agent核对说明：独立评审发现清单六个URL字段错误回退为About。已仅改成“本帧AX增量未含完整URL”，没有猜URL；[旧JSON](DOCS-SSG-P4.ui-captures/r4/native-captures.before-url-correction.json)与[修正映射](DOCS-SSG-P4.ui-captures/r4/capture-metadata-correction.json)保留，图和AX字节不变，评审原文保留此发现。
- 其他采集噪声：文件名含results/selected/destination的早期帧实际仍loading或对话框打开，未记成功；完成导航取ready-destination。一次旧AX索引变化导致再次Increment，最终按最新Reset实际归零。中文typeText未入值，成功查询取原生paste，不证明IME。原生缩放以明确200%控件及其后画面为据，不从按键次数猜数值。
- 手机曾可观察，但原有应用/页面不是本次产物，未保存其私人内容。之后因使用、麦克风占用及重连超时未进入本次站点；型号、OS、CSS视口未知。不能将模拟器或桌面Safari补填iPhone。
- 仍未覆盖：完整Chrome/Firefox/Android/VoiceOver，iPhone产品任务，Safari完整键盘/词首边界/会话/IME/减少动态/禁脚本及200%长内容等；详见独立原文补证表。没有将这些未测项记为成功或产品失败。
- 两个临时服务已核对PID/命令后停止，4199/4205无监听；[生命周期](DOCS-SSG-P4.ui-captures/r4/server-lifecycle.json)。故障module已恢复相同字节，Safari主题/缩放/窗口已恢复。重新接入手机时需先重启已有preview。
- 原计划6/7、M7及PRD终验保持；本轮只保存评审与证据，不改计划进度，不提交或发布。

## 下一步

先结束iPhone麦克风占用并锁屏，恢复可操作镜像。恢复后告知继续，Agent重启同一固定产物的preview，再从手机Safari采集阅读、导航、搜索、主题与软键盘。补齐相关原生资源后可运行：

```text
$review-ui docs/plan/DOCS-SSG-P4.md，只补U4原生证据
```

本轮没有产品差距需要apply，不自行启动修复、代码复审或发布。全部目标最终仍需一轮全量复审才能判通过。
