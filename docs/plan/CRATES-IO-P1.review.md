# CRATES-IO-P1 · 开发计划完整评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [CRATES-IO-P1](CRATES-IO-P1.md) | SHA-256：34a5a89f62ba049c8630691ab69e350654030b06b2ed3b54eeed09b699873964（未跟踪新文件） | 全文；R-1–R-6、NFR-1–NFR-3、C-1–C-2、A-1；ADR-1–ADR-3、D1–D7；M1–M6、V-1–V-16 |

评审 ID：c87b4302-8506-4530-b668-bc9b0d2130fd。调查日期：2026-10-22（Asia/Shanghai）。这是首次完整评审，不是修复效果复审。原计划与实现代码均保持只读。

## 1. 确认问题

### [P1] F-01 · 根 `[patch.crates-io]` 让归档里的 Cargo.lock 随打包时的索引漂移，R/C 校验和链无法复现

- **定位**：§1.2 L200（“根 patch 继续服务当前 GPU 分支；registry 包不能依赖它们”）、ADR-3 L118–125、D6 L92、§1.1 L159、§5.3 表 L319–326 及步骤 3–6 L334–337、§5.4 L342–344、§8 L376；关联 R-6、NFR-3、C-1、V-7、V-13、V-14、M2/M4/M5/M6。
- **问题**：
  - 根 patch 把 bitflags 2.10.0 和 smallvec 1.15.1 指向 `makepad/libs` 的路径副本。它们经 lightningcss、pulldown-cmark、notify、cssparser、url→idna 进入 style（dev 依赖）、dom（现 rustify-ui）、components、docs、cli 的依赖图，所以 L200 的前提不成立。
  - Cargo 1.99 给每个 `.crate`（包括库）写入新生成的 Cargo.lock，同时移除 `[patch]`。路径源条目没有 registry 版本可沿用，会按打包当时可见的索引取最新兼容版；其余条目沿用根锁。因此归档字节由三样东西决定：R、根锁、打包时的索引快照。
  - M5 的 receipt、`framework-release.json` 和模板锁记下的是 M5 时的 checksum。M6 `cargo publish` 会重新打包；只要两次之间 smallvec 1.x 或 bitflags 2.x 发了新版，字节就会变。下游库的归档锁又内含上游库的 checksum，漂移会一路传到 CLI 内嵌的模板锁。
  - 计划的 dry-run 比对能拦住这种漂移，但 §5.3 第 6 步只处理“库源码改动”，对“R 未变、归档已变”没有分支。CLI 单独打包时 local-registry 里放哪些版本，会直接决定 CLI 的字节，计划也没有定义。另外，发布出去的锁会使用工作区从未测试过的 bitflags/smallvec 版本。
- **证据**：
  - [Cargo.toml](../../Cargo.toml) L39–41；[Cargo.lock](../../Cargo.lock) L177–178、L2896–2897：这两条既没有 source，也没有 checksum。
  - `cargo tree --offline -p rustify-docs-cli -i bitflags@2.10.0` 显示经 lightningcss、notify、pulldown-cmark 进入；`-i smallvec` 显示经 cssparser、idna、icu_normalizer 进入。
  - 主 agent、facts、architecture 各自独立用 cargo 1.99.0 做了实验，结果一致：
    - 工作区 patch 为 smallvec 1.15.1 时，归档锁变成 crates.io 的 1.16.3 并带 checksum；bitflags 同理变为 2.13.2。
    - 未被 patch 的 itoa 保持根锁版本。
    - 库的 `.crate` 同样含 Cargo.lock；`cargo publish --help` 没有排除锁文件的选项。
    - 同一 commit 换 clone、换 mtime 打包，字节完全一致。所以漂移只来自索引，不是打包本身不可复现。
  - Cargo Book cargo-package 页（2026-10-22 读取）：`Cargo.lock` 总会被打包，`[patch]` 会被移除。
  - 对比本机 registry 缓存：`makepad/libs/bitflags/src` 与 crates.io 2.10.0 相同；smallvec 只有 rustfmt 差异。
- **最小修订**：
  - **首选：从根上消除**。在 M2 让 `makepad/platform`、`platform/script`、`platform/studio` 改用 registry 精确版本 `bitflags = "=2.10.0"`、`smallvec = "=1.15.1"`，删掉根 patch 的这两项，使根锁为它们记录 crates.io source 和 checksum；在 sources.lock.json 登记这处 drift；再用一次 GPU 构建确认依赖图里仍只有一份副本。
  - **若保留 patch**，需要补三处：
    - ADR-3 和 §1.1 写明归档字节的三个输入；
    - receipt 登记索引快照（根锁的 registry 条目 ∪ 全部归档锁），freeze-cli、package-cli、verify 只能使用这份快照；
    - 补上 M6 dry-run 不一致时的分支：R 不重冻，用实际归档重新执行 freeze-cli 得到 C′，再重跑 V-7–V-9 中受影响的部分。
  - **两种方式都要做**：`inspect` 断言发布闭包内没有 path 源或被 patch 的包；receipt 记录每个归档锁的 digest；改正 L200。
- **复核**：
  - 改稿后：§1.2、ADR-3、§5.3、§5.4、§8 对 patch 的处理和漂移分支一致。
  - 实现后：`inspect` 在发布闭包里找不到无 source 的条目。采用首选方案时，同一 R 在两份不同索引快照下打出的 sha 相同；采用备选方案时，脚本测试能检出差异并走漂移分支。
- **置信度**：机制为高，三次独立实验复现。M5→M6 窗口内真实发生的概率为中：smallvec 在 2026-09-11 至 10-10 间连续发了 1.16.1–1.16.3。**阻塞**：M2（manifest 与锁）、M4（脚本设计）、M5、M6。

### [P2] F-02 · 受影响包的快速检查都排在 R 冻结之后，任何静态或 corpus 问题都会触发整条归档链重冻

- **定位**：§10 M1–M4 退出条件 L449–452、提交点 L456；V-1–V-6 L391–396、V-11 L401、V-12 L402；§9.2 L412；§5.3 第 1、6 步 L332、L337；关联 R-3、NFR-1、NFR-2。
- **问题**：
  - M1–M4 的退出条件只跑定向测试。受影响包的以下检查，都要到 M5 才第一次运行，而 R 在 M4 已经提交：
    - fmt、clippy `-D warnings`；
    - `xtask css --check`、`sources verify`；
    - `rustify_repo_tests` 哨兵；
    - 按 §2.3 docs.rs 选项跑的 `cargo doc`；
    - 工作树打包。
  - 这些检查引出的修复大多落在八个库包内：clippy、CSS 产物、corpus cfg、docs.rs metadata、include 清单。按 ADR-3 都要重冻 R。`.cargo_vcs_info.json` 里的 sha1 一变，八个归档的 checksum 全变，模板锁、C、CLI 归档跟着换，V-7–V-10（含串行的浏览器项目）和 V-12 整组失效。§9.2 说“只重跑相应包/契约与受影响消费者”，低估了这个范围。
  - 这里不是要求中间里程碑跑全量：上面每一项都可以限定到包，是快速检查。
- **证据**：[verify.yml](../../.github/workflows/verify.yml) L40–56、L109–117 已有分模式 clippy、`sources verify` 和 `css --check`。`css.rs` 在 M1 就会断掉，见 F-03。
- **最小修订**：
  - M1–M4 的退出条件分别加入本里程碑触及包的快速检查：
    - fmt 和按模式的 clippy；
    - 从 M1 起跑 `css --check`；
    - M2、M3 跑 `sources verify`；
    - M2 跑 `rustify_repo_tests` 哨兵，以及按选定 features 的 `cargo doc`。
  - M5 写死执行顺序：先在 R 上跑 V-11 的库部分，通过后才 package-libraries；CLI 部分在 C 上、package-cli 之前跑。
  - §9.2 写明失效范围：重冻 R 时 V-7–V-10、V-12 整组失效；只改 CLI 时，V-7 的 CLI 部分和 V-8–V-10、V-12 失效。
- **复核**：改稿后，M1–M4 的检查都限定到包且排在 R 之前，M5 的顺序明确。实现后，M4 记录里有这些检查的通过证据。
- **置信度**：高，由计划自身的 ADR-3、§5.3 和现有 CI 直接推出。**阻塞**：M1 开工前修订，因为 M1 一开始就会引入 css 断点。

### [P2] F-03 · §1.2 改动面漏了被移动或删除文件的读取者，其中 `css.rs` 的产物属于库 R

