# DOCS-SSG-P4 · 代码评审 R2（修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 · 修复效果复审，并对上轮未覆盖的登记提交做新增问题检查 | [DOCS-SSG-P4 v2](DOCS-SSG-P4.md) | `06be2ee3f0db519a7316b0d53d57c94c7ee46fd1..4894d4ccc7c462a344489748553615a352031192` 的累计净差异；工作树指纹 `2e36f79c11edac2daa65a97d1456f97fe59ece9072f4b79a1abecf78a6b8e0a7`，其中没有产品代码 | [R1](DOCS-SSG-P4.code-review.md) |

- 评审 ID：`b7178384-b6a0-4f39-9800-588a613f076a`；复审关联 R1 评审 `9cd6da78-0bc0-4297-9f6b-6fffde099e72`。
- 日期：2026-10-20T11:09+08:00；仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；观察 checkout：`main / 4894d4ccc7c462a344489748553615a352031192 / dirty`。
- 上轮报告：[R1](DOCS-SSG-P4.code-review.md)，SHA-256 `662a32eeb9ff1e2a4c31b0fb6d3310b598e16c3e6750487a7278ec75f708c7e4`（未跟踪文件，内容与 R1 登记一致）。R1 审查 `77ca4f47..06be2ee3`，结论为 patch is correct、0 findings，并列了 4 条待核实项；R1 没有处置记录，因为不需要修复。
- 三个版本：上轮审查基线 `06be2ee3`；本轮修复差异 `06be2ee3..4894d4cc`；当前核查版本为 HEAD `4894d4cc` 加上工作树。最终公开配对 C1 `1146b7d8febdf53ca1bc1a4c01946604e7625a95`（CLI 0.1.31）与 R0 `0fd34edaffda1926b38ea64c023b2fe1fc5115c5` 都在该范围内。
- 计划版本：工作树 v2，SHA-256 `52d138436b01590d869ab82e003f425f51e32c8cdc07ae2cf226f88a9f7c97f6`；完成记录为 7/7，全部有提交 SHA。
- 呈现路径：DOM。不涉及 GPU。
- 本轮由主 agent 独立完成，没有委派 reviewer。代码和计划只读；本轮只新增本报告和 SDLC 评审登记。

## 上轮问题复审

R1 没有 findings，所以没有需要逐项关闭的缺陷。下表核对 R1 的全部待核实项，编号用 R1「待核实项与范围限制」的序号。

| 来源报告 / 编号 | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / 待核实 1a | M7 发布门未完成：原生稳定浏览器、VoiceOver、原生 200%、公共 HTTPS Git、Pages 配置、线上全页与回滚都缺证；R1 不判 M7 | M7 提交 `3c38da04`…`f88a4fe6`，[M7 记录](DOCS-SSG-P4.records/M7.md)写四轮 Pages 均通过（C0 首发、C1 发布、C0 回滚、C1 恢复） | 已解决 | 本轮实取线上 `rustify-docs-manifest.json`：build_id `9e8ea80d57750509`，cli_version 0.1.31，base `/rustify-ui/`，97 个文件（96 条加 manifest，含 `.nojekyll`），manifest SHA-256 `5c7db8d1…f6e7`，与 M7 记录的最终 C1 一致。原生浏览器与 VoiceOver 只按 M7 记录和 PRD D18 的本人结论认定，本轮没有重做 | 无；原生证据的产物范围见待核实 1 |
| R1 / 待核实 1b | 同条中的两台实体手机 | PRD D17 与计划 C-5、§9.3 取消本次手机实机验收 | 不再适用 | 用户决定已写入 [PRD](../PRD-DOCS-SSG.md) 的 D17 和计划 §9.3；旧缺证没有改记为通过 | 无 |
| R1 / 待核实 2 | 全量回归与正式性能未在 R1 重跑，只引用 M6 历史版本 | M7 在 C29（`3dbb079a`）上重跑了受影响的套件，见 `docs/validation/docs-ssg-p4/m7/native-20261018-resume/` | 已解决（依记录） | 抽查 `c29-search-performance.json.gz`：passed=true，4 个场景，failures 为空，cli_build_info 的 source 为 `3dbb079a`。R0→C1 与 R0→C0 都只改了五个配对文件和文档，所以 C29 的产品代码与当前 HEAD 一致。本轮没有重跑全量 | 无 |
| R1 / 待核实 3 | R1 的临时浏览器证据只存在当时那台机器 | — | 不再适用 | 这是 R1 自己的取证限制。本轮的同类限制写在下文「覆盖与验证」 | 无 |
| R1 / 待核实 4 | R1 时工作树没有新增产品代码 | — | 不再适用 | 本轮重新核对：839 个 dirty/untracked 文件全部在 `.xgent-ai/` 与 `docs/plan/` 下，index 为空 | 无 |

