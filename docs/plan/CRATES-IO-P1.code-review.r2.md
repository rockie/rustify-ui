# CRATES-IO-P1 · 代码评审 R2（修复效果复审）

**判定：通过验收（patch is correct；上轮 F-1–F-4 已解决）**

- 评审类型：代码（开发计划模式，修复效果复审）
- 评审时间：2026-10-24 09:31 UTC（17:31 +0800）
- 计划：[CRATES-IO-P1](CRATES-IO-P1.md)（sha256 `938cac01137d1c1b37546d39bb1c2adcdad8a6e3458a46e6abc61784527ce3ed`）
- 上轮报告：[CRATES-IO-P1 代码评审 R1](CRATES-IO-P1.code-review.md)（sha256 `f6cb2d18c5e2852dd50a1250e91e7b8d85dc30ce5c70c976e41238ea0b1959e9`）

## 1 评审对象

| 项 | 值 |
| --- | --- |
| 轮次 | R2（修复效果复审；上轮 R1） |
| 仓库根 | `/Users/rockie/Documents/gh-rockie/rustify-ui` |
| 分支 / HEAD | `main` / `c693157206f4a4c80d2f37a2261a1a827da9565e` |
| 上轮审查基线 → 上轮目标 | `2c0897c67347f23cca48e85e0bc2665f0f2d2f86`（`37c77f5e^`，树 `b3aedd558afc`）→ `44b19999`（树 `6cc1b3b3c712`） |
| 本轮修复差异 | `44b19999` → `1a8376e0`（树 `2690b1282c9f`），13 个非计划文件 + 4 份里程碑记录 + 计划正文 |
| 计划回写提交 | `c6931572`（只改 `docs/plan/CRATES-IO-P1.md`，不进入代码 findings 对象） |
| 当前工作树 | dirty；仅 `.xgent-ai/sdlc/*` 台账与 R1 报告（`docs/plan/CRATES-IO-P1.code-review.md`，untracked），无产品代码差异 |
| 比较方式 | `git diff --no-ext-diff --no-textconv 44b19999 1a8376e0`；工作树 `git diff HEAD` + untracked 枚举 |
| 文件限制 | 无 |
| 计划版本 | 现行计划（`c6931572` 后工作树版本，sha256 见上） |

### 复审记录

- 上轮报告相对链接与版本：[R1](CRATES-IO-P1.code-review.md)（sha256 `f6cb2d18…`）。所审对象版本 `2c0897c6..44b19999`；本轮核查版本 `44b19999..1a8376e0` + 工作树 `c6931572`。
- 上轮问题由 `1a8376e0` 处置，处置记录：[M1](CRATES-IO-P1.records/M1.md)、[M2](CRATES-IO-P1.records/M2.md)、[M4](CRATES-IO-P1.records/M4.md)、[M5](CRATES-IO-P1.records/M5.md) 的「代码评审 R1 处置」段；计划登记见 `c6931572`。
- 本轮默认复审上轮全部问题（F-1–F-4）及直接受修复影响的回归；未重跑作者的浏览器全量与远端 V-12，范围限制见 §5。

## 2 上轮问题复审

| 来源 | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| F-1（R1，P2） | 拆分后 `identify_runtime`/GPU `watch_assets` 留在 `rustify-ui`，启动不再安装资源失败钩子；`rustify-ui` 根 `identify_runtime` 解析到核心空实现 | `crates/rustify-ui/src/diagnostics.rs:9-13` 新增门面 `identify_runtime`，`lib.rs:23` 显式重导出；`crates/rustify-ui/Cargo.toml:20` 移除孤立依赖 | **已解决** | wasm32+gpu 的 LLVM IR 显示 `rustify_ui::diagnostics::identify_runtime` → `rustify_dom::diagnostics::identify_runtime` + `rustify_ui::diagnostics::watch_assets`，后者调用 `makepad_platform::script::res::on_resource_failure`；遮蔽语义经最小实验确认（显式 `use` 胜 glob、模块内 item 胜 glob） | 无（真实浏览器终验随发布门，见 §5） |
| F-2（R1，P2） | 发行 workflow 只预取 3 个锁，脚本实收 8 个，空缓存下 packaging 会去取网 | `.github/workflows/crates-io.yml:61-68` 补齐 8 条预取；新增回归 `tests/crates-io/test_release.py:52-60` | **已解决** | 用被测脚本函数复算：修前 `required-available` 差 7 个 registry 输入，修后差 0（384/384）；新测试实跑 ok | 远端 CI 未派发（§5） |
| F-3（R1，P3） | `new components` 目录名与依赖同名时产出改名后的非法锁 | `crates/rustify-docs-cli/src/new.rs:232-262` 重名即拒绝，调用点在写盘前（`new.rs:329`）；新增单测与集成用例 | **已解决** | `new::` 单测 6 passed；`--test new_components` 6 passed（含新的拒写断言：无目录、无 staging、无 cargo 调用） | 已发布 `rustify-docs-cli 0.2.0` 仍含此缺陷，需新版本（§5） |
| F-4（R1，P3） | 根锁断言用 `HEAD:Cargo.lock` 自比，恒真 | `tests/crates-io/test_boundaries.py:63-72` 改为固定期望集并显式列 `syn` 三版本 | **已解决** | 目标测试实跑 ok；断言现比对固定 `{(version, CRATES_IO)}` 与当前锁 | 无 |

