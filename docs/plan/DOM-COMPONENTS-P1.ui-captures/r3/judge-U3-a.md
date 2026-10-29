# DOM-COMPONENTS-P1 UI 验收 R3：U3（Studio 导入与导出）评审

- 依据：目标包「单元」U3 行、规范要点 X1–X6 与 G1–G4，以及「变更记录」全文。R2 之后的变更只涉及 U5、U7 的对比度口径（UI-156）和 U7 的采集方式（UI-97、UI-88/89），都不改变 U3 的期望，所以 U3 沿用 R2 的尺度评分。
- 材料：
  - 本轮：`r3/U3-*.png`（17 张）、`r3/U3-actions.log`、`r3/capture.md`。
  - 上一轮：`r2/U3-*.png`、`r2/U3-actions.log`、`r3/judge-inputs/r2-judge-U3.md`。
  - 像素读数用 ImageMagick 逐列、逐行取样，坐标单位为 CSS px（DPR 1）。
- 本单元为规范模式，没有目标图，也没有差分图。

---

## 1. 评分表

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.85 | 2.85 | 2.85 | 0.5 | **9.05**（R2 为 8.35） |

**各维度为什么不是更高一档**

- **布局 2.85**：上一轮的主要布局问题都已修好：
  - 审阅区只剩对话框本体一层滚动，差异表完整；
  - 对话框顶部锚定在 y=16，切换 Output 时选择器固定在 y=78–113；
  - 390 下分组行铺满整行；
  - 修订号与表单列对齐。

  不到 3 的原因：导出对话框按钮行到下边框是 32px，导入对话框是 24px，X6 规定的 1.5rem 只有导入做到了；另外 Changed 徽标比列首文字左缘右移 3px。
- **视觉 2.85**：已对上的部分：
  - label、表头、节标题都是 13px/600；
  - 备注框内的多余横线已去掉；
  - Changed 改成带边框的徽标；
  - 标识符、调用表达式、选择器都换成等宽字；
  - 选中色 #eff8ff/#175cd3、分隔线 #eaecf0/#d0d5dd、错误色 #b42318、次级文字 #475467 都与 G3 一致。

  不到 3 的原因：v1 损失清单和 Import notes 正文由 R2 的 14px/21px 降到 13px/19.5–20px，不在 G3 字级里（body 14px/1.5）；备注里的字体族值仍是比例字，在 390 下从连字符处断行。
- **交互与状态 2.85**：主路径都走通了：
  - 合法 v1 审阅：版本、名称、配置的「→」对照、6 项差异、备注、Apply/Cancel；
  - 未知版本：错误文字、红色边框、Technical details；
  - 导出：v2 JSON、v1 七条损失、CSS 两种 profile 和四种记法、Rust 修订号与 0.2.1 拒收说明、复制成功提示。

  不到 3 的原因：配置对照没有注明哪边是当前、哪边是导入。同一个「→」在 Version 行表示格式迁移，在配置行表示当前→导入，Name 行又只列导入值，容易误读。
- **细节 0.5**：R2 的 6 处细节大多已修，但本轮逐处比对仍有若干小差异：
  - 字节计数「3332 / 262144 bytes」没有千分位，和说明里的「256 KiB」单位不一；
  - Rust 说明「theme.json · theme.rs」有歧义，「Keep AppHandle.」丢了「alive」；
  - 备注与差异表的 token 命名方式不一致；
  - CSS 提示写 `data-theme=dark`，代码里是 `[data-theme="dark"]`；
  - 徽标偏移 3px；
  - 「ui-」处断行。

---

## 2. 上一轮差距复核表

