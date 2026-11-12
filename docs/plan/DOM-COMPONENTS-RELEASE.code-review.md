# DOM-COMPONENTS-RELEASE · 代码初审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮代码报告 |
| --- | --- | --- | --- |
| R1 | [DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md) | `e243405029997be74ebc8e6683e05397fc8d8a36..921802b3c9d1022033bd500681b870a430652616` 累计净差异；排除计划、记录和 SDLC 台账作为代码 finding 对象 | 无 |

日期：2026-11-11（Asia/Shanghai）。意图：将已实现的 DOM 组件、schema v2 和 Colors 以十包 0.3.0 发布，并更新官方静态站的安装说明、导航与部署链路。仅涉及 DOM/SSG 发行；GPU 仅核对未纳入 registry 发布这一边界。

本轮为代码初审；同名 `.review.md` 是计划评审，不是代码复审的前轮。单 agent 审查，未委派。

## 确认问题

未发现可确认的新增代码缺陷。没有 P0–P3 findings；下列证据缺口不伪装为代码 bug。

## 待核实项

### U-01 · 补齐 V-13 T2 的 Menubar 键盘真实入口证据

计划 [V-13](DOM-COMPONENTS-RELEASE.md#91-检查表) 明确要求：按线上 quickstart 新建 registry `=0.3.0` DOM 项目，把 menus.md 的 Menubar 示例放进去，使用键盘打开菜单并切换项目。现有 [M4 验收表](DOM-COMPONENTS-RELEASE.records/M4.md#验收表) 第 15 行虽称四任务全过，却将 T2 的键盘开启指向观察项；[观察项](DOM-COMPONENTS-RELEASE.records/M4.md#观察项不阻塞列代码评审关注) 第 28 行明确记载 menu-root 空渲染、通用选择器未开启菜单，替代证据是 CATALOG menu 和 Data table/Tree 操作。这些不能证明文档 Menubar 的键盘任务完成，也不足以独立证明“非 SDK 缺陷”。

静态核对 `docs/menus.md:88–132` 的 ApplicationShell 与 `crates/rustify-components/src/menubar.rs:1–49`、`primitives/menubar.rs:126–210`：受控值回写、Root/Menu/Trigger/Portal/Content 的使用形态一致，尚未确认示例本身存在缺陷。仓外 T2 目录保留了扩展消费 fixture，但未取得完成该指南示例键盘操作的可回放证据。本轮三篇指南的真实浏览器访问也只证明内容交付，不能关闭 U-01。

补证方式：在仓外干净 DOM 消费项目中，按线上 quickstart 使用 `rustify-dom`（别名 `rustify-ui`）和 `rustify-components =0.3.0`，在同一 mounted scope 中放入线上 ApplicationShell；用 wasm-bindgen 0.2.128 构建、导出匹配 loader/CSS。在浏览器聚焦 File，用 Enter/ArrowDown 打开并执行 Save，再用方向键切换到 Edit、执行 Undo，记录可见菜单、焦点及 status 变化，同时确认无页面错误。如果失败，先定位组装、指南或 SDK 的具体原因；只重验这一任务与实际修复波及项，不用其他目录的菜单测试替代。不要求新增局部测试作为补证。

## 覆盖与验证

### 版本与边界

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，main。
- 初始 HEAD 与最终核对 HEAD 均为 `921802b3c9d1022033bd500681b870a430652616`；工作树及 index 初始干净，无 untracked 代码。报告和本轮台账为输出，不作为输入。
- 基线：`e243405029997be74ebc8e6683e05397fc8d8a36`，即首个登记实现提交 A 的父提交。
- 计划版本：HEAD 中 `docs/plan/DOM-COMPONENTS-RELEASE.md`，Git blob `79da52221b5f240bf6e951255a0fb736465655d0`；读至末尾，结合 M1–M4 记录核对退出条件。
- 计划登记提交及记录中的补充提交连续；`ec0590375a8ef202838ae66047e8629de13e27c2`、`f5cfb2a8808ee131457550ea41bcfb834802423f` 为计划/记录/台账回写。用户本轮明确同意额外纳入 `921802b3` 的 quickstart 措辞修正。
- 读取净 diff，追踪 manifest → inventory → freeze 资产 → 新项目模板/锁 → prepare 投影 → Pages 的调用链。未把发布前已实现的 P1–P5 全部组件源码重新作为本轮 diff 进行普查。

纳入提交（完整 SHA）：

```text
53ff06661ed7353b83a0cd8f9ff656e3cc5e5049
3f2fd1e18e48b0cf76c634512411dfb0493d35d0
2445aea6d5984a828249c022623681a8f987eeef
6a0c2b62a0b20a3166f5ed059acbe86bddf5b32b
dde6fe85149c7f362f458a8a3139257fd0984c67
a25580f1c03d3a4e0ff8b991be4f282476c5f2c3
ccb58f84b5fbe30105023e2c47a0b2dd571cc8d1
1075c0e2c78265cfcfa396208dc8c128138c6793
6993fed914cbdd3dc22df593c1b45568ad96e84b
8d3544bb1a85c1e02863d1753f086f7aebe90584
a7496e34358b7ae578907179f9776e8768af887b
a99c6533ac02fbb0b002cb1213e4d33d49fe72f4
596175e3550794fd83683e433702bea0534b509c
3ed0d4ab9084e439d096ece4b8acfd8fd9f11538
458333d53340ca85a0e8c5525dcad426a84e1901
921802b3c9d1022033bd500681b870a430652616
```

### 文件与行为覆盖

| 范围 | 核对内容与结果 |
| --- | --- |
| 根 Cargo.toml/lock、十包 manifest/README、colors LICENSE、release/crates-io.toml、sources.lock.json | 十包版本、精确内部钉、colors 发布白名单与依赖顺序一致；GPU facade/bridge 未进入清单。README 版本替换未改变运行时行为 |
| CLI assets/framework-release.json、components-Cargo.lock、components 模板、theme.example.json、new.rs 与相关测试 | library_rev 指向 R，三包为 0.3.0；模板精确钉与 checksum 锁同步；四个 light token 与既有默认值对齐；测试计数同步新增指南 |
| site/Cargo.toml/lock、tests 下各改动的 Cargo.toml/lock | path 包升版；站点 registry 锁对齐根锁，注意多版本 syn 槽位；锁差异没有作为手写业务逻辑处理，实际安装/构建证据见 M2/M4 |
| 两个发行 workflow、runbook、AGENTS.md | public-panel-consumer 预取、macos-15、site 锁预取接入既有流水线；未改变 candidate 检出和 prepare 身份校验 |
| rustify-docs.toml、official.spec.ts、site/src/pages.rs | include 与英/中文 Guides 顺序一致；三篇英文回退，固定计数相应更新；中文首页描述十包 registry 发行 |
| README、CHANGELOG、quickstart/zh quickstart、docs-ssg、themes、迁移、components/forms/menus/panels | 安装钉、R/C 身份、v2 最低版本、registry 0.2.1 回滚说明与平台边界一致；921802b3 只改措辞 |
| theme-v2.spec.ts、hydration.spec.ts | 数值序列化容差保留实际值比较；无脚本测试仍检查根 token，两态元素外观留给 hydration 用例；ContextMenu 改查 data-state，未把普通 disclosure 契约强加到 context trigger。未确认测试修改引入实质回归 |

### 本轮实际执行

| 方法 | 命令/步骤 | 实际结果与限制 |
| --- | --- | --- |
| 静态/版本 | `git diff --no-ext-diff --no-textconv`、log/name-status、manifest/锁及上下游读取 | 完成上述范围核对；非全仓普查 |
| 发布库存 | `/opt/homebrew/bin/python3.12 scripts/crates-io.py inspect` | exit 0；十包全部 0.3.0，330 个 registry dependencies |
| 配对定向测试 | `/opt/homebrew/bin/python3.12 -m unittest discover -s tests/crates-io -p test_pairing.py` | 7/7 通过（0.228s）；验证 registry/source 身份、manifest 投影、错误身份拒绝及历史配对。未安装/上传包 |
| 冻结后字节门 | `git diff --quiet 3f2fd1e1 458333d5 --` 九库目录及 Cargo.toml/mise.toml/release 清单；`git diff --quiet dde6fe85 458333d5 -- crates/rustify-docs-cli` | 两门均 exit 0。当前代码相对 E 仅额外 quickstart 措辞与记录变化，本轮配对测试的实现输入未变 |
| 真实浏览器 | 已安装 `@playwright/test` Chromium，独立 newPage，逐一访问线上 `/menus/`、`/panels/`、`/dom-components-migration/`，等待 networkidle | 三页均 HTTP 200、h1 正确、侧栏 39 链接、正文含 0.3.0；pageerror 数为 0。未完成 U-01 的仓外组装及键盘任务，也未声称完整 UI 验收 |
| 外部读取限制 | web open 线上 menus 与 manifest | 工具返回 Internal Error；随后实际浏览器成功访问三篇指南。未以失败的 web 返回推导站点故障 |

本轮直接使用当前 checkout 执行 inspect/配对测试；与目标快照一致。没有运行历史 R/C 的新构建。M1–M4 所列 source 全量回归、staging/public 安装、checksum、docs.rs、CI、V-6、V-11/V-12 及其他 V-13 任务属于**既有记录证据**，本轮没有重新执行或逐个审计全部原始日志，不记为本轮测试通过。没有重新上传、部署或改动计划/源码。

### 覆盖检查表

- [x] 读取计划、当前产品/设计与工具配置，核定初审范围和报告路径。
- [x] 读取发行净差异，核对版本配对、导航与构建上下游。
- [x] 检查计划记录中的已知失败及其替代证据，不将完成自述直接视为通过。
- [x] 对候选问题寻找反证；不足以证明代码故障的项单列 U-01。
- [x] 执行有针对性的配对与线上指南检查，区分本轮/历史证据。
- [x] 保存报告并回读；交付前核对 HEAD 和输入文件未被并发修改。

## 计划符合性

| 承诺 | 核对结论 |
| --- | --- |
| R-1 / R-3：官方站与新增三篇 Guides | 配置与 spec 已实现；三篇线上指南本轮可访问。完整线上身份/语言/搜索任务引用 M4 V-12/V-13 原记录 |
| R-2 / C-2：十包 0.3.0，registry 配对 | 库存、模板/锁、R/C 字节门与本轮配对测试一致；公开上传和归档一致性引用 M2/M3，未重新发布 |
| C-1：v2 最低消费者版本 | themes、迁移及发布说明明确 0.3.0；0.2.1 仅 v1 的历史事实保留 |
| M2/M4 环境修复 | 记录明确授权了 runner 与预取修复；虽超出初始“零改动面”，不因有意且获授权的必要修复报缺陷 |
| C-3 token 收尾 | M3 记录用户明确决定保留 token，本轮尊重其决定，不再报撤销缺陷 |
| V-13 T2 / M4 收口 | **缺证**，见 U-01；其他菜单和表格操作不能代替该任务。计划“4/4 完成”的原始记录保留，本轮不自动改写 |

## 总体结论与后续

**验收判定：需补证。**

正确性结论：`inconclusive`（完整计划验收）。本轮未发现足以判定 `patch is incorrect` 的代码缺陷；发行配置、配对与线上新增指南已有正向证据，但 V-13 T2 与 M4 退出条件仍缺关键真实入口证明，不能判整期通过验收。

先补 U-01 的文档示例键盘任务。若暴露产品/文档问题，再按具体根因修复并重验相关路径；若任务通过，保存可追溯记录即可关闭该缺口，不必空跑 apply-code-review 或重跑无关全量矩阵。

补证后复审命令：

```text
$review-code docs/plan/DOM-COMPONENTS-RELEASE.md 对照 docs/plan/DOM-COMPONENTS-RELEASE.code-review.md 复审 U-01 及直接相关改动
```

本报告不构成合并、发布或部署授权。
