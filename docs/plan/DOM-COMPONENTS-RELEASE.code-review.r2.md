# DOM-COMPONENTS-RELEASE · 代码复审 R2（U-01）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 | [DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md) | `921802b3c9d1022033bd500681b870a430652616..921802b3c9d1022033bd500681b870a430652616`；仅 U-01 补证及直接相关回归，工作树输入版本见下文 | [代码初审 R1](DOM-COMPONENTS-RELEASE.code-review.md) |

日期：2026-11-11（Asia/Shanghai）。本轮为**修复效果复审（证据补齐）**，由当前 agent 独立完成，未委派。意图：确认线上 Menubar 指南在 registry 0.3.0 独立 DOM 应用中确实能通过键盘打开、切换和执行命令。仅涉及 DOM；没有 GPU 或 SSG 行为改动。

前轮对象为 `e243405029997be74ebc8e6683e05397fc8d8a36..921802b3c9d1022033bd500681b870a430652616`，原结论为「需补证」。前轮目标与当前 HEAD 相同，修复提交增量为空；当前新增输入是 [U-01 补证](../validation/dom-components-release/u01-menubar.md)及 [M4 记录](DOM-COMPONENTS-RELEASE.records/M4.md)的证据回填。原报告、原结论和计划正文均保持。

## 确认问题

未发现可确认的新增问题；没有 P0–P3 findings。没有代码修复 diff，不单列新的 patch 正确性结论。

## 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1](DOM-COMPONENTS-RELEASE.code-review.md) / U-01；评审对象 `be61162b-1236-46e7-85ed-c1908a56ca6f` | V-13 T2 缺少指南 Menubar 示例的真实键盘证据；其他菜单、表格与树的操作不能代替 | [补证记录](../validation/dom-components-release/u01-menubar.md:8)，HEAD 同 R1；独立 registry 消费项目、构建产物、浏览器脚本/结果及截图均已保留 | **已解决**（补证关闭，非代码修复） | 独立核对示例、依赖来源、资产、六步键盘结果及两张截图，详见下文 | 无 |

R1 只有 U-01，没有其他 findings 或待核实项。本轮旧项关闭 1、实际代码修复 0、不再适用 0、未复核旧项 0；新增问题 0。

## 待核实项与限制

本轮 U-01 范围内没有阻塞待核实项。上次手工组装空渲染的具体根因仍未确定；本轮证据证明公开示例的独立消费路径可用，不追认那次失败为“非 SDK 缺陷”。

本轮复用并独立审阅当前版本已存在的真实浏览器证据，没有重新运行浏览器、编译或完整发行矩阵。原始消费项目和日志位于本机仓外，仓内留有补证说明及两张可见结果截图；跨机复核须同步原始证据或按补证记录重建。本结论不等于完整 UI 视觉验收、跨平台验收或重新审查 P1–P5。

## 覆盖与验证

### 版本和范围

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，main。原输入：`$review-code docs/plan/DOM-COMPONENTS-RELEASE.md 复审 U-01`。
- 当前及前轮目标：`921802b3c9d1022033bd500681b870a430652616`。`git diff --no-ext-diff --no-textconv <前轮目标> HEAD --` 为空，index 为空；没有新增实现提交或未提交代码。
- 初始 dirty 项：M4 记录、既有 SDLC 状态/索引、未跟踪 R1 报告及补证目录。仅 M4 的 T2 结果与观察项两处回填，以及 U-01 补证文档/截图纳入本轮证据；既有台账只作路由，R2 报告和本轮台账为输出，不成为输入。
- 计划版本为 Git blob `79da52221b5f240bf6e951255a0fb736465655d0`，与 R1 相同。聚焦 V-13 T2、M4 退出条件和 §9.2 的证据失效规则，未扩大到其他里程碑的重新验收。

关键工作树输入采用 Git blob 内容标识（`git hash-object -- <路径>`，包含未跟踪/忽略文件的实际字节），结束前再次比较：

| 输入 | Git blob |
| --- | --- |
| R1 报告 | `b1e0c106f6e62ad04c45f3d00f7ce7036b1bf146` |
| M4 记录 | `04d2c45ac6067c806e13a774eca41e2b4015f6fa` |
| U-01 补证文档 | `41d84a6ce05e2d237f791a48b6bd603c32151547` |
| Edit/Undo 截图 | `27e04e41ad5632811e8d8f1a172a15a76cacc61b` |
| Saved/File 截图 | `cf31cdec61f928e36fe99857f8ee1399260640e2` |

上下游只读覆盖：[指南示例](../menus.md:88)、[DOM quickstart](../quickstart.md:225)、themed Menubar 与 primitive 的受控值回写和键盘导航；本轮没有将这些未变源码作为新增缺陷搜查范围。

### 独立核验结果

原始目录：`/Users/rockie/Documents/rustify-releases/crates-io-0.3.0-20261110/v13-t2-u01/`。以下将本轮执行与既有运行证据分开记录。

