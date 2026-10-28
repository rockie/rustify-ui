# R5 作者修复 · 自检与检查

本轮沿用人工 Markdown 循环，未建立或导入 managed 状态。验证基线为 `dirty@941a8043903c244dc2b72e1e90c38b49e193c20c`；源码、输入与 CSS 指纹见 [current-version.json](current-version.json)。报告、目标包和评审 PNG / judge 原文保持原位、原字节。

## 环境与真实入口

- mise 固定 Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13；Node 26.12.1，Playwright 1.63.0，Chromium 153.0.8010.12。无安装、升级、外部发布或推送。
- `mbx xtask build-web --example theme-studio --release` 与 `--example component-catalog --release` 重建，`mbx xtask serve` 以 strict CSP 分别运行在 4181 / 4176。最终检查后，临时服务已停止。
- 正常 Studio 页面；catalog 使用既定 `mount_button_fixtures()` / `mount_layout_fixtures()` 沙箱入口。新 context、空存储、DPR1、reducedMotion reduce；配额用例只注入 Storage 异常，无锁用例在载入前移除 navigator.locks。
- Studio build_id `eeec4e5d8bf09cf7`；catalog `32b52b66f04701ad`。build_id 只散列 wasm（`xtask/src/build.rs::build_id`），catalog 本轮只改外部 CSS，故 ID 保持原值；实际 CSS sha256 与页内计算样式一并证明当前样式已加载。

## 检查

下列命令通过 `mise exec --` 执行。所有编译、测试和 Clippy 均使用 mbx。浏览器一次一个 project，逐次串行；截图测试设置 `RUSTIFY_UI_EVIDENCE_OUT=docs/plan/DOM-COMPONENTS-P1.ui-captures/r5-apply/regression`，未覆盖旧验证 PNG。

| 命令 / 范围 | 实际结果 | 日志 |
| --- | --- | --- |
| `cargo fmt --all -- --check`；`git diff --check` | 通过 | [fmt](logs/fmt-final.log) |
| `mbx test --locked -p theme-studio --bins` | 69 通过 | [Studio](logs/test-studio.log) |
| `mbx test --locked -p rustify-components --lib` | 121 通过 | [CSR](logs/test-components.log) |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | 169 通过；与 CSR 分开运行 | [SSR](logs/test-components-ssr.log) |
| `mbx clippy --locked -p theme-studio -p rustify-components --all-targets -- -D warnings` | 通过 | [Clippy](logs/clippy.log) |
| `mbx xtask css`；`mbx xtask css --check` | 生成并确认无 drift | [生成](logs/css-generate-final.log)、[检查](logs/css-check-final.log) |
| `mbx xtask sources verify` | 通过 | [来源](logs/sources-check.log) |
| `mbx xtask catalog --write docs/components.md --check` | 无 drift | [目录](logs/catalog-check.log) |
| `mbx xtask build-web --example theme-studio --release`；`--example component-catalog --release` | 最终两项构建通过；构建有来自本轮未修改的 Makepad / DOM 文件的 unused/dead_code 警告 | [Studio](logs/build-studio-final.log)、[catalog](logs/build-catalog-final.log) |
| `npx playwright test --project=theme-studio theme-ui-review.spec.ts theme-export-ui.spec.ts --workers=1` | 21/21；覆盖四选择器的纯键盘焦点、Tab 限制 / Esc 回焦、局部深色对话框、导入导出、拾取、移动端命令条 | [Studio 浏览器](logs/browser-studio.log) |
| `npx playwright test --project=theme-studio theme-v2-storage.spec.ts -g 'a quota failure\|without Web Locks' --workers=1` | 2/2；配额保留并重试、无锁下不谎报保存 | [存储](logs/browser-storage.log) |
| `npx playwright test --project=component-catalog dom-buttons.spec.ts dom-layout.spec.ts --workers=1` | 首轮 11/13；两项失败及复验见下，总计 13 个不同用例均通过 | [首轮](logs/browser-catalog.log)、[两项复验](logs/browser-catalog-followup.log) |

合计 **359 项 host/SSR、36 个不同浏览器用例通过**。不是整期 V1–V10 重跑。

### 失败与修正

文本日志仅去除行末空白；原始 PNG 与测量结果不改写。

