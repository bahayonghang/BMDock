# T17 实施记录：正常退出排空与故障恢复

本文件是 T17 的执行记录。未实现 T18–T40，未改 T05–T16/G0，未 git commit。

## 交付

- 宿主排空闸门 `HostDrain`：`apps/bmdock-desktop/src-tauri/src/drain.rs`
- typed `begin_shutdown`（`EmptyArgs`）：`ipc.rs` / `ipc.ts`
- 排空开始后拒绝新 CRUD（`unsupported` / `host is draining`）
- Inflight 键记录为 `inflight_unknown` 或不静默重试
- zh-CN 运行状态：idle / draining / timeout_unknown / conflict
- 证据：`execution/evidence/t17-graceful-exit-recovery.json`

## AC 映射

- AC05：`ShutdownReceipt` 字段复用；idle/`not_started` 不 spawn、不 kill；FakeChild 优雅关闭 `forced=false`
- AC17：优雅路径 `forced=false`
- AC18：强杀 / Job Object / 睡眠恢复 / 磁盘故障 `UNVERIFIED`；成功排空不是 T37/T38
- AC52：排空 ≠ T16 conflict ≠ T12 restore_fixture

## 验证命令与退出码

| 命令 | exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t17-graceful-exit-recovery` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 120 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（11 passed） |
| `python -m unittest discover -s tests -v` | 1（70 ok，1 ERROR phase order） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+，未回退） |
| `just contract*` | 未运行 |

## UNVERIFIED

强杀恢复、Job Object 分配、睡眠恢复、磁盘故障、native GUI、真实 vault、活动引擎寿命排空、T37/T38。T07 单元假对象不是 process-tree。
