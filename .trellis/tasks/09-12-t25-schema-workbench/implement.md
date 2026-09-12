# T25 实施记录：Schema 工作台

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `schema_validate`，用 BMDock 自有夹具 Markdown/JSON-like frontmatter 与宿主 schema 目录（默认 `note` 要求物理 UTF-8 上的 `title`/`body`）校验夹具笔记。官方 MCP `schema_validate` / `schema_infer` / `schema_diff` 保持 `UNVERIFIED`。

## What shipped

1. `SchemaValidateArgs`：`ExplicitRouteArgs` + 必填 `identifier` + 可选 `schema_id`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。缺 identifier 为 schema。提供空 `schema_id` 为 schema。非 fixture、文件系统 identifier、文件系统 schema_id 为 policy，不打开库。
2. `SchemaValidateDto`：`verdict` 为 `valid` / `invalid` / `empty` / `unsupported`，记录 `required_fields`、`missing_fields`、`observed_title`、`observed_body`。`engine_schema=false`。生产 `EmptyLibrary` 为 empty/unsupported。测试注入 `FixtureLibrary`：物理文件匹配时 `valid`，磁盘缺必填字段时 `invalid`。信封文案不是磁盘证明。声称 `engine_schema=true` 或信封-only `valid` 为 unsupported。
3. zh-CN「Schema 工作台」：空态/错误/就绪。显示 verdict、schema_id、已观察 title/body。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。
4. capabilities 精确允许列 26。typed `schema_validate` 允许；MCP identity `schema_infer` / `schema_diff` / `call_tool` 仍拒绝。双 profile 仍为 21 vs 27 MCP tools。未增加 `schema_infer` / `schema_diff` IPC。
5. `typed-ipc-policy.md` 允许列 25 → 26。DesktopShellTests 第 19 项覆盖夹具自有校验与官方 schema MCP 缺席。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t25-schema-workbench` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 156 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（19 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `schema_validate`，删除 Schema 工作台面板，把允许列恢复为 25。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T24。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T25 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 MCP `schema_validate` / `schema_infer` / `schema_diff`。
- 未增加 typed `schema_infer` / `schema_diff` IPC 命令。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。
- 未改 T05–T24 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T26–T40。
