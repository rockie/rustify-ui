# DOM-COMPONENTS-P1-gaps · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 · 初审 | [DOM-COMPONENTS-P1-gaps](DOM-COMPONENTS-P1-gaps.md) | `b9d5d356d00ebef4d387e0753cf14e2d8917ea9e..0abf0f397b343b803cd6a375155a4d0550ab7fe2`；另核对当前工作树，tracked diff SHA-256 `248665b70037bfa36136aeaf7e9faa6b4c2d8d9ed4033c41b6959521fedbc094` | 无；本计划尚无代码评审报告 |

审查日期：2026-10-29（Asia/Shanghai）。评审 ID：`7e8ce16a-6aaa-4ab6-a01c-3404b94fa652`。当前 HEAD 为 `dc9df3547c4c164af7cf3433bef883332659b6f6`，仅在实现提交后回填计划 SHA，八个实现/文档/测试文件与目标提交完全一致。本轮不是原 P1 代码/UI 报告或本计划 `.review.md` 的复审。产品源码与计划只读，未自动修复或提交。

## 1. 确认问题

### F-01 · [P2] 在静态 CSS 接入说明中补齐必需的 scope 标记

- **定位**：[data_tools.rs:348–350](../../examples/theme-studio/src/data_tools.rs#L348)，target `0abf0f397b343b803cd6a375155a4d0550ab7fe2` 新增行。
- **问题**：外部开发者在普通静态页面使用 Rustify scoped CSS，按新增接入说明只给目标添加 `data-rustify-theme-css="<id>"`（深色再加 `data-theme="dark"`）时，导出主题完全不会匹配。实际选择器还要求同一元素具有 `data-rustify-scope`；无 WASM 的静态消费不会由 SDK 自动补它。新资料把这一不完整步骤同时放入屏内和 `theme-setup.txt`，导致本次承诺的独立静态消费仍需用户自行补步骤。
- **证据**：[codec.rs:185–194](../../crates/rustify-dom/src/theme/codec.rs#L185)生成双属性选择器；[公开指南:324–325](../themes.md#L324)的完整例子也包含 scope。Chrome 实测按接入说明构造 `fixture-a` 的深色目标并加载真实编译 CSS，`--primary` 为空、按钮背景为 `rgba(0, 0, 0, 0)`；只增加 `data-rustify-scope="preview"` 后，分别变为 `rgb(255 180 190 / 0.8)` 和 `rgba(255, 180, 190, 0.8)`，见 [selector-result.json](../validation/dom-components-p1-gaps/code-review-r1/selector-result.json)。现有独立消费者由 [generator:234–238](../../tests/theme-export/generator/src/main.rs#L234)预先补齐 scope，并未照新资料构造目标，所以九项消费者回归通过不能反证此问题。基线没有本轮新增的 `integration_notes`，此处评审的是新资料的遗漏，不要求顺带重写旧 CSS 导出器。
- **修复与验证**：在公共接入资料中给出包含 scope、主题 ID 和可选模式的完整静态根元素用法；从生成资料构造页面并消费实际导出 CSS，断言 Light/Dark 的主题变量和颜色，不能仅检查说明里有某个字符串。
- **置信度**：高；真实 CSS 选择器、公开契约及单属性差分浏览器复现一致。影响 [WORK-F20/AC-3](../PRD-DOM-COMPONENTS.md#work-f20-无损交换主题并在独立应用中消费)、计划 §5.1/V-3。

### F-02 · [P2] 为接入说明中的主题 ID 做 HTML 属性转义

- **定位**：[data_tools.rs:365–366](../../examples/theme-studio/src/data_tools.rs#L365)，target 新增的 `document.id` 插值；对应属性模板在第 348 行。
- **问题**：主题 ID 可以合法包含双引号，但新资料把原始 ID 直接写入双引号包裹的 HTML 属性。导入 ID 为 `brand"dark` 的合法主题后，说明输出 `data-rustify-theme-css="brand"dark"`；即使消费者已经知道要补 scope，照抄时浏览器仍只得到 ID `brand`，无法匹配正确导出的 CSS。这是独立于 F-01 的属性编码错误，补 scope 本身不能解决。
- **证据**：[document.rs:450–458](../../crates/rustify-dom/src/theme/document.rs#L450)只限制空白、长度和控制字符，没有禁止引号；CSS 导出器对 ID 使用 `css_string`。本轮从最新 Studio 的实际 Import → Review → Apply → Export 路径确认 `brand"dark` 被接受，导出 CSS 正确保留转义 ID，而说明未转义。Chrome 对复制出的属性解析为 `brand`，匹配结果为 false；通过 `setAttribute` 恢复完整原 ID 后为 true，见 [quoted-id-result.json](../validation/dom-components-p1-gaps/code-review-r1/quoted-id-result.json)及[实际接入说明](../validation/dom-components-p1-gaps/code-review-r1/quoted-id-setup.txt)。此问题限于用户复制资料后的 HTML 用法，不声称 Studio 文本显示执行了 HTML。
- **修复与验证**：对 HTML 属性值进行正确转义（包括 `&`、`"`），或给出保持原 ID 的 DOM API 用法；使用含引号和字符引用的合法 ID，验证浏览器取得的属性值与 JSON ID 完全相等且能匹配导出选择器。保留原 ID，不通过收紧既有 schema 规避。
- **置信度**：高；真实导入与 HTML 解析反例均已执行。影响 WORK-F20/AC-3、计划 §5.1/V-3。

## 2. 待核实项与边界

没有阻止本轮裁定的缺证或待决产品事项。以下内容不扩大成本轮验收：历史 U-01 性能归因、VoiceOver、真实 OS IME/手机、原生 Chrome 200% zoom、子路径/GPU/完整 Docs 终验与公开发布。本轮 200% 用例沿现有测试，以缩小视口和根字号放大模拟 reflow，不冒充原生浏览器缩放。

## 3. 覆盖与验证

意图：在既有 DOM 导出路径补齐可携带接入资料、当前 API 文档和真实存储反馈断言，不改变 schema、provider、存储协议或 GPU 能力。仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`。本轮由主 agent 单人完成，无并行 reviewer。

### 范围与版本

- 计划完成记录的 M1/M2 均登记实现提交 `0abf0f397b343b803cd6a375155a4d0550ab7fe2`；已确认它位于当前分支。区间只另有 `dc9df3547c4c164af7cf3433bef883332659b6f6`，该提交仅改本计划的进度/SHA 字段，排除出代码 findings。
- 累计 diff 共 36 个文件：八个实现/文档/测试文件、计划及两份记录、25 个验证产物。完整阅读八个文本变更及所在调用链；生成 JSON 按结构/字段核对，Rust 下载去掉修订 header 后与当前完整片段逐字节相同；截图抽查桌面/窄屏/文字放大，不声称逐像素复核全部旧产物。
- 八个主要文件为 `examples/theme-studio/{src/data_tools.rs,data-tools.css}`、`docs/{themes.md,dom-components-migration.md}`、`tests/browser/{theme-export-ui.spec.ts,theme-storage.spec.ts}`、`tests/theme-export/{README.md,generator/src/main.rs}`。
- 初始工作树没有额外产品代码 diff；dirty 的台账/经验索引、PRD、原 P1 评审/截图、计划评审及其资料单列为上下文或既有记录，不归为本次代码缺陷。历史目录两张原已 dirty 的布局截图在本轮未再改变。
- index diff 为空，SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。初始含 untracked 的 2724 文件内容快照 SHA-256 为 `5bcbc6755ae11a9ba2e9b85013c81e1a3252f7ec9ff5996f885daa96cdde93aa`；报告写入前逐项比较无变化，HEAD 和原 tracked diff 指纹也未变。计划指纹为 `28a6bcaaf8f0bf55dad194742c119519e244664a7d67411bcce42ef254bba9b1`。关键输入及本轮验证摘要见 [review-evidence.json](../validation/dom-components-p1-gaps/code-review-r1/review-evidence.json)。本轮新报告和证据不纳入审查输入。

### 行为与实际执行

| 检查 | 本轮结果与范围 |
| --- | --- |
| `mise exec -- mbx test --locked -p theme-studio --bin theme-studio data_tools::` | **9 passed**；原 JSON 不变、显式 mode/context、字体/回退/非打包字体及生成失败 |
| `mise exec -- cargo fmt --all -- --check`；`git diff --check` | **退出 0** |
| `mise exec -- mbx clippy --locked -p theme-studio --all-targets -- -D warnings` | **退出 0** |
| `mise exec -- mbx xtask build-web --example theme-studio --release` | **退出 0**；本轮重新构建，build ID `4b0cf79aef8237fc`；存在 Makepad/DOM 未改路径的 unused/dead-code warnings |
| `RUSTIFY_XTASK=target/debug/xtask RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_UI_EVIDENCE_OUT="$review_tmp/studio" mise exec -- npx --no-install playwright test --project=theme-studio theme-config.spec.ts theme-export-ui.spec.ts theme-exchange.spec.ts theme-storage.spec.ts --workers=1 --output="$review_tmp/studio-results"` | **26 passed，92.53s**；真实剪贴板/下载、每格式资料、Light/Dark 四参调用、JSON/history 不变、资源/许可 HTTP、EN/ZH、1440/768/390、模拟 200%、Tab/Escape 与 console/请求/CSP；三条原存储失败场景实际执行并通过，原字节/草稿/库/重试断言保留 |
| `RUSTIFY_STUDIO_SOURCE_REVISION="$(git rev-parse HEAD)" mise exec -- sh tests/theme-export/build.sh --dom-only` | **退出 0**；显式注入同一 HEAD，重建八种 CSS 和默认 DOM，未构建 GPU 消费者 |
| `RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' RUSTIFY_THEME_EXPORT_OUT="$review_tmp/consumer" mise exec -- npx --no-install playwright test --project=theme-export theme-export-runtime.spec.ts --grep 'default DOM SDK consumes original Rust and JSON\|independent .* CSS renders' --workers=1 --output="$review_tmp/consumer-results"` | **9 passed，15.34s**；八种 CSS 和原 Rust/JSON 默认 DOM 消费、普通 appearance/显式预览、字体和无 Makepad 运行时请求 |
| 两个隔离 Chrome 复现脚本 | **F-01、F-02 反例确认**；脚本只在临时目录，不向产品仓加入测试或修复。步骤和前后结果见两条 finding 及 JSON 证据 |
| `impeccable context --target examples/theme-studio/src/data_tools.rs`；`impeccable detect examples/theme-studio/data-tools.css examples/theme-studio/src/data_tools.rs` | 加载既有产品/设计上下文；检测 **0 anti-patterns**，10 条 advisory 均位于未改的旧 CSS 色值，不列新增 finding。结合实际桌面/窄屏截图核查新增区域，未做全站 UI 评分 |

上述 `$review_tmp` 的实际值为 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-gaps-code-review-r1-8j9n1ctb`；浏览器项目串行运行，证据先写该目录，少量必要结果另存本轮 [code-review-r1](../validation/dom-components-p1-gaps/code-review-r1/)。新 Studio 说明和独立消费者资料修订均为当前 HEAD；该 HEAD 与目标提交的产品代码相同。[桌面实拍](../validation/dom-components-p1-gaps/code-review-r1/setup-1440-en.png)、[中文窄屏实拍](../validation/dom-components-p1-gaps/code-review-r1/setup-390-zh.png)已实际查看。

静态审查还核对了低频生成/失败传播、两模式字体资源去重与 glyph fallback、普通 provider 与显式 preview 的区分，以及三处存储断言没有删除数据保障。没有将资料生成接入颜色发布或 autosave 热路径；本轮不另报性能数字。未调用外部服务或查阅外部技术资料。

## 4. 计划符合性

计划版本及指纹见评审对象和覆盖记录；本轮覆盖 M1/M2，未重结算原计划其他里程碑。

| 本期要求 | 实现/验证证据 | 判定 |
| --- | --- | --- |
| G-01 / WORK-F20/AC-3；默认各格式可见源码/版本边界，资料可携带、两模式字体/许可明确，JSON 不加 metadata | `integration_notes`、公共 summary/下载；9 项 host 和 26 项 Studio 浏览器；真实下载修订与资源核对 | 主要资料已交付；独立静态用法仍有 **F-01/F-02**，不能整体关闭 |
| G-02 / WORK-N5/AC-1；当前 boundary、简例与完整导出区分、真实四参 mode/rem/motion | 两份指南、`preview_call`、`ThemeBoundary` / `with_boundary` 调用链和 9 项独立消费 | 已核实所审 API 说明与实现一致；F-01/F-02 的新资料问题单列，不冒充旧 API 问题仍在 |
| V-01 / WORK-F21/AC-2—AC-3、WORK-N6/AC-1；三条失败反馈观察区域与真实数据/重试保障 | `theme-storage.spec.ts` quota、named failure/retry、denied；本轮三个场景及全 spec 均通过 | 已核实 |
| C-1/C-2 与 V-3；DOM、strict CSP、双语/窄屏/焦点、静态资源外置、版本身份 | 同版 release/consumer 重建、35 项浏览器、截图与源码 diff | 所审范围已核实；原生缩放、辅助技术和后期矩阵保留边界 |

计划现有“2/2 完成”及作者的 G-01/G-02 关闭声明作为历史实施记录保留，本次评审不改原文；独立代码验收仍受 F-01/F-02 阻塞。

## 5. 总体结论与修复顺序

**验收判定：需要修改。**

正确性结论：**patch is incorrect**。确认 **2×P2**；无 P0/P1/P3。9 项 host、35 项浏览器及构建/静态检查通过，但新增接入资料在静态根标记及合法 ID 编码两个条件下不能指导正确消费，未完全满足 WORK-F20/AC-3。

先补全静态根元素契约，再正确编码主题 ID；可在同一处资料生成逻辑中一起修改。复验须从修后资料构造普通静态消费者，覆盖 Light/Dark、普通及含引号/字符引用的合法 ID，并保留既有下载、JSON/history 和存储回归。无需为这两个局部问题扩大到 GPU、改 schema 或公开发布。

下一步：

```text
$apply-code-review docs/plan/DOM-COMPONENTS-P1-gaps.code-review.md
```

修复后再以本计划调用 `$review-code`。本结论只覆盖上述提交与固定工作树，不代表合并、发布或完整 PRD 终验。
