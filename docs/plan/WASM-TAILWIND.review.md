# WASM-TAILWIND 开发计划评审

> 第 1–5 节保留原版计划的评审与基线；第 6 节记录旧包移除修订，最新的动态入口与风险门处置见第 7 节。原 F-01 中保留旧包的建议已由用户明确的完整移除决策取代。

## 1. 确认问题

### [P2] F-01 · 动态组件应从原始类片段一次性执行上下文 cn

- **定位**：[计划 §5.1，第 250 行](WASM-TAILWIND.md#51-cn合并语义与迁移)，同时涉及第 149 行改动面、R-1、M6。
- **问题**：计划要求保留 `variants!` 的旧 derive，并将 dynamic context 的“最终输出再由 cn 合并”。如果保留当前 `.with_class()` 合并链，旧 `tw_merge` 会先消除类名。例如完整 cn 配置把 `bg-primary` 与 `bg-secondary` 放进两个不冲突的组，正确结果应保留两者；旧合并先删除 `bg-primary`，后续 cn 无法恢复它。独立 cn 的配置差分通过，不能证明组件接入遵守同一配置。
- **证据**：[variants.rs](../../crates/rustify-components/src/macros/variants.rs#L74) 的生成组件调用 `.with_class()`，第 206 行使用 `TwClass` derive；真实 [Button](../../crates/rustify-components/src/button.rs#L76) 也走此链。[Cargo.lock](../../Cargo.lock#L2093) 锁定 `tw_merge 0.1.21` 与 `tw_merge_variants 0.1.9`。已读取本机锁定依赖源码：`tw_merge_variants-0.1.9/src/class_macro.rs:24–30` 默认选 `TailwindMerge`，`:102–108` 收集 base/variant/caller 后调用它；`tw_merge-0.1.21/src/lib.rs:279–281` 再调用 `tw_merge_slice`。[仓库已有断言](../../crates/rustify-components/src/macros/mod.rs#L29) 明确旧规则会消除较早的 `bg-primary`。固定上游 [cn config.ts:58–77](https://github.com/shadcn-ui/cn/blob/b7ec0fce43d824a60517fab508584eb860fdf717/packages/cn/src/config.ts#L58-L77) 接受完整配置及配置转换函数，不能假定配置始终等于旧默认分组。
- **最小修订**：保留公开 derive、类型和未启用路径；动态组件使用未经冲突合并的 base、variant、size、caller 片段，一次性交给上下文 cn。明确动态路径绕过旧 `.with_class()` 的有损合并，把必要的宏及组件接入改动列入 M6。无需删除 `tw_merge` 或重构所有调用点。
- **复核**：改稿后确认链路为 raw fragments → cn；实现后用同一 raw 输入比较独立 cn 与动态组件输出，覆盖完整配置、override、自定义 validator，并回归未开启 feature 的旧输出。配置切换还应验证已挂载的 opt-in 组件怎样消费新配置；仅替换返回 `String` 的 `merge()` 不会自动使初始化时计算的 class 重算，不能以配置对象已切换代替组件行为证据。
- **置信度**：高，旧 derive 的有损调用链与上游配置入口均已读取；**阻塞范围**：M6 动态组件接入，不阻塞 M1、M2 核心实现。

### [P2] F-02 · GPU hover 契约应包含上游媒体条件

- **定位**：计划 §5.5 第 297 行 context、第 307 行 GPU 支持矩阵、第 314 行重求值，以及 V-7 第 375 行；关联 R-5、M5。
- **问题**：计划将 `hover` 定义为 Button/样本的显式控件状态条件，列出的 context 与更新来源没有媒体 hover 能力。Tailwind 的 `hover:` 同时要求控件 hover 和 `(hover: hover)` 成立。媒体条件为 false 时，若 GPU 只看 hovered，会应用 DOM 不生效的样式；若保留条件却当不支持，则矩阵承诺的 hover 无法完整交付。
- **证据**：固定 v4.1.13 [variants.ts:654–655](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/tailwindcss/src/variants.ts#L654-L655) 同时生成 `&:hover` 与该媒体条件。现有 [RustifyButton 状态](../../crates/rustify-ui/src/gpu/button.rs#L129) 来自 `FingerHoverIn/Out`，本身不提供媒体查询结果。全文检索未见 `matchMedia` 或等价媒体环境的获取、变化及验收契约。
- **最小修订**：在 GPU 条件 IR/context 中明确 `can_hover` 或等价有限媒体环境。浏览器通过 `matchMedia('(hover: hover)')` 提供并在变化时重求值；纯 Rust/native 调用者显式提供。只补已承诺 hover 所需的条件，不扩大成全量 media-query 引擎。
- **复核**：改稿后确认状态与媒体条件共同参与判断；实现后 V-7 覆盖 can_hover true/false × hovered true/false、条件变化和旧值恢复，并与官方 CSS 样本对照。Chrome 条件仿真用于这条语义验证，不作为移动设备整体兼容声明。
- **置信度**：中，上游条件与计划缺少明确输入可确认，但 context 尚为草案，实施者可能主动补上；**阻塞范围**：M5 hover 条件实现，不阻塞其他 GPU family 或 M1。

## 2. 待核实与待澄清

| 项目 | 缺少的证据或决定 | 影响与最晚确认点 | 提供者 |
| --- | --- | --- | --- |
| U-01 · A-1 | 最小 extractor 依赖闭包在单线程 WASM 的实际构建、运行及 fixture 结果 | M1/V-1；扫描 WASM 集成之前 | 计划指定的实施负责人 |
| U-02 · A-2 | 当前 Chrome、现有 strict CSP 下 staging/adopted sheet、scope/property/nesting 与清理的实际结果；必要时验证计划已有 CSSOM 备选 | M1/V-1；DOM runtime 集成之前 | 计划指定的实施负责人 |
| U-03 · 自定义/公开注册归属 | §5.4 第 288 行的“自定义全局资源由静态 asset bundle 管理”是否明确覆盖作者 `@property`；动态输入遇到此类注册时的分类与处理 | M1 固化资源契约，最晚 M3 全量 sheet 安装前明确 | 实施负责人，具体承接人待指定 |
| U-04 · 完整兼容账本 | 固定 cn API/配置/build、Oxide preprocessing/Scanner、Tailwind registry/tests 的逐项锁定清单 | M1 后才能据清单判断 M2/M4 覆盖；不得用正文示例代替完整范围 | 计划指定的实施负责人 |

U-01/U-02 是原计划已登记的风险门，本次没有实施它们，也不将“尚未实测”重复计为新增问题。U-04 同样是明确的 M1 交付物，不因它尚未产出报缺陷。

U-03 需要澄清的原因是：同名注册在 document 级生效，后面的有效注册可能改变其他 scope 的变量计算；这与普通 scoped declaration 不同。[CSS Properties and Values API 编辑草案 §2.1](https://drafts.css-houdini.org/css-properties-values-api/#determining-registration) 描述了该规则，查询日期为 2026-10-04。原计划已承认全局事实，并把自定义全局资源交给应用静态 bundle，还在 §7 说明仅尽量隔离 arbitrary CSS。因此，本评审**不认定它必然要求每 scope 动态注册作者 `@property`，也不将该候选列为已确认隔离缺陷**。建议在分类账明确共享资源边界、动态输入处理及两 scope 的验证；若允许动态共享注册，再补冲突与 dispose/回滚契约。

范围来源限制：计划第 56 行记载了此前用户确认，本次可见会话没有那些原始对话。评审采用计划所列范围作为待审合同，未独立认证历史授权；没有据此重新决定产品范围。

## 3. 覆盖与基线

- **调查日期**：2026-10-04，项目环境时区 Australia/Sydney。
- **计划版本**：`docs/plan/WASM-TAILWIND.md`；SHA-256 `1b4139a1b8f64e6b46e6092d23b5a173d06801d193ca80b8dc0353ad329c41c9`。
- **代码基线**：`fb288564caed23f79e34bd8ac28dae2f6f44bfeb`。本次开始时 dirty 仅为未跟踪的目标计划；代码事实来自当前工作树。计划第 7 行的“调查开始时 clean”属于作者此前调查描述，不当作本次状态。评审期间未改原计划或代码。
- **审查台账**：facts、architecture、schedule 三位 reviewer 初轮独立并行；schedule 完成后复用该 reviewer 审 delivery。主 agent 阅读原始证据、交叉追问并单写报告。四维与报告写入、回读核验均已完成。

| 维度 | 实际负责人 | 核查范围与结果 | 覆盖限制 |
| --- | --- | --- | --- |
| facts | `/root/facts` | Cargo/工具锁、CLI/CSP、宏、主题、GpuRegion、Makepad glue、固定上游入口；确认 F-01 | 没有运行上游完整 conformance 或新的 WASM probe |
| architecture | `/root/architecture` | Rust/Worker 边界、候选排序、prepare/install/ack、scope 与全局资源、有限 GPU IR；主 agent 裁定 F-02、U-03 | Worker/CSSOM 的运行可行性仍是 A-1/A-2 的未来证据 |
| schedule | `/root/schedule` 第一轮 | R/C/NFR/A → 设计 → V → M、风险门、恢复协议与资源约束；没有独立编排问题 | 无工期和实做吞吐，不能推算完成日期 |
| delivery | `/root/schedule` 第二轮 | SDK 接入、初始化、动态应用、结果确认、错误恢复、demo 交互与终验；没有独立新增问题 | UI 为文档推演，没有新功能的原型或浏览器行为证据 |
| 综合与落盘 | `/root` | 去重、反证候选、规则/代码回读、脚本、基线复核和报告 | 计划通过不等于功能实现或已上线 |

### 事实与技术选择核对

| 关键主张/设计 | 核实依据 | 裁定 |
| --- | --- | --- |
| 当前是 Leptos DOM + Makepad 浏览器 SDK，静态托管与六个示例 | [README](../../README.md)、[Cargo.toml](../../Cargo.toml)、[AGENTS.md](../../AGENTS.md) | 现状成立；本仓不是 Portal App，不追加 onebox/授权/发布门 |
| CLI 固定 Tailwind 4.1.13 | [mise.toml](../../mise.toml)、[tailwind.rs](../../xtask/src/tailwind.rs) | 固定版本 oracle 有依据，不能用 latest 预期替代 |
| utility spacing 与旧布局 gap 分离 | [sdk.css:75](../../crates/rustify-components/css/sdk.css#L75)、[theme-v4.css:27](../../crates/rustify-components/css/theme-v4.css#L27) | 延续 inline/reference 与 `--rustify-unit` 的设计合理 |
| strict CSP 不允许 JS eval 或 inline stylesheet | [serve.rs:33](../../xtask/src/serve.rs#L33) | 规则事实成立；动态 CSSOM 能否使用仍须 U-02 实测 |
| `ScopeRoots` 可直接提供 container、abort signal、base | [mount.rs:33](../../crates/rustify-ui/src/mount.rs#L33) 仅有 container；[MountConfig](../../crates/rustify-ui/src/mount.rs#L16) 有 base；[release_on_abort](../../crates/rustify-ui/src/listeners.rs#L74) 已公开封装实例释放 | 第 283 行复用描述不准确。可明确 container 取 ScopeRoots、清理复用 release_on_abort、资源 base 由新增 UtilityScope 参数或 context 提供。存在直接可用接缝，不单独计阻塞问题 |
| container 能覆盖内容与浮层 | [mount.rs:124](../../crates/rustify-ui/src/mount.rs#L124) 在同一 container 下创建 content/overlay | 未确认“少观测一棵根”的候选；boundary/overlay 终验仍按 V-8 执行 |
| Rust core + 独立 module Worker | §0.3、§2、§4；现有 Makepad glue 与纯 Worker 产物边界 | 保留，符合全 Rust 生产契约；启动/复制/整表序列化成本在 V-11 实测 |
| 当前集合重排序、整表替换 | §5.3/5.4 明确官方 order 与两阶段安装 | 能避免到达顺序决定 cascade；无须改成逐规则 append |
| 主线程成功快照作为 Worker 恢复权威 | §3、§5.4、§8 已区分 prepare、安装、ack 丢失与重放 | 契约闭环；未发现需要引入额外通用事务框架的依据 |
| namespace 要同步改引用 | §5.4 第 288 行 | theme-v4.css 的 `var(--tw-shadow-color)` 不单列缺陷，已有完整 AST 引用改写要求 |
| GPU finite IR、strict/partial、静态 opt-in | §0.4、§5.5、§8；PRODUCT/DESIGN 与现有自有控件边界 | 范围明确；不要求 GPU 全量 CSS 或原生窗口验收，保留 F-02 的已承诺条件修订 |

### 实际检查

| 检查 | 实际结果 | 人工裁定 |
| --- | --- | --- |
| `bash .agents/skills/dev-plan/scripts/check_paths.sh docs/plan/WASM-TAILWIND.md /Users/rockie/Documents/gh-rockie/rustify-ui` | exit 0；检查 25 个路径 token，9 个 MISSING | 均是 §1.2 已用 ★ 标明的拟新增路径；无据此确认的复用路径错误。脚本不覆盖所有路径与行为 |
| `bash .agents/skills/dev-plan/scripts/check_progress.sh docs/plan/WASM-TAILWIND.md /Users/rockie/Documents/gh-rockie/rustify-ui` | exit 0；0/7，记录 0，ERROR 0、WARN 1 | warn 来自仅未跟踪计划的 dirty 提醒；没有实现进展或漏回写证据。七个里程碑、空记录与快照一致 |
| Impeccable `context --target docs/plan/WASM-TAILWIND.md` | exit 0；读取 PRODUCT/DESIGN；`hasVisualImplementation=false` | 采用已有中性工具、语义色、组件一致性和状态原则；未改设计文件、未运行独立 UI critique 工作流 |
| 构建、单测、conformance、性能测试 | 未运行 | 本次只读计划评审；拟新增核心/Worker/demo 尚未存在，未为评审实施 spike，也未执行无关长时间全仓套件 |
| 真实浏览器 | **未在浏览器中验证新功能** | utility-lab 是拟新增页面，本次是文档推演。没有把 typecheck、脚本或旧页面表现算成新功能验收；M1/M5/M6/M7 的浏览器证据仍须实施时提供 |

### 交付旅程与 UI 证据

| 开发者/使用者步骤 | 计划落点 | 核对结论 |
| --- | --- | --- |
| 开 feature，接入 API，构建 root/subpath 静态资源 | §1.2、§2、§4、§9.3；M6/M7、V-10 | Worker/glue/WASM 与 manifest 同批，未启用时不增加编译器产物；最终 docs/utility.md 有任务落点 |
| 初始化 scope 与受控资源 | §3、§5.4、§8；V-1/V-9 | 协议/hash、limits、deadline、初始化错误和释放明确；仍受 U-01/U-02 前置约束 |
| 输入、合并、ensure，再应用类/props | §5.1、§5.4、§6；V-2/V-6 | 同步 cn 与异步 CSS 明确分开；Observer 只是补充，一帧延迟没有被包装成无闪烁保证；F-01 修订组件路径 |
| 确认 DOM 与 GPU 的实际结果 | §5.4–5.5、§6；V-7 | CSS revision 与 drawn revision 可观察；strict/partial/unsupported 分开，不以 CSS 成功冒充 GPU 成功 |
| 配置/编译/配额/Worker/GPU 失败后恢复 | §8；V-5/V-6/V-9 | 保留上次成功结果、修输入、显式 retry、context 恢复与 dispose 均有路径 |
| 窄屏、键盘、暗色、双语与长输入 | §6、§9 | 已有设计与未来真实浏览器验收安排；不能据文档证明布局、焦点或读屏已通过 |

## 4. 映射与实施编排

账本第 58–73 行、V-1–V-11 第 367–379 行及 M1–M7 第 423–431 行能双向追踪。本次未确认独立的需求映射断链；F-01/F-02 是已映射功能的设计缺口，分别需回写 M6/V-2 和 M5/V-7 的具体断言。

| 范围 | 实现链 | 验收链 |
| --- | --- | --- |
| R-1 | cn/config/facade → M2 → M6 组件/crossing | V-2 → M7 |
| R-2 | extractor/preprocessing、真实/虚拟源 → M2 → M3 集成 | V-3 → M7 |
| R-3 | parser/theme/AST 切片 M3 → 全量 registry/directive M4 | V-4/V-5/V-10 → M7 |
| R-4 | ensure/sheet M3 → Observer/epoch/恢复 M6 | V-6/V-9 → M7 |
| R-5 | 有限 IR 与真实控件 M5 → 双预览 M6 | V-7/V-9 → M7 |
| R-6 | 主题/boundary/旧组件与默认路径 M6 | V-8，相关 V-7 → M7 |
| R-7 | M1 完整兼容账本 → M2/M4 全范围 → M7 | V-2/V-3/V-5/V-10，不允许以切片抵扣 |
| C-1–C-3、A-1/A-2 | M1 构建/CSP 风险门，M6 运行，M7 来源与交付 | V-1/V-6/V-9/V-10 |
| NFR-1/NFR-2 | M6 有界状态与故障；M1 口径/基线 → M7 对照 | V-6/V-9、V-11 |

**依赖边成立**：M1 固定 cn/扫描 API 与 fixture → M2；M1 extractor V-1 → scanner WASM。M2 scanner V-3、可用 cn 输出与 M1 Worker/CSP → M3。M3 AST/排序/session V-4 → M4。M3 IR 与实际所需、已测试的单个 M4 family → 对应 M5 能力，无须等待无关 family。M4 完整 CSS、M5 GPU 协议和已验证 cn/scan → M6。M2–M6 所需成果/检查 → M7。同一 release 终验由主实施者收口，V-1 仅基线未变时复用。

| 波次 | 前置与可并行工作 | 写入/运行资源负责人 | 汇合门 |
| --- | --- | --- | --- |
| M1 | 范围固定；参考清单调查可与最小 WASM/CSP probe 准备分工 | 主实施者单写 Cargo/锁/xtask/sources；浏览器一个操作者 | V-1、可重放 V-11 基线；澄清 U-03 |
| M2 | API/fixture 稳定，extractor 通过必要 V-1 后 cn/scanner 独立开发 | 模块各一人；lib.rs/Cargo 主实施者集成 | V-2/V-3 局部、host/WASM 同语料 |
| M3 | 实际前置通过，初始 AST/session/protocol 尚在形成时串行集成 | 主集成人协调公共类型与 runtime | V-4、基础 V-6 |
| M4 与 M5 | 已验证必要 family 后，GPU 可与其余 DOM family 并行 | property-order/AST/shared registry 单人；控件各单写者 | family V-5 与局部真实 V-7，含 F-02 |
| M6 | 完整 CSS 与 GPU 协议就绪；raw cn 接入按 F-01 修订 | runtime/宿主清理串行集成，Playwright 单操作者 | crossing V-2、V-6/V-7/V-8/V-9 |
| M7 | 成果齐备，冻结 API 后可准备文档 | 主实施者单写最终证据与状态；单 project，测 GPU 不并行编译 | 完整 V-2–V-11 与 root/subpath 交付 |

共享 workspace、锁、protocol、xtask、sources 与 Playwright 配置的单一负责人已在计划第 431 行明确。[README 浏览器约束](../../README.md#browser-tests) 也要求 project 串行。无需新增形式化里程碑；当前没有工期数据，仅能确认拓扑上的汇合点，不能给工期关键路径或节省比例。

## 5. 总体结论与修订顺序

**建议状态：Blocked，与原计划自报状态一致。** A-1/A-2 尚缺实际证据；完整运行时实施不能直接标 Ready。F-01/F-02 是对应集成能力开工前应修订的局部契约问题，不否定全 Rust 路线，也不阻止 M1 上游调查、fixture 锁定与风险验证。

建议按以下顺序修订和推进：

1. 固化自定义/公开全局注册的资源分类与动态输入处理，准确说明 ScopeRoots、abort 清理与资源 base 的复用来源；修正 raw fragments → cn 的组件接入路径和 hover 的完整条件。
2. 将两条修订的可观察断言加入现有 V-2/V-7 及相应 M5/M6；其余账本、里程碑与集中终验安排可保留。
3. 执行 M1/V-1，实际解除 A-1/A-2；完整上游清单与 V-11 基线可重放。若失败，按计划记录技术阻塞，不放宽 CSP、不改成 TS 生产编译、不以减少兼容范围换取完成。
4. 只有关键前提有证据、资源与接口契约明确、上述局部修订已落回计划后，才能评估完整实施 Ready。M7 完整差分、故障、真实浏览器、旧主题回归与交付文档通过，才可宣称整期完成。

本报告已落盘并回读核验；末次核对计划 hash 与代码 HEAD 未变，新增写入仅本报告。未改计划状态，未实施功能，未发外部评论或消息。

## 6. 修订处置与复核

- **来源**：本报告原版 SHA-256 `bfe78d15509184b23be7bb19bf50f64049ce1513ecb339bedf4c340fe440f233`；本会话用户明确要求“不保留旧包，将需要的移到 cn 包里”。
- **目标**：[WASM-TAILWIND.md](WASM-TAILWIND.md)；修前 SHA-256 `1b4139a1b8f64e6b46e6092d23b5a173d06801d193ca80b8dc0353ad329c41c9`；修后 SHA-256 `6df61e59ce944e657036dd80c88fe7f712b2fa230f79ff0b7023f293e599d241`。
- **复核日期与代码基线**：2026-10-04，Australia/Sydney；HEAD 仍为 `fb288564caed23f79e34bd8ac28dae2f6f44bfeb`，dirty 仅计划与报告两份未跟踪文件，代码未修改。
- **本次范围**：完整移除旧合并包的用户决策及关联 F-01、组件接入、依赖、回滚和验收；没有重跑整份计划的独立多 agent 评审。

| 条目 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| 用户决策 → R-8 | 当前依赖仍见 components/Cargo.toml:14、lib.rs:104、macros/clx.rs:28–31、macros/variants.rs:204–220；用户要求删除包而非长期并存 | 已修订并复核 | §0.1/0.2/0.4、§1.1/1.2、§2/3/4/5.1、§8/9/10/11/12 | 必要 join、纯 variant 类生成与 trait 归 cn；默认/动态组件统一使用 cn；旧导出、锁项、normal/build/dev 依赖和 fallback 均列入删除合同 |
| F-01 | 原版的先旧 merge、再 cn 会丢失 raw 片段；该事实仍成立。原最小建议保留旧包已不符合新决策 | 已修订并复核 | §5.1 第 7–9 项，M2 静态迁移、M6 scope 配置，V-2/V-8/V-10 | 文档明确 raw → cn 一次合并、已提交 epoch 重算和失败保持；不保留旧 derive。仅关闭文档设计问题，代码删除/功能回归尚待实施 |
| F-02 | GPU hover 媒体条件与旧包删除无直接关联 | 本次范围外，待修订 | 原问题继续保留，§5.5/V-7 未修改 | 不计为已关闭；M5 hover 实现前仍需处理 |
| U-01/U-02 | 明示 M1 风险门仍未实测 | 待核实 | A-1/A-2 与 Blocked 保留 | 本次未构建 extractor、Worker 或运行 CSSOM 浏览器 probe |
| U-03/U-04 | 全局资源分类与完整上游账本仍待落实；旧包使用清单已加入 M1 | 待核实 | M1 新增旧包实际用法及冻结迁移 fixture；其余待核实项保留 | 不把分类账任务存在当作已产出或已通过 |

### 决策与边界

cn 指计划既定 `rustify-style` 的 `cn` 模块，不另造平行旧合并包。迁入本仓使用的纯 Rust 能力；Leptos 视图包装留在组件 crate，避免 cn 引入 DOM 依赖。必要的生成宏采用 `macro_rules`，不为替换本仓的简单枚举/类结构继续保留旧 derive 包或新建 proc-macro 发布包。

保留组件 props、`variants!/clx!/void!` 调用语法和生成类型名称、首项默认、caller 覆盖等行为；`rustify_components::tw_merge` 的公开路径明确移除，必要 trait 改从 `rustify_components::cn` 导入。不能同时承诺删除旧包和继续导出旧包完整 API。原始片段只交给一个 cn 实例；默认与上下文 cn 不是先后执行的两个合并步骤。

M2 的 cn 及静态组件迁移以 M1 已固定的 cn/迁移 fixture 为前置，不受 scanner 或 CSP 实测结果的无关阻塞；scanner 仍必须通过自身 V-1。M6 在此基础上接入动态配置。M2 退出增加静态组件局部 V-8 与旧包删除 V-10，M7 对所有 features/targets 的依赖图、静态/动态构建和真实界面再收口。七个里程碑、0/7 进度与空完成记录保持原状。

用户关于维护债务的要求作为本次设计依据；本次未调查旧包维护活动，不将“很久无人维护”写成独立核实的上游事实。历史来源说明可以保留旧包名称，未来依赖和运行链不得保留旧引擎。

### 修订后验证与结论

- 路径脚本 exit 0：26 个路径 token、9 个 MISSING；逐项仍为显式 ★ 拟新增路径，无新增错误复用。
- 进度脚本 exit 0：0/7，ERROR 0、WARN 1。WARN 由两份未跟踪文档产生；本次没有完成里程碑，也没有漏写实现记录。
- 实际执行 `mbx tree --help`，确认 `--workspace`、`--all-features`、`--target all`、`--edges normal,build,dev`、`--locked` 参数可用。依赖清零命令已列为 M2/M7 实施验收，**本次没有执行删除后依赖图检查**；当前代码仍依赖旧包。
- 回读受影响段落并核对需求 R-8 → cn 类工具/组件 → M1/M2/M6/M7 → V-2/V-8/V-10；修正所有计划内的旧包保留及回退口径。原报告第 1–5 节保留历史结论，并已加版本提示；本节是当前处置依据。
- 没有运行实现测试、构建或浏览器验收。文档修订与 F-01 的文档复核完成；尚未宣称新 cn 或旧包移除已实现。后续实施须同步 sources.lock.json 的宏归属以及 README、CLAUDE.md 等现状说明，本次不提前把它们改成未来已实现事实。

**当前仍为 Blocked**：A-1/A-2 缺实际证据，F-02 及原待核实项未在本次范围内解决。旧包移除决策已明确并完整落入计划，没有需再确认的包保留选择。

## 7. 动态入口与风险门：修订处置与复核

- **来源**：本会话用户确认将“extractor 非动态完整类名入口的必要前置；动态 CSS 补齐构建时缺失的 utility 规则；A-2 是本仓安装验证”更新到计划。修前报告 SHA-256 `7794d47239f6203d83bb8853b6fb23713f39adfc1f8f87e713c6233aefa7eec1`。
- **目标**：[WASM-TAILWIND.md](WASM-TAILWIND.md)；修前 SHA-256 `6df61e59ce944e657036dd80c88fe7f712b2fa230f79ff0b7023f293e599d241`；修后 SHA-256 `72d7cd343a9b140337bdaa85bfd869ad2518638f94f1e9d7027f3e11e19f7120`。
- **日期与代码基线**：2026-10-04，Australia/Sydney；HEAD `fb288564caed23f79e34bd8ac28dae2f6f44bfeb`，dirty 仅两份未跟踪文档，代码未修改。
- **范围**：R-2/R-4 的输入边界，U-01/A-1、U-02/A-2 的实际阻塞范围，相关 feature、M1/M2/M3/M6 与检查依赖；不重新开展完整独立评审，不修改 GPU hover 设计。

| 条目 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| R-4 完整类名入口 | 固定上游 [browser/src/index.ts](https://github.com/tailwindlabs/tailwindcss/blob/1334c99db8fd26c8ea065375dd9259800863f072/packages/@tailwindcss-browser/src/index.ts#L185-L203) 读取 classList，直接将新增类传给 compiler.build；不经过 Oxide extractor。已读取源码，浏览器包仅作为行为参考 | 已修订并复核 | §0.1/0.2、§1.3、§2、§5.2–5.4；V-4/V-6；M3 | 完整类名直接进 compiler；compiler 不隐式启用 scanner；关闭 scanner 的动态路径与重复 ensure 纳入实施验收 |
| U-01/A-1 与扫描依赖 | 原 M3 强制依赖 M2 scanner V-3，不是上述路径的技术必要条件；R-2 仍要求完整源码扫描 | 依赖已修订并复核，WASM 可行性待实测 | 账本、ADR-2、恢复快照、V-1.extractor、V-3；M1/M2/M3/M6；§11 | A-1 只阻塞扫描集成；M3 只消费 cn 与 V-1.worker/css，不等待扫描、M1/M2 全部完成；M6 的 scan_content/源码面板另消费 V-3，M7 保留全范围验收 |
| U-02/A-2 与动态 CSS | 上游已存在动态 CSS 编译路径；本仓 [serve.rs](../../xtask/src/serve.rs#L33) 的严格策略含 style-src self，不能据上游 style 更新方式宣称本仓安装已通过 | 验证契约已修订并复核，本仓可行性待实测 | A-2、§4/5.4、V-1.css、M1/M3、§11 | 最小新增 utility 的 CSSOM 安装、computed style、scope/property/nesting 和清理是实际风险门；compiler 开发不等待此门。未运行浏览器 probe，未宣称 A-2 已解除 |

### 设计与依赖复核

动态输入的职责链为完整类名 → 去重/已有集合检查 → 缺失候选规则及依赖生成 → 排序/安装 CSS → 提交 class/props。任意值、主题和 variant 按编译规则生成；缓存复用已有候选，但整表输出仍保持官方层级与排序，不改成按到达顺序任意追加。

V-1 保持原检查 ID，细分为 extractor、worker、css 三组，各组可单独运行与记录；M1 全部退出检查通过才记完成。M3 的动态 DOM 集成只消费 worker/css 及已通过的 cn 输出，compiler 开发可先推进。扫描能力留在本期，通过 M2 局部 V-3、M6 输入集成与 M7 终验交付；没有删减需求或伪造里程碑完成。

### 本次检查与当前结论

- 路径脚本 exit 0：26 个路径 token，9 个 MISSING；均仍是明确标注的拟新增路径。
- 进度脚本 exit 0：0/7，ERROR 0、WARN 1；WARN 来自两份未跟踪文档，没有实现完成记录。
- 文档断言检查通过：旧 M3 扫描强制前置与整体运行时阻塞措辞已删除；三组检查、compiler/scanner 分离和 M6 扫描接入一致。实施协议与完成记录未变，恢复快照仅更新下一步与阻塞范围；七个里程碑保持不变。tracked 文件无差异。
- 本次只运行文档检查，未实施 compiler/extractor、未构建 Worker、未在浏览器验证样式安装；上游源码调查与本仓实测明确区分。

**当前全范围状态仍为 Blocked，局部工作可以推进**：A-1 的未实测只阻塞扫描集成；A-2 的未实测只阻塞 DOM 安装集成，不阻塞 cn/compiler 开发。Worker/CSS 安装组通过后，M3 无须等待扫描器。F-02 和其他原待核实项仍未处理，旧包完整移除决策保持有效。本节结论取代第 6 节对本次受影响依赖的表述，不改写历史评审。
