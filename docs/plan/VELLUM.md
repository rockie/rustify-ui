# VELLUM · Rustify UI · examples/vellum（Vellum 设计编辑器克隆 · Leptos DOM 外壳 + 一个 Makepad WebGL2 画布区域 · 视觉与交互对标 Vellum）

> **计划状态：Ready**
>
> 调查基线：2026-09-16 · `a92caa0b5d57fc35c4a6c227c73096025107b5cd` · 工作区 clean（除本计划）。参考源码 `ref/Vellum-main` 被 `.gitignore` 忽略，其事实来自当前工作树（Vellum 0.1.0，MIT）；SDK 事实来自 HEAD。
> 输入：用户要求「用 rustify-ui 把 ref/Vellum-main clone 到 examples 里；实施时不跑 rustify-ui 自己的测试；有需要可以修改 rustify-ui 代码」。2026-09-16 评审答复：改到 SDK 模块时允许过滤单测；示例测试进 CI；**「我们是 UI 框架，视觉渲染和交互控制是两个最重要的核心需求，视觉差异要尽可能小」**；完成后要一份说明「用了框架的什么功能来实现」的文档，体现框架价值而不是「用 wasm 输出一堆 HTML 和 JS」。前序计划 P1–P3 均 Closed，本计划是独立的示例立项，不是 PRD 的 P4。
>
> 本期交付：**`examples/vellum`——可用 `cargo xtask build-web --example vellum --release` 构建、在 `cargo xtask serve` 的严格 CSP 下运行的设计编辑器示例。Vellum 的五层（WebGPU 场景画布、2D 覆盖画布、contenteditable 文本层、DOM 外壳、IndexedDB 持久化）逐层换成框架的对应物：场景画布 = 一个 `GpuRegion`，里面是 Vellum WGSL 渲染器的逐行移植；覆盖画布与指针状态机 = DOM 侧 1:1 移植；文本层 = SDK `TextEdit`；浮层 = SDK 覆盖层栈；外壳 = Vellum 的标记与样式原样进 Leptos。`.vellum` v1 与 Vellum 双向互开；Vellum 的 36 项浏览器检查逐项移植全绿；视觉与交互与 Vellum 在同机同浏览器下做孪生对照，差异计数记录并设门；随代码交付 `docs/vellum.md`：逐能力说明框架提供了什么、应用写了什么、换成裸 wasm + 手写 HTML/JS 要付出什么。**
> 本期独特职责：仓库里第一个「整块画布由一个 GPU 区域承担、编辑逻辑全部在 Rust 里」的完整应用；也是第一次以「与既有产品逐像素、逐手势对照」为验收口径。
> **顶层排除：不做 WebGPU（分叉只有 WebGL2）；不移植 Vellum 的 `build.py` 单文件版、`scripts/`、Pages 工作流与 Python 测试工具（Vellum 的 JS 只在测试里作为孪生参照运行）；Vellum 自己声明不做的（协作、插件、`.fig`、布尔运算、变体、富文本、文本沿路径）本期同样不做。**

## 实施者定位

执行本计划的 agent 是**资深软件工程师**：Kent Beck 式的 TDD 纪律加上《程序员修炼之道》式的精确。本节由骨架原样带入，不随项目改写；开始任何里程碑前先接受以下约定。

- **表达方式**：极简，每句话都可引用。说到代码给文件路径，说到需求或验收给本计划的 ID（需求账本 R-* / NFR-* / C-* / A-*，决策 D* / ADR-*，里程碑 M*）。不写铺垫、不写感想、不复述计划。
- **完成的定义**：没有通过验证的任务不算完成。里程碑退出条件里的测试、断言和走查全部通过，才能在「实施进度」记为完成；验证没跑、失败或环境缺失，就如实记为缺口或阻塞。
- **工作顺序**：先写会失败的测试（红），再写最少的代码让它通过（绿），最后重构；三步不倒序、不合并。里程碑按本计划写的顺序执行，不跳步，不同时开两个未完成的里程碑。
- **源码干净**：注释解释为什么，不解释是什么。源码里不出现工单或需求编号（如 `# FR-12`、`// BUG-42`）、本计划的 R-* / M* 编号、agent 工作流标记或任何规划元数据；追溯关系只记在本计划的「实施进度」和提交说明里。交付的是可直接上生产的代码：干净、最小，没有多余防御、空洞注释或重复样板这类 AI 生成痕迹。

## 实施进度（实施期持续更新）

本节由骨架原样带入，不随项目改写。它是跨对话恢复的**唯一**入口：新对话只读仓库规则、本计划正文、本节和当前工作树，就要能确认已完成事实、验证基线与下一步。

**回写时机（硬性顺序）**：某个里程碑的退出条件全部跑绿之后，**下一个动作就是回写本节**——早于向用户报告完成、早于开始下一个里程碑、早于提交代码。实施暂停、被阻塞、发现计划偏差或本轮对话即将结束时，同样先刷新快照再停。

**为什么不能攒到本期收尾一次补**：证据（命令、断言数、实测数字、走查结论）只有刚跑完时是准确的，事后补写的是记忆；而对话中断、上下文耗尽或用户新开对话时，没回写的里程碑对下一个实施者等于没做过——它会照着「下一步」把已完成的工作重做一遍。所有里程碑共用同一个完成时间和同一个代码基线，就是攒着补写的痕迹。

**开工前先读**：每个里程碑动手前，先读本节的「下一步」与最新一行完成记录，与 `git status` / 工作树核对。记录与工作树冲突时先查明真相再改本节，不能凭记录覆盖用户改动，也不能凭工作树臆断已完成。

**一次回写 = 三个动作**，缺一不算回写：

1. 重写「恢复快照」全部 7 行，不是只改「最近完成」。
2. 「完成记录」追加该里程碑一行（首次追加时删掉占位行）；既有行只在纠正事实时修改，不写重复流水。
3. 实现与计划不一致时（范围、决策、接口、数据、风险或退出条件有变），同步修订正文对应章节，并在快照的「当前状态」里点明改了哪一节。进度区不是绕过计划一致性的补丁堆。

**记完成的门槛**：退出条件全部通过才可记完成。验证没跑、跑红或因环境缺失跳过，就如实写进「当前状态」/「当前阻塞」，不写“基本完成”“应该可用”。

**回写后自查**，三条都对才算回写完：「当前进度 n/N」的 N 等于里程碑表行数、n 等于完成记录里的里程碑行数；「最近完成」等于完成记录最新一行；新增行的「验证证据」与「代码基线」都不是空或 `—`。

### 恢复快照

