# DOCS-SSG-P4 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P4](DOCS-SSG-P4.md) | v1；SHA-256 `d5cacc9b1f70c23ec0298c81b270a2057fd9e6b2fce658f75baeeb2324016fc3` | 全文、PRD P4 映射、实际 P3 前置及相关代码/发布入口 |

评审 ID：`687ef358-5dbb-4e7a-af5f-dc54989a28b6`。调查基线：2026-10-15 +08:00，`main` / `77ca4f47ad099d1b162cf5b6190cc0b49d775da0`，dirty。事实源是当前工作树；产品源码与 HEAD 一致，已有未提交内容为 SDLC 登记、P3 评审和本计划，不将其计入本轮改动。原计划、PRD 和产品代码保持只读。

## 1. 确认问题

### [P2] F-01 · 更新已经交付的 P3 前置，撤销失效的开工阻塞

- **定位**：[计划](DOCS-SSG-P4.md) L7、L54–56、L82、L151、L276、L419、L433、L440–441；C-1、M1。
- **问题**：计划仍以 P3 代码不存在、进度 0/5 为由阻止 M1 开工。但其要求的真实语言图和本地交付已经存在。直接按恢复快照执行会等待已完成的前置，与本计划“不等设备补验”的边界冲突。
- **证据**：[content.rs](../../crates/rustify-docs/src/content.rs) L1043–1068 已定义 `PageLocale`、`LocaleView` 与 `Analysis.locales/catalogs`。[P3 快照](DOCS-SSG-P3.md) L50–66 为 4/5，M1–M4 完成；[M5 记录](DOCS-SSG-P3.records/M5.md) L5–8、L14–19、L25–55 保存 R₂ `820cea0f0ea886606427bfd031e17c93d401aca8`、C₂ `bae5b882d9304a47254d81662b5163981bb0adaf` 及 V-4、V-7–V-9 本地结果。[P3 §11](DOCS-SSG-P3.md#11-风险开放问题与就绪状态) L467 明确这些成果可解除 P4 前置，不必等待 V-10 或整行 M5 完成。本轮核对来源与当前类型，没有重新执行历史全量测试。
- **最小修订**：刷新调查基线、C-1、恢复快照及 §4/§11，链接当前固定版本与本地证据，将前置标为已交付。保留 P4 的 0/7 和 M7 原生设备/公开发布门，不把 P3 成果计为 P4 完成。
- **复核**：文档中的每个 M1 开工条件都能对应实际成果；不再引用“P3 代码尚未交付/0/5”。整份就绪状态须结合下列问题重新定级。
- **置信度：高**。对应依赖已经改变；须在按快照恢复开发前修订。

### [P2] F-02 · 修正中英紧邻时的英文词首规则

- **定位**：[计划 §2.2](DOCS-SSG-P4.md#22-worker-查询算法) L229–230、V-2 L375；M2、WORK-F9。
- **问题**：查询用 `Intl.Segmenter` 取词，候选英文词首却按 Unicode 字母/数字/下划线排除前字符。在合法标题“使用ThemeScope”中，“用”属于 Unicode 字母，因此 `ThemeScope` 和前缀 `Theme` 都被拒绝。页面虽然包含完整英文标识符，仍可能搜不到或失去标题优先级。
- **证据**：[PRD WORK-F9](../PRD-DOCS-SSG.md#work-f9-搜索全站内容) L378、L389 要求英文大小写无关、前缀匹配及标题命中优先。主 agent 用现有 Node `v26.1.0` 对计划规则作最小复核：`Intl.Segmenter('en', {granularity:'word'})` 将该标题分成 `使用@index0` 和 `ThemeScope@index2`；`i === 0 || !/[\p{L}\p{N}_]/u.test(text[i-1])` 则对两个查询都返回 false。`heme` 查询也为 false，原有词中子串反例可继续保留。这是规则复核，不是已实现搜索器测试。
- **最小修订**：明确英文标识符与汉字之间的边界，使候选词首与查询分词一致；可采用命中位置的词段边界或明确的标识符/文字切换规则。同步定义下划线、数字与前缀行为，避免只为该例增加特殊分支。
- **复核**：V-2 增加汉字紧邻英文标识符、大小写及前缀正例，保留 `heme` 不匹配 `ThemeScope` 和下划线/数字边界反例；在目标浏览器执行相同断言。
- **置信度：高**。漏匹配可由已定规则直接推出；M2 前修订。

### [P2] F-03 · 为搜索 module 自身加载失败定义可重试机制

- **定位**：[计划](DOCS-SSG-P4.md) L249、L290–292、L349、V-4 L377；M3、WORK-F9/AC-7。
- **问题**：计划将动态导入拒绝纳入统一失败/重试，但恢复动作只明确到递增 instance、重建 Worker。首次失败若发生在 `search.<hash>.js` 导入阶段，Worker 还没有创建；在本计划固定的 Chromium 153 中，再次导入同一 URL 仍然失败。仅重建 Worker 无法完成承诺的同页恢复。
- **证据**：主 agent 用已安装 Playwright 的实际 Chromium `153.0.8010.12`，通过 `page.route` 提供内存 HTTP 响应；`/module.js` 第一次返回 503，之后应返回 200。第一次 `import('/module.js')` 失败，同页第二次仍为 `TypeError`；`import('/module.js?attempt=1')` 成功。总 module 请求为 2，分别是首次原 URL 和带参数的新 URL，同 URL 重试没有重新请求。浏览器已关闭，没有启动服务或修改产品文件。当前 [WHATWG 模块加载算法](https://html.spec.whatwg.org/multipage/webappapis.html#fetch-a-single-module-script)已规定移除网络/非成功状态的失败项；本问题依据目标引擎实测，不能写成最新版规范保证永久缓存。
- **最小修订**：明确由轻量 `site.js`/bootstrap 拥有 module 未加载时的失败 UI 与恢复。冻结兼容目标引擎的加载方法，例如受控重试参数形成新的同源 URL，同时保留原指纹文件及其字节；清除拒绝的 promise，并用 attempt/instance 避免旧导入完成后重复绑定或启动旧会话。Worker/JSON 失败仍按原机制。
- **复核**：V-4 单列“首次 module HTTP/网络失败 → 恢复 → 同页点击重试”，断言原 query 保留、确有新的加载、结果可见、无重复监听及 CSP 违规。该局部检查不替代 V-8/V-11 的产品走查。
- **置信度：高**。固定引擎已复现；M3 前修订。

### [P2] F-04 · 定义合法无描述首页的 AI 索引行为

- **定位**：[计划](DOCS-SSG-P4.md) 模块表 L176、§5.3 L304–308、V-5 L378；M4、WORK-F12/AC-1。
- **问题**：计划要求每个公开内容页有带描述的索引条目，Markdown 描述只沿用 `Page.description`，缺描述排除规则仅针对注册页。但现有合法 `layout: home` 页面可以既无 description、又无正文首段。对该 `None` 是漏页、输出空描述还是新增构建错误，计划没有确定。前两者不满足全部内容页及描述合同，后者改变现有合法输入，需要明确诊断与作者指引。
- **证据**：[content.rs](../../crates/rustify-docs/src/content.rs) L1849–1876 仅从 frontmatter/首段形成 description，并只对非 Home 的缺失值报错。[generate.rs](../../crates/rustify-docs/tests/generate.rs) L267–286 的现有用例接受只有 `title: Home`、`layout: home` 的空首页，production 分析错误数为 0。[PRD WORK-F12](../PRD-DOCS-SSG.md#work-f12-为-ai-工具提供站点索引) L468 要求全部已发布内容页的标题、地址和描述。本轮据代码与现有断言核实，没有执行该历史测试。
- **最小修订**：在 §5.3 冻结此边界的处理及空/纯空白值语义。若仍允许该首页发布，明确可用的非空描述来源/兜底；若改为要求作者补描述，明确 `check/build` 的诊断、位置、建议及兼容影响。不要静默套用“无描述注册页不入索引”的规则。
- **复核**：V-5 覆盖无 description/无首段的 Home、仅有首页元数据的 Home，以及显式描述优先级；所有仍可发布且允许 AI 的内容页恰有一项，描述非空，或得到明确的构建诊断并保护旧代。
- **置信度：高**（合法输入及合同未覆盖均已核对）。M4 前确定策略。

### [P2] F-05 · 让 V-9 工具链检查安装同一固定 C

- **定位**：[计划](DOCS-SSG-P4.md) §3.2 L268、V-7 L380、V-9 L382、M6 L424；C-3、WORK-N4。
- **问题**：V-9 直接列出不带来源参数的 `--group toolchain`。它会安装另一个工作树合成提交，即使 V-7 已正确冻结并安装 C，也不能据此将隔离 PATH、离线及模板工具链结果记为该 C 通过。该命令还与本计划所引用的 P3 §3.2 冲突。
- **证据**：[xtask/src/docs.rs](../../xtask/src/docs.rs) L1352–1355 在无 `--git/--rev` 时调用 `candidate()`；L1111–1167 将 tracked/untracked 工作树写成新提交并返回其来源。[P3 §3.2](DOCS-SSG-P3.md#32-固定交付与回退) L294–303 已规定安装路径绑定，并特别说明 toolchain 不读取 `RUSTIFY_DOCS_BIN`，须独立传入固定 C；[P3 M5 记录](DOCS-SSG-P3.records/M5.md) L35 按此执行。合成提交在本轮没有与未来 C 建立可复用证据合同。
- **最小修订**：V-9 改为 `mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"`，沿用 V-7 的已记录变量。保留 features/artifacts/包级检查的开发态来源区分。
- **复核**：静态追踪不再进入默认 `candidate()`；实施时组内 `CLI source`、`--build-info` 和 `cargo install --list` 均对应交付 C，模板框架对应 R。
- **置信度：高**。M6 固定候选验收前修订。P4 L268 已继承 P3 的浏览器二进制绑定，故不另报“V-8 缺绑定”。

### [P3] F-06 · 合并 V-9 中重复覆盖的全量 Rust 测试

- **定位**：[计划 V-9](DOCS-SSG-P4.md) L382。
- **问题与证据**：同一收口同时安排 Docs/CLI 全包测试及 artifacts/reliability 两组。[xtask/src/docs.rs](../../xtask/src/docs.rs) L60–77、L85–90 表明两组只是再次运行 generate/artifacts/new_build、Docs lib、CLI bin/reliability/faults/external 等已包含 targets，没有额外组级验证逻辑。按清单逐条执行会重复同一验收工作；本轮未测量耗时，不声称节省比例。
- **可选修订与复核**：保留完整包级执行并把结果映射到这些测试组，或列互补 targets；保留 features/toolchain 的独立行为。检查删去的是重复执行、不是故障/产物覆盖。
- **置信度：高**。不阻塞；不要求为此新增里程碑。

## 2. 待核实与既定验收依赖

以下事项已有计划节点，不能因尚未执行就当作新缺陷，也不能记为通过。

| 项目 | 当前证据与缺口 | 提供/执行方、最晚确认点 |
| --- | --- | --- |
| Pages source/environment | 本轮只读 `gh api` 确认仓库 public、默认 main、当前凭证 admin/push；Pages API 仍返回 HTTP 404，未证明目标已配置。工作流 `enablement:false` 与现状判断一致 | 维护者确认既有配置与权限；M7 dispatch 前；不擅自启用或部署 |
| 原生矩阵、真机、VoiceOver、原生 200% | P4 C-5 和 P3 M5 已如实保留缺证；本轮未独立重核资源，没有从 ARIA/模拟推导朗读或真机通过 | 维护者提供现有设备/可操作入口，Agent 执行 V-11；M7 发布前，不回流阻塞 M1–M6 |
| 1000 页完整性能 | 临时微基准缺真实 fetch、Worker、中文正式语料与绘制；完整冷/热预算仍须测量 | 实施者 V-2 局部迭代、M6 的 V-10 正式收口；不得以本轮探针代填 |
| 公共 C/R、真实 Pages 与首次回滚 | 本期候选尚未形成；本地 Git remap 不证明公共可达。C0/C1、完整目录/manifest 身份和回滚已有设计 | 维护者确认实际候选与写入授权后，Agent 在 M7 执行 V-12；本轮无推送、dispatch 或外部写入 |

## 3. 覆盖、基线与实际检查

### 四个视角与综合复核

初轮实际并行启动 facts、architecture、schedule；空出槽位后启动独立 delivery。四位 reviewer 只读，主 agent 单写报告并复核每条确认问题的原文/代码。后续仅对受影响的前置、加载恢复和固定身份交换证据。

| 维度 | 负责人 | 结果与证据缺口 |
| --- | --- | --- |
| facts | `/root/facts` | 已完成；P3 前置过期、工具链来源冲突；frontmatter 独立开关、现有 CSP/英文站及 Pages 工作流复用成立 |
| architecture | `/root/architecture` | 已完成；中英边界、module 恢复、Home 描述边界；保留按语言 JSON/Worker、本代提取/发布和固定 R/C 路线 |
| schedule | `/root/schedule` | 已完成；核对本期 AC、跨期补证、依赖与两阶段收口；来源命令问题合并 F-05，重复执行列 F-06 |
| delivery | `/root/delivery` | 已完成；读者、作者、维护者主路径闭环；缺口合并至确认问题，无独立视觉风格问题 |
| 综合裁定、写入/回读 | 主 agent | 已完成；逐条去重，保留实际运行边界与原计划状态，不改稿或实施 |

相关需求基线为 [DOCS-SSG PRD](../PRD-DOCS-SSG.md)，SHA-256 `22f4fac99b06a23cda3c34f7ee0c66c11369ccd60a34d71fecee2617aa4c16c1`；P3 当前计划为 SHA-256 `11f1ea0a2ac0b45dd29cb80fdcc2729da7595a47fd567bacce6eb6b1ab0d8c99`。评审结束核对 HEAD、P4、PRD、P3 未发生并发变化；本轮写入限独立报告与 SDLC 登记。所审 P4 尚未提交，跨机器需取得相同文件并核对 hash。

### 实际执行

| 检查 | 结果及解释 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P4.md <仓根>` | exit 0；30 个路径 token、12 个 MISSING：10 个明确新增文件、1 个相对内容根的新增 `zh/quickstart.md`、1 个实施协议中的兼容旧台账路径。未确认虚构复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P4.md <仓根>` | exit 0；0/7、里程碑 7 行、记录 0 行、ERROR 0/WARN 1。告警涉及已有 dirty 工作树；核对无 P4 实施结果，初稿空记录合法。脚本不检查前置事实，F-01 仍成立 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P4.md` | exit 0；ERROR 0/WARN 10，均为沿用 WORK 编号，已按 PRD 人工核对 |
| Node 词首规则最小探针 | F-02 的两个正例被计划规则拒绝；不是产品性能或兼容验收 |
| Playwright Chromium 153 module 最小实验 | F-03 已复现；只有内存路由/空页面，退出后关闭浏览器；不是 P4 界面验收 |
| GitHub 只读 API | 仓库信息成功；Pages HTTP 404。无设置变更、推送或发布 |

两份 shell 检查最初误用 `sh`，因 Bash 进程替换语法失败；读取其 shebang 后用 `bash` 重跑，以上是成功重跑结果，未将调用错误判为计划缺陷。未运行产品整包测试、构建、容量基准或发布；这是一轮计划评审。

### UI 证据等级

本期采用 **DOM**：静态阅读壳、原生 dialog、同源 JS/Worker，不需要 GPU 或跨区协作。已使用 impeccable 的 context 入口，并结合 [现有 surface brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md) 与 [DESIGN.docs-ssg.md](../../DESIGN.docs-ssg.md) 核对 Read 世界、状态、主题、布局、键盘、语言及错误恢复。

P4 搜索尚无可运行页面或原型，**未在浏览器中验证 P4 界面**。对搜索布局与交互的判断限文档推演；既有 P3 截图/交互记录只覆盖其原范围。本轮 module 浏览器实验只核技术前提，不能证明 dialog、CSP、响应式或辅助技术验收通过。无页面本身不构成阻塞问题；实现后按 V-8/V-11/V-12 验收。

## 4. 需求映射与实施编排

| 需求/范围 | 设计 → 任务 → 验收 | 核对结果 |
| --- | --- | --- |
| WORK-F9 全 AC | §2/§5.1–5.2/§6 → M1–M3 → V-1–V-4、V-8/V-10/V-11 | 入口、当前语种、定制页、分组/锚点、空结果、键盘及故障均有落点；F-02/F-03 补具体语义 |
| WORK-F12 全 AC | §3/§5.3 → M4 → V-5、V-8/V-12 | 分组、导航外页面、缺译源地址、无 URL 与排除有合同；F-04 补合法 Home 输入 |
| WORK-F16/AC-3/4、继承 AC-1/2；WORK-F4 补证 | §5.4/§8/§9.3 → M5–M7 → V-6/V-8/V-12 | 两份真实中文内容、既有指南/分类演示、实际发布/源预览有出口，不重开原全部功能验收 |
| WORK-N1 | §7/§9.2 → M2 局部、M6 正式 → V-2/V-10 | 100 页迭代与每语言 1000 页正式数据分开；继承预算、五冷样本和固定引擎有安排 |
| WORK-N3/N5 | §6/§9.3 → M6/M7 → V-8/V-9/V-11/V-12 | 引擎/模拟、稳定版、真机、真实朗读明确区分，保留发布风险门 |
| WORK-N4/C-3 | §3.2 → M6 → V-7/V-9 | F-05 修正固定 C 的工具链见证；P3 安装/浏览器绑定可按明确引用继承 |
| WORK-N6/N7 | §3.1/§7/§8 → M1/M4/M6 → V-1/V-5/V-8/V-9 | 有同代输出、确定字节、失败保全、公开策略/源泄露与文本安全覆盖 |
| C-1 | 实际 P3 本地成果 → M1 | F-01 更新已失效的依赖状态；不要求 P3 整期完成 |

现有七个里程碑和两阶段收口可以保留，不需新增里程碑。依赖边为：实际 P3 本地成果 → M1/V-1 → M2/V-2 → M3；V-1 → M4；M3+M4+P3 渲染 → M5；V-1–V-6+正式语料 → M6/V-7–V-10；M6+原生资源 → V-11；V-11+公共候选/Pages/有效授权 → M7/V-12。未发现依赖环。

| 波次 | 前置及解锁证据 | 可并行工作与共享负责人 | 运行资源与汇合验收 |
| --- | --- | --- | --- |
| 1 | F-01 更新后的真实 P3 前置；F-02–F-04 契约分别在受影响任务前修订 | 主实施者推进 M1，独占 content/generation/注册/配置；语料准备在输入稳定后独立推进 | V-1 冻结公开数据/URL 合同 |
| 2 | V-1 | M2 Worker/语料与 M4 AI 模块可并行；委派仅写独立模块/测试，生成器及保留路径由主实施者统一集成 | 局部浏览器串行；V-2、V-5 各负责本支退出 |
| 3 | V-2、P3 Catalog/shell/404 | M3 搜索壳与尚未完成的 AI 支路互不依赖；shell/labels/CSS/site.js 单人负责 | V-3/V-4 与 V-5 汇合 |
| 4 | M3+M4+P3 渲染 | M5 中文文案可独立编写，注册、导航及配置由主实施者集成 | V-6 后形成固定 R/C |
| 5 | V-1–V-6、正式语料和官方候选；F-05 已修订 | 主实施者收口 M6，记录开发态/安装态证据来源；版本、锁文件及配对文件单人负责 | V-7–V-10 集中执行；浏览器项目、输出目录及性能测量串行 |
| 6 | M6；V-11 所需资源；Pages/公共来源/有效授权 | 主实施者独占 M7 发布；维护者提供既有外部资源，Agent 实测 | V-11 先于 V-12，完整目录发布/回滚并核对线上身份；不重复未失效的 M6 全量 |

P4 的搜索性能端口 4207 与当前 `components-i18n-root` 相同，但 [Playwright 配置](../../playwright.docs.config.ts) L45、L73、L92–96、L168–182 及计划均限定逐项目串行、只启动所选服务，未构成已确认冲突。继续遵守资源隔离即可，不为此增加阻塞。这里给出拓扑顺序，不估算工期。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

建议状态：**Blocked**（所审全文）。确认 **5 项 P2、1 项非阻塞 P3**。与计划自报 Blocked 的字面状态一致，但“P3 尚未交付”的开工依据已过期；当前待处理的是上述局部设计/验收合同，不能继续以缺少 P3 代码解释阻塞。已确认的 P3 本地成果可以复用，修订前置后不依赖这些问题的工作可继续。

修订顺序：先处理 F-01 的实际基线和开工门；再处理 F-02 的匹配边界、F-03 的模块恢复及 F-04 的 AI 描述策略；M6 前修正 F-05 的固定来源命令。F-06 可选采纳。改稿复核须能逐项追通对应 V 检查；P4 进度仍为 0/7。设备与 Pages 的已知出口依赖保持原节点，不因本轮评审转成已通过，也不回流阻塞独立本地开发。

下一步：核实并修订计划，处置后按真实前提判定 Blocked 或 Ready；本轮不自动执行下一步。

```text
$apply-doc-review docs/plan/DOCS-SSG-P4.review.md
```

计划评审与局部浏览器实验均不表示功能已实现、完整验收通过或可以发布。

## 修订处置与复核

- 来源：本报告原始 SHA-256 `6d11cd88a1f63c9a6cb22e9cd7273e1cc8b96094f734f71256e323fbabc872d9`；评审 ID `687ef358-5dbb-4e7a-af5f-dc54989a28b6`。上方原问题、所审基线与“需要修改 / 建议 Blocked”结论完整保留；此前无处置记录。
- 目标：[DOCS-SSG-P4](DOCS-SSG-P4.md)，v1 → v2；修前 SHA-256 `d5cacc9b1f70c23ec0298c81b270a2057fd9e6b2fce658f75baeeb2324016fc3`，与本报告所审版本一致；修后 SHA-256 `48a7150509e4f8fd66f58ebb558dd0bd21ed248113fc2eddc7d27f4571d34229`。
- 复核日期与代码基线：2026-10-16 +08:00；`main` / `77ca4f47ad099d1b162cf5b6190cc0b49d775da0`，dirty。开始时已有 SDLC 登记、P3 评审、P4 计划与报告未提交；原 PRD、P3 计划和产品代码未改动。本次只修订 P4 计划、本报告处置节及受影响台账，不运行计划实施任务。
- 范围：F-01–F-06 全部；同时核对报告 §2 的既定出口依赖和 §4 的映射/编排，不重跑独立计划评审。

| 来源问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 本轮复核结果 / 后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。[content.rs](../../crates/rustify-docs/src/content.rs) 的 PageLocale/LocaleView/Analysis 字段已存在；[P3 快照/§11](DOCS-SSG-P3.md#恢复快照)、[M5](DOCS-SSG-P3.records/M5.md) 与[安装身份](../validation/docs-ssg-p3/m5/installed-identity-final.json)共同给出 4/5、R₂/C₂ 及本地成果。两个提交在当前 HEAD 可达；没有重跑其历史验收。 | 已修订并复核 | P4 基线/v2、恢复快照、C-1、§1.1、§4、M1 与 §11；V-1 核对新增集成，V-2 仍归 M2。 | **已解决（文档）**。旧“未交付 / 0/5”开工依据已清除，M1 可开始；P4 保持 0/7，P3 设备欠证仍保留。 |
| F-02 | 成立。当前 PRD WORK-F9 要求英文前缀/大小写和标题优先；旧 Unicode 前字符规则在“使用ThemeScope”拒绝两个正例。改用同语种 Segments.containing 的词段起点/isWordLike 与词段前缀，避免查询/候选边界分离。 | 已修订并复核 | §2.2、V-2、§9.3/V-11；同步中英两侧紧邻、下划线、数字、词中反例和 UTF-16 偏移。 | **已解决（文档）**。Node v26.1.0 与 Chromium 153.0.8010.12 的 15 项边界断言通过，emoji 起点为 2；标题排序、完整 Worker 与全目标浏览器行为仍由实施时 V-2/V-11 验证。 |
| F-03 | 成立。当前固定 Chromium 对首次 HTTP 503、网络失败后的同 URL 再导入均报 TypeError；对原路径添加受控参数后成功。Worker 尚不存在时确须由 bootstrap 提供恢复。 | 已修订并复核 | 文件/模块职责表、§2.3、§3、§5.1、§8、V-4 与 V-11；site.js 拥有加载前状态/重试，search.js 为无静态依赖且顶层无副作用的显式初始化模块；清 promise、受控 attempt URL、instance/attempt 防过期及成功 module 复用已定义。 | **已解决（文档）**。两种失败最小实验均观察到仅原 URL 与 `?rd_search_attempt=1` 两次请求、重试导入成功、最小 CSP 无违规。原输入、重复监听、关闭/超时后晚完成及完整 CSP 的产品断言已纳入 V-4/V-8/V-11，尚未执行。 |
| F-04 | 成立。[content.rs](../../crates/rustify-docs/src/content.rs) 的 description 只取显式值/summary，Home 豁免必填；[generate.rs 的空首页断言](../../crates/rustify-docs/tests/generate.rs)与 WORK-F12/AC-1 的全内容页合同存在未定边界。frontmatter 原样保留空白字符串，不能把它当有效描述。 | 已修订并复核 | D7、ai.rs 模块合同、§5.3、V-5；用户已选择兼容首页，按描述→首段→tagline→来源语言“{站点名}首页”兜底，空白视缺失；注册页仍必须显式描述。非 Home 的纯空白且无首段补齐原必填语义，明确文件诊断/旧代保护。 | **已解决（文档）**。合法空 Home 仍允许发布，AI 条目唯一/非空；只含元数据、优先级、空白、中英/缺译与排除均有 V-5 判据。未执行未来 ai.rs 或新生成测试。 |
| F-05 | 成立。[xtask docs::toolchain](../../xtask/src/docs.rs) 的 `(None,None)` 会走 candidate()，`(Some,Some)` 才直接安装指定来源；[P3 §3.2](DOCS-SSG-P3.md#32-固定交付与回退)明确 toolchain 不读 RUSTIFY_DOCS_BIN。 | 已修订并复核 | §3.2、V-9；显式使用 `mbx xtask docs-verify --group toolchain --git "$DOCS_LOCAL_GIT" --rev "$DOCS_C"`，承接 V-7 变量及 C/R 身份核对。 | **已解决（文档）**。静态追踪命中指定来源分支，不再进入默认 candidate()；未来固定 C 尚未形成，未执行安装/工具链验收。 |
| F-06 | 成立，采纳可选建议。[xtask::verify/docs_tests](../../xtask/src/docs.rs) 的 artifacts/reliability 两组仅包装相同 targets，没有额外组级行为。 | 已修订并复核 | V-9 与表后 targets 映射；保留 Docs/CLI 分包完整测试，去掉这两组重复命令；features/toolchain 独立执行不变。 | **已解决（文档）**。generate、artifacts/new_build、Docs lib、CLI bin/reliability/faults/external 均可映射；补跑未执行 targets、失败/ignored 不计通过已明确。未测量节省比例。 |

### 决策与未完项

- 用户 2026-10-16 在本次问答明确选择：“兼容现有首页（推荐）：依次使用显式描述、正文摘要、首页 tagline，仍缺失时用按来源语言生成的‘{站点名}首页’描述。”问题同时说明空值/纯空白视为缺失。该决定已落实 D7、§5.3 与 V-5；不采用让合法空 Home 构建失败的备选。兜底仅影响 AI 描述，不改变 Page.description/SEO，不套用到缺描述注册页。
- 报告 §2 的四类出口事项仍保留：原生稳定浏览器/真机/VoiceOver/原生 200%、M6 完整性能、M7 公共 C/R/实际 Pages/回滚。Pages 状态仅沿用 2026-10-15 的历史只读调查，本次未重查远端；到 M7 重核实际配置与发布授权。没有以文档修订关闭这些验收。
- 关联但未授权修改：[PRD WORK-F12](../PRD-DOCS-SSG.md#work-f12-为-ai-工具提供站点索引)的行为描述仍只列 frontmatter/首段，后续维护时应补入用户确认的 Home 兼容例外；P4 已保存决定和完整实施判据，不等待再次批准该规则。作者参考和设计文件仍按 M6 在实现/验收后同步；P3 原计划及记录保持原位。

### 修订后结论与验证

**本次 6 项均已修订并复核，文档问题已解决；不采纳 0、待决 0。P4 v2 为 Ready，可开始 M1，本期实施仍为 0/7。** M7 的已知出口资源与授权门保留；这里是同轮文档处置，不是独立复审通过，也不表示功能或发布完成。

| 本次实际检查 | 结果及边界 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P4.md <仓根>` | exit 0；30 个路径 token、12 个 MISSING：10 个明确新增文件、1 个相对内容根的 `zh/quickstart.md`、1 个实施协议兼容旧台账路径。无已确认的虚构复用路径。 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P4.md <仓根>` | exit 0；0/7、里程碑 7 行、记录 0 行、ERROR 0/WARN 1。计划本身未跟踪，脚本以 git diff 判“本计划没被改过”而报 dirty 告警；回读当前文件确认已修订、没有 P4 实施记录。 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P4.md` | exit 0；ERROR 0/WARN 10；10 个沿用 WORK ID 均在 PRD 中存在，变更后的设计→检查→里程碑引用已人工核对。 |
| Node / Playwright 最小探针 | 词首旧规则失败可复现，新规则 Node/Chromium 各 15 项通过；HTTP/网络失败的 module 恢复各通过。均为临时内存页面，无服务启动或产品文件改动，浏览器已关闭。 |
| 规范原文核对 | [ECMA-402 containing](https://tc39.es/ecma402/2026/#sec-%intlsegmentsprototype%.containing)提供命中词段及 UTF-16 索引；具体匹配规则仍是本计划决策。[WHATWG module fetch](https://html.spec.whatwg.org/multipage/webappapis.html#fetch-a-single-module-script)会移除网络/非成功状态的失败项，故恢复设计依据目标引擎实测，不宣称规范保证永久缓存失败。 |
| 基线/差异与链接核对 | R₂/C₂ 本地可达；修后计划的实施协议、完成记录逐字保留，原报告正文保留。原 PRD/P3 hash 未变，产品代码未改；文档链接、状态、计数和受影响台账另行回读。 |

本次没有产品整包测试/构建、完整性能或正式 UI 走查；**未在浏览器中验证 P4 界面**。最小探针不能代替 V-8/V-11/V-12 的 dialog、键盘、响应式、完整 CSP、辅助技术或真实线上验收。

下一步（仅建议，不自行启动）：

```text
/goal 按 docs/plan/DOCS-SSG-P4.md 开发
```
