# DOM-COMPONENTS-P3 代码评审 · R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 | [DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md)（v0.2，Ready / 7/7 已完成） | `614173ec`（`3f23c064^`）→ `68dde297`；工作树 `95911f87` clean，无未提交差异 | 无 |

纳入：`3f23c064`（M1 基线测试）、`4152c4c2`（M2–M4）、`ef8b0156`（M5–M7）、`ec6bd506`（CSS 整理），以及用户确认纳入的未登记提交 `68dde297`（Progress 动效）。排除：`5e20a074`、`3e16d326`、`1c97d60c`、`047639dc`（计划/SDLC 回写）、`23050140`、`439929de`、`95911f87`（skills/AGENTS 规则），不计入代码结论。

## 确认问题

### F-01 · [P3] 默认关闭的 Collapsible/Accordion 内容被卸载，Trigger 仍指向不存在的 id

- 定位：`crates/rustify-components/src/primitives/collapsible.rs:87`、`crates/rustify-components/src/primitives/collapsible.rs:151-152`
- 问题：`Content` 以 `Show when=force_mount || open` 渲染且 `fallback=||()`，默认 `force_mount=false` 时关闭状态完全没有关联节点，而 `Trigger` 无条件输出 `aria-controls=<该 id>`（`collapsible.rs:87`）。Radix 参考实现按打开状态门控同一属性（vendor `collapsible/src/collapsible.tsx:99`），默认卸载（`:136`）与 `forceMount` 时 `id` + `hidden={!isOpen}`（`:212-222`）与本实现一致；Accordion 直接复用 Radix 的 Collapsible Trigger/Content（`accordion/src/accordion.tsx:476-477,516-517`）。因此这是与复刻对象的一处偏离：默认组合产生悬空 IDREF，影响辅助技术关联与 ARIA 校验，`aria-expanded` 仍可操作。
- 证据：`collapsible.rs:87`（无条件 `aria-controls`）、`collapsible.rs:151-152`（`Show` 卸载分支）；vendor `collapsible.tsx:99`（Radix 仅在 open 时输出）、`:136`（默认卸载）、`:212-222`（forceMount 的 id/hidden/子节点）；`tests/panels_ssg.rs:80-84` 与 consumer 披露用例只覆盖 `force_mount=true`，默认关闭路径无断言。
- 修复与验证：按 Radix 门控，仅在 `open` 时输出 `aria-controls`（若要在 `force_mount` 保留面板时保持关联，用 `open || force_mount`，此时 id 存在）；默认卸载不变。补一条默认关闭的 SSG 断言：Trigger 无 `aria-controls` 或所指 id 存在。
- 置信度：高（源码直接对照；默认卸载语义已由用户裁定，见下）。

## 待核实/待决策项

- **默认卸载语义（已裁定）**：用户 2026-11-01 明确「开发计划是复刻 Radix UI，要与 Radix 一致」。Radix 默认在关闭时卸载并要求 `forceMount` 才保留，故默认卸载成立；计划 §5.2「关闭/未选内容必须 hidden/inert」按复刻语义读作保留内容的要求，无需修改实现，必要时补一句计划措辞说明（评审未改计划）。F-01 的修法因此确定为 Radix 门控 `aria-controls`。
- **仅剩的 Content 在 open 期间卸载会留下不可关闭遮罩**：`overlay.rs:2079` 在 panel 缺失时提前返回，`Content` 清理调用 `overlay::on_layer_content_cleanup()`（`floating.rs:512`）→ `deactivate`（`overlay.rs:2034-2047`）只释放注册，不置 `alive=false`；图层外框仅 `hidden=!active`（`overlay.rs:2118`），遮罩按 scope 尺寸且 `pointer-events:auto`（`css/primitives.css:46-52`、`dom-runtime.css` `.rustify-layer`）。初始缺 Content 时 `alive=false` 会 `visibility:hidden`/`pointer-events:none`，故该故障需要 Content 先挂载再被条件卸载（`<Portal><Overlay/><Show when=…><Content/></Show></Portal>`）。未在浏览器复现，未进确认问题；复现方式：按该结构卸载唯一 Content 后点击 scope 与 Escape。
- **Toast `Viewport` 单例**：`toast.rs:620` 直接覆盖 `runtime.viewport`，清理时无条件 `try_set(None)` 并清除 hover/focus 暂停（`toast.rs:627-631`）。同 scope 出现两个 Viewport（作者自组或两个 Toaster）会互相覆盖，存活视口的暂停失效。文档未声明支持多 Viewport，未计为缺陷。
- **断言式播报节点用 `role="status"`**：`toast.rs:649`，assertive 仅靠显式 `aria-live`；规范上显式 `aria-live` 覆盖角色隐含值，实际朗读应正确，故仅记录。
- **未复核提交的边界**：`68dde297` 只经静态阅读（`scaleX` + `:dir(rtl)` 原点、测试同步更新），未运行其列出的 hydration/feedback 用例。

