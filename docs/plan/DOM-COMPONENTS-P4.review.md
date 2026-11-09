# DOM-COMPONENTS-P4 · 完整开发计划评审

## 评审对象

| 类型 | 原文档 | 所审版本 | 评审范围 |
| --- | --- | --- | --- |
| 开发计划 | [DOM-COMPONENTS-P4 · 完整导航与参考交互](DOM-COMPONENTS-P4.md) | v0.1；SHA-256 `319776afdf961066ccdbff72d1be9ad58fd34da6d8fb4afebed5f32d8e451471`；未提交工作树 | 全文；P4 的 10 条 AC 与 WORK-F1/F15/N3/N6 整条终验；M1–M7、V-1–V-15、T-1–T-8 |

评审日期：2026-11-04（Asia/Shanghai）。代码基线：main / `0d96270fc5eae9cc9a71774649f3254499bf3f20`，dirty。未提交内容只有 SDLC 台账、`skills-lock.json`、P3 评审与覆盖报告和本计划，产品源码无改动。交付前再次核对，HEAD 与计划 hash 均未变化。本轮是首次完整评审，不是复审。

## 1. 确认问题

共 23 项：P2 11 项，P3 12 项；无 P0、P1。多视角报告的同根因问题已合并，并保留各自来源。所有问题都不需要用户或外部提供答案，阻塞均为“否”。计划尚未实施，未把 ★新增项计为缺陷。

### P2

