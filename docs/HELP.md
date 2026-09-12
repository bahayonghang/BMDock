# BMDock 帮助与支持

这是 T39 提交、T40 更新的支持文档。它描述当前桌面壳、探针入口与发布决策。发布决策为不得发布。本文不是 G0 或 G7 通过证明，也不是 native 读屏 / IME / native GUI / 安装器 / 现场官方引擎审计。

## 命令入口

- `just dev` 与 `just tauri-dev` 相同：启动 T08 Tauri 桌面壳。
- `just build` 仍编译 G0 探针 `bmdock-probe`。CI 继续使用该入口。它不是 Tauri，也不是安装器。
- `just tauri-build` 是桌面构建入口，不是安装器。
- `just contract` 与 `just contract-main` 仍是真实引擎探针 smoke。不要把它们当作桌面功能、native GUI 或本帮助文档的验收。

## 双 profile 隔离

- `release`：commit `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`，21 个 MCP 工具。
- `main-preview`：commit `3452c821d76c083823d020984d71e06904a1ff1e`，27 个 MCP 工具。
- 两套 profile 不得混合、合并或平均。

## 许可与 SBOM

仓库包含 `LICENSE`、`NOTICE` 与 `docs/sbom/lockfile-inventory.json`。漏洞扫描、人工法律复核与 G7 仍为 `UNVERIFIED`。本文不宣称 G0 或 G7 已通过。

## 上下文预览与近期活动（AC31）

- `preview_context` 与 `list_activity` 是夹具自有命令：Markdown 片段与文件 mtime。
- 官方 `recent_activity` / `build_context` 仍为 `UNVERIFIED`。帮助目录不加入 live engine MCP。

## 恢复库存（AC53）

恢复库存是夹具自有的 `list_backups` + `restore_fixture`。这不是云恢复，也不是安装器回滚用户 vault。`restore_sync` 不在 typed 允许列。

## 无障碍（AC57）

Renderer 拥有：

- skip-link，指向 `#main`
- `nav` landmark
- `main` landmark，`id="main"`
- 带标签面板
- 可见 `:focus-visible`
- 可键盘聚焦控件

native 读屏、IME 与 native GUI 仍为 `UNVERIFIED`。cargo test、npm build 与 UI 文案不是原生窗口证明。夹具 `help-claimed` / `a11y-cleared` 不是原生无障碍审计。

## Typed 允许列（节选）

当前 typed `ipc_invoke` 允许列包含 45 个命令，其中包括：

- `inspect_api_audit`
- `inspect_privacy`
- `inspect_install`
- `inspect_bundle`
- `inspect_help`
- `inspect_release`

完整允许列还包含 `get_capabilities`、`get_runtime_state`、`list_projects`、`run_preflight`、`discover_config`、`list_tree`、`read_note`、`list_relations`、`expand_graph`、`search_notes`、`inspect_search`、`run_recall_benchmark`、`schema_validate`、`list_resources`、`list_prompts`、`inspect_tools`、`list_cli_inventory`、`import_notes`、`inspect_extras`、`ingest_document`、`inspect_cloud`、`inspect_sync`、`list_shares`、`inspect_hooks`、`inspect_providers`、`inspect_routes`、`preview_context`、`list_activity`、`list_backups`、`restore_fixture`、`inspect_windows_runtime`、`save_draft`、`load_draft`、`write_note`、`edit_note`、`move_note`、`delete_note` 与 `begin_shutdown`。`call_tool` / `restore_sync` / `enable_provider` 不在允许列。typed 允许列是已存在命令的来源，不会从允许列推断官方 MCP。

## 边界

- 只连接生成的 `bmdock-fixture`。
- 不打开用户 vault、全局 Basic Memory 配置或 `.work/engines`。
- 不把工具清单、UI 文案、`just contract`、编译 exe、LICENSE 或 lockfile SBOM 当作 G0 / G7 / native GUI / 读屏 / IME / Job Object / 真实 vault / 安装器 / 现场官方引擎证明。

## 发布决策（T40）

发布决策为**不得发布**。`release_allowed=false`。G0 未通过。G7 未通过。生产空库 `classified_as: empty`，`files_written=false`。夹具 `release-claimed` / `gate-passed` 不是已通过的发布。`search` 与 `fetch` 保持不同被拒身份。`full_api_coverage=false`。命名缺口经 T29 具名。能力漂移保持 fail-closed。`just build` 仍是 G0 探针；`just tauri-dev` / `tauri-build` 仍是桌面入口；`just contract*` 仍是探针。
