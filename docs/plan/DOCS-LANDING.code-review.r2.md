# DOCS-LANDING · 代码评审 R2（修复效果复审）

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R2，修复效果复审 | [DOCS-LANDING](DOCS-LANDING.md) | 修复差异 `1f49749cf65c8760a402fd08538a81247eb04fe0..90667b021e26b95d76df50ca3ab121c4687898b2` 中的 9 个代码、测试与作者文档文件（代码改动只在 `0b27fdb8a213180536dac2a1666338e4e16f48db`）；无未提交改动 | [R1](DOCS-LANDING.code-review.md) |

- 评审 ID：`75bf2fdb-1034-40aa-8c05-f76eccb6b80c`。
- 日期与仓库：2026-10-21，`rustify-ui`；观察 checkout 为 `main@90667b021e26b95d76df50ca3ab121c4687898b2`，开始及验证结束时工作树、index 均干净。
- 计划版本：上述 checkout 的 [DOCS-LANDING.md](DOCS-LANDING.md)，SHA-256 `ea7f54063cb25f93b351a5f936af99390b9e22f4c0d935322604c3dc2894c128`；完成记录 M3 行登记「修复 0b27fdb8」。
- 变更意图：关闭 R1 的 F-01（透传名单的载体可被正文改写）与 F-02（站内表单目标丢查询串、只有查询串的目标落到首页）。用户请求是 `review-code docs/plan/DOCS-LANDING.md`；因已有 R1 报告，按开发计划模式执行修复效果复审。本轮没有修改产品代码或计划。

## 复审记录

- 上轮报告：[DOCS-LANDING.code-review.md](DOCS-LANDING.code-review.md)，SHA-256 `56989b46cecec14e7b1301c0136962bf60277326589f04557ca81ed10c956c88`，评审 ID `e9b98a4e-7b97-4d3a-8cab-dfcf465d9c31`；它所审的是 `b45e1ab4..1f49749c` 的 34 个文件。该报告随 `0b27fdb8` 入库，内容与 R1 落盘时一致。
- 修复差异：`1f49749c..90667b02`。`0b27fdb8` 是评审修复；`b9c50985`、`8307ccde`、`90667b02` 以及 `0b27fdb8` 中的计划、M3/M5 记录、R1 报告文件只是进度、处置或台账回写，不作为代码 findings 对象。
- 当前核查版本：`main@90667b02`；9 个修复文件在当前 checkout 与 `0b27fdb8` 逐字节一致（`git diff 0b27fdb8 HEAD -- crates tests docs/docs-ssg.md` 为空）。
- 已有处置：[M3 记录](DOCS-LANDING.records/M3.md)的「评审修复」与「评审修复后的验证」。
- 本轮类型：修复效果复审。覆盖 R1 全部问题（F-01、F-02）、R1 无待核实项，以及修复差异可能引入或激活的回归；原 34 个文件中未被修复触及的部分沿用 R1 结论，没有完整重审。

## 上轮问题复审

