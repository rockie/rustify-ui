# DX-REFINE M7 子任务记录：目录新组件与 Menu/Dialog 扩展

日期：2026-09-25。工作树 `/home/user/rustify-ui-m7`（detached HEAD），代码基线 `b29a778`（含 M6 的 SDK 原语提交 `12eae0d`、示例改用 `ba960e9` 与 M4 去前缀 `1ca403e`）。

状态：本任务范围（M7 除 Vellum 改用）内的实现与本地验收完成。**M7 未退出**：Vellum 改用 Toast/草稿核心、Vellum 回归层与基线孪生 `visual.spec.ts` 由另一 agent 负责，本记录不覆盖；计划「实施进度」由主 agent 回写。

## 实现记录

### 公开接口

| 出口 | 签名 / 行为 | 目标 |
| --- | --- | --- |
| `rustify_components::Toaster` | `Toaster { class, test_id }`：在 SDK `ToastRegion` 上用 `render` 闭包画样式卡片；区域 `fixed bottom-4 end-4 z-50`，`role=status` + `aria-live=polite` 由 SDK 核心给出，不进 `Layer` 栈；卡片 test id 为 `<test_id>-toast` | wasm |
| `rustify_components::Toast` | `Toast { toast: rustify_ui::Toast, class, test_id }`：消息 + 关闭按钮；`data-tone` 与 `border-s-4` 色边只随 `tone` 变样式；关闭按钮可访问名取 `Message::Dismiss`（随作用域语言），键盘关闭时焦点留在作用域容器而不落到 `body` | wasm |
| `rustify_components::NumberField` | `value: Signal<f64>`、`on_change(f64)`（提交请求）、可选 `on_preview: Callback<f64>`、`on_cancel: Callback<()>`、`min`/`max`/`step`、`id`、`aria_label`、`disabled`、`read_only`、`invalid`、`described_by`、`class`、`test_id`；建在 `rustify_ui::Draft<f64>` 上；`role=spinbutton` + `aria-valuenow/min/max`；草稿不可解析时 `aria-invalid`；↑/↓ 请求 ±`step`、PageUp/PageDown 请求 ±10×`step`，先按 Enter 的方式结束已输入的草稿，再从应用值起步；不夹取（夹取归应用）；disabled/read-only 不发任何请求 | wasm |
| `number_field::{parse_number, format_number, stepped}` | 纯函数：有限数解析（拒 `inf`/`NaN`/`1e`/`-`）；最短回读格式、不写 `-0`；按两者小数位取整的步进（`0.1 + 0.2 = 0.3`） | 全平台 |
| `rustify_components::ColorField` | `value: Signal<String>`、`on_change(String)`、可选 `on_preview: Callback<String>`、`on_cancel: Callback<()>`、`alpha: bool`、`id`、`aria_label`、`disabled`、`read_only`、`invalid`、`described_by`、`class`、`test_id`；建在 `Draft<String>` 上；旁边色块 `<test_id>-swatch` 为 `aria-hidden` 装饰，经 `style:background-color`（CSSOM）上色，无 `style` 属性（C-3） | wasm |
| `color_field::parse_hex(text, alpha)` | 沿用 `examples/vellum/src/shell/fields.rs:118-135` 的规则：去空白与 BOM，`#` 可省，`#rgb` 展开为 `#rrggbb`；`alpha` 时另收 `#rgba`/`#rrggbbaa`；输出小写。与 Vellum 不同处：不收 `none`（Vellum 的填充语义），输出统一小写 | 全平台 |
| `rustify_components::{ToggleGroup, ToggleItem}`、`toggle_group::pressed_after` | `value: Signal<Vec<String>>`、`on_change(Vec<String>)`（请求按下后的集合）、`items: Signal<Vec<ToggleItem>>`、`multiple`、`toolbar`、`orientation`、`disabled`、`aria_label`、`class`、`test_id`；按钮 `aria-pressed`；容器 `role=group`，`toolbar` 时 `role=toolbar` + `aria-orientation`；单一 tab stop（最近聚焦 → 首个按下 → 首个可达），方向键/Home/End 经 `roving.rs` 漫游并跳过禁用项；单选时再按已按下者请求空集，多选时按按钮顺序返回集合 | 全平台（组件无 wasm 依赖） |
| `rustify_components::{MenuItem, MenuItemKind}` | `MenuItem::heading(label)`、`MenuItem::separator()`、`.shortcut(Shortcut)`、`.reachable()`；新字段 `kind`、`shortcut`（既有调用方只用构造器，无破坏） | 全平台 |
| `rustify_components::{Menu, anchor_at}` | 标题开一个 `role=group`（`aria-labelledby` 指向 `aria-hidden` 的标题），分隔线为 `role=separator`，二者都不可聚焦、方向键跳过；快捷键列 `<kbd aria-hidden>` 显示 `Shortcut` 的 Display，菜单项带同值 `aria-keyshortcuts`；`Layer` 以 `fit=true` 放置（见 SDK 改动）；`anchor_at(element, client_x, client_y)` 给出「在点上打开、随元素滚动」的锚 | wasm |
| `rustify_components::Dialog` | 标题栏 `DialogTitleBar`：标题/描述与关闭按钮同一行；关闭按钮 `aria-label`/`title` 取应用的 `close_label`，否则 `Message::Close`（随作用域语言）；test id 为 `<test_id>-close`（目录里仍是 `dialog-close`）；按钮与 Escape 调同一个 `on_open_change(false)` | wasm |

