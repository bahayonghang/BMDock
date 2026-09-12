# T24 实施记录：中文召回与性能基准

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `run_recall_benchmark`，在 `FixtureLibrary` 上用磁盘 gold（查询如 `欢迎`，相关 permalink 以物理 UTF-8 文件正文包含查询为准）计算 recall@k，并记录进程内 `search_elapsed_ms` / `expand_elapsed_ms`。官方引擎中文召回、native GUI / WebView2 / 安装包 / hosted CI 保持 `UNVERIFIED`。

## What shipped

1. `RecallBenchmarkArgs`：`ExplicitRouteArgs` + 可选 `k`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。非 fixture 为 policy，不打开库。`k` 0 或大于 64 为 schema。默认 `k` 为 20。
2. `RecallBenchmarkDto`：gold 查询列表、`recall_hits` / `recall_relevant`、命中与相关 permalink、CJK permalink、`search_elapsed_ms`、`expand_elapsed_ms`。`semantic_enabled=false`。`engine_search=false`。`native_gui=false`。生产 `EmptyLibrary` 为零查询空态。测试注入 `FixtureLibrary`：命中仅在物理文件包含查询时计数；缺失相关文件为空态，不是用户 vault。声称 `native_gui` / `semantic_enabled` 为 unsupported。
3. zh-CN「基准」面板：空态/错误/就绪。显示 recall@k、查询数、elapsed_ms、semantic_enabled=false、native_gui=false。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。
4. capabilities 精确允许列 25。typed `run_recall_benchmark` 允许；MCP identity `search` / `call_tool` 仍拒绝。双 profile 仍为 21 vs 27 MCP tools。
5. `typed-ipc-policy.md` 允许列 24 → 25。DesktopShellTests 第 18 项覆盖夹具磁盘 gold 与 native_gui=false。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t24-chinese-recall-benchmark` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 152 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（18 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `run_recall_benchmark`，删除基准面板，把允许列恢复为 24。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T23。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T24 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方引擎中文召回。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。cargo test / npm build / UI 文案不是 AC56 native 证明。
- 未宣称全库性能。
- 未改 T05–T23 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T25–T40。
