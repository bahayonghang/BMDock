# T31 执行记录：Cloud 与远程授权连接

## 范围

在现有 `ipc_invoke` 上增加 typed `inspect_cloud`（`ExplicitRouteArgs`，`deny_unknown_fields`）。Cloud 保持 FAIL-CLOSED 本地离线。生产默认 `cloud_enabled=false`、`remote_auth=false`、`credentials_present=false`、`cloud_allowed=false`。宣称 connected/authenticated 而没有 live official cloud session 为 unsupported。未授权远程为 policy/unsupported，不是成功。夹具 `{temp}/bmdock-t31-*` 默认仍报告 `cloud_enabled=false`；BMDock 自有 `cloud-claimed` 标记文件仍不是 live cloud，分类为 unsupported，不是 connected。

不启动 Supervisor。无 rmcp。不把 `just contract` 当作 T31 证明。`just build` 仍为 G0 探针。

## 验收映射

- AC42：无隐式 cloud/remote 路由。本地离线。双 profile 隔离（21 vs 27）。不把 profile 混进 cloud DTO。
- AC45：不打开 cloud/remote/credential/real-vault 路由。capabilities 报告 `cloud_allowed=false`。不存储密钥。不从环境读取用户 token。不联系远程主机。
- AC47：未授权远程是 policy/unsupported，不是成功。夹具 `cloud-claimed` 不是 connected。

## 验证命令

见 `execution/evidence/t31-cloud-remote-auth.json`。本任务范围内最小检查：

| 命令 | 退出码 |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t31-cloud-remote-auth` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 179；bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（保留既有 T07 Supervisor dead_code 警告） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（DesktopShellTests 25） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+；未回退） |
| `just contract` / `just contract-main` | 未运行 |

## 回滚

从 `ipc_invoke` 去掉 `inspect_cloud`，删除 CloudPanel，把允许列从 35 恢复为 34。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T30。
