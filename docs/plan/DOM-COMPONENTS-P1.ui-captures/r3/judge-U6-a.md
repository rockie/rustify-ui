# U6 评审原文 · R3

评审范围：只评 U6（Button / IconButton / Link 与内嵌 ThemePanel）。

依据的材料：
- 目标包与四张目标图：radix-sink-button、radix-sink-icon-button、radix-classic-button、radix-sink-link。
- R3 中所有以 `U6-` 开头的截图、`U6-actions.log` 和 `capture.md`。
- R2 的同名截图，以及 `r2-judge-U6.md`。

目标包里 R2 之后新增的条目（UI-156、UI-97、UI-88/89）只涉及 U5、U7，或者是 U7 的采集方式。U6 的比较口径没有变化。R3 与 R2 的 `U6-actions.log` 除时间戳外逐行相同，两轮的帧可以逐张对照。

下文的像素数值用 ImageMagick 从截图中逐像素读取。亮度差按 Rec.709 luma 计算，都是相对同一行 solid 按钮的填充色。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.9 | 3.0 | 1.0 | **9.9** |

**布局 3.0**（已是满分）
- 默认尺寸下，solid 四档高度依次为 24、32、40、48px（U6-fixture-1280-light，x=152/240/340/452 列）。
- 110% 下依次为 26、36、44、52px（U6-scale110-1280-light）。
- solid 4 的左右内边距都是 24px（左缘 444 到字形 468，字形 521 到右缘 545）。目标 solid 4 左侧为 431−406=25（含字形侧距）。solid 2 为 13px，目标为 12px，差的 1px 是字形侧距。
- IconButton 是 32×32 的正方形（x 284–315，y 907–938）。
- ghost 和 ghost-offset 的几何、full 圆角的胶囊形状，以及 390 宽度下的换行，都与 R2 一致，没有溢出或截断。

**视觉 2.9**（不给 3 的原因：classic 上缘和两侧的内高光仍明显弱于目标，见 UI-81 残余）
- 各变体的选阶与 R2 一致：
  - 深色 surface 描边 #3B52A1、填充 #141727；
  - outline 描边 #425CB1；
  - soft 填充 #18244A；
  - disabled 用中性灰。
- classic 的主体渐变已从 R2 的约 1/3 提升到目标的约 80%。
- 新的改进：非 ghost 变体现在比 ghost 粗一档。ghost 与 ghost-offset 的笔画宽度比，R3 为 0.78–0.80，目标为 1.57/1.90≈0.83。R2 时两者同宽，比值约 1.0。

**交互与状态 3.0**（已是满分）
- 13 张 DPR 2 特写，以及 gold、Unavailable、Link 的焦点帧，逐张测量后都符合 B2：
  - 焦点环宽 2px。
  - classic、solid、red、high-contrast、IconButton、gold、Unavailable 留 2px 间隙。
  - soft、surface、outline、ghost、ghost-offset 不留间隙，焦点环压在边缘上。
- 焦点环颜色都用各自色族的第 8 阶：

  | 控件 | 焦点环颜色 |
  | --- | --- |
  | 默认（浅色） | #8DA4EF |
  | 默认（深色） | #435DB1 |
  | red | #EB8E90 |
  | gold | #B9A88D |
  | High contrast | #5EB1EF（蓝色族第 8 阶） |

- Link 聚焦时下划线隐藏。
- loading 状态保留按钮宽度和填充色，spinner 清晰可辨。
- 110%、圆角 full 和中文界面都可用。

**细节 1.0**（已是满分）
- ThemePanel 显示「Resolved neutral: Slate.」和「已解析中性色：蓝灰色。」，与品牌色「Indigo」「靛蓝色」的规则一致。
- 「No overrides」后面多余的前导「·」已去掉。
- 逐处比对，没有发现其他文案、格式或对齐上的差异。

## 2. 上一轮差距复核表

| 编号 | R2 结论 | R3 状态 | 依据 |
| --- | --- | --- | --- |
| UI-12（U6 部分） | 部分解决，U6 可挽回 0.15 | **已解决** | U6-fixture-1280-light、-dark、-390 都显示「Resolved neutral: Slate.」；U6-panel-zh-1280-light 显示「已解析中性色：蓝灰色。」，与品牌色「靛蓝色」同一规则。R2 的「无覆盖 · 浅色 0 · 深色 0」中多余的「·」也已去掉。U1 部分不在本次评审范围。 |
| UI-81 | 部分解决，可挽回 0.15 | **部分解决**（剩余 0.1） | 主体渐变：深色 classic 4（x=500）相对 solid 为 −3…+13，目标为 −5…+16，约达到 80%（R2 为 −1…+6）。上缘与两侧的内高光仍偏弱，详见下节。 |

另外有一项 R2 没有登记、本轮已改进的问题：R2 中 ghost 与 ghost-offset 的笔画同宽（R2 深色 ghost-offset 4 约 2.0–2.3px，ghost 4 约 2.0–2.2px），目标中 ghost 明显更细。R3 已经拉开了这个差距，不再计差距。

## 3. 差距清单（未关闭）

#### UI-81 · [P3] classic 的内高光（上缘高光带与两侧亮边）只有目标的 1/3 到 2/3（U6 · 视觉 · 部分解决 · 可挽回 0.1）

**位置**
- 被测：U6-fixture-1280-dark 的 classic 行，以 classic 4 为准：
  - 纵向取 x=500 列（y 132–187）和 x=540 列（上缘 y 135–142）；
  - 侧边取 y=150 行（x 478–487 和 x 593–602）。