- 最近更新：2026-09-16 18:35 CST
- 当前进度：8/8 个里程碑完成
- 当前状态：M8退出条件通过；142项Rust测试、11个spec共80项浏览器测试（含连续36项冒烟）全绿，43文件双构建一致；六张启动页差异0.0335%–1.651%，固定每张≤2%；§3/§5/§6已同步浮点精确往返、Canvas读回取舍与Enter默认动作修复，CI及框架说明/验收报告完稿
- 最近完成：M8 · 收尾：全套、视觉门、走查、文档与 CI
- 下一步：本计划开发与自动化验收已完成；后续继续缩小视觉差异，真人设备可用时补系统拼音和物理触控板体验记录
- 当前阻塞：无实施阻塞；按§9.4，系统拼音输入法与物理触控板明确记为未测，有头自动化及代理截图查看不替代这两项人工操作
- 代码基线：`dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；M8修改`examples/vellum/`浮点解析/键盘/入口、`tests/vellum/`、`.github/workflows/verify.yml`及文档；未提交，证据索引为`test-results/vellum/m8-verification.json`与`docs/validation/vellum/m8.md`

### 完成记录

| Milestone | 完成时间 | 准确完成摘要 | 验证证据 | 代码基线 |
| --- | --- | --- | --- | --- |
| M1 | 2026-09-16 15:17 CST | 注册 vellum crate；Forma 金样与 Rust 启动文档保留原版全部既有字段，三页 171/31/0 层。完成纯 Rust 编辑规则、80 步可撤销文档替换与资产共享、统一文本布局/光栅策略/SVG；属性编辑形成实例覆盖；`docs/vellum.md` 已说明模型层边界。 | `cargo test -p vellum`：134/134；`cargo run -q -p vellum -- --starter` 导出后 `node tests/vellum/interop.mjs --check-starter test-results/vellum/forma-rust.vellum`：原版解析 202 层且字段一致；`cargo clippy -p vellum --all-targets -- -D warnings`、`cargo fmt --all -- --check` 通过；文档路径检查 MISSING 0；日志 `test-results/vellum/m1-*.log` | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；`examples/vellum/src/`、`tests/vellum/fixtures/forma.vellum`、`tests/vellum/interop.mjs`、workspace manifests、`docs/vellum.md` |

| M2 | 2026-09-16 15:37 CST | 单区域渲染启动页 171 层/173 实例/144 次绘制；5,000 形状可编辑且 drawCalls=1；完整浮点纹理裁剪链与预乘合成通过，GPU 纹理随当前投影释放；提供原版外壳骨架、标签覆盖画布、自动化面和无 GPU 状态。首张亮色孪生差异 1.8818125%。 | release build 成功；独立 `m2-scene.spec.ts` 6/6、首张 `visual.spec.ts` 1/1；四祖先圆角像素断言与叠色 [70,6,135] 通过；20 级缩放三轮账本序列相同，峰值 12,846,744 B；5,000 形状 20 次编辑 CPU p95 5.90 ms、20 呈现帧；host 135+5 测试、Clippy、格式检查通过；日志 `test-results/vellum/m2-*.log`，探针结论见 README 与 `docs/vellum.md` | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；`examples/vellum/src/{app,automation,overlay,shaders,region,raster,painter,assets}.rs`、入口/CSS/README、`tests/vellum/`、`docs/vellum.md` |

| M3 | 2026-09-16 16:01 CST | 本地字体以字节FontFace加载，保持严格CSP；Unicode换行与样式文字像素匹配原版。修复撤销分支和同键资产旧像素；64 MiB LRU实测淘汰。冷缩放分8 ms批次并保留旧分辨率，首次呈现15–21.6 ms、2/4/6帧细化。六张视觉差异0.2646%–1.8811%，已提请用户定门值。 | release build与wasm check通过；`m3-raster.spec.ts` 7/7、M2回归6/6、六张`visual.spec.ts` 1/1；有头Chrome152/Apple Metal原版WebGPU/Canvas对照1/1；文字盒阈值24差异0；缓存58,867,200 B、104次淘汰；Clippy/fmt通过；日志 `test-results/vellum/m3-*.log`、`m3-webgpu.json`、`visual.json`及分spec截图目录 | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；字体、光栅批次/GPU更新、app/automation接线及独立浏览器探针；`docs/vellum.md`/README |

| M4 | 2026-09-16 16:21 CST | 完整指针/滚轮/捏合状态机、画布快捷键与覆盖画布；选择按原版顺序保留。六组孪生几何一致，选择手柄/悬停/框选/参考线/钢笔/标尺像素阈值24差异0；取消/失焦与双指通过。平移p95 44.5 ms，未触发ADR-2重评。 | release build、`m4-pointer.spec.ts` 13/13、六张`visual.spec.ts`默认≤2%门1/1（0.3079%–1.9233%）；Clippy/fmt通过；页内30次平移`m4-pan.json`；`test-results/vellum/m4-*.log`及分spec截图；`docs/vellum.md` M4职责与尺寸/监听说明 | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；指针/覆盖层/键盘、app/automation、M4浏览器脚本与文档 |

| M5 | 2026-09-16 16:53 CST | 完整Leptos外壳、受控检查器、页面相机/选择记忆、搜索图层树与原生拖放、SDK Layer菜单/模态、33项命令面板和主题。结构Memo与独立选择更新保留树双击目标；SDK处理Escape/焦点/inert，应用补Tab循环。六张启动页差异降至0.0335%–1.651%。 | release build成功；`m5-shell.spec.ts` 10/10、`visual.spec.ts` 2/2（六张启动页+主菜单1.661625%/帮助0.6770625%，全部≤2%）；M4回归13/13（平移p95 34.8ms）、M2回归6/6（5,000形状drawCalls=1、编辑CPU p95 6.2ms）；Clippy/fmt与文档路径检查通过；日志`test-results/vellum/m5-*.log`，最终截图`artifacts/m5-visual-final/`，`docs/vellum.md`外壳职责已写入 | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；shell全模块、app/icons/keys/automation/CSS，M5外壳与视觉脚本、README及框架文档 |

| M6 | 2026-09-16 17:16 CST | SDK TextEdit可选变换、原生文字会话/GPU skip、同步文本事务与外部值失效保护；系统剪贴板真实结果与内部回退、载荷校验、令牌下载；组件双事务及完整编辑接线。旋转/相机、Unicode、CDP组合输入、系统撤销、文字工具/双击/blur通过；文字与钢笔孪生几何一致。 | `m6-edit.spec.ts` 13/13，文字/钢笔截图0.0486875%/0.075125%；外壳回归10/10、指针13/13（平移p95 34.1ms）、`visual.spec.ts` 2/2八张均≤2%；host136+5、Clippy/fmt、wasm check/release通过；SDK过滤`cargo test -p rustify-ui --lib text::`成功但0匹配（DOM行为由浏览器覆盖），旧property-workbench wasm check通过；`test-results/vellum/m6-*.log`、框架文档与README已更新 | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；`crates/rustify-ui/src/text.rs`、text_session/clipboard、app/pointer/keys/history/automation、M6用例与文档 |

| M7 | 2026-09-16 18:07 CST | IndexedDB防抖/串行保存与localStorage回退；可撤销文件替换、图片/字体选择和拖放、SVG/PNG画家、框架演示。fatal中止旧保存/事务/rAF/字体，重启恢复已保存副本；SDK picker释放监听，字体来源随Undo/Redo恢复。Canvas固定软件读回，保留4像素AA差异以避免GPU读回性能回退，遵守用户2%门。 | `m7-files.spec.ts`22/22、互开3页202层；光栅7/7（裁图4/120000、冷缩放101.1ms/4帧）、八张视觉2/2均≤2%；文本13/13、外壳10/10、指针13/13回归；host136+5、files过滤7、Clippy/fmt与release通过；`test-results/vellum/m7-complete.log`、`m7-complete-interop.log`、`m7-canvas-{build,raster,visual}.log`及文档路径检查 | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；storage/fileio/fonts/browser_frame/presentation/app/painter、`crates/rustify-ui/src/files.rs`、M7用例与框架文档/README |

| M8 | 2026-09-16 18:35 CST | 完成36项连续冒烟、固定2%视觉门、启动测量、有头Chrome/Metal走查、CI两步和框架/验收文档；修复JSON往返1ULP漂移与Enter重开文字覆盖原文，保留原版favicon。六张Canvas参照差异0.0335%–1.651%；有头WebGPU六组整页0.019125%–0.2719375%，上下文恢复保持文档/相机；按§9.4记录系统拼音与物理触控板未测。 | `cargo test -p vellum`137+5；fmt/Clippy通过；全部11个spec逐文件80/80、0失败/跳过/重试，其中smoke连续36/36；互开3页202层；两次release构建43文件SHA-256相同；文档路径MISSING 0、49个Markdown链接有效、`git diff --check`通过。证据：`test-results/vellum/m8-verification.json`、`m8-build-determinism.json`、`m8-performance.json`与`docs/validation/vellum/m8.md` | `dirty@a92caa0b5d57fc35c4a6c227c73096025107b5cd`；`examples/vellum/Cargo.toml`及document/keys/index、M8/visual/M6浏览器脚本、`.github/workflows/verify.yml`、README、`docs/vellum.md`、architecture/quickstart及验收报告 |

## 0. 需求、范围与决策

### 0.1 需求与约束账本

| ID | 类型 | 来源 | 内容 | 设计/验收落点 | 状态 |
| --- | --- | --- | --- | --- | --- |
| R-1 | 功能 | 用户「clone 这个项目到 examples 里」 | `examples/vellum` 复刻 Vellum 的编辑能力：工作区（亮/暗、页面、可搜索图层树、折叠、隐藏/锁定、命令面板、快捷键、网格、标尺、缩放平移）、几何（矩形/椭圆/框架/直线/多边形与贝塞尔路径、旋转、画布缩放手柄、多选、框选、微移、对齐、分布、编组、层级与顺序）、排版（多行 Unicode、换行、字族/字号/字重、斜体、下划线/删除线、行高、字距、对齐、大小写、LTR/RTL、导入字体）、外观（纯色/线性渐变、描边、不透明度、圆角、投影、嵌套圆角框架裁剪）、布局（横/纵自动布局与 gap/padding/交叉轴对齐；左右居中拉伸缩放 + 上下约束）、组件（主组件、实例、传播、覆盖；按钮/卡片/徽章快速插入）、设计令牌（颜色与排版令牌、颜色令牌重映射、JSON 导出）、文件（IndexedDB 防抖保存 + localStorage 回退、`.vellum`、图片放置、PNG 与 SVG 导出）、预览（框架演示、前后导航、点击原型链接）、历史（80 步撤销重做、拖拽事务边界、可撤销的整文档替换） | §5、§6、M2–M7 | 已确认 |
| R-2 | 视觉 | 用户「视觉差异要尽可能小」+ Vellum `index.html`/`styles.css`/`renderer.js` | 同一台机器、同一浏览器、同一视口、同一文档与相机状态下，本示例与 Vellum 的截图差异尽可能小：外壳逐字移植标记与样式；场景渲染逐行移植 Vellum 的着色器与光栅策略；文本/路径/图片用与 Vellum 相同的浏览器 Canvas 2D 光栅。度量：`tests/browser/support.ts` 的 `differingPixels`（通道和阈值 24）占比，参照为 `ref/Vellum-main` 以 `?canvas` 运行（A-6） | ADR-1、D9、§6、§9.2、NFR-5 | 已确认 |
| R-3 | 功能 | Vellum README「Files」 | `.vellum` v1 JSON 与 Vellum 双向互开：Vellum 导出的文件本示例能开且层数一致；本示例导出的文件 Vellum 的 `DocumentModel.parse` 能开；未知字段原样保留 | §3、M1、M7 | 已确认 |
| R-4 | 功能 | `ref/Vellum-main/tests/smoke.py` | `window.vellum` 自动化面让 36 项检查逐项移植（§9.2 表），一次会话内全绿并留下亮/暗截图 | §4.2、M8 | 已确认 |
| R-5 | 功能 | Vellum `makeStarter` | 启动文档 Forma：三页，活动页 171 层、设计系统页 31 层、Playground 0 层（Node 实测总计 202 层；紧凑 JSON 为 123,320 UTF-16 码元、123,513 UTF-8 字节，格式化金样为 212,113 字节） | M1、M2 | 已确认 |
| R-6 | 交互 | 用户「交互控制是核心需求」+ Vellum `app.js` | 指针、滚轮、触控、键盘的语义与 Vellum 相同：同阈值（手柄 7 px、钢笔拖出手柄 3 px、移动判定 2 px、吸附 5/zoom、命中容差 4/zoom、钢笔闭合 8 px）、同修饰键、同事务边界（按下 begin、抬起 commit、Escape/取消/失焦 cancel）。度量：孪生对照——同一指针脚本在两边执行后文档几何（x/y/w/h/rotation、点数）一致 | ADR-2、§5、§9.2 | 已确认 |
| R-7 | 文档 | 用户「说明我们用了框架的什么功能来实现的，要体现框架的价值」 | `docs/vellum.md`：Vellum 五层 → 框架对应物的映射表（带文件路径）；每项能力三列：框架提供了什么、应用写了什么、裸 wasm + 手写 HTML/JS 要另写什么；可核实的数字（`app.js` 行数、应用自己注册的 DOM 监听数、构建体积、帧账本）；框架缺的与本期补的；有意不走框架的部分及理由；孪生对照结果。每个里程碑追加自己那一节 | M1–M8 退出条件、§9.6 | 已确认 |
| NFR-1 | 性能 | Vellum 自检项 35 | 5,000 形状页面可渲染、可选择、可编辑；帧账本与场景 CPU 时间只在页内测量并记录，本期不设门（没有来源数字可作门） | §7、M2 | 已知 |
| NFR-2 | 体积/启动 | `docs/reports/p3/` 只有 B0/B2/B3 的门 | 记录 `build-manifest.json` 的六类体积与页内冷启动时间；不设门 | §7、M8 | 未知（不改变决策） |
| NFR-3 | 安全 | `xtask/src/serve.rs:36` 默认 `Strict` CSP | 在 `style-src 'self'; script-src 'self' 'wasm-unsafe-eval'; img-src 'self' data:` 下无一条 CSP 违规：无 `style=` 属性、无内联脚本 | §6、§7、M5 | 已知 |
| NFR-4 | 可达性 | 仓库惯例 | DOM 侧每个控件有可达名称（沿用 Vellum 的 aria）；画布对象不进可达树，边界同 data-workbench（`docs/reports/p3/known-limitations.md`） | §6 | 已知 |
| NFR-5 | 视觉一致门 | R-2 | 三页启动文档 fit-all 亮/暗各一张、以及 M4/M6 的手势后截图，与孪生参照的差异像素占比 ≤ **2%**（1600×1000，RGB通道差之和>24计差异）。用户于2026-09-16在M3后确认，M8执行全部启动页与手势截图断言 | §9.2、§11 | 已确认 |
| C-1 | 约束 | 用户「不要跑 rustify-ui 自己的测试」+ 2026-09-16 答复「允许」过滤单测 | 实施期**禁止**：`cargo test --workspace`、无过滤的 `cargo test -p rustify-ui/rustify-components/rustify-makepad/xtask`、`makepad/` 下任何 `cargo test`、`npm run test:browser`、根 `playwright.config.ts` 的任何 `--project`、`cargo xtask verify`。**允许**：`cargo test -p vellum`；`tests/vellum/playwright.config.ts`；静态检查（`cargo fmt --all -- --check`、`cargo clippy -p vellum --all-targets -- -D warnings`、改到分叉时 `cargo xtask sources verify`）；改了 SDK 某模块时**只**允许 `cargo test -p <crate> --lib <模块>::` 这一条过滤运行 | §9.5、每个里程碑退出条件 | 已确认 |
| C-2 | 约束 | 用户「可以修改 rustify-ui 代码」+ `sources.lock.json`/`xtask/src/sources.rs` | 可改 `crates/`；改 `makepad/` 任一文件必须手工更新 `sources.lock.json` 该文件的 sha256（`sources verify` 只校验）并在 `docs/compatibility.md` 登记；SDK 改动向后兼容 | D11 | 已确认 |
| C-3 | 约束 | `rust-toolchain.toml`、`Cargo.toml`、`xtask/src/build.rs:96-165` | nightly-2026-05-20 + wasm32；示例目录须有 `Cargo.toml`、`index.html`、`app.js`、`app.css`，可选 `vendor/`；`Cargo.toml` 不提 `rustify-components` 则不复制 `rustify.css` | §1.2 | 已确认 |
| C-4 | 约束 | `makepad/platform/src/os/web/web_gl.js:1318`、`docs/compatibility.md:52`、`web.rs:28` | 区域只有 WebGL2；嵌入模式下区域不收键盘；无像素读回 | ADR-1、ADR-2 | 已确认 |
| C-5 | 约束 | `ref/Vellum-main/LICENSE`（MIT） | 移植的标记、样式、着色器、图标、启动文档与测试步骤保留版权声明：`examples/vellum/LICENSE-VELLUM` 原样收录 | M2 | 已确认 |
| C-6 | 约束 | `docs/architecture.md`「Controlled values」 | 控件受控：用户动作是请求，显示的是应用答复；拒绝的值被换回生效值 | §6 | 已确认 |
| C-7 | 约束 | 记忆（run-browser-projects-one-at-a-time、full-sweep-only-at-milestone-exit、performance-numbers-must-be-measured-in-page） | 浏览器测试一个命令一个 spec 文件、输出 `tee` 到文件；整套只在里程碑退出跑；性能数字页内测 | §9 | 已确认 |
| C-8 | 约束 | 用户「不是直接用 wasm 输出一堆 html 和 js」 | 应用逻辑 100% 在 Rust；`app.js` 只做 boot、实例通知/重启与自动化门面（与 `examples/data-workbench/app.js` 同形）；`index.html` 只有挂载点；运行时不加载任何 Vellum 的 JS，Vellum 的 JS 只在 `tests/vellum` 里作为孪生参照 | §1.2、D9、R-7 | 已确认 |
| A-1 | 假设 | Vellum 的裁剪链在片元里按存储缓冲区逐祖先求交；Makepad DSL 有带 `break` 的 `loop`（`makepad/draw/src/shader/draw_text.rs:997-1004` 的 `scan_vertical_list`）、`texture_2d(float)` 与 `sample2d`（`makepad/widgets/src/image.rs:22`），纹理格式有 `VecRGBAf32`（`makepad/platform/src/texture.rs:165`） | 裁剪表可放进一张 float 纹理，实例只带链头索引，片元按链循环采样求交——与 Vellum 同构 | M2 探针；分支：实例内联 2 条记录（各 9 float），更深的链在 CPU 侧把节点 AABB 与更远祖先的裁剪矩形求交后再画，README 记差异 | 已验证（M2 浏览器像素探针通过，无需分支） |
| A-2 | 假设 | Vellum 在渲染循环里同步光栅并上传（`renderer.js` `Atlas.get`） | 缩放跨 2 的幂边界时可见的文本/路径/图片全部重光栅并上传（每条一张纹理）在页内一帧可接受 | M3 页内测量；分支：分帧上传，未到的先画旧分辨率 | 已测：首次 CPU 光栅 31.7–36.6 ms；已采用分帧分支（8 ms 批次、未完成沿用旧分辨率），浏览器验证通过 |
| A-3 | 假设 | Vellum 管线用预乘 `one, one-minus-src-alpha`；分叉 `web_gl.js:1078` 有 `blendFuncSeparate` | 分叉的混合因子与之相同 | M2 读 `web_gl.js:1078` 并用半透明叠加截图核验；分支：着色器按分叉的因子调整输出 | 已验证（M2 浏览器像素探针通过，无需分支） |
| A-4 | 假设 | `crates/rustify-ui/src/text.rs:64` `TextEdit`原先只按轴对齐`LocalRect`放置 | 已增加可选`transform: Signal<String>`（CSSOM），旋转/相机浏览器检查与旧property-workbench wasm编译通过 | M6 | 已验证 |
| A-5 | 假设 | 金样需要 Vellum 自己的代码生成 | Node 26 能加载 `ref/Vellum-main/src/document.js` | 已验证（`node v26.1.0`：3 页 171/31/0 层，`parse` 通过） | 已解除 |
| A-6 | 口径 | 无头 Chromium 无可靠 WebGPU；Vellum 自述 WebGPU 路径的文本/路径/图片本就是 Canvas 2D 光栅（README「Rendering architecture」） | 以 `?canvas` 运行的 Vellum 作为视觉参照；M3 有头对照确认解析 SDF 与 Canvas 2D 在边缘和阴影上存在后端差异，场景差异另行记录 | M3 时在有头 Chrome 上额外对照一次 WebGPU 路径并记录 | 已验证（有头 WebGPU / Canvas 六场景测量与目视对照） |

### 0.2 决策表

| # | 决策点 | 选择 | 含义/影响 | 依据 |
| --- | --- | --- | --- | --- |
| D1 | 形态 | 一个 workspace 示例 crate `examples/vellum`（bin），一个 `rustify_ui::mount` 作用域，一个 `GpuRegion`；无路由 | 与 `examples/data-workbench` 同构；不改 loader、不改 xtask | C-3、C-8 |
| D2 | 场景渲染 | Vellum `renderer.js` 的 WebGPU 设计逐行移植进区域（ADR-1）：WGSL → Makepad DSL；`text/path/line/image` 走浏览器 Canvas 2D 光栅成纹理 | 同一套数学、同一套光栅引擎 → 视觉差异只剩抗锯齿与浮点 | R-2、C-4 |
| D3 | 输入与覆盖层 | Vellum 的 `#overlay` 2D canvas、`pointerDown/Move/Up`、`hitTest`、`drawOverlay`、`drawRulers` 1:1 移植到 DOM 侧（ADR-2）；区域是纯渲染器 | 手势、光标、手柄、参考线、标尺与 Vellum 逐像素同源；区域可随时重建 | R-6、C-4 |
| D4 | 状态归属 | 文档、历史、选择、工具、选项、相机全部在 Leptos 信号；区域 props = `Arc<Frame>` + 相机 + 跳过的节点；actions 只有 `Stats` | 区域不持有任何业务状态 | `docs/architecture.md`「Props and actions」 |
| D5 | 文本编辑 | `rustify_ui::TextEdit`（多行）盖在文本层的区域矩形上并带 `transform`（A-4）；区域跳过该层 | 光标、选择、IME、系统撤销是浏览器的；与 Vellum 的 contenteditable 同为浏览器原生文本 | `text.rs`；Vellum `startText/finishText/skipId` |
| D6 | 导出/演示 | Canvas 2D 画家 = D2 光栅的同一份 `paint_node`（Vellum `paintScene` 的移植） | 屏幕与导出用同一份代码画文本/路径/图片 | ADR-1；`web.rs:28` 无读回 |
| D7 | SVG 导出 | 纯 Rust 字符串生成（`svg.js` 移植），文本行由同一份 `layout_text`（Canvas 2D `measureText` 移植）切分 | 与 Vellum 的 SVG 文本行一致 | Vellum `svg.js` |
| D8 | 持久化 | IndexedDB `vellum-editor/documents/current` + localStorage 回退；`vellum-options`、`vellum-welcomed`；示例内用 web-sys 实现 | 同键名；只有一个调用方，不进 SDK | Vellum `openDB/save`；三次再抽象 |
| D9 | DOM 外壳 | Vellum `index.html` 的标记与 `styles.css` 逐字移植（选择器作用域到 `.vellum`）；浮层（主菜单/上下文菜单/缩放菜单/模态/命令面板/演示层）用 SDK `Layer` 承载 Vellum 的标记；表单控件是原生元素、受控；**本示例不依赖 `rustify-components`** | 每一处组件重样式都是一处视觉差异；框架价值在核心层（区域、覆盖层栈、文本会话、主题、文件、剪贴板、实例模型）而非组件库，`docs/vellum.md` 如实写 | R-2、C-8；`overlay.rs:502` `Layer` 只渲染一个容器 |
| D10 | 图层树渲染 | 转义 HTML 字符串 `inner_html` + 容器上一个委托监听（Vellum `renderLayers` 的做法） | 5,000 层不是 5,000 组 JS 引用；CSP 安全 | 记忆 externref-table-is-a-ceiling |
| D11 | SDK 改动最小集 | `TextEdit`可选`transform`（A-4）；修复`files::pick`在选择/取消后的监听与闭包释放（M7浏览器计数先红）；不碰`makepad/` | 两处API向后兼容；分别执行text/files过滤单测与示例浏览器验证，`sources.lock.json`不动 | C-2 |
| D12 | 命名与页面面 | 包名 `vellum`、`vellum.wasm`；`window.vellum`（Vellum 语义的自动化面）与 `window.__vellum`（仓库惯例的实例/诊断面）；隐藏的 `#status[data-status]` | 既能移植 smoke 检查，又沿用 `tests/browser/support.ts` 的 ready 约定 | D12 |
| D13 | 字体 | 与 Vellum 相同的 CSS 字体栈 `Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Arial, sans-serif`，不捆绑字体；用户字体经 `FontFace` + `document.fonts.add`（Vellum 同法） | 同机同浏览器下字形完全一致，含符号与非拉丁回退 | R-2；`renderer.js` `FONT_STACK`、`app.js` `loadStoredFonts` |
| D14 | 图片资产 | 浏览器 `Image` 解码 data URL（Vellum 同法），`drawImage` cover 布局进光栅 | 含 `svg+xml` 资产 | `renderer.js` `ImageStore/imageFit` |
| D15 | 测试布局 | 独立配置 `tests/vellum/playwright.config.ts`：project `vellum`（`cargo xtask serve --example vellum --release --port 4179`）；孪生参照 `python3 -m http.server 4180 --directory ref/Vellum-main --bind 127.0.0.1`，`ref/` 不存在时孪生 spec 自跳过；根 `playwright.config.ts` 与 `xtask verify` 不动 | C-1 可执行；CI 没有 `ref/`，孪生只在本机跑 | C-1、C-7 |
| D16 | CI | `.github/workflows/verify.yml` web job 加 `build-web --example vellum --release` 与 `npx playwright test -c tests/vellum/playwright.config.ts`（用户已同意） | 主干回归覆盖示例 | 2026-09-16 答复 |
| D17 | 键盘 | 全部快捷键在 DOM 侧 `document` 级监听（`isInput` 守卫、演示层/模态优先），1:1 移植 Vellum 的表 | 区域收不到键盘 | C-4；Vellum `keydown` 监听 |
| D18 | 区域尺寸 | 区域画布随 `.canvas-area` 变化由 SDK 处理；覆盖画布用 `rustify_ui::observe_resize` 同步宽高与 dpr | 两块画布同尺寸同 dpr | `crates/rustify-makepad/src/wasm/host.rs:163` |
| D19 | Leptos 的用法边界 | 文档/历史/选择/工具/相机/选项是作用域内的信号；外壳（顶栏、页面、检查器、工具栏、菜单与模态内容）是信号驱动的 `view!` 树，检查器按选中节点派生、输入 `prop:value` 受控、`on:input` live / `on:change` 提交、`class:`/`style:` 切状态；投影与覆盖画布重画是效应；异步走 `spawn_local`。**不把 Vellum 的 `innerHTML` 拼接搬进 Rust**：只有图层树保留整块渲染（D10），指针状态机与覆盖画布绘制是命令式代码，其输出（文档变更、选择集）才是信号。受控数字/十六进制输入在聚焦期间保留本地草稿，失焦或回车才取信号值——逐字段实现 Vellum「聚焦时不重绘检查器」的规则，而不是跳过整个面板 | 检查器不再需要 Vellum 的 80 ms 节流与整段重建；`docs/vellum.md` 写清响应式与命令式各占哪一层 | R-7、C-8；记忆 externref-table-is-a-ceiling；Vellum `renderInspector` 的聚焦守卫 |

