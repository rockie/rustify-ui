# DOCS-SSG-P1 开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P1 · 纯 DOM SDK 与静态内容文档站](DOCS-SSG-P1.md) | sha256 `dded28b26475af84007a60478d6f4032a3838dd20f9896d1a1b7d63f1730b37a`（439 行，未提交） | 全文 |

- 需求来源（背景资料，不是修订目标）：[DOCS-SSG PRD](../PRD-DOCS-SSG.md)，sha256 `c588c505348107acf7e90f1b9bf0a941d28e158abd85a3aeec62a57a130bfbd5`，P1 范围以 §7.1–7.3 为准。
- 调查基线：2026-10-10 +08:00 · `main` / `b0a6bc5b9010e57d1f0d758a37af573ea0712faa` · dirty。未提交的只有 `.xgent-ai/sdlc` 台账、PRD 及其评审和本计划，代码零改动。事实以当前工作树为准。评审开始与结束时计划 hash 一致。

## 1. 确认问题

### [P1] F-01 · 纯 DOM 消费者的启动契约没有定义：启动 JS 怎么分发、按什么配方构建、仓外怎么验收

- **定位**：
  - L12：本期首次让独立消费者不经过 Makepad。
  - L171：`web/dom-loader.js`、`web/dom-runtime.js` 放在仓根 `web/`。
  - L220、L223：「文档给消费者等价普通 Cargo/bindgen 启动方式」。
  - V-1 L352、V-2 L353、M1 L389、T-2 L406。
  - 关联 WORK-F17/AC-3（PRD L594）、UAT-14（PRD L857，新建只依赖默认配置的应用）。
- **问题**：
  - 仓外应用用 git 依赖引用 rustify-ui 后，拿不到页面启动所需的东西。仓根 `web/` 下的 JS 不随 crate 分发，目前只有 `xtask build-web` 会把 `web/loader.js`、`runtime.css` 拷进产物。现有 loader 写死了 `import "./bindgen.js"`。
  - 计划没写 wasm-bindgen CLI 的版本和 `--out-name`，也没写 `rustify.css` 从哪里来。
  - 计划没写剪贴板需要 `--cfg=web_sys_unstable_apis`。不加这个标志，`clipboard::available()` 会静默变成 false，和计划声称的「行为完整」不一致。
  - DOM loader 按计划不校验 schema，JS 与 Rust 中性 binding 一旦版本漂移，abort 保护可能静默失效。
  - V-1 和 V-2 只验仓内路径依赖和 `mbx xtask build-dom`，给消费者的配方从头到尾没有被执行过。PRD P1 承诺「应用开发者也能构建不带 Makepad 的纯 DOM 应用」，这一点无法由验收证明。
  - T-2 把 JS 放在哪里，会直接决定对外的启动契约，必须在 M1 开工前定。
- **证据**：
  - [web/loader.js:1-4](../../web/loader.js)
  - 仓内先例 [crates/rustify-ui/src/utility/browser.rs:15](../../crates/rustify-ui/src/utility/browser.rs)：`#[wasm_bindgen(module = "/web/utility-host.js")]` 让 JS 以 snippet 形式随 bindgen 产物分发。
  - [docs/compatibility.md:193-206](../compatibility.md)：「A build made another way is not」。
  - [crates/rustify-ui/src/clipboard.rs:39](../../crates/rustify-ui/src/clipboard.rs)
  - 根 [Cargo.toml:28](../../Cargo.toml) `wasm-bindgen = "=0.2.128"`。
- **修订**：
  - 在 ADR-1 / §4.1 定分发方式。建议 DOM runtime 放进 `crates/rustify-ui/web/`，按 module snippet 导入，每个实例用工厂对象，避免模块级状态在实例间共享。loader 也嵌入，或写明拷贝步骤，并加 hooks 协议版本握手。
  - 写明消费者配方：`wasm-bindgen-cli =0.2.128 --target web --out-name …`、rustflags、样式表来源、页面 CSP。
  - V-1 或 V-2 增加仓外 fixture：复制到仓外临时目录，以本地 `file://` git 候选作依赖，只用 cargo 和 wasm-bindgen CLI 照文档构建，然后跑 V-2 子集。M9 用真实 SHA 再验一次。
- **复核**：
  - 改稿后：§4.1 / §9.1 有分发方式、配方和仓外检查 ID。
  - 实现后：仓外产物的 `snippets/` 含 runtime；toast、resize、abort、trap 隔离通过；`clipboard::available()` 为真。
- **置信度**：高（全文只有 L223 一句涉及消费者启动；代码证据直接）。**阻塞**：M1（T-1/T-2 固定中性 host 契约前）、M9 文档。

### [P2] F-02 · 漏掉了 SDK 的仓内消费者 `tests/theme-export`，M1 会让它的构建、CI 和两个浏览器项目失败

- **定位**：§1.1 L159 只把它当隔离范式引用；§1.2 L169-178、§3.1 L209、T-3 L407、V-3 L354 都没有它。
- **问题**：
  - runtime 和 generator 以默认配置路径依赖 rustify-ui，用 cargo-makepad 构建，用仓库的 `loader.js` 启动，`build.sh` 和 CI 都用 `--locked`。
  - M1 把 Makepad 改成默认关闭后，会接连出三个问题：
    - wasm 不再导出 Makepad bridge，loader 的 hash 校验和打包失败；
    - SDK 依赖图改变，这个 workspace 自己的 `Cargo.lock` 与 `--locked` 冲突；
    - verify.yml 的宿主测试、`build.sh` 两次构建、`theme-export` / `theme-export-deep` 两个 project 全红。
  - V-1 到 V-3 都不覆盖它，实施者会在 M1 提交点被迫临时决定它开 `gpu` 还是改走 DOM boot。
- **证据**：
  - [tests/theme-export/runtime/Cargo.toml:8](../../tests/theme-export/runtime/Cargo.toml)、[generator/Cargo.toml:8](../../tests/theme-export/generator/Cargo.toml)
  - [build.sh:24-34](../../tests/theme-export/build.sh)（`--locked --offline` 加 cargo_makepad）
  - `generator/src/runtime.js:1`、`generator/src/main.rs:420`
  - [.github/workflows/verify.yml:36,109-111,240-243](../../.github/workflows/verify.yml)
  - [playwright.config.ts:24-25,48-49,171](../../playwright.config.ts)
  - 已排除的同类：`tests/theme-text` 只依赖 makepad libs，`utility-baseline` 构建的是 rustify-style-wasm，都不受影响。
- **修订**：
  - §1.2、§3.1、T-3 纳入 `tests/theme-export/{runtime,generator}/Cargo.toml` 与其 `Cargo.lock`，并写明选择。建议 runtime 开 `gpu`，保持现有契约。
  - V-3 的基线和退出范围加上 `build.sh`（根路径和 `/tools/theme-export/` 子路径）、`mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins`，以及这两个 project。
  - 写明 generator 读取的 `THEME_FONTS` 等字体目录在宿主侧保持可用。
- **复核**：改稿后 M1 的交付物和退出条件里有该路径。实现后 verify.yml 对应三步通过。
- **置信度**：高。**阻塞**：M1 提交点。

### [P2] F-03 · 性能风险门无法判定，早期测量和测量工具都没有归属；配置保存触发全站失效时，没有满足 ≤1 s 的机制

- **定位**：V-14 L365（只在 M2、M8 执行）、M2 L390「V-14 最小 PoC」、§5.5 L284-285、§7 N2 L316、§9.3 L377-378、§11 L420；`tests/docs-ssg/` 在 L176 没有归属里程碑。
- **问题**：
  - M2 只有 SSR 最小页，没有内容图、build、preview 和 dev。「最小 PoC」没有定义测什么、什么结果算停止，退出条件无法判定。
  - 下列测量都要到 M8 才第一次进行：
    - N2/AC-3：500 页构建 ≤20 s；
    - N1/AC-2–3：LCP、CLS、≤100 KB；
    - N2/AC-1：保存到可见 ≤1 s；
    - N6/AC-4：千次保存。
  - N2/AC-1（PRD L646）把配置保存也纳入 ≤1 s。按 §5.5「配置变化按影响失效全站」加整代提交，500 页要在约 0.7 s 内全部重新渲染。计划没有内容 hash 片段缓存、并行渲染或当前页优先等机制。到 M8 才暴露，会返工 M6 的 dev 架构。
  - 参考站点 B 的生成器（按 12 份指南的分布）和页内测量工具没有负责的里程碑。
  - 「M2 冻结 Chrome 版本」需要一个固定版本的浏览器。系统里的稳定版会自动更新，另装浏览器按仓库规则要先征得用户同意，计划没有登记这一项。
- **证据**：计划行号如上；PRD L611（Chrome 在 PoC 时冻结）、L646–648。
- **修订**：
  - V-14 的 M2 切片写明测量项和 go/no-go 门槛，例如：
    - 最小页 HTML+CSS+JS 经 gzip -9 的字节数；
    - 11 种语言的高亮吞吐；
    - 500 页冷、热全量渲染外推；
    - 「配置变更 → 500 页重新渲染并提交」的热耗时。
  - §5.5 补片段缓存（键为内容 hash + 高亮器版本 + 选项）与并行渲染。
  - 增加非阻塞的早期测量：M4 测 500 页构建和产物字节，M5 用 preview 测 LCP/CLS 和 ≤100 KB，M6 测保存到可见和一轮短的连续保存。超限即开优化任务。
  - B 生成器和测量工具归到 M2 或 M4。
  - Chrome 固定版本的来源列入 §11，由用户确认。
