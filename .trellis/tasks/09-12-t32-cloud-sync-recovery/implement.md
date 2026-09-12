# T32 执行记录：Cloud 同步共享与恢复管理

## 范围

在现有 `ipc_invoke` 上增加 typed `inspect_sync`（`ExplicitRouteArgs`，`deny_unknown_fields`）与 typed `list_shares`（同样的 fail-closed 规则）。Cloud 同步 / 共享保持 FAIL-CLOSED。生产默认 `sync_enabled=false`、`sharing_enabled=false`、`remote_restore=false`、`last_sync=none`。宣称 synced/shared 而没有 live official cloud session 为 unsupported。生产共享目录为空。宣称 live shared remote 为 unsupported。夹具 `{temp}/bmdock-t32-*` 默认仍报告 `sync_enabled=false`；BMDock 自有 `sync-claimed` 标记仍不是 live sync，分类为 unsupported，不是 synced。恢复仍走 T12 `restore_fixture`，不是 cloud restore。`restore_sync` 不在允许列。

不启动 Supervisor。无 rmcp。不把 `just contract` 当作 T32 证明。`just build` 仍为 G0 探针。

## 验收映射

- AC14：`inspect_sync` / `list_shares` 每次携带 `ExplicitRouteArgs`。额外 path/root/token/host 为 schema。缺路由为 schema。非 fixture 为 policy，且不打开库。
- AC42：无隐式 cloud/sync 路由。本地离线。双 profile 隔离（21 vs 27）。不把 profile 混进 sync DTO。
- AC43：Cloud 同步 / 共享 FAIL-CLOSED。生产 `sync_enabled=false`、`sharing_enabled=false`、`remote_restore=false`、`last_sync=none`。宣称 synced/shared 而没有 live official cloud session 为 unsupported。不联系远程主机、不存储密钥、不读取环境 token。
- AC52：恢复仍走 T12 `restore_fixture`。信封 `"synced"` / `"restored"` 不是磁盘证明。夹具本地恢复测试仍可观察 T12 物理落盘。未发明 cloud restore 磁盘成功。

## 验证命令

见 `execution/evidence/t32-cloud-sync-recovery.json`。本任务范围内最小检查：

| 命令 | 退出码 |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t32-cloud-sync-recovery` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 183；bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（保留既有 T07 Supervisor dead_code 警告） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（DesktopShellTests 26） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+；未回退） |
| `just contract` / `just contract-main` | 未运行 |

## 回滚

从 `ipc_invoke` 去掉 `inspect_sync` / `list_shares`，删除 SyncPanel，把允许列从 37 恢复为 35。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T31。
