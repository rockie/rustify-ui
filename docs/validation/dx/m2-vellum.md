# M2 子任务 · Vellum 重挂式复位与共享页 · 实现与验收记录

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) M2（ADR-6 及其「修订」、D3、D4「M1 修订」、§5.1、§9.2「复位等价」）
- 最近更新：2026-09-25 UTC
- 代码基线：dirty@`fa76b20`（M2 并行工作树，其他示例的改动由各自子任务记录）
- 范围：`examples/vellum/app.js`；`tests/vellum/support.ts`；`tests/vellum/` 下 `m2-scene`、`m3-raster`、`m4-pointer`、`m5-shell`、`m6-edit`、`m7-files` 六个 spec。未动 `twin.ts`、`playwright.config.ts`（两者同期由 D8 子任务修改）、`m3-webgpu`、`m8-ui`、证据层用例与 `examples/vellum/src`。

## 实现记录

### `window.__vellum.reset()`（重挂式，JS-only）

Vellum 的区域便宜（M1 探针：同页重挂 0.9 s，新 context 1.5 s），按 D4「M1 修订」走「卸载 → 重挂」，不做原地复位。`reset()` 返回 Promise，依次：

1. `__vellum.dispose()`：编辑器的 `on_cleanup` 关闭 IndexedDB 连接、中止写事务、取消防抖计时器，并删掉它登记的字体面。
2. 清存储：`localStorage.clear()`、`sessionStorage.clear()`；`indexedDB.deleteDatabase` 删除 `vellum-editor` 及 `indexedDB.databases()` 列出的其余库（首载时二者皆空；仍在关闭的连接只会推迟删除，不会让它失败）。
3. 清页面残留：`document.fonts` 里由脚本加入的字体面全部删除（样式表自带的删不掉，`delete` 对它们返回 false）；`window` 与 `navigator` 上首次 ready 之后新增的自有属性全部删除（首次 ready 时记录基线：用例留下的 `clipboardText`、`layerDragEvents`、`__pickerListeners`、`__lateRead`、`navigator.clipboard` 桩都属此类）；`hooks.runtime.errors` 截断为空；地址不是入口（模块加载时的 `location.href`）时 `history.replaceState` 回入口。
4. `__vellum.mount()`，按 rAF 轮询到 `#status` 为 ready、`snapshot().ready` 为真且场景已绘制（`renderer.instanceCount > 0`，无 GPU 模式以 `gpuError` 代之）才 resolve；实例在此期间 trap 则 reject。

实测（新 context，每次先建一个矩形并保存）：复位 1451 / 824 / 904 / 654 / 884 ms，中位 884 ms；同一页从导航到 ready + 500 ms 安静为 2.6 s。

不复位、交给 `fresh` 的：`?nogpu` 模式（由加载地址决定）、`addInitScript`、未恢复的原型桩（`IDBObjectStore.prototype.put`、`IDBFactory.prototype.open`、`Storage.prototype.setItem`、`File.prototype.arrayBuffer`、`EventTarget.prototype.add/removeEventListener`）、trap 与重启预算。跨挂载存活而未复位的：实例级 `stats()`（帧数、pump 数，用例只取增量或 `> 0`）、诊断记录、`document.ts` 的 `uid()` 序号（ID 另含时间与随机数，用例不依赖其值）、`#vellum` 容器上的 `data-theme`/`style`（新挂载按首载值重写，等价用例核对）。

### `tests/vellum/support.ts` 夹具（与 `tests/browser/support.ts` 同一设计）

