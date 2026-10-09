# WASM-TAILWIND · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，初审 | [WASM-TAILWIND](WASM-TAILWIND.md)，sha256 `b8ebbc6223f9b9d13b3ea1733298be40b14664adb0107f33cd0de035a083644b` | `53652fcc405a2679df19eb10e252efd4a4236f50` → 当前工作树，含 tracked 净差异和 untracked；输入集合指纹 `a021455f93298d8f3df2c70d5d8339a565cc7a6e905c200c9c87bcd58779e25f` | 无。本计划的 `.review.md` 是计划文档评审，不作为代码复审基线 |

评审日期：2026-10-07（Australia/Sydney）。评审对象 ID：`de7db56e-c90e-4ac5-9a71-1e763e8971c4`。计划完成记录没有实现提交列，恢复快照明确成果仍未提交；采用开发计划模式审查当前实现，不纳入未经登记的历史代码提交。原计划和代码保持只读。

## 确认问题

### F-01 · [P2] 同步重写动画变量中的关键帧引用

- 定位：[scope.rs](../../crates/rustify-style/src/scope.rs:118)，目标工作树第 118 行。
- 问题：使用普通 `@theme { --animate-card: card 1s linear infinite; @keyframes card { ... } }` 和 `animate-card` 时，scope 变换只将 `animation` / `animation-name` 声明视为动画引用，却把关键帧统一重命名为 `rustify-<owner>-card`。主题变量中的 `card` 保持原名，生成的 `animation: var(--animate-card)` 因而引用不到本 scope 的关键帧。浏览器没有其他同名全局关键帧时动画完全不运行；存在同名全局关键帧时可能误用别处的定义。此问题由新增 scope 变换引入，编译核心的标准输出可以正常运行。
- 证据：同文件第 126–128、148–150 行；[motion.rs](../../crates/rustify-style/src/compiler/motion.rs:256) 的既有测试明确保留 `animation: var(--animate-card)`。计划 §5.4 第 6 步要求关键帧命名空间化时同步改引用。当前 release / strict CSP / Chrome 复现：[animation.json](../validation/wasm-tailwind/code-review-r1/animation.json)：scope 样本 `animationName=card`、活动动画数 **0**；普通 CSS 对照同名、活动动画数 **1**，page errors 为空。
- 修复与验证：跟踪动画声明通过自定义属性引用的值，重写相应关键帧名，保留公开 token 名。覆盖默认非 inline 的 `--animate-*`、变量别名与 fallback，以及两个 scope 的同名不同定义；验证活动动画数和实际关键帧来源。
- 置信度：高，当前代码及真实浏览器故障均已确认。

### F-02 · [P2] 提供按活动集合重建 scope 的候选回收入口

- 定位：[browser.rs](../../crates/rustify-ui/src/utility/browser.rs:502)，目标工作树第 502–505 行。
- 问题：配置替换与新 Worker 初始化都会从 `snapshot.classes` 复制全部历史候选，之后再加入 seed、当前 DOM 与组件类；`State::installed` 又把新候选与旧快照集合合并。SDK 没有接受当前活动候选集合的 rebuild 入口，因此跨 epoch 也无法释放已不用的类。长期编辑不断产生新的 arbitrary 类时，即使旧类已不在 DOM/GPU 使用，历史集合仍占用 `candidate_count`；达到配额后，`replace_config` / `retry` 无法提供计划承诺的重建恢复路径，只能增加配额或销毁整个 scope。
- 证据：[state.rs](../../crates/rustify-ui/src/utility/state.rs:579) 第 579–583、626–641 行；计划 §3 第 230–231 行要求配置 epoch 用 active classes 重建，显式 rebuild/配置替换可以重建当前 active 集合；§8 的 quota 恢复为 `rebuild(active)`。[Compiler::rebuild](../../crates/rustify-style/src/compiler/mod.rs:795) 存在于核心，但 SDK 没有将它接入 scope。浏览器 [history.json](../validation/wasm-tailwind/code-review-r1/history.json)：`w-[12345px]` 的活动 DOM 匹配数为 **0**，替换配置后仍 admitted，revision 从 2 到 3；另一 scope 的基础集合 233、配额 234，保留一条不用的类后，配置替换及新类请求均返回 `candidate_count` limit。`docs/utility.md` 的 retained 描述准确反映实现，但没有提供活动集合重建能力，不能代替计划要求。
- 修复与验证：增加显式接受活动候选（包括 GPU 类和变量依赖）的 scope 重建入口，或给配置替换提供这个输入；新 epoch 快照和预算校验使用新的完整集合。Worker 故障 retry 继续重放最后成功安装集合。回归覆盖删除旧类后不增加配额即可加入新类、重建失败保持旧 sheet/programs/cn、静态 seed 与当前有效 GPU 类不被驱逐。
- 置信度：高，集合保留路径、缺少 SDK 入口与浏览器结果一致。

