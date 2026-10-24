# CRATES-IO-P1 · 代码评审 R1

**判定：需要修改（patch is incorrect）**

- 评审类型：代码（开发计划模式，初次评审）
- 评审时间：2026-10-24 08:37 UTC（16:37 +0800）
- 计划：[CRATES-IO-P1](CRATES-IO-P1.md)（sha256 `2cb149424a68f219cf68db7ca24d2aad9f8a67650ae759c25e1c395bc924cbf3`）

## 1 评审对象

| 项 | 值 |
| --- | --- |
| 仓库根 | `/Users/rockie/Documents/gh-rockie/rustify-ui` |
| 分支 / HEAD | `main` / `26d0c78ef488242ac800630e4b6df3a8ab3d5ee2` |
| 模式 | 开发计划模式（计划「完成记录」登记的提交 + 工作树）；无上轮代码评审报告，本轮为 R1 |
| 基线 | `2c0897c637…`（`37c77f5e^`，树 `b3aedd558afcdb0891744ddcfd95f26238f9cbdf`） |
| 目标 | `44b19999da2b1fa1dedb3926c914278bc54b952b`（树 `6cc1b3b3c712c56d9e77edcfdb3475584f84f9c8`） |
| 比较方式 | `<最早登记提交>^1..<最新登记提交>` 的净差异（登记提交之间只有计划/记录回写提交） |
| 规模 | 272 文件，+12 686 / −3 955 |
| 工作树 | 干净（`git status --porcelain` 0 行）；无未提交代码，无 untracked |

登记提交（计划「完成记录」『提交』列，均已解析为完整 SHA 且都在当前分支）：

| 里程碑 | 提交 |
| --- | --- |
| M1–M4 | `37c77f5eacbaba5b7e307c62118b9f178d74366a` |
| M5 | `4deb101dff9227ac4f0f313c10dc4c7b5815640c`、`6142e85e32787768db24a7730393786499b4f43c`、`ba9260ee08855a7469e4728403c087f35c8406d0`、`cc10ad5e2c7b4fb4e326dc70cdac36fdc142485d`、`43004b03e0d54ac5dffcae4d3910364a70d8d9e1` |
| M6 | `43004b03e0d54ac5dffcae4d3910364a70d8d9e1`、`54b4bb7716c916ac36a9deba4bae44a163402dba`、`9b7684cd1b81718918eeb85a337b59683cef9647`、`7257d2e34218ba2ce49e76f3f17b08af07d63b8e`、`44b19999da2b1fa1dedb3926c914278bc54b952b` |

区间内共 20 个提交。排除为进度回写（只改计划、记录目录或 `.xgent-ai/` 台账，不含代码）：`f1c798c9`、`cf9629ef`、`2fed1fa9`、`594405d8`、`43004b03`、`31a5fc76`、`68dbe5cf`、`45ddf91a`、`7eca103f`、`44b19999`、`26d0c78e`。

未在「完成记录」登记、但落在区间内的提交：`a9cfa79a`（仅 `CLAUDE.md`、`DESIGN.md`、`PRODUCT.md`、`docs/architecture.md`、`docs/components.md`、`docs/utility.md`、`docs/vellum.md` 与计划/记录，无代码）。按开发计划模式规则：它不是登记提交，但也不构成需要排除的无关代码改动，本轮把它作为文档一致性上下文纳入净差异，不作为 finding 对象。

变更意图（一句话）：把 DOM 核心抽成可上 crates.io 的 `rustify-dom`，冻结九包 0.2.0 的 registry 配对、模板锁与发布编排，并把内置 CJK/emoji 字体换成项目自供的本地字体。

## 2 确认问题

### F-1 · GPU 区域资源失败诊断在门面里静默失效（P2，置信度：高）

