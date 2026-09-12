# T12 实施记录：备份清单与 fixture 恢复基线

## 目标

把 P2 T12 做成可独立验收的产品能力：typed `list_backups` / `restore_fixture`（每次携带 `ExplicitRouteArgs`）、仅 BMDock 自有生成夹具备份、恢复后必须观察物理文件，以及 zh-CN「维护」分区。不引入 rmcp/live MCP，不 `write_note`，不恢复用户 vault，不把夹具恢复成功推断为 T17/T37/T38。

## 已完成

1. 新增 `apps/bmdock-desktop/src-tauri/src/backups.rs`：`BackupStore` trait、生产默认 `EmptyBackupStore`、测试专用 `FixtureBackupStore`。清单只列出生成的 fixture 备份标识；空列表是空态，不是用户 vault 成功。
2. `ipc_invoke` 允许列变为 10：`list_backups` 与 `restore_fixture` 消费 `ExplicitRouteArgs`（恢复另加 `backup_id`）。额外 path/root 为 schema；非 fixture / 文件系统 backup_id 为 policy 且不打开路径。未知 `call_tool` / `write_note` 仍失败。
3. `restore_fixture` 把生成 Markdown 从命名快照复制到生成的自有目标目录。测试读取物理文件正文（含中文与 wiki-link）。`envelope_is_not_disk_proof=true`。分类 `disk_verified` vs `accepted_unverified`。`files_written` 仅在实际写入自有文件后为 true。
4. renderer 增加 zh-CN「维护」分区（空态 / 错误 / 就绪）。恢复按钮只对 fixture 备份标识显示。无 `dangerouslySetInnerHTML`。不 start/stop Supervisor。
5. 更新 `typed-ipc-policy.md`、`ipc.ts` 联合、`DesktopShellTests`、`execution/status.json`（仅 T12 planned→completed）和证据。

## 恢复清单（AC53）

### 本次恢复了什么

- `backup_id`：`fixture-welcome`（测试生成的 BMDock 自有夹具备份，不是用户 vault 备份）
- 文件标识：`welcome`
- 快照正文：中文标题/正文 + wiki-link `[[欢迎]]`
- 恢复目标：测试进程生成的自有目录 `{temp}/bmdock-t12-*/target/`，不是 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 配置
- 观察：恢复后 `target/welcome.md` 存在，`read_to_string` 等于快照正文，`classified_as=disk_verified`，`files_written=true`
- 信封对照：`EnvelopeBackupStore` 返回成功形状但不写盘，分类为 `accepted_unverified`，`files_written=false`

### 观察到的路径（模式，不提交绝对用户路径）

- 快照：`{std::env::temp_dir()}/bmdock-t12-{nanos}/snapshots/fixture-welcome/welcome.md`
- 目标：`{std::env::temp_dir()}/bmdock-t12-{nanos}/target/welcome.md`
- IPC DTO 不包含文件系统 `path` 字段
- 未打开、未写入：用户 Obsidian vault、`%APPDATA%\Obsidian`、`.basic-memory`

### 仍为 UNVERIFIED

- 强杀恢复、Windows Job Object / 进程树、睡眠恢复、磁盘故障注入
- T17 正常退出排空与故障恢复
- T37 安装升级签名与数据恢复链
- T38 原生安装包及故障回归
- 用户真实 vault / 生产 / Cloud 恢复
- native GUI / WebView2 / hosted CI

## 本轮命令退出

| 命令 | 退出码 |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t12-backup-fixture-recovery` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 59 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（6 passed） |
| `python -m unittest discover -s tests -v` | 1（65 ok，1 ERROR `test_repository_phase_order`） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+ completed；未回退） |
| `just contract` / `just contract-main` | 未运行 |

## UNVERIFIED

- 强杀 / Job Object / 睡眠恢复 / 磁盘故障（AC18；沿用 `scripts/probe.py:recovery_boundaries`）
- 官方引擎备份/恢复 MCP（未加入 rmcp，未跑 `just contract`）
- native GUI / WebView2 / 真实 vault / hosted CI / 安装器恢复链（T17/T37/T38）

## 未做

- 未 git commit / push / merge / amend
- 未切换 `just build`
- 未改 T05–T11 / G0、父任务目录、T13–T40、`.work/engines`、用户 vault 或密钥
- 未实现笔记写入 / 编辑 / 移动 / 删除（T14/T15）
- 未把夹具恢复成功推断为故障注入或安装器恢复证据

## 回滚点

删除 `backups.rs`、从 `IpcCommand` 去掉 `list_backups`/`restore_fixture`、恢复 8 命令允许列和「维护」分区。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T11 或 G0。
