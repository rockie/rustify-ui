# R6 作者修复自检

- 起始版本：`dirty@d7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03`，main。呈现路径为 DOM；不改 GPU 或 runtime。
- 沿用 Markdown 人工循环：`DOM-COMPONENTS-P1.ui-review.json` 不存在；`skills-lock.json` 登记 apply-ui-review / review-ui / phoenix-ui 的 Git 来源与 hash，没有本循环 managed 安装回执。没有 import 或重置历史。
- 目标：[R6 冻结目标](../../DOM-COMPONENTS-P1.ui-target.r6.md)，规范指纹 `6c8b16d50cf7`。只处理报告的 UI-217、UI-216。
- 浏览器：Playwright 1.63.0 / Chromium 153.0.8010.12，headless，en-US，DPR 1，reducedMotion reduce；空存储的新 context。等待 ready 属性、字体、networkidle、连续 500ms 无长帧。正常 Studio 入口；Catalog 使用既定 `mount_layout_fixtures()` 入口。

命令日志只清理行尾空白，保留成功、失败及中断内容。

## 修前复现

[原始测量](before/measurements.json)与 `before/*.png`：768×1024 light、390×844 light、1440×900 dark，Graphite → Pink → Auto 后真实 Enter 展开、ArrowDown；活动行在列表视窗之外，scrollTop=0。1280/390 Code ghost 前两列分别为 `rgba(0,43,183,0.773)` 与 `rgb(31,45,92)`，与目标的中性前景处理不同。

基线脚本的 Enter 后值采样未等响应式刷新，`confirmed` 只反映即时读值，不作选择结果证据；可见性由静止后实拍及活动行几何支持。最终脚本在测量与读回前等待静止。首次脚本曾错误等待隐藏的 ready 状态为可见而超时，改为等待属性节点 attached 后成功；未据失败页判断产品问题。

| 命令 | 结果 |
| --- | --- |
| `npx playwright test --project=theme-studio theme-ui-review.spec.ts -g 'brand activity stays visible at 768' --workers=1 --output=test-results/r6-red-studio` | 1 failed，活动行完整可见断言 expected true / received false，见 [red-studio.log](red-studio.log) |
| `npx playwright test --project=component-catalog dom-layout.spec.ts -g 'Code ghost keeps neutral foreground.*1280' --workers=1 --output=test-results/r6-red-catalog` | 1 failed，期望有效 foreground `rgb(29,41,57)`，实际为强调色，见 [red-catalog.log](red-catalog.log) |

## 实现与构建

Select 在打开及 active 变化后按实际行与 list client rect 做最小纵向滚动，只滚动 list；选中值与活动项仍独立。Code ghost 的非 gray 分支消费有效 foreground，gray 保留 a11 / high-contrast 12 阶；solid/soft/outline 不改。

首次 Studio 构建在新增代码处遇到 `scrollTop` 的实验 Web API f64 / 稳定 API i32 签名差异（[原日志](build-studio.log)）。中间重试主动停止，最终改用签名一致的 `scroll_by_with_x_and_y`，重新构建并检查普通 wasm 模式。保留失败与中断日志，不计为通过。

## 静态与组件检查

[实际 argv / 退出码](checks.json)：以下均退出 0。

| 检查 | 结果 / 日志 |
| --- | --- |
| `cargo fmt --all -- --check` | [通过](fmt.log) |
| `mbx test --locked -p rustify-components --lib` | [121 passed](csr.log) |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | [169 passed](ssr.log) |
| `mbx clippy --locked -p rustify-components --all-targets -- -D warnings` | [通过](clippy-csr.log) |
| `mbx clippy --locked -p rustify-components --no-default-features --features ssr --all-targets -- -D warnings` | [通过](clippy-ssr.log) |
| `mbx check --locked -p rustify-components --target wasm32-unknown-unknown` | [普通 wasm API 通过](wasm-stable.log) |
| `mbx xtask css --check` | [通过](css-check.log)；提交前已用 `mbx xtask css` 重新生成 SDK CSS |
| `mbx xtask catalog --write docs/components.md --check` | [通过](catalog-check.log) |
| `mbx xtask sources verify` | [通过](sources.log) |

