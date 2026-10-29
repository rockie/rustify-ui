# R3 评审结论 · U3 摘录

摘自 R3 合并结论 judge.md，只含 U3 的评分、理由、未关闭差距与建议；涉及多个单元的差距只要标题含 U3 就收录。「来源」中的 a、b 指 R3 该单元的评审原文（本次不提供），每条要点以本摘录为准。

## 评分

| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用 | 另一评审 | R2 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | 2.85 | 2.85 | 2.85 | 0.5 | **9.05** | a | b 9.3 | 8.35 |

### 各维度理由

**U3（a）**

- 布局 2.85：差异表只剩对话框本体一层滚动、一帧看全；对话框顶部锚定，切换 Output 时选择器不动；390 分组行铺满；修订号与表单列对齐。扣分原因：导出对话框按钮行到下边框 32px，导入为 24px；Changed 徽标比列首文字右移 3px。
- 视觉 2.85：label、表头、节标题都是 13px/600；备注框内多余横线已去掉；Changed 改为带边框徽标；标识符、调用表达式、选择器都换成等宽；各 token 与 G3 一致。扣分原因：v1 损失清单和 Import notes 正文由 14px 降到 13px，不在 G3 字级里；备注里的字体族值是比例字，390 下从连字符处断行。
- 交互 2.85：合法 v1 审阅、未知版本错误、v2/v1/CSS/Rust 导出、复制成功都走通。扣分原因：配置对照没有注明哪边是当前、哪边是导入；Version 行的「→」与配置行含义不同；Name 行只列导入值。
- 细节 0.5：字节计数无千分位且与「256 KiB」单位不一；Rust 说明「theme.json · theme.rs」有歧义、「Keep AppHandle.」丢了「alive」；备注与差异表的 token 命名方式不一；CSS 提示写 `data-theme=dark` 而代码是 `[data-theme="dark"]`；徽标偏移 3px；「ui-」处断行。

## R3 未关闭差距（U3）

#### UI-128 · [P2] Import candidate 的「→」没有「当前 / 导入」图例；Version 行的「→」含义与配置行不同；Name 行只有导入值（U3 · 交互、细节 · 部分解决 · 可挽回 a 0.2、b 0.15）

- 位置：U3-import-review-1440-light y=598–803；U3-import-review-390-light y=648–781；U3-import-review-mid-390-light y 58–338。
- 现状 → 期望：看不出哪边是当前、哪边是导入，未变化项写成「X → X」 → X1 导入先呈现候选差异（版本、名称、配置）。
- 修法：配置与 Name 行改成与 Changes 表同一套「Field | Current | Imported」列结构；相同的行只显示一个值并标「Unchanged」，不同的行加「Changed」徽标并计入 Changes (N)；Version 行改写为「Source: Theme JSON v1 · Saved as v2」；390 卡片同样用两列小标题。
- 来源：U3 a 判 UI-128 已解决（主体）、另列新-1；U3 b 判 UI-128 部分解决，残留即这一条。判为同一问题的依据：同一区域、同一缺图例现象。按跨轮沿用原编号的规则用 UI-128，严重度取 a 的 P2。

#### UI-130 · [P3] 导出对话框按钮行下方留白 32px，导入对话框为 24px（U3 · 布局 · 部分解决 · 可挽回 a 0.1、b 0.04）

- 位置：U3-export-json-1440、U3-export-rust-1440、U3-export-json-390 留白 32px；U3-import-review-bottom-1440/390、U3-import-error-1440 为 24px。
- 修法：空的复制/下载状态行不占间距，或把状态 live region 移进按钮行作为行尾项。
- 来源：U3 a、b UI-130（均部分解决）。

#### UI-175 · [P3] v1 损失清单与 Import notes 正文降为 13px/400，不在 G3 字级里（U3 · 视觉 · 可挽回 a 0.1、b 0.07）

- 位置：U3-export-v1-1440 y=148–297 行距 19.5px；U3-import-review-bottom-1440 y=771/791 行距 20px；R2 同区域为 14px/21px。
- 修法：损失框与备注框正文用 body 14px/1.5；如果有意做成辅助说明，改用 caption 12px #475467。
- 来源：U3 a 新-4（标为回归）、U3 b 新-3。

#### UI-173 · [P3] 字节计数「3332 / 262144 bytes」没有千分位，与说明里的「256 KiB」单位不一（U3 · 细节 · 可挽回 a 0.1、b 0.08）

