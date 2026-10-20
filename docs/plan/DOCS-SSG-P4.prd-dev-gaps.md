# DOCS-SSG-P4 · PRD 与开发交付差异核查

## 1. 结论与范围

**明确遗漏（A）0 项，合理变更（B）1 项，需 Triage（C）1 项，其中已决 1、待决 0。待核实 2 项，均不阻塞分类、决策或计划。需跟进 2 项：G-01 的文档同步，以及按用户决定 D-01 补写 G-02 的指南描述。两项都由 [gaps plan](DOCS-SSG-P4-gaps.md) 承接。** P4 本期的搜索、llms.txt、官方中英站和公开发布，在下文基线和证据范围内都已交付并上线。剩余差异有两类：手机实机验收按 D17 取消，但有两处公开文档没有同步；官方指南在 llms.txt 里的描述不符合 PRD 写的“一句话描述”。报告写完不等于 PRD 整期验收完成。gaps plan 的路径与状态见 §6。

审查对象：[原计划 v2](DOCS-SSG-P4.md) → [PRD](../PRD-DOCS-SSG.md) §7.1–7.2 中的 **P4（第 4 期）**，即 WORK-F9 全部 AC、WORK-F12 全部 AC、F16/AC-3–4、N1/AC-5、N3/AC-5–6、N5/AC-2、N5/AC-4。P4 也是 F16、N1、N3、N5 的终验期，所以这四条的全部 AC 都逐条复核。另外核对了以下各项：N3/AC-2、AC-4 和 WORK-N4 的每期复核；计划从 P2 承接的 F4/AC-4 补证；N6、N7 回归；D16 规定的公开安装、发布与回滚。这是 P4 的首次覆盖核查，不是复审。

输入前提成立：计划快照为 7/7，M1–M7 的完成记录都在，每行都有提交 SHA。[代码复审 R2](DOCS-SSG-P4.code-review.r2.md) 的结论是通过验收，留有一条 P3 级的 F-01（不属于 PRD 差异，见 §5）。[UI 复审 R6](DOCS-SSG-P4.ui-review.r6.md) 的结论仍是“需补证”（U4 未评分）。R6 早于 M7 的原生补验和 D17/D18，之后没有新的独立 UI 评审。

呈现路径：构建期 Rust 工具链 + DOM 静态 HTML + 按需加载的同源搜索 module/Worker + 局部交互岛。不涉及 GPU。

- 评审对象 ID：`9d78cba7-ec90-4bf9-bc5e-372602fbb9c7`
- 审查日期：2026-10-20 +08:00
- 代码：`main` / `4894d4ccc7c462a344489748553615a352031192` / dirty。dirty 文件只在 `.xgent-ai/` 和 `docs/plan/` 下（评审报告、UI 采集），`crates/ xtask/ scripts/ tests/ site/`、根配置与各指南都没有改动。
- 线上：`https://rockie.github.io/rustify-ui/`，manifest 的 build_id 为 `9e8ea80d57750509`，CLI 0.1.31，对应 C1 `1146b7d8` / R0 `0fd34eda`。

**本次实测**（只读，访问公共站点；浏览器为 Playwright 1.63.0 自带的无头 Chromium 153.0.8010.12，经本机代理）：

1. **线上全部 76 个 HTML 页面**（74 个内容/定制页，外加 en、zh 各一个深层不存在地址）：
   - CSP 违规 0，console 错误 0；
   - 失败请求 0，状态 ≥ 400 的响应 0（两个预期的 404 除外）；
   - 外源请求 0；
   - 每页都有且只有一份 meta CSP，与严格策略去掉 `frame-ancestors` 后逐字相同；
   - 每页恰好一个 h1，`lang` 正确，搜索入口可用；
   - 48 个演示页的岛全部 ready；
   - 两个深层地址返回 404，显示对应语言的 404 页。
2. **线上纯键盘走查**：
   - 跳到正文 → `#content`；
   - 用 Tab 走侧栏链接并用 Enter 打开，焦点轮廓全程可见；
   - Dark 主题、语言切换（English → 中文，到 `/zh/quickstart/`）；
   - Ctrl+K 打开搜索，Esc 后焦点回到触发按钮；
   - 查询 ThemeScope 得到“3 个结果”，Down 选中带“尚未翻译”标记的缺译结果，Enter 跳到 `/zh/architecture/#versioned-themes`，平滑滚动结束后标题 top 为 81，在页头（bottom 57）下方；
   - 演示的 Increment 让 Count 从 0 变为 1；
   - Copy 显示 Copied，剪贴板内容是源码。
