# UI R2 作者修复与自检

本记录支撑 [M7 逐项处置](../../DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)。87 条已修改待评审，UI-97、UI-156 按既有用户裁定不改产品；没有独立评分或验收结论。

## 版本与取证边界

- 起点：`dirty@f5613b47cebcebd3a0acd47184534297bc39a22b`，main。原有改动清单及 291 个受保护文件指纹见 [baseline.json](baseline.json)。报告、目标包、评审截图及 judge 原文均保持原样。
- 当前源码、测试及静态产物 SHA-256 见 [current-version.json](current-version.json)。Studio wasm build ID 为 `1d0f7fb07ed99240`，Catalog 为 `7ea7f541a819b220`；build ID 不区分仅 CSS 的变更，静态文件指纹同时保留。
- 新增和修复走 DOM；既有 GPU 只检查状态同步、拾取、生命周期和性能。没有新增 GPU 组件、修改 runtime protocol 或主题/存储数据契约。
- [工具实测](checks/tool-versions.json)：Rust 1.99.0、Node 26.12.1、Playwright 1.63.0。`mbx --version` 当前回显 1.26.0，与 mise 的安装声明 1.15.0 不同；本轮没有执行工具安装或升级命令。浏览器实拍与机器检查用 Chromium 153.0.8010.12；headed 回归及性能用现有 Chrome 156.0.8078.48 / Apple M5 Pro。
- 构建、测试与 Clippy 使用 `CARGO_TARGET_DIR=$PWD/target MBX_TARGET_VIEWS=0`。第一次构建遇到 Makepad 旧 target 内 Rust 1.99.1 的 E0514；切换到本仓 target 后以 1.99.0 构建成功，没有清理或升级共享工具链。

## 实拍与实测

| 批次 | 实际结果 | 用途 |
| --- | --- | --- |
| [before](before/capture-log.json) | 92 图；U7 的页面 clip 首次越界，失败保留 | 改前现场 |
| [before-u7](before-u7/capture-log.json) | 26 图，0 失败 | 加 `fullPage` 后完成向外取整、四边留 8px 的页面裁切 |
| [after](after/capture-log.json) | 118 图，0 采集失败 | 首批作者自检 |
| [confirm](confirm/capture-log.json) | 118 图，0 采集失败 | 第一批确认；Catalog U6/U7 40 图的可见内容保持，沿用为当前证据 |
| [confirm-shell](confirm-shell/capture-log.json) | 78 图，0 采集失败 | 壳层修复阶段保留，后由 confirm-final 替代 Studio 帧 |
| [confirm-final](confirm-final/capture-log.json) | 78 图，另有 1 张 v2 配置变化补图 | 共享参考文案与 Slider 修改后的 Studio 当前帧 |

[最终 Inspector 补采](confirm-final-inspector/source-check.json)的 DOM/GPU 两图替换主批次的 DOM 拾取帧并增加 GPU 帧。当前冻结矩阵共 120 图：Studio 80、Catalog 40；Catalog U6/U7 的内容与可见规则未受后续 Studio 与 Slider 修改影响，复用 confirm 的 40 帧。另有 Slider 的桌面/手机浅深色 4 张补图，以及 1280/390 宽、16/20px rem 的 4 张轨道中心图。此前各批保留用于定位，每批均保留实际动作与健康记录。采集器：[capture-apply.cjs](capture-apply.cjs)；所有这些都是作者自检。

[机器检查](machine-check.json)在 strict CSP release 页面执行，脚本为 [machine-check-apply.cjs](machine-check-apply.cjs)：

