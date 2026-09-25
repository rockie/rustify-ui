# M2 子任务 · data-workbench 页内复位与回归层瘦身 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M2（ADR-6 及其修订、D3、D4 及「M1 修订」、D5、§5.1、§9.2「复位等价」）
- 最近更新：2026-09-25 UTC
- 状态：实现与复位等价用例通过；回归层两次 `--workers=2` 各有 2 个**改造前即失败**的用例（见「缺口」），其余全绿
- 代码基线：dirty@`fa76b20`（与其他示例的 M2 子任务同一工作树）
- 改动文件：`examples/data-workbench/src/main.rs`、`examples/data-workbench/app.js`、`tests/browser/p3-table.spec.ts`、`tests/browser/p3-jobs.spec.ts`、`tests/browser/p3-scene.spec.ts`

## 实现记录

### 先测成本，再选复位方式

探针脚本（会话临时目录 `cost.mjs`，未入库）：Desktop Chrome 1440×1200，每步从开始计到 ready（场景另等 `scene.drawn > 0`）+ 500 ms 安静，各 3 次取中位。

| 操作 | 中位 | 说明 |
| --- | --- | --- |
| 新 context 首载 `/table` | 1,582 ms | 表格路由**有**区域（选区条 `table-strip`），但只用 `DrawColor`，创建便宜 |
| 新 context 首载 `/scene` | 4,348 ms | 场景区域带文字（字体图集），创建约 3.5 s |
| 同页 `dispose()`+`mount()`（表格） | 799 ms | 选区条区域重建约 0.25 s |
| 路由 表格→场景 | 3,883 ms | 付一次场景区域创建 |
| 路由 场景→表格 | 850 ms | 付一次选区条区域创建 |
| `reset("/table")` 原地（表格） | 667 ms | 其中 ≥500 ms 是安静窗口本身 |
| `reset("/table")` 原地、样本已改（需再生） | 742 ms | 样本再生 90 ms（`generated_ms` 89.2/90.4） |
| `reset("/scene")` 从表格 | 4,023 ms | 等于路由切换：场景区域必须新建 |
| `reset("/scene")` 原地（场景） | 1,259 ms | 保留区域，只重画一次相机 |
| `reset()` 从场景 | 834 ms | 等于路由切换：选区条区域新建 |
| `dataset_hash()` | 70 ms | |

结论：表格路由的区域便宜，重挂与原地复位只差约 0.13 s/次；场景区域昂贵，原地复位省 2.8 s/次。选**原地复位**（最便宜、且保留区域）：应用把状态置回首载值，已显示的区域保留；路由变化时照常换区域，与点链接等价。三次首载快照逐字段一致（`window_version` 表格 3、场景 0；场景 `reported` 1、`pane` [1116, 983]、`drawn` 260），只有 `generated_ms`（计时）不同，所以等价用例只剔除这一项。

### `window.__data_workbench.reset(path = "/")`

- **Rust 导出 `data_workbench_reset(path)`**（`src/main.rs`）：
  - 实例级（无论是否挂载）：`SORT` 探针置空；样本 `version != 0`（任何写入、插入、删除都会升版本）时先丢弃旧样本再 `Dataset::generate()`（让新样本复用旧内存，不再长一份 32 MB），同时刷新 `GENERATED_MS`。未改动的样本不重生成，保持便宜。
  - 挂载级：关闭旧 `Requests` 并换新，在途任务一律不得交付；`JOB_SEQ` **加一而不归零**——复位视为比所有在途任务更新的请求，旧任务结束时不再被当作「最新」去改状态行（首次实现归零/不动时，等价用例抓到旧任务的 `cancelled` 写回状态行）；它只用于比较相等，绝对值不影响行为。
  - `TableState::reset(rows)` 与 `SceneControls::reset(region_stays)`：各字段写回首载值；`new()` 也经由 `reset()` 赋初值，两者不会漂移。场景的 `drawn`（区域上次绘制的报告）仅在区域保留时保留，否则清零，免得新区域未画前被旧报告冒充。
  - 视图局部状态（输入框文本、树的展开与键盘位置、表格滚动与视口测量、详情草稿与表单）：拆出 `TableControls`、`SceneControlsBar`，与 `Tree`、`DataTable`、`Details`、`SceneDetails` 一起包进 `Rebuilt`（`<For>` 以 `epoch` 为键），复位时同路由则 `epoch += 1`，**连节点带状态**重建；区域不在其中，保持不动。首次用 `{move || { epoch.track(); … }}` 实现时，等价用例抓到滚动容器上测试留下的内联样式仍在、`window_version` 为 2≠3——渲染器把同形视图重建进旧节点，故改为按键重建。
  - 路由：挂载时把 `Owner::current()` 存入 `MOUNTED`，复位在该 owner 内调 `navigate(path, true)`，走应用自己的路由（`/` 照常经 Effect 重定向到 `/table`），无页面加载；视图判定抽成 `view_of()`，`showing` 与复位共用一条规则。`on_cleanup` 清空 `MOUNTED`。
  - 返回是否有已挂载的工作台；没有时由 JS 处理。
