# AGENTS.md

本文件为在本仓库工作的 coding agent 提供统一指引，包含行为准则、项目架构、开发命令与验收要求。

**取舍**：这些准则偏向审慎而非速度。琐碎任务自行判断。

## 1. Think Before Coding

**不要假设。不要藏起困惑。把取舍摆出来。**

动手之前：

- 明确说出你的假设。不确定就问。
- 有多种解释时全部列出来——不要默默选一个。
- 有更简单的做法就说出来。该反驳时反驳。
- 有不清楚的地方就停下。指出困惑在哪。问。

## 2. Simplicity First

**用最少的代码解决问题。不做预留。**

- 不做没被要求的功能。
- 不为一次性代码造抽象。
- 不加没被要求的「灵活性」「可配置性」。
- 不为不可能发生的场景写错误处理。
- 写了 200 行而它可以是 50 行，就重写。

自问：「资深工程师会不会觉得这写复杂了？」会，就简化。

## 3. Surgical Changes

**只碰必须碰的。只清理自己造成的混乱。**

改既有代码时：

- 不「顺手改进」相邻代码、注释、格式。
- 不重构没坏的东西。
- 匹配既有风格，哪怕你会用别的写法。
- 发现无关的死代码就说出来——不要删。

自己的改动造成孤儿时：

- 删掉**因你的改动**而不再被使用的 import / 变量 / 函数。
- 不删既有的死代码，除非被要求。

检验标准：每一行改动都能直接追溯到用户的诉求。

## 4. Goal-Driven Execution

**定义成功标准。循环到可验证为止。**

把任务转成可验证的目标：

- 「加校验」→「为非法输入写测试，然后让它们通过」
- 「修 bug」→「写一个能复现它的测试，然后让它通过」
- 「重构 X」→「保证重构前后测试都过」

多步任务先说一句计划：

```text
1. [步骤] → 验证：[检查点]
2. [步骤] → 验证：[检查点]
```

## 5. Evidence & Quotation

**凡引用必见原文。没读到的不写。**

- 引用任何文件的措辞、编号、数值、状态、命令或日期前，先亲眼读到那段原文；不凭上下文推断补写，不把「大概如此」写成事实。
- 工具回显被截断（长文件只给首尾）时，中段是**未知**：要引用就先按行范围或搜索结果回读定位，再引。
- 正式产物（CR、计划、运维手册、commit message、给用户的事实陈述）里每一处「据 X 说 Y」，X（文件与段落）与 Y（内容）都要能当场指出。
- 发现引用与原文不符：全仓搜出同一错误的所有副本一并修正，并向负责人说明，不留两处互相矛盾的版本。

---

## 项目约定

本文件适用于支持项目指令的 coding agent。开工先读本仓 README、包管理与工具链配置、相关开发计划；已有 PRODUCT.md / DESIGN.md 时一并读取。命令、端口、服务名、数据库名和 App 身份以本仓配置为准，不套用其他项目的值。

- SDLC 台账入口：[.xgent-ai/sdlc/index.md](.xgent-ai/sdlc/index.md)，回写遵循其中的协议与状态模型；原计划和证据保留原位。

### 项目定位与实现范围

Rustify UI 是实验性 Rust SDK，按能力分工组合 Leptos CSR（DOM）与可选的 Makepad（GPU、WebGL2），在同一个浏览器页面和 wasm 实例中运行。默认构建仅含 DOM，不链接 Makepad；`gpu` feature 才启用 GPU region，现有示例均显式开启它。Leptos 负责 DOM 布局和应用状态；每个 `GpuRegion` 拥有独立的 Makepad `Cx`，绘制到 DOM 布局的 canvas。两端不共享组件和内部状态，应用通过 props 投影状态，通过 typed actions 接收反馈；Rustify 负责状态传递、生命周期、输入与浮层协作。

入口文档：[架构与运行时契约](docs/architecture.md)、[SDK 快速入门](docs/quickstart.md)。

