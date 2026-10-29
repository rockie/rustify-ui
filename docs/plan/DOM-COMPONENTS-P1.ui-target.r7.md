# DOM-COMPONENTS-P1 · UI 验收目标包

- 需求来源：[DOM-COMPONENTS PRD](../PRD-DOM-COMPONENTS.md) P1（§2.5、§3 的 P1 AC、§6.3 UAT）；计划 [DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md) §5.5–§5.8、§6、§9.3。
- 范围：本期交付的可见界面——Theme Studio 的配置—精调—预览—导入导出—本地库闭环（`examples/theme-studio`），以及新 DOM 组件 Button/IconButton/Link/ThemePanel 与布局排版（`examples/component-catalog` 组件夹具）。
- 不在本包：Colors 静态资产与独立 DOM/Docs 消费者（无自有界面，数值一致性由计划 V1/V8 的机器检查承担）；GPU 新能力（P1 不新增）；P2–P5 组件、完整 Inspector（F19）、Primitives 演示；人工读屏与 OS IME。
- 模式：U1–U5 为**规范模式**（无设计稿，期望摘自下列规范原文）；U6、U7 为**还原模式 + 规范**，目标图为 Radix Themes 3.3.0 本地快照 playground 的视觉回归截图（PRD §1.5 指定的参考基线 `ref/themes-main`），已复制到 [DOM-COMPONENTS-P1.ui-target/](DOM-COMPONENTS-P1.ui-target/)。

## 单元

