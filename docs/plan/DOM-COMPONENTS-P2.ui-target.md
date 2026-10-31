# DOM-COMPONENTS-P2 · UI 验收目标包

- 建立日期：2026-10-31；首轮；人工流程，无 managed 索引。
- 来源：[开发计划](DOM-COMPONENTS-P2.md) §5.2–§6、§9.4；[PRD](../PRD-DOM-COMPONENTS.md) WORK-F1/5–9/15/19、WORK-N1 的 P2 AC 与 UAT-4/6/9；仓根 PRODUCT.md / DESIGN.md。
- 本期呈现路径：DOM。GPU 仅保留既有预览并明确新增表单仅 DOM。
- 无本次指定或计划链接的新增表单目标图，采用规范模式。P1 旧截图只作壳/预览边界的背景，不作为新表单像素目标。
- 首轮假设：英文是固定主采集语言，中文为语言切换补充；桌面 1440×1000、平板 768×1000、手机 390×844，DPR 1。各枚举按成对组合覆盖，不做全部维度笛卡尔积。

## 单元

| 单元 | 必需状态与采集矩阵 | 目标 | 比较口径 |
| --- | --- | --- | --- |
| U1 Studio 完整本地表单 | 默认：三个视口 × light/dark；桌面 light：constraint、rule、键盘焦点、成功及 FormData、application 错误与重试、reset；手机 dark：中文及长内容 | U1-1–U1-5、G-1–G-4 | 原生编辑值/演示数据不要求像素固定；主区域、标签、帮助、错误来源、保存反馈均须可读可达 |
| U2 选择与选择卡片状态工具 | 桌面 light：Checkbox、Switch、RadioGroup、CheckboxGroup、RadioCards、CheckboxCards 各配对 size1/classic、size2/surface、size3/soft（Cards size3/classic）；Toggle/ToggleGroup 默认；代表性的 disabled/readonly/invalid、拒绝、延迟接受；dark、390px 各一代表 | U2-1–U2-3、G-1–G-4 | 覆盖每个公开尺寸/变体值，非每一个组合；局部 color/high contrast 各代表一例；Primitives 不按 Themes 皮肤打分 |
| U3 字段与插槽 | 桌面 light：TextField、TextArea、NumberField、ColorField 各配对 size1/classic、size2/surface、size3/soft；disabled/readonly/invalid/focus；聚焦未完成 Number/Color Draft 及失焦/取消；TextField Slot、RTL、utility 覆盖的实际入口；dark 代表 | U3-1–U3-4、G-1–G-4 | 未完成 Draft 可暂时不同于接受值；正常文本编辑不得丢光标/选区；Slot 只评价真实公开入口可达的声明能力 |
| U4 单值与多滑块 | 桌面 light：Slider/RangeSlider 配对 size1/classic、size2/surface、size3/soft；键盘跨越后焦点帧、拒绝请求、完成/取消；raw 竖向/反向；dark/RTL 代表 | U4-1–U4-3、G-1–G-4 | 比较接受值、请求、完成/取消和稳定焦点；不以动画瞬时位置代替最终接受值 |
| U5 Select | 桌面 light：Trigger 的 classic/surface/soft/ghost-offset/ghost 与 size1/2/3 配对；Content solid/soft；实际打开、键盘字符定位/选择/关闭焦点、取消选择、空/长列表；390px dark/RTL 打开 | U5-1–U5-3、G-1–G-4 | 无搜索 Combobox 要求；原生镜像不是第二个可见选择器；长列表只能局部滚动 |
| U6 独立无皮肤消费与兼容入口 | raw 独立消费者：1440/390 light、选择拒绝/FormData、可取消 Trigger、Portal 方向、原生表单阻塞/成功；现有 catalog 和 property-workbench 表单各桌面默认一例；Studio compare DOM/GPU 标识一例 | U6-1–U6-3、G-1–G-4 | raw 本身是正式验收入口，不要求品牌样式；仅检查本期影响的表单及主任务，不重评整个既有应用 |

## 规范要点

