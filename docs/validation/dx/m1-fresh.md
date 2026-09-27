# M1 子任务 · 回归层用例的 `fresh` 需求与复位面

- 对应计划：[DX-REFINE](../../plan/DX-REFINE.md) D4、ADR-6、A-2
- 最近更新：2026-09-25 08:05 UTC
- 方法：只读核查每个回归层用例（M1 打标后的 `RUSTIFY_TIER=regression` 集合），按 D4 五类归类：(a) 冷加载/首载字节/从导航起测；(b) 服务器故障开关；(c) `addInitScript` 或 context 级选项改写环境；(d) 深链接首载/重载；(e) 让实例 trap 或耗尽重启。另标 (f) 自开页面/context 或用 `sharedPage`，(g) 依赖页面加载以来的绝对计数。
- 代码基线：dirty@`65b2225`（M1 打标后）

## 各 project 统计

| project | 回归层用例 | 需 `fresh` | 说明 |
| --- | --- | --- | --- |
| fusion-basic | 70 | 24（若 `m4-geometry` 3 个 DPR=1 行改用 `setViewportSize` 则 21） | trap 类 9、`m4-geometry` 9 组视口×DPR（DPR 是 context 选项）、首载字体、导航离页 |
| property-workbench | 122 | 14（计入依赖诊断记录的 `m7-recovery` 则 20） | trap 3、深链接 6、剪贴板自建 context 2、CSP 监听 1、GPU 不可用 1、restart 2 |
| component-catalog | 48 | 4 | restart 2、CSP 监听 2 |
| data-workbench | 50 | 严格按 (d) 为 50 | 11 个改写实例级 `DATA`（10 万行样本，跨挂载存活，无再生接口）；36 个只是首载到 `/table`/`/scene`；3 个 e/c |
| workbench-deep | 6 | 6 | 全部是深链接首载 |
| deployment | 6 | 5 | 请求监听须见全部加载、`__fault/*` |
| vellum | 70 | 13（且每次须清 IndexedDB/localStorage） | 存储故障注入、trap、`?nogpu`、重载后恢复 |

## 影响设计的事实

1. data-workbench 的 10 万行样本 `DATA` 属于实例而非挂载（`examples/data-workbench/src/main.rs:53-54,210-221`），编辑/插入/删除在 `dispose()`/`mount()` 后仍在；只有重启或重载会重建。`dataset_hash()` 可检测漂移，缺再生接口。
2. Vellum 每次编辑自动写 IndexedDB `vellum-editor`/`documents`/`current` 与 localStorage `vellum-document`、`vellum-options`、`vellum-welcomed`（`examples/vellum/src/storage.rs`）；重新挂载会从存储读回。同 context 的新页继承同一存储，所以 Vellum 的 `fresh` 页也要先清存储。
3. property-workbench 与 data-workbench 拥有 URL，重新挂载从 URL 取状态（`examples/property-workbench/src/main.rs:395-415`、`examples/data-workbench/src/main.rs:224-233`）：复位须先 `history.replaceState` 再挂载。
4. 诊断记录无清空接口（`crates/rustify-ui/src/diagnostics.rs`）；`hooks.runtime.errors` 是上限 64 的普通数组，可截断；`stats().pumps/frames/memory` 不可复位；fusion-basic 的 `boot_instance` 起的第二实例无法卸载。
5. `deviceScaleFactor` 是 context 选项，同 context 新页拿不到，需要独立 context 或 CDP `Emulation.setDeviceMetricsOverride`。
6. Vellum 的自动 `browserErrors` fixture 每个用例 `addInitScript` 一个 CSP 监听，`__vellumCsp` 从页面加载起累计：共享页时要一次安装、按用例取增量。
7. 本工作树没有 `ref/Vellum-main`，13 个 Vellum 孪生用例照旧跳过。

## 复位须恢复的状态（按示例）

- **fusion-basic**（`examples/fusion-basic/app.js:42-183`）：挂载的 scope 集合（启动只挂 `b0`；`waitForReady` 加 `scope-a`/`scope-b`；用例还挂 `geometry`、`route-owner`/`route-guest`；`dispose(c)` 对未挂载返回 false，可逐个卸载后按基线重挂——`p3-b0:34` 要求只有 `b0`）；URL/历史（`/one`、`?a..?d`、`#anchor`）、`set_guard(false)`、`<html data-rustify-url-owner>`；视口、滚动、选区、焦点；`getContext` 桩（`m2-runtime:72-82`）与 `__probe*` 全局。不可复位：额外实例、实例编号、重启预算、`errors()`、`stats()`、诊断。
- **property-workbench**（`examples/property-workbench/app.js:127-260`）：`dispose()`+`mount()` 重建全部 Workbench 信号（对象、选择、主题、表单、工作区、拖放、传输、详情加载、编辑会话、各计数）；URL 须先置回 `/objects/1`；跨挂载存活的全局：`DELETED`、`CLOSE_ON_ACTION`（`src/main.rs:162,165`）、`window.__rustify_third_party` 计数、`lose_context_handles`、`#second-workbench` 节点；诊断记录无清空、上限 1000（`m7-recovery:260` 会填满它）；视口与根字号（`p2-zoom`）、`getContext` 桩、滚动、焦点。
- **component-catalog**（`examples/component-catalog/app.js:66-106`）：重挂重建主题、语言、内存路由（`url_owner:false`，不动地址栏）、减少动效、字体阻断、各控件值与打开的浮层；第二 scope `catalog-second` 用 `dispose_second()`；视口、悬停、焦点。
- **data-workbench**（`examples/data-workbench/app.js:60-151`）：`DATA` 须有再生接口；路由须先 `replaceState` 到 `/table` 或 `/scene`；重挂重建视图、任务、选择、草稿、场景相机与拾取；线程局部 SORT/JOB_SEQ/PATH/GENERATED_MS 在挂载之外；视口 1440×1200（部分 1100×600）、滚动容器内联 `maxHeight`、`scene-gpu` 上的 `moves` 属性与监听。
- **vellum**（`examples/vellum/app.js:87-138`）：先清存储，再 `__vellum.dispose()`+`mount()` 并等 `snapshot().ready`；`actions.resetStarter()` 是可撤销编辑，不复位选项、工具、相机与页索引；选项默认 dark/off/off/on（`tests/vellum/m7-files.spec.ts:39`）；视口 1600×1000；未恢复的原型桩（`navigator.clipboard`、`IDBObjectStore.put`、`File.arrayBuffer`、`EventTarget.add/removeEventListener`）；`document.fonts` 字体面、栅格缓存；`?nogpu` 模式。

