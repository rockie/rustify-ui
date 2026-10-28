# UI R3 作者修复与自检

本记录支撑 [M7 逐项处置](../../DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)。报告中的 64 条未关闭差距均为「已修改待评审」，没有自行评分或独立验收结论。入库文本日志仅移除行尾空格，失败内容与结果保留。

## 版本与现场

- 起点：main，`dirty@d5775d8a661ae8832188362a3fdb00488ec34ac9`。开工已有的 560 个未提交文件先留指纹；仅按协议接续本轮 SDLC，原 PRD、报告、目标、评审实拍、judge、经验资料和两张旧验证 PNG 保留。测试截图均写入本轮目录。
- 当前源码与产物身份见 [current-version.json](current-version.json)。Studio release `eab8dd7edb84d4fb`，WASM 9,893,303 B；Catalog release `32b52b66f04701ad`，WASM 8,203,814 B。build ID 不代替 CSS/JS 的内容指纹。
- 工具：本仓 Rust 1.99.0 与 Playwright 1.63.0；浏览器实测 Chromium 153.0.8010.12，headless、DPR 1、reduced motion。沿用现有工具，无环境安装、依赖升级或新增服务容器。
- 本轮主要是 DOM。共享 Button 只调整 dark classic 高光；Inset/Reset 只调整演示排版，不改边集合/重置语义。UI-76 把既有 GPU 根内缩由 16px 改为 24px；未新增 GPU 能力、runtime/主题格式或存储契约。

## 实拍与实测

| 批次 | 结果与用途 |
| --- | --- |
| [before](before/capture-log.json) | 140 图，0 采集失败；改前现场 |
| [after](after/capture-log.json) | 148 图，0 采集失败；首轮作者自检。U6/U7 的 54 图仍是当前 Catalog 证据 |
| [confirm](confirm/capture-log.json) | 94 图，0 采集失败；38 个受监视页面的 pageerror、consoleerror、失败请求与 CSP 违规均为 0。当前 Studio 确认图 |

当前矩阵由 confirm 的 Studio 帧与 after 的 Catalog 帧组成。各批的实际动作、视口、主题、语言、干预和截图均保留在日志；采集器见 [capture.cjs](capture.cjs)。改前与首轮自检没有 pageerror、consoleerror 或失败请求；这两批旧版采集器未汇总 CSP 数组，不能据空字段声称检查了 CSP。最终确认采集器已补汇总。

[实测数据](confirm/apply-checks.json)：

- 720×900、根字号 200%：编辑列 scrollWidth/clientWidth 为 319/319，scrollLeft=0；预览列 400/400。改前红测溢出 79px。有效值 `#1570ef` 完整可见，范围 x=32..132.83，处于列内；[来源实拍](confirm/APPLY-zoom200-source-720-light.png)。
- 同一放大状态下，Output 完整显示 Theme JSON v2，选择器宽 558px；[导出实拍](confirm/APPLY-zoom200-export-720-light.png)。工具条高约 252.59px，占 900px 视口的 28.07%，不使用头区内滚动。
- 1440、768、390 宽时，所选 primary 与来源解除动作可同屏查看；390 的 token 位于 y≈285.61..363.50，解除动作 bottom≈827.66，在 844px 视口内。
- 四组配置状态与解除动作间实测 gap=8px，状态文字 rgb(71,84,103)。解除按钮键盘焦点帧保留，实际 title/aria-label 为 Clear primary · Light only。
- Inspect 切换前后，工具条各按钮的宽度和 x 坐标相同。Inspector 的确认拾取不再叠长标签，信息区占预览列底部独立空间，选中控件不被面板覆盖。
- 取消升级后的 Download current theme 实际触发下载，解析所得 JSON 保留内存中的 primary=`#234567`，没有把原始库下载冒充当前主题。

## 回归与检查

| 检查 | 实际结果 | 证据 |
| --- | --- | --- |
| Studio host bins | 69 通过 | [host.log](checks/host.log) |
| components CSR lib | 121 通过 | [components-csr.log](checks/components-csr.log) |
| components SSR、集成、doctest | 186 通过、2 ignored | [components-ssr.log](checks/components-ssr.log) |
| 相关 host Clippy，`-D warnings` | 通过 | [clippy.log](checks/clippy.log) |
| wasm Clippy | 按 build-web 的 cfg、限定当前包与既有例外运行；未宣称全部依赖的严格 wasm lint 通过 | 下文命令与日志 |
| fmt、CSS/catalog 一致性、sources verify | 通过；CSS 70,128 B / 4,475 candidates。sources 仍报告可归因的既有 Makepad drift，本轮无 fork 改动 | [fmt](checks/fmt-final.log)、[CSS](checks/css-final.log)、[catalog](checks/catalog-check.log)、[sources](checks/sources.log) |
| Studio 浏览器 | 85 个不同用例：84 通过，1 个既有字体资源口径失败。分批执行，保留实际失败和复验，没有把整条命令写成全绿 | [覆盖索引](checks/browser-coverage.json)、[首批 81 项](checks/regression.log)、[确认 22 项](checks/confirm-tests.log)、[大间距复验](checks/spacing-tests.log) |
| Catalog 浏览器 | layout/buttons 两份 spec，12 通过 | [catalog-tests.log](checks/catalog-tests.log) |
| release 构建 | Studio、Catalog 均成功 | [Studio](checks/studio-build-final.log)、[Catalog](checks/catalog-build.log) |

