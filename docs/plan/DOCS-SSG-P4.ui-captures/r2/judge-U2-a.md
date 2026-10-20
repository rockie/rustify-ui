U2 本轮评分为 **10.0/10**，仅适用于本轮截图与操作记录覆盖的状态。UI-02、UI-04 在 U2 已覆盖范围均已解决，未发现新的可扣分差距。

| 维度 | 分数 | 理由 |
|---|---:|---|
| 布局 | 3.0/3 | 顺序符合 S2；桌面面板约 576px，375px 视口面板约 343px、左右各约 16px，符合 S1。长列表滚动后输入、Close 和 Show more 仍完整可见，摘要正常换行，无可见溢出或遮挡。已观察范围与目标相符，给满分。 |
| 视觉 | 3.0/3 | 浅、深主题中的背景、文字层级、扁平边界、圆角、焦点环、选中与命中样式未见偏离 G1–G3、S1–S2 的可辨差异；Retry、Close、触发按钮获焦时轮廓清楚。已观察范围无扣分依据，给满分。 |
| 交互与状态 | 3.0/3 | 模块未返回即可输入；加载中最终查询得到对应结果；模块与索引错误均有明确提示和可达 Retry，恢复后保留查询并聚焦输入；键盘 Retry、Close、Esc 归焦及新结果出现后释放旧响应的记录与截图一致，符合 S3–S5。已观察范围无扣分依据，给满分。 |
| 细节 | 1.0/1 | 单条结果均显示“1 result”，长列表显示“98 results”；状态文案、路径分隔、命中与换行未见新的细节差异。已观察范围与目标相符，给满分。 |
| **总分** | **10.0/10** | **未关闭差距可挽回分合计 0，等于 10−10.0。** |

旧差距复核：

| 编号 | 复核状态 | 本轮位置与核对结果 |
|---|---|---|
| UI-02：窄屏 dialog 比规定宽度少 6px | **已解决（限 U2）** | 对比 R1/R2 的 `U2-loading-375-light.png`、`U2-supp-longlist-04-375-light.png`：原左右约 19px，现左右约 16px、外宽约 343px，符合 S1；本轮其他 375px 面板边界一致。长列表中的输入、关闭、更多仍可见，未见扩大面板后的回归。此结论不代替 U1 或 320px 的复核。 |
| UI-04：单条英文结果显示“1 results” | **已解决** | R2 的 module/index retry、reopen、loading result、keyboard retry、晚响应前后等单条结果图均为“1 result”；`U2-supp-longlist-04-375-light.png` 仍为“98 results”。已直接对比 R1 的窄屏 module retry、桌面 index retry、晚响应释放图，原单复数错误已消失，符合 S2/S3 与文案准确性口径。 |

所有未关闭差距：**无。** 未新增“新-1”等问题编号；没有可据规范扣分的 P0–P3 问题。

不扣分建议：无新增界面或结构修改建议。未拍状态适合补证，不据此要求改界面。

本轮可见覆盖与看不出的格子：

| 项目 | 可判断范围／限制 |
|---|---|
| 必需状态矩阵 | loading、module error、module retry success、index error、index retry success，在 1280×900 dark 与 375×812 light 均有截图和操作记录，没有缺格。 |
| 加载中打开与编辑 | 窄屏补采显示模块响应仍被扣留时输入已获焦；输入由 CustomOnlyToken 改成 ReachableCustomToken，释放后显示最终查询对应结果。 |
| 恢复与键盘焦点 | 两类故障的点击恢复均有记录；窄屏另有 Tab 至 Retry、Enter 恢复、Shift+Tab 至 Close、Enter 关闭并返回 Search 的实际记录及可见焦点帧。 |
| 关闭、重开与旧响应 | 桌面 Esc 关闭返回 Search，重开保留查询；补采 09 已先显示新查询结果，补采 10 才释放旧响应并等待，当前查询与结果保持一致。 |
| 仅由图像不能确认 | 精确颜色值和 WCAG 对比度比值、live 区实际朗读、焦点环的精确 CSS 厚度，以及动态视口变化过程；本评分不等于这些项目已独立测定。 |
| 本包没有拍到或记录 | Cmd/Ctrl+K 打开、IME 组合输入、完整正反向 Tab 循环、Show more 的键盘激活及新增后的选择保持、hover/按下瞬间、普通动态效果模式、错误态另改查询后再 Retry。保持未验证，既不外推符合，也不判失败。 |
| 环境边界 | 本包是 Chromium 的桌面与 CSS 窄视口；不能外推至实体手机软键盘、其他原生浏览器、原生缩放或辅助技术。 |

整体无需结构性改动。加载、错误、恢复的层级和可达操作保持稳定，两项旧问题修正后未见相关布局或状态回归。示例正文和无关排名未作内容或像素扣分；这里不作验收判定。

实际开图清单与工具轨迹：

- 只读 `DOCS-SSG-P4.ui-target.md`、R2 的 `U2-capture.md`、`U2-capture.json`、`U2-operations.md`、`previous-U2.md`，以及 R1 `judge-U2-b.md`。首次合并读取的 JSON 回显被截断，随后单独完整读取了该 JSON。
- 使用 `functions.exec → tools.view_image` 分六批逐张打开 **28 张**图：R2 全部 23 张，R1 针对性比较 5 张。没有以尺寸或 hash 代替开图。
- 未运行页面、未读代码或脚本、未读其他报告、未打开目标包来源链接、未修改文件。

| 序号 | R2 实际打开文件 |
|---:|---|
| 1 | `U2-loading-1280-dark.png` |
| 2 | `U2-closed-01-1280-dark.png` |
| 3 | `U2-reopen-02-1280-dark.png` |
| 4 | `U2-late-response-03-1280-dark.png` |
| 5 | `U2-module-error-1280-dark.png` |
| 6 | `U2-module-retry-1280-dark.png` |
| 7 | `U2-index-error-1280-dark.png` |
| 8 | `U2-index-retry-1280-dark.png` |
| 9 | `U2-loading-375-light.png` |
| 10 | `U2-module-error-375-light.png` |
| 11 | `U2-module-retry-375-light.png` |
| 12 | `U2-index-error-375-light.png` |
| 13 | `U2-index-retry-375-light.png` |
| 14 | `U2-supp-open-before-module-01-375-light.png` |
| 15 | `U2-supp-loading-edit-02-375-light.png` |
| 16 | `U2-supp-loading-result-03-375-light.png` |
| 17 | `U2-supp-longlist-04-375-light.png` |
| 18 | `U2-supp-retry-focus-05-375-light.png` |
| 19 | `U2-supp-keyboard-retry-06-375-light.png` |
| 20 | `U2-supp-close-focus-07-375-light.png` |
| 21 | `U2-supp-close-return-08-375-light.png` |
| 22 | `U2-supp-new-result-before-old-09-1280-dark.png` |
| 23 | `U2-supp-old-released-after-new-10-1280-dark.png` |

另已实际打开的 R1 同名比较图：

- `U2-loading-375-light.png`
- `U2-supp-longlist-04-375-light.png`
- `U2-module-retry-375-light.png`
- `U2-index-retry-1280-dark.png`
- `U2-supp-old-released-after-new-10-1280-dark.png`