## 逐用例清单（需 `fresh` 的；路径相对 `tests/browser/` 或 `tests/vellum/`）

**fusion-basic**：`p3-restart:14`(e) · `p3-restart:42`(e,d) · `m1-probes:139`(a：CSP 控制台监听须见启动) · `m2-runtime:269`(e) · `m4-geometry:199`×9(c：视口×DPR) · `p2-navigation:255`(d：导航离页) · `p3-instances:98`×4(e) · `p3-instances:132`(e) · `p3-instances:156`(c,a) · `p3-instances:193`(e) · `p3-instances:278`(第二实例无法卸载) · `p3-instances:304`(e：耗尽重启) · `p3-b0:59`(a：首屏字体资源计时)。仅标记：`m1-probes:44,61,80,104`、`m2-runtime:10,26,45,54` 依赖区域数；`m2-runtime:69` 页内桩 `getContext` 只在通过时恢复；`m4-geometry:210` CDP 覆盖只在通过时清除；`m2-runtime:144` 比较 3 s 内 pumps 增量；`m2-runtime:153` 要求焦点在 BODY、hash 为空；`m4-overlay:120`、`m6-mainpath:273,309,326` 依赖 geometry 挂载以来的计数；`p2-navigation:51-142`、`:165-250` 用 `sharedPage`。

**property-workbench**：`p3-restart:14,42`(e) · `m3-workbench:459,479`(e) · `m7-recovery:111`(c：`getContext` 桩不恢复) · `p2-deeplink:281`×3、`:296`、`:308`、`:327`(d) · `p2-clipboard-files:191,218`(c：自建 context + 权限/桩) · `p3-policy:69`(c,a)。依赖诊断记录计数（无清空接口）：`m7-recovery:10,66,174,252,260,292`。仅标记：`m3-workbench:21,183,250`、`m5-text:93,124,164,188,203`、`m6-components:96,122` 依赖挂载内计数（重挂即复位）；`m3-workbench:350,381,411`、`m6-async:157` 要求 `hooks.runtime.errors` 为空；`m6-components:354` 依赖第三方垫片计数；`m5-semantics:305` 依赖跨挂载的 `DELETED`；`sharedPage`：`p2-form:29-252`、`p2-workspace:43-231`、`p2-drag:136-264,266-361`、`p2-ime:160-226,228-292`、`p2-clipboard-files:37-99,101-171`（后两块自建 context）。

**component-catalog**：`p3-restart:14,42`(e) · `p2-catalog:29`(a：控制台 CSP 监听在 goto 前挂) · `p3-policy:69`(c,a)。`sharedPage`：`p2-catalog:275-521`、`p2-theme:20-218`、`p2-semantics:50-245`、`p2-i18n:45-107,109-233`、`p2-a11y:275-399`。

**data-workbench**：`p3-restart:14,42`(e) · `p3-table:404`(c,e)。改写 `DATA`：`p3-table:124,143,173,261,317`×2、`:344`，`p3-jobs:277,295,308,321`。仅首载到 `/table`：`p3-table:59,71,83,101,209,238,279,364,377,394,431`，`p3-jobs:59,85,104,118,132,161,204,216,344,386,400,411`×2、`:423`；到 `/scene`：`p3-scene:29,130,162,195,259,299,333,356,391,419,478`。

**workbench-deep**：`p2-deeplink:281`×3、`:296`、`:308`、`:327`(d)。

**deployment**：`m7-deployment:35`(a) · `:59`(b：stale-bridge) · `:84`×3(b)。

**vellum**：`m2-scene:146`(d,c：`?nogpu`) · `m6-edit:255`(c：`navigator.clipboard` 不恢复) · `m7-files:29,314`(d：保存后重载) · `m7-files:44,72,110`(c,d：`addInitScript` 改写 IDB/Storage) · `m7-files:57`(c) · `m7-files:122,151,350`(e) · `m7-files:293,366`(c：原型桩不恢复)。孪生 `context.newPage` 用例（无 `ref/` 时跳过）：`m3-raster:194,251`、`m4-pointer:83`×6、`:221,250,273`、`m6-edit:318`、`m7-files:471`。
