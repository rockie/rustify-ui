U1 本轮评分 **10.0 / 10**，仅代表已截图、已记录的观察范围。已逐张打开 R2 登记的 38 张图及 R1 同名 28 张图。UI-01、UI-02 在 U1 范围内均已解决；未观察到有规范依据的新扣分差距。缺证状态单列，不视为通过或失败。本次不评分 U2、U3、U4。

**1．四维评分**

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
|---|---:|---:|---:|---:|---:|
| U1 搜索主路径 | 3.0 / 3 | 3.0 / 3 | 3.0 / 3 | 1.0 / 1 | **10.0 / 10** |

- **布局：**已见桌面及 375/320 视口中，标签与 Close、输入、状态、总数、结果列表、Show more 的顺序和层级符合 S1–S2；窄屏长内容换行，关键控件未被遮挡，面板宽度已符合规定。
- **视觉：**已见浅深主题的面板、输入、边界、圆角、文字层级、选择背景、高亮和焦点环与 G1–G2、S1–S2 相符，未见需要扣分的视觉偏差。
- **交互与状态：**记录覆盖点击和快捷键打开、输入聚焦、results/empty、键盘选择、连续分页及末批焦点、Esc 归焦、普通导航、同页锚点、部署 base 和中文缺译导航；已观察动作符合 S3–S6。
- **细节：**已见文案、结果数量、中文控件、缺译说明、目标语种 badge、对齐和窄屏边距相符；原有约 6px 的宽度偏差已消失。

**2．旧差距复核**

| 编号 | 状态 | 本轮证据与结论 |
|---|---|---|
| **UI-01** Show more 激活后键盘焦点丢失 | **已解决** | R1 `U1-keyboard-more-04-1280-light.png` 的按钮焦点环消失；R2 同名图保留明确焦点环。R2 `U1-more-20/40/60/80-focus-1280-light.png` 均显示 Show more 焦点，操作记录的实际焦点均为该 BUTTON；组数依次为 20、40、60、80。`U1-more-98-focus-1280-light.png` 中按钮移除后，焦点回到输入且环可见。全程 `selected` 与 `activeDescendant` 均保持 `rd-search-option-1-2-1`；末批后的 Tab/Shift+Tab 记录仍落回输入。符合 G3、S2、S3。 |
| **UI-02** 窄屏 dialog 比规定宽度少 6px | **已解决** | R2 的 375/320、浅深主题、idle/results/empty 图中，面板左右均约 16px：375 外宽约 343px，320 外宽约 288px；中文两张结果图同样符合。与 R1 约 19px 的两侧留白相比，已满足 S1 的 `viewport − 2rem`。输入焦点环、Close、内容换行未因此出现溢出。本结论仅覆盖 U1，不推及 U2。 |

**3．全部未关闭差距**

无已证实的未关闭差距，无新增编号。

本轮可挽回分合计 **0.0**，与 `10 − 10.0` 一致。以下证据缺口不登记为产品缺陷，也不扣分。

**4．不扣分建议**

- `U1-results-*` 的第二组摘要中，多个可见 `ThemeScope` 只有首处高亮。可以考虑高亮全部可见命中；S3 未要求全部命中高亮，因此不扣分。
- 桌面摘要存在直接结束于 `The` 的截取。可以考虑增加省略号；目标未规定摘要截取规则，示例正文也不作像素比较，因此不扣分。

**5．看不出的格子**

