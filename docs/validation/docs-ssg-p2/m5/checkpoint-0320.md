# M5 · 英文官方站

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T03:20:59+08:00 · 状态：进行中，三官方浏览器各47通过，真实dev11阶段通过；设计确认/目标图/B演示语料待收口。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5；未到冻结提交点。CLI经本轮默认scope样式修复升至0.1.9，M4临时Git C0.1.7及M5中间0.1.8证据原样保留。

- vellum 唯一源已译英；44 个原 Markdown 链接目标、两段 fenced 命令逐字保留，无中文剩余，所有原显式数字 token 均保留。原文恢复副本在本机 target/docs-ssg/m5-vellum-original.md。核查结果见 [translation.json](m5/vellum-translation.json)；这是翻译原记录，不重新声称其行为验收通过。
- README 介绍的“Start here”改为明确的本文件 Quick start 链接，SDK快速开始另保留；utility 的 protocol.rs:96 无效链接改为实际 `Limits` 声明的 protocol.rs#L112。历史R5 source-snapshot保留当时原文，不把旧快照当现行指南。
- 文档检查脚本exit1，21个条目来自保留的历史test-results引用当前不可见，以及utility的既有示例文件路径；未删除历史来源或造新文件来通过。44个vellum Markdown目标实际均存在，数字/代码/链接检查及diff检查通过。[原检查日志](m5/doc-check-r1.log)。

官方站源已新增：根rustify-docs.toml声明12源白名单和Guides12/Components24的导航；site独立Cargo工作区沿用两目标/四档位，注册静态首页与24分类，分类内容从CATALOG读取。24演示函数分在site/src/demos五个职责模块，注册使用直接模块路径以保持源码可寻址。首页通过RenderContext::markdown_intro读取README原文，框架提供定制首页容器与slots，能力表复用rd-table内部滚动。已真实构建与三组浏览器验收；目标图、最终版本复核及B语料增量仍待完成。

