# U6 评审 R2：Button / IconButton / Link 与内嵌 ThemePanel

只读了任务列出的材料：目标包、4 张目标图、r2 的 U6 截图、`capture.md`、`U6-actions.log`，r1 的 U6 截图与 `capture.md`，以及 r1 `judge.md` 指定的三段。颜色和尺寸数值都是用 ImageMagick 在这些 PNG 上取样得到的，取样坐标为图像像素。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U6 Button / IconButton / Link 与 ThemePanel | 3.0 | 2.85 | 3.0 | 0.85 | **9.7** |

**布局 3.0（满分）**
- 四档高度实测 24/32/40/48，与目标一致：目标 solid 4 为 258–305 行，共 48px。
- 缩放 110% 后为 26/36/44/52。
- 左右内边距约 8/12/16/24。
- IconButton 为 32×32 正方形。
- ghost-offset 的文字与 solid 文字对齐；ghost 的文字贴列边，与 radix-sink-button 的对应行同构。
- size 4 圆角约 8px，与目标相同。
- 圆角 full 后全部变为胶囊，IconButton 变为圆形。
- 390 宽下换行正常，没有溢出或截断。

**视觉 2.85**
- 各变体的选阶都对上了目标规则：soft 用第 3 阶填充；surface 用第 2 阶填充加第 7 阶描边；outline 只有第 8 阶描边；文字用第 11 阶；disabled 为中性灰。
- 不是 3 分的原因：classic 的上暗下亮渐变幅度只有目标的约 1/3，classic 主体看上去与 solid 几乎同色（UI-81 的剩余部分）。

**交互与状态 3.0（满分）**
- 13 张 DPR 2 特写，加上 gold、Unavailable、Link 的焦点帧，逐张量了环宽、颜色和 offset，全部符合 B2。
- Unavailable 的焦点环清晰可见。
- loading 保留宽度和填充色，spinner 清楚可辨。
- 缩放 110%、圆角 full、中文切换都可用。
- 没拍到的状态见第 5 节，不计入分数。

**细节 0.85**
- 上一轮的中文措辞问题已修正。
- 不是 1 分的原因：ThemePanel 里有一处色族名显示不一致。品牌色已本地化为「靛蓝色」（英文为 "Indigo"），同一面板的「已解析中性色」却显示原始 id「slate」，英文也是小写 "slate"。

## 2. 上一轮差距复核表

| 编号 | 上一轮问题 | 本轮状态 | 依据（本轮截图与取样） |
| --- | --- | --- | --- |
| UI-77 | 非 solid 变体与 Link 的文字用了第 9 级 #3E63DD | **已解决** | `U6-fixture-1280-dark`：soft、surface、outline、ghost-offset、ghost 的文字都是 #9EB1FF。`-light` 中为 #3A5BC7（soft 上为 #3658C7）。Link 正文为 #3A5BC7（`U6-links-1280-light`，r1 是 #3E63DD）。目标里这些位置都是文字级 #BAA7FF。 |
| UI-78 | 可聚焦的 Unavailable request 焦点环几乎不可见（#E3E8FB） | **已解决** | `U6-focus-unavailable`：环色 #8DA4EF，宽 2px，环与按钮之间空 2px；按钮为中性灰填充 #F0F0F3、文字 #AEB0BD，不再是整体半透明。 |
| UI-79 | 深色 soft、surface 填充与 surface、outline 描边过暗，surface 比页面还暗 | **已解决** | 深色取样：soft #18244A；surface 填充 #141727（比页面 #111113 亮）、描边 #3B52A1；outline 描边 #425CB1，内部透明。各色之间的明暗关系与目标一致（soft #2A1F45；surface #1C1526 / #5B4896；outline #6958AD；页面 #121113），surface 描边比 outline 描边暗一档。 |
| UI-80 | 浅色下 surface 与 outline 完全相同 | **已解决** | 浅色 surface 为填充 #F7F9FF 加描边 #A5B9F9；outline 内部透明，描边 #8DA4EF。两者可以区分。 |
| UI-81 | classic 渐变方向反了，且有外投影 | **部分解决** | 方向已对：深色 classic 4 从顶部 #3D62DA 渐变到底部 #4468DE，顶部有 1px 高光 #7C95E7，底部有 1px 暗线 #334DA3。外投影已去掉，按钮下方第 185 行就是页面色，r1 这里是 #181F37 投影。剩下的问题是渐变幅度不足，见第 3 节。 |
| UI-82 | disabled 带品牌色并整体半透明 | **已解决** | 浅色为 #F0F0F3 填充、#AEB0BD 文字；深色为 #202124 / #636B73。目标 disabled 为 #232225 / #6D6974，同为中性灰阶。 |
| UI-83 | loading 的 spinner 几乎不可见，底色与 disabled 相同 | **已解决** | `U6-loading`：两个 loading 按钮保留 solid 填充 #3E63DD，宽度不变；spinner 为白色前导弧加浅色轨道，与右侧灰色的 Unavailable、Disabled 明显不同。 |
| UI-84 | ThemePanel 中文说明与英文原意不对应 | **已解决** | `U6-panel-zh` 改为「分组重置只清除该组在浅色与深色两模式的覆盖；其他组和独立精调保持不变。」，与英文 "A group reset clears only that group's overrides in both modes…" 对应。 |
| UI-85 | Link 焦点环为 2px 实线、无间隙，与按钮的 3px 环不一致 | **不再适用** | B2 已改为 Radix 规则，3px 环的期望不存在了。按新 B2 核验 `U6-focus-link-…-dpr2`：环宽 2px，颜色 #8DA4EF，与文字框之间约 2px，有圆角，聚焦时下划线隐藏（文字与环底边之间第 86–98 行全为白）。符合新规则。 |

