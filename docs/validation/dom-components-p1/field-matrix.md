# 基础 Themes 字段与旧入口矩阵

计划：[DOM-COMPONENTS-P1](../../plan/DOM-COMPONENTS-P1.md)。冻结基线：`dirty@11b6912919a2a34c372aac4d6b7d51052820600f`。本表是实现/参数测试的输入，不表示新组件已经可用；M4 的逐参数编译、SSR 和浏览器断言逐项承接此表。实现状态以公开能力目录为准。

参考为 PRD §1.5 的本地 Themes 3.3.0 快照，目录指纹 `00a510b8e9b3127046f52ca624aeaa9b15af2e384c42d7fe88cfaae2001a2de9`。以下参考路径均相对 `packages/radix-ui-themes/src/`；不会成为构建依赖。Color 数据和所需主题 token/许可已正式 vendor，见 `sources.lock.json`。

## 共同类型和未设置语义

`R` 表示可响应式：单值或 initial/xs/sm/md/lg/xl 稀疏值；断点为 0/520/768/1024/1280/1640px，缺项沿最近较小断点。属性默认值的“未设”表示 Rust 调用方未选择该属性，不能等同于 CSS 计算默认值。CSS 初始/继承结果在下表另列；具体作者尺寸取决于主题和宿主。

| 类型 | 精确选项 | 来源 |
| --- | --- | --- |
| 品牌色 | gray/gold/bronze/brown/yellow/amber/orange/tomato/red/ruby/crimson/pink/plum/purple/violet/iris/indigo/blue/cyan/teal/jade/green/grass/lime/mint/sky | `props/color.prop.ts` |
| 中性色 | auto/gray/mauve/slate/sage/olive/sand | 同上、`helpers/get-matching-gray-color.ts` |
| 圆角 | none/small/medium/large/full | `props/radius.prop.ts` |
| weight R | light/regular/medium/bold，未设 | `props/weight.prop.ts` |
| text align R | left/center/right，未设 | `props/text-align.prop.ts` |
| trim R | normal/start/end/both，未设 | `props/leading-trim.prop.ts` |
| wrap R | wrap/nowrap/pretty/balance，未设 | `props/text-wrap.prop.ts` |
| truncate | 非响应式 bool，未设时不截断 | `props/truncate.prop.ts` |
| spacing | 0–9；空间基数 0/4/8/12/16/24/32/40/48/64px，经作者 spacing 和 scaling 单次调整 | `styles/tokens/space.css` |
| size 1–9 | 字号基数 12/14/16/18/20/24/28/35/60px；Text 行高 16/20/24/26/28/30/36/40/60px，Heading 行高 16/18/22/24/26/30/36/40/60px | `styles/tokens/typography.css` |

## 主题配置与来源

| 字段 | 值 / 默认 | 继承与行为 | 验证 |
| --- | --- | --- | --- |
| appearance | inherit/light/dark；inherit | 根取显式 mode/默认 Light；嵌套取父有效模式；Studio preview override 独立于作者配置 | V2/V3/V5 |
| accent_color | null 或26品牌；新标准indigo，旧Custom null | 生成 primary/ring/sidebar-primary 配对；不改旧accent | V2/V5 |
| gray_color | null/auto/六中性；新标准auto，旧Custom null | 根auto按参考配对；子只改品牌时继承父resolved灰；显式auto才重配 | V2/V3 |
| panel_background | null/solid/translucent；新标准translucent | card/popover 与独立surface角色；不改变中性壳 | V2/V3 |
| radius | null/五项；新标准medium | finite 1–6/full/thumb 分开；必要形状不跟随full变形 | V2/V4 |
| scaling | 90/95/100/105/110；100 | 只缩放声明几何，不改作者字符串、断点、显式任意CSS尺寸 | V2/V4 |
| ThemeConfigPatch | 每项 Keep/Set；可选配置Set(null)=Custom | Keep撤销局部选择，不将Custom当继承 | V2/V3 |
| ThemeBoundaryPatch | config + 两模式token patch | base→配置→文档override→外到内边界override；空patch复用父快照 | V2/V3 |
| ThemePanel | controlled config、覆盖统计、on_change/on_reset | 不持有另一个文档或历史；拒绝请求后显示实际值；解除覆盖说明模式/分组范围 | V4/V5 |

