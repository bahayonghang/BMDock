# T15 实施记录：完整笔记写入编辑移动删除

## Order

1. 读取 PRD、父任务设计、ADR-0001、typed-ipc-policy、现有 T11/T14 IPC 与工作台。
2. 扩展 `NoteLibrary`：生产默认 `EmptyLibrary` 四条 CRUD 为 `unsupported`；测试注入 `FixtureLibrary`（`{temp}/bmdock-t15-*`）。
3. 在现有 `ipc_invoke` 上增加 typed `write_note` / `edit_note` / `move_note` / `delete_note`（`ExplicitRouteArgs`；`deny_unknown_fields`）。
4. 工作台增加 zh-CN fixture CRUD 控件：写入/编辑/移动/删除（删除需确认）；空态 / 错误 / 就绪；`save_draft` 与 `write_note` 分开。
5. 更新 allowlist 13→17、typed-ipc-policy、DesktopShellTests、证据与 VERIFICATION。
6. 运行本任务最小检查；`execution/status.json` 仅将 T15 标为 completed。

## What shipped

- `apps/bmdock-desktop/src-tauri/src/library.rs`：`NoteLibrary` CRUD、`EmptyLibrary` unsupported、测试用 `FixtureLibrary` / `EnvelopeCrudLibrary`。
- `apps/bmdock-desktop/src-tauri/src/ipc.rs`：`write_note` / `edit_note` / `move_note` / `delete_note`，17 命令 allowlist。
- `apps/bmdock-desktop/src/ipc.ts`、`App.tsx`、`i18n.ts`、`styles.css`：typed helpers 与工作台 CRUD 控件。
- `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md`：T15 契约。
- `tests/test_desktop_shell.py`：CRUD 与 17 命令断言。顺序移动到已存在目标为 `unsupported`，T16 仍未验证。
- `execution/evidence/t15-note-crud-operations.json`、`docs/VERIFICATION.md`。

## Validation

| Command | Exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t15-note-crud-operations` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 97 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（9 passed） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+ completed；未回退） |
| `python -m unittest discover -s tests -v` | 1（69 tests：68 ok，1 ERROR `test_repository_phase_order`） |
| `just contract` | not_run |

## UNVERIFIED

- native GUI / WebView2 会话 / `just tauri-dev` 交互
- 官方引擎 `write_note` / `edit_note` / `move_note` / `delete_note` MCP / 用户 vault / hosted CI
- T16 同目标冲突与未知结果；不主张原子并发覆盖
- T18 Windows 编辑器安全、T17/T37/T38 恢复
- `just contract` 未作为 T15 证明运行

## Rollback

移除 `write_note` / `edit_note` / `move_note` / `delete_note`、工作台 CRUD 控件，恢复 13 命令 allowlist。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T14。
