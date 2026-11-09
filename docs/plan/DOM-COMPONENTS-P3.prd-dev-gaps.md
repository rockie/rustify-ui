# DOM-COMPONENTS-P3 · PRD 与开发交付差异核查

> 审查日期：2026-11-03 · 模式：完整核查（非复审） · 审查人：coding agent（review-prd-dev-gaps）
> 2026-11-03 更新：G-02 死遮罩疑点经隔离复现页浏览器实证后提前关闭（见 §2 G-02 证据链）；结论数字已同步（待核实 1→0）。

## 1. 结论与范围

- **PRD**：[docs/PRD-DOM-COMPONENTS.md](../PRD-DOM-COMPONENTS.md) v0.2（2026-10-26，prd-ready）· **期次**：P3（§7.2 落在本期的 12 条 AC）
- **计划**：[docs/plan/DOM-COMPONENTS-P3.md](DOM-COMPONENTS-P3.md) v0.2 / Ready，实施 7/7（M1–M7 已完成，[完整计划评审](DOM-COMPONENTS-P3.review.md)通过）
- **代码基线**：HEAD `c658b831c81f5b2dd9072c904e4b5c3703499868`（main，2026-11-03 13:53 +0800）；工作树 dirty 仅 SDLC 台账回写（R2/R3 对象）与未跟踪的 [R3 报告](DOM-COMPONENTS-P3.code-review.r3.md)，`crates/`、`tests/`、`web/` 无未提交差异。实现提交 `3f23c064`、`4152c4c2`、`ef8b0156`、`ec6bd506`、评审修复 `3d078e29`、`f2bb1bb1` 均在 HEAD 祖先链（[R3 已核](DOM-COMPONENTS-P3.code-review.r3.md)）。
- **核查范围**：PRD §7.2 分配给 P3 的 12 条功能 AC（WORK-F10/AC-1—AC-2、WORK-F11/AC-1—AC-3、WORK-F12/AC-1—AC-2、WORK-F14/AC-1—AC-2、WORK-F15/AC-3、WORK-F17/AC-1—AC-2），加 PRD §7.1/§3.2 要求适用于当期交付物的质量增量（WORK-N1—N8）与计划 §7 登记的前置/回归行（WORK-F1/AC-2、F4、F5/F6、F19、F23）。跨期 AC 只核对本期增量，终验去向保留。
- **验证方式**：完整读取 PRD、计划、M1–M7 完成记录、R1–R3 代码评审；对关键声明做静态代码/测试/产物/CI 抽验（见各覆盖行与 §5）；未重跑浏览器套件——R3 于同一 HEAD 实跑 dom-hydrate 15/15，其后 `crates/`、`tests/` 无改动，既有运行证据直接适用。G-02 疑点另于 2026-11-03 以隔离临时复现 fixture（同 SDK 源、独立 workspace 与站点，用后清理）补充浏览器实证，见 §2 G-02。

**整体结论：在上述基线与证据范围内，P3 期 12/12 条 AC 已满足，质量增量已满足；存在明确遗漏 0、合理变更 1、需 Triage 0（待决 0）、待核实 0、需跟进 0（G-02 已于 2026-11-03 提前实证关闭）。** 报告完成不等于整份 PRD 交付完成：跨期 AC（WORK-F15 其余、WORK-N1/AC-2、WORK-N6/AC-2、WORK-N7 后续、WORK-N1/AC-3 等）按 PRD 分期在 P4/P5 终验。

## 2. 差异项

### G-01 · 模态浮层不输出 `aria-modal` · B（合理变更）

