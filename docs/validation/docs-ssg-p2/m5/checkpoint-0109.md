# M5 · 英文官方站前置内容

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T01:09:46+08:00 · 状态：进行中，仅前置内容。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5；未到冻结提交点。

- vellum 唯一源已译英；44 个原 Markdown 链接目标、两段 fenced 命令逐字保留，无中文剩余，所有原显式数字 token 均保留。原文恢复副本在本机 target/docs-ssg/m5-vellum-original.md。核查结果见 [translation.json](m5/vellum-translation.json)；这是翻译原记录，不重新声称其行为验收通过。
- README 介绍的“Start here”改为明确的本文件 Quick start 链接，SDK快速开始另保留；utility 的 protocol.rs:96 无效链接改为实际 `Limits` 声明的 protocol.rs#L112。历史R5 source-snapshot保留当时原文，不把旧快照当现行指南。
- 文档检查脚本exit1，21个条目来自保留的历史test-results引用当前不可见，以及utility的既有示例文件路径；未删除历史来源或造新文件来通过。44个vellum Markdown目标实际均存在，数字/代码/链接检查及diff检查通过。[原检查日志](m5/doc-check-r1.log)。

未完成：content.include白名单与排除映射/监听、README片段依赖、首页/24分类/36项导航、官方两目标/CI及浏览器/设计验收。未生成官方站，未公开候选或部署；M5不记完成。M4验证在独立串行资源任务继续。