页面实施前已扩展[原阅读brief](../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md#p2-extension-the-english-sdk-site)，保留P1主题/系统字体/首页首屏结构，仅采用静态DOM与明确交互岛。

| 检查 / 覆盖 | 实际结果 | 命令 / 证据 |
| --- | --- | --- |
| 白名单与链接 / 原读 | 原实现1通过/13失败（未知include）；接线后14/14，含空集、先选后读、排除坏UTF8/无引用binary、定位拒绝、Exists-only正文不绑定/delete/retarget、HTML链接与引用图片 | `mbx test --locked -p rustify-docs --test content_include`；[原红](m5/include-red-r1.log)、[绿](m5/include-green-r1.log) |
| README薄桥 | 同Markdown writer/原offset，源基准链接、Unicode/CRLF/围栏H2、原始行号、重分析后原读守卫、嵌套图片进入资源图及private拒绝；helper只发路径/placeholder；6/6 | `mbx test --locked -p rustify-docs --test markdown_intro`；[6例](m5/intro-tests-r1.log)。初接误用Fragment字段的编译失败保留在intro-include-native-r1.log，修复后r2库190/白名单14通过 |
| Docs完整回归 | 190库 + 14白名单 + 16生成 + 6README + 12注册生成 + 1metadata + 19registry = 258通过 | `mbx test --locked -p rustify-docs`；[全套](m5/docs-full-r1.log)。新pipeline测试登记required-features，避免进helper-only图 |
| dev过滤单元 | CLI72通过/1ignored；精确byte/Exists角色、validation/binary/body忽略、删除、symlink/目录替换及缺失恢复。原71/1失败来自测试替换页无描述，补合法段落后通过 | `mbx test --locked -p rustify-docs-cli --bin rustify-docs`；[原失败](m5/cli-unit-r2.log)、[72通过](m5/cli-unit-r3.log)。r1误选不存在lib target原日志保留；实际notify事件/浏览器仍待验 |
| 严格CLI与二进制 | all-target Clippy通过；0.1.8真实metadata刷新lock，docs-cli构建通过 | [严格Clippy](m5/cli-clippy-r2.log)、[锁刷新](m5/cli-lock-r1.log)、[0.1.8构建](m5/cli-build-r2.log)。此前0.1.7中间候选日志保留，不作最终版本 |
| 官方37页check | 12指南 + 24分类 + 首页，host/docs-host与wasm/docs-wasm均成功，0错误/14警告；12条missing-description沿用writer首段，2条既有SDK dead-code提示，未改无关代码消警告 | `RUSTIFY_DOCS_CARGO=mbx target/docs-cli/rustify-docs check --config rustify-docs.toml --format json`；[实际37页](m5/official-check-r2.json)，为升级前0.1.7中间候选，最终0.1.8须复核 |
| 原源码静态产物 | 0.1.8 release构建37页54文件0错误，build_id c24304a030eb28b1；静态摘要核查53文件及38 HTML含404、en/单H1/main、仅24分类模块、无md/rs/toml/lock公开。空.gitkeep骨架残留已授权删除，修Link后r2产物为37页53文件/build_id65735c210d9158f3 | [首构建](m5/official-build-r1.json)、[静态audit](m5/artifact-audit-r1.json)、[移除单个自己占位文件](m5/static-placeholder-removal-r1.json)；最终0.1.9身份另见下行 |
| 真正Root dev同源与白名单事件 | 11阶段通过：实际README/quickstart HTML更新，排除MD/binary/validation与Exists-only任意正文不生新状态，delete/outside-retarget拒绝且完整旧HTTP HTML不变，restore/safe-retarget恢复；暖后0新增Cargo/epoch不变，原已交互Button节点/值1/wasm身份保留；源码finally逐字节恢复、探针删除、console/CSP/请求错误0 | [真实dev报告](m5/official-dev-r1/report.json)、[状态](m5/official-dev-r1/states.json)、[调用](m5/official-dev-r1/cargo-calls.jsonl)。0.1.9同路复验在进行 |
| 三种真实官方浏览器入口 | Chromium153.0.8010.12，0.1.9 fixed binary：subpath r5、root r2、meta r2各47/47（141总例），含12静态guides/首页零wasm、CATALOG24真实操作/精确source、36导航/deep404/no-script/主题/320与375及明确200%emulation；meta wasm200/application/wasm且无header CSP、有meta策略；新增dark/system严格grid颜色回归通过 | [subpath r5](m5/official-subpath-r5/results.json)、[root r2](m5/official-root-r2/results.json)、[meta r2](m5/official-meta-r2/results.json)。这是本地Chromium和模拟视口/缩放，不计Safari/真机/真人/线上 |

实际产品修复与原红：Link演示原生锚点被guest router拦截，r1/r2失败及[事件堆栈](m5/link-navigation-probe-r1/probe.json)确认；仅作者Link加target=_self明确绕过局部router，原精确hash/disabled/source断言保留并通过。ToggleGroup顺序、主题first-frame初始化和disabled anchor角色的测试假设按真实SDK行为修正，未放宽实际结果。

首批[设计review](m5/finish-review.md)为fix（M5-F1）：default island scope的light fallback覆盖dark文档背景、文字继承亮色，实际[computed/matched](m5/dark-table-probe-r2/computed-and-matched.json)对比度1。三theme case补严格CSS后旧候选light1绿/dark+system2红，[原红r4](m5/official-subpath-r4/results.json)保留。Docs默认tokens对descendant scopes改为继承文档属性，SDK其它app语义不改，导入projection后置且selectors匹配；CLI0.1.9真实构建/版本与[样式9例](m5/default-scope-style-r1.log)通过。新三项目严格回归均绿，唯一九图确认包在[本批r5](m5/official-subpath-r5/capture-batch/manifest.json)，独立finish确认尚待返回；不把首批47功能绿当视觉通过。

首次官方命令误将--config放在命令前，CLI报unknown command，原stderr见 [r1](m5/official-check-r1.log)。当前计划V-10已改为实际command-first顺序；同错只出现在旧计划指纹快照，保留历史原文而不重写证据。M4完成时原计划完整版本保存在 [0159快照](m5/plan-checkpoint-0159.md)。

CI verify.yml已接官方独立manifest的fmt/SSR clippy/hydrate check/CLI check-build；Playwright official-root/subpath/meta与serve.sh从实际源构建，仅临时改本地主机base，本地已实际跑通。远端CI未触发，独立目标图与SSR/hydrate门仍在本地收口。

未完成：0.1.9最终offline产物/check及dev复验、导入主题受影响case、独立目标lint/feature图与安全检查、finish确认与设计规范同步；计划M5的B演示语料/docs_corpus增量开始实现，保留P1默认500页/profile字节与预算。尚未公开冻结候选或部署；M5不记完成，M6真人/实体手机缺口继续保留。