### SDK 改动（最小、增量，不改既有行为）

| 文件 | 改动 | 理由 |
| --- | --- | --- |
| `crates/rustify-ui/src/overlay.rs` | `LocalRect::below_within(size, viewport)` 纯函数（右侧放不下左移、下方放不下翻到锚上方、都放不下贴底，左上角永不出视口）+ 4 个 host 单测；`Layer` 新增可选 `fit: bool`，只在为真时用它，视口取 `documentElement.clientWidth/Height`（不含滚动条），尺寸取 `getBoundingClientRect`（避免取整后贴边换行） | 定位属于 overlay 栈（`rustify-components/src/lib.rs` 注释「positioning … are its」）；组件拿不到栈的 `moved`（`pub(crate)`），在组件里二次测量会在滚动/缩放后失效。默认关闭，Select/Tooltip/Vellum 等既有调用方行为不变 |
| `crates/rustify-ui/src/draft.rs` | `Draft::commit()`：与 Enter 同一路径结束编辑（Enter 分支改为调用它） | 数字输入的方向键在发自己的请求前必须先结束已输入的草稿，否则草稿会比它所描述的值活得更久；此前只能伪造 `KeyboardEvent` 调 `on_keydown` |

未新增 SDK 文案：Toast 关闭按钮用 M6 已加的 `Message::Dismiss`，Dialog 关闭按钮用既有 `Message::Close`；`docs/i18n.md` 补一段说明目录组件哪些字是 SDK 的、哪些是应用的。

### 目录、样式与文档

- `crates/rustify-components/src/catalog.rs`：`Category` 加 `Toast`/`NumberField`/`ColorField`/`ToggleGroup`，`CATALOG: [Entry; 24]`；四行新条目（均无 GPU 半边，环境格为 partial 并说明 region 如何取值）；Menu 行（属性/主题/输入/可访问性/环境）与 Dialog 行（属性/动作）按扩展改写，Menu 环境格从「does not flip or clamp」改为「moves left or opens above rather than leave the viewport」；host 测试 `ALL` 与计数 20 → 24。
- `docs/components.md`：`mbx xtask catalog --write` 重新生成表格；正文「twenty」→「twenty-four」、「Six categories have no GPU half」→「Ten … four different reasons」，并加一段草稿例外说明。
- `crates/rustify-components/css/rustify.css`：`mbx xtask css` 重新生成，28,935 → 31,599 B（新增 `end-4`、`bottom-4`、`z-50`、`border-s-*`、`aria-pressed:*`、`list-none`、`m-0`、`tracking-widest`、`tabular-nums`、`font-mono` 等）。
- `CLAUDE.md`：目录「twenty」→「twenty-four」；运行时契约「Controlled values」加一句草稿例外。
- `docs/architecture.md`：两处「twenty」→「twenty-four」；「Controlled values」下加 ADR-4 例外：只在聚焦的文本类输入内、未输入时跟随应用值、输入后可解析即预览请求、Enter/失焦一次提交、Escape 撤回，失焦即回到受控。
- `tests/browser/support.ts`：`CATALOG_SIZE` 20 → 24。

### 示例

