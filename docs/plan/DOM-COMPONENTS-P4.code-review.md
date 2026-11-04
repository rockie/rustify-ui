# DOM-COMPONENTS-P4 代码评审

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 | [DOM-COMPONENTS-P4 · 完整导航与参考交互](DOM-COMPONENTS-P4.md) v0.2 | `0b6f0d6c3519f06d32e464b6517b6da9298877e1^..16ed128b52803fee557d6f83b4ee139454dc77c9` 净差异；工作树 dirty@`e37b17aa`（仅台账、skills-lock 与评审报告，无代码改动） | 无 |

## 确认问题

### F-01 · [P3] ContextMenu 默认 Trigger 在无角色 div 上输出不允许的 ARIA 状态

- 定位：[crates/rustify-components/src/primitives/context_menu.rs:57-67](../../crates/rustify-components/src/primitives/context_menu.rs)（目标 `16ed128b`，与 `:93-98` 默认 `div tabindex=0` 共同触发）
- 问题：`context_menu::Trigger` 默认渲染无 `role` 的 `div`，`decorate` 无条件给它加 `aria-haspopup="menu"`、`aria-expanded` 和开启时的 `aria-controls`。ARIA 1.2 中 `aria-expanded`/`aria-haspopup` 不适用于 generic 角色，辅助技术会忽略，自动化可访问性检查也会把它判为错误。本期新增入口，组件目录、两个消费者和公开消费者都使用这个默认 Trigger，因此每个采用默认写法的应用都会继承这个违规。参考实现 Radix ContextMenu.Trigger 只输出 `data-state`。计划 §5.2 对 DropdownMenu 行承诺了这三项 ARIA，ContextMenu 行没有承诺。
- 证据：在现有构建 `target/dom/dom-consumer`（2026-11-04 14:06 构建）的 `/menus.html?parts` 上跑 axe `aria-allowed-attr`，结果为 `#rui-context-menu-trigger-42 … ARIA attribute is not allowed: aria-expanded="false"`（critical）；参考源 [vendor context-menu.tsx:156-157](../../crates/rustify-components/vendor/radix/primitives-menus/packages/react/context-menu/src/context-menu.tsx)；计划 [§5.2 ContextMenu 行](DOM-COMPONENTS-P4.md)（第 469 行）。
- 修复与验证：ContextMenu Trigger 只保留 `data-state`/`data-rustify-part`，去掉 `aria-haspopup`/`aria-expanded`/`aria-controls`（或仅当调用方 `trigger_as` 元素有支持这些状态的角色时才输出）。在 menus_ssg 增加 context trigger 的属性断言，并在 menus.spec 对 `context-trigger` 跑一次 axe `aria-allowed-attr`。
- 置信度：高（axe 实测 + 规范 + 参考源三方一致）。

### F-02 · [P3] Toolbar 内的 ToggleGroup 在 role=group 上输出 aria-orientation

- 定位：[crates/rustify-components/src/primitives/choice.rs:1681-1708](../../crates/rustify-components/src/primitives/choice.rs)（本期新增 `toggle_group::toolbar_root`，固定 `toolbar: false`）
- 问题：计划要求 Toolbar 内的切换组输出 `role=group`（§3.7、§5.4）。`toolbar_root` 复用 `group_root`，后者对非 Checkbox 组无条件输出 `aria-orientation`（choice.rs:844），于是 Toolbar 新入口的子树里出现了 `role="group" aria-orientation="horizontal"`。`aria-orientation` 不适用于 group，axe 判为 `aria-allowed-attr` 违规。根因在 P2 既有代码，独立 ToggleGroup 的 `toolbar=false` 路径同样存在，但本期 Toolbar 公开入口把它带进了计划要求“角色正确”（WORK-N1/AC-2）的工具栏表面。
- 证据：同一构建与页面的 axe 结果为 `#rui-choice-group-29 (rustify-toolbar-group) … ARIA attribute is not allowed: aria-orientation="horizontal"`（critical）；`group_root` 的属性输出见 choice.rs:844、850。
- 修复与验证：只在 role 为 `radiogroup`/`toolbar` 时输出 `aria-orientation`（改 `group_root` 一处，同时修正独立 ToggleGroup）。在 menus_ssg 的 toolbar 用例断言组内 `role=group` 且无 `aria-orientation`。
- 置信度：高（axe 实测；归属本期的理由是新入口使其进入工具栏表面，根因已标为既有）。

## 待核实/待决策项