## 覆盖与验证

意图：在同页 DOM 面板中新增可组合浮层、切换/展开原语、内容与反馈部件，并保持受控值、焦点、通知与静态输出契约。仓库根 `/Users/rockie/Documents/gh-rockie/rustify-ui`，评审日 2026-11-01；HEAD `95911f87`，`git status --short` 为空，比较方式为两端树净差异 `git diff 614173ec 68dde297`。

已读行为链：`rustify-dom/src/overlay.rs`（全文件：栈、dismiss 仲裁、inert、Tab 收束、通知宿主）、`overlay/placement.rs`、`overlay/hierarchy.rs`（含 6 个迁移单测）、`toast.rs`/`toast/model.rs`/`toast/browser.rs`（调度、暂停组合、拒绝关闭、滑动、热键、播报）、`primitives/floating.rs`（Portal/Overlay/Content、open/close auto focus、AlertDialog 策略）、`primitives/tabs.rs` 与 `accordion.rs`/`collapsible.rs`（焦点/选中分离、RTL、动态恢复、关联 ID）、Themes `dialog.rs`，以及 `css/primitives.css`、`css/panels.css`、`dom-runtime.css`、`docs/panels.md` 相关段落和 `tests/panels_ssg.rs`、`tests/dom-only/panels.spec.ts` 的相关用例。

未覆盖：`primitives/scroll_area*`（约 1 150 行）、`content` 家族（table/data_list/avatar/aspect_ratio/separator/media）、`hover.rs`/`hover_grace.rs`/`tooltip.rs` 细节、`catalog.rs` 新增项、`tests/dom-only/*.spec.ts` 全量断言、`tests/dom-only/public-panel-consumer`。R1 初次结论时未运行浏览器；本轮补证已按下方命令执行 Playwright（见「补证记录」），仍未运行无障碍工具，不声称 AT 朗读通过。

实际命令（目标版本 `68dde297`）：

| 命令 | 结果 |
| --- | --- |
| `git status --short` / `git rev-parse HEAD` | 空；`95911f87` |
| `mbx xtask css --check` | `css: no drift (155851 bytes; 5349 SDK candidates)` |
| `mbx xtask catalog --write docs/components.md --check` | `catalog: no drift (171 public entries, 97 stable reference names, 24 legacy categories)` |

评审方式：单 agent；使用 3 个只读 scout 做分区定位，所有结论均回到目标代码原文核对；scout 提出但被反证或证据不足的候选（Tabs 焦点抢回、滑动阈值、`insert` 父节点、`role` 播报、Progress ARIA）未计入确认问题。

## 计划符合性

计划：[DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md) v0.2；本期任务 M1–M7（面板原语、内容/反馈部件、集成与性能收口）。

| 计划项 | 结果 |
| --- | --- |
| §5.1 单 Layer/Portal、Overlay 与 Content 兄弟、无效 anchor 生命周期、无死遮罩 | 结构与 inert/焦点部分与代码一致；内容被卸载后的遮罩行为见待核实项 |
| §5.2 Tabs/Accordion/Collapsible 受控值、roving、RTL、动态恢复、关闭内容 hidden/inert | 受控值与 roving 一致；默认卸载与 Radix 一致（用户裁定）；仅 `aria-controls` 门控偏离参考实现（F-01） |
| §5.3 Dialog/AlertDialog hook、AlertDialog 只经 Action 确认 | 与代码一致（`floating.rs` 策略 + `panels.spec.ts` 用例） |
| §5.6 单 timer、等待不耗时、暂停组合、拒绝关闭、播报分区 | 调度模型与代码一致且有单测；播报角色见待核实项 |
| §5.7/§5.8 皮肤、静态输出、catalog/CSS 生成物 | CSS 与 catalog 生成物无 drift（已实跑） |