- **定位**：§1.2 L182–198；§2.2 L218 只泛写了“更新所有直接受移动影响的资源读取者”；§2.3 L226 和 §1.2 L198 说 sources.lock.json 只登记宏包，与 §3.3(3) L271 冲突；关联 V-1、V-3、V-11。
- **问题**：
  - [xtask/src/css.rs](../../xtask/src/css.rs) L26–31 的 `SOURCE_TREES` 写死了 `crates/rustify-ui/src` 和 `crates/rustify-ui/web`，L46–48 遇到不存在的目录直接报错。M1 把 DOM 源码和 `web/` 资源移走后，`xtask css` 要么失败要么漏类，而它生成的 `rustify.css`、`candidates.json` 属于 components 包内容，也就是落在 R 里。
  - 以下位置写死了 rustify-ui 前缀或框架库清单，都不在改动表里：
    - [islands.spec.ts](../../tests/docs-ssg/islands.spec.ts) L622 匹配 `snippets/rustify-ui-…`；
    - [dev-components.spec.ts](../../tests/docs-ssg/dev-components.spec.ts) L54–57、L585；
    - [xtask/src/docs_corpus.rs](../../xtask/src/docs_corpus.rs) L1011、L1223、L1476，[xtask/src/docs.rs](../../xtask/src/docs.rs) L500、L517；
    - CLI 的 [src/compile_proxy.rs](../../crates/rustify-docs-cli/src/compile_proxy.rs) L78、[tests/components.rs](../../crates/rustify-docs-cli/tests/components.rs) L84、[tests/rust_dev.rs](../../crates/rustify-docs-cli/tests/rust_dev.rs) L27。
  - [sources.lock.json](../../sources.lock.json) 从 L1837 开始的 vendor 条目 `docs-static-fonts` 登记了 CLI `assets/fonts` 下的 15 个文件，[xtask/src/sources.rs](../../xtask/src/sources.rs) L164–168 遇到缺失文件会报错。M3 删掉字体后 `sources verify` 必然失败，与“仅登记宏包”的说法矛盾。
  - `custom.spec.ts` 读取被删字体的问题见 F-09。
- **最小修订**：
  - §1.2 补上述文件并写明归属：`css.rs`、`docs.rs`、`docs_corpus.rs`、`compile_proxy.rs` 和 CLI 测试归 M1；两个 spec 归 M1/M3；sources.lock.json 的 `docs-static-fonts` 条目随 §3.3(3) 在 M3 移除。
  - §2.3 和 §1.2 的“仅登记宏包”改为“宏包 manifest 的变化，以及删除 CLI 字体对应的 vendor 条目”。
  - 把 `css --check` 放进 V-1，与 F-02 一致。
- **复核**：改稿后，grep `rustify-ui` 字面量和 `crates/rustify-ui/` 路径，命中都在改动表里，或在明确不变的清单里。实现后，M1 退出前 `css --check` 无漂移，M3 退出前 `sources verify` 通过。
- **置信度**：高。**阻塞**：M1、M3。

### [P2] F-04 · corpus 门漏了 components 和 style-wasm，style 的 feature 也比现有 CI 窄

- **定位**：§1.1 L158 只写了 style；§2.3 L225、§9.3 L433–436；与 V-11 L401 以及 §2.3“不能只加 cfg 后丢失原覆盖”冲突；关联 NFR-1、NFR-2、V-3。
- **问题**：
  - 另有两个包的测试引用仓根文件：
    - components 的 cfg(test) 模块 [class_migration.rs](../../crates/rustify-components/src/class_migration.rs) L8–14 用 `include_str!` 读仓根的 `tests/utility/migration/*.json`；`catalog.rs` L728–729 在运行时读 `docs/components.md`。
    - style-wasm 的 [session.rs](../../crates/rustify-style-wasm/src/session.rs) L594 和 `public_api/design_system.rs` L758–759 用 include 读仓根的 `tests/utility/compiler/*`。
  - 按 §2.3 这些测试要改由 `rustify_repo_tests` 控制，但 §9.3 只为 style、dom、docs、cli 设了带 cfg 的命令，这几组测试此后没有任何门会运行；如果不加门控，包外的 `cargo test --lib` 又会编译失败。
  - 现行 CI 用的是 `mbx test -p rustify-style --all-features --lib`（[verify.yml](../../.github/workflows/verify.yml) L40）。计划的 corpus 命令只开 `cn,scanner,compiler`，`scanner-fs` 等分支不再被覆盖。
- **最小修订**：
  - §9.3 增加 components（`--lib`）和 style-wasm（`--no-default-features --features compiler,scanner --lib`）的 cfg 命令。
  - style 那一行改回 `--all-features`。
  - §1.1 L158 列全所有引用了仓根文件的包。
  - 每个包都放一个哨兵，证明 cfg 确实生效。
- **复核**：改稿后，`crates/*/src` 中每处 `../../` 或 `CARGO_MANIFEST_DIR` 越界引用，都能逐包对应到 §9.3 的一条命令。实现后，各包的哨兵通过。
- **置信度**：高。**阻塞**：M2（V-3）、M5（V-11）。

### [P2] F-05 · §2.2 说 GPU 用户与组件共享同一个核心，混源时不成立

- **定位**：§2.2 L216；关联 R-3、NFR-2、§0.5 L139、§2.4 L231、§5.2 L308。
- **问题**：
  - Git 使用时，Git 版 rustify-ui 里的 path+version 依赖解析为 Git 源的 rustify-dom；registry 组件依赖的是 crates.io 的 rustify-dom。Cargo 允许同名、同版本、不同来源的两个包并存。
  - GPU 应用作者照 L216 写 `rustify-ui = { git, features = ["gpu"] }` 加 `rustify-components = "0.2.0"`，就会得到两份核心：
    - 类型不匹配；
    - `external-host` 只作用于 Git 那份；
    - 两份 runtime 导出（`rustify_runtime_boot` 等）预计在 wasm 链接时冲突。
  - 在 Docs 的 components 项目里，§5.2 会直接拒绝第二个 dom，计划前后矛盾。
- **证据**：
  - architecture 的混源实验：`cargo tree` 同时出现 registry 和 git 两份同名同版本核心，编译报 `E0308 expected zz_core::Theme, found Theme`；加上 `[patch.crates-io] zz-core = { git = … }` 后只剩一份。
  - Cargo 文档 Specifying Dependencies → Multiple locations。
  - 现行 [quickstart.md](../quickstart.md) L238–239 是两个包都用同一个 Git rev。
  - [runtime.rs](../../crates/rustify-ui/src/runtime.rs) L116–117。
- **最小修订**：§2.2 和迁移说明写清支持的组合：
  - GPU 应用的 rustify-ui、components（以及 docs）全部来自同一个 Git rev；
  - 或者用 `[patch.crates-io] rustify-dom = { git, rev }`，并要求版本一致；
  - 声明本期 Docs components 项目不支持 GPU。

  另在 `tests/dom-only` 或 compat 下加一对编译探针：混源编译失败，加 patch 后通过。
- **复核**：改稿后 §2.2 与 §5.2 不再矛盾；实现后探针结果写进 M1/M2 记录。
- **置信度**：高。类型不匹配已实测，wasm 链接冲突未实测。**阻塞**：M1（§2.2 的文字）、M4（迁移文档）；不阻塞 Docs 首发。

### [P2] F-06 · 自供字体只能经完整的 theme.document 选用，仓外作者拿不到这份文档

- **定位**：§3.1 L252；R-5 L74；ADR-2 L107–116；V-16 L406。
- **问题**：
  - `[[theme.fonts]]` 只负责注册，选用字体必须经 theme.document 里的 `font-sans` / `font-serif` / `font-mono`。这份文档每种模式要写全 49 个 token，两种模式都要。
  - 作者文档说它来自 Theme Studio 导出，而 Theme Studio 只在仓内作为 GPU 示例构建，没有公开部署；`docs/` 下也只有 schema，没有可以直接复制的完整示例。
  - 默认阅读主题（不带 document，或只用 `theme.primary`）没有任何选字体的入口，而 config 会拒绝未知键。
  - 结果：不会 Rust、只装了 CLI 的作者，走不通“放自己的字体”这条主路径，V-16 大概率会卡在这里。
- **证据**：[themes.md](../themes.md) L36、L62–64；[docs-ssg.md](../docs-ssg.md) L431–438；[config.rs](../../crates/rustify-docs/src/config.rs) L23、L892–896；[docs-pages.yml](../../.github/workflows/docs-pages.yml) L6–12 只部署文档站；verify.yml L254–286 只在 CI 里构建 theme-studio。
- **最小修订**：二选一，写进 §3.1、§1.2、V-4/V-16。
  - (a) 没有 document 时，允许用 `[theme] font-sans/serif/mono` 选用已注册的字体。这会扩大配置面，需用户确认。
  - (b) 保持配置面不变：发布一份可复制的完整默认 theme.json，或让 `new` 写出一份；作者指南给出“复制后只改三行字体栈”的步骤，V-16 明确走这条路。

  推荐 (b)，它不改变产品的配置面。
- **复核**：改稿后 §3.1 和 V-16 写明作者从哪里拿到选用配置。实现后，角色模拟只凭公开文档，从空目录完成换字体。
- **置信度**：中高。现状已核实，卡点的实际程度待 V-16 实测。**阻塞**：M3 设计、V-16。