- **默认调色板变化未写入迁移说明**（待决策，不阻塞）：`sdk.css`、`Theme::light()` 和 DESIGN.md 把默认 `primary` 从 `#1570ef` 改为 `#175cd3`，把 `accent-foreground` 从白改为 `#0c111d`，以满足 V-10 的 4.5:1。这影响全部既有组件的默认外观，而 `--ring` 仍是 `#1570ef`。`docs/dom-components-migration.md` 与 `docs/themes.md` 都没有提到这次默认值变化。是否需要迁移段落由负责人决定。
- **ContextMenu 模态下再次右键 Trigger**：计划 §5.2 写的是“重写 virtual anchor”，实测为 core 先关闭、Trigger 再打开（请求计数 1→3），菜单在新坐标重新打开，可见行为与 Radix 一致。只在应用拒绝关闭时才与计划文字有差异，未作为问题。
- 浏览器复现使用的是已有构建产物。代码自 `16ed128b` 起未变，产物生成于 M7 期间，但没有重新构建来证明它与 `16ed128b` 逐字节对应。

## 覆盖与验证

- 意图：在 P3 的受控浮层栈上交付 DropdownMenu/ContextMenu/Menubar/NavigationMenu/Toolbar（两种表达）、迁移旧 Menu、加主题来源标记、完成 Studio 组件陈列与 Inspector，并完成 P2—P4 跨期终验。
- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，2026-11-04；HEAD `e37b17aa`（main）。
- 范围解析：计划完成记录登记 `0b6f0d6c`、`60c0a367`、`16ed128b`，三者都在当前分支上。夹在其间的 `bb210433`、`b8a90e59`、`e37b17aa` 只改计划、记录或台账，属于进度回写，排除。登记提交连续，因此审查 `0b6f0d6c^..16ed128b` 净差异（190 文件，其中代码/测试/文档约 128 个）。`git diff --quiet 16ed128b HEAD -- crates examples tests site` 为空。工作树 tracked diff 指纹 `8c792d440c40ea91`，untracked 内容指纹 `230f4e16137ebc01`，均非代码。
- 已逐行阅读：floating 增量、`primitives::menu`（含 model/browser）、dropdown_menu/context_menu/menubar/toolbar/navigation_menu 原语与 browser、`timer.rs`、Themes 皮肤（dropdown_menu surface/Item 及四个包装模块）、`menus.css` 与 `primitives.css` 增量、便捷 `Menu` 重组、`choice.rs` 接管参数、core 的 provider/overlay/toast/theme 增量、Studio 的 inspector.js/inspector.rs/controller/scenes/persistence/provider_fixture/app 诊断、组件目录调用方迁移、`docs/menus.md`、迁移指南、CI 与 Playwright 配置。
- 只抽读：gallery.rs、gallery_catalog.rs、parity 夹具、hydration view、menus_ssg、各 spec。
- 未覆盖：未逐行审 `tests/browser/gallery-matrix.ts`、`theme-gallery.spec.ts` 断言细节、vendor 导入与 `rustify.css`/`candidates.json` 生成物内容（依赖 `css --check`/`sources verify` 的记录结果）。
- 已核对且无问题的行为链：
  - 菜单激活顺序（on_select → 值请求 → 未取消才关根）
  - 子菜单按 `closest('[role=menu]')` 排除父层处理
  - 整树外部关闭监听只在 Sub 计数大于 0 时持有，并经 `rustify_ui::listen` 释放
  - Menubar 用 window capture 先于 core 取消外部关闭
  - Toolbar 对 `is_text_entry` 放行
  - NavigationMenu 外部关闭、计时器与 MutationObserver 经 `release_on_abort` 释放
  - 便捷 Menu 的 canvas Region 锚点被 core `inside()` 视为内部
  - 主题来源属性在 root cleanup 时随 `captureTheme` 还原
  - Toast 备用 DOM ID 全局唯一
- 实际执行（隔离脚本在 scratchpad，未写入仓库）：
  - `npx --no-install playwright test -c <scratchpad>/repro/repro.config.ts`，以 `mbx xtask serve --dir target/dom/dom-consumer --port 4290` 起服务：
    - ① ContextMenu 二次右键：通过，菜单移到新坐标，请求 1→3。
    - ② axe 全页：`aria-allowed-attr` 两个节点（F-01、F-02），另有与本期无关的 `page-has-heading-one`。
    - ③ 把 Indicator 移入 List 后跑 axe `list` 规则：无违规（aria-hidden 子元素被排除）。
- 未重跑：计划 V-1—V-15 的完整命令。本轮依据 M1—M7 记录中的结果，没有独立复现；也未运行 Rust 测试。
- 范围外线索：无。

## 计划符合性

计划 [DOM-COMPONENTS-P4.md](DOM-COMPONENTS-P4.md) v0.2，范围 M1—M7（全部里程碑）。

