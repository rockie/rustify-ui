# M2 子任务 · fusion-basic 原地复位与回归层瘦身 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M2（ADR-6 修订、D3、D4「M1 修订」、D5、§5.1、§9.2「复位等价」）
- 最近更新：2026-09-25 UTC
- 代码基线：dirty@`fa76b20`（M2 并行工作树，其他示例的改动由各自子任务记录）
- 范围：`examples/fusion-basic/**`；`fusion-basic` project 的 9 个 spec（`m1-probes`、`m2-runtime`、`m3-state`、`m4-geometry`、`m4-overlay`、`m6-mainpath`、`p2-navigation`、`p3-instances`、`p3-b0`）；`tests/browser/support.ts` 中 `waitForReady` 的 fusion 段。未动 `p3-restart.spec.ts`、证据层用例、`playwright.config.ts`、`deployment` project。

## 实现记录

### `window.__fusion_basic.reset()`（原地复位）

区域创建在本环境很贵（计数 scope 重挂约 10 s，B0 首载约 7 s），所以复位不重挂任何仍健康的区域，而是把应用状态改回首载值：

1. `set_guard(false)`，恢复原生 `HTMLCanvasElement.prototype.getContext`（加载时记下的原值）。
2. 卸载保留集之外的全部 scope。保留集 = `b0`（B0 fixture）+ `scope-a`/`scope-b`（计数 fixture，`waitForReady` 挂的）；保留集中若有区域处于 `failed`/`lost`（如 `getContext` 被拒后重挂的 `scope-a`）也卸载，由下一个 `waitForReady` 重挂。无人持有的 fixture 容器用 loader 的 `release_container` 清空。
3. `history.replaceState(加载时的 state, "", 加载时的 URL)`；`data-rustify-url-owner` 若仍标着本实例则移除（owner 卸载时 SDK 已移除，这里兜底）。
4. `fusion_basic_reset()`（Rust 导出）：逐个调用存活 fixture 注册的复位闭包——B0 把 `B0State` 置回 `B0State::new()`（未变则不写）；计数 fixture 把计数置 0，并把新属性 `CounterProps.resets` 加一。
5. B0 缺失则重挂。
6. 清除所有非浮层内元素的内联样式（SDK 只在 Layer 上写内联样式）、所有元素与窗口的滚动、选区、焦点（`blur`）；截断 `hooks.runtime.errors`；删除 `__probe*` 与 `__restore_get_context` 全局，值为 `AbortController` 的先 `abort()`（用于摘掉探针挂的监听）。
7. 等一个宏任务，让 Leptos effect（B0 报告、DOM 半边、区域属性）落地。

Rust 侧（`examples/fusion-basic/src/main.rs`）：`RESETS` 注册表 + `on_reset()`（随 owner 的 `on_cleanup` 注销）、`fusion_basic_reset` 导出；`GeometryFixture` 与 `RouteFixture` 卸载时清掉自己在 `GEOMETRY`/`ROUTES` 里的报告——原先卸载后旧报告仍在，共享页上下一个 `mountGeometry` 的「区域 ready、20 个锚点」轮询会被上一个 fixture 的旧报告提前满足，`mountRouting` 同理。

区域内部状态一处（`examples/fusion-basic/src/counter_region.rs`）：Makepad 的 `Button` 按下时总是取走 Cx 的键盘焦点（不看 `grab_key_focus`），焦点态一直画着（约 55 像素，同 scope 的孪生区域因此不再逐像素相同），`pointerleave`、DOM `blur`、状态复位都去不掉，只有点区域内别处才去掉（探针见下）。计数区域在 `resets` 变化时 `cx.set_key_focus(Area::Empty)`，复位后与首载同像素。B0 的 SDK 控件不取键盘焦点，无此问题。

不复位（无法复位）：当前条目之前的历史与 `history.length`（Chrome 只留最近 50 条）、实例编号、已用的重启次数、诊断记录、`stats()` 的 pumps/frames、线性内存、`boot_instance` 起的额外实例。依赖这些的用例见下方 fresh 清单或已改为读增量。

### `waitForReady` 的 fusion 段

只挂缺失的计数 scope（以容器上的 `data-rustify-scope` 判断）。新页上行为不变（两个都缺，都挂）；共享页上复位保留了健康的两个 scope，不再重挂。

### spec 改动