**定位**：`crates/rustify-ui/src/lib.rs:17`、`crates/rustify-ui/src/diagnostics.rs:15`、`crates/rustify-dom/src/diagnostics.rs:587-609`、`crates/rustify-dom/src/lib.rs:52-53`（本次新增；基线对照 `37c77f5e^:crates/rustify-ui/src/lib.rs:50`、`37c77f5e^:crates/rustify-ui/src/diagnostics.rs:610`）

诊断模块整体移入核心后，GPU 侧的 Makepad 资源失败钩子留在门面，但已经没有任何调用者，且门面根路径解析到的同名函数变成了核心的空实现：

1. `identify_runtime` 在核心（`crates/rustify-dom/src/diagnostics.rs:587`），函数体第 592 行调用同一模块的 `watch_assets()`；核心的这个函数是空实现（同文件 `:609`）。
2. 核心根把同名空实现再导出（`crates/rustify-dom/src/lib.rs:52-53`）；门面第 17 行 `pub use rustify_dom::*;` 把它带进门面根，而门面根此后只声明 `diagnostics` / `theme` / `utility` 三个模块（`:21-25`），没有根级显式 `watch_assets`。显式项压过 glob，但这里根本不存在竞争项，因此 `rustify_ui::watch_assets` 就是那个空函数。
3. 真正的 GPU 实现留在 `crates/rustify-ui/src/diagnostics.rs:15`（`cfg(all(target_arch = "wasm32", feature = "gpu"))`，内含 `on_resource_failure` 注册）。全仓 `grep -rn watch_assets` 只有定义、核心内的空实现调用和文档，没有调用点。
4. 基线的 `rustify-ui` 根显式重导出列表里有 `watch_assets`，解析到当时同模块里的 GPU 实现；而 `identify_runtime` 当时也在同一个模块，因此每次启动都会装上钩子。迁移把这些拆到了两个 crate，链就断了。
5. 所有 GPU 示例都经 `rustify_ui::identify_runtime` 启动：`examples/component-catalog/src/main.rs:1863`、`examples/fusion-basic/src/main.rs:976`、`examples/data-workbench/src/main.rs:1588`、`examples/theme-studio/src/app.rs:610`、`examples/property-workbench/src/main.rs:2567`、`examples/utility-lab/src/main.rs:582`、`examples/vellum/src/automation.rs:17`。

**实际 vs 期望**：wasm32 + `gpu` 构建里 `on_resource_failure` 永不注册，`ErrorKind::AssetLoadFailed`（`about_asset`）这一条诊断不再产生，区域资源 404 只剩 fork 内部日志。没有编译错误、没有测试失败，只有部署漏文件时才看得出。期望是计划 §2.1.4 的「GPU `watch_assets` 继续通过核心公开的 `note` / `record` 写同一份日志」，以及 §0.5 分期与排除的后置约束「本期仅保证提取后现有 GPU 功能不回退」（`docs/plan/CRATES-IO-P1.md:145`）。

**修复方向**：二选一。①核心把「启动时安装」做成可注册钩子（核心 `identify_runtime` 调用它，门面在启动流程里注册 `on_resource_failure`），保持「核心不依赖桥接」；②门面自己声明根级 `identify_runtime` 包装（显式项压过 glob），先调核心再调门面的 `watch_assets`。只补一行 `pub use diagnostics::watch_assets;` 不够——没有调用者，钩子仍然不装。

### F-2 · CI 预检的依赖预取与打包脚本要求的锁输入不一致，工作流在干净 runner 上无法跑通（P2，置信度：高）

**定位**：`.github/workflows/crates-io.yml:63-68` 与 `scripts/crates-io.py:513-516`、`:562-564`、`:286`（两者均由本次变更引入，后者在 `cc10ad5e` 中扩展）

`scripts/crates-io.py:513-516` 的 `lock_inputs()` 收集 8 份锁：根、`site/`、`makepad/`、`makepad/platform/script/`、`tests/docs-ssg/fixtures/**`、`tests/dom-only/**`。`package_libraries` 用这组锁 `stage_registry`（`:562-564`），而 `stage_registry` 对每个不在本批归档里的 registry 包都要求本机 sparse index 缓存与 `.crate`，缺一个就 `ReleaseError`（`:286`：`missing or inconsistent cached index for {name} {version}; run mbx fetch --locked`）。