| 需求/章节 | 实现证据 | 状态 |
| --- | --- | --- |
| §3.7、§1.2 样式/目录行、WORK-N5（V-15）：目录把 Primitives Menubar/NavigationMenu/Toolbar 改为 Yes，DropdownMenu/ContextMenu 改为独立入口，ToggleGroup 记为单组便捷用法，并重新生成 `docs/components.md` | `catalog.rs` 与 `docs/components.md` 在 `0b6f0d6c^..16ed128b` 中**零改动**（最后一次改动是 `ef8b0156`，P3 时期）。当前 [catalog.rs:941](../../crates/rustify-components/src/catalog.rs) 仍写 “not separate DropdownMenu, ContextMenu, Menubar or NavigationMenu APIs”；:1164、:1165、:1180 仍是 Primitives Menubar/NavigationMenu/Toolbar `Support::No` / “No standalone public Rust counterpart”；:1101、:1104、:1157、:1160 仍经 `Menu` 映射为 Partial；:1549 写 “no ContextMenu, Menubar, NavigationMenu or complete Toolbar API”。[docs/components.md](../components.md) 第 64、248、251、304、307、311、312、363 行同样陈旧。`catalog --check` 能通过，只因源与生成物同样陈旧。 | **存在缺陷（G-01，阻塞）**：公开 `CATALOG` 常量、组件目录示例的支持表和生成文档都在告诉使用者本期新入口不存在，违反 WORK-N5 “目录、文档与实际交付一致”。M7 记录把 V-15 记为通过（[M7.md:147](DOM-COMPONENTS-P4.records/M7.md)），不成立。 |
| ADR-3 / §5.5 Indicator：位于 List 外的轨道元素，`ul` 只含 `li` | 内部夹具（consumer/primitives/hydration/menus_ssg）都放在 List 外。[docs/menus.md:119](../menus.md) 示例却把 `<nav::Indicator />` 放进 `<nav::List>`，按文档写的 [public-panel-consumer lib.rs:134](../../tests/dom-only/public-panel-consumer/src/lib.rs) 与 [gallery.rs:456](../../examples/theme-studio/src/gallery.rs) 照抄了。 | 偏差（不阻塞）：渲染出 `ul > div` 的非法内容模型；div 是 aria-hidden，axe `list` 不报，定位不受影响。建议文档、公开消费者和陈列一起改到 List 之后。 |
| WORK-N1/AC-2 角色与名称 | 菜单/子菜单/Menubar/Toolbar/导航的主体角色、`aria-controls` 只在开启时输出、命名逻辑都已静态核对。 | 存在缺陷：F-01、F-02（均 P3） |
| WORK-F13 AC-1—AC-3、WORK-F15/AC-4、WORK-F1/AC-3、WORK-F19/AC-2、WORK-N3/AC-3、WORK-N6/AC-2、WORK-N7/AC-2 | 实现路径已静态核对，与 §5.1—§5.9 一致（例外见上）。浏览器、等价与视觉矩阵证据来自 M4—M7 记录。 | 已核实实现路径；浏览器终验结果未独立复现（引用记录） |
| §9.4、§9.5 CI 与配置 | verify.yml 增加 `menus_ssg` 和 dom-primitives 的 `menus.spec.ts parity.spec.ts`；`menus-performance.spec.ts` 带 EVIDENCE 标签。 | 已核实 |

## 总体结论与修复顺序

**验收判定：需要修改。** 正确性结论：`patch is incorrect`。

菜单、导航、工具栏与 Studio 的核心行为链经静态通读和定向浏览器复现，未发现功能性缺陷。阻塞来自计划本期承诺的目录同步缺失（G-01）：公开 `CATALOG` 与 `docs/components.md` 仍声明本期五个入口不存在。另有两处由本期新入口引入的 ARIA 违规（F-01、F-02，P3）。

修复顺序：

1. G-01：按 §3.7 更新 `catalog.rs` 的 component/ReferenceEntry/SubCapability 条目（新增五个模块入口，Primitives 改为 Yes，Themes DropdownMenu/ContextMenu 指向新模块，ToggleGroup 改为单组便捷用法），再 `mbx xtask catalog --write docs/components.md` 并 `--check`。若 Studio `gallery_catalog` 的 host 单测依赖条目映射，一并复跑 `mbx test --locked -p theme-studio --bins`。
2. F-01：去掉 ContextMenu Trigger 的 haspopup/expanded/controls，补 SSG 断言与 axe 定向检查。
3. F-02：`group_root` 只在 radiogroup/toolbar 角色输出 `aria-orientation`，补 SSG 断言；复跑 `menus_ssg`、`ssr_controls`，以及 dom-only/dom-primitives 的 `menus.spec.ts`、`primitives.spec.ts`。
4. 可选：修正 docs/menus.md、公开消费者与陈列中的 Indicator 位置。

解除阻塞的可观察条件：`docs/components.md` 中 Menubar/NavigationMenu/Toolbar 为 yes，DropdownMenu/ContextMenu 指向独立模块，`catalog --check` 通过。本结论只覆盖所审提交，不构成合并、发布或需求验收授权。