### [P2] F-07 · 现有 0.1.32 用户的迁移既没有具体配方，也没有验收

- **定位**：ADR-2 L115（“提供迁移步骤”）、§2.2 L217–218、§3.2 L264、§5.4 L345、V-15 L405；关联 R-6。
- **问题**：三类现有用户会卡在不同地方。
  1. **Git 0.1.32 生成的组件项目**：依赖要从三条 Git 依赖改成 registry 的 rustify-docs、rustify-dom、rustify-components，依赖名和 feature 转发（如 `rustify-ui/ssr`）都要跟着变，锁文件也要重新生成。现行 [docs-ssg.md](../docs-ssg.md) L961–964 的升级说明只讲“更新三个依赖并重新生成”，针对的还是 Git revision。
  2. **导入主题的内容站**：升级后，每个没有声明的非通用字体族都会出 `theme-font-system` 警告，用 `--deny-warnings` 的 CI 会直接失败。要复现原来的效果，作者得自己找字体文件：Rustify WenKai 是本仓修复后改名的字体（[themes.md](../themes.md) L142），上游拿不到；Liberation Mono 是 GPL，带源码义务。
  3. **GPU 应用**：见 F-05。

  V-15 只覆盖 CLI 自身的升级和回滚，没有任何检查是从旧项目出发的。
- **证据**：[rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) L28（FRAMEWORKS）、L717–783；[Cargo.toml.in](../../crates/rustify-docs-cli/templates/components/Cargo.toml.in)。
- **最小修订**：在迁移文档里定下三份配方（§1.2 已列 CHANGELOG.md 和 docs-ssg.md）。
  - 组件项目：推荐用 `new --template components` 新建一个项目，再搬入原 manifest 和 lock 中的作者内容；或用 `package` 别名保留原依赖名。
  - 导入主题的站点：列出原来内置字体的来源和许可、`--deny-warnings` 的处理办法，以及 CJK/emoji 回退的变化（见 F-14）。
  - GPU：见 F-05。

  V-9 或 V-15 增加一项：用 0.1.32 生成的组件项目和一个导入主题的站点，按迁移文档升级成功。
- **复核**：改稿后配方的落点和对应检查都存在；实现后，迁移 fixture 的升级记录可追溯。
- **置信度**：中高。**阻塞**：M4（文档）、M5/M6（验收）。

### [P2] F-08 · 描述内置字体和 revision 配对的作者文档不在改动清单里，模板 README 会被冻结进 C

- **定位**：§1.2 L192（`templates/` 一行只写了版本元组）、L197（没有列 `docs/themes.md`）；§5.1 L303 只要求改安装命令；V-6 L396。
- **问题**：
  - 组件模板 [README.md](../../crates/rustify-docs-cli/templates/components/README.md) L226 写着字体来自 CLI 内置的字节；L246、L256、L264 写着三个依赖钉在 framework revision 上，升级要“update all three framework revisions”。
  - [themes.md](../themes.md) L75 写着 “The CLI contains the same seven font files … no font download step is required”，官方站会收录这一页。
  - [docs-ssg.md](../docs-ssg.md) L448 和 L1865 的诊断表仍在描述内置字体。
  - 后果：0.2.0 生成的新项目自带错误说明，而模板 README 冻结在 C 里，上传后无法再改。
- **最小修订**：§1.2 补上 `docs/themes.md`、docs-ssg.md 的字体和诊断段，以及两个模板 README 中“Theme, fonts”和“Keep the CLI and framework paired”两节。V-6 增加断言：新生成的 README 和 themes.md 里不再出现内置字体和 framework revision 的说法。
- **复核**：改稿后改动清单完整；实现后 grep 断言通过。
- **置信度**：高。**阻塞**：M3、M4。

### [P2] F-09 · 生产预览下的字体浏览器验收没有落到具体 fixture 和 spec，而现有 spec 依赖将被删除的字体

- **定位**：§3.3 L271；§2.4 L233（`dev-fonts.spec.ts` 只纳入 docs-dev）；§6 L354；V-10 L400。
- **问题**：
  - [custom.spec.ts](../../tests/docs-ssg/custom.spec.ts) 在 components-root、subpath、meta、webkit 这几个项目下，直接读 `crates/rustify-docs-cli/assets/fonts/*.ttf` 比对字节，并断言 `.ttf` 和 `font/ttf`（L35–36、L46、L74、L361–385）。
  - 组件 fixture 的 theme.json 用 Noto 和 JetBrains 字体，没有声明 `[[theme.fonts]]`。
  - M3 删除字体后，这个 spec 要么直接坏掉，要么被删而没有替代。§6 要求的两件事在生产预览下都没有检查点：root、子路径、CSP 下声明的字体真的加载成功；不声明字体时零字体请求。`dev-fonts.spec.ts` 只覆盖 dev。
- **最小修订**：§1.2 和 V-10 写明以下内容，并在 root、subpath、meta 三个项目运行。
  - 组件 fixture 改为声明测试用的本地字体。
  - `custom.spec.ts` 改为比对作者自己的文件，并检查 `assets/fonts/<digest>.<ext>` 加 base 的路径、对应的 MIME 和 `document.fonts.check`。
  - 另设一个“导入主题但不声明字体”的场景，断言零字体请求、零 404。
- **复核**：改稿后 V-10 能对应到具体的 spec 和 fixture；实现后三个项目都有结果。
- **置信度**：高。**阻塞**：M3、M5。

### [P2] F-10 · V-16 作者角色模拟排在不可逆的上传之后，包内文档的卡点在 0.2.0 里改不了

- **定位**：V-16 L406（M6 收口，“卡点必须修正文档”）；V-8 L398；§5.1 L303；§5.3 第 6 步 L337；M6 L454；关联 R-4、R-5。
- **问题**：
  - 角色要读的主要文档都在已上传的归档里：crate README（也就是 crates.io 页面）、CLI 内嵌的模板 README、`--help` 文本和诊断措辞。
  - V-16 在 V-14 之后才跑，查出来的卡点只能靠发 0.2.1 修；如果改的是库的 README，还要重冻 R、重传库并更新配对。计划没有这条回路，M6 要么卡住，要么被“只改仓库文档”顶替。
  - 官方站本期不部署（§0.5 L142），V-16 里“公开文档”包括哪些也没有定义。
- **最小修订**：
  - 把作者角色模拟移到 M5，放在 V-8 之后，对 staging 安装的 CLI 执行。替换源的配置属于测试环境，对角色不可见。
  - 文档集合写明为：归档内的 README、模板 README、`--help`，加上 C 树里的 `docs/quickstart.md` 和 `docs/docs-ssg.md`。
  - 包内文档有卡点时，按 §5.3 第 6 步重冻。
  - M6 只保留公开安装复验（V-15），外加“按包 README 完成安装”这一步。
- **复核**：改稿后角色模拟的执行时点早于任何上传；实现后 M5 记录里有卡点，以及由卡点触发的重冻（如有）。
- **置信度**：高。**阻塞**：M5、M6 的编排。

### [P2] F-11 · 推送、发布授权和 V-12 的 CI 触发方式都没有登记

- **定位**：V-12 L402；§1.2 L196；§4 L284、L286；§5.4 L345；M5 L453；M6 L454（“执行发布的任务授权”）；§11 L460–467；恢复快照 L55；关联 C-2、NFR-1、A-1。
- **问题**：
  - **推送缺失**：V-12 的 Linux 门只能在 GitHub Actions 上跑，因为本机是 macOS，仓库规则又禁止擅自起容器。`workflow_dispatch` 要求 workflow 文件位于默认分支，也就是要先把含 crates-io.yml、R、C 的提交推到 main。计划全文没有出现“推送”，授权、负责人、时点和触发方式都没写。
  - **触发方式**：如果 workflow 跟着 push 触发，R 与 C 之间、以及上传前推送的 main 都会让它失败。
  - **公开文档时序**：M4 把公开安装文档放进了 R。一旦为了 V-12 推送 main，公开 README 会在 0.2.0 上传之前就指向一个不存在的版本；如果 A-1 卡住 M6，这个错误会长期公开。
  - **上传后的推送**：M6 的标签也要推送。模板 README 写死了链接 `blob/main/docs/docs-ssg.md`，上传后如果不推送，公开安装 0.2.0 的作者读到的是 0.1.32 的 Git 配方。
  - **授权登记**：“执行发布的任务授权”只出现在 M6 的前置里，没有进 §11，也没有进恢复快照。
- **证据**：
  - 计划全文没有“push”或“推送”。
  - [verify.yml](../../.github/workflows/verify.yml) L3–6 在 push main 时触发；[docs-pages.yml](../../.github/workflows/docs-pages.yml) L6–12 用 `workflow_dispatch` 加 `candidate_sha` 输入，可作先例。
  - 模板 README：[content](../../crates/rustify-docs-cli/templates/content/README.md) L4，components L4、L212。
  - 先例：[DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md) L80 把“公开推送和 Pages 发布须有覆盖本次范围的用户授权”登记为约束。
