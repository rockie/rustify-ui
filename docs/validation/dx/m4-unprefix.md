# DX-REFINE M4 子任务记录：去掉 `rui:` 前缀

日期：2026-09-25。工作树 `.claude/worktrees/agent-abbbe976a35740721`，分支 `worktree-agent-abbbe976a35740721`。代码基线：`53356b9`（M1/M3 集成提交 `fa76b20` 合入 `origin/main` 的 `f19a0bd`；合并只带来 `.agents/`、`.claude/skills/` 与 `skills-lock.json`，计划文件取 `fa76b20` 一侧，与其逐字节相同）。

状态：本任务范围内的实现与本地验收已完成；**M4 未退出**——退出条件里的 component-catalog、property-workbench、data-workbench 回归层未跑（本任务不运行 Playwright 测试套件），两处测试字符串留给主 agent 在合入时同步（见文末）。计算样式对比的非 0 差异逐项列在「验收记录」，其中没有任何标准 CSS 属性。

## 实现记录

### 交付行为

| 文件 | 改动 |
| --- | --- |
| `crates/rustify-components/src/**`、`examples/component-catalog/src/main.rs`、`examples/data-workbench/src/main.rs` | 脚本去掉 `rui:`，逐文件断言命中数（下表），`cargo fmt` 后复查三处路径 `rui:` 残留为 0 |
| `crates/rustify-components/css/rustify.tailwind.css` | 两个 Tailwind 导入去掉 `prefix(rui)`，utilities 保留 `source(none)`；新增 `@theme inline reference { --spacing: 0.25rem; }`（见偏差 1）；作用域默认令牌 `--primary`/`--ring` `#2e90fa` → `#1570ef`、`--border` `#d0d5dd` → `#858f9e`，逐个核对其余 20 个与 `Theme::light()` 相同；头注释「prefixed utility」改为「utility, which selects a class」 |
| `crates/rustify-components/css/rustify.css` | `mbx xtask css` 重新生成：26,644 → 28,935 B；类选择器 197 → 221（去掉 `rui\:` 后原有 197 个全在，新增 24 个见偏差 5）；`:root` 上的主题变量由 `--rui-*` 变为无前缀名，且不再有 `--spacing`/`--rui-spacing` |
| `crates/rustify-components/src/macros/mod.rs` | `merge` 的文档去掉「前缀不在此配置」一段，改为说明无前缀后调用方类按同类同状态替换组件类；删除 `an_unprefixed_class_does_not_collide_with_a_prefixed_one`；`a_variant_makes_two_classes_different_even_with_the_prefix` 改名 `a_variant_makes_two_classes_different`，只保留「不同状态不冲突」断言；新增 `a_callers_padding_replaces_the_components`（`p-6` 替换 `p-4`）与 `a_callers_class_in_a_state_replaces_the_components_in_that_state`（`hover:bg-secondary` 替换 `hover:bg-primary/90`，非 hover 的 `bg-primary` 保留） |
| `crates/rustify-components/src/lib.rs`（`stylesheet` 测试模块） | `every_token_the_sdk_writes_is_a_token_the_stylesheet_names`（只比名字）换成 `the_scope_defaults_are_the_light_theme`：解析输入里 `[data-rustify-scope]` 块，23 个令牌逐个与 `Theme::light().properties()` 比值，且块内不多不少；新增 `the_tokens_are_written_only_to_a_scope_root`（产物主题层不声明任何令牌名）、`a_utility_never_reads_the_scopes_spacing`（产物无 `var(--spacing)`）、`the_utilities_are_the_unprefixed_ones_an_application_writes`（输入无 `prefix(`、产物无 `.rui\:`）；`the_input_compiles_only_the_sources_it_names`（utilities 导入含 `source(none)`）从 `xtask/src/css.rs` 移入 |
| `xtask/src/css.rs` | 删去已并入 stylesheet 单测的 `source(none)` 用例（M3 记录已预告） |
| `CLAUDE.md` | Conventions「Component classes」：无前缀 Tailwind v4 utility，与应用写的相同，调用方 `class` 经 `tw_merge` 替换同类组件类 |
| `docs/quickstart.md` | 「Changing a component's classes」：去掉前缀说明；写明 `class` 合并规则（`p-6` 替换 `p-4`、`hover:` 同状态替换），保留的三条隔离（无 preflight/裸选择器、令牌只写作用域根、`dark` 绑定作用域），以及不再承诺与宿主自带 Tailwind 共存 |
| `examples/component-catalog/index.html` | 宿主控件注释去掉「the utilities are prefixed」（只改注释） |
| `docs/validation/dx/probes/style-breakdown.mjs`（新） | 按属性汇总两份计算样式快照的全部差异；前缀快照的 `--rui-x` 在元素没有自有 `--x` 时按 `--x` 比较，并先列出所有按改名比较的名字 |