3. **线上 llms.txt 与搜索索引**：
   - en、zh 两份 llms.txt 各 37 条链接，全部在已访问的 200 页中；
   - manifest 里没有 `.md` 或 `llms-full`，隐藏文件只有 `.nojekyll`；
   - 两份搜索索引各 37 页（zh 中 35 页缺译），llms 与索引里都没有本机路径。

本次没有运行 Rust 测试、仓内 Playwright 项目、正式性能、原生浏览器、VoiceOver 或线上回滚，这些按完成记录认定。原始输出只在本机会话的临时目录里，没有入库。

## 2. 三类差异

### A · 明确遗漏（0）

没有发现。逐 AC 核对见 §3。F9/F12 的行为与边界、F16 的线上交付、跨期 NFR 的终验项都已追到可观察行为，证据或来自本次线上实测，或来自完成记录中已有的测试与运行结果。

### B · 合理变更（1）

#### G-01 · 本期不做手机实机验收（D17），两处公开文档仍写着旧门槛 · B

- **PRD 期望**：WORK-N3/AC-4（[PRD L676](../PRD-DOCS-SSG.md)）原本要求每期出口在 375px 下验 iOS Safari 和 Android Chrome；AC-6（L678）和 UAT-12（L863）是 P4 终验。用户随后作出 D17（L8、§5.1 L806），取消本次 P4 的手机实机验收及其发布门槛，并已同步到 AC-4、AC-6、Q3 和 UAT-12。
- **实际交付**：最终候选没有在实体手机上验收。替代证据有：
  - 三款桌面原生浏览器在 320/375 响应式模式和原生 200% 下的走查（[M7](DOCS-SSG-P4.records/M7.md)）；
  - 固定 Chromium 153 的移动性能实验室合同（375px、CPU 4×、限速网络）。

  历史上的局部手机证据（UI R5 在旧候选上的 41 张 iPhone 图）保留原来的范围。
- **证据链**：计划 C-5、§9.3、V-11 → M7「本轮继续」 → PRD D17。
- **影响**：手机读者在真实 OS 和浏览器上的软键盘、visualViewport、触控等行为，最终候选没有实测。按用户决定，这不阻塞本期出口。
- **分类依据**：有权决策的用户明确调整了验收承诺，PRD 正文已同步，替代证据已执行，历史缺证没有改记为通过，符合 B 的条件。
- **处置**：保留该变更。需要宣称支持手机，或有设备资源时，另行补验。
- **Follow-up（已确定）**：同步两处陈述，不改变任何验收结论。
  - [docs/docs-ssg.md L1475–1479](../docs-ssg.md) 仍写“P4 retains the full browser matrix”，没有反映 D17 的实际 P4 范围（桌面三款稳定版，手机实机取消）。
  - [DESIGN.docs-ssg.md L400](../../DESIGN.docs-ssg.md) 仍写“Native stable browsers, physical phones, spoken VoiceOver, native 200% zoom and publication remain M7 gates”。这句话在 M7 完成、D17/D18 之后已经过时。
  - 验收条件：两处都与 PRD D17/D18 和 M7 记录一致，不出现“手机已通过”或“仍待 M7”的说法。

### C · 需 Triage 的实现差异（1：已决 1，待决 0）

#### G-02 · 官方站 llms.txt 里的指南描述不是“一句话”，有两条脱离上下文 · C（已决 D-01）

- **PRD 期望**：
  - WORK-F12 行为（L459）：“页面链接和一句话描述（frontmatter 描述，或正文首段摘要）”；
  - §1.4 L90、§2.1 L110 同样写“一句话描述”；
  - 背景（L457）要让工具“知道站点有哪些页面、各讲什么”；
  - F16/AC-4 与 G5 要求官方站提供 AI 输出。

  AC-1 本身只要求列出“描述”。
- **实际交付**：线上 en、zh 两份 llms.txt（build `9e8ea80d57750509`）中，12 份英文指南的描述有两种问题。
  - 10 条有 2–6 句。例如 Runtime utilities 有 6 句，Component catalogue 有 4 句，Data、Themes 各 3 句。
  - 另外 2 条虽然只有一句，但与页面主题无关：
    - Quick start 是“mbx xtask doctor reports the state of all of the above and never changes it.”，取自 Prerequisites 列表之后的第一个 `<p>`；
    - Architecture 是“Theme integration below was checked against the working tree of rockie/rustify-ui, branch dev, based on commit 7f5e504 …”。
  - zh 索引的 11 个缺译指南沿用同样的英文描述。中文快速开始、24 个组件和首页的描述都是合格的单句。