| 编号 | R2 问题 | R3 状态 | 证据（R3） |
| --- | --- | --- | --- |
| UI-125 | 差异表在限高内层滚动区，Apply 前看不全 | **已解决** | `U3-actions.log` 只滚动 `div.relative.w-full`（0→626 / 0→1429）和 textarea，不再有 `div.tools-scroll`。`U3-import-review-end-1440` 一帧内能看到 Identity (2) 到 Dark · overrides (1) 的全部 6 行、Import notes 和 Apply。390 的 end/bottom 帧连续展示 Light·base 到 Dark·overrides 与 Apply |
| UI-26 | 对话框 label、表头、节标题字重 400 | **已解决** | Input format、Theme source、Import candidate、Version、Name、Appearance…Scaling、Changes (6)、Field / token…Imported、Import notes、Output、CSS profile、Color notation、Exported code、Required source revision 都是 13px 粗体 |
| UI-126 | 切换 Output 时对话框重新居中、选择器跳动 | **已解决** | 导出 JSON/v1/CSS/Rust 四帧的对话框上边框都在 y=16，Output 选择器都在 y=78–113。导入审阅、导入错误也在 y=16 |
| UI-127 | 390 分组行只铺约 105px 并折行 | **已解决** | `U3-import-review-end-390` 的「Light · overrides (1)」「Dark · overrides (1)」等行背景铺满 x=41–349，单行显示 |
| UI-128 | 配置只列导入值、没有对比 | **已解决**（主体） | `U3-import-review-1440` y=658–803 显示「Inherit → Inherit」「Custom → Custom」「100% → 100%」。配置有变化时的 Changed 标记和计数本轮数据没触发，见第 5 节。对照缺图例见新-1 |
| UI-40 | Rust/CSS 说明里标识符只部分等宽，`16.0` 与 `16.` 不一致 | **已解决** | `U3-export-rust-1440` y=167 的 `theme.json`、`theme.rs`、`AppHandle`，以及 y=247 整个 `mount_exported_preview(ThemeMode::Light, &font_context(16., true))` 都是等宽，`16.` 与代码一致。`U3-export-css-1440` y=230 的 `data-theme=dark`、`.dark` 也是等宽 |
| UI-42 | CSS 注释中 `makepad/widgets/…` 与 `makepad_widgets/…` 两种拼法 | **已解决** | `U3-export-css-1440` y=386–427 写明许可证文档「is in the SDK source checkout (not a deployed URL): makepad/widgets/…」；y=365 写明字体 URL「relative to this deployed CSS file」，对应 y=552/594 的 `makepad_widgets/…`。两种拼法现在明确指两个不同位置 |
| UI-25 | Version/Name 行内排、配置字段上下排 | **已解决** | `U3-import-review-1440` 的 Version、Name 和六个配置字段同用 label 列 x=233 / 值列 x=405 |
| UI-44 | 损失清单 palette 与 color 混用、引号不统一 | **已解决** | `U3-export-v1-1440` 写作「Brand color “Pink”」「Neutral color “Auto (Mauve)”」「Scaling intent “100%”」 |
| UI-129 | Import notes 框内多一条横线 | **已解决** | `U3-import-review-bottom-1440` 备注框 y=719–814 内没有横线，390 也一样 |
| UI-130 | 导出对话框按钮行下方留白大于 1.5rem | **部分解决** | 390 由 51px 降到 32px；1440 仍是 32px。导入对话框为 24px，见下方差距清单 |
| UI-131 | 390 下「256 KiB」拆行 | **已解决** | `U3-import-review-390` y=108 的「256 KiB.」在同一行；计数改为字节 |
| UI-132 | v1 损失里 Auto 没写实际灰阶 | **已解决** | 写作「Neutral color “Auto (Mauve)”」 |
| UI-133 | 备注与差异表 token 命名、字体不一致；name 行作者文字等宽 | **部分解决** | 字体已修：备注路径改为等宽，name 行「Rustify Light」「Imported legacy 旧主题」改为比例字。命名方式仍不一致，见下方差距清单 |
| UI-134 | 表头「Mode / token」不准 | **已解决** | 改为「Field / token」 |
| UI-135 | 吸顶表头无底边，滚动时像叠字 | **已解决** | 内层滚动区已移除；表头下方有 #d0d5dd 底线（`U3-import-review-bottom-1440` y≈277），不再出现叠压 |
| UI-136 | 「Changed」与 label 同样式 | **已解决** | 改为带边框的浅灰底徽标（1440 与 390） |
| UI-137 | 22 字节显示为「0.02 / 256 KiB」 | **已解决** | `U3-import-error-1440` y=479 显示「22 / 262144 bytes」。新格式本身的问题见新-2 |
| UI-138 | 「Required source revision」没对齐表单列 | **已解决** | `U3-export-rust-1440` 修订号左缘 x=405，与 Output 选择器 x=405 一致 |

