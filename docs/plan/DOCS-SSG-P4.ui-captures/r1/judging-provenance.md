# 独立评审派发与编号

| 独立评审 | 上下文 | 实际白名单 | 原始结论 | 开图轨迹 |
| --- | --- | --- | --- | --- |
| /root/ui_judge_u1 | fork_turns=none，继承模型/effort，只读、不派生 | 目标包、U1-capture.md/json、U1-operations.md、28张U1 PNG | judge-U1-a.md | 原文列出28/28逐图清单 |
| /root/ui_judge_u2 | fork_turns=none，继承模型/effort，只读、不派生 | 目标包、初版U2-capture.md/json、U2-operations.md、13张U2 PNG | judge-U2-a.md | 原文列出13/13逐图清单 |
| /root/ui_judge_u3 | fork_turns=none，继承模型/effort，只读、不派生 | 目标包、U3-capture.md/json、U3-operations.md、26张U3 PNG | judge-U3-a.md | 原文列出26/26逐图清单 |
| /root/ui_judge_u2_supplement | fork_turns=none，继承模型/effort，只读、不派生 | 同目标、补采后U2清单/操作、完整23张U2 PNG；没有给首位结论 | judge-U2-b.md | 原文列出23/23逐图清单 |

派发约束：只能打开列出的材料，不跟随目标来源到代码、旧验收、完整报告、修复记录、机器结果或门槛；不搜索仓库或网络。实际提示见本会话spawn_agent调用，四个agent均已完成。白名单是信息边界，不冒称文件系统沙箱。原文由主agent按返回内容保存；未把主agent意见添加进原文。

补采原因：U2首位评审说明键盘Retry/Close、module未执行前焦点、loading继续输入、长列表以及更严格晚响应顺序看不出；追加10张supp，原13图保留。B按完整23图重新评分，不看A结论。A9.8、B9.4，选B完整四维组；两者差距并集保留。不是因争议选择更有利评审。

| 原始key | 正式ID | 合并依据 |
| --- | --- | --- |
| U1 A 新-1 | UI-01 | 键盘Show more后BODY获焦 |
| U1 A 新-2 | UI-02 | 窄屏同一dialog少6px |
| U2 A 新-1 | UI-02 | 同一dialog在375视口相同6px收窄 |
| U3 A 新-1 | UI-03 | 缺译note未呈code-bg |
| U2 B 新-1 | UI-04 | 英文单条总数1 results |
| U2 B 新-2 | UI-02 | 同一375视口dialog6px收窄 |

原始分组的可挽回分不同，按最终选择的完整评分组说明，不把两个评审扣分叠加。U1总失分1.0、U2最终0.6、U3为0.3；没有给U4补分。
