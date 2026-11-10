# DOM-COMPONENTS-P5-gaps.md · 开发计划评审报告

> 评审日期：2026-11-09（Asia/Shanghai）· 模式：完整首次评审（非修复效果复审）。
> 四视角独立 reviewer（facts / architecture / schedule / delivery，subagent 并行）+ 主 agent 交叉复核；只读，未运行测试/构建/浏览器。

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P5-gaps · 迁移事实源与扩展验收补缺](DOM-COMPONENTS-P5-gaps.md) | sha256 `78ca18c20d87f71a490c5b0e3cbfc5f50cb980528a75c47468fc6299ea791b8c`（未提交新文件） | 全文（账本 15 行、M1/M2、V-1—V-5） |

关联材料（背景，非修订目标）：[P5 PRD 差异报告](DOM-COMPONENTS-P5.prd-dev-gaps.md)（`766f1163`）、[DOM-COMPONENTS PRD v0.2](../PRD-DOM-COMPONENTS.md)（`b857222e`）、[原 P5 计划](DOM-COMPONENTS-P5.md)。代码基线：main `f12d5220b014bc7c7baa9679f38afb2388ac749b`，产品代码与测试无未提交改动；基线对照 `11b69129`。

---

## 1. 确认问题

1. **[P2] F-01 · V-1 测试过滤器只覆盖单个既有测试函数，新增断言落点未钉死，可能静默漏跑**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:384`（V-1 命令）；关联 :313（§3「扩展现有 inventory 测试」）、:328（§3.2「扩展 §3 的 inventory 断言」）、:344（§5.1 红-绿顺序）。关联 G-01/G-07/G-08。
   - 问题：V-1 的过滤器精确到 `catalog::inventory_tests::legacy_baseline_keys_resolve_exactly_once` 这一个测试函数。§3/§3.2 只说「扩展现有 inventory 测试/断言」；若实施者按常见做法在 `inventory_tests` 模块内**新增**测试函数承载 10 参数/方法身份/chart 断言，该命令不运行它们，且本计划无任何其他命令运行 rustify-components `--lib`（V-3 只有 fmt/clippy/`-p theme-studio --bins`），窄回归断言将绿着漏跑。§5.1 的「先确认当前失败」是唯一绊线——若断言未被执行，红步会意外变绿，但这依赖实施者把「绿」识别为异常。
   - 证据：`crates/rustify-components/src/catalog.rs:1728`（单一 `#[test] fn legacy_baseline_keys_resolve_exactly_once`，模块内另有 `frozen_reference_inventories…` 等函数，:1723 起 `#[cfg(test)] mod inventory_tests` 无 `rustify_repo_tests` 门——RUSTFLAGS 前缀对该过滤器是惰性但无害的既仓用法）。
   - 修订：在 §3 或 V-1 写明新断言**并入既有 `legacy_baseline_keys_resolve_exactly_once` 函数体**；或将过滤器放宽为 `catalog::inventory_tests`（模块级，覆盖新增函数），二选一，一行改动。
   - 复核：改稿后确认 §3/§3.2 的断言落点表述与 V-1 过滤器一致；实现期验证由 V-1 既有命令承担。
   - 置信度：高（过滤器行为与测试结构直接核实）；阻塞：否。

