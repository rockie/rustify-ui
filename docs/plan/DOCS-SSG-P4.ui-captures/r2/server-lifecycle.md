# R2 临时服务生命周期

- 本轮开始时4199/4204/4205/4206无监听。使用本仓tests/docs-ssg/serve.sh与固定实装CLI0.1.27启动，无新容器或基础设施。
- 仅为启动进程设置PATH=/opt/homebrew/opt/python@3.12/libexec/bin:$PATH，RUSTIFY_DOCS_BIN指向target/docs-ssg/p4/install-1bbc54b0/bin/rustify-docs。
- official/subpath4199、search/root4204、search/subpath4205、search/meta4206；各startup日志保留。meta使用项目原有static-host.mjs。
- 2026-10-16T03:53:37Z核对四端口的实际进程命令后，对本轮PID79936/83599/80922/85296发送SIGTERM；完整命令与时间见server-lifecycle.json。
- 随后lsof确认四端口无监听，四个exec会话均以143结束（主动停止）。其他已有服务未动。浏览器任务页签按kimi-webbridge约定保留，Chrome原生缩放已恢复100%。
- 重启可按同样环境运行serve.sh；R2采集器保留本机绝对路径，跨机器先对齐路径与固定CLI，不覆盖已有轮次图片。