| 单元 | 状态 | 视口 × 主题 | 目标 | 比较口径 |
| --- | --- | --- | --- | --- |
| U1 Studio 工作台框架与基础配置 | 首屏默认；选预设后改品牌色（出现覆盖标识）；预览模式切深色；中文界面；200% 文本缩放；键盘走查（预设搜索、基础配置选择器、模式按钮的焦点帧） | 1440×900、768×1024、390×844（编辑页与预览页）× 预览 light；1440 追加预览 dark | 规范要点 G1–G6、C1–C4 | 规范模式。主题名与预设内容是示例数据；壳层字号、颜色、间距、区域顺序、文案语义与状态标识须符合规范 |
| U2 Studio 精调与来源解释 | Colors 页选中 token 显示来源；被覆盖 token 的来源与单项解除；分组解除；字体与其他页；字段非法输入错误；低对比诊断 | 1440×900 × light；390×844 × light | 规范要点 F1–F4、G1–G4 | 规范模式。token 值是作者数据；来源、覆盖、重置作用域与错误位置须可从截图读出 2026-10-28 用户就 UI-115 裁定（原话「保留现在这样就可以」）：保留颜色项右端的 ↺ 按钮，沿用现有模式标识、title／可访问名称及来源区范围说明；不要求按钮或列表常显“仅浅色”等范围文字。本接受仅限 UI-115，不改变 G4 的可见键盘焦点要求。 |
| U3 Studio 导入与导出 | 导入候选审阅（合法 v1 JSON）；导入错误（未知版本）；导出完整 JSON；兼容 v1 导出的损失清单；CSS 导出；Rust 接入说明；复制反馈 | 1440×900 × light；390×844 × light（导出与导入审阅） | 规范要点 X1–X6、G1–G4 | 规范模式。导出正文内容是主题数据；版本、损失清单、差异与诊断的呈现须符合规范 |
| U4 Studio 本地库迁移与存储反馈 | 旧库升级待确认；升级 Review；取消后仅内存；升级成功；写入失败（配额）；跨标签冲突；无 Web Locks | 1440×900 × light；390×844 × light（待确认） | 规范要点 S1–S6、G1–G4 | 规范模式。旧库与冲突数据是种子数据；状态反馈、可继续动作与出口的可见性须符合规范 |
| U5 Studio 预览：真实组件、嵌套边界、浮层与拾取 | DOM 预览默认；预览深色；对比区嵌套相反模式；预览菜单打开；预览对话框打开；拾取选中部件；DOM+GPU 对比 | 1440×900 × light、dark；390×844 × light | 规范要点 P1–P5、G1 | 规范模式。预览场景内容是演示数据；仅 Button/IconButton/Link/排版属于 P1 新组件，其余预览控件为既有组件，不按 Radix 还原比较；拾取只验收既有 token 入口与 P1 来源说明，不按 F19 完整 Inspector 要求。2026-10-27 用户把 U6 对 UI-100 的接受扩到 U5、U7（原话「把接受范围扩到 U5、U7」）：P1 新组件按参考选阶或作者主题旧 token 对得到的前景/背景对比低于 4.5:1 时不计差距，例如 Code soft 用 primary 压其浅色底；但实现偏离参考选阶的仍计差距 |
| U6 Button / IconButton / Link 与内嵌 ThemePanel | 夹具默认；深色；加载中；键盘焦点帧；缩放 110%；圆角 full；ThemePanel 中文 | 1280×900 × light、dark；390×844 × light | [radix-sink-button.png](DOM-COMPONENTS-P1.ui-target/radix-sink-button.png)、[radix-sink-icon-button.png](DOM-COMPONENTS-P1.ui-target/radix-sink-icon-button.png)、[radix-classic-button.png](DOM-COMPONENTS-P1.ui-target/radix-classic-button.png)、[radix-sink-link.png](DOM-COMPONENTS-P1.ui-target/radix-sink-link.png)；规范要点 B1–B6、C3 | 还原模式比较**每个变体/尺寸的组件外观**：填充/描边/文字处理、四档高度与内边距、圆角、high-contrast、disabled、ghost/ghost-offset、链接下划线与字级。不比较：夹具页面的排列、行列标签与说明文字、测试用控制按钮；品牌色色相（夹具使用自身配置）；字体字族（随有效主题）；目标图为深色，浅色以同一变体规则对应。2026-10-27 用户接受（原话「UI-100： accept」）：按参考选阶得到的前景/背景对比低于 4.5:1 时不计差距，包括色族 solid 白字压第 9 阶、语义 accent 对 #12b76a / #ffffff；但实现偏离参考选阶的仍计差距，例如次级变体文字没用第 11 阶 |
| U7 布局与排版 | 夹具默认；缩放 110% 与 95%；非法属性诊断 | 1280×900、390×844 × light | [radix-sink-heading.png](DOM-COMPONENTS-P1.ui-target/radix-sink-heading.png)、[radix-sink-text.png](DOM-COMPONENTS-P1.ui-target/radix-sink-text.png)、[radix-sink-code.png](DOM-COMPONENTS-P1.ui-target/radix-sink-code.png)、[radix-sink-kbd.png](DOM-COMPONENTS-P1.ui-target/radix-sink-kbd.png)、[radix-sink-blockquote.png](DOM-COMPONENTS-P1.ui-target/radix-sink-blockquote.png)、[radix-text-wrap.png](DOM-COMPONENTS-P1.ui-target/radix-text-wrap.png)、[radix-sink-container.png](DOM-COMPONENTS-P1.ui-target/radix-sink-container.png)、[radix-inset.png](DOM-COMPONENTS-P1.ui-target/radix-inset.png)、[radix-reset.png](DOM-COMPONENTS-P1.ui-target/radix-reset.png)；规范要点 L1–L6 | 还原模式比较字级梯度、字重、行高、Code/Kbd/Blockquote 的外观、Inset 贴边与 Reset 的作用范围。不比较：页面排列、示例文案与图片、深浅色差异、字族；Radix 未交付到 P1 的组件（Card、Checkbox Card 等）只作为 Inset/Reset 的承载背景，不要求出现。2026-10-27 用户把 U6 对 UI-100 的接受扩到 U5、U7（原话「把接受范围扩到 U5、U7」）：按参考选阶或旧 token 对得到的前景/背景对比低于 4.5:1 时不计差距，例如 Code solid 白字压第 9 阶；但实现偏离参考选阶的仍计差距 |

## 规范要点

### G · 共同编辑壳（Studio）

