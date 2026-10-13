# DOCS-SSG-P2 · 开发计划完整评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P2](DOCS-SSG-P2.md) | SHA-256 `7c02d799bca5910ee37e52c292ad7f4e9f584e8adc77225f6be323b4d26b51fc`（v1，未跟踪新文件） | 全文：§0–§12、V-1–V-15、M1–M7 及所引 PRD P2 需求 |

评审日期：2026-10-13 +08:00。这是本计划的首次完整计划评审，不是复审。代码基线 `785f51ad9fdcb69e624a0c8f85d91e3541d58e90`（main，dirty：只有 `.xgent-ai/sdlc` 台账与本计划未提交），以当前工作树为事实源；评审结束时计划 hash 与 HEAD 未变。需求来源 [PRD-DOCS-SSG](../PRD-DOCS-SSG.md) 的 P2 期（§7.1 出口、§7.2 分期矩阵）。

## 1. 确认问题

### [P1] F-01 · 定义作者组件站的框架依赖与 CLI/库版本配对合同

- **定位**：[§2.1](DOCS-SSG-P2.md#21-编译矩阵与作者工程) 第195–197行（“锁文件就绪”“wasm-bindgen-cli-support =0.2.128”）；§1.2 第169行；§2.2 第201行（`protocol_version=1`）；§3 第239行（版本在 M7 固定）；§5.4 第302行；§5.5 第316行；V-8 第394行、V-13 第399行、V-15 第401行；M4 第450行、M7 第453行。关联 WORK-F1/AC-2、AC-5，WORK-N4，WORK-F16/AC-3，PRD Q2。
- **问题**：仓外作者执行 `new --template components` 后，站点必须以 git 依赖引用 rustify-docs、rustify-ui、rustify-components（PRD Q2）；CLI 则由另一次 `cargo install --git --rev` 安装。计划只给了 helper JSON 协议的版本号，没有定下：
  1. 模板 `Cargo.toml` 的依赖来源与 rev。模板嵌在 CLI 里，同一提交写不进自身 SHA；源码不带 `.git` 时 `install_revision()` 回落到上一版 rev，生成的组件站会指向没有 `site!` 的版本。
  2. 模板锁文件如何产生、首次构建是否 `--locked`、依赖取齐后能否离线构建（N4/AC-4、F15/AC-5 回归）。
  3. 仓内 fixture、M4 的禁网 V-8 和 M6 的未推送候选，怎样把依赖指向本地候选（P1 先例是把 path 改写为 `file://` git 来源）。
  4. 随版本变化的浏览器资源取哪一份：`dom-loader.js`、`dom-runtime.css`、`islands.js`、`candidates.json` 来自 CLI 内置，还是来自站点锁定的库。SDK 自己的契约是“与 wasm 同一发行版写出”。
  5. CLI 内置 bindgen 只比较 wasm-bindgen schema，站点锁定版本不同时会报错，并建议用户另装 wasm-bindgen-cli，与 WORK-N4 冲突。
  6. CLI 与站点库 rev 不一致时如何检测。只升级其中一个时，构建期可能看不出问题，读者浏览器里全部演示进入 failed，或展开态的类缺失；组件站的升级与回滚步骤也没有写。
- **证据**：[main.rs](../../crates/rustify-docs-cli/src/main.rs) 第46–60行（`install_revision` 回落 `PREVIOUS_REVISION`）；[new.rs](../../crates/rustify-docs-cli/src/new.rs) 第12–38、65–86行（只有内嵌 content 模板与占位符填充）；[runtime.rs](../../crates/rustify-ui/src/runtime.rs) 第22–27行（同版本写出契约）；[tests/dom-only/consumer/Cargo.toml](../../tests/dom-only/consumer/Cargo.toml) 第12–14行（`export-dom-assets` 先例）；[xtask/src/docs.rs](../../xtask/src/docs.rs) 第863–900行（path→git+rev 改写）；根 [Cargo.toml](../../Cargo.toml) 第32行 `wasm-bindgen = "=0.2.128"`；wasm-bindgen-cli-support 0.2.128 `src/wit/mod.rs` 第2818–2843、2909–2930行（schema 比较与安装建议）。计划全文没有模板依赖来源、rev 或锁文件生成的描述（已 grep）。四个视角各自独立报告了这一缺口。
- **最小修订**：在 §2.1 或 §5.4 增加“作者工程依赖合同”：
  - 依赖写法。例如模板依赖固定到库发布提交 R，CLI 发布在 R 之后，内置 R 与对应锁文件，`new` 原样写出；或者在 `new` 时用 `SOURCE_REVISION` 填占位符，并说明无修订号时拒绝生成组件模板。
  - 锁文件策略与 `--locked`。
  - 候选、仓内、离线验收时覆盖来源的方式。
  - 浏览器资源与候选类集合的来源：建议由 helper 从站点锁定的库导出，沿用 runtime 的同版本契约。
  - 预检：编译前用 `cargo metadata` 核对站点锁定的 wasm-bindgen 版本；helper describe 回报库 rev、`runtime::PROTOCOL`、岛协议与 candidates 摘要。不一致时给出定位诊断（应改成的 rev 与命令），并拒绝构建。
  - 模板 README 的组件站升级与回滚步骤；包名由目录名派生时的规范化规则（`new.rs` 目前直接用目录名填 `{name}`，如 `My Docs` 这类目录名到首次 dev 才失败）。
  - V-8、V-15 增加 rev 错配用例与“`new` 后 `build --locked` 不改写锁文件”。
- **复核**：改稿后，§2.1/§5.4 能回答“模板 Cargo.toml 写什么、锁文件从哪来、候选怎样覆盖、错配怎样报”。实施后：M4 用 `file://` 候选、M7 用 GitHub rev 各生成一次模板，依赖取齐后断网 build 成功；人为制造错配时 `check` 给出定向诊断；生成工程锁文件中的框架 rev 与已安装 CLI 的来源 SHA 一致。
- **置信度**：高。代码现状与计划缺失均已核实；四个视角独立得出同一结论。**阻塞**：M2 协议定稿（资源来源与握手）、M4、M7；V-1 与 M1 的 SDK 内部工作不受影响。

### [P2] F-02 · 把 `hydrate_from` 的运行前提写进 SDK 水合契约

- **定位**：§1.1 第153行（标为“已核实·足够”）；§2.3 第207行（“借 adapter 调用任意站点 Leptos 组件”）、第225行（“通过后提供与 SSR 相同上下文再调用 hydrate_from”）；ADR-1 第108–111行；§4 第248行；§5.3 第296行（slot 可含显式岛）；§5.5 第318行；§1.2 第163行。关联 WORK-F4/AC-1、AC-2，WORK-F5，C-3，PRD DEP-2/DEP-3/DEP-5。
- **问题**：照锁定版本的源码，计划依赖的四个运行前提没有写进契约：
  1. **上下文时序写反**：`hydrate_from` 用 `Owner::new_root` 新建无父的根。照现有 `mount` 先在外层 `provide_context` 再调用，ScopeRoots、OverlayStack、ActionSink、router 和 ID 分配器都到不了视图。
  2. **“任意 Leptos 组件”超出能交付的范围**：`HydrateSharedContext` 从 `__RESOLVED_RESOURCES`、`__INCOMPLETE_CHUNKS`、`__SERIALIZED_ERRORS` 读数据。这些全局量本来由内联 HydrationScripts 写入，而本计划因 CSP 禁止内联。每个根的 id 都从 0 起，同页多岛会在这些全局量上冲突；SSR 侧也不产出序列化数据。作者组件用了 Resource、Suspense、Transition、ErrorBoundary 等，就会水合失败或渲染不一致。24 类官方演示不用这些能力，所以验收测不出来。
  3. **shell 会删掉水合标记**：P1 的 `render_page` 对 shell 输出执行 `replace("<!>", "")`，并有测试断言。正文是事后拼接的，不受影响；但经 shell 渲染的 slot 若含岛，标记会被删，水合游标错位。
  4. **`style:` 在水合时是空操作**：tachys 字符串 `style:` 的 `hydrate` 直接返回，假定 SSR 已写出 `style=` 属性；而 CSP 与 P1 产物闸都禁止输出该属性。DataTable、Tree、Progress、ColorField、Splitter 的几何在水合后会缺失。§1.2 写了“动态布局用现有 class/外部 stylesheet 范式”，但没有说明这几个组件必须改造，而不只是在 SSR 时抑制。
- **证据**：leptos 0.8.22 `src/mount.rs` 第87–94行（`Owner::new_root`），对照 [SDK mount](../../crates/rustify-ui/src/mount.rs) 第146–163行；hydration_context 0.3.2 `src/hydrate.rs` 第20–29、131–136、163–178行；leptos `src/error_boundary.rs` 第89–92行、`suspense_component.rs` 第113–114行；[shell.rs](../../crates/rustify-docs/src/shell.rs) 第256–263、801行；tachys 0.2.19 `src/html/style.rs` 第569–574、697–699行；`style:` 用法见 `crates/rustify-components/src/{progress.rs:46,color_field.rs:109,tree.rs:247,data_table/view.rs:391-483,workspace/splitter.rs:151}`。
- **最小修订**：在 §2.3/ADR-1 写明以下四点：
  - 上下文在传给 `hydrate_from` 的闭包内提供，清理归 UnmountHandle 持有的根 Owner。
  - 岛内支持的组件范围。建议限定为同步 view；若要支持异步，由外部 `islands.js` 预置空全局量并定义 id 命名空间。在作者文档和诊断中说明。
  - 岛 HTML 与 slot 中岛的水合标记保留到最终产物，shell 的清理只作用于非岛部分。
  - 用了 `style:` 的组件改为 SSR 输出可编译的类、水合后再用 CSSOM。
- **复核**：改稿后 §2.3 不再写“任意”，上下文时序正确。实施后，V-2/V-5 增加三类用例：使用异步能力的作者组件反例，其诊断或失败表现符合定义；slot 内的岛；DataTable/Tree 水合后几何与首帧一致，且没有 `style=`。
- **置信度**：机制为高（源码级证据）；异步全局量缺失时是抛 ReferenceError 还是读到空值，具体形态待核实。**阻塞**：M1 根契约、M2 岛协议。

### [P2] F-03 · 定义 SSR/水合一致性的覆盖集与对照判据

- **定位**：M1 第447行（“本期24类初态SSR”）；V-2 第388行；V-5 第391行；§8 第369行（根契约预检可“避免错绑控件”）；§11 第466行（“以 V-2/V-5 先验证最复杂类”）；M5 第451行；V-10 第396行。关联 WORK-F4/AC-2（“交互结果与在普通 Leptos 应用中一致”）、WORK-F16/AC-2、DEP-2。
- **问题**：
  - M1 的“24 类初态 SSR”是全部 24 类，还是 §11 所说的“最复杂类”，无法判定。
  - Select、Tooltip、Menu、Dialog、DataTable、Tree、Toast、NumberField、ColorField 九类目前只在 wasm 下导出。没被选中的类会在 M5/M6 才第一次走 SSR→水合，届时若改 SDK 或组件，M2–M4 已通过的检查都要重跑。
  - 对照判据也缺失：V-2/V-5 只断言“能响应输入”和“多岛互不影响”，没有与 CSR mount 的对照，无法证明 AC-2 前半句。
  - tachys 水合只用 `Element::cast_from` 检查是否为元素，不比对标签或结构。同为元素的错位会把事件静默绑到错误节点上，§8 第369行的检测承诺在 release 构建下不成立。
- **证据**：[catalog.rs](../../crates/rustify-components/src/catalog.rs) 第30–55行（24 类）；[lib.rs](../../crates/rustify-components/src/lib.rs) 第15–101行（wasm 门控）；tachys 0.2.19 `src/html/element/mod.rs` 第534–543行、`src/hydration.rs` 第159行。
- **最小修订**：给 V-2 写明覆盖集与判据：CATALOG 24 类各取一个代表组件，在确定初态下检查——宿主 SSR 输出；接管后节点身份、ID/ARIA 一致；规范化 outerHTML 与同 props 的 CSR mount 相同；同一输入后公开状态与 CSR 一致。或者 M1 只覆盖明确列出的类（至少包括九个 wasm-only 类），其余类的同等断言写进 M5 退出与 V-10。§8 该行改为“运行期只能检测类型错位，结构一致由测试期比对保证”。
- **复核**：改稿后 M1 能逐类对照 CATALOG 判定退出。实施后，每类都有 SSR/CSR 对照记录；一个故意改变标签结构的反例能被测试期比对发现。
- **置信度**：文本歧义与源码行为均为高；返工概率为中。**阻塞**：M1 退出判定。

### [P2] F-04 · 解除 M1 退出检查对 M2 才建立的组件 fixture 的依赖

- **定位**：V-2 第388行（命令含“组件 fixture 的公开接管浏览器检查”，M1 局部执行）；M1 第447行；M2 第448行（前置“M1…成果+V-2”）；§1.2 第169、171行（组件 fixture 与 `islands.spec.ts` 都是 ★ 新增，依赖 `site!`、`rust_site` 和 CLI 构建，属于 M2 的交付）。
- **问题**：M1 收口要跑 V-2 的浏览器部分，而其载体要到 M2 才存在，M2 又以 V-2 为前置，两者成环。实施者只能临场自造一个未登记的“SSR + hydrate”页面 harness，自行决定构建方式、project 和端口。
- **证据**：[playwright.docs.config.ts](../../playwright.docs.config.ts) 第80–84、111行（`dom-only` 只服务 CSR consumer）；[serve.sh](../../tests/docs-ssg/serve.sh) 第29行（只复制 content fixture）；`xtask/src/docs.rs::external` 没有 hydrate 构建。
- **最小修订**：二选一。
  - (a) 在 M1 登记一个最小水合 harness，例如在 `tests/dom-only` 增加宿主 SSR 输出和 hydrate wasm 页面，写明 project、端口与命令。V-2 的 M1 部分只用它，组件 fixture 的浏览器检查移到 V-5。
  - (b) M1 退出只要求宿主 SSR、hydrate wasm32 编译与测试、拒绝语义；首次真实浏览器接管改记在 M2 退出，M2 前置相应改为“M1 宿主/编译检查”。
- **复核**：改稿后，V-2 每个执行时点引用的载体在该时点都已存在，依赖图无环。
- **置信度**：高。**阻塞**：M1 收口。

### [P2] F-05 · 设计浏览器检查的站点供给、project 映射与静态宿主

- **定位**：V-5 第391行（`--project=docs-subpath islands.spec.ts`，“扩展 fixture 服务选择”）；V-6 第392行；V-9 第395行（`--project=docs-dev rust-dev.spec.ts`）；V-11 第397行；§1.2 第171行（只列了 `serve.sh`）；M5 第451行（“对应局部 V-11”）。
- **问题**：
  1. `serve.sh` 固定构建 content fixture，而静态 project 的 `testMatch` 是 `docs-ssg/**/*.spec.ts`。新增的 islands/custom spec 会在 root、subpath、meta、firefox、webkit 五个 project 上对着 content fixture 跑；如果改成用变量切换站点，P1 回归 spec 又会跑到组件 fixture 或官方站上。
  2. `docs-dev` 只匹配 `dev*.spec.ts`，静态 project 也只排除这个模式。所以 V-9 的命令会报 No tests found，而 `rust-dev.spec.ts` 反被三个静态 project 收进去，M6 跑 V-11 时会失败。
  3. `docs-meta` 的 `static-host.mjs` 类型表没有 `.wasm` 与字体扩展名，会回落为 `application/octet-stream`。DOM loader 用 `WebAssembly.compileStreaming`，遇到这个 MIME 会拒绝，meta project 上所有岛都会进入 failed。
  4. content fixture、组件 fixture、官方站各用哪个 project 和端口没有分配；现有端口 4190–4193 已占用。
- **证据**：[playwright.docs.config.ts](../../playwright.docs.config.ts) 第85–107行；[serve.sh](../../tests/docs-ssg/serve.sh) 第29–36行；[static-host.mjs](../../tests/docs-ssg/static-host.mjs) 第14–23、79行；[dom-loader.js](../../crates/rustify-ui/web/dom-loader.js) 第67行；preview 已有对应映射（[preview.rs](../../crates/rustify-docs-cli/src/preview.rs) 第56行）。
- **最小修订**：在 §9.1 或 §1.2 写定以下内容，由 M2 承担 `serve.sh`、config 与 `static-host.mjs` 的改动：
  - 每个站点源对应的 project、端口、serve 变体和 `testMatch`/`testIgnore`；
  - spec 命名，例如改名为 `dev-rust.spec.ts`；
  - firefox/webkit 辅助 project 是否收新 spec；
  - 补上 `.wasm`/字体的 MIME。
- **复核**：实施期对每个 project 跑 `--list`，每个 spec 只出现在预期的 project；docs-meta 下岛页的 wasm 响应为 `application/wasm`。
- **置信度**：高（配置与脚本实况，主 agent 已复核）。**阻塞**：M2 的 V-5，以及之后所有浏览器门。

### [P2] F-06 · 定下 `site!` 的实现形态与冷态清单解析的单一来源

- **定位**：D3 第95行；§1.2 第165行（只有 `src/site.rs`）；§2 第183行；§2.1 第196行；§2.3 第205–223行；§3 第232行；M2 第448行。关联 WORK-F4/AC-3、WORK-F15/AC-3。
- **问题**：`site!` 要为每个组件生成带 serde 默认值和 `deny_unknown_fields` 的 props 类型，CLI 还要在编译前用 syn 解析同一份声明。两条路线代价都不小：
  - 用过程宏：Cargo 要求过程宏单独成 crate，§1.2 没有列出。
  - 用 `macro_rules`：CLI 需要另写一份语法解析器，冷态预检与真实展开可能对不上。

  计划提到 helper 有“描述”请求，但没有要求拿它的结果与静态清单比对。
- **证据**：`crates/` 下没有任何 `proc-macro = true` 的 crate；`syn 2.0.119`、`proc-macro2 1.0.107` 已在锁文件中。
- **最小修订**：选定实现形态，例如新增 `crates/rustify-docs-macros`。语法模块只写一份，由宏与 CLI 预检共用；编译后 describe 结果须与静态清单完全一致，不一致记为错误。§1.2 补上该路径。
- **复核**：V-3 增加“静态清单与 describe 一致”的用例，以及一个故意不一致的反例。
- **置信度**：高。**阻塞**：M2。

### [P2] F-07 · 固定 Rust 编译档位，并定义冷启动与增量编译的可见行为

- **定位**：§2.1 第195行（只写了“host profile 使用 unwind，wasm release 使用 abort”）；§5.4 第305–307行；§7 WORK-N2 第352行；§9.2 第408行；§9.3 第414行；V-13 第399行。关联 WORK-N2/AC-4、WORK-F13/AC-4、WORK-F1/AC-5、PRD §2.4。
- **问题**：
  - 组件站是独立 workspace，profile 只能写在模板根。计划没有定 dev 与 build 各用哪个档位、CLI 用什么参数调用、bindgen 用哪些选项，也没有说 N2 在哪个档位下测量。
  - 照搬仓库的 release 档（`lto="fat"`、`codegen-units=1`），作者每次保存都要对整个图重做 LTO。这段时间在 N2 计时起点之前，但直接拖慢 F13 体验与 UAT-15 的 15 分钟。改用 debug 档又会让 wasm 体积和浏览器编译挤进 ≤2 s 预算。
  - §9.3 只剔除了首次下载与编译，增量编译的等待是否计入 15 分钟没有写。
  - P1 的 dev 要等首代完成才打印地址；组件站冷编译可能长达数分钟，期间既无地址也无进度。
- **证据**：根 [Cargo.toml](../../Cargo.toml) 第41–44行；[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 第885–915行；P1 安装耗时 40 s 至 2 m 05 s（[M9](../validation/docs-ssg-p1/M9.md) 及 `m9/` 日志）。
- **最小修订**：
  - 在模板与官方站固定命名 profile：dev 用的 wasm 档开启增量编译、不带调试信息、`panic=abort`、适中的 opt-level；宿主 helper 用 unwind；build 用 release。
  - 写明 CLI 的 `--profile` 调用约定和 bindgen 选项。
  - V-9/V-12 记录测量所用档位；M4 记录参考机上增量编译耗时的五次中位，作为观测项。
  - §9.3 写明增量编译计入 15 分钟。
  - dev 先打印地址，显示“首次编译中”和单行终端进度。
- **复核**：改稿后档位与计时口径唯一。实施后，M4 记录有编译耗时与冷启动截图，V-13 按写明的口径计时。
- **置信度**：口径缺口为高；耗时风险为推断，置信度中，需 spike S3 实测。**阻塞**：M4 开始前，最好在 M2 收口前定。

### [P2] F-08 · 在“输入快照构建”与“原地构建加提交前指纹核对”之间二选一

- **定位**：§2 第184行（“输入为一次站点快照”）；§3 第235行；§5.4 第307–308行。关联 WORK-F5/AC-2、WORK-F13/AC-4、WORK-N2/AC-4。
- **问题**：第308行引入私有临时目录做输入快照，第307行又规定提交前核对输入指纹、过时代丢弃，两条路线并存。若每代把源码复制到临时目录再编译：
  - path 包的身份与 mtime 每次都变，站点 crate 和官方站的本地 path 依赖（SDK、组件、rustify-docs）每轮都要重编，耗时是分钟级；
  - Rust 错误指向快照路径，而 F5/AC-2 要求作者源文件的位置；
  - 源码切片取自快照还是原文件也不明确。
- **最小修订**：写定一条路线。建议原地构建，编译前与提交前各做一次输入摘要，不一致就丢弃该代并重建；若坚持快照，就写明固定快照目录、保留 mtime、诊断路径重映射，以及切片来源。
- **复核**：V-9 断言编译错误指向作者文件的行列，第二次增量构建不重编 path 依赖（用 Cargo JSON 的 fresh 计数判断）。
- **置信度**：原文歧义为高，性能后果为中。**阻塞**：M2/M4。

### [P2] F-09 · 定下主题字体的字节来源、回退字体与产物路径

- **定位**：D6 第98行；§2 第186行；§4 第251行；§5.5 第312–314行（“引用字体随产物打包为同源不可变资源”）；§8 第371行；V-7 第393行；WORK-N4 第78、354行。关联 WORK-F8/AC-5、WORK-N4/AC-2。
- **问题**：
  - SDK 的 `FONT_FACES` 指向 `makepad_widgets/resources/*.ttf`，这些文件只在 Makepad fork 目录里；git 安装的 CLI 和仓外作者手里都没有，CLI 也没有内置。
  - 只要主题含有打包字体，`export_css` 就会追加 WenKai（约 19.0 MB）和 NotoColorEmoji（约 10.6 MB）两个 glyph 回退字体。
  - 产物路径带 `makepad_widgets` 字样，与 WORK-N4 的“产物无 Makepad”检查口径冲突。
  - `theme.document` 同样对没有 Rust 的内容站开放。

  实施者到 M3 必须自行决定是否把约 30 MB 字体嵌进 CLI、从哪里读取、路径怎样命名，以及许可声明。
- **证据**：[fonts.rs](../../crates/rustify-ui/src/theme/fonts.rs) 第34–72行；[codec.rs](../../crates/rustify-ui/src/theme/codec.rs) 第228–266行；`makepad/widgets/resources/` 文件大小；`crates/` 下没有任何 ttf/woff2；现有 theme-export consumer 借 cargo_makepad 打包字体（`tests/theme-export/build.sh` 第33行），不能当先例。
- **最小修订**：在 §5.5 写明：
  - 字体来源，例如 CLI 只内置受支持的字面，或要求作者放进站点 static 目录；
  - 回退字体策略，例如文档站默认不带，作为显式选项，并给出体积预算；
  - 产物路径映射为中性命名的 `assets/fonts/<digest>.<ext>`；
  - 许可声明。

  V-7/V-10 增加两项断言：字体字节与 Theme Studio 一致；产物中没有 makepad 路径。
- **复核**：在导入主题的纯内容站上，git 安装环境禁网执行 build，字体全部同源，N4 产物检查通过。
- **置信度**：高。**阻塞**：M3。

### [P2] F-10 · 补全 `content.include` 契约：路径基准、排除文件的链接与读取范围

- **定位**：§3 第231行（`content.include` “均项目相对”）对照 §5.6 第322行（列裸文件名 `quickstart.md` 等）；§5.6 第324行（未发布文件“走现有 repo URL 映射”）；§12 第483行；§1.1 第152行。关联 WORK-F16/AC-1、AC-3，WORK-N2/AC-4，WORK-N7。
- **问题**：
  1. 路径基准两处互相冲突，无法同时作为配置契约成立。
  2. “现有 repo URL 映射”只作用于 content 目录**外**的文件；content 目录内、不在页面集合中的 `.md` 会报 `broken-link`。官方站 `content.dir=docs`，指南链向的 `plan/…`、`validation/…` 都在 docs 内：utility.md 第9、171行，vellum.md 第3、5、194行。`check` 因此至少报 5 个错误，F16/AC-3 要求的零错误无法达成；实施者也可能为了通过 check 把计划、验证文件发布出去，违背 AC-1。另外 utility.md 第76行链接的 `protocol.rs:96` 文件本身不存在。
  3. `snapshot` 会 `std::fs::read` content 目录下的每个文件，dev 也递归监视该目录。docs 目前 632 MB，其中 `docs/validation` 占 629 MB。白名单如果只过滤 Markdown，每轮 build/dev 都要读这 632 MB，实施期往 validation 写证据还会不断触发重建。
  4. README 介绍里 `docs/quickstart.md` 链接的解析基准没有定义：README 位于 content 目录外，在定制首页中渲染。
- **证据**：[content.rs](../../crates/rustify-docs/src/content.rs) 第340–375行（逐文件读取）、第1405–1431行（content 内 `.md` 查不到即 broken-link）、第1440–1478行（只有 content 外才映射到 repo）；[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 第93–98行；`du -sh docs docs/validation` 为 632M/629M；[README.md](../../README.md) 第7行。
- **最小修订**：在 §3/§5.6 写定：
  - include 的路径基准；
  - 未列入的 content 内文件既不读取也不监视，被链接时映射为仓库 URL——这是**新增**规则，§5.6 与 §12 不再称其为现有能力；
  - 白名单同样约束非 Markdown 资源的发现；
  - README 片段的链接以仓库根为基准，指向已发布指南时映射到站内页面；
  - 把修正 utility.md 第76行列为 M5 的内容任务。
- **复核**：改稿后 §3 与 §5.6 一致。实施后，官方站 `check --format json` 无 broken-link；V-10 断言上述链接输出仓库 URL；dev 期间往 `docs/validation` 写文件不触发重建；产物中没有排除的文件。
- **置信度**：高（代码行为与数据量均已实测，主 agent 已复核）。**阻塞**：M5、V-10。

### [P2] F-11 · 确定官方站 24 类分类页的承载形式、导航入口与首页组成

- **定位**：§5.6 第324、326行；§2.3 第223行（pages 只声明 route/title/description/渲染函数）；§3 第231行；§5.3 第294行；M5 第451行。关联 WORK-F16/AC-2、UAT-11、PRD §2.4 首页要求。
- **问题**：
  - **承载形式**：include 只列 12 份指南，24 个分类页只能是定制页。但 `site!` 的 pages 没有让定制页的 Rust 视图放入“演示块”（画面、状态、函数/全文件源码、复制）的公开 API；演示块目前只能由 Markdown 的 `rustify-demo` 围栏产生。
  - **侧栏**：`nav.groups.items` 只接受内容路径，新增的配置键也不能把定制页放进侧栏或排序。分类页因此可能根本找不到入口。
  - **首页**：只定了“读取 README 介绍原文”，没说已批准阅读表面中首页的形态（主操作、特性网格）是否适用；README 第7行“Start here to build an example”脱离 README 后指代错误。

  实施者要在 M5 临时扩展 `site!` 和配置合同。
- **证据**：[docs/docs-ssg.md](../docs-ssg.md) 第215–244行；[config.rs](../../crates/rustify-docs/src/config.rs) 第549–580行（`dir` 或 `items`，均为内容路径）；[阅读表面 brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 第8、13行。
- **最小修订**：在 §5.6 定下分类页的承载方式：定制页可放演示块，或分类 Markdown 进入 include（例如放在 `site/` 下的独立 content 来源）。在 §3 增加定制页进入导航的写法。定下首页的主操作与组成。V-10/V-11 断言侧栏集合等于“12 份指南 + 24 类”，1280 px 下主操作在首屏内。
- **复核**：改稿后 M5 无需新增未登记的公开 API。实施后，读者从首页或侧栏可以到达每个分类页。
- **置信度**：高。**阻塞**：M5；`site!` 的 API 面要在 M2 协议定稿时一起考虑。

### [P2] F-12 · 登记 `docs/vellum.md` 是中文这一事实及其处理决定

- **定位**：§5.6 第326行（“默认首页/指南/分类页均为英文”）、第324行（“无需…加 frontmatter”）；账本 WORK-F16 第74行、WORK-N5 第79行。关联 PRD §7.1 P2“以英文上线”、WORK-N5/AC-3（lang 正确）。
- **问题**：`docs/vellum.md` 正文是中文，其余 11 份指南没有汉字。在 `site.lang=en` 下它会以 `lang="en"` 发布，读屏器会用英文语音读中文，WORK-N5/AC-3 的回归失败，也与“英文站”冲突。可选做法都改变内容或范围，需要用户决定：
  - 把源文件翻译成英文：单一来源，GitHub 上也变英文；
  - 加 frontmatter `lang: zh`：P1 支持，但该页界面会变成中文；
  - 调整白名单：与 F16/AC-1 列出的 12 份冲突。
- **证据**：[docs/vellum.md](../vellum.md) 第1–5行；delivery 视角对各指南汉字数的统计；P1 的页面 `lang` 支持见 [docs/docs-ssg.md](../docs-ssg.md) 第260行。
- **最小修订**：在 §5.6 写明这一事实，在 §11 登记为待用户决定事项，最晚在 M5 前确认，并给出推荐做法；V-10/V-11 增加断言“每页 lang 与正文语言一致”。
- **复核**：改稿后 §5.6 不再笼统称“指南均为英文”。实施后，官方站每页 `lang` 与正文语言一致。
- **置信度**：高。**阻塞**：M5 的内容出口；不阻塞开工。

### [P2] F-13 · 补全 P2 对 Firefox 复核的依据，或按 PRD 纳入当期矩阵

- **定位**：C-6 第87行（来源写“用户 2026-10-12 确认”）；§0.5 第139行；V-11 第397行；§9.3 第416行；对照 M6 第452行（“范围豁免须有用户明确依据”）。关联 WORK-N3/AC-4（每期出口复核，含桌面 Firefox 最新稳定版）。
- **问题**：
  - PRD 写明 N3 的 AC-2、AC-4 在每期出口按当期能力复核，AC-4 包含桌面 Firefox。
  - P1 的 D-01 只批准“本期不做 Firefox，留后续期”；G-05 的处置是“没有依据指定它在 P2 恢复”，即没有指定恢复期，不是 P2 豁免。
  - 本计划初稿事件只记录了用户 2026-10-12 确认三个数值门槛，没有 Firefox 的用户原话。

  计划把“P2 不恢复 Firefox”当作已确认范围，排进了 V-11；M6 又要求范围豁免必须有用户依据。两者无法同时成立，到 M6 收口时只能临时去问用户。
- **证据**：[PRD WORK-N3](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容) 第657、665、672行；[P1 差异报告 G-05](DOCS-SSG-P1.prd-dev-gaps.md#g-05--firefox-从-p1-验收矩阵后移--b) 第59–65行与 D-01 第197行；[初稿事件](../../.xgent-ai/sdlc/events/plan/c0c94e42-32f2-4636-ae52-f3df76d4a463/129bb618-864e-4b9f-9a5a-e79f34186703.md) 第22、25行。
- **最小修订**：二选一。
  - 按 PRD 默认，把桌面 Firefox 稳定版纳入 V-11，Playwright Firefox 引擎只作辅助证据；
  - 在 §11 登记为待用户决定，最晚在 M6/V-11 前确认，并把 C-6 的来源改为实际依据。
- **复核**：改稿后 C-6、§0.5、V-11 与 M6 的豁免条款口径一致。
- **置信度**：中（PRD 自身措辞有歧义，但计划所称的用户确认找不到出处）。**阻塞**：M6；不阻塞开工。

### [P2] F-14 · 先冻结候选再做首次用户验收，并把候选分发列为前置

- **定位**：V-13 第399行（M6，以组件模板 README 为唯一引导）；§8 第375行（发布顺序“首次用户出口 → 固定 CLI 版本/可获取 git SHA 和文档”）；M7 第453行（才“固定版本/SHA 与 README/模板”，复核清单不含 V-13）；§9.3 第414行；§11 第463–464行。关联 WORK-F1/AC-5、UAT-15。
- **问题**：
  - 模板随 CLI 二进制分发。M7 一旦修改 README、模板或版本，就产生了新的产品候选，而 F1/AC-5 的证据对应的是旧引导文本，M7 又不复核 V-13。
  - 受试者环境只有 Rust 与 CLI，M6 时需要一个他们能安装的候选 SHA。P1 推送曾需要用户明确授权，这一前置在 P2 没有登记。
- **证据**：[M9 记录](../validation/docs-ssg-p1/M9.md)「远端收口」；PRD UAT-15 第858行。
- **最小修订**：把“固定版本、README 与模板中的安装命令、候选 SHA”提到 M6 收口第一步，V-11/V-12/V-13 都在冻结候选上执行；M7 只做推送、安装、部署与线上验收，若仍改模板或 README，就把 V-13 加入复核清单或写明复用依据。§11 增加一行“候选推送与向受试者提供的方式及授权”，最晚在 M6 前确认。
- **复核**：M6 记录的候选 SHA 等于 M7 V-15 安装的 SHA；如有差异，逐项列出且只涉及非产品文件。
- **置信度**：高。**阻塞**：M6/M7 对 F1/AC-5 的关闭。

### [P2] F-15 · 为“未修改的组件模板在 dev 中可交互”补浏览器验收

- **定位**：账本 WORK-F1 第67行（V-8/V-13/V-15）；V-8 第394行（xtask，不开浏览器）；V-9 第395行（“临时副本”看起来是 fixture）；V-15 第401行（只核对安装与来源）；§1.2 第169行（模板与 fixture 是两套）。关联 WORK-F1/AC-2、UAT-3。
- **问题**：PRD 要求新建后不做任何修改，就能在开发预览中看到可交互的演示及其源码。现有检查没有一项去打开未修改模板的 dev 预览、操作演示、核对源码页签；V-13 的受试者会修改工程，不能替代。模板缺陷可能拖到 M6/M7 才暴露。
- **证据**：[PRD WORK-F1](../PRD-DOCS-SSG.md#work-f1-安装-cli-并新建可运行的站点工程) 第188行。
- **最小修订**：在 V-8 或 V-9 增加一条：候选 CLI 执行 `new --template components`，再 `dev --port 0`；在 docs-dev 中确认演示进入 ready、一次输入改变状态、源码页签与函数原文一致。M7 用安装后的二进制（`RUSTIFY_DOCS_BIN`）重跑一次。
- **复核**：M4 记录有该检查的浏览器证据。
- **置信度**：中高。**阻塞**：F1/AC-2 在 M4、M7 的关闭。

### [P2] F-16 · 映射 P2 相关 UAT，并在 M6 串起组件站维护者的连续主路径

- **定位**：全文只有 V-13 第399行引用了 UAT-15；M6 第452行“人工 UAT”没有定义范围。关联 PRD §6.3 UAT-3、UAT-4、UAT-9、UAT-10、UAT-11 的 P2 部分。
- **问题**：AC 都拆到了 fixture 级检查，但 UAT-3 定义的连续路径没有在同一候选上串起来：在同一工程里新增演示、注册定制页、覆盖页脚，然后改坏演示——预览保留上一版，`build` 非零退出且不产生新产物。阶段收口缺少跨模块的用户主路径验证。读者侧的 UAT 大体落在 V-5/V-10/V-11，但没有映射表可以追溯。
- **证据**：[PRD §6.3](../PRD-DOCS-SSG.md#63-uat-场景预期状态与验收条件) 第846–858行。
- **最小修订**：在 §9.1 增加 UAT→V 映射；在 M6 增加一条（或扩展 V-9/V-11）检查：在冻结候选生成的组件模板上，按 UAT-3 连续执行，可以自动化。
- **复核**：M6 记录能逐条对应 UAT-3/4/9/10/11 的 P2 部分。
- **置信度**：中。**阻塞**：M6 收口的完整性。

### [P2] F-17 · 为新增界面安排设计阶段与完成评审

- **定位**：§6 第330–345行；C-4 第85行引用了 AGENTS.md「前端 UI 开发与验收」，但全文没有 impeccable 设计或 `DESIGN.docs-ssg.md` 更新的任务。
- **问题**：已批准的阅读表面 brief 明确把 components、demos 留给后续期。P2 新增的界面都没有设计依据：演示块框体、状态区位置与文案、Reload 入口、“函数/全文件”页签是否复用 code-group、长文件高度、client-only 占位、分类页与首页版式、dev 的 building 指示、Rust 多行错误如何放进面板。现有 dev 面板没有 building 状态。V-11 因此缺少视觉与状态的判定基准，这些都留给实施者决定。
- **证据**：[AGENTS.md](../../AGENTS.md) 第120行；[阅读表面 brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 第8行；`DESIGN.docs-ssg.md` 与 `assets/dev.js` 中没有 building 状态；P1 计划曾安排 brief、finish review 与 sidecar。
- **最小修订**：在 M2（岛状态、演示块）和 M5（分类页、首页）之前各加一步 impeccable 设计，扩展阅读表面与 dev 两份 brief；实现后做 finish review 并更新 `DESIGN.docs-ssg.md`。V-11 引用这些记录作为验收基准。
- **复核**：改稿后，§6 每个状态都有对应的设计记录落点。
- **置信度**：高。**阻塞**：M2 的岛状态 UI、M5 的页面版式。

### [P2] F-18 · 给作者文档和模板 README 定内容、归属里程碑与试走

- **定位**：§5.4 第302行（只写了模板含 README）；§1.2 第172行（README、`docs/docs-ssg.md`、`docs/themes.md` 只列了文件名）；V-13 第399行；§9.3 第414行；M4 第450行；M7 第453行；V-15 第401行。关联 WORK-F1/AC-5、WORK-F16/AC-3（“部署说明写明该托管交付的策略范围”）。
- **问题**：
  - 模板 README 是 UAT 的唯一引导，必须教会新增演示的四步：写函数 → 在 `site!` 注册 → 放 `rustify-demo` 围栏 → 在 dev 中查看与排错。计划没有内容清单，也没有在招募之前用 README 走一遍；README 不够用，要到真人失败后才发现，复验又要重新招人。
  - 受试者的第一条命令（`new --template components`）从哪里来没写。
  - `docs/docs-ssg.md` 的更新（围栏语法、`site!`、client-only、定制页与插槽、`theme.document`、`include`，以及“check 不写文件”对 Rust 站点不再准确）没有归属里程碑和验收。
  - F16/AC-3 要求的部署说明没有映射到任何检查。
- **证据**：[docs/docs-ssg.md](../docs-ssg.md) 第75–76行；[PRD WORK-F16](../PRD-DOCS-SSG.md#work-f16-rustify-ui-读者在官方文档站阅读指南与运行组件演示) 第572行。
- **最小修订**：
  - §5.4 列出模板 README 的必含章节。
  - M4 退出增加一次 README 试走，只作预检，不替代 V-13。
  - V-13 写明第一条命令的来源。
  - 文档更新分到 M4（作者合同）与 M7（版本、部署），以链接检查和“示例与模板一致”作为验收。
  - V-15 断言部署说明写明 meta 策略的范围。
- **复核**：M4、M7 的记录各有对应文档的检查。
- **置信度**：中高。**阻塞**：M4、M6、M7 的作者体验出口。

### [P3] F-19 · 把“全部岛立即水合 ≤1 s”的 M2 探测改为必做风险门

- **定位**：V-12 第398行（“M2 可做提前探测”）；§3 第236行（同一代 HTML 引用同一岛资源组）；WORK-N1 第351行；§9.2 第410行（不达标时“拆分资源”）。
- **问题**：拆分资源要改 M2 的资源组与协议，以及 M5 的站点构建，但要到 M6 收口才会暴露。DOM-only release wasm 的体积目前没有实测，现有记录只有带 Makepad 的约 8 MB。
- **修订**：M2 用链接全部 24 类组件的 wasm 作代理，记录 gzip 体积与 FCP→ready 的分项及预算余量；余量不足时，在 M3/M4 之前决定拆分策略。
- **置信度**：中。

### [P3] F-20 · 把过粗的前置细化到实际成果与检查 ID

- **定位**：M4 第450行（前置含 V-5）；M5 第451行（前置含 M4 的 V-8/V-9）；M7 第453行（前置为“M6 全部出口”）。
- **问题**：
  - 模板与 dev Rust 实际只需要 V-4 与岛启动可用，故障态浏览器门不是前置。
  - 官方站只需要 M2 的 check/build 和 M3 的页面图与主题；24 个演示函数只需要 M1 对应类的 SSR 与 `site!` 合同。
  - workflow 草案、Pages 启用申请、推送授权申请都不依赖 V-13。

  这些边写得过粗，压掉了可以安全并行的空间（见 §4 的依赖边与波次）。
- **置信度**：中高。

### [P3] F-21 · 让各里程碑的收口条目与检查时点、交付物一一对应

- **定位**：V-5 第391行、V-10 第396行（自述在 M6 复跑）对照 M6 第452行；V-9 第395行（自述在 M7 回归）对照 M7 第453行；M3 第449行；§9.2 第408–409行；§1.2 第159–172行。
- **问题**：
  - M6 退出没有列出 V-5/V-10；M7 的复核清单没有列 V-9。
  - M3 改动了所有岛共用的主题上下文，退出却不要求复跑 V-5。
  - V-12 需要的 dev 事件 epoch 毫秒字段没有落到 M4 的交付。
  - 参考站 B 加入演示页需要改 `xtask/src/docs_corpus.rs`，§1.2 没有登记。
  - `.github/workflows/verify.yml` 的分目标 CI 与 `docs/themes.md` 没有分配里程碑。
  - WORK-N2 在 P2 终验，但其 AC-2（`new` 到首页 ≤60 s）不在 §9.2 的回归清单里，而 M4 会改 `new` 与 doctor。
- **修订**：在相应里程碑的退出条件、§1.2 与 §9.2 补齐上述条目。
- **置信度**：高。

### [P3] F-22 · 写明 V-7 与 Theme Studio 对照的入口和资源

- **定位**：V-7 第393行。
- **问题**：theme-studio 是 GPU 示例（features 含 `gpu`）。对照需要先 `build-web --release`，再在根 `playwright.config.ts` 的 `theme-studio` project 中运行；按 CLAUDE.md，它不能与 docs 的 Playwright 运行并发。计划没有写入口、资源与顺序。
- **修订**：写明入口和串行约束；或改为宿主侧比较 `resolve()` 的结果，再加一次浏览器抽查。
- **置信度**：中高。

### [P3] F-23 · 不让作者的 helper 编译 CLI 的整条内容管线

- **定位**：D2 第94行；§2.1 第194行；§2.3 第223行。
- **问题**：rustify-docs 的默认 `ssr` 含 syntect、two-face、lightningcss、html5ever、dom_query 和 rustify-style compiler。作者的 helper 只需要 `serve_site` 加上 Leptos、SDK、组件的 ssr，却要编译整套依赖，冷编译与链接都会变慢，直接影响 UAT-15 的首次体验。
- **修订**：拆成 `site-ssr`（helper 协议与 SSR scope）与 `pipeline`（CLI 使用）两个 feature，模板只启用前者；V-8 的依赖图检查覆盖这一点。
- **置信度**：中高。

### [P3] F-24 · 写明 Pages workflow 的触发、artifact 保留与回滚做法

- **定位**：§3 第241行；§8 第373、375–377行；M7 第453行。
- **问题**：
  - “再部署上个完整 artifact”没写机制。`actions/upload-pages-artifact` 的 `retention-days` 默认为 1 天（architecture 视角 2026-10-13 读取 action.yml），过期后只能在旧 SHA 上重跑整个 workflow，包括 macOS 构建。
  - 触发方式没定：如果 push 到 main 就自动部署，后续开发会把未验收的站点上线。
  - 首次发布失败后是撤下站点还是保留，没有写。
- **修订**：写明触发方式（`workflow_dispatch`，或带审批的 environment）、`retention-days` 或“在 last-good SHA 上重跑”的回滚定义、build_id 记录，以及首次失败的处置；M7 做一次回滚演练，或明确记为缺口。
- **置信度**：中。deploy-pages 能否部署其他 run 的 artifact 尚未核实。

### [P3] F-25 · 改正若干现状描述

- **定位与事实**：
  - 第149、305行的 `Compiler::compile_inner` 在代码中不存在，dev 的生成入口是 `Builder::build`（[dev.rs](../../crates/rustify-docs-cli/src/dev.rs) 第178、250行）。
  - C-2 第83行链接的 `#b-03--cli-原生工具链与支持平台d10-第二部分` 锚点失效；实际标题是“B-03 · P1 CLI 支持平台与原生依赖约束（D10 第二部分）”（[decisions](DOCS-SSG-P1.decisions.md) 第78行）。
  - 第316行称 `candidates.json` 由“既有 xtask css”生成，但 xtask css 只调用 Tailwind CLI，xtask 引入的 rustify-style 也没开 scanner；这是新增能力。
  - 第249行写“进程计数 ID”，实际是 `thread_local` 计数。
- **影响**：实施者按名称或锚点查找时会扑空，或低估 M3 的工作量。
- **置信度**：高。

## 2. 待核实与待决策

| # | 缺什么证据或决定 | 影响的决定/任务 | 谁可提供 | 最晚确认点 |
| --- | --- | --- | --- | --- |
| U-1 | P2 是否复核桌面 Firefox 稳定版（F-13） | V-11 的范围、M6 出口 | 用户 | M6/V-11 之前；计划先在 §11 登记 |
| U-2 | `vellum.md` 翻译、标 `lang: zh` 或另行处理（F-12） | §5.6、V-10/V-11、M5 内容出口 | 用户 | M5 之前 |
| U-3 | GitHub 侧事实：has_pages=false、Pages API 404、仓库是否公开（影响受试者与读者能否 git 安装） | M6 候选分发、M7 | 站点维护者；只读 `gh api repos/rockie/rustify-ui` 与 `/pages` | M6 之前 |
| U-4 | 组件 fixture 与官方站的冷构建（host SSR + wasm32 release）能否在 webServer 600 s 内完成；docs-dev 每个测例复制 fixture，默认超时 60 s | V-5/V-9 能否运行、F-05 的拓扑 | 实施者实测 | M2 收口 |
| U-5 | DOM-only release wasm 的体积，以及参考机上 FCP→全部可交互的时间（spike S2，见 F-19） | 一站一个 wasm、立即水合 | 实施者 | M2 收口 |
| U-6 | 未定义 `__RESOLVED_RESOURCES` 等全局量时，wasm-bindgen glue 是抛 ReferenceError 还是返回 undefined | F-02 的失败形态与诊断 | 实施者读生成的 glue 或做小实验 | M1 |
| U-7 | `actions/deploy-pages` 能否部署其他 run 的 artifact | F-24 的回滚方案 | 实施者查官方文档 | M7 前 |
| U-8 | SDK 的 theme、overlay、mount、i18n 改动对 GPU 示例的回归时点与选择规则（全量约 1 h 46 m） | V-14 | 实施者在计划中写明，建议 M1 收口与 M6 各跑一次受影响项目 | M1 |
| U-9 | 除 F-10 已知的链接外，12 份指南在根配置下 `check` 是否还有其他错误（原始 HTML、标题跳级等） | V-10、F16/AC-3 | 实施者用临时根配置跑 `check --format json` | M5 之前 |
| U-10 | host 与 wasm32 两次构建共用 target 目录时，Cargo 构建锁是否让它们串行 | dev 时序、N2 余量 | 实施者实测 | M4 |

**需补证的技术前提（spike，来自 architecture 视角）**：

- **S1**：最复杂几类在 host ssr → `hydrate_from` 后是否一致，包括 DataTable 虚拟窗口、闭合的 Select/Dialog、Tabs、NumberField 草稿、Tree，同页 3 个同类岛。M1 退出前完成。
- **S2**：同 U-5。M2 收口前完成。
- **S3**：选定 profile 后，“两目标编译完成→页面显示新代”的耗时，五次中位，并记录编译本身的耗时。M2 收口或 M4 开始前完成。
- **S4**：模板依赖固定到 R 并带锁文件，CLI 在 R+1 安装后执行 `build --locked`，并验证错配诊断（F-01）。M4 开始前完成。
- **S5**：字体来源、许可、体积与路径（F-09）。M3 开始前完成。
- **S6**：helper 拆分与不拆分 feature 时的冷编译耗时（F-23）。M2 完成。

## 3. 覆盖摘要

- **基线**：计划 SHA-256 `7c02d799…26b51fc`（486 行）；代码 `785f51a`，dirty 仅限台账与本计划；PRD SHA-256 `63fdcd7f…63e06d3`（schedule 视角记录）。评审开始与结束时计划 hash、HEAD 一致，没有并发修改。
- **分工**：四个独立 subagent 并行，均为只读、继承主模型。
  - **facts**：代码事实与内部矛盾。核实了 §1.1 各条主张、依赖版本、CLI/docs/SDK/style/xtask 行为、Playwright 与 harness、锁定依赖源码。
  - **architecture**：ADR-1/ADR-2、`site!`、构建协调、主题、失败语义与发布。读了 leptos 0.8.22、leptos_macro、tachys 0.2.19、hydration_context 0.3.2、leptos_server 0.8.8、wasm-bindgen-cli-support 0.2.128 的本地源码；2026-10-13 联网读取 `actions/upload-pages-artifact` 的 action.yml 与 Rust `proc_macro::Span` 文档。
  - **schedule**：P2 每条 AC 原文到账本、V-*、M* 的双向追踪，依赖边与波次，进度协议。
  - **delivery**：作者、读者、维护者三条主路径与 UI/UX。
- **主 agent 复核**：逐条读了支持候选问题的计划段落，并亲自复核了 `playwright.docs.config.ts`/`serve.sh`/`static-host.mjs`、`content.rs` 的链接解析与 snapshot 读取、`docs` 体积、`vellum.md`、nav 配置合同、`fonts.rs`/`codec.rs`、leptos `hydrate_from`、`shell.rs` 的 `<!>` 清理、tachys `style:` 水合、`dev.rs` 入口、P1 decisions 标题、PRD WORK-N3 与 G-05/D-01、初稿事件、`install_revision`、AGENTS.md 设计规则和阅读表面 brief 的范围。
  - 去重：四个视角都报告了模板依赖与版本问题，合并为 F-01；hydrate 运行前提合并自 architecture 与 facts；`rust-dev.spec.ts` 匹配与 meta MIME 合并进 F-05。
  - 降级与移出：目录名派生包名并入 F-01；Firefox 与 vellum 既作为 finding，也列入待决策。没有撤回的误报。
- **脚本**（schedule 视角在仓库根实际运行）：
  - `check_paths.sh`：45 个路径中 14 个缺失，其中 13 个是 ★ 新增，另 1 个是协议原文中的兼容路径 `docs/project-status/protocol.md`，无错误路径；
  - `check_progress.sh`：0/7，里程碑 7 行，ERROR 0，WARN 1（工作树中与本计划实施无关的台账改动）；
  - `check_plan.py`：ERROR 0，WARN 15，全部是自定义 ID（WORK-F*/N*），已逐条回 PRD 核对，均存在且期次正确。

  脚本通过不证明映射或验收成立。
- **UI 证据等级**：P2 的全部界面（演示块、岛状态、定制页、分类页、首页、dev 构建中）尚不存在，只有文档推演，待实现后浏览器验收。P1 阅读表面只做了源码与设计记录的静态核对。本次没有启动浏览器、服务器或任何 build/test。
- **未覆盖**：GitHub 仓库与 Pages 的实时状态（U-3），以及各 spike 需要的实测数据（U-4、U-5、U-10、S1–S6）。

## 4. 映射与实施编排

**映射核对**：

- §7.1 列出的 P2 AC 都已入账本，终验期与 PRD §7.2–7.3 一致：F1/F4/F5/F6/F8/F13/N2 在 P2，F15 在 P3，F16/N1 在 P4。
- V-3/V-15 对 WORK-F4/AC-4 的分工（本地对比 + 真实 GitHub 预览）与 PRD 原文一致。
- 进度协议与里程碑表一致：0/7、下一步 M1、提交点 M2/M6/M7、每个退出条件都以“回写「实施进度」”结尾，骨架协议原样在位。
- 断链如下：

| 链路 | 断点 | 问题 |
| --- | --- | --- |
| F1/AC-2 → 浏览器验收 | 没有检查负责 | F-15 |
| F4/AC-2 前半句 → 对照断言 | 缺 CSR 对照 | F-03 |
| N2/AC-2（P2 终验）→ 回归 | 未列入 | F-21 |
| UAT-3/4/9/10/11 的 P2 部分 → V-* | 没有映射；M6 缺连续主路径 | F-16 |
| V-2（M1）→ 组件 fixture（M2） | 依赖倒置 | F-04 |
| V-5/V-6/V-9/V-11 → 站点供给与 project | 未设计 | F-05 |
| V-13 → 候选冻结与分发 | 顺序倒置；M7 不复核 | F-14 |
| V-8/V-13/V-15 → 模板依赖来源 | 未定义 | F-01 |
| V-12 → dev 事件 epoch 字段、B 语料演示页 | 交付物未登记 | F-21 |
| verify.yml、themes.md、F16/AC-3 部署说明 | 没有负责的里程碑或断言 | F-18、F-21 |
| N3/AC-4 中的 Firefox | 依据缺失 | F-13 |

**依赖边调整**：保留原计划的主干 V-1 → M1 → M2 → {M3, M4} → M5 → M6 → M7，只改以下边：

- 删除 M2 前置中的“V-2 浏览器部分”（F-04）。
- M4 前置由“V-4/V-5”细化为“V-4 + 岛启动可用”。
- M5 前置去掉 M4 的 V-8/V-9，改为 M2 的 check/build 加 M3 的 V-6/V-7。
- 新增“M3 主题上下文 → V-5 复跑”（F-21）。
- 新增“作者工程依赖合同 → M2 协议定稿、M4、M6 候选冻结”（F-01、F-14）。
- 新增“M6 候选冻结 → V-11/V-12/V-13”（F-14）。
- M7 的 workflow 草案和授权申请可提前到 M5 期间（F-20）。

**建议波次**：

| 波次/任务 | 前置及解锁证据 | 可同时做的工作 | 写入边界/共享负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| W0 基线与合同定稿 | 工作树核对；本评审 P1/P2 修订落入计划 | V-1；S1/S2 的设计；F-01 依赖合同与 F-06 `site!` 形态定稿 | 计划与记录由主实施者写 | 分包 `mbx test` 串行 | V-1 |
| W1 = M1，外加 M2/M3 的宿主侧前期工作 | V-1；主实施者先定根、seed 合同与 `site!` 语法 | SDK hydrate API；各类组件宿主 SSR 与 `style:` 改造（按类文件分工）；M1 水合 harness（F-04）；Markdown 围栏、YAML、Embed span 与静态清单；`theme.document` 导入与 CSS 投影（只改 `style.rs`） | `crates/rustify-ui/{Cargo.toml,mount.rs,lib.rs,diagnostics.rs}`、`id.rs`、根 Cargo.lock、rustify-docs 的 feature 拆分由主实施者负责；`mbx xtask css` 重新生成由单人负责 | 共用 mbx target；浏览器单 project 串行；GPU 示例回归只在 M1 收口跑一次 | V-2（按 F-03/F-04 修订）、V-14 中受 SDK 影响的部分 |
| W2 = M2 | M1 的宿主与编译检查 | `site!` 与 `rust_site`、helper 协议（主实施者）；协议 metadata 固定后，`islands.js`/`mode.js`、源码切片、组件 fixture 可分工；`serve.sh`/config/`static-host.mjs` 由单人负责；岛状态 UI 的设计步骤（F-17） | `site.rs`、`rust_site.rs`、CLI 的 `main.rs`/`report.rs`、Cargo.lock 由主实施者负责 | 浏览器串行；做必需的 S2/S3 探测 | V-3/V-4/V-5/V-14；提交点 |
| W3 = M3 ∥ M4 ∥ M5 演示函数 | M2 退出（M4 只需 V-4 + 岛启动） | 定制页与插槽（generation/shell/content，主实施者）；主题与候选（style、css、`xtask/src/css.rs`，不碰前者的文件）；模板与 doctor（`templates/components`、`new.rs`，需 F-01 先定）；dev Rust（`dev.rs`）；24 个演示函数（`site/src/demos/*`，按类分工） | `site.rs`/`rust_site.rs` 由主实施者单写；`site/Cargo.lock` 由 M5 负责人；根 Cargo.toml 的 exclude 由主实施者 | V-6、V-9、V-5 复跑共用一个浏览器队列；site 用独立 target 可减少锁竞争，但要注意内存 | M3：V-6/V-7 + V-5 复跑；M4：V-8/V-9（含 F-15、F-21） |
| W4 = M5 集成 | V-6/V-7、各类演示函数 | 官方站集成：根配置白名单、README 首页、分类页与导航（F-10/F-11，主实施者）。同期并行：起草 `docs-pages.yml`（不启用）、申请推送与 Pages 授权、招募受试者、确认设备、处理 U-1/U-2 | 配置、导航、页面注册由主实施者统一写 | 串行 | V-10 + 本地局部 V-11 |
| W5 = M6 | M1–M5；第一步冻结候选（F-14） | 不建议并行：V-11（root/subpath/meta 依次）、V-12、V-13、V-14、UAT-3（F-16） | 主实施者收口 | Playwright 单 project、单进程；性能测量独占参考机 | M6 出口；提交点 |
| W6 = M7 | M6；推送、Pages 与发布授权 | 推送 → V-15 安装 → macOS 构建 → 部署 → 线上全页验收 → 回滚演练（F-24） | 文档与台账由主实施者单写 | 部署不并发 | V-15 + 复核（含 V-9；若模板变化，含 V-13）；最终提交 |

除 F-04 外没有发现依赖环。结构上的瓶颈链是 M1 根契约 → M2 `site!`/协议 → M4 模板 → M6 候选冻结 → M7；F-01 与 F-06 的定稿决定这条链能否按序推进。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked（针对当前文本）**。计划自报 Ready，两者不一致。

- **阻塞理由**：作者工程依赖与 CLI/库版本配对（F-01）是对外契约，目前未决；hydrate 运行前提（F-02）、`site!` 实现形态（F-06）、构建档位与快照策略（F-07/F-08）都是 M1/M2 开工前就要定的技术决策，按现文实施者必须临场重做。
- **阻塞范围**：M1 的根契约与退出判定、M2 的协议定稿、M3 的字体、M4 的模板、M5 的白名单与分类页。V-1 基线核对与不涉及上述合同的 SDK 内部准备可以先做。
- **两项用户决定**：Firefox（F-13/U-1）与 vellum（F-12/U-2）只影响 M5/M6 出口，登记到 §11 并写明最晚确认点后，不阻塞开工。
- **解除条件**：P1/P2 修订写入计划，F-01、F-02、F-06、F-07、F-08 在正文中各有唯一答案；两项用户决定已登记。满足后可复审恢复 Ready。

**下一步**：`/apply-doc-review docs/plan/DOCS-SSG-P2.review.md`，核实并修订计划，处置后再判定 Blocked 或 Ready。

**修订顺序**：

1. **纠正前提与契约**：
   - F-01 作者工程依赖、版本握手与资源来源；
   - F-02 hydrate 运行前提与“任意组件”的范围；
   - F-10 `content.include` 契约与链接映射；
   - F-25 现状描述；
   - 在 §11 登记 F-12、F-13 两项用户决定，并改正 C-6 的来源。
2. **补设计决策**：
   - F-06 `site!` 形态；
   - F-07 编译档位与冷启动行为；
   - F-08 快照还是原地构建；
   - F-09 字体；
   - F-11 分类页、导航与首页；
   - F-17 设计阶段；
   - F-23 helper 的 feature 拆分。
3. **修映射与验收**：
   - F-03 一致性覆盖集与对照判据；
   - F-15 模板浏览器验收；
   - F-16 UAT 映射与 M6 主路径；
   - F-18 作者文档；
   - F-21 收口条目；
   - F-22 Theme Studio 对照入口。
4. **调编排**：
   - F-04 M1 harness；
   - F-05 浏览器拓扑；
   - F-14 候选冻结与分发；
   - F-19 M2 性能风险门；
   - F-20 依赖细化；
   - F-24 Pages 触发与回滚。

本评审只判断计划能否照做；计划通过评审不表示功能已实现或站点已上线。

## 修订处置与复核

- 来源：本报告原始 SHA-256 `be262a51416ed71fda36ff2d059360e077d408bd108714528fe90d980e82b557`；上方评审对象、F-01–F-25、原结论与所审基线原样保留。本节为 apply 文档处置，不是新一轮独立评审。
- 目标：[DOCS-SSG-P2](DOCS-SSG-P2.md)，修前 v1 / `7c02d799bca5910ee37e52c292ad7f4e9f584e8adc77225f6be323b4d26b51fc`；修后 v2 / `86a348769d4a1044beab144b8d193be5e2e53b6304185f5c38913ffb9e7434c5`。
- 复核：2026-10-13 +08:00；main / dirty@`785f51ad9fdcb69e624a0c8f85d91e3541d58e90`。开始时既有 dirty 为 SDLC 台账和未跟踪的计划/报告；产品源码未修改。修订开始时原文与所审 hash 相同，未发现需保留的另版原文或既有处置。
- 范围：全部 25 项、U-1–U-10、S1–S6及§4映射/依赖意见。亲读计划、PRD相关AC/UAT/分期、项目规则/配置、对应代码及锁定依赖源码；只修改目标计划、追加本节与登记台账，不执行计划。
- 用户确认交付为 **SSG**：CLI构建期生成HTML，线上只托管静态文件；ssr仅是Leptos/SDK构建期feature。原报告中的“SSR”机制意见按这一语义落实，未引入读者请求渲染服务。

下表的「已解决」仅指文档缺陷；实施后的功能、PoC、性能、浏览器、真人与上线验收仍按V-*、M*执行。

| 问题 | 本次核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立：main.rs的来源缺失回落、new.rs仅content、runtime.rs同版本资源、固定bindgen要求与模板发布缺口均核对 | 已修订并复核 | §2.1 R/C配对、模板预生成lock、来源/版本预检、helper资源导出、file://测试覆盖；§5.4/§8/V-3/8/13/15/M6/7 | 已解决：模板写什么、锁从哪来、候选与错配均有唯一合同。依用户选择，CLI SHA=C、库rev=R，**不采纳原复核末句要求两SHA相等**；实施核验配对而非字面同SHA |
| F-02 | 成立：hydrate_from新根不继承外层context，HydrateSharedContext使用全局数据；shell清理非正文部分<!>，新增slot岛须事后拼接保留；tachys style:水合空操作 | 已修订并复核 | ADR-1、§1.1/§2.3/§4/§5.2/V-2/5；确定初态、浏览器异步、空全局量、闭包内上下文、岛标记与几何类/CSSOM | 已解决：SSG边界依用户重新确认，初始跨端异步需求给诊断/client-only建议；反例/slot/几何测试登记，实际glue与水合待M1/M2 |
| F-03 | 成立：CATALOG 24类、九个wasm-only导出与tachys元素类型检查已读，运行期不全面验证结构 | 已修订并复核 | V-2全24类host/hydrate/独立CSR结构/公开状态/节点/ID/ARIA对照；§8与M1/§11 | 已解决：覆盖集、允许规范化内容和错标签反例明确；实现后逐类保存证据，不以“可响应”替代CSR一致 |
| F-04 | 成立：原V-2依赖M2新增fixture，而M2以前者为前置 | 已修订并复核 | §1.2新增M1独立harness、§9.1 dom-hydrate/4194/V-2；M1/M2前置 | 已解决：采用(a)，M1直接用SDK、不依赖site!/CLI。另把V-5的slot/theme部分分到M3，避免新循环 |
| F-05 | 成立：config/serve固定content；dev匹配dev*；meta MIME缺wasm/font | 已修订并复核 | §1.2/§9.1按来源分project/端口/serve/spec，dev-rust命名、static-host MIME；M2配置/M5官方spec | 已解决：旧content、components、official与dev互不误收；未来--list/MIME与冷构建检查有归属。本次未创建project或启动服务 |
| F-06 | 成立：需生成typed props及编译前解析，一份静态声明仍缺实现形态 | 已修订并复核 | D3/ADR-2/§1.2/§2.2/§2.3/V-3/M2：独立过程宏+共用语法crate、describe一致闸 | 已解决：两个真实调用方共用parser，运行期DTO留docs；未来故意不一致反例须拒绝整代 |
| F-07 | 成立：根release为fat LTO/abort；组件workspace无档位合同；dev首轮build先于地址输出 | 已修订并复核 | §2.1固定host/dev-wasm/release档、CLI flags/bindgen；§5.4先地址/building；§9.2/9.3/V-9/12/13/M2/4 | 已解决：≤2s用dev档、≤1s用release；首次冷编译剔除、增量等待计入15分钟。耗时仍待实测，不新增编译上限 |
| F-08 | 成立：快照与提交前指纹两路线并存，诊断/切片身份不清 | 已修订并复核 | §2模块表、§5.4选原地构建与前后摘要，临时目录只存输出；V-4/9/M4 | 已解决：原路径编译、本轮已读字节切片、摘要变化丢弃重建；future fresh计数/作者行列/连续保存可验证 |
| F-09 | 成立：FONT_FACES/codec引用Makepad目录，回退字体大且CLI无字节；读取现有七文件大小/许可 | 已修订并复核 | §1.2/§2.1/§5.5/§11、V-7/10/12、M3：CLI同版七字面/许可/来源摘要，中性输出路径 | 已解决：用户选内置，维持glyph回退、不要求作者额外资产；实际CLI/请求体积、禁网/字体对照及预算待M3/M6 |
| F-10 | 成立：content内未入页图的md报broken-link、snapshot逐文件read、dev递归目录；utility的protocol.rs:96失效 | 已修订并复核 | §3/§5.6/§12、V-10/M5：include以content.dir为基准，先过滤读取/监视，已存在排除文件新增repo映射；README源基准 | 已解决：没有把新增能力写成现有；12指南零错误与validation写入不重建均有门，原指南链接修正留M5 |
| F-11 | 成立：配置dir/items仅内容路径，pages声明没有公共demo块接口；README指代须处理 | 已修订并复核 | §2.3 RenderContext::demo、§3 nav.routes、§5.6首页/分类/nav36项、V-6/10/11/M3/5 | 已解决：24定制分类、12指南均可导航，首页两主操作与同源README清晰；UI设计/首屏验证留实施 |
| F-12 | 成立：亲读vellum中文正文，与英文默认/正确lang要求有冲突 | 已修订并复核 | §5.6/§11/V-10/11/M5：唯一源翻译为英文，保留事实/链接/12页集合 | 已解决（决定/计划）：用户明确选择译英；当前源仍中文，翻译是M5交付，未冒称已译 |
| F-13 | 成立：P1 D-01/G-05只批准P1后移；初稿事件仅记录三数值，没有P2豁免原话 | 已修订并复核 | C-6/§0.5/§9.1/§9.3/§11/§12、V-11/M6：用户新决定延期到P4 | 已解决（出处/口径）：不采纳恢复P2的建议，依用户2026-10-13明确延期；P2 Firefox稳定版/引擎未验证，P4仍须验。PRD AC-4/Q3同步需求单列 |
| F-14 | 成立：原冻结/文案在V-13之后，M7会改变受试候选 | 已修订并复核 | §3/§8/§9.3/V-13/15/M6/7/提交点/§11：先冻R/C/lock/模板/引导并确认分发 | 已解决：M6用户验收与M7安装同C/R；若产品/引导改动回M6重验，未把本次文档授权当推送授权 |
| F-15 | 成立：原V-8不开browser、V-13会修改工程，不能覆盖未改模板 | 已修订并复核 | WORK-F1账本、V-8/9/15、M4/M7：new后原样dev/ready/操作/源码，安装CLI再跑 | 已解决：开发/安装二进制都有未改模板浏览器门；实际浏览器留实施 |
| F-16 | 成立：PRD UAT-3/4/9/10/11有连续动作，原计划只有UAT-15明映射 | 已修订并复核 | §9.1 UAT映射，V-9与M6同工程“演示→定制页→footer→坏代码→恢复” | 已解决：作者/读者主路径可逐项追溯，搜索/语言/AI仍各按P3/P4，不扩期 |
| F-17 | 成立于设计任务缺失；局部校正：**dev brief已经列building**，尚需实现/多行Rust细节；阅读brief明确P1范围 | 已修订并复核 | §1.2/§6/M2岛块/M4dev/M5首页分类：扩展两份既有brief、finish verdict/截图、DESIGN.docs-ssg同步 | 已解决：各新状态有设计落点，未另建重复规范、未声明已使用impeccable完成新UI设计或浏览器验收 |
| F-18 | 成立：模板README未定内容/首命令，作者与部署文档无退出归属 | 已修订并复核 | §5.4必含章节、M3 themes/M4作者合同与README试走/M6引导冻结/M7部署版本核对，V-13/15 | 已解决：help提供new首命令，其后生成README唯一引导；部署meta范围与模板示例/链接检查明确 |
| F-19 | 成立于检测太晚；实际体积/余量未知，不沿用旧GPU体积猜DOM | 已修订并复核 | V-12/§9.2/§11/M2必做全24类release代理门及分项，超预算先解决再进M3/4 | 已解决：早期风险门强制，M6正式最大页不可由代理替代 |
| F-20 | 成立：M4/M5与发布准备的整里程碑依赖过粗 | 已修订并复核 | §10：M4=V-4+启动，M5=M2 check/build+M3页图/主题；函数按类V-2/site!；workflow/申请提前 | 已解决：保留用户/发布门，文件不冲突仍须契约/资源稳定；未启动并行实施或子agent |
| F-21 | 成立：复跑与交付物/N2 AC-2未逐项对上 | 已修订并复核 | §1.2/§5.4/§9.2/9.4/M2–M7：epoch、docs_corpus、verify.yml/themes归属；M3 V-5，M6 V-5/9/10，M7 V-9，content≤60s | 已解决：检查时点与出口对齐；0/7/7里程碑/15检查/空完成记录与原实施协议保持 |
| F-22 | 成立：Theme Studio为GPU示例，需release产物/根project与串行约束 | 已修订并复核 | V-7/§9.1/9.4：build-web release→theme-studio theme-dom.spec.ts（4181）→docs，未来补同文档对照断言 | 已解决：现有入口/资源已核对，未运行GPU build或browser |
| F-23 | 成立：当前docs宿主依赖含高亮/内容/CSS管线；helper原分组会带入 | 已修订并复核 | D2/ADR-2/§2.1/V-8/14/M2：默认pipeline、helper site-ssr、wasm hydrate，各图独验 | 已解决：分组增量与冷编译观测明确，实际图/耗时待实现；现库尚无这些新feature |
| F-24 | 成立：触发/保留/回滚机制缺失；本次亲读官方action与部署/取消发布文档 | 已修订并复核 | §8/M7/V-15：手动dispatch、同run artifact、retention-days=1、last-good SHA重建/摘要、首发坏站取消发布、演练 | 已解决：不依赖跨run artifact能力，不只重跑deploy；实际发布与演练须有后续授权/证据 |
| F-25 | 成立：Builder::build、thread_local、xtask仅Tailwind调用与B-03标题均核对 | 已修订并复核 | C-2/§1.1/§4/§5.4/5.5/§12纠正入口、锚点、计数与新增候选能力 | 已解决：授权目标重复错误均已搜改；原报告保留旧定位主张，本节说明当前正确事实 |

### 决策与未完项

用户本次异步答复（2026-10-13 +08:00），均已落到计划设计/验收/风险：

| 关联 | 用户明确选择 | 影响/未选方案 |
| --- | --- | --- |
| F-01 | “库 R → CLI C，两次提交固定配对（推荐）” | 预生成lock，CLI/库分别标C/R；未选new按自身SHA动态解依赖 |
| F-02 | 先澄清“别搞SSR啊，我们需求是SSG”；随后选择“构建确定初态；异步在浏览器运行（推荐）” | 线上静态文件，构建期只生成初态；未选构建等待异步数据/每岛序列化协议 |
| F-09 | “CLI 内置同版字体及许可，产物只打包引用字体（推荐）” | CLI体积代价显式记录；未选作者自备字体，维持导入即用 |
| F-12 | “M5 将唯一源文件译成英文（推荐）” | 指南保持单源；未选保留中文界面/lang:zh或删白名单 |
| F-13 | “明确延期到 P4；P2 保留未验证记录” | 新P2例外有出处，P4完整矩阵保留；未选当期恢复Firefox |

待核实与spike去向（不是本次实测）：

| 原项 | 本次状态与交接点 |
| --- | --- |
| U-1/U-2 | 决定已确认，分别落实Firefox延期/唯一源译英；实际译文/浏览器未执行 |
| U-3 | 本次只读 `gh api repos/rockie/rustify-ui` 得private=false、visibility=public、has_pages=false；`/pages`为HTTP404。证明当前设置，不证明未来R/C已推送或Pages已发布 |
| U-4 | 冷构建与webServer启动分离、spec按来源匹配、稳定target与冷用例timeout按实测配；M2必须记录耗时，官方站M5形成后再核其冷构建 |
| U-5/S2 | M2 V-12强制全24类代理；M6正式A/B最大页、gzip与FCP分项，未实测 |
| U-6 | 当前不预言缺全局量时的具体JS异常；按锁定源码预置/校验空数组并在V-2/V-5核实真实glue/反例，不把原报告推断冒作实测 |
| U-7 | 跨run artifact不再是设计前提；[deploy-pages](https://github.com/actions/deploy-pages/blob/main/README.md)的同run build→upload→deploy方案与[官方workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)已读；回滚重建旧SHA。跨run能力本次不作结论 |
| U-8 | V-14按mount/overlay/theme/i18n/table改动列旧GPU示例选择规则，M1与M6收口各跑一次对应项目，browser全局串行 |
| U-9 | M5/V-10负责12指南全量check零错误；只读原文件发现已知失效链接，本次未生成临时站或运行内容check |
| U-10 | host/wasm固定串行并复用target，取消Cargo锁并行的假设；M4记录真实串行编译与编译后可见时延，不声称已测锁竞争 |
| S1 | M1独立harness全24类对照，先复杂类/三岛；M2管线接入后V-5，slot/theme待M3 |
| S3 | M2只做档位/编译后渲染加载代理，实际dev完整链路的epoch→页面时延在M4首次验证，M6关闭；本次无耗时数字 |
| S4 | M2 Rtest/Ctest固定依赖/lock/升级错配探测；M4生成模板、M7GitHub安装与禁网均需复验 |
| S5 | 用户已选字体供给，七文件/中性路径/许可合同明确；M3复制/来源摘要/大小与禁网测试仍待实施 |
| S6 | M2分feature图、拆分前后冷编译耗时观测；不得仅因文档写了拆分就报告性能改善 |

尚未授权同步的关联原文：[PRD-DOCS-SSG](../PRD-DOCS-SSG.md) WORK-N3/AC-4、§5.2 Q3需记录本次Firefox延期到P4；WORK-F4可补SSG确定初态/浏览器异步边界说明。上游正文保持原样，P2依据本次用户决定与计划C-6/§2.3执行。M5的utility链接/README措辞/vellum译英、M3/M4/M6/M7的作者/部署文档是计划任务，未在本次提前修改。

### 修订后结论与验证

**本次判定：Ready。** F-01–F-25均已修订并完成文档层复核；已解决无需修改0、不采纳整项0、待决0。F-01的SHA相等断言、F-13的当期恢复建议及F-17对dev brief的局部描述按明确合同/证据调整，理由见表。Firefox延期不等于该浏览器验收通过。

实际检查：

- `check_paths.sh docs/plan/DOCS-SSG-P2.md <仓根>`：检查59路径、缺失20；人工逐项核对为19个★新增及1个协议兼容条件路径，无虚构复用路径。
- `check_progress.sh`：0/7、7里程碑、0完成记录，ERROR 0、WARN 1。告警因计划未跟踪，脚本无法识别其已修改；工作树只有文档/台账，无P2实施。
- `check_plan.py`：ERROR 0、WARN 15，WORK-F*/N*均已逐条查到PRD，期次/出口保持；不为告警改稳定ID。
- 手工及Python核对：原实施协议/空完成记录逐字保留、15检查/7里程碑/0/7、本地链接可达、旧错误口径已清除；逐项回读修改与依赖，M1/M2与M2/M3无验收循环。报告原472行作为完整前缀保留。
- 本次未运行产品test/build、任何站点check、GPU构建、浏览器、性能、真人或部署。图/入口/机制为静态文档与代码核实，界面仍待实现；台账记录本次文档就绪/处置，不增加实施完成数，不伪造独立复审。

下一步由用户发起：`/goal 按 docs/plan/DOCS-SSG-P2.md 开发`。Ready仅表示按已定合同开工；M1–M7的风险门、设备、五位首次用户、可获取候选与授权发布仍须真实完成。
