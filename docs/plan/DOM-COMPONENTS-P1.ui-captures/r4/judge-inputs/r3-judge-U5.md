# R3 评审结论 · U5 摘录

摘自 R3 合并结论 judge.md，只含 U5 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U5 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U5 Studio 预览 | 2.9 | 2.95 | 2.8 | 0.8 | **9.45** | a | — | 9.1 |

### 各维度理由

**U5（a）**

- 布局 2.9：拾取面板移出文档流，开启拾取不再下推预览；DOM + GPU 时 DOM 列单列；两框首行上沿对齐；390 对话框两侧各留 16px。扣分原因：拾取浮动面板固定压在预览右下角，挡住被拾取的「Create · 0」和右卡内容；拾取标签盖住相邻复选框与文字；两框首行左内缩差 8px。
- 视觉 2.95：分段控件三色正确，#8AB7F7 内框和段间双线消失；拾取矩形在蓝色按钮上清晰可辨；未标注标签改用 #475467；面板浅蓝；编辑壳在深色预览下保持中性浅色。扣分原因：未标注目标被面板遮住时，矩形描边和标签画在面板之上，与面板文字交叠。
- 交互 2.8：菜单、对话框、局部边界相反外观及其浮层、对比区 Dark、DOM + GPU、拾取后焦点进入编辑器字段、未标注说明都走通。扣分原因：三张未标注帧里被拾取部件大半藏在面板下；拾取 Create profile 时两行标签盖住相邻复选框和「I understand」，并在拾取后一直保留。
- 细节 0.8：标签在连字符处断成「primary-」「foreground」；卡片标签切掉上一行文字的下伸部；预览头部「Rustify Light」与「Light preview」之间丢了分隔符；按下态加粗使相邻控件横移 1–2px；两框首行左内缩差 8px。

## R3 未关闭差距（U5）

#### UI-161 · [P3] 预览标题旁去掉分隔点后读成「Rustify Light Light preview」（U1/U5 · 细节 · 可挽回 U1 a 0.05、U5 a 0.05）

- 位置：U1-default-1440-light 等帧及 U5-dom-1440-light、U5-gpu-compare 右上 x≈1266–1416、y≈88–98；U5-dom-1440-dark 为「Rustify Light　Dark preview」；U5-preview-390-light y≈229。R2 同位置是「Rustify Light · Light preview」。
- 现状 → 期望：主题名与预览模式连读 → C4 显示「预览模式」以免与作者外观混淆。
- 修法：恢复只在同一行时显示的「 · 」分隔（换行时去掉悬挂点），或把模式写成「Preview: Light」或中性胶囊；中文同步。
- 来源：U1 a 新-5、U5 a 新-3。判为同一问题的依据：同一位置、同一文字、同一原因（R2 的分隔点被去掉）。

#### UI-186 · [P2] 拾取浮动面板固定在预览右下角，遮住被拾取部件和预览内容；选中矩形与标签画在面板之上（U5 · 交互、布局、视觉 · 可挽回 a 0.25）

- 位置：U5-inspector-undeclared-hover / -undeclared / -undeclared-panel 面板 x 1000–1416、y 740–883；U5-inspector-card / U5-inspector / U5-inspector-picked 面板 y 707–883。「Create · 0」主体被面板压住；「Not annotated」标签（y 849–869）压过面板文字；刚开启时面板挡住右卡「Sample actions: 0」「ThemeDocument」。
- 现状 → 期望：被定位的部件和信息面板要同时可见、互不遮挡 → P2。
- 修法（需要结构性调整）：推荐把面板从覆盖层移进预览列网格的最后一行（`grid-template-rows: auto minmax(0,1fr) auto`，面板 `max-height:40vh; overflow-y:auto`），选中矩形与标签的覆盖层放在预览滚动区内并 `overflow:clip`。坚持浮动面板则至少给滚动容器留出面板高度的底部内边距、与目标相交时把面板翻到右上角、覆盖层 `z-index` 低于面板。
- 来源：U5 a 新-1（评审注明这是 UI-147 修法带出的新问题）。

#### UI-187 · [P3] 拾取标签盖住相邻内容，并在连字符处断开 token 名（U5 · 细节、交互、布局 · 可挽回 a 0.15）

- 位置：U5-inspector-1440-light、U5-inspector-picked-1440-light Create profile 上方标签 x 396–507、y 603–639，两行「primary, primary-」「foreground, ring」，盖住复选框与「I understand」；U5-inspector-card-1440-light 标签 y 361–383 切掉上一行文字的下伸部。
- 修法：标签单行 `white-space:nowrap; width:max-content`，token 不在内部断行；内容缩成「首个 token +N」；大部件把标签放进矩形内侧左上角，小部件放外侧、空间不够时翻到下边；`pointer-events:none`；拾取确认后只保留矩形。
- 来源：U5 a 新-2（评审注明这是 UI-149 修法带出的新问题）。

#### UI-188 · [P3] 选中或按下后字重变粗，按钮变宽，右侧控件横移 1–2px（U5 · 细节 · 可挽回 a 0.05）

- 位置：U5-dom-1440-light 对 U5-inspector-card-1440-light，Inspect 由 x 756–822 变为 756–824，Focus preview 右移 2px；U5-gpu-compare 选 DOM + GPU 后 Inspect 右移 1px。
- 修法：给分段与切换按钮的标签预留粗体宽度（隐藏的粗体伪元素或 `min-width`），在共用的分段组件里改一次。
- 来源：U5 a 新-4。

#### UI-76 · [P3] DOM 与 GPU 两框首行左内缩相差 8px（U5 · 细节、布局 · 部分解决 · 可挽回 a 0.05）

- 位置：U5-gpu-compare-1440-light DOM 首行 x=370（内缩 26），GPU「GPU · Cards」x=908（内缩 18）；纵向已对齐。
- 修法：GPU 场景根 padding 改为 24px，或两边统一取同一 token。
- 来源：U5 a UI-76（部分解决）。

（U5 另有 UI-161，见 U1 节。）

### U6

## 建议（不扣分，U5）

  - 简单对话框限宽 28–32rem 并垂直居中（本轮改为贴顶）。
  - 菜单与触发器留 4px 并加层级阴影；局部边界标题升级、外观选择器限宽。
  - GPU 色样标签不在连字符处断开；Host control 加一行说明。
  - 深色局部对话框的遮罩取根主题或中性深色。
  - 面板补色阶信息（属 F19，只作建议）；拾取面板提供折叠入口。
  - 390 下 Preview 页的粘性页头可收成一行（属 U1 范围）。