### 命中数（脚本断言，改写前实测 = 计划 §1.1）

| 文件 | 命中 |
| --- | --- |
| `crates/rustify-components/src/button.rs` | 52 |
| `crates/rustify-components/src/checkbox.rs` | 33 |
| `crates/rustify-components/src/data_table/view.rs` | 61 |
| `crates/rustify-components/src/dialog.rs` | 37 |
| `crates/rustify-components/src/drop_zone.rs` | 17 |
| `crates/rustify-components/src/file_picker.rs` | 19 |
| `crates/rustify-components/src/form/field.rs` | 5 |
| `crates/rustify-components/src/form/submit.rs` | 2 |
| `crates/rustify-components/src/icon.rs` | 3 |
| `crates/rustify-components/src/input.rs` | 22 |
| `crates/rustify-components/src/label.rs` | 8 |
| `crates/rustify-components/src/link.rs` | 9 |
| `crates/rustify-components/src/macros/clx.rs` | 4 |
| `crates/rustify-components/src/macros/mod.rs` | 39 |
| `crates/rustify-components/src/macros/variants.rs` | 13 |
| `crates/rustify-components/src/menu.rs` | 28 |
| `crates/rustify-components/src/progress.rs` | 10 |
| `crates/rustify-components/src/radio.rs` | 37 |
| `crates/rustify-components/src/scroll_area.rs` | 8 |
| `crates/rustify-components/src/select.rs` | 51 |
| `crates/rustify-components/src/slider.rs` | 1 |
| `crates/rustify-components/src/spinner.rs` | 4 |
| `crates/rustify-components/src/switch.rs` | 24 |
| `crates/rustify-components/src/tabs.rs` | 45 |
| `crates/rustify-components/src/textarea.rs` | 21 |
| `crates/rustify-components/src/tooltip.rs` | 9 |
| `crates/rustify-components/src/tree.rs` | 22 |
| `crates/rustify-components/src/workspace/command_palette.rs` | 41 |
| `crates/rustify-components/src/workspace/panel_tabs.rs` | 34 |
| `crates/rustify-components/src/workspace/splitter.rs` | 14 |
| **`crates/rustify-components/src` 合计** | **673** |
| `examples/component-catalog/src/main.rs` | 42 |
| `examples/data-workbench/src/main.rs` | 1 |

脚本先统计三处路径下每个文件的 `rui:` 数，与上表逐文件、逐路径合计比对，任何不符即退出且不写文件；全部相符才替换。命中含 `macros/{clx,variants}.rs` 文档示例与 `macros/mod.rs` 注释/测试里的 `rui:`；`macros/mod.rs` 的注释与前缀测试随后手工改写（上表「交付行为」）。`cargo fmt` 重排了部分行（改写 + fmt 后 `git diff` 为 32 个文件 154+/158−），之后 `grep -rn 'rui:'` 三处路径为 0。

### 偏差与决定