- 全部 9 个 spec 从 `./support` 导入 `test`（`expect` 仍从 `@playwright/test`）。
- `p2-navigation`：两个 `sharedPage` 块改用 fixture，用例改为 `({ page })` 并去掉 `serial`。A-5 块 `beforeEach` 调 `waitForReady`；V6 块 `beforeEach` 调 `waitForReady` + `mountRouting`（路由 fixture 随复位卸载）。A-5 探针的 popstate 监听改用 `AbortController`（存于 `__probe_listening`），复位时摘掉。「twenty moves」原用 `history.length` 计数——共享页的历史会涨满 Chrome 的 50 条上限，之后长度不再变化——改用 Navigation API 的条目 key：新增条目数 = 移动次数、当前条目是最后一个新增条目、后退后回到起点 key 且条目集不变。
- `p3-b0` 与 `p3-instances` 的 `openB0`/`open` 按 `isShared(page)` 跳过 `goto`。
- `p3-instances`：见下文「GC 失败」；整文件 `describe.configure({ mode: "parallel" })`（全部是 fresh，让两个 worker 分摊）。
- 新增复位等价用例（`m3-state.spec.ts`「the page a check starts on › a reset puts back the page as it loads, and keeps its regions」），见下。

### `rounds()` / `pick()`（§5.1，回归层取值；`RUSTIFY_TIER=all` 仍是原值）

| 用例 | 原值 | 回归层 |
| --- | --- | --- |
| `m1-probes` mount and dispose repeat without leaking regions | 20 回合 | `rounds(20)` = 3（去重表中挂载/释放泄漏循环的回归层保留处） |
| `m3-state` interleaved DOM and GPU actions each land exactly once | 20 回合 | `rounds(20)` = 3 |
| `m4-overlay` a modal over the region takes a hundred clicks… | 100 次点击 | `rounds(100)` = 3（仍交替落在对话框与其下的控件上） |
| `m4-geometry` one geometry for display and for hits（视口×缩放） | 9 组 | `pick` 2 组：1024×768 @100%（共享页，`setViewportSize`）、1440×900 @125%（非整数 DPR，fixture 因 context 选项自动给新页） |
| `m4-geometry` a hundred rounds of resizing and scrolling… | 第 0–99 回合 | `pick` 第 2、10、20 回合（第 2 回合越过下边、10/20 越过右边且是「每十回合看一次」的回合）；阈值按比例：点击数 > 回合数/5、截图检查 > 看的次数/3，全量时即原来的 >20、>3 |
| `p2-navigation` twenty moves are twenty entries… | 20 次 | `rounds(20)` = 3 |
| `p2-navigation` a guard refuses a move… | 20 + 20 次 | 各 `rounds(20)` = 3 |

用例标题一律未改（`--list` 守恒按标题比对）。`m4-geometry` 另 7 组、各循环的原回合数只在 `RUSTIFY_TIER=all`（`verify --suite`）中跑，与 `tests/tier.ts` 对 `rounds`/`pick` 的定义一致。

### 删除 / 打标清单

- 删除：无。
- 新打 `@evidence`：无。
- 去重表：挂载/释放泄漏循环由 `m1-probes.spec.ts`「mount and dispose repeat without leaking regions」（`rounds(20)`）在回归层保留；`m2-runtime.spec.ts`「repeated mount and dispose returns every browser resource」（300+100 回合）M1 已打 `@evidence`，本次未动。
- `--list` 守恒：`RUSTIFY_TIER=all` 下去掉行列号后与 M1 的 `all-fusion-basic` 清单相比只多出新增的复位等价用例（79 → 80）；回归层 70 → 64（`m4-geometry` −7，复位等价 +1）。

### fresh 清单（回归层 64 项中 17 项付一次页面加载）

| 用例 | 原因 |
| --- | --- |
| `m1-probes` no dynamic JS execution is needed and the bridge hash matches | 控制台 CSP 监听须在页面脚本运行前挂上 |
| `m2-runtime` every mount in the runtime is dead, and the page says which | trap 杀死实例 |
| `m4-geometry` one geometry…, 1440x900 at 125% | DPR 是 context 选项（fixture 自动） |
| `p2-navigation` the browser takes it, and the SDK does not | 导航离页；改为只 `goto` + 挂 owner，不再挂用不上的两个计数 scope |
| `p3-b0` ten DOM controls, twenty GPU controls, one region, one scope | 数的是「加载后的页面上有什么」，共享页上还有两个计数 scope |
| `p3-b0` the first screen takes one font, and it is the Latin one | 首载资源计时 |
| `p3-instances` ×9（四种 trap、诊断、策略、owner 死后重启、第二 owner 被拒、耗尽重启） | trap / `boot_instance` 起的实例无法卸载 / 耗尽重启 |
| `p3-restart` ×2（未改，仍从 `@playwright/test` 导入） | trap 与重启 |