- 新增焦点 / Kbd 回归用例先在旧 release 上运行，分别复现 outline none、圆角 / 字号比仅 0.0833：[焦点 red](logs/focus-red.log)、[Kbd red](logs/kbd-red.log)。这些预期失败不计入通过数。
- Catalog 首轮的旧断言固定要求阴影 alpha 0.3；按本轮目标更新为最终 0.32 并等待稳定值。新增 Kbd 用例立即读取了尚未投影的 100% 样式；补充轮询等待缩放后再核对全矩阵。没有为这两项改变产品行为。复验命令在上述 catalog 命令后加 `-g 'component colors, semantic overrides|Kbd rounding follows'`，2/2 通过。
- 改前采集器最初把 ready 节点当作可见元素等待、在窄屏照搬桌面 Tab 次数，并且未限定重复 testid 的作用域；均已按实际 DOM 修正。成功完成改前 29 张原图；中间采集错误保留在 [before.log](logs/before.log)，不记为产品错误或通过。

## 实拍与页面实测

[改前](before/)、[首批自检](after/)、[最终确认](confirm/) 均为本轮真实页面。首批自检 31 张、最终确认 6 张，另有回归用例 5 张。首批截图等待字体、networkidle 和 500ms 稳定帧窗口（8s 上限）；焦点停留 700ms 后拍摄。确认帧等待字体、networkidle 与额外 700ms。原始 PNG 未加工，不覆盖 R5 评审材料。

| 差距 / 风险 | 本轮观察与修复后实测 | 证据 |
| --- | --- | --- |
| UI-215 | 改前四触发器 :focus-visible=true、outline none、shadow none；1440 / 390 改后均 3px solid rgba(21,112,239,0.4)，offset 2px。规范键盘路径与回焦由回归用例验证 | [测量](after/measurements.json)、[桌面焦点](after/U3-export-kind-focus-1440.png)、[窄屏 CSS profile](after/U3-export-profile-focus-390.png) |
| UI-214 | size 9 圆角从 4px 改为 16.8px（classic 外轮廓约 18px），size 8 为 9.8px；110% / 95% 的 size 9 为 18.48 / 15.96px，未重复乘缩放。1–9、classic / soft、1280 / 390 均采集 | [矩阵](after/U7-kbd-1280.png)、[测量](after/measurements.json) |
| UI-213 | 表头吸顶位置从 y41 改为 y17，对话框上缘 y16；初始 24px 内边距保留，滚动后表头上方不再露出旧行 | [改前](before/U3-import-review-bottom-1440.png)、[改后](after/U3-import-review-bottom-1440.png) |
| UI-203 | 整体 callout 只说明草稿持久化事实；具体命名失败留在命名卡。无锁正文直接说明内存编辑 / 导出。中英文配额、无锁均实拍 | [配额](after/U4-quota-1440.png)、[中文无锁](after/U4-no-locks-zh-1440.png) |
| UI-76 | R4 `18058741` 新增的 36rem / 24rem 容器规则同时缩小外层 padding，改前 DOM 首行相对框为 x13/y13（390 为 x9/y9）。保留内部卡片紧凑 padding，只恢复首行起点；1440 / 768 / 390 均为 x25/y17，截图中首行基线与 GPU 一致 | [比较](after/U5-gpu-compare-1440.png)、[实测](after/measurements.json) |
| UI-210 | 深色 classic 仅底部 inset alpha 从 0.3 改为最终 0.32；四档末行 / 前一行的明度比 0.7400 → 0.7217，末行 #3951A0、前行 #4F70E0。首批 0.33 得 0.7141，最终小幅校正；顶部 / 侧边采样不变。明度指标不是 WCAG 对比度 | [最终实拍](confirm/U6-buttons-dark-1280.png)、[像素](classic-pixels.json)、[计算样式](confirm/measurements.json) |
| 200% 防回归 | provider 预览面板 client / scroll = 300 / 300；Create · 0 的 x386–499.4、y800.7–864.7，完整可见 | [实拍](confirm/U1-root200-preview-720.png)、[实测](confirm/measurements.json) |
| 其他单元 | U1 命令条、U2 窄屏来源与现有重置图标、U5 局部主题与浮层、U6 浅深色 / 交互、U7 语义 / 缩放随本轮回归检查 | [U2](confirm/U2-source-390.png)、[回归截图](regression/) |

首批 10 个与最终 4 个受监视 context 的 pageerror、console error、失败请求、HTTP ≥400 和 CSP 违规均为 0。首批与最终的唯一产品差异是 classic 暗线 0.33 → 0.32；最终暗线使用 confirm 取证，其余证据未因无关样式变化重拍。数据见 [首批](after/measurements.json)与[最终](confirm/measurements.json)。

## 未运行边界

未重跑整期 V1–V10、真实性能、独立 DOM/Docs/CLI 消费者或跨实例故障；人工 VoiceOver、OS IME、Safari / Firefox、真机与真人研究未验。原证据保持各自基线。UI-115 沿用用户原话「保留现在这样就可以」，未改产品或目标。没有重构轮，没有自评分；6 条均为「已修改待评审」，由下一轮独立 review-ui 决定是否关闭。
