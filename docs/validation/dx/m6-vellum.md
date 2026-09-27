# M6 子任务 · Vellum 改用 SDK 行为原语 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M6 的 Vellum 半边（ADR-5、D16–D18、C-4、C-9、NFR-4、NFR-7、§1.1「Vellum 抽取」、§5.3「Vellum 改用顺序」、§9.4「Vellum 基线孪生」、A-5）；SDK 原语见 [m6-sdk.md](m6-sdk.md)
- 最近更新：2026-09-25 UTC
- 代码基线：`12eae0d` 之上的本子任务提交（独立工作树，分离 HEAD）；孪生基线 `24f1a11`（`/home/user/rustify-ui-vellum-baseline`，其 `examples/vellum`、`tests/vellum` 与 `12eae0d` 逐字节相同，差别只在 SDK）
- 范围：`examples/vellum/{src,app.js,Cargo.toml}`，删三份死 CSS；`crates/rustify-ui/src/{listeners,frame,lib}.rs`（新增两个出口）；`tests/vellum/{m3-raster,m6-edit}.spec.ts`（A-5 门控）；`docs/vellum.md` 两段机制描述。未动 property-workbench、data-workbench、fusion-basic（M6 另一半）、`CLAUDE.md`、计划正文与「实施进度」（由主 agent 回写）。

## 实现记录

按 §5.3「Vellum 改用顺序」逐步改用；Toast 核心与草稿核心属 M7，未动。

### 1. 监听守卫 → `rustify_ui::listen`

| 原守卫 | 改为 | 备注 |
| --- | --- | --- |
| `pointer.rs` 私有 `Listener` + `Bindings` 的 `Drop`（画布 7、区域 wheel 1、window blur 1） | `Bindings { listeners: Vec<rustify_ui::Listener> }`，`Bindings::listen` 只包一层 `listen(..., ListenOptions::default(), ..)` | 原先显式 `passive: false`（wheel 要 `preventDefault`）；`ListenOptions` 默认即显式非 passive。`install` 不再返回 `Result` |
| `keys.rs` `Bindings` 的 `Drop`（document keydown/keyup） | `Bindings { _down: Listener, _up: Listener }` | 事件按原样不做类型检查地当作 `KeyboardEvent`（`unchecked_ref`，与原 `Closure<dyn FnMut(KeyboardEvent)>` 相同） |
| `fileio.rs` `DropBindings` 的 `Drop`（dragover/drop） | `DropBindings { _over, _drop }` | 原先无运行时返回 `Err("Runtime lifecycle is unavailable")`，现在无宿主时照常注册（SDK `listen` 的约定，Vellum 总在 `boot()` 之后挂载） |
| `shell/menus.rs` `OutsideClick` 的 `Drop`（document pointerdown） | `listen` + `on_cleanup(move \|\| drop(outside))` | `Listener` 是 `Send + Sync`，可直接进 `on_cleanup` |
| `shell/text_session.rs` `Bindings` 的 `Drop`（textarea input） | `Bindings { element, _input: Listener }` | `bind` 不再返回 `Result` |
| `storage.rs` `Listeners` 的 `Drop`（document visibilitychange、window beforeunload） | `State.listeners: Vec<Listener>`，`dispose` 用 `mem::take` 丢弃 | beforeunload 仍非 passive（要 `preventDefault`） |
| `storage.rs` 私有 `listen()`（IndexedDB 事务 complete/error/abort、打开请求 success/error/upgradeneeded、读请求 success/error） | 每处 `vec![listen(..), ..]`，Promise 落定后 `drop(listeners)` | 原先注册后手工 `remove_event_listener` 再丢闭包；现由 `Listener` 的 `Drop` 按同一 capture 标志移除。error 与 abort 共用一个处理器，改为克隆闭包各注册一次 |

`rustify_makepad::listener_options` 另有四处只用来问「实例是否已失败」（`app.rs` `ensure_mounted`、`fileio.rs` `mounted`、`fonts.rs` `runtime_signal`、`storage.rs` 的 `State.signal`），改用新出口 `rustify_ui::instance_failed()`（见「新增 SDK 出口」）。

### 2. 快捷键防护 → `rustify_ui::shortcut`