2. **[P3] F-02 · LEGACY_ENTRIES 精确名孤儿行无检测，笔误将静默生成公开目录死行**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:313`、:328（断言范围）；对照 `catalog.rs:1746-1764`。
   - 问题：inventory 测试第三循环只校验**通配**名（`strip_suffix('*')`）至少命中一个基线键，不检测精确 name 不在基线中的孤儿条目。M1 将新增约 20+ 精确行（10 个 prop 键、接收者限定方法键、5 个 semantic 键）；条目侧键名笔误（基线侧笔误会被「每键恰一解析」抓住，条目侧不会）会静默渲染进 `docs/components.md` 的 Legacy 表——正是本计划要消除的「目录声明与真实结果不一致」（WORK-N5/AC-2）同类盲点，且本期验收行与 V-3 人工回读均不覆盖全表。
   - 证据：`catalog.rs:1746-1764`（仅 `strip_suffix('*')` 分支被校验）；`catalog.rs:1709-1717`（`for entry in LEGACY_ENTRIES` 无条件写行）。
   - 修订：在 F-01 钉死的既有测试函数体加一行断言：每个非通配 name 必须存在于基线键集合。
   - 复核：并入 V-1 既有命令，无需新增检查；实现期由红-绿自然覆盖（新断言在补齐数据前对旧条目也应通过，属纯增量护栏）。
   - 置信度：高（测试代码直接核实）；阻塞：否。

3. **[P3] F-03 · 再生成目录的 provenance 头部句将失真，计划未列该改动**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:284`（§1.2 将 catalog.rs 改动限定为「inventory_tests 只加…最小回归断言」）；§5.4 与 V-3（:386）回读清单不含此句。
   - 问题：生成器头部（`catalog.rs:1705`）声明冻结基线「extracted mechanically from the pre-P5 commit 11b69129」。M1 手工补 10 个 prop 键、接收者限定方法键、5 个 semantic 键并删伪键后，该来源声明不再准确；再生成后的公开 `docs/components.md` 会携带失真句——与 G-04 同类的目录事实缺陷。
   - 证据：`catalog.rs:1705`（头部句原文，随 `catalogue_markdown()` 进入公开产物）。
   - 修订：§1.2 为 catalog.rs 增列同步修订该句（如「mechanically extracted, then hand-completed for theme props, receiver-qualified methods and chart semantics」），并把此句加入 V-3 人工回读清单。
   - 复核：并入 V-3 既有「人工回读」一项，不新增检查。
   - 置信度：高；阻塞：否。

4. **[P3] F-04 · V-3/§9.5 静态检查集合与 §1.2 改动面错配（theme-studio 无 clippy；css --check 被整体排除）**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:386`（V-3 clippy 仅 `-p rustify-components`）、:406（§9.5「不运行 `xtask css`」）；对照 §1.2（:285 改 `examples/theme-studio/src/gallery_extensions.rs`）。
   - 问题：① M1 修改 theme-studio 样本 Rust 代码，V-3 的 fmt 覆盖格式、`-p theme-studio --bins` 覆盖编译，但 lint 级问题（`-D warnings` 项）只在仓级 CI 才暴露，晚于本计划全部退出条件。② §9.5 断言「无 Tailwind class…不运行 xtask css」——但 `legacy.rs` 在 css 候选扫描树内（`xtask/src/css.rs:25-31`），M1 新增的 destination/reason 自由文本有 P4 先例（目录 prose 被扫为 utility，见 P4.records/M7.md:48）；计划点名的 token 已全部在 `candidates.json`（本次核实），实际漂移概率低，但「不运行」的整体排除断言过强。
   - 证据：`xtask/src/css.rs:25-31`（SOURCE_TREES 含 `crates/rustify-components/src`）；根 `Cargo.toml:20`（theme-studio 为 workspace 成员，仓级 clippy 含 examples）。
   - 修订：V-3 clippy 追加 `-p theme-studio --all-targets`（或 `-bins`）；§9.5 措辞改为「不预期产生新 class 候选；V-3 末尾跑一次 `mbx xtask css --check` 复核无漂移」（秒级命令）。
   - 复核：并入 V-3 既有命令集，不新增检查组。
   - 置信度：高（范围错配与扫描树直接核实；css 实际触发概率低）。
   - 阻塞：否。

5. **[P3] F-05 · V-4「与 V-2 共用同一次浏览器场景／执行时同步完成」不可按字面执行**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:387`（V-4 命令/入口列与执行时点列）。
   - 问题：V-2 是 Playwright 自动化运行（webServer 起停、批式执行），Agent 角色模拟无法「在同一次运行中」操作；仓规禁止同时启动两个 Playwright 运行，按字面并行第二实例违规。且 V-4 的需求列含 U-02（GPU loss 期间保存/导出），「只掌握公开文档的组件消费者」无法从公开 UI 触发 `WEBGL_lose_context` 注入——该观察只能取自 V-2 场景结果，计划未写明。
   - 证据：`tests/browser/theme-acceptance.spec.ts:409-436`（GPU loss 注入在自动化用例内）；AGENTS.md「浏览器测试」节「不同时启动两个 Playwright 运行」。
   - 修订：V-4 入口列改为：由 Agent 经可用浏览器通道在**同一 release、同一 §6 场景**上串行执行（自行 `xtask serve`，不与 V-2 并行）；U-02 的 GPU loss 观察取自 V-2 场景执行结果，不由消费者角色自行触发。
   - 复核：改稿后核对 V-2/V-4 两行表述不再含「同一次/同步」歧义；验收本身仍由既有 V-2/V-4 承担。
   - 置信度：中高；阻塞：否。

