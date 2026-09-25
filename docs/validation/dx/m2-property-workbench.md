# M2 子任务 · property-workbench 原地复位与回归层改造 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M2（ADR-6 及其「修订」、D3、D4 及其「M1 修订」、D5、§5.1、§9.2「复位等价」）
- 最近更新：2026-09-25 UTC
- 范围：`examples/property-workbench/**`；property-workbench project 的回归 spec：`m3-workbench`、`m5-text`、`m5-semantics`、`m6-theme`、`m6-async`、`m6-components`、`m7-recovery`、`p2-form`、`p2-workspace`、`p2-drag`、`p2-clipboard-files`、`p2-zoom`、`p2-ime`。未改：`p3-restart`、`p3-policy`、`p2-deeplink`（与其他 project 共用或全部自开页）、证据层 spec（`m8-*`、`p3-faults`、`p3-diagnostics`）、`tests/browser/support.ts`、`playwright.config.ts`
- 代码基线：dirty@`fa76b20`（与其他 M2 子任务同一工作树，未提交）

## 实现记录

### `window.__property_workbench.reset()`：原地复位

M1 探针：本示例每创建一次区域约 7–9 s（SwiftShader 为新 WebGL 上下文编译着色器），同页 `dispose()`+`mount()` 也付这笔（8.2 s）。所以 `reset()` 保留已挂载的 scope 和它的区域，由应用把状态置回首载值，区域随随后的属性重画。

- **登记方式**（新文件 `src/reset.rs`）：`Resets` 是一个 scope 的复位闭包表（`StoredValue<Vec<Arc<dyn Fn()>>>`），`Resets::provide()` 放进 context。`resets.signal(|| 初值)` 创建信号并登记「置回初值」，初值只写一次、就写在创建处；不是单个信号、或初值依赖别的状态的，用 `resets.on_reset(...)`，按登记顺序在其后执行。`run()` 先把表拷出再执行，执行中不持有借用。
- **置回的应用状态**（`src/main.rs` 的 `Workbench`）：`objects`、`selected`、`checks`、`saving`、`saves`、`asked_result`、`panel_sizes`、`open_tabs`、`active_tab`、`palette_open`、`region_menu`、`hovered`、`hit`、`drop_target`、`dom_target`、`wheel_propagates`、`drops`、`drag_cancels`、`press`、`theme`、`editing`、`invalidated`、`refusal`、`refusals`、`find_id`、`find_result`、`details`、`rejected`、`refused`、`accepted`、`hovers`、`transfer_status`、`imports`、`exports`、`clipboard_status`、`copies`、`pastes`、`copy_by_hand`、`third_party_present`、`third_party_updates`、`emphasis`（共 41 个信号）；另用 `on_reset` 登记：
  - 表单簿记：`Form` 没有复位接口，也不需要——对 20 个字段逐个 `changed()`（丢掉错误与在途校验、作废等待中的提交），再 `submitted(Ok(()))`（清掉 dirty、failure、submitting），就是一张没人碰过的表单。代次不回到 0，但代次只用于比较，不可观察。
  - `draft`：直接置为当前对象的 details，而不是等 effect——否则 effect 运行前草稿与对象不同，导航守卫会拒绝复位最后那次导航。
  - 拖放会话在进行中就 `cancel()`；`dragged`、`exported`（`StoredValue`）清空；`Requests::cancel()` 让复位前发出、尚未返回的异步加载不能落到复位后的状态里。
  - `src/details.rs` 的 `SelectField` 自己的 `open` 也经 context 登记（下拉列表被关上）。
- **不置回的**（镜像活着的区域/运行时）：`region_state`、`controls`（区域上报的矩形）、`group_scroll`（区域上报的组列表滚动位置）。`open_link_requests` 也不置回：区域只在它大于自己记下的值时才去开链接，倒回去会让下一次请求失声。
- **Rust 导出**：
  - `workbench_reset_page(main) -> bool`：前半程。卸载除 `main` 外的全部 scope；清 `DELETED`、`CLOSE_ON_ACTION`；`set_recording(true)`；`main` 仍在就先把第三方组件取出（`PRESENT(false)`）。返回 `main` 是否仍挂载。
  - `workbench_reset_scope(address) -> bool`：后半程。去掉 base 后调该 scope 登记的 `RESET` 缝：在该 scope 的 `Owner` 下执行 `resets.run()` 再 `navigate(path, true)`。必须在 Owner 下执行，因为 `navigate` 通过 context 找路由器，从 wasm 导出直接调用时没有 Owner。
  - 第三方组件分两步重建而非就地改值：旧组件若被改过值，就地改回会多触发一次 `update` 回调，`third_party_updates` 就不等于首载值。先取出、等一个 task 让拆除跑完，后半程置回 `third_party_present = true` 与 `third_party_updates = 0`，新组件以当前值创建、恰好触发首载时那一次回调。
