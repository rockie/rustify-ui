# R2 评审结论 · U5 摘录

摘自 R2 合并结论 judge.md，只含 U5 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U5 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.9 | 2.85 | 2.6 | 0.75 | **9.1** | b | a 9.35 |


### 各维度理由

**U5（b）**

- 布局 2.9：预览 scope 连续并有 24px 内边距，菜单对齐触发器，390 对话框两侧各留 16px，Host control 回到文档流，DOM 与 GPU 起点一致。扣分原因：开启拾取时信息面板挤进文档流，把预览下推 175px；拾取后编辑区滚动落点把来源块顶部压在粘性页头下，切掉半行字。
- 视觉 2.85：编辑壳中性，作者色只在 scope 及其浮层内，遮罩约 30%，深色下 Caller class override 对比已修复，选中态三色符合 G3。扣分原因：未选分段与 Inspect 带 #8AB7F7 蓝色内描边；未标注态用中性虚线框而标签仍用选中蓝。
- 交互 2.6：菜单、对话框、局部边界 Opposite to root、浮层随局部主题、对比区 Dark、未标注反馈、拾取拦截点击、DOM + GPU 都走通。扣分原因：拾取信息面板随预览滚走，拾取较低部件时 token 按钮与来源摘要不在视野；编辑区来源标题被页头遮住；拾取标签截掉 token 列表，较高部件的标签落在视口外。
- 细节 0.75：标签被截成「primary, primary-f…」；未标注面板多约 8px 空行；分段之间 3px 双线；拾取后编辑区第一行被切半。


## R2 未关闭差距（U5）

#### UI-02 · [P2] 未选分段带 #8AB7F7 蓝内框、段间 3px 双线，焦点环压到相邻段（U1/U5 · 视觉、交互、细节 · 部分解决 · 可挽回 U1 a 0.2 / b 0.15，U5 a 0.15 / b 0.15）

- 位置：U1-default-1440-light 的 Light/Dark、DOM/GPU/DOM + GPU、Inspect；U1-keyboard-04 Dark 焦点；U5-dom-1440-light、U5-compare-nested、U5-gpu-compare 工具栏。
- 现状 → 期望：未选段在 #D0D5DD 外框内多一圈 1px #8AB7F7，EN/中文、Edit/Preview 没有；选中段另加 2px 下划线 → G3 蓝色只用于选中与焦点；G4 选中的模式/渲染器按钮用按下态。
- 修法：所有分段控件共用一个组件：未选段白底 #475467，去掉 #8AB7F7；段间单条 1px #D0D5DD；焦点环内收或 `z-index:1`，不压邻段。
- 来源：U1 a UI-02、b UI-02；U5 a UI-02（P2）、b UI-02（P3）。严重度取 P2。

#### UI-110 · [P3] 「1 text pairs below 4.5:1」单复数错误（U1/U5 · 细节 · 可挽回 U1 b 0.1，U5 a 0.05）

- 位置：U1-default-1440/768、U1-field-tabs、U1-keyboard-01 至 03、U1-zoom200；U5-dom-1440-light、U5-inspector*、U5-gpu-compare、U5-preview-390 诊断条标题。
- 修法：按数量选择单复数（`Intl.PluralRules`）。
- 来源：U1 b 新-8；U5 a 新-2。U4 a 在建议中指出同一处。

#### UI-147 · [P2] 拾取信息面板在文档流里：开启时预览下推 175px，拾取较低部件时面板不在视野（U5 · 交互、布局 · 可挽回 b 0.3）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light「DOM」标签 y=244→419；U5-inspector-undeclared-hover/-undeclared 看不到面板；U5-inspector-undeclared-panel-1440-light 需滚回顶部才看到「Not annotated · …」。
- 现状 → 期望：P2 信息面板带 token 按钮，从实际部件定位角色与来源。
- 修法：面板改为预览滚动容器内的 sticky 层或底部 overlay，不占文档流；或把 token 按钮做成锚在拾取标签旁的小浮层。
- 来源：b 新-1（a 在建议 5 提出下推 175px，未计分）。

#### UI-148 · [P2] 拾取矩形在蓝色部件上看不出（U5 · 视觉 · 可挽回 a 0.1）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light「Create profile」：2px #1570EF 轮廓贴着同色按钮。
- 修法：保持固定蓝色，加 `outline-offset: 2px` 与内侧 2px 白色光环（P2）。
- 来源：a 新-4（b 在建议 8 提出同一点，未计分）。

#### UI-70 · [P3] 拾取后编辑区滚动落点被粘性页头遮住，来源标题不在视野、第一行切半（U5 · 交互、布局、细节 · 部分解决 · 可挽回 a 0.15 / b 0.2）

- 位置：U5-inspector-picked-1440-light 左栏 y≈300–330。
- 修法：滚动目标改为来源块标题；滚动容器 `scroll-padding-top` = 页头高度 + 16px。
- 来源：a UI-70；b UI-70。

#### UI-149 · [P3] 拾取标签截掉 token 列表；较高部件的标签落在视口外（U5 · 细节、交互 · 可挽回 a 0.05 / b 0.15）

- 位置：U5-inspector-1440-light / -picked「primary, primary-f…」；U5-inspector-card-1440-light 卡片底边在视口外。
- 来源：a 新-1；b 新-2。

#### UI-150 · [P3] DOM + GPU 时 DOM 列仍为两列卡片，标题与标签被挤成两行（U5 · 布局 · 可挽回 a 0.1）

- 位置：U5-gpu-compare-1440-light DOM 列 526px，卡片各约 226px。
- 修法：预览 scope 根 `container-type: inline-size`，用容器查询在窄容器下单列（P5 自动宽度跟随可用空间）。
- 来源：a 新-3（b 在建议 4 提出同一点，未计分）。

#### UI-151 · [P3] 未标注态标签仍用选中蓝；面板为空 token 列表留一行空位（U5 · 视觉、细节 · 可挽回 b 0.1）

- 位置：U5-inspector-undeclared-1440-light、U5-inspector-undeclared-panel-1440-light。
- 来源：b 新-3（a 在建议 3 提出标签配色，未计分）。

#### UI-76 · [P3] DOM 与 GPU 两框内的首行内缩相差约 8px（U5 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 (370, 293)，GPU 首行 (908, 285)。
- 来源：a UI-76（部分解决）；b 判已解决（两框上沿对齐）。取部分解决。


## R2 建议（U5，不扣分）

- U5：
  - 工具栏控件高度统一（a、b）。
  - 简单对话框限宽（a、b）。
  - 菜单与触发器留 4px 并加层级（b）。
  - 局部边界标题升级、外观选择器限宽（a、b）。
  - GPU 色样标签不在连字符处断开（a、b）。
  - ✓ IconButton 换不像复选框的图标（a、b）。
  - Host control 加说明（a）。
  - 深色局部对话框遮罩用根主题或中性深色（b）。
  - 面板补色阶信息（b）。