- **复核**：改稿后 M2 退出条件可判定，M4–M6 有早期数据记录项。实现后各里程碑记录里有数据。
- **置信度**：中高。**阻塞**：M2 出口、M6 开工前。

### [P2] F-04 · CLI 继承根 release profile 的 `panic="abort"`，dev 的「保留 last-good」「千次保存不重启」兜不住 panic

- **定位**：§5.5 L284-287、§7 N6 L320、§8 L340；全文没有出现 panic 或 profile。
- **问题**：
  - 根 `[profile.release]` 设了 `panic="abort"`、`lto="fat"`、`codegen-units=1`。`cargo install --git` 在源 workspace 内构建，所以 CLI 会带着 abort 编译。
  - profile 的按包覆盖不能设置 `panic`，测试又忽略 panic 设置。结果是单测里用 `catch_unwind` 做的「本代失败、保留上一代」能通过，安装后的二进制却会整进程中止。
  - syntect 第一次命中某个 grammar context 时才编译正则，失败就 `expect` 直接 panic。补 TypeScript grammar 或换 fancy-regex 时尤其容易遇到。作者保存某段代码，dev 服务就会退出。
  - fat LTO 还会拉长安装编译时间。
- **证据**：
  - [Cargo.toml:37-40](../../Cargo.toml)
  - `syntect-5.3.0/src/parsing/regex.rs:70`：`expect("regex string should be pre-tested")`
  - Cargo profiles 文档（2026-10-10 查询）：「Overrides cannot specify the `panic`, `lto`, or `rpath` settings」；「Tests … ignore the `panic` setting」。
  - architecture reviewer 读到的 cargo `cargo_install.rs` 对 git 源使用 `Workspace::new`。这条来自 master 源码，本机 1.98.1 未实测，见待核实 V-2。
- **修订**：在 ADR-2 / §5.5 定 CLI 的 panic 策略，二选一：
  - (a) 新增 `[profile.<cli>] inherits="release", panic="unwind"`（可同时改 thin LTO），安装命令加 `--profile`，每一代构建用 `catch_unwind` 隔离；
  - (b) 保持 abort，把「不得 panic」写成合同：V-4 加 11 种语言全部 context 的正则预编译测试，对畸形 Markdown/HTML/CSS 做样本测试，并写明 dev 遇到 panic 时的预期行为。
- **复核**：改稿后 §5.5 和 §8 一致。实现后用安装出来的 release 二进制注入一次 panic，验证行为符合合同。
- **置信度**：中高。**阻塞**：M2 选定高亮后端前、M6 前。

### [P2] F-05 · syntect 生成的 class 会进入 Tailwind 候选，`block`、`table` 等名字撞上工具类，代码块会错乱

- **定位**：§5.4 L276（「渲染全页后提取 class 属性」，只排除代码文本）、L277；ADR-3 L129-131。
- **问题**：
  - syntect 默认的 `ClassStyle::Spaced` 把 scope 原子直接当 class，例如 `meta.block.rust` 会变成 `class="meta block rust"`。
  - 按计划，高亮 span 上的 class 会被当成候选，rustify-style 会为 `block`、`table`、`italic` 等生成工具类规则，导致代码 span 变成 `display:block`，缩进和换行错乱，候选规模和 CSS 体积也一起膨胀。
  - CSP 和内联样式检查都拦不住这个问题。
- **证据**：
  - `syntect-5.3.0/src/html.rs:224-241`，`SpacedPrefixed` 的文档原文：「useful to prevent class name collisions」。
  - [计划 L276](DOCS-SSG-P1.md)。
- **修订**：
  - ADR-3 / §5.4 定为 `ClassStyle::SpacedPrefixed { prefix: "hl-" }`（前缀可自定）。
  - 类提取只覆盖框架模板和作者原始 HTML，高亮 span 不进候选。配色 CSS 由高亮主题单独生成。
- **复核**：V-4 断言候选集中没有高亮原子；含 `meta.block` 的 Rust 片段，span 的 computed display 为 inline。
- **置信度**：高（机制来自源码；Rust grammar 会产出 `block` 原子是强推断，M2 渲染一次即可坐实）。**阻塞**：M2 样式与高亮切片。

### [P2] F-06 · 「CLI 模式污染视为失败」与新包进入根 workspace 互相冲突，全工作区命令的 feature 合并策略没有定

- **定位**：D1 L97「不依赖 feature-unified 工作区结果」、§4.2 L229「CLI 模式污染视为构建配置失败」；另一处是 §1.2 L169「登记新包」、V-15 L366、§9.2 L372、§12 L433。
- **问题**：
  - 文档包进入根 workspace 后，CI 和 CLAUDE.md 规定的 `mbx test --workspace --lib --bins`、`mbx clippy --workspace --all-targets -- -D warnings` 会在一次编译里合并示例的 `gpu`/`csr` 和文档包的 `ssr`。
  - Leptos 0.8.22 允许 csr 与 ssr 同时开启，没有 `compile_error`，但行为会变：
    - csr 打开 `reactive_graph/effects`，`Effect::new` 会去 spawn，没有执行器时 panic；
    - tachys 在 ssr 下丢掉 `on:` 和 `prop:`。
  - 计划没有规定合并编译必须编过并通过 clippy（那样 SDK 就不能写互斥 cfg），也没有规定用 `default-members` 或 `--exclude` 排除文档包。污染检查如果做成编译期互斥，CI 的 host job 直接失败。实施者只能自己决定。
  - 另一个关键前提只写了「精确转发」：rustify-components 对 rustify-ui 必须 `default-features = false`，否则 csr 会漏进 CLI。
- **证据**：
  - [Cargo.toml:2,27](../../Cargo.toml)
  - [.github/workflows/verify.yml:24,46](../../.github/workflows/verify.yml)
  - `leptos-0.8.22/Cargo.toml` 的 features；`reactive_graph-0.2.15/src/effect/effect.rs:172`；`any_spawner-0.3.0/src/lib.rs:84-91`；`tachys-0.2.19/src/html/property.rs:25`
  - [crates/rustify-components/Cargo.toml:15](../../crates/rustify-components/Cargo.toml)（现为默认 features）。
- **修订**：V-15 / §4.2 列出必须通过的组合矩阵：
  - rustify-ui：默认 / `gpu` / `--no-default-features --features ssr`；
  - 两个文档包用 `-p` 单独跑；
  - 污染检测写明只针对单包编译，实现为测试或运行期断言；
  - union 要么列为必过冒烟项，要么排除，并在 M2 或至迟 M7 提交前同步 CI 与 CLAUDE.md。
- **复核**：改稿后 §4.2 与 V-15 一致。实现后 verify.yml:24/46 通过，`cargo tree -p rustify-docs-cli -e features` 中没有 csr 和 effects。
- **置信度**：冲突本身高，后果中。**阻塞**：M2（文档包入 workspace 时）。

### [P2] F-07 · 最小工具链没有约束原生依赖和支持的操作系统，只在 macOS 上验证

- **定位**：§4.2 L232（新依赖「M2 锁版本」）、§5.1 L251、V-11 L362、§9.3 L378；全文没有 OS 矩阵，PRD 也没有指定。
- **问题**：
  - N4/AC-3（PRD L692）要求只装 Rust 和 CLI 时五条命令全部可用。
  - `check --external-links` 需要 HTTPS 客户端。如果选了默认依赖 native-tls / openssl-sys 的库，在没装 libssl-dev 和 pkg-config 的 Linux 上，`cargo install` 会失败。syntect 默认的 onig 需要 C 编译器。
  - V-11 只在 macOS 上做受限 PATH 安装，而 macOS 的 TLS 走系统框架，这类失败测不出来。受限 PATH 还必须保留平台链接器 `cc`，计划没写。
  - 发布事务（rename、`File::lock`）在 Windows 上是否支持没有声明。
- **证据**：计划行号如上；verify.yml 的 host job 是 `macos-14`。
- **修订**：
  - §4.2 加约束：CLI 依赖不得要求 C 链接器以外的系统库或工具（OpenSSL、pkg-config、CMake、NASM），HTTPS 用 rustls + ring/webpki-roots，并确定 syntect 的正则后端。
  - 声明 P1 支持的 OS，至少 macOS 和 Linux；Windows 明确不承诺，或补验证。
  - V-11 在 Linux runner 上也跑一次 git 安装和五条命令（不需要容器）。
- **复核**：`cargo tree -p rustify-docs-cli -e normal,build` 中没有 `openssl-sys`、`cmake`；Linux 安装记录写入 V-11 证据。
- **置信度**：中（后果取决于依赖选型；缺约束、验证环境测不出来这两点确定）。**阻塞**：M2 锁依赖前。

### [P2] F-08 · F1/AC-1、F1/AC-3、F2/AC-4 中「开发预览」那一半没有检查能观察到，M4 的退出条件却引用了它们

- **定位**：V-5 L356、V-6 L357、M4 L392（退出含 V-5、V-6）、M6 L394、V-9 L360（只对应 F13）；§6 表 L296-300 没有草稿标识。
- **问题**：
  - 这三条 AC 都要求 `dev` 下可观察的行为：
    - F1/AC-1（PRD L187）：新建后「启动开发预览并浏览首页与至少两页」；
    - F1/AC-3（L189）：「`new` 与 `dev`」都输出缺失项；
    - F2/AC-4（L217）：草稿「在开发预览中带草稿标识可见」。
  - dev 到 M6 才有。V-5 只看 new，V-6 只看产物侧的草稿过滤，二者都排在 M4。照字面 M4 无法退出；如果实施者自行切片，这三处 dev 侧又没有后续检查负责。
  - 草稿标识也没有 UI 定义。
  - dev 的内容图保留草稿，「发布页链接到草稿」只在 build 时报错，作者在 dev 中看不到。
