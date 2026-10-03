# THEME-STUDIO 开发计划评审

## 1. 确认问题

### [P2] F-01 · 明确同一 CSS 产物中新旧圆角语义的隔离机制

- **定位**：[计划 §3.2](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:198)、[§5.1](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:229)、V2（367 行）、M2（426 行）；关联 C2。
- **问题**：计划要求在 component-catalog 增加新主题 fixture，同时保留原默认路径；又要求在 sdk.css 后导入 theme-v4.css，将同名圆角 utility 改为新公式，并保持一页一份 Tailwind 产物。若新 fixture 沿用 catalog 当前样式入口，`rounded-md` 将从 `r` 改为 `max(0px, r−2px)`，旧 ThemedScope 下的 Button 也会变化。旧主题默认 radius 为 6px，因此旧按钮会按新公式变为 4px。仅切换 provider 无法区分这两种全局编译公式，现有计划没有给出 scope 级版本选择或独立产物边界。
- **证据**：`examples/component-catalog/tailwind.css:1–14` 明确 SDK 与应用合编一份样式；`index.html:8` 加载该产物。`crates/rustify-components/css/sdk.css:54–57` 定义旧 sm/md/lg 为 r−2/r/r+2；`crates/rustify-components/src/button.rs:13` 使用 rounded-md；`crates/rustify-ui/src/theme.rs:82` 默认 radius 为 6，`:295–314` 的旧 provider 写同一组 CSS 变量。`docs/architecture.md:68` 禁止并挂两份同名 utility 产物。[Tailwind 官方说明](https://tailwindcss.com/docs/theme#referencing-other-variables)解释了 inline 值进入 utility 的机制；在线页面为 v4.3，不能代替固定 4.1.13 的编译验证。
- **最小修订**：在 §5.1 明确新圆角派生值使用独立 runtime 变量，utility 带旧公式 fallback，让同一产物内旧 scope 保留旧值、新 scope 提供新值；并规定局部 ThemeBoundary/浮层如何更新这些派生值。另一可行选择是让新 fixture 使用独立 HTML/CSS 产物，但需明确 opt-in 是整个样式产物的迁移边界，并同步调整兼容说明。
- **复核**：改稿后检查 §3.2、§5.1、V2、M2 的边界一致。实施时用固定 4.1.13 生成一份 CSS，在旧/新 scope 分别检查 rounded-sm/md/lg：旧值保持，新增公式正确；更改新 scope 与局部 patch 不影响旧 scope。独立产物方案则分别保留原 catalog 与新 fixture 的构建、默认值证据。
- **置信度：高**，由当前公式、默认值和单产物入口直接推导；尚未执行新 CSS 的编译实测。**阻塞：M2 对应兼容方案与 V2；不阻塞 M1。** 本问题依据本期 catalog fixture 的明确承诺，不额外要求所有未知宿主都支持新旧 provider 混用。

### [P2] F-02 · 跨标签冲突用例必须共享同一 BrowserContext

- **定位**：[V5（370 行）](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:370) 的“两标签用独立 fresh context”；关联 §5.5（288 行）、失败模式（351 行）、R5/NFR1/M5。
- **问题**：若按该句为两个标签分别建立独立 context，两者 localStorage 相互隔离，无法触发计划要验证的真实跨标签 storage 通知。验收会等不到事件，或改用人工派发事件而漏验真实写入链路。
- **证据**：`tests/browser/support.ts:685–687` 明确 fresh 是每个测试使用新 context，并把“同一 context 中第二个 page”列为申请 fresh 的场景。仓库 `package.json:6` 固定 Playwright 1.63.0；[Playwright 官方 Isolation 文档](https://playwright.dev/docs/browser-contexts)明确 BrowserContext 隔离 localStorage 等状态。[storage 事件说明](https://developer.mozilla.org/en-US/docs/Web/API/Window/storage_event)要求另一文档共享相应存储区域。
- **最小修订**：改为“每个持久化用例使用独立 fresh context；跨标签用例在该 context 中创建两个同源 page”。测试之间隔离，用例内部共享。
- **复核**：A 页真实保存，B 页收到冲突提示；B 的内存草稿在用户决定前不变；分别覆盖载入与保留/另存分支。不能仅人工 dispatch storage 事件代替实际跨页写入。
- **置信度：高**，工具语义和仓库 fixture 一致。**阻塞：V5 冲突测试设计与 M5 验收；不阻塞 M1–M4。** 原句可能意在表达“不同测试隔离”，但应消除实施者必须猜测的歧义。

未发现 P0/P1；没有把尚未实现的主题站、GPU 扩展或新文件列为缺陷。

## 2. 待核实与实施前提

| 项目 | 缺少的证据／影响 | 提供角色与最晚确认点 |
| --- | --- | --- |
| 旧阴影默认值 | §3.2 写新增阴影默认无阴影，§5.1 写 utility 映射新变量，但没有明确阴影映射是否仅 opt-in。现有 `crates/rustify-components/src/dialog.rs:18` 已使用 shadow-lg。若全局映射至 none 会丢失旧阴影；若只放 opt-in 或保留旧 fallback，则没有缺陷。目前不作为确认问题。 | DOM/CSS 负责人在 M2 前明确映射归属，并把旧浮层 computed box-shadow 纳入 V2 基线。 |
| 本地工具链 | 未重跑 doctor、Tailwind 编译或 release wasm build；不能确认本机已能执行计划命令，也不重复宣称计划记录的 PATH 问题仍存在。 | 实施负责人在 M1 开始按已有入口体检；缺依赖按仓库规则处理。 |
| GPU 与字体呈现 | 同源字体的实际选择/打包、字距的组合簇/连字、RGBA 混合、阴影裁剪/命中、恢复最新 revision 尚未实测。计划已给出明确实现任务和 V3，不属于额外产品决策。 | GPU/字体负责人在 M3/V3 提交真实绘制证据；未通过不得记完成。 |
| 性能与静态交付 | 没有本次更新 p50/p95、包体增量、根/子路径或独立导出接入的运行结果；计划未伪造数值。 | M1 取得改前基线；主集成人在 M6/V7/V8 收口。 |

下载反馈不另报缺陷：`crates/rustify-ui/src/files.rs:270–298` 只发起 anchor 下载，不能据返回成功宣称文件已保存；计划 352 行允许保留导出面板，足以采用“已发起下载”的真实语义。坏缓存恢复也不另报问题：计划 350 行已要求旧保存保留、新修改只在内存。

## 3. 覆盖、基线与实际检查

### 基线

- 调查日期：2026-10-03，Australia/Sydney。
- 计划：`docs/plan/THEME-STUDIO.md`，462 行；自报 Proposed、0/6、尚未开始。
- 计划 SHA256：`d8e529749da1ffb0224e7fce6c77f04366f862cccf2dc8fbc98addedeaa13ddf`。
- 代码 HEAD：`2a41704f3c5c87e27132753c056d7f75fdde7488`。事实源为当前工作树；开始时仅 PRODUCT.md 与该计划未跟踪，没有已跟踪代码修改。
- 产品上下文：`PRODUCT.md`，SHA256 `45f315659ba89b3e12a82096870fe58985b3e5b88acf3342278fd25385138547`。没有 DESIGN.md；未发现 examples/theme-studio 实现。
- 写报告前再次核对，HEAD、计划及 PRODUCT hash、原有 dirty 状态均未变化。报告为本次唯一写入；原计划及代码保持只读。

### 四维审查台账

| 维度 | 实际负责人／方式 | 覆盖结果与证据缺口 |
| --- | --- | --- |
| facts | 独立 subagent `/root/facts`，完成 | 核对 Theme/ThemePatch、scope CSSOM、局部覆盖、Tailwind、Portal、GPU 旧接口、字距与参考预设。提出 F-02；阴影边界留待核实。 |
| architecture | 独立 subagent `/root/architecture`，完成 | 核对统一解析、两端投影、CSS 兼容、字体/字距、阴影与导入存储。提出 F-01；真实运行能力留给既定检查门。 |
| schedule | 独立 subagent `/root/schedule`，完成 | 核对全部 R/NFR/C/A、参考矩阵、恢复协议、依赖和集中终验。独立发现 F-02；未发现额外映射断链或依赖环。 |
| delivery | facts 完成后复用该 agent，单独执行 delivery 审查，完成 | 按完整用户路径核对入口、编辑、反馈、保存恢复、导出接入、错误与可访问性；没有新增确认问题。UI 为文档推演。 |
| 综合与落盘 | 主 agent | 回读候选问题的计划和原始代码证据，去重裁定，核对官方资料及脚本结果；单独写报告并回读。 |

采用三个 reviewer 并行、空闲后复用一个 reviewer 完成第四视角；没有声称四位 reviewer 同时独立运行。UI 使用 impeccable 上下文与交互审查指导，context 已实际执行，确认产品上下文存在而新站无原型。本次没有进行新设计或修改产品文档。

### 已核实的关键事实与路线

| 主张／选择 | 核实依据与判断 |
| --- | --- |
| 旧模型需增量扩展 | `theme.rs:15–50,119–144,184–191` 确认 19 色、4 个度量/动效属性、23 个 CSS 变量及 7 字段 patch；保留旧 API、增加完整文档的理由成立。 |
| 作用域与浮层 | `theme.rs:295–315,327–357` 确认 CSSOM 和稀疏覆盖；`overlay.rs:787–801` 确认 Portal 挂载位置。计划显式传播局部有效主题，方向成立。 |
| Tailwind 编译与 spacing 分离 | `sdk.css:60–68` 确认旧 utility 单位 0.25rem，与 scope 的 8px gap 不同；`xtask/src/build.rs:143–150,252–263` 确认示例自己的输入在构建期编译。新 spacing 分名适配现状；圆角还需 F-01。 |
| 字距扩展不是现成能力 | `makepad/draw/src/text/font_family.rs:46–53` 确认 shaping 入口传零字距。计划要求贯通 layout/shape/cache/测量并验证组合簇，未将底层已有参数误称完整支持。 |
| 参考数据语义 | `ref/tweakcn-main/utils/theme-preset-helper.ts:24–39` 的深色合并顺序，以及 `config/theme.ts:5–17` 的 COMMON_STYLES 与计划一致。ref 只作调查，M1 固化必要数据符合干净 checkout 约束。 |
| 架构适配与代价 | 同一 ResolvedTheme 驱动 DOM/GPU，避免分别解析；统一 sRGB 加诊断符合本期比较边界；固定离线字体目录控制范围；单 envelope 与有限本地库匹配，无需新增服务或 adapter 框架。主要成本是旧入口兼容及 GPU 文字/阴影实现，计划已设置相应局部门。 |

### 工具与证据等级

- 实际运行 `check_paths.sh docs/plan/THEME-STUDIO.md <仓库根>`：检查 39 个路径，6 个 MISSING，退出 0。逐条核对为拟新增 theme-v4.css、theme/、docs/themes.md、examples/theme-studio（有无尾斜杠各计一次）、vendor/tweakcn/；不构成虚构复用路径。
- 实际运行 `check_progress.sh docs/plan/THEME-STUDIO.md <仓库根>`：0/6，6 行里程碑，0 行完成记录，ERROR 0、WARN 1，退出 0。告警因两份未跟踪文档且无已完成记录；当前不是遗漏实施回写。
- 已读取 dev-plan 的骨架与恢复/验收合同。顶部计数、记录路径按 CLAUDE.md 归入 docs/validation 的适配、逐里程碑回写约定成立。
- 外部查询日期为 2026-10-03：核对上文链接的 Tailwind/Playwright/storage 资料，并查看计划引用的 [CSS Color 4 转换示例](https://www.w3.org/TR/css-color-4/#color-conversion-code)。未以在线 Tailwind v4.3 页面证明固定 4.1.13 产物通过。
- **未在浏览器中验证**：新站尚无实现/可运行原型；没有启动旧例来冒充新站走查，也未运行构建、单测或性能测试。本次可判断交互方案与未来验收闭环，不能证明实际视觉、交互、CSP 或 GPU 表现。

## 4. 映射与实施编排

### 需求与用户路径

R1–R5、NFR1–NFR3、C1–C4、A1 均在 §0.1 有设计及检查落点，§4 的本地参考能力覆盖 V1–V6，延期能力有明确去向。没有独立需求漏配；确认断点是 **C2 → 圆角兼容设计 → M2/V2（F-01）** 和 **R5/NFR1 → 跨标签恢复 → V5 环境（F-02）**。

用户路径在文档层面闭合：静态站入口 → 选预设/模式 → 编辑并比较 DOM/GPU → 查看色域/字体/对比度诊断 → 撤销或恢复 → 本地保存/刷新 → JSON/CSS/Rust 片段导出 → 独立应用接入。§6 包含分栏/窄屏、键盘、IME、错误/空态、低对比度和稳定编辑壳；§9.2 明确独立 fixture，避免只在编辑站内部自证导出成功。在线服务与完整 GPU 业务控件复制已明确排除，不能视为遗漏。

### 依赖边与并行波次

现有六个里程碑的拓扑可保留，不需新增里程碑或逐项过程文档。

| 波次 | 前置及解锁证据 | 可并行工作 | 共享边界与汇合门 |
| --- | --- | --- | --- |
| M1 | 工具链体检；确定文档/解析契约，V1 与改前基线 | 共享契约未稳定前不拆下游公共类型 | 主负责人单写公共类型、Cargo/lock、来源注册；V1 解锁后续。 |
| M2、M3 纯控件、M5 codec | V1；M2 先修 F-01 | DOM/CSS/overlay、GPU/Makepad 文本、纯编解码可按稳定接口分工 | 公共导出/CSS 入口/字体目录单人集成；M3 真实集成依赖 M2 最小站。 |
| M3 集成、M4、M5 persistence | M2/V2 解锁 M4 开发；M4 编辑命令稳定解锁保存整合 | editor/previews、codec/persistence 在非冲突文件内分工 | M4 退出必须 V3；M5 UI 收口依赖 V4，V5 先修 F-02。 |
| M6 | V1–V5 和完整场景 | 文档与 CI 准备可按共享文件单写安排 | 主集成人统一最新构建、V6/V7/V8、证据和进度回写；最终用户主路径集中验收。 |

共享 Cargo/lock、公共导出、主题注册表、CSS 入口、字体目录、Playwright/CI、计划由 §10 指定的主负责人单写。浏览器/GPU 和不同 Playwright 命令串行，不因源文件可并行就并跑验收。M1 基线、M2 scope/CSP/兼容、M3 实际 GPU 绘制是局部风险门；不能挪到 M6 才发现。未给出工期数据，本报告只评依赖顺序，不估算节省时间。

## 5. 总体结论与修订顺序

**建议状态：Blocked（局部实施前置未闭合）。** 与计划自报 Proposed 不同，原因是 C2 的兼容承诺尚缺 F-01 的可执行隔离规则；并非否定总体技术路线，也不要求停止独立的 M1 工作。F-02 是 M5 验收前必须消除的局部测试契约错误。

建议依次：

1. 在 §5.1 定下圆角隔离或独立产物边界，同步 §3.2、V2、M2；一起澄清旧阴影 fallback，避免同类兼容误解。
2. 修正 V5 的 context/page 层次，不改现有六里程碑结构。
3. 针对修订后的相关链路复审。F-01 的兼容断言可实现、F-02 能验证真实跨页写入后，可解除本次设计阻塞；实际 V1–V8 仍由实施阶段完成。

工具链、GPU 视觉与性能的未实测状态已在原计划设置执行门，不单独作为计划必须先实现一遍才能通过的条件。任何就绪结论都不等于功能已实现或上线可用。本报告不修改原计划状态、不代做修订或实施。
