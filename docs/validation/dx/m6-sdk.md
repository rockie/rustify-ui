# M6/M7 · SDK 行为原语（`rustify-ui`）· 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M6（`listen`、`shortcut`、`Layer` 模态 Tab 循环、`defer`/`defer_after`/`next_frame`）与 M7 的 SDK 半边（Toast 核心、`Draft<T>`）；ADR-4、ADR-5、D16、D17、§5.3
- 最近更新：2026-09-25 UTC
- 状态：SDK 半边完成（host 单测、clippy、fmt、wasm 构建通过）；示例改用、浏览器回归与孪生视觉属于后续集成，见计划「完成记录」
- 代码基线：`234d90e`（`f19a0bd` 合入工作分支 `fa76b20`）之上的子任务提交
- 改动范围：只动 `crates/rustify-ui`；`crates/rustify-makepad`、`makepad/`、示例、spec、`rustify-components` 均未改

## 实现记录

### 公开接口

| 出口 | 签名 | 目标 |
| --- | --- | --- |
| `listen` | `fn listen(target: &EventTarget, event: &str, options: ListenOptions, handler: impl FnMut(Event) + 'static) -> Listener` | wasm |
| `ListenOptions` | `struct ListenOptions { pub capture: bool, pub passive: bool }`（`Default`、`Copy`） | wasm |
| `Listener` | `#[must_use]`；Drop 即按同一 capture 标志移除；`Send + Sync`（`SendWrapper`），可放进 `on_cleanup`/`StoredValue` | wasm |
| `shortcut` 模块 | `Shortcut::parse(&str) -> Result<Shortcut, ShortcutError>`；`Shortcut::parse_for(&str, Platform)`；`Shortcut::matches_keystroke(&self, &Keystroke) -> bool`（纯函数）；`impl Display`（`aria-keyshortcuts` 形式） | 全平台 |
| | `Shortcut::matches(&self, &KeyboardEvent) -> bool`；`is_text_entry(&EventTarget) -> bool` | wasm |
| | `Platform { Mac, Other }`、`Platform::current()`；`Modifiers { ctrl, alt, shift, meta }`；`Keystroke<'a> { key, code, modifiers }`；`ShortcutError { Empty, MissingKey, UnknownModifier, RepeatedModifier, UnknownKey }` | 全平台 |
| `defer`、`defer_after` | 直接再导出 `rustify_makepad::{defer, defer_after}`（`defer_after(ms: i32, f: impl FnOnce() + 'static)`） | wasm |
| `next_frame` | `fn next_frame(f: impl FnOnce() + 'static) -> FrameHandle`；`FrameHandle::cancel(&self)`；`FrameHandle: Clone + Send + Sync`，Drop 不取消 | wasm |
| `toast` 模块 | `ToastOptions { duration_ms: u32, tone: ToastTone }`（默认 3200 ms、`Neutral`）；`ToastTone { Neutral, Success, Warning, Error }` + `key()`；`Toast { seq, message, tone }` | 全平台 |
| | `provide_toasts() -> ToastHandle`；`use_toasts() -> ToastHandle`（未提供时 panic，同 `use_drags`）；`ToastHandle::{show(msg, ToastOptions) -> u64, dismiss(seq), current() -> Option<Toast>}`；组件 `ToastRegion { render: Option<Callback<Toast, AnyView>>, class, test_id }` | wasm |
| `Draft<T>` | `Draft::new(value: Signal<T>, format: impl Fn(&T) -> String, parse: impl Fn(&str) -> Option<T>)`；`with_preview(Fn(T))`、`with_commit(Fn(T))`、`with_cancel(Fn())`；`text() -> Signal<String>`、`invalid() -> Signal<bool>`；`on_focus()`/`on_blur()` → `Fn(FocusEvent)`，`on_input()` → `Fn(Event)`，`on_keydown()` → `Fn(KeyboardEvent)`；`Copy` | wasm |
| `Message::Dismiss` | SDK 文案目录新增一条：`dismiss this message` / `关闭这条消息`，`Message::all()` 17 → 18 | 全平台 |

### 行为要点与取舍