- **项目目标**：为交互应用提供比同等功能的 React 应用更高性能的 WASM 实现，或通过 SSG 直接提供静态 HTML，同时保持较小的 WASM binary。字体、图片等 assets 不得嵌入 WASM，应与二进制分离，放在 public 静态资源目录或 CDN，通过 URL 引用。
- 每个任务先说明采用 DOM、GPU 或跨区协作及选择依据，默认只实现本次需要的呈现路径，不自动补齐另一端。
- 用户明确要求两端支持，或实际跨区协作需要时，才覆盖两端；说明各端承担的职责，复用共享逻辑。
- 共享状态、主题和动作语义不要求组件、样式与布局双端对等；不为组件目录或演示对称而新增另一套实现。
- 当前尚未选定具体业务场景。新增 GPU 能力须有本次具体需求，不为假设场景预建组件或扩大 CSS 兼容范围。

### 架构与目录

根 workspace 排除 `makepad/`、`ref/` 和 `site/`；Makepad fork 有自己的 workspace。

- `crates/rustify-dom`：共享状态、类型和中立的 `crates/rustify-dom/src/runtime.rs` 宿主绑定（`rustify_runtime_boot`、`runtime::PROTOCOL`）。DOM loader、宿主和样式位于 `crates/rustify-dom/web/`，包括 `DOM_LOADER_JS`、`DOM_RUNTIME_CSS` 和 wasm-bindgen runtime snippet。`csr`（默认）、`ssr`、`hydrate` 分别构建。
- `crates/rustify-ui`：Git 分发的 SDK 门面，重导出核心类型，并通过可选的 `gpu` feature 增加 Makepad region。
- `crates/rustify-components`：24 类 DOM 组件；包含从 Rust/UI fork 的 `clx!` / `variants!` 宏，以及生成 `docs/components.md` 的 `CATALOG`。样式集中在 `crates/rustify-components/css/`：`sdk.css` 定义组件 source、scope 内的 `dark` variant、token utilities 和默认 tokens，每个 Tailwind input 都应导入；`rustify.tailwind.css` 单独编译这一层，产物 `rustify.css` 提交入库，由 `rustify_components::CSS` 暴露。
- `crates/rustify-makepad`：私有 Makepad 集成，包含 `RegionApp`、不复用 `RegionId` 的 region registry、导出给 JS 的 pump 入口、延迟 props 和 action outbox；`crates/rustify-makepad/web/embedded.js` 是 region canvas 与 GL 的 JS 宿主。
- `makepad/`：裁剪到浏览器后端的 Makepad wasm 闭包硬 fork，属于本仓正式源码，可直接修改；`sources verify` 将改动报告为可归因的 drift。嵌入改动集中在该目录的 `platform/src/os/web/*`、`platform/src/{action,cx}.rs`、`libs/wasm_bridge` 和 `tools/cargo_makepad`；后者只保留单线程浏览器构建。
- `web/loader.js`、`web/runtime.css`：GPU 构建的页面启动入口。`boot()` 创建 wasm 实例，检查 bridge fingerprint 与 runtime protocol，把同一个 host 交给 Makepad 和 SDK runtime，并管理静态失败提示；DOM 构建使用 SDK 自带的 `dom-loader.js`。
- `tests/dom-only/`：独立 workspace，包含使用普通 Cargo 和 wasm-bindgen 构建的外部 DOM 消费者（`tests/dom-only/consumer/`）、GPU opt-in 与负向编译探针。浏览器检查使用 `playwright.docs.config.ts` 的 `--project=dom-only`，端口 4190；`DOM_ONLY_DIR` 可指定其他构建，例如 `docs-verify --external` 在仓外生成的产物。
- `crates/rustify-docs` 与 `crates/rustify-docs-cli`：文档站生成器的库和 `rustify-docs` 二进制，将 Markdown 在宿主侧通过 `ssr` 渲染为静态站点，不与 CSR workspace 联合构建。作者参考为 `docs/docs-ssg.md`。同一 docs Playwright 配置包含 `docs-root`（preview `/`，4191）、`docs-subpath`（preview `/rustify-ui/`，4192）、`docs-meta`（不发 headers 的静态宿主，4193）；`tests/docs-ssg/serve.sh` 先重建 `docs-cli` 二进制和 fixture。`docs-dev` 的每个测试在 fixture 副本上自行启动 dev server（端口 0），spec 先重建 CLI。
- `examples/*`：每个示例包含 Rust 应用 `examples/*/src/`、`index.html` 和页面脚本 `app.js`；脚本调用 `boot()`，并暴露 `window.__<example>` 供 Playwright 操作。
- `xtask build-web`：通过 `mbx` 运行 fork 的 `cargo-makepad`，用 `wasmi` 从 wasm 中提取生成的 message-bridge JS，作为静态 ES module 发布，再写入 `build-manifest.json`。运行时不生成代码，以满足 strict CSP。
- 示例有自己的 `tailwind.css` 时（如 component-catalog），`build-web` 使用 mise 固定的 CLI 编译、压缩到产物的 `tailwind.css`，由页面引用；该生成物不提交，CLI 缺失或版本错误时构建提前失败。没有自有 input 且依赖组件库的示例直接复制已提交的 `rustify.css`，无需 Tailwind。

