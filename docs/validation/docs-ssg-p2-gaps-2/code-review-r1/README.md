# DOCS-SSG-P2-gaps-2 · 代码评审 R1 证据

[完整报告](../../../plan/DOCS-SSG-P2-gaps-2.code-review.md)。本目录为本轮新增产物，不纳入本轮代码输入。

- `scope.json`、`product.patch`、`initial-status.txt` 固定 base/target、32 个产品/测试/作者参考文件和原有 dirty 输入；439 个文件按路径映射到 SHA-256，集合指纹算法见报告。
- `frontmatter-probe.json` 对照已安装 C′/C″；两者身份均核对。`frontmatter-exits.json` 保存 C″ 的 check JSON、build JSON、check text 原输出与期望，确认 PG-01，dist 原字节保持。
- `content-browser.cjs` / `.json` / `content-dev.stderr` 为实际执行的隔离内容站浏览器探针与结果：四阶段、16 视图、Hide/Enter/Esc、恢复和故障审计。最后一阶段确认缺口，观察断言通过不代表该阶段满足产品合同。脚本从仓根用 Node 执行，读取同目录 `installed-before.json` 的本机已安装入口；该临时入口不保证跨机器存在。
- `.first` 保留首次探针误用“旧错误节点应删除”的收尾断言及其输出；实际既有合同为隐藏面板。修正探针后按 hidden/revision 核对恢复，通过；产品源码未改。
- `browser-table-results.json` 与 `browser-table/` 为仓库 docs-dev 原有手写表用例的本轮真实 Chromium 输出；JSON 中绝对路径是实际运行时的临时路径，附件已按原相对结构复制到本目录。
- 三包原始测试日志、首次 Python 缺少 tomllib 的失败、Python 3.12.13 下的最终 CLI 全量结果、格式检查、配对审计和验证结束指纹均保留；没有把首次环境失败算作产品失败。
- `installed-before.json` / `installed-after.json` 证明同一 C″ 二进制、SHA-256、source 和 panic 模式保持。未在本轮重新执行安装。

产物未提交、未推送；本目录的脚本及原始 JSON 仍保留本机运行路径，另一台机器需重建相同 Git 配对与工具环境后重新验证。
