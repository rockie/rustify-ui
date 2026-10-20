# DOCS-SSG-P4 · UI 验收目标包

- 需求来源：DOCS-SSG PRD 的 P4；计划 DOCS-SSG-P4。
- 首轮采用规范模式。用户没有提供目标图，不把历史实现截图当作设计目标。
- 范围为 DOM 静态阅读站与原生搜索对话框；不验收 GPU、AI 索引文本格式、安装工具链或发布流程本身。

## 单元

| 单元 | 状态与采集矩阵 | 目标 | 比较口径 |
| --- | --- | --- | --- |
| U1 搜索主路径 | idle、英文 results、empty × 1280×900 / 375×812 / 320×812 × light / dark；1280 light 追加快捷键、键盘选择、更多、Esc 归焦、定制页锚点导航；375 dark 追加中文真译文与缺译结果；root/meta 各追加 1280 light 结果 | S1–S6、G1–G4 | 规范模式；固定 100 页同构语料用于视觉与交互，不替代 1000 页性能验收。示例正文与排名中无关条目不作像素比较；语言提示、高亮、分组、总数、选中与导航必须一致 |
| U2 搜索加载与恢复 | loading、module error、module retry success、index error、index retry success × 1280×900 dark / 375×812 light；1280 dark 追加关闭与重开，以及旧响应释放后的当前查询 | S1–S5、G1–G3 | 规范模式；仅通过请求延迟、HTTP 失败或错误数据到达状态，不替换界面或脚本；输入与恢复操作必须可达 |
| U3 官方双语阅读与入口 | 英文首页、中文首页、中文 quickstart × 1280×900 / 375×812 / 320×812 × light / dark；1280 light / 320 dark 追加缺译页和中文深 404 搜索；375 light 追加禁脚本 quickstart 与深 404；375 light 追加语言菜单与导航展开 | O1–O4、G1–G4、S6 | 规范模式；中文首页与 quickstart 为真实译文，其他页面允许英文正文并明确缺译。代码和宽表可内部滚动；页面不得横滚；不比较系统字体跨 OS 的栅格差异 |
| U4 原生设备与辅助技术 | 当前稳定桌面 Chrome / Firefox / Safari；iOS Safari 与 Android Chrome 的阅读、导航、搜索、主题和软键盘；原生 200% 缩放；VoiceOver + Safari 的结果数量、当前选择及复制反馈 | G3–G4、S2–S5、O3–O4 | 只认可操作的原生环境、设备截图及实际朗读/操作记录。Playwright 视口、引擎和 DOM 属性不代填；无资源不打分 |

## 规范要点

### G · 共同阅读世界

- G1：保留既有 Read 世界，文档列主导，系统 sans/mono 字体，无 webfont；正文 1rem/1.75，导航不超过 0.875rem/600，正文宽 36rem，框架宽 76rem；面板圆角 0.375rem，扁平 1px 边界，无生产阴影。出处：DESIGN.docs-ssg.md 的 Overview、Typography、Layout、Shapes；计划 §6。
- G2：浅色背景 #f9fafb、正文 #1d2939、次级文字 #667085、popover #ffffff、primary/ring #175cd3；深色分别 #0c111d、#f9fafb、#98a2b3、#1d2939、#53b1fd。control border #858f9e / #717c8e，code-bg #f2f4f7 / #161b26。蓝色只用于链接、当前位置、焦点、搜索选择与命中；选中为 primary 8% 混入 popover、命中为 20%，正常前景文字。出处：DESIGN.docs-ssg.md token 与 Colors。
- G3：完全键盘可用，焦点始终可见；WCAG 2.2 AA，普通文字 4.5:1、大字和必需非文本指示 3:1；读取、导航和交互都有明确状态。关闭浮层回到真实触发点。VoiceOver 实际朗读结果数、当前选择和复制成功/失败。出处：PRD WORK-N5/AC-1、AC-2、AC-4、AC-6；计划 §5.2、§9.3。
- G4：1280px 三栏，768–1279px 两栏，更窄为单栏并收起侧栏/目录；顶栏至少 3.5rem，窄屏可增高。320px 与原生 200% 缩放没有横向页面滚动，代码和宽表只在自身容器内滚动。减少动态效果时不平滑滚动、不过渡。出处：DESIGN.docs-ssg.md Layout；PRD WORK-F8/AC-2、WORK-N5/AC-5；UAT-12。

### S · 搜索

