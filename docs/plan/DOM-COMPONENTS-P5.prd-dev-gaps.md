# DOM-COMPONENTS-P5 · PRD 与开发交付差异核查

> 审查日期：2026-11-09（Asia/Shanghai） · 模式：完整首次核查，非修复效果复审。
>
> 需求来源：[DOM-COMPONENTS PRD v0.2](../PRD-DOM-COMPONENTS.md)（sha256 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`）；开发对象：[DOM-COMPONENTS-P5](DOM-COMPONENTS-P5.md)（sha256 `6fd21ceee784a1852f07832a13df8726bed92c55246be9fc8e029f09a5b59723`）。
>
> 范围：PRD §7.2 落在 P5 的 8 条终验需求、11 条 AC，以及计划明确保留的 UAT-13/UAT-14 边界。跨期 AC 只核对本期终验所引用的结果，不冒充重新执行 P1—P4 全部验收。
>
> 基线：main `f12d5220b014bc7c7baa9679f38afb2388ac749b`，调查时工作树 dirty；既存改动仅为 SDLC 台账和未跟踪的 P5 R1/R2 代码评审，PRD、计划、完成记录、产品代码和测试无未提交改动。本结论不是发布或线上验收声明。

## 1. 结论与范围

**P5 尚未完整覆盖本期 PRD。** 计划 4/4 与代码复审 R2 通过属实，但本次从 PRD 向下追踪发现 7 项明确遗漏、1 项有依据的实现调整，并有 2 组行为条件缺少本期所承诺的浏览器证据。

| 指标 | 数量 | 说明 |
| --- | ---: | --- |
| A · 明确遗漏 | 7 | G-01—G-05、G-07、G-08；含公共迁移事实源、方法身份、颜色语义、错误语义、真实样本、目录事实和证据索引 |
| B · 合理变更 | 1 | G-06；PanelTabs 以 Delete 保留键盘关闭路径，避免已实测的嵌套交互 axe 缺陷 |
| C · 需 Triage | 0 | 本轮没有新的产品取舍需要拍板；待决 0 |
| 待核实 | 2 | U-01 扩展组件最终入口行为补证；U-02 GPU 局部故障期间保存/导出补证 |
| 需跟进 | 9 | 7 个 A 与 2 个补证任务；均由独立 gaps plan 承接 |

11 条 P5 AC 的覆盖状态：3 条已满足、6 条部分满足、2 条已确认变更。C-6/C-7/C-8 的收窄按历史决定保留，但未验证范围不记通过。VoiceOver、OS IME、物理触控仍按 AC-3 的允许口径列为 missing，不计为新增遗漏，也不记实测通过。

## 2. 三类差异

### A · 明确遗漏

#### G-01 · 四个主题组件的 10 个旧参数未进入迁移事实源 · A

- **PRD 期望**：WORK-F23/AC-2 要求每个旧公共入口及旧参数/事件都有 Kept/Replaced/Removed/Changed 去向；WORK-N5/AC-2 要求稳定参考与旧导出逐项可查。
- **实际交付**：`legacy_baseline.txt` 的 675 键没有 `ThemedScope.theme`，`ThemeOverride.{patch,class,test_id,children}`，`ThemeScope.resolved`，`ThemeBoundary.{patch,class,test_id,children}`。`LEGACY_ENTRIES` 仅以 `export:theme.rs::*`、`export:provider.rs::*` 覆盖组件名；其中 `ThemeBoundary.patch` 实际已从基线的 `Signal<ThemeValuePatch>` 变为 `Signal<ThemeBoundaryPatch>`，不能由“同名同义”的导出通配项替代参数级 Changed 说明。
- **证据链**：计划 §3.1 把 prop 提取限定为 components 包 34 个组件 → [M3](DOM-COMPONENTS-P5.records/M3.md) 明示四个主题组件只由 export 键覆盖 → 当前 `catalog/legacy_baseline.txt`、`catalog/legacy.rs`、`catalog.rs::inventory_tests` → 基线 `11b69129` 的 `rustify-dom/src/theme.rs`、`theme/provider.rs` 与当前同名文件。本次静态核实确认 10 个键均不存在；现有测试只证明“已列入的键”解析恰一次，不能发现基线漏列。
- **影响**：使用旧主题接入的消费者无法从承诺的完整对照表逐项查旧参数；尤其 `ThemeBoundary.patch` 的类型/双模式语义变化被隐藏，迁移指南虽另有 `.into()` 适配说明，事实源仍不完整。
- **分类依据**：PRD 明确覆盖旧参数，恢复原要求只需补齐事实源，不涉及新产品取舍。C-6 仅豁免旧主题路径的保留期限，不豁免参数去向。
- **处置**：补齐，不改变旧主题保留决定。
- **Follow-up**：在冻结基线加入上述 10 键；9 个同名参数标 Kept，`ThemeBoundary.patch` 标 Changed 并链接现有双模式/适配说明；再生成 `docs/components.md`。验收由 gaps M1 / V-1 承接。

#### G-02 · DropZone 的错误状态没有 `aria-invalid`，公开指南却声称已接线 · A

- **PRD 期望**：WORK-F22/AC-1 要求空、错误、禁用状态和可访问语义可完成；WORK-N1/AC-3 要求辅助技术下关键状态可感知；WORK-N5/AC-2 要求目录声明对应真实结果。
- **实际交付**：`DropZone` 在 `invalid=true` 时有 destructive 视觉、`data-invalid` 和 `aria-describedby`，但根 `role=group` 没有 `aria-invalid`。相邻 `FilePicker` 已有该属性。迁移指南“Rustify extensions”却把 FilePicker/DropZone 一并描述为具有 `aria-invalid`/described-by wiring。
- **证据链**：计划 §5.3 只明确修 FilePicker 的 ARIA，但 §6 的验收说明又要求 FilePicker/DropZone 的 `invalid`、`aria-invalid` 与拒绝理由关联 → [M1](DOM-COMPONENTS-P5.records/M1.md)、[M2](DOM-COMPONENTS-P5.records/M2.md) → 当前 `drop_zone.rs::DropZone`、`file_picker.rs::FilePicker`、`tests/extensions_ssg.rs`、`tests/browser/theme-gallery.spec.ts`、`docs/dom-components-migration.md`。SSG 与 Studio 用例只断言 FilePicker 的 ARIA，axe 不会替代这个显式状态断言。
- **影响**：读屏用户可听到拒绝理由，却不能从 DropZone 控件状态得知它处于 invalid；公开迁移说明与实际 API 行为不一致。
- **分类依据**：计划最终验收行和迁移指南均明确要求该语义，补一个 reactive 属性及公开渲染断言即可，无产品取舍。
- **处置**：补齐。
- **Follow-up**：DropZone 增加与 `invalid` 同步的 `aria-invalid`；扩展现有 SSG/Studio 断言，保留 disabled 的视觉优先级。验收由 gaps M1 / V-1、V-2 承接。

#### G-03 · Extensions 局部主题样本的 PanelTabs 关闭动作是空操作 · A

- **PRD 期望**：WORK-F22/AC-1 要求工作区操作可完成；PRD §2.1 的组件预览要求真实操作和受控状态，不放假按钮；WORK-F19/AC-3 要求扩展能力在 Studio 真实陈列与检查。
- **实际交付**：`ExtensionLocalSample` 的 `PanelTabs` 显示可关闭的 `shared` tab，但 `on_close=move |_| {}`；受控 `tabs` 不变，点击关闭或按 Delete 均不会移除该 tab。主 `WorkspaceSample` 的关闭逻辑真实，局部主题样本没有复用同等语义。
- **证据链**：计划 §6 要求真实工作区操作 → [M2](DOM-COMPONENTS-P5.records/M2.md) 只记录局部主题/浮层检查 → 当前 `examples/theme-studio/src/gallery_extensions.rs::ExtensionLocalSample`；对照同文件 `WorkspaceSample` 的关闭后选邻接 tab 并更新列表逻辑。`theme-gallery.spec.ts` 只测主样本关闭，没有测局部样本。
- **影响**：用户在用于验证局部主题的样本上遇到一个可见但无效的关闭动作，不能完成同一工作区任务，也使该样本不再是可信的真实预览。
- **分类依据**：行为与 PRD 的“真实操作可用”直接冲突，修复只需接通已有 controlled 逻辑。
- **处置**：补齐。
- **Follow-up**：局部样本按主样本的既有规则更新 `tabs/active`；浏览器验证关闭非活动项不改变活动项、关闭活动项选择邻接项，并保留永久 tab。验收由 gaps M1 / V-2 承接。

#### G-04 · `docs/components.md` 仍错误声称普通 Table 未实现 · A

- **PRD 期望**：WORK-N5/AC-2 要求目录中的能力、差异和限制可查且对应真实结果；UAT-13 要求用户能查到当前能力与限制。
- **实际交付**：`docs/components.md` 的 Source boundaries 仍写“A themed ordinary Table remains unimplemented”。但 `COMPONENTS` 已有 `Table`，`table::Root` 有真实 DOM 实现，Studio 也有 `TableSample`。
- **证据链**：计划 §1.1 已把该句列为缺口，§5.8 要求修正 → [M3](DOM-COMPONENTS-P5.records/M3.md) 却记录“过期句已随再生成移除” → 当前 `docs/components.md`、`crates/rustify-components/src/catalog.rs`、`table.rs`、`examples/theme-studio/src/gallery.rs`。本次静态回读确认旧句仍在；`xtask catalog --check` 只保证生成区一致，不证明手写 Source boundaries 正确。
- **影响**：目录同一份文档对 Table 给出相反结论，消费者可能据此放弃已交付能力；M3 的完成声明与实际文档不符。
- **分类依据**：明确事实错误，不需产品决策，也不应修改历史 M3 记录来掩盖。
- **处置**：修正当前文档，历史记录保留并在新收口记录说明。
- **Follow-up**：删除“未实现”旧句，保留 Table 与虚拟化 DataTable 的区别说明；真实使用验收从公开目录定位 Table。验收由 gaps M1 / V-3、V-4 承接。

#### G-05 · V-11 缺少 P2—P4 性能证据的实际引用 · A

- **PRD 期望**：WORK-N2/AC-2 要求受影响热路径提供前后 p50/p95、尺寸和资源差值；计划 V-11 明确要求引用 P2—P4 的 JSON 与基线。C-7 只豁免这些热路径在最终代码上的重测，没有豁免历史证据索引。
- **实际交付**：M4 的 V-11 写“引用各期证据”，实际只列出 P1 `performance-after.json`。P2/P3/P4 的测量和记录存在，但未进入 P5 的终验链。
- **证据链**：计划 §9.1 V-11、§9.3 → [M4 V-11](DOM-COMPONENTS-P5.records/M4.md) → 本次复核当前五份 P5 性能 JSON 与摘要数值一致，并定位到 [P2 M5 性能](DOM-COMPONENTS-P2.records/M5.md#性能与成本)、[P3 M7 V-13](DOM-COMPONENTS-P3.records/M7.md#v-13-最终性能与成本)、[P4 M7 菜单性能](DOM-COMPONENTS-P4.records/M7.md#菜单性能与资源成本) 及其 `baseline.json`/`current.json`。
- **影响**：终验读者不能从 P5 记录直接追溯各期基线、相同负载和测量口径；这是证据链缺口，不是已证明的性能回退。
- **分类依据**：计划明确要求的收口交付缺失；既有 JSON 可复用，无需重跑或新性能目标。
- **处置**：补齐索引，不重测 C-7 豁免项。
- **Follow-up**：在新 gaps 收口记录列出 P1—P4 的实际证据链接、负载与基线，并明确 M4 的“240 输入→发布”列是发布次数而非时长。验收由 gaps M2 / V-5 承接。

#### G-07 · 旧方法身份被折叠，且已有错误 Kept、漏键与伪键 · A

- **PRD 期望**：WORK-F23/AC-2 要求每个旧公共入口逐项给出真实去向；WORK-N5/AC-2 要求目录声明与当前公开能力一致。
- **实际交付**：方法键只有“文件 basename + 裸方法名”，不能区分接收者。`export:catalog.rs::name` 合并旧 `Category::name` 与 `Support::name`；`export:theme.rs::properties` 合并 `Theme::properties` 与 `ThemePatch::properties`。旧 `ThemeDocument::values/values_mut` 已移到 `base_values/base_values_mut` 或 `effective_values` 语义，却仍被 `export:document.rs::*` 标成 Kept；当前同名方法属于不同的 `ThemeOverrides`。此外，旧/当前均存在的 `ThemeFont::face/slot` 是 `pub const fn`，基线漏列二者，反而产生不存在的 `export:fonts.rs::fn`。
- **证据链**：计划 §3.1 的机械提取 → M3 的完备性声明 → 当前 `legacy_baseline.txt`、`legacy.rs`；本次对照基线 `11b69129` 与当前 `theme/document.rs`、`theme/fonts.rs`、`theme.rs`、`crates/rustify-components/src/catalog.rs`。迁移指南“JSON, CSS and spacing changes”已经给出 ThemeDocument 的正确替代，证明是事实源映射错误而非缺少迁移方案。
- **影响**：消费者会被生成表误导为旧 ThemeDocument 方法仍同名保留；同名方法无法独立审计，face/slot 无法查询，伪键则降低整份对照表的可信度。
- **分类依据**：PRD 要求逐项身份和真实去向，现有行为与文档已有明确替代，无产品取舍。
- **处置**：修正内部键身份和映射，不改变当前公共实现。
- **Follow-up**：基线内 impl 方法统一使用接收者限定键；识别 `pub const fn`，删除伪键；ThemeDocument 两方法标 Replaced，其余同名方法拆开并逐一核对 fate。验收由 gaps M1 / V-1 承接。

#### G-08 · `chart-1..5` 及其回退关系未进入颜色语义去向 · A

- **PRD 期望**：WORK-F23/AC-2 明确要求颜色语义逐项去向；WORK-N5/AC-2、UAT-13 要求当前限制和差异可查。
- **实际交付**：675 键没有任何 `semantic:chart-*`，迁移指南也无对应去向。M3 以“固定色板、非主题语义”排除，但基线与当前 `sdk.css` 均有 `--color-chart-1..5`，且分别回退到 `primary/success/warning/destructive/muted-foreground`；当前 `docs/themes.md` 还将它们列为 ThemeDocument 的 Data tokens。
- **证据链**：计划 §3.1 的 semantic 提取 → M3 偏差说明 → 基线/当前 `crates/rustify-components/css/sdk.css`、`legacy_baseline.txt`、`docs/themes.md`、迁移指南。本次静态核实五组映射在前后一致存在。
- **影响**：使用图表 token 或依赖回退关系的消费者没有逐项迁移事实；“全部颜色语义已覆盖”的声明不成立。
- **分类依据**：PRD 没有该排除，C-6/C-7/C-8 也不覆盖 chart token；行为未变时可标 Kept，但不能省略。
- **处置**：补齐五项及回退说明，无需改变颜色实现。
- **Follow-up**：增加 `semantic:chart-1..5` 的 Kept 映射，destination/reason 写明当前 token 与实际回退，并在颜色语义指南补充说明。验收由 gaps M1 / V-1、V-3 承接。

### B · 合理变更

#### G-06 · PanelTabs 改为可及树外鼠标关闭入口 + Delete 键 · B

- **PRD 期望**：WORK-F22/AC-1、WORK-N1 要求工作区键盘与可访问语义可完成；并未要求关闭必须是独立聚焦按钮。
- **实际交付**：close 是 `aria-hidden` 的 span，不进入 Tab 顺序；用户仍可在可关闭的活动 tab 上按 Delete 关闭，永久 tab 不响应。早期计划表中的“Tab 与关闭按钮分别聚焦”被后续 §5.4 终态取代。
- **证据链**：[M2 偏差](DOM-COMPONENTS-P5.records/M2.md#偏差与说明) 记录三种候选结构的 axe 实测：presentation 子节点提升、tab 内 button、span[role=button] 分别触发 critical/serious → 当前 `panel_tabs.rs`、`tests/extensions_ssg.rs`、`tests/browser/theme-gallery.spec.ts` 验证鼠标关闭不选中、Delete 关闭及永久项边界 → `docs/workspace.md` 与迁移指南已同步终态。
- **影响**：独立聚焦关闭按钮被移除；键盘任务仍有完成路径，鼠标路径保留。辅助技术实测仍为 AC 允许的 missing，不据此宣称完整 AT 通过。
- **分类依据**：有实际可访问性约束和三个备选结构的失败证据，替代行为已实现并有 SSG/浏览器记录，未改变“可关闭 tab”的产品目标，无需机械恢复原结构。
- **处置**：保留终态，不重复 Triage。
- **Follow-up**：无新增产品动作；G-03 的局部样本空操作仍需修复，不能用本项 B 吞掉。

### C · 需 Triage 的实现差异

无。

## 3. 完整需求覆盖账本

证据等级：**文档自述**仅指完成记录；**静态**指本次读源码/文档/JSON；**历史运行**指记录中的测试或浏览器结果，非本轮重跑；**本次实测**仅指明确列出的当前产物检查。

| PRD 需求/验收及定位 | 本期范围/已批准变更 | 计划与里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F19/AC-3 | P5 全量 Studio；GPU 局部故障不阻断保存、导出、DOM 预览；fatal 按实例边界 | §5—§7、V-2/V-7；M2/M4 | M2、M4 V-7、AC 表 | 静态+历史运行：49 组陈列、Inspector、GPU loss 后 DOM 编辑、fatal 后保存/导出/重启有证据；**GPU loss 期间保存/导出未验** | 部分满足 | U-02 → gaps M2 |
| WORK-F22/AC-1 | 保留扩展入口、选择/导航/拖放/文件/工作区、空错禁用与可访问语义；原示例浏览器任务按 C-7 豁免 | §5、§6、V-1/V-2/V-7；M1/M2/M4 | M1、M2、M4 | 静态+历史运行：公共入口和算法存在；DropZone ARIA 与局部 close 有确认缺口；Studio 对多项键盘/真实文件/指针行为未按 §6 验收 | 部分满足 | G-02、G-03、U-01 → gaps M1/M2 |
| WORK-F22/AC-2 | 统一主题、局部覆盖、100000 行窗口化；性能前后比较按 C-7 豁免 | §5—§7、V-2/V-9；M1/M2/M4 | M2、M4 V-9/V-11 | 静态+历史运行：token/ThemeBoundary、100000×6、窗口化与深浅对比证据一致 | 已满足 | 无新增动作；豁免不记通过 |
| WORK-F23/AC-2 | 每个旧入口/参数/事件/Button 变体/颜色语义/主题文件/CSS 有去向；旧主题无边界保留由 C-6 豁免 | §3、§5.8、V-3；M3 | M3、R1/R2 | 静态+历史运行：675 键恰一解析，但主题旧参数漏列，方法身份/Kept 映射和 chart 颜色均有缺口；Button/其余颜色/CSS 已有映射 | 部分满足 | G-01、G-07、G-08 → gaps M1；D-01 |
| WORK-F23/AC-3 | 消费者按说明完成原任务；C-7/C-8 收窄浏览器、SSG 与演示范围 | §3、V-4/V-6/V-7/V-8；M3/M4 | M3 角色模拟、M4 V-6—V-8 | 文档自述+历史运行：编译角色模拟、Studio/官方站浏览器、其余 host/编译；不是全消费者浏览器完成 | 已确认变更 | D-02、D-03；缺证范围不记通过 |
| WORK-N1/AC-3 | 200%、减少动效、关键浮层/导航/导出；AT 可实测或列 missing | §5—§7、V-2/V-10/V-12；M2/M4 | M2、M4 V-9/V-10/V-12 | 静态+历史运行：真实 Chrome 200%、reduced motion、axe 记录存在；DropZone 状态语义未满足；三项 AT/设备 missing 已列 | 部分满足 | G-02 → gaps M1；AT missing 不另判 A |
| WORK-N2/AC-2 | 同机前后 p50/p95、压缩尺寸、旧新分列、无积压/资源增长；C-7 豁免扩展前后和 P2—P4 最终重测 | §7、V-2/V-11；M2/M4 | M2、M4 V-11、五份性能 JSON | 历史运行+本次 JSON 复核：Studio 新旧/当前与 P5 前数值、尺寸、发布次数、idle 一致；P2—P4 索引未接入终验 | 部分满足 | G-05 → gaps M2；D-02 |
| WORK-N4/AC-2 | 根/子路径、文档演示、JSON/CSS、既有 GPU 共享主题；独立消费者等按 C-7/C-8 收窄 | §7、V-6—V-8；M4 | M4 V-6—V-8 | 历史运行+静态：官方站三宿主、Studio 根/子路径、导出与 GPU 预览记录完整；豁免项未验 | 已确认变更 | D-02、D-03 |
| WORK-N4/AC-3 | 最低版本、资产外置、不新增 Node/服务、major 不变 | §3、§7、V-13；M3/M4 | M3、M4 V-11/V-13 | 静态+文档自述：source/published 配对说明、report-size 外置字体、工具链/依赖零差异；本期未重跑 | 已满足 | 无 |
| WORK-N5/AC-2 | 目录尺寸/变体/交互/渲染与真实结果一致；dom-hydrate 浏览器重测豁免 | §3、§5.8、V-3/V-9/V-13；M3/M4 | M3、M4、R1/R2 | 静态+历史运行：R1 的 `empty_label` 虚报已修；仍有主题参数、旧方法/颜色映射、DropZone 文档虚接线和 Table 旧句 | 部分满足 | G-01、G-02、G-04、G-07、G-08 → gaps M1 |
| WORK-N7/AC-3 | 390/768/桌面与真实 200%、局部滚动、浮层/导出、语言切换不改作者数据 | §6—§7、V-2/V-7/V-8/V-10；M2/M4 | M2、M4 | 静态+历史运行：布局矩阵、真实 200% Studio/官方站、导出、locale 快照相等；GPU-offline theme-matrix 的既有 skip 不冒充通过 | 已满足 | 无新增动作；UAT-14 仍受豁免约束 |
| UAT-13（§6.3） | 参考能力、当前入口、差异和验证入口可查 | V-3/V-9/V-13 | M3/M4 | 静态：目录大部分可查，但 G-01/G-02/G-04/G-07/G-08 破坏完整性 | 部分满足 | G-01、G-02、G-04、G-07、G-08 |
| UAT-14（§6.3） | 新旧同任务、降级/资源/响应式留证；按 C-7/C-8 收窄，不宣称全部通过 | V-2、V-7—V-12 | M2/M4 | 历史运行+静态：已有分场景证据；G-03 是实际样本失败，U-01/U-02 和豁免范围未验 | 部分满足 | G-03、U-01、U-02；D-02/D-03 |

P1—P4 的 AC 不在本次重新逐条验收；本账本只使用 P5 终验所引用的历史记录。其他期需求的终验去向仍以 PRD §7.2/§7.3 为准。

## 4. Triage 与决策账本

本轮没有新 C 类问题，未发起新的交互式 Triage。以下历史决定原样沿用，不用它们证明未验证功能已通过。

| 决策 ID | 关联范围 | 用户选择与理由 | 确认日期与来源 | 新目标/验收边界 | 跟进/延期/豁免去向 |
| --- | --- | --- | --- | --- | --- |
| D-01 | C-6；WORK-F23/AC-2 | 保留旧主题路径，当前无删除计划；新应用用 ThemeScope/ThemeBoundary | 历史用户决定；原计划 §0.5、M4 AC 表（本报告未另取确认日期） | 豁免“不保留无边界期限的平行实现”子句，不豁免旧参数逐项去向 | G-01 仍需补齐 |
| D-02 | C-7；F22/F23/N1/N2/N4/N5 | 收窄原示例、扩展性能前后、P2—P4 最终重测及列明消费者浏览器验收 | 历史用户决定；原计划 §0.5、§9.1，M4 AC 表 | 只验计划表列出的 Studio/官方站/host/编译范围；未验项必须明示 | G-05 仅补历史证据索引；U-01/U-02 属保留的 Studio 范围 |
| D-03 | C-8；F23/AC-3、N4 | 完整 SSG 接入、演示构建及对应消费验收不作为 P5 必需交付 | 原计划 §5.8、§9.1 与 M4 提交/验收记录；原始确认日期未单列 | 不把排除项变成本次遗漏，不追加 SSR 功能 | 范围外；未来若重新纳入须另获确认 |

### 待决问题卡

无。

## 5. 待核实项与审查基线

### U-01 · Studio 最终入口的扩展操作证据不完整

- **缺失事实**：本期 Studio 用例未按 §6 验证 DataTable 键盘/滚轮、Tree Arrow/Enter、真实 FilePicker/DropZone 文件、Splitter 指针拖动、不可运行命令的不可激活行为；局部 PanelTabs 关闭还受 G-03 影响。
- **影响**：WORK-F22/AC-1、WORK-F19/AC-3、UAT-14。静态代码和其他示例的历史测试支持实现存在，但不能替代本期最终 Studio 入口证据；此项不预先断言这些功能已坏。
- **补证方式**：扩展 `theme-gallery.spec.ts` 的现有真实样本场景，以公开 UI 完成这些操作并观察输出/状态；重建当前 release 后运行定向 project。
- **阻塞**：不阻塞 G-01—G-05 的修复；阻塞 WORK-F22/AC-1 无保留关闭。

### U-02 · GPU 局部故障期间的保存/导出未验

- **缺失事实**：`theme-acceptance.spec.ts` 的 GPU loss 用例只继续 DOM 颜色编辑；保存命名主题、导出当前草稿是在 fatal 重启后验证，而非 WebGL 故障尚未恢复时。
- **影响**：WORK-F19/AC-3、UAT-14；不能由 fatal 恢复证据替代局部故障要求。
- **补证方式**：在现有 GPU loss 场景内，待 `gpu_state=lost` 后保存命名主题、打开导出并全选当前草稿，再编辑 DOM 颜色并断言预览值；fatal 用例保持独立。
- **阻塞**：不阻塞代码修复；阻塞 WORK-F19/AC-3 无保留关闭。

### 已关闭的疑点与证据边界

- M2 曾记录资源循环中的 Escape 不稳定。本次使用现有 `theme-studio` release（manifest `build_id=94e896b6d611cec3`，strict CSP，headless Chromium）连续完成 20 轮“打开 CommandPalette → Escape → 等待关闭”：**20/20 通过、0 pageerror、最终浮层关闭**，故不列产品缺口，也不另建跟进。前置手工脚本曾因选择器与公开入口不一致失败，修正为现有 spec 的同一入口后得到上述有效结果；这些脚本错误不是产品失败。
- 本次没有重跑 host 全量、浏览器项目、官方站三宿主或性能套件。R2 在当前 HEAD 的 `extensions_ssg` 7/7、catalog/CSS 无漂移、fmt/Clippy 结果作为历史运行证据引用；M2/M4 的其余运行记录同样不冒充本轮实测。
- VoiceOver、OS IME、物理触控仍为 missing；执行步骤已留在 M4 V-12。AC-3 允许列 missing，因此不进入 gaps 必做范围。
- `theme-matrix.spec.ts` 的 GPU-offline 组保留既有 `test.skip(!process.env.RUSTIFY_CHROMIUM)` 边界，本报告不把该组写成已验，也不据此扩大 P5 AC-3 范围。
- 工作树中的既存 SDLC/R1/R2 改动未纳入、未回退；本报告与后续 gaps plan 是本次新增材料。未执行部署、迁移、发布或业务写操作。

## 6. Follow-up 与 gaps plan

| G-ID / U-ID / PRD ID | 确定的增量工作 | 验收条件 | 依赖/阻塞 | gaps plan 任务或排除去向 |
| --- | --- | --- | --- | --- |
| G-01 / WORK-F23-AC2、WORK-N5-AC2 | 补 10 个主题旧参数及正确 fate，生成目录 | 每键恰一解析；`ThemeBoundary.patch` 为 Changed；公开生成节可查 | 无 | M1 / V-1 |
| G-02 / WORK-F22-AC1、WORK-N1-AC3、WORK-N5-AC2 | DropZone 补 `aria-invalid`，校准测试 | SSG 与 Studio invalid=true 可观察；disabled 视觉优先级不变；指南不再虚报 | 无 | M1 / V-1、V-2 |
| G-03 / WORK-F22-AC1、WORK-F19-AC3 | 接通局部 PanelTabs controlled close | 鼠标/Delete 真正更新 tabs/active；永久 tab 保留；邻接选择正确 | 无 | M1 / V-2 |
| G-04 / WORK-N5-AC2、UAT-13 | 修正 Table Source boundaries | 公开目录不再称 Table 未实现，且保留与 DataTable 的区别 | 无 | M1 / V-3、V-4 |
| G-05 / WORK-N2-AC2 | 补齐 P1—P4 性能证据索引与列口径 | 新收口记录可直达各期 baseline/current 与记录；不把发布次数写成时长 | 不重跑 C-7 豁免项 | M2 / V-5 |
| U-01 / WORK-F22-AC1 | 补本期 Studio 扩展行为场景 | §6 保留的键盘、文件、指针、禁用命令和局部关闭从真实 UI 完成 | 依赖 G-02/G-03 修复 | M2 / V-2 |
| U-02 / WORK-F19-AC3 | 补 GPU loss 中保存/导出场景 | 故障未恢复时保存与导出成功，DOM 编辑继续；fatal 证据不替代 | 无 | M2 / V-2 |
| G-07 / WORK-F23-AC2、WORK-N5-AC2、UAT-13 | 修正方法身份、ThemeDocument fate、face/slot 与伪键 | 每个旧方法可独立追踪；两个旧 ThemeDocument 方法为 Replaced；无 `fonts.rs::fn` | 无 | M1 / V-1 |
| G-08 / WORK-F23-AC2、WORK-N5-AC2、UAT-13 | 补 chart 五色及回退关系 | 五个 semantic 键均标 Kept，指南可查到当前 token 与回退 | 无 | M1 / V-1、V-3 |
| G-06 / WORK-F22-AC1、WORK-N1 | 保留鼠标 close + Delete 终态 | 无需动作 | 无 | 明确排除恢复旧结构 |

计划路径：[DOM-COMPONENTS-P5-gaps.md](DOM-COMPONENTS-P5-gaps.md)；状态：**Ready**，实施进度 0/2。只规划上述增量，不重做 P5 已完成里程碑，不修改原 PRD、原计划或历史完成记录。计划结构与引用检查为 0 error / 0 warning；未来记录文件的 missing 提示属于 §1.2 明示的新增占位，初稿进度检查仅有“新计划未纳入既有 git diff”的提示。
