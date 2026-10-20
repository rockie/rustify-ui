# DOCS-SSG-P4-gaps · PRD 交付差异复审（P4 链 R2）

## 1. 结论与范围

**G-01、G-02 均已解决，D-01 的验收边界在线上全部成立。本轮新增 A 0、B 0、C 0，待决 0，无需新的 gaps plan。** 原报告的 U-01、U-02 仍待核实（2 项，均不阻塞）；C2 没有改动岛 Wasm，所以两项范围不变。另有三件不属于 PRD 差异的事项，见 §5：代码复审 F-01 仍存在；公共 `main` 上的 `verify` 工作流有三个 job 失败，且在本计划之前就已失败；计划回填提交 `b807af26` 尚未推送。本结论只关闭 G-01、G-02，不关闭整份 PRD 的终验。

| 项目 | 本轮锚点 |
| --- | --- |
| 输入 / 需求来源 | [DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md)：Ready，实施 3/3；[PRD](../PRD-DOCS-SSG.md) **P4（第 4 期）**；补齐[原计划 DOCS-SSG-P4](DOCS-SSG-P4.md)（7/7） |
| 上轮报告 / 版本 | [P4 差异核查](DOCS-SSG-P4.prd-dev-gaps.md)，SHA-256 `f2c4d814059d971ac88704b14cf7df56321470cac858735cd4b085e69997ade6`（与登记时相同）；对象 `9d78cba7-ec90-4bf9-bc5e-372602fbb9c7` |
| 类型 / 本轮范围 | **修复效果复审**：G-01、G-02/D-01 的全部关闭条件；直接受影响的回归，包括 F12/AC-1、AC-2、AC-4，F16/AC-1、AC-3、AC-4，N1/AC-3，N7，D16 安装与说明；U-01、U-02 和 F-01 只核对当前状态。原报告其余账本行标“本轮未复核”，不是全 PRD 重审 |
| 前后版本 | 线上 C1 `1146b7d8`（CLI 0.1.31，build `9e8ea80d57750509`）→ C2 `88566ba1ed08223a5834f794840fff5e48f4ba8a`（CLI 0.1.32，build `f74f4a4ece90f08a`），框架始终为 R0 `0fd34eda`；内容提交 `53087eb7`，说明提交 `c6d49c97` |
| 已有处置 | [M1](DOCS-SSG-P4-gaps.records/M1.md)、[M2](DOCS-SSG-P4-gaps.records/M2.md)、[M3](DOCS-SSG-P4-gaps.records/M3.md)；[计划评审](DOCS-SSG-P4-gaps.review.md)（2×P2 已处置）；[代码评审 R1](DOCS-SSG-P4-gaps.code-review.md)：通过验收，findings 0 |
| 本轮身份 / 基线 | 对象 `9f3f860f-ac41-4221-9180-d6835d8f3757`；2026-10-20 +08:00；`main` / `b807af2672ac434f5039f8d727a912a6332e2f43` / dirty。dirty 文件只在 `.xgent-ai/` 和 `docs/plan/` 下；`c6d49c97..b807af26` 只改了计划、M3 记录和读回证据，产品路径与公共 `main` 一致 |

呈现路径：构建期 Rust 工具链 + DOM 静态 HTML + 同源搜索与局部交互岛，不涉及 GPU。原 PRD、计划、完成记录、代码和上轮报告都保持只读。

**本次实测**（只读 GET 公共站点与 GitHub API，经本机代理，2026-10-20）：

1. 线上 manifest：`cli_version` 0.1.32，`build_id` `f74f4a4ece90f08a`，`base` `/rustify-ui/`；文件清单中没有 `.md` 或 `llms-full`。
2. 两份 llms.txt 各 37 条，分组仍是 12 份指南、24 个组件、1 个首页，37 条描述都是一句话：
   - en 的 12 份指南逐字等于源 frontmatter；
   - zh 的 11 个缺译回退沿用同一句，标题带“（尚未翻译，内容语言：en）”；
   - 中文快速开始用自己的中文描述。
