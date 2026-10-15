# DOCS-SSG-P2 代码评审 R2 · 复审证据

报告：[DOCS-SSG-P2 代码评审 R2](../../../plan/DOCS-SSG-P2.code-review.r2.md)。观察时间 2026-10-14T22:16+08:00，checkout `main` / `d2f20b03cf58c1d1ced3785397c501d412cd4167` / dirty（下方指纹）。本目录只保存复审时实际运行的结果，不修改 R1 证据或 apply 记录。

| 文件 | 命令 / 来源 | 结果 |
| --- | --- | --- |
| [features.log](features.log) | `PATH=<python3.12>:$PATH mbx xtask docs-verify --group features`，在 HEAD 重建 `target/dom/hydration`（ssr/hydrate/csr 三图分开）及组件站 | exit 0；默认 DOM、GPU 双向开关、docs 两图隔离、组件站 1 组资源 |
| [dom-hydrate.log](dom-hydrate.log) | `npx playwright test -c playwright.docs.config.ts --project=dom-hydrate hydration.spec.ts --workers=1` | 12 passed，0 skipped/flaky，13.1 s |
| [branch-first-cleaned-first.json](branch-first-cleaned-first.json)、[branch-second-cleaned-first.json](branch-second-cleaned-first.json) | 上面两条 F-01 回归的 `branch-geometry.json` 附件 | 两分支同时打开时第一条仍 128 px（R1 为 256 px），第二条 64 px；两种清理顺序 marker 不变，另一挂载根独立；dispose 后 marker 移除 |
| [cli-unit.log](cli-unit.log) | `mbx test --locked -p rustify-docs-cli --bin rustify-docs` | 74 passed，0 failed，1 ignored（原配置性能代理）；含两条新增缓存回归 |
| [dev-deps.log](dev-deps.log)、[dev-dependencies-summary.json](dev-dependencies-summary.json) | `npx playwright test --config=playwright.docs.config.ts --project=docs-dev tests/docs-ssg/dev-components.spec.ts --grep 'hydrate-only dependency manifests\|renamed same-name build inputs' --workers=1`；`RUSTIFY_DOCS_BIN` 为已安装 C5，`CARGO_NET_OFFLINE=true`，PATH 前置 Python 3.12 | 2 passed，3.2 min，0 重试；浏览器失败请求/console/pageerror/CSP 均 0 |
| [worktree-tracked-dirty.sha256](worktree-tracked-dirty.sha256)、[worktree-untracked.sha256](worktree-untracked.sha256) | 复审开始时 `git diff --name-only HEAD` 与 `git ls-files --others --exclude-standard` 的逐文件 SHA-256 | 18 / 144 文件；验证结束后重算一致，只多出本目录文件 |

已安装 CLI：`/tmp/rustify-apply-r1-installed.SgjSel/bin/rustify-docs`，SHA-256 `5589466a132c616095dc67d99bc91b24b3ac3184a06f3b982a78880ebcdbf4fb`，`--build-info` 为 0.1.15 / panic unwind / source `45878ce31763fe21b9276e563e2920d180787dee`，与 [apply 记录](../apply-r1/dev-pair.json)一致。dev 回归的作者工程用路径依赖指向本 checkout 的三个框架包，因此同时覆盖 HEAD 库源码与 C5 CLI。

没有运行完整 `docs-dev` 文件、原生 `components`/`rust_dev` 集成测试、官方站浏览器矩阵或性能测量，这些仍以 apply 记录为准。没有安装工具或服务，没有提交、推送或发布。本轮自启的 Playwright、dev 与编译进程都已退出。
