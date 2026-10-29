1. 评分表

仅评 U1；分数只覆盖已拍摄状态，不替未展示的行为作结论。

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 | R4 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| U1 Studio 工作台框架与基础配置 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** | 9.8 |

- **布局 3.0**：已到维度上限。1440 的 320px 编辑列、弹性预览和分区对齐符合 G1–G2；768 的顶栏换行、390 的 Edit/Preview 切换及返回编辑入口正常；200% 文本重排下关键控件和来源解除按钮没有横向溢出。
- **视觉 3.0**：已到维度上限。深色预览没有污染浅色编辑壳；选中背景、边框与文字符合 G3；字段标签页保持扁平下划线。两项旧视觉差距已关闭，未发现新的规范偏差。
- **交互与状态 3.0**：已到维度上限。记录和焦点帧展示了搜索、选择器展开与键盘移动、模式按钮及字段标签页的清楚焦点；配置、Custom、Overridden、分模式覆盖数量及 auto 解析结果表达明确；200% 导出操作区可达。
- **细节 1.0**：已到维度上限。逐张对照未发现应扣分的接缝、对齐、文案或截断差异；未展示的选项与操作结果列在第 5 项。

2. 上一轮差距复核表

| 编号 | 状态 | 本轮证据与结论 | 本轮扣分 |
| --- | --- | --- | ---: |
| UI-02 | **已解决** | `U1-default-1440-light.png`、`U1-zh-top-1440-light.png`、`U1-edit-390-light.png`、`U1-preview-390-light.png` 中，EN/中文和 Edit/Preview 的接缝均只剩单列选中边框，与模式、渲染器分段的画法一致；Dark 焦点仍保持内收。只读像素复核也未在这些接缝发现 R4 的相邻灰色第二条线。 | 0 |
| UI-193 | **已解决** | `U1-zh-top-1440-light.png`、`U1-zh-configured-1440-light.png` 的「2 组文字对比低于 4.5:1 · 作者颜色保持不变」中，汉字、数字和拉丁字符现在呈一致的半粗字重，不再出现 R4 的汉字常规、拉丁字符半粗混排。 | 0 |

3. 差距清单

**无未关闭差距，也无新差距。**

本轮可挽回分合计 **0.0**，与 `10 − 10.0 = 0.0` 一致。R4 两项各 0.1 分的差距均已关闭。

4. 建议（不扣分）

若后续需要补齐人工证据，可增加一次键盘确认选择、一次键盘切换预览模式，以及一次分隔条拖拽记录。现有证据没有显示这些行为失败，不据此扣分。

5. 看不出的格子

| 图片或记录 | 无法据此确认的内容 |
| --- | --- |
| `U1-keyboard-03-1440-light.png` | 品牌色弹层只展示列表当前可见部分，无法核实完整 26 项；其他基础选择器的完整选项也未展开。 |
| `U1-keyboard-03-1440-light.png`、`U1-keyboard-04-1440-light.png` 与操作记录 | 已展示展开、ArrowDown、Escape 和模式按钮焦点；未展示键盘确认选项后的结果，也未展示键盘激活 Dark 后的结果。 |
| 桌面、768 和 200% 框架帧 | 静态画面不能确认分隔条的 8px 命中范围、拖拽行为以及 260–520px 限制；日志没有对应拖拽步骤。 |
| `U1-zoom200-export-end-720-light.png` | Copy code、Select text、Download 均完整可见，但本单元记录未点击这些按钮，不能判定其执行结果或成功反馈。 |
| U1 全部图片与记录 | 保存失败、撤销和重做的执行结果未展示；本轮仅确认可见入口及已拍摄的保存状态文案。 |

**实际打开的图片**

以下每个文件均逐张打开了 **R5 原图和 R4 同名原图**，合计 50 张；未以哈希代替视觉检查。

```text
U1-default-1440-light.png
U1-configured-1440-light.png
U1-configured-1440-dark.png
U1-top-1440-dark.png
U1-zh-top-1440-light.png
U1-zh-configured-1440-light.png
U1-default-768-light.png
U1-configured-768-light.png
U1-edit-390-light.png
U1-configured-390-light.png
U1-preview-390-light.png
U1-zoom200-720-light.png
U1-zoom200-export-720-light.png
U1-zoom200-colors-720-light.png
U1-zoom200-source-720-light.png
U1-keyboard-01-1440-light.png
U1-keyboard-02-1440-light.png
U1-keyboard-03-1440-light.png
U1-keyboard-04-1440-light.png
U1-zoom200-export-end-720-light.png
U1-field-tabs-1440-light.png
U1-field-tabs-focus-1440-light.png
U1-configured-lower-1440-light.png
U1-zoom200-source-720-light-a2.png
U1-zoom200-source-end-720-light.png
```

6. 总评

本次只评 U1，单元分 **10.0/10**。在已捕获的桌面、平板、手机、中文、深色预览、200% 文本重排和焦点状态中，未发现仍需修复的规范差距。两项 R4 遗留问题均已解决，**不需要结构性改变**。
