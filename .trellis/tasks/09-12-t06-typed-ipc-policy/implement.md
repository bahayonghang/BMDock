# T06 实施记录：typed IPC 与权限策略

## 边界

T06 在 Tauri Rust 与 React renderer 之间建立单一 `ipc_invoke` 命令入口及共享 DTO。当前允许的命令只有：

- `get_capabilities`：返回允许命令、事件和 fixture-only 策略；
- `get_runtime_state`：返回尚未启动 Supervisor 的明确 `not_started` 状态；
- `select_project`：仅接受 `bmdock-fixture`。

事件名由 `IpcEventName` 与 `listenTyped` 共享约束为 `bmdock://runtime_state`、`bmdock://policy`。T06 不启动引擎、不调用 MCP、不读写文件，也不实现笔记 CRUD、真实 vault、搜索或安装器。

## 拒绝策略

Rust `IpcCommand` 使用 tagged serde DTO，未知命令无法反序列化；`SelectProjectArgs` 使用 `deny_unknown_fields`，任意路径字段会被拒绝。已知但非 fixture 项目返回 `policy` 分类错误。响应保留 `policy`、`schema`、`unsupported` 错误分类；不提供 raw `callTool` DTO 或 handler。P0 `bmdock-probe` 与 `just contract*` 入口保持独立。

## 检查

- `cargo fmt --all -- --check`
- `cargo test -p bmdock-app --locked --offline`
- `npm run build`
- `git diff --check`

Native window interaction、Supervisor、真实引擎运行、跨平台 GUI 和 hosted CI 仍为 `UNVERIFIED`，由后续任务验证。
