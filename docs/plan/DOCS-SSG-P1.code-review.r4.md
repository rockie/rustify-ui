# DOCS-SSG-P1 · 代码评审 R4

验收判定：**需要修改**。上轮四个剩余问题的原复现条件均已解决；本轮新增 **1 项 P2（F-20）**，SVG 中的同名元素误入 HTML template 计数，导致实际存在的锚点被拒绝。修复效果 **未完全解决**，修复增量 `patch is incorrect`。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R4 | [DOCS-SSG-P1](DOCS-SSG-P1.md)，对象 `526c88c7-a4b3-4ea9-9193-371420b66d1d` | `1f0fc5ed21d7937a1297528ad38cdefe6b95f721..540462a895a48385761f8db6e3271a714957d818`；修复提交 `016110f36e7ca6aad26c19f1447557dbc30e5a40`；输入指纹 `f1dc18e6d6acfbc11250aea7fd9bbbea88f8a9aed7843f828a05a9afbe8a4329` | [R3](DOCS-SSG-P1.code-review.r3.md)，对象 `7b6a9d1a-088d-4213-be21-74348a929584` |

本轮对象 ID：`1996cbdf-4d81-4ba7-abac-d31ad78019a5`。模式为**修复效果复审**，覆盖 R3 的全部 19 个旧项、四个剩余条件及本次修复直接影响的回归；没有重新完整审查 M1–M8，M9 排除。

