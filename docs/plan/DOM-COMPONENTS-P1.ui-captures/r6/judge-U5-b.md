本次仅评 U5，依据冻结目标包全文、39 张 R6 原图、操作记录、R5 独立评审与 5 张相关旧图。未读取白名单外材料，未写文件，未派生代理。U5 按规范模式比较；已遵守目标包对参考选阶／旧 token 对低对比的接受口径，UI-115 的接受未扩展到 U5。

**1. 评分**

| 单元 | 布局 / 3 | 视觉 / 3 | 交互与状态 / 3 | 细节 / 1 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U5 Studio 预览 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00 / 10** |

- **布局 3.00**：桌面并排、390px 堆叠、局部边界、浮层及拾取面板均满足已拍范围，DOM/GPU 首行内缩和纵向起点已统一，达到本维度上限。
- **视觉 3.00**：编辑壳保持中性浅色，浅／深根主题中的相反边界及其菜单、对话框均采用创建处主题，拾取采用蓝色标识和浅蓝信息面板，未见规范偏差，达到本维度上限。
- **交互与状态 3.00**：记录与截图支持主题切换、相反边界、浮层展开、独立对比、DOM+GPU 以及普通／未标注部件拾取；390px 拾取能进入来源编辑并返回预览，已拍范围未见缺陷，达到本维度上限。
- **细节 1.00**：对照相关旧图，未发现 Confirm 尺寸、拾取框边缘、文字换行或本轮比较首行的残余差异，达到本维度上限。

未关闭差距的可挽回分合计 **0.00**，与 `10 − 10.00` 一致。

**2. 上一轮差距复核**

| 编号 | 判定 | 复核依据 |
| --- | --- | --- |
| UI-76 | **已解决** | R5 比较图中 DOM 首行比 GPU 更靠左、略靠上；R6 `U5-gpu-compare-1440-light.png` 两框首行相对左边框均约内缩 25px，字墨上沿一致。新增深色比较帧也保持相同起点，未见横向回归。 |
| UI-207 | **已解决，保持** | R6 局部深色对话框与 R5 对应图一致：深色表面、浅蓝 Confirm、深色按钮文字。补采还显示深色根中的浅色局部对话框正确采用浅色表面与蓝底白字 Confirm。 |
| UI-208 | **已解决，保持** | 对照 R5 桌面、390px 及局部深色对话框，R6 Confirm 保持同样尺寸与内边距，补采的主题组合未见尺寸回归。 |
| UI-209 | **已解决，保持** | R6 `U5-inspector-card-1440-light.png` 与 R5 对应图一致：蓝色侧边延伸至预览可视边界后裁切，没有横贯卡片的假底边。 |

**3. 未关闭差距清单**

无。未发现需要新增编号的、能由当前截图与记录确认的规范差距。

**4. 建议（不扣分）**

无新增设计建议。以下证据缺口应继续保留为待核实，不能由本次满分推定通过。

**5. 看不出的格子**

| 图片或记录 | 仍无法确认 |
| --- | --- |
| 各主预览与 DOM+GPU 图 | 宽度选择器均为 Responsive；未展示 1024／768／390 三档模拟宽度及切回自动的效果。真实 390px 视口不能替代模拟宽度验收。 |
| 所有菜单图 | 可确认展开、可见范围和局部主题；未展示 Duplicate／Archive 选中后的结果与反馈。 |
| 所有对话框图 | 可确认尺寸、换行、创建处主题和 Confirm 外观；未展示 Confirm 结果、Tab 循环、对话框 Escape 关闭及关闭后回焦。 |
| 所有拾取图 | 可确认普通 DOM 部件、来源定位及未标注提示；未展示菜单／对话框部件拾取、GPU 部件拾取或“All drawn tokens”的键盘选择结果。 |
| 局部边界菜单／对话框图 | 已补齐浅色根中的深色边界及深色根中的浅色边界；未展示浮层保持打开时主题动态变化。 |
| DOM+GPU 图 | 可确认正常英文 GPU 预览；未展示 GPU 失败、恢复及中文状态。 |
| `U5-matrix2-gpu-compare-end-390-light.png` | 图中 Color tokens 仍向下延伸，不能确认 GPU 面板最末内容和底部边界。 |
| 全部静态图与操作记录 | 不足以确认完整键盘路径、连续 hover／pressed 变化和过渡行为。操作日志中一次菜单 Escape 不足以证明其余键盘行为。 |

原图均清楚可辨；以上是状态覆盖不足，不是图像模糊。

**6. 总评**

本次只评 U5，得分 **10.00**。R5 唯一未关闭的 UI-76 已解决，补采的深色和390px组合未显示新的可确认差距。当前无需重做布局、替换组件或素材。此结论限于已拍、已记录范围。

实际开图工具：`tools.view_image`，均使用 `detail: "original"`。共实际打开 **44 张**，其中 R6 39 张、R5 5 张：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-dom-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-dom-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-local-opposite-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-menu-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-dialog-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-compare-nested-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-gpu-compare-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-card-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-picked-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-undeclared-hover-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-undeclared-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-preview-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-dialog-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-dom-caller-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-local-opposite-menu-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-local-opposite-dialog-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-inspector-undeclared-panel-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-dom-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-menu-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-dialog-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-menu-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-dialog-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-compare-nested-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-inspector-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-inspector-picked-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-gpu-compare-1440-dark.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-dom-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-menu-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-local-opposite-dialog-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-menu-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-dialog-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-compare-nested-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-inspector-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix-inspector-picked-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix2-gpu-compare-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U5-matrix2-gpu-compare-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-gpu-compare-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-local-opposite-dialog-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-inspector-card-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-dialog-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-dialog-390-light.png
```
