# WASM-TAILWIND 提交前验证

计划：[WASM-TAILWIND](../../../plan/WASM-TAILWIND.md)。

验证时间：2026-10-09T18:23:56.189614+08:00。基线：dirty@`92ee1862885a372fca9c6fdfa6907b560c556efa`。本目录保存本次实际执行结果，历史里程碑与评审证据保留原位。

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| fmt | 通过 | [fmt.log](fmt.log) |
| workspace-tests | 通过，1080 passed / 0 failed / 0 ignored | [workspace-tests.log](workspace-tests.log) |
| style-all-features | 通过，533 passed / 0 failed / 12 ignored | [style-all-features.log](style-all-features.log) |
| facade-tests | 通过，15 passed / 0 failed / 0 ignored | [facade-tests.log](facade-tests.log) |
| sdk-utility-tests | 通过，301 passed / 0 failed / 0 ignored | [sdk-utility-tests.log](sdk-utility-tests.log) |
| clippy | 通过 | [clippy.log](clippy.log) |
| sources | 通过 | [sources.log](sources.log) |
| css | 通过 | [css.log](css.log) |
| catalog | 通过 | [catalog.log](catalog.log) |
| build-root | 通过 | [build-root.log](build-root.log) |
| build-deep | 通过 | [build-deep.log](build-deep.log) |
| browser-root | 通过，12 项 | [browser-root.log](browser-root.log) |
| browser-deep | 通过，12 项 | [browser-deep.log](browser-deep.log) |

完整命令、退出码及耗时见 [checks.json](checks.json)。本次已重建 root 与 `/tools/utility-lab/` release 产物；主应用 build ID 均为 `79e53adbdde257a3`，compiler-only Worker build ID 均为 `a65f0b9a2a382faf69c4065ce51e7befae11bb38f91607d7cffa7d75ad9101ac`。身份记录见 [root manifest](manifest-root/build-manifest.json)、[deep manifest](manifest-deep/build-manifest.json) 及同目录 Worker manifest。

浏览器冒烟覆盖动态 CSS 安装、GPU 绘制与事件、scope 隔离、配置替换回滚、keyframes/公开 token 和 rebuild；实际步骤见两份 browser 日志，截图保存在对应 `browser-*-results/` 目录。浏览器测试中涉及的 console/page errors 与失败请求按各用例断言校验。

本次没有重跑 scanner 浏览器、完整上游差分、完整旧示例、性能与远端 CI，不把这些历史结果记为本次通过。style 的 12 个 ignored 保留原上游性能测试标记。

暂存源码的差异空白检查通过，保留一项明确例外：上游 `slim.rs` 的 `test_embedded_ruby_percent_w_extraction` 预期字符串第 320/323 行各有尾随空格，代表预处理后的实际输出，不修改这份固定上游 fixture。命令与排除项见 checks.json，输出见 [staged-diff.log](staged-diff.log)。

实现与验证产物已提交为 `c2d48c6b2fa3270d09de759fcaed05e702d2d44b`；[提交记录](commit.json) 保存范围与验证入口，[源码指纹](source-fingerprints.json) 保存本次验证的源码集合。
