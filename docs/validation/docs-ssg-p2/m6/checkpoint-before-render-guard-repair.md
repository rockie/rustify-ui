# M6 · 固定候选与完整出口

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T07:13:19+08:00 · 状态：进行中；原生Chrome发现favicon404后库R2已修复提交，CLI0.1.12 C2已本地提交，fixed安装及连续UAT通过，集中出口进行中。当前库基线：0494f304；以下首次R/C及计时为旧候选历史，不计新候选出口。呈现路径继续静态DOM/明确交互岛。

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

### 原生图标失败与第二次候选冻结

连续作者r1/r2均完成原模板、README演示/定制页/footer、坏Rust同树/源码/17文件dist保留、check/build非零及修复恢复，最终仍失败，不作UAT通过。r1缺console URL，r2完整观察定位唯一非预期错误为原生Chrome自动请求`/favicon.ico`404；预期deep404单独匹配，Page响应日志未捕捉该浏览器图标请求。[r1原报告](m6/uat-author-continuous-r1/report.json)、[r2含URL原报告](m6/uat-author-continuous-r2/report.json)保留，未放宽favicon过滤。

最小修复仅crates/rustify-docs/src/shell.rs/html_check.rs：head声明空data SVG图标，继续现有img-src data策略，generated rel=icon按image分类；外部图标及data样式表仍拒绝。两个窄回归先红，Docs191全部通过、strict clippy和root fmt通过；开发CLI生成纯内容站在原生Chrome155.0.8059.40零console/HTTP/pageerror，图标为声明data URL。证据：[icon分类红](m6/favicon-data-icon-red-r1.log)、[shell红](m6/favicon-shell-red-r1.log)、[Docs绿](m6/favicon-docs-green-r1.log)、[clippy](m6/favicon-clippy-r1.log)、[fmt最终](m6/favicon-fmt-r3.log)、[实际native](m6/favicon-native-development-r1.json)。该development CLI只证明修复，不冒作新固定C的组件验收。

真实新库R2=`0494f304f0f9f8b620affb7309df23db3cb2377c`，只提交两必要产品文件，六用户既有dirty指纹不变。CLI将patch到0.1.12并配对R2；hydration_harness当前独占Cargo/build/browser/ports，准备允许五文件及193包锁（不升级第三方），Root审查后提交新C并固定Git安装。C1/R1正式岛/内容结果和资源闭包均保留为历史，HTML与候选已变，受影响门须按新C重跑。

C1内容历史五样本：A首页/最大raw quickstart/最大gzip vellum/404，LCP720/1076/884/716ms、CLS0、0wasm/foreign/CSP；B最大gzip中英文LCP796/820ms，其余最大raw/home/404尚未测，不扩大覆盖。[A原测量](m6/formal-a-content-load-r1.json)、[B原测量](m6/formal-b-content-load-r1.json)。最新Chrome稳定补丁155.0.8059.79/.80（2026-10-13官方发布）高于本机.40，保留最新补丁复验项，不自动更新系统；[官方对照](m6/native-release-status-r1.json)。真机/真人及公开渠道/Pages缺口仍保留，M6不计完成。

第二次真实CLI C2冻结：`c1061ced3deaa4e6669bc45eb3382306fa85c958`（0.1.12）配对库R2`0494f304f0f9f8b620affb7309df23db3cb2377c`。本次恰好五允许产品文件；193包锁仅六框架Git source改为完整R2，全部第三方records不变，锁SHA`c42635807affd79947f5afc581ef5a5af544d1a23bd4acc99ac25c10bde78f65`；offline SSR/hydrate完整metadata/check均0，实际new两root各11文件，source-free root名外的lock不变。[配对准备handoff](m6/pair2-preparation-handoff-r1.json)、[实际identity](m6/frozen-pair2-identity.json)。prepared binary source为R2而非真实C2，不能用于最终C2验收。hydration_harness已接续固定public URL+完整C2 Git安装门，保持仅进程local remap读取，不计public GitHub可获取；新安装/模板/官方/图结果尚未预填。旧C1证据保留，新C2固定安装后再由Root关闭受影响技术门。

### C2固定安装与连续作者路径接受

Root已接受[installed handoff](m6/pair2-installed-handoff-r1.json)：CLI0.1.12/source精确C2，binary SHA88a255eafa51c7315d49b6cdc02f7f4e137d92a8fe8a50b5741b509827a3781e。两new root各11输入/锁不变，模板offline check/build0（3页/16文件）；detached官方0错误/12既有missing-description warnings（37页/53文件），11397 protected源及投影输入全部不变。五组隔离图通过；首观察器误把既有bitflags/smallvec vendor路径当禁用库，原红保留，修正按package name验证，[图最终](m6/pair2-installed-graphs-r2.json)。未配对path runtime拒绝按原validator/source/lock无差异证据复用。canonical state现指C2，旧C1 state原位保存。进程全部交还Root。

