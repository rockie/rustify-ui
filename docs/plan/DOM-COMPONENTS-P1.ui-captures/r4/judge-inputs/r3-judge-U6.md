# R3 评审结论 · U6 摘录

摘自 R3 合并结论 judge.md，只含 U6 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U6 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.9 | 3.0 | 1.0 | **9.9** | a | — | 9.7 |

### 各维度理由

**U6（a）**

- 布局 3.0：四档高度 24/32/40/48，110% 下 26/36/44/52；内边距、IconButton 正方形、ghost/ghost-offset 几何、full 圆角、390 换行都与目标一致。
- 视觉 2.9：各变体选阶与目标对应；classic 主体渐变已达目标约 80%；ghost 与非 ghost 的笔画粗细比与目标一致。扣分原因：classic 上缘高光带与两侧亮边只有目标的 1/3 到 2/3。
- 交互 3.0：13 张 DPR 2 特写与 gold、Unavailable、Link 焦点帧逐张测量，焦点环宽、间隙、色族第 8 阶都符合 B2；Link 聚焦时隐藏下划线；loading 保留宽度与填充、spinner 可辨；110%、full、中文可用。
- 细节 1.0：ThemePanel「Resolved neutral: Slate.」「已解析中性色：蓝灰色。」已与品牌色同一规则，多余的「·」已去掉，逐处比对未见其他差异。

## R3 未关闭差距（U6）

#### UI-81 · [P3] classic 的内高光（上缘高光带与两侧亮边）只有目标的 1/3 到 2/3（U6 · 视觉 · 部分解决 · 可挽回 a 0.1）

- 位置：U6-fixture-1280-dark classic 4 对 radix-sink-button classic 4。主体渐变已达目标约 80%；上缘第 1 行 +50（目标 +78），其下衰减只有 1px（目标约 3px 的 +15、+7、+2）；左右侧缘 +6（目标 +18、+4）。
- 修法：只改 Button classic 变体的 inset 阴影与背景渐变层：上缘高光不透明度提高约 1.5 倍并在其下加 2–3px 带模糊的亮带；左右亮边不透明度提高约 3 倍并向内多衰减 1px；底部暗线保持。改完运行 `mbx xtask css`。
- 来源：U6 a UI-81（部分解决）。

### U7

## 建议（不扣分，U6）

  - 夹具补 high-contrast、gray、gray + high-contrast、disabled 各列，IconButton size × variant 矩阵，各色 classic 与 classic high-contrast，Link 1–9 字级与 underline 四模式。
  - 补「文字 + 图标」Button。
  - 「Caller utilities」示例改为可读配色。
  - 补拍 Unavailable 激活后的反馈、ThemePanel 覆盖态；Link 悬停帧悬停在 auto/hover 链接上并标注 underline 属性。
