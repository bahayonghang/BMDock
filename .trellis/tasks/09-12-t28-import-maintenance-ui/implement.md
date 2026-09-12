# T28 实施记录：导入与日常维护界面

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `import_notes`。`source_id` 是 BMDock 自有夹具源标识，不是文件系统路径。导入把测试注入的 `{temp}/bmdock-t28-*` Markdown 复制进自有夹具库，并以物理 UTF-8（含中文）为磁盘证据。信封 `"imported"` 不是磁盘证明。日常维护继续复用 T12 `list_backups` / `restore_fixture`。官方 extras / 文档摄取与现场 CLI import 保持 `UNVERIFIED`（T30）。不把 `just contract` 当作 T28 证明。不启动 Supervisor。

## What shipped

1. `ImportNotesArgs`：`ExplicitRouteArgs` + 必填 `source_id`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。缺 `source_id` 为 schema。非 fixture 路由与文件系统/`%APPDATA%`/`/` `source_id` 为 policy，且不打开库或备份 store。生产 `EmptyLibrary` 返回空 `files[]`、`classified_as: empty`、`engine_import=false`，不是用户 vault 成功。
2. 测试注入 `{temp}/bmdock-t28-*` 源文件（含 `欢迎.md`）。`FixtureLibrary` 复制到自有库后观察物理 UTF-8。`files_written=true` 且 `classified_as: disk_verified` 才算磁盘核对。信封 `"imported"` / 无写入的 `disk_verified` 经 `accept_import_result` 降为 `accepted_unverified`。`scanned_user_obsidian_vault=false`，`scanned_user_basic_memory_home=false`。
3. 维护 UI 仍只列出 BMDock 生成的夹具备份（复用 T12）。导入与 `restore_fixture` 分开。不恢复用户 vault。
4. zh-CN「导入」+ 既有「维护」。空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。无新 npm 依赖。
5. capabilities 精确允许列 30 → 31。typed `import_notes` 允许。`just build` 仍为 G0 探针。DesktopShellTests 第 22 项覆盖夹具导入与官方 extras 缺席。双 profile 保持隔离（21 vs 27）。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t28-import-maintenance-ui` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 168 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（22 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `import_notes`，删除导入面板，把允许列恢复为 30，保留 T12 维护清单。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T27。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T28 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 extras / 文档摄取或现场 CLI import（T30）。
- 未扫描用户 Obsidian 或全局 Basic Memory 主目录。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。
- 未改 T05–T27 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T29–T40。
