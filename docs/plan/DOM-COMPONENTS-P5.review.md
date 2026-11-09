# DOM-COMPONENTS-P5 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P5 · 原应用迁移与整体交付（扩展组件同源主题/状态 · 迁移事实源 · Studio 与官方站收窄终验）](DOM-COMPONENTS-P5.md) | 2026-11-04；SHA-256 `d83974457f632ca188b6703116ffb0d9d267a67f9694f7eecfa6bb9998ec07d3`；729 行；未提交工作树；原文自报 Ready | 全文：§0—§12、实施者定位、实施进度、M1—M4、V-1—V-13 |

评审日期：2026-11-04（Asia/Shanghai）。代码基线：`/Users/rockie/Documents/gh-rockie/rustify-ui`，main / `3735889c89319be11b0d25857f2a2730d96c6e97`，dirty；产品源码与测试未改动，未提交内容为 SDLC 台账、`skills-lock.json`、P3/P4 评审/覆盖报告及本计划。需求基线：[PRD-DOM-COMPONENTS v0.2](../PRD-DOM-COMPONENTS.md) §3.1/§3.2、§7.2/§7.3、UAT-13/UAT-14。仓库规则：[AGENTS.md](../../AGENTS.md)。本轮是首次完整计划评审，不是复审；无上轮报告。

## 1. 确认问题

共 16 项：P2 8 项，P3 8 项；无 P0/P1。四个视角的同根因问题已合并并保留来源；所有问题均不需要用户或外部提供答案，可由实施者在对应批次内修订，阻塞均为否。计划的技术主线（统一主题角色、部件/状态标识、ARIA 结构修正、legacy 事实源、真实 200% 方案、C-6/C-7/C-8 收窄）已按代码与配置逐项核对，未发现需要重做方案的问题。

### P2

1. **[P2] F-01 · V-2 的 axe 退出条件没有承载任务，仓内 Studio 浏览器测试也没有 axe 集成**
   - 定位：[§7 WORK-N1](DOM-COMPONENTS-P5.md):521、[§9.1 V-2](DOM-COMPONENTS-P5.md):546；对照 [§1.2](DOM-COMPONENTS-P5.md):330 与 [M2](DOM-COMPONENTS-P5.md):689 的改动清单。
   - 问题：V-2 把「扩展节 axe serious/critical 为 0（含选中、空、错误态）」写成 M2 退出判据，但 §1.2 与 M2 只安排 `theme-gallery.spec.ts`、`gallery-matrix.ts` 等文件，没有 axe 扫描任务；按原文执行 V-2 得不到这项结果。实施者要么在 M2 现场加一个计划未列、口径未定的扫描，要么让 V-2 无法通过。
   - 证据：`tests/browser/` 全文无 `AxeBuilder`/`axe-core` 引用（`rg` 零命中）；根 [package.json:6](../../package.json) 已安装 `@axe-core/playwright@4.13.0`，且 [tests/docs-ssg/theme.spec.ts:1](../../tests/docs-ssg/theme.spec.ts) 已在 Docs 侧使用，说明工具可用但 Studio 侧未接。
   - 修订：在 §1.2 和 M2 的 `theme-gallery.spec.ts` 任务中写明「对 Extensions 节（含选中、空、错误态）用 `@axe-core/playwright` 扫 serious/critical」；或删除 §7/V-2 的 axe 断言，改由既有对比度矩阵与人工走查承担。优先补任务，因为计划已把它写成验收门。
   - 复核：V-2、§7、§1.2、M2 四处同现 axe（或同时移除）；M2 执行 V-2 时报告实际扫描范围。实现期由 V-2 既有命令承担，不新增检查组。
   - 置信度：高（目录级事实 + 计划三处互证）；阻塞：否。

2. **[P2] F-02 · 新样本的 Empty 控件与既有矩阵的 Restore 断言冲突，计划未写恢复动作**
   - 定位：[§6 样本表](DOM-COMPONENTS-P5.md):501-502、[M2](DOM-COMPONENTS-P5.md):689、[V-2](DOM-COMPONENTS-P5.md):546。
   - 问题：DataTable 与 Tree 样本都计划提供 `Empty` 控件，但 `theme-gallery.spec.ts` 对任何带 `Empty` 标签的样本，在切到空态后都要求出现 `Restore` 按钮；计划没有为这两个新样本写恢复动作。M2 运行 `--project=theme-studio theme-gallery.spec.ts` 时会失败；更差的处理是实施者为过断言删掉 Empty 控件，反而丢失 WORK-F22/AC-1 的空状态样本证据。
   - 证据：[tests/browser/theme-gallery.spec.ts](../../tests/browser/theme-gallery.spec.ts):275 的通用状态循环与 `:284` 的 `if (label === 'Empty') await expect(sample.getByRole('button', { name: /Restore/ })).toBeVisible();`；现有样本均配套恢复入口：[gallery.rs](../../examples/theme-studio/src/gallery.rs):309-311（`Restore options`）、`:541-543`（`Restore members`）。
   - 修订：在 §6 的 DataTable、Tree 两行「test id / 可观察结果」中写明空态包含 `Restore …` 动作（与 Select/Table 同形）；同时在 §12 的清单补一项「Empty 控件需配套 Restore 动作」。
   - 复核：§6 两行出现恢复动作；M2 运行 `--project=theme-studio theme-gallery.spec.ts` 通过。
   - 置信度：高（既有断言与现有样本写法互证）；阻塞：否。

3. **[P2] F-03 · V-10 声称「真实 Chrome 200%」，两条命令都没有选中真实 Chrome**
   - 定位：[§9.1 V-10](DOM-COMPONENTS-P5.md):554、[§4 真实 Chrome 行](DOM-COMPONENTS-P5.md):405、[D12](DOM-COMPONENTS-P5.md):256。
   - 问题：V-10 的两条命令只设 `RUSTIFY_TIER=evidence`（官方站那条另设 Python PATH），没有设 `RUSTIFY_CHROMIUM`。两份 Playwright 配置的 `launchOptions.executablePath` 都取自该变量，缺省时使用 Playwright 自带 Chromium；按命令字面执行，V-10 产物不是「真实 Chrome」，但计划要求按此写「真实 Chrome 200%」结论。§9.3 的 V-11 命令反而显式设置了该变量（`:608`、`:620`），说明是遗漏而非取舍。
   - 证据：[playwright.config.ts](../../playwright.config.ts):111-113 与 [playwright.docs.config.ts](../../playwright.docs.config.ts):128 的 `executablePath: process.env.RUSTIFY_CHROMIUM || undefined`；[theme-acceptance.spec.ts](../../tests/browser/theme-acceptance.spec.ts):354 也依赖同一变量；V-11 在计划 `:608`、`:620` 已显式设置。
   - 修订：在 V-10 两条命令前加 `RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'`；或明确 `tests/native-zoom.ts` 的解析顺序（变量 → 本机默认路径 → skip），并要求记录 `executable_path`。缺 Chrome 时按 §4 记未运行，不冒充已验证。
   - 复核：V-10 命令或 §4 行给出唯一可执行的浏览器选择；V-10 记录能读到实际浏览器路径。
   - 置信度：高（配置读取路径可查）；阻塞：否。