- **最小修订**：
  - §4/§11 增加一行“推送与发布授权”：责任人是用户，覆盖 M5 推送 main（R、C、crates-io.yml）、M6 上传、上传后推送标签和公开说明。最晚确认点是 M5 的 V-12 之前，同时写进恢复快照。
  - crates-io.yml 改为 `workflow_dispatch`，以完整的 R/C SHA 作输入。
  - 仓库层的公开安装文档改为上传成功后的“公开说明提交”，提交后再推送。范围包括 README.md、quickstart、docs-ssg 的安装和回滚段、CHANGELOG。包内 README 和模板 README 仍然随 R/C。
  - V-15 增加一项：公开链接打开的是 0.2.0 的内容。
- **复核**：改稿后 §11 有授权行，V-12 有触发命令；实现后 M5 记录含 run ID，M6 记录含推送的 SHA 和标签。
- **置信度**：高。“dispatch 需要 workflow 在默认分支”依据 GitHub 文档，没有实测。**阻塞**：M4 写 CI 和文档之前修订；M5 跑 V-12 之前必须取得授权。

### [P2] F-12 · 发行脚本的归属和测试错位，控制上传的逻辑要到发布当天才首测

- **定位**：M2 L450、M4 L452、V-2 L392、V-14 L404、§1.2 L193、§5.3 L319–326；关联 R-2、R-6、NFR-3。
- **问题**：
  - V-2 在 M2 执行 `python3.12 scripts/crates-io.py inspect`，但脚本要到 M4 才交付。
  - package、freeze、verify、publish 四个子命令到 V-7（M5）才第一次运行。
  - 续接逻辑要到 V-14 上传当天才第一次测试，包括：checksum 相同则记为已上传，不同则停止，超时先查实际状态。V-14 里写的“故障分支用脚本测试验证”也没有对应命令。
- **最小修订**：
  - M2 交付 `release/crates-io.toml` 和 `inspect`，M4 交付其余子命令。
  - 定义一项局部检查，例如 ★`python3.12 -m unittest discover -s tests/crates-io`：M2 覆盖 inspect；M4 用本地索引或假 registry 覆盖续接、checksum 不一致、超时和限流（见 F-13）这几个分支，不上传任何东西。
  - V-14 改为引用这项检查已通过的证据，自己只负责真实上传。
- **复核**：改稿后 V-2 引用的命令由 M2 交付，V-14 里没有未定义的命令；实现后 M4 记录含脚本测试结果。
- **置信度**：高。**阻塞**：M2 开工前修订。

### [P2] F-13 · 发布设计没有处理 crates.io 的新包限流，也没核对首发 token 的作用域

- **定位**：§5.4 L342–344；§8 L376；V-13 L403；V-14 L404；§4 L280；关联 A-1、NFR-3。
- **问题**：
  - 九个都是新包。crates.io 默认对每个用户的新建 crate 允许突发 5 个，之后每 10 分钟 1 个。docs 组连续上传 8 个新包，从第 6 个起预计会收到 429：“You have published too many new crates in a short period of time”。
  - 计划只处理了“等待索引超时”。publish 子命令遇到 429 可能直接停下，也可能误判后重试。docs 组至少要约 30 分钟，style-wasm 还要再等一个窗口，这也拉长了 F-01 的漂移窗口。
  - 首发需要一个带 `publish-new` 作用域、名称模式覆盖九个包名的 token，V-13 没有列这一项。
- **证据**：facts、architecture、delivery 三方独立在 2026-10-22 读取 crates.io main 的 `src/rate_limiter.rs` L24–38（PublishNew 为 600 秒、burst 5，可由环境变量覆盖）和 `src/controllers/krate/publish.rs` L219、L250–256。
- **最小修订**：
  - §5.4 和 publish 子命令把 429 定义为“未上传，按返回的时间等待后续接”，不算失败，也不重新打包；这一状态记入 receipt。
  - §8 登记这一行，并预估总耗时。
  - V-13 增加 token 作用域和名称模式的核对。
  - runbook 写明可以提前向 crates.io 申请临时提额。
- **复核**：改稿后 §5.4、§8、V-13、V-14 都有 429 和作用域的语义；实现后用脚本测试模拟 429，并能成功续接。
- **置信度**：默认值为高；线上实际配额为中。**阻塞**：M4（脚本设计）、M6。

### [P3] F-14 · Css 模式下，隐式回退字体和不带通用族的字体栈没有定义行为

- **定位**：§3.2 L262、L264；ADR-2 L114。
- **问题**：
  - 现行 `dom_font_stack` 会给非通用字体族自动追加 "Rustify WenKai" 和 "Noto Color Emoji"（[fonts.rs](../../crates/rustify-ui/src/theme/fonts.rs) L146–158）。Css 模式“保留作者完整的字体栈”后，这层回退会静默消失。作者的栈里没有这两个名字，`theme-font-system` 也不会提示，中文页面会从楷体变成系统字体。
  - 作者手填的栈如果不带通用族，解析器照样接受：0.1.32 会映射到该槽位的回退字体，0.2.0 则交给浏览器默认字体（通常是衬线体），等宽槽可能变成比例字体。
- **最小修订**：Css 模式下，栈末没有通用族时按槽位补上 `sans-serif` / `serif` / `monospace`，并加单测；迁移说明和 `theme-font-system` 的提示里写明 CJK/emoji 的内置回退已经移除。
- **置信度**：中。**阻塞**：否。

### [P3] F-15 · M2/M3 的并行表述与 M3 的前置互相抵消，共享文件也没写归属

- **定位**：M2 L450、M3 L451；§2.3 L222；§3.3 L271。
- **问题**：
  - M2 说可以和字体工作并行，M3 的前置却要求“组件 CSS 具名导出经 V-3 验证”。V-3 几乎覆盖 M2 的全部成果，所以实际只能串行。
  - 真正冲突的地方只有两处：CLI 的 `src/theme.rs`（M2 改 L17 的跨包 include，M3 删 L26–83 的字体内嵌），以及 CLI 和 docs 的 manifest。
- **最小修订**：
  - 把 M3 的前置拆细：核心和 Docs 的字体部分只需 V-1；CLI theme.rs 部分只需 `styles` 导出已合入，并通过 `mbx test -p rustify-components --lib`。
  - manifest 和 Cargo.lock 由 M2 负责人单写；theme.rs 先由 M2 改，再交给 M3。
  - 或者干脆删掉并行的表述。
- **置信度**：高。**阻塞**：否。

### [P3] F-16 · style-wasm 上传前的“公开安装通过”门没有定义，和 V-15 交织在一起

- **定位**：V-14 L404、V-15 L405、§5.4 L343、D1 L87、§0.5 L138。
- **问题**：style-wasm 要等“CLI 和 Docs 公开安装通过”才上传，但这道门检查什么没写。V-15 包含九包查询，只能在 style-wasm 上传之后完成，实施者可能把它拆成两次跑。
- **最小修订**：在 V-14 里定义这道门的最小集合：用独立的 `CARGO_HOME` 和 `--root` 做公开安装、检查 `--build-info` 的来源、跑通 `new` content 构建，并写明是否包含 components。V-15 只在 remaining 组上传之后跑一次。
- **置信度**：中。**阻塞**：否。

### [P3] F-17 · V-10 的运行环境和用例集合没写全

- **定位**：V-10 L400、§5.3 L334–335、§12。
- **问题**：
  - (a) 上传之前，components-subpath 和 docs-dev 要编译依赖 registry 包的消费者，必须用与 V-8 相同的临时 source replacement；而 serve.sh 起的进程拿不到这份配置。
  - (b) [serve.sh](../../tests/docs-ssg/serve.sh) L76–77 用 `python3` 运行 `docs-pages-prepare.py`，后者 `import tomllib`。本机 PATH 上的 `python3` 是 3.9.6，§12 没有收录这个坑。
  - (c) “融合页相关 lifecycle/theme 用例”没有列出具体 spec；fusion-basic 整个项目约 47 分钟。
- **最小修订**：V-10 写明 source replacement 的配置、PATH 中 python3 ≥ 3.11，以及从 `tests/browser` 和 `tests/dom-only` 选出的具体 spec 清单；§12 补上 Python 这个坑。
- **置信度**：中。(a) 由未来设计推导，(b)(c) 已核实。**阻塞**：否，M5 之前修即可。

### [P3] F-18 · V-12 的 Linux 门与 docs.rs 实际的构建环境不同

- **定位**：§2.3 L227、§4 L284、V-12 L402。
- **问题**：
  - docs.rs 用 nightly、没有网络；没设 `targets` 时会构建多个默认目标。计划的 V-12 是 stable、单目标。
  - dom 和 components 用 ssr 渲染文档，会隐藏 wasm32 专属的 API。这是计划已经接受的取舍，但 README 没有说明。
