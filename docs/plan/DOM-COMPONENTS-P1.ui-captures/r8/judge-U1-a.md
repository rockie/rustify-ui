**U1 本轮评分：10.0/10。UI-217 维持已解决，未发现新的、由本轮原图支持的扣分差距。**

本次只评 U1，按冻结目标的规范模式比较。已读完整目标包及变更记录、采集清单、操作日志和上一轮独立结论；实际打开本轮全部 **62 张**原图，并对照 **5 张**上轮原图。评分限于已展示的界面状态。

### 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** |

- **布局 3.0，已达上限**：1440px 双栏结构、768px 工具栏换行、390px 编辑／预览切换及返回编辑、200% 文字重排均符合 G1/G2/G6；补采帧展示了导出动作区和来源块末尾，没有发现遮住关键操作的溢出或重叠。
- **视觉 3.0，已达上限**：作者预览切深色后，编辑壳仍保持中性浅色；文字层级、细分隔线、蓝色选中与焦点、扁平字段标签页符合 G3/G4，未见可支持扣分的外观偏差。
- **交互与状态 3.0，已达上限**：搜索框、基础配置触发器、列表活动项、模式按钮和字段标签页均有可辨认焦点；Custom、Overridden、分模式覆盖数量和 Auto 解析结果清楚，作者外观与预览模式的说明符合 C2–C4。
- **细节 1.0，已达上限**：已展示的中英文基础配置文案、状态标识、计数、图标与文字对齐没有发现应扣分差异；主题名、token 和作者数据按规范模式的比较口径处理，不作翻译或内容一致性要求。

### 2. 上一轮差距复核

| 编号 | 上轮差距 | 本轮状态 | 复核依据 |
| --- | --- | --- | --- |
| **UI-217** | 已配置 Pink 后，品牌色列表缺少可见键盘活动项 | **已解决，维持关闭** | 本轮 `U1-matrix-keyboard-03-768-light.png`、`U1-matrix-keyboard-03-390-light.png`、`U1-matrix2-keyboard-03-1440-dark.png` 均能区分 **Pink 的勾选标识**与 **Plum 的蓝色活动行**，与实际打开的对应 R7 原图一致。补充连续操作帧还展示 Iris／Indigo 活动行、确认后的触发器值，以及 Escape 返回后的触发器焦点，没有看到同一问题回归。 |

上一轮没有其他未关闭差距。

### 3. 未关闭差距清单

**无。**

可挽回分合计 **0.0 = 10 − 10.0**。没有需要沿用或新增编号的扣分条目。

### 4. 建议，不扣分

拍摄 `configured-keyboard-open` 时，应等待列表实际可见再截图。本轮 `U1-configured-keyboard-open-1440-dark.png` 仍显示关闭且聚焦的 Pink 触发器，名称容易让人误认已经展示展开状态。后续列表帧及同矩阵的 `matrix2-keyboard-03` 已提供有效证据，此处属于采集建议。

### 5. 看不出的格子与证据边界

本轮没有发现冻结矩阵中**整格缺失**的必需界面状态。以下区别于整格缺证，不据此推断产品失败：

| 分类 | 图片或范围 | 看不出什么及已有补充 |
| --- | --- | --- |
| 必需键盘格内的瞬时证据不足 | `U1-configured-keyboard-open-1440-dark.png` | 只看到关闭且聚焦的触发器，不能证明这一次 Enter 后立即展开；`…down…`、`…down5…` 和 `U1-matrix2-keyboard-03-1440-dark.png` 已展示展开列表与活动项，因此整个键盘格有可用证据。 |
| 连续操作的精确时序 | 1440/dark 的 `configured-keyboard-open`、`down`、`down5` 帧 | `down` 帧仍显示 Pink 活动行，`down5` 显示 Iris；不能凭文件名认定每一次方向键与截图完全同步，也不能据此推断丢键。 |
| 格内正常纵向裁切 | `U1-zoom200-export-720-light.png` | 首帧看不到底部动作；`U1-zoom200-export-end-720-light.png` 已展示 Copy code、Select text、Download。 |
| 格内正常纵向裁切 | `U1-zoom200-source-720-light.png` | 首帧没有完整来源正文；`…source-720-light-a2.png` 和 `…source-end-720-light.png` 已补足路径、解除结果与底部按钮。 |

**矩阵之外的扩展走查**包括分隔条实际拖拽及限宽、全部基础配置选项逐项遍历、所有悬停／按下瞬间、持久化真实性与内部作者数据变化；这些没有被本组静帧独立证明，不写成通过或失败。字体文件和 CSS 数值也未作机器核验。人工读屏与 OS IME 不在冻结目标范围内。

### 6. 总评

本次仅有 U1，不作跨单元最低分比较。已展示界面达到冻结规范的满分锚点：工作台结构稳定，基础配置与覆盖语义清楚，窄屏和放大状态的关键动作可见，键盘活动项与焦点反馈完整。**没有发现需要结构性改变的区域。**

### 实际开图记录

工具：`functions.exec` 内调用 **`tools.view_image({ detail: "original" })`**，逐张显示原图。

以下共同绝对目录前缀与逐个文件名组成实际打开的完整路径。

本轮目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/`

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

上轮对照目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

```text
U1-matrix-keyboard-03-768-light.png
U1-matrix-keyboard-03-390-light.png
U1-matrix2-keyboard-03-1440-dark.png
U1-configured-keyboard-escape-390-light.png
U1-configured-keyboard-selected-390-light.png
```
