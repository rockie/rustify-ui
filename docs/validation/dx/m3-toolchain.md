# DX-REFINE M3 子任务记录：Tailwind 工具链（mise 二进制与精确扫描）

日期：2026-09-25。代码基线：`65b2225aba2bd48280214920edf77abd566a80c0` + 未提交工作树（本任务改动，另有并行 M1 对 `tests/`、`playwright.config.ts`、`.github/workflows/verify.yml`、`CLAUDE.md` 的改动，本任务未触碰）。

状态：本任务范围内的实现与本地验收已完成；**M3 未退出**——A-1（mise 选资产）在本环境不可实测，CI host job 去掉 `npm ci`、`verify` 加 `css --check`/`catalog --check` 不在本任务文件范围，里程碑由主 agent 在 CI 通过后判定。

## 实现记录

### 交付行为

| 文件 | 改动 |
| --- | --- |
| `mise.toml` | `[tools]` 加 `"github:tailwindlabs/tailwindcss" = { version = "4.1.13", bin = "tailwindcss" }`（显式 `github:` 后端，D10）；未加 `platforms`/`asset_pattern`，依据见「偏差与决定」 |
| `xtask/src/tailwind.rs`（新） | `VERSION = "4.1.13"`；`cli()`：`RUSTIFY_TAILWIND`（非空）→ PATH 上第一个 `tailwindcss`；以 `NO_COLOR=1` 跑 `--help` 解析横幅，不是 `v4.1.13`、无横幅、无法运行、不在 PATH 都报错并给出 `mise install` 提示（来自 `RUSTIFY_TAILWIND` 时提示改指向或取消它再 `mise install`）；`compile(input, output, minify)`：`-i/-o [--minify]`，cwd 固定为仓库根，失败返回 CLI 的 stderr |
| `xtask/src/css.rs` | 改用 `tailwind::compile()`，删除对 `node_modules/.bin/tailwindcss` 的依赖；`--check` 失败时在字节数之外打印首个差异行（行号 + 两侧内容，超过 120 字符时截取差异列附近的窗口，缺失的一侧显示 `<end of file>`）；模块头注释改为「产物提交入库、由 mise 固定的独立 CLI 生成、不需要 Node」 |
| `xtask/src/doctor.rs` | `tailwind` 检查改为报告 `tailwind::cli()` 解析到的二进制与版本（`tailwindcss v4.1.13 at <path>`），缺失或版本不符即 FAIL；`mise_pin` 改为 `pub` 供一致性单测复用 |
| `xtask/src/main.rs` | 注册 `tailwind` 模块；USAGE 的 `css` 行下注明 `RUSTIFY_TAILWIND=<path>` |
| `crates/rustify-components/css/rustify.tailwind.css` | utilities 导入改为 `layer(utilities) prefix(rui) source(none)`，注释说明原因；保留 `prefix(rui)` 与 `@source "../../../examples"`；产物逐字节不变 |
| `package.json` / `package-lock.json` | 删除 `@tailwindcss/cli`、`tailwindcss` 与 `css` 脚本；`npm install`（npm 11.13.0）移除 33 个包，lock 只剩 `@playwright/test`/`playwright`/`playwright-core` 1.63.0 与 `pngjs` 7.0.0，四者 `integrity` 与改前相同 |
| `docs/quickstart.md` | 第 47 行：`verify` 覆盖「以上全部，含 `css --check` 与 `catalog --check`」（按主 agent 将补的 `verify` 措辞）；第 5 行：`mise install` 的工具清单加上 Tailwind 独立 CLI 4.1.13 与 `RUSTIFY_TAILWIND`（本任务改动使原句不再完整；计划 §4 也把 `RUSTIFY_TAILWIND` 登记到 quickstart）。quickstart 中没有把 `npm ci` 写成生成 CSS 前提的文字，无需改 |

新增单测（xtask，7 项）：`tailwind::tests::{the_banner_names_the_version, only_the_pinned_version_is_accepted, mise_pins_the_version_this_checks_for}`；`css::tests::{the_first_differing_line_is_reported_with_both_sides, a_line_only_one_side_has_is_reported_against_the_end_of_file, a_long_line_is_cut_around_where_it_differs, the_input_compiles_only_the_sources_it_names}`。

