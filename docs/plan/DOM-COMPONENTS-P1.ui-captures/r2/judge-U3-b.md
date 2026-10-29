# U3 Studio 导入与导出 · R2 评审（重建基线）

评审依据：目标包规范要点 X1–X6、G1–G4，R2 的 15 张 `U3-*` 截图、`capture.md` 和 `U3-actions.log`，以及 R1 的 U3 截图和 `judge.md` 指定行。颜色和尺寸是用 ImageMagick 在截图上逐像素取样或量出来的，DPR 为 1。

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.4 | 2.6 | 2.75 | 0.7 | **8.45**（R1 为 7.0） |

各维度理由（每条都说明为什么没到更高一档）：

- **布局 2.4**
  - 已达到 X6 的部分：
    - 对话框几何：1440 下边框 x=208–1231，宽 1024px，即 64rem；导入审阅态 y=16–883，高 868px，即视口减 2rem。
    - 内边距 24px。
    - ≤560px 时表单行改为单列。
    - 复制反馈不再顶开按钮。
    - 审阅区已经结构化。
  - 没到 3 的原因：
    - 差异表被放进一个带 `overflow-y:auto` 的嵌套滚动区（`div.tools-scroll`）。在 1440 下把对话框滚到 Apply 时，末组「Dark · overrides (1)」被拦腰截断；在 390 下 6 项改动约有 4 项藏在内层滚动里，没有任何滚动提示。
    - 390 卡片模式下，分组行只有约 105px 宽，并且折成两行。

    这两处都是一眼能看出的结构偏差。
- **视觉 2.6**
  - 已对齐 G3 的部分：
    - 分隔线 #d0d5dd / #eaecf0。
    - 次级文字 #475467。
    - 错误色 #b42318，文本域有错误边框。
    - 选中项 #eff8ff / #175cd3。
    - 颜色值等宽 12px。
    - 代码 13px / 21px。
  - 没到 3 的原因：
    - 对话框 label、表头和小标题仍不是 13px/600。实测「Import candidate」中 I 的竖干约 1.44px（0.111em），14px/400 正文「Imported」中 I 的竖干约 1.49px（0.106em），两者属于同一字重档。
    - 「Import notes」框里多了一条分隔线。
    - 名称和 token 路径的字体用法前后不一致。
- **交互与状态 2.75**
  - 能走通的部分：
    - 导入审阅能看到版本、名称、配置、分组差异和诊断。
    - 未知版本给出正确诊断（「Unsupported Theme JSON version 999…」），并有 Technical details。
    - 兼容 v1 先列出 7 条损失，覆盖 X2 的六类。
    - CSS 有两种 profile、四种颜色记法，并写明「resolved values」。
    - Rust 说明写出所需修订号、0.2.1 不能消费 v2、预览模式 Light。
    - 复制成功后在按钮同行显示绿色「Copied to clipboard.」。
  - 没到 3 的原因：
    - 配置只列导入值，没有与当前值对比，X1 要求呈现「配置」的差异。
    - 嵌套滚动区让用户可能还没看完全部改动就点了 Apply。
- **细节 0.7**：还有若干小差异：
  - Rust 说明里的调用表达式只有函数名用了等宽字。
  - CSS 注释中字体路径仍有两种拼法。
  - v1 损失清单写 Auto，但不写解析出的中性色。
  - 22 字节显示成「0.02 / 256 KiB」。
  - 修订号行没有对齐表单列。

## 2. 上一轮差距复核表