- Studio 1440、768、390 编辑/预览、720/1280 根字号 200%，Catalog 1280/390 两类夹具：页面横向溢出和未被容器裁切的越界元素均为 0；导出对话框在视口内。
- console/pageerror 与 CSP 违规均为 0。采集日志同时保留请求失败监听；当前确认批次没有失败请求。
- 主路径真实 Tab 70 次、Catalog Tab 61 个落点均在屏内且有指示。导出 25 次、导入 15 次 Tab 留在模态内；Escape 后分别回到 Export、Import。
- 200% 命令条：720×900 时高约 348.6px；1280×720 时高约 237.8px。均没有限高或内部滚动条，模式、Undo/Redo、Save、Export 全在条内。普通字号下九个壳层按钮/选择器均为约 32.19px，都是 border-box。
- axe 在本次采样的 Studio 状态未报告 violation；Catalog 按钮每种视口仍有 19 个旧对比度节点，布局每种视口有 1 个 Code 节点。18 个按钮/语义色对及 Code 属用户接受的参考选阶/旧 token 对；另 1 个 Caller utilities 演示仍属原报告建议。本轮未改这些配色，不能把扫描差异当作 UI-156 已解决。axe incomplete 未逐项人工复核。
- [非法属性定向扫描](checks/invalid-diagnostic.json)：真实 width 拒绝说明、默认 `width=auto` 回退与 14px 错误文字可见；该节点 AA 扫描 violation/incomplete 均为 0。Restore 恢复合法 240px 并清除说明。
- [自然加载与配置实测](runtime-details.json)：Studio 不强制 load 时已加载 Plex 400/500/600/700，实际请求外部 WOFF2；Catalog 字重矩阵自然加载 300/400/500/700。专项用例显式加载五档全部成功。768px 下主题名和产品标题均为 16px。

## 回归与静态检查

| 检查 | 实际结果 | 原始证据 |
| --- | --- | --- |
| 相关 host：dom/components/Studio/Catalog，`--lib --bins` | 453 通过 | [host-final.log](checks/host-final.log) |
| components SSR、集成及 doctest | 186 通过，2 ignored | [ssr-final.log](checks/ssr-final.log) |
| 相关包与 xtask host Clippy、components SSR Clippy，`-D warnings` | 通过 | [host](checks/clippy-final.log)、[SSR](checks/ssr-clippy.log) |
| Studio wasm Clippy，`--no-deps -D warnings` | 1 条既有 `suspicious_assignment_formatting`，原 HEAD 也有相同宏内 `*value=!*value`；未顺手改该行 | [默认命令](checks/studio-wasm-clippy.log) |
| Studio wasm Clippy，仅放行上述已定位旧规则，其余 `-D warnings` | 通过；依赖库另有 2 条既有 rustc 告警 | [最终限定例外](checks/studio-wasm-clippy-reference.log) |
| fmt、CSS 生成一致性、catalog 文档一致性、sources verify | 通过 | [fmt](checks/fmt-final.log)、[CSS](checks/css-check.log)、[catalog](checks/catalog-check.log)、[sources](checks/sources.log) |
| Catalog 布局与按钮专项 | 13 个不同用例最终均通过（含新增 Slider 像素回归） | [首批](checks/catalog-browser.log)、[ThemePanel 定向复验](checks/catalog-theme-panel.log)、[Slider](checks/slider-after.log) |
| Studio 配置、DOM、编辑、交换/导出、存储、迁移与 UI | 93 个不同用例最终均通过；分批运行，按改动和失败范围复验 | [覆盖索引](checks/browser-coverage.json)、[36 项收尾](checks/studio-remaining-browser.log)、[壳层 14 项](checks/studio-shell-browser.log)、[最终 Inspector/UI 批次](checks/studio-inspector-browser.log)、[坐标定位复验](checks/studio-inspector-viewport.log)、[Slider 轨道中心](checks/studio-slider-center.log) |
| release 构建 | Studio、Catalog 均成功 | [Studio 最终](checks/build-studio-slider.log)、[Catalog 最终](checks/build-catalog-slider.log) |

主要命令，均从仓根运行：

```sh
export CARGO_TARGET_DIR="$PWD/target" MBX_TARGET_VIEWS=0
mbx test --locked -p rustify-dom -p rustify-components -p theme-studio -p component-catalog --lib --bins
mbx test --locked -p rustify-components --no-default-features --features ssr
mbx clippy --locked -p rustify-dom -p rustify-components -p theme-studio -p component-catalog -p xtask --all-targets -- -D warnings
mbx clippy --locked -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings
RUSTFLAGS=--cfg=web_sys_unstable_apis mbx clippy --locked -p theme-studio --target wasm32-unknown-unknown --no-deps -- -D warnings -A clippy::suspicious_assignment_formatting
cargo fmt --all -- --check
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example component-catalog --release
```