- **一份实现**：`listeners.rs` 的 `page_level()` 仍是实例 abort signal 的唯一来源；`listen` 建在它之上，新增的 crate 内 `instance_signal()` 也从同一个 `rustify_makepad::listener_options()` 取 signal。`OverlayStack` 的三个监听（Escape/Enter、scroll、resize）改用 `listen`，`disarm()` 变成丢弃守卫；router、files 的既有注册未动。
- **`passive` 总是显式设置**：Chrome 对 window 上的 wheel/touch 默认 passive，交给浏览器默认会让「本该能 `preventDefault` 的监听」静默失效。
- **快捷键**：`Mod` 在解析时按平台落成 Meta/Control，平台每页只判定一次（`navigator.platform`，`Mac*`/`iP*` 视为 Mac），host 恒为 `Other`；修饰键按名字查重（`Mod+Ctrl+K` 在两个平台都能解析）。匹配以 `KeyboardEvent.key` 为准（AZERTY 的 `Mod+A` 是打出 `a` 的那个键），修饰键精确匹配；两处例外：符号/数字键不在乎未点名的 Shift（布局决定）；当布局对字母/数字键产出的不是 ASCII 字母数字（西里尔布局、Mac 上 Option 组合出符号、US 的 Shift+1 → `!`）时按物理键 `code` 兜底，此时 Shift 仍精确。`matches` 在输入法组合中恒为 false。`is_text_entry` 照 Vellum `keys.rs` 的判定（INPUT/TEXTAREA/SELECT/contenteditable）。
- **模态 Tab 循环**：只在 `modal` 的 `Layer` 上、冒泡阶段、用 `listen` 挂在层元素上（层内控件自己用 Tab 并 `preventDefault` 时优先）；只在两端回绕，中间走浏览器原生顺序。可聚焦枚举排除 `:disabled`（含 disabled fieldset 内）、`[inert]`/`[hidden]` 祖先、负 `tabindex`、未渲染（`getClientRects` 为空）与 `visibility: hidden|collapse`；同名 radio 只保留选中项（无选中取第一个）；正 `tabindex` 按值排在前。焦点落在层内非停靠点（脚本聚焦的标题）时按文档位置判断是否已到端点。`focus_first` 改用同一枚举。非模态层与关闭时的焦点归还不变。回绕判定抽成纯函数 `wrap_tab`，host 表驱动测试。
- **`next_frame`**：不需要改 `rustify-makepad`。请求帧后把 `cancelAnimationFrame.bind(window, id)` 以 `once` 挂到实例 abort signal 上——abort 时由浏览器执行取消，不进 wasm（等价于 Vellum `__vellumAbortResource` 的 raf 分支，但无需页面胶水）；回调执行或 `cancel()` 时摘掉该监听并释放闭包（闭包由 Rust 持有，取消不泄漏）。signal 已 abort 时不再请求帧；无宿主时退化为普通 rAF。
- **`defer_after` 不会在 abort 后运行**：宿主 `enter_fatal` 清掉全部计时器与回调（`crates/rustify-makepad/web/embedded.js`），SDK 直接再导出，无第二份实现。
- **Toast**：纯状态机（序号递增、新替旧、按时间过期）与 DOM 分开。每次 `show` 用宿主的 `defer_after` 定时；计时器带序号触发，被替换/关闭过的序号什么也不做，计时器早到（粗粒度计时）则按剩余时间再挂一次。只在真正隐藏时通知订阅者。`ToastRegion` 经 `Portal` 渲染到作用域 overlay 根，`role="status"`、`aria-live="polite"`，区域元素常驻（读屏只播报已知的 live region 的变化），不进 `Layer` 栈、不参与 inert 与 Escape；默认标记为消息 + 关闭按钮（`aria-label` 取 `Message::Dismiss`，`data-testid="<test_id>-dismiss"`），自定义标记用 `render` 闭包（ADR-5）。
- **`Draft<T>`**（ADR-4）：未聚焦或聚焦未改动时显示应用值（外部变化即时跟随）；有输入后显示草稿，可解析的输入发预览请求，应用拒绝不抹草稿；Enter/失焦各结束一次编辑、提交一次；不可解析时若本次编辑发过预览则提交最后一次预览（与 Vellum「非法输入仍结束先前的有效预览」一致，保证每次发过预览的编辑都以一次 commit 或 cancel 收尾），否则不发请求，显示回到应用值；Escape 丢弃草稿，发过预览则请求 cancel；改动过的草稿在外部值变化时保留，失焦后落到应用当时的值。实现细节：只在一次编辑结束时通知显示信号（草稿本身已在输入框里，逐键写回会打断输入法组合），`invalid` 单独一个信号供 `aria-invalid`。有草稿期间向作用域 `OverlayStack` 注册 `EditSession`：作用域在捕获阶段先听到 Escape，否则会先关掉字段所在的对话框，留下既未提交也未撤回的预览；草稿结束或组件清理时注销。文档注明应绑定文本输入（`type=number` 对 `-`、`1e` 这类中间态报告空值）。
- **cfg 约定**：纯核心（`Session`、`Toasts`、`wrap_tab`）为 crate 私有，仅 wasm 使用，照 `region.rs` 先例加 `#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]`；公开数据类型（`Shortcut` 系列、`Toast`/`ToastOptions`/`ToastTone`）不分目标；DOM 相关出口一律 `#[cfg(target_arch = "wasm32")]`。
- **web-sys 特性**：`rustify-ui` 显式声明新用到的 `AbortSignal`、`FocusEvent`、`CssStyleDeclaration`、`DomRectList`、`Node`、`NodeList`。
- **API 可用性核验**：临时加一个仅 wasm 的调用方模块（`Draft` 绑定 `prop:value`/四个处理器、`ToastRegion` 默认与 `render` 两种用法、`listen` + `Shortcut::matches` + `is_text_entry`、`on_cleanup(move || drop(listener))`、`next_frame` + `cancel`、`defer`/`defer_after`），`mbx check --target wasm32-unknown-unknown` 通过后删除，未提交。