参考：`components/theme.props.tsx`。`hasBackground` 和通用 `asChild` 不属于本期公开组合承诺，不能把参考辅助参数当成本期已支持属性。

## 操作组件

来源：`components/_internal/base-button.props.ts`、`components/link.props.tsx`、相应 CSS；原生行为来自本仓 `button.rs` / `link.rs`。

| 组件 | 字段 | 属性默认 / 值 | CSS/语义默认及边界 |
| --- | --- | --- | --- |
| Button / IconButton | size R | 1/2/3/4；2 | 高度基数24/32/40/48px；ghost允许自身特殊几何 |
| 同上 | variant | classic/solid/soft/surface/outline/ghost-offset/ghost；solid | 同一原生button行为与色角色路径 |
| 同上 | color | 品牌枚举；未设 | 未设消费有效primary及显式语义覆盖；显式选色仅覆盖本组件角色 |
| 同上 | radius | 五项；未设 | 继承scope对应radius、full只在允许的形状生效 |
| 同上 | high_contrast | false | 根据本组件声明角色提高对比，不推断任意作者组合合规 |
| 同上 | loading | false | 名称/文字占位保留；aria-busy；装饰spinner；原生disabled与点击guard阻止重复提交 |
| 同上 | disabled | false | 原生disabled、handler双检查；优先于其他状态 |
| 同上 | unavailable | false | aria-disabled、保持可达并交付请求，由应用拒绝 |
| 同上 | type | button/submit/reset；button | 保留原生表单语义 |
| 同上 | on_click | 必须 | 离散请求一次；disabled/loading不交付 |
| Button | aria_label | 可选 | 空值不移除children提供的名称 |
| IconButton | accessible name | 必填 | 图标装饰；复用Button，不复制点击状态机 |
| 同上 | controls / expanded | 空 / None | 无控制对象时不生成无意义ARIA；expanded支持None/true/false |
| 同上 | class / test_id / children | 空 / 空 / children | utility merge允许同类调用方覆盖；图标按钮也保留内容占位 |
| Link | href | 必须 | 原生a；disabled移除href |
| Link | match_ | Within/Exact；Within | 原路径匹配与aria-current保留 |
| Link | size R | 1–9；未设 | 继承字号，不默认强制size2 |
| Link | weight R / trim R / wrap R | 共同类型；未设 | 各自继承/初始值；不塞入无关布局参数 |
| Link | underline | auto/always/hover/none；auto | 原生文本链接，不包button模拟 |
| Link | color / high_contrast / truncate | 未设 / false / false | 未选色消费有效primary；截断非响应式 |
| Link | disabled / class / test_id / children | false / 空 / 空 / children | 保留名称与ARIA、class merge |
| Link | 原生 target / rel 等标准属性 | 未设 | Leptos原生属性透传，不接管浏览器导航 |

## 布局组件及共享参数

来源：`components/{box,flex,grid,container,section,inset}.props.tsx` 与同名实现/CSS。

