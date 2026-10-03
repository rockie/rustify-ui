disposition: fix

输入说明：固定的 code-led Operate 扩展未提供 comp、竞选图或 QUALITY BAR card；不据此发明视觉目标。未通读 SDK 实现与完整测试源码；本次是文件和截图终审，未运行浏览器、编译或第二次 detector。

## persistence

pass。PRODUCT.md 与方向契约均存在；FORM 的 `user-pinned-theme-studio` 对应调用包明确给出的用户固定计划与双栏结构。此次没有 comp round，state/spec/diff 与 comp 选图记录不适用；没有既存 DESIGN.md 需要对齐，后续文档收口仍由主线程完成。

证据检查通过：逐张打开了 `docs/validation/theme-studio/M6/screens/` 全部 34 张 PNG、M5 的 `theme-export-css.png` / `theme-export-rust.png`，以及 `.impeccable/review/desktop.png` / `mobile.png` / `user-768.png`。桌面 1440×900、平板 768×1024、手机 390×844、恢复组 1280×720、200% 模拟组 720×900 的尺寸与内容相符；内部滚动样本截图显示其命名的字体或阴影内容，没有空白或损坏渲染。200% 导出截图是对话框内部滚动后的代码选择与导出操作视口。

原 detector 的 11px 启动状态问题已在 `app.css` 的 `#status` 改为 14px；ready 时隐藏不影响既有截图。构建目录实际包含 `runtime.css` 与 `theme-fonts.css`，静态检测的缺文件提示不是产物缺失。`restart.log` 显示宿主 fatal 恢复测试通过；完整回归日志仍有一项 reset 初始快照测试待主线程核实，本评审不把 57 passed / 1 failed 写成全绿。

## fidelity

截图的元素库存是：稳定的中性编辑栏、紧凑本地操作与预设工具、清楚的浅深色及字段切换、真实表单预览、独立 GPU 样本板、可收起的作用域对照、窄屏编辑/预览切换，以及保留原主题的导入错误对话框。

| 元素或承诺 | 状态 | 证据与判定 |
| --- | --- | --- |
| TYPE / OWN-WORLD | match | 中性工具壳使用 IBM Plex Sans；Graphite / JSON 导入后预览变为 Noto Serif，GPU 字体样本同步显示字体、中文与字形测试。技术代码采用等宽字体，没有另造展示字体。 |
| MATERIAL / THESIS | match | 截图与 `GpuPreview` 的真实 `GpuRegion` 一致；颜色、alpha 棋盘、几何、交互控件与阴影是主题样本本身，没有用装饰性图片或 CSS 仿材质替代渲染器。 |
| GROUND / OWN-WORLD | match | 白色编辑栏和中性浅灰工作区保持稳定；深色主题仅进入 DOM/GPU 预览，未将操作壳染成用户主题。符合契约明确指定的 neutral light shell。 |
| THESIS：同源主题、可读操作壳 | match | Graphite、深色与独立 JSON 图中 DOM/GPU 的主题效果和工具壳隔离都可见。 |
| STORY：选预设、编辑、检查、保存/导出 | match | 预设、修改标记、本地保存、导入/导出及 inspector 入口可见；M5 独立接入图显示导出 CSS 与 Rust/JSON 的实际使用。 |
| FIRST VIEWPORT / FORM | adaptation | 桌面保留 320px 双栏；768px 将并排预览纵向排布；390px 使用 Edit/Preview 并保留返回编辑入口。依据计划 §6.1、§6.2 的窄屏与真实控件要求。初始选中 token 在 `controller.rs` 中为 primary；截图中 foreground / Type & other 是已编辑状态，不据此认定初始入口消失。 |
| 中英文运行状态 | contradicted | `desktop-zh-dark-preview.png` 与 `tablet-zh-dark-preview.png` 的主工具为中文，GPU 标题及状态仍为 “GPU theme samples / Ready”。`gpu_preview.rs:13` 读取独立 mount 的 `use_locale()`，虽已有中文状态文案却未跟随当前编辑器 locale。这是运行状态信息，区别于允许保留英文的 token、字体与几何样本标签。 |
| Truth / 演示与原色 | match | DOM 明示本地演示，导出图明示 fixture / consumer；没有商业指标或云保存承诺。低对比度与特定字距属于用户主题样本，不能为视觉评分改写预设。 |
| Floor / 工具语义 | match | DOM/GPU 是识别渲染器的功能标签；字体样本的 emoji 是字形覆盖测试，均不是装饰性 eyebrow 或 Unicode 图标系统。卡片、阴影和嵌套作用域是计划要求的组件演示，不是页面 scaffolding。 |

## ceiling

reached，按已固定工具方向的可审查范围。双渲染器对比、原色与 alpha、跨字体字形、阴影尺寸和独立作用域构成此工具自己的视觉材料；不需要营销视觉、额外 ornament、装饰动效或 raster 资产。未提供 QUALITY BAR card，不能对不存在的 card 声称逐项达标。

## material_fixes

1. Contract / 计划 §6.2：让 `GpuPreview` 显式使用当前编辑器 locale，覆盖标题、Starting / Ready / Lost / Failed 及资源回退和恢复说明；重捕现有中文桌面、平板和手机路径，并验证中文故障提示。证据：中文预览截图及 `examples/theme-studio/src/gpu_preview.rs:13`。

## keep

保持中性可读操作壳、用户预设原色、真实 DOM/GPU 共享主题及不重挂 canvas 的编辑过程，保留字体、alpha、阴影和恢复样本的可观察性。