与 M1 子任务统计（24）相比：`m4-geometry` 9 → 1（`pick` + DPR 1 走共享页）；`m2-runtime`「a region denied a GPU context」不再 fresh（复位恢复 `getContext` 并卸载区域失败的 scope）；`p3-b0`「ten DOM controls…」新增为 fresh（见表）。每次运行的页面加载次数：17 个 fresh + 复位等价用例的 1 个参照页 + 每个 worker 1 个共享页（改造前 70 次，每次加载后还要挂两个计数 scope）。

### 复位等价用例

参照：在独立 context 的新页上 `waitForReady` 后取快照，然后关闭该 context。被测：worker 的共享页（断言 `isShared(page)`），快照先与参照比一次（即前面用例留下、经 fixture 复位后的状态），给 `b0`/`scope-a`/`scope-b` 的 5 个 canvas 打上 JS 标记，再做改动：两个计数（DOM 与 GPU 两路，GPU 按钮因此持有键盘焦点，断言此时孪生区域像素不同）、B0 的 count/title/visible/size、挂 geometry 并给其 canvas 写内联宽度、滚内层、挂 owner+guest 并点 `/one`、`pushState("?pushed#here")`、`set_guard(true)`、页面滚动/选区/`body` 内联样式/焦点、错误列表加一条、`getContext` 换成包装、`__probe*` 全局与挂在其上的 popstate 监听。断言改动后快照 ≠ 参照；然后 `reset()` + `waitForReady`，断言：

- 快照 = 参照。快照含 `b0_state()`（含 region 状态与 20 个控件矩形）、两个计数的 DOM 值、`region_states()`、`live_regions()`、`live_components()`、`routes()`、URL（含 search/hash）、`history.state`、URL owner 标记、`errors()`、带 `data-rustify-scope` 的容器、各 fixture 容器子节点数、canvas 数、内联样式元素数、滚动、选区、焦点元素、`__probe*` 全局、`getContext` 是否原生；
- 5 个 canvas 仍带标记（区域未重建）；
- `scope-a-gpu-1` 与 `scope-a-gpu-2` 逐像素相同（区域已按复位后的属性重画，按钮焦点已放下）。

注：不同 scope 的两个计数区域在新页上也不逐像素相同（约 2,947 像素差），所以像素对比只在同一 scope 的两个区域之间做。

### `p3-instances` 的「Resulting promise was garbage collected」

主 agent 基线里 `p3-instances:98`×4、`:132`、`:156` 失败，均在 `page.evaluate(() => boot_instance("instance-two", "mount"))`。探针（`scratchpad` 中的独立 Playwright 脚本，新 context、首载 ready 后执行，各 2 轮）：

| 写法 | 结果 |
| --- | --- |
| 直接返回 `boot_instance(…, "mount")` 的 promise（spec 原写法） | 4/4 报「Resulting promise was garbage collected」，但第二实例其实已启动（`fatal() = null`、`live_regions() = 2`） |
| promise 同时挂在 `window` 上，再返回其结果（anchored） | 2/2 通过 |
| promise 与结果都挂 `window`，另行轮询 | 4/4 通过 |
| 只启动不建区域（`mount_trap` fixture） | 4/4 通过 |
| 先启动（`mount_trap`），另一次 evaluate 里再挂计数 scope | 4/4 通过 |

结论：DevTools 协议对 evaluate 返回的 promise 只持弱引用。在这个浏览器构建（Chromium 141 headless shell，Playwright 1.63 期望 153）里，settle 前启动 GPU 区域的那个 promise 会被报告为已回收，而页面内的实例启动本身是成功的。这是环境侧的行为，但测试侧有一个合法且最小的修正：在页面里保持 promise 可达。`p3-instances` 新增 `bootSecond(page, fixture)`，启动期间把 pending promise 挂在 `window.__booting` 上，返回它的结果，结束后删除。三处 `"mount"` 调用改用它；`"mount_owner"` 那处本就以 reject 结束，未改。修正后 9/9 通过（见验收记录）。

