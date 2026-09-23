# Vellum 示例与框架能力

本示例将 Vellum 的编辑规则移植到 Rust，用 Rustify UI 管理 DOM 与 GPU 区域之间的投影、生命周期和输入界面。实施状态以 [VELLUM 计划](plan/VELLUM.md) 为准。

最终验证通过142项Rust测试、11个独立浏览器脚本的80项测试，其中包括同一次会话连续执行的36项原版冒烟检查。六张启动页差异为0.0335%–1.651%，每张均满足用户确认的≤2%上限；菜单、模态与编辑后截图也通过。CI已加入示例构建和独立浏览器测试入口，完整证据及两项尚未人工验证的设备输入见[最终验收报告](validation/vellum/m8.md)。

## 五层映射

| Vellum 层 | Rustify UI 对应物 | 实施状态 |
| --- | --- | --- |
| [场景画布](../examples/vellum/src/region.rs) | 一个 [GpuRegion](../crates/rustify-ui/src/region.rs) 与应用着色器 | 场景、裁剪、混合、合批与内存探针通过 |
| [覆盖画布](../examples/vellum/src/overlay.rs) | DOM Canvas 2D 与 [Rust指针状态机](../examples/vellum/src/pointer.rs) | 指针状态机、全部覆盖图形与孪生验证通过 |
| [文本编辑层](../examples/vellum/src/shell/text_session.rs) | SDK [TextEdit](../crates/rustify-ui/src/text.rs) | 原生会话、变换、组合输入与事务验证通过 |
| [DOM外壳](../examples/vellum/src/shell/mod.rs) | Leptos、[ThemedScope](../crates/rustify-ui/src/theme.rs) 与 [Layer](../crates/rustify-ui/src/overlay.rs) | 受控检查器、图层树、菜单与模态验证通过 |
| [持久化](../examples/vellum/src/storage.rs) | 应用内IndexedDB/localStorage + [挂载清理](../crates/rustify-ui/src/mount.rs) | 防抖保存、回退、故障与实例重启恢复验证通过 |

## 逐能力职责

| 能力 | 框架提供 | 应用实现 | 裸 wasm + 手写 HTML/JS 需另写 |
| --- | --- | --- | --- |
| 文档模型、几何、历史 | 无；编辑业务规则不属于 UI 框架 | `.vellum` 数据校验、场景变换、编辑命令与事务历史 | 相同的业务规则与验证 |
| GPU 区域与投影 | `GpuRegion` 的创建、重试、上下文恢复、props/actions 调度、卸载；运行时帧与 GPU 字节账本 | 场景打包、Vellum 着色器、光栅纹理缓存、可见对象与绘制次数统计 | GPU 运行时接入、DOM 尺寸同步、恢复时重建和重投影、资源清理、调度与诊断 |
| 页面状态与主题 | Leptos 信号和受控视图、`ThemedScope`、作用域清理 | 文档、相机、页面选择和 Vellum 标记/样式 | 状态订阅、DOM 更新、主题传播与订阅解绑 |
| 文本/路径/图片光栅 | Makepad `ImageBuffer` 上传、区域纹理资源回收；挂载作用域的清理入口 | Canvas 2D 画家、字素换行、图片 cover、光栅 LRU、字体加载与失效规则 | 同样的浏览器测量和画家，另接纹理上传与生命周期 |
| 工作区外壳与浮层 | Leptos受控视图、SDK `Layer`栈、覆盖层根、`ThemedScope` | 原版标记/图标、输入草稿、菜单导航、模态Tab循环、命令语义 | 状态订阅与更新、浮层顺序、Escape分派、焦点归还、背景inert、主题同步与解绑 |
| 原生文本会话 | SDK `TextEdit`原生控件、组合输入优先级、会话结束去重、外部值失效；可选CSS变换 | 世界矩阵、字体样式、自适应高度、文字事务、区域跳过编辑层 | 放置控件、维护原生草稿与外部值边界、IME/Escape优先级和资源清理 |
| 系统剪贴板与令牌文件 | SDK `clipboard`结果回调、`files::export`下载 | Vellum载荷校验、内部副本回退、图层复制语义、令牌JSON | 浏览器权限拒绝处理、系统API接入与下载资源生命周期 |
| 文件导入导出 | SDK `files::pick/import_files/export`负责选择器、文件读取结果和下载 | 大小与格式限制、文档替换事务、图片解码与定位、字体嵌入、文件名与导出内容 | 文件入口、取消/拒绝回调、Blob与下载URL生命周期 |
| 持久化与实例恢复 | 挂载清理入口；loader的fatal通知、重启入口与三次上限 | IndexedDB/localStorage、提交后防抖、选项保存、恢复文档、未保存状态 | 维护实例身份、卸载旧作用域、重启界面及资源清理接线 |
| 框架演示 | SDK `Layer`的模态栈与焦点管理、尺寸观察 | 顶级框架筛选、Canvas预览、左右导航、原型命中与跳转 | 模态层次、背景inert、Escape分派、焦点归还与观察解绑 |

