---
schema_version: 3
event_id: "4dcc7ac2-17db-4cc1-b331-a32abe1f1cb3"
object_id: "8917bbeb-7add-4826-95ee-5cdf6bdff735"
kind: "prd"
title: "DOCS-SSG 静态文档站框架与 CLI PRD"
artifact: "docs/PRD-DOCS-SSG.md"
facet: "document"
scope: "document"
state: "prd-drafted"
source_state: "草稿，待评审。五项关键产品决策已经用户确认（2026-10-10，见 §5.1 D1–D4、D10），阻塞问题已清空；其余默认值列在 §5.2，作为非阻塞问题待确认。未写入 SPMS。"
evidence_status: "confirmed"
observed_at: "2026-10-10T00:29:40+11:00"
predecessors: ["b7e93e3a-7b55-4078-bac8-b5022a453090"]
baseline: {"head": "b0a6bc5b9010e57d1f0d758a37af573ea0712faa", "branch": "main", "worktree": "dirty"}
sources: [{"path": "docs/PRD-DOCS-SSG.md", "revision": "sha256:8702b729309a4a4d5511078fa3719998823ea6e5b2296a349cdf172ad71cbb84", "anchor": "§3 WORK-F17、§5.1 D10"}]
relations: []
---

# 修订：GPU region 按需启用（D10、WORK-F17）

- 事实与证据：用户确认“应该是按需用才对，不会全部项目都要用 GPU region”。PRD 新增决策 D10（rustify-ui 默认只含 DOM，GPU 能力由应用显式开启）和需求 WORK-F17（P1，单期，5 条 AC）；DEP-1 改为由 WORK-F17 交付；同步更新了 G4、§1.4、§2.1–2.3（S14）、§6（R11、UAT-14）、§7、§8。
- 校准依据：没有新增阻塞问题，没有评审，也没有计划承接，状态仍为 prd-drafted；来源版本改变，因此接续上一条事件。
- 未完项与限制：未写入 SPMS；未评审；PRD 文件尚未提交。
- 下一步及解除条件：同上一条事件。

- [docs/PRD-DOCS-SSG.md · WORK-F17](../../../../../docs/PRD-DOCS-SSG.md)
