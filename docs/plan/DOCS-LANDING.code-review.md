# DOCS-LANDING · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1，完整初审 | [DOCS-LANDING](DOCS-LANDING.md) | `b45e1ab41dc44ee10d5832365d3acfb5637fee77..1f49749cf65c8760a402fd08538a81247eb04fe0` 中的 34 个实现、测试与作者文档文件；无未提交代码 | 无；现有 `.review.md` 是计划文档评审 |

- 评审 ID：`e9b98a4e-7b97-4d3a-8cab-dfcf465d9c31`。
- 日期与仓库：2026-10-21，`rustify-ui`；观察 checkout 为 `main@b9c50985cf458acc932ce8d5240f271ab9ee9ee7`，开始及验证结束时工作树、index 均干净。
- 计划版本：上述 checkout 的 [DOCS-LANDING.md](DOCS-LANDING.md)，SHA-256 `f117b223653feca28f83c629cef031400e6a0d7750184021813fcadfc628dbfa`，完成记录 M1–M5。
- 变更意图：为 Markdown 增加 DOM 落地页布局、命名 CSP 配置、原生表单与 iframe、广告参数透传，并使 meta、manifest、preview/dev 响应头一致。用户请求是代码评审；实现范围由计划完成记录确定。本轮没有修改产品代码或计划。

## 确认问题

### F-01 · [P2] 将透传白名单绑定到正文无法伪造的配置来源

- 定位：[crates/rustify-docs/assets/site.js](../../crates/rustify-docs/assets/site.js) 第 27–30 行，target 侧 `1f49749c`。
- 问题：新的透传逻辑直接把运行时 `<html data-rd-carry>` 当作配置白名单，但 Markdown 原始 HTML 允许再次出现 `<html>`，浏览器会把其中根元素尚未拥有的属性合并到真正的 documentElement。普通文档中加入下述两行即可通过构建，并在没有任何 `[landing]` 的站点上启用任意参数透传。打开 `guide/?secret=do-not-forward` 后，外链变成 `https://example.org/signup?secret=do-not-forward`，点击时浏览器实际发起带该参数的导航。正文作者由此绕过配置的参数名单及落地页范围；严格 CSP 仍然存在，顶层外链导航仍携带参数，违反 R-7、NFR-6，也改变了无 landing 配置站点的行为。
- 证据：`html_check.rs:57–71` 的禁止元素中没有 `html`，实际构建退出 0。Chromium 在真实 preview 中确认根属性被合并、外链被修改并捕获了导航请求；请求在浏览器中拦截返回，未发送给外部站点。对同一生成页面仅替换为基线 `b45e1ab4` 的原版 `site.js` 后，根属性仍为 `secret`，外链保持 `https://example.org/signup`，证明问题由新增透传逻辑激活。契约见 [计划](DOCS-LANDING.md) 第 81、87、89 行及 §5.6。
- 修复与验证：把名单放在主题自有、正文无法注入或覆盖的载体上，或在构建期校验最终有效根属性与所选 profile 一致。增加无 `[landing]` 的普通页、无 profile 的 landing 页和空 carry profile 的正文属性注入回归，确保均不会携带未配置参数；保留正常 profile 的透传用例。
- 置信度：高，已完成构建、真实浏览器导航和单资产基线对照。

复现正文（追加到 `tests/docs-ssg/fixtures/content` 副本的 `content/guide/index.md`）：

```html
<html data-rd-carry="secret">
<a data-rd-carry href="https://example.org/signup">Continue</a>
```

### F-02 · [P2] 保留表单目标的查询串及当前页语义

- 定位：[crates/rustify-docs/src/content.rs](../../crates/rustify-docs/src/content.rs) 第 3146–3150 行，target 侧 `1f49749c`。
- 问题：新加入的 Form 分支把站内 action/formaction 交给现有链接解析器，而该解析器以去掉查询串后的路径查找页面，随后 `Target::Page` 的生成只保留路由与锚点。这使合法的 `action="?campaign=paid"` 在 `content/lp.md` 中被解析为首页，并生成 `action="/docs/"`；一个原应提交到当前落地页的 GET 表单实际导航到首页。显式指向站内页面的 POST 目标也会丢失查询串，例如 `/thanks/?campaign=paid` 被输出为 `/docs/thanks/`。构建成功却改变了作者指定的提交目的地或参数，影响 R-5 承诺的原生表单；在禁用 JS 的浏览器上下文中同样发生。
- 证据：`content.rs:3360` 去掉查询串，`3434–3442` 把空解码路径映射为 `index.md`；[generation.rs](../../crates/rustify-docs/src/generation.rs) 第 314–317 行生成页面目标时没有恢复查询串。隔离副本构建均退出 0。Chromium 对 GET 表单实测：预期 `/docs/lp/?q=boots`，实际 `/docs/?q=boots`，标题为 `Landing Fixture`；对 POST 表单捕获到的请求路径也分别为 `/docs/` 与 `/docs/thanks/`，缺少 `campaign`。基线未识别和重写 form action；本次新增的 Form 解析分支激活了这条错误链。契约见 [计划](DOCS-LANDING.md) 第 79 行及 §5.5；[作者文档](../docs-ssg.md) 的 Forms 一节允许站内页面目标。
- 修复与验证：为表单保留查询串；只带查询串的目标应绑定当前页面，再执行需要的 base/路由转换。覆盖 action 与 formaction、仅查询串与显式页面路径、根部署与子路径部署，并在无 JS 的 GET/POST 场景核对实际请求 URL。无需扩大可提交到的站内端点范围。
- 置信度：高，生成 HTML 和真实浏览器提交请求均已确认。

