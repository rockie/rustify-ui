30 张 R6 截图与 30 张 R3 同名图已全部逐张打开并对照完毕，材料（目标包、capture.md、U3-capture.json、U3-operations.md、R3 judge）均已核对。以下为最终评审结论。

# R6 U3 官方双语阅读与入口 独立评审结论

本轮 30 张 R6 截图逐张打开，30 张 R3 同名图逐张对照；结合 U3-operations.md 实测记录与目标包 U3 行及 O1–O4、G1–G4、S6 逐项核对。评分仅限本轮可观察范围。

## 评分表（四维+总分）

| 维度 | 分数 |
|---|---:|
| 布局 | 3/3 |
| 视觉 | 3/3 |
| 交互与状态 | 3/3 |
| 细节 | 1/1 |
| **总分** | **10/10** |

## 逐维理由（引用截图文件名）

**布局 3/3**：1280 下首页无侧栏、hero 居阅读列（`U3-en-home-1280-light.png`、`U3-zh-home-1280-light.png`），quickstart 为三栏（侧栏/正文/本页目录，`U3-zh-quickstart-1280-light.png`），符合 G4/O1。375/320 全部收为单栏，导航与目录折叠为「菜单」「本页目录」details（`U3-zh-quickstart-375-light.png`、`U3-zh-quickstart-320-dark.png`），窄屏顶栏增高换行但无遮挡（`U3-en-home-320-light.png`，G4 允许）。所有截图可见范围内无横向页面滚动条、无内容越界，长代码 token（如 `wasm32-unknown-unknown`）在 320 下正常换行（`U3-zh-quickstart-320-light.png`）。缺译页与 404 页均保留文档框架（`U3-fallback-1280-light.png`、`U3-404-closed-1280-light.png`、`U3-404-closed-320-dark.png`），符合 O3/O4。已达本档上限。

**视觉 3/3**：双主题背景/正文/次级文字层次与 G2 描述一致（浅 `U3-zh-home-1280-light.png`，深 `U3-zh-home-1280-dark.png`）；蓝色仅见于链接、当前位置（侧栏「快速开始」、目录「准备工具」）、主操作与搜索选中/命中（`U3-404-search-1280-light.png` 中选中组为浅蓝底、命中词蓝色下划线）。缺译 note 为安静 code-bg 面板、细边界、小圆角，无加载/错误外观（`U3-fallback-1280-light.png`、`U3-fallback-320-dark.png`），符合 O3。代码面板、复制按钮、mono/sans 字体分工延续阅读世界（`U3-zh-quickstart-1280-dark.png`）。未见可确定偏差。已达本档上限。

**交互与状态 3/3**：实测记录与截图共同支持——主题经页面真实按钮设定（各图 focused 记录为 Light/Dark/浅色/深色按钮）；语言菜单真实打开、普通链接、当前语种蓝色标示（`U3-language-open-375-light.png`，O3 原生 details）；按 Escape 关语言菜单后导航 details 打开（`U3-navigation-open-375-light.png` 及操作记录）；中文深 404 可打开搜索、输入「快速开始」得中文结果与 live 总数「2 个结果」（`U3-404-search-1280-light.png`、`U3-404-search-320-dark.png`，S6/O4），真实 Enter 落在中文 quickstart（`U3-404-destination-1280-light.png`、`U3-404-destination-320-dark.png`）；搜索输入可见焦点环。禁脚本下 404 与 quickstart 正文、导航、语言首页链接均可读，404 页含「启用 JavaScript 后可搜索」说明（`U3-noscript-404-375-light.png`、`U3-noscript-quickstart-375-light.png`，O4/S6）。未拍到的过程列入未证清单，不在此分判过。已达本档上限。

**细节 1/1**：规范模式下，可见中文译文自然、控件命名与界面语言一致（搜索/关闭/菜单/本页目录），`htmlLang`、标题记录与画面一致；长文字换行、边界对齐、菜单靠右（`U3-language-open-375-light.png`）未见确定偏差；`U3-404-destination-320-dark.png` 与 `U3-zh-quickstart-320-dark.png` 画面一致，与 capture.json 同 sha256 记录吻合，落地页即同页顶。无像素目标图，不宣称逐像素同稿。已达本档上限。