4. **[P2] F-04 · V-4 角色模拟用基线 lockfile 加 `--locked`，会在编译前被依赖闭包拦住**
   - 定位：[§9.1 V-4](DOM-COMPONENTS-P5.md):548、[§9.3 角色模拟](DOM-COMPONENTS-P5.md):626-630。
   - 问题：V-4 从 `11b69129` 取出 `tests/dom-only/hydration`，保留其 `Cargo.lock`，再把路径依赖指向当前 checkout，却用 `mbx check --locked`。当前 SDK 已新增 `rustify-colors`（`rustify-dom`）与 `cssparser`（`rustify-components`）等依赖，基线 lockfile 没有这些条目；`--locked` 会在编译前报锁文件过期。实施者会把它误记为迁移指南卡点，角色模拟得不到有效结论。
   - 证据：`git show 11b69129:tests/dom-only/hydration/Cargo.lock` 无 `rustify-colors`/`cssparser`（仅有 `paste`）；当前 [crates/rustify-dom/Cargo.toml](../../crates/rustify-dom/Cargo.toml):30 新增 `rustify-colors`，[crates/rustify-components/Cargo.toml](../../crates/rustify-components/Cargo.toml):24 新增 `cssparser`；V-4 命令原文为 `mbx check --locked --manifest-path <副本>/Cargo.toml …`。
   - 修订：V-4 对副本去掉 `--locked`（或在副本内显式更新 lockfile），并写明这是迁移的一部分、不是指南卡点；若坚持 `--locked`，需另取与当前依赖闭包一致的基线副本，但那会削弱角色模拟的基线意义。
   - 复核：V-4 命令与 §9.3 角色说明一致；执行时先得到编译结果，而不是锁文件错误。
   - 置信度：高（基线 lock 与当前依赖直接对比）；阻塞：否。

5. **[P2] F-05 · CommandPalette 空结果态仍输出 `aria-expanded="true"` 与悬空 `aria-activedescendant`**
   - 定位：[§5.4 CommandPalette](DOM-COMPONENTS-P5.md):465-466、[V-2](DOM-COMPONENTS-P5.md):546。
   - 问题：按计划把空提示移出 listbox、并给 `ul` 加 `hidden` 后，输入框仍固定输出 `aria-expanded="true"`，`aria-activedescendant` 也仍指向打开时设定的第一条命令；查询过滤到空结果时该 option 已从 DOM 移除。屏幕阅读器会得到「已展开」但 listbox 隐藏、活动项 IDREF 悬空的状态；V-2 的 axe 空态扫描很可能命中 `aria-valid-attr-value` 一类的 IDREF 问题，即使未命中，combobox 的展开/活动项语义仍不成立。
   - 证据：[command_palette.rs](../../crates/rustify-components/src/workspace/command_palette.rs):204 的 `aria-expanded="true"`、`:207` 的恒输出 `aria-activedescendant`、`:123-141` 仅在打开时把 `active` 设为第一条、`:259-265` 当前空提示 `<li>`；`active` 被过滤后不会自动清空。
   - 修订：§5.4 补空态契约：`aria-expanded` 取 `!shown.is_empty()`；`active` 不在 `shown` 中时不输出 `aria-activedescendant`（用 `Option` 属性即可，不新增信号）；或保留空 listbox、不做 `hidden`，只把提示移出。前者更符合 combobox 模式，且不需要新状态。
   - 复核：§5.4 写出 `ul`/`aria-expanded`/`aria-activedescendant` 三者的空态取值；V-2 的空态 axe 断言通过。
   - 置信度：高（ARIA 组合与代码路径明确；axe 规则命中属实现期复核）；阻塞：否。

6. **[P2] F-06 · Splitter 的 `aria-valuemin/valuemax` 直接索引 `mins`，绕过 `drag` 的越界容忍**
   - 定位：[§5.4 Splitter](DOM-COMPONENTS-P5.md):467。
   - 问题：计划写法是 `aria-valuemin=mins[index]`、`aria-valuemax=sizes[index]+sizes[index+1]-mins[index+1]`。现有 `drag` 明确容忍 `mins.len() < sizes.len()`（用 `.get(...).unwrap_or(0.0)`），`aria-valuenow` 也用 `.get()`；按计划字面直接索引，渲染期会在短 `mins` 上 panic。
   - 证据：[splitter.rs](../../crates/rustify-components/src/workspace/splitter.rs):22-23（`mins.get(index).copied().unwrap_or(0.0)`）、`:156-160`（`aria-valuenow` 用 `.get()`）；计划 `:467` 的公式与 `drag` 的夹取边界一致，但索引安全性没有同步。
   - 修订：§5.4 写成与 `drag` 相同的 `.get(index).copied().unwrap_or(0.0)`；并写明同一分隔条拖动时 `sizes[i]+sizes[i+1]` 恒定，`valuemax` 是稳定上界。
   - 复核：§5.4 公式带 `.get()` 容忍；现有 `drag` 单测覆盖夹取，不新增测试门。
   - 置信度：高（同文件两种写法直接可读）；阻塞：否。

