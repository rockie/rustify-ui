# DOCS-SSG-P1 · Rustify UI 纯 DOM SDK 与静态内容文档站 · 阻塞决策记录

## 对象

| 类型 | 文档 |
| --- | --- |
| 开发计划 | [DOCS-SSG-P1](DOCS-SSG-P1.md) |
| 评审报告（只读，回读 F-01、F-04、F-06、F-07、F-14 与处置节） | [DOCS-SSG-P1 评审报告](DOCS-SSG-P1.review.md) |

## 处理轮次

| 轮次 | 日期 | 计划版本（处理前 → 后） | 代码基线 | 本轮解除 | 新增/重开 | 仍阻塞 | 计划状态 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 第 1 轮 | 2026-10-10 | `97b094c475d38a5bb77a6562ebcff50457d7953c76743bb65347fc2e08e0cb1c` → `b78e6302cbe2b002d994c13404f4b86cf9321d15d5b08070f74e4caea03ea58a` | `b0a6bc5b9010e57d1f0d758a37af573ea0712faa`，dirty（仅 `.xgent-ai/sdlc/`、PRD/计划/评审文档未提交；代码无改动） | B-01、B-02、B-03、B-04 | — | — | Blocked → Ready |

## 阻塞总览

| B-ID | 阻塞事项 | 来源定位 | 阻塞范围 | 类型 | 结果（最新） | 计划落点 |
| --- | --- | --- | --- | --- | --- | --- |
| B-01 | 纯 DOM 运行时、loader、基础 CSS 的分发与版本握手 | 计划 §0.2 D9、ADR-1、§4.1、§11；评审 F-01 | M1 T-1/T-2；M9 文档 | 需要决策 | 已解除（用户确认，第 1 轮） | §0.2 D9、ADR-1、§1.2、§4.1、§11、恢复快照 |
| B-02 | CLI panic/profile 策略与 CSR/SSR 分组 CI | 计划 §0.2 D10、ADR-2、§4.2、§5.5、§8、§11；评审 F-04、F-06 | M2 开工/锁依赖；M6 dev 恢复；M9 安装 | 需要决策 | 已解除（用户确认，第 1 轮） | §0.2 D10、ADR-2、§4.2、§5.5、§8、§11 |
| B-03 | P1 CLI 支持的操作系统与原生依赖约束 | 计划 §0.2 D10、§4.2、V-11、§11；评审 F-07 | M2 锁依赖；M7/M9 V-11 出口 | 需要决策 | 已解除（用户确认非推荐项「仅 macOS」，第 1 轮） | §0.1 C-6、§0.2 D10、§4.2、V-11、M7、§11 |
| B-04 | 品牌配置 v1 的主色/logo 结构 | 计划 §0.2 D11、§3、§6、§11；评审 F-14 | M4 配置 schema 冻结；M5 主题 | 需要决策 | 已解除（用户确认，第 1 轮） | §0.2 D11、§3、§6、§11 |

## 逐项记录

### B-01 · DOM 资源分发与版本握手（D9）

- 来源：计划 §0.2 D9、ADR-1「资源分发补充」、§4.1、§11 D9 行；[评审报告](DOCS-SSG-P1.review.md) F-01（处置节「待决策」）。
- 阻塞范围：WORK-F17、C-2；M1 的 T-1（中性 Rust binding）与 T-2（DOM JS host/loader 与消费者配方）开工前即卡住，M9 安装文档依赖同一配方。
- 上下文：GPU loader 无法给纯 DOM 消费者复用，而 runtime/loader/CSS 以什么形式到达仓外消费者、如何保证 JS 与 Rust 同版本，决定 T-1/T-2 的文件布局与公共入口。
- 已核实事实：
  - `web/loader.js:1-4` 顶层静态 import Makepad bridge、bindgen glue、message_bridge 与 `embedded.js`，纯 DOM 页面无法沿用。
  - `crates/rustify-makepad/src/wasm/host.rs:102-110` 的 hooks 存于 thread_local，未 `rustify_makepad_boot` 时为 `None`；`defer`/`listener_options`/`observe_resize`（同文件 131–168 行）依赖它。
  - 仓内先例：`crates/rustify-ui/src/utility/browser.rs:15` 以 `#[wasm_bindgen(module = "/web/utility-host.js")]` 随 bindgen 带出 crate 内 JS；`xtask/src/build.rs:463` 的 `copy_bindgen_snippets` 已处理 `snippets/` 目录。
  - 根 `Cargo.toml:28` 固定 `wasm-bindgen = "=0.2.128"`。