- worker 级 `slot` 持有一页共享页；测试级 `page` 在用例前对它调 `reset()`，再走与 `waitForReady` 相同的就绪条件（`#status` ready、`vellum.ready`、后端 `Makepad WebGL2`、`instanceCount > 150`、500 ms 安静）。共享页的 context 用 `browser.newContext()` 创建，Playwright 用本用例的选项补齐未给出的项，所以与项目 `use`（1600×1000、DPR 1、Desktop Chrome UA；启动参数属浏览器级，本就共用）完全一致。
- 自有页：`test.use({ fresh: true })`、带 `@evidence` 标签（证据用例照旧从加载起测，不改它们的文件）、或本用例选项（视口、DPR、locale、配色等）与共享 context 不同 → 直接用 Playwright 的每用例页。
- 用例后：移除用例期间加在页上的监听；用例失败或健康检查不过（页已关、`__vellum.reset` 不在、`data-status` 非 ready、区域数 ≠ 1、`errors()` 非空）即丢弃该页，下个用例重开；否则 `unrouteAll`、恢复 emulated media、鼠标移回 (0,0)、恢复项目视口。`reset()` 30 s 未完成或 reject 也丢页重开；重开的用例带 `page load` 注解写明原因（首个用例 / 上个用例失败 / 不健康 / 复位失败）。
- `browserErrors`（自动）照旧：共享页的 CSP 监听在建 context 时 `context.addInitScript` 一次，逐用例比对用例开始前的计数（开始点取在复位之前，复位中的违规也算进本用例）；共享页加载与复位期间的 pageerror/console 错误并入下一个用例的错误表；自有页仍逐用例 `page.addInitScript`。
- `waitForReady(page, url, gpu)`：共享页上 `url` 为 `./` 且 GPU 模式时不导航（夹具已就绪）；其他地址、无 GPU 模式、同一用例第二次调用（即重载）都抛错并提示加 `fresh`，防止漏标。

### 规格改动

- `fresh`（用无标题的 `test.describe(() => { test.use({ fresh: true }); … })` 包起来，标题与 `--list` 不变），回归层实跑 13 项：

| 用例（新行号） | 原因 |
| --- | --- |
| `m2-scene:153` GPU-unavailable mode | `?nogpu` 由加载地址决定 |
| `m7-files:35` debounced local save restores after reload | 保存后重载 |
| `m7-files:50` IndexedDB open failure falls back | `addInitScript` 改写 `IDBFactory.open` + 重载 |
| `m7-files:63` quota failure | `IDBObjectStore.prototype.put` 桩不恢复 |
| `m7-files:78` aborted write and unavailable fallback | `put` 桩 + `addInitScript` + 重载 |
| `m7-files:121` malformed saved data | `addInitScript` 写坏存储 |
| `m7-files:133` traps recover through three restarts | `enter_fatal`，耗尽重启 |
| `m7-files:162` trap aborts a storage write | `enter_fatal` |
| `m7-files:309` file read completed after disposal | `File.prototype.arrayBuffer` 桩不恢复 |
| `m7-files:330` font picker restores after reload | 保存后重载 |
| `m7-files:371` font faces leave on runtime failure | `enter_fatal` |
| `m7-files:387` SDK picker releases listeners | `EventTarget.prototype` 包装不恢复 |
| `m5-shell:343` 复位等价（新增） | 必须以真首载为参照 |

  另 13 个孪生用例（`m3-raster:41,104`、`m4-pointer:88`×6、`:232,261,284`、`m6-edit:323`、`m7-files:499`）也标 `fresh`：它们在测试自己的 context 里开孪生页，捏合用例还对被测页用该 context 的 CDP 会话；本机无孪生无法验证共享页下的行为，保持原样。无孪生时跳过，每项约 0.15 s。
- 与 [m1-fresh.md](m1-fresh.md) 的差异：`m6-edit`「SDK clipboard…」不再需要 `fresh`——`reset()` 删除 `navigator` 上的自有 `clipboard` 桩与 `clipboardText` 全局，已在共享页上连续通过。
- 每次运行的导航次数：fresh 用例 17 次（13 项，其中 4 个重载用例各两次）+ 每个 worker 1 次共享页加载 + 每次用例失败后 1 次（Playwright 在失败后换 worker），本机 workers=1 共 19 次。改造前为 61 次（57 个实跑用例各一次，4 个重载用例各多一次）。
- D3：`m2-scene`「5,000 editable primitives」的 20 次编辑采样改 `rounds(20)`（回归层 3 次，末值随奇偶断言 25/24）；`m7-files`「SDK picker」4 轮改 `rounds(4)`（回归层 2 次取消 + 1 次选中）。`m2-scene` 三轮 × 20 级缩放的保留用例、`m7-files` 三次重启（等于重启上限）、`m3-raster` 8 个分辨率边界都是断言所需的结构而非重复，保持不变。
- 复位等价用例（`m5-shell.spec.ts`「reset returns the editor, its storage, the address and the page to their first-load state」）：新页首载 → 等首次自动保存落盘 → 取状态 A：去掉 `renderer.cpuMs`（计时）后的整份 `snapshot()`、URL、localStorage/sessionStorage 条目、IndexedDB 库名与 `current` 文档、`document.fonts`、`window` 键、`navigator` 自有键、`errors()`、区域数、`data-status`、`body` 子元素、`html`/`body`/`#vellum` 属性（按名排序）、焦点、欢迎提示、主题。随后做改动：第三页建矩形并改宽、导入字体、选中、缩放相机、切主题/网格/标尺、保存、关闭欢迎提示、切到第二页、加全局、`navigator.clipboard` 桩、直接加字体面、推入运行时错误、`pushState` 到别的地址；先断言上述每一部分都已与 A 不同，再 `reset()`，断言状态 = A。