B2 逐张核验。DPR 2 下按钮边缘位于第 48 设备像素：

| 特写 | 环宽 | 环色 | offset | B2 期望 | 结论 |
| --- | --- | --- | --- | --- | --- |
| classic、solid（浅） | 2px | #8DA4EF（与 outline 描边同阶） | +2px | 第 8 阶，+2 | 符合 |
| soft、surface、outline、ghost-offset、ghost（浅） | 2px | #8DA4EF | −1px（环跨在边框上） | −1 | 符合 |
| red | 2px | #EB8E90（red 色族） | +2px | 组件色族第 8 阶，+2 | 符合 |
| High contrast（填充 #113264） | 2px | #5EB1EF（与该按钮同色族） | +2px | 组件色族第 8 阶 | 符合 |
| IconButton × | 2px | #8DA4EF | +2px | 同 Button | 符合 |
| solid（深） | 2px | #435DB1 | +2px | 第 8 阶，+2 | 符合 |
| soft、outline（深） | 2px | #435DB1 | −1px | −1 | 符合 |
| gold（`U6-focus-02`，DPR 1） | 2px | #B9A88D | +2px | 组件色族第 8 阶 | 符合 |
| Link | 2px | #8DA4EF | +2px，下划线隐藏 | 同上，并隐藏下划线 | 符合 |

surface 与 ghost-offset 的环落在设备像素 45–48，其余在 46–49，相差半个 CSS 像素，判断为元素位于亚像素位置，不计差距。

## 3. 差距清单

先按严重度、再按可挽回分排序。可挽回分合计 0.30，等于 10 − 9.7。

### UI-81 · [P3] classic 渐变幅度只有目标的约 1/3，主体与 solid 几乎同色

- **单元与位置**：U6，`U6-fixture-1280-dark.png` / `-light.png` 的 classic 行 size 1–4，以 classic 4 的 x=500 列为准；`U6-scale110`、`U6-radius-full` 的 classic 行同样如此。
- **维度**：视觉。
- **现状**（深色 classic 4，相对同图 solid 填充 #3E63DD）：内容区从顶部 #3D62DA 到底部 #4468DE。R 通道只从 −1 变到 +6，G 通道从 −1 变到 +5。肉眼看 classic 只多了上下各 1px 的高光和暗线，主体与 solid 难以区分。
- **期望**：radix-sink-button 的 classic 行（目标 classic 4 的 x=420 列）相对同图 solid #6E56CF，内容区从顶部 #6952C6 到底部 #7C67D4。R 通道从 −5 变到 +14，G 通道从 −4 变到 +17。底部明显比 solid 亮，一眼可以看出渐变。radix-classic-button 每一格也都是这种上暗下亮的可见渐变。
- **修法**：
  - 只改 Button / IconButton 的 classic 变体背景渐变层：把上暗下亮那层线性渐变的不透明度或亮度差提高到现在的约 3 倍。目标是在深色下让顶部内容区比 solid 暗约 2%，底部比 solid 亮约 5–6%。
  - 顶部 1px 高光（目前 #7C95E7）、底部 1px 暗线（目前 #334DA3）和「无外投影」这三点都已正确，保持不变。
  - 改完后在 dark 下按上面的列取样复核，并与 radix-sink-button 的 classic 4 逐行对比。IconButton classic 按同一规则复核（本轮夹具里没有 classic IconButton）。
- **严重度**：P3。
- **可挽回分**：0.15（视觉）。

### 新-1 · [P3] ThemePanel 里品牌色名已本地化，「已解析中性色」仍显示原始 id，大小写也不一致

- **单元与位置**：U6。
  - `U6-panel-zh-1280-light.png`：「品牌色族」下拉显示「靛蓝色」；「中性色族」下方说明是「已解析中性色：slate。」。
  - `U6-fixture-1280-light.png`（英文）："Brand color" 显示 "Indigo"（首字母大写），"Resolved neutral: slate."（全小写）。
- **维度**：细节（多语言）。
- **现状 → 期望**：
  - 现状：同一面板对色族名用了两套显示规则。r1 中文态的品牌色显示为 "indigo"，本轮改成了「靛蓝色」，只改了品牌色一处。
  - 期望：C2 要求「有效 auto 结果（例如中性色 auto 显示实际配对的灰阶名）」；C3 要求「ThemePanel 在 Studio 基础区与可嵌入样例中表达相同选项」。同一面板内，色族名的显示方式和大小写应该一致。
