# Anonymous design judgment

裁决：选择 **proposal-2 作为实施基线**；proposal-1 的异步驱动需修订后才能竞争。此结论仅是设计草图评审，不是可执行验证或生产一致性认证；两份方案均未实施，完整 M4 目标不变。

## 阅读与校准

已完整阅读 Ground、proposal-2，再完整阅读 proposal-1；未查看候选身份、作者映射或其他候选文件。首次合并读取被截断后，重新按 2 → 1 顺序读取全文。

合格方案必须从实际 loader await、nested await、Promise.all member reaction、scope await 与调用方 await 推出：leaf 和 chain1 在 missing rejection 被观察前完成，chain2 在其后完成；必须保留 begin setter 抛错前已启动的当前 task，并在首个 host write 前预检整个有限生命周期。串行 DFS、先完成所有成功兄弟、按深度设置固定 delay，任一种都直接失败，不能由接口或文档质量补偿。Ground 已给出 DFS 的失败记录；本次未重跑这些实验。

## 核心比较

| 维度 | proposal-2 | proposal-1 |
| --- | --- | --- |
| 导入因果 | “Exact reaction rules”明确区分 settled Promise 注册 reaction 与 pending Promise settlement，按注册顺序入队；表格从实际边推导 leaf、chain1、chain2 的边界。通过草图检查。 | `PropagationEdge`、registration 与 activation ticket 可以表达同一因果，未使用 depth 常量。通过原生模型草图检查；缺少与异步桥一致的具体投递契约。 |
| host 反馈 | 规则 1–2：先启动、再替换；walk 抛错不创建 aggregate。规则 5：complete setter 失败只拒绝该 task。Admission 段明确实际执行重新走状态转换，不能无条件重放 dry-run suffix。 | `SettlementCell` 后的段落明确当前 task 在 begin 抛错后存活；`drive_turn` 接收写入失败且不清空 frontier。方向正确。 |
| 有限预算与归属 | “Admission and host failures”要求 dry run 到 quiescence，覆盖 detached suffix、seed/working overlap、完整 journal、strong owners、host handles、callback captures；quota 在写前裁决。尚待真实容量边界证明。 | “Admission boundary”覆盖所有 host-write-success 可激活分支和 tail，正确承认可能先报 Limit；overlap、回调容量与具体计费仍需补齐。 |
| 实际 JS microtask | `post` 段明确“each specific Rust continuation at its creation time”，禁止串行 pump；规则 8 明确异步入口不挂 synthetic observer，由真实 caller reaction 定义 catch。这是决定性优势。 | `GraphCursor` 后的驱动段规定“每次 Yield 真正 queueMicrotask 后再驱动一 turn”，且在 `Published(Err)` 时 reject；此前 `drive_turn` 把 publication 放在 `ObserveCaller`。照此实现，会串行投递已同时 ready 的任务，还可能把模拟 observer 与真实 await 叠加。必须修订。 |

proposal-1 的异步缺陷有直接推理反例：同一同步 walk 已产生两个 ready loader reactions A、B，调用方随后排入 marker。正确队列应为 A、B、marker；若只投递一个 pump，执行 A 后才投递下一 turn，则变成 A、marker、B。内部 ticket 即使保持 A 先于 B，也无法修复与外部 marker 的相对次序。这是依据其驱动文字的反例推演，未执行新实验。若作者原意是在 reaction 激活时分别投递全部 callback，需要把该契约写明并重新评审。

## 接口、复杂度与红旗

两者都保留普通 native `Result` 和同步构造器，以新增 Promise 入口承担真实异步时序；未把 scheduler phases 暴露给普通调用方。host acknowledgement 确实改变控制流，属于必要边界，不是 pass-through。模块按 import 语义、公共 AST identity、host crossing 分工，没有按 load/validate/finish 时间阶段拆模块。JS 仅持有原始错误与安排 microtask，导入和错误选择仍在 Rust，未发现已明确写出的策略泄漏。

proposal-2 的 Promise 子集虽然有较多 ID，但每种 reaction 有直接上游因果；proposal-1 同时持有 event DAG、edge、registration、ticket 与 frontier，目前未展示足以抵消复杂度的实现收益。proposal-1 对 imperative queue 必然导致 caller 理解 cancellation 的批评没有可执行依据；proposal-2 已把这些知识藏在 session 内。选择 proposal-2 不依赖这一类架构标签。

## 实施门槛与结论边界

先验证真实 reaction traces，再接 WASM：覆盖 leaf/chain1/chain2、输入顺序互换、同步参数失败、直接/嵌套 parser failure、begin setter 捕获 context 后抛错及其存活 task、complete setter 原错误身份、catch 后 tail 与外部 microtask interleaving。额度验证必须包含失败后后缀超额时零写入、重叠所有权和 quiescence 后释放；成功路径也要覆盖 compatibility await。未观察 rejection、任意延迟 loader 的契约及全 M4 其余项仍是未完成工作，不能由当前有限 bundle 设计替代。

顺序敏感性检查：我在比较完成后进行了**心智逆序复核**，把 proposal-1 视为先读基线，仍会因其逐 turn pump 和 publication 边界选择 proposal-2。此步骤不是第二次独立盲审或实验。选择信心来自明确的调度契约，执行正确性仍未知；未给没有实验支持的分数。若合成 DAG 与 reaction queue 的混合方案，须作为新设计重新评审，不能自动继承本裁决。
