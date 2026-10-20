# 本轮工具能力探测

## Phoenix

前缀：`docs/plan/DOCS-SSG-P4`。本轮开始没有 `.ui-review.json`、`.ui-review*.md` 或 `.ui-target.md`；旧 bounded finish-review 不是此评分循环。未导入或改写旧结论。

实际执行：

```sh
.agents/skills/phoenix-ui/scripts/phoenix-ui engine-probe
```

退出码4，原始输出：

```text
Phoenix UI: Phoenix binary digest or size differs from the package manifest.
npm exec --package=@xgent-ai/skills@0.7.3 -- phoenix-ui engine install --project '/Users/rockie/Documents/gh-rockie/rustify-ui'
```

按 review-ui 的全新循环回退规则采用人工流程；没有执行安装、没有绕过校验运行engine、没有创建managed索引，也没有猜测review typed schema。若以后需要managed，先恢复匹配engine，再显式import人工历史；不能begin清零。

人工目标指纹采用明确命名的`manual-sha256-normative-sections-v1`，见baseline.json；没有可验证engine时未冒称其兼容`phoenix-ui-target-v1`。后续人工轮次按同一算法比较；切换engine需明确重新核对映射。

## 设计上下文

实际执行一次：

```sh
.agents/skills/impeccable/scripts/impeccable context --target crates/rustify-docs/src/shell.rs
```

退出0；定位PRODUCT.md、DESIGN.md与`.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md`。另直接读取DESIGN.docs-ssg.md、PRD和计划原文建立目标。本轮没有启动独立impeccable critique工作流，不以代码检测器替代review-ui实拍证据。

## 浏览器

- 已安装Playwright1.63.0、Chromium153.0.8010.12、axe4.13.0；无需下载浏览器。
- Kimi扩展/daemon均为2.0.22且已连接；提示2.0.23可用、命令`kimi-webbridge upgrade`，已告知用户，没有升级。
- Kimi的`Browser.getVersion` CDP调用返回`-32601`，改为读取页面实际navigator.userAgent；没有因此声称获得完整品牌浏览器版本。
- CUA inventory可见Safari；本轮新页签与原生截图成功。Safari本地App版本27.0.1。已安装Google Chrome App元数据为155.0.8059.82，但Kimi页签UA为154.0.0.0；两者不能无依据合并为同一被测会话，不声称最新稳定矩阵通过。
- `/Applications/Firefox.app`不存在；没有安装或下载。

## Python与采集等待

- 首次serve.sh使用系统Python3.9，报`ModuleNotFoundError: No module named 'tomllib'`；原日志保留attempt1。
- 本机已有`/opt/homebrew/opt/python@3.12/libexec/bin/python3`，版本3.12.13；仅调整本轮启动进程PATH，重新启动成功。
- 第一次禁脚本拍摄停在采集器的rAF等待，没有产生该格图像。终止自己的采集器进程47822，修正采集器只在启用脚本时等待帧，再续接缺格；原图不覆盖，已有22张官方截图均保留。没有修改被测产品。