- **证据链**：
  - 计划 §5.3（[L318](DOCS-SSG-P4.md)）写的是“单句描述”，[M4](DOCS-SSG-P4.records/M4.md) 实现为“全部收敛为空白分隔的单行”，单行不等于单句；
  - [ai.rs:27 `description()`](../../crates/rustify-docs/src/ai.rs) 先取显式描述，没有时取 `page.fragment.summary`，即整段首段，不截断；
  - [作者参考 L461](../docs-ssg.md) 写明没有描述时用首段并给出警告；
  - 本次实测的两份线上文件。
- **影响**：外部 AI 工具（S8、UAT-8）从索引看不出 Quick start、Architecture 讲什么，另外 10 个指南的条目是整段文字。链接、分组、缺译标注和排除规则都正确，不阻塞任何 AC。
- **分类依据**：框架按字面来源规则工作。根因是官方指南没有显式描述，生成器也不缩短首段。要做到“一句话、讲清讲什么”，需要在三条路之间取舍：补内容、改生成语义、或改 PRD 措辞。没有任何已记录的决定接受段落式描述，所以不是 B。AC 字面已满足，修复又需要取舍，所以不是 A。
- **处置**：用户于 2026-10-20 选择补写显式描述（D-01，问题卡见 §4 Q-01）。差异的原分类保留为 C，不改记为已满足。
- **Follow-up（已确定）**：
  - 12 份官方英文指南各写一句说明页面主题的 `description`；
  - `docs/components.md` 直接加 frontmatter，生成器保留标记之上的内容，`--check` 不受影响；
  - 框架的描述语义和 PRD 措辞都不改；
  - 经既有 R/C 配对与 docs-pages 工作流上线，并做线上核验；
  - 验收条件见 §4 的 D-01 和 §6。

## 3. 完整需求覆盖账本

证据等级：**实测**＝本次实际验证；**记录**＝完成记录及其登记的测试与运行结果；**静态**＝本次读码；**用户**＝用户本人的验收结论。