- G-1：Studio 使用 Operate 方向，中性编辑壳、蓝色选中/焦点，作者主题仅影响预览及所属浮层。壳 IBM Plex Sans 14px、行高 1.5；编辑区 320px、预览 minmax(0,1fr)，两边独立滚动；600px 以下 Edit/Preview 切换，390px 表单单列、组/卡片换行，长值/错误折行，二维数据才局部滚动。（DESIGN.md Typography/Layout；计划 §6）
- G-2：不为每个字段加装饰卡片；第一层完整可提交表单，第二层可展开 States and parts；控件有标签、帮助、状态，不只用颜色反馈。默认普通文本 4.5:1、必要非文本边界 3:1；焦点可见，键盘能完成主任务。（计划 §6；PRD WORK-N1/AC-1）
- G-3：所有显示值来自应用接受值；禁用/只读不发值变更；策略工具清楚区分请求数、最后请求和应用值。中文切换翻译名称/帮助/错误/控制文案，作者输入及业务值不翻译。模式/宽度不修改主题文档和编辑历史。（计划 §6；PRD WORK-F19/AC-1）
- G-4：size 默认2、variant 默认 surface。默认配置主题为 indigo 品牌、auto 中性、translucent panel、medium radius、100% scaling；兼容配色不是强制作者预览色。组件局部颜色只在自身边界生效。必要方框/圆/胶囊不被普通 radius 破坏；声明的局部 color、radius、high contrast、scaling 与调用方 utility 有可见效果。（DESIGN.md Colors/Shapes；计划 §5.6）
- U1-1：Cards 的 Form components 是真实可提交本地表单，含三态/组、卡片、标量/范围、Number/Color Draft、分组 Select，实际 FormData 与保存状态可见。（计划 §6）
- U1-2：constraint、rule、check、application 错误可区分，错误靠近字段并关联实际 Control；提交失败聚焦首错；busy 可辨识且不重复保存；失败后修改可重试。（计划 §5.4；PRD WORK-F8/AC-1/3）
- U1-3：成功文案明确仅本地演示；reset 由应用接受后清值/错误/草稿，不把旧异步结果带回；拒绝 reset 时接受值/FormData 不变。（计划 §5.4、§6）
- U1-4：提交前完成当前 Draft，但 FormData 仅包含应用接受值；名称、说明、错误和 FormData 在桌面/平板/手机浅深色可读可操作。（计划 §5.4、§6）
- U1-5：键盘完成组选择、Select 和提交；输入选区/剪贴板/原生撤销可用；不把合成事件当 OS IME、VoiceOver 或真机证据。（计划 §9.4）
- U2-1：Checkbox 正确区分 unchecked/checked/indeterminate；RadioGroup 单选，CheckboxGroup 多选，ToggleGroup 规则明确；普通选择与卡片在相同表单条件下结果一致，已有可见说明不得丢失关联，不要求每张状态样本卡片另加说明。（PRD WORK-F6/AC-1–3；2026-10-31 用户澄清）
- U2-2：RadioGroup/ToggleGroup 一个 tab stop，箭头移动、跳禁用、Home/End；CheckboxGroup 各项独立 Tab；只读可移动焦点但不请求值；拒绝/延迟后候选不假装已接受。（计划 §5.2）
- U2-3：Checkbox/Radio/组/Switch 支持 size1/2/3、classic/surface/soft；Cards 支持 size1/2/3、surface/classic；Toggle 延用既有外观，不要求不存在的上游变体。（计划 §5.6）
- U3-1：TextField/TextArea/NumberField/ColorField 的 size1/2/3 和 classic/surface/soft 有效；字段标签/名称/帮助指向编辑元素；disabled、readonly、invalid、focus 可区别。（计划 §5.4/5.6）
- U3-2：Number/Color 使用 focused Draft，Enter/blur 一次 commit，Escape 取消，失焦回接受值；未完成文本不进入命名 FormData；普通字段保留原生选区/撤销。（计划 §5.4）
- U3-3：Slot 默认中性11号前景，显式 color 使用自身 accent11，不随 Root 染色；size1/2/3 默认 gap2/2/3、左右 padding1/2/3；逻辑 Start/End 随 RTL 交换，物理 pl/pr 不换；Responsive 就近继承、pl/pr 覆盖 px、utility 覆盖组件。（计划 §5.6）
- U3-4：TextArea 响应式 resize、长文本和长错误不遮挡；公开输入/插槽没有多余包装造成的焦点或布局退化。（计划 §5.4/5.6、§6）
- U4-1：单/多值 Slider 支持3个 size、classic/surface/soft、局部色/半径；每个 thumb 有名字和值，可键盘操作。默认允许跨越，DOM/focus 身份固定，按数值排序输出。（计划 §5.3/5.6；PRD WORK-F7/AC-1）
- U4-2：min/max/step/方向/反向/RTL 一致，竖向向上增加；preserve_order/间隔限制有效；Arrow/Home/End/PageUp/PageDown 同一路径。（计划 §5.3）
- U4-3：连续请求与一次完成分开；拒绝时显示接受值而完成交付最后候选；取消不 commit；不把 UI 工具的请求数当保存次数。（PRD WORK-F7/AC-2；计划 §5.3）
- U5-1：组合 Select 有 placeholder、分组标签、禁用项、选中指示；键盘打开/字符定位/选择/Escape，正常关闭回到有效 Trigger；取消钩子保留原值和焦点。（计划 §5.5；PRD WORK-F9/AC-1）
- U5-2：空/已选移除显示 placeholder，无虚假选中；长列表活动项滚在自身 viewport；浮层适配可用视口，RTL 对齐正确；外点关闭不抢回焦点。（计划 §5.5；PRD WORK-F9/AC-2）
- U5-3：Root size1/2/3；Trigger classic/surface/soft/ghost-offset/ghost；Content solid/soft；浮层继承创建处主题/语言/方向；不向不声明的部件附加能力。（计划 §5.5/5.6）
- U6-1：仅 Primitives 的消费者可完成相同表单行为，不加载 Themes 皮肤/品牌 token/全局 reset；自定义语义元素保持名字/原生行为，取消动作后无默认动作，Portal 保留 scope 方向并可清理。（PRD WORK-F1/AC-2、WORK-F5/AC-1–4）
- U6-2：独立消费者标签、状态、键盘、接受值/FormData 一致；既有 catalog 和 property-workbench 的字段/表单仍可完成原有任务。（计划 §9.3/9.4、V-9）
- U6-3：Studio compare/GPU 保留既有内容，新表单明确仅 DOM，不展示空白 GPU 表单或承诺对等。（计划 §6）