没有修订 DESIGN.md 或目标；设计 hook 的既有 sidecar 滞后提示不在本次 UI 修复范围。

## 最终构建与浏览器自检

| 检查 | 结果 / 证据 |
| --- | --- |
| `mbx xtask build-web --example theme-studio --release` | [通过](build-studio-final.log)，build `7b556624f53e8a55` |
| `mbx xtask build-web --example component-catalog --release` | [通过](build-catalog.log)，build `18ca82701d674efd` |
| `npx playwright test --project=theme-studio theme-ui-review.spec.ts --workers=1 --output=test-results/r6-studio` | [22 passed](browser-studio.log)，含三个品牌菜单组合、四个对话框选择器焦点、局部相反外观浮层、拾取、200% 文本与窄屏、非法字段与中文诊断 |
| `RUSTIFY_UI_EVIDENCE_OUT=docs/plan/DOM-COMPONENTS-P1.ui-captures/r6-apply/after npx playwright test --project=component-catalog dom-layout.spec.ts --workers=1 --output=test-results/r6-catalog` | [8 passed / 2 failed](browser-catalog.log)，八个既有布局/排版/语义/缩放/清理检查通过；两项新增测试误取宿主继承文字色 `rgb(29,41,57)`，实际有效 `--foreground` 为 `rgb(28 32 36)` |
| `npx playwright test --project=component-catalog dom-layout.spec.ts -g 'Code ghost keeps neutral foreground' --workers=1 --output=test-results/r6-catalog-confirm` | [2 passed](browser-catalog-confirm.log)，改为读取并由浏览器归一化有效 `--foreground`；只改测试 oracle，未再改产品代码。合计 32 个不同浏览器用例通过 |

两项新增回归都有修前失败信号。Code 修前日志的中性期望取值最初来自宿主，最终已纠正为有效主题 token；修前原始色为强调色，仍与正确的中性期望不同，不把错误 oracle 保留为最终契约。

[修后原始测量](after/measurements.json)、[当前源码与构建身份](current-version.json)：

- 三个菜单组合：打开时 Pink 完整可见、scrollTop=166；ArrowDown 后 Plum 完整可见、scrollTop=198；连续方向键、Home/End 与首尾环绕均可见。Enter 实际提交 Plum；再打开后 ArrowUp / Escape 保持 Plum 并回到触发器。回归测试还逐步断言编辑列 scrollTop 不变，Pink 的选中勾与 Plum 的活动反馈分离。
- 1280/390 Code：ghost 强调色及 high-contrast 两列均为有效中性 `rgb(28,32,36)`；无填充、无描边；显式 gray 的 a11 / high-contrast 12 阶与修前一致。逐项比较非 ghost 的颜色、背景、阴影、边框与字号，均未变化。九档 ghost 字级阶梯与 Kbd 已实拍核对。
- 8 张修前 PNG、15 张修后 PNG（含布局 spec 的两张整段图），都写入本轮目录，没有覆盖原评审图。受影响的三个菜单视口及两种 Code 宽度原图均已实际打开与目标对照。
- 五个修后受监视 context 的 pageerror、console error、失败请求、HTTP ≥400、CSP 事件均为 0；修前五个成功 context 同样记录为 0。首次未完成的 ready 可见性探针不算日志通过覆盖。
- 开工已有 1,586 个变动/未跟踪文件在台账回写前逐文件核对，变化为 0；原评审/目标/judge 与两张旧验证 PNG 保留。后续只按协议修改当前对象状态与必要索引。

## 未验边界与交接

本轮自检不评分、不宣称验收通过；处置为两项“已修改待评审”。没有停滞重构、待用户决定或产品阻塞。没有重跑整期 V1–V10、真实性能、独立消费者与子路径项目；VoiceOver、OS IME、Safari/Firefox、真机和真人研究未验证。没有更新视觉 golden、修改目标、发起独立评审、推送或部署。

本轮两个手工 strict CSP 服务已停止。下一步由用户运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`，独立判断差距是否关闭。
