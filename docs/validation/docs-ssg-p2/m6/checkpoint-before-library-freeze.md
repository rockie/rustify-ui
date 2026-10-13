# M6 · 固定候选与完整出口

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T04:37:22+08:00 · 状态：进行中，冻结前消费者配对适配与安全检查已落盘；真实R/C尚未提交。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5。呈现路径继续静态DOM/明确交互岛。

| 检查 / 覆盖 | 实际结果 | 证据 |
| --- | --- | --- |
| 安全 / 依赖 / 发布源 | Docs十二源码及调用链、CLI/主题/argv/输入守卫/手动Pages配置未确认可利用漏洞；890文本高置信凭证扫描0命中。cargo-deny检查344依赖，漏洞条目0，因3条既有unmaintained返回1，未冒作绿 | [Docs审计](m6/security-docs-r1.md)、[CLI审计与官方advisory链接](m6/security-cli-r1.md)、[扫描](m6/secret-scan-r1.json)、[原依赖输出](m6/dependency-advisories-r1.jsonl) |
| 严格R/C消费者适配 | 原path fixture会被非空expectedR拒绝，生产门保持不变。统一helper从CLI source C的真实Git资产读R，new(C)核三Git依赖和source-free root lock；仅临时消费者2份Cargo文件投影。Native、dev browser/serve、CI/features、B P2/ignored proxy接入；空R保持path/lock，默认B500语义不改 | scripts/docs-pages-prepare.py；相关support/serve/verify/xtask改动，静态AST/纯投影与4拒绝例/shell/YAML/Node/rustfmt/diff已查，真实C尚未验 |
| 当前空R开发回归 | docs_corpus6/6、CLI72通过+1ignored；components纯投影case已绿，完整真实组件集成仍在运行，其后串行Rust dev，未预填完成 | [语料](m6/adapter-corpus-empty-r1.log)、[CLI](m6/adapter-cli-empty-r1.log)、[在途components](m6/adapter-components-empty-r1.log)。Root exec session3786仍活跃，由Root独占Cargo/build/browser/ports；不重启该在途组 |
| 实际桌面供给 | 原生Chrome155.0.8059.40/Safari27.0.1已从应用清单读到。Safari原生MCP initialize/tools-list通过，并创建/关闭唯一自有about:blank tab；未读现有用户tabs/内容、未改设置。仅供给证明，不计UI通过 | [本机清单](m6/environment-r1.json)、[MCP能力](m6/safari-mcp-capabilities-r1.json)、[自有空白页](m6/safari-native-availability-r1.json) |

候选准备保留真实Git C的detached worktree历史；仅官方site/Cargo.toml三依赖行及配对lock不同，其余tracked源逐文件核hash。Pages workflow只workflow_dispatch完整C、Mac固定Git安装/同run完整artifact和依赖deploy；enablement=false，无push自动部署。尚未执行Pages/GitHub外部写。

本机已存在Python3.12.13（/opt/homebrew/bin/python3.12）；默认python3为3.9.6。仅后续验证进程PATH优先既有3.12，CI显式setup-python3.12，未安装工具或修改系统。公共CLI作者不依赖该仓库测试工具。

未完成：等待当前session3786的组件/Rust dev回归；检查新适配严格clippy与完整输入不变式；按计划先提交库R，再仅以CLI发布身份/版本/模板锁/文档形成C，并用真实固定C禁网配对检查/未改模板/官方和B消费者验证。随后同候选V-5/V-9连贯UAT、V-10/V-11/V-12/V-14集中出口与最大页五次正式测量。未有公开候选或推送/发布授权；用户无法安排5位首次用户及实体手机，继续保留M6未验收缺口。Firefox按已确认范围留P4。