- S1：每类页面都有具名搜索按钮，位于语言/主题旁；桌面有 Cmd/Ctrl+K 提示，48rem 以下隐藏快捷键提示但保留可读按钮名。按钮为 1px border、面板圆角、0.8125rem 字号、0.375rem 0.5rem 内边距。原生 dialog 宽 min(36rem, viewport − 2rem)，最大高 dynamic viewport − 2rem，桌面 padding 1.25rem、窄屏 1rem，背景为 popover，遮罩 foreground 30%，无第二个 H1。出处：DESIGN.docs-ssg.md Local search；计划 §6。
- S2：对话框顺序是可见输入标签与 Close、输入、状态、live 总数、结果列表、Retry/Show more。输入为 background/foreground，3px ring 与 2px offset；状态、路径、摘要 0.875rem/1.5，标题 600。只有结果列表 flex 并内部滚动，上限 55dvh；输入、关闭、恢复与更多按钮在列表外且保持可达，长文字换行。出处：DESIGN.docs-ssg.md Local search；计划 §6。
- S3：点击或 Cmd/Ctrl+K 同步打开并聚焦输入，不等待 module/Worker/索引；上下键选择、Enter 普通导航至标题锚点、Tab 可达关闭/重试/更多，Esc 关闭并归焦。结果按页面分组，有标题、标题路径、高亮摘要；标题优先于小标题/正文。首次 20 组，每次 Show more 增加 20，live 总数为全部匹配页面数，保留当前选择。出处：PRD WORK-F9/AC-1、AC-3、AC-5；计划 §2.2、§5.2；DESIGN.docs-ssg.md Local search。
- S4：区分 closed、idle、loading、results、empty、error；idle 提示输入，loading 说明可继续输入且不丢查询；empty 明确无结果并建议换词或缩短前缀、输入保持焦点；error 显示暂不可用及 Retry，不冒充 empty。Retry 保留查询、重新聚焦输入并成功显示当前查询结果。出处：PRD WORK-F9/AC-2、AC-7、UAT-9；计划 §5.1、§6。
- S5：Close/Esc 回到实际触发点，保留查询以便重开；普通结果激活含同页锚点会关闭对话框。关闭放弃旧会话，快速重开与晚响应不得污染新查询；module 首次加载失败可同页恢复。IME 组合输入结束前不触发查询/选择。出处：DESIGN.docs-ssg.md Local search；计划 §2.3、§5.1。
- S6：搜索限定当前界面语言及部署 base；中文真实页显示译文，缺译页使用英文源文且明确“尚未翻译”/源语言，标题旁有目标语种 badge，激活后仍在中文路径。源标题/路径/摘要有真实 source lang，控件和数量文案为当前界面语言。404 也能搜索并导航。禁脚本显示有说明的 disabled 搜索入口，正文与导航仍可读。出处：PRD WORK-F9/AC-4、AC-6，WORK-N3/AC-5，UAT-5/6/9；计划 §5.1、§6。

### O · 官方站

- O1：中英首页保持相同信息结构：Rustify UI 主标题、事实性一句定位、快速开始/组件两个普通主次操作、DOM 组件/可选 GPU/静态部署三个轻分隔 feature；中文首页为真实中文。首页无侧栏，hero max 44rem、tagline max 36rem，标题 clamp(2.25rem,1.6rem + 2.6vw,3.25rem)，tagline 1.1875rem/1.55；操作高 2.75rem，圆角 8px。出处：surface brief 的 P2/P4 extension；DESIGN.docs-ssg.md Layout、Home hero；计划 §5.4。
- O2：中文 quickstart 使用真实中文源，保留源章节 ID；系统字体与阅读列、代码面板和导航延续既有规范；命令与参数保持原文语义。出处：DESIGN.docs-ssg.md Chinese official reading pages；计划 §5.4。
- O3：缺译为 H1 后、正文前的安静 note，code-bg、secondary 边界、6px 圆角，0.75rem 1rem padding、0.875rem/1.6；不作加载或错误。界面中文，正文及源标题为英文；导航/页脚/搜索保持当前语言。语言菜单使用原生 details 和普通链接，窄屏距右 1rem，内部滚动、长词换行；键盘 Escape 关闭归焦。出处：DESIGN.docs-ssg.md Languages and untranslated pages；PRD UAT-6。
- O4：深 404 保留文档框架、明确标题、当前语种回首页与语言首页链接，搜索使用已选语种。单错误页静态托管禁脚本时允许默认英文 404 与各语言首页链接；内容页禁脚本正文、普通导航、代码及演示初始内容仍可读。出处：DESIGN.docs-ssg.md Localized not-found pages；PRD WORK-N3/AC-5、UAT-6/12；计划 §6、§9.3。

## 采集

- 计划：`docs/plan/DOCS-SSG-P4.md`；规范原文位于仓根 `DESIGN.docs-ssg.md`、`.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md`、`docs/PRD-DOCS-SSG.md`。评审只读取上文摘录，不沿来源读取代码、旧验收结论或完整报告。
- 主入口：官方站 `http://127.0.0.1:4199/rustify-ui/`；产品搜索专用真实生成语料站 `http://127.0.0.1:4205/rustify-ui/`。按仓库 serve.sh 构建；R2 固定实装 CLI C 1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a / R 740610db164b3f396b0fcc551f5a5ef8f15da716（0.1.27）。当前相关产品路径与该 C 无 diff；HEAD/dirty 指纹及实际 build_id 另存本轮 capture.md，R1 身份保留在原轮目录。
- Playwright 1.63.0 Chromium 153.0.8010.12，DPR 1；桌面/窄屏均为真实浏览器渲染，窄屏只是 CSS 视口，不冒称实体手机。额外连接现有 Chrome 扩展。等待字体与网络完成；默认 reduce motion、无网络/CPU节流；显式主题通过现有主题按钮设定。
- 首轮无外部设计稿，比较已确认的设计规则与需求；没有新的视觉风格假设。UI 验收不关闭计划 M7 的发布、真实设备与辅助技术缺证。
- loading 延迟真实索引响应；module error 仅让首次模块请求返回 503；index error 仅返回 schema 不兼容数据，Retry 恢复真实请求。所有干预在逐图清单注明。
- 原生稳定矩阵、两手机、原生200%、VoiceOver若无法访问，逐格标 missing，不用引擎/视口模拟代替。

## 变更记录

- 2026-10-16：建立首轮目标包，来源为上述既有设计规范、surface brief、PRD 与 P4 计划。未提供像素目标；首次规范基线待用户确认，后续若调整期望则重建基线。
- 2026-10-16：用户确认“按现有规范继续（推荐）”。单元与规范要点未变化，目标指纹不变。
- 2026-10-16 R2：仅更新采集节的固定 CLI/框架身份；规范、单元与比较口径不变。补充原生 Chrome 200% 阅读与搜索关键帧，其余 U4 缺口在 R2 capture.md 单列。
