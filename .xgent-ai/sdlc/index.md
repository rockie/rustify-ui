# .xgent-ai/sdlc 索引

用途：本层 SDLC 状态与导航。覆盖本目录直接文件与子目录，不含索引自身；原文及证据保留在原位置。

| 文件 / 子目录 | 用途 / 对象 ID | 入口 |
| --- | --- | --- |
| event-template.md | 协议、模型或状态模板 | [event-template.md](event-template.md) |
| events | 子目录入口 | [events](events/index.md) |
| protocol.md | 协议、模型或状态模板 | [protocol.md](protocol.md) |
| state-model.md | 协议、模型或状态模板 | [state-model.md](state-model.md) |
| status | 子目录入口 | [status](status/index.md) |

维护约定：改变本层内容者同步索引；按稳定 ID/路径保留历史，无变化不重写，索引不作为事件。详见[目录索引维护](protocol.md#目录索引维护)。

## 当前重点

- [WASM-TAILWIND](status/plan/be9305f5-b240-453b-bb54-5ff7d0ed7970.md)：[R5](status/review/03a93f04-5364-4e76-ab5c-f47fe6fa8851.md) 对 R4 修复范围复审**通过验收**；F-08 已解决，F-04 按当前契约不再适用（原行为未变），F-05–F-07 回归通过，新增问题 0。本轮无需 apply；scanner/完整上游/GPU视觉/性能/远端 CI 未重验，历史实施/文档状态另见对象页。
- [DX-REFINE](status/plan/7493fc99-98cd-4d7e-aaa7-ac842d36d490.md)：6/8，待四个新类别页真人键盘走查，随后 M8。
- [THEME-STUDIO](status/plan/0222c547-cbeb-412a-8c3e-3399b943670e.md)：6/6 历史交付；F-01/F-02 已修复验证，后续共享改动与独立复审待核对。
- [VELLUM](status/plan/fb86407f-8889-4844-99a0-a0e6a580a27f.md)：8/8 历史交付；后续相关提交及共享改动未由原终验覆盖。
- [融合 Leptos CSR 与 Makepad 的框架 PRD](status/prd/cc008e52-5073-46aa-aa2f-864f262e41b0.md)：完整版本 Q1–Q3 定稿问题尚未关闭；P1–P3 分期封板不等于完整 PRD 达标。
- [DOCS-SSG PRD](status/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735.md)：文档规范状态prd-ready，已批准矩阵/测量合同待同步；P1原计划已8/9，四项交付遗漏、真机及M9缺证见[覆盖报告](status/review/5b279eaf-0dc1-4c8f-a54a-672cc942633f.md)。P2–P4终验保留，未写SPMS。
- [DOCS-SSG-P1](status/plan/526c88c7-a4b3-4ea9-9193-371420b66d1d.md)：9/9完成；固定安装1b81de2（CLI0.1.4）、最终交付945398c、回填be2cf15。本地115+远端34浏览器、219 Docs Rust、1091 CSR/GPU工作区及模式矩阵/真实安装升级回滚通过；GitHub同SHA安装/SDK/源预览完成。独立gaps2/2，D-04最新版Chrome及两手机延期、P4完整矩阵保留。下一步由用户以review-code评审原计划。

- [DOCS-SSG-P2](status/plan/c0c94e42-32f2-4636-ae52-f3df76d4a463.md): v2 **Ready**; history retained. Implementation5/7, M1-M5 complete. [M6](../../docs/validation/docs-ssg-p2/M6.md) empty-R corpus6/CLI72/components2/dev3 and strict clippy/fmt/sources/CSS/catalog pass; next actual R/C freeze and paired/concentrated gates. Users/phones remain gaps; no push/publication.
## 初始化范围与验证边界

- 初始化观察：2026-10-07T10:37:52+11:00，协议 v3 / 状态模型 v2。
- 本地基线：main / 53652fcc405a2679df19eb10e252efd4a4236f50 / dirty；未 fetch/pull、未提交/推送，不声明远端同步。
- 登记 1 份 PRD、7 份计划（P1–P3 明确封板归档）与13个实际评审/复核对象。评审包括计划文档、代码、界面和有明确目标的架构方案裁决；局部 verdict 单列，不扩为全计划通过。
- PRODUCT.md / DESIGN.md / README.md 是产品与设计上下文，不另造正式 PRD 或竞争原文；方案、合成、todo、报告与记录用来源链接引用。
- P3 的12项修复仅有[处置记录](../../docs/validation/p3/review-fixes.md)，未找到原独立报告；保留历史证据入口，不伪造报告/轮次。
- 核对实际来源存在性、登记提交可达性、Theme Studio 修复清单8个源码指纹（8/8一致）与原审276文件清单（30项不同，包含修复和后续改动）。文件指纹不代替行为验收。
- 本次未运行产品单测、构建、浏览器、性能、部署或外部系统写入；历史命令结果均指向原记录，未计为本轮通过。原始日志、截图、test-results、ref 与远端CI未逐项重验。
- 原文同步缺口：Theme Studio 顶部 Proposed 与完成记录不同步；Vellum 快照仍写未提交，但本地4e5d584已提交可达；本次只修跟进状态，不修改原计划。
- 台账及AGENTS.md入口尚未提交，未随Git共享；WASM-TAILWIND原有实施成果仍未提交。
