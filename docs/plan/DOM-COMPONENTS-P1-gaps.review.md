# DOM-COMPONENTS-P1-gaps · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P1-gaps](DOM-COMPONENTS-P1-gaps.md) | SHA-256 `872b6cb36176238bbf712fa89055e5dc0283b17c91822d3dae81071cfa571e9a` | 全文；G-01/G-02/V-01 增量、M1–M2、V-1–V-3 |

审查日期：2026-10-29（Asia/Shanghai）。本轮是该增量计划的首次完整评审；[原 P1 差异报告](DOM-COMPONENTS-P1.prd-dev-gaps.md)是需求依据，不是本轮修复复审对象。评审 ID：`9152677e-02be-4557-ac77-d2fb2244a90a`。原计划与产品源码保持只读。

## 1. 确认问题

### [P2] F-01 · 独立消费入口缺少生成器必需的源码修订输入

- **定位**：[计划 §1.1、§1.2](DOM-COMPONENTS-P1-gaps.md#11-现状与缺口) L239–250、§4 L261、[§9.2](DOM-COMPONENTS-P1-gaps.md#92-可执行命令与范围) L341；关联 V-3、M2。
- **问题**：计划把独立 DOM fixture 列为已核实足够，并直接执行 `sh tests/theme-export/build.sh --dom-only`。在没有额外设置环境变量的 shell 中，当前生成器无法编译；计划也未安排补齐这个构建接缝。M2 因而无法生成本版独立消费者，不能完成 WORK-F20/AC-3 的收口。
- **证据**：[generator/main.rs](../../tests/theme-export/generator/src/main.rs#L22) L22–30 用 `include!` 编译整份 Studio `data_tools.rs`；[data_tools.rs](../../examples/theme-studio/src/data_tools.rs#L288) L288–295 在 host 路径使用 `env!("RUSTIFY_STUDIO_SOURCE_REVISION")`。该变量由 [Studio/build.rs](../../examples/theme-studio/build.rs#L1) L1–12 注入 Studio 包；[generator/Cargo.toml](../../tests/theme-export/generator/Cargo.toml)没有构建脚本，[build.sh](../../tests/theme-export/build.sh#L26) L26–28 也没有向生成器提供它。根 `.cargo/config.toml` 仅定义 xtask alias。
- **实测**：主 agent 执行下列最小检查，退出 **101**，编译器直接报告 `data_tools.rs:293:21` 的该环境变量未在编译期定义。此检查没有运行生成器，也未改动历史证据。

  ```sh
  mise exec -- env -u RUSTIFY_STUDIO_SOURCE_REVISION mbx check --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bin theme-export-generator
  ```

- **最小修订**：在 §4 将当前独立重建前提更正为缺口，明确 fixture 从同一 checkout 获取修订的具体入口。可以只在 §9.2 的构建命令中显式注入实际 Git 修订；若选择在 `build.sh` 或 generator 构建文件中处理，就同步纳入 §1.2/M1 的允许改动与验证。不能依赖实施者 shell 中偶然已有的变量，也不能把旧产物可运行当作当前源码可重建。
- **复核**：改稿后核对变量从取得到生成器编译的完整传递；实施后从未预设此变量的 shell 运行所定入口，完成 `--dom-only`，并核对 Studio、配套资料与 fixture 的源码修订一致。
- **置信度：高**，有代码与编译器反例。**阻塞：M2/V-3 的独立重建；路线和改动边界应在进入相关实施前补齐。**

### [P2] F-02 · 接入说明函数拿不到承诺输出的当前预览模式

- **定位**：[计划 §5.1](DOM-COMPONENTS-P1-gaps.md#51-各格式共用的说明) L277–284 与 [§5.2](DOM-COMPONENTS-P1-gaps.md#52-当前-api-的单一说明口径) L292；关联 WORK-F20/AC-3、V-1/V-3、M1。
- **问题**：计划固定 `integration_notes(document, context)` 两参，同时要求屏内和配套文本的调用都显示当前实际 mode。保持作者文档和解析 context 不变，仅将 Studio 预览从 Light 切为 Dark 时，这个函数收到完全相同的输入，无法输出不同的当前模式。实施者只能自行改变已定签名，或生成不能复现所选预览的说明。
- **证据**：[ResolveContext](../../crates/rustify-dom/src/theme/resolve.rs#L37) L37–43 只有 rem、motion、字体与字体解析信息，没有预览模式；[DataTools](../../examples/theme-studio/src/data_tools.rs#L758) L758 将 `preview_mode` 作为独立 signal 接收，现有提示在 L1164 单独读取它。[provider_snapshot](../../crates/rustify-dom/src/theme/provider.rs#L30) L30–35 明确区分 preview 与作者 appearance，不能从文档的 appearance 倒推当前预览。真实 [mount_exported_preview](../../examples/theme-studio/src/data_tools.rs#L371)也要求显式 mode 参数。
- **最小修订**：为局部函数补充 `preview_mode: ThemeMode`，明确显示和下载时从现有 signal 传当前值。保留原定的低频生成、两模式资源解析、JSON 字节及作者历史不变；不需要新增资料模型或修改公共 SDK API。
- **复核**：V-1 对同一 document/context 分别传 Light/Dark，断言完整四参调用的 mode 正确；V-3 只切 preview 再下载，核对资料随当前选择变化，作者 JSON 和历史不变。
- **置信度：高**，输入类型和模式所有权均已核实。**阻塞：M1 的接入说明函数与调用契约。**

### [P2] F-03 · 独立消费者验收命令会写回原 P1 证据目录

- **定位**：[计划 §9.2](DOM-COMPONENTS-P1-gaps.md#92-可执行命令与范围) L347；与 C-3（L204）和 [§12](DOM-COMPONENTS-P1-gaps.md#12-已知坑与历史约束) L380 的历史证据保护约束冲突；关联 V-3、M2。
- **问题**：该命令没有设置独立消费者 spec 识别的输出参数。按原文运行八种 CSS 和一个 DOM 用例，会把新结果写入旧 P1 目录，并覆盖其中固定名称的 JSON/截图。L340 的 `RUSTIFY_UI_EVIDENCE_OUT` 只属于前一条命令，且 runtime spec 不读取这个变量。
- **证据**：[theme-export-runtime.spec.ts](../../tests/browser/theme-export-runtime.spec.ts#L4) L4 只读取 `RUSTIFY_THEME_EXPORT_OUT`，缺省为 `docs/validation/dom-components-p1/export`；L46–47 写 CSS JSON/截图，[L127–132](../../tests/browser/theme-export-runtime.spec.ts#L127)写 DOM JSON、桌面与手机截图。旧目录中已有 `theme-export-css.png`、`theme-export-dom.json`、`theme-export-dom.png` 和 `theme-export-dom-mobile.png` 等同名文件。此行为由写入路径即可确认，本轮未执行该浏览器命令。
- **最小修订**：在 L347 的命令前增加 `RUSTIFY_THEME_EXPORT_OUT=docs/validation/dom-components-p1-gaps/export`。无需改测试实现；本增量的取证路径统一沿用新目录。
- **复核**：改稿时逐一对照会落盘的 spec 与其实际环境变量；实施时确认新证据落入增量目录，历史证据未因本次命令改变。
- **置信度：高**，环境变量、写入位置及既有目标均已核实。**阻塞：M2 对应命令执行前；不阻塞独立的文档修改。**

## 2. 待核实项与边界

没有发现需要用户重新决定的产品取舍或新增外部服务前置。上述三项均是已确认缺口，不能再归为“实施时调查”。

| 尚待验证内容 | 影响/提供方 | 最晚确认点 |
| --- | --- | --- |
| 新 `theme-setup.txt`、默认 JSON 公共说明及模式/context 内容尚未实现 | 实施者提供真实下载和独立角色消费结果；现有 fixture 通过不能证明新说明足够 | M2/V-3 |
| 三个存储用例改为真实反馈区域后，仍须跑到数据、草稿、库及重试断言 | 实施者在最终同版 release 执行原失败测试；本轮未重跑这些故障场景 | M2/V-3 |
| 新增长资料后的双语、390/768/桌面/200%、键盘回焦和失败出口 | 实施者按 §6/§9 真实浏览器验收；本轮已有页面走查不能替代 | M2/V-3 |

这些属于计划已有的实施后验收，不因功能尚未实现而单独判缺陷。历史 U-01 性能归因及人工辅助技术/真机等限制保留原报告去向，不扩大本次范围；没有把它们记为已关闭。

## 3. 覆盖与基线

### 3.1 调查基线

- 代码：`main` / `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e`；dirty 工作树是文档事实源。相关 `crates/`、`examples/`、`tests/`、`web/`、`xtask/` 及两份当前消费指南与 HEAD 无 tracked 差异。
- [PRD v0.2](../PRD-DOM-COMPONENTS.md)：SHA-256 `b857222e9da5da10fbd85be3e0f0de47fbec36be9e7a112bf5b6ef1869d6f841`；[差异报告](DOM-COMPONENTS-P1.prd-dev-gaps.md)：SHA-256 `c17453468475b45e7dfbc6bbe208b3ac5d44f414f7515b656e83194b1a13a264`。计划/PRD/部分评审和台账尚未提交，当前输入不能仅凭 HEAD 跨机器恢复。
- 开工已有的台账修改、未跟踪材料及两张旧布局图修改保持原样。本轮新增独立评审报告与两张已有页面截图，按已启用协议登记本轮评审；未实施或改写原计划。
- 收尾核对计划 hash 和 HEAD 未变，相关产品源码无并发变化；结论仍适用于本报告的所审版本。

### 3.2 实际审查分工

| 视角 | 实际负责人 | 已完成覆盖 | 证据限制 |
| --- | --- | --- | --- |
| facts | `/root/facts` | exporter、provider、存储反馈、构建/输出入口、内部矛盾 | 历史记录不等于当前重新通过 |
| architecture | `/root/architecture` | 局部 artifact 复用、模式所有权、版本身份、字体/CSS/base 接缝及成本 | 未实现新功能；最小生成器编译失败已确认 |
| schedule | `/root/schedule` | PRD AC→设计→任务→检查、M1/M2 依赖、验证节奏与恢复快照 | 未运行未来 V-1–V-3 |
| delivery | `/root/schedule` 完成 schedule 后接续 | 开发者从默认导出到独立消费、失败恢复、响应式/双语/键盘方案 | 文档推演与源码核对；浏览器仅主 agent 操作 |
| 综合复核与交付 | `/root` | 回读候选的原文/代码、最小编译反例、已有页面、去重裁定、报告写入与回读 | 没有用 reviewer 一致意见代替原始证据 |

初轮三个 reviewer 并行独立调查；受四个总槽位限制，第四视角在空闲 reviewer 上顺序接续。共用浏览器由主 agent 单人操作，无多人写同一报告或并行 Playwright。

### 3.3 已执行的检查

| 检查 | 结果与人工判读 |
| --- | --- |
| `check_paths.sh <计划> <仓根>`，按其可执行入口运行 | 退出 0；1 个 MISSING 是明确标 ★、按实际发生才创建的 M1 记录，符合未实施状态 |
| `check_progress.sh <计划> <仓根>`，按其可执行入口运行 | 退出 0，ERROR 0/WARN 1；0/2、两行里程碑、空完成记录一致。dirty 提醒来自既有输入，未发现本增量实施后漏回写 |
| `python3 .../check_plan.py <计划>` | 退出 0，ERROR 0/WARN 4；四个自定义 WORK/AC 引用已人工对照 PRD，未发现悬空映射 |
| F-01 的 scoped `mbx check --locked --offline` | 退出 101，确认生成器缺编译期修订变量；不能记为通过 |
| 当前 release 的只读浏览器走查 | 已完成下述有限路径；不是未来新增 UI 的验收 |

两项 shell 校验脚本最初被显式交给 `sh` 时发生 shell 语法不兼容，随后改按脚本自身入口执行；表中是纠正调用后的结果。结构脚本通过不证明方案或产品验收通过。没有为本次计划评审运行整仓测试、完整四 spec、GPU/Docs/性能或发布操作。

### 3.4 UI 证据等级

按仓库要求读取 `PRODUCT.md`、`DESIGN.md`，运行一次 `impeccable context`，读取既有 Theme Studio surface brief 和 Operate 指引。新增说明沿用中性壳、现有 Select/Dialog、常驻标签与次级下载动作，方案与既定界面一致；没有发起独立整站设计改造或评分。

主 agent 通过 Kimi Browser Extension 打开仓内已有 release 的严格 CSP 预览 `http://127.0.0.1:4181/`。产物 build ID 为 `105da89e37304db9`；Rust 页及原片段的实际修订 header 均为当前完整 HEAD。步骤及结果：

1. 打开 Export，默认 JSON 中没有公共接入资料，符合本计划要补齐的现状；切换 Rust 后可见源码修订和现有接入提示。
2. 当前 Rust 提示确实只有 mode/context 两参，而原片段中函数有 container/config/mode/context 四参；这支持计划修正 G-02 的必要性，不作为本轮新增问题重复计数。
3. 查看 [桌面 1299×786 实拍](../validation/dom-components-p1-gaps/plan-review-20261029/export-rust-desktop.png)及 [390×844 实拍](../validation/dom-components-p1-gaps/plan-review-20261029/export-rust-mobile.png)，并实际打开图像核对；退出对话框后恢复浏览器尺寸。截图只记录既有界面，不证明新增说明的布局或完整焦点行为合格。
4. 本次交互期间的 error、unhandledrejection、CSP 事件收集为空；期间捕获的三项外部字体请求均为 HTTP 200。未取得启动阶段完整 console 记录，未做全量请求审计。

临时预览已停止，网络捕获已释放。未验证新说明下载字节、中文/768/200% 矩阵、原生辅助技术、完整焦点回归或存储故障；这些未来改动仍须按 M2 验证，不能由此次已有页面实拍替代。

## 4. 映射与实施编排

| 需求/来源 | 计划中的交付链 | 核对结论 |
| --- | --- | --- |
| WORK-F20/AC-3；G-01 | §5.1 公共说明/纯文本 → M1 → V-1；M2/V-3 独立消费与实际下载 | 目标与任务已承接；F-01 阻断重建，F-02 缺当前模式输入 |
| WORK-N5/AC-1；G-02 | §5.2 当前 provider、完整调用、简例口径 → M1/V-2 → M2/V-3 | 映射完整，保留人工语义核对，不能以关键词零命中代替 |
| WORK-F21/AC-2—AC-3、WORK-N6/AC-1；V-01 | §5.3 三处反馈观察同步 → M1；最终 release 的四 spec → M2/V-3 | 未减弱数据/重试合同，未把调查 probe 冒充原测试通过 |
| C-1/C-2/C-3 | §5–§9 DOM、双语/响应式/CSP、JSON 不变、资源外置与证据隔离 | 有对应检查；F-03 是证据隔离命令的具体断点 |

映射依据已对照 [PRD WORK-F20/F21](../PRD-DOM-COMPONENTS.md#work-f20-无损交换主题并在独立应用中消费)、[N5/N6](../PRD-DOM-COMPONENTS.md#work-n5-能力声明与实际结果一致)及 §7.2 分期。N5/AC-2 仍归 P5，N6/AC-2 仍归 P4；本计划没有将局部修补扩大为跨期整条需求完成。

原编排可以保留，无需增加里程碑：

| 波次 | 前置及解锁证据 | 并行/共享负责人 | 运行约束 | 汇合验收 |
| --- | --- | --- | --- | --- |
| 修订计划 | F-01 的编译反例、F-02 的输入/输出矛盾、F-03 的实际输出变量 | 主修订者统一改动相应契约和命令；不改历史报告 | 不启动实施或外部环境变更 | 三项根因均在计划中有明确处置 |
| M1 连续实施批次 | 已定 exporter/resolve/download 契约；补齐 notes mode 与 fixture 构建身份路线 | 同一实现者维护 `data_tools.rs`、指南、直接测试及必要构建入口；任务小且共享文件，不宜硬拆并行 | 批内仅最小反馈；整批齐备才跑局部检查 | V-1/V-2、fmt、scoped Clippy；若修构建接缝，先补能解锁下游的最小编译 |
| M2 阶段收口 | M1 稳定、V-1/V-2 通过，独立构建前提与新证据目录已落实 | 主集成人单一收口；没有值得另拆的独立实现波次 | Studio 重建后按 project 串行；共享浏览器/端口/结果目录避免并发 | V-3 四 spec、八种 CSS + 默认 DOM、新说明实际下载、受限资料的开发者角色模拟及 UI/CSP |

依赖边为：稳定契约与必要修订 → M1 → V-1/V-2 → M2/V-3 → 最终提交与 SHA 回填。恢复快照 0/2、尚未开始、下一步 M1 与正文相符。保留单份需求映射、连续批次和一次阶段终验；没有必要重复整组检查、重做 GPU/性能，或为解除局部前置另建里程碑。

## 5. 总体结论与修订顺序

**是否需要修改：需要。**

**建议状态：Blocked（局部开工/收口前提），与计划自报 Ready 不一致。** 三项均为 P2，修订范围有限；但 F-02 使已定函数不能满足自己的当前模式输出合同，F-01 已实证阻断核心独立消费重建，故当前全文不能原样交给实施者并宣称前置完整。阻塞不需要新增产品决定，独立的指南修正仍可推进；本轮不启动实施。

修订顺序：先明确 F-01 的构建身份传递及最小允许改动，再补 F-02 的显式 preview 输入与对应断言，最后修正 F-03 的证据输出参数并统一 §4/§9/§10/§11。计划修订后，三个问题都应具备可直接执行的路线和相应验证；最终功能通过仍由 M1/M2 实际记录证明。

下一步：

```text
$apply-doc-review docs/plan/DOM-COMPONENTS-P1-gaps.review.md
```

本报告不修改原计划状态，不表示本增量已实施或原 P1 的 G-01/G-02 已关闭。

## 修订处置与复核

- 复核日期：2026-10-29（Asia/Shanghai）；本节为上述评审的文档修订处置，不是新一轮独立评审。§1–§5 的原问题、所审基线及 Blocked 建议保持原样。
- 来源：本报告原始 SHA-256 `8976a91830a395aa65af1760f13480aecf0afda5d669ba0d4d692c05d1b1b6bd`；评审 ID `9152677e-02be-4557-ac77-d2fb2244a90a`。
- 目标：[DOM-COMPONENTS-P1-gaps](DOM-COMPONENTS-P1-gaps.md)；修前 SHA-256 `872b6cb36176238bbf712fa89055e5dc0283b17c91822d3dae81071cfa571e9a`，与所审版本一致；修后 SHA-256 `e8272d348ed90af2bc7580e7deabd1c5b9d97304c0dcd01d6d0e75738d07c6c5`。
- 代码基线：`main` / `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e` / dirty。开工已存在台账、未跟踪文档及旧布局截图改动；本次不纳入或回退。相关产品源码及两份消费指南与 HEAD 无 tracked 差异；目标计划、报告尚未提交，跨机器恢复需要同步工作树。
- 范围：F-01–F-03、§2 的待验边界与 §4 的映射/依赖。没有前次处置；本次仅修目标计划并记录复核，不改产品代码、原 PRD、原 P1 计划或历史验收事实。

| 问题/报告章节 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。亲读 generator 的 include、Studio build.rs、data_tools 的 env! 与 build.sh 的 mbx 调用；未设变量的最小编译退出 101，命令显式注入当前 Git 修订后退出 0 | 已修订并复核 | §1.1 区分消费路径与重建缺口；§1.2/§4 固定命令级注入，不新增构建脚本；§9.2 给出生成器检查及完整构建入口；同步 M1/M2 前置、§11 风险与恢复快照下一步 | **已解决（文档层面）**：修订取得 → mise → mbx → rustc 的输入链可执行；M1 对新增代码再验最小编译，M2 才完整重建并核 Studio/资料/fixture 同源身份，当前未宣称整条重建通过 |
| F-02 | 成立。ResolveContext 没有 mode；DataTools 接收独立 preview_mode signal，provider_snapshot 区分 preview 与作者 appearance；真实导出 preview 函数接收四参 | 已修订并复核 | §5.1 增加 preview_mode: ThemeMode，屏内/下载从现有 signal 传当前三项输入；§5.2 明确四参示例不能硬编码模式/context；V-1、§9.2/V-3 与 M1 同步 | **已解决（文档层面）**：同一 document/context 的 Light/Dark 断言及仅切预览后重新下载已纳入检查；低频生成、双模式资源、JSON 字节与作者历史合同保留。函数与浏览器验收待 M1/M2 实施 |
| F-03 | 成立。亲读 runtime spec 的 RUSTIFY_THEME_EXPORT_OUT 及固定文件写入；Studio export UI spec 使用另一个 RUSTIFY_UI_EVIDENCE_OUT，前一条命令的赋值不能保护后续独立消费者 | 已修订并复核 | §9.2 独立消费者命令设置 RUSTIFY_THEME_EXPORT_OUT=docs/validation/dom-components-p1-gaps/export；M2/§12 同步区分两种输出入口，并要求核对历史证据未覆盖 | **已解决（文档层面）**：取证命令与实际写入变量、增量目录一致。本次未执行会写截图/JSON 的浏览器命令；实际新证据落盘检查仍由 M2 执行 |
| §2 待核实项与边界 | 新说明/默认 JSON 资料、三个存储故障用例及新增资料后的 UI 矩阵，均依赖尚未实施的产物；不是本次改稿可补成通过的证据 | 保留实施后验收 | V-1–V-3、§6/§9/M2 保留；补充 mode 单变量检查、重建身份与证据隔离要求 | 未提前关闭：M2 仍须交真实下载、受限资料独立消费、存储数据/草稿/库/重试及双语/响应式/键盘/失败出口证据 |
| §4 映射与实施编排 | WORK-F20/AC-3、WORK-N5/AC-1、WORK-F21/AC-2—AC-3、WORK-N6/AC-1 已与 PRD 原文核对；无新增产品或外部决定 | 已修订并复核 | 保留 M1 → 局部退出 → M2/V-3；仅补生成器编译前置与模式契约，不增加里程碑；恢复快照仍为尚未开始、0/2 | 映射未断链；完成记录与实施协议未改。N5/AC-2、N6/AC-2 及 U-01/人工辅助技术/真机仍留原去向，未冒称本增量完成 |

### 决策与未完项

- 三条意见均采纳，无未采纳项、无待用户决策项。F-01 采用报告允许的最小命令修订；未选新增 build.rs/修改 build.sh，原因是当前显式注入已经解开最小编译阻塞，无需扩大代码改动面。
- 本次没有实施新的 integration_notes、公共说明、下载动作或存储断言。原 G-01/G-02/V-01 的产品/验证收口仍待 M1/M2，不因计划层 F-01–F-03 解决而关闭。
- [消费者 fixture 指南](../../tests/theme-export/README.md)及[仓根 README](../../README.md)中仍有裸 build.sh 示例，受同一编译期输入前提影响；它们不是本次授权的修订目标，记录为另需同步的构建入口说明，不把本计划的显式入口据此判成仍阻塞。主题指南/迁移说明的既有 G-02 错误继续由 M1 修正，历史报告保持原位。

### 修订后结论与验证

**本次 3 项 finding 均已修订并复核，文档层复核结果为已解决；未采纳 0、待决 0、文档待核实 0。修订后计划为 Ready，实施仍为 0/2、尚未开始。** 关键输入、调用契约、构建与证据入口均已明确；这不等于重新独立评审通过，更不等于 G-01/G-02 或实现已验收。

| 本次实际检查 | 结果与范围 |
| --- | --- |
| `mise exec -- env -u RUSTIFY_STUDIO_SOURCE_REVISION mbx check --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bin theme-export-generator` | 退出 101，亲自复现 data_tools.rs:293 的编译期变量缺失；未运行生成器或写入历史证据 |
| `RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mise exec -- mbx check --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bin theme-export-generator` | 退出 0，证明当前源码的命令级传递可编译；不是完整 --dom-only 构建或未来新增函数的通过记录 |
| `.agents/skills/dev-plan/scripts/check_paths.sh docs/plan/DOM-COMPONENTS-P1-gaps.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 退出 0；9 个路径 token，MISSING 1 为明确标 ★、按需创建的 M1 记录，符合未实施状态 |
| `.agents/skills/dev-plan/scripts/check_progress.sh docs/plan/DOM-COMPONENTS-P1-gaps.md /Users/rockie/Documents/gh-rockie/rustify-ui` | 退出 0，ERROR 0/WARN 1；0/2、两行里程碑、空完成记录一致。dirty 告警来自既有未提交/未跟踪输入，脚本不能据此识别本次未跟踪计划已改稿，人工已核对未发生实施漏回写 |
| `python3 .agents/skills/dev-plan/scripts/check_plan.py docs/plan/DOM-COMPONENTS-P1-gaps.md` | 退出 0，ERROR 0/WARN 4；四个自定义 WORK/AC ID 均与 PRD 对照，未发现悬空映射 |
| 对计划三个 sh 代码块执行 `sh -n`；对照修前副本 | Shell 语法全部通过；只解析、不执行收口命令。实施者/进度协议及空完成记录保持不变；回读模式输入、注入命令、输出变量与里程碑依赖一致 |

仅做文档修订所需的最小编译和静态检查；未运行计划中的完整构建、业务回归或浏览器验收，未在浏览器中验证新说明/下载/UI。§2 所列实施后检查全部保留到相应里程碑。

下一步由用户发起：

```text
/goal 按 docs/plan/DOM-COMPONENTS-P1-gaps.md 开发
```
