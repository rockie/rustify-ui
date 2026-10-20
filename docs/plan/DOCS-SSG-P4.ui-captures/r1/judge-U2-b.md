U2 建议通过已采集范围：**9.4/10**。发现两项 P3，未见 P0/P1/P2；未验证状态不随此结论转为通过。

| 维度 | 分数 | 理由及未更高原因 |
|---|---:|---|
| 布局 | 2.9/3 | 状态顺序、按钮位置、长列表内部滚动及换行正确；375px 对话框比 S1 规定窄约 6px，因此未满分。 |
| 视觉 | 3/3 | 实拍中的系统字体、层级、主题色、边界、圆角、焦点、选择与命中样式符合 G1–G3、S1–S2；未发现可扣分偏差，已达上限。 |
| 交互与状态 | 3/3 | 已记录的加载中编辑、两类错误恢复、键盘 Retry、Close/Esc 归焦和晚响应隔离均符合 S3–S5；已观察范围无可扣分偏差，已达上限。 |
| 细节 | 0.5/1 | 单条结果统一显示不合语法的 “1 results”，故不能给 1 分。 |
| **总分** | **9.4/10** | 可挽回分合计 **0.6**。 |

全部扣分差距，按严重度、可挽回分排序：

1. **新-1｜U2｜P3｜可挽回 0.5 分（细节）**  
   **位置：**所有单条结果图的输入框下方总数，例如 `U2-module-retry-375-light.png`、`U2-index-retry-1280-dark.png`、`U2-supp-old-released-after-new-10-1280-dark.png`。  
   **现状 → 期望：**“1 results” → “1 result”；98 条时继续显示“98 results”。数字本身没有错误。  
   **依据：**S2/S3 的结果总数文案，以及本次细节评分的文案准确性口径。  
   **具体修法：**在搜索总数格式化处按数量选择英文单复数，或通过现有本地化复数规则生成；保持当前全部匹配页数语义和 live 更新位置。

2. **新-2｜U2｜P3｜可挽回 0.1 分（布局）**  
   **位置：**所有 375×812 图的 dialog 左右边缘，以 `U2-loading-375-light.png`、`U2-supp-longlist-04-375-light.png` 最易核对。  
   **现状 → 期望：**外宽约 337px、左右各约 19px → S1 的 `min(36rem, viewport − 2rem)`，此视口应为 343px、左右各 16px。桌面约 576px 已符合目标。  
   **依据：**S1。  
   **具体修法：**搜索 dialog 根元素显式统一 `inline-size` 与 `max-inline-size`，使用 `min(36rem, calc(100vw - 2rem))`、`box-sizing: border-box`，排除原生 dialog 默认最大宽度约束对窄屏的二次收窄；保留现有 1rem 内边距。

不扣分建议：没有额外的视觉或结构改造建议。

证据覆盖与缺口：

- U2 必需的 loading、module error/retry success、index error/retry success，在 **1280×900 dark、375×812 light** 均有实拍和操作记录；该矩阵没有缺格。
- 补采证明：module 响应尚未返回时输入已获焦；加载中最终修改的查询用于恢复结果；长列表滚动后 Close、输入和 Show more 仍在视口；Tab/Enter 可完成 Retry，Shift+Tab/Enter 可完成 Close 并归焦。
- 晚响应补采顺序有效：新查询结果先出现，之后才释放旧响应，等待后仍保持新查询；两张实拍一致，与记录相符。
- 本包没有直接证实：Cmd/Ctrl+K 打开、IME 组合输入、完整 Tab 循环、Show more 的键盘激活、hover/按下瞬间及正常动态效果模式。它们保持未验证，不据此判失败，也不外推通过。
- 原生设备、原生缩放和屏幕阅读器不属于此次 U2 评分，未为其他单元补零。

无需结构性调整；修正数量文案和 dialog 尺寸约束即可。示例正文未作内容或像素扣分。

实际逐张开图清单，共 **23 张**：

- `U2-loading-1280-dark.png`
- `U2-closed-01-1280-dark.png`
- `U2-reopen-02-1280-dark.png`
- `U2-late-response-03-1280-dark.png`
- `U2-module-error-1280-dark.png`
- `U2-module-retry-1280-dark.png`
- `U2-index-error-1280-dark.png`
- `U2-index-retry-1280-dark.png`
- `U2-loading-375-light.png`
- `U2-module-error-375-light.png`
- `U2-module-retry-375-light.png`
- `U2-index-error-375-light.png`
- `U2-index-retry-375-light.png`
- `U2-supp-open-before-module-01-375-light.png`
- `U2-supp-loading-edit-02-375-light.png`
- `U2-supp-loading-result-03-375-light.png`
- `U2-supp-longlist-04-375-light.png`
- `U2-supp-retry-focus-05-375-light.png`
- `U2-supp-keyboard-retry-06-375-light.png`
- `U2-supp-close-focus-07-375-light.png`
- `U2-supp-close-return-08-375-light.png`
- `U2-supp-new-result-before-old-09-1280-dark.png`
- `U2-supp-old-released-after-new-10-1280-dark.png`
