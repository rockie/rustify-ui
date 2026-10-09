# WASM-TAILWIND R5 复审证据

报告：[R5](../../../plan/WASM-TAILWIND.code-review.r5.md)。本目录只保存评审快照、日志及隔离实验，没有修改生产代码。

- `input-snapshot.json`：R4 输入集合，加本轮开始时的 dirty/untracked 与上下文文件；按 path 排序，对 files 数组 canonical JSON（ensure_ascii=False, sort_keys=True, separators=(comma,colon)）做 SHA-256。4,910 项是来源集合，不等于逐行覆盖；`input-verification.json` 确认写报告/台账前无漂移。
- `scope.rs.diff`、`utility-runtime.spec.ts.diff`、`WASM-TAILWIND.md.diff`：R4 原位全文到本轮工作树的精确增量。`source-snapshot/` 保存本轮关键源码、测试和契约全文；未复制全部输入，其他依赖仍须按 manifest 对工作树核对。
- `native-project/src/old_scope.rs` 是 R4 原始 scope 全文（sha256 d13fd89a791e24da67947a0e3d8a2f5d0e3497ad0e74e2edd7a4816c222eb14e），新版本依赖本仓 style crate。`native-probe.json` 中 before_r4 / after_r5 有明确版本标签；5 个冲突公开变量输入全部由接受变为 UnsupportedFeature，3 个支持输入 CSS 完全相同。工程复用了 R4 的独立 Cargo.lock，仅用于隔离对照，不替代主工作区依赖验证。
- `style-tests.log`、`facade-tests.log`、`clippy.log`、`fmt.log`、`build-root.log`、`build-deep.log` 为本轮实际结果。
- `browser-root.log`、`browser-deep.log`：root/deep 各 8 项 scope、配置和 rebuild 回归。line reporter 未把通过用例的附件落为独立文件；行为细节由 `sdk-root.json`、`sdk-deep.json` 补充。
- `sdk-probe.mjs`：通过实际 release SDK/Worker 验证公开 token、内联动画、alias 的 none 覆盖，5 个旧反例替换失败后保留 CSS/revision/活动动画；记录 page/console、失败请求、HTTP >=400 和 CSP，两个入口均为空。仅启动和停止自己的 port 0 serve。
- `manifest-root.json`、`manifest-deep.json` 固定 release 身份；`verification-summary.json` 仅汇总已执行结果。

从仓根重放（先核对关键源码与输入指纹；native target 使用隔离临时目录）：

```sh
CARGO_TARGET_DIR=<临时目录>/target mbx run --locked --offline --manifest-path docs/validation/wasm-tailwind/code-review-r5/native-project/Cargo.toml -- docs/validation/wasm-tailwind/code-review-r5/candidate-cases.json > <临时目录>/native-probe.json
mbx xtask build-web --example utility-lab --release
mbx xtask build-web --example utility-lab --release --base /tools/utility-lab/
node docs/validation/wasm-tailwind/code-review-r5/sdk-probe.mjs <已有可写临时目录>
```

SDK 探针最后一条命令要求该临时目录中有本目录的 `candidate-cases.json`，会把 SDK JSON 与 serve 日志写到该目录。浏览器项目的完整串行命令见报告；native 构建日志中的路径是本轮临时工程位置。没有安装或下载基础设施、浏览器或新依赖。
