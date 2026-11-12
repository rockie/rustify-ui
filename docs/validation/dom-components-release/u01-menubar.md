# U-01 · V-13 T2 Menubar 键盘补证

- 日期：2026-11-11，Asia/Shanghai。
- 关联：[代码初审 R1 / U-01](../../plan/DOM-COMPONENTS-RELEASE.code-review.md#u-01--补齐-v-13-t2-的-menubar-键盘真实入口证据)、[M4 记录](../../plan/DOM-COMPONENTS-RELEASE.records/M4.md)。
- 仓库 HEAD：`921802b3c9d1022033bd500681b870a430652616`；本轮仅写证据与台账，未修改 SDK、指南或发布包。
- 结果：**补证通过**。线上 ApplicationShell 原样用于 registry 0.3.0 独立 DOM 项目，Menubar 键盘打开、切换、命令执行及焦点返回全部完成。原代码评审结论保留，整体验收待复审。

## 环境与真实入口

从 [线上 Menus 指南](https://rockie.github.io/rustify-ui/menus/) 的 “Compose an application shell” 提取完整 51 行 Rust 代码，未改组件、状态或回调；按 [线上 DOM quickstart](https://rockie.github.io/rustify-ui/quickstart/#a-dom-application-without-gpu-regions) 加入普通 cdylib 的 mounted scope。包装层仅提供 `my_app_mount`，保存 `AppHandle`，页面先调用 SDK `boot` 再调用挂载导出。

独立项目与原始证据保存于：

```text
~/Documents/rustify-releases/crates-io-0.3.0-20261110/v13-t2-u01/
  ApplicationShell.rs        线上原始代码
  menus.html / quickstart.html
  consumer/                 manifest、Cargo.lock、src、public 构建产物
  build.log / export.log
  browser.mjs / browser.json / browser.stderr
```

项目以新目录、新 CARGO_HOME 和干净 Chromium context 启动。CARGO_HOME 无 config、源替换或 patch；仅 registry 缓存共享本机标准缓存，不使用上轮 T2 项目或 staging registry。Cargo 首次正常解析生成独立锁。依赖：`rustify-ui = { package = "rustify-dom", version = "=0.3.0" }`、`rustify-components = "=0.3.0"`、Leptos `=0.8.22` CSR、wasm-bindgen `=0.2.128`、js-sys/web-sys `=0.3.105`。锁中四个 Rustify 包（colors/components/dom/style）均为 registry 0.3.0，无 Makepad 包。

macOS arm64，mise Rust 1.99.0、mbx 1.15.0、Node 26、已安装的 Playwright Chromium。未新装工具或启动容器。

## 构建与访问

以下命令在仓外 consumer 目录执行，PATH 前置 `~/.rustup/toolchains/1.99.0-aarch64-apple-darwin/bin`，CARGO_HOME 指向上述新目录：

```sh
mbx generate-lockfile
RUSTFLAGS="--cfg=web_sys_unstable_apis" mbx build --locked --release --target wasm32-unknown-unknown --lib
wasm-bindgen target/wasm32-unknown-unknown/release/menubar_consumer.wasm --target web --out-name app --out-dir public/pkg
mbx run --locked --bin export-dom-assets
```

均 exit 0；wasm release 构建 1m22s，wasm-bindgen 实际报告 `0.2.128`。资产导出程序来自 quickstart 链接的消费示例，编译所用依赖与 wasm 同锁。用 Python 3.12 标准静态服务在 `127.0.0.1:59457` 提供 `public/`，浏览器通过该页面实际加载 wasm、loader 和 CSS。没有注入 UI 状态或直接调用菜单内部函数。

## 键盘步骤与可见结果

| 操作 | 实际焦点 | 可见菜单 / 应用结果 |
| --- | --- | --- |
| Tab | File | 初始状态 Ready，菜单关闭 |
| ArrowDown | Save | File 展开，Save 可见 |
| ArrowRight | Undo | 切换到 Edit，File 关闭、Edit 展开 |
| Enter | Edit | Undo 执行，状态变为 Undone，菜单关闭 |
| ArrowLeft，然后 Enter | Save | 返回 File 并展开 Save |
| Enter | File | Save 执行，状态变为 Saved，菜单关闭 |

Playwright 检查实际 `document.activeElement`、菜单可见性、`aria-expanded` 与应用 status 文本，随后读取并核对执行日志。页面错误、console error、失败请求和 HTTP ≥400 响应均为 0。亲自查看下列截图，确认 Edit/Undo 菜单与最终 Saved / File 焦点均可见。

![键盘切换到 Edit，Undo 获焦](u01-edit-open.png)

![执行 Save 后显示 Saved，焦点返回 File](u01-saved.png)

## 结论与边界

U-01 所缺的指南示例真实键盘路径现已取得直接证据；本轮未重现上轮手工组装的空渲染，未对那次组装失败追认具体根因。本轮结果不依赖 CATALOG menu、Data table 或 Tree 的替代证据。

未扩大到 GPU、其他指南、性能或完整 UI 矩阵。原始项目、锁、构建产物和日志作为本次可复核证据保留；浏览器与静态服务器均已停止，无新 worktree、分支或容器。