- **修法**：
  - ThemePanel 的品牌色选项文本、中性色选项文本、「Resolved neutral / 已解析中性色」说明文本，改为共用同一个色族显示名函数。
  - 二选一：两处都本地化（如「靛蓝色」/「石板灰」，英文 "Indigo" / "Slate"）；或者两处都显示色族 id（indigo / slate）。
  - 选哪种，以 Studio 基础区现行写法为准，满足 C3 的「表达相同选项」。本评审没有打开 U1 截图，未核对 Studio 的写法。
  - 中文态和英文态都要复拍。
- **严重度**：P3。
- **可挽回分**：0.15（细节）。

## 4. 建议（不扣分）

1. **补齐夹具的比较覆盖面。** 目标图里有 high-contrast、gray、disabled 三列，以及完整的 IconButton 矩阵和 Link 字级梯度，夹具里只有零星实例，下一轮无法逐格对照。建议在按钮夹具中：
   - 给每个变体补「+ high-contrast」「gray」「gray + high-contrast」「disabled」四列，对应 radix-sink-button 右侧四列；
   - 补 IconButton 的 size 1–4 × 7 变体矩阵，对应 radix-sink-icon-button；
   - 在布局夹具的 Link 区补 size 1–9 梯度和 underline auto/always/hover/none 四行，并标出属性，对应 radix-sink-link 前几行。

   这是夹具工作，不是组件缺陷。
2. **补「文字 + 图标」的 Button 实例**，例如 "Next →"，以便比对按钮内图标间距与图标尺寸。目标图每格都有箭头，夹具里没有带图标的 Button。
3. **「Caller utilities」演示按钮几乎读不出字。** 浅色为 #D92D20 填充配 #C8070A 文字，深色为 #F97066 配 #FF9592，和 r1 相同。它演示的是调用方类覆盖，不属于组件外观，但作为示例，建议换一组能读清的调用方 utilities。
4. **Unavailable request 与 Disabled request 外观完全相同**，两者都是 #F0F0F3 / #AEB0BD。B3 只要求 Unavailable 可聚焦、不变成原生 disabled，没有要求外观不同。可以考虑在夹具里拍出它被激活后的「说明性」反馈，让这一区别能从截图读出。
5. **Link 悬停帧请悬停一个默认 auto 或 hover 下划线的链接，并标出属性。** 本轮和 r1 的悬停帧都与默认帧逐像素相同（差异像素 0）。

## 5. 看不出的格子

- **`U6-links-hover-1280-light.png`**：与 `U6-links-1280-light.png` 逐像素相同，看不出悬停反馈。也看不出 "Open a native link" 的 underline 属性：如果它是默认 auto，那么静止态就带下划线，与 radix-sink-link 第一行（auto 静止态无下划线）不符；如果是 always，则符合。图里读不出属性，所以既不算通过也不算失败。
- **按钮的悬停、按下态**：本轮没有拍。
- **high-contrast**：只有一个 solid（蓝色族）实例，看不出 classic、soft、surface、outline、ghost-offset、ghost 的 high-contrast 外观。gray 只有 solid。
- **disabled**：只有一个 solid 形态的「Disabled request」，看不出其他变体的 disabled 外观，例如 surface、outline 是否保留灰色描边，ghost 是否只有灰字。
- **IconButton**：只有 solid size 2 一格（32×32，环已核验），看不出 size 1/3/4 与其他变体。`U6-scale110` 的截取区域是 `button-matrix`，不含 IconButton。
- **Link**：看不出 size 1–9、weight、color、high-contrast 与 truncate 的完整表现。"https://example.test/..." 看得出截断，但分不清用的是 "…" 字形还是三个点。
- **loading**：看不出是否阻止了点击，也看不出 disabled + loading 时 disabled 是否优先。只拍了 solid 变体的 loading。
- **浅色 classic 的边缘**：浅色 classic 顶部有一条 1px 暗线（#385ACD），其下才是高光（#C3CFF5），底部是 2px 暗线；深色顶部直接就是高光。目标图只有深色，看不出浅色专属的边缘规则是否符合。
- **ThemePanel 的深色与 390 中文态**：中文只拍了 1280 浅色。目标包没有要求拍，这里只作说明。

## 6. 总评

- **最低分单元**：本次只评 U6，单元分 9.7（r1 为 7.75）。r1 的九条差距里，八条已解决或因 B2 规则变更而不再适用；UI-81 只剩渐变幅度一处。
- **主要剩余原因**：两处 P3 打磨。
  1. classic 渐变幅度只有目标的约 1/3（UI-81）。
  2. ThemePanel 色族名的显示规则不一致（新-1）。
- **需要结构性改变的地方**：组件本身没有。四档几何、变体选阶、disabled、loading 和 B2 焦点规则都已与目标一致，剩下的都是参数级修正。
- **影响下一轮评审可信度的地方**：夹具对 high-contrast、gray、disabled 列，IconButton 矩阵和 Link 字级梯度的覆盖不足（见第 4 节第 1 条和第 5 节）。这不扣分，但补齐之后，「与目标一致」的结论才能覆盖整张目标图，而不只是夹具展示到的那部分。
