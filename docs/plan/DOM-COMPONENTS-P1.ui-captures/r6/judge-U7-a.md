# R6 独立 UI/UX 评审 · U7 布局与排版

仅评 U7。已读取白名单内的冻结目标包及完整变更记录、本轮采集清单与操作记录、R5 独立评审，并实际打开当前、目标及所需旧图。未读取代码、完整报告、机器检查、修复记录或其他单元，未修改文件。

按 U7 比较口径，页面排列、示例文案与图片、字族、深浅色差异，以及已接受的参考选阶／旧 token 对低对比不计差距。

## 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 3.0 | 2.7 | 3.0 | 1.0 | **9.7 / 10** |

- **布局 3.0**：所拍 Container 四档宽度与三种对齐、Inset 贴边、窄屏换行和排列均符合比较口径，未发现应扣分的遮挡或内容截断，达到本维度上限。
- **视觉 2.7**：字级梯度、字重、Code 的填充／描边、Blockquote 以及修正后的 Kbd 轮廓基本对应目标；Code ghost 的强调色文字仍与参考中的中性前景处理不同，不能给 3 分。
- **交互与状态 3.0**：操作记录与 110%／95% 帧对应，非法属性帧显示属性、拒收值和回退值，其他内容继续可见，达到已采集状态的本维度上限。
- **细节 1.0**：在本轮比较范围内，未另发现需要独立扣分的微小差异；Code 用色差距统一计入视觉，没有重复扣分。

未关闭差距的可挽回分合计 **0.3**，等于 **10 − 9.7**。

## 2. 上一轮差距复核表

| 编号 | R6 状态 | 复核结果 |
| --- | --- | --- |
| UI-189 · Reset 的 input、textarea 虚线标记越过内容右沿 | **已解决，维持关闭** | 本轮 1280、390 的 Reset 分段图及整段图中，虚线右缘均收在内容边界内，未见重新越界。 |
| UI-211 · 240px 显式宽度样例正文贴左边框 | **已解决，维持关闭** | 默认、110%、95%、390 及非法属性特写中，正文与左边框保持可辨认的内边距，未见贴边回归。 |
| R5 新-1 · Kbd 大字级圆角未随尺寸形成目标中的键帽轮廓 | **已解决** | 与实际打开的 R5 同名图相比，R6 classic／soft 的 size 7–9 均已形成递增圆角，size 9 不再是近方形；外框、内侧高光与 soft 色块的轮廓均对应目标的大字级键帽形状。 |

下述 Code ghost 问题在实际打开的 R5 Code 图中已经存在，属于**新增发现，不是回归**。为避免与 R5 的“新-1”混淆，续用编号“新-2”。

## 3. 差距清单

### 新-2 · Code ghost 的 color／high-contrast 文字仍带强调色

- **单元与截图位置**：U7；`U7-type-code-matrix-1280-light.png` 的 ghost 行前两列，及 `U7-type-code-matrix-390-light.png` 中 ghost 组的 `color=indigo`、`indigo · high_contrast`。
- **维度**：视觉。
- **严重度**：P2 一般。
- **现状 → 期望**：
  - 当前 ghost 的 color 样例呈明显靛蓝色，high-contrast 样例呈深靛蓝色，与同组 gray 的中性文字形成色相区别。
  - 目标 `radix-sink-code.png` 表格的 ghost 行中，color 与 high-contrast 两列均采用接近正文的中性前景；强调色处理保留在上方 solid／soft／outline 变体中。
  - 期望依据为目标图 **Code 表格 ghost 行前两列**、U7 比较口径中的 **Code 外观**，以及 L1 的正确语义外观要求。这里比较的是 ghost 的前景处理，不对已接受的低对比情况另行扣分。
- **具体修法**：
  - 调整 **Code 的 ghost 变体前景分支**：使当前强调色 ghost 样例使用有效主题的中性 foreground，high-contrast 样例也与参考中的中性高对比前景对应。
  - 保留 ghost 的无填充、无描边外观；单独复核显式 gray 分支，避免把其他三种变体一起改色。
  - 修后并排核对 1280、390 的四列矩阵，并确认下方 ghost 字级阶梯仍使用正文前景。
- **证据定位**：目标图 ghost 行约在本地图像 `y=309`，当前 1280 分段图对应行约在 `y=294`；当前前两列分别位于约 `x=112`、`x=388`。
- **可挽回分：0.3**。

## 4. 建议（不扣分）

无。本轮未加入目标包之外的设计要求。

## 5. 看不出的格子

- **390px 的 110%／95% 与非法属性状态**：没有对应截图，不能判断其窄屏表现。
- **Reset 的实际操作与结构语义**：截图显示 `clicks: 0`，记录没有点击 Reset 样例按钮；截图不能证明原生语义、无包裹层或 scope 隔离，此处不记通过或失败。
- **非法属性恢复结果**：已拍到拒收与回退反馈，未拍到点击 Restore prop 后的结果。
- **其他响应式断点**：本轮只有 390、1280 两种宽度，不能外推其他断点。
- **窄屏 Heading 分段图的右缘**：size 9 wrap 的文字接近分段截图右缘；整段原图提供了更完整的边界，不能仅凭分段截图边缘判定产品发生截断。

## 6. 总评

本次只评 U7，得分 **9.7**。上一轮唯一未关闭的 Kbd 圆角问题已解决；当前剩余差距集中在 **Code ghost 的前景用色**。无需推倒整体布局或更换组件，调整该变体的颜色分支即可。

## 实际开图工具与路径清单

实际开图工具：**`tools.view_image`，通过 `functions.exec` 的 `image()` 展示，使用 `detail: "original"`**。下列共 **38 张**，全部实际打开；未把文件清单、尺寸读取或失败的辅助裁切尝试计入开图。

**R6 当前原图：26 张**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-fixture-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-invalid-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-invalid-detail-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-container-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-container-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-inset-demonstration-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-inset-demonstration-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-reset-comparison-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-reset-comparison-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale110-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale110-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale95-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale95-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-blockquote-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-blockquote-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-heading-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-heading-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-kbd-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-kbd-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-text-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-text-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-wrap-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-wrap-matrix-390-light.png
```

**目标图：9 张**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-heading.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-text.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-code.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-kbd.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-blockquote.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-text-wrap.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-sink-container.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-inset.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-target/radix-reset.png
```

**R5 对照原图：3 张**

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-type-code-matrix-1280-light.png
```