## 采集

- 入口：`mbx xtask build-web --example theme-studio --release` 后 `mbx xtask serve --example theme-studio --release --port 4181`；从站点 renderer DOM → Cards 进入。raw 通过 `mbx xtask build-dom --manifest-path tests/dom-only/primitives/Cargo.toml` 及已有 strict-CSP 静态 server，端口4190。
- 兼容页按仓内 build-web/serve 构建 catalog:4176、workbench:4174，串行浏览器使用；不安装服务。
- Playwright + 本机 Google Chrome；DPR1、字体完成、动画截图禁用；无账号、无外部业务数据；仅页面本地演示数据。
- 每张图片记录实际 URL、时间、viewport、主题、动作和可见结果。状态工具采用实际 select/checkbox/buttons 操作，不注入替代 UI、不合成截图；长页可分段移动真实滚动容器。
- 同一组截图按枚举配对；每个实际采集格子详见 R1 capture.md。未到达状态明确列缺证，不以实现推断。
- OS IME、VoiceOver、真机触控、浏览器 UI 原生200%缩放按计划留 P5 missing，不把重排模拟算上述通过。

## 变更记录

- 2026-10-31 R1 建立：从计划、PRD 和既有设计规范抽取期望；本次尚无用户确认的新目标图，全部使用规范模式。期望冻结；后续仅采集节可由评审自行补充。
- 2026-10-31 用户确认：“只要求已有说明正确关联（推荐）”。修正 U2-1 的首轮抽取歧义：不要求每张状态工具卡片都额外增加说明。R1 原目标保留在 DOM-COMPONENTS-P2.ui-target.r1.md；R1 评分和评审原文不回改，下一轮按本包的新指纹重建基线。