### 实测事实（决定实现的依据）

- 独立二进制来源：`https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.13/tailwindcss-linux-x64`，同 release `sha256sums.txt` 中 `./tailwindcss-linux-x64` 为 `b9ed9f8f640d3323711f9f68608aa266dff3adbc42e867c38ea2d009b973be11`，下载文件 `sha256sum` 相同；安装到 `/root/.local/bin/tailwindcss`（0755），充当 mise 会提供的 `tailwindcss`。
- 横幅：`tailwindcss --help` 在 stdout 首行打印 `≈ tailwindcss v4.1.13`，退出 0；`FORCE_COLOR=1` 时为 `ESC[34mv4.1.13ESC[39m` 的着色形式，`NO_COLOR=1` 与之同时设置时不着色——故 `cli()` 设 `NO_COLOR=1`。`--version` 不是版本开关（会跑一次默认构建并把 CSS 打到 stdout），不用。Tailwind 3 的 help 横幅为 `tailwindcss v3.4.17`（npm 包 `lib/cli/help/index.js:22`：`` `${name} v${version}` ``），单测覆盖该形式。
- 失败语义：输入缺失、`@import` 解析失败、`@apply` 未知类时 CLI 退出 1，错误在 stderr（前面带横幅行）。
- `source(none)`：放在 `prefix(rui)` 前或后，产物都与已提交 `rustify.css` 逐字节相同（`cmp` 无输出）。在一个含 `<div class="rui:w-[123px]">` 的临时目录里以之为 cwd 构建：原输入产物含该类（2 处匹配，26,688 B），加 `source(none)` 后 0 处（26,644 B）——自动扫描确实以 cwd 为根，`source(none)` 关掉了它。
- 构建耗时（本机，各 3 次，墙钟）：原输入（仓库根为 cwd、自动扫描）0.82–0.95 s；`source(none)` 0.32–0.36 s；`xtask css --check`（已编译的 xtask，含 `--help` 版本探测与比较）0.53–0.56 s。

### 偏差与决定

- **`mise.toml` 未加 `platforms`/`asset_pattern`**：mise 在本容器装不上（见 A-1），只能读源码判断。读 mise `v2026.9.2`（与本机 `mise --version` 同版）的 `src/backend/asset_matcher.rs`（经 raw.githubusercontent.com 获取）：linux 未指定 libc 时目标 libc 默认 `gnu`（`AssetPicker::with_libc`，约第 220–227 行）；`score_libc_match` 对名字带 `musl` 的资产记 −10、不带 libc 标记的记 0（约第 553–565 行），且 libc 分只在 linux/windows 计入（`score_asset`，约第 446–458 行）；`pick_best_asset` 取最高分、同分取较短名（约第 329–358 行）。据此 linux x64 应选 `tailwindcss-linux-x64` 而非 `-musl`，macOS arm64 只有 `tailwindcss-macos-arm64` 一个 OS/arch 都匹配的资产。结论只来自读码，未实测，故按「无法实测不多猜」保持计划原样的最小配置；若 CI 实测误选，再按 A-1 备选加逐平台 `asset_pattern`。`bin = "tailwindcss"` 与 mise 对裸二进制自动去 OS/arch 后缀的行为重复，按 D10 显式保留。
- **计划说「两份输入」**：当前仓库只有 `crates/rustify-components/css/rustify.tailwind.css` 一份 Tailwind 输入；示例自带输入（`examples/<ex>/tailwind.css`）是 M5 的事，届时同样须带 `source(none)`。
- **`source(none)` 的单测放在 xtask**（`css::tests::the_input_compiles_only_the_sources_it_names`），因为 `crates/rustify-components/src/lib.rs` 的 stylesheet 测试模块不在本任务文件范围；M4/M5 若把「含 `source(none)`」并入 stylesheet 单测，可删去这一项。
- **`compile()` 已被 `css.rs` 调用**（`minify = false`），因此不存在 dead code，clippy `-D warnings` 无需任何 `allow`；`build-web` 在 M5 以 `minify = true` 调用。
- **node_modules**：`npm install` 移除包后留下 4 个空的 scope 目录（`@isaacs`、`@jridgewell`、`@parcel`、`@tailwindcss`），已 `rmdir`（未跟踪文件）。

