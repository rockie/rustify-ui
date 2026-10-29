本次只评 U3。已读取冻结目标包的完整变更记录、U3 采集清单、操作记录与 R5 独立评审，并实际打开全部 26 张 R6 原图及用于复核剩余差距的 1 张 R5 原图。未读取代码、完整报告或修复记录，未修改文件。U3 属于规范模式，没有需要比较的目标 PNG。

**1. 评分表**

评分仅针对截图与操作记录能证明的状态；未拍到的结果单独列为不可判断。

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | **3.00**：桌面差异表、390 窄屏排列及内部滚动符合规范；R5 的表头上方残字已消除，已达本维度上限。 | **3.00**：已拍状态的中性面板、代码排版、错误色与选中色符合规范，未发现可扣分偏差，已达本维度上限。 | **3.00**：候选版本与差异、错误、损失清单、CSS 选项、Rust 接入提示和复制反馈均清楚；新增键盘帧显示可见焦点及选择结果，已达本维度上限。 | **1.00**：逐处检查已拍状态，未发现有规范依据的剩余细节差异，已达本维度上限。 | **10.00 / 10** |

失分合计 **0.00**；未关闭差距的可挽回分合计 **0.00**。

**2. 上一轮差距复核表**

| 编号 | 本轮状态 | 复核依据 |
| --- | --- | --- |
| UI-128 | **已解决** | 桌面 `import-review` 的 Source 行明确区分 Current 的 `Theme JSON v2` 与 Imported 的 `Theme JSON v1 · saved as v2`；390 初始与中段帧保留相同归属。 |
| UI-198 | **已解决** | 桌面候选信息采用统一的 Field / Change / Current / Imported 表；Identity & configuration 与两模式 base、overrides 共用结构；390 使用对应字段块。 |
| UI-176 | **已解决** | 桌面 Changed 徽标与同行字段、值对齐；390 的 Source、id、Name 与 token 状态从相同内容左缘开始，未见旧的额外缩进或 Name 值基线差异。 |
| UI-199 | **已解决** | Changed 使用边框徽标，Unchanged 为普通文字；桌面状态统一落在独立 Change 列，不随值长度错位。 |
| UI-200 | **已解决** | 桌面及390的 Import notes 中，Light / Dark、base 使用正文字体；`font-serif`、`ui-serif`、`Noto Serif` 等技术内容使用等宽字。 |
| UI-25 | **已解决** | Source 与身份、配置字段采用一致行结构；390 的 Source、id、Name 块采用一致间距，未再见 Source→Name 的特殊紧缩。 |
| 新-1（R5） | **已解决** | 对照 R5 与 R6 的 `import-review-bottom-1440-light.png`：R5 表头上方可见被截断的 Source、Changed 和版本值碎片；R6 表头已贴到对话框滚动视口上缘，上方不再露出残行。R6 `import-review-end-1440-light.png` 同样没有旧问题。 |

**3. 差距清单**

本轮已拍状态中，**没有仍未关闭且能由目标包支持的差距，也没有新增扣分项**。

R5「新-1」所要求的局部滚动遮挡修正已生效，不需要继续调整表格布局。未拍状态不补造问题，列于第5项。

**4. 建议（不扣分）**

无额外设计建议。当前证据没有显示需要更换组件、重排信息或重做素材。

**5. 看不出的格子**

以下内容未获充分证据，既不计通过，也不计失败：

- 导入操作停在 Review、滚动审阅与 Escape，没有点击 Apply 后的结果帧，无法确认实际替换以及历史变化；取消前后的历史保留也没有完整对照。
- `U3-import-error-1440-light.png` 只覆盖未知版本999；语法错误、超限、不支持颜色及这些错误下的主题与历史保留结果未拍摄。
- `U3-export-copied-1440-light.png` 与操作记录能确认点击 Copy 后出现成功反馈；截图没有剪贴板读回结果。复制失败、下载实际发起及下载失败路径未覆盖。
- CSS 展开帧能确认两种 profile 和四种颜色记法；未展示切换至 Tailwind v4 或其他颜色记法后的实际输出。
- Rust 首、中、末帧能读到所需 revision、旧0.2.1 SDK/CLI 不支持v2、外部字体提示及当前 Light 预览接入调用；没有完整呈现 Rust 输出中的具体字体资源清单，不能据此确认清单完整。
- 键盘证据覆盖导入格式、导出格式、CSS profile、颜色记法及下一代码字段；没有对所有按钮、关闭动作、焦点循环、焦点返回和窄屏键盘路径的完整走查。
- U3 没有中文状态帧，无法确认导入导出全部新增文案的中英文对应。

**6. 总评**

本次只评 U3，得分 **10.00 / 10**。R5 唯一未关闭的表头残字问题已经解决；此前已关闭的问题没有在本轮证据中回归。新增键盘帧呈现了明确的蓝色焦点环，导入格式也有键盘确认后的选中结果。

当前已拍界面无需结构性改变。该分数表示本轮可见状态未发现目标差距，不代表第5项未拍结果已经验证通过。

实际开图工具为 **`functions.exec` 内调用 `tools.view_image`，再使用 `image(...)` 展示图片**。全部图片均实际打开；另对已打开的错误与选中状态原图作只读像素复核，确认错误色为 `#b42318`、选中背景与文字为 `#eff8ff` / `#175cd3`。像素复核未代替开图。

实际打开的图片绝对路径，共 **27 张：R6 26张，R5 1张**：

```text
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-color-format-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-copied-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-css-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-css-notation-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-css-profile-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-json-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-json-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-kind-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-kind-keyboard-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-next-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-profile-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-rust-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-rust-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-rust-mid-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-export-v1-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-error-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-format-keyboard-selected-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-kind-keyboard-focus-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-kind-keyboard-open-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-bottom-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-end-1440-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-end-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/U3-import-review-mid-390-light.png
/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/U3-import-review-bottom-1440-light.png
```
