# Theme Studio：旧 component-catalog 主题基线

采集时间：2026-10-03 11:00:26 UTC。范围为 THEME-STUDIO M1/V1 的旧默认样式与颜色更新基线；不是 V2–V8 的完成记录。

使用已存在的 `target/makepad-wasm-app/release/component-catalog/` 静态产物，入口为 `http://127.0.0.1:4176/`，xtask strict CSP。采集时 Git HEAD 为 `7f5e5042cb647ab474f0ef6c833ea568c75b53d4`，工作树包含正在开发的 M1 纯 Rust 主题模块与依赖变更；旧 `Theme`、`ThemedScope`、catalog props、GPU 绘制和 sdk.css 默认行为未改。此次没有重新构建、修改产品或注入新的主题 API。

完整原始数据、20 次切换的逐项时间、CDP GPU 信息、manifest 和资源 SHA-256 在 [measurements.json](measurements.json)。可复跑的浏览器探针为 [collect.mjs](collect.mjs)。所有时间取自页面内 `performance.now()`；不使用 Playwright 往返时间作为性能值。

## 环境与产物身份

| 项目 | 实测值 |
| --- | --- |
| 浏览器 | Google Chrome 154.0.8037.93；性能采集使用 headed 模式 |
| GPU | ANGLE Metal Renderer: Apple M5 Pro |
| GPU/ANGLE 版本 | Version 27.0.1 (Build 26A434)；ANGLE 2.1.28733，git hash `802a8704ca94` |
| WebGL | WebGL 2.0；OpenGL ES 3.0 |
| 视口 / DPR | 1280 × 960 CSS px / 1；页面为 visible |
| wasm / build_id | 13,488,106 bytes / `3a3760a88d5fa623` |
| manifest SHA-256 | `002c929ab0c5f5e4788f928bc2fb3d8463ad840cca4a454ce86a71aca238f466` |
| wasm SHA-256 | `3a3760a88d5fa623fb80f20e9a6cdc9581c36f864cc3a5187db50559ed1d6f05` |
| Tailwind CSS SHA-256 | `c9372fa1760685f7aeecc48eb17b62435b521c606f64de90dd07ef20951ab1f6` |
| CSS / fonts / 静态目录总量 | 27,902 / 51,187,844 / 64,946,830 bytes |

其余 CSS、app.js、embedded.js 和实际构建后的 WebGL renderer JS 哈希同样保存在 JSON 的 `assets` 中。尺寸来自本次读取的 manifest，未压缩。

## 既有回归

串行运行：

```sh
RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
RUSTIFY_TIER=regression \
npx playwright test --project=component-catalog p2-theme.spec.ts \
  --reporter=list --output=docs/validation/theme-studio/baseline/p2-theme-results
```

结果：**6 passed (14.7s)**。覆盖两帧后主题采用、GPU 像素变化、按钮四态、DOM/GPU 降低动效、GPU spinner 停止、宿主样式隔离。regression tier 按既有约定只做 6 次切换，跳过 `@evidence` 的 20 次/200 ms 预算测试；本记录另用自有采集器完成 20 次完整采样。没有把跳过项记作通过。[运行摘要](regression.txt)与 [Playwright 结束状态](p2-theme-results/.last-run.json)保留在本目录。

## 旧默认样式

读的是当前产物的 computed style。圆角探针为附加到旧 scope 内的临时 DOM 元素，只使用产物已经包含的 `rounded-sm/md/lg` utilities，随后删除；没有改 CSS。

| 项目 | light | dark |
| --- | --- | --- |
| `rounded-sm` / `rounded-md` / `rounded-lg` | 4 / 6 / 8 px | 4 / 6 / 8 px |
| `--radius` / `--font-size` / `--spacing` | 6 / 15 / 8 px | 6 / 15 / 8 px |
| `--primary` | `#1570ef` | `#53b1fd` |
| `--background` | `#f9fafb` | `#0c111d` |
| primary Button 背景 | `rgb(21, 112, 239)` | `rgb(83, 177, 253)` |
| primary Button 文字 | `rgb(255, 255, 255)` | `rgb(12, 17, 29)` |
| primary Button 字号 / 内边距 / 圆角 | 14 px / 8 px 16 px / 6 px | 同 light |
| catalogue / header / header Row / example gap | 16 / 8 / 8 / 8 px | 同 light |
| scope 字号 / 字体 | 15 px / system-ui, sans-serif | 同 light |
| Dialog 面板圆角 / gap | 8 / 16 px | 同 light |
| Menu 圆角 | 6 px | 同 light |
| Dialog / Menu 背景 | `rgb(255, 255, 255)` | `rgb(29, 41, 57)` |

Dialog 的 test-id 指向外层 `rustify-layer`；阴影读取其 `[data-name="Dialog"]` 子面板。Menu 读取真实 `[data-testid="default-menu"]` 面板。浅、深模式下两者的 `shadow-lg` computed box-shadow 均为：

```text
rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0.1) 0px 10px 15px -3px, rgba(0, 0, 0, 0.1) 0px 4px 6px -4px
```

computed style 的稳态读数在主题切换 250 ms 后读取，避免把 150 ms transition 的中间颜色当成最终颜色。截图中旧 dark scope 的面板背景跟随主题，但目录标题和部分普通正文仍继承宿主 `rgb(29, 41, 57)` 文字色，深背景上明显难读；这是旧产物中实际存在的视觉缺口。本基线不是暗色可访问性通过记录，本任务只记录，没有修复。后续旧 scope 兼容比较应沿用这份事实基线，新编辑壳与预览则须正确消费前景色。

