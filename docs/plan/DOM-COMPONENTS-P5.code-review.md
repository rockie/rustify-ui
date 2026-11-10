# DOM-COMPONENTS-P5 代码评审 R1

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 | [DOM-COMPONENTS-P5 · 原应用迁移与整体交付](DOM-COMPONENTS-P5.md) | 基线 `3735889c89319be11b0d25857f2a2730d96c6e97` → 目标 `7eba08292e89bd0de1188da60bb93d7bc6a24973`（登记提交 `500022a57f6d79cce11272d6f967e47908689572`=M1–M3、`7eba0829`=M4 的净差异；中间仅进度回写 `15d67062`，HEAD `118202ba` 相对目标仅计划/台账文档）；工作树干净（`git status` 为空，无 staged/unstaged/untracked） | 无（本计划首次代码评审） |

## 确认问题

### F-01 · [P1] 补上 DataTable/Tree 的 empty_label 实现，或改正四处虚报它的目录与文档

- 定位：`crates/rustify-components/src/catalog.rs:965-966`（DataTable/Tree 两行的 `fields` 与 limitation 首个虚报点）；缺实现锚点 `crates/rustify-components/src/data_table/view.rs:50-81,592`、`crates/rustify-components/src/tree.rs:137-147,293`（两组件 props 均无 `empty_label`，空态固定渲染 `Message::Empty`）
- 问题：计划 §3 第 1 条承诺为 DataTable、Tree 增加 `#[prop(optional, into)] empty_label: String`（空串回退 locale 消息），本变更没有实现该 props，却在四处公开产物中把它写成真实参数：`catalog.rs` 两行 fields 与 limitation、生成文档 `docs/components.md:77-78`、`docs/data.md:254-258`、迁移指南 `docs/dom-components-migration.md:476-481`。按这些文档写 `empty_label="…"` 的消费者直接编译失败，应用无法覆盖空态文案；WORK-N5/AC-2「目录声明的交互可对应真实结果」与 WORK-F23/AC-2 的迁移事实源对本参数为假。V-3/V-5 只校验生成器一致性（CATALOG ↔ markdown）与 LEGACY_ENTRIES 完备性，不校验 fields 与真实 props 的一致，故全部检查通过也未暴露。
- 证据：`grep -rn empty_label crates/` 仅命中 `catalog.rs`；本报告逐字读出两个组件的完整 props 签名（view.rs:50-81、tree.rs:137-147）确认无该 prop；`tests/extensions_ssg.rs` 全部空态断言只用 SDK 默认词；M1–M4 记录无任何删除/推迟 `empty_label` 的偏差或豁免记录；对照 `invalid`（FilePicker/DropZone）已按 §3 实现，缺失仅此一项。
- 修复与验证：二选一——实现 `empty_label`（两组件 props + 空行/`tree-empty` 渲染分支 + `extensions_ssg` 增自定义文案断言），或确认本期裁剪并同步改 `catalog.rs` fields/limitation、`docs/data.md`、迁移指南后 `mbx xtask catalog --write docs/components.md --check` 再生成，同时在完成记录补偏差；建议给 `fields` 与真实 props 的一致性加最小断言防同类漂移。
- 置信度：高（机械核对源码签名与四处文档，可复验）。

### F-02 · [P3] 计划 §5.4 尾段仍是 PanelTabs 的中间设计，与交付结构和同节首段矛盾

- 定位：`docs/plan/DOM-COMPONENTS-P5.md:470`（目标版新增文本）
- 问题：该行仍描述 M1 中间形态——「关闭按钮不再嵌套在 tab 内，删除 stop_propagation」「关闭按钮 mr-1.5 self-center」，与同节首段修订后的终态（「内含 label 与可及树外的鼠标 close span；close 点击 stop_propagation」）和实际交付（close span 是 `role=tab` 子节点、`on:click` 带 `stop_propagation`、间距由 TRIGGER 的 `gap-1.5` 提供，见 `crates/rustify-components/src/workspace/panel_tabs.rs`）相反。「实施进度 → 恢复快照」是跨对话唯一续做入口，后续维护者按 470 行执行会移除 stop_propagation，重新引入首段已修复的「点关闭误激活 tab」缺陷。
- 证据：`panel_tabs.rs` 终态结构与 `tests/extensions_ssg.rs` 的 `a_panel_tab_is_a_tab_and_its_close_button_does_not_choose_it` 断言（close 为无 role/tabindex、`aria-hidden` 的 span 且点击不选 tab）。
- 修复与验证：把 470 行改写为与首段和代码一致的终态描述；对照 `panel_tabs.rs` 复读该节确认无残留矛盾。
- 置信度：高。

## 待核实 / 待决策项

- 浏览器与取证套件（V-7 全量项目 167 项、V-8 官方站三宿主、V-9 矩阵深浅双模式、V-10 真实 Chrome 200%、V-11 性能对）本轮未复跑；结论采信 M4 记录与已提交证据 `docs/validation/dom-components-p5/*.json`（来源为记录，非本轮实测）。
- `legacy_baseline.txt`（675 键）的「基线提取本身完备」依赖 M3 记录的机械提取方法（基线 `11b69129`）；本轮实测了逐键恰一解析（catalog 15/15）并抽查 ThemedScope/ButtonVariant 取值/Tooltip.children/semantic 三键/file、css 行，未重新枚举基线全量公开名。
- `500022a5` 同时捆绑约 1350 个 agent skills 文件（`.agents/`、`.claude/`、`.github/skills/`），非计划 §1.2 范围（M4 记录已注明由用户提交），未纳入评审。
- V-12 的 VoiceOver、真实 IME、物理触控为 M4 记录在案的 missing，未变。

