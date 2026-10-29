# DOM-COMPONENTS-P1-gaps 开发

任务 ID：bba16439-b67d-4fa5-bbff-fddfc2e1317a；计划：[DOM-COMPONENTS-P1-gaps](../../docs/plan/DOM-COMPONENTS-P1-gaps.md)。

## E001 — 浮层重排后再滚动
- 状态：pending
- 线索：缩窄视口后立刻 scrollIntoView 可早于 ResizeObserver 定位，导致按钮视口断言偶发失败。按现有用例先等待对话框位于 viewport 内，保留动作可达断言，专项 3 项通过。
- 定位：tests/browser/theme-export-ui.spec.ts；docs/plan/DOM-COMPONENTS-P1-gaps.records/M2.md。
