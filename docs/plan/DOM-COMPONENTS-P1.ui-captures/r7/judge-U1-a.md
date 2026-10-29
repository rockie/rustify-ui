**U1 本轮评分：10.0/10。UI-217 已解决；未发现新的、能由本轮原图支持的扣分差距。**评分仅覆盖已展示的状态，不把未拍到的行为视为通过。

已读取冻结目标包及完整变更记录、U1 采集清单、操作记录、上一轮结论；实际逐张打开本轮 **62 张**原图，并对照 UI-217 对应的 **3 张**前轮原图。

### 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** |

- **布局 3.0，已达上限**：桌面双栏、768px 工具栏换行、390px 编辑／预览切换与返回编辑、200% 内容重排均符合 G1/G2/G6；补采帧能看到放大后的导出动作和来源区末尾，没有发现影响关键动作可达性的遮挡、溢出或截断。
- **视觉 3.0，已达上限**：深色作者预览没有染黑编辑壳；壳层的文字层级、细分隔线、蓝色选中／焦点处理、扁平字段标签页符合 G3/G4，未见应扣分的可见偏差。
- **交互与状态 3.0，已达上限**：预设搜索、品牌色触发器、展开列表活动项、模式按钮及字段标签页均有可辨认焦点；Pink 起点的连续方向键、确认与 Escape 后状态可读，原 UI-217 的可见反馈缺失已消除；覆盖说明、auto 解析结果及预览模式语义符合 C2–C4。
- **细节 1.0，已达上限**：按规范模式比较，已展示的中英文配置文案、覆盖计数、状态标识、对齐和控件接缝没有发现另外需要扣分的差异；主题名、token 和作者数据按比较口径保留。

### 2. 上一轮差距复核

| 编号 | 上轮差距 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| **UI-217** | 已配置 Pink 后，品牌色展开列表缺少可见键盘活动项 | **已解决** | R7 的 `U1-matrix-keyboard-03-768-light.png`、`U1-matrix-keyboard-03-390-light.png`、`U1-matrix2-keyboard-03-1440-dark.png` 均显示 **Pink 的独立勾选标记和 Plum 的蓝色活动行**，而对应 R6 原图只有从 Custom 开始的列表，没有可见活动项。补采连续方向键帧进一步显示 Iris／Indigo 活动行保持可见；确认后触发器更新为相应值，Escape 后保留已确认值并显示触发器焦点。 |

UI-217 原有 **0.5 分**失分已恢复。本轮没有发现同一问题回归。

### 3. 未关闭差距清单

**无。**

可挽回分合计 **0.0 = 10 − 10.0**。没有需要沿用或新增的扣分条目，也没有需要推倒重做的布局或组件。

### 4. 建议，不扣分

调整补采的等待条件：拍摄 `configured-keyboard-open` 前，等待列表实际可见。当前 1440/dark 和 390/light 的这两张图片仍是关闭且聚焦的触发器，文件名容易被误读为已经截获展开状态。后续 `down`、`down5` 帧已提供有效的展开列表证据，因此不因此扣产品分。

### 5. 看不出的格子与证据边界

| 对应目标矩阵位置 | 图片 | 看不出什么，以及补充证据 |
| --- | --- | --- |
| U1／1440×900／dark／已配置品牌色的键盘选择器 | `U1-configured-keyboard-open-1440-dark.png` | 图片只显示聚焦触发器，不能证明记录中的这一次 **Enter 后立即展开**；随后的 `down`、`down5` 帧能证明列表展开后存在可见活动项。 |
| U1／390×844 编辑页／light／已配置品牌色的键盘选择器 | `U1-configured-keyboard-open-390-light.png` | 同上：没有拍到这一瞬间的展开列表；后续列表活动、确认和 Escape 帧可读。 |
| U1／200%／720×900／来源区重排 | `U1-zoom200-source-720-light.png` | 首帧没有展示完整来源块，单凭此图不能判定下半部是否可达；`…source-720-light-a2.png` 和 `…source-end-720-light.png` 补足来源正文及底部解除按钮。 |
| U1／200%／720×900／导出主路径 | `U1-zoom200-export-720-light.png` | 首帧看不到底部复制、选择文本与下载动作；`U1-zoom200-export-end-720-light.png` 已补足。 |