### 缺口与继续动作

- **A-1 待 CI（macOS，`jdx/mise-action@v4`）核实**：本容器 `mise install` 失败——`HTTP status client error (403 Forbidden) for url (https://api.github.com/repos/tailwindlabs/tailwindcss/releases?per_page=100)`，代理返回「GitHub access to this repository is not enabled for this session」；Release 直链下载可用，但 mise 的 `github:` 后端必须先走 API 列资产。CI 上需确认：`mise which tailwindcss` 指向 mise 安装目录、`tailwindcss --help` 横幅为 `v4.1.13`、`mbx xtask css --check` 在 macOS arm64 上无漂移（即 macOS 版产物与 linux 版逐字节一致）。`mise ls` 在本机能解析该条目（`github:tailwindlabs/tailwindcss 4.1.13 (missing)`），配置语法本身无误。
- **CI host job 仍在 `css --check` 前跑 `npm ci`**（`.github/workflows/verify.yml`，不在本任务范围，按 §10 并行说明在 M1 合入后由主 agent 改）；lock 更新后 `npm ci` 仍可用。`mise-action` 在 host 与 web 两个 job 都会装上 Tailwind。
- **`verify` 加 `css --check`/`catalog --check`**：`xtask/src/verify.rs` 由主 agent 改；quickstart 第 47 行已按改后的事实措辞，在此之前该句超前于代码。
- **`CLAUDE.md`「Toolchain」仍只列 Rust 与 mbx**，未登记 `RUSTIFY_TAILWIND`（计划 §4 的登记点之一）；不在本任务范围，留给主 agent 或 M8。
- **§9.3 的 `build-web --example component-catalog` 故障注入**：M3 的 `build-web` 尚不调用 Tailwind（M5 才接入），本任务无从验证，留到 M5。

## 验收记录

环境：Linux x64（Ubuntu，glibc 2.39），rustc 1.98.1（rustup 默认），mbx 1.15.0，Node 26.1.0，npm 11.13.0，mise 2026.9.2；`tailwindcss` 4.1.13 独立二进制在 `/root/.local/bin`（sha256 见上）。cargo/mbx 命令一律带 `CARGO_TARGET_DIR=/root/tgt-m3`（与主 agent 的构建隔离）；下表 `X` 指该目录下编译好的 `/root/tgt-m3/debug/xtask`。代码基线同文首。