7. **[P2] F-07 · M3 声明可与 M2 并行，但两者共用 `rustify-components` 编译输入与同一 `target/`**
   - 定位：[§10 M3](DOM-COMPONENTS-P5.md):690、[M2](DOM-COMPONENTS-P5.md):689；对照实施者定位的 `@parallel_delivery`。
   - 问题：M3 编辑 `catalog.rs` 与新增 `catalog/legacy.rs`；M2 需要 `mbx xtask build-web --example theme-studio --release`、`mbx test -p theme-studio --bins` 与 Studio 浏览器。`catalog` 模块在 `rustify-components` 中无条件编译，theme-studio 的 `gallery_catalog.rs` 还直接读取 `STABLE_REFERENCES`。若并行，M2 的 release/测试可能编译到 M3 半成品，或两次 mbx 争用同一 target 锁，M2 证据会来自未定型的 SDK。
   - 证据：[crates/rustify-components/src/lib.rs](../../crates/rustify-components/src/lib.rs):18、`:94` 无条件导出 catalog；[gallery_catalog.rs](../../examples/theme-studio/src/gallery_catalog.rs):341 直接消费 `STABLE_REFERENCES`；计划 M3 写「可与 M2 并行」。
   - 修订：在 §10 M3 行或 §10 前一句加顺序约束：M3 的 `catalog.rs`/`legacy.rs` 在 M2 构建前落定，或 M2 退出后再进入 M3；不需要新增里程碑。若坚持并行，必须给 M3 明确的「不触碰会参与 M2 构建的源码」边界，但 M3 的核心交付正是该源码，因此串行更简单。
   - 复核：§10 不再出现无约束的「可与 M2 并行」；M2/M3 的构建顺序可直接执行。
   - 置信度：高（模块无 cfg 门 + theme-studio 测试直读 catalog）；阻塞：否。

8. **[P2] F-08 · V-9 的「焦点 ≥3:1」只测默认态首个焦点元素，选中行/项的 `outline-current` 与 Invalid 边框没有进入测量**
   - 定位：[§9.1 V-9](DOM-COMPONENTS-P5.md):553、[V-2](DOM-COMPONENTS-P5.md):546、[§5.1](DOM-COMPONENTS-P5.md):416、[§6 样本](DOM-COMPONENTS-P5.md):501-502。
   - 问题：§1.1 自报旧 ring 在选中背景只有 1.74/1.21，D3 因此改用 `aria-selected:focus-visible:outline-current`；但矩阵的焦点循环只对每个样本的第一个可聚焦元素在默认态聚焦测量。`stateTarget` 只覆盖 Tabs/Accordion/Collapsible，通用状态循环只含 Disabled/Loading/Indeterminate/Empty；DataTable 行选中、Tree 项选中、FilePicker/DropZone Invalid 都不进入 `edges`。axe 不判定 outline/边框对比，V-2 也不能补这项。结果：计划最核心的选中焦点修复没有对比证据。
   - 证据：[tests/browser/theme-gallery.spec.ts](../../tests/browser/theme-gallery.spec.ts):313-326（只取首个可聚焦元素并聚焦默认态）、`:247-250`（`stateTarget` 映射）、`:275`（通用状态循环）；[gallery-matrix.ts](../../tests/browser/gallery-matrix.ts):105-111（只在 `:focus-visible`/`:focus-within` 且有 outline 时收集焦点边）。
   - 修订：在 V-9 或 V-2 的既有矩阵步骤中，先选中一个 DataTable 行/Tree 项，再聚焦该选中元素并测量 `edges`；把 FilePicker/DropZone 的 Invalid 边框也纳入 `edges`。不要把「焦点 ≥3:1」限定为默认态，因为要验证的正是选中背景上的对比。
   - 复核：V-9/V-2 的步骤与 `theme-gallery.spec.ts` 的选中/Invalid 测量一致；证据 JSON 含对应 `edges` 值。
   - 置信度：高（矩阵代码可读；未实跑）；阻塞：否。

### P3

9. **[P3] F-09 · §5.1 引用的 `HEADER_BUTTON` 焦点字面量与源码不符**
   - 定位：[§5.1](DOM-COMPONENTS-P5.md):416。
   - 问题：计划写 `GRID`、`HEADER_BUTTON` 的 `focus-visible:ring-ring/50 focus-visible:ring-[3px]` 换成 outline；实际 `HEADER_BUTTON` 是 `ring-2`，不是 `ring-[3px]`。按计划字面整串替换会漏掉表头按钮，D3 的「统一 2px outline」不成立，且没有断言能发现。
   - 证据：[data_table/view.rs](../../crates/rustify-components/src/data_table/view.rs):11（`GRID` 为 `ring-[3px]`）、`:14`（`HEADER_BUTTON` 为 `ring-2`）、`:18`（`CELL` 为 `ring-2 ring-inset`）。
   - 修订：拆成两处字面量（GRID 的 `ring-[3px]`、HEADER_BUTTON 的 `ring-2`），或写「三处 ring 形态不一，逐处替换为新 outline」。
   - 复核：§5.1 的 class 串与 `view.rs` 三处常量逐字一致。
   - 置信度：高；阻塞：否。

10. **[P3] F-10 · Glyph 从六个变七个后，`catalog.rs:220` 的「same six shapes」未列入同步清单**
    - 定位：[§5.7](DOM-COMPONENTS-P5.md):483。
    - 问题：计划只列 `icon.rs` 模块注释、`catalog.rs:215`、`catalog.rs:935`；`catalog.rs:220` 的 `environment` 能力文本仍写「the same six shapes」。它会随 `catalogue_markdown()` 原样进入 `docs/components.md`，导致目录仍声明六种，与七种 Glyph 不一致（WORK-N5/AC-2）。
    - 证据：[catalog.rs](../../crates/rustify-components/src/catalog.rs):215、`:220`、`:935`；[icon.rs](../../crates/rustify-components/src/icon.rs):6、`:15-22` 当前六个变体。
    - 修订：§5.7 清单补 `catalog.rs:220`；改后 `rg 'six|Six' crates/rustify-components/src/{icon.rs,catalog.rs}` 核对全部相关行，再跑 `mbx xtask catalog --write docs/components.md --check`。
    - 复核：计划清单覆盖三条 catalog 文本；生成物无 `six` 残留。
    - 置信度：高；阻塞：否。

11. **[P3] F-11 · §5.2「ARIA 不变」与 §5.3/§5.4/D7 的 ARIA 修正互相矛盾**
    - 定位：[§5.2](DOM-COMPONENTS-P5.md):425 对 [§5.3](DOM-COMPONENTS-P5.md):451、[§5.4](DOM-COMPONENTS-P5.md):457/466/467 与 [D7](DOM-COMPONENTS-P5.md):251。
    - 问题：§5.2 开头写「既有 `data-name`、`data-testid`、ARIA 不变」，但 §5.3 增 FilePicker `aria-invalid`，§5.4 改 PanelTabs 结构、DropZone `role=group`、Splitter `aria-valuemin/valuemax`，D7 明确列四项语义修正。实施者若把 §5.2 当约束，会保留现状；V-1 的 `role=tab` 不含 `button`、DropZone `role=group` 随即失败。
    - 证据：计划四处原文；现状 ARIA 见 [panel_tabs.rs](../../crates/rustify-components/src/workspace/panel_tabs.rs):129-165、[command_palette.rs](../../crates/rustify-components/src/workspace/command_palette.rs):206-268、[drop_zone.rs](../../crates/rustify-components/src/drop_zone.rs):73-79、[splitter.rs](../../crates/rustify-components/src/workspace/splitter.rs):149-155。
    - 修订：§5.2 改为「新增 part/state 不改变既有 `data-name`、`data-testid`；ARIA 变化以 §5.4 为准，仅限该节四类修正」。
    - 复核：§5.2 保留句与 §5.4/D7 不再互斥。
    - 置信度：高；阻塞：否。