工作流的预取步骤只有三条 `mbx fetch --locked`（根、`site/`、`tests/docs-ssg/fixtures/components/`）。按 `git ls-files` 过滤后取 8 份锁的 registry 包全集，减去这 3 份的并集，差 7 项：

| 包 | 来源锁 |
| --- | --- |
| `bitflags 2.13.1` | `makepad/Cargo.lock`（`wasmparser` 依赖） |
| `crc32fast 1.5.3`、`equivalent 1.0.3`、`manyhow 0.14.1`、`manyhow-macros 0.14.1`、`reactive_graph 0.2.16`、`libc 0.2.191` | `tests/dom-only/*/Cargo.lock` |

作者记录已从两侧印证这条链：

- `docs/plan/CRATES-IO-P1.records/M5.md:92`：本地 GPU 构建「因独立 cargo-makepad 工具的 bitflags 2.13.1 未在验收索引而停止……完整并集为 384，新增相对上一份索引仅 bitflags 2.13.1」——正是本地复现了同一失败，并以扩充 `lock_inputs` + runbook 解决。
- 同文件 `:111` 称 `cc10ad5e`「已保存 CI 预取、完整输入收集」，但该提交给工作流加的预取仍只有 3 条，而脚本收集扩到 8 份锁。
- 同文件 `:134` 自述「CI 登记 377 个……本地多出 7 项」，即两次预检的输入集合已经不同。
- `docs/crates-io-release.md:27-34` 的 runbook 列了 8 条 `mbx fetch --locked`，与工作流不一致。

历史一致性检查：C3 `ba9260ee` 的脚本只收集 3 份锁（`git show ba9260ee:scripts/crates-io.py:502-520`），因此 V-12 那次 run 的 377 项与当时脚本自洽；缺陷是 `cc10ad5e` 只扩了脚本与手册、没扩工作流。

**实际 vs 期望**：任何以当前脚本冻结的新 R/C 再次 dispatch 该工作流，都会在 `package-libraries` 的 `stage_registry` 停下（干净 runner 没有这 7 个包的缓存），V-12 门不可复跑。计划 V-12 明确要求「依赖预取后 `--offline`」，`.github/workflows/crates-io.yml` 是这个门的唯一实现。期望工作流的预取集合等于脚本的输入集合（或直接由脚本枚举 `lock_inputs` 后逐个 fetch，避免两处清单漂移）。

### F-3 · `new components` 的根包名与内嵌锁里的包名撞名时，生成必然失败的项目（P3，置信度：高）

**定位**：`crates/rustify-docs-cli/src/new.rs:163-183`（`package_name`）、`:232-256`（`named_lock`）、`:321-323`（调用）、`:274-284`（由目录名取名）

`named_lock` 只把锁里那条无 `source` 的 `docs-site` 记录改名成新包名，不检查新名字是否已存在于锁内。当根包名等于模板锁里任一包名时，锁同时含 `<name> 0.1.0`（根）与 `<name> 0.2.0/0.8.22`（依赖），Cargo 必须重解算，而 CLI 的所有后续命令都带 `--locked`。

**复现**（隔离临时目录，不修改仓库）：用 `templates/components/Cargo.toml.in` 与 `assets/components-Cargo.lock` 原样渲染，仅按 `named_lock` 规则改根记录名，再跑 `cargo metadata --locked --offline --no-default-features --features ssr`：

| 目录名 / 包名 | 结果 |
| --- | --- |
| `docs-site` | 0 |
| `my-docs` | 0 |
| `rustify-docs` | 1，`error: cannot update the lock file … because --locked was passed` |
| `rustify-dom` | 同上 |
| `leptos` | 同上 |

