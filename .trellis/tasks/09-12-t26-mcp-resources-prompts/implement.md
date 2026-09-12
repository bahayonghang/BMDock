# T26 实施记录：MCP 资源与提示词工作台

## Goal

在现有 `ipc_invoke` 上增加 typed 宿主命令 `list_resources` 与 `list_prompts`，用 BMDock 自有夹具磁盘目录列出资源标识与提示词 sidecar 模板。官方 MCP `resources/list` / `resources/read` / `prompts/list` / `prompts/get` 保持 `UNVERIFIED`。不把 `just contract` 当作 T26 证明。

## What shipped

1. `ListResourcesArgs` / `ListPromptsArgs`：`ExplicitRouteArgs` + 可选 `cursor` / `page_size`，`deny_unknown_fields`。额外 `path`/`root` 为 schema。`page_size` 0 或大于 64 为 schema。默认 `page_size` 20，上限 64（与 T11/T20 相同）。非 fixture 路由为 policy，不打开库。
2. `ResourcePageDto` / `PromptPageDto`：夹具顶层 `*.md`（排除 `*.prompt.md`）给出资源标识；`*.prompt.md` sidecar 给出提示词标识。命中必须对应物理文件。信封文案不是磁盘证明。`engine_resources=false`，`engine_prompts=false`。生产 `EmptyLibrary` 为空目录 / `classified_as: empty`，不是用户 vault 成功。测试注入 `FixtureLibrary`，根目录为 `{temp}/bmdock-t26-*`。声称 `engine_resources=true` / `engine_prompts=true`、截断页或 cursor 环为 unsupported。
3. zh-CN「资源」/「提示词」面板：空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。
4. capabilities 精确允许列 28。typed `list_resources` / `list_prompts` 允许；MCP identity `resources/list` / `resources/read` / `prompts/list` / `prompts/get` 与 `call_tool` 仍拒绝 serde。双 profile 仍为 21 vs 27 MCP tools，不写入资源/提示词 DTO。
5. `typed-ipc-policy.md` 允许列 26 → 28。DesktopShellTests 第 20 项覆盖夹具自有目录与官方 MCP 缺席。`just build` 仍为 G0 探针。

## Validation

- `python ./.trellis/scripts/task.py validate 09-12-t26-mcp-resources-prompts` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 160 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 Supervisor dead_code 警告仍在）
- `npm run build` in `apps/bmdock-desktop` → 0（未跑 `npm ci`）
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（20 passed）
- `python -m scripts.tasks unit` → 1（G0 未 passed 且 T05+ completed；未回退）

## Rollback

从 `ipc_invoke` 去掉 `list_resources` 与 `list_prompts`，删除资源 / 提示词面板，把允许列恢复为 26。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T25。

## Not done

- 未运行 `just contract` / `just contract-main` 作为 T26 证明。
- 未加入 rmcp，未启动 Supervisor。
- 未实现官方 MCP `resources/list` / `resources/read` / `prompts/list` / `prompts/get`。
- 未启动 native 窗口、WebView2 会话、安装包或 hosted CI。
- 未改 T05–T25 / G0，未改 `.work/engines`、用户 vault、密钥、父任务目录、T27–T40。