- **JS**（`app.js`）`reset()` 的顺序：
  1. `blur()` 当前焦点：文本控件失焦即提交，这次提交要落在即将丢弃的状态上。清掉选区。
  2. `lose_context_handles` 里仍处于丢失状态的上下文 `restoreContext()`，然后清空（登记时改为同时保存 `gl`，以便判断是否仍丢失）。
  3. `workbench_reset_page(handle ?? 0)`（句柄从 1 开始，0 表示一个都不保留）；移除 `mount_into` 建的容器（新增 `added` 集合跟踪）。
  4. `main` 在：等一个 task，`workbench_reset_scope(first_address)`。`main` 已被卸载（用例 `dispose()` 了，或 `close_on_next_action` 关掉了全部 scope）：`history.replaceState` 回首载地址后重新挂载——只有这时付一次区域创建。
  5. 等一个 task（scope 的 effect：地址跟随选择、快照），再轮询到第三方组件在场、`snapshot().region === "ready"`（60 s 超时即抛错，fixture 据此判该页不健康）。
  6. 第三方垫片计数归到首载值（`created = live.size`、`destroyed = 0`）；截断 `hooks.runtime.errors`；再次 `blur()`（浮层关闭时会把焦点还给打开它的元素）；移除 `<html>` 的内联样式（p2-zoom 设的根字号）；窗口与 `main` 内各元素滚回 0。
- **URL**：首载 `/`，应用把它替换成 `/objects/1`（等价用例断言了这一点）。复位把 `first_address`（模块求值时、启动之前记下的 `pathname + search`）经 `navigate(…, true)` 交给路由器，随后 scope 自己的 effect 再替换成 `/objects/1`——与首载走同一条路径，路由器的 location、参数与选择都与首载一致。用例 push 的历史条目删不掉；replace 保持路由器的条目编号与浏览器当前条目一致，但编号是用例留下的深度，不是首载的 0。本 project 的回归 spec 都不做前进/后退（`p2-deeplink` 自开页）。
- **snapshot 扩展**：新增 `find`（输入框与结果）、`emphasis`、`rejected`、`objects`（整个对象列表的摘要，`Memo` 只在对象变化时重算；`Details` 加 `Hash`）。

### spec 改造

| spec | 改动 |
| --- | --- |
| `m3-workbench` | `test` 改从 `./support` 导入（`expect` 仍从 `@playwright/test`，`support.ts` 未导出它）。两个 trap 用例放进匿名 `test.describe` 并 `test.use({ fresh: true })`（匿名 describe 不改标题路径）。回合：选择步进 `rounds(25)` 前进、`min(5, 前进-1)` 后退；一万动作 `rounds(10_000)`；指针流 `rounds(20)` 次点击。新增复位等价用例（见下） |
| `m5-text`、`m5-semantics`、`p2-zoom` | 只换 `test` 的来源 |
| `m6-theme` | 「twenty switches」改 `2 * rounds(10)`（保持偶数，回到起点） |
| `m6-async` | 「twenty pairs」改 `rounds(20)`，末值断言跟着回合数 |
| `m6-components` | 去重（见删除清单）；主题覆盖 `2 * rounds(10)`；第三方重建 `rounds(20)`；主题切换 `2 * rounds(15)` |
| `m7-recovery` | 除「ordinary text that looks like code」外 4 个 describe 都 `test.use({ fresh: true })`；二十次丢失 `rounds(20)`（断言随之）；十次开链接 `rounds(10)`。填满记录的 1,100 次 `mount_over` 是越过上限所需，不是重复次数，未改 |
| `p2-form`、`p2-workspace`、`p2-ime` | `sharedPage` 块改用 fixture，去掉 serial；p2-ime 的 setup 改为 `beforeEach`。p2-form 两处 `rounds(20)` 与「checks answered backwards」`rounds(20)`；p2-workspace `rounds(100)`、`rounds(10)` |
| `p2-drag` | 同上；五处循环改 `rounds()`。两处原本依赖前一个用例：「a drag onto a DOM target … takes the group away」先自己把对象拖进组；滚轮三个用例改为各自先滚到底（`toTheEnd`）——组列表的滚动是区域自己的状态，复位不动它。滚到底的 20 次、滚回顶的 30 次是到达端点所需，不套 `rounds()`。删除只为下一个用例复原滚动的收尾 |
| `p2-clipboard-files` | 前两个自建 context（只带 baseURL）的块改用 fixture；剪贴板两个用例保留自建 context（权限 / `addInitScript`） |