上轮无「待核实项」被降级为结论；R1 的 §5 计划符合性偏差（§2.1.6 style 依赖与 js-sys/web-sys 残留）随本轮依赖清理一并消除。

R1 列了 3 项待核实，本轮状态：wasm32 + `gpu` 构建**已关闭**（本轮实际构建并读 IR）；测试矩阵与真实 CI runner 的两项仍未独立复核（见 §4），F-2 的复现仍以脚本函数复算加作者的空 `CARGO_HOME` 记录为准。

## 3 本轮新增问题

**未发现可确认的新增问题。** `1a8376e0` 的 13 个代码/配置文件逐项核对后，未发现由该修复引入或实际激活、且有实质影响的缺陷：

- 依赖清理不改变可用组合：`rustify_style` 的全部使用点都在 `feature = "utility"` 下（`crates/rustify-ui/src/gpu/mod.rs:19-20`、`gpu/button.rs:15-16`），`gpu` 去掉 `dep:rustify-style` 后 `gpu`、`gpu,utility`、`hydrate`、`ssr` 四种组合均可编译；`js-sys`/`web-sys`/`wasm-bindgen` 在 `rustify-ui` 已无直连使用点（`region.rs:202-216` 走 `leptos::` 重导出），fixture 锁仍 `--locked` 可解析。
- 名称遮蔽不产生歧义或回退：门面以显式 `use` 覆盖 glob，`diagnostics` 内以 item 覆盖 glob，两处都由最小实验与 IR 验证。
- 新回归测试不空转：workflow 预取测试在修前 workflow 上会以 7 个缺失输入失败（本轮实测），`new_components` 用例同时覆盖新建与已存在空目录两条路径。

## 4 待核实项与范围外观察

### 待核实

| 项 | 影响 | 获取方式 |
| --- | --- | --- |
| 真实浏览器 GPU 资源失败用例（M1 记录称 `--grep 'a missing font'` 修前红、修后绿） | F-1 的端到端行为只由作者的记录与本轮 IR/编译支撑，本轮未独立复现 | `mbx xtask build-web --example fusion-basic --release --base /tools/demo/` 后跑对应 Playwright 用例 |
| V-10 完整 29 项与 V-12 双平台远端 run | 本轮未重跑，远端未派发；R1 时已记录为未覆盖 | 派发 workflow 并读回 run 结论 |
| `make` 级 gates（`mbx verify` / `xtask verify`） | 本轮只跑定向命令，未跑全量门 | 按计划 §9 入口执行 |

### 范围外观察（不进入本轮 findings，不作为判定依据）

- `examples/theme-studio` 的 wasm32 构建失败（`examples/theme-studio/src/gpu_preview.rs:63`、`:64`、`:78`，`set_scroll_top`/`scroll_top` 为 `i32` 而信号是 `f64`）。该目录自 `fb288564` 起未改动，`web-sys` 在基线与当前同为 `0.3.105`，因此与本轮修复及所审变更无关；V-10 不构建 theme-studio，宿主侧 `--bins` 又因 `main.rs` 的 `#[cfg(target_arch = "wasm32")]` 编译掉该模块，现有门禁都不会捕捉。建议另开评审对象处理。
- `crates/rustify-ui/Cargo.toml:38-39` 的 `check-cfg = ['cfg(web_sys_unstable_apis)']` 在拆分后已无使用点（使用点在 `crates/rustify-dom/src/clipboard.rs:39`，且 `rustify-dom` 保留了同样条目）。属原变更遗留的孤儿配置，影响极小，不阻塞。
- 已发布产物与源码修复的差距：`rustify-docs-cli 0.2.0` 仍带 F-3；`rustify-ui` 为 `publish = false`，F-1 不进公开归档。要让修复对外生效，需新版本 + 重冻 R/C（计划 §5.4 的既有约定），不是本轮代码评审的验收条件。

## 5 覆盖与验证

### 变更意图

源码层面的 crates.io 首发（九包 0.2.0）已完成并发布；`1a8376e0` 是 R1 四个问题的源码修复：恢复 GPU 诊断接线并清掉孤立依赖（F-1）、补齐发行 workflow 的锁预取并加回归（F-2）、`new components` 拒绝重名（F-3）、把恒真的根锁断言改成固定期望（F-4）。

### 输入与固定版本

| 项 | 值 |
| --- | --- |
| 修复差异端点 | `1a8376e0`（父 `26d0c78e`），树 `2690b1282c9f` |
| 上轮目标 | `44b19999`，树 `6cc1b3b3c712` |
| 计划回写 | `c6931572`，树同 HEAD |
| 工作树差异指纹 | `git status --porcelain` 仅 4 个 M（`.xgent-ai/sdlc/*`）+ 2 个 untracked（sdlc review 对象、R1 报告）；`git diff --stat HEAD` 9 行全部在台账 |
| 纳入文件 | workflow、`rustify-ui`（Cargo.toml/lib.rs/diagnostics.rs）、`rustify-docs-cli`（new.rs、tests）、`tests/crates-io`（2 个 py）、`Cargo.lock` + 4 个 fixture 锁 |