**触发**：`rustify-docs new components rustify-docs`，或在名为 `rustify-docs` 的空目录里 `rustify-docs new components .`（`:274-284` 把 `.` 规范化为真实目录名；`:163-183` 只做小写与连字符化）。同族还有 `rustify-components`、`wasm-bindgen`、`serde` 等锁内包名。`new` 自身不调用 cargo（`:321-372`），返回成功并落盘。

**实际 vs 期望**：项目一落地就是坏的。`check`/`build`/`dev` 的 `cargo metadata --locked`（`crates/rustify-docs-cli/src/rust_site.rs:833-880`）直接失败，用户看到的是裸 cargo 的 `cannot update the lock file`，没有任何提示说明这与目录名有关；计划 §5.2 要求「`new` 只替换根包名，不联网重新解算」（`docs/plan/CRATES-IO-P1.md:329`），前提是替换后的锁仍然自洽。期望在 `new` 阶段拒绝这些名字（或改名后校验根包名不在锁内）。

### F-4 · V-3 的根锁版本断言在干净检出上恒真（P3，置信度：高；验证缺口，非产品缺陷）

**定位**：`tests/crates-io/test_boundaries.py:63-76`

`test_inventory_and_caret_floors_preserve_locked_third_party_versions` 用 `git show HEAD:Cargo.lock` 与工作树 `Cargo.lock` 比较五个第三方包的版本集合。在任何干净检出上两者按定义相同，所以这条断言提交之后不可能失败：一旦版本漂移被提交，漂移就同时进入 `HEAD` 与工作树。它只对「未提交的锁漂移」有效。

计划 §2.3 要求「V-3 断言五个范围及根锁版本未意外变化」。这一条现在给出的是没有保护力的绿灯。建议改为与一份入库的期望值（或固定的基线 SHA）比较。

## 3 待核实项

- **测试矩阵未独立重跑**。本轮没有执行 `tests/docs-ssg` 的 Playwright 套件、`tests/crates-io/*` 的 Python 套件、浏览器门和 wasm 链接；作者在 M5/M6 记录里给出了这些门的证据，本轮沿用而未复核。若需要「测试通过」的独立结论，需要按计划 §9/§5.4 重跑。
- **`rustify-ui` 的 wasm32 + `gpu` 构建未执行**。F-1 的结论来自名字解析、导出清单与调用链的静态证据（决定性，不含推断），但没有通过实际 wasm 构建或浏览器观察确认「钩子未装」的外部表现。
- **F-2 未在真实 CI runner 上复现**。结论由「脚本要求的输入集合」与「工作流预取集合」的差集推出（差集 7 项已逐项列出并核对来源锁），未用全新 `CARGO_HOME` 跑一次 `package-libraries` 验证停在 `stage_registry`。M5 记录 `:92` 的本地同类失败是对该机制的旁证。

## 4 覆盖与验证

### 4.1 实际读取并核对的代码