## 确认问题（本轮新增）

### F-01 · [P3] 0.1.31 的 CLI 帮助和新项目 README 仍以 0.1.3 为回滚目标

- 定位：`crates/rustify-docs-cli/Cargo.toml:6`（HEAD；`1146b7d8` 起作为公开发布版的 0.1.31 配对）；`docs/docs-ssg.md:80-81`（`f88a4fe6` 新增的说明）
- 问题：本区间把 0.1.31 定为公开发布版，[作者参考](../docs-ssg.md)第 62–67 行把本次发布验证过的回滚目标改为 0.1.29（C0 `3dbb079a`）。但二进制里的 `PREVIOUS_REVISION` 和 `PREVIOUS_VERSION` 仍是 P1 的 `42757c62…`/0.1.3，所以 `rustify-docs --help` 打印 “Roll back to rustify-docs 0.1.3”，`rustify-docs new` 生成的内容站 README 写的是 “Roll back to the previous version, 0.1.3”。0.1.3 既不是上一个公开版本 0.1.4，也不是本次验证过的 0.1.29。照工具内提示回滚的作者会装上 P1 CLI，而基线文档（`06be2ee3:docs/docs-ssg.md:26`、`:289`）写明 P1 安装读不了多语言配置，于是 P4 多语言或搜索站点的 check/build 会失败。没有 git 元数据的归档构建也用同一常量，提示安装 0.1.3。`f88a4fe6` 只在参考文档里说明了这个差异，工具帮助和生成文件仍是作者最先看到的入口。
- 证据：[`main.rs:32-33`](../../crates/rustify-docs-cli/src/main.rs) 定义常量，第 70–74、114 行输出回滚提示；[`templates/content/README.md:22-25`](../../crates/rustify-docs-cli/templates/content/README.md) 写死 0.1.3。本轮实际运行公开安装的 C1 二进制（`--build-info` 的 source 为 `1146b7d8…`），`--help` 末段确实输出 `Roll back to rustify-docs 0.1.3` 和 `--rev 42757c627cb69456d241ba0de51b34f1ad85be3d`。参考文档 [`docs/docs-ssg.md:62-67`](../docs-ssg.md) 给的是 0.1.29，第 80–81 行承认帮助里保留着旧配方。
- 修复与验证：下次 CLI 配对时，把 `PREVIOUS_REVISION`/`PREVIOUS_VERSION` 改为本次验证过的回滚目标（`3dbb079a7b188edd56071225fad84fd6b92c4c52`/0.1.29）；模板 README 的版本号改用同一常量（例如新增占位），不再写死；再给 `--help` 和 `new` 生成的 README 加断言，要求回滚版本与常量一致。这需要一个新的 C 配对，可以随下一次 CLI 发布处理，不影响当前线上站点的读者产物。
- 置信度：高。直接读码，并实测了已安装二进制的输出。

本区间的其他改动没有发现可确认的问题：搜索焦点与计数、未译提示底色、对话框宽度、换行表头的锚点偏移、汉字词段兼容、Pages 完整归档、线上 a11y 来源记录、HTTP/2 下 404 的 console 文本，以及 README、作者参考和 DESIGN 的同步。

## 待核实/待决策项

