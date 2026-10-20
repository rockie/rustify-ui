# DOCS-SSG-P4 R4 · 原生补证采集清单

- 本轮范围仅U4；U1–U3本轮未复核，R3历史结论保留。目标包、规范与比较口径未改。
- HEAD `93c4d101055f16c4493c15abde7b43141f1f8a47`，初始tracked diff SHA256 `3d2f056d84b67c11f5d6e75cc2f74c87632170cd047a5507a411e1314ef1e87f`，不含未跟踪文件；完整状态见baseline.json。
- 固定实装CLI0.1.27，C `1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a` / R `740610db164b3f396b0fcc551f5a5ef8f15da716`；相关产品路径与C无diff。复用R3已经构建、manifest hash相同的真实产物，没有模拟页面。官方build_id `dd5022c567200779`，100页/语种搜索语料 `176bea4ed7e4d96f`。
- 目标全文hash `ed08081e68367b02d7ad695a041909e2c8ec26d78abe377376d1e1b0b7a6ad02`，与R3相同；沿既有人工规范指纹 `manual-sha256-normative-sections-v1:604274d613d8b2915e8c9c17ed09c2be6b1f6ef102adcb9bbef76275062486a4`。未接管managed或重建基线。
- 主入口为官方4199与搜索4205的`/rustify-ui/`。为用户提供的实体iPhone复用同一CLI的`--host 0.0.0.0`，局域网地址来自本机en0：`http://192.168.16.192:4199/rustify-ui/zh/`。没有外部部署、容器或软件安装。

## 已采与逐图出处

| 范围 | 本轮素材 | 来源与条件 |
| --- | --- | --- |
| macOS Safari | [36张原图、对应AX与操作](U4-Safari-capture.md)；逐图时间/动作/hash见[native-captures.json](native-captures.json) | CUA操作真实Safari；原生About27.0.1（22625.1.29.11.28）；原窗口/扩大窗口、100%与原生设置明确200%、跟随系统/深色；JPEG物理像素不当作CSS视口 |
| Safari首次module错误与恢复 | 包含在上述36张；[故障方法、字节身份与恢复时间](native-fault.json) | 仅暂扣真实生成产物隔离副本的module造成HTTP404；恢复相同字节后同页Retry，不替换DOM、CSS或产品脚本内容 |
| iPhone接入环境 | [接入记录](U4-device-access.md)、[使用中](U4-iPhone-connection-state.jpg)、[最新超时](U4-iPhone-timeout-state.jpg)及[AX](U4-iPhone-timeout-state.ax.txt) | CUA绑定用户已连接的iPhone Mirroring；仅环境窗口，不是产品页证据 |

Safari原图和AX均本轮新拍，未修改、裁剪、拼接或覆盖旧轮。初始AX增量缺URL的六帧曾被清单错误回退为About；独立评审指出后，已改为“本帧AX增量未含完整URL”。[原JSON](native-captures.before-url-correction.json)与[逐项修正记录](capture-metadata-correction.json)保留，所有图和AX字节未变。

## 未采到的格子与边界

- iPhone文档站、阅读/导航/搜索/主题/软键盘：镜像接入后因手机使用而中断；重连曾提示麦克风使用中，随后超时。没有产品图，不计通过或失败。需结束麦克风占用并锁屏、恢复可操作镜像后补采；设备型号/OS/CSS视口也未猜测。
- 当前稳定Chrome/Firefox、Android Chrome、VoiceOver实际朗读：本轮未接入和执行，不能由桌面Safari或R3视口样本代填。
- Safari完整键盘路径、全部词首边界集、IME组合输入、全部长代码/宽表与原生200%任务：部分尚无证据。普通Tab未到Retry的attempt如实保留；同页恢复是点击Retry完成，不记成键盘重试通过。
- U4整体缺证，不补零分或完整单元分；本轮也不是全量复核。

## 版本来源

Safari运行版本来自本轮原生About实拍。另查[Apple安全更新页](https://support.apple.com/en-us/100100)，该页在2026-10-16查阅时列最新macOS为27.0.1；它不单独证明Safari所有任务已通过。本轮不安装或升级浏览器。
