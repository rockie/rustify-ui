---
name: Rustify UI Landing
description: 面向中文开发者的 Rustify UI 介绍页；仅适用于 pages/。
colors:
  primary: "#2348e5"
  mint: "#c9f3dd"
  background: "#f5f7fb"
  foreground: "#16243b"
  muted-foreground: "#4a5870"
  supporting-text: "#596780"
  white: "#ffffff"
  showcase: "#e7edf8"
  divider: "#ccd5e5"
  action-border: "#b7c2d7"
  action-hover: "#e3e9f4"
  link: "#173dc8"
typography:
  display:
    fontFamily: "Sora, Noto Sans SC, sans-serif"
    fontSize: "clamp(2.4rem, 7.1vw, 6rem)"
    fontWeight: 600
    lineHeight: 1.08
    letterSpacing: "-0.04em"
  headline:
    fontFamily: "Sora, Noto Sans SC, sans-serif"
    fontSize: "1.875rem"
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "-0.03em"
  title:
    fontFamily: "Sora, Noto Sans SC, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
  body:
    fontFamily: "Sora, Noto Sans SC, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.625
  label:
    fontFamily: "Sora, Noto Sans SC, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 400
  code:
    fontFamily: "monospace"
    fontSize: "13px"
    lineHeight: "28px"
rounded:
  action: "14px"
  panel: "16px"
spacing:
  unit: "4px"
  gutter: "24px"
  desktop-gutter: "48px"
  section: "80px"
  desktop-section: "112px"
components:
  action-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.white}"
    rounded: "{rounded.action}"
    height: "56px"
  action-secondary:
    textColor: "{colors.foreground}"
    rounded: "{rounded.action}"
    height: "56px"
  collaboration-panel:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.white}"
    rounded: "{rounded.panel}"
  showcase-panel:
    backgroundColor: "{colors.showcase}"
    rounded: "{rounded.panel}"
---

# Design System: Rustify UI Landing

## Overview

**Creative North Star: "Technical construction kit"**

冷色浅底、深色文字与大块钴蓝组成直接、清晰的开发者页面。大字号直立标题与宽松留白建立层级，真实 Theme Studio 截图提供产品证据。此规范仅约束 `pages/` 的静态 DOM 落地页；仓库根设计规范继续约束 SDK 与 Theme Studio。

这是已实现页面的提炼，依据 [页面源码](content/index.md)、[主题](theme.json)、[字体配置](rustify-docs.toml) 和 [方向契约](.impeccable/direction.md)。对照了现有桌面、手机与焦点截图，以及 [浏览器记录](.impeccable/review/browser-check.json) 和 [交互记录](.impeccable/review/interaction-check.json)。本次文档整理未重跑浏览器；Phoenix 固定引擎缺失，未进行引擎校验。没有已批准的栅格设计稿。

**Key Characteristics:**

- 直立的大标题、精简段落与细分隔线。
- 通过色面与间距组织内容，无阴影叠层。
- 同屏并列展示 DOM 与 GPU 职责。
- 真实界面截图与可读的运行命令。

## Colors

### Primary

钴蓝 `primary` 用于首要文档操作和 DOM/GPU 协作面板；正文链接使用较深的 `link`。

### Secondary

浅薄荷 `mint` 用于 GPU 字样、结尾行动区和蓝底键盘焦点。它承担局部强调。

### Neutral

`background` 是整页冷色底，`foreground` 是标题及代码区深底，`muted-foreground` 是说明文字。`showcase` 托起截图；`divider` 分隔功能行和页脚；`action-border` 与 `action-hover` 表达次要按钮边界和悬停状态。前置 YAML 为颜色值的唯一规范层。

**The Fixed Daylight Rule.** 本页已使用的主色、背景与文字在浅色、深色偏好下保持一致；不要据此推导另一套深色页面。

## Typography

**Display / Body Font:** Sora、Noto Sans SC、sans-serif。拉丁字形由本地 Sora 承担，中文由本地 Noto Sans SC 承担；代码使用 monospace。

标题采用半粗字重，字距收紧；中文段落保持直立与舒展行距。显示标题保留源码换行。章节标题在中等断点起增至（2.25rem）；引导段从（1.125rem）增至（1.25rem），最大宽度（39rem）。一般说明段落宽度约（43–59ch）。

字体通过本地资源加载并使用 `font-display: swap`。Sora 配置字重（100–800），Noto Sans SC 配置（400–700）。中文文件只包含当前内容页、表单提示与中文 404 使用的汉字和 CJK 标点；新增字前应按 [来源说明](fonts/noto-sans-sc-source.txt) 更新子集。当前资产不代表完整中文覆盖。

## Layout

页头与正文容器最大宽度（84rem），左右留白使用 `gutter`，大断点起使用 `desktop-gutter`。页头最小高度（80px）；主内容顶部留白从（48px）增至（64px）。大章节使用 `section` 与 `desktop-section` 的垂直间距。

小、中、大断点分别为（40rem）、（48rem）、（64rem）。首屏两项操作始终并列，操作组最大宽度（26rem）。协作面板在手机上保持两列，DOM/GPU 字样不换行；中断点起加入独立居中的连接符，两侧内容顶对齐。功能行、章节导语与上手区在中断点起采用双列；手机回到单列。代码只在自身区域横向滚动，不撑宽页面。

## Elevation & Depth

本页不使用投影。浅底、钴蓝、淡蓝截图区与深色代码区形成色面层级；细边线连接相关内容，留白分隔章节。

## Shapes

大色面使用 `panel` 圆角；操作按钮、截图裁切与代码容器使用 `action` 圆角。后者来自主题半径生成的（14px），不是默认 Tailwind 数值。功能说明采用平铺行与上下边线。

## Components

### Buttons and links

首屏主按钮为钴蓝白字，次按钮为透明底深字与细边框；均居中、半粗字重，最小高度（56px）。横向内边距从（12px）增至（24px）。主按钮悬停混入前景色，次按钮使用 `action-hover`。普通内容链接悬停显示下划线。默认键盘焦点为（3px）实线轮廓、偏移（2px）。结尾薄荷色区使用深底白字按钮。

### Collaboration panel

DOM 为白色，GPU 为薄荷色；底部边线分出共享状态说明和架构链接。面板 hover 或 focus-within 时，连接线变薄荷色、链接箭头向右移动（4px），持续（200ms）。架构链接 focus-visible 使用薄荷色轮廓与（4px）偏移。减少动态效果偏好下停用位移和过渡；焦点轮廓仍可见。

### Product proof and code

使用 [Theme Studio 原始截图](content/theme-studio.png)，其 PNG 元数据记录仓库来源 `docs/validation/leptos-0.8.22/theme-studio.png` 与提交 `032756c041615dca075a49685fbb458ce7cc833f`。源图（1440×1000）未作视觉改写；页面以（16:10）顶部对齐裁切展示，保留真实示例说明及替代文本。命令区为深底、等宽字体、带说明栏的可聚焦滚动区域。

## Do's and Don'ts

### Do:

- **Do** 复用此页已有色面、细分隔线与留白层级。
- **Do** 在窄屏同时展示 DOM/GPU 标签，并保留键盘焦点和减少动态效果反馈。
- **Do** 为新增中文更新字体子集，为产品截图保留真实来源。

### Don't:

- **Don't** 把本页规范扩展为 SDK 或 Theme Studio 的规范。
- **Don't** 用生成的产品界面或虚构指标替换真实产品证据。
- **Don't** 把中文字体子集、未消费的主题色或未经验证的主题状态当作完整系统能力。
