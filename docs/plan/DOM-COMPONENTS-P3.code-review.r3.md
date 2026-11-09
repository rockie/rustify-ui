# DOM-COMPONENTS-P3 代码评审 · R3（修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3 | [DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md)（v0.2，Ready / 7/7 已完成） | `181215eb05299bb7d717c21d837a66352bf64601`（R2 审到的目标提交）→ `c658b831c81f5b2dd9072c904e4b5c3703499868`（HEAD）；工作树 dirty 仅 SDLC 台账回写，无代码差异（`git status --porcelain` sha256 `57c67640…`，`git diff HEAD` sha256 `2f3d050e…`） | [DOM-COMPONENTS-P3.code-review.r2.md](DOM-COMPONENTS-P3.code-review.r2.md)（sha256 `62c7945a…`） |

## 复审记录

- 上轮报告：R2 审查 `68dde297`→`181215eb`（工作树 clean），判定「需要修改」（patch is incorrect，限其 diff）；F-01 已解决，新增 F-02（P2）。
- 本轮核对版本：HEAD `c658b831`；范围内代码改动仅 `f2bb1bb1`（`tests/dom-only/hydration.spec.ts`，+6/−2），其余为计划快照、M7 记录、R2 报告与台账（`c658b831`）及未提交 SDLC 台账。
- 本轮范围：修复效果复审——F-02 逐项关闭核对、修复 diff 新增/激活缺陷检查、R1/R2 遗留待核实项复核。
- 处置记录：本轮未做任何修复；报告与台账为唯一落盘产物。

## 上轮问题复审表

| 来源 | 问题（摘要） | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R2 F-02（P2） | F-01 修复未同步 hydration 断言，dom-hydrate 门 2/15 失败 | `f2bb1bb1`（2026-11-03，只改测试） | **已解决** | 断言按 R2 修复方向更新且未弱化：`tests/dom-only/hydration.spec.ts:130-136` 三个 disclosure 改为按初始 `open` 分支——打开断 `aria-controls`==panel id、关闭断属性缺失，`aria-labelledby` 三者保留；`:171`、`:175` 在展开 `history`/`delivery-more` 后补断 `aria-controls` 指向新挂载面板。产物有效性核实：`target/dom/hydration` 为 2026-11-03 11:36–11:39 由含 `3d078e29` 门控的 `181215eb` fixture 源码构建，其后 fixture 源码未变（`f2bb1bb1` 只改 spec）。本轮实跑 `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts`：**15 passed（33.6s）**；CI `verify.yml:170-171` 同 spec，预期恢复绿。产品门控未被回退（`collapsible.rs:87-89` 仅 open 输出） | 无 |
| R1 F-01（P3，经 R2 判已解决） | 关闭态 disclosure `aria-controls` 悬空 | `3d078e29`（R1 授权修复） | **已解决**（沿用 R2，本轮抽验） | `git diff 181215eb c658b831 -- crates/` 为空；`crates/rustify-components/src/primitives/collapsible.rs:87-89` 现状与 R2 三态核对一致（open 才输出 content id，默认卸载/`force_mount` 语义不变）；`panels_ssg` 22/22 沿用 R2 在 `181215eb` 的实跑（其后无 Rust 改动） | 无 |
| R1 待核实 | open 期间卸载唯一 Content 是否留死遮罩 | 无处置（R2 静态反证增强，倾向不成立） | **待核实** | `overlay.rs` 自 `68dde297` 未变（`git diff 68dde297 c658b831 -- crates/rustify-dom/` 为空）。本轮重读静态链完整：`watch_lifecycle`（`overlay.rs:771-811`，observe_tree/resize/scroll → bump `moved`）→ 帧效应（`overlay.rs:1989-2013`，追踪 `moved` 调度 measure）→ measure（`overlay.rs:1857-1885`，面板断连 → `alive=false`，`seen_panel`/`reported_dead` 护栏下 `on_close()` 恰一次）→ 帧渲染（`overlay.rs:2117-2121`，`!alive` → `visibility:hidden`+`pointer-events:none`+`inert`，遮罩随帧隐藏）。仍无浏览器实证：现有 fixture 无卸载 floating Content 的控制 | 可选：临时复现页实证后关闭（非阻塞） |
| R1 待核实 | 默认卸载语义 | 用户 2026-11-01 裁定复刻 Radix | **不再适用** | 与 R2 相同，实现与裁定一致，无待决事项 | 无 |
| R1 记录 | Toast Viewport 单例、`role="status"` 播报节点 | R1 判非缺陷观察，未要求处置 | **本轮未复核** | `rustify-dom` 自 R1 后未变，R1 记录继续有效 | 无 |
| R1 待核实 | `68dde297` hydration/feedback 用例未跑 | R2 已闭合（并由此发现 F-02） | **已闭合** | 本轮 dom-hydrate 15/15 再证；feedback 由 R1-s1 实跑（41 passed / 1 skipped） | 无 |
| R2 环境备注 | docs-verify 末步因本机 Python 缺 tomllib 失败 | 非缺陷，环境限制 | **本轮未复核** | 本轮未跑 docs-verify（所需 hydration 产物已在，无构建需求），备注继续有效 | 无 |

