1. **U4 保持未评分。** 本次仅评 R4 原生 macOS Safari 补充证据，运行版本图显示 27.0.1（22625.1.29.11.28）。没有复核 U1–U3，也没有以局部 Safari 结果补齐 Chrome、Firefox、移动设备或 VoiceOver。四维只给已拍范围的定性观察，不给总分。

2. **已拍状态逐维观察。**

| 维度 | 本轮观察及边界 |
|---|---|
| 布局 | 宽窗口阅读页三栏清楚；恢复较窄窗口后，菜单与目录收起、正文单列。明确选择原生 200% 后，所拍正文能够换行，搜索输入、关闭按钮和 empty 文案都在窗口内，没有看到必需内容被裁掉。尚不能据这些静帧认定全部长内容均无页面横向滚动。 |
| 视觉 | 浅色、深色阅读与深色搜索均有清楚的文字层级；搜索输入蓝色焦点环、结果选择边界与命中背景可辨。缺译 note 位于 H1 后、正文前，表现为安静提示。照片不足以单独证明全部 WCAG AA 对比度阈值。 |
| 交互 | 已见语言普通链接切换；Cmd+K 打开并聚焦输入；loading 后显示 3 组结果；Down 选择 Architecture 后 Enter 到 `architecture/#versioned-themes`，对话框关闭。Copy 点击后显示 Copied；未核对剪贴板实际内容或朗读。模块错误后鼠标点击 Retry，保留 `CustomOnlyToken` 并显示 1 个结果。演示实际为 0→1→2，再由 Reset 回到 0。中文深 404 上粘贴查询后显示 2 个结果，Enter 到中文 quickstart，并关闭搜索。 |
| 细节 | idle、loading、empty、error 的可见文案明确区分；中文搜索标签、关闭按钮、数量和 404 文案匹配当前界面语言。缺译页保留英文正文并显示“此页面尚未翻译”。没有从截图中的源码示例推断任何行为。 |

关键证据边界：

- `06-search-results`、`07-search-selected` 实际仍是 loading；`08-search-destination` 实际仍在结果对话框内。完成导航依据是 `10-ready-destination`。
- `12-retry-focus`、`13-retry-attempt` 的 AX 焦点在 HTML 内容，`14-option-tab-focus` 在搜索容器；这组操作未完成键盘重试，不能据此判断 Retry 完整键盘路径不可达。
- `18-demo-reset` 实际是再次 Increment，不能记作 Reset 成功；归零依据是 `19-demo-reset-confirmed`。
- `22-zh-404-query` 输入为空；成功查询依据是 `23-zh-404-pasted-query`，它不证明 IME 组合输入。

3. **确认产品差距：无。**

本轮没有发现能够由实拍与实际动作直接确认、并对应目标条款的新增产品差距。未完成 attempt 保持未证，不转成产品失败。

另有一项**证据元数据问题**：初读时，清单中 `04-maximized-home` 至 `09-ready-selected` 六项 `displayedURL` 被误标为“原生About对话框”，实际照片是站点或搜索。后续已核对新旧 JSON：仅这六个字段改为“本帧AX增量未含完整URL；见原图与实际操作记录”，旧清单保留。此为元数据修正，不作为产品差距或新增成功证据。

4. **尚看不出的格子与具体补证。**

| 未证范围 | 下一步应补什么 |
|---|---|
| 原生 Chrome、Firefox | 记录实际运行版本，补阅读、导航、搜索、主题及相应键盘路径；本次不继承其他轮结果。 |
| iOS Safari、Android Chrome | 在真实设备打开固定产物，补阅读、菜单/目录导航、搜索结果导航、主题及软键盘展开后的布局与关闭操作。记录实际设备、系统和视口信息。 |
| iPhone 接入 | 最新照片及 AX 显示“已超时”，原因是连接期间 iPhone 被使用，要求连接前锁定。当前没有产品页面图。按接入记录，需结束麦克风占用、锁屏并恢复可操作镜像后再采集；这是环境缺证。 |
| Safari 完整键盘路径 | 记录当前 Safari 键盘导航设置，从明确的输入焦点开始连续拍 Tab／Option+Tab／Shift+Tab，实际到达并激活 Close、Retry、Show more；补 Esc 关闭、真实触发点归焦及重开保留查询。 |
| 200% 与内容滚动 | 补长结果列表实际滚动、更多/重试控件可达、长代码与宽表自身横向滚动，并实际检查页面没有横向滚动。当前主要覆盖正文片段及搜索 idle/loading/empty。 |
| VoiceOver + Safari | 开启真实 VoiceOver，保存实际朗读或操作记录：全部结果数量、当前选择、复制成功及失败反馈。AX 中的 selected 或 Copied 不能代替朗读。 |
| 会话与输入边界 | 补真实 IME composition；快速关闭重开及晚响应不污染查询；同页锚点激活；语言菜单 Escape 关闭归焦。 |
| G4／O4 其余状态 | 补减少动态效果偏好、禁脚本内容阅读与普通导航，以及禁脚本深 404。未拍到的状态保持未证。 |

5. **实际开图清单与总评。**

以下目录中 36 张 Safari JPEG 均已逐张打开，并核对对应同名 `.ax.txt` 的相关 URL、控件、状态、选择及焦点字段：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOCS-SSG-P4.ui-captures/r4/`

```text
U4-Safari-01-about.jpg
U4-Safari-02-en-home.jpg
U4-Safari-03-english-home.jpg
U4-Safari-04-maximized-home.jpg
U4-Safari-05-search-idle.jpg
U4-Safari-06-search-results.jpg
U4-Safari-07-search-selected.jpg
U4-Safari-08-search-destination.jpg
U4-Safari-09-ready-selected.jpg
U4-Safari-10-ready-destination.jpg
U4-Safari-01-quickstart.jpg
U4-Safari-02-copy-feedback.jpg
U4-Safari-03-zoom-attempt.jpg
U4-Safari-04-zoom-native-value.jpg
U4-Safari-05-zoom-200-setting.jpg
U4-Safari-06-zoom-200-read.jpg
U4-Safari-07-zoom-200-search-open.jpg
U4-Safari-08-zoom-200-query-attempt.jpg
U4-Safari-09-zoom-200-empty.jpg
U4-Safari-10-fault-entry.jpg
U4-Safari-11-module-error-attempt.jpg
U4-Safari-12-retry-focus.jpg
U4-Safari-13-retry-attempt.jpg
U4-Safari-14-option-tab-focus.jpg
U4-Safari-15-retry-click.jpg
U4-Safari-16-zh-button-demo.jpg
U4-Safari-17-demo-increment.jpg
U4-Safari-18-demo-reset.jpg
U4-Safari-19-demo-reset-confirmed.jpg
U4-Safari-20-zh-dark.jpg
U4-Safari-21-zh-deep-404.jpg
U4-Safari-22-zh-404-query.jpg
U4-Safari-23-zh-404-pasted-query.jpg
U4-Safari-24-zh-404-destination.jpg
U4-Safari-25-settings-restored.jpg
U4-Safari-26-window-restored.jpg
```

另打开 `U4-iPhone-connection-state.jpg`、`U4-iPhone-timeout-state.jpg`，并读取 timeout AX 与设备接入原文；两图只作环境证据。

本轮补强了 Safari 搜索选择与导航、原生 200% 局部布局、鼠标重试恢复、演示状态及中文 404 搜索的证据；尚不足以形成 U4 完整验收通过结论。