### 运行时契约

修改运行时前先读 `docs/architecture.md` 的完整契约，尤其是：

- **Instance > scope > region**：每次 `boot()` 都有独立 wasm 实例、线性内存、host hooks 和诊断；各实例共享编译后的 `WebAssembly.Module`。trap 只终止所属实例；loader 依次中止该实例的页面监听、清除 `data-rustify-url-owner` 标记、丢弃任务、销毁 region，再显示提示。每个 slot 最多重启三次；同一实例内的 `mount()` scopes 同生共死。
- **先启动 runtime**：loader 启动实例 runtime 之前，`mount` 返回 `RuntimeNotStarted`。两个 loader 都比较 wasm 与自身的 runtime protocol，boot 再检查 hooks 的 protocol。
- **页面级监听与释放**：使用 `rustify_ui::listen`（SDK 内使用 `runtime::listener_options`），携带实例的 abort signal；`panic = "abort"` 不运行析构。非监听资源通过 `release_on_abort` 释放。
- **Pump**：JS 调用 `rustify_region_process(region, msg)`，整个 pump 期间将 region 的 `Cx` 移出 registry。先执行延迟 `apply` closures，重入的 `apply` 只入队；pump 返回后才向应用交付 actions。
- **Controlled values**：控件不持有所显示的值；输入是请求，显示结果随应用决定同步。唯一例外是 `rustify_ui::Draft` 数字、颜色字段：聚焦期间保留输入草稿，失焦后回到应用值。
- **Action streams**：`Pace::Continuous("<name>")` 只覆盖自身流的状态；离散事件不合并、不重排。
- **同步宿主决策**：如 wheel 的 `preventDefault`，依据 region 上次绘制的报告作答；按设计落后一个事件。
- **目标拆分**：大量模块和重导出受 `#[cfg(target_arch = "wasm32")]` 控制，host `--lib` 测试只覆盖目标无关逻辑，浏览器行为由 Playwright 覆盖。剪贴板需要 `--cfg=web_sys_unstable_apis`，`build-web` 会设置；未设置时 `clipboard::available()` 为 false。

### 工具链与本地开发

- 用本仓已有的启动、检查、测试与构建入口，遵循现有包管理器、锁文件和工具链配置；有统一启动入口时优先使用它。
- **缺服务、缺容器就停下来问，不要擅自装。** 未获授权时不下载镜像、不另起基础设施容器；先确认已有环境和依赖，再说明缺什么、请用户协助。
- 未经用户明确要求，不升级依赖 Major。
- **不新开分支**：改动直接提交在当前分支（通常是 `main`），除非用户明确要求开分支。
- `mise install` 按 `mise.toml` 安装 Rust 1.99.0 stable（含 `wasm32-unknown-unknown`、rustfmt、clippy）、mbx 1.15.0 和 Tailwind standalone CLI 4.1.13。当前不使用 nightly、rust-toolchain.toml 或 `build-std`。`xtask` 拒绝其他 Tailwind 版本；`RUSTIFY_TAILWIND=<path>` 可指定同版本的其他副本。
- 所有编译、测试和 Clippy 走 **mbx**：`mbx build`、`mbx test`、`mbx clippy`、`mbx xtask ...`。mise 也包装普通 `cargo`，但启动嵌套构建的代码必须显式调用 `mbx`，因为 `mbx run` 给子进程的 `$CARGO` 是普通 Cargo。格式化使用 `cargo fmt`。
- mise postinstall 运行 `scripts/link-libllvm.sh`。mbx 会清除 `DYLD_*`，缺少该链接时 rust-lld 无法加载 libLLVM。`mbx xtask doctor` 只报告工具链状态，不修改环境。
- 仅浏览器测试需要 Node 26，准备命令为 `npm ci && npx playwright install chromium`；生成 CSS 不需要 Node。已有 Chromium、无法再下载时，用 `RUSTIFY_CHROMIUM=<path>` 指定 Playwright 启动的可执行文件。

