# DOM-COMPONENTS-RELEASE · 开发计划评审报告 R1

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md) | sha256 前 16 位 `106cfa0ab5caa1ef`（未入 Git，464 行） | 全文（§0–§12、实施进度、M1–M4、V-1–V-13） |

评审方式：review-dev-plan 四视角（facts / architecture / schedule / delivery）各由独立只读 subagent 评审，主 agent 交叉复核全部候选问题的原始证据后裁定；一处 reviewer 候选问题经复核撤回（见覆盖摘要）。

---

## 1. 确认问题

1. [P1] F-01 · §5.3 D 排他清单遗漏 `docs/zh/quickstart.md` 与多处「发布后即失真」的说明行，按计划字面必然上线
   - 定位：计划 §5.3 第 2 条（「只改下列说明，不碰库输入」的逐行枚举）与 §1.2 D 行；关联 R-1、C-1
   - 问题：D 的改动清单是排他性规范文本，但枚举漏掉整篇中文 quickstart 和 5 个已列文件中的失真行。0.3.0 发布后这些文本立即变为假话或直接导致用户编译失败，最早要到 M4 的 V-13 才可能暴露，而修复需要 D′＋重新部署的最贵回环。
   - 证据（逐行亲核）：
     - `docs/zh/quickstart.md:153-157`：两个 SDK 依赖钉 Git rev `1b81de25…`（2026-10-12，早于全部 P1–P5 成果），并让读者复制当前 consumer（相对链接 `../../tests/dom-only/consumer/`）后把依赖换到旧 rev——当前 consumer 含 `menu_benchmark` 等新 API，旧 rev SDK 缺这些 API，复制＋换依赖即编译失败；全篇不提 0.3.0，中文读者无从知道新能力已可用（R-1 对中文受众不成立）。清单中无任何 zh 文件。
     - `docs/quickstart.md:232`：fixture 永久链接钉在 0.2.1 的 R（`tree/b32844a4…/tests/dom-only/consumer/`），读者复制旧 fixture 源码再按指引换 `=0.3.0` 依赖——旧 fixture 的 `Tooltip` children API 在 0.3.0 已改 `trigger=`，编译失败。清单只列 238–306、406。
     - `docs/themes.md:372-374`：「The first registry release that includes v2 has not been decided yet」——0.3.0 发布即假，且正是 C-1「最低消费者版本」主题本体；清单只列 8、236。
     - `docs/dom-components-migration.md:6、155、466`：`466`「The source-only `rustify-colors` crate」（该包上架即假）、`155`「source checkout adds these DOM families」、`6`「describes the current source checkout」框架句；清单只列 7、12、241、512。
     - `docs/docs-ssg.md:333、740、792-793`：「paired CLI 0.2.1」、示例 `--version 0.2.1`、「The 0.2.1 components template…all at `=0.2.1`」；清单只更新「Install and maintain the CLI」安装节。
     - `docs/quickstart.md:405`、`docs/panels.md:9`：与已列行（406、8）同一句话的另一半，单改半句会留下自相矛盾。
   - 修订：§5.3 D 清单补入上述文件与行（zh quickstart 的 rev 钉与示例来源改为 0.3.0 事实——registry `=0.3.0` 或新标签）；同时把行号枚举兜底为规则：「以 `0.2.1`、`source checkout`、`source-only`、`nine published`、`b32844a4`、`1b81de25` 为关键词对 `README.md`、`docs/`（含 `docs/zh`）、`site/` 全量 grep，逐处改为 0.3.0 后事实」；§1.2 D 行同步。
   - 复核：改稿后按上述关键词 grep 应零残留（`0.2.1` 仅允许出现在历史/回滚语境）；zh quickstart 的依赖示例来源与 0.3.0 一致。
   - 置信度：高（失真逐行实证；两处编译失败为 API 静态 diff 推断，未实跑构建，但不改变失真结论）；阻塞：否（M3 的 D 提交前修订即可，M1/M2 可开工）

