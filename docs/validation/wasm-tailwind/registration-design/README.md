# 初始指令遍历设计与验证

计划：[WASM-TAILWIND](../../../plan/WASM-TAILWIND.md)。本轮针对初始注册指令的顺序、失败前缀与媒体包装；完整M4仍未通过。

- [x] Ground：初始源码执行链与实际WASM RED；当时扩展后45场景/90比较，86失败，含8项首错差异及82项完整图差异，类别重叠，Promise epoch无差异。
- [x] Sketch：前两案收敛同一canonical拓扑，追加真正不同的existing-value/COW overlay方案；三份候选与两轮审查已保存。共享文件系统，非访问隔离。
- [x] Agree：条件性优先验证existing-value结构，领域effect保留赋值时版本；18项cursor/3项反证和5项实际PublicValue存储/2项反证通过，生产采用证据仍不足。
- [ ] Implement：生产Current及有界assign/splice已接入ImportViews与native/public统一初始visitor；utilities/source/media/theme/custom variant/legacy variant及utility capture均共用原始图，生产typed theme/custom variant collector与事后初始theme模拟移除。主注册48场景/96比较、补充媒体4/4与既有完整API/CSS门通过；兼容后functional归一化已在同一Current完成，随后降低deferred bodies；raw theme原值发布也已在该Current执行，函数阶段保留生成声明与keyframe的完成赋值；authored variant按捕获顺序在同一Current执行，保留opaque原body数组；ownership14场景/28比较全绿并加入默认门。函数已前移到同一raw Current，按caller root/keyframe尾部顺序赋值，utility补全在替换前捕获；后期setup函数Edits及失败journal移除。导入前转换、native适配、postcompat单次ownership移交、完整准入/host投影与峰值仍未完成，见[M4](../M4.md)。
- [ ] Scrap：尚未触发；接入产生重复grammar/周期body同步/别名特例时回到Ground/Sketch。

[固定契约和摘要](manifest.json)、[扩展实际RED](../utf16-families-registration-expanded-before-red.log)及[runner](../../../../tests/utility/compiler/run-public-ast-registration-wasm.mjs)保存当前输入与对照。初始36场景/72比较的[原RED](../utf16-families-registration-detailed-before-red.log)仍保留。独立上下文继承当前模型，不作为跨模型比较；judge意外看到映射已披露。

[Ground](ground.md)、[WHY](why.md)、[初始审查](review.md)、[补充独立审查](review-existing-value.md)与[当前合成/接入合同](synthesis.md)提供续做入口；[arena条件稿](synthesis-arena-conditional.md)为历史对照。只用同一领域handler与实际发生点的效果记录，不再事后模拟初始注册语义。

隔离原型是[cursor-prototype.rs](cursor-prototype.rs)，[runner](run-cursor-prototype.mjs)加载固定[上游ast.ts](../../../../ref/tailwindcss/packages/tailwindcss/src/ast.ts)实际walk，与已构建native可执行文件比较18项。依赖仅复用已缓存serde/serde_json；临时Cargo workspace、锁文件和准确命令在[cursor证据](cursor-evidence.json)。[3项反证](cursor-mutations.json)都预期exit 1，恢复后18项全绿；strict Clippy与格式检查通过。原型没有接入生产，也不验证host/Source/Proxy/完整准入或峰值。

[第二存储原型](existing-value-prototype.rs)实际链接本仓PublicValue类型，5项存储检查与2项故意错误见证通过；错误版本预期Rust panic exit 101，[恢复后结果](../utf16-families-registration-existing-value-restored-probe.log)为5项绿。[锁文件/命令/摘要](existing-value-evidence.json)只记录隔离依赖图，没有修改本仓锁文件。它没有完整walker/handlers/host/admission或fresh-clone验收。

[补充媒体探针](run-media-source-probe.mjs)原先2场景/两入口4比较全部RED：source callback替换后Stop继续兄弟，上游给两个utilities赋params，本仓只给第一个。[原始完整图和trace](../utf16-families-registration-media-source-before-red.log)保留对照；当前生产门的相同4比较已通过。原native候选和proposal-2的first-match说法已修正，manifest保留提交前摘要与勘误，不将改过文本冒充原提交。

注册、媒体与[ownership runner](../../../../tests/utility/compiler/run-public-ast-ownership-wasm.mjs)已加入默认`compiler-api`，分别通过96、4和28项比较。authored variant现在按捕获顺序在同一Current逐项赋值，已有variant kernel保留原body opaque slot，自定义模板只在实际clone时读取当前值；原递归typed variant变换移除。postcompat functional归一化与原值theme publication保持；函数与keyframe尾部现已共用raw Current，默认函数顺序19场景/38项通过。下一步收拢apply、完整synthetic root表示和一次lowering交接。M4完整范围、冻结fixture和1398 ledger不变；浏览器产物与结果见[M4](../M4.md)。
