# M6 · 固定候选与完整出口

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T05:46:26+08:00 · 状态：进行中，冻结前消费者配对适配与安全检查已落盘；R/C及固定锁/固定C安装和最小配对门已通过，集中出口待执行。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5。呈现路径继续静态DOM/明确交互岛。

| 检查 / 覆盖 | 实际结果 | 证据 |
| --- | --- | --- |
| 安全 / 依赖 / 发布源 | Docs十二源码及调用链、CLI/主题/argv/输入守卫/手动Pages配置未确认可利用漏洞；890文本高置信凭证扫描0命中。cargo-deny检查344依赖，漏洞条目0，因3条既有unmaintained返回1，未冒作绿 | [Docs审计](m6/security-docs-r1.md)、[CLI审计与官方advisory链接](m6/security-cli-r1.md)、[扫描](m6/secret-scan-r1.json)、[原依赖输出](m6/dependency-advisories-r1.jsonl) |
| 严格R/C消费者适配 | 原path fixture会被非空expectedR拒绝，生产门保持不变。统一helper从CLI source C的真实Git资产读R，new(C)核三Git依赖和source-free root lock；仅临时消费者2份Cargo文件投影。Native、dev browser/serve、CI/features、B P2/ignored proxy接入；空R保持path/lock，默认B500语义不改 | scripts/docs-pages-prepare.py；相关support/serve/verify/xtask改动，静态AST/纯投影与4拒绝例/shell/YAML/Node/rustfmt/diff已查，真实C尚未验 |
| 当前空R开发回归 | docs_corpus6/6、CLI72通过+1ignored；components2/2（纯投影+真实完整生成链），Rust dev3/3（外部path读/删/恢复、building/content复用/失败、build-generated include/连续保存最新代）通过 | [语料](m6/adapter-corpus-empty-r1.log)、[CLI](m6/adapter-cli-empty-r1.log)、[components2](m6/adapter-components-empty-r1.log)。Root session3786已正常完成，全部自有测试进程退出；资源仍Root独占 |
| 实际桌面供给 | 原生Chrome155.0.8059.40/Safari27.0.1已从应用清单读到。Safari原生MCP initialize/tools-list通过，并创建/关闭唯一自有about:blank tab；未读现有用户tabs/内容、未改设置。仅供给证明，不计UI通过 | [本机清单](m6/environment-r1.json)、[MCP能力](m6/safari-mcp-capabilities-r1.json)、[自有空白页](m6/safari-native-availability-r1.json) |

候选准备保留真实Git C的detached worktree历史；仅官方site/Cargo.toml三依赖行及配对lock不同，其余tracked源逐文件核hash。Pages workflow只workflow_dispatch完整C、Mac固定Git安装/同run完整artifact和依赖deploy；enablement=false，无push自动部署。尚未执行Pages/GitHub外部写。

本机已存在Python3.12.13（/opt/homebrew/bin/python3.12）；默认python3为3.9.6。仅后续验证进程PATH优先既有3.12，CI显式setup-python3.12，未安装工具或修改系统。公共CLI作者不依赖该仓库测试工具。

冻结前源门：CLI/xtask all-targets严格clippy、root fmt、sources verify、CSS no drift（28814bytes/3783 candidates）、catalog24行no drift全部通过，原日志见[CLI clippy](m6/adapter-cli-clippy-r1.log)、[xtask clippy](m6/adapter-xtask-clippy-r1.log)、[fmt](m6/adapter-root-fmt-r1.log)、[sources](m6/freeze-sources-r1.log)、[CSS](m6/freeze-css-r1.log)、[catalog](m6/freeze-catalog-r1.log)。现有六份用户P1-gaps/PRD文件指纹与冻结前清单一致，未纳入本次暂存范围。

未完成：按计划先提交库R，再仅以CLI发布身份/版本/模板锁/文档形成C，并用真实固定C禁网配对检查/未改模板/官方和B消费者验证。随后同候选V-5/V-9连贯UAT、V-10/V-11/V-12/V-14集中出口与最大页五次正式测量。未有公开候选或推送/发布授权；用户无法安排5位首次用户及实体手机，继续保留M6未验收缺口。Firefox按已确认范围留P4。

