# M5 真实浏览器走查清单（V-12 早期冒烟）

计划：[DOCS-SSG-P1](../../../plan/DOCS-SSG-P1.md) §9.3 · 记录：[M5](../M5.md)

用途：在真实 Safari（本机）与可用的手机上，对同一份产物走一遍 P1 的阅读、导航与主题路径。Playwright 的 WebKit/Firefox 只是引擎回归，不能替代这一步。

## 准备（由实施者执行）

```sh
mbx build --locked --profile docs-cli -p rustify-docs-cli
sh tests/docs-ssg/serve.sh subpath 4192          # 构建内容 fixture，preview 于 http://127.0.0.1:4192/rustify-ui/
# 手机：同一产物以局域网地址只读服务（非安全上下文，复制应显示失败并选中代码）
target/docs-cli/rustify-docs preview --dir target/docs-ssg/http/subpath/dist --host <本机局域网 IP> --port 4194
```

## 步骤（走查人逐条勾选，记录浏览器版本/系统）

| # | 步骤 | 期望 |
| --- | --- | --- |
| 1 | 打开首页 `/rustify-ui/` | 标题、标语、两个按钮、特性列表；无横向滚动 |
| 2 | 点「Get started」进入 Guide | 侧栏当前项高亮；右侧（≥1280）或上方折叠（更窄）的「On this page」 |
| 3 | 依次点侧栏各页、上一页/下一页 | 每页都能到达；返回键正常 |
| 4 | 在 Setup 页切换代码分组到「Config」，再进入 Guide 页 | 同名分组都显示 Config；换页后保持 |
| 5 | 点代码块「Copy」 | 本机 Safari（127.0.0.1）：显示「Copied」，粘贴内容与代码完全一致；手机局域网地址：显示「Copy failed」且代码被选中，可手动复制 |
| 6 | 切换 Light / Dark / System，再换页 | 立即生效，换页无闪烁，保持选择；系统深色时 System 跟随 |
| 7 | 滚动 Setup 页 | 「On this page」标记当前章节，到底部标记最后一节 |
| 8 | 窄屏/手机：打开 Menu、On this page，点其中链接；按 Esc（键盘） | 面板展开可读；点链接后收起并到达目标 |
| 9 | 访问不存在地址 `/rustify-ui/x/y/` | 404 页（同样式），可回首页 |
| 10 | Safari 设置中停用 JavaScript 后刷新 Setup 页 | 正文、全部代码块（分组的每个标签都显示）、导航、目录都可读；无主题切换按钮 |
| 11 | 桌面 Safari 缩放到 200% | 无页面级横向滚动，文字不重叠 |
| 12 | VoiceOver（可选，本期只冒烟）：用转子查看地标与链接 | 有 banner、navigation「Menu」、main；侧栏链接可列出 |

## 记录

| 环境 | 版本 | 走查人/日期 | 结果 | 问题 |
| --- | --- | --- | --- | --- |
| macOS Safari | | | | |
| iOS Safari / Android Chrome | | | | |
