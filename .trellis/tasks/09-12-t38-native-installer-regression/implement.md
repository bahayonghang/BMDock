# T38 实施记录：原生安装包及故障回归

## Outcome

`accepted_with_limits`。typed `inspect_bundle` 已加入现有 `ipc_invoke`（第 43 个允许命令）。`bundle.active` 仍为 `false`。未制作 MSI / NSIS / AppImage / dmg。恢复链仍是 T12 `restore_fixture`。强杀 / Job Object / 睡眠恢复 / 磁盘故障 / native GUI 保持 `UNVERIFIED`，不得报告为已恢复。

## What was implemented

1. Rust `NoteLibrary::inspect_bundle` + `BundleInspectionDto`（`deny_unknown_fields` 路由经 `ExplicitRouteArgs`）。生产 `EmptyLibrary` 返回空 catalog、`classified_as: empty`、`files_written=false`、`installer_artifact_present=false`、`installer_bundle_active=false`、`signed=false`、`native_gui=false`、`installer_rollback=false`、`recovery_command=restore_fixture`、`restore_sync_present=false`、`search_elapsed_ms=0`、`search_elapsed_host_side=true`。`signing` / native GUI / kill / Job Object / sleep-resume / disk-failure 为 `UNVERIFIED`。
2. Fail-closed：额外 `path`/`root`/`token`/`host` → schema；缺路由 → schema；非 fixture → policy，且不打开库。vault / secrets / remote → policy。宣称原生安装包 / MSI / NSIS / AppImage / dmg / 故障注入已恢复 / native GUI → unsupported。夹具 `{temp}/bmdock-t38-*` 的 `bundle-claimed` / `installer-present` 文件旗标为 unsupported，不是原生安装包。
3. IPC：`IpcCommandName::InspectBundle` 进入 43 命令允许列；`restore_sync` 仍缺席。表驱动 `explicit_route_args_commands_fail_closed_without_opening_library` 覆盖 `inspect_bundle`。
4. Renderer：zh-CN「原生安装包 / 故障回归」面板。empty / error / ready 均显示无安装包 / 故障注入未验证。无 `dangerouslySetInnerHTML`。无新 npm 依赖。不启动 Supervisor。无 rmcp。
5. 规范：`.trellis/spec/bmdock-probe/backend/{index.md,typed-ipc-policy.md}` 允许列 42 → 43。`tests/test_desktop_shell.py` 第 32 项 `test_native_installer_regression_is_absent_and_faults_unverified`。
6. `execution/status.json` 仅将 T38 `planned` → `completed`。未改 T05–T37 / G0。未改 T39–T40。未改父任务目录。未改 justfile。

## AC mapping

| ID | Result |
|---|---|
| AC18 | 强杀 / Job Object / 睡眠恢复 / 磁盘故障仍为 `UNVERIFIED`。`inspect_bundle` 不得报告为已恢复。宣称故障注入成功为 unsupported。与 conflict / `timeout_unknown` 区分。 |
| AC50 | 原生安装包产物缺席。未制作 MSI / NSIS / AppImage / dmg。`installer_artifact_present=false`。编译 debug exe 不是安装包。 |
| AC51 | `bundle.active=false`。`installer_bundle_active=false`，`signed=false`。未启用打包。 |
| AC52 | 恢复仍是 T12 `restore_fixture`。`restore_sync` 缺席。未发明安装器回滚用户 vault。 |
| AC55 | `just build` 仍是 `scripts.tasks` build（G0 探针）。`just tauri-dev` / `tauri-build` 仍是桌面。`just contract*` 仍是探针。未改 justfile 配方。 |
| AC56 | T24 `search_elapsed_ms` 仍是宿主侧计时，不是 native GUI / WebView2 / 安装包证明。`native_gui=false`。 |

## Commands and exits

| Command | Exit |
|---|---|
| `python ./.trellis/scripts/task.py validate 09-12-t38-native-installer-regression` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 208 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 supervisor `dead_code` 警告；未启动 Supervisor） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（32 tests） |
| `python -m scripts.tasks unit` | 1（`A later task was completed before G0`；未回退 T05+） |

未运行：`just contract` / `just contract-main`、原生 GUI / `just tauri-dev` 交互、MSI / NSIS / AppImage / dmg 制作。

## UNVERIFIED

- native GUI / WebView2 session
- Job Object assignment
- kill recovery / sleep-resume / disk-failure injection
- native installer / MSI / NSIS / AppImage / dmg
- installer rollback of a user vault
- real user vault
- official live engine / MCP session
- G0 product gate / G7 / hosted CI for this revision

## What was not done

- 未把 `bundle.active` 改为 `true`，未产出 MSI / NSIS / AppImage / dmg，未签名任何包。
- 未启动 Supervisor，未加入 rmcp，未改 `just build`（仍为 G0 探针）或 `just tauri-build`（仍不是安装器）。
- 未打开 `.work/engines`、用户 vault、secrets；未改父任务目录；未改 T39–T40；未改 T05–T37 / G0。
- 未把 UI 文案、工具清单、编译 exe、`tauri-build`、cargo test 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / 安装包证据。
- 未 git commit / push / merge / amend。

## Files

- `apps/bmdock-desktop/src-tauri/src/library.rs`
- `apps/bmdock-desktop/src-tauri/src/ipc.rs`
- `apps/bmdock-desktop/src-tauri/src/windows_runtime.rs`
- `apps/bmdock-desktop/src/ipc.ts`
- `apps/bmdock-desktop/src/App.tsx`
- `apps/bmdock-desktop/src/i18n.ts`
- `apps/bmdock-desktop/src/styles.css`
- `tests/test_desktop_shell.py`
- `.trellis/spec/bmdock-probe/backend/index.md`
- `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md`
- `docs/VERIFICATION.md`
- `execution/status.json`
- `execution/evidence/t38-native-installer-regression.json`
- `.trellis/tasks/09-12-t38-native-installer-regression/implement.md`

未改：`apps/bmdock-desktop/src-tauri/tauri.conf.json`（`bundle.active` 仍为 `false`）。未改 `justfile`。
