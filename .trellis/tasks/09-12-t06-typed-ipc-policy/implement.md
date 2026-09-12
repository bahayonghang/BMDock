# T06 实施记录：typed IPC 与权限策略

## 边界

T06 在 Tauri Rust 与 React renderer 之间建立单一 `ipc_invoke` 命令入口及共享 DTO。当前允许的命令只有：

- `get_capabilities`：返回允许命令、事件和 fixture-only 策略；
- `get_runtime_state`：只读投影 managed `Supervisor` 快照（T07 共存）。IPC 不 start/stop 引擎；默认未启动投影为 `not_started`，`project` 恒为 `null`，不暴露 `child_pid`。Supervisor 的 `timeout_unknown` / `transport` / `process` 只出现在 `RuntimeStateDto`，不进入 IPC 错误联合；
- `select_project`：仅接受 `bmdock-fixture`。

事件名由 `IpcEventName` 与 `listenTyped` 共享约束为 `bmdock://runtime_state`、`bmdock://policy`。T06 不从 IPC 启动引擎、不调用 MCP、不读写文件，也不实现笔记 CRUD、真实 vault、搜索或安装器。`App.tsx` 保持静态壳，不调用 `invoke`。

P0 `bmdock-probe` 与 `just contract` / `just contract-main` 入口保持独立。在 T05–T08 全部通过前，不把 `just dev` / `just build` 切到 Tauri。

## 拒绝策略

Rust `IpcCommand` 使用 tagged serde DTO，未知命令无法反序列化；`SelectProjectArgs` 与 `EmptyArgs` 使用 `deny_unknown_fields`，任意路径字段会被拒绝。已知但非 fixture 项目返回 `policy` 分类错误。响应保留 `policy`、`schema`、`unsupported` 错误分类；不提供 raw `callTool` DTO 或 handler。双 profile（release `c0bd87c6` / 21 tools 与 main-preview `3452c821` / 27 tools）不得混合。

## 检查

最小命令：

- `cargo fmt --all -- --check`
- `cargo test -p bmdock-app --locked --offline`
- `npm run build`（`apps/bmdock-desktop`）
- `git diff --check`

便宜补充：`cargo test --workspace --locked --offline`、`cargo check --workspace --locked --offline`、`python ./.trellis/scripts/task.py validate 09-12-t06-typed-ipc-policy`。

`python -m scripts.tasks unit` 会因 G0 未 passed 且 T06/T07 已 completed 失败（`A later task was completed before G0`）。不要回退 T06/T07 去“修好”该相位顺序；单元证据改用 `python -m unittest discover -s tests -v`。不要把 `just contract`、UI 文案、工具清单或编译出的 exe 当作 T06 证明。

Native GUI、WebView2、Supervisor 实机启动、真实引擎、Job Object、真实 vault 和 hosted CI 仍为 `UNVERIFIED`。