- 发布编排：`scripts/crates-io.py`（全文 914 行，重点 `inspect`/`stage_registry`/`package_libraries`/`freeze_cli`/`package_cli`/`verify`/`publish`/`Registry`/`lock_inputs`）、`release/crates-io.toml` 语义、`.github/workflows/crates-io.yml`、`docs/crates-io-release.md` 的取锁与上传段。
- registry 配对：`crates/rustify-docs-cli/src/release.rs`、`release/source.rs`、`build.rs`、`main.rs` 的 `--build-info`/`__panic-probe`、`assets/framework-release.json`、`assets/components-Cargo.lock`。
- `new`：`crates/rustify-docs-cli/src/new.rs` 的 `package_name`/`template_pair`/`fill`/`named_lock`/`run`。
- 字体：`crates/rustify-docs/src/config.rs:295-430`（`parse_fonts`）、`:470-491`（`relative_path`）、`crates/rustify-docs/src/content.rs:167-176`（`is_private`）、`crates/rustify-docs-cli/src/theme.rs:30-243`（`prepare`/`validate_snapshot`/`input_path`）、`crates/rustify-docs/src/site.rs:60-66`、`crates/rustify-docs/src/site/registry.rs:320-330`、`crates/rustify-dom/src/theme/resolve.rs:28-32`。
- 门面与核心：`crates/rustify-ui/src/lib.rs`、`crates/rustify-ui/src/diagnostics.rs`、`crates/rustify-ui/Cargo.toml`、`crates/rustify-dom/src/lib.rs`、`crates/rustify-dom/src/diagnostics.rs`、`crates/rustify-makepad/Cargo.toml` 与 `src/lib.rs`；基线对照 `git show 37c77f5e^:crates/rustify-ui/{lib.rs,diagnostics.rs,Cargo.toml}`。
- 打包面：九个首批包的 `Cargo.toml`（`publish`/`include`/`readme`/`documentation`/`docs.rs` metadata/依赖版本形态）、根 `Cargo.toml` 的 `[patch.crates-io]` 移除、六个 Makepad manifest 与 `makepad/Cargo.lock`、`makepad/platform/script/Cargo.lock`、根 `Cargo.lock` 的 bitflags/smallvec 身份。
- 测试：`tests/crates-io/test_{boundaries,inspect,pairing,release}.py`、`crates/*/src/repo_tests.rs` 与 `rustify_repo_tests` 白名单、`tests/docs-ssg/dev-fonts.spec.ts`、`tests/docs-ssg/custom.spec.ts:492-510`、`tests/docs-ssg/serve.sh:70-105`。

### 4.2 实际执行的命令与结果

| 命令 | 结果 |
| --- | --- |
| `git status --short`、`git rev-list`、`git diff --name-status -M 37c77f5e^ 44b19999` | 工作树干净；20 提交；272 文件 |
| `/opt/homebrew/bin/python3.12 scripts/crates-io.py inspect` | 通过；九包白名单、内部拓扑、元数据与 `registry_dependencies: 330` 一致 |
| `mbx +1.99.0 check --locked -p rustify-makepad --lib` | 通过（20.6 s）。否定了一条候选问题：核心在「无渲染模式」配置下可编译，桥接包可在宿主侧检查 |
| 8 份锁 × 3 份预取锁的 registry 包差集计算（Python 3.12，`git ls-files` 过滤） | 7 项缺失，即 F-2 |
| `cargo metadata --locked --offline --no-default-features --features ssr`（5 个临时项目，/tmp 隔离） | `rustify-docs`/`rustify-dom`/`leptos` 失败，`docs-site`/`my-docs` 成功，即 F-3 |
| 三个 0.2.0 归档 SHA-256 vs `assets/components-Cargo.lock` 内 checksum | 3/3 相同（`rustify-dom 8daff510…`、`rustify-docs d9cabaee…`、`rustify-components 3c50a65c…`） |

未运行：任何 `cargo test`/`mbx test` 套件、wasm 构建、Playwright、真实上传或 registry 写入。所有临时产物都在 `/tmp` 下，未向仓库写入测试或修复。

### 4.3 已核对、未构成缺陷的点

