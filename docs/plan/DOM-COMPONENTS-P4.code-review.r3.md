# DOM-COMPONENTS-P4 代码评审 R3

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3 | [DOM-COMPONENTS-P4 · 完整导航与参考交互](DOM-COMPONENTS-P4.md) v0.2 | 修复差异 `38bb272ae343faa8816080488f0189fda5ab68bc..f5f352574f0f32aeea8b72caa80ed6cc28b52b60`（5 文件：`catalog.rs`、`docs/components.md`、计划、M7 记录、R1 报告入库）；`f5f35257..3735889c89319be11b0d25857f2a2730d96c6e97` 只改计划、M7 记录并入库 R2 报告；工作树 dirty@`3735889c`，tracked 指纹 `4aa18aeaecde270f`、untracked 内容指纹 `e24cf8d52418db6e`，均无代码改动 | [R2](DOM-COMPONENTS-P4.code-review.r2.md)（sha256 `7eeae04e9a5da3a485af1248ce0106595b709ffe38472cba3dc1683ee615af3c`） |

本轮是**修复效果复审**：覆盖 R2 的全部问题（F-03—F-05）与待核实/待决策项，并检查修复 `f5f35257` 新引入的缺陷。R2 已关闭的 R1 各项不重开，仅确认修复未触及它们的回归面。R1/R2 对原始实现（`0b6f0d6c^..16ed128b`）与 R1 修复（`16ed128b..38bb272a`）的结论保持不变。

## 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R2 / F-03（P3） | `Menubar`、`Primitives.Menubar` 两条目录把 `value`/`on_value_change` 各列两次，生成文档字段列重复 | [M7 记录 R2 节](DOM-COMPONENTS-P4.records/M7.md) 声明去重并新增字段不重复断言；提交 `f5f35257` | **已解决** | [catalog.rs:1053](../../crates/rustify-components/src/catalog.rs)、[:1054](../../crates/rustify-components/src/catalog.rs) 两条目各只保留一份（25/21 项字段）；[catalog.rs:1789-1794](../../crates/rustify-components/src/catalog.rs) 在 `for entry in COMPONENTS` 循环内以 `BTreeSet` 断言字段不重复（`BTreeSet` 已有 import，catalog.rs:1706）。独立验证：`RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib public_entries_and_property_groups` → 1 passed；对 `docs/components.md` 全表逐行去重计数，[components.md:171-172](../components.md) 无重复且全文件无任何重复字段行；`mbx xtask catalog --write docs/components.md --check` → no drift（181/97/24） | 无 |
| R2 / F-04（P3） | `MENU_CHECKS` 把不触达菜单的 `parity.spec.ts` 列为 12 行目录的验证路径 | M7 记录声明从 `MENU_CHECKS` 移除并重生成文档；提交 `f5f35257` | **已解决** | [catalog.rs:865-868](../../crates/rustify-components/src/catalog.rs) 的 `MENU_CHECKS` 只剩 `menus.spec.ts` 与 `menus_ssg.rs`；`grep parity.spec` 在 `catalog.rs` 与 `docs/components.md` 全文均为零命中，[components.md:167-176](../components.md)、[:371](../components.md)、[:373](../components.md) 的验证列只列上述两项；`catalog --check` 无漂移证明生成物同步。菜单两表达等价证据仍在 `menus.spec.ts` 的双项目用例（M7「两表达证据覆盖」），移除不损失证据指向 | 无 |
| R2 / F-05（P3） | 计划 §3 第 4 条、§5.1 第 2 条仍承诺菜单 Trigger 输出 `aria-haspopup`/`aria-expanded`/`aria-controls`，与 F-01 修复后的 ContextMenu 行为矛盾 | M7 记录声明就地修订两处正文；提交 `f5f35257` | **已解决** | [DOM-COMPONENTS-P4.md:395](DOM-COMPONENTS-P4.md) 补 ContextMenu Trigger 例外（只输出 `data-state`，Content 经 `aria-labelledby` 关联，与 Radix 一致）；[:423](DOM-COMPONENTS-P4.md) 括注「ContextMenu Trigger 除外」并回引 §3 第 4 条；与 §5.2 ContextMenu 行（[:469](DOM-COMPONENTS-P4.md)）及 R2 已核实的 [context_menu.rs:44-58](../../crates/rustify-components/src/primitives/context_menu.rs) 实际输出一致，正文内部不再矛盾 | 无 |
| R1 / G-01、F-01、F-02、Indicator 偏差（R2 已判已解决） | R2 逐项确认关闭 | R2 结论；修复 `f5f35257` 未触及 `context_menu.rs`、`choice.rs`、`gallery.rs`、`menus_ssg.rs`、`menus.spec.ts` 等任何相关行为文件 | **不重开（无回归面）** | 修复差异只含目录数据、目录测试断言与文档；`catalog --check` 与 repo_tests 断言通过，目录内容与 G-01 关闭口径（yes/Partial 入口）一致 | 无 |

**旧问题解决情况**：R2 遗留的三条 P3 全部解决，各有独立执行证据；R2 已关闭项无回归。**修复引入的新问题**：逐行审阅 `38bb272a..f5f35257` 全部 5 个文件（`catalog.rs` 3 处改动、`components.md` 12 行重生成、计划正文 2 处、M7/快照回写、R1 报告入库），未发现新缺陷；新增断言位于既有测试函数内、语义与字段列表契约一致。

