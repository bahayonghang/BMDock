# T23 实施记录：检索Inspector与模型状态

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `inspect_search`，解释 BMDock 自有 T21 夹具词法检索：查询、命中 permalink、分开的 `lexical_score` 与 `semantic_score`，以及语义关闭/模型未加载。官方引擎语义、嵌入后端、T24 召回保持 `UNVERIFIED` / 未实现。

## What shipped

1. `InspectSearchArgs`：`ExplicitRouteArgs` + 必填 `query` + 可选 `identifier`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。额外 `id`（fetch 身份对调）为 schema。缺/空 query 为 schema。空 identifier 为 schema。非 fixture、文件系统 query-as-path、文件系统 identifier 为 policy，不打开库。
2. `SearchInspectorDto`：命中复用 T21 `SearchHitDto`。`semantic_enabled=false`。`model_id=null`。`model_loaded=false`。`embedding_backend=none`。`model_class=unclassified`。`files_written=false`。`semantic_disabled_reason` 说明语义关闭。生产 `EmptyLibrary` 为空 Inspector。测试注入 `FixtureLibrary` 并核对物理 UTF-8（含中文）。声称 `semantic_enabled` / `model_loaded` 为 unsupported。
3. `get_runtime_state` 增加 `semantic_model_loaded=false`，不宣称已加载，也不启动 Supervisor。
4. zh-CN「检索 Inspector」面板：空态/错误/就绪。显示查询、命中 permalink、分数、semantic_enabled=false、model_loaded=false。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。
5. capabilities 精确允许列 24。typed `inspect_search` 允许；MCP identity `search` / `call_tool` 仍拒绝。双 profile 仍为 21 vs 27 MCP tools。
6. `typed-ipc-policy.md` 允许列 23 → 24。DesktopShellTests 第 17 项覆盖 Inspector 边界。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t23-search-inspector-model-state` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 148 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（17 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `inspect_search`，删除 Inspector 面板，把允许列恢复为 23，去掉 `semantic_model_loaded` 投影。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T22。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T23 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方语义检索/模型/嵌入后端。
- 未实现 T24 召回、T30 extras、T34 providers。
- 未改 T05–T22 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T24–T40。
