# DOCS-SSG-P4 · 代码评审 R1

## 评审对象

| 轮次 | 开发计划 | 审查范围 | 上轮报告 |
| --- | --- | --- | --- |
| R1 · 初审 | [DOCS-SSG-P4 v2](DOCS-SSG-P4.md) | `77ca4f47ad099d1b162cf5b6190cc0b49d775da0..06be2ee3f0db519a7316b0d53d57c94c7ee46fd1` 的累计净差异；另核对初始工作树指纹 `4d4abf9fbdd977d65e5fa49ab74b1345d16025385621dcd6c803c7a5446b4bc8` | 无 |

- 评审 ID：`9cd6da78-0bc0-4297-9f6b-6fffde099e72`。
- 日期：2026-10-16T09:52:26+08:00；仓库：`/Users/rockie/Documents/gh-rockie/rustify-ui`。
- 观察 checkout：`main / a6c076a7ec669aede7ef2ff019ee79cdbbdb102a / dirty`。所审产品、配置和测试文件与目标提交逐字节一致。
- 计划版本：工作树 v2，SHA-256 `94dc01c9d1e6db07f68fa276278c0a40a5abfb031950b1fd884d458f8714990f`。读取的完成记录为 M1–M6 已完成、M7 阻塞；本轮不把尚未交付的发布门作为已完成范围。
- 呈现路径：DOM。构建期 Rust 生成 HTML、搜索数据和 AI 索引；浏览器原生 dialog/module/Worker 提供搜索。不涉及 GPU 实现。
- 本轮由主 agent 独立完成，没有委派 reviewer。代码及原计划只读，仅新增报告和 SDLC 评审登记。

## 确认问题

未发现可确认的问题。新增 findings：0；没有需交给 `apply-code-review` 的 P0–P3 项。

核查了公开策略与语言投影、最终 DOM 提取、章节链接、匹配与排序、分页、高亮、安全 URL、IME、焦点、异步会话与重试、同代产物，以及固定 R/C 和 Pages 预检接线。没有把测试通过当作无未知缺陷的保证。

## 待核实项与范围限制