6. **[P3] F-06 · §6 第 1 段「DataTable …打开」在样本上无可观察结果，§1.2 未授权补输出**
   - 定位：`docs/plan/DOM-COMPONENTS-P5-gaps.md:356`（§6 走查第 1 段）；对照 §1.2（:285 对 `gallery_extensions.rs` 仅授权「局部 PanelTabs controlled 行为」）。关联 U-01/V-2。
   - 问题：`DataTableSample` 的 `on_activate=|_| {}`（`gallery_extensions.rs:70`）——Enter/F2 激活后页面无任何可观察变化。V-2 要求「从 Studio 公开 UI 观察状态…输出」，实施者面对「打开」只能跳过（无声缩水）、断言无变化（无意义）、或给样本接输出（超出 §1.2 声明的改动面）。
   - 证据：`gallery_extensions.rs:70`（空闭包）；同文件 :157/:263 有现成 `output` 模式可仿。
   - 修订：二选一，均一行级——在 §1.2/§6 授权 DataTableSample 接一个最小可观察输出（仿 CommandSample 的 `output` 行）；或将 §6 第 1 段收窄为「键盘移动/选择与局部滚轮」，激活契约注明由 `data_table/keys.rs` 既有单测覆盖。
   - 复核：改稿后 §6 断言与样本可观察面对齐；实现期由 V-2 既有场景承担。
   - 置信度：高（空闭包与 §1.2 授权范围直接核实）；阻塞：否。

---

## 2. 待核实 / 待决策

| 项 | 缺什么证据 | 影响哪个决定/任务 | 谁可提供 | 最晚确认点 |
| --- | --- | --- | --- | --- |
| 全量方法键重提后的逐项 fate | §3.1「按接收者核对全部方法」的完整结果（本次抽查 8 组样本：10 参数、两组折叠键、ThemeDocument×2、ThemeFont×2、伪键、chart×5 全部与基线/当前代码一致，其余键未逐一预核） | M1 的 G-07 交付 | 实施者（V-1 断言 + V-3 回读兜底） | M1 退出前 |
| M1 新增 reason/destination 文本是否零 css 候选漂移 | 一次 `mbx xtask css --check` 运行 | F-04 修订的必要性 | 实施者 | M1 退出前（若采纳 F-04 则自动覆盖） |

无待决策项（无产品取舍须用户拍板；§3.1 末段「需产品取舍才记阻塞」的路径已定义）。

## 3. 覆盖摘要

- **四维覆盖**：facts / architecture / schedule / delivery 各一名独立 reviewer（delegate subagent，模型继承，并行启动 3 + 复用槽位补第 4），主 agent 交叉复核全部候选问题并独立重验关键事实（基线 `11b69129` 与当前的 10 参数签名、`ThemeBoundary.patch` 类型变化、`ThemeOverrides::values` 同名陷阱、`pub const fn face/slot`、伪键/折叠键行号、`extensions_ssg.rs:240` 负向断言、`DataTableSample on_activate`、css 扫描树与 candidates.json、workspace 成员）。
- **facts 核实结论**：计划 §1.1 六条「已核实·缺口」全部与仓库现状吻合；§3/§3.1/§3.2 的 fate 判定（9 Kept + 1 Changed、两组拆分、两个 Replaced、const 方法、chart 回退）逐项与基线/当前代码一致；账本—设计—里程碑—检查无互斥；「零改动」清单与所需修复无冲突；V-1/V-2/V-3 命令、测试名、Playwright 过滤器（`-g 'extension samples'`、`-g 'DOM edits remain usable'`）全部真实可用。
- **脚本**：`check_paths.sh` 10 token、1 MISSING（`DOM-COMPONENTS-P5-gaps.records/M1.md`——§1.2 明示 ★ 按需创建的占位，预期）；`check_progress.sh` 0 error / 1 warn（工作树 18 个未提交改动 vs 计划未动——既存 SDLC/R1/R2 材料，§11 风险表已登记处置，计划 0/2 未开始，无回写缺失）。
- **浏览器**：未启动（计划未实施、无新页面；Studio 现状仅源码静态核对）。UI 证据等级：**仅计划/流程描述 + 源码静态**——§6 交互方案与既有组件行为（键处理、指针、close 控件、ARIA 现状）经静态确认成立，行为本身待实施后 V-2/V-4 浏览器验收。
- **非问题备注**：`extensions_ssg.rs:240` 现有 `assert!(!zone.contains("aria-invalid"))` 把当前缺陷钉成断言，红-绿循环（§5.1）会机械暴露并翻转，`@delivery_first` 的 `existing_local_test_fails_after_feature_change` 条款覆盖该处置，不构成 finding。快照「下一步」的「§3/§6」指代略宽（样本修正落点为 §5.3），无实质恢复风险，不计 finding。

