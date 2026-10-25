# Rustify UI 落地页

独立的 Rustify Docs 纯内容站，面向中文 Rust / Web 开发者。主要入口指向
[GitHub Pages 中文文档站](https://rockie.github.io/rustify-ui/zh/)，同时提供
[GitHub 源码](https://github.com/rockie/rustify-ui)与各功能指南。

在仓库根目录使用现有工具链：

```sh
mise exec -- mbx build --locked -p rustify-docs-cli --profile docs-cli
./target/docs-cli/rustify-docs check --config pages/rustify-docs.toml
./target/docs-cli/rustify-docs build --config pages/rustify-docs.toml
./target/docs-cli/rustify-docs preview --config pages/rustify-docs.toml --port 0
```

打开 preview 打印的地址。编辑时可运行
`./target/docs-cli/rustify-docs dev --config pages/rustify-docs.toml --port 0`。
已独立安装的 `rustify-docs` 0.2.1 也可运行相同站点命令。

- `content/index.md`：文案、布局与 utility classes。内容容器通过 utility variant
  设置生成器外壳的样式，未修改框架。
- `theme.json`：完整配色与字体；两个系统模式采用相同的落地页配色。
- `fonts/`：本地 Sora 与 Noto Sans SC 字体及 OFL 许可。中文字体为当前页面与 404 用字的子集；新增汉字时须同步更新子集，来源见 `noto-sans-sc-source.txt`。
- `content/theme-studio.png`：来自仓库真实浏览器验证的截图，图内元数据记录原始路径与提交。
- `.impeccable/direction.md`：本页面的设计方向与验证上下文；本地截图在同目录的 `review/`，由 Git 忽略。

页面采用静态 DOM，不编译站点 Rust、不加载 wasm；没有统计脚本或第三方脚本。

构建输出是 `pages/dist/`。发布前把 `site.url` 设置为本落地页的实际公开地址，
并让 `site.base` 与 URL 路径一致。目前尚无本页的发布地址，检查和构建会保留
`missing-site-url` 提醒，不生成猜测的 canonical 与 sitemap。部署时使用完整产物，
包括资源与 `404.html`。创建本目录没有更改现有文档站的部署。
