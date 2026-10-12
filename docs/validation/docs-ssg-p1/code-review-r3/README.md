# DOCS-SSG-P1 · R3 独立复审证据

报告：[DOCS-SSG-P1.code-review.r3.md](../../../plan/DOCS-SSG-P1.code-review.r3.md)。

- 固定 base：`bfbee6d3342b69e062ed513e05e7f812373124bc`。
- 固定 target：`1f0fc5ed21d7937a1297528ad38cdefe6b95f721`；代码修复 `a297dbd98f39f87910b1ad4100370dd9f630dae1`。
- `scope.json` 保存输入内容 hash；`scope-end.json` 确认输入/HEAD 未变。`commands.json` 保存实际命令、退出状态与两个二进制 hash。日志为本轮实跑，不是前轮结果复制。
- `replay-comparison.json` 是原 R2 的 11 场景两版本对照；`browser-results.json` 是其中 8 个生产浏览器场景，截图含 SVG mask 实际像素。
- `candidate-results.json` 复现 F-13 附件链接删除恢复及新增 F-18 template 非文档锚点；`css-candidate-results.json` 复现 F-16 数字实体跨度及新增 F-19 解码回归，包含分号阳性对照。
- `candidate-fixtures/` 保存本轮合成输入、两版本产物与 dev 日志；所有附件/图像仅为合成测试数据。符号链接使用工程内相对路径。
- `docs-dev-results/`、`docs-subpath-results/` 为对应 Playwright JSON 输出；常规套件共 35 项通过。浏览器是 Playwright Chromium `153.0.8010.12`，不称真机或用户 Chrome 验收。

## 复现方式

不在已保存的证据目录执行脚本，先复制脚本到新建临时工作目录。脚本从仓根执行，依赖当前已安装的 Playwright/pngjs；不会安装服务或浏览器。

1. 核对固定版本；target 用 `mbx build --locked -p rustify-docs-cli --profile docs-cli` 构建。基线源码由 `git archive <base>` 提取到工作目录的 `baseline-source/`，用独立 `CARGO_TARGET_DIR=<work>/baseline-target` 和 `mbx build --offline --locked -p rustify-docs-cli --profile docs-cli` 构建。保留当前用户 checkout，不 reset/stash。
2. 原边界对照：`python3 docs/validation/docs-ssg-p1/code-review-r2/replay.py --baseline <work>/baseline-target/docs-cli/rustify-docs --work <work>/replay`，其中 replay 目录须为空。
3. 复制本目录的 browser-cases.cjs 到 `<work>`，运行 `node <work>/browser-cases.cjs <work>/replay`。它只启动当前 CLI 的 loopback preview，退出时关闭。
4. 复制 candidates.cjs、css-candidates.cjs 到 `<work>`，分别运行 `node <work>/candidates.cjs <work>`、`node <work>/css-candidates.cjs <work>`；它们按以上路径寻找两版二进制，在临时 candidate-fixtures/ 新建输入。不要重复使用已有 fixture 目录。

恢复依赖观测以 revision 为目标，每秒检查、最多十次等下一轮；恢复后固定检查三次，随后保存正文作阳性对照。所有服务/浏览器均在 finally 中关闭，没有开放式监控。

结论：需要修改；192 项 Rust、35 项标准浏览器及 8 个原生产边界断言通过，不覆盖本次新发现的四个问题。本轮未改产品代码或计划，未提交/推送，M9/实体设备/完整性能/远端仍未验证。
