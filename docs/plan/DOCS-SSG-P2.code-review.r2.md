# DOCS-SSG-P2 · 代码评审 R2（修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 | [DOCS-SSG-P2](DOCS-SSG-P2.md)，v2，sha256 `0e2b26d03aa30b2a8cc8bf6baa80eb21b0d043a281e8953e4a513ba3c29b921f` | 修复差异 `53d5c48563420c9252cac3f9d1e14647494320a3..d2f20b03cf58c1d1ced3785397c501d412cd4167` 的净效果（代码提交 `7930fd9a`、`45878ce3`、`e8bf97b4`；`d2f20b03` 只改计划），另单列当前未提交内容，指纹见下文 | [R1](DOCS-SSG-P2.code-review.md)，sha256 `473e6a95e16a1e1c35743d17948bf2d4eaf9aff7dad527a07b092a78fa35adfb` |

评审对象 ID：`5bf4b892-430f-4412-8878-1149268cf39c`。日期：2026-10-14T22:16+08:00。由当前主 agent 独立完成，未委派 reviewer，未修改产品源码、原计划、R1 报告或 apply 记录。

## 复审记录

- 类型：**修复效果复审**，不是对 `785f51ad..53d5c485` 的完整重审。R1 的审查基线是 `785f51ad9fdcb69e624a0c8f85d91e3541d58e90..53d5c48563420c9252cac3f9d1e14647494320a3`。本轮修复差异从 R1 的目标 `53d5c485` 算到 HEAD `d2f20b03`，核查版本是 HEAD 加当前工作树。
- 已有处置：[M1 记录](../validation/docs-ssg-p2/M1.md)「代码评审 R1 修复 · F-01」、[M4 记录](../validation/docs-ssg-p2/M4.md)「代码评审 R1 修复 · F-02 / F-03」、[apply 证据入口](../validation/docs-ssg-p2/apply-r1/README.md)。处置记录引用的 R1 报告 hash 与当前文件一致，说明 R1 报告未被改写。
- 范围：复核 R1 的全部 3 项问题（R1 没有待核实项），并检查修复差异是否引入或激活了新问题。区间内的 4 个提交都由计划完成记录登记（「修复」或「验证」），没有未登记的提交。

## 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1](DOCS-SSG-P2.code-review.md) / F-01 | P2：两个独立 `Show` 的 fallback collector 依次覆盖同一个根上的几何 token，仍存活的进度条从 128px 变成 256px | 已修复，库 R5 `7930fd9a` | **已解决** | [geometry.rs](../../crates/rustify-components/src/geometry.rs) 150–178：从当前 Owner 沿父链上溯到仍属于同一 `ScopeRoots.container()` 的最高 Owner，在那里只提供一个 `ScopeHandle`（Scope + abort 恢复）。`Rules` 268、287 只保存并使用这个 token，各自的 stylesheet 仍独立清理；228–240 只在根 Owner 结束时恢复原标记。在 HEAD 重建 harness 后 `dom-hydrate` 12/12 通过，其中新增的两种清理顺序：两个分支同时打开时第一条仍为 **128px**，第二条 64px，各自更新、先后关闭、重开时 marker 都不变；同 seed 的另一挂载根有独立 marker 并保持 128px；dispose 后 marker 移除，adoptedStyleSheets 回到基线，无 style 属性，CSP 0。原有 SSR/hydrate 节点、几何、跨实例和 trap 用例同组通过 | 无 |
| [R1](DOCS-SSG-P2.code-review.md) / F-02 | P2：metadata 只按 `ssr` 读取，hydrate 专属本地依赖的 Cargo.toml 没有参与编译指纹，dev 继续提供旧 wasm | 已修复，R5 `7930fd9a`，配对 CLI C5 `45878ce3` / 0.1.15 | **已解决** | [rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) 292–316：分别以 `ssr`+宿主 triple、`hydrate`+`wasm32-unknown-unknown`（`--filter-platform` 920/951）读取两张图；647–692 只保留从作者包可达的节点；1669、1719 把两图的本地根和包并入输入与 watch；470 的 wasm 构建改用 hydrate 图。单元 `input_layout_tracks_hydrate_only_manifest_and_target_graphs` 通过。已安装 C5 的真实 dev 中，只修改 hydrate 专属依赖的 default feature：wasm SHA `86793f9d…`→`8d109bfd…`，generation 改变，SSR 初态仍为 7，点击结果 17→27，背景色随之改变；随后的 Markdown 保存保持同一 wasm/generation，不新增编译命令 | 无 |
| [R1](DOCS-SSG-P2.code-review.md) / F-03 | P2：build-script 输入按裸包名映射根目录，同名不同版本的 path 包互相覆盖，非 Rust 输入保存后不更新 | 已修复，R5 `7930fd9a`，C5 `45878ce3` | **已解决** | [rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) 1061–1068：保存 Cargo JSON 回执，保存失败报 `rust-inputs` 错误，不再静默丢弃；1928–1975 只读取 `build-script-executed`，按完整 `package_id` 查 1734 以 id 为键的根，`output` 取自回报的 `out_dir`，不再猜测 `build/<name>-<hash>`。已核对：未重跑的 build-script 也会被回报（一份几乎全 fresh 的夹具日志里，258 个 fresh、5 个重编，仍有 44 条 `build-script-executed`），所以增量构建不会丢失输入。单元 `discover_build_inputs_binds_duplicate_names_by_package_id_and_out_dir` 通过。真实 dev 中 payload v1 7→14，删除时报 2 个错误并保留上一版（good 不变），恢复为 23；v2 19→29，删除后同样保留上一版，恢复为 37。每次保存都改变对应 SSR 初态、wasm SHA 和 generation，站点源码未改 | 无 |

