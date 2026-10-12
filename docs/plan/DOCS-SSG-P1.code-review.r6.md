# DOCS-SSG-P1 · 代码评审 R6

验收判定：**通过验收（本轮修复复审范围）**。R5/F-21 已解决，F-01–F-20 保持关闭，未发现可确认的新增问题。修复效果 **已解决**，所审修复增量 `patch is correct`。M9 尚未完成，本轮不代表整期交付完成。

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R6 | [DOCS-SSG-P1](DOCS-SSG-P1.md)，对象 `526c88c7-a4b3-4ea9-9193-371420b66d1d` | `3069107cf4990e15db6ad3ddddddd520bcc86838..aabcd95810d4036bd4203aea87a0934c8d50f603`；修复 `998ee180e339ede6793c548bf4f035e8cbb9dd1f`；输入指纹 `53d45ab55298907969fe4791a28e46c2f7c3eb2f0c6bbcff260578228ac556ba` | [R5](DOCS-SSG-P1.code-review.r5.md)，对象 `53350840-75ac-44ea-82e1-3b8072575e78` |

本轮对象 ID：`717fad1d-d311-4cf4-9afa-738cb191966f`。模式为**修复效果复审**：逐项核对 R5 的全部 21 项问题及其直接相关回归；没有重新完整审查 M1–M8，M9 排除。