- **修订**：
  - V-5 / V-6 拆为 M4 的 build 侧切片和 M6 的 dev 侧切片；dev 侧并入 V-9，写明三条子句。
  - §6 加草稿标识：用文字，不只靠颜色，出现在页头与侧栏项。
  - dev 把「仅生产才报错」的链接问题显示为警告。
- **复核**：改稿后，这三条 AC 各自指向一个在 M6 或之后执行的 dev 侧检查，M4 退出条件不再引用 dev 行为。
- **置信度**：高。**阻塞**：M4、M6。

### [P2] F-09 · dev 是否写入 `build.out`、manifest 有没有模式标记、preview 与 build 并行时怎么办，都没定，可能把草稿部署出去

- **定位**：§2 L187（Docs publish 的调用者含 dev）、§5.4 L278（「防第二个 build/dev 并发发布」）、§3 L202（manifest 没有 mode 字段）、L203（dev 状态「仅进程内」）、§5.3 L270、§5.4 L280（禁止同一目录同时作为 preview 与 build 的目标，但没写机制）、§5.1 L248（preview 没有默认端口）。
- **问题**：两种实现都有问题。
  - 如果 dev 把带草稿和 dev-client 的开发态产物写进 `dist`，preview 无法识别并拒绝，作者部署后草稿外泄，违反 N7/AC-1（PRD L745）。
  - 如果 dev 在整个会话里持有 out 的锁，作者在 dev 运行时就不能 build，而这是常见工作流。
  - preview 与 build 并行，以及 preview 默认端口与 dev 冲突时，用户看到什么都没有定义。
- **修订**：
  - dev 从内存或私有目录提供服务，不写 out，也不持有 out 的锁。
  - manifest 加 `mode: production`，preview 遇到非 production 时拒绝启动。
  - 定义 preview 与 build 并行的行为：preview 按请求读取已提交的产物代次，或明确拒绝并提示。
  - 定 preview 的默认端口与冲突回退。
  - V-9 / V-10 各加一个场景。
- **复核**：改稿后 §3、§5.3、§5.4 对 dev 的输出位置只有一种解读。
- **置信度**：中高。**阻塞**：M4（manifest 契约）、M6。

### [P2] F-10 · 检查的执行时点与里程碑退出、提交点对不上，跨切改动缺少回归触发

- **定位**：
  - V-1 L352、V-5 L356、V-6 L357 写了「M7 重跑/复测/安全收口」，但 M7 L395 的退出只有 V-10、V-11、V-15。
  - M7 声称覆盖「全部 N4/N7」，N7/AC-4 却只映射到 V-7（L358），V-7 不在 M7 执行。
  - V-7 的时点列（L358）缺少 M4 L392 退出条件中的「HTTP/preview/base 切片」。
  - M8 L396 只列 V-7–V-14 和受影响的 V-1–V-3。
  - V-3 L354 规定「M8 仅重跑受影响范围」。
- **问题**：
  - M2 要把 `button.rs:99` 的 `prop:disabled` 改写为属性，7 个示例都在用 Button/Link/Icon。M2 还会给根 `Cargo.lock` 引入 lightningcss、syntect，而根 `[patch]` 把 bitflags/smallvec 指向 makepad fork。这些横切改动会在 M7 被提交，提交前不跑任何 GPU 回归，退化拖到终验才暴露。
  - M5 引入主题 JS/CSS、logo 和 CSS 中的 url 引用链，M7 做安全修复，M9 改模板 README，都改变了 V-4–V-6 的输入。照字面，M8 不重跑这些检查，会引用 M3/M4 的旧证据。F14/AC-2（PRD L494）测试时，有引用链的主题资源还不存在。
- **修订**：
  - 统一 §9.1 的时点列与 §10 的退出列表。
  - V-3 加触发规则：`crates/rustify-ui`、`crates/rustify-components`、根 `Cargo.toml` 或 `Cargo.lock` 有变化时，所在里程碑退出或下一个提交点前重跑受影响的 regression project；M2 退出加「受影响 V-3」。
  - M8 加「受影响 V-4–V-6」。
  - M7 改为「N4 全部、N7/AC-1–3」，或把 N7/AC-4 的断言纳入 M7。
- **复核**：改稿后逐个检查 ID 核对，时点列里出现的每个里程碑都在对应的退出列表中。
- **置信度**：中高。**阻塞**：M2、M7 提交点、M8。

### [P2] F-11 · 跨源资源必然违反站点自带的严格 CSP，校验规则只覆盖了 script 和 style

- **定位**：§5.2(7) L261；§7 L323-324（「框架不能发布 CSP 违例」）；§5.3 L272。
- **问题**：
  - 严格 CSP 是 `img-src 'self' data:`，`media-src 'self'`，frame 回落到 `default-src 'none'`。
  - 作者写的 `![badge](https://img.shields.io/…)`，或原始 HTML 中的 `<iframe>`、`<video src=https://…>`，在产物里必然被浏览器拦截：图片破损，控制台报 CSP 违规，违反 N3/AC-2「CSP 违规为 0」（PRD L670）。
  - D9 追求同一份 Markdown 在 GitHub 上也可读，而 README 里常有徽章。
  - 计划的 tokenizer 规则只列了内联 script、`on*`、`style` 和 `javascript:`，与 L324 自己的原则不一致。
- **证据**：[xtask/src/serve.rs:35-39](../../xtask/src/serve.rs)；计划 L261、L323-324。
- **修订**：§5.2(7) 加规则：Markdown 图片，以及 HTML 中 `img/srcset/source/video/audio/poster/iframe/object/embed` 的跨源 URL 一律报错，建议「下载到 static/」。外链 `<a>` 不受影响。
- **复核**：V-4 / V-10 各加一条负向 fixture。
- **置信度**：中（机制确定；作者实际输入频率是推断）。**阻塞**：M3。

### [P2] F-12 · 页面语言和界面文案的语言没有契约

- **定位**：§0.5 L144「单页 `lang` 属 P1 语义」；§3 L199 的 frontmatter 字段没有 `lang`，而 L194 规定未知键报错；L200 的 Page 模型有 lang；§9.3 L377 参考站点 B 中英各半。
- **问题**：
  - 作者无法声明单页语言：写 `lang:` 会被当未知键报错，中文页只能沿用 `site.lang`，违反 N5/AC-3「正确的 `lang`」（PRD L711）。
  - 界面文案（跳到正文、复制、已复制、目录、上一页、主题）用什么语言没有定。中文站配英文文案而不标注 lang，不满足 WCAG 3.1.2，axe 查不出。
  - v1 的配置 schema 一旦发布，后补字段属于契约变更。
- **修订**：
  - frontmatter 加 `lang`，默认取 `site.lang`。
  - 界面文案按页面语言在 en/zh 之间选（与 SDK 现有 Locale 一致），或给文案元素标 lang。
  - V-13 加断言。
- **复核**：改稿后 L199 与 L200 一致。
- **置信度**：中高。**阻塞**：M3（内容模型）、M5。

### [P2] F-13 · 默认阅读主题直接沿用 SDK token 和作用域 dark 变体，会违反 N5/AC-1 对比度，无脚本时也不能跟随系统模式

- **定位**：§6 L292（沿用 DESIGN.md 语义 token）、L304（无脚本按系统 CSS 呈现）、L306（主色同步到焦点、链接、按钮）、L307；§4.2 L231（SDK CSS 作为虚拟 assets）；V-13 L364。
- **问题**：
  - **链接文字对比度不足**：浅色下链接 `text-primary` #1570ef 放在页面底色 #f9fafb 上只有 4.37:1，低于 4.5:1。
  - **焦点环对比度不足**：焦点环 `ring-ring/50` 混色后对底色约 2.02:1，低于界面图形 3:1 的要求。axe 没有焦点指示对比度规则，V-13 只写了「可见 focus」。
  - **静态页上 token 和 dark 变体不生效**：`sdk.css` 把 token 默认值和 `dark` 变体都绑在 `[data-rustify-scope]` 上，这个属性由 `mount` 写入，而且按设计不跟随 `prefers-color-scheme`。静态页没有 mount，不在根元素上放作用域标记、不重定义 dark 变体，组件的 token 与 `dark:` 都不生效；无脚本加系统深色时，组件和正文配色会不一致，与 F8/AC-1（PRD L361）「跟随系统」冲突。
  - **sdk.css 自带的 `@source "../src"`** 也需要中和。
- **证据**：
  - [crates/rustify-components/css/sdk.css:16-21,90-109](../../crates/rustify-components/css/sdk.css)
  - [src/link.rs:10](../../crates/rustify-components/src/link.rs)、[src/button.rs:13](../../crates/rustify-components/src/button.rs)
  - 对比度按 WCAG 相对亮度公式由源值计算，不是渲染实测。
- **修订**：§6 写明文档站的 CSS 输入契约：
  - 静态根元素带 `data-rustify-scope data-theme`；
  - 文档自己的 `@custom-variant dark` 同时覆盖 `[data-theme=dark]` 和 `@media (prefers-color-scheme: dark)` 下的 `:not([data-theme=light])`；
  - 链接色或页面底色按阅读主题调整，焦点指示改为不透明且 ≥3:1，经调用方 class 和 `tw_merge` 覆盖，不改 SDK 默认值；
  - 中和 `@source`。
  - V-13 加：浅、深两态下计算焦点指示与相邻颜色的对比度。