用例标题一律未改（`--list` 守恒按标题比对）。

### `fresh` 与自开页用例

| 用例 | 原因 |
| --- | --- |
| `m3-workbench`「a fatal drops the work the runtime had already scheduled」「a trapped runtime takes its controls off the page」 | (e) 实例 trap |
| `m3-workbench`「the application, the address and the page read as they did on the first load」 | 复位等价要与真正的首载比 |
| `m7-recovery`「comes back with the state…」「twenty losses…」 | (g) `GpuContextLost` 条目与 `runtime.errors` 的绝对数 |
| `m7-recovery`「says so within two seconds…」 | (c) `getContext` 原型桩不恢复；(g) 重试条目 |
| `m7-recovery`「is refused, recorded once…」 | (g) 启动时的拒绝条目数 |
| `m7-recovery`「the record names…」「filling it past its ceiling…」「every entry carries…」 | (g) `dropped`、条目数；填满记录 |
| `p2-clipboard-files` 剪贴板 2 个 | (c) 自建 context：授予剪贴板权限 / `addInitScript` 让剪贴板拒绝 |
| `p3-restart` ×2、`p3-policy` ×1、`p2-deeplink` ×6 | 不在本任务范围，仍从 `@playwright/test` 导入，每用例自开页 |

- 标 `fresh` 10 个；另 11 个自开页。回归层 123 项中 102 项跑在共享页上。
- 每次运行的页面加载：21 次自开页 + 共享页（`workers=1`、`workers=2` 实测各 2 次：用 fixture 的文件与用基础 `test` 的文件 worker 哈希不同，切换时 worker 重启，共享页随之重开）= 23 次；改造前约 118 次。
- 不是导航、但仍付区域创建的：`m3-workbench` 三个关掉 scope 的用例（「closing from inside…」用例内重挂；「disposed while starting」「the batch a closing region…」由下一次复位重挂）、`m6-async`「an answer for a scope that has gone」、`m6-components`「another scope on the page…」（第二 scope）、`m7-recovery` 共 4 次上下文恢复。

### 复位等价用例（§9.2）

`m3-workbench`「a reset puts the page back the way it loaded › the application, the address and the page read as they did on the first load」，`fresh`：

1. 首载后记下：`snapshot()` 全体、`location.href`、区域数、scope 数、第三方垫片统计、诊断 `recording`、`lookup_object(43)`、`runtime.errors` 长度、`aria-expanded="true"` 的数量、焦点所在、`<html>` 内联样式、`scrollY`。断言地址为 `/objects/1`。
2. 改动：悬停网格、区域「next」点选；拖到回收区一次、Esc 放弃一次；删除 43、改 42 的名字/颜色/尺寸并锁定；批量改色、反转、加 10 个；导入文件（改名 + 进组）、导出、复制；主题、强调、第三方组件、滚轮策略；拖动分隔条、关掉一个视图、切到 locked 视图；一次加载完成；表单草稿（location 与 reference，校验在途、提交等待中，守卫为真）；区域上开一个编辑会话，再由命令面板抢走焦点提交，面板保持打开并输入查询。然后在同一个 `evaluate` 里：挂第二 scope、关诊断记录、推一条错误、根字号 200%、滚动页面、发一个 1 s 后才返回的加载、`close_on_next_action()`，紧接着 `reset()`（中间没有区域动作能插进来）。
3. 断言：轮询到 1 的整组值与首载相等；再等 1.2 s（那个加载已到期）仍相等；区域「next」点选成功且 scope 仍在（关闭已解除）。

### 删除清单（D5）

| 删除 | 由谁保留 |
| --- | --- |
| `m6-components`「the catalogue answers for every category it names」中：三列呈现文本等于支持级别、每类 DOM 为 yes、区域绘制的 14 类名单、不由区域绘制的 6 类名单及其 GPU 为 no、其环境说明含 "region" | `crates/rustify-components/src/catalog.rs` 的 `every_category_has_a_dom_component_and_a_way_to_reach_it`、`the_categories_a_region_draws_are_the_declared_subset`、`a_category_no_region_draws_says_where_it_is_drawn_instead`、`no_cell_is_blank_and_every_answer_carries_its_reason`、`every_row_is_printed_with_ten_filled_cells` |

浏览器侧保留：表格画出 `CATALOG_SIZE` 行、每行 9 个单元格都有合法的 `data-support` 与非空文本、`data-region` 与 GPU 列一致、两半（区域绘制与否）都在页上。用例本身未删，也没有新打 `@evidence` 的用例。

### 缺口