| 编号 | R1 问题 | R2 状态 | 依据（R2 截图） |
| --- | --- | --- | --- |
| UI-24 | 未知版本给出误导性诊断 | **已解决** | `U3-import-error-1440`：显示「Unsupported Theme JSON version 999. This Studio reads v1 and v2.」，下面是 ▸ Technical details。`aria-invalid` 和中文看不出 |
| UI-25 | 审阅结果是无标签散文 | **已解决** | `U3-import-review-1440`：有分隔线、小标题「Import candidate」、Version/Name 和 6 个配置字段的 label/value，以及「Changes (6)」。字重的残留并入 UI-26 |
| UI-26 | label、表头、备注标题为 14px/400 | **部分解决** | 字号已是 13px，颜色 #475467；字重仍是常规档（见上方竖干测量），不是 600 |
| UI-27 | 行线、备注框、代码框为 #858F9E | **已解决** | 行线 #eaecf0（y=360、397）；备注框、损失框、代码框、文本域都是 #d0d5dd |
| UI-28 | Rust 说明没写所需版本 | **已解决** | `U3-export-rust-1440`：「Required source revision f5613b47…」，代码首行注释同步 |
| UI-29 | 差异表颜色值不是等宽字 | **已解决** | 「#1570ef」7 个字符宽 49px，每字符 7px，即等宽 12px |
| UI-30 | 错误文字 #D92D20，文本域无错误边框 | **已解决** | 文字和文本域边框都是 #b42318（y=365） |
| UI-31 | 配置写成 snake_case 或 `null` | **已解决** | Appearance / Brand color / Neutral color / Panel background / Radius / Scaling，值为 Inherit / Custom / 100% |
| UI-32 | v1 损失清单少 appearance 和 scaling | **已解决** | `U3-export-v1-1440` 有 7 条，包含 Appearance intent 和 Scaling intent |
| UI-33 | 差异表不区分改动类型 | **已解决**（新增和删除怎么显示待核实） | 有 Change 列（Changed）和分组计数（Identity (2)、Light · base (1)…）；本例数据里只有 Changed |
| UI-34 | 390 下 token 在字符中间断行 | **已解决** | 390 改成卡片，`overrides.primary` 完整；但卡片模式引出新-2 |
| UI-35 | 「Saved on this device」与复制反馈夹着按钮 | **已解决** | 导出对话框里已没有库状态；反馈和按钮在同一行 |
| UI-36 | 复制反馈让按钮下移 | **已解决** | 复制前后 Copy code 都在 y≈756 |
| UI-37 | 复制成功没有成功语义 | **已解决** | 「Copied to clipboard.」颜色为 #067647 |
| UI-38 | 次级文字是 #667085 | **已解决** | 字节计数、label 和说明文字都是 #475467 |
| UI-39 | 字节计数单位不统一 | **已解决** | 统一为 KiB（「3.25 / 256 KiB」）；精度问题另记新-10 |
| UI-40 | Rust 说明与代码字面量不一致，标识符用比例字 | **部分解决** | `mount_exported_preview` 已是等宽字，但参数「(ThemeMode::Light, &font_context(16.0, true))」仍是比例字；说明里写 `16.0`，代码里写 `16.` |
| UI-41 | Tailwind CSS 与 Rustify scoped 命名冲突 | **已解决** | 现在是「CSS · resolved values」「Rustify scoped · data-theme」「Tailwind v4 · .dark」 |
| UI-42 | 字体路径有两种拼法 | **部分解决** | 加了说明（许可证文件相对 SDK 源码，字体资源相对部署后的 CSS），但 `makepad/widgets/…` 和 `makepad_widgets/…` 两种拼法仍同时出现 |
| UI-43 | 导出表单行距 12px 与 8px 不一 | **已解决** | Output / CSS profile / Color notation 的行距都是 48px（y=186/234/282） |
| UI-44 | 损失清单取值写法生硬 | **已解决** | 例如「Brand palette “Pink”. v1 keeps final colors, without palette selection.」；Auto 没写解析结果，另记新-9 |

## 3. 差距清单（先按严重度，再按可挽回分排序）

### 新-1 · [P2] 差异表放在嵌套滚动区里，改动被截断或隐藏，且没有滚动提示（回归）

