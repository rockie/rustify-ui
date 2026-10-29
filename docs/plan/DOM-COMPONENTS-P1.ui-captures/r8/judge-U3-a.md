# U3 · R8 独立 UI/UX 评审

本次只评 U3。已逐张实际查看本轮全部 **38 张原图**，并查看 **8 张 R7 原图**作对照。按冻结目标的规范模式，本轮可观察状态评分为 **10.00 / 10**，未发现能够由本批证据确认的未关闭差距。未展示行为保留为未核实。

## 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | **3.00**：桌面四列差异表、390 下的分组与双值排列清楚；长内容内部滚动，补图完整显示底部动作；未见应扣分的溢出、遮挡或结构偏差，已达上限。 | **3.00**：面板、遮罩、文字层级、等宽代码、选中与焦点表达符合冻结规范；错误色核对为 `#b42318`，未发现可确认的视觉偏差，已达上限。 | **3.00**：候选版本与差异、未知版本错误、v1 损失、CSS 选项、Rust 接入提示和复制反馈均有可见证据；已拍键盘焦点明确，已达所观察状态的上限。 | **1.00**：逐处检查已拍文字、长名称、版本、配置值、换行与动作标签，未发现可据规范成立的细节差异，已达上限。 | **10.00 / 10** |

失分合计 **0.00**；未关闭差距的可挽回分合计 **0.00**。

## 2. 上一轮差距复核

R7 独立结论没有未关闭差距，因此没有需要沿用编号的旧差距。其已关闭检查点复核如下。

| R7 检查点 | R8 可见依据 | 状态 |
| --- | --- | --- |
| 桌面差异表及表头上方残字问题已消除 | 对照两轮 `U3-import-review-bottom-1440-light.png`，未见表头上方残字 | 已解决，未见回归 |
| 390 差异排列与内部滚动 | 本轮 Identity、Appearance、Scaling、两模式差异、诊断与底部动作均有互补帧；对照 R7 底部图，排列未见不利变化 | 已解决，未见回归 |
| 新增选择器有独立可见键盘焦点 | Output、CSS profile、Color notation、Input format 和下一代码控件均有清楚的蓝色焦点；展开菜单活动项可辨 | 已解决，未见回归 |
| v1、CSS、Rust 与复制反馈呈现完整 | 对照 R7 对应原图，损失列表、颜色记法、接入说明与窄屏复制反馈未见不利变化 | 已解决，未见回归 |

## 3. 未关闭差距清单

**无。** 未发现可由本批原图确认的 P0–P3 差距，无新增扣分编号。

可见依据如下：

- **X1：导入候选审阅。** 显示当前 Theme JSON v2 与导入 v1、保存为 v2 的关系；Identity 与配置逐项列出当前值和导入值，Light/Dark 的 base、overrides 分组及诊断可读。底部补图完整显示 Apply import 和 Cancel。
- **X2：兼容 v1。** 默认输出标为 Theme JSON v2；显式切换 v1 后，复制与下载之前展示品牌色、中性色 Auto、覆盖来源、派生角色、appearance、radius 与 scaling 的具体损失。390 补图显示代码及三个动作。
- **X3：CSS。** 两视口均拍到 Rustify scoped、Tailwind v4 两种 profile，以及 HEX、RGB、HSL、OKLCH 四种记法；说明区分已解析 CSS 与保留作者配置的 JSON，并提示外部字体与许可证。
- **X4：Rust。** 说明显示 required source revision、匹配 SDK/CSS、CLI 构建命令、旧 0.2.1 SDK/CLI 拒绝 v2、外部字体，以及匹配当前预览的 `ThemeMode::Light`。
- **X5：已拍复制反馈。** 桌面和窄屏补图均能读到 `Copied to clipboard.`，同时保留代码与 Copy code、Select text、Download。
- **X6、G4：布局与焦点。** 对话框保持在可用视口内，390 表单单列；代码换行，长内容通过内部纵向滚动查看。已拍键盘帧具有独立、明确的可见焦点。

## 4. 建议（不扣分）

无新增产品修改建议。后续若需要验证完整行为契约，可补充下一节列出的未执行行为；这些内容不构成本轮视觉差距。

## 5. 看不出的格子与观察边界

**冻结矩阵内没有整类必需状态缺证。** 桌面规定的七类状态，以及窄屏导出与导入审阅，均有原图。

以下属于单帧观察限制或正常滚动裁切：

