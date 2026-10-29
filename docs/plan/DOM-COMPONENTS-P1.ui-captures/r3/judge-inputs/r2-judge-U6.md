# R2 评审结论 · U6 摘录

摘自 R2 合并结论 judge.md，只含 U6 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U6 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.85 | 3.0 | 0.85 | **9.7** | a | — |


### 各维度理由

**U6（a）**

- 布局 3.0：四档高度 24/32/40/48，110% 为 26/36/44/52，内边距、IconButton 正方形、ghost/ghost-offset 几何、size 4 圆角、full 圆角、390 换行都与目标一致。
- 视觉 2.85：各变体选阶（soft 第 3 阶、surface 第 2 阶 + 第 7 阶描边、outline 第 8 阶描边、文字第 11 阶、disabled 中性灰）对上目标。扣分原因：classic 上暗下亮渐变幅度只有目标约 1/3，主体与 solid 几乎同色。
- 交互 3.0：13 张 DPR 2 特写与 gold、Unavailable、Link 焦点帧逐张量环宽、颜色、offset，全部符合 B2；Unavailable 焦点环清晰；loading 保留宽度与填充、spinner 可辨；110%、full、中文可用。
- 细节 0.85：ThemePanel 品牌色已本地化为「靛蓝色」/「Indigo」，「已解析中性色」仍显示原始 id「slate」。


## R2 未关闭差距（U6）

#### UI-12 · [P3] 自动配对的中性色名显示为小写原始 id 且不翻译，与品牌色显示规则不一致（U1/U6 · 细节 · 部分解决 · 可挽回 U1 a 0.1 / b 0.05，U6 a 0.15）

- 位置：U1-configured-1440-light「Resolved neutral: mauve.」；U1-zh-configured-1440-light「已解析中性色：mauve。」；U6-fixture-1280-light「Resolved neutral: slate.」；U6-panel-zh-1280-light「已解析中性色：slate。」（品牌色已显示「靛蓝色」/「Indigo」）。
- 修法：auto 结果与选项列表共用同一个本地化显示名函数。
- 来源：U1 a UI-12、b UI-12；U6 a 新-1（同一 ThemePanel 字段的同一显示规则问题）。

#### UI-81 · [P3] classic 渐变幅度只有目标约 1/3，主体与 solid 几乎同色（U6 · 视觉 · 部分解决 · 可挽回 a 0.15）

- 位置：U6-fixture-1280-dark/-light classic 行，以 classic 4 的 x=500 列为准；对照 radix-sink-button classic 4 的 x=420 列。
- 现状 → 期望：方向、顶部 1px 高光、底部 1px 暗线、无外投影已对；深色下内容区相对 solid 仅 −1…+6，目标为 −5…+17。
- 修法：只调 classic 背景渐变层的亮度差约为现在 3 倍，其余保持。
- 来源：a UI-81。


## R2 建议（U6，不扣分）

- U6：
  - 夹具补 high-contrast、gray、gray + high-contrast、disabled 列，IconButton size × variant 矩阵，Link 1–9 字级与 underline 四模式。
  - 补「文字 + 图标」Button。
  - 「Caller utilities」示例改为可读配色。
  - 拍出 Unavailable 被激活后的说明性反馈。
  - Link 悬停帧悬停 auto/hover 下划线链接并标注属性。