2. [P2] F-02 · §5.4 差异门把 `crates/rustify-docs-cli` 纳入 R↔D 对比，而 C 必然改动该目录，门恒失败
   - 定位：计划 §5.4 第 1 步门命令（含 `crates/rustify-docs-cli` 的 `git diff --quiet "$R" "$D" -- …`）、D4（§0.2「D 中十个库目录必须与 R 逐字节相同」）、ADR-1（「`git diff --quiet R D -- <十个库目录> …`」）；关联 R-1
   - 问题：C 位于 R 与 D 之间，其冻结产物（`crates/rustify-docs-cli/assets/{framework-release.json,components-Cargo.lock}`）在 D 中存在、在 R 中不存在，故 `git diff R D -- crates/rustify-docs-cli` 恒非空，`--quiet` 恒退出 1。按计划文本 M4 的派发门永远无法通过，实施者被迫现场重设计该门。
   - 证据：计划 §5.2「`git add crates/rustify-docs-cli/assets … && git commit # 即 C`」；`scripts/crates-io.py:725` freeze 写入 `library_rev`；实测 `git diff R D -- crates/rustify-docs-cli` 将包含 C 的资产差异（结构性必然，无需实跑）。
   - 修订：门拆两段——九个库目录＋`Cargo.toml mise.toml release/crates-io.toml` 对 `"$R"` 比较；`crates/rustify-docs-cli` 对 `"$C"` 比较；`framework-release.json` 的 `library_rev == R` 子句保留。D4 与 ADR-1 的措辞同步为「九个库目录与 R、docs-cli 与 C 逐字节相同」。
   - 复核：改稿后按新命令在 R/C/D 三个提交上 dry-run 应全部 `--quiet` 通过。
   - 置信度：高（命令语义与提交顺序直接推出，主 agent 亲核原文）；阻塞：否（M4 前修订；修订属一行命令＋两处措辞）

3. [P2] F-03 · official.spec.ts 第五处计数未列入：第 315 行 `toHaveCount(36)` 需随三篇指南改 39，§1.1「已核实·共四处」计数不符
   - 定位：计划 §1.1（「第 213 行…第 311 行…第 255–256 行…第 265 行…共四处」）与 §1.2 A 行；关联 R-3
   - 问题：`tests/docs-ssg/official.spec.ts:315` 的 `await expect(links).toHaveCount(36)`（12 指南＋24 组件页侧栏链接）未列入 A 的改动清单；加三篇后侧栏为 39 个链接，A 若只按计划改四处，V-1 首跑确定性失败。
   - 证据：`tests/docs-ssg/official.spec.ts:311-316`（断言名与 `toHaveCount(36)` 分属两行）。
   - 修订：§1.1 与 §1.2 A 行加第五处（36 → 39），「共四处」改「共五处」；A 落地时以 `toHaveLength\|toHaveCount` grep 复查 spec 内全部计数断言。
   - 复核：V-1 批内反馈命令 `-g "guide /menus/\|ordinary sidebar"` 应一次通过。
   - 置信度：高（三个视角独立发现，主 agent 亲核）；阻塞：否

4. [P2] F-04 · `release/crates-io.toml` 除新增 colors 条目外，还须给 `rustify-dom` 条目依赖加 `rustify-colors`，否则 `inspect` 直接报错
   - 定位：计划 §1.2 R 行（「新增 `rustify-colors` 条目（D2）」）与 D2 组内序说明；关联 R-2、C-2
   - 问题：`crates/rustify-dom/Cargo.toml:30` 已有 `rustify-colors` path 依赖；发行脚本强制 inventory 依赖与 manifest path 依赖一致，计划只写新增条目，实施者按清单改完 `inspect` 即红。
   - 证据：`scripts/crates-io.py:401-402`（`inventory dependencies differ from manifest` 抛错）；`crates/rustify-dom/Cargo.toml:30`；其余包无 colors 依赖（全仓 grep 仅 dom 与 colors 自身）。
   - 修订：R 行补「`rustify-dom` 条目 `dependencies` 加 `rustify-colors`（顺序按现有列表）」。
   - 复核：V-2 的 `python3.12 scripts/crates-io.py inspect` 应一次通过。
   - 置信度：高（主 agent 亲核脚本与清单）；阻塞：否

