# 本地验证记录

- 范围：`pages/` 的中文落地页；源代码基线 `032756c041615dca075a49685fbb458ce7cc833f`，本页为新增文件。
- 产物：Rustify Docs 0.2.1，build `8ad74c2c572def5d`，base `/`。
- 入口：`http://127.0.0.1:60647/`，使用当前完整生产输出的 preview。

| 检查 | 实际结果 |
| --- | --- |
| `rustify-docs check` / `build` | 通过，1 页与 404；保留 `missing-site-url` 提醒。 |
| `check --external-links` | 8 个外部地址，0 错误；随后只修改字体、对齐与交互样式，链接未变。 |
| Chrome 布局 | 宽度 1440, 390, 320, 768, 1280, 1600, 1334px 均无页面级横向溢出；DOM/GPU 标题顶线差均为 0。 |
| 中文与资源 | `lang=zh`；页面图片和本地字体加载成功；H1 实际使用 Sora 与 Noto Sans SC 自定义字体。 |
| 主入口 | 实际点击“阅读中文文档”，到达 `https://rockie.github.io/rustify-ui/zh/`，标题 Rustify UI。 |
| 键盘与错误页 | 主 CTA 有可见焦点，代码框可键盘横向滚动，不存在的路径返回 HTTP 404。 |
| axe | 1440px 与 390px 的 WCAG 2 A/AA、2.1 AA 所检规则未发现违规。 |
| 动态偏好 | 普通偏好下协作箭头移动 4px；reduce 下移动 0px、过渡 0s；键盘焦点为薄荷绿实线。 |
| 无脚本 / 深色系统偏好 | 中文内容与主入口可见，无页面级横向溢出；落地页保持固定配色。 |
| ui-loop / 视觉收尾 | 首轮发现字体、标题对齐、协作反馈 3 项；修复后复判全部 resolved，ship 仅对应这 3 项修复。 |

原始浏览器结果见 [browser-check.json](review/browser-check.json)，交互结果见
[interaction-check.json](review/interaction-check.json)，视觉复判见
[finish-review.md](review/finish-review.md)。当前桌面、手机首屏与全页截图均在 `review/`，已实际打开检查。

## 本地 CTA 表单增量验证

- 产物：build `be39d29b2f168484`，3 页与 404；沿用上述 preview 入口。
- 采用静态 DOM 与渐进增强脚本，复用仓库现有 `landing-third-party.mjs` 测试接收器，未增加生产服务。

| 检查 | 实际结果 |
| --- | --- |
| 构建与脚本 | `check` / `build`、脚本语法检查通过；仍仅有 `missing-site-url` 提醒。 |
| 首页入口 | 手机浏览器实际点击底部“测试 CTA 表单”，进入 `/cta/`。 |
| 校验与反馈 | 空值、错误邮箱显示字段提示并聚焦错误项；有效提交禁用按钮，显示“正在提交…”。 |
| 原生提交 | 示例数据实际 POST 到本机接收器，收到 303 并进入 `/thanks/`；内存接收记录与输入一致。 |
| 无脚本 | 禁用 JavaScript 后仍成功原生提交并返回完成页。 |
| 布局与可访问性 | 1440、390、320px 无页面级横向溢出；输入与提交按钮均为 56px；桌面和手机 axe 所检 WCAG 2 A/AA、2.1 AA 规则未发现违规。 |
| 按页权限 | 仅测试表单允许向 `http://127.0.0.1:4211` 提交；首页仍为 `form-action 'none'`。 |
| 视觉复查 | 实际查看桌面、手机、手机错误态及完成页截图；延续现有字体、蓝色主行动与薄荷绿反馈，标签、错误提示和焦点均可见，未发现需要继续修补的排版问题。 |
| 运行错误 | 所测视口无 console error 或失败请求。 |

原始结果与上述四张截图在 [CTA 验证目录](review/cta/)，其中
[browser-check.json](review/cta/browser-check.json) 记录实际提交与接收结果。
该流程仅用于本机测试，记录保存在接收器内存中，不发送邮件；正式接入尚未配置。

## 验证边界

- phoenix-ui 的固定版本本地引擎缺失；按 skill 允许的回退流程使用原文、设计规则和真实浏览器。未声称 Phoenix context/profile/detector 或引擎校验通过。
- 静态内容、字体、样式改动未触及 SDK Rust 实现，未重跑全工作区 Rust 测试。
- 本落地页尚无正式发布地址，因此未配置 `site.url`，不生成猜测的 canonical/sitemap；严格无警告检查仍会因此失败。未执行部署。
- 未做真实流量或客户转化率实验；设计和功能证据不等于商业转化效果。
