# DOCS-SSG-P4-gaps · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，完整初审 | [DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md) | `4894d4ccc7c462a344489748553615a352031192..c6d49c97c6737968cf59a16cbbc3453bea012b70`；未提交部分另见下文指纹与排除说明 | 无；同名 `.review.md` 是计划文档评审 |

- 评审日期：2026-10-20；观察时间：2026-10-20T16:35:43+08:00。
- 评审对象 ID：`9d6d2249-b91c-4d0b-b2f7-414804385321`。
- 意图：按计划给官方指南提供明确的一句话描述，同步 P4 实际验收结论，并用不改变框架 R 的 CLI 补丁配对发布、更新安装和回滚说明。
- 当前 checkout：`main` / `b807af2672ac434f5039f8d727a912a6332e2f43`，dirty；计划 SHA-256：`830f56edc7e1c150fb442a5e9a19b38318a3b3b8159ead8ed08af4d1be416b56`。

## 1. 确认问题

未发现可确认的问题。新增 findings：0；P0/P1/P2/P3 均为 0。

本轮只裁定该 gaps 计划的实施变更。[计划 §0.5](DOCS-SSG-P4-gaps.md#05-明确不在本期) 明确排除了原 P4 代码复审中 CLI 帮助和新项目 README 的旧回滚建议；本轮没有关闭或豁免该历史问题。

## 2. 待核实项与边界

无阻塞本轮代码结论的待核实项。

- 原 P4 的手机实机、VoiceOver、原生三浏览器和性能不在本轮重新执行；按计划 D5 复用历史证据。D17/D18 的实际原文位于 [PRD 顶部及决策表](../PRD-DOCS-SSG.md)，文档没有将取消的手机测试记为通过，也没有把维护者本人确认写成 Agent 采集的完整朗读证据。
- 本轮未重新执行安装、完整构建、56 项官方浏览器检查或发布/回滚。原执行结果见 [M2](DOCS-SSG-P4-gaps.records/M2.md)、[M3](DOCS-SSG-P4-gaps.records/M3.md)；下面明确列出本轮独立执行的检查，历史结果不重复计为本轮通过。
- 四项线上浏览器回归覆盖实际 DOM 交互和新 llms.txt 断言；它们不是新一轮完整 UI/UX 或原生浏览器验收。GitHub 预览另外核看了三张已保存的 C2 实拍，见下文。

## 3. 覆盖与验证

### 3.1 范围与版本固定

仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。由当前主 agent 单独审查，未使用并行 reviewer。

计划完成记录中的三个提交均已解析，并用 `git merge-base --is-ancestor <SHA> HEAD` 确认在当前分支：

| 里程碑 | 完整提交 | 本轮处理 |
| --- | --- | --- |
| M1 | `53087eb716218aa65b15b24dff529e1d0d9d0823` | 指南 frontmatter、两处结论说明、Rust/Playwright 测试 |
| M2 | `88566ba1ed08223a5834f794840fff5e48f4ba8a` | CLI 0.1.32 与锁文件、配对说明 |
| M3 | `c6d49c97c6737968cf59a16cbbc3453bea012b70` | README、安装/升级/回滚及线上交付说明 |

比较方式为最早实施提交第一父提交到最新登记实施提交的净差异。区间中的 `b28dcafd6e06c0f3556ff4595c17583c640f5628` 只保存本计划进度、记录和验证证据，作为记录回写读取，不作为代码 findings 对象。后续 `b807af2672ac434f5039f8d727a912a6332e2f43` 同样只回填计划、M3 记录及公共读回证据；已检查它没有改变产品路径。

完整阅读了以下 19 个代码、内容、配对及公开说明文件的 diff，并沿相关调用链核查：

- `docs/{quickstart,architecture,navigation,workspace,forms,i18n,data,themes,utility,compatibility,components,vellum}.md`；
- `docs/docs-ssg.md`、`DESIGN.docs-ssg.md`、`README.md`；
- `crates/rustify-docs-cli/tests/search.rs`、`tests/docs-ssg/official.spec.ts`；
- `crates/rustify-docs-cli/Cargo.toml`、`Cargo.lock`。

计划、M1–M3 记录及相关 JSON/日志/截图作为契约和证据读取。框架源码、工作流和脚本只追踪必要上下游，不扩为全仓审查。

启动时有 863 个实际文件处于 tracked 修改或 untracked 状态，全部位于 `.xgent-ai/` 与 `docs/plan/`，没有未提交产品代码。上述元数据不作为本轮新增代码缺陷对象。已分别检查 tracked 净差异、index 和 untracked，并在写报告前复核原输入没有变化。

指纹算法：先以仓根相对路径映射每个文件原始字节的 SHA-256，再对按 key 排序、紧凑 JSON（`sort_keys=True, separators=(',', ':')`）求 SHA-256。启动时未提交集合指纹为 `802e997fd5f58403e1cd74cdca6f54525aa74cc7be7963635e3ea29c4a7caff7`；`git ls-files --stage -z` 原始输出的 SHA-256 为 `519badea82aedbdae67dfd9036856d4b8197d1e1c435cd648564985f678b222d`。未提交集合仅能由原工作树复现；本报告及本轮台账写入不属于输入集合。

运行快照通过 `git archive` 从目标 `c6d49c97…` 导出到 `/tmp/p4g-review-gfeaip1a/target`，未 checkout/reset/stash 用户工作树。验证后以 Git blob 算法对照目标树全部 15,272 个文件，差异为 0。临时 Node 依赖链接、配置和运行输出均留在隔离目录；使用的是本仓已安装依赖，没有安装新服务或容器。

### 3.2 行为链与反证

| 行为 | 核查与证据 |
| --- | --- |
| 源描述 → 页面及 AI 描述 | frontmatter 解析、`content.rs:1872–1897` 的显式描述优先级、`ai.rs:27–39` 的描述选取、`shell.rs:494–508` 三处 meta 输出均未修改；12 份源文件只新增三行 frontmatter，正文保持不变 |
| 中文回退 | [源测试](../../crates/rustify-docs-cli/tests/search.rs)第 165–239 行验证 12 个英文指南、11 个中文回退及 missing-description；[浏览器断言](../../tests/docs-ssg/official.spec.ts)第 270–287 行核对两份索引、回退标注和真实中文快速开始的独立描述 |
| catalogue 生成 | frontmatter 位于 catalogue 标记之前；`xtask/src/catalog.rs` 的标记区间更新与 `rustify-components` 的文档同步测试仍成立，实际检查通过 |
| CLI/框架身份 | C2 第一父提交到 C2 只有 `Cargo.lock`、CLI `Cargo.toml`、`docs/docs-ssg.md` 三个配对文件；R 仍为 `0fd34edaffda1926b38ea64c023b2fe1fc5115c5`，嵌入模板锁没有改动；公开安装二进制重新核对 SHA-256 和 build-info |
| 结论与公开说明 | 对读 PRD D17/D18、[原 P4 M7](DOCS-SSG-P4.records/M7.md)和本计划 M3；0.1.32/C2 安装、0.1.31/C1 回滚、run 39388531674 相互一致。组件模板交互明确说明是在同一 R 的 0.1.31 上验证，没有将历史证据写成本次新执行 |

### 3.3 本轮实际执行

以下 Rust 检查在目标快照执行，固定使用仓库已安装的 Rust 1.99.0 / mbx 1.15.0；Playwright 也从该快照读取测试和源内容。

| 检查/命令 | 实际结果 |
| --- | --- |
| `mbx test --offline --locked -p rustify-docs-cli --test search official` | 2 passed，0 failed；新增源描述测试和既有官方图测试均通过 |
| `cargo fmt --all -- --check` | exit 0 |
| `mbx xtask catalog --write docs/components.md --check` | exit 0；`catalog: no drift (24 rows)` |
| `mbx test --offline --locked -p rustify-components --lib catalog::` | 9 passed，0 failed |
| `node node_modules/@playwright/test/cli.js test -c review.online.config.ts --project=official-meta --grep 'official Chinese home\|CATALOG button uses\|deep not-found\|the dark first frame'` | 4 passed，0 skipped/failed/flaky，26.8 s；配置继承仓内 official-meta，改用线上 URL、关闭 webServer、输出到隔离目录，按环境配置代理；console、请求失败和 CSP 断言均通过 |
| 读取线上 `rustify-docs-manifest.json` | `cli_version=0.1.32`，`build_id=f74f4a4ece90f08a`，与 [V-7 记录](../validation/docs-ssg-p4-gaps/m3/c2-online.json)一致 |
| 公开安装二进制收据与 `--build-info` | 既有二进制 SHA-256 与 [公开安装收据](../validation/docs-ssg-p4-gaps/m3/public-install.json)相同；实际输出 0.1.32、`panic: unwind`、source 为完整 C2；本轮没有重新安装 |
| `python3 /tmp/p4g-review-gfeaip1a/verify-archives.py` | 实际打开并核对本机 C1/C2 两份归档的 SHA-256；逐文件结果见下段 |
| 核看 C2 GitHub 预览实拍 | [quickstart](../validation/docs-ssg-p4-gaps/m3/github-quickstart.png)、[components](../validation/docs-ssg-p4-gaps/m3/github-components.png)、[architecture](../validation/docs-ssg-p4-gaps/m3/github-architecture.png)均显示 frontmatter 表格与其后的可读 H1/正文；没有将源码 fence 裸露为正文。截图是既有 M3 证据，本轮没有重新打开 GitHub 页面 |

归档核查以[本机 C1 收据](../validation/docs-ssg-p4/m7/release-20261019-c31/c1-archive.json)和[本机 C2 收据](../validation/docs-ssg-p4-gaps/m2/c2-archive.json)为输入，两者各 97 个文件、路径集合相同，70 个逐字节不变。变化的 27 个文件恰好为 23 个指南 HTML、两份 llms.txt、sitemap 与 manifest。对 23 页逐条验证三处 meta 描述等于目标源 frontmatter，除描述与 `<time>` 外 HTML 相同；两份索引各 37 条且指南描述、中文回退与中文快速开始均匹配；sitemap 恰有 12 个指南 lastmod 更新为 2026-10-20。HTML gzip -9 最大增量为 44 B。岛、搜索与 CSS/JS 字节未变化；没有把本机归档冒充 CI 线上归档。

环境说明：隔离目录第一次经 mise shim 调用 mbx 时因未信任复制出的配置而未进入编译；改用已安装工具链的实际 PATH 和 `RUSTUP_TOOLCHAIN=1.99.0` 后通过，没有修改全局信任或工具配置。归档临时探针首次把中文文件固定行误当成 description，修正为按字段读取后通过；该探针错误不属于产品缺陷。原始运行日志、快照与临时验证脚本位于 `/tmp/p4g-review-gfeaip1a/`，属于本机临时证据；历史发布与安装的可共享证据仍保留在原目录。

## 4. 计划符合性

按当前输入计划的 M1–M3 范围核查，不评价后续里程碑或重开路线设计。

| 需求/约束 → 任务 | 实现与验证 | 状态 |
| --- | --- | --- |
| WORK-F12、D-01 → M1 / V-1、V-5、V-8 | 12 份显式单句描述；源分析、23 个 HTML 和线上两份索引均核对；没有改框架描述语义 | 已核实 |
| G-01、WORK-N3/N5 → M1 / V-3 | 作者参考与 DESIGN 同步桌面、手机和 VoiceOver 结论，保留历史范围及维护者本人验收身份 | 已核实 |
| D3、C-4 → M1/M3 / V-2、V-6 | catalogue 检查及 9 个测试通过；已核看三张对应 C2 的 GitHub 预览实拍 | 已核实 |
| C-1、D1、D5 → M2 / V-4 | C2 的三文件边界、0.1.32/R0 身份、归档 SHA 与允许变化复核通过；本轮没有重跑完整 toolchain 组，沿用 M2 原记录 | 已核实 |
| WORK-F16、C-2 → M3 / V-7–V-10 | 原记录覆盖授权、同 run 97 文件、公开安装及说明推送/读回；本轮独立复核公开安装二进制身份、线上 manifest 和 4 项浏览器任务，未执行新的公开动作 | 已核实；历史发布步骤依原证据 |

未发现本期需求遗漏或越界实现；未发现需要修复的新增兼容性、安全或性能缺陷。

## 5. 总体结论与下一步

**验收判定：通过验收。**

**正确性结论：`patch is correct`。** 本轮声明的净差异未发现需要作者修复的缺陷，关键描述、回退、配对和公开说明有静态、归档及真实浏览器证据支持。该结论不覆盖原 P4 明确排除的历史问题，不代表完整 UI/UX、需求终验或新的发布授权。

本轮无需执行 apply-code-review。可选下一步：

```text
$review-prd-dev-gaps docs/plan/DOCS-SSG-P4-gaps.md
```

本轮评审报告和 SDLC 记录由当前评审交付，代码与计划正文保持只读。