### F-03 · [P2] 在新增浏览器 CI 中安装锁定的 Tailwind oracle

- 定位：[verify.yml](../../.github/workflows/verify.yml:214)，目标工作树第 214–218 行。
- 问题：新增的 `utility-lab` CI job 执行整个 `utility-runtime.spec.ts`，其首个测试通过 `createRequire(tests/utility/compiler/oracle/package.json)` 加载 `tailwindcss`。该 job 只在仓根执行 `npm ci`，而根 manifest/lock 只安装 Playwright、pngjs 及对应 Playwright 包，不包含 Tailwind；单独 oracle 的 package.json/lock 没有安装步骤，也没有随构建 artifact 交付 node_modules。干净 CI checkout 会在运行首个测试时遇到 `MODULE_NOT_FOUND`，root 和 subpath 的新增门无法通过。本地存在的额外 Tailwind 安装掩盖了这一交付缺陷。
- 证据：[utility-runtime.spec.ts](../../tests/browser/utility-runtime.spec.ts:19) 第 19–24 行；[oracle/package.json](../../tests/utility/compiler/oracle/package.json) 固定 `tailwindcss: 4.1.13`；根 [package.json](../../package.json) / [package-lock.json](../../package-lock.json) 与 workflow 第 209–210 行。隔离目录仅提供根声明依赖后，按相同 oracle 路径解析，结果见 [ci-oracle.json](../validation/wasm-tailwind/code-review-r1/ci-oracle.json)：`MODULE_NOT_FOUND: Cannot find module 'tailwindcss/package.json'`。这是本轮新增测试/job 的依赖，不将原有 job 的问题归入本轮。
- 修复与验证：在 `utility-lab` browser job 运行前，用其独立锁文件执行 `npm ci --ignore-scripts --prefix tests/utility/compiler/oracle`，或提供等效的锁定 oracle 产物。以干净 checkout、仅 workflow 声明依赖运行 root/subpath job；无需引入生产依赖。
- 置信度：高，依赖图、job 接线与隔离解析结果一致；远端 CI 未实际运行。

## 待核实项与覆盖限制

- 本轮未重跑 1,183 项固定上游执行门，也未运行全部 cn JS/native 差分、scanner-enabled 浏览器、subpath、旧示例与性能测量；M7 的这些结果属于历史证据，不记为本轮通过。F-01 属于标准 compiler 差分之外的 runtime scope 变换。
- 生产核心规模较大：按风险阅读 cn 表/缓存与组件迁移、scanner 源入口、compiler/session/解析/import/主题及相关 emitters、scope、SDK 队列/事务/恢复、GPU cascade/状态/控件接线、build/manifest/CI。未逐行重新审查全部 utility family、上游移植测试、generated 数据、原始日志、图片及实验目录；报告不声称对全部上游合法 CSS 做了独立完整认证。
- 当前工作树另有 skill 配置、SDLC 初始化和 Theme Studio 验收文件变化；已记录指纹，未将其作为本计划新增代码缺陷对象，也不重新裁定其他计划的历史问题。
- 本轮未执行 fmt、clippy、全 workspace、远端 CI、发布。没有新的性能结论。

## 覆盖与验证

变更意图：由本计划明确承诺，用 Rust/WASM cn、scanner、compiler 接管类处理，支持 Worker 中动态编译、scope CSS 安装和有限 GPU 投影，并保持静态组件入口。

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，branch `main`。
- 比较方式：`git diff --no-ext-diff --no-textconv HEAD --`；另枚举 untracked。index 无 staged 差异，空 diff 的 SHA-256 为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。
- [输入快照](../validation/wasm-tailwind/code-review-r1/input-snapshot.json) 保存 4,740 项最初 changed/untracked 文件及必要契约的内容摘要；包含大量历史/生成验证产物，指纹数量不等于逐文件代码评审覆盖数。SHA-256 算法与集合定义在该文件中。报告落盘前复核原输入，HEAD 和全部 4,740 项内容均未变；随后仅登记本轮台账，不修改被评代码或计划。指纹不保存源码内容，dirty 目标仅能由原工作树复现；本报告及 R1 证据不属于本轮输入。
- 单 agent 完成，没有委派 reviewer。只新增评审/复现证据并登记 SDLC；没有修改实现、原计划或提交代码。