### 0.3 ADR-lite

#### ADR-1：区域里跑的是 Vellum 渲染器的移植，不是 Makepad 的绘图库

- 状态：Accepted
- 背景与驱动：R-2 要视觉差异尽可能小；Vellum 的 WebGPU 渲染器是 128 字节实例 + 一次实例化绘制 + 四层光栅图集（文本/路径/图片由 Canvas 2D 光栅），形状用解析 SDF；C-4 只有 WebGL2、无读回。
- 备选：
  - A（选）：逐行移植。顶点函数：六顶点展开、按描边/阴影外扩 `pad`、仿射矩阵旋转、相机相对平移（CPU f64 rebase 后转 f32）；片元：`sdRound`、椭圆近似、渐变 `mix`、描边覆盖、阴影（`props.y=0, params.y=blur*.5` 的软边）、裁剪链循环、预乘输出。文本/路径/直线/图片在应用侧用 web-sys Canvas 2D 按 Vellum 的 `Atlas.get` 逻辑（padding、分辨率 `clamp(2^ceil(log2(zoom*dpr)), .5, 4)`、尺寸上限）光栅，作为 RGBA 像素随 `Frame` 投给区域，区域用 `ImageBuffer::new(rgba, w, h).into_new_texture(cx)` 上传，每条一张纹理并按键缓存。两种着色器：`VellumShape`（解析）与 `VellumRaster`（采样），按绘画顺序提交，Makepad 把连续同着色器同纹理的实例合并成一次绘制。
  - B：用 Makepad 自己的 `DrawText`/`DrawVector`/`Sdf2d` 画。淘汰：Makepad 的 shaper 与浏览器换行点、字距（布局器没有）、字体回退、符号字形都不同；`DrawVector` 的接头/抗锯齿与 Canvas 2D 不同——每一项都是 R-2 的差异源。
  - C：保留 Vellum 的 `renderer.js` 作为 vendor JS。淘汰：违反 C-8。
