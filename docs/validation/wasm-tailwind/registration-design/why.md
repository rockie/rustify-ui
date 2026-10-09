# 初始注册所有权重构：理由与语义约束

## The Question

用 current canonical graph 替换初始 native `Vec` 分轮处理及事后公开投影，受到哪些明确理由和语义约束限制？现有材料支持保留行为合同；尚不支持认定某一种表示必然正确或性能更好。

## The Code in Question

锚点：`compiler/mod.rs:208` 的 `Compiler::new_with_ast`、`origin.rs:115` 的 `Edits` / `:425` 的 `Projection`、`public_api/compile_ast.rs:633` 的 `theme_input_mutations` / `:1216` 的 `Origins::convert`。前者操作 typed 树，后两者连接 caller 身份、current字段、日志和公开图；这些是当前机制，不是历史作者意图。

## What We Found — Direct

- **[Direct] 当前不可变公共快照与 host effects 分离有明确本地依据。** [既有设计](../ast-input-mutation-design.md)写道：“WASM 通过明确的变更记录操作真实 JS 对象，无需把 Rust 的公共值迁移为共享可变对象。”同文 `:59–61` 的取舍为避免两份候选事实及额外表示转换；从输出重建输入会丢内部节点、数组操作和原对象身份。
- **[Direct] 表示选择是条件性的。** [历史综合](../custom-origin-design/synthesis.md)使用“条件性实验基础”和“暂不采用”；`:57`进一步要求 current view、可达快照与峰值合同不能满足时：“无法做到这些就废弃结构，不加 CSS/位置启发式。”这不是禁止后续 canonical graph 的永久约束。
- **[Direct] 刷新有双重要求：共享 occurrence读取最新值，同时不覆盖其他 typed变换。** `origin.rs:160`：“Refresh only after an explicit edit in this phase; preserve other typed transforms.”历史综合 `:57`明确要求在 guard前解析 current view并同步 typed字段，重绑 origin本身不够。
- **[Direct] caller live身份与私有克隆别名不能混淆。** `origin.rs:425`：“Canonical current views keep private cloned array aliases together.”下一行明确 caller metadata保留 host references并单独回放效果。既有设计 `:51`解释 live handle可避免后续 metadata将已修改 caller数组恢复成旧快照。
- **[Direct] 构造变更是可观察事件。** 既有设计 `:37`要求先发布空 theme rule，再赋最终 children，以保留 setter/splice抛错前的写入；历史综合 `:51`要求 normalization文本相同也赋值。`:53`规定语义失败精确前缀、额度拒绝不写 caller、host先抛错则返回host错误、不发布半个compiler。

## What We Can Reasonably Infer — Inferred

**[Inferred] 一个共享 current owner可能降低 typed字段、公开views和事后重扫分歧的风险。** 当前 setup按指令类型提取，公开 theme操作在结果后重扫；计划仍要求补早期失败及跨阶段shared/Proxy行为（`WASM-TAILWIND.md:40`）。若同一图能同时表达原身份、当前字段和当前数组位置，似乎更容易在实际操作点记录事件。但这只是实现推断：没有证据证明必须引入新图类型，也没有测量证明更快、更省内存。保留 typed结构并修正同步亦不能仅凭现状排除。

## Constraints

重构须保留全部已表示字段、真正缺少src的状态、Source/metadata别名，以及普通Clone共享与fresh clone独立身份；不得以CSS、位置或Source span替代节点身份（历史综合 `:7`、`:9`、`:55`）。canonical身份不等于强迫所有版本共享可变storage，COW/current view仍须正确区分live caller与私有克隆。

事件须指向原数组/对象，保留同值赋值、首错和成功前缀；theme空替换、normalization、最终theme、variants、functions、apply、utilities的阶段顺序仍有效（历史综合 `:51–53`）。真实Set及原receiver的splice不能用最终值diff替代。WASM保存并返回原`JsValue`异常（`public_api/setup.rs:33`、`:58`），不能包装成新语义错误。

导入Program/Machine仍拥有reaction与已启动任务；root失败不是取消或回滚。`compile_ast/setup.rs:321`明确：“Remaining owned jobs may still run after an error”。setter捕获的新context即使抛错仍须保留handle（WASM `graph.rs:181`）。拒绝admission前不能执行compiler AST写入；此合同不保证getter没有用户副作用。新表示须覆盖完整潜在日志、handles、选定根可达版本和实际峰值，不以现有保守逻辑费用冒充allocator峰值（历史综合 `:57`；`origin.rs:340–341`）。

## What We Don't Know — Unknown

三个锚点均未跟踪；本次`git status`为`??`，`git log`及`git ls-files`均空，审阅目标commit为0。最初为何选typed Vec/COW、作者是谁、是否有私人讨论与外部源码消费者，仍未知。现有测试断言不证明完整早期注册、动态Proxy、统一峰值或性能已经通过；计划仍是3/7（`:37`）。本次未执行测试、构建或基准。

## Sources Consulted

读取[源码调查报告](why-source.md)，完整epistemics/synthesizer规则，抽查上述原文、当前代码与本地Git。长文来源为既有mutation设计、custom-origin综合和计划；未读本轮候选。GitHub/Spaces能力可用，但无本地未发布实现对应PR、ticket或Page引用，未进行无关联账户搜索。团队聊天、可观测性、外部异常追踪及分析仓库按调查报告/父任务能力映射无对应MCP，未搜索；未作外部调用。

## Confidence Summary

当前本地表示取舍及行为合同有直接文本依据；canonical graph的适用性属于待验证推断。没有证据恢复初始作者动机，也没有证据选择或验收新所有权表示。
