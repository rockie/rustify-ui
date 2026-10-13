# DOCS-SSG-P2 M5 设计同步记录

## Overview

本次将官方英文站已实现的设计增量写入 [DESIGN.docs-ssg.md](../../../../DESIGN.docs-ssg.md) 的 “English SDK site (P2 M5)” 章节，供后续维护首页、分类演示与指南时使用。沿用 [原 shell brief 的英文官方站扩展](../../../../.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md#p2-extension-the-english-sdk-site) 和 P1 的 Read 身份、冷中性色、系统字体、36rem 阅读栏、细分隔与响应式导航。

呈现路径为 DOM：静态首页与指南使用文档 shell，二十四个分类使用显式 DOM demo 岛；GPU 能力通过仓库示例链接说明。设计同步没有新增 token、视觉世界或图片资产，只新增 M5 章节与本记录。根 DESIGN.md、frontmatter、sidecar 以及既有 M2–M4 规则保留原位。

## Components

| 同步规则 | 直接核对的源码 |
| --- | --- |
| 静态首页单 H1、定位句、Quick start / Components 普通链接与三项 hairline features；介绍由原 README 的 Markdown writer 读取 | [pages.rs](../../../../site/src/pages.rs) 的 `home`、[lib.rs](../../../../site/src/lib.rs) 的 Home 注册；1280 首屏两入口由既有 finish 截图支持 |
| Guides 十二条、Components 二十四条导航；二十四个真实 demo 按模块路径注册，分类正文与能力项从 CATALOG 读取 | [rustify-docs.toml](../../../../rustify-docs.toml)、[site registry](../../../../site/src/lib.rs)、[demos/mod.rs](../../../../site/src/demos/mod.rs)、[pages.rs](../../../../site/src/pages.rs) 的 `category` |
| 分类沿用 canvas / 外部状态 / 精确 function 与 full-file source，能力表使用 `rd-table` 命名可聚焦 region，宽表由容器内部滚动 | [pages.rs](../../../../site/src/pages.rs) 的 `Component capabilities` region、[docs.css](../../../../crates/rustify-docs/assets/docs.css) 的 `.rd-table` / `overflow-x: auto`；已有 demo/source 规则继续适用 |
| Link 演示的文档 fragment 设置 `attr:target="_self"`，通过原生文档导航越过岛内局部 router；disabled 状态仍由该 demo 的信号控制 | [basics.rs](../../../../site/src/demos/basics.rs) 的 `link_demo`；[M5 实施记录](../M5.md) 保留导航失败与修复证据 |
| Docs 默认后代 scope 继承文档 theme properties，避免 light fallback 覆盖 dark / system-dark；显式导入 projection 在默认 token 后输出 | [style.rs](../../../../crates/rustify-docs/src/style.rs) 的 `tokens_css` 与 `site_css_with_theme`；[official.spec.ts](../../../../tests/docs-ssg/official.spec.ts) 的三模式 grid / body 配色等值断言 |

规则描述的是当前源码与已确认界面，不将 SDK 其他应用改为文档阅读布局，也不要求 GPU 组件与 DOM 演示对等。

## Evidence

[首批 finish review](finish-review.md) 的 disposition 为 `fix`，唯一问题 M5-F1 是深色 Data table 数据行不可辨认。[唯一修复确认](finish-review-r2.md) 将 M5-F1 评为 resolved：四行两列文字恢复可读，深色表体取样为 `#0c111d`、文字为 `#f9fafb`；其他八张 PNG 与首批字节一致，无新增可见回归。该 `ship` 仅覆盖已评分修复及截图可见回归。

确认包为 [official-subpath-r5/capture-batch](official-subpath-r5/capture-batch/manifest.json)，其中九图已由同一独立 reviewer 实际查看：桌面 1280×900 浅/深色，窄 viewport 375×812 浅色，页面为首页、Data table 和 Quick start。尺寸、字节数与 SHA-256 匹配 manifest 及 results.json 的原浏览器附件。对照图为 [原深色失败](official-subpath-r3/capture-batch/official-category-desktop-dark.png)、[深色修复确认](official-subpath-r5/capture-batch/official-category-desktop-dark.png)、[首页浅色](official-subpath-r5/capture-batch/official-home-desktop-light.png) 与 [窄屏指南](official-subpath-r5/capture-batch/official-guide-narrow-light.png)；没有为本次文档同步另拍截图或重开风格评审。

[M5.md](../M5.md) 另记录 CLI 0.1.9 在本地 Chromium 153.0.8010.12 下的 root、subpath、meta 三组各 47/47，以及真实 dev 与受影响导入主题用例。它们是主实施者的原验证结果，未在本次文档同步重新执行；其范围与静态视觉评分分别保留。首批功能测试通过不替代 M5-F1 修复前的视觉失败记录。

本次视觉证据不覆盖其他分类或指南的完整外观、下方 Capabilities 表与 GPU 链接、README 后续段落、窄屏首屏以下的 source，亦不扩为行为、性能、字体加载、导入主题或全站验收。375px 与测试中的 200% emulation 不计作实体手机或浏览器原生缩放。M6 首次用户、实体设备、Safari 和线上出口继续按原计划保存其验证状态；此记录不关闭这些出口，也不代表候选已冻结或公开部署。

## Checks

本次只读原规范、M5 实施记录、两次 finish 报告和相关实际源码，编辑范围为 DESIGN.docs-ssg.md 新增 M5 章节与本文件。documentation-writer 的文档检查与 `git diff --check` 均 exit 0；本地路径与 Markdown 标题锚点自检覆盖 DESIGN.docs-ssg.md 的 35 项及本文件的 22 项链接，失败为 0。未运行 Cargo、浏览器、端口服务、context loader 或 detector。
