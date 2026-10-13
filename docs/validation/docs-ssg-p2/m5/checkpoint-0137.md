# M5 · 英文官方站

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T01:37:10+08:00 · 状态：进行中，官方站源码待编译。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5；未到冻结提交点。

- vellum 唯一源已译英；44 个原 Markdown 链接目标、两段 fenced 命令逐字保留，无中文剩余，所有原显式数字 token 均保留。原文恢复副本在本机 target/docs-ssg/m5-vellum-original.md。核查结果见 [translation.json](m5/vellum-translation.json)；这是翻译原记录，不重新声称其行为验收通过。
- README 介绍的“Start here”改为明确的本文件 Quick start 链接，SDK快速开始另保留；utility 的 protocol.rs:96 无效链接改为实际 `Limits` 声明的 protocol.rs#L112。历史R5 source-snapshot保留当时原文，不把旧快照当现行指南。
- 文档检查脚本exit1，21个条目来自保留的历史test-results引用当前不可见，以及utility的既有示例文件路径；未删除历史来源或造新文件来通过。44个vellum Markdown目标实际均存在，数字/代码/链接检查及diff检查通过。[原检查日志](m5/doc-check-r1.log)。

官方站源已新增：根rustify-docs.toml声明12源白名单和Guides12/Components24的导航；site独立Cargo工作区沿用两目标/四档位，注册静态首页与24分类，分类内容从CATALOG读取。24演示函数分在site/src/demos五个职责模块，注册使用直接模块路径以保持源码可寻址。首页调用待接的README原文片段接口，不复制介绍。新源尚未编译/构建/浏览器验证，白名单声明尚未成为实际读取或监听保护。

页面实施前已扩展[原阅读brief](../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md#p2-extension-the-english-sdk-site)，保留P1主题/系统字体/首页首屏结构，仅采用静态DOM与明确交互岛。

未完成：content.include解析/先选后读与排除映射/监听、README片段接口与原读依赖、定制首页壳接线、官方源码两目标编译/CI及浏览器/设计验收。未生成官方产物，未公开候选或部署；M5不记完成。M4验证在独立串行资源任务继续。
