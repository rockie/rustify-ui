# R1 界面修复自检

关联：[逐项处置](../../DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)。本页记录作者自检，不评分、不替代独立 UI 复审。

被测工作树为 `dirty@13ab1a5b5b0d99a08d0b482db452273f713e10ba`。最终产品源码逐文件 SHA-256 见 [current-version.json](current-version.json)，对应修复提交由计划 M7 的本轮 `UI修复` 列指向。原报告、目标包和 R1 证据的 129 个文件由 [baseline.json](baseline.json) 保护。原有未提交资料不属于本轮提交。

文本日志为本轮运行输出的副本，仅移除行尾空格；测试内容、结果与 JSON 数据未改动。原评审日志保持原字节。

## 构建与基础检查

Rust 使用 mise 固定的 1.99.0；所有编译、测试和 Clippy 经 mbx，`CARGO_TARGET_DIR=<repo>/target MBX_TARGET_VIEWS=0`。两示例均启用既有 GPU feature；本轮以 DOM 修复为主，GPU 只改既有说明文字和位置。

| 检查范围 | 实际结果 | 日志 |
| --- | --- | --- |
| Host：DOM、组件 CSR、Studio、catalog | 258 / 121 / 69 / 5 全部通过 | [host](checks/host.log) |
| 组件 SSR、SSR controls、theme layout、doctest | 169 / 13 / 2 / 2 通过，另 2 ignored | [ssr](checks/ssr.log) |
| Host / SSR Clippy | 通过 | [host](checks/clippy.log)、[SSR](checks/ssr-clippy.log) |
| 生命周期修改及来源输入缓存后的 Studio host / Clippy | 各 69 通过、Clippy 通过；最后两行 wasm 专用色值缓存由 release 和浏览器覆盖 | [生命周期 host](checks/lifecycle-host.log)、[Clippy](checks/lifecycle-clippy.log)、[来源 host](checks/cache-host.log)、[Clippy](checks/cache-clippy.log) |
| `cargo fmt --all -- --check` | 通过 | 无格式差异 |
| `mbx xtask css` 与 `mbx xtask css --check` | 已生成并一致 | [生成](checks/css.log)、[检查](checks/css-check.log) |
| `mbx xtask catalog --write docs/components.md --check` | 无 drift；59 public entries / 97 reference names / 24 legacy categories | [catalog](checks/catalog.log) |
| `mbx xtask sources verify` | 通过 | [sources](checks/sources.log) |
| `mbx xtask build-web --example theme-studio --release` | 最终 build `2be2f32fbf78c5f3` | [最终 build](checks/color-cache-build.log)；[40319fe 历史 build](checks/build-studio.log) |
| `mbx xtask build-web --example component-catalog --release` | 最终 build `07b789e3e401b12a` | [build](checks/build-catalog.log) |

构建保留现有 Makepad/DOM 未使用项告警，没有改无关源码来消除它们。Host 检查不覆盖 wasm32 条件分支，以下真实浏览器结果补充其边界。

## 浏览器回归

使用已经存在的 release 构建与 strict CSP server；每次仅运行一个 Playwright project、一个 worker，没有并行 Playwright 进程。功能回归与截图使用已安装 headless Chromium 153.0.8010.12，视口与主题随对应测试/截图记录。