- `examples/component-catalog/src/main.rs`：四个新类别页 + Menu/Dialog 扩展，接线同既有类别（应用持有值，`FirstLoad` 登记首载值，`catalog_reset` 一并复位）。
  - `Values` 新增 `dialog_closes`、`command`、`context`（`Option<Anchor>`）、`number`/`colour`/`tint`（`Edited<T>`：值 + 首次预览前的值，供 cancel 还原）、`requests`（草稿字段发出的全部请求，含被拒的）、`align`、`styles`、`tone`、`shown`；快照同名字段，另有 `toast`（当前消息或 `null`）。
  - 数字页：界 0–100、步 5；预览越界即拒（草稿保留），提交时夹取；「set it to 25 from elsewhere」按钮 `mousedown` 阻止默认，点击不夺焦点，用于演示「聚焦但未改动的草稿跟随外部值」。
  - 颜色页：`a colour`（不收 alpha）与 `a colour with opacity`（收 alpha）两个字段 + 同类外部改值按钮。
  - 切换组页：单选 `alignment`（应用拒绝空集）与多选工具栏 `text style`（`strike` 禁用）。
  - Toast 页：语气单选切换组（拒绝空集）、`show a message`（3.2 s）、`show one that goes quickly`（1.2 s）；`Toaster` 放在 `Catalogue` 根，`provide_toasts()` 在作用域根，复位时关掉当前消息。
  - Menu 页：标题「on this page」/「on a selection」、分隔线、两条带快捷键的命令（`Alt+Shift+F`/`Alt+Shift+S`，页面用 `rustify_ui::listen` + `Shortcut` 在作用域容器上真的响应，文本输入内不响应）；新增右键区 `default-context-area`：右键在点上打开 `context-menu`，上下文菜单键或 Shift+F10 以区域为锚打开。
  - Dialog 页：`on_open_change(false)` 计入 `dialog_closes`。
  - `catalog_region.rs`：四个新类别落到 note slot。
- `examples/property-workbench/src/main.rs` 右键菜单：两条分隔线把「next/lock」「theme」「all commands」分成三组；新增 `all commands` 项，快捷键提示为既有的 `Mod+K`（与作用域监听同一个 `Shortcut` 值），激活后经 `rustify_ui::defer` 打开命令面板（等菜单关闭并归还焦点后再开，避免两层抢焦点）；`region-menu` 与 `menu-item-*` test id 不变。

### 偏差与行为变化

1. **Menu 面板去掉 UA 列表边距**：作用域不做 reset，`ul` 带 1em 上下外边距与列表标记，既有菜单因此比锚点低 15 px（本次调试 `boundingBox` 发现：层 121 px 高、面板 91 px）。`PANEL` 加 `m-0 list-none`、分组 `ul` 加 `m-0 p-0` 后，菜单贴锚点打开。这是既有 Menu 与 `region-chooser` 的可见位置变化（上移 15 px），property-workbench 与 component-catalog 回归层均全绿。
2. **Dialog 关闭按钮移入标题栏**：DOM 顺序在内容之前，打开模态时首个可聚焦（`focus_first`）从内容里的第一个控件变为关闭按钮。`p2-a11y`/`p2-semantics`/`p2-catalog` 的对话框用例不受影响，新 spec 断言了这一点。
3. **Toast 键盘关闭后的焦点**：按钮移除自身会让焦点落到 `body`（走查首轮如此）；改为关闭前若焦点在按钮上则聚焦作用域容器（与 overlay 栈关层且无处可回时的 fallback 相同）。
4. **颜色解析**：不收 Vellum 的 `none`，输出小写（受控值的规范形式）。
5. **快捷键提示文本**：按任务要求用 `Shortcut` 的 Display（`Control+K`/`Meta+K`），不做 ⌘ 等平台符号化。

### 新增测试

- host：`rustify-ui` +4（`overlay::a_fitted_layer_*` 四项）；`rustify-components` +15（`menu` 4、`number_field` 4、`color_field` 4、`toggle_group` 3），目录计数测试改为 24。
- 浏览器（component-catalog）：`tests/browser/catalog-inputs.spec.ts` 24 项，已加入 `playwright.config.ts` 的 `component-catalog` `testMatch`，用共享页 fixture（`import { test } from "./support"`）：
  - 数字（7）：spinbutton 名/值/界；聚焦草稿在应用拒绝时保留、失焦提交并由应用夹取；不可解析草稿 `aria-invalid`、不发请求、Enter 回到应用值；Escape 撤回并请求 cancel；未改动草稿跟随外部改值、改动过的草稿保留到失焦；方向键/PageUp 步进与夹取、先结束已输入草稿；disabled/read-only 不发请求。
  - 颜色（4）：`#rgb`/`#rrggbb` 与色块 CSSOM 着色；alpha 只在允许时有效；中间态保留、Escape 撤回、未改动时跟随外部；disabled/read-only 不发请求。
  - 切换组（3）：单选 `aria-pressed` 由应用决定、拒绝空集；单一 tab stop、方向键/Home/End 漫游跳过禁用、Space/Enter 按下、Tab 回到上次位置；禁用组。
  - Toast（3）：`role=status`/`aria-live=polite`、新消息替换旧消息（序号递增、tone 只改样式）；自动隐藏、被替换消息的计时器不影响后继、键盘关闭后焦点留在作用域、关闭按钮名随语言；Escape 不关 Toast、模态打开时区域不在 `[inert]` 内。
  - Menu（5）：标题命名分组、分隔线、键盘跳过二者；快捷键列 `aria-keyshortcuts` 与 `aria-hidden` 提示、页面响应快捷键；右键在点上打开、贴近右下角时左移并翻到点上方且整体在视口内；触发器下方无空间时在上方打开；Shift+F10 以区域为锚打开并归还焦点。
  - Dialog（1）：关闭按钮名随语言、为首个焦点；按钮与 Escape 各发一次同一关闭请求，应用自己的 accept 不算关闭请求。
  - 亮/暗（1）：数字输入底色与边框、按下的切换按钮、Toast 卡片、菜单分隔线、Dialog 面板在两种主题下分别等于对应令牌值。
