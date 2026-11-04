# DOM-COMPONENTS-P4 代码评审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 | [DOM-COMPONENTS-P4 · 完整导航与参考交互](DOM-COMPONENTS-P4.md) v0.2（§3.7 已就地修订） | 修复差异 `16ed128b52803fee557d6f83b4ee139454dc77c9..38bb272ae343faa8816080488f0189fda5ab68bc`（24 文件）；`38bb272a..ebb463fb32a1627ad3ceafaadb556d29bc98f867` 只改计划、里程碑记录与本评审报告；工作树 dirty@`ebb463fb`，tracked 指纹 `1d952ffa3715db21`、untracked 内容指纹 `a05e3d8337b02fa0`，均无代码改动 | [R1](DOM-COMPONENTS-P4.code-review.md)（sha256 `8faed761…`） |

本轮是修复效果复审：默认覆盖 R1 的全部问题、待核实/待决策项与修复本身引入的回归，不重新宣称原始 190 文件改动已全量重审。

## 上轮问题复审

| 来源 | 上轮问题与结论 | 修后版本 | 本轮结果 | 当前证据与验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 G-01（计划符合性，阻塞） | 目录与 `docs/components.md` 未反映本期五个新入口，违反 WORK-N5；`catalog --check` 只因源与生成物同样陈旧而通过 | `38bb272a` 重写 `catalog.rs` 条目与 `docs/components.md` | **已解决** | [catalog.rs:1050-1059](../../crates/rustify-components/src/catalog.rs) 新增 Themes/Primitives 两族入口且 `Support::Yes`；参考对应改为指向新模块的 Partial；[components.md:167-176](../components.md) 五模块两表达均为 `helper / yes`，`docs/components.md:371`、`:373` 两条 SubCapability 也指向 `primitives::dropdown_menu`/`toolbar`。独立执行 `mbx xtask catalog --write docs/components.md --check` → `catalog: no drift (181 public entries, 97 stable reference names, 24 legacy categories)`。§3.7 已就地改写为新口径 | 无（新引入的两处目录缺陷另列 F-03、F-04） |
| R1 F-01（P3） | `context_menu::Trigger` 在无角色 `div` 上输出 `aria-haspopup`/`aria-expanded`/`aria-controls`，axe `aria-allowed-attr` 违规 | 删除三项属性，只保留 `data-rustify-part`/`data-rustify-issue`/`data-state` | **已解决** | [context_menu.rs:44-58](../../crates/rustify-components/src/primitives/context_menu.rs) 已无这三项（基线见 `16ed128b:...context_menu.rs:57-67`）；[menus_ssg.rs:190-197](../../crates/rustify-components/tests/menus_ssg.rs) 对 `target`(开) 与 `default`(关) 两个 Trigger 断言只留 `data-state` 且三项 ARIA 都不存在；`mbx test --locked -p rustify-components --no-default-features --features ssr --test menus_ssg` → 13 passed；浏览器 `npx playwright test -c playwright.docs.config.ts --project=dom-only menus.spec.ts -g "names, state references"` → 1 passed，其中新增的 axe `aria-allowed-attr` 全页断言为零违规 | 无 |
| R1 F-02（P3） | `group_root` 对非 Checkbox 组无条件输出 `aria-orientation`，Toolbar 内 `role=group` 的切换组因此违规 | 收窄为 `toolbar ‖ 单选组` 才输出 | **已解决** | [choice.rs:845](../../crates/rustify-components/src/primitives/choice.rs) `aria-orientation=(toolbar || kind == GroupKind::Radio)`，与同处 `role` 推导（`toolbar`→toolbar、Radio→radiogroup、其余 group）一致；[menus_ssg.rs:313-317](../../crates/rustify-components/tests/menus_ssg.rs) 断言工具栏内组 `role=group` 且无 `aria-orientation`，独立 `toolbar=true` 组 `role=toolbar` 且为 `horizontal`；同一 SSG 运行 13 passed；浏览器用例同 F-01 通过 | 无 |
| R1 Indicator 偏差（不阻塞） | `docs/menus.md` 的示例、公开消费者与 Studio 陈列把 `nav::Indicator` 放进 `<List>`，渲染出 `ul > div` | 三处一起移到 List 之后，文档补一句 `Place Indicator and Viewport after List` | **已解决** | [menus.md:116-124](../menus.md)、[public-panel-consumer/lib.rs:131-136](../../tests/dom-only/public-panel-consumer/src/lib.rs)、[gallery.rs:453-456](../../examples/theme-studio/src/gallery.rs) 均已是 `</List>` 之后；其余夹具（consumer、primitives、hydration、menus_ssg）本来就在 List 外 | 无 |

