# DOM-COMPONENTS-RELEASE · 开发计划评审报告 R2

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md) | 工作树未提交版，613 行（sha256 前 16 位 `e4b253b17d805374`，未入 Git） | 全文（§0–§12、实施进度 4/7、M1–M4 记录核验；重点为剩余 M5–M7 与 V-14–V-19） |

本轮为对扩展后版本的**完整评审**（非复审）。上轮 [R1 报告](DOM-COMPONENTS-RELEASE.review.md)（464 行版）11 项问题的并入情况见覆盖摘要末节；R1 结论与处置记录保持原样。

---

## 1. 确认问题

未发现可确认的问题。

四个视角（facts / architecture / schedule / delivery）的排查均无候选问题存活至裁定。曾重点怀疑、最终以代码证据排除的风险项列在覆盖摘要，供后续复审直接引用。

## 2. 待核实/待决策

无。

生产环境事实（crates.io 十包 0.3.0 在架、线上 build_id `3fe8d486f30d8aa5`、workflow run 42953922752）本轮以 M1–M4 记录为据，未联网复核；这不构成 M5–M7 的实施前置缺口——V-18 会在部署后于线上重新核验同一组事实。

## 3. 覆盖摘要

**基线**：2026-11-11 17:43 +0800 · HEAD `921802b3c9d1022033bd500681b870a430652616`（main）· dirty（计划本身、M4 记录、`.xgent-ai/sdlc/` 台账、两份代码评审报告、`docs/validation/dom-components-release/`）。所审计划为当前工作树版本；代码事实核自同一工作树。

**评审方式声明**：初轮按调度规则实际启动了 4 个独立只读 subagent（四视角各一）；全部于 17:44 前后因平台 5 小时使用限额中断（限额 18:59 重置），中断时无成品结论可回收。按调度规则的失败兜底，由**主 agent 接管四视角顺序评审**并完成裁定——本报告为单 agent 顺序四视角评审，非独立多 agent 复核；此为本报告的覆盖限制之一。

**脚本校验**：

- `check_paths.sh`：56 个路径 token 中 MISSING 5，全部为计划 §1.2 标 ★ 的新增路径（`site/src/showcase.rs`、`site/theme.json`、`site/static/colors/`、`scripts/docs-pages-compose.py`、`docs/validation/dom-components-release/showcase-coverage.md`），符合预期，无虚构「已核实」路径。
- `check_progress.sh`：进度 4/7 · 里程碑表 7 行 · 记录 4 行（已完成 4）· ERROR 0 · WARN 0。

### facts：关键主张核实表