- **最小修订**：
  - 各库在 `package.metadata.docs.rs` 里显式设置 `targets`；style-wasm 设 `default-target = "wasm32-unknown-unknown"`。
  - README 的支持说明写明文档的呈现模式。
  - 可选：在 workflow 里加一次 nightly 的 `cargo doc`。
- **证据**：docs.rs/about/metadata、docs.rs/about/builds（2026-10-22 读取）。
- **置信度**：中。**阻塞**：否。

### [P3] F-19 · 发布 runbook 的必备内容和 yank 的授权人没写

- **定位**：§1.2 L197（`docs/crates-io-release.md` ★ 只出现在文件清单里）；§4 L280、L286；§8 L377。
- **问题**：
  - runbook 缺这些条目：token 的最小权限和过期时间、`cargo login` 与环境变量二选一怎么选、发布后怎么撤销 token、429 等待（F-13）、推送（F-11）。
  - yank 属于 registry 写操作，“已确认有问题”由谁确认没有写。
- **最小修订**：列出 runbook 的必备条目，规定 M6 按 runbook 执行并在记录里回链；yank 只在发布人当次明确确认后执行。
- **置信度**：中。**阻塞**：否。

### [P3] F-20 · 要发布的库对通用基础 crate 仍是精确版本钉

- **定位**：C-1 L79；§2.3 L223 只规定了本仓包之间的精确版本；关联 R-2。
- **问题**：
  - rustify-ui（将来的 rustify-dom）钉了 `serde =1.0.229` 和 `serde_json =1.0.151`；rustify-docs-syntax 钉了 `syn =2.0.119`、`proc-macro2 =1.0.107`、`quote =1.0.47` 和 `serde =1.0.229`；rustify-docs-macros 也钉了 syn、proc-macro2、quote、serde_json。
  - 发布之后，作者只要在组件项目里加入一个要求更高 serde/syn 补丁版本的 crate，Cargo 就无法解算，而且这个限制会随时间扩大。
  - 计划既没决定保留还是放宽，也没把这个限制写进 README。
- **证据**：[rustify-ui/Cargo.toml](../../crates/rustify-ui/Cargo.toml) L22–23；[rustify-docs-syntax/Cargo.toml](../../crates/rustify-docs-syntax/Cargo.toml) L10–13；[rustify-docs-macros/Cargo.toml](../../crates/rustify-docs-macros/Cargo.toml) L14–17。
- **最小修订**：在 §2.3 作出决定：发布的 manifest 里，serde、serde_json、syn、quote、proc-macro2 改用带下限的 caret 要求（工作区锁仍保持精确版本）；只对确属契约的 leptos、wasm-bindgen/js-sys/web-sys 以及 leptos 内部 crate 保留 `=`。如果必须保留精确钉，写明理由并在 README 说明这个限制。
- **置信度**：中。机制是确定的，实际会不会冲突取决于作者添加什么依赖。**阻塞**：否。

## 2. 待核实项与待决策

| 项目 | 缺什么证据 / 待谁决定 | 影响 | 最晚确认点 |
| --- | --- | --- | --- |
| F-01 选首选还是备选方案 | 计划作者决定。首选方案需要确认：去掉 patch 后，makepad 的 GPU wasm 构建里 bitflags/smallvec 仍只有一份，且 `sources verify` 正确登记 drift | M2 的 manifest/锁，M4 的脚本 | M2 开工前 |
| F-06 选 (a) 还是 (b) | 选 (a) 会扩大作者配置面，需用户确认；选 (b) 计划作者可直接定 | M3 设计、V-16 | M3 开工前 |
| 真实 `cargo publish --registry crates-io` 与快照替换的组合 | 只在采用 F-01 备选方案时需要：dry-run 已证实打包会走替换源，但还要确认首个包上传后，从 static.crates.io 下载的 sha 等于 receipt | F-01 备选方案的可行性 | M6 首次上传前（V-13） |
| wasm-bindgen snippet 目录前缀 | 按现有规律推断会改为 `rustify-dom-<hash>`，需要 M1 的 wasm 构建实测 | F-03 中依赖前缀的 spec 和工具 | M1 |
| 混源双核心在 wasm 链接时的表现 | 只验证到类型不匹配，没做链接实验 | F-05 迁移说明的措辞 | M2/M4 |
| C 提交后，仓内源码 CLI 和集成测试走哪种配对模式 | 现有测试会把生成的 Git manifest 投影回仓内 path（`tests/components.rs` 中的 `pair_component_project` 与 `docs-pages-prepare.py`）；§5.2 L313 说 helper 要识别新的 framework JSON，但 registry 模式下的投影规则没写明 | V-11 在 C 上能否不依赖还没上传的包 | M4 |
| 0.2.0 CLI 遇到 0.1.32 Git 配对项目时的行为 | §5.2 L309 的“保留现有 Git 配对分支用于旧记录”有歧义：是继续接受，还是给出迁移诊断 | F-07 中旧项目看到的报错 | M4 |
| 包名可用性和 owner | 2026-10-22 九个名称在 crates.io 查询均为 404，这只是一个时点的观察，不预留名称；`rustify-style-wasm` 在 index 也是 404 | V-13 | M6 上传前（计划已安排） |
| crates.io 线上实际限流参数、docs.rs 排队时长 | 只看到源码默认值和公开文档 | F-13 的等待设计，V-15 的结论时点 | M6 |

## 3. 覆盖、基线与实际执行

### 基线与任务台账

- **代码基线**：main / 2c0897c67347f23cca48e85e0bc2665f0f2d2f86；当前工作树就是调查的事实源。
- **dirty 情况**：开始和结束时都只有 `.xgent-ai/sdlc` 台账改动，以及未跟踪的计划和它的台账对象。计划写的“clean（调查开始时）”是它自己的历史自述。
- **结束前复核**：计划 SHA-256 未变，HEAD 未变。
- **执行方式**：按四个视角实际启动了四个独立 reviewer，并行运行，模型继承。reviewer 全部只读；证据回读、裁定、报告和台账都由主 agent 单写。

| 维度 / 负责人 | 核查范围与完成情况 | 证据缺口 |
| --- | --- | --- |
| facts / 独立 reviewer | §1.1、§1.2 的事实与拓扑，§2.1 所需符号，§5 的现有行为，回滚提交，mbx 子命令，仓根引用；做了 4 个打包小实验。已交回并复核 | 计划自报的“多包 tmp-registry”和“source replacement 保留 canonical source”两项实验没有重跑（architecture 已覆盖后者） |
| architecture / 独立 reviewer | ADR-1/2/3、feature 与类型身份、打包是否可复现、patch、source replacement、混源、限流、docs.rs、安装前置；做了 2 组玩具实验。已交回并复核 | wasm 链接冲突、线上限流配置 |
| schedule / 独立 reviewer | 需求账本 → 设计 → 交付 → 里程碑 → V 的双向追踪，依赖边，并行波次，提交点，进度协议原文。已交回并复核 | GitHub dispatch 规则没有实测 |
| delivery / 独立 reviewer | 站点作者、发布人、现有用户三类旅程；作者文档、模板、spec、已构建产物。已交回并复核 | 新功能未实现，UI 只能做文档推演 |
| 主 agent | 全文阅读；独立实验复现 patch 漂移与打包可复现性；逐条回读候选证据；去重裁定（合并 9 组重复候选）；跑脚本；写报告和台账 | 未运行构建、测试、浏览器或任何发布动作 |

### 已执行的检查

| 检查 | 实际结果与解释 |
| --- | --- |
| `check_paths.sh docs/plan/CRATES-IO-P1.md <仓根>` | 43 个路径 token，12 个 MISSING。逐项分类后均为计划标 ★ 的新增项、协议兼容路径（`docs/project-status/protocol.md`）或 Cargo 生成路径（`target/package/tmp-registry`）。没有据此报格式缺陷 |
| `check_progress.sh` | 0/6，里程碑表 6 行，完成记录 0 行；ERROR 0、WARN 1。警告只来自未提交的台账和计划本身，不表示有未回写的进展 |
| `check_plan.py` | ERROR 0、WARN 0；只证明结构和引用正确，不证明设计正确 |
| 实施协议原文 | schedule 视角逐字 diff 了骨架的「实施者定位」「实施进度」，内容一致 |
| 主 agent 的打包实验（scratchpad，cargo 1.99.0） | 带根 patch 的工作区：打包后锁里的 smallvec 从 1.15.1 变为 1.16.3。同一 commit 换 clone、换 mtime，`.crate` 的 sha 一致。库包也含 Cargo.lock |
| 外部只读查询 | Cargo Book、docs.rs 文档、crates.io 源码与 API（GET）、GitHub 上的 cargo 源码。没有登录、上传或任何写操作 |

外发说明：architecture reviewer 回报，它查询 crates.io 只读 API 时，有一次误把用户邮箱放进了 HTTP 请求的 User-Agent 头。该请求是只读 GET，之后的请求都改用通用 UA。除此之外，没有其他个人信息外发。

