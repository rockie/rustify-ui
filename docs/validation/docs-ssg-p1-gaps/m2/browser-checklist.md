# V-6 · 同候选真实浏览器走查

候选 `rustify-docs 0.1.3`；源码/二进制和四套 production 产物的身份见 [matrix-artifacts.json](matrix-artifacts.json)。真实Safari与现有Chrome模拟已按[D-04](browser-decision.json)收口，见[M2记录](../M2.md#本轮收口证据)；最新版Chrome和两种真机明确延期。预览监听0.0.0.0，以下为当前en0地址；网络变化后替换host，端口保持。设备是否能到达仍须实测。

| 产物 | 本次入口 | 重新启动（从仓根执行，端口从新输出读取） |
| --- | --- | --- |
| 完整站根路径 | http://192.168.123.44:62159/ | `target/docs-ssg/gaps/rustify-docs-candidate preview --dir target/docs-ssg/gaps/matrix/full-root/dist --host 0.0.0.0 --port 0` |
| 完整站子路径 | http://192.168.123.44:62178/rustify-ui/ | `target/docs-ssg/gaps/rustify-docs-candidate preview --dir target/docs-ssg/gaps/matrix/full-subpath/dist --host 0.0.0.0 --port 0` |
| 首页边界根路径 | http://192.168.123.44:62348/ | `target/docs-ssg/gaps/rustify-docs-candidate preview --dir target/docs-ssg/gaps/matrix/home-root/dist --host 0.0.0.0 --port 0` |
| 首页边界子路径 | http://192.168.123.44:62411/rustify-ui/ | `target/docs-ssg/gaps/rustify-docs-candidate preview --dir target/docs-ssg/gaps/matrix/home-subpath/dist --host 0.0.0.0 --port 0` |

localhost 的安全上下文可在同产物上另起默认 loopback preview（命令去掉 `--host`），测试复制成功；HTTP 局域网的复制应显示拒绝并可手选。若已有 HTTPS 入口可用，也可在该入口验证复制成功。

在最新稳定 Google Chrome、真实 macOS Safari、iPhone/Safari、Android/Chrome 各完成一次，并记录浏览器完整版本、设备/OS、走查人、时间、入口与 manifest 指纹。Chrome 的稳定渠道依据与冻结性能 Chromium 分开登记；本机旧 Chrome 的补充观察不计这一出口。

1. 完整站：首页 → Get started → guide/setup → 侧栏 / 页头 Guide → 前后页。确认当前项、普通/中文/显式标题锚点正确，标题未被页头遮挡，代码块和表格内部可滚动。
2. 切深色后跨页，再切浅色；浏览器禁脚本后复走阅读、侧栏、目录、前后页。桌面检查宽窄布局和真实 200% 缩放，适用设备检查减少动态效果。
3. 进入深地址 `a/b/c/missing/`：实际 HTTP 404、完整主题和返回首页入口；查看 console 与失败请求，区分这一预期 404 和意外资源错误。
4. 首页边界站：首页 `#home`、`explicit/#%E5%85%A5%E5%8F%A3`、`raw/#%3C%2Fh1%3E`；直接访问、Same 与跨页链接均到唯一标题。raw 页正文只应是 Same / Automatic，不残留标题或 template 的内容；法语页面的 Lire 保留 `lang=fr`，界面导航回落英语。
5. 边界站 `empty/`：actions、top/social、日期、编辑入口全空，仍有页头、navigation、main、contentinfo；页脚真实链接返回该产物的基路径首页。两 base、宽窄/无脚本下重复。
6. 复制一个代码块：在 LAN HTTP 上验证失败说明和手选；在同产物 localhost/已有 HTTPS 上验证成功并核对原文。保留必要截图，以及每步的结果、console 和失败请求。

结果按组合追加到 [M2 记录](../M2.md)，注明未运行/失败/环境缺失。本轮按D-04仅以真实Safari和现有Chrome模拟作为V-6出口；延期的真实矩阵项目不记通过。换产物或修代码后，受影响组合须在新的同候选上复验。