| 主张（计划定位） | 类型 | 证据定位 | 结果 | 影响 |
| --- | --- | --- | --- | --- |
| site 仅 24 demo、`category` 依赖 SDK CATALOG、Button/Menu/Dialog 为简单样例（§1.1） | 现状事实 | `site/src/lib.rs:9-34`（`site!` demos 恰 24 项）、`site/src/pages.rs:2,29-33`、`site/src/demos/basics.rs:5-24`（仅 Increment/Reset/Unavailable）、`feedback.rs:8-96` | 已核实·足够 | §5.5 升级对象真实且形态一致 |
| Studio 三 gallery 文件、九组陈列（§1.1） | 现状事实 | `examples/theme-studio/src/gallery{,_catalog,_extensions}.rs`；`gallery_catalog.rs:14` `ALL: [Self; 9]`、`:51` ENTRIES、`:355` EXCLUDED | 已核实·足够 | §6.2 矩阵与 gallery 对得上 |
| Studio `app.js` 相对 wasm（§5.6） | 现状事实 | `examples/theme-studio/app.js:4`（`new URL("./theme-studio.wasm", import.meta.url)`） | 已核实·足够 | 同包复制后自包含可用 |
| `build-web --base` 与子路径部署（§5.6） | 现状事实 | `xtask/src/main.rs:24`；`playwright.config.ts:246` theme-deep 已按 `--base` 走 Playwright | 已核实·足够 | 子路径 Studio 构建已被现有测试证明 |
| verify.py 拒绝 manifest 外文件、每 HTML strict meta CSP、已有 build_id 复算（§5.6） | 现状事实 | `scripts/docs-pages-verify.py:28`（META_CSP）、`:192-193`、`:159-172`（Python 端 sha256/build_id 复算已存在） | 已核实·足够 | compose 复用既有算法，「不增加第二套指纹」成立 |
| prepare.py 全树保护、仅投影 site manifest/lock（§5.6） | 现状事实 | `scripts/docs-pages-prepare.py:6,27`（`PROJECTED = {"site/Cargo.toml", "site/Cargo.lock"}`） | 已核实·足够 | 「不需新增投影」成立；M5 给 site 加 colors 依赖走投影路径 |
| Colors API/README/css 目录结构（§5.5、§6.2） | 现状事实 | `crates/rustify-colors/src/lib.rs:70-81`（`scale`/`css`）；README「31 families」；`css/` 逐族×模式文件，p3 文件相对 `@import` sRGB 文件（如 `amber-dark-p3.css:2`）；`vendor/colors/LICENSE` 存在 | 已核实·足够 | 目录整体原样复制保留相对导入；SVG `fill` 属性不受 CSP `style-src` 约束 |
| official.spec.ts 从旧 CATALOG 推导、假设一页一 island（§1.1） | 现状事实 | `tests/docs-ssg/official.spec.ts:15`（解析 `pub const CATALOG`）、`:111`（`toHaveCount(1)`） | 已核实·足够 | V-14 的扩展点是真实存在的测试假设 |
| `theme.document` 配置与标准主题存在（§5.5） | 现状事实 | `crates/rustify-docs-cli/src/theme.rs:33,116-140`（`config.theme_document`、"theme.document" 诊断）；`crates/rustify-dom/src/theme/document.rs:412`（`standard`） | 已核实·足够 | `site/theme.json` 接线路径真实 |
| `Manifest::of` 算法与嵌套 build-manifest 结构（§5.6） | 现状事实 | `crates/rustify-docs/src/publish.rs:62-94`；`xtask/src/build.rs:34-35,200-219`（`files: BTreeMap<String,u64>`、`size_report`、`finish`） | 已核实·足够 | 嵌套清单「以新 index.html 字节更新该行」与现状结构一致 |
| workflow 顺序 prepare→build→identity→verify→archive（§5.6） | 现状事实 | `.github/workflows/docs-pages.yml:67,79,94,106,113` | 已核实·足够 | 「组合必须在 identity/verify/archive 之前」的改造点明确 |
| serve.sh official 分支逐 base 重建（§5.6） | 现状事实 | `tests/docs-ssg/serve.sh:103-131`（按 `$base` 重写 config 后重建） | 已核实·足够 | `RUSTIFY_STUDIO_DIR` 指向「按相同 base 预构建」Studio 的扩展方式与现状兼容 |
| M1–M4 历史与记录 | 历史事实 | 8 个短 SHA 全部存在（53ff0666/3f2fd1e1/2445aea6/dde6fe85/6993fed9/8d3544bb/596175e3/458333d5，`git log` 逐一对上）；本地标签 `rustify-libs-v0.3.0`、`rustify-docs-cli-v0.3.0`；M4 记录与快照、run ID、build_id 一致 | 已核实·足够 | 快照 4/7 与完成记录、里程碑表、记录文件四方一致 |
| site/Cargo.toml 已有 ssr/hydrate feature 与 registry 依赖范式（V-16、§1.2） | 现状事实 | `site/Cargo.toml:17-24`（features）、`:29-33`（`rustify-dom =0.3.0` 走 registry） | 已核实·足够 | V-16 命令可执行；site 增 `rustify-colors =0.3.0` 沿既有范式 |
| rustify-docs.toml 已含三篇指南（M1 记录声明） | 现状事实 | `rustify-docs.toml` include 与 nav.groups Guides 均含 menus/panels/dom-components-migration | 已核实·足够 | M1 自述与代码一致 |

内部矛盾扫描（范围/排除、路径三处表述 §2 vs §5.6、V-15 与 serve 约定、C-7 与 §2/§5.5 边界）：未发现互斥陈述。§2「`/theme-studio/` 是完整应用」与 §5.6「入口固定为 `/rustify-ui/theme-studio/`、根路径验收时为 `/theme-studio/`」是站点内路径与含 base 部署路径的两种表述，可同时成立。

### architecture：关键决策判断表（含重点排除的风险项）