### 常用命令

按改动范围选择检查，完整入口以本仓 CI 和 xtask 为准。

```sh
# Host 检查；makepad/ 通过自己的 rustfmt.toml 排除格式化
cargo fmt --all -- --check
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins
mbx test -p rustify-ui --lib router::
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings

# Docs 使用 ssr，库与 CLI 分别测试、分别跑 Clippy，不与 CSR workspace 联合构建
mbx test --locked -p rustify-docs
mbx test --locked -p rustify-docs-cli
mbx build --locked -p rustify-docs-cli --profile docs-cli   # panic=unwind 的 Git CLI 安装 profile
(cd makepad && mbx test)
(cd makepad && mbx test --manifest-path platform/script/Cargo.toml)
mbx xtask sources verify
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check

# SDK 的 DOM 默认、GPU、SSR 模式逐个运行；DOM 核心和组件只验 DOM / SSR，无 gpu feature
mbx test --locked -p rustify-ui --lib
mbx test --locked -p rustify-ui --features gpu --lib
mbx test --locked -p rustify-ui --no-default-features --features ssr --lib

# Docs 与 DOM 专项检查
mbx xtask docs-verify --group features [--external]   # 默认依赖图/产物无 Makepad；GPU 双向切换；docs 仅宿主构建
mbx xtask docs-verify --group performance             # 参考站 B：冷渲染、热配置重渲染、页面重量、真实 build
mbx xtask docs-verify --group artifacts               # 地址、manifest、站点文件、指纹；new -> build 不编译
mbx xtask docs-verify --group reliability             # kill、满盘、权限、锁、输出事务、作者 HTML、泄漏与外链
mbx xtask docs-verify --group toolchain               # 从 Git 安装，再以仅 Rust + CLI 的作者环境离线验收
mbx xtask docs-corpus --out <dir>                     # 参考站 B；profile 固定于 tests/docs-ssg/b-profile.json
mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml   # 输出到 target/dom/dom-consumer/

# 构建、预览示例；输出到 target/makepad-wasm-app/<profile>/<example>/
mbx xtask build-web --example fusion-basic --release
mbx xtask serve --example fusion-basic --release      # strict CSP；支持 --base、--spa、--port、--csp、--fault
mbx xtask report-size --example fusion-basic --release --compressed
```

workspace 测试保留 `--bins`，因为 xtask 和示例是二进制。`docs-verify --group toolchain` 默认使用工作树提交，也支持 `--git` / `--rev`，通过 `cargo install --git … --profile docs-cli` 安装后，在 sandbox-exec 离线环境验证 `new/check/build/preview/dev`。

### 前端 UI 开发与验收（有前端时）

涉及页面、组件、布局、交互或视觉的改动，走完设计与浏览器验收两步：

- **设计阶段用 impeccable**：先读已有产品与设计上下文，覆盖视觉层级、信息架构、间距对齐、配色、动效、可访问性以及空状态和错误状态。当前环境没有该 skill 时说明缺失并提示安装，不声称已经使用。
- **验收使用当前 Agent 可用的真实浏览器通道**：Chrome extension、`kimi-webbridge` 或 Playwright 均可，按实际可用能力选择，不绑定某一种 Agent。
- **Portal App 必须从宿主进入**：从已有宿主入口打开 App，走通主路径与关键边界；首次接入时还要覆盖市场卡片、安装与授权。确认浏览器显示的是最新产物；standalone 页面或直接打开 dev server 只能作为补充。
- 检查本次改动涉及的导航、面包屑、关键操作与保存结果、空 / 错状态、权限边界；涉及响应式、主题或语言时检查相应状态。查看 console 与失败请求，保留实际步骤、结果及必要截图作为验收证据。
- **类型检查、lint、单测和 build 不能替代浏览器验收**；只看 diff 不算完成。环境跑不起来时先按现有配置排查；仍受阻则明确写「未在浏览器中验证」、说明原因与需要的协助，不声称 UI 已验收通过。

