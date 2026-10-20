R3 对照 15 张覆盖全部类别，与 R6 同名图一致。评审完成，输出结论。

# R6 U1 搜索主路径 独立评审结论

## 评分表（四维+总分）

| 维度 | 得分 |
|---|---:|
| 布局 | 3/3 |
| 视觉 | 3/3 |
| 交互与状态 | 3/3 |
| 细节 | 1/1 |
| **总分** | **10/10** |

## 逐维理由（引用截图文件名）

**布局 3/3（S1、S2、G4）**
- 对话框内容顺序为标签「Search documentation」+ Close、输入框、状态/总数、结果列表、Show more，符合 S2 顺序：见 U1-idle-1280-light.png、U1-results-1280-light.png、U1-keyboard-more-focus-03-1280-light.png。
- 1280 下对话框居中、宽度明显小于视口；375/320 下两侧保留视口边距、文字正常换行，无横向溢出：U1-idle-375-light.png、U1-results-320-light.png、U1-empty-320-dark.png。
- Show more 按钮位于结果列表可视区之外下方，长列表时输入、Close 保持可达：U1-keyboard-select-02-1280-light.png（列表中 Guide sample 5 被截断、Show more 在底部）。
- 顶栏搜索按钮位于语言/主题控件旁，1280 带「Ctrl / Cmd K」提示（U1-idle-1280-light.png）；375 关闭态顶栏只见「搜索」可读名、无快捷键提示（U1-zh-fallback-destination-375-dark.png），符合 S1 的 48rem 以下口径。

**视觉 3/3（G1、G2、S1、S2）**
- 浅色 popover 白底、深色 popover 深蓝黑底，遮罩压暗背景页：U1-idle-1280-light.png vs U1-idle-1280-dark.png。
- 选中项为浅蓝底（「ThemeScope reference」组），命中词为高亮小块（如摘要中「ThemeScope」「SearchCorpusToken」），蓝色仅用于链接/选中/命中/焦点：U1-results-1280-light.png、U1-results-1280-dark.png、U1-keyboard-select-02-1280-light.png。
- 输入框聚焦时有明显蓝色 ring（粗环+间隙）：U1-idle-375-dark.png、U1-more-98-focus-1280-light.png；Show more 焦点环同样清晰：U1-more-20-focus-1280-light.png。
- 深色中文界面控件、文本层级一致：U1-zh-translated-375-dark.png、U1-zh-fallback-375-dark.png。

**交互与状态 3/3（S3、S4、S5、S6、G3）**
- 三态区分明确：idle 提示输入（U1-idle-1280-light.png，记录 status「Type a word…」）、results 带 live 总数（「2 results」「98 results」）、empty 明确「No results. Try a different word or a shorter prefix.」+「0 results」且输入保持焦点（U1-empty-1280-light.png 及记录），符合 S4。
- 键盘链路：Ctrl+K 从品牌链接打开并聚焦输入（U1-keyboard-open-01-1280-light.png）；ArrowDown 后选中项变为第二项而焦点留输入框（U1-keyboard-select-02-1280-light.png 及记录 activeDescendant）；Tab 到 Show more（U1-keyboard-more-focus-03-1280-light.png）；Enter 分页 20→40→60→80→98 且选中项与总数保持（more 系列五张 + 记录 options 递增）；末批 Show more 消失、焦点归输入框（U1-more-98-focus-1280-light.png）；末批后 Tab/Shift+Tab 仍可操作（U1-more-final-keyboard-1280-light.png，输入框文本呈选中态）。
- Esc 关闭并归焦真实触发点（品牌链接带蓝环）：U1-keyboard-close-05-1280-light.png；Meta+K 重开 Enter 导航至 #custom-section 并关闭（U1-keyboard-destination-06-1280-light.png，URL 记录含锚点）。
- 三种 base 激活均导航到 ThemeScope reference 页、侧栏当前位置蓝色高亮、对话框关闭：U1-subpath-activation-a2-1280-light.png、U1-root-activation-a2-1280-light.png、U1-meta-activation-a2-1280-light.png；root/meta 结果与 subpath 一致：U1-root-results-1280-light.png、U1-meta-results-1280-light.png。
- 中文：译文结果为真实中文（「主题切换」「1 个结果」：U1-zh-translated-375-dark.png）；缺译结果标题旁有「zh」badge、提示「尚未翻译，内容语言：en」（U1-zh-fallback-375-dark.png）；激活后仍在 zh 路径，目标页显示「此页面尚未翻译。」note、界面中文：U1-zh-fallback-destination-375-dark.png，符合 S6。

