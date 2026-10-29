# UI R7 作者自检 · UI-218

- 时间：2026-10-29，Asia/Shanghai；源码基线 `dirty@b218075a3a2107ab511da932f831f42adfa9390c`，main。
- 范围：页面内 DOM Inspector 的未标注选中框。不是浏览器 DevTools 高亮；不修改 GPU、Rust 逻辑、设计目标或评审原文。
- 版本：[初始版本](initial-version.json)、[最终源码与构建身份](current-version.json)、[最小源码 diff](source.patch)。本轮仅删除 `app.css` 中未标注状态的灰色虚线覆盖，复用已有蓝色实线框；未标注面板与文字保持。
- 构建：[日志](build.log)。`mbx xtask build-web --example theme-studio --release` 退出 0；日志有编译警告，本轮未改 Rust。CSS-only 修改的 build_id 仍是 `6d1605ccec8b9f78`；源码与产物 app.css 均为 23897 B、SHA-256 `6396f94bf7f5ad67935ae511fbe6bc83c156b7405400db0fa820f85e837fd976`，三份 HTTP manifest 与本地产物一致，不只凭 build_id 判断新旧。

## 实拍与实测

使用本机已安装的 Playwright 1.63.0 / Chromium 153.0.8010.12，headless，DPR 1、en-US、reduced-motion reduce；从 strict-CSP 的 `http://127.0.0.1:4181/` 进入，每种组合单独空存储 context。原生 viewport 截图，未合成或修饰。

| 组合 | 未标注框 | 已标注框 | 实际行为 |
| --- | --- | --- | --- |
| 修前 1440×900 浅色 | [灰色虚线](before/unmapped-1440-light.png)，`rgb(71,84,103)` / dashed | [原蓝色实线](before/mapped-1440-light.png) | 拾取不激活按钮；Esc 后点击计数增加 |
| 修后 1440×900 浅色 | [蓝色实线](after-confirmation/unmapped-1440-light.png) | [蓝色实线](after-confirmation/mapped-1440-light.png) | 两者边框均 `rgb(21,112,239)` / solid；未标注提示保留 |
| 修后 1440×900 深色 | [蓝色实线](after-confirmation/unmapped-1440-dark.png) | [蓝色实线](after-confirmation/mapped-1440-dark.png) | 作者按钮变为浅蓝，拾取框仍固定 `rgb(21,112,239)` |
| 修后 390×844 浅色 | [蓝色实线](after-confirmation/unmapped-390-light.png) | [蓝色实线](after-confirmation/mapped-390-light.png) | 已标注项点击后切回编辑，再点 Preview 检查框；未标注说明完整可见 |

三种修后组合均验证：Inspect 点击未标注 Create 不增加计数，Esc 隐藏框和面板，随后正常点击变为 Create · 1；已标注项仍展示真实 primary 来源。三个确认 context 的 console error、pageerror、失败请求/HTTP≥400、CSP 事件均为 0。实测完整值、时间与动作见 [确认结果](after-confirmation/results.json)、[脚本](inspect-check.cjs)、[原图尺寸与哈希](images.json)。主 agent 已实际打开桌面浅/深色和窄屏原图核对。

首批修后脚本在 390 已标注项点击后立即检查隐藏的预览，失败；该行为会按现有逻辑切回编辑。修正脚本为点回 Preview 后检查，未再改产品代码。首批 4 张桌面图、未完成的窄屏记录与失败日志保留在 [after](after/results.json)、[browser-check.log](browser-check.log)；确认批次全部通过，见 [browser-confirmation.log](browser-confirmation.log)。修前结果见 [before](before/results.json)。

## 现有回归与范围

```sh
npx playwright test --project=theme-studio theme-ui-review.spec.ts --workers=1 -g 'picking an unannotated|picking a declared|inspector reserves|inspector clips' --reporter=json --output=test-results/ui218
```

4/4 通过，无跳过或 flaky；覆盖未标注动作拦截/退出恢复、已标注来源与边界、面板预留空间、滚动裁切。[JSON 结果](existing-inspector-tests.json)、[stderr](existing-inspector-tests.log)。`git diff --check` 通过。

本轮没有新增测试文件或镜像 CSS 的单测。其他单元沿用 R7 原基线；未重跑整期 V1–V10、子路径、真实性能、VoiceOver、OS IME、其他浏览器或真机。impeccable hook 的确定性检查无问题；提示的 DESIGN sidecar 过期未顺手修改。自检不评分，不将 UI-218 记为独立复核已解决。

手动服务已停止，Playwright 自有服务已随测试结束退出，4181 无监听；[服务记录](services.json)。下一步由用户运行 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`。