| 批次与版本 | 实际结果与复用范围 | 日志 |
| --- | --- | --- |
| 修前原构建 `f7f81f184ae45ccd`，新增 4 个 UI 回归 | 4/4 失败：未知部件误触、轮廓取父级、缺局部相反外观入口、非法数字草稿丢失 | [red](regression-before.log) |
| 修后 Studio `484d5dfe8ff2a79f`，原选定 70 项回归 | 首次 19 通过、3 个旧 UI 断言失败、48 未跑；调整新标签及当前可见来源的定位后续跑 | [批次一](checks/studio-browser.log) |
| 同一修后构建，剔除已过的 theme-dom 六项 | 45 通过、3 个原文显示断言失败、16 未跑。显示区改为格式化 JSON，断言改为解析后比较；存储原始字节与实际下载的精确比较保留 | [批次二](checks/studio-browser-final.log) |
| 同一修后构建，存储后续 19 项 | 17 通过；发现延迟帧销毁崩溃和失败清除后保存入口被禁用，继续修复 | [存储](checks/storage-browser-final.log) |
| 旧构建稳定复现新增布局回调问题 | 暂停真实 rAF、编辑、dispose，再刷新未取消回调；出现 disposed signal / unreachable，测试失败 | [red](checks/owner-red.log)、[实际错误](checks/owner-dispose-before.json) |
| 生命周期修正 Studio `40319fe42bb1118f` 定向 11 项 | **11/11 通过**：新增 UI 五项及 owner disposal、host fatal、参数/HSL 迟到 blur、失败清除后恢复保存、真实 lock grant 后销毁 | [最终](checks/lifecycle-browser.log) |
| 来源输入缓存 Studio `6b28136b5912eced` | 12/12 通过：UI 六项与配置六项；新增同值输入计数用例由失败转为通过 | [回归](checks/cache-browser.log)、[red](checks/noop-red.log)、[稳定反证](checks/noop-before.json) |
| 最终 Studio `2be2f32fbf78c5f3` | **17/17 通过**：UI 六项、配置六项，以及作者 alpha/格式搜索/捕获拖拽/HSL 取消与无效草稿五项 | [最终回归](checks/color-cache-browser.log) |
| 最终 catalog `07b789e3e401b12a` | **12/12 通过**：dom-buttons 五项、dom-layout 七项 | [最终](checks/catalog-browser.log) |

所选回归共 77 个不同用例的成功覆盖分布在上述批次（[逐批覆盖](checks/coverage.json)），不能合写为“最终构建一次全量 71/71”。先在 `40319fe42bb1118f` 重跑生命周期和失败清除恢复 11 项；最后补齐来源说明的相等性输入缓存、颜色编辑器共享色值/覆盖映射的单次缓存，再在最终构建重跑相关 UI/配置/颜色手势 17 项。共享组件、SSR、目录以及未改动的存储模型沿用本轮已通过且覆盖未变的结果。

测试更新仅适配真实交互变化：Titlecase 色族、清楚的导出格式名称、结构化差异与当前活动分区；陈旧升级审阅测试通过已有公开模型入口制造审阅后的草稿变化，并保留 context/history/storage 原有断言。新 UI 用例见 `tests/browser/theme-ui-review.spec.ts`。截图输出通过 `RUSTIFY_UI_EVIDENCE_OUT` 指向本目录的 `checks/`，没有覆写旧验证图片。

## 最终确认与机器检查

[最终确认](confirm/capture-log.json)在 Studio `40319fe42bb1118f`、catalog `07b789e3e401b12a` 上采集 **109 张**，覆盖 U1–U7 与新增矩阵/局部相反外观；34 个页面 context 的 page error、console error、失败请求、CSP 事件全部为 0。最后的缓存修正只改变来源说明的重复解析与颜色编辑器的共同映射读取，不改 DOM、样式或视觉，沿用这批图；其新构建回归和性能在下段另列。

[运行实测](confirm/measurements.json)：Brand color 触发器与列表宽度都为 279 px；DOM/GPU 容器 top 都为 263.890625 px；宿主探针 position=static；深色作者容器背景为 `rgb(12,17,29)`、padding=24 px；默认与显式 Indigo soft Code 都使用 `rgba(0,43,183,0.773)`；Heading/Strong 700，Blockquote 在 14/18/28 px 字号下 border=4/4/7 px。

[机器检查](confirm/machine-check.json)：Studio 1440/768/390、720 下根字号 200%，catalog 1280/390 的页面横向溢出均为 0；导出对话框在手机上保留两侧 16 px。Studio 70 次 Tab、目录 61 次 Tab 的实际焦点均有指示且位于视口内；导入/导出焦点走查留在对话框内，Escape 后分别回到 Import/Export；配置 Select 的原生方向键与 Enter 可由 Custom 改为 Gold。此几何检查不等于所有滚动场景均完美，也不代替真人可用性研究。

axe 没有再次报告壳层 UI-98 的文字对比或 UI-99 的 target-size。保留的 `color-contrast`：桌面 Studio 作者 Code 1 节点（3.89:1，尊重作者 primary）、目录按钮 19 节点（包含参考色阶、旧语义与调用方样式演示）、目录 gray solid Code 1 节点（白字/gray9，3.31:1）。UI-100 按用户决定保留比较口径，未擅自替换参考/作者颜色。`incomplete` 的 ARIA/颜色结果保留原数据，未算通过，未声明全页 WCAG AA。

