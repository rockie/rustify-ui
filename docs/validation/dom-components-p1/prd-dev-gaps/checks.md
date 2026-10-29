# P1 PRD 覆盖核查 · 本次验证

日期：2026-10-29；源码/输入/产物身份见 [baseline.json](baseline.json)。本目录为本次独立证据，不替换 M1–M7 或 UI R8。Chrome 156.0.8078.48 headless、Node 26.12.1、Playwright 1.63.0。命令从仓根运行；Cargo/xtask 均经 `mise exec -- mbx`。

## Host 与生成物

| 实际命令（均在 `mise exec --` 后） | 结果 |
| --- | --- |
| `mbx test --locked -p rustify-dom --lib` | exit 0；258 passed |
| `mbx test --locked -p theme-studio --bins` | exit 0；69 passed |
| `mbx test --locked -p rustify-components --no-default-features --features ssr --lib` | exit 0；169 passed |
| `mbx test --locked -p rustify-colors --features p3` | exit 0；3 unit + 1 doc-test passed |
| `mbx xtask colors --check` | exit 0；256 deterministic files，31 色族/两模式/alpha/色域/黑白 |
| `mbx xtask sources verify` | exit 0；Colors 5、Themes tokens 39 输入无修改；既有 Makepad/Rust/UI 归因 drift 保留 |
| `mbx xtask css --check` | exit 0；70,314 bytes、4,475 SDK candidates |
| `mbx xtask catalog --write docs/components.md --check` | exit 0；59 public entries / 97 stable reference names / 24 legacy categories |

原 host 日志为本机 `/tmp/dom-components-p1-prd-gaps-{model,components,colors}.log`；DOM 单测结果由命令回显直接读取，本页保存实际汇总。临时路径不作为跨机可取回承诺，不把历史日志当作本次输出。

## 现有真实浏览器 spec

共同环境：`RUSTIFY_XTASK=target/debug/xtask`、`RUSTIFY_CHROMIUM='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'`、`RUSTIFY_UI_EVIDENCE_OUT=/tmp/dom-components-p1-prd-gaps`。`mise exec -- npx --no-install playwright test` 每次只运行一个 project，串行执行；均使用已有同源码 release，无 Rust/CSS 改动。

| 实际参数 | 结果 / 日志 |
| --- | --- |
| `--project=theme-studio theme-config.spec.ts theme-export-ui.spec.ts theme-exchange.spec.ts theme-storage.spec.ts --workers=1 --output=/tmp/dom-components-p1-prd-gaps/playwright-studio` | exit 1；22 passed / 3 failed，2.2m；[原输出](studio.log) |
| `--project=component-catalog dom-buttons.spec.ts dom-layout.spec.ts theme-compat.spec.ts --workers=1 --output=/tmp/dom-components-p1-prd-gaps/playwright-catalog` | exit 0；18 passed，33.6s；[原输出](catalog.log) |
| `--project=theme-studio theme-v2-storage.spec.ts --grep 'cancelled legacy upgrade\|upgrade review is bound\|one writer preserves\|quota failure retains\|third write\|without Web Locks' --workers=1 --output=/tmp/dom-components-p1-prd-gaps/playwright-storage` | exit 0；9 passed，33.1s；[原输出](storage.log) |
| `-c playwright.docs.config.ts --project=colors-static --workers=1 --output=/tmp/dom-components-p1-prd-gaps/playwright-colors` | exit 0；2 passed，11.6s；[原输出](colors-browser.log) |

三条失败精确定位在 `theme-storage.spec.ts:86/104/118`：旧断言分别在 `storage-status` 查找 quota 词、命名保存失败句和拒绝存储词。当前段落分别表达内存草稿、已保存草稿与可内存编辑；失败原因/操作结果已在同页的标题和命名反馈中。原测试到此停止，其余数据断言不能冒称执行；以下独立行为验证补充事实，不把原失败改绿。专项对应报告 V-01。

`theme-compat` 会将截图写入既有 `source-boundaries.png`。开工时该文件无改动，本次运行后已恢复原 HEAD 字节；既有两张 dirty 布局图保留。其它新截图已导向 `/tmp` 或本目录。

## 本次导出与存储专项

用 `target/debug/xtask serve --example theme-studio --release --port 4181` 的 strict CSP 页面；Playwright 启动上述真实 Chrome。每组使用全新隔离 browser context，不使用个人 Chrome profile；页面动作通过实际控件，应用 handle 只读取状态。步骤与结果保存在 [browser-probe.json](browser-probe.json)。

1. 正常入口 → Export 默认 JSON：实际 `theme.json` 为 schema 2；对话框可见文本无版本/SDK/CLI/字体接入提示。已查看 [默认 JSON 原图](default-json-export.png)。
2. 切到 Rust integration：屏内提示仅给 mode/context 两参，输出代码的 `mount_exported_preview` 签名为 container/config/mode/context 四参；源码修订 header 为当前 HEAD。[原图](rust-export-guidance.png)。
3. 实际编辑 primary 为 `#552288` 并等存储稳定 → 让隔离 context 的 `Storage.setItem` 抛 quota → 编辑为 `#882255`：旧存储 bytes 不变、队列暂停、内存和实际导出保留新值；常驻标题为 Storage full。
4. 新 context 正常保存草稿后注入 quota → 命名保存 Retained name：实际独立反馈为 failed/retained，旧 bytes 不变；恢复原 setItem → 明确点击该名称的 Retry → 库内实际出现该主题。
5. 新 context 让 localStorage getter 抛 SecurityError：页面常驻显示 local saving unavailable；实际编辑并导出的 JSON 保留 `#552288`。

五组均完成；pageerror、console error、失败请求和 CSP 事件为 0。测试注入仅发生在临时 context，浏览器与本次 4181 服务已停止。未做迁移、部署、发布、外部业务写入或产品修复。

这不是性能运行、全量 UI 评分或完整兼容矩阵。独立消费者重编译、source CLI、GPU/fatal、VoiceOver、OS IME、Safari/Firefox、真机与真人研究没有在本次重跑；仅按报告明示范围引用历史证据。


## 产物自检

- P1 覆盖表与 PRD 实际逐行核对：30 条、无重复/漏行；28 已满足、2 部分满足。82 条总 AC 中其余 52 明确后期去向。
- 新计划结构检查 ERROR 0 / WARN 4；四告警为沿用的 WORK-F/N 与 AC 自定义组合编号，已逐条核对上游来源。进度检查 ERROR 0 / WARN 1，0/2、空完成记录正确；告警是仓内已有 dirty 材料，不代表本新计划有已完成实施。
- 路径检查的 MISSING 1 为明确标 ★、按需创建的本计划 M1 记录，不是缺失复用文件。完整实施协议与所读骨架逐字匹配；实际 Markdown 链接排除代码块内的协议示例链接后核对。
- 原 PRD、原计划、M1–M7、旧评审、主题指南输入 hash 与 baseline.json 一致，HEAD 未移动，产品源码无 diff；本次未实施新计划、未提交/推送。
