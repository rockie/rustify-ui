# V-12 · 真实 Google Chrome 稳定版走查

计划：[DOCS-SSG-P1](../../../plan/DOCS-SSG-P1.md) §9.3 · 记录：[M8](../M8.md) · 清单：[m5/walkthrough.md](../m5/walkthrough.md)

- 环境：macOS 27.0.1，**Google Chrome 154.0.8037.93**（`navigator.userAgentData` 的 fullVersionList；用户本机日常使用的稳定版，经 Claude in Chrome 扩展在独立标签组中操作，真实指针点击与滚轮），2026-10-10 21:50 +08:00。
- 入口：`preview` 的 `/rustify-ui/` 产物 `http://127.0.0.1:4192/rustify-ui/`（loopback，安全上下文；与 Safari 走查同一产物）。走查后清空了该源的 localStorage 并关闭标签。

| # | 步骤 | 结果 |
| --- | --- | --- |
| 1 | 首页 | 标题 Fixture Docs、标语、两个按钮（Get started / Reference）、特性两列；无横向滚动（scrollWidth − innerWidth = 0） |
| 2 | 点 Get started | 到 `/rustify-ui/guide/`，侧栏当前项 Guide，右侧 On this page 可见（3 项），Next 指向 Setup |
| 3 | 侧栏切到 Setup、再回 Guide | 均到达，当前项随之变化 |
| 4 | Setup 切代码分组到 Config | 同页两个 `install` 分组同时变为 Config，存入 `rustify-docs:group:/rustify-ui/:install`；进入 Guide 页，其分组也显示 Config；回到 Setup 仍为 Config |
| 5 | 点代码组的 Copy | 状态区播报 “Copied”，按钮随后复原。为核对剪贴板内容调用了 `navigator.clipboard.readText()`，触发 Chrome 的剪贴板读取权限询问而未完成——这是走查脚本的问题，不是站点的；复制原文逐字一致已由 Playwright（V-8）覆盖 |
| 6 | 切到 Dark，再导航到 Setup | `data-theme=dark`、背景 rgb(12,17,29)、存储 `dark`；新页面首帧即为深色（mode.js 在 `<head>` 同步执行，先于样式表绘制） |
| 7 | 用滚轮滚到 Setup 底部 | On this page 标记最后一节 “Check the version” |
| 9 | 访问 `/rustify-ui/x/y/` | HTTP 404，站点 404 页（同主题与深色模式），“Back to the home page” 链接 |

未在此浏览器做的项及原因：8（窄屏面板，需要改变用户窗口尺寸）、10（停用 JavaScript，需要修改浏览器设置）、11（200% 缩放，扩展不支持页面缩放快捷键）、12（VoiceOver）。这些在 Playwright Chromium（窄屏 375/320、禁脚本、200% 等效宽度）中已有自动化覆盖，真实设备上由 Safari/手机走查覆盖。

扩展的点击偶有未投递（同一坐标重试即生效，点中时控件均即时响应），与站点无关。