- registry 分支的同源校验按计划收紧：`[patch]`/git rev/path/别名/多版本共存/非 canonical crates.io 源都拒（`release.rs:76-160`），依赖键 alias 不影响包名判定，`Metadata::read` 只保留作者可达子图；命中即升为 `Severity::Error` 并整体返回 `None`（`rust_site.rs:499-507`）。
- 模板锁与真实归档一致：三包 `version`/`source`/`checksum` 齐备且与实际 crates.io 归档字节相符（上表），manifest 只用 `=0.2.0` registry 依赖。
- 字体配置防御到位：`family` 拒控制字符、`src` 拒 URL/绝对路径/越界/私有/输出目录、扩展名白名单、weight/style 规范化、`family+weight+style` 重复项报 `theme-font`；读取侧 `input_path`（`theme.rs:245-278`）先 canonicalize 再 `strip_prefix` 根的，逃逸 symlink 被拒。
- SSR/hydrate 字体来源一致：只有 `crates/rustify-docs-cli/src/rendering.rs:147-150` 在生产路径构造 `ThemeInput`，显式传 `Css`；另外两处 `Default::default()` 都在 `#[cfg(test)]` 里。客户端从快照读回同一字段（`site/registry.rs:323`）。
- 浏览器字体断言覆盖计划 §3.5 的关键项：`dev-fonts.spec.ts` 验证 SHA-256 文件名、`font/ttf` MIME、字节一致、删除→`theme-font` 错误且保留上一代、恢复；`custom.spec.ts:492-510` 的 system 场景验证零字体请求、无 `@font-face`、`document.fonts.size === 0`。
- corpus 门确实运行哨兵：`test_corpus_flag_runs_each_real_sentinel` 带 `--cfg rustify_repo_tests` 跑六个包并断言 `repo_tests::repo_corpus_sentinel ... ok`；各包 `unexpected_cfgs` 白名单已登记。
- 打包面：九个包显式 `publish = ["crates-io"]`，`rustify-ui`/`rustify-makepad` 仍是 `publish = false`；非 dev 路径依赖都带 `version`，紧配对边为精确版本；`docs.rs` 配置与计划一致（`dom`/`components` 关默认 feature 开 `ssr`，`style-wasm` 用 wasm32 + `compiler,scanner`，无 all-features）；根 `[patch.crates-io]` 已移除，`Cargo.lock` 里 bitflags 2.10.0 / smallvec 1.15.1 均为 registry 身份带 checksum，六个 Makepad manifest 改为 `=2.10.0`/`=1.15.1`。
- 跨包资源读取已改走 crate API（`rustify_components::styles::{SDK,SLIDER,CONTROLS,THEME_V4}`），各包 `include_str!` 全部落在自己目录内（`assets/**`、`css/**`、`templates/**` 都在对应 `include` 清单里）。
- 发布脚本的续接语义与计划一致：dry-run 归档与 receipt 逐项比对后才上传；上传后不可见即 `Waiting`（退出 75）并要求先查实际 registry 再重试；429 记录 `next_retry_at`；镜像重建索引后重打包比对 checksum。

### 4.4 未覆盖范围

- `crates/rustify-docs-cli/src/rust_site.rs` 共 3171 行，只审了配对/身份/metadata/缓存摘要相关路径；`crates/rustify-docs/src/{content,site,style}.rs` 只细读字体相关部分。
- `xtask/src/{docs,docs_corpus,css}.rs`（+229/−45）只核对了扫描树与 snippet 前缀的重写串。
- 未审查 GPU 侧 Makepad 分支源码（上游 fork，大量未改）。

## 5 计划符合性

