# DOM-COMPONENTS-RELEASE · UI 验收目标包

- 需求来源：用户 2026-11-10「更新 rustify-docs 做的 github pages，展示最新的 docs/PRD-DOM-COMPONENTS.md 成果」，2026-11-11「要按照原目标完成」「把我们做的东西充分展示」；计划 DOM-COMPONENTS-RELEASE 的 M5–M7 展示范围（计划 §2、§5.5、§5.6、§6）。
- 首轮采用**规范模式**。没有用户提供的目标图或设计稿；不把历史实现截图当作设计目标。
- 范围：官方 Docs 站（DOM/SSG + 按需 island）的首页、组件索引与分类、旧详情页、Colors、Primitives、Studio 介绍页，以及同包公开的现有 Theme Studio 应用入口。不验收 GPU 渲染、发包与部署流程、真人设备和读屏器。

## 单元

| 单元 | 状态 | 视口 × 主题 | 目标 | 比较口径 |
| --- | --- | --- | --- | --- |
| U1 首页 | 英文首屏、中文首屏、预览 island 操作后 | 1440×900 × light、dark；390×844 × light；中文 1440×900 × light | 规范要点 H1–H3、A1–A2 | 规范模式；README 引言正文不比；三层能力、主入口和预览必须在首屏可见 |
| U2 组件索引与导航 | `/components/` 总索引首屏与九组分区；分类页侧栏当前位置；窄屏导航面板展开 | 1440×900 × light、dark；390×844 × light | 规范要点 N1–N3、A1–A2 | 规范模式；组件名称、分组和链接目标须一致，条目示例文字不比 |
| U3 旧详情页（以 Button 为代表） | 默认、改变变体/尺寸/颜色并点击后、disabled、loading；源码区 | 1440×900 × light、dark；390×844 × light | 规范要点 D1–D3、A1–A2 | 规范模式；计数等示例数据不比；变体、尺寸与状态必须可见区分 |
| U4 分类页 · 交互、浮层、菜单 | `/components/interaction/`、`/overlays/`、`/menus/` 首屏；表单提交结果；AlertDialog 与 Popover 打开；Menubar 键盘打开含子菜单/禁用项 | 1440×900 × light、dark；390×844 × light | 规范要点 D2、D4、D5（交互、浮层、菜单相关行）、A1–A2 | 规范模式；示例业务文案不比；可操作结果与状态反馈必须可见 |
| U5 分类页 · 导航、内容、反馈、布局、排版、扩展 | 六个分类页的代表样例；Accordion 展开、DataTable 排序/选择、Splitter 调整、排版参数变化、Toast 显示 | 1440×900 × light、dark；390×844 × light；640×450 DPR 2（模拟 200% 放大）× light | 规范要点 D2、D4、D5（其余行）、A1–A2 | 规范模式；同上 |
| U6 Colors 与 Primitives | `/colors/` 默认、改选色族/模式/透明/P3 后、下载表；`/primitives/` raw/themed 对照、RTL 切换 | 1440×900 × light、dark；390×844 × light | 规范要点 C1–C2、P1、A1–A2 | 规范模式；色值必须是真实颜色字符串，阶号与色块一致 |
| U7 Studio 介绍与公开应用 | `/studio/` 介绍页；`/theme-studio/` 启动就绪；Components 场景；Inspector 选中组件；窄屏 Edit/Preview | 1440×900 × light；390×844 × light | 规范要点 S1–S3 | 规范模式；Studio 编辑器内部视觉以既有已验收设计为准，本轮只比公开入口、往返、完整能力入口与布局是否保持 |

## 规范要点

### H · 首页

- H1：首屏清楚呈现三层能力（Colors / Primitives / Themes）与「浏览组件 / 打开 Theme Studio」主入口，辅以真实组件构成的短预览；不把原 Git/GPU 定位或发行说明堆成唯一主体。首页同时给出 Components、Colors、Primitives、Theme Studio 入口。出处：计划 §6.1 第 1 段、§5.5 第 2 项。
- H2：首页沿用 Docs 阅读壳的 hero：左对齐，不居中；一个「Rustify UI」H1（700，`clamp(2.25rem, 1.6rem + 2.6vw, 3.25rem)`），一句 muted tagline（1.1875rem/1.55，最宽 36rem），hero 最宽 44rem；操作位于 hero 下 2rem、间隔 0.75rem，主操作为 primary 填充 + 生成的标签色，次操作为 `border` 描边 + foreground 文字，均 2.75rem 高、0.9375rem、600、8px 圆角。feature 为标题（1rem/600）+ 一句 muted 描述，各项以 hairline 分隔；首页与内容中不使用卡片、阴影、图标或插画。出处：DESIGN.docs-ssg.md「Home hero, actions and features」「Layout · Home page」「Don't」。
- H3：中文首页与英文首页信息结构相同，文案为真实中文，导航保持在中文下；未翻译页面以英文回退并明确标出。出处：DESIGN.docs-ssg.md「Chinese official reading pages」；计划 §6.3「中文导航/英文回退」。