- 文本输入防护：`keys.rs` 手写的 INPUT/TEXTAREA/SELECT/contenteditable 判定换成 `rustify_ui::is_text_entry`（SDK 照原判定实现，语义相同）。
- 修饰键：`command = meta || ctrl` 换成 `command_held()`，按 `rustify_ui::Platform::current()` 取 `Mod` 的含义——Mac 上 Meta、其余平台 Control（与 `Shortcut::parse("Mod+…")` 同一平台判定，每页只判一次）。
- 按键表（`keys.rs` 的 `key_down`）原样留在 Vellum。没有把表项逐条换成 `Shortcut::matches`：表对未点名的修饰键宽松（Shift+R 仍选矩形、Ctrl+Alt+Z 仍撤销、Ctrl+Delete 仍删除、`Escape` 不看修饰键），`Shortcut` 的修饰键精确匹配会改变这些组合，违背「行为不变」；注释写明了原因。
- 唯一有意的行为变化（任务明确要求的 `Mod` 语义）：非 Mac 平台上 Meta（Super/Win 键）+ 字母不再触发命令，Mac 上 Control + 字母不再触发命令（此前两者都算命令键）。`tests/vellum` 在 Linux 上全用 Control/`ControlOrMeta`，m4-pointer、m5-shell、m6-edit 钉住的行为不变（见验收）。滚轮缩放的 `ctrl || meta`（触控板捏合在各平台都报 `ctrlKey`）不是快捷键，未动。
- 后续修正（集成后 PR #3 的 macOS CI 发现）：上一条「在 Linux 上全用 Control」正是漏洞——m4-pointer、m5-shell、m6-edit 共 6 项在 macOS runner 上失败（Control+K/N/Z/Enter 在 Mac 上按设计不再是命令键），本地 Linux 与此前 CI 都没覆盖到。保留计划定的 `Mod` 语义，把 `tests/vellum` 里命令快捷键的 `Control+` 全部换成 Playwright 的 `ControlOrMeta+`（m4-pointer 3、m5-shell 3、m6-edit 1、smoke 3），在 Mac 上按 Meta、其余平台按 Control；原版孪生两种都认，孪生比对不受影响。拖拽/点击时按住的 Control（关吸附、深选）走 `ctrl || meta`，未改。

### 3. 模态 Tab 循环 → SDK `Layer`

- 删除 `shell/dialogs.rs` 与 `shell/presentation.rs` 的两份 `trap_tab`，以及只为它们存在的 `node_ref` 与 `on:keydown`。两处本来就是 `<Layer modal=true …>`，无需改成模态层；DOM 的 id/class/`data-*` 不变（`node_ref` 与 Leptos 事件处理器不产生属性）。
- 差异：Vellum 的选择器（`button,input,select,textarea:not([disabled]),[tabindex]:not([tabindex='-1'])`，演示层只取 `button`）不看可见性；SDK 的停靠点另含 `a[href]`、contenteditable 等并排除隐藏/未渲染/`inert` 元素、按 radio 组取一，且 Ctrl/Alt/Meta+Tab 不处理。Vellum 的对话框与演示层里没有链接、contenteditable、radio 或隐藏控件，两者得到的首尾停靠点相同；`m5-shell`「SDK menu and modal layers keep keyboard focus…」的首尾回绕断言通过。

### 4. 帧与延时 → `rustify_ui::{next_frame, defer_after}`

- 删除 `src/browser_frame.rs` 及 `main.rs` 的模块声明；6 处调用改 `next_frame`：`app.rs` 光栅细化帧、`shell/dialogs.rs` 提示框聚焦、`shell/menus.rs` 菜单首项聚焦、`shell/palette.rs` 结果滚动、`shell/text_session.rs` 会话绑定、`shell/left_panel.rs` 搜索框聚焦（后者原本不留句柄，`FrameHandle` 丢弃不取消，语义相同）。原先请求失败时的 `Err` 分支（报「Raster refresh unavailable」等）随之删除——`next_frame` 不返回错误。
- `rustify_makepad::defer_after` 两处（`shell/toast.rs` 3200 ms、`storage.rs` 500 ms 防抖）改 `rustify_ui::defer_after`（SDK 直接再导出，同一实现）。Vellum 不用 `defer`。
- 删除 `app.js` 的 `window.__vellumAbortResource`（22 行）。它在 trap 后以纯 JS 释放五类资源；raf 由 `next_frame` 接管，其余四类改用新出口 `release_on_abort` 登记绑定好的原生 JS 方法：

