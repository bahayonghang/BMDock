# T33 执行记录：官方 Agent 集成及 hook 管理

## 交付边界

在现有 `ipc_invoke` 上增加 typed `inspect_hooks`（`ExplicitRouteArgs`，`deny_unknown_fields`）。额外 `path`/`root`/`token`/`host` 为 schema。缺路由为 schema。非 fixture 为 policy，且不打开库。官方 Agent 集成保持 FAIL-CLOSED。生产默认 `hooks_enabled=false`、`agent_connected=false`、`files_written=false`。生产 hook 目录为空。宣称 installed/connected 而没有 live official agent session 为 unsupported。`list_hooks` / `install_hooks` 不在允许列。夹具 `{temp}/bmdock-t33-*` 默认仍报告 `hooks_enabled=false`；BMDock 自有 `hook-claimed` 标记仍不是 live agent，分类为 unsupported，不是 installed。

不启动 Supervisor。无 rmcp。不把 `just contract` 当作 T33 证明。`just build` 仍为 G0 探针。

## 验收映射

- AC44：官方 Agent 集成 FAIL-CLOSED。`hooks_enabled=false`，`agent_connected=false`，`files_written=false`。不写入用户 Agent 配置、Cursor rules 或真实 vault。双 profile 隔离（21 vs 27）。
- AC47：未授权远程 / env token / stored secret / remote host 为 policy。宣称 installed/connected 而没有 live official agent session 为 unsupported。夹具 hook-claimed 不是 installed。

## 实现顺序

1. library DTO / accept / EmptyLibrary 默认空目录 / FixtureLibrary `bmdock-t33-*` + `hook-claimed`。
2. ipc_invoke 允许列 37 → 38，`inspect_hooks` 调度，schema/policy/unsupported 测试。
3. ipc.ts 与 zh-CN Hook / Agent 空态/错误/就绪（未启用 / 未连接）。
4. DesktopShellTests 第 27 项。typed-ipc-policy 允许列。
5. 证据 `execution/evidence/t33-agent-hooks.json`，`docs/VERIFICATION.md`，`execution/status.json` 仅 T33 planned → completed。

## 验证命令

| 命令 | exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t33-agent-hooks` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 187；bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 Supervisor dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（DesktopShellTests 27） |
| `python -m scripts.tasks unit` | 1（G0 未通过且 T05+ 已 completed；未回退） |
| `just contract` / native GUI | 未运行 |

`python -m scripts.tasks unit` 因 G0 未通过且 T05+ 已 completed 失败；不要回退。

## 回滚

从 `ipc_invoke` 去掉 `inspect_hooks`，删除 HookPanel，把允许列从 38 恢复为 37。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T32。