### 浏览器测试

Playwright 使用**已经存在的 release 构建**，`serve` 不重建。Rust 改动后先对受影响示例重新执行 `build-web --release`。根配置只启动 `--project` 所选项目的 server，因此预构建范围限于这些示例。子路径项目 `workbench-deep`、`deployment` 分别需要 property-workbench、fusion-basic 的 `--release --base /tools/demo/` 构建，产物名称带 `@tools-demo`；缺少产物时 `serve` 拒绝启动。

`RUSTIFY_TIER` 由 `tests/tier.ts` 定义：`regression`（默认）跳过 `@evidence`，`rounds(n)` 最多三轮；`evidence` 只跑带标记的完整强度测量；`all` 跑全部。`budget*` 项目只在非 regression 层出现。

```sh
# 迭代时优先单 spec；每次只跑一个 project
npx playwright test --project=property-workbench m3-workbench.spec.ts
npx playwright test --project=fusion-basic
RUSTIFY_TIER=evidence npx playwright test --project=budget
npx playwright test -c tests/vellum/playwright.config.ts --project=vellum m2-scene.spec.ts
VELLUM_ARTIFACT_SCOPE=m4-pointer npx playwright test -c tests/vellum/playwright.config.ts --project=vellum m4-pointer.spec.ts
VELLUM_TWIN=baseline VELLUM_BASELINE_DIR=../rustify-ui-vellum-baseline RUSTIFY_TIER=evidence npx playwright test -c tests/vellum/playwright.config.ts --project=vellum visual.spec.ts
```

- 回归 spec 从 `tests/browser/support.ts`（Vellum 为 `tests/vellum/support.ts`）导入 `test`。每个 worker 为每个 project 复用一个页面，测试前调用示例 handle 的 `reset()` 原地恢复首屏状态。软件光栅器下新 WebGL context 编译 shader 需数秒，remount 会重复付出成本。需要独立页面加载（trap、deep link、`addInitScript`、context 选项）的测试声明 `test.use({ fresh: true })`，使用 Playwright 常规的新 context。
- Vellum 的 twin 在端口 4180：默认使用 `ref/` 中的原版；`VELLUM_TWIN=baseline` 时使用 `VELLUM_BASELINE_DIR` 指向 worktree 内的 release 构建。用 `git worktree add` 检出改动前提交，并在该 worktree 中 `build-web`；两种参考均不存在时，twin 比较跳过。
- `playwright.config.ts` 映射 spec 与 project，各示例、budget、deployment 各有项目，spec 只在列出它的项目下运行。
- **必须带 `--project`，每条命令只跑一个项目，不同时启动两个 Playwright 运行**。全量运行会耗尽内存；并行运行还会互相清除 `test-results/`，导致虚假的 ENOENT。
- 完整项目可能耗时数十分钟，已有记录中 fusion-basic 约 47 分钟、property-workbench 约 38 分钟；日常迭代用单 spec，全量扫查用于里程碑退出取证。
- 帧预算需要真实 GPU 的 headed Chrome，CI 有意不跑，headless 时测试自行跳过。`budget-scene` 本身就是 headed project；R30 AC1（`budget`）、B2 scroll（`budget-data`）和 Vellum pan 手工加 `--headed`，例如 `RUSTIFY_TIER=evidence npx playwright test --project=budget --headed -g "R30"`。
- 性能数字在页面内部测量，不使用 Playwright 的操作计时。
- `mbx xtask verify --suite p1|p2|p3` 执行对应发行的完整检查；VoiceOver、真实 IME 输入、缩放走查等人工记录缺失时必须列为 missing，不能记为通过。

### 编码与样式约定

