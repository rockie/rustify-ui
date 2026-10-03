# Theme Studio 未提交变更代码评审

## 确认问题

### F-01 · [P2] 非零字距时关闭可选连字，使 GPU 排版与 DOM 一致

- 定位：[makepad/draw/src/text/font_family.rs:56](/Users/rockie/Documents/gh-rockie/rustify-ui/makepad/draw/src/text/font_family.rs:56)，目标工作树第 56–58 行，新增字距入口的 diff 内。
- 问题：设置合法的 `letter-spacing = 0.2em` 或 `0.5em`，并使用已打包的 IBM Plex Sans 显示含 `fi`／`ffi` 的文本时，新入口传入非零字距，却仍使用空 features，使 shaping 保留可选连字；随后只在每个 shaped cluster 后增加一次字距。DOM 在非零 tracking 下关闭这些可选连字，因此两端的字符间距、文本宽度与后续换行不同。浏览器中的同字体、同字号样本从零字距时几乎相同，变成 `0.5em` 时相差约 15.25px。这是本次将原来固定为零的字距接入 GPU 后实际激活的缺陷，超出了计划允许的抗锯齿差异。
- 证据：[shaper.rs:241](/Users/rockie/Documents/gh-rockie/rustify-ui/makepad/draw/src/text/shaper.rs:241) 第 241–246 行按 cluster 加间距；新增的 [shaper.rs:553](/Users/rockie/Documents/gh-rockie/rustify-ui/makepad/draw/src/text/shaper.rs:553) 第 553–564 行甚至断言非零字距仍保留同一 `fi` 连字。计划 [THEME-STUDIO.md:259](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:259)、[第 265 行](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:265) 要求统一字号／字距并验证连字与组合簇；[W3C CSS Text §7.2](https://www.w3.org/TR/css-text-3/#letter-spacing-property) 建议非零 tracking 不使用可选连字，必需连字与显式低层 feature 设置另行处理。实际宽度数据见下文。
- 修复与验证：在非零字距的 shaping 路径关闭可选 `liga`／`clig`，保留必需连字、重音组合与 emoji ZWJ，并保持零字距兼容；让有效 feature 配置进入缓存键。修改当前锁定错误语义的连字测试，补充同字体的 DOM／GPU 字距增量与换行对照，同时保留组合字符和 emoji 回归。
- 置信度：高；调用链、当前单测及真实浏览器测量相互印证。

### F-02 · [P2] 保留画布高度限制时让超出部分仍可滚动访问

- 定位：[examples/theme-studio/src/preview_region.rs:183](/Users/rockie/Documents/gh-rockie/rustify-ui/examples/theme-studio/src/preview_region.rs:183)，目标工作树第 183 行，新文件内。
- 问题：在桌面并排预览中通过编辑字段将合法的 `spacing` 设为 `64px`，控件布局继续按真实间距展开，但 `preview_height()` 把整个 GPU surface 截为 4096px，且没有对应的逻辑内容滚动或分页。浏览器中 dropdown 的起点为 y=4401、tabs 为 y=5007，两者可见高度均为零；GPU board 的 `clientHeight` 与 `scrollHeight` 均为 4094，设 `scrollTop = scrollHeight` 后仍是零。因此用户无法看到、点击或用 Inspector 拾取这些控件。此输入被正常接受且没有字段错误，不能把它当作非法主题处理；缺陷由新增预览板的高度截断引入。
- 证据：[gpu_preview.rs:40](/Users/rockie/Documents/gh-rockie/rustify-ui/examples/theme-studio/src/gpu_preview.rs:40) 第 40–47 行把截断结果写入容器高度；[preview_region.rs:614](/Users/rockie/Documents/gh-rockie/rustify-ui/examples/theme-studio/src/preview_region.rs:614) 第 614–615 行以画布区域裁剪控件。计划 [THEME-STUDIO.md:196](/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/THEME-STUDIO.md:196) 允许正数 spacing 且解析值不超过 4096 CSS px，`64px` 在范围内；R4／V4／V6 的可交互预览与 Inspector 承诺受影响。真实字段输入、滚动与 draw revision 一致性均已复核。
- 修复与验证：保留 GPU surface 资源边界时，将逻辑内容高度与 surface 高度分开，通过滚动投影、分段或分页使全部样本可达；不要静默缩小已接受的主题间距。增加 `spacing = 64px` 的浏览器回归，实际滚动到 dropdown／tabs，断言可见区域非零且操作和拾取成功。
- 置信度：高；真实 UI 输入接受、最终绘制坐标和无滚动范围共同确认。

## 待核实项与验证边界

- 未重新执行完整 V6／V7／V8 验收：真实硬件 GPU 性能、全部主题浏览器 spec、根／子路径部署、独立导出应用和所有旧示例回归仍依赖另行验证。本报告不把作者已有“已完成”记录视作本轮实测通过。
- 曾观察到字号修改后的 DOM／GPU 瞬时差异；反证确认 DOM 按钮有 CSS 过渡，等待 750ms 后两端均为 40px、字距均为 8px，故排除此候选，未计入 findings。
- 没有其他达到缺陷门槛的待定问题。未运行的验收项目是覆盖限制，不代表它们已经失败。

## 覆盖与验证

### 意图与固定基线

本次变更将 Tailwind v4 主题文档、DOM／GPU 主题投影、编辑历史、主题交换及本地保存接入 SDK，并新增 Theme Studio 静态示例。

| 项目 | 本轮范围 |
| --- | --- |
| 用户输入 | `$review-code 未提交的代码 docs/plan/THEME-STUDIO.md` |
| 仓库根 | `/Users/rockie/Documents/gh-rockie/rustify-ui` |
| 评审日期 | 2026-10-04，Australia/Sydney；收口前时钟为 2026-10-03 15:53:01 UTC |
| 模式 | uncommitted；`HEAD` 对比最终工作树，包含未跟踪文件 |
| base SHA | `7f5e5042cb647ab474f0ef6c833ea568c75b53d4` |
| target | 本地工作树，无 target commit SHA |
| index | 无 staged diff |
| 文件范围 | 43 个 tracked 变更文件、229 个 untracked 文件；无路径过滤 |
| 快照 | 272 个变更文件加 4 个上下文文件，共 276 个 SHA256 指纹，见 [快照清单](THEME-STUDIO.code-review.snapshot.json) |
| 快照清单 SHA256 | `6ce67eb1da02718dcf6e116b6103ae34aa5209a6794541bed79e8154bed488bd` |
| 计划 SHA256 | `ae4113797cc43bf23cd2f3a7e8aae7ca2676c5477eeb3002c77e6c9637ab7093` |
| reviewer | 当前单一 reviewer；未使用独立 agent 复核 |

开工读取仓库规则、README、产品／设计上下文、包管理与工具链配置及指定计划。依据 review-code 与 Rust skill 核对变更与调用链。收口时重算全部 276 个指纹，均与初始快照一致，HEAD 和空 index 也未变化。新增本报告与快照清单不纳入本轮审查对象；没有修改实现或计划。

### 已覆盖行为链

- 主题内核：文档与 token 集、legacy 转换、颜色／度量／字体／阴影解析、双模式及输入边界、CSS／JSON 编解码、预设与来源。
- DOM 与兼容：ThemeScope／Boundary、CSS 派生变量、独立 scope、overlay 有效主题同步、受影响组件样式、旧 API 包装与 catalog fixture。
- GPU 与文本：各控件主题入口、RGBA／几何／字体／阴影投影，TextStyle → layout → shaping／缓存 → 绘制与测量，预览板布局、报告坐标和命中边界。
- 站点状态与交互：编辑／手势／历史、公共字段与 HSL、七场景、Inspector、存储事务／失败／跨页冲突、导入候选与导出流程、mount／dispose 生命周期。
- 交付链：workspace／lock、构建资源、Playwright project、CI 检查入口、文档与字体修复脚本。静态检查按风险聚焦生产逻辑和相关测试；验证目录的大量历史日志、截图及原始指标未逐项独立重验，新增长测试亦未逐条运行。

### 实际执行结果

所有成功验证均针对上述工作树。未下载依赖或浏览器、未启动基础设施容器；浏览器使用已有 Playwright Chromium 和隔离临时 context。

| 命令／方式 | 结果 | 限制 |
| --- | --- | --- |
| `mbx test -p rustify-ui --lib theme` | 71 passed，154 filtered out | 仅主题相关 lib tests |
| `mbx test -p theme-studio --bins` | 46 passed | 站点 host 状态／编解码等逻辑 |
| `sh tests/theme-text/run.sh --lib --quiet` | 34 passed | 文本隔离测试；包含 F-01 所述现有错误语义断言 |
| `mbx xtask sources verify` | exit 0 | 固定来源／哈希校验，不是全部历史 fork 行为回归 |
| `mbx xtask css --check` | exit 0；no drift，28725 bytes | committed component CSS 一致性 |
| `python3 scripts/repair-theme-font.py --check makepad/widgets/resources/LXGWWenKaiRegular.ttf` | exit 0，输出哈希与清单一致 | 脚本报告 `browser_ots_verified: false`；不据此宣称 OTS 通过 |
| `mbx xtask build-web --example theme-studio --release` | exit 0；build id `4f136b64c3876ce6` | wasm 28,645,106B；静态总产物 80,973,599B；构建输出含 2 个 rustify-ui warning 及 proc-macro-error2 future-incompatibility 提示 |
| `node_modules/.bin/playwright test tests/browser/theme-dom.spec.ts tests/browser/theme-editor.spec.ts --project=theme-studio --output /tmp/rustify-theme-review-playwright` | 19 passed，36.3s | 单 project 串行；未运行完整 theme suite |
| 临时 Playwright 最小复现 | F-01、F-02 均复现；字段输入 probe 的 pageerror 列表为空 | Chromium headless；不是硬件 GPU 性能测量 |

首次 Playwright 调用把 spec 路径放在 `--project theme-studio` 后，被 CLI 当作额外 project 名而提前退出，未执行测试。改用 `--project=theme-studio` 后获得上表 19 项通过结果；该调用错误不归因于目标代码。

浏览器入口为新构建的 `http://127.0.0.1:4181/`，服务器使用 `target/debug/xtask serve --example theme-studio --release --port 4181`，输出 `csp: Strict`。复现从首页编辑面板进入；通过 revision 与 drawn_revision 相等确认最终 GPU 绘制，分别核对正常字段输入、大间距可达性和同字体字距测量。预览服务器已在评审结束前停止。

### 最小复现数据

F-01：等待打包 IBM Plex Sans 加载，使用 GPU 样本 `font.sans.latin` 的实际 layout 宽度，对照同源字体、15px、`white-space: pre` 的 DOM span。文本为 `Rustify · ffi fi · é`，末尾字符使用 `e` 加 U+0301。DOM span 显式设置字体／字号／字距，避免继承样式与过渡影响。

| letter-spacing | GPU 宽度（CSS px） | DOM 宽度（CSS px） | DOM − GPU |
| --- | ---: | ---: | ---: |
| 0em | 106.035011 | 106.046875 | 0.011864 |
| 0.2em | 160.034988 | 166.281250 | 6.246262 |
| 0.5em | 241.034958 | 256.281250 | 15.246292 |

F-02：1440×900 viewport，默认预设和并排布局，点击 `editor-controls`，把 `edit-spacing` 填为 `64px` 并 Enter。作者值为 `64px`，字段错误为空；最后一次绘制报告如下。尝试将 GPU board 的 scrollTop 设为 scrollHeight，最终仍为 0。

| 项目 | 实测值 |
| --- | --- |
| GPU board clientHeight／scrollHeight | 4094／4094 |
| dropdown rect `[x,y,w,h]` | `[16,4401,492,576]` |
| dropdown visible_rect | `[16,4094,492,0]` |
| tabs rect | `[16,5007,492,576]` |
| tabs visible_rect | `[16,4094,492,0]` |

本轮原始临时日志与复现脚本保留在 `/tmp/rustify-theme-review-*`；关键输入、观测结果及复现步骤已写入本报告，避免结论仅依赖临时文件。历史 validation 证据与本轮新增观测未混为同一次验收。

## 计划符合性

对照计划为工作树中的 `docs/plan/THEME-STUDIO.md`，版本由上表 SHA256 固定。计划第 37–53 行把 M1–M6 全部标记为完成，因此本轮没有按“后续尚未实施”豁免其承诺；也没有改写原计划进度。

| 契约／验收 | 本轮核查 | 裁定 |
| --- | --- | --- |
| R1／V1：文档、解析、预设、legacy 转换 | 主题核心及站点 host tests，来源校验 | 所执行检查通过；未宣称覆盖全部 V1 历史基线 |
| R1／R2／V2：DOM scope、局部覆盖、overlay、CSS 一致性 | 静态调用链、6 项 DOM 浏览器测试、CSS check | 所执行检查通过；完整旧 catalog 同产物兼容未重跑 |
| R2／V3：GPU 字距与真实度量 | 文本 34 项测试、同源字体浏览器对照 | **存在 F-01**；缓存传递成立不等于 CSS 排版语义一致 |
| R3／R4／V4：编辑、历史、Inspector、预览控件 | 46 项 host tests、13 项 editor 浏览器测试、大间距实际输入 | 常规路径通过；**存在 F-02**，合法主题使部分控件不可达 |
| R5／V5：导入导出、保存恢复与冲突 | 编解码／存储主逻辑及 host tests，保存／导出浏览器最小探测 | 部分核查；未重跑跨页及独立导出应用完整验收 |
| R2–R5／V6：最终组合可用性 | 最新 release 根路径及局部浏览器回归 | F-01／F-02 需修复；全站主题、语言、视口及故障矩阵未重验 |
| NFR2／V7：性能与持续编辑收口 | 检查帧合并／缓存／revision 链路 | 静态核查；没有本轮硬件 GPU p50／p95 结论 |
| C1–C4／V8：交付、兼容、部署 | release build、sources／CSS checks、CI 配置阅读 | 部分通过；全示例构建、全量检查与子路径／导出运行未重跑 |

## 总体结论与修复顺序

**patch is incorrect**。确认两项 P2 缺陷：F-01 破坏 DOM／GPU 字距排版一致性，F-02 使合法主题下的 GPU 控件不可访问。核心测试、局部浏览器回归和 release 构建通过，不能覆盖这两处已复现的契约缺口。

先修 F-01 的 shaping feature 语义并保留零字距／组合簇兼容，再修 F-02 的逻辑内容可达性；修复后重跑受影响文本、编辑器与 GPU 浏览器检查，加入本报告的字距对照和 `64px` spacing 场景。解除对应问题的可观察条件是：字距增量与 DOM 语义一致，且所有预览控件在合法间距下均可滚动到达并实际操作。代码与计划维持原样，本报告不包含发布或合并授权。
