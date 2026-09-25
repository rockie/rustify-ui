# M2 子任务 · component-catalog 原地复位与回归层改造 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M2（ADR-6 及其「修订」、D3、D4 及其「M1 修订」、D5、§5.1 去重表、§9.2「复位等价」）
- 最近更新：2026-09-25 UTC
- 范围：`examples/component-catalog/**`；`tests/browser/p2-catalog`、`p2-theme`、`p2-semantics`、`p2-i18n`、`p2-reflow`、`p2-a11y` 六个 spec。`p3-restart`、`p3-policy`、`tests/browser/support.ts`、`playwright.config.ts` 未改
- 代码基线：dirty@`fa76b20`（与其他 M2 子任务同一工作树，未提交）

## 实现记录

### `window.__component_catalog.reset()`：原地复位

- **偏差（需主 agent 回写计划）**：D4「M1 修订」原文把 component-catalog 归为「已达 D9 目标、按原文重挂」。本环境实测，这个示例的区域创建约 6.5 s，同页 `dispose()`+`mount()` 与整页加载几乎一样慢（加载 6.6 s，重挂 6.2–6.6 s）。所以这里和 property-workbench 一样做**原地复位**：保留已挂载的 scope 和它的区域，由应用把自己的状态置回首载值。
- **Rust**（`examples/component-catalog/src/main.rs`）：
  - `FirstLoad`：应用状态的信号都经 `first.signal(初值)` 创建，初值只写一次，就写在创建处；它同时登记一个「置回初值」的闭包。语言不是本 scope 的普通信号（`provide_locale` 返回句柄），用 `first.then(...)` 登记。
  - `Catalogue` 新增 `handle: u32` 属性。`catalog_mount` 改为先分配句柄再挂载；挂载失败会跳过一个编号，句柄本来就不复用，所以没有影响。组件末尾调 `first.register(handle)`，把本 scope 唯一的一个复位闭包放进 thread_local `RESETS`，并用 `on_cleanup` 让它随 scope 一起移除。
  - 新导出 `catalog_reset(handle) -> bool`：先把闭包从表里取出来，再执行它。
  - **复位**：主题（含减少动效）、语言、内存路由 `page`、`Values` 的全部 15 个值（勾选、单选、开关、滑块、进度、转圈、标签页、文本、备注、下拉选择、tooltip/menu/dialog/listbox 是否打开、动作计数），以及区域下拉列表 `region_chooser` 是否打开。
  - **不复位**（它们镜像的是活着的区域）：`region`（区域状态）、`region_button`、`region_control`（区域上报的矩形）。区域保留，会接着上报。
  - **页面局部状态**：数据表选择、树的展开与选中、样本页的「阻断宽字体」（block-font）都属于各自的页面组件。Leptos 0.8 的 `set` 即使值没变也会通知，所以 `page` 置回首个类别时，当前页一定重建，这些状态随之回到初值，不必逐个登记。
  - **snapshot 扩展**：新增 `notes`、`tooltip`、`menu`、`dialog`、`listbox`（Select 的列表）、`region_list`（区域下拉列表），使快照覆盖复位所恢复的全部应用状态。`text` 与 `notes` 改用浏览器 `JSON.stringify` 编码：`notes` 的初值带换行，按原来的拼法会产出非法 JSON。
- **JS**（`examples/component-catalog/app.js`）：`reset()` 按以下顺序执行。
  1. 卸载第二 scope（`catalog-second`）。
  2. 主 scope 若已被卸载就重新挂载（只有这时才付区域创建）；否则调 `catalog_reset(handle)`。
  3. `settled()`：逐帧等待，直到 `region === "ready"`、`button` 与 `control` 都已上报、`live_regions() === 1`（样本页和第二 scope 的区域都已释放）。首个类别 button 正是区域会画的控件，所以 `control` 一定会上报。60 s 内未满足则抛错，让 fixture 把这一页判为不健康。
  4. 让焦点离开当前元素（`blur`）。放在浮层关闭之后，因为浮层关闭时会把焦点交还给打开它的元素。
  5. `scrollTo(0, 0)`。
  6. 截断 `hooks.runtime.errors`。
  - 本示例 `url_owner: false`，不动地址栏。句柄对象改为先存进局部 `catalog` 再赋给 `window`，`reset()` 不依赖 `this`。
