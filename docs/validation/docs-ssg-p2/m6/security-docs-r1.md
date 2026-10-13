| Severity | 已确认发现 |
| --- | ---: |
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 0 |
| INFO | 0 |
| Total | 0 |

# DOCS-SSG-P2 M6 · Docs 提交前安全审计

本次只读审计在已查链路中未确认可利用漏洞（No vulnerabilities found within the audited flows）。没有产品代码改动或待应用安全补丁。结论限下列源码与调用路径，不代表功能验证、完整代码评审或全仓安全通过。

快照时间：2026-10-14 04:24:31 +08:00；HEAD 为 `2d6b0a73ad887769c0bad209acabd47382659df5`，审计的是未冻结工作树，文件指纹见文末。语言为 Rust 与浏览器 JavaScript，渲染链路为 Leptos SSR / DOM hydration 和静态 Docs 生成。

## 范围与方法

主范围为 M3–M5 变更中的十二个文件：`crates/rustify-docs/src/{config.rs,content.rs,generation.rs,html_check.rs,markdown.rs,shell.rs,style.rs,site.rs,site/registry.rs,site/ssr.rs,site/hydrate.rs}` 与 `crates/rustify-docs/assets/dev.js`。读取相应 diff 和生产代码，围绕 README 桥、白名单、私有文件、路径/资源与 HTML/XSS 边界沿调用核对。另只读 [escape.rs](../../../../crates/rustify-docs/src/escape.rs#L6) 的直接转义实现作为调用依据；未扩大为全仓扫描或初次 review-code。

使用 [security-audit](../../../../.agents/skills/security-audit/SKILL.md) 的 Rust / JavaScript、XSS、数据暴露、路径与竞态参考；复核疑点后才计入发现。未运行 Cargo、构建、浏览器、端口服务、网络攻击或改动作者输入；依赖/CVE、CLI、发布与分发由 Root 的独立检查覆盖。

## 已追踪的安全边界

| 输入 → 输出路径 | 实际核对与出处 |
| --- | --- |
| 配置 / content.include → 源文件读取 | [relative_path](../../../../crates/rustify-docs/src/config.rs#L252) 拒绝绝对路径和越过项目根的 `..`；[include 解析](../../../../crates/rustify-docs/src/config.rs#L391) 要求字面 `.md` 文件、拒绝 glob、私有路径与重复项。[snapshot](../../../../crates/rustify-docs/src/content.rs#L481) 在选择模式下只读取列出的内容文件，而不是先扫描全部正文再过滤。 |
| include / 资源别名 → 真实文件字节 | [is_private](../../../../crates/rustify-docs/src/content.rs#L166) 先检查名字；[real_source](../../../../crates/rustify-docs/src/content.rs#L288) canonicalize 后用 Path 的 strip_prefix 限制在真实项目根内，并检查真实目标的私有路径；[read_source](../../../../crates/rustify-docs/src/content.rs#L354) 从这个目标读取字节。watched_source 的缺失路径推导只用于依赖记录，不授权读取。 |
| 原读 / Exists → 代际一致性守卫 | [InputReceipts](../../../../crates/rustify-docs/src/content.rs#L206) 保留最初绑定、字节与目录身份；[Sources::exists](../../../../crates/rustify-docs/src/content.rs#L1026) 记录存在性而不读取正文；[validate_inputs](../../../../crates/rustify-docs/src/content.rs#L628) 检查快照、绑定、文件类型和原字节，变化返回 render-generation-changed。此处核对守卫实现，不替 Root 验证 CLI 每个发布入口的调用时机。 |
| RenderContext::markdown_intro → README HTML | [registry](../../../../crates/rustify-docs/src/site/registry.rs#L227) 与 [SSR surface](../../../../crates/rustify-docs/src/site/ssr.rs#L284) 记录路径 / placeholder；[markdown_intro](../../../../crates/rustify-docs/src/content.rs#L1975) 再做项目相对路径、真实源读取、入口 theme 检查和 H1→首个 H2 的原段范围，复用同一 Markdown writer。返回输入 receipts / referenced，不把 helper 路径当作绕过读取边界的许可。 |
| README 嵌套 include → code / raw-copy template | [include](../../../../crates/rustify-docs/src/markdown.rs#L1223) 复用已读取的 IncludeFiles；[code_block](../../../../crates/rustify-docs/src/markdown.rs#L1284) 转义 label、language 与 raw source，并将 CR 写为字符引用。源内容中的闭合标签不会变成 raw-copy template 的结构。 |
| 排除的 Markdown / 仓库链接 → href | [repository_link](../../../../crates/rustify-docs/src/content.rs#L2515) 通过 exists 验证目标，不因普通仓库链接读取排除文件正文；[resolve_source_link](../../../../crates/rustify-docs/src/content.rs#L2101) 以原 README 所在目录解释路径、分离 query / fragment，再解析为站内页或仓库链接。白名单是页面选择范围，不是对显式 include / 资源引用的通用保密 ACL。 |
| HTML / CSS 类中的 URL → 公开资源图 | [resolve_resource](../../../../crates/rustify-docs/src/content.rs#L2563)、[Sources::get](../../../../crates/rustify-docs/src/content.rs#L992) 检查实际路径、私有文件与 theme 输入；[source candidates](../../../../crates/rustify-docs/src/content.rs#L1789) 的隐藏状态 URL 也进入同一图。[资源输出](../../../../crates/rustify-docs/src/generation.rs#L825) 只使用已分析字节，并从 assets 的直接复制中排除 private / theme document。 |
| 原始 HTML / 注册 page / slot → 实际文档 | [render_html](../../../../crates/rustify-docs/src/markdown.rs#L245)、[HTML scan](../../../../crates/rustify-docs/src/html_check.rs#L548) 拒绝 script/style/frame/plugin、事件属性、inline style 与经解码/去控制空白后的 script URL；[semantics](../../../../crates/rustify-docs/src/html_check.rs#L208) 按实际树及 template 存储节点选择检查范围。注册页面与 slots 都将 fragment issues 汇入 [Analysis](../../../../crates/rustify-docs/src/content.rs#L1440)。 |
| 已分析片段 / layout markers → shell / 最终 HTML | [resolve_fragment](../../../../crates/rustify-docs/src/generation.rs#L275) 在 href/src/srcset 与 CSS URL 位置按相应语法编码；[shell marker 插入](../../../../crates/rustify-docs/src/shell.rs#L305) 在插入作者片段之前定位所有替换范围，不让作者内容成为二次替换输入。[check_document](../../../../crates/rustify-docs/src/html_check.rs#L707) 和 [final_documents](../../../../crates/rustify-docs/src/generation.rs#L629) 再查最终 HTML policy、命名冲突和实际公开 URL / fragment。 |
| SSR helper / browser metadata → typed renderer | [SSR helper](../../../../crates/rustify-docs/src/site/ssr.rs#L30) 从 stdin 解析 typed request；[handle](../../../../crates/rustify-docs/src/site/ssr.rs#L44) 检查协议、重复 key / seed；[component factory](../../../../crates/rustify-docs/src/site/registry.rs#L58) 将 props 反序列化到声明类型。[hydrate](../../../../crates/rustify-docs/src/site/hydrate.rs#L103) 检查 protocol、容器 key、非空唯一 seed，并只接受 component / demo 目标；未见按输入调用 eval、shell 或任意函数名的路径。 |
| 编译诊断 / building state → dev DOM | [element](../../../../crates/rustify-docs/assets/dev.js#L67) 使用 textContent；[prose / breakable](../../../../crates/rustify-docs/assets/dev.js#L198) 使用文本节点和固定 code/wbr 节点，文件名、message、suggestion 均不进入 innerHTML。EventSource query 使用 encodeURIComponent。shell 仅在 dev flag 下链接该脚本；开发诊断的原路径显示不能扩成发布态泄漏结论。 |

## 疑点复核

检查过 theme document 的三种用法：[资源 get](../../../../crates/rustify-docs/src/content.rs#L992) 与 README 桥的入口路径拒绝 theme 文档，而 [Sources::read](../../../../crates/rustify-docs/src/content.rs#L1008) 允许显式源 include 将其作为 code 文本。另一方面 [ThemeInput](../../../../crates/rustify-docs/src/site.rs#L58) 的 typed ThemeDocument 本来进入 browser IslandMetadata。当前证据只支持资源/阅读入口与 source include 的策略差别，不能证明泄露了新的受保护数据，因此未计作漏洞；也不据此承诺原 JSON 中任意内容都保密。

另核对大小写别名假设：在本机 Darwin 对主范围内的 `config.rs` 使用无写入的 libc realpath 探针，`CONFIG.RS` 可访问，但 canonical leaf 为实际的 `config.rs`。这个结果未支持利用简单大小写别名绕过真实目标检查；没有读取 `.env` 或其他秘密文件。未创建 symlink 或竞争修改输入来做动态攻击。

## 凭证与执行 sink 检查

对十二个主范围文件执行强 provider key、私钥、带密码数据库连接串和 hardcoded-secret assignment 模式搜索，候选为 0；未回显密钥值。补查该范围的 unsafe block、process Command、eval / Function、document.write、innerHTML / outerHTML，未发现这些执行 sink。实际 inner_html 使用已另按上表核对，未凭关键词缺失替代调用分析。

本轮未扫描环境文件、凭据目录、Git 历史、依赖清单或 lock 的 CVE，也未重验既有 Rust/浏览器功能测试。不能将这里的 0 发现当作全仓凭证或依赖结论。

## 证据边界

Rust site registry 与 authored static assets 是构建者信任输入；SSG 检查不提供执行不可信 Rust 的 OS 沙箱。is_private 是约定的文件名/路径规则，不能识别每个任意命名文件里的秘密。输入 receipts 提供代际一致性检测，不在本次审计中证明对恶意并发文件系统修改的原子安全隔离。

审计到的是库内策略、转义、资源图和守卫实现。CLI 的网络监听/HTTP 路由、发布事务、构建进程/缓存/最终守卫调用，依赖/编译器包、真实部署的 response headers/CSP 与 R/C 分发由 Root 另查。此报告不代替功能、性能、浏览器验收、review-code 或完整安全评审；未新增待修复安全项。

## 主范围文件指纹

SHA-256 为本轮直接读取的当前文件内容，方便冻结 R/C 时辨认覆盖边界；不是已提交候选身份。

| 文件（crates/rustify-docs/ 下） | SHA-256 |
| --- | --- |
| src/config.rs | 2781aec46c3921124347930e7b1427a00c750bc2482965d3a67341cc91e48e81 |
| src/content.rs | b3c990584b520dd1c587cfdb08705f6fb061ec04310706651ee16fdff7940301 |
| src/generation.rs | 7898e44be0146b6709c127297c9ced211dcfe85ec88dacdbc8e9c588099e035f |
| src/html_check.rs | 835733d94cd99f6206004ed2ea9f662a76a8762d189268987859c68ee3dcf057 |
| src/markdown.rs | fc9b8cd488fba2cafacf71c02c75fe80185429938544ebf3547e1b831546f3eb |
| src/shell.rs | 28bae15484214f15e7706c6e887d6f47ba52644a63f3559e91b5d2481140accc |
| src/style.rs | a991d583627ab0b7046fa7dbdb167ef699cbf4329149587573c13d11755dfd9c |
| src/site.rs | 27b4b3ebfd17f3014683bbec035a57076f4203e59b65a718bfce9893d7df1852 |
| src/site/registry.rs | d665ace10c20dd9be8ca76c5d417d8d0cf0f530d0dfeccd93be66bde8421e4f6 |
| src/site/ssr.rs | be0e55a080c68c687d884a042f1b1b64fd22f3e3f9ae093305fd8c25626eb7c0 |
| src/site/hydrate.rs | f4e0cfb5c10ce21432a8035b0af07696bf10465c8df7d4690fb84e192b20cb46 |
| assets/dev.js | da43de3de6ecad49dcb3c09922b2d44cffb91c4046d1773956d14cb915505a46 |