库R冻结提交：`676a55771ab77faf8c476a836025d62905a2d3e2`，覆盖M3–M5实现/验收及M6消费者准备；原M1/M2提交归属保留。仅本地提交，六份用户既有状态文件未改/未纳入。阶段提交记录见[来源清单](m6/library-r-identity.json)。当前Cargo/build/browser/ports由hydration_harness独占，Root只维护计划与状态；下一步接收C0.1.11、公共GitHub URL/完整R锁、实际locked offline两目标结果，Root审查允许差异后提交C。

C预提交准备已验：CLI0.1.11固定R676a5577；真实193包公共URL lock为48590bytes/SHA0dd43b63c0b2ed89d0dffbd732ee57e46caeb2669c1549f214c7a33bc8dde24a，6个框架包同GitHub+完整R、bindgen0.2.128、无Makepad。两目标完整locked offline metadata及docs-host/docs-wasm check均0，锁不变；实际new的docs-site/M6 named root均11文件，后一锁仅替换一个source-free根包。原证据：[handoff](m6/pair-preparation-handoff-r1.json)、[锁audit](m6/pair-preparation-lock-audit-r1.json)、[离线checks](m6/pair-preparation-offline-checks-r1.json)、[命名锁](m6/pair-preparation-actual-new-r1.json)。模板锁新生成仅manyhow/manyhow-macros0.14.0→0.14.1两patch，无Major变更；root lock仅CLI版本改动。生成/fetch仅进程Git insteadOf替换读取transport，产品manifest/lock仍公共URL，未改global git config。预提交binary source为计划提交1b4b7e6，不冒作最终C，必须提交后固定Git C重装。

Root已接受资源交还，所有准备进程退出。允许C产品diff仅五文件：CLI Cargo.toml/framework-revision.txt/components-Cargo.lock、Root Cargo.lock、docs/docs-ssg.md；Root修正文案为由installed --build-info报告自身C，避免提交后仍写CLI commit pending。对R全部SDK/docs库/CLI功能src/模板源码/source README/site的diff0；真实C提交后再做固定安装/配对全门。

真实CLI C提交：`8e86626fce74ffc45085da6377ca9aa64312fe59`（0.1.11），R固定`676a55771ab77faf8c476a836025d62905a2d3e2`。C这一次提交恰好五个允许产品文件，库/运行/CLI功能src/模板源均未改；两者之间仅有计划记录提交1b4b7e6。当前固定C Git安装使用公共URL及真实完整C、仅进程transport读取本仓，不修改已冻产品，仍不计作GitHub公开可获取安装。执行由hydration_harness独占，结果尚未预填；Root待交还后继续集中出口。

固定C安装及最小配对门已接受：[handoff](m6/installed-c-handoff-r1.json)，来源/二进制/env与消费者路径保存在本机target/docs-ssg/m6-installed-c.json（后续必须使用其installed binary，不使用准备版或HEAD）。真实Git安装保持public URL与完整C，进程local read remap仍不作为GitHub公开安装证明。installed binary SHA1a31f4c5903ef3f0a3bf0b621f2b095f4774db5d2f7e55c67b40ed4c1b2234a8，0.1.11/panicunwind/source严格完整C。