旧问题解决情况：3/3 已解决，0 项判为不再适用。

## 确认问题

本轮新增：**未发现可确认的问题**。

检查过但没有列入 findings 的候选：

- **构建失败时回执不完整**：F-03 删除场景中，失败构建的 stdout 可能缺少未运行包的 `build-script-executed`。dev 的 watch 过滤只累加不收缩（[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 1135–1142、383–388）；下一次成功构建后，构建后的发现步骤（[rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) 489）会读取完整回执，失败代不会产出 `Prepared`。真实删除/恢复回归也通过，因此不构成缺陷。
- **两次 metadata 的重复诊断**：两图的配对校验文本相同，`check`/`dev` 输出前都会 `sort` + `dedup`（[main.rs](../../crates/rustify-docs-cli/src/main.rs) 443–444、[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 333–336）。
- **新增的 `rustc -vV` 没有设置 `RUSTUP_AUTO_INSTALL=0`**：`dev`/`new` 的 doctor 原本就在同一工作目录下直接运行 `rustc`（[new.rs](../../crates/rustify-docs-cli/src/new.rs) 95、114），所以这不是新的行为。
- **dev-dependency 也被纳入输入**：Cargo metadata 的 resolve 包含 dev-dependency，可能多跟踪少量文件。这只会造成多余的复用失效，不会让构建结果过期；修复前的输入范围更宽，不算回归。

## 待核实与验证边界

没有影响上述结论的待核实项。以下属于覆盖限制：

- 本轮没有重跑完整 `dev-components.spec.ts`（apply 记录为 4/4）、原生 `components`/`rust_dev` 集成、官方站三项目浏览器矩阵、主题/语言/缩放/故障注入矩阵或性能五样本。它们沿用 [M4 记录](../validation/docs-ssg-p2/M4.md) 与 [apply 证据](../validation/docs-ssg-p2/apply-r1/README.md)，本报告不计为本轮通过。
- 组件 wasm 的严格 clippy 被 SDK 未修改代码的 9 项既有 lint 阻塞，apply 记录已如实说明（加 `--no-deps` 后通过）。它不由修复差异引入，CI 也不跑该项。
- 浏览器只有 Playwright Chromium；M6 的稳定版 Chrome、可见 Safari、实体设备，以及 D16 后移的 M7 公开安装/Pages，仍按原计划范围保留。

## 覆盖与验证

变更意图：关闭 R1 的三项 P2——由挂载根稳定拥有几何作用域标记；分别读取 SSR/hydrate 两张图，使本地依赖清单参与编译失效；用 Cargo 回报的完整包身份绑定 build-script 输入。随后冻结新的库 R5 / CLI C5 配对，并补齐浏览器与 dev 回归。

仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。原输入：`/review-code docs/plan/DOCS-SSG-P2.md`。已有本计划的 R1 代码评审报告，因此按开发计划模式默认做复审。所用比较：`git diff --no-ext-diff --no-textconv 53d5c48563420c9252cac3f9d1e14647494320a3 d2f20b03cf58c1d1ced3785397c501d412cd4167 --`。

| 提交 | 计划登记 | 内容 |
| --- | --- | --- |
| `7930fd9a978563e76468fef69d14f444159c47b5` | M1/M4/M6「修复」 | `geometry.rs`、`rust_site.rs`、hydration harness/spec，以及 M1/M4 记录与 apply 证据 |
| `45878ce31763fe21b9276e563e2920d180787dee` | M4/M6「修复」 | CLI 0.1.15、`framework-revision.txt`→R5、模板 lock 的 6 个框架源改为 R5、`docs/docs-ssg.md` 配对说明 |
| `e8bf97b497d186d5e4f5a1dbfb2d4a3377a5dbb1` | M1/M4/M6「验证」 | `dev-components.spec.ts` 两条依赖回归与 apply 证据 |
| `d2f20b03cf58c1d1ced3785397c501d412cd4167` | 进度回写 | 只改计划，不作为代码 findings 对象 |

纳入审查的代码文件共 14 个：`Cargo.lock`、`crates/rustify-components/src/geometry.rs`、`crates/rustify-docs-cli/{Cargo.toml,assets/components-Cargo.lock,assets/framework-revision.txt,src/rust_site.rs}`、`docs/docs-ssg.md`、`tests/docs-ssg/dev-components.spec.ts`、`tests/dom-only/hydration.spec.ts`，以及 `tests/dom-only/hydration/{public/app.js,public/harness.css,src/browser.rs,src/geometry_branches.rs,src/lib.rs}`。`docs/validation/**` 只作证据读取。

阅读过的调用链：`mount`/`hydrate` 中 `ScopeRoots` 与 Owner 的建立（[mount.rs](../../crates/rustify-ui/src/mount.rs)）；reactive_graph 0.2.15 中 `Owner::parent` 与 `use_context` 沿父链查找的实现；docs 岛 `provide_geometry_styles` 的三个调用点；`prepare_checked_with` 从 metadata 到 `InputLayout`、两次构建、构建后发现输入，再到 `Prepared::reuse` 指纹比较的全过程；dev 的事件过滤与 watch 累加。另核对了 R/C 配对：`components-Cargo.lock` 只有框架源 rev 改变，仓内不再引用 0.1.14、`3fcc626a` 或 `d301344e`。修复源码中没有计划或需求 ID。

当前工作树：HEAD 未变，index 为空（`git write-tree` = `a0091f3e8cfa0b5b914c863468833a854360f1f5`）。tracked 未提交 18 个文件，逐文件清单的 SHA-256 为 `c5f70beea207882075e1de55b1cc08133864f9fa0db6e9e39e6e5cebdbe83f9c`；untracked 144 个文件为 `1a24c8b77e01c5c17f8abde7c6caf837fe775aa6096a188bdbba6f00d538fefe`（清单见[证据目录](../validation/docs-ssg-p2/code-review-r2/README.md)）。内容是 SDLC 台账、R1 报告/证据，以及夹具和官方站的私有生成缓存/日志，没有作者源码改动，因此不形成代码 findings。验证结束后重算两组指纹，均未变化，只多出本轮证据目录；本报告与证据不纳入审查对象。

| 实际执行 | 版本 / 入口 | 结果 |
| --- | --- | --- |
| `PATH=<python3.12>:$PATH mbx xtask docs-verify --group features` | HEAD 工作树；在 HEAD 重建 hydration harness（ssr/hydrate/csr 分图）与组件站 | exit 0；[日志](../validation/docs-ssg-p2/code-review-r2/features.log) |
| `npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts --workers=1` | 上一步产物，Playwright Chromium | **12 passed**，0 skipped/flaky；[日志](../validation/docs-ssg-p2/code-review-r2/dom-hydrate.log)、F-01 [两种清理顺序的观测](../validation/docs-ssg-p2/code-review-r2/branch-first-cleaned-first.json)（[另一顺序](../validation/docs-ssg-p2/code-review-r2/branch-second-cleaned-first.json)） |
| `mbx test --locked -p rustify-docs-cli --bin rustify-docs` | HEAD | **74 passed**，0 failed，1 ignored（原配置）；[日志](../validation/docs-ssg-p2/code-review-r2/cli-unit.log) |
| `npx playwright test --config=playwright.docs.config.ts --project=docs-dev tests/docs-ssg/dev-components.spec.ts --grep '<两条依赖回归>' --workers=1` | 已安装 C5（SHA-256 `5589466a…`，source `45878ce3`，0.1.15），离线；作者工程以路径依赖指向 HEAD 框架包 | **2 passed**，3.2 min，0 重试；失败请求、console、pageerror、CSP 均 0；[日志](../validation/docs-ssg-p2/code-review-r2/dev-deps.log)、[观测摘要](../validation/docs-ssg-p2/code-review-r2/dev-dependencies-summary.json) |
| `cargo fmt --all -- --check`（根目录与 `tests/dom-only`） | HEAD | 均通过 |

修复前的红证据没有重做，沿用 R1 的真实浏览器失败样本，以及 apply 记录中的定向红测（[F-01](../validation/docs-ssg-p2/apply-r1/f01-red-browser.log)、[F-02](../validation/docs-ssg-p2/apply-r1/cache-f02-red.log)、[F-03](../validation/docs-ssg-p2/apply-r1/cache-f03-red.log)）。本轮修后验证全部在当前 HEAD 或其配对 C5 上运行，没有拿其他历史版本的结果充当当前证据。没有安装工具或服务，没有执行提交、推送或发布；本轮自启的进程都已退出。

## 计划符合性

| 本期契约 | 实现 / 验证证据 | 核查状态 |
| --- | --- | --- |
| M1 / V-2 及既有 CSR 回归：确定初态、几何接管、原消费者不回退 | 挂载根共享 token；CSR 两分支回归与原 hydrate/SSR/trap 用例同组 12/12 | **已核实**（R1 F-01 关闭） |
| M4 / V-9、§3 Rust build cache、WORK-F13/WORK-N6：依赖与 build 输入失效，源码与 wasm 同代，失败保留上一版 | 两图 metadata、完整包身份回执；单元 74、真实 dev 2/2（含删除时保留上一版与恢复） | **已核实**（R1 F-02、F-03 关闭） |
| §2.1 R/C 配对纪律：库 R 冻结后，C 只改版本、模板 rev/lock 与文档 | `45878ce3` 只改版本、rev、lock 与文档；`--build-info` 与内置 R5 一致 | 已核实；沿用前几轮的做法，CLI 源码改动随 R 提交 |
| M6 环境 / 本人验收、M7 公开安装与 Pages、P3/P4 | 计划仍为 5/7，D15/D16 原状保留 | 后续范围；本轮不评判，也不因此报缺陷 |

## 总体结论与修复顺序

**验收判定：通过验收。**

修复结论：**已解决**。R1 的 F-01、F-02、F-03 均已实际关闭（3/3，0 项不再适用），没有阻塞性回归。对修复差异 `53d5c485..d2f20b03` 的结论是 `patch is correct`：修复抓住了各自的根因（挂载根级的 token 所有权、按目标图隔离的输入集合、按 Cargo 完整包身份绑定），回归测试能区分修复前后的行为，且在当前 HEAD 和配对的 C5 上实际运行通过；修复差异中没有发现新的 P0–P3 问题。

无需修复。本判定只覆盖上述修复差异和 R1 的旧问题，不是对 `785f51ad..d2f20b03` 的完整重审，也不授权合并、发布或 PRD 终验。M6 的设备/浏览器缺口和 M7 的公开发布，仍按计划 D16 留待后续。

下一步（计划来自 PRD，可选）：

```text
/review-prd-dev-gaps docs/plan/DOCS-SSG-P2.md
```