- 决策：A。证据：`makepad/draw/src/shader/draw_rotated_text.rs:87` 顶点旋转是分叉在用的模式；DSL 有 `loop`/`break`/`sample2d`（A-1 的出处）；`image_cache.rs:182` `ImageBuffer::new` 直接接受 RGBA 字节；分叉 `web_gl.js:1078` 有 `blendFuncSeparate`（A-3 核验因子）。
- 正面后果：形状与 Vellum 同一套数学；文本/路径/图片与 Vellum 同一光栅引擎；屏幕、PNG 导出、演示、SVG 文本行共用一份画家；5,000 形状仍是一次绘制。
- 负面/中性后果：像素在应用侧与 GPU 侧各一份（应用侧缓存 64 MiB 上限，与 Vellum 图集同量级；GPU 侧只留当前帧引用的键）；每条光栅一张纹理而非图集，绘制次数随文本层数增长（启动页约百余次，WebGL2 可承受，记录数字）；引擎徽章写「Makepad WebGL2」。
- 重新评估触发：分叉获得部分纹理更新（可回到单图集）；M3 测得纹理上传超帧预算（A-2 分支）。

#### ADR-2：输入在 DOM 侧 1:1 移植，区域不参与

- 状态：Accepted
- 背景与驱动：R-6 要手势语义相同；Vellum 的指针逻辑建立在 DOM 事件模型上（`setPointerCapture`、`pointercancel`、`dblclick`、`contextmenu`、`wheel` 的 `preventDefault`、多指 `pointerId`）并用 Canvas 2D 的 `isPointInPath/isPointInStroke` 命中路径；C-4 区域收不到键盘。
- 备选：
  - A（选）：一块与区域同尺寸的 DOM `<canvas>`（Vellum 的 `#overlay`）盖在区域上，接收全部指针事件；`app.js` 的 `pointerDown/Move/Up`、`hitTest`、`smartSnap`、`drawOverlay`、`drawPen`、`drawRulers` 移植为 Rust（web-sys）；区域只画场景。
  - B：区域接收 Makepad `Hit` 事件并回传手势动作（本计划上一版）。淘汰：路径命中没有 `isPointInStroke`；指针捕获、`pointercancel`、双击/右键语义要重新发明；覆盖层图形若由区域画，每处线宽/字体/圆角都是差异源。
  - C：区域画覆盖层、DOM 收输入。淘汰：两套坐标换算，覆盖层视觉仍非同源。
- 决策：A。仓库自己的原则支持它：意义在 DOM、像素在区域（`region.rs` 的 `aria-hidden` 注释；`docs/compatibility.md:52`）。
- 正面后果：交互与视觉的覆盖层都与 Vellum 同源；区域是纯函数式渲染器。
- 负面/中性后果：平移/缩放走「DOM 事件 → 信号 → props → pump → 绘制」，比 Vellum 的 rAF 多一段；P3 已让一次投影在同一帧呈现（`docs/architecture.md`「Presentation」），B1 测得同类路径输入延迟 45.5 ms。M4 页内测量并记录。
- 重新评估触发：M4 测得平移的呈现延迟 p95 > 50 ms → 相机改为区域本地（`scene_view.rs` 的 `pan_to` 模式），其余不变。

#### ADR-3：外壳原样移植、浮层站在 SDK 覆盖层栈上，不用组件库

- 状态：Accepted
- 背景与驱动：R-2；Vellum 的检查器、菜单、模态是自有标记 + 自有 CSS；`rustify-components` 的结构与 `rui:` 类不同。
- 备选：A（选）Vellum 标记与样式逐字进 Leptos，浮层用 `Layer`（Escape 顺序、初始聚焦、焦点归还、模态 inert 是栈的；Tab首尾循环由应用补充）；B 用 `Menu/Dialog/CommandPalette/Select/…` 并重样式。淘汰 B：每一处结构差异都要用 CSS 追平，追不平的就是差异；组件的语义（命令面板每个词都要匹配、不可用命令留在列表）也与 Vellum 不同。
- 决策：A。`docs/vellum.md` 明确写：本示例站在 SDK 核心层上，组件库是可选的上层。
- 正面后果：视觉同源；`Layer`提供Escape顺序、初始聚焦、关闭后焦点归还与背景inert。应用保留原版Tab首尾循环；toast通过覆盖层根的Portal展示，不入Layer栈，以免截获Escape。
- 负面/中性后果：组件库的可达性与键盘规范由 Vellum 的标记自行满足（Vellum 已带 aria）。
- 重新评估触发：用户要求示例展示组件库。

### 0.4 职责与事实所有权

|  | 浏览器 / loader | `examples/vellum`（本期） | `rustify-ui` | Makepad 分叉 |
| --- | --- | --- | --- | --- |
| 拥有 | 实例化与重启上限、CSP、IndexedDB/localStorage 实际存储、文件选择与下载、剪贴板、Canvas 2D 光栅与文本测量、字体加载 | 文档与 `.vellum` 格式、历史、选择、工具与选项、相机、指针状态机、命中、覆盖画布、光栅缓存、自动布局/约束/组件同步、令牌、导入导出、自动化面、Vellum 视觉 | 区域生命周期与重试、props/actions 顺序与 `Pace`、覆盖层栈（`Layer`/`TextEdit`）、主题令牌、文件与剪贴板入口、resize 观察、诊断 | WebGL2 上下文、我们的两种着色器的编译与绘制、纹理上传、DPI |
| 不拥有 | 应用状态 | **不在区域里放业务状态；不复制 SDK 的覆盖层/文件/剪贴板** | 任何 Vellum 规则；不新增通用「画布编辑器」抽象 | 文本 shaping、矢量细分、输入（本示例不用） |

一句话：文档、规则、输入与光栅在示例手里，投影与浮层的秩序在 SDK 手里，像素合成在分叉手里，存储、文件与字体在浏览器手里。

### 0.5 明确不在本期

- **WebGPU 后端、`?canvas` 主渲染回退开关**——不做（C-4）；参照对照用 Vellum 的 `?canvas`（A-6）。
- **`build.py` 单文件版、`scripts/`、Pages 工作流、Python 测试工具**——不移植；Vellum JS 只作孪生参照。
- **Vellum 自己声明的边界**（协作/CRDT、评论、插件、`.fig`、布尔运算、GPU 路径细分、混合模式、框架以外的遮罩、组件变体与结构传播、自动布局的换行/hug/fill、富文本、OpenType 特性、变量字体轴、文本沿路径、SVG 资产内部路径导入）——同样不做。
- **移动端密集编辑**——与 Vellum 同边界；捏合缩放按 Vellum 的 `pointers/pinch` 移植但不进自动化门。
- **图层树虚拟化、场景空间索引**——Vellum 没有。
- **把画布对象放进可达树**——同 data-workbench 边界（NFR-4）。
- **组件库展示**——ADR-3；另立示例。

## 1. 当前事实与改动面

### 1.1 现状与缺口

- **已核实·足够**：示例目录约定与构建（`xtask/src/build.rs:96-165`）；输出 `target/makepad-wasm-app/release/<name>/`；`serve` 校验 manifest 的 base。
- **已核实·足够**：区域契约（`crates/rustify-makepad/src/wasm/app.rs:12`；`crates/rustify-ui/src/region.rs` 三次重试、`Lost` 重建并重投影、`Suspended`、`refused`）；`Pace`；一次投影同帧呈现（`docs/architecture.md`「Presentation」）；`observe_resize`（`host.rs:163`）；`stats().frames/gpu_bytes` 账本。
- **已核实·足够**：`Layer{modal, anchor, on_close, labelled_by, described_by, class}` 渲染一个 `rustify-layer` 容器，`role=dialog|presentation`，模态时 scope inert、聚焦首元素、关闭归还焦点（`overlay.rs:502-620`）；`Anchor::{Region, Element, Centred}`；`TextEdit{anchor, value, multiline, on_commit, on_cancel, on_invalidated, class}`（`text.rs:64`）。
- **已核实·足够**：`files::{Limits, import_files, export}`、`clipboard::{copy, paste}`（`build-web` 打开 unstable API）、`ThemedScope` 在作用域根写 `data-theme`、`diagnostics::report_json`。`files::pick`选择/取消后的闭包释放缺口由M7补齐，不改变调用签名。
- **已核实·足够**：Makepad 可承载 ADR-1——`draw_rotated_text.rs:87` 顶点旋转；DSL 带 `break` 的 `loop`（`draw_text.rs:997-1004`）、`texture_2d(float)`/`sample2d`（`image.rs:22-48`）、`VecRGBAf32`/`VecBGRAu8_32` 纹理（`texture.rs:135-170`）；`ImageBuffer::new(rgba,w,h)` 与 `into_new_texture`（`image_cache.rs:182,257`）；`add_aligned_instance` 合批（`draw_quad.rs:116-140`）；`web_gl.js:1078 blendFuncSeparate`（因子待 A-3 核验）。
- **已核实·足够**：web-sys 提供 Canvas 2D 全套（`CanvasRenderingContext2d`、`Path2D`、`TextMetrics`、`ImageData`、`CanvasGradient`、`HtmlImageElement`、`FontFace`）与 IndexedDB 类型，只需在示例 `Cargo.toml` 开 features（`crates/rustify-ui/Cargo.toml` 同法）。
- **已核实·缺口**：turtle 裁剪只有轴对齐（`turtle.rs:374,1566`）→ A-1 裁剪表纹理 → 漏做后果：旋转/圆角框架裁剪不精确。
- **已核实·缺口**：无读回（`web.rs:28`）→ D6。
- **已核实·缺口**：区域不收键盘（`docs/compatibility.md:52`）→ D17。
- **已核实·缺口**：`TextEdit` 无变换（`text.rs`）→ A-4。
- **已核实·不用**：`DrawText`/`DrawVector`/`Sdf2d` 高层辅助、`rustify-components`、`rustify_ui::Selection`（`u32` ID，Vellum 是字符串）、`drag`（Vellum 图层树用 HTML5 DnD，DOM 内部）——理由见 ADR-1/ADR-3，`docs/vellum.md` 记录。