5. [P2] F-05 · 调查基线与恢复快照的全 SHA 不存在，「clean」与实际工作树不符，且 SDLC 台账已复制同错 SHA
   - 定位：计划头部「调查基线」行与「恢复快照·代码基线」两处，均写 `e2434050ae6ffb2f6bd0b2ac2c61bb5ce4bd1c8c`（main，与 origin/main 相同）· clean
   - 问题：该全 SHA 不是仓库对象（仅短前缀 8 位与真实 HEAD 巧合相同）；实际 HEAD＝origin/main＝`e243405029997be74ebc8e6683e05397fc8d8a36`，且工作树 dirty（本计划未跟踪＋SDLC 台账登记改动）。恢复快照是跨对话续做的唯一锚点，`@resume_protocol` 第一步即无法按该 SHA 核对基线；`.xgent-ai/sdlc/status/plan/478ef381….md` 的 `baseline.head` 已复制同一错误 SHA。
   - 证据：`git cat-file -t e2434050ae6f…` → 对象不存在；`git rev-parse HEAD origin/main` 均为 `e2434050299…`；`git status` 显示 dirty。
   - 修订：两处 SHA 改为 `e243405029997be74ebc8e6683e05397fc8d8a36`；「clean」改「dirty（本计划未跟踪＋台账登记；代码事实取自 HEAD）」；台账状态页 `baseline.head` 一并修正。
   - 复核：`git cat-file -t <新 SHA>` 为 commit；快照与台账一致。
   - 置信度：高；阻塞：否

6. [P2] F-06 · V-6 升级模拟发生在 D 之前，D 改写后的「从 0.2.1 升级到 0.3.0」说明与迁移步骤此后无任何检查
   - 定位：V-6（M2 执行时点、读 C 树文档）＋§5.3 D（迁移步骤与版本措辞在 D 改写）＋V-13 任务三（全新项目，非升级）；关联 R-2、C-1
   - 问题：V-6 的角色在 M2 读到的迁移文档仍写「published 0.2.1 / source checkout / do not publish a release」，计划自己把仓根文档卡点「留到 D」；但 D 改写完成后，既有 0.2.1 作者按最终文档升级的旅程没有任何检查再走一遍——V-13 任务三是全新项目，V-10/V-12 只验可用性与站点。C-1 的最终面向用户的升级说明实际处于未验证状态。
   - 证据：`docs/dom-components-migration.md:6-8`（当前措辞）；V-13 任务三命令（new --template components，无升级步骤）；V-6 执行时点（M2，D 之前）。
   - 修订：V-13 任务三加一句「顺带按线上迁移指南把该项目的依赖与 v1 主题升级到 0.3.0 语义（同时复核 V-6 记录的仓根文档卡点已消除）」，不新增门。
   - 复核：V-13 记录含升级步骤结果一行。
   - 置信度：中高（缺口确定；升级文档实际出错的概率中等）；阻塞：否

7. [P3] F-07 · §1.2 R 行把 `scripts/crates-io.py`、`scripts/docs-pages-prepare.py`、`xtask/src/docs.rs` 列为「0.2.1 常量」目标，三文件实际无 0.2.1 发行常量
   - 定位：§1.2 R 行（`crates/rustify-docs-cli/src/new.rs:431` 起那行）；关联 C-2
   - 问题：三文件仅含 wasm-bindgen `0.2.128`（依赖版本，非发行版本）；按清单逐文件找「0.2.1 → 0.3.0」目标会落空并怀疑遗漏。
   - 证据：`grep '0\.2\.1\|0\.1\.32'` 三文件仅命中 `0.2.128`（`scripts/crates-io.py:725`、`scripts/docs-pages-prepare.py:77,199-200`、`xtask/src/docs.rs:24`）。
   - 修订：从该行删除三文件，或注明「已核对无发行版本常量」。
   - 复核：改稿后该行剩余目标均可 grep 命中。
   - 置信度：高；阻塞：否

