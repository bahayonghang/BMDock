# T34 实施记录：外部Provider与后端档位验证

本文件是 T34 的执行记录，不是规划草案。未 git commit / push / merge / amend。未改 T05–T33/G0。未改父任务目录。未实现 T35–T40。

## 做了什么

1. 在既有 `ipc_invoke` 上增加 typed `inspect_providers`（`ExplicitRouteArgs`，`deny_unknown_fields`）。
2. 额外 `path` / `root` / `token` / `host` / `api_key` 为 schema。缺路由为 schema。非 fixture 为 policy，且不打开库。
3. 生产 `EmptyLibrary`：空 provider 目录、`classified_as: empty`、`provider_enabled=false`、`semantic_enabled=false`、`model_loaded=false`、`files_written=false`、`backend_tier=disabled`、`local_offline=true`。
4. openai / anthropic / huggingface / cloud embeddings 显式 `unavailable` / unverified，从不静默启用。
5. 检索仍走 T21 夹具词法。Providers 不启用官方 semantic / search / fetch。search 与 fetch 保持不同且被拒绝的身份。
6. 测试注入 `{temp}/bmdock-t34-*`。夹具 `provider-claimed` 标记为 unsupported，不是 connected。
7. capabilities 精确允许列 38 → 39。`enable_provider` / `list_providers` / `connect_provider` 不在允许列。
8. zh-CN「Provider / 后端档位」空态 / 错误 / 就绪均显示未启用。无 `dangerouslySetInnerHTML`。无新 npm 依赖。无 rmcp。不启动 Supervisor。
9. DesktopShellTests 第 28 项：`test_providers_are_fail_closed_unavailable_not_enabled`。
10. 更新 `typed-ipc-policy.md`、`docs/VERIFICATION.md`、`execution/evidence/t34-provider-backend-validation.json`；`execution/status.json` 仅将 T34 标为 completed。

## 验证命令

见 `execution/evidence/t34-provider-backend-validation.json`。`just contract` / `just contract-main` 未作为 T34 证明运行。`python -m scripts.tasks unit` 因 G0 未 passed 且 T05+ 已 completed 失败，未回退。

## 未做 / UNVERIFIED

- 官方 openai / anthropic / huggingface / cloud embeddings live session
- native GUI / WebView2 / 安装器 / hosted CI
- 真实用户 vault
- 远程主机联系、密钥存储（明确未做）
- T35–T40
