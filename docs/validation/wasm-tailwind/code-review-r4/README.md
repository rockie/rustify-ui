# WASM-TAILWIND R4 评审证据

报告：[WASM-TAILWIND.code-review.r4.md](../../../plan/WASM-TAILWIND.code-review.r4.md)。只保存本轮复审输入、差异、日志与隔离实验；不修改产品代码。

- [输入集合](input-snapshot.json)、[输入核验](input-verification.json)：4,851 项 hash，测试/构建后、台账修改前无漂移。包含大量历史证据，不代表逐行审查全部文件。
- [scope 增量](scope-r3-to-r4.diff)、[测试增量](runtime-tests-r3-to-r4.diff)、[计划增量](plan-r3-to-r4.diff)；R3 旧全文保留原位，本轮全文见 `source-snapshot/`。
- [结果摘要](verification-summary.json)：R3 探针恢复情况与 F-04/F-08 原始 CSS、R3、R4、真实 SDK 对照。
- [native 工程](native-project/Cargo.toml) 中 `old_scope.rs` 是 R3 快照原文（sha256 `7a8bcaf626ac6a89745039d02de0f617917a2a788f8e744bf2588fb91e24a0d4`）；当前版本依赖本仓 style crate。[结果](native-probe.json) 与归档工程的 [锁定重放](native-project-replay.json) 一致。该工程独立锁文件只使用本机已有缓存；它不是主工作区依赖验收，当前 release WASM/SDK 浏览器对照另行确认了相同故障。
- [静态样式浏览器](cascade-native-browser.json)、[root SDK](cascade-sdk-browser.json)、[deep SDK](cascade-sdk-deep-browser.json)。探针 ID `external-disable-cyclic` 实际测试的是 alias 链在根 token 被覆盖后的行为，不是循环引用；真正的循环用例是 `cycle-fallback`。`none-name` 含非法裸 `@keyframes none`，`default-registered` 的 R3/R4 输出不变，二者不作为新增 finding。
- root/deep 独立日志、manifest、单测日志均在本目录。root cascade 探针未监听 CSP；deep cascade 与 R3 animation 探针监听 CSP 且结果为空。

仓根复现（`<临时目录>` 替换为已存在的可写目录，避免将编译产物写进证据树）：

```sh
CARGO_TARGET_DIR=<临时目录>/target mbx run --locked --offline --manifest-path docs/validation/wasm-tailwind/code-review-r4/native-project/Cargo.toml -- docs/validation/wasm-tailwind/code-review-r4/candidate-cases.json > <临时目录>/native-probe.json
node docs/validation/wasm-tailwind/code-review-r4/cascade-probe.mjs <临时目录>/native-probe.json <临时目录>/cascade-browser.json
```

SDK 复现先用本仓 `mbx xtask build-web --example utility-lab --release`，再 `target/debug/xtask serve --example utility-lab --release --port 0`，把实际 URL 传给同一探针第四个参数。deep 构建/serve 都加 `--base /tools/utility-lab/`。报告记录了本轮实际 URL；端口可能随重启改变。