- 约束：C-1（消费者只需 Rust 稳定版、wasm32 目标与 CLI）、C-2（trap 隔离/abort 清理不变）、PRD N4。
- 选项：

  | 选项 | 做法 | 后果与代价 |
  | --- | --- | --- |
  | A（推荐） | runtime 作为 module snippet 随 SDK 分发；loader/基础 CSS 为 `rustify_ui::runtime` 常量、components 导出 `CSS`，由模板 `export-dom-assets` 写入 public/；协议号握手 | JS 与 Rust 必同 rev，错配明确拒启；SDK 多两个公共常量与三个 wasm 导出约定 |
  | B | 文档指导作者手动复制 loader/runtime/CSS | 公共面最小；版本一致依赖作者自觉，仍需握手；作者需访问仓库或 Cargo 缓存 |

- 推荐与理由：A。仓内已有 snippet 分发先例且构建链已处理 snippets；资源随依赖 rev 自动一致，满足 C-1 不额外增加作者步骤；握手把版本错配变成明确的 startup 失败，契合 C-2。
- 结论：已解除（用户确认）；用户 2026-10-10 在交互问答中选择「SDK 分发+握手（推荐）」。
- 计划修订：§0.2 D9 改为定案；ADR-1「资源分发」去掉待确认措辞并写明未选手动复制及其原因；§1.2 去掉“推荐布局”；§4.1 开头改为已定合同；§11 删除 D9 行；恢复快照更新下一步与阻塞。
- 复核：回读确认 §4.1 配方、T-1/T-2、V-1/V-2 与 D9 一致，无残留“待确认/定案前不实施”。

### B-02 · CLI panic/profile 策略与 CSR/SSR 分组 CI（D10 第一部分）

- 来源：计划 §0.2 D10、ADR-2「构建补充」、§4.2、§5.5、§8、§11 D10 行；评审 F-04、F-06。
- 阻塞范围：WORK-N4、WORK-N6、WORK-F13；M2 开工与锁依赖前；M6 dev last-good 与千次保存；M9 安装命令。
- 上下文：根 release profile 为 abort，若 CLI 继承之，dev 服务在任何渲染 panic 时整进程退出；且文档包入 workspace 后 csr/ssr 合并编译行为改变，CI 分组方式需定。
- 已核实事实：
  - 根 `Cargo.toml:37-40`：`[profile.release] codegen-units = 1, lto = "fat", panic = "abort"`。
  - syntect 首次使用 grammar context 时惰性编译正则并 `expect`：`~/.cargo/registry/src/*/syntect-5.3.0/src/parsing/regex.rs:70`。
  - 2026-10-10 scratchpad spike（cargo 1.99.0，临时 file:// git workspace，根 release=abort、`[profile.docs-cli] inherits="release", panic="unwind", lto="thin"`）：
    - `cargo install --git file://… --profile docs-cli` 产出的二进制 `catch_unwind` 恢复成功（输出 `recovered=true`，exit 0），证明 git 安装采用根 workspace 的自定义 profile；
    - 不带 `--profile` 安装继承 release，同一程序 abort（exit 134）；
    - `[profile.release.package.<pkg>] panic = "unwind"` 被 Cargo 拒绝：「`panic` may not be specified in a `package` profile」；成员包自身的 `[profile]` 被忽略并警告“specify profiles at the workspace root”。
  - Leptos：`leptos-0.8.22/Cargo.toml` 同时定义 csr/ssr/hydrate；`tachys-0.2.19/src/html/property.rs:25` 在 ssr 下丢弃 `prop:` 值；`.github/workflows/verify.yml:24,46` 现以 `--workspace` 一次编译。
  - `crates/rustify-components/Cargo.toml:15` 当前以默认 features 依赖 rustify-ui。
