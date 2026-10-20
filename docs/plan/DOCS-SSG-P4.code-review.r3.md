# DOCS-SSG-P4 · 代码评审 R3（F-01 修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R3 · 修复效果复审 | [DOCS-SSG-P4 v2](DOCS-SSG-P4.md) | 修复提交 `36e5f7e357a733b838455aee480a39895008ef12`，即 `b807af2672ac434f5039f8d727a912a6332e2f43..36e5f7e357a733b838455aee480a39895008ef12`；当前核查版本 `7cf9cef36679a8294df11b0b80e601e677476dc4`，工作树指纹见下文 | [R2](DOCS-SSG-P4.code-review.r2.md) |

- 评审 ID：`459cfd1e-c07b-491f-9798-9bedd11b6296`；关联 R2 评审 `b7178384-b6a0-4f39-9800-588a613f076a`。
- 日期：2026-10-20T18:05:44+08:00；仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`；观察 checkout：`main / 7cf9cef36679a8294df11b0b80e601e677476dc4 / dirty`。
- 原输入：`$review-code docs/plan/DOCS-SSG-P4.md`。用户随后明确选择「只复审本计划的 F-01 修复（推荐）」，因此不对另一计划 P4-gaps 作完整审查。
- 上轮基线：R2 审查 `06be2ee3f0db519a7316b0d53d57c94c7ee46fd1..4894d4ccc7c462a344489748553615a352031192`。R2 报告 SHA-256：`5274b8e8a93c9a1d4ca34ce8393dfa9927c9a8f554c65938cc6aa8cd159ee62c`。本轮使用当前完整原文，其版本与 [M7 处置记录](DOCS-SSG-P4.records/M7.md#评审修复)一致。
- 计划版本：v2，SHA-256 `4c756a67ec2835df8a7a86698223f7c7b8ded96f8865b665d900fb5a5a6c754a`。
- 呈现路径：本次仅 CLI 与 Markdown 模板；所服务的文档站采用 DOM，本次没有 DOM/GPU 界面变更。
- 主 agent 独立完成，采用 review-code 和 rust 技能；没有委派 reviewer。代码、原计划和历史报告保持只读。

## 上轮问题复审

| 来源报告 / 编号 | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R2 / F-01（P3） | CLI 帮助和内容模板 README 把 0.1.3 当作回滚目标；README 版本写死 | [M7 处置](DOCS-SSG-P4.records/M7.md#评审修复)，修复 `36e5f7e3`，计划登记 `7cf9cef3` | **已解决（源码范围）** | `main.rs:35–36` 将目标统一为 C2/0.1.32；`new.rs:149–158` 填充 `rollback-version`；内容模板 `README.md:22–25` 使用该占位及共用命令。当前构建的 `--help` 与新建 README 实测一致；定向测试 12/12 通过 | 随后续 CLI 配对发布，届时同步发布说明；本轮不把旧二进制记为已修复 |
| R2 / 待核实 1 | 原生 V-11 本地产物与 C1 的 CI 岛资源并非相同字节 | 历史证据限制 | **本轮未复核** | 用户限定 F-01；修复没有改动岛、站点渲染或浏览器资源 | 保留 R2 原限制，不由本轮关闭 |
| R2 / 待核实 2 | R2 没有重跑全量、性能、原生浏览器、VoiceOver 和线上回滚 | 历史验证范围 | **本轮未复核** | 本轮只执行下列 CLI 定向验证 | 不扩大为整期重验 |
| R2 / 待核实 3 | R2 临时证据只在原机器 scratchpad | 历史取证限制 | **本轮未复核** | 没有访问或迁移 R2 的临时目录 | 历史限制保留 |
| R2 / 待核实 4 | R2 清理过一条已失效 worktree 登记，另有环境残留 | 与本次修复无关的既有环境事项 | **本轮未复核** | 本轮没有启动该服务脚本或修改 Git worktree 登记 | 不纳入 F-01 结论 |

回滚目标由 R2 建议的 C0/0.1.29 调整为 C2/0.1.32，有实际版本依据：修复之前 P4-gaps 已交付 C2，公开安装收据记录其 source 为 `88566ba1ed08223a5834f794840fff5e48f4ba8a`。本轮读取该提交的 `crates/rustify-docs-cli/Cargo.toml`，版本确为 0.1.32；同时核对了已安装 C2 的 `--build-info`，并只读取得[线上 manifest](https://rockie.github.io/rustify-ui/rustify-docs-manifest.json)，HTTP 200、`cli_version=0.1.32`、`build_id=f74f4a4ece90f08a`，与 [M3 发布记录](DOCS-SSG-P4-gaps.records/M3.md)一致。带修复的后续发行版回滚至 C2 是合理处置，不要求继续使用较早的 C0。

## 确认问题（本轮新增）

未发现可确认的问题。F-01 已解决；没有新增 P0–P3 finding。

反证检查覆盖：安装与回滚函数仍保留完整 SHA、`--locked`、`--profile docs-cli` 和覆盖安装所需的 `--force`；新增占位在插入用户目录名之前展开，不会把目录名中的占位样式误作模板。`SOURCE_REVISION` 为空时，安装版本与安装 SHA 都回退到同一组更新后的常量（`main.rs:57–69`）；没有出现只改展示版本而保留旧 SHA 的情况。组件模板走独立的 `fill_components()`，本次不改变其来源配对。

## 待核实/待决策项

1. **尚未发布本次修复。** 本轮实际运行的已发布 C2 二进制仍输出 0.1.3，当前源码构建输出 0.1.32。两者的 `--build-info` 分别为 source C2 和 source `7cf9cef3`。这是 [M7 未完项](DOCS-SSG-P4.records/M7.md#评审修复后的未完项)明确保留的交付边界，当前作者参考仍针对已发布 C2，未错误宣称它已有新提示。本轮代码通过不表示该修复已送达公开安装用户。
2. **归档构建只做静态核查。** 本轮未另行构建缺少 Git 元数据的完整归档；对应安装回退分支的常量和调用链已读取，上述单元/集成验证针对当前 checkout 构建。
3. **范围外证据不关闭。** R2 的四条待核实/限制项、P4-gaps 的独立交付、完整 UI/PRD 验收均不在用户选定范围。本轮没有在浏览器中验证，也没有重跑正式性能、公开安装、Pages 发布或回滚。

F-01 源码修复范围没有阻止裁定的证据缺口。

## 覆盖与验证

### 范围与版本

变更意图：让后续 CLI 构建的帮助和新建内容站 README，共用已发布回滚版本及其完整安装命令。

R2 目标至当前 HEAD 共有七个提交。按用户限定，实际 diff 仅审查本计划登记的 F-01 修复；没有把整个区间的产品改动混入本轮结论：

| 提交 | 本轮处理 |
| --- | --- |
| `53087eb716218aa65b15b24dff529e1d0d9d0823` · 指南描述与范围说明 | P4-gaps 交付，排除；只读取与回滚契约有关的文档上下文 |
| `88566ba1ed08223a5834f794840fff5e48f4ba8a` · CLI 0.1.32 配对 | P4-gaps 交付，排除；核对其版本与公开安装身份 |
| `b28dcafd6e06c0f3556ff4595c17583c640f5628` · 本地验证登记 | P4-gaps 记录，排除 |
| `c6d49c97c6737968cf59a16cbbc3453bea012b70` · 公开安装说明 | P4-gaps 交付，排除；读取当前发布/回滚说明作为上下文 |
| `b807af2672ac434f5039f8d727a912a6332e2f43` · 公开交付登记 | 修复的父提交；仅作基线与 C2 发布证据来源 |
| `36e5f7e357a733b838455aee480a39895008ef12` · F-01 修复 | 纳入：4 个代码/模板/测试文件与 M7 处置记录 |
| `7cf9cef36679a8294df11b0b80e601e677476dc4` · 原计划回填 | 进度回写；读取、不作代码 finding 对象 |

实际逐行审读完整修复 diff，并阅读 [`main.rs`](../../crates/rustify-docs-cli/src/main.rs) 的帮助/安装/归档回退链、[`new.rs`](../../crates/rustify-docs-cli/src/new.rs) 的完整生成/模板填充链、[内容 README 模板](../../crates/rustify-docs-cli/templates/content/README.md)、[`new_build.rs`](../../crates/rustify-docs-cli/tests/new_build.rs)、测试辅助、CLI build script、包配置及当前作者参考。修复提交至当前 HEAD 之间，这四个文件没有额外修改。

工作树初始有 875 个 modified/untracked 文件，全部位于 `.xgent-ai/{experience,sdlc}` 与 `docs/plan/`；没有未提交产品代码，index 为空。工作树指纹为 `6626be49c30affa6b244bf482e9eacf85d07e065ade4aef88cd1a0634cf68ae2`：将 `git ls-files --modified --others --exclude-standard -z` 去重后的路径映射到文件字节 SHA-256，按路径排序，以 UTF-8、`ensure_ascii=False`、紧凑 JSON 序列化后再次 SHA-256。tracked patch 指纹为 `f2ebe82b381b56def65b78ac784dd5790e54dc270c08d58a2211270a0dc00eb7`；index diff 指纹为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。这些预存报告、截图和台账不作为代码 finding 对象，也未逐个审查内容。

写报告前重新比对 HEAD、计划、R2 报告、相关源码/上下游与全部初始 dirty 文件的字节指纹，均未变化。本轮新报告和 SDLC 回写不纳入本轮输入。

### 本轮实际执行

当前目标 `7cf9cef3` 的构建使用 Rust 1.99.0；实际 PATH 上 `mbx --version` 为 1.24.0（`mise.toml` 声明的是 1.15.0，本轮未安装或改动工具链）。

| 命令/步骤 | 版本与实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-docs-cli --test new_build` | 当前目标，7 passed、0 failed、0 ignored；包括完整回滚版本/命令、README 无占位残留、新站点离线构建与失败保护 |
| `mbx test --locked -p rustify-docs-cli --bin rustify-docs new::` | 当前目标，5 passed、0 failed；目录名与模板、包名、锁重命名及完整 revision 检查 |
| `cargo fmt --all -- --check` | exit 0 |
| `mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings` | exit 0 |
| `git diff --no-ext-diff --no-textconv --check b807af26 36e5f7e3 --` | exit 0 |
| 当前 `target/debug/rustify-docs --build-info`、`--help`，仓外临时目录执行 `new my-docs` | source 为完整 `7cf9cef3`；回滚标题为 0.1.32，命令为完整 C2 SHA，README 重复同一标题版本与同一命令，无 `{{` 残留；new exit 0 |
| 已发布 C2：`.rustify-docs/p4-gaps/public/install/bin/rustify-docs --build-info`、`--help`，另一个仓外临时目录执行 `new my-docs` | source 确为完整 C2；标题和生成 README 仍为 0.1.3，命令指向 `42757c62…`。这是原问题的独立输出复现，不能满足当前 0.1.32 断言；未声称在 C2 上重跑整份新测试 |
| Python `urllib.request.urlopen` 只读 GET 线上 manifest | HTTP 200，CLI 0.1.32、build `f74f4a4ece90f08a`；仅证实当前线上身份，不代表重新验证全部页面 |

