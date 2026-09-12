# T37 实施记录：安装升级签名与数据恢复链

## Outcome

`accepted_with_limits`。typed `inspect_install` 已加入现有 `ipc_invoke`（第 42 个允许命令）。`bundle.active` 仍为 `false`。未制作或签名安装包。恢复链仍是 T12 `restore_fixture`。`native_gui` / 签名 / 强杀 / Job Object / 睡眠恢复 / 磁盘故障保持 `UNVERIFIED`。

## What was implemented

1. Rust `NoteLibrary::inspect_install` + `InstallInspectionDto`（`deny_unknown_fields` 路由经 `ExplicitRouteArgs`）。生产 `EmptyLibrary` 返回空 catalog、`classified_as: empty`、`files_written=false`、`installer_bundle_active=false`、`signed=false`、`upgrade_channel=false`、`native_gui=false`、`installer_rollback=false`、`recovery_command=restore_fixture`、`restore_sync_present=false`。`signing` / native GUI / kill / Job Object / sleep-resume / disk-failure 为 `UNVERIFIED`。
2. Fail-closed：额外 `path`/`root`/`token`/`host` → schema；缺路由 → schema；非 fixture → policy，且不打开库。vault / secrets / remote → policy。宣称签名安装包 / 升级通道 / 安装器回滚 / native GUI → unsupported。夹具 `{temp}/bmdock-t37-*` 的 `install-claimed` / `signed-upgrade` 文件旗标为 unsupported，不是签名安装包。
3. IPC：`IpcCommandName::InspectInstall` 进入 42 命令允许列；`restore_sync` 仍缺席。表驱动 `explicit_route_args_commands_fail_closed_without_opening_library` 覆盖 `inspect_install`。
4. Renderer：zh-CN「安装 / 升级 / 恢复」面板。empty / error / ready 均显示未签名 / 未打包 / 恢复走夹具 restore_fixture。无 `dangerouslySetInnerHTML`。无新 npm 依赖。不启动 Supervisor。无 rmcp。
5. T13 i18n 不再写「签名仍属 T37」；改为 T37 未签名真实安装包，签名仍为 `UNVERIFIED`。`windows_runtime.rs` 断言 T37 保持 `bundle.active=false`。
6. 规范：`.trellis/spec/bmdock-probe/backend/{index.md,typed-ipc-policy.md}` 允许列 41 → 42。`tests/test_desktop_shell.py` 第 31 项 `test_install_upgrade_recovery_is_unsigned_fixture_restore`。
7. `execution/status.json` 仅将 T37 `planned` → `completed`。未改 T05–T36 / G0。未改 T38–T40。未改父任务目录。

## AC mapping

| ID | Result |
|---|---|
| AC49 | Windows 运行时原型仍走 T13 `inspect_windows_runtime`。T37 未打开原生 GUI。`native_gui` 为 `UNVERIFIED`。编译 exe / npm build / cargo test 不是原生窗口。 |
| AC51 | `bundle.active=false`。未制作签名安装包 / MSI / NSIS / AppImage。`installer_bundle_active=false`，`signed=false`，`upgrade_channel=false`。宣称签名升级为 unsupported。签名 `UNVERIFIED`。 |
| AC52 | 恢复链仍是 T12 `restore_fixture`，不是 cloud restore，也不是安装器回滚用户 vault。`restore_sync` 缺席。强杀 / Job Object / 睡眠恢复 / 磁盘故障 `UNVERIFIED`。与 conflict / `timeout_unknown` 区分。 |
| AC53 | 恢复库存仍是 `list_backups` + `restore_fixture`。`inspect_install` 报告 `recovery_command=restore_fixture`、`installer_rollback=false`。未发明生产升级恢复成功。 |

## Commands and exits

| Command | Exit |
|---|---|
| `python ./.trellis/scripts/task.py validate 09-12-t37-install-upgrade-recovery` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 204 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 supervisor `dead_code` 警告；未启动 Supervisor） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（31 tests） |
| `python -m scripts.tasks unit` | 1（`A later task was completed before G0`；未回退 T05+） |

未运行：`just contract` / `just contract-main`、原生 GUI / `just tauri-dev` 交互、签名安装包制作。

## UNVERIFIED

- native GUI / WebView2 session
- Job Object assignment
- kill recovery / sleep-resume / disk-failure injection
- installer / signed bundle / MSI / NSIS / AppImage / 真实签名
- production upgrade recovery of a user vault
- installer rollback of a user vault
- real user vault
- official live engine / MCP session
- G0 product gate / G7 / hosted CI for this revision

## What was not done

- 未把 `bundle.active` 改为 `true`，未产出 MSI / NSIS / AppImage，未签名任何包。
- 未启动 Supervisor，未加入 rmcp，未改 `just build`（仍为 G0 探针）或 `just tauri-build`（仍不是安装器）。
- 未打开 `.work/engines`、用户 vault、secrets；未改父任务目录；未改 T38–T40；未改 T05–T36 / G0。
- 未把 UI 文案、工具清单、编译 exe、`tauri-build` 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / 签名安装包证据。
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
- `execution/evidence/t37-install-upgrade-recovery.json`
- `.trellis/tasks/09-12-t37-install-upgrade-recovery/implement.md`

未改：`apps/bmdock-desktop/src-tauri/tauri.conf.json`（`bundle.active` 仍为 `false`）。