| 原 kind | 现在登记的 JS 函数 | 位置 |
| --- | --- | --- |
| `raf` | `cancelAnimationFrame.bind(window, id)`（SDK `next_frame` 内部） | `crates/rustify-ui/src/frame.rs` |
| `font` | `document.fonts.delete.bind(document.fonts, face)` | `fonts.rs` `remove_on_failure` |
| `transaction` | `transaction.abort.bind(transaction)` | `storage.rs` `release(.., "abort")`，写事务与读事务各一 |
| `database` | `database.close.bind(database)` | `storage.rs` `release(.., "close")` |
| `open`（abort 时已成功则关闭结果；abort 后才成功的「晚到连接」也关闭） | 由原生函数串成的链：请求的 `success` 直接落定一个 Promise（`resolve` 是 JS）→ `.then(Reflect.get.bind(undefined, request, "result", request))` 得到连接 → abort 时调用 `connection.then.bind(connection, Function.prototype.call.bind(IDBDatabase.prototype.close))` | `storage.rs` `close_when_opened` |

  与胶水的一处差异：胶水用 `try/catch` 吞掉异常，现在释放函数直接作 abort 监听器，抛出的异常按监听器异常上报（控制台一条，不影响其余释放与 `enter_fatal` 的顺序，loader 只把属于本实例 glue 的 `WebAssembly.RuntimeError` 当作 trap）。五类里只有「对已结束的事务 `abort()`」会抛。事务的登记在事务事件落定 Promise 后、同一微任务检查点里恢复的 future 中即被丢弃；会遇到它的只有：`dispose()` 已直接中止事务、挂起的 future 还没恢复时，同一同步卸载过程里再发生 trap。
- `app.js` 现在只剩 loader 接线、自动化门面与 `reset()`；「不留页面胶水」达成。

### 5. 死 CSS

删除 `examples/vellum/shell-inspector.css`、`examples/vellum/text-session.css`、`examples/vellum/src/shell/layers.css`。删除前核实：三者内容与 `app.css` 末段规则逐条相同；`xtask/src/build.rs:124` 只复制 `app.js`/`app.css`（另有 `index.html`、`vendor/`、按依赖名复制的组件样式表，均与三者无关）；全仓 `rg` 除计划正文外无引用，Rust 源里无 `include_str!` 指向 CSS。

### 6. C-9 / D18

Vellum 标记与 CSS 未改：id、class、`data-*` 不变，未新增 ARIA（不需要）；未改用任何带样式的目录组件，`Cargo.toml` 仍只依赖 `rustify-ui`（另删去不再使用的 web-sys 特性 `AddEventListenerOptions`）。

### 7. 新增 SDK 出口（`crates/rustify-ui/src/listeners.rs`，仅 wasm）

| 出口 | 签名 | 为什么需要 |
| --- | --- | --- |
| `release_on_abort` | `pub fn release_on_abort(release: js_sys::Function) -> AbortRelease` | 把一个 JS 函数登记为实例 abort signal 的 `once` 监听器：实例失败时由浏览器调用，不进 wasm；`AbortRelease` 丢弃时移除登记（正常卸载路径）；实例已失败时立即调用；无宿主时什么也不登记。宿主无关，文档写明只能传 JS 函数（绑定好的原生方法）、不能传 wasm 闭包 |
| `AbortRelease` | `#[must_use] pub struct AbortRelease`，`Send + Sync`（`SendWrapper`），`Drop` 即移除 | 同 `Listener` 的所有权约定 |
| `instance_failed` | `pub fn instance_failed() -> bool` | 宿主停得住的回调（监听、任务、帧）都会停，但浏览器落定的 Promise 仍会恢复 `await`；Vellum 在这些恢复点先问实例是否已失败。原先直接读 `listener_options()` 的 signal |

- `frame.rs` 改为经 `release_on_abort` 登记 `cancelAnimationFrame`（删去自己的 signal 登记/摘除代码），SDK 内「向 abort signal 登记 JS 释放」只剩一份实现；`instance_signal()` 降为模块私有。行为不变：已失败时不请求帧；执行或 `cancel()` 时摘掉登记。
- 没有新增 host 单测：两者只在 wasm 下存在、只有 DOM 行为；由 Vellum 的浏览器用例覆盖（`m7-files`「runtime traps recover saved work through three SDK restarts」「a runtime trap aborts a storage write before its transaction commits」「local font faces leave the browser when their runtime fails」）。

### 8. NFR-7

`rg "rustify_makepad::(listener_options|defer|defer_after)" examples/vellum` → 0 行（exit 1）。Vellum 里剩下的私有 crate 直连只有 `automation.rs:25` 的 `rustify_makepad::live_region_count()`（测试门面读区域数，SDK 未再导出），与监听/延时无关，保留；`Cargo.toml` 的 wasm 依赖 `rustify-makepad` 因此保留。`observe_resize`/`ResizeObservation` 改用 `rustify_ui` 的再导出（同一实现）。