8. [P3] F-08 · C 对 CLI README 的处置三处口径不一，且「规则同 8fd7fd63」与先例不符
   - 定位：§5.1 第 2 点（「CLI README 中期望的 source C / library R 两个值留到 C 再写，规则同 `8fd7fd63`」）↔ §1.2 CLI README 行括注「R（C 只改资产）」↔ §5.2 C 提交命令 `git add … crates/rustify-docs-cli/README.md`；关联 C-2
   - 问题：三处互斥——C 到底改不改 CLI README。实际 CLI README 并无「期望 C/R」字段可填；先例 `8fd7fd63`（0.2.1 冻结提交）只改 2 个资产文件、从未动 CLI README，公开说明中的 C/R 期望值先例在 `0efbea59`（根 README 与 docs-ssg.md，随公开说明提交）。
   - 证据：`git show --stat 8fd7fd63`（仅 assets 两文件）；CLI README 现无 C/R 字段；§5.3 D 第 3 条本身把「新的 C/R」写入 docs-ssg.md。
   - 修订：统一为「C 只提交 assets；期望 C/R 值随 D 写入 docs-ssg.md 与根 README（沿 `0efbea59` 先例）」；§5.2 的 `git add` 相应收窄（或注明 README 预期无 diff）；顺带把 §5.1「library_rev 在 R 保持现值」与 runbook「置空」两种说法对齐（冻结时统一覆盖，殊途同归，仅措辞统一）。
   - 复核：改稿后全文 grep「留到 C 再写」应无残留。
   - 置信度：中高；阻塞：否

9. [P3] F-09 · runbook 的 nine/eight 措辞（×9 处）不在任何改动行，AGENTS.md「九包」无去向
   - 定位：§1.2 R 行（runbook 仅改预取行）；`docs/crates-io-release.md:3、117、191、197`（nine packages）、`:88、92、177、198-199`（eight archives/jobs）；`AGENTS.md`「crates.io 九包」
   - 问题：十包发行后 runbook 计数措辞全部过期（下次发行维护者被误导），AGENTS.md 同病；属文档漂移，无功能影响。
   - 修订：R 行的 runbook 改动扩为「预取行＋nine→ten、eight→ten（九处）」；AGENTS.md 的「九包」在 E 记录一行去向（随下次仓规维护改「十包」，或本次顺手改）。
   - 复核：`grep -c nine docs/crates-io-release.md` 归零（历史段除外）。
   - 置信度：高；阻塞：否

10. [P3] F-10 · §8 回滚（重派 C2）是一次新的 `docs-pages.yml` dispatch，是否复用 M4 授权未说明
    - 定位：§8 末行回滚命令 ↔ §11 授权门仅列「M4 Pages 派发」
    - 问题：回滚场景通常发生在异常时刻，实施者不确定能否直接执行。
    - 修订：§8 回滚行加一句「回滚派发沿用 M4 已取得的派发授权，执行后回写记录」。
    - 复核：改稿后 §8/§11 口径一致。
    - 置信度：中；阻塞：否

11. [P3] F-11 · 账本两处「设计/验收落点」单元格不完整
    - 定位：§0.1 C-3 行（落点缺 §5.3、§5.4、V-10、V-11/M4 派发门）与 R-3 行（落点写 §5.4、D5，实际设计定义含 §1.2 A 行的三处插入）
    - 问题：账本是唯一映射入口，两格需绕正文才能闭环；无验收缺口。
    - 修订：C-3 落点补「§5.2、§5.3、§5.4、§11；V-7、V-8、V-10、V-11（M4 派发门）」；R-3 落点改「§1.2（A）、D5、§5.4；V-1、V-12、V-13」。
    - 复核：映射仍双向闭合。
    - 置信度：高；阻塞：否

