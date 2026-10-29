---
object_id: "07fabee1-24c5-4ecf-a54a-87c0caddcb8a"
kind: "review"
review_type: "other"
artifact: "docs/plan/DOM-COMPONENTS-P1.ui-review.r3.md"
archived: false
observed_at: "2026-10-28T11:13:34+08:00"
baseline: {"head": "5d122d6231c690a61080b91c680e9a514d2034b8", "branch": "main", "worktree": "dirty"}
included_events: ["aac6bff4-3513-42a1-8273-35d182eb5e0b", "3f62b4e2-3ff2-405c-8f36-09c8a212fe3d"]
relations: [{"type": "reviews", "target": "37a2c7b4-7052-4f85-96e9-135b6f7667a1", "scope": "P1 Studio 与 DOM 组件界面；U1–U7 全量"}, {"type": "rechecks", "target": "f803a087-f2c1-4b5e-83f8-58952f2ad545", "scope": "R2 差距 UI-02–UI-156 与作者修复 548d9943"}]
---

# DOM-COMPONENTS-P1 · UI 验收评审 R3

来源：
- [完整报告](../../../../docs/plan/DOM-COMPONENTS-P1.ui-review.r3.md)（成文为 `sha256:447a6b13a5f0…`）
- [目标包](../../../../docs/plan/DOM-COMPONENTS-P1.ui-target.md)
- [截图与评审原文](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r3/)
- 复核前轮：[UI R2](f803a087-f2c1-4b5e-83f8-58952f2ad545.md)
- 关联：[计划](../plan/37a2c7b4-7052-4f85-96e9-135b6f7667a1.md)

| 维度/范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review/UI-R3 | review-recorded | 需要修改；本轮分数 8.6（U4），门槛 8 | confirmed | [评审事件](../../events/review/07fabee1-24c5-4ecf-a54a-87c0caddcb8a/aac6bff4-3513-42a1-8273-35d182eb5e0b.md)；门槛项只剩 UI-192（P1，机器检查） |
| disposition/UI-R3 | review-partially-applied | 64 条已修改待评审；其他处置均为 0 | confirmed | [作者处置事件](../../events/review/07fabee1-24c5-4ecf-a54a-87c0caddcb8a/3f62b4e2-3ff2-405c-8f36-09c8a212fe3d.md)；[M7 逐项记录](../../../../docs/plan/DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)，等待用户独立复审 |

本轮是 UI 验收评审。仓内枚举没有 UI 类型，review_type 记为 other。

- **被测版本**：HEAD `d5775d8a`，dirty，产品路径无 diff。
- **构建**：Studio `80cad3c530b54d9f`，catalog `7ea7f541a819b220`。
- **目标指纹**：`manual-sha256-normative-sections-v1:567117725f56…`。相对 R2 重建基线。
- **采集与评审**：实拍 139 张，10 位独立评审。每个单元取较低分；补采后重评了 U2、U3、U4。
- **评分**：

  | U1 | U2 | U3 | U4 | U5 | U6 | U7 |
  | --- | --- | --- | --- | --- | --- | --- |
  | 8.7 | 8.7 | 9.05 | 8.6 | 9.45 | 9.9 | 9.3 |

  U3–U7 达标；U1、U2 因门槛项 UI-192 未达标。
- **R2 复核**：已解决 60，不再适用 1，部分解决 27，仍存在 1。R2 的四条 P1 都已解决。
- **未关闭差距**：64 条，P1 1、P2 9、P3 54；没有待裁定事项。
- **机器检查**：console、CSP、页面级溢出均为 0；键盘与对话框焦点通过；200% 下编辑列内溢出 79px，列为 UI-192。
- **不变的状态**：不改变计划的 Ready 或 7/7；达标单元也不等于合并、发布或产品终验。
- **限制**：只用 headless Chromium；冻结矩阵外的状态未采。报告与证据未提交。

## R3 作者修复续接

修复 `dc59c8ff`、计划登记 `5d122d62` 已提交，64 条保持「已修改待评审」。当前源码、148 张自检矩阵、实际失败及未验范围见 [M7](../../../../docs/plan/DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)与[检查记录](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r3-apply/checks.md)。没有新增独立评分或验收结论；原评审版本、评分和门槛判定保持。SDLC 更新留在原未提交工作树，未推送。

## 已纳入事件

- [aac6bff4-3513-42a1-8273-35d182eb5e0b](../../events/review/07fabee1-24c5-4ecf-a54a-87c0caddcb8a/aac6bff4-3513-42a1-8273-35d182eb5e0b.md) · review/UI-R3
- [3f62b4e2-3ff2-405c-8f36-09c8a212fe3d](../../events/review/07fabee1-24c5-4ecf-a54a-87c0caddcb8a/3f62b4e2-3ff2-405c-8f36-09c8a212fe3d.md) · disposition/UI-R3
