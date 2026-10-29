# DOM-COMPONENTS · Rustify UI · DOM 组件体系重构（PRD）

> 版本：v0.2 · 日期：2026-10-26 · 状态：设计草稿，范围与兼容策略已确认；已按完整评审意见修订故障与恢复边界，可交接，复审按需。
> 已确认：允许破坏性调整旧 DOM API，必须提供迁移说明；不长期维护两套组件体系。
> 已确认：以三份参考源码的稳定公共能力为完整目标，分期交付；保留 Rustify 已有扩展，unstable 能力单列候选。
> 呈现路径：DOM。复用现有共享主题语义，保持 Theme Studio 已有 GPU 预览可用，不新增 GPU 组件或要求 DOM/GPU 视觉对等。
> 本文定义产品能力、行为与验收。包拆分、Rust 签名、主题格式字段、内部实现和测试脚本由后续开发计划决定。WORK-F# / WORK-N# 是本文工作号，未写入 SPMS，不是平台需求编号。

## 0. 一句话

让 Rustify 应用开发者可以分别选用颜色、无样式交互组件或成套主题组件，并通过现有 Theme Studio 完成选主题、精调、验证和导出，以同一套行为与主题契约构建 DOM 应用。

## 1. 背景、目标与边界

### 1.1 现状与问题

Rustify 已有受控组件、作用域主题、浮层和主题编辑器。本次重构的价值是把这些能力组织成可组合、可独立使用的体系，并补齐参考体系中的公共能力。

| 已核实事实 | 对使用者的影响 | 来源 |
| --- | --- | --- |
| 当前组件库默认使用 Leptos CSR，也提供 SSR、hydrate；依赖的 DOM 核心不含 Makepad | 重构必须保持纯 DOM 接入和静态文档消费，不能把 GPU 变成组件库前置 | [组件清单](../crates/rustify-components/Cargo.toml)、[DOM 核心说明](../crates/rustify-dom/README.md) |
| CATALOG 有 24 类能力；公共导出还包括 Form、文件选择、Splitter、PanelTabs、CommandPalette 等 | 24 是目录分类数，不是全部公共组件数；不能按 Radix 清单删除本项目扩展 | [分类事实源](../crates/rustify-components/src/catalog.rs)、[公共导出](../crates/rustify-components/src/lib.rs) |
| 当前 Dialog 把标题栏和关闭按钮固定在组件内，Select 接收选项数组，组件同时提供样式与行为 | 常规使用方便，但独立组合部件和建立另一套设计语言的入口不足 | [Dialog](../crates/rustify-components/src/dialog.rs)、[Select](../crates/rustify-components/src/select.rs) |
| 当前 Checkbox 是二态；Slider 是单值原生 range | 不能直接视为已覆盖三态选择和多滑块范围输入 | [Checkbox](../crates/rustify-components/src/checkbox.rs)、[Slider](../crates/rustify-components/src/slider.rs) |
| ThemeDocument v1 每个模式要求 49 个 token，保留作者字符串；共享解析结果为 sRGB，当前不接受 Display P3 | 不能直接塞入 Radix CSS 并声称主题编辑、导出和 GPU 都已支持；新增主题配置需要明确交换和迁移规则 | [主题指南](themes.md)、[主题文档](../crates/rustify-dom/src/theme/document.rs)、[解析结果](../crates/rustify-dom/src/theme/resolve.rs) |
| Theme Studio 已有 44 个内置主题、双模式编辑、DOM/GPU/对比预览、Inspector、撤销重做、本地库与 JSON/CSS/Rust 导出 | 本次应扩展已有工作流；现有预设和已保存主题也是迁移对象 | [Theme Studio](../examples/theme-studio/README.md)、[编辑状态](../examples/theme-studio/src/state.rs) |
| Rustify 的 primary 与 accent 是不同语义；Radix Themes 的 accentColor 控制主要交互色 | 同名术语不可直接替换，否则已有主题中的绿色 accent 可能变成品牌主色 | [当前主题](../crates/rustify-dom/src/theme.rs)、[Radix Theme 配置](../ref/themes-main/packages/radix-ui-themes/src/components/theme.props.tsx) |
| 现有浮层属于 mount scope，继承创建位置的有效主题；modal 使本 scope 的底层内容 inert | 应保留作用域边界，不照搬影响整个宿主页的重置、主题或浮层逻辑 | [运行时契约](architecture.md)、[浮层](../crates/rustify-dom/src/overlay.rs) |

以上为源码与文档调查，不代表本轮执行了组件、浏览器或性能验收。旧计划中描述的历史缺口不作为当前缺口重复立项。

### 1.2 目标

- **G1 可分层采用。** 开发者能独立消费 Colors、Primitives、Themes；选择无样式入口时不被附带品牌外观，选择主题组件时无需重写交互。
- **G2 一个主题闭环。** 开发者和设计人员能在 Theme Studio 选色族、调整语义值、检查真实组件并导出；独立消费者得到与所选模式、主题配置和覆盖一致的结果。
- **G3 可组合且可访问。** 本文覆盖表中的组件均有明确结构、状态和键盘语义；组合、嵌套、禁用和应用拒绝变更时，显示状态与外部状态一致。
- **G4 可迁移且不丢数据。** 旧公共入口均有去向，旧主题、预设和本地草稿可升级；无法无损表达的降级必须提前说明，不显示虚假的兼容成功。
- **G5 保持 Rustify 的部署和运行优势。** 默认 DOM 应用无 Makepad/React 运行依赖，保留静态托管、严格 CSP、SSR/hydrate 和资源外置；热路径改动报告同条件前后数据。

### 1.3 非目标

| 不在本文范围的内容 | 去向与理由 |
| --- | --- |
| 为 Radix 全部组件新增 Makepad/GPU 实现、GPU 广色域或任意 CSS 兼容 | 属于独立 GPU 需求；本文只保持共享主题已有投影和现有预览 |
| React/JSX、Hooks、Context、ref、cloneElement 等接口逐字兼容 | 采用 Rust/Leptos 的公开表达；对齐用户行为与组合能力，不建立 React 运行时 |
| Radix 的 unstable_OneTimePasswordField、unstable_PasswordToggleField，以及内部 unstable 子入口 | 单列候选，另行决定稳定承诺；不进入本 PRD 的必交集合 |
| Radix 内部包逐个翻译成公共 Rust 包，或复制完整 Radix Icons 产品 | 内部组织由开发计划决定；保留现有 Icon/Glyph 和可访问的图标接入 |
| AI 配色生成、账号、云同步、社区主题市场、在线协作、外部网站预览 | 独立产品需求；本地编辑和导出不依赖这些服务 |
| 从任意品牌 HEX 自动生成一套可访问色阶 | 不在提供的 Colors 数据能力内；本文支持命名色阶及显式语义覆盖，不引入未经验证的色阶生成器 |
| 重做 Rustify Docs 的内容、搜索、发布系统，或在本任务直接发布 crates.io/官网 | 文档产品和发行分别沿原流程；仅跟随更新受影响的组件与主题消费 |

### 1.4 方向与取舍

| 维度 | 采用的方向 | 用户价值与取舍 | 落点 |
| --- | --- | --- | --- |
| 体系与可用性 | Colors、Primitives、Themes 分工清楚，Themes 使用相同交互语义 | 可从开箱即用过渡到自有设计语言，避免两套行为漂移；不预定包数量 | WORK-F1、WORK-F5、WORK-F15 |
| 便捷体验 | Theme Studio 增加快捷主题配置，保留现有精细编辑 | 常用决策不用逐个找 token，精调结果又不会被快捷项静默覆盖 | WORK-F3、WORK-F4、WORK-F18 |
| 美观 UI | 用成体系的色阶、尺寸、字级和状态替代逐组件拼视觉 | 参考 Radix 的秩序和层次；Rustify 编辑壳保持稳定中性，不复制 Radix 品牌 | WORK-F15—WORK-F19、WORK-N7 |
| 先进性 | 无样式交互、作用域主题与静态消费并存 | 兼顾复杂应用和静态内容；不以引入 React/在线服务换取参考体系能力 | WORK-F1、WORK-N4 |
| 失败恢复 | 保留作者值、差异预览、显式迁移和可导出草稿 | 主题探索可撤回，失败不毁掉已持久化成果；实例致命故障的未保存输入按 §2.7 列出后果 | WORK-F20、WORK-F21、WORK-N6 |
| AI 赋能 | 本需求不引入运行时 AI | 当前任务可确定性完成；模型服务会增加成本和不确定性，不能替代色彩与可访问性判据 | 非目标中的独立 AI 需求 |

### 1.5 参考基线与适用口径

本次 Rustify 源码基线为 main / 11b6912919a2a34c372aac4d6b7d51052820600f。三份 ref 是用户提供的本地快照；下面版本来自各自 package.json，不表示确认了对应上游提交或公开发行状态。

| 参考 | 本地包版本 | 用途与直接入口 |
| --- | --- | --- |
| Radix Colors | @radix-ui/colors 3.0.0 | [颜色导出](../ref/colors-main/src/index.ts)、[浅色数据](../ref/colors-main/src/light.ts)、[深色数据](../ref/colors-main/src/dark.ts)、[包信息](../ref/colors-main/package.json) |
| Radix Primitives | radix-ui 1.8.1 | [公共能力清单](../ref/primitives-main/packages/react/radix-ui/src/index.ts)、[内部能力边界](../ref/primitives-main/packages/react/radix-ui/src/internal.ts)、[包信息](../ref/primitives-main/packages/react/radix-ui/package.json) |
| Radix Themes | @radix-ui/themes 3.3.0 | [公共能力清单](../ref/themes-main/packages/radix-ui-themes/src/components/index.tsx)、[主题选项](../ref/themes-main/packages/radix-ui-themes/src/components/theme.props.tsx)、[包信息](../ref/themes-main/packages/radix-ui-themes/package.json) |

