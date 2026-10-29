本次仅评 U3，依据目标包、R5 的 18 张原始截图与操作记录，以及白名单内的 R4 同名原图和结论摘录。未读取代码、计划、修复记录或机器检查，未写入仓库。

**1. 评分表**

| 单元 | 布局（0–3） | 视觉（0–3） | 交互与状态（0–3） | 细节（0–1） | 总分 |
| --- | --- | --- | --- | --- | --- |
| U3 Studio 导入与导出 | **2.95**：桌面四列表、390 单列审阅及对话框内部滚动符合目标；滚到底时粘性表头上方仍漏出被遮行的残字，因此未给满分。 | **3.00**：已拍状态的中性壳层、表头、状态区分、代码排版与诊断颜色符合规范，未发现可扣分的视觉偏差，已达本维度上限。 | **3.00**：已拍状态呈现了明确的候选版本归属、两模式差异、未知版本错误、v1 损失、CSS 选项、Rust 接入说明和复制反馈；这些可见状态中未发现失分项，未拍结果另列为不可判断。 | **0.95**：上轮的基线、缩进、行高与备注字体问题均已消除；桌面表头上方的残字仍影响局部完成度。 | **9.90 / 10** |

失分合计 **0.10**；下列未关闭差距的可挽回分合计 **0.10**。

**2. 上一轮差距复核表**

| 编号 | 本轮状态 | 复核依据 |
| --- | --- | --- |
| UI-128 | **已解决** | `U3-import-review-1440-light.png` 的 Source 行已将 `Theme JSON v2` 放入 Current，将 `Theme JSON v1 · saved as v2` 放入 Imported；390 的 `import-review`、`import-review-mid` 也明确标有 Current / Imported。 |
| UI-198 | **已解决** | 原候选配置与 Changes 已合为一张 `Field / Change / Current / Imported` 表；Identity & configuration、Light/Dark 的 base/overrides 共用列位置、表头和分组结构。390 使用同一套卡片字段顺序。 |
| UI-176 | **已解决** | 桌面的 Changed 徽标与同行字段、值已对齐；390 的 Source、id、Name 及 token 徽标统一从内容左缘开始，未再见旧的额外 8px 缩进；Name 两侧值基线一致。 |
| UI-199 | **已解决** | Changed 保留边框徽标，Unchanged 改为普通文字；桌面状态集中在独立 Change 列并左对齐，不再随 Current 值长度错开。 |
| UI-200 | **已解决** | `import-review-bottom` 的桌面与390备注中，`Light · base` / `Dark · base` 使用正文字体，只有 `font-serif`、`ui-serif`、`Noto Serif` 等技术内容使用等宽字。 |
| UI-25 | **已解决** | Source 与其他身份/配置行已进入同一表格行结构，未再见旧的 Source→Name 特殊紧缩；移动端卡片间距也统一。 |

**3. 差距清单**

**新-1 · U3 · 桌面审阅滚到底后，粘性表头上方漏出被遮行的残字**

- **截图与位置：** `U3-import-review-bottom-1440-light.png`，对话框内容区顶部，约 x=233–1207、y=17–74；`U3-import-review-end-1440-light.png` 同一区域也显示分组内容位于粘性表头上方。
- **维度：** 布局、细节。
- **严重度：** **P3 打磨**。
- **现状 → 期望：** 表头停在距对话框上缘约24px的位置，上方这条区域仍绘制滚动中的表体；在 bottom 图中可以看到 Source、Changed 徽标和两侧版本值的字形碎片，随后才是完整的 Field / Change / Current / Imported 表头。期望滚动时表头上沿形成完整、干净的遮挡面，不出现被表头截断后仍露出的残行。依据为 X6 的内部纵向滚动要求，以及布局标尺对遮挡、截断的检查。
- **具体修法：** 调整候选差异表的粘性表头与其实际滚动容器的关系：让表头锚定滚动视口上缘，使用不透明的 popover 背景和正确的层级；若保留表头上方24px留白，给整条留白增加同色遮挡面，避免表体从该区域透出。保留初始状态的1.5rem内容内边距。
- **可挽回分：** **0.10**，其中布局0.05、细节0.05。
- **轮次判断：** 新发现；R4 原图未呈现同一粘性表头贴顶状态，不能据此断言属于回归。

**4. 建议（不扣分）**

无额外建议。本轮不要求新增布局或组件。

**5. 看不出的格子与实际阅图记录**

以下内容未由本轮截图与记录证明，既不算通过，也不计失败：

- 导入记录停在 Review、滚动审阅和 Escape；没有 Apply 后的结果帧，无法确认实际替换及历史变化。Escape 前后也没有完整历史对照。
- `U3-import-error-1440-light.png` 只覆盖未知版本999；语法错误、超限、不支持的颜色输入及其主题/历史保留结果未拍摄。
- `U3-export-copied-1440-light.png` 能确认点击 Copy 后出现成功反馈；未覆盖复制失败、下载实际发起后的反馈及下载失败路径。
- CSS 两张展开图能确认两种 profile 与四种颜色记法的选项；未展示切换到其他组合后的实际输出。
- Rust 三张代码帧能读到 revision、旧0.2.1不支持v2、外部字体提示及当前 Light 预览调用；具体字体资源清单没有在 Rust 代码帧中完整展示。CSS帧能看到部分字体文件路径，不能据此补判 Rust 清单完整。
- 没有对话框内的键盘逐项走查、Tab 焦点帧或中英文切换帧，无法判断这些未拍状态。

实际逐张打开的原始图片如下；**每个文件名均打开过 R5 和白名单中的 R4 版本，共36张原图**：

1. `U3-import-review-1440-light.png`
2. `U3-import-review-end-1440-light.png`
3. `U3-import-error-1440-light.png`
4. `U3-export-json-1440-light.png`
5. `U3-export-v1-1440-light.png`
6. `U3-export-css-1440-light.png`
7. `U3-export-css-profile-open-1440-light.png`
8. `U3-export-css-notation-open-1440-light.png`
9. `U3-export-rust-1440-light.png`
10. `U3-export-copied-1440-light.png`
11. `U3-export-rust-end-1440-light.png`
12. `U3-import-review-390-light.png`
13. `U3-import-review-end-390-light.png`
14. `U3-export-json-390-light.png`
15. `U3-import-review-bottom-1440-light.png`
16. `U3-import-review-bottom-390-light.png`
17. `U3-import-review-mid-390-light.png`
18. `U3-export-rust-mid-1440-light.png`

另打开了 `/tmp/u3-r5-import-top-inspection.png`，它是从 R5 原始 bottom 图生成的局部二倍检查裁切，仅用于确认表头上方的残字，不作为产品截图替代品。对原PNG做了只读像素核对：错误边框与文字为 `#b42318`，CSS选中项背景与文字为 `#eff8ff` / `#175cd3`，符合G3，不扣分。

**6. 总评**

本次只评 U3，得分 **9.90**。上轮六条未关闭差距均已解决；桌面统一差异表和移动端对应结构已成立，无需推倒重做。剩余问题仅为桌面审阅滚动到底时，粘性表头上方漏出残行，需要局部修正滚动遮挡关系。