上轮所审 `540462a895a48385761f8db6e3271a714957d818..3069107cf4990e15db6ad3ddddddd520bcc86838`，报告 sha256 `e5becdb4941c1cda3434feb0ee905b38979e7e9212224764826a43ef5a52e340`。本轮从其 target 起算；当前代码与 `998ee18` 相同，`aabcd95` 只登记计划进度。已有处置见 [M3](../validation/docs-ssg-p1/M3.md#评审修复) 与 [M8 · R5 修复后复验](../validation/docs-ssg-p1/M8.md#r5-修复后复验)。代码、原计划和历史报告保持只读。

## 确认问题

### 上轮问题复审

来源身份沿用 [R1](DOCS-SSG-P1.code-review.md)/F-01–F-14、[R2](DOCS-SSG-P1.code-review.r2.md)/F-15–F-17、[R3](DOCS-SSG-P1.code-review.r3.md)/F-18–F-19、[R4](DOCS-SSG-P1.code-review.r4.md)/F-20、R5/F-21。前 20 项在 R5 已关闭；本轮核对其相关代码是否变化，并运行相关现有回归。下表的“已解决”保留原问题的关闭范围，不扩大真机或整期验收范围。

| 来源 / ID | 上轮问题与结论 | 已有处置 / 修后版本 | 本轮结果 | 当前证据 / 验证 | 剩余动作 |
| --- | --- | --- | --- | --- | --- |
| R1 / F-01 | 输入真实路径出界；R5 已解决 | `12113ab` / M7 | **已解决** | real_source/read_source 授权边界未改；库/CLI 出界 symlink、资源读取和旧产物保全回归通过 | 无 |
| R1 / F-02 | 私有来源经别名/品牌发布；R5 已解决 | `12113ab` / M7 | **已解决** | 别名与真实路径的 private 校验未改；库/CLI 私有文件、品牌、include 拒绝回归通过 | 无 |
| R1 / F-03 | 私有 Markdown 被当页面；R5 已解决 | `12113ab` / M7 | **已解决** | 遍历过滤未改；private_markdown_is_never_a_page 及 CLI 私有发布测试通过 | 无 |
| R1 / F-04 | 顶栏/社交入口缺失；R5 已解决 | `12113ab` / M5 | **已解决** | 阅读壳未改；docs-subpath 页头、导航、内容/首页/404、窄屏与无脚本回归通过 | 原 Safari/手机页头复走限制保留，本轮未重验 |
| R1 / F-05 | 仅首页操作引用的附件未发布；R5 已解决 | `12113ab` / M4 | **已解决** | 操作/页头资源图未改；库与 CLI 首页附件发布测试通过 | 无 |
| R1 / F-06 | CSS 资源缺校验/改写；R5 已解决 | `12113ab`、`a297dbd` / M7 | **已解决** | CSS 资源校验与生成路径未改；库/CLI 正负例、docs-dev/docs-subpath 的 CSS 资源回归通过 | 无 |
| R1 / F-07 | URL 后缀丢失；R5 已解决 | `12113ab` / M4 | **已解决** | URL 分支未改；CLI 附件及 query/fragment 回归通过 | 无 |
| R1 / F-08 | 无换行开栏 panic；R5 已解决 | `12113ab` / M3 | **已解决** | frontmatter 保护未改；库/CLI 单独开栏诊断测试通过 | 无 |
| R1 / F-09 | 忽略实际 --config；R5 已解决 | `12113ab` / M4 | **已解决** | check/dev 指定配置回归通过；本轮独立项目均明确指定配置 | 无 |
| R1 / F-10 | 空站成功替换旧产物；R5 已解决 | `12113ab` / M4 | **已解决** | no-pages 闸门未改；空站、全草稿与输出保全回归通过 | 无 |
| R1 / F-11 | 前置发布失败无 JSON；R5 已解决 | `12113ab` / M4 | **已解决** | 输出归属/锁分支未改；a_refused_output_is_reported_in_json_as_well 及相关 CLI 测试通过 | 无 |
| R1 / F-12 | 百分号标题中断脚本；R5 已解决 | `12113ab` / M5 | **已解决** | 交互脚本未改；docs-subpath 百分号标题、目录与窄屏菜单回归通过 | 无 |
| R1 / F-13 | 附件失效后丢恢复监听；R5 已解决 | `016110f` / M6 | **已解决** | 安全恢复依赖登记未改；CLI 与 docs-dev 的真实目标、content/static 附件仅恢复目标即可清错/更新，无关页稳定 | 无 |
| R1 / F-14 | 未输出脚注贡献幽灵 ID；R5 已解决 | `12113ab`、`a297dbd` / M3 | **已解决** | 本轮改变 ID 构树后，合法脚注、重复引用、未使用脚注的同页/跨页负例仍通过 | 无 |
| R2 / F-15 | 解码后拆问号误伤文件名；R5 已解决 | `a297dbd` / M4 | **已解决** | 先拆后缀再解码未改；CLI 编码问号文件名与后缀回归通过 | 无 |
| R2 / F-16 | 数字实体后等号吞 URL 外分隔符；R5 已解决 | `016110f` / M7 | **已解决** | 实体/span 映射未改；库/CLI 的实体与 CSS 字符串断言、docs-dev 实际资源请求回归通过 | 无；未另重跑历史 20 请求脚本 |
| R2 / F-17 | CSS 文档内 SVG 片段被拒；R5 已解决 | `a297dbd` / M7 | **已解决** | 库/CLI 的合法 SVG 片段与缺目标负例、docs-subpath SVG 资源回归通过 | 未另重测 mask 像素 |
| R3 / F-18 | HTML template 内容误作文档锚点；R5 已解决 | `016110f`、`f7a3e94` / M3 | **已解决** | 原同页/跨页与嵌套模板正负例通过；4 份生产产物确认 holder 可见、ghost/hidden 不在文档树、错误 build 保全旧输出 | 无 |
| R3 / F-19 | CSS 扫描与 compiler 任意值解码不同；R5 已解决 | `016110f` / M7 | **已解决** | CSS 解码分支未改；库/CLI 生成与解码、docs-dev/docs-subpath CSS 浏览器回归通过 | 无 |
| R4 / F-20 | SVG 同名元素误入 HTML template 计数；R5 已解决 | `f7a3e94` / M3、M8 | **已解决** | 库/CLI 的 SVG 自闭合/配对、HTML 内 SVG、foreign integration point 正负例通过；docs-dev 同页/跨页导航与幽灵错误恢复通过 | 无 |
| R5 / F-21 | article 片段解析误拒完整文档的 body ID | `998ee18` / M3、M8 | **已解决** | 独立构建的 R5 binary 在两 base 的 check/build 各报 2 错；target 均成功。BODY.id=target，重复 body 的 ignored 不存在，template 的 ghost/hidden 仍隔离；同页/跨页、拒绝保全及 dev 错误恢复均通过 | 无 |

F-21 关闭的当前代码证据为 [html_check.rs](../../crates/rustify-docs/src/html_check.rs) **153–167** 的完整文档构树及 template 文档后代遍历；[markdown.rs](../../crates/rustify-docs/src/markdown.rs) **1181–1184** 把该集合交给 Fragment；[content.rs](../../crates/rustify-docs/src/content.rs) **1313–1323** 用实际集合校验链接。已读 SSR 的完整 body 和 article 插入入口（[shell.rs](../../crates/rustify-docs/src/shell.rs) **324、370**），没有把作者“已修复”的声明当作关闭证据。缓存 writer 版本 8 进入 FragmentKey，CLI 与 manifest 版本为 0.1.2。

### 本轮新增问题

**未发现可确认的新增问题。** P0–P3 均为 0；没有以风格、假设性防御或后续里程碑缺失补充 findings。

## 待核实项与覆盖限制

没有影响本轮修复裁定的待核实候选。

- 本轮真实浏览器通道是项目已安装的 Playwright Chromium `153.0.8010.12`。1280px/375px 及标准测试中的窄屏为仿真视口；没有重验用户 Chrome、真实 Safari、手机、WebKit 或远端 CI。R1 页头链接真机复走待用户决定的既有限制保留。
- 没有重新执行 SDK/GPU 全矩阵、7 示例/theme-export、500 页性能、LCP/CLS、千次保存或安装演练。只复核了与当前修复代码匹配的既有性能证据；这些不计为本轮执行通过。
- M9 的最终文档、安装 SHA/版本、仓外消费者、安装后二进制 panic 子集和 GitHub 同 SHA 安装仍属后续范围。当前 8/9、Ready 不由这份报告改写。
- 额外文档边界对照中，11 份文档去掉被拒的链接后仅用于观察实际 DOM；它们不是原输入通过 build 的证据，见 context-browser.json 的 viewOnly 字段。

## 覆盖与验证

变更意图：解决 R5/F-21，让正文贡献的 body ID 按完整文档的属性合并语义参与校验，同时保留 HTML template 独立片段及 foreign 内容规则。呈现路径为 **DOM 静态 HTML/SSR**；CLI 在宿主执行，本次无 GPU 或跨区协作改动。由当前主 agent 独立完成全部审查与裁定，未委派 reviewer。

- 仓库根：`/Users/rockie/Documents/gh-rockie/rustify-ui`；审查跨 2026-10-11/12（+08:00），报告保存于 2026-10-12。原输入：`$review-code docs/plan/DOCS-SSG-P1.md`。
- 计划 sha256：`566264d75f2ee58b854dd02f04dbbf63e6922301924d68b3ee631cc256e4e7d4`；计划完成记录的 9 个不同提交均已解析并确认在当前分支可达。前 8 个已经由历史评审覆盖，本轮纳入 `998ee18` 的修复增量；完整 SHA、提交文件及初始工作树清单见 [scope.json](../validation/docs-ssg-p1/code-review-r6/scope.json)。
- 比较方式为固定 base→target 的累计净差异。代码/配置/测试的 7 个变更文件已完整读 diff：Cargo.lock、CLI Cargo.toml、CLI tests/check.rs、docs content.rs/html_check.rs/markdown.rs、tests/docs-ssg/dev.spec.ts。另读修复记录与生成证据作为上下文；`aabcd95` 和历史只改本计划的未登记提交排除代码 findings。没有未登记的代码提交需要扩范围。
- 未提交 tracked/untracked 已枚举，index 为空，当前未提交内容均为既有报告、验证证据、经验与 SDLC 记录，没有待审产品代码。对 615 个相关输入保存“排序路径→sha256”映射，再对紧凑、键排序 JSON 计算整体指纹；包括代码、测试、配置、计划及原 dirty 文件。[scope-end.json](../validation/docs-ssg-p1/code-review-r6/scope-end.json) 确认原输入指纹和 HEAD 均未变化。本轮新报告与证据不纳入输入。
- 静态审查追踪 IDs→Fragment→内容图链接→SSR 完整页、缓存键与 CLI/manifest 版本；现有私有文件与 symlink 授权边界、安全诊断和资源图未因本次修复改变。HTML policy 没有被放宽；依赖锁除 CLI 包版本外不变。
- 基线通过 git archive 从精确 `3069107` 提取到仓外目录，独立 CARGO_TARGET_DIR，以现有工具链执行 offline/locked/docs-cli profile 构建；没有切换、reset 或 stash 用户工作树。[baseline-build.json](../validation/docs-ssg-p1/code-review-r6/baseline-build.json) 保存其 SHA、命令、路径和 binary hash。目标 binary 由当前源码重建，生产对照保存两版 binary hash。
- 浏览器按现有 impeccable Read brief 和既有 DOM 入口走查。标准 projects 及额外 preview 均串行；未运行 Portal 宿主，本项目计划明确没有 Portal App 接入。临时项目只在仓外目录，生产代码与正式测试没有加入评审实验。

| 实际命令 / 步骤 | 本轮结果与证据 |
| --- | --- |
| `mbx test --locked -p rustify-docs` | **134 项通过**：123 单测 + 11 生成集成；[库日志](../validation/docs-ssg-p1/code-review-r6/library-test.log) |
| `mbx test --locked -p rustify-docs-cli` | **73 项通过**；[CLI 日志](../validation/docs-ssg-p1/code-review-r6/cli-test.log) |
| `mbx build --locked -p rustify-docs-cli --profile docs-cli` | 构建通过；[日志](../validation/docs-ssg-p1/code-review-r6/cli-build.log) |
| 两个 docs 包分别 `mbx clippy --locked -p <包> --all-targets -- -D warnings`；`cargo fmt --all -- --check` | 均退出 0；[实际命令](../validation/docs-ssg-p1/code-review-r6/checks.json)、[库 Clippy](../validation/docs-ssg-p1/code-review-r6/docs-clippy.log)、[CLI Clippy](../validation/docs-ssg-p1/code-review-r6/cli-clippy.log)、[格式](../validation/docs-ssg-p1/code-review-r6/fmt.log) |
| 显式传当前 RUSTIFY_DOCS_BIN，`npx --no-install playwright test -c playwright.docs.config.ts --project=docs-dev` | **21/21 通过**，包括两 base body ID、template/foreign、附件恢复、CSS、诊断与 panic 恢复；[日志](../validation/docs-ssg-p1/code-review-r6/docs-dev.log) |
| 同配置 `--project=docs-subpath` | **24/24 通过**，包括页头/导航/404、主题/复制/代码分组、百分号标题、SVG、窄屏/无脚本与 axe；[日志](../validation/docs-ssg-p1/code-review-r6/docs-subpath.log)、[两次实际命令及环境](../validation/docs-ssg-p1/code-review-r6/browser-commands.json) |
| 隔离 base：`mbx build --offline --locked -p rustify-docs-cli --profile docs-cli` | 退出 0；[构建日志](../validation/docs-ssg-p1/code-review-r6/baseline-build.log)、[基线与命令](../validation/docs-ssg-p1/code-review-r6/baseline-build.json) |
| 原 [body-production.cjs](../validation/docs-ssg-p1/code-review-r5-fix/body-production.cjs)，传独立 base binary / 重建 target binary / 新输出目录 | 根/子路径共 **4 份生产产物通过**；修前 check/build 各 2 错，修后成功，BODY/重复 body/template 边界、导航、拒绝后旧树保全、manifest 0.1.2 正确；console/pageerror/失败响应/失败请求均 0。[结果](../validation/docs-ssg-p1/code-review-r6/body-production/body-production.json)、[实际命令](../validation/docs-ssg-p1/code-review-r6/production-command.json) |
| [context-browser.cjs](../validation/docs-ssg-p1/code-review-r6/context-browser.cjs)：9 种输入 × 两版 CLI/Chromium | 共 **18 份 DOM 对照**，其中 11 份为去掉被拒链接的观察版本。frameset 位置、html/body ID、关闭 article/main 后 body ID 等未发现新增误拒或幽灵锚点放行；可构建输入的实际目标与导航一致，console/pageerror/失败请求均 0。[结果](../validation/docs-ssg-p1/code-review-r6/context-browser.json)、[命令](../validation/docs-ssg-p1/code-review-r6/context-command.json) |

额外实验首轮把链接限定在 article 内，关闭 article 的边界输入因此触发定位超时；修正为按可访问名称定位后完成 18 份对照。保留 [首轮日志](../validation/docs-ssg-p1/code-review-r6/context-browser-first.log) 与 [部分结果](../validation/docs-ssg-p1/code-review-r6/context-browser-first.json)，该超时属于实验脚本假设，不作为产品缺陷。

已实际查看本轮 [1280px](../validation/docs-ssg-p1/code-review-r6/body-production/root-reported.png) 与 [375px](../validation/docs-ssg-p1/code-review-r6/body-production/subpath-merged-and-template.png) 截图：正文、页头、导航可读；生产脚本的横向溢出断言通过。其余保存的截图不记为已查看。

性能仅复核原证据：修复者对同 B 的修前/修后完整 build 中位为 **399.9/422.7 ms**，510 个非 manifest 文件逐字节一致；修后冷渲染 **208.0 ms**、热配置 **59.4 ms**。数据见 [原前后对照](../validation/docs-ssg-p1/code-review-r5-fix/performance-comparison.json) 和 [原性能](../validation/docs-ssg-p1/code-review-r5-fix/performance.json)。[本轮匹配记录](../validation/docs-ssg-p1/code-review-r6/prior-evidence-match.json) 确认其登记的 7 个代码/配置/测试输入与当前一致，测量 harness 和 B 输入不在本次 diff；没有把这些数字记为本轮实测或优化收益。

## 计划符合性

| 本次承担范围 | 独立核查 | 状态 |
| --- | --- | --- |
| WORK-F2 / M3 / V-4：真实锚点与 raw HTML | F-21 的 body 属性合并、同页/跨页引用与实际浏览器 ID 一致；脚注、HTML template、SVG/integration point 原正负例保留 | 本轮所测已核实 |
| M4 / V-6/V-10：生产构建与保全 | 两 base 的合法输入成功发布；幽灵引用失败时原产物逐字节保留，manifest 版本匹配 | 本轮所测已核实 |
| M6 / V-9：dev 共享图与恢复 | 两 base body 导航、错误恢复、template/foreign 与附件回归通过；writer 版本进入缓存键 | 本轮所测已核实 |
| WORK-N4 / §4.2、§8：依赖及版本 | Cargo.lock 仅 CLI 0.1.1→0.1.2，无新增依赖/升级；当前及隔离 base 构建通过 | 构建/静态已核实；0.1.2 最终安装交付未验 |
| WORK-N2/N6 / V-14：性能与可靠性 | 复核与当前输入匹配的历史前后性能；本轮 CLI 保全与 dev 恢复通过 | 性能复用原基线；千次保存未重跑 |
| M1–M8 全期与真实设备 | 本轮只复审修复及必要回归，不重新签署整期 UI/GPU/设备/性能出口 | 未重新完整验收 |
| M9 | 不在本轮修复范围内；计划当前 8/9 | 后续范围 |

## 总体结论与下一步

验收判定：**通过验收（本轮修复复审范围）**。

旧问题 **21/21 已解决**，其中 F-21 本轮关闭、前 20 项保持原关闭范围；没有“不再适用”项、未复核的旧问题或新增 finding。修复效果 **已解决**，所声明增量 `patch is correct`。证据包括静态调用链、207 项 Rust 测试、45 项标准 Chromium 回归、两版生产产物及文档边界对照；覆盖限制见上文。

本轮**无需 apply-code-review**。可以按原计划继续 M9，完成最终文档/版本/SHA、仓外安装与消费者验证；远端阶段仍遵循原推送授权门。整期完成后，可选：

```text
$review-prd-dev-gaps docs/plan/DOCS-SSG-P1.md
```

原计划保持 Ready、8/9；只保存本报告、评审证据及派生台账。不执行修复、M9 实施、提交、推送或发布。