### 文档与规则层

[文档模型](../examples/vellum/src/document.rs)负责格式校验、跨页唯一 ID、同页父子关系、循环与深度限制，以及未知字段的往返。`serde_json`启用`float_roundtrip`，保证旋转、路径与图片坐标以及未知字段中的小数在反复保存/恢复时精确保留；连续冒烟发现的三个1 ULP漂移值有独立模型回归。[启动文档](../examples/vellum/src/starter.rs)读取由原版生成的 Forma 金样；运行时不加载原版 JavaScript。[历史](../examples/vellum/src/history.rs)保存最多 80 步完整文档快照，图片和字体字符串通过 `Rc<str>` 共享，因此替换文档后的撤销可恢复资产，又不必逐步复制二进制内容。

[场景合成](../examples/vellum/src/scene.rs)统一计算世界矩阵、逆矩阵、包围盒、继承透明度和裁剪链。[编辑命令](../examples/vellum/src/commands.rs)、命中测试与 GPU 投影共用这些结果。[文本布局](../examples/vellum/src/text_layout.rs)通过 `Measure` 接收测量结果，让 host 测试使用固定字宽、浏览器使用 Canvas 2D；[SVG 导出](../examples/vellum/src/svg_export.rs)复用同一份换行结果。

这些模块不需要 UI 框架。它们描述文件与编辑规则，必须独立于窗口、DOM 和 GPU 上下文。框架的价值体现在投影、生命周期、受控输入和浮层中，而不是接管业务模型。

验证入口：

```sh
mbx test -p vellum
mbx run -q -p vellum -- --starter > /tmp/forma-rust.vellum
node tests/vellum/interop.mjs --check-starter /tmp/forma-rust.vellum
```

互操作工具使用本地参照源码，逐项核对原版金样的已有字段，并调用原版解析器。没有 `ref/Vellum-main` 时仍可执行纯 Rust 测试。

### 区域生命周期与场景投影

[应用](../examples/vellum/src/app.rs)把文档、相机和选择保存在 Leptos 信号中。投影效应合成场景并准备浏览器光栅，通过 `SceneProps` 交给唯一的 [GPU 区域](../examples/vellum/src/region.rs)。区域返回绘制统计，不持有编辑业务状态。`Pace::Continuous("stats")` 允许框架合并同类统计通知。

[SDK 的 GpuRegion](../crates/rustify-ui/src/region.rs)负责初始化重试、`Lost` 后重建并重新应用当前 props、恢复尺寸观察和卸载清理。应用保留文档，因此GPU初始化失败或WebGL上下文丢失不会丢失编辑数据；Wasm trap仍终止整个实例，未保存编辑会丢失。SDK 的投影调度在处理 props 后同帧呈现；应用无需另建一个管理区域生命周期的 JavaScript 循环。`?nogpu` 提供可复现的无 GPU 状态，页面与文档命令继续运行。