- **单元与位置**
  - `U3-import-review-end-1440-light`：表格区下沿 y≈715，把「Dark · overrides (1)」分组行拦腰截断，它的 `overrides.primary` 行完全不可见，下面紧接着 Import notes。
  - `U3-import-review-end-390-light`：内层只露出 Identity 的两张卡片和半张 `base.primary`。
  - `U3-actions.log`：`div.tools-scroll overflow-y:auto`，1440 下 scrollTop 0→56，390 下 0→589。
  - R1 同一帧的 6 行全部可见。
- **维度**：布局（兼交互）
- **现状 → 期望**
  - 现状：对话框本体（`div.relative.w-full`）和差异表（`div.tools-scroll`）是两层纵向滚动。只滚对话框就能拿到 Apply，却看不全 6 项改动；headless 截图里也没有滚动条、阴影或「还有更多」之类提示。
  - 期望：X6「内部纵向滚动」指对话框内部只有一层纵向滚动；X1 要求 Apply 之前完整呈现候选差异。
- **修法**
  - 去掉 `.tools-scroll` 在导入审阅中的 `max-height` 和 `overflow-y:auto`，让表格在对话框本体里自然流动。
  - 如果要防极长差异（例如超过 200 行），只在超过阈值时启用内层滚动，同时加上下滚动阴影和「Showing N of M changes」文字。≤560px 时任何情况都不嵌套。
  - 验收：1440 和 390 只滚对话框本体，就能依次看到全部 6 项、Import notes 和 Apply。
- **可挽回**：0.35

### UI-26 · [P2] 对话框 label、表头、小标题仍非 13px/600（部分解决）

- **单元与位置**
  - 导入：「Input format」「Theme source」「Version」「Name」「Appearance…Scaling」「Import candidate」「Changes (6)」「Import notes」。
  - 表头：「Mode / token / Change / Current / Imported」；390 卡片里的「Current / Imported」。
  - 导出：「Output」「CSS profile」「Color notation」「Exported code」「Required source revision」。
- **维度**：视觉
- **现状 → 期望**
  - 现状：13px、#475467、常规字重。13px「I」竖干约 1.44px，与 14px/400 正文同属一档；600 在 13px 下约 1.6px 以上。
  - 期望：G3「label 13px / 600」。
- **修法**
  - 把数据工具对话框共用的 label 和表头样式改为 `font-weight:600`（Tailwind `font-semibold`），保留 13px 和 #475467。
  - 小标题「Import candidate」「Changes (6)」「Import notes」同样 13px/600。如需与字段 label 拉开层级，见建议 6。
- **可挽回**：0.25

### 新-2 · [P2] 390 卡片模式下分组行只有约 105px 宽并折行

- **单元与位置**
  - `U3-import-review-end-390-light`：「Identity (2)」「Light · base (1)」两个灰底块宽约 105px。
  - `U3-import-review-bottom-390-light`：「Dark · overrides (1)」折成「Dark ·」「overrides (1)」两行；灰底 #f9fafb 只盖住窄格，右侧留白。
- **维度**：布局
- **现状 → 期望**
  - 现状：分组行沿用了表格单元格的宽度。
  - 期望：G2「≤560px 导入导出表单行单列」；分组标题应占满卡片宽度并保持单行，和 1440 下整行灰底一致。
- **修法**
  - 在 ≤560px 的卡片样式里，让分组行的 `tr` 和其中的 `th/td` 设 `display:block; width:100%`（Grid 写法为 `grid-column:1/-1`），灰底占满整行。
  - 分组文字不设固定宽度。
- **可挽回**：0.15

### 新-3 · [P2] 配置只列导入值，没有呈现与当前主题的差异

- **单元与位置**：`U3-import-review-1440-light` y=656–791、`U3-import-review-390-light`。六个配置字段只有一个值（Inherit / Custom / 100%），看不出当前主题（Rustify Light）在这些字段上原本是什么，也看不出哪些会变。「Changes (6)」只统计身份和 token。
- **维度**：交互
- **现状 → 期望**
  - 现状：配置只显示候选值。
  - 期望：X1「导入先呈现候选差异…版本、名称、配置…」，配置也属于差异。