### UI 证据等级

本期不设计新页面，但字体输出和核心拆分会影响既有 DOM 页面：docs 站、islands、DOM-only 和融合页。

本轮 UI 结论全部是**文档推演**，没有启动浏览器或构建。唯一的产物级证据，是只读查看已构建的组件 fixture CSS（`target/docs-ssg/features/components/assets/site.86352d006e.css`）。它确认现行字体栈会自动追加 WenKai 和 Emoji，是 F-14 的依据；但这只是静态产物检查，不证明任何渲染或交互结果。

V-10、V-16 等未来验收须按修订后的计划执行；类型检查和静态源码都不能代替它们。

## 4. 需求映射与实施编排

### 映射结论

- **账本覆盖**：§0.1 账本里，R-1–R-6、NFR-1–NFR-3、C-1–C-2、A-1 都有设计落点和检查；V-1–V-16 都有设计依据，没有孤立检查；中间里程碑没有混入全量检查或端到端检查。缺口不在于需求没登记，而在于以下几类：
  - 设计前提错误：F-01、F-05；
  - 改动面和检查不全：F-02–F-04、F-08、F-09、F-12；
  - 用户主路径与迁移：F-06、F-07；
  - 不可逆发布的编排：F-10、F-11、F-13。
- **弱链**（建议修订时一并处理）：
  - R-4 的“new 不因生成锁文件而联网”：V-8 的离线断言只覆盖 content/landing，建议 V-9 加一条在离线条件下执行 `new --template components`。
  - `docs-verify --group toolchain` 和 `features --external` 在新配对下是保留，还是由 V-8 取代，计划没写。
  - V-1 的“源码 DOM 图核对”、V-3 的“★包边界测试”、V-6 的“旧配对 helper 的 Python 测试”都没有可执行命令。
- **进度协议**：0/6、尚未开始、下一步 M1 与里程碑表一致。提交点的自洽前提是里程碑记录和 receipt 都放在包目录之外（例如 `CRATES-IO-P1.records/`）；建议在 §5.3 写明这一点，否则记录写进 `crates/` 会改变 C。

| 使用者路径 | 闭环情况与必要修订 |
| --- | --- |
| 新作者：安装 → new（content/landing）→ check/build/preview | 安装只需 Rust 和 C 编译器；内容站不需要 wasm target（`build.rs` 不编译 wasm）。闭环，但公开文档入口依赖 F-11 |
| 新作者：components 模板 → SSR/hydrate | wasm32 target 有预检，wasm-bindgen 在进程内。依赖 F-01 的锁可复现，F-20 影响作者后续加依赖 |
| 作者：自供字体 → 预览 → 部署（子路径/CSP） | CSP 已有 `font-src 'self'`，预览服务已映射 woff/woff2/otf 的 MIME。缺选用入口（F-06）、生产预览验收（F-09）和回退定义（F-14） |
| 现有 0.1.32 / GPU 用户：升级或回滚 | 回滚到 88566ba1 已核实。缺迁移配方和验收（F-07），混源说法需改正（F-05） |
| 发布人：凭证 → 预检 → 冻结 → 上传 → 续接 → 核验 | 冻结逻辑成立（打包本身可复现）；需补索引漂移（F-01）、限流与 token（F-13）、推送与授权（F-11）、脚本早测（F-12）和 runbook（F-19） |

### 可保留的架构与代价

以下选择经核实成立，修订时保留即可：

- ADR-1 B（门面重导出单一核心）：GPU 代码只用 DOM 的公开项；核心里的 gpu cfg 都在 §2.1 覆盖之内。
- 用 `package` 别名保留局部导入名。
- 减法式 feature `external-host`：语义与现有 `not(gpu)` 相同。
- Pace 下沉到核心：消除现有的两份定义。
- `FontResolution::{Packaged, Css}` 的最小接缝。
- registry 安装用标准 release（panic=unwind）。
- 用 local-registry 和 source replacement 做无服务的上传前验证：已实测会保留 canonical source。

ADR-3 分步冻结的方向正确；它缺的不是冻结顺序，而是 F-01 指出的第三个输入（索引）。可选的非阻塞改进：CLI 自带一份 `[profile.release]`，可以恢复 thin LTO。

### 依赖边、并行波次与汇合门

依赖图没有环，ADR-3 已经消除了 checksum 循环。建议保留六个里程碑，只细化前置和检查时点：

| 波次 / 任务 | 前置及解锁证据 | 可同时做的工作 | 写入边界 / 共享负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| 先修订 | F-01–F-13 落进正文；F-06 方案已定 | 只读核对可以并行 | 主 agent 单写计划 | — | 同一路线、同一组检查，没有留给实施者的重新选择 |
| W0：M1 核心拆分 | 当前源码 | `release/crates-io.toml`、`inspect` 及其 Python 测试（F-12） | M1 负责人单写根 Cargo.toml/Cargo.lock、rustify-dom、rustify-ui、pace.rs、各包的别名依赖、compat fixture、`css.rs` 和受影响的工具与 spec；脚本负责人只写 `release/`、`scripts/crates-io.py`、`tests/crates-io` | mbx 共享 target，并发构建只会排队 | V-1，加 `css --check` 和触及包的 clippy（F-02） |
| W1：M2 ∥ M3 的核心与 Docs 部分 | V-1 | M2 负责 manifest、README/LICENSE、宏包改名、corpus cfg、`styles` 导出，以及 F-01 的 patch 处置。M3 负责 rustify-dom 的 FontResolution 和 rustify-docs 的 config/content/site | manifest、Cargo.lock、sources.lock.json、inventory 只由 M2 负责人写；core theme 模块归 M3 | 同上 | V-2、V-3（含 corpus 哨兵）；V-4 的核心部分 |
| W1b：M3 的 CLI 部分 ∥ M4 的配对主体 | `styles` 导出已合入；M4 还需要 V-2、V-3 | M3 负责 theme.rs、rendering.rs、删除字体及 sources.lock 条目、改造 spec 与 fixture。M4 负责 build.rs、release.rs、validate_framework、new.rs、模板 | CLI 文件按上述拆分，缓存键最后合入 | Python ≥ 3.11 | V-4 全部；V-5 主体 |
| W2：M4 其余部分 | V-4 的字段部分 | CLI 配对（A）∥ helper、serve.sh、xtask、workflow 和脚本其余子命令（B） | 仓库层公开安装文档留到发布后提交（F-11） | — | V-5、V-6、脚本测试 |
| 门 R | M1–M4 的局部检查和快速门全部通过 | — | 主 agent 单独提交 | — | M4 记录写入 R |
| W3：M5（严格串行） | R | 只有远程 V-12 能与本地 V-10 并行 | 单一集成人；发行目录放在 `target/` 之外（mbx 可能 GC 它） | Playwright 一次一个项目；source replacement；python3.12；fusion-basic 的 release 构建 | V-11 库部分 → 库归档 → freeze → C → V-11 CLI 部分 → package-cli → V-7–V-10、作者角色模拟 → V-12（需推送授权） |
| W4：M6（串行） | M5、A-1、用户授权 | 等 docs.rs 期间可以写发行说明 | 凭证和生产 registry 只由一人操作 | 新包限流（F-13） | V-13 → docs 组 → 公开安装门 → remaining 组 → V-15 → 标签和公开说明推送 → 最终提交 |

M5 和 M6 没有安全的并行空间：从 R 到归档、锁、C、安装、浏览器，每一步都消费上一步的产物；浏览器项目必须串行；上传要按拓扑逐包确认索引可见。计划没有工期估算，这里不推算关键路径时长。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

建议状态：**Blocked**。共确认 **1 个 P1、12 个 P2、7 个 P3**。原计划自报 Ready，与本建议不一致。

- 根 patch 会让归档字节随打包时的 registry 索引变化，这直接动摇 R/C 校验和配对这一外部契约（F-01）。
- 混源时核心实例的说法不成立（F-05）。
- 自供字体在仓外没有可走通的选用入口（F-06）。
- 不可逆上传之前缺少关键的授权、早测和文档验证（F-10–F-13）。

当前不宜照做。Blocked 只是本轮的就绪建议，不表示实施失败，也不自动改写原计划或它的实施进度。

修订顺序：

1. **先纠正前提与契约**：F-01（定首选或备选方案，改正 L200，补 inspect 断言与 receipt 字段）、F-05（GPU 组合的说法）。
2. **定设计**：F-06（选 (a) 或 (b)，(a) 需用户确认）、F-14（Css 回退）、F-20（是否放宽精确版本钉）。
3. **补改动面与早期检查**：F-03、F-04、F-02，让 R 冻结前的快速门完整。
4. **修交付与验收**：F-07、F-08、F-09，以及 F-12 的脚本归属和测试。
5. **重排不可逆发布的编排**：F-10（角色模拟移到 M5）、F-11（推送与授权写进 §4/§11 和恢复快照）、F-13（限流与 token）、F-16、F-17、F-18、F-19。
6. 顺手处理 F-15 的并行表述，以及 §4 映射结论里列出的弱链。

