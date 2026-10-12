# DOCS-SSG-P1-gaps · 开发计划完整评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOCS-SSG-P1-gaps](DOCS-SSG-P1-gaps.md) | SHA-256 `38e4f3035e094c437b12aa58c3dc246300315a08f55eb17a96409a1837987a27` | 全文；M1/M2、V-1–V-7及所引用需求与原M9交接 |

评审日期：2026-10-12 +08:00。评审对象 ID：`5539f718-6df5-42fc-aeec-f8c35dd42147`。这是本计划的首次完整计划评审，非原计划代码评审或差异报告的修复复审；背景报告不是本轮自动修订目标。

## 1. 确认问题

### [P2] F-01 · 把首页 hero 与未合并的正文 H1 纳入同一标题合同

- **定位**：[计划 §5](DOCS-SSG-P1-gaps.md#5-核心机制)，第148–150行；§9/V-1、V-2，第188–189行。关联 G-02/G-03、WORK-N5/AC-3、M1。
- **问题**：计划指定统一 Markdown/raw HTML 的有效标题统计，同时明确首页只合并开头 H1，无开头 H1 时保留既有首页标题行为。正文在导语、注释或容器之后有唯一作者 H1 时，片段只统计到一个，固定生成的 hero 又贡献一个。仅实施片段统计和开头 id 转交，无法满足最终页面单 H1。计划没有明确这类未合并 H1 是定位拒绝、合并还是采用其他保持单 H1 的输出；实施者仍需决定首页壳与内容分析的组合规则。V-1着重已合并标题，V-2没有这条壳与正文组合的反例。
- **证据**：[generation::generate](../../crates/rustify-docs/src/generation.rs) 第562–571行只在 `article.starts_with("<h1")` 时删除标题；[home_layout](../../crates/rustify-docs/src/shell.rs) 第418–425行固定生成 hero H1并输出剩余正文；[content::analyze](../../crates/rustify-docs/src/content.rs) 第864–903行按 fragment 统计。PRD [WORK-N5](../PRD-DOCS-SSG.md#work-n5-可访问性与阅读体验) 第711行要求每页恰有一个一级标题。本轮[隔离反例](../validation/docs-ssg-p1-gaps-plan-review/probes.json)中，`layout: home` / `title: Home` 加“导语后 Markdown H1”“注释后 Markdown H1”“容器内 raw H1”，三组 `check/build` 均 exit 0，产物均为 hero 与正文两个 H1；[真实浏览器证据](../validation/docs-ssg-p1-gaps-plan-review/checks-and-browser.json)与[桌面截图](../validation/docs-ssg-p1-gaps-plan-review/desktop.png)确认实际 DOM 两个 H1。现状反例支持的是计划遗漏的组合边界，不是要求尚未实施的新机制现在就通过。
- **最小修订**：在§5明确 hero 参与最终标题序列与单 H1 校验，写明未被既有合并规则处理的作者 H1 的结果及源码诊断归属；V-2加入导语、注释、容器/大小写 raw H1 的首页组合。可以沿用只合并开头 H1并定位拒绝其他造成双 H1的组合；现有[CLI analyse](../../crates/rustify-docs-cli/src/main.rs) 第261–305、353行已让check/build共用真实生成，无需扩大合并范围或新建渲染流水线。
- **复核**：改稿后能从正文确定上述输入的唯一结果。实施后，接受的页面只有一个实际 H1且保留有效锚点；拒绝的输入在 `check/build/dev` 给出一致、可定位的诊断并保全旧产物。原开头标题的自动/显式 id 转交继续通过。
- **置信度**：高，代码分支与隔离产物、实际 DOM相互印证。**阻塞范围**：M1首页标题接线与V-2出口；诊断定位等独立准备不受此项阻塞。

### [P2] F-02 · 明确无 actions、无顶栏链接首页的真实导航兜底

- **定位**：[计划 §0.2/P-4](DOCS-SSG-P1-gaps.md#02-决策表)，第90行；§5第5项，第152行；§9/V-2，第189行。关联 G-01、WORK-N5/AC-3、M1。
- **问题**：计划选择让首页“已有主阅读操作”承担 navigation，但 `actions` 可以省略，顶栏/社交链接也可以全空。该合法组合没有所选的操作可包装。全页面四地标的目标已经明确，但这条具体分支的导航来源与验收没有闭合；只包装现有 actions，再在既有 fixture 上扩四地标断言，仍可遗漏零动作首页。空 nav/role又被计划明确排除。
- **证据**：[Front](../../crates/rustify-docs/src/frontmatter.rs) 第109–117行允许空 actions；[generation](../../crates/rustify-docs/src/generation.rs) 第574–584行直接传递它们；[home_layout](../../crates/rustify-docs/src/shell.rs) 第422行仅非空时输出动作区，`header` 第485–490行仅非空 entries 时生成导航。现有[首页 fixture](../../tests/docs-ssg/fixtures/content/content/index.md) 第6–10行具有两项 actions；[theme.spec](../../tests/docs-ssg/theme.spec.ts) 第357–369行只检查当前 fixture 的 H1/banner/main。V-2列有/无 git/edit/top，没有零 actions 与空 top/social 的组合。本轮 `no-actions` 隔离反例 `check/build` 均 exit 0、nav为0；实际走查的导语首页也同时满足无 actions/无 top/social，可访问树没有 navigation。记录见[probes](../validation/docs-ssg-p1-gaps-plan-review/probes.json)与[浏览器记录](../validation/docs-ssg-p1-gaps-plan-review/checks-and-browser.json)。这区别于 G-01 已知的当前缺地标：本问题是所选修复来源及新增验收仍依赖可空操作。
- **最小修订**：在P-4/§5.5确定该组合的兜底来源，例如复用已经规划的站点页脚返回入口，组成有真实链接与可理解名称的导航。保留 actions 可选，不增加产品能力。V-2补空 actions、空 top/social 的首页，在两 base、宽窄及无脚本状态查实际角色和链接。
- **复核**：改稿明确空配置时采用的入口与语义。实施后该合法组合具有实际可访问的 navigation、可用链接及其余地标；有 actions 的首页保留既有操作与阅读层级，不能靠空角色节点通过。
- **置信度**：中到高，输入合法性和现有 fixture 覆盖已核实；全页面目标能指导实现，但不能替代已选择来源在空配置分支的定义。**阻塞范围**：M1导航接线与V-2出口。

### [P2] F-03 · 将修后桌面 Chrome 稳定版验收接入 M2

- **定位**：[计划 §6](DOCS-SSG-P1-gaps.md#6-前端与真实浏览器)，第159行；§9/V-4、V-6，第191、193行；§10/M2，第203行。关联 WORK-N3/AC-4、C-3/C-4、M2。
- **问题**：V-4指定当前 docs Playwright 项目，§6/V-6具体安排 macOS Safari、iPhone Safari与Android Chrome。桌面 Chrome 最新稳定版没有明确的修后候选检查、版本核对与出口归属。即使这几项全通过，M2仍可能缺这项有效合同。D-02只冻结性能浏览器，D-01只豁免Firefox，不能把两者视为桌面Chrome兼容范围变化。
- **证据**：PRD [WORK-N3](../PRD-DOCS-SSG.md#work-n3-静态部署严格-csp-与浏览器兼容) 第657、672行要求每期出口按当期能力复核桌面 Chrome最新稳定版；[原计划 §9.3](DOCS-SSG-P1.md#93-性能千次保存与真实环境合同) 第452行明确冻结性能 Chromium与兼容性稳定版分开登记，Firefox后移后的矩阵仍包含Chrome。现有[Playwright 配置](../../playwright.docs.config.ts) 第75–78行只采用 `Desktop Chrome`设备参数和可选 `RUSTIFY_CHROMIUM`路径，未指定品牌稳定Chrome；允许覆盖路径不等于计划已安排该检查。[M8记录](../validation/docs-ssg-p1/M8.md) 第68行明确新页头的真实Safari/手机/Chrome走查未覆盖；本计划又要改首页和全页导航/页脚，旧Chrome证据无法直接覆盖新壳。计划第22行的通用受影响回归约定保留该义务，但具体 V-4/V-6/M2未接入；原M9亦未明确承担这条新候选兼容检查。
- **最小修订**：在§6/V-6加入桌面 Google Chrome最新稳定版对同一修后 production产物的主路径与受影响边界走查，登记实际版本、产物指纹与结果；也可明确由V-4另用经核对的稳定Chrome可执行文件执行对应检查。冻结Chromium继续按原性能合同使用。无需新增里程碑、恢复Firefox或升级Playwright。
- **复核**：改稿后 M2不能在缺桌面稳定Chrome证据时宣称V-6全部通过；实施记录能够分别定位性能Chromium、桌面稳定Chrome与Safari/手机的范围、版本及同产物结果。
- **置信度**：高，需求与具体执行范围可逐项核对，facts/architecture针对性复核后未找到足以补齐的出口条款。**阻塞范围**：M2兼容性出口；不阻塞M1代码开发。

## 2. 待核实项与保留的验证门

| 项目 | 缺失证据 / 影响 | 提供者与最晚确认点 | 本轮裁定 |
| --- | --- | --- | --- |
| U-01真实设备 | iPhone/Safari、Android/Chrome、macOS Safari的实际版本、走查人和同候选可达入口 | M2实施者协调用户或可用真人设备；V-6前 | 计划第159、193、211行已正确登记。缺环境阻塞M2出口，不另报遗漏，也不凭桌面375px截图关闭 |
| 有效树到源码位置 | 混排、重复文本、template隐藏子树及foreign integration能否可靠对应原文span | M1实施者；V-2/V-3出口前 | `html5ever/dom_query`、token span、LineIndex原料存在；新增关联属于第212行已登记实现风险，没有要求既有代码现在就具备新机制 |
| 修后性能/失败保全 | 新语义提取、壳与资源改变后的冻结B五样本数据、旧产物及last-good恢复 | 主实施者；M2/V-5前 | 预算与测试入口已定义，本轮没有重跑未来候选性能，不把历史R5数字算成本次通过 |
| 原M9最终候选 | 修复后的确定SHA、安装后二进制、GitHub同源预览与远端交付 | 原M9实施者；原M9出口前 | 排除范围和M2候选交接有效；gaps完成不等于P1安装/远端交付完成 |

未发现需重新选择产品范围、采购指定设备、新增GPU能力或恢复Firefox期次的决策。本轮三个修订均可在现有模块、检查与里程碑内完成。

## 3. 覆盖、基线与实际验证

调查事实源是 `main` / `aabcd95810d4036bd4203aea87a0934c8d50f603` 当前工作树。开工已存在报告、证据、经验及SDLC dirty资料，目标计划未跟踪；产品源码已提交。未 fetch/pull、提交、推送或部署，也未实施本计划。输入指纹见[baseline-start](../validation/docs-ssg-p1-gaps-plan-review/baseline-start.json)；[baseline-end](../validation/docs-ssg-p1-gaps-plan-review/baseline-end.json)确认113个相关源码、配置、需求与计划输入均未变化，HEAD与目标计划hash一致。所审工作树资料与本轮产物尚未通过Git共享。

| 维度 | 实际负责人 / 方法 | 结论与限制 |
| --- | --- | --- |
| facts | 独立 `facts` agent，源码与历史原始数据核查；主agent逐条回读 | 四类既有缺口、锁定工具链、CLI0.1.2、R5最大内容26388B及build中位422.7ms有依据；F-01及F-03证据复核 |
| architecture | 独立 `architecture` agent；针对性复核Chrome链路 | 沿用SSR、实际HTML树、原span/Diagnostic、writer失效和整代发布合适，不需新依赖/架构抽象；F-01边界需明确 |
| schedule | 独立 `schedule` agent，双向映射及依赖核查 | 0/2、两里程碑、跨期终验和M9交接成立；具体边界验收补齐见F-01–F-03 |
| delivery | schedule完成后复用该agent审第四视角，另交delivery结果；主agent为唯一浏览器操作者 | 作者失败恢复与读者主路径基本闭合；F-02空配置与F-03兼容出口未闭合；未来修复尚未验收 |

调度按四个总槽位先并行 facts/architecture/schedule，再复用空闲agent审delivery。初轮独立调查，主agent保留裁定职责；没有以多名reviewer同意替代源码、原文与运行证据。

UI按 `impeccable` 读取 PRODUCT.md、DESIGN.md、[文档主题规范](../../DESIGN.docs-ssg.md)及[Read brief](../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md)，执行一次 `context --target crates/rustify-docs/src/shell.rs`。这里只核对既定阅读层级、入口、语义、主题和状态，不进行产品重设计。静态内容继续用DOM/Leptos SSR，基础交互用现有JS；GPU路径不在本次增量。

### 3.1 脚本与命令

完整结果见[checks-and-browser](../validation/docs-ssg-p1-gaps-plan-review/checks-and-browser.json)。

| 检查 | 实际结果 | 人工判读 |
| --- | --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P1-gaps.md <仓根>` | exit 0；检查2个路径token，1个MISSING | 缺的是顶部协议中兼容旧台账的 `docs/project-status/protocol.md`；当前有效`.xgent-ai/sdlc/protocol.md`存在，不是错误复用。脚本提取范围很小，拓扑路径另人工核对 |
| 同目录 `check_progress.sh`，同参数，以Bash执行 | ERROR 0 / WARN 1；0/2，两里程碑、记录0行 | dirty资料告警不代表已实施；恢复快照和空完成行与事实一致 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P1-gaps.md` | ERROR 0 / WARN 8 | 8项为WORK编号人工核对提示，已对上PRD及本期落点；不是设计通过证明 |
| 当前CLI隔离输入 | 五组check/build均exit 0；三个非开头H1组合输出双H1；零actions输出无nav | 使用已有docs-cli二进制，SHA-256 `f281a1bd80c44f7032a129bfd93cef37df4b3abdf34aed6de43d08a9282aebb7`，与差异核查binary记录一致；仅在独立临时目录写fixture/产物 |

脚本初次误用 `sh` 因Bash进程替换语法退出2，检查shebang后改用Bash，以上是最终有效结果；未把调用失败算成计划缺陷。本轮未执行全库Rust测试、Clippy、全量Playwright、性能或故障注入；这是计划评审，当前行为反例与未来实现验收严格区分。

### 3.2 实际浏览器证据等级

通过 `kimi-webbridge` 使用真实桌面Chrome访问上述隔离production产物；UA显示Chrome 154，未独立证明它是执行时最新稳定版。原生 `Browser.getVersion`调用被通道拒绝，没有据此补写精确版本或稳定版认证。

入口由现有CLI `preview --port 0`给出：`http://127.0.0.1:63742/`；构建ID `77850e72c34aa572`，CLI0.1.2。查看实际可访问树、DOM及[1512px截图](../validation/docs-ssg-p1-gaps-plan-review/desktop.png)、[375px截图](../validation/docs-ssg-p1-gaps-plan-review/narrow.png)：首页有两个H1，无navigation/contentinfo，缩窄后页面宽度与scrollWidth均375。执行切深色→深缺失地址→真实404→返回首页，主题保持；捕获404主请求与三个200静态资源。404及返回首页的console warn/error、pageerror、unhandledrejection观察数组均为空；初始首页加载尚未安装观察器，未宣称其console已完整检查，初始资源记录另出现favicon请求。

这属于**既有基线的实际页面验证**，支持候选边界的现状证据；未实施的新地标、标题统一和诊断仍是文档推演。未在Safari、真实手机、无脚本上下文或完整原站主路径复验，375px为桌面浏览器缩窄。临时preview已停止，浏览器组按用户关闭约定保留。不能用这些结果关闭V-6或宣称gaps UI已验收。

## 4. 需求映射与实施编排

已核对本计划账本的8个WORK条目及C-1–C-4：G-01–G-04有M1实现与检查归属，G-05/G-06/U-01由M2收口，原M9另承担安装与远端，F15/N1/N2/N3/N5的终验期仍按PRD§7.3保留。没有必要重复需求表或增加内部任务文档。断链在以下三个具体分支：

| 需求/分支 | 需要补齐的链路 | 对应问题 |
| --- | --- | --- |
| N5/AC-3：首页hero + 非开头作者H1 | 最终标题组合规则→源码诊断/输出→V-2实际DOM | F-01 |
| N5/AC-3：零actions、空top/social首页 | 有效导航来源→空配置产物→V-2实际角色与入口 | F-02 |
| N3/AC-4：桌面Chrome稳定版 | 同修后候选→明确实际稳定浏览器入口→V-6/M2出口 | F-03 |

| 顺序/波次 | 前置与解锁证据 | 可并行工作及共享负责人 | 汇合/出口 |
| --- | --- | --- | --- |
| 修订本轮合同 | 处理F-01/F-02的首页规则，F-03加入现有M2检查 | 不需要新增里程碑；原计划/源码保持只读直到另行授权修订或实施 | 计划能唯一确定反例结果与每项检查归属 |
| M1反例→局部实现 | 当前入口可复现；先保存红证据，再推进相关修复 | 主实施者独占Fragment/content/generation/shell公共接线；可同时协调设备/真人/入口。四项共享模型，硬拆并行修改会增加集成风险 | V-1/V-2/V-3与相关V-4/V-5；统一标题、导航、诊断及保全回归 |
| M2最终候选与文档 | M1必要局部检查通过，候选契约稳定 | 主实施者单写文档/记录；可准备V-7及协调真机。Playwright、性能、浏览器会话串行使用 | 同候选V-4/V-5、修订后的V-6、V-7全部通过；U-01真实证据齐 |
| 原M9后继 | M2交接实际版本/源码SHA和候选 | 原M9实施者负责；本地安装先验，远端遵原授权门 | 原M9安装/仓外/远端检查；不计入gaps的2里程碑 |

原M1→M2线性依赖与集中收口成立；环境准备可以与M1并行。没有依据编造工期或收益比例，也无需因某部分前置解锁增加里程碑。最终集成人与回写负责人为主实施者，真实设备操作者只提交可核验结果。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

建议状态：**Blocked（首页标题/空配置导航合同待补齐；M2兼容出口需修订）**，与作者自述Ready不一致。原因是本期明确的最终单H1、全页有效导航和稳定版兼容承诺还没有被所选分支与出口完整承接，不是因为新功能尚未实现或未来真机尚未准备。独立源码定位、反例准备与环境协调可先进行；本范围不能在这些合同补齐前整体宣称Ready。

先修F-01的最终标题组合及真实源码诊断规则，再修F-02的可空操作导航来源，两者一并加入V-2；随后将F-03的桌面稳定Chrome接入既有§6/V-6/M2。三项均有确定最小修订，不必重开产品范围、修改性能阈值、引入新依赖或重复原M9。修订文本能唯一确定边界行为与检查归属后，可复核为Ready；未来真实设备仍是实施期已登记的M2出口门。

本次仅交付独立报告、必要证据与SDLC评审事实，不自动改计划、实施、改变进度或启动下阶段。

```text
$apply-doc-review docs/plan/DOCS-SSG-P1-gaps.review.md
```

## 修订处置与复核

- 来源：本报告原始 SHA-256 `054368adabaabce909987833181c82971f6bf93bd0debb393bed87c89d7a9422`；评审对象 ID `5539f718-6df5-42fc-aeec-f8c35dd42147`。上方原问题、证据、所审基线和总体结论保留；本节是同轮文档处置，不是新的独立评审。
- 目标：[DOCS-SSG-P1-gaps](DOCS-SSG-P1-gaps.md)，计划对象 ID `7c105838-be0d-4f3b-ae5c-9998b8f4cfdc`；修前 SHA-256 `38e4f3035e094c437b12aa58c3dc246300315a08f55eb17a96409a1837987a27`，与所审版本一致；修后 SHA-256 `ee5104ccd5ec6577dc157f27a638092726125ea40d86dd95d557f8cfcede857b`。
- 复核日期与代码基线：2026-10-12 +08:00，`main` / `aabcd95810d4036bd4203aea87a0934c8d50f603` / dirty。既有未提交报告、证据和台账保留；计划与本报告仍未跟踪。事实来自当前源码及亲自回读的原始资料，未将旧运行结果登记为本次实测。
- 本次范围：全部 F-01–F-03、其需求/检查/里程碑关联，以及§2四项保留验证门。无上次处置；关联PRD、原计划和产品实现只读。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。[generation](../../crates/rustify-docs/src/generation.rs)只删除符合开头规则的H1，[home_layout](../../crates/rustify-docs/src/shell.rs)固定输出hero，[content::analyze](../../crates/rustify-docs/src/content.rs)只按Fragment统计；[原始probes](../validation/docs-ssg-p1-gaps-plan-review/probes.json)的paragraph-before、comment-before、wrapped-raw均记录两个实际H1。PRD N5/AC-3约束最终页面，源码当前接受不等于标题结构合规。 | 已修订并复核 | 计划P-2、§4首页SSR、§5.1/§5.3与V-2：组合生成标题及剩余作者标题；仅既有开头H1合并并转交id，其他有效作者H1定位报multiple-h1，复用check/build/dev诊断与失败保全。 | 已解决（文档）。无作者H1保留hero；已合并H1保留原id；导语/注释后、容器内及未匹配开头规则的大小写raw H1拒绝；隐藏模板与代码/注释文本排除。上述输入有唯一结果，M1仍须补红绿、源码位置和实际DOM证据。 |
| F-02 | 成立。[Front](../../crates/rustify-docs/src/frontmatter.rs)的actions默认可空，[header/home_layout](../../crates/rustify-docs/src/shell.rs)在空链接/空actions时没有所选导航来源；原始probes的no-actions记录navigation为0。[Site::url](../../crates/rustify-docs/src/shell.rs)已有基路径下真实返回入口，可承接已规划的站点页脚。 | 已修订并复核 | 计划P-4、§5.5与V-2：全页main之外的站点页脚提供有名nav和site.url("/")普通链接，零actions且top/social全空仍有navigation；actions保持可选，有动作时保留主阅读导航。 | 已解决（文档）。兜底不依赖git/edit/top/social，多个导航名称可区分；V-2明确两base、宽窄与无脚本的实际角色及链接验收。实现与a11y树效果留给M1/M2，不靠空nav关闭。 |
| F-03 | 成立。亲自核对PRD N3/AC-4与[原计划§9.3](DOCS-SSG-P1.md#93-性能千次保存与真实环境合同)，桌面Chrome稳定版与冻结性能Chromium分别承担兼容性和测量；[Playwright配置](../../playwright.docs.config.ts)的Desktop Chrome参数及可选executablePath并未自行安排品牌稳定版检查。 | 已修订并复核 | 计划WORK-N3、§6、V-6、M2与§11：桌面Google Chrome最新稳定版对同修后production产物走主路径及首页边界；核对品牌、完整版本和执行时稳定渠道依据，分别登记四个实际浏览器/设备组合。 | 已解决（文档）。缺桌面稳定Chrome或任一Safari/手机组合证据时V-6不能全通过、M2不能完成。V-5仍按原冻结Chromium153测量；未升级依赖、恢复Firefox或增加里程碑。 |

### 决策与未完项

三项采纳其最小修订方向，没有需用户重新选择的产品范围或架构方案。首页沿用既有开头合并范围，拒绝的是违反既有最终单H1要求的组合；没有新增禁用raw HTML的规则，也没有改成跨导语/容器提取标题。空首页复用已规划的页脚返回入口，不要求作者新增actions。Chrome兼容检查恢复原有合同的明确归属，D-01–D-03继续有效。

| 保留验证门 | 当前状态与归属 | 解除条件 |
| --- | --- | --- |
| U-01真实设备/走查人 | 实施出口待核实；计划§6/§11与M2/V-6保留，现已明确桌面稳定Chrome另留证据 | 执行前确认Safari/两手机组合与稳定浏览器环境、操作者和可达入口；同修后候选完成实际版本/步骤/结果，缺环境阻塞M2出口 |
| 有效树到源码位置 | M1实现风险仍在；本次只确定最终组合与诊断归属，没有证明span关联已实现 | V-2/V-3验证混排、重复文本、template与foreign边界以及check/build/dev一致定位 |
| 修后性能/失败保全 | M2/V-5待执行；未把历史R5数字算成修后通过 | 同冻结B候选五样本测量与失败保全/last-good恢复通过，保留前后数据 |
| 原M9最终候选 | 继续排除于本计划两里程碑；原计划与历史记录未改 | M2交接确定候选版本/SHA；原M9另验安装后二进制、仓外及GitHub同源/远端交付，遵原授权门 |

关联PRD的Firefox本期例外与冻结测量口径、作者参考/设计sidecar的修后规则，以及原计划恢复入口仍由M2/V-7和原M9同步。本次没有越界改这些原文，也不把同步工作记作已完成。

### 修订后结论与验证

本次须改项：**3项已修订并复核，0项无需修改，0项不采纳，0项文档待决或待补证**；四项未来验证门继续保留，不计为本次验收通过。文档就绪判断：**Ready**。首页规则、空配置导航和桌面稳定Chrome出口已闭合，M1可按既有入口推进；本计划实施仍0/2、尚未开始，恢复快照和完成记录逐字保留。原计划仍8/9，独立复审、产品验收及上线完成均未由本次处置证明。

| 本次实际检查 | 结果与解释 |
| --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOCS-SSG-P1-gaps.md <仓根>` | exit 0；2个路径token、1个MISSING。缺项是实施协议里的旧台账兼容路径docs/project-status/protocol.md，当前.xgent-ai/sdlc/protocol.md存在，不是错误复用；代码拓扑与本次引用另人工核对。 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOCS-SSG-P1-gaps.md <仓根>` | exit 0，ERROR 0 / WARN 1，0/2、两里程碑、完成记录0行。告警来自dirty资料及目标未跟踪：脚本ls-files不能识别其本次改稿；不代表已有实施未回写。 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOCS-SSG-P1-gaps.md` | exit 0，ERROR 0 / WARN 8；八项WORK自定义编号已回读PRD核对，需求→设计→V-2/V-6→M1/M2连续，7个检查与2个里程碑的ID未改变。 |
| 回读、链接与版本比较 | 核对三项输入的接受/拒绝及诊断归属、零配置链接与基路径、四组合证据缺失时的M2出口；本地链接目标存在。关联PRD与原计划hash未变，实施进度未改，原报告保留于追加节之前。临时范围检查最初误计上游M9与代码示例链接，按实际章节和Markdown语义限定后通过，未据此修改模板。 |

上述检查只证明本次文档修订与关联一致性。**本次未在浏览器中验证修后UI**：任务未实施，暂无修后候选；未运行业务Rust测试、构建、Playwright、性能、故障注入或真机走查，也未提交/推送/部署。原报告中的浏览器与反例记录只作为已回读的历史证据。

建议下一步（不自动启动）：

```text
/goal 按 docs/plan/DOCS-SSG-P1-gaps.md 开发
```
