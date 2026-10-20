本次为 R5 的 U4 原生 iPhone 搜索分组独立评审，只依据目标摘录、20 张本轮截图、对应 AX 和包内动作记录。U4 矩阵不完整，四维及单元总分均留空；U1–U3 本轮未复核。

| 维度与标尺 | 已证实的表现 | 无法证实的部分 | 得分 |
|---|---|---|---|
| 布局，0–3 | 02–05、12–15、24–26、30–36 中，标签、关闭、输入、状态/总数与结果顺序清楚；窄屏文字在面板内换行。30 的 Retry、33–36 的 Show more 均可见。所拍画面未见控件被浏览器底部工具栏遮住。符合 S1–S2 的可见结构。 | 实际 CSS 视口、精确尺寸与高度上限；完整软键盘展开后的可达性；页面无横滚；结果列表内部滚动及全部后续组可达。 | |
| 视觉，0–3 | 浅、深主题下，输入焦点环、面板边界、选中组和命中背景可辨；正文、路径、缺译提示有层级。05、26 的缺译项保留英文内容，中文提示和 `zh` badge 可辨。与 G1–G2、S2、S6 的方向一致。 | 精确 token、字号及间距；WCAG AA 对比度量测；其他原生浏览器和缩放条件下的一致性。 | |
| 交互状态，0–3 | 02/03 有输入光标和焦点环；04→05 显示加载后出现 3 个结果；12→13 显示加载后进入无结果并保留输入焦点；14→15 显示关闭后重开，原查询和无结果状态保留。30→31/32 显示错误状态经实际 Retry 后，同页保留查询并得到 1 个结果。26 证实中文 404 内校正查询后能显示结果。 | 关闭后的真实归焦、Esc、完整键盘路径、Enter/点击结果落地、当前完整 URL、IME 与旧响应边界；Show more 新增组数；VoiceOver 实际朗读。 | |
| 细节，0–1 | idle、loading、empty、error 文案明确区分；中文界面的按钮、总数和无结果建议使用中文，英文语料页使用英文。23 的中文 404 有明确标题、返回首页及语言链接。可见内容无妨碍理解的截断。 | 屏外结果、极长词边界、完整 URL 和辅助技术语言信息。镜像 AX 不能证明手机页面的语义或朗读语言。 | |
| U4 单元总分 | 未评分：本分组不足以覆盖完整 U4。 | 不补零分，不生成局部分组总分。 | |

上轮复核：R4 `judge.md` 没有已确认的新增产品差距，也未给 U4 完整评分。实际查看的两张 R4 手机图分别是“iPhone 使用中”和“已超时”的镜像接入状态，不是产品界面。本轮补入了真实产品画面，但不存在可据此关闭的旧产品差距，也不能关闭整个 U4 的缺证项。

本轮没有已确认的新产品差距，不分配“新-S-*”编号，不提出产品修复要求。

有两处必须保持待核实：

- 24/25 实际查询不是 `ThemeScope`，其 0 结果不能作为目标词搜索失败。26 中实际输入 `themescope` 后出现 3 个结果，才是校正查询的结果证据。
- 33–36 始终从 `Theme switching` 首组开始显示。34 点击 Show more 后总数仍为 98，首组选中外观保留；35/36 的背景所见段落变化，但结果列表没有显示后续组。可以确认控件仍可见，不能确认已增加 20 组或列表内部滚动成功，也不能据此断言滚动实现失败。

建议优先补证，不在当前证据基础上调整产品样式。具体操作如下：

| 本分组看不出的格子 | 补证操作与成功观察点 |
|---|---|
| 完整软键盘下的搜索布局，S1–S2、U4 | 在真实手机上展开完整字母/中文键盘，分别拍结果、无结果和错误状态；实际操作输入、关闭、Retry、Show more，记录控件是否可达。当前只见输入工具栏。 |
| 结果列表内部滚动与更多，S2–S3 | 记录列表内连续滑动前后画面，出现可辨认的后续组；点击 Show more 前后核对可访问组数由 20 增至 40、总数仍为全部匹配页数，并核对当前选择保留。 |
| 原生键盘和归焦，G3、S3、S5 | 实际走 Cmd/Ctrl+K、上下选择、Enter 锚点导航、Tab 到关闭/重试/更多、Esc；关闭后继续键盘操作，证明焦点回到真实触发点。 |
| 中文 404 的结果落地，S5–S6、O4 | 激活 26 中的结果，记录对话框关闭、目标标题/锚点和 Safari 可读完整地址，确认仍在中文路径及部署 base 内。 |
| IME、会话与晚响应，S4–S5 | 记录组合输入、加载中关闭重开、改词后晚响应到达的连续动作及最终结果。 |
| 原生 200%、其他平台，G4、U4 | 本分组不代填桌面 Chrome/Firefox/Safari、Android Chrome 或原生 200% 格子；合并其他独立证据，缺失项再实机补采。 |
| VoiceOver，G3、U4 | 使用真实 Safari + VoiceOver 记录结果数、当前选择和复制成功/失败的实际朗读与操作；镜像 AX 不作替代。 |

总评：这组证据补实了原生 iPhone 上搜索打开、加载、结果、无结果、关闭重开、中文 404 搜索和模块失败后同页 Retry 恢复。所拍状态没有已确认的新产品差距。完整软键盘、列表滚动/扩展和辅助技术等仍未证实，U4 保持未评分、未完成全矩阵验收。

实际逐张通过 `view_image` 打开的文件如下。R5 共 20 张：

```text
U4-iPhone-02-search-open.jpg
U4-iPhone-03-search-input-tap.jpg
U4-iPhone-04-theme-query.jpg
U4-iPhone-05-theme-results.jpg
U4-iPhone-12-empty-query-attempt.jpg
U4-iPhone-13-empty-result.jpg
U4-iPhone-14-search-closed.jpg
U4-iPhone-15-search-reopened.jpg
U4-iPhone-23-zh-deep-404.jpg
U4-iPhone-24-404-query.jpg
U4-iPhone-25-404-results.jpg
U4-iPhone-26-404-query-corrected.jpg
U4-iPhone-29-fault-fixture-entry.jpg
U4-iPhone-30-module-error.jpg
U4-iPhone-31-retry-attempt.jpg
U4-iPhone-32-retry-result.jpg
U4-iPhone-33-many-results.jpg
U4-iPhone-34-show-more.jpg
U4-iPhone-35-result-list-scroll.jpg
U4-iPhone-36-list-settled.jpg
```

R4 共 2 张，仅用于核对接入状态：

```text
U4-iPhone-connection-state.jpg
U4-iPhone-timeout-state.jpg
```

合计实际打开 **22 张 JPG**；完整读取上述 R5 同名 **20 份 `.ax.txt`**。这些 AX 均只列出“iPhone镜像”窗口与应用菜单，没有手机页面 DOM、焦点或朗读信息。

评审工具路径：`functions.exec → tools.view_image` 逐张看图；`functions.exec → tools.exec_command` 读取白名单文档及 AX。原始采集路径按材料记录为 CUA → 原生 iPhone Mirroring，本评审未重新操作设备。未沿链接读取代码、完整报告或机器检查，未写文件、未派生 agent。实际模型：未知；effort：未知。
