# T27 实施记录：受控高级工具与 CLI 任务中心

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `inspect_tools` 与 `list_cli_inventory`。前者对照 typed 官方工具允许列与**单个** profile 的静态基线；后者列出已提交的叶子命令名。官方 MCP 工具执行与现场官方 CLI 保持 `UNVERIFIED`。不把 `just contract` 当作 T27 证明。不启动 Supervisor。

## What shipped

1. `InspectToolsArgs`：`ExplicitRouteArgs` + 必填 `profile_id`（`release` | `main-preview`），`deny_unknown_fields`。额外 `path`/`root` 为 schema。缺 `profile_id` 或非法（含 mixed）为 schema。非 fixture 路由为 policy，不打开库。每次只对照一个 profile。把 21 与 27 混合/平均为 `unsupported`。`search` 与 `fetch` 保持不同身份且为 denied。未知名称（如 `evil_tool`）为 denied，不自动放行。桌面不暴露 `call_tool`。`engine_tools=false`，`files_written=false`，`live_execution=false`。
2. `ListCliInventoryArgs`：`ExplicitRouteArgs` + 必填 `profile_id` + 可选 `cursor`/`page_size`（默认 20，上限 64）。额外 `path`/`root` 为 schema。`page_size` 0 或过大、非法/重复 cursor 为 schema。叶子是路径段；会隐藏子命令的粗分组（另一条路径的前缀）为 unsupported。`executed=false`，`engine_cli=false`。不 spawn `engine_worker.py`，不现场执行官方 CLI。
3. 生产 `EmptyLibrary` 返回空 `tools[]` / 空 `leaves[]`，`classified_as: empty`，不是用户 vault 成功或 CLI 成功。测试注入 `FixtureLibrary`，根目录为 `{temp}/bmdock-t27-*`。已提交 `compatibility/cli-leaves.json` 作为命名叶子目录（不是 T01 完整 83/104 树）。
4. zh-CN「工具」/「CLI 任务中心」空态 / 错误 / 就绪。任务只展示目录，不执行。无 `dangerouslySetInnerHTML`。无新 npm 依赖。
5. capabilities 精确允许列 28 → 30。typed `inspect_tools` / `list_cli_inventory` 允许；`call_tool`、MCP identity `search` / `fetch` / `tools/call` 仍拒绝 serde。`just build` 仍为 G0 探针。DesktopShellTests 第 21 项覆盖目录-only 与官方 MCP/CLI 缺席。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t27-controlled-tools-cli-center` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 163 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（21 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `inspect_tools` 与 `list_cli_inventory`，删除工具 / CLI 任务中心面板，把允许列恢复为 28。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T26。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T27 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 MCP 工具执行或现场官方 CLI。
- 未对照 T01 完整 83/104 命名 CLI 树（gitignored 产物）；桌面只列出已提交的命名子集。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。
- 未改 T05–T26 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T28–T40。