浏览器始终单进程、单 project、单 worker；`RUSTIFY_XTASK` 指向已构建的本仓 xtask。首批 Studio 使用报告相关的八个 spec，后续仅复验失败、未运行及新改动范围；实际 argv 与排除清单见 [remaining-browser-result.json](checks/remaining-browser-result.json)。最后的 Inspector/UI/性能批次为 `--project=theme-studio --workers=1 --headed theme-ui-review.spec.ts theme-editor.spec.ts theme-performance.spec.ts -g 'theme-ui-review|Inspector suppresses|page timestamps'`，13 项通过、1 项长 canvas 点击定位超时；将色样滚到可见处并验证实际命中后，定向 Inspector 用例通过，没有强制点击或放宽结果断言。此后共享参考状态文案的最终批次 [16 项全部通过](checks/studio-reference-browser.log)，包含命名保存、冻结升级评审、Inspector、12 项 UI 和页内性能；Slider 样式调整后 [两项 HSL](checks/studio-slider.log)也通过。截图输出用 `RUSTIFY_UI_EVIDENCE_OUT` 指向本轮目录，未覆盖旧验证 PNG。

## 自检中修正的失败

- 字体缺失：computed 700 不能证明实际有 700 字面；补齐五档静态 WOFF2（合计 326188 bytes），在 public 分发，原许可证和固定来源 SHA 一并保存，不嵌入 wasm。
- 控制页初次点击：focus 插入来源块使字体下拉触发器下移约 269.69px，pointerup 错过目标。来源移到表单后，位置保持不变；[前](checks/control-focus-before.json)、[后](checks/control-focus-after.json)与十四字段回归均支持这一点。
- 选中行 Portal：刚聚焦的输入被重新挂载卸载。改为稳定的 Portal 挂载点，在行间移动并恢复焦点；[前](checks/portal-dialog-before.json)、[后](checks/portal-dialog-after.json)保留节点身份和事件证据，GPU/DOM 拾取测试也检查同一输入节点。
- 对话框：Layer 是 JS 定位的 fixed 元素，给它 align-items 无效。仅工具对话框固定顶部 1rem；实测选择器切换前/后 y 均为 78px，未改 SDK 的默认居中行为。
- 扩大字号：1280×720 根字号 200% 下，主题区被 flex 压窄后头部增至约 723px；限制分组收缩后约 238px，真实存储重试恢复可点击。[红测](checks/wide-header-before.log)及后续回归保留。
- 状态合并：恢复了迁移失败的 Technical details、受保护缓存的状态节点，以及升级后重开页面的 SHA 绑定原始备份入口。真实备份写入/回读/v2 写入失败、碰撞、重新打开、损坏备份和失败重试均复验通过。
- Ghost：既有 SDK 使用 content-box，壳层仅设置 min-height 会额外增加约 13.81px。限定在壳层与数据工具对话框采用 border-box；[红测](checks/shell-height-before.log)与最终高度用例通过。
- Inspector 来源标题：只有来源块部分入屏仍不足。新增标题完整可见断言后实测 ratio 为 0；将最终滚动放在聚焦与挂载点移动之后，DOM/GPU 标题均在约 y=89，输入继续聚焦。后续 headed Chrome 用例暴露长 canvas 的局部坐标位于屏外；改为滚动真实色样到视口并确认 elementFromPoint 命中 canvas，再以鼠标点击；保留 token、输入节点、region 与核心状态不变断言。
- 参考状态：`is_modified()` 比较实际 checkpoint 文档；命名保存 Ocean 后用 Matches “Ocean”，编辑后用 Modified from “Ocean”。工具条与升级评审共用函数，评审只比较冻结 snapshot，刷新前不跟随当前编辑变化；保存 ack 仍独立显示。命名保存与过期评审回归通过，见 [最终 16 项](checks/studio-reference-browser.log)。
- Slider：用户指出左侧未对齐；实拍确认填充阴影穿过侧栏且原生 margin 为 2px。共享样式增加 `overflow: hidden` 与 `margin: 0`；新增像素检查在旧样式先失败（左侧 24 个填充色像素），修后浅深色 Home/ArrowRight/End、两端值与焦点环通过。既有 1280/390px、16/20px rem 的填充/空轨道中心与拖动键盘两项也通过，四张中心图保存于 [slider-center](slider-center/)；另有桌面/手机浅深色四张补图；[改前](slider-before.png)、[修后](slider-confirm/slider-chromium-1440-light.png)、[红测](checks/slider-before.log)、[绿测](checks/slider-after.log)、[几何](slider-confirm/geometry.json)。Firefox 补充启动失败（Could not find profile folder），保留 [日志](checks/slider-firefox-launch.log)，未安装或重配浏览器，未声称 Firefox 已验证。
- 旧断言：ThemePanel 分隔点、Auto 色名大小写/中文、配置当前→导入文案、无锁说明和既有 Inspector 输入聚焦行为同步到当前 UI。没有放宽编辑结果、历史、作用域、存储字节、锁、取消、GPU region 或 no-op 解析计数断言。

