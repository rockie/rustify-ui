U2 本轮为 **10.0/10**，限于本轮已拍界面与操作记录。已实际打开全部 23 张 R3 截图，并打开 5 张 R2 同名截图比对。未发现有规范依据的扣分差距；U4 不评分。

**1．评分**

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 |
|---|---:|---:|---:|---:|---:|
| U2 搜索加载与恢复 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** |

- **布局 3.0：**已达该维度上限。桌面面板约 576px、窄屏约 343px；标签与 Close、输入、状态/总数、列表及 Retry/Show more 的排列符合 S1–S2。375px 长列表滚动后，输入、Close 和 Show more 仍完整可达，未见面板或页面横向溢出。
- **视觉 3.0：**已达该维度上限。已见浅深主题的面板、输入背景、扁平边界、圆角、文字层级、焦点环、选择色与命中高亮，与 G1–G2、S1–S2 的视觉规则一致；未见阴影、异常强调或明显对比度问题。此处是截图可见表现判断，不把图片观察表述为对全部 CSS 数值的测量认证。
- **交互与状态 3.0：**已达该维度上限。加载期间可编辑查询；模块和索引失败均明确显示暂不可用与 Retry；恢复后保留查询并聚焦输入。键盘 Retry、Close、Esc 归焦、关闭重开及旧响应释放后的当前查询均有对应截图和操作记录，符合 S3–S5。
- **细节 1.0：**已达该维度上限。已见状态文案、单复数“1 result”/“98 results”、命中词、面包屑路径、按钮对齐及窄屏换行未发现规范差异。目标为规范模式，没有可据以宣称逐像素相等的外部目标图。

**2．上一轮差距复核**

允许读取的 R2 `judge.md` 将 U2 记为 10.0 分、没有未关闭差距，并将 UI-02、UI-04 记为已解决；其中没有复述两项原问题的完整定义。因此沿用编号与已有关闭状态，不补造原问题措辞。

| 编号 | 本轮状态 | 本轮依据 |
|---|---|---|
| UI-02 | **已解决，未见回归** | 当前两种视口的面板、输入、状态与恢复按钮完整；375px 长列表的顶部控件和底部 Show more 保持可达。比对 R2 同名加载、错误及长列表图，未见可见退化。 |
| UI-04 | **已解决，未见回归** | 加载中修改查询、键盘 Retry 恢复输入焦点、Close/Esc 返回 Search、重开保留查询，以及新结果出现后释放旧响应的证据均存在；当前图与 R2 对应焦点、旧响应图未见退化。 |

上述结论仅维持允许材料中的关闭结论，不声称独立恢复了未提供的原问题全文。

**3．所有未关闭差距**

**无。**没有新增编号，也没有需要按 P0–P3 排序的修复项。

可挽回分合计 **0.0**，与 `10 − 10.0 = 0.0` 一致。没有证据要求推倒布局、替换组件或更换素材。

**4．不扣分建议**

无新增产品设计建议。现有材料已经支持已拍范围的满分，不建议为了制造差距而增加错误样式、动画、图标或诊断文案。

**5．看不出的格子**

以下均保持未知，不计成功或失败，也不据此扣分：

- **IME 组合输入：**没有组合输入期间及结束后的实际操作记录，无法验证 S5 的组合输入约束。
- **完整焦点循环：**已见输入→Retry、输入→Close、关闭→Search；没有完整 Tab/Shift+Tab 循环记录，不能据此宣布所有焦点边界均已验证。
- **错误期间改词再重试：**当前恢复记录均保留原有 `CustomOnlyToken`；尚未见错误状态下修改查询后再 Retry 的独立证据。
- **悬停、按下和动态过程：**静态关键帧不能证明全部 hover/pressed 状态或过渡过程。
- **真实移动软键盘与动态视口：**375×812 为浏览器 CSS 视口，不能替代实体手机软键盘打开后的布局。
- **辅助技术：**可见总数与选择状态不等于实际朗读；本次没有 VoiceOver 原生朗读证据。U4 不评分。
- **精确实现属性：**截图不能单独证明字体加载来源、全部 token 数值、动态视口单位及所有隐藏内容的实现；本评分只评价可见效果与提供的实际操作。

**6．已评覆盖、实际开图清单与总评**

已评覆盖：

| 视口/主题 | 已见状态与操作 |
|---|---|
| 1280×900 dark | loading、module error/retry success、index error/retry success、Esc 关闭归焦、重开保留查询、旧响应释放后的当前查询、新查询先出结果后再释放旧响应 |
| 375×812 light | loading、module error/retry success、index error/retry success、模块返回前打开和输入、加载期间改词及最终结果、长列表键盘滚动、Retry 焦点与 Enter 恢复、Close 焦点与 Enter 关闭归焦 |

R3 实际逐张打开以下 **23 张**，路径均位于 `docs/plan/DOCS-SSG-P4.ui-captures/r3/`：

1. `U2-loading-1280-dark.png`
2. `U2-closed-01-1280-dark.png`
3. `U2-reopen-02-1280-dark.png`
4. `U2-late-response-03-1280-dark.png`
5. `U2-module-error-1280-dark.png`
6. `U2-module-retry-1280-dark.png`
7. `U2-index-error-1280-dark.png`
8. `U2-index-retry-1280-dark.png`
9. `U2-loading-375-light.png`
10. `U2-module-error-375-light.png`
11. `U2-module-retry-375-light.png`
12. `U2-index-error-375-light.png`
13. `U2-index-retry-375-light.png`
14. `U2-supp-open-before-module-01-375-light.png`
15. `U2-supp-loading-edit-02-375-light.png`
16. `U2-supp-loading-result-03-375-light.png`
17. `U2-supp-longlist-04-375-light.png`
18. `U2-supp-retry-focus-05-375-light.png`
19. `U2-supp-keyboard-retry-06-375-light.png`
20. `U2-supp-close-focus-07-375-light.png`
21. `U2-supp-close-return-08-375-light.png`
22. `U2-supp-new-result-before-old-09-1280-dark.png`
23. `U2-supp-old-released-after-new-10-1280-dark.png`

另实际打开 R2 同目录中的以下 **5 张同名图**进行比对：

- `U2-loading-1280-dark.png`
- `U2-module-error-375-light.png`
- `U2-supp-longlist-04-375-light.png`
- `U2-supp-retry-focus-05-375-light.png`
- `U2-supp-old-released-after-new-10-1280-dark.png`

U2 已拍范围符合目标包 S1–S5、G1–G3 的相关要求，与上一轮保持一致。没有需要结构性改变的证据；未拍状态和原生辅助技术证据保持待核实。
