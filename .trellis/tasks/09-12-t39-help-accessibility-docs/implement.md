# T39 实施记录：用户帮助无障碍及支持文档

## Outcome

`accepted_with_limits`。typed `inspect_help` 已加入现有 `ipc_invoke`（第 44 个允许命令）。帮助目录写明夹具自有 `preview_context` / `list_activity`，官方 `recent_activity` / `build_context` 仍为 `UNVERIFIED`。恢复库存是 `list_backups` + `restore_fixture`。Renderer skip-link / nav / main landmarks / `:focus-visible` 由 T39 拥有。native 读屏 / IME / native GUI 保持 `UNVERIFIED`。已提交 `docs/HELP.md`。未宣称 G0 / G7 通过。

## What was implemented

1. Rust `NoteLibrary::inspect_help` + `HelpInspectionDto`（`deny_unknown_fields` 路由经 `ExplicitRouteArgs`）。生产 `EmptyLibrary` 返回空 extra catalog、`classified_as: empty`、`files_written=false`。夹具自有 `preview_context` / `list_activity`。官方 `recent_activity` / `build_context` 为 `UNVERIFIED`。恢复库存为 `list_backups` + `restore_fixture`。native 读屏 / IME / native GUI / G0 / G7 不为已通过。
2. Fail-closed：额外 `path`/`root`/`token`/`host` → schema；缺路由 → schema；非 fixture → policy，且不打开库。vault / secrets / remote → policy。宣称 native 读屏 / IME / native GUI / G0 / G7 / 官方 recent_activity / 云恢复 → unsupported。夹具 `{temp}/bmdock-t39-*` 的 `help-claimed` / `a11y-cleared` 文件旗标为 unsupported，不是原生无障碍审计。
3. IPC：`IpcCommandName::InspectHelp` 进入 44 命令允许列；`restore_sync` 仍缺席。表驱动 `explicit_route_args_commands_fail_closed_without_opening_library` 覆盖 `inspect_help`。
4. Renderer：zh-CN「帮助 / 无障碍」面板。empty / error / ready。`contentSafetyHelpNotT39` 替换为 T39 自有文案。无 `dangerouslySetInnerHTML`。无新 npm 依赖。不启动 Supervisor。无 rmcp。
5. 规范：`.trellis/spec/bmdock-probe/backend/{index.md,typed-ipc-policy.md}` 允许列 43 → 44。`tests/test_desktop_shell.py` 第 33 项 `test_help_accessibility_docs_are_owned_and_native_a11y_unverified`。提交 `docs/HELP.md`。
6. `execution/status.json` 仅将 T39 `planned` → `completed`。未改 T05–T38 / G0。未改 T40。未改父任务目录。未改 justfile。

## AC mapping

| ID | Result |
|---|---|
| AC31 | 帮助目录写明夹具自有 `preview_context` / `list_activity`。官方 `recent_activity` / `build_context` 仍为 `UNVERIFIED`。未加入 live engine MCP。 |
| AC53 | 帮助与支持文档写明恢复库存是 `list_backups` + `restore_fixture`。不是云恢复，也不是安装器回滚用户 vault。 |
| AC57 | T39 拥有 renderer skip-link、nav/main landmarks、带标签面板、`:focus-visible`、可键盘聚焦控件。native 读屏 / IME / native GUI 为 `UNVERIFIED`。 |
| AC60 | `docs/HELP.md` 与 `just dev=tauri-dev`、`just build=G0 探针`、`just contract*=探针`、双 profile 21 vs 27、LICENSE/NOTICE/SBOM 存在但 G7 `UNVERIFIED` 一致。About/帮助列出 typed 允许列含 `inspect_help`。不宣称 G0/G7 已通过。 |

## Commands and exits

| Command | Exit |
|---|---|
| `python ./.trellis/scripts/task.py validate 09-12-t39-help-accessibility-docs` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 212 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 supervisor `dead_code` 警告；未启动 Supervisor） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（33 tests） |
| `python -m scripts.tasks unit` | 1（`A later task was completed before G0`；未回退 T05+） |

未运行：`just contract` / `just contract-main`、原生 GUI / `just tauri-dev` 交互、native 读屏 / IME 审计。

## UNVERIFIED

- native GUI / WebView2 session
- native screen reader
- IME session
- Job Object assignment
- official recent_activity / build_context MCP
- real user vault
- G0 product gate / G7 / hosted CI for this revision

## What was not done

- 未打开原生窗口，未运行 native 读屏 / IME 审计。
- 未启动 Supervisor，未加入 rmcp，未改 `just build` 或 `justfile`。
- 未打开 `.work/engines`、用户 vault、secrets；未改父任务目录；未改 T40；未改 T05–T38 / G0。
- 未把 UI 文案、工具清单、cargo test 或 `just contract` 当作 native GUI / 读屏 / IME / Job Object / 真实 vault 证据。
- 未 git commit / push / merge / amend。

## Files

- `apps/bmdock-desktop/src-tauri/src/library.rs`
- `apps/bmdock-desktop/src-tauri/src/ipc.rs`
- `apps/bmdock-desktop/src/ipc.ts`
- `apps/bmdock-desktop/src/App.tsx`
- `apps/bmdock-desktop/src/i18n.ts`
- `apps/bmdock-desktop/src/styles.css`
- `tests/test_desktop_shell.py`
- `.trellis/spec/bmdock-probe/backend/index.md`
- `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md`
- `docs/HELP.md`
- `docs/VERIFICATION.md`
- `execution/status.json`
- `execution/evidence/t39-help-accessibility-docs.json`
- `.trellis/tasks/09-12-t39-help-accessibility-docs/implement.md`

未改：`justfile`。未改父任务目录。未改 T40。
