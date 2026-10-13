disposition: ship

当前结论只覆盖唯一视觉修复及 deep 404 / 布局缩放补证。下方 persistence 至 keep 保留首次 r1 评审记录（当时 disposition: fix）；修复后评分见文末 verdict / remaining，不重新全范围寻找风格问题。

范围：M3 注册页、slots 与导入主题的只读视觉评审；依据已扩展阅读 brief 和 DESIGN.docs-ssg.md 的继承规范。未运行浏览器、Cargo 或 detector，未修改产品。未独立审计 PRODUCT.md、字体字节/实际字体命中或组件行为；未把未读文件写成缺失。r1 完整浏览器验收尚未通过，本报告不替代其验收结果。

## persistence

本次方向有持久依据：`.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md` 的 registered pages / layout slots / imported themes 扩展，以及 `DESIGN.docs-ssg.md` 的 36rem、quiet chrome、原生窄屏导航和稳定恢复占位规则。显式导入主题可提供字体、颜色和圆角，不能把默认 P1 的 system fonts / 6px 当作导入主题的缺陷。

已逐张打开 `ui-subpath-r1/` 的六张图：`first-frame-light-light.png`、`first-frame-dark-dark.png`、`first-frame-system-light.png`、`first-frame-system-dark.png`（均 1280×3556），`custom-no-script-375.png`（375×5609）、`custom-no-script-640.png`（640×10144）。六图均含文档顶部，状态/主题与文件名相符，无移位版头或意外黑块。四张 first-frame 仍显示 Loading；这不是 ready 状态的视觉证据。

## fidelity

| 评审项 | 亲见结果 / 边界 |
| --- | --- |
| TYPE | 导入主题的正文、serif 样本和 mono 样本可辨，Unicode/中文/emoji 未见缺字方块。未凭截图证明具体字体文件命中。200% 图中岛内说明/控件相对正文及外部状态文字明显较小；需区分 root 字体缩放采集方式与实际浏览器 zoom，当前不直接判为产品缺陷。 |
| MATERIAL / GROUND | 浅/深两套画面保留平面、细分隔和安静源码面板；主题圆角/颜色变化属于 brief 授权适配。未新增装饰阴影或另一套应用身份。未逐值对比原 ThemeDocument。 |
| 框架与阅读列 | header override 位于身份区域；Counters / Catalog proxy 与脚本模式控件仍可见。桌面保留 sidebar / article / TOC；文章和源码共享继承阅读列。窄屏改为 Menu / On this page，未见全页横向裁切。未测 computed 36rem 或操作导航。 |
| Slots | header、sidebar top/bottom、before-content 演示、after-content 文案和 footer 在对应画面各出现一次，未见响应式复制出的重复演示。六图全是 Custom page，没有 deep 404，不能据此审定第七个 not_found slot 或 404 的共享附加内容。 |
| 源码 / no-script | 源码路径低于正文层级；375 与 200% 无脚本图中 Function / Full file 连续可读，Copy/mode 不作为死操作出现。原生控件呈禁用状态，启用 JavaScript 的说明在外部状态行；源码长行留在面板中。未独立验证键盘、复制、ARIA 或启用后的浮层。 |
| 作者控件 / 内容 | **需修正**：Jump to details 等链接之后、Two matching registered demos 之前，出现两组三个无可见文案的细长空边框。六图均可见，浅色桌面约为 106×14px，窄屏也没有可辨作用。不能把它们的来源猜成字体加载失败。 |

## ceiling

当前阅读层级与响应式框架保留，但空边框打断文章，既不能作为可读控件，也没有内容价值。没有独立 QUALITY BAR card；本次以已有 Read 合同为准，不要求新品牌、装饰或字体方向。尚未捕获 deep 404、ready/open 控件及浅色窄屏状态，这些只列为覆盖边界，不宣称已验收。

## material_fixes

1. 处理六张图中的两组三个空边框：若为采集/测量探针，应从读者的可见阅读流移出；若为作者控件，应恢复可见标签和可读尺寸。保留它们所需的验收能力，不能靠字体采集器故障解释可见空框。

剩余一次固定 capture batch 请同时补修正后的同页浅/深状态与一张 deep 404，并明确 200% 作者控件的实际缩放覆盖；随后只评分本项修复，不重新全范围找问题。r1 采集器修复后的行为/字体结果由主实施者另行收口。

## keep

保留继承阅读列、普通框架导航和 mode、单份 slot 内容、外置稳定状态行、安静相对源码路径及无脚本的连续源码阅读。

## verdict

1. **resolved — 两组无文案空框：** 已逐张打开 `custom-subpath-r3/` 递归目录中的七张新图。四张 light/dark/system first-frame 和 375/640 无脚本图均显示三色填充短条，并分别带可见的 “Image example” / “Background example” 说明；其图形样本用途已可辨，不再呈现为无标签空控件。初审只描述了空框的可见外观，没有据此认定字体加载失败；本次依据图形和说明评分，未扩展为代码审计。

补充证据边界已闭合：`custom-deep-404.png`（1280×720）显示 Page not found、错误正文和 return home，并保留 header、框架导航/mode、sidebar top/bottom、before/after-content 文案及 footer。它补齐了初审未看到的 not_found / deep 404 表现；不据此声明已审所有页面或操作结果。

`custom-no-script-640.png` 是 640 CSSpx、DPR 2 的布局缩放模拟，产出 1280×10922 的全页图；375 图为 375×5665，四张 first-frame 均为 1280×3612。七图均从文档顶部、状态/主题与文件名相符，无移位版头或异常遮挡。新版 200% 图中正文、源码、岛内说明和作者控件的尺度关系正常，启用 JavaScript 的外部说明与连续源码仍可读。亲读 `results.json` 的 inline `custom-no-script-zoom-375.json` / `custom-no-script-zoom-640.json`：两者 bodyFont 都是 16px、controlFont 都是 14px，DPR 分别为 1/2，physicalControlFontPixels 分别为 14/28，kind 为 layout zoom emulation。初审 root 字号缩放造成的采集歧义已澄清；这不是用户浏览器原生 zoom 或物理设备验收。

`custom-subpath-r3/results.json` 记录 12 个 expected 测试全部 passed，skipped/unexpected/flaky 均为 0。这是 supplied harness 的子路径证据；本评审未独立运行浏览器、Cargo 或 detector，未把 r1 失败记录改写为通过。

本次修复引入的回归：在提供的七图中未见。

## remaining

clear。Ship 仅覆盖本次评分的修复与上述补证，不代表整个表面或 M3 全部验收。首次评审中未独立审计的行为、字体字节/具体字体命中、ready/open 控件、浅色窄屏和实际原生 zoom / 真机边界继续保留；meta 与 V-5 复跑由主实施者另行收口。本轮不再请求 capture batch 或新风格调整。

disposition: ship
