# WASM-TAILWIND · 代码评审 R3

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3，修复效果复审 | [WASM-TAILWIND](WASM-TAILWIND.md)，sha256 `6d79e35fbc904790b13f24c7164af21f732e21494c8dbd45a73e656349a809f9` | dirty@`53652fcc405a2679df19eb10e252efd4a4236f50`；输入集合指纹 `6e4ef3de5350ebb8dd63bd979e17cdcea256babe7be177b342fdc7ebd4113c8f`；复核 R2 F-04 及 scope 修复直接影响的行为链 | [R2](WASM-TAILWIND.code-review.r2.md)，sha256 `583cd2dc5e3a0da3e1ab35aac653c3d109fed05348f4fe48dcd0b5ea5bf29e10`；对象 ID `dc69f529-230c-40b8-b49f-48ad6369dca2` |

评审日期：2026-10-08（Asia/Shanghai）。本轮对象 ID：`e4b93990-4ec9-41be-826f-4be4af5fc632`。用户仅给计划，按开发计划模式取最大轮次 R2，默认做**修复效果复审**；不是 M1–M7 完整重审。已有处置：[M6 的 R2 修复](../validation/wasm-tailwind/M6.md#评审修复代码评审-r2)、[M7 的证据影响说明](../validation/wasm-tailwind/M7.md#评审修复代码评审-r1)。

R2 基线为同一 HEAD → 当时工作树，指纹 `b7960e9bf77e09986df7964d41bcbcc41ac53227fa2e637174b077981186c71b`。计划完成记录无登记的实现/修复提交，当前也无新提交。R2 没有保留 scope 源码全文，**无法恢复精确 R2 → R3 patch**；本轮用 R2 内容 hash 确认只有 `scope.rs` 和 `utility-runtime.spec.ts` 两个生产/测试源文件变化，再通读当前 scope 文件与相关测试和调用链。新增 findings 的 diff 归因与定位均采用可确认的 **HEAD → 当前工作树**：`scope.rs` 是新增文件，HEAD 没有该 runtime 变换。它们是本轮新确认的问题；不能据此声称已经恢复了修复前源码，或完整重审了原变更。

## 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R2 / F-04，P2 | 动画参数变量被当作关键帧名，原例动画停止 | M6 增加 Shorthand/Name 角色和 Layer 槽位判别；当前 scope sha256 `7a8bcaf626ac6a89745039d02de0f617917a2a788f8e744bf2588fb91e24a0d4` | **部分解决** | 原例在 host、新增浏览器回归和独立 probe 中通过；同一 `--ease` 同时供 timing 与 `animation-name` 使用时，Name 角色仍全局改写变量，原 timing 动画失效，见下文 F-04 | 按引用用途保留变量语义，避免全局角色升级使 timing 值变成名称；同时修 F-05–F-07 |

R2 中已关闭的 R1 项目作为历史状态保留：F-01 的非 inline theme、alias、fallback、两个 owner 动画用例在本轮 root/deep 都重跑通过；F-02 的 rebuild 配额回收用例在两个入口重跑通过，SDK 源码未变，未重跑其全部独立故障实验；F-03 的 CI 文件 hash 未变，**本轮未复核**远端 CI 或重复安装实验，不改写 R2 的关闭结论。原始来源 ID 为 `de7db56e-c90e-4ac5-9a71-1e763e8971c4`，详见 [R2 复审表](WASM-TAILWIND.code-review.r2.md#上轮问题复审)。

## 确认问题

### F-04 · [P2] 保留同时作为 timing 和名称使用的变量语义（沿用 R2，部分解决）

- 定位：[scope.rs:129](../../crates/rustify-style/src/scope.rs:129)，当前第 129–132 行；支持位置：第 70–75、299–306 行。旧问题复核，不作为本轮新增 finding。
- 问题：在原 F-04 输入中增加 `.other { animation-name: var(--ease); animation-duration: 10s; }`，而 `.motion` 仍使用 `animation: card 10s var(--ease) infinite`、`--ease: linear`，`@keyframes card` 与 `@keyframes linear` 也都存在时，某处流入 `animation-name` 会把 `--ease` 的统一角色升级为 Name，并将声明改成 `--ease: rustify-<owner>-linear`。`.other` 正常运行，`.motion` 的 timing 却被同一改写破坏，浏览器将其动画恢复为初始值。原始 CSS 中两者都是合法动画；当前主动画 `animationName=none`、活动数 0，对照为 `card`、`linear` timing、活动数 1。这仍违反原报告“只重写实际名称引用、保留参数变量”的关闭要求，故原例通过不足以关闭整个问题。
- 证据：[animation-probe.json](../validation/wasm-tailwind/code-review-r3/animation-probe.json) 的 `shared-role` 同时记录两个元素、变量与实际 Worker CSS；[facade-probe.json](../validation/wasm-tailwind/code-review-r3/facade-probe.json) 的同名项区分无 scope 编译与带 scope 编译，确认失败发生在 scope 变换。原例 `fixed` 及现有浏览器第 843 行用例通过。
- 修复与验证：使同一公开变量的名称引用和 timing 引用保留各自用途，避免在原变量声明上施加一个覆盖全部用途的 Name 改写。断言两个元素都运行本 owner 的正确关键帧，主动画 timing 为 `linear`；保留 R1 非 inline/alias/fallback/两 owner 及 R2 原例回归。
- 置信度：高。变量角色升级、输出和真实 Chrome 的结果一致。

### F-05 · [P2] 在变量代换上下文中识别 shorthand 的名称位置（本轮新增）

- 定位：[scope.rs:260](../../crates/rustify-style/src/scope.rs:260)，当前第 260–265 行；HEAD → 工作树新增文件。支持位置：第 90–135、299–306 行。
- 问题：合法输入 `--x: linear; animation: var(--x) 10s ease infinite` 配合 `@keyframes ease` 时，var 分支不读取实际代换值、不占用父层的 easing 槽，因此把其后的 `ease` 当 timing 而漏改；关键帧定义却已改为 `rustify-<owner>-ease`，计算后的名称仍是裸 `ease`，没有动画。反向拆分同样失效：`--name: linear; animation: 10s ease var(--name) infinite` 配合 `@keyframes linear`，变量声明从空 Layer 独立判别，又把真正的名称 `linear` 当 timing。两个原始 CSS 对照均有 1 个动画，当前 SDK ensure 成功但实际活动数均为 0。仅追踪变量名及 Shorthand 角色不能保留代换后 token 在整条 shorthand 中的位置，影响运行时 CSS/主题变量组成的动画。
- 证据：[animation-probe.json](../validation/wasm-tailwind/code-review-r3/animation-probe.json) 的 `var-before`、`var-name`；[facade-probe.json](../validation/wasm-tailwind/code-review-r3/facade-probe.json) 同名项保存无 scope/有 scope 输出。[CSS Variables §3.2](https://www.w3.org/TR/css-variables-1/#variables-in-shorthands) 规定 shorthand 的 longhand 在变量代换后解析。M6 第 21 行已记录前一个边界尚未处理，但计划 [§5.4 第 6 步](WASM-TAILWIND.md:310) 的引用同步契约并未排除此类合法输入，用户也未豁免。
- 修复与验证：让名称识别保留变量代换的完整上下文，不能把无 fallback var 当空值，也不能为每个被引用变量重置独立槽位；按用途保持公开 token/级联语义。至少回归上述两种拆分、alias/fallback 与两个 owner，断言动画名称使用当前 namespace、活动数 1 且 easing 与原输入一致。
- 置信度：高。两个独立输入均经当前 facade 和真实 SDK/Worker/CSSOM 复现，合法原始 CSS 对照通过。

### F-06 · [P2] 不要将时长数学函数占用为 easing（本轮新增）

- 定位：[scope.rs:267](../../crates/rustify-style/src/scope.rs:267)，当前第 267–268 行；HEAD → 工作树新增文件。
- 问题：`@keyframes card` 配合 `animation: calc(10s) linear card infinite` 是当前 Chrome 支持的合法动画，但新增 Function 通配分支把 `calc(10s)` 当作 easing。随后的 `linear` 因 easing 槽已占而被当作名称，即使没有对应 keyframes 也消耗 name 槽，真正的 `card` 因而不再改名；定义变成 `rustify-<owner>-card`，引用保留裸 `card`，活动数从原始 CSS 的 1 变成 0。`min(10s, 12s)` 得到同样结果。使用数学函数表达时长/延迟不应改变关键帧绑定，也不需要同名 timing 关键帧才能触发。
- 证据：[animation-probe.json](../validation/wasm-tailwind/code-review-r3/animation-probe.json) 的 `time-calc`、`time-min`；[facade-probe.json](../validation/wasm-tailwind/code-review-r3/facade-probe.json) 证明无 scope 编译保留合法声明，scope 仅改定义而漏改引用。[CSS Values §10.9](https://www.w3.org/TR/css-values-4/#calc-type-checking) 允许数学函数用于其计算结果类型可接受的位置；这里实际 computed duration 为 `10s`。
- 修复与验证：只对实际 easing 函数占用 easing 槽；按数学函数的值类型识别时长、延迟或迭代次数。回归 calc/min 时长置于 easing 和名称之前的输入，断言名称指向本 scope、duration 为 `10s`、timing 为 `linear`、活动数 1；继续保留 steps/cubic-bezier/linear 函数用例。
- 置信度：高。源码分支、两个数学函数的 Worker 输出与浏览器正反对照一致。

### F-07 · [P2] 数值迭代次数需要占用 iteration 槽（本轮新增）

- 定位：[scope.rs:212](../../crates/rustify-style/src/scope.rs:212)，当前第 212–215 行；支持位置第 219–224、226–240 行；HEAD → 工作树新增文件。
- 问题：`@keyframes infinite` 配合 `animation: 2 10s infinite linear` 时，浏览器把先出现的数值 `2` 用作迭代次数，之后的 `infinite` 是关键帧名。当前 numeric 分支却直接返回而不设置 iteration，后续 `infinite` 被当作仍未指定的迭代关键词，名称漏改；定义已 namespace 化，实际活动数从 1 变成 0，computed iteration 仍是 `2`。这会破坏合法的关键词名称动画，根因在数值槽位识别，与 F-06 的 Function 通配分支独立。
- 证据：[animation-probe.json](../validation/wasm-tailwind/code-review-r3/animation-probe.json) 的 `iteration` 与 [facade-probe.json](../validation/wasm-tailwind/code-review-r3/facade-probe.json) 同名项；[CSS Animations §3.10](https://www.w3.org/TR/css-animations-1/#animation-shorthand-property) 要求优先把关键词分配给尚未指定的其他子属性。探针中 `iteration-before-name` 的另一顺序原始 CSS 也无动画，是无效输入对照，**不作为本 finding 的失败证据**。
- 修复与验证：区分普通 number 和带 s/ms 单位的 time；数值 iteration 必须记录占用状态，再决定后续关键词是否是名称。验证整数和小数 iteration 在名称前出现、duration/delay 与逗号分层保持正确，断言本 scope 的 `infinite` 动画运行且次数为输入数值。
- 置信度：高。原始 CSS 对照有效，实际浏览器结果与分支状态直接对应。

## 待核实项与覆盖限制

| R2 待核实/未覆盖项 | 本轮处理与剩余限制 |
| --- | --- |
| 精确未提交修复 patch 缺失 | 确认 R2 文件 hash → 当前 hash 的变化范围；无法恢复旧源码。已为后续保存当前 [scope 源码快照](../validation/wasm-tailwind/code-review-r3/source-snapshot/crates/rustify-style/src/scope.rs) 与 [测试快照](../validation/wasm-tailwind/code-review-r3/source-snapshot/tests/browser/utility-runtime.spec.ts)；未恢复的精确增量不能冒充已审 |
| 远端 GitHub Actions | 本轮未执行；CI 源码未变，沿用 R2 本地安装验证的既有范围，仍不声明远端通过 |
| 完整上游门、cn 差分、旧示例、workspace、fmt/clippy、来源与性能 | 本轮只重跑下表与 scope 相关检查，完整上游执行门及其逐测试账本、旧示例和性能仍未重新核对；不能声称本轮覆盖 V-1–V-11 全量终验 |
| 完整 scanner/root/subpath 重跑 | 重跑 compiler-only root/subpath 和 facade compiler+scanner host 测试；没有重新构建 scanner 浏览器产物或运行其项目。R2 与修复记录中的 scanner 浏览器结果是历史证据 |
| 其他 dirty 内容 | Theme Studio 证据、skill 配置、其他 SDLC 对象及未受 scope 修复影响的生产代码不纳入本轮深入审查；不产生无关旧问题 finding |

上述限制不影响已复现问题的判定；本轮不自动修改实现、计划、既有报告或启动修复。

## 覆盖与验证

变更意图来自计划及 M6 处置记录：修正 runtime 动画 namespace 变换，使 timing 等参数保持原义，同时保留名称与变量引用隔离。用户本轮授权评审。

- 仓根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；branch `main`；HEAD `53652fcc405a2679df19eb10e252efd4a4236f50`。原输入：`$review-code docs/plan/WASM-TAILWIND.md`。
- 比较方式：HEAD → 工作树的 tracked 净差异与 untracked；index 为空，staged diff sha256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。无登记提交范围，不擅自选最近实现提交。
- [输入快照](../validation/wasm-tailwind/code-review-r3/input-snapshot.json) 固定 **4,810 项**文件内容 hash，来自当前 dirty/untracked 加 R2 必要输入。它是内容集合，不代表逐行审过 4,810 项。[变化清单](../validation/wasm-tailwind/code-review-r3/changed-files.txt) 共 10 项，生产/测试源变化仅 `scope.rs` 和 `utility-runtime.spec.ts`；48 个新文件都是 R2 报告、证据和台账，没有其他新增生产代码。完整原变更的其他模块沿用上轮覆盖边界。
- [核验结果](../validation/wasm-tailwind/code-review-r3/input-verification.json)：测试及构建后、写本轮报告/台账前，所有 4,810 项内容与最初 hash 一致，HEAD 未变。本轮新报告/证据不纳入输入集合；台账维护另行授权于技能和仓库协议。计划保持原位与原 hash。
- 静态阅读：完整当前 scope 文件；动画角色依赖、Layer 判别、rewrite；facade 初始化/prepare 调用；Worker → SDK 样式安装 → demo 入口；对应 6 个浏览器用例及上轮报告、处置和相关计划契约。未恢复的旧 patch、其余不变模块的全部源码和上游 fixture 不计为完整已审。
- 实际浏览器：Chrome、strict CSP、compiler-only；独立探针入口 `http://127.0.0.1:61502/`，root 常规项目 `http://127.0.0.1:4186/`，subpath `http://127.0.0.1:4187/tools/utility-lab/`。新构建 main `9926118ec160f77a`、Worker `c2a524e1ad2273647dcfa8485b47e38e52a86fbcb85fe6b4d4a100e3c8ba03da`，两份 manifest 已保存，见 [root](../validation/wasm-tailwind/code-review-r3/worker-manifest-root.json)、[deep](../validation/wasm-tailwind/code-review-r3/worker-manifest-deep.json)。探针没有 page/console error、失败请求或 CSP violation，动画故障由真实样式/活动数确认。
- 本轮没有视觉设计或页面改动；浏览器验证覆盖动画、scope 层级、selector、配置事务和 rebuild。没有完整 UI 走查、GPU 图像验收或性能测量。不是 Portal App，没有宿主/onebox 链路。
- 单 agent，应用 review-code 与 rust 技能，没有委派 reviewer。只写报告、评审证据和状态台账。

| 本轮实际命令/步骤 | 当前目标版本的结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-style --all-features --lib` | **527 passed，0 failed，12 ignored**；[log](../validation/wasm-tailwind/code-review-r3/style-tests.log)。ignored 不计通过 |
| `mbx test --locked -p rustify-style-wasm --no-default-features --features compiler,scanner --lib` | **12 passed，0 failed**；[log](../validation/wasm-tailwind/code-review-r3/facade-tests.log) |
| `mbx xtask build-web --example utility-lab --release`；随后同入口加 `--base /tools/utility-lab/` | 两份 compiler-only release 构建成功；[root](../validation/wasm-tailwind/code-review-r3/build-root.log)、[deep](../validation/wasm-tailwind/code-review-r3/build-deep.log) |
| root / deep 的六项相关 `utility-runtime.spec.ts` 用例，完整命令见下 | **root 6 / deep 6 passed**；[root log](../validation/wasm-tailwind/code-review-r3/browser-root.log)、[deep log](../validation/wasm-tailwind/code-review-r3/browser-deep.log)。两 owner、原例、rebuild、scope dispose、关系 selector 与配置成功/失败事务通过；不是完整 56/50 套件 |
| `target/debug/xtask serve --example utility-lab --release --port 0`；运行 [animation-probe.mjs](../validation/wasm-tailwind/code-review-r3/animation-probe.mjs) | strict CSP 下 9 组当前 SDK 与原始 CSS 对照：原例与 fallback 正对照通过，6 个合法输入因 F-04–F-07 没有预期活动动画；另 1 组原始 CSS 就无效，排除为问题证据。[JSON](../validation/wasm-tailwind/code-review-r3/animation-probe.json) |
| `node docs/validation/wasm-tailwind/code-review-r3/facade-probe.mjs` | 对同一输入分别使用 owner=null / review-r3，保留当前 WASM 的无 scope 和 scope 编译输出，确认名称丢失/参数改写来自 scope 变换。[JSON](../validation/wasm-tailwind/code-review-r3/facade-probe.json) |
| W3C CSS Animations §3.10、Variables §3.2、Values §10.9 | 实际读取官方文档；作为语义支持，具体实现故障以当前源码及上述实际浏览器为证据 |

两个浏览器项目串行运行的完整命令：

```sh
RUSTIFY_XTASK=target/debug/xtask npx playwright test --project=utility-lab --workers=1 --reporter=line --output=/tmp/rustify-review-r3-browser-root utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|rebuild releases|nested owners|scoped relation|configuration replacement'
RUSTIFY_XTASK=target/debug/xtask npx playwright test --project=utility-lab-deep --workers=1 --reporter=line --output=/tmp/rustify-review-r3-browser-deep utility-runtime.spec.ts --grep 'scoped keyframes|a keyframes name|rebuild releases|nested owners|scoped relation|configuration replacement'
```

复现（仓根执行；serve 重启后替换 URL）：

```sh
node docs/validation/wasm-tailwind/code-review-r3/animation-probe.mjs http://127.0.0.1:61502/ /tmp/utility-r3-probe.json
```

## 计划符合性

| 本轮相关契约 | 实现/验证 | 判定 |
| --- | --- | --- |
| R-3/R-4、§5.4 第 6 步、V-6：global keyframes 与引用同步并保持公开 token 语义 | 原 F-04 单用途例通过，混合用途仍失败；变量上下文、数学时长、数值 iteration 漏改名称，见 F-04–F-07 | **存在缺陷** |
| R1 F-01 的非 inline theme/alias/fallback/两 owner；scope selector/dispose/config/rebuild 必要回归 | root/deep 各 6 项通过；style/facade host 通过 | 本轮所测范围通过，不能抵消独立反例 |
| 其他 M1–M7、V-1–V-11 | 本轮未重新完整审查；按上述限制保持历史证据原位 | **本轮未完整复核** |

计划没有独立 PRD 需求来源；本轮不自动启动 PRD 差异核查。

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`，限于声明的 HEAD → 工作树新增 scope 实现及本轮相关行为链。修复效果：**未完全解决**。R2 F-04 的原始输入已通过，但混合用途仍违反其参数保留要求；本轮新增 **3 个 P2：F-05、F-06、F-07**。现有 host 和 6+6 浏览器回归通过，仍有独立真实浏览器失败输入。

先统一解决 F-04/F-05 的变量引用用途与代换上下文，避免继续全局猜测名称；再补齐 F-06/F-07 的函数/数值槽位识别。关闭条件是所有本轮合法探针恢复正确的 namespace、参数和活动动画数，并保持现有原例、alias/fallback、两 owner 及配置事务回归通过。最后以修后构建再复审；未覆盖的完整上游、scanner 浏览器、性能和远端 CI 仍按独立范围取证。

下一步：`$apply-code-review docs/plan/WASM-TAILWIND.code-review.r3.md`。修复后：`$review-code docs/plan/WASM-TAILWIND.md`。本轮不自动修复，不改变历史报告结论或计划完成记录。
