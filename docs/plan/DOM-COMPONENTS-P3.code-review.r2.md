# DOM-COMPONENTS-P3 代码评审 · R2（修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2 | [DOM-COMPONENTS-P3](DOM-COMPONENTS-P3.md)（v0.2，Ready / 7/7 已完成） | `68dde297`（R1 审到的目标提交）→ `181215eb`（HEAD）；工作树 clean，无未提交差异；范围内代码改动仅 `3d078e29`（F-01 修复），其余为 skills/AGENTS/docs 回写 | [DOM-COMPONENTS-P3.code-review.md](DOM-COMPONENTS-P3.code-review.md)（R1，含 R1-s1 补证与修复记录） |

## 复审记录

- 上轮报告：R1 审查 `614173ec`→`68dde297`（工作树 `95911f87` clean），判定通过验收，遗留 F-01（P3）与待核实项；用户随后授权修复，处置见 R1「修复记录」（`collapsible.rs` aria-controls 门控 + `panels_ssg.rs` 断言更新，即本轮范围内提交 `3d078e29`）。
- 本轮核对版本：HEAD `181215eb`，工作树 clean（`git status --porcelain` 为空）。
- 本轮范围：F-01 三态关闭核对、R1 待核实项逐项复核、修复 diff 新增/激活缺陷检查。
- 处置记录：本轮未做任何修复；报告与台账为唯一落盘产物。

## 上轮问题复审表

| 来源 | 问题（摘要） | 本轮结果 | 本轮证据 |
| --- | --- | --- | --- |
| R1 F-01（P3） | 关闭态 disclosure 的 `aria-controls` 指向未挂载面板 | **已解决** | 门控改为仅 open 输出（`collapsible.rs:87-89`），与 vendor `collapsible.tsx:99`（`aria-controls={open ? contentId : undefined}`）一致；三态核对：默认卸载（新增 SSG 测试断言无 aria-controls 且 body 未渲染）、`force_mount` 关闭（`panels_ssg` 断言 id/aria-labelledby 保留、aria-controls 仅 aria-expanded=true）、打开（aria-controls==panel id）。`panels_ssg` 在 HEAD 22 passed；accordion 经 `collapsible::trigger_as` 复用同源门控（`accordion.rs:183`）。浏览器证据沿用 R1 修复记录的 `panels.spec` 53 passed（其后仅 skills/docs 提交，代码未变） |
| R1 待核实 | 默认卸载语义 | 不再适用 | 用户已裁定复刻 Radix；实现与裁定一致，无待决事项 |
| R1 待核实 | open 期间卸载唯一 Content 留下不可关闭遮罩 | 待核实（反证增强，倾向不成立） | 见「待核实项」①；`rustify-dom` 自 R1 起未变（`git diff 68dde297 181215eb -- crates/` 仅 collapsible/panels_ssg） |
| R1 记录 | Toast Viewport 单例、role=status 播报节点 | 本轮未复核 | R1 已记录为非缺陷观察，代码未变 |
| R1 待核实 | `68dde297` 未运行其列出的 hydration/feedback 用例 | 已闭合 | feedback 由 R1-s1 跑（41 passed / 1 skipped）；hydration 本轮执行（见 F-02） |

## 确认问题

### F-02 [P2][置信：高] F-01 修复未同步 hydration 验收断言，V-9 浏览器门在 HEAD 失败

主定位：`crates/rustify-components/src/primitives/collapsible.rs:87-89`（`3d078e29` 引入的门控）。`3d078e29` 把关闭态 disclosure trigger 的 `aria-controls` 从恒输出改为仅 open 输出（产品口径正确，见 F-01），但受其影响的验收 spec 未同步：`tests/dom-only/hydration.spec.ts:126-134` 对 fixture 三个 disclosure（`summary-trigger`、`history-trigger`、`delivery-more`）一律断言 `aria-controls`==content id，而 `tests/dom-only/hydration/src/view.rs:330-341` 中 `history`（accordion item）与 `delivery-more`（collapsible）初始均关闭且 Content `force_mount=true`，修复后二者关闭态不再有 `aria-controls`。实测（HEAD 产物，2026-11-03 重建）：`npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` → 13 passed / 2 failed，失败用例 `hydration.spec.ts:491`、`:519` 均在 `:131` 断言 `catalog-a-history-trigger` 的 `aria-controls` 时收到缺失（expected `rui-catalog-a-collapsible-content-162`，got null）。CI `verify.yml:170-171` 运行同一 spec，HEAD 会红。基线 `68dde297` 上该 spec 通过，缺陷由修复增量引入且未被修复期验证捕获（R1 已记录 hydration 未跑的缺口）。影响面已排查：`primitives.spec.ts:638/647` 为 Select 断言、`dom-buttons.spec.ts:122` 为 Button 显式 `controls` 属性演示（`button_fixture.rs:113-115`），均不受影响。修复方向：按 `panels_ssg.rs` 已更新的模式改 `hydration.spec` 断言（`aria-expanded=true` 才断言 `aria-controls`==id；关闭态断言属性缺失；`delivery-more` 保留 id/aria-labelledby 关联断言），重建 hydration 产物后重跑 dom-hydrate 预期 15 passed。不应改回 `open || force_mount` 门控——那会偏离已裁定的 vendor 口径（vendor 仅 open 输出）。