- G1：稳定中性编辑壳、左侧编辑/右侧预览、各区独立滚动、窄屏编辑/预览切换；该规范约束编辑工具，用户预览使用作者自己的主题。编辑壳不随预览主题失去可读性；预览深色或低对比主题时编辑工具保持中性浅色。出处：PRD §2.5 首段；WORK-F18/AC-1；DESIGN.md「Do's」第一条与 The Scope Rule。
- G2：Studio 布局为 `100dvh` 全高网格，320px 编辑列与弹性预览，两区独立滚动；分隔条 8px 可拖拽目标，编辑列 260–520px。编辑区 padding 24px 20px 48px、栈间距 16px；预览 padding 24px 24px 48px。壳层用细分隔线与分组行，不是每字段一张卡片。断点：≤1100px DOM/GPU 并排改为上下堆叠；≤800px 编辑区水平 padding 16px、预览 20px 16px 48px、指标改两列、主标题 16px；≤600px 用 Edit/Preview 标签替换双栏、隐藏分隔条、预览有「返回编辑」；≤560px 导入导出表单行单列、本地库行堆叠。出处：DESIGN.md「Layout」及其断点表、Typography。
- G3：壳层字体 IBM Plex Sans；body 14px / 1.5；标题 18px / 1.3 / -0.02em；预览标题 16px / 600；label 13px / 600；caption 12px；颜色值等宽 12px。壳层分隔 #d0d5dd / #eaecf0，次级文字 #475467，搜索框边框 #98a2b3，选中背景 #eff8ff、选中边框 #84caff、选中文字 #175cd3，字段错误 #b42318。蓝色用于选中与焦点。壳层扁平，不用装饰阴影区分列。出处：DESIGN.md frontmatter（shell-*、typography）、Colors、Elevation。
- G4：新增控件键盘可用、焦点可见、无装饰动画依赖。壳层搜索框焦点为 3px 半透明蓝 outline、2px offset；字段型标签页为扁平下划线（选中 2px 蓝色下边框、13px 半粗）并有独立可见焦点；选中的模式/渲染器按钮使用按下态。出处：计划 §6 末条；DESIGN.md Inputs、Navigation。
- G5：顶部保留主题名、模式、撤销/重做/保存/导出；保存状态以实际结果为准，失败后仍能导出。出处：计划 §6 第一条；PRD §2.5 表「顶部」行。
- G6：新增配置/错误/迁移文案同步中英文；不翻译主题名、token 与作者数据。390px 编辑/预览分区、768px/桌面与 200% 放大下主路径可达，不依靠整页横向滚动到达关键动作；菜单和对话框在可用视口内可操作。出处：计划 §6 第三条与末条；PRD §2.5 表「窄屏/放大」行。

### C · 基础配置与 ThemePanel

- C1：左侧顺序为预设搜索 → 基础配置 → 精调；右侧为真实预览与既有场景/renderer 切换。少量高层选择先于 token 列表。出处：计划 §6 第一条；PRD §2.5 表「基础配置」行。
- C2：基础配置为品牌色、中性色、面板、圆角、缩放，外观与预览模式语义标清；选择器有明确标签与有效 auto 结果（例如中性色 auto 显示实际配对的灰阶名）。选项：品牌色 26 项；中性色 auto + gray/mauve/slate/sage/olive/sand；外观 inherit/light/dark；面板 solid/translucent；圆角 none/small/medium/large/full；缩放 90/95/100/105/110%。出处：计划 §6 第一、二条；PRD §2.4「快捷配置」。
- C3：Custom 与 overridden 以文字 + 状态标识显示，不只靠颜色或悬停；显示「配置已选择，N 项被覆盖」类说明；不伪装成未修改预设。ThemePanel 在 Studio 基础区与可嵌入样例中表达相同选项。出处：计划 §6 第二条、§5.2 第三条；PRD §2.5 表「基础配置」行；WORK-F3 正文。
- C4：只改预览模式不修改作者数据，界面显示「预览模式」以免与作者外观混淆。出处：计划 §3.1「appearance=inherit」后一条；WORK-F18/AC-2。

### F · 精调与来源