- **复核**：改稿后 §6 / V-13 有具体值。实现后在浏览器中计算对比度，并截取无脚本加系统深色的画面。
- **置信度**：高（源值加公式）。**阻塞**：M5。

### [P2] F-14 · `theme.primary` 和 `logo` 是单值配置，浅、深两态无法同时可读，v1 又规定未知键报错

- **定位**：§3 L194、L198；§6 L306；账本 F8/AC-3（PRD L363）。
- **问题**：
  - 要在 #f9fafb 上达到 4.5:1，颜色的相对亮度须 ≤0.173；要在深色底 #0c111d 上达到 4.5:1，须 ≥0.20。没有一种颜色能同时满足。同理，单个 logo 在其中一种模式下可能看不见。
  - DESIGN.md 自己就给了浅、深两套主色。
  - 单值契约要么让配置主色在一种模式下不可读，要么由实施者临时决定派生规则。
- **修订**：
  - 允许 `primary = {light, dark}`，或单值加有对比度保证的派生。
  - `logo` 可选 `{light, dark}`。
  - 按钮前景色自动选择。
  - 配置色不达标时 `check` 给警告，`--deny-warnings` 时失败。
  - V-8 / V-13 加一个带配置主色的 fixture。
- **复核**：改稿后 §3 的配置契约有这些字段和规则。
- **置信度**：数学部分高，产品影响中。**阻塞**：M4（配置 schema 冻结前）。

### [P2] F-15 · 默认阅读主题和 dev 诊断浮层缺少 AGENTS.md 要求的设计阶段与设计记录

- **定位**：§6 L290-307、M5 L393、T-6 L410；§12 L439 只引用了 AGENTS.md 的浏览器验收部分。
- **问题**：
  - [AGENTS.md:118-120](../../AGENTS.md) 规定页面、组件、视觉改动「设计阶段用 impeccable」，之后才是浏览器验收。仓内先例是 `.impeccable/surfaces/examples-theme-studio-src-app-rs.md` 的方向契约和 finish review。DESIGN.md 只记录了 Theme Studio 的 shell。
  - 默认主题是所有生成站点的界面。缺少这一步，下面这些都会由实施者临时决定且不留记录：标题字号层级、正文最大宽度、侧栏和目录宽度、页头高度、代码块和提示框样式、浮层布局。F-13、F-14 正是设计阶段本该拦下的问题。
- **修订**：M5 在 T-6 之前加一步：默认主题与诊断浮层各写一份 impeccable surface brief，完成后做 finish review，并把阅读主题写进 DESIGN.md 或其 sidecar。不重新设计产品。
- **复核**：改稿后 M5 的退出条件含设计记录与 finish review。
- **置信度**：高（规则存在）。**阻塞**：M5。

### [P2] F-16 · 跨浏览器和真机的风险要到 M8 才暴露，真机也没有可访问的入口

- **定位**：V-12 L363（只在 M8）、M8 L396、§11 L421（「M8 开始前」才确认）、§9.3 L381；§5.1 L248（preview 只有 `--port`）；§5.5 L288 与 §7 L324（dev 只绑 loopback）；§8 L342 排除上传。
- **问题**：
  - **真机访问不到**：iOS / Android 真机无法访问本机的 127.0.0.1。计划既没设计局域网绑定，也没有可访问的托管。即使改用局域网 http，那也不是安全上下文，手机上剪贴板 API 不可用。实施者到 M8 才会被迫临时决定。
  - **Firefox 口径未定**：N3/AC-4（PRD L672）要求 Firefox 最新稳定版，计划只说明 WebKit 不能代替 Safari，没有说 Playwright Firefox 能否代替真实 Firefox 稳定版。
  - **浏览器下载需要同意**：Playwright 的 firefox/webkit 需要另行下载，按仓库规则属于需要用户确认的环境安装。
  - **兼容问题发现太晚**：主题首帧脚本、clipboard、`details`、`scroll-margin` 等兼容问题，要到 M7 提交之后才会发现。
- **证据**：计划行号如上；[xtask/src/serve.rs](../../xtask/src/serve.rs) 只绑 127.0.0.1。
- **修订**：
  - M5 退出加：V-8 子集的 Firefox / WebKit 自动化冒烟，加一次本机真实 Safari 冒烟。
  - 在 M4 或 M5 决定真机访问方式：给 preview 加显式 `--host`（只读，打印局域网地址，并提示不是安全上下文），或使用用户提供的 HTTPS 托管或隧道。同时写明手机上复制功能的预期结果。
  - Firefox 口径、浏览器下载、设备与走查人的最晚确认点提前到 M5 退出。
- **复核**：改稿后 §11 的确认点为 M5，访问方式已定案。实现后 M5 记录里有跨引擎冒烟结果。
- **置信度**：中高。**阻塞**：M8，改稿后前移到 M5。

### [P2] F-17 · M9 用 GitHub 地址加真实 SHA 验证安装，隐含一道没有登记的推送授权门

- **定位**：§8 L340（`cargo install --git https://github.com/rockie/rustify-ui --rev <SHA>`）、D6 L102、§9.2 L373（「必须针对可达真实提交验证」）、M9 L397、提交点 L399（失败就修复并重提候选）。全文没有出现「推送」。
- **问题**：
  - 「可达」意味着 SHA 已推送到 origin（`github.com/rockie/rustify-ui.git`）。仓库规则是不开分支，所以每个候选都会推到 main，包括失败的候选。
  - 推送是对外发布动作，需要用户授权，但 M9 的前置和 §11 都没有登记，实施者会在 M9 卡住或被迫自行决定。
  - 如果改用 `file://` 验证，又证明不了文档里那条 GitHub 命令能用。
  - M8 的证据只在 M9 提交，推送受阻时会一起滞留在未提交状态。
- **修订**：
  - M9 前置与 §11 加「用户授权推送及推送方式」。
  - 退出拆成两步：在已提交的 SHA 上用 `file://` 克隆验证；推送后在 GitHub URL 上验证，单独记录。
  - 写明用户拒绝推送时 M9 如何判定，例如记为阻塞，或以本地 SHA 有条件完成并登记缺口。
- **复核**：改稿后 M9 的退出条件只有一种解读。
- **置信度**：高。**阻塞**：M9。

### [P3] F-18 · §1.1 列出的 Makepad 借用点不全

- **定位**：§1.1 L155；§1.2 L170、L172。
- **问题**：§1.1 漏了下面这些位置：
  - 组件 `data_table/view.rs:174` 使用 `rustify_ui::observe_resize`；
  - 示例 `vellum/src/{storage.rs:247,app.rs:690,shell/presentation.rs:135}`、`theme-studio/src/gpu_preview.rs:39` 直接使用 `defer_after`、`observe_resize`、`ResizeObservation`；
  - `lib.rs:84-86` 把这些中性 helper 和 `makepad_widgets`、`RegionApp` 放在同一条 re-export 里；
  - `diagnostics.rs` 用了 Makepad 的 `on_resource_failure`，§1.2 列了这个文件，§1.1 没列。
  
  实施者如果把整条 re-export 都 gate 到 `gpu` 下，DataTable 在默认配置下会编译失败。编译期能发现，但要返工。
- **修订**：§1.1 补上这些位置，并写明 `defer`、`defer_after`、`observe_resize`、`ResizeObservation` 始终由 SDK 中性实现导出，路径和签名不随 `gpu` 开关改变。
- **置信度**：高。**阻塞**：否。

### [P3] F-19 · Markdown 解析选项和 HTML writer 的契约有两处缺口

- **定位**：§5.2(2) L256、§5.2(3) L257、ADR-2 L122、§5.2(7) L261。
- **问题**：
  - 选项清单没有 `ENABLE_HEADING_ATTRIBUTES`，而 L257 要求自定义 `{#id}` 优先。开启这个选项后，标题上的 `.class` 和任意属性也会被解析，会影响类提取和 HTML 安全校验。
  - pulldown-cmark 的 `push_html` 会给对齐的表格单元格输出 `style="text-align:…"`。按计划自身的产物校验，这样的页面构建会失败（属于失败安全），F2/AC-1 的表格要到实施时才返工。
- **证据**：`pulldown-cmark-0.13.4/src/lib.rs`（`ENABLE_HEADING_ATTRIBUTES = 1 << 6`）；`src/html.rs:231-234`。
- **修订**：
  - 选项清单加上该标志，标题上只接受 `#id`，`.class` 和其他属性报诊断。
  - 写明使用自有 writer：对齐输出为 class，与 alert、heading id、外链 rel 等改写集中在同一处。
- **置信度**：高。**阻塞**：否，M3 内修正即可。

### [P3] F-20 · 部分 AC 子句没有落到检查断言里

- **定位**：V-4 L355、V-13 L364、V-8 L359、V-7 L358。
- **问题**：下面这些子句没有对应的检查断言：
  - F2/AC-1（PRD L214）的「五种 GitHub alert 以对应语义和样式呈现、同一文件在 GitHub 预览中可读」，V-4 只写了「GFM」；
  - N5/AC-3（L711）的「标题跳级由 `check` 报出」，§5.2 L262 把它定为警告，V-13 / V-4 却没有断言；
  - N3/AC-2（L670）要求「只有 meta 策略生效时，使用当期全部内置交互也零违规」，V-8 不在 `docs-meta` 下运行。
- **修订**：上述断言补进 V-4、V-13；把 `docs-meta` 加进 V-8 的运行范围，或在 V-7 中写明执行全部内置交互。
- **置信度**：中高。**阻塞**：否。