## 待核实项

1. **open 期间卸载唯一 Content 是否留死遮罩（R1 遗留）**：本轮静态反证显著增强。`overlay.rs:771-811` `watch_lifecycle` 在 scope container 布树观察（observe_tree）+resize/scroll 监听，任何子树变更（含 Content 卸载）bump `moved`；帧效应（`overlay.rs:1994-2017`）追踪 `moved` 调度 `measure`；`measure`（`overlay.rs:1863-1900`）发现 panel 断连即 `alive=false` 并请求 close 恰一次（`seen_panel`/`reported_dead` 护栏）；帧随之 `visibility:hidden`+`pointer-events:none`+`inert`（`overlay.rs:2118-2121`），帧内遮罩一并隐藏。R1 的疑点链路（deactivate 不清 alive）未计入该路径。仍未浏览器实证：现有 fixture 无卸载 floating Content 的控制（parts-dialog 只能切 Title/Description/Cancel；popover-remove 走 anchor 失效路径；life 实验为 raw Layer）。倾向不成立；如需关闭，须加临时复现页面验证帧隐藏与 close 单次请求。
2. **环境备注（非缺陷）**：本轮 `mbx xtask docs-verify --group features` 在末尾 `check_component_site` 步骤因 `docs-pages-prepare.py` 缺 `tomllib`（本机 python3 <3.11）失败；`build_hydration` 在其前已成功，hydration 产物为修复后代码（11:36–11:39 生成），不影响本轮结论。

## 覆盖与验证

| 命令 | 版本/对象 | 结果 |
| --- | --- | --- |
| `git status --porcelain` / `git rev-parse HEAD` | HEAD `181215eb` | clean；无未提交差异 |
| `git show 3d078e29` | 修复 diff | 与 R1 修复记录一致（collapsible.rs + panels_ssg.rs 两文件） |
| `mbx xtask docs-verify --group features` | HEAD | hydration 产物构建成功；末尾 docs-pages-prepare.py 因本机 Python 缺 tomllib 失败（环境限制，见待核实②） |
| `npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts` | HEAD 产物 | 13 passed / 2 failed（1.4m；失败均在 `hydration.spec.ts:131` 关闭态 aria-controls） |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --test panels_ssg` | HEAD | 22 passed / 0 failed |

静态核对：`collapsible.rs`（HEAD 全文）、`accordion.rs`、vendor `collapsible.tsx:90-110/200-230`、`overlay.rs:771-811/1846-1900/1975-2130`、`hydration` view/spec、`primitives.spec.ts:628-650`、`dom-buttons.spec.ts:115-125` + `button_fixture.rs:113-115`、`verify.yml:170-171`。未复核：全量其余 spec（无代码触碰面）、R1 记录的 `panels.spec` core-hierarchy 偶发观测、死遮罩浏览器实证（见待核实①）。

## 计划符合性

- F-01 修复与计划 §5.2/§3「复刻 Radix」口径及用户裁定一致，无越界改动。
- 计划 V-9（浏览器证据链）现状：修复后代码使静态 hydration spec 失败（F-02），M7 的修复前 V-9 证据不再覆盖当前版本；按 AGENTS.md「浏览器测试」需重跑对应门。
- 修复 diff 无未授权范围扩张。

## 总体结论与修复顺序

**验收判定：需要修改。**

patch 结论（仅覆盖本轮 diff `68dde297..181215eb`）：**patch is incorrect** —— F-01 修复本身正确且已解决，但该增量未同步受影响的 hydration 验收断言，使 V-9 浏览器门与 CI 在 HEAD 失败（F-02，P2）。上轮修复效果：F-01 **已解决**；其余旧问题见复审表。

修复顺序：

1. F-02：按 panels_ssg 模式更新 `tests/dom-only/hydration.spec.ts:126-134` 断言，重建 hydration 产物并重跑 dom-hydrate（预期 15 passed）。
2. 可选：死遮罩疑点按待核实①方式浏览器复证后关闭。

下一步：

```text
apply-code-review docs/plan/DOM-COMPONENTS-P3.code-review.r2.md
```

处理后以 `$review-code` 复审（对照本报告逐项核对 F-02 关闭）。