| 组件 | 独有字段（R除as外） | 属性默认 | CSS默认 / 必要行为 | 共享组 |
| --- | --- | --- | --- | --- |
| Box | as div/span；display none/inline/inline-block/block/contents | div；display未设 | border-box、display:block；span也遵守组件display默认 | Layout + Margin |
| Flex | as div/span；display none/inline-flex/flex；direction row/column/row-reverse/column-reverse；align start/center/end/baseline/stretch；justify start/center/end/between；wrap nowrap/wrap/wrap-reverse；gap/gap_x/gap_y | div；其余未设 | display:flex；justify-content:flex-start；between→space-between | Layout + Margin + Gap |
| Grid | as div/span；display none/inline-grid/grid；areas CSS；columns/rows 1–9或CSS；flow row/column/dense/row-dense/column-dense；align start/center/end/baseline/stretch；justify start/center/end/between；align_content start/center/end/baseline/between/around/evenly/stretch；justify_items start/center/end/baseline/stretch；gap/gap_x/gap_y | div；其余未设 | grid；align-items:stretch；justify-content:flex-start；单列minmax(0,1fr)，rows:none；between/around/evenly映射space-* | Layout + Margin + Gap |
| Container | size1–4；align left/center/right；display none/initial | size4；其余未设 | 外层flex column居中，inner width100%；max-width448/688/880/1136px，响应式；initial→flex | Layout + Margin；width/height投影到内容层 |
| Section | size1–4；display none/initial | size3；display未设 | section；上下padding24/40/64/80px再缩放；initial→block | Layout + Margin |
| Inset | side all/x/y/top/bottom/left/right；clip border-box/padding-box；p/px/py/pt/pr/pb/pl current/0 | side=all，clip=border-box；padding未设 | 只抵消最近容器inset；不是任意负margin入口 | Margin；自身受限padding，不接入任意Layout |
| Reset | 显式单个原生Leptos element装饰入口 | 不创建包装元素 | 保留元素类型、语义、事件；不接受悄悄丢元素的多children；局部reset | 原生元素属性，非Layout集合 |

所有共享字段如下，均R、默认未设；不为Em/Quote/Strong等不适用组件新增整组布局API。值须通过同一SSR/CSR校验，拒绝多声明、规则、URL、外部导入；无效值按prop定位。

| 组 | 逐字段 | 接受值 | 来源 |
| --- | --- | --- | --- |
| Padding | p/px/py/pt/pr/pb/pl | 0–9或合法对应CSS长度；负刻度不属于padding枚举 | `props/padding.props.ts` |
| Margin | m/mx/my/mt/mr/mb/ml | -9–9或合法CSS长度 | `props/margin.props.ts` |
| Gap | gap/gap_x/gap_y | 0–9或合法CSS长度 | `props/gap.props.ts` |
| Width | width/min_width/max_width | 合法对应CSS值 | `props/width.props.ts` |
| Height | height/min_height/max_height | 合法对应CSS值 | `props/height.props.ts` |
| Layout | position | static/relative/absolute/fixed/sticky | `props/layout.props.ts` |
| Layout | inset/top/right/bottom/left | -9–9或合法对应CSS值 | 同上 |
| Layout | overflow/overflow_x/overflow_y | visible/hidden/clip/scroll/auto | 同上 |
| Layout | flex_basis | 合法CSS值 | 同上 |
| Layout | flex_grow/flex_shrink | 0/1或合法数值，不接受声明字符串 | 同上 |
| Layout | grid_area/grid_column/grid_column_start/grid_column_end/grid_row/grid_row_start/grid_row_end | 合法对应CSS值 | 同上 |
| Layout | align_self/justify_self | start/center/end/baseline/stretch | 同上 |

## 排版组件

来源：`components/{text,heading,blockquote,code,kbd,em,quote,strong}.props.tsx`、同名实现/CSS。各组件保留原生属性和class；不支持通用asChild/Slot。

| 组件 | 原生元素及属性默认 | 逐字段 | 额外共享组 / CSS边界 |
| --- | --- | --- | --- |
| Text | span/div/label/p，默认span；size未设 | size1–9 R、weight R、align R、trim R、wrap R、truncate、color、high_contrast | Margin；默认继承字号，行高取有效主题 |
| Heading | h1–h6，默认h1；size6 | size1–9 R、weight R、align R、trim R、wrap R、truncate、color、high_contrast | Margin；默认bold，视觉size不改变heading层级 |
| Blockquote | blockquote；size未设 | size1–9 R、weight R、wrap R、truncate、color、high_contrast | Margin；不凭空增加align/trim |
| Code | code；size未设，variant soft | variant solid/soft/outline/ghost；size1–9 R、weight R、wrap R、truncate、color、high_contrast | Margin；使用主题mono字体与代码字号比例 |
| Kbd | kbd；size未设，variant classic | size1–9 R；variant classic/soft | Margin；不增加color/weight/trim API |
| Em | em | wrap R、truncate | 无Margin/Layout；继承字号，无独立size/color |
| Quote | q | wrap R、truncate | 同上；保留原生引用标点 |
| Strong | strong | wrap R、truncate | 同上；使用强调语义 |

