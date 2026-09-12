# T40 实施记录：全能力Release Gate与发布决策

## Outcome

`accepted_with_limits`。typed `inspect_release` 已加入现有 `ipc_invoke`（第 45 个允许命令）。生产空库 `release_allowed=false`、`g0_passed=false`、`g7_passed=false`、`classified_as: empty`、`files_written=false`。发布决策为**不得发布**。未宣称 G0 / G7 通过。未把任何门禁标为 passed。

## What was implemented

1. Rust `NoteLibrary::inspect_release` + `ReleaseInspectionDto`（`deny_unknown_fields` 路由经 `ExplicitRouteArgs`）。生产 `EmptyLibrary` 返回不得发布 DTO。双 profile 隔离：release `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`（21）与 main-preview `3452c821d76c083823d020984d71e06904a1ff1e`（27）。search / fetch 保持不同被拒身份。`full_api_coverage=false`。T29 命名缺口保持具名。typed 允许列为 45，不会从允许列推断官方 MCP。
2. Fail-closed：额外 `path`/`root`/`token`/`host` → schema；缺路由 → schema；非 fixture → policy，且不打开库。vault / secrets / remote → policy。宣称 `release_allowed` / G0 通过 / G7 通过 / 混合 profile / 自动放行未知工具 / 完整 API 覆盖 → unsupported。夹具 `{temp}/bmdock-t40-*` 的 `release-claimed` / `gate-passed` 文件旗标为 unsupported，不是已通过的发布。
3. IPC：`IpcCommandName::InspectRelease` 进入 45 命令允许列；`call_tool` / `restore_sync` / `enable_provider` 仍缺席。表驱动 `explicit_route_args_commands_fail_closed_without_opening_library` 覆盖 `inspect_release`。
4. Renderer：zh-CN「发布决策」面板。empty / error / ready 均显示不得发布 / G0 未通过 / G7 未通过。无 `dangerouslySetInnerHTML`。无新 npm 依赖。不启动 Supervisor。无 rmcp。
5. 规范：`.trellis/spec/bmdock-probe/backend/{index.md,typed-ipc-policy.md}` 允许列 44 → 45。`tests/test_desktop_shell.py` 第 34 项 `test_release_gate_decision_is_denied_and_gates_unpassed`。更新 `docs/HELP.md` 与 `docs/VERIFICATION.md`。
6. `execution/status.json` 仅将 T40 `planned` → `completed`。G0 仍为 `in_progress`。G1–G7 仍为 `not_started`。未改 T05–T39。未改父任务目录。未改 justfile。

## AC mapping

| ID | Result |
|---|---|
| AC01 | 双 profile 保持隔离。release（21）与 main-preview（27）不得在 DTO 中混合或平均。 |
| AC35 | search 与 fetch 保持不同被拒身份。未知工具不会自动放行。不得把 21+27 合并。 |
| AC36 | CLI/API 覆盖仍不完整。`full_api_coverage=false`。命名缺口经 T29 具名。前缀桶隐藏叶子不受支持。 |
| AC55 | `just build` 仍是 G0 探针。`just tauri-dev` / `tauri-build` 仍是桌面。`just contract*` 仍是探针。未改 justfile。 |
| AC58 | 能力漂移保持 fail-closed。未知危险命令缺席（`call_tool`、`restore_sync`、`enable_provider`）。 |
| AC59 | typed 允许列是已存在命令的来源（44 → 45，含 `inspect_release`）。不得从允许列推断官方 MCP。 |
| AC60 | `docs/HELP.md` 与 `VERIFICATION.md` 保持一致。T40 不宣称 G0/G7 已通过。 |

## Commands and exits

| Command | Exit |
|---|---|
| `python ./.trellis/scripts/task.py validate 09-12-t40-release-gate-decision` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 216 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 supervisor `dead_code` 警告；未启动 Supervisor） |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（34 tests） |
| `python -m scripts.tasks unit` | 1（`A later task was completed before G0`；未回退 T05+） |
| `python -m scripts.tasks gate` | 2（G0–G7 未通过；未伪造门禁） |

未运行：`just contract` / `just contract-main`、原生 GUI / `just tauri-dev` 交互、安装器、现场官方引擎。

## UNVERIFIED

- G0 product gate / G1–G6 / G7 release gate
- hosted CI for this revision
- native GUI / WebView2 session
- native installer / signed bundle
- Job Object assignment
- live official engine / just contract this session
- official search / fetch MCP
- complete official CLI/API coverage
- real user vault

## What was not done

- 未打开原生窗口，未制作安装器，未运行 `just contract*`。
- 未启动 Supervisor，未加入 rmcp，未改 `just build` 或 `justfile`。
- 未打开 `.work/engines`、用户 vault、secrets；未改父任务目录；未改 T05–T39 / G0–G7。
- 未把 UI 文案、工具清单、编译 exe、LICENSE、lockfile SBOM、cargo test 或 `just contract` 当作 G0 / G7 / native GUI / 安装器 / 现场官方引擎证明。
- 未 git commit / push / merge / amend。
- **未把 G0–G7 标为 passed。** G0 仍为 `in_progress`；G1–G7 仍为 `not_started`。

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
- `execution/evidence/t40-release-gate-decision.json`
- `.trellis/tasks/09-12-t40-release-gate-decision/implement.md`

未改：`justfile`。未改父任务目录。未改 G0–G7。