- 组件 class 使用不带前缀的 Tailwind v4 utilities，与应用写法一致；调用方通过 `tw_merge` 用同类 `class` 覆盖组件 class。修改 `crates/rustify-components` 中的 class 字符串后，运行 `mbx xtask css` 并提交 `crates/rustify-components/css/rustify.css`，否则新 class 没有规则且 CI `--check` 失败。
- 已提交的样式表只包含 SDK class。示例自己的 class 需要自有 `tailwind.css`，见 `docs/quickstart.md` 的「Using Tailwind v4 in your app」；Tailwind input 的 utilities import 始终保留 `source(none)`。修改 catalogue 后重新生成 `docs/components.md`。
- `cargo fmt` 会重排换行，对 Rust 源码做脚本查找替换时必须断言旧文本已命中。
- `ref/` 被 gitignore 排除，只作本地阅读材料，包含上游 Leptos、Makepad、Rust/UI 和 Vellum；也作为 Vellum 视觉与交互比较的 twin 参考服务（端口 4180）。

### 性能：热路径上不许 N+1，读多写少的事实必须缓存

「表小 / 量不大 / 上限只有 1 000」都不是不优化的理由——tsc、lint、verify 对 N+1 全绿，只有压测和生产 p95 看得出来。请求路径与 worker 热路径一律适用，算法上能提高的就要提高，不能偷懒：

- **循环里逐条 `await` 查询 = N+1，禁止**。同一批 id 一条 SQL `inArray` 取回，再在内存里映射；Redis 同理用 `mget`。
- **一批写入用多行 INSERT + `RETURNING`**，按业务键（不是数组下标）把返回行映射回输入；`onConflictDoNothing` 只返回真插入的行，冲突那部分要再查一次。超过 1 000 行分块（PG 单语句 65 535 个参数）。
- **读多写少的配置/字典必须有进程内短 TTL 缓存**。管理员改完要即时生效的（事件定义、策略、模板、lane 开关、渠道连接）加显式 `invalidateXxxCache()` 在写侧调用；纯展示快照（App 名/图标/颜色）30 s TTL 就是契约，不要求写侧失效——写点散在十几处且 worker 是别的进程，失效调用本来就到不了。同一次请求 / 同一批处理里同一事实只查一次（先整批取到手再进循环）。
- **不许缓存的**：发送/扣费/授权前的「当下资格」重查（成员是否 active、App 是否仍安装、端点归属）——它们存在的意义就是新鲜；一条带索引的点查即可。
- **热路径上的聚合（`count(*)` / `min()`）不许每次现算**：`/health` 与背压这类每秒会被打的入口共用一份短 TTL 缓存。
- **事务里不做外部调用，持连接只覆盖写入**；渲染、解密、供应商调用都在事务外。
- **改了热路径要给前后数字**：使用本仓已有参数化压测，把吞吐量或 p95 写进提交说明；没有压测的 SQL 路径至少 `EXPLAIN` 一次。

### 计划与证据

- 开发计划放在 `docs/plan/*.md`，使用中文；里程碑证据放在 `docs/validation/`，发行报告放在 `docs/reports/`。
- 计划的「实施进度 → 恢复快照」是跨对话唯一续做入口。里程碑退出检查通过后立即重写，再报告完成或提交；SDLC 台账按本文件开头链接的协议回写，原计划和证据保留原位。
- 源码不携带需求、计划或里程碑 ID；追溯关系写在计划和 commit message 中。

### 完成标准与发布

- 执行与改动相关的本仓检查、测试和构建入口，报告实际执行结果与未验证项。
- crates.io 九包发行走 `docs/crates-io-release.md` 与 `scripts/crates-io.py`。每个包的归档绑定自己的 `source_rev`：只有该包的目录/manifest 或它解析到的锁闭包变了才重打并升版，其余用 `package-libraries --reuse <上一发行目录>` 复用原归档、原版本与原校验和；只改 CLI 时不要顺手升库包版本。动手前先 `plan-libraries --reuse … --rev <完整 SHA>` 看谁要重打。框架三包（docs/dom/components）的版本必须同步移动，因为 CLI 的配对记录只带一个 framework revision；库包版本默认跟随 workspace，要让某个包真正单独发版，先给它自己的 `version`（其余继续继承）。
