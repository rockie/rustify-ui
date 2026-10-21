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

- [DOCS-LANDING](status/plan/ca735c8c-bc5e-44af-bb23-f591a378bc94.md)：[完整计划评审](status/review/57186ae5-4fae-4542-bbc9-8c186ed46d73.md)的 6×P2 都已修订并复核（review-applied）。规范状态 dev-plan-ready，实施 5/5（dev-plan-completed），M5 收口记录在 `1f49749c`（未推送）；[代码初审 R1](status/review/e9b98a4e-7b97-4d3a-8cab-dfcf465d9c31.md) 的 2×P2（透传配置边界、原生表单 URL）已修复并验证（review-applied，修复在 `0b27fdb8`）；[代码复审 R2](status/review/75bf2fdb-1034-40aa-8c05-f76eccb6b80c.md) 通过验收，F-01、F-02 已解决；其 F-03（P3，GET 表单的作者文档）已在 `c3c8b426` 修复并验证，待复审；用户决定本期只推送 `main`。
- [WASM-TAILWIND](status/plan/be9305f5-b240-453b-bb54-5ff7d0ed7970.md)：[R5](status/review/03a93f04-5364-4e76-ab5c-f47fe6fa8851.md) 对 R4 修复范围复审**通过验收**；F-08 已解决，F-04 按当前契约不再适用（原行为未变），F-05–F-07 回归通过，新增问题 0。本轮无需 apply；scanner/完整上游/GPU视觉/性能/远端 CI 未重验，历史实施/文档状态另见对象页。
- [DX-REFINE](status/plan/7493fc99-98cd-4d7e-aaa7-ac842d36d490.md)：6/8，待四个新类别页真人键盘走查，随后 M8。
- [THEME-STUDIO](status/plan/0222c547-cbeb-412a-8c3e-3399b943670e.md)：6/6 历史交付；F-01/F-02 已修复验证，后续共享改动与独立复审待核对。
- [VELLUM](status/plan/fb86407f-8889-4844-99a0-a0e6a580a27f.md)：8/8 历史交付；后续相关提交及共享改动未由原终验覆盖。
- [融合 Leptos CSR 与 Makepad 的框架 PRD](status/prd/cc008e52-5073-46aa-aa2f-864f262e41b0.md)：完整版本 Q1–Q3 定稿问题尚未关闭；P1–P3 分期封板不等于完整 PRD 达标。
- [DOCS-SSG PRD](status/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735.md)：文档规范状态prd-ready/stale，P2计划已承接、当前进度见计划状态页；P1原计划9/9、gaps2/2以各原计划为准。P2–P4终验与Firefox后续追踪保留，未写SPMS。
- [DOCS-SSG-P1](status/plan/526c88c7-a4b3-4ea9-9193-371420b66d1d.md)：9/9完成；固定安装1b81de2（CLI0.1.4）、最终交付945398c、回填be2cf15。本地115+远端34浏览器、219 Docs Rust、1091 CSR/GPU工作区及模式矩阵/真实安装升级回滚通过；GitHub同SHA安装/SDK/源预览完成。独立gaps2/2，D-04最新版Chrome及两手机延期、P4完整矩阵保留。下一步由用户以review-code评审原计划。

