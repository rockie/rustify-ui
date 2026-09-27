# M7 子任务 · Vellum 改用 Toast 核心与草稿核心 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M7 的 Vellum 半边（ADR-4、ADR-5、D17、D18、C-5、C-9、NFR-4、§5.3「Vellum 改用顺序」最后两步）；SDK 核心与目录组件见 [m7-components.md](m7-components.md)，M6 的 Vellum 改用见 [m6-vellum.md](m6-vellum.md)
- 最近更新：2026-09-27 UTC
- 代码基线：`d94471d`（`claude/hopeful-mccarthy-3fceae` 头，已含 M6 Vellum 改用 `8a3b569` 与目录组件 `1977e51`）之上的本子任务提交。工作树分支原在 `b15e4b9`，缺 M6 的 Vellum 改用，先 `git merge --ff-only` 快进到 `d94471d` 再改，避免与主线在同一批文件上冲突
- 范围：`examples/vellum/src/{app.rs,pointer.rs,shell/mod.rs,shell/toast.rs,shell/fields.rs}`，`tests/vellum/m5-shell.spec.ts` 一条用例加 Escape 断言。未动 SDK、`Cargo.toml`、`app.css`、`index.html`、`app.js`、计划正文与「实施进度」（由主 agent 回写）

## 实现记录

### 1. Toast → `rustify_ui::toast`

| 原实现 | 现在 | 位置 |
| --- | --- | --- |
| `ShellState.toast: Option<String>` + `toast_serial: u64` | 删除；`Editor.toasts: ToastHandle`，`Editor::new` 里 `provide_toasts()` 创建 | `app.rs`、`shell/mod.rs` |
| `shell::toast::show` 递增序号；`Toast` 组件里 `Effect` 监听 `(serial, visible)` 起 `defer_after(3200)`，到点序号未变则清空 | 删除；`shell::toast(editor, msg)` 改为 `editor.toasts.show(msg, ToastOptions::default())`，序号、替换、计时（含计时器早触发时续等余量）全归 SDK | `shell/mod.rs` |
| `#toast` 读 `ShellState` | 读 `editor.toasts.current()`（`Memo<Option<String>>`），标记逐字不变 | `shell/toast.rs` |

- 时长：`ToastOptions::default()` 为 3,200 ms，这个默认值本就取自 Vellum，并由 SDK host 单测 `a_message_shows_with_the_default_options_until_its_time_is_up` 断言。
- 句柄放在 `Editor` 而不是每处 `use_toasts()`：约 30 处调用里有的在 `leptos::task::spawn_local` 任务、`listen` 回调或 `Closure::once_into_js` 回调中（`storage.rs`、`fileio.rs`、`shell/clipboard.rs`、`shell/dialogs.rs`），这些地方没有 reactive owner，读不到 context。
- 保留 `shell::toast(editor, msg)` 这一行薄封装：它是 Vellum 的调用入口（文案归应用），现在直接委托 SDK，不再持有状态。
- **未用 `ToastRegion`**：它只在有消息时才调用 `render` 画标记，Vellum 的 spec 与样式要求常驻的 `#toast`（`smoke.spec.ts:285` 直接对 `#toast` 求值、`prepareVisual`/`m7-files` 按 id 取、`.toast.hidden` 切换并在显示时重放 `tip-in` 动画）。套用 `ToastRegion` 还会多出一层带 `class=""`、`data-testid=""` 的包裹 div 和第二个 `role=status`，`m5-shell` 的 `getByRole("status")` 会触发严格模式冲突。所以 Vellum 保留自己的 Portal 标记（与原来同一个 `overlay_root()`、同样不进 `Layer` 栈），只把状态与计时换成 SDK 核心。DOM 不变。
- 计时起点：旧实现在 `Effect`（下一个微任务）里起计时，新实现在 `show` 内同步起，相差不到一帧。toast 移出 `ShellState` 后，显示消息不再让依赖 `ShellState` 的 memo（图层树 HTML 等）重算，但这些 memo 本来就按值去重，所以没有可见差异。

### 2. 草稿 → `rustify_ui::Draft`

| 字段 | `Draft` 参数 | 请求 → Vellum |
| --- | --- | --- |
| `NumberField` | `Draft<f64>`：值 `Memo(number(nodes, prop))`，格式 `formatted(v, 100.0)`，解析 `text.parse::<f64>()` 且有限（与旧提交路径相同，不 trim） | 预览 → `set_property(.., true)`；提交 → `set_property(.., false)`；取消 → `pointer::cancel_history`（原为私有，改 `pub`：有挂起历史才恢复编辑前文档并重置栅格缓存） |
| `ColorField` 的 hex 文本框 | `Draft<String>`：值 `Memo(text(nodes, prop))`，格式 `hex_display`，解析恒为 `Some(text)` | 仅提交：Vellum 自己的 `parse_hex` 合法 → `set_property(.., false)`，否则 toast「Enter a 3- or 6-digit hex color, or None.」 |

