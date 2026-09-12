# T21 实施记录：全文语义混合检索

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `search_notes`，对 BMDock 自有夹具 Markdown 做可磁盘核对的词法检索，并在混合 DTO 上分开 `lexical_score` 与 `semantic_score`。官方引擎语义检索、MCP `search`/`fetch`、T23 Inspector 保持 `UNVERIFIED` / 未实现。

## What shipped

1. `SearchNotesArgs`：`ExplicitRouteArgs` + 必填 `query` + 可选 `cursor` / `page_size`，`deny_unknown_fields`。
2. `search_notes` 分发：先校验 fixture 路由与查询，再打开 `NoteLibrary`。额外 `path`/`root`/`id` 为 schema。缺/空 query 为 schema。非 fixture 与 query-as-path 为 policy，不打开库。`page_size` 0/过大、非法/重复 cursor 为 schema。截断库存为 unsupported。
3. `FixtureLibrary` 在物理 UTF-8 文件标题/正文中做子串命中（含中文）。命中标识是 permalink。核对物理文件后 `classified_as=disk_verified`。`EmptyLibrary` 返回空 hits + empty 观察。信封-only 命中为 `accepted_unverified`。
4. 混合 DTO：`lexical_score` 与 `semantic_score` 分开。`semantic_enabled=false`，`engine_search=false`。不把双 profile 工具数写入检索 DTO。
5. zh-CN「检索」面板：labeled 查询输入、permalink + 分数列表、空态/错误/就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖，无 vis.js。
6. capabilities 精确允许列 21。typed `search_notes` 允许；MCP identity `search` 与 `call_tool` 仍拒绝。
7. `typed-ipc-policy.md` 允许列 20 → 21。DesktopShellTests 第 15 项覆盖检索边界。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t21-hybrid-search` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 138 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（15 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `search_notes`，删除 `SearchPanel`，把允许列恢复为 20。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T20。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T21 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现 T23 Inspector。
- 未实现官方语义/嵌入后端。
- 未改 T05–T20 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T22–T40。