## 覆盖与验证

意图一句话：把扩展组件契约（主题角色/部件状态/空与错误/语义修正/默认文案/Tree 图标/Glyph）、Studio 扩展陈列与本地化、迁移事实源（LEGACY_ENTRIES+目录修正+指南重整）与终验收口作为本期代码变更审查，重点核对声明与交付一致。

- 仓库根 `/Users/rockie/Documents/gh-rockie/rustify-ui`；评审日 2026-11-09。输入 `docs/plan/DOM-COMPONENTS-P5.md`（HEAD `118202ba`）；解析为登记提交 `500022a5`、`7eba0829`（均在 main），比较方式为两提交净差异 `3735889c..7eba0829`；工作树干净，无未提交/untracked 范围；排除进度回写 `15d67062`、`118202ba`（仅计划/台账/评审文档）及 `500022a5` 内的 skills 文件。
- 逐文件读取的产品 diff：`crates/rustify-dom/src/i18n.rs`；`crates/rustify-components/src/{icon.rs,file_picker.rs,drop_zone.rs,tree.rs,data_table/view.rs,workspace/{command_palette,panel_tabs,splitter}.rs,catalog.rs}`、`catalog/legacy.rs`、`catalog/legacy_baseline.txt`、`tests/extensions_ssg.rs`、`css/{rustify.css,candidates.json}`、README；`examples/theme-studio/{src/{gallery.rs,gallery_catalog.rs,gallery_extensions.rs,main.rs},app.css,README.md}`；`tests/browser/{gallery-matrix.ts,theme-gallery.spec.ts,theme-acceptance.spec.ts,theme-config.spec.ts,theme-export-ui.spec.ts,theme-zoom.spec.ts}`、`tests/docs-ssg/official.spec.ts`、`tests/native-zoom.ts`；`.github/workflows/verify.yml`；`docs/{components.md,data.md,dom-components-migration.md,themes.md,workspace.md}`。验证 JSON（`docs/validation/dom-components-p5/`）作证据引用未逐条展开。
- 实际命令（均在本工作树 = 目标产品代码）与结果：`RUSTFLAGS='--cfg rustify_repo_tests' mbx test --locked -p rustify-components --lib catalog` 15/15 通过；`mbx test --locked -p rustify-dom --lib i18n` 6/6 通过；`mbx test --locked -p rustify-components --no-default-features --features ssr --test extensions_ssg` 7/7 通过；`mbx xtask css --check` 无漂移（169645 B / 5561 候选）；`mbx xtask catalog --write docs/components.md --check` 无漂移（182/97/24）。静态核对：PanelTabs 终态与 axe 取舍、CommandPalette aria 门控、Splitter valuemin/max 与拖拽同夹取、Tree 空根降级、DataTable 空行 ARIA、glyph/Message 计数、`theme_controls.rs:739` 的 `1.00:1 · Fails text thresholds` 与测试断言一致、`theme-dom.spec.ts:66` 仍覆盖 `preview-dialog-close`、SubmitButton/FormStatus/Primitives.Label/DirectionProvider 目录声明属实。评审人：主 agent 单独评审，未派并行 reviewer。
- 未覆盖：浏览器行为实测、性能与体积数字复核、基线公开名全量重提取、skills 文件。

## 计划符合性（计划：DOM-COMPONENTS-P5.md @ HEAD）

- WORK-F19（§5 扩展契约）：已核实——部件/状态属性、空与错误、语义修正、默认文案、Tree 图标、Glyph 七枚与 i18n 四消息逐项落地且 SSG 测试通过；唯 §3 第 1 条的 `empty_label` 未交付（F-01）。
- WORK-F22（§6 Studio 陈列）：记录与静态核实一致——49 组/9 节、六类样本、局部主题边界、GalleryLocale；PanelTabs axe 三轮就地修订已记录并回写计划（但残留 F-02 尾段矛盾）。
- WORK-F23（§3 事实源）：LEGACY_ENTRIES 152 行覆盖 675 键恰一解析经测试证实；目录 §5.8 修正项（Label/DirectionProvider/Slottable/SubmitButton/FormStatus/四组件 fields）属实，除 `empty_label` 虚报（F-01）。
- WORK-N1/N2/N7：extensions_ssg 入 verify.yml 与全量 SSG 列表、components.md 无漂移、workspace/data/themes/迁移指南同步（内容受 F-01/F-02 影响）；`=0.2.1` 配对句已补。
- 未核实/后续范围：V-7—V-13 的浏览器、官方站、性能、双端结论采信记录（见待核实）；V-12 三项 missing 沿用。

## 总体结论与修复顺序

**验收判定：需要修改**（patch is incorrect）

F-01 构成本期需求缺口：计划明确承诺的公共参数未实现，且目录、生成文档、data.md、迁移指南四处把它发布为真实参数，直接违背本期「目录与交付一致」的核心承诺；F-02 为计划恢复快照内的矛盾残留。其余已核实范围内（组件契约、SSG 测试、Studio 陈列代码、测试与生成产物一致性、五项宿主检查）未发现其他缺陷，遗留实现质量问题无。建议顺序：先对 F-01 定夺实现或裁剪并同步四处文档与记录，再顺手修 F-02 的 470 行；处理后按本报告发起 `review-code` 复审。本判定仅覆盖所审提交与工作树，不是合并、发布或需求验收授权。