- 约束：CLAUDE.md 运行契约 `panic = "abort"` 被 GPU 实例 trap 语义依赖（根 release 不可改）；C-1；PRD N6/AC-4（千次保存不重启）。
- 选项：

  | 选项 | 做法 | 后果与代价 |
  | --- | --- | --- |
  | A（推荐） | 根新增 `docs-cli` profile（unwind/thin），安装命令带 `--profile docs-cli`，每代 catch_unwind，abort 误装时 dev 拒启；CSR/GPU 与 docs 两包分组运行，SDK/components 测 default/gpu/ssr | 安装命令必须带 profile；CI invocation 增多 |
  | B | 保持 abort，把分析/渲染移入可重启子进程 | 安装命令简短；新增进程边界、IPC、last-good 交接，M6 架构更重 |
  | 排除 | 把根 release 改为 unwind | 破坏 GPU/wasm 依赖的 abort 语义与体积预算，不作为选项 |

- 推荐与理由：A。spike 已证实自定义根 profile 对 git 安装生效且能恢复，按包覆盖不可行；A 只增加一个 profile 而不引入进程边界，直接满足 N6 千次保存不重启；分组 CI 避免 csr+ssr 合并掩盖 SSR 行为。
- 结论：已解除（用户确认）；用户 2026-10-10 选择「docs-cli unwind 档（推荐）」。
- 计划修订：§0.2 D10 改为定案（含 B-03 的平台结论）；ADR-2「构建」写入 spike 证据与未选项；§4.2 开头改为已定；§5.5、§8 去掉“推荐”；§11 删除 D10 行，「SSR/原生依赖与 git 安装 profile 尚无实测」行改为当前否。
- 复核：回读确认 §5.5、§8 安装命令、V-9/V-11/V-15 与 profile 一致；spike 仅证明 Cargo 机制，本仓真实依赖图下的 file:// 排练仍归 M2，安装二进制 panic 注入归 M6/M9。

### B-03 · P1 CLI 支持平台与原生依赖约束（D10 第二部分）

- 来源：计划 §0.2 D10、§4.2 新依赖行与平台段、V-11、M7、§11 D10 行；评审 F-07。
- 阻塞范围：WORK-N4（AC-3：只装 Rust 与 CLI 五命令可用）；M2 锁依赖前；M7/M9 的 V-11 出口范围。
- 上下文：最小工具链需要明确适用的 OS；原生依赖（TLS、正则后端）是否引入额外系统库取决于 OS 承诺。
- 已核实事实：
  - `.github/workflows/verify.yml:11,61,147` 与 `evidence.yml:35` 全部 `runs-on: macos-14`，无 Linux/Windows runner。
  - `syntect-5.3.0/Cargo.toml:51` 默认 `default-onig`（需 C 编译），另有 `regex-fancy`（103 行）。
  - PRD 未指定支持的 OS；参考机为 Apple Silicon macOS（`docs/PRD-DOCS-SSG.md:166`）。
  - 推断（未实测）：macOS 上 native-tls 走系统 Security 框架、onig 只需 Xcode CLT 的 cc，二者都不额外违反 N4/AC-3；F-07 的 OpenSSL/pkg-config 风险只在 Linux 成立。
- 约束：PRD N4/AC-3；C-1；CLAUDE.md 不擅自安装服务/容器。
- 选项：

  | 选项 | 做法 | 后果与代价 |
  | --- | --- | --- |
  | A（推荐） | 承诺 macOS 与 Linux，Windows 不承诺；禁 OpenSSL 等系统库，rustls、fancy-regex；CI 增 ubuntu | 覆盖 Linux CI 建站；Linux 证据需 Linux 主机或推送后 CI |
  | B（用户选择） | 仅 macOS；Linux/Windows 未验证不承诺 | 与现 CI 一致、验证成本最低；Linux CI（如常见 GitHub Pages 流水线）建站不在承诺内 |
  | C | 三平台含 Windows | 额外验证 rename/File::lock、路径与 PATH，增加 Windows runner |