1. **原生 V-11 证据与线上产物不是同一份字节。** 本轮比对了线上 manifest（CI，build `9e8ea80d57750509`）和本地 C0/C1 产物（build `7e1e5c9e7e7392df`）：两者有 6 个岛资源不同（CI 重新编译的 wasm 目录 `916dc515…` 对本地的 `b9140be3…`），引用这些资源的 48 个组件页也不同；其余读者文件相同。计划 §4 说“0.1.31/R0 的 96 个读者文件与 C0 完全相同”，比的是两个本地构建。原生 Chrome、Firefox、Safari 测的是本地产物；线上产物的岛交互只由 M7 的线上浏览器任务覆盖（每轮 4 项，包括 Button 演示与源码）。这不改变本轮代码结论。如果需要原生浏览器直接覆盖线上 wasm，可以在三款桌面浏览器各打开一次线上组件演示做抽查。
2. **本轮没有重跑全量回归、正式性能、原生浏览器、VoiceOver 或线上回滚。** 这些都按 M7 记录与线上 manifest 抽查认定，见上表。本轮实际执行的范围见下节。
3. **本轮临时证据只存在当前机器。** Playwright 输出和归档复现位于会话 scratchpad（`/private/tmp/claude-501/…/scratchpad/`），没有提交共享。
4. **测试环境做过一处清理。** 官方站服务脚本第一次启动失败，原因是 `.git/worktrees/site` 指向 mbx 缓存里已被清除的旧临时 worktree。本轮只删除了这一条登记（等价于对该条执行 `git worktree prune`；它的 HEAD 是 `3dbb079a`，没有分支或锁）。另外 5 条同类残留没有动。这属于既有 `serve.sh` 在 mbx 清理 target 后的环境问题，不在本次 diff 内，不作为 finding。

## 覆盖与验证

### 意图与 Git 范围

用户请求为 `/review-code docs/plan/DOCS-SSG-P4.md`。计划已有 R1 代码评审，所以按开发计划模式做复审：修复差异从 R1 的目标 `06be2ee3` 算到 HEAD，并把 R1 之后登记的 M6 UI 修复和全部 M7 提交纳入新增问题检查。

区间 `06be2ee3..4894d4cc` 共 34 个提交，全部是 HEAD 的祖先。其中 14 个只改计划或 SDLC，是进度回写，排除在代码审查外：`a6c076a7`、`93c4d101`、`3ddc49c0`、`94040241`、`12e332b5`、`59762e29`、`207dd6d2`、`d19af882`、`0d9eedc7`、`bd101e21`、`ab37565a`、`f5072318`、`709ce8b9`、`4894d4cc`。其余提交都登记在完成记录的 M6 或 M7 行：

| 类别 | 提交 | 产品/测试改动 |
| --- | --- | --- |
| UI 修复（M6） | `740610db`、配对 `1bbc54b0` | search.js 焦点保留与单复数计数，labels 的 `count_one`，docs.css 未译底色与对话框宽度，search.spec |
| 锚点修复（M7） | `78f54b62`、配对 `a31509aa` | site.js 用 CSSOM 与 ResizeObserver 同步 `--rd-header`，DESIGN，official.spec |
| 汉字词段修复（M7） | `0fd34eda`（R0）、配对 `3dbb079a`（C0） | search-worker.js 的 `wordLike`，search-worker.spec |
| 发布配对 | `b3f30fbd`（历史 0.1.30）、`1146b7d8`（C1 0.1.31） | 五个配对文件 |
| Pages 修复 | `5388cc11`、`95b270c2` | docs-pages.yml 自建 GNU tar 并在上传前校验，a11y.ts 的线上来源记录，official.spec 的 404 文本 |
| 证据与文档 | `600efef4`、`3c38da04`、`a5fc4a29`、`455527ad`、`a46ad4da`、`918be79b`、`c0611fde`、`9deda1c2`、`495f5259`、`f88a4fe6` | 记录、PRD D17/D18、README、作者参考 |

净差异中有 18 个产品、测试、配置和说明文件（223 行增、64 行删），全部逐行审读：`.github/workflows/docs-pages.yml`、`Cargo.lock`、`DESIGN.docs-ssg.md`、`README.md`、`crates/rustify-docs-cli/{Cargo.toml,assets/components-Cargo.lock,assets/framework-revision.txt}`、`crates/rustify-docs/assets/{docs.css,search-worker.js,search.js,site.js}`、`crates/rustify-docs/src/labels.rs`、`docs/PRD-DOCS-SSG.md`、`docs/docs-ssg.md`、`tests/docs-ssg/{a11y.ts,official.spec.ts,search-worker.spec.ts,search.spec.ts}`。验证和截图附件只按记录摘要核对，没有逐个重审。

