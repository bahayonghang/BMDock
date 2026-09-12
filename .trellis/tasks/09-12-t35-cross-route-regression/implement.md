# T35 实施记录：条件能力与跨路由回归

本文件是 T35 的执行记录，不是规划草案。未 git commit / push / merge / amend。未改 T05–T34/G0。未改父任务目录。未实现 T36–T40。

## 做了什么

1. 在既有 `ipc_invoke` 上增加 typed `inspect_routes`（`ExplicitRouteArgs`，`deny_unknown_fields`）。
2. 额外 `path` / `root` / `token` / `host` / `api_key` 为 schema。缺路由为 schema。非 fixture 为 policy，且不打开库。
3. 生产 `EmptyLibrary`：空路由目录、`classified_as: empty`、`cloud_allowed=false`、`sync_enabled=false`、`sharing_enabled=false`、`hooks_enabled=false`、`provider_enabled=false`、`semantic_enabled=false`、`cross_project_search_allowed=false`、`full_api_coverage=false`、`files_written=false`、`local_offline=true`。
4. `inspect_routes` 不报告 `connected` / `synced` / `installed`。宣称 live cloud/sync/agent 为 unsupported。
5. 每个 routed 命令仍要求显式 workspace+project。非 fixture 检索为 policy。缺路由检索为 schema。未增加跨项目 search。
6. 测试注入 `{temp}/bmdock-t35-*`。夹具 `route-claimed` 标记为 unsupported，不是成功。
7. 表驱动回归覆盖全部 routed 命令（含 `inspect_routes` 与 T31–T34 `inspect_*`），且不启动 Supervisor。
8. capabilities 精确允许列 39 → 40。`enable_provider` / `restore_sync` / `list_hooks` / `connect_provider` / raw `callTool` 不在允许列。typed 允许列是 present commands 的来源。`full_api_coverage=false`。不从允许列推断官方 MCP。
9. zh-CN「条件能力 / 跨路由」空态 / 错误 / 就绪均显示未启用，并显示禁止跨项目检索。无 `dangerouslySetInnerHTML`。无新 npm 依赖。无 rmcp。不启动 Supervisor。
10. DesktopShellTests 第 29 项：`test_cross_route_regression_is_fail_closed_not_cross_project`。
11. 更新 `typed-ipc-policy.md`、`docs/VERIFICATION.md`、`execution/evidence/t35-cross-route-regression.json`；`execution/status.json` 仅将 T35 标为 completed。

## 验证命令

见 `execution/evidence/t35-cross-route-regression.json`。`just contract` / `just contract-main` 未作为 T35 证明运行。`python -m scripts.tasks unit` 因 G0 未 passed 且 T05+ 已 completed 失败，未回退。

## 未做 / UNVERIFIED

- 官方 live cloud / sync / agent / MCP session
- native GUI / WebView2 / Job Object / 安装器 / hosted CI
- 真实用户 vault
- 远程主机联系、密钥存储（明确未做）
- T36–T40
