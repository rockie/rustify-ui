# M6 提交前安全检查 · CLI/依赖/发布

| 严重度 | 已确认可利用问题数 |
| --- | --- |
| Critical / High / Medium / Low | 0 |
| Info | 3（既有依赖停止维护；不是漏洞条目） |

范围：本期变更的 CLI new、Rust helper 编译/输入守卫、主题文件/字体打包，以及候选准备和 Pages workflow。Docs 内容/XSS/原读链由独立 security-docs-r1 记录；此处不声称全仓或线上安全验收。产品源码当前 dirty@2d6b0a73ad887769c0bad209acabd47382659df5；真实R/C尚未冻结。

依赖：已安装 cargo-deny 0.20.2 对CLI根344个依赖运行 `cargo-deny --format json --locked --manifest-path crates/rustify-docs-cli/Cargo.toml check advisories --audit-compatible-output`。返回1：vulnerabilities为空，3条unmaintained按默认策略报错，不能写作工具全绿。[原输出](dependency-advisories-r1.jsonl)、[原日志](dependency-advisories-r1.log)。与HEAD Cargo.lock比较，paste1.0.15/unsafe-libyaml0.2.11/bincode1.3.3均既有且未改版。官方记录分别为[paste](https://rustsec.org/advisories/RUSTSEC-2024-0436.html)、[unsafe-libyaml](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)、[bincode](https://rustsec.org/advisories/RUSTSEC-2025-0141.html)，均标INFO Unmaintained且无patched版本；本期不更换不相关解析/高亮依赖。此为维护风险记录，不代表未来无漏洞。

秘密扫描：890个当前变更/新增文本文件按高置信度凭证/private-key/含凭证数据库URL模式扫描，命中0。没有打印原始匹配或环境值。[范围与结果](secret-scan-r1.json)。这不是任意敏感信息识别器。

已追链：

- `new.rs`的package_name与named_lock只生成安全根包名并仅改一个source-free root；生成采用temporary sibling/rename，README命令目录用single-quote转义。TOML/YAML名字段有quote/backslash逃逸；未发现从目录名进入shell执行的路径。
- `rust_site.rs`的cargo metadata/build均通过Command/argv传递manifest/profile/target，编译程序来自作者进程环境；不拼shell。每次locked、抑制rustup自动安装，metadata离线失败仅在作者允许网络时取依赖。author Rust/helper本身是作者可信代码，流程不是执行不可信Rust的沙箱。
- `theme.rs`的document_path核Normal路径、canonical root/private/output约束；读取256KiB+1边界，validate_snapshot比原字节及原canonical绑定。字体/许可证来自编译嵌入字节和摘要，未引入浏览器跨源字体下载。
- Pages输入通过env进入Python/Shell argv，完整40位C与checkout相等；安装同C，完整tracked消费者仅投影GitR的三个依赖行和lock。无push触发、enablement=false、build/deploy分权、同run artifact、retention1、部署串行。当前只是可审配置，不含运行、授权或线上通过；新统一project helper集成后须再核入口和不变式。

未确认可利用漏洞，因此没有建议patch；未自动修改安全审计所查产品代码。冻结前还需完成配对消费者适配和真实C测试；当前未触发GitHub Actions/Pages，也未推送。