3. 23 个指南页（12 en + 11 zh 回退）各有 3 个 meta 描述（description、og、twitter），都等于源 frontmatter，页面上的“最后更新”为 2026-10-20；`sitemap.xml` 中恰好 12 个 `lastmod` 为 2026-10-20。
4. 线上与 C1 线上 manifest 对比：基线是 [c0-browser-manifest-before.json](../validation/docs-ssg-p4/m7/publication-20261019/c0-browser-manifest-before.json)，build `9e8ea80d57750509`，与 C1 线上产物相同。两边 96 条路径相同，只有 26 个文件变了：23 个指南页、两份 llms.txt 和 sitemap。`assets/`（岛 Wasm/glue、搜索、CSS/JS）、48 个组件页和首页逐字节不变。
5. 公共 Git：`git ls-remote` 显示公共 `main` 为 `c6d49c97`；本地 `b807af26` 比它多一个提交。
6. 源与文档：12 份指南的描述都符合计划 §5.1 的规则（单句、以 `.` 结尾、≤200 字符、YAML 中没有 `: ` 或 ` #`）。逐份对照各节标题，描述与页面主题相符。`rg` 在 `docs/`、`README.md`、`DESIGN.docs-ssg.md` 中只在计划和报告的历史引文里找到旧说法。
7. CI：`verify` 在 `4894d4cc`、`b28dcafd`、`c6d49c97` 三次推送上失败的 job 相同（详见 §5）。docs-pages run 39388531674 成功。
8. 查看了 M3 保存的 C2 GitHub 预览实拍（[components](../validation/docs-ssg-p4-gaps/m3/github-components.png)）：frontmatter 显示为单行表格，H1 和正文可读。

