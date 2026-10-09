# Leptos 0.8.22 升级验证

## 范围与来源

2026-10-09，用户要求比较最新版 Leptos 与当前使用版本并升级。以最新稳定发布为目标：crates.io API 返回 `max_stable_version: 0.8.22`，未撤回，MSRV 为 Rust 1.88；0.9.0-beta2 仍为预发布。本仓 Rust 1.98.1 无需调整。

官方来源：[crates.io 元数据](https://crates.io/api/v1/crates/leptos)、[0.8.21 发布说明](https://github.com/leptos-rs/leptos/releases/tag/v0.8.21)、[0.8.22 发布说明](https://github.com/leptos-rs/leptos/releases/tag/v0.8.22)、[宏依赖迁移 PR](https://github.com/leptos-rs/leptos/pull/4892)。

| 项目 | 原版本 | 本次版本 |
| --- | --- | --- |
| leptos | 0.8.20 | 0.8.22 |
| leptos_macro | 0.8.17 | 0.8.19 |
| reactive_graph | 0.2.14 | 0.2.15 |
| tachys | 0.2.18 | 0.2.19 |
| rstml | 0.12.1 | 0.13.1 |
| server_fn_macro_default | 0.8.5 | 0.8.6 |

发布说明覆盖 Suspense 并发、LocalResource 错误通知、Signal/Memo 属性读取、For/hydration、nonce 和宏依赖维护。0.8.22 的 rstml 更新移除旧 `syn_derive` → `proc-macro-error2` 链；上游 PR 将相关公告定性为依赖不再维护，不将它扩写为已发现产品漏洞。发布说明中的 “CRITICAL USABILITY IMPROVEMENT” 实际是组件命名术语从 camelCase 修正为 PascalCase（[PR #4901](https://github.com/leptos-rs/leptos/pull/4901)）。

本仓使用 Leptos CSR；升级保留 `csr` feature 和现有 API。应用不运行 SSR/hydration，相关修复不作为本仓新增功能。直接对比本机 crates.io 源码，`leptos/src/mount.rs` 在 0.8.20 与 0.8.22 间一致，Suspense、nonce 及异步派生实现有变化；最终兼容性以本轮编译和浏览器结果为准。

## 改动

根工作区和独立 `tests/theme-export/runtime` 都锁定 `=0.8.22`，各自更新 Cargo.lock。同步 xtask doctor 的版本检查、当前 `docs/compatibility.md` 版本表和 sources.lock.json 的当前版本/校验和。来源记录中的 `reference_tree_diff` 是 2026-09-08 对 0.8.20 的历史比较，保留原文；P1 历史报告和旧验证日志亦保留。未修改组件实现、Makepad、工具链或 wasm-bindgen。

开始时工作区已有大量未提交改动，本次在这些内容上增量调整；上述验证执行时尚未提交或推送。

## 升级中的兼容性问题

只运行 `mbx update -p leptos --precise 0.8.22` 会保留 `server_fn_macro_default 0.8.5`。它通过 Syn 2 的 `parse_quote!` 创建 Path，再传入 `server_fn_macro 0.8.12` 使用 Syn 3 的接口；编译报 E0277。工作区单测和 build-web 均复现，见 [首次编译日志](initial-compile-failure.log)。

根工作区与独立主题导出工作区分别执行 `mbx update -p server_fn_macro_default --precise 0.8.6`。已核对 0.8.6 的 Cargo.toml 使用 Syn 3，后续重新编译验证。无需补丁 crates.io 源码或增加直接依赖。

## 本轮验证

本轮基线：`main / 53652fcc405a2679df19eb10e252efd4a4236f50 / dirty`。使用仓库 mise 工具链与 mbx；浏览器为本机 Google Chrome 154.0.8037.98，通过 Playwright 1.63.0 执行。浏览器运行各自 `build-web --release` 生成的新产物，采用真实 xtask preview 入口；本项目是静态 SDK 示例，非 Portal App。

| 检查 | 命令/范围 | 结果与证据 |
| --- | --- | --- |
| 格式 | `cargo fmt --all -- --check` | 通过，[日志](format.log) |
| 工作区单测 | `mbx test --workspace --lib --bins --locked` | 1080 通过，0 失败/忽略，[日志](host-tests.log) |
| 独立主题导出测试 | `mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins` | 45 通过，[日志](export-tests.log) |
| Clippy | `mbx clippy --workspace --all-targets --locked -- -D warnings` | 通过，[日志](clippy.log) |
| 工具链/版本诊断 | `mbx xtask doctor` | 全部通过，Leptos 0.8.22，[日志](doctor.log) |
| 来源 | `mbx xtask sources verify` | 通过；当前 Leptos 来源校验和另核对 Cargo.lock 一致，[日志](sources.log) |
| CSS / 组件目录 | `mbx xtask css --check`；`mbx xtask catalog --write docs/components.md --check` | 通过，[CSS](css.log)、[目录](catalog-check.log) |
| WASM 发布构建 | `mbx xtask build-web --example NAME --release` | 七个示例全部通过，见下表 |
| 独立消费者构建 | `sh tests/theme-export/build.sh` | 通过，[日志](export-build.log) |
| fusion-basic 浏览器 | m2-runtime、m3-state | 14 通过；挂载、销毁、GPU 失败、宿主共存与 DOM/GPU 状态，[结果](browser-fusion.json) |
| component-catalog 浏览器 | p2-catalog、catalog-inputs | 41 通过；全部分类、输入草稿、键盘、主题、语言、只读/禁用、重置，[结果](browser-catalog.json) |
| property-workbench 浏览器 | m3-workbench | 21 通过；属性编辑、选择、重排、删除、销毁/fatal 边界，[结果](browser-workbench.json) |
| data-workbench 浏览器 | p3-table | 20 通过；表格窗口、编辑身份、过滤、滚动、删除、ARIA 与销毁，[结果](browser-data.json) |
| theme-studio 浏览器 | theme-editor | 13 通过；主题编辑、草稿、双模式、撤销重做、Inspector 与布局，[结果](browser-theme.json) |
| utility-lab 浏览器 | utility-runtime | 27 通过；合并、observer、CSS 安装、scope、配置替换、Worker 与清理，[结果](browser-utility.json) |

以上 136 项浏览器回归均为 0 失败、0 跳过、0 flaky。实际浏览器命令均带 `RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_XTASK=target/debug/xtask`，通过 `npx playwright test --project=PROJECT SPEC --workers=1 --output=test-results/leptos-0.8.22-NAME` 逐项目执行；具体步骤、断言和耗时见各 JSON 与同名 `.log`。

| 示例 | build_id | 构建证据 |
| --- | --- | --- |
| fusion-basic | e5be99ea4c6d4339 | [日志](build-fusion-basic.log) |
| component-catalog | 581287750b1a07db | [日志](build-component-catalog.log) |
| property-workbench | 0dfffbc68e25311e | [日志](build-property-workbench.log) |
| data-workbench | 8d84a260f5529ca4 | [日志](build-data-workbench.log) |
| vellum | b053c02502959e1b | [日志](build-vellum.log) |
| theme-studio | c9c9cb2a11f33ec1 | [日志](build-theme-studio.log) |
| utility-lab | 79e53adbdde257a3 | [日志](build-utility-lab.log) |

七个示例启动冒烟全部通过：每个 preview 由 xtask 自动分配空闲端口，页面状态达到 ready，记录 build_id 并截图；7/7 无未归类的 console error、pageerror、失败请求或 HTTP >= 400。见 [结果](browser-smoke.json)、[日志](browser-smoke.log) 与 [可复现脚本](browser-smoke.cjs)（从仓根执行 `node docs/validation/leptos-0.8.22/browser-smoke.cjs`）。已查看 Theme Studio、Utility Lab 与 Vellum 截图，页面和 DOM/GPU 内容已呈现。

截图：[fusion-basic](fusion-basic.png)、[component-catalog](component-catalog.png)、[property-workbench](property-workbench.png)、[data-workbench](data-workbench.png)、[vellum](vellum.png)、[theme-studio](theme-studio.png)、[utility-lab](utility-lab.png)。

首次严格 console 检查定位到 `/favicon.ico` 的 404（[原始输出](browser-smoke-initial.log)）；fusion-basic 的既有 index.html 无 favicon 声明，本次未更改该文件。最终冒烟仅将该 favicon 404 单列为 `knownConsole`，其他 console error、pageerror、失败请求及 HTTP >= 400 均作为失败。

## 验证边界

本次为补丁依赖升级；未修改页面设计或组件实现。未运行全仓全部浏览器用例、SSR/hydration 测试、非 Chrome 浏览器、性能 evidence tier、远端 CI 或发布。Vellum 不扩写为全部编辑流程已复验；独立主题导出本轮包含测试/构建，不扩写为它的所有浏览器路径已复验。原有计划和历史验收结论未替换。

本轮对原有 dirty 内容的增量差异见 [upgrade.diff](upgrade.diff)。

## 提交前范围与安全核对

用户随后要求 commit。通过升级前工作区快照提取本次差异，仅暂存七个升级文件与本目录证据；已有 Tailwind、组件、CI、工具链和 `.xgent-ai` 工作资料均保留本地。验证记录包含该 dirty 工作区的结果，不能视为只含升级的独立 checkout 已重跑全部测试。

| 级别 | 本次引入的已确认发现数 |
| --- | --- |
| Critical / High / Medium / Low / Info | 各 0 |

使用 `security-audit` 对本次暂存差异及本目录文本/截图核对：未发现凭证或私钥；诊断逻辑仅改变版本常量，未改变授权、网络、CSP 或生产输入处理。浏览器证据脚本使用固定示例数组、固定本地路径和不经 shell 的 spawn 参数；页面数据仅收集至本地证据文件。本次依赖仍来自 crates.io，校验和已核对。此处未发现本次变化引入的漏洞；`cargo-audit` 未安装，未执行完整 RustSec 自动审计，不声称所有依赖不存在已知漏洞。

已将暂存树导出到独立临时目录核对依赖解析。根工作区初次 `metadata --locked --offline` 发现残留的 `yansi 1.0.1`；离线解析仅移除这个当前提交图不再使用的包，随后更新暂存 Cargo.lock，未覆盖工作区 Cargo.lock。暂存树保留 HEAD 的 `tw_merge` 及原有 workspace 成员，不含尚未提交的 rustify-style/utility-lab。独立主题导出工作区亦按暂存内容核对。

最终两个暂存工作区的 `mbx metadata --locked --offline --format-version 1` 均退出 0，锁文件版本一致，见 [暂存锁文件校验](staged-lock-check.json)。未在隔离暂存树重跑完整单测或浏览器。