## 确认问题

未发现可确认的问题。

## 待核实/待决策项

- **默认调色板变化未写入迁移说明**（待决策，承接 R1/R2，不阻塞）：`sdk.css`、`Theme::light()` 把默认 `primary` 改为 `#175cd3`、`accent-foreground` 改为 `#0c111d`，`--ring` 仍为 `#1570ef`；`docs/dom-components-migration.md` 与 `docs/themes.md` 无对应段落。修复 `f5f35257` 未触及；计划快照仍标「待负责人决定」。需用户拍板是否补迁移说明及 `--ring` 是否随 primary 调整，不属于代码缺陷。
- **ContextMenu 模态下再次右键 Trigger**（承接 R1/R2，本轮未复核）：`f5f35257` 未触及 `context_menu.rs`，无新复现或反证，维持 R2「可见行为与 Radix 一致，仅应用拒绝关闭时与计划文字有差异，不作问题」的记录。
- **浏览器产物逐字节对应**（承接 R2，本轮收敛）：`f5f35257` 只改目录数据、测试断言与文档，无运行时代码，浏览器产物无需重建；该疑点对本修复不再适用。后续若再改运行时并跑浏览器，仍需按提交重建产物。

## 覆盖与验证

- 意图：对 R2 三条 P3（目录字段重复、验证路径失指、计划正文未同步）的修复 `f5f35257` 做修复效果复审，并检查修复引入的新缺陷。
- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，2026-11-04，分支 `main`，HEAD `3735889c89319be11b0d25857f2a2730d96c6e97`。
- 范围解析：计划完成记录登记 `0b6f0d6c`、`60c0a367`、`16ed128b`，修复提交 `38bb272a`（R1）、`f5f35257`（R2）；上轮报告取最大轮次 R2，其代码目标为 `38bb272a`。`git rev-parse` 逐个验证，`38bb272a..f5f35257` 为本轮修复差异（5 文件）；`f5f35257..HEAD` 与工作树 tracked/untracked 改动均为台账、报告与计划进度（指纹见评审对象），按规则排除。R2 报告 sha256 与 M7 记录登记值一致。
- 修复差异逐行阅读：`catalog.rs`（`MENU_CHECKS`、两条 Menubar 条目、`inventory_tests` 断言）、`docs/components.md`（12 行）、计划 §3 第 4 条/§5.1 第 2 条、M7 记录 R2 节与快照、R1 报告入库。
- 相关上下文：`inventory_tests` 全函数与 `BTreeSet` import、`components.md` 全表字段列、计划 §5.2 ContextMenu 行、M7 记录 V-R7/V-R8、R2 报告复审表与待核实项。
- 实际执行（本轮）：`mbx xtask catalog --write docs/components.md --check` → no drift（181 条目/97 参考名/24 类）；`RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib public_entries_and_property_groups` → 1 passed（含新去重断言）。其余为静态阅读与 `git`/`grep`/awk 取证。
- 未覆盖：原始实现（R1 范围）与 R1 修复（R2 范围）未重审，沿用其结论；`cargo fmt`/clippy/css check 按引用（M7 V-R8 已跑，本轮改动未再触及 Rust 运行时与 CSS，证据未失效）；浏览器项目未重跑（修复不含运行时行为）。
- 复审分工：单一 reviewer（本 agent），无并行评审。

## 计划符合性

计划 [DOM-COMPONENTS-P4.md](DOM-COMPONENTS-P4.md) v0.2，本轮对象是 R2 修复而非新里程碑。

| 需求/章节 | 本轮证据 | 状态 |
| --- | --- | --- |
| §3 第 4 条、§5.1 第 2 条（ContextMenu Trigger ARIA 例外） | 正文已补例外并与 §5.2、代码一致（F-05 关闭） | 已解决 |
| WORK-N5 / V-15（目录与生成文档准确） | 字段无重复、验证路径不再指向 `parity.spec.ts`、`catalog --check` 无漂移、新增字段去重门禁 | 已解决（F-03/F-04 关闭） |
| 其余 10 条 AC 与四条整条终验 | 修复未改行为链；证据沿用 R1/R2 与 M4—M7 记录 | 未重审（引用记录） |

## 总体结论与修复顺序

**验收判定：通过验收。** 正确性结论（仅覆盖修复差异 `38bb272a..f5f35257`）：`patch is correct`。修复效果复审结论：**已解决**——R2 的 F-03、F-04、F-05 逐项在源码、生成文档、计划正文与独立执行的门禁（repo_tests 去重断言、`catalog --check`）上确认关闭，三条均按 R2 给出的修复方向最小实施；修复差异未引入新缺陷，R2 已关闭的 R1 各项无回归面。

无剩余 P0–P3 问题。唯一遗留是待用户决策的默认调色板迁移说明（承接 R1，非缺陷），不阻塞本判定。本结论只覆盖所审提交与工作树，不构成合并、发布或需求验收授权。

下一步：计划全部里程碑与三轮评审均已收口；如用户决定补默认调色板迁移说明，可另行小改后再审。变更不涉及新增界面路径，UI 验收（M7 已含完整矩阵）无需追加。