### [P3] F-21 · §0.5 把 F16 整条划到了 P2

- **定位**：§0.5 L143、L145。
- **问题**：PRD 中 F16 的 AC-1–3 属于 P2，AC-4 属于 P4（PRD §7.2–7.3），P4 那一行漏了 F16/AC-4。
- **修订**：改为「F16/AC-1–3 → P2」，并在 P4 行补上「F16/AC-4」。
- **置信度**：高。

### [P3] F-22 · PRD §6.3 中属于 P1 的 UAT 没有承接，也没有写去向

- **定位**：全文没有出现「UAT」；M8 L396 只要求 AC 级证据。
- **问题**：
  - 属于 P1 的有 UAT-1、2、7、10、12、13、14，以及 UAT-4 的无脚本部分。
  - 它们的通过条件跨多条 AC。例如 UAT-1（PRD L844）要求在同一个只装 Rust 和 CLI 的环境里 60 s 内打开首页，并且缺少组件时目录为空。计划把这些拆散到 V-5、V-11、V-14，分别在不同环境执行。
  - PRD §7.5 把 UAT 交给 test-plan，计划没有说明。按 AC 计的覆盖已经完整，所以定为 P3。
- **修订**：在 M8 加一张短表，把每个 P1 UAT 映射到检查 ID，并补一次作者主路径的串联走查；或者写明交给 test-plan，并说明是否构成 M8 的门。
- **置信度**：高。

### [P3] F-23 · V-3 的改前基线怎么采集、基线本身已有失败时怎么判定，都没写

- **定位**：V-3 L354、§9.2 L371、M1 L389、§11 L422。
- **问题**：
  - 现有配置下，与这 7 个示例相关的 regression 调用约 16 次，另需 `/tools/demo/`、`/tools/utility-lab/` 子路径副本；F-02 修订后还要加上 theme-export。CLAUDE.md 记载单项目 fusion-basic 约 47 min、property-workbench 约 38 min，M1 要完整跑两轮。
  - 计划没写基线是在改动前跑，还是像 Vellum 那样在 b0a6bc5 上 `git worktree add`。
  - 计划也没写基线已有失败或不稳定用例时，F17/AC-4（PRD L595）的「全部通过」怎么判定。
- **修订**：
  - 基线固定为在 b0a6bc5 的 worktree 上执行，记录每个 project 的通过与失败清单。
  - 判定规则写成「失败集合不扩大，基线本来失败的项单列」，或要求先修复。
- **置信度**：中。

### [P3] F-24 · 提交点之间跨度过大

- **定位**：提交点 L399。
- **问题**：
  - M2–M7 共 6 个里程碑（两个新 crate、SDK 的 SSR 改动、锁文件），到 M7 才提交。
  - M8 是可能跨多个对话、还要做性能或兼容修复的终验，只在 M9 提交。
  - 工作树里已有无关的 dirty 文件，跨对话丢失工作或混提交的风险偏高。
- **修订**：在 M4（第一次可部署产物，V-10 基本门已过）和 M8（P1 终验）后各加一个提交点。
- **置信度**：中。

### [P3] F-25 · 根 `[patch]` 让 CLI 依赖图里出现 `makepad/libs/{bitflags,smallvec}`，「无 Makepad」的判定标准需要写清

- **定位**：§3.1 L210、V-1 L352、V-11 L362。
- **问题**：
  - git 安装同样使用根 workspace，patch 会生效。pulldown-cmark、lightningcss 依赖的 bitflags / smallvec 会解析到 `makepad/libs`。
  - 按 PRD 的包名判定可以通过，但如果实施者用 `cargo tree | grep -i makepad` 检查就会失败，需要重新决策。另外，以后修改 fork 也会悄悄影响 CLI。
- **证据**：[Cargo.toml:33-35](../../Cargo.toml)。
- **修订**：V-1 / V-11 写明判定标准：按包名前缀判定，另附已知上游原样副本的白名单，并核对 `sources verify` 无漂移。或者写明文档包不受该 patch 影响的做法。
- **置信度**：中。

### [P3] F-26 · CLI 的安装、升级、回滚没有可区分的版本身份

- **定位**：D6 L102、§8 L340、L342、§3 L202（manifest 里的 CLI 版本）、M9 L397。
- **问题**：
  - workspace 版本固定为 `0.1.0`，没有 tag。`--version` 和 manifest 里的 CLI 版本在每个提交上都一样，无法区分。
  - 用户可见文档是否写入已验证的 SHA 没有写清，升级和回滚命令也没有给出。
- **修订**：
  - 文档写入已验证的 SHA 或 tag，给出升级、回滚命令，以及用 `cargo install --list` 查看已装 rev 的方法。
  - 在 `--version` 和 manifest 里体现 rev 或递增版本。
- **置信度**：中。

### [P3] F-27 · 生成的工程没有 `.gitignore`，私有构建区的位置也没定

- **定位**：§5.1 L245、§5.4 L278-279。
- **问题**：`dist`、staging、backup 这些同级目录和事务日志会出现在作者的 `git status` 里，容易被误提交。
- **修订**：定下私有构建区的位置；模板带 `.gitignore`；V-5 加断言。
- **置信度**：中。

### [P3] F-28 · dev 诊断浮层被关掉之后的状态没有定义

- **定位**：§5.5 L287、§6 L300。
- **问题**：作者关掉浮层后再保存一次仍然出错的内容，页面不刷新、浮层也不出现，看起来像「保存没生效」。
- **修订**：诊断集合变化时重新显示浮层，或在关闭状态下保留一个常驻小标记，例如「N 个错误 · 显示上一个可用版本」；V-9 加断言。
- **置信度**：中。

## 2. 待核实 / 待决策

| # | 缺什么证据或决定 | 影响 | 谁提供 | 最晚确认点 |
| --- | --- | --- | --- | --- |
| V-1 | 用户是否授权推送到 origin/main、以什么方式推送（F-17） | M9 退出判定 | 用户 | M9 开始前，宜在改稿时登记 |
| V-2 | 本机 cargo 1.98.1 下 `cargo install --git file://…` 是否沿用根 `[profile.release]`、`[patch]` 与锁文件。目前依据是 cargo master 源码和官方文档，未实测 | F-04 的策略选择、F-25、安装耗时 | 实施者排练 | M2 选定高亮后端前 |
| V-3 | 真实 Safari、iOS / Android 设备与走查人；Playwright Firefox / WebKit 能否下载；Chrome 固定版本的来源。按仓库规则，环境安装需要用户确认 | V-12、V-14 冻结环境、F-16、F-03 | 用户 | M2（Chrome）、M5（其余） |
| V-4 | C-5「用户确认沿用 Q1/Q3/Q5 默认」只有计划自身 SDLC 事件 `afc6e75a` 自述，没有原始对话或决策记录可查 | 浏览器矩阵与阈值是否为合同 | 用户 | 改稿时 |
| V-5 | syntect 实际输出中是否有 `block`、`table` 等原子；two-face 在 fancy-regex 后端下能否覆盖 TypeScript、TOML；lightningcss 能否与 `serde =1.0.229` 等精确锁定共存 | F-05、F-04、F-07 | M2 spike | M2 出口 |
| V-6 | 根 `[patch]` 不会传给仓外 git 依赖：默认 DOM 依赖图能否不打补丁、用上游 bitflags / smallvec 编译 | F-01 | F-01 的仓外 fixture | M1 出口 |
| V-7 | F1/AC-3 对内容模板而言的「所需工具链组件」指什么。CLI 由 cargo 安装、内容命令又不调用 cargo，受限 PATH 下缺 rustc 的场景是否有意义 | V-5 的断言设计 | 计划作者 | M4 前 |

## 3. 覆盖摘要

- **执行方式**：4 个独立只读 subagent（general-purpose，模型继承）并行，分别负责 facts、architecture、schedule、delivery。主 agent 维护需求账本，逐条复核候选问题的计划段落与原始证据，并去重合并。reviewer 之间的冲突主要在严重度上，没有需要追加追问的事实冲突。
- **主 agent 亲自复核的证据**：
  - theme-export 的依赖、构建脚本与 CI 接线；
  - `web/loader.js` 的静态 import；
  - utility snippet 先例；
  - 剪贴板 cfg；
  - 根 profile 与 patch；
  - `button.rs:99`；
  - `lib.rs:84-86` 的 re-export；
  - DataTable 与示例对 resize/defer 的使用；
  - pulldown-cmark 的选项与表格 style；
  - syntect 的 `ClassStyle` 与正则 `expect`；
  - `sdk.css` 的 token 和 dark 变体；
  - 链接与焦点环对比度（按公式计算）；
  - `docs/compatibility.md` 与 `serve.rs` 的 CSP 逐字一致；
  - AGENTS.md 的 UI 设计规则。
- **四维覆盖**：
  - facts：§1.1 每行「已核实」都追到代码，没有发现错误陈述，只有一处列举不全（F-18）。
  - architecture：ADR-1/2/3、D1、发布事务、dev 机制、CSP、安装渠道都做了可行性判断。D1、ADR-1、ADR-2、ADR-3 的方向都成立，缺口集中在 panic、class 冲突、平台、性能机制几项。外部事实（Cargo 文档、`File::lock` 自 1.89 稳定、RUSTSEC-2025-0068 等）于 2026-10-10 查询。
  - schedule：PRD §7.1 的 65 条 P1 AC 逐条映射到 V 检查。
  - delivery：覆盖作者、读者、应用开发者三条主路径。