### 1.2 拓扑与文件清单

★ 为新增。

- `Cargo.toml`：`members` 加 `"examples/vellum"`。
- ★ `examples/vellum/Cargo.toml`：依赖 `leptos`、`rustify-ui`、`serde`、`serde_json`；wasm 目标加 `rustify-makepad`、`wasm-bindgen`、`wasm-bindgen-futures`、`js-sys`、`web-sys`（features：`Window, Document, Element, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d, Path2D, CanvasGradient, TextMetrics, ImageData, HtmlImageElement, DomMatrix, FontFace, FontFaceSet, Storage, IdbFactory, IdbOpenDbRequest, IdbDatabase, IdbTransaction, IdbObjectStore, IdbRequest, Blob, BlobPropertyBag, Url, File, FileList, FileReader, KeyboardEvent, PointerEvent, WheelEvent, MouseEvent, DragEvent, DataTransfer, ResizeObserver, Performance, CssStyleDeclaration`）。**不依赖 `rustify-components`**。
- ★ `examples/vellum/index.html`：`<head>` 按仓库约定（`runtime.css`、`app.css`、`loader.js`/`bindgen.js` 预加载、`app.js` 模块；`title`/`meta`/`favicon` 取 Vellum 的），`<body>` 只有 `<div id="vellum">`、隐藏的 `#status`。
- ★ `examples/vellum/app.js`：`boot`、致命通知与重启、`window.__vellum`、`window.vellum` 门面（§4.2）。
- ★ `examples/vellum/app.css`：`styles.css` 逐字移植，根选择器 `.vellum`，`html[data-theme]` → `.vellum[data-theme]`；`[hidden]{display:none!important}`；`.rustify-layer` 归零定位样式以便 Vellum 的 `.context-menu/.modal-backdrop/.presentation` 规则接管。
- ★ `examples/vellum/LICENSE-VELLUM`（MIT 原文）；★ `examples/vellum/README.md`（运行/测试命令、差异清单）。
- ★ `examples/vellum/src/`：
  - 纯 Rust（host 也编译，单测在此）：`document.rs`、`starter.rs`、`affine.rs`、`scene.rs`（`Frame` 与裁剪链）、`history.rs`、`layout.rs`、`commands.rs`、`hit.rs`（几何命中、手柄、框选、吸附；路径命中经注入的 `PathTester` trait）、`gesture.rs`（十种手势公式）、`svg_export.rs`、`text_layout.rs`（`layoutText/displayText/fontSpec` 的算法，测量器经 `trait Measure` 注入）、`raster_policy.rs`（分辨率量化、padding、尺寸上限、缓存预算）。
  - wasm：`main.rs`（挂载、导出）、`region.rs`（`RegionApp`：props 应用、纹理缓存、绘制、`Stats`）、`shaders.rs`（`script_mod!`：`VellumShape`、`VellumRaster`）、`raster.rs`（Canvas 2D 光栅缓存：文本/路径/直线/图片）、`painter.rs`（`paint_node/paint_scene`、`export_canvas`、`measure`）、`overlay.rs`（覆盖画布：`draw_overlay/draw_pen/draw_rulers`、尺寸与 dpr）、`pointer.rs`（`pointer_down/move/up/cancel`、捏合、滚轮、双击、右键）、`keys.rs`、`shell/{topbar,left_panel,layer_tree,pages,assets,inspector,prototype,toolbar,zoom,menus,dialogs,palette,toast,presentation,text_session}.rs`、`storage.rs`、`assets.rs`（图片解码）、`fonts.rs`（字节源 `FontFace` 与挂载作用域缓存）、`icons.rs`、`automation.rs`。
- ★ `tests/vellum/playwright.config.ts`、★ `tests/vellum/support.ts`（`waitForReady/waitForQuiet/capture/differingPixels` 的本地副本，不 import 根测试）、★ `tests/vellum/twin.ts`（Vellum 参照页驱动：`window.vellum` 同名 API）、★ `tests/vellum/fixtures/forma.vellum`、★ `tests/vellum/interop.mjs`、★ `tests/vellum/m{2..8}-*.spec.ts`、★ `tests/vellum/smoke.spec.ts`、★ `tests/vellum/visual.spec.ts`。
- ★ `docs/vellum.md`（R-7）；`docs/architecture.md` 示例表加一行；`docs/quickstart.md` 加构建命令一行。
- `.github/workflows/verify.yml`：web job 加两步（D16）。
- SDK最小改动：`crates/rustify-ui/src/text.rs`（A-4）与`files.rs`选择/取消清理（D11）。
- **无需修改的邻接面**：`web/loader.js`、`web/runtime.css`、`xtask/`、`makepad/`、`sources.lock.json`、根 `playwright.config.ts`、`tests/browser/`、既有三个示例、`crates/rustify-components`。

## 2. 模块、接口与依赖

| 模块 | 调用者 | 接口与不变量 | 接缝 | 隐藏复杂度 | 依赖分类 | 测试面 |
| --- | --- | --- | --- | --- | --- | --- |
| `document.rs` | 全部 | `Document::parse(&str) -> Result<Document, ParseError>` 执行 Vellum `DocumentModel.parse` 全部规则；`serialize()` 输出 `format:"vellum",version:1`，未知字段 `#[serde(flatten)]` 回写；`page/nodes/get/children/descendants/ancestors/roots/remove/add/touch/refresh`；`revision` 单调 | 无 | ID 语法、上限、循环与深度、资产 scheme | 进程内 | host 单测 |
| `scene.rs` | 区域投影、命令、命中、导出 | `compose(&Document) -> Frame`：绘画顺序、世界矩阵与逆、AABB、继承不透明度/隐藏/锁定、裁剪链；`bounds(ids)`；`world(id)` | 无 | 一次算好，四处共用 | 进程内 | 单测 |
| `history.rs` | 事务 | `begin` 幂等、`commit` 无变化不入栈、80 上限、`cancel`、`undo/redo` 返回标签；资产/字体 `Rc<str>` 浅拷贝 | 无 | 整文档替换可撤销 | 进程内 | 单测 |
| `layout.rs` / `commands.rs` / `gesture.rs` | 外壳、指针、键盘、自动化 | 命令走 `transaction(label, …)`：`finish_text → history.begin → 变更 → apply_all_layouts → sync_components → history.commit → save_soon`；手势公式纯函数（Vellum `pointerMove` 各分支） | 无 | 顺序即 Vellum `changed({commit:true})` | 进程内 | 单测：Vellum 检查 4/5/9/10/11–13/16–21 的数字 |
| `hit.rs` | `pointer.rs`（同步） | `hit_test(frame, world, deep, frames_only, tester)`、`handles_for`、`marquee`、`smart_snap`；`trait PathTester { fn in_path(&self, node, p) -> bool; fn in_stroke(&self, node, p, width) -> bool }`，wasm 实现用 Canvas 2D `isPointInPath/isPointInStroke`，单测用折线近似 | `PathTester` | Vellum 的容差与顺序 | 进程内 | 单测 + 孪生对照 |
| `text_layout.rs` | 文本层高度、光栅、SVG | `layout_text(node, &impl Measure) -> TextLayout{lines, line_height, height, baseline}`：段落、`\s+|\S+` 分词、字素回退（`Intl.Segmenter` 有则用，否则按字符）；`display_text`（大小写）；`font_spec` | `Measure`（wasm：Canvas 2D `measureText` 含 `letterSpacing`） | 与 Vellum `layoutText` 同算法 | 进程内 | 单测（固定宽度测量器） |
| `raster.rs` | `region.rs` 投影、`painter.rs` | `RasterCache::get(node, res, images) -> Option<Arc<Raster{key, rgba, w, h, padding, width, height}>>`：键 `id:version:res`；padding = 路径/直线的 `strokeWidth/2+1`；`scale = min(res, (2044)/max(w,1), …)`；LRU 预算 64 MiB；图片未解码返回 `None`（Vellum 同：本帧不画，解码完 `invalidate`） | 无 | Vellum `Atlas.get` 的策略，减去图集打包 | 外部（浏览器 Canvas 2D） | 浏览器 spec：视觉对照 |
| `painter.rs` | 导出、演示、光栅 | `paint_node(ctx, node, images)`（Vellum `paintScene` 单节点分支）、`paint_scene(ctx, frame, included)`、`export_canvas(ids, scale)`（64 MP / 16,384 守卫） | 无 | Canvas 2D 状态机 | 外部 | 检查 28/29/30 |
| `region.rs` + `shaders.rs` | `GpuRegion` | `Props{frame: Arc<Frame>, rasters: Arc<Vec<Arc<Raster>>>, camera: Camera, skip: Option<Id>}`；`Action::Stats{visible, instances, draw_items, cpu_ms}`（`Pace::Continuous("stats")`）；`apply_props`：上传新键纹理、丢弃不在本帧的键、按绘画顺序提交实例；裁剪表写入 float 纹理（A-1） | 着色器脚本 | 实例打包与相机 rebase | 本地可替代（可重建） | 浏览器 spec：`vellum.renderer.*` |
| `overlay.rs`（示例） | `pointer.rs`、效应 | `draw_overlay(ctx, state, frame, camera, theme)`：框架名、悬停轮廓、选择轮廓与手柄、尺寸标签、框选、参考线、钢笔、路径编辑、标尺——Vellum `drawOverlay/drawPen/drawRulers` 逐行 | 无 | 与 Vellum 同字体同线宽 | 外部 | 视觉对照 |
| `pointer.rs` | 覆盖画布事件 | `pointer_down/move/up/cancel`、`wheel`、`dblclick`、`contextmenu`、`pointerleave`、失焦——Vellum 逐行；`setPointerCapture` | 无 | 十种手势 | 进程内 | 检查 4–10、22；孪生对照 |
| `shell/*` | 用户 | 信号驱动的 `view!`（D19）；受控；`input` 事件 live、`change` 提交；聚焦期间字段持本地草稿；非法 hex toast 并回滚 | 无 | Vellum `renderInspector` 分支表 | 进程内 | 检查 3/8/31–34 |
| `storage.rs` | 保存/启动 | `load() -> Option<String>`、`save(json)` 500 ms 防抖、失败指示 | 无 | IndexedDB 回调 | 浏览器本地 | 刷新恢复；注入失败 |
| `automation.rs` + `app.js` | Playwright | §4.2 | 无 | 快照 JSON | 进程内 | `smoke.spec.ts` |

删除测试：去掉 `scene.rs`，区域/命中/导出各算一次层级；去掉 `text_layout.rs`，光栅/SVG/高度三处各切一次行；去掉 `painter.rs`，光栅与导出各画一遍——都不是能删的。没有新增 SDK 抽象。

## 3. 数据模型与迁移

`.vellum` v1（与 Vellum 完全一致，出处 `ref/Vellum-main/src/document.js` 的 `node()`/`parse`）：