- **SNAPSHOT 仍是全局的（沿用原有行为）**：两个 scope 共写一个快照槽，谁的 effect 最后运行，就读到谁的快照。复位先卸第二 scope，再置回主 scope 的全部信号（`set` 必然通知），所以主 scope 的 effect 最后写，复位后读到的一定是主 scope 的快照。挂着第二 scope 时读 `snapshot()` 不可靠：等价用例只在挂第二 scope 之前核对快照，挂之后改查 DOM。

### spec 改造

| spec | 改动 |
| --- | --- |
| `p2-catalog` | `test` 改从 `./support` 导入（`expect` 仍从 `@playwright/test` 导入，`support.ts` 未导出 `expect`）。`ready()` 按 `isShared(page)` 跳过 `goto`。「boots with no policy violation…」所在 describe 加 `test.use({ fresh: true })`。`sharedPage` 块改用 fixture，去掉 serial。删掉 4 处只为「给下一个用例复原页面」的收尾操作。删除 stylesheet 用例（见下文删除清单）。新增复位等价用例 |
| `p2-theme` | fixture；去 serial。「twenty switches…」的回合数改为 `2 * rounds(10)`，保持偶数，回归层 6 次、全量层 20 次；页内结果数组改名 `taken`，避免遮蔽导入的 `rounds`。删 2 处复原点击 |
| `p2-semantics` | fixture；去 serial。「thirty rounds…」改为 `2 * rounds(15)`（回归层 6 次 / 全量层 30 次）。删 1 处复原点击 |
| `p2-i18n` | 两个 `sharedPage(openSamples)` 块改为 `test.beforeEach(openSamples)`。「twenty switches…」改为 `2 * rounds(10)` |
| `p2-reflow` | 改用 fixture 的 `test`；去 serial（`waitForReady` 已会跳过共享页的 `goto`） |
| `p2-a11y` | fixture；去 serial |

- **未套 `rounds()` 的循环**：p2-a11y 最多 300 次 Tab 是「直到焦点绕回起点」的上限，不是重复同一行为；模态层内的 20 次 Tab 用来走完一整圈焦点顺序（层内 → 宿主页 → 回到文档），砍到 3 次就走不到「焦点落到模态背后」可能出现的位置；类别循环与 NAMED 表要求覆盖面完整。
- **用例标题一律未改**（`--list` 守恒按标题比对），所以 「twenty/thirty」在回归层实际是 6 回合。

### `fresh` 用例与导航次数

| 用例 | 原因（D4 分类） |
| --- | --- |
| `p2-catalog`「boots with no policy violation and draws its region」 | (a) 控制台 CSP 监听必须在页面加载前挂上 |
| `p2-catalog`「the page after a reset is the page after a load」 | 复位等价要从真正的首载开始比对 |
| `p3-policy`「the page boots under the release policy…」 | (c,a) `addInitScript`；该文件不在本任务范围，仍从 `@playwright/test` 导入，每个用例一个新 context |
| `p3-restart` ×2 | (e) 实例 trap 并耗尽重启次数；同上，不在本任务范围 |

- 每次运行的页面加载：workers=1 时 6 次（共享页 1 次 + fresh 5 次），workers=2 时 7 次（每个 worker 各一张共享页）。改造前是 18 次：p2-catalog 自开页 7 次、6 个 `sharedPage` 块 6 次、p2-reflow 2 次、p3 3 次。
- **不是导航、但仍要付区域创建的用例**：p2-i18n 的 9 个用例和 p2-reflow「every category is still reachable」都会打开样本页，而样本页有自己的区域，每次约 3.5 s。复位要回到首页，所以样本页的区域不能跨用例保留。p2-catalog「two scopes keep their own themes」和等价用例各挂一次第二 scope，也各付一次区域创建。

