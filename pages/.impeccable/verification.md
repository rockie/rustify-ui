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

## 验证边界

- phoenix-ui 的固定版本本地引擎缺失；按 skill 允许的回退流程使用原文、设计规则和真实浏览器。未声称 Phoenix context/profile/detector 或引擎校验通过。
- 静态内容、字体、样式改动未触及 SDK Rust 实现，未重跑全工作区 Rust 测试。
- 本落地页尚无正式发布地址，因此未配置 `site.url`，不生成猜测的 canonical/sitemap；严格无警告检查仍会因此失败。未执行部署。
- 未做真实流量或客户转化率实验；设计和功能证据不等于商业转化效果。