- 删除两个字段手写的 `focused`/`dirty`/`draft` 信号与 `commit` 闭包，以及随之不用的 `web_sys::HtmlInputElement` 引入。颜色选择器（`type=color`）的 `picker_dirty` 不是草稿逻辑，保留。
- 数字字段保留 `type="number"`（`spinbutton` 角色、spin 按钮与样式都是 Vellum DOM 的一部分）。`Draft` 文档建议用文本框，是因为数字输入框把 `-`、`1e` 这类中间文本报成空串；但草稿在编辑结束前不回写字段，浏览器自己保留着输入的文本，空串只被当作「还不是数」。
- 数字字段保留 `on:change` → `draft.commit()`：spin 按钮单独以 `change` 结束一步，每步一个撤销步，与旧实现相同。hex 文本框不再绑定 `on:change`：文本框的 `change` 只随 Enter/失焦触发，这两个时机已由 `Draft` 处理（旧实现在 Enter 之后的 `change` 上可能再提交一次、再弹一次 toast，但不产生新的撤销步）。
- hex 用恒等解析，是为了保留 Vellum 的 hex 规则（见差异 4）。代价是 hex 草稿没有「非法」状态，所以不输出 `aria-invalid`；旧实现同样没有。
- 未新增任何 ARIA 属性，id、class、`data-*`、`type`、`step/min/max`、`spellcheck` 全部不变。

### 3. 发现的行为差异

| # | 场景 | 旧 Vellum | 现在 | 处理 |
| --- | --- | --- | --- | --- |
| 1 | 输入过的字段里按 Escape | 无作用：全局快捷键对输入框放行，覆盖层栈无层时也不处理；已预览的值留在文档里，失焦时照常提交 | `Draft` 在草稿被改动时向作用域覆盖层栈登记，Escape 撤回草稿。数字字段请求取消预览，Vellum 恢复编辑前文档，撤销栈不增；hex 字段本来不预览，只把文本还原为应用值。若当时有非模态层（如菜单）打开，Escape 先撤回草稿、不关层 | **采用 `Draft` 规则**（ADR-4）。用 `Draft` 就无法保留旧行为：草稿一改动就向栈登记。不接取消钩子的话，Escape 会丢掉草稿却留下未提交的挂起历史，下一次编辑会并进同一个撤销步，破坏「一次编辑一步撤销」。`m5-shell`「focused inspector drafts …」新增断言（输入 420 → Escape → 仍聚焦、显示 180、文档 w=180） |
| 2 | 聚焦但未输入时，应用值被别处改变 | 聚焦时把草稿定为当时的应用值，此后到失焦一直显示这个快照；spin 按钮 `change` 后显示浏览器值而非应用值 | 未改动的聚焦字段随应用值变化；每次提交后显示应用值 | **采用 `Draft` 规则**（ADR-4「草稿未改动则跟随外部值」）。只有在字段聚焦期间应用值被别处改写时才看得出来（自动化 API、启动时存储水合完成、应用改写提交值）；spec 未钉 |
| 3 | 先输入合法数值、再改成非法文本（如 `12` → `12e`）后结束编辑 | 两者都把最后一次预览提交为一个撤销步。旧路径只调 `history.commit(doc)`，不跑 `apply_all_layouts` / `sync_components` | 走 `set_property(.., false)`，与合法提交同一路径，自动布局与组件实例随之更新 | **采用 `Draft` 规则**。旧路径在自动布局子节点或主组件上会留下未重排、未同步的状态，与其他提交路径不一致；最终值和撤销步数不变 |
| 4 | hex 先输入合法、再改成非法（如 `FFF` → `FFFF`）后结束编辑 | 不预览；非法就 toast 并回到应用值，不退回编辑中较早出现的合法值 | 同旧 | **保留 Vellum 规则**。若以 `parse_hex` 作 `Draft` 的解析，`Draft` 会静默提交 `#FFFFFF` 且不提示；为此 hex 的 `Draft` 解析恒等，由提交钩子判定。`m5-shell` 钉住 toast 文案与回退 |