[着色器](../examples/vellum/src/shaders.rs)与区域内的纹理保留策略属于应用：形状使用解析距离场，文本/路径/图片使用浏览器光栅；相机平移在 CPU 的 `f64` 中消去，再上传 `f32`。纹理只保留当前投影需要的键。应用统计实际 draw item，框架账本记录运行时帧数与 GPU 字节；这两个视角分别用于合批断言和缩放后的资源检查。

### 浏览器光栅与字体

[画家](../examples/vellum/src/painter.rs)通过 Canvas 2D 测量并绘制文本，用浏览器的 `Intl.Segmenter` 和 locale 大小写转换保留原版字素、换行与字体回退。路径和直线使用 `Path2D`，图片由浏览器解码后按 cover 比例绘制。屏幕光栅与导出画家复用这些规则；SDK 的 Makepad 文本整形没有接管它们，因为改变测量器会改变换行和字形。

[光栅缓存](../examples/vellum/src/raster.rs)按节点版本与量化分辨率缓存 RGBA，预算为 64 MiB；CPU 缓存淘汰和 GPU 当前帧纹理保留是不同的资源边界。区域通过 `ImageBuffer` 上传预乘像素；即使键相同，只要光栅内容对象已更换，也会更新原纹理。撤销、恢复文档和字体变化必须清理旧光栅，避免分支历史复用版本号时读到旧文字。

[字体缓存](../examples/vellum/src/fonts.rs)归属于挂载作用域。它把本地字体 data URL 解码为字节，再交给 `FontFace`，因此无需给严格 CSP 的 `font-src 'self'` 增加 data/blob 例外。文档仍保存原始 data URL，便于 `.vellum` 携带字体；卸载时移除该作用域注册的字体。字体选择改变时，文字高度仍按原版属性命令更新；恢复已存字体不会改写文档中的高度。

冷缩放曾一次光栅化 93–102 条文本，实测 CPU 光栅时间为 31.7–36.6 ms，因此采用计划的分帧分支：每批在 8 ms 后停止继续生成可延后的节点，未完成的节点使用上一分辨率；真实 GPU 绘制完成后再安排下一批。8 ms 是开始下一项工作的预算判断，单个 Canvas 操作不能中断。实测批次为 2.8–9.5 ms，冷缩放首帧为 15–21.6 ms，分别经 2、4、6 次呈现完成细化；新节点没有旧光栅时仍同步生成。所有时间由页内 `performance.now()` 与 SDK 呈现帧数共同记录。

`m3-raster.spec.ts` 的七项检查覆盖原版同浏览器 Unicode 换行、方向/字距/下划线像素、撤销分支、防 CSP 违规的本地字体、图片 cover 与同键替换、LRU 淘汰和分辨率跨界。文字区域按通道和阈值 24 比较为零差异。六个大光栅触发 104 次累计淘汰，缓存最终保留 58,867,200 字节，低于 64 MiB 预算。

后续回归发现装饰线端点出现4个差异像素。浏览器诊断确认：默认复用Canvas在第3次读回时更换绘制后端，端点alpha从127变成98/105；GPU与软件抗锯齿的边缘覆盖不同。强制GPU虽消除这4像素，却使冷缩放细化从约0.1秒增至约3秒，因此未采用。应用显式`willReadFrequently=true`固定适合RGBA读回的软件路径；文字布局仍严格相等，文字裁图采用用户确认的≤2%视觉门并输出实际差异，4/120,000仅为0.003333%。诊断与取舍证据为`test-results/vellum/m7-raster-probe.log`和`m7-raster-fixed.log`，后者为未采用的GPU方案性能记录。最终`m7-canvas-raster.log`七项通过，冷缩放0.49的细化回到101.1 ms/4帧，文字裁图实测仍为4像素。

### 指针、覆盖画布与尺寸

