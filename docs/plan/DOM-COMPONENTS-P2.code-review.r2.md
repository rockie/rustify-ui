# DOM-COMPONENTS-P2 · 代码复审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [DOM-COMPONENTS-P2](DOM-COMPONENTS-P2.md)，v0.2 | `9fd21ca86ff2dc64bfa19059a560a92c9da2b96c..834b62bdbdd4c1572b83b253d9499e398575bf65`；无未提交产品代码 | [代码初审 R1](DOM-COMPONENTS-P2.code-review.md)；评审对象 `de846332-6a97-449b-93b2-e2616a992403` |

独立评审 ID：`5c50ad02-b0e1-404a-a998-d3a1a688e224`。日期：2026-10-31（Asia/Shanghai）。本轮核对 R1 全部五项问题、U-01 和修复直接影响的回归，不是原始 218 文件实现的完整重审。R1 范围为 `721651e6e71906e046753dd258df5e7b527128e7..9fd21ca86ff2dc64bfa19059a560a92c9da2b96c`；本轮修复提交为 `89ea6be23e0414813a41cf3917702fb30c44d3f1`，当前核查版本为上述 target。

处置来源：[M2](DOM-COMPONENTS-P2.records/M2.md#代码评审-r1-修复)、[M3](DOM-COMPONENTS-P2.records/M3.md#代码评审-r1-修复)、[M4](DOM-COMPONENTS-P2.records/M4.md#代码评审-r1-修复)、[M5 合并复验](DOM-COMPONENTS-P2.records/M5.md#代码评审-r1-合并复验)。审查对象是 DOM SDK 库及直接测试消费者，edition 2021、组件 MSRV 1.99。未修改产品代码、测试或原计划，未提交、推送或启动修复。

## 上轮问题复审

来源均为 R1 原报告，SHA-256 `5397edd2c08956a552a3a9ccecf802c6a83ad06f9906e9431aa3b8feec659e95`；以下沿用原编号。

| 来源 / ID | 上轮问题 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01，P2 | 成功 reset 后旧 Draft 可在 blur 再次提交 | `89ea6be2` 增加重置通知与静默 Draft reset | **已解决** | [Form::reset](../../crates/rustify-components/src/form/provider.rs#L122)、[字段回调](../../crates/rustify-components/src/primitives/field.rs#L440)、[Draft::reset](../../crates/rustify-dom/src/draft.rs#L361)；本轮 Number/Color 聚焦 reset、blur/Enter/submit、拒绝及 Busy 四项浏览器检查通过 | 无 |
| R1 / F-02，P2 | 缺 Root / Item 的公开部件 panic 丢失内容 | `89ea6be2` 增加可恢复分支，保留外层 AttributeInterceptor | **已解决** | [Select](../../crates/rustify-components/src/primitives/select.rs#L548)、[Field](../../crates/rustify-components/src/form/field.rs#L185)；10 项 missing_context_ssr 与条件挂载浏览器通过，正常属性转发/取消也通过；剩余 expect_context 在内部 ContentBody，由有上下文的 Content 调用 | 无 |
| R1 / F-03，P2 | 非零 min、小数 step 的合法间隔被误拒绝 | `89ea6be2` 改用 step 序号 | **已解决**（原复现） | [model.rs](../../crates/rustify-components/src/primitives/slider/model.rs#L75)；原 100.2/100.3 场景、精确间隔候选、不足间隔拒绝、preserve_order 单测及浏览器通过 | 截断端点回归另列本轮 F-06 |
| R1 / F-04，P2 | 标签未关联实际显式 ID / 替换控件 | `89ea6be2` 以响应式 ID 和存活控制登记更新 Label/Form::control_id | **部分解决** | CSR 显式 TextField/Select、三轮替换、legacy 首错焦点通过；隔离 SSR + 禁 JS Chromium 仍复现延迟子树失配 | 补齐 SSR 的 Show/For/动态闭包关联，详见下节 |
| R1 / F-05，P2 | Input 卸载后计数不释放，重挂载禁用 | `89ea6be2` 分别登记存活 Input 的 NodeRef，cleanup 移除 | **已解决** | [claim](../../crates/rustify-components/src/primitives/field.rs#L231)、[Input](../../crates/rustify-components/src/primitives/field.rs#L546)；多轮重挂载、重复诊断、分别移除任一个后的回正/首错聚焦通过 | 无 |
| R1 / U-01 | Slider 与父 Layer 的 Escape 优先级待明确 | 用户决定保留关闭父层；计划已明确 | **已解决**（决策与验证） | [计划第 424 行](DOM-COMPONENTS-P2.md#L424)、[用户决定](../../.xgent-ai/sdlc/events/review/de846332-6a97-449b-93b2-e2616a992403/552ddc85-e81a-422e-a6d4-0292b077fbef.md)；本轮键盘/指针各取消一次、零 commit、父层关闭通过 | 无；不计入 finding 修复数 |
| R1 / 验证边界 | hydrate、直接应用、Docs、GPU、性能与人工项未重跑 | M5 有作者补验与复用范围 | **本轮未复核**（这些整组范围） | 本轮覆盖 raw CSR、host SSR 和逻辑测试；未把作者历史测试计为本轮通过 | 保留限制；当前两项确认缺陷不依赖这些补证 |

## 确认问题

### R1 / F-04 · [P2] 让延迟子树的 SSR 标签也关联实际 Control ID（部分解决）

- 定位：[form/field.rs:36–38](../../crates/rustify-components/src/form/field.rs#L36)，当前 target；旧问题复核，沿用 R1 身份。
- 问题：同一 Field 先声明 Label，再通过 `Show when=|| true`、`For` 或动态闭包生成显式 `id="show-title"` 的 TextField 时，实际 ID 到延迟子树序列化时才写入 binding，前面的 Label 已输出默认 `rui-probe1-form-1-title`。服务端不会因后续 signal.set 重写已经输出的标签，因此无脚本 SSR 页面仍缺少控件的关联名称，点击标签也不聚焦。修复解决了直接声明和浏览器响应更新，但没有完成这个公开 SSR 组合上的原 F-04 关闭条件。
- 证据：隔离消费者使用当前公共 SSR API；直接声明对照为 `for=direct-title`、`label.control.id=direct-title`、具名 textbox 数量 1、点击后聚焦。Show 为默认 `rui-probe1-form-1-title` 对实际 `show-title`；For 为默认 `rui-probe2-form-1-title` 对 `for-title`；动态闭包为默认 `rui-probe3-form-1-title` 对 `closure-title`。三种情况下禁 JS Chromium 均为 `label.control=null`、`getByRole('textbox', {name:'Title', exact:true})` 数量 0、点击不聚焦，console/pageerror/失败请求为空。新增 [forms_ssr.rs:857](../../crates/rustify-components/tests/forms_ssr.rs#L857)只断言直接构造后的 Form::control_id，未覆盖输出 Label 的引用。契约为 [计划 §5.4 第 5 项](DOM-COMPONENTS-P2.md#54-字段与-form)和 [§5.7](DOM-COMPONENTS-P2.md#57-ssr-与-hydrate)的 SSR Label/确定引用要求。
- 修复与验证：确保 SSR 序列化 Label 时能得到实际 Control ID，覆盖延迟子树，保留显式 ID 和 CSR 动态替换。以相同初值分别 SSR、hydrate，断言无脚本时的 label.control、可访问名称和点击聚焦，接管后 ID/引用保持一致；不能以 hydration 后修好代替静态输出正确。
- 置信度：高；公共 API 输出及原生浏览器行为均已复现，直接声明对照通过。

### F-06 · [P2] 不要把截断的末端计作完整最小步距（本轮新增）

- 定位：[primitives/slider/model.rs:75–76](../../crates/rustify-components/src/primitives/slider/model.rs#L75)，target 侧新增 diff；空间校验同受第 61–62 行影响。
- 问题：`index(max)` 对非步长对齐的 max 四舍五入，separated 随后把这个被截断的末端当成完整一步。配置 `min=100, max=100.26, step=0.1, min_steps_between=1`、接受值 `100.2/100.26` 现在合法且 Thumb 可操作，实际距离却只有 `0.060000000000002274`；修前同一配置和值被拒绝。min_steps_between=3 时，整个 0.26 范围也被当成能容纳 0.3 的距离。它破坏最小距离及空间不足失败契约，使 preserve_order/ARIA 边界允许过近值。这不是原 F-03 的浮点舍入误差，差值为 0.04。
- 证据：用 git show 从 base/target 分别提取未修改的 model.rs，在同一隔离程序、同一 `rustify_ui::snap` 下比较：base 返回 `Err(values: must satisfy the declared minimum interval after normalization)`，target 返回 `Ok`。公共 SSR Root 同时输出 enabled Thumb，aria-valuenow 为 100.2/100.26，没有 invalid-config。反证检查：[新增 clamped_maximum 测试](../../crates/rustify-components/src/primitives/slider/model.rs#L545)确实主动接受 0.26 范围的三步间隔，[M2 记录](DOM-COMPONENTS-P2.records/M2.md#代码评审-r1-修复)也描述该解释；但 [计划第 421–422 行](DOM-COMPONENTS-P2.md#L421)仍明确最小距离为 min_steps_between × step、空间不足失败且不能压缩，未登记此契约的变更。故按现行契约判为回归，不仅因行为变化报错。
- 修复与验证：保留正常格点的整数序号判定，对截断端点按实际可用距离计算空间与邻接限制，避免 round 向上扩容；统一初值、候选和 preserve_order。验证 100.2/100.3 的合法小数间隔仍可用、100.2/100.26 的 0.1 约束被拒绝，以及 0.26 范围不足以容纳 0.3 间隔。
- 置信度：高；有明确计划原文、base/target 实测差异和公共 SSR 结果。

## 待核实 / 待决策项

- 没有需要用户决定才能裁定的新增事项。U-01 已按现有用户决定关闭；F-06 依据当前明确的最小距离契约裁定，不能仅以新测试接受该行为认定满足计划。
- 未重建/运行当前版本的 hydrate、完整 SDK 模式矩阵、Studio/Catalog/Workbench、Docs islands 与性能 evidence；当前没有现成 `target/dom/hydration`，本轮未为已确认的 SSR 失败扩跑全部 features。M5 的补验、Docs 产品早于最后 editor 登记补充及性能非同轮交替比较等限制保留原位，未计为本轮通过。
- OS IME、VoiceOver、真机触控及 200% 浏览器 UI 缩放沿计划保留 P5 missing；合成 composition、Chromium touch pointer 与静态无脚本探针不替代人工结果。本轮没有独立 UI/UX 视觉验收结论。

## 覆盖与验证

### 意图与版本

修复意图（依据提交及处置记录）：结束成功 reset 的旧编辑会话，恢复缺上下文部件，纠正小数间隔，统一实际 ID，并让 Input 登记随存活 owner 释放。呈现路径为 DOM，无新增 GPU 或发布范围。

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；当前主 agent 单独完成，无并行 reviewer。
- 模式：开发计划模式的修复效果复审。登记的实现 `9fd21ca86ff2dc64bfa19059a560a92c9da2b96c` 和修复 `89ea6be23e0414813a41cf3917702fb30c44d3f1` 均已解析并确认是 HEAD 祖先，无未登记产品代码提交。
- 实际净 diff：`9fd21ca86ff2dc64bfa19059a560a92c9da2b96c..834b62bdbdd4c1572b83b253d9499e398575bf65`，23 个变更路径。target tree：`4edee101808951131c9c83b97e182e21987c9f3b`。排除代码 findings 的回写提交：`3099be363f698c963fd78bd91e104b309ef1d0da`（计划 SHA）、`19cba0c604a81817d9a08a86e3ad80e02e17a053`（SDLC）、`834b62bdbdd4c1572b83b253d9499e398575bf65`（修复 SHA 回填）。处置记录仍作为上下文读取。
- 起始工作树 dirty：5 个 tracked SDLC 文件，8 个 untracked 文件（旧 R1 报告和该轮 SDLC 事件/状态）；index 无变更。全部纳入盘点后排除代码 findings，没有未提交产品代码，不覆盖原始未提交内容。
- 起始 11,530 个 tracked 文件内容聚合 SHA-256：`b7d3bf8486fb397e5f25e2f76dc509d47d7fcd39b42c73a21ce1760bdf084adb`；按 git ls-files -z 顺序拼接 path + NUL + sha256(file) 二进制摘要后再次 SHA-256。index 的 git ls-files -s -z 字节 SHA-256：`5e823ce2b0140bd2272fd6cd0ea4214a203a960b8046bfb184a1606b7c27d5fc`。另固定起始 8 个 untracked 的逐文件 hash。写报告前全部起始文件复核无变化、HEAD/index 未变；报告及台账输出不属于输入。
- 当前计划 SHA-256：`b999be6431c53dfac3438138f38c3c1f83cb430a0322acd5175fa5c26958c7b0`，同 HEAD；旧 R1 报告 hash 如上，保持原结论不动。

### 覆盖清单

| 范围 | 实际核查 | 限制 |
| --- | --- | --- |
| 六个产品 Rust 源文件及直接调用链 | 完整修复代码 diff；Form/Field、Draft、原生提交重置桥、editor、Select Content/Item、Slider model 与 snap | 旧问题逐项复核；没有重审原阶段全部 choice/overlay |
| SSR、raw spec 与三个 fixture 源文件 | 检查断言与观察入口，55 SSR + 47 raw；针对漏测组合另作隔离实验 | 原用例未覆盖 F-04 延迟 SSR 和 F-06 端点契约 |
| CSS candidates / catalog | class 扫描漂移和目录检查通过 | 不重评整套皮肤视觉矩阵 |
| 计划、M2–M5 处置、修后性能 JSON | 核对范围、处置来源、指标结构和复用限制 | 不逐样本重测历史性能，不把作者记录计为独立复验 |
| 原始 DOM 产品 | 重建后串行单 project 的真实 Chromium；现有 console/pageerror/失败请求断言 | 无真实 GPU 或 OS 辅助技术验收 |

### 本轮实际执行

共同工具链：Rust 1.99.0、mbx 1.15.0、Node 26.12.1；Chromium 153.0.8010.12。所有产品测试在当前 target 对应源码上运行。

| 命令 / 实验 | 结果 |
| --- | --- |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr --test missing_context_ssr --test ssr_controls` | 32 + 10 + 13，共 55 passed |
| `mbx test --locked -p rustify-components --lib primitives::` | 32 passed |
| `mbx test --locked -p rustify-dom --lib draft::` | 14 passed |
| `mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml` | 成功；随后浏览器使用此新构建 |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives primitives.spec.ts` | 47 passed，56.1s，单 project / 单次运行；含 R1 回归及原属性/取消/资源检查 |
| `mbx clippy --locked -p rustify-components -p rustify-dom --all-targets -- -D warnings` | 通过 |
| 根及独立 DOM workspace 的 cargo fmt 检查 | `cargo fmt --all -- --check` 与 `cargo fmt --manifest-path tests/dom-only/Cargo.toml --all -- --check` 均通过 |
| `mbx xtask css --check` | no drift，94,986 B / 4,814 candidates |
| `mbx xtask catalog --write docs/components.md --check` | no drift，105 public entries / 97 stable reference names / 24 legacy categories |
| `git diff --no-ext-diff --no-textconv --check` | 通过 |
| 公共 SSR 组合 + 禁 JS Chromium 关联探针 | 直接声明对照通过；Show/For/动态闭包复现 F-04；无 console/pageerror/失败请求 |
| base/target 原样 Slider model 比较 + 公共 SSR Root | base 拒绝而 target 接受 0.06 间隔；SSR Thumb enabled，复现 F-06 |

现有检查合计 148 项通过，仍不足以关闭两项确认问题。wasm 构建的 fallback_element / guest_click 两项 dead_code warning 与 R1/作者记录一致，非本轮新增缺陷。

隔离实验在本机临时目录 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p2-r2-review-nvlg8hej/`。ssr-probe 以 path 依赖当前 components/dom 的 SSR 公共 API，锁从仓根 Cargo.lock 复制并离线解析，仅加入探针包。没有安装工具或修改产品依赖。命令为 `CARGO_TARGET_DIR=<仓根>/target mbx run --offline --quiet --manifest-path <临时目录>/ssr-probe/Cargo.toml`；添加第二个 bin 后主程序加 `--bin dom-ssr-r2-probe`，模型比较加 `--bin gap`。模型来自 git show base/target 的原文件，共用实际 rustify_ui::snap，不是手写替代算法。初次模型探针缺 Orientation 重导出，补齐探针后形成比较结果，该失败不是产品失败。输出 ssr.html、ssr-browser.json；复现参数和实际结果已保存于本报告，临时文件不是跨机器交付凭据。未向仓库增加测试或修复；自启预览和浏览器已结束。

## 计划符合性

| 本轮相关承诺 | 结论 |
| --- | --- |
| §5.4 成功 reset 静默结束旧 Draft；拒绝/Busy 保留 | 已核实，F-01 关闭 |
| §5.1 / §5.5 缺上下文保留内容与诊断，保留正常取消/属性转发 | 已核实，F-02 关闭 |
| §5.3 整数 step 序号与候选/顺序一致，最小距离始终满足 | 原 F-03 复现解决；F-06 为新增端点回归 |
| §5.4 第 5 项、§5.7 实际 ID 与 SSR 引用 | CSR 关联/替换/legacy 聚焦通过；SSR 延迟子树仍失败，F-04 部分解决 |
| Input 登记随存活 owner，动态替换恢复 | 已核实，F-05 关闭 |
| §5.3 已选定的父 Layer Escape 语义 | 已核实，U-01 关闭 |
| V-8/V-10 的 hydrate、直接应用与整期验收 | 本轮未完整重跑；保留作者 M5 基线与限制，不扩为整期/跨期结论 |

应继续保持的设计：reset 通知只在 FormState 接受重置后运行；重复 Input 状态从存活登记派生；Select 外层 AttributeInterceptor 保留属性转发；Slider 正常格点用整数步序号避免原浮点误拒绝。这些路径有本轮通过的行为检查。

## 总体结论与修复顺序

**验收判定：需要修改。**

修复效果：**未完全解决**。R1 五项 finding 中 4 项已解决、F-04 部分解决；U-01 的决定与键盘/指针验证闭合。新增 1 项 P2（F-06），当前共 2 项 P2 待修，没有 P0/P1/P3。

本轮修复增量为 `patch is incorrect`：F-06 引入违背最小距离契约的端点行为。F-04 的剩余 SSR 失败按旧问题复审裁定，不冒充新增问题。现有测试通过和整组未复验范围不能抹去这两项有具体输入和结果的失败。

先补齐 F-04 静态关联，再修 F-06 端点容量/距离；分别以无脚本 SSR/接管、合法小数与截断端点对照验证，复验相关 raw 回归。关闭后由用户发起下一轮独立复审；本报告不自动开始修复、UI 验收或 PRD 覆盖核查。

下一步：`$apply-code-review docs/plan/DOM-COMPONENTS-P2.code-review.r2.md`；修复后：`$review-code docs/plan/DOM-COMPONENTS-P2.md`。