1. **M7 的发布门仍未完成。** [M7 记录](DOCS-SSG-P4.records/M7.md)和计划 §9.3 保留原生稳定浏览器、两实体手机、VoiceOver、原生 200% 缩放、公共 HTTPS Git、Pages 配置、线上全页/资源及实际回滚证据。本轮未执行这些检查，也没有发布、推送或变更 Pages。它们属于已声明的后续阶段，不是本轮确认的代码缺陷；不能凭本报告关闭 M7 或整期 PRD 验收。
2. **全量回归与正式性能没有在本轮重跑。** 本轮实际重跑范围见下表；根路径、纯 meta CSP、官方全部演示、其他引擎和完整工具链等历史结果仅按 [M6 记录](DOCS-SSG-P4.records/M6.md)及其实际版本引用，不改称本轮执行。未把 Playwright 的引擎结果代替原生稳定版。
3. **本轮临时浏览器证据仅在当前机器。** 截图和测试输出保留于 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p4-review-6tagycdl/browser-results/`；未提交共享。报告保留精确源码范围与可复跑命令，临时目录不是长期共享证据。
4. **未提交项没有新增产品代码。** 初始 dirty 内容均为 SDLC、计划/报告、M7 调查记录；没有将无关 P3 文档评审列入代码 findings。报告与本次登记不属于本轮审查输入。

## 覆盖与验证

### 意图与 Git 范围

用户请求为 `$review-code docs/plan/DOCS-SSG-P4.md`。按开发计划模式，审查登记的离线搜索、AI 索引、官方双语及交付验证实现，固定累计基线如下：

| 顺序 | 登记提交 | 职责 |
| --- | --- | --- |
| 1 | `fecc230cad5136c4c3f5cd008fedfe48a8f90f3b` | 初始搜索、AI、中文与验证实现 |
| 2 | `96a739c9f9d8f3e4b8214f648d14de2bd3c31bbd` | CLI 0.1.22 配对 |
| 3 | `41b19b63b88eba0503a49e291f4a27edd9d2107f` | 搜索焦点与选中文字对比度 |
| 4 | `d0d84d7b60b46b542994479cadb7c11da1e0c28a` | CLI 0.1.23 配对 |
| 5 | `a2ad3940fd8dc81e25d9b2c4e857f481d95e8cbd` | 关闭/重开会话生命周期 |
| 6 | `18b229d9b915a798f561dad14e83d7f90ad7d1a3` | CLI 0.1.24 配对 |
| 7 | `a13a27935887490372b37e34a9fa59ca328991e7` | SDK 焦点尝试及浏览器测试调整 |
| 8 | `14269ed72b20bc9ec3cdd6787d16e8dfb2109d7b` | CLI 0.1.25 配对 |
| 9 | `58b7a0d9eb913b7656120693905d66d70ab208d8` | 撤回 SDK 尝试，收敛引擎测试 |
| 10 | `936149b91f57f6dbbdb43969c8843f940348a11e` | 最终 C₅ / CLI 0.1.26 |
| 11 | `06be2ee3f0db519a7316b0d53d57c94c7ee46fd1` | 验收、设计同步及官方性能探针语言修正 |

以上提交均解析并验证为当前 HEAD 的祖先，区间内无未登记产品提交；按最终净效果裁定，不把已撤回的中间实现报成当前缺陷。`a6c076a7ec669aede7ef2ff019ee79cdbbdb102a` 只更新计划的提交登记，排除代码审查。

初始 index diff 为空，其 SHA-256 为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`；tracked 工作树 patch SHA-256 为 `28e109c0f2922f5bc649d74de4f163b61435ebf1b614d255c58baaf00f728441`。工作树指纹对初始 tracked dirty 和 untracked 中 24 个文件的 `path → SHA-256(bytes)` 映射按键排序，以紧凑 JSON 的 UTF-8 字节再次 SHA-256。逐文件指纹、初始 patch 和清单保存在浏览器证据目录的上一层：`/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p4-review-6tagycdl/review-identity.json`。报告写入前已复核输入指纹及 HEAD，无外部变化。

### 文件与行为覆盖

累计差异共 190 个文件。下列 55 个产品、测试、配置和说明文件作为代码/契约审查面；其余为计划、SDLC、设计评审及生成的 JSON/截图证据，按相关记录和摘要核对，没有逐字重审全部大附件。

```text
.github/workflows/docs-pages.yml
Cargo.lock
DESIGN.docs-ssg.md
README.md
crates/rustify-docs-cli/Cargo.toml
crates/rustify-docs-cli/assets/components-Cargo.lock
crates/rustify-docs-cli/assets/framework-revision.txt
crates/rustify-docs-cli/src/rendering.rs
crates/rustify-docs-cli/tests/search.rs
crates/rustify-docs-macros/tests/registry.rs
crates/rustify-docs-syntax/src/lib.rs
crates/rustify-docs-syntax/src/manifest.rs
crates/rustify-docs-syntax/tests/registry.rs
crates/rustify-docs/assets/docs.css
crates/rustify-docs/assets/search-worker.js
crates/rustify-docs/assets/search.js
crates/rustify-docs/assets/site.js
crates/rustify-docs/src/ai.rs
crates/rustify-docs/src/classes.rs
crates/rustify-docs/src/content.rs
crates/rustify-docs/src/frontmatter.rs
crates/rustify-docs/src/generation.rs
crates/rustify-docs/src/labels.rs
crates/rustify-docs/src/lib.rs
crates/rustify-docs/src/search.rs
crates/rustify-docs/src/shell.rs
crates/rustify-docs/tests/ai.rs
crates/rustify-docs/tests/content_include.rs
crates/rustify-docs/tests/generate.rs
crates/rustify-docs/tests/i18n.rs
crates/rustify-docs/tests/markdown_intro.rs
crates/rustify-docs/tests/registered_generation.rs
crates/rustify-docs/tests/search.rs
docs/docs-ssg.md
docs/zh/quickstart.md
docs/validation/docs-ssg-p2/m6/measure-official-islands.mjs
playwright.docs.config.ts
rustify-docs.toml
scripts/docs-pages-verify.py
site/src/lib.rs
site/src/pages.rs
tests/docs-ssg/browser.ts
tests/docs-ssg/custom.spec.ts
tests/docs-ssg/islands.spec.ts
tests/docs-ssg/official.spec.ts
tests/docs-ssg/search-perf.spec.ts
tests/docs-ssg/search-worker.spec.ts
tests/docs-ssg/search.spec.ts
tests/docs-ssg/serve.sh
tests/docs-ssg/test_pages_verify.py
xtask/src/docs.rs
xtask/src/docs_corpus.rs
xtask/src/docs_perf.rs
xtask/src/docs_search.rs
xtask/src/main.rs
```