| 选择 | 驱动需求 | 证据 | 判断 |
| --- | --- | --- | --- |
| Studio HTML 注入与 `META_CSP` 相同的 meta CSP | §5.6/§7 | `xtask/src/serve.rs:36-48` Strict 头与 META_CSP 逐指令等价（meta 标签不能携带 `frame-ancestors`，故 META_CSP 略去该项）；`playwright.config.ts:245` theme server 默认 `--csp strict`，Studio 浏览器测试长期跑在该策略下；Studio 动态颜色/几何走 CSSOM（`inspector.rs:241-243`、`color_editor.rs:615`、`app.rs:526-528`），不受 `style-src 'self'` 约束；`script-src` 含 `'wasm-unsafe-eval'` | **可行，有行为级依据**。曾怀疑 P1「注入后 Studio 无法启动/颜色不可见」，证据排除 |
| compose.py 以 Python 复算根 manifest | §5.6 | `docs-pages-verify.py:159-172` 已用 Python 实现同一 sha256/build_id 算法（与 `publish.rs::Manifest::of` 数值一致：逐文件 sha256、组合哈希前 16 hex） | 复用仓内已有跨语言实现，无第二协议漂移风险 |
| 绝对 URL 链接 + 组合期 href 本地化 | §5.6（docs 构建期无 Studio 文件） | 未发布站内目标在生成期被拒（`content.rs:3509` 等）；绝对 URL 为确定字符串，生成 HTML 的 href 逐字保留，可精确替换且不误伤同域其它链接；搜索索引不存内容 href，正文文本中残留的线上 URL 在生产环境为真值 | 可行且边界可执行；「不得泛化重写其它外链」可落地 |
| ADR-2 方案 B（Docs 与完整 Studio 分开部署、同包组合） | R-5 | theme-deep 项目已证明 `--base` 子路径 Studio 可构建运行；文档页不加载 Studio wasm 由静态组合保证，跨实例不共享内存符合运行时契约 | 保留；备选 A 的淘汰理由成立 |
| 一页多个具名 demo island（§2/§5.5） | R-4 | 生成器以 ` ```rustify-demo name: X ` 围栏嵌入（`writing-demos.md` 指南），一页多 island 为既有能力，非生成器改造 | C-7「不改生成器通用行为」与 §2 目标无冲突 |
| V-16 把组合器错误路径纳入 `tests/crates-io/test_pairing.py` | §9.1 | 该文件已加载并测试 `scripts/docs-pages-prepare.py` 的配对身份（`test_pairing.py:11-16`），组合器同属 Pages 配对链路 | 位置合理；「先写非法输入复现」符合仓库规则 |

比例原则双向核对：§8 四项失败模式均为真实部署边界（组合失败、CSP 拦截、部署后缺陷、生成器不符），检测/恢复有落点；V-16 局部测试有准入（发布边界 + 修复前先复现）；未发现缺依据的防御或验收门。

### schedule

- **映射双向核对**：账本 R-1–R-6、C-1–C-7、A-1–A-3 全部有设计/落点（§2、§5.5–§5.7、§6）→ 检查（V-14–V-19）→ 里程碑（M5–M7）；PRD §6.3 中与公开展示相关的 UAT 行由 V-19 三角色任务覆盖，迁移行已由 V-13 T4 闭环。无断链、无以无关检查占位。
- **依赖边**：M4 →（M5 ∥ M6 同批，共享文件已指定单人边界）→ V-16 批末 → V-14/V-15/V-17 同一轮三宿主 → 冻结 S → 用户授权 → dispatch → V-18 → V-19 → M7 完成。无循环、无倒置、无隐藏跨系统前置。
- **收口唯一性**：V-17 是 M5–M6 批唯一完整本地收口；V-18 唯一生产门；V-19 唯一真实使用终验；中间里程碑退出条件均无全量回归；V-14/V-15 与 V-17 合并同一轮 official-root/subpath/meta 命令，不重复运行。
- **C-4**：V-1 循环逐项目、V-17 逐条串行、V-5 全部命令串行——每命令一项目、不并发，守住。
- **成本标注**：V-1 引用 P5 §9.4 实测（约 6.5 分钟/项目）；V-17 要求实测记录；V-5 总耗时如实写「未知」。
- §9 从 9.2 直接跳 9.4（无 9.3 标题）：内容完整、仅编号缺行，不影响实施，不作为问题报告。

### delivery（证据等级：文档推演 + 历史浏览器证据背书）

- 用户主路径闭环：发现（首页三层能力 + Components 索引，§6.1，V-14 场景一）→ 操作（demo 交互/变体尺寸，§6.2，V-14）→ 带走（复制色值/下载 CSS/导出主题，§6.2/§6.4，V-14/V-15）→ 消费（独立 registry 0.3.0 项目，V-19 ③）→ 失败恢复（§8 各失败的用户表现 + V-17 console/pageerror/永远 loading 判失败 + Studio 自身 fatal 状态）。端到端无断点。
- §6.2 矩阵支撑：抽核 38+ 符号全部存在于 SDK 公开面（`ButtonVariant::ALL`/`ButtonSize::ALL` 在 `button.rs:22,56`；CommandPalette/FilePicker/DropZone/Splitter/PanelTabs/RadioCards/CheckboxCards/RangeSlider/SegmentedControl/AspectRatio/DataList/Kbd/AccessibleIcon/VisuallyHidden/Direction/Portal/FormRoot/SubmitButton/FormStatus 等命中 `lib.rs`/专文件；Menubar/NavigationMenu/AlertDialog 为模块级部件组 `menubar.rs`/`navigation_menu.rs`/`alert_dialog.rs`，与矩阵「部件」口径一致；ThemePanel 在 `components/src/lib.rs:131`）。
- §6.4 Studio 完整路径锚点：预设（`presets.rs`）、撤销/重做（`state.rs:308-376`、`controller.rs:520`）、保存恢复（`persistence.rs`）、导出（`data_tools.rs`）、Inspector（`inspector.rs`）、九组陈列（gallery）——均为已交付能力，M5–M7 只做公开接线与组合，不要求新功能。
- V-19 三角色（只读官网的读者/Studio 用户/在独立 registry 消费主题的 DOM 开发者，含故意导入非法文件恢复）可执行、覆盖 PRD §6.3 展示相关行；「干净用户状态」与发现问题的修复/延期机制已写明。
- impeccable 技能缺失的处置（§6.3：说明 + 提示安装 + 不声称已用）符合仓库「前端 UI 开发与验收」规则。
- **证据等级**：M5 展示页未实现，以上为文档推演，待实现后按 V-14/V-17 浏览器验收；既有 Studio 行为有历史浏览器测试背书；线上站点未联网复核（见第 2 节）。

### R1（464 行版）11 项问题的并入核验

逐项在当前计划找到对应处置：F-01 → §5.3 清单补入 zh/quickstart、quickstart 232、themes 372-374、migration 6/155/466、docs-ssg 333/740/792-793 + 关键词兜底；F-02 → §5.4 差异门改为对 C 比较 CLI 目录；F-03 → V-1 列 official.spec.ts 五处计数；F-04 → release/crates-io.toml colors 条目与 rustify-dom 依赖（§1.2 R 行）；F-05 → 全 SHA 已改并存在（本轮逐一核过）；F-06 → V-13 T4 升级模拟闭环；F-07 → §1.2 R 行三文件「已核对无发行常量」说明；F-08/F-09 → CLI README 处置统一、AGENTS.md 已为「十包」；F-10 → 重派授权入 §5.7 与 M4 记录；F-11 → 账本落点补全。无 R1 问题在当前版本残留为缺口。

## 4. 映射与实施编排

映射完整（见上节 schedule）。现有编排成立，按原计划执行；波次表如下：

| 波次/任务 | 前置及解锁证据 | 可同时做的工作 | 写入边界/共享负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| M5 展示源码（先 Button/Menubar 贯通最薄链路，再九组矩阵铺开） | M1–M4 发行成果（V-13 记录）；§1.1 已核实的公开 API 与 gallery 现状 | M6 全部：compose.py、workflow 顺序改造、serve.sh/verify.py 扩展、Studio 返回链接 | `site/src/lib.rs`、`rustify-docs.toml`、覆盖表由 M5 单人集成；`scripts/`、`.github/`、`tests/docs-ssg/` 属 M6 单人 | 浏览器命令逐项目串行（C-4）；浏览器类命令前 python3.12 PATH 前置；Playwright 与发行检查不同时起 | M5–M6 交付齐备后：V-16 批末 → V-14/V-15/V-17 同一轮 official-root/subpath/meta |
| M7 生产上线（严格串行） | M5–M6 本地门通过；提交并冻结 S；用户推送/派发授权（逐候选） | — | workflow dispatch 与线上核验单人执行 | GitHub Actions run；线上 `https://rockie.github.io/rustify-ui/` | V-18 → V-19 → 7/7 回填 SHA 并提交回填 |