- **PRD 期望**：WORK-F11/AC-1—AC-2（[PRD §302 行起](../PRD-DOM-COMPONENTS.md)，F11 §313）要求模态/非模态 Dialog 与「模态限制本 scope 的底层交互并约束焦点」；§1.5 以 Radix Themes/Primitives 为参考基线（能力对应口径）；WORK-N1/AC-2（P4 终验）将按 APG 校准浮层焦点/模态/关闭/宣告。
- **实际交付**：Dialog/AlertDialog/模态 Popover 的 Content 不输出 `aria-modal`；角色仍按类型指派（`primitives/floating.rs:518`：AlertDialog→`alertdialog`、Tooltip→`tooltip`、其余→`dialog`），模态由 scope 焦点域、pointer-events 与 inert 实现。
- **证据链**：计划 §5.1/§5.3 明确记录口径（scope-local 模态不宣称 APG 整页模态模式）→ M3 交付记录 → [catalog.rs:387/1025/1026/1537](../..//crates/rustify-components/src/catalog.rs) 明示「scope-local modality omits aria-modal」→ 计划评审通过（82810742 对象）→ 本次静态核实上述代码。
- **影响**：屏幕阅读器不会听到「整页模态」声明；本 scope 底层交互确实被限制、焦点被约束，AC-2 的可观察行为满足。对本 PRD 的多 scope/同页多实例架构，声明 `aria-modal=true` 反而会向 AT 谎报页面级模态。
- **分类依据**：有可核实约束（同页多 scope 并存是本 PRD 的既有契约，见 §1.1/§2.6）；替代行为已实现并有浏览器证据（V-3/V-6 模态限制与焦点断言）；口径写入计划、经完整计划评审确认并同步到 catalog/架构文档，产品目标与验收承诺保持。
- **处置**：保留变更。**Follow-up**：无新增动作；P4 WORK-N1/AC-2 终验时按 scope-local 口径复核（PRD 分期既定）。

### G-02 · 打开期间卸载唯一浮层 Content 是否遗留死遮罩 · 不再成立（已实证关闭）

- **PRD 期望**：WORK-N6/AC-2（[PRD §525](../PRD-DOM-COMPONENTS.md)，P4 终验）：触发器移除、内容动态变化、浮层取消后不遗留永久遮罩、不可解除的焦点限制。本期适用增量为生命周期失效路径回归。
- **实际行为**：浏览器实证确认不遗留死遮罩。隔离复现页（`target/g02-repro/`，复刻 `tests/dom-only/consumer` 建独立 workspace，SDK 源未改）以受控 `dialog::Root/Trigger/Portal(force_mount)/Overlay/Content` 装配，Content 包在 `Show` 中可卸载，页面带底层点击计数探针与外部卸载控制。两条卸载路径——内容内按钮、外部信号（`window.__g02.set_content(false)`）——打开状态下卸载唯一 Content 后：close 恰请求一次、open 状态回 false、遮罩 `visibility:hidden` 不再拦截底层真实鼠标点击（计数落地）、Escape 不产生第二次 close、重新挂载后可再次打开并正常 Escape 关闭；console error/pageerror 断言为空。
- **证据链**：R1 提出疑点 → R2 静态反证增强 → [R3](DOM-COMPONENTS-P3.code-review.r3.md) 静态链完整但维持待核实 → **2026-11-03 本次实证关闭**：`mbx xtask build-dom --manifest-path target/g02-repro/consumer/Cargo.toml --output target/g02-repro/site`（同 HEAD `c658b831` 源码）＋ `DOM_ONLY_DIR=target/g02-repro/site npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only g02-repro.spec.ts` → **2 passed（3.5s）**，断言集与 R3 关闭判据一致（遮罩不可见/不可拦截、close 恰一次、Escape 无二次）。静态链（`overlay.rs:771-811/1857-1885/1989-2013/2117-2121`）与实测一致。
- **影响**：疑点不成立，无死遮罩缺陷；WORK-N6/AC-2 的 P4 终验照常覆盖其余生命周期场景（触发器移除等已有 V-6/V-11 证据）。
- **分类依据**：原为待核实；实证未复现失败且行为符合设计（`reported_dead` 护栏下 close 恰一次、整帧隐藏解除拦截），判「不再成立」，非 A/B/C。过程备注：实证中两次失败均为复现装置自身问题（fixture 未保活 `AppHandle` 被 drop 即同步卸载——反向印证 scope 生命周期契约；拦截探测用真实鼠标点击触发 Dialog 默认 outside 关闭，改 `elementFromPoint` 探测后通过），SDK 产品代码零改动。
- **处置**：已关闭（2026-11-03）。**Follow-up**：无；P4 WORK-N6/AC-2 终验照常。

### 非差异观察（不构成缺口，保留去向）

1. **Toast 同 scope 第二个 Viewport 覆盖前一个**：R1 记录并判非缺陷（PRD 未要求多 Viewport）；`rustify-dom` 自 R1 未变，R3 未复核。若 P4/P5 需要多 Viewport，再立需求。
2. **Toast 播报节点显式 `aria-live` 覆盖 `role=status` 播报语义**：R1 记录，判非缺陷。
3. **Tooltip 同功能 p95 +6.9ms**：基线与终测测量端点不同（终测等待 ready 相位），输入/接受计数一致；WORK-N2 增量要求「可解释」，已在 V-13 记录中给出解释。无 SLO 被违反。
4. **消费者 wasm 增长**（raw 11.5→26MB / gzip 1.34→2.72MB，name 段占 72%）：含新增样本与符号名；计划选择保留既有构建口径不为压数字改 stripping；PRD 无体积 SLO，数据如实入 `current.json`。如未来需要压缩，属新决策。
5. **对比度覆盖方式**：默认代表表面实测过门槛（[m7-studio-form-contrast.json](../validation/dom-components-p3/m7-studio-form-contrast.json)：浅/深壳文本 5.571≥4.5、字段边界 3.301/3.463≥3、焦点 4.57≥3）；P3 新增表面未逐一测量，沿用与 P2 相同的代表采样口径，逐表面全量归 P4/P5 终验（WORK-N1 AC-2/AC-3）。
6. **环境备注**：本机 `docs-verify --group features` 末步因 Python 缺 `tomllib` 失败（R2 记录，非缺陷），沿用。

## 3. 完整需求覆盖账本

证据等级：〔静态〕＝本次静态代码/产物核实；〔记录〕＝完成记录登记的运行结果；〔评审〕＝R1–R3 实跑/复核（R3 于同一 HEAD 实跑 dom-hydrate 15/15）。

| PRD 需求/验收及定位 | 本期范围/已批准变更 | 计划与里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F10/AC-1（Tabs 方向/手动自动激活/面板关联；Accordion 单项/多项/允许折叠；Collapsible 触发状态与内容一致，[PRD §302](../PRD-DOM-COMPONENTS.md)） | 本期 AC | §5.2 → M2/M4/M7 | M2（V-2 SSG 8/8、原语 40/40、浏览器 6/6）；M4 V-6；M7 12-AC 表 | Tabs `ActivationMode`/`Orientation`（`primitives/tabs.rs:24,53,224-225`）；Accordion Single/Multiple/collapsible（模型+浏览器）；`primitives/collapsible.rs:86-88` aria-expanded 恒输出、aria-controls 仅 open 输出〔静态〕+〔记录〕 | 已满足 | 默认卸载语义为已确认变更（用户 2026-11-01 裁定复刻 Radix，见 §4） |
| WORK-F10/AC-2（SegmentedControl 单选；TabNav 真实链接；移除/禁用后焦点去处；隐藏面板不被键盘误达） | 本期 AC | §5.2 → M2/M4/M7 | M2 V-2；M4 V-6 | SegmentedControl 复用 radio_group（单选不可清空）；`tab_nav.rs:76,93` aria-current=page、原生 nav/a + `data-rustify-native-navigation`；焦点去处/inert 浏览器记录〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F11/AC-1（Dialog 模态/非模态；AlertDialog 警告确认；标题说明可读；危险默认安全；取消不执行） | 本期 AC | §5.3 → M3/M4/M7 | M3 V-3；M4 V-6；M7 | `floating.rs:518` 角色；AlertDialog Cancel 默认聚焦、Action 仅确认；Title/Description 关联〔静态〕+〔记录〕 | 已满足 | G-01（aria-modal 口径） |
| WORK-F11/AC-2（模态限制本 scope 底层交互并约束焦点；Escape/外部/自动聚焦可取消；AlertDialog 不因遮罩点击确认） | 本期 AC | §5.3 → M3/M4 | M3 V-3；M4 V-6 + Studio 集成 | `floating.rs:425` 非模态不渲染 Overlay；模态 prevent_default+焦点域；可取消 hooks；遮罩点击不确认（浏览器断言）〔静态〕+〔记录〕 | 已满足 | G-01 |
| WORK-F11/AC-3（嵌套层最上层处理关闭；焦点返回；触发器移除返回安全位置） | 本期 AC | §5.3 → M3/M4 | M3 V-3；M4 V-6；Fusion 示例回归 | 嵌套 dialog→popover、通知子层优先、触发器移除回安全位置（浏览器记录）〔记录〕+〔评审〕 | 已满足 | G-02（相关极端组合待核实，P4 终验） |
| WORK-F12/AC-1（Popover 指针+键盘；HoverCard 移动不意外关闭；Tooltip 悬停/焦点、Escape、不劫持焦点） | 本期 AC | §5.4 → M4 | M4 V-4/V-6 | `hover_grace.rs` grace 窗口；tooltip role、Escape 关闭不抢焦点（浏览器记录）〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F12/AC-2（边侧/对齐/偏移/碰撞/箭头；滚动/尺寸变化位置可用；主题方向层级一致；触控路径） | 本期 AC | §5.4 → M4 | M4 V-4/V-6 + Fusion anchor resize 回归 | `FloatingPosition`/arrow/碰撞代码与浏览器矩阵；触控点按入口/可见信息路径（记录）〔记录〕 | 已满足 | — |
| WORK-F14/AC-1（确定/不确定进度区分；数值与名称可读；Spinner 有说明；Skeleton 不重复朗读；不留失效占位） | 本期 AC | §5.5 → M6/M7 | M6 V-7；M7 V-10 | `primitives/progress.rs:29-30` aria-valuenow=原值、valuemax=max；Spinner label 取 locale（`i18n.rs:165,191`）；Skeleton inert/aria-hidden、结束恢复（feedback spec 27 项浏览器）〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F14/AC-2（Toast 重要度/队列/关闭/操作/时长可配置；滑动关闭；键盘进入；悬停/聚焦暂停；关键操作可追回；减少动效可理解） | 本期 AC | §5.5 → M6/M7 | M6 V-7；M7 V-10 | `rustify-dom/src/toast.rs` ToastConfig：policy Replace/Fifo、visible_limit=1、swipe_threshold=50、hotkey F8、方向/取消回调；暂停多因、持久 Details 可追回 Action（浏览器记录）〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F15/AC-3（内容/反馈/切换/浮层全部声明变体与尺寸可辨认；highContrast 仅限声明角色） | 本期 AC（F15 其余 AC 终验 P4） | §5.7/§6 → M4/M5/M6/M7 | M4/M5/M6 视觉矩阵；M7 12-AC 表 | 参数矩阵 spec（如 `content.spec.ts:246` 局部色板/高对比断言）+ 浅/深/390/768/桌面/RTL 截图集〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F17/AC-1（内容组件最小/长内容；Avatar 失败回退；Callout 非仅颜色；DataList 关联；Table 语义） | 本期 AC | §5.6 → M5/M7 | M5 V-5（26 用例）；M7 V-8 | `primitives/avatar.rs:90,99` delay_ms/回退代次；显式 status/alert 宣告；dl/dt/dd、caption/headers（content spec 13 项浏览器）〔静态〕+〔记录〕 | 已满足 | — |
| WORK-F17/AC-2（比例/分隔/滚动方向尺寸；滚动内容键盘可达；选择卡片用 WORK-F6 契约） | 本期 AC | §5.6 → M5/M7 | M5 V-5；M7 V-8 | `primitives/scroll_area.rs` 原生 scrollLeft/scrollTop、`Scrollbars::ALL`（Auto/Always/Hover/Scroll）；Viewport 原生键盘；cards 复用 checkbox/radio cards + FormData 回归〔静态〕+〔记录〕 | 已满足 | — |
| WORK-N1 增量（名称/角色/键盘/焦点/宣告及默认对比度；真人 AT 缺证单列，[PRD §467](../PRD-DOM-COMPONENTS.md)） | 本期增量；AC-1 已由 P2 终验、AC-2 P4、AC-3 P5 | §7 → V-6/V-8/V-10/V-11 | M4/M5/M6/M7 | ARIA/焦点/宣告断言遍布 panels/content/feedback spec；默认对比度代表表面实测过门槛（观察 5）；人工 AT 归 P5（missing 已单列）〔记录〕+〔静态〕 | 已满足 | — |
| WORK-N2 增量（同功能前后数据与新增能力分列、资源无残留，[PRD §479](../PRD-DOM-COMPONENTS.md)） | 本期增量；AC-2 终验 P5 | §8 → V-13 | M1 baseline / M7 current | `baseline.json`/`current.json` 同功能对照+新增能力分列；Tooltip 差异已解释（观察 3）；wasm 增长如实记录（观察 4）；任务/计时器/观察器 0 残留〔记录〕 | 已满足 | 观察 3/4 |
| WORK-N3 增量（隔离/输入安全/清理回归，[PRD §490](../PRD-DOM-COMPONENTS.md)） | 本期增量 | §9 → 各浏览器验证 | M2–M7 | 全部 spec 断言 console error/pageerror/requestfailed 为空；strict CSP 下运行〔记录〕 | 已满足 | — |
| WORK-N4 增量（现有消费与部署模式回归，[PRD §502](../PRD-DOM-COMPONENTS.md)） | 本期增量 | §9 → V-9/V-12/V-14 | M7 | SSG/hydrate：dom-hydrate 15/15（R3 同 HEAD 实跑）；public-panel-consumer 真实外部消费者；docs-verify features〔评审〕+〔记录〕 | 已满足 | F-02 已由 `f2bb1bb1` 修复并复审关闭 |
| WORK-N5 增量（能力声明与实际一致，[PRD §514](../PRD-DOM-COMPONENTS.md)） | 本期增量 | §9 → V-14 | M7 | catalog --check 无 drift（145 public entries/97 stable names/24 categories→M7 后 171 条目）；README/迁移指南/panels.md 同步〔记录〕+〔静态〕（文件存在） | 已满足 | — |
| WORK-N6 增量（失败不破坏有效状态的生命周期回归，[PRD §525](../PRD-DOM-COMPONENTS.md)） | 本期增量；AC-2 终验 P4 | §9 → V-6/V-11 | M3/M4/M7 | 锚点失效、abort、abort 后监听释放等浏览器断言通过；「卸载开放层唯一 Content」角落已实证关闭（G-02，2026-11-03 复现页 2 passed）〔记录〕＋〔本轮实测〕 | 已满足（本期增量） | G-02 已关闭；P4 终验照常 |
| WORK-N7 增量（两模式/长内容/窄视口/缩放/减少动效回归，[PRD §536](../PRD-DOM-COMPONENTS.md)） | 本期增量；AC-2 P4、AC-3 P5 | §9 → V-6/V-8/V-10/V-11 | M4/M5/M6/M7 | 浅/深、390/768/桌面、RTL、reduced motion、200% 密度模拟截图与断言；真实浏览器 UI 缩放/OS 级归 P5（与 PRD 期次一致）〔记录〕 | 已满足（本期增量） | — |
| WORK-N8（来源与可重现输入，[PRD §548](../PRD-DOM-COMPONENTS.md)） | 本期增量 | §9 → sources verify | M1 | Radix Themes 3.3.0 63 文件 + Primitives 1.8.1 聚合 19 文件入锁、0 modified〔记录〕 | 已满足 | — |
| WORK-F1/AC-2 前置（raw 消费者不依赖皮肤，[PRD §194 行区](../PRD-DOM-COMPONENTS.md)） | 前置复用；AC-2 整条 P4 | §9 | M7 | docs-verify features：raw 皮肤缺席、primitives.css 字节相同〔记录〕 | 已满足（本期前置） | — |
| WORK-F4 / WORK-F5 / WORK-F6（受控值/表单/选择契约回归复用） | 前置与回归 | §9 | M2/M5/M7 | ssr_controls 13/13、primitives 59 通过、FormData 回归〔记录〕 | 已满足 | — |
| WORK-F19（真实组件/状态/主题来源检查，[PRD §405](../PRD-DOM-COMPONENTS.md)） | 本期作参考输入；终验后置 | §5.7 → M1 | M1 | 入仓 props 与 base-tab-list 逐项对照，未虚构枚举〔记录〕 | 已满足（本期用途） | 终验按 PRD 分期 |
| WORK-F23/AC-1（catalog 区分已交付/计划，P1 AC） | 本期跟随文档同步 | §9 → V-14 | M7 | catalog 状态与实际交付一致（check 无 drift）〔记录〕 | 已满足 | — |

无「从未进入计划」的 P3 需求：PRD §7.2 P3 列（12 AC）与计划 §7 台账逐行对应；计划另主动纳入质量增量与前置行。

## 4. Triage 与决策账本

**本轮无待决问题（C=0），无新增决策。** 引用的既有有效决定：

| 决策 | 关联 | 用户选择与理由 | 确认日期与来源 | 新验收边界 | 去向 |
| --- | --- | --- | --- | --- | --- |
| 既有 D（无编号，沿用） | WORK-F10/AC-1、F-01/F-02 | Collapsible/Accordion 关闭内容默认卸载、aria-controls 仅 open 输出——复刻 Radix，与 vendor 口径一致 | 2026-11-01，用户在 R1 修复期裁定；见 [R2 复审表](DOM-COMPONENTS-P3.code-review.r2.md) | 关闭态无 aria-controls/无挂载面板为正确行为；`force_mount` 保留 id 关联 | 已实施（`3d078e29` 修复、`f2bb1bb1` 断言同步，R3 关闭） |

## 5. 待核实项与审查基线

| 待核实项 | 影响的需求/结论 | 补证方式 | 是否阻塞 |
| --- | --- | --- | --- |
| G-02 死遮罩角落（打开期间卸载唯一 Content）——已于 2026-11-03 提前实证关闭 | WORK-N6/AC-2（P4 终验）；不影响本期 12 AC 结论 | 已执行：隔离复现页两条卸载路径 2 passed，断言与 R3 关闭判据一致（见 §2 G-02 证据链）；复现装置用后清理 | 已关闭 |
| 环境备注：docs-verify 末步缺 `tomllib` | 不影响本核查结论（hydration 产物已另证有效） | 升级本机 Python ≥3.11 或装 tomli 后重跑 | 否 |

**审查基线**：PRD v0.2（2026-10-26，未变）；计划 v0.2 快照与 M1–M7 记录、R1–R3 报告均为当前工作树版本；代码 HEAD `c658b831` dirty（仅台账与本轮报告，无源码差异）。本次静态核实清单：`primitives/{tabs,collapsible,alert_dialog,popover,avatar,progress,scroll_area}.rs`、`primitives/floating.rs:518/425`、`rustify-dom/src/toast.rs`（ToastConfig 默认）、`i18n.rs`、`tab_nav.rs`、`catalog.rs`（aria-modal 口径）、CSS 产物（panels/content/feedback.css）、`tests/{panels,content,feedback}_ssg.rs`、`tests/dom-only/*.spec.ts`、`.github/workflows/verify.yml`（新门已接线）、`docs/validation/dom-components-p3/`（含 baseline/current.json 与 m7-studio-form-contrast/matrix.json）、`docs/panels.md`、`docs/dom-components-migration.md`、`tests/dom-only/public-panel-consumer/`。历史运行结果均引自原记录并标注，未计为本轮实跑。G-02 实证（2026-11-03）基于同 HEAD 源码的临时复现装置：`target/g02-repro/consumer`（独立 workspace、新锁文件、仅新增 g02 模块与页面）经 `build-dom` 产出 `target/g02-repro/site`，由 playwright docs 配置 `DOM_ONLY_DIR` 以 `--project=dom-only` 就绪服务后执行；装置与产物按临时资源清理，结果如上。

## 6. Follow-up 与 gaps plan

| G-ID / 需求 | 确定的增量工作 | 验收条件 | 依赖/阻塞 | gaps plan 任务或去向 |
| --- | --- | --- | --- | --- |
| G-01 / WORK-N1 | 无（变更保留，已文档化） | — | — | P4 WORK-N1/AC-2 终验按 scope-local 口径复核（PRD 分期既定） |
| G-02 / WORK-N6/AC-2 | 无（疑点已于 2026-11-03 提前实证关闭，见 §2） | 已满足：两条卸载路径 close 恰一次、遮罩不可见/不可拦截、Escape 无二次关闭 | — | 已关闭；P4 WORK-N6/AC-2 终验照常覆盖其余生命周期场景 |

**无需 gaps plan。** A=0（无明确遗漏）、C=0（无待决）、待核实 0（G-02 已提前实证关闭，非缺陷，无需修复计划）；无剩余已确定增量。已核对 `docs/plan/`：不存在 DOM-COMPONENTS-P3-gaps 计划；DOM-COMPONENTS-P1-gaps 属 P1 期，与本报告无重叠。

## 交付前核对自检

- PRD §7.2 P3 列 12 条 AC 均有覆盖行，质量增量与前置行单列；无静默漏行（§3 末尾说明）。
- 每项差异（G-01/G-02）有 PRD 定位、实际行为、证据链、影响、分类依据与处置；待核实未伪装成已解决。
- 无 C 类，无需 Triage；既有用户决定原样引用未改写。
- Follow-up 均有去向（G-01 P4 终验；G-02 已关闭），豁免/保留有理由；无需 gaps plan 的理由已写明。
- 本报告已回读；台账按 v3 协议登记独立评审对象。
