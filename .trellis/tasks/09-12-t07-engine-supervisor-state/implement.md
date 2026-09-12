# T07 实施计划：Engine Supervisor 与连接状态机

## 前置门禁

1. 运行 `python ./.trellis/scripts/task.py validate 09-12-t07-engine-supervisor-state`。
2. 确认 T02 的 profile 固定版本和 T05/T06 的 Tauri crate、typed IPC 边界仍存在。
3. 记录工作区并保留非 T07 改动；不修改 `.work/engines`、真实 vault 或全局配置。

## 有序步骤

1. 在 `apps/bmdock-desktop/src-tauri/src/supervisor.rs` 定义 profile、状态、失败分类、snapshot、launch spec 和 shutdown receipt。
2. 实现严格状态转换和 Supervisor 所有权；以注入的 child/transport 接口支持可重复单元测试。
3. 将模块纳入 Tauri crate，保留 T06 `ipc_invoke` 的 typed/raw 边界；只提供后续接线所需的 Rust API，不新增 raw MCP 路由。
4. 为正常连接、重复启动、非法 connected、transport 失败、cancel 超时、wait 超时强杀和 profile 隔离增加单元测试。
5. 更新 T07 证据记录和相关 code-spec；真实 engine、native GUI、跨平台 Job Object/进程树证据若未取得，明确写 `UNVERIFIED`。

## 最小验证

- `python ./.trellis/scripts/task.py validate 09-12-t07-engine-supervisor-state`
- `cargo fmt --all -- --check`
- `cargo test -p bmdock-app --locked --offline`
- `cargo check --workspace --locked --offline`
- `npm run build`（`apps/bmdock-desktop`）
- `git diff --check`

## 回滚点

- 若 Supervisor 设计要求真实 rmcp/异步运行时而当前证据不足，保留纯状态机和注入边界，禁止绕过测试接入 P0 probe。
- 若 Tauri IPC 类型需要破坏性扩展，先停在本任务 Rust API，不修改 renderer 命令联合或用户数据路径。

## 完成门槛

只有状态机、失败分类、两阶段关闭 receipt、profile 隔离和单元测试证据齐全后，才可推进 T08；真实官方引擎启动、native GUI 和跨平台强杀仍必须分别取得直接证据。