- **本次验收未能在仓库的 `support.ts` 上跑**：该文件在 13:21 的版本解析失败（`page` fixture 内 `const use = info.project.use` 遮蔽了 fixture 回调 `use`，Babel 报 "Missing initializer in destructuring declaration (660:66)"），到交付时仍未修。全 project 运行因此用 scratchpad 里的代理 config：同样的 projects 与 server，`tests/` 为副本，`support.ts` 只把该变量改名为 `project`，其余逐字相同。`support.ts` 修好后须在真实 config 上复跑两次。
- **区域内部状态不复位**：组列表滚动位置、名字/备注一旦画过非 ASCII 就切换到宽字体族的标记、`opened_links`。p2-drag 的滚轮用例已改为自给自足；等价用例不滚动组列表。
- **不可复位**：诊断记录（计绝对数的用例标 `fresh`）、`stats()` 计数、实例编号、用例 push 的历史条目及其深度。
- `support.ts` 的 `Window` 类型里没有 `reset()` 和新增的快照字段：等价用例对 `reset()` 做了类型断言，其余字段整体比较、不按名取用。
- 5 min 目标处在边缘：两次 `workers=2` 为 296 s 与 309 s。关键路径是一个 worker 依次跑 `m3-workbench`（约 111 s）、`m7-recovery`（128–137 s）、`p3-policy`+`p3-restart`（约 57 s）。这些时间几乎都是区域创建：`m7-recovery` 7 个 `fresh` 页 + 4 次上下文恢复，`m3-workbench` 3 个 `fresh` 页 + 3 次关掉 scope 后的重挂。可在本任务之外再省的：`waitForReady` 在共享页上仍会再等一次安静（fixture 刚等过），每个用例约 0.5 s；`p2-deeplink` 的 6 次自开页约 60 s。

## 验收记录

环境：Linux x64 容器（4 核、15 GiB、无 GPU，与其他 agent 共用，构建与 Playwright 以文件锁串行）；rustc 1.98.1、mbx 1.15.0、Node 26.1.0、Playwright 1.63.0，Chromium 141 headless shell（经 `PLAYWRIGHT_BROWSERS_PATH` 垫片，同 M1）；release 构建由本工作树 `mbx xtask build-web --example property-workbench --release` 产出。

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| 复位等价用例通过 | `npx playwright test --project=property-workbench m3-workbench.spec.ts -g "reset puts"`，以及下面各次全量 | 真实 config（`support.ts` 13:21 之前的版本）与代理 config | 通过，16.4–17.5 s |
| `m3-workbench` 整文件（真实 config） | `npx playwright test --project=property-workbench m3-workbench.spec.ts` | 同上，`support.ts` 13:21 之前的版本 | 20 过，2.0 min |
| 回归层全量 `workers=2`，连续两次 | `npx playwright test -c <代理 config> --project=property-workbench --workers=2`（见缺口第一条） | dirty@`fa76b20` | 第 1 次：123 过、0 败，296 s；第 2 次：123 过、0 败，309 s（对照 M1：122 过，654 s） |
| 回归层全量 `workers=1` | 同上，`--workers=1` | 同上 | 123 过、0 败，545 s（对照 M1：1,236 s） |
| 各文件耗时（`workers=1` / 两次 `workers=2`，秒） | JSON reporter 逐文件汇总 | 同上 | `m7-recovery` 116/128/137；`m3-workbench` 108/111/110；`p2-deeplink` 55/57/62；`p3-restart` 48/48/49；`m6-components` 44/45/45；`p2-clipboard-files` 34/34/36；`p2-drag` 26/27/33；`m5-semantics` 23/23/22；`m6-async` 18/19/18；`m5-text` 17/17/18；`p2-form` 11/12/14；`p2-workspace` 11/11/14；其余 ≤ 9 |
| `--list` 守恒 | `RUSTIFY_TIER=all … --list --project=property-workbench`，HEAD 的 spec 与现 spec 各列一次，去掉行列号比对 | 同上 | HEAD 137 项，现 138 项；差异只有新增的复位等价用例，其余标题逐一相同；回归层 123 项 |
| host 单测 | `CARGO_TARGET_DIR=/root/tgt-pw mbx test -p property-workbench --bins` | 同上 | 7 过 |
| clippy | `CARGO_TARGET_DIR=/root/tgt-pw mbx clippy -p property-workbench --all-targets -- -D warnings`；另跑 `--target wasm32-unknown-unknown` 看新代码 | 同上 | host 无告警；wasm32 下本次新代码无告警（剩余 3 条为既有：`set_editing` 未用、`explicit_counter_loop`、快照的 `format_in_format_args`） |
| 格式 | `cargo fmt --all -- --check` | 同上 | 通过 |