[指针状态机](../examples/vellum/src/pointer.rs)在DOM事件中更新Leptos信号，复用纯Rust的命中、吸附和几何规则；[覆盖画布](../examples/vellum/src/overlay.rs)按同一个场景矩阵与相机绘制选择轮廓、九个手柄、尺寸、框选、参考线、钢笔和标尺。编辑状态属于应用，GPU区域仍只接收投影。覆盖图形使用Canvas 2D，是为了保留原版的屏幕像素线宽、浏览器文字度量和指针命中坐标。

画布区通过SDK的`observe_resize`同步尺寸；DOM覆盖画布与GPU投影共用视口和DPR。框架负责尺寸观察与区域生命周期，应用负责手势语义、事务边界和覆盖图形，避免把这些编辑规则绑进渲染区域。按下指针开始历史事务，抬起提交；Escape、指针取消和活动手势中的失焦恢复原始文档。[键盘绑定](../examples/vellum/src/keys.rs)保留输入框的原生键盘行为，并处理画布工具、撤销/重做、方向键微移、缩放和空格平移。

这部分由应用显式注册9个指针/滚轮/失焦监听和2个键盘监听；`Bindings`在Leptos挂载作用域清理时移除监听并释放闭包。裸wasm同样需要这些编辑处理，还需自行管理状态订阅、视图刷新、尺寸观察与卸载顺序。`m4-pointer.spec.ts`的13项检查通过：原版矩形创建/拖动/撤销/重做/缩放/旋转；六组手势几何与选择结果一致；独立覆盖画布在RGB通道差之和>24阈值下为零差异；Escape/指针取消/失焦恢复、钢笔锚点编辑和真实CDP双指输入通过。悬停、实时框选、吸附参考线、钢笔预览和标尺另有逐像素断言。

启动页连续30次平移的输入至SDK呈现帧延迟p95为44.5 ms（最大55.6 ms），未触发ADR-2的p95>50 ms重评条件，相机继续保存在应用信号中。计时在页内完成，从指针事件分派到`requestAnimationFrame`观察到SDK帧数增加；它不是物理显示器的端到端延迟。数据见`test-results/vellum/m4-pan.json`，操作与覆盖截图见`test-results/vellum/artifacts/m4-pointer/`。

### 工作区外壳与浮层

[外壳](../examples/vellum/src/shell/mod.rs)将菜单、对话框、面板和搜索状态保存在Leptos信号中，顶栏、页面、检查器、资产、工具栏均用`view!`表达。切页保存各页相机和选择。只有[图层树](../examples/vellum/src/shell/layer_tree.rs)保留转义字符串和委托事件，以控制大文档的监听数量；名称中的HTML字符不会成为标签，缩进通过CSSOM写入，严格CSP无需放宽。

树结构字符串由Memo去重，选择变化仅更新现有行的class与ARIA，避免替换DOM打断原生双击和拖拽。`m5-shell.spec.ts`的10项检查通过，覆盖搜索、显隐/锁定、安全改名、原生拖放重父及撤销、页面相机/选择记忆、全部菜单类型、字段事务和模态操作。在当前macOS Chromium中，先按Shift会抑制原生拖动启动，原版也如此；测试先拖动8像素触发`dragstart`，再按Shift，在`drop`时断言修饰键为真，并核对六项世界矩阵不变。

[字段](../examples/vellum/src/shell/fields.rs)维护聚焦期间的本地草稿。合法数字实时更新画布，失焦提交一次历史；中间的负号、指数和空值不被响应式刷新抢写。十六进制颜色支持三位和六位，非法值恢复文档中的颜色并提示。检查器按选中节点派生，普通属性更新保留输入DOM和焦点，不需要整面板暂停刷新。