- 位置：U3-import-review-1440 y=479、U3-import-error-1440 y=479、U3-import-review-390 y=530；同对话框 y=88 写「up to 256 KiB」。
- 修法：用 `Intl.NumberFormat` 加千分位，上限与说明用同一单位；中文同步。
- 来源：U3 a 新-2、U3 b 新-1。

#### UI-174 · [P3] Rust 接入说明有歧义：「theme.json · theme.rs」读成并列；「Keep AppHandle.」丢了「alive」（U3 · 细节 · 可挽回 a 0.1、b 0.1 中的这一部分）

- 位置：U3-export-rust-1440、U3-export-copied-1440 y=167。
- 修法：改为「Put `theme.json` next to `theme.rs`.」「Keep the returned `AppHandle` alive while the UI is mounted.」；中文同步。
- 来源：U3 a 新-3、U3 b 新-2 的 Rust 部分。

#### UI-177 · [P3] 备注中的字体族值是比例字，390 下「ui-serif」从连字符处断开（U3 · 视觉、细节 · 可挽回 a 0.08、b 0.04）

- 位置：U3-import-review-end-390 y≈699/719、U3-import-review-bottom-390 y≈675/695；1440 y=771/791。
- 修法：requested/resolved 两个值用与路径相同的 inline code 样式并 `white-space:nowrap`。
- 来源：U3 a 新-6、U3 b 新-5；U3 b 在 UI-133 条下也记了作者值为比例字。

#### UI-176 · [P3] 「Changed」徽标与列、基线没对齐，表头下方是双线（U3 · 布局、细节 · 可挽回 a 0.05、b 0.08）

- 位置：U3-import-review-bottom-1440 / -end-1440 表头「Change」x=573，徽标边框 x=576、文字 x=584；徽标基线比同行文字低约 2px；y=301 表头底边与 y=302 分组行上边形成 2px 双线；U3-import-review-mid-390 徽标 x=53，同卡 token 与「Current」x=50。
- 修法：去掉徽标左缩进或让该单元格 padding 与其他 `td` 相同；徽标 `inline-flex` 居中、单元格 `vertical-align:middle`；`thead + tbody > tr:first-child{border-top:0}`。
- 来源：U3 a 新-5、U3 b 新-4（同一徽标；b 补了基线、双线与 390）。

#### UI-133 · [P3] 备注与差异表对 token 的命名方式不一致（U3 · 细节 · 部分解决 · 可挽回 a 0.06、b 0.08）

- 位置：U3-import-review-bottom-1440 y=771「base.light.font-serif」；同帧分组头「Light · base (1)」加行名「base.primary」。
- 修法：两处统一用同一套路径格式：行名只写 token、备注写「Light · base · font-serif」；或都写完整路径。
- 来源：U3 a、b UI-133（均部分解决）。

#### UI-25 · [P3] Import candidate 两组行距不一致（U3 · 布局、细节 · 部分解决 · 可挽回 b 0.06）

- 位置：U3-import-review-1440-light label 大写字母顶边依次为 594、621、654、683、712、741、770、799：Version → Name 27px，配置各行 29px，Name → Appearance 33px。
- 修法：Version、Name 与六个配置字段放进同一个 `dl` grid，只用一个 `row-gap`，去掉两块之间的额外 margin。
- 来源：U3 b UI-25（部分解决）；U3 a 判已解决。

#### UI-178 · [P3] CSS 提示中的「Scoped dark selector: data-theme=dark」不是可用的选择器（U3 · 细节 · 可挽回 a 0.05、b 0.1 中的这一部分）

- 位置：U3-export-css-1440 y=230；同帧代码 y=510 为 `[data-theme="dark"]`。
- 修法：提示改为 `[data-theme="dark"]`。
- 来源：U3 a 新-7、U3 b 新-2 的 CSS 部分。

### U4

## 建议（不扣分，U3）

  - JSON 导出 2 空格缩进，代码框 `white-space:pre` 加横向滚动（沿用 R2）。
  - 对话框标题行吸顶；Apply / Cancel 吸底；Cancel 改描边次级按钮。
  - Rust 页「Download」改名「Download theme.rs」；0.2.1 拒收单独成行警示；补普通 Cargo 构建命令。
  - 差异表颜色值旁加色块；损失框与备注框视觉区分；390 卡片「Changed」徽标放到 token 名右侧；节标题与字段 label 区分层级。
  - 「To match this preview」后用文字标出预览模式（b）。
  - 对话框本体加底部渐隐，提示下面还有内容（b）。