12. **[P3] F-12 · DropZone 默认标签仍一次取值，D6 的「中文 scope 不再显示硬编码英文」对它不成立**
    - 定位：[§5.5](DOM-COMPONENTS-P5.md):475、[D6](DOM-COMPONENTS-P5.md):250。
    - 问题：计划明确「DropZone 现有写法（一次取值）不改」，而 DataTable/Tree/Splitter/PanelTabs/CommandPalette 的默认词改为 reactive closure。运行期切到中文后，DropZone 默认标签仍是英文；同一画面出现中英并存。V-2 又要求「切 zh 后 SDK 默认词变化」，若 Studio 样本不传显式 label，会撞上这个例外。
    - 证据：[drop_zone.rs](../../crates/rustify-components/src/drop_zone.rs):43-48（setup 期一次 `locale.text(..).to_string()`）；[i18n.rs](../../crates/rustify-dom/src/i18n.rs):340-345（`LocaleHandle::text` 是 tracked，改为闭包即可响应）。
    - 修订：§5.5 把 DropZone 也列入 reactive closure（与 Splitter 同一写法，约两行）；若确实不做，写明用户可见后果与 Studio 样本必须传显式 label 的约束。
    - 复核：§5.5 不再保留无理由的「不改」；V-2 切 zh 时样本默认词一致。
    - 置信度：中（是否可见取决于 Studio 样本是否传 label；SDK 行为本身确定）；阻塞：否。

13. **[P3] F-13 · DataTable 行与 Tree 项用两种属性表达「选中」**
    - 定位：[§5.2 表](DOM-COMPONENTS-P5.md):431、`:433`。
    - 问题：DataTable 行用 `data-state=selected|unselected`，Tree 项用 `data-selected`。同一期交付的两个组件对同一概念使用不同属性；P2 的状态协议只列 `data-state` 的 checked/on/off/open/closed，`data-selected` 不在其中。调用方若要统一给选中态上样式，必须写两套选择器。
    - 证据：[DOM-COMPONENTS-P2.md](DOM-COMPONENTS-P2.md):378 的状态协议；计划 §5.2 两行。
    - 修订：两处都用 `data-selected`，或在 §5.2 写明为何行用 `data-state`、树项用 `data-selected`（树项的 `data-state` 已被 open/closed 占用），并同步 V-1 的属性断言。
    - 复核：§5.2 两行同形或已有明确理由；V-1 断言与定稿一致。
    - 置信度：高；阻塞：否。

14. **[P3] F-14 · 迁移事实源仍有两条完整性缺口：`Message::all()` 的返回长度与 `semantic:` 的键范围**
    - 定位：[§3 第 4 条](DOM-COMPONENTS-P5.md):349、[§3 第 7 条](DOM-COMPONENTS-P5.md):357、[§3.1 `semantic:` 行](DOM-COMPONENTS-P5.md):384。
    - 问题：计划说「唯 `Message`、`Glyph` 增枚举项对穷尽 `match` 是源码级破坏」，但 `Message::all() -> [Self; 19]` 在加四项后返回长度变为 23，也是公开签名变化；`examples/component-catalog/src/main.rs:1206` 正在使用它。第 7 条的「Rustify extensions」小节没有承载「新增枚举项使穷尽 match 失效、`all()` 长度 19→23」的落点。另一侧，`semantic:` 只给出 5 个示例键（accent、primary-default、accent-foreground-default、ring-default、spacing），而 `Theme::properties()` 有 23 项；ADR-1 的 host 测试只验证「每个基线键恰好落一行」，不验证基线枚举本身覆盖了全部颜色语义，WORK-F23/AC-2 的「颜色语义」仍靠人工。
    - 证据：[i18n.rs](../../crates/rustify-dom/src/i18n.rs):211-233；[component-catalog/src/main.rs](../../examples/component-catalog/src/main.rs):1206；[theme.rs](../../crates/rustify-dom/src/theme.rs):180-205；[sdk.css](../../crates/rustify-components/css/sdk.css):120-143；计划 §3.1 的 `semantic:` 行。
    - 修订：第 7 条的「Rustify extensions」补一段列出新增 `Message`/`Glyph` 变体、穷尽 `match` 需补分支、`Message::all()` 长度 19→23；`semantic:` 明确「覆盖 `Theme::properties()` 的全部 23 个 token」或列出不纳入的 token 与理由；在 M3 记录写明基线提取范围与人工补项。
    - 复核：指南小节含上述两项；`semantic:` 行与 `Theme::properties()` 对齐；V-3 的 host 测试与 `catalog --check` 通过。
    - 置信度：高（API 与 token 清单可读）；阻塞：否。

15. **[P3] F-15 · D12 把「真实浏览器 UI 缩放」当作全新机制，但 Studio 已有真实 Chrome 200% 用例**
    - 定位：[D12](DOM-COMPONENTS-P5.md):256、[§1.1 实验](DOM-COMPONENTS-P5.md):299、[§9.3](DOM-COMPONENTS-P5.md):600 区。
    - 问题：`theme-acceptance.spec.ts:354` 已用 `launchPersistentContext` + `chrome.settingsPrivate.setDefaultZoom(2)` 做真实 Chrome 页缩放（断言 DPR 翻倍、`innerWidth` 减半、根字号与 `visualViewport.scale` 不变），且属于 `theme-studio` 项目，V-7 会跑到。计划的新 `tests/native-zoom.ts` 形成第二套 profile/zoom 机制，却没有说明是取代还是并存；「关闭 P2—P4 记录的 missing」也忽略了这条既有证据，后续记录容易混用两套结果或重复维护。
    - 证据：[theme-acceptance.spec.ts](../../tests/browser/theme-acceptance.spec.ts):354-366；[playwright.config.ts](../../playwright.config.ts):160-163（`theme-studio` 项目 `testMatch: theme-*.spec.ts`）；`git blame` 显示该用例自 Theme Studio 基线起存在。
    - 修订：§1.1/§4 补一句既有实现与其局限（`settingsPrivate` 需要临时 context 与 Chrome 设置页），说明新 helper 是取代、复用还是仅服务官方站；若复用，V-10 的 Studio 用例可改为调用同一 helper，避免重复。
    - 复核：§1.1 实验条与 §4「真实 Chrome」行提到既有用例及取代关系；V-7/V-10 不重复跑同一路径。
    - 置信度：中（既有用例存在，P4 是否实际通过未核）；阻塞：否。