## 4. 映射与实施编排

- **需求映射**：账本 15 行（G-01—G-08、U-01/U-02、C-1/C-3/C-6/C-7/C-8）→ 设计落点 → M1/M2 → V-1—V-5 双向追踪**无断链**；差异报告 §6 Follow-up 的 9 项验收条件均被本计划检查同等或更强承接（G-03 另有 V-4；G-05 明确不重跑 C-7 豁免项；U-02 的 fatal 用例独立不动）。排除/豁免（G-06、C-6/C-7/C-8、VoiceOver/IME/触控 missing）均有明确去向，不冒充通过。
- **依赖边**：红-绿断言（§5.1）→ §3/§5 修复 → V-1/V-3（M1 退出）→ V-2/V-4/V-5（M2 退出）→ 最终提交与回填。M2 前置 M1 是**真依赖**（V-2 场景断言消费 M1 修复后的 DropZone/PanelTabs 终态），非过度串行；M1 内 PanelTabs 修复的浏览器验证置于 M2 属同一开发批次的单次浏览器套件，避免复跑，编排合理（D1 明示）。`build-web --release` 先于 V-2/V-4，符合「serve 不重建」仓规。
- **并行**：单实现者、共享文件串行（M1 行明确「不派并行 agent」）；无安全并行空间的需求，无需调整。M2 内性能索引整理与 V-2 无依赖可穿插。
- **提交点**：仅 M2 末尾一次最终提交，与 `@commit_protocol`（分别提交为可选、全部完成后必须提交一次）一致；M1 行届时记「待提交」。工作树既存台账改动的隔离已由 §1.2/§10/§11 逐路径暂存约束覆盖。
- **修订涉及的编排变化**：仅 F-01/F-04 各改一行命令、F-05/F-06 各改一处措辞、F-02/F-03 各加一句——不动摇 M1/M2 结构与验收分配。

## 5. 总体结论与修订顺序

**是否需要修改：需要**（存在 P2 确认问题 F-01；F-02—F-06 为 P3 小修，可一并采纳）。

- **建议状态：Ready**（上述修订落地后）。理由：全部差异均有 PRD/用户决定来源和经核实的代码落点；技术路线（接收者限定键 + 通配保留 + 精确行优先）与既有解析机制兼容、边界闭合；无未决产品取舍、外部依赖、数据/安全/发布前置；验证均可由 Agent 用本仓既有工具完成。
- **与自报状态差异**：计划自报 Ready；判定成立的前提是先落 F-01 的一行修订（V-1 过滤器与断言落点对齐），其余 P3 可在改稿时顺带采纳或由实施者就地处理。无 Blocked 项；解除「需要修改」的可观察条件 = 六条修订写入计划文本并复核。
- **修订顺序**：① F-01（检查设计前提，V-1 命令/§3 断言落点）→ ② F-04（V-3/§9.5 静态检查集合与改动面对齐）→ ③ F-03（§1.2/V-3 增 provenance 头部句修订）→ ④ F-02（§3 断言集加孤儿名护栏一句）→ ⑤ F-05/F-06（V-4 执行方式与 §6 可观察面措辞）。均为行级改动，不触发路线重审。


## 修订处置与复核

