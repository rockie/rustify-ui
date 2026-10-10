---
schema_version: 3
event_id: "f3bece9f-6ab9-4cd4-b936-eb3a6c41fc8b"
object_id: "8917bbeb-7add-4826-95ee-5cdf6bdff735"
kind: "prd"
title: "DOCS-SSG 静态文档站框架与 CLI PRD"
artifact: "docs/PRD-DOCS-SSG.md"
facet: "document"
scope: "document"
state: "prd-ready"
source_state: "草稿，已完成一轮评审修订（2026-10-10）。关键产品决策已经用户确认（2026-10-10，见 §5.1 D1–D4、D10–D14），没有阻塞问题；其余默认值列在 §5.2，作为非阻塞问题待确认。未写入 SPMS。"
evidence_status: "confirmed"
observed_at: "2026-10-09T22:33:55+08:00"
predecessors: ["4352b3fb-ba08-4217-b6bf-1c37b1897eb2"]
baseline: {"head": "b0a6bc5b9010e57d1f0d758a37af573ea0712faa", "branch": "main", "worktree": "dirty"}
sources: [{"path": "docs/PRD-DOCS-SSG.md", "revision": "sha256:c588c505348107acf7e90f1b9bf0a941d28e158abd85a3aeec62a57a130bfbd5", "anchor": "全文 §0–§8；§5.1 D11–D14；§7.1–7.3"}, {"path": "docs/PRD-DOCS-SSG.review.md", "revision": "sha256:e66600bb25a592696576141586fecfe381c0b78c30d75cd1ab821f60aa823852", "anchor": "§修订处置与复核"}]
relations: [{"type": "reviews", "target": "4a0edc00-c638-4313-af24-7726f88bc321", "scope": "document"}]
---

# 评审须改项已处置，PRD 可交接

- 事实与证据：完整评审的 10 项须改问题已在 PRD 中修订，并按[处置节](../../../../../docs/PRD-DOCS-SSG.review.md)逐项复核：AC 期次与 §7.1–7.3 一致，本地链接与 AC 引用无断链，旧表述已清理。新增 D11–D14、非目标 N15、DEP-9、F1/AC-5、F5/AC-5–6、F9/AC-5–7、F10/AC-4、N3/AC-6 与 UAT-15；F12 改为 llms.txt 索引。
- 校准依据：按状态模型，须改项已处置、没有当前有效的关键阻塞，为 prd-ready；原文状态行与之一致。
- 未完项与限制：Q1–Q6 为非阻塞默认值（Q5 包含 G1 门槛与测量合同）；SDK 组合 PoC 由下游计划完成；GitHub Pages 线上策略在 P2 实测；D12 改动的 F12 未经独立评审；没有写 SPMS，也没有承接的计划；PRD 未提交。
- 下一步及解除条件：/dev-plan docs/PRD-DOCS-SSG.md P1，计划头部记录来源 PRD 与期次。