| 固定C检查 | 实际结果 | 原证据 |
| --- | --- | --- |
| 两root未改模板/README与锁 | 各11文件，README精确C/R，new锁原字节或仅source-free root名变；两mode完整offline metadata/真实CLI check-build0，template3页16文件、3既有warnings，全部11输入/锁不变 | [身份](m6/installed-c-identity-r1.json)、[new](m6/installed-c-new-roots-r1.json)、[check](m6/installed-c-template-check-r1.json)、[build](m6/installed-c-template-build-r1.json)、[输入](m6/installed-c-template-inputs-r1.json) |
| 原path反例/旧产物 | exit1/rust-framework-pair，38个旧真实产品文件字节与源码未变，无helper cache/bindgen输出；Cargo metadata/作者目标构建调用仍如原审计所列，不说未调Cargo | [原失败](m6/installed-c-path-rejection-r1.json)、[不变式与实际compiler调用](m6/installed-c-path-rejection-audit-r1.json) |
| 真实C官方消费者 | detached C保留history，严格仅site/Cargo.toml+lock投影，11397 protected tracked文件原SHA保持；实际official check/build37页0错误/12 missing-description warnings，完整源/锁再次核不变 | [check](m6/installed-c-official-check-r1.json)、[build](m6/installed-c-official-build-r1.json)、[完整命令](m6/installed-c-official-commands-r1.json) |
| 隔离目标图 | template与official各SSR188/Hydrate186，CLI310；normal+build/all-target图验证无GPU/Makepad及按mode相反/私有pipeline依赖，原锁不变 | [真实图](m6/installed-c-graphs-r1.json) |
| 未改模板功能 | 原19+14 nodes/ID全部保留，2/7分别经真实输入变4/31，Function/Full file源码原字节，console/CSP/失败请求0 | [DOM/browser](m6/installed-c-template-browser-r1.json)。该功能probe与独立official build尾段有重叠，按报告节奏更正仅作功能证据，不作严格串行正式性能；未新增视觉批 |

自有准备/安装/目标图/浏览器进程均已退出，资源归Root；未知旧97047保留。下一步同installed C集中V-5/V-9连续UAT/V-10/V-11/V-12/V-14、原生Chrome/Safari与实际A/B最大页五样本，源/CLI版本/lock/模板/README不再改变。官方A正式测量runner已写在[m6 runner](m6/measure-official-islands.mjs)，静态24/24覆盖通过，尚未实际运行/预算通过。first-user runbook已填真实C/R，生成README最终摘要/公开渠道及真人仍待补齐；用户无法安排两手机/5用户，缺口不算通过。无推送/Pages/线上或M6完成声明。

### 正式岛测量检查点

本次仍为固定R676a5577/C8e86626f，installed binary不变，DOM静态页与明确交互岛路径；Root串行拥有Cargo/browser/ports。以下更新接续上文历史准备记录，不改变候选或M6进行中状态。

| 检查 / 覆盖 | 实际结果 | 证据 |
| --- | --- | --- |
| A官方24类 / 五次冷缓存实际输入 | 23类原r1各5/5；Radio原两轮验收选择器误用native value/label htmlFor，产品为包裹label并在RadioItem上存data-value。仅修验证选择器、保持native Space和状态/disabled断言，r3为5/5；同环境同产品，其余探针与预算不变，全部24类中位≤226.2ms、原始节点保留/水合位移0 | [合并依据/全24结果](m6/formal-island-timing-summary-r1.json)、[原24类含失败](m6/formal-a-islands-r1.json)、[Radio第二轮失败](m6/formal-a-radio-r2.json)、[Radio修正五次](m6/formal-a-radio-r3.json) |
| B P2增量 / 最大gzip与raw演示页 | 原P1 profile未变；512页check/build均0错误/1warning，含首次Rust release编译96397ms，不当作20s纯内容门。p2/vellum与p2/quickstart各5/5，中位352.3/356.2ms，节点保留/水合位移0；资源闭包完整选择依据待核 | [B输入/候选](m6/formal-b-candidates-r1.json)、[check](m6/formal-b-check-r1.json)、[build](m6/formal-b-build-r1.json)、[实际五次](m6/formal-b-islands-r1.json) |
| P1 B500纯内容构建回归 | 使用同installed C，500页/511files，全部输入不变、5次完整生成均0，中位2288.7ms≤20s；不含Rust消费或并发浏览器计时 | [五次原命令与结果](m6/formal-b-content-build-r1.json) |

未完成：资源最大页清单/内容LCP-CLS-0wasm-100KB、内容保存与new、dev编译完成到新画面、同一工程连续UAT、components/official三入口集中矩阵和原生桌面/受影响回归。用户已确认5位真人与两类实体手机无法安排，保留验收缺口；未推送/公开GitHub安装/Pages上线，不计M6完成。