1. **`--spacing` 用 `@theme inline reference`，不放进颜色所在的 `@theme inline` 块。** 放进 `@theme inline` 时，utility 确实内联为 `calc(0.25rem * 4)`，但产物主题层同时写出 `:root, :host { --spacing: 0.25rem }`：Tailwind 会为扫描到的源里出现的变量名保留主题变量，而 SDK 输入此时仍扫 `examples/`，`examples/property-workbench/app.css:19,103` 写着 `var(--spacing, 8px)`。`--spacing` 是作用域令牌名，写到 `:root` 就让作用域外读 `var(--spacing, 8px)` 的宿主规则拿到 0.25rem，违背 NFR-5 保留的「令牌只写作用域根」。`reference` 让该值只参与内联、从不输出（隔离目录实测：同一源下 `inline` 输出 `--spacing`，`inline reference` 不输出，utility 都是 `calc(0.25rem * n)`）。单测 `the_tokens_are_written_only_to_a_scope_root` 钉住这一点。
2. **`source(none)` 单测并入 stylesheet 模块**（M3 记录「偏差与决定」第 3 条允许）；xtask 单测 31 → 30，组件 crate 单测 68 → 73。
3. **新 `merge` 用例用 `hover:bg-secondary` 而不是新类名。** SDK 输入扫描 `../src` 全部文本，测试里的类字符串也会编进产物（原 `rui:hover:bg-secondary` 规则就只来自旧测试）。先写 `hover:bg-accent` 时产物多出 `.hover\:bg-accent`、少了 `.hover\:bg-secondary`；改用已有的类后，产物相对去前缀前的类集合 0 删除。
4. **格式检查用 `cargo fmt -- --check`，不是 `cargo fmt --all -- --check`。** 本工作树嵌在主检出 `/home/user/rustify-ui` 之内：`--all` 还会对 `makepad/widgets` 等路径依赖单独跑 `cargo metadata`，Cargo 向上找到主检出的 `Cargo.toml` 当作其工作区并报「believes it's in a workspace when it's not」。不带 `--all` 覆盖全部工作区成员；`makepad/` 本就由它自己的 `rustfmt.toml` 退出格式化，覆盖面相同。`cargo metadata` 与 `mbx build`/`build-web` 不受影响。
5. **（发现）源文本里的普通单词现在会编成规则。** 前缀在时，只有写成 `rui:x` 的词才是候选；去掉后，`@source` 目录里任何像类名的词都是。产物新增 24 个无组件使用的 utility：来自 `crates/rustify-components/src` 注释/标识符的 13 个——`container contents grid grow inline invisible outline resize ring shrink static table visible`（如 `data_table/view.rs:20`「One column of the table」、`macros/mod.rs` 的 `let outline = …`、`catalog.rs` 的「the focus ring」）；只来自 `examples/` 的 11 个——`blur capitalize collapse filter italic line-through lowercase shadow transform underline uppercase`，M5 删掉 examples 源后消失（隔离构建实测）。它们带来 19 个新的 `@property` 注册（`filter`/`blur`/`transform` 系与 `--tw-outline-style`），计算样式里可见的是其中有初值的两个（见验收记录）。示例自身的类名与产物求交（抽取 `class="…"`、`class:x`、`classList.*`、`className` 与各示例 CSS 的类选择器）：component-catalog 11 个、data-workbench 1 个全是它们本来就写的 utility（原带 `rui:`）；fusion-basic、property-workbench 0 个；vellum 只有 `.hidden`（不链接 `rustify.css`，同计划 §1.1）；`web/runtime.css`/`loader.js` 的类名 0 个。未加屏蔽：`@source not inline(...)` 是随注释改动失效的手写清单，放进 M5 的共享 `sdk.css` 还会让应用自己写的 `grid`/`table` 没有规则。是否屏蔽留给主 agent/用户定。
6. `tests/`、`playwright.config.ts`、各示例 `app.js` 与示例 Rust 的非类字符串部分未改。

### 留给主 agent 的测试字符串

| 位置 | 现文本 | 应改为 |
| --- | --- | --- |
| `tests/browser/p3-table.spec.ts:31` | `row.classList.contains("rui:hidden")` | `row.classList.contains("hidden")` |
| `tests/browser/p2-catalog.spec.ts:64-67` | `"rui\\:bg-card"`、`"rui\\:bg-success"`、`"rui\\:bg-warning"`、`"rui\\:text-muted-foreground"` | 去掉 `rui\\:`（计划去重表在 M2 删除 `p2-catalog.spec.ts:57-74` 整段；若 M2 已删则无需改） |

## 验收记录