[菜单](../examples/vellum/src/shell/menus.rs)、[对话框](../examples/vellum/src/shell/dialogs.rs)与[命令面板](../examples/vellum/src/shell/palette.rs)使用SDK `Layer`。框架负责Escape关闭顺序、初始聚焦、关闭后焦点归还和模态背景inert；应用负责菜单方向键、Home/End、模态Tab首尾循环及33个原版命令的过滤与执行。SDK没有内置Tab循环，这一职责保留在应用。toast通过覆盖层根的Portal显示，避免被背景inert吞掉，同时不占用Escape栈。

`ThemedScope`向整个作用域传播亮暗主题，外壳沿用原版样式。模态在SDK覆盖层根中显示，仍继承主题。`m5-shell.spec.ts`通过可达名称检查输入、主题和菜单；焦点归还、Tab/Shift+Tab、背景inert及关闭后Layer清理均有断言。所有浏览器用例检查未捕获错误与CSP违规。

完整外壳后的指针回归13/13、场景回归6/6通过；本轮平移呈现p95为34.8 ms，5,000形状属性编辑CPU p95为6.2 ms，20次编辑均呈现且绘制次数仍为1。数据来自`test-results/vellum/m5-pointer-regression.log`和`m5-scene-regression.log`，不作为跨机器性能承诺。

### 原生文本会话、历史与剪贴板

[文本会话](../examples/vellum/src/shell/text_session.rs)使用[SDK TextEdit](../crates/rustify-ui/src/text.rs)的原生多行textarea。光标、选择、组合输入、文本内的系统撤销属于浏览器；应用保留文档、文字测量和一次会话对应的一笔历史。开始编辑时GPU跳过当前文字层，结束时恢复画布文字。Escape沿原版语义提交，Ctrl/⌘Enter与失焦也提交；从画布用Enter重开时先取消该键的默认动作，防止它落到刚聚焦的输入控件并覆盖已选原文；在新的画布手势、编辑命令或切页之前同步结束旧会话，避免两个操作混进一笔撤销。

本期SDK新增可选`transform`属性，通过CSSOM设置；没有传值的既有调用不变。锚点只提供画布视口原点和文字未变换的宽高，应用提供完整世界矩阵乘相机缩放，并设置变换原点`0 0`，因而旋转与缩放不会重复叠加AABB偏移。字体、方向、字距、装饰、透明度和输入后的高度变化仍由应用设置。

外部文字值改变时，SDK通知会话失效。应用丢弃草稿和待提交历史，不把旧快照写回外部值；`History::discard_pending`有先红后绿的回归测试。诊断面`__vellum.replaceTextExternally`专门注入这种变化，它不走普通编辑事务。每个会话只额外注册一个输入监听，用于调整原生控件高度，并随会话清理。

[剪贴板](../examples/vellum/src/shell/clipboard.rs)调用SDK系统读写接口。系统写入成功才提示已复制；拒绝时明确提示仍可在Vellum内粘贴。粘贴优先解析系统中的Vellum载荷，无内部副本时支持普通文本；图层数据先经过文档解析器验证，再加入事务。载荷保留原版节点/根ID/资产格式，增加可选字体字段，旧载荷仍可用。令牌JSON下载通过SDK文件导出入口，CSS复制通过SDK剪贴板入口。

`cargo test -p rustify-ui --lib text::`按允许范围执行成功，但此DOM模块没有host单测（0项匹配，133项被过滤），不把它算作行为覆盖。原生控件行为由`m6-edit.spec.ts`在浏览器验证；`cargo check -p property-workbench --target wasm32-unknown-unknown`通过，验证旧调用方的源码兼容性。

`m6-edit.spec.ts`的13项检查通过，覆盖Unicode会话、旋转/相机定位、系统撤销、同步事务边界、文字工具/双击/blur、外部值失效、剪贴板拒绝与非法载荷、组件跨页传播/覆盖、布局约束、对齐分布、令牌下载及孪生编辑。CDP组合输入确认Enter/Escape在组合过程中不关闭会话；这是浏览器事件验证，不替代系统拼音输入法人工走查。文字工具分支取消兼容mousedown的默认焦点移动，防止新挂载的原生控件立即失焦。

