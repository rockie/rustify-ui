# R4 独立评审结论摘录 · U7

仅摘录 R4 judge.md 中本单元的评分、理由和全部未关闭差距；不含主 agent 裁定、机器检查和修复记录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | R3 |
| --- | --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 3.0 | 3.0 | 3.0 | 0.9 | **9.9** | 9.3 |


- 布局 3.0：Container 四档宽度与三种对齐正确；Inset 各 side 只消去最近容器声明的 padding，side=left/right 改为并排、四种状态下不再相压；Reset 的 input、textarea 回到值列；显式尺寸在缩放下不变。
- 视觉 3.0：Heading、Strong 粗体；Text 四档墨量递增；字级梯度、行高、Code、Kbd、Blockquote 与目标逐项对应。
- 交互 3.0：110%、95% 只缩放一次；Invalid prop 诊断带属性定位、拒收值与回退值，且不扰动其他区域；wrap、truncate、trim 各档可辨。
- 细节 0.9：Reset 的 input、textarea 虚线标记越过内容右沿约 5px；240px 样例正文贴左边框。


## 待复核旧差距


#### UI-189 · [P3] Reset 的 input、textarea 虚线标记越过内容右沿（U7 · 细节 · 部分解决 · 可挽回 0.05）

- 位置：U7-fixture-1280-light Reset 区两行虚线框右缘 x=1229，Container 分隔线止于 x=1224；U7-fixture-390-light 右缘 x≈363（内容右沿约 358）；U7-layout-reset-comparison-1280/390 元素截图里右边线被裁掉。脱离值列已修复。
- 期望：L5 容器边界不遮住内容；比较口径中 Reset 的作用范围。
- 修法：值列单元格加 `padding-inline-end: 5px`，或块级 Reset 元素的标记改 `outline-offset: -1px`；不加包裹层（L4）。
- 来源：U7 UI-189（部分解决）。

#### UI-211 · [P3] 240px 显式宽度样例的正文贴着左边框（U7 · 细节 · 可挽回 0.05）

- 位置：U7-fixture-1280-light 第 1628–1651 行，「A bad prop keeps this view alive.」首字距左边框约 1px，标注与盒顶 2–3px；U7-invalid-detail-1280、95%、110%、390 帧同样。
- 修法：样例 Box 加显式 `padding: 2px 8px`、`box-sizing: border-box`，标注与盒之间 4px。
- 来源：U7 新-1。