必要上下游另读取了语言投影/导航、最终 HTML 检查、URL 编码、注册宏展开、消费站配对、预览/静态服务及 SDK 浮层的相关实现。保留路径在配置/内容分析阶段处理；最终文件仍由同一 generation/publish 链交付。搜索和 AI 的缺译地址语义分别核对，没有把两者合并解释。

### 本轮实际执行

| 命令/检查 | 目标与结果 |
| --- | --- |
| `mbx test --locked -p rustify-docs --test search --test ai` | 当前产品树与目标一致；搜索 12/12、AI 8/8，通过 |
| `mbx test --locked -p rustify-docs-cli --test search` | CLI 3/3，通过；覆盖旧 dist 保护、合法空首页与官方双语来源/allowlist |
| `mbx test --locked -p rustify-docs-syntax -p rustify-docs-macros` | syntax 36/36 + doctest 1/1，macros 2/2，通过；涵盖布尔元数据、旧 manifest 与宏/冷读一致性 |
| `python3 -m unittest discover -s tests/docs-ssg -p test_pages_verify.py -v` | 5/5，通过；使用隔离临时目录及本地随机端口，没有访问 GitHub 或部署 |
| 下方 Playwright 命令 | 固定安装的 C₅，`docs-search-subpath` 的 Worker 7 项及 UI 21 项，共 28/28，通过（报告总时长 29.8s）；所用产物 build_id 为 `e92daa8a0efb7328` |
| 产物来源反核 | 本轮 dist 的 site/search/search-worker 三脚本与所审源码字节一致；en/zh 索引各 100 页；R₅→C₅ 恰为五个配对文件 |

浏览器执行入口为 `http://127.0.0.1:4205/rustify-ui/`，经仓库原有服务脚本重建和 preview；不是 Portal App。实际覆盖首查/热查、排序与词边界、翻页、首次 module/JSON/Worker 故障及超时、关闭重开竞态、过期消息、组合输入、中文真译/回退、深 404、禁脚本、注入反例、键盘、浅深主题、320/375/1280px 与文本 reflow。套件核查 console、pageerror、CSP、失败请求和外源请求；预期的故障注入按具体测试排除。另实际打开本轮 1280px 浅色和 320px 深色截图核查内容与控件可见性；未据此声称独立 UI/原生设备验收。

最初浏览器启动因子进程选到缺少 `tomllib` 的 Python 而失败；固定已安装的 Python 3.14.0 后通过。使用下列一次性环境，没有安装依赖或修改工具链配置。Git remap 仅用于本地固定来源消费，不算公共 HTTPS 安装证明。

```sh
MISE_AUTO_INSTALL=false \
RUSTIFY_DOCS_BIN=/Users/rockie/Documents/gh-rockie/rustify-ui/target/docs-ssg/p4/install-936149b9/bin/rustify-docs \
CARGO_NET_GIT_FETCH_WITH_CLI=true \
GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0=url.file:///Users/rockie/Documents/gh-rockie/rustify-ui.insteadOf \
GIT_CONFIG_VALUE_0=https://github.com/rockie/rustify-ui \
mise exec python@3.14.0 -- node node_modules/@playwright/test/cli.js test \
  -c playwright.docs.config.ts --project=docs-search-subpath \
  search-worker.spec.ts search.spec.ts \
  --output=/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p4-review-6tagycdl/browser-results \
  --reporter=line
```

