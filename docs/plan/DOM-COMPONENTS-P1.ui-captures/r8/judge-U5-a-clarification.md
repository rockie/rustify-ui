## 附录：U5 第 5 节的证据边界澄清

依据冻结目标包的 **U5「单元」状态矩阵**及**「采集」中的 U5 步骤**，本轮没有发现必需格子未采到、图像模糊，或整个必需状态没有可用证据的情况。**必需格子的最少补采需求：无。**

原文第 5 节列的是矩阵之外尚未直接展示的具体行为，以及正常视口裁切和局部遮挡；不能将其统称为“必需矩阵缺证”。其中一些扩展行为仍有 P1–P5 规范依据，只是没有被目标包列为独立的必拍状态。

本附录不改动原评分、差距、理由或 UI-218 的复核结果。

### 必需状态覆盖

以下文件均位于本轮 `r8/` 目录。

| U5 状态 | 1440×900 · light | 1440×900 · dark | 390×844 · light |
| --- | --- | --- | --- |
| DOM 预览及根模式 | `U5-dom-1440-light.png` | `U5-dom-1440-dark.png`、`U5-matrix-dom-1440-dark.png` | `U5-matrix-dom-390-light.png` |
| 局部边界选择相反外观 | `U5-local-opposite-1440-light.png` | `U5-matrix-local-opposite-1440-dark.png` | `U5-matrix-local-opposite-390-light.png` |
| 独立对比区选择相反模式 | `U5-compare-nested-1440-light.png` | `U5-matrix-compare-nested-1440-dark.png` | `U5-matrix-compare-nested-390-light.png` |
| 预览菜单打开 | `U5-menu-1440-light.png`、`U5-local-opposite-menu-1440-light.png` | `U5-matrix-menu-1440-dark.png`、`U5-matrix-local-opposite-menu-1440-dark.png` | `U5-matrix-menu-390-light.png`、`U5-matrix-local-opposite-menu-390-light.png` |
| 预览对话框打开 | `U5-dialog-1440-light.png`、`U5-local-opposite-dialog-1440-light.png` | `U5-matrix-dialog-1440-dark.png`、`U5-matrix-local-opposite-dialog-1440-dark.png` | `U5-matrix-dialog-390-light.png`、`U5-matrix-local-opposite-dialog-390-light.png` |
| 拾取部件并定位来源 | `U5-inspector-1440-light.png`、`U5-inspector-picked-1440-light.png` | `U5-matrix-inspector-1440-dark.png`、`U5-matrix-inspector-picked-1440-dark.png` | `U5-matrix-inspector-390-light.png`、`U5-matrix-inspector-picked-390-light.png` |
| DOM＋GPU 对比 | `U5-gpu-compare-1440-light.png` | `U5-matrix-gpu-compare-1440-dark.png` | `U5-matrix-gpu-compare-390-light.png`、`U5-matrix-gpu-compare-end-390-light.png` |

“预览深色”由 **1440×900 · dark** 的对应帧展示；目标矩阵没有要求 **390×844 · dark**。

### 局部遮挡与替代帧

| 情况 | 同一必需状态的可用证据与限制 |
| --- | --- |
| 1440 深色拾取时，面板遮住 Create profile 按钮下缘 | `U5-matrix-inspector-1440-dark.png` 与 `U5-matrix-inspector-picked-1440-dark.png` 都存在该局部遮挡，没有完全无遮挡的同视口、同主题替代帧。但两帧仍可见按钮蓝色顶边和侧边、完整 token 面板；后者另显示 Dark 来源入口，足以辨认必需的拾取及来源状态。被挡底边的完整几何仍看不出。浅色及390px帧不能替代这个暗色底边细节。 |
| 390px 拾取后切到编辑页，选中部件不再同屏 | 同一流程的 `U5-matrix-inspector-390-light.png` 展示所选部件及 token 面板，`U5-matrix-inspector-picked-390-light.png` 展示来源编辑入口；两帧共同覆盖该必需状态。 |
| 桌面 GPU 下部及390px token 列表末尾被裁切 | 没有展示全部下部内容的替代帧；但必需状态是 DOM＋GPU 对比，其已有可用证据。390px 首尾两帧分别展示 DOM 和后续 GPU 区。完整 token 列表不是矩阵中的独立必拍状态，不能把正常纵向裁切记为该格缺失。 |

### 原第 5 节其余事项的分类

| 事项 | 证据边界分类 |
| --- | --- |
| 1024／768／390 固定内容宽度选择 | P5 下尚未直接取证的配置组合；不属于列明的必拍状态及 U5 采集步骤。 |
| 浮层内拾取、浮层打开期间修改创建处主题 | P2／P3 下尚未直接取证的扩展操作；已拍浮层外观不能证明这些动态行为。 |
| 完整键盘拾取、Esc、Tab 顺序、焦点恢复 | 矩阵之外的操作行为；焦点对象记录不能替代键盘过程证据。 |
| 普通模式表单提交、示例动作及完整成功／错误／重置反馈 | P1 下尚未展示的具体操作分支；不是本轮矩阵中的独立必拍状态。 |
| GPU 失败、恢复及中文状态 | P4 下尚未直接取证的异常与语言组合；不属于列明的必拍矩阵。 |

本附录未新增采集、未新增开图，也未读取其他文件。
