## 1. 评分

仅评 U5，按冻结目标的规范模式比较；不按 Radix 页面排列、演示数据或已接受的参考选阶／旧 token 对比度差异扣分。

| 单元 | 布局 / 3 | 视觉 / 3 | 交互与状态 / 3 | 细节 / 1 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U5 Studio 预览 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00 / 10** |

- **布局 3.00**：已拍桌面 DOM/GPU 并排、390px 上下堆叠、局部边界、浮层及拾取面板均可容纳，未见非正常溢出、遮挡或对齐偏差，达到本维度上限。
- **视觉 3.00**：根与相反边界、创建处菜单和对话框的主题一致，编辑工具保持浅色，已标注与未标注部件均显示固定蓝色拾取框，未发现可扣分偏差，达到本维度上限。
- **交互与状态 3.00**：截图和操作记录支持主题与边界切换、菜单和对话框展开、拾取定位来源及 renderer 切换，已展示路径未发现操作失败，达到本维度上限；未展示行为另列。
- **细节 1.00**：逐张查看的范围内未发现额外文案、图标对齐、按钮几何、换行或省略处理偏差，达到本维度上限。

未关闭差距可挽回分合计 **0.00**，等于 `10 − 10.00`。未展示状态不算通过，也不算失败。

## 2. 上一轮差距复核

| 编号 | 状态 | 复核依据 |
| --- | --- | --- |
| UI-218：未标注部件的拾取框没有使用固定蓝色 | **已解决** | 对照 R7 的未标注悬停、点击及面板三帧，Create · 0 外缘原为灰色虚线；R8 对应三帧已显示蓝色实线框。面板仍明确显示 “Not annotated · this part has no declared token mapping.”，没有编造 token，符合 P2。 |

## 3. 未关闭差距

**无。** 本轮已展示范围内没有新增扣分条目，也没有仍需沿用的旧差距。

## 4. 建议（不扣分）

没有新增产品功能或视觉修改建议。

后续可补采固定内容宽度、浮层内拾取、浮层打开期间切换创建处主题及真实键盘操作帧，使下列证据空白能够直接判断。这些采集建议不计为产品缺陷。

## 5. 看不出的格子

所有实际打开的图片均清晰。以下限制来自状态未展示或正常视口裁切。

| 图片或材料 | 看不出的内容 |
| --- | --- |
| `U5-dom-1440-light.png`、`U5-dom-1440-dark.png`、各 `matrix-dom` 与 GPU 工具栏帧 | 宽度选择器均显示 Responsive，没有选择 1024／768／390 固定内容宽度的帧，不能核实 P5 各档模拟宽度。 |
| 所有 menu／dialog 帧 | 能判断浮层在根主题及相反局部主题下的外观和几何；没有浮层内拾取帧，也没有浮层保持打开时切换创建处主题的前后帧。 |
| 所有 Inspector 帧及操作记录 | 能判断蓝色标识、映射、未标注说明及来源入口；不能确认完整键盘拾取、Esc 退出、Tab 顺序和退出后的焦点恢复。 |
| 菜单、对话框及工具栏的焦点记录 | 记录中的焦点对象不能替代真实键盘遍历和清晰的键盘焦点帧，不能确认全部键盘行为。 |
| DOM 表单、Component samples 及其操作记录 | 未展示普通模式下提交表单、执行示例动作后的完整反馈；Inspect 下点击不能核实普通操作的成功、错误、重置及其他交互状态。 |
| `U5-matrix-inspector-1440-dark.png`、`U5-matrix-inspector-picked-1440-dark.png` | 拾取面板遮住了 Create profile 按钮下缘，不能从这两帧检查该按钮完整的外框几何；其他已拍拾取帧可见蓝色标识。 |
| `U5-gpu-compare-1440-light.png`、`U5-matrix-gpu-compare-1440-dark.png` | GPU 下部内容未进入视口，不能判断未展示部分。 |
| `U5-matrix-gpu-compare-end-390-light.png` | 展示了窄屏 GPU 区和部分 token 列表，列表底部仍被视口裁切，不能判断余下内容；正常纵向裁切不记为布局缺陷。 |
| 所有 GPU 帧 | 未展示 GPU 失败、恢复、中文状态及相应操作，不能核实 P4 的这些状态与语言同步。 |

## 6. 总评

本次唯一评审单元为 **U5，10.00/10**。上一轮唯一未关闭差距 UI-218 已解决：未标注部件使用蓝色实线拾取框，同时保留真实的未标注说明。

已拍范围未发现需要结构性改变的区域。分数仅反映本轮截图和记录所展示的界面，不覆盖上表尚不能判断的状态。

## 实际开图工具与清单

工具：**`tools.view_image`**，每张均使用 `detail: "original"` 并实际回显查看。未以文件清单或尺寸统计代替开图。U5 为规范模式，未打开其他单元的目标 PNG。

本轮共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/`

实际打开 **39 张**：

```text
U5-dom-1440-light.png
U5-dom-1440-dark.png
U5-local-opposite-1440-light.png
U5-menu-1440-light.png
U5-dialog-1440-light.png
U5-compare-nested-1440-light.png
U5-gpu-compare-1440-light.png
U5-inspector-card-1440-light.png
U5-inspector-1440-light.png
U5-inspector-picked-1440-light.png
U5-inspector-undeclared-hover-1440-light.png
U5-inspector-undeclared-1440-light.png
U5-preview-390-light.png
U5-dialog-390-light.png
U5-dom-caller-1440-dark.png
U5-local-opposite-menu-1440-light.png
U5-local-opposite-dialog-1440-light.png
U5-inspector-undeclared-panel-1440-light.png
U5-matrix-dom-1440-dark.png
U5-matrix-local-opposite-1440-dark.png
U5-matrix-local-opposite-menu-1440-dark.png
U5-matrix-local-opposite-dialog-1440-dark.png
U5-matrix-menu-1440-dark.png
U5-matrix-dialog-1440-dark.png
U5-matrix-compare-nested-1440-dark.png
U5-matrix-inspector-1440-dark.png
U5-matrix-inspector-picked-1440-dark.png
U5-matrix-gpu-compare-1440-dark.png
U5-matrix-dom-390-light.png
U5-matrix-local-opposite-390-light.png
U5-matrix-local-opposite-menu-390-light.png
U5-matrix-local-opposite-dialog-390-light.png
U5-matrix-menu-390-light.png
U5-matrix-dialog-390-light.png
U5-matrix-compare-nested-390-light.png
U5-matrix-inspector-390-light.png
U5-matrix-inspector-picked-390-light.png
U5-matrix-gpu-compare-390-light.png
U5-matrix-gpu-compare-end-390-light.png
```

上一轮共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

为复核 UI-218，实际打开 **3 张**：

```text
U5-inspector-undeclared-hover-1440-light.png
U5-inspector-undeclared-1440-light.png
U5-inspector-undeclared-panel-1440-light.png
```
