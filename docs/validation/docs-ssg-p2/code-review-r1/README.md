# DOCS-SSG-P2 R1 · 评审证据

完整报告：[DOCS-SSG-P2.code-review.md](../../../plan/DOCS-SSG-P2.code-review.md)。这些是本轮隔离实验与实际日志，未写入产品测试或修改原计划。

## 版本

- 源范围、提交、原工作树/index哈希：`manifest.json`、`commits.json`。
- `identity.json`：实测C4二进制来源/摘要、R4/C4/HEAD三个关键源文件的一致性，以及纯CSR实际依赖节点的feature。
- 实验使用已安装CLI `0.1.14`，C=`d301344eeb27e22907c59a10805ae0cebf8dee99`，R=`3fcc626a4039a43c5e045ccd546bab976db89a73`；没有重装、公开Git安装或线上发布。
- Playwright Chromium `153.0.8010.12`，macOS，本机既有Node/Rust/mbx；实验Cargo设置 `CARGO_NET_OFFLINE=true`、`RUSTIFY_DOCS_CARGO=mbx`。

## 原生验证

`docs-tests.log`、`cli-tests.log`、`components-ssr-tests.log`分别对应报告中的三个实际命令。265、72、124通过；CLI一个性能代理、组件两个doctest按原配置忽略。

## F-01：条件分支的几何规则

`probe-csr.json`是独立CSR的正式复现，`probe-csr-both.png`显示两个分支同时存活时第一条变为满宽，`probe-csr-recovered.png`是关闭第二分支后的恢复。`probe-browser.json`保存最先在hydrate产物中执行SDK mount取得的相同结果。

独立空HTML通过真实preview取得CSS、glue/wasm及DOM loader，设置严格CSP并调用SDK mount；两条Progress分别置于独立Show下，先打开A，再打开B，再关闭B，测实际DOM宽度与CSSOM选择器。每条轨道256px，A应始终128px；实测128/256/128，B为64px。console/pageerror/失败请求均空。

源码保存在 `fixtures/csr/`。为复用CLI的现有wasm编译/bindgen调度，**仅实验工程**把名为hydrate的feature转发到Leptos/SDK/components的csr；`identity.json`保存实际图中的csr，运行入口直接使用mount，没有调用hydrate_site。这个改动不是对产品组件模板支持模式的声明。

## F-02：hydrate专属依赖的清单变化

`probe-dev.json`保存所有SSE状态、三次交互/资源摘要；`probe-dev.log`是CLI stderr。`fixtures/geometry-optional/`保留源码，`optional-initial.toml`与`optional-changed.toml`保留本次唯一目标清单变化。

步骤：

1. C4生成components工程，增加仅由hydrate启用的本地browser-palette包；该包提供两种已有feature对应的类字面量。
2. dev初态为7，点击得到8和旧颜色；源函数与SSR初态保持一致。
3. 仅把依赖清单default改为alternate，等待服务完成保存事件；重新打开并点击，wasm摘要和旧颜色均不变。
4. 只给站点Rust文件追加注释作对照，等待新成功代；重新打开并点击，此时资源与颜色更新。

最初怀疑“专属依赖的CSS完全漏扫”已被实验排除；本报告只裁定清单/缓存失效问题。

## F-03：两个同名包的build输入

`probe-build-input.json`/`.log`是完成状态、浏览器值、资源摘要及CLI日志；`duplicate-packages.json`是metadata中的两份包记录；`build-script-identities.json`是本次实际Cargo JSON中的package_id/out_dir。`fixtures/duplicate/`包含完整最小源工程和两个同名不同版本包。

两个包均声明rerun-if-changed=payload.txt，由build.rs生成常量。v1输入初始7，v2另有自己的输入；站点注册函数引用两者，初态7，点击后8。只把v1输入改成14，观察3秒后状态、wasm和画面仍旧；只追加站点Rust注释后，新的初态14、点击结果15才出现。此实验输出固定class，SSR/接管初态一致。

## 复现脚本与输入保存

四个 `.mjs` 是实际执行的原脚本，原临时目录为 `/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-p2-code-review-wf25bzoq`。它们保留本次CLI路径、临时目录和动态分配端口，属于执行记录；再次运行须把路径/端口调整为新的临时工程，不能直接对仓内fixture操作。

复演时先用同一配对CLI在新临时目录生成components工程，或复制所需 `fixtures/` 子树到临时目录；F-02恢复optional-initial.toml，F-03恢复duplicate-initial.txt到v1 payload.txt。修改副本后由既有Cargo离线取得/保持锁，build或dev；preview用port 0，按新URL运行对应浏览器步骤。不要复制本次私有target/wasm缓存，本目录不携带这些二进制。

本轮所有自有浏览器和preview/dev进程均已退出。原工作树三组输入落盘前重核无变化；这些新证据和评审报告不属于被审输入。官方全页/主题/失败矩阵及真机/公开发布未在本轮重验。