- [DOCS-SSG-P2](status/plan/c0c94e42-32f2-4636-ae52-f3df76d4a463.md)：原5/7和R2旧问题关闭保留；[本次覆盖](status/review/5fdbf99f-8f49-4629-b1c8-89b2335d0a6b.md) A2/B3/C0及原待核实项保留。[独立gaps](status/plan/f2f47f04-3082-4faf-96c8-48f1a319b6d4.md)实施2/2，F-01修复后本地R′ 745ba6b7/C′ 79a77825（0.1.17）配对；[独立代码复审R2](status/review/1f739015-f48c-4e90-8370-093b2257e67f.md)确认F-01已解决、修复增量通过验收，新增0、无需apply；[R1](status/review/ee23493d-e277-489f-b5a5-5fd7b2ef763d.md)原“需要修改”及修复历史保留；可选核对本增量PRD覆盖；[计划评审](status/review/e50f2cbd-5734-4670-a4d2-31fdae182094.md)历史文档处置保持。本人C4、设备/正式性能缺证与D16公开发布留原计划/P4，不记整P2通过。[覆盖复审R2](status/review/c8480584-6578-4587-972a-8294319685fc.md)：G-01/G-02/G-05已解决，新增G-06（dev面板反引号）按用户D-05由[gaps-2](status/plan/e9817d55-60e7-4c8f-9abb-4793cc82d5f3.md)承接，原文 Ready，实施 2/2；[计划评审与同轮处置](status/review/d589882c-01b3-4031-b976-e2eedc754763.md)：原1×P1、6×P2及“需要修改/建议Blocked”保留；F-01–F-07文档缺口已修订并复核，用户选定含反引号普通作者值整段等宽，修后Ready/dev-plan-ready、review-applied，评审时实施0/2。gaps-2 当前 2/2；[独立代码复审 R2](status/review/1b86dee0-14ed-45c9-8800-e3c18a9c4439.md)确认 PG-01 已解决、修复增量通过验收，新增 0、无需 apply。R‴ 6942ec0d / C‴ b31e28b7 0.1.19 配对与安装身份已核对；本轮 CLI 6/6、fmt、真实 DOM 定向 3/3（24 视图）通过，历史全量证据限定复用。原 R1“需要修改”及 review-applied 历史保留。[覆盖R3](status/review/e9ce88ff-9dee-4eee-afc5-eeb044bb2b7e.md)确认G-06/D-05已解决，新增A0/B0/C0；本轮CLI6/6、真实DOM6/6通过，无需新计划，原U项3与5/7、M6/M7保留。
- [DOCS-SSG-P3](status/plan/607022f0-588c-4810-a08b-24b7be2dcc6f.md)：v2原文Ready；实施4/5，本地V-4/V-7–V-9已通过，C₂ `bae5b882`（0.1.21）/R₂ `820cea0f`固定交付。M5仅因V-10完整稳定浏览器、真机和VoiceOver缺证阻塞；本地成果可供P4续接，未推送/公开发布。[原文档处置](status/review/c34cd2c3-3731-433a-90c7-8bb70fdbc9e8.md)与评审历史保留，实施时序调整尚无独立文档复核。
- [DOCS-SSG-P4](status/plan/f36dda5e-ce97-49af-926c-0146968daa01.md)：v2 Ready，实施7/7，M1–M7完成；最终C1 1146b7d8（CLI0.1.31）/R0 0fd34eda已公开。公共CLI/SDK/源预览及四轮Pages完整发布/回滚/恢复通过，每轮97文件与4项浏览器任务通过。桌面原生门按D17/D18闭合，手机取消、VoiceOver本人确认。[代码复审R2](status/review/b7178384-b6a0-4f39-9800-588a613f076a.md)通过验收，修复效果已解决；新增F-01（P3）：CLI帮助与新项目README仍以0.1.3为回滚目标；2026-10-20 已修复并验证（修复36e5f7e3，未推送），回滚目标改为最新已发布C2 0.1.32；[独立代码复审R3](status/review/459cfd1e-c07b-491f-9798-9bedd11b6296.md)已确认F-01源码解决、通过验收、新增0（12项定向测试及fmt/Clippy通过），旧二进制待后续CLI配对发布。R2待核实1–4及其他范围本轮未复核；可选review-ui、review-prd-dev-gaps。独立评审历史保留：[原计划评审及同轮处置](status/review/687ef358-5dbb-4e7a-af5f-dc54989a28b6.md)历史保留。 [本地代码R1](status/review/9cd6da78-0bc0-4297-9f6b-6fffde099e72.md)通过，findings 0，95项定向检查通过；M7及发布门保持阻塞。 [UI复审R2](status/review/8b66add8-79df-4310-9cea-02a428928dc7.md)仍需补证；UI-01–04已解决，已观察U1/U2/U3均10.0，U4未评分，无全量分；新增原生Chrome200%部分冒烟，完整原生/手机/VoiceOver门保留。 [UI复审R3](status/review/00ba6df5-d8f1-416a-9425-a14177f88a69.md)续判需补证；U1–U3均10.0、旧差距无回归；新增Safari及官方页Chrome200%部分实拍，U4完整矩阵/手机/VoiceOver仍缺，计划6/7不变。 [U4原生补证R4](status/review/1f6813ff-89b8-4b7c-bbc0-64006b30c387.md)仍需补证；Safari36图已独立核查，手机镜像接入后超时、无产品图；U4未评分，6/7保持。 [U4 iPhone补证R5](status/review/800bd582-e8db-473a-ae4b-98bd76ceb9db.md)仍需补证；已真实操作iPhone，41图经三独立评审核查，无确认新增产品差距；U4未评分，软键盘/IME、内部滚动与完整原生/VO任务仍缺，6/7保持。 [UI复审R6](status/review/f299b7af-e64e-4d94-be57-1cf5f2688605.md)续判需补证；U1–U3均10.0、UI-01–04无回归、新增0；U4因共享桌面被其他agent验收占用及资源未变无新采集，未评分，6/7保持。 [P4覆盖核查](status/review/9d78cba7-ec90-4bf9-bc5e-372602fbb9c7.md)（2026-10-20）：A0/B1/C1，C 已按用户 D-01 决定；两项由 [DOCS-SSG-P4-gaps](status/plan/bf977834-399c-4b7d-b3e3-c7724c8d7cff.md) 承接（Ready，实施 3/3；本轮代码初审另见下项）。



- [DOCS-SSG-P4-gaps](status/plan/bf977834-399c-4b7d-b3e3-c7724c8d7cff.md)：Ready，实施 3/3 完成：C2 `88566ba1`（CLI 0.1.32/R0）经 run 39388531674 上线，说明提交 `c6d49c97` 在公共 main，last-good C1 0.1.31。首次[完整计划评审](status/review/efac9408-e453-47b6-906e-daeae72c3ffb.md)的 2×P2 已处置：V-4 改以同环境的本机 C1 为基线并允许 sitemap `lastmod` 变化；M3 增加说明推送与 V-10 公共读回。[代码初审 R1](status/review/9d6d2249-b91c-4d0b-b2f7-414804385321.md)通过验收，findings 0；源与 catalogue 11 项测试及线上浏览器 4 项通过，无需整改。[PRD 覆盖复审 R2](status/review/9f3f860f-ac41-4221-9180-d6835d8f3757.md)：G-01、G-02 已解决，新增差异 0，无需新 gaps plan；U-01、U-02 仍待核实，F-01 与 verify 门禁（docs site 缺 `rg` 等三个 job 早于本计划失败）为非 PRD 事项。

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

- [DOCS-SSG开发验收决定D15](events/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735/5d6fb51a-64cf-4085-93a0-8f9b2781af97.md)：开发阶段由本项目用户确认三个可见成果；本人已回复“验收通过”，原五人统计提案仅为历史研究记录。

- [DOCS-SSG发布顺序D16](events/prd/8917bbeb-7add-4826-95ee-5cdf6bdff735/44303026-e66c-4cd8-b47d-7dde4ce62426.md)：先完成P3/P4本地能力，公开发布留P4收口。