- **修法**
  - 配置区每个字段显示「当前 → 导入」，例如 `Blue → Custom`，当前值用 #475467。
  - 有变化的字段加文字标识「Changed」，没变的写「Unchanged」或弱化显示。
  - 配置改动并入「Changes (N)」计数，或单列「Configuration (k changed)」。
- **可挽回**：0.15

### 新-4 · [P3] 吸顶表头没有底边，滚到下面的行看起来像叠字

- **单元与位置**：`U3-import-review-bottom-1440-light` y≈340。表头「Mode / token…」下方直接露出被切掉上半截的「id Changed rustify-light review-legacy」，表头下沿 y=330–345 全是 #ffffff，没有分隔线。
- **维度**：布局
- **现状 → 期望**
  - 现状：表头没有底边，与被裁切的行之间没有分隔。
  - 期望：G3 壳层用细分隔线分组。
- **修法**：给吸顶的 `thead th` 加 `box-shadow: inset 0 -1px 0 #eaecf0`。`border-collapse` 下吸顶单元格的 border 会跟着行滚走，所以要用阴影；背景保持 #fff。
- **可挽回**：0.05

### 新-5 · [P3] Import notes 框内多一条分隔线（双线）

- **单元与位置**：`U3-import-review-end-1440-light`。框的上边框在 y=728（#d0d5dd），框内 y=741 又有一条 #eaecf0 横线，下面才是「Import notes」。390 两帧同样存在（y≈643 / 619）。
- **维度**：视觉
- **现状 → 期望**
  - 现状：一个分组出现两条横线。
  - 期望：G3 壳层扁平，一个分组只用一条分隔线。
- **修法**：去掉备注框内部区块的 `border-top`（或 `<hr>`），只保留外框。
- **可挽回**：0.05

### 新-6 · [P3] 名称和 token 路径的字体用法前后不一致

- **单元与位置**
  - 差异表 `name` 行的「Rustify Light」「Imported legacy 旧主题」用了等宽字，但这是作者文字，不是值。
  - Import notes 中的 token 路径 `base.light.font-serif` 用比例字，而表格里的 token 路径用等宽字。
- **维度**：视觉
- **现状 → 期望**
  - 现状：同类内容一处等宽、一处比例。
  - 期望：G3「颜色值等宽 12px」，等宽字只留给值和 token。
- **修法**：`name` 行的值用正文字体 14px；备注里的 token 路径包进 `<code>`，用等宽 12px。
- **可挽回**：0.05

### 新-7 · [P3] 改动类型「Changed」的样式与 label 相同，390 下像一个没有值的 label

- **单元与位置**
  - 1440 的 Change 列，以及 390 卡片里 token 名下面单独一行的「Changed」。
  - 它和「Current」「Imported」两个 label 都是 13px #475467。
- **维度**：视觉
- **现状 → 期望**
  - 现状：改动类型读起来像标签而不是值。
  - 期望：X1 的新增、删除、修改要一眼可读。
- **修法**
  - 用正文色 #1d2939 显示，或做成文字标签（文字加浅底），Changed、Added、Removed 各有文字。
  - 390 下把类型放到 token 名同一行的右侧，例如「base.primary · Changed」。
- **可挽回**：0.05

### 新-8 · [P3] Version/Name 是行内 label，配置字段是上下堆叠，同一区块两种排法

- **单元与位置**：`U3-import-review-1440-light` y=598/625。Version 和 Name 的 label 在左（label 列仅约 60px），值在右；下面 6 个配置字段是 label 在上、值在下的两列网格（x=233 / 732）。
- **维度**：布局
- **现状 → 期望**
  - 现状：同一个「Import candidate」区块用了两种 label 排法。
  - 期望：G2 壳层用分组行，同组排法一致。
- **修法**：把 Version、Name 放进同一个两列网格，同样 label 在上，例如第一行 Version | Name，后面是三行配置。
- **可挽回**：0.05

