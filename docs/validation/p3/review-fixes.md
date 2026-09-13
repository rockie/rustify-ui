# P3 完成后评审修复

日期：2026-09-14。代码基线：`dirty@845ed96`，开工时工作树 clean。
浏览器使用四示例的本地 release 产物、Playwright 配置中的无头 Chromium。

12 条评审意见均成立，已修复。功能回归及 B2 性能门均通过；以下记录不代替本轮未执行的完整 P3 套件。

| 意见 | 核实与修复 | 回归证据 |
| --- | --- | --- |
| P1 当前视图删除 | 先解析整个视图区间的数据集位置与稳定 ID，再按数据集位置倒序删除、修正视图并移除选择 | `p3-table.spec.ts`：排序和筛选后删除，后续显示 ID 必须精确等于删除前第 11 行起的 ID，选择只保留存活对象 |
| P1 fatal 后清除挂载标记 | property-workbench 与 component-catalog 复用 `release_container()`，包含 catalog 第二容器 | `p3-restart.spec.ts`：fatal 后标记消失，连续两次重启均重新挂载成功 |
| P2 新实例编号 | 两示例使用 `started.instance` 调用 identify | 重启后诊断编号递增；workbench URL 所有权匹配编号，再次 fatal 后清除 |
| P2 空筛选 | 空文本仍生成匹配全部的扫描作业 | 文本匹配结果及零匹配结果都可通过清空恢复 100,000 行 |
| P2 焦点槽回收 | 逻辑焦点移入新窗口；只协调已在单元格内的 DOM 焦点，使用 preventScroll 避免回滚，保留唯一单元格 tab 入口 | 横纵滚动后 Space 选择实际焦点所在 ID；表尾放大/缩小容器仍保持实际焦点与 tab 入口一致 |
| P2 表头键盘 | 网格命令只处理 gridcell 来源；焦点效果不抢表头焦点 | 表头 Enter/Space 均排序，不打开详情或选择数据行 |
| P2 尺寸重测 | 实际 scroller 的 ResizeObserver 更新窗口；宿主托管观察器，正常卸载 drop、fatal AbortSignal 均断开 | 1100×600 → 1440×1200、仅改变元素 max-height 均重建足够窗口；卸载/重挂/fatal 的观察器数依次 0/1/0 |
| P2 编辑对象 | 详情持有稳定 ID，保存时解析当前位置；Memo 比较对象内容以保留未保存草稿，对象删除关闭详情 | 打开 ID 20、编辑草稿、插入十行、保存、删除前面的十行、再保存均作用于 ID 20；删除 ID 20 后详情关闭 |
| P2 排序标记 | 成功提交视图时同步标记；取消保留旧标记，筛选提交清除标记 | 页内同一任务启动排序并取消，确认标记始终为旧排序；后续筛选清除 `aria-sort` |
| P2 父分组 | 父节点匹配整组，子节点匹配原子组 | QQ 筛选后点击 g1，结果逐项等于位置 10,000–19,999 |
| P2 重启加载失败 | 四示例 relaunch 捕获加载及挂载异常，恢复 failed 状态；共享错误提示提供重新加载按钮 | 四示例阻断 `bindgen.js?instance=*` 后显示加载错误，无未处理异常；恢复请求后按钮可重新启动页面 |
| P2 ARIA 索引 | 表头索引 1；数据行索引为视图位置 + 2；总行数包含表头，业务位置及跳行输入不变 | 表头、首数据行、末数据行索引分别为 1、2、100,001，总行数 100,001；原键盘/跳行/查找测试同步换算 |

ARIA 依据：[W3C APG：aria-rowcount 与 aria-rowindex](https://www.w3.org/WAI/ARIA/apg/practices/grid-and-table-properties/#using-aria-rowcount-and-aria-rowindex)。它要求总数包括表头，各行索引从 1 开始。

## 红绿证据

修复前运行：

```sh
npx playwright test --project=data-workbench --project=property-workbench --project=component-catalog --project=fusion-basic --grep 'review ·'
```

19 个用例，17 失败、2 通过。失败分别观察到保留挂载标记、starting 状态未退出、20,190 行不能恢复、错误删除 ID、草稿变为 NEW100000AAAAAAA、无单元格 tab 入口、尺寸扩大后仍只有 37 行等；通过的是原本正常的 fusion-basic 和 data-workbench 重启路径。其后增加观察器清理及表尾尺寸变化用例，共新增 21 个回归用例。

修复后运行：

```sh
npx playwright test p3-table p3-jobs p3-restart p3-instances p3-policy p3-scene --project=data-workbench --project=property-workbench --project=component-catalog --project=fusion-basic
npx playwright test p3-table --project=data-workbench --grep 'review ·'
```

第一条 **67/67 通过**，包括原有实例隔离、URL 所有权、重启次数上限、CSP、作业写入失效、键盘导航和场景交互；第二条在最终 data-workbench 构建上 **8/8 通过**，其中新增表尾尺寸变化 1 项。两次合计覆盖 68 个不同用例。

## 构建与静态验证

- 四个示例均执行 `cargo xtask build-web --example <name> --release`，成功；最后的排序比较器与进度计算 Clippy 修正后再次构建 data-workbench。
- `cargo test --workspace --lib`：204 通过；`cargo test -p data-workbench`：39 通过。
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`：通过。
- `cargo clippy --target wasm32-unknown-unknown -p rustify-components -p data-workbench --no-deps --locked -- -D clippy::all`：通过。额外覆盖原生构建不编译的浏览器实现；修正新增比较器写法及同文件原有的手写除零分支，未新增 lint 豁免。
- `cargo fmt --all -- --check`、`git diff --check`、`cargo xtask sources verify`、`cargo xtask css --check`：通过。
- 四示例 `app.js`、共享 loader 与 embedded host 的 `node --check`：通过。

## 性能与验证边界

`npx playwright test --project=budget-data`：**4/4 通过**。

- 五轮滚动各 60 秒，p95 为 **16.70/16.70/16.70/16.70/16.80 ms**（上限 20 ms）；五轮 >50 ms 帧占比均为 0%，每轮驱动/呈现/重算相等，前四轮各 3,601，末轮 3,602。
- 冻结内容的负向探针：301 次驱动、0 呈现，正确判定不满足门。
- 20 次排序 p95 **83 ms**，20 次筛选 p95 **33 ms**，均低于 500 ms；20 次取消反馈 p95 **14.1 ms**，低于 100 ms。
- 作业结果与独立清单一致。

本轮未重跑 `cargo xtask verify --suite p3` 全套、B0/B1 启动门、B3 有头帧门、双干净构建和两小时长时测试。M8 原有结果保留为历史证据；VoiceOver 仍为用户豁免的未测项，本轮不增加屏幕阅读器支持声明。
