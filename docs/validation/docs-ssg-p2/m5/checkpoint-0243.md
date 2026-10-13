# M5 · 英文官方站

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T02:43:16+08:00 · 状态：进行中，官方37页两目标check通过，实际dev/浏览器待验。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5；未到冻结提交点。CLI因本轮二进制变化升至0.1.8，M4临时Git C0.1.7原证据保持。

- vellum 唯一源已译英；44 个原 Markdown 链接目标、两段 fenced 命令逐字保留，无中文剩余，所有原显式数字 token 均保留。原文恢复副本在本机 target/docs-ssg/m5-vellum-original.md。核查结果见 [translation.json](m5/vellum-translation.json)；这是翻译原记录，不重新声称其行为验收通过。
- README 介绍的“Start here”改为明确的本文件 Quick start 链接，SDK快速开始另保留；utility 的 protocol.rs:96 无效链接改为实际 `Limits` 声明的 protocol.rs#L112。历史R5 source-snapshot保留当时原文，不把旧快照当现行指南。
- 文档检查脚本exit1，21个条目来自保留的历史test-results引用当前不可见，以及utility的既有示例文件路径；未删除历史来源或造新文件来通过。44个vellum Markdown目标实际均存在，数字/代码/链接检查及diff检查通过。[原检查日志](m5/doc-check-r1.log)。

官方站源已新增：根rustify-docs.toml声明12源白名单和Guides12/Components24的导航；site独立Cargo工作区沿用两目标/四档位，注册静态首页与24分类，分类内容从CATALOG读取。24演示函数分在site/src/demos五个职责模块，注册使用直接模块路径以保持源码可寻址。首页通过新接的RenderContext::markdown_intro读取README原文，框架提供定制首页容器与slots，能力表复用rd-table内部滚动。官方两目标check已实际通过，未完成产物构建或浏览器验收。

页面实施前已扩展[原阅读brief](../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md#p2-extension-the-english-sdk-site)，保留P1主题/系统字体/首页首屏结构，仅采用静态DOM与明确交互岛。

| 检查 / 覆盖 | 实际结果 | 命令 / 证据 |
| --- | --- | --- |
| 白名单与链接 / 原读 | 原实现1通过/13失败（未知include）；接线后14/14，含空集、先选后读、排除坏UTF8/无引用binary、定位拒绝、Exists-only正文不绑定/delete/retarget、HTML链接与引用图片 | `mbx test --locked -p rustify-docs --test content_include`；[原红](m5/include-red-r1.log)、[绿](m5/include-green-r1.log) |
| README薄桥 | 同Markdown writer/原offset，源基准链接、Unicode/CRLF/围栏H2、原始行号、重分析后原读守卫、嵌套图片进入资源图及private拒绝；helper只发路径/placeholder；6/6 | `mbx test --locked -p rustify-docs --test markdown_intro`；[6例](m5/intro-tests-r1.log)。初接误用Fragment字段的编译失败保留在intro-include-native-r1.log，修复后r2库190/白名单14通过 |
| Docs完整回归 | 190库 + 14白名单 + 16生成 + 6README + 12注册生成 + 1metadata + 19registry = 258通过 | `mbx test --locked -p rustify-docs`；[全套](m5/docs-full-r1.log)。新pipeline测试登记required-features，避免进helper-only图 |
| dev过滤单元 | CLI72通过/1ignored；精确byte/Exists角色、validation/binary/body忽略、删除、symlink/目录替换及缺失恢复。原71/1失败来自测试替换页无描述，补合法段落后通过 | `mbx test --locked -p rustify-docs-cli --bin rustify-docs`；[原失败](m5/cli-unit-r2.log)、[72通过](m5/cli-unit-r3.log)。r1误选不存在lib target原日志保留；实际notify事件/浏览器仍待验 |
| 严格CLI与二进制 | all-target Clippy通过；0.1.8真实metadata刷新lock，docs-cli构建通过 | [严格Clippy](m5/cli-clippy-r2.log)、[锁刷新](m5/cli-lock-r1.log)、[0.1.8构建](m5/cli-build-r2.log)。此前0.1.7中间候选日志保留，不作最终版本 |
| 官方37页check | 12指南 + 24分类 + 首页，host/docs-host与wasm/docs-wasm均成功，0错误/14警告；12条missing-description沿用writer首段，2条既有SDK dead-code提示，未改无关代码消警告 | `RUSTIFY_DOCS_CARGO=mbx target/docs-cli/rustify-docs check --config rustify-docs.toml --format json`；[实际37页](m5/official-check-r2.json)，为升级前0.1.7中间候选，最终0.1.8须复核 |

首次官方命令误将--config放在命令前，CLI报unknown command，原stderr见 [r1](m5/official-check-r1.log)。当前计划V-10已改为实际command-first顺序；同错只出现在旧计划指纹快照，保留历史原文而不重写证据。M4完成时原计划完整版本保存在 [0159快照](m5/plan-checkpoint-0159.md)。

CI verify.yml已接官方独立manifest的fmt/SSR clippy/hydrate check/CLI check-build；Playwright已声明official-root/subpath/meta并用现serve.sh直接从实际源构建，仅临时改本地主机base。这些入口尚未跑，不称远端CI或浏览器通过。

未完成：0.1.8官方最终check/build、实际notify/dev同源README/指南修改与排除事件、独立site目标lint/feature图与相关安全检查、root/subpath/meta真实浏览器及设计收口。未生成官方交付产物，未公开候选或部署；M5不记完成，M6真人/实体手机缺口继续保留。