- **JS**（`app.js`）：截断 `hooks.runtime.errors`；失焦、`window.scrollTo(0, 0)`；删 `scene-gpu` 上的 `moves`；清空 `#scratch`；调导出；若无挂载（用例 `dispose()` 过）则 `history.replaceState` 到 `path` 再 `mount()`（挂载从地址读路由）；然后等 `path` 对应视图出现、场景另等 `drawn > 0`（上限 60 s），再等一帧。表格滚动容器的内联样式随 `DataTable` 重建消失。
- **不复位的线程局部**：`HANDLES`/`NEXT_HANDLE`（挂载句柄，永不复用，不属于应用状态）；诊断记录（SDK 无清空接口）——因此 `p3-jobs`「an insert makes the job stale」改为只数本用例开始后（`at_ms >= performance.now()`）记下的 `JobCancelled`。

### spec 改动

- 三个 spec 改从 `./support` 导入 `test`（`expect` 仍来自 `@playwright/test`，`support` 不导出它）；各自的 `openTable`/`openScene` 经 `arrive()`：共享页调 `reset("/table")`/`reset("/scene")`，自有页照旧 `goto`，之后等待条件不变。视口仍由它们先设 1440×1200。`p3-table` 两个 1100×600 用例同样改用 `arrive()`。
- `openScene` 末尾加 `waitForQuiet`：场景区域刚建时字体图集阻塞主线程数秒，点击与计帧不应落在其中（与 fixture 的 ready + quiet 同理）。
- `rounds()`/`pick()`（D3，§5.1「回归层（降回合）」）：
  - `p3-table`：选区条点击 `rounds(20)`，点击位置按实际次数铺满整条；14 个表格定位器 `rounds(30)`，重画间隔 `max(1, round(n/6))`（30 回合仍是每 5 回合，3 回合则每回合都重画）。
  - `p3-scene`：二十个框选 `rounds(20)`；拖动步数 `rounds(100)`，期望相机位移按步数算；百次点选 `pick(全部, [第一个上层点, 前两个下层点])`，两层都覆盖；「a hundred each way」查询 `rounds(100)`，点选 `max(rounds(100), 51)`——列表只显示 50 个，少于 51 就验不到「未列出的仍被计数」；场景 6 个定位器 `rounds(30)`。
  - `p3-jobs` 除 fixture 与上述诊断计数窗口外不动（含 20 次排序计时，非证据标签）。
- 标题一律未改；`--list` 守恒见验收表。

### fresh 与删除清单

| spec:用例 | 处理 | 原因 |
| --- | --- | --- |
| `p3-table`「review · size observations stop on unmount and fatal」 | `fresh`（匿名 describe，标题不变） | `addInitScript` 须先于页面运行；用例末尾 `enter_fatal` 杀掉实例（D4 c、e） |
| `p3-table`「a reset leaves the page as a first load of the same address does」（新增） | `fresh` | 比较对象是首载，须自己加载两次 |
| `p3-restart` ×2 | 不改，自开页 | 按要求不动 |
| 其余 47 个 | 共享页 | m1-fresh.md 所列「改写 `DATA`」11 个与「仅首载到路由」36 个均由复位覆盖 |

导航次数（每个 worker）：共享页打开 1 次；失败用例后 fixture 换页 1 次/失败；`fresh` 两个用例共 3 次导航；`p3-restart` 各自加载。

**删除（D5）**：`p3-scene`「every one of the twenty is findable by role and name」中表格段（点 `go-table`、`open_row(0)`、14 个表格定位器 ×30 回合）删除，**由** `p3-table`「the table's fourteen named things are findable, thirty times over」（`rounds(30)`，同样的角色+名称交集断言）保留；`p3-scene` 该用例只查场景 6 个，`LOCATORS` 总数 20 的断言保留。

### 缺口

- `p3-scene`「a camera change is presented in the frame it was made in」：改造前（新页）与改造后（共享页、已等安静）都只有 10 次驱动/3 s（`drives > 30` 不成立；比例断言本身成立：9 次呈现/10 次驱动）。本容器 SwiftShader 画这一屏约 300 ms，注释假设约 70 ms；M1 记录的 macOS CI 上此用例同样失败。属环境/阈值问题，本任务未改阈值。
- `p3-table`「review · resizing at the end preserves the focused cell」：改造前后都确定失败——1440×1200 缩回 1100×600 后最后一行不在窗口内、焦点被 `DataTable` 夹回可见区（`crates/rustify-components/src/data_table/view.rs` 的焦点 Effect），元素消失。组件行为，不在本任务可改文件内；M1 记录的 macOS CI 上此用例通过，疑与平台滚动条/视口行为有关，未深究。
- 因此「`--workers=2` 连续两次全绿」未达成：两次都是 49 过 / 2 败，失败集合与改造前基线完全相同，没有新增或顺序相关的失败。

