# DOM-COMPONENTS-P1 · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，开发计划模式完整初审 | [DOM-COMPONENTS-P1 · Colors 与统一主题基础](DOM-COMPONENTS-P1.md)，SHA-256 `7e28a72926e2a3ba8b908918746cf0cb9cdb301bc5ca1563281d9488fb39c87b` | `11b6912919a2a34c372aac4d6b7d51052820600f..3193791720edbb1c933bfa852773f0b4710a7b15`；另核对当前未提交材料，未发现未提交产品代码改动 | 无 |

本次变更的明确意图是交付可独立静态消费的 Colors、v2 作者主题与统一解析、基础 DOM 组件、Studio 配置—精调—保存—导出闭环及独立消费者；新增呈现路径为 DOM，既有 GPU 仅承接共享快照与恢复回归。用户明确输入只有计划路径；提交标题和文件内容用于确定实际代码范围，不把作者“全部完成”的声明当作验收证据。

## 1. 确认问题

### F-01 · [P2] 单边 Inset 不应抵消相邻三边的容器 padding

- 定位：[crates/rustify-components/src/layout.rs:679-684](../../crates/rustify-components/src/layout.rs#L679-L684)，target `3193791720edbb1c933bfa852773f0b4710a7b15`，新增 `InsetSide::edges` 映射。
- 问题：`Top` 返回 `[true, true, false, true]`，`Bottom` 返回 `[false, true, true, true]`，`Left` 返回 `[true, false, true, true]`，`Right` 返回 `[true, true, true, false]`，分别激活三边而不是所选择的一边。`Inset` 在 `layout.rs:749-769` 对每个 active 边生成 `calc(margin - var(--rustify-inset-*))`，所以 `side=Top` 会同时抵消上、右、左 padding，其他单边也一样。实际结果是内容跨出未选择的容器边缘并被错误裁剪，违反该 prop 的公共语义。
- 证据：计划 [§5.6 的 Inset 契约](DOM-COMPONENTS-P1.md#L481-L483)要求 `side` 决定消除最近容器 inset 的方向，不允许成为任意外溢路径；`layout.rs:675-685` 的 `X/Y` 已正确只选水平/垂直两边，证明数组顺序就是 top/right/bottom/left。现有 `tests/browser/dom-layout.spec.ts:148-170` 只覆盖默认 All 与嵌套场景，`crates/rustify-components/tests/theme_layout.rs:82-84` 只覆盖 X，因此当前测试无法发现四个单边映射错误。
- 修复与验证：把 Top/Bottom/Left/Right 分别改成仅对应边为 `true` 的映射。为七个 `InsetSide` 变体逐边断言生成的 margin 与圆角，并覆盖响应式从单边切换到另一边，确认未选边保留原 margin。
- 置信度：高；完整映射与直接消费者均已阅读，触发不依赖特殊环境。

### F-02 · [P2] 排队 autosave 必须重新校验变化后的 ResolveContext

- 定位：[examples/theme-studio/src/persistence.rs:291-299](../../examples/theme-studio/src/persistence.rs#L291-L299)，target `3193791720edbb1c933bfa852773f0b4710a7b15`，新增 `serialize_snapshot` 的 proof 核对。
- 问题：一个写操作等待前序 Web Lock 时，后续 autosave 已捕获 `Snapshot.proof`，但出队准备使用的是当时的当前 `ResolveContext`。这里只核对 resolved snapshot 的 document/id/name，不核对 proof 与当前 rem、字体及解析策略是否一致，因而能把入队时的校验结果误用于已变化的上下文。比如草稿含 `radius: 200rem`，在 rem=16 时合法并入队；前序锁等待期间 rem 变为 32，出队后仍用 16px proof 写入，而 32px 下 6400px 超过 4096px 上限。最终 localStorage 中保存了当前上下文完整校验和下次加载都会拒绝的 v2，启动进入坏缓存保护，违反本期“失败保留有效主题与可取回数据”的契约。
- 证据：`controller.rs:552-568` 在入队时生成 proof，`controller.rs:681-691` 在出队时另取当前 context；`metrics.rs:36-47` 执行 4096px 上限。隔离复现直接包含当前 `persistence.rs`、`state.rs`、`write_queue.rs`：同一草稿在 16px 构造 proof，`Writer::prepare` 在 32px 返回成功并产出字节，而 `persistence::validate` 与 `persistence::load` 在 32px 均拒绝这些字节。实际命令为 `mbx run --manifest-path target/review-dom-components-p1/Cargo.toml`，输出 `reproduced: 16px proof was serialized at 32px; full validation and reload reject it`；临时 crate 执行后已删除。计划 [§3.3 的出队验证/序列化](DOM-COMPONENTS-P1.md#L381-L387)要求候选在出队后验证并保留失败数据。
- 修复与验证：在 Snapshot 中绑定完整 context key；出队时若 key 不同，使用当前 context 重新解析并校验草稿，不能仅比较 rem。保留 context 未变时复用 proof 和未变命名库缓存的优化。增加真实锁延迟用例：16px 入队 A/B，A 等待时切到 32px，B 不得写入旧 proof；随后断言原文未被无效候选替换、显式失败可重试且重开可恢复。
- 置信度：高；目标代码路径、上下文变化入口和最小运行复现一致。

### F-03 · [P3] 有效 v2 恢复后应初始化当前草稿的已保存事实

- 定位：[examples/theme-studio/src/controller.rs:173](../../examples/theme-studio/src/controller.rs#L173)，target `3193791720edbb1c933bfa852773f0b4710a7b15`，新增有效存储初始化 `Writer::new` 的调用。
- 问题：有效 v2 已从 localStorage 读出并校验，`Writer::new` 却固定把 `acknowledged` 设为 `None`（`write_queue.rs:155-168`）。`connect()` 随后调用 `refresh_storage()`，在没有待写、冲突或写保护时仍落入“当前保存未确认”的分支（`controller.rs:608-613`），把已落盘且未变化的当前草稿误报为只在内存。每次正常 reload 都会出现错误的保存状态，削弱用户对保存/恢复结果的判断；这不影响实际字节，故为 P3。
- 证据：在当前既有 release（build manifest `e26d1534a1255858`，与 M7 最终记录产物一致）真实执行保存名为 `Review reload status` 的主题：保存后状态为 `Saved on this device`；重载恢复同名草稿，状态变为 `Draft remains in memory; no current save has been confirmed.`，`snapshot.storage.acknowledged=null`、`editor_epoch=0`，pageerror 为空。计划 [§3.3 的保存状态](DOM-COMPONENTS-P1.md#L386-L387)要求“当前草稿已保存”对应当前代次、完整 draft/偏好与无更晚待写；初始有效恢复可以满足该条件。
- 修复与验证：仅在“读取到有效 v2、baseline 存在且 Permission::Ready”时，以恢复出的 draft/preferences 和初始 epoch 初始化 acknowledged；新库、坏库、legacy 待确认/取消不得初始化。增加有效 v2 reload 状态测试，并分别覆盖新库和 legacy 待确认，避免把未提交升级误显示为已保存。
- 置信度：高；初始化与状态分支静态一致，且已由真实浏览器保存—重载复核。

## 2. 待核实 / 待决策项

没有需要补外部事实才能裁定的候选问题，也没有产品范围待决策项。

验证边界不作为额外 finding：本轮未重跑完整 Playwright 项目、VoiceOver、真实 OS IME、Firefox/Safari 或真实 GPU 性能测量；这些在 M7 中有明确历史记录和人工缺口，但本轮只读取记录，不把它们计为当前独立通过。生成颜色表/CSS 由确定性命令核对，验证 JSON/PNG 未逐文件人工复审；未发现据此需要保留的代码待核实项。

## 3. 覆盖与验证

### 3.1 固定范围与工作树指纹

- 仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`，main；评审日期 2026-10-27。
- 原输入：`docs/plan/DOM-COMPONENTS-P1.md`。基线为最早登记提交 `d60a8e112d7393bf6844c20205544eedd9459372` 的第一父提交 `11b6912919a2a34c372aac4d6b7d51052820600f`，目标为当前 HEAD `3193791720edbb1c933bfa852773f0b4710a7b15`。
- 登记且均验证在当前分支的提交：`d60a8e112d7393bf6844c20205544eedd9459372`、`219b8481cc5c2d38e060c36a9b27c902a4188b14`、`620d25b17cd44f0249d49e6fbf45fcae1e6e50d1`、UI 修复 `d712990cac7d71c7e0b2a224c035370cf21cb84c`、`afa813e2266510e234e184095311424c1f9c5f1d`、验收截图 `1e4d379bf34c53b8b0b13f445caf2dd5ca49b6a8`。`git rev-list` 中其他提交均只修改本计划、其记录或验收资料，没有未登记产品代码提交，因此采用连续净差异而非逐提交叠加。
- 累计对象：602 文件、71,898 行新增、1,388 行删除（包含生成资产与验证证据）。未限定文件。历史源码证据从目标树/当前一致工作树读取，基线差异使用 `git diff --no-ext-diff --no-textconv`。
- 当前 tracked 工作树 diff 只含 `.xgent-ai` 既有索引；index 为空。未跟踪的 PRD、计划评审及 SDLC 文件已枚举，其中 PRD v0.2 作为需求输入读取；这些非代码材料不作为本轮新增 finding 的 diff 锚点。
- tracked diff SHA-256：`5a0ae3442729760ef01c71fd4b177545e29b4e0ff23f6e70e0b5aa9869e691e8`；空 index diff SHA-256：`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`；33 个未跟踪文件的 `路径+NUL+文件SHA-256+LF` 汇总清单 SHA-256：`317dd5abb7dd66fb1a239b966d70e16288a15078f7993b607abe98847a5c65ca`。报告与本评审新台账不纳入该输入指纹。
- 报告写入前 HEAD 与上述 SHA 未变；目标产品代码没有未提交改动。临时复现位于 `target/` 并已删除，不修改目标源码或测试。

### 3.2 已审行为链

- Colors：vendor 常量解析、31 色族/双模式/alpha/sRGB/P3 查询、静态 CSS 根与回退、包内资产 API、来源和生成命令。
- 主题内核：v1/compact v2/full v2 分派、未知字段/token/重复键与大小限制、base/config/overrides 优先级、双模式 source、auto-gray、单次尺度与全部阴影层、导入导出及显式 v1 损失。
- Provider 与组件：根 appearance/preview、嵌套 boundary 与非法回退、CSSOM 差异投影和释放；Button/IconButton/Link 原生行为；布局、排版、响应式、CSS 值校验、SSR 外部样式及受控 ThemePanel。
- Studio：44 预设、EditorState 历史/手势、v1→v2 完整库迁移、备份、单写者队列、锁内比较、冲突/清除、生命周期取消、独立 DOM/Docs 消费及保存状态。
- 配套：Docs/CLI 主题调用、wasm-bindgen snippet 打包、release name section 处理、能力目录及迁移资料。直接调用者、测试与 M1–M7 记录用于确认影响和既有验证边界。

本轮由主 agent 独立审查，没有并行 reviewer 或把他人结论当作证据。

### 3.3 实际执行的验证

以下命令均针对上述目标源码；同一命令链实际完成，未把历史记录冒充本轮结果。

| 命令 / 步骤 | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-dom --lib` | 257 passed / 0 failed |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | 165 passed / 0 failed |
| `mbx test --locked -p theme-studio --bins` | 64 passed / 0 failed |
| `mbx xtask colors --check` | 256 个确定性 Colors 文件一致 |
| `mbx xtask sources verify` | exit 0；新增 Colors 5、Themes 39 文件无修改；既有 fork drift 报告保留 |
| `mbx xtask css --check` | 无漂移，62,536 B / 4,435 candidates |
| `mbx xtask catalog --write docs/components.md --check` | 无漂移，59 public entries / 97 stable reference names / 24 categories |
| `git diff --check 11b69129…31937917 --` | exit 0 |
| 隔离临时 crate：16px proof → 32px `Writer::prepare` | 复现 F-02：序列化成功，当前上下文完整校验和 reload 拒绝；执行后删除临时文件 |
| 现有 release 保存→reload（真实 Chromium） | 复现 F-03：保存前状态正确，reload 后误报未确认；`Review reload status` 草稿本身恢复，pageerror 为空 |

浏览器核对通过 `mbx xtask serve --example theme-studio --release --port 4199` 启动既有产物，完成后已停止。`build-manifest.json` 的 build id `e26d1534a1255858` 与 [M7 最终 artifacts](../validation/dom-components-p1/acceptance/artifacts.json) 一致；M7 记录的源码树与当前目标只相差计划/证据提交。未重建或并行运行其他 Playwright 项目。

现有测试全绿不覆盖 F-01 的四个单边变体、F-02 的锁等待期间 context 变化及 F-03 的 reload 保存状态；这些问题因此不是通过简单增加无关全量测试就能发现的回归。

## 4. 计划符合性

| 计划范围 | 评审结果 |
| --- | --- |
| WORK-F16/AC-1—AC-2、M4/V4：布局语义与局部 Inset | **F-01**：四个单边方向与承诺不符，需修复并补矩阵。 |
| WORK-F21/AC-1—AC-3、WORK-N6/AC-1、M5/V6：写入快照、失败保留与恢复 | **F-02**：变化后的 context 可写入随后不可读的数据；**F-03**：恢复后保存状态误报。其余迁移备份、锁内比较、队列顺序与生命周期链路未发现同级缺陷。 |
| WORK-F1/F2/F3/F4/F15/F18/F20/F23、相关 NFR | 所审 Colors、v2/双模式/覆盖/尺度、原生按钮与排版、导入导出及能力资料中，未发现其他达到缺陷门槛的问题；这不等于重新完成整期全部浏览器验收。 |
| V9 性能与资源 | M7 如实保留旧颜色负载 CSS 提交 p95 26.4ms 高于配对旧站 15.4ms，且计划明确无新增毫秒 SLO；本轮未据此另造性能 finding，也未独立复测性能。 |

P2–P5 后续组件、通用 Primitives、完整 Inspector、GPU 对等目录、公开发布与部署不在本期，不将未实现这些范围记为缺陷。人工 VoiceOver、真实 OS IME 和真人研究继续按计划作为未验边界保留。

## 5. 总体结论与修复顺序

**验收判定：需要修改。**

**patch is incorrect**：F-01、F-02 是已证实的 P2 正确性/数据完整性问题；F-03 是额外 P3 保存状态问题。报告落盘与当前定向测试通过不改变该判定，也不构成 merge、发布或需求验收授权。

修复顺序：

1. 先修 F-02 的 context 绑定与出队重验，解除持久化无效数据及坏缓存风险；用真实锁延迟和 reload 验证。
2. 修正 F-01 的四个单边映射，补齐七个 side 与响应式回归。
3. 初始化 F-03 的有效恢复保存事实，覆盖有效 v2、新库和 legacy 待确认三种启动路径。随后重跑本报告相关 DOM/Studio 测试与受影响浏览器场景。

下一步：

> $apply-code-review docs/plan/DOM-COMPONENTS-P1.code-review.md

修复后应再用 `$review-code docs/plan/DOM-COMPONENTS-P1.md` 复审，逐项关闭旧问题并检查修复回归。
