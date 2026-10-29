仅评 U7。已读取冻结目标包及完整变更记录、补采清单、操作记录、上一轮评审，并实际打开本轮全部 34 张原图、9 张目标图和 5 张上一轮原图。未读取代码、机器检查、其他评审或报告，未写文件。

按 U7 比较口径，夹具排列、示例文案、字族、深浅色差异及已接受的参考选阶／旧 token 对低对比不扣分。UI-115 的范围接受不改变本单元标准。

**1. 评分表**

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0 / 10** |

- **布局 3.0**：所拍 Container 四档宽度和三种对齐、Inset 四侧贴边、窄屏换行均符合目标；整段原图确认分段图边沿附近的长标题仍完整可读，未发现可扣分的遮挡或截断，已达本维度上限。
- **视觉 3.0**：字级梯度、Heading／Strong 粗体、Text 字重、Code 四种变体、Blockquote 竖线和 Kbd 键帽轮廓均与比较口径对应；上一轮 Kbd 大字级圆角差距已关闭，已达本维度上限。
- **交互与状态 3.0**：桌面与补采窄屏帧均显示 110%／95% 的尺度变化，显式尺寸样例保持稳定，非法属性诊断包含属性、拒收值和回退值且内容继续显示；在实际采集状态内未发现扣分项，已达本维度上限。
- **细节 1.0**：对照本轮分段图、整段边界和目标图，未发现比较范围内需要独立扣分的细微差异，已达本维度上限。

此分数限于实际采集的可见效果；未拍到的操作和结构语义不推定通过。未关闭差距可挽回分合计 **0.0**，等于 **10 − 10.0**。

**2. 上一轮差距复核表**

| 编号 | 本轮状态 | 复核结果 |
| --- | --- | --- |
| R5 新-1 · Kbd 大字级圆角未形成目标键帽轮廓 | **已解决** | R5 两个视口的 size 7–9 classic／soft 仍接近小圆角方块；R6 对应图中圆角已随尺寸递增，size 9 soft 和 classic 外框均呈现目标的大圆角轮廓。390px 的 110%／95% 补采图也维持该形状，没有出现圆角未同步缩放的可见问题。 |
| UI-189 · Reset 的 input、textarea 虚线标记越过内容右沿 | **已解决，维持关闭** | R6 的 1280、390 整段图和分段图中，虚线右沿保持在内容边界内；与已打开的 R5 同名分段图一致，未发现回归。 |
| UI-211 · 240px 显式宽度样例正文贴左边框 | **已解决，维持关闭** | R6 默认、110%、95% 和非法属性帧中，正文均与左边框保留可见间距；110% 下正常换行。与 R5 已修正的样例一致，未发现回归。 |

**3. 差距清单**

无仍未关闭的、能够由本轮截图和操作记录证明的差距。没有新增扣分项。

**4. 建议（不扣分）**

无额外设计建议。当前证据不支持为了继续提高分数而改动组件外观或重排布局。

**5. 看不出的格子**

| 材料／状态 | 无法判断的内容 |
| --- | --- |
| `U7-layout-reset-comparison-1280-light.png`、`U7-layout-reset-comparison-390-light.png`，以及各整段帧 | 均显示 `clicks: 0`，操作记录没有点击 Reset 样例按钮；不能判断其点击结果、键盘操作或焦点反馈。 |
| Reset 对比图 | 可以观察原生样式与 Reset 后外观的区别，但静态截图不能证明原生元素语义、无额外包裹层以及其他 scope 的隔离。 |
| `U7-invalid-detail-1280-light.png`、`U7-matrix-invalid-detail-390-light.png` | 拒收与回退反馈清楚可读，但没有点击 Restore prop 后的对应帧，不能判断恢复动作结果。 |
| Wrap／truncate 分段图 | 可以判断换行、省略号和可见裁切效果；不能由截图证明省略后的底层文本仍完整保留。 |
| 全部当前材料 | 未提供 520、768、1024、1640px 等其他断点的对应截图，不外推这些断点的排布结果。 |

上一轮“390px 缩放与非法属性状态看不出”的材料缺口，本轮已由补采帧补齐。

**6. 总评**

本次仅评 U7，得分 **10.0 / 10**。上一轮唯一未关闭的 Kbd 圆角差距已解决；本轮未发现新的可扣分视觉或布局问题。无需推倒布局、替换组件或重做素材。未采集操作和结构语义仍按上表保留为待核实。

**实际开图工具与图片清单**

使用 `functions.exec` 调用 `tools.view_image({detail:"original"})`，并通过 `image(...,"original")` 实际呈现。长图边界另用 Python 标准库在内存中裁切放大检查，未生成或覆盖任何文件。以下 **48 张**均实际调用过 `view_image`。

本轮原图，34 张：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-fixture-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-heading-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-text-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-kbd-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-blockquote-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-wrap-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-container-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-inset-demonstration-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-reset-comparison-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale110-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale110-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale95-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-scale95-heading-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-invalid-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-invalid-detail-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-fixture-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-heading-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-text-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-code-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-kbd-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-blockquote-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-type-wrap-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-container-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-inset-demonstration-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-layout-reset-comparison-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale110-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale110-heading-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale110-kbd-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale95-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale95-heading-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-scale95-kbd-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-invalid-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U7-matrix-invalid-detail-390-light.png
```

目标图，9 张：

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

上一轮原图，5 张：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-type-kbd-matrix-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-layout-reset-comparison-1280-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-layout-reset-comparison-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U7-fixture-1280-light.png
```