## 20 次旧主题颜色切换

旧 catalog 的主题动作是 `Theme::light()` ↔ `Theme::dark()`。它不提供任意 primary 颜色输入或 wasm 导出 setter，因此本基线使用旧主题本来支持的颜色更新动作，未把 DOM-only ColorField 称为双端主题编辑。

每轮在页面内记录 click 起点；用临时 `setAttribute` 探针记录旧 `ThemedScope` 写完 tokens 后提交 `data-theme` 的时间；用临时 `WebGL2RenderingContext.drawElementsInstanced` 探针记录 catalog canvas 的实际 GL 调用返回时间；再等待两个 rAF，核对 `--primary`、模式、region 数和 runtime.frames。这里的 DOM 提交表示 CSSOM/主题属性已经更新，不表示 transition 已经完成。

| 观测点 | p50 | p95 |
| --- | --- | --- |
| input click → DOM token/attribute 提交 | 0.20 ms | 0.20 ms |
| input click → 首个 GPU draw 提交调用返回 | 1.20 ms | 1.50 ms |
| input click → 最后一个 GPU draw 提交调用返回 | 1.20 ms | 1.50 ms |
| input click → 两个 rAF 完成 | 16.40 ms | 18.10 ms |

p50/p95 用 nearest-rank（20 样本的第 10 / 19 个排序值）。所有未经取整的原始数组和每轮时间在 JSON `timing` 中。每轮发生 1 次 runtime frame、5 次 `drawElementsInstanced`；总 frame +20、GL draw call +100。所有 20 轮都到达期望模式并改变 primary。20 轮的测量窗口内长任务（PerformanceObserver `longtask`）为 0；这一窗口约 335.20 ms，不包含 GPU/字体冷启动。

切换前后 live region、runtime region、WebGL context 数均为 1，context 新增 0；GPU 资源 ledger 为 45,960 bytes，wasm linear memory 为 37,224,448 bytes，切换前后相同。旧 `Theme` 是直接值表，浏览器 handle 没有 resolver counter，也没有 draw revision。`resolver.count` 保留 null，而不是伪造 0 次 resolve。GL 时间是 CPU 端提交代理；它不能证明 revision 对应的 GPU 完成、合成完成或显示器实际出光。

reset 后额外观察 1,502.70 ms：runtime.frames +0、pumps +0，timers / animation_frames / tasks 均为 0。此结果覆盖静止的 button 分类；不是 spinner 等主动动画分类的空闲声明。

颜色字段补充观测：输入 `#d92d20` 后应用 colour 为 `#d92d20`，CSS transition 后 DOM swatch 为 `rgb(217, 45, 32)`；scope primary 前后仍为 `#1570ef`。该字段未进入 `CatalogProps.theme`，不具备 GPU 主题颜色更新链路。

## 截图、异常与边界

- [DOM/GPU light](dom-gpu-light.png)、[DOM/GPU dark](dom-gpu-dark.png)。
- [GPU light](gpu-light.png)、[GPU dark](gpu-dark.png)：320 × 171 PNG；两图有 53,802 个像素的 RGB 差之和大于 24，证明两模式的实际像素不同，不能以此证明每个主题 token 的准确性。
- console 唯一 error 已由 `location.url` 定位为 `http://127.0.0.1:4176/favicon.ico` 的 404；没有 requestfailed 或 pageerror，未观察到 CSP 报错。
- legacy diagnostics 另有 1 条 `UnsupportedCapability` error：`a region asked to set the document title; only the host page can`。runtime.stats.errors 与持久 diagnostics 并非同一计数；没有把它们统称为零异常。
- 旧 API 没有可供对照的 resolver 次数、revision acknowledgement 或任意主题颜色输入。M3/M6 的新 draw revision 和连续编辑指标仍须另行验证；当前结果不能替代。
- 页面内探针会增加少量开销。后续对比需使用同一机器/浏览器/GPU/资源状态并说明新旧探针的差异，不把当前 submit 代理直接比较为新的 revision ack。

## 复跑

先确认 4176 空闲，且对应旧产物仍是上述 hash。用已有入口串行启动服务，再运行采集器：

```sh
target/debug/xtask serve --example component-catalog --release --port 4176
RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
node docs/validation/theme-studio/baseline/collect.mjs
```

两条命令分别在服务终端和采集终端运行。不要与其他 Playwright 命令并发。完成后停止服务；此次采集器正常退出并关闭自行启动的 Chrome，4176 服务也已停止。

来源核查：`CLAUDE.md` browser 约定；`tests/browser/p2-theme.spec.ts` 的两帧/像素检查；`tests/tier.ts` 的 regression 缩减与 evidence 过滤；`crates/rustify-ui/src/theme.rs` 的旧 Theme/properties/ThemedScope；`examples/component-catalog/src/main.rs` 的 switch_theme、ColorField 与 CatalogProps；`crates/rustify-components/css/sdk.css` 圆角与 spacing 定义；`dialog.rs`/`menu.rs` 面板类名；`crates/rustify-makepad/web/embedded.js` 的 frames 与 stats；构建后的 `makepad_platform/web_gl.js` 实际 `drawElementsInstanced` 调用。