1. **[P2] F-01 · floating 的增量不止“四项”，照原文实现菜单 Trigger 会先开后关，便捷 Menu 打不开**
   - 定位：[计划 ADR-1](DOM-COMPONENTS-P4.md#adr-1菜单复用-floating-受控生命周期):272；[§5.1](DOM-COMPONENTS-P4.md#51-floating-增量与共享菜单原语):408–413；§5.2:456–458；§1.2:351；§12:723。关联 WORK-F13/AC-1、AC-2，M2/T-1。
   - 问题：§1.2 让菜单 Trigger 复用 floating 的 `button_as`，§5.1 断言 floating 只增四项。实际 `button_as` 有三处与菜单契约冲突：
     - 它无条件挂 click 切换。§5.2 要求 DropdownMenu 在 pointerdown 时请求切换、Enter/Space 在 keydown 时请求切换；应用接受后，紧随的 click 会再请求一次关闭，形成先开后关。
     - 它对所有 Trigger 无条件输出 `aria-controls`，与 §5.2、§12 要求的“只在打开时输出”不符。
     - 锚定类 kind 在既无 Anchor 部件又无 trigger_id 时，Portal 的 `active` 恒为 false，`issue()` 报 missing-anchor。便捷 Menu 只设 virtual anchor，因此永远打不开；ContextMenu 的非 button Trigger 也需要另行注册。

     另外，正文没有约定 SubContent 的 floating root 固定 `modal=false`。若继承 Root 的 `modal=true`，子层会 inert 父 panel，floating 的 focus 钩子也会阻止 focus-outside，§5.1 依赖的“宽限外移到父项时 core 的 focus-outside 关闭子层”就失效了。
   - 证据：[floating.rs](../../crates/rustify-components/src/primitives/floating.rs):221–236（click → `change.run(!open)`）、:248–258（haspopup 固定 dialog，aria-controls 无条件输出）、:85–88 和 :361–362（active/issue 门控）、:384（`context.modal` 时 focus 钩子 `prevent_default`）；[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1063、:1255–1282。参考实现 `ref/primitives-main/packages/react/dropdown-menu/src/dropdown-menu.tsx` 的 Trigger 只处理 onPointerDown/onKeyDown，不处理 onClick。
   - 修订：把 §5.1 的增量改为如实列举：
     - 菜单族自带 Trigger，复用 `floating::register(context.triggers, …)` 和 `trigger_id`，不经 `button_as` 的 click 切换，haspopup 与开启时的 aria-controls 由菜单 Trigger 自己输出。
     - `issue()` 与 Portal `active` 把 `virtual_anchor.is_some()` 视为有锚点。
     - Sub 一律以 `Kind::Menu, modal=false` 建 floating root。

     同步改写 ADR-1 的“只增四项 / 菜单专属行为不进入 floating”措辞和 §1.2 的复用说明。“既有 kind 行为不变”保留。
   - 复核：ADR-1、§1.2、§5.1、§5.2 对 floating 改动面的描述一致；§5.2 的 Trigger 行写明不挂 click 切换。
   - 置信度：高（行为级代码证据，facts 与 architecture 两视角独立发现）；阻塞：否。

2. **[P2] F-02 · 整树外部监听绕过了根 Content 的公开可取消钩子**
   - 定位：[ADR-2](DOM-COMPONENTS-P4.md#adr-2子菜单逐层关闭菜单树外指针整树关闭) 备选 B②:282；§5.1“整树关闭”:447；Content 钩子签名:421–423。关联 PRD §2.6、WORK-F13/AC-1。
   - 问题：Sub 打开时，core 只对最上层（子菜单）调用外部钩子，所以根 Content 的公开钩子 `on_pointer_down_outside`、`on_interact_outside` 不会运行。ADR-2 新增的 document 监听又只看 DOM 事件是否被取消，就直接请求根关闭。结果是：应用在根上取消了外部关闭，只要有子菜单打开，整棵菜单树仍会关闭。这违反 PRD §2.6“公开可取消的……外部交互……遵守取消结果”。
   - 证据：[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1177–1199（pointerdown 只取 `layers.last()`）、:117–134（钩子以 `Cancelable` 判定，与 DOM 事件的 defaultPrevented 无关）；[PRD §2.6](../PRD-DOM-COMPONENTS.md#26-公共交互契约)。
   - 修订：ADR-2② 与 §5.1 改为：监听命中树外时，以 `LayerDismiss::PointerOutside` 构造 `Cancelable`，依次运行根 Content 的 `on_pointer_down_outside` 和 `on_interact_outside`，未取消才请求根关闭一次。同时写明：子层在同一次点击中收到的 PointerOutside 关闭请求属于预期，不计为重复请求。
   - 复核：ADR-2 与 §5.1 两处文字一致；V-2“整树关闭模型”的用例描述包含“根钩子取消时保持打开”，并入既有用例，不新增检查。
   - 置信度：高；阻塞：否。

3. **[P2] F-03 · 菜单 Content 里的 P3 scroll_area viewport 是 Tab 停靠点，与 D4 和模态 Tab 包裹冲突**
   - 定位：[§5.6](DOM-COMPONENTS-P4.md#56-themes-视觉参数与样式):499（“Content 内用 P3 `scroll_area`（visibility Auto）承载项”）；D4:256；§5.1 Tab:442。
   - 问题：Rustify 的 scroll_area viewport 固定为 `role="group" tabindex="0"`。模态菜单中，core 的 `keep_tab_inside` 在 scope capture 阶段先于 Content 运行。由于菜单项都是 `tabindex=-1`，viewport 成了唯一停靠点，Tab 会被包裹到 viewport：项高亮丢失，菜单里还多出一个可聚焦、会被朗读的 group。D4 要求的“焦点不移出”实际变成“焦点落到滚动框”。Radix 的 ScrollArea viewport 没有 tabindex，所以参考实现没有这个问题。
   - 证据：[scroll_area.rs](../../crates/rustify-components/src/primitives/scroll_area.rs):216；[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1103–1118、:1512–1535、:347–357；`ref/primitives-main/packages/react/scroll-area/src/scroll-area.tsx`（无 tabIndex）；`ref/themes-main/.../dropdown-menu.tsx` 用 `ScrollArea type="auto"`。
   - 修订：二选一，写入 §5.6：
     - 菜单 Content 改用无停靠点的 `overflow:auto` 容器，`max-height` 取 `--rustify-floating-available-height`；
     - 或给 scroll_area viewport 增加 crate 内参数，省略 tabindex 与 role。
   - 复核：§5.6 不再无条件写“P3 scroll_area”，或注明已去掉停靠点。
   - 置信度：高；阻塞：否。

4. **[P2] F-04 · ContextMenu 打开期间的再次右键、Trigger 区内左键与浮层机制不兼容**
   - 定位：[§5.2](DOM-COMPONENTS-P4.md#52-dropdownmenu-与-contextmenu) ContextMenu 行:457（“已开只更新锚点、不再请求”）；§5.1 模态:448。
   - 问题：
     - 模态（默认）下，打开期间 scope 内容被 inert，inert 节点不参与命中测试，Trigger 收不到 `contextmenu`，“已开只更新锚点”的路径走不到。实际会发生：右键 pointerdown 被 core 判为外部而关闭菜单，随后 contextmenu 落到非 inert 的祖先，弹出浏览器原生菜单。
     - 非模态下，`anchor_at` 生成的 `Anchor::Region{canvas: trigger}` 会被 `inside()` 视为内部，Trigger 区域内左键不会关闭菜单，与参考实现不同。
   - 证据：[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1063（claim_inert）、:1452–1475（Region canvas 算内部）；[menu.rs](../../crates/rustify-components/src/menu.rs):154–170（`anchor_at` 构造 Region）。
   - 修订：§5.2 按模态和非模态分别补两条组件层规则，core 不变：
     - 打开期间按需持有 document capture `contextmenu` 监听（`rustify_ui::listen`，实例 abort 绑定）：坐标落在 Trigger rect 内时 `prevent_default`，重写 virtual anchor，未开则请求打开。
     - 非模态时，Trigger 区域内主键 pointerdown 请求关闭一次。
   - 复核：§5.2 的“已开”语义按模态和非模态各写一句；行为由 M2 的 menus 样本与 V-5 的 ContextMenu 用例验证。
   - 置信度：中高（依据规范语义与代码推导，未在浏览器实测，见待核实 U-02）；阻塞：否。

5. **[P2] F-05 · Menubar 顶层切换缺少 focus-outside 与关闭回焦的抑制**
   - 定位：[§5.3](DOM-COMPONENTS-P4.md#53-menubar):465–466。
   - 问题：
     - 菜单 A 打开时，按计划 pointerenter 另一 Trigger B 会“请求切换并聚焦它”。聚焦 B 触发 core 对非模态顶层 A 的 focus-outside 关闭。应用拒绝或延迟接受切换时，A 会再请求一次 `None`，一次指针动作产生两个请求，违反 §2.6 的单次请求契约。
     - 键盘切换时，A 卸载后的延迟回焦会先把焦点送回 Trigger A，再进入 B 的内容，读屏多播一次 A。
     - 计划在子 panel 内整体忽略 ArrowRight/Left；参考实现和 APG 都只忽略后退键，在子菜单里非 SubTrigger 项上按前进键会移到下一个顶层菜单。

     参考实现 MenubarContent 对前两点都有处理：Trigger 上的 focus-outside 执行 `preventDefault`，菜单栏仍打开时 `onCloseAutoFocus` 阻止回焦。
   - 证据：[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1255–1282、:2038–2063；`ref/primitives-main/packages/react/menubar/src/menubar.tsx`:324–337、:351–361。
   - 修订：§5.3 补三条：
     - Menubar Content 的 `on_focus_outside` 在目标为本 Menubar 的 Trigger 时取消；
     - `on_close_auto_focus` 在 Menubar value 仍为 Some 时取消，只有 Escape 或真正关闭时才回到本 Trigger；
     - 子 panel 内只忽略后退键。
   - 复核：§5.3 文字；menus.spec 已有的拒绝路径用例包含“拒绝切换时请求计数为 1”，不新增检查。
   - 置信度：中（时序依据代码推断，见 U-03）；阻塞：否。

6. **[P2] F-06 · ADR-4 的 CSSOM 匹配语义没有定义，对 Tailwind v4 产物不可靠，与缓存键也不一致**
   - 定位：[ADR-4](DOM-COMPONENTS-P4.md#adr-4inspector-从实际部件推导-token-与主题来源):303–306；[§5.8](DOM-COMPONENTS-P4.md#58-主题来源标记与-studio-完整陈列) Inspector 第 2 条:521；§7:554。关联 WORK-F19/AC-2。
   - 问题：已提交的 rustify.css 是 Tailwind 4.1.13 的嵌套产物，例如 `.hover\:bg-muted { &:hover { @media (hover: hover) { … var(--muted) } } }`；dark 变体是嵌套的 `&:where(…)` 加 `@container style()`；皮肤用了 `:where(.x:hover)`、`[data-state]` 和 `::after`。计划只写了“递归 @layer/@media/@supports + `matches(selector)`”，会出四个问题：
     - 嵌套 style rule 不在递归范围内，变体 token 全部漏掉；若对 `&:hover` 直接 `matches`，结果也错。
     - 带伪元素的选择器 `matches` 恒为 false，例如 `::after` 引用的 token 永远收集不到。
     - `@container style()` 的条件无法用 `matches` 判定。
     - `:hover`、`data-state` 规则只在当下状态命中，而缓存键不含状态，结果取决于第一次拾取时的状态。

     另外，`choose()` 在 mousemove 上运行，§7“样式表扫描只在拾取时执行”与现有代码不符。
   - 证据：[rustify.css](../../crates/rustify-components/css/rustify.css):598–606，另有约 39 处 `&`；[panels.css](../../crates/rustify-components/css/panels.css):15–24；[inspector.rs](../../examples/theme-studio/src/inspector.rs):300–313。
   - 修订：在 ADR-4 写定一种与状态无关的语义，建议：
     - 对顶层 style rule（递归 @layer/@media/@supports/@container）取“去掉伪元素和用户动作伪类后的主体选择器”与元素做 `matches`；
     - 命中后从该规则完整的 `cssText`（含嵌套变体）收集 `var(--token)`，结果即“该部件在任何状态下引用的 token”，与现有缓存键一致。

     同时把 §7 改为“拾取与悬停预览共用缓存，按键首次计算”。
   - 复核：ADR-4 与 §5.8 第 2 条一致；theme-gallery.spec 的 Inspector 用例断言菜单项无论是否 highlighted 都列出同一组 token，并入既有用例。
   - 置信度：高（产物结构已读）；阻塞：否。开工前建议做一次小实验，见 U-01。

7. **[P2] F-07 · 拾取模式与浮层的交互没有定义：无法打开再检查浮层，也会吞掉 Studio 自身的弹层**
   - 定位：[§5.8](DOM-COMPONENTS-P4.md#58-主题来源标记与-studio-完整陈列) 陈列:515（“浮层/菜单从样本触发器真实打开”）、Inspector 第 1 条:520；§6:537；§9.3 Studio:612；V-9。关联 WORK-F19/AC-2。
   - 问题：
     - 先开拾取再开菜单：拾取模式在预览根的 capture 阶段对 pointerdown 和 click 执行 `prevent_default` 加 `stop_immediate_propagation`，指针点不开 Trigger。
     - 先开菜单再开拾取：拾取开关在预览 scope 的 PreviewToolbar 里。模态菜单打开时它被 inert；非模态菜单则会因外部 pointerdown 而关闭。
     - 只剩“先开拾取、再用键盘打开”这条未写明的路径。而面板提示写着“拾取期间暂停预览动作”；拾取的 Esc 监听挂在 window capture 上且不阻止传播，一次 Esc 会同时退出拾取并关闭菜单。
     - 计划让监听根接受 overlay root 里的目标。PreviewToolbar 的 Preview scene / Preview width 两个 Select 与预览在同一 ThemeScope 下，弹层进入同一个 overlay root，并且在 `[data-inspector-ui]` 之外，拾取模式下这些选项会被吞掉。

     V-9 的“Inspector 选中浮层内容”因此没有确定的使用者路径。
   - 证据：[inspector.rs](../../examples/theme-studio/src/inspector.rs):197–203、:316–350、:356–376、:455；[app.rs](../../examples/theme-studio/src/app.rs):371–379、:535–544；[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1063。
   - 修订：在 §5.8 选定一条使用者路径，二选一：
     - 拾取模式放行 `[aria-haspopup]` 或 Trigger 部件的主键激活，浮层打开后继续拾取；
     - 或陈列中的浮层样本提供“保持展开（非模态）”演示控制。

     同时写明三件事：Esc 先退出拾取、不关闭浮层；`choose()` 只接受来源帧为 root 或已登记预览边界的目标（或给 Studio 自身 Layer 加 `data-inspector-ui` 等价标记）；提示文案同步修改。
   - 复核：§5.8 与 §9.3 Studio 条目描述同一条路径；theme-gallery.spec 以这条使用者路径打开浮层，不直接注入 open 状态。
   - 置信度：代码事实高、修法选择中（delivery、architecture、facts 三视角独立发现）；阻塞：否。

8. **[P2] F-08 · 边界内部件的来源显示与 TokenSource 的编辑对象错位**
   - 定位：[§5.8](DOM-COMPONENTS-P4.md#58-主题来源标记与-studio-完整陈列) Inspector 第 4 条:523（“Inspector 与 `TokenSource` 用该帧解析值与 `ThemeValueOrigin`（含 Boundary configuration/override）”）；§4:396。
   - 问题：TokenSource 的“路径”“解除后”“解除”按钮和 `inspect()` 跳转的编辑字段，都只作用于根主题文档的 `overrides`。计划改为按边界来源显示后，使用者会看到“边界覆盖 · <边界值>”，按钮却去解除根主题的覆盖；按跳转修改根 token，样本也不会变，因为边界值由样本自己的控件决定，例如 ProviderFixture 的外观 Select。边界显示名的来源也没有规定，未登记的边界可能露出 `boundary-<n>` 这种内部 ID，违反 AGENTS.md @user_experience。
   - 证据：[storage_controls.rs](../../examples/theme-studio/src/storage_controls.rs):339–474；[controller.rs](../../examples/theme-studio/src/controller.rs):1289–1300；[provider_fixture.rs](../../examples/theme-studio/src/provider_fixture.rs):70–92。
   - 修订：
     - TokenSource 保持根主题语义。
     - 选中部件属于边界时，Inspector 只读显示“来自局部边界〈标签〉· 由该样本控制；编辑器修改根主题”，并并列根主题的值。
     - 来源登记必须带中英标签；未登记边界显示“未登记的局部边界”，不显示原始 ID。
   - 复核：§5.8 第 4 条与 §4 的失败语义写明以上规则。
   - 置信度：中高；阻塞：否。

9. **[P2] F-09 · floating 改动直到 M7 才有 P3 浮层的浏览器回归，ADR-1 与 V-5 互相矛盾**
   - 定位：ADR-1 负面后果:274（“V-5 的 panels.spec 受影响项、V-8 复跑”）；V-5:581（只有 menus.spec 与调用方 spec）；§11:708。
   - 问题：M2 改的是 Dialog、AlertDialog、Popover、HoverCard、Tooltip 共用的 floating（还要按 F-01 增加分支），V-2 只用 panels_ssg 静态检查 haspopup。按 V-5 现写法，阶段一收口和 M4 提交都没有 P3 浮层的浏览器证据，M5、M6 会在未验证的 floating 上继续开发，回归直到 M7 才会暴露。ADR-1 自己写了 V-5 应含 panels.spec，与 V-5 的命令列互相矛盾。
   - 证据：计划 :274、:581、:708；P3 先例是 core 改动在阶段一就做浏览器回归。
   - 修订：V-5 两个 dom 项目的命令加入 `panels.spec.ts`，与 menus.spec 共用同一次服务和同一产物，不新增门。
   - 复核：ADR-1、V-5、§11 对 panels.spec 执行时点的说法一致。
   - 置信度：高；阻塞：否。

10. **[P2] F-10 · M7 的回归命令选不到预期证据，且同一项目被拆成多次运行**
    - 定位：V-8:584、V-10:586、V-11:587、V-12:588、V-13:589；[§9.4](DOM-COMPONENTS-P4.md#94-构建与真实入口):639。关联 WORK-N3/AC-3、WORK-F15/AC-1。
    - 问题：
      - V-11 的 `--project=dom-only dom-only.spec.ts menus.spec.ts -g lifecycle` 中，`-g` 会过滤所有选中文件，而 `dom-only.spec.ts` 没有一个用例标题含 “lifecycle”，会被全部跳过。被跳过的包括两实例 trap 与重启用例，它们正是 WORK-N3/AC-3“只保证其他独立实例继续可用”的 DOM 侧证据，§1.1:341 也把它列为复用依据。
      - `theme-acceptance.spec.ts -g "GPU|font|fatal"` 还会命中 “actual Chrome 200% page zoom … GPU comparison”，这是 §0.5:322 已划给 P5 的真实缩放用例，与“三个用例”不符。
      - V-8 及 V-11–V-13 中 dom-only、dom-primitives、dom-hydrate、colors-static、components-subpath、official-subpath 的命令没有写 `-c playwright.docs.config.ts`，根配置里没有这些 project，照抄运行会失败。
      - V-13 用 `dom-buttons.spec.ts` 证明 WORK-F15/AC-1，但它只覆盖 Button 和 IconButton；Link 保留原生语义的用例在 `dom-layout.spec.ts`:281，不在 V-13 内。
      - V-8、V-10、V-11 对同一 dom 项目分多次启动服务，在同一 spec 上重复跑子集。
    - 证据：[dom-only.spec.ts](../../tests/dom-only/dom-only.spec.ts):175–450（“lifecycle”只出现在第 5 行注释）；[theme-acceptance.spec.ts](../../tests/browser/theme-acceptance.spec.ts):354、:407–478；[playwright.docs.config.ts](../../playwright.docs.config.ts):137–176；[playwright.config.ts](../../playwright.config.ts):156；[dom-layout.spec.ts](../../tests/browser/dom-layout.spec.ts):281。
    - 修订：M7 每个 dom 项目各一条带 `-c playwright.docs.config.ts` 的合并命令：
      - dom-only：`parity panels content feedback menus dom-only.spec.ts`
      - dom-primitives：`parity panels content feedback menus primitives.spec.ts`

      V-10、V-11 改为引用其中的 a11y/lifecycle 描述块。Studio 第二条改为 `-g "isolated failures"`，对应 describe 标题 “V6 isolated failures preserve the document”。V-13 的 component-catalog 命令追加 `dom-layout.spec.ts`。
    - 复核：V-8–V-13 的命令列里，dom-only 和 dom-primitives 各只出现一次，并分别包含 `dom-only.spec.ts`、`primitives.spec.ts`；Studio 的 grep 只命中 :409、:440、:478；V-13 包含 dom-layout。
    - 置信度：高（用例标题已逐条核对）；阻塞：否。

11. **[P2] F-11 · V-10 视觉矩阵的组合规则没有界定，混入了 P5 维度，签名也会漏掉 focus**
    - 定位：[§9.3](DOM-COMPONENTS-P4.md#93-场景视觉矩阵与角色模拟)“视觉矩阵”:611；V-10:586。关联 WORK-N7/AC-2、WORK-F15/AC-4。
    - 问题：
      - 原文列了全部陈列组件 × variant/size ALL × 多种状态 × 浅/深 × 默认与局部色板 × high_contrast × 调用方覆盖 × 390/768/桌面，没说是全叉积还是各维度各覆盖一次。按叉积理解，约 60 个条目会在已需数十分钟的 theme-studio 项目里产生上千组合。
      - 本期验收行只要求：WORK-N7/AC-2“每个声明变体在其交互状态下可辨认”（默认浅/深）；WORK-F15/AC-4 只针对菜单、导航和工具栏。390/768 属于 WORK-N7/AC-3（P5）；其他组件的色板和 high_contrast 是 WORK-F15/AC-3，P3 已验收，回归由 V-8 承担。
      - 签名只取背景、边框、文字色和阴影，但多个皮肤的 focus-visible 用的是 outline，这些 focus 状态会被误判为“相对 rest 无变化”。
      - “截图供人工复核”没有复核人或复核时点。
    - 证据：[PRD WORK-N7](../PRD-DOM-COMPONENTS.md#work-n7-视觉层次与响应式支持任务) AC-2/AC-3 期次；[panels.css](../../crates/rustify-components/css/panels.css):24（outline focus），另 form-controls.css 5 处、layout.css 1 处。
    - 修订：§9.3 写明组合规则：
      - 全部陈列组件：各声明 variant/size × 适用状态，在桌面宽度、默认色板下做浅/深各一遍。
      - 局部色板、high_contrast、调用方覆盖：只对本期菜单、导航、工具栏族及声明了 high_contrast 的组件各取一例。
      - 390/768：只用于 §6 的应用外壳、菜单族与陈列布局。
      - regression 层用代表子集，完整强度放 evidence 层。

      签名加入 outline 的 style/width/color 和 opacity。截图标明“仅作证据，无人工复核点”。
    - 复核：§9.3 的组合规则能逐条对应 N7/AC-2 与 F15/AC-4 原文。
    - 置信度：中高（schedule、delivery、architecture 三视角独立发现）；阻塞：否。

### P3

12. **[P3] F-12 · 主题来源 ID 只在 wasm 下存在，host 单测与 hydrate 一致性的表述不成立**
    - 定位：§2:374（“rustify-dom 单测”）；V-6:582（`theme::` 的默认与 ssr host 测试，承担“来源 ID 分配/投影”）；§5.7:506；§5.8 core:510。
    - 问题：`ThemeScope`、`ThemeBoundary`、`Layer` 都是 `cfg(target_arch = "wasm32")`，SSG 只走 `provide_static_theme`。因此 host 测不到来源分配和投影，SSG 不输出边界，“来源 ID 由 ThemeScope 内计数器按渲染顺序分配、保持 hydrate 一致”没有对象；在 `provide_static_theme` 下使用 ThemeBoundary 时也没有 ThemeScope 计数器可取。
    - 证据：[provider.rs](../../crates/rustify-dom/src/theme/provider.rs):100–102、:184–186；[theme.rs](../../crates/rustify-dom/src/theme.rs):46–47；[overlay.rs](../../crates/rustify-dom/src/overlay.rs):340–345。
    - 修订：
      - 删去 §5.7 的来源 ID 与 hydration 一句。
      - 来源 ID 改由 ThemeBoundary 在浏览器端自取唯一 ID，不依赖 ThemeScope context；无 context 时根来源为 `"root"`。
      - V-6 只把 `theme::` 当回归；“来源分配/投影”改由 theme-gallery.spec（V-9）断言；§2 的测试面改为 Studio 浏览器测试。
    - 置信度：高；阻塞：否。

13. **[P3] F-13 · §10.1“文件不重叠”的并行声明不成立**
    - 定位：[§10.1](DOM-COMPONENTS-P4.md#101-内部任务):688、:701；§1.2:355–359。
    - 问题：T-2 到 T-5 共写多个不在主 agent 独占列表里的文件：`menus.css`、`primitives.css`、生成物 `rustify.css`/`candidates.json`（每次 `mbx xtask css` 都互相覆盖）、`tests/menus_ssg.rs`、两消费者的 `menus.rs`/`menus.html`/`menus.js` 和 `menus.spec.ts`。它们又同在 rustify-components 一个 crate 里，同一工作树中任一任务未完成的代码都会让其他任务编译失败；T-6 改上游 rustify-dom，会触发全部重编。
    - 修订：二选一：
      - 把这些共享文件和生成物列入主 agent 独占（子任务交回片段，由主 agent 集成并统一跑 css），并要求并行任务使用独立 worktree；
      - 或删去 T-4/T-5 的并行声明，只保留 M5 ∥ M6（见 §4 编排）。
    - 复核：:688 的独占列表覆盖 §1.2 中被两个及以上任务写入的路径，:701 写明隔离条件。
    - 置信度：高；阻塞：否。

14. **[P3] F-14 · 里程碑前置有两处不一致，M5 失效的 V-5 证据没有复验去向**
    - 定位：M4 前置:679（“NavigationMenu 开发只需 M1/V-1”）与 T-5:696、§5.5:488；M6 前置:681（“M2—M4 组件”）与 §5.9:530；§9.4:631、:638、:642；M5:680。
    - 问题：
      - NavigationMenu 依赖 M2 的 `primitives::timer`，M4 行漏写。
      - M6 的 parity.rs 只覆盖 P2 表单族，P3/P4 族由既有 specs 证明，所以 M6 实际不依赖 M2—M4；写成 M2—M4 人为串行，失去与 M5 并行的机会。
      - M5 改了 overlay.rs/provider.rs 的 Layer 投影和 Studio app.rs（GPU dropdown 所在文件），使 V-5 中 theme-review、p2-workspace 的证据失效，但 M7 的命令没有安排重跑。
    - 修订：
      - :679 改为“T-1 的 timer.rs 及 V-1”。
      - :681 改为“P2/P3 现有 fixture 与 C-1；共享 consumer/primitives `lib.rs` 由主 agent 集成”。
      - M7 的 Studio 合并命令加入 `theme-review.spec.ts`，并写明 property-workbench `p2-workspace.spec.ts` 在 M7 重跑；或把 T-6 提到 V-5 之前完成。
    - 置信度：高（前置）/中（复验，Layer 只增一个属性）；阻塞：否。

15. **[P3] F-15 · “仓内无 toolbar 产品”不准确，嵌入 Toolbar 的 ToggleGroup 角色未定**
    - 定位：§1.1:336；§5.4:471；§3。
    - 问题：公开的 `toggle_group::Root` / `ToggleGroup` 已有 `toolbar: bool` 参数，开启后输出 `role="toolbar"`。目录、站点、docs fixture 和 hydration 都在用，`SUB_CAPABILITIES` 也有 “toggle-toolbar behavior”。计划没规定嵌入新 Toolbar 的 ToggleGroup 必须输出 `role=group`（否则会出现 toolbar 套 toolbar），也没写旧参数的去向和目录措辞。
    - 证据：[choice.rs](../../crates/rustify-components/src/primitives/choice.rs):849、:1642；[toggle_group.rs](../../crates/rustify-components/src/toggle_group.rs):67–90；[catalog.rs](../../crates/rustify-components/src/catalog.rs):1549。
    - 修订：
      - 改正 §1.1 的事实。
      - §5.4 写明 Toolbar 内的 ToggleGroup 固定 `toolbar=false`（`role=group`）。
      - §3 写明旧 `toolbar` 参数保留为单组便捷用法，并更新目录。
    - 置信度：中高；阻塞：否。

16. **[P3] F-16 · Themes ContextMenu Content 漏了 `align_offset = −4×size`**
    - 定位：§5.6:499；§1.1:342。关联 WORK-F15/AC-4。
    - 问题：参考 Themes 的 ContextMenu Content 与 SubContent 一样带 `alignOffset={-Number(size) * 4}`，计划只给 SubContent 写了这一项，默认定位会与参考差 4 或 8 px。
    - 证据：`ref/themes-main/packages/radix-ui-themes/src/components/context-menu.tsx`:72。
    - 修订：§5.6 的 ContextMenu Content 默认值补上 `align_offset=−4×size`。
    - 置信度：高；阻塞：否。

17. **[P3] F-17 · D5“外部指针现在请求关闭”对 GPU 锚定的便捷 Menu 不完全成立**
    - 定位：D5:257；§3 第 2 项:384；§11:707。
    - 问题：便捷 Menu 现有调用方主要用 Region 锚点（property-workbench、Studio GPU dropdown）。core 的 `inside()` 把 Region 的 canvas 视为内部，所以在 canvas 上点击不会关闭，只有点到 canvas 外的 DOM 才关闭。迁移说明和用例若写成“点外部即关”，在 canvas 上会与实际不符。
    - 证据：[overlay.rs](../../crates/rustify-dom/src/overlay.rs):1452–1475；[property-workbench main.rs](../../examples/property-workbench/src/main.rs):2305–2318；[theme-studio app.rs](../../examples/theme-studio/src/app.rs):553。
    - 修订：D5 与迁移段改为“锚点元素或 canvas 之外的指针/焦点交互请求关闭；canvas 内由应用经 GPU action 决定”。
    - 置信度：中高；阻塞：否。

18. **[P3] F-18 · NavigationMenu 的定位前提、Indicator 位置和窄屏纵向没写清**
    - 定位：ADR-3:292；§5.5:484–485；§6:539。
    - 问题：
      - 只有 List（ul）和 Item（li）都不是定位元素时，Content 才能以 Root 为包含块与 Viewport 对齐，计划没写这个约束。“Indicator 位于 List”若放进 `ul`，违反 ul 只能含 li 的内容模型。Content 在 DOM 上位于 Viewport 之前，需要 z-index 才能盖在外框上。
      - §6 要求 390px 下 NavigationMenu“改为纵向”，但没说由应用切换 `orientation` 还是由 CSS 改布局。只改 CSS 时，`aria-orientation` 和方向键仍按横向，与视觉相反，键盘用户会走错方向。
    - 证据：`ref/primitives-main/packages/react/navigation-menu/src/navigation-menu.tsx`（indicator track 与 viewport 结构）；可复用的 ResizeObservation 在 [runtime.rs](../../crates/rustify-dom/src/runtime.rs):186–199。
    - 修订：
      - §5.5 写明 Root 是唯一定位祖先，List/Item 保持 static；Indicator 渲染在 List 外的轨道元素内，offset 相对 Root；Content 的 z-index 高于 Viewport；这些规则进 `primitives.css`。
      - §6 改为由样本按视口切换 `orientation` Signal（纵向时 Content 在所属 Item 下内联展开），或与 Menubar 一致改为“横向局部滚动、内容不超出视口”。
    - 置信度：中；阻塞：否（对齐细节见 U-04）。

19. **[P3] F-19 · §6 的状态规则没有标明归属，部件写法会丢掉可见禁用原因**
    - 定位：§6:538；§5.1 Item:427；§3 第 2 项:384。
    - 问题：
      - 空菜单空态、导航内容加载中、工具栏字段错误这三项，内容都不归组件所有；写成组件行为，会促使实施者在 SDK 内放文案。
      - 旧 Menu 把禁用原因作为可见文本显示在项旁，并输出 `aria-describedby`。§3 只写了“经 `aria-describedby`”，§5.1 Item 也没有承载原因的方式，存在丢失可见原因的回归风险。
    - 证据：[menu.rs](../../crates/rustify-components/src/menu.rs):268–276。
    - 修订：§6 把这三项标为“样本与文档指引，组件不合成文案”；§3 写明便捷 Menu 保留可见原因文本并输出 `aria-describedby`；docs/menus.md 给出部件写法下的禁用原因示例。
    - 置信度：中；阻塞：否。

20. **[P3] F-20 · 实施合同禁止另起设计，与 AGENTS.md 对新 UI 的设计要求冲突**
    - 定位：@implementation_route:59–63（MUST_NOT “另起方案设计”）；D8:260；§1.2:360（“中性编辑壳与 DESIGN.md 不变”）；§6。
    - 问题：本期有三类没有参考可循、计划也没给版式规格的新界面：
      - Menubar/NavigationMenu/Toolbar 的“Rustify 语义外观”；
      - Studio Components 场景的信息架构（分组目录、样本卡、演示控制放在作者主题还是中性壳）；
      - Inspector 新增的色族/色阶、来源帧、“浮层继承自”和分组诊断。

      AGENTS.md「前端 UI 开发与验收」要求页面、组件、布局和视觉改动用 impeccable 设计。计划文本的禁止条款会让照章实施者没有设计环节，在 P1/P2 已评为 10/10 的 Inspector 面板上直接叠加信息。
    - 证据：AGENTS.md「前端 UI 开发与验收」；DESIGN.md「Dialogs, diagnostics and inspector」；`.impeccable/surfaces/examples-theme-studio-src-app-rs.md`。
    - 修订：在 M3/M4（三组件外观）和 M5（Components 场景与 Inspector）各加一句：按 AGENTS.md 用 impeccable，依据 DESIGN.md 与 surface brief 定版式，作为 @implementation_route 的明确例外，不重设计中性编辑壳。Inspector 信息结构变化后，同步 DESIGN.md 对应小节。不新增验收门。
    - 置信度：中；阻塞：否。

21. **[P3] F-21 · 角色模拟与已有消费者样本是同一个目标，证明不了文档足以独立使用**
    - 定位：§9.3 角色模拟:613；§6:536；V-15:591。
    - 问题：要模拟的“顶部菜单栏 + 站点导航 + 带菜单的工具栏 + 右键菜单”外壳，就是实施者在 `consumer/src/menus.rs` 和 `menus.html` 写好的内容。计划没指定模拟用的 workspace，也没把测试 fixture 排除在“公开示例”之外，模拟角色可以直接照抄。P3 的先例是独立的 `tests/dom-only/public-panel-consumer/`。
    - 修订：§1.2 与 §9.3 指定独立 workspace（新建 public-menu-consumer，或扩展 public-panel-consumer）及其运行入口；写明 `tests/dom-only/consumer/`、`tests/dom-only/primitives/` 是测试 fixture，模拟角色不可读。
    - 置信度：高；阻塞：否。

22. **[P3] F-22 · Components 场景下的 GPU 视图没有说明**
    - 定位：§5.8:516；PRD WORK-F19 正文（“新组件以 DOM 展示；GPU 无对应能力时明确说明”）。
    - 问题：GPU 区域对任何场景都画通用样本，标题直接用原始场景键（“GPU · components”），不随 locale 变化。renderer 为 GPU 或对比视图时，使用者会误以为 GPU 也提供组件陈列。现有场景键是首字母大写的显示名，新键 `components` 风格不一致。
    - 证据：[preview_region.rs](../../examples/theme-studio/src/preview_region.rs):715；[app.rs](../../examples/theme-studio/src/app.rs):456；[scenes.rs](../../examples/theme-studio/src/scenes.rs):12–20。
    - 修订：Components 场景下 GPU 区域显示本地化说明“组件陈列仅 DOM；GPU 显示通用主题样本”；场景键与现有命名风格统一；theme-gallery.spec 增一句断言。
    - 置信度：中；阻塞：否。

23. **[P3] F-23 · 收口回归会覆盖已提交的前期证据截图**
    - 定位：V-8:584、V-9:585、V-13:589；提交协议:177。
    - 问题：panels、content、feedback、theme-forms、theme-ui-review 等 spec 运行时会把截图写进已入库的 `docs/validation/dom-components-p{1,2,3}/`（仅 p3 就有 59 个受版本控制的文件）。M7 重跑后这些文件都会变成已修改，计划没说恢复还是保留，容易误提交，或让工作树长期带着被覆盖的历史证据。
    - 证据：[panels.spec.ts](../../tests/dom-only/panels.spec.ts):49、:243、:304；`git ls-files docs/validation/dom-components-p3 | wc -l` = 59。
    - 修订：§9.4 或 §12 加一句：收口回归后执行 `git checkout -- docs/validation/dom-components-p1 docs/validation/dom-components-p2 docs/validation/dom-components-p3` 还原，P4 证据只写 `dom-components-p4/`。
    - 置信度：高；阻塞：否。

## 2. 待核实与待决策

都不需要用户拍板产品范围，也不需要外部服务。前五项是技术前提，确认后可能影响对应确认问题的修法细节；最晚确认点之前由实施者用小实验或首个浏览器用例回答。

| 项目 | 缺少的证据及影响 | 提供角色 / 最晚确认点 |
| --- | --- | --- |
| U-01 CSSOM 遍历 Tailwind v4 嵌套规则 | Chromium 能否稳定遍历嵌套 `CSSStyleRule.cssRules` 并从完整 `cssText` 取出 var。成功判据：对 Button 与菜单项，hover/非 hover、light/dark 下得到同一组 token。影响 F-06 的语义选择 | 实施者；M5 开工前 |
| U-02 模态 ContextMenu 打开时再次右键 | contextmenu 的实际目标、是否弹出原生菜单。影响 F-04 补监听的必要性和写法 | 实施者；M2 首个 ContextMenu 浏览器用例 |
| U-03 Menubar 切换时的焦点序列 | 旧菜单回焦与新菜单打开的先后、拒绝切换时的请求计数。影响 F-05 | 实施者；M3 |
| U-04 NavigationMenu 内联 Content 与 Viewport 对齐 | Root 包含块下（含 RTL）左上与尺寸是否一致、是否被祖先 overflow 裁切。影响 ADR-3 与 F-18 | 实施者；M4 开工 |
| U-05 子菜单 fixed 定位被父 panel 约束 | 子 Layer 是父 Content 的 DOM 后代（`position: fixed`）。若 menus.css 在 Content 上引入 transform/filter/contain（如开合动画）或 `overflow:hidden`，子菜单会被裁切 | 实施者；M2 的 menus.css 与嵌套子菜单截图 |
| U-06 便捷 Menu 的静态结构 | 改由 floating root 组装后，关闭时会常驻一层 `menu-root` div；docs fixture、site demo 和 hydration 的静态结构或布局断言是否受影响 | 实施者；V-12 |
| U-07 NavigationMenu 链接一律整页导航 | 所有链接带 `data-rustify-native-navigation`，同源普通点击也绕过 scope router 并整页加载。这与 D7、PRD“不伪装成应用命令菜单”和 TabNav 先例一致，但 CSR 应用的使用者需要知道。最低限度在 docs/menus.md 写明；是否另给路由内导航入口，由计划作者在 M4 前决定，不阻塞 | 计划作者；M4 |
| U-08 既有 theme-acceptance V6 两处断言疑似过期 | 计划 §11 已登记；本轮静态核对与计划描述一致，未运行确认 | 已登记；M7 记录 |
| U-09 dom-primitives 下 panels/content/feedback 的 `test.skip` | 是否让某些 P3 族缺拒绝路径。计划 §5.9 已安排在 M7 覆盖表补行 | 实施者；M7 |

## 3. 覆盖摘要

- **基线**：计划 v0.1 / SHA-256 `319776af…1471`；代码 `0d96270f`，dirty 仅台账、报告与计划。四位 reviewer 获相同基线和原始材料。交付前复核，HEAD 与计划 hash 均未变。
- **分工**：主 agent 实际启动 4 个只读 subagent（facts、architecture、schedule、delivery），一个视角一位，模型继承。主 agent 同期整理 PRD P4 需求账本，收齐结果后逐条回到计划段落与源码裁定，并亲自复核了关键证据：floating 的 click 切换与 aria-controls（floating.rs:221–258）、Portal 激活门控（:361–362）、scroll_area 停靠点（scroll_area.rs:216）、Tailwind 嵌套产物（rustify.css:598–606）、ThemeBoundary 的 wasm 门控（provider.rs:100–102、:184–186）、拾取捕获（inspector.rs:316–376）、旧 Menu 可见原因（menu.rs:268–276）、spec 标题（dom-only.spec.ts、theme-acceptance.spec.ts）、ContextMenu alignOffset（ref context-menu.tsx:72）、dom-layout 的 Link 用例与项目映射。
- **四维覆盖**：facts 核对 §1.1 全部 [已核实] 主张、§5 接口符号、§9 project/spec/测试名、§11 既有断言，表中 30 余项成立，缺口已归入 F-01、F-12、F-15、F-16 等。architecture 审 ADR-1–ADR-4、主题来源、Toolbar 接管、Menubar、性能基线与 parity 规模。schedule 做 AC 双向追踪、依赖边、并行与提交合同。delivery 走开发者、最终使用者、Studio 使用者三条主路径并审 UI/UX。四维都没有跳过。
- **脚本**：
  - `check_paths.sh`：43 个路径 token，16 个 MISSING，均为 ★新增，无复用路径缺失。
  - `check_progress.sh`：0/7，ERROR 0，WARN 1（工作树有本计划外的未提交台账改动）。
  - `check_plan.py`：ERROR 0，WARN 18，均为自定义 WORK-* ID 需人工核对，已按 PRD §7.2 人工核对。

  脚本通过不证明行为或验收。
- **浏览器与 UI 证据等级**：本期新 UI 未实现，按“仅计划/流程描述”做文档推演，待实现后浏览器验收。现有 Studio 只读源码，未在浏览器验证。没有运行 mbx、build-web 或 Playwright。
- **外部规范**：APG 结论依据已知规范，未联网复核；参考实现结论来自本地 `ref/` 快照（Themes 3.3.0、Primitives 1.8.1）。

## 4. 映射与实施编排

**需求映射。** 本期 10 条 AC 都有设计、任务和检查落点：WORK-F13/AC-1–AC-3 → §5.1–§5.5、T-1–T-5、V-2–V-5；WORK-F15/AC-4 → §5.6、V-2–V-5、V-10；WORK-F1/AC-3 → §5.9、T-8、V-8；WORK-F19/AC-2 → §5.8、T-6/T-7、V-6/V-9；WORK-N1/AC-2 → V-9/V-10；WORK-N3/AC-3、WORK-N6/AC-2 → V-11；WORK-N7/AC-2 → V-10。

四条整条终验涉及的前期 AC 已逐条到 spec 核对，覆盖成立：

| 前期 AC | 证据 |
| --- | --- |
| WORK-F1/AC-1 | colors.spec |
| WORK-F1/AC-2 | docs-verify features 断言无皮肤 CSS + primitives.spec |
| WORK-F15/AC-2 | theme-forms、catalog-inputs |
| WORK-F15/AC-3 | panels、content、feedback |
| WORK-N3/AC-1 | theme-compat 的 CSP 违规监听、theme-dom 宿主不改写 |
| WORK-N3/AC-2 | theme-exchange 的拒绝与导入校验 |
| WORK-N6/AC-1 | theme-exchange、theme-storage、theme-v2-storage |

断链只有两处，修订见 F-10：WORK-N3/AC-3 的 DOM 两实例证据被 `-g` 滤掉；WORK-F15/AC-1 的 Link 证据缺失。反向核对：没有无来源的验收门；M1 菜单基线有 AGENTS.md「性能」与 PRD Q4 依据，parity 覆盖 P2 全族是 WORK-F1/AC-3“全部已覆盖的交互组件”的要求，都不算过度。

**依赖边**（修订后）：

| 前置 → 后置 | 产物 / 解锁证据 |
| --- | --- |
| C-1 → M1 | P3 7/7；旧 Menu 未改 |
| M1/V-1 → T-1 | before JSON 与 M1 提交；vendor 与 sources.lock |
| M1 → T-2 样本 | consumer 的 `menus.html/js` |
| T-1 → T-2 | floating `Kind::Menu`（按 F-01）、`primitives::menu` 部件签名、model 测试 |
| T-1+T-2（V-2）→ T-3 | 共享菜单部件与 base-menu 皮肤 |
| T-1 → T-4 | choice.rs 的 `navigation`、toggle group 参数；嵌入菜单场景还需 T-2 |
| T-1 → T-5 | `timer.rs`（F-14） |
| V-2–V-4 + 三个示例产物 → V-5 | 阶段一收口（按 F-09 含 panels.spec） |
| T-6 → T-7 | `use_theme_source` 与来源属性 |
| M2–M4 → T-7 | 菜单族陈列样本 |
| P2/P3 fixture → T-8 | 原文写 M2–M4，过粗（F-14） |
| M5 → M7 复验 | theme-review、p2-workspace 重跑（F-14） |
| M7 内部 | 文档 → V-15；docs-verify features → dom-hydrate；最终产物 → V-14 current |

依赖图无环、无倒置。瓶颈链是 T-1 → T-2 → V-5 → M7。

**并行波次与集成门：**

| 波次/任务 | 前置及解锁证据 | 可同时做的工作 | 写入边界/共享负责人 | 运行资源约束 | 汇合验收 |
| --- | --- | --- | --- | --- | --- |
| W0 M1 | C-1 | 无 | 主 agent：benchmark、menus 页、vendor、lock、提交 | dom-only 4190，evidence 层 | V-1，M1 提交 |
| W1 T-1 | V-1 | 无（T-6 仅在独立 worktree 时可并行） | floating.rs、primitives/menu、timer.rs、choice.rs 由单人负责 | 单 crate 编译串行 | V-2 中 model 的先红后绿 |
| W2 T-2 → T-3/T-4，以及 T-5 | T-1 签名（T-5 需 timer） | 同一工作树内顺序做；并行须 worktree，且按 F-13 由主 agent 负责共享 CSS、SSG、样本与 spec | 主 agent：lib.rs、mod.rs、styles、`mbx xtask css` | 编译缓存串行 | V-2、V-3、V-4 |
| W3 V-5 | V-2–V-4 + 三个 build-web | 无 | 主 agent | 浏览器串行：dom-only → dom-primitives → catalog → workbench → studio | V-5，M4 提交 |
| W4 M5 ∥ M6 | M5 需 M2–M4 组件；M6 只需 P2/P3 fixture | 写入面分离：M5 写 theme-studio 与 rustify-dom，M6 写 parity 系列 | 主 agent：consumer/primitives `lib.rs`、playwright 配置、CI | rustify-dom 改动触发下游重编，编译串行或用 worktree；两者都不跑浏览器 | V-6、V-7 |
| W5 M7 | V-1–V-7 | 无 | 主 agent：文档、catalog、生成物、CI、证据目录还原（F-23） | 每命令一个项目、串行，按 F-10 每个 dom 项目一次 | V-8–V-15 |

其余编排成立，不需调整：中间里程碑只跑局部检查，M4 是旧 Menu 重组的风险门并做阶段一浏览器收口，M7 集中终验；提交点 M1/M4/M7 与“待提交”语义符合骨架。§12 末条“按 SDLC v3 同步最小事件”与 @project_index 不矛盾（仓内协议为 v3，后者已规定旧协议强制事件时只留最小内容）。

## 5. 总体结论与修订顺序

**是否需要修改：需要。** 有 11 项 P2 确认问题；计划自报 Ready，与本轮“修订前不宜按原文开工 M2”不一致。

- **建议状态：修订后 Ready。** 所有问题都有明确的最小修订，修法由计划作者在正文内选定即可，不需要用户或外部给出答案，所以不判 Blocked。修订前，M1（基线、vendor、lock）可以按原文执行，它不受任何确认问题影响。
- **与原状态的差异**：原文自报 Ready，但 F-01–F-05 会让 M2–M4 照章实现出错（Trigger 先开后关、便捷 Menu 打不开、根钩子取消失效、模态菜单 Tab 落到滚动框、Menubar 切换重复请求），F-06–F-08 会让 M5 的 Inspector 无法交付 WORK-F19/AC-2 所需的可检查性，F-09–F-11 会让验收漏证或过量。
- **修订顺序**：
  1. 先纠正前提与契约：F-01（floating 改动面与 ADR-1 措辞）、F-12（来源 ID 的 wasm 前提）、F-15（现有 toolbar 参数事实）、F-17（D5 canvas 语义）。
  2. 再补设计：M2 的 F-02、F-03、F-04、F-16、F-19；M3 的 F-05；M4 的 F-18；M5 的 F-06、F-07、F-08、F-20、F-22。
  3. 最后修映射与编排：F-09、F-10、F-11、F-13、F-14、F-21、F-23。
- **解除条件**：改稿后，§5.1 对 floating 的描述与代码事实一致；§5.1–§5.3、§5.6、§5.8 写明上述契约；V-5、V-8–V-13 的命令可直接运行且能选中预期用例；§9.3 有明确的矩阵组合规则；§10.1 的并行条件写明隔离方式。满足这些条件即可判 Ready，开工 M2。Ready 只表示可以按计划实施，不表示已实现、已验收或已发布。

下一步：

```text
/apply-doc-review docs/plan/DOM-COMPONENTS-P4.review.md
```

## 修订处置与复核

- **来源**：本报告（所审计划 v0.1，SHA-256 `319776afdf961066ccdbff72d1be9ad58fd34da6d8fb4afebed5f32d8e451471`）；F-01—F-23 的原问题、证据与原结论保留在上方，不改写。
- **目标**：[DOM-COMPONENTS-P4.md](DOM-COMPONENTS-P4.md)，修前 v0.1 / `319776af…1471`，修后 v0.2 / `e840cee484d22c17b48311d30c656140b3d399b7f7e289db36d35027f381cd32`；路径未移动。
- **复核日期与代码基线**：2026-11-04；`0d96270fc5eae9cc9a71774649f3254499bf3f20`；dirty 仅 `.xgent-ai/sdlc/*`、`skills-lock.json` 与未跟踪的 P3/P4 计划、评审报告，产品源码与测试无改动。
- **本次范围**：报告全部 23 项。未改代码，未重跑独立评审，未开始里程碑。

| 来源问题 | 核实结论与依据 | 最终处置 | 修改位置与关联同步 | 复核结果/后续 |
| --- | --- | --- | --- | --- |
| F-01 | 成立。`floating.rs:244-252` 的 `button_as` 无条件 click 切换、固定 `aria-haspopup="dialog"`、无条件 `aria-controls`；`issue()`/Portal `active` 只按 anchors/triggers 判定，未计虚拟锚点。 | 已修订并复核 | ADR-1、§1.1、§1.2、§5.1 第 1—4 条、§5.2、§12：菜单 Trigger 不走 `button_as`；虚拟锚点进门控；Sub 固定 `modal=false`。 | 迁移路径与 Trigger 所有权一致；无“先开后关”残留在菜单路径。 |
| F-02 | 成立。`overlay.rs:1177-1199` 只取最上层钩子；ADR-2 的 document 监听绕过根 Content 的公开 `Cancelable`。 | 已修订并复核 | ADR-2 备选 B②、§5.1 整树关闭、§8、V-2：先运行根 `on_pointer_down_outside`/`on_interact_outside`，未取消才关。 | 根钩子取消保持打开进入 V-2 模型；子层同击关闭仍不重复。 |
| F-03 | 成立。`scroll_area.rs:216` viewport 为 `role=group tabindex=0`；core 的 `keep_tab_inside` 在 scope capture 先于 Content 运行（`overlay.rs:1114-1135、1626-1663`）。 | 已修订并复核 | §1.2 Themes 行、§5.6、§12：菜单内容改用无停靠点的 `overflow:auto` 容器；Tab 仍由 Content 消费。 | 模态菜单的 Tab 包裹仍由 Content 处理，滚动框不再成为唯一停靠点。 |
| F-04 | 成立。模态菜单 `claim_inert`（`overlay.rs:1063`）使 Trigger inert；非模态 `inside()` 把 `Anchor::Region` 的 canvas 视为内部（`overlay.rs:1452-1475`）。 | 已修订并复核 | §5.2：打开期间按需持 document capture `contextmenu`；非模态 Trigger 区内主键 pointerdown 请求关闭；已开只同步锚点。 | 再次右键、非模态 Trigger 区关闭与原生菜单抑制均有明确路径。 |
| F-05 | 成立。参考 `menubar.tsx:324-337、351-361` 的 `focusOutside` 取消与 `onCloseAutoFocus` 取消未写入。 | 已修订并复核 | §5.3：补 Trigger 目标取消、value 仍为 Some 时取消回焦、子 panel 只忽略后退键。 | 顶层切换不重复请求；键盘打开的延迟回焦得到抑制。 |
| F-06 | 成立。`rustify.css:590-615` 是 Tailwind 嵌套产物（`&:hover`、`@media`、`@container`，39 处 `&`）；ADR-4 未定义匹配语义，缓存键却含状态维度。 | 已修订并复核 | ADR-4、§5.8 第 2 条、§7、§12：顶层规则去伪元素、用户动作伪类与运行态属性后 `matches`，命中后从完整 `cssText` 收集；缓存与状态无关。 | 语义改为“该部件在任何状态下引用的 token”，不是当前生效值；报告建议的伪类剥离已扩展到运行态属性以自洽。 |
| F-07 | 成立。`inspector.rs:197-203、316-350、356-376` 的 capture 会消费 pointerdown/click；Escape 未阻断；Studio 自身 Layer 在编辑壳边界内。 | 已修订并复核 | D9、§5.8 第 1 条、§6、§9.3：只接受根/已登记预览边界来源帧；放行打开浮层的激活；Esc 先退拾取；更新提示文案。 | 采纳“拾取中放行浮层激活”路径，不另造常展开演示；Studio 弹层不被拾取吞掉。 |
| F-08 | 成立。`storage_controls.rs:339-474` 与 `controller.rs:1289-1300` 表明 TokenSource 只写根主题 overrides，边界内值只读。 | 已修订并复核 | §4、§5.8 第 4 条、§1.2：TokenSource 保持根语义；Inspector 显示边界标签、局部只读与根值并列；未登记边界显示固定文案。 | 编辑对象与来源显示不再错位。 |
| F-09 | 成立。ADR-1 引用 V-5 的 panels.spec，但 V-5 原命令只有 menus.spec；floating 改动直到 V-8 才碰 panels。 | 已修订并复核 | ADR-1、V-2、V-5、§8、§11：V-5 两个 dom 项目均加 panels.spec；整树关闭含根钩子取消。 | P3 浮层回归回到 M4 阶段出口，V-8 只作最终引用。 |
| F-10 | 成立。V-8/V-11—V-13 的 project 只在 docs 配置；`-g lifecycle` 无匹配标题；`-g "GPU|font|fatal"` 会多选；V-13 用 dom-buttons 覆盖 Link。 | 已修订并复核（偏离一条建议） | V-5、V-8、V-9、V-10、V-11、V-12、V-13、§9.3、§9.4：补 `-c playwright.docs.config.ts`、完整文件、`dom-only.spec.ts`、`-g "isolated failures"`、`dom-layout.spec.ts`、colors 项目。 | 建议把 dom-only/dom-primitives 合成一条命令；按 AGENTS.md“每命令一项目”，改为两条顺序命令、同一构建产物与基线，目标（同一代码取证据）不变。 |
| F-11 | 成立。§9.3 未界定 variant/size/state 组合；把 P5 的 390/768 混入 WORK-N7/AC-2；签名漏 focus 的 outline 与 opacity。 | 已修订并复核 | §9.3、V-10：桌面默认色板逐组件、声明变体×适用状态不全组合、局部色板/high_contrast/caller 覆盖限已落地组件、390/768 只验布局；签名加 outline 与 opacity。 | 回归层取代表性子集，完整强度只在 evidence；P5 维度不再提前认领。 |
| F-12 | 成立。`provider.rs:100、145-200` 的 ThemeScope/ThemeBoundary 仅 wasm；host 测试与 hydrate 一致性断言不成立；计数器依赖渲染顺序。 | 已修订并复核 | §2、§5.7、§5.8 core、V-6、§12：Boundary 在浏览器端自取唯一 ID，root 为 `"root"`，SSG 不输出来源属性；来源分配/投影由 V-9 断言。 | host 只保留 `theme::` 回归；hydrate 不再要求来源一致。 |
| F-13 | 成立。T-2—T-5 共用 menus.css、primitives.css、生成物、menus_ssg、样本与 spec，主 agent 独占清单未列这些文件。 | 已修订并复核 | §10.1：删去 T-4/T-5/T-6 的并行声明；只保留 M5 ∥ M6，并写明独立 worktree/串行编译与写入边界。 | 不再以“文件不重叠”作为并行依据。 |
| F-14 | 成立。M4 前置只写 M1/V-1，但 T-5 需 T-1 的 timer；M6 前置写 M2—M4，但 parity 只覆盖 P2；M5 改动 app.rs/overlay/provider 后 V-5 的 theme-review/p2-workspace 证据失效。 | 已修订并复核 | M4、M6、M7、T-8、§9.4：timer 前置；P2/P3 fixture + C-1；M7 重跑 theme-review 与 p2-workspace 并加入 Studio 合并命令。 | 证据失效有明确复验去向。 |
| F-15 | 成立。`choice.rs:849、1642` 与 `catalog.rs:960、1012` 表明公开 ToggleGroup 已有 `toolbar` 参数并输出 `role=toolbar`；目录“无 toolbar 产品”不准确。 | 已修订并复核 | §1.1、§3 第 7 条、§5.4、§12：Toolbar 内固定 `toolbar=false`；旧参数保留为单组便捷用法；目录改记独立 Toolbar 与单组便捷用法。 | toolbar 套 toolbar 的 ARIA 冲突消除。 |
| F-16 | 成立。参考 `context-menu.tsx:72` 的 Content 设 `alignOffset={-Number(size) * 4}`，计划只给 SubContent。 | 已修订并复核 | §1.1、§5.6：ContextMenu Content 默认补 `align_offset=−4×size`。 | 与参考源的定位契约一致。 |
| F-17 | 成立。`overlay.rs:1452-1475` 把 Region 的 canvas 当内部；便捷 Menu 的 canvas 内点击不会触发外部关闭。 | 已修订并复核 | D5、§3 第 2 条、§5.2 便捷 Menu、§11：外部关闭限定为锚点元素或 canvas 之外；canvas 内由应用经 GPU action 决定。 | 迁移说明与 GPU 调用方语义一致。 |
| F-18 | 成立。参考 `navigation-menu.tsx:462` 用 List 外轨道包 ul；计划把 Indicator 放进 List、Content 与 Viewport 的定位前提未写；窄屏“改纵向”与 orientation 语义冲突。 | 已修订并复核 | ADR-3、§5.5、§6：Root 唯一定位祖先、List/Item static、Indicator 在 List 外轨道、Content z-index 高于 Viewport；窄屏改横向局部滚动。 | 不再切换 orientation，避免 aria 与视觉相反。 |
| F-19 | 成立。部件写法只有 children，`Item` 无承载原因的 ARIA 通道；§6 状态规则未标归属。 | 已修订并复核 | §3 第 2 条、§5.1 `described_by` 与角色说明、§6：便捷 Menu 保留可见原因与 `aria-describedby`；部件经 `described_by` 引用调用方原因；状态文案归样本/文档。 | `docs/menus.md` 的示例要求已写入。 |
| F-20 | 成立。`@implementation_route` 禁止另起方案设计，与 AGENTS.md 对新 UI 的 impeccable 要求冲突。 | 已修订并复核 | M3、M4、M5：按 AGENTS.md 用 impeccable、依据 DESIGN.md 与 surface brief，标为 `@implementation_route` 的明确例外；M5 同步 DESIGN.md。 | 不重设计中性编辑壳，不新增验收门。 |
| F-21 | 成立。`tests/dom-only/consumer` 与 `primitives` 是测试 fixture；角色模拟不能与样本同目标。 | 已修订并复核 | §1.2、§9.3、§9.4：独立公开消费者 `public-panel-consumer/`；构建命令与 `public-panel` project 入口；fixture 不得作为答案来源。 | 文档独立性有可执行路径。 |
| F-22 | 成立。`preview_region.rs:715` 用 `format!("GPU · {}", props.scene)`，`scenes.rs` 与 `app.rs:371-379` 的场景键风格不一致。 | 已修订并复核 | §5.8 场景名、§9.3、V-9：键统一为 `Components`；GPU 区域本地化说明“组件陈列仅 DOM”；spec 断言入 V-9。 | 不再把场景键直接拼进标题。 |
| F-23 | 成立。`panels.spec.ts:49、243、304、400-404、454、592-598、642` 写入已入库的 P3 截图；`git ls-files` 为 59 个。 | 已修订并复核 | §9.4、§12：收口后 `git checkout -- docs/validation/dom-components-p1/p2/p3` 还原；P4 证据只写 `dom-components-p4/`，以 git diff 核实。 | 历史证据不被覆盖。 |

### 决策与未完项

- **F-10 的偏离**：报告建议把 dom-only/dom-primitives 合成一条命令；AGENTS.md 明确“每命令一项目”，故保留两条顺序命令，使用同一构建产物与代码基线。目标（同一代码取证据）不变，计划中已写明顺序与不并发。
- **F-06 的语义补全**：报告举例剥离伪元素与用户动作伪类；为兑现“与状态无关”，计划同时剥离 `[data-state]`、`[data-highlighted]`、`[data-disabled]` 与状态位 `[aria-*]`。这是同一问题的自洽补全，不扩大检查范围。
- **F-07/F-13/F-18**：分别采纳“拾取中放行浮层激活”“删除 T-4/T-5/T-6 并行声明”“窄屏横向局部滚动”，均报告已列出的备选路径。
- 无待用户拍板项；无代码、SPMS、issue 或外部评论写入。

### 修订后结论与验证

- 23/23 项已修订并复核（文档层面）；0 项未处理、0 项不采纳、0 项待核实。复核只证明修改消除了报告所述文档矛盾，不等于实现或验收通过。
- `check_paths.sh docs/plan/DOM-COMPONENTS-P4.md .`：检查 46 个路径样式 token，MISSING 16 个，均为本计划新增的 ★ 路径（`menus.css`、`primitives/menu.rs`、`menus_ssg.rs`、vendor 目录、`docs/menus.md`、`dom-components-p4/`、`gallery.rs`、`theme-gallery.spec.ts`、`menus*.spec.ts`、consumer 样本等），无复用路径缺失。
- `check_progress.sh docs/plan/DOM-COMPONENTS-P4.md .`：0 ERROR、1 WARN（7 个里程碑未完成、工作树有未提交台账；计划文件未被 Git 跟踪，脚本因此提示“没被改过”），与 0/7 未开始一致。
- 未运行代码测试、构建或浏览器验收；本轮只改文档，不把文档检查当作实现证据。
- 关联 PRD 未被指定为修订对象，未修改；本次新增的 `docs/menus.md`、DESIGN.md 同步、`public-panel-consumer` 样本与 `docs/validation/dom-components-p4/` 属后续实施产物。
- 修后计划仍为 **Ready**（0/7，M1 可开工）；本处置节是修订复核，不是新一轮独立评审通过。