## 验收记录

环境：Linux x64 容器（4 核、15 GiB、无 GPU），rustc 1.98.1、mbx 1.15.0、Node 26.1.0、Playwright 1.63.0，Chromium 141 headless shell（经 `PLAYWRIGHT_BROWSERS_PATH` 垫片，环境偏差同 M1）；release 构建由本工作树 `flock /tmp/build.lock mbx xtask build-web --example vellum --release` 产出（改 `app.js` 后重建）。所有 Playwright 运行都在 `flock /tmp/pw.lock` 下，墙钟从取得锁开始计。

| 退出条件 | 命令或步骤 | 环境/代码基线 | 结果 |
| --- | --- | --- | --- |
| 改前基线 | 改动前把 `tests/vellum`、`tests/tier.ts` 快照到临时目录，config 的 `root` 指回仓库，`npx playwright test -c <快照>/tests/vellum/playwright.config.ts` | `fa76b20` 原样 | 256 s（4.3 min）：56 过 / **1 败** / 16 跳过。按文件：m7-files 83.9 s、m2-scene 57.2、m5-shell 35.3、m6-edit 33.3、m3-raster 27.1、m4-pointer 13.0 |
| 复位等价 | `npx playwright test -c tests/vellum/playwright.config.ts m5-shell.spec.ts -g "first-load"` | 本改动 | 通过（7.0 s）；首跑因 `#vellum` 属性的设置顺序在重挂后不同而失败，改为按名排序比较后通过，属性值本身一致 |
| 回归层第 1 次 | `npx playwright test -c tests/vellum/playwright.config.ts` | 本改动 | 200 s（3.3 min）：57 过 / 1 败（同上既有失败）/ 16 跳过；共享页加载 2 次（首个用例 + 失败后换 worker） |
| 回归层第 2 次 | 同上 | 同上 | 211 s（3.5 min）：结果同第 1 次，失败项与像素数相同 |
| `--workers=2` | 同上加 `--workers=2`，每 2 s 采 chrome/node 进程 RSS 之和 | 同上 | 173 s（2.9 min），结果同上；峰值约 3.6 GB（含机器上其他 node 进程）。按文件分给两个 worker，m7-files 独占一侧（70 s），收益有限 |
| 单示例 ≤ 5 min（D9） | 上三行 | 同上 | 达标：workers=1 为 3.3–3.5 min（基线 4.3 min）。最慢文件 m7-files 64–70 s（11 项 fresh）、m5-shell 36 s、m2-scene 35 s |
| `--list` 守恒 | 改前快照与改后各跑 `RUSTIFY_TIER=all/regression/evidence … --list`，去掉行列号比对标题 | 同上 | all 80 → 81、regression 73 → 74，唯一新增为复位等价用例；evidence 7 → 7 不变；fresh 包装未改任何标题 |

### 缺口

- ~~「回归层连续两次全绿」未达成~~（已解除，见下节「覆盖层选区残留修复」）：`m3-raster.spec.ts`「editing a new history branch cannot reuse stale text pixels」改前改后都以 1188 个差异像素失败。当时判断为覆盖层 Effect 未重画，下节的逐帧日志证明 Effect 照常重画，残留来自 Chromium 加速 2D canvas 在只有 `clearRect` 的帧上呈现复用的旧缓冲；修复后本 config 连续两次全绿。
- `m6-edit`「SDK clipboard…」改为共享页后连续 3 次运行通过；孪生用例因本机无 `ref/Vellum-main`（也未设 `VELLUM_TWIN=baseline`）仍全部跳过，其在 `fresh` 下的行为与改前相同。

