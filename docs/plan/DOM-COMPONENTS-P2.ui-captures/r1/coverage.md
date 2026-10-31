# R1 覆盖与缺证

初采112张主证据，U1/U2/U3补采11张，共123张独立评审输入；另3张主 agent 机器检查实拍。原始PNG共145张，包括19张首采取景不完整但未删除的尝试。数量不代表全部矩阵通过。每张初采出处见 capture.md，补采逐项页面读数见 supplement-U1/U2/U3.md；机器3图见 machine-checks.md。

| 单元 | 已有真实证据 | 尚缺的必需格子/证明 | 原因与补证方法 |
| --- | --- | --- | --- |
| U1 完整表单 | 三视口×浅深默认；constraint/rule/application就近错误、首错焦点、本地成功/FormData、重试、reset可见结果；分组Select键盘打开/字符定位/选择回焦；数量Draft - 保持FormData12，完成17.5后保存；390深色中文上半段 | busy及重复提交次数；check错误；完整Tab→组/卡片→Select→提交；长错误/长FormData窄屏；深/平板/手机展开数据及中文下半段；Draft仍聚焦提交瞬间、Color成对结果；reset草稿/FormData及拒绝/异步边界 | busy读数先于截图，实际图片已成功，不能冒充；本次没有到达check与长错误；补采只覆盖部分按键。下轮用已有本地演示入口逐步到达并在状态持续时拍摄；到达不了的check/拒绝reset须明确缺入口或提供真实fixture，不注入替代UI。 |
| U2 选择工具 | 各公开尺寸/变体配对，Toggle/ToggleGroup默认，disabled/readonly/invalid、拒绝/延迟/接受；dark局部色/HC、390卡片；Checkbox三态、禁用点击无请求、只读Radio箭头移动焦点且不请求 | 普通Radio/CheckboxGroup和ToggleGroup操作及卡片表单一致性；单Tab stop、Home/End、跳禁用、CheckboxGroup逐项Tab；延迟接受后完整焦点路径；局部能力单变量对照 | 补采只覆盖三个代表动作，不能扩大成完整组契约。下轮从真实工具/完整表单操作同批数据，采集请求、接受值、FormData和焦点。已有说明关联按用户新口径复核，不新增说明。 |
| U3 字段/插槽 | 四类字段尺寸/变体配对、disabled/readonly/invalid/focus；Number/Color未完成Draft及Escape恢复；consumer RTL、390、utility；TextField单次Meta+Z与0–2选区 | Enter/blur一次commit、失焦恢复、连续应用更新光标；TextArea撤销/选区和实际resize；长文本/长错误；标签点击及关联；Slot默认/显式色、物理pl/pr和responsive完整优先级 | 初采/补采未展示这些成对动作；下轮在现有入口操作并记录次数和聚焦元素。未完成Draft的FormData只有U1数量一路，不扩展全部字段。 |
| U4 滑块 | 单/多值尺寸/变体、跨越端点焦点、拒绝/commit记录、取消计数、raw竖向/反向、dark/RTL | thumb名字/值实际语义；preserve_order/间隔、min/max/step边界、完整键盘；完成事件最后候选payload；disabled/readonly及局部radius/scaling | 现有图只证明展示动作，commit标记没有payload。下轮用可观察事件输出记录最后候选与接受值，补边界按键和本机无障碍树。 |
| U5 Select | 五类Trigger变体与尺寸配对、两类Content、打开/字符定位/选择回焦、空/长列表End、390深色RTL；U1另有分组展开证据 | 不同值候选被取消后原值/焦点保持；Escape和外点关闭；禁用项实际不可选；移除已选项后placeholder；局部滚动读数；中文浮层继承 | 取消帧尝试选择的值已是当前值，不能区分正常同值选择与取消。下轮先固定A，再取消B；采集外部焦点和列表/页面scrollTop。长列表叠字需先修复才能可靠观察活动项。 |
| U6 raw/兼容 | raw1440/390首段、取消Trigger无默认请求、scope内容、拒绝后接受值/FormData、原生阻塞/保存/失败/重试记录；catalog slider/number分类；Workbench邮箱错误及更正；compare明确DOM表单 | raw完整键盘与390下半段字段/Select/保存反馈；Portal方向前后和关闭清理；自定义元素名字/原生行为；catalog实际编辑结果；Workbench更正后成功保存 | 当前静态/部分操作不能证明最终完成任务。下轮沿真实入口走通并拍结果，不把选择分类或清除错误当成功保存。 |

共性尚缺：Edit/Preview往返及两区独立滚动的成对记录；主题文档/历史在宽度和模式切换后保持；完整标签/帮助/错误关联及辅助技术行为。机器检查补充的局部读数不替代这些主路径。

这些缺证不等于产品缺陷，也不因历史单测/工程退出已通过而自动关闭。它们使本轮无法判通过。无需新账号或外部业务数据；现有本地演示数据足够补大部分状态，其余需补真实可达入口/fixture。

OS IME、VoiceOver、真机触控、浏览器UI原生200%缩放本轮未执行，按原计划留P5 missing，不作为本期新增门槛。
