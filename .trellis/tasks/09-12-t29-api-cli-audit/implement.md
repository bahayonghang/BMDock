# T29 实施记录：公开 API 与 CLI 遗漏审计

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `inspect_api_audit`。`ExplicitRouteArgs` + 必填 `profile_id`（`release` | `main-preview`）。对照已提交的命名 CLI/API 叶子与 typed IPC 允许列，缺口列为 missing/unverified。官方 live MCP / 现场 CLI / extras / cloud 保持 `UNVERIFIED`。不把 `just contract` 当作 T29 证明。不启动 Supervisor。

## What shipped

1. `InspectApiAuditArgs`：`ExplicitRouteArgs` + 必填 `profile_id`（`release` | `main-preview`），`deny_unknown_fields`。额外 `path`/`root` 为 schema。缺 `profile_id` 或 mixed 为 schema。非 fixture 路由为 policy，且不打开库。每次只审计一个 profile。把 21 与 27 混合/平均为 unsupported。
2. 审计列出命名 CLI 叶子（`compatibility/cli-leaves.json`）与 MCP 基线叶子（typed IPC 允许列 vs 所选 profile）。缺口列为 missing/unverified。会隐藏子命令的粗分组为 unsupported。未知工具（如 `evil_tool`）为 denied，不自动放行。`search` 与 `fetch` 保持不同身份且为 denied。
3. 不可用能力显式列出：semantic / extras ingest / cloud 为 unavailable；live MCP / official schema MCP / live CLI 为 unverified。`semantic_enabled=false`，`model_loaded=false`，`full_api_coverage=false`。
4. 生产 `EmptyLibrary` 返回空审计、`classified_as: empty`。测试注入 `FixtureLibrary`，根目录为 `{temp}/bmdock-t29-*`。不 spawn `engine_worker`，不现场执行官方 CLI。
5. zh-CN「API/CLI 审计」空态 / 错误 / 就绪。present 与 missing 分开显示。无 `dangerouslySetInnerHTML`。无新 npm 依赖。
6. capabilities 精确允许列 31 → 32。typed `inspect_api_audit` 允许。`just build` 仍为 G0 探针。DesktopShellTests 第 23 项覆盖命名缺口与完整 API 覆盖缺席。双 profile 保持隔离（21 vs 27）。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t29-api-cli-audit` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 171 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（23 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `inspect_api_audit`，删除审计面板，把允许列恢复为 31。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T28。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T29 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 extras / 文档摄取、cloud、live MCP 或现场 CLI。
- 未对照 T01 完整 83/104 命名 CLI 树（gitignored 产物）。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。
- 未改 T05–T28 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T30–T40。
