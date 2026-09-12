# BMDock 帮助与支持

这是 T39 提交的支持文档。它描述当前桌面壳与探针入口，不是 G0 或 G7 通过证明，也不是 native 读屏 / IME / native GUI 审计。

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

当前 typed `ipc_invoke` 允许列包含 44 个命令，其中包括：

- `inspect_api_audit`
- `inspect_privacy`
- `inspect_install`
- `inspect_bundle`
- `inspect_help`

完整允许列还包含 `get_capabilities`、`get_runtime_state`、`list_projects`、`run_preflight`、`discover_config`、`list_tree`、`read_note`、`list_relations`、`expand_graph`、`search_notes`、`inspect_search`、`run_recall_benchmark`、`schema_validate`、`list_resources`、`list_prompts`、`inspect_tools`、`list_cli_inventory`、`import_notes`、`inspect_extras`、`ingest_document`、`inspect_cloud`、`inspect_sync`、`list_shares`、`inspect_hooks`、`inspect_providers`、`inspect_routes`、`preview_context`、`list_activity`、`list_backups`、`restore_fixture`、`inspect_windows_runtime`、`save_draft`、`load_draft`、`write_note`、`edit_note`、`move_note`、`delete_note` 与 `begin_shutdown`。`call_tool` / `restore_sync` 不在允许列。

## 边界

- 只连接生成的 `bmdock-fixture`。
- 不打开用户 vault、全局 Basic Memory 配置或 `.work/engines`。
- 不把工具清单、UI 文案或 `just contract` 当作 native GUI / 读屏 / IME / Job Object / 真实 vault 证明。