| 对象 | 关键字段与约束 | 生命周期/权威来源 | 备注 |
| --- | --- | --- | --- |
| 文档 | `format:"vellum"`、`version:1`、`name`、`pageId`、`pages[]`（1–100）、`tokens{colors[],typography[]}`、`assets{id: data:image/(png|jpeg|webp|gif|svg+xml);…}`、`fonts{family: data:…}` | 内存权威在 `Document`；IndexedDB 是防抖副本；`.vellum` 是可携副本 | `pageId` 无效则取第一页 |
| 页 | `id`（`^[-_a-zA-Z0-9]{1,128}$`）、`name`、`nodes[]`（绘画顺序，全文档 ≤ 50,000） | — | — |
| 节点 | `id`（同上，全文档唯一）、`type ∈ {rect,ellipse,text,frame,group,path,line,image}`、`name`、`parentId`（同页、无环、深度 ≤ 100）、`x,y,w,h,rotation,opacity`（有限、绝对值 ≤ 1e7，`w,h ≥ 0`）、`fill/fill2/fillType/gradientAngle/fillOpacity/stroke/strokeWidth/radius/visible/locked/clip/shadow*/version`；文本 `text`（≤ 100,000）、`fontFamily/fontSize/fontWeight/fontStyle/lineHeight/letterSpacing/textAlign/textDecoration/textCase/direction`；路径 `points[{x,y,in?,out?}]`（≤ 50,000）、`closed`、`pathW/pathH`；图片 `assetId`；布局 `layout/gap/padding/layoutAlign`、`constraintH/constraintV`；组件 `component/isInstance/sourceId/overrides{}`；原型 `prototypeTarget` | 解析时按 `node(type, props)` 补默认值 | 未知字段保留（R-3） |

- 序列化 `serde_json` 缩进2并启用`float_roundtrip`；数值语义精确保留（含旋转/路径小数及未知字段），不承诺JSON键顺序或字符串逐字节与JS一致；`tests/vellum/interop.mjs` 用 Vellum 的 `parse` 验证而非逐字节比对。
- 浏览器存储：IndexedDB 库 `vellum-editor` v1、store `documents`、键 `current`；localStorage `vellum-document`、`vellum-options`、`vellum-welcomed`。

### 3.1 兼容与迁移

无既有数据、无 schema 变更；回滚 = 删除示例目录与成员行。

## 4. 集成与契约

### 4.1 SDK、分叉与浏览器

| 依赖 | 当前状态 | 行为级证据 | 本期处理 | 失败语义 |
| --- | --- | --- | --- | --- |
| `GpuRegion`/`RegionApp`/`Pace` | 已核实足够 | `region.rs`；`scene_region.rs:pace` | 零改动；流名 `stats` | `Failed` → 徽章「GPU unavailable」+ `role=alert`，其余功能可用；`Lost` → 徽章「Restoring」，恢复后重投影 |
| `Layer` | 已核实足够 | `overlay.rs:502-620` | 零改动；Vellum 的 `.context-menu/.modal-backdrop+.modal/.presentation/命令面板` 标记放进 `Layer`（菜单 `Anchor::Element`，模态 `Anchor::Centred` + `modal`） | Escape 顺序与焦点归还是栈的 |
| `TextEdit` | 已核实缺口 | `text.rs:64` | 精确增量：可选 `transform`（A-4） | 值被外部改 → `on_invalidated` → 取消事务 |
| `files`/`clipboard` | 已核实，选择器清理由D11补齐 | §1.1 | `files::pick`释放监听与闭包，其余API不变；`Limits`：`.vellum` 与 `.json` 80 MB，图片 25 MB，字体 15 MB；粘贴优先系统剪贴板 JSON，回退内部剪贴板，纯文本粘成文本层 | `Refused` → toast；剪贴板不可用 → 内部剪贴板 |
| `ThemedScope`/`use_theme_values` | 已核实足够 | `theme.rs` | 零改动；`data-theme` 驱动 `app.css`；覆盖画布的强调色按 Vellum 的两套常量 | — |
| `observe_resize` | 已核实足够 | `host.rs:163` | 覆盖画布尺寸 | — |
| Makepad 着色器/纹理/合批 | 已核实足够（A-1/A-3 待核验） | §1.1 | 两种着色器 | — |
| 浏览器 Canvas 2D / `Image` / `FontFace` / IndexedDB | 已核实足够（web-sys 类型存在） | `crates/rustify-ui/Cargo.toml` 的 feature 用法 | 示例 `Cargo.toml` 开 features | Canvas 2D 不可用 → 无此浏览器（Vellum 同前提） |

### 4.2 自动化面（对 Playwright 的外部契约）

`app.js` 在 `boot` 后发布：

- `window.__vellum`：`{hooks, instance, mount(), dispose(), diagnostics(), live_regions(), errors(), stats()}`；`#status[data-status]` 由 `starting/ready/failed/fatal` 驱动。
- `window.vellum`（Vellum 语义）：`ready`、`version: "0.1.0"`；`doc`（每次访问取 `vellum_snapshot()`：`nodes`、`page{name,id}`、`data{name,pages,pageId,tokens,assets(仅键),fonts(仅键)}`、`get(id)`、`world(id)`、`serialize()`）；`state`（`selection: Set`、`camera`、`tool`、`gesture`、`editing`、`pathEdit`）；`renderer`（`backend: "Makepad WebGL2"`、`instanceCount`、`visibleCount`、`drawCalls`、`cpuMs`、`gpuError`、`width`、`height`、`dpr`、`exportCanvas(ids, scale) -> Promise<HTMLCanvasElement>`）；`select(ids)`、`fit(ids?)`、`setTool`、`setProperty(prop, value)`、`transaction(label, patches: [{id, prop, value}])`、`createAtCenter(type, props) -> node`、`insertAsset`、`instantiate`、`doExport`、`save`、`importDocument(File) -> Promise`、`importImage(File, at?) -> Promise`、`exportSVG(ids) -> string`、`parse(text)`、`actions.<name>()`（Vellum `actions` 全部键）、`render()`。
- DOM 钩子：标记保留 Vellum 的 `id` 与 `data-*`（`#overlay`、`#dismiss-tip`、`[data-page]`、`[data-layer]`、`[data-tool]`、`input[data-prop]`、`#theme-toggle`、`#presentation`、`#presentation-canvas`、`#close-presentation`、`#toast`、`#file-name`、`#zoom-value`、`#command-search`、`#modal-backdrop`）；`TextEdit` 用 `test_id="text-editor"`（smoke 的 `#text-editor` 改为 `[data-testid=text-editor]` 一处）。
- 孪生参照：`tests/vellum/twin.ts` 在 `http://127.0.0.1:4180/?canvas` 上等待 `window.vellum.ready`，两边用同一段脚本驱动。

## 5. 核心机制

- **主流程（一次编辑）**：① 入口调用 `Editor` 方法；② `finish_text`；③ `history.begin(label)`；④ 改 `Document` 并 `touch`；⑤ 提交路径 `apply_all_layouts → sync_components → history.commit → save_soon`，live 路径只 `touch` 并把图层树/检查器刷新节流 80 ms（输入聚焦时不重绘检查器）；⑥ `revision` 或相机变化 → 效应重算 `Arc<Frame>`、按 `raster_policy` 求当前分辨率并从 `RasterCache` 取可见光栅、投给区域；同一效应重画覆盖画布；⑦ 区域绘制并回报 `Stats`；性能行 `${visible} layers · ${cpuMs} ms CPU` 与 Vellum 同格式（`cpuMs` = 投影构建 + `apply_props` 的页内计时）。
- **文本层高度**：`set_property` 改排版属性、`finish_text`、缩放手势——按 Vellum 的位置调用 `layout_text(node, measure)` 写 `h`；测量器是 Canvas 2D `measureText`，同步。
- **指针（ADR-2）**：覆盖画布 `pointerdown` → `setPointerCapture`；Vellum 的分支顺序原样：右键忽略 → 关菜单 → 记录 pointer → 双指捏合 → 结束文本 → 平移（中键/空格/手型）→ 路径编辑手柄 → 钢笔 → 文本工具 → 绘制工具 → 手柄 → 命中（shift 切换、alt 复制、⌘深选）→ 框选。`pointermove` 分支：捏合、无手势时的光标与悬停、平移、钢笔手柄、路径编辑、框选（实时选择集）、创建（shift 正方/alt 中心/直线角度）、移动（shift 锁轴、吸附）、缩放（shift 等比、alt 对称、单/多选）、旋转（shift 15°）。`pointerup`：创建 < 3 px 给默认尺寸、路径归一化、实例覆盖记录、提交或刷新 UI。`pointercancel`/失焦：取消事务。滚轮：⌘/Ctrl/Alt 缩放（`exp(-clamp(dy,-100,100)*.012)`）、shift 横向、否则平移；模态开着时忽略。
- **相机**：`fit(ids?)`、`zoomAt`、切页记忆——Vellum 公式；相机是应用状态，投给区域。
- **键盘（D17）**：`document` 级；演示层 → Escape/←/→；模态 → `Layer`处理Escape、背景inert与关闭后焦点归还；应用按原版补Tab/Shift+Tab首尾循环（SDK未内置循环，M5核实）；`isInput` → 只拦 ⌘S；其余按 Vellum 的表。空格状态影响光标与平移。
- **组件同步与实例化**：`sync_components` 按 Vellum 的省略键集与 `overrides`；跨页实例化复制子树重映射 ID。
- **文本会话**：`editing = Some(id)` → 区域 `skip`；`TextEdit`锚`Anchor::Region(canvas, LocalRect(0,0,w,h))`，由SDK设置canvas视口原点与未变换尺寸；`transform`为`matrix(a·z,b·z,c·z,d·z,e·z+cam.x,f·z+cam.y)`、原点`0 0`，避免AABB重复偏移。应用用CSSOM设置`fontSpec/lineHeight/letterSpacing/textAlign/textDecoration/textTransform/direction/color/opacity`；提交按`finishText`写`text/name/h/overrides`；Escape/⌘Enter提交；从画布以Enter重开时取消该键默认动作，防止新聚焦的textarea收到换行并覆盖已选原文；外部改动使会话失效时丢弃草稿与待提交事务，保留外部值。
- **Canvas读回策略**：显式`willReadFrequently=true`固定软件光栅，避免浏览器按读回次数切换抗锯齿。M7探针确认文字装饰端点仅4/120,000像素（0.003333%）与原版GPU Canvas不同；强制GPU虽消除差异却使冷缩放细化从约0.1秒增至约3秒，未采用。文字布局严格比较；裁图与整页按用户≤2%门验收并记录实测，不将最初的0差异观测当作额外硬门。
- **光栅缓存**：键 `id:version:res`；缩放跨 2 的幂边界时可见条目重光栅（A-2 测量）；`undo/redo/import/font load/reset` 清缓存（Vellum `resetRasterCaches`）；图片解码完成 → `invalidate`。
- **并发**：单线程；文件与图片解码为 `spawn_local` 任务，图片与字体完成时应用当前页。整文档替换在读取前记录revision/page，期间若文档变更或切页则拒绝晚到结果并提示重试，避免覆盖新编辑；所有入口先检查挂载作用域仍存活。导出用独立图片缓存，演示按代次只提交最新画面。

## 6. 前端与交互

