# R5 独立 UI/UX 评审 · U7 布局与排版

仅评 U7。依据目标包、R5 原始 PNG 与操作记录、白名单内 R4 原图和独立结论判断。未读代码、修复记录、机器检查或其他单元，未写仓库。

按 U7 比较口径，页面排列、示例文案、字族、深浅色差异及已接受的参考选阶／旧 token 对低对比不计差距。

## 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U7 布局与排版 | 3.0 | 2.6 | 3.0 | 1.0 | **9.6 / 10** |

- **布局 3.0**：所拍 Container 四档宽度与三种对齐、Inset 四侧贴边及窄屏排列均符合目标；长标题在整段原图中完整可读，未发现需扣分的遮挡或截断，达到本维度上限。
- **视觉 2.6**：字级梯度、Heading／Strong 粗体、Text 四档字重及 Code、Blockquote 的可见处理与目标对应；Kbd 大字级的 classic／soft 圆角明显偏小，尚不能给 3 分。
- **交互与状态 3.0**：110%／95% 帧显示文字与主题间距随比例变化，显式 CSS 尺寸保持稳定；非法属性帧给出属性、拒收值和回退值，其他区域仍可见，达到已采集状态的本维度上限。
- **细节 1.0**：R4 两项细微问题均已关闭；在本轮比较范围内未另发现应独立扣分的小差异。Kbd 形状差异统一计入视觉，未重复扣分。

未关闭差距的可挽回分合计 **0.4**，等于 **10 − 9.6**。

## 2. 上一轮差距复核表

| 编号 | R5 状态 | 对照结果 |
| --- | --- | --- |
| UI-189 · Reset 的 input、textarea 虚线标记越过内容右沿 | **已解决** | R4 整段图中两行虚线右缘越过内容右沿；R5 的 1280、390 整段图中已内收，与内容边界对齐。分段图中右侧虚线也已可见。 |
| UI-211 · 240px 显式宽度样例正文贴左边框 | **已解决** | R5 默认、110%、95%、390 及非法属性特写中，正文与左边框已有明确内边距，标注与盒顶之间也有间隔；110% 时句子正常换成两行。 |

本轮新增发现的 Kbd 圆角问题在 R4 同名原图中已经存在，因此属于**新增发现，不是回归**。本轮分数低于 R4 的 9.9，原因是补记该视觉差距。

## 3. 差距清单

### 新-1 · Kbd 大字级圆角未随尺寸形成目标中的键帽轮廓

- **单元与截图位置**：U7；`U7-type-kbd-matrix-1280-light.png`、`U7-type-kbd-matrix-390-light.png`，classic／soft 两列的 size 7–9，size 9 最明显。
- **维度**：视觉。
- **严重度**：P2 一般。
- **现状 → 期望**：
  - 当前 size 9 soft 是约 **84×84px、约 3px 圆角**的近方形色块；目标 `radix-sink-kbd.png` 的 size 9 soft 同为约 **84×84px**，但圆角约 **18px**。size 8 目标也呈明显圆润转角，当前仍只有小圆角。
  - classic 的外框和内侧高光同样过方，没有目标大字级键帽的圆润轮廓。
  - 依据为 U7 比较口径中的 **Kbd 外观**、L1 的语义外观要求及 L6 的参考尺度规则；此差异不属于字族或深浅色差异。
- **具体修法**：调整 **Kbd 组件的 `border-radius` 尺度映射**，让圆角随对应字级递增；以目标 size 9 soft 约 18px、size 8 约 10px 的转角为校准点。classic 的外轮廓、内侧高光和阴影轮廓同步使用匹配半径，主题缩放按 L6 只应用一次。修后复核 size 1–9 的 classic／soft，避免只修最大一档。
- **证据定位**：分段 PNG 本地坐标中，目标 size 9 soft `/` 位于约 `(503,186)`；当前对应键帽位于约 `(664,172)`。两者宽高相同，转角差异清楚可见。
- **可挽回分：0.4**。

## 4. 建议（不扣分）

无。本轮没有加入目标包之外的设计要求。

## 5. 看不出的格子

- **390px 的缩放与非法属性状态**：本轮只有 390px 默认夹具及各分段图，没有对应 110%、95% 或非法属性帧，不能判断这些状态的窄屏表现。
- **Reset 的实际操作及结构语义**：`U7-layout-reset-comparison-1280/390-light.png` 显示 `clicks: 0`，操作记录没有点击该按钮；截图也不能证明原生元素语义、无包裹层或 scope 隔离。此处不记通过或失败。
- **非法属性恢复动作**：`U7-invalid-detail-1280-light.png` 清楚呈现拒收和回退反馈，但没有点击 Restore prop 后的帧，无法判断恢复结果。
- **未采集断点**：390px 与 1280px 之间、之外的其他断点没有对应截图，不能外推其布局结果。

很长的整段图经 `/tmp` 检查用裁切查看；所有判断仍对应原始 PNG，未覆盖或改写证据。

### 实际打开过的图片

**R5 当前原图，共 26 张：**

```text
U7-fixture-1280-light.png
U7-type-heading-matrix-1280-light.png
U7-type-text-matrix-1280-light.png
U7-type-code-matrix-1280-light.png
U7-type-kbd-matrix-1280-light.png
U7-type-blockquote-matrix-1280-light.png
U7-type-wrap-matrix-1280-light.png
U7-layout-container-matrix-1280-light.png
U7-layout-inset-demonstration-1280-light.png
U7-layout-reset-comparison-1280-light.png
U7-scale110-1280-light.png
U7-scale110-heading-1280-light.png
U7-scale95-1280-light.png
U7-scale95-heading-1280-light.png
U7-invalid-1280-light.png
U7-invalid-detail-1280-light.png
U7-fixture-390-light.png
U7-type-heading-matrix-390-light.png
U7-type-text-matrix-390-light.png
U7-type-code-matrix-390-light.png
U7-type-kbd-matrix-390-light.png
U7-type-blockquote-matrix-390-light.png
U7-type-wrap-matrix-390-light.png
U7-layout-container-matrix-390-light.png
U7-layout-inset-demonstration-390-light.png
U7-layout-reset-comparison-390-light.png
```

**目标图，共 9 张：**

```text
radix-sink-heading.png
radix-sink-text.png
radix-sink-code.png
radix-sink-kbd.png
radix-sink-blockquote.png
radix-text-wrap.png
radix-sink-container.png
radix-inset.png
radix-reset.png
```

**R4 重点视觉复核原图，共 8 张：**

```text
U7-fixture-1280-light.png
U7-fixture-390-light.png
U7-layout-reset-comparison-1280-light.png
U7-layout-reset-comparison-390-light.png
U7-invalid-detail-1280-light.png
U7-type-kbd-matrix-1280-light.png
U7-scale110-heading-1280-light.png
U7-scale95-heading-1280-light.png
```

## 6. 总评

本次仅评 U7，得分 **9.6**。R4 的两条细节差距已经关闭，当前主要差距集中在 **Kbd 大字级圆角**。需要调整组件的圆角尺度映射和 classic 内外轮廓，无须推倒整体布局或更换组件。