R1 解除阻塞的可观察条件（Menubar/NavigationMenu/Toolbar 为 yes、DropdownMenu/ContextMenu 指向独立模块、`catalog --check` 通过）已全部满足，F-01/F-02 的 ARIA 违规在静态与浏览器两层均已不再出现。

## 确认问题

未发现 P0–P2 问题。以下三条为本轮新增的 P3，均由 `38bb272a` 引入。

### F-03 · [P3] 两条新目录把 `value`/`on_value_change` 各列了一次以上

- 定位：[catalog.rs:1054](../../crates/rustify-components/src/catalog.rs) 与 [catalog.rs:1055](../../crates/rustify-components/src/catalog.rs)（`Menubar`、`Primitives.Menubar` 的 `fields`），生成结果在 [components.md:171-172](../components.md) 的 fields 列。
- 问题：两行的字段数组把 Root 的 `value`/`on_value_change` 与 RadioGroup 的同名参数各收一份，`catalogue_markdown` 用 `codes(entry.fields)` 原样逐项输出，于是公开目录的字段列出现两遍 `value`、两遍 `on_value_change`。该列是“本模块声明的 Rust 字段”的机器生成清单，重复项会被读成两个不同参数；`catalog --check` 只比较源与生成物，不会发现这类内容错误。
- 证据：对 `docs/components.md` 全部表格行做逐项计数，只有这两行的字段列出现重复。
- 修复与验证：声明处去重（或按部件分组写明），再 `mbx xtask catalog --write docs/components.md` 并 `--check`。
- 置信度：高。

### F-04 · [P3] 12 行新目录把不会触达菜单的 `parity.spec.ts` 记为验证路径

- 定位：[catalog.rs:865-869](../../crates/rustify-components/src/catalog.rs)（`MENU_CHECKS` 第二项），影响 [components.md:167-176](../components.md)、`:371`、`:373` 的验证列。
- 问题：`MENU_CHECKS` 把 `tests/dom-only/parity.spec.ts` 列为五个菜单模块与两条 SubCapability 的验证路径，但该文件全文没有一处菜单引用：它只在 `/parity.html` 上跑 checkbox/radio/switch/toggle/toggle-group 的接受/拒绝用例，`tests/dom-only/{consumer,primitives}/src/parity.rs` 两个夹具也不含菜单。目录头明确写“Verification paths name checks and consumers to rerun”，因此这 12 行都在指向一个重跑后不会验证菜单的检查。
- 证据：`grep -ic menu tests/dom-only/parity.spec.ts` → 0；两个 `parity.rs` → 0；M7 的 V-8 两表达证据段把 36 项接受/拒绝用例归给该文件，那些用例的对象是选择控件，菜单证据在 `menus.spec.ts` 的 controlled trees。
- 修复与验证：从 `MENU_CHECKS` 去掉 `parity.spec.ts`（或确实在 parity 夹具加入菜单样本）后重新生成文档。
- 置信度：高。

### F-05 · [P3] 计划 §4 第 4 条、§5.1 第 2 条仍承诺菜单 Trigger 输出弹层状态