---

## 3. 差距清单（先按严重度，再按可挽回分排序）

#### 新-1 · [P2] 配置对照的「→」没有图例；Version 行的「→」含义与配置行不同；Name 行只有导入值（U3 · 交互、细节 · 可挽回 0.2）

- **位置**：
  - `U3-import-review-1440-light` y=598–803：Version「JSON v1 → v2」、Name「Imported legacy 旧主题」、Appearance「Inherit → Inherit」、Brand color / Neutral color / Panel background / Radius「Custom → Custom」、Scaling「100% → 100%」；
  - `U3-import-review-390-light` y=648–781；
  - `U3-import-review-end-1440`、`U3-import-review-bottom-1440` 顶部。
- **现状**：同一个「→」在 Version 行表示「源格式 v1 迁移到 v2」，在配置行表示「当前 → 导入」，但页面上没有任何字样说明。Name 行只列导入值，与下面配置行的成对写法不一致。未变化的项写成「X → X」，没有「Unchanged」之类的标识。下方差异表已经用「Current / Imported」列头，上方对照却没有，两处表达不一致。
- **期望**：X1「导入先呈现候选差异——版本、名称、配置……」。审阅者应能从截图直接读出哪一侧是当前主题、哪一侧是候选，以及哪些配置会变。
- **修法**：
  1. 把配置对照改成与 Changes 表同一套列结构：表头「Field | Current | Imported」，沿用 `th` 的 13px/600/#475467 样式与底线；值列沿用 x=405 起的对齐。
  2. 当前值等于导入值的行只显示一个值，旁边加 12px #475467 的「Unchanged」caption；不同的行加与差异表相同的「Changed」徽标，并计入 Changes (N)。
  3. Version 行改写为「Source: Theme JSON v1 · Saved as v2」，或用 label「Source version」「Stored as」分两行，不再与配置对照共用「→」。
  4. Name 行或者按同一列结构写「Rustify Light | Imported legacy 旧主题」，或者删掉（Identity 组的 `name` 行已经呈现）。
  5. 390 卡片模式下同样用「Current / Imported」两列小标题，与差异卡片一致。

#### UI-130 · [P3] 导出对话框按钮行下方留白 32px，导入对话框为 24px（U3 · 布局 · 部分解决 · 可挽回 0.1）

- **位置**：
  - `U3-export-json-1440`：按钮 y=485–516，内侧白区到 y=548，边框在 y=549，留白 32px；
  - `U3-export-rust-1440`：按钮 673–704，边框 737，留白 32px；
  - `U3-export-json-390`：按钮 516–547，边框 580，留白 32px；
  - 对照：`U3-import-review-bottom-1440` 的 Apply 827–858、边框 883，留白 24px；390 为 771–802 / 827，留白 24px。
- **现状 → 期望**：导出 32px、导入 24px → X6「1.5rem padding」，两个对话框都应为 24px。
- **修法**：
  - 导出按钮行之后多了一个 8px 间距，疑为空的复制/下载状态行加了 gap 或 margin（`U3-export-copied-1440` 出现「Copied to clipboard.」后底部变为约 30px）。
  - 状态行保留为 `aria-live` 节点，但空时 `margin:0; min-height:0`，或者用 `.status:empty{display:none}`；间距只在非空时用 `margin-top:8px` 给出。
  - 或者把状态文字放进按钮行（`display:flex; align-items:center; gap:8px`），放在 Download 右侧。

