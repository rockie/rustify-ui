# UI R4 作者修复与自检

本记录支撑 [M7 逐项处置](../../DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)。处置为 28 条「已修改待评审」、3 条「不采纳」；没有自行评分或独立验收结论。

## 版本与现场

- 起点：main，`dirty@5d122d6231c690a61080b91c680e9a514d2034b8`。开工已有的 PRD、评审、目标、judge、旧验证 PNG 与台账材料均保留；本轮只修改界面实现、相关测试和本轮记录。
- 当前源码与产物身份见 [current-version.json](current-version.json)。Studio release `dc2af2f348e959b3`，WASM 9,905,609 B；Catalog release `32b52b66f04701ad`，WASM 8,203,814 B。Catalog 的 build id 未变，但示例 CSS 已更新。
- 工具：Rust 1.99.0、Playwright 1.63.0、Chromium 153.0.8010.12，headless、DPR 1、reduced motion、strict CSP。未安装或升级依赖，未新增服务容器。
- 本轮主要是 DOM。共享 Button 只调整 dark classic 阴影顺序；GPU 仅把既有场景根的水平/顶部内距拆为 24/16px，未新增 GPU 能力或另一套组件。

## 实拍与实测

| 批次 | 结果与用途 |
| --- | --- |
| [before](before/capture-log.json) | 142 图，0 采集失败；与 R4 同一 build id 的改前现场 |
| [after](after/capture-log.json) | 首轮 142 图，0 采集失败；随后 U5 14 图与 EXTRA 13 图按最终构建重拍 |
| [机器检查](after/machine-check.json) | 43 个受监视页面的 pageerror、consoleerror、失败请求和 CSP 违规均为 0；页面级横向溢出均为 0 |

- UI-207/208：Opposite to root 的 Confirm 实测 `rgb(83,177,253)` / `rgb(12,17,29)`，Inherit root 为 `rgb(21,112,239)` / 白色，高度均 32px；[实测](after/machine-check-followup.json)与[最终实拍](after/U5-local-opposite-dialog-1440-light.png)。390 对话框左右各保留 16px。
- UI-212：首轮修复后 Create 已完整可见，但表面仍有 23px 溢出。实测定位为底部按钮行与等宽代码句，补上换行后复测 `scrollWidth/clientWidth = 300/300`，Create 可见宽 113.4px；[最终实测](after/machine-check-followup2.json)和[实拍](after/MC-root200-preview-clip-720-light.png)。
- UI-210：classic 4 同像素末行/上一行亮度比由 0.802 降为 0.740，接近目标约 0.72；上缘和两侧增量未改。
- 三条不采纳的实测：UI-172 展开 disclosure 后逐组列出 Light/Dark token；UI-115 的键盘焦点为 3px 可见 outline，aria-label/title 与来源按钮同为 `Clear primary · Light only`，可见模式为 Light；UI-160 搜索框/Clear 高度为 32/32.1875px，差 0.1875px。[实测](after/scope-probe.json)及对应 APPLY 帧。

## 回归与检查

| 检查 | 实际结果 | 证据 |
| --- | --- | --- |
| Studio host bins | 69 通过 | [host.log](host.log) |
| components CSR lib | 121 通过 | [components-csr.log](components-csr.log) |
| components SSR、集成、doctest | 186 通过、2 ignored | [components-ssr.log](components-ssr.log) |
| scoped Clippy，`-D warnings` | 通过 | [clippy.log](clippy.log) |
| fmt、CSS/catalog 一致性、sources verify | 通过；CSS 70,128 B / 4,475 candidates。sources 报告既有可归因 Makepad drift，本轮无 fork 改动 | [fmt](fmt.log)、[CSS](css.log)、[catalog](catalog.log)、[sources](sources.log) |
| Studio、Catalog release 构建 | 均成功；最终 Studio 另经失败项复验构建确认 | [Studio](studio-build.log)、[Catalog](catalog-build.log) |
| Catalog 浏览器 | buttons/layout 12 通过 | [catalog-tests.log](catalog-tests.log) |
| Studio 浏览器 | 73 个不同用例全部覆盖通过：首轮 64/9，之后分批复验剩余 9 项 | [首轮](studio-first.log)、[复验 7/9](studio-confirm-1.log)、[复验 1/2](studio-confirm-2.log)、[复验 1/1](studio-confirm-3.log) |

首轮 9 个失败已逐项处理：`import-version` 属性渲染、预览对话框视口布局、Portal 包装层级与直接子标题选择器、冲突区滚动位置、Opposite/Inherit 状态恢复。没有把首轮或中间失败改写成全绿；其余 64 项保留原通过结果，最终构建上复跑全部剩余项。

主要命令：

```sh
mbx test --locked -p theme-studio --bins
mbx test --locked -p rustify-components --lib
mbx test --locked -p rustify-components --no-default-features --features ssr
mbx clippy --locked -p rustify-components -p theme-studio -p component-catalog --all-targets -- -D warnings
cargo fmt --all -- --check
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
npx playwright test --project=component-catalog dom-buttons.spec.ts dom-layout.spec.ts --workers=1
npx playwright test --project=theme-studio theme-ui-review.spec.ts theme-exchange.spec.ts theme-export-ui.spec.ts theme-storage.spec.ts theme-v2-storage.spec.ts --workers=1
```

浏览器始终一个 project、一个 worker 串行；构建完成后再启动对应 release 服务。200% 残留溢出另经页面元素测量和最终构建复测，不以代码阅读代替。

## 未验边界

- 未重跑整期 V1–V10、真实性能、独立 DOM/Docs/CLI 消费者及配对；本轮不证明真实 GPU 帧预算。
- 未运行人工 VoiceOver、OS IME、Safari/Firefox、真机或真人研究。既有字体资源口径测试不在本轮五份 Studio spec 内，未宣称该项已解决。
- 未推送或发布。下一步由用户运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md` 独立复审；28 条保持「已修改待评审」，3 条「不采纳」附实测依据。