[完整连续UAT C2](m6/uat-author-continuous-c2-r1/report.json)七阶段通过：原样模板真实输入与函数/全文件源→仅从生成README提取的新演示与围栏→注册定制页/独立counter→footer含deep404→坏Rust保留同一document/root/全部nodes/IDs/source及17个旧dist文件、check/build均非零且诊断原作者行列→修复新初值21经真实点击得22→final check/build0及lock/README/manifest不变。唯一同一dev/session与真实compiler生命周期、SSE及HTTP wasm SHA保留；console/pageerror/CSP/异常请求0，预期deep404明确归属。nativeChrome155.0.8059.40中favicon问题已修复；该UAT不替代五真人或正式性能。

C2 [生成README/help与摘要](m6/first-user-runbook.md)已更新，C1原字节保留为历史；公开渠道与真人表仍空。Safari自有tab的[evaluate/Inspector实际schema](m6/safari-owned-schema-probe-c2-r1.json)已确认且tab关闭，尚不算完整Safari验收；[新A声明资源清单](m6/formal-resource-selection-c2-a-r1.json)52文件/38页验证，B尚待新构建。

集中矩阵components-root36/36通过；reporter相对路径在验证wrapper位置解析导致汇总脚本收集失败，实际原JSON及trace已恢复，未重跑/丢样本。[恢复证据](m6/matrix-report-location-recovery-c2-r1.json)。后续wrapper只把JSON输出改为明确根路径。4200被外部Bun90039占用，未操作该服务，official-meta从OS分配验证端口，其他配置/端口沿仓入口。[端口原证据](m6/matrix-ports-preflight-c2-r1.json)。正在继续其余project，整体矩阵未预填通过。

### Chrome集中矩阵接受与Safari环境诊断

固定C2/R2原生Chrome155.0.8059.40：[351distinct汇总](m6/matrix-native-chrome-c2-accepted-r1.json)。components-root/subpath/meta各36，official三入口各46，P1 content三入口各27；docs-dev最终24：原未改Rust dev2/生成UAT1保留，P1 dev21在修正观察器后全过。旧原docs-dev15pass/9fail保留：[原24报告](m6/matrix-docs-dev-c2-r2/results.json)、[新21报告](m6/matrix-dev-p1-c2-r3/results.json)。旧P1 harness把advertised地址当首轮构建完成，过早写坏内容造成无last-good/inline region，并在building阶段断言500；另把正常导航的exact事件流ERR_ABORTED当失败。只改tests/docs-ssg/dev.spec.ts等待实际SSE revision>0且building=false（初始错误也可终态）及精确事件流关闭归属；268原断言表达式未改，无固定sleep、产品未改，不需要新R/C。

[Safari header首轮38页](m6/native-safari-header-c2-r1.json)全部原件保留，0/38，不计验收。14静态页误将exact声明data图标当foreign网络，24岛来自真实mode30s startup deadline，HTTP wasm200/applicationwasm。单页[真实启动诊断](m6/safari-startup-diagnostic-c2-r1.json)12 snapshots显示hidden/无focus、rAF始终1，wasm14ms完成且mounted seed已在，但双rAF不继续；单纯Safari activate无效。[定向owned URL前台窗口尝试](m6/safari-owned-foreground-c2-r1.json)已选择自有URL并恢复窗口状态，之后仍hidden/loading；没有读出用户其他tabs内容，没有改rAF、visibility或产品期限。SafariWebDriver既有binary的[原能力probe](m6/safari-webdriver-capability-c2-r1.json)解析过早，后以[raw HTTP确认](m6/safari-webdriver-capability-c2-r2.json)status ready=true，但创建own session10s超时；未执行--enable/修改设置，不声称已建会话或原生交互通过。MCP自有tabs全部关闭。

首轮原生Safari的普通首页链接/light主题/375px无overflow/原生details Enter小项有实际通过证据，但不覆盖整体；dark grid因岛未attach仍浅色，320请求实测336（窗口约束），均保留未通过。meta入口未运行。验证runner已严格只单列exact声明+head href+200/imageSVG的inline图标，并在hidden时直接拒绝环境、逐页保存/首败停止；原HTTP(s)同源规则及30s产品期限保持。已异步询问用户可见Safari/最新Chrome155.0.8059.79/.80环境渠道，待答；没有把既有手机/真人无法安排决定扩展为桌面豁免。

当前所有Cargo/build/browser/ports已独占交hydration_harness执行V14源码/分包/features/artifacts/reliability第一批，Root仅维护记录，不启动runtime。C2正式A/B、dev/content性能和旧DOM/GPU示例回归尚未执行，M6仍进行中/5of7；公开Git可获取、Pages、真人/真机及原生桌面完整门均未关闭。