#### 新-4 · [P3] v1 损失清单与 Import notes 正文降为 13px/400，不在 G3 字级里（U3 · 视觉 · 回归 · 可挽回 0.1）

- **位置**：
  - `U3-export-v1-1440` y=148–297：引导句和 7 条损失，行距 19.5px，x-height 9px；
  - `U3-import-review-bottom-1440` y=771/791 两条备注，行距 20px；
  - 390 的同一个框。
- **对照**：R2 同一区域为 14px/21px（`r2/U3-export-v1-1440` y=203–362，x-height 10px）。同一句引导文字 R3 宽 709px、R2 宽 770px，比例 0.92，约等于 13/14；而对话框说明行（y=88）两轮宽度只差 1%。
- **现状 → 期望**：13px/400 → G3「body 14px / 1.5」。13px 在 G3 里只用于 label 13px/600。
- **修法**：把损失框与备注框里的 `p`、`li` 设为 body token 14px/1.5（0.875rem）。如果有意把它们做成辅助说明，应改用 caption 12px + #475467，不要另设 13px 档。

#### 新-2 · [P3] 字节计数「3332 / 262144 bytes」没有千分位，和说明里的「256 KiB」单位不一（U3 · 细节 · 可挽回 0.1）

- **位置**：
  - `U3-import-review-1440` y=479「3332 / 262144 bytes」；
  - `U3-import-error-1440` y=479「22 / 262144 bytes」；
  - `U3-import-review-390` y=530；
  - 对照：同一对话框 y=88 写「up to 256 KiB」。
- **现状**：同一个上限在一个对话框里出现两种单位；六位数没有分组，不易读。
- **期望**：数字格式统一、可读（细节维度要求逐处比对一致）。
- **修法**：用 `Intl.NumberFormat(locale)` 加千分位；上限与说明使用同一单位。例如「3,332 bytes of 256 KiB」，或「3.3 KiB / 256 KiB」（不足 1 KiB 时显示「22 bytes / 256 KiB」）。中文界面写「3,332 字节 / 256 KiB」。

#### 新-3 · [P3] Rust 接入说明文案有歧义：「theme.json · theme.rs」读成并列文件；「Keep AppHandle.」丢了「alive」（U3 · 细节 · 可挽回 0.1）

- **位置**：`U3-export-rust-1440`、`U3-export-copied-1440` 的 y=167：「Place the matching JSON beside the Rust source: theme.json · theme.rs. … Keep AppHandle. Fonts remain external.」
- **对照**：R2（`r2/U3-export-rust-1440` y=246）写作「Download the matching theme.json beside theme.rs … Keep AppHandle alive.」
- **现状**：
  - 「·」把两个文件名并列，读不出谁放在谁旁边；
  - 「Keep AppHandle.」只剩动词，读不出要求的是什么：保持存活？保留引用？
- **期望**：X4 的接入说明要写清楚。
- **修法**：改为「Put `theme.json` next to `theme.rs`.」「Keep the returned `AppHandle` alive while the UI is mounted.」。中文文案同步。

#### 新-6 · [P3] 备注中的字体族值是比例字，390 下「ui-serif」从连字符处断开（U3 · 视觉、细节 · 可挽回 0.08）

- **位置**：
  - `U3-import-review-end-390` y≈699/719 与 `U3-import-review-bottom-390` y≈675/695：「Requested ui-」换行后接「serif; preview uses Noto Serif」；
  - 1440 的 y=771/791：路径 `base.light.font-serif` 是等宽，紧跟的 `ui-serif`、`Noto Serif` 是比例字。