**解除条件**：以上 P1 和 P2 都在正文、里程碑和检查中得到一致、可执行的处置；F-06 的方案已经确定（若选 (a)，有用户确认）；修订后复核受影响的链路，确认不再有关键前置留给实施者决定。其余已核实的架构选择可以保留，不需要重写整份计划。

计划评审就绪不代表功能已实现或已经发布。

下一步：

    /apply-doc-review docs/plan/CRATES-IO-P1.review.md

## 6. 修订处置与复核

- 来源：本报告首次完整评审 `c87b4302-8506-4530-b668-bc9b0d2130fd`；追加前 SHA-256 `eeb48257b4256358e89fc6496454469f2c89fd131cb10cc3f9dcd2d8f072f33c`。以上原问题、所审基线、原结论和建议保持原文；本节是同轮处置，不是独立复审。
- 目标：[CRATES-IO-P1](CRATES-IO-P1.md)。修前 SHA-256 `34a5a89f62ba049c8630691ab69e350654030b06b2ed3b54eeed09b699873964`，与所审版本一致；修后 SHA-256 `5248dcee8e5e8443c5640afcbcd6e3b16e76cc9160c94b6b3d34434c70f43fb0`。
- 复核日期与代码基线：2026-10-22（Asia/Shanghai），main / `2c0897c67347f23cca48e85e0bc2665f0f2d2f86`；开始时计划、报告及 SDLC 台账已 dirty/未跟踪。相关代码只读，未回退原有改动；本次仅修目标计划、追加本节并同步本轮台账。
- 范围：F-01–F-20、§2 待核实/待决策表、§4 的弱链与编排建议。首次 apply，之前没有已关闭处置；下表“已解决”仅指计划缺陷已消除，不表示实现、浏览器或发行验收通过。

