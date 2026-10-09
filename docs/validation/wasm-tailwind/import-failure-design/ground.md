# Import failure Ground

固定目标：[WASM-TAILWIND](../../../plan/WASM-TAILWIND.md)的完整M4契约不变。当前公开调用仍是`new AstCompiler(input, config)`；Native PublicAst为不可变Arc快照，Facade按journal操作真实JS数组。当前失败载体可重放前缀并拒绝额度，不发布半个compiler。

## How

调用链：WASM public_api.rs:487读取图 → compile_ast.rs:1591转换native节点/Origin → mod.rs:208 → imports.rs:350 Bundle.read。当前asset lookup在imports.rs:407，begin在414–418；DFS递归成功才complete，任何错误使后续兄弟未访问。ImportGraph保存初始替换与完成事件；实际parsed节点绑定fresh公共Origin和Source，view更新使用原父数组句柄，Facade预检后Reflect.set/splice。

参考at-import.ts:25–79同步walk：参数解析 → 启动loader IIFE →原数组替换为空context →跳过该节点；walk结束后才await Promise.all。参数错误会停止同步walk；loader/parser错误在异步任务里，不停止同步兄弟walk。已经启动的任务不取消，可能在顶层compileAst拒绝后继续赋nodes。parseCss在index.ts:149等待导入；compileAst在:709等待parseCss。

真实WASM四项RED见[当前重现](../utf16-families-import-failures-before-red.log)：single/first missing没有替换；last/nested missing缺第二条替换。只挪begin不足以修复整个调度。

## Why与约束

Git ls-files/log目标均空，模块及记录尚未跟踪，无本地commit/PR能证明原始作者为何选择DFS。现有理由直接来自M4、代码注释和固定参考；不能把实现便利推断成用户要求。

保留：虚拟bundle/规范路径/base，不引入真实I/O；public每次parse产生fresh节点和tracked Source，src/dst tuple独立并共享Source；native地址缓存strong owner保活/计费；实际原数组/嵌套handle，原import.src包装，无第二次Facade parse；成功完整图、CSS、quota和函数失败门。

改变：同步walk与异步完成应分别表达，错误发生不删除已启动兄弟工作；prefix仅到调用方观察拒绝的边界，后续工作必须有明确所有者。不能以固定depth分支、单次DFS或“全部成功任务先完成”伪装Promise时序。

外部工具映射：Git/GitHub源码控制、SPMS工单与Pages文档能力可用，但本模块无commit/issue/Page定位，外部原始意图未知，未把个人文档或工单猜为本代码动机。chat/infra/error/analytics未发现匹配工具。本次理由调查仅本仓事实与记录，没有外部写入。

## Horizon反证

[固定oracle实验](../utf16-families-import-failures-horizon-oracle.log)使用现有async立即完成的虚拟loader，queueMicrotask marker标记epoch；这些数字是观测，不是生产delay常量。root缺资源在epoch6被await调用方观察：leaf outer.nodes在2、单层chain在6，均先于捕获；两层chain在10、三层在14，发生在捕获之后。七层chain更深loader本身也在捕获之后运行。成功chain3+leaf时，leaf先填nodes，和输入顺序无关。同步第二条参数错误epoch3捕获，先前leaf在2仍完成。

实验已反驳“所有兄弟成功任务都在拒绝前完成”。不得把post-error mutation并到prefix。当前同步Facade入口如何保留后续完成/释放handle、预算完整预留与原host错误对未启动工作影响需要设计；真实外部异步loader不在当前assets入口中，完整API仍是目标中的待办。