16. **[P3] F-16 · FilePicker/DropZone 的 Invalid 样本只有边框与 `aria-invalid`，没有错误理由文本**
    - 定位：[§6 第 3 行](DOM-COMPONENTS-P5.md):503、[§5.3](DOM-COMPONENTS-P5.md):453。
    - 问题：计划把 `described_by` 定义为应用提供的错误说明，但 Studio 的 Invalid 样本只承诺「Invalid 边框与 `aria-invalid`」。使用者能看到红色边框，却看不到为什么被拒或如何恢复；屏幕阅读器只得到 invalid 状态而无原因。这弱化了 WORK-F22/AC-1「错误状态可辨认」，也没有实际演示 SDK 的错误说明契约。
    - 证据：计划 §5.3「错误说明沿用既有 `described_by`，由应用提供」；§6 FilePicker · DropZone 行；`drop_zone.rs`/`file_picker.rs` 的 `described_by` 属性。
    - 修订：§6 该行补「样本同时提供 `described_by` 文本，与 invalid 同步」；Studio 样本是应用，提供一句理由属于其职责，不需要 SDK 判断文件内容。
    - 复核：§6 行出现错误说明文本要求；V-2 的 Invalid 态能读到 `aria-describedby` 与文本。
    - 置信度：中（按计划文本判定，未运行页面）；阻塞：否。

## 2. 待核实 / 待决策

以下项目不影响本轮「是否需要修改」的结论；除注明外均不需要用户拍板。

| 项目 | 缺什么证据 | 影响哪个决定/任务 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| VoiceOver、OS IME、物理触控能否在本机执行 | 实际 V-12 尝试记录；AppleScript/辅助功能权限未知 | WORK-N1/AC-3 的 missing 清单 | M4 | 否；AC 允许列缺失 |
| V-11 的旧侧是 `11b69129`（P1 前基线），不是 P5 前 HEAD | 旧 spec 能否在当前 Playwright/Chrome 下跑通；旧侧输出路径与 §9.3 的 `cp` 是否一致 | WORK-N2/AC-2 的归因与「旧功能/新能力分列」 | M4 首次 V-11 | 否；失败记未运行并保留尺寸数据 |
| `extensions_ssg` 首次在 SSG 下渲染 DataTable/Tree 的体积与耗时 | 首跑结果；DataTable 默认 10×60 池在 host 的 HTML 体积 | M1 的 V-1 | M1 | 否；M1 内可调整顺序（先 Tree/Splitter，再 DataTable） |
| `rustify-components 0.2.1` 是否已在 crates.io 发布 | registry 查询或既有发行记录 | M3 把 README 的 `=0.2.0` 改成 `=0.2.1`；D13 的「source checkout」措辞 | M3 | 否；未发布则保持 source 说明，不改发行范围 |
| `axe 4.13.0` 对空态悬空 `aria-activedescendant` 的具体命中 | M2 首跑 axe 的结果 | F-01/F-05 的修订后验证 | M2 | 否；ARIA 组合本身已需修正 |
| `mbx test -p theme-studio --bins` 是否存在可运行的 bin 测试 | 当前 theme-studio 的 `#[cfg(test)]` 与 `--bins` 结果 | V-2 的首条命令 | M2 | 否；空跑会暴露，改用 check/spec |
| `tests/native-zoom.ts` 的 Chrome 解析顺序与 profile 清理 | 实施后的 helper 代码与 V-10 记录 | F-03/F-15 | M4 | 否 |
| P4 代码复审 R3 只覆盖 R2 修复差异，未重审 P4 全量实现 | 不是 P5 的缺陷；引用 P4 恢复快照 7/7 与 M7 记录 | C-1 的依赖可信度 | M1 前 | 否；现有 P4 完成记录已足够作为前置 |

## 3. 覆盖摘要

- **计划与基线**：计划全文 729 行、SHA-256 `d8397445…`；仓库 main / `3735889c` / dirty；PRD v0.2；AGENTS.md；dev-plan 骨架与实施进度合同。计划头部的工作树声明与实测一致：产品源码/测试无改动，未提交项为台账、`skills-lock.json`、P3/P4 报告与本计划。
- **实际分工**：四个独立只读 reviewer subagents，分别覆盖 facts、architecture、schedule、delivery；同一计划版本与基线，未把彼此结论作为前提。主 agent 保留综合裁定，逐条回读计划段落、代码与配置，合并同根因并核对、收窄/撤回了三处过宽主张（见下）。
- **脚本**：`check_paths.sh docs/plan/DOM-COMPONENTS-P5.md .` 退出 0，6 个 MISSING 全为 ★ 新增路径（`catalog/legacy.rs`、`legacy_baseline.txt`、`extensions_ssg.rs`、`docs/validation/dom-components-p5/`、`theme-zoom.spec.ts`、`native-zoom.ts`），无复用路径缺失。`check_progress.sh docs/plan/DOM-COMPONENTS-P5.md .` 退出 0，0/4、完成记录 0 行、ERROR 0；WARN 为「计划文件未被 git 修改」的未跟踪提示，符合尚未开工。脚本通过不代表行为验收。
- **浏览器/构建**：本轮未运行 `mbx`、`cargo`、Playwright 或任何构建；未打开 Studio/官方站。UI 证据等级为「仅计划 + 现有源码/spec 静态核对」；计划 M2 的新 Extensions 节与 M4 的收口均未实现，不能据本报告声称任何界面已通过。计划自身的 C-7 收窄与 §9.1 豁免表已如实登记。
- **被纠正/收窄的 reviewer 主张**：
  - facts 视角曾主张「矩阵完全不产生焦点边」。实际 `theme-gallery.spec.ts:313-326` 会按键盘模态聚焦每个样本首个可聚焦元素并测量 `edges`；本报告收窄为 F-08「只测默认态，选中/Invalid 未测」，不保留「完全不测焦点」的说法。
  - architecture 视角对 Splitter 公式的算术判断成立；本报告保留其索引安全部分（F-06），不把「公式本身错误」写成缺陷。
  - schedule 视角的「提交点排除台账」措辞争议未进入 findings：计划原文的「已有…既存未提交内容」可读为排除他人既有改动，且 `@project_index` 已要求同步台账；保留为实施时的措辞注意，不判缺陷。