## 验收记录

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| host 单测 | `mbx test -p rustify-ui --lib` | Linux x64，Rust 1.98.1，子任务提交 | 通过：159 项（基线 133；新增 26：`draft` 13、`shortcut` 6、`toast` 6、`overlay::tab_wraps_only_at_the_two_ends_of_a_modal_layer` 1；`i18n` 既有 3 项覆盖新文案） |
| 依赖方单测 | `mbx test -p rustify-components --lib` | 同上 | 通过：68 项（文案目录 17 → 18 不影响） |
| clippy（host，全工作区） | `mbx clippy --workspace --all-targets -- -D warnings` | 同上 | 通过（exit 0） |
| clippy（wasm，本 crate） | `mbx clippy -p rustify-ui --target wasm32-unknown-unknown` | 同上 | 新代码无告警；仅余基线既有告警（`overlay.rs` 的 `fallback_element` 未用、`router.rs`/`text.rs`/`theme.rs`/`region.rs`/`gpu/lists.rs` 各处） |
| 格式 | `cargo fmt -p rustify-ui -- --check` | 同上 | 通过。注：嵌套 worktree 下 `cargo fmt --all` 会沿 `makepad/widgets` 向上找到外层仓库的 workspace 而报错（环境问题，非代码问题），故按包执行；CI 的 `cargo fmt --all -- --check` 在普通检出中不受影响 |
| wasm 编译 | `mbx build -p rustify-ui --target wasm32-unknown-unknown` | 同上 | 通过 |
| 浏览器回归（`p2-a11y`、`m4-overlay`、`p2-semantics`、Vellum `m5-shell`/`m7-files` 等） | — | — | 未跑：按 §12 浏览器验证由主 agent 串行执行，且须在示例改用之后；`Layer` Tab 循环影响所有模态层，集成时请跑 component-catalog 与 fusion-basic 的回归层 |
| `build-web --example component-catalog --release` | — | — | 未跑：会话磁盘配额接近用满（清理本 worktree 自己的 mbx 目标后约 1–2 GiB 可用，其余目标属于在用检出），以 `mbx build -p rustify-ui --target wasm32` 代替 |
