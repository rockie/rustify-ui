# DOCS-SSG-P2-gaps-2 · 开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md) | SHA-256 `b0391d7885e08d3ea7f0f0dc350bfc93ca9e1b4570ce2108763c9d77389cdef3` | 全文，M1/M2、V-1–V-5；首次完整评审 |

- 评审日期：2026-10-15 +08:00；评审对象 ID：`d589882c-01b3-4031-b976-e2eedc754763`。
- 调查基线：`main` / `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` / dirty。事实取当前工作树；计划是未跟踪初稿。既有未提交项为台账、报告、验证证据、fixture 日志与缓存，产品源码没有未提交差异。本轮未修改计划、产品源码或既有证据。
- 需求来源：[PRD](../PRD-DOCS-SSG.md) 的 P2 增量，以及[差异复审报告 D-05](DOCS-SSG-P2.prd-dev-gaps.r2.md#已决)。后者是需求输入，不是本计划的上轮评审；本报告不宣称关闭其中的历史问题。
- 呈现范围：DOM dev 面板与诊断生产方；GPU 不适用。只审本地固定配对交付，不扩大到原 M6/M7、U-01–U-03 或公开发布。

## 1. 确认问题

### [P1] F-01 · 写法函数的 pipeline 边界无法覆盖已纳入的 SSR/hydrate 调用方

- **定位**：[计划 §1.2 L146](DOCS-SSG-P2-gaps-2.md#L146)、[§5.2 L184](DOCS-SSG-P2-gaps-2.md#L184)、[M1 门 L259](DOCS-SSG-P2-gaps-2.md#L259)。
- **问题**：计划要求 `site/registry.rs` 的自有消息统一调用唯一 docs 写法函数，同时明确把该函数放在仅 `pipeline` 启用的 `diagnostic` 模块。真实组件工程关闭默认特性，分别以 `site-ssr` 与 `hydrate` 使用 registry；按此位置实施会引用不存在的模块。强开 `pipeline` 也不能修复 hydrate，因为它同时启用 `site-ssr`，触发互斥。M1 默认特性测试及浏览器可以通过，直到冻结 R″ 后的 M2 两目标检查才暴露失败。
- **证据**：[lib.rs L16–28、L53–54](../../crates/rustify-docs/src/lib.rs#L16) 的互斥与模块门；[Cargo.toml L12–18](../../crates/rustify-docs/Cargo.toml#L12) 的特性边；[registry.rs L139–162](../../crates/rustify-docs/src/site/registry.rs#L139) 与 [L309–315](../../crates/rustify-docs/src/site/registry.rs#L309) 的真实调用点；[组件模板 L19–27](../../crates/rustify-docs-cli/templates/components/Cargo.toml.in#L19) 的独立特性图。另，[dev-components.spec.ts L32–64](../../tests/docs-ssg/dev-components.spec.ts#L32) 虽重建工作树 CLI，仍调用 [docs-pages-prepare.py L51–67、L116–133](../../scripts/docs-pages-prepare.py#L51)，按 CLI 所报提交的配对 R 投影 fixture；当前仍为 R′，不能代替新 registry 源码的特性检查。
- **最小修订**：确定不依赖 pipeline、能被相关特性图共同访问的轻量函数位置与导出，列明相应文件增量；保留 docs/syntax 受控重复，不必新增 crate。同步调整站点与岛运行时无需修改的范围声明。在冻结 R″ 前增加当前工作树的独立 `site-ssr` 与 hydrate/wasm 检查，可复用[现有 CI L70–73](../../.github/workflows/verify.yml#L70) 的入口。
- **复核**：改稿后 helper 可见性与两目标调用链闭合；实施后无 pipeline 的两图均通过，hydrate 不夹带 pipeline。M2 仍保留固定 R″ 的真实 new 工程检查。
- **置信度**：高，特性关系与真实模板均已读；**阻塞**：M1 函数位置和 registry 改写。

### [P2] F-02 · 空内容不能按既定规则读回，还会改变无反引号输入的显示

- **定位**：[计划 §5.1 L172–180](DOCS-SSG-P2-gaps-2.md#L172)、[V-1 L245](DOCS-SSG-P2-gaps-2.md#L245)。
- **问题**：空字符串不含反引号，写出后是两个相邻反引号。读法把它识别成一个长度 2 的串，没有关闭串，因而原样显示。因此任意内容读回原内容的性质不成立，不含反引号内容的结果也会改变。它还可能与后续同长串发生跨片段配对。
- **证据**：[config.rs L298–305、L478–488](../../crates/rustify-docs/src/config.rs#L478) 中 `site.lang = ""` 是真实可达的错误输入。本轮[浏览器探针](../validation/docs-ssg-p2-gaps-2/plan-review-r1/browser-probe.json) 的 `empty language` 阶段得到 message `"`` is not a language tag"`，旧面板 `textContent` 为 `" is not a language tag"`；按新规则会保留那两个反引号。[旧 prose L208–218](../../crates/rustify-docs/assets/dev.js#L208) 确认当前会跳过空片段。
- **最小修订**：明确空内容的无歧义编码与兼容规则，同步限定无反引号输出逐字节不变的适用范围；给两份 writer、读回测试和一致性测试加入空串及空片段与后续片段组合，V-3 加真实空语言输入。
- **复核**：单独空内容及完整诊断中的空片段均读回空内容，不吞并后续片段；旧常规输入保持原结果。
- **置信度**：高，规则矛盾及可达输入均已核实；**阻塞**：M1 编码合同。

### [P2] F-03 · 普通文本中的作者插值没有编码规则，仍会丢失字面量反引号

- **定位**：[计划本期交付 L11](DOCS-SSG-P2-gaps-2.md#L11)、[§5.2 L185–188](DOCS-SSG-P2-gaps-2.md#L185)、[V-2/V-3 L246–247](DOCS-SSG-P2-gaps-2.md#L246)。
- **问题**：D2 只改原先包在代码片段里的插值。但自有诊断也把作者数据放在普通文本中，D5 会继续把这些数据自带的成对反引号当分隔符。即使所有原有代码片段都正确改写，本期逐字保真目标仍不能成立；这类文本不是 D4 排除的第三方原文。
- **证据**：[content.rs L2320–2331](../../crates/rustify-docs/src/content.rs#L2320) 的 broken-anchor 消息直接拼入 `{file}`，未加围栏。本轮创建临时页面 ``content/a`b`c.md`` 并引用编码后的路径，SSE message 中的文件名含两个字面量反引号，四种视口/主题下的面板均显示 `content/abc.md`，见[探针](../validation/docs-ssg-p2-gaps-2/plan-review-r1/browser-probe.json)。若只按计划修 anchor 片段，未改的 `{file}` 仍会被新读法配成 `code[b]`，丢失文件名字符。修后仍失败的判断来自计划规则推演，不冒称已实现实测。
- **最小修订**：为自有模板中普通文本作者值确定字面反引号处理办法，明确其与常规输出不变的兼容关系；将这类插值纳入生产方改写与审计。V-1/V-3 增加普通文本值与正式代码片段混合的真实反例，并在固定版本出口复验。
- **复核**：文件名的反引号全部保留，真正的代码片段边界正确；不能靠删除全部反引号或用同一错误解析器生成期望值证明通过。
- **置信度**：高，自有模板、作者入口和当前浏览器反例均确认；**阻塞**：M1 生产方覆盖合同。

### [P2] F-04 · V-2 的搜索条件漏掉带固定前缀的插值片段

- **定位**：[计划 §5.2 L185、L188](DOCS-SSG-P2-gaps-2.md#L185)、[V-2 L246](DOCS-SSG-P2-gaps-2.md#L246)。
- **问题**：全覆盖承诺只用“开反引号后紧接 `{`”的搜索核验，不能发现固定前缀后的插值。实施者即使让该搜索归零，仍可能留下同类显示缺陷；这与 F-03 的未包装作者值不同，这些位置已经是正式代码片段。
- **证据**：[content.rs L2330](../../crates/rustify-docs/src/content.rs#L2330) 的 `#{anchor}`、[generation.rs L677、L721](../../crates/rustify-docs/src/generation.rs#L677) 的 `#{id}`/`#{anchor}`、[markdown.rs L1314](../../crates/rustify-docs/src/markdown.rs#L1314) 的 `rustify-region:{region}` 及 endregion 均不匹配该搜索。本轮链接 `a%60b%60c.md#a%60b` 经 percent decode 生成带反引号的 anchor，SSE 为末尾含 `` `#a`b` `` 的消息，当前面板显示 `#ab`；来源与实测见 F-03。
- **最小修订**：明确 helper 接收完整片段（含固定前后缀、多个插值和格式化内容）；扩大审计到全部插值代码片段，记录剩余命中的分类。为 percent-encoded anchor 增加精确文字与 code 节点断言。
- **复核**：上述前缀形式均纳入改写清单；不能以旧正则零命中作为全覆盖结论。
- **置信度**：高，漏网模板及可达 anchor 均确认；**阻塞**：M1 的 V-2 全覆盖验收。

### [P2] F-05 · 逐字读法与新增的 CommonMark 消费合同不一致

- **定位**：[计划 D1 L85](DOCS-SSG-P2-gaps-2.md#L85)、[读法 L178](DOCS-SSG-P2-gaps-2.md#L178)、[作者参考 L196](DOCS-SSG-P2-gaps-2.md#L196)。
- **问题**：计划保留代码内容的换行，且只在围栏长度 ≥2 时裁剪两端空格，却称与 CommonMark 一致并让 JSON 读者按 Markdown 读取。遵循作者参考的标准解析器会得到不同于面板的文字。
- **证据**：[CommonMark 0.31.2 §6.1](https://spec.commonmark.org/0.31.2/#code-spans)（查询 2026-10-15）规定代码片段中的行结束符转为空格，并按内容条件去掉两端各一个空格，不以围栏长度区分。输入 `" x "` 按计划写成单围栏片段，面板保留两端空格而标准读法得到 `"x"`；输入 `"a\nb"` 按计划保留换行，标准读法得到 `"a b"`。这是标准原文与计划规则的对照，不是版本升级建议。
- **最小修订**：保留逐字目标时准确声明这是 Rustify 的片段读法，写清其与 CommonMark 的差异，撤掉通用 Markdown 读取等价的承诺；或在计划中确定另一套满足目标的编码办法。V-1 加入首尾空格、LF/CRLF，作者参考同步。
- **复核**：文档规定的消费者读法与面板结果一致；若继续承诺标准兼容，须有对应的标准解析断言。
- **置信度**：高，官方原文已读取；**阻塞**：M1 协议与作者参考定稿。

### [P2] F-06 · V-1 指定的含反引号 name 不可能触发未注册组件诊断

- **定位**：[计划 V-1 L245](DOCS-SSG-P2-gaps-2.md#L245) 的“YAML `name` 含反引号的未注册组件”。
- **问题**：真实作者入口先校验 name 是否为 Rust 标识符，反引号在这一层就被拒绝。因此该退出断言无法按现有拒绝语义实现；为让测试通过而放宽 name 语法会违反本计划“不改拒绝语义”的范围。
- **证据**：[embed.rs L72–89](../../crates/rustify-docs/src/embed.rs#L72) 返回 `embed-payload`；[markdown.rs L1207–1212](../../crates/rustify-docs/src/markdown.rs#L1207) 在该错误后直接结束；[rendering.rs L633–650](../../crates/rustify-docs-cli/src/rendering.rs#L633) 的 `unregistered-component` 只处理成功解析的声明。
- **最小修订**：改为含反引号 name 仍产生既有 `embed-payload`，普通合法但未注册的名称覆盖兼容行为。需要动态文字保真时使用已列的 YAML 未知键或 F-04 的真实 anchor 输入，不绕开入口构造不可能的声明。
- **复核**：V-1 的输入、实际 rule 和预期结果可同时成立，不改变解析器接受范围。
- **置信度**：高，拒绝链完整；**阻塞**：M1 对应验证项。

### [P2] F-07 · C″ 的五文件审计门与已知 dirty 基线冲突

- **定位**：[计划 §5.5 L204–205](DOCS-SSG-P2-gaps-2.md#L204)，与 [L7](DOCS-SSG-P2-gaps-2.md#L7)、[提交点 L307](DOCS-SSG-P2-gaps-2.md#L307) 冲突。
- **问题**：当前已有 tracked 台账及 fixture 日志修改；按提交点要求保留、不纳入这些内容时，原始 `git diff --name-only` 相对 HEAD 必然超过五文件。字面执行该门无法通过，还可能诱使实施者清理或暂存用户已有修改。
- **证据**：本轮 `git status --short` 确认 `.xgent-ai/sdlc/index.md` 等台账及 `tests/docs-ssg/fixtures/components/.rustify-docs/logs/*` 为已跟踪修改。原增量计划[§5.4 L203](DOCS-SSG-P2-gaps.md#L203) 和[提交点 L328](DOCS-SSG-P2-gaps.md#L328) 已把产品差异与纯进度/证据差异分开；新计划省掉了这项审计口径。
- **最小修订**：明确审计本次 C″ 拟提交的产品差异，例如 staged 五文件；保存并核对既有 dirty 清单，单列协议要求的进度/证据差异。相对 R″ 的库、运行资源、功能 src、模板仍要求零产品差异，不要求清理既有工作树。
- **复核**：保留当前无关 tracked 修改时，仍能证明 C″ 产品提交恰好五个允许文件且无夹带。
- **置信度**：高，当前基线直接可见；**阻塞**：M2 提交前审计，不单独阻塞 M1。

## 2. 待核实项与已明确边界

没有需要额外访问外部服务才能裁定的关键前提。上列问题是已确认的设计/验证缺口，不能用“实施时再决定”代替修订。

| 范围 | 当前证据及限制 | 责任与最晚确认点 |
| --- | --- | --- |
| 新写法/读法、两份实现一致性、全部模板覆盖 | 尚未实施；本轮只做规则与代码调用链复核，不声称新行为通过浏览器 | 实施者，修订后的 M1 出口 |
| 新 R″/C″、两图、真实安装与 new 工程出口 | 未来产物；本轮未安装、提交或发布 | 主实施者，M2 出口 |
| 第三方/编译器原文的成对反引号 | D4、§0.5、§11 已显式接受其歧义，本轮按该边界审查；不是“所有原始字符串无条件逐字显示”的证明 | 作者参考与最终交付说明保留范围限定；不扩大豁免到 F-03 的自有作者值 |
| 原 M6/M7、U-01–U-03、P3/P4 | 明确排除；本轮浏览器不能补足真机、正式性能或公开来源证据 | 原计划及相应期次承接 |

## 3. 覆盖与验证

### 3.1 实际分工与基线

3 个 subagent 首轮分别独立审查 facts、architecture、schedule；首轮结束后复用 schedule agent 审 delivery。delivery 是单独视角，因复用上下文不称为第四位全新独立 reviewer。主 agent 阅读所有候选的原计划和原始证据、去重裁定，并独占浏览器、报告与台账写入。未运行迁移、种子、部署或外部业务写入。

| 维度 | 已覆盖 | 结论/限制 |
| --- | --- | --- |
| facts | 三 crate 调用边、诊断传递、模板、审计与测试入口、原文矛盾 | F-01/F-02/F-04/F-06；未来产物未当作现状要求 |
| architecture | 编码可逆性、模块/特性边界、普通文本数据、标准消费者合同、R/C 路线 | F-01/F-02/F-03/F-05；两份 writer 的受控重复与 JSON/SSE 结构不变路线可以保留 |
| schedule | 需求→设计→检查→M1/M2、退出时点、提交点和共享资源 | 总拓扑成立；F-01 的新源码特性检查须前移，F-07 修审计口径 |
| delivery/UI | 安装→new→三入口诊断→面板读取→修复清除；设计约束与真实现状反例 | F-01/F-03/F-06；新行为是文档推演，当前行为有浏览器证据 |

读取并执行了 impeccable 的 `context --target crates/rustify-docs/assets/dev.js`，以 PRODUCT/DESIGN 和既有 dev 面板 brief 核对范围。维持原信息层级、主题、等宽片段、断行、焦点和恢复路径的方向合理；没有按个人视觉偏好报问题，也未执行重设计。

[provenance.json](../validation/docs-ssg-p2-gaps-2/plan-review-r1/provenance.json) 记录本轮 19 个相关输入文件的工作树 SHA-256。结束核对 HEAD、计划 hash 及相关产品路径无变化；这些未提交报告/证据尚未随 Git 共享，另一 checkout 仍须核实来源。

### 3.2 脚本结果

| 检查 | 实际结果 | 人工解释 |
| --- | --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P2-gaps-2.md <仓根>` | exit 0；16 个路径 token、3 个 MISSING | M1/M2 记录已标新增、实施后才建；`docs/project-status/protocol.md` 是兼容候选，当前采用 `.xgent-ai/sdlc/protocol.md`；均非虚构复用路径 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P2-gaps-2.md <仓根>` | exit 0；ERROR 0、WARN 1；0/2、两行里程碑、零完成记录 | dirty 警告来自既有未提交内容；初稿 0/2 与正文一致，没有据此推断完成 |
| `python3.12 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P2-gaps-2.md` | exit 0；ERROR 0、WARN 3 | WORK-F13/F15/N6 自定义编号已人工回读 PRD 的定义及验收 |

路径与进度脚本最初误用 `sh` 启动，因 Bash 语法报错；按脚本所需的 Bash 重跑后得到上表结果，未把首次错误归为计划缺陷。没有运行未来 M1/M2 的全量 Rust 测试、clippy、构建或安装；静态检查不证明行为完成。

### 3.3 本次真实浏览器证据

使用现有已安装 C′，`rustify-docs 0.1.17` / source `79a77825eadd07207154348ee9ffba48195d7766` / panic unwind；二进制 SHA-256 `47486f96ff0985f63d36590d13f5bbd7ddbcbba3d501f40f6f561f00be8c9d7c`。Playwright 驱动真实 Chromium `153.0.8010.12`；入口 `http://127.0.0.1:52008/docs/`，仅临时 content fixture，服务已停止。

1. 健康站点开始，0 error/0 warning；添加临时目标文件及含 percent-encoded 反引号的失效锚点链接。
2. SSE 中保留路径和 anchor 原文，面板丢字；在 1280/375 × 浅/深四种状态均复现，记录实际 `textContent` 与 `code` 列表。没有页面/面板横向溢出，console、page error、CSP 与失败请求均为空。
3. Hide 后焦点到 marker，Enter 重开、Esc 关闭并返回 marker；还原内容并删除临时页后 0 error/0 warning、面板隐藏。
4. 再输入空语言，取得 F-02 的真实诊断及旧面板结果。这里只确认旧行为与新合同的反例，**新方案未在浏览器中验证**。

原始状态与文字见 [browser-probe.json](../validation/docs-ssg-p2-gaps-2/plan-review-r1/browser-probe.json)；已查看[桌面浅色截图](../validation/docs-ssg-p2-gaps-2/plan-review-r1/baseline-ticks-1280-light.png)与[窄屏深色截图](../validation/docs-ssg-p2-gaps-2/plan-review-r1/baseline-ticks-375-dark.png)。同目录保存另两种状态截图。未以该内容站探针代替未来组件默认值、已安装 C″ 或全量 docs-dev 验收。

## 4. 需求映射与实施编排

| 需求/约束 | 已有映射 | 需要修订的链路 |
| --- | --- | --- |
| G-06/D-05、WORK-F13 | §5.1–5.3 → M1 V-1/V-2/V-3 → M2 V-4、固定 C″ 的 V-3/V-5 | F-02/F-03/F-04 补齐真实输入；不能只以解析后的同源期望判定保真 |
| WORK-F15 | 写法/三 crate/作者参考 → V-1/V-2 | F-01 特性、F-05 消费合同、F-06 验收触发 |
| WORK-N6 | 原位置、非零、dist/last-good、修复清除 → V-1/V-3/V-4 | 路线完整；修订后保留这些回归断言 |
| C-1 | 设计上下文 → M1 首轮浏览器 → M2 已安装入口复验 | 已有时点与责任，不需新增设计阶段 |
| C-2 | R″ 冻结 → C″ 审计 → 提交/安装 → new/浏览器 | F-01 前移新源码两图检查，F-07 单列既有 dirty |
| C-3 与上游 M6/M7/V-11/V-12/V-15 | §0.5 排除并由原计划承接 | 无范围偷渡；本增量结束仍不表示整 P2 或 WORK-F15 终验 |

保留两里程碑。依赖边为：编码/普通文本合同与 helper 特性位置确定 → 两份 helper 及红测试 → 生产方改写与面板/spec → M1 局部及集成检查（含新源码两图） → R″ 提交回填 → 五文件 C″ 制作与锁审计 → C″ 提交 → 隔离安装与身份固定 → new 工程 V-4、已安装入口 V-3/V-5 → 最终证据与进度收口。没有发现循环依赖，不需要增加里程碑。

| 波次 | 前置与可并行工作 | 写入/资源负责人 | 汇合验收 |
| --- | --- | --- | --- |
| 合同修订与 M1 起步 | F-01–F-06 的契约先确定；再补精确输入与失败基线 | 主实施者统一 helper 位置、公开接口与验证表 | 共享函数特性边和字面协议可执行 |
| M1 实施 | 契约稳定后，Rust 模板支线可与 `dev.js`/spec 支线并行；文档可独立编辑 | 每文件一个写入者；主实施者负责 `lib.rs`/共享 helper、Cargo 与 target；浏览器串行 | V-1/V-2/V-3/V-5、格式/分包检查及新源码两图全部通过，再冻结 R″ |
| M2 配对与出口 | 必须等完整 R″；制作→审计→提交→安装→new/浏览器顺序执行 | 主实施者独占锁、target、安装入口和浏览器；进度记录单写 | V-4 与已安装 C″ 的 V-3/V-5；五文件边界和既有 dirty 均可追溯 |

主实施者负责最终集成验收和计划回写。M2 没有值得单独并行的稳定独立出口；并行安装、锁审计与浏览器会混淆二进制/配对身份。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked。** 共确认 7 项：1×P1、6×P2。与计划自报 `Ready` 不一致。当前 blocker 是 helper 特性位置和逐字协议/生产方覆盖，不能靠实施退出门最终报错来替代设计决策；F-07 只阻塞 M2 审计。Ready 不涉及已经实现或公开可用的认定。

建议依次修订：

1. F-01：确定 helper 的跨特性可见性，把新源码两图验证放在 R″ 冻结前。
2. F-02/F-03/F-05：确定空内容、普通文本作者值和标准消费者边界，统一 writer/reader 与作者参考的承诺。
3. F-04/F-06：补完整片段审计与可达测试输入，保留真实命令及浏览器的独立精确期望。
4. F-07：修 C″ 五文件审计口径，保留既有 dirty 清单及证据/进度提交边界；维持 M1→M2 拓扑。

解除条件：修订后的计划不再要求实施者临时决定上述语法/特性边界，所有必需验收可由真实入口触发，五文件门可在保留当前工作树的情况下执行；随后按修订范围复核就绪。收集反例和整理测试输入可以先进行，不将原方案整体宣称 Ready。

下一步：

```text
$apply-doc-review docs/plan/DOCS-SSG-P2-gaps-2.review.md
```

本次只交付评审与事实索引，不自行修订计划或启动实施。


## 修订处置与复核

- 复核日期：2026-10-15 +08:00；首次 apply，之前无处置记录。本节接续评审对象 `d589882c-01b3-4031-b976-e2eedc754763`，不是新一轮独立评审。
- 来源报告原始 SHA-256：`4e4551251134fe70cf140cdcc0520d387fc447a3dd80dd7d9d5ca527ca73a333`；上方原问题、原结论与所审基线完整保留。
- 目标：[DOCS-SSG-P2-gaps-2](DOCS-SSG-P2-gaps-2.md)；修前 SHA-256 `b0391d7885e08d3ea7f0f0dc350bfc93ca9e1b4570ce2108763c9d77389cdef3`，与所审版本一致；修后 SHA-256 `e43ad686af27dccd77f197c459518afb78a3d582ab191d4f358fccb40252fcde`。
- 代码基线：`main` / `8a8268e6c5abb2d14a5bc7cdd67a7a1f2051debd` / dirty。亲自回读相关代码与既有浏览器探针；当前产品源码无 diff，index 无 staged 内容。保留既有台账、报告、证据、fixture 日志与缓存；本次只修计划、追加本处置并按协议登记台账。
- 范围：F-01–F-07 全部，以及 §2 的必要边界、§4 的需求/实施映射。当前目标身份与基线无歧义，不回退原文；实施仍 **0/2、尚未开始**。

| 问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 本轮复核结果 / 剩余动作 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。回读 [lib.rs](../../crates/rustify-docs/src/lib.rs)、[features](../../crates/rustify-docs/Cargo.toml)、[registry](../../crates/rustify-docs/src/site/registry.rs) 与[组件模板](../../crates/rustify-docs-cli/templates/components/Cargo.toml.in)：diagnostic 只在 pipeline 下存在，registry 则供独立 site-ssr/hydrate 使用；开启 pipeline 会引入 site-ssr，不能用于 hydrate。fixture 的 prepare 投影也不能验证新工作树 registry。 | 已修订并复核 | D3、§1.2、§5.2：新增 std-only `diagnostic_text.rs`，由 lib.rs 无条件公开 code_span/literal，保留 syntax 受控重复。§9.3/M1：沿现有 CI 前移工作树独立两图门；§1.2/§9.1 修运行时范围声明。 | **已解决（文档层）**。调用方可见性与依赖边闭合；没有通过强开 pipeline 修复 hydrate。新源码两图需实施后执行；M2 固定 R″ 的 new 工程两图门保留。 |
| F-02 | 成立。回读 [config.rs](../../crates/rustify-docs/src/config.rs) 的空语言拒绝链、[prose](../../crates/rustify-docs/assets/dev.js) 及[既有探针](../validation/docs-ssg-p2-gaps-2/plan-review-r1/browser-probe.json)：空串确实可达，旧单围栏写法会合并成一个串。 | 已修订并复核 | D1/D5、ADR-1、§4、§5.1/§5.3、V-1/V-3/V-4：空代码值写为两个反引号、两个 ASCII 空格、两个反引号；读回空串/空 code。无反引号输出不变明确限于非空代码值；普通文本空值仍原样。 | **已解决（文档层）**。空值有独立编码、不会吞并有模板分隔的后续片段；无分隔相邻代码值须合成一次编码。旧显示空值保持；真实空语言出口与两份 writer 一致性待实施验证。 |
| F-03 | 成立。[content.rs](../../crates/rustify-docs/src/content.rs) 的 broken-anchor 把 file 直接插在普通文本中，属于自有作者值，不能用第三方原文豁免。既有探针原始 SSE 与面板 textContent 可直接对照。用户 2026-10-15 已选择含反引号时整值显示为代码片段。 | 已修订并复核 | D2、ADR-1、§0.5/§1.2/§5.1–§5.4/§6、V-1–V-4：普通作者值统一经 literal，含反引号才调用 code_span；其余保持字节和样式。审计另沿自有诊断构造点追踪没有围栏的作者值。 | **已解决（文档层）**。普通文本作者值与正式片段共用同一语法；V-3 手写完整文件名和 code 列表，V-4 固定安装出口重放。外部原文边界未扩大，产品行为尚未验证。 |
| F-04 | 成立。回读 content 的 `#{anchor}`、generation 的 `#{id}`/`#{anchor}`、markdown 的 region/endregion：均可能漏过原正则。 | 已修订并复核 | D2、§1.1、§5.2、V-1/V-2/V-3：helper 接收含前后缀/多个插值/格式化内容的完整片段；搜索全部反引号并逐处分类，另审普通作者值。原统计只是下界。 | **已解决（文档层）**。明确包含所有已知漏网形式，不再以旧正则归零证明全覆盖；percent-encoded anchor 的显示期望为完整 `#` 加原 anchor，不能删除字面量反引号。实际全量审计在 M1。 |
| F-05 | 成立。本次直接读取 [CommonMark 0.31.2 §6.1](https://spec.commonmark.org/0.31.2/#code-spans)：标准会归一化行结束符，首尾空格裁剪也不同于原计划。逐字目标与原等价承诺矛盾。 | 已修订并复核 | D1/D5、ADR-1、§4、§5.1/§5.4、§7、V-1：明确 Rustify 自有逐字读法，保留空格和 LF/CRLF；标准 JSON 可解析与消息内容的读取规则分开。作者参考列完整规则和第三方边界。 | **已解决（文档层）**。撤掉 CommonMark/通用 Markdown 等价承诺，空格串、首尾空格、LF/CRLF 均有精确验证要求。没有把标准兼容当作实现验收项。 |
| F-06 | 成立。回读 [embed.rs](../../crates/rustify-docs/src/embed.rs) 的标识符闸、[markdown.rs](../../crates/rustify-docs/src/markdown.rs) 的失败即返回、[rendering.rs](../../crates/rustify-docs-cli/src/rendering.rs) 的成功声明分支：含反引号的 name 到不了 unregistered-component。 | 已修订并复核 | V-1：含反引号 name 预期 embed-payload；合法 MissingCounter 覆盖 unregistered-component；动态保真另用 YAML 未知键与真实 anchor。 | **已解决（文档层）**。输入、实际 rule 与预期结果能同时成立，不放宽名称语法或绕开真实入口；正式测试待 M1。 |
| F-07 | 成立。本次 git status/index 与[首个增量计划 §5.4/提交点](DOCS-SSG-P2-gaps.md) 核实：既有 tracked dirty 必须保留，原始工作树 diff 不会恰好五文件。 | 已修订并复核 | §5.5、V-4、§10 提交点：制作前保存 dirty/index 基线；审计本次 staged 五文件及拟提交产品树；相对 R″ 的零产品差异与纯进度/证据提交分开。 | **已解决（文档层）**。当前无 staged 内容，保留原有 unstaged 修改也能执行五文件门；只暂存本次相关 hunks，不清理用户差异。真实 C″ 提交/锁审计仍为 M2 门。 |

### 决策与未完项

- 用户 2026-10-15 在本会话交互问答中选择：“含反引号时显示为完整代码片段（推荐）：复用现有围栏规则，普通输入的文字与样式保持原样。”该选择与保留逐字目标、撤掉 CommonMark 等价承诺的提问上下文一并落实到 D1/D2、§5.1、§6 和 V-1–V-4。未选方案为保持普通文本样式而新增转义语法；本计划不引入该第二套语法。
- 无未采纳项、无待决项；7 项均为文档缺口的关闭。新增 helper、两份 writer、生产方改写、源码两图、真实浏览器、R″/C″ 配对/安装与 new 工程出口均未实施或运行，不计完成。
- 原 M6/M7、U-01–U-03、P3/P4 与公开发布仍由原计划承接。第三方/编译器原文的成对反引号仍可能歧义；本次没有把它们纳入逐字保证。
- 关联 PRD、原 P2/首个增量计划、覆盖复审报告、设计 brief 与作者参考正文未修改。作者参考的规则同步是修后 M1 的明确任务（§5.4），待代码与面板一起实现，不能提前写成当前已支持。

### 修订后结论与验证

**本次处置：7 项已修订并复核；无需修改 0、未采纳 0、未完/待决 0。修后计划 Ready，实施 0/2。** 这表示本报告指出的文档与开工阻塞已解除，不改写上方历史“需要修改 / 建议 Blocked”，也不表示新一轮独立评审、产品验收或整 P2 通过。

| 本次检查 | 实际结果与解释 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P2-gaps-2.md <仓根>` | exit 0；19 个路径 token、4 个 MISSING。diagnostic_text.rs 与 M1/M2 记录均为计划明确新增，实施时才建；兼容候选 docs/project-status/protocol.md 不存在，当前实际使用 .xgent-ai/sdlc/protocol.md。无虚构复用路径。 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P2-gaps-2.md <仓根>` | exit 0；ERROR 0、WARN 1；0/2、两里程碑、零完成记录。dirty/“计划没被改过”告警来自计划仍未跟踪及既有工作树差异；已按修前副本对照确认本次确实改稿，没有新增实施完成事实。 |
| `python3.12 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P2-gaps-2.md` | exit 0；ERROR 0、WARN 3。回读 PRD WORK-F13/F15/N6 的定义、分期和 AC；自定义 ID 有来源且原编号保留。 |
| 临时 Python 规则模型（仅 §5.1 合同推演） | 对 a、反引号、空格、LF、CR、反斜杠六字符的长度 0–5 输入穷举 9,331 项，分别核对 code_span/literal，共 18,662 条读回断言；另 10 个手写编码/显示/code 期望与 121 组有分隔的双片段组合均通过。未调用产品 writer/prose，不作为实现或浏览器通过证据。 |
| 回读与关联核对 | 对照修前副本，确认编号、0/2、实施协议与原有历史记录保留；新旧口径已在目标各重复位置同步。未启动计划内 Rust 测试、clippy、构建、安装或浏览器；本次新方案**未在浏览器中验证**，由修后 M1/M2 承接。 |

下一步仅建议，不自行启动：

```text
/goal 按 docs/plan/DOCS-SSG-P2-gaps-2.md 开发
```