| 来源报告 / F-ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| [R1](DOCS-LANDING.code-review.md) / F-01 | P2：`site.js` 把根元素的 `data-rd-carry` 当名单，正文里的 `<html data-rd-carry="secret">` 会被浏览器合并进根元素，没有 `[landing]` 的站点也能透传任意参数 | 名单移到主题写在 `<head>` 的 `<meta name="rd-carry">`，`site.js` 只从 `document.head` 读取；`0b27fdb8` | **已解决** | [shell.rs](../../crates/rustify-docs/src/shell.rs) 第 525 行根元素不再带名单，第 530 行仅在 `carry` 非空时写 head meta；[site.js](../../crates/rustify-docs/assets/site.js) 第 23–25 行只查 `document.head`。正文写 `<meta>` 是构建错误（[html_check.rs](../../crates/rustify-docs/src/html_check.rs) 第 69 行），实测 `<meta name="rd-carry">` 与 `<svg><meta …></svg>` 都以 `meta-element` 退出 1；正文里的 `<meta>` 即使存在，HTML 解析器也把它留在 body，进不了 head。按 R1 原复现（无 `[landing]` 的 `content` fixture）重建并在 Chromium 打开 `guide/?secret=do-not-forward`：根属性仍被合并为 `secret`，但 head 无名单，链接保持 `https://example.org/signup`，点击后捕获的导航不带参数。落地页正常透传由 spec 覆盖，`docs-subpath`、`docs-root` 均通过 | 无 |
| [R1](DOCS-LANDING.code-review.md) / F-02 | P2：表单站内目标交给链接解析器，查询串丢失；`action="?campaign=paid"` 被解析成首页 | 新增 `Target::Form { page, query, anchor }` 与 `form_page_target`，查询串原样保留，只有查询串的目标绑定当前页；`page_url` 拼接查询；`0b27fdb8` | **已解决** | [content.rs](../../crates/rustify-docs/src/content.rs) 第 3309–3353 行、[generation.rs](../../crates/rustify-docs/src/generation.rs) 第 262–273 行。隔离副本（关 JS）实测：GET `?campaign=paid` 落在本页 `/docs/probe/?q=boots`，标题 `Form probe | Landing Fixture`，不再是首页；POST 的 `/thanks/?utm_source=a&amp;utm_campaign=b`、原始 `?a=1&b=2`、`thanks.md?campaign=paid`、`?q=a%20b&amp;n=%E4%B8%AD`、`formaction="/thanks/?via=button"`、带锚点的 `/form/?c=1#…` 都输出正确属性，请求 URL 保留完整查询；指向不存在页面的 `/nope/?c=1` 仍报 `broken-link`。根部署由新 Rust 测试与 `docs-root` spec 覆盖 | 无 |

R1 对 F-01 的回归建议包括「无 profile 的落地页、空 carry 配置」两种情形。修复后名单只在配置的 `carry` 非空时由主题写入（[shell.rs](../../crates/rustify-docs/src/shell.rs) 第 506–508、530 行），`site.js` 不再读根元素，载体与页面类型无关，所以这两种情形在构造上也没有名单；它们没有单独的浏览器注入用例，不影响关闭判定。

## 确认问题（本轮新增）

### F-03 · [P3] 写明 GET 表单不会发送 action 中的查询串