本轮没有运行 Rust 测试、仓内 Playwright 项目、本机构建、公开安装、原生浏览器或回滚。这些检查按完成记录及[代码评审 R1](DOCS-SSG-P4-gaps.code-review.md#33-本轮实际执行)对 `c6d49c97` 快照的实跑结果认定：源测试 2 passed，catalog 无漂移，组件测试 9 passed，线上浏览器 4 passed。实测输出只在会话临时目录，没有入库。

### 上轮问题复审

| 来源报告 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [P4 报告](DOCS-SSG-P4.prd-dev-gaps.md) / G-01 | B：D17 取消手机实机，但 `docs/docs-ssg.md` 仍写“P4 retains the full browser matrix”，`DESIGN.docs-ssg.md` L400 仍把手机等列为 M7 门槛；要求同步，不改验收结论 | gaps M1 §5.2、V-3，内容提交 `53087eb7`，经 `c6d49c97` 到公共 `main` | **已解决** | 静态：[docs-ssg.md L1487–1490](../docs-ssg.md) 写明桌面 Chrome、Firefox、Safari 最新稳定版已验证；按维护者决定没有做手机实机，不宣称任何手机浏览器已验证；VoiceOver 由维护者本人验收。[DESIGN L400](../../DESIGN.docs-ssg.md) 写明原生三款（含 200%）和公开发布已验证，没有测手机，VoiceOver 由本人验收，并链接 M7 记录。两处均与 PRD D17（L8、L806）和 D18（L9、L807）一致；全仓文档中旧说法只在历史计划和报告里出现 | 无；B 分类与 D17/D18 保留 |
| P4 报告 / G-02 | C（已决 D-01）：12 份指南在 llms.txt 中的描述多为 2–6 句，Quick start、Architecture 两条与主题无关 | gaps M1–M3：frontmatter 描述 → C2 配对 → run 39388531674 上线 → V-6–V-10 | **已解决** | 实测：线上 en、zh 两份 llms.txt 的指南描述各 12 条，都是一句说明主题的话，逐字等于源；23 页 meta 同源。记录：GitHub 预览可读（V-6，另看了实拍），catalog `--check` 通过（V-2、代码评审 R1），同 run 97 文件（V-7）。静态：C2 只改配对文件，R0 不变，`ai.rs` 的描述语义没有改 | 无；原 C 分类与 D-01 保留，不改记为“原本已满足” |
| P4 报告 / U-01 | 待核实：三款原生浏览器测的是本机产物；线上 CI 产物的岛 Wasm 不同，只有 Chromium 测过 | 未纳入 gaps plan（§0.5） | **待核实** | 实测：C2 线上 `assets/` 与 C1 线上逐字节相同，所以原生浏览器依然没有在线上 CI 的 Wasm 上实测过，缺口范围不变 | 可选补证，方法同原报告 §5；不阻塞 |
| P4 报告 / U-02 | 待核实：B_P2 的岛输入最后测于 a31509aa | 未纳入 | **待核实** | 记录 V-4：本机 C2 与本机 C1 的岛资产逐字节相同；实测：线上岛资产也未变。C2 没有使已有测量失效，也没有补测 | 可选补证；不阻塞 |
| P4 报告 §5 / 代码复审 R2 F-01（非 PRD） | CLI `--help` 和 `new` 生成的 README 以 0.1.3 为回滚目标 | gaps §0.5 排除，未获用户确认纳入 | **仍存在** | 静态：`crates/rustify-docs-cli/src/main.rs:33` `PREVIOUS_VERSION = "0.1.3"`，`templates/content/README.md:22`。作者参考 [L81–83](../docs-ssg.md) 已写明内置帮助仍是旧的 0.1.3，本次回滚应使用 0.1.31 | 由代码评审承接；需要新的 C 才能发布，不是 PRD 差异 |
| P4 报告 §3 其余账本行 | 已满足 / 已满足（记录） / 已确认变更 | — | **本轮未复核** | 只核对了它们是否受 C2 影响：岛、搜索、CSS/JS 和组件页的字节都没有变（V-4 记录，加上本次线上 manifest 对比），原报告的证据仍然适用 | 沿用原报告 |

旧问题：G-01、G-02 已解决；U-01、U-02 仍待核实，范围不变；F-01 仍存在，但不是 PRD 差异。修复没有引入新的 PRD 差异。未覆盖范围：原报告中与本增量无关的 F9、N1/AC-1/2/4/5、N3/AC-1/2/3/5、N5/AC-1/3/5/6、N4、N6 各行，本轮都没有重新验证。

## 2. 三类差异

### A · 明确遗漏（本轮新增 0）

没有发现。D-01 的验收边界是：两份 llms.txt 中每份指南一句说明主题的话、meta 同源、GitHub 预览可读、catalog `--check` 通过、经 R/C 配对与 docs-pages 发布并做线上全量核验。这几项都有对应的实测或记录证据，见下文。

### B · 合理变更（本轮新增 0）

没有新增。实施期有一处偏差，不改变需求：data 指南的定稿描述写了“Tables and trees larger than the screen”，但正文“What is not here”节写明没有虚拟化的树，所以改为只说表格（[M1 记录](DOCS-SSG-P4-gaps.records/M1.md)“偏差”），规则不变，计划 §5.1 已同步。这次改动让描述更贴合页面主题，不构成 PRD 差异。

### C · 需 Triage 的实现差异（本轮新增 0，待决 0）

没有新增。

### G-01、G-02 的关闭依据

| 关闭条件（来源） | 证据 | 等级 |
| --- | --- | --- |
| G-01：作者参考的 P4 浏览器范围与 D17/D18 一致（原报告 §2 G-01，计划 §5.2） | [docs-ssg.md L1484–1490](../docs-ssg.md)：P1 那句和引擎模拟说明保留，P4 句改为桌面三款稳定版、没有手机实机、VoiceOver 本人验收 | 静态 |
| G-01：DESIGN 不再把手机、VoiceOver 写成待通过的 M7 门槛 | [DESIGN L400](../../DESIGN.docs-ssg.md) 改为已完成的结论，链接 [M7 记录](DOCS-SSG-P4.records/M7.md)，并保留 `search-text-200.png` 是 text/layout emulation 的说明 | 静态 |
| G-01：不出现“手机已通过”或“仍待 M7” | `rg "P4 retains the full browser matrix\|remain M[0-9] gates\|phones passed"`：`docs/`、`README.md`、`DESIGN.docs-ssg.md` 中只在计划和报告的历史引文里出现 | 实测（本地 grep） |
| G-01：同步内容已公开 | 公共 `main` 为 `c6d49c97`，包含内容提交 `53087eb7` | 实测 |
| G-02/D-01：两份 llms.txt 中每份指南都是一句说明主题的话 | 线上 en 12/12、zh 11/11 回退等于源 frontmatter，中文快速开始用自己的中文描述；37 条全部是一句话；各描述与对应页面的节标题对得上 | 实测 + 静态对读 |
| G-02/D-01：页面 meta 描述同源更新 | 线上 23 页 × 3 个 meta 全部等于源；首页、组件页、中文快速开始逐字节不变 | 实测 |
| G-02/D-01：GitHub 预览仍可读（C-4、A-1） | V-6：三份指南的 frontmatter 都显示为表格，H1、正文、代码块和 catalogue 表正常；本轮另看了 components 实拍 | 记录 + 本次核看 |
| G-02/D-01：catalog `--check` 通过 | V-2 和代码评审 R1 对 `c6d49c97` 实跑：`catalog: no drift (24 rows)`，组件测试 9 passed；之后 `docs/components.md` 没有改动 | 记录（评审实跑） |
| G-02/D-01：不改框架语义和 PRD 措辞 | C2 只改 `Cargo.lock`、CLI `Cargo.toml` 和 `docs/docs-ssg.md`，R 仍为 R0；`ai.rs` 和 PRD 没有改（PRD 哈希仍是 `ab9b1e87…`） | 记录 + 静态 |
| G-02/D-01：经 R/C 配对 → docs-pages 发布 → 线上全量核验 | V-4（C2 边界、0 error、预检 97 文件、与本机 C1 比对只有允许的 27 处变化）；V-7（run 39388531674 的 build 和 deploy 都成功，同 run 归档与线上 97 文件一致）；V-8 线上 4/4；本次线上 manifest 0.1.32/`f74f4a4e…` 及与 C1 线上的对比 | 记录 + 实测 |

D-01 的边界保持原样：显式描述只用于官方站；没有 frontmatter 的作者页面仍按 PRD 的“或正文首段摘要”取首段，并收到 `missing-description` 警告。这是用户没有选择方案 2 的直接结果，不重新打开。

## 3. 需求覆盖账本（本轮范围）

证据等级同原报告：**实测**＝本次实际验证；**记录**＝完成记录及其登记的测试与运行结果；**静态**＝本次读码或读文档；**用户**＝用户本人的验收结论。不在本表的 P4 账本行沿用[原报告 §3](DOCS-SSG-P4.prd-dev-gaps.md#3-完整需求覆盖账本)，本轮未复核。

| PRD 需求/验收及定位 | 本期范围/已批准变更 | 计划与里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| F12 行为：“一句话描述（frontmatter 描述，或正文首段摘要）”（L459；§1.4 L90、§2.1 L110） | P4；D-01 | gaps §5.1；M1 V-1，M2 V-5，M3 V-8 | M1–M3 | 实测：线上两份 llms.txt 的 37 条都是一句话，12 份指南的描述说明了主题 | 已满足（原为部分满足） | G-02 已解决 |
| F12/AC-1 列出每个内容页和有描述的定制页，分组与导航一致，链接可访问（L471） | P4（回归） | gaps V-5、V-8 | M2、M3 | 实测：en、zh 各 37 条，分组为 12/24/1，路径与 C1 相同；23 个指南页 200。记录：V-7 的 97 文件 | 已满足 | — |
| F12/AC-2 每语言一份，缺译指向默认语言并标明（L472） | P4（回归）；D13 | gaps V-5、V-8 | M2、M3 | 实测：zh 的 11 个回退指向 en route，标题带“尚未翻译，内容语言：en”，描述沿用源 | 已满足 | — |
| F12/AC-4 草稿、ai=false、无描述的定制页不出现，产物没有 Markdown（L474） | P4（回归） | gaps V-4 | M2 | 实测：manifest 中没有 `.md` 或 `llms-full`；记录：V-4 预检通过 | 已满足 | — |
| F16/AC-1 12 份指南与源文件同源，在 GitHub 上仍然可读（L561、L573；D9） | 终验期 P4（回归）；C-4 | gaps V-6 | M3 | 记录：V-6 三页实拍，A-1 成立；本次核看了 components 实拍 | 已满足 | — |
| F16/AC-3 check 零错误，子路径部署后 CSP 违规 0（L575） | P4（回归） | gaps V-4、V-7、V-8 | M2、M3 | 记录：check/build 0 error、35 warning（都是 `i18n-missing-page`，原来的 12 条 `missing-description` 已消失）；V-8 线上 4/4 含 CSP 断言。HTML 只有描述和日期变化，meta CSP 不变（V-4 比对）。本轮没有重扫 76 页 | 已满足（记录） | — |
| F16/AC-4 站内搜索、中英切换、中英各一份 llms.txt（L576） | P4 | gaps V-5、V-7、V-8 | M2、M3 | 实测：两份 llms.txt 可达且描述合格；搜索索引和 Worker 与 C1 线上逐字节相同 | 已满足 | G-02 已解决 |
| N1/AC-3 首载 gzip ≤ 100KB（L627） | 终验期 P4（回归） | gaps §7、V-4 | M2 | 记录：变化页的 gzip -9 增量最大 +44 B；C1 时最大为 40,584 B | 已满足（记录） | — |
| N3/AC-4、AC-6 浏览器范围（L676、L678）在公开文档中的陈述 | P4；**D17** | gaps §5.2、V-3 | M1 | 静态：作者参考与 DESIGN 已与 D17 一致；验收本身沿用原 M7，桌面三款原生通过 | 已满足（桌面）＋已确认变更（手机） | G-01 已解决 |
| N5/AC-4 VoiceOver（L716）在公开文档中的陈述 | P4；**D18** | gaps §5.2、V-3 | M1 | 静态：两处都写明本人验收，不冒充 Agent 采证；用户：D18 | 已满足（本人验收） | G-01 已解决 |
| N7 不泄露源文件、机器路径（L749–752） | 本期回归 | gaps V-4 | M2 | 记录：V-4 预检通过；实测：描述是静态文字，manifest 中没有 `.md` | 已满足 | — |
| D16、Q2、Q4：公开 Git 安装、部署与回滚说明（L805、L814、L816） | P4（增量） | gaps §5.4、§8；V-9、V-10 | M3 | 记录：V-9 在隔离缓存下公开安装 C2，`--build-info` 为 0.1.32、unwind、source C2；V-10 公共读回 14 项通过。实测：公共 `main` 为 `c6d49c97`。回滚目标 C1 本身已在原 M7 的四轮中发布、回滚和恢复过，C2 → C1 没有演练（计划 D4） | 已满足（记录） | — |

UAT：UAT-8（F12/AC-1–4 及“一句话描述”）本轮复核通过；UAT-11（F16/AC-1–4）中与本增量相关的部分通过；其余 UAT 沿用原报告。

## 4. Triage 与决策账本

### 待决

无。本轮没有新的 C 类差异，也没有向用户提问。

### 已决（沿用，本轮没有改变）

| 决策 ID | 关联 G-ID | 用户选择与理由 | 确认日期与来源 | 新目标/验收边界 | 跟进/延期/豁免去向 |
| --- | --- | --- | --- | --- | --- |
| D-01 | G-02 | 补写 12 份官方指南的一句话显式描述；不改框架语义，不改 PRD 措辞 | 2026-10-20，[原报告 §4](DOCS-SSG-P4.prd-dev-gaps.md#4-triage-与决策账本) | 见 §2 关闭依据 | **已交付并上线**（gaps M1–M3） |
| PRD D17 | G-01 | “手机的测试不做，其他的继续” | PRD L8、L806 | 本次 P4 不验手机实机 | 保留变更；文档同步已交付 |
| PRD D18 | —（N5/AC-4） | “继续验收其他的，VoiceOver通过了” | PRD L9、L807 | 以本人验收计为通过 | 文档已注明本人验收 |
| 计划 D4（非用户决定） | — | 发布 C2 时不做回滚演练；只在核验失败时回退 C1 | [gaps 计划 §0.2](DOCS-SSG-P4-gaps.md#02-决策表)，计划评审已覆盖 | 回滚机制沿用原 M7 四轮结果 | 不构成 PRD 差异：D16 要求的回滚目标 C1 已发布并验证过；核验没有失败，因此没有触发回退 |

## 5. 待核实项与审查基线

### 待核实（沿用原报告，均不阻塞）

| ID | 缺失的事实 | 影响的需求与结论 | 补证方式 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| U-01 | 三款原生浏览器没有在线上 CI 构建的岛 Wasm 上实测过。C2 线上 `assets/` 与 C1 线上逐字节相同，范围不变 | N3/AC-6、F16/AC-2 的原生证据只覆盖同源码的本机产物 | 在原生 Chrome、Firefox、Safari 中各打开一个线上组件演示，操作一次并复制源码 | 否 |
| U-02 | B_P2 的岛输入（Quickstart/Vellum）最后测于 a31509aa；C2 没有改岛字节 | N1/AC-4 中 B_P2 的数值 | 在当前线上或本机 C2 产物上，用原 `measure-islands.mjs` 对 B_P2 重测五冷样本 | 否 |

### 相关但不属于 PRD 差异的事项

- **代码复审 R2 F-01，仍存在**：见 §1 复审表。改它需要改 CLI 源码，再出一个新的 C 发布。由代码评审承接，入口是 `/apply-code-review docs/plan/DOCS-SSG-P4.md`。
- **公共 `main` 的 `verify` 工作流有三个 job 失败，本计划之前就已存在**：`4894d4cc`（run 39358665772）、`b28dcafd`（39388461649）、`c6d49c97`（39392878084）失败的 job 相同。`verify` 最近一次成功是 2026-10-03 的 `6a4dd1e1`。
  - `docs site`：`tests/docs-ssg/serve.sh: line 62: rg: command not found`，CI runner 上没有 ripgrep，`4894d4cc` 与 `c6d49c97` 的原因相同；
  - `host checks`：`xtask` 的 `css::tests::published_candidates_match_the_sdk_sources` 失败（`xtask/src/css.rs:341`）；
  - `build (theme-studio)`：“Independent exported theme consumers”一步失败。

  PRD 中的 CI 角色只要求作者的 CI 运行 `check` 和 `build`（L122、L133），这一点由 docs-pages 工作流承担，run 39388531674 成功。所以这三项是仓库自身的回归门禁问题，不是 P4 的 PRD 差异；但 `docs site` 失败意味着推送后 CI 不再重跑官方站浏览器回归，目前只能靠本地和线上核验。M3 记录已注明这些失败与本计划无关。如需处理，建议另行立项修复，例如在 CI 中安装 ripgrep，或把 `serve.sh` 中的 `rg -q` 换成 `grep -q`。
- **计划回填提交 `b807af26` 只在本地**：用户授权的推送只到说明提交为止。它只改了计划和证据，公开说明已在 `c6d49c97` 中交付，不影响 PRD 结论。是否推送由用户决定。

### 审查基线

| 项 | 版本/状态 |
| --- | --- |
| PRD | `docs/PRD-DOCS-SSG.md`，SHA-256 `ab9b1e870b42337a471da1a57ecabe85934b6a7124ae6b4f5e7ea3aa698fb584`（最后提交 a46ad4da，与上轮相同） |
| gaps 计划 | `docs/plan/DOCS-SSG-P4-gaps.md`，SHA-256 `830f56edc7e1c150fb442a5e9a19b38318a3b3b8159ead8ed08af4d1be416b56`，3/3 |
| gaps 完成记录 | M1 `fcc3fbf6f627`，M2 `e303f6058b50`，M3 `249e343c4fdd`（SHA-256 前缀） |
| 原计划 | `docs/plan/DOCS-SSG-P4.md` `52d138436b01…`（与上轮相同，7/7） |
| 关联评审 | 上轮报告 `f2c4d814059d…`；代码评审 R1 `f7aae7c8391e…`（通过验收） |
| 代码 | `main` / `b807af26` / dirty（只有 SDLC、评审与采集文件）；公共 `main` 为 `c6d49c97`；C2 `88566ba1`、内容提交 `53087eb7` 是 HEAD 的祖先 |
| 线上 | build `f74f4a4ece90f08a`，CLI 0.1.32，96 个文件加 manifest；只读 GET，2026-10-20 |
| 本次实测环境 | macOS 27.0.1，Python 3.9（`-I`，标准库 `urllib`），curl，gh；经本机代理；输出只在会话临时目录 |
| 证据等级 | 线上 llms、meta、sitemap、manifest 对比、公共 `main` 与 CI 状态为本次实测；文档与源码对读为静态；构建、预检、安装、浏览器与 GitHub 预览按完成记录和代码评审 R1 的实跑认定 |

## 6. Follow-up 与 gaps plan

| G-ID / D-ID / PRD ID | 确定的增量工作 | 验收条件 | 依赖/阻塞 | gaps plan 任务或排除去向 |
| --- | --- | --- | --- | --- |
| G-01 / D17、D18 | 无（已解决） | — | — | [DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md) M1 已交付 |
| G-02 / D-01 / F12、F16/AC-4 | 无（已解决） | — | — | DOCS-SSG-P4-gaps M1–M3 已交付并上线 |
| U-01、U-02 | 可选补证，不是已确定的修复 | 见 §5 | 无 | 不立项，除非用户要求 |
| F-01（非 PRD） | CLI 回滚提示 0.1.3 | 见代码复审 R2 | 新的 C 配对 | 由代码评审承接 |
| `verify` 三个 job 失败（非 PRD） | 修复仓库 CI 门禁 | `verify` 在 `main` 上恢复成功 | 无 | 不属于本 PRD 覆盖核查；需要时另行立项 |

**无需新的 gaps plan**：所有已确定的 follow-up 都已由 DOCS-SSG-P4-gaps 交付；剩余的是可选补证和非 PRD 事项，都不是本期 PRD 的已确定增量。本轮没有调用 dev-plan。