## 同机性能与体积

执行本仓现有性能用例，参数与修前相同；测量期间没有并行编译或其它浏览器测试：

```sh
RUSTIFY_XTASK=target/debug/xtask RUSTIFY_THEME_PERF_OUT=docs/plan/DOM-COMPONENTS-P1.ui-captures/r1-apply/performance-after.json RUSTIFY_TIER=evidence RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' npx playwright test --project=theme-studio theme-performance.spec.ts --headed --workers=1 -g 'page timestamps measure'
```

同机 Apple M5 Pro / ANGLE Metal、安装的 headed Chrome **156.0.8078.48**、1440×900、DPR=1、根字号16 px、40 次颜色编辑。配置覆盖的 user-agent 字符串为 Chrome153/Windows，实际 browser.version 与 executable_path 均原样记在 JSON，不用 UA 代替实际浏览器身份。

| 页面内指标 | 修前 | 最终 | 单位 |
| --- | --- | --- | --- |
| DOM 提交 p50 | 21.9 | 16.5 | ms |
| DOM 提交 p95 | 22.6 | 27.0 | ms |
| GPU 确认 p50 | 23.0 | 17.6 | ms |
| GPU 确认 p95 | 23.8 | 28.6 | ms |
| 颜色稳定 p50 | 66.7 | 66.6 | ms |
| 颜色稳定 p95 | 67.7 | 67.2 | ms |

[修前原数据](performance-before.json)，[最终原数据](performance-after.json)，[最终通过日志](checks/perf-after.log)。DOM/GPU p95 分别增加 4.4/4.8 ms；p50 更低，不能据此宣称全面提速。

原性能测试首先以“同值编辑后 SDK 多解析一次”失败（[日志](checks/perf-after-red.log)）。[无输入稳定后再重复同值](checks/noop-before.json)仍复现，模型解析/发布/历史不变，根因是新来源说明依赖未经相等性过滤的响应式读；已修正并保留原断言。其后中间构建的 DOM/GPU p95 为30.2/31.6 ms（[原数据](performance-before-color-cache.json)）；检查新增色值与覆盖标记的反复映射克隆后，以两行 Memo 在 ColorEditor 内缓存共同输入，最终为27.0/28.6 ms。这里只报告这几次实际观测，不把单次40样本当作严格统计归因。

修前与最终均：40 次编辑的 editor/SDK 解析=80/160，240 个连续输入合并为30次发布，gesture 最终 revision 等于 drawn revision，region/canvas 不重建；长任务0。最终5秒空闲期间 pump/frame、timer/task/animation frame 均无增长，只有一个 region，errors=0。连续输入时长与空闲 CPU 的原值保留在 JSON，未作为额外 SLO。

| Release WASM | 修前 B | 最终 B | 差值 |
| --- | --- | --- | --- |
| theme-studio | 9,307,019 | 9,735,795 | +428,776 B（+4.61%） |
| component-catalog | 8,080,867 | 8,172,292 | +91,425 B（+1.13%） |

修前字节取自[原 R1 采集记录](../r1/capture.md)和修前性能 JSON；最终取自 [Studio manifest](checks/theme-studio-build-manifest.json)、[catalog manifest](checks/component-catalog-build-manifest.json)。字体仍是独立静态资源，两个最终 manifest 的字体合计均为51,732,356 B，没有嵌入 WASM。

## 保留边界

本轮没有重跑整期 V1–V10、独立 Docs/CLI 作者环境、全部跨实例故障和性能套件；人工 VoiceOver、OS IME、Firefox/Safari 与手机真机仍未验证。既有证据保留各自版本。本轮不改变 Ready 和 7/7 的实施状态，不声明独立复审或公开发布完成。

设计 hook 对 DESIGN.md 已声明的 IBM Plex Sans 产生部分名称误报，仅增加 `design-system-font=plex sans` 的 value ignore，理由记录于 `.impeccable/config.json`；没有整条规则豁免。剩余参考/作者配色差异由最终机器检查段列出，不据此宣称全页面 WCAG AA。