除上表外，已逐条对照的路径均与旧实现一致：数字字段每次可解析的输入都是一次实时预览；Enter/失焦/spin `change` 各提交一次；只在未改动时失焦不提交；从未合法过的文本不提交并回到应用值。hex 字段的合法提交、非法 toast 并回到应用值，以及 Enter 与 Tab 两种结束方式也都一致。

## 验收记录

环境：Linux x64 容器，rustc 1.98.1、mbx 1.15.0；工作树 `/home/user/rustify-ui/.claude/worktrees/agent-ab1874e033f58da39`（嵌套在主检出内）。按主 agent 要求未运行任何 Playwright 与 `xtask serve`。

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 格式 | `cargo fmt --all -- --check` | 未能运行，属环境问题：本工作树嵌套在主检出的 `.claude/worktrees/` 下，`--all` 为 makepad 路径依赖跑 `cargo metadata` 时向上找到主检出的 `Cargo.toml`，报「believes it's in a workspace when it's not」，与改动无关 |
| 格式（替代） | `cargo fmt -- --check`（工作区全部成员）；`rustfmt --check --edition 2021` 五个改动的 `.rs` 文件 | 均 exit 0 |
| host 单测 | `mbx test --workspace --lib --bins` | 全过：vellum lib 137 + bin 5、rustify-ui 163、rustify-components 89、xtask 32、data-workbench 39、property-workbench 7、component-catalog 5、rustify-makepad 3、fusion-basic 0 |
| clippy（host，全工作区） | `mbx clippy --workspace --all-targets -- -D warnings` | exit 0 |
| wasm 编译与 lint | `mbx check -p vellum --target wasm32-unknown-unknown`；`mbx clippy -p vellum --target wasm32-unknown-unknown` | 通过。改动文件无告警；余下都在未改动的文件里（`painter.rs`、`pointer.rs`、`raster.rs` 的死代码与 lint，`dialogs.rs:153,271`、`topbar.rs:7` 的 clippy 提示），与 M6 记录一致 |
| release 构建 | `flock /tmp/build.lock mbx xtask build-web --example vellum --release` | 通过，173 s；wasm 20,225,213 B（M6 记录 20,229,672 B）、JS 247,080 B、CSS 32,652 B |
| 旧 toast 路径已删 | `rg -n "toast::show\|toast_serial\|shell\.toast\|\.toast\s*=\|defer_after\(3200" examples/vellum/src` | 0 行（exit 1） |
| 字面模式 | `rg "shell::toast\|toast_serial" examples/vellum/src` | 13 行，均为保留的 `shell::toast(editor, msg)` 薄封装调用（现委托 SDK）与 `<crate::shell::toast::Toast editor/>` 标记挂载点；`toast_serial` 0 处 |
| 旧草稿状态已删 | `rg -n "focused\|dirty" examples/vellum/src/shell/fields.rs` | 仅剩颜色选择器的 `picker_dirty`（非草稿逻辑） |
| 依赖与资源（D17/D18、C-9） | `git diff --quiet -- examples/vellum/{Cargo.toml,app.css,index.html,app.js}`；`rg "rustify.components\|rustify_components" examples/vellum` | 均未变；Vellum 不依赖 `rustify-components` |
| NFR-7 保持 | `rg "rustify_makepad::(listener_options\|defer\|defer_after)" examples/vellum` | 0 行 |
| DOM 不变（C-9） | 审阅 `view!` 差异 | `#toast` 与两个 `input` 的标签、属性逐字不变；未加 ARIA |
| Vellum 回归层 | `npx playwright test -c tests/vellum/playwright.config.ts` | **未跑，留给主 agent**。含 `m5-shell` 本次新增的 Escape 断言（写了但未运行） |
| 基线孪生视觉门（NFR-4、§9.4） | `VELLUM_TWIN=baseline VELLUM_BASELINE_DIR=<基线工作树> RUSTIFY_TIER=evidence npx playwright test -c tests/vellum/playwright.config.ts visual.spec.ts` | **未跑，留给主 agent**。截图前 `prepareVisual` 会隐藏 `#toast`，改动的字段不在截图状态里，预期 8 张视图差异为 0 |

### 缺口与说明

- 行为差异 1–3 按 `Draft`（ADR-4）的规则处理，差异 4 保留 Vellum 的规则，理由见上表。若要让 Escape 在 Vellum 字段里恢复成无作用，只能不用 `Draft`，与 M7「Vellum 改用草稿核心」冲突，需主 agent 决定。
- 需要真实浏览器确认的只有差异 1 的新断言，以及 spin 按钮 `change` 路径（旧新两份实现都靠同一个 `change` 事件，未改变）。