文字与钢笔/锚点编辑后，两边文档几何一致，整页截图差异分别为779/1,600,000（0.0486875%）与1,202/1,600,000（0.075125%），均低于2%。证据为`test-results/vellum/m6-edit-fixed.log`、`m6-edit-visual.json`与`artifacts/m6-edit-fixed/`；模型136项、着色器5项及Clippy/格式检查通过。

### 文件、持久化与演示

[文件适配](../examples/vellum/src/fileio.rs)通过SDK `files::pick`打开选择器，通过`files::import_files`读取文件，通过`files::export`下载字节。应用负责80 MiB文档、25 MiB图片、15 MiB字体限制与格式校验；拒绝和取消不会提交编辑。图片由浏览器解码，超过64百万像素时拒绝；放入文档时最长边不超过800逻辑像素，资产保留原始分辨率。导入文档构成一笔可撤销的整文档替换，资产随历史快照恢复。

[持久化](../examples/vellum/src/storage.rs)使用原版的`vellum-editor/documents/current`键，IndexedDB打开或读取失败时回退到`vellum-document`本地存储。文档变更防抖500 ms，待提交的拖拽、检查器或文本事务不会作为恢复副本写入。写入串行执行并跟踪保存代次，只有对应副本真正写入成功才显示已保存；失败显示红色状态和导出提示。`vellum-options`与`vellum-welcomed`单独保留用户选项。普通卸载清理监听、数据库连接和待完成写入，计时代次使旧任务失效；loader的实例重启重新执行恢复流程，未保存编辑不会跨实例保留。

故障注入曾发现裸浏览器计时器会在实例fatal后继续保存。保存与toast延时现使用框架`defer_after`，页级输入及数据库事件使用`listener_options`提供的运行时AbortSignal；[动画帧适配](../examples/vellum/src/browser_frame.rs)保留rAF时序，同时绑定同一信号。`app.js`只增加原生资源关闭胶水：signal终止时取消rAF、终止事务、关闭数据库并移除本地字体，完全不调用已死Wasm。已排入的框架内部Promise微任务不由示例统一取消，应用异步落地入口另查运行时是否存活，保证不能继续写文档或存储。

本期还修复[SDK文件选择器](../crates/rustify-ui/src/files.rs)的既有闭包释放缺口：选择或取消后立即移除监听、释放闭包并只回调一次，监听同时绑定运行时终止信号。浏览器回归先观察到取消后仍保留2个监听，修复后多次取消和成功选择均回到0。字体随文档替换与Undo/Redo同步来源，移除已删除字族并使旧加载结果失效；同名字体切换的字宽回归也先红后绿。fatal时的字体移除同样由原生JS终止回调执行，重启后再从已保存文档载入。

PNG与[演示预览](../examples/vellum/src/shell/presentation.rs)使用同一份Canvas画家，先等待图片解码。导出使用独立图片缓存，避免旧文档快照的异步渲染改动当前屏幕缓存；输出限制为64百万像素及每边16,384像素。SVG继续由纯Rust生成并复用浏览器文字测量。演示层通过SDK `Layer`处理模态顺序与焦点，应用筛选顶级可见框架、循环导航并解析原型链接；异步预览用代次与作用域存活检查，避免快速换页或关闭后写入旧画面。

`tests/vellum/m7-files.spec.ts`的22项检查全部通过，包含真实文件选择/取消、拖放、拒绝路径、存储事务中止、三次fatal重启、故障字体清理、550×250 PNG像素、SVG结构和原型跳转；日志为`test-results/vellum/m7-complete.log`。双向互开保留三页202层，另用`interop.mjs`解析浏览器导出的文件通过（`m7-complete-interop.log`）。SDK仅执行允许的`cargo test -p rustify-ui --lib files::`，7项通过、126项过滤；浏览器检查补足选择器生命周期验证。

## 可复核数字