### UI-40 · [P3] Rust 说明里的代码表达式只有一半是等宽字（部分解决）

- **单元与位置**：`U3-export-rust-1440-light` y=307。`mount_exported_preview` 是等宽字，参数「(ThemeMode::Light, &font_context(16.0, true))」是比例字。第一行说明里的 `theme.json`、`theme.rs`、`AppHandle` 也是比例字。字面量在说明里写 `16.0`，代码里写 `16.`。
- **维度**：细节
- **现状 → 期望**
  - 现状：同一个调用表达式用了两种字体。
  - 期望：同一表达式只用一种字体，字面量写法与代码一致。
- **修法**：整个调用表达式和三个标识符各自包进 `<code>`（等宽 12px）；字面量统一为代码里的写法。
- **可挽回**：0.05

### UI-42 · [P3] CSS 注释中字体路径仍有两种拼法（部分解决）

- **单元与位置**：`U3-export-css-1440-light` y=496 写 `makepad/widgets/resources/FONT-LICENSES.md`，y=621、663 写 `makepad_widgets/resources/…ttf`。
- **维度**：细节
- **现状 → 期望**
  - 现状：虽然加了「相对源码 / 相对部署」的解释，读者仍会当成笔误。
  - 期望：X4 写清外部字体资源。
- **修法**
  - 许可证文件和字体资源一起部署，统一用部署后的相对路径 `makepad_widgets/resources/…`。
  - 或者在每个路径前标明前缀「source:」「deployed:」。
- **可挽回**：0.05

### 新-9 · [P3] v1 损失清单的 Auto 没写解析出的中性色（R1 有，本轮丢了）

- **单元与位置**：`U3-export-v1-1440-light` 第 2 条「Neutral palette “Auto”. v1 cannot retain Auto pairing…」。R1 同一处写的是「auto (resolved mauve)」。
- **维度**：细节
- **现状 → 期望**
  - 现状：只写 Auto。
  - 期望：C2「auto 显示实际配对的灰阶名」；损失清单应说明将固化成哪一族。
- **修法**：改为「Neutral palette “Auto” (resolved: Mauve)」，中英同步。
- **可挽回**：0.05

### 新-10 · [P3] 22 字节显示为「0.02 / 256 KiB」

- **单元与位置**：`U3-import-error-1440-light` y=611。
- **维度**：细节
- **现状 → 期望**
  - 现状：小输入显示成 0.02，接近 0，看不出实际大小。
  - 期望：小于 1 KiB 时也能读出实际大小。
- **修法**：小于 1 KiB 显示「22 B / 256 KiB」，不小于 1 KiB 时保持两位小数。
- **可挽回**：0.03

### 新-11 · [P3] 「Required source revision」行没有对齐表单列

- **单元与位置**：`U3-export-rust-1440-light` y=215。修订号从 x=394 开始，上方 Output 下拉框从 x=341 开始。
- **维度**：布局
- **现状 → 期望**
  - 现状：修订号与上方控件不在同一条竖线上。
  - 期望：G2 壳层分组行对齐。
- **修法**：让该行使用与 Output 相同的 label 列宽，值从 x=341 开始，label 过长就折行；或改成 label 在上的堆叠排法。
- **可挽回**：0.03

**可挽回合计**：0.35 + 0.25 + 0.15 + 0.15 + 0.05 × 8 + 0.03 × 2 = **1.36**。10 − 8.45 = 1.55，差 0.19，在 0.5 以内。

## 4. 建议（规范里没有依据，不扣分）