- 定位：[docs/docs-ssg.md](../docs-ssg.md) 第 1312–1315 行，target `0b27fdb8`（与当前 HEAD 相同）。
- 问题：修复新增的作者文档说表单目标的查询串「会被保留，是读者发送内容的一部分」，并举例 `action="?campaign=paid"` 会「带着该查询提交到本页」，但没有限定方法。对 `method="get"` 的表单，浏览器提交时用表单字段替换 action URL 的整段查询，`campaign=paid` 不会被发送；只有 POST 才把它随请求带上。构建器本身没有错（它原样写出 `action="/docs/probe/?campaign=paid"`），错在文档向作者承诺了 GET 下不存在的行为：作者照此用 GET 表单附带投放参数，归因会静默丢失。同一修复的 fixture `form.md` 用的正是 GET 例子，spec 注释也写明「a GET submission replaces the query with the form's own fields」，但作者文档没有同步这一点。
- 证据：Chromium 关 JS 提交隔离副本的 `form#get`：属性为 `/docs/probe/?campaign=paid`，到达地址为 `/docs/probe/?q=boots`，没有 `campaign`；同一页的 POST 表单都保留了查询串。[landing.spec.ts](../../tests/docs-ssg/landing.spec.ts) 第 297–299 行的注释。标准依据：WHATWG HTML 表单提交算法的 [Mutate action URL](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#submit-mutate-action) 用序列化后的字段设置 action URL 的 query。
- 修复与验证：把这一条限定为 POST（例如示例改成 `<form method="post" action="?campaign=paid">`），并补一句：GET 表单的查询会被字段替换，需要固定带上的参数改用隐藏字段（透传参数用 `data-rd-carry` 隐藏字段）。纯文档改动，按 V-8 重新核对文档即可。
- 置信度：高。浏览器实测与标准一致；影响限于作者文档，严重度低。

## 待核实项与验证限制

- 没有影响上述结论的待核实项。
- 本轮浏览器只重跑了 `docs-subpath`、`docs-root` 的 landing spec 和隔离复现。M3 记录中的 `docs-meta`、`docs-webkit` landing spec，整个 docs 项目，以及 `components-subpath`、`official-subpath` 的回归没有重跑，计为未复核；Firefox 仍按 M5 记录的限制处理。
- M3 记录里「修前失败」的两条 spec 没有重跑。本轮以 R1 的修前复现和本轮修后实测对照；静态看，两个新 Rust 测试和两条新 spec 在修前版本上会失败（修前输出 `action="/docs/"`，根元素带名单）。
- 没有执行独立 UI/UX 验收、发布或远端 CI。

## 覆盖与验证

### 范围固定与文件覆盖

| 类别 | SHA / 范围 | 处理 |
| --- | --- | --- |
| 评审修复 | `0b27fdb8a213180536dac2a1666338e4e16f48db` | 纳入：`crates/rustify-docs/{assets/site.js,src/content.rs,src/generation.rs,src/shell.rs,tests/landing.rs}`、`crates/rustify-docs-cli/tests/landing.rs`、`tests/docs-ssg/{landing.spec.ts,fixtures/landing/content/form.md}`、`docs/docs-ssg.md` |
| 进度、处置与台账回写 | `b9c50985cf458acc932ce8d5240f271ab9ee9ee7`、`8307ccde3e4b48a18ebb0a6f9d7792b7deee47b4`、`90667b021e26b95d76df50ca3ab121c4687898b2`，以及 `0b27fdb8` 中的计划、M3/M5 记录与 R1 报告 | 不作为代码 findings 对象；用作契约与处置证据 |
| 未提交变更 | 开始及验证结束时为空 | 本报告与后续台账登记不纳入本轮输入 |

取证命令：`git diff --no-ext-diff --no-textconv 1f49749c 90667b02 -- crates tests docs/docs-ssg.md`。9 文件指纹：对 `git diff --name-only -z` 清单逐文件计算 SHA-256，再对按路径排序、紧凑 JSON 编码的 `{path: sha256}` 计算 SHA-256，得 `8e8471c247481f216d0416452561e952ee52772b1ca126ddfbd8822adddb765b`；结束时复核未变。

| 行为链 | 阅读与核查范围 | 结果 |
| --- | --- | --- |
| 名单载体：配置 → `page_view` → head meta → `site.js` | `shell.rs` 的 `page_view`、`site.js` 的 `carriedNames`/`carry`、`html_check.rs` 的 `FORBIDDEN`，以及 body 内 `<meta>` 的 HTML 解析位置 | F-01 关闭；无 `[landing]` 站点不输出名单（隔离副本 `guide/` 中 0 处） |
| 表单目标：`RefKind::Form` → `form_target` → `form_page_target` → `Target::Form` → `page_url` | `content.rs` 的 `resolve_references`、`form_target`（空串和纯 `#` 仍为 `Keep`）、`form_page_target`、`LinkGraph::check_anchor`（从闭包抽出，语义不变）；`generation.rs` 的 `resolve_fragment`、`target_url` | F-02 关闭 |
| 新变体的消费者 | 全部 `Target::Page`/`Target::Form` 匹配：`header_links` 只解析配置链接，`collect_resources`、`refuse_theme` 只看资源，`markdown_intro` 的 `form_target` 固定返回 `Keep`；含路径目标仍经 `resolve_link`，草稿与语言投影照旧 | 未发现回归 |
| URL 序列化 | 槽位值是实体解码后的值（`html_check.rs` 第 51 行），`escape_url_into` 再做百分号编码与 HTML 转义 | 多参数、`&amp;`、编码查询实测正确，没有双重转义 |
| 测试与 fixture | `tests/landing.rs` 两个新测试、CLI landing 的 `page_policies` 键集、spec 两条新用例、`form.md` | 断言覆盖两条修复；新页进入 `page_policies` 后测试已同步 |
| 作者文档 | `docs/docs-ssg.md` Forms 一节新增条目 | F-03 |

由当前主 agent 完成，未委派 reviewer。

### 本轮实际运行

命令均在仓根执行，编译经 mise 固定的工具链与 mbx。隔离复现放在本会话的系统临时目录，没有向产品源码加入测试或修复；外链导航与 POST 请求由 Playwright route 拦截并就地应答，未接触真实服务。

| 命令 / 检查 | 实际结果 |
| --- | --- |
| `mbx test --locked -p rustify-docs --lib --test landing` | 256 个单元测试、6 个 landing 集成测试通过 |
| `mbx test --locked -p rustify-docs --test generate --test i18n --test search --test ai` | ai 8、generate 17、i18n 33、search 12，全部通过 |
| `mbx test --locked -p rustify-docs-cli --test landing` | 3 项通过 |
| `cargo fmt --all -- --check` | 通过 |
| `mbx clippy --locked -p rustify-docs --all-targets -- -D warnings` | 通过 |
| `mbx clippy --locked -p rustify-docs-cli --all-targets -- -D warnings` | 通过（两个包分别运行） |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 通过；隔离复现使用该二进制 |
| `git diff --no-ext-diff --no-textconv --check 1f49749c 90667b02 -- crates tests docs/docs-ssg.md` | 通过 |
| `npx playwright test -c playwright.docs.config.ts --project=docs-subpath tests/docs-ssg/landing.spec.ts` | Chromium 9/9 通过，含两条新用例 |
| `npx playwright test -c playwright.docs.config.ts --project=docs-root tests/docs-ssg/landing.spec.ts` | Chromium 9/9 通过 |
| `content` fixture 副本 + R1 的 F-01 复现正文 → build → preview → Chromium | 构建退出 0；根属性被合并为 `secret`，head 无名单，链接与实际导航都不带参数 |
| `content` fixture 副本，正文写 `<meta name="rd-carry">`，另一副本写 `<svg><meta …></svg>` | 两次构建都以 `meta-element` 退出 1 |
| `landing` fixture 副本 + 七个表单的探针页 → build → preview → Chromium（关 JS） | 属性与请求 URL 见上轮问题复审表；GET 结果见 F-03 |
| 同一探针页把一个目标改为 `/nope/?c=1` | 构建以 `broken-link` 退出 1 |

## 计划符合性

| 本期承诺 | 实现与证据 | 判断 |
| --- | --- | --- |
| R-7、NFR-6 | 名单只来自配置；正文无法写入或改写（F-01 复审） | 已核实 |
| NFR-4 | 无 `[landing]` 站点每页无名单；`a_site_without_profiles_is_as_strict_as_before` 通过；`site.js` 指纹变化在允许范围 | 已核实 |
| R-5、NFR-5 | 站内表单目标保留查询串，只有查询串的目标指向本页；关 JS 的 GET/POST 实测 | 已核实 |
| R-9 | Forms 一节新增条目，GET 情形描述不准 | F-03（P3） |
| 计划正文同步 | §4、§5.4、§5.5、§5.6、D13 与实现一致。V-3（[计划](DOCS-LANDING.md)第 654 行）仍写「`page_policies` 只包含 `lp/`、`thanks/` 两项」，§9.1 的 fixture 约定也没有列出 `form.md`，而 [tests/landing.rs](../../crates/rustify-docs/tests/landing.rs) 第 293–299 行已断言三项；[M3 记录](DOCS-LANDING.records/M3.md)第 34 行登记的 R1 报告 SHA-256 有一位笔误（`cecee14` 应为 `cecec14`，台账中的值正确） | 计划与记录的小偏差，不是代码问题，不影响验收；建议下次回写时顺手更正 |

计划来源为用户需求，无 PRD。CLI 发布、线索后端等排除项没有被要求补做。

## 总体结论与修复顺序

**验收判定：通过验收。**

修复结论：**已解决**。R1 的 2 项全部实际解决，0 项按不再适用处理；修复没有引入阻塞回归。修复差异判定为 `patch is correct`；新增的 F-03 是 P3 作者文档问题，不阻止通过。主机端测试、Clippy、fmt 和两个部署形态的 landing spec 都通过；R1 的两条复现和多种表单目标在真实浏览器中按预期工作。

未覆盖：M3 记录中的 `docs-meta`、`docs-webkit` 等回归本轮未重跑；原 34 个文件中未被修复触及的部分沿用 R1 结论。

可选：用 `apply-code-review docs/plan/DOCS-LANDING.code-review.r2.md` 处理 F-03，顺带更正计划 V-3、fixture 约定与 M3 记录的 SHA 笔误，之后照常复审。本判定只覆盖所审修复差异，不代表合并或发布授权。