- 信息架构、状态、主题、可达性：与上一版一致，但标记与样式逐字取自 Vellum：顶栏、左栏、画布区（区域画布 `#scene` 位置 + 覆盖画布 `#overlay` + 顶线 + 提示/性能 + 浮动工具栏 + 缩放控件 + 欢迎提示 + `#canvas-world` 点网格背景）、右栏、上下文菜单、toast、模态、演示层。
- 视觉对照方法（NFR-5）：`tests/vellum/visual.spec.ts` 在 1600×1000 视口、`deviceScaleFactor 1` 下，两边 `resetStarter → select([]) → fit()`，隐藏 toast，截图整页；再切页、切亮/暗各截一张；`differingPixels` 占比 `console.log` 并写 `test-results/vellum/visual.json`；M8 按门值断言。区域徽章文字（「Makepad WebGL2」vs「Canvas 2D fallback」）与性能行数字是已知差异，对照前把两者置为相同占位文本（自动化面 `vellum.actions.blankBadges()`，仅测试用）。
- 真实浏览器走查（M8，有头 Chrome，记录到 `docs/validation/vellum/m8.md`）：亮/暗、拼音输入法编辑文本层、图层拖拽与 Shift 重父、触控板捏合、演示层键盘、WebGPU 路径的 Vellum 与本示例并排目视对照（A-6）。

## 7. NFR、安全与运行保障

| ID | 基线/来源 | 目标或未知项 | 超限/失败行为 | 机制 | 验证 |
| --- | --- | --- | --- | --- | --- |
| NFR-1 | Vellum 检查 35 | 5,000 形状 `visibleCount==5000`、可编辑；页内记录 `cpuMs` p95、`stats().frames`、`drawCalls==1` | 无门 | 视口剔除 + 单着色器合批 | `m2-scene.spec.ts` |
| NFR-2 | 未知 | 记录体积与冷启动 | 无门 | — | M8 |
| NFR-3 | Strict CSP | 0 条违规 | 测试失败 | Leptos `style:` 指令与 CSSOM；`inner_html` 只含转义文本与 class/data 属性 | 每个 spec 监听 console 的 CSP 消息 |
| NFR-4 | 仓库惯例 | 控件有可达名称 | — | Vellum 标记已带 aria | `getByRole` |
| NFR-5 | R-2 | 差异像素占比 ≤ 2%（用户已确认） | 测试失败 | ADR-1/2/3 | `visual.spec.ts` |

安全：导入只解析不求值；名称/文本进入 DOM 只经转义或文本节点；ID 白名单；资产 scheme 白名单；文件大小先于读取拒绝。可观测性：`diagnostics()`、`vellum.renderer`、`__vellum.stats()`。

## 8. 失败模式、发布与回滚

| 失败/触发 | 爆炸半径 | 数据后果 | 用户表现/降级 | 检测 | 恢复/补偿 | 验证 |
| --- | --- | --- | --- | --- | --- | --- |
| GPU 上下文三次拿不到 | 场景画布 | 无 | 徽章「GPU unavailable」+ alert；覆盖画布、面板、导入导出、PNG/SVG 仍可用 | `RegionState::Failed` | 用户重载 | `m2`：`?nogpu` 跳过区域挂载演练降级 |
| 上下文丢失 | 场景画布 | 无 | 徽章「Restoring」；恢复后重投影，纹理由缓存重新上传 | `Lost` | SDK 自动 | 手工 `WEBGL_lose_context`（M8） |
| 实例 trap | 整个实例 | 未保存编辑丢失；IndexedDB 保留上次保存 | loader 通知 + 重启入口；重启后从存储恢复 | `on_fatal` | 重启 ≤ 3 | `m7`：`enter_fatal` 模拟 |
| 存储满/不可用 | 保存 | 未落盘 | 指示器红 + toast | 事务 `onerror` | 用户导出 | `m7` 注入 |
| 导入非法文件 | 无 | 无 | toast；文档不变 | `ParseError` | — | 检查 26/27 |
| 图片/字体过大或不可解码 | 无 | 无 | toast | `Refused`/`onerror` | — | `m7` |
| 光栅上传超帧预算 | 一帧 | 无 | 掉帧 | M3 页内计时 | A-2 分支 | `m3` |
| 光栅缓存增长 | 内存 | 无 | — | 缓存字节数 + `gpu_bytes` | LRU 64 MiB；纹理只留本帧键 | `m3` 20 级缩放后有界 |
| 动作队列拒绝 | 一次 `Stats` | 无 | 无（只是统计流） | `refused` | — | 观察为 0 |

发布顺序：示例代码 → workspace 成员 → CI 步骤。回滚：删除示例及测试目录、workspace成员行、CI两步与新增文档入口；`TextEdit.transform`与`files::pick`清理修复向后兼容，可单独保留。

## 9. 验证

### 9.1 行为与接口验证（host 单测，`cargo test -p vellum`）

`document.rs`：金样解析 → 3 页、202 层、活动页 171；每条拒绝规则一个用例；未知字段往返；`serialize→parse` 相等。`affine.rs`/`scene.rs`：绕中心旋转、逆矩阵、旋转 30° 后 `box.w > w`、裁剪链只含 `frame && clip`、隐藏节点不进 `items` 但进 `world`。`history.rs`：无变化不入栈、81 次截断、`cancel`、撤销整文档替换恢复资产键。`layout.rs`：横 `16/116, y=16`；纵 `y=76, x=16`；`right` 约束 +100。`commands.rs`：分组世界矩阵不变、解组、复制 ID 唯一、顺序四向、对齐六向、分布 ≥ 3、组件传播/覆盖/跨页实例化、令牌重映射、粘贴偏移 24。`hit.rs`：手柄 7 px、深选择、锁定跳过、椭圆边界、框选包含、吸附 5/zoom 与参考线轴。`gesture.rs`：创建 shift/alt、移动 shift 锁轴、缩放 shift/alt、单/多选缩放、文本随缩放改字号、旋转 shift 15°、直线角度吸附、路径编辑 alt 断对称。`text_layout.rs`：固定宽度测量器下的段落/换行/字素回退/高度/基线；`display_text` 三种大小写。`raster_policy.rs`：分辨率量化、padding、尺寸上限、预算淘汰顺序。

### 9.2 浏览器验证（`npx playwright test -c tests/vellum/playwright.config.ts <spec>`）

Vellum 36 项检查 → spec 映射（编号按 `ref/Vellum-main/tests/smoke.py`）：

| # | 检查 | 落点 | 适配 |
| --- | --- | --- | --- |
| 1–2、35 | 初始化 171/3 页；实例数 > 150 且 backend ∈ {Makepad WebGL2}；5,000 形状 | M2 `m2-scene` | backend 集合 |
| 15 | 排版属性绑定 | M3 `m3-raster` | 原样 |
| 4,5,6,7,9,10 | 矩形工具 180 宽；拖动 +50；撤销；重做；手柄到 300；旋转包围盒 | M4 `m4-pointer` | 原样（`#overlay` 保留） |
| 3,8,31,32,33,34 | 切页；`w=240`；亮；命令面板 rulers；暗；1280 无溢出 | M5 `m5-shell` | 原样 |
| 11,12,13,14,16–22 | 复制；分组；解组；多行 Unicode 编辑；组件三项；自动布局两项；约束；钢笔 | M6 `m6-edit` | 19 用 `transaction(label, patches)`；`#text-editor` → `[data-testid=text-editor]` |
| 23–30 | 图片导入；撤销替换恢复资产；往返；循环拒绝；注入拒绝；SVG；PNG 550×250；演示 | M7 `m7-files` | `parse`/`exportSVG(ids)` |
| 36 | 无未捕获错误 + 0 CSP 违规 | 每个 spec | — |
| 全部 | 一次会话原顺序 36 项 + 亮/暗截图 | M8 `smoke.spec.ts` | — |

孪生对照（`ref/` 存在时）：`m4-pointer`/`m6-edit` 的每个指针脚本在 Vellum（`?canvas`）与本示例上各跑一遍，比较 `doc.nodes` 的 `x/y/w/h/rotation/points` 与 `state.selection`（容差 0.5 px / 0.5°）；`visual.spec.ts` 按 §6 截图并记录差异占比。互开：`m7` 打开金样 → 202 层；导出 → `interop.mjs` 解析通过。

### 9.3 NFR 与故障注入

`m2-scene` 页内记录 5,000 形状的 `cpuMs` p95、`frames`、`drawCalls`；`m3-raster` 记录缩放跨界的重光栅耗时与缓存字节；§8 的注入用例在 `m2`/`m7`。

### 9.4 真实环境与 UI

M8 有头 Chrome 走查 §6 六项，写 `docs/validation/vellum/m8.md`；无法执行的记「未测」。

### 9.5 静态校验

`cargo fmt --all -- --check`；`cargo clippy -p vellum --all-targets -- -D warnings`；`cargo xtask build-web --example vellum --release`（双构建逐字节一致）。**不跑** C-1 的禁止项。

### 9.6 文档验证（R-7）

`docs/vellum.md` 每节的文件路径经 `.claude/skills/dev-plan/scripts/check_paths.sh docs/vellum.md .` 核对无 MISSING；数字（`app.js` 行数、应用注册的 DOM 监听数、体积、帧账本、孪生对照结果）每个都有生成它的命令或 spec 文件名。

## 10. 里程碑（每步可独立验收）