### N · 组件索引与导航

- N1：`/components/` 总索引按九组（Interaction、Overlays、Menus、Navigation、Content、Feedback、Layout、Typography、Extensions）分区，列出具体组件名称，不让用户只能搜索一个旧分类词；每个条目可直达分类页锚点或旧详情页；分类/详情与 Guides 双向链接。出处：计划 §5.5 第 2 项、§6.1、§2。
- N2：当前分类/面包屑与页面标题一致；侧栏有 Explore/Components 等入口并标出当前位置（primary）；移动端导航可用（48rem 以下侧栏与目录为原生 `<details>` 面板，带旋转 chevron）；旧 24 个 `/components/<name>/` 页可直接访问。出处：计划 §6.1；DESIGN.docs-ssg.md「Layout」。
- N3：阅读壳布局：76rem 框架，36rem 正文 measure；1280px 起三栏（15rem 侧栏、正文、14rem 目录），48–80rem 两栏，48rem 以下单栏；页面不横向滚动，宽内容在自身面板内滚动；扁平 1px hairline，单一 6px 面板圆角，无阴影。出处：DESIGN.docs-ssg.md「Overview」「Layout」「The Measure Rule」「Do's and Don'ts」。

### D · 样例页面

- D1：详情结构统一为：组件用途与使用条件 → 可操作样例和适用参数 → 关键状态/组合 → 实际源码 → 公共路径/部件结构/相关指南 → Studio 对应场景入口。部件与类型辅助说明跟随所属家族；没有业务行为的布局/排版以真实语义、响应式和视觉变化证明，不放假操作按钮。出处：计划 §6.1 第 2 段。
- D2：样例 island：DOM 画布在先（1rem 内边距），之后是外部状态行和源码；源码用 Function / Full file 标签、安静的项目相对路径与 Copy，可见源码内部滚动最高 24rem；控制台（参数控件）与展示区域分开；只暴露该 API 支持的参数，不给不存在的功能加空控件；每个可操作示例有明确结果和复位；禁用/只读/加载/错误只用于对应语义；空示例与永远 loading 都是失败。出处：计划 §2 末两段、§6.3 第 2 段；DESIGN.docs-ssg.md「DOM demos and static blocks」。
- D3：Button 至少可切换全部变体（Classic、Solid、Soft、Surface、Outline、GhostOffset、Ghost）、全部尺寸（1–4，参考高度 24/32/40/48px）、颜色与高对比、disabled 与 loading；点击有结果，禁用/加载不触发；loading 保留原标签宽度并加装饰性 spinner；焦点为 2px 实线 outline。出处：计划 §5.5 第 3 项、§6.2「操作与图标」；DESIGN.md「Buttons」。 默认色族的 Solid 标签与第 9 阶表面须达 4.5:1，允许偏离参考标签色；本站不以默认开启 High contrast 代替 SDK 修复（用户 2026-11-13 决定）。
- D4：分类页内一页多个具名样例，每个新家族有独立锚点、标题、真实样本和源码；旧组件条目链接回旧详情页。出处：计划 §2 第 2 项、§5.5 第 3 项。
- D5：§6.2 中读者必须能实际看到或完成的事（逐组摘录）：
  - 基础选择：三态、单选/多选、受控回写、方向与禁用；SegmentedControl 不伪装成 Tabs。
  - 卡片选择：整卡交互、选中/禁用、键盘和表单值；可对比尺寸/变体。
  - 范围输入：多滑块、垂直/反向、值/提交、键盘、最小间距及应用拒绝请求时显示值保持。
  - 字段与表单：前后插槽、只读/禁用/invalid、数字/颜色草稿，真实提交/错误恢复/reset，展示应用最后接受值与原生表单结果。
  - 选择器：打开/选择/清空、滚动与键盘、disabled 项不可选，正确焦点返回。
  - 模态/浮层：确认/取消有不同结果，Escape/外点/嵌套与焦点，Popover 内实际操作；Tooltip 用可聚焦原生目标。 模态仅覆盖所属 scope，样例需提供足够空间让标题、描述、操作完整可读；不要求覆盖整个文档视口。
  - 菜单：点/右键/键盘入口、复选/单选/子菜单/禁用、菜单间移动、执行命令和焦点返回。
  - 导航/切换：网站链接与应用命令区分，工具栏 roving focus，Tabs/展开受控结果、单项/多项选择及键盘操作。
  - 反馈：确定/不确定进度、真实等待/空/完成反馈，Toast 更新/关闭；不只放永远转动的装饰。
  - 内容：图片失败回退、可见状态/变体、真实文本和数据；普通 Table 与 DataTable 分清用途。
  - 尺寸内容：比例随容器变化、分隔方向、横纵滚动与长内容，不用静态外框代替行为。
  - 布局：响应式间距/对齐/列数/容器尺寸，Inset 与最近容器关系，Reset 作用域不影响页面其它内容。
  - 排版：原生语义、字级/字重/对齐/裁切/换行等实际支持字段；同一文本随参数变化可见，不以 class 文本代替视觉。
  - Rustify 扩展：排序/筛选/选择，树展开与键盘，用户主动文件选择/拖放，拖动/键盘调分隔、切换面板、搜索执行命令；演示数据和操作结果可见。
  出处：计划 §6.2 表格「读者必须能实际看到或完成的事」列。