最终release产物的六类体积如下，来自`test-results/vellum/m8-build-manifest.json`的`size_report`；单位为原始字节，不代表压缩传输量。

| 类别 | 字节 |
| --- | ---: |
| Wasm | 20,221,645 |
| JavaScript（包含框架与生成胶水） | 244,922 |
| CSS | 32,652 |
| 字体 | 51,187,844 |
| 图片 | 20,701 |
| 数据与许可 | 8,050 |
| 总计 | 71,715,814 |

字体是构建流水线随Makepad资源复制的部分，不能把全部产物体积当作首屏实际下载量。连续两次`cargo xtask build-web --example vellum --release`生成的43个文件（含manifest）SHA-256完全一致，证据为`test-results/vellum/m8-build-determinism.json`与`m8-build-compare.log`。

应用入口`app.js`为145行、7,115字节（`wc -l examples/vellum/app.js`及manifest的`files["app.js"]`）。它负责loader接线、薄自动化门面和fatal原生资源关闭；编辑规则、文件格式、持久化策略与Canvas画家均在Rust中。其余JavaScript来自框架或构建胶水。

应用Rust显式常驻DOM监听为16个：指针/滚轮/失焦9、键盘2、拖放2、菜单外点1、持久化页生命周期2。原生文字会话临时增加1个input监听；IndexedDB打开/读取/写入分别临时增加3/2/3个。`app.js`另按原生资源绑定AbortSignal：数据库连接1、每个本地字体1、每个待执行rAF1、打开请求2（abort与晚到success）、每个活动事务1。因此16不是整个实例的总数，也不包含Leptos声明式事件、SDK内部监听与ResizeObserver；SDK文件选择器另有2个临时监听，选择或取消后释放。

计数可用以下命令复核注册点，并展开`pointer.rs`的两组事件循环：

```sh
rg -n 'for name in|bindings\.listen' examples/vellum/src/pointer.rs
rg -n -A4 'add_event_listener_with_callback_and_add_event_listener_options' examples/vellum/src/{keys,fileio}.rs examples/vellum/src/shell/{menus,text_session}.rs
rg -n -A4 '\blisten\(' examples/vellum/src/storage.rs
rg -n 'addEventListener|removeEventListener' examples/vellum/app.js
```

`m8-metrics.spec.ts`在三个独立浏览器上下文禁用HTTP缓存，使用页内`performance.now()`记录ready、首次SDK呈现和光栅细化完成；数据为`test-results/vellum/m8-startup-1.json`至`m8-startup-3.json`，不把操作系统缓存视为已清除，也不把它称作物理显示延迟。

三次启动实测如下（Chromium 153.0.8010.12、SwiftShader）；首个样本的ready耗时较高，保留原始值，不以三次样本作跨机器性能保证。

| 样本 | ready（ms） | 首次呈现（ms） | 首次光栅细化完成（ms） |
| --- | ---: | ---: | ---: |
| 1 | 1913.2 | 337.6 | 337.7 |
| 2 | 314.0 | 256.2 | 256.4 |
| 3 | 313.3 | 256.8 | 257.0 |

Forma 金样为三页、202 层，分别为 171、31、0 层；命令 `node tests/vellum/interop.mjs --generate` 可从本地原版源码重新生成并验证。

浏览器渲染探针使用 Chromium、SwiftShader、1600 × 1000、DPR 1：5,000 个形状的实际绘制次数为 1，连续 20 次属性编辑均产生呈现帧。20 级缩放重复三轮的 GPU 字节序列相同，峰值为 12,846,744 字节；光栅纹理当前保留量随视口和分辨率变化，没有逐轮增长。数值来自 `test-results/vellum/m2-scene.log`，不是性能门限。

