# R2 评审结论 · U3 摘录

摘自 R2 合并结论 judge.md，只含 U3 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U3 就收录。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.4 | 2.65 | 2.8 | 0.5 | **8.35** | a | b 8.45 |


### 各维度理由

**U3（a）**

- 布局 2.4：对话框宽高上限、≤560px 单列、390 差异卡片正确。扣分原因：差异表被限高裁切（1440 截掉最后一组，390 只完整露出 2/6 项）；切换导出类型时对话框重新居中，Output 选择器位移 12–109px；390 分组行只铺约 105px。
- 视觉 2.65：次级文字、分隔、错误、选中色均对上，颜色值等宽。扣分原因：label、表头、节标题仍是 13px/400；备注框内多一条线；Rust 说明里的调用表达式只有函数名是等宽字。
- 交互 2.8：审阅（版本、名称、配置、分组差异、备注）、未知版本诊断与错误边框、v1 七条损失、CSS 两 profile 四记法、Rust 修订号与 0.2.1 拒收说明、绿色复制成功都走通。扣分原因：Apply 前候选差异无法一眼看全，被裁切且没有滚动提示。
- 细节 0.5：「256 / KiB」拆行；palette 与 color 混用、引号不统一；Auto 没写实际灰阶；备注与表格 token 命名不一致；表头「Mode / token」不准；`16.0` 与代码 `16.` 不一致。


## R2 未关闭差距（U3）

#### UI-125 · [P2] 差异表放在限高的内层滚动区，Apply 前看不全改动，且无滚动提示（U3 · 布局、交互 · 可挽回 a 0.45 / b 0.35）

- 位置：U3-import-review-end-1440-light 在 y≈715 把「Dark · overrides (1)」拦腰截断；U3-import-review-end-390-light 只完整露出 2/6 项；U3-import-review-bottom-1440/390 显示内层（`div.tools-scroll`，`overflow-y:auto`）滚到底后的内容；U3-actions.log 记录两层滚动。
- 现状 → 期望：对话框本体与差异表两层纵向滚动 → X1 Apply 前完整呈现候选差异；X6 对话框内部只有一层纵向滚动。
- 修法：去掉审阅中 `.tools-scroll` 的 `max-height`/`overflow`，表格随对话框本体滚动；极长差异才启用内层滚动，并加滚动阴影与「Showing N of M changes」；≤560px 不嵌套。
- 来源：a 新-1；b 新-1（同一容器、同一帧）。补采确认容器是 `overflow-y:auto`，a 提出的「若是 hidden 应升 P1」不成立。

#### UI-26 · [P2] 对话框 label、表头、节标题字重仍为 400（U3 · 视觉 · 部分解决 · 可挽回 a 0.25 / b 0.25）

- 位置：导入的 Input format、Theme source、Import candidate、Version、Name、配置字段名、Changes (6)、表头、Import notes；导出的 Output、CSS profile、Color notation、Exported code、Required source revision。
- 现状 → 期望：13px/400/#475467 → G3 label 13px/600。
- 修法：数据工具对话框的 label、`dt`、`th`、节标题统一 `font-weight:600`（并确认 600 字面已加载，见 UI-09）。
- 来源：a UI-26；b UI-26。

#### UI-126 · [P2] 切换 Output 时对话框重新垂直居中，选择器跳动 12–109px（U3 · 布局 · 可挽回 a 0.15）

- 位置：U3-export-json/v1/css/rust 对话框上沿 179/70/107/95，Output 选择器 241/132/169/157；导入错误态与审阅态上沿 148/17。
- 现状 → 期望：X6 对话框宽高为视口减 2rem、内部纵向滚动，切换内容时头部与已操作控件不动。
- 修法：导入导出对话框 `height: calc(100dvh - 2rem)` 正文内滚，或顶部锚定（`align-items:flex-start; margin-top:1rem`）。
- 来源：a 新-9（b 在建议 8 提出同一现象，未计分）。

#### UI-127 · [P2] 390 卡片模式下分组行只铺约 105px 并折行（U3 · 布局 · 可挽回 a 0.1 / b 0.15）

- 位置：U3-import-review-end-390-light「Identity (2)」「Light · base (1)」；U3-import-review-bottom-390-light「Dark · / overrides (1)」折两行。
- 修法：≤560px 时分组行 `display:block; width:100%`（Grid 用 `grid-column:1/-1`）。
- 来源：a 新-2；b 新-2。

#### UI-128 · [P2] 配置只列导入值，没有与当前主题的对比（U3 · 交互 · 可挽回 b 0.15）

