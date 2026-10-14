# DOCS-SSG-P2 · R1 修复证据

来源：[代码评审 R1](../../../plan/DOCS-SSG-P2.code-review.md)，sha256 `473e6a95e16a1e1c35743d17948bf2d4eaf9aff7dad527a07b092a78fa35adfb`。起点 `53d5c48563420c9252cac3f9d1e14647494320a3`；原报告未改。

- F-01：[M1 处置](../M1.md)，根 Owner 共享 geometry token、collector 独立样式表；独立 CSR 红2/2，完整 DOM CSR/hydrate 绿12/12。
- F-02/F-03：[M4 处置](../M4.md)，SSR/host 与 hydrate/wasm 图隔离取输入；Cargo package_id/out_dir 归属 build 输入。两项单测先红后绿；新C5真实 dev 全组4/4，含清单保存、两包分别保存/删除/恢复、last-good、Markdown零Cargo与锁不变。
- 新配对与受影响终验：[M6](../M6.md)，库R5 `7930fd9a978563e76468fef69d14f444159c47b5`、CLI C5 `45878ce31763fe21b9276e563e2920d180787dee` / 0.1.15；[安装身份](installed-identity.json)、[锁与图](pair-preparation.json)。真实Git安装只用进程内本地传输重映射，不代表公共Git可获取。

| 验证 | 本轮结果 / 原始证据 |
| --- | --- |
| CLI新配对 | 74 unit passed、1原配置ignored；严格all-target clippy通过：[unit](paired-cli-unit.log)、[lint](paired-cli-clippy.log) |
| SDK/组件隔离 | 默认/GPU/SSR六组单元及宿主严格lint通过，两包hydrate wasm check、根/fixture fmt通过：[完整命令](f01-checks.log)。额外wasm严格lint被9项既有SDK问题阻塞，组件no-deps通过；未越界修复，不称整个wasm lint绿色 |
| 浏览器 | DOM CSR/hydrate 12/12：[日志](f01-green-browser.log)；dev 4/4：[命令/细项](dev-validation.md)。已查看SSR目录、feature新颜色及删除输入last-good截图；console/pageerror/CSP/失败请求均0 |
| 原生真实生成/恢复 | components2/2、rust_dev3/3：[日志](native-integration.log)，没有并行运行Cargo或浏览器 |
| 编译完成→实际输入绘制 | 同24类别meter，五次中位1163.4ms≤2000、位移0/无shift/错误0：[摘要](performance-summary.json)、[全样本](dev-performance/dev-complete-performance.json)。历史C4同脚本1117ms，非同时受控A/B；本轮源码保存全程26.16–26.55s，不能混写成保存全程≤2s |

无效旧CLI脚本setup尝试另见 [说明](dev-setup-attempts.md)，不充作产品红证据；有效修前依据是原R1浏览器样本与本轮两个定向红单测。原本人验收属于C4；C4完整官方/旧GPU浏览器矩阵、实体设备、最新稳定浏览器与公开安装/Pages没有在本轮重跑。计划仍5/7，不把三项修复当作整期完成；待用户发起独立复审。

## 提交前局部安全自查

| 新增已确认风险 | Critical | High | Medium | Low |
| --- | --- | --- | --- | --- |
| 本轮差异 | 0 | 0 | 0 | 0 |

仅检查两处Rust修复及其输入/生命周期边界、R/C身份差异：未新增shell拼接、unsafe、inline样式/CSP豁免、凭证、网络服务或授权路径。框架Git source移至R5，三方包版本/校验和未变；未重查最新advisory数据库，不冒称全仓安全审计。代码/Markdown whitespace检查通过；原始工具日志的尾空白按证据原样保留。原有八份已修改fixture日志已逐字节恢复开工内容，既有未提交报告与台账未代用户提交。