工作树：index 为空（SHA-256 `e3b0c442…b855`）；tracked patch SHA-256 为 `a471dc47e40928e1f09caadcf6a7eac4cf73031cdac26df39aa4015ba67a79ec`。工作树指纹的算法：对 `git ls-files --modified --others --exclude-standard` 列出的 839 个文件，取 `path → SHA-256(bytes)` 映射，按键排序后序列化为紧凑 JSON，再对其 UTF-8 字节取 SHA-256。这些文件只涉及 SDLC、评审报告、UI 采集和计划附属文档，没有产品代码。报告写入前重新核对过 HEAD 和计划 SHA-256，没有外部变化。

### 已审行为链

- **结果计数与“显示更多”**（`search.js`）：`more` 从原生 `disabled` 改为 `aria-disabled`。核对了 `next()` 的守卫，以及 `clear()` 和结果到达时 `hidden` 与 `aria-disabled` 同步：按钮可见时不会残留 `true`。shell 渲染的 more 按钮没有 `disabled` 属性（`shell.rs:835`），CSS 也没有对应的 `:disabled` 规则。焦点在 `hidden` 生效前读取，避免了原来依赖浏览器焦点修正时机的竞态。`count_one` 经 serde 展平成 `search.count_one` 键，由 `shell.rs:814` 的 `search.` 前缀过滤带进对话框数据；第三语言可以覆盖它，缺键走 P3 的英文回退与诊断。`PluralRules` 用 `count` 的实际语言选择。
- **表头高度**（`site.js`）：表头栏的 `min-height` 是常量 3.5rem，不读 `--rd-header`，所以 ResizeObserver 不会形成反馈循环。adopted sheet 不在任何 layer 里，能覆盖 components layer 里的默认 3.5rem。禁用脚本时保留默认值。constructable stylesheet 符合 PRD AC-4/AC-6 的“最新稳定版”矩阵，不需要为旧引擎降级。
- **Worker 词段**（`search-worker.js`）：查询分词和候选校验都改用 `wordLike`，与计划 §2.2 修订后的文字一致。`han` 是 `\p{Script=Han}`，不包括中文标点。英文 token 的判定不受影响。
- **Pages 归档**（`docs-pages.yml`）：checkout 的是候选 C，所以归档步骤 `runpy` 调用的是 C 自己的 `docs-pages-verify.py`。该脚本自 `fecc230c` 起没有改过，C0 和 C1 都有 `check_artifact`。归档拒绝符号链接；硬链接成员不是常规文件，会被拒绝（失败即停，不会部署）。产物名 `github-pages` 和单一 `artifact.tar` 与 `deploy-pages@v4` 的约定一致；线上 verifier 的 `workflow_artifact` 能解析同一布局。
- **测试与文档**：`a11y.ts` 的线上来源记录要求 build_id、run_id 和 40 位 C 三者都有；official.spec 的 404 正则仍要求 URL 在预期集合内。README 和作者参考的 C1、R0、C0 SHA 与版本号都与配对文件一致（R0→C1、R0→C0 只改五个配对文件和文档）。只有回滚提示与二进制不一致，见 F-01。

### 本轮实际执行

| 命令/检查 | 目标与结果 |
| --- | --- |
| `mbx test --locked -p rustify-docs --lib labels::` | 当前树，16/16 通过 |
| `mbx test --locked -p rustify-docs --lib shell::` | 当前树，12/12 通过 |
| `python3 -m unittest discover -s tests/docs-ssg -p test_pages_verify.py` | 5/5 通过；只用本地隔离环境 |
| 归档步骤复现 | 从 HEAD 的 workflow 原样抽出 Python 块，在 scratchpad 里对本地 C0 正式产物副本（`.rustify-docs/p4-public-g676xxnz/archive-test/…/dist`，build `7e1e5c9e7e7392df`）执行，`identity.json` 按该 manifest 合成。结果：97 个文件，含 `./.nojekyll`；`check_artifact` 通过；文件模式只有 0644 和 0755；manifest SHA `9c34a28c…` 与 M7 记录一致 |
| Playwright `docs-search-subpath`：`search-worker.spec.ts search.spec.ts` | `RUSTIFY_DOCS_BIN` 指向公开安装的 C1（0.1.31，source `1146b7d8`）；30/30 通过（44.9s）。覆盖汉字 `isWordLike=false` 兼容、末批焦点回到输入框、单复数计数、未译提示的两种主题底色、对话框宽度，以及原有恢复与安全用例 |
| Playwright `official-subpath`：`official.spec.ts -g "section targets stay below\|deep not-found retains"` | 同一 C1 二进制；服务脚本用 `docs-pages-prepare` 准备 C1，构建出 build `7e1e5c9e7e7392df`（96 条加 manifest，与 M7 记录的本地 C1 一致）；2/2 通过（1.9m） |
| 线上 manifest 抽查 | 只读 `GET https://rockie.github.io/rustify-ui/rustify-docs-manifest.json` 返回 200，身份与 M7 最终 C1 一致（见上表）；与本地产物的差异见待核实 1 |
| C1 二进制 `--help` | 实测输出 0.1.3 回滚配方，见 F-01 |

