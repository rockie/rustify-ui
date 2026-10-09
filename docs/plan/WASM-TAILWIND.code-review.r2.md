# WASM-TAILWIND · 代码评审 R2

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [WASM-TAILWIND](WASM-TAILWIND.md)，sha256 `2fe12bedb3ac9b5d08c064f81ea90cd4a1691f33047955822b81c2aef8384b3a` | dirty@`53652fcc405a2679df19eb10e252efd4a4236f50`；当前输入集合指纹 `b7960e9bf77e09986df7964d41bcbcc41ac53227fa2e637174b077981186c71b`；核对 R1 全部问题及修复涉及的行为链 | [R1](WASM-TAILWIND.code-review.md)，sha256 `fb7348d09f1d462340db49801cde5a0807944523b1745ba39aeed1cdd9af7209`；对象 ID `de7db56e-c90e-4ac5-9a71-1e763e8971c4` |

评审日期：2026-10-08（Asia/Shanghai）。本轮对象 ID：`dc69f529-230c-40b8-b49f-48ad6369dca2`。用户仅给计划，因此采用开发计划模式；已有 R1，默认做修复效果复审，未重新完整审查 M1–M7。处置记录：[M6](../validation/wasm-tailwind/M6.md#评审修复代码评审-r1)、[M7](../validation/wasm-tailwind/M7.md#评审修复代码评审-r1)。代码和计划保持只读。

R1 基线为同一 HEAD → 当时工作树，指纹 `a021455f93298d8f3df2c70d5d8339a565cc7a6e905c200c9c87bcd58779e25f`。两轮均未提交，没有登记的新实现或修复提交。R1 保存内容 hash，没有保存源码全文，故**无法恢复精确 R1 → R2 修复 patch**；本轮根据 hash 变化、处置记录和当前代码检查修复相关路径，新增问题归于 HEAD → 当前工作树中的新增 scope 模块，修复增量归因存在上述限制。留存旧 facade 的二进制对照仅作为补充证据，不冒充恢复了 R1 源码版本。

## 上轮问题复审

| 来源 / ID | 上轮问题 | 已有处置 / 当前版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01，P2 | 非 inline 动画变量没有随关键帧改名，动画无法运行 | M6 已添加变量依赖追踪、别名和 fallback 改写；dirty@上述 HEAD | **已解决**（原问题）；相关修复出现本轮 F-04 | [scope.rs:70](../../crates/rustify-style/src/scope.rs:70)、[scope.rs:186](../../crates/rustify-style/src/scope.rs:186)；新增 host 单测通过；root/deep 的两 owner、非 inline theme、别名、fallback 测试通过，各有 1 个活动动画且末帧来自本 owner | 修 F-04，保持这些已通过场景 |
| R1 / F-02，P2 | SDK 没有活动集合重建入口，历史候选占满配额后无法回收 | M6 新增 `UtilityScope::rebuild(active)`，`Batch::retains_installed` 区分重建与保留语义 | **已解决** | [browser.rs:332](../../crates/rustify-ui/src/utility/browser.rs:332)、[state.rs:595](../../crates/rustify-ui/src/utility/state.rs:595)、[state.rs:647](../../crates/rustify-ui/src/utility/state.rs:647)；host 与 root/deep 回收配额、seed/DOM/显式活动类保留、超额失败不换快照通过；独立浏览器检查安装失败保留 report、ack 丢失后重放新活动集合、scanner 变量重建和 retry 保留通过 | 无原问题剩余动作 |
| R1 / F-03，P2 | 新增 browser CI 没有安装独立 Tailwind oracle | M7 在 utility-lab 矩阵项使用 oracle 自己的锁文件安装 | **已解决** | [verify.yml:212](../../.github/workflows/verify.yml:212)；按 CI 相对目录结构建立无 node_modules 的隔离目录，执行相同 `npm ci --ignore-scripts --prefix tests/utility/compiler/oracle` 成功，加载版本 4.1.13 并编译 `flex` 成功；YAML 可解析 | 远端 Actions 未运行，不计为远端 CI 通过 |

R1 的待核实/覆盖限制另行核对：scanner 浏览器与 subpath 已在当前产物重跑；完整上游执行门、完整 cn 差分、旧示例独立回归、性能和全 workspace 检查仍未由本轮重跑。未逐行重新审查全部 family、generated 数据和上游测试，保持 R1 的相应覆盖边界。

## 本轮确认问题

### F-04 · [P2] 只重写实际关键帧引用，保留动画参数变量的语义

- 定位：[scope.rs](../../crates/rustify-style/src/scope.rs:186)，当前目标第 186 行；此文件是 HEAD → 工作树的 untracked 新文件。支持路径为同文件第 83–91、145–159 行。
- 问题：输入同时定义 `@keyframes card`、`@keyframes linear`，并使用 `--ease: linear; animation: card 1s var(--ease) infinite` 时，新增变量依赖追踪把 `--ease` 视作动画名称变量，随后按关键帧字面表把其值改成 `rustify-<owner>-linear`。这个变量本应填入 timing function，替换后 shorthand 含有两个名称而没有合法的 easing 值，浏览器将整条动画置为初始值；当前实际结果为 `animationName=none`、活动动画数 0，合法原始 CSS 对照为 `card`、`linear`、活动动画数 1。受影响的是支持自定义 CSS/变量的 runtime scope，另一控件定义一个合法同名关键帧就能让原动画停止；这不是未知 utility 或 CSSOM 抛错，ensure 仍成功。可达性只说明变量参与动画，不能证明其中所有匹配词都是关键帧引用。
- 证据：[probe.json](../validation/wasm-tailwind/code-review-r2/probe.json) 的 `keyword` 保存当前 Worker 输出和真实 Chrome 结果；[comparison-browser.json](../validation/wasm-tailwind/code-review-r2/comparison-browser.json) 对同一输入安装留存旧 facade 与当前 facade 的 scope CSS：旧值保持 `linear`、1 个活动动画；当前值变成 `rustify-new-linear`、0 个活动动画。二进制来源限制与 hash 见 [provenance](../validation/wasm-tailwind/code-review-r2/old-facade-provenance.json)。计划 [§5.4 第 6 步](WASM-TAILWIND.md:310) 要求同步引用并保留公开 theme tokens；动画 shorthand 必须区分关键帧名与其他参数，规范也要求先将匹配其他子属性的关键词解析到相应子属性。[W3C CSS Animations §3.10](https://www.w3.org/TR/css-animations-1/#animation-shorthand-property)
- 修复与验证：按 `animation-name` / animation shorthand 的语义识别名称引用，不将整条可达变量值中的匹配词一律重命名；保持 easing、迭代、方向、fill、play-state 等参数变量的含义。回归至少覆盖本例中的 timing 变量和同名关键帧，断言当前 scope 的 card 动画运行且 timing 为 `linear`；同时保留 F-01 的非 inline theme、别名、fallback 和两 owner 隔离断言。
- 置信度：高。当前源码、Worker 实际输出、真实浏览器失效、合法 CSS 对照与留存 facade 对照一致；精确 R1 源码 diff 不可恢复的限制已单列。

## 待核实项与覆盖限制

- **精确修复 patch 缺失**：R1 工作树只有指纹；本轮能判断当前问题与相关路径，不能给出未经恢复的逐行修复增量清单。后续如需完整原变更重审，应提供可恢复的源码快照或明确提交范围。
- **远端 CI 未执行**：F-03 的安装步骤及 oracle 加载已用隔离环境验证，不能代替 GitHub Actions 的 Linux runner 和 artifact 链路运行。
- 本轮未重跑上游 1,183 项执行门、完整 cn JS/native/WASM 差分、旧示例、全 workspace、fmt、clippy、来源验证、性能及发布；历史实施证据只作背景，不计本轮通过。不扩为完整 Tailwind 兼容认证。
- 当前工作树另有 Theme Studio 证据、skill 配置、SDLC 文件变化；不是本轮代码 findings 对象。本轮没有修改实现、原计划、R1 报告，也未提交或推送。

## 覆盖与验证

变更意图来自计划：用 Rust/WASM cn、scanner、compiler 支持动态 CSS scope 与有限 GPU 投影；本轮聚焦 R1 三项修复及其相关回归。

- 仓根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；branch `main`；HEAD `53652fcc405a2679df19eb10e252efd4a4236f50`。
- 比较入口：`git diff --no-ext-diff --no-textconv --name-status -M HEAD --`、tracked patch 与 untracked 枚举；另核对 index，staged 差异为空。计划完成记录没有提交列，恢复快照写未提交，不擅自纳入其他历史提交。
- [输入快照](../validation/wasm-tailwind/code-review-r2/input-snapshot.json) 固定最初 tracked dirty、untracked 与 R1 必要输入共 **4,762 项**的内容 hash；算法在文件中。集合包含大量验证资料，数量不等于代码逐文件覆盖数。落报告前核对全部原输入内容未变；之后仅维护本轮报告/证据和 SDLC。dirty 源码没有另存全文，仍仅原工作树可复现。
- R1 已有项中 **16 个文件**的 hash 改变，完整清单见 [changed-files.txt](../validation/wasm-tailwind/code-review-r2/changed-files.txt)。代码路径为 `.github/workflows/verify.yml`、`scope.rs`、SDK `browser.rs/state.rs/cn.rs`、utility-lab `app.js/src/main.rs`、`utility-runtime.spec.ts`；另读计划、utility guide、M6/M7 和状态记录。未重新逐行审查未变的全部生产代码及既有测试。
- 已审链路：动画变量依赖 → scope 变换 → facade → Worker → CSSOM → 浏览器；rebuild → cn preflight → epoch/队列 → 活动集合/seed/DOM → budget/安装/ack/retry；CI oracle 独立锁文件 → 矩阵安装 → 测试加载。默认入口、错误恢复、DOM/GPU 主路径、键盘与窄屏检查使用实际浏览器测试；不是 Portal App，不涉及宿主安装或 onebox。
- 单 agent 完成，没有委派 reviewer；按 review-code/rust 技能工作。只写报告、隔离实验及证据与台账。

| 本轮实际命令/步骤 | 当前版本结果 / 证据 |
| --- | --- |
| `mbx test --locked -p rustify-style --all-features --lib` | **526 passed，0 failed，12 ignored**；[log](../validation/wasm-tailwind/code-review-r2/style-tests.log) |
| `mbx test --locked -p rustify-ui -p rustify-components --features rustify-components/utility --lib` | **SDK 301、components 94 passed，0 failed**；[log](../validation/wasm-tailwind/code-review-r2/sdk-tests.log) |
| `mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib` | **12 passed，0 failed**；[log](../validation/wasm-tailwind/code-review-r2/facade-tests.log) |
| `mbx xtask build-web --example utility-lab --release`，随后同入口加 `--base /tools/utility-lab/` | compiler-only 两份构建成功；[root](../validation/wasm-tailwind/code-review-r2/build-root.log)、[deep](../validation/wasm-tailwind/code-review-r2/build-deep.log) |
| `RUSTIFY_XTASK=target/debug/xtask npx playwright test --project=utility-lab --workers=1 --reporter=line --output=<隔离临时目录>/browser-root`，随后独立运行 `utility-lab-deep` | **root 55 / deep 49 passed**；[root](../validation/wasm-tailwind/code-review-r2/browser-root.log)、[deep](../validation/wasm-tailwind/code-review-r2/browser-deep.log)；含原问题修复与 cn、GPU、配置事务、恢复、两 scope、键盘及窄屏回归 |
| 相同 build 入口分别加 `--utility-scanner`、`--base /tools/utility-lab/ --utility-scanner`；串行运行 `utility-lab-scanner`、`utility-lab-scanner-deep` | **scanner root 5 / deep 5 passed**；[build](../validation/wasm-tailwind/code-review-r2/build-scanner.log)、[deep build](../validation/wasm-tailwind/code-review-r2/build-scanner-deep.log)、[root tests](../validation/wasm-tailwind/code-review-r2/browser-scanner.log)、[deep tests](../validation/wasm-tailwind/code-review-r2/browser-scanner-deep.log) |
| `target/debug/xtask serve --example utility-lab --release --port 0`，运行独立 [probe.mjs](../validation/wasm-tailwind/code-review-r2/probe.mjs) | `http://127.0.0.1:61771/`，strict CSP / 实际 Chrome；F-04 活动动画 0 vs 合法对照 1；rebuild 安装失败完整保留 report，ack timeout 后重放 w-[248px]；page errors、console errors、失败请求、CSP violations 均为空。[结果](../validation/wasm-tailwind/code-review-r2/probe.json) |
| [facade-comparison.mjs](../validation/wasm-tailwind/code-review-r2/facade-comparison.mjs)，再运行 [comparison-browser.mjs](../validation/wasm-tailwind/code-review-r2/comparison-browser.mjs) | 留存旧/当前 facade 输出 `--ease: linear` vs `--ease: rustify-new-linear`；真实浏览器活动动画 1 vs 0。scanner `rebuild(['--color-probe','w-[147px]'])` 后变量为 #123456、宽 147px，retry 保留变量。[结果](../validation/wasm-tailwind/code-review-r2/comparison-browser.json) |
| 无 node_modules 的隔离目录复制 oracle 的 package.json/package-lock.json，按 CI 相对目录执行 `npm ci --ignore-scripts --prefix tests/utility/compiler/oracle`；随后 createRequire 加载并编译 flex | 安装 **3 packages**，加载 **4.1.13**，输出 `display: flex`；[log](../validation/wasm-tailwind/code-review-r2/ci-oracle-exact.log)。初次使用仓外任意命名目录与绝对 prefix 的改编实验报 EUSAGE，[原 log](../validation/wasm-tailwind/code-review-r2/ci-oracle.log) 保留，不作为实际 CI 缺陷；与 CI 一致的相对路径实验成功 |
| `ruby -ryaml -e 'YAML.load_file(".github/workflows/verify.yml")'` | 可解析；未执行远端 CI |
| 验收后按默认 build 入口恢复 compiler-only root/subpath | 两份均成功，scanner_enabled=false；[root restore](../validation/wasm-tailwind/code-review-r2/restore-root.log)、[deep restore](../validation/wasm-tailwind/code-review-r2/restore-deep.log) |

当前 compiler-only 主应用 build_id **`9926118ec160f77a`**，Worker **`c2ee34fd0c5981850540bb13c80e3be250809e4b893950881077905cf110b6a4`**；root/subpath 一致，manifest 已归档到本轮证据目录。scanner 两份 manifest 单列保留，不混为默认构建证据。已实际查看本轮 [desktop](../validation/wasm-tailwind/code-review-r2/desktop.png) / [mobile](../validation/wasm-tailwind/code-review-r2/mobile.png)：桌面并排、窄屏纵向，关键控件可见；这些截图不证明 F-04 动画正确。实际结果摘要见 [verification-summary.json](../validation/wasm-tailwind/code-review-r2/verification-summary.json)。

复现 F-04（仓根执行，URL 换成当前 serve 输出）：

```sh
node docs/validation/wasm-tailwind/code-review-r2/probe.mjs http://127.0.0.1:61771/ /tmp/utility-r2-probe.json
```

## 计划符合性

| 本轮相关契约 | 当前实现 / 验证 | 判定 |
| --- | --- | --- |
| R-3/R-4，§5.4 第 6 步，V-6：动画与全局资源隔离 | 原 F-01 的非 inline/alias/fallback/两个 owner 已验证；变量追踪误改 timing 参数，见 F-04 | **存在缺陷** |
| NFR-1，§3/§8，V-9：显式 active 重建及配额恢复 | rebuild 入口、快照预算不并入历史；seed/DOM/活动类与变量、失败保留、ack/retry 已验证 | 本轮相关范围通过 |
| C-3，M7/V-10：oracle 独立锁定与 CI 接线 | utility-lab 矩阵安装步骤与隔离解析通过 | 本地验证通过；远端未核实 |
| 其他 M1–M7 / V-1–V-11 完整承诺 | 本轮只核对 R1 问题与修复相关回归 | **本轮未完整复核**，不更新计划完成状态 |

计划没有独立 PRD 需求来源；本轮不自动启动 PRD 差异核查。

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`。**R1 的 3 项原问题均已解决，但本轮确认 1 个 P2 回归 F-04**。修复效果结论：**未完全解决**，因为相关动画回归仍阻止通过。现有 55+49+5+5 浏览器测试通过，不抵消独立复现的动画失效。

先修 F-04 的动画名称/参数语义识别，再验证其失败输入恢复为当前 scope 的正确动画与 linear timing，保持 R1 F-01 的别名/fallback/两 scope 以及 F-02/F-03 回归通过。下一轮默认复核本轮 F-04 和直接影响范围；完整原变更、远端 CI 与性能仍按所述边界分别取证。评审结论不代表合并、发布或整期 PRD 验收授权。

下一步：`$apply-code-review docs/plan/WASM-TAILWIND.code-review.r2.md`。修复后：`$review-code docs/plan/WASM-TAILWIND.md`。本轮不自动启动修复。