目标矩阵的主要界面状态均有可用帧；上述两处 **Enter 单键后的即时展开状态**仍不能独立确认，不将其写成通过或失败。

**矩阵外未测内容单列如下，不推断产品缺陷：**分隔条实际拖拽与宽度限制、基础配置全部选项的完整遍历、所有悬停／按下瞬间、持久化真实性及作者数据是否发生内部变化，不由本组静帧独立证明；字体文件与 CSS 数值也没有作机器级核验。人工读屏、OS IME 不在冻结目标包范围内。

### 6. 总评

本次只评 U1，不作跨单元最低分比较。U1 可见界面已达到冻结目标的满分锚点；上轮唯一未关闭问题 UI-217 的活动项可见性已有跨视口证据支持关闭。**不需要结构性改变。**

### 实际开图记录

工具：`functions.exec` 内逐张调用 **`tools.view_image({ detail: "original" })`**。

以下文件均实际开图。绝对路径由所列共同目录与文件名直接组成。

共同目录：`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

```text
U1-default-1440-light.png
U1-configured-1440-light.png
U1-configured-1440-dark.png
U1-top-1440-dark.png
U1-zh-top-1440-light.png
U1-zh-configured-1440-light.png
U1-default-768-light.png
U1-configured-768-light.png
U1-edit-390-light.png
U1-configured-390-light.png
U1-preview-390-light.png
U1-zoom200-720-light.png
U1-zoom200-export-720-light.png
U1-zoom200-colors-720-light.png
U1-zoom200-source-720-light.png
U1-keyboard-01-1440-light.png
U1-keyboard-02-1440-light.png
U1-keyboard-03-1440-light.png
U1-keyboard-04-1440-light.png
U1-zoom200-export-end-720-light.png
U1-field-tabs-1440-light.png
U1-field-tabs-focus-1440-light.png
U1-configured-lower-1440-light.png
U1-zoom200-source-720-light-a2.png
U1-zoom200-source-end-720-light.png
U1-matrix-zh-top-768-light.png
U1-matrix-zh-configured-768-light.png
U1-matrix-keyboard-01-768-light.png
U1-matrix-keyboard-02-768-light.png
U1-matrix-keyboard-03-768-light.png
U1-matrix-keyboard-04-768-light.png
U1-matrix-zh-top-390-light.png
U1-matrix-zh-configured-390-light.png
U1-matrix-keyboard-01-390-light.png
U1-matrix-keyboard-02-390-light.png
U1-matrix-keyboard-03-390-light.png
U1-matrix-keyboard-04-390-light.png
U1-matrix-default-1440-dark.png
U1-matrix-zh-top-1440-dark.png
U1-matrix-zh-configured-1440-dark.png
U1-matrix2-zh-top-1440-dark.png
U1-matrix2-zh-configured-1440-dark.png
U1-matrix2-keyboard-01-1440-dark.png
U1-matrix2-keyboard-02-1440-dark.png
U1-matrix2-keyboard-03-1440-dark.png
U1-matrix2-keyboard-04-1440-dark.png
U1-matrix3-zh-preview-390-light.png
U1-configured-keyboard-open-1440-dark.png
U1-configured-keyboard-down-1440-dark.png
U1-configured-keyboard-down5-1440-dark.png
U1-configured-keyboard-selected-1440-dark.png
U1-configured-keyboard-escape-1440-dark.png
U1-configured-keyboard-open-768-light.png
U1-configured-keyboard-down-768-light.png
U1-configured-keyboard-down5-768-light.png
U1-configured-keyboard-selected-768-light.png
U1-configured-keyboard-escape-768-light.png
U1-configured-keyboard-open-390-light.png
U1-configured-keyboard-down-390-light.png
U1-configured-keyboard-down5-390-light.png
U1-configured-keyboard-selected-390-light.png
U1-configured-keyboard-escape-390-light.png
```

共同目录：`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`

```text
U1-matrix-keyboard-03-768-light.png
U1-matrix-keyboard-03-390-light.png
U1-matrix2-keyboard-03-1440-dark.png
```