| 原图 | 单张图看不出的内容 | 本批补充情况 |
| --- | --- | --- |
| `U3-export-copied-390-light.png` | 复制成功提示未入画，文件名不能证明反馈可见 | `U3-export-copied-feedback-390-light-a2.png` 明确显示提示 |
| `U3-export-rust-390-light.png`、`U3-export-rust-mid-390-light.png`、`U3-export-rust-end-390-light.png` | 底部动作未完整入画 | 复制反馈补图完整显示三个动作 |
| 两视口 `U3-import-review-end-…` | 滚动边缘不能在一张图内覆盖全部配置和差异 | bottom、mid、Appearance 与 Scaling 补图提供互补内容 |
| `U3-export-v1-390-light.png` | 损失列表之后的代码与动作在折线以下 | `U3-export-v1-actions-390-light-a2.png` 显示代码及完整动作区 |
| Rust 代码首、中、末帧 | 非连续截图不能逐行证明完整生成代码及全部资源地址正确 | 只评价已显示说明和排版，不推断未入画正文 |

**矩阵外扩展行为或未执行行为，保持未核实：**

- 日志未执行 Apply import，不能确认替换结果与历史。
- Escape 不能证明 Cancel 的历史保留契约；语法错误、超限和不支持颜色也未执行。
- 未点击 Download 或 Download matching JSON，不能判断实际下载及成功时序。
- 复制记录与截图展示授予权限后的反馈，不能确认剪贴板内容、成功时序或拒绝权限后的失败恢复。
- CSS 列表证明两种 profile、四种记法可见，未逐一选择所有组合核查生成结果。
- 已拍键盘焦点与菜单移动，未完整展示所有确认、Escape 返回焦点、焦点循环和全操作键盘闭环。

上述内容均未被推断为成功或失败。

## 6. 总评

本次只评 U3，没有跨单元的最低分比较。U3 已拍表现与冻结规范一致，未发现需要结构性调整的区域。窄屏补图覆盖了复制提示、v1 动作区和导入配置中段的单帧观察空白。

## 实际开图工具与清单

开图工具：**`functions.exec` 调用 `tools.view_image`，全部使用 `detail: "original"`**。另对已打开的错误态 PNG 做只读像素抽样，核对错误边框及文字实体像素为 `#b42318`；没有以尺寸或像素统计代替实际开图。

本轮绝对目录前缀：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/
```

该目录下实际打开的 38 张：

```text
U3-import-review-1440-light.png
U3-import-review-end-1440-light.png
U3-import-error-1440-light.png
U3-export-json-1440-light.png
U3-export-v1-1440-light.png
U3-export-css-1440-light.png
U3-export-css-profile-open-1440-light.png
U3-export-css-notation-open-1440-light.png
U3-export-rust-1440-light.png
U3-export-copied-1440-light.png
U3-export-rust-end-1440-light.png
U3-import-review-390-light.png
U3-import-review-end-390-light.png
U3-export-json-390-light.png
U3-import-review-bottom-1440-light.png
U3-import-review-bottom-390-light.png
U3-import-review-mid-390-light.png
U3-export-rust-mid-1440-light.png
U3-export-kind-keyboard-focus-1440-light.png
U3-export-kind-keyboard-open-1440-light.png
U3-export-profile-keyboard-focus-1440-light.png
U3-export-color-format-keyboard-focus-1440-light.png
U3-export-next-keyboard-focus-1440-light.png
U3-import-kind-keyboard-focus-1440-light.png
U3-import-kind-keyboard-open-1440-light.png
U3-export-v1-390-light.png
U3-export-css-390-light.png
U3-export-css-profile-open-390-light.png
U3-export-css-notation-open-390-light.png
U3-export-rust-390-light.png
U3-export-rust-mid-390-light.png
U3-export-rust-end-390-light.png
U3-export-copied-390-light.png
U3-import-error-390-light.png
U3-export-copied-feedback-390-light-a2.png
U3-import-review-config-appearance-390-light-a2.png
U3-import-review-config-scaling-390-light-a2.png
U3-export-v1-actions-390-light-a2.png
```

上一轮绝对目录前缀：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/
```

该目录下实际打开的 8 张：

```text
U3-import-review-bottom-1440-light.png
U3-import-review-bottom-390-light.png
U3-import-error-1440-light.png
U3-export-kind-keyboard-focus-1440-light.png
U3-export-v1-1440-light.png
U3-export-css-notation-open-1440-light.png
U3-export-rust-mid-1440-light.png
U3-export-copied-feedback-390-light.png
```
