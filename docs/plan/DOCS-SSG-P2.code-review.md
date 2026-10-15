# DOCS-SSG-P2 · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 | [DOCS-SSG-P2](DOCS-SSG-P2.md)，v2，sha256 `2ed168cfd709662798161157e06a8bedc48ee42df37176af1a1d62639cc2a33d` | `785f51ad9fdcb69e624a0c8f85d91e3541d58e90..53d5c48563420c9252cac3f9d1e14647494320a3` 累计净差异，另单列当前未提交内容，指纹见下文 | 无；已有 `.review.md` 是计划文档评审 |

评审对象 ID：`f96764e1-b485-4472-bfa2-1ce820ec8dff`。日期：2026-10-14 +08:00。首次代码评审，由当前主 agent 独立完成；未委派 reviewer，未修改产品源码、原计划或提交代码。

## 确认问题

### F-01 · [P2] 让同一挂载根内的几何样式共享稳定的作用域标记

- 定位：[geometry.rs](../../crates/rustify-components/src/geometry.rs) **163–165**，target `53d5c48563420c9252cac3f9d1e14647494320a3`，与本轮新增文件 diff 重叠。
- 问题：普通 CSR 消费者通过 `rustify_ui::mount` 挂载界面，且两个独立 `Show` 分支分别创建 Progress 等几何组件、没有显式提供全根 `GeometryStyles` 时，每个分支会建立自己的 fallback collector。这里却把两份 collector 的不同 token 写到同一个 `ScopeRoots.container()` 属性上；第二个分支覆盖 token 后，第一份仍存活的样式表选择器不再匹配，原本 50% 的进度条立即变成满宽。关闭第二个分支又恢复第一份 token，画面才恢复。此变更将原先各元素独立的 `style:width` 改成了共享容器属性约束的 CSSOM 规则，因而引入了既有 CSR 路径的可见回归；文档岛预先提供统一 collector，不能证明普通消费者没有这个问题。
- 证据：同文件 69–89 的 fallback context、98–130 的规则建立、149 的共享容器获取与 173–174 的选择器。独立 CSR 产物的 Leptos、SDK、components 三个实际依赖节点均只有 `csr`；严格 CSP 下实测第一条进度条宽度 **128px → 256px → 128px**，轨道始终 256px，第二条 25% 进度条为 64px。两份规则均存在，改变的是容器 token。见 [CSR 原始结果](../validation/docs-ssg-p2/code-review-r1/probe-csr.json)、[故障截图](../validation/docs-ssg-p2/code-review-r1/probe-csr-both.png)、[依赖/版本证据](../validation/docs-ssg-p2/code-review-r1/identity.json)。同样的 `mount` 场景在 hydrate 产物中也复现，随后用独立 CSR 产物排除了呈现模式的干扰。
- 修复与验证：作用域标记应由整个挂载根稳定拥有，多个 collector 的建立/清理不能使其他存活规则失配。增加真实 CSR 回归：两个独立条件分支分别开关几何组件，验证每条尺寸和交互更新，并覆盖先后清理及不同挂载根的隔离；保留已有 SSR/hydrate 几何与节点身份检查。
- 置信度：高；纯 CSR 浏览器复现、样式选择器证据与 diff 的因果关系一致。

### F-02 · [P2] 把 hydrate 专属依赖的清单纳入编译缓存指纹

