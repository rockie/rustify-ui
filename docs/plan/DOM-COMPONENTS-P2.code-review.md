# DOM-COMPONENTS-P2 · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，首次代码评审 | [DOM-COMPONENTS-P2](DOM-COMPONENTS-P2.md)，v0.2 | `721651e6e71906e046753dd258df5e7b527128e7..9fd21ca86ff2dc64bfa19059a560a92c9da2b96c`；开始时工作树/index 干净，无未提交代码 | 无；[既有报告](DOM-COMPONENTS-P2.review.md)是计划文档评审，不作为代码复审前轮 |

审查 SDK 库及其直接消费者；Rust edition 2021、组件 MSRV 1.99，工具链以仓内 mise 配置为准。以下问题均为本轮新增，定位在 target 侧改动行；当前 checkout 的相关源码与 target 完全一致。未修改产品源码、测试或原计划，未提交或推送。

## 确认问题

### F-01 · [P2] 成功重置表单时一并结束旧 Draft 会话

- 定位：[form/provider.rs:98–99](../../crates/rustify-components/src/form/provider.rs#L98)。
- 问题：聚焦 NumberField 并输入 `18`，随后通过原生 `HTMLFormElement.reset()` 触发应用的 reset 回调，即使应用已经将接受值设为 `12` 并调用 `Form::reset()`，此处也只清 FormState，没有通知已注册字段丢弃旧 Draft。输入框继续显示 `18`，FormData 暂时为 `12`；下一次 Tab/blur 又提交旧草稿，把刚重置的业务值改回 `18`。ColorField 使用同一路径。新增原生 reset 桥因此没有完成“应用处理后显示与应用状态一致”的契约，且旧编辑会话能污染重置后的新会话。
- 证据：本轮真实 Chromium 在原始 `Fields` 消费者上观测到 reset 后 `{editor:"18", accepted:"12", requests:""}`，blur 后为 `{editor:"18", accepted:"18", requests:"number:18"}`。消费者 [fields.rs:73](../../tests/dom-only/primitives/src/fields.rs#L73)明确在接受 reset 时设置业务值并调用 `form.reset()`；[原生 reset 桥](../../crates/rustify-components/src/primitives/form.rs#L210)没有字段会话清理；[Draft::blur](../../crates/rustify-dom/src/draft.rs#L109)继续提交未结束的 edit。契约为 [PRD WORK-F8/AC-1](../PRD-DOM-COMPONENTS.md#work-f8-填写字段并获得一致的表单反馈)、[计划 §5.4 第 6 项](DOM-COMPONENTS-P2.md#54-字段与-form)。
- 修复与验证：为**应用已接受且成功的** `Form::reset()` 建立字段会话失效通知，清除旧 Draft 并同步当前接受值；拒绝 reset、Busy 不应清掉草稿。回归应在编辑器保持焦点时调用原生 reset，确认立即显示重置值，之后 blur/Enter/submit 不再交付旧请求；NumberField、ColorField 均覆盖。
- 置信度：高；实际浏览器复现，接受值和请求日志均可观察。

### F-02 · [P2] 缺少 Root 时保留 Select 部件内容并输出诊断

- 定位：[primitives/select.rs:548–550](../../crates/rustify-components/src/primitives/select.rs#L548)。
- 问题：公开的 `select::Viewport` 无条件 `expect_context::<Context>()`。将它放在缺少 Select Root 的条件分支中，组件构造直接 panic，WASM 报 `RuntimeError: unreachable`，调用方 children 不会输出。计划已经把“缺 Root”定义为带定位诊断、保留原内容的可恢复组合错误；这里将它升级成 WASM abort。Value、Content、ItemText、ItemIndicator、scroll button，以及 Field Description/Message 也存在同类 context 假设，应按同一根因排查。
- 证据：隔离消费者通过按钮显示 `<p::select::Viewport><p>Caller content</p></p::select::Viewport>`，实际得到 `retainedContent=0`，错误栈明确包含 `expect_context::<...select::Context>` → `__component_viewport` → `__rust_abort`。作为反证，[Trigger](../../crates/rustify-components/src/primitives/select.rs#L332)和 [Item](../../crates/rustify-components/src/primitives/select.rs#L614)已有 `missing-root` 降级路径，并非 SDK 统一选择 panic。约定见 [计划 §5.1](DOM-COMPONENTS-P2.md#51-组合状态与-scope)末项及 §4 的“缺 Root 保留内容并给 part 诊断”。
- 修复与验证：对缺少所属上下文的公开部件使用一致的 `missing-root`/缺所属 Item 诊断，保留 children 并禁止默认交互；不要通过全局捕获 panic 实现。增加 host 与条件挂载浏览器检查，断言内容和诊断存在、无 pageerror，其他控件仍能操作。
- 置信度：高；运行错误栈和计划明确约定相符。

### F-03 · [P2] 按 step 序号比较 Slider 最小间隔

- 定位：[primitives/slider/model.rs:97–98](../../crates/rustify-components/src/primitives/slider/model.rs#L97)。
- 问题：间隔判断直接用两个规范化后的浮点值相减，容差却只按 gap 的量级计算。合法配置 `min=100, max=200, step=0.1, min_steps_between=1`、两个具名值 `100.2/100.3` 会得到差值 `0.09999999999999432`，小于阈值 `0.09999999999999983`，因此被当成非法间隔。初次挂载会显示配置错误并禁用全部 Thumb；同一判断也会拒绝邻接的合法候选。普通金额/小数范围即可触发，不需要 NaN 或越界输入。
- 证据：[Model::new 的间隔校验](../../crates/rustify-components/src/primitives/slider/model.rs#L180)和 [candidate](../../crates/rustify-components/src/primitives/slider/model.rs#L261)均调用此函数。本轮真实浏览器对上述值观测为 `data-rustify-issue="invalid-config"`、`Invalid slider configuration`、Thumb disabled。数值差与阈值另经 JavaScript 双精度运算核实。[计划 §5.3](DOM-COMPONENTS-P2.md#53-slider值身份手势)明确要求以 min 为基点的整数 step 序号避免此类误差；对应 PRD WORK-F7/AC-1。
- 修复与验证：使用规范化 step 序号判断距离和边界，使初始校验、候选与 preserve_order 使用一致规则。覆盖非零 min、小数 step、恰好最小间隔及确实小于间隔的情况，验证合法配置可操作、非法间隔仍被拒绝。
- 置信度：高；数值根因和实际禁用结果均已复现。

### F-04 · [P2] 将 Field 标签关联到控件实际采用的 ID

- 定位：[form/field.rs:22–25](../../crates/rustify-components/src/form/field.rs#L22)。
- 问题：FieldBinding 和 Label 预先固定使用 `form.control_id(field)`，但此处允许首个控件返回显式 `id`，没有更新绑定。正常组合 `<Field><Label/><text_field::Root id="explicit-title">…</Root></Field>` 因而产生指向不存在节点的 `for`；Select Root 同样受影响。标签点击不能定位控件，标签文本也没有成为控件的关联名称。未显式指定 ID 的控件在同一 Field 中重新挂载时还会因 `claimed` 不释放而改用另一个 ID，留下相同问题。
- 证据：现有 Select fixture [selects.rs:104–105](../../tests/dom-only/primitives/src/selects.rs#L104)已直接使用这一组合。本轮在未经改动的原始验收页面发现 `Fruit` 标签 `for="rui-form-35-fruit"`、实际 Trigger ID 为 `raw-select-trigger`，前者在文档中不存在，`getByRole('combobox', {name:'Fruit', exact:true})` 数量为 0。隔离 TextField 也复现标签指向不存在的 `rui-form-38-title`、实际 ID 为 `explicit-title`。关联承诺见 [迁移指南第 101 行](../dom-components-migration.md#L101)、计划 §5.4 第 5 项和 PRD WORK-F8/AC-1。
- 修复与验证：让 Field 的标签/绑定使用实际 Control 的同一 ID，并处理该 Control 的释放和替换；保留显式 ID 的公开能力。覆盖显式 ID 的 TextField、Select 及条件重挂载，断言 `label.control`、可访问名称、点击聚焦和首错定位均指向当前节点。
- 置信度：高；仓内现有消费者已触发，另有独立字段复现。

### F-05 · [P2] 按当前挂载数量判断重复 Input

- 定位：[primitives/field.rs:229–233](../../crates/rustify-components/src/primitives/field.rs#L229)。
- 问题：`claim()` 只累加构造次数，Input 卸载时没有减计数，`duplicate` 也只会变成 true。在一个持续存在的 text_field Root 内用 Show 显示单个 Input，隐藏后再次显示，虽然 DOM 始终最多一个编辑器，第二次构造却会设置 `multiple-inputs`；`disabled()` 随即把新编辑器永久禁用。NumberField/ColorField 的 Input 复用这套实现，因此常见的条件展示、编辑模式切换也会失效。
- 证据：隔离浏览器复现依次得到 `{count:1, disabled:false}` → 隐藏后的 `count:0` → `{count:1, disabled:true, issue:"multiple-inputs"}`，全程无其他 Input。禁用因果链为 [disabled](../../crates/rustify-components/src/primitives/field.rs#L220) → [claim](../../crates/rustify-components/src/primitives/field.rs#L229) → [input_as 调用](../../crates/rustify-components/src/primitives/field.rs#L523)。这是本轮新增组合入口的问题，原先没有 Root/Input 的独立条件挂载路径。
- 修复与验证：将登记和清理绑定到每个 Input 的实际 owner，以当前存活的编辑器数量派生重复状态；从双 Input 恢复到单 Input 时也应解除诊断。测试单 Input 多轮卸载/重挂载后仍可输入，以及真实双 Input 仍能被诊断。
- 置信度：高；稳定复现且无其他禁用来源。

## 待核实 / 待决策项

- **U-01：活动 Slider 的 Escape 与父 Layer 的优先级。** 实测 modal 内 Thumb 按住 ArrowRight 后按 Escape，父 Layer 在 document capture 中先关闭；Slider 因 cleanup 收到一次 cancel、零 commit。Slider 自己的 Escape handler 没有机会执行。计划 §5.3 承诺 Escape 取消手势，§5.5 又只明确 composing/Draft 的优先级，未明确 Slider 是否应先消费 Escape；因此本轮不将“父层同时关闭”列为确认缺陷。需明确这一优先级，再用嵌套 Layer 的键盘/指针手势验证。当前证据位置：[overlay::route](../../crates/rustify-dom/src/overlay.rs#L828)、[Slider key_down](../../crates/rustify-components/src/primitives/slider/browser.rs#L335)。
- **验证边界。** 本轮没有重跑 hydration、Studio/Catalog/Workbench、Docs islands、真实 GPU、性能 evidence 和全部 workspace 测试。[M5 作者记录](DOM-COMPONENTS-P2.records/M5.md)仍保留其基线和数值，未计作本轮通过。OS IME、VoiceOver、真机触控及 200% 浏览器 UI 缩放按计划保留 P5 missing，不能由本轮 headless Chromium 或合成事件补齐。以上边界不影响五项已复现问题的判定。

## 覆盖与验证

### 意图与版本

根据计划与提交说明，本次把 DOM 表单拆成独立可用的无样式部件，并让 Themes 共用受控交互，增加原生表单、具名 Slider、Select 与 Studio 样本。呈现路径为 DOM；GPU 新能力、发布和门户接入不在本次范围。

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；日期：2026-10-30，Asia/Shanghai。
- 输入：`$review-code docs/plan/DOM-COMPONENTS-P2.md`；采用开发计划模式、初审，由当前主 agent 单独执行。
- 计划完成记录 M1–M5 均登记 `9fd21ca86ff2dc64bfa19059a560a92c9da2b96c`，已验证可达。基线为其第一父提交 `721651e6e71906e046753dd258df5e7b527128e7`，审查累计净差异，共 218 个变更文件。
- 目标树：`9028cf1303908c93915495fc0acfd44ef5a3cd77`。验证 checkout 为 `main / 19cba0c604a81817d9a08a86e3ad80e02e17a053`，与目标之间只有本计划及其 SDLC 完成回写，没有产品源码、依赖或测试变更。因此当前构建可验证此 target 的代码。
- 排除代码 findings 的后续提交：`3099be363f698c963fd78bd91e104b309ef1d0da`（计划 SHA 回填）、`19cba0c604a81817d9a08a86e3ad80e02e17a053`（本计划 SDLC 收口）。计划/记录/旧评审和台账按契约上下文读取，不作为新增产品代码缺陷的定位。
- 工作树/index 开始时均干净，untracked 为空。审查前和写报告前，全仓 11,527 个 tracked 文件内容聚合 SHA-256 均为 `6e890cd8d3a5c6c7361a71712e11b20dd699081d24ffbdc75bc934044e25ad94`；算法为按 `git ls-files -z` 顺序逐项拼接 `path + NUL + sha256(file)二进制摘要` 再 SHA-256。本报告及其台账写入不属于审查输入。
- 所用计划为 HEAD 上 v0.2，SHA-256 `d2413f6b5effda6e597385fe1197a5de0d1e2dfb820721f8089673a37db0b95c`；相比实现提交只变更恢复快照和 SHA 回填，契约正文未变。

### 覆盖清单

| 行为 / 文件群 | 本轮核查 | 限制 |
| --- | --- | --- |
| primitives 组合、方向、Portal、choice 与 Themes 包装 | 公共接口、owner/context、事件取消、受控回正、匿名组、required 与注册清理 | 浏览器操作以独立 raw fixture 为主；没有完整重评 Themes 视觉矩阵 |
| slider/model/browser 与便捷投影 | 身份/排序/间隔、接受值与请求值、指针捕获、键盘结束/取消、隐藏表单值 | 发现 F-03；父层 Escape 的语义单列 U-01 |
| field/Form/Draft 与 FieldBinding | composition、accepted mirror、submit/reset、错误来源、原生约束和 ID 关联 | 发现 F-01、F-04、F-05；真实 OS IME 未测 |
| Select 与 overlay | 元数据索引、活动项、typeahead、原生镜像、取消、聚焦、定位和 anchor 清理 | 发现 F-02；没有把所有旧浮层当成一次全仓审计 |
| Studio 样本、直接调用迁移、Docs 样式接线 | 阅读改动及必要调用方、核对公共能力和范围 | 既有页面全量与三模式结果仅引用作者记录，不冒充本轮复验 |
| CSS、CATALOG、来源锁与 vendor | 实现皮肤/结构层阅读；生成漂移、归档 include 和来源指纹检查 | 第三方 React 参考源码未作独立逐行正确性审计；冻结 benchmark/JSON 不逐条重测 |
| 验证设施 | 对照新增纯逻辑/SSR/浏览器用例和 fixture 的观察入口，检查对旧选择器的迁移 | 没有因已有测试全绿而排除其未覆盖的组合 |

### 本轮实际执行

| 命令 / 实验 | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr` | 31 passed |
| `mbx test --locked -p rustify-components --lib primitives::` | 27 passed |
| `mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml` | 成功，随后浏览器使用此新构建 |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives primitives.spec.ts` | 38 passed，20.8s，单 project / 单次运行；用例的 console/pageerror/失败请求断言通过 |
| `mbx clippy --locked -p rustify-components -p rustify-dom --all-targets -- -D warnings` | 通过 |
| `mbx xtask sources verify` | exit 0；新增 `radix-themes-forms` 29 文件、`radix-primitives-forms` 26 文件均 0 modified；既有 Makepad drift 非本轮改动 |
| `mbx xtask css --check` | no drift，94,986 B，4,812 SDK candidates |
| `mbx xtask catalog --write docs/components.md --check` | no drift，105 public entries / 97 stable reference names / 24 legacy categories |
| `git diff --no-ext-diff --no-textconv --check` | 通过；写报告前产品输入指纹未变 |
| 原页面的 Field/Select 标签关联探针 | 直接确认 F-04，无需修改 fixture |
| 隔离消费者的动态 Input、小数 Slider、聚焦 Draft reset、缺 Root Select 部件 | 分别复现 F-05、F-03、F-01、F-02；未故意触发 panic 的交互探针无意外 console/pageerror/失败请求；F-02 单独触发并捕获预期的 WASM panic |

隔离复现目录为本机临时目录 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p2-review-i9nbuzou/`。其 `probe` 复制当前独立 primitives 消费者，只在副本增加公开 API 场景；产品 crate 仍指向本仓未改的 target 源码。最终锁从原 `tests/dom-only/Cargo.lock` 离线解析，仅删除无关 workspace/GPU 包，没有增加包或更换版本；早期重新生成锁的候选未用于最终裁定。构建使用 `CARGO_TARGET_DIR=<仓根>/tests/dom-only/target mbx xtask build-dom --manifest-path <临时目录>/probe/Cargo.toml --output <临时目录>/site`；之后由本仓 `serve --dir` 施加 Strict CSP，在 Chromium `153.0.8010.12` 串行操作。临时源码、`probe.cjs`、`probes.json` 和 `missing-root.json` 可供本机核对；它们不是仓内交付或跨机器证据，本报告已保存复现输入、步骤、实际结果和源码定位。所有本轮自启预览已停止。

构建成功仍有 `fallback_element` / `guest_click` 两项原有 wasm dead_code warning；[M1 原记录](DOM-COMPONENTS-P2.records/M1.md#未完项与继续动作)亦记录相同 warning，本轮未作为新增 finding。

## 计划符合性

| 本期承诺 | 当前核查 |
| --- | --- |
| WORK-F1/AC-2、F5 的无样式组合/方向/辅助入口 | 独立 raw 产品可构建，相关浏览器和纯逻辑测试通过；缺 Root 的降级仍有 F-02 |
| WORK-F6/AC-1—AC-3 | 选择/组/卡片确实复用同一行为；现有 SSR/raw 测试通过，本轮未确认新增缺陷 |
| WORK-F7/AC-1—AC-2 | 单/多值共用身份与手势；合法小数最小间隔被 F-03 阻断，嵌套 Escape 优先级留 U-01 |
| WORK-F8/AC-1—AC-3 | FormState 错误来源和原生提交桥存在；reset、标签关联和动态编辑器有 F-01/F-04/F-05，不能以 M5 的“通过”记录替代这些失败场景 |
| WORK-F9/AC-1—AC-2 | 已有 Select 正常、取消、空/动态/长列表与资源回收测试通过；缺上下文失败属于 F-02 |
| WORK-F15/AC-2、F19/AC-1、N1/AC-1 | 样本和视觉 props 已接线，CSS/catalog 无漂移；本轮没有独立 UI 视觉验收结论，F-04 直接影响名称关联 |
| WORK-N4/AC-1、V-10/V-11 | 本轮验证 host SSR 与 raw CSR；hydrate、Docs islands 和性能数值保留 M5 作者证据及其限制，没有重跑或扩大到 P5 终验 |

未以 P3–P5 尚未实现的能力报缺陷，也未要求新增 GPU 对等组件。

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`。确认 5 项 P2、0 项 P0/P1/P3；均有本次 diff 锚点和真实可达复现。现有 96 项测试通过，但没有覆盖上述组合；不能据此通过本期代码验收。U-01 是语义待明确项，未充作第六条 finding。

建议先处理旧 Draft 写回与缺 Root 的 abort（F-01/F-02），再修复 Field/Input 的注册与身份生命周期（F-04/F-05），随后修复 Slider 间隔计算（F-03）。关闭条件是各条列出的复现不再失败，原 raw/SSR 回归仍通过，修复影响到的 hydrate/直接消费者有对应版本证据，再进行独立复审。

下一步：`$apply-code-review docs/plan/DOM-COMPONENTS-P2.code-review.md`；修复后使用 `$review-code docs/plan/DOM-COMPONENTS-P2.md` 复审。本轮不自动开始修复、UI 验收或 PRD 覆盖核查。