本轮合计 95 项定向检查通过，未重跑整仓 build、Clippy、全部浏览器项目或正式性能。

### 已有证据的核对

- [C₅ identity](../validation/docs-ssg-p4/m6/artifacts/c5/identity.json)与[本地预检](../validation/docs-ssg-p4/m6/artifacts/c5/preflight.json)指向同一 C₅/R₅，CLI 0.1.26，build_id `ae467c9c97900d12`，97 文件/76 HTML/2 搜索索引/2 AI 索引。本轮读取并核对记录，未将其称为线上部署。
- [正式搜索报告](../validation/docs-ssg-p4/m6/performance/search-c5.json)保存 C₅ 身份、4 个语言/入口场景、6 类查询、每类 5 冷/5 热样本，共 240 样本；各查询中位数最大冷 164ms、热 49.4ms，热态请求为空。核对了计时实现从 input 到对应结果 ID 的 DOM 更新及两个 rAF；这些数值是原 M6 实测，不是本轮重新量测。
- 当前净差异中 SDK overlay 源码没有保留中间尝试；测试的引擎差异处理不修改产品 CSP 或结果语义。官方性能探针的实际修改仅将硬编码英文改为非空文档语言与组件环境语言一致。

## 计划符合性

本次只对已登记实现作代码裁定，不重新设计计划，也不回写里程碑完成状态。

| 本期承诺 | 实现与证据 | 本轮判定 |
| --- | --- | --- |
| M1 / WORK-F9 AC-4/5、WORK-N7，V-1 | search/content/generation；真实分析/生成测试，注册布尔声明及默认草稿、缺译、独立开关、保留路径 | 已核实 |
| M2 / WORK-F9 AC-1/2/4，V-2 | 生产 Worker；中文整词、英文词首/大小写、UTF-16、联合章节匹配、标题优先、稳定分组分页及 100 页真语料 | 已核实 |
| M3 / WORK-F9 AC-3/6/7，V-3/4 | shell/site/search；静态入口、当前语言 404、无脚本、输入焦点、IME、失败重试及会话隔离 | 已核实 |
| M4 / WORK-F12，V-5 | ai/generation；导航去重/非导航补齐、来源地址、描述兜底、排除、确定性与旧代保护 | 已核实 |
| M5 / WORK-F16 AC-1/2/4，V-6 | 官方配置、HomeZh、唯一中文 quickstart；CLI 来源测试与实际源核对，官方全路径走查按 M6 历史范围保留 | 本地实现已核实；未重跑本轮官方全站 UI |
| M6 / V-7–V-10 | 固定五文件配对、真实 C₅ 浏览器构建、性能/发布预检工具；历史安装/全量/正式性能证据按原版本识别 | 本地代码无确认缺陷；未扩称全部检查本轮重跑 |
| M7 / V-11/12 | 计划与 M7 记录均明确阻塞；预检脚本有本地协议测试 | 后续范围；原生/公共发布门未关闭 |

## 总体结论与下一步

**验收判定：通过验收（本轮 M1–M6 本地代码范围）。**

正确性结论：`patch is correct`。本次净变更中没有发现有充分证据、值得作者修复的缺陷；95 项定向检查及固定 C₅ 的实际浏览器结果支持这一判断。该结论不表示 M7、整期 PRD、独立 UI 验收、公共安装或发布已经通过。

无需启动空的修复轮。M7 按原计划补齐环境与公开交付证据；若需要独立 UI/UX 验收，可运行 `$review-ui docs/plan/DOCS-SSG-P4.md` 并明确只验当前本地范围。整期完成后，可再运行 `$review-prd-dev-gaps docs/plan/DOCS-SSG-P4.md` 核对 PRD 覆盖。原计划、源码和已有历史报告不改。

