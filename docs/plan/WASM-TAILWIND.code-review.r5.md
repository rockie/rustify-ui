# WASM-TAILWIND · 代码评审 R5

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R5，修复效果复审 | [WASM-TAILWIND](WASM-TAILWIND.md)，sha256 `4c258902598f42d20b2bca27d99c05ea708690185666606e6b6705020d304ead` | dirty@`53652fcc405a2679df19eb10e252efd4a4236f50`；输入指纹 `b7a0c5308c2d932e46bb3e2bc422c5b775bbafb0e00a05e0c3d3b5c8e51d33bb`；R4 F-04/F-08 及修复直接影响的回归 | [R4](WASM-TAILWIND.code-review.r4.md)，sha256 `ed08909879bd2682fae2f8486017701433d726173dfa78776816b4f8728e2776`；对象 ID `c7223620-4496-4573-abf0-96667ca05014` |

评审日期：2026-10-09（Asia/Shanghai）。本轮对象 ID：`03a93f04-5364-4e76-ab5c-f47fe6fa8851`。原输入 `$review-code docs/plan/WASM-TAILWIND.md`；采用开发计划模式，以最大轮次 R4 为上轮，执行**修复效果复审**，未完整重审 M1–M7。处置来源：[M6 的 R4 修复记录](../validation/wasm-tailwind/M6.md#评审修复代码评审-r4)、[R4 处置事件](../../.xgent-ai/sdlc/events/review/c7223620-4496-4573-abf0-96667ca05014/31c4a708-d747-49b2-9153-a8961fba7e73.md)。

上轮和当前 HEAD 相同，计划没有登记实现/修复提交列，成果仍在未提交工作树。R4 保存了关键源码全文，本轮恢复了精确 [scope 增量](../validation/wasm-tailwind/code-review-r5/scope.rs.diff)、[测试增量](../validation/wasm-tailwind/code-review-r5/utility-runtime.spec.ts.diff) 与 [计划增量](../validation/wasm-tailwind/code-review-r5/WASM-TAILWIND.md.diff)。scope 从 sha256 `d13fd89a791e24da67947a0e3d8a2f5d0e3497ad0e74e2edd7a4816c222eb14e` → `1325b0479f95a1e5c9199ccde4f7a425a9d57f71161b2860210af7823fa302b3`；测试从 `b6636fcaaf0759e96c42eb775f7dbd75f9d8626401e5585ee1c168d5bda3929e` → `f9d1e63726e0756b5aef681ca30721046ef1b9b91d617732b661de3f2863b492`。

## 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R4 / F-04，沿用 R2/R3 / F-04，P2 | 公开 token 被改成关键帧名，外部普通消费者语义改变；部分解决 | M6 记录用户选择收窄契约；当前计划 §5.4 第 6 步及开发者文档明确公开名称保留、关键帧值改名、外部普通读取看到改名值 | **不再适用**，依据契约变化；不记为原行为已修复 | [scope.rs:671](../../crates/rustify-style/src/scope.rs:671)、[计划:310](WASM-TAILWIND.md:310)、[文档:143](../utility.md:143)；隔离 R4/R5 对照中 `public-inline-consumer` CSS 完全相同；root/deep 真实 SDK 中公开值仍为 owner 名，外部 transition timing 仍为 `ease`，内联动画绑定本 owner | 原“普通值必须不变”的关闭要求已退出当前契约；保持这项边界说明，不把它写成恢复原语义 |
| R4 / F-08，P2 | 私有副本切断对公开 token 的外部覆盖；本轮新增于 R4 | scope `plan()` 拒绝非 `--tw-*` 的多版本用途，配置失败保留旧 revision；单一用途直接读公开变量 | **已解决** | [scope.rs:674](../../crates/rustify-style/src/scope.rs:674)、[测试:933](../../tests/browser/utility-runtime.spec.ts:933)；5 个 R4 公开冲突输入从接受变为 `UnsupportedFeature`；两 SDK 入口均拒绝替换且保持 CSS/revision/活动动画；支持输入的元素、根、alias、内联动画 `none` 覆盖实际停止目标动画 | 无本问题剩余动作；解决方式是提交前拒绝冲突，不是使静态副本跟随公开覆盖 |
| R4 / F-05，沿用 R3 / F-05，P2 | var 代换未推进 shorthand 槽位；R4 已解决 | 相关生产逻辑没有本轮变化 | **已解决**，关闭结论继续有效 | style 单测及 root/deep 的 var-before、var-name、fallback/alias/两 owner 回归通过；当前 scope 全文与增量均已阅读 | 无 |
| R4 / F-06，沿用 R3 / F-06，P2 | 数学时长占错 easing 槽；R4 已解决 | math/function_slot 没有本轮变化 | **已解决**，关闭结论继续有效 | style 单测及 root/deep 的 `calc(10s)`、`min(10s, 12s)` 浏览器对照通过 | 无；不扩大为全部 CSS 数学函数覆盖 |
| R4 / F-07，沿用 R3 / F-07，P2 | 无单位数值未占 iteration 槽；R4 已解决 | numeric/step 没有本轮变化 | **已解决**，关闭结论继续有效 | style 单测及 root/deep 整数、小数 iteration 浏览器对照通过 | 无 |

R1 F-01 的 namespace/alias/fallback/两 owner、F-02 的 rebuild 直接回归也通过。F-03 的 CI 源码未变化，本轮未重跑远端 Actions 或 oracle 安装实验，保留历史关闭结论，不声称重新验证。旧报告及其原判定保留。

## 确认问题

本轮未发现可确认的新增问题，没有未解决的 P0–P2 项。F-04 的行为仍存在，但按当前明确契约列为“不再适用”；没有将契约调整伪装成技术修复或追加用户豁免。

## 待核实项与覆盖限制

| R4 待核实 / 未覆盖项 | 本轮处理与剩余范围 |
| --- | --- |
| 精确修复 patch | 已恢复 R4 → 当前 scope、测试及计划增量；`docs/utility.md` 只有 R4 hash、没有该轮全文，读取了当前完整相关契约，未声称生成其精确文档 diff |
| scanner 浏览器 | facade compiler+scanner host 12 项通过；本轮构建 compiler-only，未重跑 scanner 浏览器，不把 R3 的 5+5 记为本轮结果 |
| 远端 CI、完整 upstream/cn/旧示例/workspace/来源检查 | 未重新执行；fmt 和本轮相关 Clippy、style/facade 检查实际通过；历史记录不替代本轮执行 |
| V-11 性能、GPU 图像与完整视觉验收 | 未压测冲突拒绝或完整重跑 GPU/视觉；本轮浏览器验证动画、变量、配置事务和 scope。生产增量仅新增公开变量版本数判断，不作性能数字或完整 UI 验收声明 |
| 其他 dirty 内容 | 相对 R4 的生产/测试源变化只有 scope.rs 和 utility-runtime.spec.ts。Theme Studio、其他状态对象、skills 及不变模块不做全仓重审 |

M6 的用户裁定以仓内处置记录及同步的当前契约为依据；本轮会话未重新取得或补造该历史用户原话。F-04 在本轮的处理是有明确契约依据的“不再适用”，不是本轮用户豁免。没有待核实项阻止判断当前限定修复范围；上述独立范围仍保留覆盖限制。

## 覆盖与验证

变更意图来自当前计划和 M6：公开 token 的关键帧值继续按 owner 改名，对需要不同改写的公开用途提交前报错，副本仅保留给内部变量，保留单一用途的外部覆盖和配置失败回滚。

- 仓根 `/Users/rockie/Documents/gh-rockie/rustify-ui`；main；HEAD 如报告头。未刷新远端引用、提交、推送或部署。
- 基线为 R4 保存的工作树源码，不是 HEAD 的旧实现；当前 tracked/untracked 净变更单列于 [输入快照](../validation/wasm-tailwind/code-review-r5/input-snapshot.json)。index 无差异，没有选择未登记的历史提交或把整段 HEAD 差异冒充本次修复。
- 输入集合为 R4 的 paths，加本轮开始时 dirty/untracked 及上下文文件；排序后的 files 数组 canonical JSON（`ensure_ascii=False, sort_keys=True, separators=(',', ':')`）做 SHA-256。共 **4,910 项**，相对 R4 **9 项变化、59 项新增**；这是来源集合，非逐行覆盖数。新增项为评审/修复证据与台账，本轮 R5 产物排除于输入。
- [输入核验](../validation/wasm-tailwind/code-review-r5/input-verification.json)：测试、构建及浏览器验证后，写报告/台账前，HEAD 相同，4,910 项无漂移。当前关键源码/测试/契约全文见 [快照说明](../validation/wasm-tailwind/code-review-r5/README.md)。其他依赖只保存指纹，重放时须核对工作树，不能仅凭 HEAD 复现 dirty 目标。
- 完整阅读 scope 源码与修复 diff、测试增量、相关契约；追踪 resolver 版本收集 → `plan()` 拒绝 → facade initial/prepare/replacement → Worker 错误 → SDK install/commit 边界。确认失败发生在安装之前，普通变量同一改写不生成副本，内部变量复制逻辑未改。
- 单 agent，使用 review-code 与 rust；无委派。没有产品界面实现改动；浏览器以真实 release SDK、Worker 和 Chrome 验证。项目不是 Portal App，不涉及 onebox。

| 本轮实际命令 / 步骤 | 实际结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-style --all-features --lib` | **533 passed、0 failed、12 ignored**；[log](../validation/wasm-tailwind/code-review-r5/style-tests.log)，ignored 不计通过 |
| `mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib` | **12 passed、0 failed**；[log](../validation/wasm-tailwind/code-review-r5/facade-tests.log) |
| `mbx clippy --locked -p rustify-style --all-features --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 两项通过；[Clippy](../validation/wasm-tailwind/code-review-r5/clippy.log)、[fmt](../validation/wasm-tailwind/code-review-r5/fmt.log) |
| `mbx xtask build-web --example utility-lab --release`；同入口加 `--base /tools/utility-lab/` | 两份构建成功；[root](../validation/wasm-tailwind/code-review-r5/build-root.log)、[deep](../validation/wasm-tailwind/code-review-r5/build-deep.log) |
| root/deep 相关 runtime 用例，完整命令见下 | **root 8 / deep 8 passed**；[root](../validation/wasm-tailwind/code-review-r5/browser-root.log)、[deep](../validation/wasm-tailwind/code-review-r5/browser-deep.log)。namespace/alias/fallback/两 owner、数学/iteration、公开覆盖/冲突、配置失败、关系 selector、嵌套/销毁及 rebuild |
| 隔离 native 工程，`mbx run --locked --offline --manifest-path <临时目录>/native-project/Cargo.toml -- <临时目录>/candidate-cases.json`，target 也放临时目录 | 成功；8 个合法输入：5 个 R4 接受的公开冲突输入在当前版本拒绝，3 个支持输入输出相同。含 F-08 原 external-disable-name/multi-inherit、alias，以及 F-04 原 public-inline-consumer；[输入](../validation/wasm-tailwind/code-review-r5/candidate-cases.json)、[对照](../validation/wasm-tailwind/code-review-r5/native-probe.json)、[构建](../validation/wasm-tailwind/code-review-r5/native-build.log)。旧 scope hash 与 R4 全文一致；独立锁文件不充当主工作区依赖验收 |
| `node <临时目录>/sdk-probe.mjs <临时目录>`，脚本启动 strict port 0 serve，root/deep 串行 | 两入口全部断言通过；[root SDK](../validation/wasm-tailwind/code-review-r5/sdk-root.json)、[deep SDK](../validation/wasm-tailwind/code-review-r5/sdk-deep.json)、[log](../validation/wasm-tailwind/code-review-r5/sdk-probe.log)。普通外部读值保留新契约行为；类规则、alias 与内联动画绑定本 owner；元素 `none` 只停目标动画，根上 `none` 停止全部相关动画；5 个冲突替换全部报 compile/unsupported_feature，CSS/revision 不变且旧动画继续。两入口 page/console errors、失败请求、HTTP ≥400、CSP violations 均为空 |

浏览器为 **Chrome 154.0.8037.98**。Playwright 入口 `http://127.0.0.1:4186/`、`http://127.0.0.1:4187/tools/utility-lab/`；独立 SDK 实际 URL 在 JSON 中。main build ID `9926118ec160f77a`；compiler-only Worker build ID `a65f0b9a2a382faf69c4065ce51e7befae11bb38f91607d7cffa7d75ad9101ac`，root/deep 相同，见 [root manifest](../validation/wasm-tailwind/code-review-r5/manifest-root.json)、[deep manifest](../validation/wasm-tailwind/code-review-r5/manifest-deep.json)。只停止自己启动的 serve，没有安装服务或下载依赖。

实际串行命令（临时路径见 [metadata](../validation/wasm-tailwind/code-review-r5/review-metadata.json)）：

```sh
RUSTIFY_XTASK=target/debug/xtask npx --no-install playwright test --project=utility-lab --workers=1 --reporter=line --output=<临时目录>/browser-root utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|keyframes names keep|a public property|rebuild releases|nested owners|scoped relation|configuration replacement'
RUSTIFY_XTASK=target/debug/xtask npx --no-install playwright test --project=utility-lab-deep --workers=1 --reporter=line --output=<临时目录>/browser-deep utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|keyframes names keep|a public property|rebuild releases|nested owners|scoped relation|configuration replacement'
```

本轮不依赖外部版本资料作缺陷判定，没有新增外部规范引用；结果来自当前源码及实际 Chrome。可重放探针、锁文件及边界说明见 [证据说明](../validation/wasm-tailwind/code-review-r5/README.md)。

## 计划符合性

| 当前相关契约 / 任务 | 实现与验证 | 判定 |
| --- | --- | --- |
| §5.4 第 6 步：公开 token 保留名称，流入关键帧的值改名；普通外部读取看到改名值 | 原 public-inline-consumer 输出未变；真实 SDK 普通读取与内联动画验证；文档同步 | **当前契约满足**；原 F-04 的保留原值要求不再适用 |
| §5.4 第 6 步：公开用途需要不同改写时报 UnsupportedFeature；副本只用于内部变量 | 精确增量、host、native 5 组旧反例及两 SDK 入口拒绝验证 | **已核实** |
| §5.4 配置事务：失败保留旧配置/快照，运行时值变化不换 CSS 配置 epoch | 新 browser 用例及独立探针拒绝后保持 CSS/revision/动画；原配置事务用例通过 | **本轮相关范围已核实** |
| scope、rebuild、参数槽位回归 | root/deep 各 8 项及 host 检查通过 | **本轮相关范围已核实** |
| 其他 M1–M7 / V-1–V-11 | 保留原记录，未完整重跑 | **本轮未完整复核** |

计划顶部没有独立 PRD「需求来源」，本轮不自动启动 PRD 差异核查，不修改原计划进度、readiness 或实施状态。

## 总体结论与修复顺序

**验收判定：通过验收。**

`patch is correct`，仅覆盖精确 R4 → 当前 scope 修复增量及直接相关回归。修复效果：**已解决**；本轮须处置的两项中，F-08 实际解决 1 项，F-04 有契约变化依据而不再适用 1 项。F-05/F-06/F-07 的历史关闭结论继续成立，新增 findings 为 0。533+12 host、root/deep 8+8 浏览器、8 组 native 对照及两个真实 SDK 探针支撑此结论。

本轮修复范围代码验收完成，无需执行 `$apply-code-review`。F-04 的原语义没有恢复，公开 token 的普通读取仍看到改名值；这项已记录的边界必须随契约保留。scanner 浏览器、完整上游/旧示例、GPU/视觉、性能与远端 CI 仍按各自范围保留未重验说明，不将局部复审扩大为整份 SDK 无未知缺陷、合并或发布授权。

若后续改变公开 token 或 scope 逻辑，再运行 `$review-code docs/plan/WASM-TAILWIND.md`；本轮不自行启动修复或下一轮评审。