最终产物重新测得5,000形状编辑CPU p95为6.3 ms，20次编辑均呈现且绘制次数仍为1；30次平移输入至SDK呈现帧的p95为34.8 ms，未触发50 ms重评条件。跨分辨率冷缩放首次呈现14.7–19.6 ms，细化完成67.5–146.3 ms、耗用2–5帧。启动页为171层、173实例、144次绘制、1个区域，SDK GPU账本为1,450,576字节；这些是本机样本，不是跨机器性能保证。汇总见`test-results/vellum/m8-performance.json`，来自最终`m2-scene.spec.ts`、`m3-raster.spec.ts`、`m4-pointer.spec.ts`日志。

四层祖先圆角裁剪通过像素断言，浮点纹理裁剪链可用；预乘叠色实测 `[70,6,135]`，理论值 `[70.25,6.5,134.75]`。浏览器上下文的 `premultipliedAlpha` 为 `true`。两项探针均不需要计划中的备用实现。

## 补充能力与应用自管部分

文档模型保留为纯 Rust，便于在 host 上验证文件互操作与编辑语义；无需把业务状态塞入 GPU 区域。浏览器光栅、覆盖画布、输入语义和持久化策略由应用维护；框架负责挂载作用域、信号投影、区域恢复、浮层顺序、原生编辑与浏览器能力入口。本期SDK只补`TextEdit`可选变换与`files::pick`清理，未改Makepad分叉。

## 孪生对照结果

首张启动页 fit-all 亮色对照差异为 30,109 / 1,600,000 像素，即 **1.8818125%**（RGB 通道差绝对值之和 > 24）。比较时统一性能/引擎标签并隐藏欢迎提示，同一 Chromium 的原版以 `?canvas` 运行。该早期差异包含当时外壳骨架尚缺的色块、图层树缩进和图标，以及形状/阴影光栅差异；此数字不是最终验收结果。用户已在M3后确认视觉门值为每张≤2%。

已额外运行有头 Google Chrome 152.0.7977.84，确认原版使用 `WebGPU`、适配器为 `apple metal-3`，并与同一浏览器的原版 Canvas 对照。三页亮/暗场景差异分别为 2.4268% / 1.0125%、0.3712% / 0.7527%、0% / 0%；数据见 `test-results/vellum/m3-webgpu.json`。对照锁定相同文档散列、相机、视口和 DPR。目视检查启动页确认主要内容和位置一致，GPU 阴影较 Canvas 的模糊阴影更紧；这说明 Canvas 参照自身与原版 GPU 也有后端差异，不能把全部像素差都归为移植错误。

最终产物的六张 Rust / 原版 Canvas 对照如下。截图为1600×1000，DPR 1，RGB通道差之和>24计为差异；测试固定对每张执行≤2%断言。六张均达标，日志为`test-results/vellum/m8-final-visual.log`。

| 页面 | 亮色差异 | 暗色差异 |
| --- | ---: | ---: |
| Design exploration | 1.651% | 0.742375% |
| Design system | 0.0768125% | 0.311375% |
| Playground | 0.03875% | 0.0335% |

完整数据与截图位置见`test-results/vellum/visual.json`、`test-results/vellum/artifacts/m8-final-visual/`。

最终主菜单打开状态差异为1.661625%，帮助模态为0.6770625%；文字编辑后为0.0335%，钢笔编辑后为0.075125%，也均满足每张≤2%。数据见`test-results/vellum/shell-visual.json`、`test-results/vellum/m6-edit-visual.json`，对应`m8-final-visual.log`与`m8-final-m6-edit.log`。先前M3–M7的阶段数字保留在计划完成记录中，后续继续缩小差异时以上表为基线。

最终有头Chrome 152/Apple Metal另与原版WebGPU锁定同文档及相机比较六张：整页差异0.019125%–0.2719375%，画布区0.0015922%–0.0073641%。代理逐组查看截图确认内容/位置/字体布局一致，外壳仍有图标、文字截断与细小间距差异。WebGL上下文丢失与恢复保持文档和相机完全相同，区域0→1、中心颜色及绘制恢复；详见[最终验证报告](validation/vellum/m8.md)。正式2%验收仍使用同浏览器Canvas参照。
