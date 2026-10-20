# 临时服务生命周期

本轮开始，4199/4205没有监听进程；随后按项目serve.sh启动。4204/4206也由本轮serve.sh启动，没有停止既有其他服务。

| 入口 | 启动脚本参数 | 成功运行PID | 停止方式 |
| --- | --- | ---: | --- |
| http://127.0.0.1:4199/rustify-ui/ | official subpath 4199 | 41493 | `kill -TERM 41493` |
| http://127.0.0.1:4205/rustify-ui/ | search subpath 4205 | 41526 | `kill -TERM 41526` |
| http://127.0.0.1:4204/ | search root 4204 | 48055 | `kill -TERM 48055` |
| http://127.0.0.1:4206/rustify-ui/ | search meta 4206 | 48097 | `kill -TERM 48097` |

启动均前置本轮PATH与RUSTIFY_DOCS_BIN，详见capture.md；成功/首次失败日志保留在本目录。

实际统一停止命令退出0：

```sh
kill -TERM 41493 41526 48055 48097
```

随后核对：

```sh
lsof -nP -iTCP:4199 -iTCP:4204 -iTCP:4205 -iTCP:4206 -sTCP:LISTEN
```

退出1、无输出，四个端口均无监听。报告内HTTP链接记录采集时真实入口；现在服务已停止，图像和记录仍可离线查看。没有停止Kimi daemon或关闭用户的既有页签。
