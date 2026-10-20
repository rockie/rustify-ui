全部 23 张 R6 与 R3 同名图逐位一致（SHA256 相同），23/23 SAME。评审材料已齐，给出结论。

# R6 U2 搜索加载与恢复 独立评审结论

## 评分表（四维+总分）

| 单元 | 布局 /3 | 视觉 /3 | 交互与状态 /3 | 细节 /1 | 总分 /10 |
|---|---:|---:|---:|---:|---:|
| U2 搜索加载与恢复 | 3.0 | 3.0 | 3.0 | 1.0 | **10.0** |

## 逐维理由（引用截图文件名）

**布局 3.0（已达上限）。** 1280 dark 下对话框宽约 576px（min(36rem, viewport−2rem)），居中偏上，面板内顺序为「Search documentation 标签 + Close → 输入 → 状态/总数 → 列表 → Retry/Show more」，与 S1–S2 一致（U2-loading-1280-dark.png、U2-module-error-1280-dark.png、U2-reopen-02-1280-dark.png）。375 light 下对话框约 343px、左右留白均匀，无横向溢出（U2-loading-375-light.png、U2-module-error-375-light.png）。长列表帧 U2-supp-longlist-04-375-light.png 中列表内部滚动，输入、Close 在顶部、Show more 在底部均在视口内，符合 S2「输入、关闭、恢复与更多按钮在列表外且保持可达」。错误态 Retry 独立于列表区全宽显示（U2-index-error-375-light.png）。所有矩阵状态（loading/module error/module retry/index error/index retry × 两视口主题 + 关闭重开 + 旧响应释放）23 张齐全，无缺格。

**视觉 3.0（已达上限）。** 深色面板为 popover 深蓝、扁平 1px 边界、小圆角、无阴影，页面遮罩可见压暗（U2-loading-1280-dark.png）；输入焦点为蓝色粗环加可见 offset 间隙（U2-loading-1280-dark.png、U2-loading-375-light.png）。浅色选中项为浅蓝底（primary 8% 混入的可见效果），命中词高亮更浓（U2-supp-longlist-04-375-light.png 中 Guide sample 20 选中与 SearchCorpusToken 命中层级可辨；U2-supp-loading-result-03-375-light.png 中 ReachableCustomToken 命中加粗高亮）。Retry、Close、页头 Search 的焦点环均清晰可见（U2-supp-retry-focus-05-375-light.png、U2-supp-close-focus-07-375-light.png、U2-closed-01-1280-dark.png、U2-supp-close-return-08-375-light.png），符合 G2、G3 焦点可见与蓝色用途限定。文字层级（标签 600、状态/摘要小号、标题 600）与 G1 一致。此为截图可见效果判断，非全部 token 数值的测量认证。

**交互与状态 3.0（已达上限）。** 实测记录与截图互证：module 未返回即同步打开并聚焦输入（U2-supp-open-before-module-01-375-light.png，S3）；loading 期间可编辑查询且不丢（U2-supp-loading-edit-02-375-light.png → U2-supp-loading-result-03-375-light.png，S4）；module 与 index 失败均显示「Search is unavailable. Please try again.」+ Retry，不冒充 empty（U2-module-error-1280-dark.png、U2-index-error-375-light.png，S4）；Retry 保留查询、重聚输入并显示当前结果，鼠标与键盘两路均证（U2-module-retry-1280-dark.png、U2-index-retry-375-light.png、U2-supp-keyboard-retry-06-375-light.png，S4）；Esc 关闭归焦页头 Search（U2-closed-01-1280-dark.png）、Close 按钮关闭归焦（U2-supp-close-return-08-375-light.png）、重开保留查询（U2-reopen-02-1280-dark.png，S5）；旧响应释放后当前查询不受污染（U2-late-response-03-1280-dark.png、U2-supp-new-result-before-old-09-1280-dark.png、U2-supp-old-released-after-new-10-1280-dark.png，S5）；19 次 ArrowDown 至首批末项选择正确（U2-supp-longlist-04-375-light.png，S3）。干预仅数据层（扣留响应/首次 503/schema_version 99/Retry 真实恢复），符合 U2 比较口径。

**细节 1.0（已达上限）。** 单复数「1 result」/「98 results」正确（U2-reopen-02-1280-dark.png、U2-supp-longlist-04-375-light.png）；loading 与 error 文案区分且 loading 明示可继续输入；标题路径「Custom search home › Custom section」（U2-late-response-03-1280-dark.png）；375 下快捷键提示隐藏但保留可读「Search」按钮名（U2-supp-close-return-08-375-light.png，S1）；对话框无第二个 H1。规范模式下未发现可扣分细节。

## 与 R3 对照（回归/无回归）

- 23 张 R6 截图与 r3/ 同名图逐张目检一致，并经 SHA256 核验 **23/23 逐位相同**：无任何回归。
- R3 评审维持关闭状态的 UI-02（面板/输入/恢复按钮完整与长列表可达）、UI-04（加载改词、键盘 Retry、Close/Esc 归焦、重开保留查询、晚响应不污染）本轮证据完整，未见复发。
- R3 无未关闭差距，本轮亦无新增。
- 附注：逐位一致与 capture.md 的「确定性产物（build_id 与 R2–R5 逐位一致）、本轮实拍、无旧图复用」声明自洽；本轮 capture.json 内部亦存在同态同 hash（module-error 与 index-error 相同），符合该管线特征。

## 已确认差距（或「无」）

**无。** 没有新增编号，可挽回分合计 0.0，与 10 − 10.0 = 0.0 一致。

## 待核实

1. **截图新鲜度（流程层面，非 UI 差距）**：R6 与 R3 全部 23 张逐位相同。采集声明为「确定性产物下的本轮实拍」，该解释自洽；但可独立佐证新鲜度的材料（采集日志、browser-raw.jsonl）不在我的白名单内，我无法独立复核「本轮实拍」本身。不影响评分，留主 agent 结合禁读材料核验。

## 未证清单

以下从本轮截图与操作记录中看不到，不计成功或失败，不扣分：

- **IME 组合输入**：无组合输入期间/结束后的记录，S5 的 IME 约束未验证（R3 同）。
- **错误态改词后再 Retry**：本轮恢复记录均保留原 CustomOnlyToken；capture.md 提及 keyboard-checks.json 覆盖此项但该文件禁读，我的材料内无独立证据（R3 同）。
- **完整 Tab/Shift+Tab 循环边界**：仅见 输入→Retry、输入→Close（Shift+Tab）、Esc 归焦三段。
- **hover/pressed 与过渡过程**：静态帧不能证明。
- **实体手机软键盘与动态视口**：375×812 为 CSS 视口。
- **辅助技术朗读**：可见总数与选中不等于朗读；U4 本轮整体 missing，不评分。
- **精确实现属性**：字体来源、token 数值、dvh 单位、遮罩精确 30% 等仅按可见效果判断。
- **错误文案与截屏时刻的时序**（如 250ms 等待是否足以暴露污染）只能依据操作记录文字，截图本身不携带时序证明。

## 建议（不扣分）

1. 后续轮次可补两帧以关闭长期未证项：错误状态下修改查询后再 Retry 的结果帧；IME 组合输入期间与结束帧。
2. 鉴于确定性产物下逐位重拍信息量有限，可在 capture.md 中说明「逐位相同即预期」的判定标准，或在逐图清单中标注与上轮 hash 相同，省却评审逐张比对的歧义。
