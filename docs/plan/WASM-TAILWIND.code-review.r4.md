# WASM-TAILWIND · 代码评审 R4

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R4，修复效果复审 | [WASM-TAILWIND](WASM-TAILWIND.md)，sha256 `3edd269f5bc815990a232e2cda688969a12bd2d64cab363bc726fe36f8e52e48` | dirty@`53652fcc405a2679df19eb10e252efd4a4236f50`；输入集合指纹 `9a6946c6d92b9663a110a1aecc104205bf833b12bd3bc5d32ae35f3632372143`；R3 F-04–F-07 及 scope 修复直接影响的回归 | [R3](WASM-TAILWIND.code-review.r3.md)，sha256 `61515317fe9ef1f5593e9f3d7133fd172951ae3795edac6d3c17204a21ae7ae2`；对象 ID `e4b93990-4ec9-41be-826f-4be4af5fc632` |

评审日期：2026-10-09（Asia/Shanghai）。本轮对象 ID：`c7223620-4496-4573-abf0-96667ca05014`。用户输入 `$review-code docs/plan/WASM-TAILWIND.md`；按开发计划模式，以最大轮次 R3 做**修复效果复审**，不是 M1–M7 完整重审。已有处置：[M6 的 R3 修复](../validation/wasm-tailwind/M6.md#评审修复代码评审-r3)。

R3 与当前 HEAD 相同，成果仍未提交；计划没有登记实现/修复提交范围。R3 保存了源码全文，本轮可以恢复精确增量：[scope patch](../validation/wasm-tailwind/code-review-r4/scope-r3-to-r4.diff)、[浏览器测试 patch](../validation/wasm-tailwind/code-review-r4/runtime-tests-r3-to-r4.diff)。scope 从 `7a8bcaf626ac6a89745039d02de0f617917a2a788f8e744bf2588fb91e24a0d4` → `d13fd89a791e24da67947a0e3d8a2f5d0e3497ad0e74e2edd7a4816c222eb14e`；测试从 `f91eddcbd2c31a7ef3c602bef9b2e5a0caab7ff51a5b360cc3bc506911c1b7c5` → `b6636fcaaf0759e96c42eb775f7dbd75f9d8626401e5585ee1c168d5bda3929e`。新增 finding 归因于此修复增量，不沿用 R3 的 HEAD → 新增文件归因限制。

## 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R3 / F-04，沿用 R2 / F-04，P2 | 同一变量用于 timing 和名称时全局改名，R3 判部分解决 | M6 按用途生成变量版本，首个用途改写保留在公开声明；当前 scope hash 如上 | **部分解决** | 原 `fixed`、`shared-role` 均恢复正确动画；但 `public-inline-consumer` 仍把公开 `--v: linear` 改成名称，外部普通用途失效，见 F-04 | 不以首个动画用途决定公开 token 的值；覆盖编译输入之外的普通消费者 |
| R3 / F-05，P2 | var 代换没有推进整条 shorthand 的槽位，名称漏改 | `References::process/reference/resolve` 按声明和 fallback 展开，并推进状态集合 | **已解决** | 独立 SDK 探针 `var-before`、`var-name`、`var-fallback` 与原始 CSS 的名称、timing、duration、iteration、活动数及末帧一致；host 与 root/deep 新增代换用例通过 | 保持 alias/fallback/两 owner 回归；新副本覆盖问题另列 F-08 |
| R3 / F-06，P2 | 数学时长函数被当作 easing | `math/calculation/function_slot` 区分结果量纲及 easing 函数 | **已解决** | `time-calc`、`time-min` 恢复本 owner 的 card，duration `10s`、timing `linear`、活动数 1；新增 host 与浏览器用例通过 | 无原问题剩余动作；不扩称已覆盖所有 CSS 数学函数 |
| R3 / F-07，P2 | 无单位数值没有占 iteration 槽 | `numeric/step` 区分 number/time，数值占 iteration | **已解决** | `iteration` 恢复本 owner 的 infinite，iteration `2`、活动数 1；整数/小数等 host 用例与 root/deep 浏览器用例通过 | 无原问题剩余动作；`iteration-before-name` 原始输入也无效，排除为失败证据 |

R1 已关闭项继续保留历史结论：F-01 的非 inline theme、alias、fallback、两 owner 和 F-02 的 rebuild 活动集合回归在 root/deep 重跑通过；未重跑其全部故障实验。F-03 的 CI 源码未发生本轮变化，本轮未复核远端 Actions 或重复 oracle 安装实验；不改写 [R2 的关闭结论](WASM-TAILWIND.code-review.r2.md#上轮问题复审)。

## 确认问题

### F-04 · [P2] 保留公开 token 的原值供普通消费者使用（沿用 R2/R3，部分解决）

- 定位：[scope.rs:673](../../crates/rustify-style/src/scope.rs:673)，当前第 673 行；支持位置第 645–656 行及 `References::word` 的 Name 分支。旧问题复核，不作为新增 finding。
- 问题：当输入为 `:root { --v: linear; }`、`@keyframes linear`、`.motion { animation-name: var(--v); animation-duration: 10s; }`，并有应用通过 inline style 使用 `transition-timing-function: var(--v)` 时，首个动画用途仍将公开声明改成 `--v: rustify-<owner>-linear`。该普通消费者没有进入编译器的用途扫描，读到的是名称而非 easing，实际 transition timing 从合法原始 CSS 的 `linear` 变成初始值 `ease`。本轮确实修复了同一输入样式表内的混合用途，却仍在公开变量上施加一个覆盖未知消费者的名称改写；这保留了 F-04 的语义污染根因，未满足原报告及计划的公开 token 保留要求。
- 证据：[root 对照](../validation/wasm-tailwind/code-review-r4/cascade-sdk-browser.json) 与 [deep 对照](../validation/wasm-tailwind/code-review-r4/cascade-sdk-deep-browser.json) 的 `public-inline-consumer`：raw 为 `linear`，R3、R4 原生 scope 输出和当前真实 SDK 均为 `ease`，公开 `--v` 均被改成 owner 名称。R3 原 `shared-role` 的关闭证据单独保留于 [animation-probe.json](../validation/wasm-tailwind/code-review-r4/animation-probe.json)，不把原例通过写成整个根因关闭。计划 [§5.4 第 6 步](WASM-TAILWIND.md:310) 仍要求“保留应用公开 theme tokens”。自定义属性可被 inline style 与 CSSOM 使用，不限于同一个编译输入。[W3C CSS Variables §2](https://www.w3.org/TR/css-variables-1/#defining-variables)
- 修复与验证：公开 token 保留普通值，名称变换由私有用途承担；不能以输入内没发现普通引用推断外部没有消费者。新增真实浏览器断言：动画仍绑定本 owner，公开 `--v` 与普通消费者保持 `linear`；同时覆盖普通规则出现在动画规则前后两种顺序，并保留现有混合用途回归。
- 置信度：高。当前源码、R3/R4 原生输出及两个真实 SDK 入口的结果一致。

### F-08 · [P2] 让变量副本用途保留公开变量的动态覆盖（本轮新增）

- 定位：[scope.rs:478](../../crates/rustify-style/src/scope.rs:478)，当前第 478–480 行，与精确 R3 → R4 diff 重叠；支持位置第 674–680 行的副本声明生成。
- 问题：`--v: linear` 同时供 `.motion` 的 `animation: card 10s var(--v) infinite` 和 `.other` 的 `animation-name: var(--v)` 使用时，新实现把后者改为读取私有副本，并从输入声明复制一个静态值。应用随后执行 `other.style.setProperty('--v', 'none')`，公开变量的 computed value 已是 `none`，但副本仍为 owner 的 linear，`.other` 继续有 1 个活动动画；原始 CSS 和 R3 的同一元素都变成 `animationName=none`、活动数 0。普通用途也受影响：将 `multi-inherit` 的公开变量覆盖为 `ease-in` 后，R3 与原始 CSS 的 transition timing 为 `ease-in`，R4 与 SDK 却仍为副本的 `linear`。复制同选择器、条件和 important 只能保持输入内声明的级联，不能让另一属性跟随外部对公开属性的覆盖；这会使运行时主题或禁用动画操作失效。
- 证据：[静态 R3/R4 浏览器对照](../validation/wasm-tailwind/code-review-r4/cascade-native-browser.json)、[root SDK](../validation/wasm-tailwind/code-review-r4/cascade-sdk-browser.json)、[deep SDK](../validation/wasm-tailwind/code-review-r4/cascade-sdk-deep-browser.json) 的 `external-disable-name` 与 `multi-inherit`；两组分别记录覆盖前后 computed value、引用副本的输出 CSS 和真实效果。只比较 R3 的 `.other`/transition 消费者，不用其已知失效的主动画作正确基线。M6 的 R3 修复“未完项”已承认副本用途不跟随外部覆盖，但计划 [§5.4](WASM-TAILWIND.md:310) 仍保留公开 token，并在运行时主题变化段明确保留 CSS 配置 epoch；没有相应契约收缩或用户豁免。`none` 应取消动画。[W3C CSS Animations §3.2](https://www.w3.org/TR/css-animations-1/#animation-name)
- 修复与验证：让副本用途与公开 token 的运行时值保持约定关系；不能把公开覆盖转换为对一个固定快照无效的操作。若某种变换无法保留语义，应在提交前明确拒绝该不支持输入，避免 ensure 成功后静默安装错误效果。回归外部 inline/CSSOM 覆盖、根上继承的覆盖、alias 和两个 owner，至少断言 `none` 真正停止目标动画、`ease-in` 真正更新普通用途，并保持配置失败保留旧 revision 的测试。
- 置信度：高。精确修复前源码对照、当前原生输出和当前 release Worker/SDK 的两个入口都复现；不用动态外部关键帧名称的猜测作为证据。

F-04 是公开声明本身仍被改写；F-08 是新增副本切断公开覆盖的传播。两者分别保留旧 ID 和新增 ID，不能仅靠调整首个用途顺序同时关闭。

## 待核实项与覆盖限制

| R3 待核实 / 未覆盖项 | 本轮处理与剩余范围 |
| --- | --- |
| 精确修复 patch | 本轮已恢复 R3 → R4 的 scope 与测试增量；R2 → R3 缺失的历史源码未补造 |
| 完整 scanner 浏览器 | 本轮只构建 compiler-only root/deep，并运行 facade compiler+scanner host 测试；M6 记录的 scanner 浏览器结果是修复作者的历史证据 |
| 远端 CI、完整上游/cn/旧示例/workspace/fmt/clippy/来源检查 | 本轮未重新执行；不以修复记录或文件 hash 替代本轮运行结果 |
| V-11 性能 | M6 明确修复后未复测；本轮也未压测新增展开/副本变换，不作性能判断 |
| 其他 dirty 内容 | Theme Studio、其他 SDLC 对象、skills 和未受此修复影响的生产代码不纳入本轮深入评审 |

探索性探针共 13 组，不将它们全部计为通过：`none-name` 使用非法裸 `@keyframes none`，排除为有效反例；`default-registered` 的 R3/R4 输出相同，其现象不是本轮新增，未据此扩大 findings。变量循环/fallback 与继承对照用于反证，详细结果保留于证据文件。以上限制不影响 F-04/F-08 已复现的判定。

## 覆盖与验证

变更意图来自计划与 M6：以 var 代换后的槽位判定名称，为不同用途生成版本，修复 F-04–F-07 并保留 scope、参数与公开 token 行为。

- 仓根 `/Users/rockie/Documents/gh-rockie/rustify-ui`；branch `main`；HEAD 如报告头。未刷新远端引用，未提交、推送或部署。
- 比较范围是 R3 快照 → 当前 dirty 工作树。index 无差异；[输入快照](../validation/wasm-tailwind/code-review-r4/input-snapshot.json) 固定 4,851 项：R3 输入集合加当前 dirty/untracked 与适用规则，按路径排序后对 files 数组的 canonical JSON 做 SHA-256（`ensure_ascii=False, sort_keys=True, separators=(',', ':')`）。数量是来源集合，不代表逐行审查覆盖数。
- 相对 R3，11 个既有输入变化、41 个新输入；生产/测试源变化只有 `scope.rs` 与 `utility-runtime.spec.ts`。另核对 [计划增量](../validation/wasm-tailwind/code-review-r4/plan-r3-to-r4.diff) 和 M6/M7 处置记录。当前源码/测试/计划的全文保存在本轮 `source-snapshot/`，旧全文保留 R3 原位。
- [输入核验](../validation/wasm-tailwind/code-review-r4/input-verification.json)：测试/构建后、写报告与台账前，4,851 项无漂移，HEAD 与指纹一致。本轮新增证据/报告不属于审查输入；之后只维护本轮报告/证据及协议要求的台账，原计划和产品代码保持原 hash。
- 静态阅读完整当前 scope 实现和修复 diff、对应测试及必要上下游：facade prepare → Worker → SDK 安装 → utility-lab；检查状态推进、变量版本、alias/fallback/cycle、内部注册复制、错误/限额和事务边界。未完整重审其他不变模块或所有 CSS 语法。
- 单 agent，使用 review-code 与 rust 技能；没有委派。无页面或产品实现改动，本轮浏览器验收聚焦实际动画/变量/scope/配置行为，没有完整视觉、GPU 图像或性能验收。项目不是 Portal App，不涉及 onebox。

| 本轮实际命令 / 步骤 | 结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-style --all-features --lib` | **532 passed，0 failed，12 ignored**；[log](../validation/wasm-tailwind/code-review-r4/style-tests.log)，ignored 不计通过 |
| `mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib` | **12 passed，0 failed**；[log](../validation/wasm-tailwind/code-review-r4/facade-tests.log) |
| `mbx xtask build-web --example utility-lab --release`；随后同入口加 `--base /tools/utility-lab/` | 两份构建成功；[root](../validation/wasm-tailwind/code-review-r4/build-root.log)、[deep](../validation/wasm-tailwind/code-review-r4/build-deep.log) |
| root/deep 的 7 项相关 runtime 用例，命令见下 | **root 7 / deep 7 passed**；[root](../validation/wasm-tailwind/code-review-r4/browser-root.log)、[deep](../validation/wasm-tailwind/code-review-r4/browser-deep.log)。覆盖 namespace/alias/fallback/两 owner、代换/math/iteration、rebuild、嵌套边界/销毁、关系 selector 和配置成功/失败事务 |
| strict root serve；复用 R3 的 `animation-probe.mjs`，输出到 R4 | 9 组均与原始 CSS 对照一致（8 个合法输入及 1 个两侧均无效的负对照），无 page/console error、失败请求、CSP violation；[JSON](../validation/wasm-tailwind/code-review-r4/animation-probe.json) |
| 隔离 native 工程编译 R3 快照与当前 scope；同一批输入在 Chrome 安装 raw/R3/R4 CSS | 确认 F-04 旧根因残留和 F-08 新增回归；[输入](../validation/wasm-tailwind/code-review-r4/candidate-cases.json)、[原生输出](../validation/wasm-tailwind/code-review-r4/native-probe.json)、[Chrome](../validation/wasm-tailwind/code-review-r4/cascade-native-browser.json)。归档工程使用独立锁文件和已有缓存，`--locked --offline` 重放结果相同，不充当主工作区依赖验收 |
| 同一探针走当前真实 SDK/Worker，root 与 deep | 两入口均复现 F-04/F-08；page/console error、失败请求为空；deep 有 CSP 监听且结果为空，root cascade 未监听 CSP。见 [root](../validation/wasm-tailwind/code-review-r4/cascade-sdk-browser.json)、[deep](../validation/wasm-tailwind/code-review-r4/cascade-sdk-deep-browser.json)、[摘要](../validation/wasm-tailwind/code-review-r4/verification-summary.json) |
| W3C Variables §2 与 Animations §3.2 | 实际读取官方规范，支持外部自定义属性/覆盖与 `none` 的语义；故障判定以当前源码和浏览器为主 |

Chrome **154.0.8037.98**。常规入口为 `http://127.0.0.1:4186/` 与 `http://127.0.0.1:4187/tools/utility-lab/`；独立 strict probe 入口为 `http://127.0.0.1:53793/` 与 `http://127.0.0.1:54989/tools/utility-lab/`。本轮 main build ID `9926118ec160f77a`、compiler-only Worker `85684fa83661cc07ece63c26504f47b52dc061fca89401129d5ea28664fcc1f7`，root/deep 相同；[root manifest](../validation/wasm-tailwind/code-review-r4/worker-manifest-root.json)、[deep manifest](../validation/wasm-tailwind/code-review-r4/worker-manifest-deep.json)。仅停止本轮自行启动的 serve。

串行浏览器命令（实际临时目录见 [metadata](../validation/wasm-tailwind/code-review-r4/review-metadata.json)）：

```sh
RUSTIFY_XTASK=target/debug/xtask npx playwright test --project=utility-lab --workers=1 --reporter=line --output=<临时目录>/browser-root utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|keyframes names keep|rebuild releases|nested owners|scoped relation|configuration replacement'
RUSTIFY_XTASK=target/debug/xtask npx playwright test --project=utility-lab-deep --workers=1 --reporter=line --output=<临时目录>/browser-deep utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|keyframes names keep|rebuild releases|nested owners|scoped relation|configuration replacement'
```

完整对照工程、锁文件、脚本与复现步骤见 [证据说明](../validation/wasm-tailwind/code-review-r4/README.md)。动态覆盖只使用合法 `none`/`ease-in`，不依赖编译器猜测外部关键帧名；无需为复现安装服务或下载依赖。

## 计划符合性

| 相关契约 / 任务 | 当前实现与证据 | 判定 |
| --- | --- | --- |
| §5.4 第 6 步、V-6：名称/内部引用隔离、公开 theme tokens 保留 | 原 R3 动画反例恢复；公开 token 仍按首个用途被改写，见 F-04 | **部分满足** |
| §5.4 运行时主题值变化：保持 CSS 配置 epoch | 输入内副本声明覆盖随原规则复制；输入之外对公开 token 的覆盖不传播，见 F-08 | **存在回归**；M6 的限制记录不等于批准收缩契约 |
| scope/配置事务/rebuild 必要回归 | root/deep 各 7 项通过，host style/facade 通过 | 本轮所测范围通过，不抵消独立反例 |
| 其他 M1–M7 / V-1–V-11 | 按上述范围保留历史证据，未完整重跑 | **本轮未完整复核** |

计划没有独立 PRD 需求来源；不自动启动 PRD 差异核查，也不更新计划进度、readiness 或实施状态。

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`，限于精确 R3 → R4 的 scope 修复增量及直接影响的行为。修复效果：**未完全解决**。旧项 F-05/F-06/F-07 已解决（3 项），F-04 部分解决（1 项），新增 **1 个 P2：F-08**。532+12 host 和 7+7 浏览器回归通过；公开 token 的独立 Chrome 反例仍成立。

先明确并落实公开 token 原值与动态覆盖的保留机制，关闭 F-04/F-08；不能只靠改变首个用途顺序或在记录中追加限制。关闭条件：公开变量的普通消费者保持原语义，外部 `none` 覆盖停止目标动画、`ease-in` 覆盖更新普通用途，root/deep、alias/两个 owner 与现有配置回滚测试继续通过。性能、scanner 浏览器及远端 CI 按独立覆盖范围补证，不将本次局部复审当成发布授权。

下一步：`$apply-code-review docs/plan/WASM-TAILWIND.code-review.r4.md`。修复后：`$review-code docs/plan/WASM-TAILWIND.md`。本轮不自动修复或改写历史结论。