- **脚本**：
  - `check_paths.sh`：MISSING 10 个。9 个是 ★ 新增路径；`docs/project-status/protocol.md` 是协议里的兼容提法，不是错误引用。
  - `check_progress.sh`：0/9，ERROR 0；唯一 WARN 来自无关的 dirty 文件。
  - `check_plan.py`：ERROR 0；18 条 WARN 都是自定义 WORK-* ID，已人工核对。
  - 脚本通过不证明行为或验收成立。
- **未执行**：没有运行 build、test 或 Playwright。UI 部分只做了文档推演，没有可运行的 docs 页面，也未在浏览器中验证。对比度是由源值算出的，不是渲染实测。

## 4. 映射与实施编排

**映射核对范围**：PRD §7.1 的 65 条 P1 AC（18 个 WORK 需求），每一条都能在 V 表的「需求/验收行」找到，AC 编号与含义没有错号。账本「设计/验收落点」列与 V 表一致。V-1 到 V-15 都有设计依据和执行里程碑。§0.5 的 P3、P4 排除与 PRD §7.2 一致，P2 行只有 F16/AC-4 一处偏差（F-21）。恢复快照、完成记录占位行、记录路径 `docs/validation/docs-ssg-p1/` 符合 CLAUDE.md 的 Plans 约定。断链只有以下几类：

- 只覆盖了 build 侧：F1/AC-1 与 F1/AC-3 的 dev 部分、F2/AC-4 的开发预览部分（F-08）。
- 断言缺子句：F2/AC-1、N5/AC-3、N3/AC-2（F-20）。
- 执行时点不一致：V-1、V-5、V-6、V-7、N7/AC-4，M8 缺 V-4–V-6，跨切改动缺 V-3 触发（F-10）。
- 缺失节点：tests/theme-export（F-02）、B 语料生成器与测量工具（F-03）、真机访问方式（F-16）、推送授权（F-17）、仓外 DOM 消费者 fixture（F-01）。

**依赖边**：只列需要调整的部分；其余沿用计划 §10 与 §10.1。

| 前置 → 后置 | 产物与解锁证据 | 判定 |
| --- | --- | --- |
| b0a6bc5 worktree 上的 V-3 基线 → M1 退出 | 每个 project 的通过与失败清单（含 theme-export） | 采集方法待写明（F-23） |
| DOM 启动契约定案 → T-1 / T-2 | ADR-1 的分发方式与配方 | 缺失（F-01） |
| T-1 + T-2 → V-2 | 中性 hooks 加 DOM host | V-2 是两者的汇合门，不只是 T-1 的检查 |
| tests/theme-export 开启 gpu → M1 提交点 | build.sh 加两个 project | 缺失（F-02） |
| M2 组件与锁文件改动 → 受影响 V-3 → M7 提交点 | regression 结果 | 缺失（F-10） |
| B 生成器与测量工具 → V-14（M2 门 / M4–M6 早期 / M8） | 语料与计时 | 缺少负责的里程碑（F-03） |
| M6 dev → V-5 / V-6 的 dev 侧切片 | dev 行为 | 原稿倒置在 M4（F-08） |
| 设备、浏览器下载、访问方式 → V-12；用户推送授权 → M9 | 环境与授权 | 隐藏前置，确认太晚（F-16、F-17） |
| M8 → M9 文档 | — | 过粗：README 和指南可在 M7 之后与 M8 并行撰写，只有候选提交和安装验证需要等 M8 |

**建议波次**：没有工期估算，只给拓扑顺序。Playwright 全局串行，一次一个 project。

| 波次 | 前置及解锁证据 | 可同时做的工作 | 写入边界 / 共享负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| W0 基线 | b0a6bc5 worktree，release 与子路径构建，theme-export `build.sh` | 主工作树写 T-1；DOM 启动契约设计 | worktree 只读；主实施者写 M1 记录 | 占用 Playwright 端口 4173–4187，CPU 与 mbx 缓存共用 | 基线清单 |
| W1 M1 | T-1 与启动契约固定 | T-2（DOM JS、`tests/dom-only`、仓外 fixture、docs config）∥ T-3（7 个示例加 theme-export 的 Cargo） | 根 Cargo、SDK lib、`web/loader.js` 由主实施者单写 | dom-only 与 V-3 串行 | V-1、V-2（含仓外）、V-3（含 theme-export）、V-15 → 提交 |
| W2 M2 | M1 退出 | 组件 SSR 与依赖锁定（主实施者）∥ B 生成器与测量工具 ∥ grammar 与 panic 策略 spike | Cargo.lock 单写 | 受影响 V-3 串行 | V-4 切片、有门槛的 V-14、组合矩阵、受影响 V-3 |
| W3 M3 ∥ T-6 先行 | M2 契约；T-6 用 fixture，设计阶段先完成 | T-4 → T-5；impeccable 设计 → T-6 | Page / Diagnostic 类型单写 | 主机测试 | V-4 全组 |
| W4 M4 ∥ M5 | M3 的 V-4 | CLI 与产物 ∥ T-6 / T-7 | manifest、URL、发布事务单写 | 浏览器项目串行 | M4：build 侧 V-5/V-6、V-7 切片、V-10 基本门、500 页早期数据 → 建议提交；M5：V-7、V-8、V-13 局部、跨引擎与 Safari 冒烟、真机访问方式定案 |
| W5 M6 | M4 的 V-10 与 M5 UI | — | dev 状态单写，fixture 编辑目录隔离 | docs-dev 串行 | V-9（含 dev 侧 V-5/V-6、草稿标识）、短轮连续保存 |
| W6 M7 ∥ M9 文档草稿 | M4–M6 | 故障注入 ∥ 消费者演练（macOS 与 Linux）∥ 文档撰写 | 文档只写 `docs/` 与 README | 断网演练单独执行 | V-10、V-11、V-15、受影响 V-3 → 提交 |
| W7 M8 | M7；设备与浏览器已确认 | 自动化项目串行 ∥ 人工真机走查 | 证据单写 M8 记录 | 一次一个 project | V-7–V-14、受影响 V-1–V-6、UAT 映射 → 建议提交 |
| W8 M9 | M8 通过；推送授权 | — | 候选提交由主实施者负责 | 需要联网 | `file://` 与 GitHub 两步安装、V-15 → 提交 |

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

- **建议状态**：`Blocked`。计划自报 `Ready`，两者不一致。
- **理由**：
  - F-01 的消费者启动契约属于外部契约。它决定 M1 第一项任务的文件布局，按现稿也无法验收 PRD P1「应用开发者构建纯 DOM 应用」。
  - 另有 16 条 P2，分别会在 M1（F-02）、M2（F-03 到 F-07、F-10）、M3–M6（F-08、F-09、F-11 到 F-15）、M8–M9（F-16、F-17）迫使实施者临时决策，或让验收无法判定。
  - 整体技术路线成立，问题都能就地修订，不需要改变范围或产品契约。
  - 唯一可以先行、且与这些问题无关的工作，是在 b0a6bc5 worktree 上采集 V-3 改前基线。
- **修订顺序**：
  1. **先修前提与契约**：
     - F-01：DOM 启动分发、配方、仓外 fixture；
     - F-02：theme-export 纳入 M1；
     - F-06：workspace 组合矩阵；
     - F-04：panic 策略；
     - F-07：原生依赖与 OS 约束；
     - F-17：推送授权门写进 M9 与 §11。
  2. **再修设计**：
     - F-05：高亮 class 前缀；
     - F-11：跨源资源规则；
     - F-09：dev 输出位置与 manifest mode；
     - F-12：lang 契约；
     - F-13、F-14：阅读主题 CSS 与主色配置契约；
     - F-15：设计阶段；
     - F-19：Markdown writer。
  3. **然后修映射与编排**：
     - F-03：性能门与早期测量；
     - F-08：dev 侧切片；
     - F-10：时点与回归触发；
     - F-16：跨引擎前移、真机访问；
     - F-20 到 F-24。
  4. **其余 P3**：F-18、F-25 到 F-28。
- **解除阻塞的可观察条件**：
  - 改稿后，§4.1 / ADR-1 写明 DOM 启动 JS 的分发方式、消费者配方，并有仓外检查 ID。
  - M1 的交付物和退出条件含 tests/theme-export。
  - §4.2 / V-15 有组合矩阵与原生依赖约束，§5.5 有 panic 策略，ADR-3 有高亮 class 前缀，V-14 的 M2 切片有可判定的门槛。
  - §11 登记推送授权、设备、浏览器下载和 Chrome 固定版本，每项都有最晚确认点。这些用户决定只阻塞 M5、M8、M9，登记后不妨碍 M1 开工。
  - 复核：P1、P2 都关闭后可以改判 `Ready`。
- **评审限制**：本报告只评审计划文本。计划评审通过不代表功能已实现或可上线。

**下一步**：`/apply-doc-review docs/plan/DOCS-SSG-P1.review.md`

### 附：非阻塞建议（同样可行的其他做法，不算缺陷）

- YAML 不要用已弃用的 `serde_yaml`，也不要用 `serde_yml`（RUSTSEC-2025-0068，unsound 且停止维护），可选 serde_norway、saphyr 一类。用预编译 dump 时可关闭 syntect 的 `yaml-load`。
- git 更新时间按 UTC 固定格式渲染，避免因时区不同导致两台机器的产物字节不同。
- ADR-1 中 Pace 的两份定义可以用 `#[path]` 共享同一份源文件，或用合同测试断言变体和 derive 一致。
- `File::lock` 自 Rust 1.89 稳定，不需要第三方锁库；可以声明 `rust-version`。
- 若资源目录以 `_` 开头，部署说明需提示 GitHub Pages 的 Jekyll 行为，或在产物中输出 `.nojekyll`。
- CSP 字面值不要靠 xtask 链接 rustify-docs 来做等值测试，否则 `mbx xtask` 自身会进入 feature union。