## 验收记录

环境：Linux x64 容器（4 核、15 GiB、无 GPU），rustc 1.98.1、mbx 1.15.0、Node 26、Playwright 1.63.0，Chromium 141 headless shell（M1 记录的垫片）；其他子任务的 Playwright 运行经 `/tmp/pw.lock` 串行，构建经 `/tmp/build.lock`。

| 检查 | 命令或步骤 | 结果 |
| --- | --- | --- |
| 改造前基线（回归层，旧 spec + 旧构建） | 冻结的 spec 副本 + 包装 config，`npx playwright test --project=data-workbench`（workers=1） | **460 s（7.7 min）**，47 过 / 3 败：`p3-scene:29`（10 次驱动）、`p3-scene:419`（2.1 min 超时）、`p3-table:431`（焦点） |
| 成本探针 | 见「先测成本」 | 表格区域便宜、场景区域约 3.5 s；原地复位最省 |
| 复位等价 | `npx playwright test --project=data-workbench p3-table.spec.ts -g "a reset leaves the page"` | 通过（24.7 s）：首载 `/scene` 与 `/` 各取状态；改表格（排序、过滤、选择、编辑单元格并保存、插入、删除两次、收起树、跳行、查找框、滚动容器内联样式、留一条 `errors`、`#scratch` 塞节点、留一个在途排序）+ 去场景（相机、查找、点选、改名、`moves`）后 `reset()` = 首载 `/`；再改表格后 `reset("/table")` = 首载 `/`；`reset("/scene")`（表格→场景）= 首载 `/scene`；再改场景后原地 `reset("/scene")` = 首载 `/scene`。比较项：`snapshot()`（去 `generated_ms`）、`dataset_hash()`、URL、视图前 30 行、选择、场景选择、区域数、`errors`、焦点、页面滚动、滚动容器滚动与内联样式、带 testid 的输入框值、展开节点、`aria-sort`、前 5 行 id、详情标题、场景列表项数、`moves`、`#scratch` 子节点数 |
| 回归层 `--workers=2` 第 1 次 | `flock /tmp/pw.lock npx playwright test --project=data-workbench --workers=2` | **166 s（2.8 min）**，49 过 / 2 败（上述两个既有失败） |
| 回归层 `--workers=2` 第 2 次 | 同上 | **165 s（2.7 min）**，49 过 / 2 败（同上两个） |
| 回归层 `--workers=1` | 同上 `--workers=1` | **220 s（3.7 min）**，49 过 / 2 败（同上） |
| 两个失败的确定性 | `… p3-scene.spec.ts:58 p3-table.spec.ts:462 --repeat-each=2` | 4/4 失败，与基线同因（10 次驱动；元素不存在） |
| `--list` 守恒 | 改前（冻结副本）与改后各列 `RUSTIFY_TIER=regression`、`all`，去行列号比对 | regression 50→51、all 61→62，差集只有新增的复位等价用例；无标题改动、无删除用例 |
| 单测 | `CARGO_TARGET_DIR=/root/tgt-dw mbx test -p data-workbench --bins` | 39 passed |
| clippy | `CARGO_TARGET_DIR=/root/tgt-dw mbx clippy -p data-workbench --all-targets -- -D warnings` | 通过；另跑 wasm32 目标的 clippy，本示例只有改造前就有的 `chunks_exact_to_as_chunks`（`Details`，未改） |
| 格式 | `cargo fmt --all -- --check` | 通过 |

耗时对比（墙钟）：基线 460 s（workers=1）→ 改造后 220 s（workers=1）/ 166 s（workers=2），两次 workers=2 为 166 s 与 165 s，均 ≤ 5 min。逐文件用例时长合计（workers=1）：基线 `p3-scene` 358 s、`p3-table` 52 s、`p3-jobs` 40 s；改造后 `p3-scene` 103 s、`p3-table` 66 s（含 24 s 的复位等价用例）、`p3-jobs` 38 s（`p3-scene` 的下降主要来自降回合：「a hundred each way」2.1 min 超时→29 s，「a hundred clicks」66 s→7 s，「a hundred steps」45 s→7 s），表格与任务类用例在共享页上 1.1–2.9 s/项，与基线新页相当（表格首载本就只约 1.5 s，共享页在这里省不下多少）。