## 确认问题

未发现可确认的问题。修复 diff 仅强化验收断言（关闭态断缺失、展开后断指向），未触及产品代码；文档/台账改动与实际提交一致。

## 待核实/待决策项

- **死遮罩疑点（R1 起遗留，非阻塞）**：静态链在当前版本完整成立（见复审表），但无浏览器实证。关闭方式：在隔离临时目录复刻 consumer fixture，增加「卸载开放层唯一 Content」的控制，卸载后点击 scope 与 Escape，断言遮罩不可见/不可拦截且 close 恰请求一次。WORK-N6 整条在 P4 终验，本轮不强制处置。

## 覆盖与验证

意图：核对 F-02 修复效果并检查修复增量的新缺陷。仓库根 `/Users/rockie/Documents/gh-rockie/rustify-ui`，评审日 2026-11-03；HEAD `c658b831`，比较方式为两端树净差异 `git diff 181215eb c658b831` + 工作树 dirty（仅 `.xgent-ai/sdlc/**` 台账，指纹见评审对象）。计划完成记录登记的 6 个提交（`3f23c064`、`4152c4c2`、`ef8b0156`、`ec6bd506`、`3d078e29`、`f2bb1bb1`）全部解析为完整 SHA 且在 HEAD 祖先链。排除（非本计划代码，不计结论）：`5e20a074`、`3e16d326`、`1c97d60c`、`047639dc`（计划/SDLC 回写），`23050140`、`439929de`、`95911f87`（skills/AGENTS），`c658b831`（R2 报告与台账）。

已读：`f2bb1bb1` 全量 patch、M7 记录与计划快照 diff、未提交台账内容、`collapsible.rs:80-97`、`overlay.rs:771-811/1852-1912/1988-2032/2096-2135`、`hydration.spec.ts:126-180`。

| 命令 | 版本/对象 | 结果 |
| --- | --- | --- |
| `git rev-parse --verify '<sha>^{commit}'` + `git merge-base --is-ancestor <sha> HEAD`（6 个登记提交） | HEAD `c658b831` | 全部解析成功且在祖先链 |
| `git diff --stat 181215eb c658b831 -- crates/ tests/ xtask/ web/` | 修复增量 | 仅 `tests/dom-only/hydration.spec.ts`（+6/−2） |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` | HEAD 产物（2026-11-03 11:36–11:39 构建，fixture 源码未变） | **15 passed（33.6s）** |
| `git diff 68dde297 c658b831 -- crates/rustify-dom/` | 死遮罩链文件 | 为空（自 R1 目标起未变） |

未覆盖/沿用：`panels_ssg` 22/22、SSG/Clippy 等未重跑（`181215eb` 后无 Rust 改动，直接引用 R2 实跑）；docs-verify 未跑（tomllib 环境限制且无构建需求）；死遮罩无浏览器实证（见待核实项）；M1–M7 原始变更范围未重审（R1 已覆盖并通过，R2/R3 为修复闭环）。评审方式：单 agent，全部结论回到目标代码/报告原文核对。

## 计划符合性

- 计划：[DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md) v0.2；本轮对应 M7 登记的评审修复 `f2bb1bb1` 与 V-9 浏览器门。
- V-9（WORK-N4）：`--project=dom-hydrate hydration.spec.ts` 在当前版本 15/15，静态 SSG 侧证据沿用未失效；F-02 关闭后 V-9 门恢复，M7 的 V-9 证据链由 [M7 记录](DOM-COMPONENTS-P3.records/M7.md)「评审修复」节如实衔接。
- 计划快照与完成记录的「修复 `3d078e29`、`f2bb1bb1`」登记与实际提交一致；修复只改测试，无越界改动。

## 总体结论与修复顺序

**通过验收。**

修复效果复审结论：**已解决**——所选旧问题 F-02 按关闭判据解决（断言同步且实跑 15/15），F-01 维持已解决，无回归、无新增问题；「默认卸载语义」不再适用，其余为非缺陷观察或已闭合项。patch 结论（仅覆盖本轮 diff `181215eb..c658b831`）：**patch is correct**。

限制：死遮罩疑点仍为待核实（静态链完整、无浏览器实证，非阻塞）；Toast Viewport/`role="status"` 观察未复核（代码未变）；tomllib 环境备注继续有效。本判定覆盖 R2 目标到当前 HEAD 的修复增量及全部旧问题复审，不重开 R1 已通过的原始范围，也不是合并、发布或需求验收授权；VoiceOver/真实 IME/真机等人工 missing 按计划归 P5。

下一步（可选，均由你发起）：

```text
review-prd-dev-gaps docs/plan/DOM-COMPONENTS-P3.md
```

（计划来自 PRD；如需核对 PRD 与交付差异可用上条。当前版本增量仅测试与 ARIA 属性，M7 已有真实浏览器走查；如仍要独立 UI 验收，可 `review-ui docs/plan/DOM-COMPONENTS-P3.md`。均不需要时，本计划代码验收完成。）