- 目标：radix-sink-button 的 classic 4：
  - 纵向取 x=420 列（y 182–237）和 x=460 列（y 184–191）；
  - 侧边取 y=200 行（x 402–411 和 x 512–521）。

**现状 → 期望**（亮度差相对同列 solid 的填充色）

| 部位 | R3 | 目标 | 结论 |
| --- | --- | --- | --- |
| 主体渐变 | −3…+13 | −5…+16 | 约 80%，可接受 |
| 上缘高光（第 1 行） | #7C95E7，+50 | #B7ABE6，+78 | 只有 64% |
| 上缘高光下方衰减 | +4、+0.5，只有 1px 线 | +15、+7、+2，约 3px 的柔和亮带 | 衰减带基本缺失 |
| 左右侧缘亮边 | +6，1px（#4668D8，内部 #3D62DA） | 第 1px +18、第 2px +4（#7C69CD → #7058CC，内部 #6C54CB） | 只有约 1/3 |
| 底部暗线 | #334DA3，−22 | #584B90，−17 | 已接近 |

放大看，R3 的 classic 像一个带轻微渐变的 solid。目标则是顶部有一条明显的亮边，再往下柔和过渡。这就是剩余差异的来源。

**修法**
- 只改 rustify-components 中 Button classic 变体的 inset 阴影与背景渐变层，不动 solid 和其他变体：
  - 把上缘 1px inset 高光的不透明度提高约 1.5 倍；
  - 在它下面加一层约 2–3px 带模糊的 inset 亮带（或在背景渐变顶部 0–8% 处加一个亮色 stop），复现目标中 +15、+7、+2 的衰减；
  - 把左右 inset 亮边的不透明度提高约 3 倍，并让它向内多衰减 1px；
  - 底部暗线保持现状。
- 改完 class 字符串后，按仓规运行 `mbx xtask css` 并提交 `rustify.css`。
- 验收方法：在深色 classic 4 的同样位置复测，上缘第 1 行应达到约 +75，侧缘第 1px 应达到约 +18。

（UI-81 的 0.1 合计约等于 10 − 9.9。）

## 4. 建议（不扣分）

以下各项在目标图或规范要点中没有可比依据，或者截图没有覆盖到：

- 夹具补齐以下内容，让目标图里的格子都能逐格比对：
  - Button 的 high-contrast、gray、gray + high-contrast、disabled 各列（目前只有 solid 的「High contrast」和「Disabled request」）；
  - IconButton 的 size × variant 矩阵（目前只有一个 solid size 2 的「×」）；
  - 26 色 classic 与 classic high-contrast 列（对应 radix-classic-button）；
  - Link 的 size 1–9 和 underline 四种模式（对应 radix-sink-link）。
- 补一个「文字 + 图标」的 Button，用来核对目标中「Next →」那样的文字与图标间距、图标透明度。
- 「Caller utilities」示例目前是红底配暗红字，几乎不可读，建议换成可读的配色。
- 补拍 Unavailable 按钮被激活后的说明性反馈。
- Link 悬停帧请悬停在 underline=auto 或 hover 的链接上，并在夹具里标注该链接的 underline 属性（见第 5 节）。
- 补拍 ThemePanel「Seed both-mode overrides」之后的覆盖态，用来核对 C3 要求的「N 项被覆盖」类说明。

## 5. 看不出的格子

- **U6-links-hover-1280-light**：与 U6-links-1280-light 逐字节相同（`cmp` 一致）。「Open a native link」在静止时就有下划线，截图看不出它的 underline 属性；悬停是否改变下划线也无从判断。目标中 auto 链接在静止时没有下划线。
- **U6-loading-1280-light**：目标图里没有 loading 状态，spinner 的形态没有比对基准，只核对了 B3 的「保留宽度、阻止点击、spinner 装饰」。
- **IconButton**：只拍到 solid size 2 与圆角 full 两个实例，radix-sink-icon-button 的其他 27 格没有对应。
- **classic 多色**：夹具只有 accent 色的 classic，radix-classic-button 的各色 classic 与 high-contrast 无法比对。
- **Button 的 gray、high-contrast 和各变体的 disabled**：没有拍到对应格子（目标图第 5–8 列）。
- **字重**：截图无法确认实际计算出的 font-weight 是不是 500。只能由 ghost 与非 ghost 笔画宽度比（R3 0.78–0.80，目标 0.83）推断与目标一致。两者字族不同，绝对笔画宽度不可比。
- **「Primary pair」「Accent pair」「Destructive pair」**：在圆角 full 下仍是直角、细字重，R2 和 R3 相同。截图看不出它们是 Button 组件还是调用方的 token 配对演示，所以没有计入差距。
- **ThemePanel 的覆盖态（C3）**：没有拍到。

## 6. 总评

- U6 本轮 **9.9 分**，比 R2 的 9.7 高 0.2：
  - UI-12 已关闭，中性色的英文和中文显示名都已本地化，细节 +0.15；
  - classic 主体渐变从约 1/3 提升到约 80%，视觉 +0.05；
  - 此外，ghost 与非 ghost 的字重差已与目标一致。
- 唯一剩下的是 UI-81：classic 上缘和两侧的内高光偏弱，属于 P3 打磨级别。
- 不需要推倒重做布局、更换组件或重做素材，只需在 classic 变体的 inset 阴影和渐变层上做数值调整。
- 目标图中较大一部分格子（IconButton 矩阵、多色 classic、gray / high-contrast / disabled 列、Link 字级与下划线模式）在夹具里没有对应实例，这部分既不算通过也不算失败。
