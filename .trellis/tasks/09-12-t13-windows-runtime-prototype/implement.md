# T13 实施记录：提前验证干净 Windows 运行时原型

## 目标

把 P2 T13 做成可独立验收的产品/验证能力：typed `inspect_windows_runtime`（`EmptyArgs`）、观察与未验证主张分离的 DTO、zh-CN「运行状态」下的「运行时」卡片，以及 AC49 / AC51 / AC55 证据。这不是 T37/T38 安装器/签名，不是 T17 崩溃恢复，也不是 native GUI 会话。

## 已完成

1. 新增 `apps/bmdock-desktop/src-tauri/src/windows_runtime.rs`。`inspect_windows_runtime` 只观察宿主 OS、已知 EdgeWebView 安装目录/loader、Job Object API 文档存在性，以及提交的 `tauri.conf.json` `bundle.active`。不创建 Job Object、不分配子进程、不启动 WebView2/Tauri 窗口、不写文件、不扫描用户 Obsidian / Basic Memory 主目录。
2. `ipc_invoke` 允许列变为 11：`inspect_windows_runtime` 使用 `EmptyArgs`（`deny_unknown_fields`）。额外 `path`/`root` 为 schema。未知 `call_tool` / `write_note` 仍失败。`list_backups` / `restore_fixture` 仍要求 `ExplicitRouteArgs`。不 start/stop Supervisor。
3. DTO 字段为 snake_case，并在 `ipc.ts` 镜像：`host_os`、`webview2_files_present`、`webview2_session_verified`、`job_object_assigned`、`job_object_api_documented`、`installer_bundle_active`、`files_written`、`scanned_user_obsidian_vault`、`scanned_user_basic_memory_home`。证据分类写进测试：编译 exe / npm build / cargo test ≠ native GUI；WebView2 文件 ≠ 会话；Job Object API/文档 ≠ 已分配；`just contract` ≠ Windows 运行时；T12 夹具恢复 ≠ Windows 恢复。
4. renderer 在「运行状态」下增加 zh-CN「运行时」卡片（空态 / 错误 / 就绪）。已观察与未验证分区分开显示。无 `dangerouslySetInnerHTML`。不 start Supervisor。
5. 更新 `typed-ipc-policy.md`（10→11）、`ipc.ts` 联合、`DesktopShellTests`、`execution/status.json`（仅 T13 planned→completed）和证据。

## 本机观察（不是会话证明）

- `host_os=windows`
- 已知 EdgeWebView `Application` 目录在 `Program Files` 与 `%LOCALAPPDATA%` 下不存在；`webview2_files_present=false`
- `webview2_session_verified=false`（未打开原生窗口或交互）
- `job_object_api_documented=true`（Windows 目标上的 API/文档观察）
- `job_object_assigned=false`（未创建 Job Object，也未分配子进程）
- `installer_bundle_active=false`，与 `tauri.conf.json` `bundle.active` 一致
- `files_written=false`；未扫描用户 vault / Basic Memory 主目录

## 本轮命令退出

| 命令 | 退出码 |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t13-windows-runtime-prototype` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 70 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（7 passed） |
| `python -m unittest discover -s tests -v` | 1（66 ok，1 ERROR `test_repository_phase_order`） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+ completed；未回退） |
| `just contract` / `just contract-main` | 未运行 |
| native GUI / `just tauri-dev` 交互 | 未运行 |

## UNVERIFIED

- native GUI 会话 / WebView2 会话交互（`webview2_session_verified`）
- Job Object 创建并分配子进程（`job_object_assigned`）
- 安装器、签名、hosted CI（AC51 余项；签名仍属 T37）
- T17 正常退出排空与故障恢复
- T37/T38 安装升级与原生安装包回归
- 真实用户 vault / 生产 / Cloud

## 未做

- 未 git commit / push / merge / amend
- 未切换 `just build`
- 未启用 `bundle.active`，未制作安装器
- 未改 T05–T12 / G0、父任务目录、T14–T40、`.work/engines`、用户 vault 或密钥
- 未把编译 exe、npm build、cargo test、`just contract` 或 T12 夹具恢复推断为 native GUI / WebView2 会话 / Job Object 分配 / Windows 恢复

## 回滚点

删除 `windows_runtime.rs`、从 `IpcCommand` 去掉 `inspect_windows_runtime`、恢复 10 命令允许列和「运行时」卡片。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T12 或 G0。