**细节 1/1**
- 单复数与数量文案正确（2 results / 0 results / 98 results / 1 个结果）；分页后选中项不丢（more 系列记录 selected 恒为 rd-search-option-1-2-1）；320 下 empty 输入框长文本呈原生横向滚动（U1-empty-320-light.png 显示「.bsolutely…」），属单行 input 正常行为，非页面横滚。

## 与 R3 对照（回归/无回归）

无回归。实际打开 R3 同名图 15 张逐张对照，覆盖全部类别，与 R6 一致：U1-results-1280-light、U1-results-320-light、U1-results-320-dark、U1-idle-375-light、U1-empty-320-light、U1-empty-375-dark、U1-keyboard-open-01、U1-keyboard-select-02、U1-keyboard-close-05、U1-keyboard-destination-06、U1-more-20/40/80/98-focus、U1-more-final-keyboard、U1-zh-translated、U1-zh-fallback、U1-zh-fallback-destination、U1-root-results、U1-meta-results、U1-subpath-activation-a2（其中 15 张逐项核对像素级一致，其余类别由 capture.md 记录的 build_id 与 R2–R5 逐位一致佐证）。

R3 评审已关闭差距 UI-01、UI-02（R3 judge 原文只载编号与「已解决」结论，无原始标题；R3 实际核对点为窄屏面板边距与分页焦点）：本轮 320/375 面板两侧边距保持（U1-idle-320-light.png、U1-idle-375-dark.png）、Show more 在列表外（U1-more-20-focus-1280-light.png）、连续分页焦点环与末批归焦输入框维持（U1-more-80-focus / U1-more-98-focus）。未见复发。

## 已确认差距（或「无」）

无。

## 待核实

无。

## 未证清单

以下行为/属性在本轮材料中看不到或测不出，不猜不判：

1. 点击 Close 按钮后的归焦画面（本轮归焦证据仅为 Esc 自品牌链接打开的场景，U1-keyboard-close-05-1280-light.png）。
2. 关闭后重开、改写查询之前是否完整保留原查询（U1 矩阵无此帧）。
3. IME 组合输入过程中是否避免触发查询/选择（S5）。
4. 结果列表滚动到底、首/尾选项导航与选中项自动滚入可视区；more 系列截图均停在列表上部，组数增长仅由记录证明。
5. Close 的键盘焦点画面、各控件悬停与按下态。
6. 「Search documentation」标签与结果标题的语义标签层级（S1「无第二个 H1」）；截图只能确认视觉上无大标题样式。
7. 源标题/路径/摘要各自的真实 source lang 语义（S6）；可见文本语言可确认，元数据只有 htmlLang。
8. 具体颜色 token 数值与对比度比值（G2/G3 的 4.5:1、3:1）；截图只能确认视觉口径，数值测量不在本评审材料内。
9. 原生设备软键盘、原生 200% 缩放、VoiceOver 朗读——属 U4，本轮 capture.md 声明整体 missing，不由 U1 材料代证。

## 建议（不扣分）

1. 同一摘要内多次命中目前只高亮首个可见命中：U1-results-1280-light.png 第二组「Guide sample 3」摘要含十余次 ThemeScope，仅首处高亮。S3 未要求高亮全部出现位置，可考虑增强。
2. 被截断的摘要末尾无省略号（同上图第二组末尾「…ThemeScope The」截断无标示）。S3 未规定，可考虑补充以示截断。
