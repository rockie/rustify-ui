# R1 页面实测

本文件由主 agent 在独立评审原文及初版 judge.md 保存后记录。结果未交给评分评审，不调整独立分数。所有检查来自 HTTP 运行页面，不凭源码或样式表推断。

- Chrome 156.0.8078.74；Playwright 1.63.0；axe-core 4.13.0；headless、DPR1。
- Studio：`http://127.0.0.1:4181/`，DOM → Cards → Form components / States and parts。
- 独立 Themes 消费者：`http://127.0.0.1:4190/fields.html`。
- 原始结果：[runtime-checks.json](logs/runtime-checks.json)、[extra-checks.json](logs/extra-checks.json)、[稳定后复核与 Chrome 无障碍树](logs/recheck.json)。日志目录按仓库规则忽略，仅本机可复现。

## 已确认的额外差距

| 编号 | 页面实测 | 期望及处理 |
| --- | --- | --- |
| UI-18 / P1 / U1、U2 | 浅色默认 `Fail next local save` 及 `Set indeterminate`：普通15px文字 `#1570ef` 对 `#f9fafb`；axe 报4.37:1，按实测色计算4.372367774254193:1。见[按钮帧](M-default-contrast-1440-light.png)。 | G-2普通文本4.5:1。调整默认浅色相应按钮文字/底面配对，复测正常及交互态；不要求作者自定义任意配色都被强行替换。 |
| UI-19 / P2 / U1 | 完整表单 CheckboxGroup 的实际元素 `role="group" aria-required="true"`；浅/深/手机深色均触发 axe `aria-allowed-attr`。 | 控件语义及必填信息应可被正确解释。为组提供有效的必填说明/关联并保留组行为，避免 unsupported 属性；不要只为消除扫描改成不符合多选语义的 role。 |
| UI-20 / P1 / U5、U6 消费者 | `select-responsive`、`select-typed` 两个 Trigger 可见值 Apricot，但 axe 两次均报 `button-name`；稳定700ms后 Chrome AX Tree 实测 `role=combobox`、`name=""`、`value="Apricot"`。Playwright快照 `- combobox: Apricot` 的冒号后内容是值，不能当作名称。见[实际帧](M-select-names-1440-light.png)。 | G-2/U6-1的控件标签与名字。为这两个消费样例添加可见标签和实际关联，复核 AX name；不是把值当名称，也不推定所有 Select 均缺名。 |

## 补充测量与未新增差距

- UI-03 的视觉现象得到实测支持：稳定后正常关闭 soft Switch 轨道 `#f2f4f7`、透明边框，预览底面及 thumb `#f9fafb`，轨道/底面对比1.054378786890847:1。外层示例框不能代替控件边界。[实拍](M-soft-switch-1440-light.png)。初次即时样式读数处在过渡中，不能采用；以上采用稳定700ms后的值。原独立 P2 及分数保留，修复时一并核对 G-2 的必要边界3:1。
- 独立消费者 `Submit fruit` 为83.421875×21px；axe WCAG2.2 AA `target-size` 报安全点击直径21px、应24px。目标包未单独冻结 WCAG2.2 触控尺寸门槛，本轮列可用性建议，不追加扣分差距。可增大命中区域或相邻间距。
- Studio 表单桌面 `document.scrollWidth=1440`、表单client/scroll均1022；390px `document.scrollWidth=390`、表单client/scroll均340。所查范围未测得页面/表单横向溢出。少量thumb内部隐藏内容的scrollWidth大于clientWidth，不能据此判页面溢出。
- 真实 Tab/Shift+Tab 后名称输入聚焦且 `:focus-visible=true`，外层3px实线轮廓。单次原生 Meta+Z 有恢复记录。机器脚本的 Home 没有移到 macOS 字首，未形成选区；不能记产品失败。独立补采 a5 用 Meta+ArrowLeft 后得到0–2选区。
- 实测 Slot 所在Root：1440px gap16/padding-left12/right16；390px gap4/left12/right8；颜色均 `rgb(206,44,49)`。只登记现象，不把此自定义utility样例与默认Slot间距混作同一目标；完整优先级矩阵尚未验证。
- axe 对部分颜色/复杂范围留下 incomplete；扫描零 violation 仅代表所查规则和范围，不是完整 AA 或辅助技术通过。

## 控制台与网络

采集日志 [browser.jsonl](logs/browser.jsonl) 共162条：13次session、149条console，其中log134、warning10、error5。无记录到 pageerror、requestfailed 或页面 response≥400。

- 10条warning均为 Makepad嵌入区不支持 `document_title`，已有GPU兼容入口可见，无本轮 DOM 失败证据。
- 5条error均为资源404，但console记录未携带URL，页面response监听未捕获对应请求；本轮不能确认具体资源。保留为待定位环境记录，不断言是favicon，也不把它记为零错误。
- 无请求拦截、替代UI或业务后端依赖。原始console文本和哈希保留，不向独立评分者提供。

## 结果文件 SHA-256

| 文件 | SHA-256 |
| --- | --- |
| runtime-checks.json | 15aad648551f92564a706634134e4af1b0a3be2fac7ba436946794acaa1c43e2 |
| extra-checks.json | 2a2820950624b458ee368cc9ff2c0071049cef16b9ff5ef1df8c81c015c4c9dd |
| recheck.json | 37f07cb3993a26b02d4e91f69430077c5b56d1b3fb920cdfad741accdda99777 |
| browser.jsonl | 5cca3c845757de6828fb5038fe46e886a08ff1adb845213123b682cef819e968 |