GET 复现正文（替换 landing fixture 副本的 `content/lp.md` 正文，保留 `layout: landing`、`landing: ads`）：

```html
<form method="get" action="?campaign=paid"><input name="q" value="boots"><button>Send</button></form>
```

## 待核实项与验证限制

- 没有影响上述两个问题成立性的待核实项。
- 本轮浏览器使用本机已安装的 Playwright Chromium；未重跑 WebKit、Firefox、真实厂商、页面性能或角色模拟。其历史证据及 Firefox BiDi 限制保留在 [M5 记录](DOCS-LANDING.records/M5.md)，没有计为本轮通过。
- 未执行独立 UI/UX 打分验收、发布或远端 CI。代码评审中的 DOM、导航、表单及 axe 检查不代替独立 `review-ui`。

## 覆盖与验证

### 范围固定与文件覆盖

按开发计划模式审查两个已登记实现提交的累计净效果：

| 类别 | SHA / 范围 | 处理 |
| --- | --- | --- |
| M1–M4 实现 | `b49aa8fe7652b31dce323367a3b2a975b815b740` | 纳入 |
| M5 收口修复 | `1f49749cf65c8760a402fd08538a81247eb04fe0` | 纳入，与前一实现提交合并看最终行为 |
| 进度、计划评审及台账回写 | `b481b960d8ee62913740e5fd4dbd0d014aa1a027`、`2c08d9f32d7742bbc65b121db191b4df00d7c7c1`、`b9c50985cf458acc932ce8d5240f271ab9ee9ee7` | 不作为代码 findings 对象；计划和记录用作契约及历史证据 |
| 未提交变更 | 开始及验证结束时为空 | 报告和后续台账回写不纳入本轮输入 |

实际 diff 使用 `git diff --no-ext-diff --no-textconv b45e1ab41dc44ee10d5832365d3acfb5637fee77 1f49749cf65c8760a402fd08538a81247eb04fe0 -- crates tests docs/docs-ssg.md`。上述 34 个文件在当前 checkout 与 target 完全一致，运行检查使用这份相同源码。末次内容指纹复核无变化。

34 文件指纹：对该命令的 NUL 分隔文件清单逐文件计算 SHA-256，再对按路径排序、紧凑 JSON 编码的 `{path: sha256}` 计算 SHA-256，结果为 `0e9d6bf56d55dd5e5a0d0aad1f72c1f4433116b2e94ccf4fab20cb512dc18b4e`。

| 行为链 | 阅读与核查范围 | 结果 |
| --- | --- | --- |
| 配置 → profile → CSP | `config.rs`、`landing.rs`、`frontmatter.rs`、调用方和新增单元测试 | 命名、来源/端口/通配校验、脚本来源合并、严格策略默认值已核查 |
| Markdown → 页面分析 → 生成闸门 | `markdown.rs`、`html_check.rs`、`content.rs`、`generation.rs`，新增及直接相关测试 | 普通页表单兼容、iframe 按页拒绝/放行、资源与本地脚本、最终引用检查；F-02 |
| 策略 → manifest → HTTP | `publish.rs`、CLI `main.rs`/`compile_proxy.rs`/`preview.rs`/`dev.rs`、CLI landing 测试 | 按页头、默认严格头、损坏 manifest 拒绝、dev 内存快照已核查 |
| DOM 布局与可发现性 | `shell.rs`、`search.rs`、`ai.rs`、`lib.rs`、包测试声明、集成测试 | 品牌/导航/页脚、hero、noindex/搜索/AI 默认值；未引入 GPU 路径 |
| 浏览器透传与提交 | `site.js`、`landing.spec.ts`、第三方替身、fixture 全部文件、serve/static-host 改动 | 现有浏览器主路径通过；额外反证确认 F-01、F-02 |
| 作者资料与完成契约 | `docs/docs-ssg.md` diff、计划正文、M1–M5 记录、仓库指令与产品/设计上下文 | 对照本期承诺及排除项；未改原计划或记录 |

