# DOM-COMPONENTS-P2 · 代码复审 R3

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3，修复效果复审 | [DOM-COMPONENTS-P2](DOM-COMPONENTS-P2.md)，v0.2 | `834b62bdbdd4c1572b83b253d9499e398575bf65..64bb17281567a646b312a4b7050219fe5a7f0c55`；另核对用户本轮要求的 AGENTS.md 修改 | [代码复审 R2](DOM-COMPONENTS-P2.code-review.r2.md)，评审对象 `5c50ad02-b0e1-404a-a998-d3a1a688e224` |

独立评审 ID：`09cb5ab0-a1c6-4684-a9bd-65b998cf31dd`。日期：2026-10-31，Asia/Shanghai。当前主 agent 单独完成。审查对象为 DOM SDK 库及直接消费者，Rust edition 2021、组件 MSRV 1.99。本轮复核 R2 全部问题和相关回归，不是原始 218 文件变更的完整重审。

R2 所审范围为 `9fd21ca86ff2dc64bfa19059a560a92c9da2b96c..834b62bdbdd4c1572b83b253d9499e398575bf65`；本轮修复提交为 `9658e5d9c7680acc5cfacec3469c6ed81d231c42`，核查 target 为上述 `64bb1728`。处置依据：[M2](DOM-COMPONENTS-P2.records/M2.md#代码评审-r2-修复)、[M3](DOM-COMPONENTS-P2.records/M3.md#代码评审-r2-修复)、[M5](DOM-COMPONENTS-P2.records/M5.md#代码评审-r2-合并复验)。R2 原报告 SHA-256：`bb0a516ddbc0cdc0635416e5c75718beca25d48934a85dcb42f33c29ada1a6fe`。

### 产品路线与本轮范围校准

用户在评审中明确：项目只有 CSR 和 SSG，并要求写入 AGENTS.md。本轮已按授权修改 [AGENTS.md:89](../../AGENTS.md#L89)：不走请求时服务端渲染、流式 SSR 或 SSR 服务部署；内部 `ssr` feature 用于构建期静态 HTML，`hydrate` 用于静态 HTML 的浏览器接管。本文中的 `--features ssr` 和历史测试命名均按这个用途理解。

实际 SSG 页面和组件生成入口分别使用 [shell.rs:415](../../crates/rustify-docs/src/shell.rs#L415) 和 [site/ssr.rs:140](../../crates/rustify-docs/src/site/ssr.rs#L140) 的同步 `to_html()`；[作者文档](../docs-ssg.md#L994)也要求初始生成同步、确定。评审前段额外调查的 `to_html_stream_*` / Suspense 场景不在这些入口内，已排除，不列 finding、不作为验收条件或补证要求。这是范围纠正，不是用户豁免缺陷。

## 上轮问题复审

沿用来源编号；R1 来源为[初审](DOM-COMPONENTS-P2.code-review.md)，R2 已逐项复核，本轮在此基础上检查修复与直接回归。

| 来源 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01，P2 | reset 后旧 Draft 再次提交；R2 已关闭 | `89ea6be2` 的成功 reset 通知保留 | **已解决** | 本轮 raw 浏览器的 Number/Color 聚焦 reset、blur/Enter/submit、拒绝及 Busy 对照全部通过；相关实现本次未改 | 无 |
| R1 / F-02，P2 | 缺上下文部件 panic；R2 已关闭 | `89ea6be2` 的恢复分支保留 | **已解决** | 10 项 missing_context_ssr、raw 条件挂载及 Select 属性转发/取消通过 | 无 |
| R1 / F-03，P2 | 非零 min 的合法小数间隔被误拒绝；R2 已关闭原复现 | `9658e5d9` 保留普通格点的整数步序号 | **已解决** | 36 项 primitives 单测及 raw 小数 Slider 通过，合法 100.2/100.3、候选和 preserve_order 的原回归保持通过 | 无 |
| R1 / F-04，P2，经 R2 接续 | CSR 已修，延迟子树的静态 HTML Label 仍失配；R2 部分解决 | `9658e5d9` 的字段局部缓冲与最终引用替换 | **已解决** | [FieldHtml](../../crates/rustify-components/src/form/ssr_field.rs#L48)在同步子树生成后完成关联；34 项 forms_ssr 覆盖 direct/Show/For/闭包、嵌套和转义；本轮禁 JS/接管检查覆盖 3 scopes × 4 声明方式的 label.control、名称、点击聚焦、原节点及 ID；raw 动态替换/legacy 首错焦点通过 | 无 |
| R1 / F-05，P2 | Input 重挂载后错误判重复；R2 已关闭 | `89ea6be2` 的存活 editor 登记保留 | **已解决** | raw 多轮挂载、分别移除重复 Input 后恢复及首错聚焦通过；本次未改 editor 登记 | 无 |
| R2 / F-06，P2 | 截断末端被计为完整步距；R2 新增 | `9658e5d9` 对末端使用实际分数容量，限位向可行格点收紧 | **已解决** | [index/bounded](../../crates/rustify-components/src/primitives/slider/model.rs#L70)、[bounds](../../crates/rustify-components/src/primitives/slider/model.rs#L232)及本轮单测：100.2/100.26 的 0.1 约束拒绝、0.26 范围不足三步拒绝、候选/顺序边界、零间距、向下 snap 和大 min/小 step 均通过 | 无 |
| R1 / U-01，经 R2 关闭 | 活动 Slider Escape 与父 Layer 优先级待明确 | 保留既有用户决定及计划 §5.3 | **已解决** | 本轮 raw 键盘/指针 Escape 用例通过：取消一次、零 commit、父层关闭 | 无；不计 finding 修复数 |
| R2 / 验证边界 | 完整应用、Docs、GPU、性能与人工范围未独立重跑 | M5 保留作者证据与复用边界 | **本轮未复核**（其余整组范围） | 本轮新增静态 HTML/接管补证，raw 也重新运行；完整入口、性能和人工剩余范围见下文 | 保留限制，不扩成整期 UI 或跨期验收结论 |

## 确认问题

未发现本轮 CSR/SSG 修复范围内可确认的新增问题。R2 两项待修问题均已解决；R1 已关闭问题未见相关回归。没有 P0–P3 findings。

## 待核实 / 待决策项

- 无影响本轮修复结论的待决事项。SSR 服务和流式渲染已明确排除，不要求为这些场景修复或补验。
- 未重跑完整 workspace、Studio/Catalog/Workbench、完整 Docs CLI/islands、GPU UI 和性能 evidence；只核对了本轮性能 JSON 与前轮的参数、样本数、请求/资源计数和体积记录，未将作者测量记为本轮独立通过。SSG 库行为由本轮静态生成测试与新产物的禁脚本/接管浏览器检查证明，不声称完成整个文档站验收。
- OS IME、VoiceOver、真机触控和浏览器 UI 缩放仍按计划留 P5 missing。本轮没有完整 UI/UX 视觉验收结论。

## 覆盖与验证

### 版本与输入

修复意图：让静态字段标签在延迟子树生成后关联实际控件，并恢复 Slider 截断端点的实际容量和最小距离。呈现路径为 DOM，覆盖 CSR 与 SSG，不增加 GPU 能力或发布范围。

- 仓根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。输入为 `$review-code docs/plan/DOM-COMPONENTS-P2.md`，按已有最高轮次 R2 进入修复效果复审。
- 登记的实现 `9fd21ca86ff2dc64bfa19059a560a92c9da2b96c`、R1 修复 `89ea6be23e0414813a41cf3917702fb30c44d3f1`、R2 修复 `9658e5d9c7680acc5cfacec3469c6ed81d231c42` 均已解析并确认是 HEAD 祖先。没有未登记产品代码提交。
- 净差异为评审对象表中的完整范围，共 15 个路径；target tree 为 `6ec537eee09ca389ba450383f112ee40bb3ea386`。`64bb17281567a646b312a4b7050219fe5a7f0c55` 只回填本计划 SHA，不作为代码 finding 对象。
- 起始有 5 个 tracked SDLC 修改及 15 个 untracked 文件（R1/R2 报告和台账），index 无变化；均盘点后排除产品代码 findings。本轮新报告与台账不纳入审查输入。用户随后明确授权的 AGENTS.md 修改单列核对，没有修改产品源码、测试、原计划或历史报告。
- 起始 11,547 个文件（全部 tracked 与起始 untracked）内容聚合 SHA-256：`8ccf662e4c6f69b302d324592e6cff6d4b52f75e37bed56f31ae83e9f9168c67`；算法为按路径排序，拼接 `path + NUL + sha256(file)二进制摘要` 后再 SHA-256。index 的 `git ls-files -s -z` 字节 SHA-256：`104bfb5f72d3a8cdec0284b23ba58bbd22a397d3669f4bbad67aa34c77bafc73`。
- 写报告前逐项复核，只有授权的 AGENTS.md 内容变化；HEAD/index 不变，产品源码及计划无变化。相同集合的新聚合为 `fda0d4a8288badfb8237ea5b62da0fdb8e0dfc1ec204e4344edc4b5a511bbbf8`；AGENTS.md SHA-256 为 `e8158518e7b8959675425e7a712ad6535de42f02da13710dd9d59905fa6addc6`。
- 当前计划 SHA-256：`f3a015abb4d8483abd0ffc080f93d9bc9c459b37dbb96880b806ecb7610b65d5`。上轮报告内容和结论保持不动。

### 覆盖清单

| 范围 | 实际核查 | 限制 |
| --- | --- | --- |
| 全部产品 Rust 修复 diff | Field/FieldBinding、Label、私有 FieldHtml、Slider model 及直接调用链；检查属性转义、owner、格点、候选和邻接边界 | 流式 API 不属于 CSR/SSG 产品验收范围 |
| 新增/变更测试与 fixture | forms_ssr、hydration spec 和 form_parts；检查用例确实断言标签原生关联和接管节点身份 | 未完整重审原始阶段所有测试 |
| CSS candidates、记录和数值产物 | 完整候选清单差异；M2/M3/M5 本轮处置；读取性能 JSON 的实际参数、40 个样本/负载、计数和资源状态 | 未重新测量性能，原始数值不代表同轮交替比较 |
| CSR 与 SSG 静态产物 | 重新构建 raw 消费者、重新生成静态 HTML、独立构建 hydrate/CSR WASM 后串行运行浏览器 | 无 SSR 服务、完整 Docs 站、真实 GPU 或 OS 辅助技术验收 |

### 本轮实际执行

工具链：Rust 1.99.0、mbx 1.15.0、Node 26.12.1。所有下列产品检查针对当前 target 的源码；浏览器按单 project 串行运行，检查 console/pageerror/失败请求及原用例的资源断言。

| 命令 / 验证 | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test forms_ssr --test missing_context_ssr --test ssr_controls` | 34 + 10 + 13，共 57 passed；构建期静态 HTML 检查 |
| `mbx test --locked -p rustify-components --lib primitives::` | 36 passed，含 19 项 Slider 逻辑测试 |
| `mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml` | 成功，随后 raw 浏览器使用新产物 |
| `RUSTIFY_XTASK=target/debug/xtask npx --no-install playwright test -c playwright.docs.config.ts --project=dom-primitives primitives.spec.ts` | 47 passed，24.5s |
| hydration fixture 静态导出、独立 hydrate/CSR release 构建与 bindgen 装配（步骤见下段） | 全部成功，浏览器使用新静态 HTML 与新 WASM/glue |
| `RUSTIFY_XTASK=target/debug/xtask npx --no-install playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` | 14 passed，26.5s；含禁 JS、显式接管及独立 CSR 比较 |
| `mbx clippy --locked -p rustify-components -p rustify-dom --all-targets -- -D warnings` | 通过 |
| `mbx clippy --locked -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings` | 通过，覆盖静态生成模块 |
| 根、tests/dom-only、tests/dom-only/hydration 各自的 `cargo fmt --all -- --check` | 全部通过，各独立 workspace 使用相应 `--manifest-path` |
| `mbx xtask css --check` | no drift，94,986 B / 4,834 candidates |
| `mbx xtask catalog --write docs/components.md --check` | no drift，105 public / 97 reference / 24 类 |
| `git diff --no-ext-diff --no-textconv --check` | 通过 |

现有产品测试合计 154 项通过。WASM 构建仍有 fallback_element / guest_click 两项既有 dead_code warning，与上轮记录一致，不作为本轮新增问题。

为避免扩跑整个 features 组，本轮按 [xtask 的 build_hydration](../../xtask/src/docs.rs#L1001) 执行受影响 fixture 的相同步骤：`CARGO_TARGET_DIR=<仓根>/target/dom-hydration-ssr mbx run --locked --quiet --manifest-path tests/dom-only/hydration/Cargo.toml --no-default-features --features ssr --bin export-hydration-assets -- target/dom/hydration`；hydrate 与 csr 分别使用 `target/dom-hydration-<mode>`，执行 `RUSTFLAGS=--cfg=web_sys_unstable_apis mbx build --locked --release --manifest-path tests/dom-only/hydration/Cargo.toml --no-default-features --features <mode> --lib --target wasm32-unknown-unknown`。随后以隔离临时小程序调用现有 wasm-bindgen-cli-support 0.2.128，按 xtask 的 `web(true)/typescript(false)/out_name("app")` 装配到各自 `pkg/`。临时包复制仓根锁并离线解析，不安装工具或修改产品依赖；没有声称完整 features 组通过。

临时目录为本机 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p2-r3-review-hqynqe4t/`，保存输入指纹、临时装配程序及前段额外调查；不是跨机器交付所需文件。额外调查使用过前轮 git archive 的隔离源码，没有 checkout/reset/stash 用户工作树。它不参与本轮 CSR/SSG 验收判定。

## 计划符合性

| 相关承诺 | 本轮结论 |
| --- | --- |
| §5.4 第 5 项、§5.7：静态 HTML 中 Label 指向实际 Control，接管保持引用与身份 | 已核实；F-04 关闭 |
| §5.3：普通格点避免浮点误拒绝，最小距离及空间不足契约始终成立 | 已核实；F-03 保持关闭，F-06 关闭 |
| reset、缺上下文恢复、存活 Input、Escape 决策的直接回归 | 已核实；旧结论保持 |
| 用户新增的明确路线限制及 AGENTS.md 写入 | 已完成；只按 CSR/SSG 实际入口评审，没有新增 SSR 产品要求 |
| V-9/V-10/V-11 及跨期人工范围 | 本轮未重新执行全部入口/性能/视觉验收，保留原记录的基线与限制 |

应保留本次修复中的普通格点整数序号、末端实际容量判断、标准 HTML 属性转义，以及浏览器存活 owner 的原清理路径；它们分别由本轮逻辑、静态生成及浏览器回归支持。

## 总体结论与下一步

**验收判定：通过验收。**

修复效果：**已解决**。R2 两项待修 finding 均实际解决；此前关闭的 R1 四项及 U-01 相关回归通过。新增 findings 为 0，没有需 apply 的问题。

本轮声明范围内的修复增量为 **`patch is correct`**，限 CSR/SSG 产品路径及上述验证。原始整阶段、UI/UX、跨期需求和发布不由这一结论自动通过。产品源码、测试、计划和旧报告保持不变；仅按用户明确要求修改了 AGENTS.md，另保存本报告与台账，未提交、推送或发布。

可选下一步：`$review-ui docs/plan/DOM-COMPONENTS-P2.md`；本计划来自 PRD，也可用 `$review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P2.md` 核对本期覆盖。这里只给命令，不自行启动。