- F1：精调沿用 Colors / Typography / Other（字体与其他）分区；展示有效值、来源、覆盖与重置；重置有明确作用域；错误贴近字段；未完成输入不破坏有效预览。出处：PRD §2.5 表「精细编辑」行。
- F2：来源说明能回答「值来自哪里」「哪个覆盖在生效」「解除将影响哪些项」。重置分为「解除该 token」「解除分组覆盖」「恢复预设/文档」，须显示范围，不自动清空其他模式或分组。出处：计划 §6 第二条、§5.2 第三条。
- F3：自定义低对比主题保留作者颜色，在预览侧给诊断与可定位来源；编辑壳始终可读，不以警告为由静默改色或阻止 JSON 取回。出处：PRD §2.5 表「自定义低对比主题」行。
- F4：颜色 token 与其重置按钮合成一个外缘圆角的分体控件；颜色值等宽字；拾色字段焦点为 picker 蓝 40% outline（3px，2px offset），非法输入使用壳层错误边框；长 token 名和错误文字换行。出处：DESIGN.md Shapes 第二段、Inputs 第二段。

### X · 导入与导出

- X1：导入先呈现候选差异、版本和诊断——版本、名称、配置、两模式 base 与 override 新增/删除及诊断；Apply 后才替换；取消、语法错误、超限或不支持的颜色输入均保留当前主题与历史。出处：WORK-F20/AC-2；计划 §5.8 第三条。
- X2：默认完整 JSON 导出 v2。显式「兼容 v1」先列具体损失：色族/auto 选择、appearance 意图、radius full/组件尺度角色、scaling 意图、覆盖来源、非旧 49 键派生角色，且需用户显式选择后才导出。出处：WORK-F20/AC-4；计划 §5.8 第四条。
- X3：CSS 提供标准 Tailwind v4 与 Rustify scoped 两种 profile 及四种颜色记法；标明 CSS 只是已解析结果、不能恢复全部作者意图。出处：计划 §5.8 第五条；WORK-F20/AC-4。
- X4：接入说明标明所需 SDK/CLI 版本与外部字体资源；写明旧 registry 0.2.1/旧 CLI 不能消费 v2；「按当前预览接入」的说明标出所选 preview 模式。出处：WORK-F20/AC-3；计划 §5.8 第六、七条。
- X5：复制成功、下载实际发起后才报成功；失败保留可选文本与下载路径。出处：计划 §5.8 末条；WORK-F20/AC-4。
- X6：导入/导出对话框宽高为视口减 2rem、最大宽 64rem、内部纵向滚动；导出代码等宽 .8125rem / 1.6；对话框为 popover 色带边框圆角面板、1.5rem padding、30% foreground 遮罩；≤560px 表单行单列。出处：DESIGN.md Layout 末段、Typography、Dialogs。

### S · 本地库迁移与存储

- S1：迁移待确认、缓存损坏、外部冲突、只读持久化、导入失败、复制失败均有常驻反馈与继续动作；保存失败不显示已保存；下载原始数据与当前主题出口始终可达。出处：计划 §6 第四条。
- S2：旧库先恢复旧草稿到内存，再 Review、显式「升级并保存当前草稿」或取消；取消后常驻「仅内存编辑」与重新检查入口；模式/偏好变化不能绕过确认。出处：计划 §6 第五条、§3.3 升级状态表。
- S3：升级 Review 展示原 v1 库的完整迁移结果及将保存的当前草稿/偏好；提交成功前不显示升级/保存成功；提供「下载升级前原始数据」。出处：计划 §3.3 升级状态表「已确认待提交」「提交成功」行与末条。
- S4：保存状态区分排队、先前快照成功而当前仍待保存、当前草稿确已保存；命名库动作单独反馈；待锁不禁用编辑/历史/导出。出处：计划 §6 第五条、§3.3「当前草稿已保存」条。
- S5：跨标签冲突时 Load / Keep / Save copy 显式决策，展示冲突版本，不静默覆盖或合并；未处理的命名动作保持可见。出处：WORK-F21/AC-3；计划 §3.3 Load/Keep/Save copy 条。
- S6：无 Web Locks、storage 不可用或配额失败时明确「仅内存编辑/可下载」，本地写操作禁用或报真实失败；写失败保留内存草稿且仍可导出。出处：计划 §3.3 无锁条；WORK-F21/AC-2。

### P · 预览