## 修订处置与复核

- 来源：本报告原始 sha256 `2eb10b43a9d0d1c63d2032217d1cb01d73cc7d02d47247aaa38e91b83a1fe0c5`，原 565 行的问题、证据、结论及所审版本完整保留；本节属于 apply-doc-review 处置，不是新一轮独立评审。
- 目标：[DOCS-SSG-P1](DOCS-SSG-P1.md)；修前 sha256 `dded28b26475af84007a60478d6f4032a3838dd20f9896d1a1b7d63f1730b37a`，修后 sha256 `97b094c475d38a5bb77a6562ebcff50457d7953c76743bb65347fc2e08e0cb1c`（527 行，未提交）。
- 复核日期与代码基线：2026-10-10 +08:00；main / dirty@b0a6bc5b9010e57d1f0d758a37af573ea0712faa。代码未改，未启动计划实施、产品测试、浏览器验收或安装/推送；实施协议保留，进度仍 0/9。
- 本次范围：F-01–F-28、§2 的七项待核实/待决策、§4 映射与依赖缺口及影响一致性的非阻塞建议。PRD 与其处置只读，不扩大 P1 范围。表内“已解决”仅指文档缺口；所有实现后的检查仍是未来里程碑要求。

### 逐条处置

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果 / 剩余动作 |
| --- | --- | --- | --- | --- |
| F-01 | 成立：根 loader 静态依赖 GPU/glue，SDK utility 有 crate 内 snippet 先例；clipboard 有 unstable cfg 闸，当前计划没有仓外启动闭环 | 待决策 | D9、ADR-1、§1.2、§4.1、T-2、V-1/V-2、M1/M9；写出 SDK 资源、工厂、协议、普通 Cargo/bindgen/CSS 配方与仓外 git fixture 推荐稿 | 部分解决；用户未确认 D9，推荐稿不能视为外部合同定案。确认后同步全稿并复核；仓外实际启动留 M1/M9 |
| F-02 | 成立：已读 theme-export 两 Cargo、独立锁、build.sh、generator 字体读取与 CI/project 接线 | 已修订并复核 | §1.1–1.2、§3.1、V-3、§9.2、M1、T-3；两包显式 gpu，保留宿主字体目录、根/子路径与两 project | 已解决；交付物、基线和出口均含消费者。真实构建/回归留 M1 |
| F-03 | 成立：原 M2 门无判据、B/harness 无归属，配置保存仍须满足 PRD N2/AC-1 | 已修订并复核 | §5.5/§7、V-14、§9.3、M2/M4–M6/M8、T-8、§11；hash 片段缓存、有界并行/独立 Owner与整代提交，分阶段测量及 go/no-go | 已解决；M2 有独立可执行的风险门，不借尚未实现的 build/dev；正式 AC 仍由后续实际链路测量 |
| F-04 | 成立：根 release 为 abort/fat，syntect 正则惰性编译存在 expect；已核对官方 Cargo profile 规则，单测不是安装 profile 证据 | 待决策 | D10、ADR-2、§5.5、§8、V-9/V-11/V-15、M2/M6/M9；提出 docs-cli unwind/thin、每代隔离与丢弃失败代可变对象 | 部分解决；待 D10；git 安装 profile 排练与安装二进制 panic 注入仍未运行，分别归 M2/M6/M9 |
| F-05 | 成立：syntect html.rs 的默认 scope 原子会撞工具类，SpacedPrefixed 提供独立前缀 | 已修订并复核 | ADR-3、§5.4、V-4、M2/M5；固定 hl- 前缀，高亮节点不进候选，单独配色 CSS | 已解决；候选断言与 computed inline 已有时点，实际 span 输出/浏览器计算仍留 M2/M5 |
| F-06 | 成立：已读 CI workspace 命令、两组件依赖及 Leptos/effects/SSR 源码；默认 feature 泄漏与 union 风险确实存在 | 待决策 | D10、§1.2/§4.2、V-15、§9.2、M2；components 禁 SDK 默认，明确 default/gpu/ssr、docs 两包与 exclude 矩阵，不加 SDK 互斥 compile_error | 部分解决；矩阵稿已一致，分组 CI 方案尚待 D10 确认；未来在新包入 workspace 时同步 CI/README/CLAUDE |
| F-07 | 成立：最小工具链原未覆盖 OS/TLS/grammar 后端，现 CI 只有 macOS；syntect 有 fancy-regex 后端 | 待决策 | D10、§4.2、V-11、M2/M7、§11；推荐 macOS/Linux、无额外系统库、rustls 根证书和禁 onig 的 grammar 方案 | 部分解决；待 D10 确认 OS/约束。具体版本、许可、11 语言与两原生 runner 安装留 M2/M7，不宣称已集成 |
| F-08 | 成立：PRD F1/AC-1、3 与 F2/AC-4 明确含 dev 行为，原 M4 时点倒置 | 已修订并复核 | 账本、§5.2/§6、V-5/V-6/V-9、M4/M6、UAT-1/2；new/build 与 dev 切片分工，草稿双处文字标记、dev 草稿链接警告 | 已解决；M4 不再要求未交付的 dev，三个 dev 子句均由 M6 的 V-9 接住 |
| F-09 | 成立：原 dev 输出/锁与 preview 的解释互相冲突，manifest 无模式 | 已修订并复核 | §2–3、§5.1/§5.3–5.5、V-9/V-10、M4/M6；dev 只在内存、不写/锁 out；production manifest；preview 共享锁、build 独占锁及明确拒绝；4189/回退 | 已解决；dev+build 与同 out 的 build/preview 各有唯一合同和负向检查 |
| F-10 | 成立：原 V-1/5/6/7 时点和出口不一致，组件/锁/CSS 改动会使旧证据失效 | 已修订并复核 | V 表全部时点、§9.2、M2/M4/M7/M8/M9；M2 受影响 V-3、M7 重跑/安全门、M8 受影响 V-4–6 与 M9 模板重验 | 已解决；逐组核对执行时点/切片与出口。无法证明横切无影响时跑全范围；文档一致性不算回归通过 |
| F-11 | 成立：Strict CSP 的资源指令不能允许任意远端图片/media/frame | 已修订并复核 | §5.2(7)、V-4/V-10、M3/M7；按资源上下文拒跨源、定位并建议 static，普通 a 外链保留 | 已解决；补强为 iframe/object/embed 同源也拒绝，与 default/object 策略一致；不照抄仅“跨源报错”的不足建议 |
| F-12 | 成立：frontmatter 无 lang 与 Page/lang、未知键拒绝冲突；正文/界面语言需分别标注 | 已修订并复核 | §3/§6、V-13、M3/M5；frontmatter.lang 默认 site.lang，en/zh 主标签选界面，其他有效标签英文界面标 lang=en | 已解决；语言字段、HTML/文案与检查一致，不扩到 P3 语言路由 |
| F-13 | 成立：已读 sdk.css 的 source/作用域 dark/token 与 Button/Link 半透明 focus，按 WCAG 公式核对源色 | 已修订并复核 | §6、V-8/V-13、M5；静态 scope/system、显式/系统深色 CSS、虚拟输入去 source；阅读默认 #175cd3/#53b1fd 与不透明 focus | 已解决；具体值及 ≥4.5:1/≥3:1 计算判据均在稿中。实际 computed style、状态色与无脚本深色截图留 M5/M8 |
| F-14 | 成立：单一主色不能同时保证两默认底色的普通文字对比度，logo 也需可声明模式资源 | 待决策 | D11、§3/§6、V-8/V-13、M4/M5、§11；推荐 primary 成对/logo 单或成对、按钮黑白前景、低对比警告/deny-warnings | 部分解决；待用户选择成对还是单色派生。不得先发布旧单色 schema，选择后同步契约和 fixture |
| F-15 | 成立：仓库 UI 规则要求 impeccable 设计先于实现/验收，原计划只给阅读方向 | 已修订并复核 | §1.2/§6、M5/M6、T-6/T-7；两 surface brief、DESIGN.docs-ssg.md 与 finish review | 已解决；先设计后实现的门和归属明确，本轮没有实际 UI 产物/设计验收 |
| F-16 | 成立：loopback 无真机入口，下载/设备确认太晚，引擎不能证明最新稳定版矩阵 | 已修订并复核 | §5.1、V-12、§9.3、M4/M5/M8、§11；显式只读 preview --host，HTTP 复制失败/手选，M5 冒烟/确认，M8 真实矩阵 | 已解决；固定 Chrome 的最晚点为 M2，其余为 M5。登记门不代表已获下载授权或已有设备，真实环境仍需未来确认 |
| F-17 | 成立：GitHub rev 安装需要远端可达，原隐藏推送授权门 | 已修订并复核 | §8、M9、提交点、§11；先具体本地候选验证，再取得用户推送指示，对同 SHA 远端复验；无授权 M9 阻塞 | 已解决；没有把当前调用解释成推送授权。M8 增加提交点，证据不滞留在该门之后 |
| F-18 | 成立：已读 lib.rs/diagnostics 与 DataTable、Vellum、Theme Studio helper 调用 | 已修订并复核 | §1.1/§4.1；中性 helpers 保留 SDK 路径/签名和 wasm32 可用性，GPU 类型/资源 hook 分开 gate | 已解决；不能把整条现有 reexport 都 gate 到 gpu，保留实际消费者合同 |
| F-19 | 成立：pulldown-cmark heading-attrs 还接受 class/其他属性，默认表格 writer 输出 style | 已修订并复核 | §5.2(2–3)、V-4、M3；启用 heading attrs 但只接受 id，统一 writer 输出 class 对齐和 alert/rel/id | 已解决；自定义锚点、表格、HTML 禁 style 不再冲突，含负向断言 |
| F-20 | 成立：PRD 的五 alert/GitHub 预览、跳级诊断、meta-only 全交互原未显式承接 | 已修订并复核 | V-4/V-7/V-8/V-13、M3/M5/M9；本地源格式→浏览器语义/样式→授权可达 GitHub 文件；跳级警告与 deny-warnings | 已解决；docs-meta 覆盖全部基础交互，检查执行阶段与输入就绪相符 |
| F-21 | 成立：PRD §7.2–7.3 将 F16/AC-4 放 P4 | 已修订并复核 | §0.5 | 已解决；P2 只排 F16/AC-1–3，P4 明列 AC-4，无新增 P1 工作 |
| F-22 | 成立：PRD §6.3 的 P1 UAT 原没有计划去向，AC 分散不能替代作者串联路径 | 已修订并复核 | 新 §9.4、M8；八个 P1/局部 UAT 对应 V 组，同环境作者链路；跨期不提前终验 | 已解决；不以未来 test-plan 代替 M8 的串联观察，不在本轮另启测试计划技能 |
| F-23 | 问题成立，报告的“失败集合不扩大”选项低于 PRD F17/AC-4 全部通过要求 | 已修订并复核 | V-3/§9.2/§11、M1；固定 b0a6bc5 worktree，按 project 保存完整用例结果与环境，全部适用原回归通过才退出 | 已解决；采用报告的“先定位/修复既有失败”分支，不采纳弱化验收的分支。无自动豁免 |
| F-24 | 成立：原 M2–M7/M8–M9 跨度让已验证成果迟迟不提交 | 已修订并复核 | §10 提交点、M4/M8/M9 | 已解决；保留原 M1/M7/M9 并新增 M4/M8，每次先过该阶段门并回写；本轮不提交 |
| F-25 | 成立：根 patch 的通用包路径会触发误判；sources verify 实现允许报告漂移后成功退出 | 已修订并复核 | §3.1、V-1/V-11/V-15；按 PRD 包名禁项，登记 bitflags/smallvec 来源，逐文件另断言无新增漂移，仓外不复制 patch | 已解决；本轮对导入指纹核对 bitflags 43/43、smallvec 8/8，无变更/未登记文件。只证明导入基线一致，不声称与 crates.io 原版逐字相同；git 安装 patch 生效仍留 M2 |
| F-26 | 成立：固定 workspace version 不能区分实际 CLI 行为候选，原升级/回滚不具体 | 已修订并复核 | §3/§8、V-11/V-15、M9；独立 CLI version、--version/manifest 同值，README 填实际 SHA，install --list 与 --force 升级/回滚 | 已解决；版本与来源可区分，不在消费者编译时要求 git 取 rev。D10 定案后确定安装 profile；实际候选安装留 M9 |
| F-27 | 成立：原生成目录缺 ignore/私有区归属 | 已修订并复核 | §5.1/§5.4、V-5、M4；out 同文件系统 .rustify-docs/<输出标识>/，staging/backup/log/lock；模板 ignore 默认 out/私有区 | 已解决；git status 检查已在 new→build 后，已有工程自定义 out 不擅改忽略规则 |
| F-28 | 成立：原关闭浮层后的 last-good 状态与重复保存没有可观察说明 | 已修订并复核 | §5.5/§6、V-9、M5/M6；常驻错误数/last-good 标记、可展开；重复失败更新 revision/数量，修复清除 | 已解决；首构建失败、关闭/再次失败/修复均有定义，不抢阅读焦点 |

