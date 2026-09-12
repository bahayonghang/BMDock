# T14 实施记录：草稿持久化与编辑器会话

## Order

1. 读取 PRD、父任务设计、ADR-0001、typed-ipc-policy、现有 T11/T12 IPC 与工作台。
2. 增加 BMDock 自有 `DraftStore`：生产默认 `EmptyDraftStore`；测试注入 `FixtureDraftStore`（`{temp}/bmdock-t14-*`）。
3. 在现有 `ipc_invoke` 上增加 typed `save_draft` / `load_draft`（`ExplicitRouteArgs` + identifier，save 另加 body；`deny_unknown_fields`）。
4. 工作台增加 zh-CN labeled textarea 编辑器会话：空态 / 错误 / 就绪；区分内存未保存、disk_verified、engine_persisted=false。
5. 更新 allowlist 11→13、typed-ipc-policy、DesktopShellTests、证据与 VERIFICATION。
6. 运行本任务最小检查；`execution/status.json` 仅将 T14 标为 completed。

## What shipped

- `apps/bmdock-desktop/src-tauri/src/drafts.rs`：`DraftStore`、`EmptyDraftStore`、测试用 `FixtureDraftStore` / `EnvelopeDraftStore`。
- `apps/bmdock-desktop/src-tauri/src/ipc.rs`：`save_draft` / `load_draft`，13 命令 allowlist。
- `apps/bmdock-desktop/src-tauri/src/main.rs`：生产注入 `EmptyDraftStore`。
- `apps/bmdock-desktop/src/ipc.ts`、`App.tsx`、`i18n.ts`、`styles.css`：typed helpers 与工作台编辑器。
- `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md`：T14 契约。
- `tests/test_desktop_shell.py`：编辑器与 13 命令断言。
- `execution/evidence/t14-draft-persistence-editor-session.json`、`docs/VERIFICATION.md`。

## Validation

| Command | Exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t14-draft-persistence-editor-session` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 82 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（8 passed） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+ completed；未回退） |
| `python -m unittest discover -s tests -v` | 1（68 tests：67 ok，1 ERROR `test_repository_phase_order`） |
| `just contract` | not_run |

## UNVERIFIED

- native GUI / WebView2 会话 / `just tauri-dev` 交互
- 官方引擎 `write_note` / 用户 vault / hosted CI
- T15 CRUD、T18 Windows 编辑器安全、T17/T37/T38 恢复
- `just contract` 未作为 T14 证明运行

## Rollback

移除 `save_draft` / `load_draft`、`drafts.rs` 与工作台编辑器，恢复 11 命令 allowlist。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T13。
