# T22 实施记录：上下文预览与近期活动

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `preview_context` 与 `list_activity`。预览是 BMDock 自有夹具 Markdown 的物理 UTF-8 片段（含中文子串核对，`executed=false`）。近期活动是夹具 Markdown permalink 的文件 mtime 顺序。官方 MCP `recent_activity` / `build_context` 保持 `UNVERIFIED`。

## What shipped

1. `PreviewContextArgs`：`ExplicitRouteArgs` + 必填 `identifier` + 可选 `query`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。缺 identifier 为 schema。非 fixture 与文件系统 identifier / query-as-path 为 policy，不打开库。
2. `ListActivityArgs`：`ExplicitRouteArgs` + 可选 `cursor` / `page_size`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。非 fixture 为 policy。`page_size` 0/过大、非法/重复 cursor 为 schema。截断库存为 unsupported。
3. `FixtureLibrary` 从物理 UTF-8 文件切出预览片段（可选 query 窗口）。片段必须是物理文件子串。HTML 仍为文本。核对物理文件后 `classified_as=disk_verified`。缺文件与 `EmptyLibrary` 为空片段 + empty 观察。信封-only 片段为 `accepted_unverified`。`engine_context=false`。
4. `FixtureLibrary` 列出自有 `.md` 的 permalink + `observed_mtime`（mtime 降序）。测试注入夹具并观察列出的标识对应物理文件。`engine_activity=false`。不是官方 `recent_activity`。
5. zh-CN「预览」/「近期活动」面板：空态/错误/就绪。预览用 `<pre data-preview="text" data-executed="false">`。活动列出 permalink 与观察 mtime，不是 FS 路径。检索命中预览复用 `preview_context`。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。
6. capabilities 精确允许列 23。typed `preview_context` / `list_activity` 允许；MCP identity `search` / `recent_activity` / `build_context` 与 `call_tool` 仍拒绝。双 profile 仍为 21 vs 27 MCP tools。
7. `typed-ipc-policy.md` 允许列 21 → 23。DesktopShellTests 第 16 项覆盖预览与活动边界。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t22-context-activity-preview` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 145 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（16 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `preview_context` 与 `list_activity`，删除预览与活动面板，把允许列恢复为 21。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T21。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T22 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 `recent_activity` / `build_context`。
- 未实现 T23 Inspector，未做 T39 帮助/无障碍扩展。
- 未改 T05–T21 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T23–T40。