- P1：预览使用真实组件、变体、尺寸和交互状态；真实操作可用；演示状态与被禁用的真实状态清楚区分，不放假按钮。出处：PRD §2.5 表「组件预览」行。
- P2：Inspector（拾取）从实际部件定位相关色阶、语义角色及覆盖来源；找不到映射时明确显示未标注，不编造 token；浮层同样可检查。选中矩形为固定蓝色 outline，信息面板为浅蓝并带 token 按钮。出处：PRD §2.5 表「Inspector」行；DESIGN.md「Dialogs, diagnostics and inspector」第二段。
- P3：嵌套边界可选择与根相反的外观；局部浮层随创建处的有效主题更新；修改任一区域不改变其边界外的主题与宿主元素。出处：WORK-F3/AC-2、WORK-F4/AC-2；DESIGN.md Colors 中 ThemeScope/ThemeBoundary 段。
- P4：编辑工具保持中性浅色，作者颜色只出现在预览 scope 及其浮层内；GPU 状态与恢复文字跟随编辑器语言，失败可见。出处：DESIGN.md The Scope Rule、Dialogs 第二段。
- P5：预览宽度选择器模拟 1024/768/390 内容宽度，自动宽度跟随可用空间；≤1100px DOM/GPU 上下堆叠。出处：DESIGN.md Layout。

### B · Button / IconButton / Link

- B1：Button size 1–4（默认 2）；variant classic/solid/soft/surface/outline/ghost-offset/ghost（默认 solid）；color 与 radius 未设时继承；high_contrast、loading。四档参考高度 24/32/40/48px（缩放前）；ghost 保留自身 offset 几何。出处：计划 §5.5 表；DESIGN.md Buttons。
- B2：键盘焦点按 Radix Themes 3.3.0 的规则，即 `:focus-visible` 时 2px 实线 outline：
  - classic、solid：焦点色第 8 阶（`--focus-8`，对应本仓 ring / 组件色第 8 阶），offset 2px。
  - surface、outline、ghost、ghost-offset：同色，offset −1px。
  - soft：组件色第 8 阶，offset −1px。
  - 设了 color 的组件（gray 除外）用组件自身色族的第 8 阶。
  - IconButton 同 Button。
  - Link：2px 实线焦点色第 8 阶，offset 2px，圆角 `0.07em × radius-factor`，聚焦时隐藏下划线。
  - 出处：用户 2026-10-27 裁定，原话「Link + Button follows Radix」；`ref/themes-main/packages/radix-ui-themes/src/components/_internal/base-button.css`（classic/solid/soft/ghost/outline/surface 的 `:focus-visible`）、`components/link.css`（`:focus-visible` 及「all focused states underline」段）。
  - 本条取代 DESIGN.md Buttons 第二段的「3px 半透明环」。
- B3：loading 保留原名称与宽度，增加装饰性 spinner，阻止点击；disabled 永远优先；unavailable 保持可聚焦的说明性请求，不变成原生 disabled。出处：计划 §5.5 第一条；DESIGN.md Buttons 第二段。
- B4：IconButton 与 Button 同视觉选项，正方形尺寸对应同档高度，需可访问名称。出处：计划 §5.5 表；DESIGN.md Buttons。
- B5：Link size 1–9、weight、trim、wrap；underline auto/always/hover/none（默认 auto）；color/high_contrast、truncate；默认不覆盖继承字号；disabled 移除 href。出处：计划 §5.5 表。
- B6：未选 color 的 solid Button 与 Link 消费有效 primary/foreground；组件选色只改变该组件负责的色阶；缩放同时影响字级、间距和控件尺寸，圆角 full 不改变必要形状。出处：计划 §5.2 第三条；WORK-F3/AC-3。

### L · 布局与排版

