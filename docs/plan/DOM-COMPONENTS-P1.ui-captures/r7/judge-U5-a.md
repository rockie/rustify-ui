## 1. 评分

仅评 U5；依据冻结目标的规范模式比较，不按 Radix 页面排列或演示数据扣分。

| 单元 | 布局 / 3 | 视觉 / 3 | 交互与状态 / 3 | 细节 / 1 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U5 Studio 预览 | 3.00 | 2.80 | 3.00 | 1.00 | **9.80 / 10** |

- **布局 3.00**：桌面 DOM/GPU 并排、390px 上下堆叠、局部边界、菜单、对话框与拾取面板在已拍范围内均可容纳；未见需要扣分的溢出、遮挡或对齐问题，达到本维度上限。
- **视觉 2.80**：根与相反边界的主题、创建处浮层、浅色编辑壳和浅蓝拾取面板均符合规范，但未标注部件被点击后的拾取框仍是灰色虚线，与 P2 要求的固定蓝色 outline 不一致，不能给满分。
- **交互与状态 3.00**：记录与截图支持根主题切换、相反局部边界、菜单和对话框展开、独立主题对比、已标注与未标注拾取，以及窄屏进入来源编辑后返回预览；已拍范围未发现操作失败，达到本维度上限。
- **细节 1.00**：除已单独归入视觉维度的拾取框问题外，已拍范围未发现额外的文案、按钮几何、图标对齐、换行或截断差异，达到本维度上限。

未关闭差距可挽回分合计 **0.20**，等于 `10 − 9.80`。未拍状态不算通过，也不算失败。

## 2. 上一轮差距复核

| 上一轮条目 | 本轮结论 |
| --- | --- |
| R6 未关闭差距：无 | 没有可沿用编号的未关闭差距。 |
| R6 总体结论：10.00 | 本轮新增识别「新-1」。对照 R6 未标注拾取原图，同样的灰色虚线已存在，因此不是 R7 界面回归。 |
| R6 所述桌面并排、窄屏堆叠与拾取面板表现 | 本轮相关原图未见变差；对照已打开的 R6 桌面 GPU 对比、390px 拾取及 GPU 首尾帧，排布维持一致。 |

## 3. 未关闭差距

### 新-1 · 未标注部件的拾取框没有使用固定蓝色

- **单元与位置**：U5；`U5-inspector-undeclared-1440-light.png`、`U5-inspector-undeclared-panel-1440-light.png`，Provider & overlay checks 区右上角的 **Create · 0** 按钮外缘。悬停帧 `U5-inspector-undeclared-hover-1440-light.png` 也可看到同样处理。
- **维度 / 严重度**：视觉 / **P2 一般**。
- **现状 → 期望**：点击未标注部件后，目标部件用灰色虚线框表示；底部正确显示 “Not annotated · this part has no declared token mapping.”。期望保留这条真实的未标注说明，同时让被拾取部件的选中矩形使用固定蓝色 outline。依据为目标包 **P2**：“选中矩形为固定蓝色 outline，信息面板为浅蓝并带 token 按钮。”其中没有对未标注部件豁免选中框颜色。
- **具体修法**：修改 Inspector 的未映射部件选中标识，使其与已映射部件复用同一个固定蓝色 outline 规则；不要让这条选中标识继承作者的 border/foreground 颜色。未标注信息继续放在面板文字中表达，不需要编造 token 或新增完整 Inspector 功能。
- **验证**：开启 Inspect，点击 Create · 0 后拍摄；应仍显示未标注说明，按钮外缘改为与 Create profile 拾取态一致的固定蓝色标识。
- **可挽回分：0.20**。
- **历史判断**：R6 对应悬停与点击原图中已存在；这是本轮新识别的既有偏差，不标为回归。

## 4. 建议（不扣分）

没有新增产品功能建议。后续采集可以补上固定宽度选择、浮层拾取及真实键盘操作帧，使以下未核实状态可直接判断；这些证据缺口本轮均未作为缺陷扣分。

## 5. 看不出的格子

所有实际打开的图片均清晰；以下限制来自状态未展示或视口裁切：

| 图片或材料 | 看不出的内容 |
| --- | --- |
| 所有出现 `Responsive` 的工具栏帧，包括 `U5-dom-1440-light.png`、`U5-matrix-dom-390-light.png` | 没有选择 1024/768/390 固定内容宽度的帧，不能核实 P5 的各档模拟宽度。 |
| 所有 menu/dialog 帧 | 能判断浮层的主题和几何；没有浮层内拾取帧，不能核实 P2 的“浮层同样可检查”。也没有浮层保持打开时修改创建处主题的前后帧。 |
| `U5-inspector-1440-light.png`、`U5-inspector-picked-1440-light.png`、深色和390px对应帧 | 能判断部件映射、来源呈现及来源编辑入口；不能从静态图确认全程键盘拾取、Esc 退出、Tab 顺序和退出后的焦点恢复。 |
| 菜单、对话框的焦点记录 | 记录说明了焦点对象，但未展示真实键盘遍历和清晰的键盘焦点帧，不能据此确认全部键盘行为。 |
| DOM 表单与 Component samples 帧 | 没有普通模式提交表单、执行示例动作后的完整前后帧，不能确认成功、错误与重置反馈的全部表现；Inspect 下的点击不能替代普通操作验收。 |
| `U5-gpu-compare-1440-light.png`、`U5-matrix-gpu-compare-1440-dark.png` | 仅展示 GPU 上部内容与部分色块，不能判断未进入视口的下部内容。 |
| `U5-matrix-gpu-compare-end-390-light.png` | 展示了窄屏 GPU 区及部分 token 列表，但底部仍被视口截断；不能据此判断余下列表。正常纵向裁切不记为布局缺陷。 |
| 所有 GPU 帧 | 未展示失败、恢复及中文状态，不能核实 P4 的 GPU 失败反馈与语言同步。 |

## 6. 总评

本次只评 U5，得分 **9.80/10**。已拍界面满足主要布局、主题隔离、浮层和拾取入口要求，无需结构性重做。唯一扣分是未标注部件的拾取框使用灰色虚线，偏离 P2 的固定蓝色标识要求。

## 实际开图清单

工具：**`tools.view_image`**，每张均使用 `detail: "original"` 并实际回显查看。未使用尺寸检查替代看图。U5 为规范模式，本次没有打开其他单元的目标 PNG。

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

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

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`

实际打开 **6 张**：

```text
U5-inspector-undeclared-hover-1440-light.png
U5-inspector-undeclared-1440-light.png
U5-matrix-inspector-390-light.png
U5-matrix2-gpu-compare-390-light.png
U5-matrix2-gpu-compare-end-390-light.png
U5-gpu-compare-1440-light.png
```