- **未覆盖**：P1—P4 的实现/验收历史只按 P4 恢复快照与 M7 记录引用，未重审；`ref/` 与外部 registry 未查询；VoiceOver/IME/触控未尝试；旧基线 worktree 未实际构建。上述未覆盖不改变本轮计划文本结论。

## 4. 映射与实施编排

### 4.1 需求映射

11 条本期 AC 与 8 条整条终验的映射已逐条核对，未发现硬断链：

| 本期 AC | 设计与任务 | 里程碑 | 验收 | 结论 |
| --- | --- | --- | --- | --- |
| WORK-F19/AC-3 | §5.2/§6、D4/D10；`gallery_extensions.rs`、`ExtensionLocalSample` | M2/M4 | V-2、V-7、V-9 | 闭合；GPU 部分由 V-7 全项目覆盖 |
| WORK-F22/AC-1 | §5.1—§5.7、D2—D7；组件属性/ARIA/空错禁用 + Studio 样本 | M1/M2 | V-1、V-2 | 闭合；原示例同类任务按 C-7 记豁免 |
| WORK-F22/AC-2 | D2、§5.1；选中前景成对、`text-foreground`、窗口化零改动 | M1/M2 | V-2、V-9 | 闭合；前后性能比较按 C-7 记豁免 |
| WORK-F23/AC-2 | ADR-1、§3.1、D8/D9/D14；`LEGACY_ENTRIES`、基线、生成节、指南 | M3 | V-3 | 闭合；旧主题「无平行实现」子句按 C-6 记不成立 |
| WORK-F23/AC-3 | §3 第 6/7 条、§9.3；指南、V-4 角色模拟、V-6 编译门、V-8 官方站 | M3/M4 | V-4、V-6、V-8 | 闭合；其余浏览器任务按 C-7 豁免 |
| WORK-N1/AC-3 | §5.1—§5.4、§6、`native-zoom.ts`/`theme-zoom.spec.ts` | M1/M2/M4 | V-2、V-10、V-12 | 闭合；axe 任务与选中态测量见 F-01/F-08 |
| WORK-N2/AC-2 | §7、§9.3；Studio 新旧对比、三示例尺寸、`rustify.css` 差值 | M4 | V-11 | 覆盖 AC 三要素；旧侧归因见 §2 |
| WORK-N4/AC-2 | D11、§7；Studio 根/子路径/deep、官方站三部署、theme-export host | M4 | V-7、V-8、V-13 | 闭合；独立消费者/DOM-only 浏览器按 C-7 豁免 |
| WORK-N4/AC-3 | D11、§7；资产外置、无新服务/Node、工具链不动 | M4 | V-11、V-13 | 闭合 |
| WORK-N5/AC-2 | ADR-1、§5.8；目录修正、指南、矩阵 | M3/M4 | V-3、V-9、V-13 | 闭合；迁移完整性缺口见 F-14 |
| WORK-N7/AC-3 | §6、§7；Extensions 节、`GalleryLocale`、局部滚动、zoom | M2/M4 | V-2、V-7、V-8、V-10 | 闭合；图表/数据区局部滚动有既有 `SCROLLER` 支撑 |

`C-6/C-7/C-8` 与 §0.1、§0.5、§9.1、§11 的豁免口径一致；计划明确「P5 完成不等于 PRD UAT-13/UAT-14 全部通过」，未把豁免记为通过。按 `@plan_proportionality` 与用户已确认取舍，不把 C-7 的浏览器收窄本身作为缺陷。

### 4.2 依赖边与波次

| 波次 | 任务 | 前置及解锁证据 | 写入边界 | 运行资源 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| W1 | M1 扩展契约 | C-1（P4 7/7、R3） | `i18n.rs`、8 个组件文件、`css/*`、`extensions_ssg.rs`、`verify.yml` | mbx host；`mbx xtask css` | V-1 |
| W2 | M2 Studio 陈列 | M1 的 part/状态/新参数（V-1） | `examples/theme-studio/*`、`tests/browser/theme-gallery.spec.ts`、`gallery-matrix.ts` | themePort、release 构建（约 3 min）单占 | V-2 |
| W2′ | M3 迁移事实源 | M1 最终接口（V-1） | `catalog.rs`、`catalog/legacy.rs`、`legacy_baseline.txt`、`docs/components.md`、5 份文档 | mbx host；**与 M2 共用 `rustify-components` 与 `target/`** | V-3、V-4 |
| W3（串行） | M4 收口 | V-1—V-4；`native-zoom.ts`/`theme-zoom.spec.ts`/official `native 200%` | `tests/*`、`docs/validation/dom-components-p5/` | 基线 worktree、themePort/themeDeepPort/official 4198—4200、headed Chrome、`$TMPDIR/rustify-p5-old-studio` | V-5→V-6→V-7→V-8→V-9→V-10→V-11→V-12→V-13 |

- **唯一建议的可并行点**是 W2 ∥ W2′，但如 F-07 所述，M3 的 `catalog.rs`/`legacy.rs` 与 M2 的 release/测试共用 `rustify-components` 和 `target/`。建议改为「M3 目录数据落定 → M2 构建与浏览器」或直接串行；不要在同一 target 上并发 mbx。
- **M4 内部无并行空间**：V-7/V-9/V-10 共用 Studio 产物、端口与 `docs/validation/dom-components-p5/`；V-11 明确要求无其他构建/测试并行；V-8 与 V-7 串行。计划「每命令一个 project」「不同时启动两个 Playwright 运行」与 AGENTS.md 一致。
- **共享资源负责人**：`target/`、themePort/themeDeepPort、official 4198—4200、`docs/validation/dom-components-p5/`、`$TMPDIR/rustify-p5-old-studio` 均应由主 agent 单点调度；旧站 `RUSTIFY_THEME_OLD_SITE` 与当前站不要同时起同一端口。
- **时序措辞**：§9.1 V-5 写「M4：收口首项」，§9.3 写「M4 最先执行：旧产物」，§10 M4 又先建基线产物；三者对「首项」的指代不同。执行上可归一为「基线产物构建 → V-5 → …」，不构成缺陷，但建议在 §9.1 V-5 改成「M4 检查序列首项（基线产物构建在此之前）」。
- **阶段收口**：M4 是唯一阶段收口，承担 11 条 AC 与 8 条整条终验；V-13 的「AC → 证据 → 豁免」表与 missing 清单是本期的收口负责人产物，计划已写。局部里程碑不会冒充阶段或跨期终验。