- **现状 → 期望**：同一条备注里 token 路径用代码样式、CSS 值用正文样式，值被断成「ui-」和「serif」→ 与 UI-40 已统一的规则一致：标识符和 CSS 值用等宽 inline code，且不在值内部断行。
- **修法**：requested/resolved 两个值用与路径相同的 inline code 样式（12px 等宽），加 `white-space:nowrap`；或者沿用 CSS 导出注释的引号写法：Requested "ui-serif" → resolved "Noto Serif"。

#### UI-133 · [P3] 备注与差异表对 token 的命名方式不一致（U3 · 细节 · 部分解决 · 可挽回 0.06）

- **位置**：
  - `U3-import-review-bottom-1440` y=771「`base.light.font-serif`」：区段.模式.token；
  - 同帧 y=414/452 是分组头「Light · base (1)」加行名「`base.primary`」：模式写在组头，行名又重复「base」；
  - 390 同样。
- **现状 → 期望**：两种路径写法混用 → 同一对话框里同一类 token 只用一种命名。
- **修法**：二选一，并在备注、差异表、390 卡片三处统一：
  - 差异表行名只写 `primary`（组头已有「Light · base」），备注写「Light · base · `font-serif`」；
  - 或者行名、备注都写完整路径 `base.light.primary`。

#### 新-7 · [P3] CSS 提示中的「Scoped dark selector: data-theme=dark」不是可用的选择器（U3 · 细节 · 可挽回 0.05）

- **位置**：
  - `U3-export-css-1440` y=230「Scoped dark selector: `data-theme=dark`; standard selector: `.dark`」；
  - 同帧代码 y=469「data-theme="dark" selects dark」，y=510「`[data-theme="dark"]`」。
- **现状 → 期望**：提示里写的是属性赋值，而同一行的 `.dark` 是真实选择器 → 两个都写成可直接复制的选择器。
- **修法**：提示改为 `[data-theme="dark"]`，与代码一致。

#### 新-5 · [P3] 「Changed」徽标比 Change 列首文字左缘右移 3px（U3 · 布局、细节 · 可挽回 0.05）

- **位置**：`U3-import-review-bottom-1440`：表头「Change」文字左缘 x=573，徽标边框 x=576，徽标内文字 x=584。
- **对照**：其他列表头与单元格左缘一致：Field 242/242，Current 729/729，Imported 972/973。
- **现状 → 期望**：徽标盒偏右 3px → 与本列表头文字左缘对齐（x=573）。
- **修法**：去掉徽标的 `margin-left`，或者让徽标单元格的 padding-left 与其他 `td` 相同；徽标本身的内边距不变。

**可挽回分合计**：0.2 + 0.1×4 + 0.08 + 0.06 + 0.05×2 = **0.84**。10 − 9.05 = 0.95，差 0.11，在 0.5 以内。

---

## 4. 建议（不扣分）

以下各条在 X1–X6、G1–G4 中找不到直接依据：

1. **完整 JSON 导出可读性**：
   - 现状：v2/v1 JSON 是单行压缩，靠软换行折断。`U3-export-json-1440` 出现「"chart-」换行后接「1":"#606060"」，键名被拆开；390（`U3-export-json-390`）还出现「"background":"」与「#f0f0f0"」分处两行。
   - 建议：按 2 空格缩进输出，代码框用 `white-space:pre` 加横向滚动（R2 a、b 同提）。
