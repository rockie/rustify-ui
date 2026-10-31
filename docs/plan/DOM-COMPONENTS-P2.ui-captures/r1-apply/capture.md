# UI R1 修复自检

- 被测：main，`dirty@64bb17281567a646b312a4b7050219fe5a7f0c55`；实现提交从[计划 M5 的 UI修复条目](../../DOM-COMPONENTS-P2.md#完成记录)续接。产品路径为 components 的 choice/CSS、Studio 表单及独立 DOM 消费者；原有未提交评审/台账/规则未纳入产品差异。
- 人工 Markdown 循环；没有 managed 索引，没有 import/reset。作者自检，不评分，不替代独立复审。目标包保持 `c83c6339be77…`，R1 报告、目标及 judge 未改写。
- Playwright 1.63.0 + Google Chrome 156.0.8078.74，headless，DPR1；en-US、Asia/Shanghai；英文主采、中文补充。1440×1000、768×1000、390×844。页面实际操作、真实字体、截图禁用动画；无拦截、模拟 UI 或图片编辑。
- Studio `http://127.0.0.1:4181/` → DOM → Cards → Form components / States and parts；raw `http://127.0.0.1:4190/`；themed consumer `http://127.0.0.1:4195/fields.html`。均由本仓 strict-CSP xtask serve 启动。本轮先构建，再串行检查。

## 实际步骤与结果

1. Studio 空表单提交、单字符 rule、失败本地保存、修改后重试：来源文字分别为 Input requirement / Form rule / Local save，并关联 Control；成功明确仅此预览。`-` 草稿实拍时接受 FormData 仍为12；自动用例另验 Escape、Enter、原生 requestSubmit 及从草稿直接点击下方 summary，提示清除且操作不丢失。提示保留固有布局空间。
2. 状态工具实际切换 Checkbox、Switch、Cards、ToggleGroup、Text/ColorField、RangeSlider：标签/帮助和强制 invalid 演示文字可见；原生编辑元素及卡片/Toggle 的实际 item 获得说明。切掉已选 Toggle 后接受值为 `[]`。完整表单 Team 的实际 AX name 仍含已有说明。
3. RangeSlider End 到100，LTR与深色RTL端点实拍有完整圆形和焦点轮廓；根 overflow 为 visible。独立用例覆盖 Home/End、跨越与同一焦点身份。
4. raw 拒绝 B：接受值仍 A，最后请求/请求数/接受数逐行具名；竖向请求36与接受35分别标明。保存中、失败、重试完成均有独立状态。Select 空/长列表有不透明底面与边界；End到末项。A→取消B后仍A，输出 `Cancelled: b`。390px 反馈折行，Trigger 文本与 Open 分隔。
5. 两个消费者 Trigger 的 AX name 实读为 Responsive fruit / Custom trigger fruit，值为 Apricot；定向 button-name 检查0违反。Studio 表单定向 color-contrast / aria-allowed-attr / button-name 检查0违反。raw 末次采集控制台 error 与失败响应均0；Studio 自检 pageerror/requestfailed 0。定向数据见[observations.json](observations.json)。
6. 实看下列31张修复后实拍。768表单、390深色中文与长FormData、RTL浮层均可读；preview没有横向溢出。compare 保留实际GPU内容，表单顶部的DOM说明见完整表单截图；此帧未重新评审整个既有场景。

[对比度原始数据](contrast.json)：两处按钮正常12.653、hover/focus12.027、active10.827；soft Switch 外边界浅3.159、深3.680。按实际 alpha 合成与祖先底面计算，等待过渡结束；未对任意作者主题作认证。

## 产物

| 构建 | build_id | manifest SHA-256 |
| --- | --- | --- |
| theme-studio | `f4031762b5858f2a` | `6ccbdf779ef3b6385749d18601c5666fe4a21c974a51eaee00ec668707780f2b` |
| component-catalog | `3df5d32b4cf3c6f8` | `13c18195f68264f5c6a6d7574919856cbe00078db136981fe6bf148d369d43d8` |
| property-workbench | `b2026f416f646082` | `d1851b0dfa74674d5d2c8019378ce046638eceaa53caa853aab0b05a4f33f7db` |

对应命令为 `mbx xtask build-web --example <name> --release`；raw/themed consumer 另以 `mbx xtask build-dom --manifest-path tests/dom-only/{primitives,consumer}/Cargo.toml` 构建。没有部署。

## 图像

17张 `before-*` 保留修复前实际状态；下列31张为最终自检。PNG按既有规则忽略，仅本机可直接打开；外机需从修复提交重建重拍。原R1目录没有覆盖。

| 实拍 | 视口 |
| --- | --- |
| [U1-application-1440-light.png](U1-application-1440-light.png) | 1440×1000 |
| [U1-default-768-light.png](U1-default-768-light.png) | 768×1000 |
| [U1-rule-1440-light.png](U1-rule-1440-light.png) | 1440×1000 |
| [U1-saved-1440-light.png](U1-saved-1440-light.png) | 1440×1000 |
| [U1-saved-390-dark-zh.png](U1-saved-390-dark-zh.png) | 390×844 |
| [U2-switch-1440-dark.png](U2-switch-1440-dark.png) | 1440×1000 |
| [U2-toggle-empty-1440-light.png](U2-toggle-empty-1440-light.png) | 1440×1000 |
| [U3-invalid-390-dark-zh.png](U3-invalid-390-dark-zh.png) | 390×844 |
| [U4-end-1440-dark-rtl.png](U4-end-1440-dark-rtl.png) | 1440×1000 |
| [U5-open-390-dark-rtl-zh.png](U5-open-390-dark-rtl-zh.png) | 390×844 |
| [U6-compare-1440-dark.png](U6-compare-1440-dark.png) | 1440×1000 |
| [U6-status-390-light.png](U6-status-390-light.png) | 390×844 |
| [after-cards-invalid-1440-light.png](after-cards-invalid-1440-light.png) | 1440×1000 |
| [after-checkbox-1440-light.png](after-checkbox-1440-light.png) | 1440×1000 |
| [after-constraint-1440-light.png](after-constraint-1440-light.png) | 1440×1000 |
| [after-draft-1440-light.png](after-draft-1440-light.png) | 1440×1000 |
| [after-field-1440-light.png](after-field-1440-light.png) | 1440×1000 |
| [after-field-invalid-1440-light.png](after-field-invalid-1440-light.png) | 1440×1000 |
| [after-raw-busy-1440-light.png](after-raw-busy-1440-light.png) | 1440×1000 |
| [after-raw-cancelled-1440-light.png](after-raw-cancelled-1440-light.png) | 1440×1000 |
| [after-raw-choices-1440-light.png](after-raw-choices-1440-light.png) | 1440×1000 |
| [after-raw-choices-390-light.png](after-raw-choices-390-light.png) | 390×844 |
| [after-raw-failed-1440-light.png](after-raw-failed-1440-light.png) | 1440×1000 |
| [after-raw-saved-1440-light.png](after-raw-saved-1440-light.png) | 1440×1000 |
| [after-raw-select-empty-1440-light.png](after-raw-select-empty-1440-light.png) | 1440×1000 |
| [after-raw-select-long-1440-light.png](after-raw-select-long-1440-light.png) | 1440×1000 |
| [after-raw-slider-1440-light.png](after-raw-slider-1440-light.png) | 1440×1000 |
| [after-select-names-1440-light.png](after-select-names-1440-light.png) | 1440×1000 |
| [after-slider-end-1440-light.png](after-slider-end-1440-light.png) | 1440×1000 |
| [after-switch-1440-light.png](after-switch-1440-light.png) | 1440×1000 |
| [after-toggle-1440-light.png](after-toggle-1440-light.png) | 1440×1000 |

## 边界

这不是 R2 评审，不把本次截图视为完整覆盖矩阵。Check错误入口、完整保存中/键盘/长错误/Portal等R1补证项目仍由review-ui逐项确认；OS IME、VoiceOver、真机触控和浏览器UI原生200%缩放仍为P5 missing。未重跑全workspace、Docs islands、hydrate或GPU/性能整套；相关旧结果保留原适用范围。
