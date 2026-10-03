# Rustify UI

<!-- impeccable:product-schema 1 -->

## Platform

web

## Product Purpose

Rustify UI 是实验性的 Rust UI SDK：在同一个浏览器页面和 wasm 实例中组合 Leptos DOM 与 Makepad WebGL2，由应用统一拥有状态，通过 props 和 typed actions 与 GPU region 通信。现有能力和运行边界以 README.md、docs/architecture.md 为准。

## Capabilities and Constraints

- 主题系统在 DOM 与 GPU 之间共享语义值，保留多作用域隔离能力。
- 2026-10-03 用户确认：新增基于 Tailwind CSS v4 的主题系统及 demo site，尽量完整复刻本地 `ref/tweakcn-main` 的编辑体验，第一期同时覆盖 DOM 与 Makepad GPU。
- 同日确认：本期完成本地编辑体验；AI 生成、账号云端保存、社区发布和外部网站预览留待后续。
- 现有技术栈为 Rust、Leptos CSR、Makepad WebGL2 和 Tailwind 独立 CLI；工具版本以 mise.toml 为准。浏览器运行产物采用静态托管。
- 字体资源、GPU 呈现差异、导入格式和兼容策略由对应开发计划明确，不把浏览器 CSS 支持等同于 GPU 支持。

## Operating Context

现有仓库包含组件目录、属性工作台、数据工作台和 Vellum 等示例。主题编辑站面向选择预设、调整主题、查看真实组件和导出可用于应用的主题这一工作流程。目标使用者推定为采用或评估 Rustify UI 的开发者与设计人员。

## Brand Commitments

产品名为 Rustify UI。新主题编辑站参考 tweakcn 的编辑布局和交互，不复制其产品身份、在线服务或商业承诺。

## Evidence on Hand

- README.md：产品定位、开发入口和运行限制。
- crates/rustify-ui/src/theme.rs：现有主题、作用域与局部覆盖。
- crates/rustify-components/css/sdk.css：Tailwind v4 集成与作用域 token。
- ref/tweakcn-main：用户指定的本地参考，属于 gitignored 调查资料，不能成为生产构建依赖。
- docs/plan/THEME-STUDIO.md：本期主题系统与 demo site 的设计、范围和验收契约。
