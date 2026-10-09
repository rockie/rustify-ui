# Utility Lab · GPU 扩展文档核验

本次是既有 SDK 示例的局部扩展。已核对 [PRODUCT.md](../../../PRODUCT.md)、[DESIGN.md](../../../DESIGN.md)、[sidecar](../../../.impeccable/design.json)、[方向契约](utility-gpu-design.md)、[M3 设计记录](utility-lab-design.md) 与 [finish review](utility-gpu-review.md)。依据 impeccable 的 document/new-work 普通扩展规则，保留 incumbent 设计文件，不建立新世界或改写全局 token。

## 结论与范围

reviewer 第二次 Verdict Pass 的 disposition 是 **ship**：原始三项修复（终止 GPU 失败的原因与恢复说明、四节点可见结果与诊断、桌面预览对齐）以及修复引入的桌面内部行间距回归，均评分为 resolved。本记录完成这些修复范围的文档交接，**不代表 whole-surface、RustifyButton 集成、M5 或 WASM-TAILWIND 完成**。没有重新评分整个界面，也没有运行浏览器、编译、产品测试或 detector。

## 与 incumbent 系统的比较

事实源：[app.css](../../../examples/utility-lab/app.css)、[index.html](../../../examples/utility-lab/index.html)、[app.js](../../../examples/utility-lab/app.js)、[main.rs](../../../examples/utility-lab/src/main.rs) 与 [字体声明](../../../web/theme-fonts.css)。

| 方面 | 完成产物与既有系统 |
| --- | --- |
| 色彩与材质 | 保留中性浅底、白色输入/绘制面、细边界、蓝色 Apply/焦点以及可见错误文本；输入 utility 控制预览样本，不把样本颜色推广成工具壳 token。 |
| 字体与层级 | 保留 IBM Plex Sans 工具壳及 Liberation Mono 类名/CSS 字段。30px 主标题、14px 等宽输入、13px 输出已在 M3 记录中存在；DESIGN.md 的 Theme Studio 层级不要求所有示例同字号。没有新字体。 |
| 布局与形状 | 保留最大 1080px 内容宽度、4px 输入/按钮圆角与 264px 预览框。桌面 subgrid 对齐预览顶边，列距 32px、内部行距 0；640px 以下保留单列、24px 间隔。局部布局不改变 Theme Studio 的侧栏或断点。 |
| 状态与诊断 | 编译安装状态与 GPU 结果明确分开；每个节点结果可见，受影响节点列出 candidate/property/reason 与移除类或 supported-only 恢复方法。终止失败保留 DOM 编辑，显示实际原因并说明复制输入后 reload 重试 GPU；Restart compiler 仍只负责编译器。完整 JSON 保留在 disclosure。 |
| 深度与资源 | 工具壳仍以留白、细边界和字号分层。真实 DOM/GPU 绘制是产品内容；本次没有新增 shipping raster。验收截图是证据，不是产品插图。 |

本次未确认 incumbent 系统的实质漂移。上述字号、控件和页面密度差异已有 M3 依据，属于局部 adaptation；sidecar 的 Theme Studio 组件与断点也不要求增加 Utility Lab 的页面规则。现存 detector 的 `mono` warning 与实际 Liberation Mono 声明不符，reviewer 已核对字体并处置；不据此改字体或刷新全局设计文件。[方向契约](utility-gpu-design.md) 记录 detector exit 2 与缺少生成 linked CSS 的覆盖限制，因此不宣称 detector 全通过。

## 核验过的证据

本 documenter 逐一打开以下四张最终截图，确认文件内容、文档顶部、完整页面和对应状态；同时读取 PNG 尺寸。桌面两框保持对齐，手机保留可读输入/操作和纵向预览，失败图显示真实 WebGL2 原因与恢复说明，边界图显示三个非按钮节点拒绝的具体原因。

| 文件 | 实际尺寸 | 用途 |
| --- | --- | --- |
| [matrix-desktop.png](utility-gpu/matrix-desktop.png) | 1280 × 1557 | 桌面比较、紧凑间距和正常 GPU 结果 |
| [matrix-mobile.png](utility-gpu/matrix-mobile.png) | 390 × 2029 | 手机换行与纵向布局 |
| [gpu-terminal-failure.png](utility-gpu/gpu-terminal-failure.png) | 1280 × 1570 | GPU 初始化终止失败，DOM 可继续编辑 |
| [gpu-boundaries.png](utility-gpu/gpu-boundaries.png) | 1280 × 1729 | Container/Text/Color block 可见拒绝诊断 |