## 5. 总体结论与修订顺序

是否需要修改：**需要**

建议状态：**Proposed**（当前计划文本仍有 8 项 P2；完成修订并复核后转 Ready；无 Blocked 项）。与原文自报 Ready 的差异：自报 Ready 的依据是方案与前置已定，但按原文执行时 V-2 的 axe 门没有任务（F-01）、Empty 样本会撞矩阵断言（F-02）、V-10 不能选中真实 Chrome（F-03）、V-4 会在锁文件处失败（F-04）、CommandPalette 空态 ARIA 与 V-2 axe 门矛盾（F-05）、Splitter 短 `mins` 会 panic（F-06）、M2/M3 并行会污染构建（F-07）、选中态焦点对比没有测量（F-08）。这些都是本地可修订的文本/任务/检查问题，不需要用户或外部提供答案，因此不是 Blocked；修订后可以按 Ready 继续。

核心设计判断保留：D1 留在现有 crate、D2/D3 主题与焦点方案、D4 part/state、D5 只补真实状态、D6 reactive 文案、D7 ARIA 结构修正、D8/ADR-1 legacy 事实源、D12 真实 200%、D13 不升版、C-6/C-7/C-8 收窄均已有代码/配置证据支持（facts 与 architecture 两视角独立核对；D2 对比度复算 1.83→7.19/9.86，D3 的 `outline-none` + `outline-solid` 与 `outline-current` 特异性成立，Splitter 上限公式与 `drag` 夹取一致）。本轮不需要另起方案。

### 修订顺序

1. **先修 M1 的契约矛盾与崩溃边界**：F-05（CommandPalette 空态 ARIA）、F-06（Splitter `.get()`）、F-11（§5.2「ARIA 不变」限定为 §5.4）、F-09（HEADER_BUTTON 字面量）；同时决定 F-12（DropZone 文案）与 F-13（选中属性统一）。这些直接影响 §5.1—§5.5 的实现和 V-1。
2. **再修 M2 的验收任务**：F-01（axe 扫描写入 §1.2/M2）、F-02（Empty 样本的 Restore 动作）、F-08（选中/Invalid 对比测量）、F-16（Invalid 理由文本）。这些直接影响 V-2/V-9 能否按原文执行。
3. **再修 M3/M4 的验证身份**：F-04（V-4 去 `--locked` 或更新 lock）、F-03（V-10 设 `RUSTIFY_CHROMIUM` 或明确 helper 解析）、F-07（M2/M3 串行边界）、F-14（`Message::all()` 与 `semantic:` 范围）、F-10（`catalog.rs:220`）、F-15（既有真实缩放用例的取代/复用关系）。
4. **最后做文档措辞**：§9.1 V-5 的「收口首项」、§10 的并行说明、§12 的清单补充；不改变设计。

### 解除条件与下一步

- 无用户/外部阻塞；不需要 `resolve-blocked`。修订 P2 后，用报告中的复核点逐项核对，即可把 readiness 从 `dev-plan-reviewed` 提升为 `dev-plan-ready`。
- 修订与复核由用户按既有文档处置流程发起，不自动改计划；本报告不构成实现完成、合并或发布授权。
- 下一步（按当前客户端前缀）：

```sh
$apply-doc-review docs/plan/DOM-COMPONENTS-P5.review.md
```

若客户端使用 `/` 前缀，则为 `/apply-doc-review docs/plan/DOM-COMPONENTS-P5.review.md`。

## 修订处置与复核

