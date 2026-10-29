# R6 独立 UI/UX 评审原文 · U5

本次仅评 U5。已读取冻结目标包的规范与完整变更记录、本单元采集清单和操作记录、R5 独立评审，并实际打开 R6 全部 18 张原图及 R5 的 4 张相关原图。未读取代码、完整报告、机器检查或修复记录，未修改文件。

U5 属于规范模式，没有独立目标 PNG；本次依据 P1–P5、G1 及 U5 比较口径评判。UI-115 的范围接受仅涉及 U2，不改变 U5 尺度；U5 已接受的参考选阶／旧 token 对低对比情形不扣分。

## 1. 评分表

| 单元 | 布局 / 3 | 视觉 / 3 | 交互与状态 / 3 | 细节 / 1 | 总分 |
| --- | ---: | ---: | ---: | ---: | ---: |
| U5 Studio 预览 | 3.00 | 3.00 | 3.00 | 1.00 | **10.00 / 10** |

- **布局 3.00**：桌面 DOM/GPU 比较框首行内缩与纵向起点已统一，390px 预览和对话框在已拍范围内没有横向溢出或关键操作截断，达到本维度上限。
- **视觉 3.00**：浅／深预览、相反外观的局部边界及其菜单和对话框均呈现对应有效主题，编辑壳保持中性浅色，达到本维度上限。
- **交互与状态 3.00**：记录与截图支持模式切换、局部边界切换、浮层展开、独立比较区、DOM+GPU、部件拾取及来源定位；未标注部件有明确说明，已拍范围内未见可确认缺陷，达到本维度上限。
- **细节 1.00**：逐图检查文案、token 标识、按钮尺寸和拾取边框，并对照相关旧图，未发现仍可确认的细节差异，达到本维度上限。

未关闭差距可挽回分合计 **0.00**，与 `10 − 10.00` 一致。满分只覆盖现有证据能够判断的状态；下列证据不足项不算通过。

## 2. 上一轮差距复核表

| 编号 | 判定 | 本轮原图复核结果 |
| --- | --- | --- |
| UI-76 | **已解决** | R6 `U5-gpu-compare-1440-light.png` 中，DOM “Local demonstration…” 和 GPU “GPU · Cards” 相对各自外框均约内缩 25px，首行字墨上沿也已齐平。对照 R5 同名图，DOM 首行此前更靠左、更靠上，本轮差异已消失。 |
| UI-207 | **已解决，保持关闭** | R6 `U5-local-opposite-dialog-1440-light.png` 的面板为深色，Confirm 为浅蓝底、深色字；与已打开的 R5 同名图一致，未出现局部浮层按钮恢复浅色主题用色的回归。 |
| UI-208 | **已解决，保持关闭** | 本轮桌面、390px 及局部深色对话框中的 Confirm 均保持正常的 32px 高度与一致内边距；390px 图和局部深色图与已打开的 R5 对应原图一致，未见尺寸回归。 |
| UI-209 | **已解决，保持关闭** | R6 `U5-inspector-card-1440-light.png` 中，卡片蓝色侧边延伸至预览滚动边界后被裁切，没有在拾取面板上方形成错误的闭合底边；与 R5 同名图一致。 |

## 3. 差距清单

**无仍未关闭的可确认差距，无新增扣分项。**

UI-76 本轮可以关闭；其余上一轮已解决项未见回归。不需要为现有截图重做布局、替换组件或重做素材。

## 4. 建议（不扣分）

无新增设计建议。证据覆盖建议列于下一节，不作为产品缺陷扣分。

## 5. 看不出的格子

| 图片或记录 | 无法确认的内容 |
| --- | --- |
| `U5-dom-1440-light.png`、`U5-preview-390-light.png` | 宽度选择器显示 Responsive；未展示模拟 1024／768／390 三档及切回自动后的效果。实际 390px 视口不能替代模拟宽度状态。 |
| 两张菜单图及操作记录 | 可确认菜单展开及局部主题；未展示选择 Duplicate／Archive 后的实际结果或反馈。 |
| 三张对话框图及操作记录 | 可确认可见布局、主题、关闭入口和 Confirm；未展示 Confirm 后结果、完整 Tab 循环、对话框 Escape 关闭及关闭后的回焦。记录中有局部菜单后的 Escape，但不足以证明对话框相关行为。 |
| 全部拾取图 | 可确认普通部件、卡片、来源跳转和未标注提示；未展示对菜单或对话框内部部件的拾取，不能确认浮层同样可检查。 |
| `U5-local-opposite-*` | 已展示浅色根中的深色边界；未展示深色根中的浅色边界，也未展示浮层保持打开时切换创建处主题的动态更新。 |
| `U5-gpu-compare-1440-light.png`、390px 两图 | 可看到正常英文 GPU 预览；未展示 GPU 失败、恢复或中文状态。390px 对话框背景可见 DOM 后接 GPU，但未完整展示整个 GPU 区域。 |
| `U5-preview-390-light.png` | 首屏下方内容在视口外，不能仅凭此图确认全部场景控件在窄屏下的最终排列及操作结果。 |
| 全部原图及现有记录 | 不足以确认完整键盘路径、所有 hover／pressed 状态、加载／错误／成功反馈和连续过渡行为；没有拍到的状态既不算通过，也不算失败。 |

所有实际打开的原图均可辨认，没有因模糊导致的判读障碍。

## 6. 总评

本次只评 U5，得分 **10.00 / 10**，R5 为 **9.95 / 10**。上一轮唯一未关闭的 UI-76 已解决，相关旧问题未见回归。现有证据没有显示需要结构性改变的地方；未拍状态仍保留为证据不足，不能据此宣称 U5 全部交互均已验收。

实际开图工具：**`view_image`**，通过 `functions.exec` 调用 `tools.view_image`，并展示返回的图像。实际打开图片共 **22 张**：

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
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-gpu-compare-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-local-opposite-dialog-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-dialog-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U5-inspector-card-1440-light.png
```