该修复轮 [spacing-root-browser.json](utility-gpu/spacing-root-browser.json) 的 stats 为 expected 15、skipped/unexpected/flaky 均 0，[日志](utility-gpu/spacing-root-browser.log) 记录 15 passed；这是已有浏览器执行证据，不是 documenter 新跑的测试。[manifest](utility-gpu/matrix-main-manifest.json) build 为 `9ae7212d2dc204b7`。reviewer 明确记录 CSS-only 间距修正沿用该 build 标识；新截图和 `app.css` 是这次 CSS 修正的证据，不仅凭 wasm build 标识判断新鲜度。`a6d42f3c4669da61` 的初始评审与六项 GPU 结果属于历史基线，不替代这轮最终证据。

## 写入边界与后续

仅写本记录及方向契约的完成段。DESIGN.md 与 `.impeccable/design.json` 保持逐字节原样，核验 SHA-256 分别为 `f49cc61fd291f2c9e4ed475e53c55e9eae6025f81050db447c30208ac803f76d` 和 `45981d97521945d90ffe258f74368f93c1b646875024d5ebd6bbbc08bcf471b9`。

[M5 记录](M5.md) 仍为进行中，其 RustifyButton、ThemeScope/暗色/host rem、完整矩阵与旧示例 None 路径的最终复验不在此次交接范围。后续相关实现或界面改动须重新核对受影响的证据与文档。

## 后续语义接线重核

新版 [main.rs](../../../examples/utility-lab/src/main.rs) 在真实 ThemeScope/ThemeBoundary owner 内通过 `Signal::derive` 读取 `effective.get()`，将 effective theme、layout rem 和当前浏览器 viewport 传入四节点 GPU props。[app.js](../../../examples/utility-lab/app.js) 增加宿主/测试 `api.theme`，没有新增可见控件；[model.rs](../../../crates/rustify-ui/src/gpu/utility/model.rs) 的 `update_baseline` 同步 layout rem，并在拒绝绘制值时保留最新 theme/context 与此前可绘制 style。现有 CSS、字体与全局 token 继续沿用 incumbent 系统；DESIGN.md 和 sidecar 的 SHA 与上文一致。

已读取新版 [root JSON](utility-gpu/theme/root-browser.json) / [日志](utility-gpu/theme/root-browser.log) 和 [deep JSON](utility-gpu/theme/deep-browser.json) / [日志](utility-gpu/theme/deep-browser.log)：分别 expected 21/16，skipped/unexpected/flaky 均 0，日志分别记录 21/16 passed。[root manifest](utility-gpu/theme/main-manifest.json) 与 [deep manifest](utility-gpu/theme/deep-main-manifest.json) 同为 release `90d1cc287cd574da`，base 分别为 `/` 和 `/tools/utility-lab/`。已读 [主题用例](../../../tests/browser/utility-gpu.spec.ts)，其断言覆盖作用域隔离、颜色 patch、暗色边框、layout rem、独立 media rem、invalid patch 保留、清除 patch 和安装报告不变，并比较实际 GPU 像素为 `[171,205,239,255]`（`#abcdef`）。这证明已运行的用例范围，不把 37 项累计通过等同于完整 M5 支持矩阵。

已逐一打开新版 root 的 [桌面](utility-gpu/theme/root-gpu-desktop.png)（1280 × 1557）、[手机](utility-gpu/theme/root-gpu-mobile.png)（390 × 2029）、[终止失败](utility-gpu/theme/root-gpu-gpu-terminal-failure.png)、[支持边界](utility-gpu/theme/root-gpu-gpu-boundaries.png)，及 [root 主题边界](utility-gpu/theme/root-gpu-gpu-theme-boundary.png) / [deep 主题边界](utility-gpu/theme/deep-gpu-gpu-theme-boundary.png)（均 1000 × 1643）。正常布局、失败恢复说明和可见诊断保留；主题图显示局部浅蓝样本与独立 scope 的原蓝样本，工具壳没有新视觉身份。这是后续局部文档重核，没有新 full-review；原 reviewer 的受限 ship 仍仅绑定 `9ae7212d2dc204b7` 的三个修复及间距回归，不扩展为新版 whole-surface 或 M5 approval。M5 保持进行中；本 documenter 未运行浏览器、编译或 detector。
