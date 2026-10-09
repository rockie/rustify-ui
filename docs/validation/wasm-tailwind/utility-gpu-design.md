# Utility Lab · GPU 扩展设计与验收范围

本记录是既有 Utility Lab 的局部 Operate 扩展，沿用 PRODUCT.md、DESIGN.md 与 M3 页面事实，不建立新的视觉身份。用户目标是执行 WASM-TAILWIND 计划，当前扩展负责四个节点的类名输入、DOM/GPU 对照、strict/partial、状态和真实绘制结果；源码输入、cn 分片及配置切换仍待后续集成。

## Direction contract

THESIS：输入完整类名，比较已安装的 DOM 样式与 GPU 实际画面，支持边界和失败恢复必须可检查。

OWN-WORLD：继承现有浅色中性工具壳、IBM Plex Sans、代码字段 Liberation Mono、蓝色主操作、4px 控件圆角和细边界。样本本身由输入样式决定。

STORY：先输入按钮类名，必要时展开容器、文字和色块输入；Apply 后查看两个预览及独立 scope，对照 applied/drawn 与诊断。

FIRST VIEWPORT：标题和类名输入在上方，单一 Apply 主操作与恢复、复制操作相邻；policy 和 disabled 控制在输入后；桌面两个预览并列，390px 下纵向排列；独立 scope 与输出折叠区继续保留。

FORM：既有页面的代码扩展，局部请求不运行 concept-seed，没有 seed key、comp 或新增摄影/插图。此契约是实施后的审计记录，没有宣称它在代码前已落盘。

FINISH：unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance

## Quality bar 与实际证据

以本仓既有中性工具界面为标准：读者能找到输入、Apply、GPU 状态和具体诊断；键盘可操作，控件命名准确，移动端无横向溢出，失败保留可恢复结果。GPU 像素、字体、几何与生命周期采用实际浏览器断言，不靠 mock 图。原设计文件保持原样；没有新增 shipping raster。

六项 GPU 检查覆盖真实色彩/边框/字体像素与几何、strict/partial/删除恢复、指针和键盘/disabled、浏览器 viewport/alpha、文本资源/row gap/ring，以及 WebGL context loss 后使用最新 props。另12项 runtime/fault 和深层路径7项通过。证据限定于 build `a6d42f3c4669da61`；已有 RustifyButton 接入仍在实现，不能用 UtilityLeaf 的按钮路径替代它。

审阅输入：[桌面](utility-gpu/matrix-desktop.png)、[手机](utility-gpu/matrix-mobile.png)、[GPU 浏览器](utility-gpu/matrix-browser.json)、[root runtime](utility-gpu/runtime-browser.json)、[deep runtime](utility-gpu/deep-browser.json)、[manifest](utility-gpu/matrix-main-manifest.json)。截图已打开验证完整内容和实际 GPU 绘制；两轮主线外观检查结束，后续视觉修改仅按独立 reviewer 的意见进行。

[detector](utility-gpu/detect.json) 报告 textarea `mono`。命令 exit 2，同时提示源目录没有构建生成的 runtime.css/theme-fonts.css，因此 linked CSS 检查不完整；没有再次运行。实际 app.css 与 DESIGN.md 均声明 Liberation Mono，reviewer 已核对构建字体声明并处置该 warning；不宣称完整 detector 通过。

## Finish 与文档交接

[独立 finish review](utility-gpu-review.md) 的第二次 Verdict Pass 为 **ship**，仅覆盖三个原始修复及引入的桌面内部行间距回归；四项评分均 resolved。最终同名截图为桌面 1280 × 1557、手机 390 × 2029、终止失败 1280 × 1570、支持边界 1280 × 1729；[spacing root 浏览器记录](utility-gpu/spacing-root-browser.json) expected 15，skipped/unexpected/flaky 均 0。manifest build 为 `9ae7212d2dc204b7`，CSS-only 间距修正沿用此标识；上方 `a6d42f3c4669da61` 与六项 GPU 检查是初始评审的历史基线。

[文档核验](utility-gpu-documentation.md) 已比较最终截图、源样式和 incumbent 系统，保留 DESIGN.md 与 `.impeccable/design.json`；无新增 shipping raster，未确认实质系统漂移。此交接完成上述修复的 review/documentation，不是 whole-surface、RustifyButton、M5 或完整计划验收；ThemeScope 与完整支持矩阵仍须继续。后续改动需重核受影响证据。