### 删除清单（D5）

| 删除 | 由谁保留 |
| --- | --- |
| `tests/browser/p2-catalog.spec.ts`「the stylesheet the page loads is the one the components were compiled against」（原 57–74 行） | 「无 preflight、无裸元素选择器」：host 单测 `crates/rustify-components/src/lib.rs` 的 `stylesheet::the_stylesheet_resets_nothing_and_selects_no_bare_element`（覆盖 `@layer base` 与 html/body/input/button/a/select/textarea/`*`）。「目录用到的类都有规则」：`mbx xtask css --check`（已提交的 `rustify.css` 与类字符串一致）。`build-web` 原样复制这个已提交文件（`xtask/src/build.rs:137-145`），所以页面加载的就是被检查的那份 |

计数：删 1 项、增 1 项（复位等价），回归层仍为 49 项。

## 验收记录

环境：Linux x64 容器（4 核、15 GiB、无 GPU），Node 26、Playwright 1.63.0，Chromium 141 headless shell（`PLAYWRIGHT_BROWSERS_PATH` 垫片）。release 构建由 `flock /tmp/build.lock mbx xtask build-web --example component-catalog --release` 产出。浏览器运行一律经 `flock /tmp/pw.lock` 串行，但同机还有其他 agent 在构建或跑测试，墙钟含这部分干扰。

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| 复位等价用例通过 | `npx playwright test --project=component-catalog p2-catalog.spec.ts -g "reset is as good"` | 同上 | 通过，20.3 s。用例改动了：主题、减少动效、语言、类别；勾选、文本、备注、单选、开关、滑块、标签页、转圈、下拉选择（动作计数 > 0）；样本页的字体阻断；挂上第二 scope 并切换其主题；主 scope 留一个打开的模态对话框（焦点在层内、scope 其余部分 inert）；页面滚动；注入一条运行时错误。之后调 `reset()`，比对快照、`#catalog` 的 ariaSnapshot、scope 数、区域数、错误数、`[inert]` 数、焦点与滚动位置，全部与首载相等；再进样本页，确认字体阻断已为 false |
| 全 project 回归层第 1 次（workers=1） | `npx playwright test --project=component-catalog --workers=1` | 同上 | 49 passed，**2.8 min（166 s 墙钟）**。改造前 M1 为 3.1 min（185 s） |
| 全 project 回归层第 2 次（workers=2），与上一行连续 | `npx playwright test --project=component-catalog --workers=2` | 同上 | 49 passed，**1.7 min（102 s 墙钟）**。两次都在 D9 的 5 min 以内 |
| `--list` 计数 | `npx playwright test --list --project=component-catalog`，另加 `RUSTIFY_TIER=all` 与 `RUSTIFY_TIER=evidence` 各跑一次 | 同上 | 回归层 49 项、全量层 49 项、证据层 0 项（8 个文件）。改造前为 49 / 49 / 0，删 1 增 1 |
| （作废）首次全量运行 | 同第 1 次命令 | 同上 | 43 过 / 6 败，全部是环境问题：磁盘写满，报 `ENOSPC`、`net::ERR_INSUFFICIENT_RESOURCES`、`Target crashed`。删掉本任务自己的 `/root/tgt-cc`（2.4 GB）后重跑，即上面的第 1 次 |
| host 检查 | `CARGO_TARGET_DIR=/root/tgt-cc mbx test -p component-catalog --bins`；`… mbx clippy -p component-catalog --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 同上 | 5 个 bin 单测通过；clippy 无警告；fmt 通过。另外对 wasm 目标跑了一遍 clippy（不加 `-D`，因为依赖 `rustify-makepad` 已有 `type_complexity` 警告），本次改动的代码没有新警告 |

单用例耗时（workers=1）：共享页上的用例 0.7–1.5 s，其中「fails quickly」按设计会等 5 s。p2-i18n 每项 4.1–4.8 s，主要是样本页的区域创建。复位等价用例 19.9 s，包含首载、样本页和第二 scope 两次区域创建。p3-restart 22.3 s。