| 退出条件 | 命令 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| `css --check` 无漂移，且不依赖 npm 的 Tailwind | `ls node_modules/@tailwindcss node_modules/tailwindcss`；`mbx run -q -p xtask -- css --check` | `npm install` 之后 | 两个目录均不存在；`css: no drift (26644 bytes)`，exit 0 |
| 生成 CSS 不需要 Node | `env PATH=/root/.local/bin:/usr/bin:/bin sh -c 'command -v node; X css --check'` | 同上 | `node: not on PATH`；`css: no drift (26644 bytes)`，exit 0 |
| 重新生成的产物不变 | `mbx run -q -p xtask -- css`；`git diff --quiet crates/rustify-components/css/rustify.css` | 同上 | 产物无差异 |
| `source(none)` 产物逐字节相同 | `tailwindcss -i <输入> -o <out>`；`cmp <out> rustify.css`（`source(none)` 在 `prefix(rui)` 前、后各一次） | 独立二进制 4.1.13 | 两次均相同 |
| `source(none)` 关掉自动扫描 | 以含 `rui:w-[123px]` 的临时目录为 cwd，分别构建原输入与加 `source(none)` 的输入，`grep -c 123px` | 同上 | 原输入 2 处 / 26,688 B；`source(none)` 0 处 / 26,644 B |
| NFR-3 构建耗时 | 同一输入各跑 3 次，bash `time` | 同上 | 自动扫描 0.82–0.95 s；`source(none)` 0.32–0.36 s；`X css --check` 0.53–0.56 s |
| 新单测通过 | `mbx test -p xtask` | 工作树 | 31 passed，0 failed（含上述 7 项新单测） |
| 输入改动不破坏组件 crate 单测 | `mbx test -p rustify-components --lib` | 工作树 | 68 passed，0 failed |
| 静态校验 | `mbx clippy -p xtask --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 工作树 | clippy 无告警；fmt exit 0 |
| 故障注入①：PATH 上没有 `tailwindcss` | `env PATH=/usr/bin:/bin X css --check` | 工作树 | exit 1：``tailwindcss is not on PATH; run `mise install` for the tailwindcss 4.1.13 that mise.toml pins, or point RUSTIFY_TAILWIND at one`` |
| 故障注入①：二进制临时移出 PATH，走 `mbx run` | `mv /root/.local/bin/tailwindcss /root/.local/bin/tailwindcss.off`；`mbx run -q -p xtask -- css`；再移回 | 工作树 | exit 1，同上消息；二进制已恢复（`which tailwindcss` → `/root/.local/bin/tailwindcss`） |
| 故障注入①：`RUSTIFY_TAILWIND` 指向不存在的文件 | `RUSTIFY_TAILWIND=/nonexistent mbx run -q -p xtask -- css --check` | 工作树 | exit 1：``cannot run /nonexistent (from RUSTIFY_TAILWIND): No such file or directory (os error 2); point RUSTIFY_TAILWIND at tailwindcss 4.1.13, or unset it and run `mise install` for the one mise.toml pins`` |
| 故障注入②：4.1.12 假二进制被拒 | `RUSTIFY_TAILWIND=<打印 ≈ tailwindcss v4.1.12 的 sh 脚本> mbx run -q -p xtask -- css --check` | 工作树 | exit 1：``<脚本> (from RUSTIFY_TAILWIND) is tailwindcss v4.1.12, not v4.1.13; point RUSTIFY_TAILWIND at tailwindcss 4.1.13, or unset it and run `mise install` …`` |
| 补充：无横幅的二进制被拒 | `RUSTIFY_TAILWIND=<只打印 usage 的脚本> X css --check` | 工作树 | exit 1：``… printed no `tailwindcss v<version>` banner for --help, so its version is unknown; …`` |
| 补充：着色环境不误判 | `FORCE_COLOR=1 X css --check` | 工作树 | `css: no drift (26644 bytes)`，exit 0 |
| `--check` 打印首个差异行 | `RUSTIFY_TAILWIND=<包装脚本> mbx run -q -p xtask -- css --check`（包装脚本调真实 CLI，再把产物中第一个 `display: flex;` 改成 `display: grid;`） | 工作树（已提交产物未改） | exit 1：`(26644 bytes committed, 26644 bytes generated)` 后接 `first difference, line 63:` / `committed:     display: flex;` / `generated:     display: grid;`——字节数相同的漂移也能定位 |
| `doctor` 显示 Tailwind 版本 | `mbx run -q -p xtask -- doctor` | 工作树 | `ok    tailwind           tailwindcss v4.1.13 at /root/.local/bin/tailwindcss`；`chrome` FAIL（Linux 无 `/Applications`，预期），其余 ok |
| `doctor` 对缺失/错版本报 FAIL | `env PATH=/usr/bin:/bin:/opt/node26/bin X doctor`；`RUSTIFY_TAILWIND=<4.1.12 脚本> X doctor` | 工作树 | 两者 `FAIL  tailwind`，消息同故障注入①/② |
| `mise.toml` 条目可解析 | `mise ls`（仓库根） | mise 2026.9.2 | `github:tailwindlabs/tailwindcss  4.1.13 (missing)  /home/user/rustify-ui/mise.toml  4.1.13` |
| A-1：mise 为 macOS arm64 / linux x64 选中裸二进制并以 `tailwindcss` 暴露 | `mise install`（临时目录，仅含该条目） | 本容器 | **不可实测**：GitHub API 403（会话网络策略）。读码结论见「偏差与决定」；**待 CI（macOS，mise-action）核实** |
| CI host job 不再为 CSS 跑 `npm ci` 且全绿 | — | — | **未做**：`.github/workflows/verify.yml` 不在本任务范围，待主 agent 在 M1 之后修改并在 CI 上验证 |