- 定位：[rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) **847–850**，target `53d5c48563420c9252cac3f9d1e14647494320a3`，与本轮新增构建协调代码 diff 重叠。
- 问题：作者将一个本地 path 依赖只放到站点的 `hydrate` feature 中时，这里仅按 `ssr` 取得 metadata，因此 `Metadata.local_roots` 和 `InputLayout` 的清单集合没有该包。虽然 Cargo dep-info 后来补入其已编译 `.rs` 文件，修改该包的 Cargo.toml（例如切换已有的默认 feature）仍不会改变 `Prepared::reuse` 比较的编译指纹。dev 能观察到保存并完成一次 render，却继续提供旧 wasm，读者交互也继续执行旧 feature；再修改站点 `.rs` 才真正重编译并应用变化。这违反本期对本地依赖/功能变更使缓存失效和源码、wasm 同代的承诺。
- 证据：同文件 643–649 的 local roots、1591–1615 的清单输入构造、138–147 的缓存命中，以及 [计划 §3 的 Rust build cache 合同](DOCS-SSG-P2.md#3-派生数据缓存与兼容)（279 行）。C4 实际 dev 工程中仅把 `browser-palette/Cargo.toml` 的默认 feature 切到 `alternate`，服务端完成事件仍保留原 compile generation，重新打开页面后 wasm SHA-256 完全不变，点击后的背景仍为 `rgb(18, 58, 188)`；随后只给站点源码追加注释，wasm/generation 才改变并得到应有的 `rgb(254, 220, 186)`。见 [完整状态与浏览器结果](../validation/docs-ssg-p2/code-review-r1/probe-dev.json)、[dev 日志](../validation/docs-ssg-p2/code-review-r1/probe-dev.log)、[变更前](../validation/docs-ssg-p2/code-review-r1/optional-initial.toml)/[变更后](../validation/docs-ssg-p2/code-review-r1/optional-changed.toml)清单。初态均为 7，变更后的类只在点击后出现，实验保留了 SSR/接管确定初态。
- 修复与验证：分别读取实际 SSR、hydrate 图，完整收集两端的本地包清单及构建输入，并让这些输入参与复用判断；不要用两模式联合构建代替隔离。回归应只修改 hydrate 专属依赖的 Cargo.toml/default feature，要求 dev 更新 wasm 与可观察行为；同时验证普通 Markdown 保存仍复用已编译 Rust。
- 置信度：高；实际 C4 dev 保存事件、资源摘要和强制重编译对照共同确认。

### F-03 · [P2] 用包身份区分同名依赖的 build script 输入

- 定位：[rust_site.rs](../../crates/rustify-docs-cli/src/rust_site.rs) **1655–1656**，target `53d5c48563420c9252cac3f9d1e14647494320a3`，与本轮新增输入指纹代码 diff 重叠。
- 问题：Cargo 允许通过重命名依赖同时使用两个同名、不同版本的本地包，但这里按 package name 保存唯一根目录，后一项覆盖前一项。`discover_build_inputs` 再从 `build/<name>-<hash>` 取名字查询这张表，导致两个 build script 的相对 `rerun-if-changed` 都被解释到同一个包根。被覆盖包的非 Rust 输入没有进入编译输入/监视集合；保存后 dev 既不启动新一代，也不更新 helper/wasm，直到另一次 Rust 源码变化强制重编译才生效。这是独立于 F-02 的问题：本例两个包都在 SSR metadata 中，补齐 hydrate 图仍会保留名称碰撞。
- 证据：同文件 1850–1857 的包根查询、1872–1876 的相对输入解释，以及 `dev.rs` 276–285 的 Rust/build 输入事件筛选。[metadata 原始包信息](../validation/docs-ssg-p2/code-review-r1/duplicate-packages.json)包含两个 `payload`（1.0.0、2.0.0），各自 build script 都读取自己的 `payload.txt`；[Cargo 回报](../validation/docs-ssg-p2/code-review-r1/build-script-identities.json)已提供不同的 `package_id`/`out_dir`。C4 dev 中只把 v1 输入从 7 改为 14，观察三秒后完成状态/资源摘要未变化，重新打开页面并点击仍得到 8；随后只追加站点源码注释，重新生成的初态与交互才得到 14/15。见 [完整结果](../validation/docs-ssg-p2/code-review-r1/probe-build-input.json)与[日志](../validation/docs-ssg-p2/code-review-r1/probe-build-input.log)。源码函数、两份 Cargo.toml 和输入都保存在隔离 fixture 中。
- 修复与验证：根据 Cargo 回报的完整包身份及对应 build-script 输出目录绑定输入，避免以裸包名猜测唯一根。用两个同名不同版本的 path 包回归，分别只改各自 `rerun-if-changed` 文件，要求预览更新对应 SSR 初态、wasm 行为和 generation；覆盖保存、删除和恢复。
- 置信度：高；实际合法 Cargo 图、build-script 身份、浏览器结果及重新编译对照一致。

## 待核实与验证边界

未有影响上述三项裁定的待核实点。以下属于覆盖限制，不记为新增 finding：

- 本轮没有重跑官方站所有页面、root/subpath/meta 全项目、主题/语言/缩放/无脚本/失败注入全矩阵或正式性能五次测量；本轮浏览器证据只证明上面实际操作的场景。
- 当前稳定版 Chrome、可见 Safari、实体手机以及公共 Git/Pages 的缺口沿用计划与 [M6 记录](../validation/docs-ssg-p2/M6.md)的明确范围；Firefox和公开发布的后移不作为本轮新增需求遗漏。没有声称真机、最新稳定版或线上站点已验证。
- 巨量历史验证 JSON/日志、截图、字体二进制及私有生成缓存没有逐项重验或逆向。跟踪/未跟踪的缓存与日志已单列文件哈希；不以这些产物的存在代替源代码审查或验收。

## 覆盖与验证

变更意图：为 P1 静态文档增加确定初态的 DOM 交互岛、同源 Rust 演示、定制页/插槽/主题、组件模板与 Rust dev 更新，并在本地提供英文官方站。本次采用 DOM 路径，检查共享 SDK 修改对既有 CSR 消费者的影响；GPU 演示及 P3/P4 功能不在本期。

仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。原输入：`$review-code docs/plan/DOCS-SSG-P2.md`。计划模式从完成记录解析提交；用户随后明确选择“一并纳入，审查累计差异（推荐）”，因此包含 `8e86626f`、`0494f304`、`c1061ced`。固定比较方式为 `git diff --no-ext-diff --no-textconv 785f51ad9fdcb69e624a0c8f85d91e3541d58e90 53d5c48563420c9252cac3f9d1e14647494320a3 --` 的净差异，进度回写不作为代码 findings。

登记及补充的主要代码提交：

| 提交 | 本次归属 |
| --- | --- |
| `5632d1329a79800faa2901a7b3de6d52dbc0687e` | M1/M2：SDK 水合、组件 SSR、注册/演示与双目标协调 |
| `676a55771ab77faf8c476a836025d62905a2d3e2` | M3–M5、M6准备：定制/主题/模板/dev/官方站及消费者适配 |
| `8e86626fce74ffc45085da6377ca9aa64312fe59` | 用户补充纳入：C1/0.1.11配对 |
| `0494f304f0f9f8b620affb7309df23db3cb2377c` | 用户补充纳入：favicon修正 R2 |
| `c1061ced3deaa4e6669bc45eb3382306fa85c958` | 用户补充纳入：C2/0.1.12配对 |
| `c562008291ef1b1826891944e0d81a5d61d775b8` | R3：稳定扫描错误与输入变化区分 |
| `043840a18bd7298471073b235ab6be74eb31d2d3` | 已声明资源及完成代的测试观测 |
| `6721dc73db18421826c4abaf0823d51d8d6ceffb` | C3/0.1.13配对 |
| `3fcc626a4039a43c5e045ccd546bab976db89a73` | R4：最终文档检查并行化 |
| `d301344eeb27e22907c59a10805ae0cebf8dee99` | C4/0.1.14配对 |

区间内其他提交、文件及完整 SHA 见 [提交清单](../validation/docs-ssg-p2/code-review-r1/commits.json)。`6a337396`、`f7bb8dcc`、`3566f627`、`4497d656` 等记录/验证/验收口径提交用于合同及证据核查；没有将文档状态变动当作产品功能。

已深入核查注册/props/源码切片 → 双目标构建/describe/资源组 → Markdown、定制页与插槽的页面/资源图 → 最终 HTML 检查，以及 SSR/hydrate 根、ID、几何、浏览器闸门与恢复、主题字体、dev 复用/事件/last-good。检查了模板、官方站声明/导航、特性分包、发布身份和 CI 接线。测试及历史证据按上述行为链选择阅读；没有将全部历史测试产物或 GPU 业务模块扩成全仓普查。

工作树/index固定证据：[manifest.json](../validation/docs-ssg-p2/code-review-r1/manifest.json)。算法为每文件 SHA-256，再对按路径排序的紧凑 JSON 映射取 SHA-256，三个集合分开：

- 相关源码/测试/配置与计划上下文 193 文件：`fb47cf9afd0179da9795f088d0862b8072766401c4003500bf17325f3f4661bf`。
- 原有 tracked 未提交 14 文件：`55a63dc4e8dc17f3e7ce6571a54501f2012f9a61b3af44ccb0355d6ac9f0e799`。
- 原有 untracked 70 文件：`8cb77ed7e9ee8e29e86094e66ad374d2cfae9e60f9b1042148a7273b01782eeb`。

本轮开始时无 staged diff。原有未提交内容是状态/经验记录和生成缓存/日志，没有额外作者源码；逐文件清单、哈希和 index tree 保存在上述 manifest。落盘前复核这三个原输入集合均无变化，HEAD 未变化；本报告、独立证据和随后的评审台账不纳入本轮输入。未提交缓存只保留指纹、没有复制所有二进制快照，跨机器须重核原工作树。

| 实际执行 | 版本/入口 | 结果 |
| --- | --- | --- |
| `mbx test --locked -p rustify-docs` | 当前 checkout `53d5c485…` | 265 passed、0 failed；[日志](../validation/docs-ssg-p2/code-review-r1/docs-tests.log) |
| `mbx test --locked -p rustify-docs-cli --bin rustify-docs` | 同一 checkout | 72 passed、0 failed、1 ignored（原配置的性能代理）；[日志](../validation/docs-ssg-p2/code-review-r1/cli-tests.log) |
| `mbx test --locked -p rustify-components --no-default-features --features ssr` | 同一 checkout，单独 SSR 模式 | 124 passed、0 failed，2 个 doctest 按原配置 ignored；[日志](../validation/docs-ssg-p2/code-review-r1/components-ssr-tests.log) |
| C4 `new … --template components`、离线 `build`、`preview` | 实际已安装 CLI C4 `d301344e…` / 库 R4 `3fcc626a…`；临时工程 | new/build成功；严格 CSP 下 Playwright Chromium `153.0.8010.12` 实测 F-01。另编译纯 CSR 图确认同一缺陷，pageerror/console error/失败请求均 0 |
| C4 `dev --port 0`、保存依赖清单、交互与强制重编译对照 | 同一 C4/R4 临时工程 | F-02复现；保存被观测但 wasm/行为仍旧，修改站点源码后更新 |
| C4 `dev --port 0`、保存同名依赖的 build 输入、交互对照 | 两个合法 path 包，均在 SSR 图 | F-03复现；非 Rust 输入保存未更新，修改站点源码后更新 |

浏览器和实验原始脚本、源码、步骤见 [证据说明](../validation/docs-ssg-p2/code-review-r1/README.md)。只使用已有 Rust/mbx/Playwright和已安装C4，没有安装服务/容器、执行发布或访问业务数据。所有自有预览/dev与浏览器进程已经退出。

实际 C4 二进制 SHA-256 为 `5eae4e6bf727505a030fd67680c7c3a2c82bb6e195815a0d834b18cf4d737b71`；独立读取 `--build-info` 确认 source C4、0.1.14、panic unwind。三个问题的关键源文件在 R4、C4 和当前 HEAD 的逐文件 SHA-256 一致，见 [identity.json](../validation/docs-ssg-p2/code-review-r1/identity.json)。因此没有把其他历史版本的运行结果冒充当前代码证据；也没有重装或声称本轮完成了公开 Git 安装。

## 计划符合性

| 本期契约 | 实现/验证证据 | 核查状态 |
| --- | --- | --- |
| M1/V-2及既有CSR回归：确定初态与几何接管，保留原消费者 | SDK roots/ID、组件属性、GeometryStyles；原有 SSR 测试通过，新增真实 CSR 分支实验 | **存在缺陷 F-01**；不能仅以预先提供collector的岛/harness覆盖普通消费者 |
| M2/V-3、V-4：注册、props、同源源码与资源协议 | 共用parser/宏、源码resolver、helper校验、原生Docs/CLI测试及真实临时工程build | 相关已运行行为已核实；未重跑所有故障注入和冷/热配对矩阵 |
| M3/V-6、V-7：定制页、七slot、主题与字体 | 页面/资源图、shell替换、主题投影与本轮Docs测试 | 局部核实；主题首帧、全slot浏览器矩阵未重验 |
| M4/V-9、§3、WORK-F13/WORK-N6：依赖和build输入失效、同代更新 | Prepared缓存、InputLayout、watch filter、三种实际保存/浏览器对照 | **存在缺陷 F-02、F-03** |
| M5/V-10：12指南与24类DOM官方站 | include、站点声明、CATALOG映射/普通导航接线；M6原证据按其原版本保存 | 静态链路已查；本轮未重新宣布全站浏览器通过 |
| M6环境及本人验收 | 计划当前5/7、M6仍进行中；D15本人验收记录与各环境缺口分别保留 | 不修改用户已确认的本人结论；本轮代码缺陷另行处置 |
| M7/V-15公开安装、Pages及回滚；P3/P4排除项 | 当前计划D16及顶层排除 | 后续范围，未因延期报新增缺陷，未执行发布 |

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`。确认 **3项P2问题**：共享SDK的条件分支几何回归，以及两种独立的dev依赖缓存/监视遗漏。现有已运行测试全部通过，但没有覆盖这些实际失败场景；故不能据测试计数判代码通过验收。

优先修复 F-02/F-03 的完整输入归属与缓存失效，再修 F-01 的挂载根样式所有权；各自保留本报告的失败样本，验证只修改目标输入就能更新行为、任一分支开关都不改变存活组件尺寸。遵守库R/CLIC与版本纪律，并复验受影响的隔离模式/岛/dev回归后，再按本计划复审。代码评审不改变原计划的实施计数，也不构成合并、发布或PRD整体验收授权。

下一步：

```text
$apply-code-review docs/plan/DOCS-SSG-P2.code-review.md
```

修复后：

```text
$review-code docs/plan/DOCS-SSG-P2.md
```