### 9. 文档

`docs/vellum.md`「文件、持久化与演示」一段与「可复核数字」的监听计数段改写为现行机制（`listen`/`next_frame`/`release_on_abort`/`instance_failed`，`app.js` 不再有资源胶水），并把复核命令换成新的注册点；原段落里指向已删除 `browser_frame.rs` 的链接一并去掉。该节其余历史数字（产物体积、`app.js` 行数等）未动，留给 M8 的 `docs/vellum.md` 整理。`CLAUDE.md`「Page-level listeners go through `rustify_makepad::listener_options`」对示例已不准确，按计划归 M8 同步，本子任务未改。

## 验收记录

环境：Linux x64 容器（4 核、15 GiB、无 GPU），rustc 1.98.1、mbx 1.15.0、Node 26.1.0、Playwright 1.63.0，Chromium headless shell（`/root/pw-compat` 垫片，同 M2）；`node_modules` 为指向主检出的符号链接（已被 `.gitignore` 忽略）。release 构建由本工作树 `flock /tmp/build.lock mbx xtask build-web --example vellum --release` 产出（2 m 45 s；wasm 20,229,672 B）。所有 Playwright 运行都在 `flock /tmp/pw.lock` 下；耗时取 Playwright 自报，不含等锁。

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| NFR-7 私有出口直连 | `rg "rustify_makepad::(listener_options\|defer\|defer_after)" examples/vellum` | 本提交 | 0 行（改前 `12eae0d` 为 14 处，10 个文件）；Vellum 全部 `rustify_makepad::` 用法 17 → 1（`automation.rs:25` `live_region_count`） |
| wasm 编译与 lint | `mbx check -p vellum --target wasm32-unknown-unknown`；`mbx clippy -p vellum -p rustify-ui --target wasm32-unknown-unknown` | 同上 | 通过；改动文件无新告警，余下均为改前既有（Vellum `painter`/`pointer`/`raster` 死代码、`dialogs.rs:153,271` 与 `topbar.rs` 的 clippy 提示；SDK `overlay`/`router`/`text`/`theme`/`region`/`gpu/lists` 各处） |
| release 构建 | `flock /tmp/build.lock mbx xtask build-web --example vellum --release` | 同上 | 通过，2 m 45 s；wasm 20,229,672 B、JS 247,178 B、CSS 32,652 B |
| host 单测 | `mbx test -p vellum`；`mbx test -p rustify-ui --lib` | 同上 | vellum lib 137 + bin 5 通过；rustify-ui 159 通过（与 SDK 记录相同，新出口仅 wasm） |
| clippy（host，全工作区） | `mbx clippy --workspace --all-targets -- -D warnings` | 同上 | 通过（exit 0） |
| 格式 | `cargo fmt --all -- --check`；另跑 `cargo fmt -p vellum -p rustify-ui -- --check` | 同上 | 均通过（本工作树里 `--all` 未受嵌套 worktree 影响） |
| Vellum 回归层，第一轮 | `flock /tmp/pw.lock npx playwright test -c tests/vellum/playwright.config.ts` | 同上（门控改动之前） | 58 passed、16 skipped、0 failed，4.0 min。16 个跳过 = 默认模式无 `ref/` 的 13 个孪生用例 + 3 个需有头 Chrome 的用例（`m3-webgpu:101`、`m8-ui:79,256`）；本改动不改变默认模式的跳过集合 |
| Vellum 回归层，第二轮 | 同上 | 同上 | 58 passed、16 skipped、0 failed，3.2 min |
| 门控改动后的抽查 | 同上，只跑 `m3-raster.spec.ts m6-edit.spec.ts` | 本提交 | 17 passed、3 skipped（孪生用例，默认模式无孪生），0 failed |
| 基线孪生视觉门（NFR-4、§9.4） | `VELLUM_TWIN=baseline VELLUM_BASELINE_DIR=/home/user/rustify-ui-vellum-baseline RUSTIFY_TIER=evidence flock /tmp/pw.lock npx playwright test -c tests/vellum/playwright.config.ts visual.spec.ts` | 被测：本提交；孪生：`24f1a11` release | 2 passed（30.5 s）。8 张视图每张 0 / 1,600,000 像素不同，比例 0：启动页 0/1/2 × 亮/暗 6 张、主菜单、帮助对话框。门 ≤ 2% |
| A-5 孪生用例试跑 | `VELLUM_TWIN=baseline VELLUM_BASELINE_DIR=… RUSTIFY_TIER=all flock /tmp/pw.lock npx playwright test -c tests/vellum/playwright.config.ts m3-raster.spec.ts m4-pointer.spec.ts m6-edit.spec.ts m7-files.spec.ts` | 同上 | 55 项：52 passed、3 failed（`m3-raster:41`、`m6-edit:323`、`m4-pointer:192`，见下表与下文）；改门控/选择器后复跑 `m3-raster.spec.ts:41 m3-raster.spec.ts:104 m6-edit.spec.ts:323`：2 passed、1 skipped |

