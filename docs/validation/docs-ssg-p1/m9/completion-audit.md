# P1 完成核查

范围：[原计划](../../../plan/DOCS-SSG-P1.md) §0/§8–10 的 P1 交付、V-1–V-15 和 UAT 子集；D-04 用户决定见[原文](../../docs-ssg-p1-gaps/m2/browser-decision.json)。安装源码为1b81de25f23a25702f61dbfd3d0ecdfe323d4d44/CLI0.1.4，writer9。P2–P4保持原分期。

| 要求 | 当前证据 | 核查结果 |
| --- | --- | --- |
| M1 / F17，默认DOM、GPU opt-in、生命周期与仓外消费者 | [M1](../M1.md)、本轮features/mode矩阵、[GitHub仓外](github-external.log)、[GitHub DOM12](github-dom-only.json) | 当前graph/导出/运行通过；M1的7GPU示例/theme-export 604适用回归保留原范围，GPU/SDK实现自gaps基线未改 |
| M2 / 原生SSR、CSS/高亮、CLI与构建profile | [M2](../M2.md)、库142/CLI77、[图](features.log)、[实际GitHub安装](github-toolchain.log) | 独立SSR图、完整HTML、11语言、原生工具链与unwind成立 |
| M3 / F2/F3/F15内容图、锚点、GFM、诊断 | [M3](../M3.md)、当前库/CLI测试、root/subpath/meta/dev及[真实GitHub预览](github-preview-checks.json) | 当前行为回归通过；五alert/GFM/代码/前言可读，GitHub自定义ID与include差异按已说明规则 |
| M4 / F1/F11/F14，五命令、模板、元数据、资源、生产事务 | [M4](../M4.md)、artifacts/CLI故障测试、实际安装模板/manifest、preview/header/meta测试 | 产物/资源/草稿/manifest/404/输出保全通过；本地与GitHub同SHA产物文件集合及字节一致 |
| M5 / F6/F7/F8/N5，阅读主题与无wasm基础交互 | [M5](../M5.md)、当前81生产页浏览器、截图、gaps真实Safari/Chrome模拟 | 导航/主题/复制/无脚本/窄屏/AA/地标通过；D-04延期实体项目保留 |
| M6 / F13，watch/SSE/滚动与错误恢复 | [M6](../M6.md)、本地dev22/[GitHub dev22](github-docs-dev.json) | 实际安装二进制的panic、last-good、依赖恢复、诊断与UAT通过 |
| M7 / N4/N6/N7，最小工具链、安全和故障恢复 | [M7](../M7.md)、本轮库/CLI faults/reliability/external、GitHub受限PATH/禁网 | 私有路径/资源/CSP/事务/离线/外链边界通过，无新增运行时工具依赖 |
| M8 / N1/N2/N3/N6 性能、部署与UAT | [M8](../M8.md)、[gaps M2](../../docs-ssg-p1-gaps/M2.md)、本轮115浏览器/已安装UAT | 原正式1000/1000保存证据存在；gaps500页与加载/保存/体积阈值通过。渲染/SDK代码自42757c6无diff，M9只改安装身份/说明与验收环境；按计划复用不受影响的性能/GPU证据 |
| M9 / 安装分发、帮助、维护文档与提交 | [M9](../M9.md)、[GitHub来源](github-installed-source.json)、GitHub安装/仓外/浏览器与预览，固定SHA的README/帮助/模板 | 本地/远端两步均通过；授权与推送已发生；最终提交/SHA回填由原计划完成记录证明 |

V-1/V-2：当前features及GitHub仓外/DOM；V-3：M1同集合全适用GPU回归、源码无新变更；V-4–V-11/V-13：当前Docs/CLI及115浏览器、实际GitHub安装与源预览；V-12：用户D-04批准的本轮矩阵与gaps证据；V-14：M8正式长跑及gaps同B回归；V-15：当前22命令矩阵与验收器变更后的xtask44/Clippy/fmt。原失败、跳过或未复验的历史范围保留其记录。

逐项核对提交可达、里程碑总数、记录链接及current源码变化；最终9/9与M9提交SHA仍须在完成记录回填后验证。整份计划的最终交付状态不由本表单独推导。