| PRD 需求/验收及定位 | 本期范围/已批准变更 | 计划与里程碑 | 完成记录 | 实际行为与证据等级 | 覆盖状态 | G-ID/去向 |
| --- | --- | --- | --- | --- | --- | --- |
| F9/AC-1 千页语料中“主题切换”“ThemeScope”首位标题含该词，结果显示标题、路径、高亮摘要（L392） | P4 | §2.2；M2 V-2，M6/M7 V-10 | [M6](DOCS-SSG-P4.records/M6.md) V-10；[M7](DOCS-SSG-P4.records/M7.md) C29 | 记录：每语言 1000 页，`search-perf.spec.ts:90-92` 断言首位标题，[C29 报告](../validation/docs-ssg-p4/m7/native-20261018-resume/c29-search-performance.json.gz)通过。实测：线上 ThemeScope 的结果带路径与摘要 | 已满足 | — |
| F9/AC-2 无结果状态、换词建议、输入保持焦点（L393） | P4 | §5.2、§6；V-8 | M6、M7 | 记录：search.spec 区分 idle、results、empty；三款原生浏览器实测不存在的词得到 0 个结果并保持焦点 | 已满足 | — |
| F9/AC-3 纯键盘打开、移动、打开结果并到达标题，Esc 回到触发点（L394） | P4 | §5.2；V-8、V-11 | M7 原生三浏览器 | 实测：线上 Ctrl+K、Esc 回到触发按钮，Down、Enter 到 `#versioned-themes`，标题 top 81 > 页头 57。记录：原生 Chrome、Firefox、Safari 同样通过 | 已满足 | — |
| F9/AC-4 中文界面只搜中文界面页面，有译文用译文，缺译标“尚未翻译”并停在中文界面，草稿和 search=false 不出现（L395） | P4；D13 | §2.1；V-1、V-8 | [M1](DOCS-SSG-P4.records/M1.md) 7/7；M7 | 实测：zh 索引 37 页，其中 35 页缺译，结果带“尚未翻译，内容语言：en”，打开后 URL 仍为 `/zh/…`。记录：V-1 覆盖草稿、search=false、默认草稿的译文 | 已满足 | — |
| F9/AC-5 定制页的文字可搜，选中后到达该页（L396） | P4；D14 | §2.1；V-1、V-8 | M1、M6 | 记录：fixture 的 CustomOnlyToken 得到 1 个结果。实测：线上索引包含已注册的组件页和首页 | 已满足 | — |
| F9/AC-6 404 页可用快捷键或按钮打开搜索并到达结果（L397；§2.4 L164） | P4 | §5.1；V-3、V-8 | M3；M7 | 实测：线上两个深层 404 的搜索入口可用，语言正确。记录：三款原生浏览器在 `/zh/missing/deep/` 搜索后 Enter 到达快速开始 | 已满足 | — |
| F9/AC-7 加载失败显示暂不可用与重试，重试成功显示结果，加载期间保留输入（L398；UAT-9） | P4 | §2.3、§5.1；V-4、V-11 | [M3](DOCS-SSG-P4.records/M3.md) 15/15；M7 | 记录：module、JSON、Worker、watchdog 全部恢复用例通过；三款原生浏览器做了首次 module 404 → 同页 Retry 的故障注入 | 已满足 | — |
| F9 行为：完全离线，不请求外部服务（L386） | P4 | D1–D3；V-8 | M6 | 实测：线上 76 页外源请求 0，CSP 的 connect-src、worker-src 为 self | 已满足 | — |
| F12/AC-1 列出每个内容页和有描述的定制页，分组与导航一致，链接可访问（L471） | P4；D14 | §5.3；V-5、V-12 | [M4](DOCS-SSG-P4.records/M4.md)；M7 | 实测：en、zh 各 37 条，分为 Guides 12、Components 24、其他页面 1（首页），全部可访问 | 已满足 | 描述质量见 G-02 |
| F12/AC-2 根路径与子路径都在基路径下，链接带基路径，每语言一份，缺译指向默认语言并标明（L472） | P4；D13 | §5.3；V-8 | M6、M7 | 记录：official-root 以 `/`、subpath 以 `/rustify-ui/` 部署，`official.spec.ts:252-262` 断言两份索引；C29 的 root、subpath、meta 各 56/56。实测：线上 zh 的缺译链接指向英文 route 并带语言标注 | 已满足 | — |
| F12/AC-3 未配置站点地址时构建成功并警告，链接为带基路径的根相对地址（L473） | P4 | §5.3；V-5 | M4 | 记录：`ai.rs` 测试 `ai_root_relative_addresses…` 用 base `/docs/` 断言根相对；M4 复用既有 site-url 警告 | 已满足（记录） | — |
| F12/AC-4 草稿、ai=false、无描述的定制页不出现，产物没有 Markdown（L474） | P4；D12 | §5.3；V-5、V-12 | M4 | 记录：库测试 8/8、CLI 测试 2/2。实测：线上 manifest 没有 `.md` 或 `llms-full` | 已满足 | — |
| F12 行为：“一句话描述”（L459；§1.4、§2.1） | P4 | §5.3 写“单句描述” | M4 写“单行” | 实测：12 份指南中 10 条多句，2 条脱离上下文 | 部分满足 | **G-02（C，D-01 补写描述）** → gaps plan |
| F16/AC-1（P2）12 份指南与源文件同源，不发布计划、验证、报告和 PRD（L573） | 终验期 P4 | §5.4；V-6、V-8 | M5、M7 | 记录：C29 的 official 56/56 断言 12 份指南和配置顺序，V-6 冷源图。实测：线上没有计划类页面 | 已满足 | — |
| F16/AC-2（P2）24 个分类有可运行 DOM 演示和同源源码，不加载 Makepad（L574） | 终验期 P4 | V-6、V-8 | M5、M7 | 实测：线上 48 个演示页（24 × 2 语言）的岛全部 ready，外源 0。记录：official 断言真实函数源码、无 makepad 请求 | 已满足 | 原生浏览器只测了本地产物，见 U-01 |
| F16/AC-3 check 零错误，部署到子路径后浏览全部页面，CSP 违规 0，部署说明写明策略范围（L575） | P4；D11、Q4 | §8；V-12 | M7 四轮发布 | 实测：线上 76 页 CSP 违规 0，meta CSP 逐字正确。记录：check、build 0 error，[四轮发布](../validation/docs-ssg-p4/m7/publication-20261019/four-stage-result.json)。静态：[docs-ssg.md L1357–1363](../docs-ssg.md) 写明 GitHub Pages 只交付 meta，没有 `frame-ancestors` | 已满足 | — |
| F16/AC-4 站内搜索、中英切换、中英各一份 llms.txt，首页与快速开始有中文版（L576） | P4 | §5.4；V-6、V-12 | M5、M7 | 实测：线上 `/zh/`、`/zh/quickstart/`、两份 llms.txt、搜索、语言切换都可用 | 已满足 | 描述质量见 G-02 |
| F4/AC-4（P2 欠证）GitHub 文件预览中组件围栏可识别（L265） | 计划承接 P2 欠证 | §9.3；V-12 | M7 | 记录：[GitHub 实拍](../validation/docs-ssg-p4/m7/publication-20261019/github-markdown.png)显示三处 Counter 围栏和 CounterDemo 占位 | 已满足（记录） | — |
| N1/AC-1（P1）无交互组件的内容页 wasm 0 字节，基础交互可用（L625） | 终验期 P4 | §7；V-10 | M7 a31509aa | 记录：A、B_P2、B_P1 的 reader 报告中内容页 wasm 均为 0 | 已满足（记录） | — |
| N1/AC-2（P1）移动端 LCP ≤ 2.5s、CLS ≤ 0.1（L626） | 终验期 P4 | §9.2；V-10 | M7 a31509aa | 记录：最高中位 LCP 1184/1360/884ms，CLS 0（[A](../validation/docs-ssg-p4/m7/native-20261018/reader-A-a31509aa.json)）。C29 只改了 Worker 和岛 Wasm，site.js 与 CSS 未变，按记录复用 | 已满足（记录） | — |
| N1/AC-3（P1）首载 HTML/CSS/脚本 gzip ≤ 100KB（L627） | 终验期 P4 | §7；V-10 | M7 | 记录：最大为 40404/40584/34411B，含搜索 bootstrap | 已满足（记录） | — |
| N1/AC-4（P2）FCP → 全部演示可交互 ≤ 1s，水合位移 0（L628） | 终验期 P4 | V-10 | M7 | 记录：A 的 Data table 在 C29 上五冷中位 55.3ms；B_P2 的 Quickstart/Vellum 在 a31509aa 上为 383/365ms | 已满足（记录） | B_P2 没有在 C29 上重测，见 U-02 |
| N1/AC-5（P4）千页语料：输入立即获焦，冷查询 ≤ 1s，热查询 ≤ 100ms（L629） | P4 | §9.2；V-2、V-10 | M6、M7 | 记录：C29 共 240 个样本，最慢中位数冷 183.9ms、热 55.9ms，立即获焦，热态请求 0 | 已满足 | — |
| N3/AC-1（P1）静态托管根路径和子路径全部可用，深层 404 返回站点 404 页（L673） | 终验期 P4 | §8；V-8、V-12 | M7 | 实测：线上全部可用，两个深层地址返回 404，显示对应语言的页面。记录：root、subpath、meta 三种托管 | 已满足 | — |
| N3/AC-2（P1，每期复核）响应头完整策略与 meta 策略都零违规（L674） | 本期复核 | V-8 | M6、M7 | 记录：docs-search-root（响应头）与 meta 项目通过。实测：线上 meta-only 76 页零违规 | 已满足 | — |
| N3/AC-3（P1）只向站点自身的源发起请求（L675） | 终验期 P4 | V-8 | M6 | 实测：线上外源请求 0 | 已满足 | — |
| N3/AC-4（P1，每期复核）桌面三款最新稳定版；375 手机（L676） | 本期复核；**D17 取消手机** | V-11 | M7 | 记录：Chrome 155.0.8059.85、Firefox 158.0、Safari 27.0.1 原生通过 | 已满足（桌面）＋已确认变更（手机） | **G-01（B）** |
| N3/AC-5（P4）禁用脚本时正文、侧栏、目录、上一页/下一页可用，搜索与演示给出说明（L677） | P4 | §6；V-8、V-11 | M6、M7 | 记录：official、search 的 no-script 用例；三款原生浏览器禁脚本后再恢复 | 已满足 | — |
| N3/AC-6（P4）桌面 Chrome、Firefox、Safari 最新稳定版通过全部读者端验收（L678） | P4；**D17** | §9.3；V-11 | M7 | 记录：三款原生浏览器都做了阅读、导航、搜索与缺译、语言与主题、复制与演示、键盘、更多结果、禁脚本、200%、320/375，词边界 7 组，首次 module 恢复（[captures](../validation/docs-ssg-p4/m7/native-20261019/captures.json)） | 已满足（桌面）＋已确认变更（手机） | G-01；U-01 |
| N4（P1 单期，每期复核）无 Makepad、最小工具链、离线（L694–697） | 本期复核 | V-7、V-9 | M7 | 记录：C1 的 [toolchain](../validation/docs-ssg-p4/m7/release-20261019-c31/toolchain.log) 覆盖纯 Rust、离线、法语，new/check/build/preview/dev；双图没有 Makepad 或 CSR | 已满足 | — |
| N5/AC-1（P1）两种主题对比度达 AA（L713） | 终验期 P4；新页面类型：搜索对话框 | §6；V-8 | M6、M7 | 记录：search.spec 的 AA 用例、official 的 AA 用例；UI R2–R6 中 U1 和 U2 评 10.0 | 已满足 | — |
| N5/AC-2（P4）纯键盘完成跳到正文、侧栏、搜索、主题与语言、演示、复制，焦点可见（L714） | P4 | §5.2；V-8、V-11 | M6、M7 | 实测：线上纯键盘走查覆盖了全部六类操作，走过的焦点都有可见轮廓。记录：原生三款浏览器的搜索键盘与焦点归位 | 已满足 | — |
| N5/AC-3（P1）恰有一个 h1、四类地标、`lang` 正确（L715） | 终验期 P4 | V-8 | M6 | 实测：线上 76 页各一个 h1，`lang` 为 en 或 zh 且与内容一致。记录：theme.spec 的地标用例 | 已满足 | — |
| N5/AC-4（P4）VoiceOver 朗读结果数量、当前选项、复制成功或失败（L716） | P4；**D18** | §9.3；V-11 | M7 | 用户：D18（“VoiceOver通过了”）。Agent 没有采集逐项音频或转录 | 已满足（本人验收） | 证据边界已在 PRD D18 写明 |
| N5/AC-5（P1）320px、200%、减少动效（L717） | 终验期 P4 | V-8、V-11 | M7 | 记录：原生 200%（Chrome、Firefox、Safari）、320/375 响应式；official 的 reflow 与减少动效用例 | 已满足 | — |
| N5/AC-6（P1）全部页面类型的 axe 严重与重大违规为 0（L718） | 终验期 P4；搜索对话框 | V-8 | M6、M7 | 记录：official 的 gaps a11y 扫描 27 个路由 × 4 种组合；search.spec 扫描对话框各状态 | 已满足 | — |
| N6（P1 单期，本期回归）字节确定、失败保留旧产物（L732–735） | 本期回归 | V-5、V-9 | M4、M7 | 记录：AI 测试两次构建字节一致，坏输入时保留旧 dist；CLI 182 项通过、1 项既有 ignored | 已满足 | — |
| N7（P1 单期，本期回归）不泄露源文件、机器路径，不注入 HTML，无第三方请求（L749–752） | 本期回归 | V-1、V-5、V-8 | M1、M4 | 实测：线上 llms 与索引没有本机路径，没有 `.md`，外源 0。记录：search.spec 注入用例 | 已满足 | — |
| D16、Q2、Q4：公开 Git 安装、官方站部署与回滚（L805、L814、L816） | P4 | §8；V-12 | M7 | 记录：隔离缓存下公共 HTTPS 安装 C0 再到 C1 的回滚与恢复；Pages 四轮（C0 首发、C1 发布、C0 回滚、C1 恢复）各自完成同 run 的 97 文件核验和 4 项浏览器任务。实测：线上 manifest 与 C1 一致 | 已满足 | 相关的非 PRD 项：代码评审 F-01，见 §5 |