| 本轮实际命令/步骤 | 结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-style --all-features --lib` | 525 passed，0 failed，12 ignored；93.03s。完整原始 log 未保存，最终工具回显记入 [summary](../validation/wasm-tailwind/code-review-r1/verification-summary.json) |
| `mbx test --locked -p rustify-ui -p rustify-components --features rustify-components/utility --lib` | components 94、SDK 300 passed，0 failed：[log](../validation/wasm-tailwind/code-review-r1/sdk-tests.log) |
| `mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib` | 12 passed，0 failed：[log](../validation/wasm-tailwind/code-review-r1/session-tests.log) |
| `mbx xtask sources verify` | exit 0；新 cn 与 Oxide 来源无漂移：[log](../validation/wasm-tailwind/code-review-r1/sources-verify.log)。Makepad 报告的是相对导入基线的已登记改写，不能解释为本轮改写 Makepad |
| `mbx xtask utility-build --group compiler --output /tmp/wasm-tailwind-review-facade` | 当前源 compiler-only WASM 构建通过；standard/scoped CSS 对比确认关键帧引用差异：[log](../validation/wasm-tailwind/code-review-r1/facade-build.log) |
| `mbx xtask build-web --example utility-lab --release` | 当前 release 构建通过：[log](../validation/wasm-tailwind/code-review-r1/build.log)。app build `a971d7b03cffe9f6`；Worker `5faa752bf64b6a9fd1c25d1f341ffd1e89f7171559c6bfca704c8e83b5efec46`，scanner disabled |
| `npx playwright test --project=utility-lab --workers=1 --reporter=line --output=/tmp/wasm-tailwind-review-playwright` | 当前 release / Chrome，53 passed，57.1s：[log](../validation/wasm-tailwind/code-review-r1/browser-tests.log)。覆盖现有主路径、cn 配置事务、Observer、两 scope、GPU 状态/恢复与故障；上述新增动画/活动集合复现不在现有断言中 |
| `target/debug/xtask serve --example utility-lab --release --port 0`，再运行 [repro.mjs](../validation/wasm-tailwind/code-review-r1/repro.mjs) 的 `animation`、`history`、`ci-oracle` 模式 | 浏览器入口 `http://127.0.0.1:64907/`，strict CSP；animation/history 均通过复现断言，page errors 为空。CI 模式模拟根声明依赖可见、独立 oracle 尚未安装的解析环境，并非远端 CI 运行 |

复现方式（仓根执行，先按上表 build/serve，URL 换成当前 serve 输出）：

```sh
node docs/validation/wasm-tailwind/code-review-r1/repro.mjs animation http://127.0.0.1:64907/
node docs/validation/wasm-tailwind/code-review-r1/repro.mjs history http://127.0.0.1:64907/
node docs/validation/wasm-tailwind/code-review-r1/repro.mjs ci-oracle
```

## 计划符合性

本期范围按 M1–M7 全部实现自述核查；需求账本来自用户确认与已有仓库契约，计划顶部没有独立 PRD 需求来源。这里只核查代码承担的契约，不重新评审计划方案。

| 需求/检查 | 实现及本轮验证 | 判定 |
| --- | --- | --- |
| R-1/R-8，V-2/V-8 | cn/typed variants、全部组件合并接入与 epoch 更新；94 项组件和 SDK 相关测试、root 浏览器 | 已验证本仓相关测试路径；完整上游 cn 差分本轮未重跑 |
| R-2，V-3 | scanner 源入口、源选择与 facade；style all-features、scanner session 测试 | native/会话测试通过；scanner-enabled 浏览器本轮未运行 |
| R-3/R-4，V-4/V-5/V-6 | compiler 核心、Worker、scope AST 变换、ensure/Observer | 常规测试通过；F-01 违反关键帧引用隔离；全上游执行门本轮未重跑 |
| NFR-1，§3/§8/V-9 | 有界队列、事务与重放 | 现有故障测试通过；F-02 缺少活动集合重建及配额恢复路径 |
| R-5/R-6，V-7/V-8 | GPU cascade、状态/主题/布局、Button 默认值恢复 | 300 项 SDK 与 root 浏览器相关测试通过；旧示例独立回归本轮未重跑 |
| C-1/C-3，V-10，M7 CI | 新 crate/features、Worker build、来源校验与 CI | 本地 build/来源检查通过；F-03 阻塞干净 CI oracle 测试 |
| NFR-2，V-11 | M7 性能记录 | 仅作为历史证据；本轮没有复测或性能裁定 |

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`。确认 **3 个 P2**：runtime 动画引用不完整、scope 无活动集合回收入口、新增浏览器 CI 缺 oracle 依赖。当前测试通过不足以关闭这些已复现缺陷；未逐行/未重跑范围见上文，不将本轮评审扩大为完整 Tailwind 兼容认证。

建议先修 F-01/F-02 的运行时契约，再补 F-03 的干净 CI 接线；分别补相应回归，确认原有 root/subpath、事务恢复与 GPU 活动类保留仍通过。修改后针对本报告复审，不把计划的 7/7 自述直接当作代码验收通过。

下一步：`$apply-code-review docs/plan/WASM-TAILWIND.code-review.md`。本轮不自动启动修复。