- 定位：本次删除处的基侧 [16ed128b:context_menu.rs:57-67](../../crates/rustify-components/src/primitives/context_menu.rs)，目标侧为 [context_menu.rs:52-58](../../crates/rustify-components/src/primitives/context_menu.rs)；计划 [DOM-COMPONENTS-P4.md:395](DOM-COMPONENTS-P4.md)、[:423](DOM-COMPONENTS-P4.md)。
- 问题：F-01 修复后 ContextMenu Trigger 只输出 `data-state`，而计划 §4 第 4 条仍写“菜单 Trigger … 自行输出展开状态与开启时的 `aria-controls`”，§5.1 第 2 条更具体地写“菜单 Trigger（§5.2）… 并输出 `aria-haspopup=menu`、`aria-expanded`、只在开启时 `aria-controls`”。§5.2 的 ContextMenu 行（[:469](DOM-COMPONENTS-P4.md)）没有这些承诺，两处正文现在互相矛盾。本轮 §3.7 已随修复就地改写，这两处没有跟进，后续验收或维护会按 §5.1(2) 认为 ContextMenu 缺了 §5.1 的行为。
- 证据：`dropdown_menu.rs:95-104` 与 `menu.rs:556-557` 仍输出这三项（与正文一致），`context_menu.rs` 的 `decorate` 已不再输出；M7 的「评审修复」段已记录该偏差与理由，但按计划约定偏差应就地修订正文。
- 修复与验证：在 §4 第 4 条、§5.1 第 2 条补一句例外（自定义目标保留自身角色，只输出 `data-state` 与 `aria-labelledby` 关联），或与 §5.2 口径统一。
- 置信度：高（正文与代码直接对照）；若作者本意是这两条只约束 DropdownMenu 系 Trigger，也需在正文写明，否则与 §5.2 冲突。

## 待核实/待决策项

- **默认调色板变化未写入迁移说明**（待决策，不阻塞，承接 R1）：`sdk.css`、`Theme::light()` 与 DESIGN.md 把默认 `primary` 从 `#1570ef` 改为 `#175cd3`、`accent-foreground` 从白改为 `#0c111d`，`--ring` 仍是 `#1570ef`；`docs/dom-components-migration.md` 与 `docs/themes.md` 仍无这次默认值变化的段落（本次修复只加了 `docs/menus.md` 的 Indicator 位置说明）。计划恢复快照已把它列为“待负责人决定”，仍未决定。
- **ContextMenu 模态下再次右键 Trigger**（承接 R1，本轮未复核）：修复未触及该路径，本轮没有新的复现或反证，维持 R1 的“可见行为与 Radix 一致，仅应用拒绝关闭时与计划文字有差异”的记录，不作为问题。
- **浏览器产物与提交的逐字节对应**（承接 R1，部分收敛）：本轮浏览器用例仍跑在既有产物 `target/dom/*`（2026-11-04 15:21 构建）上，没有为 `38bb272a` 重新构建来证明逐字节对应。该疑点对 F-01/F-02 已可收敛——修复前同一页面的 axe 有两条 `aria-allowed-attr` 违规，现在同一断言零违规，说明产物确含修复——但不能推广为“该产物等于提交内容”。
- **未重跑的门禁**：`cargo fmt --check`、workspace clippy、`css --check`、`sources verify`、`dom-primitives` 项目、`theme-studio` bins 与 `gallery` 浏览器矩阵本轮均未执行，按 M7 记录引用；这些记录不构成本轮独立验证。

## 覆盖与验证

- 意图：在 P3 的受控浮层栈上交付 DropdownMenu/ContextMenu/Menubar/NavigationMenu/Toolbar（两种表达）、迁移旧 Menu、加主题来源标记、完成 Studio 组件陈列与 Inspector，并完成 P2—P4 跨期终验；本轮只核修复效果。
- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，2026-11-04，分支 `main`；探针时 HEAD `ebb463fb32a1627ad3ceafaadb556d29bc98f867`。
- 范围解析：计划完成记录登记 `0b6f0d6c`、`60c0a367`、`16ed128b`；R1 判“需要修改”，其修复提交为 `38bb272a`（`fix(components): list menu entries in the catalogue and drop disallowed ARIA`）。`git rev-parse` 逐个解析后在当前分支上；`git merge-base --is-ancestor` 成立。`git diff --name-status 38bb272a HEAD` 只含 `docs/plan/DOM-COMPONENTS-P4*.md` 与 `.xgent-ai/sdlc/*`，故代码目标为 `38bb272a`；`ebb463fb` 只是 R1 的进度回写，按规则排除。
- 修复差异逐行阅读：`catalog.rs`（`MENU_CHECKS`、10 条 component 条目、ReferenceEntry、SubCapability）、`context_menu.rs`、`choice.rs`、`menus_ssg.rs`、`menus.spec.ts`、`candidates.json`、`docs/{components,menus}.md`、`gallery.rs`、`public-panel-consumer/src/lib.rs`，以及同提交的计划/记录回写。
- 相关上下文：`context_menu.rs` 全文 `decorate`/`Trigger`、`dropdown_menu.rs` 的 Trigger ARIA、`menu.rs` 的 SubTrigger ARIA、`choice.rs` 的 `group_root` 角色推导、`catalog.rs` 的生成与 `inventory_tests`、M7「评审修复」段、计划 §3.7/§4/§5.1/§5.2。
- 实际执行（本轮）：`mbx xtask catalog --write docs/components.md --check` → no drift（181 条目/97 参考名/24 类）；`mbx test --locked -p rustify-components --no-default-features --features ssr --test menus_ssg` → 13 passed；`npx playwright test -c playwright.docs.config.ts --project=dom-only menus.spec.ts -g "names, state references"` → 1 passed（1.1s，复用既有产物）。其余内容为静态阅读与 `git`/`grep` 取证。
- 未覆盖：修复差异之外的原始实现未重审（沿用 R1 结论）；未跑 F-04 提到的 `parity.spec.ts`（本轮只用于判断它是否覆盖菜单）；未跑 `dom-primitives` 项目与 CSP/Studio 矩阵。
- 复审分工：单一 reviewer（本 agent），无并行评审。

