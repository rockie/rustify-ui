# DOM-COMPONENTS-P1 · 代码复审 R3

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3，修复效果复审 | [DOM-COMPONENTS-P1 · Colors 与统一主题基础](DOM-COMPONENTS-P1.md)，SHA-256 `bc58f3c36774f1cb593f9c4ecd5d958810ebdbad9d92ce0ef9a8c4e68738e536` | `2304903c04ef8b634728b45173fb95123e2f928e..13ab1a5b5b0d99a08d0b482db452273f713e10ba`；代码修复为 `38bc9c310bb055fb03acdefc10bfe64c0fb4b0e9`，另核对当前工作树，没有未提交产品代码 | [代码复审 R2](DOM-COMPONENTS-P1.code-review.r2.md)，SHA-256 `3ab2ece98d55653bfb2c33628c32fb1136bbad6bf2bd96bc034f3ad25b3ed7d6` |

本次复审的明确输入只有计划路径。按计划完成记录和既有报告进入第三轮修复效果复审：逐项核对 R2 的旧问题、F-04 与待核实/待决策项，并检查 R2 之后修复差异的直接回归；不把作者“已修复”声明当作关闭证据，也不把本轮扩大成原始完整变更的重审。

## 复审记录

- 上轮所审范围：`3193791720edbb1c933bfa852773f0b4710a7b15..2304903c04ef8b634728b45173fb95123e2f928e`，结论为需要修改，新增 F-04（P2）。
- 本轮修复差异：`2304903c04ef8b634728b45173fb95123e2f928e..13ab1a5b5b0d99a08d0b482db452273f713e10ba`。其中 `38bc9c31` 是登记的 R2 修复提交，`13ab1a5b` 只改计划；没有未登记产品提交。产品代码、测试和扫描清单的净差异只有 `crates/rustify-components/src/layout.rs`、`crates/rustify-components/css/candidates.json`、`tests/browser/dom-layout.spec.ts`。
- 已有处置记录：[M4 评审修复 R2](DOM-COMPONENTS-P1.records/M4.md#评审修复-r2)、[M7 评审修复 R2 的受影响终验](DOM-COMPONENTS-P1.records/M7.md#评审修复-r2-的受影响终验)。这些记录用于定位修复和证据；下表结论由本轮当前代码、参考原文及实际运行独立核对。

## 上轮问题复审

| 来源报告 / F-ID 或稳定章节 | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / R1 F-01 | R1 将参考的三边 Inset 语义误判为缺陷；R2 判为不再适用，并由 F-04 追踪错误单边修复 | `38bc9c31` 回退单边映射，恢复原参考行为 | **不再适用** | 当前 [layout.rs:684-687](../../crates/rustify-components/src/layout.rs#L684-L687) 与 R1 前 `3193791720edbb1c933bfa852773f0b4710a7b15:crates/rustify-components/src/layout.rs:681-684` 一致，也符合参考 [inset.css:49-88](../../ref/themes-main/packages/radix-ui-themes/src/components/inset.css#L49-L88)。原 R1 报告保留历史结论，R2 的撤回不被本轮改写 | 无 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / R1 F-02 | 排队 autosave 复用旧 ResolveContext proof；R2 已确认解决 | `135f1d6f` 绑定完整 `ContextKey`，本轮未再改存储链 | **已解决** | 当前 [write_queue.rs:395-404](../../examples/theme-studio/src/write_queue.rs#L395-L404) 只在 key 匹配时复用 proof，否则完整序列化；[state.rs:39-72](../../examples/theme-studio/src/state.rs#L39-L72) 覆盖 revision、rem、reduce motion、字体表、回退字体与解析策略，[controller.rs:552-569](../../examples/theme-studio/src/controller.rs#L552-L569) 从同一 context 生成 proof/key。`2304903c..13ab1a5b` 对 `examples/theme-studio/src` 和 `crates/rustify-dom/src` 的 diff 为空，R2 已运行的模型与真实锁等待浏览器证据仍适用于这些未变源码 | 无；本轮不重复施加补丁或重跑无关存储项目 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / R1 F-03 | 有效 v2 reload 后误报未确认保存；R2 已确认解决 | `135f1d6f` 在有效 baseline 且 Ready 时初始化 ack，本轮未再改存储链 | **已解决** | 当前 [write_queue.rs:157-159](../../examples/theme-studio/src/write_queue.rs#L157-L159) 保留初始化条件；[controller.rs:173-189](../../examples/theme-studio/src/controller.rs#L173-L189) 用同一恢复 envelope 建立 editor 且 epoch 为 0，[controller.rs:609-614](../../examples/theme-studio/src/controller.rs#L609-L614) 仍按完整 draft/preferences、epoch 和 pending 判定保存状态。上述源码自上轮无差异，R2 的有效 reload、legacy 取消和坏 v2 浏览器证据未被本轮 Inset 改动失效 | 无 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / F-04 | [P2] R1 修复把 Inset 单边改为只抵消一边，偏离三边/两角参考语义 | `38bc9c31` 恢复三边映射，保留圆角逻辑并更正 SSR/浏览器 oracle | **已解决** | [layout.rs:681-687](../../crates/rustify-components/src/layout.rs#L681-L687) 的 Top/Bottom/Left/Right 分别恢复三边；[layout.rs:791-797](../../crates/rustify-components/src/layout.rs#L791-L797) 因而只在两个相邻外角生成半径。[SSR 矩阵:1145-1216](../../crates/rustify-components/src/layout.rs#L1145-L1216) 独立列出七方向×两种 clip 的边和角；[浏览器断言:185-207](../../tests/browser/dom-layout.spec.ts#L185-L207) 使用同一参考边集合并覆盖 390/519/520/767/768/1023/1024/1280px。当前 SSR 166、CSR 121、SSR 集成 2 均通过；最新 Catalog `dd1a11d8f8c000cb` 的定向浏览器用例 1/1 通过，桌面/390px 截图已实际查看 | 无 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / D-1 | 是否有意放弃 Radix side 语义；默认恢复冻结参考 | 已按现有契约恢复，没有引入单边替代契约 | **不再适用** | 当前实现、字段矩阵和 catalog 均继续沿用参考，不需要维护者另作产品决策 | 无 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / D-2 | 更正 R1 F-01 误报在 M4/M5/M7、计划和当前台账中的传播副本 | `13ab1a5b` 及当前未提交台账已接续纠错，历史报告/事件不改写 | **已解决** | [M4:44-48](DOM-COMPONENTS-P1.records/M4.md#L44-L48) 明确撤回及回退，[M4:65-90](DOM-COMPONENTS-P1.records/M4.md#L65-L90) 分离历史与当前验证；[M5:61](DOM-COMPONENTS-P1.records/M5.md#L61)、[M7:90-118](DOM-COMPONENTS-P1.records/M7.md#L90-L118) 和[计划恢复快照/完成记录:177-195](DOM-COMPONENTS-P1.md#L177-L195) 不再把 F-01 当作成立缺陷。当前 R1/R2 SDLC 页也已注明误报、回退和 F-04 修复；检索到的旧“成立”措辞仅在历史评审原文或无关计划的同名编号中 | 无 |
| [R2](DOM-COMPONENTS-P1.code-review.r2.md) / V-1 | R1 修复前后单对测量的 CSS/GPU 提交 p50 上升，归因未定；非阻塞、可选补证 | R2 修复未复测；本轮只改 Inset，也未复测 | **待核实（保留，非阻塞）** | 当前修复差异不含 Studio 保存、主题发布或 GPU 链路，不能据 Inset 浏览器通过关闭该项，也不能把单次 p50 波动归因为代码回退。原测量和归因不确定性继续保留 | 如需关闭，交替执行至少三轮修前/修后测量并比较分布；不阻塞本次 F-04 验收 |

## 1. 确认问题（本轮新增）

未发现可确认的新增问题。

## 2. 待核实 / 待决策项

- **V-1 保持可选待核实**：本轮没有性能复测，不宣称 R1 测量中的 p50 差异已解释或消失；它不是 F-04 修复链上的新增回退，也不阻塞本次代码验收。
- 没有新的产品范围待决策项。未重跑整期 V1–V10、完整 Playwright 项目、GPU/性能、VoiceOver、真实 OS IME、Firefox/Safari 或真人研究；这些边界不冒充本轮通过，也不因本轮修复范围较窄而升级为新增 finding。

## 3. 覆盖与验证

### 3.1 固定范围与工作树指纹

- 仓库 `/Users/rockie/Documents/gh-rockie/rustify-ui`，main，HEAD `13ab1a5b5b0d99a08d0b482db452273f713e10ba`；评审日期 2026-10-27。
- 原输入 `docs/plan/DOM-COMPONENTS-P1.md`；R2 target 为 `2304903c04ef8b634728b45173fb95123e2f928e`，本轮 current target 为上述 HEAD。完成登记的全部提交均已验证在当前分支；R2 之后只有 `38bc9c31` 产品修复和 `13ab1a5b` 计划回写。
- 当前 tracked 工作树 diff 只含既有 `.xgent-ai` 索引和两张既有布局验收 PNG，SHA-256 `171f42f40bc853c98d3c6aad83093b920f656c728c057946775ebb597ceafddb`；index 为空，SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。
- 写报告前枚举 48 个未跟踪文件，均为 PRD、计划/代码评审、SDLC 和处置材料；按路径排序，以“路径 + NUL + 文件 SHA-256 + LF”计算的总清单 SHA-256 为 `cffe107baa9d571a7fc071754051bae3557ea15e6c1ea4921f9ffd8ae4981cd5`。没有未提交产品代码；本报告和随后本轮台账不计入输入指纹。
- 关键目标文件 SHA-256：`layout.rs` `587c89f4dda2abe4a31958cdbede2ff18fa48baada01fcba5209a61071ea71c6`，`dom-layout.spec.ts` `9c7856726bb87bad2bebd5773c3d40757b80ee5e62b0257d27f8c885e02a53f8`，`candidates.json` `d31334cb0eb57a21c56355bd58faaa2166e157b651ad81c43a93d93dee5be6d5`。验证只写 `target/` 与 `test-results/`，没有修改目标源码或既有验收 PNG。

### 3.2 已审行为链

- F-01/F-04：`InsetSide::edges` → 每断点 margin/圆角生成 → SSR 七方向×两 clip 矩阵 → Catalog fixture → 浏览器逐边和响应式断言；对照 Themes `inset.css`、M1 字段矩阵、catalog 限制和计划 §5.6。
- F-02/F-03：回读当前 context key、proof 复用/完整重验分支、恢复时 ack 初始化及保存状态分支，并以精确 diff 证明这些源码自上轮未变；上一轮真实运行证据只用于未变行为，不冒充本轮重新执行。
- D-2：计划、M4/M5/M7、历史 R1/R2 报告及当前 SDLC 页之间的纠错链；历史结论保留，当前处置不再互相矛盾。
- 本轮由主 agent 单独完成，没有并行 reviewer。R1 已覆盖而本轮修复未触及的原始实现没有重新完整审查。

### 3.3 实际执行的验证

| 命令 / 步骤 | 目标版本 | 实际结果 |
| --- | --- | --- |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | HEAD `13ab1a5b` | 166 passed / 0 failed，包含当前七方向、两 clip、边和圆角矩阵 |
| `mbx test --locked -p rustify-components --lib` | 同上 | 121 passed / 0 failed |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test theme_layout` | 同上 | 2 passed / 0 failed |
| `mbx clippy --locked -p rustify-components -p component-catalog --all-targets -- -D warnings`；`mbx clippy --locked -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings` | 同上 | 两条均 exit 0 |
| `cargo fmt --all -- --check`；`git diff --check 2304903c..13ab1a5b` | 同上 | 均 exit 0 |
| `mbx xtask css --check` | 同上 | no drift，62,536 B / 4,441 candidates |
| `mbx xtask build-web --example component-catalog --release` | 同上 | 通过，build `dd1a11d8f8c000cb`，WASM 8,080,867 B；仅保留既有依赖 warnings |
| `RUSTIFY_XTASK=target/debug/xtask RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' mise exec -- npx --no-install playwright test --project=component-catalog dom-layout.spec.ts --workers=1 -g 'Inset selects each side'` | Catalog `dd1a11d8f8c000cb` | 1 passed（10.1s）；七个方向的逐边 margin 和八个宽度下的 Top→Right→Bottom→Left 响应式三边切换通过 |
| 实际查看本次 `test-results/` 的 Inset 桌面和 390px 截图 | 同一浏览器产物 | 两图已读取；可见参数 fixture 的贴边/留白与断言方向一致。圆角关闭判据由 SSR 矩阵承担，不从无背景截图推断 |
| `git diff --exit-code 2304903c..13ab1a5b -- examples/theme-studio/src crates/rustify-dom/src` | 当前修复范围 | exit 0；F-02/F-03 的源码与依赖链未变 |

未运行完整 `dom-layout.spec.ts`、其他 Playwright 项目或 Studio 存储测试；本轮只执行 F-04 的直接浏览器回归，避免把无关项目的旧结果计为当前独立通过。两个 Clippy 命令顺序执行，浏览器只运行一个 project，没有并行 Playwright。

## 4. 计划符合性

| 计划范围 | 本轮结果 |
| --- | --- |
| WORK-F16/AC-1—AC-2、M4/V4：Inset 布局语义、响应式和 SSR 外部 CSS | F-04 已解决；当前实现恢复冻结参考的三边/两角语义，SSR 与真实浏览器直接证据满足 R2 的关闭条件。R1 F-01 维持不再适用 |
| WORK-F21/AC-1—AC-3、WORK-N6/AC-1、M5/V6：有效恢复、快照校验和保存状态 | F-02/F-03 维持已解决；相关源码自上轮未变，没有证据表明 Inset 修复影响该契约 |
| AGENTS Evidence & Quotation、R2 D-2 | 误报的当前作者记录和台账副本已更正，历史评审原件保留 |
| V9 性能 | V-1 的可选补证未做，保留原不确定性；本期没有新增毫秒 SLO，本轮也不作性能无回退声明 |

P2–P5 后续组件、通用 Primitives、完整 Inspector、GPU 对等目录、公开发布与部署不在本期，不记为缺陷。

## 5. 总体结论与修复顺序

**验收判定：通过验收。**

- 修复结论：**已解决**。F-02、F-03、F-04 均已解决；F-01 维持有依据的不再适用。D-1 无需新的产品决策，D-2 已解决；V-1 仅是保留的非阻塞可选补证。
- 修复差异的 patch 结论：**patch is correct**。本轮未发现新增 P0–P3，R2 的 F-04 关闭判据已由当前代码、SSR 矩阵和最新 release 浏览器行为共同满足。
- 覆盖限制：这是 R2 修复效果复审，不是原始 602 文件完整变更的重新审查；未重跑整期 V1–V10、性能或人工辅助技术检查，相关历史边界不被本轮关闭。

没有必须修复项。若选择关闭 V-1，再单独进行多轮同环境性能测量；无需为通过本次代码验收扩大范围。

本计划代码验收完成。若还需独立 UI/UX 验收或核对本期 PRD 覆盖，可分别执行：

> /review-ui docs/plan/DOM-COMPONENTS-P1.md

> /review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P1.md

本判定只覆盖声明的修复差异和当前工作树，不构成合并、发布或部署授权。