- L1：Text（span/div/label/p，size 1–9 默认继承）、Heading（h1–h6 默认 h1，size 1–9 默认 6，视觉大小不决定层级）、Blockquote、Code（solid/soft/outline/ghost 默认 soft）、Kbd（classic/soft 默认 classic）、Em/Quote/Strong 均可消费，且具有正确语义外观。出处：计划 §5.6 表；WORK-F16/AC-1。
- L2：字重 light/regular/medium/bold，对齐 left/center/right，trim normal/start/end/both，wrap wrap/nowrap/pretty/balance；truncate 保留完整文本并显示省略。出处：计划 §5.6 末段；DESIGN.md Typography 末段。
- L3：Box/Flex/Grid 保持 div/span 语义；Container size 1–4（内容最大宽 448/688/880/1136px）与 left/center/right 对齐；Section 纵向 24/40/64/80px；Inset 只消除最近容器声明的 padding，使内容贴边，不成为任意负 margin。出处：计划 §5.6 表；DESIGN.md「Reusable DOM layout」。
- L4：Reset 只作用于调用方提供的单个原生元素，保留语义，不加包裹层，不重置宿主或其他 scope。出处：计划 §5.6 表；WORK-F16/AC-2。
- L5：响应式断点为最小宽度 0/520/768/1024/1280/1640px；长文、长链接和容器边界不遮住内容。出处：计划 §5.6 首段；WORK-F16/AC-2。
- L6：主题缩放只作用于参考尺度（字级、行高、主题间距、有限圆角、控件尺寸）一次；显式 CSS 尺寸与断点不随缩放变化。非法值给出属性定位诊断，组件默认保持可用。出处：计划 §5.3、§5.4 末条；DESIGN.md「Reusable DOM layout」末段。

## 采集

- 计划：[DOM-COMPONENTS-P1](DOM-COMPONENTS-P1.md)。评审只读本目标包摘录与目标图，不沿来源读取代码、计划记录或旧验收截图。
- 环境：`mbx xtask build-web --example theme-studio --release` 与 `--example component-catalog --release` 从被测版本重新构建；`mbx xtask serve --example theme-studio --release --port 4181`、`mbx xtask serve --example component-catalog --release --port 4176`（strict CSP）。被测版本记 HEAD 与产品路径 diff；构建 ID 取产物 `build-manifest.json`。无账号；浏览器新 context，localStorage 为空，除 U4 外不预置数据。
- 条件：Playwright 1.63.0 Chromium（headless），DPR 1，`reducedMotion: reduce`，等待 `#status[data-status=ready]`、字体加载完毕、网络空闲与连续 500ms 无长帧；截图为可视区域。语言默认英文，中文态通过界面语言按钮切换。
- 入口：Studio 为 `http://127.0.0.1:4181/` 正常页面；组件夹具为 catalog 页面 `http://127.0.0.1:4176/` 载入后调用页面已暴露的 `window.__component_catalog.mount_button_fixtures()` / `mount_layout_fixtures()`——这是本仓组件验收的既定沙箱入口（计划 V4），夹具内控制按钮为真实组件操作。