## 覆盖层选区残留修复

补上「缺口」一节里的 `m3-raster`「editing a new history branch cannot reuse stale text pixels」失败（1188 个差异像素）。改动只有 `examples/vellum/src/overlay.rs` 一处。

- **原因**：先前的判断（覆盖层 Effect 没有重跑）不成立。临时加日志后看到，第 3 步 `setProperty("text", "CCCC"); select([])` 之后 Effect 照常重跑，`selection` 为空，拿到的是 `#overlay` 本身的画布，`draw_overlay` 返回 `Ok`。这一帧里唯一的绘制操作是整幅 `clearRect`。在 Chromium 的加速 2D 画布上（本机为 141 headless shell + SwiftShader），这种「只有整幅清除」的帧会拿出一块回收的缓冲，里面还留着更早一帧的像素。连续几帧都只清除时，截图在「残留选区框（1380 像素）」和「干净」之间逐帧交替，周期为 2，是双缓冲轮换的特征。用 JS 直接对同一画布 `clearRect`、`canvas.width = canvas.width` 或 `ctx.reset()`，结果都一样交替。反过来，只要对该画布调一次 `getImageData`，现象就消失，因为读回让画布离开了加速路径；这也是 `zoomAt`、鼠标移动之类的探针结果不稳定的原因。撤销之后的那几帧只是恰好让下一帧从带选区框的缓冲开始。
- **修复**：`draw_overlay` 不再用 `clearRect` 擦除上一帧，而是在单位变换下以 `globalCompositeOperation = "copy"` 用透明色填满整幅画布，再恢复 `source-over`（`overlay.rs:47–54`）。这是一次会被记录的真实绘制，每个像素都会被替换。同样条件下，这种写法以及「整幅清除后再加一次局部 `clearRect`」都不再交替；前面列出的其他写法都会交替。像素结果与 `clearRect` 相同，都是全透明。
- **单元测试**：没有加主机测试。`overlay.rs` 只在 wasm32 下编译，要验证的是浏览器画布的呈现行为，主机测试钉不住。由 Playwright 用例覆盖。

| 检查 | 命令或步骤 | 结果 |
| --- | --- | --- |
| 复现 | 修前 `flock /tmp/pw.lock npx playwright test -c tests/vellum/playwright.config.ts m3-raster.spec.ts -g "stale text pixels"` | 失败，1188 个差异像素，与缺口记录一致 |
| 定位探针（临时，已撤回） | Effect/`select`/`draw_overlay` 打日志；截图逐帧计数覆盖层中心的强调色像素 | 第 3 步后 Effect 重跑，`sel=[]`，`Ok(())`，但画布仍有选区框；只清除的帧交替 1380/0；「copy 透明填充」为 0/0/0/0 |
| 目标用例 | 修后同上命令 | 通过（5.7 s） |
| 修后探针 | 第 3 步后连续 4 次 `select([])`，再 3 轮选中/取消 | 取消后全部为 0，选中后为 1380 |
| `m3-raster.spec.ts` 整文件 | `… m3-raster.spec.ts` | 5 过 / 2 跳过（孪生） |
| 回归层第 1 次 | `flock /tmp/pw.lock npx playwright test -c tests/vellum/playwright.config.ts` | 195 s（3.2 min，取得锁后计时）：58 过 / 0 败 / 16 跳过 |
| 回归层第 2 次 | 同上 | 197 s（3.3 min）：58 过 / 0 败 / 16 跳过 |
| 主机检查 | `CARGO_TARGET_DIR=/root/tgt-vl mbx test -p vellum`；`mbx clippy -p vellum --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 137 + 5 过；clippy 无告警；fmt 通过。另对 wasm32 目标跑 clippy，`overlay.rs` 无告警。之后已删除 `/root/tgt-vl` |

至此「回归层连续两次全绿」达成。