## 与 R3 对照（回归/无回归）

- **无回归**。30 对同名图逐张对照，全部视觉一致；capture.md 记录本轮 build_id 与 R2–R5 逐位一致（确定性产物），与对照结果吻合。
- **R3 已关闭差距 UI-03 未复发**：R6 `U3-fallback-1280-light.png`、`U3-fallback-320-dark.png` 中「此页面尚未翻译。」note 仍位于 H1 后、正文前，双主题均保留 code-bg 层次与细边界，与 R3 同位置同形态，符合 O3。
- R3 评审列出的三条不扣分建议（窄屏顶栏折行、展开导航长度、禁脚本说明位置）本轮表现相同，维持为建议。

## 已确认差距（或「无」）

**无。** 未关闭差距可挽回分合计 0.0，与 10 − 10 = 0 一致。

## 待核实

无拿不准的差距候选。R3 至 R6 同名图逐对一致，未发现新的疑点需要额外证据裁决。

## 未证清单

以下本轮看不到，不猜、不判通过或失败：

1. **完整键盘路径与焦点可见性**：仅见搜索输入焦点环（`U3-404-search-1280-light.png`）与 summary 打开态；无完整 Tab 顺序、全部控件焦点样式、悬停/按下/过渡过程。
2. **语言菜单 Escape 的即时归焦**：操作记录写明真实按 Escape，但截图是后一次点击状态，归焦瞬间未拍。
3. **普通链接激活结果**：首页 Quick start/Components、语言菜单链接、导航条目、目录锚点、404 的「返回首页」与各语言首页链接的实际点击落地未拍。
4. **O2 源章节 ID 保留与锚点定位精度**：截图可见标题与目录，不能验证 ID 与跳转位置。
5. **精确 token 数值**（36rem/76rem、hero clamp、字号行高、hex、圆角）与无 webfont、系统字体来源：截图支持观感一致，不能证明数值。
6. **长页与内部滚动**：代码面板横向滚动、宽表、长目录滚动未拍；G4「320 无页面横滚」仅就各图可见范围确认。
7. **禁脚本细节**：搜索入口的 disabled 外观/行为从静态图无法确认；quickstart 禁脚本首屏未见说明文字（页面下部未拍）；禁脚本下展开导航与实际点链接未拍。
8. **768–1279px 两栏状态**：不在本轮 U3 矩阵内，未拍。
9. **减少动态效果的实际行为**：本轮默认 reduce motion，无对照证据。
10. **辅助技术实际朗读**（VoiceOver 等）：属 U4 范畴，本轮未采，不补分。

## 建议（不扣分）

1. **窄屏次要入口占用**：`U3-en-home-375-light.png`、`U3-en-home-320-light.png` 中英文顶栏主题控件与 GitHub 各占一行，hero 起点下移；如需更多首屏正文可收紧次要入口。G4 允许窄屏顶栏增高，无遮挡，不扣分。
2. **展开导航长度**：`U3-navigation-open-375-light.png` 中导航按文档流展开，组件列表把正文推出首屏；可考虑长列表内部滚动并验证键盘焦点跟随。规范未要求，不扣分。
3. **禁脚本说明位置**：说明文字见于 `U3-noscript-404-375-light.png` 正文下部，`U3-noscript-quickstart-375-light.png` 首屏未见；可靠近搜索入口提高可发现性。不能因首屏未拍到就断言整页缺失，不扣分。
4. **搜索摘要词边界**：`U3-404-search-1280-light.png` 与 `U3-404-search-320-dark.png` 中第二组「Rustify UI」摘要首词显示为「OM 组件」，疑为「DOM 组件」截断所致；可考虑按词边界截取摘要。规范要点未规定摘要截断方式，不扣分。
