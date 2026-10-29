这是本轮补采后的全新独立复核；不知道首位评审的本轮分数，也不读取本轮 judge 或报告。只依据完整当前材料给本单元整组四维分。不要修改或新建文件。

本任务只评 U1，不要给其他单元打分。本轮是 R6；目标包已记录用户对 UI-115 的范围接受，完整变更记录须读取。你继承宿主模型与 effort，不需要读取任何其他技能或仓库规则，不派生，不改文件。将完整结论直接返回给主 agent。

严格白名单：
- 任务说明：本文件。
- 冻结目标包：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target.r6.md，只沿其中目标 PNG 链接读取附件，不沿来源链接打开其他文件。
- 本轮本单元清单：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-U1-b.md。
- 本轮本单元实际操作：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-actions-b.log。
- 本轮原图：仅下列 45 个绝对路径：
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-default-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zh-top-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zh-configured-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-default-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-edit-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-colors-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-01-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-02-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-03-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-keyboard-04-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-export-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-field-tabs-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-configured-lower-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-720-light-a2.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-zoom200-source-end-720-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-configured-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-01-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-02-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-04-768-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-configured-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-01-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-02-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-03-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-keyboard-04-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-default-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix-zh-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-zh-top-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-zh-configured-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-01-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-02-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-03-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U1-matrix2-keyboard-04-1440-dark.png
- 上一轮本单元独立评审原文：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge-inputs/r5-judge-U1.md。
- 上一轮原图白名单：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge-inputs/r5-images-U1.txt 中的 PNG。优先对照未关闭差距所需的旧图；其余按需比对。

只打开上述材料，不读同目录其他 JSON、日志、代码、完整报告、机器检查或修复记录。先读清单与目标，再实际调用 view_image 打开当前原图与所需目标/旧图；纯文件尺寸与像素统计不能代替开图。勿凭未操作、未拍的状态补造通过或失败。

你是独立的 UI/UX 验收评审，只读，不修改文件，不派生其他 agent。请判断当前产品界面离目标还差多少，按标尺打分，并给出完整、可执行的差距清单。

材料（只打开这里列出的文件和目录；目标包里指向其他文件的链接，除目标图外都不要打开）：
- 目标包：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target.r6.md（单元、目标图、规范要点、比较口径）
- 本轮截图与交互记录：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6；每张图对应的单元、状态、视口和主题见 /Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-U1-b.md
- 上一轮截图与评审结论：/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge-inputs/r5-judge-U1.md；上一轮截图只读 /Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge-inputs/r5-images-U1.txt 列出的文件（首轮没有）
- 差分图（可选）：无

逐个单元打分，可以给小数：
- 布局（0–3）：区域结构，主要组件的位置、尺寸、对齐、间距和层级；每个视口下的排布、换行和滚动；有没有溢出、遮挡、截断。
- 视觉（0–3）：配色与用色语义，字体（字族、字号、字重、行高），对比度，图标与图片，圆角、边框和阴影，主题；是否与规范 token 一致。
- 交互与状态（0–3）：主路径能否走通；加载、空、错误、禁用、成功等状态及反馈；悬停、焦点、按下态；键盘操作与焦点可见；表单校验与提示；过渡是否帮助理解。
- 细节（0–1）：逐处细抠文案、数字与日期格式、图标对齐、像素级间距、截断与省略、多语言。一个像素都不应不同。

锚点：
- 3 分：与目标一致，只剩应归入「细节」的微小差异。
- 2 分：方向与结构正确，但有设计师或用户一眼就能看出的偏差。
- 1 分：只有部分正确，有结构性偏差或多处缺失。
- 0 分：缺失、根本不同或不可用。
- 细节维度：1 分 = 逐处比对没发现差异；0.5 分 = 有若干小差异；0 分 = 差异很多。
每个维度用一句话说明为什么不是更高一档。

要求：
1. 挑剔、精确、完整。差距很多就列一份很长的清单。目标是另一位 agent 照单修完回来后，分数明显提高。
2. 每个单元的失分都要能由差距解释：该单元各条差距的「可挽回分」相加，约等于 10 减去单元分，误差不超过 0.5。
3. 每条差距都要可执行，写清以下内容：
   - 单元与截图中的位置（区域、元素或文字）；
   - 「现状 → 期望」，期望引用目标图的位置或规范要点；
   - 具体修法：改哪个组件、用什么布局方式、哪个 token 或属性、什么素材、什么结构；
   - 严重度和可挽回分。
   不要写「看起来廉价」「不够精致」这类话，要说出造成这种观感的具体原因和改法。
4. 期望在目标图和规范要点里都找不到依据的，单独列为「建议」，不扣分。按目标包的比较口径，忽略示例数据之类的差异。
5. 需要推倒重做布局、换组件或重做素材时，直说，不要粉饰。目标是做到与目标一致，不要放水。
6. 只依据截图与记录下判断，不凭常识、经验或对常见实现的了解去推测界面效果。没拍到的状态既不算通过也不算失败；图缺失、模糊或看不出状态时，写明哪张图看不出什么。
7. 有上一轮评审时，保持评判尺度一致，但不必维持或提高分数；变差了就给更低的分。逐条给上一轮差距标：已解决、部分解决、仍存在、回归、待核实（看不出）、不再适用。同一问题沿用原编号；新问题用临时编号「新-1」「新-2」……
8. 未评单元不补零分或假结论；保留这次输出作为评审原文。managed 输入由你本人输出，或由主 agent 忠实转换并保留原文与转换依据，不能替你补分、改理由或删差距。

严重度：
- P0 阻断：主任务无法完成；内容或数据错误、不可见；必需视口的布局崩坏。
- P1 严重：用户会明显困难或误解；主路径不满足 WCAG AA（对比度、键盘、焦点）；主要区域与目标明显不符。
- P2 一般：能察觉的偏差，但不妨碍完成任务。
- P3 打磨：几乎察觉不到的细微差异。

输出：
1. 评分表：每个单元的四个维度分和单元总分，每个维度附一句理由。
2. 上一轮差距复核表（首轮省略）。
3. 差距清单：所有仍未关闭的差距，包括沿用编号的旧差距和新差距。先按严重度、再按可挽回分排序。每条写编号、单元与截图、维度、严重度、现状 → 期望（依据）、修法、可挽回分。
4. 建议（不扣分）。
5. 看不出的格子：哪张图、看不出什么。
6. 总评：最低分单元及主要原因；哪些地方需要结构性改变。

请在末尾附上实际使用的开图工具和实际打开的图片绝对路径清单；不要把仅列目录或读取尺寸算作开图。
