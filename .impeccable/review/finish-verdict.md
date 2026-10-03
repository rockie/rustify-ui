## verdict

1. **resolved** — 上轮唯一 material fix（GPU 标题、运行状态与资源回退/恢复说明跟随编辑器语言）：已逐张打开同路径的新截图 `docs/validation/theme-studio/M6/screens/desktop-zh-dark-preview.png`、`tablet-zh-dark-preview.png`、`mobile-zh-dark-preview.png`。桌面明确显示「GPU 主题样本」「已就绪」，与中文编辑器一致；平板和手机截图有效、中文预览布局完整，但 GPU 位于截图视口之外，不将这两张图声称为 GPU 文案的直接证据。`examples/theme-studio/src/gpu_preview.rs` 显式接收 locale signal，标题、全部 RegionState 文案及资源失败恢复说明均经同一个翻译闭包读取；`app.rs:480` 传入当前编辑器的 `state.locale()`。`tests/browser/theme-acceptance.spec.ts:225` 的三个尺寸用例断言中文标题与就绪状态，`:307` / `:309` 断言中文与英文 GPU 恢复提示，`:335` / `:337` 断言英文与中文缺字体恢复说明；`docs/validation/theme-studio/M6/studio-final.log` 的 acceptance 1–8 全部通过，补足状态与视口外文案的行为证据。

## remaining

clear。本轮证据未显示此修复引入的视觉回归；ship 仅覆盖上述已评分修复，不代表重新审计整个界面或宣告完整回归通过。当前 `studio-final.log` 仍记录全套 57 passed / 1 failed（导出用例等待启动状态超时），其复跑与归因由主线程另行收口。本轮未运行浏览器、构建或 detector。

disposition: ship