### C · Colors

- C1：选择色族/模式，看到色块、阶号和真实颜色字符串，复制值/下载匹配 CSS；覆盖全部色族的 12 阶、浅深、实色/透明、黑白透明，sRGB 与 P3；P3 标明浏览器/显示器条件与 sRGB 回退，不暗示 GPU P3。出处：计划 §6.2「Colors」行。
- C2：色块显示真实 sRGB 值并保留可读数值；可交互选择的状态随用户操作更新；内容页不为色块强制加载 Studio；复制/下载沿浏览器原生能力，失败必须对用户可见且仍能选中原始文本。出处：计划 §6.2 表后段落。

### P · Primitives

- P1：同类 raw/themed 受控样例都可操作，看到结构、状态及公共路径；示范 RTL、原生按钮/链接语义与组合；不把 Props/helper 单独凑 demo。`/primitives/` 给出无样式组合与 themed 对照和各家族入口。出处：计划 §6.2「Primitives」行、§2 第 3 项。

### S · Studio

- S1：`/studio/` 为可搜索的工作台介绍页，链接公开完整应用；不把应用伪装成静态指南、不放空 iframe；首页、主题指南和组件页可到达 Studio。出处：计划 §2 第 3 项、§5.6 第 1、7 项。
- S2：公开应用在 `/rustify-ui/theme-studio/` 与文档同包运行，提供返回官网的普通链接；保留预设、品牌/中性色族、面板、圆角和缩放配置，Colors/Typography/Other 精调，Components 场景，Inspector（有效值与根/局部/继承来源），撤销/重做，保存，导出 JSON/CSS/Rust，导入、本地库；不可替换成一个删减面板；默认读者主路径为 DOM，GPU/对比仍为可选模式并显示真实支持与错误。出处：计划 §5.6、§6.4；需求 R-5。
- S3：Studio 壳沿用 DESIGN.md：全高网格，320px 编辑器 + 弹性预览，两区独立滚动；中性编辑工具不随作者主题变色；600px 以下以 Edit/Preview 标签替代两栏。Components 场景在响应式网格上放真实样本，演示控件与组件分开。出处：DESIGN.md「Layout」「Component gallery and inspection」「Do's」。

### A · 共同主题、状态与可访问性

- A1：站点主题为 schema v2 标准主题（indigo 品牌），两模式 `primary` 与 `ring` 覆盖为 light `#3a5bc7`（标签 `#ffffff`）、dark `#9eb1ff`（标签 `#11131f`）；primary 只用于链接、当前位置、焦点、选中标签与主操作，不做装饰或标题色；页面主题切换与首帧外观一致；文档壳与局部主题互不污染，浮层继承创建样例的有效主题。出处：计划 §5.5 第 6 项（用户 2026-11-13 决定）、§6.3；DESIGN.docs-ssg.md「The One Blue Rule」。
- A2：文档壳的 `a`、`button`、`summary`、`[tabindex]` 在键盘聚焦时有不透明 3px ring、2px offset 的可见焦点；组件保持 2px outline，颜色使用本站 ring；普通文字对比 ≥4.5:1；窄屏与放大无横向页面滚动；错误资源、页面错误、空示例与永远 loading 都是失败。出处：DESIGN.docs-ssg.md「States」；计划 §6.3。