| 单元 · 状态 | 入口与前置 | 到达步骤 |
| --- | --- | --- |
| U1 · 首屏默认 | Studio，空存储 | 打开页面，等就绪 |
| U1 · 改品牌色/覆盖标识 | 同上 | 预设搜索 Graphite → 选择 Built-in · Graphite → 品牌色选 pink、中性色选 Auto，滚动到基础配置 |
| U1 · 预览深色 | 同上 | 点模式 Dark |
| U1 · 中文 | 同上 | 点 中文 |
| U1 · 200% | 1440 打开后改为 720×900 视口并设根字号 200%（WCAG reflow 手法） | 键盘到精调页，打开导出；另开一页同条件点精调 Colors，把首个 token 滚到编辑列顶部拍网格，再选 primary，把来源块滚到顶部再拍 |
| U1 · 窄屏 | 390×844 | 编辑页首屏；点 Preview 进入预览页 |
| U2 · 来源 | Graphite + pink 状态 | 精调 Colors 页，选 primary（被覆盖）与 ring/其他生成值 token；点单项解除；点品牌分组解除；另把编辑列滚到「Restore document」入口再拍 |
| U2 · 字段错误 | 默认 | 字体与其他页，radius 填 `abc` 回车 |
| U2 · 低对比 | 默认 | background、foreground 均设为 #ffffff，展开诊断 |
| U3 · 导入审阅/错误 | 默认 | 点 Import，粘贴合法 v1 JSON 点 Review；另填 `{"schema_version":999}` 点 Review；390 另把 Identity 组滚到视口中部再拍 |
| U3 · 导出 | Graphite + pink | 点 Export，依次切换完整 JSON、兼容 v1、Tailwind CSS、Rust；点复制；Rust 另把代码滚到纵向中点再拍 |
| U4 · 升级待确认/Review/取消/成功 | localStorage 只预置 `rustify-ui.theme-studio.v1` 合法旧库（两个命名主题） | 重开页面；点升级 Review；取消；再 Review 并确认；另开一页点 Review 后展开完整迁移 JSON 再拍 |
| U4 · 配额失败 | 正常 v2 库 | 页面内让 `Storage.prototype.setItem` 抛 QuotaExceededError（只改存储环境，不改界面代码），命名保存 |
| U4 · 冲突 | 正常 v2 库 | 同源第二页修改并保存，回到第一页 |
| U4 · 无 Web Locks | 页面载入前移除 `navigator.locks` | 编辑并尝试命名保存 |
| U5 · 预览/浮层/拾取 | 默认或 Graphite + pink | 打开预览菜单、对话框；预览内局部边界外观选「Opposite to root」；展开对比区切 Dark；点 Inspect 后点预览主按钮；点 DOM + GPU |
| U6 · 各状态 | catalog 按钮夹具 | 默认；点夹具 Dark；点 loading 切换；Tab 聚焦主按钮；点 scale 110；点 radius full；点 ThemePanel 中文 |
| U6 · B2 焦点特写 | catalog 按钮与布局夹具，DPR 2 | 各变体 size 2、red、High contrast、IconButton、Link 分别用 Shift+Tab、Tab 键盘聚焦，截取控件四周 24px；夹具 Dark 下再拍 solid、soft、outline |
| U7 · 各状态 | catalog 布局夹具 | 默认；点 scale 110 / 95；点 invalid；另截 Heading、Text、Code、Kbd、Blockquote、Wrap、Container、Inset、Reset 各分段元素图。整段夹具帧（默认、110%、95%、invalid）不用元素截图，改为页面裁切：夹具根 boundingBox 四边向外取整到整数 CSS px，再四周各留 8px；留边内的像素在夹具根之外 |

补充采集（R7）：全量沿用 R6 已补齐的状态 × 视口 × 主题组合。已配置 Pink 后，1440 深色、768 与 390 浅色分别用键盘展开品牌色列表，拍摄移动、确认和 Escape 返回；390 导出补齐 v1、CSS、Rust、复制反馈与未知版本导入错误。U4 另拍升级 Review 与确认按钮的真实键盘焦点、Escape 返回、完整升级 JSON 的中段/末尾及冲突原文末尾。上述只增加采集，不改变单元和规范期望。

## 变更记录