- 位置：U3-import-review-1440-light y=656–791、U3-import-review-390-light 六个配置字段只有一个值。
- 现状 → 期望：X1 导入先呈现候选差异——版本、名称、配置……
- 修法：每个字段显示「当前 → 导入」，变化的加「Changed」，配置改动并入 Changes 计数。
- 来源：b 新-3。

#### UI-40 · [P3] Rust/CSS 说明里的标识符只有部分等宽，`16.0` 与代码 `16.` 不一致（U3 · 视觉、细节 · 部分解决 · 可挽回 a 0.15 / b 0.05）

- 位置：U3-export-rust-1440-light 第 4 行调用表达式，第 2 行 theme.json、theme.rs、AppHandle；U3-export-css 说明行 data-theme=dark、.dark。
- 来源：a UI-40；b UI-40。

#### UI-42 · [P3] CSS 注释中字体路径仍有 `makepad/widgets/…` 与 `makepad_widgets/…` 两种拼法（U3 · 细节 · 部分解决 · 可挽回 b 0.05）

- 位置：U3-export-css-1440-light y=496 与 y=621、663。
- 来源：b UI-42（部分解决）；a 判已解决。取部分解决。

#### UI-25 · [P3] 「Import candidate」节里 Version/Name 行内排、配置字段上下排，两种排法（U3 · 布局 · 部分解决 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-1440-light y 598–625 与 y 656–791。
- 来源：a UI-25 余项；b 新-8（同一处，b 判 UI-25 本身已解决）。取部分解决。

#### UI-44 · [P3] 损失清单用 palette 而基础配置用 color，引号用法不统一（U3 · 细节 · 部分解决 · 可挽回 a 0.05）

- 位置：U3-export-v1-1440-light 损失框「Brand palette “Pink”」「Scaling intent 100%」。
- 来源：a UI-44 余项（部分解决）；b 判已解决。取部分解决。

#### UI-129 · [P3] 「Import notes」框内多一条横线（U3 · 视觉 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440-light 框边 y=728、框内 y=741；390 同。
- 来源：a 新-3；b 新-5。

#### UI-130 · [P3] 导出对话框按钮行下方留白大于 1.5rem（U3 · 布局 · 可挽回 a 0.05）

- 位置：U3-export-json-390-light 按钮底到下边框 51px；1440 为 32px。
- 来源：a 新-8。

#### UI-131 · [P3] 390 下「256 KiB」被拆到两行（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-4。

#### UI-132 · [P3] v1 损失清单里的 Auto 没写出实际配对的灰阶（U3 · 细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-export-v1-1440-light 第 2 条（R1 写过「resolved mauve」）。
- 来源：a 新-5；b 新-9。

#### UI-133 · [P3] 备注与差异表对 token 的命名和字体不一致，`name` 行的作者文字用了等宽（U3 · 视觉、细节 · 可挽回 a 0.05 / b 0.05）

- 位置：U3-import-review-end-1440/390 备注「base.light.font-serif」（比例字）对表格「Light · base」+ 等宽 `base.primary`；差异表 `name` 行「Rustify Light」「Imported legacy 旧主题」为等宽。
- 来源：a 新-6；b 新-6（同指备注 token 路径与表格写法不一致）。

#### UI-134 · [P3] 差异表首列表头「Mode / token」已不准确（U3 · 细节 · 可挽回 a 0.05）

- 来源：a 新-7。

#### UI-135 · [P3] 吸顶表头没有底边，滚动时下方行像叠字（U3 · 布局 · 可挽回 b 0.05）

- 位置：U3-import-review-bottom-1440-light y≈340。
- 来源：b 新-4。

#### UI-136 · [P3] 改动类型「Changed」与 label 同样式，390 下像无值的 label（U3 · 视觉 · 可挽回 b 0.05）

- 来源：b 新-7。

#### UI-137 · [P3] 22 字节显示为「0.02 / 256 KiB」（U3 · 细节 · 可挽回 b 0.03）

- 位置：U3-import-error-1440-light y=611。
- 来源：b 新-10。

#### UI-138 · [P3] 「Required source revision」行没有对齐表单列（U3 · 布局 · 可挽回 b 0.03）

- 位置：U3-export-rust-1440-light y=215，修订号 x=394 对 Output x=341。
- 来源：b 新-11。


## R2 建议（U3，不扣分）

- U3：
  - JSON 导出按 2 空格缩进；代码框用 `white-space: pre` 加横向滚动（a、b）。
  - Rust 页底部「Download」改名「Download theme.rs」（a）。
  - 0.2.1 拒收 v2 单独成行警示（a）。
  - 差异表颜色值旁加色块（a、b）。
  - Apply 吸底（b）。
  - Cancel 改描边次级按钮（a、b）。
  - 损失框与备注框区分（b）。
  - 补普通 Cargo 构建命令（b）。