上轮所审 `bfbee6d3342b69e062ed513e05e7f812373124bc..1f0fc5ed21d7937a1297528ad38cdefe6b95f721`，报告 sha256 `bc3560b1bfa527661549d0363418a53da751b6547c955fe7f8a710a139e3cd67`。本轮从该目标起算，当前核查版本为 `540462a895a48385761f8db6e3271a714957d818`，未提交代码差异为空。已有处置来自 [M3](../validation/docs-ssg-p1/M3.md#评审修复)、[M6](../validation/docs-ssg-p1/M6.md#评审修复)、[M7](../validation/docs-ssg-p1/M7.md#评审修复)及 [M8 的 R3 修复后复验](../validation/docs-ssg-p1/M8.md#r3-修复后复验)。原报告与计划保持原位。

## 确认问题

### 上轮问题复审

F-01–F-14 身份沿用 [R1](DOCS-SSG-P1.code-review.md)（对象 `65ddeb43-1845-4bee-b70c-bbf9647aa8b7`），F-15–F-17 沿用 [R2](DOCS-SSG-P1.code-review.r2.md)（对象 `ff41c49f-1b63-4b5b-85f2-4dbdfb96d73d`），F-18/F-19 属于 R3。下表核查的是当前实现；“已解决”只关闭对应旧问题，不掩盖本轮新增回归。

| 来源 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01 | 输入真实路径出界；R3 已解决 | `12113ab` | **已解决** | `real_source/read_source` 授权边界保留；新监听不授权读取，库/CLI 工程外链接拒绝及旧产物保全测试通过 | 无 |
| R1 / F-02 | 私有来源经别名/品牌发布；R3 已解决 | `12113ab` | **已解决** | 别名和真实路径的 private 校验保留；新依赖测试排除私有及工程外目标，CLI 私有资源测试通过 | 无 |
| R1 / F-03 | 私有 Markdown 被当页面；R3 已解决 | `12113ab` | **已解决** | 遍历过滤未改，库 `private_markdown_is_never_a_page` 和 CLI 私有发布回归通过 | 无 |
| R1 / F-04 | 顶栏/社交入口缺失；R3 已解决 | `12113ab` | **已解决** | 壳接线未改，docs-subpath 页头、导航、首页/内容/404及窄屏、无脚本测试通过 | 真机页头复走仍未重验，不重开已关闭实现项 |
| R1 / F-05 | 仅首页操作引用的附件未发布；R3 已解决 | `12113ab` | **已解决** | 普通链接/操作发布分支未改，CLI artifacts 首页附件回归通过 | 无 |
| R1 / F-06 | CSS 资源缺校验/改写；R3 已解决 | `12113ab`、`a297dbd` | **已解决** | 真实发布集合闸门保留；缺失前缀资源与注入生成器的负例通过，20 个生产 CSS 资源请求取得正确字节 | 无；模板锚点回归另见 F-20 |
| R1 / F-07 | URL 后缀丢失；R3 已解决 | `12113ab` | **已解决** | 普通附件和后缀分支未改，库/CLI 原后缀及各入口附件回归通过 | 无 |
| R1 / F-08 | 无换行开栏 panic；R3 已解决 | `12113ab` | **已解决** | frontmatter 保护未改，库和 CLI 无换行开栏测试通过 | 无 |
| R1 / F-09 | 忽略实际 --config；R3 已解决 | `12113ab` | **已解决** | 参数接线未改，CLI check/dev 指定配置测试及本轮隔离配置构建通过 | 无 |
| R1 / F-10 | 空站成功替换旧产物；R3 已解决 | `12113ab` | **已解决** | no-pages 闸门未改，库无页面诊断及 CLI 旧产物保全测试通过 | 无 |
| R1 / F-11 | 前置发布失败无 JSON；R3 已解决 | `12113ab` | **已解决** | CLI 输出拒绝/归属/锁及 JSON 回归通过 | 无 |
| R1 / F-12 | 百分号标题中断脚本；R3 已解决 | `12113ab` | **已解决** | 脚本未改，docs-subpath 百分号标题、目录及窄屏菜单测试通过 | 无 |
| R1 / F-13 | snapshot 内附件失效后丢恢复监听；R3 部分解决 | `016110f` / M6 | **已解决** | `walk/snapshot` 在真实读取失败前登记安全恢复目标；content/static 两种附件仅恢复目标即清错、刷新到新字节，无关页保持，CLI 与真实浏览器回归通过 | 无 |
| R1 / F-14 | 未输出脚注贡献幽灵 ID；R3 已解决 | `12113ab`、`a297dbd` | **已解决** | 最终 HTML ID 集合及未输出脚注过滤保留，同页/跨页、合法/重复/未用脚注测试通过 | 无 |
| R2 / F-15 | 解码后拆问号误伤文件名；R3 已解决 | `a297dbd` | **已解决** | 先拆后缀再解码的普通链接分支未改，CLI 编码文件名与各入口附件回归通过 | 无 |
| R2 / F-16 | 数字实体后等号吞 URL 外分隔符；R3 部分解决 | `016110f` / M7 | **已解决** | numeric 引用后 `=` 独立映射；括号/引号实体有无分号均正常生成，生产页 computed backgroundImage 有效且资源字节正确 | 无 |
| R2 / F-17 | CSS 文档内 SVG 片段被拒；R3 已解决 | `a297dbd` | **已解决** | 普通同页 SVG mask/filter 的 ID 校验、URL 保留和缺目标拒绝回归通过；docs-subpath SVG/class 背景测试通过 | 本轮未重复像素级 mask 测量；F-20 是新增命名空间条件 |
| R3 / F-18 | HTML template 内容误作文档锚点 | `016110f` / M3 | **已解决** | 同页/跨页、嵌套 HTML template 正反测试通过；ghost/deep 被拒且旧输出保全，document 中不可见、holder/普通 ID 可导航 | 原条件关闭；新增 foreign-content 回归见 F-20 |
| R3 / F-19 | CSS 资源扫描与 compiler 任意值解码不同 | `016110f` / M7 | **已解决** | URL/url、underscore、转义 underscore、%20 与空格文件均与 compiler 一致；生成路径避免二次 underscore 解码，两个 base 的 20 个加载请求均正确 | 无 |

### F-20（本轮新增）· [P2] 只让 HTML 命名空间的 template 隔离锚点

- 定位：[html_check.rs](../../crates/rustify-docs/src/html_check.rs) **169–170**，target `540462a895a48385761f8db6e3271a714957d818`，位于修复 diff。
- 问题：计数仅按标签名进入 template 状态，未区分 HTML 与 SVG 命名空间。作者写入 `<svg><template/></svg>` 后再写普通 `## Visible` 和 `[Visible](#visible)`，计数会一直保持非零，后续标题的真实 ID 不再进入锚点集合，`check/build/dev` 将其报为 `broken-anchor` 并拒绝生成；显式闭合的 SVG 同名元素也会让其后代 ID 被误排除。SVG 里的该元素没有 HTMLTemplateElement.content 隔离语义，浏览器中的 `#visible`/`#circle` 均实际存在。基线接受同一输入，此误拒由本次新增计数引入，属于 F-18 修复带来的回归。
- 证据：[双版本构建结果](../validation/docs-ssg-p1/code-review-r4/candidate-results.json) 中 `svg-template-selfclose`、`svg-template-pair`、`nested-svg-template` 均为 base 退出 0 / target 退出 1；[浏览器结果](../validation/docs-ssg-p1/code-review-r4/candidate-browser-results.json) 确认 SVG 同名元素 namespace 为 `http://www.w3.org/2000/svg`、没有 content，后续 `visible` 是可找到的 HTML H2，基线链接能跳到 `#visible`。为查看 target 的真实文档，只移去显式引用链接后构建，其 H2/ID 仍实际存在，见 [target 截图](../validation/docs-ssg-p1/code-review-r4/svg-template-selfclose.png)。因果链为 [markdown.rs](../../crates/rustify-docs/src/markdown.rs) 1182 → [content.rs](../../crates/rustify-docs/src/content.rs) 1313–1321。契约依据为 [计划 §5.2](DOCS-SSG-P1.md#52-单一内容图可定位语法与安全检查) 的真实锚点检查与 SVG/嵌套 HTML 支持。[WHATWG foreign-content 规则](https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inforeign)规定按当前命名空间插入 foreign 元素，并在自闭合时弹出；本轮浏览器结果与该规则一致。
- 修复与验证：根据实际命名空间和 HTML integration point 判断 template 边界，foreign 同名元素及其自闭合标记不能改变 HTML template 深度；也可使用能给出实际树边界的解析结果。保留原 HTML template 幽灵 ID 负例，补 SVG 同名元素显式闭合/自闭合、HTML template 内嵌 SVG 及 foreignObject 转入 HTML 的正反例，断言真实文档 ID、同页/跨页链接和 CLI 构建结果一致。
- 置信度：**高**；隔离基线二进制、当前重建二进制和真实 Chromium 文档查询共同确认。

## 待核实项与覆盖限制

没有缺证的候选 finding。F-20 已证实，无需先补环境才能处理。

- 本轮使用已安装的 Playwright Chromium `153.0.8010.12`；未重验用户 Chrome、真实 Safari、手机、WebKit 或远端 CI。375px 为仿真视口；已查看本轮窄屏和候选截图，不代称真机通过。
- 原实现的 GPU/DOM feature 矩阵、7 示例、theme-export、500 页性能、LCP/CLS、千次保存及安装演练本轮未重跑，不记为本轮通过。当前修复没有改 SDK、GPU、锁文件、工具链、调度或阅读壳；本轮结论限于声明的修复范围。
- M9 的独立 CLI 版本、锁 SHA 安装、仓外消费者、GitHub 同 SHA 验证及推送授权仍属后续；计划的 8/9 与 Ready 不因本报告改写。

## 覆盖与验证

变更意图：按 R3 关闭附件恢复、数字实体跨度、HTML template 锚点和 CSS 任意值资源解码四个条件。呈现为 **DOM 静态 HTML/SSR**，CLI 在宿主执行，无 GPU 改动。全部审查、复现和裁定由当前主 agent 完成，未委派 reviewer。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；日期：2026-10-11，+08:00；原输入：`$review-code docs/plan/DOCS-SSG-P1.md`。
- 计划 sha256：`74046ff5e67fa01f5b1cc49d7e6d95a3d2f908ae346880e4e809d70eee34c135`。完成记录登记的 7 个提交均解析为完整 SHA 并确认是 HEAD 祖先，见 [scope.json](../validation/docs-ssg-p1/code-review-r4/scope.json)。前六个登记提交由前轮覆盖，本轮核对其旧项；`016110f36e7ca6aad26c19f1447557dbc30e5a40` 纳入修复增量。
- 比较方式：固定 base→target 累计净差异；区间只有修复 `016110f36e7ca6aad26c19f1447557dbc30e5a40` 和仅改计划的 `540462a895a48385761f8db6e3271a714957d818`，后者作为进度登记排除代码 findings。无未登记的代码提交。
- 8 个代码/测试路径已完整阅读 diff：docs 库的 content/generation/html_check/markdown/style、库 tests/generate、CLI tests/dev 和浏览器 dev.spec.ts。追踪 compiler 任意值解码、资源输出、HTML span、最终锚点、CLI filter/失败轮与壳等必要上下游。实施记录和生成证据仅用于核实处置，不当作代码 finding。
- 当前 index 无 staged 差异；tracked/untracked 均枚举，只有原报告、证据和 SDLC 文档。输入集合按仓根相对路径记录文件 sha256，再对排序的映射计算整体指纹；[scope-end.json](../validation/docs-ssg-p1/code-review-r4/scope-end.json) 确认 HEAD 与全部固定输入未变。本轮新报告/证据不纳入原审查对象。
- 当前验证对应 target checkout。基线以 `git archive` 提取到隔离临时目录，以独立 CARGO_TARGET_DIR、offline/locked 和 docs-cli profile 编译；未 checkout/reset/stash 用户工作树。二进制 hash 见 candidate-results，精确命令和证据入口见 [commands.json](../validation/docs-ssg-p1/code-review-r4/commands.json)。

| 命令 / 实际步骤 | 本轮结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-docs` | 128 项通过（117 单测、11 生成集成）；[日志](../validation/docs-ssg-p1/code-review-r4/rustify-docs-test.log) |
| `mbx test --locked -p rustify-docs-cli` | 70 项通过，含读取安全、保全、JSON、真实附件恢复、配置与发布；[日志](../validation/docs-ssg-p1/code-review-r4/rustify-docs-cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 当前二进制重建通过；[日志](../validation/docs-ssg-p1/code-review-r4/cli-build.log) |
| 隔离 base：`mbx build --offline --locked -p rustify-docs-cli --profile docs-cli` | 通过；[日志](../validation/docs-ssg-p1/code-review-r4/baseline-build.log) |
| docs 两包 Clippy `--all-targets -- -D warnings`；`cargo fmt --all -- --check` | 均退出 0；[Clippy](../validation/docs-ssg-p1/code-review-r4/clippy.log)、[格式](../validation/docs-ssg-p1/code-review-r4/fmt.log) |
| docs-dev，显式传当前二进制，单 project | 15/15；含 content/static 附件仅恢复真实目标、无关页保持、CSS 解码与实体、HTML template、错误/panic/重启及 axe；[日志](../validation/docs-ssg-p1/code-review-r4/docs-dev.log) |
| docs-subpath，单 project，与其他浏览器运行串行 | 24/24；含页头/导航/404、主题、复制、分组、窄屏、无 JS、百分号标题、SVG/class 资源、对比与 axe；[日志](../validation/docs-ssg-p1/code-review-r4/docs-subpath.log) |
| 原 production.cjs 指向当前二进制及隔离输出 | 根路径与 `/docs/` 共 20 个 CSS 资源请求全为 200，红/蓝 SVG 字节正确；HTML template 锚点负例拒绝并保全旧产物；console/pageerror/失败请求为 0；[结果](../validation/docs-ssg-p1/code-review-r4/production/production-results.json) |
| [reproduce.py](../validation/docs-ssg-p1/code-review-r4/reproduce.py)，传隔离 base 和当前 target | 5 场景双版本构建；3 个 foreign-template 回归、合法 HTML template 对照、ghost 拒绝对照；[结果](../validation/docs-ssg-p1/code-review-r4/candidate-results.json) |
| [candidate-browser.cjs](../validation/docs-ssg-p1/code-review-r4/candidate-browser.cjs) 逐个 preview 隔离产物 | 10 份产物的 namespace/ID 断言通过，基线链接可导航，浏览器错误 0；[结果](../validation/docs-ssg-p1/code-review-r4/candidate-browser-results.json) |

复现方式：从所审 SHA 分别构建两个 docs-cli 二进制；从仓库根运行 `python3 docs/validation/docs-ssg-p1/code-review-r4/reproduce.py <base-bin> <target-bin> <仓外新临时目录>`，再运行 `node docs/validation/docs-ssg-p1/code-review-r4/candidate-browser.cjs <同一临时目录> <target-bin> <base-bin>`。脚本和输入已保存，产品代码未添加实验测试。标准测试全绿没有覆盖 SVG 同名元素与自闭合模板的命名空间边界。

## 计划符合性

| 本次承担范围 | 独立核查 | 状态 |
| --- | --- | --- |
| WORK-F13 / M6 / V-9：删除恢复依赖、错误清除、受影响页刷新 | content/static snapshot 附件仅恢复真实目标即可刷新，无关页保持 | 本轮已核实 |
| WORK-F2、WORK-F14 / M7 / V-4、V-6：CSS 资源解码与最终加载 | 数字实体、URL/url 与 underscore 语义一致，两 base 的 20 个加载正确 | 本轮已核实 |
| WORK-F2 / M3 / V-4：真实锚点、HTML/SVG 内容 | 原 HTML template 幽灵 ID 已排除；foreign 同名元素导致真实锚点误拒（F-20） | 存在缺陷 |
| M5/M8 的修复相关阅读与错误状态 | 39 项标准浏览器回归及生产边界通过，未重签全期真机与性能 | 本轮所测已核实，其余未重验 |
| M9 | 没有纳入本轮实施/修复差异 | 后续范围 |

## 总体结论与修复顺序

验收判定：**需要修改**。

旧项 19/19 在各自原问题范围内已解决，其中 R3 的四个剩余条件本轮确认关闭；新增 F-20 为已证实 P2 回归。因此修复效果 **未完全解决**，声明的修复增量 `patch is incorrect`。198 项 Rust 与 39 项标准浏览器测试通过，不能抵消这一未覆盖的兼容性故障。本轮是局部复审，不提升为 M1–M8 完整重审或 PRD 整期验收。

先校准 template 的命名空间/树边界，再补本报告列出的正反例；关闭条件是三份已保存的 foreign-template 输入成功构建且 ID 与浏览器一致，原 HTML template 幽灵链接仍被拒绝，相关库/CLI/浏览器回归通过。随后独立复审，通过后按计划继续 M9。

下一步：`$apply-code-review docs/plan/DOCS-SSG-P1.code-review.r4.md`；修后 `$review-code docs/plan/DOCS-SSG-P1.md`。本轮只保存报告、证据和派生状态，不执行修复或修改原计划，不登记合并、发布或推送。