- 2026-10-27 R1 建立。期望来源为 PRD P1、计划 §5–§6、DESIGN.md 与 Radix Themes 3.3.0 本地 playground 视觉回归截图（`ref/themes-main/apps/playground/tests/__snapshots__/visual-regression.spec.ts-snapshots/*-chromium-darwin.png`，2026-10-27 复制）。首次基线待用户确认。
- 2026-10-27 R1 补采（只改采集做法，单元与规范要点不变）：对话框与长列表的下半部滚动到动作区再拍；下拉选项需展开拍；键盘焦点帧和缩放帧拍摄前把指针移到 (0,0)；拾取态在开启后等待再悬停/点击；U6 链接取自布局夹具的 `type-link` 区；U7 非法属性态拍整段夹具。
- 2026-10-27 R1 之后，用户裁定 UI-100，原话「UI-100： accept」。U6 比较口径追加一条：按参考选阶或旧 token 对得到的对比度不计差距。这一改动改变了目标指纹，R2 按重建基线评审。
- 2026-10-27 R1 之后，用户裁定 UI-85。用户先问「UI-85：how about radix-ui?」，再决定「Link + Button follows Radix」。规范要点 B2 由「3px 半透明环、2px offset（DESIGN.md）」改为 Radix Themes 3.3.0 的 Button、IconButton 与 Link `:focus-visible` 规则。R2 按重建基线评审。
- 2026-10-27 R2 采集（只改采集做法，单元与规范要点不变）：R1 补采做法并入首采；新增 U5 局部边界相反外观帧、U6 B2 焦点 DPR 2 特写、U7 分段元素图；Link 聚焦改用键盘；选项文字按界面现名选取（品牌色「Pink」、导出「CSS · resolved values」）。
- 2026-10-27 R2 补采（只改采集做法）：对话框内被裁切的滚动区（含导入差异表）滚到底再拍；200% 导出对话框滚到动作区；冲突面板底边对齐视口底部，再拍其后的状态区；深色预览滚到 Caller class override；局部边界相反外观下从边界内打开菜单与对话框；拾取未声明部件后把拾取面板滚入视野；U1 补拍字段标签页（含键盘焦点）与基础配置下半段。
- 2026-10-27 R2 之后，用户裁定 UI-156，原话「把接受范围扩到 U5、U7」：U5、U7 比较口径追加与 U6 相同的参考选阶 / 旧 token 对对比度条款。这一改动改变了目标指纹，R3 按重建基线评审。
- 2026-10-27 R2 之后，用户裁定 UI-97，原话「改采集方式」：U7 整段夹具帧改为取整并留边 8px 的页面裁切（只改采集节，指纹不变）。
- 2026-10-27 R2 之后，用户裁定 UI-88、UI-89，原话「当作缺陷补齐字面」：期望不变（Heading、Strong 应为粗体），由修复方处理；目标包「单元」「规范要点」不因此改动。
- 2026-10-27 R3 采集（只改采集做法，单元与规范要点不变）：R2 补采做法并入首采；U4 冲突面板增拍「Draft differences」展开帧；U7 整段夹具帧按上条改为页面裁切。
- 2026-10-28 R3 补采（只改采集做法）：U2 加拍基础配置后编辑列顶部的「Restore document」入口；U3 加拍 Rust 接入说明滚到首个字体文件处、390 导入审阅滚到 Identity 组；U4 升级 Review 展开「Complete migration JSON」再拍。
- 2026-10-28 R4 采集（只改采集做法，单元与规范要点不变）：R3 补采做法并入首采（Rust 中段改为滚到代码纵向中点；完整迁移 JSON 按界面现名选取）；U1 · 200% 增拍精调 Colors 网格与所选 token 来源块（R3 机器检查发现 UI-192 的位置，R3 报告建议并入）。
- 2026-10-28 R4 补采（只改采集做法）：U1 · 200% 选 primary 后先等页面静止再把来源块滚到顶部（首拍时界面自身的选中滚动在脚本滚动之后发生，来源块落在折线以下）；来源块高于视口时另拍其末尾（含解除按钮）。
- 2026-10-28 R5 采集（只改采集做法，单元与规范要点不变）：沿用 R4 全矩阵及补采步骤；U2 另开默认页，进入 Colors 后把颜色搜索框与 Clear 一行滚入视野并拍摄，供跨单元的同一搜索行复核。
- 2026-10-28 R5 补采（只改采集做法）：独立评审看见分组重置影响列表的折叠入口，但首采未展开；另开 Graphite + Pink + Auto 页，展开该列表并拍摄首尾，原图保留。
- 2026-10-28 R5 机器检查补采（只改采集做法）：U3 导入/导出对话框增加真实键盘到格式选择器的焦点帧；CSS profile、颜色记法同样拍摄；每次停留 700ms 后再拍，另拍下一控件作可见焦点对照。后续采集沿用这些步骤，规范 G4 不变。
- 2026-10-28 R5 之后用户裁定 UI-115，原话「保留现在这样就可以」：保留当前重置图标与提示，只接受不常显范围文字这一处偏差；已写入 U2 比较口径。不是整体界面豁免，UI-215 及其他差距仍按 R5 报告处理。R5 原审目标冻结于 [R5 目标快照](DOM-COMPONENTS-P1.ui-target.r5.md)，下一轮按新指纹重建基线。
- 2026-10-29 R7 采集：沿用 R6 全矩阵，新增上述配置后键盘连续操作、390 导出各格式与错误、U4 焦点和长文本中末段；只修改采集节，目标指纹不变。独立评审分别使用本轮采集前的 R6 冻结目标或本轮 R7 快照，两者「单元」「规范要点」及目标附件一致。