## 计划符合性

计划 [DOM-COMPONENTS-P4.md](DOM-COMPONENTS-P4.md) v0.2，本轮对象是 R1 修复而非新里程碑。

| 需求/章节 | 本轮证据 | 状态 |
| --- | --- | --- |
| §3.7、WORK-N5（V-15）：目录与生成文档反映五个新入口 | 十个新条目为 `helper / yes`，参考对应为指向新模块的 Partial，`catalog --check` 无漂移 | 已解决（G-01 关闭）；字段内容有新缺陷 F-03，验证列有 F-04 |
| ADR-3 / §5.5 Indicator 在 List 外 | 文档、公开消费者、Studio 陈列三处已改到 List 之后 | 已解决 |
| WORK-N1/AC-2 菜单角色与名称 | F-01/F-02 的违规属性已移除，SSG 与浏览器断言覆盖 | 已解决；§4 第 4 条/§5.1 第 2 条正文未同步（F-05） |
| WORK-F13 AC-1—3、WORK-F15/AC-4、WORK-F1/AC-3、WORK-F19/AC-2、WORK-N3/AC-3、WORK-N6/AC-2、WORK-N7/AC-2 | 修复未改动这些行为链；证据仍来自 R1 与 M4—M7 记录 | 未重审（引用记录） |
| §9.4、§9.5 CI 与配置 | 修复未改 CI 配置 | 未重审（R1 已核实） |

## 总体结论与修复顺序

**验收判定：通过验收。** 正确性结论（仅覆盖修复差异 `16ed128b..38bb272a`）：`patch is correct`。复审修复结论：**已解决**——R1 的 G-01、F-01、F-02 与 Indicator 偏差逐项在代码、SSG 断言与浏览器 axe 上得到确认，相关回归无阻塞。

剩余均为 P3 的文档/目录准确性问题，不影响正确性：F-04 让 12 行验证列指向不会验证菜单的检查，最值得先修；F-03 是生成文档的重复字段；F-05 是计划正文与 F-01 修复后的行为不一致。R1 的待决策项（默认调色板迁移说明）仍待负责人决定，不计入本轮判定。

修复顺序：

1. F-04：从 `catalog.rs` 的 `MENU_CHECKS` 去掉 `tests/dom-only/parity.spec.ts`（或给 parity 夹具补菜单样本），重生成 `docs/components.md`。
2. F-03：去掉两条 Menubar 条目里重复的 `value`/`on_value_change`，重生成同一文档。
3. F-05：在计划 §4 第 4 条与 §5.1 第 2 条写明 ContextMenu 自定义目标只输出 `data-state` 的例外，或与 §5.2 口径统一。
4. 建议一处门禁：`catalog` 增加“同一条目字段不重复”的断言，避免同类内容错误再次绕过 `--check`。

解除阻塞的可观察条件已满足（`catalog --check` 无漂移、五模块两表达为 yes、R1 的两条 ARIA 违规消失）。本结论只覆盖所审计提交与工作树，不构成合并、发布或需求验收授权。
