# T16 实施记录：并发冲突与未知结果协调

本文件是 T16 的执行记录，不是规划占位。未 git commit / push / merge / amend。未改父任务目录、T17–T40、`.work/engines`、用户 vault 或 secrets。`execution/status.json` 仅将 T16 从 `planned` 标为 `completed`；未改 T05–T15/G0。

## 做了什么

1. 新增 `ConflictCoordinator`（`apps/bmdock-desktop/src-tauri/src/conflict.rs`），对 typed `write_note` / `edit_note` / `move_note` / `delete_note` 做进程内同标识 inflight 守卫。`move_note` 同时占用 source 与 destination。
2. 第二次重叠同目标调用返回 CRUD DTO `classified_as: conflict`（`files_written=false`，`disk_verified=false`）。不是 `disk_verified`，不是 `timeout_unknown`，不是 policy-for-path，也不是 IPC error category。
3. 顺序移动到已存在目标仍为 `unsupported`，文案改为明确“不是 T16 same-target conflict”。这不是原子并发覆盖。
4. `timeout_unknown` 留在 `RuntimeStateDto` / `ShutdownReceipt`。IPC 错误联合体仍为 `policy` / `schema` / `unsupported`。`auto_retry_non_idempotent_write` 在 `timeout_unknown` 之后不调用 retry helper；测试用 AtomicBool 证明 helper 未被调用。用户再次发起的 typed write 不是自动重试。
5. 工作台用独立 zh-CN 文案展示 conflict / timeout_unknown / disk_verified / accepted_unverified。无 `dangerouslySetInnerHTML`。不启动 Supervisor。
6. 未新增 IPC 命令，未添加 rmcp。`just build` 仍为 G0 探针。未把 `just contract` 当作 T16 证明。

## 未做 / UNVERIFIED

- 真实并发 OS-thread 文件系统竞态 / OS file lock（进程内 inflight 只证明宿主协调）。
- 强杀、Job Object 分配、磁盘故障注入。
- T17 正常退出排空与 T37 安装升级恢复。
- 官方引擎 CRUD MCP、用户 vault、native GUI 会话、hosted CI。
- 丢失响应 / 接受后取消 / 对官方引擎的 timeout_unknown 现场注入。

## 验证命令

见 `execution/evidence/t16-conflict-unknown-result.json`。本机 Windows（2026-09-12）：

- `python ./.trellis/scripts/task.py validate 09-12-t16-conflict-unknown-result` exit 0
- `cargo fmt --all -- --check` exit 0
- `cargo test --workspace --locked --offline` exit 0（bmdock-app 109 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` exit 0（既有 T07 dead_code 警告）
- `npm run build`（`apps/bmdock-desktop`）exit 0；未跑 `npm ci`
- `git diff --check` exit 0
- `python -m unittest tests.test_desktop_shell -v` exit 0（10 passed）
- `python -m scripts.tasks unit` exit 1（G0 vs T05+，未回退）
- `python -m unittest discover -s tests -v` 70 项：69 ok，1 ERROR `test_repository_phase_order`
- `just contract*` 未运行
