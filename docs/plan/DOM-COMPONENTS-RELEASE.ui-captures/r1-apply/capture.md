# UI R1 修复采集清单

- 来源：[R1 报告](../../DOM-COMPONENTS-RELEASE.ui-review.md)；[目标包](../../DOM-COMPONENTS-RELEASE.ui-target.md)；[逐项处置](../../DOM-COMPONENTS-RELEASE.records/M6.md#ui-验收修复)。本清单为作者自检材料，不是独立验收结论。
- 被测源码：dirty@05e100f0，改动落点见 M6；最终提交从计划完成记录的 `UI修复` 项定位。原评审报告、r1 截图和 judge.md 均未修改。
- 目标指纹：`f6114edec02d7dfd85885c20fc197fc0fdd02ec5f248c725e1d713636b637dca`。沿用 R1 算法：目标包「## 单元」至「## 采集」前文本，rstrip 后补 LF 的 SHA-256=`e4bad2a931fa4bc868906287157126cf34f03bdc35ac8f03b24e10af15132404`；再对 `sections:<上述值>\nattachments:\n` 求 SHA-256，附件清单为空。
- 环境：本地 source CLI + 工作树 Docs/Studio，`scripts/docs-pages-compose.py` 组合，`rustify-docs preview` strict CSP，地址 `http://127.0.0.1:4199/rustify-ui/`。CLI 显示版本仍为 0.3.0，**包含本地未发行修复，不代表 registry 0.3.0 已更新**。
- Chromium / Playwright 1.63.0，headless，DPR 1，reducedMotion=reduce；1440×900 与 390×844。按页面开关切浅深色；中文路径 `/zh/`。等字体与 island ready，使用内置演示数据，无网络拦截。
- 构建 A：组合 `20fc44380fd464ce`，Studio `99f6d44abfda598d`；首次修复取证。
- 构建 B：组合 `6fb3f0b15dab8c9f`，Studio `99f6d44abfda598d`；Studio CSS 修正 compare 隐式列与手机返回链接换行，wasm 未变。
- 构建 C：组合 `7d0b0eb89070cb43`，Studio `e71aeb09f8140ea9`；新增返回链接原生导航标记及 DataTable 选中态 ring。A/B 中其它未受影响页面可复用。其后仅恢复组件索引 description 元数据，不改变截图布局；由最终三宿主测试核验搜索/AI 索引。

- 最终元数据修正后的测试产物：root `068fd5a3f353cedf`；subpath/meta `cda83a83052b6241`。与构建 C 的差异为索引 description 及其派生搜索/AI 元数据，截图布局不变。

## 截图与复用范围

保留 35 张采集（30 张可复用），失效图明确标记；同状态多个差距共用一图。PNG 与 logs 按仓库规则本地保留、不入 Git。

| 文件 | 状态 | 视口 | 主题/语言 | 构建 | UTC 时间 | 复审使用 |
| --- | --- | --- | --- | --- | --- | --- |
| [U1-home-1440-light.png](U1-home-1440-light.png) | 首页短预览 | 1440×900 | light/en | A | 2026-11-13T07:41:58.770Z | 是 |
| [U1-home-1440-dark.png](U1-home-1440-dark.png) | 首页短预览 | 1440×900 | dark/en | A | 2026-11-13T07:41:58.948Z | 是 |
| [U1-home-390-light.png](U1-home-390-light.png) | 窄屏首屏 | 390×844 | light/en | A | 2026-11-13T07:41:59.103Z | 是 |
| [U1-home-zh.png](U1-home-zh.png) | 中文入口与回退说明 | 1440×900 | light/zh | A | 2026-11-13T07:41:59.280Z | 是 |
| [U2-index.png](U2-index.png) | 具体组件分组索引 | 1440×900 | light/en | A | 2026-11-13T07:41:59.474Z | 是 |
| [U3-button.png](U3-button.png) | 参数分区与 Crimson 默认标签/焦点 | 1440×900 | light/en | A | 2026-11-13T07:43:22.119Z | 是 |
| [U3-api.png](U3-api.png) | 用途公共路径指南与 Studio | 1440×900 | light/en | A | 2026-11-13T07:43:22.165Z | 是 |
| [U4-form.png](U4-form.png) | 提交后 10 秒保持完成 | 1440×900 | light/en | A | 2026-11-13T07:43:33.063Z | 是 |
| [U4-alert-1440.png](U4-alert-1440.png) | scope 内完整标题描述与确认取消 | 1440×900 | light/en | A | 2026-11-13T07:43:33.461Z | 是 |
| [U4-alert-390.png](U4-alert-390.png) | scope 内完整标题描述与确认取消 | 390×844 | light/en | A | 2026-11-13T07:43:33.778Z | 是 |
| [U4-popover-dark.png](U4-popover-dark.png) | 深色 Popover 边界 | 1440×900 | dark/en | A | 2026-11-13T07:43:34.085Z | 是 |
| [U4-dropdown.png](U4-dropdown.png) | 按钮入口与子菜单键盘焦点 | 1440×900 | light/en | A | 2026-11-13T07:43:34.392Z | 是 |
| [U5-toolbar.png](U5-toolbar.png) | primary 选中态与纯标题 | 1440×900 | light/en | A | 2026-11-13T07:43:34.816Z | 是 |
| [U5-data-table.png](U5-data-table.png) | 选择焦点与列对齐 | 1440×900 | light/en | A | 2026-11-13T07:43:35.025Z | 否，选中焦点尚未修正，使用 -final 图 |
| [U5-command.png](U5-command.png) | scope 内 CommandPalette 与遮罩 | 1440×900 | light/en | A | 2026-11-13T07:43:35.425Z | 是 |
| [U5-splitter-390.png](U5-splitter-390.png) | 窄屏标签省略 | 390×844 | light/en | A | 2026-11-13T07:43:35.494Z | 是 |
| [U6-colors.png](U6-colors.png) | 暗色透明 P3 色阶 | 1440×900 | light/en | A | 2026-11-13T07:43:35.775Z | 是 |
| [U6-downloads.png](U6-downloads.png) | 表格与目标高亮 | 1440×900 | light/en | A | 2026-11-13T07:43:35.843Z | 是 |
| [U6-alpha-dark.png](U6-alpha-dark.png) | 黑白透明色对比底 | 1440×900 | dark/en | A | 2026-11-13T07:43:36.074Z | 是 |
| [U7-intro-light.png](U7-intro-light.png) | Studio CTA | 1440×900 | light/en | A | 2026-11-13T07:43:36.261Z | 是 |
| [U7-intro-dark.png](U7-intro-dark.png) | Studio CTA | 1440×900 | dark/en | A | 2026-11-13T07:43:36.446Z | 是 |
| [U7-app.png](U7-app.png) | 默认 DOM 与返回入口 | 1440×900 | Studio Light/en | A | 2026-11-13T07:43:36.875Z | 否，使用同名 -final 图 |
| [U7-compare.png](U7-compare.png) | 可选对比布局 | 1440×900 | Studio Light/en | A | 2026-11-13T07:43:41.410Z | 否，使用同名 -final 图 |
| [U7-inspector.png](U7-inspector.png) | Inspector 分隔与定位按钮 | 1440×900 | Studio Light/en | A | 2026-11-13T07:43:42.157Z | 否，使用同名 -final 图 |
| [U7-preview-390.png](U7-preview-390.png) | 窄屏 DOM 预览 | 390×844 | Studio Light/en | A | 2026-11-13T07:43:42.329Z | 否，使用同名 -final 图 |
| [U7-app-final.png](U7-app-final.png) | 默认 DOM 与返回入口 | 1440×900 | Studio Light/en | B | 2026-11-13T07:46:48.838Z | 是 |
| [U7-compare-final.png](U7-compare-final.png) | 可选对比布局 | 1440×900 | Studio Light/en | B | 2026-11-13T07:46:53.527Z | 是 |
| [U7-inspector-final.png](U7-inspector-final.png) | Inspector 分隔与定位按钮 | 1440×900 | Studio Light/en | B | 2026-11-13T07:46:54.300Z | 是 |
| [U7-preview-390-final.png](U7-preview-390-final.png) | 窄屏 DOM 预览 | 390×844 | Studio Light/en | B | 2026-11-13T07:46:54.488Z | 是 |
| [U3-shell-focus-light.png](U3-shell-focus-light.png) | 文档壳键盘焦点 | 1440×900 | light/en | C | 2026-11-13T08:00:32.696Z | 是 |
| [U3-button-focus-light.png](U3-button-focus-light.png) | 组件键盘焦点 | 1440×900 | light/en | C | 2026-11-13T08:00:32.742Z | 是 |
| [U3-shell-focus-dark.png](U3-shell-focus-dark.png) | 文档壳键盘焦点 | 1440×900 | dark/en | C | 2026-11-13T08:00:32.951Z | 是 |
| [U3-button-focus-dark.png](U3-button-focus-dark.png) | 组件键盘焦点 | 1440×900 | dark/en | C | 2026-11-13T08:00:32.985Z | 是 |
| [U5-data-table-final.png](U5-data-table-final.png) | 选中单元格 primary 焦点 | 1440×900 | light/en | C | 2026-11-13T08:00:33.225Z | 是 |
| [U6-download-table.png](U6-download-table.png) | 下载表列标题与行名 | 1440×900 | light/en | C | 2026-11-13T08:00:33.882Z | 是 |

## 实测与路径

- [facts.json](logs/facts.json)：Crimson 黑字/#e93d82；组件与壳浅深焦点的计算样式；首页预览 y=639.36；表单 0/1/3/6/10 秒均无 Saving；Reset 后重提 Ada；AlertDialog 两视口 clientHeight=scrollHeight=186；DataTable 两列均 180px；CommandPalette 结果 copy-json；桌面/手机返回官网。
- UI-33 复走 `/components/interaction/` → Display name=Grace Hopper → Save contact → 观察 10 秒 → Reset → 再 Save，提交可完成且结果更新。
- UI-03 `/components/extensions/` → Run a command → 搜索 JSON → ArrowDown → Enter → copy-json，面板关闭且焦点返回。
- UI-06 `/theme-studio/` 就绪 → Back to the site；390 宽切 Preview 后同样点击，均返回 `/rustify-ui/`。
- [baseline-measurements.json](logs/baseline-measurements.json)：原构建的 DataTable 等宽列和 Share→ArrowRight→Copy link 焦点反证。原图只引用 r1，不标为本轮新采集。
- [browser.json](logs/browser.json)：Studio 可选 GPU 对比出现既有 Makepad document_title 能力提示和软件 GPU ReadPixels 性能提示；Docs 自检无 pageerror/失败请求。正式整站测试按既有 audit 规则验证 console、CSP 与请求边界。
- [final-check.log](logs/final-check.log)：最终焦点/命令/返回路径定向检查；源码 `aria-allowed-role` 为零，通用代码 tab 切换与跨页记忆单独回归通过。

## 检查记录

最终命令、结果、实际失败及未完项统一见 [M6 UI 修复记录](../../DOM-COMPONENTS-RELEASE.records/M6.md#ui-验收修复)。本目录 logs 保存各宿主结果及 SDK/Docs/site/Studio 检查日志。原生 VoiceOver、真实 IME、实体 P3 显示器与原生浏览器缩放未在本轮补测；窄屏/200% layout-text emulation 由 official 项目覆盖，不冒称人工验收。