主要命令从仓根运行，构建与 Rust 检查使用本仓 target：

```sh
export CARGO_TARGET_DIR="$PWD/target" MBX_TARGET_VIEWS=0
mbx test --locked -p theme-studio --bins
mbx test --locked -p rustify-components --lib
mbx test --locked -p rustify-components --no-default-features --features ssr
mbx clippy --locked -p rustify-components -p theme-studio -p component-catalog --all-targets -- -D warnings
cargo fmt --all -- --check
mbx xtask css --check
mbx xtask catalog --write docs/components.md --check
mbx xtask sources verify
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example component-catalog --release
```

wasm 检查先发现两条本轮 lint（slice into_iter 与手写倍数判断），已修正。无 cfg 的命令出现与 build-web 不同的 web-sys 滚动坐标类型；全依赖严格检查还在未修改的 rustify-dom provider/theme/router/text 中报 9 项错误，保留 [原日志](checks/wasm-clippy.log)，没有扩展修复范围。最终使用 `RUSTFLAGS='--cfg=web_sys_unstable_apis'` 与 `--no-deps`：Studio 只放行原有 `clippy::suspicious_assignment_formatting`，Catalog 只放行原有 `clippy::unnecessary_unwrap`、`clippy::useless_conversion`，其余 `-D warnings`；[Studio](checks/wasm-clippy-studio-final.log)、[Catalog](checks/wasm-clippy-catalog.log)。没有在源码增加 lint suppressions。

浏览器始终一个 project、一个 worker 串行执行；`RUSTIFY_XTASK=target/debug/xtask`，截图输出用 `RUSTIFY_UI_EVIDENCE_OUT`、GPU 输出用 `RUSTIFY_THEME_GPU_OUT` 指向本轮目录。

```sh
npx playwright test --project=component-catalog dom-layout.spec.ts dom-buttons.spec.ts --workers=1
npx playwright test --project=theme-studio theme-ui-review.spec.ts theme-config.spec.ts theme-export-ui.spec.ts theme-exchange.spec.ts theme-storage.spec.ts theme-v2-storage.spec.ts theme-gpu.spec.ts theme-startup.spec.ts --workers=1
npx playwright test --project=theme-studio theme-ui-review.spec.ts theme-v2-storage.spec.ts theme-editor.spec.ts theme-review.spec.ts -g 'theme-ui-review|migration .* failure|immutable backup collision|Inspector suppresses|large spacing|tracking of optional' --workers=1
npx playwright test --project=theme-studio theme-review.spec.ts --workers=1
```

## 失败定位与未验边界

- 改前两项新增回归均先失败：[红测](checks/red.log)为放大时横向溢出 79px、Inspector 覆盖控件；当前同两项通过，包含在 UI 的 14 项确认用例中。
- 首批 81 项为 77 pass / 4 fail；把 Technical details 收到普通错误卡片时，误使升级失败状态隐藏了详情。恢复非普通状态的详情后重新 release，备份写入/回读/v2 写入失败与不可变备份碰撞 4 项全通过；普通配额失败的详情仍在错误卡片动作之后。首批其余覆盖范围未受这次条件渲染修正影响，复用其通过结果。
- 确认 22 项为 20 pass / 2 fail：手机拾取后的断言错误地要求查看已经隐藏的预览信息区；改为验证被拾取 token 的实际选中状态，390/1440 大间距交互均通过，没有扩大点击容错或强制点击。
- 另一项字体字宽测试独立重跑仍失败：GPU=106.0350px，DOM=104.421875px。实际资源是 GPU Plex Text TTF 与 DOM Plex Regular WOFF2；在浏览器显式加载 GPU 同一 TTF 后测得 106.046875px，与 GPU 相差约 0.012px。[实测及 HEAD 资源一致性](checks/font-parity-probe.json)表明字体映射与两份文件均与开工 HEAD 相同；该差异在本轮 64 条差距之外，未改字体、未放宽原测试容差，保留这一项未通过结果供后续处理。
- 构建未结束时曾有一次 server 拒绝启动（产物目录正在事务重建），没有执行测试；构建 exit=0 后才启动上表的有效批次。改前红测前也有一次端口占用启动失败，停止本次自建 server 后再跑，没有把启动失败计入产品用例。
- 未重跑整期 V1–V10、旧颜色负载/真实性能、独立 DOM/Docs/CLI 消费者及配对。未运行人工 VoiceOver、OS IME、Safari/Firefox、真机或真人研究；本轮 headless 功能结果不证明真实 GPU 帧预算。历史性能与人工缺口继续保留各自基线。
- 下一步由用户运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md` 独立复审。本轮没有改写评审、目标、judge 或关闭问题。