| 尚不能判断的状态 | 已有证据的边界 | 需要补充的证据 |
|---|---|---|
| 窄屏关闭状态搜索入口的完整矩阵 | 新增中文缺译目的页证明了 **375 dark 中文**入口可读且未显示快捷键提示；其余窄屏图仍主要为打开状态 | 320/375、浅深主题的英文关闭状态，以及其余中文对应状态；核对按钮名和快捷键提示隐藏 |
| 点击 Close，以及鼠标点击 Search 后关闭的归焦 | 当前只有快捷键从品牌链接打开、Esc 回到该品牌链接的完整记录 | 点击 Search→点击 Close、点击 Search→Esc 的关闭后截图及实际活动元素 |
| 查询保留后的重开瞬间 | Esc 后记录保留 `SearchCorpusToken`；重开后随即输入了新查询 | 重开后不输入、不清空的截图和查询记录 |
| 列表滚动与离屏选择 | 已证明列表区域与外部 Show more 分离，未拍列表滚到底或选择越过可视范围 | ArrowDown 连续移出首屏、列表末尾、ArrowUp 返回的截图；记录选择及滚动位置 |
| 窄屏长列表下 Show more 的持续可达性 | 窄屏结果仅有两组；长列表及分页证据来自桌面 | 320/375 长列表，滚动前后输入、Close、Show more 的可达状态 |
| 真译中文结果的实际激活目的页 | 已见中文真译结果；缺译结果已补齐真实 `/rustify-ui/zh/page-0051/` 目的页 | 激活中文真译结果后的实际 URL、页面及关闭状态 |
| 源标题、路径、摘要各自的真实 source lang | 图片能确认英文文本和“内容语言：en”；记录仅有文档级 `htmlLang` | 对应结果片段实际语言语义的观测记录，不能用页面 `htmlLang` 代填 |
| IME、hover、按下态、完整 Tab 路径及过渡 | 无这些状态的专门截图或动作记录 | 组合输入前后查询/选择记录，Close 的键盘焦点状态，以及有关指针和动态状态 |
| 打开与聚焦的同步时机 | 当前为“等候 idle”之后的快照 | 在模块/索引尚未就绪时，打开即显示 dialog、输入已获焦的动作和时间记录；当前 U1 材料无法独立证明这一时序 |

本轮新增材料已消除上一轮关于 **ArrowUp 实际操作、root/meta 激活后的 base 保持、中文缺译导航地址、连续分页至末批焦点**的主要证据缺口。ArrowUp 越过列表可视范围的滚动行为仍未拍到。

**6．总评、结构判断及工具轨迹**

已观察范围内，搜索布局和状态结构与目标相符，两个旧差距均有截图及操作记录闭环。**无需结构性改变，也没有依据要求更换组件或重做视觉。**剩余工作是补齐上表中的证据格子；不能从当前满分推导为所有未拍状态均符合规范。

只读取了以下白名单文字材料：目标包、本轮 `U1-capture.md`、`U1-operations.md`、`U1-capture.json`、`previous-U1.md`，以及 R1 `judge-U1-a.md`。通过 `functions.exec` 调用 `exec_command` 读取，首次合并回显截断后，单独重读完整操作记录，并用 `jq` 展开 capture.json 的文件、URL、动作及观测字段。未读取实现、脚本、计划、PRD、其他报告或修复记录；未运行页面、未修改文件。

图片通过 `tools.view_image` **逐张打开并逐张呈现，共 66 次**。没有用哈希、尺寸或文件名替代看图。未打开未登记的 `U1-subpath-activation-1280-light.png`。

下列 **28 个文件名分别打开了 R1、R2 两份**，共 56 张：

```text
U1-idle-1280-light.png
U1-results-1280-light.png
U1-empty-1280-light.png
U1-idle-1280-dark.png
U1-results-1280-dark.png
U1-empty-1280-dark.png
U1-idle-375-light.png
U1-results-375-light.png
U1-empty-375-light.png
U1-idle-375-dark.png
U1-results-375-dark.png
U1-empty-375-dark.png
U1-idle-320-light.png
U1-results-320-light.png
U1-empty-320-light.png
U1-idle-320-dark.png
U1-results-320-dark.png
U1-empty-320-dark.png
U1-keyboard-open-01-1280-light.png
U1-keyboard-select-02-1280-light.png
U1-keyboard-more-focus-03-1280-light.png
U1-keyboard-more-04-1280-light.png
U1-keyboard-close-05-1280-light.png
U1-keyboard-destination-06-1280-light.png
U1-zh-translated-375-dark.png
U1-zh-fallback-375-dark.png
U1-root-results-1280-light.png
U1-meta-results-1280-light.png
```

另逐张打开了 **R2 新增的 10 张**：

```text
U1-more-20-focus-1280-light.png
U1-more-40-focus-1280-light.png
U1-more-60-focus-1280-light.png
U1-more-80-focus-1280-light.png
U1-more-98-focus-1280-light.png
U1-more-final-keyboard-1280-light.png
U1-subpath-activation-a2-1280-light.png
U1-root-activation-a2-1280-light.png
U1-meta-activation-a2-1280-light.png
U1-zh-fallback-destination-375-dark.png
```