## 同机性能参考

沿用真实 headed Chrome、1440×900、各 40 次页内测量，不使用 Playwright 操作耗时。数据与环境见 [改前摘要](checks/performance-before.json)、[最终完整数据](checks/performance-reference-final.json)、[比较](checks/performance-comparison.json)。

| 页内指标（ms） | 改前 p50 / p95 | 最终 p50 / p95 |
| --- | --- | --- |
| DOM CSS commit | 24.0 / 25.6 | 23.1 / 25.0 |
| DOM color settled | 66.9 / 74.2 | 66.7 / 67.8 |
| GPU draw ack | 25.2 / 26.7 | 24.2 / 26.0 |

两次均 editor/SDK resolves 80/160，240 inputs → 30 publications，最终 revision/drawn 70/70，0 long task；5 秒 idle 的 frames/pumps/tasks/memory 无增量。wasm 9735795 → 9802741 bytes（+66946），字体另行分发。

这是单个改前运行与最终运行的参考，不证明归因或多轮性能无回退。保留的中间修后运行 p95 为 25.4/26.4ms 与 22.8/23.7ms（DOM/GPU），可见环境波动；另两次中间运行是 24.6/25.7ms 与 24.5/25.5ms（精确值保留在各原始 JSON）；本表采用最后一次实测，没有挑选较快的一次。最终 Slider 仅改 CSS，wasm 和 build ID 与该性能运行一致；未重复不受影响的页内性能测量。首次性能命令误用了 configuration 用例的输出变量，写到了旧 M6 文件；该历史文件已恢复原字节，改前摘要从原始运行日志恢复，改前 40 条 raw 未保留。最终输出使用正确的 `RUSTIFY_THEME_PERF_OUT`，包含 raw；中间数据和失败日志均保留。

## 保留边界与交接

- 未重跑整期 V1–V10、独立 Docs/CLI 作者环境、全部跨实例故障或性能套件；人工 VoiceOver、OS IME、Firefox/Safari、真机和真人研究仍未验。
- 5 条既有作者预览字体的设计检测误报仅在 `web/theme-fonts.css` 按值限定忽略；未全局关闭规则。DESIGN.md 的字体事实同步，设计 sidecar 较旧的刷新提示保留，没有在修复 UI 时重写整份设计资料。
- SDLC 与经验在原有未提交资料中续写；不把此前未跟踪的评审、目标包和整套台账纳入本轮提交。没有推送、部署或发布。
- 下一步由用户运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`，按已确认目标包重建基线。87 条仍为已修改待评审，UI-97 按新采集复核，UI-156 交独立复审按用户口径处理。
