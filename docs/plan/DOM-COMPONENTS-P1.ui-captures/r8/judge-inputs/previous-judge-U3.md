U3 本轮已取证状态评分 **10.00 / 10**。已逐张实际查看 R7 全部 38 张原图，并对照 R6 独立结论及 8 张原图；未发现有规范依据、能够从本批证据确认的剩余差距。未执行行为另列，不据此判通过。

## 1. 评分表

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | **3.00**：桌面四列差异表与 390 下分组堆叠均清楚，长内容通过内部滚动到达，补图可见完整动作区；未发现结构、遮挡或横向溢出差距，已达上限。 | **3.00**：中性面板、边框、文字层级、等宽代码、错误与焦点处理在已拍状态中一致，未见可凭原图确认的规范偏差，已达上限。 | **3.00**：版本和候选差异、诊断、v1 损失清单、CSS 选项、Rust 接入提示及复制反馈均有可见证据，已拍键盘帧焦点清楚；已达本批可观察状态的上限。 | **1.00**：逐处查看已拍内容，长名称、配置值、版本文字、列表换行和动作标签未见剩余细节差异，已达上限。 | **10.00 / 10** |

失分合计 **0.00**；未关闭差距可挽回分合计 **0.00**。

## 2. 上一轮差距复核

R6 独立结论没有未关闭差距，因此没有需要沿用编号的旧差距。

| R6 结论中的检查点 | R7 复核 | 状态 |
| --- | --- | --- |
| 桌面差异表及表头上方残字问题已消除 | 对照两轮 `U3-import-review-bottom-1440-light.png`，本轮未见表头上方残留文字 | 已解决，未见回归 |
| 390 差异排列及内部滚动符合规范 | 本轮首段、Identity 中段、Appearance 至 Scaling、两模式差异及底部动作均有原图 | 已解决，未见回归 |
| 新增选择器具有可见键盘焦点 | Output、CSS profile、Color notation、Input format 及下一代码控件均有明确蓝色焦点；格式菜单键盘展开后活动项可辨 | 已解决，未见回归 |
| v1、CSS、Rust 与复制反馈呈现完整 | 桌面与 R6 对应原图未见不利变化；本轮新增 390 原图覆盖这些状态 | 已解决，未见回归 |

## 3. 未关闭差距清单

**无。** 没有可据本批原图成立的 P0–P3 差距，也没有新增扣分编号。

具体可见依据：

- **X1**：候选审阅显示当前 v2 与导入 v1／保存为 v2 的关系、id、名称及配置；桌面和窄屏都能读到 Light/Dark 的 base 与 overrides 分组及导入说明。Apply import、Cancel 在补拍的底部帧中完整可见。
- **X2**：默认输出明确标为 Theme JSON v2；切换兼容 v1 后，复制／下载之前列出品牌色、中性色 Auto、覆盖来源、派生角色、appearance、radius 和 scaling 的具体损失。390 的补图可见 v1 代码及 Copy code／Select text／Download。
- **X3**：两种 CSS profile 和 HEX／RGB／HSL／OKLCH 四种记法在两视口的展开列表中可见；说明明确区分已解析 CSS 与保留作者配置的 JSON。
- **X4**：Rust 说明显示所需 source revision、匹配 SDK/CSS、构建 CLI 的命令、旧 0.2.1 SDK/CLI 拒绝 v2、字体保持外部资源，以及当前预览的 `ThemeMode::Light`。CSS 原图可读到外部字体资源和许可证提示。
- **X5 的已拍成功态**：桌面复制后出现 `Copied to clipboard.`；390 补拍同样把反馈、代码及三个动作放进画面。
- **X6、G4**：两视口面板均在视口内；390 表单单列、代码换行；长内容内部滚动可达动作区。已拍选择器和代码区具有独立、明显的键盘焦点。

## 4. 建议（不扣分）

无新增产品修改建议。后续如需验证完整行为契约，可补充下列行为取证；它们不构成本轮视觉差距。

## 5. 看不出的格子与未执行行为

**必需矩阵格子未见：无。** U3 规定的七类状态在 1440×900 与 390×844 的 light 条件下均有证据。

局部帧的观察限制：

| 原图 | 单张图看不出的内容 | 本批是否已有补充 |
| --- | --- | --- |
| `U3-export-copied-390-light.png` | 看不到复制成功提示，不能仅凭文件名判成功 | **有**：`U3-export-copied-feedback-390-light.png` 清楚显示提示 |
| `U3-export-rust-390-light.png`、`U3-export-rust-mid-390-light.png`、`U3-export-rust-end-390-light.png` | 底部动作在这些滚动位置未完整入画 | **有**：复制及反馈补图完整显示三个动作 |
| 两视口 `U3-import-review-end-…` | 滚动边缘不能单张覆盖完整配置与差异正文 | **有**：bottom、mid 及两张配置补图提供互补内容 |
| Rust 代码首／中／末帧 | 非连续截图不能逐行证明整份生成代码或所有资源地址的正确性 | 本轮只确认可见接入说明与排版，不推断未入画正文 |

**矩阵外或未执行行为，保持未核实：**

- 日志未执行 Apply import，不能确认应用后的替换结果及历史。
- 日志中的 Escape 不能证明 Cancel 的历史保留契约；也未覆盖语法错误、超限和不支持颜色。
- 未点击 Download 或 Download matching JSON，不能判断下载是否实际发起及成功时序。
- 复制为授予权限后的成功路径；未覆盖拒绝权限、复制失败、下载失败及失败恢复。
- CSS 列表证明两种 profile、四种记法可见；未逐一选择所有组合验证生成结果。
- 已拍键盘焦点和菜单移动；未完整取证所有确认、Escape 返回焦点、焦点循环与全部操作的键盘闭环。

这些内容均未被推断为通过或失败。

## 6. 总评

本次只评 U3，没有其他单元可作最低分比较。U3 的已拍状态与冻结规范一致，未见需要结构性调整之处。R7 的窄屏补图消除了原始单帧对复制提示、v1 动作区及导入配置中段的观察空白。

实际开图工具为 **`functions.exec` 调用 `tools.view_image`，使用 `detail: "original"`**。以下均实际打开；没有以列文件或读取尺寸代替开图。

R7：38 张。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-json-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-v1-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-profile-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-notation-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-copied-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-mid-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-json-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-bottom-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-bottom-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-mid-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-config-appearance-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-review-config-scaling-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-kind-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-kind-keyboard-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-profile-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-color-format-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-next-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-kind-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-kind-keyboard-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-v1-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-profile-open-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-css-notation-open-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-mid-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-rust-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-copied-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-import-error-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-copied-feedback-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/U3-export-v1-actions-390-light.png
```

R6：8 张。

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-kind-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-v1-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-css-notation-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-rust-mid-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-copied-1440-light.png
```