不能进一步安全并行的原因：M7 是生产部署与线上验收，依赖冻结候选与用户授权，天然串行；M5 与 M6 并行边界已由计划 §10 指定（共享文件各自单人负责，浏览器会话串行）。

## 5. 总体结论与修订顺序

**是否需要修改：无需**

**建议状态：Ready**（与计划自报状态一致）

- 理由：剩余 M5–M7 的实施前置（现状事实、技术路线、接口与算法锚点、组合与部署顺序、验证与生产门、用户授权点）在本轮全部核到行为级证据，无 P0–P2 确认问题、无须写入计划的待核实项；账本、里程碑、检查、进度记录四方一致；比例原则双向核对无过度或缺口。
- 与原状态的差异：无。计划自报 Ready，本轮维持。
- 覆盖限制（不影响就绪判断，如实声明）：①四 subagent 因平台限额中断，四视角由主 agent 顺序完成（第 3 节）；②M5 展示 UI 尚未实现，Ready 表示实施前置完备，不表示已实现或上线可用；③线上与 crates.io 状态以 M1–M4 记录为据、未联网复核，V-18 将重新核验。

### 下一步

无需修改；计划可直接续做：

```
/goal 按 docs/plan/DOM-COMPONENTS-RELEASE.md 开发
```

（续做入口为计划「恢复快照」：当前 4/7，下一步 M5–M6 同批实施。）