UAT 对应：UAT-5 → F9/AC-1–6、N1/AC-5、N5/AC-2/4；UAT-8 → F12/AC-1–4；UAT-9 → F9/AC-7；UAT-11 → F16/AC-1–4；UAT-12 的 P4 部分 → 搜索在窄屏可用，以桌面响应式模式验证，手机见 G-01。以上 UAT 均已逐行覆盖。

## 4. Triage 与决策账本

### 待决

无。Q-01 已于 2026-10-20 由用户决定，见下方 D-01；问题卡保留如下。

#### Q-01 · G-02 · llms.txt 中官方指南的描述怎么处理（已决 → D-01）

- **差异与来源**：PRD WORK-F12 行为、§1.4、§2.1 要求“一句话描述”，用来说明“各讲什么”。线上 12 份英文指南的描述中，10 条有 2–6 句；Quick start 和 Architecture 虽然只有一句，但取自与主题无关的段落。原因是 [ai.rs:27](../../crates/rustify-docs/src/ai.rs) 用的是整段首段，官方指南又都没有显式描述。
- **影响与原因**：S8 中外部 AI 工具的发现质量下降，所有 AC 和链接都不受影响。已证实：生成器按字面规则工作，作者参考对没有描述的页面会给出警告。尚未核实：官方 check 的 47 条既有警告中具体有几条是 missing-description。
- **待决定**：是补齐“一句话、讲什么”，还是接受段落式描述并修订 PRD 措辞？
- **选项与后果**：
  1. **补写官方指南的显式一句话描述**（推荐）。给 12 份 `docs/*.md` 指南加 `description`，其中 `docs/components.md` 由 `xtask catalog` 生成，要改生成器一并输出（提问时的判断；事后读码核实不需要改生成器，见 D-01）。不改框架语义，llms.txt 和页面 meta 描述同时改善。代价：GitHub 预览会把 frontmatter 显示成一个小表格，正文仍可读；要走一次 R/C 配对 → Pages 发布 → 线上核验。验收：两份 llms.txt 中每份指南一句话，能说明页面主题；GitHub 预览可读；catalog 的 `--check` 通过。推荐理由：这是框架给作者的既有契约（描述可选，没有时警告），官方站作为 G5 的示范站应当遵守，修复也不引入新的产品语义。
  2. **生成器只取首段第一句作为 AI 描述**（SEO 描述不变）。所有站点通用，10 条多句会变成一句。但 Quick start、Architecture 的首句仍然与主题无关，只部分达到目标。需要改生成器、补测试，再配对和发布。
  3. **接受现状，把 PRD 的“一句话描述”改为“描述（frontmatter 或正文首段摘要）”**。没有代码或发布成本；只需修订 PRD 措辞并同步作者参考。AI 工具继续拿到段落式描述，那两条与主题无关的描述保持不变。