测试日志有 mbx 缓存绕过/环境变化提示，但上述命令均以 0 退出。没有执行安装命令、提交、推送或发布。

临时对照输出、生成 README 与初始指纹保存在本机 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p4-code-r3-71_yfhwa/`，不作为跨机器可用证据链接；必要输出、命令、版本和判断已在本报告保存。另一次 `new` 的临时项目执行后自动清理。

## 计划符合性

| 本次承诺 | 实现证据 | 本轮判定 |
| --- | --- | --- |
| M7 / R2 F-01：统一工具内回滚版本、SHA 和新建 README | `main.rs:35–36,73–75`、`new.rs:149–158`、内容模板 `README.md:22–25`；本轮输出对照与 12 项测试 | 已核实，源码修复完成 |
| 回滚指向已交付版本，避免继续引导作者安装 P1 | C2 的 Cargo 版本、公开安装收据、现存 C2 build-info 与线上 manifest | 已核实；采用 C2/0.1.32 有依据 |
| 后续固定 C 配对及公开发布 | 计划 §3.2 与 M7「评审修复后的未完项」明确保留 | 后续范围，本轮不声称已完成 |
| 其余 M1–M7、P4-gaps 与 PRD/UI 验收 | 用户明确只选 F-01 | 本轮未复核 |

## 总体结论与下一步

**验收判定：通过验收（仅 F-01 源码修复及其直接回归）。**

修复效果：**已解决**，选定旧问题 1/1 已关闭；修复引入的新问题 0。正确性结论：`patch is correct`，仅覆盖 `b807af26..36e5f7e3` 的修复及当前目标中对应代码，不替代 R2 的完整范围或 P4-gaps 的独立评审。

本轮无需 apply。已发布 C1/C2 的旧提示保持事实原样，修复需随新的 CLI 配对到达用户；本结论不是已发布、已合并或整期 PRD/UI 验收结论。

F-01 代码验收完成。若要继续整期 UI 的独立验收，可选命令为 `$review-ui docs/plan/DOCS-SSG-P4.md`；若要重新核对 PRD 覆盖，可选 `$review-prd-dev-gaps docs/plan/DOCS-SSG-P4.md`。这两项均不由本次局部通过自动启动。
