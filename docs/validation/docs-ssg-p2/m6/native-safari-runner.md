# 原生 Safari 兼容输入 runner 交接

[verify-official-safari.mjs](verify-official-safari.mjs) 仅做验证。Root 在安装候选的官方 dist 上串行启动两个已有 host 后运行；脚本不构建、不启动服务，不改产品、Safari 偏好或用户已有标签页，也不拍截图。

## 输入与执行

类别和源码地址来自当前 C4 的 [formal-a-official-islands-c4-r1.json](formal-a-official-islands-c4-r1.json) 二十四项 categories / source_sha256；脚本从 [measure-official-islands.mjs](measure-official-islands.mjs) 提取 until / runProbes 及其局部 helper，不复制另一份类别清单，不复用该报告的 Chromium 性能结论。

执行时读取仓根 `target/docs-ssg/m6-installed-c.json`，核对实际 installed binary SHA 与 source identity，再与 host manifest 的 cli_version 配对；不再读取旧 C2 身份文件。Root 应先完成当前候选重冻/安装/official build，再从实际 host 输出设置 SAFARI_HEADER_ORIGIN 和 SAFARI_META_ORIGIN，分别包含真实端口与 base。以下命令各执行一次并串行运行，已有输出不得覆盖：

```sh
node docs/validation/docs-ssg-p2/m6/verify-official-safari.mjs --origin "$SAFARI_HEADER_ORIGIN" --out docs/validation/docs-ssg-p2/m6/native-safari-header-c4-r1.json
node docs/validation/docs-ssg-p2/m6/verify-official-safari.mjs --origin "$SAFARI_META_ORIGIN" --out docs/validation/docs-ssg-p2/m6/native-safari-meta-c4-r1.json
```

脚本通过 `/usr/bin/safaridriver --mcp` 的 stdio NDJSON JSON-RPC 初始化，创建一个自有 about:blank tab。每次页面动作前显式切回该 handle，console/network 调用另传自有 handle；不调用 list_tabs 或读取原有 tab。finally 尝试恢复自有上下文的 viewport / 文档主题存储，关闭该自有 tab，并终止 MCP 子进程；失败清理会使报告不通过。

## 覆盖与记录

遍历首页、十二篇指南、二十四个组件页与深层 404，检查英语 lang、单 H1、实际导航 href / 当前项、meta CSP、原 README 介绍与 Function / Full file raw template 的原字节。二十四类输入复用现有公开状态断言，在 ready、font readiness 和两个 animation frame 后才开始。

`__officialNativeKey` 创建队列项与 promise；Node 轮询页面 snapshot，使用 MCP page_interactions 的 native keyPress，再 resolve 对应队列项。没有用合成 KeyboardEvent 替代默认键盘行为。原探针的文本输入仍是原生 value setter 加 InputEvent，普通 DOM 点击也沿用原探针语义；这不是全部人类输入或读屏测试。

Inspector 网络/console、wasm MIME 与页面 error / unhandledrejection / CSP 事件进入报告。首次导航前在自有 about:blank 清空并订阅 network / console，后续每页同样先清空再导航；缺少确切主文档请求会失败，零请求不算通过。wasm 读取实际 `detail.request.response_headers` / `response_mime_type`，同时要求列表和详情 status 为 200、Content-Type 与 response_mime_type 都为 application/wasm。

Safari Inspector 会把 head 的 data SVG icon 列为资源。只有 URL 与实际 [shell.rs](../../../../crates/rustify-docs/src/shell.rs) 的唯一声明、页面 head 原 href 都精确一致，status 为 200 且 MIME 为 image/svg+xml 时，才记录为独立 `inline_resources`。其他 data 资源拒绝；HTTP(S) 请求仍严格要求同源且不得包含 Makepad。favicon 等 HTTP 失败不作预期过滤；只允许 deep404 主文档确切 URL 的 404。

