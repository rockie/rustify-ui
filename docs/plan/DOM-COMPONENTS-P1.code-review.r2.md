# DOM-COMPONENTS-P1 · 代码复审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [DOM-COMPONENTS-P1 · Colors 与统一主题基础](DOM-COMPONENTS-P1.md)，SHA-256 `10ceb5407176e805f0cc9fb03668acad3bf6c6a71e1d5f38b145718289ad273e` | `3193791720edbb1c933bfa852773f0b4710a7b15..2304903c04ef8b634728b45173fb95123e2f928e`（代码只在修复提交 `135f1d6f5c1c480aa54bf592cdcc41c5cca406c5`）；另核对当前工作树，没有未提交的产品代码 | [代码评审 R1](DOM-COMPONENTS-P1.code-review.md)，SHA-256 `56a0d58e632f53de65156bfae5cc9e88ce089a546db65f480431608f5e96d2dd` |

## 复审记录

- 方式：修复效果复审，不是完整重审。逐项核对 R1 的 F-01、F-02、F-03（R1 没有待核实项），并检查修复差异是否新引入或激活了问题。
- 上轮基线：R1 审查了 `11b6912919a2a34c372aac4d6b7d51052820600f..3193791720edbb1c933bfa852773f0b4710a7b15`。R1 报告的 SHA 与 M4/M5 处置记录引用的一致，说明 R1 报告之后没有改动。
- 修复差异：`3193791720edbb1c933bfa852773f0b4710a7b15..2304903c04ef8b634728b45173fb95123e2f928e`。共两个提交：`135f1d6f` 是计划完成记录里登记为「修复」的评审修复提交；`2304903c` 只改了计划文件，是进度回写。代码与测试的净差异只有 8 个文件，SHA-256 `23c334479078cfacfa628bfab2e33e3bc6db80ab0bb30cc6ca23b2394edcb4cd`：`crates/rustify-components/{src/layout.rs,css/candidates.json}`、`examples/component-catalog/src/layout_fixture.rs`、`examples/theme-studio/src/{controller,state,write_queue}.rs`、`tests/browser/{dom-layout,theme-v2-storage}.spec.ts`。
- 已有处置记录：[M4 评审修复 R1](DOM-COMPONENTS-P1.records/M4.md#评审修复-r1)、[M5 评审修复 R1](DOM-COMPONENTS-P1.records/M5.md#评审修复-r1)、[M7 受影响终验](DOM-COMPONENTS-P1.records/M7.md#评审修复-r1-的受影响终验)。这些记录作为线索阅读，下面每项结论都对照当前代码和本轮实际执行的结果独立核对。

## 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1](DOM-COMPONENTS-P1.code-review.md) / F-01 | [P2] 单边 `InsetSide` 激活了三条边；R1 认为每个单边只应激活自身那一条边 | `135f1d6f` 把 Top/Bottom/Left/Right 改成单边映射，补了 SSR 矩阵和响应式浏览器用例 | **不再适用：R1 原判不成立（误报）** | 不是契约变了，而是 R1 读错了契约。M1 冻结的[字段矩阵](../validation/dom-components-p1/field-matrix.md)第 71 行写明，Inset 来源是 Radix Themes 的 `inset.props.tsx`「与同名实现/CSS」。参考版 3.3.0 的 `inset.css` 中，`side-top` 抵消上、右、左三边并给上方两角圆角，其余单边同理。修复前 `3193791:crates/rustify-components/src/layout.rs:681-684` 与参考完全一致。R1 写的「计划 §5.6 要求 `side` 决定消除最近容器 inset 的方向」在计划第 481 行原文中不存在（全文检索「side 决定」无结果），是把计划没说的要求归给了计划。这次修复反而造成了偏离，记为本轮 F-04 | 按 F-04 恢复参考语义；在作者侧记录中更正 R1 F-01「成立」的认定，见 §2 的 D-2 |
| [R1](DOM-COMPONENTS-P1.code-review.md) / F-02 | [P2] 排队的 autosave 在 ResolveContext 变化后仍复用旧 proof，可能写入当前上下文和 reload 都会拒绝的 v2 | `135f1d6f`：`Snapshot.context` 绑定完整 `ContextKey`；出队时 key 不一致就走 `persistence::serialize` 做完整校验 | **已解决** | [write_queue.rs:395-400](../../examples/theme-studio/src/write_queue.rs#L395-L400) 只在 `ContextKey::matches` 为真时复用 proof。[state.rs:39-73](../../examples/theme-studio/src/state.rs#L39-L73) 的 key 覆盖 revision、rem、reduce motion、字体表、回退字体和解析策略，即 `ResolveContext` 的全部输入。proof 和 key 在 [controller.rs:562-568](../../examples/theme-studio/src/controller.rs#L562-L568) 由同一个 context 生成。不一致时调用的 [persistence.rs:186-207](../../examples/theme-studio/src/persistence.rs#L186-L207) 会重新解析 draft 和全部命名主题的两种模式。失败后照旧走 `fail` → 暂停、保留、显式重试；Autosave 的命名库缓存本身以 context 为键（[persistence.rs:300-311](../../examples/theme-studio/src/persistence.rs#L300-L311)），不会把旧 context 的结果带进来。本轮实际运行：Studio 68/68，包含 16px→32px 拒绝并保留、五类 context 输入逐项重验、context 不变时暖库 0 次解析；真实 Chrome 中锁等待期间切换 rem 的用例通过 | 无 |
| [R1](DOM-COMPONENTS-P1.code-review.md) / F-03 | [P3] 有效 v2 重载后，未变化的当前草稿被误报为未确认保存 | `135f1d6f`：`Writer::new` 仅在 baseline 存在且 `Permission::Ready` 时，用 epoch 0 和恢复出的 draft/preferences 初始化 ack | **已解决** | [write_queue.rs:158-159](../../examples/theme-studio/src/write_queue.rs#L158-L159)。`Writer::new` 唯一的调用点 [controller.rs:173](../../examples/theme-studio/src/controller.rs#L173) 的 editor 由同一个 `envelope.draft`/`preferences.mode` 恢复，`editor_epoch` 初值为 0，所以 [controller.rs:611](../../examples/theme-studio/src/controller.rs#L611) 只在没编辑过时判为已保存。新库（无 baseline）、坏 v2（Protected）、legacy 待确认或已取消都保持 `None`。`reset_fixture` 会推进 epoch，不会被误判为已保存。本轮实际运行：模型测试和真实 Chrome 的有效 reload、legacy 取消、坏 v2 三条路径通过 | 无 |

## 1. 确认问题（本轮新增）

### F-04 · [P2] 恢复 Inset 单边方向的 Radix 三边语义

- 定位：[crates/rustify-components/src/layout.rs:681-684](../../crates/rustify-components/src/layout.rs#L681-L684)，target `2304903c04ef8b634728b45173fb95123e2f928e`（由 `135f1d6f` 引入）。修前为 `3193791:crates/rustify-components/src/layout.rs:681-684`。
- 问题：修复把 `Top`/`Bottom`/`Left`/`Right` 从「所选边加两条相邻边」改成了「只选所选边」，`Inset` 的公共 `side` 语义因此偏离本期冻结的参考契约。按参考，`side=Top` 应贴住容器的上、左、右三边，并给 top-left/top-right 两个外角圆角（卡片顶部图片出血的标准用法），其他单边同理。修复后，`Top` 只抵消上边，左右仍保留容器 padding，四角全部是 0px；`Bottom`、`Left`、`Right` 也一样。[layout.rs:787-795](../../crates/rustify-components/src/layout.rs#L787-L795) 的圆角选择逻辑（top-left 需要 top 和 left 同时激活，其余角同理）本来就是按三边集合设计的；改成单边映射后，所有单边方向都再也选不中任何圆角，这一点也说明原映射是有意为之。任何 SDK 消费者按目录承诺的 Themes 3.3.0 `Inset` 语义使用 `side=Top/Bottom/Left/Right`，都会得到缩进且直角的内容，而且现有测试会把这个错误行为固定下来。
- 证据：本地参考 `ref/themes-main/packages/radix-ui-themes/src/components/inset.css`（package 3.3.0，SHA-256 `983044e9d1405a664ad00785f0941f63004c5f8094eae1b83e82668433c9c0ed`）中，`.rt-r-side-top` 覆盖 top、right、left 的 margin，圆角只给上方两角；`-bottom` 覆盖 right、bottom、left；`-left` 覆盖 top、bottom、left；`-right` 覆盖 top、right、bottom；`-x`/`-y` 的 `border-radius: 0px`；`-all` 覆盖四边。参考 playground 的 `test/inset/page.tsx:48` 在 Card 内用 `side="bottom" clip="padding-box"` 做图片出血，`:57`/`:66` 在 CheckboxCards 内用 `side="left"`。[字段矩阵 :71、:80](../validation/dom-components-p1/field-matrix.md#L71-L80) 写明来源是 `inset.props.tsx` 与同名实现/CSS；[catalog.rs:976](../../crates/rustify-components/src/catalog.rs#L976) 把 Themes 3.3.0 `Inset` 标为 partial，限制只列了 asChild/Slot 和 React 导出兼容，不包括 side 语义；计划 [§5.6](DOM-COMPONENTS-P1.md#L472-L490) 只要求「side默认all…消除最近容器 inset，不成为任意负 margin 逃逸」，并要求按参考登记。修复同时把单边行为写进了 [layout.rs:1147-1150](../../crates/rustify-components/src/layout.rs#L1147-L1150) 的 SSR 矩阵和 [dom-layout.spec.ts:185-207](../../tests/browser/dom-layout.spec.ts#L185-L207)；本轮在重建的 Catalog `863c3ec49352123f` 上跑这个 spec，7/7 通过，即当前 `layout-inset-top` 的 margin-left/right 是 3px 而不是 -13px，确认了实际行为。
- 修复与验证：把四个单边映射恢复为 `Top=[T,T,F,T]`、`Bottom=[F,T,T,T]`、`Left=[T,F,T,T]`、`Right=[T,T,T,F]`，圆角逻辑不变。保留这次新增的七方向×两种 clip 的 SSR 矩阵和响应式浏览器用例，但期望值改为逐条从参考 `inset.css` 推出：各方向的 margin 集合，单边只给两个相邻外角圆角，X/Y 圆角为 0，All 四角都有。响应式用例的断言也改成每个断点对应的三边集合，`layout_fixture.rs` 中的 fixture 可以保留。修复前，修正后的 SSR 测试应在当前代码上失败；修复后，SSR 测试和 `--project=component-catalog dom-layout.spec.ts` 都应通过。
- 置信度：高（0.9）。参考源码、冻结字段矩阵、目录承诺、修复前代码和圆角逻辑五方面一致；当前行为已在真实浏览器中确认。

## 2. 待核实 / 待决策项

- **D-1 · 是否有意放弃 Radix 的 side 语义（待决策，默认不需要）**：现有计划、字段矩阵和目录都把参考语义作为契约，没有任何材料表明要换成单边语义，所以 F-04 按「恢复参考」给出修复方向。如果维护者确实想要单边语义，那就是契约变更，需要同时修改字段矩阵、catalog 限制说明和迁移/组件文档，并给出替代用法；这需要维护者决定，不是作者侧修复能定的。
- **D-2 · 更正 R1 F-01 误报的已传播副本（随 F-04 处理）**：按仓库规则 [AGENTS.md「Evidence & Quotation」](../../AGENTS.md)，引用与原文不符时要找出所有副本一并修正并说明。R1 报告是历史，保持不变，由本报告撤回 F-01。作者侧还有这些副本把它认定为「成立/已修复」：[M4.md](DOM-COMPONENTS-P1.records/M4.md) 第 5、46、52、53、58、61 行；计划完成记录 M4 行（[DOM-COMPONENTS-P1.md:193](DOM-COMPONENTS-P1.md#L193)）；[M7.md:90](DOM-COMPONENTS-P1.records/M7.md#L90) 的 F-01 处置链接。处理 F-04 时应在这些记录里注明 R1 F-01 已撤回、原单边修复已回退，不能留下两份互相矛盾的说法。
- **V-1 · 修复前后 CSS 提交 p50 上升（非阻塞，可选补证）**：[M7 受影响终验](DOM-COMPONENTS-P1.records/M7.md#评审修复-r1-的受影响终验)只做了一对测量，CSS 提交 p50 从 15.4 ms 到 23.8 ms，GPU ack p50 从 16.6 ms 到 25.0 ms，两者 p95 都没变（24.7→24.6、26.0→25.9）。本轮按阶段拆开两份 JSON：变化全部来自 `publish_latency_ms` p50（9.0→17.9 ms，约一个 120 Hz 帧）；`sync_edit_ms`（0.5/0.6）、`css_projection_ms`（6.3→6.0）、resolves 和 long task 都没变。修复在 context 一致时的热路径只多了一次 `ContextKey` 构造和比较，O(字体表长度)，而且同步编辑耗时没有变化，因此无法把这次上升归因到代码，更像 rAF 相位导致的单次波动。作者记录已经如实保留了这个结果，本轮不另立 finding。如果要关闭，可以交替测 ≥3 轮修前/修后产物并比较中位数分布。

## 3. 覆盖与验证

### 3.1 固定范围与工作树指纹

- 意图：复核 R1 三项问题的修复效果，并检查修复是否引入新问题。用户输入只有计划路径；本计划已有 R1 代码评审报告，因此按开发计划模式进入复审。
- 仓库 `/Users/rockie/Documents/gh-rockie/rustify-ui`，main，HEAD `2304903c04ef8b634728b45173fb95123e2f928e`；评审日期 2026-10-27。
- 完成记录里登记的提交都在当前分支上：`d60a8e11`、`219b8481`、`620d25b1`、UI 修复 `d712990c`、`afa813e2`、`1e4d379b`，以及修复 `135f1d6f`。R1 已覆盖到 `31937917`；R1 target 之后只有 `135f1d6f`（修复）和 `2304903c`（只改计划），没有未登记的产品提交。
- 工作树：tracked diff 只有既有的 `.xgent-ai` 索引，SHA-256 `2afda7a3ec88c105954035dc6a2e3a05a205aba511d7fbf0027f9bc975199a31`；index 为空，`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`；40 个未跟踪文件（PRD/评审/R1 报告/SDLC 台账，计算方法同 R1：按路径排序，每行为路径+NUL+文件 SHA-256+LF，再对整个清单取 SHA-256）的汇总 SHA-256 为 `15fcff783839ac2d3aaa8eb49f9a42f876c105f8ae2c1a2ffa11c62e2122fb63`。这些都是非代码材料，不作为 finding 锚点；本报告和本轮台账不计入输入指纹。
- 写报告前 HEAD 没有变化。验证没有修改目标源码或测试；release 产物重建在 mbx 管理的 `target/` 下。

### 3.2 已审行为链

- F-01/F-04：`InsetSide::edges` → `Inset` 的逐断点 margin/圆角生成 → SSR 矩阵 → Catalog fixture → 浏览器响应式用例；对照参考 `inset.css`/`inset.props.tsx`、playground 用例、M1 字段矩阵、catalog 参考映射和计划 §5.6；检查了仓内其他 `Inset` 消费者（只有 catalog 冒烟、`theme_layout.rs` 的 X 方向和 fixture）。
- F-02：`StudioState::snapshot` → `Writer::enqueue` → `pump` → `Writer::prepare` 的匹配和不匹配两个分支 → `Autosave::serialize_snapshot` 的命名库缓存键 → `persistence::validate` → 失败后 `fail`/暂停/重试；`Keep` intent 在两个分支中的行为；`resolution_count` 是线程本地计数（[resolve.rs:15-19](../../crates/rustify-dom/src/theme/resolve.rs#L15-L19)），并行测试不会互相干扰。
- F-03：`Writer::new` 唯一的调用点 → editor 恢复 → `refresh_storage` 的状态分支 → 提交后的 `is_current` 自动补存 → `reset_fixture` 的 epoch 推进 → app.rs 快照里的 ack 输出。
- 只读了修复差异和它的上下游；R1 已覆盖、本次修复没碰到的范围沿用 R1 结论，没有重新完整审查。本轮由主 agent 单独完成，没有并行 reviewer。

### 3.3 实际执行的验证

| 命令 / 步骤 | 目标版本 | 实际结果 |
| --- | --- | --- |
| `mbx test --locked -p theme-studio --bins` | HEAD `2304903c` | 68 passed / 0 failed，包含四个新增 write_queue 测试 |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | 同上 | 166 passed / 0 failed；`inset_sides_preserve_unselected_margins_and_select_only_joined_corners` 通过，即单边语义已被固定 |
| `cargo fmt --all -- --check` | 同上 | exit 0 |
| `mbx xtask css --check` | 同上 | no drift（62,536 B，4,439 candidates） |
| `mbx xtask build-web --example theme-studio --release`，然后 `--example component-catalog --release` | 同上 | 均通过；build id 分别为 `f7f81f184ae45ccd`、`863c3ec49352123f`，与作者在 M5/M4 记录的产物一致 |
| `RUSTIFY_XTASK=target/debug/xtask RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' mise exec -- npx playwright test --project=component-catalog dom-layout.spec.ts --workers=1` | Catalog `863c3ec49352123f` | 7 passed（26.2s）；新用例确认当前 Top 只抵消上边 |
| 同样的环境，`--project=theme-studio theme-v2-storage.spec.ts --workers=1 -g 'queued autosave revalidates\|valid v2 reload reports\|cancelled legacy upgrade stays\|invalid v2 protects its exact bytes\|one writer preserves named action order'` | Studio `f7f81f184ae45ccd` | 5 passed（24.1s）：F-02 锁等待中切换 rem、F-03 有效 reload、legacy 取消和坏 v2 的 ack 为 null、命名操作顺序回归 |
| 读取修前/修后性能 JSON 的阶段统计 | 作者已提交的证据 | 见 V-1；本轮没有重新测性能 |

没有重跑完整 Playwright 项目、Clippy、`sources verify`、catalog 生成、GPU/性能测量、VoiceOver、真实 OS IME 或 Firefox/Safari；这些在作者记录里有各自的结果，本轮只作为历史证据，不算本轮独立通过。两个 Playwright 项目分开先后运行，没有并发。

## 4. 计划符合性

| 计划范围 | 本轮结果 |
| --- | --- |
| WORK-F16/AC-1—AC-2、M4/V4：布局语义与 Inset | **F-04**：修复后 `side` 的单边方向偏离 §5.6 要求按参考登记的语义和 M1 字段矩阵来源；原 R1 F-01 撤回 |
| WORK-F21/AC-1—AC-3、WORK-N6/AC-1、M5/V6：写入快照、失败保留与恢复 | F-02、F-03 已解决；context 变化重新校验、失败后保留并可显式重试、有效恢复的保存状态与计划 §3.3 一致 |
| V9 性能 | 计划没有新增毫秒 SLO；修复前后的单次 p50 差异按 V-1 处理，不阻塞 |

P2–P5 的后续组件、完整 Inspector、GPU 对等目录和公开发布不在本期，不计为缺陷。

## 5. 总体结论与修复顺序

**验收判定：需要修改。**

- 修复结论：**未完全解决**。选定的 3 项旧问题中，F-02、F-03 实际解决（2 项）；F-01 是 R1 误报，本轮撤回，记为不再适用（1 项）。但 F-01 的修复引入了阻塞性的回归 F-04（P2）。
- 修复差异的 patch 结论：**patch is incorrect**。依据是 F-04：`135f1d6f` 把与参考一致的 Inset 单边语义改错了，并用测试固定了错误行为。F-02、F-03 的修复本身正确，没有发现新问题。

修复顺序：

1. 先修 F-04：恢复四个单边映射，把 SSR 矩阵和响应式浏览器用例的期望值改为由参考 `inset.css` 推出，跑 SSR 测试和 `--project=component-catalog dom-layout.spec.ts`。
2. 同一批更正 D-2 列出的记录副本，注明 R1 F-01 已撤回、原修复已回退。
3. V-1 可选：如果要关闭性能疑问，交替做多轮修前/修后测量。

这个判定只覆盖所审修复差异和当前工作树，不构成合并、发布或需求验收的授权。

下一步：

> /apply-code-review docs/plan/DOM-COMPONENTS-P1.code-review.r2.md

修复后再用 `/review-code docs/plan/DOM-COMPONENTS-P1.md` 复审 F-04。