### 已决（沿用既有决定）

| 决策 ID | 关联 G-ID | 用户选择与理由 | 确认日期与来源 | 新目标/验收边界 | 跟进/延期/豁免去向 |
| --- | --- | --- | --- | --- | --- |
| **D-01** | G-02 | 选项 1「Add guide descriptions」：为 12 份官方指南补写一句话的显式 `description`，`docs/components.md` 同样加在文件开头（问题卡选项里说要改生成器，经读码核实不需要：`xtask catalog` 只重写两个 `<!-- catalogue -->` 标记之间的表，保留其上的内容）；不改框架的描述语义（未选生成器截首句，也未选修订 PRD 措辞） | 2026-10-20，本会话 AskUserQuestion 回答 | 线上 en、zh 两份 llms.txt 中，每份指南（zh 包括 11 个缺译回退）是一句能说明页面主题的描述；页面 meta 描述同源更新；GitHub 预览仍可读；catalog `--check` 通过；通过既有 R/C 配对 → docs-pages 发布 → 线上全量核验上线 | 进入 gaps plan |
| PRD D17 | G-01 | “手机的测试不做，其他的继续” | 2026-10 本轮（已记入 [PRD L8、§5.1 D17](../PRD-DOCS-SSG.md)） | N3/AC-4/6、UAT-12 中，本次 P4 不验手机实机，其余门槛保留 | 保留变更；文档同步并入 G-01 的 follow-up |
| PRD D18 | —（N5/AC-4） | “继续验收其他的，VoiceOver通过了” | 已记入 PRD L9、§5.1 D18 | 以本人验收计为通过，不冒充 Agent 采证 | 无后续动作 |
| 计划 D7 | —（F12） | 无描述的首页依次用正文、tagline、Catalog 文案兜底 | 用户 2026-10-16（[计划 §0.2](DOCS-SSG-P4.md)） | 合法的空首页仍可发布，AI 描述不为空 | 已实现（M4），不是 PRD 偏差 |
| PRD D16、Q2、Q4 | — | Git 安装；GitHub Pages `/rustify-ui/`；公开发布留到 P4 | PRD §5.1、§5.2 | 公开安装、部署与回滚在 P4 完成 | 已交付（M7） |