- 来源：本报告原版本 SHA-256 `6f1f120f1abd26cf8c2808873974800fbbbf551ed4167037663b20ac9a677adb`；F-01—F-16 与 §1—§5 原文、原结论不变。
- 目标：[DOM-COMPONENTS-P5](DOM-COMPONENTS-P5.md)；修前 SHA-256 `d83974457f632ca188b6703116ffb0d9d267a67f9694f7eecfa6bb9998ec07d3`（与所审版本相同），修后 SHA-256 `655c067ddc32acf1d39d97bbe4988585627895975c6ee10dd2f96e6f3222a39f`，735 行。
- 复核日期与代码基线：2026-11-04（Asia/Shanghai）；main / `3735889c89319be11b0d25857f2a2730d96c6e97`，dirty；产品源码与测试未改，本次只改计划、本报告与台账。
- 本次范围：全部 F-01—F-16、§2 待核实项与 §4.2 时序措辞。这是作者修订复核，不是新一轮独立评审。

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果 / 后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。`tests/browser/` 无 axe 引用；`@axe-core/playwright` 已在根 package.json，`tests/docs-ssg/a11y.ts` 已用 | 已修订并复核 | §1.2、§6 新增「对比与 axe 用例」、§7 WORK-N1、V-2、M2 | 四处同现且范围一致；CommandPalette 在 Layer 内不属于节容器，故单独扫其 Layer。实际命中由 M2 跑 V-2 确认 |
| F-02 | 成立。`theme-gallery.spec.ts:284` 要求 Restore；`gallery.rs` Select、Table 样本已配套 | 已修订并复核 | §6 DataTable、Tree 行与「状态」、§12 | 两行写出 `Restore rows`、`Restore tree`；spec 通过由 M2 确认 |
| F-03 | 成立。两份配置的 `executablePath` 都只取 `RUSTIFY_CHROMIUM`；V-11 已设、V-10 未设 | 已修订并复核 | V-10 两条命令、§2 `native-zoom.ts` 行、§4 真实 Chrome 行 | 浏览器选择唯一：只取该变量，未设置或路径不存在则 skip 并记未运行；记录路径与版本 |
| F-04 | 成立。基线 `tests/dom-only/hydration/Cargo.lock` 存在且无 `rustify-colors`、`cssparser`；当前 SDK 依赖它们 | 已修订并复核 | V-4、§9.3 角色模拟「权限」 | 命令去掉 `--locked`，写明 lockfile 更新是迁移的一步、不记卡点 |
| F-05 | 成立。`command_palette.rs:204,207` 恒输出；`active` 只在打开时设定，过滤后不清空 | 已修订并复核 | §5.4、V-2 | §5.4 写出 `ul`、`aria-expanded`、`aria-activedescendant` 的空态取值；同时覆盖“有结果但 `active` 被滤掉”的 IDREF 悬空 |
| F-06 | 成立。`drag` 与 `aria-valuenow` 都用 `.get()`；`drag` 确实保持两侧之和不变 | 已修订并复核 | §5.4 Splitter | 公式带 `.get()`，`sizes` 同样容忍，并写明 max 稳定 |
| F-07 | 成立。`lib.rs` 无条件导出 `catalog`，`gallery_catalog.rs` 测试直读 `STABLE_REFERENCES` | 已修订并复核；选 M2 → M3 | §10 首段、M3 行 | 无“可并行”字样。选 M2 在前：M2 的浏览器验收可能暴露 SDK 接口缺陷，M3 的目录与指南应写定型后的接口 |
| F-08 | 成立且比报告更深：矩阵用 `background(node.parentElement)`，Tree 选中项的 accent 背景在自身，按报告原修法会量到错误的背景；FilePicker 的错误边框在 `::file-selector-button`，读元素边框也量不到；V-2 只跑代表名单，新样本不进矩阵 | 已修订并复核；扩大修法 | §6「对比与 axe 用例」、M2、V-2、V-9、§12 | 选中焦点与 Invalid 的步骤、背景口径、伪元素读取和名单都已写明；实测值由 V-2、V-9 给出 |
| F-09 | 成立。`view.rs:11,14,18` 三种形态 | 已修订并复核 | §5.1 | 三处字面量与源码逐字一致。panel_tabs、splitter、tree、file_picker 的字面量也已核对，无需改 |
| F-10 | 成立，但报告的复核口径过宽：`catalog.rs:143,1549,1698` 的 six 指类别、配置控件、能力维度，不能清零 | 已修订并复核；不采纳“无 `six` 残留”判据 | §5.7 | 列出 `icon.rs:6`、`catalog.rs:215,220,935` 和不改的三行；生成物按 Icon 行与 `icon` 类别核对 |
| F-11 | 成立 | 已修订并复核 | §5.2 首句 | 不再与 §5.3、§5.4、D7 互斥 |
| F-12 | 成立。`drop_zone.rs:43-48` 一次取值 | 已修订并复核；选改为 reactive | §5.4 DropZone、§5.5、§6 样本行、V-2 | 样本不传 `label`，V-2 切 zh 能直接看到结果 |
| F-13 | 成立 | 已修订并复核；选统一 `data-selected` | §5.2 两行与说明 | 仓内 SDK 无 `data-selected` 或 `data-state=selected` 消费方，改动无迁移成本；V-1 按 §5.2 断言，无需改文字 |
| F-14 | 成立。基线 `Theme::properties()` 恰 23 项；基线 `sdk.css` 另有 10 个 `--color-*` 别名（`card-foreground`、`popover-foreground`、8 个 `sidebar*`），同样是颜色语义 | 已修订并复核；扩大范围 | §3 引言与第 7 条、§3.1 `semantic:`、M3 | D14 也已核对：`--primary`、`--accent-foreground` 默认值变化，`--ring` 不变。键从基线机械提取，提取范围写入 M3 记录 |
| F-15 | 成立。既有用例用 settingsPrivate，默认跑 Playwright Chromium；P4 未实跑，当前是否通过未知 | 已修订并复核；选并存 | §1.1、D12、§4、§9.4 | 既有用例零改动，随 V-7 作回归；真实 Chrome 结论只来自 V-10。不合并：合并后 regression 要么依赖本机 Chrome，要么失去 GPU 对比路径 |
| F-16 | 成立。`file_picker.rs`、`drop_zone.rs` 已有 `described_by` | 已修订并复核 | §6 样本行、V-2 | 样本提供理由并关联；SDK 不判断文件内容 |
| §4.2 时序措辞 | V-5「收口首项」与 §9.3「最先执行」指代不同 | 已修订并复核 | V-5 | 写明基线产物与 M4 交付物在 V-5 之前 |

### 待核实项（报告 §2）

| 项目 | 本次结果 |
| --- | --- |
| `rustify-components 0.2.1` 是否已发布 | 已解决：crates.io 索引有 0.2.0、0.2.1，均未撤回。M3 把 README 改为 `=0.2.1` 成立，D13 的“registry 0.2.1 不兼容”也成立 |
| `mbx test -p theme-studio --bins` 是否有测试 | 已解决：[P4 M5 记录](DOM-COMPONENTS-P4.records/M5.md)为 72/72，`gallery_catalog.rs` 含 `#[cfg(test)]` |
| V-11 旧侧输出路径与 §9.3 的 `cp` | 已解决：基线 `theme-performance.spec.ts:83` 写入 `docs/validation/theme-studio/M6/performance.json`。旧 spec 能否在当前 Chrome 跑通仍待 M4，§11 已写失败处理 |
| `tests/native-zoom.ts` 的 Chrome 解析 | 已解决：§2、§4 已定；profile 清理按实现确认 |
| VoiceOver、IME、触控；`extensions_ssg` 首跑体积；axe 4.13 实际命中；P4 R3 范围 | 仍按报告留到实施期，均不阻塞；计划不需要改 |

### 决策与未完项

本次没有需要用户决定的事项。F-07、F-12、F-13、F-15 的选择都是可逆的实现口径，不改变产品范围、C-6/C-7/C-8 或任何 AC。无未完的修订项。

### 修订后结论与验证

- 16 条全部为「已修订并复核」，其中 F-08、F-14 扩大了报告修法，F-10 不采纳其复核判据；无误报，无待决。计划正文判定 **Ready**：P2 涉及的任务、命令和契约都已落到可执行的文字，无 Blocked。这只是文档判定，不代表实现或验收通过；实施仍 0/4，恢复快照未变。
- 实际运行：`check_paths.sh` 退出 0，32 个路径中 6 个 MISSING，全是 ★ 新增路径；`check_progress.sh` ERROR 0、WARN 1（计划未跟踪，git 判定为“未修改”，尚未开工，可忽略）；`check_plan.py` ERROR 0、WARN 10（上游 WORK-* 自定义 ID 的提示，这些 ID 已在账本且均有引用）。回读改动并搜索旧口径，`mins[index]`、`data-state=selected`、`可与 M2 并行`、`一次取值`、`ARIA 不变`、带 `--locked` 的 V-4 命令均已无残留。
- 未运行：`mbx`、Playwright、浏览器和任何构建。§6 的 axe 与对比用例、`Restore` 样本、V-4 编译结果都要到 M2、M3 才有实测，上表的“复核”只覆盖计划文字。
- 未修改的关联文档：PRD 不受影响；P4 文档中的“真实浏览器 UI 缩放归 P5”仍然准确。