## 验收记录

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| 复位等价用例通过 | `npx playwright test --project=fusion-basic m3-state.spec.ts p3-instances.spec.ts --workers=2` | 本机（4 核、15 GiB、SwiftShader），与其他子任务共用机器，按 `/tmp/pw.lock` 串行 | 12/12 通过；复位等价 29.7 s（含参照页加载）；此后四次全量运行中也都通过（28.8–29.4 s） |
| 回归层全绿（第 1 次，p3-instances 并行前） | `npx playwright test --project=fusion-basic --workers=2` | 同上 | 64/64 通过，311 s |
| 回归层连续两次全绿（p3-instances 并行后） | 同上，连跑两次 | 同上 | 64/64、273 s（4.5 min）；64/64、274 s（4.6 min） |
| 单 worker 计时 | `npx playwright test --project=fusion-basic --workers=1` | 同上，重新构建后（与上两次只差 `app.js` 一处注释） | 64/64、497 s（8.3 min）；连同上两次共三次连续全绿 |
| `--list` 守恒 | `RUSTIFY_TIER=all npx playwright test --project=fusion-basic --list`，去行列号后与 M1 清单 diff | 同上 | 只多出复位等价用例（79 → 80）；回归层 64 |
| host 检查 | `cargo fmt --all -- --check`；`CARGO_TARGET_DIR=/root/tgt-fb mbx clippy -p fusion-basic --all-targets -- -D warnings`；`CARGO_TARGET_DIR=/root/tgt-fb mbx test -p fusion-basic --bins` | 同上 | 全部通过（host 上 bin 只有 `fn main`，0 个测试） |
| wasm 目标 lint（补充） | `mbx clippy -p fusion-basic --target wasm32-unknown-unknown --no-deps -- -D warnings` | 同上 | 新代码无告警；报出的 2 处是既有代码（`main.rs:30` 未用的 `pub use`、B0 `Layout` 分支多余的 `return`），未改 |

改造前对照（主 agent，`fa76b20`，同一命令，机器负载重）：15.0 min，64 过 / 6 败（均为上述 GC 失败），每个用例约 26 s（一次页面加载 + 两个计数 scope，共 5 个区域启动）。

### 耗时构成（按文件合计，秒）

| spec | workers=2 第 2 次 | workers=2 第 3 次 | workers=1 |
| --- | --- | --- | --- |
| `p3-instances`（9 个 fresh） | 130.5 | 131.1 | 133.9 |
| `m2-runtime` | 113.2 | 110.2 | 85.9 |
| `m1-probes` | 107.2 | 107.5 | 105.0 |
| `m4-geometry` | 49.7 | 47.7 | 36.9 |
| `m3-state` | 33.1 | 33.2 | 40.2 |
| `p3-restart`（未改） | 29.2 | 30.3 | 30.0 |
| `m4-overlay` | 23.9 | 24.4 | 22.4 |
| `p2-navigation` | 18.1 | 18.5 | 17.3 |
| `p3-b0` | 18.0 | 18.3 | 16.8 |
| `m6-mainpath` | 4.7 | 4.9 | 4.7 |
| 合计 / 墙钟 | 528 / 273 | 526 / 274 | 493 / 497 |

- 目标（D9：本地单示例 ≤ 5 min）：workers=2 达标（4.5–4.6 min）；workers=1 为 8.3 min，未达标。
- 剩下的时间几乎都是区域启动：
  - 每个 worker 第一次打开共享页并挂两个计数 scope，约 24 s。
  - 每个 fresh 用例约 6–27 s（只有 B0 的约 6–7 s，要挂计数 scope 或第二实例的 13–27 s）。
  - 用例卸掉 `scope-a` 后，下一个用例重挂它，约 10 s。`m2-runtime` 有 6 个用例以卸掉 `scope-a` 为测试对象，这一项约 60 s；`m1-probes` 的泄漏循环每回合重挂一次，3 回合约 28 s。
  - 共享页上其余用例 1–3 s。
- 最慢的三个文件及原因：
  - `p3-instances`：9 个用例都要 trap 或起第二实例，每个都是新页，加上第二实例的两个计数区域。
  - `m2-runtime`：如上，卸掉即重挂；另有 1 个 trap 的新页。
  - `m1-probes`：共享页首开、泄漏循环、CSP 新页。
- `p3-instances` 设为并行后，两个 worker 的用例时长合计从 308/210 s 拉平到 255–271 s，墙钟从 311 s 降到 273 s。

## 给主 agent 的注记

- `tests/browser/support.ts` 的 `Window["__fusion_basic"]` 类型里没有 `reset()`。复位等价用例用 `as unknown as { reset(): Promise<void> }` 调用，fixture 也是这样调的。若要补类型，只需在该类型里加一行。
- fixture 的共享 slot 用「第一个拿到它的用例」的 context 选项开页。如果某个 worker 的第一个用例（或某个用例失败、worker 重启后的第一个用例）带 `deviceScaleFactor ≠ 1`，共享页就按那个 DPR 打开，之后该 worker 上所有默认选项的用例都被判为 `differs`，改走新页。这样仍然正确，只是慢。fusion-basic 里唯一的 DPR≠1 用例排在 DPR 1 的同类用例之后，本次三轮都没有碰到。
- 本环境只在同一 scope 的两个计数区域之间逐像素一致，不同 scope 之间差约 2,947 像素（新页即如此）。