---

## 2. 待核实/待决策

| 项 | 缺什么证据 | 影响 | 谁可提供 / 最晚确认点 |
| --- | --- | --- | --- |
| zh quickstart 与 quickstart:232 的「编译失败」推断 | 未实跑「旧源码＋0.3.0 依赖」构建 | 不改变失真结论本身（F-01 修订已覆盖）；仅严重度佐证 | 实施者 / 无需专门验证 |
| §1.1「三篇加入后 check 80 页 / 0 error / 40 warning」 | 计划写作期自跑主张，评审未复跑 | V-1 前置即覆盖 | V-1 执行时 |
| 线上 manifest（cli_version 0.1.32、build `f74f4a4e…`、96 文件、run 39388531674）与十包 owner 逐包状态 | 网络实时核验 | 计划已安排 V-8/V-10/V-12 运行时重查；facts 视角已抽查 rustify-colors 404、九包 0.2.1、`rustify-dom` owner=rockie | 发布人 / V-8、V-10、V-12 |

无待用户拍板的决策项（A-1/A-2/A-3 均有运行时验证点与明确失败路线，不改变范围、安全、数据或外部契约）。

---

## 3. 覆盖摘要

- **计划基线**：`docs/plan/DOM-COMPONENTS-RELEASE.md`（未入 Git，sha256:106cfa0a…，464 行）；仓库 HEAD `e2434050299…`（main＝origin/main），工作树 dirty（本计划未跟踪＋SDLC 台账登记，代码事实取自 HEAD）；评审日期 2026-11-10。
- **四维覆盖**：facts / architecture / schedule / delivery 各由一个只读 subagent reviewer（模型继承主 agent）独立评审；主 agent 整理账本与全局矛盾，并亲自复核全部 admitted findings 的原始证据（§5.3 清单逐行、spec 315 行、脚本 401-402 行、SHA、grep 名、runbook 措辞等）。
- **撤回的候选问题**：architecture 视角曾报「V-12 `--grep` 引用两个不存在的用例名」（P2）。复核发现 spec 的用例名由模板生成（`guide ${route} is English…`、`CATALOG ${name} uses…`、`the ${preference} first frame…`），A 改动后 8 个 grep 名全部对应真实用例（含 `CATALOG Button uses`，Button 为 24 类之一，Playwright grep 大小写不敏感）——撤回，不进 findings。
- **脚本**：`check_paths.sh` 47 个 token、MISSING 3 个，均为计划标注的新增路径（`.rustify-docs/release-0.3.0/` 临时目录、`crates/rustify-colors/LICENSE`★、`DOM-COMPONENTS-RELEASE.records/`★），无虚构复用路径；`check_progress.sh` 0/4 里程碑、0 ERROR、1 WARN（未提交台账登记，属计划注册流程预期）。
- **证据等级**：全部为文档推演＋静态评审（计划文本、目标文档、spec、配置、只读 git 取证、crates.io 公开 API 只读查询）；新站点尚不存在（线上为 C2 时代 0.1.32 产物），无浏览器验证需求（计划评审，未实施）。

---

## 4. 映射与实施编排

**映射核对**：R-1/R-2/R-3、C-1–C-7、A-1–A-3 → 设计（§5.1–§5.4、ADR-1、D1–D9）→ 文件清单（§1.2 逐行标 A/R/C/D/E）→ M1–M4 → V-1–V-13 双向闭合，无断链 ID；D8/D9（不改/回滚目标类）落 §0.5/§8。仅两处落点单元格不完整（F-11）。

**依赖与波次**（schedule 视角核对成立，主 agent 认可；四个里程碑因冻结→上传→部署链本质串行，无可并行收益对）：

