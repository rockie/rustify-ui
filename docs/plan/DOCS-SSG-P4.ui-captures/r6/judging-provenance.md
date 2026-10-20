# R6 · 评审派发与材料边界记录

- 三位评审均为全新上下文 subagent（delegate），只读指令、不派生；模型继承主 agent（返回标识 kimi-coding/k3），无 override。主 agent 本对话未参与被测界面实现或修复，但仍按规则外派评审，不自行打分。
- run 标识：U1 `349aa863`、U2 `dd3b28e3`、U3 `beb34dd8`；三位均一次完成，无补采重派、无确认评审、无因争议换评审。
- 每位评审只获得：目标包 `docs/plan/DOCS-SSG-P4.ui-target.md`；本轮 capture.md、本单元 capture.json、operations.md 与本单元全部截图（U1=38、U2=23、U3=30，逐张名单写入派发指令）；上一轮（R3）同单元 judge 原文与 R3 同名图对照权限。
- 明确禁止：源码、计划、PRD、修复记录、完整报告、机器检查结果（runtime-checks.json、keyboard-checks.json、axe-incomplete.json、M1-/M2- 图）、browser-raw.jsonl、启动日志、baseline.json、其他轮次目录及材料内链接。门槛分未告知任何评审。
- 白名单是信息边界，不声称宿主提供文件权限沙箱。
- 评审原文原样落盘：judge-U1-a.md、judge-U2-a.md、judge-U3-a.md；合并结论 judge.md。