环境：Linux x64，rustc 1.98.1，mbx 1.15.0，`tailwindcss` 4.1.13 独立二进制（`/root/.local/bin`），Node 26，Playwright 库（`node_modules` 以符号链接借用主检出）+ `/root/pw-compat` 的 Chromium。`X` 指本工作树 `target/debug/xtask`。代码基线：`53356b9` + 本提交的改动。

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 命中数断言 | `python3 unprefix.py`（一次性脚本，在会话草稿目录、未入库；逻辑见上一节） | 32 个文件逐个相符；673 / 42 / 1 |
| fmt 后无残留 | `cargo fmt`；`grep -rn 'rui:' crates/rustify-components/src examples/component-catalog/src examples/data-workbench/src \| wc -l` | 0 |
| 组件 crate 单测 | `mbx test -p rustify-components --lib` | 73 passed |
| host 单测 | `mbx test --workspace --lib --bins` | 全过：component-catalog 5、data-workbench 39、fusion-basic 0、property-workbench 7、rustify-components 73、rustify-makepad 3、rustify-ui 133、vellum lib 137 / bin 5、xtask 30 |
| clippy | `mbx clippy --workspace --all-targets -- -D warnings` | 无告警 |
| fmt | `cargo fmt -- --check`（偏差 4） | exit 0 |
| SDK 样式表无漂移 | `X css --check` | `css: no drift (28935 bytes)` |
| 目录表无漂移 | `X catalog --write docs/components.md --check` | `catalog: no drift (20 rows)` |
| 源锁 | `X sources verify` | exit 0；`rust_ui … 20 files, 0 verbatim, 20 rewritten`（去前缀不需改锁，同计划 §1.1） |
| release 构建 | `X build-web --example component-catalog --release` | 成功；size report css 35,063 B；产物 `rustify.css` 与提交版逐字节相同 |
| 计算样式快照 | `X serve --example component-catalog --release --port 4196`；`node docs/validation/dx/probes/computed-style.mjs snapshot http://127.0.0.1:4196/ target/dx-style/after.json` | 44 个页面状态、5,648 个元素快照（与改前相同，DOM 路径一一对应） |
| 计算样式对比（原样） | `node docs/validation/dx/probes/computed-style.mjs compare /home/user/rustify-ui/target/dx-style/before.json target/dx-style/after.json` | 2,638,744 个值比较，**139,804 个不同**，全部是自定义属性（分解见下） |
| 计算样式对比（按属性分解） | `node docs/validation/dx/probes/style-breakdown.mjs <before.json> <after.json>` | 按改名比较 11 个主题变量后：2,576,616 个值比较，15,548 个不同，涉及 3 个自定义属性；**标准 CSS 属性差异 0**；0 个元素只在一侧 |

计算样式差异逐项（139,804 = 124,256 + 5,648 + 5,648 + 4,252）：

| # | 属性 | 数量 | 前 → 后 | 来源 | 判定 |
| --- | --- | --- | --- | --- | --- |
| 1 | `--rui-container-lg`、`--rui-default-transition-duration`、`--rui-default-transition-timing-function`、`--rui-font-weight-medium`、`--rui-font-weight-semibold`、`--rui-text-lg`、`--rui-text-lg--line-height`、`--rui-text-sm`、`--rui-text-sm--line-height`、`--rui-text-xs`、`--rui-text-xs--line-height` | 11 × 5,648 × 2 = 124,256 | `--rui-x: v` 消失、`--x: v` 出现，值逐一相同 | Tailwind 主题变量名随前缀改变（ADR-2 负面后果「`:root` 上出现无前缀的 Tailwind 主题变量」） | 预期；按改名比较后 0 差异 |
| 2 | `--rui-spacing` | 5,648 | `0.25rem` → 无 | D12：间距 utility 内联为 `calc(0.25rem * n)`，不再读变量；`reference` 不把 `--spacing` 写到 `:root`（偏差 1）。作用域的 `--spacing: 8px` 前后相同 | 预期 |
| 3 | `--tw-drop-shadow-alpha` | 5,648（全部元素） | 无 → `100%` | 偏差 5：`examples/` 文本里的 `filter`、`blur` 生成 `.filter`、`.blur`，随之注册 filter 系 `@property`，其中 `--tw-drop-shadow-alpha` 有初值（`inherits: false; initial-value: 100%`）；没有元素带这两个类，`filter` 等标准属性 0 差异 | 无视觉影响；M5 删 examples 源后 SDK 产物不再有 |
| 4 | `--tw-outline-style` | 4,252（4,070 无 → `solid`；182 `none` → `solid`） | 见左 | 偏差 5：注释/标识符里的 `outline` 生成 `.outline`，注册 `@property --tw-outline-style { inherits: false; initial-value: solid }`。改前它未注册，`outline-none` 写的 `none` 按普通自定义属性继承给后代（即那 182 个）；注册后不继承、后代取初值。产物里读 `var(--tw-outline-style)` 的只有 `.outline`，没有元素带它，`outline-style` 等标准属性 0 差异 | 无视觉影响；来自 SDK 源，M5 后仍在 |

作用域默认令牌改值（`--primary`/`--ring`/`--border`）在快照里没有差异：作用域在挂载时把 `Theme::light()`/`dark()` 的值写到自己的根上，覆盖默认值（`crates/rustify-ui/src/theme.rs`），而探针只记录作用域内的元素。

未验证（M4 退出条件，留给主 agent）：component-catalog、property-workbench、data-workbench 回归层；`p3-table.spec.ts:31` 与 `p2-catalog` 字符串同步后的对应用例。