由当前主 agent 完成，未委派 reviewer。两项 findings 都经过反证：F-01 使用原版脚本单资产对照；F-02 关闭 JS 并观察浏览器原生提交。省略 form action 的检查边界已在作者文档写明，没有把它另报为缺陷。

### 本轮实际运行

命令均在仓根执行；编译入口经 `mise exec --` 使用仓库固定工具链。临时复现项目使用系统临时目录，没有向产品源码加入测试或修复。

| 命令 / 检查 | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-docs --lib --test landing` | 256 个单元测试、4 个 landing 集成测试通过 |
| `mbx test --locked -p rustify-docs --test generate --test i18n --test search --test ai` | generate 17、i18n 33、search 12、ai 8，全部通过 |
| `mbx test --locked -p rustify-docs-cli --test landing` | 3 项通过，含 preview/dev 按页头及损坏 manifest |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 通过；随后浏览器运行使用更新后的二进制 |
| `cargo fmt --all -- --check` | 通过 |
| `mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` | 通过 |
| `mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings` | 通过；两个包分别运行 |
| `npx playwright test -c playwright.docs.config.ts --project=docs-subpath tests/docs-ssg/landing.spec.ts` | Chromium 7/7 通过：CSP/来源隔离、白名单参数与字面量、iframe 参数/历史、JS 开/关的 POST→303、四宽度、页脚导航、axe |
| 隔离 `content` fixture，追加 F-01 HTML，build→preview→Chromium；原版 `site.js` 对照 | 无 landing 配置仍携带未配置参数；基线脚本不携带。外部导航由浏览器 route 拦截，未接触真实服务 |
| 隔离 `landing` fixture，替换 lp 表单，build→preview→Chromium | F-02 的 GET 导航到首页；POST 请求丢查询串；均关闭 JS，POST 请求在浏览器 route 中捕获返回 |
| `git diff --no-ext-diff --no-textconv --check b45e1ab4 1f49749c -- crates tests docs/docs-ssg.md` | 通过 |

原生表单复现是在请求发出时确认目标 URL，未搭建或调用真实线索接收后端。F-01 的基线对照只替换已读取的旧 `site.js`，不是对整个旧 CLI 的运行验证。

## 计划符合性

| 本期承诺 | 实现与证据 | 判断 |
| --- | --- | --- |
| R-1、R-2、R-3、R-4、NFR-1、NFR-2 | profile/布局/策略生成、manifest、CLI 与浏览器安全门 | 在所审路径上已核查，未发现额外确认缺陷 |
| R-5、NFR-5 | 原生表单主路径及无 JS 的 POST→303 通过；含查询串的站内目标复现错误 | 存在 F-02 |
| R-6 | Markdown iframe 按 profile 校验，渲染面仍禁止；浏览器嵌入及历史检查通过 | 已核查 |
| R-7、NFR-6、NFR-4 | 正常 carry 测试通过；无配置普通页可由原始 HTML 自行启用 | 存在 F-01 |
| R-8 | 同一份生成策略驱动 meta、manifest、preview/dev，CLI 定向测试与真实 preview 通过 | 已核查 |
| R-9 | 文档 diff 与实现契约对照；M4/M5 记录有示例与角色模拟结果 | 已阅读；未重新执行真实厂商与角色模拟 |
| NFR-3 | landing 集成测试含框架 gzip 预算；M5 有页面内性能记录 | 重量断言本轮通过；LCP/CLS 本轮未重测 |

本轮覆盖计划 M1–M5 登记的已交付代码；计划来源为用户需求、无 PRD。CLI 发布、官方站点更新、线索后端、同意管理等排除项没有被要求补做。

## 总体结论与修复顺序

**验收判定：需要修改。**

`patch is incorrect`。确认 2 项 P2：F-01 使正文可绕过配置启用参数透传，F-02 改变合法原生表单的目的地与查询参数。已有测试及本轮定向检查通过，但没有覆盖这两个已复现的边界。

先修 F-01 的配置与正文边界，再修 F-02 的表单 URL 处理；补上对应反例，重跑相关 landing 主机端与浏览器回归后复审。本判定不改变历史实施完成记录，也不代表合并或发布授权。

下一步：`$apply-code-review docs/plan/DOCS-LANDING.code-review.md`；修复后执行 `$review-code docs/plan/DOCS-LANDING.md`。