必要小范围检查是普通首页阅读链接、Data table light/dark 配色、375/320px 页面无横向溢出，以及窄导航 details 的 native Enter。请求宽度与实际 innerWidth 分别保存；实际宽度不等于请求值时仍失败，不以 CSS 伪造原生 320px。

结果保存原 MCP responses、页面输入步骤、错误、来源摘要和清理结果。逐页完成后保存进度；首个验证失败即停止后续页并列出未运行 routes，finally 仍保存失败与清理证据。ready 轮询记录真实 visibility / focus、coordinator state / failure / stats 和 mounted / inert / disabled；hidden 明确作为可见 tab 环境缺口拒绝，不继续等待三十八页。单 RPC 上限三十秒、单页面就绪三十秒、输入九十秒，整个 MCP 运行上限十五分钟；超时取消后只允许自有 tab 清理动作。

## 尚未验证

先前 C2 修订仅完成静态核对。当前 C4 已实际复查：[header C4 r1](native-safari-header-c4-r1.json) 在第一个首页观察到 hidden / focus=false，首败停止；其余37页未执行。自身标签页已关闭、MCP及预览进程已退出。Node语法通过，旧C2报告与验证器快照保留。

Root 的 [safari-owned-schema-probe-c2-r1.json](safari-owned-schema-probe-c2-r1.json) 已记录一个实际 C2 自有 tab 的 evaluate、network、wasm 详情和空 console 原响应，并记录自有 tab 已关闭。本轮据该记录完成 MIME 字段适配与静态解码核对，保留原就绪等待。协议层仍保存原响应；未知列表或字段形状明确失败，不按空数组通过。

Root 实际 [header C2 r1](native-safari-header-c2-r1.json) 尝试三十八页，零页通过：十四个静态页被原验证器的 data icon 分类阻断，二十四个组件页触发产品三十秒 startup deadline。普通首页链接、375px 和 native details Enter 的局部检查通过；dark grid 检查失败，320px 请求实际得到 336px。此原始报告全部 samples 保留，不能据局部结果宣称完整输入门通过。

[单页 C2 诊断](safari-startup-diagnostic-c2-r1.json) 的十二个 snapshot 均为 hidden / focus=false，raf 计数停在 1，已有 mounted seed。实际 [islands.js](../../../../crates/rustify-docs/assets/islands.js) 路径是 parsed → boot → bind → docs_hydrate → 两个 rAF → attached 检查 → complete；[mode.js](../../../../crates/rustify-docs/assets/mode.js) 在 parser 进入 interactive 后启动三十秒期限，失败后恢复静态 canvas 并释放 runtime。由这些证据推断，MCP 自有 tab 的不可见状态阻断了完成所需绘制回调；wasm HTTP 成功本身不能证明 coordinator 已 complete。[DOM loader](../../../../crates/rustify-ui/web/dom-loader.js) 的 boot 还包含 compileStreaming、glue import / init、protocol 检查和 runtime boot；coordinator stats 在 bind 前为 null，可帮助区分尚未 bind 与绑定后的失败。

Root 可在当前候选上有界复查可见性；hidden 时首个页面即停止，不盲目遍历。完整两 host 与二十四类 native key 主路径仍未通过。后续运行应指定新的 --out，保留上述 C2 原报告与 [C2 版本验证器](verify-official-safari-c2-history.mjs)。验证器不改产品期限、document visibility 或 rAF 实现；当前候选的实际结果以新的 C4 报告为准。

此 runner 属于水合后的兼容输入门，不证明原始 SSR DOM 身份保留、水合布局位移为零或性能预算。它也不替代实体手机、浏览器原生 200% zoom、VoiceOver、完整 V-11 或线上验收。


当前C4的实际命令/来源见[command](native-safari-header-c4-command-r1.json)。这次没有修改Safari设置、产品期限、visibility或rAF。尚未运行C4 meta host；须先具备真实可见入口，再按上述新文件名接续，不重写本轮hidden报告。