### A-5：孪生用例对基线

| 孪生用例 | 对基线 | 原因 / 实测 |
| --- | --- | --- |
| `m3-raster:41` typography … Unicode layout matches the browser reference | 不能，基线模式跳过 | 在孪生页里 `import("/src/renderer.js")` 调原版的 JS 布局函数作期望值；基线是同一 wasm 示例，没有该模块（实测 `Failed to fetch dynamically imported module`）。门控由 `!hasReference` 改为 `twin !== "original"` |
| `m3-raster:104` styled text raster … unit zoom | 能 | 0 / 120,000 像素不同 |
| `m4-pointer:88` twin geometry ×6（create、alt-shift-move、alt-shift-resize、shift-rotate、marquee、wheel-and-space-pan） | 能 | 6/6 通过，几何逐字段相等 |
| `m4-pointer:232` pen handles … original path geometry | 能 | 通过，路径点完全相等 |
| `m4-pointer:261` two touch pointers pinch | 能 | 通过，相机与几何相等（zoom 1.6） |
| `m4-pointer:284` hover, marquee, guides, pen preview, rulers | 能 | 通过，5 张覆盖层截图在 RGB 阈值 24 下 0 像素不同 |
| `m6-edit:323` text and pen editing … screenshot gate | 能（改选择器后） | 原先在孪生页找 `#text-editor`（原版文本会话的 id），本例的会话是 `[data-testid="text-editor"]`，首跑超时。改为按孪生来源选择器（原版照旧 `#text-editor`）；几何相等，整页截图 text 0、pen 0 像素不同 |
| `m7-files:499` reference and Rust portable documents open in both directions | 能 | 通过（两边 171/31/0 层，逐页相等） |

依赖原版专有行为、在基线模式下保持跳过的只有 `m3-raster:41`。`?canvas` 的 Canvas 2D 渲染器是原版专有的，这一点已由 `twin.ts` 的 `openTwin` 在基线模式下改为断言「Makepad WebGL2」处理，不需要额外门控。能对基线跑的 12 项可纳入 M6/M7 退出条件（本表即 M6 的一次实跑）。

### 证据层用例 `m4-pointer:192`「pan presentation latency」

A-5 以 `RUSTIFY_TIER=all` 运行时连带跑了这个证据层用例，p95 315.8 ms > 50 ms 而失败（前 6 个样本 7–21 ms，其后 134–438 ms）。为区分回归与环境，在同一把锁下交替跑两轮（被测页分别由本提交与 `24f1a11` 基线的 release 构建提供，spec 相同；基线那轮用 `test-results/` 下的临时 config 覆盖 4179 服务的 `cwd`，跑完即删）：

| 轮次 | 本提交 p95 / 中位 | 基线 p95 / 中位 |
| --- | --- | --- |
| 1 | 219.8 / 164.6 ms | 290.2 / 159.5 ms |
| 2 | 624.6 / 227.2 ms | 517.3 / 208.6 ms |

两边同样失败、同一量级，失败来自环境（4 核容器、SwiftShader、同机其他 agent 的构建与浏览器运行，`uptime` 负载约 4），不是本改动的回归。历史值 34.8–44.5 ms（`docs/vellum.md`）是另一台机器的样本。该用例不在回归层，也不在本子任务的退出条件内；按计划，证据层阈值不改。

### 缺口与说明

- 磁盘：本工作树的 mbx 目标约 3.8 GiB；运行期间根分区可用空间在 3.2–11 GiB 间波动（其他工作树同时构建），最低 3.2 GiB，未低于 3 GiB。
- 回归层两轮在门控改动之前运行；门控改动只影响孪生用例（默认模式跳过），已用抽查运行确认。
- 未跑 property-workbench、component-catalog、fusion-basic 回归层（M6 另一半与集成的退出条件，归主 agent）。
- `CLAUDE.md` 的 Page-level listeners 条目与 `docs/vellum.md` 的历史数字待 M8 同步。