快照目录指纹为 SHA-256：文件按目录内相对路径排序，逐个拼入「相对路径 UTF-8、NUL、文件内容 SHA-256 二进制摘要」后再求摘要；排除 .git 和 .DS_Store。

| 目录 | 文件数 | 目录指纹 |
| --- | --- | --- |
| colors-main | 20 | bd6ef30b319b5bed241415bea949c0a988b75230e12653cae3cc95ad5d335485 |
| primitives-main | 703 | 73ae9279f0211bd6f34d04da801af4a557ed5c9d9ab2205073c23ea80d675399 |
| themes-main | 668 | 00a510b8e9b3127046f52ca624aeaa9b15af2e384c42d7fe88cfaae2001a2de9 |

ref 被仓库忽略，以上链接用于本地核对，不能作为干净 checkout 的构建依赖。实现所需数据、许可和来源记录必须正式纳入仓库。色阶用途补充参照 [Radix 官方说明](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale)；交互与可访问性分别参照 [WAI-ARIA APG](https://www.w3.org/WAI/ARIA/apg/patterns/) 和 [WCAG 2.2](https://www.w3.org/TR/WCAG22/)，不把参考库的宣传等同于本项目已通过认证。

## 2. 使用场景与能力全景

### 2.1 领域语言与分层

| 名称 | 本文定义 | 边界 |
| --- | --- | --- |
| Colors | 可独立消费的命名颜色与色阶数据 | 不包含组件状态和应用业务 |
| Primitives | 提供可组合结构、交互、焦点及可访问语义的 DOM 基础组件 | 不附带品牌皮肤；必要的布局、隐藏和定位行为仍须可用 |
| Themes | 使用同一交互能力、提供完整视觉和通用视觉参数的组件 | 不再持有另一份业务值或另建主题事实源 |
| 品牌色族 | Radix accentColor 所表达的主要交互色族 | 不是 Rustify 既有 accent token 的改名 |
| 语义覆盖 | 作者对 primary、accent、card 等角色或度量值的显式修改 | 不可被选项变化或模式切换悄悄丢弃 |
| 主题文档 / 已解析主题 | 前者保存作者意图与值，后者是可消费结果 | 延续现有区别；预览归一化不回写作者值 |
| mount scope / 主题边界 | 前者拥有运行时资源和浮层；后者在其中改变主题继承 | 嵌套主题不创建新 wasm 实例，也不夺取宿主的全局主题 |

### 2.2 用户与主旅程

| 角色 | 任务 | 边界 |
| --- | --- | --- |
| 应用开发者 | 选用无样式或主题组件，接入表单、浮层、导航和内容 | 业务状态与授权由应用持有；组件库不做业务权限系统 |
| 设计系统维护者 | 配置色族、尺寸、视觉变体和局部覆盖，验证完整状态 | 可创建自定义主题；低对比度作者值可保留，问题须可见 |
| Theme Studio 使用者 | 从预设开始，精调、预览、撤回、保存并导出 | 使用本地数据；没有账号或云端权限模型 |
| 应用最终使用者 | 用鼠标、触控、键盘、辅助技术完成界面任务 | 不能因选择某个变体失去同一组件的基本操作能力 |

主旅程：开发者选择 Themes 或 Primitives → 在 Theme Studio 选择预设/色族与模式 → 精调并检查真实组件状态 → 导出主题 → 独立应用消费 → 根据迁移说明替换旧入口并完成原有任务。

| 场景 | 给定条件与动作 | 必须达到的状态 |
| --- | --- | --- |
| S1 分层接入 | 开发者分别使用 Colors、Primitives、Themes | 三种消费方式成立，样式与运行依赖符合选择 |
| S2 创建品牌主题 | 从命名色族开始，调字体、圆角、缩放并导出 | 编辑器、预览和消费者使用同一配置 |
| S3 延续旧主题 | 打开旧 JSON、本地草稿或 tweakcn 预设后修改 | 原有作者值保留；升级/覆盖范围可理解且可撤回 |
| S4 受控表单 | 应用接受、拒绝或延迟接受输入请求 | 控件显示以应用值为准，合法草稿规则除外；表单结果一致 |
| S5 嵌套交互 | 对话框内打开 Select、菜单或局部主题浮层 | 焦点、层级、关闭顺序和主题均正确 |
| S6 内容与导航 | 展示长文本、表格、菜单、工具栏和工作区 | 语义正确，键盘可达，保留已有扩展能力 |
| S7 错误恢复 | 导入失败、存储失败、跨标签冲突、GPU 局部故障或实例致命故障 | 已持久化主题可取回；GPU 局部故障时 DOM 编辑和导出仍有可用路径；实例致命故障只按 §2.7 重启恢复最近持久化草稿 |
| S8 静态和多宿主 | SSR/hydrate、根/子路径、多个 scope 共存 | 初始内容可读、接管一致，样式和资源不污染其他区域 |

### 2.3 组件覆盖表

这是用户已确认的完整目标集合。下列同名项表示能力对应，不承诺相同 Rust 类型名、参数或包路径。已有能力须重构/复用并补足契约，不能仅凭同名判定完成。

| 组 | 参考公共能力 | Rustify 现状与目标 | 需求 |
| --- | --- | --- | --- |
| 组合与辅助 | Slot / Slottable、Direction、Portal、AccessibleIcon、VisuallyHidden | 已有 Icon、scope 和浮层；补齐公开组合/方向/辅助入口，保留语义与事件 | WORK-F5 |
| 基础选择 | Checkbox、RadioGroup、Switch、Toggle、ToggleGroup；Themes Radio、CheckboxGroup | 已有二态 Checkbox、RadioGroup、Switch、ToggleGroup；补齐三态、单项与组契约 | WORK-F6 |
| 范围输入 | Slider | 已有单值；补齐多滑块、方向和提交语义 | WORK-F7 |
| 字段和表单 | Form、Label；TextField、TextArea | 已有字段、Form/Field 等；统一原生表单、校验和组合插槽 | WORK-F8 |
| 选择器 | Select | 已有受控选择；补齐部件组合、分组、长列表和空值契约 | WORK-F9 |
| 展开与切换 | Accordion、Collapsible、Tabs；SegmentedControl、TabNav | 已有 Tabs；补齐展开面板与各自正确语义 | WORK-F10 |
| 模态 | Dialog、AlertDialog | 已有 Dialog；补齐部件组合及警告确认语义 | WORK-F11 |
| 浮层信息 | Popover、HoverCard、Tooltip | 已有 Tooltip；补齐交互面板、悬停卡片和锚定能力 | WORK-F12 |
| 菜单和导航 | DropdownMenu、ContextMenu、Menubar、NavigationMenu、Toolbar | 已有 Menu；统一底层行为，区分命令菜单、网站导航与工具栏 | WORK-F13 |
| 反馈 | Toast、Progress；Spinner、Skeleton | 已有 Toast/Toaster、Progress、Spinner；补齐反馈状态与加载骨架 | WORK-F14 |
| 操作视觉 | Button、IconButton、Link | 已有 Button、Link、Icon；统一视觉参数与加载状态，保留原生链接行为 | WORK-F15 |
| 布局 | Box、Flex、Grid、Container、Section、Inset、Reset | 提供成套布局入口；Reset 只作用于显式区域 | WORK-F16 |
| 排版 | Text、Heading、Blockquote、Code、Em、Kbd、Quote、Strong | 提供语义正确的字级与排版；不把所有文字封成无语义容器 | WORK-F16 |
| 内容 | Avatar、Badge、Callout、Card、DataList、Table、AspectRatio、Separator、ScrollArea | 已有 ScrollArea；新增主题内容组件，普通 Table 与 DataTable 区分 | WORK-F17 |
| 卡片选择 | CheckboxCards、RadioCards | 与相应选择 Primitive 使用相同值、禁用和表单语义 | WORK-F6、WORK-F17 |
| 主题入口 | Theme、ThemePanel | 对接现有主题文档/scope；轻量主题面板和 Studio 使用相同配置，不另造完整编辑器 | WORK-F3、WORK-F18 |
| Rustify 扩展 | NumberField、ColorField、DataTable、Tree、FilePicker、DropZone、Splitter、PanelTabs、CommandPalette、Icon/Glyph | 全部保留实际业务能力，迁入统一主题/状态契约；不要求在 Radix 中找到同名对象 | WORK-F8、WORK-F22 |
| 代码辅助导出 | Themes icons、ThemeContext/useThemeContext，以及各组件 Props 类型 | 保障必要的图标、主题消费与类型能力；不按 React 导出符号逐字移植 | WORK-F5、WORK-F23 |

### 2.4 颜色与主题的产品规则

**颜色目录。** Colors 包含 31 个色族：中性 gray、mauve、slate、sage、olive、sand；彩色 tomato、red、ruby、crimson、pink、plum、purple、violet、iris、indigo、blue、cyan、teal、jade、green、grass、brown、bronze、gold、sky、mint、lime、yellow、amber、orange。每个色族有浅/深、实色/alpha、sRGB/P3 数据；另有 blackA、whiteA，均以本地导出为准。

**色阶用途。** 1–2 为背景层次，3–5 为交互背景，6–8 为分隔/边界，9–10 为实色操作及悬停，11–12 为文字层次。这是选色依据，不能推导任意前景/背景组合必然满足 WCAG。[用途来源](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale)

**快捷配置。** 品牌色族采用参考中的 26 个选项：gray、gold、bronze、brown、yellow、amber、orange、tomato、red、ruby、crimson、pink、plum、purple、violet、iris、indigo、blue、cyan、teal、jade、green、grass、lime、mint、sky。中性色族为 auto 加上述六种中性色；auto 的配对结果应与参考规则一致。外观为 inherit/light/dark，面板为 solid/translucent，圆角为 none/small/medium/large/full，缩放为 90%/95%/100%/105%/110%。字体、字距、具体圆角、spacing、layout-gap、阴影和减少动效沿用 Studio 精调能力。[选项](../ref/themes-main/packages/radix-ui-themes/src/components/theme.props.tsx)、[配色](../ref/themes-main/packages/radix-ui-themes/src/props/color.prop.ts)、[配对](../ref/themes-main/packages/radix-ui-themes/src/helpers/get-matching-gray-color.ts)、[圆角](../ref/themes-main/packages/radix-ui-themes/src/props/radius.prop.ts)

**语义和覆盖。** 品牌色族为主操作 primary 等角色提供基础值；Rustify 既有 accent、secondary、destructive、success、warning、chart 和 sidebar 角色保持独立含义。最终值优先级为「组件显式选择/覆盖 → 最近主题边界的显式语义覆盖 → 当前主题的显式语义覆盖 → 当前模式的色族/基础配置 → 默认主题」。组件选色只影响该组件声明负责的颜色，不冲掉局部字体或其他角色；取消覆盖恢复继承。旧主题导入为“自定义”，不得把其 primary 猜成某个色族后重新生成。

**色域边界。** Colors 的独立静态资源可以按需使用 P3 并附 sRGB 回退；共享主题与 Studio 默认仍以 sRGB 已解析结果驱动 DOM/GPU，必须区分作者值和呈现值。本文不增加 GPU P3 能力，也不把 P3 文本伪装成旧版主题解析器可接受的输入。

### 2.5 Theme Studio 的交互与视觉

保留 [现有设计规范](../DESIGN.md) 和 [Studio 工作台方向](../.impeccable/surfaces/examples-theme-studio-src-app-rs.md)：稳定中性编辑壳、左侧编辑/右侧预览、各区独立滚动、窄屏编辑/预览切换。此规范约束编辑工具；用户预览使用作者自己的主题。

| 区域/任务 | 信息与主操作 | 状态、视觉和反馈 |
| --- | --- | --- |
| 顶部 | 当前主题、模式、保存状态、撤销/重做、导入/导出 | 保存成功以实际结果为准；失败后仍能导出 |
| 基础配置 | 预设搜索、品牌/中性色、面板、圆角、缩放 | 少量高层选择先于 token 列表；显示自定义/覆盖，不伪装成未修改预设 |
| 精细编辑 | 沿用 Colors/Typography/Other；展示有效值、来源、覆盖与重置 | 重置有明确作用域；错误贴近字段，未完成输入不破坏有效预览 |
| 组件预览 | 真实 Themes 组件、变体、尺寸和交互状态；可切换 Primitives 的结构演示 | 真实操作可用；演示状态与被禁用的真实状态清楚区分，不放假按钮 |
| Inspector | 从实际部件定位相关色阶、语义角色及覆盖来源 | 找不到映射时明确显示未标注，不编造 token；浮层同样可检查 |
| 窄屏/放大 | 编辑与预览可往返，长 token、错误和导出内容可阅读 | 不依靠整页横向滚动到达关键动作；菜单和对话框在可用视口内可操作 |
| 自定义低对比主题 | 保留作者颜色，在预览侧给诊断与可定位来源 | 编辑壳始终可读；不能以警告为由静默改色或阻止 JSON 取回 |

### 2.6 公共交互契约

- **应用拥有显示值。** value、checked、open、选中项等业务/开放状态由应用决定；Primitives 和 Themes 都遵守现有受控契约，不照搬 Radix 的 defaultValue/defaultChecked 非受控通道。焦点位置、悬停延迟等临时交互可由组件管理。Draft 是已限定的输入例外，不扩大成组件各自存一份业务值。
- **结构与外观可以分开组合。** 所有适用部件公开稳定的状态标识，例如开闭、选中、禁用、方向和定位边侧，允许自定义样式消费；不能要求开发者依赖内部 DOM 层级或自动生成 class 才能辨认状态。
- **事件可组合、默认行为可控制。** 单次离散操作只产生一次对应请求；公开可取消的选择、外部交互、Escape 和自动聚焦遵守取消结果。程序性关闭与用户取消的含义不能混为一谈。
- **禁用、只读、加载、无效分别表达。** 禁用/只读不请求业务值变化；加载不删除名称或制造重复提交；无效字段仍能获得说明和恢复路径。
- **表单结果跟随应用。** 同名控件的值和分组语义明确；提交/重置向应用交付请求，最终显示、校验与表单值同步到应用处理结果。自定义交互元素不得牺牲原生编辑、链接和辅助技术语义。

这些契约由 WORK-F1、WORK-F5—WORK-F14 和 WORK-N1 承接；[当前受控与 Draft 约束](architecture.md#runtime-contracts) 是既有约束，新增的公开组合/状态能力是本 PRD 的要求。

### 2.7 故障与恢复边界

沿用[运行时契约](architecture.md)的实例/scope 生命周期。本节只把既有边界写成可判断的产品结果，不新增 GPU 故障隔离能力；三类失败的作用范围、可继续任务和数据后果不同，不能合并成同一承诺。

| 失败类别 | 触发示例 | 仍可继续的任务 | 数据与恢复结果 |
| --- | --- | --- | --- |
| 正常清理 | scope 释放、浮层关闭、主题边界移除 | 其他有效 scope、浮层和预览不受影响 | 已保存主题和作者值不变；清理对象不留监听、遮罩、焦点限制或定位资源 |
| GPU 局部故障（region/资源级） | 单个 GPU 预览的上下文丢失、GPU 字体资源请求失败 | 同一实例内的 DOM 查看、编辑、保存和导出继续可用 | 作者值留在应用状态；GPU 视图显示真实失败，按既有恢复链路重新应用最新有效主题，不需重启实例 |
| 实例致命故障 | wasm trap 等使该实例无法继续的运行时错误 | 只保证其他独立实例继续可用；本实例全部 scope 和 GPU 预览终止，不能继续编辑或导出 | Studio 显示 fatal 与重启入口；重启恢复最近持久化草稿，未持久化的输入可能丢失且需重新输入；已保存主题和本地库不受影响 |

## 3. 需求与验收

条目描述完整能力；AC ID 在需求内稳定，跨条目引用使用“WORK-F#/AC-#”。期次列仅承接 §7 的交付建议，不缩小正文范围。没有外部服务依赖；组件之间的必要前置在条目和 §7 中说明。所有条目均为设计要求，不能记为已实施。

### 3.1 功能需求

#### WORK-F1 分别选用颜色、无样式组件和主题组件

> 分期：P1 / P2 / P4；终验期 P4。阶段完成只计对应 AC，不记整条需求已完成。

优先级 high；重要度 critical。应用开发者可以按任务选择三类公共入口。Themes 与 Primitives 的相同交互遵守同一行为契约；颜色和无样式消费者无需引入完整主题组件。UAT-1。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 仅消费 Colors 的静态页面可使用命名颜色与模式资源，不需要启动 Rustify wasm、Makepad 或 React。 | P1 |
| AC-2 | 仅消费 Primitives 的应用可完成所选组件交互，不加载 Themes 皮肤、全局 reset 或品牌 token。 | P2 |
| AC-3 | 对全部已覆盖的交互组件，切换 Primitives/Themes 表达后，应用接受或拒绝同一请求时的值、焦点和关闭结果一致，单次离散动作不重复交付。 | P4 |

#### WORK-F2 使用完整且可按需消费的 Colors 色阶

> 分期：P1；终验期 P1。

优先级 high；重要度 high。设计系统维护者可使用 §2.4 的色族和变体；颜色名称、阶序和值对齐本地快照，不以人工近似调色替代数据。UAT-2。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 31 个色族的浅/深、实色/alpha 数据各含完整 1–12 阶；blackA、whiteA 及 sRGB/P3 数据与参考导出逐项一致。 | P1 |
| AC-2 | 消费者能显式选择所需色族、模式和色域资源；未选择 P3 时无 P3 资源依赖，选择 P3 时有可用的 sRGB 回退。 | P1 |

#### WORK-F3 用统一选项配置主题

> 分期：P1；终验期 P1。

优先级 high；重要度 critical。开发者和 Studio 使用者可使用 §2.4 的品牌色、中性色、外观、面板、圆角和缩放；Themes 的轻量 ThemePanel 与 Studio 基础配置表达相同选项，不另存一份主题。UAT-2。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 所有列明的快捷选项可选择并在真实预览与导出消费者中生效；auto 中性色的结果与参考配对一致，界面可辨认有效结果。 | P1 |
| AC-2 | 根主题与嵌套主题能明确选择继承或指定外观；未覆盖的选项随最近主题变化，移除边界恢复继承。 | P1 |
| AC-3 | 圆角 full 不改变 Checkbox、Radio、Switch 等控件的必要形状；缩放同时影响声明受其控制的字级、间距和控件尺寸，不产生二次缩放。 | P1 |

#### WORK-F4 保留语义覆盖和作用域隔离

> 分期：P1；终验期 P1。

优先级 high；重要度 critical。维护者按 §2.4 的优先级覆盖主题；品牌色族与既有 accent 语义明确分开。依赖统一主题选项；不增加跨实例的全局主题所有者。UAT-3、UAT-6。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 切换色族、模式或快捷参数不会悄悄删除作者覆盖；受影响的覆盖可辨认，只有明确重置动作才移除对应覆盖。 | P1 |
| AC-2 | 同页两个 mount scope、一个嵌套主题和宿主元素共存时，修改任一区域不改变其边界外的主题；局部浮层随创建处的有效主题更新。 | P1 |
| AC-3 | 字体、尺寸、阴影与颜色的局部覆盖可分别解除；spacing 单位和 layout-gap 保持不同含义，旧导入不发生重复换算。 | P1 |

#### WORK-F5 组合部件并保留原生语义

> 分期：P2；终验期 P2。

优先级 high；重要度 high。开发者可单独组合 Trigger、Content、Title、Description、Item、Indicator 等适用部件，或把触发能力交给自己提供的单个语义元素；这些名称仅表达部件职责。Slot、Portal、Direction 与辅助组件采用 Rust/Leptos 表达。UAT-4、UAT-6。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 自定义触发元素保留其可访问名称、属性、焦点与原生行为，不产生 button 内嵌 button 或链接内嵌链接；应用取消可取消的动作后不执行该默认动作。 | P2 |
| AC-2 | LTR/RTL 可继承并局部指定，方向相关的按键、对齐和视觉边缘一致；改变布局方向不会改变应用存储值。 | P2 |
| AC-3 | VisuallyHidden 内容可被辅助技术读取而不占普通视觉位置；装饰图标不重复朗读，独立图标操作有名称；Portal 保持所属 scope 的主题、语言与清理边界。 | P2 |
| AC-4 | 适用部件公开稳定的开闭、选中、禁用、方向与定位边侧状态，值随实际状态变化；开发者无需依赖内部节点层级即可定制对应样式，非法单元素组合给出可定位诊断而不静默丢弃内容。 | P2 |

#### WORK-F6 完成二态、三态及分组选择

> 分期：P2；终验期 P2。

优先级 high；重要度 high。最终使用者可操作 Checkbox、Radio/RadioGroup、Switch、Toggle/ToggleGroup、CheckboxGroup 及选择卡片。业务值受控；组与卡片复用同一选择语义。UAT-4。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Checkbox 正确区分 checked、unchecked、indeterminate；RadioGroup 为单选，CheckboxGroup 为多选，ToggleGroup 明确单选/多选与是否允许清空；显示和可访问状态与应用值一致。 | P2 |
| AC-2 | 可操作项支持适用的键盘移动与激活；disabled、read-only 和 invalid 的表现与请求规则明确，禁用/只读不向应用发出值变更请求。 | P2 |
| AC-3 | 普通选择与卡片选择在相同 name、value 和表单条件下给出相同选择结果；禁用项不被误选，卡片可见说明不丢失关联。 | P2 |

#### WORK-F7 操作单值与范围 Slider

> 分期：P2；终验期 P2。

优先级 high；重要度 high。使用者可通过指针、触控和键盘调整一个或多个值，区分连续预览和手势完成；不把全部输入都当作独立历史提交。UAT-4、UAT-10。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 单值、多滑块、水平/垂直、反向及 RTL 配置遵守 min/max/step；多滑块默认允许跨越并按值排序，保持顺序选项开启后受相邻值限制，最小间隔有效，每个滑块都有可辨认名称和值。 | P2 |
| AC-2 | 连续输入与完成提交可区分；应用拒绝请求后显示回到应用值，完成时交付最终请求值，取消手势不冒充完成提交。 | P2 |

#### WORK-F8 填写字段并获得一致的表单反馈

> 分期：P2；终验期 P2。

优先级 high；重要度 high。开发者使用 TextField、TextArea、NumberField、ColorField、Form、Field 和 Label，支持字段插槽、名称、说明、校验与提交状态。保留原生编辑、IME 与合法的 Draft 例外；不引入业务后端。UAT-4。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 字段支持标签、描述、错误关联及适用的 name、value、required、disabled、read-only；表单提交和重置的外部请求可观察，应用处理后显示与应用状态一致。 | P2 |
| AC-2 | 字段的剪贴板、光标、选区、IME 和原生撤销可用；NumberField/ColorField 的未完成草稿只留在聚焦输入内，失焦、确认和取消遵守既有 Draft 契约。 | P2 |
| AC-3 | 客户端校验失败和应用返回错误可分别呈现，错误可定位到字段；提交中状态可辨认，默认避免同一提交的重复触发，失败后能修改并再次提交。 | P2 |

#### WORK-F9 在 Select 中完成可组合选择

> 分期：P2；终验期 P2。

优先级 high；重要度 high。开发者可组合触发器、值、分组、标签、选项与指示器；使用者可选择长列表中的可用项。Select 是选项选择器，不隐含新增可搜索 Combobox。UAT-4、UAT-6。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | placeholder、分组、禁用项、键盘打开/选择、字符定位和关闭后的焦点返回正确；显示值只来自应用接受的值。 | P2 |
| AC-2 | 选项为空或已选值被移除时不显示虚假选中项；长列表可滚动到活动项，浮层在可用视口内可操作，取消不改变选择值。 | P2 |

#### WORK-F10 切换页面内容与展开区域

> 分期：P3；终验期 P3。

优先级 medium；重要度 high。使用者操作 Tabs、Accordion、Collapsible、SegmentedControl 和 TabNav；开发者能按任务选择内容面板、展开区域、状态选择或链接导航，不能混用其角色。UAT-5。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Tabs 支持适用方向、手动/自动激活及面板关联；Accordion 支持单项/多项展开和允许折叠规则；Collapsible 的触发状态与内容一致。 | P3 |
| AC-2 | SegmentedControl 以单选状态工作，TabNav 保留真实链接行为；动态移除或禁用当前项后焦点有确定去处，隐藏面板中的内容不被普通键盘路径误达。 | P3 |

#### WORK-F11 完成普通对话和警告确认

> 分期：P3；终验期 P3。

优先级 high；重要度 critical。开发者可组合 Dialog/AlertDialog 的触发、遮罩、内容、标题、说明和操作；使用者能理解风险、确认或取消。依赖可组合部件及 scope 浮层。UAT-6。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Dialog 支持模态/非模态，AlertDialog 使用警告确认语义；标题与必要说明可被读取，危险确认默认聚焦安全操作，取消不执行危险动作。 | P3 |
| AC-2 | 模态限制本 scope 的底层交互并约束焦点；Escape、外部交互和自动聚焦可按公开规则取消，AlertDialog 不因普通遮罩点击确认或丢失待确认内容。 | P3 |
| AC-3 | 嵌套层只由合适的最上层处理关闭；关闭后返回有效触发器，触发器已移除时返回明确的安全位置，不把焦点留在已删除节点。 | P3 |

#### WORK-F12 使用交互浮层、悬停卡片和提示

> 分期：P3；终验期 P3。

优先级 medium；重要度 high。使用者通过 Popover 操作辅助面板，通过 HoverCard 阅读补充信息，通过 Tooltip 获取简短说明。悬停信息不承载唯一关键操作；Tooltip 不承载可聚焦交互内容。UAT-6。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Popover 可用指针和键盘打开、操作、取消；HoverCard 的触发到内容移动不意外关闭；Tooltip 响应悬停与键盘焦点，Escape 能关闭，关闭不劫持当前焦点。 | P3 |
| AC-2 | 适用浮层支持边侧、对齐、偏移、碰撞与箭头，触发器滚动/尺寸改变后位置仍可用；主题、方向与所属层级一致，触控用户仍有获取关键内容的路径。 | P3 |

#### WORK-F13 操作菜单、网站导航与工具栏

> 分期：P4；终验期 P4。

优先级 medium；重要度 high。最终使用者可操作 DropdownMenu、ContextMenu、Menubar、NavigationMenu、Toolbar；现有 Menu 的调用方有迁移去向。依赖浮层和组合能力。UAT-7。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 命令菜单支持分组、分隔、禁用、勾选、单选和子菜单；指针与键盘的进入、退回、字符定位和关闭顺序正确，子菜单切换不误触底层操作。 | P4 |
| AC-2 | ContextMenu 可用右键及适用键盘方式打开；Menubar 在顶层菜单间移动；NavigationMenu 中链接保留打开新标签页等原生行为，不把网站导航伪装成应用命令菜单。 | P4 |
| AC-3 | Toolbar 使用组内焦点移动，包含按钮、切换组与菜单时不吞掉字段编辑按键；应用取消选择后的开放/关闭结果符合公开契约。 | P4 |

#### WORK-F14 理解加载、进度和短暂反馈

> 分期：P3；终验期 P3。

优先级 medium；重要度 high。使用者通过 Progress、Spinner、Skeleton 和 Toast/Toaster 了解状态；反馈样式不能替代可访问名称、必要说明或可恢复操作。UAT-8。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 确定进度与不确定进度可区分，数值与名称可读取；Spinner 有加载说明，Skeleton 不制造重复朗读，加载结束不留下失效占位。 | P3 |
| AC-2 | Toast 的重要程度、队列、关闭、操作和持续时间可配置；支持滑动关闭及键盘进入通知区，用户悬停或聚焦时暂停关闭，关键操作不只存在于不可追回的短暂提示中，减少动效时仍能理解状态。 | P3 |

#### WORK-F15 使用一致且按组件定义的视觉变体

> 分期：P1 / P2 / P3 / P4；终验期 P4。阶段完成只计对应 AC，不记整条需求已完成。

优先级 high；重要度 high。开发者使用适用于该组件的 size、variant、color、radius、highContrast 和 loading。对齐源中各组件自己的定义，不把一个全局枚举硬套给所有组件；例如 [Button](../ref/themes-main/packages/radix-ui-themes/src/components/_internal/base-button.props.ts) 有 classic/solid/soft/surface/outline/ghost-offset/ghost，而 [TextField](../ref/themes-main/packages/radix-ui-themes/src/components/text-field.props.tsx) 的变体不同。UAT-2、UAT-9。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Button、IconButton、Link 的公开视觉选项和默认值与能力说明一致，Button 加载时保留名称并避免重复操作；Link 保留 href 和原生导航语义。 | P1 |
| AC-2 | 表单选择和输入组件覆盖本地参考的稳定公共视觉参数、变体和尺寸，禁用、只读、错误与焦点样式有可见区别；调用方 class 仍能按既有同类 utility 覆盖规则生效。 | P2 |
| AC-3 | 内容、反馈、切换和浮层组件在其全部声明变体与尺寸下保持层次、操作和状态可辨认；highContrast 的影响仅限其声明角色。 | P3 |
| AC-4 | 菜单、导航和工具栏遵守同一视觉参数契约；不存在“参数接受但视觉无效”或以另一套组件逻辑补做变体的情况。 | P4 |

#### WORK-F16 构建有语义的布局与排版

> 分期：P1；终验期 P1。

优先级 medium；重要度 high。开发者使用 §2.3 的布局和排版组件，组合一致的字级、行高、字重、间距、对齐和响应式表达；HTML 语义不由视觉大小推断。UAT-2、UAT-9。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 所列布局和排版组件均可消费；Heading 层级、引用、代码、强调和按键文字具有正确语义，视觉尺寸与语义元素可分别选择。 | P1 |
| AC-2 | 声明支持响应式的参数在各声明断点生效；长文、长链接和容器边界不遮住内容，Reset 只影响显式区域，不重置宿主或其他 scope。 | P1 |

#### WORK-F17 展示结构化内容与选择卡片

> 分期：P3；终验期 P3。

优先级 medium；重要度 high。开发者使用 Avatar、Badge、Callout、Card、DataList、Table、AspectRatio、Separator、ScrollArea 与选择卡片。普通 Table 提供语义化表格与主题外观，不暗含排序、筛选、虚拟化或 DataTable 改写。UAT-8、UAT-9。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 所列内容组件可显示最小及长内容；Avatar 图片失败有可理解回退，Callout 状态不只依赖颜色，DataList 标签和值关联正确，Table 保留表头和单元格语义。 | P3 |
| AC-2 | 比例容器、分隔与滚动区域遵守声明的方向/尺寸，滚动内容可被键盘到达；选择卡片使用 WORK-F6 的选择契约而非独立维护选择值。 | P3 |

#### WORK-F18 在现有 Studio 中完成基础配置和精调

> 分期：P1；终验期 P1。

优先级 high；重要度 critical。使用者在同一工作台完成 §2.5 工作流。预设、快捷配置、单 token 编辑与 Inspector 修改同一主题；不另做 Radix 风格的平行编辑站。UAT-2、UAT-3。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 现有 44 个预设仍可搜索、使用；色族快捷配置与精调往返时有效值一致，自定义/覆盖标识准确，编辑壳不随预览主题失去可读性。 | P1 |
| AC-2 | 用户编辑公共字体、度量和阴影时沿用既有双模式规则，导入的模式差异在用户显式编辑前保留；只改预览模式不修改另一模式的作者数据。 | P1 |

#### WORK-F19 检查真实组件、状态和主题来源

> 分期：P2 / P4 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

优先级 medium；重要度 high。使用者在 Studio 检查本文覆盖组件、尺寸、变体和关键状态，使用 Inspector 定位消费来源。保留已有场景及 DOM/GPU/对比视图，但新组件以 DOM 展示；GPU 无对应能力时明确说明。UAT-9、UAT-12。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 已交付的表单、布局和基础主题组件有可操作样本，可查看其声明状态；预览宽度和模式选择不会重置正在检查的主题。 | P2 |
| AC-2 | 完整参考集合中的交互、内容和导航均有真实 DOM 样本与结构说明；可从实际部件定位其色族、语义角色或覆盖来源，浮层继承来源也可辨认。 | P4 |
| AC-3 | 现有 Rustify 扩展及已保留的场景在新主题下可检查；GPU 视图只报告实际支持。GPU 局部故障时 DOM 查看、编辑、保存与导出仍可用；实例致命故障按 §2.7 终止本实例并进入重启恢复，不宣称未保存输入仍可取回。 | P5 |

#### WORK-F20 无损交换主题并在独立应用中消费

> 分期：P1；终验期 P1。

优先级 high；重要度 critical。使用者可导入现有 v1 JSON/CSS，并导出新主题的完整 JSON、标准/作用域 CSS 与 Rust 接入说明。格式须版本化；具体新增字段由开发计划定义，不能把隐藏元数据塞入 v1 未知 token 假装旧版支持。UAT-3、UAT-11。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 旧 v1 JSON 升级后保留两模式作者值、名称和可保留的未知字符串 token，不按色族重生成；完整新格式往返保留快捷选项、作者值和显式覆盖，未知版本被明确拒绝。 | P1 |
| AC-2 | 导入先呈现候选差异、版本和诊断，Apply 后才替换；取消、语法错误、超过既有 256 KiB 限制或不可支持的颜色输入均保留当前主题与历史。 | P1 |
| AC-3 | 独立 DOM 应用能按导出的 CSS/JSON/Rust 说明消费相同模式与有效主题；导出标明所需 SDK/CLI 版本及外部字体资源，不依赖 Studio 页面状态。 | P1 |
| AC-4 | 旧版兼容导出若不能表达完整新主题，界面在导出前列明损失且需用户显式选择；CSS 仅代表声明的已解析结果，不宣称能恢复全部作者意图，复制/下载反馈反映真实结果。 | P1 |

#### WORK-F21 保留历史、本地主题与失败恢复

> 分期：P1；终验期 P1。

优先级 high；重要度 critical。使用者继续使用撤销/重做、重置、本地保存和跨标签处理。主题格式变化不得清空既有库；迁移失败保留可取回的原始数据，不自动写坏缓存。UAT-3、UAT-10。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 快捷配置、语义覆盖、重置和一次连续编辑均可作为完整操作撤销/重做；取消操作不入历史，草稿内原生撤销不被主题快捷键抢走。 | P1 |
| AC-2 | 原有本地主题与偏好可迁移后重新打开；迁移前原始数据可取回，坏缓存不被自动覆盖，写入失败保留内存草稿且仍可导出。 | P1 |
| AC-3 | 重名、覆盖、删除和跨标签 Load/Keep/Save copy 延续显式结果；发生冲突时不静默覆盖或合并，失败不改变已保存事实。 | P1 |

#### WORK-F22 延续 Rustify 的工作区与大数据能力

> 分期：P5；终验期 P5。

优先级 medium；重要度 high。原应用开发者继续使用 DataTable、Tree、FilePicker、DropZone、Splitter、PanelTabs、CommandPalette 和 Icon/Glyph；只调整必要的主题/公共接口适配，不为目录整齐重写其业务和性能机制。UAT-7、UAT-13。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 所列公共能力均有保留入口或明确的新入口，原有选择、导航、拖放、文件、工作区操作和可访问语义可完成；空、错误与禁用状态可辨认。 | P5 |
| AC-2 | 扩展组件使用统一主题角色且局部覆盖有效；DataTable/Tree 等原有大数据能力不因套用普通 Table 或通用内容组件而丢失，规模与性能比较采用原有同类负载。 | P5 |

#### WORK-F23 查阅完整能力与迁移既有应用

> 分期：P1 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

优先级 high；重要度 critical。开发者能从目录判断已支持范围，并依据旧→新映射迁移全部公共组件、主题接入和样式入口；不长期维护两套组件实现。UAT-1、UAT-13、UAT-14。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 目录分别标识已可用、部分支持及未交付能力，并注明参考差异、呈现路径和可用入口；计划覆盖不被显示为已支持。 | P1 |
| AC-2 | 每个旧公共入口及旧参数/事件、Button 变体、颜色语义、主题文件、CSS 用法均有保留/替换/移除理由；移除前有可用替代和示例，不保留无边界期限的平行实现。 | P5 |
| AC-3 | 仓内示例、DOM-only 消费者、受影响的 Rustify Docs 演示和独立主题消费者均能按新说明完成原有任务；公开类型、组件状态表和示例与交付物一致。 | P5 |

### 3.2 非功能需求

#### WORK-N1 关键操作可访问

> 分期：P2 / P4 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

属性 usability；优先级 high；重要度 critical。作用域为所有交付组件、默认主题和 Studio 编辑壳。采用适用的 WCAG 2.2 AA 条款，并以 APG 校准交互；不将某个库整体称为已认证。UAT-4、UAT-6、UAT-14。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 表单与基础控件均有可访问名称、角色、值和状态，键盘可完成主要任务，焦点可见；默认壳普通文本对比度至少 4.5:1，必要非文本边界至少 3:1，适用例外按标准注明。 | P2 |
| AC-2 | 浮层、导航和反馈满足适用的焦点顺序、模态、关闭和状态宣告要求；用户选择低对比主题时显示带上下文的诊断，不能仅凭色阶名称断言合格。 | P4 |
| AC-3 | 放大、减少动效及辅助技术下关键任务保持可完成；人工验证缺失项明确列为未验证，不用自动扫描替代其结论。 | P5 |

#### WORK-N2 热路径与产物代价可解释

> 分期：P1 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

属性 performance；优先级 high；重要度 high。沿用 [Theme Studio §7 既有性能约束](plan/THEME-STUDIO.md) 的机制与测量口径，不编造新的毫秒 SLO。UAT-10、UAT-12。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 连续主题编辑不重建 wasm 实例或现有 GPU region，每帧最多发布一个最新主题快照；输入结束后最终有效主题可达，空闲时无持续主题更新或额外绘制。 | P1 |
| AC-2 | 受影响热路径交付同环境、同功能负载的前后 p50/p95、产物 WASM/CSS 压缩尺寸和新增资源差值；旧功能与新能力增量分列，无未解释的持续积压或随交互次数增长的资源残留。 | P5 |

#### WORK-N3 隔离、输入安全与清理

> 分期：P1 / P4；终验期 P4。阶段完成只计对应 AC，不记整条需求已完成。

属性 security；优先级 high；重要度 critical。适用 SDK 样式、主题导入、浮层和页面监听，沿用现有 scope、CSP 与实例生命周期契约。UAT-6、UAT-10、UAT-14。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 在既有严格 CSP 下加载和编辑无新增违规；无全局 preflight、裸元素污染或对宿主 root/body 的主题改写，动态位置和主题不要求放宽内联策略。 | P1 |
| AC-2 | 导入的 CSS 作为数据处理，不执行样式、脚本、URL 或外部导入；无效输入、字体值和未知版本不会绕过校验或改变当前主题。 | P1 |
| AC-3 | scope 释放或浮层关闭后，其监听、定位、焦点限制和相关资源不再影响页面，其他有效 scope 和浮层继续可用；所属实例致命故障时按 §2.7 终止本实例全部 scope，不遗留影响页面的资源，只保证其他独立实例继续可用。 | P4 |

#### WORK-N4 保持现有消费与部署模式

> 分期：P2 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

属性 compatibility；优先级 high；重要度 critical。以 [架构契约](architecture.md) 和 [当前工具链](../mise.toml) 为约束；运行通过环境仍以仓内真实证据为准，不继承参考库的浏览器支持承诺。UAT-1、UAT-11、UAT-14。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 纯 DOM 的 CSR、SSR 和 hydrate 可分别消费已交付组件，默认图和产物无 Makepad；静态内容不依赖浏览器 API 才能生成，确定性初始状态接管无重复节点/ID。 | P2 |
| AC-2 | 根路径和子路径静态消费、文档演示、主题 JSON/CSS 消费及既有 GPU 共享主题接入继续成立；完整新主题所需版本在接入文档中明确，旧消费者不被错误宣称兼容。 | P5 |
| AC-3 | 字体、图像等资产通过外部静态资源引用，不嵌入 WASM；默认任务不要求新增后台服务、Node 运行时或改变仓内工具链 major。 | P5 |

#### WORK-N5 能力声明与实际结果一致

> 分期：P1 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

属性 maintainability；优先级 high；重要度 high。适用于公共清单、组件文档、Studio 和迁移说明。UAT-1、UAT-9、UAT-13。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 同一能力在目录、示例和文档中具有一致状态；计划项、参考已有、Rustify 已实现和实际已验收明确区分。 | P1 |
| AC-2 | 全部稳定公共参考能力及旧公共导出有逐项去向；组件声明的尺寸、变体、交互与渲染模式均能对应真实可消费结果，差异和限制可查。 | P5 |

#### WORK-N6 失败不破坏有效状态

> 分期：P1 / P4；终验期 P4。阶段完成只计对应 AC，不记整条需求已完成。

属性 reliability；优先级 high；重要度 critical。覆盖主题编辑、交换、本地持久化及浮层失效。UAT-10、UAT-12。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 解析、导入、保存、复制或迁移失败时，最后有效主题和可取回作者数据仍在；界面给出实际失败与可继续路径，不显示成功。 | P1 |
| AC-2 | 触发器移除、内容动态变化、浮层取消或 GPU 局部故障时，剩余 DOM 任务可继续；不会遗留永久遮罩、不可解除的焦点限制或阻止导出的错误状态。实例致命故障按 §2.7 终止本实例并进入重启恢复，不按局部失败处理。 | P4 |

#### WORK-N7 视觉层次与响应式支持任务

> 分期：P1 / P4 / P5；终验期 P5。阶段完成只计对应 AC，不记整条需求已完成。

属性 usability；优先级 medium；重要度 high。范围为默认 Themes 组件与 Studio，不强制用户自定义主题长成默认样式。UAT-2、UAT-9、UAT-14。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | Studio 的主题名、基础配置、精调和预览层级清楚，编辑壳沿用中性工作台；颜色错误、覆盖来源和保存状态不靠悬停才能发现。 | P1 |
| AC-2 | 默认浅/深主题的字级、间距、边界、面板深度和焦点规则一致；每个声明变体在其交互状态下可辨认，加载、空内容和错误布局不跳失关键操作。 | P4 |
| AC-3 | 390px、768px 和桌面预览，以及 200% 放大时，主题编辑、导航、浮层与导出主路径可达；需二维呈现的数据区域可局部滚动。新增配置和诊断沿用中英文编辑壳，语言切换不改写主题名、token 和作者数据。 | P5 |

#### WORK-N8 来源与可重现输入完整

> 分期：P1；终验期 P1。

属性 maintainability；优先级 high；重要度 high。适用于采纳的 Radix 代码、颜色数据与分发资产；不把本地 ref 当作生产依赖。UAT-1、UAT-11。

| AC ID | 验收原文 | 期次 |
| --- | --- | --- |
| AC-1 | 正式采用的代码/数据具有可取回的来源、快照版本或指纹、修改说明及许可证；颜色输入与 §1.5 的实际采用基线一致，变更可归因。 | P1 |
| AC-2 | 干净 checkout 的正常构建和外部消费者无需 ref 目录或在线抓取上游源；资产与许可随相应交付物提供。 | P1 |

## 4. 约束与前提

| 来源 | 约束与产品影响 | 影响条目 |
| --- | --- | --- |
| 本次用户原始诉求及范围回复 | 以本地 Colors/Primitives/Themes 的稳定公共能力为终局，保留既有扩展；主题结合现有 Studio | 全部 WORK-F |
| 本次用户兼容回复 | 允许破坏性 API 调整，提供迁移说明；迁移不能只列重命名，必须包含行为、样式和主题数据 | WORK-F20—WORK-F23 |
| [AGENTS.md](../AGENTS.md)、[工作区清单](../Cargo.toml) | DOM 优先；共享状态不要求双端组件对等；不自动扩展 GPU 范围 | WORK-F1、WORK-F19、WORK-N4 |
| [运行时契约](architecture.md) | instance/scope 生命周期（含 §2.7 的三类故障边界）、受控值、Draft、页面监听和释放、局部浮层/主题所有权保持成立 | WORK-F4—WORK-F14、WORK-F19、WORK-N3、WORK-N6 |
| [主题指南](themes.md) 和 [原 Studio 计划](plan/THEME-STUDIO.md) | 作者值与呈现值分离、共享 sRGB 投影、语义 token、spacing/layout-gap、资源外置 | WORK-F2—WORK-F4、WORK-F18—WORK-F21 |
| [当前样式约定](../crates/rustify-components/css/sdk.css) | 保留无前缀 Tailwind、调用方 class 合并和单份应用样式集成；Primitives 不自动导入 Themes/CSS reset | WORK-F1、WORK-F15、WORK-N3 |
| [SSR/hydrate 契约](architecture.md#static-scopes-and-hydration) | 模式分别构建，初始视图须确定；浮层沿用静态关闭触发器、浏览器接管后创建的边界，不承诺任意已打开 Portal 的宿主渲染 | WORK-N4、WORK-F23 |
| [PRODUCT.md](../PRODUCT.md)、[DESIGN.md](../DESIGN.md) | Studio 采用已有编辑工作台；作者主题不受编辑壳默认颜色/字体强制约束 | WORK-F18、WORK-N7 |
| [mise.toml](../mise.toml)、[仓库开发入口](../README.md) | 复用固定工具与构建/检查入口，不擅自安装服务或升级 major；本 PRD 不决定新增包数量 | WORK-N4、WORK-N8 |

前置依赖均在仓内：可分层消费的公开能力、统一主题交换、对应组件实际交付。未满足时，目录和 Studio 必须标明未交付/不可用；不能用静态摆件或旧组件截屏冒充新组件可交互。第三方参考只提供设计与数据来源，不提供运行时服务。新主题消费所需的最低版本必须与交付物一并确定，否则新格式导出不满足 WORK-F20/AC-3。

## 5. 产品决策与待落实项

### 5.1 决策记录

| ID | 决策与状态 | 结论及理由 | 未采用方向 | 影响 |
| --- | --- | --- | --- | --- |
| D1 | 用户已确认：兼容策略 | 允许破坏性调整，以完整迁移说明收口，避免长期双体系维护 | 永久维持旧 API、无限期兼容层 | WORK-F23 |
| D2 | 用户已确认：覆盖范围 | 稳定公共能力完整设计并分期交付，保留 Rustify 扩展，unstable 单列候选 | 只改旧目录中的组件、把其余稳定组件降成候选 | WORK-F1—WORK-F23 |
| D3 | 既有约束：呈现职责 | 重构 DOM 体系；主题保留共享投影和现有 GPU 预览 | 自动补齐全量 GPU 组件 | WORK-F19、WORK-N4 |
| D4 | 本稿设计：分层消费 | Colors 可独立使用，Primitives 不附品牌皮肤，Themes 统一行为 | 只换一套皮肤，或为无样式/主题组件分别实现交互 | WORK-F1、WORK-F5、WORK-F15 |
| D5 | 本稿设计：主题关系 | Studio 是完整编辑入口，ThemePanel 只提供同源快捷配置；快捷项和精调使用一个主题文档 | 再建 Radix 编辑器，与现有 Studio 双向同步两份数据 | WORK-F3、WORK-F18 |
| D6 | 本稿设计：语义与旧数据 | 品牌色族不覆盖既有 accent 含义；旧主题作为自定义值导入，保留作者字符串和明确覆盖 | 猜测最近色族并重新生成旧主题 | WORK-F4、WORK-F20 |
| D7 | 既有约束的明确延续：状态 | 保持受控契约，提供可组合部件；不为对齐 React API 新增非受控值通道 | 同时承诺 controlled/uncontrolled 两种业务状态来源 | WORK-F5—WORK-F14 |
| D8 | 本稿设计：色域 | Colors 提供独立 P3 数据与回退；共享主题/Studio 默认维持既有 sRGB 结果 | 宣称 DOM/GPU 已等价支持广色域，或丢弃参考中的 P3 数据 | WORK-F2、WORK-F20 |
| D9 | 本稿设计：可访问性 | 默认主题和编辑壳满足适用门槛，自定义低对比主题保留并提示 | 将所有 Radix 配色组合直接标为合格，或强制改写作者颜色 | WORK-N1、WORK-N7 |

D1、D2 来自本次明确回复；D3、D7 的约束来自仓库。其余是本 PRD 提交评审的产品设计，不冒称用户已逐项批准。没有尚待用户回答、会阻塞本稿成文的问题；完整评审意见已落实到 §2.7 等章节，本稿可按 §7.4 交接，但评审不代表实现或运行验收完成。

### 5.2 待落实项

以下是交付时需要落实的实现/发行信息，不改变本稿范围或验收含义，不作为延迟成文的理由。

| ID | 项目 | 当前口径与理由 | 责任角色 | 解决节点 |
| --- | --- | --- | --- | --- |
| Q1 | 新公共入口的 Rust 命名与包边界 | 采用本文三层消费契约；不在 PRD 预设三个新 crate 或公开每个内部模块 | SDK 维护者 | 对应开发计划定稿前 |
| Q2 | 新主题格式版本号、字段与迁移载体 | 完整新格式须显式版本化；读取旧数据可无损升级，旧版输出损失需说明 | 主题模块维护者 | 主题交换开发计划定稿前 |
| Q3 | 新 SDK/CLI 配对和破坏性发行版本 | 使用仓内现有发布规则；必须把新导出格式对应的最低消费者版本写入接入说明 | 发行维护者 | 对外提供新格式/迁移说明前 |
| Q4 | 可复测的性能比较数据 | 沿用原有 workload 与口径，报告热路径和体积前后数据，不预设未测毫秒指标 | 实施者 | 各相关热路径变更交付时 |

完整稳定参考集合是固定基线。下游若拟删除某个公共状态、变体、组合或现有扩展，应作为产品范围变更回到本 PRD 评审，不能以少声明参数或计划排不下为由默默跳过。

## 6. 追踪与 UAT 状态

### 6.1 原始诉求去向

| 原始诉求 | 下场 | 落点 |
| --- | --- | --- |
| R1 设计参考 Radix 体系的 DOM 组件库重构 PRD | 新建设计工作稿；现有能力作为复用/重构基线 | WORK-F1、WORK-F5—WORK-F17、WORK-F22、WORK-F23 |
| R2 使用 ref/colors-main 颜色体系 | 新建完整颜色目录要求，接入统一主题 | WORK-F2—WORK-F4、WORK-N8 |
| R3 使用 ref/primitives-main 交互基础 | 新建无样式组合与行为要求；同名既有组件补齐差异 | WORK-F5—WORK-F14 |
| R4 使用 ref/themes-main 带主题组件 | 新建完整主题组件与视觉参数要求 | WORK-F3、WORK-F15—WORK-F17 |
| R5 主题结合 examples/theme-studio | 更新产品能力要求，沿用已有本地编辑与恢复能力 | WORK-F18—WORK-F21、WORK-N2、WORK-N6 |
| R6 用户确认允许破坏性调整并提供迁移说明 | 已采纳；迁移覆盖 API、行为、样式和数据 | WORK-F20、WORK-F22、WORK-F23 |
| R7 用户确认完整稳定体系、分期及保留扩展 | 已采纳；unstable 候选显式排除，其余逐项覆盖 | §2.3、WORK-N5、§7 |

未执行 SPMS 查重或写入；“新建/更新”仅描述本地工作稿的需求性质，不表示已经存在或创建平台 FR/NFR 实体。

### 6.2 目标 → 场景 → 需求

| 目标 | 场景 | 需求与验收落点 | UAT |
| --- | --- | --- | --- |
| G1 可分层采用 | S1、S8 | WORK-F1/AC-1—AC-3、WORK-F5/AC-1—AC-4、WORK-N4/AC-1、WORK-N8/AC-1—AC-2 | UAT-1、UAT-14 |
| G2 一个主题闭环 | S2、S3、S7 | WORK-F2—WORK-F4、WORK-F18—WORK-F21 各有效 AC；WORK-N2/AC-1、WORK-N6/AC-1 | UAT-2、UAT-3、UAT-9—UAT-12 |
| G3 可组合且可访问 | S4、S5、S6 | WORK-F6—WORK-F17 各有效 AC；WORK-N1、WORK-N3、WORK-N7 各有效 AC | UAT-4—UAT-9、UAT-14 |
| G4 可迁移且不丢数据 | S3、S6、S7 | WORK-F20/AC-1—AC-4、WORK-F21/AC-1—AC-3、WORK-F22、WORK-F23 各有效 AC；WORK-N5/AC-2 | UAT-3、UAT-10、UAT-11、UAT-13 |
| G5 保持部署和运行优势 | S1、S7、S8 | WORK-N2、WORK-N3、WORK-N4、WORK-N6、WORK-N8 各有效 AC；WORK-F19/AC-3 | UAT-1、UAT-10—UAT-12、UAT-14 |

### 6.3 UAT 预期状态

这里定义角色、动作和结果，不规定测试脚本或工具。开发验收可由 Agent 按角色约束执行；需要真实浏览器/辅助技术或人工确认的项目须保留实际证据和缺失项。

| UAT | 角色与场景 | 动作后的预期状态与通过条件 | 需求/验收 |
| --- | --- | --- | --- |
| UAT-1 | 开发者；S1，具有公开接入说明 | 三类入口分别可消费；无样式入口没有主题皮肤；目录能准确区分已交付与计划能力 | WORK-F1/AC-1—AC-3、WORK-F23/AC-1、WORK-N5/AC-1、WORK-N8/AC-1—AC-2 |
| UAT-2 | 设计系统维护者；S2，从色族创建主题 | 色阶目录完整、快捷项与精调一致；实际按钮/排版可见；导出消费者使用同样有效值 | WORK-F2/AC-1—AC-2、WORK-F3/AC-1—AC-3、WORK-F15/AC-1、WORK-F16/AC-1—AC-2、WORK-F18/AC-1—AC-2、WORK-N7/AC-1 |
| UAT-3 | Studio 使用者；S3，已有自定义旧主题/保存库 | 打开与升级保留作者值；快捷切换不丢覆盖；取消升级/导入不改主题，原数据可取回 | WORK-F4/AC-1—AC-3、WORK-F18/AC-2、WORK-F20/AC-1—AC-2、WORK-F21/AC-2 |
| UAT-4 | 最终使用者与组件使用方；S4，受控表单及应用接受/拒绝/延迟处理 | 值、选择和开放状态跟随应用；三态/范围/组选择正确；IME、字段错误、提交和取消有一致结果，公开部件状态可被自定义样式消费 | WORK-F5/AC-1—AC-4、WORK-F6/AC-1—AC-3、WORK-F7/AC-1—AC-2、WORK-F8/AC-1—AC-3、WORK-F9/AC-1—AC-2、WORK-N1/AC-1 |
| UAT-5 | 最终使用者；S6，切换/展开和链接导航共存 | 标签面板、单/多展开、状态选择与链接导航各按自身语义工作；移除当前项后仍可继续 | WORK-F10/AC-1—AC-2 |
| UAT-6 | 最终使用者；S5，对话框内打开局部主题浮层 | 正确继承主题与方向；只关闭应关闭层；取消不执行危险操作，关闭后焦点可继续，边界外页面不被错误改写 | WORK-F4/AC-2、WORK-F5/AC-3、WORK-F9/AC-2、WORK-F11/AC-1—AC-3、WORK-F12/AC-1—AC-2、WORK-N3/AC-1、AC-3 |
| UAT-7 | 最终使用者；S6，命令菜单、网站导航和工作区并存 | 子菜单/上下文/顶层菜单与工具栏可键盘操作；链接保留原生导航，工作区不丢原有任务能力 | WORK-F13/AC-1—AC-3、WORK-F22/AC-1 |
| UAT-8 | 最终使用者；S6，加载/进度/图片失败/通知出现 | 反馈状态可读取且不重复；图片失败可理解；内容结构正确，关键操作不随短暂提示永久消失 | WORK-F14/AC-1—AC-2、WORK-F17/AC-1—AC-2 |
| UAT-9 | 维护者；S2/S6，检查真实预览和所有视觉状态 | 本地参考的稳定组件、变体和尺寸有可核查结果；Inspector 来源准确，长内容与两模式下层次可辨认 | WORK-F15/AC-1—AC-4、WORK-F16/AC-1—AC-2、WORK-F17/AC-1—AC-2、WORK-F19/AC-1—AC-3、WORK-N5/AC-1—AC-2、WORK-N7/AC-2 |
| UAT-10 | Studio 使用者；S7，连续编辑、存储失败或跨标签冲突 | 一次手势对应一次历史操作；最终主题可达；失败保留草稿，冲突未解决时不显示保存成功，仍可导出 | WORK-F7/AC-2、WORK-F21/AC-1—AC-3、WORK-N2/AC-1—AC-2、WORK-N3/AC-2、WORK-N6/AC-1 |
| UAT-11 | 外部开发者；S2/S8，消费导出与兼容格式 | 新完整格式可往返，独立消费者使用匹配版本；损失清单先于兼容导出出现，字体路径/许可完整 | WORK-F20/AC-1—AC-4、WORK-N4/AC-2—AC-3、WORK-N8/AC-1—AC-2 |
| UAT-12 | Studio 使用者；S7，同实例两个 scope 下 GPU 预览上下文丢失，或发生 wasm trap | 两类失败给出不同结果：GPU 局部故障时 GPU 视图显示真实失败，DOM 预览、编辑、保存/导出继续可用；实例致命故障时本实例全部 scope 终止，Studio 显示 fatal 与重启入口，重启恢复最近持久化草稿并说明未保存输入可能丢失。现有共享语义不被新 DOM 组件范围扩大 | WORK-F19/AC-3、WORK-N2/AC-1、WORK-N3/AC-3、WORK-N6/AC-2 |
| UAT-13 | 原应用开发者；S3/S6，按公开指南迁移 | 旧公共入口全部有去向，原有工作区和大数据任务可完成，主题和样式不依赖未公开兼容层 | WORK-F22/AC-1—AC-2、WORK-F23/AC-2—AC-3、WORK-N5/AC-2 |
| UAT-14 | 读者与键盘/辅助技术使用者；S8，静态、接管、窄屏与放大 | 静态内容可读，接管不重复；关键路径可达、焦点与状态正确；根/子路径和多 scope 隔离成立 | WORK-N1/AC-1—AC-3、WORK-N3/AC-1—AC-3、WORK-N4/AC-1—AC-3、WORK-N7/AC-3 |

## 7. 分期交付建议

分期改变交付顺序，不删除完整目标。每期开发计划建议 5–8 个 Milestone；实际拆解超过 8 个时优先进一步拆期，连续性确有必要可到 9–10 个并说明原因，最多 10 个。这里不预定代码模块或实现任务。

### 7.1 每期用户价值与出口

| 期次 | 使用者新增的可完成任务 | 代表性出口判据 | 前置 |
| --- | --- | --- | --- |
| P1 统一颜色和主题创作 | 设计人员可在原 Studio 中选色族、精调并导出，开发者能构建有统一按钮/排版的静态内容页 | WORK-F2/AC-1 的完整色阶、WORK-F18/AC-1 的同源编辑、WORK-F20/AC-3 的独立消费均成立 | 本稿评审；旧数据迁移与新格式说明可供消费 |
| P2 可组合的表单 | 开发者能用无样式或主题入口构建三态、范围、多种字段和 Select 表单 | WORK-F1/AC-2 的无皮肤入口、WORK-F6/AC-1 的选择状态、WORK-F8/AC-1 的表单结果成立 | P1 的主题/样式/交换契约 |
| P3 可操作的应用面板 | 使用者能完成对话确认、辅助浮层、内容切换、反馈及结构化内容任务 | WORK-F11/AC-1—AC-3 的模态结果、WORK-F12/AC-1 的浮层任务、WORK-F17/AC-1 的内容语义成立 | P2 的组合、焦点和表单基础 |
| P4 完整导航与参考交互 | 使用者能在菜单、子菜单、网站导航和工具栏间完成任务，维护者可检查完整参考组件 | WORK-F13/AC-1—AC-3 的导航操作、WORK-F19/AC-2 的完整参考样本成立 | P3 的层级、取消和浮层契约 |
| P5 原应用迁移与整体交付 | 原应用和外部消费者能迁移全部旧公共入口，在文档、工作区及独立应用继续原任务 | WORK-F22/AC-1—AC-2、WORK-F23/AC-2—AC-3 及全部质量门成立 | P1–P4；真实消费、可访问性与性能证据齐备 |

各期都执行适用于当期交付物的质量要求；下表中跨期 NFR 的终验指完整范围覆盖结束，不能解释为前面期次可忽略质量要求。

### 7.2 需求与 AC 分配

所有行来源均为“本 PRD 新建的工作条目”，不引用虚构平台编号。每个有效 AC 只分配到一个终验期；跨期条目的累计验收必须全部通过。

| 需求 | P1 | P2 | P3 | P4 | P5 | 终验期 |
| --- | --- | --- | --- | --- | --- | --- |
| WORK-F1 | AC-1 | AC-2 | — | AC-3 | — | P4 |
| WORK-F2 | AC-1—AC-2 | — | — | — | — | P1 |
| WORK-F3 | AC-1—AC-3 | — | — | — | — | P1 |
| WORK-F4 | AC-1—AC-3 | — | — | — | — | P1 |
| WORK-F5 | — | AC-1—AC-4 | — | — | — | P2 |
| WORK-F6 | — | AC-1—AC-3 | — | — | — | P2 |
| WORK-F7 | — | AC-1—AC-2 | — | — | — | P2 |
| WORK-F8 | — | AC-1—AC-3 | — | — | — | P2 |
| WORK-F9 | — | AC-1—AC-2 | — | — | — | P2 |
| WORK-F10 | — | — | AC-1—AC-2 | — | — | P3 |
| WORK-F11 | — | — | AC-1—AC-3 | — | — | P3 |
| WORK-F12 | — | — | AC-1—AC-2 | — | — | P3 |
| WORK-F13 | — | — | — | AC-1—AC-3 | — | P4 |
| WORK-F14 | — | — | AC-1—AC-2 | — | — | P3 |
| WORK-F15 | AC-1 | AC-2 | AC-3 | AC-4 | — | P4 |
| WORK-F16 | AC-1—AC-2 | — | — | — | — | P1 |
| WORK-F17 | — | — | AC-1—AC-2 | — | — | P3 |
| WORK-F18 | AC-1—AC-2 | — | — | — | — | P1 |
| WORK-F19 | — | AC-1 | — | AC-2 | AC-3 | P5 |
| WORK-F20 | AC-1—AC-4 | — | — | — | — | P1 |
| WORK-F21 | AC-1—AC-3 | — | — | — | — | P1 |
| WORK-F22 | — | — | — | — | AC-1—AC-2 | P5 |
| WORK-F23 | AC-1 | — | — | — | AC-2—AC-3 | P5 |
| WORK-N1 | — | AC-1 | — | AC-2 | AC-3 | P5 |
| WORK-N2 | AC-1 | — | — | — | AC-2 | P5 |
| WORK-N3 | AC-1—AC-2 | — | — | AC-3 | — | P4 |
| WORK-N4 | — | AC-1 | — | — | AC-2—AC-3 | P5 |
| WORK-N5 | AC-1 | — | — | — | AC-2 | P5 |
| WORK-N6 | AC-1 | — | — | AC-2 | — | P4 |
| WORK-N7 | AC-1 | — | — | AC-2 | AC-3 | P5 |
| WORK-N8 | AC-1—AC-2 | — | — | — | — | P1 |

### 7.3 跨期终验登记

| 跨期需求 | 涉及期次 | 整条需求终验期 |
| --- | --- | --- |
| WORK-F1 | P1 / P2 / P4 | P4 |
| WORK-F15 | P1 / P2 / P3 / P4 | P4 |
| WORK-F19 | P2 / P4 / P5 | P5 |
| WORK-F23 | P1 / P5 | P5 |
| WORK-N1 | P2 / P4 / P5 | P5 |
| WORK-N2 | P1 / P5 | P5 |
| WORK-N3 | P1 / P4 | P4 |
| WORK-N4 | P2 / P5 | P5 |
| WORK-N5 | P1 / P5 | P5 |
| WORK-N6 | P1 / P4 | P4 |
| WORK-N7 | P1 / P4 / P5 | P5 |

以上条目在首个交付阶段完成后均只记对应 AC 已完成，不记整条需求已交付/已上线。特别是 WORK-F1、WORK-F15 终验在 P4，WORK-F19、WORK-F23 终验在 P5。若未来录入 SPMS，跨期声明须同时进入条目正文和逐行验收期次，版本挂终验期，不能因首期关联 Issue 完成而提前转终态。

### 7.4 下游交接

开发计划以工作号和 AC 集合承接，计划落在 docs/plan/；不得将重命名、分 crate 或替换实现本身当作产品验收。按每期实际依赖确定实现顺序，若调整期次须同步本节、条目 AC 和跨期终验关系。测试计划承接 §6.3 的预期状态、有效 AC 及前置条件，另行决定测试方法与证据安排。

当前没有新增开发计划或测试用例。完整 PRD 评审已完成、意见已落实；下一步以本文件和 P1 需求集合编制开发计划。

## 8. 本稿交付状态

- 产物：本 PRD，包含 23 条功能需求、8 条非功能需求、完整能力映射、14 个 UAT 场景及五期建议。
- 用户已确认：完整稳定体系/保留扩展/unstable 候选的范围，以及允许破坏性调整并提供迁移说明。
- 未写入 SPMS：本次为仓库文档设计，当前未核实 projectId，也未进行平台查重；工作号不得冒充已分配编号。
- 未开始组件重构，未执行产品测试、build、真实浏览器、辅助技术或性能验收；本稿的“必须/通过条件”描述目标，不是验证结论。
- 文档检查仅覆盖本稿结构、工作号/AC/期次映射、引用文件与本地来源；本稿自检不作为一次独立 PRD 评审。