本轮新增的用户决定只有 D-01，原 D17、D18、D7 均未改变。

## 5. 待核实项与审查基线

### 待核实（均不阻塞）

| ID | 缺失的事实 | 影响的需求与结论 | 补证方式 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| U-01 | 原生 Chrome、Firefox、Safari 测的是本地构建（build `7e1e5c9e7e7392df`）。线上 CI 构建（`9e8ea80d…`）有 6 个岛资源的 Wasm 不同，引用它们的 48 个组件页也不同（[代码复审 R2](DOCS-SSG-P4.code-review.r2.md) 待核实 1）。线上岛只有 Chromium 测过：M7 的 56 项 + 每轮 4 项，加本次无头实测 48 页全部 ready | N3/AC-6、F16/AC-2 的原生证据只覆盖同源码的本地产物 | 在三款桌面原生浏览器里各打开一个线上组件演示，操作一次并复制源码 | 否：PRD 不要求必须在线上字节上验原生 |
| U-02 | B_P2 的岛输入（Quickstart/Vellum 383/365ms）最后测于 a31509aa。C29 改了岛 Wasm 字节（原因未确定），之后只重测了 A 的最大页（55.3ms） | N1/AC-4 中 B_P2 的数值 | 在 C1 产物上按原 `measure-islands.mjs` 对 B_P2 重测五冷样本 | 否：离 1s 预算余量大，最大候选 A 已重测 |

**相关但不属于 PRD 差异的事项**：

