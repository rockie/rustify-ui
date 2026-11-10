# DOM-COMPONENTS-P5 代码评审 R2（修复效果复审）

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 | [DOM-COMPONENTS-P5 · 原应用迁移与整体交付](DOM-COMPONENTS-P5.md) | 上轮所审目标 `7eba08292e89bd0de1188da60bb93d7bc6a24973` → 当前 HEAD `f12d5220b014bc7c7baa9679f38afb2388ac749b`（评审修复 `857507ba`、SHA 回填 `f12d5220`；产品代码净差异仅 `crates/rustify-components/src/{data_table/view.rs,tree.rs}` 的 `empty_label` 与 `tests/extensions_ssg.rs` 断言，其余为计划/M1/M4 记录与台账）；工作树未提交变更仅 `.xgent-ai/sdlc/` 台账与本 R1 报告（`git diff HEAD -- crates/ tests/ examples/` 为空，无代码对象） | [DOM-COMPONENTS-P5 代码评审 R1](DOM-COMPONENTS-P5.code-review.md)（sha256 `420a612fc5373c2b562a50d273d1c58c28e6322181c519e776b3b01d06870e7d`） |

复审记录：本轮为**修复效果复审**。上轮报告 R1 判「需要修改」（patch is incorrect，F-01 P1 / F-02 P3）；处置见 [M1 记录「评审修复」](DOM-COMPONENTS-P5.records/M1.md#评审修复)（修复 `857507ba` 基于当时 HEAD `118202ba`，其间无其他产品代码提交）。本轮范围：R1 全部问题逐项复审 + 修复 diff（`7eba0829..HEAD`）新增/激活缺陷检查。

## 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01（P1） | DataTable/Tree `empty_label` 未实现，catalog fields、`docs/components.md:77-78`、`docs/data.md`、迁移指南四处虚报该参数；判需实现或改正文档 | 选实现路线：`857507ba` 为两组件增 `#[prop(optional, into)] empty_label: String`，`StoredValue` 持有、空串回退 `Message::Empty`，`extensions_ssg` 增断言 | **已解决** | 实现锚点：[view.rs:88](../../crates/rustify-components/src/data_table/view.rs)、[view.rs:593-601](../../crates/rustify-components/src/data_table/view.rs)（空行 gridcell closure）、[tree.rs:146](../../crates/rustify-components/src/tree.rs)、[tree.rs:294-302](../../crates/rustify-components/src/tree.rs)；空串经 reactive closure 取 `locale.text(Message::Empty)`（语言切换生效）。断言：`extensions_ssg.rs:86`/`:150` 自定义文案（no rows match / no nodes match），`:262`/`:270` 显式空串走 zh 回退（`没有内容` ×2）。本轮实跑 `--test extensions_ssg` 7/7 通过；`mbx xtask catalog --write docs/components.md --check` 无漂移（182/97/24）证明生成文档与实现一致；`docs/data.md:255-258` 与迁移指南 478-479 行表述与实现语义（缺省/空串回退 SDK locale 消息）一致——四处文档全部转为真实 | 无 |
| R1 / F-02（P3） | 计划 §5.4:470 残留 PanelTabs 中间设计（close 移出 tab、删 stop_propagation、mr-1.5），与首段、交付代码和 SSG 断言矛盾 | `857507ba` 改写 TRIGGER 文本块与尾段为终态；未改产品代码 | **已解决** | 计划 467 行 TRIGGER 含 `flex items-center gap-1.5`，与 [panel_tabs.rs:15](../../crates/rustify-components/src/workspace/panel_tabs.rs) 逐字一致；470 行改「close span 嵌在 `role=tab` 内并由 TRIGGER 的 `gap-1.5` 提供间距；close 点击保留 `stop_propagation`」，与首段及 `panel_tabs.rs:181` 的 `stop_propagation` 一致；`grep 'mr-1.5\|self-center\|删除 stop_propagation\|不再嵌套'` 在计划与 M1 记录均无残留 | 无 |

R1 修复建议中附带的「给 fields 与真实 props 加一致性断言防漂移」是针对裁剪路线的可选建议，实现路线下未采纳，不构成未关闭项。

## 确认问题

本轮新增问题：未发现可确认的问题。修复 diff 全量读取（产品代码 22 行 + 测试 8 行）：`StoredValue` 用法与 `panel_tabs.rs:81-82` 既有模式相同；回退 closure 中 `locale.text` 受 track，语言切换可更新；prop 可选不破坏既有调用方；未触及 class 字符串与 CSS（`css --check` 无漂移佐证）；`cargo fmt --all -- --check`、`mbx clippy --locked -p rustify-components --all-targets -- -D warnings` 本轮实跑均通过。

## 待核实 / 待决策项

- V-7—V-13 浏览器/性能套件本轮未复跑，理由：修复未触及浏览器渲染面——prop 缺省时空态输出与修前逐字相同（closure 空串分支返回同一 `Message::Empty` 文案）、无 class 变更（css 无漂移）、Studio 样本不使用 `empty_label`（grep 无命中），M4 在 `7eba0829` 采集的浏览器证据对未变路径继续有效；新行为（自定义文案）由本轮实跑的公开渲染入口覆盖。结论仍采信 M4 记录与 `docs/validation/dom-components-p5/*.json`（记录来源，非本轮实测）。
- `legacy_baseline.txt`（675 键）基线提取完备性继续采信 M3 记录的机械提取方法（R1 同款抽查结论未变）。
- `500022a5` 捆绑的约 1350 个 skills 文件不在计划 §1.2 范围，未评审（沿用 R1）。
- V-12 的 VoiceOver、真实 IME、物理触控仍为 M4 记录在案的 missing（AC 允许列出）。

## 覆盖与验证

意图一句话：对 R1 判「需要修改」的 F-01/F-02 修复效果做逐项复审，并检查修复增量是否引入新缺陷。

- 仓库根 `/Users/rockie/Documents/gh-rockie/rustify-ui`；评审日 2026-11-09。输入 `docs/plan/DOM-COMPONENTS-P5.md`（HEAD `f12d5220`）；上轮 R1 报告 sha256 `420a612f…`；修复差异 `7eba0829..f12d5220`，产品代码改动仅 view.rs/tree.rs/extensions_ssg.rs，已逐行读取；工作树未提交仅台账与 R1 报告（非代码 findings 对象），无 untracked 代码文件。
- 实际命令（当前工作树 = HEAD `f12d5220`，2026-11-09）：`mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg` **7/7 通过**；`mbx xtask catalog --write docs/components.md --check` **无漂移**（182 公开项/97 参考名/24 类）；`mbx xtask css --check` **无漂移**（169645 B / 5561 候选）；`cargo fmt --all -- --check` **通过**；`mbx clippy --locked -p rustify-components --all-targets -- -D warnings` **零警告**。
- 静态复读：计划 §5.4 全节与 `panel_tabs.rs` 终态一致；`empty_label` 四处文档（catalog fields、components.md:77-78、data.md:255-258、迁移指南:478-479）与实现语义一致；M1/M4 记录与恢复快照/完成记录同步（M1 行含 `500022a5；修复 857507ba`）。
- 未覆盖：浏览器行为实测、性能数字复核、SSR/DOM lib 全量与 V-5—V-13（理由见待核实；lib 234/187 为 M1 记录来源）。评审人：主 agent 单独复审，未派并行 reviewer。

## 计划符合性（计划：DOM-COMPONENTS-P5.md @ `f12d5220`）

- R1 缺口已闭合：§3 第 1 条（`empty_label` 参数与空串回退）按承诺交付，WORK-N5/AC-2「目录声明的交互可对应真实结果」与 WORK-F23/AC-2 迁移事实源对该参数恢复为真；§5.4 恢复快照事实源与交付一致（F-02）。
- 11 条 AC 与 8 条整条终验的可追溯口径仍按 M4 记录「AC → 证据 → 豁免」与用户决定 C-6/C-7/C-8；P5 完成不等于 PRD 无保留验收（计划原文口径，本轮未复核浏览器侧）。

## 总体结论与修复顺序

**验收判定：通过验收**

修复结论：**已解决**（R1 两项全部实际解决，0 项不再适用）。修复增量 `7eba0829..f12d5220` 在本轮验证范围内 **patch is correct**：F-01 按计划承诺实现并经当前版本实跑的公开渲染入口验证（extensions_ssg 7/7），四处文档转为真实且生成物无漂移；F-02 计划文本与代码终态一致；修复未引入新缺陷；无 P0–P3 遗留。主路径真实入口证据：新参数行为由当前 HEAD 实测的组件渲染断言覆盖，默认渲染路径未变故 M4 浏览器证据继续适用。本判定仅覆盖所审提交与工作树，不是合并、发布或需求验收授权；浏览器与性能结论仍为采信来源（见待核实）。

下一步（可选）：计划「需求来源」指向 [DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md)，如需核对 PRD 交付差异可运行 `review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P5.md`；界面走查已由 M4 的 V-2/V-7/V-10 覆盖且本次修复未改默认渲染，如需当前版本 UI 复核可运行 `review-ui docs/plan/DOM-COMPONENTS-P5.md`。无剩余 P3，无需 apply-code-review。