## 旧公共入口去向

以 `crates/rustify-components/src/lib.rs` 和 `button.rs`、DOM主题公开重导出为基线；完整参考稳定集合及非P1子能力在M6目录收口，不将这张表冒充已实现组件总数。

| 旧入口 | 本期去向 / 约束 |
| --- | --- |
| Button / button::ButtonClass / ButtonVariant / ButtonSize | 同一Button实现更新；Class参数迁到组件参数/公开新枚举，不保留LegacyButton |
| Default / Secondary / Outline / Ghost | Solid / Soft+gray / Outline / Ghost |
| Accent / Destructive / Link variant | 同一局部色角色路径消费旧accent/destructive；导航改Link，纯操作改Ghost+显式下划线 |
| size Default / Sm / Lg / Icon | 2 / 1 / 3 / IconButton size2 |
| Link / LinkMatch / CurrentPath / provide_current_path / current_path / is_within | 原生导航和路径语义保留；增加声明的视觉参数 |
| Checkbox / RadioGroup / RadioOption / Switch / Select / SelectOption / Slider / TextField / TextKind / TextArea / Label | 保留当前受控功能；只适配共享主题；三态、部件组合、多滑块留后续 |
| Form / Field / FieldBinding / FormStatus / SubmitButton / provide_form / use_form | 保留业务、原生表单和校验能力；不被重构删除 |
| Dialog / Menu / MenuItem / MenuItemKind / anchor_at / Tooltip | 保留scope浮层、焦点和有效主题订阅；不在本期伪称完整Primitives部件API |
| Tabs / Tab / TabPanel / Orientation / ScrollArea / Boundary | 保留当前功能；不拿布局新组件替代 |
| Progress / Spinner / Toast / Toaster | 保留当前反馈功能；新Skeleton等留后续 |
| NumberField / ColorField / DataTable / Column / Tree / TreeNode | 保留Rustify扩展和Draft例外；不重写大数据路径 |
| FilePicker / DropZone / Splitter / PanelTabs / PanelTab / CommandPalette / Command | 保留工作区/文件/命令扩展 |
| Icon / Glyph / id / roving / geometry | 保留辅助能力；IconButton复用现有图标体系 |
| clx! / variants! / macros / cn / paste / CSS / CANDIDATES_JSON / styles / GeometryStyles / provide_geometry_styles | 保留class合并、包内样式消费、SSR外部样式收集；扩展媒体规则 |
| CATALOG / Capability / Category / Entry / Presentation / Support | 扩展能力事实源；24是旧分类数，不是完整公共符号数 |
| ThemeDocument.styles / values / values_mut | 改base_values(mode)显式访问base；编辑用override命令；迁移所有直接.styles消费者 |
| ThemeDocument::from_legacy / Theme / ThemePatch | 保留旧值语义的v2 Custom适配；旧Theme值类型不改名为组件 |
| ThemeScope / ThemeBoundary / provide_static_theme / use_resolved_theme / ResolvedTheme::patched | 同revision发布完整双模式来源与快照；边界不能把已缩放当前快照复制成双模式来源 |
| rustify_components::theme::{Theme,ThemePanel,ThemeBoundary} | 新公共Themes入口；Theme重导出增强后的ThemeScope，不另造主题所有者 |
| import_json / import_css / export_json / export_css | 同一codec保留旧输入；完整v2默认导出；兼容v1显式列损失 |

所有未交付参考能力、通用asChild/Slot、完整Primitives与unstable候选都必须标明状态；本期DOM呈现，不新增GPU对等组件。