| 波次/任务 | 前置及解锁证据 | 写入边界/共享负责人 | 汇合验收 |
| --- | --- | --- | --- |
| M1·A（三篇指南进导航＋spec 计数） | 无；A 与 R 互不影响（库输入=包目录＋仓根 Cargo.toml＋mise.toml＋清单，已核 `scripts/crates-io.py` `library_inputs_changed`） | `rustify-docs.toml`＋`official.spec.ts` 单人 | V-1（批内反馈 `-g "guide /menus/\|ordinary sidebar"`） |
| M1·R（十包 0.3.0＋colors 元数据＋dom inventory 依赖＋锁＋CI 预取） | V-2 通过后提交 R | 全部 manifest/锁单人 | V-2 |
| M2（冻结收口，严格串行） | R＋预取；V-3 库部分→package-libraries→freeze→C→V-3 CLI 三行→package-cli→V-4→V-5→V-6→授权 | `DIR` 单人、不上传 | V-3–V-7 |
| M3（发布＋D＋标签） | DIR 与 V-7；V-8 授权凭证 | 凭证/registry 单人、token 不打印 | V-8–V-10 |
| M4（Pages＋真实使用验收） | D 在公共 main、V-10 通过、派发授权 | `$PREVIEW` 产物仓外 | V-11–V-13 |

**风险门**：V-7（推送＋dispatch）、V-8（上传授权，含 D/标签范围）、docs/remaining 组间公开门（V-9）、标签在 V-10 后、M4 派发授权各自独立保留，无被合并绕过；M2 携带全量回归/大矩阵属上传不可逆操作的前置风险验证，编排正当。**修订时点**：全部 findings 为一次编辑批次即可完成；其中 F-01 必须在 M3 的 D 提交前、F-02 在 M4 前、F-03/F-04 在 M1 内（V-1/V-2 即暴露，提前改省一轮红灯）。

---

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

建议状态：**修订后 Ready**（计划自报 Ready；差异即本次 11 项 findings，全部有最小修订且无一项需要用户或外部先给答案）。无 Blocked 项：A-1/A-2/A-3 均有运行时验证点与明确失败路线；三道授权门是发布流程的阶段门，不是就绪阻塞。

发布机制本体（冻结/配对/差异门/公开门/回滚/授权时序、批次结构与依赖安排、进度合同与骨架一致性）经四视角核实成立；问题集中在三处事实性枚举不全（D 清单、spec 计数、inventory 依赖）、一处门命令设计错误、一处基线锚点损坏，及若干口径不一。

修订顺序（一次编辑批次）：

1. **F-01**（外部契约/用户可见，M3 D 前必须）→ 2. **F-02**（M4 门，改一行命令＋两处措辞）→ 3. **F-03 / F-04**（M1 的 V-1/V-2 必红项，先改省一轮）→ 4. **F-05**（基线锚点＋台账）→ 5. **F-06**（V-13 加一句）→ 6. F-07–F-11（P3 随手）。

改稿后建议先跑 `check_paths.sh`/`check_progress.sh` 复核，再按 `/goal 按 docs/plan/DOM-COMPONENTS-RELEASE.md 开发` 实施。

---

## 修订处置与复核