### 6.1 逐项处置

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 本轮复核结果 / 后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。根 manifest/锁、六个 Makepad manifest 和只读 cargo tree 证实路径源同时进入 GPU 与普通依赖；Cargo 打包会移除 patch。首选建议只列三个 manifest 不够，rustybuzz、pulldown-cmark、unicode-bidi 也有直接 path 边。原证据“没有排除锁文件选项”不准确，本机 cargo 1.99.0 的 package --help 明确有 --exclude-lockfile；该点不采信，见 §6.2 | 已修订并复核 | ADR-3、§1.1/1.2、§2.3、§5.3/5.4、§8、V-3/V-6/V-7/V-14、M2/M4/M5 | 已解决：选择消除覆盖，登记全部消费边/独立锁、归档锁 digest、异常字节即停和重冻规则；GPU 编译、两种索引下真实归档一致性是未来 V-3/V-7，不冒称本次已跑 |
| F-02 | 成立。原 M1–M4 退出条件缺快速静态门；现有 verify.yml 已有分模式 clippy/CSS/来源门，R 改变又会改变归档身份 | 已修订并复核 | §5.3、§9.2/9.4、V-1–V-6/V-11、M1–M5 | 已解决：限定包快速门在 R 前，V-11 库/非 CLI 在 R 打包前、CLI 在 C 打包前；R/C 改动的失效组明确 |
| F-03 | 成立。亲读 css.rs、docs.rs、docs_corpus.rs、compile_proxy.rs、两个 CLI 测试及 islands/dev-components spec 的路径/包名读取；sources verify 会读取 vendor.files，字体条目不能保留 | 已修订并复核 | §1.1/1.2、§2.2/2.3、§3.3、V-1、§9.4、M1/M3 | 已解决：路径读取者及单测归属齐全，snippet 前缀由 M1 实产物确认，css --check 与字体删除后的 sources verify 都有退出门 |
| F-04 | 成立。class_migration/catalog 和 style-wasm session/design_system 确有仓根引用；CI 的 style 命令为 --all-features | 已修订并复核 | §1.1、§2.3、V-3/V-11、§9.3/9.4 | 已解决：补 components/style-wasm cfg 门，style 恢复 all-features，逐包哨兵拒绝 0 tests；默认包测试与完整 corpus 分开 |
| F-05 | 成立。manifest 的 Git/path 来源与 registry 来源不同；Cargo 的多来源规则和现行 quickstart 同 rev 配方支持这一判断。未将报告推断的 wasm 符号冲突当作已实测事实 | 已修订并复核 | §2.2、§5.2、V-3/V-5/V-9、M2/M4 | 已解决：GPU 同 Git rev 或显式统一核心 patch；Docs components 首发坚持纯 registry，不支持 GPU；真实类型探针纳入实施门 |
| F-06 | 成立。config 拒绝未知键，fonts 注册不等于选择；themes/docs-ssg 要求完整 document，new.rs 当前没有可复制入口 | 已修订并复核 | §1.2、§3.1、§5.1、V-4/V-6/V-16、M3/M4 | 已解决：选建议 (b)，两类 new 提供未启用的完整系统字体 theme.example.json，作者复制并修改两模式字体槽，不新增配置面 |
| F-07 | 成立。旧模板依赖 Git 三元组且 feature 转发引用旧包名，旧主题从内嵌字节迁移后还受 warning/deny-warnings 影响 | 已修订并复核 | §2.2、§3.2、§5.2、§8、V-9/V-15、M4/M5 | 已解决：三类迁移配方、作者文件搬迁而非覆盖新锁、字体来源/历史许可/CJK 回退、旧版本回滚均有落点；上传前验证旧组件和导入主题站点 |
| F-08 | 成立。亲读组件模板 README 的 Theme/fonts 与配对段、themes 的静态站说明和 docs-ssg 导入字体段，均描述旧内嵌/Revision 契约 | 已修订并复核 | §1.2、§3.3、§5.1、V-6、M3/M4/M6 | 已解决：包内/模板文案在 R/C 前改，新发行章节有断言；0.1.32 历史说明与候选说明分开，公开默认安装入口在上传后切换 |
| F-09 | 成立。custom.spec.ts 读取 CLI assets/fonts 且限定 ttf；playwright 配置将其分配给 components root/subpath/meta，现 fixture 未声明作者字体 | 已修订并复核 | §1.2、§2.4、§3.3、V-10、§9.5、M3/M5 | 已解决：有许可的作者字体、woff2/ttf MIME/digest/base/加载断言、三入口有 document 无 fonts 变体及 dev 恢复均有明确准备方式和命令 |
| F-10 | 成立。README、模板及帮助属于待上传包，上传后不能原版本改字节；原 V-16 确在 M6 | 已修订并复核 | §5.3、§6、V-16、§9.2、M5/M6 | 已解决：角色模拟移至 M5/V-8 之后、所有上传之前；资料集明确，包内卡点触发重冻；M6 只做公开安装复验 |
| F-11 | 成立。原文未登记推送和授权；verify.yml 随 main push 触发，docs-pages.yml 已有显式 SHA dispatch 先例，[GitHub 规则](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#workflow_dispatch)要求 workflow 在默认分支 | 已修订并复核 | 恢复快照、§1.2、§4、§5.1/5.4、V-12/V-13/V-15、§10/§11 | 已解决：main/R/C/workflow 推送、完整 SHA dispatch、上传后公开说明与标签推送均有责任/时点；未来操作尚未授权，未据本次 apply 执行 |
| F-12 | 成立。原 V-2 在 M2 调用 M4 才交付的脚本，V-14 故障测试也无命令 | 已修订并复核 | §1.2、§5.3、V-2/V-3/V-6/V-14、M2/M4 | 已解决：M2 先交付 inspect/边界测试，M4 补余下命令及 unittest；超时/同异 checksum/429/漂移在本地测，真实上传门只消费已有证据 |
| F-13 | 成立。本次读取 crates.io 的 rate_limiter.rs 与 publish.rs，确认源码默认 burst/补充间隔、PublishNew/PublishUpdate 权限及按 crate 名检查；不推断线上实际配额 | 已修订并复核 | §4、§5.3/5.4、§8、V-6/V-13/V-14、runbook 约束 | 已解决：429 的等待时间与续接状态落 receipt，九包默认额外等待估算注明来源/限制；token 范围、名称模式及有效期进入发布前门 |
| F-14 | 成立。fonts.rs::dom_font_stack 当前自动追加 WenKai/Emoji，Css 新策略需明确移除后的行为 | 已修订并复核 | §2.2、§3.2、V-4/V-9/V-10 | 已解决：末尾无通用族时按槽补通用回退，保留具名字体语义；迁移/warning 提示旧 CJK/emoji 字节不再隐式供应，Packaged 保持原义 |
| F-15 | 成立。原 M3 等 V-3 整体验证，与 M2 所说并行抵消，theme.rs/manifest/锁存在共享写入 | 已修订并复核 | M2/M3、§1.2、§9.4 | 已解决：核心/Docs 字体仅需 V-1，CLI 部分等 styles 导出和定向测试；theme.rs 先 M2 后 M3，manifest/锁单写，sources.lock 顺序合入 |
| F-16 | 成立。原 V-14 的公开门未定义而 V-15 已含九包查询，存在前后依赖混淆 | 已修订并复核 | §5.4、V-14/V-15、M6 | 已解决：docs 组后用独立 CARGO_HOME/--root 核身份，content 与 components 通过才发 remaining；V-15 在九包及公开说明齐全后一次终验 |
| F-17 | 成立。serve.sh 的 python3/helper 调用、helper 的 tomllib/file_digest、浏览器项目继承方式均已回读；本机 python3 --version 为 3.9.6 | 已修订并复核 | §5.3、V-10、§9.5、§12 | 已解决：source replacement 放 CARGO_HOME 并继承给子进程；临时 python3 入口用已有 3.12；DOM/hydrate、融合生命周期/动作与属性工作台主题各有具体 spec/命令 |
| F-18 | 成立。[docs.rs metadata](https://docs.rs/about/metadata)及[builds](https://docs.rs/about/builds)明确默认目标、nightly 与无网络沙箱；stable 单目标门不能等同其环境 | 已修订并复核 | §2.3、V-3/V-12/V-15、README 交付要求 | 已解决：显式 default-target/targets，host SSR 文档说明 wasm API 可见性；保留 stable 可移植性门，实际 docs.rs 成功另验，不新增本地 nightly 安装前置 |
| F-19 | 成立。原 runbook 只有文件清单，无必备条目及 yank 当次确认人 | 已修订并复核 | §5.4、§8、V-13–V-15、M4/M6 | 已解决：token 获取/保存/撤销、login 与环境变量、限流/续接、推送/dispatch、公开门/标签和 yank 授权均列清；M6 回链实际 runbook 版本 |
| F-20 | 成立。亲读 dom 现身 ui、syntax、macros 和 CLI manifest，通用 serde/syn 等存在精确钉；[Cargo caret 规则](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#caret-requirements)支持限定同一兼容范围 | 已修订并复核 | C-1、§2.3、V-3、M2 | 已解决：五个通用依赖用当前版本作 caret 下限，根锁不顺便更新；配对/框架/绑定依赖保留精确约束，范围断言有检查落点 |

### 6.2 决策、证据校正与原待核实项

本次没有引入需用户重新拍板的产品范围：F-01 采用报告允许计划作者选择的首选路线，F-06 采用不增加配置面的 (b)，F-20 采用通用库 caret/配对依赖精确约束。没有选择保留根 patch 的索引快照备选，也不新增 theme 字体选择键。

F-01 的建议有两处证据/改动面需要校正：

- 本机实际 `cargo --version` 是 1.99.0，`cargo package --help` 列有 `--exclude-lockfile`，[当前 Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-package.html)也有该选项。不能沿用“没有选项”的断言。本路线仍保留归档锁（尤其 CLI 的 --locked 安装依赖它），采用消除路径覆盖和比较归档/锁的方案，不以排除锁隐藏漂移。
- 对六个 Makepad manifest 的全文搜索及 GPU wasm 目标只读 tree，确认仅改报告列出的三个平台 manifest 会漏 rustybuzz/pulldown-cmark 等直接 path 来源，故全部列入同根因修订。缓存源码比对中 bitflags/src 40 个文件字节一致；smallvec/src 有 2 个不同文件，回读差异包含格式和 `core::usize::MAX`/`core::isize::MAX` 改为 primitive 常量路径，不能简单称“只有 rustfmt 差异”。本次未执行替换后的 GPU 编译，该验证仍为 V-3 的必要门。

| 原待核实项 / 弱链 | 本次最终处理 | 剩余动作与最晚节点 |
| --- | --- | --- |
| F-01 首选/备选与真实 publish 配快照 | 首选已定，M6 明确不携 staging source replacement；备选特有未知不再适用 | V-3 检查精确 registry 单份身份/GPU 编译，V-7 验证同 R 两种索引视图，V-14 核真实上传 checksum |
| F-06 (a)/(b) | (b) 的完整模板来源/默认值/复制步骤已定 | V-4 真实解析；M5 V-16 角色仅凭发行资料完成字体接入 |
| snippet 目录前缀 | 不先断言 rustify-dom-<hash> 已存在 | M1 的 build-dom 生成实际 wasm-bindgen 产物，再同步相关前缀断言 |
| 混源 wasm 链接结果 | 类型/源身份已足以拒绝错误组合；正文不声称链接冲突已实测 | V-3 类型失败/统一后通过探针；无需以链接失败作为支持边界的前提 |
| C 上源码 CLI / 集成测试的配对 | source CLI 严格当前 checkout path 锁；registry CLI 严格 registry 模板锁；无 JSON 的旧 CLI 才走旧 R/C | M4 V-5/V-6 测三分支，C 上 V-11 CLI 在 package-cli 之前执行 |
| 0.2.0 CLI 遇 0.1.32 Git 项目 | 明确拒绝并提示迁移或恢复旧 CLI；历史 helper 能力不等于新 CLI 接受旧元组 | V-5/V-9 核诊断与实际旧项目迁移 |
| 名称/owner、线上限流及 docs.rs 队列 | 不使用本次只读调查预留包名，也不以默认限流/静态文档推断真实环境成功 | V-13 重查身份/范围/名称；V-14 按响应等待；V-15 排队/失败保留未通过项 |
| components 离线 new、toolchain/features external 的去向 | V-9 显式检查离线 components new；旧 Git 命令保留历史能力，本次 registry 断言由 V-7–V-10 承接 | M4 V-6 测模式选择，M5 实验验证；不重复跑历史 Git 全量冒充 registry 验收 |
| 未定义的局部命令与记录落点 | DOM tree/build、包边界/inspect/helper unittest 均有命令；receipt/记录在包目录与 target 外 | 局部门分别在 M1/M2/M4，持久发行目录在 M5 准备 |

外部操作依赖尚未执行：M5 的 main 推送/无凭证 dispatch，M6 的真实上传、公开说明与标签推送需在产物可审阅后核对用户授权；crates.io 账号、邮箱、owner/token 由发布人配置，yank 另需当次确认。这些是已登记的后续阶段门，不是本轮待决设计，也不阻塞 M1 开工。

### 6.3 修订后结论与实际验证

**文档判定：Ready，可从 M1 开工。** F-01–F-20 共 **20 项已修订并复核**，文档层面复核结果均为已解决；已解决无需改 0、不采纳整项 0、待决/待修订 0。部分证据或建议未照抄的理由已在 F-01/F-06/F-18/F-20 处明确。原 §5 的“需要修改 / 建议 Blocked”是所审版本的历史结论，保留不改；本结论不代表重新完成独立计划评审。

| 本次实际检查 | 结果与边界 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/CRATES-IO-P1.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 64 个路径 token、15 个 MISSING；逐一核对均为已标 ★ 的新增项（含 framework-release.json 的 crate 相对简写）及协议兼容路径 docs/project-status/protocol.md。未发现错误引用现有复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/CRATES-IO-P1.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 0/6、里程碑 6 行、完成记录 0 行；ERROR 0、WARN 1。脚本对未跟踪计划的 ls-files 判定为空，因而提示“计划未改”；与修前快照比较已确认本次改稿，未发现未回写实施 |
| `python3.12 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/CRATES-IO-P1.md` | ERROR 0、WARN 0；检查结构、需求/检查/里程碑 ID 与引用，不代替设计或实现验收 |
| 协议/历史及修改范围核对 | 实施者定位、实施进度协议和空完成记录与修前逐字一致；稳定 M1–M6、V-1–V-16 与 0/6 未重排/虚增完成；原报告正文保留，代码未改 |
| 只读代码/工具核实 | GPU wasm 目标的 bitflags/smallvec cargo tree、Cargo/Python 版本及 package help、缓存源码差异、引用文件与浏览器配置均已亲读。只证明当前前提，不证明未来方案构建通过 |
| 格式/链接/差异 | git diff --check 通过；目标路径身份、关联段落、旧口径和 Markdown 本地链接已核对；新增路径按计划声明处理 |

本次未运行 Rust 测试、构建、打包实验、浏览器、远端 CI、上传、推送或 yank；未在浏览器中验证。代码和包内/作者文档的实际改动、归档身份实验、字体生产预览、旧项目迁移和公开安装仍须按 M1–M6 实施，不能由本处置关闭实施验收。源 README、themes、quickstart、docs-ssg、模板和 workflow 在本轮均只读，其后续修改归属已写入计划，没有借改稿同步修改其他原文。SDLC 按仓内协议接续同一 review 的 disposition，实施仍 0/6。

下一步（仅建议，不自动执行）：

    /goal 按 docs/plan/CRATES-IO-P1.md 开发