- 浏览器（property-workbench）：`tests/browser/p2-workspace.spec.ts` 新增 1 项：右键菜单 2 条分隔线且键盘跳过、`all commands` 的 `aria-keyshortcuts` 为 `Control+K`/`Meta+K` 且提示 `aria-hidden`、End + Enter 打开命令面板并由其持有焦点。

### 键盘走查（自动化，非人工）

无人值守，按 §6 要求以 Playwright 键盘脚本代走：脚本 `walkthrough.cjs`（会话 scratchpad，未入库）对 release 构建（`target/debug/xtask serve --example component-catalog --release --port 4186`）从导航按钮 Enter 进入各页、Tab 进入控件，逐键记录焦点元素（test id、role、`aria-pressed`/`aria-invalid`/`aria-valuenow`、输入值）与应用快照；先亮色走四页，`reset()` 后在 `toggle-theme` 上按 Enter 切到暗色再走一遍。每种主题 77 步，亮/暗逐步比较焦点与状态（去掉 Toast 序号）**0 处差异**。摘要（亮/暗相同）：

| 页 | 按键 | 结果 |
| --- | --- | --- |
| number field | Tab×5 | 经 `nav-color-field`…`nav-samples` 到 `default-number`（值 40） |
| | ↑ / ↓ / PageUp | 45 / 40 / 90，各一次 `commit` |
| | Ctrl+A，逐字输入 `120` | 预览 1、12（接受）、120（拒绝）；输入框保留 `120`，应用值 12 |
| | Enter | `commit 120` → 应用夹到 100，输入框显示 100 |
| | Ctrl+A，输入 `-` | `aria-invalid=true`，无请求 |
| | Escape | 回到 100，无请求 |
| | Ctrl+A，输入 `35`，Escape | 预览 3、35 后 `cancel`，回到 100 |
| | Tab，Enter | 焦点到「set it to 25 from elsewhere」，应用值 25 |
| | Shift+Tab | 回到数字框，显示 25（未改动草稿跟随外部值） |
| color field | Tab×4 | 到 `default-colour`（`#1570ef`） |
| | Ctrl+A，输入 `#abc`，Enter | 预览并提交 `#aabbcc` |
| | Ctrl+A，输入 `#1f6feb80` | 途中 `#1f6`、`#1f6feb` 可解析即预览；最终 8 位不收 alpha → `aria-invalid=true`，应用停在最后一次预览 `#1f6feb` |
| | Escape | `cancel`，回到 `#aabbcc` |
| | Tab，Ctrl+A，输入 `#1f6feb80`，Tab | 到 `default-tint`；`#1f6f` 按 `#rgba` 预览为 `#11ff66ff`，最终提交 `#1f6feb80` |
| toggle group | Tab×3 | 到 `default-align-start`（`aria-pressed=true`） |
| | → / Space / Space | 焦点 centre；按下 centre；再按请求空集被拒，仍按下 |
| | End / → | 到 end；回绕到 start（只移焦点，不改选择） |
| | Tab / → / → | 到工具栏 bold；italic；跳过禁用的 strike 到 underline |
| | Enter / Home / Space | underline 按下；回到 bold；bold 取消（`styles=["underline"]`） |
| | ← / Shift+Tab / Tab | 回绕到 underline；回到对齐组的 stop（start）；再进工具栏落在 underline |
| toast | Tab×6，→，Space | 到语气组 neutral；焦点 success；选中 success |
| | Tab，Enter，Enter | 「show a message」：message 1，再按被 message 2 替换，`data-tone=success` |
| | Escape | 消息仍在 |
| | Tab，Enter，等 1.6 s | 「show one that goes quickly」：message 3 出现并自行消失 |
| | Shift+Tab，Enter，Tab×2，Enter | message 4；Tab 经过「quickly」到 `toaster-toast-dismiss`（名「dismiss this message」）；Enter 关闭，焦点到作用域容器 `#catalog` |