## 采集

- 计划：[DOM-COMPONENTS-RELEASE](DOM-COMPONENTS-RELEASE.md)；规范原文：仓根 `DESIGN.docs-ssg.md`、`DESIGN.md`。评审只读本目标包摘录，不沿来源读取代码、计划正文或历史报告。
- 环境：本地组合官方站 `http://127.0.0.1:4199/rustify-ui/`，由 `RUSTIFY_STUDIO_DIR=<匹配 /rustify-ui/theme-studio/ 的 Studio 构建> sh tests/docs-ssg/serve.sh official subpath 4199` 以当前工作树的 source CLI 构建 Docs，并与 Studio 经 `scripts/docs-pages-compose.py` 组合后由 `rustify-docs preview` 服务（CSP 为响应头）。被测版本取 HEAD 与工作树指纹；组合包 build_id 取 `dist/manifest.json`。无账号；演示数据为样例内置数据；不写真实业务数据。
- 条件：Playwright 自带 Chromium，DPR 1（放大格为 DPR 2）；`reducedMotion: reduce`；等待字体、网络空闲与 island ready；主题经页面自带模式开关设定；语言经页面语言菜单或中文路径进入。窄屏为 CSS 视口，不冒称实体手机；640×450 DPR 2 为 200% 放大模拟，不是原生缩放。

| 单元 · 状态 | 入口与前置 | 到达步骤 |
| --- | --- | --- |
| U1 · 英文首屏 | `/rustify-ui/` | 打开页面，等预览 island ready |
| U1 · 中文首屏 | `/rustify-ui/zh/` | 打开页面 |
| U1 · 预览操作后 | 英文首页 | 点击预览中的按钮与段控，观察状态 |
| U2 · 总索引 | `/rustify-ui/components/` | 打开，滚到「Browse by group」 |
| U2 · 窄屏导航 | 390 宽任一分类页 | 展开导航 `<details>` |
| U3 · Button | `/rustify-ui/components/button/` | 打开；改变变体/尺寸/颜色并点击；开启 disabled、loading 后点击 |
| U4 · 分类页 | `/rustify-ui/components/{interaction,overlays,menus}/` | 打开；在表单样例填写并提交；打开 AlertDialog、Popover；键盘打开 Menubar 并进入子菜单 |
| U5 · 分类页 | `/rustify-ui/components/{navigation,content,feedback,layout,typography,extensions}/` | 打开；展开 Accordion；DataTable 排序并选择；Splitter 键盘调整；排版参数切换；显示 Toast |
| U6 · Colors | `/rustify-ui/colors/` | 打开；切换色族、模式、透明与 P3；跳到下载表 |
| U6 · Primitives | `/rustify-ui/primitives/` | 打开；勾选 raw 与 themed；切换 RTL 并用方向键调 Slider |
| U7 · Studio | `/rustify-ui/studio/` → 「Open Theme Studio」 | 打开介绍页；进入应用等就绪；切到 Components 场景；用 Inspector 选一个组件；390 宽切 Edit/Preview |

## 变更记录

- 2026-11-13 R1 建立。来源为计划 §2、§5.5、§5.6、§6 与 DESIGN.docs-ssg.md、DESIGN.md；无像素目标。

- 2026-11-13 用户确认 P-1：「采用报告推荐：浅深色 ring 与 primary 一致；组件保留 2px，文档壳保留 3px，并同步目标包」。A1 补 ring，A2 明确文档壳与组件的几何边界，D3 保持 2px。

- 2026-11-13：用户确认「允许必要 SDK/Docs 修复，保持 scope 隔离；对话框完整可读，目标包注明 scope 内模态；新版本发行另行处理」。D5 的模态口径限定所属 scope，必要的 SDK/Docs 本地缺陷修复纳入本轮。
- 2026-11-13：用户确认「要求修改 SDK 的默认标签色达到 AA；纳入必要本地修复，新版发行另行处理」。D3 默认色族的 Solid 标签与第 9 阶表面须达 4.5:1，允许偏离参考标签色；不以本站默认开启 High contrast 代替 SDK 修复。