- [代码复审 R2 F-01](DOCS-SSG-P4.code-review.r2.md)（P3）：CLI 0.1.31 的 `--help` 和 `new` 生成的 README 仍以 0.1.3 为回滚目标，与作者参考写的 0.1.29 不一致。PRD 没有关于 CLI 帮助文案的 AC，该项由代码评审承接，台账处置为 review-not-required（可选）。随下一次 CLI 配对处理即可，入口是 `/apply-code-review docs/plan/DOCS-SSG-P4.md`。
- [UI 复审 R6](DOCS-SSG-P4.ui-review.r6.md) 的结论“需补证”早于 M7 的原生补验和 D17/D18，之后没有独立 UI 复审。这不构成 PRD 缺口：N3/AC-6 和 N5/AC-2 有 M7 记录和本次实测支撑。如果需要独立的视觉与交互判定，可以另跑 `/review-ui`。

### 审查基线

| 项 | 版本/状态 |
| --- | --- |
| PRD | `docs/PRD-DOCS-SSG.md`，sha256 `ab9b1e870b42337a471da1a57ecabe85934b6a7124ae6b4f5e7ea3aa698fb584`（最后提交 a46ad4da） |
| 计划 | `docs/plan/DOCS-SSG-P4.md` v2，sha256 `52d138436b01590d869ab82e003f425f51e32c8cdc07ae2cf226f88a9f7c97f6`，7/7 |
| 完成记录 | M1 `4a82e37b3d4a`，M2 `1d7d9dd779b5`，M3 `42134b3467b6`，M4 `601255b73350`，M5 `9940892188dd`，M6 `c734b5b63af2`，M7 `a3d40d742afa`（均为 sha256 前缀） |
| 关联评审 | 代码复审 R2 `5274b8e8a93c`；UI 复审 R6 `efd4a89605ce` |
| 代码 | `main` / `4894d4cc` / dirty（只有 SDLC、评审与采集文件；产品路径干净）；C1 `1146b7d8`、R0 `0fd34eda` 是 HEAD 的祖先 |
| 线上 | build `9e8ea80d57750509`，CLI 0.1.31，96 个文件加 manifest；只读 GET，2026-10-20 |
| 本次实测环境 | macOS 27.0.1，Playwright 1.63.0 自带的无头 Chromium 153.0.8010.12，经本机代理；输出只在会话临时目录，未入库 |
| 证据等级 | 历史测试和原生结果按完成记录认定（记录）；线上全页、键盘、llms 与索引为本次实测；ai.rs 与作者参考为本次读码；VoiceOver 为用户本人结论 |

## 6. Follow-up 与 gaps plan

| G-ID / D-ID / PRD ID | 确定的增量工作 | 验收条件 | 依赖/阻塞 | gaps plan 任务或排除去向 |
| --- | --- | --- | --- | --- |
| G-01 / PRD D17、D18 / N3/AC-4、AC-6，N5/AC-4 | 同步 `docs/docs-ssg.md` L1475–1479 的浏览器范围和 `DESIGN.docs-ssg.md` L400 的 M7 门槛陈述 | 两处与 PRD D17/D18、M7 记录一致，不宣称手机通过或“仍待 M7” | 无 | [gaps plan](DOCS-SSG-P4-gaps.md) M1（§5.2，V-3） |
| G-02 / D-01 / F12 行为、F16/AC-4 | 12 份官方指南补写一句话 `description`（含 `docs/components.md`，生成器不改）；内容提交 + CLI 0.1.32 补丁配对 C2（框架仍为 R0），经 docs-pages 发布并核验 | 线上 en、zh llms.txt 中每份指南是一句说明主题的描述；meta 描述同步；GitHub 预览可读；catalog `--check`、官方 check/build 0 error；线上完整文件与浏览器核验通过 | 公开推送与 Pages 发布需要用户发布授权（按原计划 §8 的授权门） | [gaps plan](DOCS-SSG-P4-gaps.md) M1–M3（§5.1、§5.3、§8；V-1、V-2、V-4–V-9） |
| U-01、U-02 | 可选补证，不是已确定的修复 | 见 §5 | 无 | 不纳入计划，除非用户要求 |
| 代码评审 R2 F-01（非 PRD） | CLI 回滚提示 0.1.3 → 0.1.29 | 见代码复审 R2 | 新的 C 配对 | 由代码评审承接，gaps plan §0.5 已注明：用户另行确认后可放进 M1 的内容提交，随 C2 发布 |

gaps plan：[DOCS-SSG-P4-gaps](DOCS-SSG-P4-gaps.md)，状态 **Ready**，进度 0/3（M1 内容与测试 → M2 C2 配对与本地核验 → M3 授权、发布与线上核验）。它只承接 G-01 和 G-02/D-01；U-01、U-02 和 F-01 没有纳入。