- 复核日期：2026-11-10（Asia/Shanghai）；首次 apply，先前无处置记录。本节是文档修订复核，不是新一轮独立评审；以上原问题、原结论与所审基线保持不变。
- 来源：本报告修订前完整内容 sha256 `4327353d7ff3f8548b5babfbc1b14d37aec4873d9dbe78581d1364969abefa8a`；评审对象 ID `6b1d35bf-077b-440d-8d7b-bfef8fdeeff4`，问题身份为该评审下的 F-01—F-06。
- 目标：[当前计划](DOM-COMPONENTS-P5-gaps.md)；修前 sha256 `78ca18c20d87f71a490c5b0e3cbfc5f50cb980528a75c47468fc6299ea791b8c`，与原审一致；修后 sha256 `7a2007dcff8c4eb4f88201e1a2394f38232ef53d096a7754fe6c1275a4136453`。原文身份和路径未变，无既有计划差异需合并。
- 代码基线：main `f12d5220b014bc7c7baa9679f38afb2388ac749b` / dirty；产品代码与测试无未提交改动。既存 SDLC、R1/R2、差异报告及未跟踪计划/报告保留；本次仅修订目标计划、追加本节并按仓内协议回写关联状态，不提交或推送。
- 范围：F-01—F-06 全部，以及 §2 两项实施期待核实的承接安排；不实施 M1/M2，不修改关联 PRD、原 P5 计划、代码或历史验收记录。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 本轮复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。[catalog.rs](../../crates/rustify-components/src/catalog.rs) 的 `inventory_tests` 包含多个测试，V-1 原过滤器仅选中 `legacy_baseline_keys_resolve_exactly_once`，原 §3 未限定新增断言落点。 | 已修订并复核 | §1.2、§3、V-1 明定 §3—§3.2 全部新增目录断言并入该既有函数体；保留单函数过滤器与红绿顺序。 | 已解决（文档层）。不放宽到模块级，避免增加无关回归；实现后由 V-1 确认新断言实际执行。 |
| F-02 | 成立。同一函数中 `LEGACY_ENTRIES` 条目侧只校验通配名前缀，非通配名直接跳过；`catalogue_markdown()` 无条件输出每行，新增精确名笔误可成为公开死行。 | 已修订并复核 | §3、V-1 增加“每个非通配精确名必须存在于基线键集合”，并入 F-01 的同一函数，不增检查组。 | 已解决（文档层）。条目与基线双向约束明确；不声称该断言已实现或已运行。 |
| F-03 | 成立。[catalog.rs](../../crates/rustify-components/src/catalog.rs) 的 `catalogue_markdown()` 仍把来源仅写为从 `11b69129` 机械提取，无法解释计划要求的手工核对补全。 | 已修订并复核 | §1.2、§3 明定生成器同步说明机械提取后按旧签名补全主题参数、接收者限定方法和 chart 语义；V-3 人工回读该句及生成产物，M1 同步承接。 | 已解决（文档层）。保留原始基线归属，不把补全后的数据描述为未经修正的机械输出。 |
| F-04 | 成立。[根 Cargo.toml](../../Cargo.toml) 含 theme-studio；原 V-3 只对 components 跑 Clippy。[css.rs](../../xtask/src/css.rs) 的 `SOURCE_TREES` 含 components/src；[P4 M7 的 V-R2](DOM-COMPONENTS-P4.records/M7.md#评审修复) 留有目录文字 `transition` 被扫成 utility 的真实先例。 | 已修订并复核 | V-3 Clippy 增加 `-p theme-studio --all-targets`，保留 host 检查，末尾加一次 `mbx xtask css --check`；§1.2/§9.5 去掉 CSS 检查整体排除，叙述误入候选时优先改写文字。 | 已解决（文档层）。检查与改动面一致；未运行未来的 Clippy/CSS 命令，不预判新增文本零漂移。 |
| F-05 | 成立。[Playwright 配置](../../playwright.config.ts) 为自动化启动服务且 `reuseExistingServer=false`；[GPU loss 用例](../../tests/browser/theme-acceptance.spec.ts) 通过内部故障注入触发，消费者只凭 UI 无此入口。 | 已修订并复核 | V-4 改为 V-2 结束并释放服务后，以同一 release、既有 4181 端口和可用浏览器通道串行走查；结束停止本次 serve。U-02 的注入及故障期间观察明确引用 V-2 结果，不计为角色独立实测。 | 已解决（文档层）。V-2/V-4 不再要求“同一次/同步”操作；未追加并行浏览器或故障入口。 |
| F-06 | 成立。[gallery_extensions.rs](../../examples/theme-studio/src/gallery_extensions.rs) 的 `DataTableSample` 使用空 `on_activate`，同文件 `CommandSample` 已有回调更新状态并渲染 `output` 的模式；原 §6 的“打开”无可观察结果。 | 已修订并复核 | §1.2 授权样本补最小激活行输出；§6 明定 Enter/F2 后观察当前行，M1 列入该输出，V-2 沿现有场景验收；同步 §6 的文案边界。 | 已解决（文档层）。保留 U-01 原激活意图，不采用删除激活验收的备选；不新增编辑器、业务动作或组件算法。 |
| §2：全量方法键重提后的逐项 fate | 这是 M1 的交付事实，修订文档不能代替按接收者逐项比对旧签名和当前去向。 | 待核实（实施期） | §3.1 保留全量方法核对、真实迁移去向及需产品取舍时阻塞的处理；V-1/V-3、M1 退出承接。 | 本次未执行重提，不能记已验证；不阻塞已明确的技术路线。 |
| §2：新增 reason/destination 的 CSS 候选漂移 | 新文本尚未实施，当前扫描树只能证明检查需要，不能证明未来结果。 | 待核实（实施期） | F-04 已将一次 CSS 检查纳入 V-3，§9.5 明确失败时的处理。 | M1 退出前执行；文档安排已闭合，零漂移结论仍待实际运行。 |

### 决策与未完项

无新增产品或架构取舍需用户决定，未采纳意见 0。F-01 选择固定既有函数而非放宽过滤器；F-06 选择补样本输出而非删去激活验收，均维持原目标、复用既有模式。G-06 与 C-6/C-7/C-8、人工 missing 的既有决定不变。

两项原待核实保留为 M1 实施/退出工作；全部代码、构建、V-1—V-5 及真实使用验收仍未执行。关联 PRD、差异报告与原 P5 记录无需为了本次修订改变承诺或历史结果，保持只读；未来补全事实源、样本输出与 CSS 验证只能由实施记录证明。

### 修订后结论与验证

- F-01—F-06：**6 项已修订并复核，文档层面已解决**；无需修改/不采纳/文档层未完均为 0，待决策 0。原 §2 的两项实施期待核实不计作已验证。
- 当前计划判定：**Ready**。设计、改动面和检查已对齐，无关键开工阻塞；实施仍为 **0/2，尚未开始**。此判断是本次 apply 的修订复核，不替换原报告“需要修改”的历史结论，不代表完整独立复审、产品实现或上线通过。
- `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P5-gaps.md /Users/rockie/Documents/gh-rockie/rustify-ui`：10 个路径 token，唯一 MISSING 为 §1.2 明示 ★ 按需创建的 `DOM-COMPONENTS-P5-gaps.records/M1.md`，不是缺失复用路径；同列 M2 记录也尚未创建，符合未开始状态。
- `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P5-gaps.md /Users/rockie/Documents/gh-rockie/rustify-ui`：0 error / 1 warn，0/2、2 个里程碑、完成记录 0 行。脚本观察到 21 个未提交路径，但未跟踪计划不在 `git ls-files`，产生“计划没被改过”的启发式提示；已对照修前副本确认计划已修订，工作树无本计划实施改动，无进度漏回写。
- `git diff --check` 无空白错误；另对未跟踪计划与修前副本执行 `git diff --no-index --check` 无诊断（退出 1 表示内容差异），并人工审查逐行差异。实际本地文档链接已核对；实施协议中的 `FEATURE.records/M1.md` 是格式示例，不是证据链接。
- 人工回读 §1.2、§3—§3.2、§6、V-1/V-3/V-4、§9.5、M1 与恢复快照：断言落点、来源说明、静态检查、串行浏览器步骤、可观察输出及 0/2 一致。
- **未在浏览器中验证**：本次只改文档，无新产品产物；未运行计划中的 Rust 测试、Clippy、构建、CSS 或 Playwright，不将历史测试或静态核对当成本次行为验收。

下一步仅作建议，由用户发起：`/goal 按 docs/plan/DOM-COMPONENTS-P5-gaps.md 开发`。