| 计划项 | 实现证据 | 状态 |
| --- | --- | --- |
| §2.1.1–2.1.3 核心拆分、同一类型、`Pace` 归核心 | `crates/rustify-dom` 新增；门面 glob + 三个局部门面模块；`cargo check -p rustify-makepad --lib` 通过 | 已核实 |
| §2.1.4 GPU `watch_assets` 继续写同一份日志 | 见 F-1 | **存在缺陷** |
| §2.1.5 `external-host` 与 DOM host 导出判定 | `crates/rustify-dom/src/runtime.rs` 的 `not(external-host)` 判定；门面 `gpu` 启用该 feature | 已核实（`dom-loader.js` + gpu 同图的组合不在本期验收范围） |
| §2.1.6 原 feature 转发、清理孤立依赖 | 转发齐备；但 `gpu` 仍启用 `dep:rustify-style`（`crates/rustify-ui/Cargo.toml:20`），而 `rustify_style` 全部使用点都在 `feature = "utility"` 下；`rustify-ui` 的 `js-sys`/`web-sys`/`wasm-bindgen` 已无直连使用者（只经 `leptos::` 使用） | 轻微偏差，无编译/行为后果 |
| §2.2 移除组件 `gpu` 便捷 feature | `crates/rustify-components/Cargo.toml` 无 `gpu` feature；GPU 由应用在 SDK 上启用 | 已核实 |
| §2.3 打包清单、元数据、caret 下限、根 patch 移除 | `inspect` 通过；根 patch 已删；六个 Makepad manifest 改 registry 精确版本；`syn/quote/proc-macro2` 为 caret | 已核实；`rustify-classification-macros` 保留 `syn = "2"` 等宽主版本下限（测试注释说明为未修改的上游宏），属有意偏差 |
| §2.3 仓根测试 cfg 化 + corpus 门 | `cfg(all(test, rustify_repo_tests))` + 六包 `unexpected_cfgs` 白名单 + 哨兵断言 | 已核实 |
| V-3「根锁版本未意外变化」 | 见 F-4 | 验证缺口 |
| §3 字体：注册不选中、去重、警告、`@font-face` 注入、默认系统字体 | 配置校验与路径防护齐备；`assets/fonts/<sha256>.<ext>`；`theme-font-system` 按 lowercase 族名去重；闭引号族名不误报；CJK/emoji 回退已移除 | 已核实 |
| §5.3/§5.4 发行脚本的白名单、冻结、验证、上传、续接 | 见 §4.3；未执行真实上传 | 已核实（静态 + 单测 + `inspect` 实跑） |
| V-12 双平台预检工作流 | 见 F-2 | **存在缺陷** |
| §5.2 `new` 只替换根包名、不联网重解算 | 只改根记录名、不跑 cargo、不联网；但缺名冲突防护 | 见 F-3 |

## 6 总体结论

**需要修改（patch is incorrect）。**

本期主线交付是成立的：九包 0.2.0 的边界、元数据、配对记录与内嵌模板锁彼此自洽（三份真实归档 checksum 与模板锁逐字节相符，`inspect` 实跑通过），发布脚本的同源/checksum/续接语义经得起推敲，字体管线在配置校验、路径防护、SSR/hydrate 一致性和浏览器断言上都到位。上一轮文档评审暴露的路线问题没有以代码形式复发。

三条需要作者处理的缺陷分别属于三类边界：

- F-1 是**本次拆分直接造成的功能回归**，而且静默——违反计划自己写下的「提取后现有 GPU 功能不回退」。它不影响本期上架到 crates.io 的九包，但会让所有 GPU 应用失去一条诊断通道，且越晚发现越难归因。
- F-2 是**发布门自身的可复跑性**：脚本与工作流两处清单漂移，下一次冻结候选时 V-12 会在 `package-libraries` 停下。作者在本地已经踩过同一个坑（M5 记录 `:92`），只是没有把修复补到工作流。
- F-3 是**新用户首次落地的坑**，概率不高但代价直接：目录名撞上框架包名就得到一个立刻坏掉的项目，错误信息还指不到原因。

F-4 是验证缺口，不阻塞交付。

**修复顺序**：F-1 → F-2 → F-3 →（可选）F-4。F-1 需要改核心与门面两处并补一个能观察钩子的断言；F-2 最小改动是在工作流的预取步骤补齐八条（或让脚本枚举自己的输入集合再 fetch，避免两处清单再次漂移），改完需一次干净 runner 的 dispatch 作为证据；F-3 在 `named_lock` 后加一条「根包名不得与锁内任何包同名」的校验并补 `new` 的单测即可。

按当前客户端的技能调用写法，下一步：

```text
$review-code docs/plan/CRATES-IO-P1.code-review.md 复审修复效果
```

（若使用 Claude Code：`/apply-code-review docs/plan/CRATES-IO-P1.code-review.md` 处理上述问题，修复后再用本技能复审。）

本轮评审未修改任何产品代码、计划或记录；报告与台账登记是本轮唯一产物。判定只覆盖所审提交与工作树，不是合并、发布或需求验收授权。