### 实际执行的验证

| 命令 / 动作 | 结果 |
| --- | --- |
| `python3.12 -m unittest discover -s tests/crates-io -p test_release.py -k workflow_prefetch` | 1 test ok |
| `python3.12 -m unittest discover -s tests/crates-io -p test_boundaries.py -k caret_floors` | 1 test ok |
| 复算 `lock_inputs` 与 workflow 预取集合（修前 / 修后 workflow） | 修前缺 7 个 registry 输入，修后缺 0（384/384） |
| `mbx test --locked -p rustify-docs-cli --bin rustify-docs new::` | 6 passed |
| `mbx test --locked -p rustify-docs-cli --test new_components` | 6 passed |
| `mbx check --locked -p rustify-ui --no-default-features --features gpu --target wasm32-unknown-unknown` | 通过 |
| 同上，`--features gpu,utility` / `--features hydrate` / `--features ssr`（后两者 wasm32 与宿主） | 均通过 |
| `mbx clippy --locked -p rustify-ui --all-targets -- -D warnings`（默认，及 `--features gpu,utility`） | 两次均无告警 |
| `mbx rustc -p rustify-ui --features gpu --target wasm32 -- --emit=llvm-ir` | IR 见 §2 F-1 证据（`identify_runtime` → `note`/`record`/`on_resource_failure`） |
| `/tmp` 最小实验：显式 `use` 与模块内 item 对 glob 的遮蔽 | 输出 `core-identify` / `gpu-watch` / `gpu-watch` |
| `cargo metadata --locked --no-deps`（`tests/dom-only`、`gpu-negative`、`hydration`、`crates-io/compat`） | 4 个锁均 OK |
| `mbx check --locked -p property-workbench --target wasm32` | 通过（1 个既有 dead_code 告警），验证门面根 `identify_runtime` 对示例可用 |
| `mbx check --locked -p theme-studio --target wasm32` | 3 个类型错误（范围外，见 §4） |
| 真实 CLI `target/debug/rustify-docs new rustify-docs --template components` | 因本机默认工具链缺 wasm32 target 在工具链检查处退出（环境限制，未进入命名检查）；重名拒绝行为以 `new_components` 集成用例为证据 |

### 未覆盖

跨平台完整 corpus、docs.rs 复跑、远端 CI、真实浏览器全量、发布动作均未在本轮执行；本轮结论限定于所列提交与工作树范围。

## 6 计划符合性

| 计划条款 | 本轮核对 |
| --- | --- |
| §2.1.4 启动诊断与 `watch_assets` 纪律 | 门面解析 → 核心 `identify_runtime` + GPU `watch_assets`，经 IR 确认；核心 `note`/`record` 仍是唯一写入点 |
| §2.1.6 feature 转发与依赖裁剪 | `gpu` 不再启用 `dep:rustify-style`，`utility` 保留；`js-sys`/`web-sys`/`wasm-bindgen` 已移除，四种 feature 组合均可编译 |
| 运行时边界：作者项目门、发行记录、预取八锁 | workflow 与 `docs/crates-io-release.md:27-34`、`scripts/crates-io.py:510-517` 一致；新回归锁定该集合 |
| §5.2 三包模板与本地校验 | `new components` 重名前置于写盘，模板与发行记录字节未变（记录声称，本轮只核源码与用例） |
| V-3 根锁与 caret 下限 | 断言改为固定期望集，漂移即失败 |
| V-12 预取后离线 | 预取集合与脚本实收一致；远端未重跑 |

## 7 总体结论

**通过验收**（修复差异 `patch is correct`；上轮 F-1–F-4 结论均为**已解决**，没有仍需修复的问题；无 P0–P2，未新增 P3）。

- 四条修复都落在根因上：F-1 用门面接线而不是在示例里补调用，F-2 用集合等价的回归测试而不是只补命令行，F-3 在写盘前拒绝，F-4 把「自比」换成固定期望。本轮以编译、IR、单测与集成测试分别复核，未发现修复引入的新问题。
- 结论范围：只覆盖 `44b19999..1a8376e0` 与当前工作树。R1 审查过的其他区域本轮未重新全量审查；`theme-studio` 的既有 wasm 构建失败与 `rustify-ui` 的孤儿 `check-cfg` 属范围外观察，不构成本轮判定依据。
- 后续动作（不属代码验收）：如需让 F-3（以及公开建档行为上的 F-1/F-2 相关变更）对外生效，须按计划换版本并重冻 R/C 后再走发布门；远端 V-12、真实浏览器全量与其余未跑门在发布前补齐。

按当前客户端的技能调用写法，下一步：

```text
（无需修复）本计划代码验收完成；如需针对范围外观察开新对象：
$review-code 审查 examples/theme-studio 的 wasm32 构建失败
```

（若使用 Claude Code：`/review-code` 同上。本轮评审未修改任何产品代码、计划或记录；报告与台账登记是本轮唯一产物。）
