U3 在已见状态中未发现有冻结目标依据的产品差距，评分 **10.00/10**。下列未入画或未执行部分仍保留证据边界，不计为通过。

### 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | **3.00**：桌面对话框、差异表及 390 单列选择器、逐字段比较结构和内部滚动符合 G2、X6；已见底部动作没有横向溢出，达到本维度上限。 | **3.00**：中性面板、文字层级、等宽代码、边框、选中态和错误态符合 G3、X6；补充读取原图像素确认错误色 `#b42318`、选中背景 `#eff8ff`、选中文字 `#175cd3`，未发现可扣分偏差，达到本维度上限。 | **3.00**：已见候选版本与两模式差异、诊断、未知版本错误、默认 v2、v1 具体损失、两种 CSS profile、四种颜色记法、Rust 接入提示、桌面复制反馈和键盘焦点均符合对应规范；达到已见状态的本维度上限。 | **1.00**：逐处检查已见标签、文案、数值、换行、图标和代码排版，未发现有规范依据的剩余细节差异，达到本维度上限。 | **10.00 / 10** |

失分合计 **0.00**；未关闭产品差距的可挽回分合计 **0.00**。

### 2. 上一轮差距复核

| 上一轮项目 | 本轮状态 | 依据 |
| --- | --- | --- |
| R6 未关闭差距 | 无，不适用 | R6 独立结论明确没有未关闭差距，因此没有需要沿用的旧编号。 |
| R6 提及的差异表表头残字已消除 | 维持已解决 | 本轮桌面审阅首尾、底部帧的表头清晰；普通滚动造成的表体边缘裁切没有侵入表头文字。 |

### 3. 可执行产品差距

**无。** 本轮没有需要重做布局、替换组件或修改视觉 token 的差距，也没有新增扣分编号。

### 4. 建议（不扣分）

补齐下面三处具体截图覆盖即可；它们是采集建议，不能据此认定产品存在缺陷。

### 5. 看不出的矩阵格子

| U3 目标格子 | 当前证据与限制 | 可执行补采 |
| --- | --- | --- |
| 复制反馈 × 390×844 × light | `U3-export-copied-390-light.png` 可见 Copy code、Select text、Download，但没有复制成功文字；日志记录了授权后点击 Copy，不能替代结果反馈入画。 | 复制后将反馈所在区域滚入对话框可视区，连同复制按钮拍摄。 |
| 兼容 v1 导出 × 390×844 × light 的代码与动作区 | `U3-export-v1-390-light.png` 完整显示损失清单，但代码与操作区位于下缘之外，无法由该帧确认这一格式下的底部排列。 | 保持 v1 格式，滚至代码末端及 Copy/Download 区再拍一帧。 |
| 导入候选审阅 × 390×844 × light 的部分配置行 | 首帧和中段帧显示版本、id、名称；末段显示两模式 base/override、诊断和 Apply/Cancel。Appearance 至 Scaling 等中间配置行没有完整入画。 | 将上述配置行滚入视野补拍，不需要更换候选数据。 |

所有原图均清晰，没有因模糊而无法判断的图片。

**矩阵外或本轮记录未执行的行为边界：**

- 没有点击 Apply 后替换主题、取消后核查历史的结果证据。
- 候选展示了 Changed/Unchanged；Added/Removed 的具体呈现未覆盖。
- 语法错误、超限、不支持颜色输入、复制失败及实际下载结果未覆盖。
- 键盘记录证明了聚焦、打开列表和移动高亮；没有完整提交选项及关闭后焦点返回的结果帧。
- 桌面复制成功文字已见；剪贴板内容本身没有独立读取记录。

这些项目不补造通过结论，也不作为本轮产品扣分项。

### 6. 总评

本次只评 U3，无其他单元可比较。U3 已见状态与规范一致，桌面和窄屏均不需要结构性改变；剩余工作是补齐上述证据覆盖。

### 实际开图记录

工具：**`tools.view_image`，通过 `functions.exec` 调用，使用 `detail: "original"`**。共实际打开 **37 张独立图片**，其中本轮 34 张全部打开，前轮按需对照 3 张。

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/`

```text
U3-import-review-1440-light.png
U3-import-review-end-1440-light.png
U3-import-error-1440-light.png
U3-export-json-1440-light.png
U3-export-v1-1440-light.png
U3-export-css-1440-light.png
U3-export-css-profile-open-1440-light.png
U3-export-css-notation-open-1440-light.png
U3-export-rust-1440-light.png
U3-export-copied-1440-light.png
U3-export-rust-end-1440-light.png
U3-import-review-390-light.png
U3-import-review-end-390-light.png
U3-export-json-390-light.png
U3-import-review-bottom-1440-light.png
U3-import-review-bottom-390-light.png
U3-import-review-mid-390-light.png
U3-export-rust-mid-1440-light.png
U3-export-kind-keyboard-focus-1440-light.png
U3-export-kind-keyboard-open-1440-light.png
U3-export-profile-keyboard-focus-1440-light.png
U3-export-color-format-keyboard-focus-1440-light.png
U3-export-next-keyboard-focus-1440-light.png
U3-import-kind-keyboard-focus-1440-light.png
U3-import-kind-keyboard-open-1440-light.png
U3-export-v1-390-light.png
U3-export-css-390-light.png
U3-export-css-profile-open-390-light.png
U3-export-css-notation-open-390-light.png
U3-export-rust-390-light.png
U3-export-rust-mid-390-light.png
U3-export-rust-end-390-light.png
U3-export-copied-390-light.png
U3-import-error-390-light.png
```

共同绝对目录：

`/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/`

```text
U3-import-error-1440-light.png
U3-export-css-profile-open-1440-light.png
U3-export-kind-keyboard-focus-1440-light.png
```