### 决策与未完项

用户 2026-10-10 明确回答「沿用这些默认和验收合同（推荐，保持现有范围）」。据此 C-5 采用 Rustify Docs / rustify-docs、完整桌面 Chrome/Firefox/Safari 与真实 iOS/Android 矩阵，以及本期 ≤100 KB、LCP≤2.5 s、CLS≤0.1、保存≤1 s、新建≤60 s、500 页构建≤20 s 的合同；测量后不自动放宽。这个新确认接续原自述，不能倒写成此前已存在原始决定。

已通过交互工具提出 D9、D10、D11 三组选择，当前均未收到回复；默认推荐项不算确认。各推荐方案的完整约定已写入计划，对应五条 finding 仍待决策。D9 阻塞 T-1/T-2，D10 阻塞 M2，D11 阻塞 v1 配置冻结；整份计划暂为 Blocked，只可先采集不依赖它们的改前基线。用户回复后应同步决策表、ADR、契约、检查、风险与处置结果，再复核。

§2 中原待核实编号与计划 V 编号属于不同来源，以下以本报告的编号接续：

| 原待核实项 | 当前处置 / 剩余动作 |
| --- | --- |
| V-1 · 推送授权 | 已登记 M9 的具体候选本地验证→用户推送指示→同 SHA 远端验证；本轮没有推送授权，当前不请求未来未知候选的授权 |
| V-2 · git 安装 profile/patch/lock | 官方文档规则已核对，真实 git 安装未运行；推荐 docs-cli profile 待 D10，实际 file:// 排练在 M2 出口前，不能用 cargo master/单测冒充本机证据 |
| V-3 · 设备/下载/固定 Chrome | 最晚点与可达只读入口已登记：Chrome M2，其余 M5，完整矩阵 M8；当前没有新增环境/下载授权，缺环境按该门阻塞 |
| V-4 · C-5 原始来源 | 本轮用户新确认已落入 C-5 与本节；已解决。只覆盖本期列明默认/合同，不补造先前决定或跨期验收事实 |
| V-5 · 高亮输出/grammar/锁兼容 | 前缀与候选隔离已写，输出/context/11 语言、fancy-regex 及精确 serde 锁的真实测试仍归 M2；不声称已测 syntect 产物 |
| V-6 · 仓外无 patch | D9 推荐稿已给仓外 git fixture 与普通配方，须在 M1 出口/M9 远端候选实际跑；当前没有编译/启动证据 |
| V-7 · 内容模板缺工具 | §5.1 明确只检查 rustc 稳定工具链可用、不要求内容模板 wasm32、不编译站点；已装 CLI/移除 rustc 的 new/dev fixture 分别在 V-5/V-9；文档层面已解决 |

非阻塞建议：更新时间采用 UTC 固定格式；Pace 保留既定类型身份和合同测试，不另扩共享源码重构；输出 `.nojekyll`；xtask 不链接文档库做 CSP 比较。YAML 具体库与依赖维护/安全/许可在 M2 选型取证，不在未验证时锁定报告提到的候选版本；OS 文件锁与所需 rust-version 同阶段实测确定。

### 修订后结论与验证

**文档层面已修订并复核 23 项，待用户决策 5 项；当前 Blocked，实施仍 0/9。** 五项是 F-01、F-04、F-06、F-07、F-14，集中对应三组选择。其余条目有设计、任务/里程碑与检查落点，关闭不表示实现验收通过，也不改写原独立评审结论。

| 本次实际检查 | 结果与限制 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P1.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 30 个路径 token，MISSING 12；人工核对为 9 个 ★ 拟新增路径、2 个 public/pkg 生成示例路径、1 个原协议的 docs/project-status 兼容路径。复用路径无缺失；工具不识别所有 brace/无斜线路径，已补人工核对 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P1.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 0/9，9 个里程碑、0 条实施记录，ERROR 0/WARN 1。计划为未跟踪文件，脚本的 git diff 陈旧提示不能识别其修改；已按修前快照确认实际改稿，既有 dirty 文件不当成实施 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P1.md` | ERROR 0/WARN 18，均为沿用的 WORK 自定义 ID；逐条核对 PRD 来源，无新增/错配需求 |
| P1 AC 与 V 表核对 | 从 PRD 的 AC 行读取 18 个需求/65 条 P1 AC，与 V 表显式引用展开对照：65/65，无缺项、无纳入跨期 AC；另人工核对 dev 子句、meta 全交互、alert/GitHub、UAT、执行切片与出口 |
| 来源/历史保全 | 通用副本 51 个文件与 sources.lock 导入指纹一致；原实施协议逐字保留；PRD sha256 未变；原报告 565 行作为不变前缀保留；只修计划、处置与受影响 SDLC，不改代码/无关 dirty 文件 |

以上为文档结构、静态来源与一致性检查，未运行产品 build/test/Playwright、500 页/千次保存、真实设备或安装故障注入；本轮没有页面可运行，未在浏览器中验证。执行这些检查属于将来实施出口，不是本轮缺失的文档测试。

关联同步需求：上游 PRD §5.2 Q1/Q3/Q5 仍写非阻塞默认/待确认提案。本轮仅获授权修订本计划，PRD hash 保持不变；后续按授权把本期用户新确认接续进去。代码、CI、CLAUDE/README、真实验收和远端可安装性均按新计划实施，不借本次处置宣布完成。

**下一步**：回复 D9–D11 后继续 `$apply-doc-review docs/plan/DOCS-SSG-P1.review.md` 完成定案复核；若需逐项交互解除阻塞，使用 `$resolve-blocked docs/plan/DOCS-SSG-P1.md`。