| # | 里程碑 | 内容 | 验证/退出条件 |
| --- | --- | --- | --- |
| M1 | 文档模型与规则（纯 Rust） | crate 注册；Node 生成金样；`document/starter/affine/scene/history/layout/commands/hit/gesture/text_layout/raster_policy/svg_export` 及单测；`make_starter` 与 Vellum 逐层同构；`docs/vellum.md` 骨架（章节：五层映射表、逐能力三列表、数字、补的与不走框架的、对照结果） | `cargo test -p vellum` 全绿且覆盖 §9.1；`interop.mjs` 解析 `starter` 序列化通过并报 202 层；clippy/fmt；`docs/vellum.md` 写入 M1 节（模型层为何不需要框架）；回写「实施进度」 |
| M2 | 区域渲染器与场景骨架 | `index.html/app.js/app.css/LICENSE-VELLUM`；`mount` + `ThemedScope` + `#status`；`shaders.rs`（WGSL 逐行移植：顶点外扩/相机 rebase/`sdRound`/椭圆/渐变/描边/阴影/裁剪链/预乘）、`region.rs`（实例打包、纹理缓存、`Stats`）；A-1 裁剪表探针、A-3 混合核验；`raster.rs` 首版（文本/路径/直线/图片，Vellum `Atlas.get` 策略）；覆盖画布骨架（框架名标签）；`fit/zoomAt`；`window.vellum` v1；`?nogpu` 降级；`tests/vellum` 配置与孪生服务；首张视觉对照（启动页 fit-all 亮） | `m2-scene.spec.ts`：检查 1、2、35；`drawCalls==1`（5,000 形状）；20 级缩放 `gpu_bytes` 有界；`visual.spec.ts` 记录首个差异占比；`build-web` 成功；A-1/A-3 结论进 §11 与 README；`docs/vellum.md` M2 节（`GpuRegion` 提供了什么：生命周期、重试、`Lost` 重投影、`stats` 账本、同帧呈现）；回写「实施进度」 |
| M3 | 光栅保真、文本布局与图片 | `text_layout.rs` 的 wasm 测量器；`display_text/fontSpec`；装饰、方向、字距、大小写；`FontFace` 字体加载与 `loadStoredFonts`；图片解码与 cover；缓存预算与淘汰；缩放跨界重光栅计时（A-2）；三页启动文档亮/暗对照；有头 Chrome 上与 Vellum WebGPU 路径目视对照（A-6） | `m3-raster.spec.ts`：检查 15；`visual.spec.ts` 三页 × 亮/暗差异占比记录并提请用户定 NFR-5 门值；A-2/A-6 结论回写；`docs/vellum.md` M3 节（为何光栅走浏览器而不是 Makepad 文本；`ImageBuffer` 上传路径）；回写「实施进度」 |
| M4 | 指针状态机与覆盖画布 | `pointer.rs` 十种手势逐行移植；`overlay.rs` 全部图形（悬停、选择、手柄、尺寸标签、框选、参考线、钢笔、路径编辑、标尺）；光标表；吸附；捏合；画布键盘子集（工具键、撤销重做、Escape、空格、⇧1/2/0、Delete、方向键、[ ]、+/-）；`pointercancel`/失焦取消；平移呈现延迟页内测量 | `m4-pointer.spec.ts`：检查 4、5、6、7、9、10；孪生对照：六个手势脚本两边几何一致；平移 p95 记录（> 50 ms 触发 ADR-2 重评）；`docs/vellum.md` M4 节（覆盖画布为何在 DOM；`observe_resize`）；回写「实施进度」 |
| M5 | 工作区外壳 | Vellum 标记逐字进 Leptos：顶栏、左栏（页签、搜索、页面列表与记忆、图层树 D10 含展开/多选/锁定/可见/双击改名/拖拽重排 + Shift 重父、资产面板）、检查器全部分区、浮动工具栏、缩放控件与菜单、欢迎提示、上下文/主/缩放菜单（`Layer`）、模态（`Layer modal`：改名/新建/帮助/设置/导出/令牌/CSS）、命令面板（`Layer modal`，Vellum 语义）、外壳键盘表、主题切换、`panels-hidden/left-hidden`、toast | `m5-shell.spec.ts`：检查 3、8、31、32、33、34；`getByRole` 定位；0 CSP 违规；`visual.spec.ts` 加外壳对照（菜单打开、模态打开）；`docs/vellum.md` M5 节（`Layer` 栈：Escape 顺序、焦点、inert；`ThemedScope`）；回写「实施进度」 |
| M6 | 编辑操作、组件与文本会话 | 撤销/重做 toast、复制/粘贴、复制、删除、分组/解组/框架化、顺序、对齐/分布、组件、自动布局与约束 UI、令牌编辑器与导出、快速插入、预设、`TextEdit` 会话（A-4）、钢笔与路径编辑、Inspect CSS、改名 | `m6-edit.spec.ts`：检查 11–14、16–22；孪生对照（钢笔、路径编辑、文本编辑后几何）；`cargo test -p rustify-ui --lib text::`（C-1 允许的一条）；`docs/vellum.md` M6 节（`TextEdit`：IME/系统撤销/失效语义；`clipboard`）；回写「实施进度」 |
| M7 | 持久化、导入导出与演示 | `storage.rs`；`.vellum` 导入/导出；图片放置（选择器与拖放）；字体导入；PNG 画家；SVG；演示层（`Layer`）；压力测试与恢复启动文档 | `m7-files.spec.ts`：检查 23–30；互开双向；§8 存储与 trap 注入；`docs/vellum.md` M7 节（`files`、实例重启后恢复）；回写「实施进度」 |
| M8 | 收尾：全套、视觉门、走查、文档与 CI | `smoke.spec.ts`；`visual.spec.ts` 按用户定的门值断言；亮/暗截图；README；`docs/vellum.md` 完稿（数字、对照结果、结论）；`docs/architecture.md`/`docs/quickstart.md` 各加一行；`verify.yml` 两步；`docs/validation/vellum/m8.md` | `smoke.spec.ts` 36/36；`visual.spec.ts` 全部 ≤ 门值；`tests/vellum` 每个 spec 逐文件各跑一遍全绿（`tee` 到 `test-results/vellum/*.log`）；双构建一致；静态检查全过；§9.6 通过；回写「实施进度」 |

依赖链 M1 → M2 → M3 → M4 → M5 → M6 → M7 → M8。M2/M3 的探针失败不阻塞后续，按 §0.1 各假设的分支执行并改正文。

## 11. 风险、开放问题与就绪状态

| 项目 | 影响 | 责任人/解除办法 | 最晚确认点 | 是否阻塞 |
| --- | --- | --- | --- | --- |
| A-1 裁剪表纹理 | 视觉（旋转/圆角裁剪精度） | 已解除：M2 四层祖先圆角裁剪像素探针通过；使用完整浮点纹理链 | M2 完成 | 否 |
| A-2 重光栅上传预算 | 缩放时掉帧 | 已测首次光栅 31.7–36.6 ms、总 CPU 约 37 ms；执行 8 ms 批次，真实绘制后续批，未完成沿用有效旧图；首次呈现15–21.6 ms、2/4/6帧细化，已验证 | M3 | 否 |
| A-3 混合因子 | 半透明叠加视觉 | 已解除：ONE/ONE_MINUS_SRC_ALPHA；预乘像素/着色器/canvas 上下文，叠色实测 [70,6,135] | M2 完成 | 否 |
| A-4 `TextEdit.transform` | SDK 改动 | M6已验证旋转/相机定位及旧调用方编译兼容 | 已验证 | 否 |
| A-6 `?canvas` 参照口径 | 对照有效性 | 已测量：Chrome 152.0.7977.84 / apple metal-3；原版 WebGPU 对原版 Canvas 六场景差异 0–2.4268%，目视确认边缘与阴影差异；报告 `m3-webgpu.json` | M3 | 否 |
| NFR-5 门值 | 验收口径 | 用户已于2026-09-16确认每张≤2%；M8执行断言 | 已确认 | 否 |
| 平移呈现延迟 | 手感 | M4 页内 p95；> 50 ms 走 ADR-2 重评 | M4 | 否 |
| 每条光栅一张纹理的绘制次数 | 性能 | M2 启动页实测 144 次绘制、173 实例；5,000 纯形状为 1 次 | 已记录 | 否 |
| 5,000 层图层树 DOM 体量 | 内存 | D10 | M5 | 否 |

- 最终状态：**Ready**。
- 定级理由：范围、契约、验证与里程碑闭环；用户三条答复已落成 C-1（过滤单测允许）、D16（CI）、R-2/R-6/NFR-5/ADR-1–3（视觉与交互对标）与 R-7（文档）；开放假设均有已核实分支；NFR-5的门值已由用户于2026-09-16确认每张≤2%。

## 12. 已知坑与历史教训

- 浏览器测试一个命令一个 spec 文件、`tee` 到文件、不并发；整套只在里程碑退出跑（记忆 run-browser-projects-one-at-a-time、full-sweep-only-at-milestone-exit）。
- 时间数字页内测（记忆 performance-numbers-must-be-measured-in-page）。
- externref 表上限：一个元素一组 JS 引用；无消息 `unreachable` 先读 debug 栈（记忆 externref-table-is-a-ceiling）→ D10。
- 内存增长先当缓存；判有界前先走一遍工作集（记忆 growth-is-a-cache-until-proven-otherwise）→ 光栅缓存的 20 级缩放先走完再读。
- 脚本改源先 `assert old in src`（记忆 assert-before-replacing-source-text）。
- 区域首帧后字体到达再呈现一次，软光栅上约 6 s（`tests/browser/support.ts` `waitForQuiet`）；本示例区域不画字，但 `waitForQuiet` 仍保留。
- `serve` 校验 manifest 的 base，换 `--base` 必须重建。
- CSP `style-src 'self'`：`style="…"` 属性会被拒 → Leptos `style:` 指令与 CSSOM（`TextEdit.transform` 同理）。
- 嵌入模式区域不收键盘（`docs/compatibility.md:52`）。
- `ref/` 被 `.gitignore` 忽略：孪生与视觉 spec 在 `ref/Vellum-main` 缺席时必须 `test.skip` 而不是失败；CI 没有它。
- `document.js` 在 Node 下按 ES module 加载会有 `MODULE_TYPELESS_PACKAGE_JSON` 警告，无害；`interop.mjs` 用 `--input-type=module` 或 `import()`。
- Vellum 的 `renderInspector` 在输入聚焦时不重绘（避免打断输入）；移植时保留，否则 live 输入会被换回。
- Vellum 的 `pasteSelection` 先读系统剪贴板再回退内部；`navigator.clipboard.readText` 在无头浏览器需要权限，spec 里授予 `clipboard-read`。

## 13. 需求 → 设计 → 验证映射

| ID | 需求/约束/假设 | 设计落点 | 验证/解除办法 | 结果 |
| --- | --- | --- | --- | --- |
| R-1 | 十个领域 | §5、M2–M7 | §9.1 + 检查 1–35 | 覆盖 |
| R-2 | 视觉差异尽可能小 | ADR-1、ADR-3、D2/D9/D13/D14、§6 | `visual.spec.ts` + NFR-5 | 覆盖 |
| R-3 | 互开 | §3、M1/M7 | 金样 + `interop.mjs` | 覆盖 |
| R-4 | 36 项 | §4.2、§9.2、M8 | `smoke.spec.ts` | 覆盖 |
| R-5 | 启动文档 | `starter.rs` | 单测 + 检查 1 | 覆盖 |
| R-6 | 交互同语义 | ADR-2、D3、§5 | 孪生对照（M4/M6） | 覆盖 |
| R-7 | 框架价值文档 | `docs/vellum.md`，逐里程碑 | §9.6 | 覆盖 |
| NFR-1 | 5,000 形状 | §7、M2 | 检查 35 + 记录 | 覆盖（无门） |
| NFR-2 | 体积/启动 | M8 | 记录 | 覆盖（无门） |
| NFR-3 | CSP | §7、D10 | CSP 监听 | 覆盖 |
| NFR-4 | 可达 | §6 | `getByRole` | 覆盖 |
| NFR-5 | 视觉门 | §6、M3/M8 | 每张≤2%（用户已确认） | 覆盖 |
| C-1 | 不跑 SDK 测试（过滤例外） | §9.5、里程碑 | 命令清单 | 覆盖 |
| C-2 | SDK 改动登记 | D11 | 不碰分叉 | 覆盖 |
| C-3 | 示例约定 | §1.2 | `build-web` | 覆盖 |
| C-4 | WebGL2/无键盘/无读回 | ADR-1/2、D6 | — | 覆盖 |
| C-5 | MIT | `LICENSE-VELLUM` | 文件存在 | 覆盖 |
| C-6 | 受控 | §2 | 检查 8 | 覆盖 |
| C-7 | 测试纪律 | §9、§12 | — | 覆盖 |
| C-8 | 逻辑在 Rust | §1.2、D9 | `app.js` 行数进 `docs/vellum.md` | 覆盖 |
| A-1 | 裁剪表 | ADR-1 | M2 | 已验证（M2 像素探针） |
| A-2 | 上传预算 | ADR-1 | M3 | 分帧分支已验证 |
| A-3 | 混合因子 | ADR-1 | M2 | 已验证（M2 像素探针） |
| A-4 | `TextEdit.transform` | D5 | M6 | 已验证 |
| A-5 | Node 金样 | M1 | 已验证 | 已解除 |
| A-6 | `?canvas` 参照 | §6 | M3 | 已测量（保留后端差异说明） |
| — | **不在本期：WebGPU、单文件版、Pages、Vellum 自身排除项、移动端、虚拟化、可达树、组件库展示** | §0.5 | 去向已注明 | 排除 |