- 来源：本报告 R1（处置前 sha256 `2351e5655e96de432f693d176e93c4a31d023035540e2ce6bf29cc2ea3aadfa8`）；原问题 ID F-01—F-11 及原结论不变。
- 目标：[DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md)；修前 sha256 `106cfa0ab5caa1ef46e227519ba6525b95bd838c50c91f6c432f279014acf631`（与所审版本相同，464 行），修后 sha256 `ce081793e65d0724e03a63371fa58ad8e4cef7470491da0142d43d863b50f76f`（468 行），均未入 Git。
- 复核日期与代码基线：2026-11-10；HEAD＝origin/main＝`e243405029997be74ebc8e6683e05397fc8d8a36`，工作树 dirty（本计划与报告未跟踪、SDLC 台账登记）；代码事实取自 HEAD，只读核对。
- 本次范围：全部 F-01—F-11 与 §2 待核实表；未重跑四视角评审。

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。亲读 `docs/zh/quickstart.md:153-157`（Git rev `1b81de25…`）、`docs/quickstart.md:232,405`、`docs/themes.md:372-374`、`docs/dom-components-migration.md:6,155,466`、`docs/docs-ssg.md:333,740,792-793`、`docs/panels.md:9`。按建议关键词全量检索另发现报告未列的 `README.md:9,262`、`docs/forms.md:12`、`docs/dom-components-migration.md:13,26`。报告原关键词 `0.2.1` 会误中 `0.2.128` 与 `site/Cargo.lock` 中第三方包的 0.2.1，检索式收窄为 `0\.2\.1([^0-9]\|$)`、`site/src`，并排除计划/验证/报告/评审目录 | 已修订并复核 | §5.3 第 2 条逐行清单补全（含 zh quickstart 改为 registry `=0.3.0`、fixture 链接改指新 R），另加关键词兜底；§1.2 D 行加 `docs/zh/quickstart.md`；V-10 加「提交 D 前关键词检索无残留」 | 当前树按新检索式检索，命中 47 行，全部落在 §5.3 已列行号内（核对到行）；D 落地后的零残留由 V-10 实施期验证 |
| F-02 | 成立。C 必改 `crates/rustify-docs-cli/assets/*`（§5.2，`8fd7fd63` 先例），R↔D 对 CLI 目录恒有差异 | 已修订并复核 | §5.4 第 1 步拆为两条：九个库目录及仓根输入对 R，`crates/rustify-docs-cli` 对 C；D4、ADR-1 措辞同步；§5.3 写明 D 不碰 `crates/rustify-docs-cli` | 文档层：门命令与 D 的改动范围一致。R/C/D 尚不存在，dry-run 留到 V-11 |
| F-03 | 成立。`tests/docs-ssg/official.spec.ts:315` `toHaveCount(36)` | 已修订并复核 | §1.1 改为五处（含标题 fifteen、36→39），并要求 A 落地时用 grep 复查；§1.2 A 行、M1 行「四处」→「五处」 | 全文已无「四处」；V-1 实跑留到 M1 |
| F-04 | 成立。`scripts/crates-io.py:401-402` 强制清单依赖与 manifest 一致；`crates/rustify-dom/Cargo.toml:30` 依赖 colors；`release/crates-io.toml:29` 只有 `rustify-style` | 已修订并复核 | §1.2 R 的 `release/crates-io.toml` 行补 dom 条目依赖 | `inspect` 实跑留到 V-2 |
| F-05 | 成立。`git cat-file -t e2434050ae6f…` 对象不存在；HEAD＝`e2434050299…`；工作树 dirty。台账计划状态页 `baseline.head` 已是正确 SHA（无需改），错误 SHA 只留在不可变的计划起草事件 `5bce2801…` 中，本次新事件给出更正 | 已修订并复核 | 计划头「调查基线」与恢复快照「代码基线」改为正确 SHA，并把 clean 改为 dirty 及原因 | 新 SHA `git cat-file -t` 为 commit；快照与台账一致 |
| F-06 | 成立。V-6 在 M2 读 C 树的旧措辞；D 改写后的升级旅程无任何检查 | 已修订并复核（偏离建议） | V-13 新增任务四：从 crates.io 0.2.1 CLI 生成站点，只凭线上迁移指南与 docs-ssg 升级到 0.3.0、换 v2 主题后 build/preview，并确认 V-6 记录的仓根卡点已消除；V-13 需求列与 C-1 落点加 C-1。偏离理由：建议把升级步骤挂在任务三上，但任务三本来就是 0.3.0 新项目，没有可升级的旧状态，只能单列一个任务；这不是新门 | 映射：C-1 → V-10、V-13 闭合 |
| F-07 | 成立。三文件只命中 wasm-bindgen `0.2.128` | 已修订并复核 | §1.2 该行移出三文件，注明已核对无发行版本常量 | 该行剩余五个目标都确有发行版本断言 |
| F-08 | 成立，另有一点修正。CLI README 只有第 23 行「Expect version 0.2.1」，R 中已改；`8fd7fd63` 只改两份资产；C/R 期望值的先例是 `0efbea59`。另外，报告说 library_rev「殊途同归」不准确：runbook 第 1 步要求 R 中不写库版本，`b32844a4` 在 R 中把 `framework-release.json` 置为 `library_rev: null`、三包 0.2.1。按计划原文「保持现值」，R 中的 CLI 会带着 0.2.1/`b32844a4` 的旧配对，`new --template components` 测试会走已冻结分支 | 已修订并复核 | §5.1 第 2 条改为：R 中 `library_rev` 置 null、三包写 0.3.0（同 `b32844a4`），C 只提交两份资产（同 `8fd7fd63`），C/R 期望值随 D 写入（同 `0efbea59`）；§1.2 新增 R 的 `framework-release.json` 行，CLI README 行改为只在 R 提交；§5.2 的 `git add` 收窄为 assets | 全文已无「留到 C 再写」「保持现值」「规则同 `8fd7fd63`」 |
| F-09 | 成立。runbook 里 nine 出现在第 3、117、191、197 行，eight 出现在第 88、92、177、198–199 行；`AGENTS.md:271` 写「九包」。修正：建议说 eight→ten，但 eight 指的是库归档、Docs 组和 docs.rs 任务，加入 colors 后应为九，十包只指 nine 那几处 | 已修订并复核（偏离建议） | §1.2 runbook 行写明 nine→ten、eight→nine 的行号；新增 R 中 `AGENTS.md` 改「十包」一行；头部开发范围同步 | 改动随 R；计数逐行核对过 |
| F-10 | 成立 | 已修订并复核 | §8 回滚行与 §11 授权行写明回滚重派沿用 M4 派发授权，执行后回写 M4 记录 | §8 与 §11 口径一致 |
| F-11 | 成立 | 已修订并复核 | §0.1 C-3 落点改为 §5.2、§5.3、§5.4、§8、§11；V-7、V-8、V-10、V-11；R-3 落点改为 §1.2（A）、D5、§5.4 | 账本双向闭合 |
| §2 待核实三行 | 均为实施期运行时项，计划已安排在 V-1、V-8、V-10、V-12，不影响文档修订 | 待核实（按计划检查点） | 无需改稿 | 留给实施 |