| 证据层 | 方法及实际结果 |
| --- | --- |
| 本轮来源核验 | Python 3.12 只读脚本提取保存的线上 HTML 代码块，对比 `ApplicationShell.rs`、消费项目 `src/lib.rs` 和仓内指南：完整代码一致；挂载包装保存 AppHandle，`app.js` 先 boot 后 mount，没有注入菜单状态 |
| 本轮线上核验 | web open 工具返回 Internal Error；随后 Python `urllib.request.urlopen` 访问 [线上 Menus](https://rockie.github.io/rustify-ui/menus/) 得到 HTTP 200，提取后的完整 ApplicationShell 与消费项目一致。工具读取失败未当成站点故障 |
| 本轮依赖核验 | 解析独立 `Cargo.toml`/`Cargo.lock`：colors/components/dom/style 四包均为 crates.io registry 0.3.0；唯一无 source 的包是消费项目自身；无 Git 依赖、Makepad、patch/replace 或 CARGO_HOME config 源替换 |
| 本轮资产核验 | 导出的 DOM loader、DOM runtime CSS 和组件 CSS 与该 CARGO_HOME 中 registry 0.3.0 对应文件逐字节一致；wasm 产物存在。第一次脚本误用 `web/runtime.css` 路径导致 FileNotFoundError；读 runtime 常量后修正为 `web/dom-runtime.css`，剩余检查通过。这是核验脚本路径错误，无目标产品改动 |
| 既有构建证据，本轮读取 | `build.log` 记录 `mbx build --locked --release --target wasm32-unknown-unknown --lib` 成功（1m22s），`export.log` 记录 host 资产导出成功；wasm-bindgen 0.2.128 的命令与环境见 [补证记录第 29–38 行](../validation/dom-components-release/u01-menubar.md:29)。未声称本轮重建 |
| 既有真实入口，本轮读取 | `browser.mjs` 通过新 Chromium context 加载真实 public 页面，仅发送键盘事件；定位可见 menubar/menuitem，断言 focus、aria-expanded、菜单关闭及 status。`browser.json` 六步为 File → Save → Undo → Edit → Save → File，status 为 Ready → Ready → Ready → Undone → Undone → Saved；页面/console 错误、失败请求和 HTTP ≥400 列表均空，stderr 为空 |
| 本轮实际看图 | 阅读 [Edit/Undo 展开](../validation/dom-components-release/u01-edit-open.png)和 [Saved/File 焦点](../validation/dom-components-release/u01-saved.png)两图，并逐字节核对仓内截图与原始截图一致；可见结果支持日志，而非依靠测试通过数代替用户结果 |

核验脚本均为终端内联只读代码，没有写入新测试、修改消费项目或重跑已覆盖路径。未启动服务器、浏览器、容器、worktree 或分支，无本轮临时运行资源需要清理。关键输入未发生并发变化。

### 覆盖检查表

- [x] 锁定 R1、当前版本、U-01 关闭标准及 R2 独立报告路径。
- [x] 读取完整证据回填 diff，核对示例 → registry 锁 → boot/mount → 浏览器输入与输出。
- [x] 核对真实入口原始脚本/结果，实际看图；不使用 CATALOG 等替代证据。
- [x] 核对代码增量为空，保留历史失败和非本轮范围限制。
- [x] 保存并回读报告，交付前再次比较 HEAD、计划及证据输入版本。

## 计划符合性

| 承诺 | 本轮结论 |
| --- | --- |
| V-13 T2：依线上 quickstart 新建 registry 0.3.0 DOM 项目，使用 Menus 示例，通过键盘打开并切换项目 | **已核实**。原样示例、依赖来源和实际输入输出相互对应；还完成 Undo/Save 与焦点返回 |
| M4 收口中 R1 指出的 T2 缺口 | **已关闭**。M4 当前回填以直接证据替换旧替代证据，未修改计划退出条件 |
| 其余发行/部署、V-13 T1/T3/T4 | 本轮未重新验收；代码、指南、计划与 R1 相同，延用 R1 已声明覆盖与历史记录，不以本轮局部复审扩张为全量重审 |

## 总体结论与后续

**验收判定：通过验收（U-01 限定复审）。**

修复结论：**已解决**，通过补证关闭 U-01；无需代码修复。没有新增代码 diff 或阻塞回归。R1 唯一保留的证据缺口已满足关闭标准，与原审查范围未变的前提结合，本计划代码验收完成；R1 原「需补证」历史仍保留。

不需要运行 apply-code-review 或再做同范围复审。本轮不是完整 UI 验收；若另需整站 UI 走查，可选：

```text
$review-ui docs/plan/DOM-COMPONENTS-RELEASE.md
```

计划明确承接 PRD §5.2 Q3；若另需核对需求到实现的完整覆盖，可选：

```text
$review-prd-dev-gaps docs/plan/DOM-COMPONENTS-RELEASE.md
```

报告与台账留在当前工作树，未提交、推送、上传或部署；本判定不授予这些操作权限。