## 验收记录

| 退出条件 | 命令或步骤 | 环境 / 代码基线 | 结果 |
| --- | --- | --- | --- |
| host 单测 | `mbx test --workspace --lib --bins` | Linux x64，Rust 1.98.1，本提交 | 通过：rustify-ui 163（基线 159）、rustify-components 88（基线 73）、component-catalog 5、data-workbench 39、property-workbench 7、rustify-makepad 3、vellum lib 137 + bin 5、xtask 30、fusion-basic 0 |
| clippy | `mbx clippy --workspace --all-targets -- -D warnings` | 同上 | 通过（exit 0）。另跑 wasm 目标 `mbx clippy -p rustify-components -p component-catalog --target wasm32-unknown-unknown`：新代码无告警，余下均为基线既有（`overlay.rs` `fallback_element`、`router.rs`、`text.rs`、`theme.rs`、`region.rs`、`gpu/lists.rs`、`catalog_region.rs:638`） |
| 格式 | `cargo fmt --all -- --check` | 本工作树 | 通过（exit 0；本嵌套工作树下 `--all` 可用） |
| 样式表漂移 | `mbx xtask css --check` | 同上 | 通过 |
| 目录文档 | `mbx xtask catalog --write docs/components.md --check` | 同上 | `catalog: no drift (24 rows)` |
| 新 spec | `flock /tmp/pw.lock npx playwright test --project=component-catalog catalog-inputs.spec.ts` | release 构建 | 24/24 通过（33.4 s） |
| component-catalog 回归层 ×2 | `flock /tmp/pw.lock npx playwright test --project=component-catalog --workers=2` | 最终 release 构建 | 第 1 次 73/73，2.1 min（墙钟 128 s，含起服务）；第 2 次 73/73，2.1 min（130 s）。73 = 基线 49 + 本 spec 24 |
| property-workbench 回归层 ×1 | `flock /tmp/pw.lock npx playwright test --project=property-workbench --workers=2` | 右键菜单改动后的 release 构建 | 126/126，4.3 min（墙钟 260 s） |
| 键盘走查（亮/暗） | 见上节，Playwright 键盘脚本 | 最终 release 构建 | 四页各走一遍、亮暗各 77 步，行为一致；**自动化代走，非人工** |
| wasm 构建 | `flock /tmp/build.lock mbx xtask build-web --example component-catalog --release`、`… property-workbench --release` | 同上 | 成功（component-catalog wasm 13,497,019 B，css 37,727 B） |

### 说明与缺口

- host 单测与 clippy 在最后三处改动之前运行：Toast 关闭后的焦点（`toast.rs`，整个模块 `#[cfg(target_arch = "wasm32")]`）、去掉颜色输入的 `maxlength`（`color_field.rs` 的 wasm 子模块）、目录示例的辅助函数改名（`component-catalog` 的 wasm `app` 模块）。三处都不进 host 编译；它们随后的 wasm release 构建、`cargo fmt --all -- --check`、`css --check`、`catalog --check` 与两轮回归层都在最终代码上通过。没有在最终代码上重跑 host 单测是因为磁盘：整机剩余在 2.6–3.3 GB 间波动（本工作树 target 约 3 GB，其余为其他工作树），按约束不再为此重建 host 调试产物。
- 最后一次 component-catalog 构建开始时整机剩余 2.8 GB（< 3 GB 阈值，构建命令在同一行先读 `df` 后直接开始）；构建成功，之后未再启动任何构建。
- 首轮走查脚本用 `flock /tmp/pw.lock bash -c '…&'` 起服务，`kill` 只杀到 `mbx` 外壳，子进程 `xtask serve` 继承锁 fd，约 5 分钟（17:41–17:46 UTC）占住 `/tmp/pw.lock` 与 4186 端口，发现后手动结束；第二轮改用 `flock -o` 与直接运行 `target/debug/xtask`。
- 不在本任务：Vellum 改用 Toast/草稿核心、Vellum 回归层、基线孪生 `visual.spec.ts`（M7 其余退出条件）；计划回写。
- 已知限制：菜单打开后自身尺寸变化（条目变多）不会重新夹取，`Layer` 只在锚点、滚动、窗口尺寸变化时重放；数字输入无增减按钮，Home/End 保留给光标；颜色输入无原生取色器（色块为预览）；Toast 键盘关闭后焦点回到作用域容器而非进入 Toast 前的位置；切换组无 read-only 态。
