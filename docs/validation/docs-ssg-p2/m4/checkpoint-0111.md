# M4 · 组件工程与 Rust 随改随看

计划：[DOCS-SSG-P2](../../plan/DOCS-SSG-P2.md) · 更新：2026-10-14T01:11:53+08:00 · 状态：进行中。基线：dirty@2d6b0a73ad887769c0bad209acabd47382659df5；M3 核心恢复副本见 [accepted-core-sources.json](m3/accepted-core-sources.json)。

采用 DOM 阅读面与显式交互岛。CLI patch 更新至 0.1.7；未新增 GPU 呈现或服务。工作树未到 M6 冻结提交点。

| 覆盖范围 | 当前结果 | 证据 |
| --- | --- | --- |
| components 模板 | Counter/函数演示、两指南、固定 site_docs 库名、host/wasm 入口、作者 README；包名归一化、工具链先检、原子写入与无配对拒绝已接。真实完整锁已内嵌；工作树expectedR仍空，拒绝未配对组件工程。模板源Cargo.toml.in避开Git发现未展开manifest | templates/components、new.rs、new_components.rs；[new 单元](m4/new-unit-r2.log)、[锁r2](m4/template-lock-r2.log) |
| 普通内容路径与新建边界 | dev 14/14、content new 6/6、components 工具链/拒绝路径 5/5 | [第三次集成](m4/dev-new-integration-r3.log)。第一次11个旧启动等待假设失败；第二次剩1个building重复旧reload信号；原日志均保留 |
| 编译复用前提 | 全轮输入守卫和编译输入指纹分开；普通内容可重绑本轮读，Rust/manifest/lock/include/build输入/环境及源码切片仍受编译证据保护。无rerun-if的构建脚本保留全包输入 | [编译输入21例](m4/compile-cache-unit-r1.log)；构建脚本规则以 [Cargo Book](https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection) 为依据 |
| 符号链接编译身份 | 反例：A.rs/B.rs都存在时，module.rs换目标而canonical文件集合不变。现在指纹另记录alias→real，编译输入21/21绿；目录匹配预先建祖先集合 | [反例](m4/compile-alias-red-r1.log)、[修复21/21](m4/compile-alias-green-r1.log) |
| dev 构建状态 | 先watch/listen/打印地址和首次静态building shell，再构建；host/wasm/render状态与真实epoch/两目标结果经SSE传递。状态sequence与产物revision分开；旧Good整套保留，失败不换，stale丢弃并排下一轮 | dev.rs、rust_site.rs、dev.js/dev.css；CLI68单元 [初验](m4/cli-unit-r1.log)、[receipt接线后复验](m4/cli-unit-r2.log)与[严格all-target clippy](m4/cli-clippy-r1.log)通过 |
| 外部监听与恢复 | 实际注册metadata/path-dependency/build输入目录；缺失声明依赖从Cargo TOML提取并监听最近存在祖先。target/输出/私有区过滤，监听与缓存分离 | 初接代码已在rust_site.rs/dev.rs；真实外部依赖、连续保存恢复用例尚未运行 |
| 实际渲染读守卫 | 原四条反例先红，原read receipt补全后9/9绿：outside/include/config/符号链接、缓存命中与存在性-only；dev及build/check最终守卫已共用 | [四条0/4](m4/render-guard-red-r1.log)、[九条9/9](m4/render-guard-green-r1.log)；已改content.rs/markdown.rs；[Docs完整238/238](m4/docs-full-r1.log)通过 |
| 真正Rust dev路径 | 首次building、真实两目标、Markdown/config/theme零Cargo、失败保留、行列、修复/lock不变1/1通过 | rust_dev.rs；[首跑1/1](m4/rust-dev-r1.log)；后续receipt补全后再做受影响复验 |

M3 的 compiled generation 在缓存命中时保持，表示同一 Rust/源码/wasm 对；本轮 receipt 更新配置与内容守卫，完整站点文件仍按实际字节生成 build_id。这样内容保存不会仅因身份字段改变而刷新无关岛页。缓存轮不伪造新编译完成时刻。

M4 设计扩展已写入原 dev surface brief，保留原面板/中性色/字体和收起交互；新增首次构建页、旧版上building标记及多行诊断。浏览器两路径已通过，九图已交独立finish review；尚未收到设计结论，不扩大为全部M4验收。

浏览器更新：[r1失败原记录](m4/browser-r1/)、[第一完整路径r2](m4/browser-r2/)、[Rust路径r3](m4/browser-r3/)。冷态首候选因补齐构建输入而stale丢弃，重试两目标仍串行；测试改为实际进程start/end与epoch绑定。旧ready早于新编译完成的等待、诊断文件组strict定位和合法warning marker的断言按真实状态修正，源码/节点/新步进/资源/滚动断言保留。

临时配对：Rtest23bb0d88fe61ff96b9e1e3b93de69266b9fa6483、Ctest733201474914cf40a934729bef23f7ae6017bab7；[真实锁r2](m4/template-lock-r2.log)、[Git安装CLI0.1.7](m4/template-git-install-r1.log)通过。C相对R仅改CLI四文件；不是M6公开冻结。安装后new/README/未改模板与独立性能仍在串行验证。早期模板非法包名原日志保留，源改.in后该错误消失。

继续动作：完成已安装模板/README试走、外部依赖/连续保存、完整保存性能及设计收口；docs/docs-ssg.md已修订待主线核对。M6真人/实体手机缺口、公开候选分发和Pages授权仍保留。