- 推荐与理由：推荐 A，理由是 GitHub Pages 类部署通常在 ubuntu runner 上构建。用户选择 B；该选择不与已核实事实或 PRD 冲突，也不引入新的阻塞，后果已在提问选项中说明：Linux 建站不在 P1 承诺内。
- 结论：已解除（用户确认非推荐项）；用户 2026-10-10 选择「仅 macOS」。
- 计划修订：账本新增 C-6（P1 CLI 平台）；§0.2 D10 写入仅 macOS；§4.2 新依赖行把 rustls/fancy-regex 由硬性要求改为优先选择，硬约束保留为 N4/AC-3 在 macOS 上判定；§4.2 平台段、V-11、M7 去掉 Linux；§11 新增非阻塞行「P1 仅承诺 macOS」并写重评触发（P2 官方站 CI 或用户需要 Linux/Windows 建站时）。
- 复核：回读确认全文无“macOS/Linux”“两 OS”残留；V-11 只要求 macOS 原生环境，不再依赖 Linux 主机或推送后 CI。

### B-04 · 品牌配置 v1 结构（D11）

- 来源：计划 §0.2 D11、§3 站点配置行、§6 品牌段、§11 D11 行；评审 F-14。
- 阻塞范围：WORK-F8/AC-3、WORK-N5；M4 配置 schema 冻结前；M5 主题与 V-8/V-13 fixture。
- 上下文：v1 对未知键/形式报错，schema 发布后改形是破坏性变更；单值主色能否在浅深两态都可读决定结构。
- 已核实事实（WCAG 相对亮度公式计算，非浏览器实测）：
  - 浅色底 `#f9fafb` 相对亮度 0.9548，主色需 ≤0.1733 才达 4.5:1；深色底 `#0c111d` 相对亮度 0.0057，主色需 ≥0.2005。不存在同时满足两态的单色。
  - `DESIGN.md:7` 浅色 primary `#1570ef` 在 `#f9fafb` 上 4.37:1（不足 4.5）；计划阅读主题 `#175cd3` 为 5.73:1；`DESIGN.md:26` 深色 `#53b1fd` 在 `#0c111d` 上 8.15:1。白字在 `#175cd3` 上 5.99:1，`#0c111d` 字在 `#53b1fd` 上 8.15:1。
  - `docs/theme-document.schema.json` 的 `styles` 必需 `light` 与 `dark` 两组值，P2 主题文档导入（F8/AC-5）为成对结构。
- 约束：PRD F8/AC-3（主色与 logo 两种模式下都生效）、N5 对比度；§3 v1 未知键报错。
- 选项：

  | 选项 | 做法 | 后果与代价 |
  | --- | --- | --- |
  | A（推荐） | `theme.primary` 可缺或 `{light,dark}`；logo 可缺、单路径或 `{light,dark}`；按钮前景自动黑/白；低对比警告，deny-warnings 失败 | 作者需填两色；与主题文档结构一致；以后可无破坏追加单值简写 |
  | B | 单色加自动派生 | 配置最简；至少一态发布色与填写色不同，派生算法成为 v1 合同 |
  | C | 两种写法都接受 | 最灵活；两条路径与派生算法都需在 v1 冻结并测试 |

- 推荐与理由：A。数学上单色不可能两态同时达标；与 DESIGN.md 及主题文档的浅深成对结构一致；v1 先发严格形状，后续追加单值简写是兼容扩展，反向则破坏兼容。
- 结论：已解除（用户确认）；用户 2026-10-10 选择「浅深成对（推荐）」。
- 计划修订：§0.2 D11 改为定案；§3 去掉“待用户定案后冻结/品牌推荐形状待确认”；§6 品牌段改为已定规则；§11 删除 D11 行。
- 复核：回读确认 §3 字段、§6 规则、V-8/V-13 品牌 fixture 一致。

## 未解除阻塞

无。以下是计划 §11 已登记的后续里程碑门，不属于开工阻塞，本轮未处理：固定 Chrome 来源（M2 风险门前）、Firefox/WebKit 与真实 Safari/手机/走查人（M5/M8）、远端候选推送授权（M9）、GPU 全回归基线既有失败（M1 出口）。

## 就绪结论（最新一轮）

- 计划状态：Blocked → Ready
- 理由：D9–D11 三组影响外部契约、架构与配置 schema 的决定均已由用户确认并写入计划；剩余 §11 项是带负责人与最晚确认点的环境/授权/验证门，只阻塞对应里程碑出口，不影响范围、安全、数据、外部契约或关键 NFR 的设计定案。
- 再次运行前需要：无。