2. **对话框头部固定**：`U3-import-review-end/bottom-1440` 滚到底后，标题「Import theme」和关闭按钮 × 都已滚出视野。建议标题行 `position:sticky; top:0`，带 #eaecf0 底线，只滚动正文。
3. **Apply / Cancel 吸底**：1440 下要滚约 626px 才能看到 Apply（R2 b 提）。可以把动作行 `position:sticky; bottom:0`，带顶线，与第 2 条一起做。
4. **Cancel 改为描边次级按钮**：现在是蓝色文字链样式，与 Apply 的层级差过大（R2 a、b）。
5. **Rust 页底部按钮写明对象**：「Download」改为「Download theme.rs」，与上方「Download matching JSON」对应（R2 a）。
6. **「0.2.1 SDK/CLI reject v2」单独成行警示**：现在混在 12px caption 长句中（R2 a）。
7. **补普通 Cargo 构建命令**：现在只给 `mbx build …`，仓外消费者不一定有 mbx（R2 b）。
8. **差异表颜色值旁加色块**：例如 `#1570ef → #7c3aed`（R2 a、b）。
9. **损失框与备注框视觉区分**：两者现在同为 #f9fafb 圆角框（R2 b）。
10. **390 卡片**：「Changed」徽标单独占一行，可以放到 token 名右侧（`display:flex; justify-content:space-between`），每张卡少一行。
11. **节标题与字段 label 区分层级**：「Import candidate」「Changes (6)」「Import notes」与「Version」「Name」同为 13px/600，层级扁平。可以给节标题加大上间距，或用 14px/600。

---

## 5. 看不出的格子

- **键盘焦点（G4）**：
  - U3 没有键盘焦点帧。`export-kind`、`data-tools-dialog-close`、`export-copy`、`import-review` 虽然登记为焦点所在，但 `U3-export-v1/css/rust-1440` 的 Output 选择器、`U3-export-json-1440` 的 ×、`U3-export-copied-1440` 的 Copy code、`U3-import-review-1440` 的 Review changes 都没有可见焦点环；
  - 日志没有写明这些焦点是鼠标还是键盘到达的，所以无法判断 `:focus-visible` 是否合规。
- **X5**：下载实际发起后的反馈、复制失败、下载失败都没有拍到；只看到复制成功（`U3-export-copied-1440` 的「Copied to clipboard.」#067647）。
- **X1 的其余状态**：
  - 语法错误、超过 256 KiB、不支持的颜色输入都没有拍到，只拍了未知版本；
  - 取消、错误之后「保留当前主题与历史」无法从截图验证；
  - Apply 之后的结果没有拍到；
  - 差异表的「新增 / 删除」类型，以及配置变化时的 Changed 标记与计数，本轮数据没有触发。
- **390 Identity 组卡片**：`U3-import-review-390` 截止在「Brand color」，`U3-import-review-end-390` 从「base.primary」开始，中间的配置下半段和 Identity (2) 的 `id`、`name` 卡片没有任何 390 帧拍到。
- **X3 产物**：Tailwind v4 profile 的导出正文，以及 HEX、HSL、OKLCH 记法的导出正文都没有拍到，只拍了列表展开态与 RGB 正文。
- **X4（Rust 页）**：代码注释说「Copy the listed same-origin font files」，但被列出的字体文件在 `U3-export-rust-1440`（顶部）和 `U3-export-rust-end-1440`（末尾）都看不到，中段未拍。Rust 页是否列出外部字体资源无法确认；CSS 页有列出。
- **G6 中文**：U3 矩阵不含中文态，新增导入/导出文案的中文版本无法核对。

---

## 6. 总评

- **只评 U3，得分 9.05**（R2 为 8.35）。R2 的 19 条差距中 16 条已解决，2 条部分解决（UI-130、UI-133），没有回归到旧问题。
- **主要扣分**：
  - 导入审阅的配置对照没有「当前 / 导入」图例，Version 行的「→」又表示另一种含义（新-1，P2）；
  - 损失清单与备注正文降到 13px（新-4）；
  - 导出对话框底部多出 8px（UI-130）；
  - 字节计数格式、Rust 说明文案、命名方式、选择器写法、徽标偏移、「ui-」断行等细节（新-2、新-3、新-5、新-6、新-7、UI-133）。
- **结构性改变**：不需要推倒重做。唯一需要小范围调整结构的是配置对照：改成与 Changes 表同一套「Field | Current | Imported」列结构，并加 Changed / Unchanged 标识（新-1）。其余都是 token、间距与文案级的修正，修完后预计能回到 9.6 左右。