1. **JSON 导出格式化**：v2 和 v1 导出都是压成一行的 JSON，折行落在键名中间，例如 `"accent-` / `foreground"`、`"chart-` / `1"`，390 下尤其难读。建议按 2 空格缩进格式化，与导入文本域里的格式一致；或者用 `white-space:pre` 配合横向滚动。
2. **Apply 常驻可见**：导入对话框底部的 Apply import / Cancel 可以做成吸底动作区。目前 1440 审阅首帧看不到 Apply，要先滚动。
3. **Cancel 样式**：Cancel 现在是纯文字链接样式，与 Choose file、Review changes 的描边按钮不一致。建议改成描边次级按钮。
4. **字段边框**：下拉框边框是 #858f9e，文本域和代码框是 #d0d5dd，同一表单里两种字段边框。建议统一。
5. **损失框与备注框区分**：v1 损失框和 Import notes 框完全同款（#f9fafb 底、#d0d5dd 边框）。建议损失框加上「7 losses」小标题或警示色，与中性备注区分。
6. **标题与 label 的层级**：小标题（Import candidate / Changes / Import notes）可以用正文色 #1d2939，字段 label 保持 #475467，拉开层级。
7. **颜色值色块**：差异表的颜色值旁可以加 12px 色块，方便直接比较新旧颜色。
8. **对话框位置**：错误态对话框变矮后重新居中，顶边从 y=17 移到 y=149。建议对话框顶部固定，避免状态切换时跳动。这是对两帧位置的对比，动效本身没有拍到。
9. **外部接入者的构建命令**：Rust 说明里的构建命令用的是 `mbx build …`。可以补一个普通 Cargo 的等价命令，方便外部接入者。

## 5. 看不出的格子

- **键盘焦点（G4）**：所有 U3 帧的焦点都来自指针或程序。`capture.md` 记录焦点在 `import-review`、`export-kind`、`export-copy` 等元素上，但截图里没有 focus-visible 样式。下拉框、按钮、文本域、Technical details 的键盘焦点能否看见，判断不了。
- **中文态（G6）**：没有中文的导入导出帧，看不出错误、损失、说明文案是否已中英同步。
- **悬停和按下态**：没有采。
- **X1 其他分支**：取消、语法错误、超限、不支持的颜色输入时是否保留当前主题和历史，以及 Apply 之后的结果，都没有采。
- **Added / Removed 的样式**：样本数据里只有 Changed，看不出新增、删除，以及缺失一侧显示「—」的样子。
- **复制失败和下载（X5）**：只拍到复制成功；复制失败时是否保留可选文本与下载路径、下载是否实际发起，都没有采。
- **Rust 说明里的字体资源（X4）**：`U3-export-rust-*` 只拍到代码开头和末尾。注释说「Copy the listed same-origin font files」，但字体清单是否在没拍到的中段，看不出。
- **Technical details 展开后的内容**：未展开。
- **文本域的 `aria-invalid`**：截图上无法判断。
- **390 下 v1 / CSS / Rust 导出**：不在采集矩阵内。
- **对话框高度的口径**：导出对话框和错误态导入对话框都比「视口减 2rem」矮。沿用 R1 的读法，按最大尺寸处理，不扣分。

## 6. 总评

- **分数**：U3 是本次唯一评审的单元，**8.45**，比 R1 的 7.0 高。R1 的 21 条差距里 16 条已解决，3 条部分解决（UI-26、UI-40、UI-42）。剩下的 2 条（UI-33、UI-34）主体也已解决：UI-33 的新增和删除怎么显示还要用新数据核实，UI-34 的卡片化改法引出了新-2。
- **主要失分**：
  1. 新-1：差异表被做成嵌套滚动区。这是本轮回归，1440 和 390 都会把改动藏在内层滚动里，是剩余最大的单项（0.35）。
  2. 新-2：390 卡片模式的分组行宽度不对。
  3. 新-3：配置没有显示与当前值的差异。
  4. UI-26：对话框 label、表头、小标题的字重仍不是 600。
- **改动范围**：不需要推倒对话框或换组件。需要的局部结构调整有三处：
  - 撤掉 `.tools-scroll` 的内层滚动，让对话框本体成为唯一的纵向滚动。
  - 卡片模式的分组行改成整行块级元素。
  - 配置区从「只列导入值」改成「当前 → 导入」的对比结构。

  其余都是样式和文案层面的修改。