浏览器执行用了以下一次性环境，没有安装依赖，也没有改工具链配置。Git remap 只用于本地固定来源的消费，不作为公共 HTTPS 安装的证明：

```sh
MISE_AUTO_INSTALL=false \
RUSTIFY_DOCS_BIN=$PWD/.rustify-docs/p4-public-g676xxnz/install/bin/rustify-docs \
CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0=url.file://$PWD.insteadOf GIT_CONFIG_VALUE_0=https://github.com/rockie/rustify-ui \
mise exec python@3.14.0 -- node node_modules/@playwright/test/cli.js test -c playwright.docs.config.ts --project=<project> <spec> ...
```

两个项目串行执行，结束后 4199 和 4205 端口均已释放，仓库里没有新增文件。没有重跑整仓 build、Clippy、其他浏览器项目、正式性能或线上回滚，也没有推送、dispatch 或改动 Pages。

## 计划符合性

本轮只对 R1 之后登记的实现作代码裁定，不重新设计计划，也不回写进度。

| 本期承诺 | 实现与证据 | 本轮判定 |
| --- | --- | --- |
| M6 UI 修复（UI R1 的四项） | `740610db`/`1bbc54b0`；search.spec 的焦点、计数、底色、宽度用例本轮通过 | 已核实 |
| M7 / V-11：原生发现的锚点与汉字词段 | `78f54b62`、`0fd34eda`；official 锚点用例与 Worker 汉字用例本轮通过；原生三浏览器结果按 M7 记录 | 代码已核实；原生证据的产物范围见待核实 1 |
| M7 / V-12：固定 Git 安装、Pages 发布、回滚与恢复 | `5388cc11`/`95b270c2` 的归档在本轮复现中通过；线上 manifest 与最终 C1 记录一致；四轮 run 按 M7 记录 | 已核实（线上回滚没有重做） |
| M7：README/作者参考同步公开安装与回滚 | `f88a4fe6`；SHA 和版本一致 | 存在缺陷：F-01（P3），工具内回滚提示与文档不一致 |
| C-5/D17/D18 | PRD 与计划同步了手机取消和 VoiceOver 本人通过 | 用户决定，按记录认定，不视为技术实测 |

## 总体结论与修复顺序

**验收判定：通过验收**（R2 范围：`06be2ee3..4894d4cc` 与当前工作树；仍有 1 条 P3）。

正确性结论：`patch is correct`。M6 UI 修复和 M7 的全部产品、工作流改动中，没有发现影响正确性或阻塞验收的缺陷。F-01 是低优先级的发布文案和工具提示不一致，不影响当前线上站点的读者功能。

修复效果结论：**已解决**。R1 没有 findings；它的 4 条待核实项中，已解决 2 条（1a、2，后者依记录），有依据不再适用 3 条（1b、3、4），相关回归没有阻塞。

覆盖限制：原生浏览器、VoiceOver 和线上回滚按 M7 记录认定，本轮只抽查了线上 manifest；原生证据针对的是本地产物，不是 CI 构建的 wasm（待核实 1）。本结论不是 PRD 整期验收、合并或发布授权。

修复顺序：F-01 可以随下一次 CLI 配对一起处理，先改常量和模板占位，再补 help 与 `new` 输出的断言。