### 决策与未完项

没有需要用户拍板的事项。F-06、F-08、F-09 三处偏离了建议的写法，但都只是用最小改动解决原问题，没有改变范围或外部契约。AGENTS.md「九包」按报告给出的「本次顺手改」处理，放进 R。

### 修订后结论与验证

- 11 项均已修订并复核（其中 3 项偏离建议），不采纳 0 项，未完 0 项；§2 的三项仍是实施期待核实。
- 检查：`check_paths.sh` 共 50 个 token，MISSING 3 个（`.rustify-docs/release-0.3.0/`、`crates/rustify-colors/LICENSE`★、`DOM-COMPONENTS-RELEASE.records/`★），都是计划内新增路径，与评审时相同；`check_progress.sh` 0/4，0 ERROR，1 WARN（计划未提交，属预期）。另按 F-01 检索式和 F-03/F-05/F-08 的旧口径检索，没有残留。未运行构建、测试或浏览器（计划尚未实施）。
- 就绪：**Ready**。版本、包集合、门命令、冻结与说明的切换都有确定路线；A-1（凭证）只阻塞 M3，三道授权门是阶段门。这是本次修订复核的结论，不是新一轮独立评审。
- 关联文档：除 `AGENTS.md` 已作为 R 的待改项写进计划外，没有其他需要同步的文档；本次没有修改计划以外的原文。