## 补证记录（R1-s1）

补证触发：用户 2026-11-01 要求补齐 R1 的缺口；范围、基线与只读边界不变。目标是补上浏览器证据、内容/滚动区块的行为覆盖，并纠正 R1 的一处调用错误。

| 命令（目标 `68dde297`） | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg --test content_ssg --test feedback_ssg --test ssr_controls --test missing_context_ssr` | 21 + 26 + 14 + 13 + 10 全部通过 |
| `mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml` | 成功；`target/dom/dom-consumer`（修好 `target` 的失效 mbx 软链后） |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only panels.spec.ts` | 53 passed（40.3s） |
| `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only content.spec.ts feedback.spec.ts` | 41 passed、1 skipped（浏览器 UI 缩放，环境限制；55.6s） |

订正：R1 首次运行 SSG 用例时误用默认 feature（csr+ssr），出现 12 个 `Executor::spawn_local` 失败；按计划/CI 记录的 `--no-default-features --features ssr` 重跑全部通过，原失败不成立，未计入问题。

新增覆盖：滚动区逻辑（`primitives/scroll_area/model.rs:50-61` 的 RTL 逻辑坐标与 clamp、`scroll_from_pointer` 单测）与内容/反馈家族由上述 spec 实际执行覆盖（表格、列表、图片回退、Avatar 换源、进度/骨架屏、通知队列与滑动）。

仍未覆盖：水合接管（`--project=dom-hydrate`）未执行——`mbx xtask build-dom` 不是该 fixture 的构建入口（manifest `default = []`，需逐模式 feature），正确入口是 `mbx xtask docs-verify --group features` 的 `build_hydration`；未运行无障碍工具、移动真机与视觉采集。`primitives/table.rs`、`data_list.rs`、`avatar.rs` 等未逐行读源码，仅由通过的浏览器用例覆盖行为。

## 总体结论与修复顺序

**通过验收。**

补证后在已执行范围（84 个 SSG 用例、94 个浏览器用例、CSS/catalog 生成物检查）内未发现 P0–P2 缺陷，仅保留一条 P3（F-01，修法已按 Radix 复刻口径确定）与一条待核实项（Content 条件卸载后的遮罩状态）。R1 初次结论为「需补证」，现被本轮证据取代。

限制：水合接管 spec 未由本次执行覆盖（构建入口用错，正确入口 `mbx xtask docs-verify --group features`），内容家族部分只经浏览器行为用例验证；本结论不保证不存在未知缺陷，也不等于 UI/无障碍验收、合并或发布授权。

验收判定只覆盖 `614173ec..68dde297` 与当时工作树，不是合并、发布或需求验收授权。

## 修复记录（R1 授权修复）

2026-11-01 用户回复「改」，授权按 F-01 的 Radix 复刻口径修复；原始评审基线与 F-01 结论保持不变，以下是修后结果，不等于已完成修复效果复审。

- 改动：
  - `crates/rustify-components/src/primitives/collapsible.rs:87`：`aria-controls` 改为仅在 `open` 时输出，与 vendor `collapsible.tsx:99` 一致；默认卸载与 `force_mount` 保留不变。
  - `crates/rustify-components/tests/panels_ssg.rs`：`force_mounted_disclosures_are_inert_and_keep_heading_associations` 调整为「打开才校验 `aria-controls`，关闭保留面板只校验 id/aria-labelledby」；新增 `a_closed_disclosure_never_points_at_an_unmounted_panel`。
- 验证（修后，目标 `68dde297` + 未提交改动）：
  - `mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg`：22 passed。
  - `mbx clippy --locked -p rustify-components --all-targets -- -D warnings`：通过。
  - `mbx xtask build-dom --manifest-path tests/dom-only/consumer/Cargo.toml`：成功。
  - `npx --no-install playwright test -c playwright.docs.config.ts --project=dom-only panels.spec.ts`：首次 52 passed / 1 failed（`core hierarchy keeps descendant portals...`），该用例单独重跑与第二次全量（53 passed）均通过，判为 flake，未复现。
  - 测试运行改写的 `docs/validation/dom-components-p3/*.png` 已还原；工作树只保留上述两处源码/测试改动与本报告（未提交）。
- 复审：F-01 仍需按 `$review-code` 的复审约定核对关闭（默认卸载、`force_mount`）与打开三态后记「已解决」；本记录不把 P3 直接改写为已关闭。
